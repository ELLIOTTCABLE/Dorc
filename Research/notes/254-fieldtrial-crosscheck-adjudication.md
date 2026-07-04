# 254 — round-25 protocol adversarial-crosscheck: adjudication + retrospective record

AI-authored (Fable conductor), 2026-07-04. Adjudication of the two crosscheck passes over the
round-25 field-trial protocol (`plans/252`). **Dual purpose:** (1) the conductor's take + what was
applied; (2) a **retrospective-attribution record** — the human's stated long-term value: *"high-signal
when a later investigation into a running issue comes up with 'adversarial said this ten turns back and
it was ignored'."* So EVERY finding is recorded with its disposition, including the deferred and
discarded. **If a later running issue matches a deferred/discarded finding here, that is the signal to
revisit it.**

## Setup
Two Fable passes, clean-context, isolated worktrees, over `252`/`250`/`251` at `2e1fdc0`: **neutral**
(disowned/supportive) + **adversarial** (disowned-artifact + owned-doubt). Prompts: `quarantine/25xxx`
(committed `1057e51`). Raw pass notes lived in the reviewers' worktrees `agent-a36da…` / `agent-a72b…`
(`254a`/`254b`, ephemeral); the load-bearing findings are captured below. Both passes ran the
kill-your-own-findings discipline (each verified against docs **and spike code** and withdrew
suspicions — e.g. the adversarial pass killed its own "the CLI is aspirational" suspicion by reading
`crates/cli`).

## Credal stance (`[[crosscheck-adjudication-skepticism]]` + the human's anti-time-sink instruction)
Deflationary by default; credit only **convergent + doc-verified + goal-material + cheap-to-fix**. The
corpus has historically over-credited adversarial review (>½ the human's total time). Here, unusually,
both passes converged *independently* on one through-line, doc-verifiable, and the whole fix-set is one
paper afternoon — so it's credited, and the discards are named so the filter stays auditable.

## The human's ruling (2026-07-04)
- Overarching lean: *"this sounds like a lot of overengineering"* — **explicitly self-flagged as a
  biased source** (his own optimism/defensiveness); weighted, not decisive.
- Apply-as-conductor-sees-fit; **at most ONE non-trivial fix**, trivial ones freely; "not that big a
  round." Stamp this record.

## The through-line (both passes, ~same sentence)
**Instrument-rich, decision-poor.** The protocol built the measuring apparatus but exempted *itself*
from the pre-commitment + calibration its own B4 embodies. Of eight "pre-registered questions," only
B4 carries a real action-fork — and it rides the weakest numbers in the plan.

## Findings ledger — convergence · verification · disposition

| # | finding | passes | verified | disposition |
|---|---|---|---|---|
| F1 | §4 decisions are vibe-words (no thresholds → post-hoc-gradeable = the woo-cool adversary; violates own `signal-reducibility`) | both, +SURE | §4 is qualitative-only | **APPLIED** `252 §7` (pre-register numbers) |
| F2 | differential `∅` cannot fire on a real box (fs-diff never empty; world-drift; probe/apply asymmetry) → verdict decided by an unspecified, team-authored, incentive-biased noise-filter = self-greening relocated | both, +SURE | 252 has no noise/volatility treatment | **APPLIED §7** (world-drift verdict category; A/A dorc-independent noise-calibration approach). BUILD deferred Phase-A |
| F3 | A1 bite-rate is circular (team plants bugs *and* builds host-states); seeded-in-run inverts "near-zero ⇒ safe" (means blind, not safe) | both, +SURE | A1 as written seeds P5, reads near-zero as reassurance | **APPLIED §7** (planted = 100%-sensitivity GATE; organic = existence-only; different-lineage plants; **seeded set disjoint+labeled from Phase-B stdlib**) |
| F4 | B4 (the one real fork) rides team-LLM-authored coverage on **unbuilt Stage-4/5** → low number condemns an under-built tier, falsely confirming the 233-bias B4 exists to check | both, +SURE | Stage-4/5 unbuilt at `2e1fdc0` | **APPLIED §7** (gate the yeet-decision on Stage-4/5 landing; else evidence-gathering) |
| F5 | missing denominator — nobody computed what the game-server book *can* yield; the design *predicts* low numbers (steamcmd un-oracleable, `curl\|tar` ⊤, heredoc units refuse-homed) | adversarial-led, +SURE | 252 has no ceiling | **THE ONE NON-TRIVIAL** — paper dry-run (Opus, pre-Phase-A); doubles as F1's prediction source |
| F6 | drift-arm missing — `250`/USER_STORY are steady-vs-drifted; 252 tests only cold + converged-re-run; B4's tier only pays on drifted days | neutral, +SURE | 252 has no drift arm | **PARTIAL** — B4-gate + the dry-run's scenario-mix cover the decision-risk; full drift-arm BUILD deferred Phase-A. *Recorded.* |
| F7 | probe-inertness arm missing — DESIGN's "plan doesn't mutate" + `087` have no test-arm; P4 diffs apply-deltas only | neutral, +SURE | no probe-only arm in P4 | **DEFERRED Phase-A** — the sole test of the sacred weld against real tools + LLM stdlib. *Recorded — revisit if a probe ever mutates.* |
| F8 | B3 felt-product rides a signal the docs disclaim (`n1-honesty`/`find-perception-gap`/B3-itself), baseline never runs, owner-subject → positives ~void | both, +SURE | B3 baseline is imagined | **APPLIED §7** (pre-commit the asymmetry: negative counts, positive confirms nothing) |
| F9 | confound-isolation regressed 251→252 — the layered conditions vanished; `dorc why` *alone* can't separate executor-bug / careful-book / engine-⊤ / unfamiliarity | both, +SURE | 251 conditions absent in 252 | **APPLIED §7** (reinstate the two conditions; = the owed `+something-else`) |
| F10 | gap-log (priority-1) rides the weakest instrument; **no Phase-A gap channel** → agents fix-and-forget upstream at scale | both, +SURE | no Phase-A gap channel in contracts | **APPLIED §7** (Track-A gap-ledger; zsh friction-button; same-evening cued-recall debrief; recorder redundancy) |
| F11 | schedule honesty — "parallel with r24" is plumbing-only (science rides r24's tail); execution-before-review = sunk-instrument risk | both, partial | LIVING_STATUS sequence | **APPLIED §7** (freeze §2 after review; log wallclock anyway). Parallel-caveat *recorded.* |
| D1 | "first-contact is mostly already-spent, stop protecting it" | adversarial, ~SUSPECT | — | **DISCARDED** — its convergent core (owner-subject positives-void) kept under F8; the reframe changes no cheap action |
| D2 | Tier-3 polish (per-oracle attention-lines receipt; affordance inventory; …) | neutral | — | **DISCARDED** — someday, not day-deciding |

## Net
"Feels productive" → "actually decides" is ~one paper afternoon (F5's dry-run) + the F1–F11
pre-registration one-liners now in `252 §7`. The non-trivial *builds* (A/A harness, `086`-sensitivity,
probe-arm, full drift-arm) are Phase-A, deferred by the human's at-most-one lean. **Retrospect hook:
if the day fails to decide something, or a Phase-A green later proves hollow, F2 / F3 / F7 are the
first ledger rows to re-read.**
