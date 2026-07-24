# 297 security-refresh phase packets

Quarantined implementation log for `297-security-refresh-build-plan.md`. This is a
compression boundary, not a general security document.

> HISTORICAL, kept current only where marked (ported to the `ai/main` lineage
> 2026-07-24; it had lived only on `ai/r29-ingress`). Two things date it. The
> phase-zero seam map describes the PRE-repair report scaffold, which phase one
> removed - it is now the best surviving record of what was deleted, and the
> sizing input for the repair specified at `29A`. And its native-shell limitation
> is no longer true: `sh` and `dash` are present in the Git Bash environment, and
> the full gate set including 97/97 e2e ran natively on the shipped revision
> 2026-07-24 (`29A`). The phase-one through phase-three packets remain the frozen
> specs those landings were built against; read them before touching report
> rendering, aggregate mints, or ingress.

## phase-zero-current-seam-map

Baseline revision: `e09eb44ab0f786ad37347a2a70825a5a206f7abd` on
`ai/r29-impl`. The human's native-Windows mise amendment to plan 297 is present in
the worktree. Cargo 1.96.0 reaches compilation when the six environment roots in
that amendment are set; `MISE_OFFLINE=1` avoids unrelated global-tool metadata
refreshes. Native `cargo test --workspace` compiles and reaches the test suite, but
the 52 `plan/tests/render_corpus.rs` cases refuse because neither `dash` nor `sh` is
on the native PowerShell `PATH`. Treat that as a recorded platform limitation, not
as a blessable baseline. The WSL lane is now verified: outside-sandbox `wsl.exe`
sees Ubuntu under the human's Windows identity, a fresh subagent can trust its own
worktree's `mise.toml`, and Cargo builds successfully. Run the shell syntax/e2e net
there; do not install a native shell.

Current report capture is confined to
`plan::render::probe::record_scaffold_draining`, called by
`ProbePlan::render_sh` in `spike/crates/plan/src/lib.rs`. It emits a predictable
`${TMPDIR:-/tmp}/dorc-drep.<nonce>.<site>` pathname, truncates it, binds
`DREP_V1`, reopens it with `cat`, truncates again, and removes it. Runtime report
parsing is in `cli::drain_runtime_reports`/`parse_report_record`; static report
recognition is in `oracle::report` and `oracle::predict` and is not a report-channel
ownership mint.

Current mutation authority types and mints live in
`spike/crates/plan/src/lib.rs`: `ReplaceLicense`, `GuardLicense`, `VerdictVouch`,
`Vouches`, `build_vouches`, and `prove_members_replaceable` /
`prove_inline_replaceable`. `ReplaceLicense::mint` accepts an optional vouch for
non-aggregate effects, while the members/inline branches can currently mint from
aggregate observations without one vouch per erased establish. `StepReceipt` has
only a representative optional vouch locus. `ReplacementLicense` is the plan's
unifying enum name in 297; the code currently calls the corresponding witness
`ReplaceLicense`.

Host records enter through `plan::records::deframe`, then
`cli::parse_results`; whylog replay enters through `cli::load_whylog_replay` and
`plan::whylog::parse`. The present readers use owned `String`s and do not establish
the full bounded/admitted/scoped chain required by phase three. Whylog writes and
retention are in `cli::write_whylog` / `whylog_entries`.

Display routes needing the later sink inventory include `cli::report_at`,
`render_coord`, `render_provenance`, runtime report `raw`, resolver/reach/derivation
fields, stdout/stderr/tool-error lanes, why/whylog renderers, paths, and lint JSON.
`cli::sanitize_report_raw` is the present report-only sanitizer and grants no typed
sink distinction.

Authority classification for the two parallel phases:

- report bytes: supplier = oracle body/managed host; interpreter = report parser;
  permitted consequence = aid only; scope = one controller-created attempt/site
  channel; expiry = drain/cleanup; never a claim or license input.
- verdict vouch: supplier = reached authored `__is_converged` path; interpreter =
  planner mint; permitted consequence = mutation erasure/guard at the exact site;
  scope = site/fact/oracle source/current attempt where represented; expiry = current
  plan generation; never a fact-plane value.
- probe observation: supplier = bounded host result; interpreter = result fold;
  permitted consequence = fact plane only until a separate private mint consumes a
  matching vouch; scope = current site/attempt; never substitutes for authorship.
