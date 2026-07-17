# inline21-wrapper-diverged-runs (arch-2 — the diverged pole; calls are INDEPENDENT):
apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }
apt_install nginx
apt_install curl
