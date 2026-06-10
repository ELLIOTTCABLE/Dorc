#!/bin/sh
# ============================================================================
# LLM-GENERATED. Part of an intentionally quality-varied artificial testing
# corpus/tooling for a static-analysis project (Dorc). NOT production tooling.
# An artificial corpus cannot expose the truth of real-world ops-code.
#
# INERT census fixture. Every command word here is a stub (hork / wombat /
# echo / true / : ) or a POSIX builtin -- NOTHING system-touching. Like the
# corpora it models, this file is NEVER EXECUTED; census-selftest.sh only READS
# it. Its purpose is to exercise every construct class census.sh counts, with
# hand-countable frequencies. Expected counts live in census-selftest.sh and
# were derived BY HAND from this file; if the tool disagrees, fix the tool.
#
# Traps for the scanner are deliberate: command-like text inside heredoc
# bodies, comments, and quotes MUST NOT be counted; `case` pattern labels are
# not commands; env-prefix vs plain assignment must be distinguished.
# ============================================================================

set -eu                                  # CON set -eu (1)

# ---- plain vs env-prefix assignments ----
FOO=1                                    # CON plain assign (1)   [FOO=1]
BAR=2                                    # CON plain assign (2)   [BAR=2]
BAZ=3 hork --go                          # CON env-prefix assign (1); CMD hork(ext)

# ---- function definitions (both forms) ----
alpha() {                                # CON func def (1); glued name()
    hork alpha-body                      # CMD hork (ext)
}
beta () {                                # CON func def (2); spaced name ()
    wombat beta-body                     # CMD wombat (ext)
}

# ---- function calls ----
alpha                                    # CON func call (1); alpha is a function
beta                                     # CON func call (2)

# ---- command substitution: $(...) and backtick ----
now=$(echo today)                        # CON $(..) cmdsub (1); CMD echo (builtin) inside; plain assign (3)
host=`hork hostname`                     # CON backtick cmdsub (1); CMD hork (ext) inside; plain assign (4)
nested=$(echo "$(true)")                 # CON $(..) cmdsub (2 AND 3); CMD echo(builtin), true(builtin); plain assign (5)

# ---- pipes (stages-1 per pipeline) ----
hork a | wombat b                        # CON pipe stage (1); CMD hork, wombat
hork a | wombat b | hork c               # CON pipe stage (2 AND 3); CMD hork x2, wombat

# ---- heredocs: unquoted, quoted, and <<- (tab-stripped) ----
# Body lines below contain command-looking text that MUST be ignored.
wombat <<EOF                             # CON heredoc unquoted (1); CMD wombat (ext)
this is hork not a command
$now is expanded but still body
EOF
wombat <<'EOF'                           # CON heredoc quoted (1); CMD wombat (ext)
literal hork wombat true here, all ignored
EOF
	wombat <<-EOF                         # CON heredoc <<- (1) AND heredoc unquoted (2); CMD wombat (ext)
	tab-stripped hork body ignored
	EOF

# ---- redirections ----
hork out > /tmp/x.out                    # CON redirect > (1); CMD hork (ext)
hork app >> /tmp/x.out                   # CON append >> (1); CMD hork (ext)
wombat < /tmp/x.in                       # CON redirect < (1); CMD wombat (ext)
hork noise 2>&1                          # CON fd-dup (1); CMD hork (ext)
hork quiet > /dev/null                   # CON redirect > (2) AND to-null (1); CMD hork
hork quiet2 2> /dev/null                 # CON redirect > (3) AND to-null (2); CMD hork (the 2> '>' counts as redirect)

# ---- trap ----
trap 'echo cleanup' EXIT                 # CON trap (1); CMD trap(builtin); echo inside single-quote is NOT a command

# ---- case (patterns are not commands) ----
case "$FOO" in                           # CON case (1)
    1) hork one ;;                       # pattern 1) -> body: CMD hork (ext)
    *) wombat other ;;                   # pattern *) -> body: CMD wombat (ext)
esac

# ---- for / while / until ----
for x in a b c; do                       # CON for (1)
    hork "$x"                            # CMD hork (ext)
done
n=0                                      # CON plain assign (6)
while [ "$n" -lt 3 ]; do                 # CON while (1); CON test numeric (1); CMD [ (builtin)
    n=$((n + 1))                         # CON arith $((..)) (1); CON plain assign (7)
done
until [ -f /tmp/done ]; do               # CON until (1); CON test file (1); CMD [ (builtin)
    wombat waiting                       # CMD wombat (ext)
    break                                # CMD break (builtin)
done

# ---- if / elif / else with various tests ----
if [ -d /tmp ]; then                     # CON if/elif (1); CON test file (2); CMD [ (builtin)
    hork dir-exists                      # CMD hork (ext)
elif [ "$FOO" = "1" ]; then              # CON if/elif (2); CON test string (1); CMD [ (builtin)
    wombat foo-one                       # CMD wombat (ext)
else
    true                                 # CMD true (builtin)
fi

# ---- && || ! ----
hork x && wombat y                       # CON && and-list (1); CMD hork, wombat
hork x || wombat z                       # CON || or-list (1); CMD hork, wombat
! hork fails                             # CON ! negation (1); CMD hork (ext)

# ---- subshell vs brace group ----
( hork sub )                             # CON subshell (1); CMD hork (ext)
{ wombat grp; }                          # CON brace group (1); CMD wombat (ext)

# ---- command -v existence guard (guarded name is NOT a command) ----
if command -v hork > /dev/null; then     # CON command -v (1); CON if/elif (3); CON redirect > (4) AND to-null (3); CMD command(builtin). 'hork' here is the guarded arg, NOT counted as a command.
    true                                 # CMD true (builtin)
fi

# ---- parameter expansions, one of each form ----
echo "$FOO"                              # CON param $VAR plain (1); CMD echo (builtin)
echo "${BAR}"                            # CON param braced (1); CMD echo (builtin)
echo "${BAZ:-def}"                       # CON param braced (2) AND :-default (1); CMD echo (builtin)
echo "${BAZ:=set}"                       # CON param braced (3) AND :=assign (1); CMD echo (builtin)
echo "${BAR#pre}"                        # CON param braced (4) AND #prefix (1); CMD echo (builtin)
echo "${BAR%suf}"                        # CON param braced (5) AND %suffix (1); CMD echo (builtin)
echo "$1 $2 $9"                          # CON param positional (1,2,3); CMD echo (builtin)
echo "$@ $* $# $? $$ $!"                 # CON param $@ (1), $* (1), $# (1), $? (1), $$ (1), $! (1); CMD echo (builtin)

# ---- glob + tilde candidates (unquoted only) ----
hork *.txt                               # CON glob char (1); CMD hork (ext)
wombat ~/data                            # CON tilde candidate (1); CMD wombat (ext)

# ---- local / exec ----
gamma() {                                # CON func def (3)
    local v=5                            # CON local (1) AND plain assign (8); CMD local (builtin)
    echo "$v"                            # CON param $VAR plain (2); CMD echo (builtin)
}
gamma                                    # CON func call (3)

# A bare colon and a couple of trailing builtins to round out counts.
:                                        # CMD : (builtin)
true                                     # CMD true (builtin)
false                                    # CMD false (builtin)