- receipt/evidence: supplier = private decision mint/collapse; interpreter = aid
  renderer; permitted consequence = narration only; no conversion back to authority.

## phase-one-owned-report-channel

Validated revision correction (`ai/r29-report-channel`,
`c69b1b006a0781e3180b87f8216bac29efc5b047`): the phase-zero baseline predates this
builder worktree, but the report call-site inventory is unchanged. The sole pathname
constructor remains `plan::render::probe::record_scaffold_draining`; its sole caller
remains the `ProbePredict::emits_report` branch in `ProbePlan::render_sh`. The CLI's
`parse_report_record` remains downstream parsing only. There is no execution-edge
descriptor-inheritance contract in this revision, so this phase takes the frozen fallback:
runtime report capture is disabled in emitted probes while static decline classification
remains available. No CLI/hostsim execution-edge file is therefore in the mechanical
migration inventory.

Scope files: `spike/crates/plan/src/render.rs`, `spike/crates/plan/src/lib.rs`,
their focused tests, and only the minimum CLI/hostsim execution-edge files needed
to supply a real owned capability. Read every entered directory's `CLAUDE.md`.

Frozen direction: remove pathname report plumbing. The pure renderer must accept
only a private capability-shaped report sink, never `String`, `Path`, nonce, or
host-selected pathname. If native descriptor inheritance cannot be established at
the current execution edge without designing a new executor, disable emitted
runtime report capture and retain static decline classification. Setup failure must
prevent oracle execution. Report-free probe bytes remain identical. Do not change
the authored `${DREP_V1:-/dev/null}` idiom or static parser.

Applicable law: `sinv-owned-probe-channel`, `sinv-host-evidence-ingress`,
`sinv-controller-attribution`, `sinv-hostile-sensitive-orthogonal`,
`sinv-integrity-failure-mutation`, `sinv-production-fences`; plus
`spike/CLAUDE.md:inv-determinism`, `report-lane-versioned-entry`,
`two-plane-aid-law`, and `plan/CLAUDE.md:ap-2-runnable`.

Required tests: former predictable targets remain byte-identical; hostile
`TMPDIR` is irrelevant; collisions/replacement cannot cross ownership; unavailable
inheritance and setup failure execute no oracle; writer/reader/cleanup failure is
bounded aid only; concurrent sites do not cross; report-free bytes are identical.
Shell-shape tests run under mocks only. No general transport, new executor, remote
protocol, or arbitrary-path compatibility constructor.

Stop if a correct implementation requires a public path/string conversion, a new
authority mint, or an execution-edge design not frozen above.

## phase-two-every-establish-vouched

Validated call-site correction at baseline `c69b1b006a0781e3180b87f8216bac29efc5b047`:
`prove_members_replaceable` and `prove_inline_replaceable` each have one production caller
(`members_disposition` and `inline_disposition`) plus crate-local focused callers. Both production
callers are reached only from the `build_plan_walled` class walk. The frozen scope must therefore
also migrate that walk and thread its existing `Vouches` input into both aggregate disposition
helpers. `InlineSite` already carries exact body `CfgNodeId` plus classification, and Members
already carries the aggregate site `CfgNodeId` plus ordered facts, so no analysis-crate interface
widening is required. `build_vouches` currently ignores both aggregate classes; it must mint only
the specifically reached aggregate-site/body-site verdict vouches through the existing private
lift path, without changing the public `Vouches` representation or constructor surface. Existing
test-only broad helpers are not acceptable positive coverage for the new aggregate paths and must
be replaced there by exact site-specific vouches.
The phase-zero reference to `StepReceipt` is stale at this baseline: aggregate narration presently
rides public `Derivation`, whose single optional `vouch_span` is consumed by CLI why rendering and
excluded by `plan::erasability`. Phase two must replace that representative-only field shape for
aggregate mutation erasure with ordered per-establish receipt entries while keeping those entries
decision-inert and erasability-exempt; no separate `StepReceipt` caller exists to migrate.
Because `cli::render_why_chain` and `cli::whylog_entries` are the only consumers of that
receipt locus, the frozen requirement that why output attribute every erased establish also adds
the minimum corresponding receipt-render migration in `spike/crates/cli/src/main.rs`; executable
probe/apply rendering remains wholly in `plan` and must not inspect the expanded narration.

