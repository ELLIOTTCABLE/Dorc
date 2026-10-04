# plan — design-model mechanisation prior art

> Conductor-owned problem-space map for this research phase (the `interactive-research` skill's
> `plan.md`). Rewritten in place as the phase narrows; per-front gathering lives in the numbered
> turn files, each minted by its researcher. The design sittings this phase fed are ledgered in
> `Research/notes/30Ya`; the phase's durable outputs are `Research/notes/30Y` (assay, the tooling),
> `Research/plans/30Z` (the specification praxis), and `.claude/skills/using-alloy/` (the Alloy
> authorship praxis). Grades on claims: +SURE / ~SUSPECT / -GUESS / --WONDER. Nothing here is ruled.

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
- front-tractable-decomposition-of-checked-specifications (added 2026-10-04): how projects keep
  a large, interdependent, mechanically checked specification tractable while it grows. Its
  section below carries the question, the sub-questions, and the net.

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

## front-alloy-authorship-praxis: established (turn05 subagent-graded; turn06 and turn07 conductor-read; the output is `.claude/skills/using-alloy/`)

- The first-party sources teach one loop: declare, empty run, constrain from instances, re-run
  after every fact change, then check; every source names a way a command passes for the wrong
  reason, and the skill's vacuity catalogue is their union (turn06, all sources conductor-read).
- The field adds what the book does not name: a `run` labelled with a predicate's name never
  applies it [B-alloy-discourse-run-label-shadows-pred-2020]; `init; always next` leaves the first
  transition free [B-alloy-discourse-init-semicolon-first-transition-2022]; `until` asserts its
  right side [B-wayne-alloy6-about-time-2021]; novices are more often too permissive than too
  strict, and the empty set is the usual hole [A-jovanovic-novices-write-alloy-models-2024]; the
  pre-6 `sig Time` idiom persisted in half of a 2024 cohort
  [B-padalino-alloy6-temporal-teaching-module-2024] (turn05).
- Peer agent skills exist and are compressions of the same book [C-adzerk-alloy-skill-2026]; the
  one with an evaluation measured near parity with no skill on quality and a token saving, and
  corrects one blanket claim we had copied, that stutter is always mandatory
  [C-lablambworks-alloy6-evals-2026] (turn07).
- The testing literature supplies two mechanisms the skill lacked: the mutation-operator list as
  the concrete kill menu, with an equivalence check per mutant
  [B-wang-mualloy-mutation-testing-2018]; and the test as a valuation plus a command, negatable
  and partial, with size zero as a coverage criterion [B-sullivan-aunit-test-automation-2018]
  (turn07).
- Measured language-model behaviour, all on toy tasks: syntax errors in up to 14 of 20 and wrong
  formulas in up to 18 of 20, one feedback round clearing the syntax
  [B-hong-llms-writing-alloy-formulas-2025]; repair loops stalling by the fourth round
  [A-alhanahnah-llm-repair-alloy-specs-2025]; two agents rebuilding a helper from primitives
  instead of reusing it [C-lablambworks-alloy6-maintenance-reference-2026].

## front-tractable-decomposition-of-checked-specifications: established (turn08, subagent-graded; the conductor checked about twenty quoted excerpts against seven archived copies and read no source whole)

**[HUMAN]** 2026-10-04, the question: how do other projects split a specification into
manageable chunks? Any real specification, model, or protocol has many mutually interdependent
parts. It cannot be that every project using Alloy, Dafny, or TLA+ writes one interdependent
object of ten thousand lines that can only be checked whole, in runs of days. What is the
*practice* that keeps progress on a model-checked specification tractable?

