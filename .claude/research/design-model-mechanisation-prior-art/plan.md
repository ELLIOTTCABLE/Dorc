# plan — design-model mechanisation prior art

> Conductor-owned problem-space map for this research phase (the `interactive-research` skill's
> `plan.md`). Rewritten in place as the phase narrows; per-front gathering lives in the numbered
> turn files, each minted by its researcher. Grades on claims: +SURE / ~SUSPECT / -GUESS /
> --WONDER. Nothing here is ruled.

## Question

Can a significant subset of the project's design corpus, its relatively stable core components
as stated in prose, be mechanically built, regression-tested, and eventually tied to the
existing check-ladder (types, Kani, property tests, DST, runtime certification, minispec over
derived Lean), while each component is still churning as prose, without the mechanical copies
becoming cruft, a second design venue, or one more typed system sitting beside the others with
the incorrectness pushed to the seams?

Scope, per the human: several core components, treated one at a time. The identity-and-relation
model (`Research/notes/311`) is the first, because it is the core of the core and an unbounded
source of bugs when not carefully considered. The further components are the human's to name;
the method chosen must generalise across them, so single-model answers are weaker than
corpus-shaped ones.

## Landscape as currently understood (provisional; to be confirmed or overturned by the fronts)

- Instrument by phase. ~SUSPECT Alloy-class bounded model-finding fits a pre-code relational
  model (small-scope counterexamples, seconds per check, one-to-one with prose sentences);
  Kani shares its epistemics once the component is code; minispec/Lean is the terminal home
  once a component stabilises. The corpus's tooling work (`plans/021`, `notes/28T`, minispec)
  evaluated only code-side instruments; Alloy, Forge, and TLA+ appear nowhere in `Research/`.
- Artifact shape. Two candidates: one authoritative artifact per component carrying both the
  checkable claims and the normative-but-untestable ones, with the untestable residue tracked
  and driven toward zero; or prose plus a separate model with a checked seam. The corpus's own
  traceability substrate is `docID:slug` (root `SLUGS.md`, `mise run slugs`); any format must
  interoperate with it. minispec (English-authoritative prose + `Prop` + instance battery) is
  the in-house instance of the single-artifact posture for code-side laws.
