# minimal package oracle (apt/dpkg), lifted statically by dorc.
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge kill installed
# command-keyed check(): flag-strip (pre/post-verb), bind the verb, annotate the single
# operand as `package`; `[ "$2" = "" ]` refuses a second operand. The engine threads the
# inlined call's POSITIONAL-bound argv (`apt-get install -y nginx`, `$1`→nginx) through this.
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
}
