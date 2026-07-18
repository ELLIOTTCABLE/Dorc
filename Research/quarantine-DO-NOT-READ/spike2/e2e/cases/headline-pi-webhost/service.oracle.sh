# service oracle (systemd). enable->#enabled, start->#active (distinct selectors).
# A real oracle needs is-enabled AND is-active (notes/193 strain-7); this under-probes.
oracle_kind=service
oracle_probe_service() { systemctl is-active --quiet "$1"; }
oracle_effect systemctl enable establish enabled
oracle_effect systemctl start establish active
oracle_effect systemctl disable kill enabled
