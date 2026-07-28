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

   # The guard everyone writes, and the reason it is NOT `command -v nix`:
   # a multi-user install puts nix under /nix/var/nix/profiles/default and
   # arranges PATH by appending a `. …/nix-daemon.sh` snippet to /etc/bashrc,
   # /etc/profile.d/nix.sh, /etc/zshrc and friends — none of which a
   # non-login, non-interactive shell sources. `command -v nix` says "absent"
   # on a box where nix is perfectly well installed.
   #
   # So the guard tests the exact file the next line sources. One fact, one
   # guard, no gap between what was proven and what is used.
   NIX_PROFILE_SH=/nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh

   if [ ! -e "$NIX_PROFILE_SH" ]; then
      # Download-then-exec rather than `curl | sh`, for the reason five of
      # six comparable tools give: the script keeps its own stdin, so an
      # interior prompt cannot eat the script's remaining bytes.
      #
      # And it is the Determinate installer rather than upstream's for two
      # boring reasons that both matter to a book: flakes are enabled at
      # install time (upstream still ships them off, and formally
      # experimental), and it is the only one of the two with an automated
      # uninstall — upstream's uninstall path is literally commented out in
      # install-multi-user.sh, which instead PRINTS a multi-step runbook.
      # A tool with no uninstall is a tool a book cannot converge backwards.
      curl -fsSL https://install.determinate.systems/nix >/tmp/nix-installer.sh
      $SUDO sh /tmp/nix-installer.sh install --no-confirm
   fi

   # ... and the wart the guard above exists around, in the installer's own
   # words: "Nix won't work in active shell sessions until you restart them."
   # Sourcing is not optional and is not idempotence-relevant; it is how the
   # next line finds the binary at all. Everyone writing this script for the
   # first time gets bitten here, and no amount of guard-lifting helps —
   # this line is not a mutation and has nothing to converge.
   # shellcheck source=/dev/null
   . "$NIX_PROFILE_SH"

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
#     authenticated state that no local closure can describe. NixOS closes
#     part of the gap — set `authKeyFile` and the module generates a
#     `tailscaled-autoconnect.service` that runs `tailscale up` for you —
#     but the module's own option documentation states the residue outright
#     for the routing features: "you will still need to call
#     `sudo tailscale up` with the relevant flags". That sentence, in the
#     incumbent's own docs, is the whole thesis of this book in one line.
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
   verb=${1:-}; shift 2>/dev/null || :

   # Walk the flags we care about. The argparse IS the type-checker of the
   # vouch, so what it refuses matters as much as what it parses.
   flakeref=/etc/nixos
   while [ $# -gt 0 ]; do
      case $1 in
      --flake) flakeref=${2:-}; shift 2 || break ;;
      --flake=*) flakeref=${1#--flake=}; shift ;;

      # THE decline that matters. `--target-host` and `--build-host` move
      # the work to another machine over ssh. Our whole check reads
      # /run/current-system and /nix/var/nix/profiles/system — on the
      # machine the check runs on. Under --target-host those are the
      # CONTROLLER's, and answering with them would be a measurement of a
      # different machine than the one the line acts on: the wrong-world
      # verdict, which is the cardinal-sin shape.
      #
      # Worth noticing what this is, structurally: an incumbent shipping a
      # second host-scope inside one command, in a book that otherwise
      # addresses one machine. That is the attribution-scope re-entry
      # trigger arriving from a direction nobody was watching.
      --target-host|--build-host)
         printf 'decline nixos-rebuild %s (second host scope)\n' "$1" \
            >>"${DREP_V1:-/dev/null}"
         return 2 ;;
      --target-host=*|--build-host=*)
         printf 'decline nixos-rebuild second host scope\n' >>"${DREP_V1:-/dev/null}"
         return 2 ;;

      # Anything that makes the evaluation non-reproducible.
      --override-input|--impure|--recreate-lock-file|--no-write-lock-file|--upgrade|--upgrade-all|--rollback)
         printf 'decline nixos-rebuild %s (unpinned evaluation)\n' "$1" \
            >>"${DREP_V1:-/dev/null}"
         return 2 ;;

      *) shift ;;
      esac
   done

   case $verb in
   switch)
      # Half one is nearly free, and it is the entire difference between
      # `switch` and `test`: is the running system ALSO the boot default?
      # `nixos-rebuild` advances /nix/var/nix/profiles/system for switch and
      # boot only — never for test — so after a `test` these two symlinks
      # disagree, and a book that elided `switch` on the strength of
      # yesterday's `test` would leave a machine that reverts on reboot.
      cur=$(readlink -f /run/current-system) || return 2
      bootdef=$(readlink -f /nix/var/nix/profiles/system) || return 2
      [ "$cur" = "$bootdef" ] || return 1

      # Half two: is that closure the one this flake evaluates to?
      _nixos_rebuild_expected "$flakeref"
      ;;
   boot|test)
      # Each satisfies exactly one half of what we measure. We have not
      # authored the halves separately, so we decline rather than answer a
      # narrower question with a wider check. Declining is ordinary control
      # flow, and a decline runs the line.
      return 2 ;;
   dry-activate|dry-build|build|build-vm|build-vm-with-bootloader|list-generations|edit|repl)
      return 2 ;;
   *)
      printf 'decline unmodeled nixos-rebuild verb: %s\n' "$verb" \
         >>"${DREP_V1:-/dev/null}"
      return 2 ;;
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
# (a NixOS closure evaluation, seconds and climbing — nixpkgs eval time is a
# known, tracked, worsening cost), total, and the soundest convergence check
# any incumbent offers.
#
# PRECISION, because the loose version of this claim is wrong and would
# embarrass us: nix is INPUT-addressed by default, not content-addressed. An
# ordinary derivation's store path is a hash over the derivation's inputs —
# the recipe — not over the bytes it produced. (Content-addressed store
# objects exist, but only for fixed-output derivations and for the
# still-experimental `ca-derivations` feature.) That is FINE for us, and in
# one respect better: input-addressing is precisely a statement about the
# recipe, and "was this system built from this recipe" is exactly the
# question a convergence check should ask. But we must not say
# "content-addressed"; the tool's own glossary is explicit and someone will
# check.
#
# Either way the property we want holds: path equality is not a heuristic
# about the system, it IS the system's identity.
#
# The hermeticity precondition is `flake.lock`, and it is why the local-path
# gate below is not fussiness: a remote flakeref (`github:me/cfg#host`)
# re-resolves its inputs on every evaluation, so two evaluations minutes
# apart can honestly disagree. A locked local path is reproducible by
# construction. `kVOLATILES-exclude` says hermeticity is a precondition for
# any sound skip system; nix hands it to us, but only through the lock.

