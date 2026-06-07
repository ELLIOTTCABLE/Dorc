# ═══════════════════════════════════════════════════════════════════════
# oracle: nginx — config validate / reload          [STRAWMAN, for discussion]
# ═══════════════════════════════════════════════════════════════════════
# SHAPE: config-test. nginx's only non-mutating mode is `nginx -t` — it validates
# the ON-DISK config. That answers "is the config loadable?" (a PRECONDITION) but
# NOT "would `nginx -s reload` change running behaviour"; nginx exposes no
# running-config to diff against. So this shape does not fit a convergence .check
# cleanly — that mismatch is the finding:
#   - what nginx offers is a VALIDATE (precondition), distinct from convergence;
#   - reload re-reads config and is effectively always would-change at this
#     altitude; the real convergence ("did the config files change?") lives in the
#     BOOK's file-diff (sp-change `cmp -s`), not in nginx.
#
# So .check = validate-then-pessimistically-would-change; .diff is omitted
# (degenerate — no running baseline to diff against). Helpers plain POSIX.

nginx.check() {
   command -v nginx >/dev/null 2>&1 || { echo "nginx oracle: no nginx" >&2; return 2; }
   if ! nginx -t >/dev/null 2>&1; then
      # config invalid -> reload/start would FAIL at apply. A warning the admin
      # wants (q-warn-channel); >&2 stand-in. Refuse to vouch.
      echo "nginx oracle: 'nginx -t' fails — config invalid; apply would error" >&2
      return 2
   fi
   return 1   # valid, but reload re-reads → treat as would-change (pessimistic)
   # HOLE(reserved): config-test tools want a PRECONDITION verb (e.g. `.valid` =
   #   `nginx -t`) distinct from a convergence `.check` — extending the verb-ladder
   #   is your call; not invented here.
   # HOLE(ceiling): "would reload change running behaviour" needs running-vs-disk
   #   config diff; nginx exposes no running-config. Convergence comes from the
   #   book's file-diff, not nginx.
   # NOTE(q-probe-privilege): `nginx -t` may need root to open the configured
   #   error_log / pid paths; an unprivileged probe can mis-fail. Tracked.
}

nginx.version() {
   command -v nginx >/dev/null 2>&1 || return 2
   nginx -v 2>&1   # nginx prints its version to STDERR, hence 2>&1.
}
