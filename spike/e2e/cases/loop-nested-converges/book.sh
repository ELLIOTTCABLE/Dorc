# loop-nested-converges (task-L1 item-4c: nested-loop convergence smoke). Two nested
# `for` loops lower to TWO back-edges feeding one another; the monotone worklist over the
# finite-height value/reaching-defs domains must still reach a fixed point (no hang, no
# capped non-convergence). The inner body install's operand `"$p$q"` is ⊤ (multi-word
# for-vars) ⇒ Opaque ⇒ in-loop ⇒ runs at depth 2; the loop expands to four installs
# (ac, ad, bc, bd) in the run-set. `echo all-done` runs after (pure). The load-bearing
# property is TERMINATION on nested cycles; the run-set witnesses the body runs at depth.
for p in a b; do for q in c d; do apt-get install -y "$p$q"; done; done
echo all-done
