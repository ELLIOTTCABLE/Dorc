=== DISPATCH: sol | mode=review ===

You are adjudicating SPEC COMPLIANCE, and nothing else.

You are not a code reviewer here. Do not comment on style, naming, architecture quality, test coverage, performance, or security posture. Do not propose improvements. Every one of those is out of scope and will be discarded.

Your one question, applied to each item in the list below:

**Does this departure from the specification represent (A) compliance after all — the spec permits it and the conductor misread it as a deviation; (B) a soft spot or omission in the spec, where the departure is the correct reading of what the spec meant; (C) a genuine departure the spec does not license, which a human must rule on; or (D) a departure that contradicts the spec and should be reverted?**

Where the spec is genuinely soft — silent, ambiguous, or self-contradictory — say so plainly and give your adjudication of what it should mean. That is the second half of your job and the reason you were asked rather than a general reviewer.

## Read these first, in full

Both live in the repository at your working directory:

- `Research/quarantine-DO-NOT-READ/30Ra-durable-whylog-security-review.md` — the product and design authority, with rationale.
- `Research/quarantine-DO-NOT-READ/30Rb-secure-durable-receipts-build-specification.md` — the build specification. Its `30Rb:how-to-read-requirements` section defines REQUIRED / TYPE LEAN / IMPLEMENTOR CHOICE / V1 STRETCH / LATER / STOP, and those labels are the backbone of your adjudication. A TYPE LEAN departure is nearly always (A) or (B); a REQUIRED departure is where your attention belongs.

You may read the implementing code to check a claim. The built tree is at your working directory under `spike/crates/receipt/`, `spike/crates/receipt-crypto/`, `spike/crates/core/`, `spike/crates/plan/`, and `spike/crates/cli/`. Read what you need to answer accurately; do not audit beyond the listed items.

