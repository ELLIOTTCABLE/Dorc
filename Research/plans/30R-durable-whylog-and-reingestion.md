# 30R - Durable receipt family

> Tier: conductor-facing project design. Full specification, constraints,
> alternatives, and rationale live in
> `Research/quarantine-DO-NOT-READ/30Ra-durable-whylog-security-review.md`.
> The current-tree schedule is quarantine `30Rb`; the default local durable edge is
> specified only by quarantine `30Rd`. Do not follow those pointers unless explicitly
> authorized. This document is sufficient for ordinary product planning, but it is
> not an implementation brief for the local durable edge.
>
> Changes in this region defer to the relevant crate-local `inv-*` laws and
> require the standing opaque-review process before the design or implementation
> settles. The project is unreleased; replace the current whylog format in place
> and carry no compatibility adapter or dead implementation.

## the-result-in-one-screen

Dorc produces a graph of immutable receipts rather than one mutable log:

```text
                              ApplyIntent A1 -> ApplyOutcome O1
                             /
PlanReceipt P -------------- ApplyIntent A2 -> no outcome
                             \
                              ApplyIntent A3 -> ApplyOutcome O3
```

This is one readable slice. The actual report-only relation is M:N: one presented
plan can feed many apply assignments, and one assignment can cite zero, one, or many
originating plans under admin control. An intent holds a non-empty ordered assignment
set; an intent has zero or one outcome.

`PlanReceipt` is the primary whylog: probing, analysis, vouches, decisions, and
the emitted plan. `ApplyIntent` records the exact admin-adopted assignment images
and contexts immediately before first mutative dispatch. `ApplyOutcome` records actual
execution on every graceful terminal path. Missing graph edges remain explicit and
imply nothing about unrecorded outcomes.

All three use one strict, directly readable, versioned textual envelope. Plain
receipts contain only the structural projection. Rich receipts add exactly one
grouped detail region, physically separate from the readable skeleton and mapped
back to structural slots only from the detail side. Both forms carry one required
exact-byte document-validation footer. Receipts remain report-only and never feed
planning, probing, mutation, saved approval, or caching.

## v1-spike-scope-and-exit

V1 proves the high-lock architecture in a standalone receipt crate, a sibling
holding its algorithm-provider implementations, and two product-spanning DST
routes. The split is a dependency-graph obligation, not taste: the provider
implementations carry nondeterministic dependencies, and the analyzer kernel's
graph stays clean of those. Every authority mint stays in the pure crate. It
includes:

- the three receipt species and graph identities;
- strict plain and rich format round trips through the selected libraries;
- one grouped reverse detail region;
- exact single-stream and multi-file `ApplyArtifactImage` round trips;
- projection, validation, completeness, and recorded/live typestates;
- injected algorithm/provider and receipt source/sink interfaces;
- one default local-file provider and immutable per-user receipt store, plus an
  explicit admin-selected receipt-store folder, sufficient for the shipped binary
  to persist and reopen the new format;
- the smallest current plan values needed by one plan/why route;
- the dispatch-state seam exercised under DST; and
- plan/why plus apply/why e2e coverage.

V1 does not include alternative provider/configuration surfaces, convenience or
hardening actions, rotation/import/export, custom key roots, full store hardening,
retention, source archive, every semantic species, complete executor/multi-host
wiring, or the deeper pre-publication verification programme.

The arc ends with the old format, reader, writer, fixtures, compatibility paths,
and one-file assumptions entirely removed. Temporary coexistence while constructing
the replacement is not a product state.

## standing-invariants

- receipt read-back is report-only and never authorizes action;
- recorded and re-derived conclusions remain distinct, including disagreement;
- receipt writing persists only material the invocation already holds and performs
  no additional observation;
- output outside the accounted record/report channels remains uncollected by
  default;
- plain/rich projection never implies generic scrubbing or safe sharing;
- document, decision, wire, and current-world identities remain distinct; and
- missing, partial, unavailable, unknown, and damaged material never reads complete.

## ruled-product-shape

The following are human-ruled directions:

- one canonical physical grammar, not a user-selectable format;
- directly readable structural content remains a product goal;
- byte equality is document equality; no semantic reserialization participates in
  identity or document validation;
