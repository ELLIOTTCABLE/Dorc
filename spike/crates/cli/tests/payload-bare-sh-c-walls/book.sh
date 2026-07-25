#!/bin/sh
# payload-bare-sh-c-walls (274 §12 finding-scope-clarification; lane-payload-v1 rung-0): a bare
# `sh -c` payload site with NO eval'er oracle loaded WALLS opaquely — the payload decomposition is
# invisible without consumption (MODELS-only lane; empty-world-byte-identical). The site runs
# verbatim; `sh` is the escape-hatch head that licenses nothing here.
sh -c 'hork tune'
