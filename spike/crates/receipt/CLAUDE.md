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
  `Reingested`. No `Deref`, no `AsRef`, no `Borrow`, no generic `map`, no `into_inner`, no
  raw accessor; decomposition answers another `Reingested` or a report-only scalar. Nothing
  read back may reach a license, a plan, an artifact, probing, or an apply.
- **recorded-and-current-stay-four-states** — `RecordedCurrent` preserves recorded-only,
  current-only, both-agreeing, and both-disagreeing. Disagreement is a finding that keeps
  both values; it is never resolved by picking one.
- **an-unreadable-grade-reads-highest** — absent, unknown, malformed, or unverifiable
  influence material reads `MostInfluenced`. Losing this metadata may only ever make a
  reader more careful.
- **partial-never-becomes-complete** — there is no conversion from `PartialReceipt` to
  `Receipt`, and no field of a partial document is promoted because it looks plausible. A
  bounded structural view renders whole, under one status.

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
  filling it from a counter is a fixture and lives only in a test.
- **content-identities-hash-in-their-constructor** — `PlanningInputId`, `PresentedPlanId`,
  and `ApplyArtifactImageId` are computed from complete typed material in the same operation
  that stores them. No public constructor accepts one ready-made.
- **provider-identities-never-convert** — the signing and encryption identities derive under
  separate domains and no conversion exists in either direction. An identity in a document
  aids lookup; it never selects an implementation or grants acceptance.

## Owed

- **owed-fixture-identity-fence** — `ReceiptId::of_source_bytes` is the correct seam, but
  nothing yet stops a production file calling it with fixed bytes. The lexical
  non-empty-walk gate over its callers is owed alongside the reingestion consumer gate.
- **owed-rich-projection** — `Rich` is declared and no rich document is emitted. The region
  model, its validator, and the sealed round trip are the next stage's work; do not emit a
  partially built rich document outside a test.
- **owed-record-models** — the recorded per-species models, the apply image, and the graph
  are declared in the module map and not yet built. Nothing here may grow an empty field or
  a speculative public API ahead of them.
