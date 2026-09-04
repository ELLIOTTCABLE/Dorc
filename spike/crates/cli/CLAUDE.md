# spike/crates/cli — CLAUDE.md

Role: the round-trip driver (book + oracles → read-only probe → results → eliding
apply) and the e2e acceptance harness's contract. The ONE place determinism is
relaxed — real I/O at the edges only. Read `spike/CLAUDE.md` first. Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law

- **lib-target-is-a-loom-seam** (`289:rul-worldless-route-honest-trigger`; widened at `28H`/`28L`;
  re-cut at `30X`) — the lib target is the INTERNAL invocation-and-render surface: usage text,
  `Args`/`LintArgs`/`Mode`, the parsers, `humane_read_error`, the drifted-why seat, `engine.rs` (the
  ONE parser-independent semantic pipeline, with ordered live output events and typed injected
  edges), and `compose.rs` — the COMPOSITION ROOT `compose::run(&Seams)`, which acquires sources and
  implements the clock/entropy/transport/disk/terminal edges as selected by the `Seams` bundle
  (`30X:model-seams-are-one-bundle`; `seam.rs`: `Seams::os()` is the ONLY constructor of the
  production variants; `HarnessSeams` is the constructor-side subtype with NO arm for `RealSsh` or
  `Os` roots and a total `From`; `HarnessSeams::from_env` is the ONE parser of the `DORC_SEED` /
  `DORC_SEAM_*` variables). The three bins are edge VALUES plus one call
  (`30X:inv-division-at-the-narrowest-edge`, whose above-the-seam remit is bounded and must never
  grow): `bin/dorc.rs` = `compose::run(Seams::os())`; `bin/dorc-harness.rs` = `from_env` over the real
  environment, refusing loudly when no seam is set; `bin/dorc-sh.rs` = `dorc_transport::Posix::find()`
  (the resolved shell is an edge VALUE — `one-shell-answer`, never a bare PATH lookup) plus
  `compose::shim_strip_and_run`. The shipped `dorc` reads NO harness-shaped environment
  (`rul-fixture-identity-never-production`, public-interfaces reading; its six former env pins are
  seam columns). The other extracted modules: `why.rs` (`WhyReport` + `why_report_parts`) ·
  `world.rs` (`WhyWorld::analyze`/`analyze_measured` + the shared ship-body helpers; `WhyWorld` is
  retained from the same engine result rather than re-analyzing) · `kinds.rs` · `survival.rs` ·
  `results.rs` (the intake segment; `admit_controller_records` vs the fenced `admit_fixture_records`)
  · `fixpoint.rs` · `bundle.rs` · `source_comparison.rs`. It exists so `dorc-loom` can drive REAL
  invocations in-process; it is NEVER a public API — `publish = false`, nothing outside `dorc-loom`
  and the three bins may depend on it. VALUES cross the seam, QUERIES do not: the edge IMPLEMENTATIONS
  live in `compose.rs` parameterized by the bundle; if something elsewhere in the lib starts wanting
  a clock, a file, or an env READ, it is on the wrong side of the seam.
- **invocation-errors-are-registry-codes** (`288` §6) — the parsers return typed `Diag`s, never
  strings. The `dorc: ` / `dorc: lint: ` / `dorc-sh: ` prefixes and the usage synopsis are print-seat
  CHROME the three report seats own, never catalog prose. Exit codes are unchanged and never read
  severity. A new invocation error mints a code + a defining case like any other surface.
- **chrome-comes-from-the-registry** (`289:rul-arrangement-home-is-registry-plus-transcripts`) —
  the help page and the seat-appended usage synopsis are arrangement-registry entries
  (`dorc_cli::help_text` / `usage_text`), not consts; their words are edited through
  `crates/aid/tests/cli-help-page.loom`, never in source. A new user-facing chrome string mints a
  registry entry, not a `const`.
- **io-at-edges-only** — keep I/O in the composition module's edge implementations and the
  bins; the pipeline (`parse → cfg → classify → compile_probe/build_plan`) stays a total
  `Carrier<T>` function of its inputs; never let a clock/RNG/env-read leak inward "to help".
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
  planning, settlement, receipt publication, or host contact; executable placement and source-line
  replacement remain the post-`30L` artifact stage. Rider (r30): a `PlainInclusion` source
  (book-sourced, no dorc-lang marker) is mirrored BYTE-VERBATIM at its authored relative path
  — never stripped, never bundled, never renamed, never inlined into a single stream (the
  flattened form refuses by name), and its authored `.` is never re-said.
  `StaticLoadSnapshot::modelled_refs` is the ONE seat deciding which sources a lift sees; a
  new lift or index consumer takes it, a new PLACEMENT consumer takes the real bytes.
