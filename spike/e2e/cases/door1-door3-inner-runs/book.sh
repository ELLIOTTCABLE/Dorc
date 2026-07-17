# door1-door3-inner-runs (door-1 × door-3 d×d composition; charter 20V §4 / notes 215 §5/§7
set -e
dpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }
