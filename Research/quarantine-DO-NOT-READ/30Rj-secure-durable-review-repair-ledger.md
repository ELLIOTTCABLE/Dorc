# 30Rj — secure-durable review repair ledger

> Tier: quarantined. The repair of the four `30Ri` blockers, one section each: the shape chosen,
> what it does and does not carry, the seams kept open for debugging, and any lexical fence left
> standing as authority. No commit hashes, no tip churn, no per-test diary.

## `30Ri:fnd-dispatch-publication-remains-unbound`

**Shape: ownership, not a second commitment.** The chain is now
`ApplySessionReady → PreparedApplyIntent → AccountedApplyIntent → PublishedApplyIntentV1 →
MutationDispatchPermit → MutationDispatched`, and every state OWNS its predecessor by value.
`account_images` takes `self`; `publish_through` takes `self`; `permit` takes no second argument.
So "publication A dispatches intent B" is not a refusal any more — it is unspellable, and the
pins that assert it are compile-fail doctests rather than runtime assertions.

Nothing was hashed. A second commitment over session/generation/policy/assignments/images was
considered and dropped: ownership already establishes the same invariant, and a digest would have
been a value somebody could compare against the wrong thing.

**The durability proof is gone as a free-standing value.** `DurablePublicationProof` had a public
mint fenced only lexically, and one plus any prepared intent produced a permit. It is replaced by
`RequiredPlacementLanding` — a primitive report (`document_digest`, `policy_identity`) carrying no
authority — and the publication is minted INSIDE
`AccountedApplyIntent::publish_through(id, place)`, which calls the placement and hands it the
identity the publication will record. The mint has private fields and no public constructor.

**RESIDUAL, stated plainly.** No Rust type can prove a file reached a disk in a crate this one
does not know about. What the types now carry is: a publication value exists only where an
accounted intent — unforgeable outside `dorc-receipt` — was moved through a placement call. The
placement's truthfulness is an INJECTED CAPABILITY chosen by the composition root, which is a
different and honest claim. A fixture placement can still answer a landing; it cannot manufacture
an accounted intent, and that is the half that was doing the work.

**The bypass route is deleted, not hidden.** `ConfiguredReceiptBypass` and
`IntentPublicationGate` are gone; `ApplyAuthorization` has one arm. `RecordedApplyPolicy::
ConfiguredBypass` and `ReceiptPolicyWitness::configured_bypass()` remain, because the document
GRAMMAR can spell that posture and a projection must be able to write the row — but no route
turns one into a permit, and `publish_through` refuses such an intent BEFORE calling the
placement.

**Fences.** The lexical roster `the_durable_publication_proof_is_minted_by_one_production_file`
(over `of_required_placement`) is RETIRED: the type now carries the property, so no list is kept.
In its place `crate_boundary.rs` asserts the mint's privacy directly and that the two deleted
spellings have not returned — an assertion over one file's own text, not a roster of permitted
callers.

**Testability kept.** Exhaustive mismatch and affine-transition cases live in
`receipt/tests/dispatch.rs` and drive the real chain with a modelled landing. `apply_projection`
and `cli/tests/receipt_route.rs` reach a `MutationDispatched` the same way — through
project/account/publish/permit — because there is no shortcut left and a battery should not
pretend there is.

**Falsified.** The four new compile-fail pins were re-run un-fenced: `E0061` (an argument with
nowhere to go), `E0451` twice (a private field), `E0433`/`E0425` (a name that is gone). The error
codes are recorded beside the pins so a later reader knows what each one is actually refusing.

## `30Ri:fnd-trusted-signer-remains-forgeable`

**Shape: the receipt crate stops claiming trust.** The forgeability was not a bug in the resolver;
it was that `TrustedReceiptVerificationKey` and `VerificationKeyResolver` were both public traits,
so `Receipt<_, _, TrustedReceiptSigner>` meant "somebody said so". No arrangement of Rust
visibility inside `dorc-receipt` can fix that — it cannot tell one downstream crate's key from
another's — so the claim moved rather than being re-plumbed.

- `capability`: two marker traits become one `ReceiptVerificationKey`, which asserts nothing about
  ownership; `VerificationKeyResolver` keeps one method, `material`.
- `model`: `SignerTrust`, `TrustedReceiptSigner`, `SelfAssertedReceiptSigner` are DELETED, and with
  them the third type parameter on `ReceiptSignatureChecked`/`ParsedReceiptSkeleton`/`Receipt`, the
  `Checked`/`ReadPlain`/`ReadRich` two-armed enums, and `signer_provenance()`. `read_plain` and
  `read_rich` answer one sealed value: the signature is VALID under material the resolver held.
- `graph`: `ingest_*` takes `RecordedSignerTrust` as a VALUE from the ingesting seat instead of
  deriving it from a marker. The field is report-only and its doc says whose statement it is.

