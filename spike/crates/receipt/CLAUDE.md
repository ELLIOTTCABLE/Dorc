# spike/crates/receipt — CLAUDE.md

Role: the durable receipt family — recorded models, the exact `dorc-receipt/1` grammar,
identities, limits, projections, the reverse-overlay validator, and the reader/writer
states. Read `spike/CLAUDE.md` first; `Research/plans/30R` is the design surface this
implements. Registry discipline: one rule per bullet, slugged; append to the matching
section.

**Comment budget here is near zero, and deliberately so.** Doc-comments state the mechanical
contract — what must hold, what refuses, what a caller may and may not do — and stop there.
Make the code self-documenting instead of explaining it.

## Law — the crate cut

- **the-pure-crate-carries-one-dependency** — `dorc-receipt` depends on `sha2` and nothing
  else. It reads no clock, no environment, no filesystem, and no operating-system
  randomness, which is what lets `dorc-plan` — the last kernel stage — depend on it without
  breaking `inv-determinism`. `sha2` is a pure function of its input bytes and is the ONLY
  edge this arc adds to the kernel's graph. Adding a second is a conductor act.
- **implementations-live-in-the-sibling-crate** — every capability that would break the
  bullet above is a trait in `capability.rs` and an implementation in
  `dorc-receipt-crypto`. That crate depends on this one; this one must never depend on it.
  `tests/crate_boundary.rs` is the lexical gate, and the allow-list of crates permitted to
  name the implementation crate is an explicit list there — adding an entry is the visible
  act, never a local edit.
- **authority-states-mint-only-here** — a verifier answers a boolean, a signer answers
  bytes, an opener answers bounded plaintext. `ReceiptSignatureChecked`, `Receipt`, and
  `Reingested` are constructed in this crate and are unreachable from the implementation
  crate, so an implementation cannot promote its own output. Keep it that way: a capability
  trait that returned a checked state would dissolve the split.

## Law — the format (`30R:canonical-readable-envelope`)

- **byte-equality-is-document-equality** — the reader parses the exact span it checked. No
  path may re-encode, normalize, canonicalize, or re-read a document to produce a second
  copy of its bytes. `format::assemble` and `format::signed_body` are the one place a
  document's bytes come into being.
- **one-writer-form-one-reader-form** — ASCII, LF only, fixed line and field order, closed
  vocabularies, canonical integers, exact-length lowercase hex. No comments, tabs, trailing
  spaces, blank lines, duplicate fields, unknown-field recovery, alternate encodings, or
  bytes between the terminator and the trailer. A permissive parse is not a kindness here;
  it is a second grammar.
- **the-grammar-table-is-the-agreement** — `grammar.rs` maps every `(record kind, field
  key)` to one scalar type and token set, with no wildcard arm. Writer and reader both read
  it, which is what makes them agree by construction. A new kind stops it compiling; a new
  key appends immediately before `account`, which is last on every kind.
- **no-whitespace-splitter** — record lines are consumed one literal space at a time. A
  whitespace splitter would accept runs, tabs, and trailing spaces the format refuses.
- **tokens-are-projected-exhaustively** — a receipt-local enum projected from a source enum
  covers every source variant. Two variants may not merge when they differ in what they ask
  a reader to repair, what they license, or how they were known — split the token set
  instead. `survival.outcome`, `site-classification.class`, and `render-decision.kind` are
  each split for exactly that reason.
- **absent-is-a-token** — a semantically absent scalar carries `absent`. Never an empty
  value, never an omitted field.

## Law — projections (`30R:plain-and-rich-projections`)

- **plain-cannot-represent-a-region** — the plain projection has no region and no
  region-valued field, and the runtime grammar refuses one independently of the type. Both
  halves are load-bearing; neither substitutes for the other.
- **rich-carries-exactly-one-region** — absent, duplicate, trailing, or extra regions refuse
  completeness. The readable skeleton never names a region member, offset, or path; only the
  decrypted region names structural slots, and only in that direction.
- **stored-region-is-lf-only** — the selected implementation writes CRLF armor and this
  format admits LF only, so the adapter normalizes at its seam. Line endings frame the
  payload and are not part of it; the outer signature binds exactly what is stored.
- **rich-to-plain-is-a-remint** — narrowing a rich document to plain parses, projects
  through the typed plain model, serializes a new document, and signs the new bytes. It is
  never a textual strip.

