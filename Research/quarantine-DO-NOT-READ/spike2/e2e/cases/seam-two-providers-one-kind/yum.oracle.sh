# yum provider for the SAME `package` kind (the 17N cross-oracle Seam). Effects only:
# the probe is supplied by the sibling apt oracle (kind is the cross-oracle anchor).
# NB this emits a spurious `oracle-missing-probe` diagnostic — the lifter checks probe-
# completeness PER FILE, not per-kind-across-files; the index is complete anyway, so
# both providers' converged installs still elide. notes/199 cluster-E (the Seam strain).
oracle_kind=package
oracle_effect yum install establish installed
oracle_effect yum remove kill installed
