# 30Yf — assay hot loop: the lock's semantics, the incremental key, and the tiers

> Tier: LLM-authored conductor ledger (Fable conductor, round 30; the 2026-09-28/29 sittings that
> followed the Alloy praxis work of `.claude/skills/using-alloy/`). Subordinate to the root docs and
> to `notes/30Y` (assay: what it reads, builds, runs, and reports; it is rewritten to what *is* when
> this lane lands, never before). Siblings: `plans/30Z` (how a specification is written),
> `notes/30Yc` (the assay build ledger and chafe register), `notes/312d` (the 311 mechanization arc,
> whose § 16 sitting produced the handoff this ledger supersedes). Grades: **[TYPED]** the human
> typed it · **[ACKED]** confirmed in dialogue · **[CONDUCTOR]** conductor-derived, unratified.
> Claim grades where they matter: +SURE / ~SUSPECT / -GUESS.

## § 0-remit-and-state

**Closed 2026-09-29** [TYPED: "we're done with this arc"]. The human's closing rulings, all
[TYPED]: no opaque review for this arc ("it's not relevant here"); the composition of several
specification documents is sat separately and nothing of it is ruled or recorded here; the 311
specification's lock is rebuilt from scratch under the human's oversight rather than migrated row
by row; the 311 arc's root handoff file is removed. The conductor's worktree is kept for a
rewind. What a successor needs is `notes/30Y` (what is) and `plans/30Z` § 5 to § 6 (the words and
the gates); this ledger is the record of why.

The remit [TYPED]: speed up the hot loop of writing and checking a specification without
weakening any correctness property; decide what belongs in the tooling; decide what pre-commit,
the hot loop, the gate, and CI each contain. Two loops exist and want different first levers: the
311 specification's (39 lock rows, seven laws that time out at 120 s, a full pass near 25 minutes,
every command paying ~17 s of translation for two memo-table facts) and the small-command regime
(strawman-3, freehand use of the skill), where a trivial command's 880 ms of wall time is 50 ms of
solving and the rest is JVM start, parse, and translation.

State: the lane is folded into `ai/main` (twelve commits ending at "State the batch clip, the
official tier's belt, and the cap interplay"), and so is its documentation lane, which also
repaired one defect it found (the official write replaced a definite row with an unmeasurement
on an unchanged key). `notes/30Y` is the description of record of what is; where it and this
ledger differ in a detail of mechanism, `30Y` was written from the code and wins. Measured at the
lane's report over the frozen strawman-3 (92 rows, one
red): a full `--write` went from 425 s (one JVM per command) to 126 s at the gate tier (40 rows
solved, 62 green by entailment) and 71 s at the official tier with two children; `--check` with
nothing changed 4 s; one book's world fact edited 10 s; one law edited 4 s; every verdict
identical across the old path, the new path, and every command solved singly; pre-commit on the
311 specification 2.36 s. The lane also repaired a hook case the 311 arc's `.assay.md` rename had
left red. The runner already refuses a `--command` that names nothing, honours
`expect`, keeps the expect text out of a row's `scope`, and quiets kodkod's stderr (landed
2026-09-28 from the praxis lane).

## § 1-the-correctness-properties-the-loop-must-keep

The list every lever below is measured against [CONDUCTOR; read by the human, unobjected]:

- **`prop-verdict-sound`** — a verdict is what Alloy 6 semantics at the stated scope, options, and
  module closure prescribe for that exact text.
- **`prop-verdict-reproducible`** — the same text gives the same verdict across runs, solvers,
  platforms, and time; instances may differ, verdicts may not.
- **`prop-instance-genuine`** — a shown counterexample is an instance of the current model that
  violates the claim.
- **`prop-green-composite`** — a green means something only beside its twin, the empty run, and
  the kill; no reordering, skipping, or caching may separate them.
- **`prop-lock-honest`** — "no answer changed" holds only if every row was recomputed on the
  current text or provably could not have changed; the key is the whole property.
- **`prop-attribution-first-red`** — the first red names the line.
- **`prop-timeout-is-a-result`** — caps are real; a timeout, an unrun, a deferral, a platform
  failure is reported as itself, never as green, never as another row's verdict.
- **`prop-scope-as-declared`** — the scope that produced a verdict is the one reported.
- **`prop-isolation`** — one command's solver state never touches another's verdict.
- **`prop-loop-speed-is-design-correctness`** — the kill tests, twins, and scope escalations an
  author actually runs are bounded by loop latency; speed is an input to correctness, not its
  opposite, so long as none of the above is traded for it.
