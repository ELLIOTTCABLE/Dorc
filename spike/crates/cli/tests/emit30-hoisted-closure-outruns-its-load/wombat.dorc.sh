# dorc-lang/v0.2
# The ordinary two-part closure shape (`pin28-closure-travels-with-the-definition`): a file-level
# constant and a helper the role body delegates to. Both are top-level declarations the funcdef span
# does not cover, so both travel with the pinned definition — and the constant is what collides with
# the book's own read.
WOMBAT_ROOT=/etc/wombat

_wombat_dest() {
   wombat cmp -- "$1" "$WOMBAT_ROOT/$1"
}

wombat__is_converged() {
   _wombat_dest "$2"
}
