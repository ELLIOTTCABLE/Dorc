# package-index freshness oracle. NB kind named `pkgindex` not `package-index`:
# a hyphen in `oracle_probe_<kind>` is not a valid POSIX function name, so the lifter
# can't bind a probe for a hyphenated kind (notes/195 F2). Hyphen-free dodges it.
oracle_kind=pkgindex
oracle_probe_pkgindex() { test -n "$(find /var/lib/apt/lists/ -maxdepth 1 -newermt '-1 hour' 2>/dev/null)"; }
oracle_effect apt-get update establish fresh
# command-keyed check(): `apt-get update` is a NULLARY verb (no operand) — the
# value-less annotation `idx : pkgindex` resolves the Singleton cell (task-W §4).
apt_get__check() {
   verb=$1; shift
   case $verb in
      update) idx : pkgindex; test -n fresh ;;
   esac
}