Scope files: `spike/crates/plan/src/lib.rs`, `spike/crates/plan/src/erasability.rs`
if required, `spike/crates/plan/tests/{erasability,observable_matrix,render_corpus}.rs`,
and focused e2e fixtures only when a unit/integration test cannot pin the emitted
artifact. Read every entered directory's `CLAUDE.md`.

Frozen types: private `EstablishVouch` binds exact `CfgNodeId`, `FactKey`, and one
consumed `ByVouch<VerdictVouch>`; private non-empty `AllEstablishesVouched` uses a
head-plus-tail representation and is constructible only from the exact ordered
effect-bearing establish sites plus `Vouches`. Query-only aggregates use a distinct
read-substitution proof and invent no vouch. Aggregate mutation-erasure mints consume
the all-vouched proof by value. Renderer unification remains an enum; executable
rendering cannot inspect receipt narration.

Refusal is atomic for absence, decline, dynamic argv, duplicate/extra/reordered or
site/fact/cardinality mismatch. Existing convergence, self-reach, grade, consumption,
predicted-status, renderability, top-containment, and all-or-nothing gates remain
necessary. Mint-time receipt carries every erased establish's ordered identity and
exact vouch locus/source rather than a representative first fact.

Applicable law: `sinv-mutation-elision-vouch`, `sinv-authority-map`,
`sinv-private-authority-mints`, `sinv-controller-attribution`,
`sinv-bounded-observables`; plus `spike/CLAUDE.md:claim-tier-gating`,
`rul-vouch-is-verdict-authoring`, `inv-one-observable`, `two-plane-aid-law`, and
`plan/CLAUDE.md:sole-mint-witnesses`.

Required tests are the full `297:aggregate-vouch-tests` matrix. Positive fixtures
construct the specific reached vouches; never bulk-add `test_vouch()`. Compile-fail
coverage must show observation, silence, and aid evidence cannot satisfy the mint.
Narration-only changes leave probe/apply bytes identical.

Stop if exact site/fact identity is unavailable without widening a public constructor,
if query substitution and mutation erasure cannot remain separate, or if any proposed
conversion promotes observation/evidence into a vouch.

## phase-three-bounded-host-evidence-admission

Mapped baseline: `f9679855d9193337d5a82588d8229f2c738805f0` on
`ai/r29-ingress`, including the conservative query-status correction from phase two.
This packet freezes phase three only; it does not authorize phase-four sink work,
phase-five production fences, transport design, or fleet identity semantics.

Current ingress map:

- Live host-result bytes enter `cli::run_one_book` in
  `spike/crates/cli/src/main.rs` through `std::fs::read_to_string` for `--results`
  or unbounded `stdin.read_to_string`; replay substitutes
  `Replay.doc.raw_results`. All three converge on
  `plan::records::deframe` and then private `cli::parse_results`.
- `plan::records::deframe` accepts an already allocated UTF-8 `&str`, scans it with
  `contains`/`lines`, allocates an owned `String` per retained record, parses header
  `sites` as unbounded `usize`, and exposes independent `records`, `refused`, and
  `framed` fields. A caller can parse `records` despite `refused`; the production
  caller currently does exactly that. `LegacyPolicy::Tolerate` is selected by the
  ambient `DORC_ALLOW_LEGACY_RESULTS` environment variable in the production binary.
- `cli::parse_results`, `parse_site_record`, `parse_report_record`, `split_key`,
  `parse_site_key`, and `parse_leaf` are the inner grammar. Free fields are copied
  into derivation, resolution, reach, and report collections. `stdout=` and
  `stderr=` are interned immediately. Numeric entry points are leaf/member `u32`,
  record rc `i32`, derivation count `u32`, reach arm `usize`, and the ordinal's
  unchecked `usize as u64`. Duplicate keys meet safely, but duplicate/cardinality
  storms remain allocation work.
- `facts_from_sites` turns site records into `Observable`s; `probe_origins` mints
  measured receipt origins; `merge_derived_footprints` turns derivation text into
  trusted at-most footprints; `build_resolutions` interns canonical entities;
  `expand_footprints_via_reaches` interns reached entities; report records feed aid.
  These flow into `build_plan_walled`, where observations plus separately held
  vouches can authorize replacement/guard, and derived footprints can authorize
  flag-gated survival past mutation. Therefore a deframer refusal is an
  attempt-integrity event, not merely a missing-fact event.
