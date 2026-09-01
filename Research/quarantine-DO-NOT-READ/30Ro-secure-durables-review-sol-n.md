# 30Ro — Secure-durable receipt implementation review, Sol-N

> Review base: `7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`.
> Result reviewed, not box compliance. Concurrent work imported by the range was considered only where it composes with the receipt family.
> No product code was changed.

## Headline

The implementation has unusually strong strict-format, projection, local-store, and read-back-sealing work. Its central pre-dispatch claim does not yet hold at the type boundary, however: an ordinary caller can fabricate the “required placement” value and mint a mutation permit without any store operation. The shipped apply route also publishes before it has a genuine resolved session, `--no-receipt` is ignored on that mutative route, and receipt-root selection is still a filename-shaped whole-store listing rather than authenticated rooted graph selection. Those are architectural defects, not missing polish.

## Verified defects

### `30Ro:required-publication-proof-is-caller-mintable`

- **Severity:** High
- **Confidence:** +SURE
- **Code:**
  - `spike/crates/receipt/src/dispatch.rs:662-670` publicly exposes `RequiredPlacementLanding::of(document_digest, policy_identity)`.
  - `spike/crates/receipt/src/dispatch.rs:725-746` accepts any caller closure as `place`, checks only that its caller-supplied digest equals another caller-supplied digest, does not validate `policy_identity`, and privately mints `PublishedApplyIntentV1`.
  - `spike/crates/receipt/src/dispatch.rs:795-813` turns that value into `MutationDispatchPermit`.
  - `spike/crates/receipt/tests/dispatch.rs:121-122,409-420` is a concrete cross-crate demonstration: an in-memory fixture constructs the landing, returns it from a closure, and spends the resulting permit. The focused test passed.
- **Governing claim:** `30Ra:pre-dispatch-publication-boundary` at `30Ra:811-815` says a permit cannot be minted from a fixture landing; `30Ra:822-824` says the concrete store's private required-grade proof is what production consumes and fixture/volatile proof cannot satisfy it. `30Rd:838-842` repeats that V1 has no fixture bypass.
- **Failure world:** a future caller prepares and image-accounts an intent, computes `sealed`, returns `RequiredPlacementLanding::of(sealed, "required-local-v1")` from the public closure without touching disk, and calls `.permit().spend()`. All steps compile. The current default binary caller does use the real store, so this is not a claim that today's `main.rs` already takes the bypass; it is a claim that the advertised type invariant is false and the next caller can bypass it accidentally.
- **Consequence:**
  - **Admin:** a mutative dispatch can occur with no durable intent despite the required policy.
  - **Engineer:** the type names and tests suggest “publication happened,” while the API proves only “a closure returned a matching digest.” A fixture is positively demonstrated as permit-capable.
- **Smallest repair direction:** move the permit mint to the dependency-outward composition seat that can consume `RequiredLocalPublicationV1`'s private proof directly. Do not accept a public primitive landing report as authority. DST should either use a separate non-production transition or drive the deterministic store model to earn the same opaque proof; it must not satisfy the production mint with a caller-constructed digest pair.

### `30Ro:receipt-root-selection-is-not-authenticated`

- **Severity:** High
- **Confidence:** +SURE
- **Code:**
  - `spike/crates/cli/src/main.rs:474-520` always opens and enumerates the selected/default store, even for `--receipt <file>`; it keys `RecordedDocument` and the maximum cohort from `entry.name().receipt_id()` and filename order.
  - `spike/crates/cli/src/main.rs:537-567` verifies receipt bytes but never calls `OwnedReceiptEntry::agreement`; every store/read/verification reason is discarded into untyped `unread`/`None` at `494-496` and `553/560/567`.
  - `spike/crates/receipt-local/src/store.rs:561-581` contains the required filename/header comparison, but its only live callers are tests.
  - `spike/crates/cli/src/engine.rs:2151-2158` makes `ReceiptRoot::File` select no document at all.
  - `spike/crates/cli/src/engine.rs:2200-2209` appends the graph rendering for the entire store unconditionally, after filtering only the document listing.
