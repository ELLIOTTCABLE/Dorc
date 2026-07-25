# dorc-lang/v0.2
# hork: an inert package-manager stub, NO tolerance vouch ⇒ a sudo-wrapped `hork install` degrades
# to run (default dial shifts only `tolerates:`-vouched functions) + the one-line adoption hint.
hork__is_converged() {
   case "$1" in
   install) hork query "$2" ;;
   *) return 2 ;;
   esac
}
