# 30Rb - Secure durable receipts: current-tree build specification

> Tier: quarantined, builder-facing implementation specification.
> Snapshot scouted: `ai/main` at `0630885b0b98`, 2026-08-24.
>
> This document answers "build what, where, in what order, with which type
> boundaries, today." The build conductor and every builder MUST read `30Ra` in
> full. `30Ra` owns the product/security design and rationale; this document does
> not restate it. Root human documents and later human rulings outrank both.
>
> `AGENTS.for-builders-only.md` remains independently binding. Every governed
> surface still receives its required review; this plan is not a substitute.
>
> The minimal production local key provider, receipt store, product assembly, and
> testing/fault discipline are specified only by `30Rd`. Builders touching that
> edge MUST read `30Rd` in full; this document schedules it and does not restate it.

## 30Rb:how-to-read-requirements

The labels below are normative:

- **REQUIRED** - an effect, boundary, ordering, or verification property. A
  builder may not trade it away locally.
- **TYPE LEAN** - the preferred concrete Rust spelling. Builders may improve the
  spelling while preserving every listed effect and reporting the deviation.
- **IMPLEMENTOR CHOICE** - intentionally local latitude. Choose the smallest
  readable implementation that preserves the required effects.
- **V1 STRETCH** - the human's gentle lean to carry more already-existing,
  purpose-bearing semantic state. Build after the core route is coherent; the
  conductor may cut a row that materially threatens V1 completion, but must keep
  an explicit projection-omission record and may not replace it with an empty stub.
- **LATER** - explicitly outside V1. Do not add a stub, empty field, compatibility
  path, or speculative public API for it.
- **STOP** - return to the conductor before proceeding.

When a TYPE LEAN conflicts with a REQUIRED effect, the effect wins. When a type
blocks a shortcut, treat that as the type doing its job, not as permission to add
an escape constructor.

## 30Rb:result-and-exit

V1 exits only when all of the following hold:

1. A standalone `dorc-receipt` crate owns the receipt models, exact format,
   identities, projections, graph, and reader/writer states; narrow sibling crates
   own concrete crypto adapters and the local durable edge without entering the
   analyzer dependency graph.
2. `PlanReceipt`, `ApplyIntent`, and `ApplyOutcome` all round-trip as signed plain
   and signed rich receipts under one `dorc-receipt/1` grammar.
3. Rich receipts contain one Age-armored reverse overlay; plain receipts cannot
   represent or parse one.
4. One exact single-stream and one exact multi-file `ApplyArtifactImage` round-trip
   without rebundling, normalization, deduplication, relocation, or byte changes.
5. A plan route writes, reads, and explains a `PlanReceipt` from the real current
   plan pipeline.
6. An apply route writes `ApplyIntent`, consumes the first-dispatch permit through
   DST/hostsim, records `ApplyOutcome` or explicit missing outcome, then explains
   the correlated graph.
7. Receipt read-back has no conversion to live evidence, influence accounts,
   vouches, licenses, `PlanAuthority`, plans, artifacts, probing, caches, or
   mutation.
8. The `30Rd` minimal production baseline can initialize/reopen its versioned local
   keyset, publish/read versioned receipts, and complete real-binary process-restart
   plan/why and apply/why routes on Windows and Unix.
9. The old whylog format, parser, writer, store assumptions, fixtures, replay
   authority, FNV decision digest, and compatibility paths are deleted.
10. The required conformance, mutation, compile-fail, DST, e2e, and existing
   Windows/WSL builder-completion gates are green without re-blessing unrelated
   output. This includes `30Rd`'s concrete Windows/Unix baseline tests under the
   repository's ordinary `mise run both gate:full-quiet`; it does not imply the
   deferred full native Windows/Unix/macOS or power-loss filesystem matrix.

Temporary coexistence is allowed only inside an unlanded construction branch. No
folded stage may expose two production readers or writers.

## 30Rb:current-tree-replacement-map

| Current surface | Current fact | V1 action |
|---|---|---|
| `core/src/spine.rs` | Sixteen sealed species; four old durable views; twelve newer species | Keep Spine live and authority-bearing. Project a richer report-only receipt view; do not serialize Spine records directly. |
| `plan/src/whylog.rs` | Entire `dorc-whylog/2` grammar, parser, writer, `ApplyLine`, `DurableAccount`, and replay envelope | Delete only in Stage 6 after Stage 5A/`30Rd` D4 proves the concrete production replacement. Reuse tested bounded-parser techniques, not types or tokens. |
| `plan/src/spine.rs` | `PlanAuthority::of_admitted_replay()` re-authorizes replay | Delete. Reingested bytes must enter a report-only derivation route. |
| `plan/src/erasability.rs` | One FNV-1a-64 `decision_digest` over the current canonical identity plane | Delete the FNV digest. Preserve the erasability differential, but use the one SHA-256-backed `PresentedPlanId` identity path. |
| `cli/src/artifact.rs` | `ArtifactSet` owns final `plan.sh` plus deterministic dependencies, but drops roots/edges/modes | Retain the live emission type; carry enough topology to mint an exact `ApplyArtifactImage`. |
| `cli/src/main.rs` | Scattered whylog write/replay flow and raw single-stream remote apply | Replace with typed receipt projection and apply orchestration. |
| `cli/src/whylog_store.rs` | Direct env/fs default store, indexed names, retention deletion | Replace with `30Rd`'s versioned local store. Reuse mechanisms only where they satisfy `30Rd`; carry no old indexing, retention, flags, or format assumptions. Delete the old module only after the concrete restart routes are green. |
| `cli/src/results.rs` | Width-one live attribution and replay claim matching | Keep live intake scope. Recorded receipt claims never mint or recover this scope. |
| `cli/src/why.rs`, `world.rs`, `lib.rs` | `Receipt`, `ReplayLoad`, `DriftedReceipt`, digest-only disagreement | Replace with `Reingested<T>`, `ReceiptGraph`, source-resolution states, and recorded/re-derived comparison. |
| `dorc-loom/src/consumer.rs` and its tests | Directly parses old whylog fixtures through `dorc_plan::whylog`; the old CLI store still contains a stale comment naming a no-longer-present loom-side store | Move to receipt reader/report adapters and new receipt fixtures; delete every direct old-reader use, old flag parser, and stale cross-reference. |
| `cli/src/transport_edge.rs` | `ship_apply` calls `SessionDriver::run` directly; no affine boundary | Place one global `MutationDispatchPermit` immediately above the first call that may dispatch mutation. |
| `transport/src/lib.rs` | Single borrowed artifact stream and closed session outcomes | Preserve as DI. Do not teach transport receipt authority; orchestrator consumes the permit before calling it. |
| `hostsim`, `transport::SimDriver` | Deterministic modeled host and scripted transport outcomes | Use together for the V1 apply route; no real mutator is executed. |
| `aid/tests/whylog-*.loom` and drift cases | Old-format user surfaces | Rip and replace with receipt cases. Builders add empty prose registers only. |

Snapshot anchors for scouts/builders (line numbers are for `0630885b0b98`; names
are the durable reference):

- `plan/src/whylog.rs:35-153` - old tags, `ApplyLine`, account export, and
  recorded influence flattening;
- `plan/src/whylog.rs:271-335, 444-520, 648-770, 852-920` - old limits,
  envelopes, projection, writer, and admission API;
- `plan/src/spine.rs:40-112, 114-198` - plan authority and projection;
- `plan/src/erasability.rs:91-178` - canonical decision and FNV digest;
- `cli/src/artifact.rs:265-320, 897-975` - `ArtifactFile`, `ArtifactSet`,
  `Selection`, and `with_plan`;
- `cli/src/transport_edge.rs:194-315` - direct apply dispatch and outcome;
- `transport/src/lib.rs:179-280` - `SessionRequest`, `SessionOutcome`, and DI
  trait;
- `cli/src/whylog_store.rs:37-73, 103-301` - default location, bounds,
  exclusive create, and retention;
- `dorc-loom/src/consumer.rs:779-795, 1058-1085, 1190-1234` - old
  in-process whylog consumer and parser.

## 30Rb:crate-and-dependency-cut

### Required dependency direction

```text
dorc-core

dorc-receipt -> sha2
dorc-receipt-crypto -> dorc-receipt, ed25519-dalek, age
dorc-receipt-local -> dorc-receipt, dorc-receipt-crypto

dorc-plan -> dorc-core, dorc-receipt, ...
dorc-cli  -> dorc-plan, dorc-receipt, dorc-receipt-crypto,
             dorc-receipt-local, dorc-transport, ...
dorc-loom -> dorc-cli, dorc-receipt, ...       (test/authoring consumer)
dorc-hostsim -dev-> dorc-receipt            (only for receipt DST fixtures)
```

