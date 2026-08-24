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
| 2A apply image | FOLDED into `ai/r30-receipt` @ `5ba1c9c0` | DONE |
| 2B overlay + age | FOLDED @ `8d7311f4` | DONE |
| 2C recorded models + graph | FOLDED @ `575bf489` | DONE |
| 3 presented plan + PlanReceipt | `ai/r30-receipt` | dispatched from `761a6ea0` |
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

## my scheduling error — concurrent WSL gates

`spike/CLAUDE.md` already rules that heavy WSL work is SERIALIZED across concurrent
lanes (the ~20GiB WSL cap binds, not host RAM). I dispatched three parallel lanes each
ending in `mise run both gate:full-quiet` and broke it. 2A took three preflight refusals
(2.8 → 1.9 → 0.5 GiB available, `vmmemWSL` RISING) and correctly refused every wrong
escape — no `DORC_PREFLIGHT=skip`, no `wsl --shutdown`, no touching sibling processes —
decomposing the wrapper into its constituent checks instead and reporting the wrapper as
NOT obtained. Also learned: polling makes the reading worse, since each probe consumes
the RAM it measures. Both remaining lanes now carry the handling rule; serialize on
request. `preflight-bounds-before-spend`'s "run Windows first" advice does not cover the
three-lanes-at-once shape — worth a line from the steering author.

## from the models lane (2C, `db3104df`)

Deviations, all reported not taken: two extra modules to keep files under ~1500 lines;
graph stores recorded models + a trust token rather than heterogeneous `Receipt<D,P,T>`
(`30Rb` explicitly permits when clearer); `RecordedApplyAssignment` is the row and
`AssignedTarget` the composed M:N value with no public constructor; `PartialReceipt` and
`OutcomeAvailability` non-generic. Ratified: two refusal families (`RefusalReason` for
grammar, `ModelRefusal` for the model), which composes with 2A's local `ImageRefusal` and
2B's overlay arms — check at fold that three has not become a thicket.

The two-way fence caught two stale `MAY_NAME_IT` entries on its FIRST run (`cli` names no
receipt crate; `receipt-crypto` cannot depend on itself). A one-way fence would have
carried both forever while asserting nothing. List is now empty and true; the stage that
first wires a production caller adds its entry as a visible act.

**Third sighting of one pattern, now generalized:** `ids.rs` has nine public `of_hex`
constructors from bare digest text, and `Sha256Digest::over` takes a caller-chosen domain
— together they mint any content identity over arbitrary bytes. 2A's
`ApplyArtifactImageId::over` and 2B's `OverlayPlaintext::of` are instances of this root.
DECIDE THE NARROWING AT THE STAGE 2 FOLD.

Invariant prose from this lane:

A row's `atoms` write positionally into the grammar table's field order, and nothing in
the type system relates the two; `SkeletonRecord::build` cannot help, because it validates
each atom's type independently and `leaf` and `ast` are both counts. The round-trip test is
the whole fence, and it bites ONLY because every fixture row uses distinct values in every
same-typed field. Someone simplifying a fixture to zeros because "the numbers don't matter"
disarms it with nothing going red. The numbers are the mechanism, not decoration.

The render row's identity axis is unrepresentable-when-wrong on the writing side but
RE-DERIVED on the reading side, because `of_record` reconstructs the subject from two bare
integer slots by consulting the kind's axis. Two copies of one rule. Add a kind and only
the writer's copy fails to compile.

The region ordinal space is the one cross-row reference with no type behind it: region rows
define it by position, render rows name it by a bare integer. The in-range check catches a
DANGLING reference, never a WRONG one — so if a projection numbers regions in one order and
emits them in another, every region-keyed render row silently describes a different region
and the document still closes. Whatever mints those ordinals must derive the number and the
emission position from one walk.

Each accessor on a `Reingested` is a decision about what may leave a recorded document, and
nothing would notice a careless one — a future method returning something structured rather
than a report scalar or another sealed value compiles and passes everything. The habit is
the check: scalars and sealed values out, nothing else. That is exactly why the generic
accessor had to go — it moved the guarantee onto a membership list instead of the wrapper.

`MissingOutcome` is unconstructible outside correlation only because it carries a private
field of a private unit type. Load-bearing, and looks like clutter; removing or publishing
it lets any caller holding an intent identity assert no outcome exists — a claim about the
record set that only correlation can make.

Graph order-independence rests on two separable things: `BTreeMap` storage and sorted
returns. One test notices, and only because it feeds the same documents backwards. Anything
new returning a collection must sort and must join that reversal test.

Several reads are `Option`-typed for shapes the grammar already forbids, and those branches
fail QUIETLY — a document whose identity would not parse is skipped rather than reported.
Fine while the field type guarantees the shape; a silent disappearance the moment anything
loosens it.

## MY ERROR — the gate could answer green without asking a question

