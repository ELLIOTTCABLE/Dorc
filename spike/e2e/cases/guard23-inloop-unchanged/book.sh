# guard23-inloop-unchanged (donor: loop-members-all-converged-elides) (task-L2 item-7a, `209` brk-1(b) — the payoff):
# `for pkg in nginx curl; do apt-get install -y "$pkg"; done`. The for-var is Members-bound
# [nginx, curl] and the body references it, so the install is an EstablishMembers site: it
# evaluates the check ONCE PER MEMBER (probe records `site 0.0` nginx, `site 0.1` curl).
# BOTH members are already converged (the host reports both installed), and the site is
# self-reached (only its own per-member establishes reach it — no external writer), so the
# all-or-nothing in-loop license (item-3) lifts the render-floor: the body is substituted
# by `true` and the loop iterates twice over `true`. Run-set: EMPTY — no apt-get runs.
# THIS is the brk-1 payoff finally landed (a converged install loop fully elided).
# GUARD23 ADDITION (a PASSING floor): the package oracle here carries the strawman
# converged-vouch, and NOTHING may change — in-loop sites are outside the guard tier's
# initial reach (plans/233: "loops and multi-operand invocations, at least initially";
# per-iteration check-then-act is deferred, notes/218a u-9), and the existing MEMBER-
# granular elide machinery (task-L2, probe-fact-licensed) must not be perturbed by a
# vouch's presence. If a build reaches guards into loop bodies, or lets the vouch touch
# member elision, this case goes red — and that reach must then be argued consciously,
# not drifted into.
for pkg in nginx curl; do apt-get install -y "$pkg"; done
