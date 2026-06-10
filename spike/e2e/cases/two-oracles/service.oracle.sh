# minimal service oracle (systemd), lifted statically by dorc.
# NB selector mismatch (notes/193 strain-2 / F-BLESSED): `enable` gates #enabled but
# the probe reads is-active (#active). A real service oracle needs BOTH is-enabled and
# is-active; this scrappy fixture under-probes. The e2e only `sh -n`-checks (never runs)
# the rendered artifact, so the body is just syntax-checked, not executed.
oracle_kind=service
oracle_probe_service() { systemctl is-active --quiet "$1"; }
oracle_effect systemctl enable establish enabled
oracle_effect systemctl disable kill enabled
