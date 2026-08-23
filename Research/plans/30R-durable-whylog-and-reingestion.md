# 30R - The whylog durable: projection, storage, reconstruction, and firefighter review

> Tier: LLM-authored plan from the 2026-08-23 human design dialogue and the r30
> durable review. Root human docs and `spike/CLAUDE.md` outrank it. This is the
> complete ordinary-engineering implementation surface; its quarantined sibling
> `30Ra` carries additional rationale that builders do not need.
>
> Scope: the final shape of one `.whylog`, the projection from Spine, filesystem
> publication, bounded read-back, reconstruction, and `dorc why` across current and
> historical runs. This plan does not select a serialization or encryption format.
>
> Further rationale and details are available in `quarantine/30Ra`; to be read
> *only* if you have been given explicit access to the quarantine. Otherwise,
> everything you need to know is in this document.
>
> Grades: **[TYPED]** the human typed it; **[ACKED]** the human accepted the
> substance; **[DIRECTIONAL]** a strong lean remains intentionally adjustable;
> **[PROPOSED]** implementation shape to validate against the live tree.

## 30R:the-design-in-one-screen

One invocation writes one immutable, bounded, readable whylog. It is a lossy
projection of Spine, but the projection is deliberately rich: every truthfully-minted
stable semantic species receives a durable view, together with stable narrative and
locator records. The durable records what the invocation concluded; replay reconstructs
what it can and compares rather than replacing history.

One file format carries two distinct projections:

- **rich**: the default; structural records remain readable while fields capable of
  carrying opaque runtime/source text are encrypted;
- **plain**: explicitly requested or selected after encryption failure; rich-only
  fields do not exist in this projection, and the remaining document is readable by
  eye before sharing.

The plain projection is not the rich projection with encryption disabled. Projection
types and field classification make that impossible. Encryption failure may remint the
plain projection and record the fallback; it never emits a rich-only field plaintext.

Source text remains reconstruction-first rather than copied into the base durable.
Arbitrary stdout/stderr not represented by an oracle contract remains uncollected by
default. Historical cleanup is explicit user work, never a side effect of an unrelated
plan/apply/why invocation.

## 30R:standing-invariants

- **`inv-debugging-detail-has-no-sensitivity-guarantee`** - any opaque authored/runtime
  value may contain material Dorc cannot classify. Each projection promises only which
  field classes it contains; it never promises generic scrubbing or safe sharing.
- **`inv-reingested-material-never-authorizes-action`** - read-back values are recorded
  report values only, with no conversion into live claims, accounts, licenses,
  `PlanAuthority`, probing, artifacts, or apply.
- **`inv-recorded-and-rederived-remain-distinct`** - every consumer preserves
  recorded-only, rederived-only, both-agreeing, and both-disagreeing as distinct states.
- **`inv-whylog-collection-never-expands-observation`** - writing a receipt persists only
  material the invocation already holds; it performs no additional host/controller
  observation.
- **`inv-unaccounted-output-stays-remote-by-default`** - stdout/stderr outside the
  oracle-accounted channels is not transported or persisted by default; later pull is a
  separate operation about the later world.

## 30R:current-r30-starting-point

The implementation baseline is `ai/r30-conduct` after the influence-carriage fold:

- sixteen Spine species;
- four production durable views: invocation, record stream, disposition, digest;
- twelve transitory species, four currently unminted;
- per-entity `InfluenceAccount` carriage with no decision consumer;
- a disposition-account wire/read round-trip behind `ACCOUNT_EXPORT = false`;
- one line-framed v2 grammar with bounded outer and nested record parsing; and
- one default-on per-user store with exclusive creation and automatic count pruning.

**`30R:rul-enable-influence-export` [ACKED]** - enable the already-built export when
this plan lands. The final format carries recorded influence on every semantic record,
not only dispositions.

## 30R:one-spine-many-durable-views

`SpineSpecies::census_arm` remains the closed species census. `DurableView` remains the
only route from a semantic record to a whylog field. Records never serialize themselves.
A new species cannot land unclassified; entering the durable arm or adding a View field is
a reviewable diff.

`New` means unreviewed/transitory, not permanently omitted. This plan's target is:

- every minted stable semantic species has a durable view;
- every view carries the record's `RecordedInfluence` projection;
- unminted species say so until a truthful writer exists;
- process-local handles are resolved into stable IDs/locators before projection;
- omitted fields/species produce durable omission records; and
- no recorded type implements a conversion to its live decision-plane cousin.

### Species matrix

| Species | Final durable view |
|---|---|
| Invocation | Producing invocation identity, replay recipe, argv according to projection, ordered source manifest, controller semantic identity, analysis policy, target/context/generation, timestamps. |
| Record stream | Typed admission result, correspondence/arrival metadata, and exact already-admitted bytes in rich mode. |
| Disposition | Full `SiteId`, stable source locator, disposition, recorded influence, predicted/actual kind, compact recorded authority account. |
| Digest | Named collision-resistant decision identity at real persistence boundaries; fixture digest remains fenced. |
| Load decision | Load occurrence, selected/withheld binding, source identity, custody, exactness/havoc, and placement consequence. |
| Site classification | Stable class code, verdict lane, invalidator ownership, cell account, degradation cause. |
| Solve certification | Per-pass outcome, trip latch, bounded failure indexes/summaries; optional bounded transition trace. |
| Vouch | Site/fact, exact body/source identity, custody, attached/suspended result and decline. |
| Probe ship | Site, lane, defining source, context and refusal/unresolvable reason. |
| Admission | All outcomes, including refused, exact fault and available report-only material. |
| Observation | Site/fact observation, merge/collapse result and references into received records. |
| Validity/effective round | Round ordinal, proof/erasure references, certification and cascades. |
| Survival | Complete crossings, footprint/backing inputs, resolver/reach contributions, reference result, consent and authority account. |
| Render decision | Binding, refusal, neutralisation, defensive emission, import rewrite, placement, artifact form and fallback. |
| Region decision | Region identity, decision, complete keyed/unkeyed route population and contributor references. |
| Outcome | Run/exit outcome, refusal state, advisory route, write result and apply eligibility. |

Stable `CollapseNarrative` views and the locator DAG are durable siblings of the species
table. They carry stable IDs and semantic operands, never arena ids or pointers.

## 30R:recorded-influence

The wire type is separate from `InfluenceAccount`:

```text
RecordedInfluence =
   AuthoredBeforeContact
 | HostInfluenced
 | Untracked
 | Missing
 | Unknown(token)
```

It preserves exactly what the historical record said. It provides display and a
conservative `was_influenced_or_unknown` query only. It has no conversion, join, mint, or
deserialization path into `InfluenceAccount`.

Missing/malformed/unknown reads at the most-influenced report posture, but the render keeps
the original state distinguishable rather than rewriting it to `Untracked` silently.

## 30R:rich-and-plain-projections

**`30R:rul-one-readable-file` [ACKED]** - structural framing stays plaintext and
inspectable. Whole-file encryption is out.

**`30R:rul-two-projections-one-grammar` [ACKED]** - rich and plain use the same record
vocabulary and one parser, but separate constructors and closed field-class tables.

Suggested field classes, subject to the format research:

```text
Structural
   fixed tags, enums, record kinds, schema versions, bounded counts,
   ordinals, stable numeric ids, digests, omission markers

OpaqueValueCapable
   argv values, paths, entity/canonical values, source excerpts, report
   tails, tool errors, raw bytes, rendered foreign text

RichOnly
   exact admitted opaque blocks, optional anomaly traces, any future
   explicitly collected output
```

Rich mode leaves Structural fields plaintext and encrypts each OpaqueValueCapable/RichOnly
field or reviewed field group. Plain mode omits RichOnly and emits only fields its own type
admits. For an omitted opaque field it may emit Structural metadata such as presence,
length/count and stable source id. Content commitments require separate review because
low-entropy values can be guessed.

The file records `projection=rich|plain` and `fallback=encryption-unavailable|none` in
Structural fields. A rich write either completes with every encrypted field authenticated or
falls back by constructing a new plain projection from Spine. It never continues the rich
writer after an encryption failure.

## 30R:format-research-before-format-choice

No representation is selected here. Run a live research round over academic and shipped
formats before choosing line framing, CBOR, protobuf, another format, or a focused custom
container.

The research compares:

- bounded streaming decode before allocation;
- exact binary values and no forced UTF-8 conversion;
- definite lengths, checked arithmetic and nesting limits;
- deterministic Structural encoding independent of encryption randomness;
- readable framing around encrypted fields;
- authenticated binding of encrypted value to record kind, field name, scope, ordinal,
  schema version and declared length;
- unknown-field retention and visible incompleteness;
- partial-damage behavior and final-completion detection;
- dependency size, auditability, test vectors and independent implementations;
- Windows/macOS/Unix support; and
- inspection/export tooling.

The plan is pre-user. Once selected, reshape v2 in place as one change; do not carry a
compatibility parser or adapter merely for the spike.

## 30R:source-reconstruction-first

**`30R:rul-reconstruction-is-the-default` [DIRECTIONAL]** - the base whylog stores
ordered source paths, content identities and stable locators, not source bytes by value.

Replay source outcomes are typed:

```text
CurrentMatching
CurrentDrifted
Absent
Unreadable
NotSelected
```

Recorded conclusions remain renderable in every outcome. Current source is an optional input
to rederivation, never a prerequisite for reading history.

