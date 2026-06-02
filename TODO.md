## Architecture-planning / footgun-uncovering-research-phase

- [ ] design-pass focused on error-reporting (for both language and orchestrator) and provenance (for analysis, planner, and orchestrator.) this tool will be *very* sensitive to record-keeping and reporting-accuracy, given how much it Touches The Human with comparison to something more plan-the-world. things will inevitably fail more often, and in weirder ways, and with less expectation/planning, in a less-controlled environment; which greatly ups the value of (already-important) error-messages and context-communictaion ...
- [ ] deep security-dive on both the 1. language/analysis and 2. orchestrator architectures,
  - specifically flag 'including seccomp in the core'; LLM claims it's secure-by-design, but I very much do not trust that claim without a specific threat-model and analysis
- [ ] skim shell-script corpii to validate design

---

*(below: agent-dropped during the kill-criteria session, 2026-06-01 — for human review/integration)*

- [ ] **elision-soundness hazard.** A script that *parses cleanly* but whose CFG silently *under-models* a control-flow-altering construct (`set -e`, `set -o pipefail`, `trap`, especially when set conditionally or mid-script) is more dangerous than one that fails to parse — the parse-failure is a visible, gracefully-rejected cliff, whereas the under-modeled-but-accepted script gets reasoned-over with missing CFG edges and yields a *wrong skip*, an unsoundness that never announces itself. Two distinct teeth:
  - these constructs are a *parsing/acceptance* danger, not (only) a *reasoning* danger — so acceptance must gate on **modeling-completeness, not parse-success**: a control-flow-altering construct that isn't *fully* modeled must be **strongly rejected** (or ⊤-poison the whole script), never silently best-effort'd. "it parsed, ship it" is the trap, and it'll slip through easily because `set -e` is utterly ordinary syntax.
  - the calibration harness must specifically hammer `set -e`/`trap`/redirection reachability against a dash/bash differential (phase-1 already calls this the subtlest correctness trap; this just elevates it to a tracked item).
