# package-index freshness oracle. NB kind named `pkgindex` not `package-index`:
# a hyphen in `oracle_probe_<kind>` is not a valid POSIX function name, so the lifter
# can't bind a probe for a hyphenated kind (notes/195 F2). Hyphen-free dodges it.
oracle_kind=pkgindex
oracle_probe_pkgindex() { test -n "$(find /var/lib/apt/lists/ -maxdepth 1 -newermt '-1 hour' 2>/dev/null)"; }
oracle_effect apt-get update establish fresh
