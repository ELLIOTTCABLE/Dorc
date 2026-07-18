# LLM-GENERATED ORACLE SEED — intentionally quality-varied artificial testing
# corpus for a static-analysis project (Dorc). NOT real ops code; FROZEN EVIDENCE,
# NEVER EXECUTE. An artificial oracle cannot expose the truth of real-world ops-code.
# Validation is `dash -n` (parse-only) plus reading.
#
# Models the book's su-restriction (§1 L84): the play runs, unconditionally,
#   dpkg-statoverride --update --add root suusers 4750 /bin/su
# and tolerates ONLY the "already exists" failure by stderr-matching `*exist*` (L85).
#
# THE CLEANEST CHECK-THEN-ACT PAIR IN THE CORPUS:
#   - `dpkg-statoverride --add <user> <group> <mode> <path>`  → ESTABLISH
#       (an override on <path>). The man page is explicit: `--add` FAILS a sanity
#       check if an override for <path> already exists (that is the exact failure the
#       book's `*exist*` arm swallows). So `--add` is NON-idempotent by itself.
#   - `dpkg-statoverride --list [<path>]`                     → QUERY (read-only)
#       rc 0 if an override for <path> exists, rc 1 if none. This is the genuine
#       three-outcome read the book LACKS — it is the oracle-grounded REPLACEMENT for
#       the scrappy stderr-match. The entity is the PATH (`/bin/su`): the override is
#       keyed on the path; (user, group, mode) are its VALUE, not its identity.
#
# WHAT AN ORACLE-GROUNDED VERSION BUYS, AND ITS LIMIT (um-stat-1):
#   - With the `--list <path>` probe, a converged `--add` (override already present
#     for /bin/su) is detected up-front and elided — no need to RUN the mutator and
#     catch its failure. That is strictly better than the book's run-and-swallow.
#   - LIMIT: `--list <path>` confirms an override EXISTS for the path, NOT that it has
#     the desired (root, suusers, 4750). A path with a DIFFERENT override (wrong mode)
#     reads "present" ⇒ the `--add` is elided ⇒ the wrong mode persists. This matches
#     the book (its `*exist*` tolerance also does NOT correct a divergent existing
#     override — `--add` won't overwrite without `--force-statoverride-add`), so it is
#     faithful; but value-divergence is unmodelled. A value-exact probe would parse
#     `--list`'s columns (user/group/mode/path) and compare all three — a stdout-
#     content comparison, not a presence rc. Sketched, not built (presence-only here).
#
# REFUSES: this oracle declares NO `--remove` verb (the book has none); a `--remove`
# elided on a can't-tell `--list` would be a wrong deletion-elision (`inv-kfail`).
oracle_kind=statoverride
oracle_probe_statoverride() { dpkg-statoverride --list "$1" >/dev/null 2>&1; }
oracle_effect dpkg-statoverride add establish overridden

# command-keyed check(): `dpkg-statoverride <flags…> <user> <group> <mode> <path>`.
# The ACTION is spelled as a FLAG (`--add`/`--list`), not a positional verb, so the
# check DERIVES the verb by assignment (`verb=add`, flipped to `list` if `--list` is
# seen) — a legitimate dialect move (the engine reads whatever is bound to the name
# `verb`). `--update` is a modifier, stripped by the loop. For `--add`, the ENTITY is
# the PATH — the LAST of the four operands — so after the flag loop we shift past
# user/group/mode to land on the path and annotate it `statoverride`. `--list` is a
# read-only query the book never calls standalone and is given NO arm: it reaches no
# annotation ⇒ ⊤ ⇒ runs (a query never licenses elision anyway).
dpkg_statoverride__check() {
   verb=add
   while [ "${1#-}" != "$1" ]; do
      case $1 in
         --list) verb=list ;;
      esac
      shift
   done
   case $verb in
      add)
         shift
         shift
         shift
         path : statoverride = "$1"
         dpkg-statoverride --list "$path" >/dev/null 2>&1
         ;;
   esac
}