- **Governing claim:** `30Ra:208-223` rules one authenticated attention root, direct `--receipt <file>` admission, and no disconnected graph contribution. `30Rd:242-246` permits latest selection only after filename/header agreement; `30Rd:764-770` requires internal version/species/order/id comparison and authenticated cohort derivation.
- **Concrete failures:**
  1. Rename a valid signed receipt with internal id/order `Y/old` to a valid-looking filename claiming `X/new`. `--receipt-id X` selects the filename record and emits a listing whose authenticated first line is `receipt Y`. `--receipt-last` can select it by the forged filename order. The existing primitive agreement test does not affect this path.
  2. `--receipt path/to/file` never opens that file. If the default store is absent, the command fails at store open; if the store exists, no document listing can match because `File(_) => false`.
  3. Two disconnected receipt histories in one store: selecting one root still prints every edge/finding from both histories. A newest unreadable member can be followed by unrelated older graph output, preventing even the generic empty-output refusal.
- **Consequence:**
  - **Admin:** `why` can confidently answer the wrong named/latest document, disclose unrelated receipt relationships, or ignore the explicit file they supplied.
  - **Engineer:** root selection, graph construction, and filename agreement are three separate mechanisms whose current composition defeats each one's local tests.
- **Smallest repair direction:** acquire the requested root first (explicit file by bounded direct read; id/latest only after verified filename/header agreement), then build and render only `ReceiptGraph::closure_from(root)`. Preserve every partial/read/authentication reason as a typed sibling/root state. Filename claims may remain findings but must not choose an authenticated identity or order when they disagree.

### `30Ro:apply-ready-target-is-only-an-argv-spelling`

- **Severity:** High
- **Confidence:** +SURE
- **Code:** `spike/crates/cli/src/apply.rs:378-386` makes a “resolved” context from the addressed host spelling and five `NotEstablished` axes; `401-410` immediately wraps it in `ReadyApplyTarget`/`ApplySessionReady`; `449-452` spends the permit before the first real `SessionDriver::run`. The transport driver is not even built until `spike/crates/cli/src/main.rs:2506-2511`, after intent publication machinery is ready.
- **Governing claim:** `30Rb:1039-1048` requires either a genuine standup or a non-authorizing bypass and explicitly says “Faking `ReadyApplyTarget` from the host string is forbidden.” `30Rd:841-842` requires genuine standup identity on the default route.
- **Failure world:** an SSH config alias, DNS change, proxy/jump rule, authenticated account, or local debug transport sends `web9.example.net` somewhere different from the bare argv implication. The signed intent says the target is ready before any transport has resolved or authenticated anything; all user/account/namespace/cwd/environment/credential axes remain unknown.
- **Consequence:**
  - **Admin:** the write-ahead record can misattribute which host/context an authorized mutation was sent to, the project's highest-ranked explanation failure.
  - **Engineer:** `ReadyApplyTarget` no longer means what future call sites and type reviews will assume it means.
- **Smallest repair direction:** add a genuine transport standup returning immutable controller-owned resolved target/session/context and retain that same session through dispatch. Until that exists, the one-shot SSH route must refuse the required arm rather than mint `ReadyApplyTarget` from argv.

### `30Ro:no-receipt-is-ignored-for-host-apply`

- **Severity:** High
- **Confidence:** +SURE
- **Code:** `spike/crates/cli/src/lib.rs:533-539` promises `--no-receipt` writes no receipt and says refusal must be typeable. `spike/crates/cli/src/main.rs:182-185` diverts host apply before the only `!args.no_receipt` use at `209-213`; `ship_consented_apply` unconditionally opens the production receipt edge at `2492-2497` and publishes intent/outcome.
- **Governing claim:** `AID-NEEDS:law-whylog-is-sensitive` at `AID-NEEDS.md:140-142` treats every receipt as sensitive. `30Rd:838-842` allows no V1 bypass, so the safe interpretation of “no receipt” on apply is refusal before dispatch, not silent persistence and not mutation without intent.
- **Failure world:** an operator runs `dorc apply --host H --plan P --no-receipt` specifically to avoid persisting host, plan, and execution material. Dorc writes rich intent/outcome receipts anyway.
- **Consequence:**
  - **Admin:** explicit retention/privacy intent is violated on the command that persists the most sensitive artifact family.
  - **Engineer:** one flag has opposite behavior depending on whether apply uses the engine route or the host shortcut.
