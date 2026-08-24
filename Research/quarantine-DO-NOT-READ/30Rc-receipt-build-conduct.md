# 30Rc — receipt build: conductor ledger

> Tier: quarantined, conductor state. `30Ra` owns design/rationale; `30Rb` owns the
> build specification. This file duplicates neither: it carries only lane state,
> adjudications, and deviations.

## lanes

Conductor: `ai/r30-conduct` @ `.claude/worktrees/r30-conduct`.
Build lane: `ai/r30-receipt` @ `.claude/worktrees/r30-receipt` (serial stages).
Stage-2 lanes branch from the Stage-1 tip; fold 2A → 2B → 2C.

| Stage | Lane | State |
|---|---|---|
| 0 laws/crate/deps/vectors | `ai/r30-receipt` | LANDED `88f71314..23b5a9b7` |
| 1 identity + plain kernel | `ai/r30-receipt` | LANDED; gate blocked, see below |
| 2A apply image | `ai/r30-receipt-image` | BUILT `f7f6aa53`, both legs green; one reversal owed |
| 2B overlay + age | `ai/r30-receipt-overlay` | dispatched 11:35 |
| 2C recorded models + graph | `ai/r30-receipt-models` | dispatched 11:35 |
| 3 presented plan + PlanReceipt | `ai/r30-receipt` | not started |
| 4 intent/dispatch/outcome | `ai/r30-receipt` | not started |
| 5 why/correlation/re-derivation | `ai/r30-receipt` | not started |
| 6 rip old implementation | `ai/r30-receipt` | not started |

Every builder: scout → STOP → conductor ack → implement.

## standing brief riders (carry into every dispatch this arc)

- Opacity: out-quarantine artifacts carry mechanism, never reasoning. `adversarial`
  and `threat` never appear outside quarantine; `untrusted`/`authenticated` are code
  identifiers only, not prose. Comment budget near zero — a doc-comment states the
  mechanical contract and stops.
- Citations spell `quarantine/<docID>[:slug]`. Never the directory's real name in a
  citation, never a resolvable path; the non-resolving form is the speed-bump.
- Non-correctness tooling chafe (lints, hk routing, mise ergonomics, config) is
  FIXED, not documented or worked around — human-authorized 2026-08-24. Correctness,
  testing, and verification machinery is excluded and escalates to me.
- Read-only Sonnet scouts: 1–2 concurrent, ~3–4 per stage; find and collate, never
  opine or decide. One carve: a narrow read-write scout may repair tooling per above.
- Every builder: scout → STOP → conductor ack → implement.

## adjudications

Stage 0 checkpoint, 2026-08-24:

- **Crate split — a deliberate deviation from `30Rb:result-and-exit` item 1.** `age`
  pulls `rand`, and `plan -> dorc-receipt -> age` would land an RNG in the kernel's
  graph. Root `AGENTS.md` (human-authored, outranks `30Rb`) requires kernels stay
  clean of nondeterministic deps. A cargo feature gate fails to workspace feature
  unification. So: pure `dorc-receipt` (models, grammar, ids, limits, overlay
  validator, PAE, digests, reader/writer states, graph, reingestion, capability
  traits) + `dorc-receipt-crypto` (Age/Ed25519 impls only). `plan` takes the pure
  crate; `cli` takes both. Every authority mint stays pure-side — the impl crate
  cannot mint checked/trusted/complete states — so the split strengthens
  verify-before-interpret rather than relocating it. `sha2` is the one dependency
  reaching the kernel; deterministic, and chosen over moving the hand-rolled
  vector-pinned SHA-256 in, because taking a second unforced deviation on top of a
  forced one is creativity.
- `age` pinned 0.12.x with `armor` explicit (not a default feature). 0.11 measured
  as a data point only; a switch returns to me.
- `core::spine::InvocationMode` untouched (human-ruled at `30N` §4); the receipt
  mints its own mode enum from the CLI dispatch seat. `attempt` ADDED to the
  `invocation` record — `sinv-controller-attribution` binds regardless of the
  field list, and the checkpoint is where the reviewed table is set.
- `render-decision`: subject as integer-or-`absent`, axis carried in `kind`, detail
  tag selected by kind. Protects `30N:rul-region-refusal-discloses-region-keyed`.
- Splits accepted rather than merges: `survival.outcome` (8) — a solver defect
  reported as a book fact is mis-attribution, `271:rul-sin-ordering`'s worst tier;
  `site-classification.class` (8) — the bools decide licensing.
  `SessionOutcome::LostAfterSend` maps to `unknown`, never `transport-failed`.
- `apply-context` added at order 16 of the opaque-field-tag table.
- `solve-certification.pass` projects the closed five-token set now, not waiting on
  `30M`. `region-decision.routes` keeps the total; the keyed/unkeyed split takes a
  projection-omission.

## deviations reported by builders

- Stage 0+1: `crypto.rs` → `capability.rs` in the pure crate (the adapters it named
  moved to the sibling); `check_signature` returns a two-arm enum rather than a
  generic, because a generic IS the "caller requests its preferred trust marker" the
  spec forbids. Both accepted — the second actively enforces a REQUIRED effect.
