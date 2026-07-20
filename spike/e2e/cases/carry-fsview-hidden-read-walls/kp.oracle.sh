# dorc-lang/v0.1
# kp verdict reads ONLY the marked KernelParam cell + argv (read-set-closed, 27C §4(a)-(B)).
kp__is_converged() {
   kp check "$1" "$2"   : sm.dorc.KernelParam:"$1"
}
sm_dorc_KernelParam__state_stored_only_in() {
   printf 'kernel-sysctls\n'   : stored-in kernel
   : undivided-by-transit-across fs-view
}
