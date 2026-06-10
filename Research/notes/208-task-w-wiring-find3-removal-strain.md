# 208 — task-W landed: find-3 removal, classify on the value-plane + check-resolution

> Round-20 spike note, append-only. Records the keystone wiring (task-W, 205 §6): the
> `find-3` engine-side argparse stand-in in `analysis::effect` is DELETED and replaced by
> the real mechanism — book-side value-flow (`analysis::value`, task-A) threaded through
> the oracle's own `check()` (`oracle::check::evaluate`, task-C) to its inline
> kind-annotation. "Identity is declared, never inferred" is true in code now. AI-authored,
> confidence-marked. Trust R/D/I/K + 19H/19I + the human rulings over this.

## §0 What landed

- `analysis::effect`: `command_effect`/`classify` re-keyed onto `ValueFlow` argv +
  per-file `CheckSet`s; `resolve_entity` (find-3 flag-strip) and `verb = word-1` deleted;
  every `find-3 STAND-IN` marker gone (`rg 'find-3' spike/crates` returns only the
  UNRELATED errexit-finding "note 166 find-3" in `cfg.rs` — see strain-W6).
- `oracle::check`: `Resolved.entity` became `ResolvedEntity::{Operand(String), Singleton}`;
  a **value-less annotation** (`index : pkgindex` — no `= value`) is the explicit nullary
  spelling; `eval_test` now treats a past-end positional as the empty string (sh-faithful)
  so flag-strip `while`s terminate and a `[ "$2" = "" ]` single-operand guard works.
  `map_provider_name` exported as the single home of the hyphen↔underscore convention.
- `oracle` (KindIndex): `effect_of` returns a `Vec<EffectCell>` (multi-cell verbs legal);
  `add_effect` does duplicate-detection (same `(provider, verb, selector)` ⇒ loud
  `oracle-duplicate-effect`, first-wins); `empty_verb()` is the shared ε-verb (`""`).
- corpus: every `*.oracle.sh` + `fixtures/package.oracle.sh` gained a `<provider>__check()`
  (ADDITIVE — the `oracle_probe_*` bodies stay; the probe artifact still reads them this
  round). `useradd`/`command -v` effect rows moved to the `''` ε-verb.
- gates all green: `fmt`, `clippy --workspace --all-targets -D warnings` (no new expects),
  `cargo test --workspace` (145 lib/integration across crates), `sh e2e/run.sh` 43/43
  (standing render xfail intact), `typos spike`.
  <!-- /* corrections 2026-06-10 (round-20 harness-crosscheck acc-1/acc-2): the commit this
  note describes (1f66bbd) shipped with 44 case dirs (the orchestrator's review added
  exec-opaque-var-runs and performed the §4 rename in the same commit — so §4's "flag for the
  human / if the human approves" misdescribes its own commit: the rename HAPPENED, to
  `exec-resolved-var-elides`, with the ⊤-floor re-grounded in `exec-opaque-var-runs`). */ -->

## §1 The classify signature change + call-sites (the contract task-D inherits)

`classify` is now:
```rust
pub fn classify(cfg, value: &ValueFlow, idx: &KindIndex, checks: &[CheckSet], interner)
    -> Carrier<Vec<(CfgNodeId, SkipClass)>>
```
Changes from the old `(cfg, ast, idx, interner) -> Vec<…>`: (a) `ast` dropped (the argv comes
from `value`, which the caller built from the ast); (b) `value: &ValueFlow` added; (c)
`checks: &[CheckSet]` added; (d) returns `Carrier` (kind-disagreement warnings, 204 §6).
`command_effect` is now `(idx, checks, argv: &[ValueOf], interner, diags) -> Vec<CommandEffect>`.

Call-sites updated (all build value-flow first, lift per-file CheckSets, thread both):
- `cli/src/main.rs` `run()` — lifts a `CheckSet` per oracle source (shared interner),
  `value::analyze`, `classify(...).value`, `report("check"/"classify", …)`.
