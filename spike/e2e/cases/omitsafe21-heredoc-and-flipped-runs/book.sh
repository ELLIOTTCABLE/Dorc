# omitsafe21-heredoc-and-flipped-runs (round-21 f1 — the `&&` dual, the rc-DIVERGENT
# cell): `dpkg -s nginx <<EOF … && systemctl reload nginx` with the authored probe saying
# nginx ABSENT (rc 1) ⇒ the `&&` body is fold-dead (Omit). The heredoc guard is render-
# refused ⇒ KEPT live; the omit-safety gate must keep the dead reload VERBATIM behind it.
# This `&&` cell is where the pre-fix artifact (`… && :`) was not merely dishonest but
# rc-DIVERGENT: on a flipped host (nginx present at apply) the live guard proceeds and
# `:` hands the list rc 0 — "guard held AND body succeeded" — though nothing ran; the
# bare book runs the reload (rc = the reload's own), and frozen-both (`false && :`) holds
# the probe-sourced rc 1. No world produces the pre-fix composite (a fabricated rc-0 for
# a never-run body — inv-probe-sourced-values pierced via the render seam). Post-fix the
# artifact IS the bare book: at apply on the flipped host the live guard proceeds and the
# kept reload RUNS (kFAIL-perform). run-set [dpkg, systemctl]; list rc 0 ⇒ exits 0.
# PROBE_RESULTS=authored: authored=probe-time (absent rc=1), mocks=apply-time (present).
set -e
dpkg -s nginx <<EOF >/dev/null 2>&1 && systemctl reload nginx
omit-safety heredoc payload
EOF
