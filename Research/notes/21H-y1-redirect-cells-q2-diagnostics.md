# 21H — y-1 redirect-write cells + q-2 `$()` ⊤-diagnostics floor (build note)

> Round-21 build note, append-only. Two pre-spelled slices built in the MAIN worktree
> (`8d87e15`-based): **y-1** (redirect-write poison-correctness, from `21F` imp-1 via
> `19H`/`19I`) and **q-2** (the `$()` ⊤-diagnostics floor, from `219` §q-2 shaped by `21G` §3).
> AI-authored, confidence-marked. Trust README/DESIGN + the human rulings + `19H`/`19I` over
> this. The optional s-2 was assessed and SKIPPED (not ≤20 lines cleanly; see §s-2). No git
> mutation performed — edits left uncommitted for the orchestrator.

---

## 0. Executive summary

- **q-2 (the floor) — built, fully green.** A diagnostic CATALOG now lives in `core::diag`
  (the `ch-catalog`/Pottier-embryo, `21G` §3 rq-1/rq-2/rq-3). Three silent `$()`→⊤
  degradation sites (`219` q-1.f silent-1/2/3) now emit honest **Note**-severity disclosures:
  `dq-cmdsub-operand-top` (a ⊤ operand/word forces Opaque, in `command_effect`),
  `dq-cmdsub-inner-nonleaf` (an effect-bearing `$()`-inner command runs un-elidably, at
  classify's leaf-drop), `dq-site-unresolvable` (cli-edge stderr readout of
  `ProbePlan::unresolvable`). All Note-severity ⇒ gate-3 (which keys only on `error[`) is
  untouched. +SURE.
- **y-1 (redirect-effects) — built, fully green, imp-1 hole closed.** A WRITE-shaped redirect
  (`>`/`>>`) to a resolved non-`/dev/null` target now GENS a per-path `file:<path>#written`
  cell (previously invisibly `Pure`). It is a WRITER ⇒ st-3's coarse invalidation makes a
  downstream Query non-pristine ⇒ `valid: false`. This closes the `21F` imp-1 stale-guard hole:
  a `printf >> f` before a `grep`-guard of `f` no longer mints a stale-guard elision. A ⊤/dynamic
  target joins ⊤ (Opaque-poison) + a `dq-redir-target-top` disclosure. `/dev/null` and fd-dups
  (`2>&1`) stay exempt. +SURE (unit + e2e + manual-traced contrast).
- **Churn: NONE to existing goldens** (all 90 prior e2e cases byte-identical; +3 new y-1 cases =
  93). +SURE (git status verified — §3).

## 1. What built — file-by-file

### q-2 (catalog + 3 emit sites)
- `core/src/lib.rs`: added `Diagnostic::note(...)` (Note severity had no constructor); `pub mod diag;`.
- `core/src/diag.rs` (NEW): the catalog. Per code: a `pub const DiagCode`, a `template()` match
  arm (the phrasing — rq-1, decoupled from emit), a structured-param constructor
  (`cmdsub_operand_top(span, position)` / `cmdsub_inner_nonleaf(span, inner)` /
  `site_unresolvable(span, leaf, source)` / `redir_target_top(span)`), and a `CATALOG` registry
  entry. Completeness test `every_registered_code_has_a_nonempty_template` (rq-2) + a
  Note-severity-invariant test (the gate-3 floor) + a constructor fill-test. `fill()` does
  `{key}`-substitution (no macros — `inv-no-unsafe`).
- `analysis/src/effect.rs`: `command_effect` gained a `site: Option<Span>` param and emits
  `dq-cmdsub-operand-top` at its two ⊤-operand Opaque-returns (the command-word ⊤ at
  word0, and the ⊤-arg in the arg loop, naming `operand N`). Classify's leaf-drop emits
  `dq-cmdsub-inner-nonleaf` for an EFFECT-BEARING `$()`-internal command (gated on non-Pure —
  a pure inner command discloses nothing). `render_argv` helper renders an argv to display text.
- `cli/src/main.rs`: `unresolvable_diagnostics(probe, plan, ast, book_src)` maps each
  `probe.unresolvable` LeafId → the plan Step's AstId → source-span text → a `dq-site-unresolvable`
  Note, mirroring `report()`/`emit_debug_argv` plumbing.

### y-1 (redirect-write cells)
- `analysis/src/value.rs`: new `redir_target: BTreeMap<CfgNodeId, ValueOf>` field on `ValueFlow`
  + `redir_target(node)` accessor + a `redir_pass` (a separate post-solve pass off the converged
  solution, the Members/inline precedent). It resolves each WRITE-shaped (`Write`/`Append`)
  non-`/dev/null` `Redir` node's target word against the node's INCOMING env state via
  `resolve_recipe` (single value, not field-split — a redirect target is one field); an
  expansion-hazard target (glob/tilde) ⇒ ⊤.
- `analysis/src/effect.rs`: extracted the per-node effect computation to `node_effects(...)` (to
  keep `classify` under the line cap), and added a `CfgNodeKind::Redir` arm: a resolved literal
  target ⇒ `Establishes(file_write_cell(path))`; a ⊤ target ⇒ `Opaque` + `dq-redir-target-top`;
  a non-write redirect (absent from `redir_target`) ⇒ `Pure`. `file_write_cell` builds
  `FactKey { kind: file, entity: Operand(path), selector: written }`.

### e2e cases (3 new, hand-derived goldens, mocks-free)
- `y1-redirect-write-invalidates-query` — `set -e; printf 'x' >> nginx.conf; grep -q x nginx.conf
  || apt-get install -y nginx` + a `confline` grep-query oracle. Probe says the guard HOLDS; the
  apply renders VERBATIM (the install is NOT folded — the imp-1 regression pin).
- `y1-devnull-exempt` — `: > /dev/null` then a converged `apt-get install -y nginx`: the devnull
  redirect gens nothing ⇒ the downstream install ELIDES (`true`).
- `y1-top-target-poisons` — `: > "$dyn"` then `apt-get install -y nginx`: the ⊤ target poisons ⇒
  the install runs verbatim + `dq-redir-target-top` fires.

## 2. The catalog shape (`21G` §3 rq-1/rq-2/rq-3)

ONE location: `core::diag` (flagged choice — `core` is the only crate `cli` AND `analysis` both
depend on, and it owns `DiagCode`; a catalog in `analysis` couldn't host the cli-emitted
`dq-site-unresolvable`). Shape per code = `{ const DiagCode } + { template() arm } + { constructor
taking structured params } + { CATALOG entry }`. rq-1: emit sites call the constructor and never
write prose. rq-2: `every_registered_code_has_a_nonempty_template` over `CATALOG`. rq-3: the
constructors are the only path to these codes on the new lines. Deliberately NOT retrofitting the
existing scattered codes (`effect-kind-disagreement`, `cfg-top-node`, …) — `21G` §3 limits folding
to "trivial" and "no analyzer is built". ~SUSPECT this is the right scope: retrofitting all would
be a large mechanical churn for no behavior change, and the future Pottier path-enumeration gate
(`21G` §2 layer-1) is what makes the full retrofit load-bearing.

## 3. Churn table (expect: none)

| artifact class                     | change |
|------------------------------------|--------|
| existing `expected.out` goldens    | **NONE** (all 90 byte-identical; git verified) |
| existing `expected.ran`            | NONE |
| existing unit tests                | NONE modified (counts grew: analysis 115→127, core 5→7, cli 9→10 — all additions) |
| new e2e cases                      | +3 (`y1-*`) |
| new source                         | `core/src/diag.rs` |
| modified source                    | `core/lib.rs`, `analysis/{effect,value}.rs`, `cli/main.rs` |

Gate-3 interaction (the load-bearing churn-risk q-2 was warned about): gate-3's `scan_diagnostics`
greps `^[a-z]+: error\[` and returns early if no error lines — it NEVER inspects Notes/Warnings
(`run.sh:382-383`; the `expected-diagnostics` file only suppresses *error* lines). So every q-2/y-1
diagnostic, being Note-severity, is invisible to gate-3. +SURE no existing case trips. This is why
no `expected-diagnostics` file was needed anywhere.

## 4. The imp-1 regression pin — traced (acceptance requirement)

`y1-redirect-write-invalidates-query` with probe saying the guard HOLDS (`site 2 effect=holds rc=0`):
the apply renders `grep -q x nginx.conf || apt-get install -y nginx` VERBATIM (no fold). `--debug-argv`:
```
argv 0 run set -e
argv 1 run printf x
argv 2 run grep -q x nginx.conf
argv 3 run apt-get install -y nginx      ← run, NOT replace: the stale mint is refused
```
CONTRAST (same book WITHOUT the `printf >> nginx.conf` write) DOES fold:
```
set -e
true || :   # dorc: elided [grep -q x nginx.conf; apt-get install -y nginx] (already converged / dead branch)
```
So the write-redirect is precisely the load-bearing invalidator: present ⇒ install runs LIVE;
absent ⇒ install folds dead. The mechanism: the `Redir` node gens `file:nginx.conf#written` ⇒
the grep guard's reaching-defs in-state is non-pristine ⇒ `QueryResolvable { valid: false }` ⇒
the cli withholds the guard rc (status ⊤) ⇒ the `||` can't resolve ⇒ install stays live. +SURE.

