#!/bin/sh
# ============================================================================
# LLM-GENERATED. Part of an intentionally quality-varied artificial testing
# corpus/tooling for a static-analysis project (Dorc). NOT production tooling.
# An artificial corpus cannot expose the truth of real-world ops-code.
#
# census.sh -- a MECHANICAL command/construct census over POSIX-sh corpora.
# It READS its inputs AS TEXT ONLY (awk). It never sources, evals, or executes
# any input file, or any fragment of one. The frozen-evidence corpora it
# targets must never be run; this tool only counts tokens in their text.
#
# Usage:
#   sh census.sh FILE...            # writes the three reports to stdout
#   sh census.sh -o DIR FILE...     # writes commands.tsv, constructs.tsv and
#                                   # summary.md into DIR instead
#
# Multiple input FILEs aggregate. The TSV tables carry a leading `file` column,
# so per-file provenance survives aggregation; summary.md reports cross-file
# totals.
#
# Method (see Research/notes/1A7-census-tool.md for the full method note and an
# honest KNOWN-LIMITATIONS section): a single awk program reads every input
# line into memory, then makes two in-memory passes. Pass 1 collects function
# definitions so that calls to them classify as `function`. Pass 2 runs a
# character-level scanner that approximates a shell lexer: it tracks single and
# double quotes, comments, line-continuations, heredocs (quoted/unquoted
# delimiters, several per opener line, `<<-` tab-stripping), `$(...)`/backtick
# command substitution with nesting, and command position. It is NOT a parser.
# ============================================================================

set -eu

OUT_DIR=''
while [ $# -gt 0 ]; do
   case $1 in
      -o)
         [ $# -ge 2 ] || { echo 'census.sh: -o needs a DIR argument' >&2; exit 2; }
         OUT_DIR=$2
         shift 2
         ;;
      -o*)
         OUT_DIR=${1#-o}
         shift
         ;;
      --)
         shift
         break
         ;;
      -*)
         echo "census.sh: unknown option: $1" >&2
         exit 2
         ;;
      *)
         break
         ;;
   esac
done