- **Smallest repair direction:** reject `--no-receipt` with mutative host apply before key/store/transport setup, naming the required-intent constraint. Do not add a bypass in V1.

### `30Ro:entropy-failure-mints-fixed-zero-identities`

- **Severity:** High
- **Confidence:** +SURE
- **Code:** `spike/crates/receipt/src/ids.rs:171-177` sets `faulted` when entropy refuses but still returns an all-zero `ReceiptId`; `166-167` exposes a later `intact()` check. A workspace-wide caller search found no `.intact()` call. `ReceiptId::of_source_bytes` is also public at `116-117` despite the production-fence claim.
- **Governing claim:** `30Rb:242-245` requires collision-resistant controller-minted identities and structurally test-only deterministic sources. `AGENTS.for-builders-only.md:38` (`sinv-production-fences`) forbids fixed identities from default persistence and real transport.
- **Failure world:** `getrandom` fails once. The production plan route signs and stores a zero-id receipt; the production apply route mints zero session, generation, target, intent, and outcome identities and can dispatch. A later same-species write collides with the fixed filename rather than receiving an explicit entropy refusal.
- **Consequence:**
  - **Admin:** history can become unpublishable or ambiguously correlated exactly under platform degradation, while an apply may still mutate.
  - **Engineer:** correctness depends on remembering an out-of-band latch after a trait whose return type says success; current callers do not remember it.
- **Smallest repair direction:** make identity minting fallible (`Result<ReceiptId, EntropyFailure>`) and propagate failure before any receipt/publication/permit transition. Remove the separate `intact` protocol and make the raw-byte mint private or lexically test-only.

### `30Ro:apply-outcome-never-records-sites`

- **Severity:** Medium
- **Confidence:** +SURE
- **Code:** `spike/crates/cli/src/apply.rs:452-460` maps every `SessionOutcome` to `ApplyOutcomeReport` with `Vec::new()` site reports. The receipt model supports per-site outcomes (`spike/crates/receipt/src/project.rs:382-446`; `outcome.rs:253-331`), but the production apply route never populates them. `spike/crates/cli/tests/durable_route.rs:307-316,356-403` deliberately proves only intent/outcome correlation and an inert whole-script terminal status.
- **Governing claim:** `30Rb:966-970` requires per-site/assignment identity and status available from the V1 DST route; Stage 4 at `30Rb:1321-1326` requires current `SessionOutcome`/hostsim per-site mapping.
- **Failure world:** a multi-command plan fails halfway. The outcome says only `command-failed` for the whole shell and `sites 0`; it cannot answer which guarded/replaced/run site executed, failed, or remained unattempted.
- **Consequence:**
  - **Admin:** the durable fails its central post-incident question, “what actually ran?”
  - **Engineer:** the rich typed site model and extensive tests over hand-built reports can look complete while the one production mapper always supplies an empty population.
- **Smallest repair direction:** route an execution adapter that emits bounded per-site/assignment outcomes into `ApplySiteReport`, with an explicit whole-script/uninstrumented omission state where production cannot know. The required V1 acceptance route must observe at least one non-empty site population.

### `30Ro:missing-age-key-hides-authenticated-skeleton`

- **Severity:** Medium
- **Confidence:** +SURE
- **Code:** `spike/crates/receipt-local/src/keyset.rs:681-700` intentionally returns `VerificationReady` when only signing material survives, but `spike/crates/cli/src/durable.rs:263-313` checks for an opener first and returns `RegionUnopenable` before signature verification/parsing for all species.
- **Governing claim:** `30Rd:557-561,823-825` and acceptance item `30Rd:1184-1185` require authenticated skeleton reading when encryption material is unavailable.
- **Failure world:** `encryption-private-v1.age` is lost or damaged while signing material and manifest remain valid. `dorc why` cannot distinguish a valid locally signed skeleton with unavailable detail from an unauthenticated/unread receipt.
- **Consequence:**
  - **Admin:** losing detail custody also loses readable structural recovery, unnecessarily multiplying damage.
  - **Engineer:** the keyset's role-specific state is correct locally but discarded at the next composition seam.
