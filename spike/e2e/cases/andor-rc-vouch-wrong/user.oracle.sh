# A `user` oracle declaring `useradd <name>` as establishing user#present. The model
# is (provider, verb) -> effect, so `useradd deploy` reads verb=`deploy` (a Singleton
# user#present cell; the baked username is a fixture wart, not load-bearing). The oracle
# CANNOT express "useradd exits non-zero when the user already exists" — that
# non-conformance is the premise, supplied by the inert mock (exit 9). The probe body is
# never executed by the harness (notes/198 2.3); only the rendered apply runs.
oracle_kind=user
oracle_probe_user() { getent passwd "$1" >/dev/null 2>&1; }
oracle_effect useradd deploy establish present