- **`prop-agent-honesty`** — an agent can tell a fresh verdict from a cached or entailed one; the
  report carries provenance, the lock carries verdicts.

## § 2-the-lock-definite-results-and-unmeasurements

- **`lock-definite-versus-unmeasured`** [ACKED] — a command yields a *definite* result (`sat`,
  `unsat`, `counterexample`, `no-counterexample`), a fact about the model at that scope that no
  budget changes, or an *unmeasurement*: `timeout` (with `phase`: translating or solving),
  `platform-fail` (the jar refuses the model here; Windows and `util/natural`), `unsupported-here`
  (a `1.. steps` command with no complete checker shipped), `not-run` (the batch cap), `deferred`
  (§ 6). Each is recorded as its own kind [TYPED: platform failure separate from timeout, same
  semantics]. Every row carries `budget` (the per-command cap it was measured under, in CPU
  seconds; wall stays the harness safety), and a definite or timeout row carries `size` (primary
  variables and clauses after translation, deterministic given the jar). `premise` is derived from
  the twin row at report time, not stored twice.
- **`lock-asymmetric-match`** [ACKED] — a definite result found at any budget may be written; an
  unmeasurement never overwrites a definite result and never matches one; a `counterexample` at
  any budget against a locked `no-counterexample` is a mismatch; an unmeasurement on a *changed*
  key unlocks the row as unmeasured and never passes [TYPED: "unlocks as 'unmeasured', not
  stays-passing"]. Equal `size` across a changed key proves nothing about the verdict (an operator
  swap keeps the clause count) and is never a pass. The prior `timeout`-matches-`timeout` rule was
  unsound: a slow machine could pass a new red on exactly the seven core laws.
