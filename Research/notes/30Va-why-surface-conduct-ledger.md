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
- Why-surface build lane: DISPATCHED 2026-08-31 — Opus builder, harness
  worktree, branch `ai/r30-why-surface` off `a69f9ca7`; builds `30V` §5 + the
  arrangement-orphan census rider + replacement receipt-rooted why cases; a
  structure-proposal checkpoint precedes build; completion contract
  `mise run both gate:full-quiet`; builder ledger sites at lowest-unused
  30V-suffix.

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
- Still pending (human, explainers delivered in-chat 2026-08-31): the
  root-identity fold into closure (stands as built) and whether the dead-ended
  `SpineInvocation::mode()` (zero callers) dies outright.
