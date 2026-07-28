#!/bin/sh
# dorc-lang/v0.2
# ═══════════════════════════════════════════════════════════════════════════
#  FROZEN EVIDENCE · STRAWMAN · IMAGINATION-TIER
#  NOT RUNNABLE. NEVER EXECUTE — not this file, not a fragment of it, not by
#  hand and not by tool. It is a design document that happens to be shaped
#  like sh.
#  Features shown here MAY NOT EXIST and may never exist. Every spelling is
#  invented for this exhibit; NO format-, flag-, kind-name-, or wire-compat
#  is promised by anything below. Real command names and real flags appear
#  ONLY so the exhibit is grounded in how the tools actually behave.
#  The only sanctioned executor of fixture material in this repo is
#  `mise run test:e2e`, and this file is not fixture material.
# ═══════════════════════════════════════════════════════════════════════════
#
#  nix-machine.sh — the glue layer around a nix machine, spelled as what
#  you'd type anyway.
#
#  Day zero this file needs nothing but `sh`: not nix, not git, not Dorc.
#  `curl … | sh` on a naked box and walk away. Day N it is a book: same
#  file, `dorc plan nix-machine.sh`, and the standup folds dead.
#
#  Every annotation rides the `#:` comment carrier (`281` §3) rather than
#  the salient colon form, for one reason stated once here: THIS book's
#  headline property is that it runs bare, on a machine that has never
#  heard of Dorc, before Dorc exists on it. A colon-form mark is not inert
#  under a stock shell; a `#:` block is a comment on every route. The
#  day-zero population is exactly the population `kTYANNOT-eol-comment`
#  was offered for. (See the companion note, §mark-carrier-choice.)

set -eu

FLAKE_DIR="$HOME/src/machine-flake"
FLAKE_URL=https://github.com/example-person/machine-flake.git


# ── 0. who am I ────────────────────────────────────────────────────────────
# The densest real-world glue idiom there is: source os-release, branch on
# ID. Two machine classes get two different convergence verbs, and the
# admin's own `case` is what says so. Nothing Dorc-specific here; this is
# just what the file would say anyway.

. /etc/os-release

case "${ID:-}" in
nixos)  MACHINE_CLASS=nixos ;;
*)      MACHINE_CLASS=foreign ;;
esac


# ── 1. privilege, bound once ───────────────────────────────────────────────
# Three independent first-tier installers compute exactly this and thread it
# through every mutating line as a prefix variable. It is an early-bound host
# fact, not a per-line decision.

if [ "$(id -u)" = 0 ]; then SUDO=
elif command -v sudo >/dev/null 2>&1; then SUDO="sudo"
elif command -v doas >/dev/null 2>&1; then SUDO="doas"
else printf 'need root, or sudo, or doas\n' >&2; exit 1
fi


# ── 2. nix itself ──────────────────────────────────────────────────────────
# On NixOS, nix is the OS and this whole region is dead by construction —
# the admin's own outer guard says so, and Dorc folds the region with an
# `omit` (a value-flow proof that the branch cannot run), not an elision.
# No per-line vouches are consumed inside an omitted region, and an omitted
# region casts no walls. That matters here more than anywhere: install
# regions sit at the TOP of books, the worst possible wall real-estate.

if [ "$MACHINE_CLASS" = foreign ]; then

   # The guard everyone writes, and the reason it is not `command -v nix`:
   # the upstream installer puts nix under /nix/var/nix/profiles/default and
   # arranges for PATH via a profile script that a non-login, non-interactive
   # shell never sources. `command -v nix` says "absent" on a box where nix
   # is perfectly well installed. Test for the daemon profile, not the name.
   if [ ! -e /nix/var/nix/profiles/default/bin/nix ]; then
      curl -fsSL https://install.determinate.systems/nix >/tmp/nix-installer.sh
      $SUDO sh /tmp/nix-installer.sh install --no-confirm
   fi

   # ... and the wart the guard above exists to survive: even immediately
   # after a successful install, THIS shell has no nix on PATH. Sourcing the
   # profile script is not optional and is not idempotence-relevant; it is
   # how the next line finds the binary at all.
   # shellcheck source=/dev/null
   . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh

fi


# ── 3. the flake ───────────────────────────────────────────────────────────
# The flake is nix's, entirely. Dorc does not read it, model it, template
# it, or have an opinion about it. What Dorc handles is getting it onto the
# box and keeping it current — the two lines that live in a README today.

if [ ! -d "$FLAKE_DIR/.git" ]; then
   git clone "$FLAKE_URL" "$FLAKE_DIR"
fi
git -C "$FLAKE_DIR" pull --ff-only


# ── 4. the delegation line ─────────────────────────────────────────────────
# One line per machine class. Both are pure delegation: nix owns everything
# past this point, and Dorc's entire job is deciding whether the line runs.
#
# This is where nix is at its strongest and Dorc at its most modest. The
# oracle below does not model what a nixos-rebuild does; it asks two
# questions that nix answers exactly, and vouches on the pair.

case "$MACHINE_CLASS" in
nixos)
   $SUDO nixos-rebuild switch --flake "$FLAKE_DIR#$(hostname)"
   ;;
