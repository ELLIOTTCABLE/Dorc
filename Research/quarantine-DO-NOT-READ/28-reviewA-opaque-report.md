# Opaque accrual review 28-reviewA

- Review identity: `28-reviewA`
- Pass: `initial` (no prior CONSTRAIN)
- Exact range: `833bbe0b..933dae59` on `ai/r28-loom-final`
- Reviewed tree: worktree `C:\Users\ec\Sync\Code\Dorc\.claude\worktrees\loom-final`
- Reviewer model: local Opus, per human authorization typed 2026-07-31 (normal lane is the foreign relay)
- Assigned scope: the loom-final arc — render-boundary weld with stamped provenance, placeholder/revision
  editability, the Rust-surface contract, ownership/metadata machinery, the `cli/src/results.rs`
  records-admission seam, the `main.rs` lib-seam extractions (why/world/kinds/survival/results/fixpoint),
  the `aid/src/foreign.rs` foreign-text seal, reason-enum migrations, ~60 new defining/world cases, and the
  conduct ledger `Research/notes/28L-loom-final-conduct-ledger.md` as design record.

## Evidence inspected

- The range's commit list (~130), full diffstat, and focused diffs of every surface below.
- `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` (read complete, before the range).
- The predecessor opaque report `29-reviewA-opaque-report.md`, for continuity on the width-one attribution
  posture.
- `spike/CLAUDE.md`, `spike/crates/cli/CLAUDE.md`, `spike/crates/aid/CLAUDE.md` (the governing law).
- `cli/src/results.rs` in full — both intake entries, `WidthOneAttemptScope`, `ScopedHostEvidence`,
  `facts_from_sites` (the rc firewall), `probe_origins`, the meet rules.
- `aid/src/foreign.rs` in full; `aid/src/display.rs` encoders; `aid/src/catalog.rs` `serialize_lock` and the
  `ParamText` → `RenderPart` mapping; `aid/src/diag.rs` `params_of`/`params_of_raw`/`render_cli_parts`;
  `aid/src/weave.rs` `to_runs`/`foreign_run`; `aid/src/arrangement.rs` `push_arrangement_sentence`.
- `aid/src/fixture.rs` (the compiled-in stand-in world table) and both lexical fences in
  `aid/tests/diag_tidy.rs`: `fixture_payloads_are_unreachable_from_production`,
  `fixture_intake_is_unreachable_from_production`, `foreign_edge_constructor_is_fenced`.
- `errorloom/src/address.rs` (the new alignment-based section addressing) and `container.rs`'s
  frontmatter change.
- `dorc-loom/src/generate.rs` (lock generation, absent-means-keep, metadata drift),
  `dorc-loom/src/bin/dorc-loom.rs` generic-executor dispatch, `dorc-loom/src/repository.rs`.
- `cli/src/transport_edge.rs`, `lint/src/render.rs`, `lint/src/source_external.rs`,
  `plan/src/records.rs`.
- Pre-range baselines at `833bbe0b` for `main.rs` (attribution machinery), `aid/src/diag.rs`
  (`params_of`), `aid/src/catalog.rs` (`is_foreign_param`), `dorc-loom.rs` (executor dispatch), to
  separate delta from unchanged architecture.
- Supplied test evidence taken as given (not re-run; this review is read-only): `both gate:full-quiet`
  green at every fold, Windows 1696 / WSL 1692+1 skipped; artifact byte floor held; both lock fixpoints
  and `transcript_bytes_equal_production_bytes` green.

## Accrual-threshold assessment

### The flagged item: `admit_controller_records` vs `admit_fixture_records`

+SURE This does not create a second scope, and does not widen the identity space. The scope constructor
`WidthOneAttemptScope::new` remains private and single; both entries funnel through it. The fixture entry
takes no `Framing`, host, nonce, attempt, or identity parameter and none can be supplied — it builds
`Framing::spike` internally, which is the already-named single substitution point that the *production*
path in `main.rs` (lines 1072, 1723) also uses today. So the fixture entry mints nothing the production
path could not already mint; it removes a parameter rather than adding a capability.

+SURE Bounding is preserved through the fixture path: it spends `read_host_evidence(stream,
HostEvidenceLimits::spike_default())` before delegating, so committed case bytes cross the same
aggregate/line/record/field budget as host bytes. All three `Admission` arms are honoured and `Refused`
propagates rather than collapsing into a measurement.

+SURE Both conductor-stated riders landed and are non-vacuous: the rustdoc carries the law citation and the
argument, and `fixture_intake_is_unreachable_from_production` asserts `scanned > 0` before it can pass.
The walk is recursive over `crates/*/src` including `src/bin`, so the `cli` binary is covered by the
`ALLOWED_CLI_FILES = ["results.rs"]` restriction.

