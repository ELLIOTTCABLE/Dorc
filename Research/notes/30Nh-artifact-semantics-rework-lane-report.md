# 30Nh — the artifact-semantics rework lane: `30Ng`'s four work items

> Tier: **LLM-authored, builder (Opus-class)**, lane `ai/r30-artifact-semantics` from
> `ai/r30-conduct@e4314a8b`. Ten commits, `72e83f3e..5d4da347`.
>
> Read with: **`notes/30Ng`** (THE charter — the human's typed rulings, plus the two mid-lane
> addenda this lane received and applied) · `notes/30Ne` / `notes/30Nf` (the as-built this reshapes)
> · `plans/30I` (the arc spec; its bundle/cwd/multipart machinery persists under the new defaults) ·
> `notes/30N` §3 (the endorsements the sitting reversed).
>
> HEADLINE: the default materialized form now BUNDLES a book's dorc-lang subgraphs and the generated
> plan SAYS where its imports point; a kept stdout carries a complete plan or refuses pre-network;
> and 37 goldens moved, every one of them enumerated and classified in §6.

## §1 — The semantics table as built

The artifact is a COLLAPSED SINGLE RESOURCE, and which stream carries it derives from stdout
interactivity read at the cli edge (`cli::artifact::artifact_stream`, `main::stdout_posture`).

| stdout | `--artifact-dir` | posture | what `auto` does |
|---|---|---|---|
| non-interactive | named | **REFUSE**, pre-network | `artifact-form-refused`, cause `two-artifact-claimants` |
| non-interactive | absent | `PipedArtifact` | the complete flattened stream, or **REFUSE** with cause `incomplete-single-stream` |
| interactive | named | `Materializable` | the bundled multipart tree to the directory, the render to the terminal |
| interactive | absent | `TerminalRender` | the flattened stream where it is available, else the preserved tree WITH its explanation |

Three properties are worth stating because each is a place the table could have been softer:

- **Constant across every flag-form.** `--form preserved-book-tree` on a kept stream refuses too
  (`naming_the_preserved_tree_does_not_override_a_kept_stream`): the posture is semantics, not a
  default a flag overrides. Only the `TerminalRender` cell has a fallback at all, and it has one
  because nothing there is being kept to run later.
- **The refusal names the CLAIMANTS, not a winner.** This is `30I` §2.5's collapsed-resource rule
  applied one level up, to the artifact rather than to a stream: a non-interactive stdout is an
  IMPLICIT claim, `--artifact-dir` is an explicit one, and ranking them silently would decide on the
  user's behalf which of two competing complete artifacts they meant.
- **Injection, never absence.** `StdoutPosture` is an edge value threaded into `run()`; the real edge
  asks `IsTerminal`, and both cells are drivable. The kernel below the edge asks nothing
  (`inv-determinism` · `io-at-edges-only`).

### The posture pin, and the honest limit on its coverage

`DORC_STDOUT_POSTURE` (`interactive` | `piped`; anything else asks the terminal) is the harness's
injection point, sited beside `FIXTURE_CLOCK_ENV` and on exactly its footing. The e2e battery sets
`interactive` for every round-trip drive, and that is not a fiction of convenience: the battery reads
a RENDER (it captures stdout to compare bytes) while driving the artifact SET to `--artifact-dir`,
which IS the terminal cell. Left to the true answer, every `ARTIFACT_SET` case would sit in the
incoherent cell and refuse before rendering anything to compare.

**COVERAGE LIMIT, disclosed rather than papered over**: the kept-stream cell's two observable
behaviours are both PRE-NETWORK REFUSALS with empty stdout, and the round-trip battery hard-fails
empty output before any lens (`crash/empty guard`). So the piped cell is pinned natively
(`which_stream_carries_the_artifact_is_a_closed_table`,
`a_kept_stream_refuses_where_a_terminal_render_falls_back`,
`naming_the_preserved_tree_does_not_override_a_kept_stream`) and has NO end-to-end case. Making one
expressible is a harness capability (an expected-empty-stdout lane), named here and not built.

## §2 — The bundling boundary rules, and their evidence

`BundleRoot::bundled()` composes ONE text per book-sited root: the entry's stripped bytes with every
ABSORBABLE nested `.` replaced, in place, by the bytes that `.` would have loaded — recursively,
depth-first, descending by line so an earlier substitution cannot move a later index.

