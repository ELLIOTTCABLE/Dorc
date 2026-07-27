#!/bin/sh
# ── book: r26 live-smoke ────────────────────────────────  [round-26 live-target book]
# A scrappy Debian-12 book for the r26 live run: small, fast, RAM-light (the box is 2 GB),
# root-only (no sudo — the target logs in as root), and idempotent by construction so the
# second run is a real converged-world measurement rather than a re-provision.
#
# Deliberately spans the disposition spectrum rather than maximising elision:
#   · a bare `apt-get update` that is KNOWN not to elide (errexit consumes its status) and
#     that walls what follows — the teaching artifact, kept on purpose;
#   · four hand-guarded installs, the `dpkg -s x || apt-get install -y x` idiom whose LHS
#     the dpkg oracle lets dorc lift;
#   · two `cp` config drops whose convergence an oracle checks by content;
#   · `systemctl enable` then `start`, two service cells, one verdict each — split rather than
#     written `enable --now`, because two cells cannot ride a single exit status (the oracle
#     declines `--now` and says so);
#   · two honest walls at the tail — `curl` (no oracle at all) and `logger` (an oracle that
#     deliberately declines: writing a syslog line has no convergence criterion).
#
# NEVER EXECUTED BY ANY AGENT. This file and the oracles beside it are real-command material;
# they are frozen evidence in-repo. The sanctioned executor is a human, on the throwaway box
# described in `Research/notes/26E-live-target.md`. Companion ledger: `predictions.md`.
#
# The two `cp` sources are relative — run from this directory (or ship the whole directory).

set -eu

apt-get update

dpkg -s ca-certificates >/dev/null 2>&1 || apt-get install -y ca-certificates
dpkg -s curl            >/dev/null 2>&1 || apt-get install -y curl
dpkg -s jq              >/dev/null 2>&1 || apt-get install -y jq
dpkg -s nginx           >/dev/null 2>&1 || apt-get install -y nginx

cp ./r26-smoke.conf /etc/nginx/conf.d/r26-smoke.conf
cp ./r26-motd /etc/motd

systemctl enable nginx
systemctl start nginx

curl -fsS -o /dev/null http://127.0.0.1:8088/r26

logger -t dorc-r26 "r26 smoke book applied"
