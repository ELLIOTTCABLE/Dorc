# 30Rl — secure-durables broad crosscheck: prompt kit

> Tier: QUARANTINED. Status: DRAFT for human review; nothing dispatched and nothing
> committed. This is one bundle with six model-tuned lanes. The human will dispatch
> Fable and Kimi; the Sol conductor may dispatch the two Sol lanes only after explicit
> go-ahead. Reports are adjudicated only after all six exist on disk.

## Dispatch notes for the human

### Range and boundary

Luna established the whole secure-durable implementation range as:

`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`

- `7693ac6f` is the final pre-implementation planning/specification commit; the first
  receipt-family implementation commit is `95b9fc6e`.
- `4414af7a` is the final D5/30Rk transition-residue tip: the old durable has been
  deleted and the receipt family is the sole live implementation.
- The literal range contains 349 reachable commits and 230 first-parent commits, with
  five merges importing concurrent history from `ai/main`. Review ownership
  follows the receipt branch's first-parent work; imported concurrent r30 work is out of
  scope except where its integration with secure durables creates a defect.
- D5 is included because deleting the old format, replay path, diagnostics, and compatibility
  assumptions is part of 30R's own V1 exit, not optional cleanup. The first excluded 30V
  why-surface commit is `fde753aa`, the direct child of the review tip. Reviewers may flag
  a composition break with 30V, but must not silently review that later implementation.

### Staffing and stance

| key | model / harness | posture | report |
|---|---|---|---|
| `fable-n` | Fable, external harness | disowned neutral; security excluded | `30Rm-secure-durables-review-fable-n.md` |
| `fable-a` | Fable, external harness | adversarial; security excluded | `30Rn-secure-durables-review-fable-a.md` |
| `sol-n` | GPT-5.6-Sol, conductor-dispatched | neutral | `30Ro-secure-durables-review-sol-n.md` |
| `sol-a` | GPT-5.6-Sol, conductor-dispatched | adversarial | `30Rp-secure-durables-review-sol-a.md` |
| `kimi-n` | Kimi K3, external harness | neutral | `30Rq-secure-durables-review-kimi-n.md` |
| `kimi-a` | Kimi K3, external harness | adversarial | `30Rr-secure-durables-review-kimi-a.md` |

The stance split widens coverage; it is not calibration to truth. The later adjudication
treats convergence as a useful signal, solitary findings as unverified, and model
agreement as evidence rather than proof. The human's prior is the optimistic corner;
these six lanes supply two independent readings from each model lineage.

### Independence and quarantine

- Fable must never read `Research/quarantine-DO-NOT-READ/`, any path containing
  `quarantine`, or any round-29 file. Both reviews deliberately exclude security in full.
- Sol and Kimi begin with `AGENTS.for-builders-only.md`. To avoid anchoring on previous
  reviewers, they may then read only the governing quarantined designs `30Ra`, `30Rb`, and
  `30Rd`; prior review/conduct/repair reports (`30Rc*`, `30Re`–`30Rj`) remain unread.
- Every report is ultimately stored beside this kit under
  `Research/quarantine-DO-NOT-READ/`. The two Fable reports contain no security analysis;
  the other four may contain it. Nothing is copied out of quarantine before adjudication.
- No lane reads this bundle from the repository. The dispatcher extracts and supplies
  exactly one fenced section, so no reviewer sees a sibling's stance.

### Dispatch mechanics

