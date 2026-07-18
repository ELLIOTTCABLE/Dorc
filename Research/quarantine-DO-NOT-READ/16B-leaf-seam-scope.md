# 16B — leaf-seam scope: `$()` bodies are effect-bearing non-leaves (find-cli-1)

> **Status (2026-06-05): spike, round-16 stage.** Resolves the headline cli-capstone
> finding (note 169 find-cli-1): command-substitution inner commands were leaking
> into the plan as spurious run/skip leaves (rendered as garbage `#!/bin/s`). This
> stage makes the dn-3 leaf-seam **scope** precise. Standalone note (append-only;
> the round counter goes 169 → 16A → 16B, staying in round 16). Confidence-marked.
> HEAD `28446ae`.

## 0. The ruling: what is a plan/apply leaf
A **leaf** (dn-3 — an individually-wrappable unit of executable work with a stable
`LeafId → AstId`) is a command in **execution position**: top-level, an `if`/`case`/
`&&`/`||` branch body, a subshell `( )` body, or a group `{ }` body. A command
inside a **command-substitution `$( … )` / backtick body is NOT a leaf** — it runs
during *word expansion* of some enclosing command, not as a standalone step. The
enclosing command (e.g. `echo "… $(hostname) …"`) is the leaf, and its verbatim
text legitimately *contains* the `$(…)`.

Crucially, this splits two scopes that the engine had conflated: **effect-analysis
scope** (everything, incl. `$()` internals — a `$(rm -rf x)` must still
poison/establish) vs **leaf scope** (only execution-position commands). They are
different, and now tracked separately.

## 1. What changed
- `cfg`: a per-node `expansion_internal` bitset, set for the whole body range of
  each `$( … )` in `lower_word_substs` (covers nested substitutions). Subshell
  `lower_scoped` and group lowering do NOT set it — their bodies are real leaves.
  Exposed as `Cfg::is_expansion_internal(id)`.
- `effect::classify`: its **output** loop skips `expansion_internal` commands —
  but they remain in the reaching-defs `effects` computation, so their mutations
  still flow (the effect-vs-leaf split, §0). One added `||` clause; the dataflow
  is untouched.
- This also dissolves the compounding span bug (note 169): the offending nodes had
  substring-relative spans (`parser::parse_subst_body` re-lexes from offset 0), but
  since they are no longer rendered as leaves, the garbage `#!/bin/s` output is
  gone — no parser-span change needed.
- Regression tests: a cfg test (`$(uname)` body is expansion-internal; `( uname )`
  body is a leaf) and a plan test (`echo $(uname)` + an install ⇒ exactly two
  Steps, no third garbage leaf). Whole workspace: 90 tests green, clippy clean.
- Verified on the real fixture: the plan no longer emits the two spurious
  `#!/bin/s` lines; it opens with the genuine commands.

## 2. Leaf-model items still open (deferred, not regressions)
- **find-cli-3 (heredoc leaf text):** a `cat > f <<'EOF' … EOF` leaf is correctly
  *identified*, but its rendered text stops at the redirection operator (the
  `Simple` span excludes the here-document body), so the run-line is incomplete.
  This is leaf-text *completeness*, not spurious leaves — a leaf's text isn't always
  one `[lo,hi)` slice. Fix when the plan render needs to be runnable (give the leaf
  a span/text covering its heredoc body).
- **fs-3 (double-quoted literal entity):** `apt-get install "nginx"` → the entity
  `"nginx"` is not matched as a fixed token (`word_literal` accepts only
  `[Literal]|[SingleQuoted]`), so the command is ⊤/MustRun. A double-quoted
  expansion-free string IS a fixed token; accepting it is sound and a real
  test-enabler (double-quoted args are common). Cross-cuts ~4 `word_literal` sites —
  do consistently, early in the next word-model touch.
- **pipeline-as-leaf (new, open question):** a pipeline `a | b` is two `Command`
  nodes ⇒ today two leaves, but it is arguably one run/skip unit. Not exercised by
  current fixtures (no top-level multi-stage pipeline). Decide when pipelines carry
  an effect (e.g. `… | tee /etc/x`).
- **plan flattening (note 169):** leaves are still emitted source-ordered without
  reproducing their `if`/`case` guards — the plan shows mutator dispositions, not a
  runnable control-flow rewrite. The leaf-seam / wo-1 tension; a faithful in-place
  rewrite is later.

## 3. Why this was the right next step
find-cli-1 is the prerequisite for **apply** (note 16A §4): apply *executes*
leaves, so "what is a runnable leaf" had to be settled before an executor could run
them — a `$()`-internal `hostname` is not something you run/skip. With the leaf
scope precise, the arc (note 16A §5) is unblocked: **apply executor (single-host)**
→ multi-host fan-out → unreliable-oracle DST. The deferred items above (heredoc
text, double-quote, pipeline-leaf) are word/leaf-model refinements that can ride
the test-suite expansion rather than block apply.

**NOTES INDEX:** …168 round-16 build summary · 169 cli capstone findings · 16A
apply + multi-host direction · 16B (this — leaf-seam scope / find-cli-1).
