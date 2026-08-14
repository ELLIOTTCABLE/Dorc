# Crosscheck dispatch bundle — verification-tooling research effort

Four read-only review lanes over the same corpus: fable-neutral, fable-adversarial
(Anthropic lineage, dispatched natively), sol-neutral, sol-adversarial (OpenAI
lineage, via the codex-reviewer shim). Built per the adversarial-crosscheck skill:
one neutralised and one adversarial rewrite per lineage; constraints bracketed
verbatim, stance rewritten. Reports land in this directory.

=== DISPATCH: fable-neutral | mode=review ===

SAFETY (from the project's standing law; binds you absolutely):
- No git mutation of any kind; never, ever push. You have NO commit authority —
  the conductor commits. You are READ-ONLY on the repository except for exactly
  ONE file you will create: your report (path below).
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config).
- Real-command strawmen in the repo are frozen evidence; never execute fixture
  material; never hand-run books/mocks/rendered artifacts. Do not run builds or
  test suites at all — this is a reading review.
- NEVER read `Research/notes/quarantine-DO-NOT-READ/` or `Research/corpora/`.
- NAMING LAW (human-typed): the effort under review is deep research, NOT a
  numbered round — do not mint any rNN label for it in your report's name or
  prose (citing existing corpus doc-IDs like `28R:slug` is fine; era reference,
  if unavoidable: "the tail of r28").

THE TASK. A multi-day research-and-design effort examined adopting
higher-correctness tooling for this project's static-analysis kernel:
result-certification of the fixpoint solver, Kani/property-test tiers, Flux
refinement types, a Lean model of the "sparing algebra", Aeneas
Rust-to-Lean translation, and DST extensions. It produced a six-turn research
round with a graded source base, a synthesis-of-record, a post-synthesis
amendments ledger, and a Lean formalization spike. Assess it. Specifically:

a. Internal validity: are the round's conclusions supported by its own cited
   evidence? (The source base is taken as given — its GRADING may be critiqued;
   missing coverage may be noted as a gap, never re-gathered.)
b. Plan coherence: is the final portfolio plus the amendments-ledger plan-state
   coherent — internally, and against the project's standing law (the CLAUDE.md
   invariant registries, the cited typed rulings)? Are the tradeoffs correctly
   priced, especially toolchain/churn/agent-loop costs?
c. Formalization faithfulness: is the Lean spike faithful to the spec AS
   WRITTEN (`Research/notes/277` + `272` §4 + the CLAUDE.md law sections)? Are
   its theorem STATEMENTS the right statements (the proofs are machine-checked;
   statements are not)? Are its self-declared SPEC-GAPS correct, correctly
   severity-ordered, and complete enough — did formalization pressure miss
   underdetermination it should have caught?
d. Kernel fidelity: do the effort's claims about the kernel hold against the
   actual code (`lattice.rs`, `solve.rs`, `claim.rs`)?

The corpus's own hedges and self-flagged concerns are visible in its text; judge
whether that hedging is honest and complete, but do not limit your review to the
already-flagged spots.

READING LIST (all under `C:\Users\ec\Sync\Code\Dorc`; read what you need, deeply):
- Research round: `.claude\research\refinement-types-industrial-cost\turn01-2026-08-13-notes.md`
  through `turn06-2026-08-13-notes.md` (turn06 is the synthesis-of-record) +
  `sources.json` (22 graded sources; the raw archives are NOT present locally —
  treat quoted citations as given; you may spot-check a specific quotation
  against its recorded URL if a finding turns on it, but gather no new sources).
- Amendments ledger (supersedes turn06 where they disagree — judge the COMBINED
  plan-state): `.claude\research\refinement-types-industrial-cost\crosscheck\input-turn06-pending-amendments.md`