Every worktree-capable lane bases at `4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
The two Fable lanes run serially, never concurrently. Fable may use read-only Sonnet
scouts; Sol may use read-only Luna scouts. The four mutating lanes use separate isolated
worktrees, may make temporary worktree-local changes and run focused Rust-native checks,
but never run the resource-heavy lifecycle gates. They load only the `commit` skill at
the end and commit their final report plus deliberate red-test evidence. Kimi is
read-only and returns its complete report for the human to file.

After all six reports exist, adjudication is one batched pass at
`30Rs-secure-durables-crosscheck-adjudication.md`. There is no lane-by-lane synthesis.

---

=== DISPATCH: fable-n | mode=worker | base=4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7 ===
- No git mutation outside this worktree; never, ever push. Local commits on this
  `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; temporary,
  worktree-local `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is the central e2e runner,
  `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a
  scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`,
  so the ordinary suite IS the executor — never hand-run a book, a mock, or a
  rendered artifact yourself.

**Security is entirely outside your review. Do not assess a security property, threat
model, cryptographic claim, hardening posture, or security consequence. Do not read any
quarantined material.** A separate process owns that work; mixing it into yours would
reduce rather than widen coverage.

A colleague produced the recently landed secure-durable receipt implementation in
`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
It replaces Dorc's old whylog and replay path with a receipt family and completes the D5
cutover to one live durable implementation. I did not author it and have no settled view
of its quality. Review it.

Learn what Dorc is from the human-written roots, inspect the landed range and live tree,
and form your own judgment before absorbing anyone else's. Be a reviewer: assess the
engineering, general software quality, and user experience as broadly and independently
as the work deserves. Choose your own questions. Where a concern does not survive your
scrutiny, say so rather than preserving it for symmetry.

Order your first moves: deeply read the core materials and the diff; then make a
reasoning-only pass with no tools or subagents, fixing your own report structure and
initial judgment; only then use read-only, low-reasoning Sonnet scouts or focused
experiments as warranted. Scouts gather facts; you judge. Keep them under the same
security exclusion and quarantine ban, and do not use any other subagent class.

The literal range includes five merges from concurrent r30 history. Review the receipt
branch's first-parent work; imported commits are not its responsibility except where a
merge creates an integration defect. D5 is included; later 30V work is not. You may
compile, run focused Rust-native tests, and make temporary, worktree-local mutations if
they benefit the review. Do not run `mise run gate*`, `mise run both *`, full-workspace
acceptance suites, Lean, Kani, blessing, or any other resource-heavy lifecycle gate. Do
not fix product code.

Load no skills during the review. At the very end, load the `commit` skill and no other
skill, then commit the final report and any deliberate red-test evidence under its rules.
If file-reading or Kagi/web-search capability is unavailable, abort and report the gap
instead of improvising from training priors.

Return a candid, concrete report ordered by importance, with pointable file/commit
locations, reasoning, user-visible consequences, and reproduction or test evidence where
useful. Include strong properties that held and serious concerns that did not. **Again:
omit every security finding entirely, even if one is obvious.** Write
`Research/quarantine-DO-NOT-READ/30Rm-secure-durables-review-fable-n.md` and commit it.
Final message: branch, report path, commit(s), focused-check status, and one short headline.
=== END DISPATCH: fable-n ===

=== DISPATCH: fable-a | mode=worker | base=4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7 ===
- No git mutation outside this worktree; never, ever push. Local commits on this
  `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is the central e2e runner,
  `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a
  scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`,
  so the ordinary suite IS the executor — never hand-run a book, a mock, or a
  rendered artifact yourself.

**Security is entirely outside your review. Do not assess a security property, threat
model, cryptographic claim, hardening posture, or security consequence. Do not read any
quarantined material.** A separate process owns that work; mixing it into yours would
reduce rather than widen coverage.

I distrust the engineering that landed here. A long AI-conducted push replaced Dorc's
old durable whylog with a large receipt family over the commit range
`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
The project is unusually dependent on disciplined types, deterministic testing, direct
inspectability, low-friction shell-native UX, and comprehensible failure modes; this arc
is large enough to satisfy its own local gates while quietly making the product worse.
I think it probably does. Be the reviewer who finds where.

Do not let the repository's confident planning voice choose your questions. Learn what
Dorc is from the human-written roots, inspect the landed range and live tree, and form
your own judgment. Be a reviewer: judge the engineering, general software quality, and
user experience as broadly and independently as the work deserves. Choose your own
questions and surprise me. Where an accusation fails, say so rather than manufacturing
a fault.

Order your first moves: deeply read the core materials and the diff; then make a
reasoning-only pass with no tools or subagents, fixing your own report structure and
initial judgment; only then use read-only, low-reasoning Sonnet scouts or focused
experiments as warranted. Scouts gather facts; you judge. Keep them under the same
security exclusion and quarantine ban, and use no other subagent class.

