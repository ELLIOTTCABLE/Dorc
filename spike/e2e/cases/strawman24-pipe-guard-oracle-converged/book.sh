# strawman24-pipe-guard-oracle-converged (XFAIL — the OWED VALUE of the round-25 pipe-guard).
# The field-trial flagship line + the strongest plausible authored oracle (otelcol's own
# author writes predict() + is_converged(), the full role-split). HOST: converged — otelcol
# IS 0.155.0, so the version-check matches and the admin's own `||` short-circuits the
# install at runtime.
#
# OWED (the assertion this pin makes, EXPECTED to fail at HEAD): a described tool on a
# converged host should let this line LIFT — exactly as the single-command
# `dpkg -s otelcol || curl|tar` already elides (sibling strawman24-pipe-guard-single-lifts).
# The desired run-set is EMPTY (the whole no-op line elides, saving the check-tax + attention,
# USER_STORY Stage 1). expected.ran encodes that desired-future.
#
# WHY IT CANNOT LIFT AT HEAD (the gap; NO engine change in this round — pin, don't fix):
#   - The `||` reads the check-pipeline's GOVERNING status = the LAST stage, `grep -q` — a
#     generic text filter no oracle models. The author's tool `otelcol` is the NON-last stage:
#     its stdout is consumed by the pipe and its status is cleared, so it is skip-unresolvable
#     and ships no probe (even with an oracle). Authoring an oracle for the tool you OWN never
#     touches the status the `||` consumes.
#   - Even were `grep` modeled, rule-query-validity invalidates it: opaque `otelcol` reaches it
#     through the pipe ⇒ non-pristine ⇒ valid:false ⇒ its rc is withheld to ⊤ ⇒ the fold is
#     blocked (classify pins effect::opaque_pipe_predecessor_invalidates_downstream_query).
#   - `otelcol.is_converged()` mints no guard: it keys the non-last stage, not an
#     EstablishWritten site (gap-anatomy (c)).
# So at HEAD every site is `run`; the bare book short-circuits curl|tar (converged), leaving
# {otelcol --version, grep -q 0.155.0} — the two-sided head-expected.ran signature. A surprise
# XPASS means the pipe-guard lift landed: DIFF the emitted plan by eye before promoting (23C-fd4).
# RAN_ORDER=lax (pipe stages log concurrently). ru-26 churn-note: stderr not modeled here.
otelcol --version | grep -q "0.155.0" || curl -sL https://example.com/otelcol.tar.gz | tar xz
