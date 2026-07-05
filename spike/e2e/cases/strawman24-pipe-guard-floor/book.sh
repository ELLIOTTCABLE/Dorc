# strawman24-pipe-guard-floor (round-25 field-trial book; the pipe-guard shape at the
# SAFETY FLOOR — no oracle). Three lines of the idiomatic version-check-then-install:
#   <tool> --version | grep -q "<ver>" || curl -sL <url> | tar xz
# A book-side pipeline (`<tool> --version | grep -q`) as the CHECK side of a `||` guard,
# with a second pipeline (`curl … | tar xz`) as the fallback mutator. With NO oracle, every
# site is unresolvable ⇒ runs; nothing elides/guards. The mutators (curl|tar) run and cast
# walls, but there is nothing they could wrongly elide here — the floor is provably SAFE:
# the apply is byte-identical to the book (gate-6 dual-rail: zero elision delta). This is
# the "no worse than what they already did" promise (USER_STORY Stage 0). The owed VALUE
# (a described tool ⇒ the converged check-line lifts) is the sibling XFAIL pin
# strawman24-pipe-guard-oracle-converged; why it cannot lift is the classify pins
# effect::{opaque_pipe_predecessor_invalidates_downstream_query,…}.
# HOST: diverged — each tool prints no version ⇒ grep -q finds no match (rc 1) ⇒ the `||`
# fires ⇒ curl|tar run. Every one of the 12 sites executes (RAN_ORDER=lax: pipeline stages
# log concurrently). ru-26 churn-note: stderr is not modeled by this harness (no case asserts it).
otelcol --version | grep -q "0.155.0" || curl -sL https://example.com/otelcol.tar.gz | tar xz
promtail --version | grep -q "2.9.0" || curl -sL https://example.com/promtail.tar.gz | tar xz
node_exporter --version | grep -q "1.7.0" || curl -sL https://example.com/nodeexp.tar.gz | tar xz