- **bundle-diagnostics-compose-occurrences-not-paths** (`30I` step 6) — production bundle
  validation diagnostics compose `LoadAccount` occurrence identity, `BundleFile` storage/copy
  identity, and the existing strip line map onto `aid::locator`; they never reconstruct an origin
  from path strings, comments, or source similarity. The ordinary authored diagnostic stays the
  primary frame while generated and nested-load frames remain visible. Comment-origin readback
  resolves to current source only on exact snapshot-byte agreement and stays aid-only either way.
  Static incoherence renders its located diagnostic but returns before archive stdout, preserving
  the existing refusal/exit. No locator value may enter loading, planning, or authority.
- **artifact-forms-derive-from-one-structure** (`30I:step-7-reify-plan-artifact-forms`,
  re-cut by `30Ng`'s human-typed rulings) — `cli::artifact` settles ONE `Selection` (form
  + fallback + dependency files + IMPORT EDITS) from authored-before-contact inputs, and
  `Selection::with_plan` binds it to the plan projection. The stdout stream and the
  published tree both READ that `ArtifactSet`; there is deliberately no second assembly
  of the same bytes to fall back to. WHICH STREAM carries the artifact derives from
  stdout INTERACTIVITY, injected at the edge (`30Ng:rul-piped-stdout-carries-a-full-plan`):
  a non-interactive stdout is a stream the user is KEEPING to review, so it carries a
  COMPLETE plan or the run refuses pre-network, under every flag-form — and naming
  `--artifact-dir` beside it claims one artifact twice, which refuses naming both
  claimants rather than ranking them (the loading design's collapsed-resource rule, one
  level up; this cell is a conductor derivation, veto-eligible). An interactive stdout
  carries the RENDER: with a directory the tree publishes there, without one `auto` may
  settle for a less flattened form and SAY SO. THE DEFAULT BUNDLES
  (`30Ng:rul-bundle-at-dorc-lang-boundaries`): each book-sited root at which dependencies
  become dorc-lang composes ONE generated file, and the generated plan's own import is
  RE-SAID to name it. That rewrite is the single edit Dorc reserves over a plan it
  generated — a generated plan is a durable but not an OFF-RAMP durable — and it is a
  first-class plan edit: an input to `Plan::decided`, disclosed as
  `plan-import-rewritten` on the plan surface, recorded as
  `RenderDecision::ImportRewritten` on the plane. The authored BOOK is never written and
  every byte of it that reaches the artifact reaches it verbatim (`two-surfaces`).
  BUNDLING IS POSITIONAL AND MEASURED-ONLY: a nested `.` is absorbed IN PLACE, so an
  include guard still decides whether the absorbed bytes run, and only at the shape
  `floor30-inline-dot-boundary` measured (a `.` that is the whole of its own line, and
  for a BOOK `.` also a top-level redirect-free command). Anything else stays a separate
  generated file and the authored `.` that names it survives — when in doubt, separate
  files. Front-HOISTING a bundle ahead of the book is the emission planner's first
  consumer (`plans/30P:the-emission-planner`): legal only under a PROVEN closed set — no
  book observation or mutation of a bundle-bound name above the `.` — never a settlement
  question; failing the set falls to in-place, never to a guess. Three cells sit under every
  book load line (r30; `30P:law-no-unsoundness-below-a-blind-act` +
  `30P:rul-rewrite-permission-is-derived`): `BookLoad::permits` (`LoadPermission`, minted from
  the analysis's own `FuncEnv::load_certainty` answer and `artifact::operand_is_explicit` — the
  cli explicitness seat, answering the same question as `funcenv::ResolvedHead::explicitness()`;
  unification owed) answers `may_rewrite()` = explicit ∧ EXACT and `may_ship()` = EXACT.
  EXACT∧explicit ⇒ re-point/paste + bundle · EXACT∧inexplicit ⇒ verbatim, MIRRORED at its
  authored relative path so the author's own operand finds it · ¬EXACT (below a `cd` of a
  non-`/…`/`./…`/`../…` operand or any blind act) ⇒ verbatim and NOTHING shipped for it, the
  `load-carriage-withheld-under-unknown-cwd` code naming the clobbering line; both placement
  seats (`bundle_files`, `mirrored_files`) read it. Separately, a book `.` carrying a LEADING ASSIGNMENT is not
  absorbable (`ImportEdit::Inline` replaces the whole command node, so `MODE=prod . ./entry.oracle.sh`
  would lose the assignment; `Repoint` moves only the operand and stays eligible — the
  assignment costs the FORM, not the bundling). `Selection::emission()` is what reaches the
  plan: the carriage account and the import edits as ONE value, because they are one answer
  about one form. Mirroring is stated against the LOAD CWD
  (`dorc_core::loadpath::Cwd::relativize`, the inverse of `resolve_operand`), never
  against a stored path's own spelling: every source a book `.` reaches is filed under
  its CANONICAL key, which is ABSOLUTE whenever the edge could answer where the run
  stands, so a seat asking whether the stored spelling looked relative answered
  "unplaceable" for every real invocation while every in-process test said the opposite
  (`30Nf:fnd-multipart-never-placed-anything-in-production`). A dependency OUTSIDE the
  load cwd is unplaceable rather than fudged (`need-controller-paths-never-cross-hosts`).
  BOTH ENDS OF THE BUNDLE-POINT AXIS stay reachable by name — one emission
  (`flattened`) and none at all (`mirrored-tree`) — with the default at neither. Every
  form name and file name here is STRAWMAN and renames in place
  (`rul-strawman-formats-no-compat`); what is ruled is the axis, the stream semantics,
  and the rewrite's scope.
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
  artifact rendering, or receipt publication, and emits no plan carrying mutation
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
- **source-comparison-is-one-cli-seat** (opaque-ruled 2026-08-31;
  `30Va:rul-source-comparison-is-one-cli-seat`) — recorded-source comparison and every
  filesystem behavior it needs live at ONE seat (`cli/src/source_comparison.rs`), fed only by
  `receipt`'s `visit_for_comparison` packet. The seat owns platform path rehydration, the
  bounded non-following regular-file read, current/recorded correspondence policy (exact
  path first, exact content second), and destination encoding — its `FilesystemBytes`
  encoder is a second DESTINATION, never a display seat. Authentication asymmetry: a
  receipt-provided path triggers an implicit read only for a LOCALLY-AUTHENTICATED receipt;
  imported/self-asserted material compares only against a file the user named. Future
  comparison features extend THIS seat — never a new receipt exit, never a scattered raw
  path read. The seat takes an injectable current-source reader for the in-process world (the
  shipped binary passes `None`; the read, its bounds, non-following, and the authentication
  asymmetry are unchanged) — the sanctioned in-place extension, review-eligible.
