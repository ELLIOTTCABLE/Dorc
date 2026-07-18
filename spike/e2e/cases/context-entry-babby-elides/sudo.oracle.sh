# dorc-lang/v0.1
# A COHERENT sudo wrapper (273 + 27C §3): predict env-scrubs then execs the guest; lend_map maps
# user (root, or `-u TARGET`) and FULL-lends fs-view + netns (sudo shifts neither — the enumerate-
# every-dimension law); the entry form re-runs sudo non-interactively (`-n`, non-interactive by
# construction). Authoring `sudo__enter` IS the traversal vouch (27C:rul-entry-denoted-siting-vouch).
sudo__predict() {
   while [ "${1#-}" != "$1" ]; do case "$1" in -u) shift 2 ;; *) shift ;; esac; done
   env -i HOME=/root "$@"
}
sudo__lend_map() {
   target=root
   while [ "${1#-}" != "$1" ]; do case "$1" in -u) target="$2"; shift 2 ;; *) shift ;; esac; done
   printf '%s\n' "$target" : user
   :   : fs-view
   :   : netns
   "$@"
}
sudo__enter() {
   sudo -n "$@"
}