## 5. Kind-vocabulary choice (flagged for review)

The file cell is `FactKey { kind: KindId("file"), entity: EntityRef::Operand(OpaqueToken(path)),
selector: SelectorId("written") }`. Rationale: `file` is a Tier-A blessed well-known kind name
(`core` doc: "`file`, `tool`, `freshness`"), so it follows the existing vocabulary rather than
inventing one. `written` selector: append and truncate are BOTH write-shaped this round ⇒ one cell
(no read-back/content discrimination, per the charter). The path is the entity operand
(referent-agnostic — interned, never decoded beyond the syntactic `/dev/null` check at resolution).
~SUSPECT residuals worth a human eye:
- **`written` vs a richer selector**: a future oracle that wants to QUERY a file's content (the
  `confline`/grep idiom in the regression case is exactly this shape) keys a DIFFERENT kind
  (`confline`), not `file#written`. So `file#written` and `confline:<f>#present` are distinct cells
  on the same path — the write does NOT discharge the read (correct: writing a file doesn't tell
  you it contains the pattern). But it means a write-then-read of the SAME file via these two cells
  never coordinates by cell-identity — only by the pristine-prefix invalidation. That is the
  *intended* y-1 behavior (gen-and-poison, nothing licenses), but it is worth confirming the human
  wants `file` writes and content-queries to stay separate kinds rather than selectors of one kind.
- **no probe/elision on `file` cells**: a `file` cell has no oracle/probe, so it can never become a
  probe-resolvable `EstablishAmbient` that licenses elision — AND a `Redir` node is never a plan
  leaf anyway (classify only emits `Command` leaves). So "gen and poison, nothing licenses" holds
  doubly. +SURE.

## 6. Adversarial hunt-list (where this might break — for a crosscheck)