Note: the specification is authoritative over the conductor's reasoning, but the repository's own root documents (`README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `AGENTS.md`) are human-authored and outrank the specification — `30Ra` says so itself. Where a departure was justified by appeal to one of those, check the appeal.

## The departures, as the conductor recorded them

### Crate structure
1. **Two crates, not one.** `30Rb:result-and-exit` item 1 requires one standalone `dorc-receipt` crate owning models, format, identities, projections, graph, reader/writer states, **and crypto adapters**. As built: a pure `dorc-receipt` plus a `dorc-receipt-crypto` holding only the Age/Ed25519 implementations of the pure crate's capability traits. Justification given: `age` depends on `rand`, `dorc-plan` depends on `dorc-receipt`, and root `AGENTS.md` states "correctness-critical kernels *must* stay clean of nondeterministic deps (or deps at all)". A cargo feature gate was rejected on workspace feature-unification grounds. All authority mints (checked/trusted/complete states) stay in the pure crate.
2. `sha2` remains a dependency edge reaching `dorc-plan` (the last kernel stage), accepted as deterministic.
3. `crypto.rs` renamed `capability.rs` in the pure crate.
4. `check_signature` returns a two-arm concrete enum rather than a generic, on the grounds that a generic would let a caller request its preferred trust marker, which `30Rb` forbids.

### Grammar table amendments (made by the conductor at the `30Rb` Stage 0 checkpoint)
5. `invocation` row gained an `attempt` field, not in the spec's field list. Justified by `sinv-controller-attribution` naming attempt identity.
6. `admission` row gained an opaque-capable `stream` field, not in the spec's field list, so that opaque tag 4 (`record-stream`) has a slot; the spec requires the admitted record bytes in rich but gives them nowhere to land.
7. `render-decision` row gained a `member` field: `subject member kind detail account`. Justified by sibling rows carrying `leaf member`.
8. `OpaqueFieldTag` gained a 17th member, `apply-context` at order 16, because `apply-assignment.context` had no tag.
9. `survival.outcome` split to eight tokens and `site-classification.class` to eight, rather than the source enums' five and six, to avoid merging variants with differing repair/authority/epistemic meaning.
10. `solve-certification.pass` projects the full five-token set though only one is reachable today.
11. `region-decision.routes` records a total; the keyed/unkeyed split takes a projection-omission rather than a field.
12. `nonce` deliberately NOT recorded (it carries a fixture-pin environment override; the skeleton forbids environment-sourced values), while `attempt` is.

### Type and API departures
13. `Projection` gained an associated `type Region` (`Plain → NoOpaqueOverlay`, `Rich → ValidatedOpaqueOverlay`).
14. `OverlayPlaintext::of` narrowed from public to crate-private.
15. Three refusal families exist rather than one: `RefusalReason` (grammar), `ImageRefusal` (image), `ModelRefusal` (model), with `RefusalReason::Image(..)` and `::Overlay(..)` nesting.
16. `Reingested::as_report` — a generic accessor returning `&T` — was removed entirely.
17. `PartialReceipt` is non-generic; `30Rb:recorded-receipt-graph`'s TYPE LEAN writes `PartialReceipt<PartialReason>`.
18. `OutcomeAvailability` is non-generic for the same reason.
19. `ReceiptGraph` stores recorded models plus a trust token rather than heterogeneous `Receipt<D,P,T>` values.
20. No shared `NonEmpty<T>` type was minted; private `Vec` fields sit behind validating constructors that refuse empty.
21. Identity constructors renamed from `X::over(&[u8])` to `of_canonical_inputs` / `of_canonical_decision` / `of_canonical_image`, so a whole-identifier lexical fence can match them. This is a change to a Stage 1 public API after Stage 1 was declared frozen.
22. Two of those three mints are currently UNFENCED, because the fence is two-way and a fence with no subject fails its own check; their callers arrive in a later boundary.

### Semantic rulings
23. **A rich-to-plain remint KEEPS the rich document's identity.** Consequence: the graph's "one identity, differing bytes" finding was refined to fire only on same-identity-same-projection; same identity across different projections correlates to one node.
24. The overlay's inner skeleton digest is **plain SHA-256** over the literal skeleton span, not domain-separated through the PAE envelope, on the grounds that it is a binding check rather than an identity mint and that `sha256sum`-by-hand reproducibility serves the readable-format goal. The crate's own drafted law said every derived identity hashes through the envelope.
25. `licensor` rows are emitted for site dispositions only; a shared region's license takes a projection-omission rather than a row axis.
26. `PresentedPlanId`: `30Rb:receipt-identity-map` specifies one associated type on `DecidePlane`. As built, that one associated type is bound to a plan-owned struct carrying **three** identities (planning-input, presented-plan, planned-image), because the `presented-plan` row carries three and `core` can name none of them.
27. `pre-target-specialization-final`, one of the seven `PresentedPlanId` mintability preconditions, is recorded **satisfied-vacuously** — the spike is width-one host and the precondition has no referent in the tree.
28. `pre-settlement-quiescent` was made typed rather than left as a boolean plus a debug assertion.

### Changes to `core` and `plan` that `30Rb` does not name
29. **`SourceRole` moved into `core`; `SourceClaim` widened to `{path, digest, role, bytes}`; `SpineInvocation`'s `book` + `oracles` collapsed into one ordered `sources` vector.** Justification: `30Rb` requires ordered source identities carrying role, ordinal, digest and byte length; the ordinal means load order; actual acquisition order is oracles-then-book; a two-way split therefore cannot express a truthful ordinal. (The human has given this a gentle, temporary, explicitly-unwelded ack and wants outside grounding.)
30. `SpineSiteClassification.class` became a closed eight-arm enum in `core`, replacing a `&'static str` whose producer discarded two booleans the receipt's token set needs.
31. `SpineSiteClassification` gained an `ast` field; the spec's row requires `ast` and the record had none.
32. Classification recording moved out of `cli` into `plan::settle`, deleting a hand-built `ast → leaf` back-map in favour of a mapping the settlement already computes.
33. `Settlement` replaced `capped: bool` + `discarded_on_cap: u32` with a typed quiescence enum.
34. `SpineDigest` renamed `SpinePresentedPlan` across `core`, the recorded species, and the wire token (`digest` → `presented-plan`).

### Schedule departure
35. `plan::invocation::book_digest` (a hand-rolled SHA-256) was folded onto the receipt crate's SHA-256 during Stage 3. `30Rb:staged-build-order` schedules that removal for Stage 6. Justification: measured byte-identical, `plan` gains the dependency in Stage 3 regardless, and root `AGENTS.md` says not to defer what can be done now.

### Conductor-initiated work outside the specification entirely
36. A **gate discovery floor** was added: a new `internal-tooling` subcommand and `mise` task inserted into the project's completion gate, which refuses a gate run that would check nothing. Motivated by a measured false green (the gate returned success having executed no checks; three distinct causes were found, one being that a pure-deletion changeset selects zero checks). Not mentioned anywhere in `30Ra` or `30Rb`.
37. `.gitattributes` binary pins for byte-exact vector extensions; a `_typos.toml` regex rule preventing a linter from rewriting base64 inside committed vectors; a crate-scoped `#![expect(clippy::multiple_crate_versions)]`.

## Your report

**Emit the complete adjudication as your FINAL MESSAGE.** That message is captured verbatim as the report. You have no write tool and need none — do not ask where to put it, do not write a file, and do not ask a clarifying question about the destination. Keep it MINIMAL and FOCUSED — the conductor asked for that explicitly, and a long report is a worse report here.

Format: one line per item, numbered to match above, each carrying its verdict letter (A/B/C/D) and at most two sentences of reasoning. Then a short closing section listing ONLY the items you graded C or D, in priority order, since those are the ones a human must look at. If an item's spec basis is genuinely soft, mark it and say what you think the spec should be read to mean.

Do not restate the spec back. Do not summarize what was built. Do not include praise or general observations.

Constraints: do the work yourself and do NOT spawn subagents. If your file-reading tools or web search are unavailable, ABORT and report that gap rather than reasoning from training priors.

=== END DISPATCH: sol ===
