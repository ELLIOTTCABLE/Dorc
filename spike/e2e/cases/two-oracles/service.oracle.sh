# minimal service oracle (systemd), lifted statically by dorc.
# This book only `enable`s (gating #enabled), so the oracle declares the matching
# per-selector probe (task-P/find-1): is-enabled discharges #enabled. (Single-selector
# here, so the kind-default rule would also permit a bare `oracle_probe_service`, but the
# per-selector form is the correct, mismatch-free shape — the strain-2 F-BLESSED gripe
# this fixture used to carry is now resolved for the selector it actually uses.)
oracle_kind=service
oracle_probe_service_enabled() { systemctl is-enabled --quiet "$1"; }
oracle_effect systemctl enable establish enabled
oracle_effect systemctl disable kill enabled
# command-keyed check(): the verb selects a different probe per arm (enable→is-enabled,
# start→is-active, disable→is-enabled); annotate the unit operand as `service`.
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" ;;
      start)   systemctl is-active  -- "$svc" ;;
      disable) systemctl is-enabled -- "$svc" ;;
   esac
}
