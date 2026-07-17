# strawman24-pipe-guard-unvouched-mid (24J NEGATIVE CONTROL — silence-is-wall preserved). The same
otelcol --version | cat | grep -q "0.155.0" || curl -sL https://example.com/otelcol.tar.gz | tar xz
