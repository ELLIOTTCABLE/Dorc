#!/bin/sh
# pi-webhost provision — a scrappy real book (the lazy admin lets dorc do idempotency).
set -e

dpkg -s ca-certificates >/dev/null 2>&1

dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx

apt-get update
apt-get install -y curl
apt-get install -y htop

systemctl enable nginx
systemctl start nginx

dpkg -s vim >/dev/null 2>&1 || apt-get install -y vim

ufw allow 80/tcp
