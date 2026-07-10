# dorc-lang/v0.1

# ============================================================================
# 02 — git config: --system / --global / --local        (STRAWMAN, never run)
#
# THE REAL-WORLD FACTS. `git config --system` writes /etc/gitconfig;
# `--global` writes $HOME/.gitconfig (so `sudo git config --global` silently
# edits ROOT's gitconfig — a classic footgun); `--local` writes
# $REPO/.git/config, i.e. cwd-scoped, not user-scoped at all.
#
# WHY NON-TRIVIAL: it looks like a three-way mixed-topology kind (worse than
# systemd!) — but worked through, it mostly DISSOLVES, and where it dissolves
# to is the interesting part.
# ============================================================================

# The book lines under analysis (run as alice in /srv/app):
sudo git config --system core.autocrlf false     # /etc/gitconfig
git config --global user.email alice@example.net # /home/alice/.gitconfig
sudo git config --global user.email oops@nope    # /root/.gitconfig — footgun
git -C /srv/app config --local receive.denyNonFastForwards true

# ---------------------------------------------------------------------------
# THE DISSOLVING MOVE: bind ADDRESS-SHAPED entities. git's oracle author
# never mints a GitConfig kind at all — the scope flag selects a FILE PATH,
# and the path (resolved under the site's ρ) IS the entity:
# ---------------------------------------------------------------------------

git__is_converged() {
   scope_file=''
   while [ $# -gt 0 ]; do
      case "$1" in
      -C) cd -- "$2" || return 2; shift 2 ;;         # cwd is a ρ component
      config) shift; break ;;
      *) return 2 ;;                                  # other verbs: not today
      esac
   done
   case "${1-}" in
   --system) scope_file=/etc/gitconfig; shift ;;
   --global) scope_file="$HOME/.gitconfig"; shift ;;
   --local)  scope_file="$(pwd)/.git/config"; shift ;;
   *) return 2 ;;
   esac
   key="${1-}"; want="${2-}"
   [ "$(git config --file "$scope_file" --get "$key")" = "$want" ] \
      : sm.dorc.File:"$scope_file"#"key_$key"
}

# ANALYSIS WALKTHROUGH — watch ρ do the work the trichotomy was hired for:
#  - Line 1 (`sudo ... --system`): entity resolves to /etc/gitconfig under
#    ANY ρ — same spelling both sides of the boundary. The remaining question
#    ("same path, alice-context vs root-context: same cell?") is not git's
#    question at all — it is the BOOTSTRAP FILE KIND's user-axis topology,
#    declared ONCE, by us, in the stdlib.
#  - Line 2: entity = /home/alice/.gitconfig (ρ: HOME=/home/alice). Probed and
#    applied in one context; nothing crosses.
#  - Line 3: under sudo's declared ρ-scrub, HOME=/root ⇒ entity =
#    /root/.gitconfig ⇒ a DIFFERENT CELL from anything alice's probe measured
#    ⇒ no transport ⇒ the line runs. The footgun is not hidden, and no kind
#    ever declared "sensitive" — the VALUE PLANE keyed the cells correctly,
#    for free (opaques7-finding3: env reaches coordinates only through
#    values).
#  - Line 4: entity = /srv/app/.git/config via the cwd ρ-component. cwd is
#    not a coordinate axis and never needed to be — absorbed the same way.

# ---------------------------------------------------------------------------
# WHAT REMAINS FOR EACH SHAPE: only the bootstrap file kind, once, ever.
#
# SHAPE A:   : sm.dorc.File user=invariant
# SHAPE D:
sm_dorc_File__lives_at() { printf '%s\n' "$1" ;}     # the identity function
#
# D's version is DEGENERATE — a function that echoes its argument is pure
# ceremony (the boilerplate-cargocult razor bites). The honest resolution:
# the engine SUPPLIES the grounding for the bootstrap file-ish kinds
# (entity-is-the-address is engine knowledge, not authored knowledge), and
# neither shape's spelling is exercised here at all.
# ---------------------------------------------------------------------------

# VERDICT (the biggest finding of the whole exercise, f3). A LARGE CLASS of
# tools needs NO kind topology whatsoever: bind resolved addresses as
# file-kind entities and let ρ key the cells. The topology machinery is only
# needed where entities are LOGICAL — names that hide their store behind a
# binary (package:nginx, cron entries, service units). Consequences:
#  - the authored topology surface shrinks to: (a) one engine-shipped
#    declaration for the bootstrap file kind, (b) logical-entity kinds only;
#  - "address-entity vs logical-entity" becomes a new line in the
#    behaviour-menu (rul-kind-or-selector-is-a-behaviour-choice): logical
#    entities buy cross-tool collaboration (dpkg and apt-get sharing
#    package:nginx) at the price of needing owned topology; address entities
#    are collaboration-poor but topology-free;
#  - and a lint writes itself: an oracle minting a logical kind whose every
#    bind is a resolvable path should be nudged toward file-entities.
