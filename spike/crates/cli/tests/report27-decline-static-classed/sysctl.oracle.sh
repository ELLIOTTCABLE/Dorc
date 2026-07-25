# sysctl oracle (`27W:rul-strawman-tool-set`): a MARKERLESS verdict function. Its declining arms
# emit a `decline <class> …` line to the versioned report sink (total off-Dorc via the
# `:-/dev/null` default). vm.drop_caches is a write-only trigger key — unprobeable by construction.
sysctl__is_converged() {
   key=$1
   case $key in
   vm.drop_caches|vm.compact_memory)
      printf 'decline unsound %s is a write-only trigger key\n' "$key" >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   *) sysctl -n -- "$key" >/dev/null 2>&1 ;;
   esac
}
