# door1-cascade-multistatement (door-1 NESTED REGION, charter 20V §4 / note 215): the
set -e
dpkg -s nginx >/dev/null 2>&1 || {
   sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config
   if [ -f /etc/ssh/sshd_config.bak ]; then cp /etc/ssh/sshd_config /etc/ssh/sshd_config.bak; fi
   systemctl restart sshd
}
