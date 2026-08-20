#!/bin/sh
install_both() {
   apt-get install -y nginx
   apt-get install -y curl
}

install_both