- **inv-receipt-collection-never-expands-observation** (née inv-whylog-collection-never-expands-observation) — receipt writing persists only data the
  invocation already holds; it performs no additional host call, environment sweep, unrelated
  controller read, or debug probe. Later pull is a separate invocation about the later world
  (`30R:standing-invariants`).
- **inv-unaccounted-output-stays-remote-by-default** — stdout/stderr outside the
  oracle-accounted channels is not transported or persisted by default. Explicit collection is
  selected before transport; later debugging pull is temporally distinct
  (`30R:accounted-and-unaccounted-output`).
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
  plan, narrative, render, receipt write, and `report_at` sits outside it. The sole deliberate
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
  Oracle-only is still right for the receipt/attempt-scope record of what was LOADED; the
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
- **display-paths-relativize-at-one-seat** (`30Xa` lanes B1/B2a/D2b; human-acked 2026-09-04) — a
  controller path shown to the user renders RELATIVE to the load cwd when it lies under it, at ONE
  seat (`why::relativize_for_display` over `Cwd::relativize`): the why-lens's `.`-sourced dependency
  paths (as `--pre-source` oracle paths already rendered) and the contested-helper diagnostic's
  locus. Canonical keys stay ABSOLUTE (`need-controller-paths-never-cross-hosts` governs placement,
  not display). Never a post-hoc normalizer: both-streams transcripts are what made the absolute
  paths visible, and determinism is made at the source or not at all.
