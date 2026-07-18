#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
demo__state_stored_only_in() {
   printf '/var/lib/demo\n' : fs
   :                        : invariant:user
   grep -q needle $1
}
