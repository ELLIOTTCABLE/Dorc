# dorc-lang/v0.2
# Answers `provision` with a real measurement, markless body (the typeless floor: the site keys the
# per-provider auto-cell). The mock's `hork status` exits 1, so the site measures DIVERGED and the
# line really runs — a modeled, attributed, RUNNING wall. Every other invocation-shape declines.
hork__is_converged() {
   [ "$1" = provision ] || return 2
   hork status
}