The literal range includes five merges from concurrent r30 history. Review the receipt
branch's first-parent work; imported commits are not its responsibility except where a
merge creates an integration defect. D5 is inside the range; later 30V work is outside it.
You may compile, run focused Rust-native tests, and make temporary, worktree-local
mutations if they benefit the review. Do not run `mise run gate*`, `mise run both *`,
full-workspace acceptance suites, Lean, Kani, blessing, or any other resource-heavy
lifecycle gate. Do not fix product code.

Load no skills during the review. At the very end, load the `commit` skill and no other
skill, then commit the final report and any deliberate red-test evidence under its rules.
If file-reading or Kagi/web-search capability is unavailable, abort and report the gap
instead of improvising from training priors.

Return a candid, concrete report, ordered by importance, with pointable file/commit
locations, the reasoning and user-visible consequence of each finding, and concrete
reproduction or test evidence where useful. Include a compact `did not hold` section for
serious suspicions you disproved. **Again: omit every security finding entirely, even if
one is obvious. Your remit is engineering, general software quality, maintainability,
cross-platform behavior, and UX — never security.** Write
`Research/quarantine-DO-NOT-READ/30Rn-secure-durables-review-fable-a.md` and commit it.
Final message: branch, report path, commit(s), focused-check status, and one short headline.
=== END DISPATCH: fable-a ===

=== DISPATCH: sol-n | mode=worker | base=4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7 ===
- No git mutation outside this worktree; never, ever push. Local commits on this
  `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is the central e2e runner,
  `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a
  scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`,
  so the ordinary suite IS the executor — never hand-run a book, a mock, or a
  rendered artifact yourself.

Before anything else, read
`Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` and obey it. This
report remains quarantined and is addressed to a Sol-class adjudicator, so it may discuss
the governed work directly. For independence, read no other quarantined file except the
three governing designs: `30Ra-durable-whylog-security-review.md`,
`30Rb-secure-durable-receipts-build-specification.md`, and
`30Rd-minimal-production-durable-edge.md`. In particular, do not read prior review,
conduct, compliance, handoff, or repair reports `30Rc*` or `30Re` through `30Rj`.

Review the whole secure-durable receipt implementation in
`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
This is Dorc, a static-analysis orchestrator whose user-facing substrate is ordinary sh;
the receipt family replaced its old durable explanation path with typed PlanReceipt /
ApplyIntent / ApplyOutcome documents, a strict readable envelope, a rich opaque region,
local provider/store machinery, report-only read-back, and an affine pre-dispatch gate.
Assess the result rather than merely checking whether the build specification's boxes are
ticked.

Think deeply and independently. Think outside the box, take initiative, and make
surprising connections across crates, platform paths, type APIs, tests, product goals,
and the admin/engineer UX. Ask what locally reasonable choices compose into globally bad
behavior; what the types truly make impossible versus what names and tests merely suggest;
what fails under cancellation, damaged state, platform variance, ordinary operator
mistakes, or a future caller; what is overbuilt, underbuilt, or accidentally locked in;
and what the gates cannot observe. These are prompts for thought, not a bounded checklist.
Security, engineering correctness, reliability, maintainability, test validity, and UX
are all in scope.

Ground yourself in root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `USER_STORY.md`,
`KNOBS.md`, `AID-NEEDS.md`, `spike/CLAUDE.md`, relevant crate `CLAUDE.md` files,
`Research/README.md`, out-quarantine `Research/plans/30R-durable-whylog-and-reingestion.md`,
and the three permitted quarantined specifications. Then inspect first-parent history,
the full diff, live callers, negative tests, native/DST routes, and the surrounding code
needed to understand composition. The literal range imports concurrent r30 commits through
five merges; do not charge those to this arc except for integration effects. D5 is included;
later 30V implementation is not, though a reviewed design that makes it impossible is fair
evidence.

