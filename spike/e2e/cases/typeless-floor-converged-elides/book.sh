#!/bin/sh
# typeless-floor-converged-elides (24L §2 — the floor exemplar). A MARKERLESS, pure-POSIX oracle
# carrying ONLY a verdict function: its ambient converged site ELIDES; the same site past an
# opaque wall GUARDS (re-checks live). No `set -e` here so the ambient rc is unconsumed and the
# elide tier is reachable (under `set -e` an auto-cell's book-command rc is ⊤ ⇒ guard-only).
foobar sync-certs /etc/nginx/certs
hork tune --profile web
foobar sync-certs /etc/nginx/certs