## Law — reading back (`30R:standing-invariants` · `30R:recorded-versus-rederived`)

- **read-back-authorizes-nothing** — every value recovered from a document wears
  `Reingested`. No `Deref`, no `AsRef`, no `Borrow`, no generic `map`, no `into_inner`, and no
  generic accessor of ANY kind: there is deliberately no way to ask a `Reingested<T>` for its
  `T`. Decomposition is per-species inherent methods answering another `Reingested` or a
  report-only scalar. This is structural, not a rule about which types join `RecordedType` — a
  generic accessor would move the seal onto that membership list, which is honour-system, and
  the crate had one until the recorded models made the hole reachable. Nothing read back may
  reach a license, a plan, an artifact, probing, or an apply.
- **comparison-is-not-extraction** — `Reingested<T>` implements `PartialEq`/`Eq`/`Clone` where
  `T` does. Comparing two sealed values, or holding a second handle on one, hands nothing out;
  it is what lets graph correlation tell one document read twice from two documents claiming
  one identity without an accessor existing.
- **a-model-comes-only-from-a-document** — the sole route to `Reingested<RecordedX>` is
  `Reingested<Receipt<X, _, _>>::model()`. `RecordedX::of_records` is public because a
  projection must BUILD one to write it; what it answers is a bare model, never a sealed one.
- **recorded-and-current-stay-four-states** — `RecordedCurrent` preserves recorded-only,
  current-only, both-agreeing, and both-disagreeing. Disagreement is a finding that keeps
  both values; it is never resolved by picking one.
- **an-unreadable-grade-reads-highest** — absent, unknown, malformed, or unverifiable
  influence material reads `MostInfluenced`. Losing this metadata may only ever make a
  reader more careful.

  The floor is decided at ONE seat and that is fenced (`crate_boundary.rs`), because the guarantee
  had to travel with its subject: it used to be an entry in `plan`'s untracked-adapter inventory,
  which walks `plan`'s own sources and therefore cannot see a floor that lives here. The fence's
  subject is the FLOOR POINT rather than the reader — a seat that decides a grade without naming
  the floor is not flooring at all — and the reader (`of_token`) is deliberately not the subject,
  since that name is too common to match without crying wolf
  (`a-fence-matches-identifier-boundaries`).
- **partial-never-becomes-complete** — there is no conversion from `PartialReceipt` to
  `Receipt`, and no field of a partial document is promoted because it looks plausible. A
  bounded structural view renders whole, under one status.

## Law — the apply image (`30R:receipt-species-and-correlation`)

- **the-container-encodes-never-changes** — `image.rs` may encode an apply image and may never
  alter one. No bundling, flattening, relocation, import rewriting, path normalization,
  deduplication, or byte change: re-materializing reproduces every entry, path, mode, root, edge,
  entrypoint and byte. The grammar REFUSES a shape it cannot record; it never repairs one.
- **framing-is-by-declared-length** — content and path blocks are consumed by their declared
  byte length and never scanned for a delimiter, which is what lets an entry hold any byte,
  including a run spelling `image-end`. The corpus carries that vector; keep it.
- **identity-is-minted-in-the-constructor** — the two mints validate, encode, hash, and store in
  one operation. No constructor accepts an `ApplyArtifactImageId`, and there is no digest-only
  or archive-reference constructor.
- **parse-compares-against-a-supplied-identity** — the container carries no identity of its own,
  so `parse` REQUIRES the expected `ApplyArtifactImageId` and refuses a mismatch before an image
  exists. There is deliberately no unchecked variant: the caller threading the skeleton's digest
  in is what binds the two. Every invalid vector is handed the identity of its own bytes so a
  departure can never be masked by the identity check.
- **the-canonical-bytes-are-stored-once** — an image owns the exact bytes it was minted or read
  with, and `encode()` hands those back rather than re-running the encoder. Re-encoding to
  compare is the failure class this format exists to avoid. The corpus proves encoder and parser
  agree by RE-MINTING a parsed image through the public constructors and comparing bytes —
  comparing a parsed image's own `encode()` to its file is vacuous.
