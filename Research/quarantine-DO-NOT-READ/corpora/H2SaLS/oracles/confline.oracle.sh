# ============================================================================
# LLM-GENERATED ORACLE SEED — NOT REAL OPS CODE. Part of an intentionally
# quality-varied artificial testing corpus for the Dorc static-analysis
# project; it cannot expose the truth of real-world ops-code and must never be
# run. FROZEN EVIDENCE: the probe bodies below name real read-only commands
# (grep) but are NEVER executed, under any flag or fragment. Validation is
# `dash -n` plus reading. See Research/corpora/H2SaLS/README.md.
# ============================================================================
#
# confline — the GENERIC line-in-file end of the file-state spectrum: the
# entity is "this exact line is present in that file." Models the book's two
# dominant idioms (harden.sh `set_sshd_line` L166-178, `set_conf` L427-437):
#   grep -qxF "$line" "$file"   || printf '%s\n' "$line" >> "$file"   (append)
#   grep -q   "$re"   "$file"   && sed -i "s|$re.*|$line|" "$file"     (replace)
#
# THE IDENTITY PROBLEM (the central finding — see note 1A9 §confline). The
# entity is TWO-part: (path × line). But the annotation idiom binds ONE operand
# (`name : kind = "$1"`). DECISION: the PATH is the kind-annotated referent —
# it is the durable, cross-oracle-shareable identity (another oracle could name
# the same /etc/ssh/sshd_config); the line/pattern is a per-site DETAIL the
# resolver binds into the probe argv but does NOT mint a KindId for. The probe
# re-runs `grep -qxF <line> <path>`: a check OF the (path,line) pair, keyed in
# the engine on confline:<path> with the line carried as the probe's argument.
#
# REFUSALS are deliberate and broad here: grep's operand order varies wildly in
# the wild (`grep PATTERN FILE`, plus -q/-x/-F/-E/-v flag soup), and a
# mis-bound path is a wrong-entity elision (kFAIL-withhold). When the argv does
# not present exactly one pattern and one readable path, the resolver binds
# nothing and the site runs (sound). This kind honestly models only the
# narrow, anchored, metacharacter-free shapes this book actually writes.

oracle_kind=confline

# Probe: `grep -qxF <line> <path>` asks "is this EXACT line present?" — a
# read-only QUERY of confline:<path> (the append-idiom guard, L169/L111). `$1`
# is the PATH (the kind entity); `$2` is the line text (the per-site detail).
# THREE-OUTCOME HAZARD (recorded, not fixed): grep rc conflates 1=no-match with
# 2=tool-failure (unreadable file, bad regex). A two-outcome probe wrapper
# cannot distinguish "line absent" from "couldn't look" — see 1A9 um-file-2.
oracle_probe_confline() { grep -qxF -- "$2" "$1" >/dev/null 2>&1; }

# The `grep -q <re> <path>` substring/anchored variant (the replace-idiom
# guard, L172/L326/L431): present-by-pattern, not present-by-exact-line. A
# DISTINCT question (a `^Port ` pattern matches `Port 22` AND `Port 55899`),
# so it is a distinct selector — neither discharges the other.
oracle_probe_confline_pattern() { grep -q -- "$2" "$1" >/dev/null 2>&1; }

# WHICH PROVIDERS GET EFFECT LINES — the central breakdown (1A9 §confline):
# - grep IS a read-only Query of presence → `query present`. Sound.
# - printf/cat (the `>> "$file"` append) DO establish a line, but printf is not
#   a file-mutator command — it writes to stdout; the FILE mutation is the `>>`
#   REDIRECT, which is not a command token at all. There is NO provider token to
#   key an `establish` effect on. This is the deep reason file-state resists the
#   (provider,verb) model: the mutating "verb" is shell syntax, not a command.
# - sed -i (the replace) IS a real mutator command, but `sed` is GENERIC — the
#   same `sed -i` edits ANY file with ANY script; binding it to confline:<path>
#   would require the resolver to parse the `s|re|line|` argstring to know which
#   line, which the analyzer must NEVER do (it does not parse argstrings).
# So this kind declares ONLY the grep Query. The establish/kill side is
# UN-KEYABLE here and is left to fall out structurally (it does not). Recorded
# as the spectrum verdict's sharpest edge.
oracle_effect grep '' query present

# command-keyed check(): `grep [-qxF|-q] <pattern> <path>`. The engine's
# entity-resolver. Constrained-dialect argparse: strip leading flags, then bind
# the LAST operand as the path (kind entity) and the operand before it as the
# pattern detail. REFUSE (bind nothing ⇒ site runs) unless exactly two operands
# survive — `grep -rl pat dir1 dir2`, `grep pat` (stdin), piped greps all fall
# through to running, which is sound.
grep__check() {
   exact=0
   while [ "${1#-}" != "$1" ]; do
      case $1 in
         *x*) exact=1 ;;
      esac
      shift
   done
   pattern=$1; shift
   path : confline = "$1"; shift
   # refuse on a surviving extra operand (multi-file / multi-pattern grep): the
   # (path,line) identity is no longer single, so bind nothing and run.
   if [ "$1" = "" ] && [ "$path" != "" ] && [ "$pattern" != "" ]; then
      if [ "$exact" -eq 1 ]; then
         grep -qxF -- "$pattern" "$path" >/dev/null 2>&1
      else
         grep -q -- "$pattern" "$path" >/dev/null 2>&1
      fi
   fi
}
