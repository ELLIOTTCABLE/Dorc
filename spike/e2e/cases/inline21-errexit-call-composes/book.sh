# inline21-errexit-call-composes (arch-2 `i-5` errexit composition × silence=wall `23Ib-fd10`):
set -e
apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }
apt_install nginx
apt_install curl || true
