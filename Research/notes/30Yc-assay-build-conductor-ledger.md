# 30Yc — the assay build: conductor ledger

> Tier: LLM-authored conductor ledger (Fable, round 30, opened 2026-09-28). The plan it builds is
> `notes/30Y`; the design ledger behind the plan is `notes/30Ya`; the runner lane that preceded it
> is `notes/30Yb`. Compression-resistant: state, rulings, and the chafe register, never a
> play-by-play. Grades: **[HUMAN]** typed by the human · +SURE / ~SUSPECT / -GUESS on the
> conductor's own claims.

## § 1-remit

**[HUMAN]** 2026-09-28: build the tool `notes/30Y` describes, as an MVP, so the process can be
tested against the in-flight 311 work. Primary remit is narrowness and simplicity: keep the
builder reined in, no overengineering, no productizing. Windows leg only for this stretch (the
platform the runner lane used). No builder parallelism where both builders would run Alloy.
Either one small arc, or a split with a merge point at which a mildly functioning MVP reaches
`ai/main` and the human is notified, so design conductors can use it while the rest lands.

Already built before this ledger opened (`notes/30Yb`): the pinned JDK and Alloy 6.2 jar in
`mise.toml`; `spike/verify/alloy/AlloyRunner.java` behind `mise run alloy`, one child JVM per
command, JSON rows in the lock's shape, `--open` overlay, `--command`, `--instances`,
`--solver`, `--timeout`. Both strawmen have been run for real and their findings banked
(`notes/30Ya-strawman-3/FINDINGS.md` "The division"; `30Ya` "The pre-step").

What remains, per `30Y` § 3: the compiler (Markdown in, one flat directory of Alloy modules
out), the lock and report with their exit codes, the four lints, gate placement, and the
fixtures wired so that assay's own output replaces the hand-written `report.json` files.

## § 2-standing-rulings-for-this-build

- **[HUMAN]** siting of build products, 2026-09-28: one home for committed generated products
  (lean: flat, beside the spec file, a distinguishing extension); a different home for
  temporary products (`.tmp/` or, if it does not disturb Rust, `target/`); nack on `spec/build/`
  or any new build directory; no fourth or fifth convention, share Rust's or Lean's.
- The strawman fixtures under `notes/30Ya-strawman-{2,3}/` are non-normative and stay where
  they are (`30Y` § 3 [TYPED nack] on promotion). Their `spec/*.md` are the compiler's inputs and
  are read-only to the builder except for mechanical fence-form repairs needed to make them
  well-formed under `30Y` § 2.1, each reported as chafe (§ 4). No semantic edit to any Alloy
  content in them: that is a stop-and-report.
- The spec tier proper (`specs/`) does not exist yet and is not minted by this arc.
- **[HUMAN]** 2026-09-28, the dispatch rulings: temporary generated products go under
  `target/alloy/<spec-stem>/` (the human floated `target/alloy/` in case assay is one of a
  stable; the conductor's lean was taken); the lock, when it exists, is the only committed
  generated product and sits beside its spec; no generated `.als` is committed. Line-mapping is
  a culture to keep: the tooling should eventually paper over line-number and location
  inconsistencies the transpilation causes; not immediate unless trivial. Leaks are avoided:
  authors and LLMs must be able to bypass assay and invoke Alloy directly; no unnecessary
  wrappers or indirection. The strawmen are not durables: free to churn as experimental
  targets, frozen once this arc ends; assay's own self-test content is built, Dorc-agnostic
  and meaningless, in the project's test convention, never borrowed from the strawmen (whose
  content about Dorc is apparently normative and incorrect). Ceremony minimal; build upfront
  only what the midpoint needs.
- **[HUMAN]** the midpoint is tuned back to two things a competent conductor can glue by hand
  for the first rounds of authorship: an invocation of Alloy itself (`mise run alloy`, built),
  and a compiler command that emits `.als` files into `target/`. Lock, lock checking, runner
  integration, hk, gates, and any fancier runner are the second half.
- **[HUMAN]** resource-exhaustion tooling is first-half work: reuse and enrich `preflight`
  (several disk-exhaustion events this week); add CPU and wall-clock bounds to the solver's
  safety tooling, janky to start (a SAT command that never returns puts an LLM to sleep for
  hours; it happened during the runner lane); a global, not project-local, lock on a
  standardized file so parallel builders and worktrees cannot run very expensive work
  concurrently, which DENIES and tells the caller to work on something else, never spin-waits.
  The WSL RAM reading may be unreliable; out of scope unless hit.
