# 27N — block-context lane-integration landing + residue

AI-authored (Opus builder, r27 lane-integration session, 2026-07-17). Records what landed for
`270:block-context`'s INTEGRATION lane (`27D` lane-integration mint) — the cross-pipeline wiring
that makes a wrapped BOOK site actually elide end-to-end (the "real sudo mechanism", block-stdlib's
precondition, `270` §2). Authority: root docs + `spike/CLAUDE.md` rulings + `271`/`273`/`274`/`27C`/
`27D`/`27K`/`27L` outrank this. Companions: `27D` (the ledger / lane-integration mint), `27K`
(lane-wrapper-peel — the peel machinery this consumes), `27L` (lane-payload-v1 — the shim model),
`plans/27C` (THE wrapper/context spec — §8 babby-sudo is the acceptance).

## Branch / fold state (READ FIRST — the conductor must reconcile)

- Branch `ai/r27-book-integration`, based on `ai/spike3-r27` @ **`42cdaca`** (the brief's corrected
  base — the conductor's mid-run step-zero fix; parent `c4c3276`). NOT rebased mid-build (fold-time
  rebase is the protocol; the lineage tip moved to doc-only commits `eba4a73`+ during the run — no
  code overlap, the rebase should be clean, but VERIFY).
- Tip **`c2cc57f`**. 11 commits (`8b4b5fc..c2cc57f`), oldest→newest:
  1. `8b4b5fc` (new oracle) — book-side peel execution: `peel_consumed` + `resolve_lend_values`.
  2. `b002157` (re) — **the conductor's `rider-strike-is-diverged-dual-residue`** (below).
  3. `be4717f` (new oracle) — `peel_book_chain` (`WrapperIndex`/`WrapperModel`) + `strip_enter`.
  4. `b3bb412` (fix oracle) — clippy arithmetic-side-effects fix.
  5. `fc073c2` (new ana) — thread the peel-map into classify; wrapped sites born in-context.
  6. `d22f0c7` (new cli plan) — entry-composed probe emission (`WrappedProbes`, `EntryComposed`).
  7. `eff0228` (new plan cli) — wrapped-site vouches (converged-in-context elides).
  8. `e4f5e6d` (new e2e) — babby-sudo acceptance: 4 cases.
  9. `57cf61c` (test plan) — machine-pin entry-composed probe ships oracle bytes.
  10. `f4b4ac9` (re e2e) — tighten fixture headers (comment budget).
  11. `c2cc57f` (re) — tighten verbose doc-comments (comment budget).

## Acceptance summary (all green at the tip)

- **Four gates clean** on the whole workspace: `cargo fmt --check` · `clippy --workspace
  --all-targets -D warnings` (0 warnings) · `cargo deny check licenses bans sources` · `typos spike`.
- **Unit: 863** (was 857; +6: `peel_consumed`×2, `resolve_lend_values`, `babby_sudo_peels`,
  `unwrapped_site_does_not_peel`, `entry_composed_probe_renders_enter_forms_never_raw_book_bytes`).
  No failures.
- **e2e: 88/88** foreground (was 84). The **84 pre-existing cases are BYTE-STABLE** (rung-0 proof) —
  including the landed `wrapper-entry-form-coheres` / `wrapper-entry-incoherent-fails-fast` /
  `wrapper-modeled-peel-coheres-walls` (their inner commands are unmodeled `hork`, so a peel yields
  an opaque inner ⇒ the site walls exactly as before). 4 new babby-sudo cases.
- **REFERENDUM (`270` §2 / `273` §11) did NOT fire**: no build contact forced a wrapper-aware arm
  into a TOOL oracle. All wrapper/entry awareness lives in `oracle::{wrapper,entry}` + the cli edge;
  `analysis::classify` consumes a PRE-COMPUTED peel-map (pure data), never the wrapper models.

## The five wiring points (rework-vs-conformant accounting against the brief)

1. **Peel wrapped BOOK sites in classify/value — CONFORMANT (new).** `classify_with_why_diags` gains
   a `peeled: &BTreeMap<CfgNodeId, PeeledSite>` param; a wrapped node resolves its INNER command via
   `command_effect` on the peeled argv and re-keys the fact `in_context` (`peeled_node_effects`). The
   thin `classify` wrapper + all non-cli callers (coverage/sweep/plan-tests) pass an empty map
   ⇒ rung-0 byte-identical. The peel-map is built at the cli (`build_wrapped_analysis` →
   `oracle::entry::peel_book_chain`).
