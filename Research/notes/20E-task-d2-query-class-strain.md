# 20E — task-D2 (the Query effect-class): what landed, the validity-bit design, the firewall asymmetry, and what strained

> Round-20 spike note, append-only. Records task-D2 (205 §6 final / 20C §7): read-only guards become
> the first-class `Query` effect-class; `rule-query-validity` (the pristine-prefix reaching bit) gates
> whether a Query's probed rc is fold-usable; the wrong-concrete firewall RELAXES for valid Query sites
> only; the transitional `declared-rc` lane (the 19I §2 rc-injection mechanism) is fully dead. AI-authored,
> confidence-marked. Trust R/D/I/K + 19H/19I + the human rulings over this. Builds on 20C (the WIRE),
> 205 §2 (rule-query-validity), 20A §4 st-3 (the query-doesn't-invalidate-query refinement), 206 (the
> headline cost + the recovery doors).

## §0 What landed

- **`oracle::Polarity` gains `Query`** (third variant, alongside Establish/Kill). The `oracle_effect`
  grammar accepts a third polarity word `query`: `oracle_effect command '' query present` declares
  `command -v` as a read-only observer of `tool:<x>#present`. Lifts like any cell (`EffectCell`);
  verbless guards key the ε-verb as before.
- **`analysis::CommandEffect::Queries(FactKey)`** — a read-only observation. In `classify`'s
  reaching-defs transfer it gens NOTHING (treated exactly like `Pure`): a Query poisons no downstream
  ambient-ness and invalidates no downstream Query. This is the gen-side of rule-query-validity.
- **`SkipClass::QueryResolvable { fact, valid }`** — a Query site is probe-resolvable like an
  `EstablishAmbient` (its check IS the probe), carrying its `valid` bit. `valid` = the site's IN-state
  is pristine (`Reach::is_pristine`: the empty fact-set — no write-or-unknown reached from entry).
- **The firewall RELAXATION, Query-only** (`cli::facts_from_sites`): a record's rc feeds the fold's
  Status channel ONLY for a `ProbeSiteKind::Query { valid: true }` site (the guard's own rc); an
  establish site's rc stays `Predicted::Top` UNCONDITIONALLY (it is the probe-command's rc, never the
  mutator's), and an invalid Query's rc is withheld too (stale). `ProbeCheck` gained a `site_kind`
  discriminant to carry this engine fact to the cli.
- **The guard's own elision** (`ReplaceLicense::prove_query_replaceable`): a valid Query guard with a
  known probe-sourced rc, passing the consumption gates, is `Replace`d by `StandIn::from_rc(rc)` — rc 0
  ⇒ `true`, rc 1 ⇒ `false` (the formerly zero-coverage Exit(n)/non-zero path, us-sure-drift closed).
  Convergence does NOT gate it (a Query has no mutation to be already-done); rule-query-validity +
  known-rc + consumption do.
- **The transitional lane is DEAD**: `declared-rc <id> rc=N` record form removed from the cli parser;
  `SiteResults` re-shaped to one record per site (`SiteRecord { verdict, rc }`). The 19I §2 rc-injection
  mechanism is fully retired (stage-2 complete).
- **`fold-oror-guard-omits` re-grounded**: `tool.oracle.sh` re-declares `command -v` as `query` (was
  `establish`-shaped — 20B cov-5); `probe-results.txt` drops the `declared-rc` line and keys a genuine
  `site 0 effect=holds rc=0` Query record; the fold folds from THAT rc. Re-blessed; the apply section is
  byte-identical to the prior golden (only the case comment + the now-genuine rc-source changed).
- **Two new e2e exec cases**: `exec-query-guard-composition` (the 206-priced recovery door, genuine) and
  `exec-query-after-mutator-runs` (the invalidation pin). §4.

## §1 The validity-bit design (rule-query-validity as one reaching bit)

The bit is "does a write-or-unknown reach this site from entry?" — implemented on the EXISTING `Reach`
reaching-defs solve, no new pass:

- `Reach` already tracks `Facts(BTreeSet<FactKey>)` (mutators genned each cell) or `Top` (an Opaque ran).
  A Query gens NOTHING, and a blessed-pure builtin gens nothing — so the IN-state at a node is exactly
  the writes-or-opaque that reached it. `solve.rs` documents `states[v]` is the IN-state (the state
  *before* node v), which is precisely what a guard's validity asks about (excluding the guard itself —
  and a Query has no self-gen anyway, so in == out for a Query node).
