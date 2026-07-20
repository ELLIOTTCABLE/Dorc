# dorc-lang/v0.2
# A COHERENT sudo wrapper (273): predict env-scrubs then execs the guest; lend_map answers the
# user (mapped) and fs-view (full) dimensions and peels to the same tail. netns is unanswered ⇒ ⊤
# (the enumerate-every-dimension law). Both members flag-strip identically, so their "$@" agree.
sudo__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   env -i TERM="${TERM-}" HOME=/root "$@"
}
sudo__lend_map() {
   while [ "${1#-}" != "$1" ]; do shift; done
   printf '%s\n' root : lends user
   : lends fs-view
   "$@"
}
