> LLM-generated; part of an intentionally quality-varied artificial testing
> corpus/tooling for a static-analysis project (Dorc); not production tooling;
> an artificial corpus cannot expose the truth of real-world ops-code.

# 1A7 — mechanical census tool (D2 of round-1A)

A durable, re-runnable command/construct census over POSIX-sh corpora, plus an
inert fixture and a self-test. Counts are produced mechanically by the tool
(a character-level awk lexer-approximation), not by reading the file. This note
records the extraction rules, the brutally-honest known-limitations (each marked
live or theoretical against *this* corpus), the self-test result, and the
verbatim top tables.

Deliverables:
- `Research/corpora/tools/census.sh` — the tool (dash-clean sh + one awk program)
- `Research/corpora/tools/census-fixture.sh` — inert fixture (hork/wombat/echo/true)
- `Research/corpora/tools/census-selftest.sh` — runs census over the fixture, diffs
  against hand-derived counts, exits nonzero on mismatch
- `Research/corpora/H2SaLS/census/{commands.tsv,constructs.tsv,summary.md}` — the
  census output over `harden.sh`

Confidence in the *headline corpus numbers*: ~SUSPECT (the self-test is green and
I cross-checked the high-count rows and every zero against `grep`, but a non-parser
on hand-written sh always has residual edge-mistakes; see limitations).

## How it works

One awk program reads every input line into memory, then makes two in-memory
passes per file (no per-line shell-out; one awk invocation total):

- **Pass 1 (collect)** runs the *same* scanner with output suppressed, purely to
  populate the function table `FUNC`. Using the real scanner (not a cheaper
  regex) means the two passes can never disagree about what is heredoc body, and
  a function *called before* its textual definition still classifies correctly.
- **Pass 2 (emit)** runs the scanner and writes a tagged record stream
  (`CMD`/`CON`/`ERR`) that the sh wrapper renders into the three reports.

The scanner is a character state-machine over each *logical* line (line-
continuations joined, multi-line quoted strings joined, heredoc bodies skipped).
It tracks: single/double quotes; comments (`#` only at a word boundary outside
quotes); `$(...)`/backtick command substitution with a save/restore stack so the
inner command starts fresh and the enclosing word resumes after `)`; arithmetic
`$((…))` (skipped); parameter-expansion forms; redirections; control operators;
`case` pattern-vs-body; and command position. A leading-assignment tally resolves
`NAME=val cmd` (env-prefix) vs `NAME=val` (plain) at separators/end-of-line,
gated to top level so command-substitution contexts don't corrupt it.

### Extraction rules (command position)

