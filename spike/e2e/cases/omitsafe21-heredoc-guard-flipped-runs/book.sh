# omitsafe21-heredoc-guard-flipped-runs (round-21 f1 — the kFAIL-perform pole of
set -e
dpkg -s nginx <<EOF >/dev/null 2>&1 || apt-get install -y nginx
omit-safety heredoc payload
EOF
