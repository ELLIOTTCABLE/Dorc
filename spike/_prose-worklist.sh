#!/bin/sh
# TEMPORARY (the r28 prose-author pass) — delete once the prose round is done. Mechanical only:
# every prose register across both stores still awaiting a human, one line per item, as
#   <slug> · <store> · <state> · <edit-home>
# `help: None` is NOT listed: a code with no help register is complete, not unwritten.
set -eu
cd "$(dirname "$0")"

emit() { # slug store state [lock-only]
   home="crates/aid/src/${2%%:*}_lock.rs"
   if [ -f "crates/aid/tests/$1.loom" ] && [ -z "${4:-}" ]; then
      home="crates/aid/tests/$1.loom"
   fi
   printf '%s · %s · %s%s · %s\n' "$1" "$2" "$3" "${4:+ $4}" "$home"
}

awk -F'"' '
   /^        slug: / { s = $2 }
   /^        message: None/ { print s, "catalog:message", "unwritten" }
   /^        message: Some\("sm / { print s, "catalog:message", "sm-migrated" }
   /^        help: Some\("sm / { print s, "catalog:help", "sm-migrated" }
' crates/aid/src/catalog_lock.rs | while read -r slug store state; do emit "$slug" "$store" "$state"; done

# A multi-word arrangement entry refuses a transcript edit (289:seam-multiword-chrome-render-only),
# so its only edit-home is the lock however many looms name it.
awk -F'"' '
   /^        slug: / { s = $2 }
   /words: Words::Unwritten/ { print s, "arrangement:words", "unwritten", "" }
   /words: Words::Migrated/ { print s, "arrangement:words", "migrated", (gsub(/", "/, "") ? "lock-only" : "") }
' crates/aid/src/arrangement_lock.rs | while read -r slug store state only; do emit "$slug" "$store" "$state" "$only"; done
