# dorc-lang/v0.2
if [ "${sm_common_loaded-}" != 'sm.common/v1' ]; then
   . "$SM_ORACLE_ROOT/common.dorc.sh"
fi

alpha_book_step() {
   sm_common_query alpha "$1"
}
