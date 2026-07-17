# guard23-explicit-rc-consumers-run (the narrowest, uncontested slice of the rc-consumer
if apt-get install -y nginx; then echo ok; fi
apt-get install -y curl || echo fallback
apt-get install -y vim; rc=$?
echo "rc was $rc"
