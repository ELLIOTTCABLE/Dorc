#!/bin/sh
# TARGET (`30P:model-symbolic-dollar-zero` + the slashless paragraph): both load-head lints, on
# one book, end to end through the real binary — the honest firing route their defining cases
# cannot reach, since both mints sit in the binary's own load-edge driver.
#
# The first `.` RESOLVES under the spelling Dorc invokes (`$0` = `./book.sh`, so `${0%/*}` is `.`)
# and is provably fatal under `sh book.sh` (`$0` has no `/`, so `${0%/*}` is the whole word and the
# operand becomes `book.sh/hork.dorc.sh` — a path under a FILE). EXACT holds; the off-ramp does not.
. "${0%/*}/hork.dorc.sh"

# The second carries no `/` at all, which POSIX makes a PATH search rather than a cwd lookup — so
# no file beside the book answers it and the site walls. It sits BELOW the first deliberately: the
# cwd it makes unknowable must not reach back up and cost the resolution above it.
. wombat-helpers.sh

hork tune web
