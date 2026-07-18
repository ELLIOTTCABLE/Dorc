# The shell dialect

Everything you write in an oracle is shell, and this page pins down exactly which
shell, and how to write it well. The strictness has one motivation, and it is not
purity: oracles are the shared artifact. Your oracle will be run by Dorc on hosts
whose `/bin/sh` you do not control, stripped into a plain library and pasted into
consumers' own scripts, and read by people who daily-drive bash, zsh, or whatever
their distro shipped. An oracle written in your favorite shell's dialect reaches
only people with your tastes; one written in the portable core reaches everyone,
forever. Books - your own private run-this-machine scripts - are a looser story
still being worked out. Oracles are strict, on purpose, and the target is simply
stated: write good portable shell, plus `local`.

## The definition is executable

There is no thousand-page spec to memorize. The dialect's baseline is defined by
two small, real shells: a valid oracle, after Dorc strips its annotations, parses
and runs identically under both `dash` and `posh` (currently pinned at 0.5.12 and
0.14.1). Where those two disagree, the construct is outside the dialect - and
that sentence is a test you can literally run. Both shells install in seconds;
running your stripped oracle under each, before publishing, is the conformance
check. Two mechanical linters catch most drift before you get that far:
`checkbashisms` (built for exactly this dialect) and `shellcheck` (broader; its
wiki page for each finding is the best shell education on the internet, and these
docs assume you will lean on it).

This is a deliberately well-trodden target. It is, nearly exactly, what Debian
has required of maintainer scripts for twenty-five years, so an enormous corpus
of working code and institutional advice already exists for it. You are not
learning a Dorc dialect; you are learning portable shell, a durable skill these
docs merely put to use.

## The rulings

What follows are the fixed decisions, each with its reason - most matter because
your code must mean the same thing on every shell that will ever run or host it.

Quoting is law, not style. Every expansion is quoted - `"$1"`, `"$dest"`,
`"$(cmd)"` - unless you can say aloud why this one must not be. Unquoted
expansions word-split and glob on some shells and not others (zsh inverts the
default), which makes the same bytes mean different things for different
consumers; quoting is what makes your file one program instead of several. This
single habit does more for oracle quality than everything else on the page.

Print with `printf`, never with `echo`-plus-anything. `echo` with flags or with
content containing backslashes is genuinely unspecified across implementations -
`echo "$var"` breaks the day `$var` starts with `-n` or contains a backslash.
`printf '%s\n' "$var"` says exactly what it means everywhere. Plain `echo` of a
known-literal word is tolerable; the moment a variable or an escape is involved,
it is `printf`.

`local` is in. It is the one deliberate addition above the POSIX core - defensive
functions need it, every shell in the care-set has it. One trap inherited with
it: `local x=$(cmd)` folds the command's exit status into the assignment, so a
failure cannot be seen. Where the status matters (and in a verdict body it
usually does), declare first, assign second: `local x; x=$(cmd) || return 2`.

Declare functions as `name() { ... }` only. The `function name {}` form is not
portable and is rejected.

The bash-family constructs are banned outright, because each has a portable
spelling: `[[ ]]` and `==` (use `[ ]` and `=`, or `case` for patterns),
`${x/pat/rep}` and `${x^^}` and `${x:off:len}` (use `${x#...}`, `${x%...}`,
`case`, or one small `sed`/`cut`), `<<<` herestrings (use a heredoc or a pipe),
`&>` and `|&` (spell the redirections out). Likewise `$'\n'` quoting is out for
now - `printf` covers those needs. Pattern matching without `[[` is one idiom
worth committing to memory, since it needs no external command at all:

```sh
fnmatch() { case "$2" in $1) return 0 ;; *) return 1 ;; esac ; }
```

Never write `test -a` or `test -o` (deprecated, ambiguous); connect separate `[`
commands with `&&` and `||`.

Never write `eval` in an oracle. Delegation is running an actual command;
generated code has no place in a body whose every line is a vouched claim. If a
tool truly forces dynamic construction on you, the honest home for that is the
escape hatch from the wrappers page, outside the licensed world.

Pipelines and `set -o pipefail`: the analyzer models pipeline status first-class,
and pipefail is legal in the dialect - but ancient shells lack it, and your
stripped file must not crash them. On any durable, paste-facing surface, use the
self-gating spelling, which is itself the check-then-act idiom this whole product
lifts:

```sh
(set -o pipefail 2>/dev/null) && set -o pipefail
```

And prefer pipe shapes that read their input fully (`grep x >/dev/null`) over
early-exit forms (`grep -q x`) when the producer minds a closed pipe; early-exit
consumers can race the producer into a meaningless status.

Avoid bare globs in oracle bodies. Beyond being host-dependent input, an
unmatched glob aborts outright under zsh's defaults - and your stripped file will
be sourced into zsh users' lives. Glob results you genuinely need should pass
through an existence check.

## Defensive habits for other people's machines

The dialect above keeps your file portable; these habits keep it correct against
hostile inputs - and every filename, package name, and operand your oracle
receives is hostile input, because it comes from books you have never read.

Reach for `"${1-}"` (and `"${2-}"`) when a positional may be absent. Bare `$2`
under `set -u` - which defensive books run - is a crash; the `-` default makes
absence an ordinary empty string your gates can test.

Put `--` before operands you pass to other commands (`foobar status -- "$dest"`),
and prefix relative paths with `./` when `--` is not honored. A filename is
allowed to begin with `-`, contain spaces, newlines, or glob characters, and
somebody's eventually will; `--` and quoting together close the whole class.
Never parse `ls` output; when you must walk files, `find path -exec cmd {} +` is
the robust shape.

Test for a command with `command -v foo >/dev/null 2>&1`, never `which`. Read
lines with `IFS= read -r line`. Skip `expr` entirely - parameter expansion and
`$(( ))` cover it. When parsing a tool's output where sort order or number
formats matter, pin the locale for that one command (`LC_ALL=C sort ...`), since
the host's locale is one more thing you do not control. And remember that `$( )`
strips all trailing newlines from its output - almost always what you want, worth
knowing the day it is not.

None of this is Dorc-specific, and that is the point: an oracle is just a very
well-behaved shell library. The canonical deeper sources, in rough order of
usefulness:

- the ShellCheck wiki - per-finding pages; read them, do not just silence
  codes;
- Rich Felker's "sh tricks" page - the classic on doing portable shell
  correctly;
- David Wheeler's essay on filenames in shell - the full horror and the full
  fix;
- the autoconf manual's "Portable Shell" chapter - decades of cross-platform
  scar tissue;
- the pure-sh-bible - a catalog of no-external-command idioms;
- shellhaters.org - a navigable map of the actual POSIX spec;
- and Greg's wiki - excellent, but bash-first; filter its advice through the
  dialect before importing it.

<!-- quoted: 276 rul-base-dialect-ruling-list, rul-spec-two-binary-floor,
     rul-pipefail-four-lanes; 278 section 1; spike/CLAUDE.md
     dialect-quality-law, two-binary-floor, emit-never-class; KNOBS kWHICHSH;
     etalabs sh_tricks; SC2155/SC2086 rationale -->
