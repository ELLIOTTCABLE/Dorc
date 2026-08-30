# 30Ri — secure durable handoff in-flight review

> Tier: quarantined, security-focused in-flight review of the unified `ai/r30-receipt` handoff at
> `535bee8c`. This is one review lane, at one review seat, by one reviewer. Read-only explorers only
> mapped paths, symbols, and callsites. This is **not** a full adversarial panel, an independent
> multi-lineage crosscheck, a complete product audit, or evidence that unmentioned surfaces are
> secure.
>
> Scope: only security/adversarial weaknesses that also encode difficult-to-reverse direction in
> the secure-durables/whylog arc. This ledger records only blockers that must be repaired before the
> handoff continues out of quarantine. It deliberately omits ordinary continuation residue and
> already-accounted deferred work.

## review verdict

Continuation is blocked on four boundaries. The current production CLI generally supplies matching
values by discipline, but the arc's central claim is stronger: types and crate boundaries are meant
to prevent a later caller or agent-authored edit from manufacturing trust, publication, or plaintext
release. At the four seats below, that claim is not implemented.

Keep the remediation narrow. Do not reopen the receipt grammar, algorithms, event graph, source
custody, retention, why arrangement, or correctness kernel. Do not replace a failed type boundary
with another lexical roster.

## `30Ri:fnd-dispatch-publication-remains-unbound`

**Blocker.** A publication earned for one apply intent can clear the dispatch gate for another.

- `receipt/src/dispatch.rs:593-616`: `ExactApplyImagesPresent` is an unbound unit witness.
- `receipt/src/dispatch.rs:644-665`: `DurablePublicationProof::of_required_placement` is a public
  constructor; its production restriction is only the lexical fence in
  `receipt/tests/crate_boundary.rs:834-845`.
- `receipt/src/dispatch.rs:722-743`: `PublishedApplyIntentV1::minted` compares the proof with a
  caller-supplied receipt ID, ignores `policy_identity`, and receives no prepared-intent identity.
- `receipt/src/dispatch.rs:797-812`: `IntentPublicationGate::permit` accepts an arbitrary
  `PreparedApplyIntent` and does not compare it with the published arm's receipt ID, document digest,
  policy, or image witness.
- `receipt/src/dispatch.rs:618-631`: the bypass capability is also publicly mintable.

Thus gate A plus prepared intent B produces a permit for B. A caller can also manufacture the
nominal durable proof without publishing anything. The real CLI's present call order does not repair
the authority API.

### minimal required repair

1. Bind one private, collision-resistant prepared-intent commitment to the exact session,
   generation, policy, ordered assignments, and canonical images.
2. Carry that same commitment through rich-intent projection, required local publication, and the
   published gate; `permit` must consume the matching prepared intent or a value that already owns
   it. No caller-supplied ID may perform the binding.
3. Remove public production mints for `DurablePublicationProof` and `ConfiguredReceiptBypass`.
   Fixture publication/bypass values must be structurally test-only and unable to satisfy the
   production required arm.
4. Add failing-direction tests: publication A cannot dispatch B; A's image witness cannot pair with
   B; a wrong policy/digest cannot clear the gate; an external crate cannot mint the production
   proof or bypass.

Do not broaden this into transport or multi-target work. This repair ends at the existing
`MutationDispatchPermit` mint.

## `30Ri:fnd-trusted-signer-remains-forgeable`

**Blocker.** Any downstream crate can label arbitrary verification material as controller-trusted.

- `receipt/src/capability.rs:34-54`: `TrustedReceiptVerificationKey` and
  `VerificationKeyResolver` are public and unsealed.
- `receipt-crypto/src/lib.rs:121-143`: `TrustedEd25519Key::of` is public.
- `receipt/tests/graph.rs:83-101`: an integration-test crate successfully implements the trusted
  marker and resolver, demonstrating that this is ordinary external visibility rather than a
  theoretical concern.

`check_signature` turns whichever `trusted()` answer a caller supplies into
`Receipt<_, _, TrustedReceiptSigner>`. The current local resolver checks its key ID correctly, but
that implementation discipline does not make the public trust type authoritative. A workspace grep
cannot constrain future crates or downstream consumers.

### minimal required repair

1. Make trusted-key admission an unforgeable capability minted only from the validated local-keyset
   policy path. Remove the public trusted-wrapper constructor and caller-implementable trusted
   marker route.
2. Keep generic/imported verification material self-asserted unless a concrete controller policy
   explicitly promotes it through the closed mint.
