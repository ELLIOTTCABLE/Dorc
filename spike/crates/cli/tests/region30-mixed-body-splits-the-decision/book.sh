#!/bin/sh
main() {
   apt-get install -y nginx
   hork tune-packages
   apt-get install -y curl
}

main "$@"
