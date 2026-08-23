# 30Ra - The whylog under persistence pressure: primary review and direction

> Tier: quarantined primary review, GPT-5.6-Sol, from the 2026-08-23 human
> sitting. The quarantine was explicitly opened for this work. This document is
> the full rationale-bearing sibling of `plans/30R`; `30R` is the ordinary
> engineering surface and must remain sufficient for implementation. This file
> explains the constraints that would otherwise look removable. It does not
> authorize implementation by itself.
>
> Scope: the one `.whylog` file, its filesystem store, the information projected
> from r30's Spine, `dorc why` as a firefighter's reconstruction tool, and every
> read/replay/export path over the durable. Code-level nits that a later review can
> rediscover are deliberately footnoted rather than promoted into the design.
>
> This document is the full version of `plans/30R`; and must be kept in-sync. If
> one is modified, the other should not contradict it. 30R is sanitized for
> Fable-class condutors, kept clear of memetic-hazards / security-flavoured
> prose and rationale - any explicit mention of threats, attackers, hardening,
> and so on.
>
> Authority: root human docs and human-typed rulings outrank this review.
> `plans/30R` and this document are the authoritative conceptual homes for the
> resulting design. Grades: **[TYPED]** the human typed the direction;
> **[ACKED]** the human accepted the stated substance; **[DIRECTIONAL]** a strong
> human lean remains intentionally unwelded; **[REVIEW]** this review's conclusion.

## 30Ra:the-result-in-one-screen

The final whylog should be an aggressively informative, immutable receipt of one
invocation, not a cache and not a saved authority object. Almost every stable
semantic object added to Spine in r30 belongs in a durable view. Withholding
structured conclusions while persisting the admitted record stream keeps the
harder-to-interpret and often more-sensitive material and throws away the context
that makes it debuggable. That is the wrong resting point.

The correction is not "persist everything blindly." It is:

1. one readable file format with two distinct projections;
2. a rich projection that encrypts opaque-value-capable fields while leaving the
   structural record readable;
3. a plaintext projection, selected explicitly or on encryption failure, whose
   type cannot contain rich-only fields;
4. stable recorded forms for decisions, narratives, locators, and influence, with
   no conversion into live authority;
5. reconstruction from source paths and digests rather than default source copies;
6. exact, bounded persistence of bytes Dorc already transported under a contracted
   channel, while unaccounted stdout/stderr remains on the host by default;
7. one immutable whylog per invocation, with correlation recorded between files
   rather than later invocations updating old files; and
8. no automatic historical cleanup during an unrelated invocation.

The engine is referentially agnostic. It cannot identify a bearer token embedded
in argv, a password hard-coded in book text, or a private value printed into an
oracle report. Encryption and field selection reduce exposure. They do not make a
whylog "scrubbed", "secret-free", or safe to share. The product must say this
loudly. A false promise would cause admins to stop applying the only generally
effective control available: knowing what they put into debugging material and
reviewing a plaintext export before sharing it.

## 30Ra:current-r30-ground-truth

The review targets `ai/r30-conduct` at `b4e11a91`, not the project-root checkout.
At that tip:

- `core::spine` has sixteen species. Four have durable views: invocation,
  admitted record stream, per-site disposition, and digest. Twelve remain in the
  `New` census arm. Four of those twelve have no writer yet.
- Influence carriage is landed as architecture only. Every stable semantic
  object carries an immutable `InfluenceAccount`; no decision consumer exists at
  v0. The object-global stamp is gone.
- The disposition-account durable export is implemented end-to-end behind
  `plan::whylog::ACCOUNT_EXPORT = false`. Read-back is report-only and has no
  conversion to a live account, claim, license, Spine record, or plan authority.
- The whylog already persists full argv, source paths and digests, host identity,
  controller timestamps, predicted dispositions, decision digest, and the exact
  admitted host/report record bytes.
- The store is default-on, keeps 64 files, writes unique predictable names by
  exclusive create, and prunes automatically. That retention behavior is now
  contrary to the direction in this sitting.

The account export is cleared by this review. **`30Ra:rul-enable-influence-export`**
**[ACKED]**: enable the built export when this review is folded. It is not the final
shape: every durable semantic record eventually carries a recorded influence value,
and a final recorded type should preserve the distinction between a recorded
`host-influenced` value and a recorded `untracked` value without exposing a live
account constructor.

## 30Ra:mirrored-builder-invariants

Each `sinv-*` rule below mirrors one newly-minted `inv-*` rule in steering. The
ordinary rule states what a careful builder must preserve. This section supplies
the literal reason it must not be simplified away.

- **`sinv-debugging-detail-has-no-sensitivity-guarantee`** mirrors
  `inv-debugging-detail-has-no-sensitivity-guarantee`. Dorc cannot recognize
  arbitrary sensitive values in argv, paths, entity names, report tails, source,
  errors, or transformed values. Encryption, omission, and sink encoding narrow
  exposure but cannot justify any generic "safe to share" or "secrets scrubbed"
  statement. The absence of such a promise is intentional product pressure on the
  admin to treat all debugging detail as potentially sensitive.
- **`sinv-reingested-material-never-authorizes-action`** mirrors
  `inv-reingested-material-never-authorizes-action`. Durable content describes a
  past world-moment and may have been replaced, truncated, replayed, or selected by
  another principal. Even an `authored-before-contact` value is temporally stale.
  Every read-back type is a recorded/report type with no conversion to claims,
  licenses, live influence accounts, `PlanAuthority`, artifact execution, probing,
  or mutation. This is distinct from influence rehydration and must survive it.
- **`sinv-recorded-and-rederived-remain-distinct`** mirrors
  `inv-recorded-and-rederived-remain-distinct`. Stored conclusions are attacker-
  influenceable evidence about what the old engine said; current conclusions are
  a fresh execution of current semantics. Replacing one with the other lets a
  modified conclusion impersonate computation and lets version drift silently
  rewrite history. Every render preserves recorded-only, rederived-only, both-
  agreeing, and both-disagreeing as four distinct states.
- **`sinv-whylog-collection-never-expands-observation`** mirrors
  `inv-whylog-collection-never-expands-observation`. A persistence feature that
  opens extra host files, captures the process environment, re-reads unrelated
  controller paths, or launches another probe creates a new collection act under
  the cover of debugging. It broadens both sensitivity and authority surfaces.
  The writer may persist only material the invocation already held. Later pull is
  a new consented operation about "now", never recovery of "then".
- **`sinv-unaccounted-output-stays-remote-by-default`** mirrors
  `inv-unaccounted-output-stays-remote-by-default`. Arbitrary stdout/stderr is the
  largest uncontrolled content source and a high-N transport/controller-load
  multiplier. Keeping it on the host preserves a useful containment boundary even
  when the host can influence contracted records. Collection is opt-in and
  front-loaded; debugging pull is separately consented and temporally labelled.

## 30Ra:the-current-exclusion-line

`ExcludedContent` currently names influence, narrative operands/handles, freeform
host output, and working lattice state. None is correct as a categorical semantic
exclusion in the final design:

| Current exclusion | Review disposition | Why |
|---|---|---|
| Influence | Remove after enabling the built export | It is compact causal accounting and already structurally report-only on read-back. |
| Narrative operands / `ProvId` / arena handles | Replace raw representation with stable durable views | Process-local handles are meaningless; the semantic operands and locator edges are essential. |
| Freeform host output | Split by collection contract | Unaccounted stdout/stderr stays remote; already-transported opaque report material may persist only in rich encrypted fields. |
| Working lattice state | Make conditional, not forbidden | Final conclusions and compact round traces persist; bounded deltas may persist on checker anomaly or explicit forensic mode. |

The final categorical exclusions are narrower and representation-focused:

1. live credentials, private keys, bearer values, passphrases, and active
   controller capabilities intentionally known to be such;
2. live authority-bearing Rust values or any deserialization path into them;
3. process-local pointers, descriptors, arena handles, and memory addresses instead
   of their stable semantic projections;
4. bytes that did not pass the applicable total, field, count, nesting,
   decompression, and allocation bounds; and
5. state acquired solely for persistence rather than already held by the invocation.

The first item is a rule about intentional collection, not a claim that Dorc can
detect unknown credentials in arbitrary values. Opaque material may still contain
them. That is why the broad sensitivity disclaimer remains load-bearing.

## 30Ra:what-the-final-durable-keeps

