# 204 — task-C build: the `check()` dialect parser + evaluator, and where it strained

> Round-20 spike note, append-only. Records the build of `dorc_oracle::check` (task-C,
> 202 §6): the command-keyed `check()` contract — a dedicated mini-parser for the
> constrained oracle-contract dialect (`adj-dialect-parser`, 203 §4) plus a concrete
> evaluator that traces a known argv through a check's argparse to its kind-annotation
> (202 §1 face-check). AI-authored, confidence-marked. Additive-only: the existing
> `oracle::lift`/`KindIndex`/`Polarity` are untouched; this is the NEW input side that
> task-W later wires in to replace the `find-3` engine-side argparse. Trust
> R/D/I/K + 19H/19I + the human rulings over this.

## §0 What landed

A new `check` module in `dorc-oracle` (`src/check.rs` + `check/{ast,lexer,parser,eval}.rs`),
plus `crates/oracle/tests/check.rs` (33 adversarial integration tests). All four gates
green (`fmt`, `clippy --workspace --all-targets -D warnings` with **no new `#![expect]`**,
`test --workspace`, `typos spike`); e2e 42/42 (xfail intact — corpus behavior unchanged).

Public API (additive, in `dorc_oracle::check`):
- `lift_checks(&mut Interner, &str) -> Carrier<CheckSet>` — parse one oracle source's
  `<provider>__check` functions into a `CheckSet`, fail-soft per function.
- `evaluate(&Check, argv: &[&str]) -> Resolution` — concretely trace `argv` (the full
  verbatim args, NOT the command word — C-1) through a check.
- `CheckSet` (`.get(Symbol) -> Option<&Check>`, `.providers()`, `.len/.is_empty`),
  `Check`, `Resolution::{Resolved(Resolved)|Top(TopReason)}`, `Resolved{kind, entity,
  verb, probe_body}`, `TopReason` (closed enum).

## §1 The dialect grammar I ended up with (greppable)

The mini-parser admits exactly this and ⊤-rejects (per-function lift diagnostic)
everything else. Grown only as 19H §2's five examples demanded.

- **funcdef**: `<provider>__check () { <stmt>* }`. The name before `__check` is the
  provider (underscore↔hyphen, §3). A leading `function` keyword is NOT admitted (no
  example uses it; bashism).
- **stmt** = one of:
  - **assignment** `name=<word>` (`verb=$1`, `pkg=$1`). Bare `name=` ⇒ empty literal.
  - **shift** `shift` | `shift <int-literal>`. A dynamic `shift $x` ⇒ ⊤.
  - **while** `while [ <test> ]; do <stmt>* done`.
  - **if** `if [ <test> ]; then <stmt>* [else <stmt>*] fi` (admitted though no §2
    example uses it — 19H §2 says "`if`/`then`/`fi` where needed").
  - **case** `case <word> in (<arm>)* esac`; arm = `[(] <pat> (| <pat>)* ) <stmt>* (;; | esac-peek)`.
  - **annotation** `name : reverse.dns.Kind = <word>` — recognized as the 5-word shape
    `[ident, ':', kind, '=', value]` (the `ch-shape-anno` inline form; 202 §4).
  - **command** any other word-led line: `<word>+ <redirect>*`, span preserved verbatim.
- **test** (inside `[ … ]`): `<word> = <word>` | `<word> != <word>` ONLY (the
  `${1#-}`-strip idiom needs just string eq/neq). Other test operators ⇒ ⊤.
- **word** = `literal` | `$N` | `"$N"` | `${N#prefix}` | `$name` | `"$name"` |
  `'…'`(single-quoted literal). `$@`/`$*`/`$#`/`$?`/mixed words/`${…}`-other ⇒ kept as
  a non-resolving literal ⇒ degrades to ⊤ if it reaches a value-position (the safe
  direction). `${N#prefix}` admits only a **literal** prefix (`${1#-}`).
- **pattern** (case arm): `literal` | `*`. Any non-trivial glob (`?`, `[…]`, `*foo`) ⇒ ⊤
  (arm-selection must be concrete equality, never a pattern-match — `inv-kfail`).
- **redirect**: `>`/`>>`/`<`/`N>`/`N>&M`/`>/dev/null` lexed opaque, folded into the
  preceding command's verbatim span (never interpreted).

