# dorc-lang/v0.1

# ============================================================================
# 01 — systemd: system units vs `--user` units          (STRAWMAN, never run)
#
# THE REAL-WORLD FACTS. `systemctl enable nginx` talks to pid 1: enabled-ness
# is a symlink under /etc/systemd/system/*.wants/; active-ness is pid-1
# runtime state. `systemctl --user enable syncthing` talks to the INVOKING
# USER'S private manager instance: enabled-ness is a symlink under
# ~/.config/systemd/user/*.wants/; active-ness lives in that per-user manager
# (socket under /run/user/$UID). Same binary, same verbs, same
# unit-name-shaped operands — two entirely separate state universes, selected
# by an argv flag. And the classic footgun: `sudo systemctl --user status foo`
# targets ROOT'S user manager (usually not even running), not alice's.
#
# WHY NON-TRIVIAL: one tool, one noun-vocabulary, MIXED topology — the exact
# case a flat per-(kind × axis) token cannot say.
# ============================================================================

# The book lines under analysis (run as alice):
sudo systemctl enable --now nginx            # system universe, via wrapper
systemctl --user enable syncthing            # alice's universe, bare
sudo systemctl --user enable syncthing       # ROOT's universe — the footgun

# ---------------------------------------------------------------------------
# SHAPE A (minted clause). The flat token is stuck:
#
#    : sm.dorc.Service user=invariant      # WRONG for --user units
#    : sm.dorc.Service user=sensitive      # kills the sudo headline for
#                                          # system units
#
# A's only escape is to SPLIT THE KIND — and then each clause is honest:
#
#    : sm.dorc.Service      user=invariant
#    : sm.dorc.UserService  user=sensitive
#
# ...but note what A is doing: asserting both classifications blind, with
# nothing for a checker to bite on. The split itself happens in the TOOL
# oracle's binding (below), identically for both shapes.
# ---------------------------------------------------------------------------

# The tool oracle's binding arm chooses the kind from argv — this is where
# the mixed topology actually resolves, under EITHER shape:
systemctl__is_converged() {
   userflag=''
   [ "${1-}" = "--user" ] && { userflag=1; shift; }
   case "${1-}" in
   enable)
      if [ -n "$userflag" ]
      then systemctl --user is-enabled --quiet -- "${2-}"   : sm.dorc.UserService:"$2"#enabled
      else systemctl is-enabled --quiet -- "${2-}"          : sm.dorc.Service:"$2"#enabled
      fi ;;
   *) return 2 ;;
   esac
}

# ---------------------------------------------------------------------------
# SHAPE D (address-derived). One member per kind; the engine derives the
# classification from what the addresses depend on:
# ---------------------------------------------------------------------------

sm_dorc_Service__lives_at() {
   case "${2-}" in
   enabled) printf '/etc/systemd/system\n'     # wants/-symlink farm
            printf '/usr/lib/systemd/system\n' # vendor presets participate
            ;;
   active)  printf '/run/systemd\n' ;;         # pid-1 runtime — a STAND-IN;
                                               # see the fence note below
   *)       return 2 ;;
   esac
}

sm_dorc_UserService__lives_at() {
   case "${2-}" in
   enabled) printf '%s/.config/systemd/user\n' "$HOME" ;;
   active)  printf '/run/user/%s/systemd\n' "$(id -u)" ;;
   *)       return 2 ;;
   esac
}

# DERIVATION WALKTHROUGH.
#  - Service: no user-axis-owned input anywhere ⇒ INVARIANT. Line 1
#    (`sudo systemctl enable --now nginx`): probe-as-alice measured
#    Service:nginx#enabled converged; invariance bridges the alice-context
#    fact to the root-context site ⇒ elides. The 24S §2 headline survives.
#  - UserService: `$HOME` and `$(id -u)` (blessed who-am-I) ⇒ SENSITIVE ⇒
#    cells KEY per user-axis value. Line 2 probes and elides entirely within
#    alice's context (no boundary crossed). Line 3 keys to
#    UserService:syncthing @user=root — a cell nobody probed ⇒ runs. Dorc
#    didn't hide the footgun; the line runs and fails as loudly as it always
#    did (a future hint could name the mismatch — kWARN-rich says build it).
#  - Per the carve, sensitivity licenses NO disjointness: a running line-3
#    still collides with alice's UserService facts (may-alias). Safe; the
#    real independence of the two universes is value left on the table, to
#    be picked up later as a declared act if the walls annoy.
#
# THE FENCE DIVIDEND (new finding, f1-narrowed-knife). `#active`'s true home
# is pid-1 memory; `/run/systemd` is an approximation. Under the co-reference
# fence that is FINE: the address is consumed ONLY for its dependence shape
# (does who-am-I appear?), never for identity — so invariant addresses may be
# approximate. The exhaustiveness knife narrows to exactly one obligation:
# DO NOT FORGET AN AXIS-DEPENDENT STORE. (Forgetting /usr/lib/systemd above
# costs nothing; forgetting ~/.config/systemd/user in a kind you called
# invariant is the knife.)
#
# VERDICT. Both shapes force the same kind-split (kinds are the TOPOLOGY
# UNIT — extends rul-kind-or-selector-is-a-behaviour-choice's menu). D's
# version is derivable and checkable (a binding-smell lint can see a
# user-scoped probe read against an invariant kind's address-set); A's is a
# blind assertion. D strictly dominates here, modulo authoring effort.
