#!/bin/sh
# pi-webhost provisioning (bare-mutation core; the lazy admin lets Dorc do idempotency)
set -e

apt-get update
apt-get install -y nginx

ufw allow 80/tcp
ufw allow 443/tcp

systemctl enable nginx
systemctl start nginx

nginx -t && systemctl reload nginx

echo "pi-web up"
