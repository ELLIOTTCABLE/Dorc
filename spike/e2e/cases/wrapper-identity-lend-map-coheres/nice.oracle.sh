# dorc-lang/v0.2
# An IDENTITY wrapper (nice): the model drops the niceness (irrelevant to state) and re-execs the
# guest — a bare-"$@" peel that claims NOTHING on env (271:rul-env-claim-inversion). Its lend_map
# colon-lines every dimension = full lend (same world everywhere) ⇒ inner = HostDefault.
nice__predict() {
   while [ "${1#-}" != "$1" ]; do shift 2; done
   "$@"
}
nice__lend_map() {
   while [ "${1#-}" != "$1" ]; do shift 2; done
   : lends user
   : lends fs-view
   : lends netns
   "$@"
}
