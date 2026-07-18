#!/usr/bin/env bash
# scripts/server/01-harden.sh
# Baseline security hardening for the onService PH production server (Ubuntu 24.04).
# Idempotent — safe to re-run. Run as root.
set -euo pipefail

echo "==> [1/9] System update"
export DEBIAN_FRONTEND=noninteractive
apt-get update -y
apt-get upgrade -y

echo "==> [2/9] Core tools"
apt-get install -y ufw fail2ban unattended-upgrades curl ca-certificates gnupg \
  lsb-release apt-transport-https software-properties-common jq git

echo "==> [3/9] Automatic security updates"
cat >/etc/apt/apt.conf.d/20auto-upgrades <<'EOF'
APT::Periodic::Update-Package-Lists "1";
APT::Periodic::Unattended-Upgrade "1";
APT::Periodic::AutocleanInterval "7";
EOF
cat >/etc/apt/apt.conf.d/50unattended-upgrades <<'EOF'
Unattended-Upgrade::Allowed-Origins {
  "${distro_id}:${distro_codename}-security";
  "${distro_id}ESMApps:${distro_codename}-apps-security";
  "${distro_id}ESM:${distro_codename}-infra-security";
};
Unattended-Upgrade::Automatic-Reboot "false";
Unattended-Upgrade::Remove-Unused-Dependencies "true";
EOF
systemctl enable --now unattended-upgrades >/dev/null 2>&1 || true

echo "==> [4/9] Firewall (allow only SSH/HTTP/HTTPS)"
ufw --force reset >/dev/null
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp    comment 'SSH'
ufw allow 80/tcp    comment 'HTTP (redirects to HTTPS + certbot)'
ufw allow 443/tcp   comment 'HTTPS'
ufw --force enable
ufw status verbose

echo "==> [5/9] fail2ban (SSH brute-force protection)"
cat >/etc/fail2ban/jail.local <<'EOF'
[DEFAULT]
bantime  = 1h
findtime = 10m
maxretry = 5
backend  = systemd

[sshd]
enabled = true
port    = ssh
EOF
systemctl enable --now fail2ban
systemctl restart fail2ban

echo "==> [6/9] SSH hardening (key-only, no passwords, no root password login)"
mkdir -p /etc/ssh/sshd_config.d
cat >/etc/ssh/sshd_config.d/99-onservice-hardening.conf <<'EOF'
# onService hardening — key-only auth, no passwords.
PermitRootLogin prohibit-password
PasswordAuthentication no
KbdInteractiveAuthentication no
ChallengeResponseAuthentication no
PubkeyAuthentication yes
PermitEmptyPasswords no
X11Forwarding no
MaxAuthTries 3
LoginGraceTime 30
ClientAliveInterval 300
ClientAliveCountMax 2
EOF
# Validate before applying so we never lock ourselves out.
sshd -t
systemctl restart ssh || systemctl restart sshd

echo "==> [7/9] Kernel/network sysctl hardening"
cat >/etc/sysctl.d/99-onservice-hardening.conf <<'EOF'
net.ipv4.conf.all.rp_filter=1
net.ipv4.conf.default.rp_filter=1
net.ipv4.icmp_echo_ignore_broadcasts=1
net.ipv4.conf.all.accept_redirects=0
net.ipv6.conf.all.accept_redirects=0
net.ipv4.conf.all.send_redirects=0
net.ipv4.conf.all.accept_source_route=0
net.ipv6.conf.all.accept_source_route=0
net.ipv4.tcp_syncookies=1
kernel.kptr_restrict=2
EOF
sysctl --system >/dev/null

echo "==> [8/9] Swap (2G safety, idempotent)"
if [ ! -f /swapfile ]; then
  fallocate -l 2G /swapfile
  chmod 600 /swapfile
  mkswap /swapfile >/dev/null
  swapon /swapfile
  echo '/swapfile none swap sw 0 0' >>/etc/fstab
  echo 'vm.swappiness=10' >/etc/sysctl.d/99-swappiness.conf
  sysctl -p /etc/sysctl.d/99-swappiness.conf >/dev/null
fi

echo "==> [9/9] Deploy user (non-root, sudo, same SSH key)"
if ! id deploy >/dev/null 2>&1; then
  adduser --disabled-password --gecos "" deploy
  usermod -aG sudo deploy
  echo 'deploy ALL=(ALL) NOPASSWD:ALL' >/etc/sudoers.d/90-deploy
  chmod 440 /etc/sudoers.d/90-deploy
  mkdir -p /home/deploy/.ssh
  cp /root/.ssh/authorized_keys /home/deploy/.ssh/authorized_keys
  chown -R deploy:deploy /home/deploy/.ssh
  chmod 700 /home/deploy/.ssh
  chmod 600 /home/deploy/.ssh/authorized_keys
fi

echo "HARDENING_DONE"
