# Secure durable praxis: critical synthesis

> Tier: quarantined research synthesis. This document analyzes the eleven
> `secure-durable-praxis` reports against Dorc's authoritative goals and the
> partially settled direction in `Research/quarantine-DO-NOT-READ/30Ra`.
> It is not a final product design, a format selection, or implementation
> authorization.
>
> Evidence posture: source grades describe the authority of a source for what it
> says, not the correctness or transferability of a researcher's conclusion.
> Later reports (`turn07` through `turn11`) had little or no contemporaneous human
> review and receive no implied assent from that silence.
>
> Confidence vocabulary: **+SURE** is reserved for conclusions triangulated across
> unlike fields or fixed directly by project rulings; **~SUSPECT** is the normal
> resting point for a strong but context-sensitive synthesis; **-GUESS** marks a
> plausible recommendation needing design work; **--WONDER** marks an open lead.

## syn-round-result-in-brief

**+SURE** The round produced more durable value in principles and failure shapes
than in package selection. Its strongest findings concern authority separation,
bounded parsing, completeness, cryptographic context binding, identity separation,
and the non-transferability of at-rest recovery semantics to a live channel. Those
findings recur across standards, academic attacks, shipped systems, and Dorc's own
threat posture.

**+SURE** The round did not settle a coherent final construction. The late proposed
stack (`cborrs` plus a project-written C2SP `chunked-encryption` implementation plus
`std`) conflicts with the human-acked readable-envelope direction, leaves the
structural-skeleton binding underspecified, and combines mutually inconsistent
statements about encryption granularity and salt use. Its source experiments are
valuable candidate measurements, not a design decision.

**~SUSPECT** `30Ra` remains directionally sound on its central boundaries:

- re-ingested material never authorizes action;
- recorded and re-derived conclusions remain visibly distinct;
- arbitrary sensitivity is classified by data shape, not guessed content;
- rich and plain are separate projections, not a mode bit on one writer;
- collection does not expand merely because persistence is enabled;
- unaccounted stdout/stderr stays remote by default;
- source reconstruction is preferable to silently copying source by value; and
- the store is a security-relevant cross-platform subsystem, not `fs::write` plus a
  permissions comment.

**~SUSPECT** `30Ra` is least settled where it becomes concrete: one readable file,
per-field versus grouped encryption, encryption-unavailable behavior, one file per
invocation, explicit-only retention, exact publication semantics, and the rule that
every stable Spine species should become durable. These are precisely the places the
reports expose as product choices rather than deductions.

## syn-weighted-decision-register

The order below weights retrofit cost, implementation difficulty, likelihood of an
ordinary good engineer choosing the wrong default, and conflict with Dorc's product
identity.

| rank | finding | confidence | lock-in | present disposition |
|---:|---|---|---|---|
| 1 | Re-ingested evidence must remain structurally unable to mint authority | +SURE | absolute | preserve as law |
| 2 | Rich/plain projection and hostile-input boundaries must be type-visible, not review conventions | +SURE | high | preserve; exact grammar open |
| 3 | The document needs bounded independent units, explicit completion, scope, and damage states before a format is chosen | ~SUSPECT | high | decide shape first; family open |
| 4 | The proposed encryption/container integration is not yet a coherent construction | +SURE | high | reject the late recommendation as a decision |
| 5 | Storage guarantees must be stated per platform and exceed what `std` alone currently establishes | +SURE | high | preserve `30Ra`'s stronger contract; mechanism open |
| 6 | The pre-mutation durable boundary is real, but its Dorc topology and failure policy remain unresolved | ~SUSPECT | high | human design sitting required |
| 7 | Direct human readability versus tool-mediated binary inspection remains an open product fork | +SURE | high social, medium technical | do not accept `turn10`'s closure |
| 8 | At-rest and transport may share primitives and data vocabulary, but not recovery policy, identities, keys, or error behavior | +SURE | high | reserve separate typed constructions |
| 9 | Persisted content should be selected by explanatory necessity, not automatically by Spine membership | ~SUSPECT | high schema/sensitivity | re-review `30Ra`'s every-species direction |
| 10 | Retention needs an honest, bounded product policy; neither keep-64 nor explicit-only is established | ~SUSPECT | medium | reopen; reject the late universal claims |
| 11 | Key lifecycle and encryption-unavailable behavior can make the aid product disappear exactly when needed | ~SUSPECT | medium-high | open product decision |
| 12 | Concrete crate, format, proof-tool, and algorithm selections are dated candidates only | +SURE | medium-high | remeasure at design/implementation time |

## syn-load-bearing-principles

### syn-replay-never-restores-authority

**+SURE** A whylog is evidence about a past decision, never a serialized capability.
This survives every source vector examined:

- Dorc's human-ruled `306b:rul-reingestion-drives-no-action` separates temporal
  staleness from host influence.
- The CRIU restore vulnerability shows a hostile checkpoint becoming live privilege
  when recorded credentials are reconstituted as authority [B-criu-checkpoint-credential-spoof-2026].
- Kubernetes separates desired `spec` from observed `status`, including separate
  update authority [A-k8s-spec-status-conditions-2026].
- The transaction literature distinguishes a record of intent from the real action
  and requires separate recovery properties before replay is safe
  [A-gray-transaction-real-actions-1981].

**+SURE** This is stronger than "do not call apply from `dorc why`." Recorded
types need no conversion, deserialization, generic trait, or shared constructor path
to live claims, vouches, influence accounts, plan authority, probe requests, artifacts,
or mutation. A report renderer may use recorded data liberally; an operation endpoint
may not accept it.

