# door1-cascade-diverged-runs (door-1 POLE, charter 20V §4 / note 215): the SAME book as
set -e
dpkg -s nginx >/dev/null 2>&1 || { sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config; systemctl restart sshd; }
