# stdlib grep oracle (24J §1 — the pipe-guard MEDIUM core). grep IS stdlib material
# (USER_STORY stage 1, "coreutils and friends"); its oracle vouches what EVERY oracle vouches
# and NOTHING more: read-only-ness + Query-class. The engine NEVER interprets grep's rc meaning
# (no filter-semantics table, no "rc 0 = match" anywhere) — rc is opaque, welded. The pattern is
# NOT a persistent entity and grep's match depends on piped STDIN, so a LONE `grep -q` site has no
# independent fact (silence-is-wall); the CONNECTED probe (24J §2) runs the real `A | F` and reads
# back the governing rc. This predict declares only: grep is a read-only OBSERVE (`:?`), so a
# vouched pipe-predecessor of it stays a valid Query (rule-query-validity — Query gens nothing).
grep__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   pat : grepmatch = "$1"
   grep -q -- "$pat" :? grepmatch:"$pat".matched
}