foreign)
   nix run home-manager/master -- switch --flake "$FLAKE_DIR#$(id -un)@$(hostname)"
   ;;
esac


# ── 5. the residue ─────────────────────────────────────────────────────────
# Everything nix structurally cannot own. This is the part of the file that
# is nobody's product today — the README's numbered steps after "then run
# nixos-rebuild switch".

# (a) An imperative daemon enrolment. `services.tailscale.enable` declares
#     the daemon; the node's membership in a tailnet is remote, mutable,
#     authenticated state that no local closure can describe. The admin's
#     own guard is the honest one: ask the daemon.
if ! tailscale status >/dev/null 2>&1; then
   $SUDO tailscale up --ssh --accept-routes
fi

# (b) User services. nixos-rebuild's own man page says it: user services
#     "need to be started manually as they aren't detected by the activation
#     script". A closure that is fully converged still leaves this line.
systemctl --user daemon-reload
systemctl --user restart syncthing.service

# (c) Secrets. The store is world-readable; this is the one thing everyone
#     agrees does not go in a flake. It arrives out of band and lands
#     outside the store, so it is ours to guard, or nobody's.
if [ ! -s "$HOME/.config/sops/age/keys.txt" ]; then
   printf 'age key missing; fetch it from the password manager and re-run\n' >&2
   exit 1
fi

# (d) The bit nobody writes down. Garbage collection is a policy decision
#     with a wall-clock in it, which makes it exactly the class of check
#     that can never be hermetic — so it never elides, forever, and that is
#     correct rather than a gap.
$SUDO nix-collect-garbage --delete-older-than 30d


# ═══════════════════════════════════════════════════════════════════════════
#  ORACLES
#
#  Below the book, in the same file, because that is the on-ramp: the admin
#  who got annoyed at line 4 running for ninety seconds every morning puts
#  the engineer hat on for a coffee and appends this. It is still one file
#  and it still runs bare.
# ═══════════════════════════════════════════════════════════════════════════


# ── nixos-rebuild ──────────────────────────────────────────────────────────
#
# The delegation check nix makes possible and nobody else does.
#
# `nixos-rebuild switch` is documented as two acts fused: "Build and
# activate the new configuration, AND make it the boot default." `test` does
# only the first; `boot` does only the second. So a vouch for `switch` needs
# BOTH halves to hold, and a book that ran `test` yesterday must not elide
# `switch` today. That is the whole shape of the arms below.
#
# What we deliberately do NOT delegate to: `nixos-rebuild dry-activate`.
# It looks like the convergence verb — it is even documented as showing
# "what changes would be performed by the activation" — but its own man page
# ends the paragraph with "The list of changes is not guaranteed to be
# complete." An incomplete change-list cannot license not-running anything.
# This is the ansible `--check` decline (USER_STORY rung 3) with a much
# better tool on the other end and the same honest answer: the verb exists,
# it is not a convergence verb, and we say so by not using it.

nixos_rebuild__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift

   case $verb in
   switch)
      # Two halves. Half one is free: is the running system also the boot
      # default? A `readlink` each. This is what separates switch from test.
      cur=$(readlink -f /run/current-system) || return 2
      boot=$(readlink -f /nix/var/nix/profiles/system) || return 2
      [ "$cur" = "$boot" ] || return 1

      # Half two: is that closure the one this flake evaluates to? Two rungs,
      # cheap first.
      flakeref=${1:-}
      _nixos_rebuild_expected "$flakeref"
      ;;
   boot|test|dry-activate|dry-build|build)
      # `boot` and `test` each satisfy exactly one half of what our check
      # measures, and we have not authored the halves separately. Declining
      # is ordinary control flow, and a decline runs the line.
      return 2
      ;;
   *)
      printf 'decline unmodeled nixos-rebuild verb: %s\n' "$verb" \
         >>"${DREP_V1:-/dev/null}"
      return 2
      ;;
   esac
}

# Rung one: the cheap check, available only to admins who opted in by
# setting `system.configurationRevision = self.rev` in their flake — the
# convention nix's own templates ship. When the working tree is clean, the
# top-level git revision covers the entire evaluated input set INCLUDING
# flake.lock, so revision equality implies closure equality. When the tree
# is dirty, `self.rev` is null, the running system's recorded revision is
# meaningless, and we decline rather than guess.
#
# Rung two: evaluate the closure and compare store paths outright. Slower
# (a NixOS closure evaluation, seconds), total, and the soundest convergence
# check any incumbent offers — a nix store path is a complete function of the
# evaluated inputs, so path equality is not a heuristic about the system, it
# IS the system's identity.
#
# The hermeticity precondition is `flake.lock`, and it is why the local-path
# gate below is not fussiness: a remote flakeref (`github:me/cfg#host`)
# re-resolves its inputs on every evaluation, so two evaluations minutes
# apart can honestly disagree. A locked local path is reproducible by
# construction. `kVOLATILES-exclude` says hermeticity is a precondition for
# any sound skip system; nix hands it to us, but only through the lock.