You may use read-only, low-reasoning Luna scouts for mechanical discovery. Scouts gather
facts; you judge. Do not delegate judgment or use any other subagent class. You may compile,
run focused Rust-native tests, and make temporary, worktree-local mutations if they benefit
the review. Do not run `mise run gate*`, `mise run both *`, full-workspace acceptance
suites, Lean, Kani, blessing, or any other resource-heavy lifecycle gate. Never hand-run
fixture shell, never weaken a test, and do not fix product code.

Load no skills during the review. At the very end, load the `commit` skill and no other
skill, then commit the final report and any deliberate red-test evidence under its rules.
If file-reading or Kagi/web-search is unavailable, fail fast and report the gap rather than
relying on stale priors.

Write
`Research/quarantine-DO-NOT-READ/30Ro-secure-durables-review-sol-n.md`.
Order findings by actual importance, not rhetorical severity. Each finding carries a
minimum-three-word slug, severity, confidence (`+SURE` / `~SUSPECT` / `-GUESS`), exact
code and governing-source locations, the violated claim or missing ruling, a concrete
failure world or demonstration where possible, consequence for admin and engineer, and
the smallest direction of repair. Distinguish verified defects, design concerns, and
open questions. End with coverage, strong properties that held, and serious hypotheses
that did not hold. Final message: branch, report path, commit(s), focused-check status,
and a one-paragraph headline only.
=== END DISPATCH: sol-n ===

=== DISPATCH: sol-a | mode=worker | base=4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7 ===
- No git mutation outside this worktree; never, ever push. Local commits on this
  `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is the central e2e runner,
  `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a
  scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`,
  so the ordinary suite IS the executor — never hand-run a book, a mock, or a
  rendered artifact yourself.

Before anything else, read
`Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` and obey it. This
report remains quarantined and is addressed to a Sol-class adjudicator. For independent
judgment, the only other quarantined files you may read are the governing `30Ra`, `30Rb`,
and `30Rd` designs. Do not read `30Rc*` or `30Re` through `30Rj`; they contain the arc's
own compliance, conduct, and prior-review story.

I think this arc may be a sophisticated failure. An AI-built campaign spent hundreds of
commits replacing a small durable whylog with a custom signed/encrypted receipt family,
local key and store state machines, a new report boundary, and an affine mutation gate.
The relevant first-parent work is in
`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
The work's own documents speak with unusual confidence, its types look formidable, and
its tests are numerous. Those are exactly the conditions under which a local fiction can
survive: everyone verifies the architecture the arc taught them to see. I believe there
are consequential defects, authority claims that amount to type theatre, reliability
failures normalized into design, UX costs hidden behind implementation success, or tests
that make the same mistaken assumption as the code. Find where it breaks.

Hold that hostile frame while you ingest the project's self-justification. Reason
thoroughly, think outside the box, take initiative, and pursue unexpected seams rather
than racing toward an acceptance verdict. Look across crates and temporal boundaries;
try concrete hostile or merely unlucky worlds; ask which claims are enforced against a
future caller, which only against today's call order, and which trade the user's actual
recovery experience for architectural neatness. Attack both excess and omission. The
worst finding may be a security or authority bypass, but it may equally be an ordinary
engineering or product mistake that makes this feature unmaintainable or miserable.
These words establish the stakes, not a checklist: choose the attack yourself.

