# dorc-lang/v0.1
# hork: an inert package-manager stub; `:   : tolerates:user` vouches read-only under a user shift
# (27C §2), so a sudo-wrapped `hork install` may probe in the root context.
hork__is_converged() {
   :   : tolerates:user
   case "$1" in
   install) hork query "$2" ;;
   *) return 2 ;;
   esac
}
