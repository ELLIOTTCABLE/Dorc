## Architecture-planning / footgun-uncovering-research-phase

- [ ] design-pass focused on error-reporting (for both language and orchestrator) and provenance (for analysis, planner, and orchestrator.) this tool will be *very* sensitive to record-keeping and reporting-accuracy, given how much it Touches The Human with comparison to something more plan-the-world. things will inevitably fail more often, and in weirder ways, and with less expectation/planning, in a less-controlled environment; which greatly ups the value of (already-important) error-messages and context-communictaion ...
- [ ] deep security-dive on both the 1. language/analysis and 2. orchestrator architectures,
  - specifically flag 'including seccomp in the core'; LLM claims it's secure-by-design, but I very much do not trust that claim without a specific threat-model and analysis
- [ ] skim shell-script corpii to validate design
