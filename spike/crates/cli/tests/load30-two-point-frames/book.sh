#!/bin/sh
# one entrypoint at two positions and in two function frames (`30I` specimen 3)
SM_ORACLE_ROOT=crates/cli/tests/load30-two-point-frames

. "$SM_ORACLE_ROOT/entry.dorc.sh"
package_step ambient
(
   sm_pick() { common high "$@"; }
   . "$SM_ORACLE_ROOT/entry.dorc.sh"
   package_step regional
)
package_step after
