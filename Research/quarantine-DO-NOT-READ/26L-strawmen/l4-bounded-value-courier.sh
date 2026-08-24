#!/bin/sh
# Layer 4: control produces bounded values that become worker command arguments.
# The values are validated as data; kubeadm's printed shell command is never eval'd.

set -eu
umask 077

ssh_host() {
   host=$1
   shift
   case "$host" in
   ''|*[!A-Za-z0-9._-]*)
      printf 'refusing invalid SSH destination: %s\n' "$host" >&2
      return 64
      ;;
   esac

   ssh -T \
      -o BatchMode=yes \
      -o ClearAllForwardings=yes \
      -o ForwardAgent=no \
      -o StrictHostKeyChecking=yes \
      "$host" "$@"
}

api_endpoint=control.example.net:6443

token=$(ssh_host control 'sudo -n kubeadm token create --ttl 15m')
ca_hash=$(
   ssh_host control 'sudo -n sh -s' <<'CONTROL_READ'
set -eu
openssl x509 -pubkey -in /etc/kubernetes/pki/ca.crt |
   openssl pkey -pubin -outform DER |
   openssl dgst -sha256 -hex |
   sed 's/^.*= /sha256:/'
CONTROL_READ
)

if ! printf '%s\n' "$api_endpoint" |
   grep -Eq '^[A-Za-z0-9.-]+:[0-9]{1,5}$'
then
   printf 'refusing malformed API endpoint\n' >&2
   exit 1
fi
if ! printf '%s\n' "$token" |
   grep -Eq '^[a-z0-9]{6}\.[a-z0-9]{16}$'
then
   printf 'refusing malformed kubeadm token\n' >&2
   exit 1
fi
if ! printf '%s\n' "$ca_hash" |
   grep -Eq '^sha256:[0-9a-f]{64}$'
then
   printf 'refusing malformed Kubernetes CA hash\n' >&2
   exit 1
fi

# Validation above excludes shell metacharacters from every substituted field.
join_command="sudo -n kubeadm join $api_endpoint --token $token --discovery-token-ca-cert-hash $ca_hash"
ssh_host worker "$join_command"
