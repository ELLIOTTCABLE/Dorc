#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# otelcol oracle — the tool author's own, full role-split (rul-role-split). predict()'s `--version`
# is a read-only OBSERVE that DELEGATES to the real command, so a composed pipe stage ships this
# body and prints the genuine version banner on stdout for the downstream grep to read
# (271:rul-only-oracle-bytes-ship: only oracle bytes ship; a non-last stage must produce real bytes).
otelcol__predict() {
   case $1 in
      --version)
         collector : io.opentelemetry.Collector = "otelcol"
         otelcol --version :? io.opentelemetry.Collector:"otelcol"#version
         ;;
   esac
}
otelcol__is_converged() {
   case $1 in
      --version) otelcol --version | grep -q 0.155.0 ;;
   esac
}
