# spike/crates/syntax — CLAUDE.md

Role: hand-rolled lexer + recursive-descent parser + arena AST for the modeled sh
subset. A **disposable test front-end** with a **non-disposable boundary
discipline**. Read `spike/CLAUDE.md` first. Registry discipline: one rule per
bullet, slugged; append to the matching section.

## Law — boundary discipline (durable even though the subset's contents are not)

- **fixture-over-grammar** — massage fixtures past the parser; accepting arbitrary
  shell-input is a non-goal; grow the grammar demand-driven only (a downstream
  need forces a construct, never prospection). A weirdo that hits a syntax limit
  is fixed by the fixture, not the grammar.
- **top-reject-here** — anything unmodeled becomes an explicit `Unsupported` node
  with the right reason PLUS a loud `Error` diagnostic — never silent; salvage
  children so unrelated analysis proceeds. Under-modeling is a correctness
  boundary (a half-understood construct can hide a mutation that invalidates an
  elision). The parser is the engine's highest-risk surface: bias every ambiguity
  to ⊤-reject-with-diagnostic.
- **syntactic-top-triggers** (fixed; shrinking one is a deliberate design act) —
  `eval` · dynamic command name · dynamic source target (literal `. /etc/x` is
  kept) · `$(( ))` in command position · lvalue-taking builtins (`unset "$x"`,
  `printf -v`, `test -v`) · background `&` · over-deep nesting · the residual
  loop shapes (no-`in` `for`; `break`/`continue`; command-subst/arithmetic in a
  for-list word).
- **semantic-top-not-here** — the dynamic-word/expansion surface (unquoted `$x`,
  `$()` arguments) is the analyzer's ⊤, not the parser's: preserve it LOSSLESSLY
  (`Word::may_split`), never collapse a may-split word to ⊤ in the parser —
  ⊤-rejecting only dynamic command-names would under-count the real surface.
- **lossless-quoting-is-correctness** — an unquoted expansion changes a command's
  arity AND its effect-target set; flattening a `Word` to a `String` is an
  elision-soundness regression. Redirections are first-class nodes
  (`: > /etc/x` mutates regardless of the no-op command word).

## Law — the dialect surface (parse target; `plans/281` is THE mark grammar (v0.2); `notes/277` entity algebra; `notes/278` base-dialect reference)

- **marker-gates-syntax** — `# dorc-lang/v0.2`, exact-match, stands alone, first
  ~10 lines: gates ALL non-POSIX syntax below. An unmarked file is plain sh; a
  marker naming an unrecognized version fails loud (recognized-set {v0.2}).
  `__role` NAME-recognition is ungated and permanent.
- **the-authored-surfaces** — inline binds (`pkg : sm.dorc.Package = "$1"`; the
  only bind form wired in production — trailing `:=`/`bind` ⊤-rejects) ·
  trailing/standalone marks with head sugar `:`/`:!`/`:?` for the core verbs
  (asserts/refutes/reads) and word verbs for the rest (safe-across · disturbs ·
  lends · stored-in · undivided-by-transit-across); TWO carriers, `:` (salient)
  and `#:` (inert comment) · the attached-`@` selector (`@` lexes as a word char
  under the floor shells and needs no comment-avoidance rule; entity-less
  `KIND:@SEL`) · brace-alternation `@{a,b}` (attached) / `verb {a,b}` (whole
  payload), refused ONLY on verdict payloads (rc-arity) · a statement-leading
  `:`/`#:` intro is a standalone mark line (a LONE `:` stays the null command) ·
  the `dorc:` command-word prefix (the only prefix-position dorcism). Kinds are
  reverse-DNS ≥2 dots (the verbs-dotless/kinds-dotted keystone, `281` §4); the
  selector rides `@`, and there is no `.prop` production (dots belong to kind
  names and entity content only).
- **charsets-posix-in-spirit** — all lexical minutiae follow
  `271:rul-posix-in-spirit-defaults` (find the POSIX rule, simplify, match in
  spirit): selector tokens are POSIX names in spirit; unquoted entities are the
  portable-filename set + `/`; quoted entities use POSIX quoting simplified.
  Narrow-start is deliberate — characters once granted can never be clawed back.
- **loud-or-nothing** — a mark failing its charset is a loud parse diagnostic,
  never a silent ⊤. No nested annotation inside opaque payloads
  (`271:rul-no-nested-annotation`, plan-time parse-failure tier).
- **fence-rejection-rc** — never depend on the exit code or error text of a
  construct the floor shells REJECT (dash exits 2 where posh exits 1); "parses
  and runs identically" is scoped to ACCEPTED constructs.

## Law — strip (the off-ramp; correctness-critical)

- **strip-is-pure-erasure** — binds + marks erased WHOLE-STATEMENT; `dorc:`
  prefix-erasure (`dorc:sh` → bare `sh`); the shebang-runner rewrite; NO in-body
  name rewriting (names are already bare POSIX NAMEs); `dorc-sh` typed directly
  is untouched (documented-dangle: half-strip is worse than no-strip).
- **last-substantive-command-rule** — a bare-mark statement is an annotation-LINE
  (deleted whole; it is NOT a POSIX `:` command): the author's last substantive
  command must remain the last status-affecting statement in the stripped body.
  A stripped-in trailing `:` clobbers the body's tool-rc to 0 = an always-skip
  guard.
- **executable-off-ramp-test** — stripped output parses and runs identically
  under `posh 0.14.1` ∩ `dash 0.5.12` (the kWHICHSH floor weld). Guaranteed for
  lint-clean text only: bare `set -o pipefail` is accepted-and-modeled but fails
  the floor by design, and strip never rewrites it (`276:rul-pipefail-emit-never`).

## Law — totality mechanics (`inv-no-throw` rests on these; preserve when extending)

- `MAX_DEPTH` (256) in `parse_command` — hostile nesting blows the native stack
  otherwise; past it, ⊤-reject and stop descending.
- Anti-stall guards (`parse_command_list`, `lex_word`): if no token is consumed,
  force one byte/token of progress so `parse` always terminates.
- The lexer always terminates the stream with `Eof`; unterminated
  quotes/heredocs/substitutions close at EOF (parser raises the diagnostic). The
  hostile-inputs never-panic table must keep passing.
- **crlf-hazard** — a `\r` in authored sh corrupts compares/heredocs/`read`/
  `case`; don't silently mangle CRLF in line/heredoc logic — surface it (the
  wire-transform fix is the emitter's job).

## Trust model + tensions (flag, don't resolve)

- **differential-not-proof** — the trust model is differential testing against
  real shells (even CoLiS declined to prove its parser). Hand-rolled + boring is
  the right altitude.
- **tn-marks-corrupt-bare** — trailing marks on real commands corrupt silently
  under `sh file` (marks become argv); the surface is narrowed by the shebang's
  loud-127, the marker gate, and strip — the residue is the kTYANNOT
  experiment's priced cost (`277` §4g). Record friction; don't redesign mid-task.
- **tn-massage-vs-fixture-trust** — a fixture massaged into parseability may no
  longer be runnable sh, which the ap-2 harness then can't vouch for; keep
  massaged inputs `sh -n`-clean.
- **tn-coarse-subst-provenance** — command-substitution bodies re-lex with
  inner-relative spans; accepted for the spike; will fight the locator-DAG when
  provenance fidelity matters.
