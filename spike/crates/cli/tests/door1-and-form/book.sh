# door1-and-form (door-1 && DUAL, charter 20V §4 / note 215): the `&&` direction of the
set -e
dpkg -s nginx >/dev/null 2>&1 && { systemctl stop nginx; rm -f /etc/nginx/sites-enabled/default; }
