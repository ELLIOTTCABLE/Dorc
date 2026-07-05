# strawman24-pipe-guard-if-form (24J beautification-robustness). The SAME connected check-pipe as
# strawman24-pipe-guard-oracle-converged, but in the `if ! A | F; then M; fi` form a beautifier would
# rewrite the `A | F || M` guard into. It must lift IDENTICALLY: the connected probe keys the
# governing (last) stage grep and captures the RAW pipe rc (NOT the `!`-negated rc — the connected
# body is the stages' span, and the fold replays `!` over the captured rc via Pipeline{negated}). On
# a converged host the pipe is rc 0 ⇒ `! 0` = false ⇒ the then-body (curl|tar) is DEAD, otelcol omits
# (subsumed), grep substitutes to `true`. Run-set EMPTY (expected.ran). Proves the pipe-guard rides
# the EXISTING StatusRelaxable-on-if-guards path (24J: "Beautification-proof by construction").
# RAN_ORDER=lax (pipe stages log concurrently). ru-26 churn-note: stderr not modeled here.
if ! otelcol --version | grep -q "0.155.0"; then curl -sL https://example.com/otelcol.tar.gz | tar xz; fi