The lexer is byte-oriented and total; hostile input (NUL, unterminated quotes, deep
nesting, BOM, multibyte) ⇒ `Tok::Error` ⇒ per-function lift diagnostic, never a panic
or hang (a lexer iteration cap + a parser step guard back the budget belt-and-suspenders).

## §2 Evaluator semantics

Concrete interpreter (no fixpoint): `positionals: Vec<String>` (mutated by `shift`),
`vars: BTreeMap<Symbol,String>`, a `verb: Option<String>`, an accumulated
`probe_body: Vec<Span>`, and the first `annotation` reached. Budget = `4*argv.len()+32`
steps (loop iterations + statements); exceed ⇒ `Top(BudgetExceeded)`.

- `${N#-}` strip, `case` first-matching-arm, `shift`/`shift N` consume — all exactly as
  sh. `case` with no matching arm and no `*` **falls through** (sh semantics), NOT ⊤.
- `verb` is the value bound to a variable the oracle names `verb` (compared by interned
  `Symbol`, stamped on `Check.verb_sym` at lift — never decoding the var's text). No
  `verb=` ⇒ no verb (first-class absence; `useradd`/`command` bind none).
- `Resolution`: `Resolved` iff an annotation resolved concretely AND ≥1 probe command
  ran on the selected path; else `Top`.

### `TopReason` variants (every degrade — closed enum)
`EmptyArgv` · `NonConcreteWord(&str)` (unbound var, positional-past-end, `$0`,
shift-past-end) · `MissingAnnotation` · `NoProbeReached` (annotation resolved but the
selected path ran no probe — e.g. `systemctl` unknown verb falls through all arms) ·
`UnresolvedAnnotationValue` (the annotation's value-word didn't resolve — e.g. `"$3"`
over a 2-arg argv, or an unbound var) · `BudgetExceeded`.

## §3 The underscore↔hyphen provider rule (chosen; flagged tc-*)

`apt_get__check` ⇒ provider `apt-get` (every `_` in the pre-`__check` fragment → `-`).
**Lossy**: a command with a literal `_` cannot be named (none of the §2 examples have
one; sh function names cannot contain `-`, so the mapping is the *only* way to name a
hyphenated command, and hyphenated commands dominate). +SURE this is right for the
five examples; ~SUSPECT it bites eventually (e.g. a tool literally named `foo_bar`).
This is a `tc-*`-shaped cross-cutting decision (the engine's provider-name vocabulary is
shared with `KindIndex`'s `ProviderId` and the book's command-word interning) — **flagged
up, not resolved locally**. The conservative knob if it ever matters: an explicit
provider-name escape in the dialect, or a `__check`-adjacent declaration. Recorded so
task-W doesn't re-derive it.

## §4 What strained (primary deliverable)

- **strain-1 — 19H §2.1 is internally inconsistent on flag ordering (the sharp one).**
  The book line is `apt-get install -y "$pkg"` ⇒ argv `[install, -y, nginx]` (flag AFTER
  the verb). But 19H §2.1's walkthrough says "the `while` consumes `-y`, stops at
  `install`" — which only holds if argv were `[-y, install, nginx]` (flag FIRST). The
  transcribed `while [ "${1#-}" != "$1" ]` strips only **pre-verb** flags. So for the
  book-order argv, the code yields entity=`-y` (after `verb=install; shift`, `$1` is the
  unstripped post-verb `-y`), NOT nginx. The prompt's own expectation (`["install","-y",
  "nginx"]` ⇒ entity nginx) inherits the inconsistency. Per the prompt's tiebreaker
  ("assert exactly what its code does, not what apt-get really does"), I pinned the
  faithful results in **two** tests: book-order ⇒ entity=`-y` (strain-1), flag-first ⇒
  entity=nginx. +SURE this is a 19H defect, not an evaluator bug — verified by tracing
  both orderings. **Consequence for the model**: the transcribed apt-get check does NOT
  actually resolve a real `apt-get install -y nginx` book line to `nginx`; a correct
  apt-get oracle must strip flags *after* the verb too (or interleave), which the §2.1
  one-liner doesn't. Task-W / the oracle-author bar must confront this: the canonical
  example check is wrong about its own tool's grammar. (Does not affect the engine's
  correctness — a wrong entity from a wrong oracle is the oracle's bug; the engine
  faithfully ran the code. But the flagship example misteaches the pattern.)

- **strain-2 — `case` fall-through forced an evaluator design fork, resolved via a new
  `Top` reason.** My first cut made any no-arm-matched `case` ⇒ `Top(NoArmSelected)`.
  That broke the legitimate flag-strip `case $1 in -v) shift ;; esac` when `-v` is
  absent (real sh falls through harmlessly, and `command -v nginx` must still resolve).
  Fix: `case` fall-through is `Flow::Normal` (faithful sh); the systemctl-unknown-verb
  case (`restart`) is caught instead at `finish()` by **`NoProbeReached`** — annotation
  resolved but no probe command ran because the probes live inside the arms. +SURE this
  is the right seam: it keeps the evaluator faithful to sh while still biasing the
  un-actionable (probe-less) resolution to ⊤. Found by the `command_v [nginx]` test
  failing — exactly the kind of cross-cell break the exclusion-check warns about (a "fix"
  for systemctl broke command).

- **strain-3 — the prompt's "unconsumed flag reaching the annotation ⇒ Top" expectation
  collides with `inv-referent-agnostic`.** If an oracle's argparse doesn't strip a flag
  and the annotation reads `$1`=`-y`, the engine binds entity=`-y` — because the engine
  parses NOTHING; only the oracle's code decides what's a flag (C-1 / `inv-referent-
  agnostic`). Refusing `-y`-as-entity would require the engine to recognize flag-shape,
  which is precisely the find-3 sin we're removing. So I pinned entity=`-y` (a faithful
  `Resolved`), NOT a `Top`. ~SUSPECT the prompt's framing assumed a well-formed oracle
  that strips its flags; for a *malformed* oracle, the faithful flag-as-entity is correct
  and the oracle's bug, not the engine's. Recorded as a deliberate divergence from the
  prompt's Top-case list.

- **strain-4 — span fidelity is clean, but only because probe bodies are single
  commands.** The verbatim `Command.span` round-trips exactly (tested: `dpkg-query -W
  "$pkg"`, `command -v -- "$tool" >/dev/null`, both systemctl arms incl. a preserved
  doubled space). No pain *here* — but the spans are per-`Command`, and a probe body that
  is a pipeline / multi-command sequence is currently out of dialect (the parser rejects
  `|` in command position). If a real oracle's probe is `dpkg-query … | grep …` (the very
  idiom `17O F-1`/`package.oracle.sh` warns *against*, but which exists in the wild), it
  ⊤-rejects. +SURE that's the safe degrade; ~SUSPECT it's a coverage gap the oracle bar
  will hit. Deferred-not-irrelevant.

- **strain-5 — budget calibration is untested against a *real* terminating argparse's
  depth.** `4*argv.len()+32` is generous for the five examples (their loops do O(argv)
  work). The budget test uses a deliberately non-terminating `while [ "$1" = "$1" ]`. I
  have NO evidence about where the constant should sit for a pathological-but-legal
  oracle (deeply nested `case` in a `while`). ~SUSPECT it's fine (the dialect can't
  express much per step), but it's a guess, not measured.

- **strain-6 — `eval` in a probe body is NOT caught by this parser.** `eval "$x"` lexes as
  a plain command and lifts as a probe-body span (a test pins this). The dialect
  constrains *control constructs*, not command *words* — so `eval`/backticks-as-a-word
  inside an already-parsed command aren't rejected here. The probe-inertness gate
  (`dq-reflexive-probe-inertness`, 19H §1.3) is a *separate component* (out of task-C
  scope) that must catch a mutating/`eval`-ing probe body. Flagged so task-W/the
  inertness builder knows this module does not provide that guarantee. (Backticks and
  `$(…)` *as a word being lexed* ARE rejected — they produce `Tok::Error` — but `eval`
  as a bare command word is not.)

## §5 Decisions taken locally (within charter latitude; cheap to overrule)

- **Separate front-end, zero `dorc-syntax` reuse** (203 §4 executed). The dialect parser
  shares NOTHING with the book parser; books with loops stay ⊤ (sound). +SURE this was
  right — the book parser's loop-rejection is load-bearing and I'd have had to unpick it.
- **`verb` as a reserved-by-convention variable name** (§2). Recognized by `Symbol`
  equality, not text decode. ~SUSPECT this should be documented as a dialect rule (is
  `verb` reserved?); flagged tc-*-adjacent. The safe fallback (no `verb=` ⇒ no verb) means
  misrecognition only ever loses a verb, never invents one.
- **First-annotation-wins; first-arm-wins** (sh `case` semantics). A second annotation on
  a path is ignored. No example has two; +SURE.
- **`$1`/`"$1"` identical; `'$1'` is the literal `$1`** (sh quoting). Single-quoted value
  ⇒ `Resolved` with entity=`"$1"` (the two-char string), tested. Documented.
- **partially-concrete argv is NOT this module's concern.** `evaluate` takes `&[&str]`
  (fully concrete) per 202 §1's face-check scope rule. A book site with a `⊤`-hole in its
  argv never calls `evaluate` (the caller degrades the site). Deferred-not-irrelevant
  (the lazy-admin one-variable-package case, 202 §1).

## §6 The contract task-W (wiring) needs

How a caller goes from a book site to a `Resolution`:
1. face-book (task-A, `analysis::value`) resolves the site's command word + argv to
   concrete strings. The command word selects the provider; **strip it** — pass only the
   trailing args to `evaluate` (C-1). If any argv element is ⊤, do not call `evaluate`
   (site unresolved ⇒ runs).
2. Look up the provider's `Check` in the file's `CheckSet` (key = the interned,
   hyphen-mapped provider `Symbol`). The `CheckSet` comes from `lift_checks` over the
   oracle source(s). Provider-name interning MUST match the book's command-word interning
   AND `KindIndex`'s `ProviderId` interning — same `Interner`, same hyphen convention
   (`apt-get`, not `apt_get`). **tc-* (§3) lives exactly here.**
3. `evaluate(check, &args)`:
   - `Resolved{kind, entity, verb, probe_body}` → the site's cell. `kind` is the
     reverse-DNS string (intern to a `KindId` — it is the `an-named-kind` cross-oracle
     anchor, never decoded). `entity` is the resolved operand text (→ an `OpaqueToken` via
     the interner → `EntityRef::Operand`). `verb` (if `Some`) is the selector key; `None`
     keys on `(provider, ε)` per 202 §2. `probe_body` is the verbatim span(s) to ship.
   - `Top(reason)` → the site is un-probeable and un-elidable (runs). `reason.as_str()`
     feeds a diagnostic.

### What of the OLD `lift`/`KindIndex` this replaces vs. coexists with
- **Replaces** (task-W, eventually): the `find-3 STAND-IN` in `analysis::effect`
  (`command_effect`'s `verb=word-1` + `resolve_entity`'s flag-strip). `evaluate`'s
  `(entity, verb)` is the principled source of both — the engine does ZERO argparse.
  The `find-3 STAND-IN` doc-comment block in `effect.rs` names exactly this hand-off.
- **Coexists with / still needed**: the **effect-map** (`KindIndex.effects`, the
  `(provider, verb) → (kind, selector, Polarity)` table from `oracle_effect` markers) is
  NOT replaced by `check` — 202 §2 keeps it as a separate oracle-side declared map, now
  keyed by the *argparse-derived* verb (`evaluate`'s output) instead of word-position.
  So: `check` resolves `(entity, verb, kind, probe_body)`; the existing effect-map still
  supplies `(selector, Polarity)` per `(provider, verb)`. **Open seam for task-W**:
  `evaluate` ALSO yields `kind` (from the annotation) — this overlaps the effect-map's
  `kind`. ~SUSPECT they must agree (the annotation's kind == the effect-map's kind for
  that provider); a mismatch is a tc-* (which wins? — I lean: the annotation is the
  entity's kind, the effect-map's kind is redundant-but-should-match, lint on divergence).
  Flagged; NOT resolved here.
- The `FactProbe` (per-kind probe body in `KindIndex`) is **superseded** by `check`'s
  per-site `probe_body` spans for the command-keyed path — `check` ships the check
  function + the selected verbatim body, not a per-kind probe. 202 §3's site-keyed probe
  artifact is the transport. (The old `FactProbe` may linger for any non-`check` path
  until task-W/D fully cut over.)

## §7 Confidence

+SURE: the grammar covers all five §2 examples and they resolve as the *code* dictates;
the no-throw/determinism properties hold (hostile-garbage test + deterministic-lift test);
all gates green; corpus unchanged. ~SUSPECT: budget constant (strain-5), the
annotation-kind vs effect-map-kind agreement (§6), `verb`-reservation spelling (§5). The
sharp finding is strain-1 (the flagship apt-get example mismodels its own tool's flag
grammar) — task-W and the oracle-author bar must reckon with it.
