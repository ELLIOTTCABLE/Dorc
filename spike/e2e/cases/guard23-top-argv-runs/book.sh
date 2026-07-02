# guard23-top-argv-runs (rul-guard-license: "Unpropagatable argv => no path reached => no
# vouch => run" — a PASSING floor). The vouch exists, but the operand is a command-
# substitution RHS: the value-plane marks PKG TOP, the check() is never evaluated, no path
# through the check-body is reached, and the witness cannot form — with or without the
# vouch, the site RUNS. This is the constprop half of the witness triple, and the tripwire
# against any build that keys the vouch on the PROVIDER (all apt-get sites) instead of on
# REACHED PATHS: such a build would guard (or worse, elide) a site whose entity it cannot
# even name. kFAIL-perform: unknown identity never elides — and never guards either (a
# guard needs an entity to check).
PKG=$(cat /etc/pkg)
apt-get install -y "$PKG"
