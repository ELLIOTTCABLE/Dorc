#!/bin/sh
set -e

# Create plik system user and group if they don't exist
if ! getent group plik >/dev/null 2>&1; then
  groupadd --system plik
fi

if ! getent passwd plik >/dev/null 2>&1; then
  useradd --system \
    --gid plik \
    --home-dir /var/lib/plik \
    --shell /usr/sbin/nologin \
    --comment "Plik file upload server" \
    plik
fi

# Fix ownership of data directories
chown -R plik:plik /var/lib/plik

# Adjust default config paths for Debian layout
CFG=/etc/plik/plikd.cfg
if grep -q 'WebappDirectory.*= "../webapp/dist"' "$CFG" 2>/dev/null; then
  sed -i 's|WebappDirectory.*= "../webapp/dist"|WebappDirectory     = "/usr/share/plik/webapp/dist"|' "$CFG"
  sed -i 's|ClientsDirectory.*= "../clients"|ClientsDirectory    = "/usr/share/plik/clients"|' "$CFG"
  sed -i 's|ChangelogDirectory.*= "../changelog"|ChangelogDirectory  = "/usr/share/plik/changelog"|' "$CFG"
  sed -i 's|Directory = "files"|Directory = "/var/lib/plik/files"|' "$CFG"
  sed -i 's|ConnectionString = "plik.db"|ConnectionString = "/var/lib/plik/plik.db"|' "$CFG"
fi

# Reload systemd and enable (but don't start) the service
if command -v systemctl >/dev/null 2>&1; then
  systemctl daemon-reload
  systemctl enable plikd.service || true
fi