- rich and plain are distinct typed projections;
- plain cannot contain the rich detail region;
- rich carries one grouped detail region, never inline encoded detail;
- the readable skeleton never points into that region;
- the detail region may enrich structural slots in the reverse direction;
- every valid plain or rich receipt carries the required document-validation footer;
- the two algorithm roles remain distinct;
- no append, compression, clear detail fallback, or in-file algorithm selection in v1;
- later policy changes never rewrite or reinterpret existing receipts; and
- contradictions return to this design rather than producing a local compatibility
  path or weak mode.

## receipt-species-and-correlation

One plan may lead to zero or many apply attempts. Each intent has zero or one
outcome. The three species remain distinct top-level types and validation domains.

`PlanReceipt` eventually keeps the stable semantic account needed to explain the
plan after source drift. V1 projects only the current invocation/source identity,
admitted records, disposition, decision digest, recorded influence, and enough
location/narrative data for its plan/why e2e. Broader species are later enrichment
and do not appear as empty v1 promises.

Rich plan receipts keep the exact bytes of every acquired general-sh source that
was not accepted as `dorc-lang`, once per source identity, alongside path, source
class, digest, length, and order. Valid `dorc-lang` sources keep those identities
and provenance but not their full bytes. These mechanical classes are what the
user-facing “book” (general/possibly-mutative) and “oracle” (mutation-pure by
`dorc-lang` contract) glosses denote. Plain receipts mark general-sh bytes withheld
rather than exposing them. Persistence reads no file merely because source or argv
named it: only bytes already acquired for planning qualify.

Each recorded site carries a receipt projection of the existing source locator DAG,
including its authored/loaded/generated stages, exact byte spans, fan-in, and head.
Process-local source/stage ids become receipt ordinals; no new line-only locator is
introduced. Exact book bytes plus an authored byte span recover the historical physical
line without reconstructing the syntax arena.

`ApplyIntent` binds non-empty ordered admin-owned assignments, each with one exact
by-value apply image: every stream and file that assignment will use, the original
artifact form, entrypoints, roots, complete transitive dependency topology,
target-relative placement, and exact bytes. An image may be
encoded in a deterministic txtar-like receipt container, but receipt creation never
flattens, bundles, relocates, rewrites, or otherwise changes what apply actually uses.
A digest, lossy diff, or best-effort archive reference cannot mint dispatch authority.

`ApplyOutcome` eventually carries actual per-site execution, statuses, divergence,
guard results, cancellation/quiescence, admitted output, unknowns, and durable-write
outcome. V1 carries only the fields produced by its DST/hostsim route, including
intent correlation and explicit no-outcome.

Correlation joins immutable identities for explanation only. It never joins world
freshness, host generations, influence, or authority.

## receipt-rooted-attention-and-cli

The user-facing unit is one selected **root receipt**, plus the causally relevant
receipt-graph closure needed by the question. A disconnected receipt graph never
contributes to that answer. Traversal is automatic rather than an option: a required
missing, unreadable, or disagreeing sibling makes the answer explicitly partial.

The receipt vocabulary has one meaning per spelling:

- `--receipts <folder>` selects the immutable receipt store used for publication
  or graph lookup across plan, apply, and why. Without it Dorc uses the standard
  per-user store.
- `--receipt <file>` selects one explicit root receipt file for `why`; it never
  publishes and never authorizes action.
- `--receipt-id <id>` selects one exact root document from the selected/default
  store.
- `--receipt-last` derives the most recent root from the selected/default store.
  With equal-order documents, graph ancestors collapse beneath terminal members;
  one terminal root is selected, while multiple incomparable roots report
  ambiguity rather than receiving an arbitrary tie-break.
- `--all` controls explanation depth only. It never selects unrelated receipts.

The three root selectors are mutually exclusive. `--receipts` is orthogonal: it
also gives an explicitly named receipt file a bounded place to resolve graph
siblings by typed identity. Starting from a receipt follows only edges relevant to
the requested explanation. For example, an outcome reaches its intent and the
intent's originating plans; selecting one historical plan does not pull every
later apply attempt merely because all share a connected component.

There is no whole-store explanation mode. The implementation may enumerate a
bounded store to find typed reverse edges, but that is graph discovery, not a
user-visible union of histories.

