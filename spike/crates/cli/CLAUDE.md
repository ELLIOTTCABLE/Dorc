# spike/crates/cli — CLAUDE.md

Role: the round-trip driver (book + oracles → read-only probe → results → eliding
apply) and the e2e acceptance harness's contract. The ONE place determinism is
relaxed — real I/O at the edges only. Read `spike/CLAUDE.md` first. Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law

- **lib-target-is-a-loom-seam** (`289:rul-worldless-route-honest-trigger`; widened at the W4
  drifted-driver fold, `28H`; the loom-final arc moved SIX regions across it — `28L`) — the lib
  target is the INTERNAL invocation-and-render surface: usage text, `Args`/`LintArgs`/`Mode`, the
  parsers, `humane_read_error`, the drifted-why seat, and the extracted modules `why.rs`
  (`WhyReport` + `why_report_parts`) · `world.rs` (`WhyWorld::analyze`/`analyze_measured` +
  the shared ship-body helpers) · `kinds.rs` (resolver/reaches) · `survival.rs` (footprints/
  wrapped-analysis/carry) · `results.rs` (the intake segment: scope types, `parse_admitted_results`,
  `admit_controller_records` vs the fenced `admit_fixture_records`) · `fixpoint.rs` (validity
  rounds + cascade attribution) · `bundle.rs` (the pure occurrence-keyed bundle/storage
  projection). One implementation — `main.rs` keeps call sites and every I/O
  edge (clock readers, git, terminal width). It exists so `dorc-loom` can drive REAL invocations
  in-process; it is NEVER a public API — `publish = false`, nothing outside `dorc-loom` and the
  two bins may depend on it. VALUES cross the seam, QUERIES do not. If something in the lib
  starts wanting a clock, a file, or an env read, it is on the wrong side of the seam.
  (`RunClock::Absent` on every loom path — a committed transcript must be a fixpoint.)
- **invocation-errors-are-registry-codes** (`288` §6) — the parsers return typed `Diag`s, never
  strings. The `dorc: ` / `dorc: lint: ` / `dorc-sh: ` prefixes and the usage synopsis are print-seat
  CHROME the three report seats own, never catalog prose. Exit codes are unchanged and never read
  severity. A new invocation error mints a code + a defining case like any other surface.
- **chrome-comes-from-the-registry** (`289:rul-arrangement-home-is-registry-plus-transcripts`) —
  the help page and the seat-appended usage synopsis are arrangement-registry entries
  (`dorc_cli::help_text` / `usage_text`), not consts; their words are edited through
  `crates/aid/tests/cli-help-page.loom`, never in source. A new user-facing chrome string mints a
  registry entry, not a `const`.
- **io-at-edges-only** — keep I/O in `run()`/`main`; the pipeline
  (`parse → cfg → classify → compile_probe/build_plan`) stays a total
  `Carrier<T>` function of its inputs; never let a clock/RNG/env-read leak
  inward "to help".
- **stdout-contract** — plan-producing modes emit EXACTLY probe-then-apply (split on
  shebangs); `bundle` emits only its deterministic inert archive. Diagnostics go to
  stderr in every mode. A new stdout species must be mode-owned rather than interleaved
  with either contract.
- **bundle-projection-is-pre-contact-and-not-placement** (`30I` step 5b) —
  `bundle::project` consumes only `StaticLoadSnapshot` plus the loader's complete
  `LoadAccount`; it resolves and reads nothing. Every textual occurrence remains distinct,
  speculative branches are included, and copied bytes plus line maps come only from
  `strip_file_with_map`. Generated `storage_path`s name inert archive entries, never runtime
  `.` targets or a materialization recipe. `dorc bundle` returns before records intake,
  planning, settlement, whylog writing, or host contact; executable placement and source-line
  replacement remain the post-`30L` artifact stage.
- **bundle-diagnostics-compose-occurrences-not-paths** (`30I` step 6) — production bundle
  validation diagnostics compose `LoadAccount` occurrence identity, `BundleFile` storage/copy
  identity, and the existing strip line map onto `aid::locator`; they never reconstruct an origin
  from path strings, comments, or source similarity. The ordinary authored diagnostic stays the
  primary frame while generated and nested-load frames remain visible. Comment-origin readback
  resolves to current source only on exact snapshot-byte agreement and stays aid-only either way.
  Static incoherence renders its located diagnostic but returns before archive stdout, preserving
  the existing refusal/exit. No locator value may enter loading, planning, or authority.
