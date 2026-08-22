# dorc-lang/v0.2
# The sibling package the book locates from `$0`. It carries both members for `hork`: the model
# that names the cell, and the verdict that vouches for it. Both bind BELOW the unknown source, so
# the case fails whenever the havoc reaches past its own line.
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
