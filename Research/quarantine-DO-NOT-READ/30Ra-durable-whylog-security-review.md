# 30Ra - Secure durable receipts: design review and specification

> Tier: quarantined, implementation-facing security and product design. This
> document supersedes the prior `30Ra` review and all of `30Rc`. `plans/30R`
> remains the out-of-quarantine implementation surface until the waiting
> implementation-planning conductor reconciles it against this document.
>
> Authority: root human documents and human-typed rulings outrank this design.
> **[ACKED]** marks direct human direction. **[ACKED, SOFT]** marks accepted
> direction intentionally more malleable than ordinary Dorc policy while security
> research and implementation experience mature. **[REVIEW]** marks the reviewed
> construction selected from the research rather than a human-authored security
> judgment. **[OPEN]** marks work this document deliberately does not settle.
>
> Scope: the durable receipt family, its strict physical envelope, rich/plain
> projections, cryptographic and key boundaries, plan/apply correlation, storage,
> read-back, product policy, type architecture, invariants, and verification. It
> does not turn the receipt into a cache, approval object, executable artifact,
> or source of live authority.

# normative-design-and-plan

## design-in-one-screen

**[ACKED, SOFT]** Dorc writes a graph of immutable receipts rather than one
mutable log. A `PlanReceipt` records probing, analysis, decisions, and why. Each
application attempt adds a small pre-dispatch `ApplyIntent` containing the exact
admin-authorized executable bytes and apply context. Every graceful terminal
state attempts an `ApplyOutcome` containing actual execution and host results.

**[REVIEW]** All three use one versioned, directly readable, strict textual
envelope. The readable skeleton contains no arbitrary authored or host text. A
plain receipt contains no opaque-value-capable region. A rich receipt contains
exactly one grouped encrypted opaque overlay, physically dislocated from the
skeleton and reverse-indexed back to structural slots. The skeleton never points
into encrypted content.

**[REVIEW]** Both rich and plain receipts carry one whole-document public-key
signature over the exact literal serialized bytes. Rich opaque data is encrypted
as one Age v1 message, taken substantially as-is from its maintained
implementation. The encrypted plaintext repeats the document identity,
projection, receipt species, and a digest of the literal skeleton before carrying
the reverse overlay. Signing and encryption use separate keys and separate
lifecycles.

**[ACKED, SOFT]** The default policy is the strongest posture that does not force
a non-dominating platform or UX split: private key files under the per-user
configuration root, signed rich receipts, and required pre-dispatch publication.
Platform keychains or hardware are not a default barrier to entry. Two standalone,
synchronous configuration actions may expand into closed ordinary settings: one
convenience downgrade and one guided hardening upgrade. Neither is attachable to
another command, neither infers consent from TTY presence, and neither silently
absorbs future options.

**[ACKED, SOFT]** Required receipt failure refuses before the first mutative
dispatch. After Dorc commits to dispatching the first potentially mutative book
command, durable/debugging failure alone does not abort otherwise coherent
orchestration. Transport, execution, attribution, or mutation-integrity failure
continues to follow its own fail-fast policy.

## v1-spike-scope-and-exit

**[ACKED, SOFT]** `dorc-receipt/1` is a spike-quality architectural kernel, not
the complete product described throughout this document. V1 implements the smallest
coherent shape that forces the difficult and retrofit-hostile decisions:

- one standalone receipt crate with no filesystem, environment, network, or user
  configuration;
- the three receipt species and typed correlation identities;
- strict literal-byte textual serialization and parsing;
- signed plain and grouped-rich projections using maintained Age and Ed25519 libraries;
- reverse-overlay validation and exact multi-file `ApplyImage` round trips;
- reader/writer trust, completeness, projection, and affine typestates;
- report-only recorded types with no authority conversion;
- minimal projection of values that already exist in the current whylog path;
- injected fixture signer/sealer and receipt sink/source interfaces;
- one minimal production local-file key provider and immutable receipt store — standard
  per-user by default or explicitly sited by the admin — specified exclusively by `30Rd`;
- the first-dispatch permit and failure-direction state represented and exercised under
  DST and through the concrete local durable edge; and
- one or two full product routes that write, read, correlate, and explain all three
  species.

The required product-spanning routes are:

1. book/oracle plus hostsim probe -> plan -> `PlanReceipt` write/read -> `dorc why`
   explanation; and
2. single- or multi-file apply image -> `ApplyIntent` -> DST/hostsim execution ->
   `ApplyOutcome` or explicit missing outcome -> correlated `dorc why` explanation.

V1 exits when these routes are green, the crate boundaries and typestates are in use,
the selected libraries interoperate in ordinary round trips, strict malformed/truncated
inputs are covered, and the old format, reader, writer, fixtures, and live paths are
entirely removed. It does not wait for every semantic species, alternative/platform/
hardware key providers, rotation/import/export, profiles, retention, custom key roots,
broad multi-host execution, source archives, or the full defense-in-depth verification
programme.

Everything else in the normative half is the target shape to preserve while staging.
Sections marked later are not v1 build obligations unless the implementation conductor
finds that omitting one would foreclose or distort the v1 architecture.

## v1-minimal-production-durable-edge

**[ACKED]** V1 must leave the shipped binary able to persist and read its new receipt
family after the old whylog is removed. The minimal production baseline is one
versioned local-file keyset under the per-user configuration root plus one versioned
immutable receipt store under the per-user state root. It is not a temporary weak mode
and not a complete provider ecosystem.

`Research/quarantine-DO-NOT-READ/30Rd-minimal-production-durable-edge.md` is the
single normative specification for its key files, first-use state machine, store,
platform guarantees, product assembly, failure directions, and test/fault discipline.
This document retains only the enclosing architecture and claims. `30Rb` schedules
the `30Rd` work before old-durable removal and does not restate it.

## threat-model-and-security-claims

**[ACKED, SOFT]** The primary confidentiality threat is an unintended recipient
obtaining receipt files, filenames, directory listings, backups, sync copies, or
support attachments without controlling the operator account, source tree,
running Dorc process, or applicable private keys.

The useful distinction is which material escapes together:

- a receipt attached to an issue without its keys retains its field-level
  confidentiality;
- a state directory synced without the configuration/key directory may retain
  that separation; and
- a whole-home backup containing receipts and private keys defeats most of it.

Same-user malicious code, a compromised operator account, controller root, and
an attacker holding both receipt and private keys are outside the whylog
confidentiality and historical-integrity claim. They can read inputs before
encryption, invoke authorized key operations, replace Dorc, or spend fleet
credentials directly. Do not imply that local cryptography keeps secrets from the
principal authorized to use its keys.

Other in-scope surfaces remain independent:

- managed hosts may shape bytes, omissions, response timing, and resulting
  conclusions, but never controller-owned target, attempt, generation, source-set,
  or invocation attribution;
- a receipt may be malformed, damaged, sync-conflicted, version-skewed, or
  adversarially supplied to a reader;
- other local users receive ordinary OS isolation; and
- crashes, full disks, concurrent runs, and sync clients are reliability failures
  whether or not an attacker caused them.

The format promises only which field classes a projection can contain. It never
promises generic secret detection, scrubbing, safe sharing, truthful hosts,
non-repudiation, or protection after private-key compromise.

## influence-tracking-stays-upstream

**[ACKED, SOFT]** Influence tracking is a larger upstream concern than durability.
It is a cross-cutting analysis over host-reported data and the analyzer's own data
and control flow; it reaches the pure kernel and stable semantic types. The receipt
subsystem does not define, compute, or simplify those semantics.

The durable overlap is narrow and mandatory:

- project each recorded influence account without lowering it;
- preserve missing, malformed, and unknown grades distinctly at the conservative
  report posture;
- continue influence through projection, selection, arrangement, and rendering;
- keep recorded influence orthogonal to attribution, sensitivity, and authority;
  and
- provide no route from rehydrated accounts to live decisions or licenses.

A valid signature proves controller authorship of bytes, not truth. Encryption
faithfully preserving host-influenced material does not make that material trusted.

## receipt-event-graph

**[ACKED, SOFT]** Receipt history is a graph:

```text
                              ApplyIntent A1 -> ApplyOutcome O1
                             /
PlanReceipt P -------------- ApplyIntent A2 -> no outcome
                             \
                              ApplyIntent A3 -> ApplyOutcome O3
```