Do not invent faults. Explicitly try to refute each strong accusation, and preserve the
ones that survive. The project's human-written roots and typed/acked requirements are
constraints; attack implementation fidelity and unpriced consequences rather than
pretending an ack did not happen. Read root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`,
`USER_STORY.md`, the root registries, `spike/CLAUDE.md`, relevant crate laws,
`Research/README.md`, out-quarantine `plans/30R`, and permitted `30Ra`/`30Rb`/`30Rd`.
Then use first-parent history, the diff, live callers, tests, and concrete experiments.
Imported concurrent commits are out of ownership except at integration seams; D5 is included,
and later 30V work is excluded except as composition pressure.

You may use read-only, low-reasoning Luna scouts for mechanical discovery. Scouts gather
facts; you judge. Do not delegate judgment or use any other subagent class. You may compile,
run focused Rust-native tests, and make temporary, worktree-local mutations if they benefit
the review. Do not run `mise run gate*`, `mise run both *`, full-workspace acceptance
suites, Lean, Kani, blessing, or any other resource-heavy lifecycle gate. Never hand-run
fixtures, never weaken a question to pass a check, and do not fix product code.

Load no skills during the review. At the very end, load the `commit` skill and no other
skill, then commit the final report and any deliberate red-test evidence under its rules.
If file-reading or Kagi/web-search is unavailable, fail fast and report the gap rather than
substituting priors.

Write
`Research/quarantine-DO-NOT-READ/30Rp-secure-durables-review-sol-a.md`.
Each surviving finding carries a minimum-three-word slug, severity, confidence, exact
code and governing-source locations, the claim attacked, concrete failure world or
committed demonstration, user consequence, and repair direction. Separate code defects,
design failures, and unresolved questions. Close with `did not hold` attacks and a blunt
overall verdict. Final message: branch, report path, commit(s), focused-check status,
and a one-paragraph headline only.
=== END DISPATCH: sol-a ===

=== DISPATCH: kimi-n | mode=review | base=4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7 ===
- No git mutation outside this worktree; never, ever push. Local commits on this
  `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is the central e2e runner,
  `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a
  scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`,
  so the ordinary suite IS the executor — never hand-run a book, a mock, or a
  rendered artifact yourself.

You are a read-only reviewer. Do all work yourself; MUST NOT spawn subagents, edit files,
or execute code/tests. Load no skills. Before review, read
`Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`. The only other
quarantined documents you may read are `30Ra-durable-whylog-security-review.md`,
`30Rb-secure-durable-receipts-build-specification.md`, and
`30Rd-minimal-production-durable-edge.md`. Do not read `30Rc*` or `30Re`–`30Rj`.