A token is in command position at logical-line start and after `;`, `&`, `&&`,
`||`, `|`, `(`, `{`, ``` ` ```, `$(`, and after the command-introducing keywords
`if`/`elif`/`while`/`until`/`then`/`else`/`do`/`!`. `for`/`case`/`in` do *not*
keep command position (the next word is a loop-var / case-word / pattern).
Env-prefix assignments (`VAR=x cmd`) keep command position for the following
word. Classification: `function` if defined in the corpus (both `name()` and
`name ()` forms, detected line-anchored and stripped before scanning), else
`builtin` (POSIX special + common dash builtins: `[`, `test`, `printf`, `echo`,
`set`, `export`, `read`, `cd`, `:`, `true`, `false`, `trap`, `eval`, `exec`,
`local`, `unset`, `return`, `exit`, `shift`, …), else `external`. `command -v X`
counts the `command -v` construct and does *not* emit `X` as a command. Heredoc
bodies, comment text, and single/double-quoted contents never yield command
tokens. `case` pattern labels never yield command tokens.

### A gawk gotcha worth recording (+SURE)

gawk hex escapes (`\x27` etc.) are reliable in **string** literals but are a
parse error / mis-match inside **`/regex/`** literals (`/\x5c/` errors with
"invalid trailing backslash"). The first draft used `/\x5c$/` to detect line-
continuations and silently treated *every* line as a continuation (the whole file
vanished). Fix: compare quote/backslash chars as string literals in the scanner,
and build the one needed backslash regex as a *dynamic* regex string (where a
literal backslash needs two backslash chars: `RE_TAILBS = BS BS "$"`). This bug
is the reason the self-test exists in the form it does.

## Self-test

The fixture exercises every construct class with hand-countable frequencies and
deliberately plants traps: command-like text inside heredoc bodies, comments, and
quotes (must not count); `case` pattern labels (not commands); env-prefix vs
plain assignment; `$(…)` inside `"…"`; nested `$( $( ) )`; `<<-` tab-stripping.
Expected counts were written by hand first; the tool reproduces them exactly. I
verified the test can fail (corrupting one expected value yields exit 1).

```
$ sh census-selftest.sh
ok: all construct counts match
ok: all command counts match
ok: external total = 33
ok: builtin total = 26
ok: function total = 3
ok: command-token total = 62
SELFTEST: PASS
```

## Corpus run

```
$ sh census.sh -o ../H2SaLS/census ../H2SaLS/harden.sh
census.sh: wrote commands.tsv, constructs.tsv, summary.md to ../H2SaLS/census
```

Headline (over `harden.sh`, 696 lines):

| metric | value |
|---|---|
| total command tokens | 174 |
| distinct external commands | 33 |
| total construct instances | 295 |
| command tokens by class | external 107, builtin 43, function 24 |

### Top-20 commands (verbatim)

```
printf            builtin    15  78,87,100,112,175,244,245,246,283,292,329,432,435,577,595
grep              external   14  75,97,111,169,172,281,282,286,288,326,431,451,574,592
apt-get           external   12  51,54,126,127,130,637,638,640,659,662,663,665
set_sshd_line     function   12  262,263,264,265,266,267,268,269,270,271,272,273
sed               external   11  76,98,173,242,249,289,327,432,433,575,593
[                 builtin     9  38,300,573,591,650,685,688,691,694
set_conf          function    9  439,440,441,442,443,444,536,537,539
cat               external    8  196,340,464,475,494,516,553,656
ufw               external    8  183,184,409,410,413,414,417,421
service           external    6  188,628,686,689,692,695
true              builtin     5  611,626,627,671,672
chmod             external    4  114,534,557,644
cp                external    4  96,256,301,568
mktemp            external    4  95,239,240,452
rm                external    4  104,259,465,626
echo              builtin     3  38,541,673
getent            external    3  58,64,109
:                 builtin     2  86,287
add_psad_logging  function    2  469,470
awk               external    2  302,453
```

Distinct external tail (count 1, by line): `ansi2html`(672), `chgrp`(533),
`chown`(115), `cut`(109), `dpkg-statoverride`(84), `gpg`(651), `groupadd`(58),
`hostname`(444), `id`(38), `mv`(304), `openssl`(65), `psad`(473), `tee`(302),
`useradd`(65), `visudo`(102); plus count-2 `cmp`(250,303), `install`(103,110),
`lynis`(671,672), `mail`(541,673), `rkhunter`(611,612), `wget`(627,643). The
profile is `sed`/`grep`-heavy idempotent line-editing over config files, an
`apt-get`-driven install spine, and the `ufw`/`service` firewall+restart work.

### Top-20 constructs (verbatim; full table incl. zeros in summary.md)

```
param $VAR plain          96
plain assign              45
func call                 24
if/elif                   23
redirect >                14
$(..) cmdsub              11
append >>                 11
param positional $1..$9   10
|| or-list                 8
heredoc unquoted           8
! negation                 6
pipe stage                 6
test [ ] numeric           5
func def                   4
brace group { }            3
fd-dup (N>&M)              3
redirect to-null           3
test [ ] file              3
&& and-list                2
for                        2
```

Zeros (each a finding, cross-checked vs `grep`): `backtick cmdsub`, `arith
$((..))`, `heredoc <<-`, `command -v`, `env-prefix assign`, `eval`, `exec`,
`local`, `trap`, `until`, `subshell (..)`, `tilde candidate`, `glob char
(unquoted)`, `redirect <`, `set -e`, `set -u`, and *all* `${..}`-operator /
`$@`/`$*`/`$#`/`$?`/`$$`/`$!` forms. The corpus is plain `"$VAR"`-style
expansion only — no braced-operator expansions survive outside heredoc bodies
(the single `${distro_codename}`, line 364, is heredoc content). `eval`/`exec`
are absent (good); flagged loudly only if nonzero.

## KNOWN LIMITATIONS (a non-parser misses real shapes)

Each marked **LIVE** (the corpus actually contains the shape and the count is
affected) or **theoretical** (the shape is absent from this corpus, so the limit
is latent). Charter is this-corpus accuracy, so live ones matter most.

- **lim-1. Multi-line `$( … )` command substitution — LIVE (1 instance).** The
  scanner's substitution-depth (`W_sp`) is per-logical-line and resets each
  physical line, but a `$(` whose body spans physical lines via an embedded
  heredoc is NOT joined into one logical line (heredoc-skip and quote-join are,
  but cmdsub-across-lines is not). Concretely: line 196 `block_body=$(cat <<'EOF'`
  … `)` at line 237. Effect: the `block_body` assignment is the one assignment-
  shaped line the tool misses (`plain assign`=45 vs a true ~46); the inner `cat`
  and the `$(..) cmdsub` ARE still counted (line 196), and the orphan `)` at 237
  produces no spurious token. ~SUSPECT this is the single largest correctness gap
  on this corpus, and it is small.

- **lim-2. Line attribution on joined logical lines — LIVE (cosmetic).** When a
  logical line spans physical lines (continuations, multi-line quotes, the
  joined awk program at 453–463), tokens after the first physical line are
  attributed to the logical-line's *start* line, not their exact physical line.
  E.g. `$file`/`$tmp` and the `> "$tmp"` on physical line 463 are reported at 453.
  Counts are right; the line pointer is coarse. +SURE.

- **lim-3. Multi-line quoted strings — handled, but by a heuristic — residual
  risk theoretical here.** I added quote-state continuation so the embedded awk
  program (single-quoted, lines 453–463) scans as one quoted argument instead of
  leaking `print`/`!done`/`/^COMMIT/` as commands (it did, in an earlier version:
  8 false tokens). The corpus has exactly one multi-line quoted span and it is
  now correct. The heuristic (`tail_quote_state`) re-derives quote state per
  candidate line; pathological mixes of quotes-inside-comments-inside-continued-
  lines could still fool it, but none exist here. ~SUSPECT.

- **lim-4. `name$(...)` / `foo``bar`` ` concatenation splits the word —
  theoretical.** Outside quotes, a command word glued to a substitution (e.g.
  `foo$(bar)`) is flushed as `foo` plus a separate inner context rather than one
  word. Doesn't change command *classification* and the corpus has no such glued
  command-position case, so count-impact here is nil. -GUESS it could matter for
  argument-shaped concatenations elsewhere.

- **lim-5. Builtin/keyword list is fixed, not shell-introspected — theoretical.**
  Classification of `builtin` vs `external` uses a hard-coded dash builtin set.
  A real binary that shadows a builtin name (or a dash builtin I omitted) would
  be mis-classed. The corpus uses only common, unambiguous names. +SURE for this
  corpus; a caller running it on arbitrary sh should sanity-check the list.

- **lim-6. `[`/`test` operator attribution is positional, not bracket-matched —
  theoretical-ish.** Operators (`-f`, `=`, `-eq`, …) are counted as test
  operators only while a `[`/`test` command is "open" on the line (until `]` or
  end of line). Nested or `]`-less malformed tests, or these tokens appearing as
  ordinary arguments to a non-test command, could mis-count. The corpus's tests
  are all well-formed single-line `[ … ]`; spot-checked correct. ~SUSPECT.

- **lim-7. Heredoc opener detection trusts the scanner's tokenizer — fine here.**
  `<<WORD` / `<<-WORD` / `<<'WORD'` / `<<"WORD"` are detected mid-scan (so a `<<`
  inside a string/comment is not mistaken for an opener), several per opener line
  (the `done <<EOF` and `cat > f <<EOF` forms both work). `<<<` (bash here-string,
  not POSIX) is correctly NOT treated as a heredoc. No `<<-` in the corpus
  (count 0 is real). +SURE.

- **lim-8. No alias/`.`-source expansion, no real word-splitting/globbing —
  by design.** The tool counts *textual* command positions, not a resolved
  execution graph. `glob char`/`tilde candidate` are syntactic candidate-flags
  (unquoted `*?[`/leading `~`), not "this actually globbed". Both are 0 here.

### Cross-cutting judgment calls (flagged, not resolved — tc-style)

- **tc-1.** A `>/dev/null` is counted in *both* `redirect >` and `redirect
  to-null` (one physical redirect, two construct classes). Kept as double-
  representation because the spec lists them as separate classes; a consumer
  summing "all redirects" must not add these two columns naively.
- **tc-2.** `[` is counted as a `builtin` command *and* its operators feed the
  `test [ ] *` constructs. Intentional, but means `[` (9) and the test-operator
  rows are correlated, not independent.
- **tc-3.** The `func def` construct is counted per definition (4) while the
  function-name commands table shows *calls* only; the definition line itself is
  not a "call". A reader wanting "definitions vs calls" should read `func def`
  (4) and `func call` (24) together, not the commands table alone.
- **tc-4.** Multi-character `set` flags are bucketed: `-eu`/`-ue` → `set -eu`;
  any single `-…e…` → `set -e`; `-…u…` → `set -u`. `set -o errexit` long-form is
  NOT recognized (absent here). The corpus's lone `set -eu` (line 24) is right.
