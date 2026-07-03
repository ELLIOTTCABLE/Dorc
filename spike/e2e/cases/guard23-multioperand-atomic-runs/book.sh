# guard23-multioperand-atomic-runs ("the command is the atomic unit", human axiom
# 2026-07-02 — a PASSING floor). A vouched multi-operand install: the oracle's own check
# refuses a second operand (its argparse annotates exactly one), so no entity resolves, no
# probe ships, no path through the predict-body is reached, and no witness forms — the whole
# line RUNS. The axiom this case carries forward: commands are never disassembled — IF a
# whole-line witness ever becomes mintable (a check modeling the full operand list), a
# guard may serve the line ALL-OR-NOTHING (one diverged member => the entire line runs);
# per-member splitting (installing "just the missing half") is hard-deferred, possibly
# forever. No pin here or anywhere in guard23-* asserts partial-member behaviour.
# PREDICT-BODY HAZARD, pinned by this floor (Research/notes/23A): this oracle's refuse-path
# exits 0 (`if [ "$2" = "" ]` false => rc 0), so a build that shipped the predict as a guard
# WITHOUT proving the invocation reaches a vouched path would mint `check || install` here
# and the rc-0 refuse-path would wrongly SUPPRESS the install — the witness's reached-path
# component is load-bearing exactly here.
apt-get install -y nginx curl
