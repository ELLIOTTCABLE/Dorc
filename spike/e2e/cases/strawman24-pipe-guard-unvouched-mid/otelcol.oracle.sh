#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# otelcol oracle — the tool author's own, full role-split (rul-role-split). predict()'s `--version`
# is a read-only OBSERVE that DELEGATES to the real command (271:rul-only-oracle-bytes-ship). Here
# the unvouched middle `cat` still walls the pipe (NEGATIVE CONTROL), so otelcol stays an orphan and
# runs — the clean predict shape does not rescue a non-connected pipe.
otelcol__predict() {
   case $1 in
      --version)
         collector : io.opentelemetry.Collector = "otelcol"
         otelcol --version :? io.opentelemetry.Collector:"otelcol"@version
         ;;
   esac
}
otelcol__is_converged() {
   case $1 in
      --version) otelcol --version | grep -q 0.155.0 ;;
   esac
}
