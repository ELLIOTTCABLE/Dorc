# dorc-lang/v0.2
# hork: an inert package-manager stub; `: safe-across user` vouches read-only under a user shift
# (27C §2), so a sudo-wrapped `hork install` may probe in the root context.
hork__is_converged() {
   : safe-across user
   case "$1" in
   install) hork query "$2" ;;
   *) return 2 ;;
   esac
}
