#!/bin/sh
# subshell fallback source boundary under `set -e` (`30I` specimen 2)
set -e
SM_ORACLE_ROOT=.

. "$SM_ORACLE_ROOT/base.dorc.sh"
(
   unset -f sm_pick
   [ "${sm_fallback_loaded-}" = 'sm.fallback/v1' ] || . "$SM_ORACLE_ROOT/fallback.dorc.sh"
   sm_pick inside
)
sm_pick outside
