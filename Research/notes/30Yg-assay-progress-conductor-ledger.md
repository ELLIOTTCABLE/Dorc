# 30Yg — assay progress: what a solving pass says about itself, and the official tier's cap

> Tier: LLM-authored conductor ledger (Fable conductor, round 30, the 2026-09-29 sitting that
> followed `notes/30Yf`). Subordinate to the root docs and to `notes/30Y`, which is rewritten to
> what *is* as this lane lands and wins on any detail of mechanism. Grades: **[TYPED]** the human
> typed it · **[ACKED]** confirmed in dialogue · **[CONDUCTOR]** conductor-derived, unratified.
> Claim grades where they matter: +SURE / ~SUSPECT / -GUESS.

## § 0-remit-and-state

The remit [TYPED]: two further tunes to assay, the first being more informative output when a
person runs it. The defect that opened it: an official `--write` over the 311 specification
showed one preflight line and then nothing, with no bound; the person could not tell whether it
would return in a minute or tomorrow. The rule for the build [TYPED]: "don't overengineer, start
small; MVP." The second tune arrives from the 311 mechanization conductor and is not yet here.

State: `ai/r30-alloy-praxis-conductor` carries everything that moves no lock key: the
progress lines (now `tracing` events), `--quiet`, the official cap, per-heap preflight, the
sibling's three fold-ins, the tooling profile, and the Rust half of the effort counters, which
is inert until the adapter speaks. The adapter's counters commit, the one that re-keys every
lock row, sits alone on `ai/r30-assay-effort-counters` directly above; the human is "fine with
the work that will churn the lock-format sitting in a branch for the moment" [TYPED] and lands
it when the next official run is due anyway. The human's official pass ended during the
evening of 2026-09-29 and left a schema-2 lock, keyed under the pre-counters adapter,
uncommitted in the primary checkout; its duration is the calibration for § 2's figure. The
completion gate is green on both legs at the tip that folded into `ai/main`
[TYPED: "land and fast-forward into ai/main the *non* lockchurn changes when your builder is
done"]. `notes/30Y` § 2.7 and § 3 describe what is.

## § 1-what-a-pass-says

- **`prog-loud-by-default`** [TYPED] — progress is on unless `--quiet`, "just like any tool in
  the world"; an agent or hook that pays for lines it did not want is a praxis defect, and the
  praxis (the skill's run bullet, `spike/CLAUDE.md`'s task list, the task description, hk's
  `assay-lock` step) carries `--quiet`, never the tool a terminal-detection or an environment
  switch. `--quiet` silences progress and never a finding.
- **`prog-report-not-predict`** [TYPED] — the lines are progress reports in the sense of telling
  a superior what you are doing, never a percentage; and every line is strictly true. No wall-
  clock estimate is printed [TYPED: "no estimate if it's guaranteed to be wildly off"], no
  fraction of work done; effort against the caps is the only ratio shown.
- **`prog-start-and-end-are-the-minimum`** [TYPED] — a loud pass prints at least its start and
  its end, so that a run which times out or dies leaves how long it ran; built as an elapsed
  stamp on every line and an end line on every exit path.
- **`prog-heartbeat-five-minutes`** [TYPED] — the still-alive line stays ("if only so you know
  the process didn't freeze and the terminal isn't frozen"), at about five minutes per in-flight
  command, and it does not depend on the child having started.
- **`prog-no-clock-seam`** [TYPED] — the crate reads `Instant` directly, as it already did;
  "this isn't in the actual codebase, no DST will ever apply over this".
- The adapter learns no names; a caller hands it a label. stdout stays the report alone, carried
  by one sink type (`spike/crates/internal-tooling/AGENTS.md`).

## § 2-the-official-tier-has-a-cap

- **`cap-official-eight-hours`** [TYPED that a cap is needed; the figure CONDUCTOR] — the
  official tier's batch cap defaults to eight hours, overridable by `--batch-timeout`. Before it,
  a from-scratch pass over the grown 311 specification had a ceiling near fifty hours (`laws`
  alone ~50 commands at 3600 s wall each, on one child whatever the child count).
- **`cap-not-resumable-on-purpose`** [TYPED] — the per-command ceiling is the shared,
  project-wide law that makes a timeout a design-meaningful residue, not a prompt to churn the
  Alloy until it fits; the batch cap means "stop, move on" and nothing else. A capped `--write
  --official` writes what it measured, records rows never started as `not-run` where the key
  changed or no row existed, keeps a committed definite row otherwise, exits 0, and the next
  `--check` reads those rows as owed; the route to them is a longer `--batch-timeout`. The
  default must therefore exceed what the specification actually needs, and the running official
  pass's duration is the calibration.

## § 3-preflight-sized-for-what-runs

**`preflight-follows-the-heap`** [TYPED: correct and dynamic, one shared home, no new task] —
assay already preflighted in-process; it now states its memory need from the tier's heap plus
the 512 MiB one child costs beside it, and the runner does the same for its own `--heap` before
taking the lock (its mise-level preflight step is gone). `preflight.rs` stays the single table.
The scout's finding that opened this: `preflight alloy` demanded 3 GiB while one official child
needs about 4.5 GiB, and the child count clamps to at least one.

## § 4-conduct-for-this-arc

- **`conduct-never-snipe-a-process`** [TYPED] — with a multi-hour official pass alive on the same
  machine, a builder ends only a process it started, by exact pid, never by name or pattern; it
  builds only in its own worktree. A build cannot kill that run on Windows; the run makes a
  build's uplift of `internal-tooling.exe` fail instead (`300:finding-bless-driver-self-lock-on-windows`).
- **`conduct-lock-bypass-under-measured-bounds`** [TYPED, this arc only] — the heavy-work lock may
  be bypassed through the tooling's own held route if, before each heavy step, the builder
  measures available memory, subtracts what the live run's children may still claim (their heap
  plus overhead less current use) and the step's own need, and keeps two gibibytes to spare;
  disk likewise against preflight's bound. The lock's takeover rule was read first: age is never
  consulted; only a dead or unreadable holder is taken over (`exclusive.rs`).

## § 5-posed-and-open

- **`dec-tooling-profile-then-tracing`** [TYPED: "remarkably sane and prevents this shape in
  the future. ship it."] — the human likes `tracing` here ("two tools communicating") but not
  builds breaking on a long run. The crate's Windows-empty dependency table guarded against
  cargo unifying a shared dependency's features differently under `-p internal-tooling` than
  under `--workspace`, which relinks the binary and re-copies it over a running exe. Measured
  first: at `dorc-loom`'s exact feature set, zero differences today, so ~SUSPECT safe and
  fragile by construction. Resolved cargo-natively rather than by re-executing from a copy of
  the binary (the first proposal): the tooling builds and runs under `[profile.tooling]`, so
  its exe lives in `target/tooling/` and no workspace build writes it (measured: a
  `cargo build --workspace` leaves its mtime alone; the cold profile build cost 6 s on a warm
  dependency cache and 229 MiB). Then `tracing` at `dorc-loom`'s specification replaced the
  hand-rolled sink; `--quiet` is a level filter. The manifest's dependency bar is restated:
  disk and cold-build cost, no longer feature unification. It does not close
  `finding-fmt-runs-a-jvm`: that race is between two hk steps that both run the tooling exe
  while one of them rebuilds it after a tooling-source edit, inside the tooling's own profile
  directory, and it recurred once after the profile landed (a `fmt` step could not remove
  `target\tooling\internal-tooling.exe`; the immediate re-run was green). An hk `depends`
  ordering would close it; posed, unfixed.
- **`open-effort-counters-fold-timing`** [TYPED: "try it if it's easy and drop it if it
  chafes"] — built, as the two top commits of the branch, separable from everything below:
  the adapter wraps sat4j in an adapter-local subclass of `SAT4JRef` (same id, so the
  effective-options text is unchanged) that keeps the solver it creates, and each tick carries
  its `conflicts`, `restarts`, `learned`, and `decisions` when readable; the still-alive line
  shows them once translation has ended. Read from the pinned jar's bytecode, +SURE and not
  exercised: the temporal path shares the factory. The Java commit moves every lock key, so
  when it lands is the human's; the Rust commit alone is inert. The first cut broke every parse
  through a Kodkod class-initialisation order (the ticker thread touched the factory's subclass
  before anything had touched the factory); the fix, moving the static off the subclass, is
  loud-if-wrong and untested live for want of memory.
- **`finding-gate-e2e-harness-uplift-race`** (~SUSPECT; seen once, 2026-09-29, at the tip
  that landed) — one `both` run failed on the Windows leg in hk's `test-real-tools`: a single
  e2e case (`glob-for-word-runs`) could not spawn `target\debug\dorc-harness.exe` ("The system
  cannot find the file specified") while every other case spawned it; the same leg at the same
  code was green before and after. The builder's reading of `hk.pkl`: the completion gate runs
  its twelve steps concurrently (only `gate:arc` passes `--jobs 1`), and `test-floor-suite`
  (`cargo test --workspace`, then nextest over the workspace) is the one step whose package
  selection is the whole workspace, so it builds every product binary into `target\debug`
  and, if the workspace-unified feature set differs from `-p dorc-cli`'s, re-uplifts
  `dorc-harness.exe` under the e2e tests' feet; Windows removes before it copies. The same
  hazard class the tooling profile retired for the tooling exe, on a product binary; not
  confirmed by fingerprint, not fixed, posed for the human.
- **`finding-fmt-runs-a-jvm`** (~SUSPECT the mechanism of the earlier "Access is denied") —
  `mise run fmt` is hk's fix over all files, so its `assay --staged` step parses every spec in
  a 1 GB JVM, in parallel with the cargo steps; a rebuild of `internal-tooling` racing that
  parse cannot replace the running executable. Bites whoever edits the tooling crate.
- `open-respawn-parse-eats-wall-budget` (a child after a spawn or kill re-parses inside its
  next solve, against the wall cap only) is posed and unanswered; so is whether the deferral
  note's byte-exact test stays (the builder pinned strings the conductor had specified, arguing
  the same test is the only pin on which rows defer). `specs/AGENTS.md` carries no `--quiet`
  line; the praxis lives in the skill, `spike/CLAUDE.md`, and the task description.
- Line wording is deliberately unretuned [TYPED: "as long as it's honestly reporting and
  what's-reported is mildly interesting or arguably useful, i'm letting it lay"].
- The human's rule for this arc's chat: bank builder returns, report when asked; ledgering is
  welcome under that posture ("will help untangle errors caused by my aggressive 'go away'
  posture") [TYPED].

## § 6-the-second-sitting-the-report-and-the-commit

The human's rulings of the late evening of 2026-09-29, all [TYPED], and what each became:

- **`rep-report-goes-to-a-file`** — "if that's important information that the lock doesn't
  hold, it should default to writing a temp-file … with `--json <path>` to specify an out.
  stdout should instead contain a relatively simple summary, and the path to the JSON report."
  Built for the solving modes only: `.tmp/assay/<stem>-<UTC stamp>.json` by default, `--json`
  a file or a directory or `-` for stdout, and a plain summary that is rendered from the report
  file itself so it can state only what the file holds. The non-solving modes keep stdout JSON;
  they are sub-second hook and agent surfaces. The `--write` diff moved into the summary under
  the lock's repo-relative path, which retired the noisy per-line absolute path ("fix the
  noisy full-path if you can easily"). Report files accumulate under `.tmp/assay/`, one per
  document per solving run, hk's `assay-lock` included; nothing prunes them [CONDUCTOR: left
  as is, flagged].
- **`rep-lock-records-the-commit`** — "lock should gain the git-hash it started running on (and
  fail-fast if given --write and it isn't in a quiet, committed tree), so it's clear what
  precise text it's with-reference-to." The header became `{"schema": 2, "commit": …}`, the
  report and summary carry `commit`, and `--write` refuses (exit 2) when the document or a
  shared half differs from `HEAD` in any way; "quiet, committed tree" was narrowed
  [CONDUCTOR] to the inputs the lock is with reference to, so the lock's own modification and
  unrelated files never refuse and write-look-write stays possible. A first cut missed ignored
  copies because `git status --ignored=matching` reports a directory, not its files;
  `--ignored=traditional` lists the file.
- **`rep-quiet-is-a-task`** — "ensure there's -quiet tasks for assay-related work": `mise run
  assay-quiet` carries the flag, per the repository's `-quiet` convention, and hk's
  `assay-lock` runs it; the runner has no progress to silence.
- **`rep-counters-landed-sibling-regenerates`** — "land the java piping, now that we're safe";
  the human committed the pre-counters lock (`(test new) Lock the specs for the first time`)
  before the Java landed, and ruled that the 311 mechanization conductor owns regenerating the
  lock from scratch on the new adapter, which its own substantial spec changes owe anyway. The
  Java commit landed through its own green gate on both legs; the counters branch and its
  worktree are gone. Conductor error recorded against itself: the counters worktree was removed
  in the same command as the containment test, unchained, before the fold had happened; the
  branch was intact and nothing was lost, but the order was wrong.

## § 7-the-sibling's-needs

From the 311 mechanization conductor's list, folded here as the same vein: `--help` printing the
flags, a deferral note naming the measurement that deferred the row, and `expect` on report
rows. Not folded, for the second tune or later: a shared world module for books (the one
feature), a plain-text instance form, a `blockquote-is-headed` lint, an advisory `=` lint.