2. **Emit the entry-composed probe — CONFORMANT (new).** `ProbePredict` gains an
   `entry: Option<EntryComposed>` field; `compile_probe` gains a `wrapped: &WrappedProbes` map and
   ships the entry-composed form (`sudo__enter <inner-check> <peeled-argv>`) for `WrappedProbe::Enter`
   sites, else records them unresolvable (Degrade ⇒ run). Only-oracle-bytes law holds: the render
   emits the enter funcdefs + the inner check funcdef + a nested invocation — never book bytes. The
   machine pin `composed_probe_renders_predicts_never_raw_book_bytes` is EXTENDED with a sibling
   `entry_composed_probe_renders_enter_forms_never_raw_book_bytes`. Batching: one entered segment per
   composed-context key falls out of the `FactKey.context` keying (same-context wrapped facts share a
   key). ⚠ The real boundary-crossing execution needs the per-run PATH shim MATERIALIZATION (`274`
   §5 / `27L` task-14) — DEFERRED (disclosed gap below).
3. **Thread the composed `Context` into the shipped `FactKey` + records lane — CONFORMANT.** The
   context rides `FactKey.context` (already landed by lane-context-entry — `FactKey::in_context`),
   which the probe carries in `ProbePredict.fact` and the plan reads via `observe(fact)`. **The
   records-lane WIRE grammar is UNCHANGED**: records key by SITE (LeafId); the cli re-keys site→fact
   (context-carrying) from the probe plan, so the wire needs no context field. **FactKey-widening
   decision (I OWN this, né `tc-context-slot-on-coord-not-factkey`, the 47-site map revisit):** done
   via the pre-computed CLI peel-map threaded into classify, NOT via wrapper-recursion inside
   `command_effect`. Minimal honest shape; kernel stays wrapper-unaware. See tc-flags.
4. **Read back the context-qualified verdict — CONFORMANT (rides FactKey).** `observe(fact_in_context)`
   is context-qualified automatically because the fact carries the context. No-collision /
   no-cross-context-transport / cross-context-compare=Unknown pins (landed) stay green.
5. **Elide/guard per the consent trace — CONFORMANT (elide/run; in-context GUARD deferred).** The
   dial × capability × vouch decision (`decide_entry`) governs at the cli: default dial shifts only
   `tolerates:`-vouched functions; unvouched ⇒ run + the one-line adoption hint
   (`wrapped-site-adoption-hint`); every degrade rung ⇒ unresolvable ⇒ run. Converged-in-context ⇒
   elide (a wrapped vouch minted from the inner verdict over the peeled argv, `build_wrapped_vouches`).
   The babby §8 story is elide-or-RUN (establish sites); the in-context GUARD (`27C` §5, query sites)
   is scaffolded (`EntryComposed` carries the guard-shape invocation in the wrapped vouch) but NOT
   exercised — no wrapped query site in the acceptance; the conditional-tail cascade is FENCED OUT.

## Acceptance evidence (`27C` §8 babby-sudo, e2e under inert mocks)

Book (all 4 cases): `hork install wombat` (ambient) + `sudo hork install frob` (sudo-WRAPPED).
`hork`/`sudo` are inert mocks. The unwrapped site is FIRST (see the context-blind-wall limitation).
The wrapped site's shipped probe (pinned in every golden):
`sudo__enter hork__is_converged 'install' 'frob'` — the sudo entry form wrapping the inner verdict
body, invoked with the PEELED argv; only oracle bytes; the book bytes `sudo hork install frob` never
appear in the probe.

- **`context-entry-babby-elides`** (`:   : tolerates:user`, both converged): BOTH elide —
  `# hork install wombat # elided` + `# sudo hork install frob # elided`. Two contexts, both
  converged, both answered.
- **`context-entry-babby-diverges`** (vouched; wombat holds ambient, frob absent-in-root): wombat
  ELIDES, frob RUNS — `# hork install wombat # elided` + `sudo hork install frob`. Two contexts, two
  independent answers, nothing traveled, no flag typed.
- **`context-entry-unvouched-runs`** (no `tolerates:`): frob RUNS + the adoption hint fires
  (`wrapped-site-adoption-hint`, stderr); wombat elides.
- **`context-entry-noescalation-runs`** (`--no-probe-escalation`, the degrade case): the dial forbids
  ⇒ frob RUNS; wombat elides.

