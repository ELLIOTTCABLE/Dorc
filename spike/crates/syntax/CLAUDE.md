# spike/crates/syntax — CLAUDE.md

The hand-rolled lexer + recursive-descent parser + arena AST for the modeled sh
subset. Read `spike/CLAUDE.md` and `Research/plans/190-spike2-keystone-charter.md`.

## What this crate is for spike-2 — a disposable test front-end

For spike-2 the parser is a **disposable test front-end** (`ch-shape-anno`): its
only job is to feed the analyzer a faithful tree from the fixtures. You are
explicitly allowed to **massage input scripts/values** to get them past parsing;
accepting arbitrary shell-input is **not** a goal, and parser/lexer nightmares
aren't worth the time. Grow the grammar **demand-driven** — add a construct only
when a downstream analyzer/keystone need forces it, not prospectively.

Durable framing (`16P T2`): the subset's *contents* are disposable, but the
**boundary discipline is not**. Under-modeling is a *correctness* boundary
(elision-soundness), not a TODO — a half-understood construct could hide a
mutation that invalidates an elision, so anything unmodeled must become an
explicit ⊤ that is both un-probeable and un-elidable. Rule of thumb from the last
spike: *a weirdo that hits a syntax limit is fixed by the fixture, not the
grammar.* Prefer rewriting the fixture over chasing a grammar corner.

## The current demand-set (what already parses clean)

