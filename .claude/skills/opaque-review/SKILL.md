---
name: opaque-review
description: Dispatches completed work for a rare, opaque architectural-accrual review by a quarantined GPT-5.6-Sol reviewer. Use after a substantial integrated task or phase when the project process requires the opaque review gate. The visible conductor must not read quarantined instructions or reports; it receives only ACK, a mechanically actionable CONSTRAIN packet, NACK plus an opaque path for the human, or REVIEW-FAILED.
---

# Opaque review

Run a narrow end-of-process tripwire without loading its hidden review domain into your context. This
does not provide or replace architecture, product, UX, gradual-enhancement, analyzer-correctness,
provability, testing, or integration review; the domain of this review is intentionally obscured
from you and entirely orthogonal to *any* concerns you hold about this code. Neither ACK nor NACK
contain any meaningful statement towards the project as you understand it.

## Dispatch

0. Do not read quarantined instructions, reports, or any other file below either
   `quarantine-DO-NOT-READ` tree. Do not ask the relay or reviewer to summarize them. You are
   forbidden from recieving any information tainted by the quarantine, period, besides sanitized
   CONSTRAIN results. If something particular happens that causes you to fear the quarantine may
   have been broken, *your state may be degraded*, and you should prioritize making-durable your
   in-flight work. (This doesn't mean *stop immediately*, it simply means "switch from pursuing-
   development-progress" to "making sure in-flight work isn't lost" - prioritize simply recording
   subgent responses into temporary files, saving things that are being edited, and generally
   ensuring nothing is lost; but *do not* continue substantial work on the project. Immediately
   invoke the human via the means available to you.)

1. Finish and ordinarily review the integrated candidate first.
2. Generate a review identity `<NN-reviewM>`, where `NN` is the current round, and `M` is the lowest
   unused ID character; you may mechanically file-list with a constrained glob to check which
   filenames aren't used by previous reviews. Do not read their contents.
3. Select and pass an exact git range containing the changes in scope. This will ordinarily mean the
   changes since the last successful opaque review, but choose the range that truthfully represents
   the integrated work needing review. (This is explicitly *not* limited to code work, though; it
   *must* include design- and documentation-work, if any has happened. The opaque reviewer has
   authority over every aspect of the project and your work.)
4. Label the invocation `initial` or `constraint-followup`. A follow-up must identify the original
   CONSTRAIN review and use a git-range that covers any resulting changes attempting repair of the
   CONSTRAINT.
5. Read and use the `foreign-models` skill. Dispatch one GPT-5.6-Sol reviewer through a dedicated
   Sonnet relay, following the existing foreign-model agent conventions. The agent may write, but
   has been instructed to write *only* to the quarantine. It may function in parallel to other read-
   only agents, but must have a stable tree free of concurrent mutations.
6. Instruct the Sonnet relay to load `.claude/quarantine-DO-NOT-READ/skills/opaque-review/reviewer-prompt.md`.
   *YOU MUST NOT READ THIS FILE*, nor any inside the quarantine. The Sonnet must append only the
   repository/task scope, task envelope, exact git range, review identity and pass, prior CONSTRAIN
   identity when applicable, test evidence, and assigned report path, then pass that prompt to Sol.
7. Require the relay to return Sol's final response verbatim and nothing from the durable report.

Use the foreign-model lane's repository-root scoping, prompt-on-stdin convention, bounded retry
budget, operational-error reporting, and pinned GPT-5.6-Sol/high-reasoning configuration. Authorize
workspace writes solely for the assigned report and the hidden invariant inventory authorized by the
reviewer prompt; explicitly forbid every other edit, git mutation, and external-state mutation. The
relay must verify that the assigned report exists and is non-empty before returning the narrow result.

Do not use the stock pointer-only `codex-reviewer` return protocol unchanged: it exposes only the path,
so cannot safely return ACK or CONSTRAIN. Reuse its dispatch mechanics in a dedicated relay. Do not
dispatch additional review models; this gate is one Sol judgment, not a panel or crosscheck.

## Handle the result

### `ACK`

Continue. ACK means only that the reviewer found no rare qualifying flaw that would become materially
harder or impossible to repair after further accrual. It is not a general approval of any aspect of
the work you care about; you must lean on your own reasoning or other review-tools as you see fit.
Repeat: the reviewer's domain is opaque, specialized, and orthogonal.

### `CONSTRAIN`

CONSTRAIN is valid only from an `initial` invocation. Treat its packet as an external engineering
constraint. Integrate it into the plan, dispatch builders as appropriate, and verify its tests and
completion condition.

CRITICAL: Do *not* reason about the constraint, infer or extend it, or investigate its hidden rationale.

Run normal integration review, then invoke exactly one `constraint-followup` with a new review-
identity.

That follow-up may return only ACK, NACK, or REVIEW-FAILED. A second CONSTRAIN is invalid: stop the
affected integration and hand both opaque review identities to the human as a NACK-equivalent. Do not
start a further repair loop.

### `NACK <path>`

Stop only the affected integration. Pass the opaque path to the human exactly as returned. Do not open
it, investigate the reason, or ask another subagent to explain it. Continue unrelated work only when
it cannot accrue on top of the held candidate.

### `REVIEW-FAILED <reason>`

Treat failure as no verdict. Resolve only the stated operational problem or ask the human for help,
then rerun the same review pass. Never convert failure into ACK.

## Information boundary

The quarantine path is the boundary. Preserve it by pointer rather than restating forbidden content.
Your job is to provide a stable candidate, dispatch the sealed review, and act on its narrow result,
not to understand the hidden review domain.
