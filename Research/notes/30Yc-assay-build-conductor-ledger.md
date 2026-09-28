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
- The spec tier proper (`spec/`) does not exist yet and is not minted by this arc.
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

- **Lane 1, the first half** — one Opus builder on `ai/r30-assay` at `.tmp/trees/r30-assay`,
  Windows leg. Deliverables: the `assay` subcommand of `internal-tooling` and its `mise run
  assay` task (compile only, `.als` into `target/alloy/<stem>/`, JSON report, exit 0 or 2); the
  Dorc-agnostic fixture and its golden test; the runner's resource bounds (child heap cap,
  CPU-time cap, batch cap, processor count) and a `preflight alloy` profile; the global
  `exclusive` lock in `internal-tooling`, wrapping `mise run alloy`; strawman-3 compiled from
  its `.md` and run through the runner as the smoke, results against `report.json`; a lane
  report under the next free `30Y`-letter note ID with every fight classed A, B, or C.
  Completion: `mise run
  gate:full-quiet` on Windows. Then the fold to `ai/main` and the midpoint notification.
- **Lane 2, the second half** — a fresh builder: the lock and `--check`/`--write`, runner
  integration, the Alloy-parse lint, hk and gate placement path-filtered to spec files, the
  `exclusive` lock on the other heavy tasks, `30Y` § 3 currency, the strawman `build/`
  directories retired or left frozen, the one steering line. Brief written after lane 1 folds.

## § 4-chafe-register

**[HUMAN]** the division to keep, for every fight this build meets:

- **A. bad, Alloy** — Alloy itself is difficult or introduces unnecessary work.
- **B. bad, assay** — punted assay work that could be better, and is clearly not
  design-improvement-yielding.
- **C. good, design-chafe** — the tool's precision forcing better design work, even when it
  feels like chafe. (Expected rare in this build, since no spec is being modified.)

Entries are appended as they are met; the strawman-era fights already sorted live in
`notes/30Ya-strawman-3/FINDINGS.md` "The division" and are not repeated here.

(none yet)
