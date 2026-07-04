# package-index freshness oracle (kind `pkgindex`), a nullary-verb Singleton: `apt-get
# update` resolves the Singleton cell via the empty-entity mark `: pkgindex:.fresh`.
# command-keyed predict(): `apt-get update` is a NULLARY verb (no operand) — the
# value-less annotation `idx : pkgindex` resolves the Singleton cell (task-W §4).
# The effect mark `: pkgindex:.fresh` carries the Singleton's EMPTY entity slot
# (jc-singleton-mark): kind `pkgindex`, the-one entity, selector `fresh`.
apt_get__predict() {
   verb=$1; shift
   case $verb in
      update) idx : pkgindex; test -n fresh : pkgindex:.fresh ;;
   esac
}

# THE VOUCH (elide-weld, 24D §3): vouches update (singleton establish); declines unknown.
apt-get.is_converged() {
   verb=$1; shift
   case $verb in
      update) test -n fresh ;;
      *) return 2 ;;
   esac
}
