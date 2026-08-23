# dorc-lang/v0.2
# An ordinary verdict body, pre-sourced: its bindings are AMBIENT, so the artifact carries them in
# the hoisted preamble and nowhere else — which is what makes the emitted NAME the only thing
# standing between the book's squat and the guard.
wombat__is_converged() {
   case "${1-}" in
   sync) wombat cmp -- "${2-}" ;;
   *) return 2 ;;
   esac
}