All four: `PROBE_RESULTS=authored` (the entry-composed probe cannot self-report faithfully under inert
mocks without the deferred shim — the simulated results drive the apply; emission is pinned by
expected.out). The `wrapper-entry-form-coheres` / `-incoherent-fails-fast` fixtures stay green
(byte-stable, unmodeled inner).

## Per-case golden delta classes

- 84 pre-existing e2e: **NO delta** (byte-stable; `empty-world-byte-identical` — no wrapper oracle in
  the pre-existing corpus, and `hork`-inner wrapper cases wall as before).
- 4 NEW cases (delta class = **new-fixture**, no pre-existing golden churned).

## Comment budget (`24P` §8 rider) — OVER, with disclosed tension

- `git diff ai/spike3-r27...HEAD --numstat` denominator (added lines): **1509**.
- `git diff ai/spike3-r27...HEAD | grep -cE '^\+\s*(#|//)'` numerator: **421** (27.9%).
- **Breakdown:** Rust `///` doc-comments = **170** (11.3% of the denominator ALONE); Rust inline
  `//` ≈ 67; sh `#` ≈ 184, of which ~113 are NON-AUTHORED (generated `expected.out` goldens ≈ 69 +
  copied `mocks/.log` ≈ 44). Authored comments ≈ 308; authored non-golden denominator ≈ 1269 ⇒ ~24%.
- **The 10% target is unreachable for this lane.** `spike/CLAUDE.md` MANDATES a doc-comment on every
  public type/fn ("Doc-comment every public type/fn with *why*, citing the research slug"); this lane
  adds substantial new public API (`peel_consumed`/`resolve_lend_values`/`peel_book_chain`/
  `WrapperModel`/`WrapperIndex`/`PeeledChain`/`strip_enter`; `EntryComposed`/`WrappedProbe`/
  `WrappedProbes`/`build_wrapped_vouches`; `PeeledSite`; the cli helpers). The 170 required
  doc-comments exceed 10% on their own. I trimmed ALL trimmable comment mass (redundant fixture
  headers, verbose doc-blocks, generated-golden inflation identified) without stripping required or
  load-bearing (invariant-citing / deferral-disclosing) doc-comments. **FLAGGED for adjudication:**
  the budget-vs-doc-comment-rule tension is structural for a public-API-heavy lane.

## tc-flags (flagged UP, NOT settled)

- **`tc-factkey-widening-via-cli-peelmap` (né `tc-context-slot-on-coord-not-factkey`, 47-site map
  revisit — I OWN it at this brief, implemented, flagging the deeper judgment).** The wrapped-site
  peel + context re-key is done via a CLI-precomputed peel-map threaded into classify, NOT via
  wrapper-recursion inside `command_effect`. This keeps the densest crate wrapper-unaware and rung-0
  safe. The `analysis/CLAUDE.md` `thread-the-flat-coordinate` direction anticipates peel eventually
  moving INTO `command_effect`; whether to migrate it there (kSTATE-adjacent, cross-cutting) is a
  conductor+human judgment, NOT settled here.
- **`tc-context-blind-stage1-wall`** — a wrapped establish that RUNS (diverged/degraded) walls
  DOWNSTREAM converged establishes via the context-BLIND Stage-1 total wall (survival off). So a
  running `sudo hork install frob` demotes a downstream ambient `hork install wombat` even though
  the contexts are disjoint. SAFE (over-execute, priority-2), but over-conservative. The acceptance
  fixtures order the unwrapped site FIRST to sidestep it. Context-qualified WALL sparing (compare
  answers Unknown across the context gap ⇒ collide at Stage-1) is a survival-tier follow-on, NOT
  this lane. Flagged, not settled.
- **`tc-wrapped-guard-shape-unexercised`** — the in-context GUARD (`27C` §5,
  `( sudo__enter check ) || <original bytes>`) is SCAFFOLDED (the wrapped vouch carries the
  entry-composed invocation + preamble) but no wrapped QUERY site exercises it (the babby establish
  is elide-or-run). The guard-render path for a wrapped vouch is UNVERIFIED end-to-end. Flag for the
  lane that lands wrapped query sites.
- **`inv-superposition`**: nothing needed flagging UP — the peel-map + PeeledSite are phase-agnostic
  data; the entry DECISION collapses at the cli/plan edge, never in the kernel.

## Conductor riders (accounting)