- Ties between instruments. Either the model is structurally derived from or into another
  instrument (Nitpick is Kodkod inside Isabelle; Aeneas derives Lean from Rust; Nunchaku is a
  cross-assistant model finder; proof-producing SAT inside Lean), or the seam is irreducible and
  is checked differentially (Cedar's Lean model against Rust). The human's prior: typed systems
  beside each other push incorrectness to the seams.

## Axes the decision turns on

- Regression rate per prose rewrite (measured: the panels' text-drift population) against the
  drift cost of keeping a second artifact aligned.
- Phase and cost curve: what a rewrite of a churning component costs under each instrument.
- Abstraction level: the mechanical copy must sit at the prose's level (objects, relations,
  laws; no names, spellings, or UX) or it becomes a design venue by accident.
- Who edits, and how an edit is accepted: a check that fails is a finding routed to the prose
  and the human, never a silent pick; ambiguity the prose permits must surface, not be resolved
  by the encoder.
- Where it lives and how it composes across several components: per-component files with a
  shared vocabulary, or one model; how production users organise multiple models.
- What cannot be modelled: committee speech and who-can-know (refag), product-value decisions,
  unbounded structure. Whether that residue can be tracked in the same artifact.

## Fronts, serial

- front-production-alloy-users: which production, open-source projects actively use Alloy;
  where the model sits (same repo, directory, separate repo); whether CI runs the Analyzer; how
  model edits are governed and reviewed; a Rust example. Gathered by an Opus scout, banked as
  its own turn with `graded-by: subagent`.
- front-single-artifact-prose-and-checks: a standard format for one authoritative artifact
  holding checkable claims beside normative-untestable ones, with residue tracking. In the net:
  Alloy's markdown mode (fenced `alloy` blocks the Analyzer runs directly), git-native
  requirements-traceability tooling with coverage (Doorstop, StrictDoc, sphinx-needs,
  OpenFastTrace), literate proof-assistant documents (Isabelle document preparation, Lean
  Verso, Alectryon), ISO-style conformance clauses and RFC 2119 tables, minispec's posture,
  and the counter-thesis (two artifacts with a checked seam). The LLM-assisted translation
  angle (nl2spec and kin) is out of scope for this front: the human will use LLMs their own way.
- front-structural-ties-between-instruments: structural or provable ties between an
  Alloy-class model and Kani, Lean, or a proof assistant. In the net: Nitpick/Kodkod in
  Isabelle, Nunchaku, Lean's Plausible and proof-producing SAT, Alloy-to-SMT and
  Alloy-to-Isabelle translations, Event-B refinement chains to code, Aeneas-style derivation,
  Cedar-style differential checking as the counter-thesis, and the seam-pushing failure mode
  the human has observed.

## front-production-alloy-users: established (turn01, subagent-graded; conductor re-read the load-bearing archived copies)

- One model per concern, standalone, is the norm; shared vocabulary via `open` is rare and
  appears only in generated or fossil model sets. No project shows multi-year growth of a model
  set; every pre-2023 user is a single-model or single-burst fossil. ~SUSPECT this reflects
  adoption pattern more than the tool: the survivors with several models are all under a year
  old. +SURE the corpus's own layout (one module per `docID`, opening a shared identity core)
  has no production precedent to lean on either way.
- The only mechanical model-to-code tie found is silo's sigil sync: every `[SILO-*]` tag in a
  model must appear in a Rust comment and the reverse, run on every PR
  [B-gadget-silo-ci-alloy-jobs-2026]:21-22. Its Alloy verification itself is path-filtered to
  model and runner changes only [B-gadget-silo-ci-alloy-jobs-2026]:214-236, so a code change
  never re-runs the model. Tag-level traceability, not semantic. The sigil script and a
  sigil-bearing model were not read; front 1 fetches them.
- Vacuity is checked as a CI failure in one project (a `run` that finds no instance fails the
  build) [C-emilia-protocol-run-alloy-doc-2026]:58-61, which is exactly the empty-universal
  bug class the panels hit three times. Vulkan runs its ~90 litmus tests as goldens: expected
  SAT/UNSAT per test, diffed [A-khronos-vulkan-alloy-litmus-makefile-2019]:7-10.
- Models as regression records of real bugs: nixbot annotates each assertion with the
  production bug it caught and maps predicates to named Python functions by comment
  [B-nixbot-scheduler-alloy-model-2026]:81-82,154-156; aws-cdk keeps a `run` whose comment
  records the counterexample that reversed a belief [B-aws-cdk-iam-policy-merging-alloy-2022]:191-201.
  Both are the "refuted shape banked as a run" pattern `311u` needs.
- In-artifact traceability tables exist at low grade: emilia's per-assertion "facts relied on"
  column [C-emilia-protocol-run-alloy-doc-2026]:117-135. A standards body embeds the model in
  the prose spec itself (RISC-V ISA manual, `mm-formal.adoc`; observed via API, not read).
  Both are front 1 leads.
- Governance is thin everywhere: one CODEOWNERS entry [C-pta-standards-codeowners-formal-2026],
  one ADR choosing Alloy for relational fit [C-pta-standards-alloy-over-tlaplus-adr-2026], no
  evidence anywhere that model and code change in one PR. The big-name users (lnd, aws-cdk,
  gnatcoverage) hold one-time design artifacts with no CI [B-lnd-alloy-models-readme-2024].
- Grades to re-check later: the 2026 adopters (kimberlite, pta-standards, emilia, kiri, fspec)
  carry LLM-era signals and are graded C for that reason; their mechanisms are real files.

## Provisional findings (corpus-internal, from the in-chat analysis the human has read)

- The three crosscheck rounds over 311 (`311p`, `312c`, `312ch`) split into two populations:
  genuine model defects, narrowing in scope per round (species and seats; then procedure
  domains; then single clauses), and text-drift regressions from folds and rewrites, which do
  not narrow and scale with edit volume (76 commits to the file in 11 days, six rewriting 40 to
  100 percent of it). Round 3 was substantially a review of round 2's repairs.
- One bug class recurred three times across two rounds: a universal quantifier over an empty
  set read as vacuously safe (`312c` thread 6; `312ch:srv-unmarked-verdict-readset-closed-empty`;
  the pre-existing pin `notes/277` § 5 that ⊤ is never the empty set).
- `Research/notes/311u` is a regression suite in prose: each entry names a refuted shape and the
  case that refuted it; nothing re-runs it.
- Every panel witness so far fits a model-finder scope of 4 to 6 atoms (two directory entries
  and one inode; two filesystems on one disk; one stranger key).
- The process's structurally weakest verdicts are "no answer changed" after a rewrite and "no
  counterexample exists"; the `312cg` § 28 equivalence claim for the forward rewrite was passed
  by one lane and broken (`312ch:srv-injection-sentences-collide-every-fact`).

## Pending

- Scout task 1 report (21 web hits, buckets A to E, several graded from partial reads):
  scratchpad `kagi-scout-mechanising-models.md`. Only the fully-read subset is to be
  registered, by the scout, in its turn.
- Scout task 2 (production Alloy users): running; minting its own turn.
