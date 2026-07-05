# strawman24-pipe-guard-unvouched-mid (24J NEGATIVE CONTROL — silence-is-wall preserved). The same
# check pipe as the flagship but with an UNVOUCHED middle stage `cat` (no oracle ⇒ Opaque). 24J
# NARROW FIRST: a connected probe ships ONLY when EVERY stage is a vouched read-only Query — an
# unvouched stage ⊤s the whole thing to the wall floor. So the pipe is NOT recognised: grep (and
# otelcol) become ORPHAN stages — stdin-dependent with no independent fact, they ship NO probe
# (silence-is-wall) and RUN. Nothing elides: the apply is byte-identical to the book, and the fallback
# is NOT wrongly elided — on this host the real check does not match, so the `||` fires and curl|tar
# RUN, EXACTLY as the bare book does (gate-6 zero delta). This is the "no worse than what they already
# did" floor when the pipe is out of dialect. HOST: diverged (otelcol prints 0.99.0; cat passes it
# through; grep -q "0.155.0" finds no match ⇒ the `||` fires ⇒ curl|tar run). RAN_ORDER=lax (pipe
# stages log concurrently). ru-26 churn-note: stderr not modeled here.
otelcol --version | cat | grep -q "0.155.0" || curl -sL https://example.com/otelcol.tar.gz | tar xz