- **hunt-1 redirect-target resolution edges (TESTED, behave conservatively):**
  - tilde `>> ~/x` ⇒ ⊤ + `dq-redir-target-top` (the `word_expansion_hazard` catches `~`; sound — we
    can't reproduce `$HOME`). ✓
  - var-with-default `>> ${f:-/tmp/x}` ⇒ ⊤ + disclosure (`ParamComplex` is opaque). ✓
  - relative literal `>> rel.log` ⇒ resolves ⇒ gens `file:rel.log#written` (invalidates a downstream
    query). ✓ (no disclosure — it's a clean literal)
  - absolute literal `>> /abs/path` ⇒ resolves ⇒ gens `file:/abs/path#written`. ✓
  - var-resolved `logfile=app.log; : > "$logfile"` ⇒ resolves through the value plane to `app.log`
    ⇒ gens `file:app.log#written` ⇒ invalidates the downstream query. ✓ NOW PINNED
    (`var_resolved_redirect_target_invalidates_query`, the value-plane integration the charter
    emphasizes — `resolve_recipe` against the node's incoming state, shared with `argv_values`).
- **hunt-2 `<>` read-write redirect:** `RedirOp` has NO `<>` variant; `: <> /tmp/x` produces a loud
  `parse: error[syntax-unsupported]` (a ⊤-reject), so a `<>` write can NEVER slip through as
  invisibly Pure. The charter listed `<>` as write-shaped; it is covered by the parser's existing
  ⊤-reject, not by y-1. +SURE (tested). A crosscheck should confirm this is acceptable (we
  ⊤-reject-the-whole-statement rather than model `<>` as a write — the safe over-refusing direction).
- **hunt-3 gate-2 / exec-case interaction:** the redirect-safety scan (gate-2, `run.sh:97`) REFUSES
  absolute/dynamic/escaping redirect targets — but ONLY for cases with a `mocks/` dir. My 3 y-1
  cases are mocks-FREE (dash-n + golden), so gate-2 never runs on them, so `>> "$dyn"` and
  `/dev/null` are fine. **If a future y-1 exec case wants a literal-path write, the target must be a
  bare RELATIVE path** (lands in the sandbox); an absolute or dynamic target would be gate-2-refused.
  Flagged so a later exec-validating crosscheck doesn't author an absolute-path write and get a
  confusing gate-2 failure.
- **hunt-4 the regression oracle's kind-disagreement:** my earlier combined-file strawman emitted a
  spurious `effect-kind-disagreement` (the apt and confline effects collided when `oracle_kind`
  bled across one file). The shipped case uses SEPARATE oracle files (`package.oracle.sh` +
  `confline.oracle.sh`), which is clean. A crosscheck authoring more multi-kind oracles should put
  each kind in its own file (the `oracle_kind=` is file-level current-kind state).
- **hunt-5 q-2 disclosure noise:** `dq-cmdsub-operand-top` fires for EVERY ⊤ operand on EVERY
  command (e.g. it fires on existing `exec-opaque-var-runs`, headline cases). This is the intended
  find-3 floor (disclose every silent ⊤), and it's Note-only so it's harmless to gates — but on a
  large real book it could be verbose. ~SUSPECT acceptable for the spike (the human's spike-4 layer
  wants MORE disclosure, not less); flagged in case the volume is judged excessive.
- **hunt-6 span-provenance gap:** `dq-cmdsub-operand-top` and `dq-cmdsub-inner-nonleaf` pass
  `span: None` (classify holds the CFG, not the AST — a span isn't cheaply reachable). The message
  names the ⊤ POSITION/inner-text, which is what `report()` surfaces (it renders no spans this
  round), so this is presently invisible. If a future round renders spans, these two want the AST
  threaded into classify (or the per-node AstId carried). `dq-site-unresolvable` (cli-edge) DOES
  carry a span (it has the AST). -GUESS low-priority while `report()` ignores spans.

## 7. s-2 (optional accessor) — assessed, SKIPPED

s-2 asked for "a public accessor on classify's output exposing per-site CommandEffect/⊤-reason
(dashboard seam-1) — accessor only," gated on "genuinely ≤20 lines." `classify` computes the
per-node `effects: Vec<Vec<CommandEffect>>` internally and DISCARDS it (returns only `SkipClass`).
Exposing per-site CommandEffect cleanly requires either threading `effects` out of `classify`
(changing its return type — ripples through `cli`/`plan` callers) or a second public function that
re-runs `node_effects` over the graph (duplicating the build, and re-resolving checks costs an
`&mut Interner`). Neither is ≤20 lines without a hack. **SKIPPED** per the explicit gate. ~SUSPECT
the right shape when it IS built: change `classify` to return the `effects` alongside `SkipClass`
(or a small `SiteEffects` view) as a deliberate signature change with the dashboard as its first
consumer — not a bolted accessor. Flagged for the dashboard slice.

## 8. Exclusion-check (the four-by-two, per AGENTS.md)

- **reverse direction (backward/Must):** y-1's file cell is a forward-may gen (`Establishes`); it
  never participates in a Must/backward analysis (none instantiated). No reverse hazard this round.
- **other phase (probe vs apply):** a `file` cell has no probe ⇒ it never ships a check ⇒ the probe
  phase is unaffected (the `Redir` node isn't a leaf, isn't in `compile_probe`'s site set). Apply
  phase: it poisons/invalidates only. Symmetric and safe. +SURE.
- **other user (admin vs oracle-author):** the admin writing `printf >> f` in a book gets the
  poison-correctness for free (no annotation needed — the redirect IS the signal, "spelled in sh").
  The oracle author is unaffected (no new authoring surface). Aligned with "metadata is spelled in sh".
- **other reliability (unreliable oracle):** y-1 doesn't depend on any oracle (the file cell is
  engine-synthesized from the redirect AST), so oracle-unreliability can't weaken it. It is strictly
  MORE conservative than before (a write that was Pure is now a WRITER) ⇒ the kFAIL direction is
  preserved (over-poison ⇒ over-run ⇒ sound).

## 9. Confidence summary

- +SURE: q-2's three Notes fire correctly and are gate-3-invisible (Note-severity); the catalog
  rq-1/rq-2/rq-3 shape; y-1's file-cell gen + the imp-1 regression closure (unit + e2e + traced
  contrast); zero golden churn; the `<>` parser-⊤-reject; the devnull/fd-dup exemptions.
- ~SUSPECT: the var-resolved redirect target (`>> "$logfile"`) works via shared constant-prop
  machinery but lacks its own e2e/unit pin (hunt-1); the kind-vocabulary `file#written` vs a richer
  shape is the one design choice a human should ratify (§5).
- -GUESS: the span-provenance gap (hunt-6) is low-priority while `report()` ignores spans.
- Flagged-not-resolved: s-2 skipped as not-≤20-lines (§7); q-2 disclosure volume on large books
  (hunt-5).