A source address always means the same physical path and line number in the current
and recorded book; Dorc never guesses that a moved line is semantically the same
operation. If that line's exact bytes differ, the address-specific answer refuses
pending an explicit current-versus-recorded selector. It still renders every unrelated
receipt fact it can and shows both source states. If the line bytes match but other book
bytes differ, the recorded answer remains available under a book-drift warning; until a
precise dependency comparison exists, analysis-derived explanation is labeled historical
rather than claimed unaffected. Exact source bytes are never newline-normalized: locator
spans and the existing source-map line table use the acquired byte domain, so CRLF/LF
changes are source changes.

Damaged, partial, or unauthenticated receipts never yield an authenticated explanation,
but aid does not stop merely because trust was lost. Bounded recoverable structure
continues through report-only derivation with the break attached; nothing recovered that
way may authorize action or silently fill a missing graph edge.

Inline encrypted book custody is the V1 placement. A later dislocated/deduplicated
store may replace that placement if designed, without changing these recorded/current
and locator semantics.

## recorded-versus-rederived

Historical conclusions and conclusions reconstructed under current inputs and
semantics never substitute for one another. Every consumer preserves recorded-only,
re-derived-only, both-agreeing, and both-disagreeing states. Unavailable inputs and
controller-version differences remain explicit comparison context.

The do-now durable boundary seals a report-only `RecordedWhyFacts` model from verified/
partial receipt state, rooted graph closure, recorded decisions, exact general-sh source,
and durable locators. It contains no raw receipt authority and no route to plan/apply.
Building the report-only kernel re-derivation that populates the re-derived arms requires a
separate correctness-kernel round and is deliberately deferred; the kernel stays frozen in
this arc. Recorded-only explanation remains the fallback, not a replacement for that target.
The user-facing arrangement/render over the sealed model is ordinary why-surface work and
need not remain in the quarantined durable implementation phase.

## canonical-readable-envelope

The replacement is the `dorc-receipt/1` family. Exact grammar tokens belong to the
implementation plan and committed conformance vectors, under fixed rules:

- ASCII structural syntax and LF newlines only;
- one fixed field and record ordering;
- closed vocabularies and one representation for every scalar;
- no comments, duplicate fields, alternate indentation, tabs, trailing spaces,
  ignored bytes, permissive recovery, or alternate newline forms;
- no arbitrary authored or host text in the readable skeleton;
- plain or rich projection declared inside the validated content;
- one mandatory final validation record followed immediately by EOF; and
- exact refusal of unknown format versions.

Document validation covers a fixed encoding of the exact bytes before the final
record. The exact validated slice is the exact slice later parsed. No normalized or
reconstructed model substitutes.

Before public release the grammar may be recut in place. Once a receipt version is
published, preserve its exact reader rather than widening it into a compatibility
parser.

## plain-and-rich-projections

The plain trusted type cannot represent a rich detail region or detail-value field.
The runtime plain grammar independently rejects rich-only content. Rich-to-plain is
a typed semantic projection that serializes and validates a new document, never a
textual strip operation.

Rich receipts contain exactly one encoded detail region. The skeleton carries only
useful structural facts such as capture status; it contains no detail member ids,
offsets, paths, or fetch instructions.

The decoded region maps structural record identities and closed detail-field tags
to exact bytes. It also carries outer document identity, receipt species,
projection, and literal-skeleton digest. The reader validates the complete
relationship in both directions before releasing any detail value. Missing, extra,
duplicate, dangling, aliased, cross-document, or unknown entries leave the region
unavailable as a whole.

The detail region may use a small binary-safe internal sequence. It is never a
direct user surface. Do not introduce a general schema language where a fixed
tagged byte sequence suffices.

## accounted-and-unaccounted-output

Accounted result/report bytes already transported by Dorc may enter the rich detail
region under independent limits. Their recognized structural account may enter the
readable skeleton. Output outside those channels remains on the host by default. A
later explicit pull is a new observation about a later world and never fills a
historical gap silently.

## fixed-algorithm-envelope

V1 fixes the algorithms, their roles, encodings, and domain labels. Use maintained
implementations through narrow adapters and disable unused optional surfaces.

The document-validation input binds receipt version, species, projection, body
length, readable skeleton, and encoded rich region. Validation status and provider
acceptance remain distinct states. A provider identifier in a receipt aids lookup
under controller policy and never selects an implementation or grants acceptance.