**+SURE** Influence rehydration does not replace this boundary. Even material recorded
as authored-before-contact describes an old invocation and old world moment. The two
rules protect different dimensions and must remain separate.

### syn-sensitivity-follows-data-shape

**+SURE** Dorc cannot promise secret detection, generic scrubbing, or "safe to share."
This follows from Dorc's referential agnosticism and is reinforced by unrelated bodies
of evidence: command-line values escape through history and process inspection
[B-clig-command-line-guidelines-2026]; hashes over low-entropy identifiers are cheaply
reversible [A-hagen-contact-discovery-hash-reversal-2022]
[A-demir-pitfalls-hashing-privacy-2018]; and local application encryption leaves copies
in surrounding logs, caches, memory, and source [A-grubbs-encrypted-db-not-secure-2017].

**+SURE** The useful classifier is therefore what a field can represent, not what one
observed value appears to contain. Arbitrary authored text, host bytes, paths, argv,
entity values, source excerpts, report tails, and tool errors are opaque-value-capable.
Encoding for a sink does not declassify them; redaction does not make them structurally
safe; encryption narrows exposure but does not make them harmless.

**~SUSPECT** The plain projection should carry no public digest, truncated digest, or
salt-stored-beside-digest derived from an omitted opaque value. The negative result is
well supported: a public salt permits one guess-confirmation hash per candidate, while
withholding the salt requires a second protected channel the plain projection does not
have [A-rfc9901-sd-jwt-selective-disclosure-2026]
[A-enisa-pseudonymisation-techniques-2019]. Presence, counts, and lengths are weaker
leaks, but they are still values and should be included only when the plain product
actually needs them [A-nikitin-purbs-padme-2019]
[A-alexeev-percival-chunking-attacks-2025].

**~SUSPECT** The reports over-transfer the magnitude of length leakage from genomic,
music, package, and generic file corpora to Dorc receipts. They establish that length
is not categorically safe; they do not establish that a whylog record length identifies
a host fact or secret at a practically relevant rate.

### syn-format-shape-precedes-format-family

**~SUSPECT** The most defensible format result is a shape, not "CBOR wins":

- a flat top-level sequence of independently bounded, self-delimiting units;
- a closed type discriminator per unit;
- definite lengths and checked arithmetic before allocation;
- a canonical gap-free ordinal checked against the reader's count;
- a mandatory terminator, cryptographically bound to the sequence in rich mode;
- explicit document, projection, schema, target, attempt, generation, and record scope;
- bounded unknown material retained only as report-only raw bytes;
- separate states for complete, valid-prefix-plus-damage, unknown, omitted, and unavailable;
- no resynchronization that silently restores completeness after damage; and
- no map or generic `Any` surface unless a later requirement earns its parser complexity.

**~SUSPECT** This shape is supported from different seats. CBOR Sequences supplies
self-delimiting concatenation while explicitly disclaiming detection of wholly missing
tail items [A-rfc8742-cbor-sequences-2020]. LangSec work independently favors flat
independent records and a deliberately restrictive grammar
[A-bratus-curing-vulnerable-parser-2017]. AWS's message format stores and checks frame
sequence numbers [A-aws-esdk-message-format-2026]. Age and C2SP constructions require
authenticated termination [A-c2sp-age-format-spec-2026]
[A-c2sp-chunked-encryption-2026]. Parser vulnerabilities show why byte, count, depth,
and allocation bounds are separate obligations
[A-dagcbor-preallocation-amplification-2026]
[A-protobuf-unknown-field-recursion-dos-2024].

**+SURE** A container alone does not close tail truncation or make a damage model real.
The format needs an application completion rule, the reader must check it, and rich mode
must cryptographically bind it. CBOR
Sequences explicitly has no end marker [A-rfc8742-cbor-sequences-2020]; Avro's ignored
sync marker demonstrates that a format feature no reader enforces is no property at all
[A-arrow-avro-unchecked-sync-marker-2026].

**+SURE** The format family remains open. Deterministic CBOR has unusually strong
strict-recognizer and proof work [A-ramananandro-evercbor-evercddl-2025], while text
sequences preserve direct inspection, quoting, and recovery with ordinary tools
[A-rfc7464-json-text-sequences-2015]. Those are different product properties. Neither
one follows from the unit shape above.

### syn-projections-need-two-mechanical-properties

**+SURE** "Plain cannot carry rich material" is at least two properties:

1. no encoded unit or document is accepted as both rich and plain; and
2. the in-memory plain type cannot represent an opaque-value-capable field.

**+SURE** A schema disjointness proof supplies only the first. Exhaustive type structure
or equivalent compile-time checks supply the second. `turn11` demonstrated the gap by
adding an optional ciphertext field to a plain record that remained disjoint by its
leading discriminator. This is a valuable general result even if EverCDDL is never
adopted.

**~SUSPECT** A third property is missing from the late reports: the document must not
mix projection modes unit by unit. Proving `"dec"` and `"dec+"` units disjoint does not
prove that a whole sequence is uniformly plain or rich, nor that an unknown-unit retention
path cannot carry an encrypted rich unit through the plain reader. The mode must be fixed
at the document boundary and enforced throughout the sequence.

### syn-cryptographic-context-needs-one-owner

**+SURE** Encryption must bind ciphertext to its exact semantic context. Per-field or
per-record encryption without document, projection, schema, record species, field,
ordinal, and scope binding permits cut-and-paste or cross-protocol reinterpretation.
This is established by the AEAD associated-data definition's injective-encoding
requirement [A-rogaway-associated-data-problem-2001], practical structured-encryption
failures [B-arciszewski-encryption-at-rest-threat-model-2024], and protocol-confusion
attacks against Matrix and Threema [A-albrecht-matrix-vulnerabilities-2023]
[A-paterson-threema-lessons-2023].

