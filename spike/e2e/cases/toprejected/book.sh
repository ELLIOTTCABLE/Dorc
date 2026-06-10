# toprejected (task-L1: the RESIDUAL loop ⊤-reject that survives the trigger-shrink). A
# literal-list `for`/`while` now PARSES (see loop-analyzed-body-runs), but `break`/`continue`
# is still ⊤-rejected: un-modeled early exit breaks the back-edge fixpoint's reaching-uses
# soundness, so the whole loop collapses to an absorbing ⊤ node (inv-top-reject) with a loud
# `syntax-unsupported` + `cfg-top-node` pair. The ⊤ poisons the curl install below it
# (⊤-containment + havoc), so curl runs VERBATIM — the SAFE degrade (kFAIL-perform): an
# unmodeled construct never silently licenses elision.
for x in a b; do break; done
apt-get install -y curl
