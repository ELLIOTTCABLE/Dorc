#!/bin/sh
main() {
   apt-get install -y nginx
   apt-get install -y curl
}

main "$@"
