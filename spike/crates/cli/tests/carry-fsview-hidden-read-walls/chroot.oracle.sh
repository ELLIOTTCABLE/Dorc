# dorc-lang/v0.2
# chroot: fs-view wrapper, NO __enter form ⇒ entry degrades to the carry fallback (27C §4(a)).
chroot__predict() {
   shift
   "$@"
}
chroot__lend_map() {
   printf '%s\n' "$1"   : lends fs-view
   : lends user
   : lends netns
   shift
   "$@"
}
