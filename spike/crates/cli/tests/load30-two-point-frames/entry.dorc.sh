# dorc-lang/v0.2
if command -v sm_pick >/dev/null 2>&1; then
   :
else
   . "$SM_ORACLE_ROOT/fallback.dorc.sh"
fi

package_step() {
   sm_pick "$@"
}