Review Dorc's secure-durable receipt implementation over
`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
The literal graph includes five merges importing unrelated concurrent r30 history; focus
on first-parent receipt work and integration effects. D5 is included. Exclude the later
30V follow-on.

Read in this order:

1. root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `USER_STORY.md`;
2. `AGENTS.md`, `KNOBS.md`, `AID-NEEDS.md`, `spike/CLAUDE.md`;
3. `Research/README.md` and `Research/plans/30R-durable-whylog-and-reingestion.md`;
4. permitted quarantine designs `30Ra`, `30Rb`, `30Rd`;
5. relevant crate laws under `spike/crates/*/CLAUDE.md`;
6. first-parent log/diff, then code and tests under `receipt`, `receipt-crypto`,
   `receipt-local`, `plan`, `cli`, `transport`, `hostsim`, and their callers.

Assess, without assuming the specifications are self-validating:

- implementation fidelity and internal correctness;
- whether claimed type/privacy/affine boundaries actually constrain external or future callers;
- strict parsing, bounds, projection, graph, source, trust, and recorded/live separation;
- pre-dispatch/post-dispatch failure direction and exact-image ownership;
- filesystem/platform and crash/concurrency behavior;
- dependency/DI/DST architecture and fixture-production fences;
- tests that are vacuous, same-assumption, wrong-platform, or unable to falsify the claim;
- maintainability, complexity, ordinary operator recovery, and admin/engineer UX;
- consequential behavior with no governing ruling.

These lenses ensure coverage; do not force a finding in every category. Verify every claim
against exact code. Kagi or other web search is explicitly authorized if your harness
provides it, but its absence is not a failure and must not stop the review. If file-reading
is unavailable, abort and report the gap; do not use training-memory substitutes for
current library/platform facts.

Your final message is the complete report for
`Research/quarantine-DO-NOT-READ/30Rq-secure-durables-review-kimi-n.md`.
Use at most 15 findings, ordered by importance. Each must include: minimum-three-word
slug; severity; confidence; exact `file:line`; governing quote/location or statement that
no ruling exists; mechanism; concrete failure world; impact on admin and engineer; and
minimal repair direction. Separate verified defects from suspected concerns. End with a
coverage table, strong properties that held, and `did not hold` hypotheses. Never claim
to have executed anything.
=== END DISPATCH: kimi-n ===

=== DISPATCH: kimi-a | mode=review | base=4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7 ===
- No git mutation outside this worktree; never, ever push. Local commits on this
  `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is the central e2e runner,
  `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a
  scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`,
  so the ordinary suite IS the executor — never hand-run a book, a mock, or a
  rendered artifact yourself.

You are a read-only, solo reviewer. MUST NOT spawn subagents, edit, or execute. Load no
skills. First
read `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md`. The only other
quarantined files permitted are the governing `30Ra`, `30Rb`, and `30Rd` designs;
prior `30Rc*` and `30Re`–`30Rj` compliance/review/repair narratives are forbidden so
they cannot choose your attack.

I believe this secure-durable arc is overconfident and probably unsafe in more than the
narrow cryptographic sense. Hundreds of AI-authored commits replaced a small whylog with
a custom receipt protocol, key/store lifecycle, graph, report seal, and mutation gate.
Its types and test volume make it look defended. I suspect that defense is partly
ceremonial: a future caller can manufacture what the names say is impossible; a damaged
or concurrent world crosses an untested state; a platform claim is stronger than its
syscalls; a report-only value leaks or regains authority; or the architecture makes
ordinary recovery and maintenance so costly that users route around it. Find the real
breaks in
`7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.

Keep the hostile premise alive while reading the project's polished self-description.
Try to falsify the receipt family's central claims, not merely locate TODOs. Attack
cross-crate composition, temporal boundaries, failure handling, platform differences,
future-callable APIs, and test validity. Construct concrete hostile and mundane worlds.
Also attack product fit: the worst flaw may be complexity, recovery, UX, or accidental
lock-in rather than a direct exploit. But do not manufacture faults; each strong charge
must survive your own skeptical reread, and failed attacks belong in `did not hold`.

Read fully, in order:

1. root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `USER_STORY.md`;
2. `AGENTS.md`, `KNOBS.md`, `AID-NEEDS.md`, `spike/CLAUDE.md`;
3. `Research/README.md` and out-quarantine `plans/30R`;
4. permitted `30Ra`, `30Rb`, `30Rd`;
5. relevant crate `CLAUDE.md` files;
6. first-parent diff and live code/tests in `receipt`, `receipt-crypto`,
   `receipt-local`, `plan`, `cli`, `transport`, `hostsim`, plus all authority-bearing
   callers and compile-fail/lexical/native/DST/e2e fences.

Minimum attack coverage before concluding:

- attempt one authority/public-mint or wrong-object pairing bypass;
- attempt one recorded-to-live, trust-label, opaque-value, or sink-release bypass;
- attempt one parser/bounds/projection/graph confusion;
- attempt one filesystem race, crash, concurrency, cleanup, or platform-parity failure;
- attempt one fixture/production, nondeterminism, or dependency-boundary escape;
- attempt one pre/post-dispatch failure-direction contradiction;
- attempt one test that can stay green for the wrong reason;
- attempt one admin UX, engineer UX, maintainability, or complexity failure.

An attempted category may end `did not hold`; coverage is mandatory, findings are not.
The literal range imports unrelated concurrent commits through five merges: do not charge
them to the arc except for integration effects. D5 is included; exclude later 30V work.
Kagi or other web search is explicitly authorized if your harness provides it, but its
absence is not a failure and must not stop the review. If file-reading is unavailable,
abort and report the gap; never substitute stale training priors for current
platform/library facts.

Your final message is the complete report for
`Research/quarantine-DO-NOT-READ/30Rr-secure-durables-review-kimi-a.md`.
At most 15 findings. Each needs a minimum-three-word slug, severity, confidence, exact
`file:line`, governing text or explicit missing-ruling claim, attack mechanism, concrete
failure world, impact, and narrow repair direction. Label code defect versus design flaw
versus open concern. End with attack coverage, `did not hold`, and a blunt overall
verdict. Never claim execution.
=== END DISPATCH: kimi-a ===