~SUSPECT The conductor's one-scope-per-run / second-controller-not-second-scope reasoning is sound on the
law's own text, and matches `29-reviewA`'s ~SUSPECT-but-ACK on the same width-one posture. The residual
obligation is unchanged and unmoved: when real transport, concurrency, retry, cross-host reuse, or saved
approval first makes a second scope representable, the fixture entry must be deleted or type-gated at that
moment. That obligation now sits at a *named, tested* choke point rather than an implicit one, which makes
it more visible than before the range, not less.

+SURE Repairability is trivially local: one `pub fn`, one call site in `dorc-loom`, one lexical fence. No
persisted format, no public contract, no caller audit beyond a `publish = false` workspace. Fails
qualification half 2 decisively.

### The foreign-text seal (`aid/src/foreign.rs`)

+SURE This is a net strengthening of the governed sink-encoding surface, not a widening. It replaces a
parameter-NAME heuristic (`is_foreign_param(param) == "detail"`, which encoded exactly one hole name) with
a type: `ForeignBytes` has a private `raw` field, no raw accessor, two edge constructors, and exits only
through `on_measured_sink`/`on_plain_sink`; `ForeignText` has no encoder-skipping public constructor;
`Debug for ForeignBytes` renders the escaped form so a panic or test dump cannot carry a terminal escape.
Coverage strictly widens: `names`, `excerpt`, `output`, the transport spawn-refusal words and the whylog
relay values were previously unencoded on the string render and now are sealed at mint.

+SURE `from_io_edge`'s bare-`&str` hole is lexically fenced (`foreign_edge_constructor_is_fenced`) with a
two-way assertion — an ALLOWED entry that stops naming the constructor fails, so the fence cannot drift
wider than the code.

+SURE Hostility and sensitivity stay orthogonal in the type design: the module doc states it, and the two
types keep raw material separate from sink-encoded material rather than letting encoding confer trust.

### Reason-enum migrations and typed refusals

+SURE Replacing `String` passthroughs with closed reason enums at the CFG builder, syntax parser, predict
lexer/parser, check dialect, footprint coherence, durable parse, and the transport not-attempted split
*reduces* the population of host- and source-derived strings that reach prose at all. The transport case
is exemplary: one `String` reason became two typed worlds, with the platform's own words sealed through
`from_io_edge` at the first point the dependency-free `dorc-transport` boundary permits.

+SURE `params_of_raw`'s exhaustive no-`..` destructuring makes a new payload field an `E0027` at the one
seat that decides whether a value is loom-visible and whether it is ours or somebody else's. That is a
compile-time obligation on exactly the surface where a silent passthrough would otherwise appear.

### Lib-seam extractions out of `main.rs`

+SURE The seam holds. No `std::fs`, `std::env`, `std::process`, `Command`, `SystemTime`, `Instant`, or
`std::net` appears in `why.rs`, `world.rs`, `survival.rs`, `fixpoint.rs`, `kinds.rs`, or `results.rs`.
Values cross; queries do not. The clock is a DI enum (`RunClock`) with `Absent` and `Recorded` as
first-class non-clocks, so a replay dates records from the run that made them rather than from the moment
of reading.

~SUSPECT The extraction makes `SiteResults` (all-`pub` fields, `Default`) and `facts_from_sites` visible to
`dorc-loom` where they were binary-private before, so an unattributed `SiteResults` can be folded into
facts from one more crate. Three things keep this below threshold: `ScopedHostEvidence::new` became
*more* private in the same move (module-only, where it was file-wide before); `dorc-loom` is a
`publish = false` dev tool that opens no socket and drives no host; and dropping the scope at the fold is
the unchanged pre-range semantics of a width-one carry-don't-check regime, not something this range
introduced. Repair is a visibility annotation.

### The authoring pipeline (errorloom / dorc-loom)

+SURE `address.rs` replaces content-shape re-detection with a bounded alignment over the render's immutable
runs, refuses on ambiguity rather than guessing, and carries an explicit work ceiling. This moves edit
authority *toward* stamped provenance and away from byte-shape inference, which is the direction
`replay-editability-is-provenance` already required.

+SURE Generated-lock emission escapes through Rust `{:?}`, so authored prose cannot break out of a string
literal into generated source. Unchanged by the range except for the `help` field's variant.

+SURE The generic shell executor route is unchanged: it still requires an explicit `--shell=PATH` and is
absent by default. The range did not widen what reaches it.

