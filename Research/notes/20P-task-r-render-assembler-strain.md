# 20P — task-R: the one artifact-assembler module (`cm-3`'s second target); emission inventory + findings

> Round-20 spike note, append-only. Records task-R (the `cm-3` countermeasure's SECOND target,
> 20A §2 / 20D §3/§5: the render-assembler, named there "the largest remaining fam-B surface" and
> "the next cm-3 candidate"). Extract ONE module (`plan::render`) owning every place the engine
> EMITS sh text, so future render work has a single audited home. Behavior-preserving by mandate:
> ZERO golden changes, ZERO test-outcome changes (verified §5). The deliverable is *the API + the
> emission inventory + what diverged/resisted + bypass findings*, NOT green tests. AI-authored,
> confidence-marked. Trust R/D/I/K + 19H/19I + the human rulings over this. Companion to 20D (task-S,
> the WORD-semantics `cm-3` module `sem` — task-R is its render-side sibling) and 20G item-4 (the
> T14 in-situ case-arm substitution this assembler now hosts).

## §0 What landed (gates §5; all green within task-R scope)

New module `spike/crates/plan/src/render.rs` (`pub mod render`), dependency-clean (depends only on
`dorc_core` for `EntityRef`/`Interner` and `dorc_syntax::sem` for the one quoting call — both already
in `plan`'s dep graph). It owns **byte-level sh construction** for all three renders; the render
*methods* (`Plan::render_sh`, `Plan::render_apply`, `ProbePlan::render_sh`) keep the *decision* logic
(which site/line/disposition gets which treatment — entangled with the `Plan`/`ProbePlan`/`Cfg`/`Ast`
walks). The split mirrors 20D's rs-* boundary for `sem`: the dep-light kernel hosts the rule; the
caller holding control-flow state applies it.

Two sub-modules + one free fn:
- `render::standin_sh(StandIn) -> String` — the value-preserving substitution bytes
  (`true`/`false`/`(exit n)`). `StandIn::sh()` now DELEGATES here (kept as the public method —
  `observable_matrix.rs` calls it).
- `render::probe::{header, site_comment, wrapper_def, invocation, record_scaffold, unresolvable_comment}`
  — the probe artifact (`inv-site-keyed-results`).
- `render::apply::{plan_header, apply_header, flat_replace_block, flat_omit_block, inline_arm_subst,
  commented_line}` — the two apply renders.

Every emitter is doc-commented with a `GUARANTEE:` line stating WHAT its output carries: under which
preconditions it is `dash -n`-clean, and which gate proves it (the prompt's mandated shape, mirroring
20D's per-fn XCU-clause + dash-note convention).

The seeded crate-root `#![expect(..., clippy::format_push_string, ...)]` in `plan/src/lib.rs` was
RATCHETED (the `format_push_string` token removed): the refactor eliminated the LAST
`push_str(&format!(…))` in the crate (the emitters return owned `String`s; methods do
`push_str(&emitter(…))`). This is the spike's documented self-ratchet ("an unfulfilled expect warns,
so it self-removes as the seeded layer is replaced", spike/CLAUDE.md), NOT a policy relaxation —
`render.rs` is fresh code under the full lint bar and needed no expects. `missing_docs` +
`arithmetic_side_effects` stay (still fulfilled by other seeded lib.rs code).

## §1 The module API (the deliverable)

```
// free fn — the substitution bytes, ONE home (StandIn::sh delegates here)
pub fn standin_sh(stand_in: StandIn) -> String

pub mod probe {
    pub const fn header() -> &'static str                                  // #!/bin/sh + record-grammar banner
    pub fn site_comment(site: LeafId, label: &str) -> String               // # site N: <label>
    pub fn wrapper_def(fn_name: &str, body: &str) -> String                // <fn>() <brace-group-body>
    pub fn invocation(fn_name: &str, entity: EntityRef, interner: &Interner) -> String  // F-QUOTE site
    pub fn record_scaffold(invocation: &str, site: LeafId) -> String       // …; _rc=$?; if…; printf 'site N…'
    pub fn unresolvable_comment(site: LeafId) -> String                    // # site:N skip-unresolvable
}

pub mod apply {
    pub const fn plan_header() -> &'static str                             // flat-listing banner
    pub const fn apply_header() -> &'static str                            // runnable-rewrite banner
    pub fn flat_replace_block(leaf: u32, sh: &str, stand_in: StandIn, fact_label: &str) -> String
    pub fn flat_omit_block(leaf: u32, sh: &str) -> String
    pub fn inline_arm_subst(prefix: &str, stand_in: StandIn, suffix: &str) -> String   // T14 in-situ
    pub fn commented_line(line: &str, indent: &str, filler: &str) -> String            // whole-line
}
```

Design choices (within the prompt's "refine within" latitude):
- **Free fn for `standin_sh`, not a method**, because the stand-in bytes are consumed by BOTH apply
  sub-emitters and `StandIn::sh()`; a free fn is the shared root all three reach.
- **`probe` / `apply` sub-modules**, not a flat namespace, because the two artifacts have disjoint
  emitters and grammars; the split makes "which artifact is this byte for" syntactically obvious and
  keeps the doc-blocks co-located by artifact.
- **Emitters take already-decided inputs** (a resolved `EntityRef`, a chosen `StandIn`, pre-sliced
  `prefix`/`suffix`), never a `&Plan`/`&Cfg`. They never reach back into a plan — the one-way
  dependency keeps the assembler a leaf the orchestration calls, not a co-recursive partner.
- **Headers are `const fn`** (pure `&'static str`) — no allocation for the fixed banners.

## §2 Emission-site inventory (before → after) — the mandated §-table

Every site in `plan/src/lib.rs` that produced sh-artifact or stand-in *bytes*, mapped to its
assembler home. "Before" line-refs are the pre-task-R `lib.rs`.

| # | construct (before) | before site | after (assembler home) |
|---|---|---|---|
| e-1 | probe header `#!/bin/sh …` const `PROBE_HEADER` | `lib.rs` const ~687 | `render::probe::header()` |
| e-2 | per-site comment `# site N: <label>` | `render_sh` ~640 | `render::probe::site_comment` |
| e-3 | wrapper funcdef `<fn>() <body>` | `render_sh` ~647 | `render::probe::wrapper_def` |
| e-4 | invocation w/ F-QUOTE operand bind | `render_sh` ~652–659 | `render::probe::invocation` (the lone quoting site) |
| e-5 | self-report scaffold `…; _rc=$?; if…; printf` | `render_sh` ~660 | `render::probe::record_scaffold` |
| e-6 | unresolvable `# site:N skip-unresolvable` | `render_sh` ~672 | `render::probe::unresolvable_comment` |
| e-7 | stand-in bytes `true`/`false`/`(exit n)` | `StandIn::sh` ~440 | `render::standin_sh` (`StandIn::sh` delegates) |
| e-8 | flat-plan header | `render_sh`(Plan) ~988 | `render::apply::plan_header()` |
| e-9 | flat replace comment-block | `render_sh`(Plan) ~998 | `render::apply::flat_replace_block` |
| e-10 | flat omit comment-block | `render_sh`(Plan) ~1007 | `render::apply::flat_omit_block` |
| e-11 | apply (line-granular) header | `render_apply` ~1120 | `render::apply::apply_header()` |
| e-12 | T14 in-situ arm subst (prefix+standin+suffix+comment) | `render_apply` ~1083–1090 | `render::apply::inline_arm_subst` |
| e-13 | whole-line comment-out + stand-in filler | `render_apply` ~1103–1108 | `render::apply::commented_line` |
| e-14 | verbatim line / leaf pass-through | `render_apply` ~1110 / `render_sh` Run-arm | NOT moved (trivial `push_str(line)` — see rs-1) |

### Residual sh-text-assembly sites NOT moved (and why)

- **rs-1 — verbatim pass-through** (`out.push_str(&step.sh)` / `out.push_str(line)`). A literal copy
  of book bytes is not *assembly* — there is no construct to get wrong, no dash-belief, no quoting.
  Moving it to an `emit_verbatim(s) -> String` wrapper would be ceremony with zero audit value.
  LEFT inline. (Contrast e-12/e-13, which interleave the verbatim slice WITH stand-in/comment bytes —
  those carry the empty-clause hazard, so they ARE assembly and moved.)
- **rs-2 — `check_fn_name`** (`<kind>_<selector>__check`). Assembles a POSIX *identifier*, not
  sh-statement text; its dash-correctness (hyphen→underscore) already lives in
  `dorc_oracle::to_funcname_segment` (a `cm-3`-style single source). It FEEDS the emitters (passed to
  `wrapper_def`/`invocation`) but is name-construction, not artifact-emission. LEFT in `lib.rs` (it
  needs the `Interner` + the oracle funcname map; it is the render methods' input, like `fact_label`).
- **rs-3 — `fact_label`** (`kind:entity#selector`). Display/provenance label (`inv-referent-agnostic`,
   rides in comments + the cli's stdin grammar), not sh-statement text. Its bytes appear inside e-2/e-9
  comments, but the LABEL grammar is the cli's stdin contract, not a render-assembly belief. LEFT in
  `lib.rs`. (Arguable: it is "text the engine emits". I scoped task-R to sh-*statement*-assembly — the
  dash-`-n` surface — not every interned-string formatter; `fact_label` carries no dash hazard.)
- **rs-4 — the `:` wholly-dead filler choice** (`line_standin.get(&i)` else `":"`). The *decision* "a
  dead Omit-only line gets `:`, a survivor gets its stand-in" stays in `render_apply` (it reads
  `line_standin`, a per-line decision map). The `:` STRING is trivial; the stand-in string is
  `standin_sh`. The emitter (`commented_line`) takes the already-chosen `filler: &str`. Correct
  boundary: the byte-source is centralized, the *which-filler* policy stays with the line walk.
- **rs-5 — test fixtures + assertions** (`CORPUS_CHECK_SRC`, `rendered.contains("printf 'site …")`).
  Test INPUT (a check-dialect strawman) and test CONSUMERS of the assembler output. Not engine
  emission. Untouched (per safety: fixtures are spec).

## §3 Same-construct-different-assembly divergences (the task-S dv-pattern — the headline)

The prompt's headline ask: where two emission sites assemble the SAME construct DIFFERENTLY (latent
fam-B). Result, after putting every site side-by-side in one module: **no divergence-BUG found; one
benign-by-design variant pair preserved; one checked-and-dismissed near-miss.** +SURE (traced in
source). The render-assembler is in this respect HEALTHIER than the word-semantics module was (20D §1
found three real representational divergences); the render bytes were already fairly disciplined.

- **dv-render-1 (benign, preserved as named variants) — the two apply headers differ.**
  `plan_header()` (`# dorc plan (apply phase). Replaced leaves are already converged.`) vs
  `apply_header()` (`# dorc apply: the book, with already-converged/dead lines elided …`). Same
  *construct* (a `#!/bin/sh` + one-comment banner), DIFFERENT text. This is NOT a bug: the flat
  `render_sh` is a per-leaf disposition LISTING (drops guards, not runnable); `render_apply` is the
  runnable book-faithful rewrite. They are genuinely different artifacts with different contracts, so
  their banners SHOULD differ. Preserved as two named `const fn`s (per the prompt's "preserve both
  behaviors behind named variants") so neither silently converges onto the other — a convergence would
  be golden churn AND would mislabel one artifact as the other. Flagged here so a future reader does
  not "DRY" them into one shared header (that WOULD be a regression). ~SUSPECT this is the ONLY
  same-construct pair in the whole assembler.
- **dv-render-2 (checked, dismissed — NOT a divergence) — the leaf-id appears in THREE formats.**
  `# site N:` (e-2, space) · `# site:N skip-unresolvable` (e-6, colon) · `printf 'site N effect=…'`
  (e-5, no `#`). At first glance "the same id, spelled three ways." On inspection these are THREE
  DISTINCT GRAMMAR TOKENS in two different lanes: e-5 is a runtime stdout RECORD the cli's
  `parse_results` keys on (`site N` un-prefixed); e-2/e-6 are ARTIFACT COMMENTS (`#`-prefixed,
  human/differential-facing) — and e-2 vs e-6 mark different site classes (resolvable-with-record vs
  unresolvable-no-record). They are not one construct assembled inconsistently; they are different
  outputs that happen to embed the same integer. Left as-is (unifying them would CHANGE the cli's
  stdin grammar — a behavior change). Recorded so the near-miss is not "found later" and mistaken for
  unfinished work.
- **The stand-in bytes were ALREADY single-sourced** (via `StandIn::sh()`), consumed at e-7/e-9/e-12/
  e-13. So no divergence was even possible there pre-task-R; task-R only relocated the single source
  from a method to `render::standin_sh` (the method delegates). Noted because it is the strongest
  evidence the render layer had less latent fam-B than the word layer — the human's `StandIn` enum
  (19A §5) had already forced one home for the highest-risk bytes (the `(exit n)`-not-`true`
  under-execute fix).

## §4 Quoting-bypass audit (the prompt's explicit check)

**Finding: NO quoting bypass. +SURE — proven by the compiler.** The F-QUOTE single-quote is the only
quoting decision in any artifact, and it routes through `dorc_syntax::sem::single_quote` (the 20D §6
`cm-3` word-quoting home) at exactly ONE call-site: `render::probe::invocation`. Evidence:
- After moving the F-QUOTE call out of `render_sh` into `render::probe::invocation`, the `use
  dorc_syntax::sem;` import in `lib.rs` went UNUSED (compiler warning) — i.e. `lib.rs` no longer
  performs ANY `sem` operation, quoting included. Removed the import. So the only `sem::single_quote`
  caller in the whole crate is now the one assembler fn.
- No emitter hand-rolls a quote: a grep for `'\\''`/manual `'`-wrapping across `render.rs` finds only
  the `sem::single_quote` delegation. The comment/header/record emitters emit FIXED text or
  `#`-commented book bytes (a comment needs no quoting — it is not re-parsed), and the verbatim
  pass-through (rs-1) copies already-valid book bytes.
- The operands that flow to `invocation` are interned book tokens (`EntityRef::Operand`); every one is
  single-quoted before it touches the artifact. A `Singleton` carries no operand, so it emits the bare
  fn name (nothing to quote). This is the `kFAIL` both-directions guarantee (`notes/198`): no
  word-split (wrong entity, `kFAIL-perform`), no metachar re-parse (probe-mutation, `kFAIL-withhold`).

## §5 Behavior-preservation evidence (the mandate) + gates

- **`git status spike/e2e`: EMPTY** — zero golden diffs (+SURE; the load-bearing check). The probe +
  both apply artifacts are byte-identical, so no consumer's observable output moved. This is the real
  proof the relocation is pure: the goldens are the rendered bytes, and not one changed.
- **`sh e2e/run.sh`: 57/57, ZERO xfail**, all six gates (ap-2 `dash -n` + apply/probe exec gates,
  redirect sandbox, ordered run-set, stderr floor, argv-echo differential). Identical to baseline.
- **`cargo test --workspace`**: identical pass counts to baseline (plan 34 + plan-integration 19+1ignore
  + 18; core/syntax/oracle/analysis 84/45/9/4/8 — every number unchanged; the 1 ignore is the
  pre-existing HOLE#1 subst-in-redir spec, untouched). No test added, removed, or weakened — the
  existing render tests (`probe_render_*`, `render_one_liner_case_arm_*`, `converged_ambient_install_*`)
  pass UNCHANGED, which is the preservation proof: they pin the exact bytes the old inline assembly
  produced, and they still pass against the relocated emitters.
- **`cargo clippy --workspace --all-targets -- -D warnings`**: clean. NO new `#[expect]`s
  (`render.rs` is fresh code under the full bar, needed none); ONE seeded expect token ratcheted away
  (`format_push_string`, §0) — the sanctioned direction, never a relaxation.
- **`cargo fmt --check`**: my two files (`lib.rs`, `render.rs`) are clean (verified via `rustfmt
  --check` on them directly). NB the workspace `fmt --check` reports a diff in
  `analysis/src/value.rs` — a CONCURRENT, unrelated user edit (a temporary `zzz_baseline_glob_word_*`
  probe test in the analysis crate, present in the live worktree, NOT touched by task-R and outside
  its scope). Flagged, not touched (per the human-direct-edit + live-worktree discipline).
- **`typos spike`** (from worktree root): clean.

## §6 What the next render slice inherits (the cm-3 compounding payoff)

- **The assembler is the home for the next render dash-fact.** The prompt named three future surfaces;
  each now lands as a new `render::` emitter with a `GUARANTEE:` doc, NOT a fresh inline `format!`:
  - **guard-capable substitution** (retiring the `StatusRenderFloor`, C-5/seam-prov): a new
    `render::apply::inline_guard_subst` reproducing an `if`/`elif` guard's Status channel in-situ while
    keeping `then`/`fi` balanced — the 20G §6 flag's "harder than the case-arm" case. Its GUARANTEE
    doc is where the `then`/`fi`-balance precondition gets stated and gate-proven.
  - **member-elision list rewriting** (`209` brk-1(b)): a `render::apply` emitter rewriting a `for`'s
    iteration-list to the diverged members. Lives beside `inline_arm_subst` (both are in-situ
    byte-span rewrites of a structural construct).
  - **check-body shipping with `rule-anno-render`**: the moment any emitter ships `check()`-body
    spans, `dq-reflexive-probe-inertness`'s vouch-closure REVIVES (20A standing ruling) — and the
    revival trigger is now a single grep-able place (`render::probe`), not scattered `format!`s.
- **dv-render-1 is a standing tripwire**: do NOT merge `plan_header`/`apply_header`. If a future change
  makes them identical, that is a signal the two renders' contracts have converged (worth a conscious
  decision), not a DRY cleanup.
- **The assembler now makes a render `dash -n` failure a UNIT-level locus** (mirroring 20D §5's "a
  dash-divergence becomes a `sem` unit-test failure"): each emitter's GUARANTEE names its precondition,
  so a broken artifact traces to the one emitter whose precondition was violated, rather than an
  end-to-end mystery. The e2e `dash -n` gate remains the live proof; the module is where a reviewer
  reads the claim.

## §7 Confidence summary

- +SURE: zero golden churn (git-verified), 57/57 e2e zero-xfail, identical workspace test counts — the
  relocation is behavior-preserving.
- +SURE: no quoting bypass; the lone `sem::single_quote` call-site is `render::probe::invocation`,
  proven by the now-unused `sem` import in `lib.rs`.
- +SURE: no same-construct-different-assembly divergence-BUG; dv-render-1 (the two headers) is benign
  by-design and preserved as named variants; dv-render-2 (three site-id formats) is two distinct lanes,
  not a divergence.
- +SURE: the `format_push_string` ratchet is the documented self-removal (last such pattern gone), not
  a policy relaxation.
- ~SUSPECT: the rs-2/rs-3 boundary (leaving `check_fn_name`/`fact_label` in `lib.rs`) is the right line
  — they are identifier/label construction with their dash-correctness already single-sourced
  elsewhere, not sh-statement assembly — but a stricter reading of "every place the engine emits text"
  could pull them in. I scoped to the `dash -n` statement surface; flagged for the orchestrator if a
  wider scope is wanted.
- ~SUSPECT: the `analysis/value.rs` fmt drift is purely a concurrent unrelated user edit; I did not
  touch it and the worktree is live. If `cargo fmt --check` must be globally green for the commit, that
  file (the user's `zzz_baseline_glob_word_*` probe) needs the human's attention, separate from task-R.
