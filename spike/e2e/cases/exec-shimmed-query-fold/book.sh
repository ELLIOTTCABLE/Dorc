# exec-shimmed-query-fold (task-P/item-3, 20I find-2): the `dpkg -s X || apt-get install X`
set -e
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx
