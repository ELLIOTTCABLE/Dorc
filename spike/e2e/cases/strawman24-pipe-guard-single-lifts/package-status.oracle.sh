# package-status QUERY oracle: `dpkg -s <pkg>` READS installed-status — a read-only Query of
# pkgstate:<pkg>#installed (task-D2 `query` polarity, `:?`). Verbless; `-s` is a stripped flag.
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : pkgstate = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? pkgstate:"$pkg".installed
}
