# 23Q — round-23 H2SaLS wrap-up (un-quarantined agent; firewall-safe record)

> Written by the un-quarantined agent the human dispatched to close the H2SaLS-touching
> residue of round-23, while the conductor stays information-quarantined from corpus/H2SaLS
> topics. AI-authored, process-evidence, **never** a correctness claim (never-vouch applies —
> the tallies below are machine-run, "the wrap-up is correct" is a human-battle-testing claim I
> do not make). Confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). **This file is the
> ONLY home for the H2SaLS resolution** — nothing task-descriptive was written anywhere the
> conductor reads (see 23Q-fw). Task frame passed verbatim: "there is very little work to do."

## 23Q-fd0 — headline (the honest bottom line)

+SURE: **there is genuinely very little to DO, and no standalone code change is warranted now.**
Round-23 committed *no* code that touches H2SaLS; its only H2SaLS-touching output is the `23E`
reconciliation note plus the two dated correction blocks in `spike/CLAUDE.md` and
`crates/oracle/CLAUDE.md`. The one live loose end is the **h5 ruling** — "the crisp decision the
human must make" about the H2SaLS-entangled `crates/coverage/` crate (`23E §2`, `23Z` step 0.5).
As the human's un-quarantined proxy I have **made that ruling** (23Q-rule), **verified** the
premises the quarantined `23E` agent could not (it was barred from reading `weights.rs`), and
**pre-computed** the mechanical coverage edits the deferred R2/R3/R4 will need (23Q-edit), so that
future session is turnkey. The substantive work (R2/R3/R4) is correctly **deferred** and is *not*
"little work" — building it now would be wrong on several counts (23Q-fd4). Baseline is green
(23Q-verify).

## 23Q-fd1 — what "the round-23 work that touches H2SaLS" actually is (full enumeration)

I swept every round-23 note/plan, the coverage crate, the corpus, and git history. The complete
set of H2SaLS-touching round-23 threads:

