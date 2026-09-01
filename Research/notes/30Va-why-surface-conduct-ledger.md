# 30Va — why-surface refresh: conduct ledger

> The r30 receipt arc's engineering close, plus the receipt-backed `dorc why`
> presentation work rehomed out of the quarantined phase. Conductor: Fable-class
> (quarantine-blind by law; builders read `AGENTS.for-builders-only.md` first).
> The reserved design-doc slot for anything directional this arc settles is 30V
> (plans-tier, minted late, only if refinement produces directional rulings).
> This ledger is state, not history; git carries chronology.

## Remit

1. Transition residue of the durable-receipt family (report-API tidy · inert
   legacy-whylog deletion + singular-implementation census · CLI
   vocabulary/help) — builder lane on `ai/r30-receipt`; the builder sites its
   own ledger at the lowest-unused 30R docID.
2. Receipt-backed `dorc why` arrangement/render + replacement user-facing
   cases over the sealed `dorc-receipt::report::RecordedWhyFacts` model —
   built TOWARD a conductor-authored plaintext spec (human-directed
   2026-08-30: the UX must not fall out of in-flight builder direction).
   The spec: `notes/30Vb` (the strawman surface set).
3. Arc close: `gate:arc` from the populated branch · LIVING_STATUS refresh
   (several entries stale: the "nothing upstreamed" line, the
   loom-production-path in-flight entry, the reaped branch inventory) ·
   worktree/branch cleanup.

Out of scope, by direction: everything security-shaped (addressed, closed);
report-only kernel re-derivation (separate authorized round; kernel frozen);
loaded-source locator composition (named uncollected V1 residue).

## State

- Worktree/branch cleanup: DONE (sonnet lane, 2026-08-30) — 9 worktrees
  removed, 12 dead branches deleted, 76 dead-husk directories dumped
  (human-authorized). The five survivors were then triaged (opus lane, same
  day): ALL-DIE, per-commit supersession evidence below; nothing salvaged;
  `ai/r30-salvage-sweep` minted empty and deleted. Deleting the five is the
  human's act (`-D` is hook-blocked for agents): `ai/r30-lane-fruit` ·
  `ai/r30-lane-fruit-2` · `ai/r30-lane-load` · `review-verify-adv` ·
  `review-verify-neutral`. The `30N` §4 hold on `review-verify-adv` is
  discharged and annotated in place.

### banked-branch-triage-dispositions (2026-08-30; every unique commit → DIE)

- `85808626` brace-range catalog entry → `DiagCode::ForLoopBraceRangeRunsOnce`
  + published lock + loom + lint case in main, which also absorbed the lane's
  own later oracle-shape fix and re-derived the lock.
- `73f34fce` zero-site-oracle warning → `DiagCode::OracleMatchedZeroSites` +
  loom + a whole-product round-trip case the lane never had.
- `f32ae2a3` lock re-publish → subsumed by the above re-derivation.
- `461b220a` paste-hygiene diagnostic → `DiagCode::EmittedLineUnsafeForPaste`
  + `plan::render::paste_hygiene_hazards` + cases, all in main.
