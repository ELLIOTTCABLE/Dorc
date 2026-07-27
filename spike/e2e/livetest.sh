#!/bin/sh
# livetest — the live-acceptance loop: Dorc's own plan/apply pipe, over real ssh, against a real
# Debian in a throwaway container.
#
# NOT A GATE. Nothing in `gate:*`, `bless` or pre-commit reaches this file, and nothing may: it
# needs a container runtime, a network, and roughly a minute of real `apt-get`. It is invoked by
# hand, by someone who wants to know whether the product still works end to end.
#
# What it proves, in one pass:
#   1. a probe really ships over ssh, runs on a machine that is not this one, and its real records
#      come back through admission (no authored fixture anywhere in the chain);
#   2. the plan built from them matches the hermetic baseline the corpus pins;
#   3. that plan really applies — real `apt-get`, real files, real exit status via the sentinel;
#   4. a SECOND probe of the now-converged world elides what the first pass ran;
#   5. applying that second plan changes nothing.
#
# Step 4 is the one that cannot be faked, and it is the reason this exists.
#
# usage: livetest.sh [run|target|clean|remote <dest>]
#   run     (default) provision, drive the whole loop, tear down
#   target  provision and leave it running, print the destination to talk to
#   clean   remove containers this script created
#   remote  drive the loop against a destination you supply; provisions nothing, tears down nothing
#
# env: DORC_CONTAINER_CLI  override the container runtime (else docker/podman/nerdctl/wslc)
#      LIVETEST_KEEP=1     keep the container and the run directory on success
#      DORC                the dorc binary (else the workspace debug build)

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
spike=$(CDPATH= cd -- "$here/.." && pwd)
root=$(CDPATH= cd -- "$spike/.." && pwd)
kit=$root/Research/trial/r26
# Built by a native-Windows cargo, this is `dorc.exe`; everywhere else it is `dorc`. Probed rather
# than branched on uname, because the shell running this may be git's while the build was MSVC's.
dorc=${DORC:-}
if [ "$dorc" = "" ]; then
   dorc=$spike/target/debug/dorc
   [ -x "$dorc" ] || [ ! -x "$dorc.exe" ] || dorc=$dorc.exe
fi

IMAGE=debian:12-slim
BOOK=$kit/container-book.sh
ORACLES=$kit/oracles
NAME_PREFIX=dorc-livetest

# The baselines, from `Research/trial/r26/renders/`. Counts only — never the byte render, which
# carries absolute paths and a decision digest that no live run reproduces.
BASELINE_PRISTINE='sites=12 elide=1 omit=0 guard=0 run=11'
BASELINE_CONVERGED='sites=12 elide=4 omit=4 guard=0 run=4'

say() { printf 'livetest: %s\n' "$*" >&2; }
die() { printf 'livetest: %s\n' "$*" >&2; exit 1; }

# ── the container runtime ────────────────────────────────────────────────────────────────────

# The probe list, in preference order. wslc appears twice by absolute path because it ships at a
# fixed location whose PATH entry an already-running process does not inherit — `command -v` alone
# misses a runtime that is installed and working.
PROBED='docker, podman, nerdctl, wslc, /c/Program Files/WSL/wslc.exe, /mnt/c/Program Files/WSL/wslc.exe'

