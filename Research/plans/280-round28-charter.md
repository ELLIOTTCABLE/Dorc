# 280 — round-28 charter: the syntax unification + the errorloom pipeline

AI-authored (Fable conductor, 2026-07-19; human-acked in-dialogue the same day — the
lane topology, the lane names, and the `#:` carrier are all human-typed). Authority:
root docs, `spike/CLAUDE.md`, and human-typed rulings outrank. The two seed specs this
charter executes: **`plans/281`** (the annotation mark-grammar, THE spec) and
**`plans/282`** (the transcript-case prose pipeline). Companions: `notes/27U` (the
aid-phase as-built this round builds on) · `notes/27Q` (block-stdlib preconditions) ·
`plans/270` §5 (the standing arc chain this charter resumes and re-sequences).

## §0 — Mission and blocker state

Round 28 lands both seed specs as built truth in minimal wallclock: two parallel
lanes with disjoint file-surfaces, then one serial lane for everything that needs
both. **No blockers are outstanding at charter-mint:** the `#:` comment carrier is
human-acked (typed 2026-07-19; the governing tension is registered as
`KNOBS:kSALIENCE`, committed at `7851eeb`), and respell timing is answered by this
charter's existence. Standing pending-rulings and small fixes are deliberately NOT
enumerated here — they are banked at `TODO-ADDTL.md`'s tail (human-directed,
2026-07-19) for interactive digestion with a fresh conductor.

## §1 — `lane-errorloom-crate` (parallel)

`282` phases 1+3: the transport engine (tokenizer · word-diff aligner · provenance
attribution · re-holer · refusal classes · the round-trip property test) and the
container/runner (txtar+frontmatter parse/materialize · the sequential replay
runner on the e2e safety rails · inline-on-bless · coherence gates), as the
external/publishable **`errorloom`** crate. Fully dorc-free: runner self-tests
drive mock commands; the crate's only repo touch is the workspace-member line.
This lane deliberately EXCLUDES `282` phase-2 (the tagged render): that phase
edits the render seat, `core/diag.rs` arms, and `catalog.rs` — exactly the files
the respell churns — so it rides the serial lane instead, keeping the two parallel
lanes disjoint by construction. Branch: `ai/r28-errorloom-crate`.

## §2 — `lane-syntax-unification-respell` (parallel)

`281` Part I into the engine: the intro grammar (`:` / `#:` + head sugar `!`/`?`/`=`),
the verb-driven mark parse (`rul-verbs-dotless-kinds-dotted`), the word-verb
vocabulary (`asserts`/`refutes`/`reads`/`bind` · `safe-across` · unified `disturbs` ·
`lends` · `stored-in` · `undivided-by-transit-across`), the `@` selector,
brace-alternation as a general payload combinator, continuation lines, rc-arity, and
the new strip semantics (marks erase to NOTHING; `#:` strips iff valid, diagnosed
otherwise). BOTH carriers ship. Then the corpus respell off `281`'s closing
grep-map (fixtures, e2e cases), goldens re-blessed at lane close
(conductor-exclusive bless), and the crate-`CLAUDE.md` authored-surface blocks
updated. New parse-diagnostic codes mint with EMPTY prose per
`27V:rul-error-authorship-tier`. Slug discipline: existing `DiagCode` slugs stay
(wire permanence); spelling-mentions inside `sm `-prose update; any genuine
wire-slug question is flagged, never resolved in-lane. Branch:
`ai/r28-syntax-respell`.

## §3 — `lane-errorloom-unify` (serial; off the folded tip of §1+§2)

Everything that needs both, in order: the tagged render (`282` §4, with the
product-renders-byte-unchanged gate) → the generation flip (`282` §6/§8 —
promote-v2, git mode-gates, carry-forward, the CI fixpoint gate, roster
retirement; dispatched as the map-then-execute two-phase checkpoint, since it
rewires catalog ownership) → the case-corpus backport (`282` phase 5, authored in
the NEW mark grammar, embedding respelled sources) → de-passthrough (`282` phase 6,
the type-gated foreign-text audit) → ONE docs/steering/registry re-synthesis pass
covering both changes (the `spike/docs` quoted-footer grep-sweep · the
`spike/CLAUDE.md` authored-surface rewrite · `AID-NEEDS.md` law wording ·
`spike/skills/author-oracle`). Branch: `ai/r28-errorloom-unify`.

## §4 — Shared surfaces and the bless discipline

The three genuinely-shared surfaces, and how the cut dodges each: the
catalog/diag files (the respell ADDS codes; the flip re-owns the FILE — separated
in time, §2 before §3); the e2e corpus + goldens (the respell re-blesses once at
its close; the unify re-blesses once at integration; `lane-errorloom-crate` never
blesses — one bless per serial segment, zero concurrent bless, bless stays
conductor-exclusive); the docs tree (stale under the respell; re-synthesized once,
at §3's tail — optionally split to a cheap parallel lane beside §3 once §2 folds,
if wallclock demands). Folds: conductor-executed (the 2026-07-19 git-surgery
relaxation) and conductor-verified own-hand (never-vouch); fold order
`lane-errorloom-crate` first (near-conflict-free), then
`lane-syntax-unification-respell`, then branch §3.

## §5 — Sequencing and wallclock

§1 ∥ §2 from the `ai/r27-aid` tip; §2 is the long pole of the parallel front; §3
dominates the total. Shave options, builder-latitude at dispatch: the docs
re-synthesis as a parallel lane beside §3; §3's case-authoring fans out across
codes once the generation flip lands. The `records-*` corruption case tail and
the prose-quality sprint stay OUT (`282` §9 phase-7 / §10 — lazy burn-down;
sprint at a surface-stability moment).

## §6 — The horizon (the `270` §5 chain, resumed with one re-sequencing)

**block-stdlib** follows `lane-syntax-unification-respell` at minimum (seed
oracles and teaching templates must be born in the new grammar — `281` §12's
respell-before-stdlib rider discharged by this ordering; human-led conductor per
the standing ruling; on-ramp `notes/27Q`, §2 before any oracle; MH2
prerequisite-acts + the charset-confirm land at its dispatch) → yardstick-
measurement → the r25 field-trial revival (+ `26B:ask-trial-counts-capture-walls`)
→ the r26 multi-host resumption (bank: `26B`/`26C`) → the corpus-rewrite metaplan
(`Plans/metaplan.md`) stays PARKED, earliest pickup unchanged: post-block-stdlib.

## §7 — Dispatch law (pointers, not restatement)

Standing brief law rides every lane: the `spike/CLAUDE.md` safety block verbatim ·
worktree step-zero onto the stated `ai/r27-aid`-lineage tip + step-0.5 + step-one ·
the sonnet sub-spawn clamp · the comment budget + counting command · four gates +
foreground e2e per commit-chunk · granular `(AI …)` commits · builders author zero
user-facing prose (placeholders only) · naming discipline (`270` §1) · tc-*-shaped
judgment calls flagged up, never resolved in-lane. `27U` §5's candidate laws bind
as brief text until registered: worktree-file-access (the primary checkout is
radioactive for ANY access), foreground-final-verification,
conduct-bless-is-the-verify-entrypoint.