- **a-stream-has-no-path-and-no-mode** — enforced twice, and both halves are load-bearing:
  `ApplyImageEntry::stream` cannot be given either, and the parser refuses a declared path or
  mode on a stream entry independently, because a document is not built through that
  constructor. At most one stream per image — two path-less entries could not be told apart at
  materialization.
- **paths-refuse-what-cannot-materialize** — beyond the byte and component grammar, two
  whole-image rules are siblings and belong together: no two paths equal under ASCII
  case-folding, and no path names a directory another path names as a file. Both refuse for the
  same reason — the pair cannot exist on any filesystem — not as a policy preference.
- **an-unknown-mode-is-unrepresentable** — `RecordedMode` is `Unused` or `Octal(ModeBits)`,
  with no unknown arm. `unused` means mode is not an execution input, NEVER that a relevant
  mode was not known. A caller that cannot tell which it has must refuse at its own seat; it
  cannot record the question. Nothing in this crate can enforce THAT obligation — see the
  note in `Owed`. What the crate does enforce is the range: `ModeBits` holds the
  four-octal-digit bound in a private field, so a value the wire form could not spell cannot
  be constructed anywhere. The mint is not trusted to check it, and no call site can be the
  place that forgot.
- **roots-are-artifact-units-not-files** — a root is a top-level authored unit the artifact
  covers, pointing at the entry that materializes it; several roots may name one entry, which is
  what a flattened artifact is. A single external stream is its own root and its own entrypoint —
  that is not a fabricated bundle root, because no bundle exists.
- **every-entry-is-accounted-for** — topology validates both directions: forward, every
  entrypoint, root target, and edge endpoint names an existing entry; reverse, every entry is
  named by a root, an entrypoint, or as some edge's child. An orphan refuses.
- **a-cycle-is-recorded-never-refused** — a load cycle is bounded by `topology_depth` (the walk
  ignores the edge that closes it) and otherwise recorded as-is. The container reports what an
  apply uses; it does not adjudicate whether the book is sensible, and refusing here would refuse
  an apply before its intent was ever published.
- **edges-sort-but-identifiers-do-not** — edge order carries no information, so the mint sorts
  into canonical order and the parser requires strictly ascending (which is also the no-repeat
  check). Entry and root ordinals are the caller's, so a non-contiguous one REFUSES rather than
  being renumbered; entrypoint ORDER is execution order and is preserved exactly.

## Law — bounds

- **limits-are-one-policy-value** — every parser takes a complete `ReceiptLimits`; a nested
  parser consumes both the parent budget and its own. A parser that carried its own bound
  would let two parsers disagree about one document.
- **never-allocate-from-a-declared-count** — a count a document declares is checked against
  policy before anything is read or reserved. Lowering a bound to keep a path bounded is a
  local act; widening or removing one is a conductor act with boundary-minus/at/plus tests.

## Law — identities

- **domains-are-separated-by-the-envelope** — every derived identity is a digest over the
  one injective envelope with a distinct payload type. Two different `(type, body)` pairs
  cannot encode alike, which is what makes the domains separate rather than merely
  different inputs.
- **a-document-identity-is-minted-not-derived** — a receipt identity comes from an injected
  `ReceiptIdSource`, not from content: two documents over identical content must not
  collide. `ReceiptId::of_source_bytes` is the one seam every source mints through; a source
  filling it from a counter is a fixture and lives only in a test. The gate over its callers is
  lexical and two-way (`crate_boundary.rs`), because no type can privilege one file over another.
- **a-fence-matches-identifier-boundaries** — every lexical gate in `crate_boundary.rs` matches
  whole identifiers, never substrings: `age` occurs inside `storage`, `package`, `message`, and
  `ApplyArtifactImage`, and a fence that cries wolf is one people learn to route around rather
  than read. `names_identifier` is the one matcher and carries its own regression test. Each gate
  is two-way — an entry that no longer names its subject fails, so the list cannot rot into a
  description of what used to be true — and asserts a non-empty walk, so a gate looking in the
  wrong place fails rather than passing over nothing.
