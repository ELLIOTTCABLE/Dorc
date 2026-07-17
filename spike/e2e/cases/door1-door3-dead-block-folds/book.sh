# door1-door3-dead-block-folds (door-1 × door-3 d×d host-flip; charter 20V §4 / notes 215 §5
set -e
dpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }
