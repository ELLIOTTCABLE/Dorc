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

## The strawman (2026-09-27; `strawman.md` in this directory holds it verbatim, written by the human from the conductor's output)

- Shape shown: `world/objects.json` (named grammatical objects, NGOs, each `forall` or
  `exists`); `speech/<speaker>.json` (L0: one speaker, one claim, one object); per-coordinate
  `vocab.json` (species, relations, answers, `weaker` order, safe set, dangers, consumer map, as
  data), `rules.als` (hand-written beside the prose), `file.json` (L1: every L0 claim filed or
  explicitly unfiled), `measure.json` (each book's world read into the coordinate's species);
  books as `book.sh` + `world.json` + `expect.json` (intent in L0 ids and product verdicts);
  `laws/generic.als` (tool-owned: `Decl`, `Speaker`, `Danger`, `Ans` with declared order,
  `Query`, `True`, `answer`, `wrong`, `restsOn`; six checks: never wrong when all true,
  monotone in speech, stranger-safe, attribution honest, sufficient, minimal); generated
  instances with atoms named by L1 id; `lock.json`; later tiers (`spell.json` spans and a
  one-syllable count, loader recovery by content, a Kani harness built from the same ids,
  hostsim and e2e runs).
- **[HUMAN]** objects are `exists` or `forall`, meaningless except as objects of speech; the
  shape is yielded by 311 itself; the world's behaviour enters only at the very end, on a
  host. No `world/same.json`: declared once, a thing is one thing; two things get two names.
  **[HUMAN] nack** any proposal form: a proposal is edits to the spec; the whole is one
  specification summed across files and vocabularies, mechanically checkable in its totality;
  checking is running it and seeing green; acceptance is merging the diff or a human ack; git
  is the substrate and never a first-class concept of the tooling. **[HUMAN]** "largely
  overengineered, but that's what I expected."

## The inversion and the human's leans (2026-09-27, human-typed)

- **[HUMAN]** the JSON strawman is backwards: it authors awkward forms of what Kani, Alloy,
  and the readable specification each need. The inverted shape: a literate-Markdown
  specification as the primary product, with inline Alloy fences and inline NGO/speaker
  identifier fences, and tools that strip or generate the checker inputs. Its cost is losing
  a generated one-to-one mapping of shared things across representations. Conductor's
  reconciliation, presented and not objected to: logic hand-written in the target language
  and tied by name lints; data (objects, speech, filings, books) kept structured inside the
  document and generated into every target; Alloy's Markdown mode runs the document directly.
  JSON was over-tuned: structured data and tool output only; never sentence-shaped content
  inside a tool artifact except an explicit `md` endpoint that builds prose from facts.
- **[HUMAN] lean, "by law":** mechanical trumps prose when the two disagree, in any analysis
  where both can be present; a disagreement is a design question to sit and burn down into
  more mechanical clarity. The burn-down: the two disagree (most often the prose implies
  something the model does not encode); a reviewer notices; additive constraints go into the
  checker and the prose is tuned; a wrong prose tune can only be walked back by further
  additive narrowing. The checker's job is to force explicitness over time.
- **[HUMAN] lean, firewall:** mechanizable and unmechanizable rules cannot coexist in prose.
  Only sections that are entirely unmechanizable may carry normative prose; anything
  mechanizable is extracted to its own section containing no normative prose, fully normative
  by mechanization, so that commentary beside mechanical law never smuggles in normative bits
  licensed by inexpressibility.
- **[HUMAN]** refinements to the firewall: no section tag; a section is mechanized iff it has
  an Alloy fence. Inverted convention: all prose outside backticks is non-normative;
  normative content always sits in a fence, ```alloy or ```normative or similar. 311's
  reshaping is not urgent.
- **[HUMAN] ack:** name lints are probably a large part of the actual build-out: `simon-1` is
  one identifier everywhere; its Alloy form exists; that form mentions Alloy `Store` iff the
  claim relates to NGO `Store`; and so on.
- **[HUMAN] lean:** instances and classes are global across the product, not per document;
  an id means one thing everywhere and must cohere where repeated; a local variant is
  reminted under a new id with its own associations, never modified in place for one consumer.
- **[HUMAN]** attribution stays inside the model and the system; attribution is the product.
  Refag will be broken slowly downhill from 311 for product value; things must stay true
  regardless. A GOTCHA in classic form is one or more speaker statements plus a world and an
  outcome, one step below a speaker column. The answer-lattice observation is held for later.

## Corpus, inlining, and the book form (2026-09-27, human-typed leans)

- **[HUMAN] lean:** a new corpus tier for normative documents, something like
  `spec/311-identity-and-relations.md` at the project root; `Research/` is not built to hold
  very churny extra-normative documents. Most design documents will not reach that tier soon;
  the tier must be ready when they do.
- **[HUMAN] lean, convincible:** everything inline, no intended-to-be-edited global registry
  (a generated lockfile at most); duplication across documents is acceptable so context sits
  beside its subject for LLM and human reading; exact reuse of a book or an NGO across
  documents is expected to be rare; speakers will duplicate most, and opening each spec
  document with a fixed speaker list is fine.
- **[HUMAN] strawman, books:** every word of a book is a literal entity (`exists`) or a
  `<class>` (`forall`): `<cmd-any> arbitrary-arg <file-path>`. The book appears twice, the
  normative class form and a non-normative instance rendering, `chmod arbitrary-arg
  /etc/file.conf`. The checker then checks the ```sh content against the Alloy claims, that
  the classes the book mentions are the ones the claims are about.

- **[HUMAN]** refinements, same day: nack bindings, the tool is a shell tool and can deal with
  shell syntax; `{class}` is the inline forall syntax (braces are the least-overloaded
  choice); nack free text, it leads LLMs down refag roads, so a book is pure (normative,
  mechanized) and concrete only; nack duplicating the book, use inline comments instead. The
  sketch: a ```claims fence describing the claims in terms of users (a pared-down form of
  `sm.Path`, `sm.Inode`), then one ```sh fence in which each normative line is a `#}` comment
  over names (`#} cmd-chmod chmod-recursive <a-path>`) followed by the concrete line that
  inhabits it (`chmod -R g-w /srv/a`). A global table holds the names (`cmd-chmod: "chmod"`,
  `chmod-recursive: ["-R","g-w"]`, `a-path: Instance<sm.Path, "/srv/a">`). The normative
  lines are checked against the Alloy; the concrete lines are run someday and discussed; the
  tool enforces that each concrete line inhabits its normative line. Names are global and
  fixed project-wide (`/srv/a` is always `a-path`); a new constraint mints a new name
  (`an-etc-path`, `a-path-under-bind-mount`).

## Rules for the spec form, from the first drafted specification (2026-09-27, human-typed; `strawman-2/spec/two-paths-one-inode.md` stands unrevised until the rewrite)

- A normative shell word is either bare, an `exists`, a literal name lifted into the corpus
  and required to inhabit the same value in that word position everywhere (`#} chmod` over
  `chmod` lifts `chmod: "chmod"`); or braced, a `forall`, a class whose agreement across the
  corpus is about who uses it and what instances it gathers. Class names and instance names
  are unrelated unless expressed otherwise; the name space is one slugged space, so `path`
  cannot appear bare where `{path}` exists. Unambiguous words are typed as themselves;
  disambiguation is needed only where Dorc-visible claims tie names together.
- No `names` list: names are lifted where used; the only check is a lazy net for
  incoherence. The exception is a thing with underivable attributes, a speaker defined once
  for prosodic reasons, which has no other metadata. No `describes` per speaker (who spoke
  is an attribute of the claim); no declared classes; no declared instances.
- No prosodic notes inside any fence; fences are purely mechanical and carry their own
  meaning entire; commentary is allowed only outside, explicitly non-normative. Rule, possibly
  linted: no comment syntax at all in `sh` or Alloy fences; `#` only for machine-checked
  siblings; pure code. The one intentional hole: names are implicit claims about the world,
  so a long explicit name is the correct carrier of intent the code cannot hold, and names
  are correctness-sensitive. Lean: Alloy names match the model's names exactly (`MReferent`).
- Normative Markdown: one sentence per line, no hard wraps, in `> blockquote` marked
  `<!-- normative: -->`, so renderers wrap it; inverting 311's blockquote convention.
- Lean: drop the `#>` directives and let books repeat; `loads` spelled as sh, stilted if need
  be, compiled later; ideally few loads; specifications will eventually use shell loading
  order because Dorc does.
- **[HUMAN]** the oracle/speech-claim corner is the least baked; the rest is coming together.
- **[HUMAN]** style nits after the rewrite (`020d25fc`), for the next mechanical pass: indent
  the concrete shell line four spaces so it aligns with the mapping line; no long padded
  lines; invert to a three-line cadence, shell, then `#}` map, then `#>=` outcome, wrappable
  because each has its own introducer; fold repeated `.` load lines into a named fake oracle
  set, a block headed `# tessa-fs.sh` that other blocks source; reintroduce `# filename.sh`
  at the top of every code block, optional but favoured, since explicit slugs and permanence
  have served the project.

## The collapse, the churn walk, and derived ownership (2026-09-27; conductor findings the human read and reacted to)

- A claim collapses to one Alloy atom: a speaker, a species sig, and world-name parameters,
  written directly under the sig whose `fact` says what it means when true. The `claims` and
  `filed` fences and the proposition prose are gone; the claim's long name carries the
  world-intent; commentary about why the speaker can say it sits outside the fence. Coherence
  is structural: parameters are typed by the sig's fields. **[HUMAN]** agreed that `filed`,
  `alloy`, and `claims` were one thing and that the proposition prose was a restatement that
  could drift.
- `danger` is never authored: it is the inverse of `restsOn` per claim, mapped through the
  consumer map to the wrong answer a false claim would cause; a lock column.
- Static versus derived is structural: class sigs as parameters versus instance atoms. A
  measure is one tuple of one model relation, witnessed by a `#}=` line in the book whose
  `#=>` carries the tuple; the Alloy fact is lifted from it, never hand-written. Loads are set
  expressions in a run at the design tier and `.` lines in sh at the spelling tier. Both
  "least-baked" corners dissolve into the model; the harness holds only the welded verdict
  vocabulary, `MDecl` with a speaker, an ordered answer set, `Query`, and three predicates.
  **[HUMAN]** any declaration the harness holds must be a universal truth that cannot range
  over values as the product evolves; fine to try measures entirely in the book; it may come to
  resemble errorloom. Conductor: a book is a transcript case; -GUESS the loom case format can
  be reused outright.
- The churn walk over four of 311's own changes (one species to two; the aspect, cell,
  per-cell-store, and parent-declares generations; the closure rewrite; the re-seated store
  closure): the document changes (sigs, meaning facts, one line per claim under a touched
  species), the lock changes (verdict rows move; the reboot book fails a law at the
  singleton-sort generation, which was `311p` thread 2; the start-then-enable row's elide,
  guard, guard, elide is `312c` thread 4 as a diff), and the harness does not. Cost of a
  rewrite is countable as diff lines per claim.
- "Who can know" is not speaker metadata: 311 § 1.2 makes a sort one owner's vocabulary and
  § 3.5 lists per species whose line a statement must be. Ownership is derived: the speaker of
  a sort's identity claims owns it; a per-species owner-role table in the coordinate makes
  the seat rule a check, and `311p` thread 4 fails it. Strangers and cooperative cases fall
  out. A speaker needs a stable id and a prosodic line only. **[HUMAN]** the claim is the
  atomic unit of collaboration; a region of knowledge is the sort, principled because it is
  the engine's own unit; lean to name speakers by expertise and unification
  (`expert-docker-1`) with the mint-a-new-id praxis. Consequence: the identity claim is always
  the first line of any speaker's speech about a thing; an unowned sort's sort-level claims
  cannot pass the seat check, and its sites guard.

## The settled book form and the state at rewind (2026-09-27; human-typed rulings, conductor findings the human read)

- Two strata in the model. Truth: `reaches` and `worldParent` on `MKey`, named by no claim, read
  only by `wrong`. Knowledge: what the loaded speech says, including derived claims,
  `Resolution { of, to: lone }` and `Placement { of, within }`, whose speaker is derived from
  the owner of the key's scheme and whose truth is computed against the truth stratum, not
  assumed. A resolution with no target is a decline, always true. **[HUMAN]** the split of
  literal measurement from claimed measurement is the preferred approach; declining matters
  for loader, fail-through, and collaboration, since a declined answer can be re-answered, and
  that is unsettled by design, so the checker must be able to talk about it.
- Default resolutions are derived by the harness from the static scheme claims composed with
  the world facts, under the hypothesis that the lookup is correct, and generated into the
  book module; a human writes only deviations, a decline or a wrong lookup. **[HUMAN]** the
  written resolution lines felt like claims belonging with the oracle content; they were the
  hand-written default, and the errorloom analogy holds: setup is the world, the oracle's
  output is derived unless the test is about the oracle misbehaving.
- Two comment forms and nothing else. `#}` binds shell words to names, shell-lexed, `{}` a
  word kind, underscores so one spelling serves shell and Alloy. `#=` is line-scoped Alloy,
  lifted verbatim across consecutive `#=` lines, `this` bound to the site the line above
  generates, linted so every free name is that line's own: its map names, instance names it
  introduces, `Loaded`, claim atoms, `this`. A line whose `#=` holds no verdict form is a
  fixture line and binds no `this`. Cross-line statements go in an Alloy fence; the seven
  books have none. **[HUMAN]** collocation by scope is the principle; freeform Alloy beside a
  shell line is valid only where it is mechanically checked to hold of that line, and
  file-line references are a non-starter. The introducer characters are free to grow; the
  syntax serves only this project, keeps information local for an editing LLM, and is
  skimmable by the human.
- Verdicts are Alloy: `this in Ran`, `this in Elided`, `this in Guarded`, three subset sigs of
  `Site` defined once from the consumer map; the e2e compiler recognises exactly these forms
  to emit the expected transcript. **[HUMAN]** inverted from sugar: a fixed set of Alloy forms
  is the source and the expected-run files are compiled from them. Attribution is never
  authored: `restsOn` is computed and locked; an attribution intent is a scoped `#=` check
  beside the outcome it constrains (`… not in by[this]`).
- Books are named `# <name>.sh`; repeated loads fold into named oracle sets, one block per
  speaker, sourced by books. The site's argument is the last key-typed name on its map line,
  a generator convention, soft.
- 311 § 1.3 added as a fact: a scheme yields or is primary, never both; without it the laws
  module admits an instance where a stranger's primary claim on `Path` breaks monotonicity.
- State: `strawman-2/spec/two-paths-one-inode.md` and `strawman-2/build/` at the commit that
  follows this entry hold the final shape. Everything is unrun; every result in `report.json`
  is expected, not observed. Soft spots an implementer inherits: braced class words appear in
  no book, so `{class}` may be unnecessary in books; the `#=` verdict forms are the only
  vocabulary the e2e compiler knows; sites are single-argument, redirects and pipes are outside
  this slice; the claim name's speaker prefix duplicates the `speaker` field, one should give;
  re-answering a declined key by another scheme is representable and unmodelled; the temporal
  module (a resolution leaving `True` after a write, § 3.3) is the next missing species;
  `attributionSufficient` may fail on redundant speech, since `restsOn` is single-removal;
  scope five or six is the likely ceiling for the subset-quantified laws.
- **[HUMAN]** closing: no code under this conductor; the next step is a clean-context attempt at
  implementation from this ledger and the strawman, and the phase's durable output is one
  `30Y` document extending `notes/301` to design-tier theorems and Alloy.

## At the rewind (2026-09-27; the successor conductor's read-in, and the human's typed corrections to it)

- Conductor's observation, **[HUMAN]** "a true observation": the generic laws call `Query`,
  `answer`, `wrong`, `restsOn`, `by`, `Site`, and the three verdict sigs, all defined by the
  spec and none by the harness, so the tool's contract with a spec is a by-name signature that
  nothing yet checks. **[HUMAN]** a meaningful spec document must specify what it is talking
  about as part of its specification; the division between the harness and the part of a spec
  shared by most or all specs is a meaningful distinction, not noise. Where it lies is not ruled.
- **[HUMAN]** the spec tier is no-builder-edits, a do-not-touch tier during unattended build
  runs, near minispec's posture though not identical (minispec stays LLM-built). In practice
  the pattern is the same as every design corner: a frontier model and the human, attacked by
  adversarials and by eye, firmed slowly into something exhaustive. Far out; the present work
  is the Alloy harness only.
- **[HUMAN]** ack the shape: a minimal harness first, then back to 311 work to use it. **[HUMAN]
  nack** promoting the strawman into `spec/`: turning 311 into a spec is delicate, clean-context,
  product-focused frontier work, guaranteed to surface a dozen underspecifications at once.
- Conductor's holds, unreacted (chat-only until reacted): the laws module opens the claim
  atoms; the correspondence half stays out of the first cut.

## The agnostic recut of assay (2026-09-27; human-typed leans, and the consequences the human read and acked; the plan is `notes/30Y`)

- **[HUMAN]** 30Y is tooling only: what assay reads, builds, runs, reports. No praxis in it
  (where normativity sits, fence rules, name-correctness lints, which are not mechanically
  checkable). The tool's name is assay.
- **[HUMAN]** assay stays agnostic of anything 311-shaped, and as far as is reasonable of
  anything Dorc-shaped: a generic solver over sh-spelled lines mapped to abstract claims about
  those lines. No model-hygiene checks. It will eventually own the outcome vocabulary, for
  mapping between tools, and nothing else Dorc-shaped; punted.
- **[HUMAN]** Markdown in, Alloy out; JSON in, JSON out; a nonzero exit is how it speaks. No
  prose, opinions, suspicions, or hints, in text or smuggled into JSON. Lean on the user
  invoking Alloy directly wherever possible. Fancy reporting nacked for now; listed as later.
- **[HUMAN]** a class is an assay-specific opaque category of strings: arbitrary, never parsed,
  never meaningful, not a scheme; its correspondence with mScheme is accidental. It exists so two
  claims can agree about two different literals as one kind of text; where they can share the
  literal, they just do. Braced words are join nodes, not quantifiers (the prior conductor
  doubted a book forall exists); a "for every inhabitant seen so far" check may come later.
- **[HUMAN]** `#}` binds only the immediately preceding line, naming or classing each of its
  components; never a general truth over other lines. Any component may be classed, the
  command word included. A bounded word-set form (`{foo} rest... a_bar`, both ends bound) maybe
  later. Conductor's join rule, acked: the literal is the join key; one literal named on one line
  and classed on another is one atom carrying both.
- **[HUMAN]** the unit is the logical line; assay owns shell truths and exposes decomposed
  structure to Alloy (command and argv now; parts, redirects, substitution trees later), never
  literals. Nack on refusal machinery: absolute MVP, least work, hand the map line to the syntax
  crate's lexer.
- **[HUMAN]** statements of what flavour or scheme a thing is are claims, not fixture lines;
  `inode_x.class = inode` belongs to the oracle that mints inodes. Consequence, acked in the
  strawman: a class on a map line and a claim about the command's argument position type a named
  word; the fixture states world truth. Residue the strawman keeps: a book pins the scheme of
  each token the host printed (see the open-world finding below).
- **[HUMAN]** no declared literals in a spec: every word is minted by assay. Conductor's fix: a
  `words.als` module per document, below claims, holding every literal, class, and introduced
  name, with class memberships closed.
- **[HUMAN]** assay opens the tree-global `spec/shared` into every generated module; anything
  subtler is opened explicitly. Integers stay (argv positions; positional catalogs). Three-space
  indentation. Claims are atomic; a load is a set of them; no partial override; the wrapping
  concern is withdrawn; "oracle sets" are load files, since claims may later ride lines of
  oracle sh.
- Conductor findings the human read, unreacted beyond the ack to proceed: a line's run carries
  the lines above it (prefix runs), else a later run can dissolve an earlier query by choosing
  its convergence; a `#=` that is a declaration is emitted at module level; an argv word is a key
  only where a claim in force reads it, else a described verb's empty write set spares
  vacuously; a `run` over the open world a fixture leaves passes when any admitted world gives
  the verdict, so the book must pin what the solver would otherwise choose, and a `check` with
  convergence taken as given is the alternative to price in the first experiment; `Key in
  Shword` makes one value under two parents unrepresentable, a modelling choice for the 311 work.
- State: `strawman-2/` rewritten on this harness (`harness/assay.als`, `spec/shared.md`, the
  spec, `build/`); everything unrun; `report.json` is expected, not observed. Next: rewind and
  dispatch the implementation of `30Y`.

## Checker as adversary, and convergence as a claim (2026-09-27; human-typed leans, and the consequences the human acked)

- **[HUMAN]** the first purpose of assay, ahead of the two payoffs `30Y` listed, is to enforce
  rigor: make it hard for the design process to write a wishy-washy line into the spec. Strong
  lean: checker as adversary. Conductor: that is `check` semantics. A `run` passes when any
  world the fixture admits gives the verdict (the solver fills every gap in your favour); a
  `check` passes only when the verdict is forced (the solver fills every gap against you), and
  its counterexample is the sentence you forgot to write. The laws and corpus checks already
  were checks; only book outcomes were runs, inherited from a strawman whose generator pinned
  the world tightly enough that a run was as good as a check.
- **[HUMAN]** "the key whose word is `a_path`" is good, not a cost: a key is a first-class thing
  a line takes, and the spec should be explicit about it. What stays out of a spec is assay and
  Alloy minutiae (scopes, bounds, munged names). `Key` becomes its own sig carrying a word.
- **[HUMAN]** the engine's final decision, which lines elide, guard, and run, is as a whole what
  the model concludes; the outcome is always the checked statement, `Ran` included.
- **[HUMAN]** two characters at most for any new introducer, and suspicious that one is needed.
  It is not. The given the adversary must not flip is "this line converged", and **[HUMAN]**
  convergence is not a measurement-as-given: a measurement is a claim; only abstract world facts
  exist outside claims. Consequence, **[HUMAN]** "nearly ideal": a verdict claim
  `Verdict { of: one Line }` is declared on the site line (`one sig carl__… extends Verdict {}
  { of = this }`), its speaker derived from the verb's check owner, true by the static default
  until a spec models the state it measures; `Converged` is derived as the lines whose verdict
  claim is in force and true; a claim cannot be flipped. The static lift: on a site line a
  declaration is a claim and a formula is the outcome; world facts never need `this`. assay
  emits a `check` per line with the earlier lines' outcomes as premises and one `run` per book
  that all outcomes hold together. A line that runs writes only its outcome. **[HUMAN]** this
  makes "a convergence claim is fallible human speech" visible and atomic in the spec.
- Conductor finding, posed after the fact and corrected: with a first-class `Key`, Alloy's
  default scope of three no longer fits a book. **[HUMAN]** significant findings are posed
  before the patch, not resolved inside one; and assay cannot fix this, since it cannot know the
  sizing bounds of modelling objects.