- Whylog replay enters through `cli::load_whylog_replay`: unbounded
  `read_to_string` then `plan::whylog::parse(&str)`. That parser scans lines,
  allocates argv/path/digest/apply fields, accepts `results bytes=<usize>`, copies
  the opaque inner result block, and returns it to the live result ingress. The
  writer's `WHYLOG_CAP` is not a read-side defense: pre-existing files bypass it,
  and its truncation can deliberately produce a corrupt durable.
- Fixture and simulation bypasses are `cli` test helpers `parse_str` and
  `parse_framed`, direct `plan::records::deframe` unit/DST callers, the e2e
  `DORC_ALLOW_LEGACY_RESULTS` export, and hostsim's `fault::mutate` plus
  `differential::exec_probe` lossy UTF-8/string normalization. They may test the
  same admission kernel, but none may mint a production attempt scope or enable
  headerless admission in a production binary.

Frozen dependency direction and types:

```rust
// plan::records; fields private, constructors validate, no Default.
pub struct HostEvidenceLimits { /* private bounded non-zero fields */ }
pub struct BoundedHostBytes { /* private Vec<u8> + charged counters */ }
pub struct AttemptScope { /* private controller-attributed fields */ }
pub struct FixtureAttemptScope { /* cfg(test), no conversion to AttemptScope */ }
pub struct ParsedRecord<T> { /* private T + per-record retained charge */ }
pub struct ScopedHostEvidence<T> { /* private AttemptScope + T */ }
pub enum Admission<T> { Admitted(T), NoObservation, Refused(AdmissionRefusal) }

pub fn read_host_evidence<R: std::io::Read>(
    reader: R,
    limits: HostEvidenceLimits,
) -> Result<BoundedHostBytes, AdmissionRefusal>;

pub fn admit_host_records(
    bytes: BoundedHostBytes,
    expected: &AttemptScope,
    limits: HostEvidenceLimits,
) -> Admission<ScopedHostEvidence<ParsedHostRecords>>;

// plan::whylog; outer parser never accepts String/&str from the CLI.
pub fn admit_whylog<R: std::io::Read>(
    reader: R,
    outer: WhylogLimits,
    inner: HostEvidenceLimits,
) -> Admission<ScopedWhylogReplay>;
```

`AttemptScope` is created only at the controller I/O edge from values the controller
already owns. In the width-one spike it binds the existing host, book target/digest,
attempt, nonce/source set, and current run generation; the payload may only be checked
against those values. The host never supplies or refreshes scope. This representation
reserves target/source-set/generation distinctions without inventing remote IDs or retry
semantics absent at HEAD. Constructors expose comparison/display accessors only; there is
no public struct literal, `From<String>`, parse-from-payload, `Default`, or scope-rebinding
conversion. `FixtureAttemptScope` is test-only, has a distinct admission entry point, and
cannot convert into `AttemptScope` or `ScopedHostEvidence`.

`plan::records` owns byte reading, physical framing, scope verification, the closed
record-tag grammar, numeric parsing, and bounded retained byte records. It remains pure
after the injected `Read`. The CLI owns conversion from admitted record variants into its
private `SiteResults` and the shared interner. That conversion accepts only
`ScopedHostEvidence<ParsedHostRecords>` by value and returns a scope-preserving private
`ParsedHostEvidence`; it has no raw-string/record-vector overload. The only production
admission-to-plan function consumes `Admission<ParsedHostEvidence>` and returns either an
admitted plan input, conservative no-observation input, or a refusal disposition. No
`Refused` arm constructs `Observable`, origins, derived footprints, resolutions, reaches,
or calls `build_plan_walled`.

Closed disposition law:

- `Admission::Admitted` means framing, scope, grammar, numeric, and all resource checks
  succeeded. Only this arm may intern admitted fields and reach observation/footprint
  consumers.
- `Admission::NoObservation` means a correctly owned, complete attempt carried no usable
  observation. It follows ordinary conservative behavior: facts are unknown, no host-derived
  footprint/resolution/reach exists, and the unchanged book runs as required. It is not an
  integrity diagnostic.
