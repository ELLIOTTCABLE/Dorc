# 30Ne — the artifact-forms lane: `30I:step-7-reify-plan-artifact-forms`

> Tier: **LLM-authored, builder (Opus-class)**, lane `ai/r30-artifact-forms` from
> `ai/r30-conduct@d9afde93`. Seven commits, `b12f5479..b2aa26cf`.
>
> Read with: `plans/30I` (THE spec; §7 and §2.5 are this lane's) · `notes/30Nd` §11 (the handoff
> this executes) · `notes/30Ib` §15/§17/§18 (the loader and bundle this consumes) ·
> `plans/30L` §8 (the region laws and the single-stream presentation note) · `notes/30N` §3/§4
> (the rulings that bind, and the human queue this feeds).
>
> HEADLINE, and the number a fold reviewer should read first: **existing-corpus drift is ZERO**.
> 178/178 e2e cases pass, of which 176 are pre-existing and byte-identical; `bless:dry` writes
> nothing and leaves a clean tree. §8 is the enumeration, and it is empty by measurement.

## §1 — The form inventory, and their one-structure derivation

`30I` §7.1 names three semantic forms. All three are built, in `cli::artifact` — a pure module
(`lib-target-is-a-loom-seam`: no clock, no file, no env), consuming one already-settled `Plan` and
one already-resolved `BundleProjection`.

| form | what it emits | available when |
|---|---|---|
| `Flattened` | `plan.sh` alone | the book has NO book-sited load occurrence |
| `Multipart` | `plan.sh` + every contracted dependency, MIRRORED at its authored relative path | the artifact stream is a directory AND every dependency path is relative and traversal-free |
| `PreservedBookTree` | `plan.sh`, authored source boundaries untouched, no dependencies | always — it is the honest degenerate form |

**ONE structure.** `artifact::select(…)` settles a `Selection` (form + fallback + the dependency
files) from authored inputs alone; `Selection::with_plan(plan.render_apply(…))` binds it to the plan
projection, producing an `ArtifactSet`. `main.rs` then PRINTS `artifact.primary().bytes` and
PUBLISHES `artifact.files()`. There is deliberately no second assembly of the same bytes anywhere:
the print seat has nothing to fall back to, which is what makes "human and executable forms derive
from one final structure" structural rather than reviewed.

**`fnd-mirroring-is-the-cwd-analysis-answer`.** `30I` §7.4 asks the planner to spend cwd analysis
before scaffolding. The answer this lane found is stronger than the spec anticipated and costs
nothing: a book's `. "$ROOT/pkg/entry.oracle.sh"` already names a path RELATIVE to the load cwd, and
multipart execution begins in the artifact directory (§7.6) — so MIRRORING the controller-side
relative layout under the artifact root makes every authored operand resolve unchanged. The book's
own `.` needs no rewrite, no generated root variable is emitted, and nested operands inside a copied
dependency (the diamond case) resolve for the same reason. The availability question then becomes a
PATH question, which is exactly the one `need-controller-paths-never-cross-hosts` cares about: an
absolute or escaping controller path cannot be mirrored, and the form is unavailable rather than
fudged (`artifact::placeable`).

A consequence worth stating plainly: `plan.sh` is BYTE-IDENTICAL across all three forms. A form is
about where the generated files LIVE, never about what the plan says. That is also why the corpus
did not move.

**What is NOT in a form's file set**: a `--pre-source` root and its dependency tree. Those are not in
the book's bytes, so no form has anything to place for them — the guard preamble already carries
whatever of them the artifact needs (`pinned-definitions-are-the-artifact's-binding`).

## §2 — The selector, and its safety argument

```text
posture = --artifact-dir named ? Materializable : SingleStream
request = --form <name> ? Explicit(name) : Auto
```

- **Auto + SingleStream** → `Flattened` if available, else `PreservedBookTree` WITH an
  `artifact-form-fallback` note naming the form and the cause.
- **Auto + Materializable** → `Multipart` if placeable, else `PreservedBookTree` + the same note.
- **Explicit, unavailable** → `artifact-form-refused`, **pre-network**, exit 16. Never a different
  form (`30I` §14 keeps that out of builder latitude).

