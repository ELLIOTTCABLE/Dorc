# dorc-lang/v0.2
# A coherent sudo wrapper (273 + 27C §3): predict env-scrubs + execs the guest; lend_map maps user
# and full-lends fs-view + netns (enumerate-every-dimension); the entry form re-runs sudo `-n`
# (non-interactive by construction). Authoring `sudo__enter` IS the traversal vouch.
sudo__predict() {
   while [ "${1#-}" != "$1" ]; do case "$1" in -u) shift 2 ;; *) shift ;; esac; done
   env -i HOME=/root "$@"
}
sudo__lend_map() {
   target=root
   while [ "${1#-}" != "$1" ]; do case "$1" in -u) target="$2"; shift 2 ;; *) shift ;; esac; done
   printf '%s\n' "$target" : lends user
   : lends fs-view
   : lends netns
   "$@"
}
sudo__enter() {
   sudo -n "$@"
}