- **`rider-strike-is-diverged-dual-residue`** (commit `b002157`): DONE. `strip_verdict` drops the
  `mangled_suffix` param and hardcodes `"__is_converged"` (matching every sibling); callers
  (`cli`/`plan`) + the test updated; `VERDICT_SUFFIX` KEPT (genuinely consumed by
  `plan::verdict_fn_name`, not orphaned). All dual `is_diverged`/`VerdictSense` doc residue struck
  across the 9 named sites + 2 the rider missed (`derive.rs`, `parser.rs`). The new `__enter` role
  got its OWN hardcoded `strip_enter` (not the parameterized shape). Acceptance:
  `grep -rn is_diverged crates --include=*.rs` returns ONLY `reserved.rs`; no `VerdictSense`
  survives; the `is_diverged_is_neither_reserved_nor_recognized` pin stays green.
- **single-plan-mint choke-point** (conductor rider): CONFORMANT. All wiring threads DATA (peel-map,
  `WrappedProbes`, wrapped vouches) INTO the existing `build_plan_walled` (the sole plan mint) and
  `compile_probe`→`ProbePlan` (the sole probe mint). NO second plan/probe-assembly exit was added.
- **27H capture seams stay untouched + OPEN** (conductor rider): CONFORMANT. This lane is
  entry-composition, not read-value; no capture / `seam-re-bind` / read-value seam was touched,
  narrowed, or closed.

## Disclosed gaps / deferred pieces (`ru-26`)

- **Shim MATERIALIZATION (the last mile).** The entry-composed probe SHIPS the composed form but the
  real `sudo` boundary crossing needs the per-run PATH shim to materialize oracle bytes as
  executables (`274` §5 / `27L` task-14). Deferred: the acceptance drives the apply via simulated
  results (`PROBE_RESULTS=authored`); the emission + context-qualified readback are proven, the
  boundary-crossing execution is the shim-materialization follow-on. Under inert mocks the shipped
  form 127s (mock `sudo` cannot exec a shell-function guest) ⇒ can't-say ⇒ run (SAFE).
- **Cross-link ρ-threading.** `peel_book_chain` resolves each link's mapped values against its OWN
  peeled argv; the `27C` §3 ruling-3 cross-link ρ-threading (an inner link's argv referencing an
  outer link's claimed env) is NOT threaded — an unresolvable ρ-dependent value degrades to ⊤ ⇒
  walls (SAFE). Single-link chains (the acceptance) are unaffected.
- **`has_entry_form` conservatism.** `decide_entry`'s `has_entry_form` is computed as "every chain
  link has an entry form" — a mixed chain (an identity `nice` link with no entry form + a crossing
  `sudo` link with one) conservatively DEGRADES. SAFE (run); refine per-crossed-dimension attribution
  if a real mixed chain wants it.
- **Marked-oracle inner (predict-inner).** `resolve_inner_check` prefers the inner `__predict` body,
  falling back to the auto-cell `__is_converged` — the acceptance exercises only the VERDICT-only
  (auto-cell) inner. The predict-inner entry-composed path exists but is UNVERIFIED end-to-end.
- **`resolve_lend_values` value model** — the strawman `printf FMT VAL… : dim` idiom only; any other
  producing shape is a value-⊤ (walls). A richer mapped-value surface is a later refinement.

## What later lanes must maintain

- **read-value-slice** (next): the capture fold on the landed wire + recipe machinery. Do NOT touch
  the 27H capture seams (STRUCK from this block, deferred to r26 per the conductor). The
  `ProbePredict.entry` field + `EntryComposed` are additive — a read-value capture composes with
  entry composition (a captured value inside an entered context) additively.
- **lane-fallback-carry** (`27C` §4): the conservative cross-context carry (read-set-closure pass).
  The `compare`-answers-Unknown-across-context gap this lane relies on is the SAME chokepoint the
  carry lane will license through; keep it. `tc-context-blind-stage1-wall` is the survival-tier
  cousin — context-qualified wall sparing lands there or in the survival round.
- **block-stdlib**: the ~40 bootstrap oracles author against THIS surface. The `sudo__enter` /
  `hork__is_converged` strawman shapes (colon-line `tolerates:`, all-3-dimension `lend_map`) are the
  authoring template. The stdlib sudo entry form's siting-vouch discharge (`27C` §9: the `-l` gate +
  same-head tripwire) is stdlib work, not built here.
- **Whoever migrates the FactKey-widening into `command_effect`** (the `tc-factkey-widening` flag):
  the peel-map's `PeeledSite { inner_argv, context }` is the exact data `command_effect` would need;
  the re-key logic is `peeled_node_effects` (already isolated).
