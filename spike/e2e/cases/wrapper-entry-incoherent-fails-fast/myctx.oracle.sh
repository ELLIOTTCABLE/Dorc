# dorc-lang/v0.1
# An INCOHERENT context wrapper (27C:rul-fold-entry-coherence-failfast): the lend_map consumes ONE
# leading arg (the fs-view target) before the guest; the entry form consumes TWO. The entry drops an
# arg the fold relied on — static incoherence, caught pre-network (never a semantic-effect check).
myctx__lend_map() {
   printf '%s\n' "$1" : lends fs-view
   shift
   "$@"
}
myctx__enter() {
   a=$1; shift
   b=$1; shift
   realctx "$a" "$@"
}