- **apply-outcome-unwritten-is-a-sibling-code** (`30Xa:rul-post-dispatch-durable-failure-is-a-sibling-code`;
  human-acked 2026-09-04) — an apply that DISPATCHED (intent published; the machine perhaps changed)
  but could not record its outcome fires `apply-outcome-unwritten`, a SIBLING of the plan-time
  `durable-receipt-unwritten`, never a reason arm of it (`AID-NEEDS:law-codes-vary-by-world-not-grammar`:
  the worlds and the repairs differ — re-plan, versus check the host and keep the intent id); WHICH
  write step failed is a typed reason within the one code, and `ApplyWriteStep::word()` in `aid` is
  the ONE home of the write-step words (the pre-dispatch surface converts `receipt::DurableFailure`
  → `ApplyWriteStep`). The exit code stays the apply's: the record failed, the apply happened. The
  identities chrome line (`cli-apply-identities-line`) renders whenever BOTH documents were recorded,
  transport status regardless. ONE apply implementation, `compose::dispatch_and_report_apply` over
  `&mut dyn LocalIo`, serves production (`NativeIo` + disk) and the loom (the session store + a
  scripted `hosts/<name>/apply-outcome` section).
- **speculate-and-intercept** — the probe model resolves probe-gated branches by
  running the read-only check for real (oracles intercept; not Ansible
  check-mode blindness).

## The acceptance harness (`tests/e2e.rs`, the ONE runner; this contract is law — `notes/30X`)

- **one-runner-one-walk** (`30X:loom-one-runner`) — `tests/e2e.rs` is the ONE `harness = false`
  runner: one walk of every `crates/*/tests`, one named, filterable trial per case, over BOTH
  populations — looms, and the Rust batteries' fixture dirs beside them (`autotests = false` +
  explicit `[[test]]` targets is what lets case DATA share the dir). The discovery floor guards both
  populations (zero trials exits RED); **count-drifts**: never pin a count. Corpus walks skip
  `*.sync-conflict-*`.
- **a-case-is-a-shell-session** (`30X` §5) — a loom's replay is a POSIX shell session: its `$` lines
  run in ONE persistent `sh` (`dorc_transport::Posix::find`, `one-shell-answer`) in the materialized
  dir, `dorc` on a shim PATH resolving to `dorc-harness` (**loom-shim-is-a-hard-link**: one link per
  runner process, unix symlink then copy as cross-volume fallbacks — a copy per process is how the
  2026-09-03 disk-full happened), sentinel-framed, BOTH streams captured in the order the user saw
  them, each block compared to its committed block under `strip_trailing_newlines` and nothing else.
  `export`, `cd`, `<`, `> <literal file>` + `cat`, `echo $?` are native shell; the artifact-execution
  rail (`PATH=<mocks>` only, `env -i`, a throwaway cwd, `umask 022`) stays a separate runner-owned
  process. The session starts SCRUBBED (`30X:loom-syntax-grants-no-production-authority`): no
  inherited credential variables, runner-owned roots — `DORC_SEAM_ROOTS=pinned:<absolute literal>`
  (**rul-roots-pinned-is-a-literal**: never resolved through `APPDATA`/`HOME`/`XDG_*`; absent ⇒ the
  harness refuses) — while the runner's own process keeps reading its opt-in lane variables.
