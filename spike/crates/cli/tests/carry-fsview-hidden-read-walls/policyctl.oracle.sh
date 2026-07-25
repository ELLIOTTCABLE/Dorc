# dorc-lang/v0.2
# policyctl STRADDLER (27Xb counterexample): compares against `$(cat /etc/policy)`, an unmarked
# fs-view-dependent read ⇒ (B) not read-set-closed ⇒ carry walls.
policyctl__is_converged() {
   want="$(cat /etc/policy)"
   policyctl check "$1" "$want"   : sm.dorc.KernelParam:"$1"
}