- `Reach::is_pristine(&self) -> bool` = `matches!(self, Reach::Facts(s) if s.is_empty())`. Non-empty
  (some mutator genned a cell) OR `Top` (an opaque ran) ⇒ non-pristine ⇒ invalid.
- This gives the three st-3 facts for FREE (no special-casing):
  - upstream MUTATOR (any Establish/Kill, ANY cell) ⇒ non-empty Facts ⇒ invalid (pristine-prefix, NOT
    same-cell — 205 §2's correction of the 202 §2 same-cell rule);
  - upstream Opaque ⇒ `Top` ⇒ invalid;
  - upstream QUERY ⇒ gens nothing ⇒ pristine preserved ⇒ downstream Query STAYS valid (st-3 — the
    guard-stack idiom `command -v a || …; command -v b || …` keeps all its folds);
  - upstream blessed-pure builtin (`:`/`set`/`echo`…) ⇒ gens nothing ⇒ pristine preserved ⇒ valid.

+SURE this is sound and minimal: it is the same machine the establish-ambient gate already runs; the
only new line is the `is_pristine` query and the `QueryResolvable` arm. The bit is a phase-agnostic
fact computed in the engine (`inv-superposition` honored — §3).

Pinned directly at the classify layer (`analysis::effect::tests`): `lone_query_guard_is_resolvable_and_valid`,
`query_does_not_poison_downstream_establish`, `query_after_query_stays_valid_st3`,
`query_after_mutator_is_invalid`, `query_after_opaque_is_invalid`.

## §2 The firewall asymmetry — test evidence, BOTH directions (the heart of the task)

The disaster-class asymmetry (20C §2 made concrete): a record's `rc` means a DIFFERENT observable per
site-class, so the cli must feed it to the fold's Status differently. Three directions, each pinned at
the cli unit layer (`cli::tests`) AND end-to-end:

- **dir-1 — establish site rc NEVER feeds Status** (`firewall_establish_site_rc_never_becomes_fold_status`):
  a `ProbeSiteKind::Establish` site reporting `holds rc=0` ⇒ `Observable.status == Predicted::Top`. The
  rc carried is `dpkg-query`'s, not `apt-get`'s; feeding it would be the confidently-wrong concrete. This
  is unconditional — there is no validity that would admit an establish site's rc.
- **dir-2 — valid Query site rc DOES feed Status** (`firewall_valid_query_site_rc_feeds_fold_status`):
  a `ProbeSiteKind::Query { valid: true }` reporting `holds rc=0` ⇒ `status == Value(0)`; reporting
  `absent rc=1` ⇒ `status == Value(1)` (both rc directions carry through). This is the relaxation that
  replaces the dead `declared-rc` lane.
- **dir-3 — invalid Query site rc WITHHELD** (`firewall_invalid_query_site_rc_withheld`): a
  `ProbeSiteKind::Query { valid: false }` reporting `holds rc=0` ⇒ `status == Predicted::Top`. The
  validity bit is the engine's; the cli only honors it.

End-to-end, the SAME guard idiom proves the asymmetry both ways (§4): valid ⇒ folds the install +
substitutes the guard (empty run-set); invalid ⇒ both run (run-set proves it). The matrix
(`plan/tests/observable_matrix.rs`) mirrors the cli firewall in `plan_query` and pins the plan-layer
dispositions: `query_guard_holds_omits_install_and_substitutes_guard`,
`query_guard_absent_keeps_install_live_exit_revival`, `query_guard_invalid_after_mutator_runs_for_real`,
`query_guard_consumed_stdout_blocks_substitution`.

Anti-masking (`inv-probe-sourced-values`): the invalidation pin's probe-results deliberately report the
guard `holds rc=0` — the value that WOULD fold the install if valid — so the test proves the validity
GATE, not the rc, blocks the fold. No test hand-injects a status the check itself should predict.

## §3 `inv-superposition` + `inv-must-may` honored

- The `valid` bit is a phase-/orientation-agnostic fact emitted by `classify` (the engine); the COLLAPSE
  (feeding the rc into Status, or withholding it) happens in the phased caller (`cli::facts_from_sites`).
  The engine bakes no phase. `prove_query_replaceable` additionally gates on `valid` directly (belt +
  suspenders) so an incorrectly-wired caller cannot smuggle a stale rc past the bit.
- `inv-must-may`: the consumption fact arrives as `May<Powerset<Channel>>` and can only BLOCK
  (`consumption_ok`, the shared gate factored out of both substitution paths). A Query guard's
  substitution is NOT a mutation-elision (it has no mutation), so `inv-must-may`'s mutation-licensing
  rule does not bind it — it is value-preservation, gated by validity + known-rc + consumption. The
  `Derivation` records `LicenseVia::QueryGuard` to keep the witness honest about which path it proved.
- `tc-*` flagged, not settled in-component: see §5.

## §4 The two new e2e cases (the composition + the invalidation pin)

- **`exec-query-guard-composition`** (20B cov-5, the 206 recovery door made genuine): a `set -e` book,
  `command -v nginx || apt-get install -y nginx`, guard holds (`site 1 effect=holds rc=0`), nothing
  mutates upstream (valid). RESULT: the fold reads the guard's known rc 0 ⇒ the install is OMITTED, AND
  the guard substitutes to its exact stand-in `true`, UNDER errexit. Run-set: EMPTY (only `set -e`, a
  builtin, and `true` run). This is the composition cov-5 flagged "composed-but-never-tested" — a
  genuinely probe-sourced Query rc under `set -e` that BOTH elides the install and substitutes the guard.
  Contrast `exec-errexit-top-status-runs` (a MUTATOR under `set -e`, ⊤ rc, runs): the Query rc is
  probe-sourced, so it folds where the mutator rc cannot.
- **`exec-query-after-mutator-runs`** (the invalidation pin): same guard BELOW `apt-get install -y curl`
  (a mutator establishing `package:curl#installed`). The guard reports `holds rc=0` BUT a write reaches
  it ⇒ INVALID ⇒ the firewall withholds the rc ⇒ the fold cannot resolve the `||` ⇒ the nginx install
  stays LIVE and the guard runs for real. Run-set: `apt-get install -y curl` AND `apt-get install -y
  nginx` (the nginx install's presence is the proof the invalid guard did NOT fold it). The guard
  `command -v` is a shell builtin, so it logs no `ran:` line — its running-for-real is witnessed by the
  install not being omitted (noted in the case).

Both cases pass the ap-2 `dash -n` gate AND the exec-under-mocks gate.

## §5 What strained / tc-* flagged (conservative defaults taken; flagged up, not settled)

- **strain-D2-bare-query-elision (tc-query-bare-elision)** — a VALID Query guard with a known rc whose
  observables are ENTIRELY UNCONSUMED (a bare `command -v nginx` on its own line, no `||`/`if`/capture)
  is STILL substituted (to its rc stand-in `true`/`false`), since `consumption_ok` finds nothing to
  block. This is sound (a Query mutates nothing; the rc is probe-sourced) but cosmetically aggressive —
  it elides a read the admin wrote, replacing it with `true`. No corpus case exercises a bare unconsumed
  Query (the matrix + e2e all consume the guard via `||`), so it is untested-in-anger. ~SUSPECT the
  right long-run rule is "only substitute a Query whose rc is actually consumed" (a bare read has no
  observable worth eliding, and keeping it costs nothing — it is already read-only), but that is a
  judgment call about elision AGGRESSIVENESS, not correctness. Flagged for the human/orchestrator;
  conservative-for-now = the current behavior is SOUND, just possibly over-eager. (It cannot
  under-execute: a Query has no mutation to drop.)

- **strain-D2-query-effect-verdict-unused** — a Query site reports an Effect verdict (`holds`/`absent` ⇒
  `Converged`/`Diverged`), which flows into `Observable.effect` and thus the `Derivation.verdict`, but
  NOTHING downstream consumes a Query's Effect channel: the guard's elision is driven by its Status (rc),
  and the install's omit is driven by the fold reading that Status. So a Query's Effect verdict is
  carried-but-inert (provenance only). +SURE this is correct (a Query's "convergence" is not a
  mutation-elision signal — there is no mutation), but it is a latent oddity: an observer might expect
  `effect=absent` to matter. It does not. Recorded so a future reader doesn't wire it to something.

- **strain-D2-per-selector-probe (inherited from 20C strain-D1-perselector)** — UNCHANGED by D2 but
  worth re-noting under the Query lens: the probe wrapper is per-KIND. A `tool` kind has one selector
  (`#present`) so the `command -v` Query is unaffected, but a multi-selector Query kind (none in the
  corpus) would ship one probe body for all selectors. Deferred (the `an-per-entity-selector`-into-probe
  work, 20C §9 tc-perselector-probe).

- **strain-D2-multi-cell-query** — a verb mixing `Query` with `Establish`/`Kill` cells (a hypothetical
  multi-cell verb) falls to `MustRun` (the `[CommandEffect::Queries(f)]` arm requires EXACTLY one Query
  cell). Sound (`kFAIL-perform`: runs), but it means a "reads X and writes Y" verb is never
  Query-resolvable. No corpus case; deferred-not-irrelevant (returns if a real verb both reads and writes
  in one declaration).

- **strain-D2-validity-is-coarse (recorded, priced, accepted — 205 §2)** — the pristine-prefix bit
  surrenders ALL guard-folds after ANY mutator (incl. after `apt-get update`, an unrelated cell). In an
  update-first book (`apt-get update; …; command -v nginx || install`) the guard's fold is lost even
  though `update` cannot affect `tool:nginx#present`. This is the deliberate cost: the precise fix is
  cross-kind dependency edges (`tool:X#present` depends-on `package:X#installed`), the recursive-algebra /
  dir-timed-probe direction, deferred with that linkage recorded (205 §2). The bit is a SOUND
  over-approximation, not a precision claim.

## §6 What died with the transitional lane

- The `declared-rc <leafid> rc=N` record form (cli parser) — gone. `parse_results` now parses ONE line
  form (`site <id> effect=W rc=N`) and carries the rc in `SiteRecord`; the firewall decides usability.
- `SiteResults`'s `verdict` + `declared_rc` split maps → one `records: BTreeMap<LeafId, SiteRecord>`.
- The cli's `firewall_site_record_rc_never_becomes_fold_status` + `declared_rc_lane_feeds_fold_status`
  tests → replaced by the three firewall-direction tests (§2). The dead `declared-rc` line is now just an
  unrecognized line (dropped ⇒ kFAIL-perform), pinned by `parse_results_drops_garbage_kfail_perform`.
- 19I §2's rc-injection mechanism is fully retired (stage-2 complete, per 20C §7 / 20B §3).

## §7 Goldens touched (golden discipline)

- `spike/e2e/cases/fold-oror-guard-omits/expected.out` — re-blessed. REASON: the case re-grounds from
  the dead `declared-rc` injection to a genuine Query record's rc (`tool.oracle.sh` `establish`→`query`;
  `probe-results.txt` drops `declared-rc`, keys `site 0 effect=holds rc=0`). The APPLY section is
  byte-identical to the prior golden (the line still collapses to `true` — guard `Replace`d + install
  `Omit`ted); only the case comment and the (now-genuine) rc-source changed. Diffed before bless: zero
  apply-disposition delta beyond the intended comment text.
- `spike/e2e/cases/exec-query-guard-composition/` (NEW) — fresh `expected.out` + `expected.ran` (empty
  run-set).
- `spike/e2e/cases/exec-query-after-mutator-runs/` (NEW) — fresh `expected.out` + `expected.ran` (curl +
  nginx installs).
- The other 43 cases: BLESS-run left them byte-identical (git shows no change) — verified via
  `git status`.

## §8 Gate status

All green (from `spike/`): `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -D
warnings` clean (NO new expects; two `then`→`then_some` + one doc-backtick fix were the only clippy
nits); `cargo test --workspace` 252 pass / 0 fail / 1 ignored (the pre-existing HOLE#1 subst-in-redir
spec, untouched). `sh e2e/run.sh` 46/46 (44 prior + 2 new, incl. the standing `render-case-arm-oneliner-wrong`
xfail). `mise x -- typos spike` clean (from worktree root).

## §9 What D3 inherits

- **The probe-exec-under-mocks gate (rule-probe-exec-gate, 205 §1) is STILL NOT a gate.** D2's e2e cases
  execute the rendered APPLY under mocks (the existing exec gate), but the rendered PROBE is still only
  `dash -n`-checked, not executed under check-command shims. The probe's self-reported rc (the Query
  record's `rc=`) is hand-authored in `probe-results.txt`, not produced by running the probe. D3's
  probe-exec gate would run the rendered probe under `dpkg-query`/`command`/`getent` shims and assert the
  emitted records — closing the loop on the Query rc's provenance (today it is fixture-authored, the
  19I §3 trap's residual; 20C §3 noted the establish-side equivalent). The Query firewall makes this MORE
  load-bearing: a valid Query's rc now actually feeds the fold, so a wrong probe-emitted rc would be a
  wrong fold — exactly the class the probe-exec gate catches.
- **`tc-query-bare-elision`** (§5 strain-D2-bare-query-elision) wants a human/orchestrator ruling: should
  a Query whose rc is unconsumed be substituted at all? Conservative-sound default is in place; the
  ruling is about aggressiveness.
- **The vouch-closure check (`dq-reflexive-probe-inertness`, 205 §3 dev-reflexive) is STILL OWED.** D2
  ships Query check bodies into the probe (`tool__check`, `command -v -- "$tool"`), but nothing refuses a
  probe body containing a call that is neither the oracle's own command, a declared Query, nor a
  blessed-pure builtin. The Query class makes "declared Query" a real category the closure-check can now
  name (a Query's check IS a sanctioned read), so D2 sharpens what the check should allow — but the check
  itself is unbuilt (task-D scope, 205 §6).
- **The cross-kind dependency edge** (strain-D2-validity-is-coarse, §5) is the precise fix for the
  update-first cost; it is the recursive-algebra / dir-timed-probe direction, deferred with linkage
  recorded (205 §2).
- **`ProbeSiteKind` is the seam the probe-exec gate + any future per-site policy keys on.** It carries
  Establish-vs-Query{valid} to the cli; a richer probe model (per-selector, or a real probe-plan-builder
  — `F-FW3`) would extend it.

## §10 Confidence summary

- +SURE: the validity bit is sound + minimal (it is the existing reaching-defs IN-state, read through
  `is_pristine`); the firewall asymmetry is correct and pinned both directions; the transitional lane is
  fully dead; the apply-disposition of `fold-oror-guard-omits` is unchanged (re-grounded, not re-behaved).
- +SURE: the Exit(n)/non-zero-rc guard path now has coverage (the `false` substitution, both matrix and
  the absent-direction firewall test).
- ~SUSPECT: `tc-query-bare-elision` is the one place current behavior is sound-but-possibly-over-eager;
  flagged for ruling rather than guessed.
- ~SUSPECT: the Query Effect verdict being carried-but-inert (strain-D2-query-effect-verdict-unused) is
  correct but a latent reader-trap; recorded.
