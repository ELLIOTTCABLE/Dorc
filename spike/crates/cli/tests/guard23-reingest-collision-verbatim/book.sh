# guard23-reingest-collision-verbatim (the off-ramp closure — a PASSING floor). This book
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg="$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
}
apt_get__predict install -y nginx || apt-get install -y nginx