**Where local authentication went.** `cli::durable::LocallyAuthenticated<T>` — a private-field
newtype whose only values are the ones `ReadEdge::read_{plan,intent,outcome}` produce, and a
`ReadEdge` exists only where `LocalReceiptEdgeV1::open_for_read` opened and validated the keyset
under this controller's own standard roots. `recorded.rs`'s listings and `recorded_facts.rs`'s
`SelectedRoot` take the envelope, so the report surfaces cannot be fed a document that did not come
through it. `Debug` is redacted; a derived one would have been a second plaintext exit.

**Kept, deliberately.** The self-asserted path is now the ordinary path: `Ed25519Verifier`
implements the one marker, and conformance vectors, imported receipts and bounded debugging all
read through it exactly as before. Nothing lost a route; what was lost is a LABEL nobody could
constrain.

**Fences.** `verification_material_is_supplied_from_one_production_file` is now a HYGIENE census,
and says so in its own words: neither the marker nor the resolver decides trust any more, so the
one-entry resolver list buys "a second key-policy seat is a diff somebody reads" and nothing
stronger. It additionally asserts the two retired marker traits are named nowhere in production,
and fences `SignerTrust`/`TrustedReceiptSigner` two-way to `receipt/src/lib.rs`, where the
compile-fail pin that proves them gone is the ONE naming. `MAY_NAME_THE_READ_BACK_WRAPPER` gains
`cli/src/durable.rs`. A new assertion pins the envelope's field privacy and that its mint is
reachable from one file.

**RESIDUAL.** `LocallyAuthenticated`'s mint is closed to code OUTSIDE `dorc-cli`; inside that
crate, any module could write the literal. Rust offers `pub(in path)` for the constructor but not
for a tuple-struct literal, so the narrowing is the crate, plus the lexical assertion above naming
the one file that writes one. `dorc-cli` is `publish = false` and reached only by `dorc-loom` and
the two binaries, so the crate boundary is the real perimeter.

**Falsified.** The envelope's compile-fail pin re-run un-fenced: `E0423`, a tuple struct with
private fields.

## `30Ri:fnd-opaque-plaintext-remains-extractable`

**Shape: split the writer's input from the reader's storage, and leave the reader one exit.**

- `OverlayEntry` is now explicitly the WRITER-side value. A caller building one already holds the
  bytes, so offering them is not a release. Its `bytes()` is crate-private; `Debug` is redacted;
  `PartialEq`/`Eq` are gone; `agrees_with` answers a `ByteAgreement`.
- `ValidatedOpaqueOverlay` is the READER-side storage. `entries()` is gone, `value()` is
  crate-private, `Debug` is redacted, `PartialEq`/`Eq` are gone. What survives publicly is
  `slots()` — record ordinals and closed tags, structure and never content — and `agrees_with`.
- `Receipt::{detail,region}` and `Reingested::detail` are crate-private.
- THE EXIT: `Reingested::recorded_details()` / `recorded_detail()` answer `RecordedDetail`, which
  carries a `RecordedValue` sealed under `ValueClass::of_tag(tag)`. `ValueClass` grew from five
  names to eleven and the tag→class map is TOTAL and exhaustively matched, so a new tag cannot
  land unclassified and take a neighbour's encoding.
- The listing adapter (`cli::recorded::opaque_lines`) walks that exit through a
  `TerminalEncoder` matching every class BY NAME. Today every class answers one question; the
  exhaustive match is what makes a twelfth class a red build rather than a silent default.
- The `RecordedWhyFacts` builder was already class-aware; what it gained is the crate-private
  `detail` it now reads through, plus redacted `Debug` on `CurrentSourceReading` and
  `WhyFactsInput`, both of which were printing a whole source file.

**Comparison, not equality.** `RecordedValue::agrees_with` became public alongside the two new
ones. All three are all-or-nothing and say nothing about WHERE two runs differ, so a caller cannot
walk a value out of them; a derived `PartialEq` would compose into orderings and hashes, which do
leak structure, and none of the three plaintext-bearing types has one.

**Ripple, named.** `ProjectedPlan`, `ProjectedApplyIntent` and `ProjectedApplyOutcome` lost their
derived `PartialEq`/`Eq` (they hold details). Four batteries that compared bytes now compare
verdicts, and each gained a NEGATIVE arm — a cousin image, a transposed context, a normalized
CRLF, a truncated excerpt — because a verdict comparison is only worth as much as its ability to
say no.

**Sentinel.** `recorded_why_facts.rs`'s `no_debug_or_report_surface_carries_the_recorded_bytes`
asks five surfaces — the read-back document, its sealed model, the model input, the derived facts,
and a current-source reading — for a run of the fixture book, with a non-vacuity floor on each and
the encoder as the positive control. FALSIFIED: with the region's `Debug` restored to a derived
one, it fails on the first surface and prints the book's own shell text, which is the leak.

**No lexical fence added.** Nothing here is a roster; every property is carried by privacy, by a
redacted `Debug`, or by an exhaustive match.

**OPEN, for the conductor.** `spike/crates/receipt/CLAUDE.md`'s
`inv-reader-writer-states-only-narrow` still lists "trust" among the states the reader keeps
separate, and `inv-identities-never-cross-domains` reads as though a provenance newtype survives.
Both are now describing a parameter that no longer exists. Steering files were out of this
builder's remit, so they are unedited and named here instead.