- **Smallest repair direction:** add a verified-rich-skeleton result that runs bound → locate → signature check → skeleton parse without opening detail, and wrap that state as locally authenticated with `DetailState::Unavailable`; never represent it as a complete rich receipt.

### `30Ro:private-key-write-callback-extracts-owned-bytes`

- **Severity:** Medium
- **Confidence:** +SURE
- **Code:** `spike/crates/receipt-crypto/src/key_document.rs:90-101` explicitly demonstrates copying a private identity to `Vec<u8>`; both generic public callbacks return arbitrary `R` at `277-279` and `370-373`. The production initializer does exactly that into ordinary, non-zeroizing vectors at `spike/crates/receipt-local/src/keyset.rs:493-502`.
- **Governing claim:** `30Rd:388-393` allows only a narrow persistence callback and no raw accessor; `30Rd:1076-1084` requires scoped exposure, minimized copies, and zeroized mutable intermediates.
- **Failure world:** any caller returns or captures an owned copy from the generic callback; today initialization retains two ordinary `Vec<u8>` copies until function exit, without zeroizing their buffers.
- **Consequence:**
  - **Admin:** key bytes have a wider accidental logging/crash-dump/lifetime surface than the custody design claims.
  - **Engineer:** compile-fail coverage proves a borrowed slice cannot escape, while the easier owned-copy escape is a documented success case.
- **Smallest repair direction:** replace the arbitrary-return callback with one persistence-specific, caller-censused operation and zeroizing owned buffers at the I/O seam. At minimum constrain the callback result to a status type, prohibit returned material in API tests, and wrap every unavoidable copy in `Zeroizing`.

### `30Ro:fresh-keyset-children-are-created-by-path`

- **Severity:** Medium
- **Confidence:** +SURE
- **Code:** on successful directory creation, `spike/crates/receipt-local/src/native.rs:145-148` records only a pathname/kind and retains no directory handle. `parent_handle` consults only the `open` map at `102-104`, so subsequent `create_directory`/`create_file` falls back to absolute `mkdir`/`open` at `336-360`. Fresh initialization performs product-root → keys-dir → keyset-dir → key files in that state at `spike/crates/receipt-local/src/keyset.rs:471-502`; handles are opened only later during validation/synchronization.
- **Governing claim:** `30Rd:295-305` requires retained ownership-bearing handles and handle-relative authority-bearing child creation on Unix; `30Rd:321-328` requires each newly created component to be validated before descendants.
- **Failure world:** a sync client or concurrent same-account process moves/replaces a freshly created internal directory between creation and the next child operation. The next absolute path walk follows the replacement ancestor and creates key material outside the directory this attempt created. Same-user malicious code is outside the confidentiality claim, but sync/concurrency replacement is explicitly in reliability scope.
- **Consequence:**
  - **Admin:** first-use can strand or mis-site the only decryption/signing keyset.
  - **Engineer:** native module documentation says child acts are handle-relative, but that is true only for pre-existing/opened parents, not the clean-profile path most important to D4.
- **Smallest repair direction:** after every successful directory create, immediately open non-following, inspect, and retain that directory handle before creating any child; express successful creation as an owned directory token/handle rather than `()`.

## Design concerns

### `30Ro:planning-input-identity-omits-live-controls`

- **Severity:** Medium
- **Confidence:** +SURE
- **Code:** `spike/crates/cli/src/engine.rs:100-106` shows the planner consumes escalation dial and connection capability; `spike/crates/plan/src/planning_input.rs:38-47,87-99,166-171` identities only mode and `risk_faultless_skips`. The production construction passes only those two at `spike/crates/cli/src/engine.rs:1899-1909`. `CONTROLLER_SEMANTICS` is `dorc/0.0.0` for every unreleased build (`spike/crates/cli/src/receipt_edge.rs:38-42`).
- **Governing claim:** `30Rb:263` says `PlanningInputId` identifies the complete planner input tuple; `AGENTS.for-builders-only.md:26` (`sinv-decision-identity`) includes analysis-relevant knobs/admin policy, controller semantics/version, target/context, generation, and executable bytes.
- **Failure world:** the same source/records run once with `--no-probe-escalation` or degraded capability and once with escalated/root capability. Those controls can change which wrapped facts are measurable and therefore decisions, yet the planning-input IDs remain equal. Two different commits also both claim `dorc/0.0.0`.
- **Consequence:**
  - **Admin:** comparison/re-derivation can label distinct planning worlds as the same inputs.
  - **Engineer:** the exhaustive census is internally consistent but exhaustively checks an incomplete list.
