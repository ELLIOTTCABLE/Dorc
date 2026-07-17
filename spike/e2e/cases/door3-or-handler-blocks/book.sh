# door3-or-handler-blocks (door-3 NEGATIVE pole, charter 20V §4 / note 213): `cmd || { …; }`
set -e
apt-get install -y nginx || { printf 'recovering\n'; }