## Scope is the spec's (2026-09-27; human-typed, from the first-principles explainer)

- **[HUMAN]** the ceiling must be configurable by the spec author and shareable across spec
  files ("a key is a thing across the entire spec"). Conductor, acked: Alloy has no
  document-wide scope, only a clause per command, so the spec spells it as a command with an
  agreed name and an empty body, `run bookScope {} for 12 but 4 Int` in `spec/shared`, which
  assay copies onto every command it generates; `bookScope_<book>` in the owning document
  overrides one book; a trailing `for` on an outcome line overrides one command. The ceiling
  caps one world, not a total across books; the headroom above what a fixture names is where
  the adversary builds counterexamples; the shared number is the common case plus headroom.
- **[HUMAN]** "eight named things" felt absurdly small, with the counter-thesis that the scope
  also sizes the exploration space, and a worry that scope pressure pushes toward not modelling
  interim things like keys, missing exactly where precision was needed. Conductor, acked:
  names are exact and free; the scope caps only unnamed kinds, so first-class keys added no
  atoms to any book, only a kind that needs a ceiling; eight was the strawman's largest fixture
  plus one and is a weak adversary; twelve for this strawman. The laws explore the free
  universe at scope six regardless of how books are sized.
- **[HUMAN]** a per-book escape is required: a gnarly book of eight commands and ten to fifteen
  keys may need a cap of twenty without the forty two-line exemplars paying for it. Conductor,
  acked: a scope is paid per command, so a large book's ceiling costs only its own commands;
  the three-tier convention above is the escape. `30Y` § 2.4 carries it as
  `mech-scope-is-spelled-in-alloy`; the strawman's shared module carries `bookScope`.
- Also this sitting: `Key.word` is `w` (so `w.inode_x`) and `Query`'s ends are `writer` and
  `reader`, a mechanical rename the human delegated.

## The pre-step: Alloy in anger before assay exists (2026-09-27/28; human-typed framing, conductor's actions)

- **[HUMAN]** not the `30Y` build: a last proving-out, deploying the upstream tool against a real
  Dorc problem without the constraints of perfect specification text. An Opus builder stands up
  the minimum that runs `.als` files (mise-managed JDK and Alloy jar, one runner task, one
  platform; no compiler, no scaffolding); the conductor hand-writes a second strawman, spec and
  build both, targeting completeness and compileability, on something tractable that is not
  311 so the state-space coverage grows; then the builder merges it and fights it toward green.
  Fable tokens go to the authorship, not the tooling.
- **[HUMAN]** the primary output of the series is the record of where Alloy fights the author,
  divided carefully at the end into good correctness-chafe and things the tooling should
  absorb. Where Alloy fights because the design is genuinely not firm, that is the tool doing
  its job: log it as a surfaced softness, then the conductor has full latitude to invent an
  answer for the strawman and pursue green, for this session only. The builder has no such
  latitude: it repairs mechanically and returns design questions to the conductor.
- **[HUMAN]** every file in a strawman directory carries a strawman, non-normative header, so
  invented decisions can never be read as Dorc's design. Also from this sitting: three
  backticks inline break the human's renderer; name a fence kind in words.
- Conductor's choice of subject: the per-line outcome algebra past a wall (elide, guard, run,
  survive under the flag; footprints, backings, withholds; the identity tier's answers as
  opaque claims), the vocabulary assay is meant to own one day and the region `USER_STORY`
  stages 2 and 5 narrate. `30Ya-strawman-3/` in the evidence base: nine books, ten laws, four
  corpus checks; `FINDINGS.md` there is the running fight record; everything unrun at the
  commit that carries it. Two design answers invented there and marked so: a guarded line is
  a wall (it may run), and attribution is the derivation's support rather than single-removal,
  because two independent falsehoods mask each other under removal.

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