- **Smallest repair direction:** make `PlanningPolicy` consume the complete closed analysis-control value (survival, escalation, capability, and every future decision-affecting field) and use a build semantics identity that changes whenever controller behavior can change. Add perturbation tests for every actual `AnalysisOptions` member.

### `30Ro:receipt-draft-mint-is-an-open-field-bag`

- **Severity:** Medium
- **Confidence:** +SURE
- **Code:** `spike/crates/receipt/src/format.rs:138-149` exposes all `Skeleton` fields; `spike/crates/receipt/src/writer.rs:17-30` lets any caller construct any species/projection draft from that bag, and `138-143` signs it. The production `WriteEdge` publicly exposes signer and placement at `spike/crates/cli/src/durable.rs:397-415`. Identity raw-byte/text constructors are similarly public (`receipt/src/ids.rs:116-117,195-245`).
- **Governing claim:** `30Rb:403-415` forbids public fields and bare-string/digest constructors on these boundaries; `30Rb:434-437` says species projectors, not arbitrary callers, create drafts. `AGENTS.for-builders-only.md:28` (`sinv-private-authority-mints`) requires private/narrow mints.
- **Failure world:** a future internal CLI caller opens the local write edge, fills arbitrary well-typed skeleton atoms, signs with the local key, and files a receipt. Read-back calls it locally authenticated even though no authoritative plan/apply projector minted its semantics. This does not itself mint a mutation permit, but it forges the historical account operators rely on.
- **Consequence:**
  - **Admin:** controller-authenticated history can contain invented semantic rows from a locally convenient caller.
  - **Engineer:** negative API tests protect the later read-back wrapper while leaving the original signed-document mint broad.
- **Smallest repair direction:** make `Skeleton` construction/parser-only and `DraftReceipt` construction crate-private behind species-specific projectors or narrowly reviewed producer facades. Keep parsing identities separate from live/controller-minted identities; remove production-visible fixed/raw constructors.

## Open questions

### `30Ro:partial-root-state-needs-one-owner`

- **Severity:** Open question
- **Confidence:** ~SUSPECT
- **Locations:** `spike/crates/cli/src/main.rs:494-567` drops store and reader reasons; `receipt/src/graph.rs:367-376,538-544` can retain partials but production never feeds them; `receipt/src/report/states.rs` has richer states intended for the sealed handoff.
- **Missing ruling/decision:** which one layer owns conversion from local store/read/key failures into root/sibling partial states before final why arrangement? The current listing path and the deferred `RecordedWhyFacts` path duplicate the question.
- **Direction:** the production acquisition should yield one typed rooted input consumed by both the temporary listing and later arrangement. Do not fix this by inventing prose or by keeping two partial-state adapters.

### `30Ro:whole-script-outcome-needs-explicit-product-posture`

- **Severity:** Open question
- **Confidence:** ~SUSPECT
- **Locations:** `spike/crates/cli/src/apply.rs:452-460`; `spike/crates/transport/src/lib.rs` exposes whole-session results; `30Rb:966-970,1321-1326` expects per-site V1 data.
- **Missing ruling/decision:** ordinary external `plan.sh` execution cannot inherently report per-site truth. Is production V1 required to instrument the applied artifact, or should an uninstrumented whole-script route explicitly record that site outcomes are unavailable?
- **Direction:** make that limit a typed product state before broad executor work. An empty vector is not an answer because it conflates “zero sites existed,” “none ran,” and “the executor cannot observe sites.”

## Coverage

