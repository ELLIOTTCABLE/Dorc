# dorc-lang/v0.2
# Glob member one. Both members are ordinary complete packages for their own tool, so each one's
# elision is independent evidence that the set-valued operand acquired it.
wombat__predict() {
   verb=$1; shift
   store : sm.dorc.Widget = "$1"
   case $verb in
      sync) wombat status --synced -- "$store"   : sm.dorc.Widget:"$store"@synced ;;
   esac
}

wombat__is_converged() {
   verb=$1; shift
   case $verb in
      sync) wombat status --synced -- "$1" ;;
      *) return 2 ;;
   esac
}