# Set RUNTIME to $1 if it is usable. Kept a separate call per candidate rather than a loop,
# because the candidates contain spaces and word-splitting a list of them is how that breaks.
try_runtime() {
   [ "${RUNTIME-}" = "" ] || return 0
   case $1 in
   /*) [ -x "$1" ] && RUNTIME=$1 ;;
   *) command -v "$1" >/dev/null 2>&1 && RUNTIME=$1 ;;
   esac
   return 0
}

resolve_runtime() {
   RUNTIME=
   if [ "${DORC_CONTAINER_CLI-}" != "" ]; then
      if ! command -v "$DORC_CONTAINER_CLI" >/dev/null 2>&1 && [ ! -x "$DORC_CONTAINER_CLI" ]; then
         die "DORC_CONTAINER_CLI=$DORC_CONTAINER_CLI is not runnable"
      fi
      RUNTIME=$DORC_CONTAINER_CLI
      return 0
   fi
   try_runtime docker
   try_runtime podman
   try_runtime nerdctl
   try_runtime wslc
   try_runtime '/c/Program Files/WSL/wslc.exe'
   try_runtime '/mnt/c/Program Files/WSL/wslc.exe'
   [ "$RUNTIME" != "" ] || die "no container runtime found. Probed: $PROBED
     Set DORC_CONTAINER_CLI to the one you have, or install docker."
   say "runtime: $RUNTIME"
}

# Every runtime call goes through here, for one reason: git-bash rewrites arguments that look like
# unix paths into Windows ones before a native .exe sees them, which mangles what wslc is handed.
# Disabling that conversion is therefore necessary — and disabling it PROCESS-WIDE breaks dorc,
# which is also a native .exe and is relying on exactly that conversion to receive `--book`. So the
# suppression is scoped to the runtime child and nothing else.
rt() {
   MSYS_NO_PATHCONV=1 MSYS2_ARG_CONV_EXCL='*' "$RUNTIME" "$@"
}

# wslc spells container removal `remove`; the docker-lineage runtimes spell it `rm`.
rt_remove() {
   case $RUNTIME in
   *wslc*) rt remove -f "$1" >/dev/null 2>&1 || true ;;
   *) rt rm -f "$1" >/dev/null 2>&1 || true ;;
   esac
}

# ── ssh material, minted per run ─────────────────────────────────────────────────────────────

# A per-run keypair and a per-run known_hosts. Both exist to keep this script from touching the
# user's ssh state at all: a container that is recreated on the same port every run would
# otherwise poison `~/.ssh/known_hosts` with a host key that changes each time, and the recovery
# (`ssh-keygen -R '[localhost]:PORT'`) would be homework this script handed its own user.
# The product's default host-key posture is untouched; only this harness's config is scoped.
mint_ssh_material() {
   ssh-keygen -t ed25519 -N '' -q -C "$CONTAINER" -f "$RUNDIR/key" \
      || die "ssh-keygen failed"
   chmod 600 "$RUNDIR/key"
   : >"$RUNDIR/known_hosts"
   cat >"$RUNDIR/ssh_config" <<EOF
Host *
   IdentitiesOnly yes
   IdentityFile $RUNDIR/key
   UserKnownHostsFile $RUNDIR/known_hosts
   StrictHostKeyChecking accept-new
   BatchMode yes
   LogLevel ERROR
   IgnoreUnknown UseKeychain
EOF
}

ssh_to() {
   dest=$1
   shift
   ssh -F "$RUNDIR/ssh_config" -T -p "$PORT" "$dest" "$@"
}

# ── provisioning ─────────────────────────────────────────────────────────────────────────────

ensure_image() {
   say "ensuring $IMAGE"
   rt pull "$IMAGE" >"$RUNDIR/pull.log" 2>&1 \
      || die "could not pull $IMAGE — see $RUNDIR/pull.log"
}

# Port collisions are resolved by ASKING THE RUNTIME, not by pre-checking: a probe-then-bind has
# a race in it, `/dev/tcp` is a bashism this script cannot use, and every runtime already refuses
# a taken port. Let it refuse, and step.
start_container() {
   PORT=22300
   while [ "$PORT" -lt 22400 ]; do
      if rt run -d --name "$CONTAINER" -p "$PORT:22" "$IMAGE" sleep infinity \
         >"$RUNDIR/run.log" 2>&1; then
         DEST_SPEC=root@localhost:$PORT
         say "started $CONTAINER on port $PORT"
         return 0
      fi
      rt_remove "$CONTAINER"
      PORT=$((PORT + 1))
   done
   die "no free port in 22300..22399 — see $RUNDIR/run.log"
}

provision() {
   ensure_image
   start_container

   mint_ssh_material
   say "installing sshd"
   {
      printf 'set -eu\n'
      printf 'export DEBIAN_FRONTEND=noninteractive\n'
      printf 'apt-get update -qq\n'
      printf 'apt-get install -y -qq openssh-server >/dev/null\n'
      printf 'mkdir -p /root/.ssh /run/sshd\n'
      printf "printf '%%s\\\\n' '%s' > /root/.ssh/authorized_keys\n" "$(cat "$RUNDIR/key.pub")"
      printf 'chmod 700 /root/.ssh\n'
      printf 'chmod 600 /root/.ssh/authorized_keys\n'
      printf '/usr/sbin/sshd\n'
   } | rt exec -i "$CONTAINER" sh -s >"$RUNDIR/sshd.log" 2>&1 \
      || die "could not install sshd — see $RUNDIR/sshd.log"

   wait_for_ssh
}

wait_for_ssh() {
   waited=0
   while [ "$waited" -lt 30 ]; do
      if ssh_to "$DEST" 'exit 0' >/dev/null 2>&1; then
         say "ssh up at $DEST_SPEC"
         return 0
      fi
      sleep 1
      waited=$((waited + 1))
   done
   die "sshd never answered at $DEST_SPEC — see $RUNDIR/sshd.log"
}

# The two `cp` sources are relative in the book, so they must exist in the remote login
# directory. Shipped over the same ssh config the run uses, so a broken config fails here
# rather than halfway through a plan.
ship_sources() {
   for source in r26-smoke.conf r26-motd; do
      ssh_to "$DEST" "cat > /root/$source" <"$kit/$source" \
         || die "could not ship $source to $DEST_SPEC"
   done
}

# ── the loop ─────────────────────────────────────────────────────────────────────────────────

summary_of() {
   sed -n 's/.*plan-summary \(sites=[0-9]* elide=[0-9]* omit=[0-9]* guard=[0-9]* run=[0-9]*\).*/\1/p' "$1" \
      | tail -n 1
}