**What licenses the substitution, exactly.** `floor30-inline-dot-boundary` MEASURED one operation:
an inert child's bytes written where its `.` stood behave identically to the `.` under dash∩posh
(cell 1). It measured a DIVERGENCE at the `||` position (cell 2), and it measured nothing at all
about moving those bytes anywhere else. So the rules are:

1. **Positional, always.** The bytes stand where the `.` stood. The author's include guard therefore
   survives verbatim around them, which is the whole reason it cannot be a concatenation: prepending
   a dependency ahead of its sourcer loads a package the author asked to load only when absent.
2. **A nested `.` inside a dorc-lang file is a top-level `Simple` by construction** — `load_inert`
   admits a `.` only as a whole top-level item or a guard-branch item, never as an `&&`/`||` operand
   — so the position check there is about the LINE, not the grammar.
3. **Four conditions gate the line** (`absorbable_line`): the child names a locus; that locus is the
   WHOLE of its authored line; exactly one stripped line survives from that authored line; and the
   surviving line still reads as the load it came from. Any failure leaves the `.` verbatim and the
   child is recorded in `separate()` — a file the artifact still has to carry.
4. **A BOOK `.` is gated harder** (`BookLoad::absorbable`): top-level `Simple`, no redirects, alone
   on its line. A book is arbitrary sh, so none of (2) is free there.

Everything outside that is the conservative answer, which is the shape `fnd-loader-function-errexit-
diverges` already left the nested boundary in. The floor-refuted generated-loader-function is not
revived, and no new floor measurement was minted (`bless:floor` never ran; both `floor30-*` cases are
byte-identical).

**What the default emits.** One bundle per book-sited root, named by replacing the mirrored path's
`.sh` suffix (and a `.dorc` segment ahead of it) with `.dorc-bundle.sh`, plus a mirrored file at its
authored path for anything `separate()` names. The name is STRAWMAN and renames in place
(`rul-strawman-formats-no-compat`); what it must NOT be is the authored spelling, because the file is
generated — stripped bytes composed by the engine — and publishing it under the author's name would
put bytes on a target under a name that promises to be somebody's file.

**Both ends of the axis stay reachable by name** (`30Ng` §5): `--form flattened` is one emission with
the subgraph in the stream; `--form mirrored-tree` is the no-flatten end — every reached source at
its own authored spelling, no import re-said, which is the placement machinery this arc inherited,
kept as an explicit mode. The default (`multipart`) sits at neither.
`both_ends_of_the_bundle_point_axis_are_reachable_by_name` drives the pair over one world.

## §3 — The import-rewrite disclosure shape

`plan::ImportEdit` has two shapes and ONE mint: it is an INPUT to `Plan::decided`, settled by
`cli::artifact::Selection` from authored-before-contact inputs. There is no post-process.

- `Repoint { ast: <operand WORD>, path }` — only the operand moves, so it works wherever an author
  put the load: inside a guard, as an `||` right operand, inside a subshell.
- `Inline { ast: <the `.` COMMAND>, sh }` — the bundle's own bytes stand where the `.` did, licensed
  only at the measured shape.

It reaches three places from that one decision, and
`a_rewritten_import_reaches_the_bytes_the_surface_and_the_plane` asserts all three together:

1. **The artifact bytes**, through the ordinary span-edit machinery, SELF-COMMENTED (the shared
   provenance line says "elided", and a re-pointed load is not elided — it still runs, from somewhere
   the artifact carries).
2. **The plan surface**, as `plan-import-rewritten` (Note, `Floor::None`, payload `{verb, names}`,
   `message: None` ⇒ `[unwritten:]`), sited at the authored `.`. `two-surfaces` puts the account here
   and the bytes there; `rul-attention-honesty` is what makes it a disclosure rather than an option.
3. **The decision plane**, as `RenderDecision::ImportRewritten`, site-less and region-less on
   `DefensiveEmission`'s precedent (`a-second-key-axis-never-widens-siteid`: an import edit belongs to
   a book line, and a new identity gets a new axis rather than borrowing one that means something
   else).

**The license is scoped narrowly and says so at the type.** The authored BOOK is never written; every
byte of it that reaches the artifact reaches it verbatim. What moved is where a GENERATED plan's
import points — a durable, but not an OFF-RAMP durable (`30Ng` §5, human-typed).