The diagram is one readable slice, not a cardinality rule. The actual graph is M:N:
one plan may be edited and applied repeatedly, and one apply assignment may compose
zero, one, or many originating presented plans under admin control. An intent contains
one or more target/image assignments; one intent has zero or one outcome. Plan and
apply are not one transaction and do not form a fixed one-to-one-to-one trio.
Correlation is report narration over immutable identities, never continuity of world
freshness, host generation, or authority.

The reader has explicit states for:

- plan-only;
- plan plus apply intent with no recorded outcome;
- plan plus intent plus outcome;
- intent whose plan receipt is unavailable;
- outcome whose intent is unavailable; and
- disagreement among recorded identities.

Missing graph edges are incompleteness. They never synthesize history or imply
success, failure, or no mutation.

## receipt-rooted-attention-and-selection

**[ACKED]** A `why` question has one receipt as its attention root. The reader follows
only the typed graph edges needed to answer that rooted question; disconnected graphs
in the same store never merge into one explanation. Graph closure is automatic, not a
user-selected breadth. A missing, unreadable, untrusted, damaged, or disagreeing
required sibling leaves the answer explicitly partial and never substitutes another
history.

The user-facing vocabulary is fixed as follows:

- `--receipts <folder>` selects the exact receipt-store directory used by plan/apply
  publication and by why graph lookup. Absent, the standard per-user store applies.
  It never changes key custody.
- `--receipt <file>` admits one explicit root receipt file to `why`. It is report-only
  and cannot mint publication, trust, approval, or action.
- `--receipt-id <id>` selects one exact root from the selected/default store.
- `--receipt-last` derives the newest root from the selected/default store.
- `--all` selects the deepest explanation register only; it never enumerates unrelated
  receipt histories.

The root selectors are mutually exclusive; `--receipts` is orthogonal and may supply
typed sibling lookup for an explicit file. Sibling acquisition uses authenticated typed
identities against the bounded selected store, never a receipt-provided pathname,
plugin, provider, or arbitrary adjacent-file scan.

The last-root derivation first takes the maximum-order cohort, then collapses members
that are graph predecessors of another cohort member. One remaining terminal root is
usable; several incomparable terminal roots are an ambiguity. No random receipt-ID
or species ordering breaks the tie, and damaged newest material never falls back to an
older complete root.

The closure is question-directed rather than the entire undirected component. An
outcome may require its intent and originating plans; selecting one plan does not pull
all later apply attempts. Store enumeration needed to discover reverse typed edges is
an implementation act and never a semantic “all receipts” answer.

`--receipts` is a controller-supplied path resolved once at the CLI edge. The local
store validates and owns that root under the same platform-specific publication rules
as the default store. Host bytes, receipt content, source text, TTY state, and fallback
logic cannot site it. `--receipt` is independently byte/count bounded and verified
before semantic rendering; selecting an external file grants neither signer trust nor
permission to discover a backend.

## plan-receipt-content

`PlanReceipt` is the primary whylog and may be large. Its stable semantic views
cover, where truthfully minted:

- invocation, policy, controller semantics, target/context, attempt, and
  generation;
- ordered source and oracle identities;
- exact emitted plan/decision identity;
- admitted contracted records and their admission outcome;
- observations and merge/collapse results;
- vouch attachment and suspension;
- probe shipment and decline;
- solve certification and validity rounds;
- dispositions and universal region decisions;
- survival witnesses, crossings, resolver/reach inputs, consent, and reference
  results;
- load, custody, and render/artifact decisions;
- stable narratives and locator edges;
- influence accounts; and
- explicit projection omissions.

V1 projects only the smallest useful current slice needed by the plan/why e2e:
invocation/source identity, admitted record stream, disposition, decision digest,
recorded influence, and enough stable location/narrative data to explain the selected
sites. The remaining species are pre-publication enrichment, not v1 stubs or empty
format promises.

Species membership in Spine is not by itself sufficient. A durable view must
answer a named firefighter or oracle-author question, preserve a historical fact
that cannot be reconstructed, or account for a projection loss. Working lattice
state, process-local handles, live licenses, and aspirational empty fields are not
durable content.

## planning-book-bytes-and-durable-locators

**[ACKED]** A rich `PlanReceipt` carries the exact bytes Dorc already acquired for every
general-sh source not accepted as valid `dorc-lang`, once per source identity. Valid
`dorc-lang` sources carry their ordered identity, path, digest, length, source class,
and provenance but not full source bytes. This is the mechanical boundary beneath the
user-facing book/oracle gloss: general source may mutate; `dorc-lang` is mutation-pure by
contract. Plain receipts mark general-sh content withheld. A source omitted by a receipt
bound remains explicit and
makes later source-dependent explanation partial; it does not invalidate unrelated
recorded conclusions.

Persistence expands no observation: only bytes already acquired for analysis qualify.
A path or argv value never licenses a new read. Exact bytes enter the grouped encrypted
overlay under the receipt's existing field and aggregate budgets, with a distinct full
book-content tag rather than masquerading as an excerpt. They are neither normalized
nor transcoded. The source-map and durable locator use the same acquired byte domain:
LF indexes physical lines and a CR in CRLF remains an input byte, so newline conversion
is drift rather than invisible equivalence.

Every durable site decision carries a receipt-owned projection of
`aid::locator::Locator`, not a flattened line pair and not deserialization into the live
type. The fixed V1 projection preserves:

- the closed stage kind (`Authored`, `Loaded`, `Copied`, `Generated`, `Claimed`);
- source ordinals or generated-artifact identity;
- exact byte spans;
- bounded origin edges and the locator head; and
- explicit withheld/omitted/damaged state.

`SourceFileId` and `StageId` are process-local and become document ordinals. Locator
payloads are report-only, bounded, and complete-or-partial by type; no recorded locator
converts into a live `Locator`, `ProvId`, license, plan input, or authority. Exact book
bytes plus an authored span recover the historical physical line without reparsing into
an identical syntax arena. Oracle loci remain useful as path/digest/span attribution;
matching current oracle bytes may enrich display, while absent or drifted oracle bytes
do not erase the recorded conclusion.

A current `path:N` is compared only with historical physical line N of the same source.
Dorc never infers that a moved line is the same operation. Exact line bytes permit the
recorded address; differing/missing bytes refuse only that specific address pending a
future explicit current-versus-recorded selector. The rest of the receipt still renders
best-effort. Any other book drift produces a finding and labels analysis-derived links
historical until a later dependency comparison can narrow that qualification.

Authentication, structural recovery, and authority remain independent. Failed outer
verification releases no authenticated receipt and no opaque value whose binding is not
independently established. The aid plane may still carry bounded lexical/strictly
recoverable structural fragments into maximally degraded report nodes, preserving where
corruption prevented further derivation. Such nodes never enter graph authority or
action, and uncertainty propagates rather than being rounded away.

Inline encrypted book custody is V1's do-now placement. A future dislocated/deduplicated
encrypted source store, if designed, may replace that placement while preserving the
same content identities, locator projection, and explicit unavailable states.

## recorded-versus-rederived-and-frozen-kernel

Historical conclusions and conclusions reconstructed under current inputs and semantics
never substitute for one another. Every consumer preserves recorded-only, re-derived-only,
both-agreeing, and both-disagreeing states. Unavailable inputs and controller-version
differences remain explicit comparison context.

The do-now durable boundary is the public `dorc-receipt::report` surface. It seals a
report-only `RecordedWhyFacts` model from verified or partial receipt state, rooted graph
closure, recorded decisions, exact general-sh source, durable locators, and typed
current-source observations supplied as data. The pure receipt crate performs no I/O. It
carries explicit authentication/completeness/influence state and exposes no raw receipt
payload, provider/key choice, live `Disposition`, claim, license, `PlanAuthority`, operation
endpoint, or conversion back into planning/probing/apply.

Arbitrary recorded source/detail values stay private behind receipt-owned typed handles.
They have no bare byte/string accessor and no revealing `Display`, `Debug`, serde, equality,
ordering, or hash implementation. Their one public exit is an encoder-mediated method: a
consumer supplies an explicit value-class-aware encoder, making every byte release a named
review seat rather than a convenient formatting call. The CLI adapter implements that exit
through the existing destination-specific aid encoders.