- **rul-runner-varies-only-what-it-set** — the runner's per-block injection yields to any authored
  assignment: an authored `$ export DORC_SEAM_CLOCK=…` / `DORC_SEED=…` wins for the rest of the
  session. The runner-owned defaults are ONE seat, `dorc_loom::runner_seams` (`RUN_SEED`,
  `SESSION_ROOT`, `session_seams(ordinal)`, `DORC_SEAM_TRANSPORT=local:<shell>;<interp>`), consumed by
  the shell driver (exported into the session) and the in-process driver (fed to
  `HarnessSeams::from_env` over the MODELLED environment) — never a second copy. The clock ticks one
  day per INVOCATION block (`fold_clock_seed(seed, ordinal)`; exports, `cd`, `echo $?`, `cat` do not
  tick; the shell's shadow line preserves `$?` across the injection).
- **session-stdin-is-the-framed-stream** — a `--results -` block reads FRAMED records from the
  session's fd 0. The runner frames RAW fixture records into the controller wire format at ONE seat
  both drivers consume (`dorc_loom::records_framing`; a `dorc-records/`-headed fixture passes verbatim
  on both — **rul-framing-is-one-rule-both-drivers**; a fixture that then fails in the shell path is
  rot, fixed as a fixture), admits them through the CONTROLLER intake with the seam's attempt nonce
  (**rul-in-process-sessions-take-the-controller-intake**, for EVERY session line), and restores them
  raw for gate-1. The fixture-intake seat (`Framing::spike`) is KEPT for its one non-session user,
  behind the `fixture_intake_is_unreachable_from_production` fence.
- **rul-gates-attach-to-what-a-block-produced** (`30X:loom-gates-attach-by-kind`) — the runner hands
  each `$ dorc …` line to the product's own arg parser (**block-argv-classifier-is-read-only**:
  classification never drives). The artifact battery — dash `-n` on BOTH artifacts (the load-bearing
  runnability gate; the historical trap was a text-only golden shipping a non-runnable empty
  `then`-clause green, twice), the redirect scan, guard-shape, argv-echo, dual-rail (derived from the
  book's own multiline argv), the artifact-set generation rule — runs iff the block's `Mode` emits an
  executable artifact AND its stdout is one (non-empty, shebang-led, the probe/apply pair split):
  **rul-errored-plan-has-no-artifact-gate**; the crash guard is "an artifact-producing mode exited 0
  with empty stdout"; exec-under-mocks iff the case carries `mocks/` (sorted run-set asserted against
  the `expected.ran` section, which MUST exist; a mocks case the shell declines is a loud authoring
  refusal); the exec rail's expected exit is the `apply-exit` key. A loom's diagnostic assertion IS
  the session compare plus `defined_code_fired`
  (**rul-loom-diagnostic-assertion-is-the-session-compare**; `transcript_slugs_are_catalog`
  validates every `error[<slug>]` header in a transcript against the catalog — the honest form of the
  retired needle keys). Gates re-drive their subject block with split streams for parsing; a re-drive
  that would publish roots in a THROWAWAY store, never the session's own
  (**inspection-redrives-carry-no-durable** — the same for `dorc-loom publish`/`vars`/`sections`).
- **an-artifact-set-runs-from-its-own-generation** (`30Nf` §4) — an artifact-producing block carrying
  `--artifact-dir` has its exec gates run the PUBLISHED `<generation>/plan.sh` from inside that
  generation — the cwd the multipart execution contract gives an artifact. Exactly one generation is
  required; the published plan is asserted byte-equal to the apply block on stdout; a published
  plan's own LITERAL relative imports must resolve inside the generation
  (`unresolved_generated_imports`). Copying a case's AUTHORED sources into the sandbox is the refused
  alternative (a case minted to DEMONSTRATE a capability must OBSERVE it —
  `30Nf:fnd-multipart-never-placed-anything-in-production` is the burn). The counterfactual rails
  (gate-5/gate-6) compare an artifact against the BOOK and run in the generation laid over a copy of
  the case's top-level files; `exec_check` runs the published plan from the generation ALONE. The
  runner's posture seam is `pinned:interactive` (the round-trip battery READS a render while
  driving the artifact set to a directory — the terminal cell); the kept-stream cell's pre-network
  refusals render on stderr and are ordinary session transcripts now.
- **rul-drivers-decline-symmetrically** (`30X:loom-driver-is-derived-and-reported`) — which driver
  proves a session is DERIVED from its content and REPORTED in the trial (`[proven by: shell+in-process]`
  / `[…; in-process declined: <reason>]` / `[…; shell declined: <reason>]`), never declared. Each driver
  owns a typed decline set, ONE vocabulary the runner reads (**rul-decline-sets-are-one-vocabulary**):
  in-process `dorc_loom::LoomDecline` (RemoteApply · ApplyWithPlan · OracleDirs · ExplicitReceiptFile ·
  ReceiptsOverride · ShellOrExternal · Unexpressible · a non-root `cd`), shell `ShellDecline` (an
  `edge-fault` section, a `dorc-loom` block, a `dorc-sh` block — the shim provides only `dorc`). A
  session is proven by every driver that does not decline it; BOTH ⇒ they must agree byte for byte
  (**gate-two-drivers-agree**; the shell proof is authoritative where it runs); NEITHER ⇒ the runner
  refuses (`PROVEN_NEITHER`). A disagreement is a FIDELITY finding, fixed in `dorc-loom` or the shared
  cli seats so both agree — never by changing a product crate's semantics (a STOP), never a normalizer,
  never a golden edit. The in-process expressible set only SHRINKS its declines and may never regress
  below what errorloom's outer grammar admits (`30X` §9); no roster or meta-test polices it.