## §4 — Addendum acknowledgements

**Addendum 1 (the piped plan is a REVIEW SURFACE)** — APPLIED. Every durable sentence this lane wrote
frames a non-interactive stdout as a stream the user is KEEPING to review — a pager, an editor, a
file — and the strengthened obligation ("what the reviewer approves is exactly what executes") is the
argument the `PipedArtifact` doc-comment and the refusal both rest on. No pipe-to-ssh or
pipe-to-remote-execution example was written at any point; none needed renaming.

**Addendum 2 (two sections + a divider)** — PARTLY built, and the unbuilt half is a soundness
refusal rather than an omission. What landed: the artifact's lifted material is now CLOSED by
`render::apply::lifted_section_close()`, so a reader knows where the low-attention section ends and
the book they came to review begins. That discharges `30Ne:tc-flattened-section-boundary`, which is
where the addendum said it should land, and it is emitted uniformly (any form with a non-empty lifted
section) so the drift is paid once — which is 34 of the 37 moved goldens.

What did NOT land, with the reason stated so it can be overruled cheaply: **lifting the oracle
BUNDLES to the front is not safe under this engine, and I did not build it.** Moving a bundle above
the book code that precedes its `.` changes which definition is live at book lines between the top
and the load point — and `pinned-definitions-are-the-artifact's-binding` exists because a
misalignment there swaps WHOSE judgment executes, which is pope-sin tier. The analysis has already
frozen its answers against the AUTHORED positions (`visibility-is-full-positional`), so an emission
that re-orders bindings makes the plan describe a program the artifact is not. Three concrete
counter-examples the corpus already contains: a book that defines a name the package redefines
(lifting inverts the winner); a `.` inside a subshell whose definitions must die at `)`
(`load30-two-point-frames`); a package whose own operands read a root the BOOK sets
(`load30-rooted-shared-dependency`).

A closed enumeration COULD license the lift per-book (the `.` is top-level; nothing the book binds
above it collides with what the bundle binds; the bundle is fully absorbed; the bundle's top level
reads no parameter it does not bind) — that is `rul-happy-path-is-a-closed-set`'s exact shape, and it
is what I would build next. But it is a licence surface, not a layout one, and it wants the
settlement to model the lifted position rather than the emission asserting it: the same feedback
`30Ng:attn-render-refusal-feeds-the-spine` is queued for. **Flagged as `tc-bundle-lift-needs-the-spine`
(§9), OPEN.** Meanwhile the single stream keeps the bundle POSITIONAL, which is the measured cell —
so the addendum's attention goal is met for the preamble and unmet for the bundles.

## §5 — The routes account, loud and complete

`SpineRegionDecision.routes` is now `core::spine::RegionRoutes` — deliberately NOT an `Account`:

- **The keyed set is COMPLETE.** The cap (`309:law-spine-operands-capped`) bounds an operand list
  whose length is a property of the WORLD; a region's contributor population is a property of the
  analysed unit, bounded by the census, which is bounded by `cfg::inline_budget`'s per-book node
  budget. It is also the answer two pull surfaces ask for BY NAME, and a sampled contributor set
  points a reader at some of the calls that share an edit while silently omitting the rest.
  RegionDecision is a transitory (never durable) species, so nothing here reaches operator disk.
  **Flagged: this is a deliberate carve of a `309` law (§9 `dev-region-routes-uncapped`).**
- **The unkeyable route is RETAINED, not filtered.** `UnkeyedRegionRoute { ast, reason:
  RegionRouteUnkeyed::NoPlanLeaf }` keeps the identity it has and states the one it lacks. The
  retired shape was a `filter_map`, which left an account that read as complete and was not.
- **The consequences ride the same value.** `Plan::decided`'s `live_regions` now asks
  `every_route_is_keyed()` (an account it cannot ask a neutralisation question of keeps its edit);
  `render-region-refused`'s `routes` payload reports `total()` rather than the shown count.
- **The why plane reaches every contributor.** `region_invocation_lines` walks `routes.asts()`, which
  an unkeyable route carries too, and `region_lines_executed_by` matches call-ward against the same
  union.

