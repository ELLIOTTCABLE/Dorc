# door1-cascade-block-elides (door-1 PAYOFF, charter 20V §4 / note 215): the
set -e
dpkg -s nginx >/dev/null 2>&1 || { sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config; systemctl restart sshd; }