Every MINTED stable Spine species receives a durable view. An unminted species first
gets a truthful writer; its type's representable-but-empty fields do not become a
format promise merely because the enum exists.

| Species / structure | Durable direction |
|---|---|
| Invocation | Producing invocation identity, replay recipe, argv shape/value according to projection, exact ordered source manifest, controller semantic identity, policy/knobs, target/context/generation, and timestamps. |
| Record stream | Exact already-admitted contracted bytes in rich mode, typed admission outcome, correspondence and controller arrival facts. |
| Disposition | Full `SiteId`, stable source locator, recorded disposition, recorded influence, predicted/actual species, and compact mint-time authority receipt. Never serialize a live license. |
| Digest | Collision-resistant decision identity for real boundaries; fixture FNV remains fenced. |
| Load decision | Every relevant load occurrence and binding/withholding answer, source identity, custody, exactness/havoc, and placement consequence. |
| Site classification | Stable class code, verdict lane, invalidator ownership, cell account, and degradation cause. |
| Solve certification | Per-pass result, trip latch, bounded failing indexes and summaries; bounded transition deltas only on anomaly/explicit mode. |
| Vouch | Recorded site/fact, exact oracle/body identity, custody, attachment/suspension, and decline. No live vouch constructor. |
| Probe ship | Site, ship lane, defining source, decline/unresolvable reason, and denoted context. |
| Admission | All three arms, including `Refused`, with exact fault and a report-only bounded raw block where available. |
| Observation | Typed per-site/fact observation, merge/collapse outcome, and references into the exact received records. |
| Validity/effective rounds | Compact round ordinal, proof/erasure references, certification, and cascades; not arbitrary compared-state snapshots. |
| Survival | Complete wall crossings, footprint/backing comparison, resolver/reach inputs, reference-model result, consent, and authority receipt. |
| Render decision | Binding, refusal, neutralisation, defensive emission, import rewrite, placement, artifact form, and fallback reason. |
| Region decision | Region identity, decision, complete keyed/unkeyed route population, and exact contributing proof references. No silent cap. |
| Outcome | Exit/outcome class, refusal status, advisory routing, write outcome, and apply eligibility. |
| Narratives | Stable class, speech act, stable source/decision references, bounded semantic operands, and explicit omission counts. |
| Locator DAG | Stable source, generated, host, and surface loci plus derivation/distribution/dependency edges. Never raw arena ids. |
| Apply report | Actual per-site execution, guard result, status, divergence, cancellation, and quiescence once an executor exists. |
| Projection drops | Every omitted species/field class, count, reason, and whether it is reconstructable. |

Persisting exact source or artifact bytes by value is deliberately NOT in the base
projection. Recorded conclusions must make a historical why useful when reconstruction
fails; otherwise the design would force source-copy persistence as the only way to
answer after drift.

## 30Ra:two-projections-one-file-format

**`30Ra:rul-one-readable-envelope`** **[ACKED]**: the whylog remains one file and
somewhat readable by eye. Whole-file encryption is rejected. Structural record names,
type tags, bounded counts, enums, stable numeric identities, digests, omission markers,
and values whose types cannot carry opaque runtime/source text remain plaintext.
Opaque-value-capable fields are independently encrypted in rich mode.

**`30Ra:rul-rich-and-plain-are-different-projections`** **[ACKED]**: rich-encrypted
and plaintext are separate typed projections over one Spine and one wire grammar. The
plaintext file is not rich output with encryption turned off. It cannot contain fields
classified rich-only. Encryption failure may fall to the plaintext projection only,
with the fallback recorded visibly; it never writes a rich field plaintext.

The plaintext projection is the inspection/share posture. It remains explicitly not
secret-free. Its value is that a careful admin can inspect exactly what exists before
sharing it and that opaque-value-capable fields were never written. Generic redaction
is not part of either projection.

The exact representation and encryption granularity remain open. Per-record, per-field,
and grouped encrypted values each have different metadata leakage, overhead, salvage,
and implementation properties. No CBOR, protobuf, JSON, age, or other format is selected
by this review. A dedicated live research round reads academic and shipped prior art raw
before that choice.

Research criteria:

- bounded streaming decode before allocation;
- exact binary preservation for admitted opaque blocks;
- definite lengths and checked arithmetic;
- deterministic structural encoding independent of encryption randomness;
- field-class-aware encryption without hiding the structural document;
- authenticated association of encrypted bytes with field name, scope, ordinal,
  schema version, and declared length;
- unknown-field behavior that stays report-only and makes incompleteness visible;
- partial-damage reporting without presenting a partial document as complete;
- portable implementations on Windows, macOS, and Unix controllers;
- maintainable dependency and test-vector story; and
- readable inspection tooling for the plaintext structure and decrypted rich view.

## 30Ra:referential-agnostic-sensitivity

The hard problem is not cryptography. It is classification. `argv` is an arbitrary
string vector; source paths, entity values, report tails, and error text are similarly
opaque. A field called `--password` is not reliably identifiable; a value without that
name may be a password. Transformations and encodings defeat pattern-based scrubbing.

Consequences:

- The projection classification is by DATA SHAPE, never guessed content. Any string or
  byte field capable of carrying arbitrary authored/host text is opaque-value-capable.
- The rich projection encrypts such fields even when one observed value looks harmless.
- The plaintext projection omits the field or emits only structural metadata such as
  presence, count, length, stable source id, and a carefully chosen commitment.
- Commitments over low-entropy values can themselves enable guessing. The format research
  must distinguish public content digests from keyed commitments used only for local
  correlation.
- No documentation may claim that encryption compensates for careless handling. It is a
  containment layer, not the ecosystem's security argument.

## 30Ra:raw-output-and-debug-pull

**`30Ra:rul-unaccounted-output-defaults-to-no-transport`** **[DIRECTIONAL]**:
arbitrary stdout/stderr not accounted for by an oracle stays on the host by default.
This simultaneously narrows what a partially compromised host can send, keeps the
controller and SSH tunnel lighter at high host counts, avoids host-side durable growth,
and declines to duplicate an organization's observability policy.

Contracted result/report bytes that Dorc already transported are different. They are
bounded and closed-parsed where recognized; unknown/freeform tails remain opaque.
Rich mode may persist the exact admitted block under encryption. Plain mode does not.

Debug pull is an explicit later operation. It answers about the world at pull time or
about retained host-side residue, never about the historical run's exact moment. It
cannot silently escalate context, deploy credentials, or be rendered as contemporaneous
with the original decision.

## 30Ra:source-reconstruction-and-copying

**`30Ra:rul-reconstruction-is-the-default`** **[DIRECTIONAL, strong human
suspicion]**: source bytes remain out of the base whylog. Persist ordered paths,
content identities, stable loci, and enough recorded semantic conclusions for useful
historical explanation. Replay re-reads a bounded regular file, checks identity, and
then labels the result current-matching, drifted, absent, or unreadable.

The objection to by-value source is not merely size. Books and oracles can hard-code
credentials. Copying them to a hidden state directory creates a new long-lived location
the user did not expect and may sync, back up, or attach. Deduplication by content hash
reduces disk use while making the hidden lifetime less intuitive, not more.

A dislocated content-addressed source store remains a researchable middle ground, never
an implicit extension of whylog writing. It would need explicit enablement, inventory,
inspection, and explicit cleanup. The whylog would reference it by content identity and
remain useful without it. No default is accepted here.

Current replay's durable-supplied path read deserves replacement even under
reconstruction-first: a path in an untrusted whylog should not freely nominate an
arbitrary local file. The ordinary engineering design may require explicit user
confirmation, confinement to invocation-recorded roots, or controller-side source
selection before reading. Exact mechanism remains implementation work.

## 30Ra:invocation-boundaries-and-correlation

**`30Ra:rul-one-whylog-per-invocation`** **[DIRECTIONAL]**: each invocation writes
one immutable whylog. A later `apply` does not update a prior `plan` receipt. Mutation
would complicate atomic publication, custody, recorded-versus-rederived meaning, and
concurrent use.

Cross-invocation explanation is a correlation problem. Each whylog carries an invocation
identity and explicit references to known predecessors/successors through decision and
artifact identities. `dorc why` may discover and join those files into one explanation.
Correlation is narration, never continuity of authority, host attempt, generation, or
world freshness.

One invocation may cover many targets. Every target-derived record carries explicit
target/attempt/generation/exchange scope; no ambient host exists. Cross-target conclusions
are their own record species with the exact source scopes listed.

