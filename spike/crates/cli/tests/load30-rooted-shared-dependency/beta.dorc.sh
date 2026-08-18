# dorc-lang/v0.2
if command -v sm_common_query >/dev/null 2>&1; then
   :
else
   . "$SM_ORACLE_ROOT/common.dorc.sh"
fi

beta_book_step() {
   sm_common_query beta "$1"
}
