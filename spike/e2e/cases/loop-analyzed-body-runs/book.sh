# loop-analyzed-body-runs (task-L1 re-pin of the former loop-degrades-safely; brk-1):
# a `for` over a literal list now PARSES (no syntax-unsupported diagnostic) and lowers to
# a real cyclic CFG with a back-edge — NOT an absorbing ⊤ node. The body install RUNS for
# two reasons that stack (the SAFE degrade, kFAIL-perform):
#   1. `"$x"` resolves to ⊤ (the for-var JOINs two distinct words a,b ⇒ ⊤) ⇒ Opaque; and
#   2. even were it a single resolvable word, the in-loop render floor (`209` brk-1) bars
#      eliding one iteration of a line-granular render.
# The install BELOW the loop is poisoned to unresolvable: the body's Opaque propagates
# Reach::Top across the back-edge and OUT, so curl runs too. Both body iterations (a, b)
# and curl appear in the run-set; nothing elides. (The elision UNLOCK below a *pure* loop
# is the sibling case loop-post-elision-revives.)
for x in a b; do apt-get install -y "$x"; done
apt-get install -y curl