- `plan/src/lib.rs` `#[cfg(test)] plan_for` + `consumption_fact_total_…` (a local
  `CORPUS_CHECK_SRC`).
- `plan/tests/observable_matrix.rs` — a `classify_value` helper + `CORPUS_CHECK_SRC` (3 sites).
- `hostsim/src/lib.rs` `#[cfg(test)]` — a `classify_value` helper + `CORPUS_CHECK_SRC` (3 sites).
- `analysis::effect` tests — a `classify_src` helper + `CORPUS_CHECK_SRC`.

## §2 The shared provider-mapping helper's home

`dorc_oracle::check::map_provider_name(&str) -> String` (the `_`→`-` map). It was the
parser's private `map_provider_name`; now `pub` at the `check` module root, re-used by
`analysis::effect` (the book command word routes through it before interning the provider
symbol). The book word is already hyphenated, so the map is a no-op there — but routing
through the ONE helper is what welds the `CheckSet` key, `KindIndex`'s `ProviderId`, and the
book's command-word interning to one vocabulary (204 §6 seam #2). Verified, not assumed: the
classify tests resolve `apt-get`/`ufw`/`systemctl`/`command`/`useradd` checks through this
path, all sharing the caller's interner.

## §3 The ε-verb spelling chosen (flagged)

