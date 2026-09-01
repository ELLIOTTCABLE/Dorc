# 30Rp — Secure durable receipts hostile review, Sol A

> Scope: independent hostile review of first-parent work
> `7693ac6f785a055133ef887bd44f725ba91a247f..4414af7af92e4a4b83e9ea94b3ff0e05ecec23d7`,
> including D5 and excluding later 30V work. I read only the permitted quarantined builder law and
> `30Ra`/`30Rb`/`30Rd`; I did not read `30Rc*` or `30Re`–`30Rj`.
>
> Method: root constraints and registries; out-of-quarantine `30R`; permitted designs; first-parent
> history/diff; crate law; production call paths; tests; and focused Rust-native checks. Findings
> below survived an explicit attempt to explain them as a deliberate V1 cut, an out-of-scope threat,
> or a type/API property stronger than the implementation actually claims.

## Executive result

**NACK.** The cryptographic format kernel is not the main failure. The arc breaks at product and
temporal integration seams: production mints a “ready” apply session from argv before any standup;
the advertised explicit-file recovery selector never opens its file; the receipt privacy opt-out is
silently ignored on remote apply; verification-only recovery is thrown away; normal plan→apply runs
cannot correlate the plan receipt; and `why` scans/decrypts the whole bounded store before choosing a
root, then emits global graph material and opaque details without a functioning depth distinction.
The types are locally formidable, but several strongest names certify only today’s call order or a
caller’s assertion. The tests frequently pin that local fiction rather than the end-user claim.

## Code defects

### `30Rp:fnd-standup-is-self-certified`

- **Severity:** High
- **Confidence:** High
- **Code:** `spike/crates/cli/src/apply.rs:368-423`, especially
  `thin_session_context`, `ReadyApplyTargetId::mint`, and `ReadyApplyTarget::of`; permit publication
  follows at `apply.rs:425-452`. `spike/crates/receipt/src/dispatch.rs:146-157` makes
  `ReadyApplyTarget::of` public and accepts any `ResolvedApplyContext`. The real transport driver is
  constructed only later at `spike/crates/cli/src/main.rs:2488-2519`.
- **Governing sources:** `30Ra:394-396` requires tunnel/session and resolved target identity;
  `30Ra:796-799` requires standup before intent publication. `30Rb:1039-1048` explicitly permits only
  a genuine standup or a non-authorizing bypass and says: “Faking `ReadyApplyTarget` from the host
  string is forbidden.” `30Rd:1169-1174` requires the production apply route to publish through the
  held genuine session.
- **Claim attacked:** the affine chain proves a required intent was published for the exact resolved
  target/session/context before mutation authority exists.
- **Failure world:** `dorc apply --host alias --plan plan.sh` turns the syntactically valid `HostId`
  string into a `ReadyApplyTarget`, records all five context axes as not established, publishes the
  intent, and spends the permit before OpenSSH has resolved the alias, config, jump path, host key,
  remote user, or any session identity. The transport is an arbitrary `SessionDriver` supplied
  beside that already-minted permit; the type has no relation to whatever that driver will resolve.
- **User consequence:** the receipt confidently records a pre-dispatch “ready session” that never
  existed. It cannot support deputy/target/session mismatch detection, and a future caller can obtain
  the same authority simply by constructing the public target/context values. The permit is affine,
  but its premise is caller-authored.
- **Repair direction:** move target/session minting behind a genuine transport standup capability and
  make the ready-target constructor private to that edge. Publication and dispatch must consume the
  same held session object. Until that exists, the real SSH route must refuse required publication;
  do not preserve the current thin self-certification as a compatibility path.

### `30Rp:fnd-receipt-disable-is-ignored`

- **Severity:** High
- **Confidence:** High
- **Code:** `spike/crates/cli/src/lib.rs:536-539` defines `--no-receipt` as “write no receipt for this
  run”; parsing accepts it at `lib.rs:960-961` with no apply-mode refusal. The remote-apply early return
  at `spike/crates/cli/src/main.rs:181-185` bypasses the only consumer of `args.no_receipt`, which is
  the ordinary engine-options construction at `main.rs:204-214`. `ship_consented_apply` always opens
  the production durable edge and publishes rich intent/outcome at `main.rs:2488-2524`.
