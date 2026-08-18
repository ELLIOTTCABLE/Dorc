#!/bin/sh
# book-owned location; two entrypoints share one guarded dependency (`30I` specimen 1)
SM_ORACLE_ROOT=crates/cli/tests/load30-rooted-shared-dependency

. "$SM_ORACLE_ROOT/alpha.dorc.sh"
. "$SM_ORACLE_ROOT/beta.dorc.sh"

alpha_book_step first
beta_book_step second
