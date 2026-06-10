# exec-query-after-mutator-runs (rule-query-validity, 205 §2 / 20A §4 st-3 — the
# invalidation pin): the SAME Query-guard idiom as exec-query-guard-composition, but
# now BELOW an oracled mutator. `apt-get install -y curl` establishes
# package:curl#installed — a WRITE that reaches the `command -v nginx` guard from entry
# ⇒ the guard is an INVALID Query (its resting rc is stale: a mutator ran). So the
# firewall WITHHOLDS the guard's rc (status ⊤):
#   - the fold cannot resolve the `||` (the guard rc is ⊤) ⇒ the nginx install stays
#     LIVE (runs), never folded dead;
#   - the guard itself is StatusRelaxable-consumed with a ⊤ rc ⇒ no license ⇒ it RUNS for
#     real at apply (re-observing the possibly-changed state — kFAIL-perform).
# The run-set proves it: BOTH installs run (curl, and nginx — the latter's presence is
# the proof the guard did NOT fold it). Contrast exec-query-guard-composition, where the
# same guard with NOTHING upstream is valid and DOES fold the install. (The guard
# `command -v` is a shell builtin, so it logs no `ran:` line — its running-for-real is
# witnessed by the nginx install NOT being omitted.) HOST: curl installs fine; nginx
# absent on PATH.
set -e
apt-get install -y curl
command -v nginx >/dev/null 2>&1 || apt-get install -y nginx