**Why flattening REFUSES rather than inlining, and why that is the safety argument.** Textual
inlining of a load-inert child at its `.` position is ARGUED sound and NOT MEASURED (`30Ib` §5
row 8); the obvious alternatives are already gone (`fnd-loader-function-errexit-diverges` refuted the
generated loader function, and a subshell kills the definitions the load exists to install). `30I`
§7.1 sanctions failing before network when a supported construct cannot yet be flattened safely, and
`30Ib` §15 recommends exactly this posture. So:

- a book with NOTHING to inline is already one stream ⇒ flattened, byte-identical, no refusal;
- a book WITH a book-sited load ⇒ flattened is unavailable ⇒ explicit intent refuses, auto falls
  back and says so.

That is what keeps the flattened form's goldens off an unproven construct: v0's flattened form emits
no inlining construct at all.

**`tc-piped-stdout-vs-named-artifact-dir` (FLAGGED UP — an interpretation of a TYPED rule).**
`rul-piped-stdout-implies-one-flat-plan` is typed: a non-terminal stdout is single-stream intent, and
explicit multipart refuses. I implemented the posture from `--artifact-dir` alone rather than from an
`IsTerminal` probe, reading §2.5's own scoping — "it governs the artifact stream alone", and "the
artifact set publishes to disk while stdout carries the render" — as meaning the rule binds whichever
stream is CARRYING the artifact. Naming `--artifact-dir` moves that stream off stdout. Consequences:
(a) `dorc plan --artifact-dir out book.sh > log` proceeds rather than refusing; (b) no terminal probe
exists, so nothing non-hermetic entered the edge and both cells are drivable deterministically —
which is what §2.5 asks of the injection; (c) explicit multipart WITHOUT a directory refuses
(`no-artifact-stream`), which is the rule's observable consequence and is pinned. If the conductor
reads the typed rule as also demanding the terminal probe, the change is one edge value threaded into
`select_artifact_form`, and the e2e coverage in §6 would have to go (the rail captures stdout).

## §3 — The flattening presentation shape

`30L` §8's single-stream note is an obligation recorded for this lane: on a fully flattened plan,
dorc-lang material is present in the one stream but is NOT attention surface, and the render must
separate "pay attention here" from the bundled non-mutative miscellany.

**As built: DISCHARGED AT V0'S SCALE, and the shape decision deferred with the inlining.** v0's
flattened form contains no dorc-lang material beyond the GUARD PREAMBLE, which is already top-lifted
by `pinned_definitions` and already opened by `render::apply::guard_preamble_banner` — the separation
the note asks for, at the only scale that exists today. There is nothing else in the stream to
separate, because the form refuses to inline.

**`tc-flattened-section-boundary` (FLAGGED UP).** The preamble is opened and not CLOSED: a reader has
no marker for where the miscellany ends and the book begins. Adding one closing comment line is a
~5-line change in `render::apply` and is what I would land — except that it moves the artifact bytes
of EVERY case carrying a guard preamble, which is unblessable drift under this brief's golden rule.
Priced and declined rather than done: the conductor can take it with one scoped bless, or leave it to
land with the inlining, where the boundary has real content to fence and the drift is paid once.

## §4 — The region-refusal disclosure, as built

`30N:rul-region-refusal-discloses-region-keyed` [CONDUCTOR, binding] answered
`30Nd:tc-region-refusal-disclosure-home`. Built exactly as ruled — a region-keyed record axis plus a
region-keyed diagnostic site, and NEVER a smear across the contributing invocations.

- **The plane**: `RefusedEdit` gains `region: Option<ElisionRegion>` beside its `leaf`. Exactly one
  is populated; keeping both rather than one nullable leaf is what let the region half stop being
  silently undisclosed.
- **The record**: `SpineRenderDecision` gains a second key axis, `region: Option<ElisionRegion>`, on
  `SpineRegionDecision`'s own precedent. `record_render_decisions` writes a `Refused` row on
  whichever axis the refusal wears. `SiteId` was NOT widened — a region owns no execution, and
  `inv-site-keyed-results` is unweakened.