CLI remains the composition root: it acquires current source and receipt/store material,
calls the pure report API, and later joins facts with aid/weft. `dorc-aid` stays generic and
MUST NOT depend on `dorc-receipt`; otherwise receipt persistence enters analysis/oracle/syntax
transitively through the describe plane. No new crate is required. A dependency or signature
from `dorc-receipt::report` to filesystem, provider, key implementation, `dorc-plan`,
`dorc-aid`, weft, or CLI is a boundary violation.

Report-only kernel re-derivation over original inputs remains an intended capability: it
will populate `ReDerivedDisposition` and the four-way recorded/current comparison and
allow a later Dorc version to say where its answer changed. The existing correctness-kernel
is frozen for this arc, so factorizing calculation from authority minting is DEFERRED to a
separate kernel-authorized round. Running the authority-bearing operation kernel and merely
hiding its plan output is not a substitute.

Recorded-only facts remain the fallback whenever re-derivation is unavailable. Once the
sealed facts boundary exists, arranging and rendering those facts is ordinary why-surface
work: it may proceed outside quarantine without reopening receipt parsing, trust, graph
selection, source custody, or authority. A non-security-focused consumer receives only the
sealed model and ordinary steering; it never needs the rationale in this document.

## apply-intent-content

`ApplyIntent` is semantically narrow:

- producing apply invocation and policy;
- exact originating `PlanReceipt` identity, if available;
- non-empty ordered admin-owned assignments, each carrying one exact `ApplyImage`
  containing every executable stream and file that assignment will use, by value;
- original artifact form, entrypoints, ordered roots, transitive dependency topology,
  target-relative paths, file kinds/modes where relevant, and exact bytes;
- target/context and generation;
- controller semantics/version;
- tunnel/session and resolved target identity established at standup;
- decision/artifact identity; and
- the requested publication policy and the exact prepared pre-publication state.

A digest identifies bytes that remain available; it does not recover edited stdin
or a deleted artifact. An informal line diff is insufficient for arbitrary added,
removed, multipart, or remapped shell. Unlike plan-source reconstruction, every byte
the apply depends upon is therefore required by value before dispatch.

`ApplyImage` preserves what apply actually uses: one stream, a multipart tree, a set
of books and generated bundles, or an entire transitive `.sh` dependency tree. Receipt
serialization may carry that file-set through a deterministic txtar-like archive or
another lossless container, but that is only an encoding of the observed apply image.
It never grants permission to flatten, bundle, relocate, rewrite imports, normalize
paths, or otherwise change executable form. If apply was not bundled, receipt creation
must not bundle it semantically. Re-materializing the recorded image reproduces the
same topology and bytes.

The complete apply image is opaque-value-capable and may make this physically larger
than its semantic role suggests. If it exceeds required receipt limits, strict policy
refuses before dispatch or uses a separately reviewed required-image publication;
best-effort source-archive availability cannot satisfy the mutation gate.

## apply-outcome-content

`ApplyOutcome` is attempted on every graceful terminal state, including:

- complete success;
- command failure;
- detected transport failure;
- mutation-integrity abort;
- cancellation;
- partial multi-target completion;
- guard result and plan divergence; and
- unknown remote outcome.

It carries actual per-site execution, statuses, cancellation/quiescence, bounded
host output admitted by policy, durable failure, and references to the exact
`ApplyIntent`. It does not repeat plan-time reasoning except by identity.

Controller crash, process kill, power loss, or an unusable post-dispatch durable
sink may leave no outcome. Absence means only that no outcome was durably recorded.
A reingested `ApplyIntent` proves only that these exact signed intent bytes exist. It
records prepared intent, requested policy, and pre-publication prerequisites; it does
not reconstruct the ephemeral required-grade publication proof or prove dispatch
eligibility. It cannot prove that the affine permit was minted/consumed, that a
dispatch call occurred, or that anything reached or changed the host. Presence
without an outcome remains explicitly unknown.

V1 needs only the outcome fields produced by its DST/hostsim route, including success,
failure/unknown, per-site identity/status, intent correlation, and explicit no-outcome.
Complete executor, cancellation, quiescence, output, and multi-target coverage is later.

## default-policy-and-profile-expansion

**[ACKED, SOFT; LATER PRODUCT SURFACE]** Security defaults are not "maximum conceivable security."
They are the strongest generally useful posture before a non-dominating platform,
availability, or UX split appears.

The pre-publication product baseline is:

- receipt signature required for any receipt called valid;
- rich projection preferred for ordinary plan/apply receipts;
- private file key providers under the per-user configuration root;
- pre-dispatch `ApplyIntent` publication required;
- signed plain fallback only when explicitly configured;
- no cleartext opaque-value projection;
- no automatic inference from TTY presence;
- no in-file algorithm negotiation;
- no automatic deletion during an unrelated invocation; and
- filename policy excludes host identity and opaque values by default.

The baseline filename may carry receipt species, a UTC ordering component, and a
random document token from a case-fold- and normalization-invariant alphabet. A
directory listing remains activity metadata and is never described as private.

## convenience-profile-is-closed-expansion

**[ACKED, SOFT; LATER PRODUCT SURFACE]** A standalone convenience action, strawman
`dorc --leave-me-alone`, may configure a predetermined set of ordinary settings
for a low-risk accepting user. It is not attachable to plan, apply, why, or another
command.

It synchronously shows every setting changed, the cost accepted, and the command
that reverses that individual setting. A common intended reaction is to retain the
profile while walking one or more changes back manually.

The first product expansion may include:

- best-effort rather than required pre-dispatch receipt publication;
- signed plain fallback when rich key material is unavailable;
- a more revealing filename policy; and
- an explicitly selected bounded automatic-retention policy if that policy is
  designed before release.

It never enables cleartext opaque fields in the initial product format. Future settings remain at their
unconfigured baseline and never silently join a prior expansion. The user must
rerun the action or set the new option individually. The expanded settings and the
profile revision that selected them enter receipt policy and decision identity; no
generic "security off" bit exists.

## hardening-profile-is-guided-expansion

**[ACKED, SOFT; LATER PRODUCT SURFACE]** A mirrored standalone hardening action requires interaction and
walks the user through platform-specific improvements that are valuable but too
failure-prone or burdensome for first-run defaults. Strawman product names remain
open.

The first guided product expansion may offer:

- macOS Keychain, Windows protected storage, Linux Secret Service, TPM, hardware
  token, or organization-provisioned key providers where usable;
- dislocated private-key paths outside the home directory;
- rich-required with no plain fallback;
- explicit trusted signing-key policy for imported receipts;
- stricter crash/power-loss publication grade where the platform supports it;
- locked retention policy; and
- organization-managed public-key distribution.

The action explains headless, SSH, CI, recovery, backup, and portability costs
before each choice. It expands into ordinary settings, records them individually,
and does not remain an opaque hardening bit. Future settings do not join without a
new guided run.

TTY presence remains a Dorc I/O/mode signal and never implies user availability,
security consent, or permission to run either profile.

## rul-one-readable-envelope

**[ACKED, SOFT]** V1 has one physical grammar. Format is not a user preference.
Rich/plain projection, field retention, filename disclosure, key provider, and
receipt gating are policy; multiple hostile parsers and damage models are not.

**[REVIEW]** The outer envelope is an age-shaped strict textual format rather than
general JSON, YAML, protobuf, or CBOR. It uses:

- ASCII structural syntax;
- LF only;
- fixed line and section forms;
- closed record and field vocabularies;
- decimal integers with no sign or leading zero except literal zero;
- lowercase hexadecimal or canonical unpadded base64 where those encodings are
  explicitly required;
- no comments, tabs, trailing spaces, ignored bytes, alternate indentation,
  Unicode normalization, duplicate fields, or tolerated recovery; and
- a mandatory signature trailer followed immediately by EOF.

The writer has one serialization for each typed value. The reader accepts only
that serialization. Byte equality is the format's equality relation. No parsed
model is canonicalized and reserialized for signature or identity comparison.

Exact grammar tokens and field ordering are implementation-plan work, but they
must instantiate this one-form language and land with committed valid/invalid
vectors before broad content growth.

## literal-bytes-are-the-authenticated-object

**[REVIEW]** Every document signature authenticates a domain-separated function
of the exact bytes before the signature trailer. Use a standardized injective
envelope such as DSSE PAE rather than raw concatenation
[A-dsse-protocol-spec-2024].