2B measured `gate:full-quiet` returning rc=0 in THREE LINES having run nothing. Cause
confirmed: I minted the three Stage-2 lane worktrees with `git worktree add -b <branch>
<path> <commit>`, which sets NO UPSTREAM; `hk check --pr` then has no base to resolve,
matches no paths, and with a clean tree `--staged`/`--unstaged` match nothing either.
Three no-ops, exit 0. `spike/CLAUDE.md` already holds this principle for the corpus
runners (a discovery floor; zero trials would otherwise exit green) — the GATE has no
equivalent. Upstreams now set on all lane branches; a floor is being built on
`ai/r30-gate-floor`. Standing process fix: every lane worktree is minted FROM `ai/main`,
never from a bare commit.

Corollary worth holding: 2A reported a green Windows wrapper on an upstream-less branch,
so that green was probably vacuous too. Its WSL leg (208s, cold cache) was real, and every
lane independently ran `mise run test` directly — which is where the substantive evidence
for all three lanes actually comes from, not from the wrapper.

## from the overlay lane (2B, report recovered after the session ended)

Its 17 commits survived intact; only the write-up was lost, then recovered on resume.
Rich reading was structurally IMPOSSIBLE on the base, not merely untested. Two committed
invalid vectors were refused for genuinely the wrong reason (`blank-line` and
`record-count-too-high` both landing on `RecordCount`). `OpaqueFieldTag` did not exist at
all on its base — the whole 17-member table is 2B's, with `apply-context` at 16.
`#![expect]` verified to suppress the duplicate-version lint on clippy 0.1.96 and to
self-ratchet. `Projection::Region` added (approved: `30Rb` requires the effect, and an
`Option` would leave an unvalidated rich receipt representable).

RULED `tc-remint-keeps-the-document-identity`: a plain remint KEEPS the rich document's
identity — the identity names the receipt-event, not the byte-document, and species and
projection both sit inside the signature domain. Cross-lane consequence neither lane could
see: 2C's graph treats one identity with differing bytes as a FINDING, which would fire
spuriously on a rich receipt beside its own plain remint. Refined: the finding is
same-identity, same-projection, differing bytes; same identity across DIFFERENT projections
is legitimate and correlates to one node. 2B pins it.

Owed and assigned to 2B at rebase: the 2A refusal bridge (`RefusalReason::Image`), and
`plan-frozen-rich-document-vectors`, which is a V1 EXIT condition
(`30Rb:receipt-verification-map` wants a valid plain AND rich receipt per species), traded
away under time pressure for the remint and the exact-refusal table. No age identity was
generated or committed, so its four conditions activate now.

Invariant prose from this lane:

A plain document's signed body and its skeleton span are BYTE-IDENTICAL — the locator sets
body-end to skeleton-end when there is no region — so every plain test passes whichever of
the two you hand the parser. They diverge only for rich, which did not exist when the choice
was made. That is exactly how the reader came to parse the whole body: correct for
everything that existed, structurally unable to succeed once a region followed the
terminator. What must stay true is that the skeleton span and the armor span are both slices
of the ONE located body, taken in a single pass, and that neither is re-derived by a second
walk. Nothing stops a future reader from locating the armor separately — and then the bytes
verified and the bytes opened could differ with no type complaining.

The line-ending normalization is safe only because it happens before serialization, hence
before signing, so the signature covers exactly the stored form — and because nothing on the
read path normalizes at all. Move that same conversion anywhere onto the read side and it
silently becomes a normalization sitting between the bytes verified and the bytes used,
which is the single failure the literal-byte discipline exists to prevent. It is correct
because of WHERE IT SITS, not what it does.

Paired and equally invisible: `seal` strips trailing newlines because the format supplies
the one that closes the region; `open` pushes exactly one back. They mirror each other
ACROSS A CRATE BOUNDARY with nothing enforcing the pairing, and change either half alone and
the round trip breaks looking like a decryption failure rather than a framing one. The
append is legitimate only because it happens AFTER the outer signature has checked.

The region terminator search works because of somebody else's alphabet: the locator finds
the region end by searching for a literal, unambiguous only because Age's armor is base64
plus marker lines and standard base64 excludes the hyphen. That argument lives inside
another project's encoder and is one upgrade away from false. Now asserted locally by a
writer-side shape check, itself measured against a REAL sealed region — so an Age change
fails one loud test instead of every rich document mysteriously failing to locate.

Token-set constants are `const`, not `static`, so they inline at each use site and
`core::ptr::eq` on them is unreliable — a slot-versus-grammar cross-check failed on exactly
that. Compare token sets by value.

The overlay's total-account property rests on the captured-slot set being computed from the
SKELETON ALONE. If that computation ever took a hint from the region, the region would be
describing the set it is required to satisfy, and both directions of the two-way check
collapse into one. Short function, looks unimportant.

