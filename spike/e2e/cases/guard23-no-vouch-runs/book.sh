# guard23-no-vouch-runs (rul-guard-license: "No vouch => run" — the flagship's CONTROL, a
# PASSING floor that must never regress). Same shape as guard23-ternary-flagship, but the
# package oracle carries NO converged-vouch. site 0 (nginx, converged, before the wall)
# elides on probe-facts exactly as at HEAD; site 1 (hork) is the opaque wall; site 2
# (curl, converged-past-wall) has NO vouch => NO witness => runs BARE, forever. If a build
# ever mints a guard here — from the effect-map headline, from the probe verdict, from
# anything but an explicit converged-vouch — this case goes red: that is the claim-noop
# conflation (`dpkg -s nginx` passing does not make `apt-get install nginx` skippable;
# plans/233 §"The guard-license"), and silence must stay meaningless (233: silence
# neither vouches nor poisons; it merely fails to upgrade).
apt-get install -y nginx
hork wombat
apt-get install -y curl