The signature domain includes at least:

- `dorc-receipt` protocol/version;
- document species;
- projection mode; and
- exact serialized body length and bytes.

The exact byte slice verified is the exact byte slice later parsed. No code path
parses, normalizes, reconstructs, or re-reads a payload to produce different bytes
for verification. This follows DSSE's explicit same-serialized-body rule and avoids
the SOPS class where writer and reader MAC different reconstructed trees
[A-dsse-background-rationale-2021]
[B-sops-mac-mismatch-sequence-comment-2026].

## plain-projection-is-signed-and-opaque-free

Plain means opaque-value-free by schema, not cryptography-free and not generically
secret-free.

The trusted `Plain` type has no encrypted overlay, clear opaque field, raw host
tail, or opaque-member variant. The runtime plain grammar separately rejects:

- an opaque compartment;
- a rich-only record or field;
- an unknown structural species;
- projection/domain mismatch; and
- any bytes between the skeleton terminator and signature trailer.

The signature attests that the controller signing identity produced these exact
bytes under Dorc's plain projection, schema, and policy. It cannot prove that a
field was correctly classified or that structural metadata is harmless. User-facing
claims remain shape-specific and never say "safe to share."

Rich-to-plain is semantic reminting, never textual stripping:

1. verify and parse the rich receipt;
2. narrow through the total typed plain projection;
3. serialize a new plain document;
4. sign the new exact plain bytes under the plain signature domain.

## rich-projection-has-one-reverse-overlay

Rich receipts contain one encrypted opaque overlay. The readable skeleton never
contains opaque member identifiers or offsets. It may contain only useful non-secret
status such as whether a field was captured, withheld, unavailable, or uncollected.

The decrypted overlay points back to already authenticated structural slots:

```text
(record-id, opaque-field-tag) -> exact bytes
```

The overlay validator requires:

- every key is unique;
- every target record exists in the authenticated skeleton;
- the field tag is opaque-capable for that record species;
- required captured fields exist exactly once;
- every overlay entry is consumed by the skeleton/schema account;
- no entry targets another document, target, generation, or projection;
- the inner document identity and species equal the outer values;
- the inner skeleton digest equals the digest of the literal skeleton bytes; and
- unknown, dangling, duplicate, aliased, orphaned, extra, or missing entries make
  the overlay incomplete and release no opaque values.

The complete overlay authenticates and validates before any opaque value enters a
report model or renderer. The reverse direction avoids the PDFex class where
attacker-controlled readable structure directs the reader into encrypted objects
[A-mueller-pdfex-partial-encryption-2019].

## opaque-overlay-uses-age-v1

**[REVIEW]** Use Age v1 as the rich overlay's whole-message encrypted envelope,
through its maintained implementation, with plugins disabled and one local X25519
recipient in the initial implementation [A-c2sp-age-format-spec-2025].

Age owns nonce generation, recipient wrapping, chunk authentication, streaming,
canonical inner format, and final-chunk truncation detection. Dorc does not expose
or reimplement its stream primitive.

The outer file carries one canonical ASCII-armored Age message. The armor is opaque
to the skeleton parser and covered byte-for-byte by the outer signature. The space
overhead and aggregate ciphertext length leakage are accepted for direct textual
inspection and expected receipt sizes. No compression is applied before or after
encryption.

Age exposes no caller-controlled AAD. That is acceptable in this construction because:

- the independent outer signature binds exact skeleton plus exact Age bytes before
  decryption;
- the authenticated Age plaintext repeats the outer document identity, species,
  projection, and SHA-256 digest of the literal skeleton; and
- the reverse overlay validates the complete relationship after authentication.

The inner digest is defense in depth, not the primary outer authenticator. A mismatch
releases no opaque value.

## whole-document-signature-is-mandatory

**[REVIEW]** Every valid v1 receipt, plain or rich, carries one public-key signature
over the complete preceding envelope. Use Ed25519 through a maintained implementation,
with one fixed algorithm under the v1 format and a domain-separated signing input.

The signature binds:

- format version;
- receipt species;
- projection mode;
- complete readable skeleton;
- Age armor bytes when rich;
- all identities, policy, omission markers, and terminators; and
- exact ordering and presence.

Per-field signatures are excluded. They do not prove completeness, multiply key and
format operations, and earn nothing absent independently portable selective disclosure.

Signature validity and signer trust are separate types and render states. A public key
embedded in or named by a receipt is self-asserted until controller policy or an
explicit caller supplies trust. The file may name a key identifier; it never selects
a key backend or grants trust.

## signing-and-encryption-keys-are-separated

Signing and encryption use independent keypairs. Neither is derived from the other,
and no root key is used directly for both modalities.

The default provider stores private key material under the per-user configuration
root, separately from receipts under the per-user state root. On Unix this means
`$XDG_CONFIG_HOME` (or its standard fallback) versus `$XDG_STATE_HOME`; platform
equivalents apply elsewhere. Creation is exclusive and restrictive. Key paths never
appear in receipts.

The write path needs only the Age public recipient but must access the signing private
key. The why path needs signing public keys and, for rich detail, the corresponding Age
private identity.

Record key identifiers and retain old signing public keys. Retain private Age identities
while corresponding rich receipts are intended to remain decryptable. Rotation starts
a new key era and never rewrites receipts or derives a replacement key from its
predecessor. Key deletion is loss of detail, not secure erasure.

The receipt crate accepts injected signing and encryption capabilities from day one.
V1 supplies those capabilities through the minimal local provider in `30Rd`.
Organization-managed files, dislocated mounts, platform keychains, TPMs, hardware
tokens, provider selection, and richer configuration are later product work. Merely
placing a readable key in `/etc` does not create separation if backup and access policy
still co-propagate it.

Documentation must state that a whole-home or whole-machine copy containing both
receipts and private keys defeats most local cryptographic separation.

V1 proves key-role separation through both fixture capabilities and the concrete local
provider. First-use generation and private-file storage are V1 obligations under `30Rd`.
Rotation, import/export, trust distribution, provider selection, and alternative custody
remain later product work.

## no-in-file-crypto-negotiation

The v1 format fixes:

- Ed25519 signature semantics and domain;
- Age v1/X25519 overlay envelope;
- skeleton digest algorithm;
- armor encoding;
- key-id representation; and
- overlay plaintext encoding.

No algorithm name from an untrusted file selects code, plugins, key stores, or
fallbacks. A future cryptographic change mints a new receipt format version with a
separately named reader. Before public release, v1 may be recut in place; after
publication, old readers remain exact and never become permissive compatibility paths.

## reader-state-machine

The reader follows one monotone state machine:

```text
BoundedBytes
  -> LocatedEnvelope
  -> SignatureChecked<SignerTrust>
  -> ParsedSkeleton<Document, Projection>
  -> OverlayState<Absent | Unavailable | Authenticated>
  -> CompleteReceipt | PartialReceipt
  -> SinkEncodedRender
```

Before signature verification, only a bounded lexical locator may run. It finds
the exact skeleton, optional armor, signature spans, and bounded key identifier used
to look up already configured verification keys. It performs no semantic
interpretation, file-selected backend/plugin dispatch, source read, host call,
allocation from file claims, opaque decryption, or rendering.

After signature verification:

- the exact signed bytes are parsed under the exact species/projection grammar;
- rich requires exactly one Age compartment and plain requires none;
- rich overlay authentication and complete validation precede opaque release;
- recorded influence, source paths, and conclusions remain report-only; and
- every source/host-derived value passes through its destination-specific encoder.

Signature missing, unknown signer, unavailable key, damaged armor, truncation,
unsupported version, and overlay mismatch are distinct report states. A bounded raw
structural view may be rendered under one global unauthenticated banner; individual
fields are never selectively promoted because they look plausible.

`PartialReceipt` and `CompleteReceipt` have no conversion in the permissive direction.
Valid-prefix evidence remains visibly partial and cannot satisfy APIs requiring a
complete receipt.

## writer-state-machine

The writer follows affine typestates:

```text
DraftReceipt<D, P>
  -> SerializedReceipt<D, P>
  -> SignedReceipt<D, P>
  -> PublishedReceipt<D, P, Grade>
```

Private fields and consuming transitions prevent callers from:

- signing a mutable semantic object instead of exact bytes;
- publishing unsigned bytes as a receipt;
- adding an overlay after signing;
- serializing `Plain` with an overlay;
- serializing `Rich` without exactly one overlay;
- changing policy or projection after serialization;
- reusing a publication capability; or
- converting a recorded receipt into live authority.

Serialized and signed values are immutable byte owners. Per-document signing operations
and publication permits are non-`Clone`, private, and consumed where one-time use is
load-bearing; the underlying configured signing authority may sign later documents.

## pre-dispatch-publication-boundary

**[ACKED, SOFT]** Tunnel standup remains fail-fast and precedes `ApplyIntent`
publication so the intent can bind resolved target/session identity. Under the default
policy, Dorc publishes the exact intent at the required platform durability grade before
committing to first mutative dispatch.

The one-way control type is:

```text
TunnelReady
  + ApplyIntentPrepared
  + ReceiptPolicyWitness
  + PublishedIntentOrConfiguredBypass
  -> MutationDispatchPermit
```

`MutationDispatchPermit` is affine and consumed by the first potentially mutative
dispatch. It cannot be minted from a failed strict publication, a plan receipt, an
outcome, a durable read-back value, or TTY presence.

After consumption, the apply enters `MutationDispatched`. Durable-only failure no
longer withholds mutation. This transition occurs even if delivery or remote outcome
later becomes unknown; it records authority spent, not mutation proved.

V1 threads and exercises this state machine both through the DST/hostsim injected-sink
route and through `30Rd`'s concrete local store. The concrete store's private required-
grade proof is what the production permit mint consumes; fixture/volatile proofs cannot
satisfy it. Broader filesystem grades and user configuration are later.

## post-dispatch-durable-failure-direction

After `MutationDispatched`, failure to prepare, write, sync, sign, encrypt, or publish
later receipt material does not by itself abort otherwise coherent orchestration.
Dorc reports the failure, retains bounded in-memory outcome material where possible,
and attempts a signed plain terminal report only if policy and keys permit without
claiming successful durable publication.

Loss of execution, transport, target attribution, generation, or mutation integrity
remains a separate abort condition. No "durables fail open" rule may catch those errors.

## no-append-in-v1

V1 uses immutable documents and no append. Append is not a file-count optimization;
it purchases incremental partial-outcome survival across controller crash and therefore
requires an incremental journal architecture, synchronization cadence, intentional valid
prefixes, encrypted continuation, concurrent readers, and finalization.

Without append, controller crash can lose in-memory outcomes for commands that completed.
The durable `ApplyIntent` remains and remote state is reported unknown. This is an explicit
forfeited value, not an accidental claim that no commands ran.

## storage-and-publication-contract

The store is a per-user collection of immutable independently identified receipt files,
not a mutable database, cache, or `latest` pointer. Required publication is a typed
runtime fact and is never reconstructed merely because complete receipt bytes are later
found. Incomplete material never reads complete, and platform guarantees remain explicit
rather than being normalized into false parity.

`30Rd` is the sole V1 mechanism specification for roots, names, publication,
synchronization, enumeration, incomplete files, platform differences, and testing.
Custom roots, stronger Windows ACL inspection, cleanup/retention, network-filesystem
grades, and broader platform hardening remain later design work.

## typesystem-architecture-and-mints

Aim high on types. Boilerplate is cheaper than a locally plausible security shortcut in an
agent-heavy codebase.

Use sealed newtypes for:

- `DocumentId`, `DecisionId`, `ApplyIntentId`, `OutcomeId`, and `KeyId`;
- `ApplyImageId`, `ApplyImageEntry`, `RecordedApplyPath`, and `ApplyTopology`;
- `ReceiptVersion`, `ReceiptSpecies`, and `ProjectionMode`;
- `SkeletonBytes`, `SignedBodyBytes`, `AgeArmorBytes`, and `SignatureBytes`;
- `RecordId`, `OpaqueFieldTag`, and `OverlaySlotKey`;
- every byte/count/depth/line/allocation limit;
- trusted versus self-asserted signer identity;
- authenticated versus unauthenticated versus damaged material;
- complete versus partial receipt;
- live versus recorded influence;
- publication grade; and
- pre-dispatch versus mutation-dispatched execution phase.

Projection should be a sealed type parameter with an associated overlay shape:

```text
Receipt<D, Plain> owns NoOpaqueOverlay
Receipt<D, Rich>  owns ExactlyOne<EncryptedOpaqueOverlay>
```

Receipt species should be sealed type parameters or distinct concrete types, not a free enum
at mutation-sensitive constructors. Signature domains derive from the type and cannot be caller
selected.

Reader trust should remain a type parameter:

```text
SignatureChecked<TrustedSigner>
SignatureChecked<SelfAssertedSigner>
SignatureUnavailable
```

Only the trusted/complete path may render authenticated claims. All paths remain report-only.

Use Rust move semantics as affine enforcement for:

- per-document signing operations;
- unpublished private temporary objects;
- `MutationDispatchPermit`;
- first-dispatch transition;
- rich overlay plaintext before sealing;
- decrypted opaque bytes before complete overlay validation; and
- publication ownership used for cleanup.

Do not implement `Clone`, `Default`, public fields, broad `From`, serde deserialization into
authority-bearing types, or generic downgrade/upgrade conversions on these values.

## builder-security-invariants

The following new builder laws are governed surfaces. They will become ordinary `inv-*`
engineering laws in the relevant crate-local steering files when the implementation plan
assigns ownership. Existing `sinv-host-evidence-ingress`, `sinv-controller-attribution`,
`sinv-generation-revocation`, `sinv-decision-identity`, `sinv-private-authority-mints`,
`sinv-hostile-sensitive-orthogonal`, `sinv-sink-encoding`, `sinv-sensitive-artifacts`,
`sinv-integrity-failure-mutation`, and `sinv-durable-projection-growth` remain binding.

V1 immediately installs the laws governing the receipt graph, exact apply image,
lossless receipt encoding, literal-byte signature, projection separation, one rich overlay,
reverse overlay, verify-before-use, key-role separation, fixed algorithms, no clear fallback,
partial/complete separation, one grammar, no compression, no append, and report-only read-back.
Production publication, archive, profile expansion, non-retroactive config, and propagation
wording remain prospective until their later surfaces are built; their shapes must not be
foreclosed by v1.

- **`sinv-receipt-event-graph-is-typed`** - `PlanReceipt`, `ApplyIntent`, and
  `ApplyOutcome` are distinct top-level species with exact typed edges. Missing edges remain
  explicit; no one-file run abstraction or mutable history may impersonate them.
- **`sinv-apply-intent-binds-exact-bytes`** - the pre-dispatch intent binds exact ordered
  executable bytes and complete file/stream topology, target/context, policy, controller semantics,
  tunnel identity, and originating plan identity. Digest-only, lossy diff, or best-effort archive
  inputs cannot mint the dispatch permit.
- **`sinv-receipt-encoding-never-rebundles-apply`** - serializing `ApplyImage` may place its entries
  in a lossless receipt container but never changes executable form: no new bundle, flattening,
  relocation, import rewrite, path normalization, deduplication, or source elision. Round-trip
  materialization reproduces every entry, path, root, dependency edge, and byte.
- **`sinv-planning-byte-persistence-never-expands-observation`** - plan receipts project only bytes
  the invocation already acquired. A path or argv value never authorizes a new file read for the
  receipt or optional archive.
- **`sinv-source-archive-is-best-effort-report-data`** - a dislocated content archive may satisfy
  digest lookup for explanation only. Missing, corrupt, unavailable, or cleaned archive objects
  never affect receipt completeness, authority, or action.
- **`sinv-pre-dispatch-receipt-gates-mutation`** - under required policy, only a published
  `ApplyIntent` at the required grade may mint the affine first-mutation dispatch permit. Plan
  publication, TTY, attempted write, or recorded receipt cannot substitute.
- **`sinv-post-dispatch-durable-failure-does-not-abort`** - after the first-dispatch permit is
  consumed, durable-only failure cannot stop coherent orchestration. Transport/execution/attribution
  failures remain separately typed and may abort.
- **`sinv-literal-bytes-are-signed`** - signatures authenticate a domain-separated function of
  the exact immutable serialized bytes later parsed. No semantic reserialization, normalization,
  or second source read enters signature verification.