The project's not-yet-coherent "user event" may later become a correlation id. It must
not force mutable multi-invocation whylogs now.

## 30Ra:retention-and-cleanup

**`30Ra:rul-cleanup-is-explicit`** **[DIRECTIONAL, strong human lean]**: a Dorc
invocation does not delete historical whylogs while the user is pursuing another goal.
Dorc is run-once, not a daemon. The current keep-64 pruning is removed when this design
lands.

Each file remains independently bounded before creation and on read. Store inspection
may report accumulated bytes/counts. Deletion is an explicit pull operation such as
`dorc clean`, with a preview and user-selected scope. Failure is visible. The core design
does not require age-based cleanup, background work, or a mutable index.

## 30Ra:storage-and-publication

The store must work equivalently on Windows, macOS, and Unix controllers. Unix modes are
not the architecture; Windows inherited ACLs are not the architecture. The architecture
is an exclusively owned per-user store and one cross-platform publication abstraction
with platform-specific implementations and the same tests.

Required final properties:

- use a trusted per-user directory, opened and retained as an ownership-bearing handle;
- create a private temporary/unnamed object exclusively in that directory;
- stream the bounded document, authenticate encrypted fields, flush and synchronize it;
- publish one immutable final name atomically without replacement;
- synchronize the directory where the platform's durability contract requires it;
- make partial/uncommitted files invisible to `why --last`;
- enumerate to a bound plus one and refuse/report overflow rather than truncating an
  unordered directory walk;
- never follow symlink/reparse replacement through create, publish, enumerate, read, or
  explicit cleanup;
- remove only an object whose ownership/identity the store established;
- report write, synchronization, enumeration, and explicit-clean failures; and
- keep filesystem, clock, and randomness at injected edges for DST.

No mutable `latest` pointer is necessary. `why --last` can select among committed
immutable names after a bounded complete enumeration.

## 30Ra:reingestion-and-firefighter-why

The whylog is most valuable when reconstruction is imperfect. The firefighter needs the
historical engine's answer, its exact supporting records, and the current engine's best
attempt side by side. Making successful source reconstruction a prerequisite to every
answer defeats the purpose of persisting enriched Spine records.

Every recorded conclusion therefore has a stable recorded type. Replay may produce a
separate current conclusion. The renderer exposes exactly:

1. recorded only;
2. rederived only;
3. recorded and rederived, agreeing; or
4. recorded and rederived, disagreeing.

Different controller semantics/version is a first-class comparison input. It is not
corruption. A missing decryption key, absent rich field, source mismatch, unsupported
record species, or damaged field each contributes an explicit incompleteness record and
does not silently remove the historical conclusion.

Read-back parsing remains byte-first and bounded. Unknown fields/records are retained as
opaque report material where the representation permits, never interned into coordinates,
paths, source, templates, or claims. The plaintext and rich projection both feed the same
report-only model; neither is a fallback route to plan/apply.

## 30Ra:format-and-identity-separation

Three identities must never collapse:

- the decision identity: exact ordered source identities, analysis policy, controller
  semantics, target/context/generation, and exact executable artifact bytes;
- the whylog document identity: exact bytes/fields of this one receipt; and
- the current-world freshness observations: what remains true now.

Current fixture FNV and width-one identities remain fenced. A collision-resistant
algorithm is required before saved approval, concurrency, multi-host caching, default
public durability, or external consumers. Human render and executable artifact derive
from one identified decision object; the whylog records it but never becomes it.

Writer authentication and cross-file append history remain open format-research topics.
Local authentication does not protect against a controller account that also controls the
key. A previous-root field and optional independently-held witness receipt are cheap format
reservations, but no external service is required by this plan.

## 30Ra:implementation-sequence

1. Fold this review and enable the already-built influence export.
2. Research the representation and per-field encryption mechanism from primary and
   shipped prior art; do not choose CBOR or another format from this review alone.
3. Re-cut the pre-user whylog grammar in place; no compatibility adapter.
4. Define rich and plaintext projections as separate types over one reviewed field
   classification. Make rich-to-plaintext fallback impossible except by reminting through
   the plaintext projection.