~SUSPECT `aid/src/fixture.rs` compiles a canned stand-in payload table into the production `aid` library.
The fence (`fixture_payloads_are_unreachable_from_production`) allows the whole `aid` crate rather than the
one file, which is coarser than the code needs; verified no `aid/src` caller outside `fixture.rs` exists
today. The payloads are decision-inert narrative under `two-plane-aid-law` and cannot become a license, so
the worst case is a misattributed diagnostic, repairable by narrowing the fence to a file suffix.

### The render vocabulary's encoding seat (the one genuine reveal)

~SUSPECT Encoding for engine-classified values happens at the WEAVE seat, not at the MINT seat.
`RenderPart::ParamValue` is encoded by `weave::to_runs` (`encode_foreign(text, RENDER_VALUE_CAP)`) and
`RenderPart::ArrangementValue` by `sentence_value`, but `push_arrangement_sentence` and the `ParamText::Ours`
arm store raw bytes. Any consumer that calls `RenderParts::text()` *without* the weft round-trip therefore
concatenates raw values. `render_cli_parts` / `render_body_parts` do weave, so the main diagnostic surface
is covered; the non-weaving `.text()` seats (`chrome_line_parts`, `lint::render_human_parts_at`'s directly
pushed parts) are not.

~SUSPECT The range materially widens the *population* riding this shape by moving several composed emit-site
sentences into registry-sentence form with interleaved values, and by adding `.text()` seats. Live exposure
today is argv-supplied paths (`chrome("cli-why-pointer-line", &[book_name])`) and, on an unchanged and
untouched line, an external linter's relay message (`lint-relay-message`, `render.rs`). The relay-message
raw push predates the range and is explicitly out of verdict scope as unchanged old architecture.

+SURE This nevertheless fails qualification half 2. The parts vocabulary is workspace-internal with no
persisted form and no caller outside a `publish = false` workspace; the repair is to encode at the mint
seat or to make `.text()` unreachable without a weave — one or two seats. Critically, the range's own
typing (`ForeignBytes` provenance, stamped parts, exhaustive `params_of_raw`) makes that repair *easier*
than it was before the range, so deferral destroys no information or design freedom. Under the standing
instruction to ACK when uncertain, and to not let a count of localized defects move the verdict, this is
banked as a construction obligation rather than raised as a concern.

### Not findings (recorded so a successor does not re-derive them)

- Two untracked SyncThing conflict files sit in the worktree (`survival.sync-conflict-*.rs`,
  `wrapper-entry-incoherent.sync-conflict-*.loom`). Untracked, outside the range, and the human's standing
  environment note says to ignore this class.
- `plan::records::expected_header_prefix` is newly `pub` and formats nonce/attempt/host/book-digest into a
  refusal string; today only the fixture-side loom consumer calls it, with `Framing::spike`. Localized.

## Qualifying concerns

None. No change in this range is both (a) large/fundamental/cross-cutting in a way likely to spread through
representations, persisted formats, public behavior, many callers, authority-minting paths, or user-authored
artifacts, and (b) such that deferral would require a compatibility break, state migration, an unbounded
caller audit, recovery of discarded provenance, or reversal of a widely-assumed semantic contract.

## Repairability judgment

+SURE Every residual named above sits at an identified, tested choke point: one `pub fn` plus a lexical
fence for the fixture intake; one visibility annotation for the extracted results types; one mint seat for
the render-value encoding; one file-suffix narrowing for the fixture-payload fence. None persists a format,
mints an authority type, or discards provenance. The range's net direction on the governed surfaces is
toward provenance being carried in types rather than inferred from names or byte shapes, which strictly
improves the repair position of a later, deeper security review.

## Confidence

~SUSPECT High for the exclusive accrual question. Moderate-to-low for ordinary localized correctness, which
this review deliberately does not certify — in particular the alignment cost/ambiguity accounting in
`address.rs` (`Reading::extended` propagates the predecessor's `readings` count unchanged) was read but not
adversarially exercised, and the ~60 new case transcripts were not individually read. Neither bears on
architecture-scale accrual. Test evidence was taken as supplied rather than re-run.

## Hidden invariant inventory maintenance

Two surgical merges into existing entries; no new bullet, no unrelated entry touched.

- `sinv-sink-encoding` — added the `aid::foreign` seal as the required route for not-ours bytes, and named
  the non-obvious bypass shape this work revealed: encoding for engine-classified values lives at the weave
  seat, so a hand-pushed part reaching `RenderParts::text()` without the weft round-trip carries raw bytes,
  and `ParamText::Ours` is an authorship class rather than a safety claim.
- `sinv-production-fences` — named the two lexical ALLOW-list fences now guarding the fixture intake and the
  foreign edge constructor, and made ALLOW-list growth an explicitly governed act rather than a local edit.

## Final outcome

ACK