- **The diagnostic**: a new code `render-region-refused` (Error, `Floor::WarnOrDeny`, spanless-allow-
  listed NO — it is SPANNED, at the authored region), payload `{verb, command, routes}`, prose
  `message: None` ⇒ `[unwritten: render-region-refused]`. `routes` is the count that says why this is
  not a per-call disclosure. Defining case at `crates/aid/tests/render-region-refused.loom`.
- **The narrative**: `CollapseKind::RenderRefusal`'s `site: SiteId` became
  `subject: RefusedEditSubject::{Site, Region}` — a new enum in `aid::narrative` rather than two
  nullable fields, so the two identities stay un-flattened. `collapse-mints-narrative` therefore
  still holds for a region refusal, and the diagnostic/narrative cardinality pairing is preserved by
  construction (both walk the same two halves).

A SIBLING CODE rather than a reason arm, deliberately: `28L:rul-reason-enums-not-sibling-codes` binds
same-world reason sentences, and this is a different SUBJECT (one edit many share, versus one
execution). The catalog row's `why` carries that argument.

## §5 — The two priced remedies, landed

- **`30Nd:fnd-plan-steps-stay-publicly-mutable`** — `Plan::steps`/`Plan::regions` are PRIVATE behind
  `steps()`/`regions()` readers. ~90 mechanical call sites across nine crates; no behaviour moved.
  The hazard it closes is real for this lane specifically: a decided plan carries a render plane
  decided AGAINST those values, and a mutating caller would leave the plane describing a plan that no
  longer exists — which a second artifact FORM would then typeset from.
- **`30Nd:fnd-the-canon-does-not-destructure-plan`** — `erasability::canonical_decision` now
  destructures `Plan` exhaustively, each field classified: `steps`/`regions` identity,
  `survival_report` exempt (`Exempt::Timing`, instrumentation), `render` exempt as DERIVED (a pure
  function of the other inputs, all of which reach the canon through the byte-exact `render.apply`).
  A new `Plan` field now stops the canon compiling. A `== regions ==` section joins the canon, gated
  on non-empty so a book with no eligible calls keeps its pre-region digest — which is why
  `cli-plan-summary-line.loom`'s committed digest did not move.

## §6 — The red-first cell, the floor manifest, and the new cases

**`p-x-sentinel-value-conjunct` (red-first, registered).**
`cli::artifact::tests::a_version_mismatched_sentinel_takes_the_source_arm`. A package assigns
`sm_common_loaded='v1'`; a second package's include guard tests for `'v2'`. A real shell compares the
VALUES, finds them different, and takes the SOURCE arm. **MEASURED**: the engine records that
occurrence's route as `LoadRoute::Reused` — `funcenv::sentinel_decides`'s condition 6 reads whether
the target closure's names are BOUND, and they are, because the package was pre-sourced first. The
target assertion (`Taken`) is written down and left failing. Greening trigger: the human's
`rule-sentinel-value-conjunct` ruling (`30N` §4 item 1); horizon `end-of-r31`, `Unscheduled` with its
reason. Sited at the FORM boundary on purpose: a wrongly-empty load account is what would make the
flattened form look available, and the flattened form is the one that would drop the file. Contained
today ONLY because flattening refuses to inline at all — which is why the corner must not be
golden-promoted meanwhile, and is not.

**`floor30-inline-dot-boundary` (the sentinel manifest, AUTHORED, awaiting the mint).** A
which-am-I emitter over two cells: an inert child at a plain `.` position against the same bytes
written where the `.` was; and the same child as the LEFT operand of `||`, where the two shapes part
company (a `.` is ONE command so the `||` covers the whole child; the inlined bytes are N commands
and it covers only the last). Committed as an ordinary round-trip case, GREEN, with its transcript
scope-blessed.

> **OWED TO THE ORCHESTRATOR, two acts in this order**: create an EMPTY
> `crates/cli/tests/floor30-inline-dot-boundary/expected.emitted` (that file's presence is what opts
> the case into gate-9), then, FROM WSL, `mise run bless:floor -- floor30-inline-dot-boundary`.
>
> Why the empty section is not already committed: `test:floor` DOES ride the path-routed
> `gate:full-quiet` when `crates/cli/tests/**` moved, so an unminted section reddens the builder gate
> (measured — it did, once). The book's own header carries these two steps for whoever finds the case
> later.

