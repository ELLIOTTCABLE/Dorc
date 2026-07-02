# guard23-cross-oracle-vouch-scoped (23C-fd9: a vouch is scoped to its OWN oracle's reached
# path, never to the oracle SET — a PASSING floor). Two oracles past one wall:
#   apt-get install -y nginx  — package oracle A, VOUCHED, converged BEFORE the wall ⇒ elides
#                               at HEAD (A's vouch is present and "active" on a real site)
#   hork wombat               — opaque wall
#   systemctl enable foo      — service oracle B, UNVOUCHED, past the wall ⇒ runs BARE, forever
# The pin: B's site must RUN. A guard mints only from a matching (call-site, reached converged-
# vouch, probe-verdict) witness where the vouch is a mark on a path through THIS site's oracle's
# check-body (rul-guard-license); A's apt vouch is inadmissible in B's reasoning (it never
# enters the fact-plane, and provider-set membership is not a license). A build that keys "some
# vouch exists in the -o set" instead of this-site's-oracle's-reached-path would mint a B-guard
# off A's vouch — this floor reds if it does (an extra `systemctl is-enabled` check appears, or
# the enable is suppressed). rundelta pins verb-scope within ONE oracle; no-vouch-runs pins the
# zero-vouch case; this pins the CROSS-oracle scope the other two leave open.
apt-get install -y nginx
hork wombat
systemctl enable foo
