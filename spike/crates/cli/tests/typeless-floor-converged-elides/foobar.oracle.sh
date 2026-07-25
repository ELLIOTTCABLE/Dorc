foobar__is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs) foobar status --certs-current -- "$1" ;;
   *) return 2 ;;
   esac
}
