# loop-members-all-converged-elides (task-L2 item-7a, `209` brk-1(b) — the payoff):
# `for pkg in nginx curl; do apt-get install -y "$pkg"; done`. The for-var is Members-bound
# [nginx, curl] and the body references it, so the install is an EstablishMembers site: it
# evaluates the check ONCE PER MEMBER (probe records `site 0.0` nginx, `site 0.1` curl).
# BOTH members are already converged (the host reports both installed), and the site is
# self-reached (only its own per-member establishes reach it — no external writer), so the
# all-or-nothing in-loop license (item-3) lifts the render-floor: the body is substituted
# by `true` and the loop iterates twice over `true`. Run-set: EMPTY — no apt-get runs.
# THIS is the brk-1 payoff finally landed (a converged install loop fully elided).
for pkg in nginx curl; do apt-get install -y "$pkg"; done
