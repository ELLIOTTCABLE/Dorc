# guard23-ternary-flagship (rul-ternary-verdict; the {elide, guard, run} verdict map on one
# book — the guard pin-set's centerpiece; XFAIL until the guard tier lands). Four sites:
#   site 0  apt-get install nginx — converged BEFORE any wall ⇒ the elide-tier fires exactly
#           as at HEAD (attention is saved ONLY by provable elision, rul-attention-honesty;
#           the guard tier must never downgrade a provable elision — two-halves doctrine).
#   site 1  hork wombat — un-oracled ⇒ opaque ⇒ runs verbatim, and stands as the poison
#           wall: every downstream site loses its elide-license (plans/233 §0).
#   site 2  apt-get install curl — converged PAST the wall, and the package oracle carries
#           a converged-vouch ⇒ the (call-site, reached vouch, probe-verdict) witness mints
#           a GUARD (rul-guard-license): the oracle's own check body ships strip-only as a
#           preamble function and the line becomes `check-invocation || original-bytes`;
#           the original command's bytes survive verbatim (rul-ternary-verdict). At apply
#           the predict re-reads the live host (mock: curl installed, rc 0) ⇒ short-circuit
#           ⇒ the mutator is skipped by LIVE re-check, never by the stale plan verdict.
#   site 3  apt-get install vim — DIVERGED past the wall (probe: absent) ⇒ plain run, vouch
#           or no vouch: the mint is converged-past-wall only (a guard at a predicted-change
#           site buys nothing); no guard, no check-tax, the mutator runs bare.
# The probe half additionally pins the sibling the witness needs: a VOUCHED site past the
# wall still ships its read-only probe (plan-prediction and apply-guard are the same code,
# plans/233 §"The guard-license"; no probe-verdict ⇒ no witness ⇒ no guard).
apt-get install -y nginx
hork wombat
apt-get install -y curl
apt-get install -y vim
