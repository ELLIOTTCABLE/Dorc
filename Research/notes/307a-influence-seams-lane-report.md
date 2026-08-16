# 307a — lane-influence-refusal-seams: builder lane report

> Tier: **LLM-authored, builder (Opus-class)**, working `ai/r30-lane-influence-seams` from
> `ai/r30-conduct@8c1a3132`. Vocabulary and laws are `notes/306b`'s; the arc proposal is
> `plans/306c`. Everything here is as-built and measured; confidence markers where not.

## §1 — What landed, per item

- **`item-influence-grade`** (`306b` §1a/§1b · `306c` §2) — `core::influence` mints the three-grade
  sealed type; widening is free, narrowing does not compile, pinned by a `compile_fail` battery in
  the `core::claim` manner. ONE mint at the intake edge, lexically fenced. Carriage rides
  `Admission::Admitted` into `ScopedHostEvidence`, which now holds the phase marker beside the
  controller scope. In-memory only; nothing renders it and nothing persists it.
- **`item-record-lane-codes` step 1** (`306b` §6e) — the strict admission path now NAMES which
  records condition it observed. Pure refinement under `rul-all-nine-refuse-on-the-strict-path`:
  every condition refuses exactly as the undiscriminated framing refusal did, so no disposition
  moved and no golden drifted.
- **step 4** — the eight case-less codes have defining cases; the coverage ratchet is EMPTY, closing
  the catalog census's only long-term exception. One whole-product e2e case pins the real binary
  refusing and naming a code end-to-end; the identity-shaped twin is BLOCKED (§5).
- **step 5** — `read_header`'s folded `which: String` is a typed reason enum with four
  registry-backed prose components, minted unwritten.
- **`item-report-only-output` (`306c` §3a) and steps 2–3 (re-home + fence the forgiving parser)** —
  NOT BUILT; stopped at checkpoint-1 on a resemblance verdict (§4). The forgiving deframer family is
  untouched, test-callers included.

## §2 — The seat table

| seat | before | after |
|---|---|---|
| `plan::records::admit_records` | every framing fault → `AdmissionRefusal::Framing` | discriminates into `AdmissionRefusal::Records(RecordsFault)`; same refusal, named |
| `plan::records::parse_header` | `Err(Framing)`, key identity dropped | per-key `RecordsFault::IntegrityMismatch(RecordsHeaderMismatch)` |
| `plan::records::read_header` (forgiving) | `RecordsIntegrityRefused { which: String }`, the strings authored in-crate | the typed reason enum; builder prose leaves a kernel crate |
| `plan::records::RecordsFault::spanless_diagnostic` | — | the nine codes' production emitter; every payload spelled literally for the spanless gate |
| `plan::records::admit_unscoped_host_records` | `Admission<AdmittedUnscopedHostRecords>` | `Admission<Influenced<HostReported, …>>` — the one influence mint |
| `plan::whylog::admit_unscoped_whylog_replay` | — | reads the grade off at the seam; the durable carries none |
| `cli::results::ScopedHostEvidence` | scope + value | scope + value + `InfluencePhase`, with an `influence()` accessor |
| `cli::results::{no_observation, replayed_records}` | — | phase by free widening through one named seat (§6, `dis-phase-by-free-widening`) |
| `aid::diag::RecordsIntegrityRefused` | `which: String` | `which: RecordsHeaderMismatch` + `records_header_mismatch_text` reason→component fn |
| `aid::arrangement_lock` | — | four hand-seeded rows, `words: None`, serializer field order |
| `aid::fixture::canonical_payloads` | one records entry | nine |
| `catalog_defining_cases::DEFINING_CASE_RATCHET` | 8 records entries | EMPTY |
| `hostsim` byte-tier fault DST | refused-or-clean-subset | plus a sometimes-assert that tear/glue reach their NAMED refusals |

Deliberately untouched, per ruling: the forgiving `deframe` family and its test callers; the loom
consumer's refusal-is-an-error seat; every runner and harness gate; the four Migrated registers whose
prose the strict path outdates (§6).

## §3 — Goldens and evidence

Sanctioned new-case mints, named individually:

- eight world-as-payload defining cases, `crates/aid/tests/records-{headerless-refused, glued-line,
  header-missing, sentinel-nonce, integrity-refused, torn-line, alien-line, late-line}.loom`;
