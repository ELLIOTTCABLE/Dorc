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

State: built on `ai/r30-alloy-praxis-conductor`, Rust only, `AlloyAdapter.java` and the wire
protocol untouched so no lock key moves. Smoked on the crate's fixture at a small heap under the
lock bypass of § 4; the completion gate on both legs is owed, held by memory, not by the lock
(§ 4). `notes/30Y` § 2.7 and § 3 describe what is.

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

- **`open-tracing-after-a-re-exec`** — the human likes `tracing` for this ("two tools
  communicating") but not builds breaking on a long run. The crate's Windows-empty dependency
  table guards against feature unification differing between `-p internal-tooling` and
  `--workspace`, which relinks and re-uplifts the binary. Measured: at `dorc-loom`'s exact
  feature set, zero differences today (~SUSPECT safe, fragile). Proposed: long-running
  subcommands re-execute from a copy of their own executable (rustup's move), which retires the
  hazard for any dependency graph; then adopt `tracing` with the subscriber's uptime timer.
  Awaiting the human.
- **`open-effort-counters`** [TYPED: "try it if it's easy and drop it if it chafes"] — sat4j's
  conflicts and restarts on the heartbeat would be the one honest enrichment; it needs the
  adapter, and any adapter change moves every lock key, so it rides the next key-moving moment.
  Feasibility inside the pinned jar is being read.
- `open-fmt-races-its-own-rebuild` (~SUSPECT, the builder's diagnosis: an hk step running
  `internal-tooling.exe` while a sibling step rebuilds it) and
  `open-respawn-parse-eats-wall-budget` (a child after a spawn or kill re-parses inside its next
  solve, against the wall cap only) are posed and unanswered.
- Whether `specs/AGENTS.md` may carry the `--quiet` line is unanswered.

## § 6-the-sibling's-needs

From the 311 mechanization conductor's list, folded here as the same vein: `--help` printing the
flags, a deferral note naming the measurement that deferred the row, and `expect` on report
rows. Not folded, for the second tune or later: a shared world module for books (the one
feature), a plain-text instance form, a `blockquote-is-headed` lint, an advisory `=` lint.