**REQUIRED:** `dorc-receipt` never depends on `plan`, `cli`, `hostsim`,
`transport`, `aid`, filesystem/config crates, or user-configuration code. It owns
no environment, network, clock, path lookup, or persistent-store operation.

**REQUIRED:** `dorc-receipt` has no Dorc-internal production dependency. Live
types are projected by `plan`/`cli` adapters into receipt-owned recorded types.
Do not introduce a `dorc-core` edge to avoid writing that explicit boundary.

**REQUIRED:** `aid` does not depend on `dorc-receipt`. `cli` converts the
report-only receipt graph into existing aid/weft parts. This keeps Age randomness
and crypto dependencies out of the describe-plane dependency graph.

**REQUIRED:** the additional boundaries, dependencies, and fixture/production
fences at the local durable edge are exactly `30Rd:component-and-dependency-boundaries`
and `30Rd:test-and-fixture-fences`.

### Dependency selection constraints

Builders select current maintained Rust implementations, pin exact versions and
features in the committed lockfile, and include the resulting dependency/feature
diff in review. Required adapter constraints:

- Age exposes only one X25519 recipient/identity path plus canonical ASCII armor.
  Do not expose passphrases, SSH recipients, plugins, async, CLI helpers, remote
  recipients, or algorithm choice.
- Ed25519 uses ordinary signing and strict verification. The receipt crate exposes
  no concrete key generation or serialization. The crypto sibling enables only the
  V1 PKCS#8/zeroization surfaces required by `30Rd`; legacy compatibility, batch,
  prehash, hazardous, serde, and PEM convenience stay off.
- SHA-256 owns source/content IDs, skeleton digest, key ID derivation, presented
  plan identity, and apply-image identity behind domain-specific wrappers.
- Receipt-ID entropy and private-key generation are separate injected production
  capabilities with non-convertible outputs. `cli::receipt_edge::OsReceiptIdSource`
  remains the sole receipt-ID adapter; `30Rd` owns the separate complete key-generator
  edge. Tests and DST inject both, and no kernel calls OS randomness.
- DSSE PAE may be implemented directly from its tiny standardized injective
  construction or through a narrow maintained implementation. Do not import a
  Sigstore object ecosystem or create a second envelope merely to obtain PAE.
- Lowercase-hex encoding is an encoding detail, not a second cryptographic
  primitive; use the smallest reviewed implementation.

The Age adapter is the one accepted nondeterministic implementation edge. The
format, overlay validator, identity encoders, parser, and all decisions remain
pure. Calls into Age happen only through `OverlaySealer`/`OverlayOpener` injected
at writer/reader transitions.

## 30Rb:implementation-module-map

TYPE LEAN for the new crate:

```text
spike/crates/receipt/
  Cargo.toml
  CLAUDE.md
  src/
    lib.rs             exports only deliberate receipt API
    ids.rs             receipt/species/key/content identities
    limits.rs          typed byte/count/depth limits and budgets
    model.rs           species/projection sealed traits
    plan.rs            PlanReceipt recorded model
    apply.rs           ApplyIntent and admin M:N assignment model
    outcome.rs         ApplyOutcome recorded model
    image.rs           ApplyArtifactImage and lossless container
    projection.rs      Plain/Rich and opaque-field accounting
    overlay.rs         reverse overlay model and validator
    format.rs          exact skeleton/inner grammar and DSSE PAE
    writer.rs          affine writer states
    reader.rs          monotone reader states and partial outcomes
    reingested.rs      non-extractable Reingested<T>
    graph.rs           report-only M:N ReceiptGraph
  tests/
    conformance.rs
    mutation.rs
    apply_image.rs
    crypto_interop.rs
    authority_seals.rs
    fixtures/
```

Small modules may be combined when that improves scanning. Do not combine the
reader and writer, the overlay and skeleton parser, or recorded and live adapters.
Concrete Age/Ed25519 adapters live in the dependency-outward `dorc-receipt-crypto`
sibling. `30Rd` alone specifies the additional `dorc-receipt-local` modules.

New adapter files to add:

```text
plan/src/receipt.rs          Spine/Plan -> PlanReceipt projection
cli/src/receipt_edge.rs      IDs, signer/sealer, source/sink orchestration
cli/src/apply.rs             session -> intent -> permit -> outcome orchestration
cli/src/why.rs               ReceiptGraph -> aid/weft render projection
```

## 30Rb:receipt-identity-map

### Receipt graph identity

TYPE LEAN:

```rust
struct ReceiptId([u8; 32]);          // private shared representation
struct PlanReceiptId(ReceiptId);
struct ApplyIntentId(ReceiptId);
struct ApplyOutcomeId(ReceiptId);

struct KeyId([u8; 32]);              // private shared representation
struct SigningKeyId(KeyId);
struct EncryptionKeyId(KeyId);
```

**REQUIRED:** These are collision-resistant controller-minted receipt identities,
not content hashes and not payload claims. The receipt crate receives them through
an injected `ReceiptIdSource`; deterministic fixture sources are structurally test
only. No fixed-ID constructor reaches CLI production paths.

Signing and encryption key IDs are distinct public newtypes over a private
representation. They derive with separate SHA-256 domains from the exact public
verification key and Age recipient encoding. No conversion exists in either
direction. A key ID aids lookup only; the configured resolver chooses the concrete
key capability and trust, never the receipt.

### Planning and presentation identity

TYPE LEAN:

```rust
struct PlanningInputId(Sha256Digest);
struct PresentedPlanId(Sha256Digest);
struct ApplyArtifactImageId(Sha256Digest);
```

`PlanningInputId` identifies the complete input tuple the planner consumed.

`PresentedPlanId` identifies one complete approval surface. It is mintable only
after:

- collection/admission has reached a terminal non-refused state;
- settlement is quiescent;
- the solve-certifier latch is spent;
- every site and region decision is final;
- target specialization represented by that surface is final;
- artifact form, import edits, render refusals, and exact executable bytes are
  final; and
- human and executable views are projections of that same final state.

It is not one Spine row, one leaf decision, one target assignment, or an identity
that follows an admin edit. Replace current `SpineDigest`/FNV with this one path;
do not retain both.

To keep `core` dependency-free while retaining the identity in the one complete
Spine, widen `DecidePlane` with an associated presented-plan identity and replace
the stringly `SpineDigest` with a generic `SpinePresentedPlan<P>` record. In the
plan instantiation, `PlanPlane::PresentedPlanIdentity = PresentedPlanId`. The
record is set only after final artifact/presentation material exists; no
provisional settlement round can mint it.

`ApplyArtifactImageId` identifies exact bytes and topology the apply will use.
An unchanged apply has the same planned and actual image ID. An admin edit changes
the actual image ID while retaining the originating presented-plan ID for
narration.

The V1 domains are exact:

```text
PlanningInputId       application/vnd.dorc.receipt.v1.planning-input
PresentedPlanId       application/vnd.dorc.receipt.v1.presented-plan
ApplyArtifactImageId  application/vnd.dorc.apply-artifact-image.v1
SigningKeyId          application/vnd.dorc.receipt.v1.signing-key-id
EncryptionKeyId       application/vnd.dorc.receipt.v1.encryption-key-id
```

Each ID is SHA-256 over DSSE PAE of its domain plus one exact injective component
encoding. Stage 0 MUST commit one exhaustive identity table fixing every component,
field order, width/length framing, source byte encoding, and empty/absent spelling for
`PlanningInputId` and `PresentedPlanId`, plus valid and cross-field-substitution
vectors. The table has no wildcard/default and is reviewed before Stage 1 mints either
ID. `ApplyArtifactImageId` remains fixed by `30Rb:apply-artifact-image-format`.

All five use independent domain-separated encodings. A `[u8; 32]` from one domain
cannot construct another domain's public newtype.

No public constructor accepts any of these three IDs. Their sole mints consume the
complete typed identity material and hash it in the same operation. On read,
`PlanningInputId` and `PresentedPlanId` are signed recorded claims: recompute and
compare when the exact identity material is available, preserve recorded-only when
it is not, and report disagreement rather than silently selecting either. They
never authorize. A rich `ApplyArtifactImageId` is different because its complete
image is present by value: parsing must recompute it before exposing the image.

### Admin-owned plan-to-target mapping

The admin, not Dorc, owns the mapping from presented plans to applied targets.
The receipt graph is therefore M:N, not a one-plan-to-many-intents tree.
This is a human correction made during 30Rb planning (2026-08-24) to `30Ra`'s
illustrative singular-origin graph: an admin may duplicate, compose, hand-edit,
cross-apply, or decline Dorc's target specialization before one coherent apply.

TYPE LEAN:

```rust
enum OriginatingPlans {
    Unavailable,
    Known(NonEmpty<PlanOriginOccurrence>),
}

struct PlanOriginOccurrence {
    ordinal: OriginOrdinal,
    receipt: PlanReceiptId,
    presented: PresentedPlanId,
}

struct PendingApplyAssignment {
    ordinal: AssignmentOrdinal,
    target: ReadyApplyTargetId,
    image: ApplyArtifactImage,
    origins: OriginatingPlans,
}

struct ApplyIntent {
    id: ApplyIntentId,
    session: ApplySessionId,
    assignments: NonEmpty<RecordedApplyAssignment>,
    // controller semantics, generation, policy, and commitment follow
}
```

Required behavior:

- Duplicate origin occurrences are legal and retained; no set/dedup semantics.
- One presented plan may feed many assignments and targets.
- One assignment/image may compose zero, one, or many presented plans.
- A plan specialized for host A may be assigned to host B; record both truths and
  render the mismatch, but do not rewrite provenance or pretend Dorc chose it.
- `Unavailable` is explicit. Empty `Known` is unrepresentable.
- Actual assignments are controller-attributed apply context, never inferred from
  plan payloads or receipt filenames.
- `RecordedApplyAssignment` has no public constructor. It is minted only by
  consuming `ApplySessionReady` together with `PendingApplyAssignment`s; each
  target ID must name exactly one member of that aggregate session, and the mint
  copies that member's resolved target/context/session identity into the record.

## 30Rb:critical-type-effect-map

The names are TYPE LEAN; the effects are REQUIRED.

| Type family | Required effect and forbidden state |
|---|---|
| `Receipt<D,P,T>` | The only complete reader result. `D`, `P`, and `T` implement private sealed traits; `D` supplies its exact associated recorded model and projection capabilities. Exact signature checked, exact species/projection parsed, and rich overlay completely validated where applicable. No `CompleteReceipt` sibling. |
| `PartialReceipt<R>` | Distinct damaged/unavailable state. No conversion to `Receipt`; no field-level selective promotion. |
| `Reingested<T>` | Marks every durable read-back value. `T` must implement a private sealed recorded-type trait, and no public generic constructor exists. No `Deref`, `AsRef<T>`, `Borrow<T>`, generic `map`, `into_inner`, or raw accessor. Species-specific decomposition returns another `Reingested<U>` or a report-only scalar. |
| `ReceiptGraph` | Holds only reingested receipts/partials and graph findings. No authority methods or conversion to live types. |
| `Plain` / `Rich` | Sealed projection parameters with associated overlay shape. `Plain -> NoOpaqueOverlay`; `Rich -> ExactlyOne<EncryptedOpaqueOverlay>`. |
| `ReceiptSignatureChecked<T>` | Exact DSSE-PAE body verified with a controller-supplied key already carrying sealed trust marker `T`. Private fields and constructors; still unparsed and not truth. |
| `TrustedReceiptSigner` / `SelfAssertedReceiptSigner` | Private sealed markers for controller policy provenance of the verification key. The key resolver's two private mints are the only constructors; receipt bytes and generic callers cannot select or upgrade the marker. |
| `BoundedReceiptBytes` | Aggregate input bound enforced before parsing/allocation. |
| `ReceiptOrderToken` | Exactly 20 authenticated decimal digits from the injected controller clock. Store selection only: no authority, freshness, graph edge, or receipt-ID conversion. A local filename must match it. |
| `LocatedReceiptEnvelope` | Only exact byte spans and bounded prefix tokens located. No semantic fields interpreted. |
| `ParsedReceiptSkeleton<D,P,T>` | Exact checked body parsed under the species/projection grammar used for signature domain. |
| `OpaqueOverlay` family | Opaque values inaccessible until Age authentication and complete reverse-overlay validation. |
| `ApplyArtifactImage` | Non-empty exact image, paths/bytes/topology owned by value. Its private `ApplyArtifactImageId` is computed only from the validated canonical image encoding in the same constructor; callers never supply the ID. No digest-only or archive reference constructor. |
| `RecordedApplyPath` | Exact target-relative path bytes, never a controller path or a file-open capability. |
| `ApplyTopology` | Entrypoints, roots, entries, and dependency edges validated in both directions. No dangling IDs or implicit dedup. |
| `ApplySessionReady` | One aggregate per `dorc apply` invocation, containing a non-empty set of fully resolved target/session/context records. Not one per target. |
| `PreparedApplyIntent` | Exact image assignments, origins, policy, controller semantics, session, target context, and generation frozen. |
| `IntentPublicationGate` | Closed `Published` or explicit `ConfiguredBypass`; attempted/failed publication is neither. The local `Published` arm consumes one private value binding exact intent receipt, image-account witness, requested policy, and `30Rd` required local publication proof; callers cannot pair these after the fact. |
| `MutationDispatchPermit` | Non-`Clone`, one use, minted atomically from the complete publication-gate value and consumed immediately before the first potentially mutative dispatch globally. |
| `MutationDispatched` | Authority-spent phase after permit consumption. Durable-only failure no longer aborts coherent orchestration. |
| `ReceiptSigner`, `ReceiptVerifier`, `OverlaySealer`, `OverlayOpener`, `ReceiptSink`, `ReceiptSource` | One-purpose injected capabilities. No generic crypto/store provider, key acquisition, algorithm selection, fallback, or environment lookup. The concrete local implementations narrow through `30Rd`'s typed name, bound, ownership, and publication-proof APIs rather than exposing the weak raw string/`Vec` trait shapes at the CLI. |

### Mandatory negative API rules

Do not implement any of the following on identity, trust, completeness,
publication, overlay-release, reingestion, or dispatch types:

- `Default`;
- public fields;
- serde deserialization;
- broad `From`/`Into` across domains;
- `Clone` on affine writer/publication/permit/plaintext states;
- conversion from recorded to live values;
- constructors from bare `String`, path, digest text, or caller-selected enum tag;
- an unchecked test constructor visible to production crates.

Compile-fail tests must pin the most tempting violations across a real crate seam.

The sealed species trait owns the exact associated model for each projection. In
particular, `ApplyIntent`'s rich model carries a private
`ExactApplyImagesPresent` capability minted by complete image-slot accounting;
the plain model has no such associated capability. Required dispatch consumes
that capability from a published rich intent. A generic
`Receipt<ApplyIntent, P, T>` is never sufficient by itself.

## 30Rb:writer-state-map

TYPE LEAN:

```text
DraftReceipt<D, P>
  -> SerializedReceipt<D, P>
  -> SignedReceipt<D, P>
  -> PublishedReceipt<D, P, Grade>
```

Required transitions:

1. Species-specific projectors create `DraftReceipt`; arbitrary callers cannot
   fill a bag of fields.
2. Plain serialization proves no opaque field is captured and emits no overlay.
3. Rich serialization consumes one non-cloneable overlay plaintext, seals it once,
   and emits exactly one canonical Age armor block.
4. `SerializedReceipt` owns immutable exact skeleton/body bytes. No semantic value
   can mutate after this point.
5. Signing consumes one per-document operation and appends the signature trailer.
6. Publication consumes `SignedReceipt`; only a successful sink result mints
   `PublishedReceipt`.
7. A configured signing authority may sign later documents; the per-document
   operation and serialized value are still affine.

The writer returns whole-document refusal. It never returns partial serialized or
signed bytes as a receipt.

## 30Rb:reader-state-map

TYPE LEAN:

```text
BoundedReceiptBytes
  -> LocatedReceiptEnvelope
  -> ReceiptSignatureChecked<SignerTrust>
  -> ParsedReceiptSkeleton<Species, Projection, SignerTrust>
  -> Receipt<Species, Projection, SignerTrust>
```

Rich inserts:

```text
ParsedReceiptSkeleton<D, Rich, T>
  -> DecryptedOpaqueOverlay
  -> ValidatedOpaqueOverlay
  -> Receipt<D, Rich, T>
```

Required reader behavior:

- The locator may read only the fixed prefix/trailer and locate bounded spans.
- It may use `KeyId` to query an injected resolver. The returned key already
  carries `TrustedReceiptSigner` or `SelfAssertedReceiptSigner`.
- The trust trait and both markers are private/sealed. The resolver exposes two
  concrete result types (`TrustedReceiptVerificationKey` and
  `SelfAssertedReceiptVerificationKey`), not `resolve<T>()`; generic callers
  cannot request their preferred trust marker. The receipt crate alone converts
  either concrete key into the matching checked state after strict verification.
- Unknown key, unavailable key, missing signature, invalid signature, unsupported
  version, malformed armor, truncation, and overlay mismatch remain distinct
  partial reasons.
- Verification uses Ed25519 strict verification over DSSE PAE of the exact body
  span. The same body bytes then enter parsing.
- Species/projection parsed from the body must equal the values used to select the
  fixed signature domain. Mismatch is signature/domain failure, never coercion.
- Rich decryption cannot begin before outer signature success.
- Decrypted bytes cannot enter a report model until reverse-overlay validation
  succeeds completely.
- A partial receipt may expose only one bounded structural report under one global
  unauthenticated/damaged status. It cannot promote plausible-looking fields.

`Receipt<D,P,T>` means format-complete for that projection. It does not mean true,
fresh, safe to share, or authorized. A complete plain ApplyIntent remains unable to
satisfy required mutation gating because it contains no exact apply image.

## 30Rb:one-canonical-outer-grammar

The V1 outer grammar is exact. ASCII tokens below are literal; every line ends in
LF. No CR, tabs, comments, trailing spaces, blank lines, alternate ordering,
duplicate fields, ignored bytes, or alternate encodings are accepted.

```text
dorc-receipt/1
species <plan|apply-intent|apply-outcome>
projection <plain|rich>
receipt-id <64-lower-hex>
order <20-decimal-digits>
signing-key-id <64-lower-hex>
[rich only] encryption-key-id <64-lower-hex>
records <canonical-u64>
<exactly records skeleton record lines>
skeleton-end
[rich only]
opaque-overlay
<one canonical Age ASCII-armored message, including its own markers>
opaque-end
signature <128-lower-hex>
<EOF>
```

Angle brackets and `[rich only]` are specification metasyntax and never file
bytes. A plain document proceeds directly from `signing-key-id` to `records`; a
rich document has exactly one intervening `encryption-key-id` line and exactly one
overlay region after `skeleton-end`.

Decimal field values have no sign and no leading zero except literal `0`. The
top-level `order` is the one fixed-width exception: exactly 20 decimal digits, with
leading zeroes required, and it is a `ReceiptOrderToken` used for local store
selection only. Hex is exact length and lowercase. Record IDs are canonical decimal integers, unique and
contiguous from `0` through `records - 1`; the writer assigns them, never host
payloads.

The order token is inside the signed body. A local store filename carrying an order,
species, or receipt ID must match these authenticated header values after verification;
filename metadata never overrides them.

The literal skeleton span is byte 0 through and including `skeleton-end\n`.
The signed body is byte 0 through and including `skeleton-end\n` for plain, or
through and including `opaque-end\n` for rich. The `signature` line is excluded
from its own input and must be followed immediately by EOF.

Fixed DSSE payload types are:

```text
application/vnd.dorc.receipt.v1.plan.plain
application/vnd.dorc.receipt.v1.plan.rich
application/vnd.dorc.receipt.v1.apply-intent.plain
application/vnd.dorc.receipt.v1.apply-intent.rich
application/vnd.dorc.receipt.v1.apply-outcome.plain
application/vnd.dorc.receipt.v1.apply-outcome.rich
```

The file contains no algorithm tokens. The version fixes Ed25519, DSSE PAE,
SHA-256, Age v1/X25519, and canonical Age armor.

For rich, `encryption-key-id` is mandatory and sits exactly between the signing
key and record-count lines; plain rejects it. The Age region accepts only the
exact `ArmoredWriter(AsciiArmor)` shape: literal begin marker, canonical unpadded
base64 lines at the maintained writer's width (all full-width except the final
data line), literal end marker, LF, then `opaque-end`. A fixture emitted by the
selected locked Age implementation is the positive oracle; alternate wrapping,
padding, blank lines, armor
headers, extra markers, or trailing spaces are invalid even if Age's reader would
decrypt them. The outer strict locator validates this lexical shape before handing
the exact bytes to Age.

### Skeleton record rule

Every skeleton row has this shape:

```text
record <RecordId> <closed-record-kind> <closed ordered key=value fields>
```

Skeleton values are limited to canonical integers, fixed-length IDs/digests,
closed enums, booleans spelled `yes|no`, and opaque-field states:

```text
captured | withheld-plain | unavailable | uncollected | omitted-limit
```

No path, argv, source text, shell, host-chosen coordinate, tool output, diagnostic
tail, user name, host name, environment value, or other arbitrary authored/host
text appears in the skeleton. Those values either occupy typed opaque slots in
rich or carry an explicit state.

The exact record production is:

```text
record-line = "record" SP record-id SP record-kind *(SP field) LF
field       = field-key "=" field-atom
field-key   = lower-alpha *(lower-alpha | digit | "-")
field-atom  = one closed enum token, canonical integer, or exact fixed hex
SP          = one ASCII space
```

No parser uses `split_whitespace`; it consumes the literal one-space grammar.
Each kind has exactly these keys, in this order:

| Record kind | Ordered fields |
|---|---|
| `invocation` | `mode started argv target account` |
| `source` | `ordinal role digest bytes path excerpt account` |
| `admission` | `outcome records bytes account` |
| `presented-plan` | `planning-input presented-plan planned-image account` |
| `site-decision` | `leaf member ast disposition shell account` |
| `region-decision` | `region ast disposition routes shell account` |
| `load-decision` | `ordinal outcome name custody account` |
| `site-classification` | `leaf member ast class verdict-lane invalidator operands dropped account` |
| `solve-certification` | `pass consistent tripped account` |
| `probe-ship` | `leaf member lane source account` |
| `survival` | `leaf member outcome wall aggregate poison account` |
| `render-decision` | `subject kind detail account` |
| `narrative` | `ordinal speech kind operands dropped account` |
| `licensor` | `leaf member license custody locus account` |
| `projection-omission` | `species count reason account` |
| `apply-intent` | `session generation policy assignments origin-state account` |
| `apply-assignment` | `ordinal target context image image-state origins account` |
| `plan-origin` | `assignment ordinal receipt presented account` |
| `apply-outcome` | `intent terminal sites durable account` |
| `site-outcome` | `ordinal assignment leaf member status tool-rc stdout stderr account` |

The Stage 0 grammar table defines every field's closed enum and numeric width;
the names above and their order are REQUIRED. `member`, `started`, `tool-rc`,
`wall`, and other semantically absent scalars use a closed `absent` token, never
an empty value or omitted field. Opaque-capable fields carry only one of the five
opaque-field-state tokens. Unknown record kinds, keys, enum tokens, extra fields,
or missing fields refuse completeness.

Stage 0 ends at a conductor checkpoint over one exhaustive Rust table that maps
every `(record kind, field key)` above to exactly one scalar type and token set.
The table has no wildcard/default arm and generates neither code nor parser
behavior; it is the reviewed input both writer and reader tests must agree with.
No broad parser work starts before this checkpoint. At minimum, the common token
sets are fixed now:

```text
opaque-state = captured | withheld-plain | unavailable | uncollected | omitted-limit
account      = authored-before-contact | host-influenced | untracked
bool         = yes | no
projection   = plain | rich
species      = plan | apply-intent | apply-outcome
disposition  = run | replace | omit | guard
origin-state = unavailable | known
image-state  = captured | withheld-plain | unavailable | omitted-limit
```

Semantic fields such as classification, survival, render-decision, narrative,
terminal outcome, and failure kind are closed receipt-local enums projected
exhaustively from their current source enums. Their wire token tables are part of
the Stage 0 checkpoint and fixture diff; a builder may choose clear tokens but may
not merge source variants with different repair, authority, or epistemic meaning.

### Opaque field tag table

`OpaqueFieldTag` is a closed enum with the following exact wire token and
canonical order:

| Order | Token |
|---:|---|
| 0 | `argv` |
| 1 | `target-name` |
| 2 | `source-path` |
| 3 | `source-excerpt` |
| 4 | `record-stream` |
| 5 | `shell` |
| 6 | `fact` |
| 7 | `locator` |
| 8 | `custody` |
| 9 | `import-path` |
| 10 | `emitted-name` |
| 11 | `diagnostic-operand` |
| 12 | `apply-artifact-image` |
| 13 | `stdout` |
| 14 | `stderr` |
| 15 | `error-detail` |

Tokens match `[a-z][a-z0-9-]{0,63}` but matching that alphabet never admits an
unknown token. Schema code maps each `(record kind, field tag)` to allowed and
required capture status. Canonical overlay ordering uses the numeric table above,
not locale/string sorting. Adding a tag changes this reviewed table and its valid
and invalid vectors.

Commit complete per-kind vectors before broad projection work. The parser has one
function per record kind or one closed state machine with exhaustive kind dispatch;
no generic map/object parser.

## 30Rb:reverse-overlay-grammar

The authenticated Age plaintext is one binary-safe length-framed sequence:

```text
dorc-receipt-overlay/1\n
receipt-id <64-lower-hex>\n
species <plan|apply-intent|apply-outcome>\n
projection rich\n
skeleton-sha256 <64-lower-hex>\n
entries <canonical-u64>\n
entry <RecordId> <OpaqueFieldTag> <canonical-u64-byte-length>\n
<exact byte-length bytes>\n
... repeated in (RecordId, OpaqueFieldTag enum order) ...
overlay-end\n
```

The LF after an entry payload is framing and is not part of the value. Zero-length
opaque values are legal where the field schema permits them. The parser consumes
the declared byte length before looking for the framing LF.

Validation requires exact equality of:

- outer and inner receipt ID;
- species and projection;
- SHA-256 of the literal skeleton span;
- unique overlay keys and the schema's exact captured-slot set;
- every record target and field tag;
- total consumed entry count and bytes; and
- EOF immediately after `overlay-end\n`.

Overlay ordering is canonical. Unknown, missing, extra, duplicate, dangling,
aliased, cross-document, wrong-species, or wrong-field entries yield a partial
receipt and release no opaque values.

## 30Rb:apply-artifact-image-format

`ApplyArtifactImage` is a separate lossless binary-safe container embedded as an
opaque field in rich ApplyIntent. It is not another user-facing receipt format.

Required model:

```rust
struct ApplyArtifactImage {
    id: ApplyArtifactImageId, // private; derived after canonical validation
    form: RecordedArtifactForm,
    entrypoints: NonEmpty<ApplyEntryId>,
    roots: NonEmpty<ApplyRoot>,
    entries: NonEmpty<ApplyImageEntry>,
    topology: ApplyTopology,
}

struct ApplyImageEntry {
    id: ApplyEntryId,
    path: Option<RecordedApplyPath>, // absent only for an input stream
    kind: ApplyEntryKind,
    mode: RecordedMode,
    bytes: ApplyEntryBytes,
}
```

No constructor accepts `ApplyArtifactImageId`. The sole mint validates the image,
serializes its canonical identity encoding, computes the ID, and stores both in
one operation. Parsing recomputes and compares the ID before producing an image.

Required constructors validate:

- at least one entry/root/entrypoint;
- unique IDs and paths;
- every root, entrypoint, and edge endpoint exists;
- target-relative traversal-free paths without normalization at receipt time;
- exact ordering supplied by the applied artifact;
- every mode/kind required by that artifact form;
- no unaccounted entry; and
- exact materialization round-trip.

### V1 recorded apply path grammar

`RecordedApplyPath` is a receipt-owned portable relative-path grammar, applied by
the same function at live construction, serialization, parsing, and
materialization:

- bytes are ASCII only in V1; no Unicode decoding or normalization occurs;
- `/` is the only separator;
- path length is 1..=4096 bytes; each component is 1..=255 bytes;
- no leading/trailing `/`, empty component, `.` component, or `..` component;
- no byte below `0x20`, DEL, NUL, backslash, colon, `<`, `>`, double quote,
  vertical bar, question mark, or asterisk;
- no component ends in space or dot;
- DOS device stems `CON`, `PRN`, `AUX`, `NUL`, `COM1`..`COM9`, and
  `LPT1`..`LPT9` are rejected case-insensitively, with or without an extension;
- no two paths in one image compare equal under ASCII case-folding; and
- the accepted bytes are stored and reproduced exactly, never cleaned or
  normalized.

This deliberately narrow V1 grammar covers current generated artifact names and
avoids POSIX/Windows materializer disagreement. An external tree carrying a path
outside it refuses before intent publication. Widening the grammar is a receipt
version/design act, not a platform adapter workaround. A single input stream has
no fabricated `RecordedApplyPath`.

The exact image encoding is:

```text
dorc-apply-artifact-image/1\n
form <flattened|multipart|mirrored-tree|preserved-book-tree|external-stream>\n
entrypoints <u64>\n
entrypoint <ApplyEntryId>\n
roots <u64>\n
root <ApplyRootId> <ApplyEntryId>\n
entries <u64>\n
entry <ApplyEntryId> <stream|file> <unused|four-octal-digits> <path-bytes> <content-bytes>\n
<exact path-bytes bytes>\n
<exact content-bytes bytes>\n
... exact count, ApplyEntryId ascending ...
edges <u64>\n
edge <parent-ApplyEntryId> <child-ApplyEntryId> <loads|contains>\n
... lexicographic numeric endpoint order ...
image-end\n
```

IDs are canonical contiguous decimal ordinals in their own domains. A stream
entry declares `path-bytes=0`; a file entry declares a nonzero path length and the
path must pass the grammar above. The LF after each raw block is framing, not
content. `unused` means file mode is not an execution input; it does not mean an
unknown relevant mode. Unknown relevant mode refuses image construction.

`ApplyArtifactImageId` is SHA-256 over DSSE PAE with fixed payload type
`application/vnd.dorc.apply-artifact-image.v1` and the exact bytes from the first
line through `image-end\n`. The container does not carry its own ID, avoiding a
circular identity; the ApplyIntent skeleton carries the ID and rich parsing
recomputes it from the opaque image bytes.

Do not use txtar if it cannot carry this information without escaping or
normalization.

`ArtifactSet -> ApplyArtifactImage` is not a field rename. Extend `Selection` and
`ArtifactSet` to retain roots, dependency edges, entrypoints, and modes from
`BundleProjection`/placement. The conversion copies exact bytes by value and
validates. External stdin/file apply uses a distinct single-stream constructor and
does not fabricate bundle roots.

Receipt encoding may contain the image; it may not alter it. Re-materialization
must reproduce every entry, path, mode, root, edge, entrypoint, and byte.

## 30Rb:plan-receipt-content

### Required V1 projection

The human lean is to carry as much useful existing semantic state as safely
practical. Build the required core route first, then pursue the existing-content
tranche below as V1 STRETCH rather than postponing it reflexively. A conductor cut
must be explicit and must leave omission accounting; a builder may not cut rows
locally.

Required core records:

- invocation identity and policy;
- ordered source identities: role, ordinal, digest, byte length, location state;
- admitted record-stream identity, count, arrival instants, and exact accounted
  bytes in rich;
- `PlanningInputId`, `PresentedPlanId`, and planned
  `ApplyArtifactImageId`;
- site dispositions keyed by full `SiteId` plus `AstId`;
- region decisions keyed by `ElisionRegion`, never smeared onto a call site;
- decision influence accounts, including explicit `untracked`;
- explicit projection omissions.

V1 STRETCH existing-content tranche:

- load/custody withholding decisions;
- site classifications and invalidator/verdict-lane status;
- solve certification and trip status;
- probe shipment lane/unresolvable decisions;
- admission outcome;
- survival outcomes and re-derivation disagreement;
- render decisions, import edits, span refusals, defensive emission;
- complete region route populations already retained in Spine;
- every current `CollapseNarrative` and its closed speech-act/kind;
- vouch/licensor attachment recoverable from existing
  `ReplaceLicense`/`GuardLicense` derivations, including custody and source locus;
  do not wait for the currently unminted `SpineVouch` species.

Do not add empty durable rows for currently unminted `SpineObservation`,
`SpineValidityRound`, or old `SpineOutcome`. Actual apply outcome belongs to the
top-level ApplyOutcome receipt. Intermediate fixpoint rounds remain non-products.

### Field placement

Skeleton:

- receipt and semantic IDs/digests;
- counts, ordinals, byte lengths, timestamps;
- full `SiteId` numeric components and `AstId`;
- closed disposition/classification/admission/ship/survival/render/narrative tags;
- closed influence labels and omission states.

Rich overlay:

- argv;
- source/target/controller paths and names;
- exact bounded source excerpts;
- exact admitted records block and accounted report tails;
- shell/source snippets;
- coordinates/facts/kinds/selectors;
- vouch function/invocation text and locators;
- import paths, emitted names, and arbitrary diagnostic operands.

Never persist in V1:

- unaccounted stdout/stderr that was not already transported;
- live claim/license/PlanAuthority values;
- working lattice/fixpoint state or process-local handles;
- full planning sources by default;
- private keys or provider paths;
- a value newly read only to enrich the receipt.

Every nonzero existing semantic population omitted from PlanReceipt mints an
explicit projection-omission record. The projection census must fail when a new
Spine species lands until it is classified.

## 30Rb:apply-intent-and-outcome-content

ApplyIntent records the actual admin-authored application mapping, not Dorc's
planned mapping:

- non-empty ordered assignments;
- each resolved target/context/session identity;
- one exact by-value `ApplyArtifactImage` per assignment in rich;
- `OriginatingPlans` per assignment;
- actual policy and controller semantics;
- generation and invocation identity;
- requested publication policy and exact prepared pre-publication state; and
- requested publication grade and configured-bypass provenance in the receipt model;
  achieved publication proof remains an ephemeral typed result and is never inferred
  from finding the file later.

