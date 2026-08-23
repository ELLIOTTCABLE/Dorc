# dorc-lang/v0.2
# The sibling the book locates from `$0`. Since the fold, the `.` line stays verbatim and this
# file's bytes never appear in `expected.out`; the resolution is observed only through the
# `script-relative-load-dies-slashless` diagnostic, which can fire only once the `${0%/*}` head
# has resolved.
hork__is_converged() {
   case ${1-} in
   tune) hork status --tuned -- "${2-}" ;;
   *) return 2 ;;
   esac
}
