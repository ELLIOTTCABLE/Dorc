#!/bin/sh
# subshell fallback source boundary under `set -e` (`30I` specimen 2)
set -e
SM_ORACLE_ROOT=.

. "$SM_ORACLE_ROOT/base.dorc.sh"
(
   unset -f sm_pick
   command -v sm_pick >/dev/null 2>&1 || . "$SM_ORACLE_ROOT/fallback.dorc.sh"
   sm_pick inside
)
sm_pick outside
