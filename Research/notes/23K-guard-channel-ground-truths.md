# 23K — MOVED: promoted and rewritten as `Research/plans/240-interface-round-charter.md`

This note's content (the human's guard-channel ground-truths, the rc/verdict naming
discipline, the sibling-functions direction, the round fences) was promoted to plans-tier on
2026-07-02 and restructured as the round-24 charter. Read `plans/240` instead.

For any in-flight brief that pointed here for the naming discipline, its essence: bare "rc"
is banned in design text — qualify as **tool-rc** (raw tool exit status, oracle-internal,
opaque to Dorc) / **predicted-rc** (probe-sourced replacement VALUE, `inv-probe-sourced-values`)
/ **apply-rc** (the exit status minted once, controlledly, from a guard-verdict to drive the
artifact's sh connectives). Verdicts (**plan-verdict**: engine-side ternary; **guard-verdict**:
host-side authored boolean) are never rc's; exactly two blessed crossings exist between the
worlds (tool-rc → verdict, authored in one place per oracle; verdict → apply-rc, the mint).
Full text and rationale: `plans/240`.