- **Governing sources:** `AID-NEEDS.md:140-143` says receipt contents are sensitive; the CLI’s own
  contract makes the opt-out per-invocation and subtractive. `30Ra:794-807` / `30Rd:828-840` require
  rich intent publication for the default apply policy, so an incompatible opt-out must be refused,
  not ignored.
- **Claim attacked:** the user can type the receipt privacy/retention opt-out and it means what it
  says.
- **Failure world:** `dorc apply --no-receipt --host web1 --plan plan.sh` parses successfully and
  still creates a keyset if needed, writes a rich ApplyIntent containing exact executable bytes, and
  attempts an ApplyOutcome.
- **User consequence:** a user who explicitly declined sensitive persistence receives the exact
  persistence they declined, with no warning. This is worse than lacking an opt-out because the flag
  induces a false belief about disk state, backups, and support artifacts.
- **Repair direction:** reject `--no-receipt` on a required-publication apply before filesystem or
  transport work, with a typed incompatibility. If the human later authorizes a true no-receipt
  apply policy, it must be a separately reviewed non-authorizing/bypass design, not this silent
  ignore.

### `30Rp:fnd-explicit-root-file-is-dead`

- **Severity:** High
- **Confidence:** High
- **Code:** `spike/crates/cli/src/main.rs:187-201` calls `read_receipt_store(edge)` without the file
  selector. `read_receipt_store` at `main.rs:474-525` opens and enumerates only the selected/default
  store and never reads `Args::receipt_file`. `spike/crates/cli/src/engine.rs:2139-2160` then makes
  `ReceiptRoot::File(_)` reject every stored document on purpose. No edge admits the named file.
  Tests at `main.rs:3586-3606` and `lib.rs:2005-2044` prove parsing/orthogonality only.
- **Governing sources:** `30R:160-173`; `30Ra:208-229`; `30Rb:1394-1398`. All define
  `--receipt <file>` as an explicit report-only attention root, with `--receipts` merely supplying
  sibling lookup.
- **Claim attacked:** an operator can take one receipt file from an attachment, backup, or another
  location and ask `dorc why --receipt FILE`.
- **Failure world:** with no local store, `dorc why --receipt incident.dorc-receipt` reports the
  standard store/keyset unreadable without opening `incident.dorc-receipt`. With a populated store,
  it reads/decrypts that store, selects no document for the File arm, and may print only unrelated
  global graph lines.
- **User consequence:** the primary degraded-condition/imported-artifact recovery path is wholly
  nonfunctional, precisely when the ordinary local store is missing or untrusted.
- **Repair direction:** admit and bound the explicit file first, verify it under an explicit trust
  state, make it the root, then use the selected store only for typed sibling discovery. Add a
  process-level test that deletes/omits the standard store and proves the external file is actually
  opened and selected.

### `30Rp:fnd-verification-only-recovery-disappears`

- **Severity:** High
- **Confidence:** High
- **Code:** `spike/crates/receipt-local/src/keyset.rs:585-595` intentionally produces
  `VerificationReady` with no opener. But `load_encryption` at `keyset.rs:664-683` collapses every
  missing, permission, malformed, noncanonical, and manifest-mismatch reason into `None`.
  `spike/crates/cli/src/durable.rs:256-315` then refuses all three receipt species as the generic
  `RegionUnopenable` whenever the opener is absent. `main.rs:492-503` turns every such receipt into
  an unread listing.
- **Governing sources:** `30Rd:570-584` requires distinct availability states; `30Rd:618-623` and
  `30Rd:817-825` require local verification material to remain useful without an opener;
  `30Rd:1180-1185` makes authenticated-skeleton recovery an acceptance condition. The readable
  skeleton is a central product property in `30R:39-43` and `30Ra`’s reader state machine.