- `3b2e72d4` five red pins (unknown-source/`$0`/computed operand) → all five
  PROMOTED in main (behavior built, tests green, each marked "né p-x-…,
  promoted").
- `176e0818` sentinel-literal drift demo → landed green with a corrected
  expectation (see the `30N` §4 annotation).
- `1dbca1ab` discarded certifier trips → built + fenced
  (`plan/CLAUDE.md certifier-trip-cleanup-runs-in-every-driver`, `TripSpent`
  witness, lexical producer roster).
- `5e614861` lost sourced assignments demo → same test in main, correctly
  re-housed under `xfail_until("p-x-book-level-dot-locals", …)`.
- `c304dc99` replayed pre-source authority demo → green in main, citing
  `30Mc:finding-transitive-pre-source-replays-as-root`.
- `d3388076` review synthesis ledger → strict subset of `notes/30Mc`, which
  carries citations/worlds/slugs the orphan lacks.
- Residue lane: COMPLETE — `ai/r30-receipt` at `4414af7a` (14 commits,
  +729/−4971), `mise run both gate:full-quiet` green both legs, tree clean,
  builder ledger `notes/30Rk`. Old whylog fully cut over (census: no live
  code, no loom replay, no flag table); report-API tidy landed; receipt flag
  family mechanics + selector-exclusion tests landed; the fence turned out
  never to apply (the mode field reaches no durable — breakpoint claim
  corrected in `30Rk`) and the acked rename landed as `Unstated` anyway.
- Standing rulings pending (human): the root-identity fold into closure and
  the ancestors-only closure walk (both stand as built); whether the now
  dead-ended `SpineInvocation::mode()` (zero callers) should die outright.
- Steering/register recast: LANDED 2026-08-31 (`b3552abd`/`99ebe1b1`/`b44bd5e0`).
  Verification first (the human's suspicion held): the `30Rj`-flagged
  receipt/receipt-local rows and `plan/CLAUDE.md`'s durable-replay section were
  ALREADY current — the genuinely-stale set was `spike/CLAUDE.md` (five spots),
  `core/CLAUDE.md` (the DurableAccount flattening cite), `cli/CLAUDE.md` (five
  spots), `core/src/spine.rs`'s module/`CensusArm`/`ExcludedContent` docs, and
  `plan/src/spine.rs`'s untracked-inventory doc (a dead `plan/src/whylog.rs`
  bullet + a stale "list is empty" sentence). All recast to receipt vocabulary;
  renames carry née tags (`receipt-durable-write-only-report-back` ·
  `receipts-not-a-cache` · `law-receipts-are-sensitive` · `aid-receipt-posthoc-why`
  · `an-receipt-durable` · `inv-receipt-collection-never-expands-observation`).
  The account-export expectation is re-homed in place: excluded-by-absence again,
  pin `p-x-durable-account-export-is-enabled` parked `Reserved`, rebuild =
  receipt-contents change, review-first.
- Fold-time queue (remaining): cli/src doc-comment citers of the née'd slugs
  (`cli/src/lib.rs:536` · `cli/src/results.rs:291`, plus the stale
  `whylog_store`-as-reference comments in `cli/src/artifact_store.rs` and kin —
  deferred while the render lane owns that tree) · prose massage (five unwritten
  receipt-flag help rows; two receipt placeholders; six stale `why:` metadata
  citations in the generated locks; slop-tier ACKED) · LIVING_STATUS final
  refresh · `gate:arc` · end-of-work review per root `AGENTS.md` · worktree
  cleanup (`r30-receipt`/`r30-conduct` now trail `ai/main`).
- Why-surface design: exploration CLOSED and banked — `notes/30V` (the
  re-exploration record: model, doctrine, rulings, §5 build direction,
  §6 owed-list) + `notes/30Vc` (four adjudicated render generations, raw);
  `30Vb` is gen-1, historical. Build-now per `30V` §5: the reconstruction
  plane + an INTENTIONALLY-TEMPORARY total surface (no drawn gutter, interim
  register, replaced without ceremony later); the graph library, settled
  register, and curated tiers deferred to proper process and attention.
- Why-surface build lane: DISPATCHED 2026-08-31 — Opus builder, branch
  `ai/r30-why-surface` (base `f6317f43`, deliberately behind `ai/main`; fold is
  conductor-side); builds `30V` §5 + the arrangement-orphan census rider +
  replacement receipt-rooted why cases; completion contract
  `mise run both gate:full-quiet`; builder ledger sites at lowest-unused
  30V-suffix. The structure checkpoint was adjudicated same-day: new pure crate
  `dorc-why` (edges → receipt + aid only; render/`--json` at the cli edge);
  the `Known`/`Held` wrapper pair; listing-surface retirement RULED (all four
  listing seats die); `Rooted::{Plan, OtherSpecies}` one-face RULED;
  `--receipt <file>` roots-a-question repair in scope; the `.rs` whole-product
  battery substitutes for a loom case (the loom `run:` vocabulary cannot drive
  a store-side route — a widening is a queued TODO row, not this lane).
  Builder finding of record: the receipt durable RECORDS fifteen row families
  but sealed `report::RecordedWhyFacts` PROJECTS seven — most of the
  durable-gap audit is a report-API gap, not a carrier gap
  (`fnd-report-api-carries-seven-of-fifteen`, its ledger).
- rul-report-projection-becomes-exhaustive (OPAQUE-RULED, relayed by the human
  2026-08-31; resolves `tc-model-families-reach-the-reconstruction`). The
  packet, verbatim (its `_tmp-extension-remit.md` carrier is ephemeral):
  "The correct repair is not more persistence, format work, or kernel work.
  Extend RecordedWhyFacts with typed report projections for the existing row
  families: closed recorded tokens remain typed; influence/material
  availability stays independent; opaque operands, shell, locators, and
  details use the existing RecordedValue encoder exit; no raw
  RecordedPlanReceipt or overlay accessor is exposed. Ideally the report
  projection becomes exhaustive over the durable model: every persisted family
  is either represented by a typed facts collection or explicitly classified
  as unavailable/not relevant. That gives 30V and later aid work one complete
  public read surface without reopening receipt internals." Relayed to the
  render lane with scope interpretation: read-surface projection work only
  (no grammar/writer/wire/projection-state/provider change); the fifteen plan
  families are the core mandate, intent/outcome rows in-scope where the
  `Rooted` face needs them; a closed no-wildcard family classification so an
  unclassified sixteenth cannot land; any need to reopen internals is a STOP.
  The cli-side decomposition route stays forbidden.
- Render lane, second builder (2026-08-31, wound down at budget, endorsed):
  `712ab177..8646a4f6` — ALL fifteen plan families projected
  (`FamilyCoverage::RecordedButUnprojected` unreachable except by removing a
  projection); `cli::why_total` (Coverage at the emit site; exclusion ledger
  empty by uninhabited type; weft-woven); `why_json`
  (`dorc-why-json/unstable`; state+value keys, never absent); 37 arrangement
  rows seeded unwritten; both legs green at tip. UNBUILT then, spec sharpened
  in `30Vd:what-remains`, since BUILT by the third builder: item 4 (cli wiring: facts_for production caller,
  listing-seat deletion, `ReceiptRoot::File` repair, whole-store-listing drop,
  digest-matched address resolution) + item 5 (publish→why e2e cases + the
  minimal runner extension). New flags: `tc-licensor-custody-speaks-its-own-act`
  (lane-ledger-minted; CONDUCTOR-RULED: custody→act mapping stands — engine-voice `Derived` would
  dress an authored judgment as derivation; refinement owed in the wiring lane:
  `vouched-severally` populates the voice-set as an inseparable committee of
  unnamed voices, not a single unnamed voice) ·
  `tc-nonplan-root-depth` re-sharpened (CONDUCTOR-RULED: shallow fill of
  `NonPlanRoot` from the already-public intent/outcome accessors now; deeper
  intent/outcome family projection deferred with a `NotRelevant`-tier
  classification and a flag — it would need new sealed accessors, a fresh act
  under the same law, not this arc's).
- rul-source-comparison-is-one-cli-seat (OPAQUE-RULED, relayed by the human
  2026-08-31; resolves the file-line address question — the
  `_tmp-recorded-source-path-handoff.md` carrier is ephemeral). Packet,
  verbatim: "dorc-receipt validates/classifies recorded source material, then
  releases one narrowly typed source-comparison packet to a single CLI seat.
  Comparison and filesystem behavior live outside receipt. Receipt API:
  RecordedSourceMaterial::visit_for_comparison(SourceComparisonConsumer). The
  visit carries exact recorded path bytes, source identity/digest, exact
  recorded general-sh content where held, locator spans, and
  authentication/material state. This is source-specific — not a generic raw
  RecordedValue accessor — and none of these values gain Display, revealing
  Debug, serde, or ambient formatting. One production consumer in CLI owns:
  platform-aware path rehydration; bounded, non-following regular-file reads;
  exact current/recorded path and content comparison; current
  same-physical-line policy; future diff/content-aware correspondence
  policies; destination encoding for anything displayed. The consumer must
  distinguish locally authenticated receipts from imported/self-asserted
  material. Receipt-provided paths never trigger implicit reads from the
  latter; an explicit user-named file may still be compared. Mechanically
  enumerate the one implementation and callsite as hygiene. Do not move
  filesystem I/O or matching policy into receipt, and do not widen arbitrary
  opaque-detail access. Future comparison features extend this one CLI seat
  rather than adding receipt APIs or scattered raw path exits." Relayed
  mid-lane to the wiring builder with: `file.sh:N` now BUILDS (path bytes +
  line computed from spans over exact recorded content, at the seat,
  display through destination encoding; ordinal-and-span stays the fallback
  where the visit holds no content); the authentication asymmetry as stated;
  the hygiene enumeration structural-first, lexical only as the
  packet-requested (hence human-ack'd) exception.
- Render lane, third builder (2026-08-31): COMPLETE — items 4+5 + both
  packets + the orphan instrument, eight commits, both legs green at tip. The
  lane is BUILT in full; rebased onto `ai/main` for the fold. Conductor
  adjudications of its disclosures: `--json`-beside-`--results` refusal
  ENDORSED (a silently-ineffective flag is exactly what the human would flag)
  · ambiguity-explains-nothing ENDORSED (30R's no-arbitrary-tie-break, a
  disclosed narrowing) · signing-key-off-the-surface ENDORSED (skeleton
  material, not report material) · `tc-correspondence-falls-back-to-content`
  ENDORSED (seat-owned policy per the comparison packet) ·
  `tc-file-root-order-comes-from-the-name` STANDS-AS-BUILT with a banked
  lean: the honest end-state is probably a typed order accessor on the read
  surface (projection of a persisted closed token, within the
  exhaustive-or-classified law) — a later small act, not this arc's ·
  `tc-address-refusal-is-a-datum-not-a-diagnostic` HELD FOR THE HUMAN
  (conductor recommendation: a NO-MATCH address is an answer, not ambiguity —
  the datum-row satisfies 30R's render-the-rest; the `[TYPED]`
  fails-fast-on-ambiguity rule should bind the MULTI-match case, which should
  stop-and-ask; split accordingly if the human agrees). Residue rows for
  TODO-ADDTL at close: the unwitnessed authentication-asymmetry and
  non-following-read branches (no fixture world reaches them) · the
  unwitnessed `vouched-severally` committee arm · the loom `run:` vocabulary
  widening (deferred) · the five orphan arrangement rows awaiting the human's
  deletion word (`syntax-unsupported-source-of-dynamic-target` ·
  `why-drift-address-unanswerable` · `why-drift-analysis-suppressed` ·
  `why-receipt-plan-tally-unsplit` · `why-receipt-when-replayed`).
- Conductor prose pass (2026-08-31, slop-tier per ack, on the lane): the six
  receipt/machine-register help rows, both receipt-code messages
  (`{{store}}`/`{{reason}}`-holed), three register-guide `why:` recasts (the
  dead "whylog is a deterministic reproducer" guidance now points at the
  published receipt), the dorc-loom usage example slug, and the two usage
  transcripts refreshed via the dump-rescue loop. Steering sited on the lane:
  `cli/CLAUDE.md source-comparison-is-one-cli-seat` ·
  `why/CLAUDE.md file-line-addresses-come-from-the-visit`. NOT authored: the
  37 `why-total-*` label rows — structural finding: their only rendering
  surface carries per-run entropy (receipt ids), so no committed transcript
  can be their fixpoint-authoring home and the loom words-mint cannot reach
  them; they rest `[unwritten:]` legally, and the clean future fix is a
  synthetic-world loom case rendering `why_total` over an entropy-free
  reconstruction (a later sitting's machinery). Two lock `why:` rows keep
  historical fence-names by their old spellings deliberately
  (`nack-whylog-stores-book-bytes`, the 27V clock cite) — history, not drift.
- Worktree-reap incident (2026-08-31, resolved; the `worktree-file-access-law`
  hazard fired as documented): the harness reaped the builder's managed
  worktree at its checkpoint turn-boundary; the orphaned skeleton's dead `.git`
  pointer made bare git answer FROM THE PRIMARY. The builder's step-zero
  discipline held — read-only commands only, stopped and reported; primary
  verified clean; zero lane commits existed so nothing was lost. Resolution:
  conductor-minted persistent worktree `.claude/worktrees/why-surface` (not
  harness-managed ⇒ immune to return-triggered reaping), residue verified
  zero-file and deleted. Lesson already durable in steering; no new law.
- Cleanup queue addendum: `.claude/worktrees/` still holds ~36 `agent-*` husks
  and ~14 named directories that are NOT registered worktrees (`git worktree
  list` knows only play/r30-conduct/r30-receipt/why-surface) — presumably the
  same empty-husk class the 2026-08-30 sweep dumped. Verify zero-file, then
  dump at arc close.

## Close (2026-08-31)

The arc is CLOSED: `gate:arc` green (rc=0; both-platform completion + arc-tier
verifiers + advisories) from the populated lane, then `ai/main` fast-forwarded
to the lane tip (`ea295267`). One post-gate red was caught and fixed in-lane
(the six worded help rows joined `cli-help-page.loom`'s `owns:`). The
opaque-review gate stayed builder-initiated: all three builders read
`AGENTS.for-builders-only.md` first and none instructed loading it. Remaining
human items ride `TODO-ADDTL:why-surface-close-residue` + the round-close
ceremony; worktrees/branches cleaned at close.

## HANDOFF — the test-architecture rebuild

The arc's PRODUCT is sound and folded; its test architecture is rebuilt rather than patched, in a
separate non-concurrent suite arc. The design statement, priorities, and the build-planning tail a
successor conductor starts from are **`notes/30X`** (seams · sessions · seeds). Nothing was built
under the earlier handoff that stood here; the human rewinds before pursuing.

## Ack-ledger (only what the human has TYPED counts)

- ack-cleanup-authorized (2026-08-30): dead branches/worktrees deleted, husks
  dumped unceremoniously; human `-D`'d the two dangling receipt-arc branches
  themselves.
- ack-30v-docid-reservation (2026-08-30): 30V = final design doc for
  directional tweaks; 30Va = this ledger; builder residue ledger sites under
  30R at lowest-unused suffix.
- ack-conductor-authors-the-spec (2026-08-30): the conductor hand-builds the
  prospective `dorc why` surfaces as TUI-as-spec plaintext; builders build
  toward it.
- Everything in `30Vb` is UNACKED strawman until adjudicated; its ask-list is
  the adjudication queue. (Superseded 2026-08-30: the whole ask-list was
  overtaken by the sitting's rulings, banked with `[TYPED]` tags in
  `notes/30V` §2 — that section is now the authoritative ack record for the
  why-surface direction.)
- ack-slop-prose-authorized (2026-08-31): the conductor may author the owed
  prose at Slop tier (the five receipt-flag help rows, the two receipt
  placeholders, the six stale lock `why:` citations), full-loom reads owed.
- ack-ancestors-only-walk-correct-as-built (2026-08-31): receipts are
  never-append, so forward links cannot exist in the files; walking from the
  explicitly-named root toward causes is correct. Selector conveniences like
  `--receipt-last` may enumerate/search a bounded store; that is graph
  discovery, not a union of histories.
- ack-steering-suspicion-confirmed (2026-08-31): the human suspected the
  receipt-crate steering rows were already written and the `30Rk` note stale —
  verified true (see the recast bullet in State).
- Root docs: the human committed their own USER_STORY re-cut (`1fc95c45`) and
  TODO-ADDTL row (`f6317f43`) mid-arc; both root docs stay conductor-untouched.
- ack-root-identity-fold-soft (2026-08-31): stands as built; SOFT ack only —
  the human has not investigated deeply. Not settled law; revisitable.
- ack-mode-deletion (2026-08-31): `SpineInvocation::mode()` dies outright —
  landed conductor-side: `core::spine::InvocationMode` deleted, `minted()`
  lost its mode parameter, five callers fixed (incl. `cli/src/receipt_edge.rs`
  — a known trivial fold-conflict candidate with the render lane).
- rul-no-reflexive-lexical-fences (human-typed 2026-08-31; SCOPE SHARPENED
  same day): the target is grep-gates as BACKSTOPS TO TYPECHECKING —
  agent-maintainable ones are ~valueless (the agent edits both sides) and
  disincentivize proper typing discipline; the valuable ones are exactly the
  human-requested, human-ack-only ones. Scans over inherently-textual
  material (declared-source rosters, frontmatter, literals) are legitimate
  INSTRUMENTS — but instruments report; deleting a user-facing string on
  their say-so stays a conductor-at-least, human-maybe decision (human lean,
  typed). Prefer types/structure; flag up where only a lexical backstop
  would serve. NOT a removal directive — existing fences stand. Durable:
  `spike/CLAUDE.md lexical-fences-are-human-ack-instruments`; relayed to the
  render lane (structural mechanisms built). Consequence: the
  orphan-arrangement census is UN-WITHHELD, redirected to a report-only
  doctor-family instrument (never a gate, never a deleter), last in the
  wiring lane's priority order.
- rul-whole-product-proof-rides-the-e2e-corpus (human-typed 2026-08-31): no
  new test TYPE — the publish-then-why whole-product proof lives as e2e-corpus
  cases, with a minimal e2e runner/case-shape extension authorized if the
  grammar cannot yet express a two-invocation sequence; the standalone
  `why_surface_route.rs` battery is dropped. The loom `run:` widening stays
  deferred. The `crates/why` unit/integration gates remain ordinary Rust
  tests. (SUPERSEDED IN PART 2026-09-01 by the HANDOFF section: the builder
  satisfied this ruling's letter with a runner-hardcoded needle gate, which the
  human ruled cruft to rip.)
- ack-test-architecture-rip-owed (human-typed 2026-09-01): "I'm shocked sequences
  are not modeled. Yes, this is absolutely owed … a lot of cruft-rip is needed
  here." The HANDOFF section is the directive's record; the human clears context
  before the fix proceeds.
- The 2026-09-01 design sitting (Fable conductor; every item below TYPED by the human):
  - ack-published-means-public-interfaces: "anything published" in
    `rul-fixture-identity-never-production` = PUBLIC INTERFACES; the narrow reading is
    "a durable-file feature/state we never want for production users shouldn't be
    producible by any code-path typeable into `cli/main.rs`" — NOT that such state may
    not exist (that would rule out deterministic testing).
  - ack-separate-harness-binary: "ack on separate binary-name, I see no downsides; but
    the division must be right out at the furthest, narrowest edge, because everything
    before the division is effectively untestable without inventing a sixth kind of
    test. that needs to become an invariant."
  - ack-e2e-is-the-tier: the state-only Rust batteries are e2e (the catch-all for
    testing the product above unit level that isn't a loom); looms are the editable
    subset; dir-cases should slowly become rare.
  - ack-tooling-never-deferred: good tooling work is always owed and never deferred
    unless catastrophically huge; it just cannot be done in flight or in parallel — so
    finding cruft OUTSIDE an arc is the moment to do it. Lane D is in scope; the only
    legal cruft is "clear improvement, deeply wanted, needs kernel mutation"; nothing
    from a cleanup arc becomes a TODO row.
  - ack-breadth-more-abstract: the testing-target universe is every seam of multiple
    unix processes on machines Doing Things; the general structure must COVER it, not be
    built by accretion — "more abstract, not less; more powerful, not smaller."
  - ack-unpeg-per-seam: separate flags/env per determinism seam, so determinism can be
    selectively un-pegged.
  - ack-looms-are-shell-sessions: no artificial restrictions on structure, ordering, or
    content; in-memory execution is a suite-perf optimization that must not lead design;
    "not supporting" is a closed-and-shrinking set (no meta-test for it).
  - ack-seams-as-sh-lines: seam/env mapping set IN the loom as `export` lines, with
    defaults for all looms; "we express things as sh in this house."
  - ack-facts-are-data: world-facts have no seam because they are facts (declared case
    data); the conductor's seed "partition" was withdrawn as a muddle.
  - ack-varied-seed-default: declared seed = regression + output-stabilization, an
    opt-in per case across the ENTIRE testing infra (regression e2es, unit tests, looms
    alike); looms DEFAULT to varied seed; pinning is the one-line fix when a surface
    contains ND.
  - ack-dogfood-the-engine (hard): use our own parser/env-model/rho/const-prop for looms
    if it doesn't chafe; HARD NACK if it softens a correctness invariant; HARD DEFER if
    it needs invasive kernel changes (hold the ideal picture for the next kernel arc).
  - ack-rebless-authorized: the whole point of non-concurrent suite work; fully separate
    product work from suite work.
  - ack-frontmatter-reduction-on-target: rip frontmatter only where a CLEARLY better
    option exists; stay on the specific goals; no rip-and-tear of merely-messy things.
  - ack-age-is-an-open-front: no plan or lean; carried into opaque review; not a TODO.
  - ack-opaque-review-may-be-owed: the conductor ignores opaque concerns mid-design and
    may break invariants for excellent praxis; the human routes the plan through review
    if owed, BEFORE any building step.
  - call-spine-baseline-dies (conductor's call, delegated): delete it.
  - ack-husks-cleaned: the human removed the six husk dirs themselves.
  - ack-no-dispatch-rewind: no builder dispatched; the human rewinds/restarts before
    pursuing; this ledger's HANDOFF is the brief.
