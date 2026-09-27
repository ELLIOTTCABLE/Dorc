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

## front-single-artifact-prose-and-checks: established (turn02, subagent-graded; conductor read the notes' citations, not the archived copies, to preserve context)

- No standard single-artifact format exists that is both mechanically analysable and carries
  a typed notion of evidence for claims no tool can check. A 2015 survey states the gap
  directly and nothing found since closes it in a maintained tool
  [A-ernst-dependability-case-language-2015]. +SURE the gap is real, not a search failure.
- The nearest thing is a research artifact: an Alloy model in which every component property
  is guarded by an uninterpreted `evidence[tool, args, kind]` predicate, the checker runs the
  named tool to discharge it, and an "expert evidence" kind (a prose document with author and
  date) covers claims only a human can make, with sufficiency left to audit
  [A-pernsteiner-safety-case-pluggable-checkers-2016]. Forcing evidence onto every claim
  exposed a modelling error. Never mainlined [C-alloy-discourse-dependability-cases-2024].
  ~SUSPECT this is the shape to copy: the checkable part and the residue live in one
  artifact, and the residue is typed, attributed, and dated rather than merely prose.
- Standards bodies converge on one pattern: a stable in-prose ID per normative sentence
  (Vulkan VUIDs, RISC-V `norm:` anchors), the check or the recorded refusal to check kept in a
  separate artifact keyed by that ID, a reasoned register of IDs declared uncheckable, and a
  drift rule that a semantic change retires the ID [A-khronos-vulkan-vuid-style-guide-2026]
  [A-khronos-vvl-unimplementable-validation-2026] [A-riscv-normative-rules-tagging-2026]. The
  corpus's `docID:slug` discipline already is the ID half; the drift rule and the register are
  what it lacks.
- Literate containers exist and none reads the prose: Alloy Markdown runs the fences and
  ignores the text [B-alloy-docs-markdown-2023]; Quint and the Ethereum executable spec work
  the same way [A-quint-literate-specifications-2026] [A-ethereum-consensus-specs-md-to-spec-2026].
  No production literate-Alloy artifact was found. A container, not a tie.
- Proof-assistant documents are the one family where prose references are type-checked
  against formal entities and the unproved residue is a machine census
  [A-isabelle-isar-reference-manual-2026] [A-lean-reference-axioms-2026]. The Lean blueprint is
  the closest living practice: prose statements carry links to declarations, dependency
  edges, and author-asserted `leanok` claims; only the existence check is mechanical, the
  claim of completeness is by hand, and a lint for "sorry but marked done" was proposed and
  not built [A-massot-leanblueprint-readme-2026] [B-tao-pfr-blueprint-tour-2023]
  [B-leanblueprint-issue-absorb-functionality-2023].
- Requirements tooling keeps prose authoritative with stable IDs and adds three things worth
  copying: a per-item declaration of what coverage it owes, with "none owed" explicit
  [A-openfasttrace-writing-a-specification-2026]; a revision or fingerprint that voids links on
  semantic change [A-openfasttrace-concepts-and-terms-2026] [A-doorstop-item-reference-2026];
  and a typed discharge kind per sentence (implementation, test, implication, exception with
  reason, todo with issue) with a checked-in coverage snapshot that CI diffs
  [A-awslabs-duvet-annotations-2026] [A-awslabs-duvet-reports-2026]. The systems-engineering
  form of the same idea is a per-requirement verification method (analysis, demonstration,
  inspection, test) [A-nasa-se-handbook-product-verification-2023].
- Field evidence on the seam: links go stale and matrices fill with suspect links; human link
  vetting is wrong about a quarter of the time; links get minted at certification time
  [A-cleland-huang-traceability-trends-2014]. Two cautionary specimens from turn01: silo's
  sigil check is exact string-set equality and nothing more [B-gadget-silo-sigil-validator-2026];
  emilia's fact labels are decoration, and several of its checks restate a fact verbatim and
  cannot fail [C-emilia-protocol-relations-model-2026].
- Counter-thesis at scale: Cedar keeps the Lean model as the spec and differential-tests Rust
  against it, and publishes the bugs the seam missed [A-disselkoen-cedar-verification-guided-2024].

Conductor's read, ~SUSPECT, for the human's gate: the answer is a composable pattern, not a
product. Slugs are the IDs (already have). Add a typed discharge record per normative slug
(the duvet kinds plus Pernsteiner's attributed, dated expert-evidence kind for committee
speech), a drift rule (semantic change retires or re-revisions the slug), a generated
register like `SLUGS.md` that reports the residue by kind, and a CI-diffed snapshot. The
real fork is container shape: one literate Alloy-Markdown file per component holding prose,
fences, and the discharge table; or prose in `Research/` plus a model file plus a generated
seam register. Front 2 bears on which, since a proof-assistant terminal home favours the
second.

## front-structural-ties-between-instruments: established (turn03, subagent-graded; conductor read the notes, not the archived copies)

- No tie from an Alloy-family model to Kani, CBMC, or Rust code exists anywhere found; every
  tie found goes model-to-prover, model-to-generated-tests, or trace-to-spec. Absence is
  ~SUSPECT (small result sets), not +SURE.
- A finder and a prover share literally one statement only when the finder runs inside the
  prover's logic or both read one language: Nitpick translating Isabelle goals into Kodkod
  under a scope, with "genuine" versus "potential" counterexamples and an unsound mode
  [A-blanchette-nitpick-kodkod-isabelle-2010]; ProB running on Rodin's own proof obligations
  as disprover and prover [A-krings-prob-disprover-rodin-2015]; TLC, Apalache, and TLAPS over
  one TLA+ spec, each rejecting a different fragment, first combined on one spec in 2022
  [A-konnov-tla-trifecta-tlc-apalache-tlaps-2022]. Lean's counterparts are weaker: Plausible
  is random testing on the goal [A-lean-plausible-readme-2026]; Nunchaku's Lean frontend was a
  plan [A-cruanes-nunchaku-dependent-types-2016].
- A bounded check becomes a theorem only when the search is provably exhaustive and the
  encoding is itself proved: ProB accepts exhaustive no-counterexample as proof; Lean's
  `bv_decide` checks an LRAT certificate with a verified checker [A-lean-bvdecide-api-docs-2026];
  the general form yields "a true theorem about the wrong formula" unless the statement-to-CNF
  encoding is written and proved in Lean [B-szeider-lrat-catcher-lean-theorems-2026]. Nothing
  found puts Kodkod's translation under such a certificate.
- Alloy has been translated into provers five times, each a one-off (PVS 2007, KeY 2012, B
  2018, Coq 2019, Lean 4 via Forge 2024) [A-frias-dynamite-alloy-pvs-2007]
  [A-ulbrich-kelloy-alloy-proof-assistant-2012] [A-krings-alloy-to-b-translation-2018]
  [A-souaf-alloy-to-coq-translation-2019] [B-chen-lforge-forge-in-lean-2024]. The mismatches
  land in three places every time: bounded integers, finiteness (Lforge needs `Fintype` and
  `Inhabited` axioms per sig), and multiplicities dropped or hand-axiomatised. +SURE those are
  exactly the features 311 leans on (`lone`, `one`, per-shape arity, finite chains).
- Design specs and conformance specs pull in opposite directions: MongoDB abandoned trace
  checking against a 345-line design spec after ten engineer-weeks with 252 lines changed,
  while test generation from a spec transcribed from code gave full branch coverage
  [A-davis-extreme-modelling-mongodb-2020]. TLA+ trace validation is the strongest production
  seam discipline found: it checks the trace shares a behaviour with the spec, not refinement,
  and runs in CCF's CI [A-cirstea-tla-trace-validation-2024].
- Seams fail at interfaces, empirically: 16 bugs across three verified distributed systems,
  none in verified code; a specification omission that let a deduplication-disabling patch
  verify; a client assertion that restated its own branch condition and could never fail; a
  build tool that reported success when the prover crashed
  [A-fonseca-verified-distsys-bug-study-2017]. ProB found never-firing events in fully proven
  Event-B models; Nitpick refuted five formulas two provers had "proved"
  [A-blanchette-nitpick-kodkod-isabelle-2010] [A-krings-prob-disprover-rodin-2015].
- Agreement between independent toolchains is itself treated as evidence by the ProB,
  Alloy2B, and Nunchaku authors; relying on one counterexample strategy is "a mistake"
  [A-cruanes-nunchaku-dependent-types-2016]. The kimberlite specimen runs six instruments with
  no tie and a hand matrix that misdescribes its own CI [C-kimberlite-fv-traceability-matrix-2026].

Second-half inventory additions (the 17 unread ranges, read in full): seven small closed
vocabularies surfaced, none large: the ρ-claim grammar over predict bodies (`271`, four rungs,
TYPED), the decidable-condition fold (`28M` § 9, ACKED), cross-family registration (`28M` § 11,
UNRULED), monologue-versus-dialogue licenses (`28M` § 8, landed by construction), the transit
classes A to D (`26M`, superseded into `30W`), the `$0` authority spelling and the
withhold framing (`26N` § 5–6, ACKED). The one material correction: `26N` § 4, the capability
system, is design-complete and build-deferred, a keyword-by-context lattice with a join
`required(chunk) ⊑ measured(context)` and six invariants; it moves from cluster 4 to cluster 2.
Also: `28Q` has five of six stages built; `30J`'s dialect key is three-part and dissolves
`28M`'s committee fence; `rul-flag-is-razor-residue` is cited as load-bearing in three ranges
and was defined in none of them.

## front-shared-unit-in-referent-projects: established (turn04, subagent-graded; conductor read the notes)

- Where a proof tier consumes an example suite, it does so one way every time: an executable
  is extracted from the proof-side definitions and run under the ordinary test runner; the
  theorems never consume the examples, and the suite validates the *definitions* the proofs
  are about, not the proofs [A-wasmcert-coq-readme-2026] [A-wasmcert-isabelle-readme-2026]
  [A-armstrong-sail-isa-semantics-2019].
- Refinement lineages hold no example unit across tiers; their constant is an abstract state
  machine plus a state relation (seL4, KEVM's symbolic backend)
  [A-klein-sel4-formal-verification-2009] [A-kevm-evm-semantics-readme-2026].
- The held-constant input splits by where its verdict lives: stored in the unit (wast, Oils,
  goblint, SV-COMP, smoosh); computed by an executable reference per configuration (RISC-V
  ACT via Sail, Ethereum's pyspec, SibylFS as oracle); or absent, with majority vote across
  implementations (Csmith) [A-sv-benchmarks-readme-2026] [A-riscv-arch-test-readme-2026]
  [A-ridge-sibylfs-oracle-testing-posix-fs-2015] [A-yang-csmith-finding-bugs-c-compilers-2011].
- Stored verdicts carry per-implementation qualifications: Oils' OK / N-I / BUG overrides;
  goblint's `UNKNOWN!` (soundness) versus `UNKNOWN` (intended imprecision) versus `TODO`
  (precision owed) [A-oils-var-op-test-spec-file-2026] [A-goblint-developer-testing-guide-2026].
- Shell lineage: smoosh's unit is a script plus expected output and status run unchanged
  against the semantics and seven shells, written in Lem to extract to OCaml and Coq, with no
  proof consuming the suite; CoLiS's interpreter passed 8 of its 161
  [A-greenberg-smoosh-executable-posix-semantics-2020]. Oils requires a case green on other
  shells before OSH implements it [B-oils-spec-tests-wiki-2026]. CoLiS gives every seam its
  own unit and oracle, no cross-tier artifact [A-becker-colis-platform-maintainer-scripts-2022].
  SibylFS stores no verdicts, uses the model as oracle, and mechanically checks that every
  logically possible combination has a test [A-ridge-sibylfs-oracle-testing-posix-fs-2015].
- Drift control found: regeneration of stored verdicts from implementation output is
  characterisation (tree-sitter, Nix, goblint cram) [A-tree-sitter-writing-tests-2026]
  [A-nix-manual-running-tests-2026]; SV-COMP's rule that maintenance edits must not change the
  intended verdict; generation from an executable spec; per-release versioning with a review
  board [A-kubernetes-conformance-tests-guide-2026]; a manifest carrying identity
  [A-ethereum-consensus-test-formats-2026].
- Units accrue renderers (wast to JS to WPT; SV-COMP to Horn clauses; Sail to emulators and
  provers) [A-webassembly-test-suite-readme-2026]. Disagreement across consumers is triaged,
  not auto-resolved: RISC-V ACT's five-way triage (device, test, configuration, reference
  model, specification ambiguity) [A-riscv-arch-test-readme-2026]. Validation surfaced defects
  in the specs and suites themselves (POSIX suite bugs, Austin Group issues, a dash
  nonconformity).
- Product siblings have no cross-tier unit; the field is their only tier
  [A-hashicorp-terraform-acceptance-tests-2025] [A-ansible-integration-tests-guide-2026].
  Kubernetes is the exception, with a unit that graduates and demotes, the opposite of a
  never-graduating witness.

## The unit, as reasoned with the human (2026-09-27; read and not objected to; nothing ruled)

- Claims do not cross tiers; witnesses do. The model tier's claims are conditionals over
  speech ("given true declarations, `compare()` never lies"), whose truth-maker is
  consistency; the field tier's claims are about the truth of speech, whose truth-maker is
  correspondence with a host. § 0 and § 3.5 build the split in. So there is no consistent unit
  of claim across tiers, and there is a consistent unit of refutation for behavioural claims:
  the book, a few lines of sh with an expected verdict per line, which the panels already used
  against the abstract model and which is the product's own input at the field tier.
- Three kinds of claim: behavioural (share the book across tiers); structural (refag itself,
  complexity bounds; tier-local witnesses such as types, fences, counts; no book);
  correspondence (books, but only at the field tier). **[HUMAN]** ack that some concepts have
  no meaningful book and still deserve rigor.
- Vocabulary, the panels' own: book, rule (a slugged law restated per tier), walk (one tier's
  derivation of a verdict for a book), falsifier. Badges attach to rules; books never graduate,
  they accrue renderers. Engine nouns (mSort, mScheme) never appear in a book.