- Lean spike (in a linked worktree; read-only):
  `.claude\worktrees\agent-a46d5d32e3e70a008\.claude\research\refinement-types-industrial-cost\spike-lean-sparing\`
  — `README.md`, `REPORT.md`, `SparingAlgebra\{Coordinate,Dialect,Compare,Sparing,Laws}.lean`.
- The spec it formalizes: `Research\notes\277-entity-algebra-design.md` (esp.
  §§1-3, §5, §6), `Research\notes\272-address-derived-topology.md` §4,
  `spike\CLAUDE.md` (the "Invariants — separation, worlds & survival" cluster),
  `spike\crates\core\CLAUDE.md` (the "Law — the coordinate" section).
- Kernel ground truth: `spike\crates\analysis\src\lattice.rs`,
  `spike\crates\analysis\src\solve.rs`, `spike\crates\core\src\claim.rs`,
  `spike\crates\analysis\CLAUDE.md`.
- Only if a finding turns on them: `Research\plans\021-static-analysis-engine.md`
  (§1), `Research\plans\055-analysis-architecture.md` (dec-6),
  `Research\plans\271-block-settle-rulings-ledger.md`,
  `Research\notes\28R-context-kernel-review.md` (+ per-lane siblings), root
  `KNOBS.md` / `ANALYZER-NEEDS.md` / `AID-NEEDS.md`.

PROCESS MANDATE (in this order):
1. Deep read of the core materials first.
2. Then a reasoning-only pass — no tools — in which you lay out your report
   structure and commit your initial judgements.
3. Only then, focused verification reads (and, if genuinely useful, read-only
   Explore subagents for mechanical searching — never for judging; do not let
   any subagent's output frame your review).

EXCLUSIONS (out of scope; do not spend attention here):
- style/idiom/naming taste, comment style, formatting;
- re-running literature research;
- re-building or re-running anything (accept the recorded CHECKED/green
  statuses as facts about machine-checking);
- the Lean spike's mise/elan toolchain mechanics, except where they mis-price a
  cost claim in the plan.

REPORT: write exactly ONE file:
`C:\Users\ec\Sync\Code\Dorc\.claude\research\refinement-types-industrial-cost\crosscheck\report-fable-neutral.md`
- Findings severity-ranked. Each finding: a slug (minimum three full hyphenated
  English words), the claim in one-two sentences, evidence (file + section or
  lines), and a certainty marker (+SURE / ~SUSPECT / -GUESS / --WONDER).
- Include a section on what HOLDS — the load-bearing judgements you examined
  and found sound (name them; this is as much the deliverable as the faults).
- Close with one paragraph: does any finding invalidate a decision the record
  marks as human-typed (turn06's ack-ledger; the typed rulings)?
- Cite by path+section; do not paste large corpus excerpts.
- Your final assistant message: only the report path plus a 5-10 line summary.

=== END DISPATCH: fable-neutral ===

=== DISPATCH: fable-adversarial | mode=review ===

SAFETY (from the project's standing law; binds you absolutely):
- No git mutation of any kind; never, ever push. You have NO commit authority —
  the conductor commits. You are READ-ONLY on the repository except for exactly
  ONE file you will create: your report (path below).
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config).
- Real-command strawmen in the repo are frozen evidence; never execute fixture
  material; never hand-run books/mocks/rendered artifacts. Do not run builds or
  test suites at all — this is a reading review.
- NEVER read `Research/notes/quarantine-DO-NOT-READ/` or `Research/corpora/`.
- NAMING LAW (human-typed): the effort under review is deep research, NOT a
  numbered round — do not mint any rNN label for it in your report's name or
  prose (citing existing corpus doc-IDs like `28R:slug` is fine; era reference,
  if unavoidable: "the tail of r28").

THE TASK. A research conductor ran a multi-day research-and-design effort on
adopting higher-correctness tooling for this project's static-analysis kernel
(result-certification, Kani/property tiers, Flux refinement types, a Lean model
of the "sparing algebra", Aeneas translation, DST extensions), producing a
six-turn research round, a synthesis-of-record, a post-synthesis amendments
ledger, and a Lean formalization spike. I do not trust it, and I want it broken
where it breaks. My specific suspicions, stated first-person:

- I think some of the round's conclusions lean on thinner or more misread
  evidence than their confident wording admits, and that the grading ritual
  launders secondary claims into load-bearing ones.
- I think the plan-state — a synthesis overwritten by amendment layers ruled in
  fast dialogue — has accumulated internal contradictions and unreconciled
  redundancies that nobody has re-read for coherence end-to-end.
- I think the tradeoff pricing flatters a direction that was already preferred,
  and the "honest costs" paragraphs are the visible tip of costs that are
  actually larger (toolchain pins, agent-loop brittleness, sync taxes).
- I think the Lean spike proves tidy statements that are not quite the spec's
  statements — that between `notes/277`-as-written, the CLAUDE.md law
  registries, and the model's encoding choices there are unflagged semantic
  divergences beyond the seven gaps it self-declares, and that "zero sorry,
  clean axiom profile" is doing rhetorical work the statements don't earn.
- I think at least one claim made about the kernel code does not survive
  actually reading `lattice.rs` / `solve.rs` / `claim.rs`.

Find where this effort breaks down, and say so plainly. GUARD, equally binding:
where a suspicion of mine genuinely does not hold, report that it does not hold
and why — do not manufacture a fault to satisfy the framing; a fabricated or
inflated finding is worse than none. The corpus's own hedges are visible in its
text; an honest, complete hedge is a point in its favor, not a target.

Constraints they specified, to be followed exactly:
- The source base is taken as given: grading may be critiqued, coverage gaps
  may be noted, but no literature is re-gathered (spot-checking one specific
  quotation against its recorded URL is allowed if a finding turns on it).
- The amendments ledger supersedes turn06 where they disagree; judge the
  COMBINED plan-state, not the mere fact of supersession.
- The proofs are machine-checked; do not re-build anything. Statements,
  encodings, and faithfulness are the target.

READING LIST (all under `C:\Users\ec\Sync\Code\Dorc`; read what you need, deeply):
- Research round: `.claude\research\refinement-types-industrial-cost\turn01-2026-08-13-notes.md`
  through `turn06-2026-08-13-notes.md` (turn06 is the synthesis-of-record) +
  `sources.json` (22 graded sources; raw archives NOT present locally).
- Amendments ledger: `.claude\research\refinement-types-industrial-cost\crosscheck\input-turn06-pending-amendments.md`
- Lean spike (linked worktree; read-only):
  `.claude\worktrees\agent-a46d5d32e3e70a008\.claude\research\refinement-types-industrial-cost\spike-lean-sparing\`
  — `README.md`, `REPORT.md`, `SparingAlgebra\{Coordinate,Dialect,Compare,Sparing,Laws}.lean`.
- The spec: `Research\notes\277-entity-algebra-design.md` (esp. §§1-3, §5, §6),
  `Research\notes\272-address-derived-topology.md` §4, `spike\CLAUDE.md` (the
  "Invariants — separation, worlds & survival" cluster),
  `spike\crates\core\CLAUDE.md` (the "Law — the coordinate" section).
- Kernel: `spike\crates\analysis\src\lattice.rs`,
  `spike\crates\analysis\src\solve.rs`, `spike\crates\core\src\claim.rs`,
  `spike\crates\analysis\CLAUDE.md`.
- Only if a finding turns on them: `Research\plans\021-static-analysis-engine.md`
  (§1), `Research\plans\055-analysis-architecture.md` (dec-6),
  `Research\plans\271-block-settle-rulings-ledger.md`,
  `Research\notes\28R-context-kernel-review.md` (+ per-lane siblings), root
  `KNOBS.md` / `ANALYZER-NEEDS.md` / `AID-NEEDS.md`.

PROCESS MANDATE (in this order):
1. Deep read of the core materials first.
2. Then a reasoning-only pass — no tools — in which you lay out your report
   structure and commit your initial judgements.
3. Only then, focused verification reads (and, if genuinely useful, read-only
   Explore subagents for mechanical searching — never for judging; do not let
   any subagent's output frame your review).

EXCLUSIONS (out of scope; do not spend attention here):
- style/idiom/naming taste, comment style, formatting;
- re-running literature research;
- re-building or re-running anything;
- the Lean spike's mise/elan toolchain mechanics, except where they mis-price a
  cost claim in the plan.

REPORT: write exactly ONE file:
`C:\Users\ec\Sync\Code\Dorc\.claude\research\refinement-types-industrial-cost\crosscheck\report-fable-adversarial.md`
- Findings severity-ranked. Each finding: a slug (minimum three full hyphenated
  English words), the claim in one-two sentences, evidence (file + section or
  lines), and a certainty marker (+SURE / ~SUSPECT / -GUESS / --WONDER).
- Include a "considered and rejected" section: each suspicion above (and any
  criticism you developed yourself) that did NOT survive contact with the
  material, with the reason it fails.
- Close with one paragraph: does any finding invalidate a decision the record
  marks as human-typed (turn06's ack-ledger; the typed rulings)?
- Cite by path+section; do not paste large corpus excerpts.
- Your final assistant message: only the report path plus a 5-10 line summary.

=== END DISPATCH: fable-adversarial ===

=== DISPATCH: sol-neutral | mode=review ===

You are reviewing, read-only, a multi-day research-and-design effort in this
repository (cwd = the repo root). It examined adopting higher-correctness
tooling for the project's Rust static-analysis kernel: result-certification of
the fixpoint solver, Kani/property tiers, Flux refinement types, a Lean 4 model
of a "sparing algebra", Aeneas Rust-to-Lean translation, DST extensions. It
produced a six-turn research round with graded sources, a synthesis-of-record
(turn06), a post-synthesis amendments ledger, and a Lean formalization spike.
Assess the ANALYSIS (not the literature):

a. Internal validity — are the round's conclusions supported by its own cited
   evidence? The source base is a given; its grading may be critiqued; missing
   coverage may be noted as a gap, never re-gathered.
b. Plan coherence — is the final portfolio (turn06) plus the amendments ledger
   coherent, internally and against the project's standing invariants (the
   CLAUDE.md registries, the typed rulings both documents cite)? Are tradeoffs
   correctly priced (toolchain pins, churn, agent-loop brittleness)?
c. Formalization faithfulness — is the Lean spike faithful to the spec AS
   WRITTEN? Are the theorem STATEMENTS the right statements (proofs are
   machine-checked; statements are not)? Are its seven self-declared SPEC-GAPS
   correct and complete enough, or did it miss underdetermination?
d. Kernel fidelity — do claims made about the kernel code hold against the
   actual `lattice.rs` / `solve.rs` / `claim.rs`?

The corpus's own hedges are visible in its text; judge whether the hedging is
honest and complete, but do not limit yourself to already-flagged spots.

READ (paths relative to the repo root; read deeply, in roughly this order):
1. `.claude/research/refinement-types-industrial-cost/turn06-2026-08-13-notes.md`
   (synthesis-of-record), then `turn01` … `turn05` (evidence base), then
   `sources.json` (raw source archives are NOT on disk; treat quotes as given).
2. `.claude/research/refinement-types-industrial-cost/crosscheck/input-turn06-pending-amendments.md`
   — the amendments ledger; it SUPERSEDES turn06 where they disagree; judge the
   combined plan-state.
3. The Lean spike, inside a linked worktree directory:
   `.claude/worktrees/agent-a46d5d32e3e70a008/.claude/research/refinement-types-industrial-cost/spike-lean-sparing/`
   — `README.md`, `REPORT.md`, and all of `SparingAlgebra/*.lean`.
4. The spec it formalizes: `Research/notes/277-entity-algebra-design.md`
   (§§1-3, §5, §6), `Research/notes/272-address-derived-topology.md` §4,
   `spike/CLAUDE.md` (section "Invariants — separation, worlds & survival"),
   `spike/crates/core/CLAUDE.md` (section "Law — the coordinate").
5. Kernel: `spike/crates/analysis/src/lattice.rs`,
   `spike/crates/analysis/src/solve.rs`, `spike/crates/core/src/claim.rs`,
   `spike/crates/analysis/CLAUDE.md`.
6. Only if a finding turns on them: `Research/plans/021-static-analysis-engine.md` (§1),
   `Research/plans/055-analysis-architecture.md` (dec-6),
   `Research/plans/271-block-settle-rulings-ledger.md`,
   `Research/notes/28R-context-kernel-review.md`, root `KNOBS.md`,
   `ANALYZER-NEEDS.md`, `AID-NEEDS.md`.

HARD CONSTRAINTS:
- Read-only. Change nothing. Run no builds, no tests, no fixture material.
- NEVER read `Research/notes/quarantine-DO-NOT-READ/` or `Research/corpora/`.
- Do the work yourself, in this one run; spawn nothing.
- FAIL-FAST: if you cannot actually read the listed files from your sandbox,
  say exactly that and stop — do NOT produce a review from priors.
- No literature re-research. (Web search is available; use it at most to
  spot-check one recorded quotation against its recorded URL if a finding
  turns on it.)
- Out of scope: style/naming taste; formatting; the spike's mise/elan toolchain
  mechanics except where they mis-price a plan cost.
- Do not label the effort under review with any round number.

OUTPUT: your entire final message IS the report (it is captured verbatim to a
durable file). Structure:
- Findings, severity-ranked. Each: a short hyphenated slug (>=3 full English
  words), the claim in 1-2 sentences, evidence (file + section/lines), and a
  confidence marker (+SURE / ~SUSPECT / -GUESS / --WONDER).
- A section "what holds": the load-bearing judgements you examined and found
  sound — named explicitly; this is as much the deliverable as the faults.
- A closing paragraph: does any finding invalidate a decision the record marks
  as human-typed (turn06's ack-ledger; the typed rulings)?
- Cite by path+section; do not paste long excerpts.

=== END DISPATCH: sol-neutral ===

=== DISPATCH: sol-adversarial | mode=review ===

A research conductor in this repository (cwd = the repo root) ran a multi-day
research-and-design effort on adopting higher-correctness tooling for the
project's Rust static-analysis kernel (result-certification of the fixpoint
solver, Kani/property tiers, Flux refinement types, a Lean 4 model of a
"sparing algebra", Aeneas Rust-to-Lean translation, DST extensions). It
produced a six-turn research round, a synthesis-of-record (turn06), a
post-synthesis amendments ledger, and a Lean formalization spike. I do not
trust this work, and I want it broken where it actually breaks. My suspicions,
first person:

- I think some conclusions rest on thinner or more misread evidence than their
  confident wording admits, and the grading ritual launders secondary claims
  into load-bearing ones.
- I think the plan-state — a synthesis overwritten by amendment layers ruled in
  fast dialogue — carries internal contradictions and unreconciled redundancy
  nobody has re-read end-to-end.
- I think the tradeoff pricing flatters an already-preferred direction, and the
  "honest costs" lists understate toolchain-pin, agent-loop, and sync-tax
  costs.
- I think the Lean spike proves tidy statements that are not quite the spec's
  statements — that beyond its seven self-declared gaps there are unflagged
  semantic divergences between `notes/277`-as-written, the CLAUDE.md law
  registries, and the model's encoding choices — and that "zero sorry, clean
  axiom profile" does rhetorical work the statements don't earn.
- I think at least one claim about the kernel code fails against the actual
  `lattice.rs` / `solve.rs` / `claim.rs`.

Find where it breaks down; state each break plainly with evidence. BINDING
GUARD: where one of my suspicions does NOT hold, say so explicitly and why —
do not manufacture or inflate a fault to satisfy this framing; a fabricated
finding is worse than none. An honest, complete hedge in the corpus counts in
its favor.

Their constraints, to be followed exactly:
- The source base is a given: grading may be critiqued, coverage gaps noted,
  no literature re-gathered (at most, spot-check one recorded quotation
  against its recorded URL if a finding turns on it).
- The amendments ledger supersedes turn06 where they disagree; judge the
  COMBINED plan-state, not the mere fact of supersession.
- Proofs are machine-checked; re-build nothing. Statements, encodings, and
  faithfulness are the target.

READ (paths relative to the repo root; read deeply):
1. `.claude/research/refinement-types-industrial-cost/turn06-2026-08-13-notes.md`,
   then `turn01` … `turn05`, then `sources.json` (raw archives NOT on disk).
2. `.claude/research/refinement-types-industrial-cost/crosscheck/input-turn06-pending-amendments.md`
3. The Lean spike, inside a linked worktree directory:
   `.claude/worktrees/agent-a46d5d32e3e70a008/.claude/research/refinement-types-industrial-cost/spike-lean-sparing/`
   — `README.md`, `REPORT.md`, all of `SparingAlgebra/*.lean`.
4. The spec: `Research/notes/277-entity-algebra-design.md` (§§1-3, §5, §6),
   `Research/notes/272-address-derived-topology.md` §4, `spike/CLAUDE.md`
   ("Invariants — separation, worlds & survival"),
   `spike/crates/core/CLAUDE.md` ("Law — the coordinate").
5. Kernel: `spike/crates/analysis/src/lattice.rs`,
   `spike/crates/analysis/src/solve.rs`, `spike/crates/core/src/claim.rs`,
   `spike/crates/analysis/CLAUDE.md`.
6. Only if a finding turns on them: `Research/plans/021-static-analysis-engine.md` (§1),
   `Research/plans/055-analysis-architecture.md` (dec-6),
   `Research/plans/271-block-settle-rulings-ledger.md`,
   `Research/notes/28R-context-kernel-review.md`, root `KNOBS.md`,
   `ANALYZER-NEEDS.md`, `AID-NEEDS.md`.

HARD CONSTRAINTS:
- Read-only. Change nothing. Run no builds, no tests, no fixture material.
- NEVER read `Research/notes/quarantine-DO-NOT-READ/` or `Research/corpora/`.
- Do the work yourself, in this one run; spawn nothing.
- FAIL-FAST: if you cannot actually read the listed files from your sandbox,
  say exactly that and stop — do NOT produce a review from priors.
- No literature re-research beyond the single-quotation spot-check allowance.
- Out of scope: style/naming taste; formatting; the spike's mise/elan toolchain
  mechanics except where they mis-price a plan cost.
- Do not label the effort under review with any round number.

OUTPUT: your entire final message IS the report (captured verbatim to a durable
file). Structure:
- Findings, severity-ranked. Each: a short hyphenated slug (>=3 full English
  words), the claim in 1-2 sentences, evidence (file + section/lines), and a
  confidence marker (+SURE / ~SUSPECT / -GUESS / --WONDER).
- A "considered and rejected" section: each suspicion above (and any criticism
  you developed yourself) that did NOT survive contact with the material, and
  why it fails.
- A closing paragraph: does any finding invalidate a decision the record marks
  as human-typed (turn06's ack-ledger; the typed rulings)?
- Cite by path+section; do not paste long excerpts.

=== END DISPATCH: sol-adversarial ===
