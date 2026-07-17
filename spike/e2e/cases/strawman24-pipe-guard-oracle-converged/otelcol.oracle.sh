#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# Vendor-authored oracle: a third-party kind lives under the VENDOR's domain
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