- **Claim attacked:** losing only the Age identity loses opaque detail, not signed structural
  history, and the user is told the specific recovery state.
- **Failure world:** an operator restores `signing-private-v1.pk8` and the manifest but the Age key
  is temporarily unreadable, permission-broken, malformed, or missing. Keyset opening correctly
  reaches `VerificationReady`; the CLI immediately discards that capability, labels every rich
  receipt unread, and erases the distinction among all causes.
- **User consequence:** the “directly readable signed skeleton” buys nothing on the most important
  partial-key recovery path; receipt IDs, species, counts, policy, omission states, and graph
  structure vanish together with encrypted detail, and diagnostics cannot direct repair.
- **Repair direction:** expose a signature-checked rich-skeleton state independently of overlay
  availability, preserve the exact encryption-role failure, and let report construction consume
  authenticated skeleton plus `DetailState::Unavailable(reason)`. Never promote opaque slots.

### `30Rp:fnd-root-selection-follows-global-exhaustion`

- **Severity:** High
- **Confidence:** High
- **Code:** `spike/crates/cli/src/main.rs:481-525` enumerates, reads, verifies, opens, and builds
  listings for the whole store before root selection. A single global graph budget is spent in
  filename order at `main.rs:490-503`. `spike/crates/cli/src/engine.rs:2180-2210` selects only after
  that work, then appends `reading.graph()` for the entire store unconditionally.
  `spike/crates/cli/src/recorded.rs:214-237` renders every graph edge/finding/partial, disconnected
  or not. Limits are 4,096 entries and 256 MiB at
  `spike/crates/receipt-local/src/limits.rs:30-48`.
- **Governing sources:** `30Ra:208-212` ACKs one attention root and only required typed edges;
  `30R:145-181` says disconnected graphs never contribute; `30Rd:779-785` permits bounded
  enumeration for reverse-edge discovery but still forbids disconnected DAGs joining the answer.
- **Claim attacked:** rooted attention limits both semantic contribution and unrelated corruption/
  size blast radius.
- **Failure world:** several old, disconnected rich receipts consume the 256 MiB graph budget before
  the newest selected root is reached. The selected root becomes `unread` even though its own file
  is valid and within the 64 MiB bound. Even below that threshold, unrelated graph findings and
  partials are appended to every rooted answer. Filename sorting by species, not causal relevance,
  determines which receipts survive the budget.
- **User consequence:** an unrelated old history can make `why --receipt-id X` or `--receipt-last`
  fail or misfocus, and the supposedly rooted report produces unrelated noise. Work scales with the
  entire store and decrypts material the question never needed.
- **Repair direction:** select/admit the root first; perform a bounded metadata/index pass sufficient
  to discover typed candidate edges; then verify/open only the question-directed closure. Account
  unrelated damage as store-level diagnostics outside the rooted explanation, and never let it
  spend the selected closure’s byte budget.

### `30Rp:fnd-default-why-dumps-sensitive-detail`

- **Severity:** High
- **Confidence:** High
- **Code:** `spike/crates/cli/src/recorded.rs:48-99` always appends `opaque_lines`; the latter walks
  every detail slot and terminal-renders it at `recorded.rs:135-155`. All value classes, including
  source text, argv, target, host output, coordinates, encoded apply image, and diagnostic detail,
  take the same terminal path at `recorded.rs:102-133`. The receipt-reading branch at
  `spike/crates/cli/src/main.rs:187-201` never passes `args.all`, so `why` and `why --all` have the
  same detail selection.
- **Governing sources:** `AID-NEEDS.md:95-114` requires a curated pull default and a separately
  labeled exhaustive `--all` tier; `AID-NEEDS.md:140-143` says receipts are sensitive and encoding
  is not declassification. `30Ra:220-224` defines `--all` as deepest explanation depth.
- **Claim attacked:** sensitive opaque detail is retained for recovery but surfaced according to
  the user’s question and depth consent.
