# dorc-lang/v0.2
# Glob member two, describing a different tool: an ordered population has to acquire BOTH members,
# and a case that only ever needed the first could not tell an ordered set from a lucky singleton.
zork__predict() {
   verb=$1; shift
   cache : sm.dorc.Widget = "$1"
   case $verb in
      prime) zork status --primed -- "$cache"   : sm.dorc.Widget:"$cache"@primed ;;
   esac
}

zork__is_converged() {
   verb=$1; shift
   case $verb in
      prime) zork status --primed -- "$1" ;;
      *) return 2 ;;
   esac
}
