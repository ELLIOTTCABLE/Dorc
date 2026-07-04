# guard23-vouch-gates-elision (rul-guard-license + the Part-B elide-weld, 24D §3): a vouch now
# GATES elision. Under the weld a converged mutator elides ONLY IF a reached vouch licenses it,
# so the vouch is the sole difference between eliding and running. Two converged sites (probe:
# both hold), no poison wall between them:
#   apt-get install -y nginx — apt oracle carries a converged-vouch (apt-get.is_converged,
#                              rul24-vouch-is-verdict-authoring) ⇒ ELIDES (`true # dorc: elided …`).
#   systemctl enable nginx   — service oracle carries NO vouch ⇒ RUNS bare.
# Same convergence, same (absent) wall, opposite verdict — the vouch is the whole difference,
# shown inside one case. Contrast guard23-no-vouch-runs (no vouch ANYWHERE ⇒ every converged
# site runs; the weld closed the old vouchless-elide gap). This floor reds if a build elides the
# un-vouched systemctl site, or runs the vouched apt site.
apt-get install -y nginx
systemctl enable nginx
