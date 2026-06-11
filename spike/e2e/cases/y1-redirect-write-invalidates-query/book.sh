set -e
printf 'x' >> nginx.conf
grep -q x nginx.conf || apt-get install -y nginx