A plain ApplyIntent records IDs, topology/count summaries, and
`withheld-plain`; it cannot mint a required publication witness. In V1, a signed
plain intent may be emitted only as report data on the explicit
`ConfiguredBypass` route. That route's bypass witness, not the plain receipt,
permits dispatch. Later product policy may choose when such a bypass is
configurable; V1 does not treat plain publication as satisfying required policy.

ApplyOutcome records only what execution knows:

- exact ApplyIntent ID;
- graceful terminal state;
- per-site/assignment identity and status available from the V1 DST route;
- complete, unknown, not-attempted, command-failed, transport-failed,
  mutation-integrity-aborted, or cancelled state as closed types;
- admitted bounded output in rich;
- cancellation and quiescence separately where represented; and
- durable publication success/failure as narration, never execution integrity.

No-outcome is graph absence, not a fabricated ApplyOutcome document. The report
model makes that absence explicit:

```rust
enum OutcomeAvailability {
    Recorded(Reingested<Receipt<ApplyOutcome, Projection, SignerTrust>>),
    Missing { intent: ApplyIntentId },
}
```

`Missing` is constructed by graph correlation, is never serialized as an
ApplyOutcome, and says only that no outcome receipt was found.

## 30Rb:apply-dispatch-sequence

The V1 transition is:

```text
NonEmpty<ReadyApplyTarget>
  -> ApplySessionReady
  -> PreparedApplyIntent
  -> IntentPublicationGate::Published(
       PublishedReceipt<ApplyIntent, Rich, Grade>,
       ExactApplyImagesPresent
     )
  -> MutationDispatchPermit
  -> MutationDispatched
  -> ApplyOutcome attempt or explicit graph absence
```

The only second route is disjoint:

```text
PreparedApplyIntent
  + ConfiguredReceiptBypass
  -> IntentPublicationGate::ConfiguredBypass
  -> MutationDispatchPermit
```

It may separately attempt a signed plain report, but neither success nor failure
of that attempt is an input to the bypass mint. There is no generic conversion
from plain publication to `Published`.

`ApplySessionReady` is one aggregate per `dorc apply` invocation. A per-target
standup record does not itself license intent publication or mutation.

TYPE LEAN:

```rust
struct ApplySessionReady {
    id: ApplySessionId,
    generation: ApplyGenerationId,
    targets: NonEmptyMap<ReadyApplyTargetId, ReadyApplyTarget>,
}

impl ApplySessionReady {
    fn prepare_intent(
        self,
        assignments: NonEmpty<PendingApplyAssignment>,
        policy: ReceiptPolicyWitness,
    ) -> Result<PreparedApplyIntent, IntentPreparationRefusal>;
}
```

The consuming mint rejects unknown targets, duplicate assignment ordinals,
ambiguous target membership, a ready target omitted contrary to active apply
policy, and any assignment whose image/origin account is incomplete. It produces
private `RecordedApplyAssignment`s bound to this session; callers cannot compose a
ready target from one session with the ID/generation of another.

Current real SSH transport does not establish a reusable tunnel before
`SessionDriver::run`. V1 must not lie about that. The acceptance route uses an
injected standup/session edge that genuinely supplies all required controller-owned
identity. For the current one-shot SSH route, the builder must choose one safe V1
outcome:

1. implement a genuine standup that can mint `ReadyApplyTarget`; or
2. keep that route behind an explicit injected configured-bypass policy and ensure
   it cannot satisfy required publication.

Faking `ReadyApplyTarget` from the host string is forbidden. This choice is local
only between the two safe outcomes; report it to the conductor.

Consume `MutationDispatchPermit` immediately around the one call to
`SessionDriver::run` (or its successor) that commits the first potentially mutative
dispatch. `transport` remains unaware of receipt authority. A `NotAttempted` result
still consumes the permit because the controller committed to dispatch and must not
retry an apply.

After `MutationDispatched`, separate:

```rust
enum PostDispatchFailure {
    DurableOnly(DurableFailure),
    TransportIntegrity(TransportIntegrityFailure),
    ExecutionIntegrity(ExecutionIntegrityFailure),
    AttributionIntegrity(AttributionIntegrityFailure),
    GenerationIntegrity(GenerationIntegrityFailure),
    TargetIntegrity(TargetIntegrityFailure),
    MutationIntegrity(MutationIntegrityFailure),
}
```

Durable-only failure is reported and orchestration continues. Transport,
execution, attribution, generation, target, or mutation-integrity failure follows
its existing abort/unknown policy and is never caught by a generic durable fallback.
The continuation function accepts `DurableFailure` directly; it does not accept
`PostDispatchFailure` and re-match permissively.

## 30Rb:reingestion-and-why

Delete `PlanAuthority::of_admitted_replay()` and every equivalent route. This is a
required architectural change, not a rename.

TYPE LEAN:

```rust
pub struct Reingested<T: sealed::RecordedType>(T); // private field and mint

pub struct WhyPhase { /* private CLI/aid witness */ }

fn describe_plan(
    value: Reingested<Receipt<PlanReceipt, Rich, TrustedReceiptSigner>>,
    phase: &WhyPhase,
) -> ReceiptReport;
```

`WhyPhase` may license a report projection; it never extracts bare `T`.

Every recorded semantic object that had an `InfluenceAccount` at projection
carries a receipt-local `RecordedInfluence` token. Reingestion never reconstructs
a live account. An absent, unknown, malformed, unverifiable, or failed-
recomputation token becomes report-only `MostInfluenced`, never authored. Where
current re-derivation computes an account, recorded and current accounts use the
same four-way comparison below; disagreement is a finding and neither side is
silently selected.

Recorded/re-derived comparison retains:

```rust
enum RecordedCurrent<T> {
    RecordedOnly(Reingested<T>),
    CurrentOnly(T),
    BothAgreeing { recorded: Reingested<T>, current: T },
    BothDisagreeing { recorded: Reingested<T>, current: T },
}
```

The `current` arm is itself a report-only derivation type, not live `Disposition`,
`Plan`, or license.

Refactor replay analysis so reingested host records cannot call live license mints.
The exact implementation is an IMPLEMENTOR CHOICE, but these effects are required:

- factor decision calculation from authority minting; or instantiate a separate
  report plane whose decision payload is `ReDerivedDisposition`;
- no reingested type appears in a signature of `ReplaceLicense`, `GuardLicense`,
  `PlanAuthority`, `Plan::decided`, artifact selection, probe construction, or apply;
- a lexical non-empty allow-list gate enumerates every `Reingested` consumer across
  crates, two-way and file-narrow;
- a compile-fail test proves a reingested disposition cannot satisfy a live plan
  consumer; and
- re-derivation disagreement is a finding preserving both values.

These are independent acceptance properties: preserving the conservative
recorded influence grade does not permit an action, and making all reingested
content report-only does not permit an absent grade to read as authored.

Source reconstruction V1 implements current digest match, bounded rich excerpt,
drifted, absent, unreadable, and omitted-by-limit. Archive lookup remains LATER but
its enum arm may exist only if no empty implementation is required by callers.

## 30Rb:recorded-receipt-graph

`ReceiptGraph` is report-only and order-independent. It correlates complete receipts
by typed IDs, while retaining partial inputs beside graph findings.

TYPE LEAN:

```rust
struct ReceiptGraph {
    plans: BTreeMap<PlanReceiptId, Reingested<Receipt<PlanReceipt, P, T>>>,
    intents: BTreeMap<ApplyIntentId, Reingested<Receipt<ApplyIntent, P, T>>>,
    outcomes: BTreeMap<ApplyOutcomeId, Reingested<Receipt<ApplyOutcome, P, T>>>,
    edges: Vec<ReceiptEdge>,
    findings: Vec<GraphFinding>,
    partials: Vec<PartialReceipt<PartialReason>>,
}
```

Do not use this exact heterogeneous generic storage if an enum is clearer; preserve
the effects:

- many plan-origin edges may enter one intent;
- one plan may feed many intents;
- one intent has zero or one outcome;
- intent-without-plan and outcome-without-intent are first-class;
- duplicate identical files, duplicate IDs with different bytes, extra outcomes,
  and identity disagreement remain distinguishable;
- missing edges synthesize no history and imply no execution state;
- filenames and enumeration order never mint edges; and
- graph correlation never joins world freshness, generations, authority, or
  influence accounts.

## 30Rb:version-one-limit-policy

All limits are private-field newtypes assembled into `ReceiptLimits`; every parser
accepts a complete limit policy and every nested parser consumes both parent and
local budgets.