- **artifact-forms-derive-from-one-structure** (`30I:step-7-reify-plan-artifact-forms`) —
  `cli::artifact` settles ONE `Selection` (form + fallback + dependency files) from
  authored-before-contact inputs, and `Selection::with_plan` binds it to the plan
  projection. The stdout stream and the published tree both READ that `ArtifactSet`;
  there is deliberately no second assembly of the same bytes to fall back to. A form's
  dependency LAYOUT is the authored relative one, mirrored under the artifact root,
  because that is what makes every authored `.` operand — the book's and every nested
  one — resolve on the target unchanged (the cwd-analysis answer): the availability
  question is therefore a PATH question, and an absolute or escaping controller path
  makes the form unavailable rather than fudged. `plan.sh` is byte-identical under all
  three forms; a form is about where the generated files live, never about what the plan
  says. FLATTENING REFUSES rather than inlining while textual inlining rests on the
  `floor30-inline-dot-boundary` measurement alone, so an explicitly named form that
  cannot be served refuses pre-network and `auto` falls back and SAYS SO — never a
  silently different form. Mirroring is stated against the LOAD CWD
  (`dorc_core::loadpath::Cwd::relativize`, the inverse of `resolve_operand`), never
  against a stored path's own spelling: every source a book `.` reaches is filed under
  its CANONICAL key, which is ABSOLUTE whenever the edge could answer where the run
  stands — so a seat asking whether the stored spelling looked relative answered
  "unplaceable" for every real invocation while every in-process test, whose modelled
  cwd is the flat virtual one, said the opposite. That divergence shipped once and was
  invisible, because `plan.sh` is byte-identical under all three forms and a case
  asserting stdout alone cannot see which form it took
  (`30Nf:fnd-multipart-never-placed-anything-in-production`). A dependency OUTSIDE the
  load cwd is unplaceable rather than fudged
  (`need-controller-paths-never-cross-hosts`).
- **region-openers-are-demanded-not-defaulted** (`30N:rul-census-inputs-are-non-optional`) — the
  elision-region census is handed `region::CensusOpeners`, whose constructor requires EVERY
  opener signal the census cannot see for itself: `funcenv::unresolvable_loads`, the definition
  vectors, and the string-execution sites. An opener the census does not see is a population
  wrongly CLOSED, which is a wrong-elision one abstraction level up, so the shape is a required
  constructor rather than a defaulted parameter and a driver acquiring a new signal must visit
  that seat to drop it. Both drivers build it — the binary and `WhyWorld` — from the same frozen
  inputs, for the reason `one-definition-table-two-drivers` gives.
- **only-invocation-roots-are-ambient** (`30Mc:required-root-occurrence-identity`) — acquisition
  retains the explicit ordered pre-source ROOTS separately from the files it opens for their load
  programs. Only the roots reach `push_ambient`; a dependency is `SourceRole::LoadDependency`,
  loadable and positional, reached at its authored `.` inside its root's own `LoadProgram`. A
  dependency promoted to a root replays its program AFTER the authored one finished, which
  restores definitions the author `unset -f`'d — engine-created vouch authority. The
  classification is DEMANDED by `snapshot::LoadPositions` rather than defaulted, because two bare
  index sets side by side are swappable without a type error.