5. Harden publication and bounded reading before broad content growth.
6. Add durable views for every truthfully minted Spine species, then stable narrative and
   locator projections. Add writers for currently unminted species only when their
   semantics are firm.
7. Add refusal receipts and report-only partial ingestion without adding plan authority.
8. Add cross-invocation correlation and multi-target scope before reactive/multi-host work.
9. Add explicit inspection/export and explicit cleanup surfaces.
10. Run a fresh focused review over the merged representation, storage, and reader.

## 30Ra:acceptance-obligations

- boundary minus one / at / plus one for every byte, line, field, count, nesting,
  encrypted-value, and retained-allocation limit;
- wrong key, missing key, malformed encrypted field, field substitution, duplicate field,
  reordered field, partial final write, and unknown schema member;
- encryption failure proves rich-only fields never occur plaintext;
- source drift/absence still produces the recorded historical explanation;
- recorded and rederived disagreement stays visible at every render depth;
- compile-fail/lexical fences prove recorded values cannot reach live authority types;
- forged host/target/attempt/generation and cross-target swaps remain unattributed report
  material;
- symlink, junction/reparse, ancestor replacement, concurrent publish, final-name
  collision, crash before/after synchronization, directory overflow, and explicit-clean
  failure on all supported controller families;
- raw control bytes and malformed text pass every destination-specific encoder;
- no whylog write performs a new host call, environment sweep, or unrelated source read;
- unaccounted stdout/stderr is absent from default transport and both projections; and
- both admin and oracle-author workflows, both phases, and reliable/unreliable oracle
  cells are exercised.

## 30Ra:attendant-code-findings

These are review evidence, not durable design. A later implementation review should
rediscover/fix them against the chosen architecture rather than preserve their current
spellings:

- the current final filename becomes visible before writing is complete;
- `flush()` is not crash synchronization, and the containing directory is not synced;
- directory enumeration truncates before sorting and silently ignores errors;
- automatic prune ignores deletion failures and operates through pathnames;
- current trusted-directory checking does not retain a directory handle through later
  operations;
- current replay performs metadata-then-open on a durable-nominated local source path;
- current disposition view collapses `SiteId` to leaf and the live license to a tag;
- current account export covers dispositions only and conservatively flattens a recorded
  host-influenced token to untracked on read; and
- current FNV and width-one identities remain fixture-grade.

## 30Ra:prior-art-read

Primary sources read or gathered for the review:

- [A-owasp-logging-guidance-2026] application logs require purpose-driven collection,
  explicit field definitions/lengths, untrusted-source handling, sink encoding, strict
  access, resource-exhaustion tests, visible failure behavior, and deliberate disposal.
- [A-rfc3227-evidence-2002] collection precedes analysis; evidence should be accurate,
  complete, reproducible, minimally changed, access-restricted, and custody-accounted.
- [A-rfc5116-aead-2008] authenticated encryption requires unique nonces per key,
  unambiguous associated-data construction, bounded inputs, and all-or-fail decryption.
- [A-rfc8949-cbor-2020] CBOR supplies binary values, streaming and deterministic forms,
  but separates well-formed, valid, and application-expected input; it is a research
  candidate, not a decision.
- [A-nist-storage-encryption-2007] storage encryption choice depends on data, location,
  and operational environment; whole-disk, volume, and file/folder mechanisms provide
  different properties.
- [A-w3c-provenance-model-2013] provenance distinguishes entities, activities, agents,
  derivations, responsibility, collections, and provenance-of-provenance.
- [A-slsa-provenance-v1-2023] separate external inputs, internal parameters, resolved
  dependencies, invocation identity, and debugging byproducts that are hard to reproduce.
- [A-rfc9162-transparency-2021] inclusion and append-only consistency are different
  properties; inconsistent views require independent comparison/witnessing.
- [A-age-file-encryption-2026] age is a candidate for per-field/group encryption study:
  per-file keys, multiple recipients, chunked authenticated streaming, and hardware
  recipients. Whole-file age wrapping is rejected by the human direction here.

## 30Ra:handoff

`plans/30R` carries the complete implementation-facing design without this rationale.
Builders receive `30R` plus the short `inv-*` steering rules. A builder that finds the
ordinary rules inexplicably expensive does not simplify them locally; the matching
`sinv-*` rule here is the review input.