- `Admission::Refused` means malformed framing/grammar, invalid UTF-8 where text is required,
  truncation, overflow, stale/alien/forged scope, duplicate-integrity ambiguity, or any limit
  breach. It produces one root-cause refusal plus bounded aid and prevents creation of any
  mutation-authorizing apply plan for that attempt. It must never be mapped to `Unknown`, an
  empty `SiteResults`, `NoObservation`, or a partial accepted collection.

Frozen limits and enforcement order: introduce explicit conservative constants in one
`HostEvidenceLimits::spike_default()` and boundary-test each value; changing values later is
policy, not format. The first builder may select exact constants within these ceilings:
8 MiB total stream, 64 KiB physical line, 65,536 records, 16 KiB any free field, 4 MiB total
retained decoded fields, 32,768 entries in any one collection, and 16 ASCII digits for every
numeric token. Every increment uses checked arithmetic; overflow is refusal. The bounded reader
stops at total-bytes-plus-one before UTF-8 conversion. Line length and record count are checked
on bytes before allocating line strings. Tag/key/numeric grammar is checked on borrowed slices.
Free-field length and aggregate retained charges are checked before copying. Collection
cardinality and duplicate-storm work are charged before insertion. Site stdout/stderr,
canonical entities, reached entities, derived coordinates, report raw text, paths, argv, and
all other host/durable strings are never interned or owned beyond the bounded buffer until their
field and retained budgets pass. Unknown tags/keys may remain additive only if their bytes and
records are charged; an unknown record tag is malformed, not silently dropped.

Whylog composes two independent budgets. `admit_whylog` first applies an outer durable byte,
line, record, field, numeric, allocation, argv/oracle/apply-cardinality budget. Its
`results bytes=N` prefix is digit-bounded, checked-add parsed, and must fit both remaining outer
bytes and the configured inner stream ceiling before slicing/copying. The opaque block is then
passed as bounded bytes to `admit_host_records`, which independently charges every inner byte,
line, record, field, retained allocation, collection, and numeric value. Outer accounting never
credits or disables inner accounting; the inner block consumes both budgets. A wrong version,
missing sentinel, repeated results block, trailing/overlapping block, scope mismatch, or either
budget failure is `Refused`. Replay returns the recorded `AttemptScope` only after comparing it
with controller-owned current book/oracle digests and allowed replay context; it cannot mint a
fresh scope from durable text.

Ordered mechanical call-site checklist:

1. Replace `--results` and stdin `read_to_string` in `run_one_book` with
   `read_host_evidence`; remove the environment-selected legacy policy.
2. Replace `Deframed`'s separable public fields and `LegacyPolicy` with the closed admission
   return; make the framed/inner parser consume bounded bytes and emit closed typed variants.
3. Move all numeric and free-field validation ahead of ownership/interning; make
   `parse_results` consume admitted variants only and preserve scope.
4. Gate `facts_from_sites`, `probe_origins`, `merge_derived_footprints`,
   `build_resolutions`, `expand_footprints_via_reaches`, report pairing, and
   `build_plan_walled` behind the single `Admitted` match. Add an explicit refusal return before
   artifact rendering/whylog writing.
5. Replace `load_whylog_replay`'s `read_to_string` and `whylog::parse(&str)` with composed outer
   and inner admission; validate replay digests/scope before re-entering the plan pipeline.
6. Remove public/raw convenience paths that accept `&str`, `Vec<String>`, or caller-set
   `refused`; keep only cfg-test fixture admission using `FixtureAttemptScope`.
7. Change e2e legacy fixtures at the harness boundary: derive the current emitted header/sentinel
   and wrap fixture inner records before invoking the production binary. Do not retain an env or
   CLI switch that relaxes production admission.
8. Route hostsim faults as bytes through the production admission kernel. Lossy UTF-8 conversion
   may exist only after an explicit fixture refusal assertion, never before admission.

Required tests: every byte, line, record, free-field, retained-byte, collection, numeric-digit,
and allocation limit at boundary-minus-one, boundary, and boundary-plus-one; invalid UTF-8;
missing newline/token/header/sentinel; torn, glued, alien, late, duplicate header/sentinel/results
block; integer overflow and signed/unsigned edge cases; unknown/malformed tag/key; duplicate-key
and unique-key storms; stdout/stderr, report, derivation, resolution, and reach interner storms;
forged host/target/attempt/source-set/generation; stale replay and current-scope mismatch;
whylog outer-only, inner-only, and simultaneous budget exhaustion; pre-existing oversized whylog;
empty well-owned stream as `NoObservation`; and assertions that every refusal emits no apply plan
with mutation authority. Add compile-fail/privacy coverage showing raw bytes, parsed fixture data,
payload scope, `NoObservation`, and `AdmissionRefusal` cannot become
`ScopedHostEvidence` or reach the private plan-input constructor. Existing torn/glued/permutation
DST cases remain, routed through the new entry point.

