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

## § 3-lanes-and-state

To be filled at dispatch.

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