3. Add an external-crate compile-fail pin proving arbitrary verification material cannot yield a
   `TrustedReceiptSigner` receipt. Keep a positive production-keyset read proving the real route.

Do not build key import, rotation, provider selection, or trust-distribution UX in this repair.

## `30Ri:fnd-opaque-plaintext-remains-extractable`

**Blocker.** The sealed facts model is not the only plaintext exit; lower public receipt APIs expose
and format decrypted detail directly.

- `receipt/src/reingested.rs:286-292` and `receipt/src/reader.rs:354-360`: public `detail()` methods
  return raw `&[u8]` from reingested rich receipts.
- `receipt/src/overlay.rs:66-95,511-529`: `OverlayEntry::bytes`,
  `ValidatedOpaqueOverlay::value`, and `entries` expose plaintext; both types derive revealing
  `Debug`/comparison implementations.
- `receipt/src/reader.rs:206-234` and `receipt/src/reingested.rs:115`: `Receipt` and `Reingested`
  derive `Debug` over that region.
- `receipt/src/report/build.rs:30-81`: current-source readings and `WhyFactsInput` derive `Debug`
  over source bytes.

`RecordedValue`'s encoder-mediated exit is sound only after a caller voluntarily enters that newer
API. The older, easier raw path remains public and is already used by the listing adapter. Debug,
panic, assertion, or future diagnostic output can therefore carry exact source/host bytes and
terminal control sequences around the sink encoder.

### minimal required repair

1. Split writer-side opaque input from reader-side validated opaque storage if needed; the read side
   must expose neither raw entries nor byte slices.
2. Route the existing listing adapter and `RecordedWhyFacts` builder through one receipt-owned,
   value-class-aware encoder/visitor exit. No `Reingested` raw-detail accessor remains public.
3. Replace every plaintext-bearing derived `Debug` with a redacted implementation carrying only
   type, class, IDs where public, and byte counts. Remove plaintext-bearing `Eq`/`Hash` surfaces
   unless a closed in-crate comparison returns a typed result.
4. Add a sentinel test proving `Debug`, errors, and report plumbing do not contain opaque bytes,
   plus a positive test proving the explicit encoder can render them.

Do not implement general secret recognition or taint analysis here. This repair only makes the
already-ruled sink-encoding boundary real.

## `30Ri:fnd-local-nofollow-contract-remains-unimplemented`

**Blocker.** The production operation named `OpenExistingNoFollow` performs a metadata pre-check
followed by an ordinary pathname open.

- `receipt-local/src/native.rs:15-21` acknowledges the swap window.
- `receipt-local/src/native.rs:75-82,188-198,346-354` runs `symlink_metadata`, then
  `OpenOptions::open`, which may follow a replacement final component.
- `receipt-local/src/store.rs:1255-1305` validates store directories against write bits and redirect
  shape but does not require effective-user ownership, despite `30Rd:controller-root-resolution`.
- Removal ownership in `receipt-local/src/native.rs:42-44,144-160` is retained as a path string,
  not an opened-object identity.

The default private roots reduce exposure, but explicit store roots, mutable ancestors, mounts, and
sync races are part of the durable edge this abstraction is meant to bound. The accepted weaker
Windows posture does not authorize silently using the same pathname claim on Unix.

### minimal required repair

1. On Unix, use retained directory handles and handle-relative non-follow opens for authority-bearing
   key, manifest, receipt, and owned-removal operations. Revalidate the opened object, not the name.
2. Require effective-user ownership for Unix store landings while preserving the separately ruled
   read-permission posture.
3. Bind cleanup/removal to the object identity created by this attempt; a remembered pathname is
   insufficient after the namespace may have changed.
4. Leave the explicitly weaker Windows baseline explicit and unchanged unless the selected safe API
   supports an equivalent improvement. Add native Unix swap/redirect and wrong-owner cases.

Do not build a general filesystem capability layer or ACL framework. Repair only the existing local
key/store operations whose names and publication proofs already claim these properties.

## repair exit

This review lane is discharged only when all four blocker slugs have a code repair and a
failing-direction test at their claimed boundary, focused receipt/crypto/local/CLI tests are green,
and `mise run both gate:full-quiet` is green from the unified repair tip. The repair ledger must name
any remaining lexical fence used as authority; none may be silently inherited as a type guarantee.

No full adversarial-review or whole-product-security claim follows from that green. It closes only
this single-seat in-flight review.