- **`sinv-projection-mode-is-signed`** - receipt version, species, projection, policy, and complete
  content ordering lie inside the signed domain. Rich-to-plain stripping, species substitution,
  and unsigned fallback always fail.
- **`sinv-plain-opaque-region-is-unrepresentable`** - the trusted plain writer type cannot carry
  an overlay or opaque field, and the runtime plain grammar rejects both. Compile-fail and hostile-byte
  tests own the two independent halves.
- **`sinv-rich-has-exactly-one-overlay`** - rich receipts contain exactly one complete Age envelope;
  absent, duplicate, trailing, or extra compartments refuse completeness.
- **`sinv-readable-skeleton-never-references-opaque`** - no readable field contains an opaque member
  id, offset, path, fetch instruction, or blob-selected key. Only the authenticated decrypted overlay
  may name structural slots.
- **`sinv-reverse-overlay-is-bijective`** - overlay-to-skeleton keys are unique, in-document,
  field-class-valid, and exactly account for required captured opaque slots in both directions.
  Any mismatch releases no opaque values.
- **`sinv-verify-before-interpret-or-render`** - before signature verification, only the bounded
  lexical region locator runs. Before complete overlay authentication/validation, no opaque value
  enters semantic interpretation or rendering.
- **`sinv-signing-and-encryption-keys-never-alias`** - signing and Age keys are independently
  generated, stored, rotated, and typed; no conversion, derivation, shared raw root, or cross-use
  exists.
- **`sinv-file-key-id-never-selects-authority`** - receipt key ids are checked against controller
  policy and may aid lookup; they never choose a plugin/backend, grant signer trust, or authorize
  key acquisition.
- **`sinv-no-in-file-algorithm-negotiation`** - every algorithm and encoding is fixed by exact
  receipt version. Unknown versions refuse through a separate reader; file content never selects
  cryptographic implementation.
- **`sinv-no-clear-opaque-fallback`** - encryption/key failure may refuse or remint through the
  typed signed plain projection according to policy; opaque bytes never appear clear as a fallback.
- **`sinv-partial-never-becomes-complete`** - truncated, unknown-version, signature-failed,
  unavailable-key, and overlay-mismatched receipts remain distinct partial/report types with no
  permissive conversion to complete/authenticated.
- **`sinv-policy-expansion-is-closed-and-visible`** - convenience and hardening actions expand into
  a versioned closed setting set, synchronously disclose each change/reversal, and never absorb
  future settings automatically.
- **`sinv-policy-is-recorded-not-retroactive`** - the effective individual settings and their
  expansion provenance enter decision/receipt identity; later configuration never rewrites or
  reinterprets old receipts.
- **`sinv-private-key-propagation-is-honest`** - documentation and diagnostics distinguish state
  and key propagation. No pathname choice, OS store, backup, or hardware claim implies protection
  against the authorized account or a copy containing both receipt and private key.
- **`sinv-valid-signature-does-not-mean-true`** - signature status is controller-byte authorship
  only. Host influence, authored claims, recorded/re-derived disagreement, and signer trust remain
  independently represented and rendered.
- **`sinv-one-canonical-receipt-grammar`** - one exact writer form and one strict reader form exist
  per version. No permissive parser, comments, alternate whitespace/newline, unknown field recovery,
  or second configurable physical format may enter v1.
- **`sinv-no-compression-in-receipts`** - neither skeleton, opaque plaintext, nor ciphertext uses
  compression in v1; hostile and opaque values never share a receipt compression context.
- **`sinv-no-append-in-v1`** - receipt publication is immutable. No append or mutable completion path
  appears without a separate reviewed incremental-journal design.

## implementation-sequence-sketch

The implementation-planning conductor owns exact scheduling against the live tree. Preserve this
v1 dependency order:

1. Create the standalone receipt crate and its three species, projection, graph-id,
   completeness, signer-trust, and recorded/live type boundaries.
2. Implement strict skeleton serialization plus bounded lexical/semantic parsing and signed
   plain round trips with injected fixture keys.
3. Implement exact `ApplyImage` single-stream and multi-file archive round trips, preserving
   topology and bytes.
4. Implement the reverse-overlay model and complete validation over inert bytes.
5. Integrate maintained Age and Ed25519 libraries behind narrow injected capabilities and
   round-trip one rich fixture.
6. Project the smallest current plan values needed by one plan/why e2e route.
7. Thread the affine dispatch/failure-direction state through one DST apply route that emits
   intent plus outcome or explicit no-outcome.
8. Correlate all three recorded species in `dorc why` and prove the two full routes.
9. Build the minimal production key provider/store and prove real-binary restart routes exactly as
   specified by `30Rd`.
10. Delete the old format, reader, writer, fixtures, compatibility paths, and one-file assumptions;
   the arc ends with exactly one live receipt implementation.

Do not build alternative key discovery/providers, profile/config surfaces, full store
hardening, retention, source archive, every Spine species, complete executor/multi-host
coverage, or pre-publication polish in v1. Do not emit a partially built rich format
outside tests until its complete sign/encrypt/read path is present.

The later/pre-publication build-out, in rough dependency order, is:

1. alternative key providers, production hardening beyond the `30Rd` baseline, and rotation;
2. purpose-driven PlanReceipt species and bounded source excerpts;
3. full apply intent/outcome integration and mutation gate;
4. individual policies followed by convenience/hardening expansions;
5. retention, cleanup, trust import/export, rotation, and filename policy;
6. broader platform/multi-host/executor routes.

## verification-and-supporting-infrastructure

### verification-required-now

Build one small committed conformance corpus independent of spike internals:

- one valid plain and rich example covering all three receipt species;
- representative strict-grammar refusals, truncation, signature failure, and projection substitution;
- representative overlay missing/extra/duplicate/cross-document failures;
- single-stream and multi-file `ApplyImage` round-trip topology/byte identity;
- standard library vectors sufficient to prove the selected Age/Ed25519 APIs are being used correctly;
- one small mutation sweep over a complete fixture; and
- injected receipt-sink success/failure on both sides of first dispatch.

The two product-spanning plan/why and apply/why DST routes plus `30Rd`'s concrete local-provider/
store process-restart routes are the v1 acceptance tests. Keep fixture keys and nondeterminism at
the receipt edge, and keep fixture selection structurally out of the production path. Do not build
the full native macOS/power-loss filesystem matrix, source archive, profile, rotation, or multi-host
test families now.

### verification-deferred-until-build-out

Before publication to real users, grow the corpus to every byte boundary and every independent
byte/count/depth/allocation limit; fuzz lexical, skeleton, armor, overlay, and graph parsers;
differential-test Age against its reference CLI; add key-era/rotation, filename, source archive,
profile, retention, and full filesystem crash matrices; and run native Windows, Unix, and macOS legs.
An independent tiny recognizer is useful at that stage, not a v1 requirement.

### kani-verifies-bounded-kernels

Kani is well suited to bounded pure kernels here. Add narrow harnesses for:

- lexical state-machine progress and no panic;
- offset/length checked arithmetic;
- plain/rich automaton disjointness;
- overlay-key uniqueness and exact set equality at small populations;
- partial-state inability to call complete-only APIs; and
- one-time `MutationDispatchPermit` consumption.

Harnesses must assert non-vacuous reachability and use exact small bounds. Do not claim coverage
beyond those bounds. V1 needs only the harnesses the implementation conductor judges cheapest and
most useful for the parser/overlay/dispatch architecture; the complete list is pre-publication work.

### flux-later-if-stable

Flux may be useful defense in depth once the format and parser settle, especially for span intervals,
declared-versus-present length relationships, allocation budgets, and offset monotonicity. Do not make
v1 depend on adopting an unstable verification toolchain. Reassess after ordinary types, checked
arithmetic, fuzzing, and Kani expose the actual residual burden.

### lean-and-aeneas-not-owed

Lean/Aeneas adds little at this boundary relative to its cost. Cryptographic correctness belongs to
the maintained Age and Ed25519 implementations; outer parser risk is dominated by byte handling,
allocation, platform I/O, and glue. Keep the verified-core tools focused on the algebra they already
own. Reopen only if a small pure minispec emerges whose proof would remove an actual implementation
choice rather than restate the grammar.

### dependency-and-vendoring-practice

Treat encryption, signature, and parser dependencies as security-critical:

- committed lockfile and no automatic upgrades;
- publication-age quarantine where registry tooling permits;
- `cargo vet`/equivalent named criteria and explicit ownership;
- diff every update, including feature/default changes;
- disable build scripts, proc macros, plugins, remote key backends, and unused algorithms where
  possible;
- record exact source revision and advisory coverage;
- keep each behind a narrow project-owned value API that does not reconstruct framing or crypto;
  and
- test the wrapper seam more heavily than the primitive.

These dependency-governance practices are pre-publication targets. V1 must at least pin exact package
versions/features, disable unused plugin/remote surfaces, and keep the adapters narrow; broader audit
process and an independent recognizer may follow after the architecture proves useful.

# rationale-alternatives-and-limitations

## rationale-readable-literal-byte-envelope

Direct readability is not aesthetic. The receipt exists for degraded conditions: source drift,
old binaries, vendor handoff, issue discussion, partial damage, and distrust of Dorc's own rendering.
Binary plus a renderer changes the product into tool-mediated inspectability.

Turn 12's corrective prototype demonstrated that an age-shaped strict grammar can round-trip
byte-identically, reject alternate forms, detect every tested mutation/truncation, carry arbitrary
opaque bytes under encryption, and remain useful under `grep`, `diff`, and `less`. Its most important
finding was not text versus binary but literal-byte authentication versus a re-derived semantic model.

SOPS is the negative exemplar: path-derived AAD and a MAC over reconstructed values creates writer/
reader disagreement and structural gaps [A-sops-encryption-protocol-reference-2026]
[B-sops-mac-mismatch-sequence-comment-2026]. Age's header MAC and DSSE's same-serialized-body rule
show the opposite discipline [A-age-v1-grammar-header-mac-2026]
[A-dsse-protocol-spec-2024].

Text still adds a human parser. Authenticated bytes can be displayed differently from machine
interpretation, as OpenPGP clearsigning and parser-differential research demonstrate
[A-gpgfail-notdashescaped-forgery-2025]
[A-ali-smith-parser-differential-antipatterns-2023]. The containment is a closed structural alphabet,
no raw opaque text, exact signature spans, strict sink encoding, and rendering only authenticated
semantic fields.

## rationale-grouped-reverse-overlay

Inline ciphertext has almost no readable utility. The field name and capture-status marker carry the
human value; randomized ciphertext communicates presence and approximate length while interrupting
the document.

Turn 13 found grouping's concrete gains:

- aggregate rather than per-field length/change leakage;
- one nonce/stream owned by a maintained package;
- one padding decision;
- one safe contiguous skeleton for issue quoting; and
- fewer per-field AAD/framing seams.

Its strongest cost was a readable reference namespace into encrypted objects, the precondition used
by PDFex [A-mueller-pdfex-partial-encryption-2019]. The reverse overlay removes that direction. The
authenticated decrypted compartment enriches already signed structural records; the readable skeleton
never instructs the reader to fetch, select, or decrypt an object.

One compartment concentrates opaque corruption and loses per-field ciphertext diff localization.
Age chunk authentication can localize cryptographic failure, but a failed complete overlay withholds
all opaque values because partial enrichment risks misleading the firefighter. The readable skeleton
and signature status remain useful.

## rationale-signature-over-encryption

Encryption answers who can read opaque values. A public-key signature answers which signing identity
authorized exact bytes. AEAD integrity is insufficient for the plain projection and permits content-key
holders to re-author. An independent outer signature:

- authenticates signed plain output;
- blocks rich-to-plain stripping;
- binds readable skeleton and Age ciphertext;
- separates decryptors from writers; and
- permits verification without decryption.

The composition is deliberately narrow: encrypt the opaque overlay, serialize the complete immutable
outer document, then sign every preceding byte. The expected signing key comes from controller policy,
not the file. Unknown signatures can be cryptographically valid and still untrusted.

The signature does not prove truth, secret absence, or intended recipient. It attests controller
authorship under a typed projection. Host influence and authored claims remain visible. This avoids the
overclaim that caused earlier signing/encryption systems to fail despite sound primitives
[B-filippo-age-authentication-2022].

## rationale-key-location-and-propagation

Private keys must be usable by the authorized controller principal. There is no mechanism by which
Dorc can use a key while resisting an attacker controlling Dorc and the operator account. Hardware
can make bytes non-exportable but still allows an authorized compromised process to request signing
or decryption.

Default config/state separation is idiomatic and helps selective propagation. It does not protect a
whole-home copy containing both. Platform key stores and dislocated mounts may improve specific
propagation paths but introduce headless, SSH, CI, recovery, and portability failures
[A-gcm-credential-stores-2026]
[A-lawrence-chromium-local-data-encryption-2020]. They belong in a guided hardening path, not first-run
requirements.

The project therefore promises separated paths and honest policy, not protection from the authorized
account.

## rationale-plan-intent-outcome-split

The original one-file-per-invocation design missed Dorc's actual temporal structure. Most "why" is
known at `dorc plan`: probes, analysis, vouches, and decisions. Between plan and first mutation, the
new critical fact is what the admin did to the emitted plan and which exact bytes apply adopted.
After dispatch, actual outcomes become available.

Write-ahead intentions establish only that no authority was spent without a record of intent; they
do not prove remote action. Arbitrary shell commands are Gray's real actions: generally not undoable,
restartable, or remotely deduplicated [A-lampson-sturgis-intentions-1979]
[A-gray-transaction-real-actions-1981]. The three typed documents express exactly what each moment can
know without pretending to transactionality Dorc does not own.

Append is excluded because its only substantive gain is partial-outcome survival across controller
crash. That is valuable but is an incremental journal, not a minor file-count optimization. Immutable
documents keep publication, damage, and correlation simpler while the product is young.

## rationale-planning-input-byte-custody

Planning sources and apply bytes have different durability obligations. Exact acquired general-sh
(non-`dorc-lang`) bytes support historical addressing and drift comparison and therefore ride the
rich PlanReceipt; valid `dorc-lang` bytes do not, because their ordered identities/digests usually
recover matching current material without multiplying the durable corpus. Recorded conclusions remain useful when either
class is unavailable, with source-dependent explanation explicitly degraded. The durable projection
of the existing locator DAG preserves byte-level source/generated provenance without creating a
second line-only location model.

Apply bytes are the object of the authority spend. They may arrive as stdin, one generated stream,
multipart plans, unbundled books plus generated bundles, or a transitive `.sh` tree. Their exact bytes
and topology must exist durably before dispatch under required policy. Repackaging them for receipt
storage is valid only as a lossless archive representation; turning an unbundled apply into a bundle
would record a cousin rather than what Dorc authorized.

The distinction also preserves the no-expanded-observation rule: Dorc may archive bytes it already
acquired, never every non-sh file a command happened to name.

## rationale-opposite-failure-directions

Before first mutative dispatch, Dorc can leave the world untouched. A required receipt failure is
immediately actionable for the normally present interactive user and fits Dorc's established
"no, but do this" culture. CI/cron users are secondary and can configure policy explicitly.

After first dispatch, the apply may already be partial and the user may be absent. Stopping for a
local debugging failure cannot restore history and may worsen convergence. The failure-direction
change therefore belongs to an affine authority-spend boundary, not to elapsed time or command
success.

This policy remains configurable because homelabbers, unattended CI, and more regulated teams have
different availability and receipt requirements. Strict teams may be especially likely to demand
crash-surviving pre-dispatch records. TTY never decides the policy.

## rationale-balanced-default-and-guided-profiles

Maximally secure defaults can create non-dominating failures: platform keychains can make headless
or remote operation unreliable, while plain file keys reduce separation. The baseline chooses the
highest generally reachable posture without forcing platform-specific setup.

The convenience and hardening actions preserve Dorc's gradual-enhancement product identity. Users
should not need to discover dozens of obscure settings, and Dorc should not hide the cost of one-click
configuration. Closed expansion, synchronous explanation, individual reversal, and no automatic future
growth make both directions teach rather than surprise.

## alternatives-not-selected

### alternative-deterministic-cbor

EverCBOR/EverCDDL offers serious strict-parser and proof advantages, but choosing it would replace
direct readability with renderer-mediated inspection and import a research-toolchain/bus-factor/panic
surface. Turn 12 showed that byte-identified strict text is feasible. CBOR remains prior art and a
possible future opaque-overlay encoding, not the v1 outer format.

