# loop-members-partial-runs (task-L2 item-7b, `209` brk-1(b) — the all-or-nothing floor):
# `for pkg in nginx curl; do apt-get install -y "$pkg"; done`. nginx is converged but curl
# is DIVERGED (the host reports curl absent). The in-loop Members license is all-or-nothing
# (item-3(a)): a single non-converged member refuses the WHOLE leaf — partial-member elision
# (rewriting the list to just the diverged members) is a recorded LATER direction, not this
# slice. So the body RUNS every iteration: the run-set is BOTH installs (nginx AND curl).
for pkg in nginx curl; do apt-get install -y "$pkg"; done