- Stage 0+1 rewrote two integration-test files panic-free rather than using the
  documented file-top `#![expect]`, costing ~1h. MY briefing gap, not its error: the
  lint posture lives in `spike/Cargo.toml` + `spike/clippy.toml`, and
  `spike/CLAUDE.md`'s "(tests may)" is misleading without the caveat. Now a standing
  rider in every brief.
- Stage 0+1 reported `gate:full-quiet` as environmentally blocked. I called that
  transient after two clean `mise run build` runs; I WAS WRONG — `build` never has to
  REPLACE `dorc.exe`, which is the denied operation. Confirmed reproducing under
  `cargo test --no-run`. See blocker below.

## OPEN — completion gate on Windows (INTERMITTENT, not a hard block)

REVISED after 2A: it ran `gate:full-quiet` green on BOTH legs and never hit the
signature, so this fires on a SyncThing scan window rather than deterministically.
Lanes can retry. The fix below is still owed — an intermittent gate is a gate people
learn to re-run until it passes, which is how a real failure gets waved through.

`cargo nextest`'s relink fails `Access is denied` removing
`<worktree>/spike/target/debug/dorc.exe`. Windows `CARGO_TARGET_DIR` is
`{{config_root}}/spike/target` (`mise.toml:118`) — inside the SyncThing-synced tree,
with no `.stignore` covering it; SyncThing holds the handle. The WSL leg already
points outside the tree and is unaffected. Two fixes, human-owned: exclude
`**/target` from SyncThing (the actual root cause; target dirs are huge and
machine-specific and should never sync), or extend `mise.toml:118`'s Windows arm to a
per-worktree cache outside the tree, mirroring the WSL arm. Blocks the completion
contract for every builder on this arc.

Stage 2 checkpoints, 2026-08-24 (all three lanes):

- **Base red, found independently by 2A and 2B, fixed by me at `50e1b2d4`.** The
  `carriage-returns.skeleton` invalid vector had ZERO CR bytes committed — root
  `.gitattributes` `* text=auto eol=lf` ate them at `git add`. Green in the authoring
  tree only because git never rewrites the working file. Renamed to `.crlf` (the
  pre-existing `*.crlf binary` rule, until now matching zero files). I fixed it on base
  rather than in a lane: three lanes repairing one shared file is a three-way conflict.
  Generalized rule for the arc: never trust the working tree as evidence of what was
  committed; verify with `check-attr` + `cat-file -p :<path>`.
- **Table amended three times total** (`invocation.attempt`, `admission.stream`,
  `render-decision.member`), each closing a live loss inconsistent with the table's own
  conventions. Principle stated to lanes: "record more" is the tiebreaker where the spec
  is SILENT, never a licence to grow a REQUIRED field list whenever something is
  droppable. Applied in the negative to `narrative`'s dropped site — its consumer is
  owned by another round (`289:seam-narrative-render-unconsumed`), so it takes a
  projection-omission instead.
- **I reversed my own clippy ruling on 2B's measurement.** I chose `clippy.toml`'s
  `allowed-duplicate-crates` because it "names the specific duplicates"; 2B measured TEN
  duplicates, not seven, three being generic ecosystem churn (`syn`, `thiserror`) — and
  `clippy.toml` is workspace-wide, so that option is BROADER than the crate-root
  attribute I rejected. Now `#![expect(…, reason)]` in `dorc-receipt-crypto` only
  (`expect`, so it self-ratchets). My error: I ruled from a crate list I took on faith
  instead of asking for the measurement first.
- Rich parsing could not succeed at all (2B): `ParsedReceiptSkeleton::of` fed the whole
  rich body to a parser whose trailing check demands the skeleton terminator. Invisible
  because no rich document existed. 2B fixes it by parsing the skeleton prefix.
- **A pattern, not two slips:** 2A and 2C independently found public constructors taking
  bare bytes (`ApplyArtifactImageId::over`, `OverlayPlaintext::of`, `Reingested::as_report`).
  Same shape three times in one crate. Worth the steering author's attention.
- Skeleton digest stays PLAIN SHA-256 over the literal skeleton span (both governing docs
  say so) against 2B's domain-separation lean: it is a binding check, not an identity
  mint, and `sha256sum`-by-hand reproducibility IS the readable-format product goal.
  If crate law reads as covering it, that is crate-law wording to sharpen.
- Approved: a committed age fixture identity (fresh-generated, fixture-named, seals only
  committed vectors, covered by the lexical fence); `*.receipt`/`*.overlay`/`*.applyimage`
  binary-pinned by extension, `*.skeleton` left text for reviewable diffs; 2C's inert
  deterministic test signer inside `dorc-receipt`'s own tests.
- Both fences 2C offered: two-way allow-list (`sinv-production-fences` requires it) and
  resolver-implementor enumeration.

## banked for later stages