- **Failure world:** bare `dorc why` on a rich PlanReceipt prints a capped prefix of every captured
  source/path/shell/record-stream field. Bare `why` on an ApplyIntent prints prefixes from the exact
  executable image. Adding `--all` changes nothing.
- **User consequence:** the first troubleshooting command can spill source/argv/host-shaped bytes
  to a terminal capture, CI log, pager, or redirected support artifact even when none answers the
  question. The 240-byte cap limits volume, not sensitivity.
- **Repair direction:** route the sealed `RecordedWhyFacts` model through goal-derived selection;
  default to the causal structural facts needed by the question; reveal opaque values only in
  explicitly selected links/fields, with `--all` as the actual exhaustive tier. Keep sink encoding
  after selection.

### `30Rp:fnd-directory-handle-is-abandoned`

- **Severity:** High
- **Confidence:** Medium-high
- **Code:** `spike/crates/receipt-local/src/native.rs:67-94` retains opened handles and claims later
  acts are handle-relative, but `Request::EnumerateBounded` at `native.rs:172-182` calls
  `std::fs::read_dir(path)` by pathname. `StorePresence::probe` opens and inspects the store handle,
  then uses that pathname enumeration to authorize first-use key generation at
  `spike/crates/receipt-local/src/keyset.rs:266-292`. Normal store enumeration uses the same path
  call via `spike/crates/receipt-local/src/store.rs:1019-1033`.
- **Governing sources:** `30Rd:303-308` requires authority-bearing Unix child operations relative
  to the retained validated directory handle; `30Rd:468-474` requires proof the existing store is
  absent/empty before generating a new key era; `30Rd:751-758` explicitly warns enumeration is not
  a stable snapshot.
- **Claim attacked:** opening/retaining the store root binds subsequent authority decisions to the
  object that was validated rather than to a replaceable name.
- **Failure world:** process A opens non-empty `receipts-v1` and retains its handle. Before
  enumeration, a sync client or concurrent process renames that directory and installs an empty
  directory at the same pathname. `read_dir(path)` observes the replacement and returns empty;
  `StorePresence` authorizes a fresh keyset while old receipts survive in the renamed original.
  Conversely, normal `why` can enumerate one directory and then read names relative to another
  retained handle.
- **User consequence:** ordinary races can create the unannounced new-key-era state the keyset gate
  exists to prevent, or make store selection depend on a mismatched pair of directory objects.
  The deterministic model cannot stage an external rename between operations, so its exhaustive
  fault schedule greens the wrong abstraction.
- **Repair direction:** enumerate through the retained directory handle (or an ownership-bearing
  directory capability) on Unix, and carry the enumerated entry’s parent identity into open/read.
  If the safe operation is unavailable on a platform, refuse this authority gate rather than fall
  back to path enumeration.

## Design failures and omissions

### `30Rp:fnd-normal-plan-apply-never-correlates`

- **Severity:** High
- **Confidence:** High
- **Code:** production intent preparation hardcodes `PendingOrigins::Unavailable` at
  `spike/crates/cli/src/apply.rs:393-422`. The apply CLI accepts only plan bytes and has no surface
  carrying an originating receipt/presentation identity. The integration test explicitly blesses
  this as normal at `spike/crates/cli/tests/receipt_route.rs:894-900`.
- **Governing sources:** `30Ra:190-201` requires plan+intent+outcome as a reader state;
  `30Ra:1021-1025` says correlate all three recorded species in `dorc why`; `USER_STORY.md:759-763`
  says the receipt covers what was measured, decided, and actually ran.
- **Claim attacked:** the new graph connects Dorc’s ordinary plan→apply workflow into one causal
  answer.
- **Failure world:** run `dorc plan ... >plan.sh`, inspect it, then `dorc apply --plan plan.sh`.
  The plan receipt is written, but the intent records no origin even when the bytes are exactly the
  emitted plan. The outcome reaches only its intent; it cannot reach the plan’s probes, vouches,
  decisions, locators, or source custody.
