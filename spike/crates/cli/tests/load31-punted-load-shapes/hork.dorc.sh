# dorc-lang/v0.2
# The package the book locates through `$(dirname "$0")` — the command-substitution spelling of
# the same script-location question `${0%/*}` answers without running anything.
hork__predict() {
   verb=$1; shift
   widget : sm.dorc.Widget = "$1"
   case $verb in
      tune) hork status --tuned -- "$widget"   : sm.dorc.Widget:"$widget"@tuned ;;
   esac
}

hork__is_converged() {
   verb=$1; shift
   case $verb in
      tune) hork status --tuned -- "$1" ;;
      *) return 2 ;;
   esac
}