**New cases (5), all minted by SCOPED bless of NEW cases only, with scope verified after each run:**

| case | what it observes | scope verification |
|---|---|---|
| `crates/aid/tests/render-region-refused.loom` | the region-keyed refusal render | in-process; no bless |
| `crates/aid/tests/artifact-form-refused.loom` | the pre-network refusal render | in-process; no bless |
| `crates/aid/tests/artifact-form-fallback.loom` | the auto-fallback render | in-process; no bless |
| `crates/aid/tests/artifact-publish-refused.loom` | the publication refusal render | in-process; no bless |
| `crates/cli/tests/floor30-inline-dot-boundary/` | the inlining manifest (unmeasured) | `mise run bless -- floor30-inline-dot-boundary` → `bless: gates ok \| e2e 1 blessed`; `git status --short` after: only that untracked dir |
| `crates/cli/tests/emit30-multipart-publishes-its-dependency.loom` | the multipart form through the REAL binary, with `--artifact-dir` | `mise run bless -- emit30-multipart` → `bless: gates ok \| e2e 1 blessed`; `git status --short` after: only that untracked file |

Native pins (10 trials in `cli::artifact`, 4 in `cli::artifact_store`): the corpus's own no-loads
shape flattening unchanged · explicit flattening refusing with its cause · auto explaining instead of
refusing · explicit multipart on one stream naming the STREAM · `placeable`'s eight path cells ·
multipart mirroring a real book-sourced dependency as STRIPPED bytes at its authored path · the same
world on one stream preserving and explaining · the red-first sentinel cell · publication atomicity
(a complete set under one generation name, a second publish leaving the first untouched, a failed
mid-publication leaving nothing, a traversing destination and an oversized set refused).

## §7 — Riders honoured, by name

`inv-determinism` (the module is pure; every collection is a `BTreeMap`/`BTreeSet` or a sorted walk;
no clock or env reaches the lib — `artifact_store` is `main.rs`-private, on the I/O side of the loom
seam) · `inv-no-throw` (no new panic path; every refusal is a closed enum reaching a `Diag`) ·
`inv-must-may`, `claim-tier-gating`, `pin-no-outcome-as-generator` (no mint signature touched; a
form READS the plane and no form outcome re-enters a decision) · `the-render-decides-nothing` (the
forms read `DecidedRender`; `PinnedDefinitions::hoisted()` became a typesetting FUNCTION over the
form-neutral `definitions()`, which is `30Nd` §6.2's split) · `one-settlement-one-world` and
`acts-and-dispositions-mint-together` (untouched; the selector runs before the settlement and reads
none of it) · `no-specialized-shell`, `rul-edit-authored-definition-once`, `pin-no-generated-
specialization` (no form edits a region; region identity is CONSUMED — a region's authored bytes and
its one shared edit survive every form, because no form rewrites book bytes at all) ·
`empty-world-byte-identical` (measured: zero corpus drift) · `silence-licenses-nothing` (an
unresolvable or unheld load root is UNPLACEABLE, never silently skipped) ·
`rul-strawman-formats-no-compat` (every name here renames freely; no adapter) · `stdout-contract`
(plan-producing modes still emit exactly probe-then-apply; the artifact SET is a new stream, not a
new stdout species) · `io-at-edges-only` (`cli::artifact` is pure; `artifact_store` and the publish
call live in `main.rs`) · `rul-probe-writes-only-what-it-owns` at the controller's own edge (every
published path is controller-derived, relative and traversal-free; every file is created exclusively
inside a directory the call made; nothing pre-existing is opened, truncated or removed) ·
`two-binary-floor` / `emit-never-class` / the byte-floor law (no new emitted shape reaches a durable
or paste surface: the artifact bytes are unchanged, and the published dependencies are `dorc strip`
output, which is pure erasure) · `rul-durable-contents-reviewed-before-design` (**HARD STOP
respected and never approached**: `whylog.rs`, `DurableView`, `try_serialize_v2` and the replay
intake are untouched; the new `SpineRenderDecision.region` axis sits in the TRANSITORY arm, which no
`DurableView` names) · grades read-only (`rule-spine-grade-boundary`: no grade representation
touched; every new row is `grade: None` and stamped by `Spine::minted_at` like every other) ·
`TripSpent` (no new plan-producing path was added, so the roster fence and the trip-spend are
unchanged in membership) · `rul-error-authorship-tier` (four new codes, ALL `message: None`; zero
user-facing prose authored) · bundle comments still read back as aid-only `BundleOriginClaim` (no
locator or comment value reaches loading, planning or authority — nothing in this lane consumes one).

