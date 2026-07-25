# loop-var-body-reassign-tops (task-L2 item-7d, `209` brk-1(b) — the item-1 degrade):
for pkg in a b; do pkg=evil; apt-get install -y "$pkg"; done
