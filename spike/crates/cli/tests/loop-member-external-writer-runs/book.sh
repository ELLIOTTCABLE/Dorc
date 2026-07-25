# loop-member-external-writer-runs (task-L2 item-7c, `209` brk-1(b) — the self-reach core):
apt-get purge -y curl
for pkg in nginx curl; do apt-get install -y "$pkg"; done
