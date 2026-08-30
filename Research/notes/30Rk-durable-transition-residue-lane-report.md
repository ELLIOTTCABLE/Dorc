# 30Rk — durable-transition residue: lane report

> Tier: builder ledger for the out-quarantine residue named in `plans/30R` and in the
> 2026-08-29 LIVING_STATUS entry — the report-API tidy, the D5 deletion, the CLI receipt
> vocabulary, the singular-implementation census. Current state and open items only; git
> carries the chronology. Nothing quarantined is reproduced here; where a quarantined
> ledger is load-bearing it is named by filename alone.

## report-api-close: what the three tidy items became

The three items are `30Rh`'s recorded API-close residue, taken as stated rather than
re-derived.

- **`30Rk:site-identity-is-one-value`** — `report::SiteFacts` carried `leaf: u32` +
  `member: Option<u32>`, and `AddressResolution`'s three arms plus
  `AddressFacts::resolved_site` carried the same pair as a bare tuple. All five now carry
  `rows::RecordedSite`, which the crate already owned and which the recorded model already
  hands out (`site.site()`), so the projection stopped decomposing an identity and
  re-composing it. `RecordedAst` was already exported on this surface, so no new domain
  crosses the report boundary (`inv-identities-never-cross-domains` holds).

- **`30Rk:order-is-carried-not-respelled`** — `RootFacts::order` was a `String`. The
  residue calls for a re-parsed `ReceiptOrderToken`; the edge turned out to already HOLD
  one (`receipt-local`'s `EntryName::order()`), and the `String` was a `.spelled()` hop
  the CLI seat took on the way in. So the token is carried end to end and nothing
  re-parses. There is no parse and therefore no unparseable state to type.

- **`30Rk:closure-membership-is-the-graphs-answer`** — `ClosureFacts::of` took
  `reached: Vec<RecordedDocumentId>`, so a caller could name a document the graph never
  reached. Membership is now `graph::ReachedClosure`, a private-field value whose ONE mint
  is `ReceiptGraph::closure_from(&RecordedDocumentId)`. The walk is question-directed
  toward CAUSES — an outcome reaches its intent and that intent's originating plans, a
  plan reaches nothing later — which is `30R:receipt-rooted-attention-and-cli` read
  literally. Only documents the graph HOLDS are reached; the root is the one exception,
  because it may have been opened as an explicit file outside any store, and a required
  sibling the graph cannot hold stays `SiblingState`'s to report.
  `WhyFactsInput::identity` and `SelectedRoot::identity` are GONE: the closure carries the
  root, so the root is named once. Both this and the ancestors-only walk direction are
  BUILT AS DESCRIBED and awaiting human adjudication; neither is to be reworked meanwhile.

Behaviour change worth naming: the retired `Vec` shape let a plan-rooted question be told
it reached a later intent and outcome. The replacement cannot say that, and the case that
asserted it now pins the rule instead. Nothing in production supplied a non-empty vector,
so no shipped answer moved.

## d5: what the deletion took

Deleted outright, with no adapter and no alias
(`inv-format-changes-are-one-cutover`, `rul-strawman-formats-no-compat`):

- `plan::whylog` in whole — the `dorc-whylog/2` grammar, `WhylogV2Metadata`/`Write`,
  `try_serialize_v2`, `WhylogLimits`, `DurableProjection`, `ApplyLine`, `DurableAccount`,
  `ACCOUNT_EXPORT`, `inspect`, the `view` module, and both admission entry points.
- `cli::whylog_store`, the engine's `write_whylog`/`report_whylog_unwritten`/
  `serialize_refusal_reason`, the `publish_whylog` and `durable_label` edge members, and
  `GeneratedOutput::Whylog`.
- The whole REPLAY lane: `engine::Replay`, `EngineRequest::replay`, the engine's
  replay-admission arm, `results::replayed_records` (the laundering seat, whose own module
  doc said it goes when the durable does), `results::replay_scope`,
  `WidthOneAttemptScope::matches_claims`, the loom consumer's `--whylog=` replay chain and
  its `parse_direct_why`, the `whylog-publish` edge fault, and gate-8's replay half.
- The drifted-render lane: `cli::DriftedReceipt`, `drifted_receipt`, `recorded_tally`,
  `drifted_why_parts`, and `PlanTally` — which collapsed to `dorc_plan::DispositionCounts`
  once its drifted arm went, taking `is_drifted` and the drift row with it.
- `Receipt::replayed` and `Receipt::narratable`, both dead-constant with no replay to vary
  them, and `aid::narrative::PLANE_VERSION`, whose only purpose was the replay
  version-coupling `narratable` carried.
- Five diagnostic codes (`whylog-version-refused` · `-book-desync` · `-absent` ·
  `-corrupt` · `-unwritten`), their payloads, `WhylogCorruptReason`, their fixture rows,
  their census entries, and their six orphaned arrangement rows.
- Thirteen cases: the eight `whylog-*` looms, the two `why-drift-*` looms,
  `whygallery-drifted-book-degraded-receipt.loom` (the seventh case `30Rh` left owed), and
  the retired replay pair inside `whygallery-survive-trusted-footprint.loom`.

Cases and tests that used a retired case merely as a convenient fixture were re-pointed at
live ones rather than deleted.

## the cli receipt vocabulary

`--whylog` / `--whylog-dir` / `--last` were already out of `parse_args_from`; what
remained was every surface still SPELLING them, which is what kept the corpus red.

- **`30Rk:a-known-flag-suggests-nothing`** — `nearest` skipped only distance-0, so a word
  the table already held fell through to its nearest NEIGHBOUR: `--receipt` answered "did
  you mean `--receipt`s?". A word the table holds is spelled correctly, so it now suggests
  nothing at all.
- **`30Rk:loom-publication-followed-the-named-store`** — the loom driver passed
  `args.receipts.is_some()` as its durable gate while production passes `!args.no_receipt`.
  This is `30Rh:fnd-publication-was-gated-on-a-named-store` surviving in the second driver:
  naming a store moves WHERE a receipt lands, never WHETHER one is written, so under the
  loom every case that named no store silently published nothing. It is also a
  `dorc-replay-is-production-semantics` breach — the loom is required to run production's
  own semantics — and it is what made `durable-receipt-unwritten` stop firing its own code.
- The three root selectors were already mutually exclusive across all three pairs, and
  `--receipts` already orthogonal and legal in every mode; nothing pinned either. Both are
  now pinned per pair, including `--receipts` beside each selector and outside `why`.
- The five help rows for the new vocabulary render, seeded UNWRITTEN.

## the invocation mode, renamed under a narrow human ack

**`30Rk:the-recorded-mode-token-said-replay`** — `core::spine::InvocationMode::WhylogReplay`
(token `whylog-replay`) stamped every live invocation. It is now `Unstated` / `unstated`.

CORRECTION to this lane's own earlier reading: that value never reached the durable. The
receipt records `tokens::RecordedInvocationMode` (plan / apply / round-trip), minted at the
publication seat from the CLI's mode and passed to `invocation_row` beside the Spine
record, which never reads `SpineInvocation::mode()` — a method with no caller in the
workspace. The rename is therefore in-memory only and moves no durable byte.

## census: one live implementation

- Code, excluding generated locks and comments: `whylog` appears nowhere in `crates/` or
  `verify/`.
- Generated locks: `catalog_lock.rs` retains four mentions and `arrangement_lock.rs` two,
  all inside `why:` METADATA prose of live rows (historical citations such as "its whylog
  sibling", "the whylog is a deterministic reproducer"). Prose is not this lane's to
  rewrite; listed under owed prose.
- Corpus: no `.loom` case drives a retired flag.
- Compile-level: `cargo check --workspace --all-targets` clean with zero warnings, and the
  whole workspace suite green, so nothing reaches the deleted module by any path a build or
  a test can see.
- `crate_boundary.rs`'s laundering-seat census is inverted: it counted the seat's callers
  down to one, and now proves it has none.

## open items

- **`30Rk:the-account-export-died-with-its-lane`** — `ACCOUNT_EXPORT` was the built-whole,
  switched-off influence-account durable export. Deleting the old durable deleted it.
  Nothing shipped changed (the switch was off) and the receipt durable already carries a
  per-site influence GRADE, but the richer per-row ACCOUNT export would have to be rebuilt
  against the receipt durable, which is itself a durable-contents question. The xfail pin
  `p-x-durable-account-export-is-enabled` is therefore `Reserved` rather than `Live` — the
  census's own word for deliberately-unbuilt — with its trigger corrected to stop naming a
  deleted const. Retiring or re-horizoning it is a conductor act.
- **`30Rk:steering-lines-that-name-the-deleted-module`** — `spike/CLAUDE.md`'s
  `influence-is-carried-by-the-object` cites `plan::whylog::ACCOUNT_EXPORT`;
  `whylog-write-only-replay` and `probe-tape-not-a-cache` describe the retired durable;
  `core/src/spine.rs`'s `ExcludedContent` doc cites the same const. Also standing from
  `30Rj`: `receipt/CLAUDE.md`'s `inv-reader-writer-states-only-narrow` lists a "trust"
  reader state that no longer exists, and `receipt-local/CLAUDE.md`'s
  `inv-owned-handles-authorize-operations` ends with a cleanup clause that is no longer
  true. Steering is the conductor's seat.
- **`30Rk:the-arrangement-mirror-is-its-own-lock`** — `aid::arrangement` re-exports
  `ARRANGEMENTS` FROM `arrangement_lock.rs`, so the generator's mirror-union reads its own
  output: a row whose owner disappears has no declaring source to drop it, and persists
  until removed by hand. Six orphans were removed that way here and the byte-identity gate
  accepted the result, but the shape means orphan rows accrue silently. A census of rows no
  source declares would catch them; not built.
- **`30Rk:the-recorded-why-surface-is-the-next-round`** — a receipt-rooted `why` still
  LISTS rather than explains (`30Rh:fnd-store-route-lists-it-does-not-explain`). Gate-8's
  replay half asserted the retired renderer's chain and is deleted rather than re-aimed;
  its live half is untouched. The sealed model exists and reaches the real reading path,
  but joining it to aid/weft is the next conductor's work, so no case asserts a
  receipt-rooted explanation today.

## owed prose

Builder-authored prose is zero. What is owed, all rendering `[unwritten: <slug>]` or
carrying stale citations:

- the five help rows the vocabulary cutover minted: `cli-help-option-receipts` ·
  `-no-receipt` · `-receipt-last` · `-receipt-id` · `-receipt`, seeded `words: None` in
  `arrangement_lock.rs` and rendered by `cli-help-page.loom`. A case may `own` each once it
  has words.
- `durable-receipt-unwritten` and `durable-receipt-unreadable` still render
  `[unwritten: <slug>]` as their message (pre-existing, not caused here).
- `catalog_lock.rs` and `arrangement_lock.rs` `why:` metadata still cites the whylog as a
  live reproducer in six rows; `cli-flag-requires-mode`'s `why` lost its `tc-whylog-…`
  clause when its case was retargeted.
