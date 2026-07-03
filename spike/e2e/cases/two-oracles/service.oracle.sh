# minimal service oracle (systemd), lifted statically by dorc.
# This book only `enable`s (gating #enabled); the predict's `enable` arm probes with
# is-enabled, discharging #enabled — the correct, mismatch-free shape for the selector
# it actually uses.
# command-keyed predict(): the verb selects a different probe per arm (enable→is-enabled,
# start→is-active, disable→is-enabled); annotate the unit operand as `service`.
systemctl__predict() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
      start)   systemctl is-active  -- "$svc" ;;
      disable) systemctl is-enabled -- "$svc" : service:"$svc".enabled! ;;
   esac
}
