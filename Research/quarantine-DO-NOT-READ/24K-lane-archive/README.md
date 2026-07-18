# 24K lane archive — raw material from the cross-model language-design crosscheck

QUARANTINED. This directory holds the verbatim, un-adjudicated inputs and outputs of the 24K
six-lane review (2026-07-05, corpus pinned at `259b27d`). It exists for retrospective
attribution — when a later running issue matches a deferred/discarded finding, come here to see
what the lane actually said. It is NOT part of the readable corpus:

- The adjudicated verdict — the only citable artifact — is
  `Research/notes/24Kc-language-crosscheck-adjudication.md`.
- Do not read these files during future crosscheck skill-ups: they contain pre-registered
  lessons and stance-engineered framings that would contaminate a fresh reviewer's gate.
- Foreign-lane files are raw foreign-model output (purity protocol: provenance-labeled raw
  material, never a paste-through into an assessment). Archived to durables at the human's
  explicit direction after the conducting session was compacted mid-flight.

## Manifest

Conductor material (Fable, adjudicating session):
- `24K-adjudication-ledger.md` — working ledger: six-lane distillations, convergence tallies,
  the nine tooling-friction items. Superseded by 24Kc on any conflict.

As-dispatched packets (all derived from the quarantined 24K prompt-pair,
`../25xxx-language-design-crosscheck-prompts.md`):
- `24K-packet-codex-{neutral,adversarial}.md`
- `24K-packet-deepseek-{neutral,adversarial}.md` (the v2 as-run revisions)
- `24K-packet-deepseek-neutral-continuation.md` (the gate-stall recovery dispatch)

Raw foreign-lane reports:
- `24K-lane-codex-{neutral,adversarial}-report.md` — Codex/GPT-5.5 (OpenAI lineage)
- `24K-lane-deepseek-neutral-gate.md` — DeepSeek V4-Pro first turn (lessons only; turn ended at
  the pre-registration gate)
- `24K-lane-deepseek-neutral-report.md` — DeepSeek neutral continuation (the findings)
- `24K-lane-deepseek-adversarial-report.md` — DeepSeek adversarial

Anthropic Fable lanes (verbatim extracts; the byte-authoritative copies are the branch commits,
which also prove gate-before-corpus ordering):
- `24Ka-langreview-01..06` — NEUTRAL lane, from `19df800` on branch `ai/24Ka-langreview`
- `24Kb-01..04` — ADVERSARIAL lane, from `fd5fa82` on branch `worktree-agent-abd7ff8be88067e1b`
- Both sets are pending cherry-pick to the main line as proper corpus notes; these copies are
  the loss-insurance, not the canonical home.

Deliberately dropped from the archive: the CLI JSON envelopes around the DeepSeek reports
(session metadata only; the `.md` files are the extracted content), a superseded v1 DeepSeek
neutral packet, an intermediate conductor digest folded into the ledger, and the two
already-delivered-and-acked conductor memos from the earlier drift-audit arc.
