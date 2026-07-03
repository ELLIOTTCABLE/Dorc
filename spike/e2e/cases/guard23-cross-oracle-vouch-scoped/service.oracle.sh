# service oracle (systemd) — the cross-oracle-scoping fixture's UNVOUCHED oracle B (23C-fd9).
# Models `enable` (establishes service:<unit>#enabled) but carries NO converged-vouch line, so
# even converged and past a wall its sites never mint a guard (rul-guard-license: no vouch ⇒
# run; the vouch is a mark on a path through THIS oracle's own check-body, and this oracle marks
# none). It exists to prove the package oracle's (A's) vouch NEVER licenses a guard on this
# oracle's site — a build keying "a vouch exists in the set" / provider-set membership rather
# than THIS-site's-oracle's-reached-path would wrongly guard `systemctl enable foo` off the apt
# vouch. There is deliberately no converged-vouch here.
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
   esac
}