- The ratchet's content is the verdict-set over the book corpus, monotone except by human
  retirement; encodings churn beneath it. Seed corpus: `Research/GOTCHAS.md` (64), `311u`'s
  cases, the panels' witnesses. **[HUMAN]** hard ack on negative verification and mutation
  testing as a required guard.

## The proposed shape (conductor's synthesis, presented 2026-09-27; the human's reactions beside it)

One corpus of witnesses, one register of claims, one lock over both; every instrument reads
the corpus and writes to the register. Alignment of pieces that exist, not invention.

- Book corpus: a scenario as a speech record (declarations in force by owner and fingerprint;
  measurements with traversals and closures), sh lines, an expected verdict per line with the
  attribution set where the aid plane depends on it, and at least one falsifier. Books pin
  x1 and x2 only, never graduate, accrue renderers. Seed: `GOTCHAS.md`, `311u`, the panels'
  witnesses, `USER_STORY` stages as value books. A finding enters as a book that fails on the
  current model before its repair.
- Rule register: slug, quantifier prefix over (world, speech, design, implementation),
  dependency slugs fingerprinted as the x3 key, and discharges per instrument: `attributed`,
  `cased`, `model-checked(scope)`, `pinned`, `proved`, `demonstrated`, `kill-tested`, and
  `fenced` or `measured` for structural rules. Every ∃-witness travels with its ∀¬-fence; every
  fence with its un-fencing mutant. Ergonomics glosses that are checkable (add-one-true-
  statement-never-harms; gradual enhancement monotone) are two-speech-set rules, S ⊆ S′.
- Lock: computed-versus-committed mismatch fails in either direction; moves only by a human
  commit; the minispec catalogue lock extended downward, not a second lock. The residue report
  is one query: rules whose only discharge is `attributed`, by component.
  **[HUMAN] nack** on a generated index in the `SLUGS.md` style: "it's gotten large and
  unwieldy"; leaning to pull-generation (`internal-tooling md <something>` producing
  model-friendly, fully current output for a named form) or plain JSON, since models read
  JSON fine. So: the lock is a queryable artifact, its views generated on demand.
