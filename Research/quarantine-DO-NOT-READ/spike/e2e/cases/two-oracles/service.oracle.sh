# minimal service oracle (systemd), lifted statically by dorc.
oracle_kind=service
oracle_probe_service() { systemctl is-active --quiet "$1"; }
oracle_effect systemctl enable establish
oracle_effect systemctl disable kill
