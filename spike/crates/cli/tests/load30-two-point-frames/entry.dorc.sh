# dorc-lang/v0.2
if [ "${sm_fallback_loaded-}" != 'sm.fallback/v1' ]; then
   . "$SM_ORACLE_ROOT/fallback.dorc.sh"
fi

package_step() {
   sm_pick "$@"
}