expect_summary() {
   got=$1
   want=$2
   which=$3
   if [ "$got" = "$want" ]; then
      say "$which plan matches baseline ($got)"
      return 0
   fi
   printf 'livetest: FAILED — %s plan does not match its hermetic baseline\n' "$which" >&2
   printf '  baseline: %s\n' "$want" >&2
   printf '  live:     %s\n' "$got" >&2
   # The single most likely way to get here on purpose: pointing `remote` at a host the book has
   # already been applied to. Saying so beats making someone diff two count strings to find out.
   if [ "$which" = pristine ] && [ "$got" = "$BASELINE_CONVERGED" ]; then
      printf '  This host is ALREADY CONVERGED — that is the converged baseline, exactly.\n' >&2
      printf '  `remote` expects a host this book has not been applied to. Use a fresh one,\n' >&2
      printf '  or run `mise run livetest` for a throwaway container.\n' >&2
   fi
   printf '  artifacts: %s\n' "$RUNDIR" >&2
   exit 1
}

plan_into() {
   out=$1
   err=$2
   "$dorc" plan --book="$BOOK" --oracle-dir "$ORACLES" \
      --host "$DEST_SPEC" --ssh-config "$RUNDIR/ssh_config" \
      --probe-timeout 120 --no-whylog >"$out" 2>"$err"
}

apply_from() {
   plan=$1
   out=$2
   err=$3
   "$dorc" apply --host "$DEST_SPEC" --ssh-config "$RUNDIR/ssh_config" \
      --plan "$plan" --apply-timeout 600 --no-whylog >"$out" 2>"$err"
}

drive_loop() {
   say "planning against the pristine world"
   plan_into "$RUNDIR/plan-1.sh" "$RUNDIR/plan-1.err" \
      || die "the first plan failed (exit $?) — see $RUNDIR/plan-1.err"
   expect_summary "$(summary_of "$RUNDIR/plan-1.err")" "$BASELINE_PRISTINE" pristine

   say "applying plan 1 (real apt-get; this is the slow part)"
   started=$(date +%s)
   status=0
   apply_from "$RUNDIR/plan-1.sh" "$RUNDIR/apply-1.out" "$RUNDIR/apply-1.err" || status=$?
   if [ "$status" -ne 0 ]; then
      printf 'livetest: FAILED — the first apply exited %s\n' "$status" >&2
      tail -n 20 "$RUNDIR/apply-1.err" >&2
      printf '  artifacts: %s\n' "$RUNDIR" >&2
      exit 1
   fi
   say "apply 1 succeeded in $(( $(date +%s) - started ))s"

   say "re-planning against the converged world"
   plan_into "$RUNDIR/plan-2.sh" "$RUNDIR/plan-2.err" \
      || die "the second plan failed (exit $?) — see $RUNDIR/plan-2.err"
   expect_summary "$(summary_of "$RUNDIR/plan-2.err")" "$BASELINE_CONVERGED" converged

   say "applying plan 2 (must be materially a no-op)"
   started=$(date +%s)
   status=0
   apply_from "$RUNDIR/plan-2.sh" "$RUNDIR/apply-2.out" "$RUNDIR/apply-2.err" || status=$?
   if [ "$status" -ne 0 ]; then
      printf 'livetest: FAILED — the second apply exited %s; a converged world must re-apply cleanly\n' "$status" >&2
      tail -n 20 "$RUNDIR/apply-2.err" >&2
      exit 1
   fi
   second=$(( $(date +%s) - started ))
   say "apply 2 succeeded in ${second}s"

   # The no-op claim, checked rather than asserted: the elided plan must not carry the install
   # lines at all, and re-applying must not have reached a package manager.
   if grep -q '^apt-get install' "$RUNDIR/plan-2.sh"; then
      die "the converged plan still carries an install line — it did not elide"
   fi
   if grep -qi 'Setting up\|Unpacking' "$RUNDIR/apply-2.out"; then
      die "the second apply installed something — the world was not converged"
   fi
}

# ── entry points ─────────────────────────────────────────────────────────────────────────────