## §8 — Drift enumeration

**EMPTY.** Triangulated at the final tip:

- `mise run test:e2e` — 178 cases pass, of which 176 are the pre-existing corpus, byte-identical;
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`, `git status --porcelain` clean
  afterwards, so no golden would have been rewritten;
- `mise run test:looms` — every committed transcript is still a render fixpoint.

Why zero was achievable rather than lucky, since the brief expected drift: the forms do not rewrite
book bytes. `plan.sh` is the same projection under every form, so the only case in the corpus with a
resolved book-sited load (`pin28-variable-resolved-source-loads`) gains a NOTE on stderr and not one
artifact byte — and gate-3 only fails on undeclared ERROR-severity diagnostics.

The three `load30-*` XFAILs are unchanged and still XFAIL; `floor30-dot-loader-function-errexit` is
byte-identical.

## §9 — Deviations, each OPEN, none self-endorsed

1. **`dev-flattening-refuses-rather-than-inlining`** — the flattened form does not inline; a book
   with a book-sited load makes it unavailable. Argued from `30Ib` §15 and `30I` §7.1, and it is what
   keeps the goldens off the unproven construct. It also means the ONLY form that ever differs from
   today's output is multipart, and only in which FILES accompany the plan. If the conductor wants
   inlining in this arc, it lands after the floor mint and carries its own drift.
2. **`dev-posture-is-the-artifact-dir-not-the-terminal`** — see `tc-piped-stdout-vs-named-artifact-
   dir` in §2. An interpretation of a TYPED rule; flagged rather than assumed.
3. **`dev-probe-accepts-the-form-flags`** — `--artifact-dir`/`--form` are accepted (and inert) on
   `dorc probe`, refused on `bundle` and `why`. The reason is the harness's shape: `framed_results`
   drives `dorc probe` with the case's own DORC_FLAGS, so refusing there makes any form-flagged case
   unexpressible end-to-end. The engineering reading is that probe is the round-trip's own first
   PHASE and a flag shaping something the phase does not reach is inert rather than wrong — but the
   alternative (refuse, and lose the e2e coverage in §6) is a real option and is the conductor's.
4. **`dev-multipart-mirrors-rather-than-rewrites`** — `30I` §7.1 mode 2 says each contracted root is
   "replaced by a source of a generated bundle under a small dependency directory". As built, the
   generated bundle IS the mirrored file at the authored relative path, so the authored operand
   already names it and nothing is replaced. Believed to serve the rule's purpose better than a
   rewrite (no book byte moves, nested operands resolve), but it is a reading of a [TYPED direction]
   sentence and should be looked at.
5. **`dev-storage-path-is-not-the-published-path`** — `BundleFile::storage_path` stays the inert
   archive name `30Ib` §17 made it; the multipart PUBLISHED path is computed separately by
   `artifact::placeable` from `snapshot.source_paths()`. Two names for two surfaces, deliberately;
   flagged because a reader may expect one.
6. **`dev-flattened-boundary-priced-and-declined`** — see `tc-flattened-section-boundary` in §3.
7. **`dev-exit-16-reused-after-its-first-tenant`** — `EXIT_ARTIFACT_UNSERVABLE = 16` takes the code
   `EXIT_UNANNOUNCED_CROSS_CUSTODY` vacated when `30Ib` §16 deleted the cross-custody refusal.
   Pre-user, so no compat surface; flagged because a reader of older notes will find 16 described
   differently.
8. **`dev-publication-generation-is-a-fresh-directory`** — each run publishes `artifact-<NNNN>/`
   under the named root rather than replacing a stable path, so "a plan may never point at a sidecar
   from an earlier generation" holds by never mutating a published generation. Exact naming is
   builder latitude per §14; flagged because a stable `latest` entry point is an obvious later want
   and would need the temp-then-rename dance `whylog_store`'s header prices.
9. **`dev-collapse-kind-render-refusal-reshaped`** — `CollapseKind::RenderRefusal`'s `site` field
   became a `subject` enum. A shape change to an existing narrative class, taken rather than minting
   a second class, because the census is a no-wildcard match and two classes for one collapse would
   split its mint schedule.

## §10 — `tc-*` judgment calls, flagged UP

- **`tc-piped-stdout-vs-named-artifact-dir`** (§2) — the interpretation of a TYPED rule. The sharpest
  item here.
- **`tc-flattened-section-boundary`** (§3) — close the preamble section, at the price of one scoped
  bless over every guard-preamble case; or leave it to land with the inlining.
- **`tc-probe-mode-form-flags`** (§9 dev-3) — accept-and-ignore on `probe`, or refuse and lose the
  compound e2e.
- **`tc-multipart-has-no-published-tree-assertion`** — the e2e case drives publication through the
  real binary but asserts only stdout, because the runner captures no filesystem. The mirrored tree
  is pinned natively instead. Whether an e2e gate should learn to assert a published directory is a
  harness question, not this lane's.
- **`tc-form-flag-spelling`** — `--form flattened|multipart|preserved-book-tree` and
  `--artifact-dir` are builder latitude per §14, but they are user-facing tokens and the help page
  does not document them (see §11).

## §11 — Proposed steering text (conductor's to place; NOT edited by this lane)

**`spike/crates/cli/CLAUDE.md`, new bullet under **Law**:**

> - **artifact-forms-derive-from-one-structure** (`30I:step-7-reify-plan-artifact-forms`) —
>   `cli::artifact` settles ONE `Selection` (form + fallback + dependency files) from
>   authored-before-contact inputs, and `Selection::with_plan` binds it to the plan projection. The
>   stdout stream and the published tree both READ that `ArtifactSet`; there is deliberately no
>   second assembly of the same bytes to fall back to. A form's dependency LAYOUT is the authored
>   relative one, mirrored under the artifact root, because that is what makes every authored `.`
>   operand — the book's and every nested one — resolve on the target unchanged (`30I` §7.4): the
>   availability question is therefore a PATH question, and an absolute or escaping controller path
>   makes the form unavailable rather than fudged. `plan.sh` is byte-identical under all three
>   forms; a form is about where the generated files live, never about what the plan says.
>   FLATTENING REFUSES rather than inlining while the textual-inlining floor cell is unminted
>   (`floor30-inline-dot-boundary`), so an explicitly named form that cannot be served refuses
>   pre-network and `auto` falls back and SAYS SO — never a silently different form (`30I` §14).

**`spike/crates/plan/CLAUDE.md`, appending to `pinned-definitions-are-the-artifact's-binding`:**

