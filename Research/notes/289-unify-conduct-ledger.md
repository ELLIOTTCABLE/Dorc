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

## §2i — Phases-2–3 checkpoint rulings + the phase-4 go (conductor, 2026-07-24)

- Checkpoint @ `40733595` on `ai/r28-unify-p24x` (17 commits; cold gates 1175 unit /
  97 e2e / clippy 0). Phases 2–3 COMPLETE: mint-seam (mirror-union from cases ·
  scaffold · repair-command-verbatim reds · walkthrough loom actually exercised
  end-to-end) · the full hardening package · prose sweep · Unexplained steering
  correction · lint unification (7 codes, all `sm `-migrated; framed/compact split
  named a policy; SCANNED_CRATES widened).
- **`289:finding-census-caught-renderrefusal-unminted`** — the h1/h2 census went red
  on its FIRST run: `RenderRefusal` collapses were never minted (the `291` nine-site
  table omitted it). Builder minted it (decision-inert, zero output change, paired by
  a shared predicate). The hardening package paid for itself same-day; the
  value-carriage escalation rung correctly NOT taken (missing mint ≠ census leak).
- **`289:rul-reflow-fix-in-phase-four`** (their flag 2) — the loom bin's
  `unreflow`/`normalize_layout` heuristics are framed-render-specific and REFUSE
  compact-rendered transcript edits (`MarkerOutsideEditableSection`). This will bite
  phase 4's case prose and phase 6's conversion: the fix (compact-aware
  normalization + a regression test compiling an edited compact-line transcript)
  joins phase-4 scope. The builder's fallback (direct catalog edit + promote) reads
  as WORKING-STATE under the `28A:rul-respell-atomic-cutover` licensing; the
  authoritative check is the fixpoint gates on MY cold binary at fold.
- NON_EMIT_CRATES split + negative control (their flag 3): ACCEPTED — the literal
  §4d instruction would have weakened `every_catalog_variant_is_constructed`;
  gate-strength preserved is the correct reading of intent.
- **`289:rul-comment-budget-bills-plain-only`** — standing interpretation for the
  arc: the numeric budget bills plain `//` comments only; `///`/`//!` doc-comments
  ride the doc-comment-every-public-type law un-billed; mandated prose-sweep
  rewordings and pure moves don't bill. Under it this lane sits ~41/45: PASS.
- Deviations accepted: render-policy-before-routing ordering · the
  `authored-decline-class{,-unreadable}` two-code split (law-codes-vary-by-world) ·
  compact line emitted as parts (byte-identical; loom-editability is the arc's
  point) · relay findings frame-less under `--verbose` (empty source; relays stay
  relays) · the module-doc roster correction · walkthrough on phase-3's first code.
- PHASE-4 GO issued to the same builder (context-hot), with: merge the stack tip
  into the lane FIRST (the touches fold landed after their base; surface conflicts
  now, gates re-verified post-merge), then the pre-briefed W2 route + invocation
  codes + `dorc-sh` + the reflow fix.

## §2j — Phases-2–4 lane COMPLETE + folded (conductor, 2026-07-24)

- LANDED @ `7948a0cb` (27 commits + a clean zero-conflict stack merge), FOLDED @
  `6686250d`. Cold gates at lane tip: 1184 unit / 97 e2e / clippy 0 / all four.
  Catalog now 87 rows; ALL 28 lane-minted codes carry `sm `-migrated shipped
  sentences (`message: None` count unchanged at the pre-existing 6); ceiling 15
  stands slack. The reflow fix found TWO bugs — the compact-line gutter heuristic
  AND a latent `str::lines()` trailing-newline drop that had refused EVERY case
  edit corpus-wide (compact lines were merely first to reach `compile_preview`).
  The W2 lib target extracted as a pure move (562 lines, one `use`), pinned
  both-ways (own-command fires / different-code refused / clean-parse falls
  through to payload world).
- Flag rulings: `cli-shim-dir-unwritable` beyond-inventory mint ACCEPTED (forced
  by one-catalog-no-legacy) · the sm-verbatim two-splits (18→21 codes) ACCEPTED
  (law-codes-vary-by-world doing its job) · `{USAGE}` NOT carried into rows —
  ACCEPTED, and banked as PHASE-7 INPUT (usage/help text is seat-appended today;
  the one non-verbatim deviation) · `result_large_err` resolved by targeted
  reasoned `#[expect]`s ACCEPTED (perf-doctrine: once-per-process cold path;
  boxing would obscure the mint shape the lexical gate greps) · the three
  whylog cases that are NOT render-fixpoints (blank-line divergence; churn
  reverted) banked as PHASE-6 INPUT · comment budget 74-net ACCEPTED for a
  three-phase two-crate lane (the bulk is world-state-split rationale — deleting
  reasoning to hit a quota is Goodhart).
- **`289:rul-spanless-gate-stays-lexical`** (their flag 1) — the gate stays a
  lexical grep (it caught two helper-factorings and a doc-comment needle this
  lane; working as designed); the footgun is now a bullet in
  `crates/aid/CLAUDE.md` (`spanless-gate-is-lexical`) so it is prevented at
  authoring rather than caught at gate.
- Conductor cold verification at the fold: full clean + build + clippy +
  `DRY=1 conduct-bless` (includes both fixpoint gates on MY binary — the
  working-state promote licensing check).
- 2026-07-24: phase-5 (Opus, fresh worktree, bg) — two-stop: central runners + differential proof, CHECKPOINT, then atomic move + run.sh retirement + safety-law edit. Off `5bb13199`.
- 2026-07-24: 2-4 fold cold-verified own-hand (full clean; build ok | unit 1184 | e2e 97 | gates ok, incl. both fixpoint gates on my binary).

## §2k — Phase-5 stop-1 rulings (conductor, 2026-07-24)

- Stop 1 LANDED @ `8307ca36` (4 commits, additive): central runners with structural
  discovery (entry-file-names-the-kind; no marker files), the WHOLE run.sh gate set
  ported, looms runner adds a render-fixpoint per committed loom. Differential proof:
  97=97 discovery · 11/11 identical mutation failures with identical gate tags ·
  BLESS 96/97 byte-identical (the 97th = the known lax-order class) · ~30× faster.
  Cold gates: 1332 workspace (1184 unit + 97 e2e + 51 looms) / deny ok
  (libtest-mimic pinned =0.8.1 for the anstream-major coherence; 12 dev-only lock
  entries; the dorc-cli⇄dorc-loom DEV-ONLY dep cycle is cargo-legal and test-scoped
  — noted for future crate-graph readers).
- **`289:rul-e2e-stays-in-workspace-suite`** (their flag 2) — the e2e target STAYS
  in the default `cargo test --workspace` run: at 8–15 s parallel the cost is
  noise and every builder's standard gate now covers the corpus automatically
  (gates strengthen; correctness lean). conduct-bless's tally re-points at stop 2
  with a unit/e2e/looms split.
- **`289:rider-real-tools-lane-rc-bitrot`** (their flag 1) — pre-existing red at
  HEAD under `DORC_E2E_REAL_TOOLS=shellcheck`: shellcheck 0.11's findings-exit
  (rc 1) false-fails the lane's absent-tool check; both harnesses agree byte-level.
  PHASE-6 rider: distinguish findings-exit from tool-absent.
- **`289:rider-fixpoint-gate-rationalize`** (their flag 4) — `DIRECT_PLAN_CASES`'
  4-case restriction is demonstrably stale (48/51 hold), but widening it now would
  duplicate the looms-runner's new corpus-level fixpoint. PHASE-6 rider: rationalize
  the two gates into one authority when looms convert.
- KNOWN_NON_FIXPOINTS pinned with XPASS-on-surprise (their flag 3): exactly right;
  accepted. The five port deviations (umask-shim unix-only, per-case framed_results,
  divergence-window, terse-quiet, pre-flight batteries): accepted as noted-at-site.
  Budget 9/20.
- STOP-2 GO issued. Post-stop-2, the SAFETY BLOCK text changes (sanctioned-executor
  line): every subsequent brief in this arc carries the NEW wording.

## §2l — Phase-5 COMPLETE + folded (conductor, 2026-07-24)

- Stop 2 LANDED @ `e3a74744` (3 commits), FOLDED. 956 renames, every one R100;
  content edits confined to runners + 6 path-constant files + yardstick;
  `sh e2e/run.sh` RETIRED; conduct-bless re-pointed with the three-way tally
  (`unit 1184 | e2e 97 | looms 51`); the sanctioned-executor role moved to
  `cargo test -p dorc-cli --test e2e` (new Safety-block wording REVIEWED and
  ACCEPTED — all subsequent briefs carry it). Violated-gate proofs: 6/7 RED,
  three of them because the builder CLOSED silent directions (zero-trial
  libtest exits 0; `load_corpus` empty-on-unreadable now panics + asserts
  non-empty); `is_case_owned` now manifest-local (the depth-coupling gone).
- **`289:dec-case-count-residual-accepted`** (their flag 2) — deleting a case
  dir still shrinks the suite silently (v1 stays green), UNCHANGED from run.sh:
  accepted as the named residual in cli's `count-drifts` (deletion is
  git-visible; the floor guards the invisible direction). No count-ratchet.
- Accepted as reported: the scaffold path cosmetic; receipt.rs's opaque fixture
  identifiers deliberately unrewritten; budget 17/20.
- **SyncThing incursion #4 (their flag 1), HUMAN-OWNED, untouched:** two
  `*.sync-conflict-*PHNHRER` snapshots of stop-1's injected-and-reverted
  differential mutations sit in the p5 agent-worktree's `spike/e2e/cases/` husk
  — harmless content, but (a) `.claude/worktrees/` demonstrably still syncs, and
  (b) the files are INVISIBLE to git (`status -uall`/`ls-files --others`/
  `check-ignore` all disagree with `find`) — a fixture tree that can gain
  git-invisible files is a real hazard, escalated to the human.

## §2m — SyncThing incursion #5: BUILD-BREAKING, conductor worktree (2026-07-24)

- The phase-5 fold cold-verify FAILED (exit 101) on machine-sync junk, not code:
  SyncThing dropped SEVEN `*.sync-conflict-*PHNHRER` copies into the CONDUCTOR
  worktree — two in cargo-visible locations (`dorc-loom/src/bin/*.rs`,
  `dorc-loom/tests/*.rs`) where cargo auto-targets them and dies on the illegal
  crate name; plus copies of `cli/src/main.rs`, root `AID-NEEDS.md`, and THIS
  LEDGER. Another machine is actively writing into live build trees.
- Disposition: all seven QUARANTINED (paths preserved) to the session scratchpad
  `sync-conflict-quarantine/` — moved, never deleted (conflict cleanup stays
  human-owned); verification re-run; the phase-6 builder pre-warned with the
  quarantine-and-continue protocol.
