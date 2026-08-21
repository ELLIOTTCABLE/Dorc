#!/bin/sh
install_pkg() {
   apt-get install -y "$1"
}

install_pkg nginx
install_pkg curl