Plain and rich both require the final validation record. Rich additionally repeats
document identity and literal-skeleton digest inside the decoded detail region.
A future algorithm change creates a new receipt version with a separately named
reader.

## provider-and-storage-location

The two algorithm-provider roles are independently generated, stored, rotated, and
typed. Neither derives from the other. Private provider paths never enter receipts.

The V1 baseline uses restrictive private files under the per-user configuration root,
while receipts live under the per-user state root. Explicit organization paths,
dislocated mounts, platform stores, and hardware providers are later guided choices.
Old public verification material remains available for old receipts; old rich-detail
material remains readable only while its matching provider remains available.

V1 keeps injected fixture capabilities for deterministic tests and also supplies one
concrete local provider for the normal binary. Import/export, provider selection,
rotation, and guided setup are later.

## default-and-guided-policy

This is a later/pre-publication product surface, not a v1 obligation.

The V1 baseline selects validated rich receipts, a local private-file provider,
required `ApplyIntent` publication before mutative dispatch, no automatic rich-to-plain
fallback, no clear detail mode, no unrelated-command deletion, and filenames without
host identity or detail values.

A standalone convenience action may expand once into a closed, versioned set of
ordinary lower-friction settings. A standalone interactive hardening action may walk
through platform providers, dislocated material, rich-required policy, stricter
publication grades, provider acceptance, and retention. Both synchronously show each
setting changed, its cost, and its individual reversal. Neither is attachable to
another command or remains an opaque profile bit, and future settings never silently
join an earlier expansion.

TTY presence is an I/O/mode signal only. It never implies user availability, policy
consent, or permission to alter receipt policy.

## read-and-write-state-machines

The design expects private typestates and newtypes rather than caller discipline. At
minimum, keep separate types for receipt species/projection, exact byte regions,
document/graph/provider identities, apply-image entries/topology, structural/detail
fields, provider status, detail-region status, complete/partial receipt, live/recorded
influence, publication grade, and pre-dispatch/mutation-dispatched execution.

The writer moves affinely through draft, serialized, validated, and published states.
Plain owns no detail region; rich owns exactly one. Serialized bytes cannot change
after validation. Per-document validation operations, publication ownership,
decoded-unvalidated detail bytes, and first-dispatch permits are non-cloneable
one-use values; configured provider authority remains reusable.

The reader moves monotonically through bounded bytes, lexical location, document
status, typed skeleton, detail status, complete/partial recorded receipt, and
sink-specific render. Before document validation only a bounded lexical locator runs;
it may look up a bounded provider id among already configured providers and performs
no semantic interpretation, file-selected backend dispatch, source read, host call,
detail decoding, or rendering. Partial never converts to complete.

Recorded receipt types have no conversion into live claims, vouches, influence
accounts, plan authority, probing, artifacts, mutation, or cache input.

## publication-and-dispatch-boundary

Tunnel standup remains pre-dispatch and fail-fast. The apply then prepares and
publishes the exact `ApplyIntent` under active policy before committing to the first
potentially mutative book command.

The mutation-dispatch permit is an affine capability minted only from tunnel-ready
state, prepared intent, policy witness, and either required publication or an
explicit configured bypass. It is consumed by first mutative dispatch. A plan
receipt, attempted write, TTY, durable read-back, or failed required publication
cannot substitute.

After first dispatch commitment, durable-only failure no longer aborts otherwise
coherent orchestration. Transport, execution, target attribution, generation, and
mutation-integrity failures remain separate and retain their own abort behavior.

V1 exercises this state through both an injected receipt sink in DST and the default
local store in its product route. Broader filesystem grades and configuration remain
later. V1 does not append outcomes; a controller crash may lose in-memory results
while the intent remains.

## receipt-store-contract

V1 includes a per-user immutable store with private exclusive creation, explicit
bounds, no-replace publication, named platform-specific synchronization results,
unselectable incomplete content, bounded enumeration, filename/internal-identity
agreement, and injected filesystem/clock/randomness. It adds no automatic cleanup,
custom-root policy, mutable `latest` pointer, or sidecar database.

## target-crate-boundaries

The target dependency direction is:

- `core` owns shared identities and live influence vocabulary;
- Spine and plan/execution code mint stable semantic records at their authoritative
  seats;
- a standalone dependency-light receipt crate owns recorded types, grammar,
  projections, graph identities, capability traits, and parser/writer states;
- a dependency-outward crypto sibling owns the concrete algorithm adapters;
- `plan` projects the minimal plan semantics into `PlanReceipt`;
- execution/orchestration mints `ApplyIntent` and `ApplyOutcome`;
- a local-edge sibling owns the default key/store I/O without entering the analyzer
  dependency graph;
- `cli` owns production assembly, publication policy, and later user-selected
  acceptance; and
- `aid` renders recorded models and never feeds them back to decisions.

The implementation conductor may stage extraction through existing `plan::whylog`,
but no physical-format/provider I/O may leak into the deterministic kernel and no
recorded type may leak into live authority.

## implementation-direction-and-order

The detailed schedule belongs to the implementation-planning conductor. Preserve
this v1 order:

1. Create the standalone receipt crate with the three species, graph identities,
   projection/status/completeness typestates, and recorded/live boundary.
2. Implement strict skeleton bytes and plain round trips with fixture capabilities.
3. Implement exact single-stream and multi-file `ApplyArtifactImage` receipt round trips.
4. Implement reverse detail validation and one rich library round trip.
5. Route minimal current plan values through receipt write/read and `dorc why`.
6. Route intent, hostsim/DST execution, outcome or no-outcome, correlation, and why.
7. Thread the dispatch/failure-direction state through that route.
8. Build and test the default local provider/store through real-binary process-restart
   plan/why and apply/why routes.
9. Delete the old format, parser, writer, fixtures, compatibility paths, and one-file
   assumptions. The arc ends with exactly one live implementation.

Later/pre-publication work adds alternative providers, broader store hardening,
broader semantic species/excerpts, full executor and mutation gating, individual
policy/configuration, guided profiles, retention, source archive, platform breadth,
and final polish.

## verification-direction-and-tools

V1 keeps a small format corpus outside disposable implementation details: valid
examples for all species/projections, representative malformed/truncated/validation/
detail failures, one mutation sweep, exact single/multi-file apply-image round trips,
and injected sink success/failure around first dispatch. The plan/why and apply/why
e2e/DST routes are the acceptance test.

Use standard vectors sufficient to exercise the selected library APIs. Add only the
cheapest useful Kani harnesses for parser arithmetic/state, projection disjointness,
detail equality, or affine dispatch; none is individually mandatory if ordinary
types/tests are stronger for that seat.

Before real users, add exhaustive boundaries, fuzzing, reference differentials,
provider-era and profile tests, full filesystem crash injection, native platform
legs, dependency review process, and an independent recognizer. Flux remains optional
later defense in depth. Lean/Aeneas is not owed absent a small pure specification that
removes a real implementation choice.

## limitations-and-open-items

The selected direction intentionally accepts a custom grammar, provider custody and
acceptance UX, dependency/encoding overhead, aggregate detail-length disclosure,
grouped detail failure, readable activity metadata, loss of partial outcomes on
controller crash, invalidation by newline conversion/manual edits, and weak separation
when receipts and private provider material co-propagate.

The v1 plan must settle exact grammar tokens, library packages/adapters, exact
`ApplyArtifactImage` receipt encoding, minimal current projection fields, fixture
interfaces, the default local provider/store, e2e routes, and old-code deletion. Later
planning owns alternative providers, acceptance UX, policy names, stronger publication
grades, retention, padding, later retuning/widening of the fixed V1 limits, source
excerpts/archive, and unvalidated render limits.

These details may refine but not silently reopen the receipt graph, single readable
grammar, plain/rich split, grouped reverse detail, separate algorithm roles,
literal-byte document validation, or dispatch-boundary failure directions.

## governed-change-process

Every implementing crate receives its relevant `inv-*` rules before its lane starts.
A change to receipt contents, grammar, projection, algorithms/version, provider roles,
recorded/live boundaries, event graph, publication gate, or post-dispatch failure
direction runs opaque review before settlement. Builders do not locally add fallbacks,
compatibility readers, unvalidated modes, inline detail values, additional compartments,
or authority conversions to unblock implementation.
