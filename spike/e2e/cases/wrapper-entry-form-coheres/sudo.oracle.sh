# dorc-lang/v0.1
# A COHERENT sudo wrapper (273 + 27C §3): predict env-scrubs then execs the guest; lend_map answers
# user (mapped) / fs-view (full); the entry form re-runs sudo non-interactively (`-n`), passing the
# guest verbatim. Authoring `sudo__enter` IS the traversal vouch (authoring-is-vouching): the author
# answers for the entry's self-effects (the auth-log line — 27C:rul-probe-mutation-ownership-split).
sudo__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   env -i TERM="${TERM-}" HOME=/root "$@"
}
sudo__lend_map() {
   while [ "${1#-}" != "$1" ]; do shift; done
   printf '%s\n' root : user
   :                        : fs-view
   "$@"
}
sudo__enter() {
   sudo -n "$@"
}
