# 30Rk — durable-transition residue: lane report

> Tier: builder ledger for the out-quarantine residue named in `plans/30R` and in the
> 2026-08-29 LIVING_STATUS entry — the report-API tidy, the D5 deletion, the
> singular-implementation census. Current state and open items only; git carries the
> chronology. Nothing quarantined is reproduced here; where a quarantined ledger is
> load-bearing it is named by filename alone.

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
  re-parses: `SelectedRoot::order`, `WhyFactsInput::order` and `RootFacts::order` are all
  `ReceiptOrderToken`. A renderer that wants the digits still asks `spelled()`.
  DEVIATION, disclosed: this is stronger than the recorded item, which assumed the edge
  had only a spelling. There is no parse and therefore no unparseable state to type.

- **`30Rk:closure-membership-is-the-graphs-answer`** — `ClosureFacts::of` took
  `reached: Vec<RecordedDocumentId>`, so a caller could name a document the graph never
  reached. Membership is now `graph::ReachedClosure`, a private-field value whose ONE mint
  is `ReceiptGraph::closure_from(&RecordedDocumentId)`. The walk is question-directed
  toward CAUSES — an outcome reaches its intent and that intent's originating plans, a
  plan reaches nothing later — which is `30R:receipt-rooted-attention-and-cli` read
  literally. Only documents the graph HOLDS are reached; the root is the one exception,
  because it may have been opened as an explicit file outside any store, and a required
  sibling the graph cannot hold stays `SiblingState`'s to report.
  DEVIATION, disclosed: `WhyFactsInput::identity` and `SelectedRoot::identity` are GONE.
  The closure carries the root, so the root is named once; keeping a second field beside
  it would re-open the same can-disagree hole one level up. `facts_for` lost an argument
  as a result.

Behaviour change worth naming: the retired `Vec` shape let a plan-rooted question be told
it reached a later intent and outcome. The replacement cannot say that, and the case that
asserted it is rewritten to pin the rule instead. Nothing in production supplied a
non-empty vector, so no shipped answer moved.

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
  replay-admission arm, `results::replayed_records` (the laundering seat, whose own
  module doc said it goes when the durable does), `results::replay_scope`,
  `WidthOneAttemptScope::matches_claims`, and the loom consumer's `--whylog=` replay
  chain, its `parse_direct_why`, and the `whylog-publish` edge fault.
- The drifted-render lane: `cli::DriftedReceipt`, `drifted_receipt`, `recorded_tally`,
  `drifted_why_parts`, and `PlanTally` — which collapsed to `dorc_plan::DispositionCounts`
  once its drifted arm went, taking `is_drifted` and the drift row with it.
- `Receipt::replayed` and `Receipt::narratable`, both dead-constant with no replay to
  vary them, and `aid::narrative::PLANE_VERSION`, whose only purpose was the replay
  version-coupling `narratable` carried.
- Five diagnostic codes (`whylog-version-refused` · `-book-desync` · `-absent` ·
  `-corrupt` · `-unwritten`), their payloads, `WhylogCorruptReason` and its four
  registry sentences, their fixture rows, and their census entries.
- Eleven cases: the eight `whylog-*` looms, the two `why-drift-*` looms, and
  `whygallery-drifted-book-degraded-receipt.loom` — the seventh case `30Rh` left owed,
  which rode the drifted-replay path and was unbacked from the moment that path went.

Three loom-pipeline tests were reading a whylog case as a convenient fixture rather than
as their subject; they now read `dangling-reference.loom`, and two editable-surface tests
moved to `cfg-inline-refused.loom` and `why-claims-payload.loom` for the same reason.

## the blocker, and why it is not this lane's to fix

**`30Rk:the-lock-regeneration-is-gated-on-the-flag-table`** — both generated locks
(`aid/src/{catalog,arrangement}_lock.rs`) still carry rows for the retired codes, and
neither `dorc-loom publish --all` nor a scoped publish can regenerate them: the run
aborts on `cli-flag-requires-mode.loom`, whose replay is
`$ dorc plan --whylog=run.whylog --book=webhost.sh`. That case's SUBJECT is a live,
whylog-free code — "this flag is only valid with that mode" — and `--whylog` is merely
the example it happens to use. Retargeting it (`--host` outside plan/apply is the
obvious live pair) changes bytes outside the replay-output islands, which `publish`
refuses as a non-prose change; the sanctioned route is the `DORC_LOOM_DUMP` loop.

