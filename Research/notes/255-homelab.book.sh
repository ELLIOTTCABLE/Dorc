#!/bin/sh
# ── book: bring up my mini-homelab ──────────────────────  [round-25 field-trial book]
# The r25 real-machine trial target (human-LOCKED 2026-07-04; OTel stack SWAPPED in for
# Windmill the same day): a single fresh Debian-12 box, reverse-proxied by nginx over
# self-signed TLS, fronting a lean OpenTelemetry monitoring stack — otel-collector +
# prometheus + grafana (native binary on a system postgres) — and Home Assistant (Container).
# Scrappy, target-specific, low reuse — the lazy-admin end (DESIGN's "tool you use when you
# want to be lazy"). Pure POSIX; quality is deliberately uneven: some idempotency guards an
# admin writes anyway (the parts dorc lifts), and some bare mutations with no guard (where
# dorc borrows a probe from an oracle to decide whether to skip).
#
# WHY THREE NATIVE SERVICES (not Windmill): windmill's native multi-unit install is
# admin-invented (upstream documents only docker/compose); compose would hide the composition
# behind one opaque `docker compose up` dorc can't exercise, and is redundant with HA's docker
# wall. The OTel stack is genuine multi-service as SEPARATE native systemd units dorc can see,
# each installed by a version-guarded binary download — so there are now THREE tractable vendor
# walls (the u3-test + the hand-oracle target), not one. (Per notes/256: on-box-native is a
# deliberate exercise-dorc divergence — the human's real observability stack is containerised.)
#
# PAPER STATUS: written for the dry-run (notes/255-homelab-dryrun.md); UNTESTED pre-VPS.
# Realism-uncertain lines carry an inline `# FLAG:` — verify on the day. NEVER executed by the
# dry-run; the sanctioned executor is the trial's ssh-apply runner on a real box.
#
# Companion prediction ledger + ceiling + decisions-log: notes/255-homelab-dryrun.md. The cp'd
# sidecar files (otelcol-config.yaml, otelcol-contrib.service, prometheus.yml,
# prometheus.service, grafana.service) + the inline nginx vhost are quoted in that note's
# appendix; on the day they sit beside this book in the working dir.

set -eu

# only my homelab box; bail harmlessly elsewhere (host-selection idiom, per pi-webhost)
case "$(hostname)" in
   homelab|hl-*) : ;;
   *) echo "not the homelab box ($(hostname)); nothing to do"; exit 0 ;;
esac

# FIRMED (255-firming, first-party release APIs 2026-07-04): all three are the real latest tags.
OTEL_VER=0.155.0                     # otelcol-contrib (open-telemetry/opentelemetry-collector-releases)
PROM_VER=3.13.0                      # prometheus (prometheus/prometheus)
GRAF_VER=13.1.0                      # grafana OSS (dl.grafana.com; GitHub ships only build-numbered debs)
HASS_TAG=2024.12                     # FLAG: pin a real Home Assistant image tag

# ── 1. base packages ─────────────────────────────────────────────────────────
# guards I'd write anyway -> dorc can lift each `dpkg -s` and fold the install when present
apt-get update
dpkg -s nginx      >/dev/null 2>&1 || apt-get install -y nginx
dpkg -s postgresql >/dev/null 2>&1 || apt-get install -y postgresql
dpkg -s docker.io  >/dev/null 2>&1 || apt-get install -y docker.io
dpkg -s openssl    >/dev/null 2>&1 || apt-get install -y openssl

# ── 2. otel-collector binary (pinned) — vendor tool #1, version-guarded download ──
# the version check is my hand-guard; a stale binary must NOT pass (a bare `command -v` would).
# This `svc --version | grep -q "$VER" || download` is the idiom dorc's 6-line hand-oracle
# lifts (stage C) — REPEATED for all three services here (§2/§3/§4), so one oracle-shape buys
# three walls back.
# FIRMED: asset `otelcol-contrib_0.155.0_linux_amd64.tar.gz` is real (contrib distro, GitHub
# release v0.155.0). Unlike windmill, the native path IS documented — the .deb ships a
# first-class systemd unit + /etc/otelcol-contrib/config.yaml; we use the tarball form
# DELIBERATELY to keep it a tractable hand-oracle wall (dec-2). Config path + OTLP ports
# (4317/4318) firmed against opentelemetry.io.
otelcol-contrib --version 2>/dev/null | grep -q "$OTEL_VER" \
   || { curl -fsSL "https://github.com/open-telemetry/opentelemetry-collector-releases/releases/download/v${OTEL_VER}/otelcol-contrib_${OTEL_VER}_linux_amd64.tar.gz" -o /tmp/otelcol.tgz \
        && tar -xzf /tmp/otelcol.tgz -C /usr/local/bin otelcol-contrib; }
install -d /etc/otelcol-contrib
cp ./otelcol-config.yaml     /etc/otelcol-contrib/config.yaml
cp ./otelcol-contrib.service /etc/systemd/system/otelcol-contrib.service

# ── 3. prometheus binary (pinned) — vendor tool #2, version-guarded download ──
# FIRMED: `prometheus-3.13.0.linux-amd64.tar.gz` is real (GitHub release v3.13.0); the tarball
# holds `prometheus`+`promtool`+a sample yml under a versioned dir (hence --strip-components).
# prometheus documents the pre-compiled-binary path FIRST-PARTY (it's the headline install) but
# ships NO systemd unit — the unit below is mine (admin-invented, like windmill's). Web port
# :9090 firmed.
prometheus --version 2>/dev/null | grep -q "$PROM_VER" \
   || { curl -fsSL "https://github.com/prometheus/prometheus/releases/download/v${PROM_VER}/prometheus-${PROM_VER}.linux-amd64.tar.gz" -o /tmp/prom.tgz \
        && tar -xzf /tmp/prom.tgz -C /usr/local/bin --strip-components=1 \
             "prometheus-${PROM_VER}.linux-amd64/prometheus" \
             "prometheus-${PROM_VER}.linux-amd64/promtool"; }