- one whole-product e2e case + its transcript,
  `crates/cli/tests/records30-glued-line-refuses-the-attempt.loom` (scoped bless, trial filter
  `records30`);
- `crates/aid/src/catalog_lock.rs`, republished by `dorc-loom promote --accept-metadata` over exactly
  those eight slugs: `when_fires`/`why` re-sourced onto the strict admission path, and
  `records-integrity-refused`'s derived `example` following its new reason component. **Every
  `message:` register is byte-identical** — no user-facing prose moved.

Zero existing goldens re-blessed. The zero-flip expectation held: `records-sentinel-nonce` already
refused on the strict path, so the behaviour change `306c` anticipated does not exist there.

Fold gates, foreground, per leg:

- `mise run check` — green.
- `mise run both gate:full-quiet` — Windows leg 2058/2058 (1 skipped); WSL leg 2054/2054 (1 skipped).
- `mise run bless:dry` — green, working tree clean afterwards (zero golden writes).

Comment budget, `git diff ai/r30-conduct...HEAD -- '*.rs'`: **814 added lines, 243 matching
`^\+\s*//`** (29.9%). Decomposed: **200 are `///`/`//!` doc-comments**, mandated by
`spike/CLAUDE.md`'s doc-comment-every-public-type rule and dominated by `core/src/influence.rs`'s
module doc, which IS the `compile_fail` battery (the `core::claim` precedent carries 117 such lines);
**43 are plain `//` why-comments, 5.3%**. The only lever on the raw figure is deleting executable
pins.

## §4 — `fnd-report-only-is-a-resemblance` (checkpoint-1, credited)

`306c` §3a proposed siting the report-only output as an extension of the solve-certifier's consumer
floors. Measured against the code it is a resemblance, on four grounds, and the half was dropped:

1. every certifier floor's product is a LADDER demotion — everything to `MustRun`, i.e. a plan in
   which every line runs, carrying mutation authority. `306b` §4a forbids demotion as a substitute
   for refusal and §4b forbids that plan.
2. the verdict rides as a FIELD with a `trusted()` accessor and the floored product is the same type
   rebuilt at ⊤ — the flag shape `306b` §4b rules out.
3. `FailedChecks<L>` carries node indices, edges, lattice values, components, a replay. An intake
   refusal has none of those; widening it would launder a channel fact into a post-fixpoint check.
4. **decisive**: the dependency edge is `plan → analysis`, and intake lives in `plan::records`.
   Records reach the model only as the cli's `observe` closure and through the validity fixpoint —
   `classify_with_why_diags` never sees one, and returns a 7-tuple rather than an output type. There
   is no analysis output an intake verdict could reach without inverting an edge.

The extension point that does exist is `Admission`/`rul-admission-is-a-closed-outcome`, whose
`Refused` consumer is one seat in `main.rs` — today `report_at` then an early return, before the
fixpoint, before `build_plan_walled`, before render, before the whylog.

`tc-report-substrate-is-the-plan` (flagged, unresolved, banked for the human-led re-plan):
`cli::why::WhyReport` holds `plan: &dorc_plan::Plan` and `WhyWorld` owns one, so every
disposition-bearing report surface is a function of the plan. "A complete analysis and a full graded
report that cannot yield a plan step" resolves only by containing at plan emission (which `306c` §3a
forbids) or by minting a second report substrate beside `Plan` (a parallel path, which `306c` §7
calls worse than no trigger).

## §5 — `fnd-defining-case-placement-pincer`, and the blocked identity pin

Two structural walls, both verified against code, both discovered on contact.

**The placement pincer** (why the eight are world-as-payload, not whole-product):

1. `catalog_defining_cases::is_case_owned` resolves ownership from `CARGO_MANIFEST_DIR/tests` — the
   aid collection ALONE, not the corpus-wide scan. A case elsewhere cannot shrink the ratchet.
2. the aid corpus loader REFUSES a `.loom` there declaring neither `code:` nor `arrangement:`.
3. `check_hygiene(Some("code"))` demands the slug in every replay block's OUTPUT, and the looms
   runner runs it BEFORE the `fixpoint: executed` shortcut.
4. the diagnostic prints on stderr; a loom transcript records stdout.

(1)+(2) force `code:`; (3)+(4) make `code:` unsatisfiable for a whole-product case. This is
`rul-slug-decides-loom-placement` working as written — the canonical loom for a registered slug
belongs in the one primary collection, and only the payload route can live there.