- **[HUMAN]** the `both` gate is held for this arc: Windows leg only. WSL is acting up and
  `both` freezes the machine, low value while no project Rust the WSL leg would see differently
  changes; it becomes necessary in the second half, when the gates themselves are worked on.
- **[HUMAN]** two Opus builders, serial; a fresh conductor context between the halves.
- Self-tests, the conductor's decision under the human's gentle lean: a minimal Dorc-agnostic
  fixture spec with a golden over its generated modules, built first, then the compiler built
  against it; strawman-3 is the run-for-real smoke, not a committed test. Taken because it is
  short in wall-clock and is what the builder develops against anyway.
- Corrections applied to `30Y` at dispatch (it is a living plan and reads current): wall-clock
  and translation size are report columns, never lock columns, since the lock is compared in
  both directions; words are minted from `#=` lines AND from claim-atom bodies, which strawman-3
  needs (`pkg_index` and kin appear only in claims); minting words needs assay to recognise
  declaration heads and quantifier binders, more than "a sig's name and parent"; the corpus
  book's null command has an assay-minted word; the runner carries the resource bounds above.

## § 3-lanes-and-state

- **Lane 1, the first half — CLOSED 2026-09-28, folded to `ai/main` by fast-forward** at the
  builder's tip `(AI dsn re) Record the corpus inhabitation run, the reversal, and the three
  requested fixes`; its worktree and branch removed. As built: `notes/30Yd`. The midpoint the
  human asked for holds: `mise run assay -- <spec.md>` compiles a document into
  `target/alloy/<stem>/`, and `mise run alloy -- --timeout <s> <that directory>` runs it, bounded,
  under `preflight alloy` and the heavy-work lock. The Dorc-agnostic fixture and its golden test
  are the crate's; the strawman-3 smoke matched every verdict of the hand build; strawman-2
  refuses under the identifier rule and stays frozen.
- Adjudication of the lane's eleven disclosed deviations, re-derived from the global picture:
  ten endorsed, one reversed. Endorsed with the conductor's own mistake named: the out-of-remit
  clippy fix (the baseline was red from the runner lane and the brief never had the builder
  check it); the two extra lints (the brief's rules required them and its lint list omitted
  them); stale-module removal (recompile semantics were unspecified); the document-level
  `bookScope` tier (additive, and what the human's "shareable across spec files" wanted; `30Y`
  § 2.4 now carries it); scoped non-premise runs to `laws.als`; any sig head as a `#=`
  declaration; the smoke split into three runs under the harness ceiling; expected files carrying
  only the generated header; a liveness check that cannot run refusing rather than admitting a
  second solver; preflight figures derived rather than measured. The `tests` commit label was the
  brief's error, copied from a generic table when `.gitlabels` says `test`. Reversed:
  `dev-no-corpus-run`; an all-claims-in-force universe can be unsatisfiable and would green every
  corpus check vacuously, so the corpus book owes the inhabitation run every book owes; built in
  the follow-up, sat on strawman-3.
- **Lane 2, the second half — NOT YET BRIEFED; a fresh conductor context [HUMAN].** Its remit,
  for the successor: the lock and `--check`/`--write` (`30Y` § 2.7, timing out of the lock);
  runner integration from assay, kept thin so an author can still invoke Alloy directly
  [HUMAN: no leaky wrappers]; the Alloy-parse lint (`30Y` § 2.6, fourth item); hk pre-commit
  lints and `gate:full-quiet` lock recomputation, both path-filtered to spec files; the
  `exclusive` lock on the other heavy tasks (`gate:full*`, `bless*`, `verify:kani`,
  `verify:lean`), which changes every lane's completion contract and so wants the human's typed
  ack before it lands; the `both` gate returns for this lane [HUMAN: it is necessary once gates
  are worked on]; `30Y` § 3 kept current; `plans/30Z` § 6 written once the lock and gates rule
  what an author owes and when (this ledger's § 5 holds the current thinking). The strawman
  `build/` directories stay as they are: frozen artifacts, never fixtures. Minting `specs/` and
  translating 311 are NOT lane 2; that is the human's clean-context design work, which lane 1's
  midpoint exists to serve.
- Open for the human, surfaced 2026-09-28 (`30Yd:disagree-braced-only-literals`): a literal that
  is only ever braced on map lines has no identifier for its atom. Built as a refusal
  (`braced-literal-is-named`). `30Y` § 2.1 says any component may be either. Refuse, or munge?
  Belongs in `plans/30Z` § 5 once ruled.

## § 4-chafe-register

**[HUMAN]** the division to keep, for every fight this build meets:

- **A. bad, Alloy** — Alloy itself is difficult or introduces unnecessary work.
- **B. bad, assay** — punted assay work that could be better, and is clearly not
  design-improvement-yielding.
- **C. good, design-chafe** — the tool's precision forcing better design work, even when it
  feels like chafe. (Expected rare in this build, since no spec is being modified.)

Entries are appended as they are met; the strawman-era fights already sorted live in
`notes/30Ya-strawman-3/FINDINGS.md` "The division" and are not repeated here. Lane 1's fights
are `notes/30Yd` § 4 in the builder's words; the conductor's sort:

- **A** `30Yd:chafe-alloy-adds-a-default-command` — Alloy synthesizes a `Default` run for a
  command-less module. Absorbed: the runner skips a module whose only command has Alloy's
  unknown source position. Stays A: an Alloy quirk, not a design fact.
- **B** `30Yd:chafe-powershell-does-not-expand-the-book-glob` — absorbed: the runner takes a
  directory. `30Yd:chafe-foreground-ceiling-below-the-batch-cap` — absorbed: defaults 120 and
  540 seconds plus a shutdown hook. `30Yd:chafe-typos-fix-mode-rewrites-identifiers` — a real
  tooling hazard (fix mode rewrote a string literal and changed behaviour silently); absorbed:
  the bare word is ignored in both typos configs. `30Yd:chafe-main-was-clippy-red` — absorbed
  in lane; the conductor's scouting gap. `30Yd:chafe-lexer-is-private` — fine as built: words
  come from the real parser's spans, and a redirection is already one span.
  `30Yd:chafe-tests-label-not-in-gitlabels` — the brief's error; briefs cite `.gitlabels`, not
  the generic table.
- **C** none in this lane: no specification content changed. The one design-adjacent item is
  the braced-only literal question in § 3, which is a spec-form ruling, not chafe.
- Not chafe, corrections to `30Y` made at the fold: `30Yd:disagree-literal-components-in-maps`
  (§ 2.8's example now names its components) and `30Yd:disagree-load-displacement` (§ 2.4 now
  says union at v0). `30Yd:disagree-harness-claim-abstract` needs nothing: `30Y` is right and
  strawman-3's hand harness is the stale one, frozen.

## § 5-what-belongs-in-30Z-section-6

**[HUMAN]** 2026-09-28: `plans/30Z` § 6 (results, locks, what an author owes) stays a TODO until
the lock and gates are built and the ruling of what an author owes is made; the conductor notes
here what it thinks belongs there. All [CONDUCTOR]:

- The lock as the ratchet: computed versus committed, a mismatch in either direction fails, and
  it moves only by a human commit; timing and translation size are report columns, never lock
  columns (`30Y` § 2.7).
- Stored verdicts are intent, qualified in goblint's manner where an instrument disagrees:
  `UNKNOWN!` an unsoundness, `UNKNOWN` intended imprecision, `TODO` precision owed; never
  regenerated from an implementation's output, which is characterisation, not specification.
- An expected red is the whole-law TODO, honest and coarse (§ 2.6 of `30Z` has the fine-grained
  form); the two are recorded differently and the report says which.
- Disagreement across instruments (design tier, field tier, later Kani or Lean) is triaged five
  ways, implementation, book, record, model, prose, and named in the failure output; never
  auto-resolved.
- The mutation lane as the mechanical form of the kill-by-hand habit: drop a fence line, re-run,
  a law or book must go red; a mutant nothing kills is reported.
- The standard of done for a document: every append-half law green with a witnessed premise;
  every book's run sat; no check `premise: absent`; the hole count and the count of
  `<!-- normative -->` sections reported as the residue.
- What an author owes and when: every edit runs the document's corpus before the claim is shown
  to a human; a moved result is a finding or a human-committed retirement, never silent; holes
  only decrease between adjudications; a reword of prose or Alloy leaves the lock unmoved.

## § 6-the-second-half-dispatch (2026-09-28; human-typed rulings, then the lane)

- **[HUMAN]** proceed with the second builder now, in this conductor context after all, on the
  same worktree name (recreated, since lane 1's was removed at the fold).
- **[HUMAN]** the braced-only literal question is dissolved, and the refusal was wrong: a word
  defaults to *itself, literally*, under a deterministic, injective munge into Alloy's identifier
  alphabet, exactly as any other instance; most class words will never appear elsewhere as
  literals, so their name is beside the point, and no ceremony may force one to be minted.
  Consequence taken by the conductor: the same rule dissolves `component-is-an-identifier` for a
  bare component that repeats its literal (strawman-2's `-c`, `g-w`), which now names itself; a
  bare component that is neither an identifier nor the literal is the one remaining refusal.
  `30Y` § 2.1 and § 2.6 and `30Z` § 5 now say so; lane 2 builds it.
- **[HUMAN]** the translation block is optional but strongly recommended, and every change to a
  section's fences must include a change to its translation prose (adding one where none exists);
  it cannot prove a match, it fights drift. `30Z` 1.2-the-firewall carries both. Section references
  inside a document use the full section slug (`2.4-inhabit-before-you-believe-green`) except on
  repetition within one paragraph; `30Z` rewritten to that convention.
- **[HUMAN]** enrichment, not pressing: running a minimal set during the hot loop, one section or
  book by slug, when a document is large and slow. Already available at module and command
  granularity (one book is one module; `--command` names one law); noted in `30Z` § 5 and the
  handoff; an `--only <book>` filter on the compiler is a later nicety.
- A gitignored root handoff, `_tmp-assay-handoff.md`, tells the sibling conductor who will
  concretize 311 how to use the midpoint tooling, the hot loop, the form, the limits until lane 2
  folds, and where to record chafe.
- **Lane 2 — DISPATCHED 2026-09-28**, one Opus builder on `ai/r30-assay` at `.tmp/trees/r30-assay`.
  Remit: self-naming words with the injective munge and the collision refusal; the lock and
  `--check`/`--write` beside the spec, timing kept out of it, the runner invoked as a subprocess
  with caps passed through; the Alloy-parse lint on `--check`/`--write` and on demand; the
  translation co-change check over staged documents; hk pre-commit lints and the
  `gate:full-quiet` lock recomputation, path-filtered to `specs/**`, ready before the tier exists;
  the fixture extended for self-named and braced-only words and for the lock; a lane report at the
  next free `30Y` letter with the A/B/C division. Held for the human's typed ack, not in the brief:
  wrapping the other heavy gates in `exclusive`. Completion: `mise run gate:full-quiet` on Windows
  through the hot loop, `mise run both gate:full-quiet` once at the end [HUMAN: necessary once the
  gates are worked on]. The sibling conductor may hold the heavy-work lock at any time; the builder
  treats exit 75 as "do other work".
- **[HUMAN]** later the same day, three corrections applied to the brief by message: the
  translation co-change rule is LAW, NOT CHECKED (no sound mechanisation that is not fragile); a
  fence-only change commits only on an active, direct human ack for that commit; the drift-check
  deliverable is withdrawn from lane 2 and `30Z` 1.2-the-firewall says so. Wrapping the other heavy
  tasks in `exclusive` is ACKED: it changes no contract, the same final state is owed, it is merely
  enforced serial rather than concurrent; added to lane 2, with re-entrancy for nested tasks
  (`gate:arc` runs completion inside itself). And the conductor's remaining refusal ("a bare
  component that is neither an identifier nor the literal") was ceremony too: every bare component
  is a name for the literal beneath it, munged when it is not an identifier, self-named when equal;
  every word trivially has a name; the only refusal is one name over two literals or one literal
  under two names. `30Y` § 2.1, § 2.6 and `30Z` § 5 corrected.

## § 7-the-second-half-close (2026-09-28)

- **Lane 2 — CLOSED**, folded to `ai/main` by rebase onto the moved tip (only the generated slug
  index overlapped) and fast-forward at `(AI dsn new) Report the lock-and-gates lane as built`;
  worktree and branch removed. As built: `notes/30Ye`. Both gates green on both legs, run through
  the wrapped tasks. Strawman-2 compiles (zero refusals, six literals self-named); its frozen
  shared half still spells the pre-rename harness field `before`, so its laws error under Alloy
  and stay unrecorded; frozen, no action.
- Adjudication of the thirteen disclosed deviations: twelve endorsed, none reversed, one held for
  the human (`30Ye:dev-exclusive-wrapping-partly-denied`). Conductor mistakes named: the brief
  never said what hk hands a step (`{{files}}`, several documents, halves and locks standing for
  their documents); the `test` task's one-filter limit was unknown to the brief; the hidden
  `<task>:held` twin was the only shell-free way to wrap a multi-step task, and its visible bypass
  is accepted as such. The hand-rolled JSON module (`json.rs`, now shared by the lock and the lock
  reader) was a lane-1 choice made without asking about a dependency; it works and is small; a
  review point, not a defect.
- Chafe, sorted. **A**: `30Ye:chafe-scope-error-surfaces-at-solve` (a missing `bookScope` parses
  clean and errors only at solve, so the parse lint cannot see it; an `error` row under `--check`
  is where it lands) · `30Ye:chafe-scope-renders-int-lowercase` (cosmetic). **B**:
  `30Ye:chafe-main-slug-index-stale`, whose root cause is the conductor's: the `slugs` check reads
  the WORKING TREE, so the regeneration at the lane-2 dispatch indexed the sibling conductor's
  uncommitted edits and every other worktree then refused any Markdown commit; the tooling should
  index committed or staged content, a fix the human schedules ·
  `30Ye:chafe-test-task-takes-one-filter` (`mise run test -- a b` dies in the doctest half) ·
  `30Ye:chafe-too-many-lines` · `30Ye:chafe-out-of-repo-paths-verbatim`. **C**: none; no
  specification content changed in either lane.
- Open for the human, in order of weight: (1) the seven heavy tasks the classifier left unwrapped
  (`bless`, `bless:dry`, `bless:case`, `bless:floor`, `verify:kani`, `verify:lean`,
  `verify:translate`), theirs to apply or authorise, following the three `:held` twins that
  landed. (2) `30Ye:disagree-locked-red-still-exits-one`: `--check` exits 1 on any red row even
  when the committed lock records it red, so a document carrying an expected red can never pass
  the `assay-lock` step, against `30Z` 2.6-hold-a-question-open's recorded-red form. Conductor's
  recommendation: `--check` exits 0 iff computed matches committed, reds included, and the report
  lists every red as residue; a NEW red is a mismatch and exits 1; committing the lock is the
  ceremony that accepts a red. (3) `30Ye:disagree-book-checks-premise-absent`: a book line's
  witness is its book's run; the lock records `absent` today and should record the run's result;
  `30Y` § 2.5 now says so; a small follow-up when assay is next touched.
- Nothing else builds in this arc. `30Z` § 6 is written once (2) is ruled and the lock's gate
  behaviour is what the section describes.

### § 7.1-the-third-round (2026-09-28, same lane, same builder)

- **[HUMAN]** rulings: a set of reds fully acked by the committed lock is a PASS; the seven denied
  wrappings are applied under the human's manual approval, first thing; names need no further
  injectivity work, since a spec author has the `#}` line to write a readable name; the passing
  slug-index incoherence between sibling commits is accepted, fix optional.
- Built and folded by fast-forward at `(AI dsn re) Report the wrapped lanes, accepted reds, and
  book witnesses`: the seven wrappings landed in one edit with no denial (`bless`, `bless:dry`,
  `bless:case`, `bless:floor`, `verify:kani`, `verify:lean`, `verify:translate`; `bless:dry` ran
  under the lock in 177 seconds and released it); `--check` exits 0 iff the run matches the lock
  row for row, reds included, and the report lists `accepted_reds`; a book line's and the corpus
  book's checks take their premise from the book's run. `30Y` § 2.7 now states the ruling. Items
  (1), (2), and (3) of § 7 are closed.
- Deviations, both endorsed: on Windows the three `verify:*` tasks hand off to WSL and the
  hand-off itself takes no Windows lock, the WSL run taking WSL's own (the heavy work is where the
  lock is); `bless:case`'s environment moved onto its hidden twin with the body it applies to. One
  praxis note: the builder folded a non-compiling intermediate commit into its successor by soft
  reset rather than committing it under `DORC_KNOWN_BROKEN`; allowed surgery, and the honest
  history the commit skill prefers would have kept it.
- Remaining in this arc: `plans/30Z` § 6, which now waits only on the ruling of what an author
  owes and when (§ 5 of this ledger is the conductor's draft of it).
- **[HUMAN]** corrections on the tier, same day: it is `specs/`, with the s; a docID names ONE
  document in the corpus, so `notes/311` MOVES to `specs/311` when mechanised, it is not copied
  and nothing outranks anything; `Research/notes/` → `Research/plans/` → `specs/` are tiers of
  different purpose, not a promotion ladder; the docID space is shared with the quarantine, which
  occasionally holds one. Applied: the hk globs and the wiring test, `30Y`, this ledger,
  `LIVING_STATUS`, the map, the handoff. The conductor's earlier "a specification outranks the
  note it was translated from" was a misreading and was never written into a durable.
- Steering minted at the human's direction, in the human's own terse style, five bullets each at
  most: `spike/crates/internal-tooling/AGENTS.md` (assay's code invariants: agnosticism, heads and
  binders only, no JVM in tests, results-only lock with reds-as-pass, re-entrant lock and visible
  bypass) and `specs/AGENTS.md` (the firewall in one line, builders never edit, holes not facts,
  what commits and completion run, the shared halves); each with an `@AGENTS.md` pointer
  `CLAUDE.md`. The `verified-core-discipline` skill gains assay as the design-tier instrument.
