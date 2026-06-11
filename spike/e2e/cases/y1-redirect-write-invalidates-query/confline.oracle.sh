# confline QUERY oracle: `grep -q PATTERN FILE` READS whether FILE contains PATTERN — a
# read-only QUERY of confline:<file>#present, mutating nothing (task-D2 `query` polarity).
# `grep` is external (shimmable), the file argument is the entity (the cell is keyed on
# the file the guard reads).
oracle_kind=confline
oracle_probe_confline() { grep -q x "$1" >/dev/null 2>&1; }
oracle_effect grep '' query present
# command-keyed check(): `grep -q PATTERN FILE` — strip the `-q` flag, annotate the FILE
# (operand 2, after the pattern) as `confline`; the probe re-greps the bound file.
grep__check() {
   case $1 in -q) shift ;; esac
   pat=$1; shift
   file : confline = "$1"
   grep -q -- "$pat" "$file" >/dev/null 2>&1
}
