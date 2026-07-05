# minimal fs (config-file) oracle for a `writeconf <path>` tool — the 24F closure on the `fs` kind.
# predict()/is_converged() use a MOCKED read-only `conf-exists` (never the ambient `test -e`, which
# would read the real fs — non-hermetic); touches() emits the `fs:<path>` coordinate; and the
# `fs` KIND's resolver `fs.resolve()` canonicalizes a path via ~`realpath -m` (a symlink and its
# target share one canonical). Kind-keyed (corr-kind-keying §10): `fs.resolve` applies to every
# `fs:` coordinate — footprint and backing — whoever emitted it.
writeconf__predict() {
   path : fs = "$1"
   conf-exists "$path" : fs:"$path".written
}

writeconf.touches() {
   printf 'fs:%s\n' "$1"
}

# THE VOUCH (elide-weld, 24D §3): a converged writeconf elides ONLY with a reached vouch.
writeconf.is_converged() {
   conf-exists "$1"
}

# THE RESOLVER (24F §3): the fs kind's canonicalizer — a symlink resolves to its target's real path.
fs.resolve() {
   realpath -m -- "$1"
}