> `PinnedDefinitions` is SPLIT along the line a second artifact form forces (`30Nd` §6.2): `invoked`
> is the DECISION (which body a guard calls, under what name — what the Spine records, and where a
> misalignment is pope-sin tier), `definitions()` is the ordered form-neutral material, and
> `hoisted()` is one form's sh typesetting over it. A form that lays its dependencies out differently
> re-typesets from the same bindings rather than re-deriving them.

**`spike/crates/plan/CLAUDE.md`, appending to `the-render-decides-nothing`:**

> A refused edit carries BOTH identities and exactly one is populated: `RefusedEdit.leaf` for an
> execution, `RefusedEdit.region` for the one authored edit many executions share
> (`30N:rul-region-refusal-discloses-region-keyed`). The three disclosure surfaces read whichever the
> refusal wears — `render-heredoc-refused` at a leaf, `render-region-refused` at a region — and
> NEVER smear one region's refusal across its contributing invocations, which would report N
> refusals for one edit and point N readers at calls that did nothing wrong
> (`271:rul-sin-ordering`).

**`spike/crates/core/CLAUDE.md`, appending to the Spine section:**

> - **a-second-key-axis-never-widens-siteid** (`30N:rul-region-refusal-discloses-region-keyed`) —
>   `SpineRenderDecision` carries `site` AND `region`, at most one populated, on
>   `SpineRegionDecision`'s precedent. A region owns no execution, so keying its row by a
>   contributing invocation's `SiteId` would be the smear the ruling forbids, and widening `SiteId`
>   to hold a region would weaken `inv-site-keyed-results` for every other consumer. A new identity
>   gets a new axis.