Pins: `an_invocation_with_no_plan_leaf_stays_in_the_contributor_account` (drives the real census, then
starves `leaf_of` of one of two real routes — a `filter_map` over a hand-built input would prove only
that the input was short) and `an_unkeyable_contributor_still_names_its_line`.

**Why the reach pin is at the VALUE seat and not against rendered bytes**: the register these values
fill is `[unwritten: why-reason-region-universal-over]` and legally stays that way
(`prose-provenance-states`), so a render assertion would pass by measuring a placeholder.

## §6 — Golden enumeration, classified

**37 cases moved. Every one is in this table; nothing outside it moved** (verified by
`git status --short` immediately after the scoped bless, against the exact case-path list
`bless:dry` itself printed). `bless:floor` never ran; both `floor30-*` cases are byte-identical.

### (a) The lifted-section boundary — 34 cases, +2 lines each

Ruling: Addendum 2 (`tc-flattened-section-boundary` closed). Correct because the added bytes are a
`#`-comment line plus a blank, emitted only where a lifted section EXISTS, and the artifact's
executable content is unchanged — the run sets did not move in any of them.

`contest28-polyfill-guard-defers-to-the-oracle` · `context-entry-wrapped-guard` ·
`emit30-definition-vector-munges-everything` · `emit30-two-live-verdicts-under-one-name` ·
`exec-subst-body-nonleaf` · `frame30-a-regional-decline-is-a-decline` ·
`frame30-region-removal-dies-at-the-paren` · `guard23-fallthrough-canttell-runs` ·
`guard23-fallthrough-drift-runs` · `guard23-mutator-fails-book-continues` ·
`guard23-nounset-book-survives` · `guard23-ternary-flagship` · `guard23-var-namespace-isolated` ·
`guard26-classed-decline-guards-below` · `guard26-diverged-wall-guards-below` ·
`guard26-unmodeled-wall-guards-below` · `pin28-closure-travels-with-the-definition` ·
`pin28-contested-helper-resolves-by-last-wins` · `pin28-helper-package-entrypoints-lift` ·
`pin28-reach-arm-death-walls-total` · `pin28-survival-body-death-walls-total` ·
`pin30-swapped-entrypoints-source-the-helpers` · `region30-drifted-route-guards-the-shared-region` ·
`region30-mixed-body-splits-the-decision` · `strawman24-alias-provides` · `strawman24-alias-symlink` ·
`strawman24-reach-crossauthor` · `strawman24-reach-static-service` · `strawman24-survive-multiwall` ·
`typeless-floor-converged-elides` · `whygallery-wall-guards-downstream` · `whygallery-webhost-whole`
(32) — plus the two below, which carry the boundary AND an import edit.

### (b) A re-pointed import, multipart — 3 cases

Ruling: `30Ng:rul-bundle-at-dorc-lang-boundaries`. Correct because the operand now names a file the
same run PUBLISHES, and the harness's new import gate proves it does (`§7`). **`expected.ran` did not
move in any of the three** — the artifact still runs exactly what it ran, which is the strongest
statement available that the rewrite changed where a load resolves and nothing else.

| case | what moved |
|---|---|
| `load30-rooted-shared-dependency` | two `.`s → `./alpha.dorc-bundle.sh`, `./beta.dorc-bundle.sh`; each bundle absorbed `common` |
| `load30-two-point-frames` | both `.`s → ONE `./entry.dorc-bundle.sh` (two roots, one destination, deduped) |
| `load30-subshell-errexit-fallback` | the top-level `.` and the `\|\|`-operand `.` both re-pointed — the operand-only shape works where inlining may not |

### (c) An inlined bundle, single stream — 1 case

`pin28-variable-resolved-source-loads` — `. "./$PKG.oracle.sh"` becomes the bundle's own bytes under
`# dorc bundle: …`. Ruling: the same, at the `PipedArtifact`/`TerminalRender` end. Correct because the
`.` is a top-level command alone on its line, which is `floor30-inline-dot-boundary`'s measured cell,
and the operand's own value-flow resolution is what the case exists to pin — unchanged.

### (d) The demonstration case — 1 case, re-authored

`emit30-multipart-publishes-its-dependency` — the import edit, the lifted boundary, the
`artifact-set: published` frontmatter, a declared `expect-diagnostic: plan-import-rewritten`, and a
rewritten case header (the old one described the retired mirroring). Its transcript now shows
`. './wombat.dorc-bundle.sh'`, and the harness proves the generation carries that file.

