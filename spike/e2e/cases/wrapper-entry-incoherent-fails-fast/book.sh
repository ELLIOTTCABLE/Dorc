#!/bin/sh
# wrapper-entry-incoherent-fails-fast (27C:rul-fold-entry-coherence-failfast): the loaded wrapper's
# __enter and __lend_map disagree on argv flow by STATIC sh-structure — the entry shifts TWO leading
# args, the lend-fold consumed ONE, so the entry drops an arg the fold relied on. Declarations-
# genuinely-contradict ⇒ dorc fast-fails (exit 11, EXIT_WRAPPER_INCOHERENT), pre-network. The
# artifact still ships; the book's own `hork setup` site is unrelated and walls.
hork setup
