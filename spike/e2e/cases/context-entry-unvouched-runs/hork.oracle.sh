# dorc-lang/v0.1
# hork: an inert package-manager stub, NO tolerance vouch — a sudo-wrapped `hork install` degrades
# to run (the default dial shifts only `tolerates:`-vouched functions) + the one-line adoption hint.
hork__is_converged() {
   case "$1" in
   install) hork query "$2" ;;
   *) return 2 ;;
   esac
}