### New material (no golden moved; minted, not re-blessed)

`crates/aid/tests/plan-import-rewritten.loom` — the defining case for the new code, published through
`dorc-loom publish` (in-process render, no binary, no execution). `message: None`.

## §7 — Harness changes

- **`artifact-set: published`** joins `dorc_loom::FRONTMATTER_KEYS` (23 keys) with `run_lane: true`,
  materializing the dir form's `ARTIFACT_SET` marker. The value is a CLOSED vocabulary with its own
  refusal (`ArtifactSetDeclaration::parse`), on `Normalizer::parse`'s precedent — an unread
  declaration is exactly the silence `30Nf:dev-artifact-set-is-dir-form-only` left open, on the very
  case minted to demonstrate the capability.
- **The published tree OBSERVES its own imports**: `unresolved_generated_imports` reads every LITERAL
  relative `.`/`source` operand out of the published plan and requires the generation to carry it.
  Literal-only, and the cut is honest — an operand the plan builds from a variable is resolved by the
  target's shell against values the artifact sets, which this seat holds no model of. What it covers
  is exactly the class an import rewrite produces. This is the assertion `30Nf`'s burn asks for: a
  case that can pass with the feature OFF is not a demonstration.
- **The replay command DISCLOSES the artifact stream** (`--artifact-dir=$ARTIFACT_DIR`), because
  naming one decides which FORM the run takes; a shell variable rather than the scratch path, since a
  committed transcript must be a fixpoint.
- **The counterfactual rails get their own world.** Gate-5 (argv echo) and gate-6 (dual rail) compare
  an ARTIFACT against the BOOK, and since the bundling those two no longer resolve their imports in
  one place. `counterfactual_root` lays the published generation over a copy of the case's authored
  top-level files, so both rails are runnable and a delta is a delta. **This is NOT a relaxation of
  `an-artifact-set-runs-from-its-own-generation`**: `exec_check` still runs the published plan from
  the generation ALONE, and that is where self-containment is asked. Without it, gate-6 reported
  `apply-only: common alpha first` — a false finding about the engine, caused by the bare rail's shell
  exiting at its first unresolvable `.`.
- **`DORC_STDOUT_POSTURE=interactive`** on every `Harness::dorc` invocation (§1).

**The three dir-form `ARTIFACT_SET` markers were ALREADY EMPTY** — the charter's "stale renamed
run-set bytes" item is a no-op. `f8401606` renamed three `head-expected.ran` files to `ARTIFACT_SET`
and all three were 0 bytes on both sides of the rename (`git ls-tree` confirms the empty blob
`e69de29b`). Recorded rather than silently skipped: the *finding* is that those `head-expected.ran`
assertions were vacuous before the promotion, not that bytes needed removing.

## §8 — Verification

- **`mise run both gate:full-quiet` — BOTH LEGS GREEN, rc=0, FOREGROUND**, at tip `5d4da347`,
  Windows leg first (`preflight-bounds-before-spend`). WSL trust taken for the worktree and the
  nested `spike/verify/aeneas` config first (`wsl-trust-per-worktree`).
- `mise run test` — **2505 passed, 2 skipped**.
- `mise run test:e2e` — **177/177**. `mise run test:looms` — **285/285**.
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`; `git status --porcelain` EMPTY
  afterwards, so no golden would be rewritten.
- `mise run xfail:census` — **9 live pins, 1 reserved; NO horizon expired.** Both ringfenced pins
  (`p-x-sentinel-value-conjunct`, `p-x-loop-population-closes-over-literal-members`) present and
  untouched; the census is unchanged by this lane.
- `mise run clippy` clean under `-D warnings`; `mise run check-quiet` rc 0.
- **Comment budget: 33** added inline `//` lines —
  `git diff e4314a8b..HEAD -- "*.rs" | grep -cE "^\+\s*//($|[^/])"` = 39, minus 6 `//!` module-doc
  lines. Within the briefed ≤35. The rationale that would have sat inline lives in the `///` docs of
  the items it explains.

## §9 — Deviations, each OPEN, none self-endorsed

