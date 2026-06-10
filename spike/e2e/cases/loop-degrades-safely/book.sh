# loop-degrades-safely (19I floor / 20B §3 boundary strawman): a `for` loop mid-stream is
# OUTSIDE the modeled subset ⇒ the parser ⊤-rejects it (inv-top-reject) and the cfg marks
# it an absorbing ⊤ node. ⊤ poisons both neighbours: the install ABOVE has the ⊤ loop as
# its CFG successor (⊤-containment, 16G hole-5 ⇒ no elision even though it probes converged
# — the bait), and the install BELOW is poisoned to unresolvable by the havoc. BOTH run
# VERBATIM — the SAFE degrade (kFAIL-perform): an unmodeled construct never silently
# licenses elision. The run-set proves everything (incl. the loop body) runs.
apt-get install -y nginx
for x in a b; do apt-get install -y "$x"; done
apt-get install -y curl