A rich receipt cannot exist without a validated region only because its constructor is
private and reachable solely from the validating read path — enforced by VISIBILITY, not by
the type. The region field would happily hold an unchecked value if a second constructor
appeared.

## the vacuous gate — my diagnosis was WRONG; the floor is the answer anyway

CORRECTION: I asserted "no upstream" as the confirmed cause, to the human and to two
builders. The gate-floor builder TESTED it and disproved it — hk reads `hk.pkl`'s
`default_branch` (pinned since 2026-08-18), not the git upstream, and resolves `ai/main`
correctly on a branch with none. Setting upstreams changed nothing causally; the Windows
leg going 3 lines → 196 correlated with a rebase, not with my fix. I ruled from a plausible
mechanism instead of testing it, having just told a builder off for the same shape.

Three vacuity causes now known, and only ONE reproduces:
- `default_branch` absent/unreadable ⇒ hk guesses `origin/HEAD` (this remote is `ec`, so it
  fails), falls back to `main`, prints three git fatals and EXITS 0.
- The WSL-leg instance 2B measured: does not reproduce. Both legs now resolve
  `default_branch` = `ai/main` and `hk check --pr --plan` selects 19 files on each. Most
  likely hk's per-worktree state cache (2B separately observed hk caching a prior run's
  steps). Transient, invisible, undebuggable after the fact.
- **PURE DELETIONS SELECT NOTHING** (the builder's find, reproduced live): every glob is
  matched against paths no longer on disk, so a deletion-only change selects ZERO steps and
  the gate reports green over a tree that cannot compile. **STAGE 6 IS A DELETION STAGE** —
  ripping `whylog.rs`, `whylog_store.rs`, the fixtures, the flags. Without the floor, Stage
  6's completion gate would have been vacuous by construction. Accepted cost: a
  deletion-only commit now refuses; that is correct, not cry-wolf.

That two of three causes are transient or non-reproducible is the whole argument for
asserting the OBSERVABLE (work was selected) over diagnosing causes. Floor limits, stated
by its builder: it predicts SELECTION from hk's own plan, not EXECUTION; and `hk config get`
hangs forever outside a git repo (hk 1.53.0), so it confirms a worktree via git first.

## STAGE 2 CLOSED — assembled at `ai/r30-receipt`

Fold order held: 2A → 2B → 2C → gate-floor. 53 commits over `ai/main`. Every conflict
resolved as a UNION with nothing discarded (2C's rule, taken from 2B's `.gitattributes`
precedent). Removed two now-false crate `CLAUDE.md` Owed entries that 2B discharged.

Two nits carried into Stage 3, neither worth a lane:
- The crate has TWO public `RecordedMode` enums — 2A's in `image.rs` (a file mode) and 2C's
  in `tokens.rs` (an invocation mode). Different modules, neither root-re-exported, so it
  compiles unambiguously — but one crate, two same-named public types, unrelated meanings.
  Rename at the top of Stage 3.
- `crate_boundary.rs` now carries two crate-name matchers (2A's `names_crate`, 2C's
  `names_identifier`). Redundant, and neither lane's to delete.

2C's added invariant prose: a recorded row model and the grammar table are two files that
must move together, and the failure is NOT silent — the model emits through `to_record` and
the table refuses a row it could not read back. The writer's self-check is what turns a
cross-lane grammar widening from a corruption into a test failure, and it works only while
every row goes out through `to_record`; a projection hand-building a `SkeletonRecord` would
bypass the one thing that caught it. Also: retaining the document image means node identity
depends on caller-supplied bytes, so an ingest caller passing a RE-SERIALIZED image rather
than the bytes it read would make two reads of one document look divergent — the
literal-bytes discipline arriving one layer up, where no type enforces it.

## stage-2 acceptance — the strongest evidence of the arc

`mise run both gate:full-quiet` on the assembled `ai/r30-receipt` @ `761a6ea0`: rc=0, BOTH
legs, all three hk checks executed on each, zero failures — and the new floor fired on both
(`127 changed file(s), 11 check(s) selected`), so non-vacuity is ASSERTED rather than
assumed. A direct re-run minutes later hit the `Access is denied` relink lock, re-confirming
that lock as intermittent. NB for my own future reading: a `cmd > file` inside a
backgrounded Bash call leaves the harness's own capture EMPTY — read the redirect target,
not the task output file. I raised a false alarm on exactly that.

Builder sizing, measured across this arc (carry into every later brief):
scouting 10–17 min, consistently. Build phases 37–80+ min. TWO builders degraded or died
past ~70 min / ~700k tokens; the healthiest lane totalled 47 min across three calls. Size a
call at ~45 min and PLAN the handoff. The scout/STOP/ack split costs ~12 min and buys a warm
restart point — both lanes that died had banked their scouting and their commits, so two
deaths cost one write-up total, and even that was recovered by resuming the agent the
harness had given up on.