**+SURE** Context encoding must be injective and domain-separated. Concatenating
variable-length fields without lengths is not sufficient. This is security glue, not
ordinary serialization glue.

**~SUSPECT** Key commitment is strongly desirable if Dorc trial-decrypts immutable
receipts against multiple retained keys. Standard AEAD does not guarantee that a
ciphertext has one valid key/plaintext interpretation, and key rotation is one cited
application for commitment [A-albertini-key-commitment-2020]
[A-rfc9771-aead-properties-2025]. The conclusion should remain conditional on the
eventual key-selection design; it is not a universal reason every encrypted field must
carry a separate commitment.

**~SUSPECT** Compression should remain absent from the whylog unless a later need earns
a fresh threat analysis. Mixing attacker-influenced bytes and opaque values in one
compression context has a well-established length side channel
[A-kelsey-compression-leakage-2002]
[A-paterson-threema-lessons-2023], and decompression introduces independent resource
amplification. The reports do not establish that Dorc's remote-host adversary always has
the adaptive controller-file-length oracle needed for practical recovery, so "the attack
always applies" is too strong. The no-compression default remains cheap and prudent.

### syn-recommended-encryption-is-not-coherent

**+SURE** The late C2SP recommendation does not specify one coherent integration. It
simultaneously claims:

- one injected 24-byte salt draw per document (`turn10`);
- context binding that includes record identity and ordinal (`turn10`);
- a flat sequence of structural units with encrypted opaque fields (`turn03`/`turn10`);
- C2SP `chunked-encryption` as the encryption layer; and
- one authored cryptographic seam.

**+SURE** C2SP defines one salt, commitment, key, base nonce, and ciphertext stream per
message; the salt must not repeat for an input key [A-c2sp-chunked-encryption-2026].
If each record or field is a message, one salt per document is insufficient absent an
additional per-record key derivation the reports never specify. If the complete opaque
value set is one message, the design needs a separate index/blob layout and a binding of
the entire structural skeleton to that blob. That is the split-artifact seam from
`turn01`, not the per-record design `turn10` appears to describe.

**+SURE** The reports also never close whole-document structural integrity. Authenticating
each opaque blob in local context does not by itself detect insertion, removal, or
reordering of plaintext structural units. A whole-skeleton authenticator, a final
authenticated manifest, or a construction that makes every unit relationship part of the
cryptographic context would be a design choice. None is supplied by merely selecting CBOR
plus C2SP.

**+SURE** Therefore the "one authored cryptographic seam" count is not established. The
schema-to-crypto binding, document completion, skeleton integrity, projection mode,
record ordering, key selection, and key lifecycle are still seams owned by Dorc.

**~SUSPECT** C2SP remains a high-quality candidate specification. It independently
matches many correctly derived requirements: derived nonces, context in KDF info,
commitment before decrypt, fixed chunks, short final chunk, explicit bounds, and negative
vectors. Its current editor-copy status, one listed Go implementation, and absence of a
maintained Rust implementation mean it is not yet a package recommendation.

**~SUSPECT** `turn10` selects Cobblestone-256 without establishing a Dorc requirement for
it, while the specification recommends Cobblestone-128 absent another requirement
[A-c2sp-chunked-encryption-2026]. The algorithm variant is part of the later selection,
not a finding of this round.

### syn-readable-envelope-fork-remains-open

**+SURE** `turn10` does not merely choose a format; it changes a human-acked product
requirement. `30Ra:rul-one-readable-envelope` says the file remains somewhat readable by
eye and rejects whole-file encryption for that reason. A deterministic CBOR sequence with
tool-rendered EDN is not readable by eye. A magic prefix visible in a hexdump identifies
the file type; it does not expose the receipt's structural account
[A-rfc9277-cbor-stable-storage-2022].

**+SURE** Tool-mediated rendering has real costs for this product:

- the firefighter may be using an old or damaged Dorc binary;
- the file may be inspected on a machine without Dorc;
- a vendor receiving the artifact cannot inspect it with ordinary text tools;
- the renderer becomes another trusted interpretation step; and
- Dorc's plain projection cannot reassure someone who cannot execute Dorc.

**~SUSPECT** The binary case is still substantial. A closed structural alphabet plus
encrypted opaque values makes a renderer less risky than rendering attacker text, and a
strict deterministic decoder can be simpler and less ambiguous than a permissive text
parser. That is evidence for keeping the fork open, not for silently moving the product
requirement.

**+SURE** A later design must explicitly choose between direct file readability and
tool-mediated inspectability. The late research cannot make that value decision on the
human's behalf.

### syn-storage-contract-needs-platform-truth

**+SURE** The store is not a generic file-writing helper. The reports support these
distinct obligations across filesystem research, platform APIs, and mature storage
systems:

- trusted per-user siting;
- ownership-bearing directory and object handles where the platform exposes them;
- restrictive creation at the creation operation, not a later chmod;
- no replacement of an existing final object;
- bounded complete enumeration or explicit overflow;
- synchronization with documented per-platform guarantees;
- an unambiguous committed-versus-partial state;
- no pathname-based deletion of an object whose ownership was never established;
- visible failures; and
- injected filesystem, clock, and randomness edges for DST.

**+SURE** Cross-platform guarantees are not identical. Unix/macOS and Windows differ on
directory synchronization, path walking, rename/replace, open-handle deletion, ACLs, and
safe-Rust reachability [A-posix-readdir-spec-issue8-2024]
[A-ms-file-disposition-posix-semantics-2024]
[A-apple-fsync-fullfsync-manpage-2014]. A truthful abstraction must expose the common
contract and the weaker platform-specific guarantees rather than pretending POSIX modes
or Windows inherited ACLs are the architecture.