new_rundir() {
   RUNID=$(date +%Y%m%d-%H%M%S)-$$
   RUNDIR=${TMPDIR:-/tmp}/dorc-livetest/$RUNID
   mkdir -p "$RUNDIR"
}

# The container always goes; the run directory only goes on success. A failed run's transcripts,
# shipped probe, both plans and timings are the entire reason anyone would look, and deleting them
# on the way out is how a harness makes its own failures unreadable.
teardown() {
   if [ "${LIVETEST_KEEP-}" = "1" ]; then
      say "LIVETEST_KEEP=1 — leaving $CONTAINER up and $RUNDIR in place"
      return 0
   fi
   rt_remove "$CONTAINER"
   if [ "${OUTCOME-}" = "ok" ]; then
      rm -rf "$RUNDIR"
   else
      say "artifacts kept at $RUNDIR"
   fi
}

cmd_run() {
   new_rundir
   resolve_runtime
   CONTAINER=$NAME_PREFIX-$RUNID
   DEST=root@localhost
   [ -x "$dorc" ] || die "no dorc binary at $dorc — run \`mise run build\` first"
   OUTCOME=fail
   trap 'teardown' EXIT
   provision
   ship_sources
   drive_loop
   OUTCOME=ok
   trap - EXIT
   teardown
   say "OK — probe shipped, plan matched baseline, applied for real, re-planned converged, re-applied clean"
}

cmd_target() {
   new_rundir
   resolve_runtime
   CONTAINER=$NAME_PREFIX-$RUNID
   DEST=root@localhost
   provision
   ship_sources
   printf '%s ready\n' "$DEST_SPEC"
   printf 'key:        %s\n' "$RUNDIR/key"
   printf 'ssh config: %s\n' "$RUNDIR/ssh_config"
   printf 'ssh:        ssh -F %s -p %s %s\n' "$RUNDIR/ssh_config" "$PORT" "$DEST"
   printf 'dorc:       mise run dorc -- plan --book=%s --oracle-dir %s --host %s --ssh-config %s\n' \
      "$BOOK" "$ORACLES" "$DEST_SPEC" "$RUNDIR/ssh_config"
   printf 'remove it:  mise run livetest:clean\n'
}

cmd_clean() {
   RUNDIR=${TMPDIR:-/tmp}/dorc-livetest
   mkdir -p "$RUNDIR"
   resolve_runtime
   removed=0
   names=$(rt ps -a --format json 2>/dev/null || rt ps --format json 2>/dev/null || true)
   for id in $(printf '%s' "$names" | tr ',{}' '\n\n\n' \
      | sed -n 's/.*"Names\{0,1\}" *: *"\('"$NAME_PREFIX"'[^"]*\)".*/\1/p' | sort -u); do
      rt_remove "$id"
      say "removed $id"
      removed=$((removed + 1))
   done
   if [ "$removed" -eq 0 ]; then
      say "no $NAME_PREFIX containers found"
   fi
   rm -rf "${TMPDIR:-/tmp}/dorc-livetest"
}

cmd_remote() {
   [ "$#" -ge 1 ] || die "livetest.sh remote <ssh-destination>"
   new_rundir
   DEST_SPEC=$1
   case $DEST_SPEC in
   *:*) DEST=${DEST_SPEC%:*}; PORT=${DEST_SPEC##*:} ;;
   *) DEST=$DEST_SPEC; PORT=22 ;;
   esac
   RUNTIME=
   [ -x "$dorc" ] || die "no dorc binary at $dorc — run \`mise run build\` first"
   # No key is minted here: a destination the caller supplied is reached with the caller's own
   # ssh config, which is the product's real posture. Only known_hosts is scoped, so a throwaway
   # box cannot rewrite the user's.
   : >"$RUNDIR/known_hosts"
   cat >"$RUNDIR/ssh_config" <<EOF
Host *
   UserKnownHostsFile $RUNDIR/known_hosts
   StrictHostKeyChecking accept-new
   BatchMode yes
   LogLevel ERROR
   IgnoreUnknown UseKeychain
EOF
   if [ "${LIVETEST_SSH_KEY-}" != "" ]; then
      printf '   IdentitiesOnly yes\n   IdentityFile %s\n' "$LIVETEST_SSH_KEY" >>"$RUNDIR/ssh_config"
   fi
   say "driving the loop against $DEST_SPEC (provisioning nothing, tearing down nothing)"
   ship_sources
   drive_loop
   say "OK — $DEST_SPEC survived the whole loop; artifacts in $RUNDIR"
}

case ${1-run} in
run) cmd_run ;;
target) cmd_target ;;
clean) cmd_clean ;;
remote) shift; cmd_remote "$@" ;;
*) die "unknown subcommand: $1 (want run|target|clean|remote)" ;;
esac
