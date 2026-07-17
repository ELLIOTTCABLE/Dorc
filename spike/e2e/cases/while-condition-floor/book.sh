# while-condition-floor (task-L1 item-4b): a `while` loop whose CONDITION's status is
set -e
while dpkg -s nginx; do echo installing nginx; done
apt-get install -y curl
