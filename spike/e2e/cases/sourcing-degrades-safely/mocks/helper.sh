#!/bin/sh
# Shipped helper for `sourcing-degrades-safely`. The book sources it (`. helper.sh`,
# PATH-found under mocks at exec time). Sources to a harmless no-op — dorc treats the
# source as OPAQUE (it cannot see this content), which is the whole point: the downstream
# install runs because an opaque effect reached it. Mutates nothing.
:
