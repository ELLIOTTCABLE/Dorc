# sysctl oracle (`27W` §3 tier-3): a MARKERLESS verdict function. Its declining arm emits a
# `decline <class> …` line to the versioned report sink, but the format string is DYNAMIC
# (`printf "$fmt"`), so the static reader yields class None — the class is a RUNTIME fact.
sysctl__is_converged() {
   key=$1
   fmt='decline unsound %s is a write-only trigger key\n'
   case $key in
   vm.drop_caches|vm.compact_memory)
      printf "$fmt" "$key" >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   *) sysctl -n -- "$key" >/dev/null 2>&1 ;;
   esac
}
