#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# stdlib grep oracle (24J §1 — the pipe-guard MEDIUM core). grep IS stdlib material
grep__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   pat : sm.dorc.GrepMatch = "$1"
   grep -q -- "$pat" :? sm.dorc.GrepMatch:"$pat"#matched
}
