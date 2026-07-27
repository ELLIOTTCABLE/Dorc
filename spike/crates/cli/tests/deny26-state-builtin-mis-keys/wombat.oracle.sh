# dorc-lang/v0.2
# The `set --` is the whole point: it is idiomatic POSIX (THE list workaround), no contract clause
# warns against it, and the tracer used to walk straight past it with its own positionals intact.
wombat__predict() {
   set -- alpha
   pkg : sm.dorc.Thing = "$1"
   wombat query "$pkg" >/dev/null 2>&1 : sm.dorc.Thing:"$pkg"@present
}
