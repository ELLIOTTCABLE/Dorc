# dorc-lang/v0.2
# The sibling package the book sources relatively. Its bytes reach no artifact: the `cd` above the
# `.` means Dorc cannot say which file that operand names on the target.
hork__is_converged() {
   case ${1-} in
   tune) hork status --tuned -- "${2-}" ;;
   *) return 2 ;;
   esac
}
