# strawman24-alias-symlink (24F Stage 5 — the aliasing closure on the `fs` KIND, per-kind gradual
# enhancement). Same closure as strawman24-alias-provides, a DIFFERENT kind: a file and a SYMLINK to
# it are two names for one referent (`/etc/app.conf` vs `/etc/app.conf.lnk`). token-equality calls
# `fs:/etc/app.conf` and `fs:/etc/app.conf.lnk` disjoint ⇒ the converged victim would wrongly survive.
# The `fs` kind's owner ships `fs.resolve()` (~`realpath -m`): the engine canonicalizes both coords
# through it before disjoint.
#   site 0  writeconf /etc/app.conf      — DIVERGED (absent) ⇒ RUNS. The footprinted wall (fs:/etc/app.conf).
#   site 1  writeconf /etc/app.conf.lnk  — CONVERGED (holds), PAST the running wall. Its backing
#           (fs:/etc/app.conf.lnk) realpath-canonicalizes to `/etc/app.conf`; the wall footprint
#           (fs:/etc/app.conf) canonicalizes to `/etc/app.conf` too ⇒ a canonical HIT ⇒ the closure
#           correctly DEMOTES the victim to RUN. Under token-equality it would have elided (wrong-
#           survival). Differential: expected.ran runs BOTH — the under-execute closed on a second kind.
writeconf /etc/app.conf
writeconf /etc/app.conf.lnk
