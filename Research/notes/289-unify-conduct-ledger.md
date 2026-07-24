# 289 — the aid/loom unification conduct ledger (the `288`-execution arc)

AI-authored (Fable conductor, seated 2026-07-24). Executes `plans/288` whole. Split out
of `notes/28A` by human direction (28A grew too long; it stays closed). Authority: root
docs, `spike/CLAUDE.md`, human-typed rulings outrank. Builders write NO landing notes;
as-built detail lives in granular commits + this ledger. Conductor stack:
**`ai/r28-unify`** (worktree `.claude/worktrees/r28-unify`; base `fbbf88f1` = ai/main
tip at seat), re-pointed/merged at verified lane tips as folds land.

## §0 — Session directives (human-typed 2026-07-24, this sitting)

- Conduct the `288` execution; land it WHOLESALE, except **phase 8 (the Fable prose
  burn-down) is HELD for the human's attention and ack**.
- Conserve conductor context/tokens brutally; trust Opus builders to execute briefs
  faithfully; no conductor churn over CLI invocations/tests.
- Ledger lives in a fresh note (this file), not 28A.
- Conductor works IN A WORKTREE (never the primary checkout); builders get clean/fresh
  worktrees for any modification; conductor rebases/merges their work; a cleanup
  builder at arc end removes completed worktrees/branches (guarded, merge-checked,
  surface-never-delete-unmerged — the `28C` janitor posture).
- Reach the human via notification when genuinely needed; otherwise maintain momentum.

Standing orders still binding (from `28A` §6/§4b, survive rewind): opaque-review
DEFERRED (infrastructure non-functional; do not attempt, do not re-ask) ·
strawman-formats-never-compat-targets (no mapping layers pre-user) · cold-clippy rider
(`28A:finding-incremental-clippy-serves-stale`) in every worktree builder brief.

## §1 — Arc state

- Base: `fbbf88f1` (ai/main; carries the 288 promotion + .loom respell).
- `288` §10's sole open ask is CLOSED: `289:rul-mint-hardening-package` (§2). No open
  asks to the human; phase 8 remains the one held item.
- Phase plan per `288` §8 + the acked compression lean: 0∥1 (running) → 2–4 as one
  checkpointed lane → 5 serial (atomic path move) → 6 → 7 (arrangement-home design
  sitting + build; help-text pilot) → 8 HELD (human ack required) → cleanup builder.

## §2 — Rulings (conductor, this arc)

- **`289:rul-mint-hardening-package`** (HUMAN-ACKED 2026-07-24, closes `288` §10) —
  build prop-mint-completeness-hardening as the four-part gate package, NOT the
  type-level/value-carriage kernel refactor: (1) a no-wildcard exhaustive
  `match CollapseKind` inside the completeness gate (compiler forces every new
  collapse class to visit the pairing site); (2) a tidy-style census over
  collapse-constructor call-sites (`error_codes.rs` shape); (3) the merge-mint
  pairing `debug_assert` promoted to a release-mode test gate; (4) DST
  fault-injection assertions per collapse class (force the seam, assert the
  narrative minted AND the chain renders it) — item 4 leans MANDATORY per the
  human's correctness-over-machinery lean ("if you consider 50% and 60%, choose
  60%, don't force 98%"), and is hard-mandatory if the builder finds a missing
  narrative silently OMITS a chain link rather than rendering `Unexplained`
  (builder must check + report which is as-built). Named escalation seam, build
  NOTHING now: if the census gate ever leaks a real under-narration,
  value-carriage-in-the-join is the priced next rung. Rides phase-1-exec or the
  2–4 lane, whichever `notes/290` says is cheapest.

## §2b — Phase-0 landing (conductor, 2026-07-24)

