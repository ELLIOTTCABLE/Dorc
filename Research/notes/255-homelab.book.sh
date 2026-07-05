#!/bin/sh
# ── book: bring up my mini-homelab ──────────────────────  [round-25 field-trial book]
# The r25 real-machine trial target (human-LOCKED 2026-07-04): a single fresh Debian-12
# box, reverse-proxied by nginx over self-signed TLS, fronting Windmill (native binary
# on a system postgres) and Home Assistant (Container). Scrappy, target-specific, low
# reuse — the lazy-admin end (DESIGN's "tool you use when you want to be lazy"). Pure
# POSIX; quality is deliberately uneven: some idempotency guards an admin writes anyway
# (the parts dorc lifts), and some bare mutations with no guard (where dorc borrows a
# probe from an oracle to decide whether to skip).
#
# PAPER STATUS: written for the dry-run (notes/255-homelab-dryrun.md); UNTESTED pre-VPS.
# Realism-uncertain lines carry an inline `# FLAG:` — verify on the day. NEVER executed
# by the dry-run; the sanctioned executor is the trial's ssh-apply runner on a real box.
#
# Companion prediction ledger + ceiling + decisions-log: notes/255-homelab-dryrun.md.
# Referenced sidecar config files (windmill.service, homelab.nginx.conf) are quoted in
# that note's appendix; on the day they sit beside this book in the working dir.

set -eu

# only my homelab box; bail harmlessly elsewhere (host-selection idiom, per pi-webhost)
case "$(hostname)" in
   homelab|hl-*) : ;;
   *) echo "not the homelab box ($(hostname)); nothing to do"; exit 0 ;;
esac

WM_VER=1.747.0                       # FIRMED: real latest tag (2026-07-03); prior 1.470.2 404'd. bump on the day
HASS_TAG=2024.12                     # FLAG: pin a real Home Assistant image tag

# ── 1. base packages ─────────────────────────────────────────────────────────
# guards I'd write anyway -> dorc can lift each `dpkg -s` and fold the install when present
apt-get update
dpkg -s nginx      >/dev/null 2>&1 || apt-get install -y nginx
dpkg -s postgresql >/dev/null 2>&1 || apt-get install -y postgresql
dpkg -s docker.io  >/dev/null 2>&1 || apt-get install -y docker.io
dpkg -s openssl    >/dev/null 2>&1 || apt-get install -y openssl

# ── 2. windmill server binary (pinned) — the vendor tool, version-guarded download ──
# the version check is my hand-guard; a stale binary must NOT pass (a bare `command -v`
# would). This is the line I'll grudgingly write a 6-line oracle for (stage C).
# FIRMED: asset `windmill-amd64` is real (CE amd64, present in v1.747.0 release assets) — this curl
# works. But upstream documents the native path NOWHERE (README/self_host = compose/helm/cloud only;
# first-party Q&A: "run from binary... not recommended unless you know what you're doing"). dec-2
# validated by-absence → native+systemd is admin-invented; compose is the blessed fallback.
windmill --version 2>/dev/null | grep -q "$WM_VER" \
   || curl -fsSL "https://github.com/windmill-labs/windmill/releases/download/v${WM_VER}/windmill-amd64" \
        -o /usr/local/bin/windmill
chmod 755 /usr/local/bin/windmill
cp ./windmill.service /etc/systemd/system/windmill.service
systemctl daemon-reload

# ── 3. postgres role + db for windmill (peer auth ⇒ run the psql as the postgres user) ──
# the `su - postgres -c` wrapper is the idiomatic Debian spelling; the SELECT-guard makes
# each half idempotent. FLAG: the mutation lives inside su's -c string (opaque to dorc).
su - postgres -c "psql -tAc \"SELECT 1 FROM pg_roles WHERE rolname='windmill'\" | grep -q 1 \
   || psql -qc \"CREATE ROLE windmill LOGIN PASSWORD 'changeme'\""
su - postgres -c "psql -tAc \"SELECT 1 FROM pg_database WHERE datname='windmill'\" | grep -q 1 \
   || createdb -O windmill windmill"

# ── 4. bring windmill up (needs the db to exist first) ──
systemctl enable --now windmill

# ── 5. home assistant, containerised (the opaque one; host-net for device discovery) ──
# FLAG: `docker run` is non-idempotent as written — a second run errors on the existing
# name. A real lazy admin often leaves this bare (and eats the re-run error) or hand-guards
# with `docker ps`. Left bare here to stand as the honest poison-wall.
docker run -d --name homeassistant --restart unless-stopped \
   -v /srv/hass:/config -v /etc/localtime:/etc/localtime:ro --network host \
   "ghcr.io/home-assistant/home-assistant:${HASS_TAG}"

# ── 6. nginx reverse-proxy + self-signed TLS ──
install -d -m 0700 /etc/nginx/certs
# self-signed cert; the `[ -f ]` guard is lazier than correct — it never checks EXPIRY,
# so a present-but-expired cert wrongly passes (the converged≠no-op trap). On-brand.
[ -f /etc/nginx/certs/homelab.crt ] \
   || openssl req -x509 -newkey rsa:2048 -nodes -days 365 \
        -keyout /etc/nginx/certs/homelab.key -out /etc/nginx/certs/homelab.crt \
        -subj "/CN=homelab.lan"
# only drop the vhost if it's missing (also skips UPDATES to it — lazy, on-brand). The
# heredoc body is why dorc can't cleanly edit this line's span (heredoc-refusal edge).
if [ ! -f /etc/nginx/sites-available/homelab ]; then
   cat > /etc/nginx/sites-available/homelab <<'EOF'
server {
   listen 443 ssl;
   server_name homelab.lan;
   ssl_certificate     /etc/nginx/certs/homelab.crt;
   ssl_certificate_key /etc/nginx/certs/homelab.key;
   location /windmill/ { proxy_pass http://127.0.0.1:8000/; }
   location /         { proxy_pass http://127.0.0.1:8123/; proxy_set_header Host $host; }
}
EOF
fi
ln -sf /etc/nginx/sites-available/homelab /etc/nginx/sites-enabled/homelab
rm -f /etc/nginx/sites-enabled/default
# the one careful bit: validate before reloading (the change-signal idiom, per pi-webhost)
nginx -t && systemctl reload nginx

# ── 7. firewall ──
ufw allow 22/tcp
ufw allow 443/tcp

echo "homelab up"