[ $# -ge 1 ] || { echo 'census.sh: need at least one input FILE' >&2; exit 2; }
for f in "$@"; do
   [ -f "$f" ] || { echo "census.sh: not a readable file: $f" >&2; exit 2; }
done
[ -z "$OUT_DIR" ] || mkdir -p "$OUT_DIR"

INPUTS="$*"          # captured for the summary header (the renderers take no args)

# ----------------------------------------------------------------------------
# The awk program is a single self-contained string. It NEVER shells out and
# reads input only as text. Character literals that would fight shell quoting
# are written with gawk hex escapes: \x27 = single quote, \x22 = double quote,
# \x60 = backtick. This keeps the whole program inside one sh single-quoted
# heredoc with no nested-quote juggling.
#
# Output: a tab-separated record stream on stdout, one record per line, tagged
# in column 1:
#   CMD <file> <name> <class> <count> <comma-separated-lines>
#   CON <file> <construct> <count> <comma-separated-lines>
#   ERR <file> <message>
# The sh wrapper renders this stream into the requested report(s).
# ----------------------------------------------------------------------------
AWK_PROG=$(cat <<'AWK_EOF'
# ====================== small helpers =======================================
function is_name(w)   { return (w ~ /^[A-Za-z_][A-Za-z0-9_]*$/) }
function is_assign(w) { return (w ~ /^[A-Za-z_][A-Za-z0-9_]*=/) }
function is_builtin(w){ return (w in BUILTIN) }

function note_cmd(name, ln,    k) {
   if (COLLECT_ONLY) return            # pass 1 only populates FUNC, emits nothing
   k = CURFILE SUBSEP name
   CMD_C[k]++
   CMD_L[k] = (CMD_L[k] == "") ? ln : CMD_L[k] "," ln
   CMD_F[k] = CURFILE; CMD_N[k] = name
}
function note_con(con, ln,    k) {
   if (COLLECT_ONLY) return
   k = CURFILE SUBSEP con
   CON_C[k]++
   CON_L[k] = (CON_L[k] == "") ? ln : CON_L[k] "," ln
   CON_F[k] = CURFILE; CON_N[k] = con
}
function decl_con(file, con,    k) {
   k = file SUBSEP con
   if (!(k in CON_C)) { CON_C[k] = 0; CON_F[k] = file; CON_N[k] = con; CON_L[k] = "" }
}
function declare_all(file,    i, a, n) {
   n = split(CONSTRUCTS, a, ",")
   for (i = 1; i <= n; i++) decl_con(file, a[i])
}

# ====================== command classification ==============================
function on_command(word, ln,    cls) {
   if (word == "" || word ~ /^[<>|&;()]/) return
   note_cmd(word, ln)
}

# ====================== parameter-expansion classifier ======================
# s = the text immediately AFTER a `$`. Counts the expansion form. Returns the
# number of characters of s consumed (so the scanner can advance i by that).
function classify_param(s, ln,    c, inner, depth, j, ch) {
   c = substr(s, 1, 1)
   if (c == "{") {
      inner = ""; depth = 1; j = 2
      while (j <= length(s)) {
         ch = substr(s, j, 1)
         if (ch == "{") depth++
         else if (ch == "}") { depth--; if (depth == 0) break }
         inner = inner ch; j++
      }
      note_con("param ${..} braced", ln)
      if (inner ~ /:-/)            note_con("param ${..:-default}", ln)
      else if (inner ~ /:=/)       note_con("param ${..:=assign}", ln)
      else if (inner ~ /^[^#%]+#/) note_con("param ${..#prefix}", ln)
      else if (inner ~ /^[^#%]+%/) note_con("param ${..%suffix}", ln)
      return j               # through the closing }
   }
   if (c ~ /^[0-9]$/) { note_con("param positional $1..$9", ln); return 1 }
   if (c == "@") { note_con("param $@", ln); return 1 }
   if (c == "*") { note_con("param $*", ln); return 1 }
   if (c == "#") { note_con("param $#", ln); return 1 }
   if (c == "?") { note_con("param $?", ln); return 1 }
   if (c == "$") { note_con("param $$", ln); return 1 }
   if (c == "!") { note_con("param $!", ln); return 1 }
   if (c ~ /^[A-Za-z_]$/) {
      note_con("param $VAR plain", ln)
      j = 1                  # swallow the whole NAME so it does not re-tokenize
      while (j < length(s) && substr(s, j + 1, 1) ~ /[A-Za-z0-9_]/) j++
      return j
   }
   return 0                  # a bare $ (e.g. end of line) -- not an expansion
}

# ====================== glob / tilde on an unquoted word ====================
function scan_features(word, had_quote, ln) {
   if (had_quote) return     # globbing/tilde only on fully-unquoted words
   if (word ~ /\*/ || word ~ /\?/ || word ~ /\[[^]]+\]/) note_con("glob char (unquoted)", ln)
   if (word ~ /^~/) note_con("tilde candidate", ln)
}

# ====================== the character scanner ===============================
# Scans ONE logical line. State machine with an explicit stack ST[] whose top
# (W_sp) is one of: "" (top level), "dq" (inside double quotes), "sub" (inside
# $(...) or backtick command substitution -- a fresh command-position context).
# Heredoc OPENERS push delimiters into HD_* for the driver to consume on the
# following physical lines. W_cmdpos is 1 when the next word starts a command.
#
# The scanner's working state is GLOBAL (W_* + ST) so the small helper end_word()
# can flush the pending word and resolve leading assignments without threading a
# dozen out-parameters through every operator branch.
function end_word(ln, sep,    rc) {
   # Flush the accumulated word, then maintain command-position and the leading-
   # assignment tally. sep=1 means a command separator (or end of line) follows.
   rc = flush(W_word, W_hadq, ln, W_cmdpos)
   if (W_sp == 0) {                       # assignment logic only at top level
      if (rc == 1) push_assign(ln)        # leading NAME=val: remember it
      else if (rc == 2) assigns_are("env-prefix assign")   # cmd followed -> env-prefix
   }
   # Clear command-position once a real word is consumed -- UNLESS the word was a
   # command-introducing keyword (rc==3: if/while/then/...), which keeps it so the
   # following word is still treated as a command.
   if (W_word != "" && !is_assign(W_word) && rc != 3) W_cmdpos = 0
   if (sep && W_sp == 0 && ASSIGN_N > 0) assigns_are("plain assign")  # nothing consumed them
   W_word = ""; W_hadq = 0
}

# Enter a $(...) / backtick command-substitution context. The enclosing word
# fragment (e.g. the `HOSTNAME ` in "HOSTNAME $(hostname -s)") is SAVED and the
# inner command starts with a fresh, command-position word; pop_sub() restores
# the outer fragment so the rest of the enclosing word keeps accumulating.
function push_sub(    ) {
   W_sp++
   ST[W_sp] = "sub"
   WSAVE[W_sp] = W_word; WSAVE_Q[W_sp] = W_hadq
   W_word = ""; W_hadq = 0
   W_cmdpos = 1
}
function pop_sub(    ) {
   ST[W_sp] = ""
   W_word = WSAVE[W_sp]; W_hadq = WSAVE_Q[W_sp]
   W_sp--
   W_cmdpos = 0
}

function scan(s, ln,    n, i, c, c2, j, d, q, strip, rest) {
   n = length(s)
   i = 1
   W_cmdpos = 1
   W_word = ""; W_hadq = 0
   W_prev = ""               # last significant char class for # and { detection
   W_sp = 0; ST[0] = ""

   while (i <= n) {
      c = substr(s, i, 1)

      # ----- inside double quotes ------------------------------------------
      if (ST[W_sp] == "dq") {
         if (c == "\x22") { W_sp--; W_prev = "q"; i++; continue }
         if (c == "\x5c") { W_word = W_word substr(s, i + 1, 1); i += 2; continue }
         if (c == "\x60") { note_con("backtick cmdsub", ln); push_sub(); i++; W_prev = "`"; continue }
         if (c == "$") {
            c2 = substr(s, i + 1, 1)
            if (c2 == "(") { note_con("$(..) cmdsub", ln); push_sub(); i += 2; W_prev = "("; continue }
            j = classify_param(substr(s, i + 1), ln)
            i += 1 + j
            W_word = W_word "$"        # keep word non-empty; the value is irrelevant
            continue
         }
         W_word = W_word c; i++; continue
      }

      # ----- comment: # at a word boundary, outside quotes -----------------
      if (c == "#" && W_word == "" && (W_prev == "" || W_prev ~ /[ \t;&|(){}<>]/)) break

      # ----- whitespace ----------------------------------------------------
      if (c == " " || c == "\t") { end_word(ln, 0); W_prev = " "; i++; continue }

      # ----- single-quoted span (fully literal) ----------------------------
      if (c == "\x27") {
         W_hadq = 1; i++
         while (i <= n && substr(s, i, 1) != "\x27") { W_word = W_word substr(s, i, 1); i++ }
         i++; W_prev = "q"; continue
      }

      # ----- open double-quote ---------------------------------------------
      if (c == "\x22") { W_hadq = 1; W_sp++; ST[W_sp] = "dq"; i++; W_prev = "q"; continue }

      # ----- backslash escape (outside quotes) -----------------------------
      if (c == "\x5c") { W_word = W_word substr(s, i + 1, 1); i += 2; W_prev = "w"; continue }

      # ----- dollar expansions (outside quotes) ----------------------------
      if (c == "$") {
         c2 = substr(s, i + 1, 1)
         if (c2 == "(" && substr(s, i + 2, 1) == "(") {
            note_con("arith $((..))", ln)
            i = skip_arith(s, i)
            W_prev = "w"; continue       # arithmetic value is part of a word
         }
         if (c2 == "(") {
            note_con("$(..) cmdsub", ln)
            end_word(ln, 0)              # close any preceding word at top level
            push_sub(); i += 2; W_prev = "("; continue
         }
         j = classify_param(substr(s, i + 1), ln)
         i += 1 + j
         W_word = W_word "$"
         W_prev = "w"; continue
      }

      # ----- backtick command substitution (outside quotes) ----------------
      if (c == "\x60") {
         # A backtick both opens and closes; toggle. Count only on OPEN.
         if (W_sp > 0 && ST[W_sp] == "sub") { end_word(ln, 0); pop_sub() }
         else { end_word(ln, 0); note_con("backtick cmdsub", ln); push_sub() }
         i++; W_prev = "`"; continue
      }

      # ----- close of $(...) -----------------------------------------------
      if (c == ")" && W_sp > 0 && ST[W_sp] == "sub") {
         end_word(ln, 0); pop_sub(); i++; W_prev = ")"; continue
      }

      # ----- heredoc opener: << or <<- (check before single <) -------------
      if (c == "<" && substr(s, i + 1, 1) == "<") {
         end_word(ln, 0)
         i += 2; strip = 0
         if (substr(s, i, 1) == "-") { strip = 1; i++ }
         while (i <= n && substr(s, i, 1) ~ /[ \t]/) i++
         d = ""; q = 0; c2 = substr(s, i, 1)
         if (c2 == "\x27") { q = 1; i++; while (i <= n && substr(s, i, 1) != "\x27") { d = d substr(s, i, 1); i++ }; i++ }
         else if (c2 == "\x22") { q = 1; i++; while (i <= n && substr(s, i, 1) != "\x22") { d = d substr(s, i, 1); i++ }; i++ }
         else if (c2 == "\x5c") { q = 1; i++; while (i <= n && substr(s, i, 1) ~ /[A-Za-z0-9_]/) { d = d substr(s, i, 1); i++ } }
         else { while (i <= n && substr(s, i, 1) ~ /[A-Za-z0-9_]/) { d = d substr(s, i, 1); i++ } }
         if (d != "") {
            HD_N++
            HD_DELIM[HD_N] = d; HD_STRIP[HD_N] = strip
            if (strip) note_con("heredoc <<-", ln)
            if (q) note_con("heredoc quoted", ln); else note_con("heredoc unquoted", ln)
         }
         W_cmdpos = 0; W_prev = "<"; continue
      }

      # ----- redirections ---------------------------------------------------
      if (c == ">" && substr(s, i + 1, 1) == "&") {           # fd-dup  N>&M / >&
         note_con("fd-dup (N>&M)", ln); end_word(ln, 0)
         i += 2; while (i <= n && substr(s, i, 1) ~ /[0-9-]/) i++
         W_cmdpos = 0; W_prev = ">"; continue
      }
      if (c == ">" && substr(s, i + 1, 1) == ">") {           # append >>
         note_con("append >>", ln); end_word(ln, 0)
         i += 2; rest = substr(s, i)
         if (rest ~ /^[ \t]*\/dev\/null/) note_con("redirect to-null", ln)
         W_cmdpos = 0; W_prev = ">"; continue
      }
      if (c == ">") {                                          # output redirect >
         note_con("redirect >", ln); end_word(ln, 0)
         i++; rest = substr(s, i)
         if (rest ~ /^[ \t]*\/dev\/null/) note_con("redirect to-null", ln)
         W_cmdpos = 0; W_prev = ">"; continue
      }
      if (c == "<") {                                          # input redirect <
         note_con("redirect <", ln); end_word(ln, 0)
         i++; W_cmdpos = 0; W_prev = "<"; continue
      }

      # ----- control operators ----------------------------------------------
      if (c == "|" && substr(s, i + 1, 1) == "|") {
         note_con("|| or-list", ln); end_word(ln, 1)
         W_cmdpos = 1; i += 2; W_prev = "|"; continue
      }
      if (c == "|") {
         note_con("pipe stage", ln); end_word(ln, 1)
         W_cmdpos = 1; i++; W_prev = "|"; continue
      }
      if (c == "&" && substr(s, i + 1, 1) == "&") {
         note_con("&& and-list", ln); end_word(ln, 1)
         W_cmdpos = 1; i += 2; W_prev = "&"; continue
      }
      if (c == "&") {
         end_word(ln, 1)
         W_cmdpos = 1; i++; W_prev = "&"; continue
      }
      if (c == ";") {
         if (substr(s, i + 1, 1) == ";") {        # ;; -> case body ended
            end_word(ln, 1)
            if (CASE_DEPTH > 0) CASE_STATE = "pat"
            W_cmdpos = 1; i += 2; W_prev = ";"; continue
         }
         end_word(ln, 1)
         W_cmdpos = 1; i++; W_prev = ";"; continue
      }

      # ----- subshell ( and brace group { } --------------------------------
      if (c == "(" && W_word == "") {
         note_con("subshell (..)", ln)
         W_cmdpos = 1; i++; W_prev = "("; continue
      }
      if (c == ")") {
         if (CASE_DEPTH > 0 && CASE_STATE == "pat") {   # close a case pattern
            CASE_STATE = "body"; W_word = ""; W_hadq = 0
            W_cmdpos = 1; i++; W_prev = ")"; continue
         }
         end_word(ln, 1); W_cmdpos = 0; i++; W_prev = ")"; continue
      }
      if (c == "{" && W_word == "" && (W_prev == "" || W_prev ~ /[ \t;&|()]/)) {
         note_con("brace group { }", ln)
         W_cmdpos = 1; i++; W_prev = "{"; continue
      }
      if (c == "}" && W_word == "") {
         W_cmdpos = 1; i++; W_prev = "}"; continue
      }

      # ----- ordinary word character ---------------------------------------
      W_word = W_word c; W_prev = "w"; i++
   }

   end_word(ln, 1)        # trailing word + plain-assign resolution at end of line
}

# Skip an arithmetic $(( ... )) beginning at the `$`; return index past `))`.
function skip_arith(s, i,    n, depth) {
   n = length(s); i += 3; depth = 2
   while (i <= n && depth > 0) {
      c = substr(s, i, 1)
      if (c == "(") depth++
      else if (c == ")") depth--
      i++
   }
   return i
}

# Resolve a completed word into command / keyword / assignment / test-operator /
# argument and emit the right constructs. RETURN CODE (used by end_word to set
# command-position and classify leading assignments): 1 = leading NAME=val
# assignment; 2 = a command word; 3 = a command-INTRODUCING keyword (next word is
# still a command); 0 = anything else (for/case/in keyword, argument, operator).
function flush(word, hadq, ln, cmdpos) {
   if (word == "") return 0

   # (Function definitions are detected and stripped line-anchored in the pass-2
   # driver before scan() runs, so no `name()` word reaches here on a def line.)

   if (cmdpos) {
      if (is_assign(word)) return 1           # leading NAME=val; scan() tracks it

      if (word == "if" || word == "elif") { note_con("if/elif", ln); return 3 }
      if (word == "while") { note_con("while", ln); return 3 }
      if (word == "until") { note_con("until", ln); return 3 }
      if (word == "for")   { note_con("for", ln); return 0 }
      if (word == "case")  { note_con("case", ln); CASE_DEPTH++; CASE_STATE = "pat"; return 0 }
      if (word == "esac")  { if (CASE_DEPTH > 0) CASE_DEPTH--; return 0 }
      if (word == "then" || word == "else" || word == "do") return 3
      if (word == "fi" || word == "done" || word == "in") return 0
      if (word == "!")     { note_con("! negation", ln); return 3 }

      if (word in FUNC) note_con("func call", ln)
      on_command(word, ln)

      if (word == "[")       TEST_DEPTH++
      if (word == "test")    TEST_DEPTH++       # `test ...` uses same operators
      if (word == "trap")    note_con("trap", ln)
      if (word == "local")   note_con("local", ln)
      if (word == "exec")    note_con("exec", ln)
      if (word == "eval")  { note_con("eval", ln); EVAL_SEEN[CURFILE] = 1 }
      if (word == "set")     PENDING_SET = 1
      if (word == "command") PENDING_CMDV = 1
      return 2
   }

   # ---- NON command position: an argument / operand ----
   if (PENDING_SET == 1) {
      if      (word == "-eu" || word == "-ue") note_con("set -eu", ln)
      else if (word ~ /^-[a-z]*$/ && word ~ /e/ && word ~ /u/) note_con("set -eu", ln)
      else if (word ~ /^-[a-z]*$/ && word ~ /e/) note_con("set -e", ln)
      else if (word ~ /^-[a-z]*$/ && word ~ /u/) note_con("set -u", ln)
      PENDING_SET = 0
   }
   if (PENDING_CMDV == 1) {
      if (word == "-v") { note_con("command -v", ln); PENDING_CMDV = 2; return 0 }
      PENDING_CMDV = 0
   } else if (PENDING_CMDV == 2) {           # swallow the guarded name
      PENDING_CMDV = 0; return 0
   }

   if (TEST_DEPTH > 0) {
      if (word == "]") { TEST_DEPTH--; return 0 }
      if (word ~ /^-[fedrwxsLhpbcgkuSOGN]$/) { note_con("test [ ] file", ln); return 0 }
      if (word == "=" || word == "!=" || word == "-z" || word == "-n") { note_con("test [ ] string", ln); return 0 }
      if (word ~ /^-(eq|ne|lt|le|gt|ge)$/) { note_con("test [ ] numeric", ln); return 0 }
   }

   scan_features(word, hadq, ln)
   return 0
}

# Record one pending leading assignment (at sp==0). Resolved by scan() into
# env-prefix (a command word followed) or plain (a separator/EOL followed).
function push_assign(ln) { ASSIGN_N++; ASSIGN_LINE[ASSIGN_N] = ln }
function assigns_are(kind,    j) {
   for (j = 1; j <= ASSIGN_N; j++) note_con(kind, ASSIGN_LINE[j])
   ASSIGN_N = 0
}

# ====================== driver: read all, two passes ========================
BEGIN {
   SUBSEP = "\034"
   # gawk hex escapes are reliable in STRING literals but NOT inside /regex/
   # literals (in a regex, \x5c is a parse error). Quote/backslash chars are
   # therefore compared as STRING literals (SQ/DQ/BS) in the scanner. RE_TAILBS
   # is a DYNAMIC regex matching one trailing backslash: in a dynamic regex one
   # backslash is an escape, so matching a literal backslash needs two.
   SQ = "\x27"; DQ = "\x22"; BS = "\x5c"
   RE_TAILBS = BS BS "$"
   CONSTRUCTS = "func def,func call,$(..) cmdsub,backtick cmdsub,pipe stage,heredoc unquoted,heredoc quoted,heredoc <<-,redirect >,append >>,redirect <,fd-dup (N>&M),redirect to-null,set -e,set -u,set -eu,trap,case,for,while,until,if/elif,&& and-list,|| or-list,! negation,subshell (..),brace group { },test [ ] file,test [ ] string,test [ ] numeric,command -v,param $VAR plain,param ${..} braced,param ${..:-default},param ${..:=assign},param ${..#prefix},param ${..%suffix},param positional $1..$9,param $@,param $*,param $#,param $?,param $$,param $!,arith $((..)),glob char (unquoted),tilde candidate,local,eval,exec,env-prefix assign,plain assign"
   nb = split("[ test printf echo set export read cd return exit shift trap eval exec local unset : true false pwd umask wait jobs kill getopts hash type command alias unalias readonly times break continue . source", b, " ")
   for (i = 1; i <= nb; i++) BUILTIN[b[i]] = 1
}

# Store every physical line in memory, partitioned by file, preserving order.
{
   if (FNR == 1) { NFILES++; ORDER[NFILES] = FILENAME }
   LN[FILENAME, FNR] = $0
   NLINES[FILENAME] = FNR
}

END {
   for (fx = 1; fx <= NFILES; fx++) {
      file = ORDER[fx]
      CURFILE = file
      declare_all(file)
      # Pass 1 (collect_only=1) populates FUNC using the SAME scanner and heredoc
      # handling as the census, so forward-referenced calls classify correctly
      # and the two passes can never disagree on what is heredoc body. note_* are
      # suppressed during pass 1. Pass 2 (collect_only=0) emits the census.
      run_pass(file, 1)
      run_pass(file, 0)
   }
   emit()
}

# One pass over a file's stored lines. Assembles logical lines (heredoc bodies
# skipped, line-continuations joined), detects+strips function definitions, and
# runs the scanner. collect_only=1 records only FUNC (no construct/command
# output); collect_only=0 emits the full census.
function run_pass(file, collect_only,    k, raw, bs, stripped, cont, cont_ln, logical, useln, nm) {
   COLLECT_ONLY = collect_only
   reset_heredoc()
   reset_line_state()
   cont = ""; cont_ln = 0
   for (k = 1; k <= NLINES[file]; k++) {
      raw = LN[file, k]
      if (consume_heredoc(raw)) continue

      # line continuation: an odd count of trailing backslashes escapes newline
      bs = count_trailing_bs(raw)
      if (bs % 2 == 1) {
         stripped = raw; sub(RE_TAILBS, "", stripped)
         if (cont == "") cont_ln = k
         cont = cont stripped
         continue
      }
      if (cont != "") { logical = cont raw; useln = cont_ln; cont = "" }
      else            { logical = raw;        useln = k }

      # Multi-line quoted string: if the logical line ends inside a '...' or
      # "..." (e.g. an embedded awk program), pull in following physical lines --
      # joined with a real newline -- until the quote closes, so their contents
      # scan as a quoted argument rather than as commands. These joined lines are
      # part of the string, so they are NOT tested for heredoc bodies.
      while (tail_quote_state(logical) != "" && k < NLINES[file]) {
         k++
         logical = logical "\n" LN[file, k]
      }

      # Function definition (both `name()` and `name ()` forms), line-anchored.
      # Record FUNC + the func-def construct, then strip the `name () {` prefix
      # (including a same-line body brace) so the scanner neither emits the name
      # as a command nor counts the body brace as a brace group.
      if (match(logical, /^[ \t]*[A-Za-z_][A-Za-z0-9_]*[ \t]*\(\)[ \t]*\{?[ \t]*/)) {
         nm = logical
         sub(/^[ \t]*/, "", nm); sub(/[ \t]*\(\).*/, "", nm)
         if (is_name(nm)) FUNC[nm] = 1
         note_con("func def", useln)
         logical = substr(logical, RSTART + RLENGTH)
      }

      # Per-command states must not bleed across physical lines (an unclosed `[`,
      # a dangling `set`/`command -v`, or unconsumed leading assignments are all
      # malformed if they cross a line). case/for/while DO span lines, so CASE_*
      # is intentionally preserved across iterations here.
      TEST_DEPTH = 0; PENDING_SET = 0; PENDING_CMDV = 0; ASSIGN_N = 0

      HD_N = 0                                 # heredoc openers queued by THIS line
      scan(logical, useln)
      if (HD_N > 0) { in_hd = 1; hd_idx = 1; hd_pending = HD_N }
   }
}

# ---- heredoc helpers shared by both passes ----
function reset_heredoc() { in_hd = 0; hd_idx = 0; hd_pending = 0; HD_N = 0; delete HD_DELIM; delete HD_STRIP }
function reset_line_state() {
   CASE_DEPTH = 0; CASE_STATE = ""; TEST_DEPTH = 0
   PENDING_SET = 0; PENDING_CMDV = 0; ASSIGN_N = 0
}

# If currently inside a heredoc body, consume this line; return 1 if consumed.
function consume_heredoc(raw,    chk, term) {
   if (!in_hd) return 0
   term = HD_DELIM[hd_idx]
   chk = raw
   if (HD_STRIP[hd_idx]) sub(/^\t+/, "", chk)
   if (chk == term) {
      hd_idx++
      if (hd_idx > hd_pending) in_hd = 0
   }
   return 1
}

function count_trailing_bs(s,    n) {
   n = 0
   while (s ~ RE_TAILBS) { n++; sub(RE_TAILBS, "", s) }
   return n
}

# Lightweight quote/comment tracker: returns the quote state at the END of s,
# one of "" (balanced), "sq" (single-quote left open), "dq" (double-quote left
# open). Used by the driver to join MULTI-LINE quoted strings into one logical
# line BEFORE scanning, so command-like text inside a multi-line '...' (e.g. an
# embedded awk program) is treated as a quoted argument, not commands. Mirrors
# the scanner's rules: # starts a comment only at a word boundary outside
# quotes; backslash escapes the next char outside single quotes.
function tail_quote_state(s,    n, i, c, st, atword) {
   n = length(s); i = 1; st = ""; atword = 1   # atword: are we at a word boundary?
   while (i <= n) {
      c = substr(s, i, 1)
      if (st == "sq") { if (c == "\x27") st = ""; i++; continue }
      if (st == "dq") {
         if (c == "\x22") st = ""
         else if (c == "\x5c") i++          # skip escaped char inside double quote
         i++; continue
      }
      if (c == "#" && atword) return ""      # comment to EOL: line ends balanced
      if (c == "\x5c") { i += 2; atword = 0; continue }   # escaped char (incl \" \')
      if (c == "\x27") { st = "sq"; i++; atword = 0; continue }
      if (c == "\x22") { st = "dq"; i++; atword = 0; continue }
      atword = (c ~ /[ \t;&|(){}<>]/) ? 1 : 0
      i++
   }
   return st
}

# ====================== emit the record stream ==============================
function emit(    key, nm, cls, f) {
   for (key in CMD_C) {
      nm = CMD_N[key]; f = CMD_F[key]
      cls = (nm in FUNC) ? "function" : ((nm in BUILTIN) ? "builtin" : "external")
      printf "CMD\t%s\t%s\t%s\t%d\t%s\n", f, nm, cls, CMD_C[key], CMD_L[key]
   }
   for (key in CON_C)
      printf "CON\t%s\t%s\t%d\t%s\n", CON_F[key], CON_N[key], CON_C[key], CON_L[key]
   for (f in EVAL_SEEN)
      printf "ERR\t%s\t%s\n", f, "eval present (flagged loudly)"
}
AWK_EOF
)

STREAM=$(awk "$AWK_PROG" "$@")

TAB=$(printf '\t')

# ----------------------------------------------------------------------------
# Renderers. Each consumes $STREAM (the record stream) and emits one report.
# Sorting: commands by count desc; constructs by count desc; both stable on name.
# ----------------------------------------------------------------------------
render_commands_tsv() {
   printf 'file\tcommand\tclass\tcount\tlines\n'
   printf '%s\n' "$STREAM" \
      | awk -F'\t' '$1=="CMD"{print $2"\t"$3"\t"$4"\t"$5"\t"$6}' \
      | sort -t"$TAB" -k4,4nr -k1,1 -k2,2
}

render_constructs_tsv() {
   printf 'file\tconstruct\tcount\tlines\n'
   printf '%s\n' "$STREAM" \
      | awk -F'\t' '$1=="CON"{print $2"\t"$3"\t"$4"\t"$5}' \
      | sort -t"$TAB" -k3,3nr -k2,2
}

render_summary_md() {
   total_cmd=$(printf '%s\n' "$STREAM" | awk -F'\t' '$1=="CMD"{s+=$5} END{print s+0}')
   total_con=$(printf '%s\n' "$STREAM" | awk -F'\t' '$1=="CON"{s+=$4} END{print s+0}')
   distinct_ext=$(printf '%s\n' "$STREAM" \
      | awk -F'\t' '$1=="CMD" && $4=="external"{print $3}' | sort -u | wc -l | tr -d ' ')

   printf '# Census summary\n\n'
   printf '> LLM-generated; part of an intentionally quality-varied artificial testing\n'
   printf '> corpus/tooling for a static-analysis project (Dorc); not production tooling;\n'
   printf '> an artificial corpus cannot expose the truth of real-world ops-code.\n\n'
   printf 'Inputs: %s\n\n' "$INPUTS"
   printf '| metric | value |\n|---|---|\n'
   printf '| total command tokens | %s |\n' "$total_cmd"
   printf '| distinct external commands | %s |\n' "$distinct_ext"
   printf '| total construct instances | %s |\n\n' "$total_con"

   printf '## Command tokens by class\n\n| class | tokens |\n|---|---|\n'
   for cls in external builtin function; do
      c=$(printf '%s\n' "$STREAM" | awk -F'\t' -v k="$cls" '$1=="CMD" && $4==k{s+=$5} END{print s+0}')
      printf '| %s | %s |\n' "$cls" "$c"
   done
   printf '\n## Top 20 commands (all classes)\n\n| command | class | count |\n|---|---|---|\n'
   printf '%s\n' "$STREAM" \
      | awk -F'\t' '$1=="CMD"{c[$3]+=$5; cls[$3]=$4} END{for(k in c) printf "%s\t%s\t%d\n", k, cls[k], c[k]}' \
      | sort -t"$TAB" -k3,3nr -k1,1 | head -20 \
      | awk -F'\t' '{printf "| %s | %s | %s |\n", $1, $2, $3}'

   printf '\n## Constructs (count desc; zeros included)\n\n| construct | count |\n|---|---|\n'
   printf '%s\n' "$STREAM" \
      | awk -F'\t' '$1=="CON"{c[$3]+=$4} END{for(k in c) printf "%s\t%d\n", k, c[k]}' \
      | sort -t"$TAB" -k2,2nr -k1,1 \
      | awk -F'\t' '{printf "| %s | %s |\n", $1, $2}'

   if printf '%s\n' "$STREAM" | awk -F'\t' '$1=="ERR"{f=1} END{exit !f}'; then
      printf '\n## Loud diagnostics\n\n'
      printf '%s\n' "$STREAM" | awk -F'\t' '$1=="ERR"{printf "- %s: %s\n", $2, $3}'
   fi
}

if [ -n "$OUT_DIR" ]; then
   render_commands_tsv   > "$OUT_DIR/commands.tsv"
   render_constructs_tsv > "$OUT_DIR/constructs.tsv"
   render_summary_md     > "$OUT_DIR/summary.md"
   echo "census.sh: wrote commands.tsv, constructs.tsv, summary.md to $OUT_DIR" >&2
else
   render_summary_md
   printf '\n----- commands.tsv -----\n'; render_commands_tsv
   printf '\n----- constructs.tsv -----\n'; render_constructs_tsv
fi
