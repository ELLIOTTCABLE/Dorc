#!/bin/sh
# Layer 6: Advil's observation licenses omission of Beverly's probe and mutation.
# The admin treats equal EC2 image IDs as an equivalence claim. Runtime drift makes
# that claim dubious; the point here is to spell the cross-host omission mechanism.

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

advil_instance=i-0123456789abcdef0
beverly_instance=i-0fedcba9876543210
desired_nginx_version=1.22.1-9+deb12u3

image_id() {
   aws ec2 describe-instances \
      --instance-ids "$1" \
      --query 'Reservations[0].Instances[0].ImageId' \
      --output text
}

advil_image=$(image_id "$advil_instance")
beverly_image=$(image_id "$beverly_instance")

for image in "$advil_image" "$beverly_image"; do
   if ! printf '%s\n' "$image" | grep -Eq '^ami-[0-9a-f]+$'; then
      printf 'refusing malformed EC2 image identity: %s\n' "$image" >&2
      exit 1
   fi
done

advil_version=$(ssh_host advil "dpkg-query -W -f='\${Version}\\n' nginx")

if [ "$advil_image" = "$beverly_image" ] &&
   [ "$advil_version" = "$desired_nginx_version" ]
then
   printf '%s\n' \
      'omitting beverly: advil passed and both hosts claim the same EC2 image identity'
   exit 0
fi

# Beverly is contacted only when Advil or the external equivalence claim did not license omission.
ssh_host beverly 'sudo -n sh -s' <<'BEVERLY_BOOK'
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y nginx=1.22.1-9+deb12u3
nginx -t
systemctl restart nginx
BEVERLY_BOOK