### alternative-inline-field-encryption

SOPS demonstrates the UX and failure burden: per-field nonce/AAD/framing, exact length leakage,
deterministic equality leakage, difficult padding, a separate document MAC, and model-reserialization
bugs [B-getsops-length-leak-issue-2021]
[B-getsops-format-wishlist-issue-2025]. Inline can be done correctly, but grouping removes seams and
loses little readable utility.

### alternative-project-authored-c2sp

C2SP chunked encryption is a strong specification, but no maintained Rust package exposes the whole
construction. Implementing it locally would put machine-authored crypto and context glue in the
project. Age whole-message encryption plus independent signature avoids that risk. C2SP remains useful
for truncation, nonce, commitment, and negative-vector reasoning
[A-c2sp-chunked-encryption-2026].

### alternative-age-without-outer-signature

Age has no caller-controlled AAD, and its header MAC derives from the file key. Without an outer
signature, skeleton/blob binding would be a post-decryption digest check and every decryptor could
re-author. The independent signature makes Age usable substantially as-is while binding the full
artifact and authenticating plain receipts.

### alternative-associated-data-only

A whole-message AEAD with skeleton bytes as AAD is elegant and verifies context before release, but
available maintained Rust choices either do not own a complete file envelope, chain damage in unwanted
ways, lack stable support, or reintroduce custom framing/key management. The outer signature plus inner
digest is a slightly larger two-key design with more available mature components and public
verification.

### alternative-digest-inside-plaintext-only

An inner skeleton digest detects substitution after decryption and can work under the threat model,
but it authenticates neither plain receipts nor the outer readable artifact before opaque release.
It remains defense in depth rather than the primary construction.

### alternative-multiple-formats

Configurable physical formats multiply hostile parsers, damage models, compatibility paths, and
cryptographic joins. Security posture is expressed through projection and policy within one grammar.
Reopen only if the strict readable format proves intractable in implementation.

### alternative-clear-opaque-projection

No clear opaque-value projection ships in v1. It would turn a convenience setting into long-lived
plaintext copies whose users may misunderstand. The closed convenience profile may be widened only
after explicit product review and synchronous cost disclosure.

### alternative-platform-key-store-default

Platform stores can improve selective exfiltration but fail in SSH, headless, CI, locked-session, and
cross-platform recovery modes. They are a guided hardening option, not a prerequisite for first success.

### alternative-database-or-sidecar-index

SQLite/redb and mutable sidecar indexes hide platform gaps, fight immutable independent failure
domains, and remove direct file readability. Bounded enumeration and typed graph correlation do not
need a database at expected scale.

### alternative-forward-secure-sealing

Forward-secure logs or external witnesses could detect retroactive edits after controller compromise,
but that principal is outside the current confidentiality/integrity claim and the machinery requires
independent custody the project does not have. The outer format may reserve a previous-root field later;
no sealing service is built now.

## chosen-path-downsides

The selected design is not low-cost or provably secure.

- Dorc owns a custom strict textual grammar and two parsers (outer skeleton and decrypted overlay).
- The outer signature adds signing-key generation, custody, trust distribution, rotation, and
  verification UX.
- Age brings a substantial Rust dependency graph and inaccessible internal randomness, which must
  stay at the injected I/O edge rather than inside the deterministic kernel.
- ASCII armor adds roughly one third to opaque ciphertext size and preserves aggregate length leakage.
- One grouped overlay loses per-field ciphertext diff localization and withholds all opaque values on
  any complete-overlay validation failure.
- The readable skeleton necessarily leaks its selected structure, record count, timestamps, policy,
  activity, and filename metadata.
- A whole-home copy carrying keys and receipts defeats most confidentiality and signing separation.
- Signed receipts authenticate controller bytes, including controller bugs and host-influenced lies.
- No-append v1 loses partial per-command outcomes on controller crash.
- Exact-byte signatures make newline conversion, sync rewriting, or manual edits invalid by design.
- Long-term verification requires preserving old public keys, format readers, and key-era metadata.
- The implementation will be written and reviewed largely by fallible LLMs and a human who does not
  claim security expertise, without a budget for professional review.

These costs are accepted because the alternatives either hide the same joins, add custom cryptography,
or sacrifice the product's direct-debugging value.

## security-development-posture

This project is an unfavorable environment for bespoke security design: no paid audit, no dedicated
security maintainer, agent-heavy implementation, and a small userbase unlikely to supply broad field
testing. The design responds by reducing novelty rather than claiming assurance:

- mature encryption and signature primitives/packages;
- one small explicit composition;
- literal bytes rather than semantic canonicalization;
- private typed mints and affine transitions;
- no compatibility burden while unreleased;
- aggressive invalid-input and failure testing;
- reference interoperability;
- narrow feature set;
- no optional weak crypto modes; and
- explicit claims and exclusions.

Audit language must remain modest. Tests demonstrate exercised properties, not absence of unknown
vulnerabilities. Formal tools cover bounded pure questions, not the whole filesystem/crypto/product
composition. A later implementation review should rotate models and targets like fuzzing, but no
model vote upgrades evidence to proof.

## research-chain-and-traceability

The design rests on several independently useful chains:

- **literal bytes over re-derived models:** DSSE same-body rule
  [A-dsse-protocol-spec-2024] + SOPS writer/reader mismatch
  [B-sops-mac-mismatch-sequence-comment-2026] + turn-12 prototype;
- **strict readable grammar feasibility:** Age header/armor rules
  [A-age-v1-grammar-header-mac-2026] + RFC 7468 strict grammar
  [A-rfc7468-textual-encodings-noncanonical-2015] + parser-differential research
  [A-ali-smith-parser-differential-antipatterns-2023] + turn-12 prototype;
- **grouping:** SOPS length/equality leakage
  [B-getsops-length-leak-issue-2021] + SOPS format wishlist
  [B-getsops-format-wishlist-issue-2025] + turn-13 seam inventory;
- **reverse overlay:** PDFex cross-object-reference requirement
  [A-mueller-pdfex-partial-encryption-2019] + the product observation that inline ciphertext has no
  readable utility;
- **verify before interpretation:** DSSE ordering
  [A-dsse-background-rationale-2021] + SOPS decrypt-before-MAC
  [A-getsops-tree-walk-source-2026] + EFail best-effort rendering
  [A-poddebniak-efail-exfiltration-2018];
- **whole-document signature:** plain projection downgrade from turn 13 + Age recipient re-authorship
  [B-filippo-age-authentication-2022] + separate signing/encryption semantics;
- **event graph and dispatch boundary:** write-ahead intentions
  [A-lampson-sturgis-intentions-1979] + real-action limits
  [A-gray-transaction-real-actions-1981] + Dorc's plan/apply user workflow;
- **key-provider baseline:** cross-platform store availability
  [A-gcm-credential-stores-2026] + local encryption limits
  [A-lawrence-chromium-local-data-encryption-2020]; and
- **bounded hostile parsing:** LangSec's restrictive grammar
  [A-bratus-curing-vulnerable-parser-2017] + allocation-first decoder failures
  [B-russh-allocation-first-parsing-2026] + current Dorc intake law.

The raw research reports remain in
`.claude/research/secure-durable-praxis-quarantined-DO-NOT-READ/`, especially turns 12 and 13. Their
package measurements are dated selection evidence; their principles and first-party citations carry
longer.

## open-product-and-implementation-questions

`30Rb` settles the V1 receipt grammar/packages/limits and `30Rd` settles the V1
local key/store baseline. The remaining later product questions are:

- trusted-key import/export and signer-fingerprint UX;
- policy/config names and exact convenience/hardening pre-publication expansions;
- stronger Windows ACL implementation and publication grades beyond the V1 baseline;
- signed plain fallback defaults for plan and apply;
- retention/store-budget defaults and explicit cleanup UX;
- padding policy for the grouped overlay;
- exact bounded excerpt budgets and optional source-archive design;
- how much bounded unauthenticated structure `dorc why` renders after signature failure;
- whether an independent recognizer ships before publication or later; and
- key rotation, recovery, alternative providers, custom key roots, and native macOS validation.

None may reopen the ruled topology silently. A contradiction discovered during implementation returns
here for review rather than being solved by a local compatibility path, unsigned mode, inline ciphertext,
or custom cryptographic primitive.
