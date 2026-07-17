# render21-if-guard-query-elides (arch-1, note 214 — the if-guard elision POLE A): an `if`
set -e
if ! dpkg -s nginx >/dev/null 2>&1
then
   apt-get install -y nginx
fi