Worth recording as a non-blocker: `main.rs` prints the probe artifact BEFORE intake, so a round-trip
refusal leaves non-empty stdout and gate-1's crash/empty guard passes; `exit:` already carries the
exit-12 contract. The harness was never the obstacle.

**`fnd-identity-fault-is-not-expressible-e2e`** — the second, identity-shaped whole-product case is
not expressible. `e2e.rs::framed_results` REBUILDS the frame: it synthesizes the header from a real
`dorc probe` run over the same book, strips any header/sentinel/token from the case's
`probe-results.txt`, re-frames every body line with the correct nonce and token, fills missing site
records, and appends a correct sentinel. Every identity key is therefore controller-minted twice from
the same inputs and always agrees. Surveying the nine against that machinery, exactly one is
reachable end-to-end — a body line carrying an EMBEDDED terminal token, which survives the trailing
strip and lands as `records-glued-line`. Header-missing, headerless, sentinel-nonce, torn, alien,
late and fact-truncated are each normalized away; integrity-refused cannot be provoked at all.

So one case landed, not two. The question is §7's.

## §6 — Flags, each phrased as the question it would have asked

- `flg-four-migrated-registers-now-misdescribe` — HELD for the human by ruling; recorded here so it
  is not lost. `records-torn-line`, `-alien-line` and `-late-line` say "discarded (counted, never
  folded)" and `records-sentinel-nonce` says "ignored", but the strict path refuses the whole attempt
  for all four. Pre-existing (the strict path refused all nine before this lane too) and newly
  visible only because they now have production emitters and committed transcripts. No register was
  authored or changed. *Would have asked: should these four be re-authored, and by whom?*
- `dis-phase-by-free-widening` — ENDORSED as-built by ruling. `no_observation` and `replayed_records`
  hold no graded carrier, so their `InfluencePhase` comes from widening an authored-before-contact
  unit through one named seat rather than a second mint. Over-claiming influence is the conservative
  direction, and both paths earn it: whether bytes arrived is host-determined, and a durable's
  contents are host-shaped. *Would have asked: thread a real carrier through the replay path
  instead, at the cost of touching §3-adjacent surface?*
- `flg-fact-truncated-metadata-left-stale` — `records-fact-truncated`'s catalog metadata still names
  `plan/records.rs finalize` while its eight siblings now name the strict path. Left alone to keep
  the change surface to the eight this lane minted. *Would have asked: fold it in for consistency?*
- `flg-count-of-one-on-the-strict-path` — the strict walk stops at the first offending line, so
  `records-{torn,alien,late}-line` report `count: 1` there where the forgiving deframer aggregates.
  An honest report of what was seen before the refusal, and no consumer reads it as a total
  (`306b` §6b). Surveying the whole stream instead would be a behaviour change. *Would have asked:
  is a count that can only ever be 1 worth carrying on the strict path?*
- `flg-allow-list-entry-not-added` — `306c` §3b step 3's governed allow-list entry was never needed:
  the forgiving parser was not re-homed, so no fence was added and no allow-list grew. Recorded
  because the checkpoint flagged it for review at fold.

## §7 — The one open ask

`ask-identity-shaped-pin-is-unreachable` — the ruling's second whole-product case cannot be written
without changing `framed_results`, and the same ruling excludes harness changes. Four ways out, none
taken:

1. accept ONE e2e pin (landed) and record the identity family as covered by unit tests only —
   `strict_admission_names_each_records_condition` already drives all four header keys and every
   other condition against the real admission path;
2. add a case-declared raw-stdin escape to the e2e runner (a harness change, and a permissive one:
   every other case's frame would stay synthesized, but the escape is a second intake route into the
   corpus);
3. pin the identity family at the DST tier instead — extend the byte-tier fault mutator with a
   header-key mutation, which reaches the real admission path with no harness change and no new
   golden;
4. land a second framing-shaped e2e case instead of an identity-shaped one — but only
   `records-glued-line` is reachable, so there is no second one to land.

My lean is (3): it reaches the same production seat, keeps the harness untouched, and puts the
identity family under seeded coverage rather than a single example. (1) is the honest minimum. Not
built either way — the ruling said two e2e cases, and this is the question rather than an act.