**ε-verb = `''` (an empty single-quoted word), interned as the empty string `""`.** Picked
over `-` because `-` collides with real flag-shaped verbs (`dpkg -i`'s verb is `-i`), whereas
an empty string can never be a real argv verb token. The `oracle_effect` grammar spells a
verbless provider's row `oracle_effect useradd '' establish present`; `empty_verb(interner)`
is the single function both sides call (the lifter interns the `''` arg to `""`; the wiring
maps a check's `verb: None` to `empty_verb`). +SURE this is right for the corpus; ~SUSPECT a
real tool with a genuinely empty arg is degenerate enough never to matter.

## §4 Every golden touched + why (GOLDEN DISCIPLINE)

Only TWO e2e goldens changed; both are sanctioned (a find-3 artifact dying, and the
value-plane landing). The other 41 are byte-identical.

- **`andor-rc-undeclared-runs`** — `expected.out` only (the probe artifact); `expected.ran`
  UNCHANGED (`mkdir` + `useradd` still both run). The cell moved `user#present` (bare-kind
  Singleton) → `user:deploy#present` (Operand), so the probe renders `user__check 'deploy'`
  and the results key is `user:deploy#present`. Reason: the **baked-verb wart dies** (19I §2 /
  the prompt's keeper-oracle instruction). find-3 mis-read `useradd deploy` as verb=deploy ⇒
  Singleton; the no-verb check correctly binds the FIRST OPERAND `deploy` as the entity.
  `probe-results.txt` re-keyed to `user:deploy#present` to match. The apply outcome is
  unaffected (the install runs because its branch-consumed status carries an undeclared ⊤ rc,
  independent of convergence).
- **`exec-opaque-var`** — `expected.out` + `expected.ran` (ran `apt-get install -y nginx` →
  EMPTY/elided). Book is `PKG=nginx; apt-get install -y "$PKG"`. Reason: the **value-plane
  landing** — value-flow resolves `"$PKG"` → `nginx`, so the site is now fully concrete and
  the apt check resolves entity=nginx; converged ⇒ the install elides. find-3 saw `"$PKG"` as
  a non-literal operand ⇒ Opaque ⇒ ran. This is the headline value-flow win, but the case is
  now MISNAMED (the var is no longer opaque to the engine). **Flag for the human**: rename the
  case dir (e.g. `exec-flowed-var-elides`) and add a *new* `exec-opaque-var` testing a
  genuinely-⊤ operand (an UNASSIGNED `"$X"` or a `$(...)`), which still ⊤s ⇒ runs (I added a
  unit test `opaque_var_operand_is_top_when_unresolved_but_resolves_when_flowed` pinning both
  poles, but the e2e case name is now stale).

## §5 Every Diagnostic added

- `oracle-duplicate-effect` (warning→error: `Diagnostic::error`) — `KindIndex::add_effect`
  returns `Option<EffectConflict>`; `lift`'s `bind` emits this on a duplicate same-cell
  `oracle_effect`, first-wins (205 §3 us-effectmap). Unit-tested (`duplicate_effect_…`).
- `effect-kind-disagreement` (warning) — `analysis::effect::cell_effect` emits this when a
  cell's effect-map kind ≠ the check's annotation kind; the **annotation wins** (the cell is
  re-keyed under the annotation kind). In the corpus this NEVER fires (the checks annotate the
  file's `oracle_kind` short name, so they always agree — that's deliberate, §6 strain-W2).

## §6 What strained (the primary deliverable)

- **strain-W1 — the annotation kind MUST be the lifted `oracle_kind`, not the 19H §2
  reverse-DNS form.** The whole probe/effect-map/cell machinery is keyed by the SHORT kind
  (`package`, `pkgindex`, `service`). If a corpus check annotated `com.debian.apt.Package`
  (the 19H §2 example convention), the cell's kind would become that reverse-DNS string,
  `idx.probe_for(reverse-dns)` → None (the probe is registered under `package`), and the
  install could not be elided — AND the kind-agreement warning would fire on every command.
  So the kind-agreement rule (annotation wins) is sound but, for a COHERENT oracle, the
  annotation MUST match the effect-map kind. The corpus checks annotate the short kind; the
  reverse-DNS form survives only in the task-C unit tests (`check.rs`, which test the
  evaluator in isolation, not against a real effect-map). +SURE this is the right call for the
  spike; ~SUSPECT a real reverse-DNS↔short-kind story is owed (the `an-named-kind` anchor is
  the reverse-DNS string per 175 C2, but the effect-map/probe key on the short `oracle_kind` —
  these are two different identifiers today, papered over by making them equal).

- **strain-W2 — the nullary/Singleton case forced an evaluator extension (the sharp one).**
  `apt-get update` resolves to `package-index#fresh` as a **Singleton** (no operand), and the
  probe golden (`pkgindex__check` with NO arg, results key `pkgindex#fresh`) is load-bearing.
  But task-C's `evaluate` required an annotation with a resolved VALUE (entity mandatory;
  `MissingAnnotation`/`UnresolvedAnnotationValue` ⇒ Top). A nullary verb has no operand to
  annotate. I extended the dialect with a **value-less annotation** (`index : pkgindex`, no
  `= value`) ⇒ `ResolvedEntity::Singleton`; the wiring keys it on `EntityRef::Singleton`. The
  alternative (annotate the verb-string as the entity ⇒ `Operand("update")`) would have
  changed the golden (`pkgindex__check 'update'`) and is semantically wrong (update has no
  entity). The value-less spelling is EXPLICIT (a wholly-missing annotation still ⇒
  `MissingAnnotation` Top, the safe direction) — pinned by `value_less_annotation_with_
  equals_is_an_error`. ~SUSPECT this is a `tc-*`-adjacent dialect decision (is a value-less
  annotation "idiomatic sh"? it reads as a no-op `name : kind` command — sanctioned spike debt,
  like the valued form already breaks the off-ramp).

- **strain-W3 — multi-operand refusal moved from the engine into the oracle's own code, and
  it depends on sh-faithful past-end semantics.** find-3 refused `>1 operand` (Opaque → run),
  which kept `apt-get install nginx curl` SAFE (it must run — eliding it would drop curl, a
  `kFAIL-perform` wrong-elision; the corpus pins this via `exec-multi-entity`). The new check
  is single-`$1`, so it would resolve entity=nginx and (if converged) wrongly elide, silently
  dropping curl. The fix: the apt check guards its probe with `if [ "$2" = "" ]` — a SECOND
  operand ⇒ no probe reached ⇒ `Top(NoProbeReached)` ⇒ Opaque ⇒ runs. But this only works
  because I made `eval_test` treat a past-end `$2` as the empty string (sh's unset-parameter
  semantics) — task-C's evaluator errored on past-end positionals UNIVERSALLY (the safe
  choice for an ANNOTATION value, but WRONG for a `[ ]` test, where sh expands unset to empty).
  I scoped the empty-string semantics to `resolve_in_test` ONLY (the annotation value-position
  stays strict). This is load-bearing for BOTH the multi-operand guard AND the post-verb
  flag-strip `while [ "${1#-}" != "$1" ]` terminating on an exhausted argv. +SURE it's sh-
  faithful and the right seam; pinned by `test_context_past_end_positional_is_empty_string`.
  **Consequence the oracle-author bar inherits**: the engine no longer provides multi-operand
  safety — each oracle's check must refuse extra operands itself (the `[ "$2" = "" ]` idiom),
  or it will wrongly resolve a multi-operand invocation to its first operand. This is a real
  oracle-quality cliff the find-3 stand-in was hiding.

- **strain-W4 — two checks per provider across files, resolved by "try each, first
  `Resolved` wins".** `apt-get` is declared in TWO oracle files at once
  (`package.oracle.sh`: install/purge → package; `pkgindex.oracle.sh`: update → pkgindex),
  each with its OWN `apt_get__check`. `CheckSet` is keyed by provider per-file, so a merge
  would collide. The wiring iterates ALL files' CheckSets for the provider and takes the first
  that yields a `Resolved` (the others `Top` on the verb they don't handle — the package check
  Tops on `update` via the strict annotation-value past-end; the pkgindex check Tops on
  `install` via case-fall-through `MissingAnnotation`). The partition is clean for the corpus.
  **tc-* (flagged, NOT resolved)**: if two checks both `Resolved` the same command, the
  first-in-oracle-file-order wins — order-dependent, but no corpus case is ambiguous. A real
  collision (two oracles both fully claiming `apt-get install`) is the `an-cross-oracle-
  coherence` contract's job (a CI lint, never enforced — inc-9), not the engine's.

- **strain-W5 — the dialect rejects pipelines, and an oracle's natural probe body IS a
  pipeline.** The `ufw` and (real) `package` probes are `ufw status | grep` / `dpkg-query |
  grep` — the `|` is out of the check dialect (strain-4 in 204). When I first transcribed the
  firewall check's body verbatim with the pipe, the WHOLE `ufw__check` failed to lift ⇒ ufw
  Opaque ⇒ it poisoned the systemctl commands downstream ⇒ the headline probe lost BOTH its
  firewall AND service checks (a cascade). I rewrote the check's probe body to a single command
  (`ufw status "$rule" >/dev/null`). This is harmless THIS round (the probe artifact comes from
  `oracle_probe_*`, not the check's body — the check only resolves identity), but it is a real
  coverage gap: task-D's probe-projection (which WILL ship the check's `probe_body` spans) must
  confront that the idiomatic blessed probe is a pipeline the dialect can't carry. The `package`
  oracle's REAL probe (`dpkg-query … | case`) is in `oracle_probe_package`, not the check —
  the check's body is a simplified `dpkg-query -W "$pkg" >/dev/null 2>&1` placeholder.

- **strain-W6 — two `oracle::lift` and `check::lift_checks` double-parse the same file, and
  the book parser chokes on the dialect.** `oracle::lift` book-parses the WHOLE oracle file
  (for `oracle_kind`/`oracle_probe_*`/`oracle_effect`); the `<provider>__check` funcdefs now
  contain `while`/`case`, which the book parser ⊤-rejects, emitting `syntax-unsupported: loop
  constructs` diagnostics that broke `lifts_the_package_fixture_cleanly` (it demands a clean
  lift) and polluted e2e stderr. Fix: `lift_one` now SUPPRESSES parse diagnostics whose span
  falls inside a `*__check` funcdef body (those funcdefs are `check`'s dialect, not book sh,
  and `lift` ignores them anyway). A parse error OUTSIDE a `__check` body still surfaces. This
  is the `adj-dialect-parser` separation (203 §4) biting at the file level: ONE file, TWO
  front-ends with incompatible grammars. ~SUSPECT the cleaner long-run shape is a single
  pre-pass that splits the file into book-items vs check-funcdefs before either parser runs.

- **strain-W7 — `find-3` is an overloaded slug.** The prompt asked for `rg 'find-3'
  spike/crates` to be empty, but `cfg.rs`/`cfg.rs` tests use "find-3" for a DIFFERENT finding
  (the errexit edge-pruning "note 166 find-3", the `|| true` family). I removed every
  *entity-resolution* find-3 reference (the STAND-IN markers + my explanatory comments now say
  "the deleted engine-side argparse stand-in"), but left the 4 errexit-finding ones intact —
  renaming them would corrupt an unrelated finding's provenance. So `rg 'find-3' spike/crates`
  returns 4 hits, all the errexit finding. **Flag**: if the human wants the slug truly unique,
  the errexit finding should be re-slugged (it predates this task).

- **strain-W8 — `SkipClass` is single-fact, so a multi-cell verb folds to `MustRun`.** The
  effect-map now stores a `Vec<EffectCell>` and the reaching-defs `gen` applies EVERY cell
  (so a multi-cell mutator correctly poisons/establishes all its cells). But `SkipClass`
  (`EstablishAmbient(FactKey)`) holds ONE fact, so a node with >1 establish cell has no
  single-fact elision representation — it folds to `MustRun` (sound: `kFAIL-perform`, never
  wrongly elided; the run-it floor). No corpus verb is multi-cell, so this is untested beyond
  the unit `multi_cell_verb_different_selectors_both_recorded` (which tests the index, not the
  plan). Multi-fact elision is unbuilt past `SkipClass`'s shape — deferred-not-irrelevant.

## §7 What task-D needs to know

- **The check's `probe_body` spans are computed but UNUSED this round.** The probe artifact
  still comes from `oracle_probe_<kind>` (via `compile_probe` + `idx.probe_for`), keyed by the
  cell's kind. `evaluate` accumulates `Resolved.probe_body` (verbatim spans of the selected
  path's probe commands), but `analysis::effect` discards it (it only reads `kind`, `entity`,
  `verb`). When task-D re-keys probe-projection to ship the check's per-site `probe_body`
  (202 §3 site-keyed artifact), it must reckon with strain-W5: a `probe_body` span that is a
  PIPELINE is out of dialect, so the corpus checks carry single-command placeholder bodies, NOT
  the real (pipelined) `oracle_probe_*` bodies. The two probe sources will need reconciling.
- **rule-anno-render (205 §1) is still owed.** The check bodies' annotation lines
  (`pkg : package = "$1"`) are NOT inert under dash (they PATH-execute `pkg`); when task-D
  ships a check body, the emitter must render the annotation node as a plain assignment
  (`pkg="$1"`). The annotations currently reach the corpus oracle FILES only (which the e2e
  `dash -n`-checks but never executes, and which `oracle::lift` reads structurally); shipping
  them verbatim into a probe would break it.
- **The ε-verb (`empty_verb` = `""`) and the kind-agreement rule are the two seams task-D
  inherits at the cell boundary.** A verbless command's cell is built from `effect_of(provider,
  ε)`; the cell kind is the annotation kind (not the effect-map kind) on disagreement.
- **`exec-opaque-var` is misnamed** (§4) — fold the rename + the new genuinely-⊤ case into
  task-D's corpus additions if the human approves.
- **The "try each CheckSet, first-Resolved-wins" ambiguity (strain-W4)** is an unguarded
  order-dependence; if task-D adds cross-oracle coherence checking, this is where a real
  collision would surface.
