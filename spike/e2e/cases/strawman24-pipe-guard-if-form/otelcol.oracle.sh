#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# otelcol oracle — the tool author's own, full role-split (rul-role-split). predict()'s `--version`
# arm is a read-only OBSERVE (`:?`, a Singleton `otelcol` cell) — the HONEST mark for a version
# check that mutates nothing. That read-only vouch is what 24J §2 needs: as the NON-last stage of the
# check-pipe `otelcol --version | grep -q …`, otelcol (a) does NOT invalidate grep's downstream Query
# (rule-query-validity — a Query/read-only predecessor gens nothing), and (b) makes the pipe an
# all-vouched-read-only CONNECTED probe. is_converged() (the guard-verdict, rul24-vouch-is-verdict-
# authoring) is now redundant for THIS line — the connected-probe path rides the read-only Query
# vouch, not the mutator-guard lane — but kept to show the author's full oracle. (Stripped body is
# runnable sh.)
otelcol__predict() {
   case $1 in
      --version) v : io.opentelemetry.Collector; otelcol --version >/dev/null 2>&1 :? io.opentelemetry.Collector:#v0155 ;;
   esac
}
otelcol__is_converged() {
   case $1 in
      --version) otelcol --version | grep -q 0.155.0 ;;
   esac
}
