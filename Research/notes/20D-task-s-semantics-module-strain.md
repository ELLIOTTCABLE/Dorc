# 20D — task-S: the one shell-semantics module (`cm-3`); divergences + coverage map

> Round-20 spike note, append-only. Records task-S (the `cm-3` countermeasure, 20A §2): extract
> ONE word-semantics module (`dorc-syntax::sem`) and route every existing model-site through it.
> Behavior-preserving by mandate: ZERO golden changes, ZERO test-outcome changes (verified §4).
> The deliverable is *what diverged between the old copies* (the latent fam-B quarry) + *what
> resisted extraction* + *the coverage map*, NOT green tests. AI-authored, confidence-marked.
> Trust R/D/I/K + 19H/19I + the human rulings over this. Companion to 20A (the cm-3 charter) and
> 209 (the value-plane breakdown the enrichment roadmap will multiply).

## §0 What landed

New module `spike/crates/syntax/src/sem.rs` (`pub mod sem`), dependency-clean (syntax depends only
on `core`; all three consumer crates — analysis/oracle/plan — already depend on syntax, so `core`
stays dep-free per its weld). Six concept-areas, each public fn doc-commented with its POSIX.1-2024
XCU §2.x clause + a one-line dash-behavior note (the prompt's mandated shape):

- **§1 parameter classification** — `is_name(&str) -> bool` (the POSIX *name* predicate
  `[A-Za-z_][A-Za-z0-9_]*`, XCU §3.235) + `classify_param(&str) -> ParamClass{Name|Positional(u32)|
  Special}` (XCU §2.5). One definition of "is this `$name` a plain var."
- **§2 quoting classes** — `FragClass<'a>{Literal|Var|OpaqueValue|SplitRisk}` + `classify_frag(part,
  quoted) -> Option<FragClass>` (XCU §2.2/§2.6/§2.6.5). The value-plane's "unquoted expansion ⇒ ⊤"
  rule is now the named `SplitRisk` variant; `OpaqueValue` is the quoted-but-⊤ (arity-safe) case.
- **§3 the one modelable expansion** — `parse_prefix_strip(inner) -> Option<PrefixStrip>` (the
  is-modelable predicate the dialect uses to REJECT globby/`##` forms; XCU §2.6.2) +
  `strip_prefix_literal(s, prefix) -> &str` (the shortest-match literal strip).
- **§4 unset-parameter policy** — `UnsetPolicy{ExpandEmpty|Unresolved}` (XCU §2.5.3): the
  test-context-empty vs strict-context-⊤ fork, named.
- **§5 literal-text extraction** — `const_literal_text(parts) -> Option<String>` (the
  "no-variables-at-all" compile-time-constant rule; the `ValueOf::Literal` *narrower* sibling).
- **§6 shell-quoting** — `single_quote(s) -> String` (F-QUOTE, XCU §2.2.2 single-quote-always).

12 unit tests in `sem.rs` (pinning the divergence-sensitive corners: `$0`/`${12}`/specials,
quoted-positional-is-OpaqueValue-not-Var, globby/`##`/var strips ⇒ None, const-text excludes even a
quoted var, the `'\''` escape). +SURE all green.

Call-sites converted + local copies DELETED:
- `analysis::value`: `is_plain_var` (deleted), `collect_frags` re-expressed over `classify_frag`,
  `literal_text` re-expressed over `const_literal_text`.
- `oracle::check::parser`: `is_ident` (deleted; 3 call-sites → `sem::is_name`), `parse_word_lexeme`'s
  `${N#…}` parse → `sem::parse_prefix_strip`, the `${name}`/`$name` ident checks → `sem::is_name`.
- `oracle::check::eval`: `strip_prefix_once` (deleted) → `sem::strip_prefix_literal`; `resolve` +
  `resolve_in_test` unified into `resolve_with(word, UnsetPolicy)` + free `unset_positional(policy)`.
- `plan` render: `sh_single_quote` (deleted) → `sem::single_quote`.
- `syntax::parser`: `is_func_name` → `sem::is_name` (and `is_assignment_name` transitively).

## §1 Divergences found between the old copies — the headline (the fam-B quarry)

The round's thesis (20A §1) was that the N hand-rolled re-implementations are *independent
dash-divergence surfaces*. Result of putting them side-by-side: **no NEW disaster-class divergence
surfaced** (the two priority-1 bugs this round — prefix-env argv visibility, `${N#pat}` glob — were
already found+fixed by the 20B crosscheck before task-S; task-S confirms they were fixed
*consistently*, see dv-3). But the extraction surfaced **three real representational divergences**,
all latent (none currently produces a wrong concrete on the corpus), recorded because each is a
trap the enrichment roadmap (209) re-arms:

- **dv-1 — the value-plane's `is_plain_var` CONFLATED three POSIX classes into one ⊤ bucket; the
  dialect kept them distinct.** +SURE (traced in source). `value.rs::is_plain_var` returned a bare
  `bool`; everything it rejected — positional `$1`, special `$@`, *and* a multi-digit braced
  `${12}` (which the book lexer emits as `Param{name:"12"}`) — fell into one "⇒ ⊤" path via the
  same `false`. The oracle dialect, by contrast, modeled `Positional(n)` as a first-class resolvable
  value. So the SAME concept ("what is `$1`?") had two different ontologies: "not-a-trackable-var,
  lump it" (value-plane) vs "a positional I can resolve against argv" (dialect). Reconciled by
  `classify_param` returning the full 3-way `ParamClass`; `is_name` preserves the boolean caller.
  This is the single sharpest finding: the two engines did not *disagree on an answer*, they
  disagreed on *what questions exist* — exactly the kind of split that makes a later precision
  increment (209 brk-9 partial-⊤, brk-3 deliberate-split) wrong in one plane but not the other.

- **dv-2 — `${12}` (braced multi-digit positional) is handled in the value-plane, DROPPED in the
  dialect.** +SURE. The book lexer's `lex_braced_param` accepts any all-alphanumeric body, so
  `${12}` ⇒ `Param{name:"12"}`, which `classify_param` now calls `Positional(12)`. But the dialect's
  `parse_word_lexeme` only ever made a `${…}` into a `Var` (via `is_name`, which rejects "12") or
  `Unmodeled` — it has NO `Positional` path for the braced form (only bare `$N` reaches
  `Word::Positional`). So `${12}` in a check body is `Unmodeled` ⇒ Top. **I deliberately PRESERVED
  this** rather than silently unify (routing the dialect's `${…}` through `classify_param` would
  have widened `${12}` from Top to a resolved positional — a behavior change, and exactly the
  "silent unification = behavior change" the prompt forbade). Flagged in-code at the dialect
  `${name}` arm. ~SUSPECT harmless forever (no §2 idiom uses `${12}`; the safe direction is Top),
  but it is a genuine asymmetry: the shared `classify_param` is RICHER than the dialect consumes.

- **dv-3 — the `${N#pat}` glob-rejection logic was DUPLICATED verbatim in two places, now
  single-sourced.** +SURE. `oracle::check::parser` (the modelable-form gate: digits-parse ∧
  `!starts_with('#')` ∧ `!contains(['*','?','['])`) and `oracle::check::eval::strip_prefix_once` (the
  strip itself) implemented the two halves of one rule in two functions; the value-plane has NO
  prefix-strip model at all (it ⊤s any `ParamComplex`, which is where `${1#-}` lands book-side — see
  rs-1). The two oracle halves AGREED (both literal-only, both reject `##`/glob — the 20B fix landed
  them consistently), so no divergence-bug; but they were two edit-sites for one dash-fact. Collapsed
  into `parse_prefix_strip` (the gate) + `strip_prefix_literal` (the op). A future edit to "what
  prefix-strip forms are modelable" now has ONE home.

- **dv-4 (minor, recorded) — the name-predicate had FIVE byte-for-byte-identical copies.** +SURE.
  `value.rs::is_plain_var`, `check::parser::is_ident`, `check::parser::split_assignment` (inline via
  is_ident), `syntax::parser::is_func_name`, `syntax::parser::is_assignment_name`, and the book
  lexer's inline `lex_dollar` scan — all encode `[A-Za-z_][A-Za-z0-9_]*`. No divergence (identical),
  but six maintenance sites for one POSIX clause. Four routed to `sem::is_name`; the lexer scan
  stays (rs-2).

## §2 What resisted extraction (semantics entangled with control-flow)

- **rs-1 — the prefix-strip lives in DIFFERENT representations book-side vs check-side; only the
  check-side is modeled.** The dialect parses `${1#-}` from a raw lexeme STRING (its own lexer keeps
  `$`/`#`/`{` literal in the lexeme); the book lexer turns `${1#-}` into a `WordPart::ParamComplex`
  (opaque, never decoded). So `sem::parse_prefix_strip` is consumed only by the dialect — the
  value-plane can't use it without the book lexer first *decoding* ParamComplex (it deliberately
  doesn't; 209 lists `${x%s}`/`${x:-d}` on knowns as unbuilt enrichment). The module HOSTS the
  shared rule, but today has one consumer. Not a strain in the module; a note that the two input
  *representations* (structured `WordPart` vs raw lexeme text) are themselves un-unified — `sem`
  offers both a `WordPart`-classifier (`classify_frag`) and text-level parsers (`parse_prefix_strip`,
  `is_name`) because its two consumers feed it differently. ~SUSPECT that representational split is
  the deeper fam-B residue, below task-S's remit.

- **rs-2 — the book lexer's `$`-scan (`lex_dollar`) is byte-streaming; it resisted `is_name`.** It
  advances `self.pos` over name bytes during tokenization, not operating on a complete `&str`.
  Forcing it through `sem::is_name` would need a `is_name_start(u8)`/`is_name_continue(u8)` split —
  over-engineering for one streaming site. LEFT AS-IS, deliberately: it is *lexical* ("where does
  the name text end"), not *semantic* ("what does `$1` mean"). The meaning is now assigned downstream
  by `classify_param`. Clean separation (the lexer emits `Param{name:"0"}`; `classify_param("0")`
  ⇒ Special), so no belief is duplicated — only the character-class scan, which is tokenizer
  mechanics. Recorded so a later reviewer doesn't mistake the non-extraction for an oversight.

- **rs-3 — `UnsetPolicy` is a NAMED rule but its APPLICATION stays at the call-site.** The unset
  fork is genuinely entangled with the evaluator's `Result<String, TopReason>` type and its
  positional-lookup state (`self.positional(n)`), so `sem` exposes the *policy enum* (the named
  concept, single-sourced) but the dialect evaluator applies it (`resolve_with` + `unset_positional`).
  This is the honest boundary: the dependency-free kernel can't hold the evaluator's argv state.
  The value-plane has NO test-context (it never evaluates `[ ]` tests — that's the dialect's job), so
  `UnsetPolicy` has one consumer too. The concept is single-sourced; the mechanism is not (and
  shouldn't be — different Result types).

- **rs-4 — `const_literal_text` vs the `ValueOf::Literal` "fully-expanded" contract are TWO
  different guarantees; only the narrower one extracted.** `const_literal_text` = "no variables at
  all" (compile-time constant, for recognizing `unset name`'s shape). The WIDER `ValueOf::Literal`
  contract — "the single argument dash would pass, given dataflow state" — needs the live value
  environment (a quoted `$x` that the worklist resolved to `nginx`), which the dep-free kernel must
  not hold. So `sem` owns the narrow rule; the wide contract stays documented at its consumer
  (`analysis::value`, built on `classify_frag` + the worklist state). Stated once each; not unified
  (they genuinely differ — a quoted `$x` is wide-literal but not const-literal).

## §3 Coverage map — which POSIX clauses are now single-sourced vs still scattered

SINGLE-SOURCED in `sem` (the cm-3 win — O(1) edit-site each):
- name predicate `[A-Za-z_][A-Za-z0-9_]*` (XCU §3.235) — was 5 copies → 1 (+ lexer scan, rs-2).
- parameter classification name/positional/special (XCU §2.5) — was 2 ontologies (dv-1) → 1.
- quoting-class / split-risk rule (XCU §2.2, §2.6.5) — was value-plane-only `collect_frags` → 1
  named `classify_frag` (the value-plane consumes it; +SURE no other site classifies WordParts).
- `${N#literal}` modelable-gate + strip (XCU §2.6.2) — was 2 oracle halves (dv-3) → 1 pair.
- unset-parameter context fork (XCU §2.5.3) — concept named once (rs-3); applied at 1 site.
- single-quote / F-QUOTE (XCU §2.2.2) — was plan-only `sh_single_quote` → 1 (+SURE one consumer).

STILL SCATTERED (named, NOT moved — the honest residual; these own their own dash-beliefs):
- **render word-assembly** — `plan::render_sh`/`render_apply` reconstruct sh from spans/leaves
  (line-granular comment-out). This is its own dash-correctness surface (the standing T14 xfail IS a
  render-assembly bug — a case-arm one-liner mangled). `sem` owns *quoting* (single_quote) but NOT
  *assembly*; the render emits verbatim spans + the F-QUOTE'd operand, so the assembly belief
  (what's a valid POSIX rewrite) lives in `plan`. ~SUSPECT this is the largest remaining fam-B
  surface (the round's only standing semantics xfail is here). Out of task-S remit (the prompt named
  the quoting call-site, not the assembler).
- **errexit edge semantics** — `analysis::cfg::materialise_errexit_edges` (the `||true`/negated-
  pipeline/`if`-cond pruning, note 166/T9) is a dash-belief about `set -e` propagation. Statement-
  level, entangled with CFG construction; NOT word-level. Correctly outside `sem`.
- **command-prefix argv-vs-environment ordering** (XCU §2.9.1) — `value::site_argv`'s "argv expands
  before prefix assignments take effect" (the priority-1 prefix-env fix). This is a *statement*-level
  expansion-ordering belief, not a word-level one; it stays in the value-plane's site logic.
- **`case`/`while`/`shift` argparse control-flow** — `oracle::check::eval`'s loop/arm execution is
  control-flow semantics (XCU §2.9.4 / §2.6.2 case), distinct from word resolution. Stays in eval.
- **the two lexers' tokenization** (book `lexer.rs`, dialect `check/lexer.rs`) — quote-grouping,
  metachar recognition, heredoc/subst balancing. Lexical, per-representation; rs-1/rs-2 note why
  they stay split. `sem` is post-lexical.

Net: every WORD-LEVEL belief the prompt enumerated is now single-sourced. The residual scatter is
all statement-/command-/lexical-level, correctly outside a word-semantics module — but the
render-assembler (the one with a live xfail) is the place the next cm-3-shaped extraction should aim
(--WONDER whether a `sem`-hosted "is this leaf-comment a valid POSIX rewrite" predicate could pull
the T14 belief out of `render_apply`; not attempted — it is assembly, not word-semantics, and would
widen task-S's scope).

## §4 Behavior-preservation evidence (the mandate)

- `cargo test --workspace`: 240 pass, 1 ignore (the pre-existing HOLE#1 subst-in-redir spec, 20C §8
  — untouched). +12 new `sem` unit tests vs the pre-task-S count; no prior test removed or weakened
  (the converted call-sites' existing tests — value.rs's 35, oracle's 39 incl. the glob/test-context
  cases, plan's F-QUOTE case — all pass UNCHANGED, which is the real preservation proof: they pin the
  exact behaviors the old local copies produced).
- `sh e2e/run.sh`: 44/44 (43 ok + the standing T14 render xfail). `git status spike/e2e`: EMPTY —
  **zero golden diffs** (+SURE; the load-bearing check — the probe/apply artifacts are byte-identical,
  so no consumer's observable output moved).
- `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean (NO new
  `#[expect]`s added; the seeded crate-root expects in syntax/plan still ratchet — `sem.rs` is fresh
  code under the full lint bar and needed none). `mise x -- typos spike` clean.

## §5 What D2/D3 inherit

- **`sem` is the home for the next word-semantics dash-fact.** 209's enrichment roadmap (brk-3
  deliberate word-splitting `cmd $PKGS`; `${x%s}`/`${x:-d}` operator-forms on knowns; brk-9 partial-⊤
  argv) ALL add word-level beliefs. Each lands in `sem` as a new clause-documented fn, consumed by
  whichever plane needs it — NOT a sixth private copy. The `FragClass` enum is the extension point
  for brk-3 (a literal-valued unquoted `$PKGS` could become a new `SplitInto(n)` class instead of
  today's `SplitRisk`-⇒-⊤); `classify_param`/`ParamClass` already carries the positional index brk-9
  needs.
- **dv-2's asymmetry will bite if a check ever uses `${12}`.** The dialect drops it to Top today
  (safe). If a future oracle idiom needs braced multi-digit positionals, the fix is a one-line
  `${…}`-arm change in `check::parser` routing through `classify_param` — but that is a *deliberate*
  behavior widening (a new resolvable form), to be made consciously, not a silent unification.
- **cm-2 (the argv-echo / check-eval differential gate, 20A §2 / 20B §3) now has a single validation
  target.** The prompt's cm-3 rationale: "validated once by cm-2, reused everywhere." With the word
  semantics single-sourced, cm-2 validates `sem` against dash ONCE and every consumer inherits the
  guarantee — instead of cm-2 having to separately exercise each plane's private copy. D3's differential
  gate should aim its assertions at `sem`'s functions (the resolutions on the executed path trace
  through `classify_frag`/`classify_param`/`parse_prefix_strip`), making a dash-divergence a `sem`
  unit-test failure rather than an end-to-end mystery.
- **The render-assembler (T14) is the next cm-3 candidate, NOT a word-semantics gap.** §3 names it as
  the largest remaining fam-B surface with a live xfail. Left for the render-fidelity work (seam-prov
  leaf-exact); flagged so it is not mistaken for something `sem` should already cover.