Priority tension: maintainability favors one admission kernel and opaque state transitions;
simplicity favors retaining string parsers; validation and security require byte-first parsing,
checked accounting, and a closed refusal type. Controller-local performance is subordinate to
remote command time, but pre-allocation enforcement prevents adversarial memory/CPU work. Prefer
named small state machines and repeated explicit checks over a generic parser framework.

Exclusion check: reverse propagation is blocked because aid/receipt/render values have no
conversion into admitted evidence; probe admits only controller-scoped observations; apply
cannot be built after refusal and otherwise retains its fail-toward-run behavior; admins receive
one bounded root-cause refusal rather than warning storms; oracle engineers retain attributed
malformed/decline diagnostics only after admission; reliable and unreliable oracles share the
same scope and resource checks, with disagreement meeting conservatively only inside an admitted
attempt. `NoObservation` and known divergence remain usable conservative states; unknown
integrity never permits continued mutation.

Non-goals: transport/SSH, multi-host fan-in, retries, cryptographic identity redesign, production
host IDs, cache/reuse, freshness windows, phase-four output sanitization, artifact-store policy,
whylog prose, serde/general schema machinery, and changing the records/whylog wire version except
where a format break is unavoidable and separately ruled. Forbidden conversions include
bytes/string/fixture/payload/durable/evidence to `AttemptScope`; refused/partial input to
`SiteResults`, `Observable`, `TrustedFootprints`, `Resolutions`, or plan input; and any public
constructor that assembles scoped evidence from independently supplied scope and value.

Unresolved ruling menu before execution:

- `297:rule-exact-default-budget-values`: accept the stated ceilings as exact spike defaults, or
  choose lower exact values before the builder writes boundary tests.
- `297:rule-empty-framed-observation`: classify a valid header-plus-sentinel with zero declared
  sites as `NoObservation` (recommended), while a zero-byte/headerless stream remains `Refused`.
- `297:rule-whylog-format-version`: decide whether strict token/repeated-block validation can stay
  `dorc-whylog/1` as a reader hardening or requires `/2`; do not silently make that compatibility
  choice in implementation.
- `297:rule-refusal-artifact-surface`: freeze the spike's no-apply-output behavior and diagnostic
  severity for attempt refusal; no builder-authored user-facing prose.

Execution rulings (continuity-owner, 2026-07-20):

- `297:rule-exact-default-budget-values`: the stated ceilings are the exact spike defaults and
  remain injectable for boundary/property tests.
- `297:rule-empty-framed-observation`: a valid, scoped header-plus-sentinel declaring zero sites is
  `NoObservation`; zero-byte and headerless inputs are `Refused`.
- `297:rule-whylog-format-version`: mint `dorc-whylog/2`. A bounded reader may recognize `/1` only
  to issue an incompatible-version refusal; there is no compatibility parser or replay.
- `297:rule-refusal-artifact-surface`: attempt refusal emits no apply artifact, decision digest, or
  whylog durable; emits exactly one structured error-floor diagnostic with explicitly empty
  builder prose; exits the CLI nonzero; and displays no raw host value before phase-four encoding.
- `297:rule-dependent-fixture-boundary`: dependent-crate tests may cross a narrow, non-default
  compile-time harness/dev boundary that is absent from the production CLI. Plain `cfg(test)` is
  insufficient across crate compilation boundaries. Fixture scope remains a distinct type with no
  conversion to production `AttemptScope` or production scoped evidence.

Stop if implementation needs scope parsed from host/durable payload, a production legacy bypass,
public fieldwise scoped-evidence construction, partial admission after a limit/integrity failure,
interner insertion before budget enforcement, `Refused -> Unknown/NoObservation`, a mutation-capable
plan after refusal, or new transport/fleet/retry identity semantics.

## serial-phase-placeholders

Phase four sink matrix/artifact store and phase five production fences remain intentionally
unfrozen. No builder may begin them from this packet.