1. **`dev-bundle-lift-not-built`** — Addendum 2's front-lift of oracle BUNDLES is not built; only the
   section boundary is. The argument is §4's, and it is a soundness argument rather than a scope one:
   an emission that re-orders bindings makes the plan describe a program the artifact is not. If the
   conductor wants the lift in this arc, it needs the closed enumeration AND the settlement to model
   the lifted position — see `tc-bundle-lift-needs-the-spine` below.
2. **`dev-region-routes-uncapped`** — `RegionRoutes` carves `309:law-spine-operands-capped` for this
   one species. The human's directive offered exactly this ("uncap for region routes, or chain the
   account"); the carve is documented at the type with its bound and its non-durability. Lifts
   cleanly to a chained account if the conductor would rather keep the law universal.
3. **`dev-stdout-posture-is-an-env-pin`** — the harness injects the edge fact through
   `DORC_STDOUT_POSTURE` rather than a CLI flag. `30I:lean-every-detected-mode-is-also-requestable`
   (a direction note, not-now) says a detected mode should also be REQUESTABLE, which argues for a
   flag; I did not mint one, because a user-facing flag needs a name and a help row, and the help page
   already owes `--artifact-dir`/`--form`. Cheap either way.
4. **`dev-fourth-artifact-form-minted`** — `ArtifactForm::MirroredTree` is a FOURTH semantic form
   where `30I` §7.1 names three. Taken because `30Ng` §5 rules both extremes fully supported and the
   old behaviour otherwise became unreachable. `30I` §14 gives builders the NAMES; the fourth mode
   itself is the ruling's, not §7.1's, so it is flagged.
5. **`dev-bundle-name-drops-a-dorc-segment`** — `alpha.dorc.sh` → `alpha.dorc-bundle.sh` rather than
   `alpha.dorc.dorc-bundle.sh`. Two rules where one would do, for readability of the goldens a human
   reviews. Renames in place.
6. **`dev-counterfactual-rails-see-a-union-world`** — §7. Argued as the only world in which both
   rails are runnable; the strict question stays at `exec_check`.
7. **`dev-piped-cell-has-no-e2e`** — §1's coverage limit. Native-only, because the battery cannot
   express a case whose stdout is legitimately empty.
8. **`dev-why-world-carries-no-import-edits`** — `WhyWorld` passes `&[]`, so a why report explains the
   run's DECISIONS without its import edits. On `308b` F7's precedent (the why driver already models a
   narrower world); disclosed at the call site.

## §10 — `tc-*` judgment calls, flagged UP

- **`tc-bundle-lift-needs-the-spine`** — the sharpest item here, and the one the conductor most needs
  to rule. Front-lifting a bundle is an EMISSION decision that changes which definition is live at
  book lines it moved past, and the analysis has already frozen its bindings against the authored
  positions. Either the emission proves a closed enumeration (and the proof becomes a licence surface
  reviewed as one), or the lifted position feeds back into the settlement — which is
  `30Ng:attn-render-refusal-feeds-the-spine`'s own shape, one species over. Until then, the single
  stream keeps bundles positional and the attention goal is met only for the preamble.
- **`tc-incoherence-table-is-conductor-derived`** — §1's first row (piped stdout + `--artifact-dir`
  ⇒ refuse) is the conductor's `[PROPOSED, veto-eligible]` derivation, built to as briefed. Its
  practical bite is worth naming: `dorc plan --artifact-dir out book.sh > plan.log` — an entirely
  ordinary CI shape — now REFUSES. That follows from the human's typed reversal
  (`adj-endorse-artifact-stream-reading` NACKED), so the alternative is not a re-reading but a
  different ruling.
- **`tc-book-code-loads-are-not-in-the-model`** — the charter's worked example has a book sourcing
  BOOK code, and this engine does not acquire that at all: `book_reached` admits only
  contract-satisfying dorc-lang, so a book `.` naming ordinary sh is outside the load model and its
  site walls, exactly as it always has. I therefore did NOT make such a `.` force the single-stream
  refusal, and the reason is a named law rather than a preference:
  `ambient-dependencies-are-ordinary-shell` (human-typed) says visibility creates no new correctness
  category and a refusal is not the conservative reading of an ambiguity. The consequence is that the
  refusal is reachable today only through a book-sited dorc-lang load outside the measured shape.
  When book-code acquisition lands, this is the seat that has to be revisited.
