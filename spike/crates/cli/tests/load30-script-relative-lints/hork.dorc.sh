# dorc-lang/v0.2
# The sibling the book locates from `$0`. It binds only if the `${0%/*}` head really resolved, so
# the case observes the RESOLUTION as well as the lint.
hork__is_converged() {
   case ${1-} in
   tune) hork status --tuned -- "${2-}" ;;
   *) return 2 ;;
   esac
}