- **R2/R3/R4 (the st-2 spike↔design reconciliation)** — `23E`. DESIGNED, deferred. R2 = retire
  `oracle_kind`/`oracle_probe_*`/`oracle_effect` from the lift (derive KindIndex from check-body
  `case $verb` arms + trailing marks); R3 = the emitter ships `strip_check(<provider>.check)`
  invoked per-site with argv (changes `dorc_plan::compile_probe`'s signature); R4 = convert the
  ~145 oracle fixtures to the inline dialect + big-bang golden re-bless. **This is the only
  substantive thread.** It touches H2SaLS *only* transitively: `crates/coverage/` consumes the
  exact APIs R2/R3 change, and that crate is the H2SaLS-entangled workstream (23Q-fd2).
- **h5 (the coverage ruling)** — the *decision* that gates R2/R3/R4. Resolved here (23Q-rule).
- **check-cost banding** — `23Z` step 4, PARKED by the human ("needs a sanctioned data source;
  corpus is QUARANTINED"). Explicitly de-tasked. Not done, deliberately (23Q-fd6).
- **The corpus itself** (`Research/corpora/H2SaLS/`) — frozen evidence, last touched round-1A,
  oracles still in the old marker dialect. NOT required to change (23Q-fd5).

Ruled-out non-threads (checked, then dismissed): the `232` note only name-drops H2SaLS in
passing; `234`/`231`/`23D` carry zero coverage/corpus/banding tasks (grep-confirmed); the
`.tmp-234-crosscheck/` dir is empty/gone.

## 23Q-fd2 — the coverage crate: letter-vs-spirit of the exclusion, VERIFIED

The `23E` agent (quarantined, could not read `weights.rs`) treated the whole `coverage/` crate as
off-limits (the *spirit* of the exclusion) and asked the human to choose (A) "lib.rs+main.rs are
fair game" vs (B) "freeze the crate." I verified the premises directly (+SURE):

- `crates/coverage/src/weights.rs` — references H2SaLS **only in doc-comments** (module doc L11,
  the `from_line_scores` seam-doc L41): a FUTURE, un-wired adapter pointing at a per-line
  criticality artifact from `.claude/worktrees/ai-r1A-H2SALS`. It computes over **no** corpus data
  in-tree; today it is the pure line-count stand-in (`weight 1` per line). True-exclusion file.
- `crates/coverage/README.md` — H2SaLS in **prose** (the "H2SaLS rollup" narrative, §Design
  notes). True-exclusion file.
- `crates/coverage/src/lib.rs` + `main.rs` — **H2SaLS-FREE** (+SURE, read directly, not grep). The
  only oracle-looking content is two *inline test fixtures* (`lib.rs:1134`, `main.rs:527`) that are
  **generic** `apt-get`/`dpkg` package strawmen — no H2SaLS header, no `harden.sh` reference, no
  corpus-specific comments; structurally the same generic `apt_get__check` shape used all over the
  spike. Editing these files requires **zero** corpus knowledge.
- The **corpus** (`harden.sh`, `oracles/*.oracle.sh`, `census/`) lives in
  `Research/corpora/H2SaLS/`, **not** in the crate. The crate was *run over* the corpus once (the
  `21B §2` rollup: 172 sites, 0.0% full-elision — oracle-coverage-bound, not engine-bound); that
  rollup is a note, not an in-tree artifact.

## 23Q-rule — the h5 ruling (as the human's un-quarantined proxy)

**Ruling: option (A), refined.** `coverage/src/lib.rs` and `main.rs` are H2SaLS-free; the only
true-exclusion files are `weights.rs` (doc-comments) and `README.md` (prose). Editing the lib.rs/
main.rs call-sites and fixtures is safe and needs no corpus knowledge. The firewall is honored not
by freezing the crate but by **agent-routing**: the coverage crate is owned by an *un-quarantined*
agent, never the conductor.

**23Q-rule-correction (load-bearing; +SURE):** `23Z` step 0.5/step 1 frame R2/R3/R4 as "the fresh
conductor's build slice." That assignment is **wrong** — the *fresh conductor is also
quarantined*, and R2/R3 change a **shared workspace API** (`compile_probe`'s signature, the
lift's markers) that the coverage crate consumes. A workspace-wide signature change cannot be
split across the quarantine boundary inside one atomic, always-compiling commit: whoever changes
`compile_probe` must, in the same commit, fix `coverage/lib.rs:426`'s call-site, or
`cargo build --workspace` (a conductor gate) breaks with no one able to fix it. **Therefore
R2/R3/R4 must be built end-to-end by an un-quarantined agent (engine + coverage + goldens
together), not the conductor.** The conductor's actual spine is the **guard tier (the 6 `guard23-*`
xfails, door-4)** — that build does *not* require R2/R3/R4 (it touches the plan/analysis emitters,
not `compile_probe`'s signature nor the marker lift), so the conductor is **not blocked** by this.

## 23Q-fd4 — why I did NOT build R2/R3/R4 now (deliberate, not timidity)

+SURE on each:
- **Not "little work."** R4 is a big-bang golden re-bless across ~145 cases (`23E §5`); R2/R3 are
  moderate engine rewrites (lift derivation, emitter signature). The human's "very little work"
  cannot mean this.
- **BLESS is exclusive and forbidden right now.** `spike/CLAUDE.md`: BLESS re-blesses from
  whatever `target/debug/dorc` exists that instant; concurrent agents share one `target/`; never
  BLESS while a sibling builds. `git worktree list` shows a live conductor + many active
  `agent-*`/`bridge-*` worktrees. A big-bang re-bless here is precisely the contamination hazard
  the rule forbids. R2/R3/R4 needs a controlled, sibling-free, orchestrator-supervised session.
- **It carries design-STOP items, not mechanical ones.** `23E §3`: `jc-dpkg-i` (an effect with no
  check function to derive from — needs new oracle authoring or a pinned-test break) and
  `jc-fblessed` (the multi-selector `resolve_probe` structural floor *evaporates* under
  check-as-oracle — a real soundness-posture shift). Both are human rulings, not agent cleanup.
- **The coverage side cannot be done standalone.** Converting coverage's fixtures to the inline
  dialect makes them *parse* (R1 landed additive parsing) but *not lift* to effects (R2 hasn't
  wired the lift), so the coverage tests would break (0 sites oracled). Editing the `compile_probe`
  closure to the new signature won't compile (the new signature isn't in the plan crate). So there
  is **no** standalone coverage change to make — it is engine-gated.

## 23Q-edit — pre-computed mechanical coverage edits (turnkey for the future R2/R3/R4 session)

The complete list of coverage-crate touch-points, so the un-quarantined R2/R3/R4 session is
mechanical. Only THREE spots consume the changing APIs (grep-exhaustive over `coverage/src`):

- **23Q-edit1 (R3, the closure) — `lib.rs:426`:**
  `dorc_plan::compile_probe(&parsed.value, &cfg, &classes, |kind, selector| idx.resolve_probe(kind, selector).map(|p| p.body.clone()))`.
  This is a **byte-for-byte mirror of the cli's** `compile_probe` call. When R3 reshapes the 4th
  param from `impl Fn(KindId, SelectorId) -> Option<String>` to the new check-body+argv source,
  apply the *same* edit the cli's call-site gets. No coverage-specific logic.
- **23Q-edit2 (R2, the fixtures) — `lib.rs:1134-1153` and `main.rs:527-539`:** two inline
  `PKG_ORACLE`-style strawmen using `oracle_kind=package` / `oracle_probe_package()` /
  `oracle_effect apt-get …` / `apt_get__check`. When R2 retires the markers, convert both to the
  inline dialect per `23E §5`'s package recipe: `apt-get.check()`, keep `pkg : package = "$1"`, add
  `case $verb in install|purge) … : package:"$pkg".installed[!] ;; esac`. The coverage attribution
  logic downstream is unchanged (it reads dispositions, not the authored surface), so re-run the 23
  coverage tests; they should stay green with the new fixtures once R2's lift is live.
- **23Q-edit3 (goldens):** none in coverage — the crate emits no e2e goldens (its `main.rs` renders
  a table/TSV checked by unit tests, not the `spike/e2e/` goldens R4 re-blesses).

~SUSPECT this is the whole coverage surface; the differential-test discipline `23E §3` prescribes
(new_lift == old_lift on every fixture) will catch anything I missed.

## 23Q-verify — baseline certification (as of HEAD 4609e9e, tree clean)

- `cargo test --workspace` (mise): **all binaries pass, 0 failed** (1 pre-existing ignored). Oracle
  lib 42, plan 48/47/…, coverage lib 19 + bin 4. +SURE green.
- `cargo test -p dorc-coverage`: 19 + 4 pass, 0 fail.
- Matches `23E §9`'s R1 certification (118 e2e / 6 designed xfail / 0 red; unchanged code since —
  HEAD is a doc-only commit atop R1). I did not re-run `e2e/run.sh` (unchanged compiled surface +
  BLESS/contention caution); relying on the `23E` certification + clean tree + the workspace-test
  spot-check. ~SUSPECT the e2e tally is unchanged (nothing touched the emitted surface).

## 23Q-fd5 / 23Q-fd6 — the two things I deliberately left alone

- **23Q-fd5 corpus oracles (old dialect):** frozen evidence (`dash -n` only, "frozen from birth"),
  used only by the manual `21B` rollup (not a gate, not a build input). R1 is additive so they
  still parse/lift. Converting them is neither required nor safe-to-freeze-break now. Leave frozen.
- **23Q-fd6 check-cost banding:** the human PARKED it (`23Z` step 4) and de-tasked it. It would use
  the quarantined corpus as a calibration data source — exactly the train/test contamination the
  firewall exists to prevent. Not in scope; not done. If ever un-parked, it too is un-quarantined
  work with a *sanctioned* data source, not the corpus-by-default.

## 23Q-rec — flagged for the human (NOT done unilaterally — cross-cutting, per inv-superposition)

~SUSPECT worth a decision, but a posture change I will not make on my own:
- **23Q-rec1 (decouple option):** `crates/coverage` is a full `--workspace` member, so it will
  *keep* colliding with every future engine-internal API change under the quarantine (not just
  R3). If that recurring friction becomes painful, consider moving `coverage` out of the default
  workspace gates (`exclude`) so engine evolution never breaks the conductor's `--workspace` build;
  an un-quarantined agent then reconciles coverage on its own cadence. **Cost:** coverage loses
  gate coverage (can rot silently). It is a non-kernel measuring instrument ("never fails a build")
  so the cost is arguably acceptable — but it's the human's call, and it's *not* needed now (the
  guard-tier build doesn't touch these APIs). Do NOT do this pre-emptively.
- **23Q-rec2 (dead-end noted so no one re-chases it):** `weights.rs`'s 1A adapter seam has **no
  artifact to wire** — round-1A produced a *capability* matrix (analyzer-can-model, the
  `observable_matrix.rs` test), not the *per-line criticality* scores `from_line_scores` wants. The
  `ai/r1A-H2SALS` worktree exists but contains no criticality file. The seam correctly stays
  unfulfilled; the doc-comment is forward-looking, not stale. Don't touch `weights.rs`.

## 23Q-fw — firewall discipline observed

Everything task-descriptive is in THIS file only. I deliberately did **not** update the
"blocked on the C2/coverage ruling" wording in `spike/CLAUDE.md` or `crates/oracle/CLAUDE.md`,
even though h5 is now resolved: those are conductor-read docs, and writing the resolution there
would leak the H2SaLS/coverage decision across the firewall. From the conductor's quarantined POV
the ruling stays "pending"; the human receives the resolution out-of-band via this note. The one
commit is generically messaged. No push. No system mutation. No corpus/spike-code change.
