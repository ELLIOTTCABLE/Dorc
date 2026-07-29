# 294 — Oracle loading and resolution: the build ledger

Builder ledger for `plans/28K` (the function-environment pass), branch `ai/r28-oracle-loading`,
based at `ai/main` @ `7068224d`. Append-only; one section per stage as it lands. Confidence marks
per `spike/CLAUDE.md`. This is the *what strained* record — the design authority is `28K` itself
and the code is its own documentation.

Stage order, as ruled: A load-inert · B source-file identity · C-v0 function-environment domain
(literal source targets) · D environment drives every lift · E cross-unit shadow refusal ·
**[conductor checkpoint]** · F pin-by-definition-bytes · G two-kind fixture respell ·
H differential load-order battery · C2 value-flow source targets (budget permitting).

## Stage A — the marked-file load-inertness gate (LANDED)

`28K` §2a `rul-marked-file-is-load-inert`, implementing this crate's long-standing but unbuilt
`oracle/CLAUDE.md declarations-only-files` law. Marker-gated; a marked file's top level must hold
only function definitions and bare assignments whose values expand statically.

### fnd-two-parsers-disagree-on-funcdefs (carry into stage C — load-bearing)

`dorc_syntax::parse` and the dialect parser (`predict::lift_predicts`) do not agree on what a
function definition is, and the disagreement is SILENT. Measured, on the `munge-name-invalid`
fixture's `中pkg__predict() { :; }`:

- the dialect parser lifts it as a role funcdef (which is why the charclass refusal fires at all);
- `dorc_syntax::parse` yields THREE top-level items — a bare `Simple` command, an empty
  `Subshell`, and a `Group` — and emits **zero** diagnostics.

+SURE of the measurement (probed directly). This matters well beyond stage A: `28K` §2's function
environment reads `dorc_syntax::parse`'s `FuncDef` nodes, while every role lift reads the dialect
parser. Where the two disagree about what is defined, the environment and the lifts disagree about
which oracle answers a site — and today that disagreement produces no diagnostic at all.

Containment, ~SUSPECT but well-supported: the divergence is confined to names the sh lexer will not
take as a NAME, and `reserved.rs`'s charclass refusal already rejects exactly that class at Error
severity, so the two parsers agree on every name that can legally ship. Stage C should pin that
containment with a test rather than assume it, and `syntax/CLAUDE.md top-reject-here` arguably owes
a loud `Unsupported` where a funcdef header fails to lex — flagged, not fixed (out of lane).

### dec-one-diagnostic-per-file-not-per-item

First cut minted one diagnostic per offending top-level item. Measured against the real corpus that
was wrong: `unmodeled-wall-inventory` (a book wearing an `oracle.sh` filename) went 4 → 9 errors and
buried its own pinned info line, and `munge-name-invalid` went 1 → 4, the three extras all being
consequences of the single unlexable funcname above.

Collapsed to at most ONE mint per file, spanned at the FIRST offending item. The argument is the
mis-attribution test, not aesthetics: load-inertness is one claim about one file with one
remediation ("make this file definitions-only"), so N mints are a correlated cascade
(`AGENTS.md` fail-fast: only root-cause is reported) and N-1 of them point the author at the wrong
thing (`271:rul-sin-ordering` ranks mis-attribution worst). Cost, accepted: an author fixing the
first offender re-runs to find the next. If a count is ever wanted for prose, a payload field is a
cheap addition — the payload is deliberately empty today.

### Corpus churn (all four re-blessed, each +1 line, every prior pin byte-identical)

`mark-rc-arity-exceeded` · `mark-standalone-rc-consumer` · `unmodeled-wall-inventory` ·
`munge-name-invalid`. The first three were forecast by the CLI-round survey; `munge-name-invalid`
was not, and it is the one that surfaced `fnd-two-parsers-disagree-on-funcdefs`.

The two `mark-*` cases put their mark at top level *structurally*: `lint_mark_subset` only runs for
a file containing no `__`, so wrapping the mark in a funcdef to make the file legal would stop the
case firing at all. Re-blessing was the only option, and both errors firing is honest.

### res-load-inert-conservatism

Refused-for-now, each cheap to relax and expensive to re-tighten
(`271:rul-posix-in-spirit-defaults`): `export`/`readonly` (commands by AST shape); the operator
forms of parameter expansion (`${x:-y}`). The second has a real reason beyond conservatism — the
lexer collapses EVERY operator form to one opaque `ParamComplex` and discards the body, so
`${x:-$(hostname)}` is indistinguishable from `${x:-lit}` at this layer, and accepting the shape
would accept a hidden command substitution. Relaxing it needs the lexer to retain the body first.

### res-oracle-claudemd-bullet-now-implemented

`oracle/CLAUDE.md declarations-only-files` reads as unimplemented law; it is now enforced. The
bullet is not wrong, so it is left alone — a conductor-tier sharpening, flagged rather than taken.
