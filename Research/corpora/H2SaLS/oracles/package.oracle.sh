# LLM-GENERATED ORACLE SEED — intentionally quality-varied artificial testing
# corpus for a static-analysis project (Dorc). NOT real ops code; FROZEN EVIDENCE,
# NEVER EXECUTE. An artificial oracle cannot expose the truth of real-world ops-code.
# Validation is `dash -n` (parse-only) plus reading.
#
# Models the apt-get/dpkg state-effects THIS book (harden.sh) exercises:
#   - `apt-get install -y <pkg…>`  (§1 L54 single `sudo`; §2 L130 the TWENTY-operand
#     set; §11 L640/L665 single gpg/lynis) → establishes package:<pkg>#installed
# The single-entity install/purge core mirrors the exemplar (probe `dpkg-query -W`);
# the book never PURGEs, but the purge cell is kept (exemplar parity) so a purge site
# resolves correctly rather than silently mis-resolving.
#
# ONE KIND PER FILE: this oracle is `package` ONLY. Cache-freshness (`apt-get update`)
# is a DIFFERENT kind (`pkgindex`) and belongs in its own file (cf. the spike's
# `exec-singleton-update/pkgindex.oracle.sh`); modelling it here would either conflate
# two kinds under one `oracle_kind` or bind selector `fresh` to `package` wrongly. It
# is intentionally LEFT OUT (um-pkg-3): `update` therefore resolves no effect ⇒ runs.
#
# REFUSES (→ site runs, the sound default; never a wrong elision):
#   - any install with a SECOND operand (the §2 twenty-pkg line): `[ "$2" = "" ]`
#     degrades multi-operand argv to no-probe (R2-MULTIOP). Probing only the first
#     operand would elide the whole install on a host that has pkg#1 but not pkg#20.
#   - `apt-get upgrade` (L127/L638/L663): entity = "every installed package",
#     un-enumerable — the check reaches no annotation ⇒ ⊤ ⇒ runs (um-pkg-2). There is
#     no per-entity establish to name, so no effect cell exists for it either.
#   - `apt-get update`: see the one-kind note above (um-pkg-3 / kVOLATILES).
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge   kill      installed

# command-keyed check(): the oracle's OWN argparse → inline kind-annotation (identity
# only; task-W). Flag-strip pre- and post-verb (drops `-y`), bind the verb, then branch.
# install/purge annotate the single operand `package`; the `[ "$2" = "" ]` guard refuses
# a SECOND operand so the §2 line resolves NO probe ⇒ runs (R2-MULTIOP). `update` and
# `upgrade` match no arm ⇒ no annotation reached ⇒ ⊤ ⇒ runs (their entities are not a
# single nameable `package`).
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      install|purge)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
         ;;
   esac
}
