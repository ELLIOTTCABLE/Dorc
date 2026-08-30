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