**`Research/plans/30I`, "Where the build stands"** — move `step-7-reify-plan-artifact-forms` from the
owed list to landed, and re-cut the `Not built` line to: the textual-inlining floor MEASUREMENT (the
manifest is authored and awaits `bless:floor`), the flattened form's inlining itself, and
`step-8-promote-executable-specification`'s XFAIL promotion and e2e lowering.

**Owed prose (conductor/human, not builder):** four registers render `[unwritten:]` —
`render-region-refused`, `artifact-form-refused`, `artifact-form-fallback`,
`artifact-publish-refused`. Each catalog row's `why` names what its help register should say. The
help page also does not document `--artifact-dir` or `--form`; documenting a flag is authorship, and
`30Ib` §9 dev-5 already has this class of debt queued.

## §12 — Verification

- **`mise run both gate:full-quiet` — BOTH LEGS GREEN, rc=0, foreground**, at tip `b2aa26cf`,
  Windows leg first (`preflight-bounds-before-spend`). Quiet is silent on success by design.
  (First attempt failed on the WSL leg's untrusted config — `wsl-trust-per-worktree`; trusted the
  worktree and the nested `spike/verify/aeneas` config, then green.)
- `mise run test` — **2488 passed, 2 skipped** (Windows); **2484 passed, 2 skipped** (WSL; the
  4-test delta is the pre-existing `cfg`-gated coverage difference `30Nc` also measured).
- `mise run test:e2e` — 178/178, of which 176 pre-existing and byte-identical.
- `mise run clippy` — clean, `-D warnings`. `mise run check-quiet` — rc 0, all four gates.
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`; clean tree afterwards.
- `mise run xfail:census` — renders; **9 live pins, 1 reserved** (was 8+1; the new one is
  `p-x-sentinel-value-conjunct`). No horizon expired.
- **Comment budget: 30** added inline `//` lines, by
  `git diff d9afde93..HEAD -- "*.rs" | grep -cE "^\+\s*//($|[^/])"` = 90, minus 60 `//!` module-doc
  lines (three new modules: `cli::artifact`, `cli::artifact_store`, and the doc widenings). Exactly
  at the briefed ≤30. The rationale that would have sat inline lives in the `///` docs of the items
  it explains.
- **Golden drift: ZERO** — §8.

## §13 — Handoff to `30I:step-8-promote-executable-specification`

1. **The floor mint is the first act, and it is orchestrator-only.** §6's two steps, from WSL. Until
   it runs, `floor30-inline-dot-boundary` is an authored manifest nobody has asked a shell about, and
   the flattened form's inlining stays unbuilt on purpose. Step 8's own remit already names "mint
   the owed floor cell for textual inlining before flattening rests on it" — this is that cell.
2. **The `load30-*` XFAILs are untouched and still XFAIL.** Nothing in this lane changed what they
   assert or what they measure; their promotion is step 8's, unblocked by nothing here.
3. **The corpus is byte-identical**, so step 8 inherits no drift and no pending bless.
4. **The e2e lowering step-8 owes has a new entry**: `emit30-multipart-publishes-its-dependency` is
   the ONE compound case for the artifact interaction, and everything finer-grained is already at its
   ownership seat (14 native trials, §6). If step 8 lowers more e2es, this is the shape to copy.
5. **`p-x-sentinel-value-conjunct` blocks nothing** but must not be promoted by step 8: its greening
   trigger is a human ruling, not a lane.
6. **The four unwritten registers** are the prose queue this lane adds to `30N` §4 item 9.
7. **What a THIRD form would cost, if one is ever wanted**: `Selection` is the seam. A new form adds
   an `ArtifactForm` variant, an availability predicate in `select`, and its own typesetting over
   `PinnedDefinitions::definitions()`. Nothing in `plan` needs to move, because no form rewrites book
   bytes.
