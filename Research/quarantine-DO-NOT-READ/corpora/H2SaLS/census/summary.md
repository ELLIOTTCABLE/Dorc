# Census summary

> LLM-generated; part of an intentionally quality-varied artificial testing
> corpus/tooling for a static-analysis project (Dorc); not production tooling;
> an artificial corpus cannot expose the truth of real-world ops-code.

Inputs: ../H2SaLS/harden.sh

| metric | value |
|---|---|
| total command tokens | 174 |
| distinct external commands | 33 |
| total construct instances | 295 |

## Command tokens by class

| class | tokens |
|---|---|
| external | 107 |
| builtin | 43 |
| function | 24 |

## Top 20 commands (all classes)

| command | class | count |
|---|---|---|
| printf | builtin | 15 |
| grep | external | 14 |
| apt-get | external | 12 |
| set_sshd_line | function | 12 |
| sed | external | 11 |
| [ | builtin | 9 |
| set_conf | function | 9 |
| cat | external | 8 |
| ufw | external | 8 |
| service | external | 6 |
| true | builtin | 5 |
| chmod | external | 4 |
| cp | external | 4 |
| mktemp | external | 4 |
| rm | external | 4 |
| echo | builtin | 3 |
| getent | external | 3 |
| : | builtin | 2 |
| add_psad_logging | function | 2 |
| awk | external | 2 |

## Constructs (count desc; zeros included)

| construct | count |
|---|---|
| param $VAR plain | 96 |
| plain assign | 45 |
| func call | 24 |
| if/elif | 23 |
| redirect > | 14 |
| $(..) cmdsub | 11 |
| append >> | 11 |
| param positional $1..$9 | 10 |
| || or-list | 8 |
| heredoc unquoted | 8 |
| ! negation | 6 |
| pipe stage | 6 |
| test [ ] numeric | 5 |
| func def | 4 |
| brace group { } | 3 |
| fd-dup (N>&M) | 3 |
| redirect to-null | 3 |
| test [ ] file | 3 |
| && and-list | 2 |
| for | 2 |
| heredoc quoted | 2 |
| test [ ] string | 2 |
| while | 2 |
| case | 1 |
| set -eu | 1 |
| arith $((..)) | 0 |
| backtick cmdsub | 0 |
| command -v | 0 |
| env-prefix assign | 0 |
| eval | 0 |
| exec | 0 |
| glob char (unquoted) | 0 |
| heredoc <<- | 0 |
| local | 0 |
| param $! | 0 |
| param $# | 0 |
| param $$ | 0 |
| param $* | 0 |
| param $? | 0 |
| param $@ | 0 |
| param ${..#prefix} | 0 |
| param ${..%suffix} | 0 |
| param ${..:=assign} | 0 |
| param ${..:-default} | 0 |
| param ${..} braced | 0 |
| redirect < | 0 |
| set -e | 0 |
| set -u | 0 |
| subshell (..) | 0 |
| tilde candidate | 0 |
| trap | 0 |
| until | 0 |