- **`lock-early-versus-full-timeout`** [TYPED, refined] — a timeout below the project's ceiling is
  *early* and owes exactly one thing, a run at the ceiling; a timeout at or above the ceiling is
  *full* and owes analysis (`size` and `phase` say whether it is translation-bound, the spec's
  restructuring class, or solve-bound, the solver's or a change of claim) or acceptance. "Full" is
  relative to the *current* ceiling, so a raised ceiling demotes accepted rows to early and they
  owe their run again with no special case. An accepted full timeout is the third residue kind
  beside accepted reds and unmeasured rows, listed under its own heading, and is never a green
  (`30Z:pos-halo-is-the-hazard`).
- **`lock-oom-and-error-rows`** [CONDUCTOR, ruled at the breakpoint] — out-of-memory is an
  unmeasurement of its own kind, `out-of-memory`, with the heap cap recorded beside `budget` and
  the same early-versus-full logic against the official tier's heap; a child dying otherwise is
  `error` with its message. `error` is neither definite nor an unmeasurement: a deterministic
  refusal, never skipped, never carried across a changed key, and matched as accepted residue only
  on the same key and message.
- **`lock-tier-invariant`** [ACKED] — a lower tier never makes the lock worse: it writes only
  definite results and never overwrites one with an unmeasurement; only the official tier makes
  the lock whole. Precisely [CONDUCTOR, ruled at the documentation lane's finding of a defect]: at
  `--official --write` a computed unmeasurement never replaces a committed definite row whose key
  is unchanged (the verdict is known; the row is carried); where the key changed or no row
  exists, the official tier writes the computed row, unmeasurement included, so that a full
  timeout on new text is recordable for acceptance. Hot and gate `--write` never write an
  unmeasurement. Committing the lock stays the ceremony; rows are never hand-edited to invent a
  budget.
- Prior art, for the record: Why3 sessions (each attempt stores prover, version, limits, status;
  a changed task marks the attempt obsolete; a parse error detaches rather than deletes; replay
  rewrites only when everything reproduces) is this semantics. Boogie caches timeouts and
  re-reports them without retrying; we do not.

## § 3-the-incremental-key

- **`key-composition`** [ACKED] — a row may be skipped on `--check` or `--write` only when it holds
  a definite result and its key matches. The key hashes: the raw bytes of every module in the
  command's transitive closure as Alloy actually loaded it (the `loaded` map after
  `parseEverything_fromFile`, bundled `util/*` included), with command paragraphs removed
  textually by assay, which emitted them; the command's own generated text; the row's scope
  string; the jar's digest; the adapter's source hash; and the *effective* `A4Options` as the
  adapter reports having used (symmetry, skolem depth, overflow, unrolls, decompose mode,
  partial-instance inference, core options, solver id), never a flag vector. Generated modules
  carry no comments: assay writes its line map to a sidecar, so the key is comment-blind without a
  canonicaliser [ACKED: "ack comment-strip"].
- **`key-hashes-what-the-jvm-parses`** [CONDUCTOR, ruled at the breakpoint] — the bytes hashed are
  exactly the bytes the JVM parses; a normalisation that is not emitted is never hashed, so a
  normaliser defect surfaces as a parse error or a verdict change in the emitted module rather
  than as a silent false key match. Items are emitted one per line with a single space only where
  the source had a gap and adjacent tokens kept adjacent (`this/A`, `1..3`), string literals
  verbatim, and anything the tokenizer is unsure of is a lint refusal; the sidecar maps by column.
- **`key-platform-recorded-not-keyed`** [CONDUCTOR] — verdicts are platform-independent, parse
  failures are not; the platform is a row attribute, a cross-platform hit says so, and
  `platform-fail` is its own unmeasurement.
- **`key-why-stripping-commands-is-sound`** — a command is a query; Alloy 6.2's grammar lets it
  bind no name, add no constraint another command inherits, declare no atom, open no module; the
  premise-embedding channel (a book line's check carries the lines above; the corpus run conjoins
  the unscoped checks) is covered because the key hashes the *generated* command text.
- The belt: the official tier ignores keys and recomputes from scratch. Prior art: iAlloy (Wang,
  Gligoric, Khurshid, TACAS 2019) keyed commands on checksums of their dependent paragraphs and
  disabled itself on any signature change; unmaintained, a design not a dependency.

## § 4-targeted-runs

**`run-targeted-is-a-slice-not-a-block`** [TYPED: "a vertical pyramidal slice"] — `--only
<command>` and `--module <book>` pass through assay to the runner's `--command`; a targeted run
carries what its green needs to mean anything (the target's twin, and the module's consistency
run: the book run for a book, the corpus run for the corpus book, and nothing extra for `laws.als`,
where the twin is each law's witness and a law without one is already `premise: absent`) and never
writes the lock; `--write` with a target is a usage error. Cheap-first ordering by recorded `size`; in the hot tier a
deterministic size threshold above which a row is `deferred`, listed, never green, so the hot loop
never produces a timeout by design.

## § 5-the-whole-book-conjunction

- **`book-conjunction-entails-the-lines`** [ACKED, with the dependency lifted] — one command per
  book whose body is the conjunction of the per-line check bodies *verbatim, premises included*,
  run before them. `valid(A and B)` if and only if `valid(A)` and `valid(B)`, over the same facts at
  the same scope, whatever A and B are; so a green conjunction makes every per-line row green by
  entailment, and a red one sends the per-line checks to find the first red line
  (`prop-attribution-first-red` kept). N book commands become one per green book, and the saving
  multiplies with the spec side's removal of the memo-table tax from book commands.
- **`book-conjunction-assay-invariants`** [CONDUCTOR, the must-holds this rests on, all assay's
  own and none the spec's]: the conjoined commands share one module (same facts) and one scope
  clause with the same exact bounds; a line carrying its own `for` override runs alone, outside
  the conjunction; per-line rows are written green by entailment only when the conjunction
  *completed* green, never on a timeout or error. The first-stated form, the conjunction of bare
  outcomes, was equivalent only because each per-line premise is exactly the chain of earlier
  outcomes; a hole guard or any premise beyond the chain would have turned it into N+1 solves
  silently (a cost regression, not unsoundness). The verbatim form has no such dependency.

## § 6-counterexample-replay

**`replay-evaluate-before-solving`** [TYPED: "fantastic, hard ack"] — for any row whose last
result carried an instance (a counterexample, or a sat run's witness, twins and book runs
included), evaluate the stored instance against the edited model before solving; if it still
satisfies the current command formula it is a genuine result at once; otherwise re-solve. Sound by
evaluation, under the guard below.

**`replay-guard-declarations-byte-identical`** [CONDUCTOR, from the builder's measured finding at
the lane's breakpoint] — `Command.formula` is the explicit facts and the negated claim only; it
carries no signature facts, multiplicities, abstractness, or subset constraints, and
`A4SolutionReader` enforces none of them, so an instance that violates a newly added signature fact
evaluates as a counterexample: a false red. The declaration constraints are built privately inside
the translator and are not evaluable through the API. Therefore replay only when every paragraph
of the module closure other than `fact`, `assert`, and commands is byte-identical to the closure
the instance came from, and the scope string is identical; then the declaration constraints and
bounds are unchanged, the instance satisfied them when found, and evaluating the current formula
is complete. A `pred`, `fun`, `sig`, `enum`, or `open` edit re-solves; a fact, outcome, or
world-fact edit replays. The three XML guards (same bitwidth; per-signature atom counts within the
bounds, equal for exact ones; every signature and field still declared) stay as belts. Temporal
replay is included: the builder verified that Alloy 6's instance XML round-trips a lasso trace and
that the evaluator evaluates temporal formulas over the whole read-back trace.

## § 7-hygiene-with-teeth

[ACKED as lower priority except where the 312d arc asked]: refuse an unscoped `run` not named
`<check>_premise` (today it silently becomes a corpus outcome and its meaning inverts from "exists"
to "forced"; the arc asked for this, and it is correctness); clean the out directory on compile
(asked); `--write` prints the diff it would have checked (asked); refuse two commands of one label
in one module (the lock's row identity is `(module, name)` and Alloy permits the duplicate; a
compile-time string check).

## § 8-tiers-and-pre-commit

- **Pre-commit** [TYPED: parse and key-diff belong there; path-filtered to `specs/**` so the
  1.5 s JVM rarely shares a commit set with anything else]: the solver-free compile lints; the
  parse lint through the adapter's parse verb, in a fresh child at a 1024 MB heap (there is no
  cross-invocation daemon to reuse; the human's "ideally pre-commit uses the daemon if it exists"
  is satisfied vacuously and stands as the rule should one ever exist); the key-diff against
  the committed lock over the *staged* bytes, reporting "N rows go unmeasured with this commit".
  The key-diff warns; the completion gate refuses. Nothing solver-shaped. Tune by measurement.
- **Hot** (the author's loop): targeted runs; incremental `--check` over changed keys;
  cheap-first; deferral; ~120 s CPU per command; the 540 s batch cap for the harness window.
- **Gate** (builder completion): incremental over changed keys, ~600 s per command, under the
  machine lock; the batch cap stays the 540 s that fits a foreground harness window (overridable),
  rows it does not reach recorded `not-run`, since a command that never returns is the failure the
  human named first and the official tier is where the long tail goes. Exit codes [CONDUCTOR,
  ruled at the breakpoint]: 0 every row green or accepted residue; 1 a mismatch or a new row; 2 a
  lint; 3 the runner could not run (75 for contention surfaces as 3); 4 no mismatch but rows
  unmeasured or owed, distinct from 1 because the reader's next act differs (run the official
  tier, versus fix the model); rows the hot tier defers by design do not count toward 4.
- **Official** (nightly or CI): from scratch, which means keys ignored, no replay, and no
  entailment [CONDUCTOR, ruled at the lane's report]: every per-line check is solved individually
  and the conjunction runs as a row of its own, so a disagreement between a conjunction's verdict
  and its lines' verdicts is reported as a finding against the construction, the permanent form of
  the one-time individual cross-check the lane ran. 1800 CPU s per command (the ceiling), K
  children under preflight's RAM bound with the official heap at 4096 MB; writes a candidate lock
  and prints the diff; a human commits it. CI standup (runner choice) is separate work.
- **The batch cap is a hard bound** [CONDUCTOR, ruled at the lane's report]: a command starts only
  with a wall cap of the smaller of its tier's and the batch's remaining time, and one cut by the
  clip is a `timeout` at the budget it actually got, which the early-versus-full rule reads as
  early and owed; `not-run` is for commands never started. A started command never outlives the
  batch cap. Contention exits 75 directly from assay, the repository's convention for heavy tasks
  (an earlier ruling had it surface as 3).

## § 9-architecture-rust-with-a-four-verb-java-adapter

- **`arch-mostly-rust-tiny-java`** [ACKED] — everything decidable from text and JSON is assay's
  Rust: keys, the lock and its asymmetry, tiers and budgets, ordering and deferral, the
  conjunction (generated command text), the replay policy and its fit guards, reporting. The JVM
  keeps four verbs, spoken as JSON lines over stdio: `parse` (root in; loaded closure and command
  list out), `solve` (one command under given options; verdict, instance, the effective options
  used, translation size, phase out), `eval` (model, instance XML, formula; true or false), and
  parse-only. Nothing reimplements Alloy.
- **`arch-one-java-file`** [CONDUCTOR, ruled at the breakpoint] — `mise run alloy` is
  reimplemented in Rust over the adapter, keeping its CLI, row shape, exit codes, and every
  behaviour the runner has (the fail-closed `--command`, `expect` judged by agreement, no expect
  text in `scope`, kodkod quiet, the five caps, directory arguments); the adapter replaces the
  runner, compiled once into the target directory keyed by its source hash (a precompiled class
  starts and parses a small model in about 0.2 s; the source launcher took 0.57 s, and the old path
  paid it twice per command).
- **`arch-adapter-loop-is-the-daemon`** [TYPED soft ack: not a core goal, "if it chafes, drop it",
  least work, sketched] — one adapter process per document: parse once, run its commands in
  sequence, Rust enforces every budget by killing and respawning the child (losing only the parse
  cache). No cross-invocation daemon, no socket, no `WorkerEngine` (Java-to-Java serialisation,
  unreachable from Rust). This alone removes the per-command JVM start in the small-command regime.
- **`arch-three-platforms`** [TYPED] — Windows, Linux, and macOS must be supportable; WSL is
  secondary. sat4j is pure Java and the default. No design may depend on a bundled native: minisat
  and glucose exist for Windows and Linux, the core-producing minisat and plingeling for Linux
  only, macOS and Apple Silicon unverified. A faster solver, if it ever earns its place, is an
  external DIMACS solver (kissat or cadical) pinned per platform through mise like the jar and the
  JDK, driven by a small factory in the adapter; 6.2 has no generic external factory.
- **`arch-solver-choice-is-a-measurement`** [CONDUCTOR] — no published sat4j-versus-native
  numbers on Alloy problems exist; the translation phase can vastly exceed solving (Aluminum's
  authors; our own rows). The `phase` column decides per row: translation-bound timeouts are the
  spec's restructuring or the warm JVM's to fix, solve-bound ones the solver's. Nothing to build
  until `phase` is recorded.

## § 10-temporal-mode

What carries unchanged [CONDUCTOR]: the lock semantics, the key (the scope string carries
`steps`), cheap-first, deferral, and the conjunction (validity over a trace set is a conjunction of
validities at the same step bound). Two verifications owed before replay covers temporal commands,
both -GUESS today: that Alloy 6's instance XML round-trips a lasso trace through
`A4SolutionReader`, and that the evaluator then evaluates a temporal formula over the read-back
trace rather than its first state. `for 1.. steps` lands as `unsupported-here`, never as a timeout
or an error.

## § 11-not-now-and-not-ever

Excluded as risky or not obviously sound [ACKED by the owed-change list]: scope escalation (exact
scopes and `util/ordering` break monotonicity); any size-equality pass; sharing translation between
a check and its twin (a check is its twin's problem plus one conjunct, exactly Kodkod's incremental
monotone case, but Alloy exposes no path to it); partial-instance inference as a *choice*: the
lane found it is the API's default and on for every verdict ever recorded, the 311 lock included,
so it stays on as shipped, the key records it, and turning it off would re-key every row
[CONDUCTOR, from the lane's finding]; Platinum-style slice reuse of unsat
results (a 2020 prototype with a lossy canonicalisation). A deterministic budget unit (a SAT
conflict limit, in the manner of Lean's heartbeats, making "unmeasured at K conflicts" machine
independent) is a later refinement of `budget`, not owed. Spec-side restructurings (the memo-table
facts as a premise for only the commands that read them; the walk's cost restructurings) belong to
the 311 arc under `check { old iff new }`, never to a builder.

## § 12-coordination-with-the-311-arc

The lane and the arc run in parallel in the one repository. The lane never touches `specs/`
(`specs/AGENTS.md`), so the arc's lock migrates by schema version: old rows read as budget-unknown
and size-unknown and are re-measured at their first official pass, never hand-edited [handoff § 7,
ACKED]. Both sides touch `assay.rs`, `hk.pkl`, and `mise.toml`; the lane keeps new logic in new
modules where it can and rebases onto `ai/main` often. The root `_tmp-assay-hot-loop-handoff.md`
is the arc's and is spent once this ledger and the lane exist; its owner deletes it. Findings that
belong to the arc (the key found unsound, a lock row that moves under a schema migration) go to
the human, who decides what reaches `notes/312d`.

## § 13-landing-checklist

When the lane lands, a documentation lane (a fresh builder briefed with the lane's report): `notes/30Y`
rewritten to what is (the lock's kinds and columns, the key, the tiers, the adapter and its verbs,
the platform rule, the four per-module orderings and the deferral threshold as set), never as a
build-against-future;
`LIVING_STATUS` current; `.claude/skills/using-alloy/SKILL.md`'s how-to-run bullet updated if the
invocation or the red rule changed; the chafe register in `notes/30Yc` current; this ledger
compressed to what a reader of `30Y` still needs.
