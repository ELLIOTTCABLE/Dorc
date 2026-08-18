#!/bin/sh
# The same entrypoint is loaded at two textual positions under different function
# environments; its root bundles therefore remain per-load-occurrence (`30I` specimen 3).
SM_ORACLE_ROOT=crates/cli/tests/load30-two-point-frames

. "$SM_ORACLE_ROOT/entry.dorc.sh"
package_step ambient
(
   sm_pick() { common high "$@"; }
   . "$SM_ORACLE_ROOT/entry.dorc.sh"
   package_step regional
)
package_step after