- Read the required root product/design/implementation/user/knob/aid documents, root research map, out-quarantine `30R`, all three permitted quarantined designs, builder firewall, spike steering, and relevant receipt/crypto/local/plan/CLI/core/aid crate steering.
- Inspected first-parent history and the full range/name/stat surface. The range is approximately 63k insertions across 444 files; review concentrated on the new receipt family, local durable edge, plan identity/projection, apply gate, production CLI composition, rooted read-back, negative tests, and native/model fault routes.
- Traced live production callers from `main.rs` through plan publication, apply intent/permit/outcome, local key/store, read-back, graph/listing, and sealed report facts. Checked fixture callers specifically where they claim not to satisfy production authority.
- Inspected strict grammar, reader/writer transitions, overlay accounting, image container, key documents, native I/O, store/keyset failure sweeps, graph/reingestion/report APIs, root selectors, and focused tests.
- Focused checks run:
  - `cargo test -p dorc-receipt --test dispatch`: 13 passed.
  - `cargo test -p dorc-receipt-local --test store_sweep`: 29 passed.
  - `cargo test -p dorc-receipt-local --test keyset_sweep`: 17 passed.
  - `cargo test -p dorc-plan --test identity_vectors`: 3 passed.
- One initial focused-check command failed before compilation because it was launched at the repository root without `--manifest-path spike/Cargo.toml`; rerun with the trusted worktree's manifest succeeded. No heavy gate, full workspace suite, Kani, Lean, blessing, real mutator, fixture shell, or network resource was run.

## Strong properties that held

- `30Ro:literal-signed-bytes-hold` — signature verification and skeleton parsing consume copies of the same located exact body/skeleton spans; no semantic reserialization enters verification.
- `30Ro:strict-envelope-and-overlay-hold` — the outer grammar is closed and strict; plain/rich projection mismatch, trailing bytes, count/id/order departures, armor shape, and overlay missing/extra/duplicate/dangling/wrong-field states are represented and tested. The reverse overlay checks both directions before detail release.
- `30Ro:opaque-readback-seal-holds` — validated arbitrary receipt detail has no public raw accessor or revealing `Debug`; `RecordedValue` exits through a class-aware encoder. Recorded dispositions/influence/locators do not convert to live authority.
- `30Ro:apply-image-byte-fidelity-holds` — the image container is length-framed and validates entry/root/edge/path topology without normalization; single-stream and multipart vectors are substantial.
- `30Ro:key-role-separation-holds` — Ed25519 and Age documents, ids, algorithms, and APIs are distinct; key files use library-owned canonical PKCS#8/Age encodings; unused crypto negotiation/plugin surfaces are disabled.
- `30Ro:immutable-store-basics-hold` — final-name exclusive create, no replacement/append/latest/retention, independent read bounds, aggregate graph budget, typed platform sync properties, newest-partial no-fallback primitive, and production cleanup decline are implemented and fault-swept.
- `30Ro:old-format-removal-holds` — old whylog parser/writer/store/replay authority is removed rather than retained behind compatibility adapters.
- `30Ro:post-dispatch-durable-direction-holds` — the live apply path spends the permit before shipping and handles outcome-publication failure as durable-only narration rather than retroactively aborting coherent execution.

## Serious hypotheses that did not hold

- `30Ro:no-semantic-signature-gap-found` — no path was found that verifies a reconstructed model instead of exact serialized bytes.
- `30Ro:no-rich-to-plain-strip-found` — projection and signature domains prevent textual compartment removal from becoming a valid plain receipt; semantic narrowing remints a new identity/document.
- `30Ro:no-partial-overlay-release-found` — failed rich overlay validation releases no individual opaque slot.
- `30Ro:no-parser-allocation-first-found` — outer, line, field, record, overlay, image, entry, and aggregate budgets generally precede document-directed allocation; checked arithmetic and boundary vectors are extensive.
- `30Ro:no-local-replace-or-cleanup-found` — neither publication nor failure cleanup overwrites/removes an existing name in production.
- `30Ro:no-recorded-to-live-conversion-found` — the reingestion seal, report-only current comparison type, and dependency direction prevent recorded conclusions from satisfying the live plan/apply APIs reviewed.
- `30Ro:no-crypto-role-alias-found` — no Ed25519/X25519 conversion, shared stored root, or in-file algorithm dispatch was found.