**+SURE** `turn10`'s recommendation to use `std` alone regresses the requested contract.
Creating directly under the final name makes the incomplete file visible before its bytes
are complete and synchronized. Path-based follow-then-check operations do not retain the
resolved directory's identity through later create, enumerate, and delete operations. A
caller-side metadata check cannot close a TOCTOU race
[A-rust-remove-dir-all-toctou-advisory-2022].

**~SUSPECT** Symlink usability and race resistance need a narrower question than either
"reject every symlink" or "follow and use pathnames." The user-facing state-root may need
to be symlinkable for dotfile/sync layouts, while entries inside the resolved owned store
need not inherit that freedom. Resolve-and-validate can be a setup step; retaining the
result as an ownership-bearing handle is a separate runtime property. The reports collapse
those two layers.

**+SURE** Safe Rust currently leaves genuine Windows gaps. That is a design constraint,
not evidence that the property no longer matters. Plausible later outcomes include a
maintained safe dependency with reviewed `unsafe`, an explicitly weaker Windows guarantee,
or a narrower store contract. Selecting among them belongs to the design phase.

### syn-write-boundary-needs-product-ruling

**~SUSPECT** There is strong cross-field support for durably recording an intended
authority spend before initiating an irreversible effect. Write-ahead intentions are the
canonical transaction pattern [A-lampson-sturgis-intentions-1979]; Kubernetes can fail a
request specifically when its pre-delegation audit stage cannot be recorded
[A-k8s-audit-stages-blocking-strict-2025]; and sudo can require its command record before
execution [A-sudoers-log-failure-flags-2026]. A completion-only record is lost in exactly
the crash where it is most wanted.

**+SURE** The guarantee is narrow: no effect occurred without a durable record of the
intent. It does not prove the effect occurred. Arbitrary remote shell commands are Gray's
"real actions": not generally undoable, restartable, named for deduplication, or remotely
transactional [A-gray-transaction-real-actions-1981]. A timeout still leaves success
unknown.

**+SURE** `turn07` overstates the implication as one decision document plus one outcome
document per invocation. Dorc already has a plan/apply split, a human-approved executable
artifact, and separate invocation identities. A saved plan artifact or the plan invocation's
receipt may already be the pre-effect intention; the apply invocation may need only to bind
it exactly before network contact. Conversely, a streamed or regenerated apply may need a
new pre-effect record. The answer follows from Dorc's final command semantics, not from the
transaction papers alone.

**~SUSPECT** The failure policy is a direct conflict with Dorc's product principles. Failing
before mutation when the intention record cannot be durably published preserves the why
promise and auditability. It also lets a full or broken local state directory prevent an
admin from fixing a remote outage, violating the "no worse than running sh" floor and
placing record-keeping in the critical path. Shipped systems occupy both poles, sometimes
within one product [A-sudoers-log-failure-flags-2026]
[A-auditd-conf-failure-actions-2025]. Prior art does not choose Dorc's policy.

**~SUSPECT** Post-effect publication failure cannot retroactively refuse the mutation. It
must produce a loud, non-authoritative incomplete-outcome report. The suggestion to render
only a structurally plain projection to stderr is promising because it avoids dumping opaque
values into CI logs, but it needs to be reconciled with Dorc's deliberately narrow apply
console and error-loom policy.

### syn-retention-policy-remains-unsettled

**~SUSPECT** The current automatic count pruning is not defensible as silent incidental work.
Deleting an unreconstructible receipt while the user performs an unrelated plan/apply can
erase the only account of a bad run, and ignored deletion failures make the retention promise
false. `30Ra` is right to reject invisible keep-64 behavior as a resting point.

**+SURE** `turn08`'s universal reconstructibility rule is false within its own evidence.
It claims automatic cleaners never delete unreconstructible primary records, while also
recording that journald automatically removes old archived journal files
[A-journald-damage-rename-archival-2026]. Archived does not mean reconstructible. The
evidence supports a weaker statement: mature tools classify what they delete, expose
policy, preserve in-flight/unclean artifacts specially, and accept availability costs.

**~SUSPECT** The Restic decoy-snapshot attack does not directly establish that count-based
whylog retention is host-steerable. A managed host cannot ordinarily create controller
invocations or files by itself; one controller invocation may cover many targets; and
controller-owned scope must prevent host bytes from minting document identity. The analogy
becomes relevant only if host-influenced events can cause extra durable documents or
re-decisions. Duration-shaped retention may still be better UX, but the security claim is
not transferred.

**~SUSPECT** Explicit-only cleanup also fights Dorc's laziness and attention goals. A
default-on durable with no automatic bound can fill the disk; if pre-effect persistence is
mandatory, that eventually takes the orchestrator offline. The correct resting point may be
explicit cleanup, an honestly advertised automatic policy, a mode, or a bounded warning plus
refusal. The reports do not settle it.

**-GUESS** Date sharding, an append-only deletion ledger, longer retention for orphan
decisions, and a two-document pairing protocol are plausible mechanisms only after the
retention and write-boundary policies are chosen. They are not principle-level findings.
The deletion ledger in particular recursively creates another unreconstructible durable,
records traces of material the user intended to remove, and needs its own retention story.

**+SURE** Whatever policy is selected must use honest verbs. Files can be removed from
this store; Dorc cannot promise obliteration from copy-on-write media, flash translation,
backups, sync histories, indexers, or copies given to another party
[A-nist-cryptographic-erase-preconditions-2025]. No `secure-delete`, `shred`, or generic
erasure claim is available.

