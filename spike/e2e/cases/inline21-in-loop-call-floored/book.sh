# inline21-in-loop-call-floored (arch-2 + task-L1 composition — the in-loop floor holds for an
w() { apt-get install -y "$1" >/dev/null 2>&1; }
for pkg in nginx; do w "$pkg"; done