install -d /etc/prometheus /var/lib/prometheus
cp ./prometheus.yml     /etc/prometheus/prometheus.yml
cp ./prometheus.service /etc/systemd/system/prometheus.service

# ── 4. grafana binary (pinned) — vendor tool #3, version-guarded download; needs a homepath ──
# FIRMED: grafana ships a standalone linux tarball at dl.grafana.com (GitHub has only
# build-numbered debs). Unlike a single binary, grafana wants its whole tree under a homepath
# (/usr/local/grafana) — the tarball unpacks to grafana-$VER/ which I rename. Native path is
# first-class here too (apt.grafana.com / deb ship grafana-server.service + /etc/grafana) —
# tarball form is the deliberate exercise-dorc choice (dec-2). Grafana reads postgres + serves
# :3000 via GF_* env in grafana.service; behind /grafana/ it needs root_url + serve_from_sub_path
# (firmed) — set in the unit.
# FLAG: the exact version subcommand (`grafana --version` vs `grafana server --version`) and the
# tarball top-dir name (`grafana-13.1.0` vs `grafana-v13.1.0`) — verify on the day.
/usr/local/grafana/bin/grafana --version 2>/dev/null | grep -q "$GRAF_VER" \
   || { curl -fsSL "https://dl.grafana.com/oss/release/grafana-${GRAF_VER}.linux-amd64.tar.gz" -o /tmp/grafana.tgz \
        && tar -xzf /tmp/grafana.tgz -C /usr/local \
        && rm -rf /usr/local/grafana && mv "/usr/local/grafana-${GRAF_VER}" /usr/local/grafana; }
cp ./grafana.service /etc/systemd/system/grafana.service
systemctl daemon-reload

# ── 5. postgres role + db for grafana (peer auth ⇒ run the psql as the postgres user) ──
# the `su - postgres -c` wrapper is the idiomatic Debian spelling; the SELECT-guard makes each
# half idempotent. FLAG: the mutation lives inside su's -c string (opaque to dorc). Grafana reads
# this DB via GF_DATABASE_* env in grafana.service — a real drift-able adequacy substrate (dec-3).
su - postgres -c "psql -tAc \"SELECT 1 FROM pg_roles WHERE rolname='grafana'\" | grep -q 1 \
   || psql -qc \"CREATE ROLE grafana LOGIN PASSWORD 'changeme'\""
su - postgres -c "psql -tAc \"SELECT 1 FROM pg_database WHERE datname='grafana'\" | grep -q 1 \
   || createdb -O grafana grafana"

# ── 6. bring the stack up (grafana needs the db to exist first) ──
systemctl enable --now otelcol-contrib
systemctl enable --now prometheus
systemctl enable --now grafana

# ── 7. home assistant, containerised (the opaque one; host-net for device discovery) ──
# FLAG: `docker run` is non-idempotent as written — a second run errors on the existing name.
# A real lazy admin often leaves this bare (and eats the re-run error) or hand-guards with
# `docker ps`. Left bare here to stand as the honest poison-wall — the SOLE opaque hork.
docker run -d --name homeassistant --restart unless-stopped \
   -v /srv/hass:/config -v /etc/localtime:/etc/localtime:ro --network host \
   "ghcr.io/home-assistant/home-assistant:${HASS_TAG}"

# ── 8. nginx reverse-proxy + self-signed TLS (grafana :3000, prometheus :9090, HA :8123) ──
install -d -m 0700 /etc/nginx/certs
# self-signed cert; the `[ -f ]` guard is lazier than correct — it never checks EXPIRY, so a
# present-but-expired cert wrongly passes (the converged≠no-op trap). On-brand.
[ -f /etc/nginx/certs/homelab.crt ] \
   || openssl req -x509 -newkey rsa:2048 -nodes -days 365 \
        -keyout /etc/nginx/certs/homelab.key -out /etc/nginx/certs/homelab.crt \
        -subj "/CN=homelab.lan"
# only drop the vhost if it's missing (also skips UPDATES to it — lazy, on-brand). The heredoc
# body is why dorc can't cleanly edit this line's span (heredoc-refusal edge).
if [ ! -f /etc/nginx/sites-available/homelab ]; then
   cat > /etc/nginx/sites-available/homelab <<'EOF'
server {
   listen 443 ssl;
   server_name homelab.lan;
   ssl_certificate     /etc/nginx/certs/homelab.crt;
   ssl_certificate_key /etc/nginx/certs/homelab.key;
   location /grafana/    { proxy_pass http://127.0.0.1:3000/; proxy_set_header Host $host; }
   location /prometheus/ { proxy_pass http://127.0.0.1:9090/; }
   location /            { proxy_pass http://127.0.0.1:8123/; proxy_set_header Host $host; }
}
EOF
fi
ln -sf /etc/nginx/sites-available/homelab /etc/nginx/sites-enabled/homelab
rm -f /etc/nginx/sites-enabled/default
# the one careful bit: validate before reloading (the change-signal idiom, per pi-webhost)
nginx -t && systemctl reload nginx

# ── 9. firewall ──
ufw allow 22/tcp
ufw allow 443/tcp

echo "homelab up"
