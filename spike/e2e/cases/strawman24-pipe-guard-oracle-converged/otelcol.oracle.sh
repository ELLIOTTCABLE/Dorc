# otelcol oracle — the tool author's own, full role-split (rul-role-split). predict() resolves
# the entity + ships the read-only version probe; is_converged() is the guard-verdict
# (rul24-vouch-is-verdict-authoring). BOTH key on `otelcol` — the check pipeline's FIRST stage,
# whose status the `||` does NOT read. This is the strongest an author can write and it still
# buys nothing here: the governing status is grep's. (Its stripped body is runnable sh.)
otelcol__predict() {
   case $1 in
      --version) otelcol --version >/dev/null 2>&1 : otelcol:otelcol.v0155 ;;
   esac
}
otelcol.is_converged() {
   case $1 in
      --version) otelcol --version | grep -q 0.155.0 ;;
   esac
}
