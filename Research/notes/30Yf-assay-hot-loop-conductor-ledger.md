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

The remit [TYPED]: speed up the hot loop of writing and checking a specification without
weakening any correctness property; decide what belongs in the tooling; decide what pre-commit,
the hot loop, the gate, and CI each contain. Two loops exist and want different first levers: the
311 specification's (39 lock rows, seven laws that time out at 120 s, a full pass near 25 minutes,
every command paying ~17 s of translation for two memo-table facts) and the small-command regime
(strawman-3, freehand use of the skill), where a trivial command's 880 ms of wall time is 50 ms of
solving and the rest is JVM start, parse, and translation.

State: the lane `ai/r30-assay-hot-loop` is the build; it runs in parallel with the 311
mechanization arc, which keeps using `--parse`, `--check`, `--write`, and `mise run alloy` as they
are until the lane lands. The runner already refuses a `--command` that names nothing, honours
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
- **`lock-tier-invariant`** [ACKED] — a lower tier never makes the lock worse: it writes only
  definite results and never overwrites one with an unmeasurement; only the official tier makes
  the lock whole. Committing the lock stays the ceremony; rows are never hand-edited to invent a
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
run) and never writes the lock. Cheap-first ordering by recorded `size`; in the hot tier a
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

**`replay-evaluate-before-solving`** [TYPED: "fantastic, hard ack"] — for a red row, evaluate the
stored counterexample against the edited model before solving; if it still satisfies the facts and
violates the claim it is red at once with a genuine instance; otherwise re-solve. Sound by
evaluation. Fit guards [CONDUCTOR, from iAlloy's stated gap]: same bitwidth; per-signature atom
counts within the command's bounds, equal for exact bounds; every signature and field in the
instance still declared; otherwise re-solve. Static commands first; temporal commands after § 9's
two verifications. The mechanism is in-tree (`A4SolutionReader` against a fresh parse, then eval),
which the LSP already does.

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
  parse lint through the adapter's parse verb, which uses a live child when one exists and spawns
  one when not [TYPED: "ideally pre-commit uses the daemon if it exists"]; the key-diff against
  the committed lock over the *staged* bytes, reporting "N rows go unmeasured with this commit".
  The key-diff warns; the completion gate refuses. Nothing solver-shaped. Tune by measurement.
- **Hot** (the author's loop): targeted runs; incremental `--check` over changed keys;
  cheap-first; deferral; ~120 s CPU per command; the 540 s batch cap for the harness window.
- **Gate** (builder completion): incremental over changed keys, ~600 s per command, a batch cap
  derived from rows-to-run, under the machine lock; rows over the cap recorded `timeout` with
  their budget; exit under the asymmetric rules.
- **Official** (nightly or CI): from scratch, keys ignored, 900 to 3600 s per command, K children
  under preflight's RAM bound; writes a candidate lock and prints the diff; a human commits it. CI
  standup (runner choice) is separate work.

## § 9-architecture-rust-with-a-four-verb-java-adapter

- **`arch-mostly-rust-tiny-java`** [ACKED] — everything decidable from text and JSON is assay's
  Rust: keys, the lock and its asymmetry, tiers and budgets, ordering and deferral, the
  conjunction (generated command text), the replay policy and its fit guards, reporting. The JVM
  keeps four verbs, spoken as JSON lines over stdio: `parse` (root in; loaded closure and command
  list out), `solve` (one command under given options; verdict, instance, the effective options
  used, translation size, phase out), `eval` (model, instance XML, formula; true or false), and
  parse-only. Nothing reimplements Alloy.
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
monotone case, but Alloy exposes no path to it); partial-instance inference (an experimental
option; only after a differential run over the strawmen); Platinum-style slice reuse of unsat
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

When the lane lands: `notes/30Y` rewritten to what is (the lock's kinds and columns, the key, the
tiers, the adapter and its verbs, the platform rule), never as a build-against-future;
`LIVING_STATUS` current; `.claude/skills/using-alloy/SKILL.md`'s how-to-run bullet updated if the
invocation or the red rule changed; the chafe register in `notes/30Yc` current; this ledger
compressed to what a reader of `30Y` still needs.