Initial V1 conformance policy:

| Limit | Value |
|---|---:|
| outer receipt bytes | 64 MiB |
| literal skeleton bytes | 8 MiB |
| structural line bytes | 64 KiB |
| structural field bytes | 16 KiB |
| skeleton record count | 65,536 |
| Age armor bytes | 48 MiB |
| decrypted overlay bytes | 32 MiB |
| opaque overlay entries | 65,536 |
| one opaque field | 24 MiB |
| ApplyArtifactImage aggregate | 24 MiB |
| one apply-image entry | 16 MiB |
| apply-image entries | 8,192 |
| topology edges | 65,536 |
| topology depth | 1,024 |
| recorded path bytes | 4 KiB |
| argv entries | 32,768 |
| source identities | 32,768 |
| source excerpt per span | 64 KiB |
| source excerpt aggregate | 1 MiB |
| admitted records aggregate | existing independent 4 MiB budget |
| admitted host output aggregate | 4 MiB |
| canonical integer digits | 20 |

These are V1 spike policy, not forever protocol promises. Builders may lower a
limit to keep a path bounded. Widening or removing one requires conductor review,
boundary-minus/at/boundary-plus tests, and an allocation analysis. Never allocate
from a file-declared count before checking remaining bytes, count policy, checked
arithmetic, and aggregate budget.

## 30Rb:staged-build-order

### Stage 0 - laws, crate, dependencies, vectors (serial)

Touch:

- `spike/Cargo.toml`, `spike/Cargo.lock`;
- new `crates/receipt/{Cargo.toml,CLAUDE.md,src/lib.rs}` and the dependency-outward
  `crates/receipt-crypto` sibling boundary;
- `hk.pkl` only if ordinary workspace routing fails to include the crate;
- committed grammar/overlay/image valid and invalid fixture vectors.

Required output:

- exact selected package/features and reviewed lockfile diff;
- crate-local versions of every immediately-built 30Ra invariant;
- no implementation beyond types needed to compile vectors;
- a dependency-graph test or lexical fence proving forbidden Dorc edges absent;
- valid/invalid vectors committed before parser growth.

Do not parallelize lockfile or format-token authorship.

### Stage 1 - identity and plain kernel (serial foundation)

Touch new receipt modules for pure identity/format/state work and `receipt-crypto`
only for the concrete Ed25519 adapter.

Build:

- IDs and domain-separated SHA-256 encoders;
- typed limits/budgets;
- sealed species/projection/trust markers;
- `Receipt<D,P,T>`, partials, reader/writer states;
- exact skeleton grammar and locator;
- DSSE PAE in `receipt`; Ed25519 signing/strict verification in `receipt-crypto`
  behind the receipt capability traits;
- signed plain round-trips for all three species;
- compile-fail authority/state bypasses.

Exit: one canonical plain writer and one strict plain reader; no rich format emitted.

### Stage 2 - three independent kernels (parallel after Stage 1 API freezes)

#### Stage 2A - apply artifact image

Touch `receipt/src/image.rs`, focused tests, and read-only adapter prototypes.

Build exact single-stream and multi-file container, topology validation, identity,
and materialization round-trip. Do not touch CLI artifact selection yet.

#### Stage 2B - reverse overlay and Age

Touch receipt `projection.rs`/`overlay.rs`, `receipt-crypto`, and rich conformance
tests. No receipt-local `crypto.rs` or concrete crypto dependency is permitted.

Build inert reverse-overlay validator first, then Age adapter and one complete rich
round-trip. No partially built rich production writer is exposed between commits.

#### Stage 2C - recorded models and graph

Touch species model modules, `reingested.rs`, `graph.rs`, focused tests.

Build PlanReceipt record families, M:N origins/assignments, outcome states,
non-extractable reingestion, and graph correlation over fixture receipts.

These lanes may share only Stage 1 public types. Fold 2A, then 2B, then 2C and run
whole-crate tests after each; do not resolve API drift independently in all lanes.

### Stage 3 - presented plan and expanded PlanReceipt (serial)

Touch:

- `plan/Cargo.toml`, `plan/src/receipt.rs`, `plan/src/spine.rs`,
  `plan/src/erasability.rs`, `plan/src/invocation.rs`;
- `core/src/spine.rs` only for truthful record/accessor/census changes;
- `cli/Cargo.toml`, `cli/src/main.rs`, `artifact.rs`, `receipt_edge.rs`;
- focused plan/receipt tests.

Build:

- `PlanningInputId` and `PresentedPlanId` at the final presentation boundary;
- replacement of FNV and `SpineDigest` string plumbing with
  `DecidePlane::PresentedPlanIdentity` / `SpinePresentedPlan<PlanPlane>`;
- artifact topology carriage into planned `ApplyArtifactImageId`;
- one PlanReceipt projection from final Spine/Plan/artifact/source context;
- the conductor-approved V1 STRETCH tranche and complete omission census;
- signed plain and rich plan route through injected sink/source.

Exit: product-spanning injected plan -> receipt -> read route green. The old whylog
remains the sole default production writer until `30Rd` Stage D4 makes the replacement
usable by the real binary; there is never a second default production writer.

### Stage 4 - apply intent, dispatch, and outcome (kernel work may parallel; CLI integration is serial)

Touch:

- `cli/src/apply.rs`, `transport_edge.rs`, `main.rs`, `artifact.rs`;
- `transport/src/sim.rs` and, only if required, `transport/src/lib.rs`;
- `hostsim` focused DST adapters/tests;
- receipt apply/outcome modules and tests.

Build:

- `ArtifactSet -> ApplyArtifactImage` plus external single-stream path;
- `ApplySessionReady` aggregate and target/context identity;
- Prepare/publish/gate/permit/dispatch sequence;
- pre- versus post-dispatch failure direction;
- current SessionOutcome and hostsim per-site mapping to ApplyOutcome;
- explicit no-outcome graph path;
- signed rich required-publication route and signed plain non-authorizing route.

The receipt-model, image, and hostsim kernel work may proceed beside late Stage 3
only after receipt module APIs freeze. The CLI integration is serial: both stages
rewrite `cli/main.rs`, so fold Stage 3 first and rebase Stage 4 rather than claiming
independently compilable orchestration lanes or hand-merging two rewrites.

### Stage 5 - why, correlation, and report-only re-derivation (serial integration)

Touch:

- `plan/src/spine.rs` and settlement seams needed to separate calculation from
  authority minting;
- `cli/src/{why.rs,world.rs,lib.rs,main.rs,results.rs,source_match.rs}`;
- `dorc-loom/src/consumer.rs`, its direct-why parser, and consumer/editable-surface
  tests that currently call `dorc_plan::whylog` or embed old receipt bytes;
- `aid` report-model/loom cases without builder-authored prose.

Build:

- delete replay plan authority;
- `Reingested<T>` and `WhyPhase` report projection;
- source resolution states and bounded excerpt fallback;
- recorded/re-derived four-way comparisons;
- M:N ReceiptGraph narration for all three species;
- signer trust, projection, partial/completeness, missing edge, and disagreement
  report states;
- destination-specific sink encoding for every arbitrary value.

Exit: both product routes end in `dorc why`; receipt bytes cannot reach any operation
endpoint.

### Stage 5A - minimal production durable edge (serial integration)

Build every stage and acceptance property in `30Rd:implementation-sequence` and
`30Rd:v1-acceptance-and-exit`. `30Rd` is the sole mechanism specification; do not
copy a subset into a lane brief and treat omitted constraints as optional.

Exit: the real binary initializes/reopens the V1 local keyset, publishes/reads the
V1 local store, survives a process restart, and completes the plan/why and apply/why
routes on both platform legs. Fixture/volatile capabilities cannot satisfy the
production route. The old whylog remains until this exit is green.

### Stage 6 - rip old implementation and harden (serial close)

Stage 6 MUST NOT begin before Stage 5A exits.

Delete:

- `plan/src/whylog.rs` and all `WhylogV2*`, `WHYLOG_*`, `ApplyLine`,
  `DurableProjection`, old `DurableAccount`, and `ACCOUNT_EXPORT`;
- `cli/src/whylog_store.rs` and old default-env/retention behavior after their
  `30Rd` replacement is live;
- `PlanAuthority::of_admitted_replay`;
- `Replay`, `ReplayLoad`, old `DriftedReceipt`, old digest-only disagreement,
  and old receipt rendering structs where superseded;
- old `--whylog`, `--whylog-dir`, `--no-whylog` parsing after the new exact
  receipt-selection/output flags land; no aliases or dual spelling;
