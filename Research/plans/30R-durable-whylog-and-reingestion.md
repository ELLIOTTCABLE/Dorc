# 30R - Durable receipt family

> Tier: conductor-facing project design. Full specification, constraints,
> alternatives, and rationale live in
> `Research/quarantine-DO-NOT-READ/30Ra-durable-whylog-security-review.md`.
> Do not follow that pointer unless explicitly authorized to access the
> quarantine. This document is sufficient for ordinary planning and construction.
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

`PlanReceipt` is the primary whylog: probing, analysis, vouches, decisions, and
the emitted plan. `ApplyIntent` records the exact admin-adopted apply image and
context immediately before first mutative dispatch. `ApplyOutcome` records actual
execution on every graceful terminal path. Missing graph edges remain explicit and
imply nothing about unrecorded outcomes.

All three use one strict, directly readable, versioned textual envelope. Plain
receipts contain only the structural projection. Rich receipts add exactly one
grouped detail region, physically separate from the readable skeleton and mapped
back to structural slots only from the detail side. Both forms carry one required
exact-byte document-validation footer. Receipts remain report-only and never feed
planning, probing, mutation, saved approval, or caching.

## v1-spike-scope-and-exit

V1 proves the high-lock architecture in one standalone receipt crate and two
product-spanning DST routes. It includes:

- the three receipt species and graph identities;
- strict plain and rich format round trips through the selected libraries;
- one grouped reverse detail region;
- exact single-stream and multi-file `ApplyImage` round trips;
- projection, validation, completeness, and recorded/live typestates;
- injected algorithm/provider and receipt source/sink interfaces;
- the smallest current plan values needed by one plan/why route;
- the dispatch-state seam exercised under DST; and
- plan/why plus apply/why e2e coverage.

V1 does not include production provider/configuration surfaces, convenience or
hardening actions, full store hardening, retention, source archive, every semantic
species, complete executor/multi-host wiring, or the deeper pre-publication
verification programme.

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

Complete planning files are not copied by value by default. Record their path/role,
digest, length, ordering, and stable locations, plus bounded exact explanatory
excerpts in rich receipts. `dorc why` prefers a current digest-matching file, then
an optional configured content archive, then the recorded excerpt, then semantic
records alone. Archive presence never affects receipt validity or action. V1 records
existing identities and may carry one excerpt required by its e2e; general excerpt
and archive policy is later.

`ApplyIntent` binds one exact by-value apply image: every stream and file the apply
will use, the original artifact form, entrypoints, roots, complete transitive
dependency topology, target-relative placement, and exact bytes. The image may be
encoded in a deterministic txtar-like receipt container, but receipt creation never
flattens, bundles, relocates, rewrites, or otherwise changes what apply actually uses.
A digest, lossy diff, or best-effort archive reference cannot mint dispatch authority.

`ApplyOutcome` eventually carries actual per-site execution, statuses, divergence,
guard results, cancellation/quiescence, admitted output, unknowns, and durable-write
outcome. V1 carries only the fields produced by its DST/hostsim route, including
intent correlation and explicit no-outcome.

Correlation joins immutable identities for explanation only. It never joins world
freshness, host generations, influence, or authority.

## recorded-versus-rederived

Historical conclusions and conclusions reconstructed under current inputs and
semantics never substitute for one another. Every consumer preserves recorded-only,
re-derived-only, both-agreeing, and both-disagreeing states. Unavailable inputs and
controller-version differences remain explicit comparison context.

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

The eventual baseline uses restrictive private files under the per-user configuration
root, while receipts live under the per-user state root. Explicit organization paths,
dislocated mounts, platform stores, and hardware providers are later guided choices.
Old public verification material remains available for old receipts; old rich-detail
material remains readable only while its matching provider remains available.

V1 uses injected fixture capabilities. Production generation, custody, import/export,
provider selection, rotation, and guided setup are later.

## default-and-guided-policy

This is a later/pre-publication product surface, not a v1 obligation.

The baseline eventually selects validated rich receipts, private-file providers,
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

V1 exercises this state through an injected receipt sink in its DST route. Production
filesystem grades and configuration enforcement are later. V1 does not append
outcomes; a controller crash may lose in-memory results while the intent remains.

## receipt-store-contract

This is a later product edge. V1 reserves receipt source/sink and publication
result/grade types and uses deterministic in-memory or throwaway implementations.

The eventual per-user immutable store owns private exclusive creation, explicit
bounds, complete no-replace publication, named synchronization grades, unselectable
incomplete content, bounded enumeration, filename/internal-identity agreement, owned
cleanup, user-selected symlinked-root handling, and injected filesystem/clock/randomness.
No mutable `latest` pointer or sidecar database is required.

## target-crate-boundaries

The target dependency direction is:

- `core` owns shared identities and live influence vocabulary;
- Spine and plan/execution code mint stable semantic records at their authoritative
  seats;
- a standalone dependency-light receipt crate owns recorded types, grammar,
  projections, graph identities, algorithm adapters, and parser/writer states;
- `plan` projects the minimal plan semantics into `PlanReceipt`;
- execution/orchestration mints `ApplyIntent` and `ApplyOutcome`;
- `cli` eventually owns I/O, provider/configuration, publication, and user-selected
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
3. Implement exact single-stream and multi-file `ApplyImage` receipt round trips.
4. Implement reverse detail validation and one rich library round trip.
5. Route minimal current plan values through receipt write/read and `dorc why`.
6. Route intent, hostsim/DST execution, outcome or no-outcome, correlation, and why.
7. Thread the dispatch/failure-direction state through that route.
8. Delete the old format, parser, writer, fixtures, compatibility paths, and one-file
   assumptions. The arc ends with exactly one live implementation.

Later/pre-publication work adds production store and providers, broader semantic
species/excerpts, full executor and mutation gating, individual policy/configuration,
guided profiles, retention, source archive, platform breadth, and final polish.

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
`ApplyImage` receipt encoding, minimal current projection fields, fixture interfaces,
e2e routes, and old-code deletion. Later planning owns provider files, acceptance UX,
policy names, publication grades, retention, padding, production limits, source
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
