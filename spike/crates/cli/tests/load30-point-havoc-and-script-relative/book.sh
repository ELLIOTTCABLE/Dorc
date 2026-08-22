#!/bin/sh
# TARGET (`principle-unknown-source-is-a-point-havoc` +
# `principle-load-operands-evaluate-over-controller-known-inputs`, the `${0%/*}` half): a book that
# sources an unreadable host profile keeps every binding made AFTERWARDS, and finds its own sibling
# package from `$0` with no command run to locate it. Both loads are unconditional and the package
# load sits BELOW the unknown one, so the package's definitions are exactly what the havoc must not
# swallow.
#
# TARGET RUN SET: empty. `hork tune web` is the book's only mutating line, the probe fixture says
# its cell holds, and the package's own verdict function vouches for it — so the published plan
# runs nothing under the mocks. Anything in `expected.ran` means a binding made below the unknown
# source was lost and the site fell back to running.
. "${SITE_PROFILE:-/dev/null}"

. "${0%/*}/hork.dorc.sh"

hork tune web