- **User consequence:** `why --receipt-last` after a successful apply cannot answer the flagship
  “why did this line run/elide?” question from the plan receipt without the user separately finding
  and selecting that disconnected document. The hundreds-of-types graph is absent on the primary
  product path while tests normalize the absence.
- **Repair direction:** design an explicit, non-inferential plan-origin handoff. It must bind exact
  final plan bytes and the plan receipt/presentation identity while preserving admin edits and
  M:N composition; a hidden filename guess or automatic same-bytes inference is not enough.

### `30Rp:fnd-immutable-store-has-hard-expiry`

- **Severity:** High
- **Confidence:** High
- **Code:** every default plan publishes one receipt; successful apply publishes intent and outcome.
  Publication has no entry-count admission at `spike/crates/receipt-local/src/store.rs:926-1002`,
  while every read first enumerates and refuses over 4,096 entries at `store.rs:1019-1033` and
  `spike/crates/receipt-local/src/limits.rs:39-48`. The native test
  `receipt-local/tests/native_store.rs:244-277` positively pins the hard-refusal behavior at a
  lowered bound. No retention or cleanup exists.
- **Governing sources:** `USER_STORY.md:759-763` promises a small boring receipt, no log ocean, and
  “Ask tomorrow; ask next week.” `30Rd:1253-1258` acknowledges no retention and says growth can
  eventually block required publication, but the implementation actually lets publication continue
  while all store-based reading reaches the enumeration cliff.
- **Claim attacked:** default-on immutable durability remains a zero-setup recovery aid over the
  lifetime of an ordinary user profile.
- **Failure world:** about 1,366 normal plan+apply cycles create more than 4,096 files. The next
  `dorc why`, `--receipt-last`, and `--receipt-id` all refuse before selection. Unknown or sync
  conflict files accelerate the cliff. Applies can continue adding files because publication never
  checks the walk bound. The only selector meant to avoid the store (`--receipt FILE`) is dead under
  `30Rp:fnd-explicit-root-file-is-dead`.
- **User consequence:** the default audit feature eventually disables its own recovery interface
  and keeps consuming disk. There is no Dorc cleanup, retention, direct-ID lookup, or documented
  safe lifecycle in the shipped surface.
- **Repair direction:** do not call this product-complete without a bounded lifecycle. Add an
  explicit, reviewable retention/archival policy or a sharded/indexed immutable layout with bounded
  per-query discovery; make direct ID/file reads independent of full enumeration; surface capacity
  before apply is blocked. Never silently delete history.

### `30Rp:fnd-mandatory-durable-breaks-shell-floor`

- **Severity:** Medium-high
- **Confidence:** High
- **Code:** `spike/crates/cli/src/main.rs:2488-2519` refuses remote apply when roots, keyset,
  sealing, signing, store creation, write, or sync cannot satisfy required publication. No bypass
  exists; the apparent per-invocation opt-out is ignored on this route.
- **Governing sources:** this is an acknowledged design choice in `30Ra:794-807` and
  `30Rd:828-840`, not an accidental implementation departure. The unpriced conflict is with the
  human root posture: `DESIGN.md:75-79` promises a trivial immediate shell off-ramp, and the Stage-0
  floor in `USER_STORY.md` is “no worse than what they already did.”
- **Claim attacked:** the receipt family is an explanation subsystem rather than a new local
  availability prerequisite for doing the user’s actual work.
- **Failure world:** the target and SSH path are healthy, but the operator’s home is read-only,
  quota/full, keyset incomplete after a prior crash, XDG roots unavailable, directory sync refused,
  or the immutable store is operationally wedged. Plain `ssh host 'sh -s' <plan.sh` works; `dorc
  apply` refuses before transport solely because its debugging artifact cannot be durably written.
- **User consequence:** an incident-response orchestrator can block remediation for controller-local
  audit housekeeping. Users will learn the raw-shell off-ramp precisely when they need Dorc most,
  or will seek unsafe ways to disable the gate. This is a product-level reliability cost of the
  ACKed security posture, not a bug that types can erase.
- **Repair direction:** human re-adjudication is required. Either narrow the “no worse/raw floor”
  promise and accept Dorc apply as audit-dependent, or design an explicit, loudly consented raw
  apply posture that does not pretend to have published intent. Do not silently weaken required
  publication inside the existing type chain.

## Unresolved questions

### `30Rp:ask-encoder-boundary-enforces-nothing`

`RecordedValue` has no raw accessor, but `ValueEncoder` is a public unsealed trait receiving the raw
bytes (`receipt/src/report/value.rs:150-157`), and `agrees_with` is a public chosen-input equality
oracle (`value.rs:174-188`). This does not violate the current same-user threat model by itself, and
the sole production encoder found in this review does call `aid::display::encode_foreign`. It does,
however, mean the “only encoded exit” is a review convention with ceremony, not a type-enforced
security property against a future caller. Decide whether the boundary promises enforcement or only
an auditable seat; document and test the weaker true claim if the latter.

### `30Rp:ask-current-world-comparison-remains-unwired`

The sealed `RecordedWhyFacts` path exists, but the production listing remains the old direct receipt
listing and `RecordedWhyFacts` is not joined to user output. Re-derivation is explicitly deferred,
which is allowed, but current-source comparison and rooted selection are already required recorded
facts. The current route’s global listing makes it unclear whether later arrangement can be added
without replacing, rather than wrapping, the live behavior. Treat this as an integration redesign,
not cosmetic 30V pressure.

## Attacks that did not hold

- **`30Rp:held-literal-signature-bypass-failed`** — I found no parse/normalize/reserialize split in
  the checked path that would make signature verification cover different bytes from semantic
  parsing. The exact-body and strict grammar tests are substantial.
- **`30Rp:held-rich-to-plain-downgrade-failed`** — plain/rich species are genuinely distinct at the
  writer/parser, and textual stripping does not mint a valid plain receipt.
- **`30Rp:held-partial-receipt-authority-failed`** — partial and reingested values do not appear to
  convert into live plan/license/apply authority. The deletion of replay authority is real.
- **`30Rp:held-key-role-alias-failed`** — signing and Age identities are independently generated,
  separately encoded, and non-convertible in the reviewed production route.
- **`30Rp:held-final-component-link-attack-failed`** — on Unix, final key/store members use retained
  handles, `openat`/`O_NOFOLLOW`, restrictive exclusive creation, and handle-based inspection/read.
  The surviving race is directory enumeration abandoning the retained handle, not the final-member
  attack the tests cover.
- **`30Rp:held-durable-failure-aborts-after-dispatch-failed`** — the current apply route does spend
  the permit before shipment and treats later outcome-publication failure as durable-only
  narration. I found no generic catch that obviously swallows transport/execution failure there.
- **`30Rp:held-custom-crypto-primitives-failed`** — the project owns a large amount of framing and
  glue, but it does not reimplement Ed25519 or Age stream cryptography. I found no concrete primitive
  misuse strong enough to report.

## Focused check record

- `mise trust` completed for the isolated worktree.
- `mise run test -- the_three_root_selectors_refuse_one_another` — green; importantly, this proves
  only parser mutual exclusion, not that `--receipt FILE` is opened.
- `mise run test -- a_walk_past_a_narrowed_entry_bound_refuses_on_a_real_directory` — green; this
  positively demonstrates the store-wide hard refusal at `bound + 1`.
- No full workspace acceptance, `gate*`, `both`, Lean, Kani, blessing, fixture hand-execution, or
  real mutator was run.

## Blunt verdict

This is a sophisticated failure in the precise sense posed by the review: strong local types and a
large aligned test corpus verify the architecture’s internal story while the normal user journeys
fail outside that story. The crypto/format core may be salvageable as an experiment, but the arc as
an integrated replacement for the small durable whylog is not acceptable at this tip. Do not close
or market the secure durable transition until the apply session premise is genuine, explicit-file
recovery works, receipt opt-out is honest, verification-only recovery survives, normal plan/apply
correlates, and rooted `why` no longer depends on or emits the whole store.
