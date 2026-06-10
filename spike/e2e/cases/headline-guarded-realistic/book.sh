#!/bin/sh
# pi-webhost provision — a scrappy real book (the lazy admin lets dorc do idempotency).
# A Query-guard stack, a Singleton establish, converged+diverged installs, distinct-
# selector service mutators, a guard BELOW the mutators, and a ufw rule. The run-set diff
# proves which commands run vs which dorc neutralised (20B §2 group-J). `dpkg -s` is the
# read-only status query the pkgstate oracle declares `query` (an EXTERNAL command, unlike
# the un-shimmable `command -v` builtin — so its probe is mock-reproducible: gate-1 ENFORCES).
set -e

# A bare pre-flight Query: assert the base image already has ca-certificates. VALID
# (nothing mutates above it) + holds ⇒ dorc value-substitutes it to its probed rc `true`.
dpkg -s ca-certificates >/dev/null 2>&1

# Idempotent install via the || idiom. The guard reports absent here (nginx not yet
# installed), so the install is LIVE and runs; the guard runs for real to gate it.
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx

apt-get update
apt-get install -y curl
apt-get install -y htop

systemctl enable nginx
systemctl start nginx

# A guard BELOW the mutators: writes have reached it ⇒ INVALID Query ⇒ it runs for real.
dpkg -s vim >/dev/null 2>&1 || apt-get install -y vim

ufw allow 80/tcp
