#!/bin/sh
# ── book: bring up my pi as a webhost ──────────────  [STRAWMAN, parser fixture]
# Verbatim copy of Research/strawmen/books/pi-webhost.straw.sh (worktree 01Xpbd),
# used as the spike parser/analyzer's first real-world fixture — the do-4 dogfood
# book. Scrappy, target-specific, low-reuse: the lazy-admin end. Pure POSIX (runs
# in dash). Quality is deliberately uneven: some idempotency guards an admin
# writes anyway (the parts Dorc can lift), and some bare mutations with no guard
# at all (where Dorc would borrow a probe from an oracle to decide whether to skip).

set -e

# only my web boxes; bail harmlessly elsewhere (host-selection idiom)
case "$(hostname)" in
   pi-web*|webhost-*) : ;;
   *) echo "not a webhost ($(hostname)); nothing to do"; exit 0 ;;
esac

# a guard I'd write anyway -> Dorc can lift this whole block as one probe
if ! command -v nginx >/dev/null 2>&1; then
   apt-get update
   apt-get install -y nginx
fi

# ...but these just fire every run, no guard — the bare mutations:
ufw allow 80/tcp
ufw allow 443/tcp
systemctl enable --now nginx

# only drop the vhost if it's missing. note: this also skips UPDATES to the
# config — lazier than correct, which is exactly on-brand for a book.
if [ ! -f /etc/nginx/sites-enabled/pi-web.conf ]; then
   cat > /etc/nginx/sites-available/pi-web.conf <<'EOF'
server {
   listen 80;
   server_name _;
   root /srv/pi-web;
}
EOF
   ln -sf /etc/nginx/sites-available/pi-web.conf /etc/nginx/sites-enabled/pi-web.conf
fi

# the one careful bit: validate before reloading (the change-signal idiom)
nginx -t && systemctl reload nginx

# crude "done" marker so I can eyeball it later (not a real convergence check)
touch /var/lib/pi-web.provisioned
echo "pi-web up"
