# guard23-inverted-vouch-never-backwards (23J conv-rc-soundness facet (i); the rc-soundness
# pin-set, pinned BEFORE the guard tier — a FLOOR). `apt-get purge oldpkg` is oracle-modeled
# as a KILL (the `!`/inverted effect). Its oracle VOUCHES the purge path — but the vouch is now
# the authored INVERTED-sense verdict function `apt-get.is_diverged()` (rul24-vouch-is-verdict-
# authoring, 24A §1c), not a tilde mark. THE LAW (never-backwards): no build may ever mint a
# guard whose pass-direction fires exactly when the world has DRIFTED to needing the command —
# the backwards guard, which would SKIP the purge precisely when it is required. The only legal
# glue for an inverted verdict is the engine's lossless sense-flip
# `( apt_get__is_diverged args; [ $? -eq 1 ] ) || apt-get purge …` (rul-rc-partition). At HEAD
# no guard mints (the verdict function is inert — the lift keys only on .predict), so the purge
# runs BARE. Floor: `ran: apt-get purge oldpkg`; RED if a future build mints a backwards guard
# (the mutator suppressed when the mock reports the package present ⇒ drift ⇒ purge needed).
apt-get purge oldpkg