### syn-transport-reuse-needs-red-lines

**+SURE** The at-rest and live-transport surfaces should not share one behaviorally
parameterized reader or cryptographic context merely because their record grammar looks
similar. This is a high-lock-in finding that pushes against ordinary DRY instincts.

**+SURE** The strongest red lines are supported across formal channel work, deployed
protocol attacks, and mature specifications:

- an at-rest reader may return a valid prefix plus damage; a transport reader must stop on
  first authentication failure, discard partial message state, and never resynchronize into
  authority [A-fischlin-stream-based-channels-2017]
  [A-baumer-terrapin-prefix-truncation-2024];
- detailed at-rest key diagnostics must not become distinguishable peer responses;
- file keys, salts, document tags, and retained old keys must not become transport key
  material;
- each transport direction needs distinct key/counter state;
- a plaintext projection must not become a negotiated weak wire mode
  [A-poddebniak-starttls-analysis-2021];
- transport identity should be separately minted for its scope rather than reusing the
  public, re-derivable decision digest [A-ietf-idempotency-key-header-2025]; and
- retention policy must not become host-controlled protocol input.

**~SUSPECT** The correct reuse boundary is likely one maintained primitive library and
shared semantic record vocabulary, with separately named at-rest and transport
constructions, keys, conformance vectors, and failure types. Sharing the grammar does not
license sharing recovery policy.

**~SUSPECT** `turn09`'s absolute "no compression at any layer" and detailed wire-identity
lifetime recommendations exceed this round's scope and assumptions. Preserve the red-line
principles, not every proposed transport parameter.

### syn-durable-content-needs-purpose-test

**~SUSPECT** `30Ra` correctly rejects the old posture of storing raw admitted records while
throwing away the structured conclusions needed to interpret them. For Dorc, the highest-value
durable content is likely the typed account around a risky non-action: survival witnesses,
vouches, observations, refusal/admission, region universals, render decisions, source locators,
influence, and actual apply outcomes. This is especially important for `kHALVES-elide-half`:
the whole reason for a whylog is to explain why Dorc did not run authored sh.

**~SUSPECT** "Every minted stable Spine species gets a durable view" is nevertheless too
broad as a content criterion. Stability in the in-memory architecture does not establish:

- that the species is needed for firefighter explanation;
- that its semantics are stable enough to become a file-format commitment;
- that its sensitivity and metadata leakage are justified;
- that a future reader can interpret it without the old engine; or
- that persisting it improves more than it expands parser, encryption, retention, and sharing
  surfaces.

**~SUSPECT** A better later test is purpose-shaped: persist a stable semantic projection when
it is necessary to explain a decision after source drift, to distinguish recorded from
re-derived truth, or to account for an omission. Record projection loss explicitly, but do
not let the omission ledger become a reason to preserve every internal detail.

**+SURE** Working lattice state and process-local handles are not durable forms. Stable
semantic views may be. A handle's meaning must be resolved before projection; serializing an
arena id or live license is not a shortcut.

### syn-source-and-output-containment-stands

**~SUSPECT** Reconstruction-first source handling remains the safer base posture. Books and
oracles can hard-code credentials; copying source into a hidden long-lived store creates a new
sync, backup, and bug-report location. Persist ordered paths, identities, stable loci, and
enough historical conclusions that `dorc why` remains useful when source is absent or drifted.

**+SURE** A durable-supplied path is untrusted input, not a capability to read any controller
file. Source selection must be bounded and confined by controller policy before identity
comparison. The current source location is a candidate reconstruction input, never the historical
source merely because its path matches.

**~SUSPECT** By-value source remains a legitimate explicit future mode or dislocated content
store, but it needs separate enablement, visibility, cleanup, and sensitivity policy. It should
not arise as an incidental side effect of a whylog becoming richer.

**~SUSPECT** Unaccounted stdout/stderr should remain remote by default. This is a strong
containment boundary and avoids turning Dorc into a fleet-wide log collector. Contracted bytes
already transported are different: rich mode may retain their bounded exact form; plain mode
should retain only the structural account. Later debug pull answers about a later world and must
say so.

## syn-critical-review-of-late-reports

### syn-binary-selection-changes-product

**+SURE** `turn10:fnd-container-fork-closed` should be rejected as a conclusion. It
optimizes parser assurance by changing direct readability into renderer-mediated readability,
contrary to `30Ra:rul-one-readable-envelope`. The report identifies the conflict but resolves it
without human authorization.

### syn-crypto-selection-violates-posture

**+SURE** Recommending project-authored C2SP implementation code cuts directly against the
round's human posture: maintained external security code and seams are preferred precisely
because this project's alternative is machine-written security code. "About 250 lines" is not
a security property. A byte-level specification and negative vectors reduce risk; they do not
turn a new cryptographic implementation into ordinary glue.

**+SURE** The fixed 2019-audited RustCrypto defect does not reverse that comparison
[A-rustsec-aesgcm-plaintext-on-tag-failure-2023]. It establishes that dependencies still require
care and updates; it does not establish that a new local implementation is safer.

### syn-verified-parser-remains-research-dependency

**~SUSPECT** EverCBOR/EverCDDL is valuable evidence and a serious candidate, not a settled
dependency. Positive vectors include a machine-checked strict fragment, safe Rust extraction,
zero runtime dependencies, and direct experiments from `turn10`/`turn11`. Negative vectors
include a research-project bus factor, no crates.io release for the selected artifact, a large
toolchain unavailable natively on Windows, generated code that is hard to review, and numerous
proof-extraction panic sites.