The grammar is grown to `fixtures/pi-webhost.book.sh` + `fixtures/package.oracle.sh`
and the three oracle idioms. Already modeled (don't re-derive): `set -e`; `case`
(alternation `a|b`, `*`, `:` null-cmd, command-subst scrutinee `"$(hostname)"`);
`if/elif/else` with a negated-pipeline condition (`if ! command -v nginx …`);
`&&`/`||` (left-assoc); pipelines; `( )` subshell / `{ }` group; `name() { … }`
funcdef; redirs incl. `>/dev/null`, `2>&1` dup, and `cat > f <<'EOF'` heredoc
(body captured, quoted-delimiter flag); the standalone-assignment statement
(`oracle_kind=package`, the **dn-1** metadata-in-sh anchor) and the
`oracle_effect apt-get install establish` marker-call (just a `Simple`). Lossless
quoting is wired (`WordPart`/`Word::may_split`). +SURE these are green
(`tests/parse.rs::fixture_pi_webhost_top_level_shape`).

## The ⊤-trigger set — fixed, and split by locus

The ⊤-trigger set is **fixed**; don't extend or relax it. It is split by *where*
it fires:
- **Syntactic ⊤, caught here** (`an-unsafe-boundary`, §K; `021 §2`): `eval`;
  dynamic command name (first word not a fixed literal); `. "$dyn"`/`source $f`
  (literal-target `. /etc/x` is kept, not rejected); `$(( … ))` in command
  position; lvalue-taking builtins (`unset "$x"`, `printf -v`, `test -v`/`[ -v ]`);
  loops (`for`/`while`/`until`); background `&`; over-deep nesting. Each → an
  `Unsupported` node with the right `UnsupportedReason` **plus** an `Error`
  diagnostic — loud, never silent (`inv-top-reject`). Salvage children so
  unrelated analysis proceeds (`dn-7`).
- **Semantic ⊤, NOT here** — deferred to the dataflow: no-oracle-entry, and the
  *dynamic word* (unquoted `$x` as an argument). The crate's job is to *preserve*
  the signal losslessly (`Word::may_split`), not to ⊤-reject it. ~SUSPECT this is
  the easy thing to get wrong: do not collapse a `may_split` word to ⊤ in the
  parser.

The real ⊤-surface is **dynamic arguments + command-substitution**, not just
dynamic command *names* (`an-top-surface`, §K, `150 fN-ANALYZABILITY`; the #2
construct by frequency). The parser must keep `$()`/args lossless so the
analyzer can size and gate that surface; ⊤-rejecting only dynamic command-names
would under-count it.

## Lossless quoting is a correctness need, not a nicety

`an-word-expansion` (§B; `021 §2` "first-class hazard", 80% of real scripts carry
≥1 smell): an unquoted expansion changes a command's **arity and its
effect-target set**, so `echo "$x"` (one field) and `echo $x` (may split into
many) must stay distinguishable. The `WordPart` ladder records *how* each
fragment was quoted; `Word::may_split` *derives* the splitting hazard. Keep this
intact when you touch words — flattening a `Word` to a `String` is an
elision-soundness regression. Same for redirs-as-first-class (`an-redirection-effect`):
`: > /etc/x` mutates regardless of the (no-op) command word, so a redir is its
own node, not a flag on the command.

## The `kTYANNOT-inline` experiment (the one thing to *build* here)

`ch-shape-anno` locks the **inline type-annotation strawman** for this spike (the
`kTYANNOT` inline pole the human flagged). Concretely the shape is, e.g.,
`local w : com.frobber.Wombat{frocked} = "$1"` / a `return … : "$w" is …` /
dotted `frobctl.check()` (see `17O F-OFFRAMP` strawman A). -GUESS the parse work
lands on the `Assign`/word path + funcdef-name lexing; it is **not yet built** —
the current `Assign` only carries the bare `oracle_kind=package` anchor. Parse it
demand-driven when the keystone/oracle-lift needs it.

**Accepted debt — do not "fix" it.** This breaks the off-ramp weld
(`17O F-OFFRAMP`, half-violating `kOOB`), verified live: `local w : T=…` →
`dash` aborts (`local: :: bad variable name`); `bash` leaves `w` empty at rc 0
(silent corruption); dotted `frobctl.check` → `dash -n` "Bad function name".
That is the known, charter-accepted cost for this spike. Do **not** build the
correctness-critical strip/transpile pass (`ch-shape-anno`, OUT in charter §6) —
it is a source-to-source transpiler, not a regex strip, and out of scope.

## Why hand-rolled + disposable is legitimate

The trust model is **differential testing against dash/bash**
(`an-differential-vs-shell`), not proof. Even CoLiS — maximally
formal-methods-capable — declined to prove its shell parser correct ("the spec
is informal … we do not even claim the absence of bugs"), trusting it via review
+ differential tests (`notes/010`). A clean-room hand-rolled recursive-descent
parser is the right altitude and *boring* is the goal (`notes/040`: the
permissive off-the-shelf options don't fit, so a ~2–5k-LoC hand-roll is the
realistic path). Lexing *unrestricted* shell is undecidable (alias/eval-driven);
we define that away by ⊤-rejecting those constructs. The `parse∘pretty-print =
identity` round-trip (`an-roundtrip-identity`) is a *specified-not-built*
obligation that would live wherever the renderer does, **not** here — don't go
looking for it in this crate.

## Totality mechanics already in place — preserve them

`inv-no-throw` rests on these; keep them when extending:
- `MAX_DEPTH` (256) depth-bound in `parse_command` — hostile nesting
  (`(((((…`, `$( $( …`) would otherwise blow the native stack (a panic-equivalent);
  past it we ⊤-reject and stop descending.
- Anti-stall guards (`parse_command_list`, `lex_word`): if no token is consumed,
  force one byte/token of progress so `parse` always terminates (the no-progress
  cases are 24-GiB-allocation / infinite-loop footguns — see the lexer's
  backtick comment).
- The lexer always terminates the stream with `Eof`; unterminated quotes/heredocs/
  substitutions close at EOF (the parser raises the diagnostic). `tests/parse.rs::
  totality_hostile_inputs_never_panic` is the table this must keep passing.
- `an-crlf-hazard` (§K): a `\r` in authored `.dorc.sh` corrupts compares/heredocs/
  `read`/`case` (and `\r` in a shebang is an un-guardable exec failure). Out of
  scope to *handle* here, but if you touch the lexer's line/heredoc logic, don't
  silently mangle CRLF — surface it (the wire-transform that fixes it is the
  emitter's job, `an-wire-transform`).

## Honor

`inv-top-reject` (unmodeled → explicit ⊤ node + loud `Error`, never silently
best-effort'd); `inv-no-throw` (`dn-7`: total — never panic on hostile/untrusted
input, errors are data via `Carrier<T>`); `inv-determinism` (same bytes ⇒
byte-identical arena + diags; no hashed iteration into output).

## Tensions (flag, don't resolve)

- `tn-anno-vs-offramp`: the `kTYANNOT-inline` build *is* the off-ramp break — the
  two are the same act. The charter accepts it for this spike, but it sits
  squarely against `inv-top-reject`'s spirit (we're parsing sh that stock dash
  rejects). Surface where it bites; don't try to reconcile it.
- `tn-massage-vs-fixture-trust`: "massage inputs past the parser" (`ch-shape-anno`)
  is in mild tension with `ap-2`'s executable-acceptance — a fixture massaged into
  parseability may no longer be runnable sh. Keep an eye on whether a massaged
  input still `sh -n`-checks; if not, the acceptance harness can't vouch for it.
- `tn-coarse-subst-provenance`: command-substitution bodies are re-lexed and their
  inner diagnostic spans are *relative to the inner text*, not the outer source
  (parser comment at `parse_subst_body`). Accepted coarse provenance for the spike;
  it will fight the locator-DAG (`an-loc-user-src`, §F) if provenance fidelity
  later matters.
