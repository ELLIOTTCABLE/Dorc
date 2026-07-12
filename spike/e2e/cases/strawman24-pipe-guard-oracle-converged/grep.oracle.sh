#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# stdlib grep oracle: vouches read-only Query-class (`:?`) and nothing more — the engine
# measures the pipe's governing rc, never interprets what a grep match MEANS.
grep__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   pat : sm.dorc.GrepMatch = "$1"
   grep -q -- "$pat" :? sm.dorc.GrepMatch:"$pat"#matched
}