- ESCALATED to the human (with §2l's git-invisibility observation): five
  incursions across two rounds, now build-breaking and ledger-touching; the
  `.stignore` exclusion repair is the standing human-owned fix.
- AMENDMENT: the "git-invisible files" mystery is RESOLVED, and it was worse in a
  different way — the two e2e-husk conflict files had been SWEPT INTO COMMIT
  `cc46a948` by the stop-2 add (tracked-and-unmodified = invisible to every
  status view; that was the whole "invisibility"). Excised from the repo at the
  follow-up commit; disk copies quarantined; content persists in history. LESSON,
  standing for all lanes: an incursion-window `git add` can COMMIT junk — every
  builder's pre-commit `git status` check must eyeball for `sync-conflict` names
  explicitly. Re-verify after excision: `unit 1184 | e2e 97 | looms 51 | gates ok`.
- **`289:rul-conflict-files-delete-this-session`** (HUMAN-TYPED 2026-07-24) —
  `.claude/worktrees` cannot be stignored (cross-machine viewing of worktree work
  is load-bearing); PHNHRER is idle right now; for THIS SESSION ONLY, builders and
  conductor brutally delete any `*sync-conflict*` file on sight (report paths).
  NOT a standing offer — the quarantine-don't-delete posture resumes at session
  end. The arc-end cleanup builder sweeps the whole repo + all worktrees under
  this license before it expires.

## §2n — Phase-6 COMPLETE + folded (conductor, 2026-07-24)

- LANDED @ `950ffe7e` (13 commits), FOLDED. Normalizer vocabulary: ONE citizen
  (`pipe-stage-order`, capture-and-check symmetric; `RAN_ORDER=lax` retired, six
  logs re-canonicalized — the `28A` lax-order nit discharged); `sigpipe-rc-landing`
  and `temp-path-nonce` named-not-minted. Structural needles: slug-validated,
  DECLARED-IS-ASSERTED. All four riders discharged: lint tier converted + the NEW
  clean-run loom (`289:dec-clean-render-net-rides-loom` closed) · the whylog trio
  root-caused to CASE BYTES (missing canonical blank-line; `KNOWN_NON_FIXPOINTS`
  deleted; divergence-window diagnosis added) · real-tools lane reads the exit
  trichotomy, GREEN at HEAD under shellcheck · ONE corpus-level fixpoint authority
  (the looms runner; `DIRECT_PLAN_CASES` + the per-loom duplicate deleted, the
  non-empty floor preserved). 21 cases converted (all materializer keys exercised
  by removal); tail = 4 blocked-by-shape (2 preflight anchors + 2 real-tools
  dir-keyed) + 75 opportunistic post-arc. Windows two-tool real-tools red stays
  BY DESIGN (checkbashisms unspawnable; no platform-skip law minted). Cold gates:
  1352 / 0.
- **`289:finding-needle-assertion-caught-dead-fixture`** — `render21-heredoc-refusal`
  had asserted NOTHING for several rounds: its oracle carried only `__predict`, so
  no Replace license ever minted (`rul-vouch-is-verdict-authoring`), so the refusal
  path was never REACHED — world (c), no silent narrowing on any taken path
  (`refused_render_steps` drives BOTH diag and narrative; the discriminating test
  with `__is_converged` added shows `replace` + the refusal firing). Repaired by
  restoring the vouch; zero golden churn; declaration re-asserted.

## §2o — The arrangement-home RULING (the phase-7 design sitting; conductor, 2026-07-24)

- **`289:rul-arrangement-home-is-registry-plus-transcripts`** — options 2+3 of
  `288` §7b COMPOSE, generalizing the proven catalog pipeline to a second table:
  a generated ARRANGEMENT registry (arrangement-slug → ordered-words entries; a
  lock like `catalog_lock.rs`, same three-state prose protocol, same fixpoint
  gates) is the STORAGE; renderer-stamped Arrangement spans (the schema already
  carries the slug) are the EDIT surface; prose-bless attributes arrangement-word
  edits through the span map to registry entries. `282:rul-arrangement-words-
  exempt-v1` is thereby LIFTED for migrated chrome. Help text is the pilot
  (`$ dorc --help` as a whole-page loom — static, param-free); usage text joins as
  seat-appended entries (the phase-4 `{USAGE}` disposition confirmed).
- Chain anticipation (`289:rider-arrangement-home-anticipates-chains`) is SHAPE
  ONLY: entry keys carry an optional occurrence/position discriminator (mirroring
  span instance-ids) and storage is never welded flat-string-only — room for
  sequence-structured entries (link connectives, tier-word slots) with NOTHING
  chain-shaped built this arc.
- Fences: the tier-word SET stays typed law (only spellings become entries —
  trust-tier-is-syntax intact); artifact-plane strings (anything landing in the
  byte-floored `.sh`) stay OUT, flagged not migrated; no build toward `plans/286`
  (its density registers share the collection at ITS unpark); kFLOW's refused
  extreme stays refused — the registry stores words, never grammar machinery.
- `plans/288` §7b gets rewritten to record the taken option at the lane's landing
  (plans are ahistorical).
- 2026-07-24: phase-7 (Opus, fresh worktree, bg) — the §2o registry+transcripts ruling, two-stop (registry+help pilot, CHECKPOINT, chrome migration). Off `9f112f69`.
- 2026-07-24: phase-6 fold cold-verified own-hand (full clean; conduct-bless: build ok | unit 1182 | e2e 98 passed | looms 72 | gates ok).

## §2p — Phase-7 stop-1 rulings (conductor, 2026-07-24)

- Stop 1 LANDED @ `0c0a4cd3` (8 commits): the arrangement registry
  (`ArrangementEntry` with occurrence-optional keys, `Words::{Unwritten,Migrated,
  Authored}` — sequence-shaped, never flat-string) · the generated
  `arrangement_lock.rs` with mirror-union + its own fixpoint gate · editability via
  the new `RenderPart::ArrangementWords` species (registry-sourced bytes only;
  renderer-computed chrome stays immutable Structure) · the help-page pilot loom
  whose render fixpoint IS the byte-identity proof (the `HELP` const deleted) ·
  the full edit→compile→promote→re-render loop proven in-process AND with the real
  tool (run + reverted; working-state licensing, conductor cold verify
  authoritative). Cold gates 1366 / 0; budget 8/25.
- **`289:rul-migrated-marker-typed-for-chrome`** (their flag 1) — ACCEPTED as the
  forced-correct departure: chrome renders verbatim into product bytes, so the
  `sm ` marker moves out-of-band into `Words::Migrated`; `authored_words_are_
  case_owned` is the enforcement twin. Cost accepted (migration state visible in
  the lock, not the transcript); the phase-8 author enumerates via the lock.
- **`289:rul-version-line-stays-code-owned`** (their flag 2) — `--version` does NOT
  join the registry, at stop 2 or later this arc: the number is a value, the
  split-span shape buys nothing, and per-version transcript churn is a real cost.
- Dead `LINT_USAGE` deleted rather than seeded (their flag 3): accepted; the
  pre-existing UX gap (lint argument errors carry no usage line) banked as a
  post-arc residual, NOT built. `SectionKey.code`→`.owner` rename-in-place (flag
  4): per standing order, accepted. STOP-2 GO issued.
- Human spot-check findings (2026-07-24): the derived-survive header truncation was
  r24 damage (`458b26c1`'s comment-rip cut a purpose header mid-sentence) —
  completed same-line, one-case conductor bless (3-line diff: 2 comments + the
  book digest). **`289:residual-loom-mock-ergonomics`** — the per-case
  `_dorc_logged`/mock boilerplate (~40 lines/case) is the converted looms' worst
  noise; candidate homes assessed: errorloom-builtin recording stubs (lean —
  consumer-neutral executor capability, pre-publication is the cheap time) >
  shared in-tree mocks (erodes case self-containedness) > bats (bash-dependent,
  wrong for a posh∩dash-floor project). Banked post-arc unless the human pulls it
  forward.
- HUMAN-TYPED queue additions (2026-07-24, late): (1) conduct-bless FAILS for the
  human under WSL/zsh in the conductor worktree (my greens are msys/git-bash —
  ~SUSPECT mixed-platform target/ contamination or worktree-gitdir path
  translation; NOT critical) — thread to a builder when idle, before arc close.
  (2) `289:residual-loom-mock-ergonomics` gets a QUICK RESEARCH ROUND first, not a
  build: Sonnet + Kagi survey of bats-alikes / Rust CLI-test frameworks
  (trycmd/snapbox/cram/testscript-class) against our needs profile, esp. the
  scope-bloat question ("does errorloom grow a feature per future test need?");
  errorloom-feature stays the fallback lean; decision deferred — possibly a
  pre-phase-8 matter for a clean conductor.

## §2q — Phase-7 COMPLETE + folded (conductor, 2026-07-24)

- Stop 2 LANDED @ `42fa45da` (12 commits total), FOLDED. 17 registry rows, all
  `Words::Migrated` verbatim, ZERO golden churn stop-wide; the artifact-plane
  fence consumed the whole plan-render annotation class (`artifact-plane-strings-
  stay-out` + `layout-is-not-a-word` now steering law); occurrence-sharing gives
  repeated chrome one editable home; `remediation_hint` stays parked; `288` §7b
  rewritten in place. Cold gates 1371 / 0 (unit 1200 / e2e 98 / looms 73);
  budget 11/25.
- Design correction accepted: value-bearing chrome renders as ONE span whose
  entry holds ordered fixed runs interleaved with computed values
  (`arrangement_sentence`, arity-guarded — mismatch renders `[unwritten:]`,
  never a mangled line); trailing newlines stay computed so a render never ends
  inside an editable span.
- **`289:seam-multiword-chrome-render-only`** (their flag 3) — the 5 multi-word
  entries (2 lint sentences, 3 CLI stderr lines) REFUSE transcript edits
  (`ArrangementIsSequenceStructured`) until value-boundary re-splitting is
  designed; the 12 single-word entries are fully loop-capable. Same family as
  `28A:rul-glued-param-rehole-seam` (dumb word-boundary transport vs structured
  values) — the phase-8 author edits those 5 in the lock; both seams belong to
  one future transport-design sitting.
- Migration-net pins for faceless chrome (their flag 1) accepted — a real
  transcript face for the three stderr lines is `286`/post-arc territory.
  `describe_arrangement` rename (flag 2): internal, accepted.