- **probe-ships-oracle-bytes-only** — the compiled probe is synthesized
  scaffolding + oracle bodies, never book contents (it never inherits the
  book's `trap`s). The `24J` raw-ship debt is REPAIRED and machine-pinned
  (probe-render tests assert the raw book site cannot appear in emitted bytes);
  their failure means it returned.
- **results-fold-to-run** — a missing or unparseable FACT folds to
  `Verdict::Unknown` ⇒ run (`kFAIL-perform`); keep that default, it is
  load-bearing. Never silently drop a selector on parse and widen a verdict to
  the whole entity — that is a wrong-elision under apply's fail-direction.
- **admission-precedes-the-fold** — the fold-to-run default answers a MISSING
  FACT, never a broken CHANNEL. Bytes reach the fold only through the bounded
  intake, whose outcome is three-way: `Admitted` proceeds; `NoObservation`
  (well-owned attempt, no usable fact) takes the fold above; `Refused` (framing,
  bounds, attribution, or integrity failure) returns BEFORE plan construction,
  artifact rendering, or whylog writing, and emits no plan carrying mutation
  authority. Never collapse `Refused` into `Unknown` and continue — "run
  everything" is the safe answer to not knowing the WORLD, and the wrong answer
  to not knowing whether we are still talking to the world we think we are
  (`rul-integrity-failure-withholds-mutation`).
- **attribution-is-controller-minted** — the run's framing/scope is minted HERE,
  at this edge, from controller-owned values; an incoming payload frame is
  CHECKED against it and never mints it. The width-one scope types are private
  and deliberately unshared. When a second scope first becomes representable —
  real transport, concurrency, retry, cross-host reuse, saved approval — carrying
  the scope has to become checking it, and this is the choke point where that
  lands.
- **the-fixpoint-owns-the-rounds-and-builds-nothing-else** (`26H` §4/§4¾, generalized by `30K`) —
  `plan::settle_effective_world`, driven by `fixpoint::WorldRoundModel`, re-derives classify,
  certified effective reach, decisions, and the records fold against the residual model until a
  round proves no further mutation un-runnable. Three things bind. FROZEN: book/CFG/value-flow, the
  ADMITTED records (admission runs ONCE, before the loop — no re-probe, no re-admission), the
  vouches, and the compiled probe. Probe EMISSION is untouched because there is exactly ONE
  `ProbePlan`, built from round 1 and never rebuilt; what moves per round is a validity VIEW over
  it, and nothing else about the record intake moves — an erased site KEEPS contributing its
  measurement, because the deadness of the line that measured the world does not un-measure the
  world. NEVER-SURVIVES: intermediate rounds are unobservable not because they are discarded but
  because they are never built — the loop body constructs a classification and a fold, and every
  plan, narrative, render, whylog write, and `report_at` sits outside it. The sole deliberate
  exception is the round-tagged derivation link (`attribute_cascades`), durable so `dorc why` can
  answer a cascaded elision; that is a HARD requirement, not polish. Cap-hit is unreachable
  (erasure is monotone, bound = site count) and DISCARDS the ledger to re-derive from origin
  rather than ship a partial fixpoint, so the degraded answer is exactly the pre-W-C one —
  `solve`'s own unenforceable-termination bargain, `debug_assert`-loud in dev and under DST.
- **the-frozen-set-includes-the-function-environment** (`28K` §2) — env resolutions (both
  visibility regimes) and the contested-family verdicts join the FROZEN set named above
  (book/CFG/value-flow/admitted-records/vouches/probe): computed ONCE from the origin model,
  before the loop. The fixpoint's ratchet erases EFFECTS; it has no authority over BINDINGS.
  Named forbidden scenario: a records-proven-dead branch containing a funcdef must NOT re-run
  env resolution and un-contest a family mid-run — a license once withheld is never regained by
  a later round. Enforced lexically at both ends (`the_fixpoint_loop_body_calls_no_funcenv_entry_point`
  here; `dorc_analysis::funcenv`'s `this_module_names_no_fixpoint_reachable_type` there), because
  the property is "the loop body cannot even spell it", which no type bound expresses.
- **the-book-is-a-definition-source** (`28K` §2a in-book lift) — the predict/verdict LIFT and
  SHIP lanes consume the SOURCE-wide vectors (`source_srcs`/`source_refs`/`source_paths` from
  `source_table`), never the oracle-only ones: a book's `foobar__is_converged` is an ordinary
  oracle recognized by name alone (USER_STORY stage 3), and those lanes zip per-file lifted sets
  POSITIONALLY — handing them a shorter `oracle_srcs` truncates the book's definitions away
  SILENTLY rather than failing, which is how the first cut of this shipped the wrong body.
  Oracle-only is still right for the whylog/attempt-scope record of what was LOADED; the
  survival lanes (`touches`, kind resolvers/reaches) stay oracle-only coherently among
  themselves in BOTH drivers, and widening them is its own dispatch — a separate question from
  whether the contested withdrawal reaches them, which it does
  (`withdrawal-is-applied-once-never-consulted`).
- **one-definition-table-two-drivers** (`28K` §2; the seat UNIFIED at `28Q` §1's conversion) —
  `world::definition_table` is the ONE reader of role funcdefs, and the binary and `WhyWorld` both
  call it: a why report answering from a different function environment than the run would be a
  decoration, which is the failure `lib-target-is-a-loom-seam` exists to prevent. `WhyWorld` now
  lifts the SOURCE-wide vectors — oracles in load order, then the book — so the book's
  `SourceFileId` is the LAST of them rather than one past a shorter vector, and a site a book
  definition owns answers there exactly as it does in the run. The interim shape (oracle-only
  vectors, the book sited one past them, book-owned sites withholding) is RETIRED: it was safe and
  agreed with the binary only by the coincidence that nothing in the corpus resolved a locus to a
  book-sited definition. Its widening carried the obligation
  `withdrawal-is-applied-once-never-consulted` names, and discharges it there. NAMED RESIDUE
  (`308b` F7, pre-existing): `WhyWorld` models no wrapped sites (`world.rs` `peeled` stays empty,
  disclosed at the module header), so a why report over a wrapped book explains a NARROWER world
  than the run — a scope cut, not a divergence; widening it rides whatever lane next touches the
  why-driver.
- **withdrawal-is-applied-once-never-consulted** (`28K` §1; carve CLOSED at `28Q` §1's
  conversion) — a contested family is removed from EVERY lifted set at THIS edge, before
  `classify` sees them, so the family is indistinguishable from one nobody described and no
  downstream seat has to remember to ask. Its sites fall to `Opaque` ⇒ `MustRun` ⇒ no vouch
  candidate, no probe ship, no license — the `erasure-is-applied-once-never-consulted` shape.
  Never re-plumb this as a per-seat predicate: a flag every present and future consumer must
  remember is the surface that rule exists to refuse. NO SEAT SITS OUTSIDE THE EDGE. Every
  lifted vector routes through it: `idx`/`checks`/`verdict_sets` in the binary, the same three
  in `WhyWorld` (which lifts the SOURCE-wide vectors and mints the contested fact from the same
  two `funcenv` calls, so a why report explains the run's own world), and the survival lane's
  `disturbs` sets through `survival::{lift_touches_sets, pair_touches_sets}` — withdrawal
  REMOVES at-most claims, which is fewer disjointness derivations, which is less sparing, the
  over-execute direction. Oracle-only-ness is a question about WHICH FILES a lane lifts (the
  kind-owner trio loads from the ambient prefix by design — `vocabulary-acts-stay-ambient`; the
  survival lane's own widening is still its own dispatch), never about whether the contested
  fact applies to them. The wrapper lane's edge act is `survival::WrapperSets::lift`
  (`308` §1): it lifts AND withdraws the `__lend_map`/`__enter` vectors in one constructor, so
  an un-withdrawn value cannot be spelled — a contested wrapper family peels nothing and enters
  nothing.
- **wrapped-ship-seat-verdict-primacy** (stage-0, 2026-08-16) — at a wrapped site the
  vouching inner VERDICT ships ahead of the predict, and `build_wrapped_vouches` mints
  its guard from `composed.inner_fn`/`inner_sh` — which is why, pre-stage-0, a predict
  body could reach apply-time GUARD position while the vouch traced the verdict
  (`fnd-wrapped-guard-carried-the-predict-model`; declared output in guard position, a
  standing refusal — closed). Two ratified asymmetries: when the verdict VOUCHES but its
  body cannot ship (contested closure), `resolve_inner_check` returns `None` and the
  site RUNS — never a predict fallback, because the vouch would still mint and a guard
  carrying a model is worse than a site that runs; and `entry_tolerance` lifts consent
  from the inner verdict ONLY when the shipped body IS that verdict (`safe-across` is
  per-FUNCTION consent — a declining verdict's top-level mark licenses nothing for an
  unmarked predict body; pinned both halves by
  `consent_rides_the_body_that_ships_and_no_other`).
- **rul-wrapper-members-resolve-independently** (`308` §1, the crosscheck burndown) — each
  wrapper member (`__predict`, `__lend_map`, `__enter`) is its own funcdef and binds
  independently in sh, so each resolves its OWN frame answer at the wrapped site; `detect_peel`
  runs on the RESOLVED predict, and a frame-live declining or absent body means the word is not
  a wrapper HERE (the site walls). A cross-FILE resolved pair is coherence-checked at the site
  (dual-peel tail positions AND the enter×lend shift pair — either mismatch understates crossed
  dimensions, the under-consented direction) and WALLS with a narrative record
  (`WrapperPairIncoherent`), never fail-fast: composition is nobody's self-contradiction, so
  `declarations-genuinely-contradict` does not apply; the per-FILE whole-unit check stays the
  authoring-time fail-fast. Consent, entry bytes, the peel model, and the carry proof all hang
  off `definition_before` now — funcenv precision is license-review-tier in this lane too.
- **one-helper-index-two-lanes** (`28K` §4) — `dorc_oracle::closure::HelperIndex` is built ONCE at
  this edge, from the same source vector the bodies are sliced out of, and threaded into BOTH the
  guard lane (`build_vouches`) and the probe's three ship seats (`ship_predict_body` /
  `ship_verdict_body` / `ship_predict_stage`). Building it per site would re-parse every source per
  site and, worse, leave two copies of the resolution rule to drift — the failure
  `oracle/CLAUDE.md the-frame-lookup-is-the-only-resolution-seat` records for the role lane. Since
  the emission stage the SURVIVAL/kind/entry-form lanes (`ship_touches_body`,
  `compile_resolvers`, `collect_reach_probes`, `strip_enter`) carry snapshots too, and the
  wrapper lane consumes the SHARED index rather than building a second one. The remaining
  closure-less residue is the wrapped guard's inline blob (`composed.inner_sh`): a wrapped and
  an ordinary guard sharing a helper would emit it twice — unreachable in the corpus; closing
  it means threading `Closure` through `ShippedCheck`.
- **helper-conflicts-report-at-the-load-edge** — `helper_conflict_diagnostics` mints one
  `helper-declaration-contested` per NAME, spanned at the second declaration, whether or not any
  pinned definition reaches it: loading both sources already rebound the name for every caller. A
  per-definition report would be a correlated cascade pointing N-1 authors at somebody else's file
  (`28O:dec-one-diagnostic-per-file-not-per-item`).
- **speculate-and-intercept** — the probe model resolves probe-gated branches by
  running the read-only check for real (oracles intercept; not Ansible
  check-mode blindness).

## The acceptance harness (`tests/e2e.rs` + `tests/looms.rs`; this contract is law)

- **an-artifact-set-runs-from-its-own-generation** (`30Nf` §4) — a case declaring `ARTIFACT_SET`
  gives its round-trip drive an `--artifact-dir`, and the exec gates run the PUBLISHED
  `<generation>/plan.sh` from inside that generation — the cwd the multipart execution
  contract gives an artifact (`30I`, the artifact-forms step). Exactly one generation is
  required; none means the run took a form that materializes nothing, and every exec gate
  below would then have measured the plan alone in an empty sandbox and passed. The
  published plan is asserted byte-equal to the apply block on stdout. Copying a case's own
  AUTHORED sources into the sandbox is the refused alternative: it would green a case
  against controller-side files the target never receives. The general law this encodes: a
  case minted to DEMONSTRATE a capability must OBSERVE that capability — an assertion that
  cannot distinguish feature-on from feature-off is not a demonstration
  (`30Nf:fnd-multipart-never-placed-anything-in-production` is the burn).

- **runners-live-here-cases-are-peers** (`288:phase-flat-tree-move`) — this crate owns the
  two central `harness = false` runners and the round-trip / lint case collections that sit
  beside them in `tests/`. `sh e2e/run.sh` is RETIRED; every gate below moved into
  `tests/e2e.rs` unchanged, and `cargo test --workspace` now runs the corpus. `autotests =
  false` + explicit `[[test]]` targets is what lets case DATA share the dir with the
  runners; the shape rules that classify a dir are `spike/CLAUDE.md`'s
  flat-test-tree-and-loom-placement.
- **per-case gates** — `dash -n` on BOTH rendered artifacts (the load-bearing
  runnability gate; the historical trap was a text-only golden diff shipping a
  non-runnable empty `then`-clause green, twice) · exec-under-mocks for cases
  with a `mocks/` dir (sorted run-set asserted against `expected.ran`, which
  MUST exist — missing ⇒ loud fail, never empty-want) · crash/empty guard
  (dorc rc≠0 or empty output hard-fails before the xfail lens and before
  bless) · the content golden-diff as a secondary check (catches wrong-elision
  CONTENT, to which `-n` is blind) · XFAIL/XPASS pin machinery (XFAIL is
  golden-text-BLIND by design — structural gates only; a surprise pass is a
  loud XPASS-to-promote).
- **bless-never-first** — `BLESS=1` regenerates goldens; gates run before bless,
  but bless cannot prove an elision RIGHT: fresh verified binary,
  orchestrator-only, diff inspected case-by-case (BLESS exclusivity —
  `spike/CLAUDE.md`).
- **bless-writes-renders-not-measurements** (r30; `spike/CLAUDE.md`
  emitted-is-measure-once-ground-truth) — bless has authority over what the ENGINE produced
  (`expected.out`, `expected.ran`, transcripts) and none at all over what the floor BINARIES
  produced. A case carrying `expected.emitted` is therefore REFUSED by `BLESS=1` — in
  `run_loom` before materialization and in `run_round_trip` for the dir form, both through the
  one pure `floor_bless_refusal` seat — and is writable only under the `BLESS_FLOOR=1` +
  `DORC_E2E_FLOOR_SHELLS` mint, where gate-9 re-measures and `bless_loom` folds the manifest in
  the same write that commits the transcript. Never widen `bless_loom` to fold a section no gate
  just re-derived: the discarded-measurement shape this closes looked green for three lanes.
- **bless-folds-only-on-pass** (r30) — `bless_loom` runs only when the case's own gates PASSED.
  Safe because nothing depends on partial folding: every gate comparing against a bless-WRITTEN
  golden is bless-aware and cannot fail on staleness (the content diff and the extra-replay
  compare are `!bless`-guarded; `exec_check`, `run_lint`, and gate-9 write-and-return before
  theirs), so what stays reachable under bless is structural, authored-fixture, or environmental
  — unhealable by a write, which gate-1 already says in its own words. Ungated, `exec_check`'s
  early `expected.ran` write folded into cases a LATER gate had just failed, leaving a fresh
  run-set beside a stale transcript. XFAIL is unaffected (its lens returns `Ok`, keeping its
  deliberate golden-text-blindness); XPASS now folds nothing, which is right — a case still
  wearing its marker has no asserted transcript to commit. Pinned falsifiably by
  `bless_folds_only_on_pass_selftest` (verified red with the gate removed).
- **empty-ran-has-two-stable-spellings** (r30 landmine) — an empty `expected.ran` section exists
  in the corpus as ONE blank line (content `""`) and as TWO (content `"\n"`), and both are
  fixpoints, so neither is drift. The fork is `exec_check`: it writes `format!("{got_ran}\n")`
  — `"\n"` for an empty run-set — but ONLY for a case carrying `mocks/`; a case without mocks
  never reaches it and folds the materialized empty file straight back. That is exactly why the
  floor mint is byte-stable across all eleven floor cases despite the asymmetry. Making that
  write unconditional, or "normalizing" either spelling, churns six goldens for nothing — the
  gate compares under `strip_trailing_newlines` and cannot tell them apart.
- **loom-form-is-the-same-battery** (`288:phase-e2e-loom-conversion`) — a whole-product case may be
  a single-file `.loom`: frontmatter instead of `NAME=value` markers, txtar sections instead of a
  fixture dir (`mocks/` included, dotfiles included, `expected.ran` as a byte section), and the
  committed transcript instead of `expected.out`. It is NOT a second harness — `run_loom`
  MATERIALIZES the case into exactly the dir shape and runs the unchanged gate battery over it, so
  a conversion cannot quietly drop a check. ONE closed key vocabulary exists and an unread key is
  refused against it in both runners: `dorc_loom::FRONTMATTER_KEYS` (the full ~22-key set, every
  row naming the gate that reads it), of which the e2e runner's run-lane set is a PROJECTION
  (`run_lane_key_names`, the `run_lane` flag) rather than a second list — the two had to stay
  subset-related by hand, nothing checked it, and a key one runner accepted and the other refused
  would redden the same file from the far side. `owns` is in the run-lane set although no e2e gate
  reads it, because ownership is corpus-wide and refusing the key there left a component with no
  authoring home. `dorc-loom keys` PRINTS the set (with the `code:` vs `arrangement:` split), so an
  author finds it without first provoking a refusal. A new key joins the vocabulary in the same
  commit that mints it, or its cases go red. The replay COMMAND is compared against the invocation the runner actually
  drives, so a transcript can never show one command while the gates run another. Corpus walks
  skip `*.sync-conflict-*` (sync residue is never a case).
- **one-fixpoint-authority-per-case** — `crates/cli/tests/looms.rs` render-fixpoints every committed
  loom through the in-process consumer; a whole-product loom declares `fixpoint: executed` instead,
  because its transcript is proven by running the REAL binary here (the stricter proof, and the only
  one the sanctioned-executor law allows for a case that materializes mocks). `fixpoint: executed`
  without a `run:` key is refused in the looms runner — otherwise the transcript is proven by
  nothing. The old 4-case `DIRECT_PLAN_CASES` gate in `dorc-loom` is GONE
  (`289:rider-fixpoint-gate-rationalize`); do not re-mint a second render-fixpoint authority.
- **tolerate-is-a-closed-vocabulary** (`288:prop-normalizer-closed-vocabulary`) — a case DECLARES the
  named nondeterminism class it tolerates (`tolerate=<class>` marker / `tolerate:` frontmatter) from
  an engine-owned vocabulary, and the named normalizer is applied to the CAPTURE at bless AND at
  check, so the committed bytes are the canonical form. Never a free regex; never a check-only
  relaxation (the retired `RAN_ORDER=lax` shape blessed raw bytes and compared sorted ones, so the
  committed file recorded an interleaving nothing asserted). Current vocabulary: `pipe-stage-order`.
- **needles-are-structural** (`288:prop-structural-needles-only`) — `expected-diagnostics` /
  `expect-diagnostic:` is a list of code SLUGS; the `error[<slug>]` needle is DERIVED and every slug
  is validated against the generated catalog, so a dead slug is REFUSED and a declaration is an
  ASSERTION (a declared-but-unfired code is red). The why/hint/why-chain needles stay free text —
  legal, rare, and carrying real semantic content rather than catalog prose.
- **count-drifts** — the case-count drifts; count the dirs, never trust a
  literal. The runners pin only a NON-EMPTY discovery floor (a zero-trial suite would
  exit green — the one failure their own path constants can cause and not report).
  RESIDUAL, unchanged from the sh harness: deleting ONE case dir shrinks the suite
  silently. That deletion is visible in the diff; a broken root is not, which is why the
  floor guards the root and nothing guards the count.

## Direction

- **wire-records** — the ad-hoc stdin results format is replaced by the `262`
  §2 records lane at block-rebuild: framing header/sentinel · per-record
  terminal token · coordinate fields last-to-token · partial deriv-family ⇒
  wall-total · value stdout carries arbitrary single-line bytes (embedded
  spaces survive round-trip — `279f` rider).
- **probe-projection-second-caller** — the probe plan-builder is the only real
  SECOND phased caller of `inv-superposition` (the load-test of "engine emits,
  caller collapses"): build it as a genuine `Phase::Probe` caller; never bake a
  posture into the kernel to make it easier.
- **scope-boundary** — the real apply-executor, transport (`KNOBS:kCOMMS`), and
  multi-host fan-in stay out of spike scope. Keep the binary a thin driver:
  arg-parse, file-read, call the kernel, print. Resist absorbing pipeline
  logic.