- **`tc-divider-is-artifact-plane-prose`** — the addendum called the section header "structure-only
  prose (`[unwritten:]`)", but `artifact-plane-strings-stay-out` puts artifact comment bytes in
  `plan/src/render.rs` as hardcoded emitters (the guard-preamble banner's own precedent), NOT in the
  registry. I followed the standing law. If the conductor wants the divider loom-editable, that is a
  law change for the whole artifact comment family, not a lane item.
- **`tc-import-note-is-advisory-routed`** — `plan-import-rewritten` rides `report_at`'s advisory
  route, so an `apply` does not print it. Argued: it is a Note about the emission this run chose, not
  a correctness give-up. A reader who wants it on every surface would move it to `report`.

## §11 — Proposed steering text (conductor's to place; NOT edited by this lane)

**`spike/crates/cli/CLAUDE.md`** — REPLACE `artifact-forms-derive-from-one-structure` wholesale (the
committed bullet ends by describing itself as "the pre-rework as-built"):

> - **artifact-forms-derive-from-one-structure** (`30I:step-7-reify-plan-artifact-forms`, re-cut by
>   `30Ng`'s rulings) — `cli::artifact` settles ONE `Selection` (form + fallback + dependency files +
>   IMPORT EDITS) from authored-before-contact inputs, and `Selection::with_plan` binds it to the plan
>   projection. The stdout stream and the published tree both READ that `ArtifactSet`; there is
>   deliberately no second assembly of the same bytes to fall back to. WHICH STREAM carries the
>   artifact derives from stdout INTERACTIVITY, injected at the edge
>   (`30Ng:rul-piped-stdout-carries-a-full-plan`, human-typed): a non-interactive stdout is a stream
>   the user is KEEPING to review, so it carries a COMPLETE plan or the run refuses pre-network, under
>   every flag-form — and naming `--artifact-dir` beside it claims one artifact twice, which refuses
>   naming both claimants rather than ranking them (`30I` §2.5's collapsed-resource rule, one level
>   up). An interactive stdout carries the RENDER: with a directory the tree publishes there, without
>   one `auto` may settle for a less flattened form and SAY SO.
>   THE DEFAULT BUNDLES (`30Ng:rul-bundle-at-dorc-lang-boundaries`, human-typed): each book-sited root
>   at which dependencies become dorc-lang composes ONE generated file, and the generated plan's own
>   import is RE-SAID to name it. That rewrite is the single edit Dorc reserves over a plan it
>   generated — a generated plan is a durable but not an OFF-RAMP durable — and it is a first-class
>   plan edit: an input to `Plan::decided`, disclosed as `plan-import-rewritten` on the plan surface,
>   recorded as `RenderDecision::ImportRewritten` on the plane. The authored BOOK is never written and
>   every byte of it that reaches the artifact reaches it verbatim (`two-surfaces`).
>   BUNDLING IS POSITIONAL AND MEASURED-ONLY: a nested `.` is absorbed IN PLACE, so an include guard
>   still decides whether the absorbed bytes run, and only at the shape
>   `floor30-inline-dot-boundary` measured (a `.` that is the whole of its own line, and for a BOOK
>   `.` also a top-level redirect-free command). Anything else stays a separate generated file and the
>   authored `.` that names it survives — when in doubt, separate files.
>   Mirroring is stated against the LOAD CWD (`dorc_core::loadpath::Cwd::relativize`, the inverse of
>   `resolve_operand`), never against a stored path's own spelling: every source a book `.` reaches is
>   filed under its CANONICAL key, which is ABSOLUTE whenever the edge could answer where the run
>   stands, so a seat asking whether the stored spelling looked relative answered "unplaceable" for
>   every real invocation while every in-process test said the opposite
>   (`30Nf:fnd-multipart-never-placed-anything-in-production`). A dependency OUTSIDE the load cwd is
>   unplaceable rather than fudged (`need-controller-paths-never-cross-hosts`).
>   BOTH ENDS OF THE BUNDLE-POINT AXIS stay reachable by name — one emission (`flattened`) and none
>   at all (`mirrored-tree`) — with the default at neither. Every form name and file name here is
>   STRAWMAN and renames in place (`rul-strawman-formats-no-compat`); what is ruled is the axis, the
>   stream semantics, and the rewrite's scope.

**`spike/crates/cli/CLAUDE.md`, appending to `an-artifact-set-runs-from-its-own-generation`:**

> A published plan's own LITERAL relative imports must resolve inside the generation
> (`unresolved_generated_imports`), which is what makes the import rewrite observable end-to-end
> rather than a stdout-only claim. The COUNTERFACTUAL rails are the one carve and they are a
> different question: gate-5 and gate-6 compare an artifact against the BOOK, and since the bundling
> those two resolve their imports in different places, so they run in the generation laid over a copy
> of the case's authored top-level files. `exec_check` is untouched and still runs the published plan
> from the generation ALONE — that is where self-containment is asked, and nothing here relaxes it.

**`spike/crates/plan/CLAUDE.md`, appending to `the-render-decides-nothing`:**

> An IMPORT EDIT (`30Ng:rul-bundle-at-dorc-lang-boundaries`) is decided like every other render
> answer: `plan::ImportEdit` is an INPUT to `Plan::decided`, settled before the plan exists from
> authored-before-contact inputs, and it reaches the artifact bytes, the plan surface
> (`plan-import-rewritten`) and the decision plane (`RenderDecision::ImportRewritten`) from that one
> decision. There is no emission-time substitution, and there must never be one: the whole reason the
> rewrite is a narrow grant is that no use of it is silent.

**`spike/crates/core/CLAUDE.md`, appending to `a-second-key-axis-never-widens-siteid`:**

> `RenderDecision::ImportRewritten` is site-less AND region-less on `DefensiveEmission`'s precedent:
> an import edit belongs to a book LINE, which is neither an execution nor an authored region, so it
> borrows neither axis. And `SpineRegionDecision.routes` is `RegionRoutes` rather than an `Account`
> (`30Ng` §2, human-typed): the contributor set is COMPLETE, because a region's population is a
> property of the analysed unit rather than of the world and it is what two pull surfaces ask for by
> name; a route the round could not key to a plan leaf is RETAINED under
> `RegionRouteUnkeyed::NoPlanLeaf` rather than filtered, because an account that quietly drops a
> contributor reads as complete and is not.

**`Research/plans/30I`, "Where the build stands"** — re-cut the landed/owed lines to:

> Landed since: `30Ng`'s artifact-semantics rework (lane report `notes/30Nh`): the default bundles
> book-reached dorc-lang subgraphs and re-says the generated plan's imports; the stream posture
> derives from stdout interactivity with a pre-network refusal where two things claim one artifact;
> the artifact's lifted material is closed by a section boundary; the region contributor account is
> complete. `rul-piped-stdout-implies-one-flat-plan` (§2.5) is SUPERSEDED by
> `30Ng:rul-piped-stdout-carries-a-full-plan` — the rule binds stdout's own posture, never whichever
> stream happens to carry the artifact. Not built, and owed: the front-LIFT of oracle bundles in the
> single stream (`30Nh:tc-bundle-lift-needs-the-spine`), book-code load acquisition, and the prose
> queue.

**Owed prose (conductor/human, not builder):** `plan-import-rewritten` renders `[unwritten:]` and
joins the four registers `30Ne` queued. The help page documents neither `--artifact-dir`, `--form`,
nor the new `mirrored-tree` value.

## §12 — Residue

1. **The bundle front-LIFT** (§4, `tc-bundle-lift-needs-the-spine`) — the addendum's unmet half.
2. **Book-code load acquisition.** The charter's worked example needs it; the engine does not model
   it, and `pin-complex-book-source-render` is where it lives. Until then the single-stream refusal is
   reachable only through a dorc-lang load outside the measured shape.
3. **No e2e for the kept-stream cell** (§1) — a harness capability (expected-empty-stdout).
4. **`--form mirrored-tree` has no case of its own**; it is pinned natively
   (`both_ends_of_the_bundle_point_axis_are_reachable_by_name`).
5. **The prose queue grows by one register and one help row** (§11).
6. **`p-x-sentinel-value-conjunct` still red**, still waiting on the human's ruling; its trigger text
   remains accurate, and this lane's bundling makes it slightly sharper — a wrongly-`Reused` route
   still contributes its target to the bundle (the possible-load projection is the conservative
   union), so the artifact carries the file either way.