So the corpus half of D5 is welded to the CLI vocabulary work. The two were briefed as
separate items and are not separable in this direction: the flag table has to lose
`--whylog`/`--whylog-dir`/`--last` before the locks can be regenerated without the
retired codes, and until the locks regenerate the byte-identity gates stay red.

## STOPPED at the fence: the invocation mode is a durable content question

**`30Rk:the-recorded-mode-token-still-says-replay`** — `cli::receipt_edge::invocation_record`
stamps EVERY live invocation with `core::spine::InvocationMode::WhylogReplay`, whose wire
token is `whylog-replay`, and that field reaches the new receipt durable and is read back.
The name is stale twice over: it names a lane that no longer exists, and it says "replay"
about runs that are not replays.

NOT TOUCHED. Correcting it changes what the durable persists and what re-ingestion
consumes, which is `spike/CLAUDE.md:rul-durable-contents-reviewed-before-design` exactly.
It is named here for the conductor to route, not fixed locally.

## census: what still names the old format

Grep-level, over `crates/` and `verify/`, excluding the two generated locks:

- the CLI vocabulary itself — `--whylog`, `--whylog-dir`, `--last` — in
  `cli/tests/e2e.rs`'s durable case, `cli-flag-requires-mode.loom`,
  `cli-flags-mutually-exclusive.loom`, `cli-help-page.loom`,
  `durable-receipt-unwritten.loom`, and four other looms' replay commands;
- `InvocationMode::WhylogReplay`, above, and its four test call sites;
- `dorc-loom`'s usage text and three unit tests that name a retired slug as an example
  argument (`whylog-unwritten`, `whylog-absent`) — cosmetic, and cheapest to change in
  the same pass that fixes the cases;
- `durable_route.rs`'s assertion that no `whylog` directory appears under the state root,
  which is a NEGATIVE and is now trivially true — it should either go or be re-aimed;
- the two generated locks, which regenerate once the blocker above clears.

Compile-level: `cargo check --workspace --all-targets` is clean with zero warnings, so
nothing reaches the deleted module or its types by any path a build can see.

## open items

- `30Rk:the-account-export-died-with-its-lane` — `ACCOUNT_EXPORT` was the built-whole,
  switched-off influence-account durable export, and `internal_tooling::xfail`'s live pin
  `p-x-durable-account-export-is-enabled` still names it in its trigger, horizon
  `r31:kernel-punt-glance`. Deleting the old durable deleted the export. Nothing shipped
  changed (the switch was off, so it moved no byte), and the new durable already carries a
  per-site recorded influence GRADE — but the richer per-row account export would have to
  be rebuilt against the receipt durable, which is itself a durable-contents question. The
  pin now describes something that does not exist; retiring or re-horizoning it is a
  conductor act.
- `30Rk:steering-lines-that-name-the-deleted-module` — `spike/CLAUDE.md`'s
  `influence-is-carried-by-the-object` cites `plan::whylog::ACCOUNT_EXPORT` and the
  `p-x-durable-account-export-is-enabled` round-trip pin; `whylog-write-only-replay` and
  `probe-tape-not-a-cache` describe the retired durable; `core/src/spine.rs`'s
  `ExcludedContent` doc cites the same const. Steering is the conductor's seat, so these
  are named rather than edited.
- `30Rk:collision-detection-is-inert-in-production` — found in passing, not this lane's:
  `cli/src/main.rs`'s `ingest_recognized` passes `&[]` as every document's image to
  `ReceiptGraph::ingest_*`, so `same_identity_pair` compares two empty slices and
  `GraphFinding::IdentityCollision` can never fire on a real store walk. The graph's own
  battery covers the mechanism with real images, so this is a wiring gap at the edge
  rather than a broken correlator.