Why it is asked now, as measured facts: the first specification written under this phase's
praxis (`specs/311-identity.assay.md`) is one closure of about 4,350 lines and 49 signatures.
Every command pays for the whole closure (about 20 s of translation per command for a law that
reads little; a full pass is hours; three laws do not finish at a 30-minute ceiling; a
three-line scenario's checks exceed 150 s). A plan to cut it by product question
(`Research/notes/313`) was reviewed (`Research/notes/313a`): the per-rule step check and the
induction over derivations stood, but the cut does not subdivide the work, since the answers are
mutually recursive, the hard nouns pool in a shared core, and a check made below has to be asked
again in the assembled universe.

What the earlier fronts already say, and its limit: among production Alloy users one standalone
model per concern is the norm and no multi-year model set exists (turn01); TLC, Apalache, and
TLAPS each take a different fragment of one TLA+ spec, and design specs and conformance specs
pull apart (turn03). Both fronts asked where models live and how they tie to code. Neither asked
how a large, interdependent specification is kept checkable while it grows.

Sub-questions, each wanting practice with a primary source behind it:

- `sub-what-the-unit-is` — by component, by property, by abstraction layer, by protocol phase,
  by feature or version; and what crosses between two units (an interface, shared constants, an
  assumed lemma, an environment model).
- `sub-how-interdependence-is-cut` — assume-guarantee, abstract or interface modules, refinement
  mappings, uninterpreted or axiomatised stand-ins, stubs; mutual recursion between parts above
  all.
- `sub-what-argues-the-composition` — whether the soundness of putting the pieces back together
  is proved, checked at a bound, tested, or knowingly left open.
- `sub-how-iteration-stays-cheap` — per-unit checking and caching; small constants and scopes per
  configuration; resource limits per obligation; a fast tier beside a nightly or scheduled one;
  simulation or random walks beside exhaustive runs; inductive invariants in place of
  reachability; symmetry and data abstraction.
- `sub-how-a-specification-grows` — step-wise refinement, a specification per feature or fork,
  layering; what is re-checked when one part changes.
- `sub-where-it-failed` — decompositions that did not pay, monoliths that were simply paid for,
  projects that changed instrument because checking stopped scaling, brittleness and timeouts.
- `sub-the-numbers` — specification size, check time, state counts, verification time per module,
  wherever a project reports them.

Counter-theses to look for with equal effort: successful projects keep each specification small
by abstraction and never compose them; large monolithic specifications exist and are paid for in
compute; the composition's soundness is accepted unverified in practice; model-checked
specifications stop scaling and projects move to deductive tools, or the reverse.

A primary source for this front is a specification repository itself (its directory layout, its
imports, its model-checker configurations, its CI), a paper or experience report by the
project's own authors, first-party tool documentation on modularity, or a maintainer's own
talk or essay. A listicle or a vendor's account of someone else's project is not.

Established (34 sources; the turn file carries a project table, counter-evidence, and the
scout's own applicability reads; leads not taken are listed at its tail: Quint, F*, CompCert,
hardware formal verification, mCRL2 and CADP, WebAssembly, RISC-V, K, among others):

- No project found checks a large interdependent object whole in its working loop. The practices
  that avoid it are five, below; each counter-thesis also found real support.
- Small and never composed is the commonest practice and a successful one: AWS's specifications
  ran 102 to 939 lines, one per component, with nothing crossing between them
  [A-newcombe-formal-methods-at-aws-2014]; Chord is about 100 lines
  [A-zave-lightweight-modeling-chord-2012]; ShardStore keeps one small model per component and
  never checks their composition [A-bornholt-shardstore-lightweight-fm-2021].
- A growing design is a chain of separate specifications by abstraction level, each tied to the
  one above by a refinement mapping and checked on a tiny model in about a second; the levels are
  never assembled [A-lamport-voting-tla-refinement-module-2019]
  [A-lamport-paxos-tla-refines-voting-2019]. Every proved layering needs parts that depend on
  each other in one direction only [A-taube-ivy-modularity-for-decidability-2018]
  [A-gu-certikos-certified-abstraction-layers-2015] [B-silva-eventb-decomposition-tool-2011].
- Mutual dependence is handled by circular assume-guarantee: each part is checked against
  stand-in abstractions of the others. The composition theorem is proved on paper by induction
  over trace length; the obligations are tested or checked by hand
  [A-desai-modp-compositional-testing-2018] [B-gu-ipa-compositional-tla-consensus-2022]. It paid
  10x to 288x where a part had internals to hide and about 3x where it did not, and one module
  was merged back.
- The unit of iteration is the obligation, with only what it uses in view: TLAPS fingerprints
  each obligation after cutting its context to the hypotheses used
  [A-cousineau-tla-proofs-fingerprints-2012]; MongoDB cut one induction step into twenty
  conjuncts by eight actions [A-schultz-mongoraftreconfig-tlaps-proof-2022]; VeriBetrKV hides
  definitions by default and fixes any unit over twenty seconds before work continues
  [A-hance-veribetrkv-disciplined-automation-2020]. One-step induction replaces trace search
  [A-apalache-running-inductive-invariants-2026] [A-practicalalloy-inductive-invariants-2026].
  Alloy has no counterpart: every command carries every fact
  [A-practicalalloy-module-system-2025], and the Alloy research tools for incremental analysis
  all need sparse dependence [B-wang-ialloy-incremental-evolving-alloy-2019]
  [B-zheng-platinum-reusing-constraint-solutions-2020].
- What stays monolithic is paid for by schedule: CCF runs a 300-second simulation on pull
  requests and exhaustive checks weekly, narrows the state space in a wrapper module and never in
  the specification, and fails a run whose coverage invariant was never reached
  [A-ccf-tla-shallow-verification-workflow-2026] [A-ccf-mcccfraft-harness-module-2026]; the TLA+
  corpus runs models under 30 seconds to completion and smoke-runs the rest
  [A-tlaplus-examples-contributing-guide-2026].
- The large Alloy developments are smaller than ours and were paid for in hours: Mondex, about
  nine modules of 34 to 274 lines, one command per theorem, existence separated from refinement,
  explicit witnesses in place of existentials, a hardest theorem that did not finish in four days
  until a human case split [B-ramananandro-mondex-alloy-refinement-slides-2006]; the flash
  filesystem's refinement check, about eight hours at scope five
  [A-kang-flash-filesystem-alloy-2008].
- Putting the parts back together is the least mechanised step in the flagship projects (a paper
  proof in IronFleet, a hand check in IPA, tests in ModP)
  [A-hawblitzel-ironfleet-layered-refinement-2015]. Projects also change instrument: MongoDB to a
  TLAPS proof; ShardStore away from Alloy and SPIN to executable reference models in the
  implementation language.

Conductor's read, ~SUSPECT, for the human's gate:

- The identity specification is larger than any Alloy development whose size the scout found
  reported (Mondex's modules sum to about 1,700 lines), and it is one closure. The prior art does not offer a way to keep an object of that shape cheap; it offers
  ways not to have one.
- The per-rule step check that `Research/notes/313a` reports as standing is circular
  assume-guarantee under another name, and leaving its composition as a paper induction beside a
  bounded assembly check is the mainstream posture, not a shortcut.
- The cheapest practice with the widest support is the one this phase has not tried: many small
  standalone models, one per design question, each re-declaring the little vocabulary it needs,
  never composed. This sitting's own scratch models were of that kind (about a hundred lines,
  checks in milliseconds). Its price is that global coherence is argued and not checked.
- Three tactics transfer with no change of approach: a hard time limit per command, fixed before
  work continues; a human case split, with a coverage check, for the laws that do not finish;
  explicit witnesses in place of existentials.

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

- Nothing in flight. `front-tractable-decomposition-of-checked-specifications` is banked as
  `turn08`; its sources are subagent-graded and twelve of them were read only in the sections
  that bear on decomposition (the turn file lists them, and three sources that could not be
  read).
- The five earlier fronts are banked as `turn01`–`turn07`; the web scout's task-1
  hits that were read only in part are listed at the tail of `turn01` and stay unregistered.
- `examples.md` has been executed against the pinned jar (turn07, a builder lane in scratch over
  the book's own model repository): nine of twenty sections disagreed with their own claims and
  were repaired, and four `SKILL.md` claims were narrowed to what the jar showed. Residue the
  skill now states: the jar ships no complete model checker, so `1.. steps` errors; on Windows the
  jar cannot parse `util/natural`. Both rulings landed the same day: the runner honours `expect`
  (a command carrying one is red only on disagreement, and its row reports it), and the overflow
  entry names the out-of-range literal, reintroduced after a conductor run agreeing with the
  book's wrap reading.
- Leads not taken, judged low value for the skill: the unsat-core papers (a GUI feature the
  headless runner does not expose), Aluminum and provenance (not in Alloy 6), the Electrum
  papers, the `util/*.als` sources, the Alloy 4 tutorial and grammar, Software Abstractions
  (paywalled), Wayne's remaining essays.
