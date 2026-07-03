# package-status QUERY oracle (the DESIGN `dpkg -s nginx || apt-get install` idiom).
# `dpkg -s <pkg>` READS whether a package's status is installed — a read-only QUERY of
# pkgstate:<pkg>#installed, mutating nothing (task-D2: `query` polarity). Unlike the
# `tool` kind's `command -v` (a shell BUILTIN that cannot be PATH-shimmed), `dpkg` is an
# external command, so its probe IS mock-reproducible (gate-1 parity enforces here).
# Verbless provider (the effect-map keys the ε-verb); `-s` is a flag the predict strips.
# command-keyed predict(): `dpkg -s <pkg>` — strip the `-s` status flag, annotate the
# single operand as `pkgstate`; the probe re-runs `dpkg -s` against the bound entity.
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : pkgstate = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? pkgstate:"$pkg".installed
}