- Instruments: Alloy is the design tier's checker only (one module per component over a
  shared identity core; laws as `check`, books as exact-bound `run`, every `check` paired with
  a satisfiable `run`, a mutation lane, an equivalence check on every reword commit, pinned
  jar, small scope in CI and larger nightly). The speech record is the seam, schema from
  `311` § 5, hand-written and marked `hand` until the analyzer's front half produces it, then
  graduated by diff. `sparing-reference` and the `compare` chokepoint with Kani restate laws
  at bounds over the same records. minispec restates laws as `Prop`s with the records as the
  instance battery. DST renders books as simulated worlds; e2e runs them on hosts and is the
  only correspondence tier. Types and fences discharge structural rules. Panels are
  retargeted to attack encodings and mint falsifiers; their output is corpus entries.
- Container: prose stays in `Research/` with one discharge line per normative slug; models,
  books, and records under `spike/verify/` beside the harnesses. A weak lean, open.
- Flow: a 311 edit changes prose and module in one commit and runs the corpus; a flip is a
  finding, resolved by a human retiring a verdict in the lock commit or by repair. Graduation
  never rewrites a book.
- Front-3 corrections, conductor's read, presented 2026-09-27: `proved` is discharged over
  definitions the corpus validated (an executable extracted from the proof-side definitions
  runs the books; the theorems never see them); verdicts are stored intent, qualified in
  goblint's manner (`UNKNOWN!` soundness, `UNKNOWN` intended imprecision, `TODO` precision
  owed), never regenerated from implementation output; disagreement across instruments is
  triaged five ways (implementation, book, record, model, prose), named in the lock's failure
  output; SibylFS-style coverage (every matched shape times every answer has a book) is a
  candidate metric, cost unknown until the model's size is known.
- **[HUMAN]** 2026-09-27: "we're headed in a productive direction." No code proceeds under
  this conductor; the shape, if pursued, gets a fresh context and a concrete plan. The phase's
  final output is one `30Y` document extending `notes/301` and kin to cover design-tier
  theorems and Alloy; no separate synthesis document.

## Meta-test cases for the 30Y system (conductor's set, presented 2026-09-27; read by the human)