_nixos_rebuild_expected() {
   dir=${1%%#*}
   attr=${1#*#}

   case $dir in
   /*|./*|../*|.) : ;;
   *) printf 'decline non-local flakeref (unlocked inputs): %s\n' "$1" \
         >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   esac
   [ -f "$dir/flake.lock" ] || {
      printf 'decline unlocked flake: %s\n' "$dir" >>"${DREP_V1:-/dev/null}"
      return 2
   }

   # rung one
   if rev=$(nixos-version --configuration-revision 2>/dev/null) \
      && [ -n "$rev" ] \
      && [ -z "$(git -C "$dir" status --porcelain 2>/dev/null)" ] \
      && head=$(git -C "$dir" rev-parse HEAD 2>/dev/null)
   then
      [ "$rev" = "$head" ]
      #: org.nixos.SystemClosure:"$attr"@activated reads org.nixos.FlakeLock:"$dir"@current
      return $?
   fi

   # rung two
   want=$(nix eval --raw "$dir#nixosConfigurations.$attr.config.system.build.toplevel") \
      || return 2
   [ "$(readlink -f /run/current-system)" = "$want" ]
   #: org.nixos.SystemClosure:"$attr"@activated reads org.nixos.FlakeLock:"$dir"@current
}


# ── home-manager ───────────────────────────────────────────────────────────
#
# Same shape, one half instead of two: home-manager has no bootloader, so
# "activated" is the whole story. The current generation is a profile
# symlink, exactly as the system profile is.
#
# The `switch`-only vouch is not laziness. `home-manager build` produces a
# result symlink and activates nothing; `home-manager expire-generations`
# and `remove-generations` destroy state whose absence is not evidence of
# anything (the purge asymmetry from USER_STORY stage 4). Neither gets a
# yes from us, ever.

home_manager__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift

   case $verb in
   switch)
      hmprofile=${XDG_STATE_HOME:-$HOME/.local/state}/nix/profiles/home-manager
      [ -e "$hmprofile" ] || return 1
      dir=${1%%#*}
      attr=${1#*#}
      case $dir in /*|./*|../*|.) : ;; *) return 2 ;; esac
      [ -f "$dir/flake.lock" ] || return 2

      want=$(nix eval --raw "$dir#homeConfigurations.\"$attr\".activationPackage") \
         || return 2
      [ "$(readlink -f "$hmprofile")" = "$want" ]
      #: org.nixos.HomeGeneration:"$attr"@activated reads org.nixos.FlakeLock:"$dir"@current
      ;;
   news|generations|build|expire-generations|remove-generations|uninstall)
      return 2 ;;
   *)
      printf 'decline unmodeled home-manager verb: %s\n' "$verb" \
         >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   esac
}


# ── the footprints ─────────────────────────────────────────────────────────
#
# What a running nix activation is allowed to have disturbed. These are the
# at-most claims that let a converged line below the delegation survive a
# drifted morning where the delegation really runs — the `kSURVIVAL` tier,
# and they are opt-in on both ends (the author writing them, the admin
# typing `--risk-faultless-skips`).
#
# They are also where I would most expect a nix oracle to be WRONG, and the
# note says so at length. A system activation restarts systemd units, so its
# footprint reaches service state; it rewrites /etc through the activation
# script, so it reaches config files; and there is no honest way to bound
# either from the outside. So the claim below is deliberately WIDE. A wide
# at-most claim buys nothing (it collides with everything downstream) and
# risks nothing. That asymmetry is the correct posture for an oracle
# describing somebody else's whole-system activation.

nixos_rebuild__disturbs() {
   while [ "${1#-}" != "$1" ]; do shift; done
   case $1 in
   switch|boot|test)
      printf '%s\n' "$(hostname)"
      #: disturbs {org.nixos.SystemClosure,sm.dorc.Service,sm.dorc.File}
      ;;
   esac
}

home_manager__disturbs() {
   while [ "${1#-}" != "$1" ]; do shift; done
   case $1 in
   switch)
      printf '%s\n' "$HOME"
      #: disturbs {org.nixos.HomeGeneration,sm.dorc.File}
      ;;
   esac
}


# ── tailscale ──────────────────────────────────────────────────────────────
#
# The residue's one modelable line, and the point of including it: the
# residue is not a lost cause, it is just unowned. `tailscale status` is a
# read-only membership query with a documented exit code, which is more
# than most residue offers.
#
# It declines `up` with an auth key, though — a key is a credential whose
# validity is remote and time-bounded, so "we are currently a member" does
# not establish "re-running this is noise I accept". That is an authored
# judgment about somebody else's tool, which is the whole job.

tailscale__is_converged() {
   verb=${1:-}; shift 2>/dev/null || :
   case $verb in
   up)
      for a in "$@"; do
         case $a in
         --authkey=*|--auth-key=*)
            printf 'decline authkey-bearing tailscale up\n' >>"${DREP_V1:-/dev/null}"
            return 2 ;;
         esac
      done
      tailscale status --json >/dev/null 2>&1
      #: org.tailscale.Node:"$(hostname)"@joined
      ;;
   status|version) return 2 ;;
   down|logout) return 2 ;;
   *) return 2 ;;
   esac
}
