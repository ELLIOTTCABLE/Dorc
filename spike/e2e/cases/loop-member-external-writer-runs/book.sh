# loop-member-external-writer-runs (task-L2 item-7c, `209` brk-1(b) — the self-reach core):
# a PRE-LOOP `apt-get purge curl` writes `package:curl#installed` — a cell the loop's
# curl-member establishes. So the in-loop install site is NOT self-reached (item-3(b)): a
# NON-self writer (the purge) reaches it, breaking the fixed-point argument (the elision's
# own effect no longer accounts for all writes). The license REFUSES even though BOTH members
# are reported converged (the bait). The loop body RUNS both iterations. Run-set: the purge
# + both installs. (Pin: convergence alone does NOT license an in-loop members elision; the
# self-reach carve-out is the load-bearing guard.)
apt-get purge -y curl
for pkg in nginx curl; do apt-get install -y "$pkg"; done
