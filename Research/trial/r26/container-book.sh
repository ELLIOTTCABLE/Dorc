#!/bin/sh
# ── book: r26 livetest (container variant) ──────────────  [round-26 live-target book]
# `smoke-book.sh`, restricted to what an UNPRIVILEGED container can actually converge. This is
# the book `mise run livetest` drives; `smoke-book.sh` remains the VPS book and is unchanged.
#
# Two deletions, both forced rather than chosen:
#
#   · the `systemctl enable`/`start` pair and the `curl` reachability check that depends on
#     them. No container runtime reachable from this project grants the privileges systemd
#     needs as PID 1 (`wslc run` exposes no --privileged and no --cap-add), so `systemctl`
#     answers "System has not been booted with systemd as init system" and `set -eu` takes the
#     book down at that line. Demanding a privileged container from a first-time contributor
#     would be the wrong trade even where it is possible.
#   · the bare `apt-get update`. It is the teaching artifact that costs `smoke-book.sh` every
#     elision on purpose — which makes it exactly wrong here, where the whole assertion is that
#     a second pass over a converged world elides what the first pass ran.
#
# What survives still spans the spectrum livetest needs to observe: four `dpkg -s x ||
# apt-get install -y x` lines whose ladder folds once the world is converged, two `cp` drops
# an oracle checks by content, and one honest wall (`logger`, an oracle that deliberately
# declines — an append-only log has no convergence criterion).
#
# The two `cp` sources are relative; the run's cwd must hold them. livetest ships them to the
# remote login directory before it plans.

set -eu

dpkg -s ca-certificates >/dev/null 2>&1 || apt-get install -y ca-certificates
dpkg -s curl            >/dev/null 2>&1 || apt-get install -y curl
dpkg -s jq              >/dev/null 2>&1 || apt-get install -y jq
dpkg -s nginx           >/dev/null 2>&1 || apt-get install -y nginx

cp ./r26-smoke.conf /etc/nginx/conf.d/r26-smoke.conf
cp ./r26-motd /etc/motd

logger -t dorc-r26 "r26 livetest book applied"