Twelve elements, each chosen to stress a different mechanism; names from the tree, no code
read. Forward from design: `311:3.2` two-tops (the full chain, x3 fingerprinting; three panel
books exist); `311:3.5` committee law (attributed, with the checkable fragment that every
verdict's attribution set is nonempty and names only declared speakers); `311:1.1` "the engine
never holds an mReferent" with `30T:inv-no-world-facts-in-engine` (structural; fenced;
human-acked fences only); `311:3.3` lifecycle mutation (a two-phase book with a reboot; the
temporal module). Backward from code: the Kani harness
`the_consumer_map_is_exhaustive_and_exclusive` (pins a consumer map `311:4.2` supersedes;
-GUESS it is green against the old map: the seam failure made visible); the minispec units
`JoinIsCommutative` and kin with their Kani twins (two instruments, no link; `proved` resting
on unvalidated definitions); the `sparing-reference` unit test
`provably_disjoint_feeds_survival_sparing` (a code-tier book already; register one row citing
both; rename in place); the DST pin `permuting_edge_insertion_moves_no_set_and_no_summary`
(a family-witness law). Lateral and value: `spike/fixtures/pi-webhost.book.sh` with
`package.oracle.sh` (stage 5's line 9 as `collide, TODO: spare once the stdlib keys the boot's
children`); `GOTCHAS:a-host-is-not-a-partition` (∀ over x3, fence and mutant, an NFS fixture
at the field tier). Non-311: `28M`'s withhold-only wall (∀¬ over composites, multi-author
book); `30D`'s decision table and `ORACLE_PROVIDES:provides-finished-definition` (table
completeness; a cardinality rule that is also an oracle lint). Left out by choice:
`kBACKFLIPS` (a weld; breach discipline is a different meta-test) and
`inv-known-held-never-convert` (types suffice; the system must allow "no design-tier
discharge, by decision" without reading it as residue).

**[HUMAN]** 2026-09-27: "extraction" overstates it; no complex mechanism is aspired to beyond
collation and monotonicity assertion. Some existing tests may deserve moving or enrichment so
they sit alongside the Alloy spec rather than only as e2e.

## The narrowing to two concerns, and speech as the crux (2026-09-27, human-typed and reacted)

- **[HUMAN]** the remit is two concerns and most else is noise: (1) LLM-driven design
  regression, a thing stated imprecisely and read wrong by a later rewrite, composition, or
  assumption; (2) correspondence between firmed claims and the implementation: the *same*
  cases run against the product, one-to-one greening, a single-source input that cannot be
  rewritten between checkers (an sh book with outcomes). Loader facts, sh behaviour, and host
  faithfulness are not concerns. The tool is tracked-not-proved: a pre-commit or CI gate an
  LLM runs before showing a claim to a human, as an alternative to a full adversarial review.
- **[HUMAN]** "no ruling without a book" is fine but vastly insufficient: books are small and
  stay green across design churn; what churns is a *statement* of user speech, from its
  spelling to its fundamental objects. Two claim classes beyond book-with-outcomes: same book
  and outcomes but the speech path is gradual and risk-monotonic; same book, outcomes, and
  single-user experience but collaboration carries less risk. Both require the speech slice
  in the claim. How user speech is encoded into the Alloy, and how that reaches tests, Kani,
  and Lean, is the whole game.
- Conductor's reply, acked in part: three layers of speech. L0, the ops proposition with its
  speaker, stable across design churn (`GOTCHAS.md` minus the speaker); L1, its
  classification under a design coordinate (species, relation, grain, danger), churns with
  311, is the record; L2, the spelling, churns with 312. Speech is an input relation with
  `speaker`, `kind`, `danger` labels, never a fact; `compare` is a predicate of the
  declaration set; the generic laws (never wrong when all true; monotone in speech;
  stranger-safe; attributed to the speaker only) are species-free and survive rewrites
  verbatim; per-species checks are the churny layer. **[HUMAN] ack** the three classes as
  spelled (run over the full set; check over prefixes with danger order; check over
  multi-speaker unions with attribution; cross-coordinate risk reduction as a lock diff,
  tracked not proved) as goals of `30Y`.
- **[HUMAN]** two corrections: L2 must enter at later stages, since the chartered evolution is
  abstract model, then spelling, then implementation, each keeping the old checkable truths
  and gaining ones unrepresentable before; and the L0/L1/L2 strawman as first shown carried
  no identifiers or spans linking an L0 proposition to its L1 line to the Alloy, so its value
  was not visible. **[HUMAN]** an unexpected finding: speakers may be static world facts
  ("ops distributes unevenly over speakers"), which is not how ops realities had been treated.

## Scope map: the modelable region (conductor's read over a Sonnet inventory of 40 corpus documents; the raw inventory is scratchpad-tier and not banked)

Heuristic for "worth modelling now", -GUESS as a rule, ~SUSPECT per cluster below: a closed
answer set plus ∀-shaped laws; a typed cut (design-of-record or ruled, not TABLED or DRAFT);
a supersession list or several consumers (blast radius); crosscheck or refutation history
(bug density). Anti-signals: TABLED, "nothing welded", superseded-in-part by a newer document
(model the newer one), or a component the newer model is dissolving.

**[HUMAN]** 2026-09-27: whether a component has code is not the inflection. The spike is a
meta-spike, proving how the tooling ties together and how LLMs work with it; most of the
design will fall short of the Kani and Lean line for a long time. The design-model
instrument may be the realistic, productive one for almost all of the spike and design.
Consequence for cluster 2 below: built-ness does not move a component out of design-model
territory; the small built algebras are candidates too, and cheap ones, since they are
already closed algebras with typed cuts.

- Cluster 1, identity and sparing, the region where modelling pays now and is wider than
  311: `311` plus what its § 4.2 supersedes and what its § 2.5, 2.6, 3.4 consume: the
  finished-definition record and entailment (`plans/30U`, unbuilt, "pending successor
  rewrite"), index-kinds and the pivot algebra (`plans/30W`, "nothing welded", six owed
  rulings), the filesystem binder's per-aspect identity (`plans/30T`, the stdlib File instance
  of 311), the residue of address-derived topology (`notes/272`, superseded-in-part), the
  wrapper lend algebra (`plans/27C` § 3 and `notes/273`, built with DST pins; 311 § 3.4 is its
  identity-side twin). One model family, several modules over a shared identity core; the
  only cluster with three crosscheck rounds behind it.
- Cluster 2, small closed algebras already built: the ternary verdict (`plans/239`, SIGNED,
  25 citing docs), the claim tier, the region-decision meet (`plans/30L`), the solve-certifier
  (`plans/302`), the value-grade lattice (`notes/275`), influence grades (`notes/306b` FIRM
  rules), ⊤-propagation (`plans/27C` § 3). Their instrument is Kani and minispec, already in
  place or owed there; a design model would be a retro-fit. Exception worth one tiny shared
  module: the verdict and claim tier as the vocabulary every other model's outcomes map onto,
  so consumer maps ("KNOWN_UNSPOKEN never spares") can be checked across modules.
- Cluster 3, decision tables and matrices never crosschecked, the cheapest wins: the
  prediction-channel contract's five-row table (`plans/30D`), the channel capability matrix
  (`plans/26O`), static loading's four-case cross-custody classification (`plans/30I`), the
  disjointness rc-predicate (`plans/30W` § 2). Completeness and disjointness of a table is a
  one-line check and catches the "text a builder must guess at" class. -GUESS an hour each.
- Cluster 4, temporal or state-machine shapes that are tabled or unruled: `HostPhase` and the
  failure taxonomy (`plans/260`, TABLED), the availability domain (`plans/28Q`, "no more
  piecemeal", 44 percent read), the emission planner (`plans/30P`), receipt typestates
  (`plans/30R`), the host-capability census (`notes/26N`, 31 percent read). Different
  instrument (Alloy 6 temporal, TLA+, P) and premature: modelling before the human has typed
  the cut reproduces the `312cg` § 21/22 apply-then-retract shape. Note the shape; defer.
- Dissolving, do not model: the selector dialect (`notes/277` § 3; 311 § 4.1 excludes it) and
  the pre-311 compare chokepoint (`notes/277` § 1–2; 311 § 3.2 replaces it).
- Not components: `notes/301`, `notes/30X`, `plans/128` are instruments and postures.

Sweep additions (a second Sonnet, open remit over everything the inventory excluded; root
documents and orphans were the misses, not old rounds):

- `Research/GOTCHAS.md`: 64 named forcing-functions, each a one-line counterexample. This is
  the corpus-wide refuted-shapes register, `311u`'s pattern at corpus scale; ~SUSPECT its
  entries are the first `run` cases for any model in cluster 1, and most of them are about
  identity (`a-host-is-not-a-partition`, `address-inequality-is-not-referent-inequality`).
- `ORACLE_PROVIDES.md`: 22 `provides-*` shapes with two closed answer sets, a four-value
  STATUS and a four-rung TRUST lattice (`271:rul-sin-ordering`). A cluster-2 shape (small
  closed algebra with a typed cut) that no inventory row carried.
- `plans/24S` (impossibility ledger `imp-1` to `imp-7`, mostly superseded) and `plans/28M`
  (seven "walls, each provably ∅"): impossibility claims are the cheapest model-finder
  checks of all, a `check` that must find no instance. Cluster 3.
- `FORFEITS.md`: 14 records on a fixed six-field schema, each a ⊤-narrowing claim with a
  back-out. Not a model; a register whose RULE fields are monotonicity statements a model
  could carry as assertions. Cluster 3, low.
- `plans/281`, the annotation mark grammar: a closed verb vocabulary and ~25 laws, but it is
  syntax. Different instrument (grammar and property tests), out of the design-model region.
- `plans/17N` (named-kinds discipline, F/inc/kill/fw registers) and `plans/102` (threat model):
  the first is pre-311 vocabulary now dissolving into cluster 1; the second is not modelable.
- Not opened by either pass, by density: `plans/282`, `288`, `24L`, `24R`, `310`, `233`,
  `16P`, `191`, `22W`, `24T`, and the `311a` to `311v` ledgers of the identity model.

Gaps in the inventory's read: `plans/28Q` from its fourth section on, `notes/26N` § 4 (the
capability census, the document's own central ask), `notes/306b` § 4b on, `plans/28M` § 8 on,
and the tails of `30J`, `28K`, `260`–`262`, `26K`, `26O`, `128`, `26M`, `30X`. The inventory
records exact unread line-ranges; a follow-up pass can start there.

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
