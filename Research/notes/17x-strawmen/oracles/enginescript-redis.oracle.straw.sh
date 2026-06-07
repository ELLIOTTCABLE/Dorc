#!/bin/sh
# Oracle EXTRACTED from books/enginescript-redis.book.sh (EngineScript @3efaf87). [STRAWMAN]
# The MESSY end (honest sample). Idempotency is mostly a WHOLE-SCRIPT SENTINEL, not per-kind.
# Kinds: group:redis · user:www-data (SHARED, cross-script) · service:redis-server
#
# (A) LIFTED — the ONLY per-kind guard the human wrote:
redis_group() { getent group redis >/dev/null 2>&1; }   # book L74: if ! getent group redis
#
# (sentinel, NOT kind-correlatable — recorded so the coverage tally is honest:)
# book ~L5: `source /etc/enginescript/install-state.conf; [ "$REDIS" = 1 ] && exit 0`  -> Dorc-opaque.
#
# (B) SUPPLIED — mutated UNGUARDED; getent-pattern probes COULD discharge/elide them:
wwwdata_user()     { getent passwd www-data >/dev/null 2>&1; }       # book L77 `usermod -aG redis www-data`
                                                                     #   assumes www-data EXISTS — no guard, no token
                                                                     #   tying it to its creator. THE gap (see _correlate).
wwwdata_in_redis() { id -nG www-data 2>/dev/null | grep -qw redis; } # would elide the usermod itself
redis_svc()        { systemctl is-enabled --quiet redis-server 2>/dev/null; }   # book L82, unguarded enable
#
# Coverage: 1 of ~4 state-changes author-guarded; rest are supplied-probe or sentinel-gated. The sed'd
# redis.conf CONTENT is not a getent-pattern kind at all (out of reach).