A durable path is a hint, not a local read capability. The reader selects/confines source
reads under controller policy, bounds and regular-file checks before comparing identity. A
durable alone does not nominate arbitrary controller files for opening.

A dislocated content-addressed source cache remains OPEN research. It is never an implicit
part of whylog writing. Any future design needs explicit enablement, inventory, source-byte
visibility and explicit cleanup; a whylog remains useful when the cache is absent.

## 30R:accounted-and-unaccounted-output

**`30R:rul-unaccounted-output-defaults-to-remote` [DIRECTIONAL]** - arbitrary
stdout/stderr outside oracle-accounted channels is not collected in ordinary plan/apply.

Already-transported contracted result/report bytes keep the existing admission split:

- recognized closed records become typed observations;
- unknown/freeform tails remain bounded opaque material;
- rich mode may preserve the exact admitted opaque block in encrypted fields;
- plain mode preserves only its Structural account; and
- refused intake can still write a report-only whylog without producing a plan.

An explicit future collection mode is selected before transport and has its own cost/consent
surface. Debug pull after the run is a separate invocation and its values are labelled as
later observations, never historical replacements.

## 30R:one-immutable-file-per-invocation

**`30R:rul-one-whylog-per-invocation` [DIRECTIONAL]** - each invocation publishes
one immutable file. A later apply never updates a plan whylog.

Each file carries:

- invocation id;
- decision/artifact identities it produced or consumed;
- explicit predecessor/reference ids known at invocation time;
- per-target attempt/generation/exchange scopes; and
- any user-event/correlation id once that concept is designed.

`dorc why` correlates immutable files by exact identities. Correlation creates an explanation
graph only; it does not merge attempts, generations, influence or authority. A single
invocation may cover many targets, but every target-derived record carries its full scope and
cross-target conclusions list their input scopes explicitly.

No mutable `latest` file/pointer is required. `why --last` selects the newest committed file
after a bounded complete enumeration.

## 30R:explicit-cleanup-only

**`30R:rul-cleanup-is-explicit` [DIRECTIONAL]** - ordinary Dorc invocations do not
delete historical receipts. Remove current automatic keep-count pruning when this design
lands.

Each file has hard write/read limits. The store may report its accumulated file/byte count.
Deletion belongs to an explicit pull operation, provisionally `dorc clean`, with preview,
user-selected scope and visible failures. No daemon, timer, background cleanup, age expiry or
cleanup piggybacked on another task is required.

## 30R:storage-publication-contract

One cross-platform store abstraction owns platform-specific implementation and shared
behavioral tests. The contract is the same on Windows, macOS and Unix:

1. select/create a private per-user directory and retain an ownership-bearing handle;
2. exclusively create a private temporary/unnamed object in that directory;
3. stream the bounded document and finish/authenticate every encrypted field;
4. flush and synchronize the complete object;
5. publish one immutable final name atomically without replacement;
6. synchronize the directory where needed by the platform contract;
7. keep partial/uncommitted names invisible to selection;
8. enumerate completely up to `limit + 1`, reporting overflow/errors rather than truncating;
9. never follow replacement links/reparse points during create, publish, read or cleanup;
10. delete only objects whose store identity/ownership is established; and
11. report write, synchronization, enumeration and explicit-clean failures non-fatally.

Filesystem, clock, key access and randomness stay at injected edges. The semantic projection
is deterministic; encryption/publication nondeterminism is outside it.

## 30R:bounded-read-and-field-status

The reader enforces independent limits for:

- total file bytes;
- Structural framing line/frame bytes;
- encrypted field bytes;
- decrypted field bytes;
- record/species counts;
- per-target/per-exchange counts;
- nesting/depth;
- retained metadata/allocation;
- numeric width; and
- optional decompressed bytes if compression is ever admitted.

Every field read has a closed status:

```text
PresentPlain
PresentEncrypted
UnavailableKey
AuthenticationFailed
Malformed
OmittedByProjection
UnknownField
Missing
```

Unavailable/damaged rich fields do not make the Structural document disappear. They make
reconstruction explicitly incomplete. No incomplete file is presented as a complete replay.

## 30R:recorded-versus-rederived

The durable keeps inputs and recorded conclusions. Replay computes current conclusions from
available inputs and compares; it never substitutes recorded conclusions into the kernel.

```text
RecordedOnly
RederivedOnly
Agree { recorded, rederived }
Disagree { recorded, rederived }
```

Controller build/semantic identity is part of the comparison. A conclusion rederived by a
different engine is labelled accordingly. Disagreement is a finding and remains visible at
summary, per-line why, route/witness expansion and export.

Every `Recorded*` type is display/report-only. No `From`, deserializer, generic conversion or
helper yields a live claim, influence account, vouch, license, `PlanAuthority`, executable
artifact, probe request or apply request.

## 30R:firefighter-why

`dorc why` must remain useful when source drifted, encrypted fields are unavailable, an old
engine is not present, or rederivation disagrees. Its data priority is:

1. the recorded historical conclusion and its recorded influence/scope;
2. the exact stable contributor/witness identities the durable retained;
3. current reconstruction and comparison, where available;
4. explicit omissions/unavailable fields; and
5. optional later observations, clearly dated as later.

The highest-value durable records are survival witnesses, universal region decisions,
render/artifact decisions, admission/refusal, probe-ship identity, vouch attachment, typed
observations and stable narrative operands. They explain why an authored mutation did or did
not appear without requiring raw output.

Every rendered host/source-derived value still passes through its destination-specific
encoder after canonicalization. Encoding does not change the record's influence or field
status.

## 30R:projection-omissions

The durable itself records what it omitted. Each omission names:

- species and field class;
- count/byte count where known;
- projection (`rich` or `plain`);
- reason (`not-minted`, `plain-projection`, `limit`, `collection-disabled`,
  `encryption-unavailable`, `unsupported`, or explicit policy);
- whether deterministic reconstruction is possible; and
- stable source/scope references where available.

An omission account is not a substitute for its content. It prevents an absent record from
reading as "nothing happened."

## 30R:decision-document-and-world-identities

Keep three identities distinct:

1. decision identity: exact ordered source identities, policy, controller semantics,
   target/context/generation and exact executable artifact bytes;
2. whylog document identity: exact fields/bytes in this receipt; and
3. world freshness: current observations and guards.

Use collision-resistant identities at every non-fixture boundary. The deterministic fixture
digest and width-one identities stay structurally fenced. Recording a decision identity never
makes the whylog an executable/saved plan.

Reserve representation room for a previous-document root and independent witness receipt,
without requiring an external service in this phase.

## 30R:implementation-order

1. Enable the existing disposition influence export and delete the corresponding exclusion.
2. Run the live representation/encryption prior-art round; record the selected format and
   field-encryption algorithm separately.
3. Re-cut the pre-user grammar in place and define closed field classes.
4. Build distinct rich/plain projection types and the typed fallback.
5. Replace the storage/publication/read edge with the cross-platform contract above.
6. Add durable views for every minted stable Spine species, recorded influence, stable
   narratives and locator DAG.
7. Add truthful writers for vouch, observation, validity/effective rounds and outcome as their
   semantics firm; make refused admission durable report-only.
8. Build recorded/rederived comparison and degraded firefighter renders.
9. Add immutable cross-invocation correlation and multi-target scope.
10. Add explicit inspect/export and explicit cleanup commands.

## 30R:acceptance

- Every field/count/byte/depth limit is tested at boundary minus one, boundary and boundary
  plus one, on write and read independently.
- Removing encryption support or forcing encryption failure can produce only the plain
  projection; rich-only bytes never appear plaintext.
- Wrong/missing keys, modified encrypted fields, field substitution, truncation, duplicate
  fields and unknown schema members produce closed field/document statuses without panic.
- Source match, drift, absence and unreadability all preserve the recorded conclusion.
- All four recorded/rederived states render distinctly and survive every explanation depth.
- Recorded types have compile-fail/lexical fences against every authority-bearing sink.
- Target/attempt/generation swaps cannot acquire ambient scope.
- Publication tests cover concurrent writers, partial writes, crash points, final-name
  collisions, symlinks, junctions/reparse points, replacement races, overfull directories and
  explicit-clean failure on all controller families.
- No whylog path performs a new host call, environment sweep or unrelated controller read.
- Default transport and both projections contain no unaccounted stdout/stderr.
- Plain projection omissions are complete and the document remains readable by eye.
- Rich projection structural framing remains readable without decrypting opaque values.
- Both drivers (`main` and `WhyWorld`) consume one reader/model; no second replay semantics.

## 30R:open-items

- Exact serialization and per-field/group encryption representation: dedicated live research.
- Exact plaintext field classification, especially paths and source/entity display values:
  decide from shapes, never observed content.
- Local key storage/recovery and multi-recipient UX on all controller families.
- Optional content-addressed source cache: research only, explicitly enabled if ever built.
- Final correlation/user-event vocabulary.
- Exact explicit-clean command/spelling and preview UI.
- Whether bounded transition traces are anomaly-default or explicit-mode only.
- Whether previous-root/witness fields are built now or merely reserved.

## 30R:non-goals

- no generic secret detector or scrubber;
- no source bytes by value in the base projection;
- no default arbitrary stdout/stderr collection;
- no mutable whylog spanning plan and apply invocations;
- no automatic deletion during unrelated work;
- no whylog-derived plan, probe, apply or cache input;
- no cross-version compatibility layer for the pre-user v2 grammar;
- no daemon, background retention service or shared database; and
- no format choice before the focused research round.
