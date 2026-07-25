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
| phase 1 exec: the extraction cutover | `ai/r28-unify-p1x` | fresh executor per `290` + `289` §2c | LANDED+FOLDED @ `2ac85127` (§2e) |
| phases 2–4: mint/lint/cli-errors (map `notes/291`) | `ai/r28-unify-p24` + `-p24x` | map-then-execute, mid-lane checkpoint | LANDED+FOLDED @ `7948a0cb` (§2i/§2j) |
| touches-fix: disturbs-family name pin | `ai/r28-unify-touches` | parallel small lane | LANDED+FOLDED @ `d9ff7b81` (§2h) |
| phase 5: flat tree + central runners + run.sh retirement | `ai/r28-unify-p5` | two-stop | LANDED+FOLDED @ `e3a74744` (§2k/§2l) |
| phase 6: e2e→loom conversion + riders | `ai/r28-unify-p6` | single stop + follow-up | LANDED+FOLDED @ `950ffe7e` (§2n) |
| phase 7: arrangement home + help pilot + chrome | `ai/r28-unify-p7` | two-stop per §2o ruling | LANDED+FOLDED @ `42fa45da` (§2p/§2q) |
| WSL cross-platform repairs | `ai/r28-unify-wsl` | bounded investigation | LANDED+FOLDED @ `09fe9aa0` (§2r) |
| harness research (turns 1+2) | none (`.claude/research/loom-harness-alternatives/`) | Sonnet web survey | COMPLETE; decision deferred to the pre-phase-8 conductor |
| cleanup: worktree/branch janitor | none (repo-global, guarded) | single dispatch | COMPLETE (§2s); all lane branches/worktrees retired |
| phase 8: Fable prose burn-down | — | HELD — human attention + ack | HELD (the arc's one open gate) |

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
- 2026-07-24: phase-7 fold cold-verified own-hand (full clean; conduct-bless: build ok | unit 1200 | e2e 98 passed | looms 73 | gates ok). Stack `75344041` verified — ALL BUILD PHASES (0–7) COMPLETE.

## §2r — The WSL investigation, folded (conductor, 2026-07-24)

- LANDED @ `09fe9aa0` (4 commits), FOLDED. REPRODUCED under real WSL; all three
  initial suspects wrong-or-secondary. Mechanisms: (1) `#[cfg(windows)]` receipt
  machinery ⇒ correct Linux-only dead_code errors under `-D warnings` — five
  items now platform-gated (deliberately NOT `allow`'d: a future cross-platform
  caller must fail to RESOLVE); (2) a `#[cfg(unix)]` shim-chmod path that had
  NEVER type-checked (`String` into a `Diag` seat) — fixed via same-seat reuse
  (`shim_dir_unwritable`; ruling: ACCEPTED, same edge + same world-state, a
  separate code would be grammar-driven against law-codes-vary-by-world);
  (3) the git wall: `extensions.relativeWorktrees` needs git ≥ 2.48, WSL stock
  2.43 refuses the REPO — conduct-bless now pre-flights git AND mise and refuses
  in one line in ~50ms instead of failing after a ten-minute green run.
  Suspect (a) target-contamination CLEARED with evidence (host-keyed unit
  hashes; the extensionless `dorc` is a PE hardlink, not an ELF intruder) — no
  guard built against a problem that isn't there.
- New steering law: `one-platform-green-is-not-cross-platform-green` (+
  `wsl-needs-a-modern-git`), spike/CLAUDE.md build section.
- Gates: msys cold full green (1200/98/73); WSL clippy 0 + 1365 tests (the
  1371−1365 gap = exactly the 7 windows-only minus 1 unix-only cfg tests;
  builder-verified, accepted as reported).
- HUMAN STEPS (structural, not built): upgrade WSL git past 2.48
  (`ppa:git-core/ppa`); make mise activation survive a non-login `sh` if
  conduct-bless should run under WSL. Until the git upgrade, NO git-touching
  Dorc tooling works there (incl. `dorc-loom promote`) — the pre-flight refuses
  cleanly.
- 2026-07-24: WSL-repairs fold cold-verified own-hand (conduct-bless: build ok | unit 1200 | e2e 98 passed | looms 73 | gates ok). Stack `fbc44956` verified.
- HUMAN steer (2026-07-24): harness research gets a SECOND turn, entirely Rust-ecosystem-focused — what e2e/sh testing real Rust projects use (adoption signals, who rolled their own and why), and which capability choices make errorloom maximally-useful/minimally-bloated to the most people at publication (verify the prose-transport-has-no-competitor claim). Queued to the researcher as turn02.

## §2s — Cleanup sweep + ARC CLOSE (conductor, 2026-07-24)

- Cleanup COMPLETE (guarded janitor, `28C` posture): 10 lane branches + the empty
  `ai/loom-string-centralize` deleted with merge-guards intact (`-d` from inside
  the conductor worktree so git's own merged-into-HEAD check tested the right
  target; `-D` never used); 9 agent worktrees removed + pruned; 585/586
  sync-conflict files deleted under the session license (list:
  session-scratchpad `conflicts.txt`). The harness attached a bulk-deletion
  security warning to the sweep; conductor judgment: within the human's typed
  session-scoped license, announced twice pre-dispatch; surfaced to the human
  with the audit pointer.
- Sweep guards that caught real things: `.git` EXCLUDED (four sync-conflict
  BRANCH REFS exist under `.git/refs/heads/` incl. one shadowing the conductor
  branch — SyncThing is writing into `.git`; HUMAN-OWNED, ignore-config-level
  fix, never ref deletion); the r29-quarantine hit HELD (fence beat license);
  two tracked conflict fixtures restored on the lane before worktree removal.
- SURFACED for human adjudication (untouched; continuity with `28A` §4's
  three-dirty-worktrees item): `agent-a3557130737d11c12` (1 unmerged commit +
  dirty `Research/trial/observe/recon.sh`) · `agent-af67e0c672b0f437e` (four
  uncommitted 279a/279b Sol-N review reports + a `.claude-commit` sentinel —
  looks like finished, never-committed review work) · `agent-a4bc512f21f7ea336`
  (codex scratch only, near-certainly disposable) · the four `.git` conflict
  refs · the dead `loom-case-extension` orphan dir.
- ARC STATE AT CLOSE (amended — see §2t): every `288` §8 phase 0–7 LANDED+FOLDED+cold-verified;
  stack `ai/r28-unify`, final verified tally
  `build ok | unit 1200 | e2e 98 | looms 73 | gates ok`. Opaque-review NOT run
  (standing human-typed deferral: infrastructure non-functional, do-not-re-ask).
  Phase 8 is the single open gate, HELD for the human. The phase-8 package:
  (a) catalog prose — the `sm ` corpus + 6 `[unwritten:]` codes; (b) the
  arrangement registry — 17 `Words::Migrated` rows (5 multi-word = lock-edited,
  `289:seam-multiword-chrome-render-only`); (c) the coupled doc-comment/message
  respell (`28A:finding-old-prose-coupled-to-message-strings`) + the
  `.touches()` display vocabulary (`289:dec-touches-display-vocab-phase8`);
  (d) the transport seams one sitting should take together: glued-param
  re-holing (`28A:rul-glued-param-rehole-seam`) + multi-word re-splitting;
  (e) deferred-by-design context: catalog canonicalization
  (`28A:rul-catalog-canonicalization-is-conductor`), the harness-research
  decision, `289:residual-loom-mock-ergonomics`, the errorloom LICENSE/publish
  forks.

## §2t — Pre-prose hint-homing (HUMAN-DIRECTED 2026-07-25; re-opens the arc briefly)

- HUMAN-TYPED unit: the `remediation_hint` park (§2q) is UNPARKED — the class-keyed
  `&'static str` fn in `aid/src/diag.rs` (~:2715) is the last unhomed user-facing
  prose class and falsifies the phase-8 author's mandate ("the human edits only loom
  files, ever after") on day one. Ruling rationale: the phase-7 arrangement registry
  IS class-keyed word storage with the three-state protocol, so this is a STORAGE
  move (`Words::Migrated` verbatim, occurrence-less rows, render seat reads the
  registry), not new design; strawman-formats keeps the home freely renameable if
  the deferred register-schema sitting wants it elsewhere. RIDER folded in: a
  one-command prose-worklist enumeration (`sm `/`[unwritten:]`/`Migrated` across
  catalog + arrangement, lock-edit-only rows marked) so the phase-8 author starts
  from a mechanical list.
- Dispatched 2026-07-25: one Opus lane, fresh worktree, `ai/r28-unify-hints` off
  `f78c0620`. Expected user-visible delta ZERO; sequence-structured bucket landing
  acceptable (reported, not redesigned); no gate hacks; flags up.
- AMENDED mid-flight (HUMAN-TYPED 2026-07-25): the worklist rider is a tiny
  TEMPORARY grep shell-script (deleted after the prose pass), NOT a dorc-loom
  subcommand; low criticality, no churn. Relayed to the running builder.

## §2u — Hint-homing LANDED + FOLDED + verified; flag rulings (conductor, 2026-07-25)

- LANDED @ `4f669ee6` on `ai/r28-unify-hints` (4 commits), FOLDED; conductor cold
  verify own-hand (`clean -p dorc-aid` + full clippy 0 + `DRY=1 conduct-bless`:
  build ok | unit 1201 | e2e 98 | looms 73 | gates ok). Zero user-visible bytes
  changed; zero golden/loom churn; both fixpoint gates green (the hand-seeded
  `Migrated` rows are generator-fixpoint, per the `sm `-row precedent). Four
  occurrence-less `why-remediation-*` rows, `Words::Migrated` verbatim; the
  hardcoded `remediation_hint()` fn is dead; four byte-identity migration pins.
- **`289:seam-whylens-render-seat`** (rules the builder's
  `tc-whylens-has-no-render-seat`, ACCEPTED as disposed) — the why-lens reason is a
  FRAGMENT embedded mid-line by two consumers, so it cannot own the trailing
  computed layout an editable span needs; the rows are read as plain text
  (`arrangement_text`, the `usage_text` precedent) and are lock-edit-only,
  FACELESS (single-word bucket; the multi-word seam never fired). The builder's
  new aid-CLAUDE.md law `a-registry-row-need-not-mint-a-span` is conductor-
  reviewed and ACCEPTED. Giving the why-lens a real parts-stream seat is a
  render-form design act — banked to the ONE future transport/register-schema
  sitting alongside `28A:rul-glued-param-rehole-seam` +
  `289:seam-multiword-chrome-render-only`.
- **`289:finding-reason-opener-still-hardcoded`** (banked, NOT extended) — the
  `why()` reason OPENER ("ran because … (when unsure, run)") remains a hardcoded
  `format!` in `diag.rs`; it was the OTHER half of `288` §6's class-keyed-prose
  bullet. Migrating it interleaves computed values (multi-word/lock-only) and
  re-raises the seat question, so it belongs to the same deferred sitting; the
  human's unit named the hints, and no-churn was typed. Surfaced to the human —
  pull it forward if the residue offends.
- The worklist script (`spike/_prose-worklist.sh`, 30 lines, marked temporary)
  emits the 102-item phase-8 inventory (74 catalog `sm ` messages + 1 `sm ` help +
  6 unwritten + 21 arrangement Migrated); its independent 5-row lock-only count
  confirms `289:seam-multiword-chrome-render-only`'s inventory. `help: None`
  deliberately excluded (register-absence is completeness, not debt).
- ARC STATE: phases 0–7 + the hint-homing unit ALL LANDED+FOLDED+verified; stack
  tip = this fold. Phase 8 (Fable prose burn-down) is the sole open gate, HELD
  for the human's ack; its entry point is the worklist script; its package is
  §2s(a)–(e) with (a) now including the 4 `why-remediation-*` lock rows and the
  opener finding above.

## §2v — Fold state + a forwarded seam, deferred (conductor, 2026-07-25)

- Human folded the stack into `ai/main` (+pushed) and fast-forwarded `ai/r28-unify`
  onto the merge (`836feb7b`); stack and ai/main coincide at this writing.
- **`289:seam-diagnostics-print-not-carried`** (forwarded from a human-run
  investigation agent; DEFERRED, no action, conductor-ruled) — `cli/src/main.rs`
  emits diagnostics by PRINTING from inside analysis helpers instead of
  accumulating them: `build_kind_resolvers` (~:2105) calls `report_at`/
  `report_by_oracle_file` mid-body and returns bare values, not `Carrier<T>`
  (~25 such sites; `advisory: bool` — pure render policy — threads through
  analysis signatures to serve them), against `inv-no-throw`'s stage shape and
  cli's io-at-edges-only. Symptom: `report()` writes real fd-2 via
  `anstream::stderr()`, bypassing libtest capture, so one green unit test prints
  a red `error[resolver-conflict]` frame interleaved into `cargo test` output
  (the diag IS the asserted behaviour; run correct, rc 0). No correctness/
  license/DST exposure; costs = diagnostics unreachable-as-values to in-process
  tests, and emission-order-by-scheduling as a preview of the multi-host
  concurrency cell. Landed r27-era (`2c36bb00`, `00664b14`); orthogonal to this
  arc. HOME: the same future seat/walker sitting as
  `289:seam-whylens-render-seat` + `289:seam-narrative-render-unconsumed` — it
  is the EMISSION-end face of the identical seam (aid output must exist as
  data to be composable at a seat; print-in-place forecloses both tagging and
  Carrier accumulation). The test-noise papercut rides along (unfixable
  cheaply while writes bypass capture by construction).

## §2w — The seat design-round OPENED (human-directed 2026-07-25; conductor = the sitting)

- Scope: synthesize `289:seam-whylens-render-seat` + `289:seam-narrative-render-
  unconsumed` + `289:seam-diagnostics-print-not-carried` (+ the transport siblings:
  `28A:rul-glued-param-rehole-seam`, `289:seam-multiword-chrome-render-only`) one
  step toward settled; bank partial human feedback, settle nothing permanently.
- Corpus finding first: the why-CHAIN has ZERO committed transcripts (the flagship
  loom pins only the plan/probe artifact; chain renders live in unit asserts;
  whylog looms are refusal-only; one e2e `expected-why` carries arm-inlining).
- DISPATCHED 2026-07-25: whygallery lane (Opus, fresh worktree,
  `ai/r28-unify-whygallery` off `d964d8c1`) — ~5 non-defining gallery looms, one
  shared webhost-shaped world, replays across plan-stderr / zero-arg why +
  TRUST-SPENDS / why-N on elided·guarded·survived·declined / whylog + `--last`;
  captured-output-only, zero engine changes, findings-as-deliverable.

## §2x — Gallery landed+folded; the strawmen phase; errorloom word-model steer (2026-07-25)

- Gallery LANDED @ `8355d4ba` (5 looms, fixture-only, gates cold-green incl.
  e2e 103 / looms 78), FOLDED @ `7f09d181`. Findings banked (builder-observed,
  design inputs for the sitting): **fnd-why-surface-is-not-committable** (bless
  drives only `blocks().first()`, zips one output — multi-view transcripts
  impossible as-built, vs `282:rul-multi-replay-per-case`'s letter) ·
  **fnd-in-process-driver-needs-a-code** (caseless tier has one drivable shape) ·
  **fnd-zero-arg-why-unreachable** (argv-reachable, gate-unreachable) ·
  **fnd-decline-class-is-push-only** (pull-side `why N` on a declined line says
  nothing — inverts rul-chain-is-pull-only's intent) ·
  **fnd-why-heading-carries-the-argv-path** (machine-specific headings) ·
  **fnd-one-annotation-for-three-mechanisms** (elide/omit/survive render as one
  "elided" comment — rul-attention-honesty review item) ·
  **fnd-guard-preamble-precedes-the-books-shebang** ·
  **fnd-errexit-erases-the-gallery** (README's own `set -eu` example would elide
  nothing) · **fnd-needle-tension-on-the-why-lane** (expect-why needles are
  free-text by construction; kept structural-anchored).
- HUMAN-TYPED (2026-07-25): the strawmen phase — five DISPARATE why-output cases
  on-disk as looms with real output; human then edits them into proposed forms;
  machinery requirements derived from what satisfies all five generically.
  Conductor ACKED ready. DISPATCHED: `ai/r28-unify-whystrawmen` off `7f09d181`
  (elided-minimal · guarded-mid · survived-maximal+`--last` · declined-thin-truth
  · zero-arg problems-report), with ONE narrowly-licensed machinery change:
  complete multi-replay drive-and-bless per `282:rul-multi-replay-per-case`
  (spec-completion of the zip gap; sequential blocks, shared scratch, every
  block blessed). No render changes; ugly commits ugly.
- **`289:steer-errorloom-best-to-use`** (HUMAN-TYPED 2026-07-25) — errorloom is
  unpublished; zero backwards-compat weight. Optimize for best-to-use: if the
  whitespace-only word-boundary is artificial and consumers must painfully work
  around it (glued-params; multi-word/sequence entries), fix the word-model IN
  errorloom — possibly as an enriched, more-complex-API mode (span-aware
  tokenization/alignment). VOIDS the prior "reaches into the published crate's
  core word-model" objection against `28A:rul-glued-param-rehole-seam` option
  (a); reframes `prop-span-boundary-tokenization` toward errorloom-core.
  Sitting output, not an immediate build.
- 2026-07-25: gallery fold cold-verified own-hand (clippy 0; conduct-bless: build ok | unit 1207 | e2e 103 | looms 78 | gates ok).

## §2y — Whystrawmen LANDED + FOLDED + verified; the corpus is ready (2026-07-25)

- LANDED @ `7b6a4a5a` (3 commits), FOLDED @ `7954ff97`; conductor cold verify
  own-hand (clippy 0; conduct-bless: build ok | unit 1207 | e2e 103 | looms 78 |
  gates ok; builder additionally proved byte-stability by double unblessed run).
- **`289:rul-why-blocks-ride-the-gallery`** (rules the builder's naming flag) —
  ACCEPTED: the five why transcripts live as ADDED replay blocks in the five
  existing `whygallery-*.loom` worlds (no `whystrawman-*` files; no world
  duplication/drift; the prefix now honestly means "the why-surface gallery").
  Harness: `282:rul-multi-replay-per-case` drive+bless COMPLETED in
  `cli/tests/e2e.rs` (+183/−22: sequential blocks, shared scratch so the whylog
  flows, per-block bless, scratch-path-leak refusal at capture, empty-slice
  leaves committed bytes untouched). ALL five outputs are committed bytes;
  nothing needled or hand-typed.
- Findings delta banked (sitting inputs): **fnd-guarded-chain-omits-the-wall**
  (the pulled guard chain never mentions the wall that caused the demotion —
  push-only again) · **fnd-replayed-voice-is-byte-identical** (`--last` has no
  replay banner/staleness framing) · the declined thin-truth CONFIRMED as
  committed bytes (class+arm live in the narrative plane, never reach the pull
  chain — `289:seam-narrative-render-unconsumed` in the flesh) ·
  fnd-why-heading is argv-echo not a bug (case-relative driving ⇒ stable bytes).
- Bug riders banked (real, small, non-blocking; ride any convenient lane):
  **`289:rider-why-last-address-order`** (`dorc why --last 10` silently parses
  `10` as the BOOK and renders the zero-arg report at rc 0 — silent wrong
  surface) · **`289:rider-sibling-note-false-fires-relative`**
  (`aid-unloaded-sibling-oracle` false-fires listing already-loaded oracles when
  they were named by relative path).
- The comparison corpus is COMPLETE: as-built truth committed in the five
  gallery looms; the conductor's unpoisoned aspirations in
  `notes/292-why-output-strawmen/` (written+committed BEFORE this landing was
  read, `7ab5e497`); the human's editing round is next — in-place uncommitted
  edits (harvested as diff) or 292-style copies, their choice.

## §2z — The human's first strawmen-reaction round (typed 2026-07-25; pre-reading the as-built)

Ruling-grade (typed firm):
- **`289:rul-ascii-output-forever`** — "no unicode, ever. period. anywhere...
  permanently": product output is pure ASCII, 90s-leaning, forever. NOTE the
  as-built chain gutter uses `└─` (top_run_reason) — an ASCII respell is owed,
  cheap, rides any render-touching lane; committed transcripts re-bless with it.
  Unifies with spike/docs' existing ASCII law; steering-sync rides next lane.
- **`289:rul-trust-spent-first-argless-why`** — the zero-arg report leads with
  TRUST-SPENT, always; danger in the user's face before anything else.

Typed leans (banked, not welded):
- lean-prose-down-one-step (7/10 → ~6/10 prose-iness; "what would fix/what would
  bring back" become STRUCTURAL sections, not paragraphs — annoyed-debugging
  register).
- lean-why-is-whylog-reconciliation — `dorc why` may collapse to
  always-receipt-reconciling (against the on-disk book); the live/fuller render
  becomes `dorc plan --why`; why/plan differ only in consent + ask over the one
  global model. (Conductor correction owed in-chat: as-built live why does NOT
  probe — it recomputes from supplied records; the fold is a surface
  simplification, not a safety fix. Hardens the always-on-whylog requirement;
  raises the drifted-book question: refuse-on-desync (22F, as-built) vs
  reconcile-with-drift-annotations.)
- lean-tree-rendering-is-its-own-firewalled-crate — nested code/prose/why-block
  rendering + reflow + syntax-highlighting = a segregated internal mini-product
  (errorloom-precedent); NEEDS INVENTORY BEFORE SHOPPING (no library-bending);
  fixpoint-reflow fear noted (conductor pointer: Wadler/Oppen document-algebra
  pretty-printing is the battle-tested linear-ish prior art; sh highlighting can
  ride Dorc's OWN lexer, no foreign grammar).
- lean-start-without-mutual-awareness — cross-fragment prose awareness starts
  OFF; (conductor distinction offered: walker-DERIVED structural awareness
  ("the only claimed link") is cheap and precedented — the flagship epilogue is
  already derived-from-evidence-presence; only prose-knowing-prose is deferred).
- nit-why-steps-are-a-dag — numbering must express join shape (1a/1b/1c -> 2),
  not force a false total order.
- ask-cell-human-description — why-surfaces want "what is this cell tracking,
  in user terms"; possibly a first-class oracle-language display-tier surface
  (richer user-facing oracle metadata generally); comment-mining is the unloved
  fallback. Joins `26C:feeder-oracle-why-metadata`'s umbrella; NOT settled.
- "problems" retitles partially toward "improvements" (could-do-if-you-did).
