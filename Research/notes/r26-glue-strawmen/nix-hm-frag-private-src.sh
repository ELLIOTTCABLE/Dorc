#!/bin/sh
# ═══════════════════════════════════════════════════════════════════════════
#  FROZEN EVIDENCE · STRAWMAN · IMAGINATION-TIER
#  NOT RUNNABLE. NEVER EXECUTE. No format- or wire-compat is promised.
#  The only sanctioned executor of fixture material in this repo is
#  `mise run test:e2e`, and this file is not fixture material.
# ═══════════════════════════════════════════════════════════════════════════
#
#  The fragment `nix-hm-splice.nix` splices in, by store path.
#
#  Note what is NOT here: no marker line, no `#:` marks, no oracles, no
#  Dorc anything. A fragment is a BOOK, and books are the lazy end. This
#  one is eleven lines of the sh a person would write, and its entire
#  Dorc-facing property is that it can be pointed at.
#
#  Note also what it is ABOUT. Everything below is state Home Manager
#  cannot own, for reasons that are structural rather than missing-feature:
#
#  - a git working tree is mutable, and the store is immutable;
#  - this repository is private, and the store is world-readable — the Nix
#    manual's own Secrets section says so and tells you to read secrets
#    "from the filesystem (with appropriate access controls) at run time";
#  - `~/.ssh` mode bits belong to a directory Home Manager does not manage,
#    and its `checkLinkTargets` block exists precisely to refuse collisions
#    with unmanaged files rather than to take them over.
#
#  So this is not a workaround for a Home Manager gap. It is the residue,
#  and the residue has to live in an activation block or in a person's
#  memory. Today the ecosystem's answer is the second one, with a doc
#  telling you to make it idempotent yourself.

set -eu

SRC="$HOME/src/notes"
REMOTE=git@git.example.invalid:me/notes.git

# A guard the admin writes anyway, and exactly the shape that lifts: a
# read whose exit code proves the fallback branch dead.
if [ ! -d "$SRC/.git" ]; then
   git clone "$REMOTE" "$SRC"
fi

git -C "$SRC" fetch --quiet origin
git -C "$SRC" merge --ff-only origin/main

# Mode bits on a directory nobody declared. `install -d` is the idempotent
# spelling; `mkdir -p && chmod` is the two-line one. Either lifts; this one
# reads better.
install -d -m 0700 "$HOME/.ssh"