- **content-identities-hash-in-their-constructor** — `PlanningInputId`, `PresentedPlanId`,
  and `ApplyArtifactImageId` are computed from complete typed material in the same operation
  that stores them. No public constructor accepts one ready-made.

  All three mints still take the canonical ENCODING as bytes, because no type can state "these
  bytes are a complete canonical encoding of X". Each is therefore named for what it CONSUMES
  (`of_canonical_inputs` / `of_canonical_decision` / `of_canonical_image`) rather than for what it
  produces, so a whole-identifier fence can match it, and each is gated by the same two-way lexical
  allow-list `ReceiptId::of_source_bytes` uses. A mint named `over` could not be fenced at all —
  the identifier is too common to match without crying wolf, which is the property
  `a-fence-matches-identifier-boundaries` rules out.
- **provider-identities-never-convert** — the signing and encryption identities derive under
  separate domains and no conversion exists in either direction. An identity in a document
  aids lookup; it never selects an implementation or grants acceptance.

## Law — the recorded models

- **a-row-writes-positionally-and-reads-by-key** — `RecordedRow::atoms` emits in table order and
  `of_record` reads by key. `SkeletonRecord::build` checks each atom against the table, which
  catches a wrong TYPE and never a swapped PAIR — `leaf` and `ast` are both counts. The round
  trip is the fence, so every row fixture uses DISTINCT values in same-typed fields; equal values
  make a transposition invisible.
- **relational-closure-lives-in-the-model** — declared counts agreeing with rows present, ordinals
  contiguous from zero, an ordinal naming a row that exists. These are checks only a typed model
  can express, so `format` stays a pure one-exact-form byte check and the aggregates refuse.
  Intra-document references are checked here; a reference reaching ANOTHER document
  (`site-outcome.assignment` into its intent) is graph work, because a document read alone cannot
  know what the other one declared.
- **two-refusal-families** — `RefusalReason` is the grammar answer and `ModelRefusal` the model
  answer, and they are not interchangeable. A document can parse and fail to close over itself;
  that is not a byte-level departure and must not be spelled as one. Adding a variant to
  `RefusalReason` is a conductor act.
- **the-render-axis-is-a-function-of-the-kind** — `render-decision` carries `subject member`, and
  `RecordedRenderKind::subject_axis` decides which the row may populate: a leaf (with an optional
  member) for the site-keyed kinds, a region ordinal in the `region-decision` space for the
  region-keyed ones, neither for `import-*` and `defensive-emission-*`. `RenderSubject` makes the
  disagreeing combinations unrepresentable rather than merely refused, because a region owns no
  execution and a row keyed by a contributing invocation names the wrong thing.
- **emission-is-canonical** — a model emits in its species kind order, then by ordinal within a
  kind, and reading requires ordinals already ascending. Two documents carrying the same content
  cannot differ in bytes, which is what "one exact writer form" means above the byte layer.
- **one-identity-may-span-two-projections** — a receipt identity names the receipt-EVENT, not the
  byte-document, so a rich document and the plain remint of it legitimately share an identity and
  legitimately differ in bytes. Correlation therefore classifies a second claimant through
  `projection::same_identity_pair` and never by comparing models or content: differing bytes are a
  finding only WITHIN one projection. `GraphNode` retains the projection word and the exact image
  for that reason alone — the rule lives beside `narrow_to_plain`, which is what creates the
  legitimate case, and is consumed here rather than re-derived. The pair that separates the rule
  from the bug is two adjacent cases, one per crate: same projection differing bytes is a finding,
  two projections of one event is one node.
- **a-negative-case-names-its-exact-refusal** — every refusal fixture asserts the refusal AND its
  operands, and every graph fixture asserts the whole shape: node counts, the exact edge list,
  the exact finding list. A test asserting only that something was refused is satisfied by a
  refusal for any other reason, which is how a guard silently stops covering the departure it is
  named for. Graph findings are retentions plus a verdict, so this matters most there: the wrong
  finding, the right finding for the wrong pair, and one finding where two were owed all satisfy
  "a finding was recorded".

## Owed

- **owed-unknown-mode-refusal-at-the-caller** — `RecordedMode` makes an unknown mode
  unrepresentable, which means the obligation to REFUSE when mode-relevance is unknown lands
  wholly on the artifact-to-image conversion. No type here can state that obligation, and no test
  here can fail if that conversion quietly records `Unused` for a mode it never determined.
- **owed-reingestion-consumer-gate** — the identity mint now has its lexical gate
  (`crate_boundary.rs`), and the sibling gate enumerating every `Reingested` consumer across
  crates lands with the stage that first wires one.