_nixos_rebuild_expected() {
   dir=${1%%#*}
   case $1 in *#*) attr=${1#*#} ;; *) attr=$(hostname) ;; esac

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

   # The rc partition, used as a control-flow primitive rather than
   # described: rung one answers 0 or 1 authoritatively when it can, and 2
   # when it cannot. Only a 2 falls through to the expensive rung. An author
   # who wrote `_rev_match || <rung two>` would have thrown away the
   # difference between "diverged" and "cannot say", and rung two would run
   # every time the revisions honestly differed.
   _nixos_rebuild_rev_match "$dir"; rung1=$?
   [ "$rung1" -ge 2 ] || return "$rung1"

   want=$(nix eval --raw "$dir#nixosConfigurations.$attr.config.system.build.toplevel") \
      || return 2
   [ "$(readlink -f /run/current-system)" = "$want" ]   #: org.nixos.SystemClosure:"$attr"@activated reads org.nixos.FlakeLock:"$dir"@current
}

# Rung one, factored out so its last statement is the marked one — the strip
# law wants the author's last substantive command to stay the last
# status-affecting statement, and a mark that has to reach around a `return`
# is a sign the function wanted splitting anyway.
_nixos_rebuild_rev_match() {
   dir=$1
   rev=$(nixos-version --configuration-revision 2>/dev/null) || return 2
   [ -n "$rev" ] || return 2
   [ -z "$(git -C "$dir" status --porcelain 2>/dev/null)" ] || return 2
   head=$(git -C "$dir" rev-parse HEAD 2>/dev/null) || return 2
   [ "$rev" = "$head" ]   #: org.nixos.SystemClosure:@activated reads org.nixos.FlakeLock:"$dir"@current
}


# ── home-manager ───────────────────────────────────────────────────────────
#
# Same shape, one half instead of two: home-manager has no bootloader, so
# "activated" is the whole story. The current generation is a profile
# symlink, exactly as the system profile is — but it is NOT the profile
# most people think of. Home Manager keeps two: the PACKAGE profile
# (`~/.nix-profile`, or `/etc/profiles/per-user/<u>` under the NixOS module)
# and the GENERATION profile, `$XDG_STATE_HOME/nix/profiles/home-manager`,
# which is the one `writeBoundary` advances and therefore the only one that
# answers "which configuration is live". Reading the wrong one gives a
# confident answer to a different question.
#
# The `switch`-only vouch is not laziness. `build` activates nothing;
# `expire-generations` and `remove-generations` destroy state whose absence
# is not evidence of anything (the purge asymmetry). And `test` is a live
# footgun worth declining loudly rather than silently: Home Manager's CLI
# ACCEPTS `test` in its argument parser and then rejects it at dispatch with
# a generic "Unknown command" — the implementation is commented out. A tool
# that parses a verb it will not run is exactly why capability-probing has
# to probe behaviour rather than names.

home_manager__is_converged() {
   verb=${1:-}; shift 2>/dev/null || :

   case $verb in
   switch)
      hmprofile=${XDG_STATE_HOME:-$HOME/.local/state}/nix/profiles/home-manager
      [ -e "$hmprofile" ] || hmprofile=/nix/var/nix/profiles/per-user/$(id -un)/home-manager
      [ -e "$hmprofile" ] || return 1

      flakeref=
      while [ $# -gt 0 ]; do
         case $1 in
         --flake) flakeref=${2:-}; shift 2 || break ;;
         --flake=*) flakeref=${1#--flake=}; shift ;;
         -n|--dry-run) return 2 ;;
         *) shift ;;
         esac
      done
      [ -n "$flakeref" ] || return 2

      dir=${flakeref%%#*}
      case $flakeref in *#*) attr=${flakeref#*#} ;; *) return 2 ;; esac
      case $dir in /*|./*|../*|.) : ;; *) return 2 ;; esac
      [ -f "$dir/flake.lock" ] || return 2

      want=$(nix eval --raw "$dir#homeConfigurations.\"$attr\".activationPackage") \
         || return 2
      [ "$(readlink -f "$hmprofile")" = "$want" ]   #: org.nixos.HomeGeneration:"$attr"@activated reads org.nixos.FlakeLock:"$dir"@current
      ;;
   news|generations|build|test|rollback|expire-generations|remove-generations|uninstall)
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
   case ${1:-} in
   switch|boot|test)
      host=$(hostname)
      printf '%s\n' "$host"   #: disturbs {org.nixos.SystemClosure,sm.dorc.Service,sm.dorc.File}
      ;;
   esac
}

home_manager__disturbs() {
   case ${1:-} in
   switch)
      printf '%s\n' "$HOME"   #: disturbs {org.nixos.HomeGeneration,sm.dorc.File}
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
         --authkey=*|--auth-key=*|--auth-key)
            printf 'decline authkey-bearing tailscale up\n' >>"${DREP_V1:-/dev/null}"
            return 2 ;;
         esac
      done
      # Bound to a variable rather than written inline: a coordinate's
      # entity is resolved through value-flow, never expanded, so a bare
      # `$(hostname)` in entity position is not something the analyzer can
      # follow. The bind is where the value gets a name.
      node=$(hostname)
      tailscale status --json >/dev/null 2>&1   #: org.tailscale.Node:"$node"@joined
      ;;
   status|version|down|logout) return 2 ;;
   *) return 2 ;;
   esac
}