- **the-in-process-world-is-the-production-edge-over-models** (`dorc-replay-is-production-semantics`)
  — the in-process driver composes the REAL edges over deterministic models, one session = one world:
  the real `LocalReceiptEdgeV1` over a platform-shaped `ModelIo` store (born empty, dies with the
  session — never a cache), reached through `dorc_cli::durable`'s re-exports (`dorc-loom` never names
  `dorc_receipt_local`; both human-ack rosters untouched — route B); ONE receipt-edge implementation
  in `compose.rs` (`publish_rooted_receipt` / `read_rooted_receipt` / `dispatch_and_report_apply` over
  `&mut dyn LocalIo`) that `ProductionEdges` calls with `NativeIo` and the loom edges with the
  session's store; the `$` lines read through `dorc_syntax` (`dorc_loom::session_grammar`, cross-checked
  against errorloom's outer gate — a disagreement declines); a CONCRETE dash environment
  (`session_env`; its exported subset is what `HarnessSeams::from_env` reads — the kernel's own
  environment is three kernel-arc changes away, `30X:front-dogfood-ceiling`); the kernel's own `Cwd`;
  the stdout posture from the SEAM, never a block's `> /dev/null`
  (**in-process-posture-reads-the-seam**); the dependency walk as the edge's own BFS parameterized by
  byte source — disk for the binary, sections in-process (**in-process-snapshot-mirrors-source-acquisition**;
  `$0`-relative `.` operands through the binary's own `ScriptSpellings` — **book-reached-resolves-dollar-zero-loads**);
  the source-comparison seat's injectable current-source reader (the binary passes `None`). A store the
  session cannot reach is a DECLARED `edge-fault` (**rul-rootless-worlds-are-declared-faults**;
  `EdgeFault::ReceiptRead`), never a history flag. The transport seam's `Scripted` column is
  constructible ONLY through `HarnessSeams::with_scripted_transport`
  (**rul-scripted-column-is-a-fenced-seam-variant**; the case section `hosts/<name>/apply-outcome`,
  strawman), driving the DST `SimDriver` through the same marker scan the real drivers use.
- **thirteen-keys-by-criterion** (`30X:loom-frontmatter-is-registry-metadata-only`;
  **rul-survivors-are-the-criterion-not-the-count**) — `dorc_loom::FRONTMATTER_KEYS` is ONE list of
  thirteen: `code` · `arrangement` · `when-fires` · `when-used` · `why` · `owns` · `todo` · `envelope` ·
  `tests-critical-law` · `probe-results` (`authored` only — the gate-1 opt-out for hand-authored
  records) · `tolerate` (**tolerate-is-a-closed-vocabulary**: a declared nondeterminism class from an
  engine-owned vocabulary, normalized in the RUN LOG at bless and at check; current: `pipe-stage-order`;
  a runner-read `export` would be sidecar config in an sh costume — `KNOBS:kOOB`) · `apply-exit`
  (default 0) · `xfail: <pin-slug>` (**rul-xfail-is-a-registry-keyed-key**: names a pin in
  `dorc_testbed::xfail::PINS`; structural gates tolerated and reported, the transcript ENFORCED, XPASS
  loud; a target-tense golden has no loom home — **xfail-in-loom-needs-a-structural-failure**). A key
  survives iff it is about the case as an AUTHORING HOME or is a case-level declaration no honest
  session line can carry; run knobs live in the session (**loom-run-knobs-live-in-the-session**: flags
  and `--artifact-dir` on the `$ dorc` line, exits as `$ echo $?`, records as `<`). `dorc-loom keys`
  prints the set; a new key joins in the same commit that mints it — after stopping at a conductor
  (`30X:rul-loom-escape-stops-at-the-conductor`).
- **seeds-vary-and-pins-are-sh** (`30X` §6) — every run draws a fresh seed (`dorc_testbed::run_seed`:
  `DORC_SEED` from the runner's environment, else one OS draw per process, memoized), printed once at
  start and on every FAIL line with the replay and pin spellings; a case pins with
  `$ export DORC_SEED=<n>` (seed 0 is the fold's identity for dates; one spelling across every tier).
  The e2e bless and `dorc-loom publish` REFUSE a candidate that does not reproduce under a derived
  second seed, naming the first differing block and the pin remedy.
- **bless-never-first** — `BLESS=1` regenerates transcripts; gates run before bless, but bless cannot
  prove an elision RIGHT: fresh verified binary, orchestrator-only, diff inspected case-by-case (BLESS
  exclusivity — `spike/CLAUDE.md`).
- **bless-writes-renders-not-measurements** (`spike/CLAUDE.md` emitted-is-measure-once-ground-truth) —
  bless has authority over what the ENGINE produced (transcripts, `expected.ran` sections) and none
  over what the floor BINARIES produced: a case carrying `expected.emitted` is REFUSED by `BLESS=1`
  through the one pure `floor_bless_refusal` seat, and is writable only under the `BLESS_FLOOR=1` +
  `DORC_E2E_FLOOR_SHELLS` mint, where gate-9 re-measures and the manifest folds in the same write
  that commits the transcript. Never widen the fold to a section no gate just re-derived.
- **bless-folds-only-on-pass** — a case's transcript folds only when its own gates PASSED; every gate
  comparing against a bless-written section is bless-aware, so what stays reachable under bless is
  structural, authored-fixture, or environmental. XFAIL folds nothing (its lens tolerates structure and
  enforces the transcript). Pinned by `bless_folds_only_on_pass_selftest`.
- **empty-ran-has-two-stable-spellings** — an empty `expected.ran` exists as ONE blank line (`""`) and
  as TWO (`"\n"`), both fixpoints, neither drift: `exec_check` writes `format!("{got_ran}\n")` for a
  case carrying `mocks/`, and a case without mocks never reaches it. Normalizing either spelling churns
  goldens for nothing — the gate compares under `strip_trailing_newlines`.
- **floor-cases-see-a-modelled-dollar-zero** — a floor case's manifest is a top-level file, and no
  manifest cell may observe `$0`: the rail's `$0` SPELLING is platform-bound
  (`30P:gap-dollar-zero-shape-is-platform-bound`), which is why the engine models both live spellings
  from the authored book path (`ScriptSpellings`) and invokes what it ships in a spelling it modelled
  as live. A case that wants the resolution observed does it through a diagnostic or a ship.
- **receipt-state-is-the-one-state-only-home** (`30X` §3 shape (c), §3a) — `tests/receipt_state.rs`
  is the ONE Rust battery asserting STATE, EXITS, STRUCTURE and RELATIONS over the receipt store —
  never a render golden (goldens are looms; typed internal decisions stay pipeline-tier). It spawns the
  seeded harness and keeps the shipped-binary liveness witness
  (`the_shipped_binary_draws_live_os_identities_and_ignores_harness_seams` —
  `30X:inv-division-at-the-narrowest-edge`'s bounded remit). A durable failure no shell can reproduce
  is witnessed in-process over `ModelIo` + a `FailureSchedule`
  (**rul-unshell-reproducible-failures-are-in-process-only**), the failing write step DISCOVERED by a
  throwaway intact drive, never an op-count constant
  (**rul-outcome-fault-occurrence-is-discovered-not-constant**).
- **runner-reap-at-exit-not-drop** — `libtest_mimic::run(...).exit()` and preflight's `process::exit`
  skip `Drop`, so every temp the runner mints (the shim dir, the profile parent) is reaped by an
  idempotent `Harness::reap()` called at every exit path, `Drop` included; net-zero temp entries across
  the whole suite on both legs is the measured floor.
- **lint-looms-are-ordinary-sessions** — `$ dorc lint …` blocks drive through the harness like every
  other block (`run_lint`, `E2eKind::Lint`, and the `X/cmd` shape are gone); the binary's operational
  lint error renders through the same staged-parts seat as every invocation error and carries
  `error[<slug>]`. The two `lint-real-*` dirs are the real-tools test's own fixture space
  (**real-tools-owns-its-fixtures**), not corpus cases.
- **the-census-tier-reads-the-loom-corpus** (`30X:tier-census`) — the "for every case …" batteries
  (`region_artifacts.rs`, `definition_frames.rs`, `sh_parity`'s munge-witness census) stay Rust and read
  their inputs from the loom corpus through errorloom's `Case::parse` (the apply artifact from the
  transcript, the book from its section); the munge-witness roster keys case STEMS. A per-case golden
  cannot say "for all cases".

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