- LANDED @ `2bf13785` on `ai/r28-unify-p0` (2 commits): count-conditional `plural()`
  helper; BOTH lint sentences fixed — the tally (`render.rs:58`, the `288` §6 phase-0
  item) and the clean sentence (`render.rs:36`, conductor pull-forward under the
  human's correctness lean; its phase-7 arrangement-home disposition unmoved). 10
  expectations hand-edited (8 `.loom` cases + 2 e2e lint `expected.out`); no BLESS;
  no catalog/lock coupling. Builder gates cold-green (1163 unit / 97 e2e / four
  gates). Folded @ `c45be8b8`; conductor own-hand verification: cold `-p dorc-lint`
  rebuild + full clippy + `DRY=1 conduct-bless`.
- **`289:dec-clean-render-net-rides-loom`** — the builder's coverage flag (the clean
  render is untested below the substring level) is answered by a RIDER, not a unit
  pin: pinning full bytes in `report.rs` welds an explicitly-unwelded surface
  (`27V:rul-output-form-unwelded`) and would churn at phase 7 anyway. RIDER on
  phase 6: the lint-case loom conversion adds a CLEAN-run case (transcript pin,
  re-blesses freely) alongside the findings cases.

## §2c — Phase-1 MAP checkpoint rulings (conductor, 2026-07-24; on `notes/290`'s flags)

- Map LANDED @ `00155aaf` on `ai/r28-unify-p1` (notes-only; folded to the stack). The
  §3a rename decisions are ACCEPTED WHOLESALE — specifically dec-collapse-kind-kept +
  dec-trust-tier-kept (rename buys nothing, forces law-slug churn) and
  dec-hostevidence-untouched + dec-errorloom-untouched (genuinely different concepts;
  renaming would be a semantic error).
- **`289:rul-carrier-collision-path-rewrites-only`** (flag 1) — oracle's local
  `enum Carrier` in `predict/mark_grammar.rs` (the `#:`/`:` mark carrier) keeps its
  name; every rewrite touching that file is import-path-based, never bare-identifier;
  the §4d bare-identifier step EXCLUDES it. Correction accepted: cli (44 sites) is the
  heaviest file, not oracle.
- **`289:rul-user-aid-block-stays-pointered`** (flag 2) — the POINTER reading: the
  spike/CLAUDE.md User-aid block STAYS at root (it binds emission sites in every
  crate — plan/cli/analysis builders never auto-load `crates/aid/CLAUDE.md`); the new
  aid CLAUDE.md carries the crate-local registry plus duplicated deeply-critical
  bullets (AGENTS.md sanctions repetition for exactly this); the root block gains one
  relocation-pointer sentence for the moved types.
- **`289:rul-law-reslug-rides-phase-one`** (flag 3) — re-slug at this lane's
  steering-sync: `AID-NEEDS:law-collapse-mints-narrative` (née
  law-collapse-mints-evidence) and spike/CLAUDE.md `collapse-mints-narrative` (née
  collapse-mints-evidence), body wording updated, in-code citation comments grepped
  and updated mechanically; historical docs untouched. Motive: "evidence" was
  deliberately RESERVED for possible future correctness-plane use
  (`288:rul-narrative-layer-naming`); a kept-current law squatting on the word
  defeats the reservation.
- SiteId (flag 4): **TAKE**, as the map's §6 additive prelude, exactly as specified.
- Dead doc-links (flag 5): FIX the two dead links in a prelude commit (stale refs to
  killed types); NO cargo-doc gate this arc (machinery past the lean).
- Ratchet baseline skip-once (flag 6): accepted as designed; must NOT be "fixed";
  executor reports that it fired.
- **`289:rider-diag-tidy-scan-set`** (flag 7) — `SCANNED_CRATES` omits lint +
  dorc-loom (pre-existing): widen it in PHASE 3 when lint findings join the registry.
