#!/bin/sh
# wrapper-dual-peel-incoherent-fails-fast (273 §5): the loaded wrapper oracle's __predict and
# __lend_map peel to DIFFERENT tail positions (predict shifts the verb, lend_map does not) — static
# incoherence ⇒ dorc fast-fails (exit 11, EXIT_WRAPPER_INCOHERENT). The artifact still ships; the
# book's own `hork setup` site is unrelated and walls (unmodeled). Pins the fail-fast is loud + the
# artifact is emitted (declarations-genuinely-contradict, pre-network).
hork setup
