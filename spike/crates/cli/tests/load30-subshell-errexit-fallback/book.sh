#!/bin/sh
# A real dot boundary under `set -e`: the subshell removes the ambient definition,
# loads a fallback through `||`, then loses that fallback at `)` (`30I` specimen 2).
set -e
SM_ORACLE_ROOT=crates/cli/tests/load30-subshell-errexit-fallback

. "$SM_ORACLE_ROOT/base.dorc.sh"
(
   unset -f sm_pick
   command -v sm_pick >/dev/null 2>&1 || . "$SM_ORACLE_ROOT/fallback.dorc.sh"
   sm_pick inside
)
sm_pick outside