- Flags 8/9/12 (prov-stays-core · compile_fail doctest · catalog_lock wiring):
  confirmed clean, banked so no one re-derives. Flags 10 (plain-`.rs` test moves, no
  pre-created case dirs) and 11 (hostsim/sweep stay aid-free; investigate-don't-add):
  accepted as proposed.
- Hardening disposition: `289:rul-mint-hardening-package` RIDES PHASE 2 (the map's
  reasoning accepted — a behavior change would destroy the cutover's
  clean-diff-is-the-correctness-proof property).

## §3 — Lane map (update on every change)

| lane | branch | shape | state |
|---|---|---|---|
| phase 0: lint-tally-pluralization | `ai/r28-unify-p0` | single dispatch, worktree | LANDED+FOLDED 2026-07-24 @ `2bf13785` → merge `c45be8b8` |
| phase 1 map: aid-crate extraction spec (`notes/290`) | `ai/r28-unify-p1` | map half (map-then-execute), no engine edits | LANDED+FOLDED 2026-07-24 @ `00155aaf` |
| phase 1 exec: the extraction cutover | `ai/r28-unify-p1x` (base `c1cab82a`) | fresh executor per `290` + `289` §2c | DISPATCHED 2026-07-24 |
| phases 2–4: mint-seam+scaffold · lint-unification · cli-error-migration | — | one checkpointed lane | pending |
| phase 5: flat-tree move + run.sh retirement + safety-law edits | — | serial, atomic paths-only | pending |
| phase 6: e2e→loom conversion | — | serial | pending |
| phase 7: arrangement-home sitting + build (help-text pilot) | — | design sitting then lane | pending |
| phase 8: Fable prose burn-down | — | HELD — human attention + ack | held |
| cleanup: worktree/branch janitor | — | guarded, end of arc | pending |

## §4 — Ack-ledger (only human-TYPED items count)

- 2026-07-24 seat brief: execute 288; wholesale except phase 8; brutal token
  conservation; trust builders; notification tool available. (TYPED.)
- 2026-07-24 mid-turn: fresh ledger, not 28A. (TYPED.)
- 2026-07-24 mid-turn: conductor-in-worktree; fresh worktrees for builders; cleanup
  builder at arc end. (TYPED.)
- 2026-07-24: "Ack and approve" on the hardening package + the standing correctness
  lean (prefer 60% over 50%, never force 98%) — binds this arc's judgment calls.
  (TYPED; → `289:rul-mint-hardening-package`.)

## §5 — Dispatch log

- 2026-07-24: phase-0 (Opus, fresh worktree, bg) — lint tally pluralization + golden
  hand-edits.
- 2026-07-24: phase-1 MAP (Opus, fresh worktree, bg) — mechanical extraction spec →
  `notes/290` on `ai/r28-unify-p1`; no engine edits.
- 2026-07-24: phase-1 EXECUTOR (Opus, fresh worktree, bg) — the 290 cutover as amended by 289 §2c, on `ai/r28-unify-p1x` off `c1cab82a`.

## §2d — r29 evidence-types fence (HUMAN-TYPED 2026-07-24)

- The in-flight r29 work mints NEW `*Evidence*` types that are NOT narration-related
  (correctness/license-plane — the exact future use `288:rul-narrative-layer-naming`
  reserved the word for). The Narrative rename must never stomp them: explicit-file-list
  rewrites only, no blind grep (the `290` §4 design already complies; the human also
  briefed the executor directly). Standing for the whole arc and any successor: the
  word "evidence" BELONGS to the correctness plane now — never rename toward it, never
  sweep it.

## §2e — Phase-1 EXEC landing + residue rulings (conductor, 2026-07-24)

- LANDED @ `2ac85127` on `ai/r28-unify-p1x` (4 commits: SiteId prelude · doc-link
  prelude · THE atomic cutover · steering-sync+reslug). Folded @ `35954157`. Builder
  gates cold-green at parity (1163 unit / 97 e2e; full `cargo clean`); goldens and
  `.loom` cases byte-identical throughout; ratchet skip-once fired and re-arms;
  rewrite counts exact-parity with `290` §4a; hostsim/sweep/errorloom took NO aid dep;
  the r29 evidence fence verified two ways (identifier-allowlist rename; every
  surviving `*Evidence` identifier untouched). Conductor own-hand cold verification
  at the fold: full clean + build + clippy + `DRY=1 conduct-bless`.
- Deviations accepted (all mechanical): crate-root `#![expect]` carries `missing_docs`
  only (the map's `indexing_slicing` claim was wrong — `string_slice` territory);
  two build-found sites the map omitted (test-mod `LeafId` re-import; core inline
  Carrier-test deletion); `[GroupingKey]` doc-link de-linked (type stayed describe-plane).
- **`289:rider-dead-diagcode-link`** — `aid/src/diag.rs:243`'s `[crate::DiagCode]`
  (pre-existing dead link, one-word fix) rides the 2–4 lane's first commit.
- **`289:rider-narrative-prose-sweep`** — doc-comment PROSE in the collapse-mint lane
  still says "evidence" (~25 cli + a few analysis/plan mentions). Rule: sweep it
  WHERE IT NAMES THE NARRATIVE PLANE as part of phase 2's hardening work (which
  touches those same 9 mint sites); judgment-tier, flag ambiguous mentions; never
  touch license-plane/HostEvidence prose. Motive: post-r29, "evidence" prose over
  narrative machinery actively misleads (§2d fence).

## §2f — Phases 2–4 lane (one checkpointed lane per the acked lean)

- Shape: map-then-execute. MAP claims `notes/291`; riders attached: the hardening
  package (`289:rul-mint-hardening-package`, phase-2 seat) · `289:rider-diag-tidy-scan-set`
  (phase 3) · `289:rider-dead-diagcode-link` · `289:rider-narrative-prose-sweep` ·
  the `covered()⊆case-owned` drift guard (`28A` §2u) · the `touches`→`disturbs`
  fixture residue (`28A:finding-touches-rename-half-done`; verify-in-other-cells
  first, drop if it resists).
- 2026-07-24: phases-2–4 MAP (Opus, fresh worktree, bg) — `notes/291` on `ai/r28-unify-p24` off `35954157` (fold-tip; conductor cold-verify in flight in parallel; exec half gated on both).
- 2026-07-24: conductor cold verification of the extraction fold PASSED (full clean; build ok | unit 1163 | e2e 97 | gates ok). `35954157` is verified base for the 2–4 exec.

## §2g — Phases-2–4 MAP checkpoint rulings (conductor, 2026-07-24; on `notes/291`'s flags)

- Map LANDED @ `83e93632` (notes-only; folded). Executor: ONE fresh builder, ONE
  mid-lane checkpoint after the phase-3 close (before phase-4's lib extraction +
  case fan-out), per the map's sizing. Comment budget 45 accepted (net-new lane).
- **`289:rul-hardening-h4a-all-h4b-one`** (flag 1) — the silent-omit finding is
  CONFIRMED and worse: `Unexplained` exists NOWHERE; the why-chain reads witnesses,
  not narratives (`emit_why_lens` ignores its narrative slice by signature); only
  VerdictDecline reaches a user surface. So: h4a mint-assertions for ALL NINE classes
  (hard-mandatory per `289:rul-mint-hardening-package`); h4b render-assertion for the
  one renderable class; the narrative→render consumption gap is a NAMED SEAM
  (`289:seam-narrative-render-unconsumed` — future why-render/arrangement round owns
  it, out of this arc per `288` §2d). RIDER: correct the STALE kept-current registry
  text (spike/CLAUDE.md + AID-NEEDS "Unexplained … renders self-advertisingly") to
  as-built truth — kept-current registries must not lie; the law itself stands.
- **`289:rul-lint-render-split-is-policy`** (flag 2) — preserve the framed/compact
  split; NAME it a selection policy (render-form-unwelded; avoids 11-case churn and
  losing caret frames from default lint).
- **`289:rul-sm-where-ancestor-exists`** (flag 3) — migrated codes whose shipped
  ancestor sentence exists carry it verbatim under the `sm ` marker (migration, not
  authorship — the d1 precedent); `[unwritten:]` only for genuinely-new codes.
  Constantly-firing surfaces must not regress to placeholders while phase 8 is held.
- **`289:rul-worldless-route-honest-trigger`** (flags 4+9) — W2: real-fired
  invocation-error cases via a thin `dorc-cli` lib target. The `28A` §2n economics
  flip here — honest firing is trivially cheap for this family ("`$ dorc strip` IS
  the world"). The lib target is an INTERNAL loom-harness seam, doc-commented as
  such, never a public API surface.
- Flag 5 (`external-text` stays a relay): ACCEPTED. Flag 6 (prose-sweep scope):
  narrative-plane prose + PRIVATE local identifiers in the nine mint-site files at
  builder judgment; never public API, never `HostEvidence*`/r29-adjacent; flag
  ambiguity. Flag 7 (mirror-union seeds from CASES, departing `288` §4's letter):
  ACCEPTED — the spirit is loud-mintability, and red-until-case-exists with the
  repair command named VERBATIM at every red satisfies `288:rul-loom-mint-guarantee`
  better than a silent placeholder row; the flow is enum-variant → red gate naming
  `dorc-loom scaffold <slug>` → scaffold → `[unwritten:]` row → author world → green.
- **`289:rul-unwritten-ceiling-one-bump`** (flag 10) — ONE conscious bump for the
  lane: ceiling = existing 6 + the map's counted genuinely-new codes + 2 headroom
  (builder computes and states the number; per-entry notes retained). Never
  per-commit chores, never a silent weaken; phase 8's arc-close invariant (zero
  unwritten) supersedes the ceiling at close.
- **`289:rul-touches-mismatch-own-lane`** (flag 8) — the touches rider is a LIVE
  BUG (def `__disturbs` vs invocation `__touches`; fails safe, value-dead), dropped
  from the 2–4 lane and dispatched as its own parallel small lane: fix the
  invocation side to the ruled `__disturbs` suffix, pin def↔invocation to one
  source, and make `strawman24-derived-survive` exercise the LIVE lane (legitimate
  golden churn; conductor blesses at fold).
- Banked: nothing pins the `dorc: `/`dorc-sh: ` prefixes — phase 4 is golden-churn-free.

- 2026-07-24: touches-fix lane (Opus, fresh worktree, bg) + phases-2–4 EXEC (Opus,
  fresh worktree, bg), both off the post-map stack tip.

## §2h — Touches-fix landing (conductor, 2026-07-24)

- LANDED @ `d9ff7b81` (6 commits), FOLDED. The live bug (shipped derivation probes
  defined `__disturbs` but invoked `__touches`; rc 127 ⇒ every derived footprint
  walled; failed SAFE, value-dead) is fixed at its one site; all four role-family
  sh-suffixes now flow through role-owning `pub const`s (verdict included — the
  identical `strip_verdict` trap closed on follow-up); def↔invocation pinned by
  anti-masking-verified regression tests for disturbs, reaches, AND verdict.
  Goldens conductor-inspected line-by-line at fold: 4 renames + the `sed` mock
  un-drifted (`file:` → `sm.dorc.File:`, doubly stale). e2e gains gate-1(d) deriv
  parity (mocked probe's produced coords ≡ authored coords; adversarially verified).
  Builder cold gates: 1167 unit / 97 e2e / all four.
- WHY NOTHING CAUGHT IT, banked: gate-1 never inspected `deriv ` records;
  `framed_results` round-trips the AUTHORED fixture, not probe execution; and
  **`289:seam-sweep-blind-to-shipped-bodies`** — the DST/sweep lane sources
  footprints from `Host::derive` with `sh: String::new()`, structurally blind to
  the rendered/executed shipped body. Named seam; future DST round owns it.
- **`289:dec-touches-display-vocab-phase8`** — the `.touches()` display locus
  (probe comments + why-lens origin vocabulary) stays retired-spelling until the
  phase-8 prose pass (message-string-coupled class,
  `28A:finding-old-prose-coupled-to-message-strings`).
- **`289:rider-arrangement-home-anticipates-chains`** (from the human sitting,
  unopposed) — phase 7's arrangement-home sitting MUST design the home to
  accommodate future chain-link/narration prose (multi-link sequences, tier-worded
  connectives, per-link provenance stamps) even though nothing fills them this arc;
  the shape-anticipation answer to `289:seam-narrative-render-unconsumed`.
- 2026-07-24: touches fold cold-verified own-hand (build ok | unit 1167 | e2e 97 | gates ok). Stack tip `397e8e78` verified.