- every old whylog fixture, loom, parser test, and format token;
- direct old-format consumers in `dorc-loom/src/consumer.rs`,
  `dorc-loom/tests/{consumer,editable_surface,coverage}.rs`, and old whylog
  examples in invocation/usage tests; the replacement consumes the receipt reader
  rather than retaining an in-process compatibility parser;
- hand-rolled SHA/FNV identity code replaced by receipt-owned SHA-256;
- one-file apply assumptions from receipt/image paths.

Then run the full verification ladder and inspect a no-bless diff. The arc ends with
one receipt implementation.

## 30Rb:receipt-verification-map

### Required receipt-crate corpus

Commit:

- one valid plain and rich receipt for each species;
- exact writer byte equality for every valid fixture;
- CRLF, missing LF, tabs, trailing spaces, blank lines, comments, duplicate fields,
  unknown fields/kinds, alternate integers/hex, reordered fields/records, trailing
  bytes, wrong EOF, and unsupported version refusals;
- body, species, projection, authenticated order, key ID, signature, and
  signature-domain mutation;
- missing/extra/duplicate/dangling/cross-document/wrong-field overlay entries;
- damaged/truncated Age armor and unavailable key;
- single-stream and multi-file image topology/byte round-trip;
- boundary-minus/at/plus for every V1 limit family;
- one bounded mutation sweep over a complete fixture;
- standard SHA-256, Ed25519, DSSE PAE, and Age interoperability vectors;
- rich-to-plain semantic remint and proof textual stripping fails;
- recorded influence round-trip, absent/unknown/malformed influence reading as
  `MostInfluenced`, recomputation disagreement, and proof that every such value
  remains report-only;
- no-compression/no-append/no-second-overlay grammar refusals.
- local filename/internal version/species/order/receipt-ID agreement, maximum-order
  ambiguity cohorts, and newest-partial-no-fallback behavior through `30Rd`.

### Required compile-fail/fence corpus

Prove:

- `Plain` cannot carry opaque data;
- `Rich` cannot serialize without exactly one sealed overlay;
- partial cannot call complete-only APIs;
- `Reingested<Disposition>` cannot satisfy a live consumer;
- an external type cannot implement the recorded/species/trust marker traits or
  construct `Reingested<T>`/`ReceiptSignatureChecked<T>`;
- a generic caller cannot request `TrustedReceiptSigner` from the key resolver;
- recorded influence cannot become `InfluenceAccount`;
- receipt IDs cannot cross species;
- signing and encryption key IDs/capabilities cannot alias or convert;
- apply-image ID cannot become presented-plan ID;
- published/permit/plaintext states cannot be cloned or reused;
- no fixture ID/parser constructor is reachable from production files;
- every safe authority-capable adapter is lexically enumerated, with a non-empty
  and stale-entry-fails walk.

### Required DST and product routes

Plan route:

```text
book/oracle + hostsim probe
  -> settlement/presentation
  -> PlanReceipt rich publication
  -> bounded read/signature/overlay validation
  -> Reingested receipt
  -> why explanation
```

Apply route:

```text
single or multi-file ApplyArtifactImage
  -> ApplySessionReady
  -> ApplyIntent rich required publication
  -> consumed MutationDispatchPermit
  -> SimDriver/hostsim execution
  -> ApplyOutcome publication OR explicit missing outcome
  -> ReceiptGraph
  -> why explanation
```

Inject sink failures:

- before dispatch under required policy: refuse and prove no dispatch call;
- before dispatch under explicit configured bypass: dispatch only through the
  bypass arm;
- after dispatch: record durable failure and prove coherent execution continues;
- transport/execution/attribution failure after dispatch: prove it is not caught as
  durable-only;
- late/duplicate/reordered outcome receipt: graph finding, no action.
- supersession/cancellation before permit consumption revokes that generation's
  publication/dispatch authority; late success, failure, or output is bounded
  report data only and cannot resurrect a permit. Cancellation acknowledgement
  does not prove execution quiescence.
- target, user/privilege, namespace, cwd, environment-policy, credential-scope,
  or session mismatch between standup and intent refuses before permit minting;
  nested/unavailable context cannot be rounded to ready.
- run the integrity/failure-direction cases across probe versus apply, admin versus
  oracle-author report questions, and reliable versus unreliable oracle inputs;
  a fix for one cell must not turn malformed authority into universal run in
  another.

### Verification tooling

- Ordinary tests and compile-fail doctests are mandatory.
- Add the cheapest useful Kani harnesses only for pure bounded parser progress,
  checked span arithmetic, plain/rich disjointness, and small overlay set equality.
- Do not add Lean/Aeneas or Flux for V1.
- Keep fixture crypto keys at the receipt edge and structurally out of production.
- Add no new bless authority. Receipt conformance bytes are reviewed fixtures, not
  generated user prose.
- Final builder gate: `mise run both gate:full-quiet`.
- Arc-close gate: `mise run gate:arc`, after preflight and only at conductor close.

## 30Rb:diagnostics-and-prose

Builders may mint typed codes, payloads, and defining-case structure for:

- unsupported receipt version;
- missing/invalid/untrusted signature states;
- unavailable overlay key and damaged/mismatched overlay;
- receipt partial/incomplete graph edges;
- duplicate/colliding IDs and graph disagreement;
- source current/drifted/absent/unreadable/omitted states;
- pre-dispatch publication refusal;
- post-dispatch durable failure;
- apply intent without outcome and orphan outcome;
- origin/actual-target or origin/actual-image mismatch.

Builders author zero user-facing prose. New registers start `None` and render
`[unwritten: <slug>]` until the conductor/human authors transcript prose through the
loom pipeline. Existing whylog prose is not copied mechanically onto semantically new
receipt states.

## 30Rb:stage-local-stop-conditions

STOP and return to the conductor if any builder believes they need:

- a second format/parser or a compatibility reader;
- unsigned output or clear opaque fallback;
- more than one rich compartment;
- a new crypto algorithm, feature, plugin, backend, or key role;
- normalization/reserialization before signature verification;
- a public/raw accessor on `Reingested`, opaque bytes, partial receipts, or IDs;
- a conversion from recorded to live authority;
- a fake target/session identity to satisfy `ApplySessionReady`;
- digest-only ApplyIntent in the required publication arm;
- a receipt-content field not assigned to skeleton/overlay/never with explicit
  bounds;
- a new observation made solely for durability;
- a permissive parse, unknown-field recovery, or ignored trailing bytes;
- append, compression, mutable completion, retention, alternative key providers,
  provider discovery/selection, archive, policy profiles, or store work beyond the
  required `30Rd` baseline in V1;
- a user-facing prose string authored by a builder; or
- weakening a negative test to make a check pass.

## 30Rb:builder-handoff-checklist

Every lane report states:

1. exact commit/tree base and crate CLAUDE files read;
2. required effects implemented and TYPE LEAN deviations;
3. constructors/mints added, their complete caller census, and bypasses removed;
4. every new field's skeleton/overlay/never class and bounds;
5. every new ID's domain encoding and why cross-domain substitution cannot compile;
6. every nondeterministic/I/O operation and its DI edge;
7. every old surface deleted or still blocking stage exit;
8. tests run, including exact focused tests and final foreground gate;
9. comment budget; and
10. any `tc-*` decision left for the conductor rather than resolved locally.

The build conductor reads `30Ra`, this document, and `30Rd` together. Builders receive only
the stage they own plus all cross-stage type contracts it consumes; they do not
locally reinterpret the receipt architecture.

## 30Rb:post-compliance-source-and-identity-advice

> Status: conductor-tier advice, directionally acknowledged by the human on
> 2026-08-24, not a human ruling. A stage-compliance review found two places where
> this specification fixed the durable output but underspecified the live value
> supplying it; these narrow amendments govern V1 without opening adjacent loader
> or identity redesign.

- **FIX NOW - source table:** Retain one ordered, role-carrying `SourceClaim`
  vector. Its ordinal is deterministic acquired-source/`SourceFileId` table order,
  not dynamic shell load-occurrence order; preserve the current one-main-book
  invariant at its private construction boundary. `SourceRole` is V1 table
  classification only. Repeated, multi-role, and multi-target semantics remain
  owned by `30I`'s occurrence account and are not added here; punt them unless
  truthfulness forces refusal or omission.
- **FIX NOW - presentation binding:** `PlanPlane::PresentedPlanIdentity` remains
  exactly `PresentedPlanId`. A private final-presentation witness may carry
  `PlanningInputId`, `PresentedPlanId`, and the planned `ApplyArtifactImageId`
  atomically into receipt projection, but it is not an identity and cannot satisfy
  identity APIs. Construct it, and mint its `PresentedPlanId`, only after the human
  and executable views and artifact bytes are final; pin cross-plan field
  substitution. Its storage location is implementor choice; do not build a generic
  identity-bundle framework.