**+SURE** A bounded Kani harness cannot recreate an unbounded proof. It can add local agreement
for chosen sizes and can also pass vacuously if over-constrained
[A-kani-verification-result-statuses-2026]. It is not a substitute for the upstream proof or for
reviewing the handwritten boundary.

**~SUSPECT** Using EverCDDL only as a checker over a separately handwritten reader proves a model,
not the implementation. That can still be useful if model/code drift is mechanically tested, but
the report correctly identifies it as judgment rather than result.

### syn-write-split-is-not-deduced

**+SURE** The transaction literature supports pre-effect intent and post-effect observation as
different facts. It does not require two files for every Dorc invocation. The report imports a
topology from other systems before settling whether a plan artifact, plan whylog, apply whylog,
or one invocation's preflight record already occupies each temporal role.

**+SURE** `turn07` also claims shipped splits are temporal rather than taxonomic, then relies on
sudo's different failure policy for different record classes. The defensible principle is that a
split must correspond to an independently meaningful atomicity, authority, or failure boundary.
"Never taxonomic" is too broad.

### syn-retention-analogy-does-not-transfer

**+SURE** The explicit-only retention conclusion rests partly on a false universal and partly on
an attacker model that may not mint whylog count. The practical conflict with disk exhaustion and
mandatory prewrite is not resolved. Treat the report as a map of failure modes, not a policy.

### syn-symlink-lean-was-overgeneralized

**+SURE** A desire to support symlinked state roots does not imply pathname-based operations inside
the resolved store. The late report uses the usability lean to delete the capability/handle problem,
but root resolution and later ownership-preserving operations are separate concerns.

### syn-identity-findings-need-separation

**~SUSPECT** The three-way identity split is strong:

- decision identity is public and re-derivable over an injective tuple;
- document identity is a random prefix-determined token that survives damage; and
- current-world freshness is observed, never inferred from either identity.

**~SUSPECT** Public hashes over low-entropy opaque values are not an additional identity class;
they are a disclosure channel. A self-stored whole-file hash detects accidental damage only if the
expected value is independently available; an attacker able to rewrite both can recompute it. It
must not be called tamper evidence absent an external witness, secret MAC key, or authenticated
chain.

### syn-key-retention-is-not-forever

**+SURE** "Retain every key forever" overstates the first-party guidance. NIST says retain until
the key is no longer needed to decrypt the protected data
[A-nist-sp800-57-key-management-2020]. If all documents from a key era have been explicitly
removed, perpetual retention no longer follows. Key deletion still does not erase copied
ciphertext, and rotation/revocation needs its own explicit semantics.

### syn-dependency-hygiene-is-secondary

**~SUSPECT** Publication-age quarantine, committed lockfiles, effect-focused review, no unneeded
build scripts/proc macros, and named vendoring ownership are sound engineering hygiene
[A-rust-arrayref-supply-chain-attack-2026]
[A-chromium-adding-third-party-libraries-2026]. They do not select the durable architecture and
should not crowd out the higher-lock-in product decisions.

## syn-where-security-hurts-dorc

These are the findings most likely to attract justified human pushback because they make Dorc
less itself.

### syn-rich-detail-expands-sensitive-surface

**+SURE** Dorc's most distinctive promise is maximal, attributable explanation of why it did not
run a command. Generic logging minimization cuts directly against that product. Persisting fewer
typed survival/vouch/region/render conclusions may reduce exposure while preserving less of the
only account that can debug `kHALVES-elide-half`. The secure answer cannot be "log less" without
pricing lost recovery.

### syn-source-copies-improve-aid-and-hurt-containment

**~SUSPECT** By-value source is better for self-contained post-drift explanation and vendor handoff.
It is worse for hidden secret copies, backup lifetime, and surprising retention. Neither pole is a
generic engineering best practice; Dorc must choose a base plus an explicit stronger collection
mode.

### syn-prewrite-can-block-emergency-repair

**~SUSPECT** Mandatory pre-effect persistence gives the strongest receipt guarantee and may make a
full local disk stop remote remediation. An admin in an outage may rationally prefer a loud,
one-invocation override that spends the why guarantee to restore sh parity. The override itself can
become habitual theater. This is a real product policy, not a security theorem.

### syn-encryption-can-erase-debug-history

**~SUSPECT** Rich encryption protects artifact copies that leave without the key, and can protect
backups only when their key custody is meaningfully separate; it also adds key availability as a
precondition to historical aid. Platform key stores fail in headless, SSH, CI, and locked-session
contexts [A-gcm-credential-stores-2026]. A local key file is more available and weaker against
same-user code. Falling to plain preserves a smaller receipt but may discard exactly the opaque
detail needed after failure. Refusing preserves the promise and prevents operation. Every option
has a product cost.

### syn-binary-format-needs-own-tool

**+SURE** A binary durable can buy a stricter parser while making the debugging artifact dependent
on Dorc itself. That pushes against directness, plain-text interoperability, vendor handoff, and the
firefighter scenario. It is a particularly sharp conflict because the whylog exists for times when
ordinary tooling or the current binary may be unavailable.

### syn-explicit-cleanup-spends-attention

**~SUSPECT** Explicit-only cleanup respects user control and turns long-term housekeeping into a
task the lazy admin must remember. Automatic cleanup preserves availability and may silently delete
the evidence the admin needed. A mode or visible policy may be unavoidable.

### syn-separate-transport-paths-duplicate-code

**+SURE** Keeping separate at-rest and transport readers, errors, identities, and key material costs
code and review attention. Reusing one parameterized path is simpler and has repeatedly produced
cross-protocol, downgrade, and truncation failures. This is one of the clearest cases where ordinary
maintainability/DRY instincts should lose.

