# door1-door3-inner-elides (door-1 × door-3 COMPOSITION, charter 20V §4 / note 215 §5 hunt-C):
set -e
dpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }
