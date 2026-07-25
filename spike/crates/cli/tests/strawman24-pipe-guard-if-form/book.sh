# strawman24-pipe-guard-if-form (24J beautification-robustness). The SAME connected check-pipe as
if ! otelcol --version | grep -q "0.155.0"; then curl -sL https://example.com/otelcol.tar.gz | tar xz; fi