## syn-cross-cell-exclusion-check

| cell | durable consequence | confidence |
|---|---|---|
| Probe phase | Records describe reads and planning inputs; a whylog write must not trigger another host observation. Lost intake integrity yields report-only output and no mutation authority. | +SURE |
| Apply phase | Actual execution, guards, divergence, cancellation, and quiescence differ from predicted disposition. An intent record cannot stand in for an outcome. | +SURE |
| Admin workflow | Local firefighter detail and low setup cost dominate; key loss, binary-only inspection, and manual retention burden are product regressions. | ~SUSPECT |
| Oracle-author workflow | Shareable plain output, exact source/oracle identity, stable locators, and attributed claims dominate; raw host output is usually less useful than typed conclusions. | ~SUSPECT |
| Reliable oracle | Rich structured evidence can explain elision and survival without arbitrary output. The durable still remains report-only and stale. | +SURE |
| Unreliable oracle | Contracted report bytes may contain arbitrary content; declined, malformed, or contradictory records must stay bounded and visibly incomplete. | +SURE |
| Single target | No ambient host is still preferable; the full scope may look redundant but becomes the migration seam. | ~SUSPECT |
| Multiple targets/retries | Every target, attempt, generation, and exchange must be explicit. Arrival order and deadline behavior become influenced inputs, not controller truth. | +SURE |
| Pre-wall elision | The minimum receipt needs measured fact, exact vouch, decision, source identity, and actual apply outcome. | ~SUSPECT |
| Post-wall survival | This is the highest-value durable case: complete crossing, footprint/backing comparison, resolver/reach input, consent, reference result, and authority receipt. Omitting it preserves raw noise and loses the explanation Dorc uniquely owes. | ~SUSPECT |

## syn-assessment-of-thirty-ra

| `30Ra` direction | assessment after the round |
|---|---|
| Rich immutable receipt, never cache/authority | **+SURE keep.** Strongly aligned with project goals and external failure evidence. |
| Shape-based sensitivity; no scrubbed/safe-share promise | **+SURE keep.** One of the best-supported findings. |
| Rich/plain separate projections | **+SURE keep the separation;** exact fields and grammar remain open. |
| One readable envelope; no whole-file encryption | **+SURE still human-acked and unresolved by research.** `turn10` conflicts rather than supersedes. |
| Every minted stable Spine species gets a view | **~SUSPECT narrow to purpose-driven semantic projections.** Species membership is not enough. |
| Exact already-admitted records in rich | **~SUSPECT keep if bounded and encrypted;** a typed parsed account should remain primary. |
| Source reconstruction by default | **~SUSPECT keep.** Add an explicit by-value/cache question later, not incidentally. |
| Unaccounted output stays remote | **~SUSPECT keep.** Strong containment and anti-log-collector boundary. |
| One immutable file per invocation | **~SUSPECT not refuted.** Re-evaluate only after mapping plan/apply artifact and temporal commit boundaries. |
| Explicit-only cleanup | **~SUSPECT open.** Silent count prune is wrong; unbounded default-on storage is also wrong. |
| Trusted directory, private create, atomic no-replace publication | **+SURE keep as desired contract.** Later `std` recommendation does not meet it. |
| Recorded/re-derived four-way state | **+SURE keep.** Distinguishes history, current semantics, damage, and drift. |
| Enable existing influence export | **+SURE conceptually cleared by the human review;** final recorded type must remain report-only and preserve unknown/missing distinctly. |
| Reserve previous-root/witness room | **--WONDER.** Cheap representation room may be useful; no current integrity story earns implementation. |

## syn-package-and-tool-candidate-status

| candidate | useful evidence | blocking uncertainty | synthesis status |
|---|---|---|---|
| C2SP `chunked-encryption` | Strong, current, narrow spec; key commitment; deterministic nonce derivation; termination; negative vectors [A-c2sp-chunked-encryption-2026] | Editor copy; one listed Go implementation; no maintained Rust package; Dorc integration topology unspecified | **~SUSPECT high-value candidate, not selected** |
| EverCBOR `cborrs` | Strict deterministic fragment; proof-backed parser; safe Rust extraction; direct report experiments [A-ramananandro-evercbor-evercddl-2025] | Research dependency, bus factor, vendoring governance, panic policy, direct readability, long-term tool availability | **-GUESS candidate requiring independent rerun and ownership decision** |
| EverCDDL | Can check disjoint alternatives; valuable invariant decomposition | Model/code drift if checker-only; generated code unreviewable; platform/toolchain/bus-factor cost | **-GUESS optional checker research, not architecture** |
| `age` whole blob | Maintained format/specification owns many crypto seams; mature construction [A-c2sp-age-format-spec-2026] | Whole-blob mismatch, Rust dependency graph, tool randomness/DST seam, skeleton/blob binding, direct readability | **~SUSPECT live counter-candidate** |
| `minicbor`/ordinary strict decoder | Small, maintained, reviewable fallback per report | Dorc would own canonicality, schema, unknowns, and strict-recognizer enforcement | **--WONDER fallback to remeasure later** |
| `std` storage | Small and portable baseline | Cannot establish the full handle, ACL, publication, sync, and deletion contract on every platform | **+SURE insufficient as the whole answer** |
| SQLite/redb | Mature transaction/concurrency machinery | Wrong artifact shape, mutable shared failure domain, readability loss, configuration seams | **~SUSPECT reject for the base durable** |

## syn-first-party-verification-notes

**+SURE** The synthesis read all eleven reports. It selectively read raw first-party
material for the highest-load-bearing claims rather than treating report quotations as
independent evidence:

- C2SP `chunked-encryption` for salt, context, commitment, chunk, nonce, and truncation
  semantics [A-c2sp-chunked-encryption-2026];
- CBOR Sequences for self-delimitation and tail-truncation limits
  [A-rfc8742-cbor-sequences-2020];
- CBOR stable storage for what its file labeling actually provides
  [A-rfc9277-cbor-stable-storage-2022];
- Lampson/Sturgis for write-ahead intentions and the atomic-commit precondition
  [A-lampson-sturgis-intentions-1979]; and
- Gray for real actions, restartability, compensating actions, and the limits of remote
  transaction reasoning [A-gray-transaction-real-actions-1981].

**~SUSPECT** The package measurements and executed probes in `turn10` and `turn11` are
useful direct observations, but this synthesis did not reproduce them. Most were run on one
Windows host, some generation work through WSL, and several selections have a shelf life of
months. They should be re-run from a checked-in measurement recipe at the point a design is
ready to select dependencies.

**~SUSPECT** Storage, write-ordering, cleanup, and vendoring had no single external
specification against which the whole proposed Dorc policy could collide. Their component
facts are often strong; their policy syntheses are the least independently constrained parts
of the round.

## syn-next-phase-design-questions

The next phase should answer these before naming a final format or crate. These are questions,
not recommendations.

1. **ask-threat-principals-and-guarantees:** Which exact principals does rich encryption defend
   against: another local user, copied backup, stolen unencrypted disk, sync provider, vendor,
   or compromised same-user process? What does it explicitly concede to each?
2. **ask-direct-readable-file-requirement:** Must an operator inspect and quote the stored file
   without Dorc, or is a bundled inspector sufficient? This reopens or closes binary honestly.
3. **ask-document-crypto-topology:** Are opaque values independently encrypted records, one
   document blob, or reviewed groups? What exact construction binds the complete structural
   skeleton, ordering, projection, and termination to them?
4. **ask-projection-global-mode-proof:** What mechanically proves the whole document is rich or
   plain, and that plain unknown retention cannot carry rich-only bytes?
5. **ask-plan-apply-temporal-boundary:** What durable artifact exists immediately before the
   first remote packet in every invocation form, and which later record can honestly describe
   actual outcomes?
6. **ask-prewrite-failure-product-policy:** Does inability to publish that pre-effect record
   refuse mutation, proceed with a loud downgrade, or require explicit override? Which Dorc
   promise is being spent?
7. **ask-encryption-unavailable-policy:** Is plain a normal explicit projection, a visible
   automatic pre-write resolution, or a refusal? What detail is lost and can the user discover
   it before mutation?
8. **ask-store-root-and-entry-links:** Which user-selected roots may resolve through symlinks,
   and what handle/identity guarantees apply after resolution to internal create/read/delete?
9. **ask-platform-durability-contract:** Which guarantees are common, which are weaker on
   Windows or network/sync volumes, and which missing safe-Rust primitives justify a dependency
   or a documented limit?
10. **ask-purpose-driven-durable-species:** For each Spine species/field, which firefighter or
    oracle-author question becomes unanswerable if it is omitted? Is the value reconstructable?
11. **ask-retention-and-availability-policy:** What default bounds disk growth without silently
    deleting the one receipt the user expects? How is the policy made visible and testable?
12. **ask-key-era-and-document-lifecycle:** How are new-encryption and old-decryption authority
    separated, how are key eras bounded, and when is an old key genuinely no longer needed?
13. **ask-at-rest-transport-type-separation:** Which shared vocabulary is safe, and what compile-time
    boundaries prevent at-rest prefix recovery, errors, identities, or keys from reaching transport?
14. **ask-selection-measurement-replay:** What checked-in commands reproduce crate graph, unsafe,
    build-script, parser-vector, panic, maintenance, and platform measurements when selection occurs?

## syn-final-research-disposition

**+SURE** Carry forward as durable principle:

- replayed material is report-only and temporally stale;
- influence, sensitivity, attribution, and authority are orthogonal;
- opaque sensitivity is shape-classified;
- projections are distinct types and global document modes;
- parsing is byte-first, total, independently bounded, and strict;
- a flat sequence needs an explicit completion rule, cryptographically bound in rich mode;
- context encoding, identity domains, and cryptographic uses are injective and separated;
- document, decision, wire, and freshness identities do not collapse;
- storage guarantees are per-platform and handle/ownership-oriented;
- at-rest recovery behavior must not leak into transport; and
- detailed source/output collection is explicit and never sold as scrubbed.

**~SUSPECT** Carry forward as strong candidate requiring a human design sitting:

- rich encryption plus a narrower plain projection;
- reconstruction-first source;
- a pre-effect intention boundary;
- explicit source/output enrichment modes;
- C2SP-style committing chunked encryption;
- immutable independently scoped documents; and
- a state-directory store with visible retention policy.

**+SURE** Do not carry forward as settled:

- deterministic CBOR as the chosen format;
- `cborrs` or EverCDDL as dependencies;
- project-authored Cobblestone code;
- one salt per document combined with per-record C2SP messages;
- one decision plus one outcome file per invocation;
- `std` alone as the full storage contract;
- pathname-based follow-then-validate as the symlink answer;
- explicit-only cleanup, date sharding, or deletion ledger;
- perpetual retention of every historical key;
- human-readable semantic filenames containing host identity;
- every stable Spine species automatically becoming durable; or
- any claim that rich or plain output is generically safe to share.

**+SURE** The most important result is negative: the research has made the dangerous
joins visible, but has not yet reduced them to one reviewed construction. That is a successful
gathering round. Treating its late coherence as a design would throw away that success.