- Stage 4: `transport::SessionOutcome` is whole-artifact, not per-site, and no apply
  executor exists — V1 per-site rows come from hostsim/DST only.
- Stage 2A: `ArtifactSet`/`ArtifactFile` discard modes, roots, and edges, and hold
  bytes as `String`; the image container is binary-safe. 2A is "carry topology the
  live type throws away", not "add a container".
- Stage 3 is BIGGER than `30Rb` sizes it, three ways (2C measured): `class_label`
  (`cli/main.rs:2924`) discards both booleans before they reach Spine, so widening
  `SpineSiteClassification` is MANDATORY not optional — my 8-token split created this;
  four Spine species are unminted, so `licensor` deriving from `ReplaceLicense`/
  `GuardLicense` is new `plan` code, not an accessor read; and `SpineSiteClassification`
  is CFG-node-keyed against a site-keyed decision plane, so `cli/main.rs`'s hand-built
  `ast → leaf` back-map must become a real seat. That is a refactor, not a projection.
- Stage 4: `gap-plain-intent-topology-summary` — `30Rb` promises plain intents carry
  "topology/count summaries" but `apply-assignment` has nowhere to hold them; deferred
  here with a projection-omission rather than rippling the grammar mid-fan-out.
- Stage 5: `dorc-loom` reads receipts and so needs a verifier; decide there whether
  it takes the impl crate or a fixture verifier. Also: narratives carry no site, so no
  narrative is attributable to a line — `dorc why N` is site-keyed.
- At the Stage 2 fold: decide `ApplyArtifactImageId::over(&[u8])`'s public visibility
  (2A left it, correctly, rather than change a Stage-1 type mid-fan-out).
- `mise run lint:docids` accepts and checks `quarantine/<docID>` (prefix transparent
  to the matcher, resolved against a name-only listing — no quarantined file opened).
  It scans Markdown only; `.rs` citations are unvalidated, a crate `CLAUDE.md` is not.

## invariant prose (raw; for the design agent to synthesize — do not pre-format)

From the apply-image lane:

The mode type has no unknown arm, so a caller cannot record "I don't know whether this
entry's mode is an execution input." That reads as safety and is, but the obligation it
creates lands entirely outside the crate: whoever writes the artifact-to-image conversion
will hold a two-arm choice, and the path of least resistance is to write "unused" for
anything not obviously executable — which is the exact failure the type was meant to
prevent, merely relocated. No signature states the duty and no test in the crate can fail
if that conversion quietly guesses. The conversion has to refuse, and only someone knowing
to will make it.

The container refuses a great deal, and every refusal happens before an intent is
published. That is the right seat, but it makes the refusal set load-bearing on
availability in a way an ordinary parser's is not: a path outside the grammar does not
degrade an image, it stops an apply. Anyone tempted to widen the refusal set should notice
they are deciding what apply images are expressible at all, not tightening a parser.

`encode()` hands back the bytes the image was minted or read with — deliberately, so the
bytes an identity was computed over and the bytes a consumer reads are one object. The
consequence is that the obvious round-trip test (parse a file, compare `encode()` to the
file) is vacuous and passes however broken the encoder is. The real proof is a `remint`
helper pushing a parsed image back through the public constructors so the encoder actually
runs again. Simplify `remint` away as redundant and the corpus keeps passing while testing
nothing.

Two entry-shape rules are enforced twice on purpose — the stream constructor cannot be
handed a path or mode, and the parser separately refuses a declared path or mode on a
stream row. The second is not redundant: a document does not arrive through the
constructor. Deleting either half leaves every current test green.

The depth walk is deterministic only because starts are visited in ascending order and
children in canonical edge order, and canonical edge order exists because the mint sorts.
On an acyclic graph none of that matters. On a cyclic one it decides which edge is
classified as cycle-closing, so a change to edge sorting silently changes the depth a
cyclic image reports — and one vector in the corpus would notice.

Path normalization and path recording hold opposite postures on purpose: emission-side
code collapses and rewrites, the recorded-path type refuses non-canonical input rather
than repairing it. Both are right at their own seat. If normalization ever drifts into the
recording side, the bytes recorded stop being the bytes applied and nothing says so.

Recurring across lanes, worth the author's attention as one thing rather than three:
public constructors taking bare bytes kept appearing (a content-identity mint, an overlay
plaintext wrapper, a generic report accessor). Each was individually harmless and each
would have become a hole as its type set widened.

And: two lanes independently shipped negative tests that passed for a reason other than
the one they claimed — a vector refused at framing rather than at its named departure, and
a guard never reached by the suite that named it. The fix that generalizes is pinning each
negative case to its exact refusal rather than to the fact of refusal.

## conductor-owed at close

- `plans/30R` reconciliation against `30Rb` (out-of-quarantine surface; opacity
  discipline — mechanism yes, threat reasoning no).
- `LIVING_STATUS` entry; `FORFEITS` row for no-append (`30Ra:no-append-in-v1`).
- prose queue: every register minted `None` this arc.
- `gate:arc` from the populated branch before the fold.
