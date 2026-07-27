#!/bin/sh
# Turn a bare Debian container into something dorc can ssh into.
#
# Not run on your machine — piped INTO a container, with your public key as its one argument:
#
#    docker exec -i mybox sh -s -- "$(cat ~/.ssh/id_ed25519.pub)" < spike/e2e/container-sshd.sh
#
# (`wslc`, `podman` and `nerdctl` all spell that the same way.) The container must already be
# running with port 22 published — `docker run -d --name mybox -p 2222:22 debian:12-slim sleep
# infinity` — after which `root@localhost:2222` is a destination `dorc --host` accepts.
#
# `mise run livetest:target` does all of this for you, including minting a throwaway keypair. This
# file is for when you want the container to be yours rather than the harness's.

set -eu

if [ "${1-}" = "" ]; then
   printf 'container-sshd: pass your PUBLIC key as the first argument\n' >&2
   exit 2
fi

export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq openssh-server >/dev/null

mkdir -p /root/.ssh /run/sshd
printf '%s\n' "$1" >/root/.ssh/authorized_keys
chmod 700 /root/.ssh
chmod 600 /root/.ssh/authorized_keys

/usr/sbin/sshd

printf 'sshd up; root login enabled for the key you passed\n'
