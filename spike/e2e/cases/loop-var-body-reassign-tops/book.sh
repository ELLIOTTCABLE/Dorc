# loop-var-body-reassign-tops (task-L2 item-7d, `209` brk-1(b) — the item-1 degrade):
# the body REASSIGNS the for-var (`pkg=evil`) before the install. The Members binding is the
# head binding ONLY (item-1): any body reassignment invalidates it, so this is NOT a Members
# site — the value-plane degrades, and the install takes the single-fact in-loop path, which
# the render-floor runs. The body runs every iteration. Run-set: the install twice (the
# reassigned `evil` value, once per loop iteration a/b). (Pin: the Members carve-out does not
# survive a body write to the loop var — the safe degrade.)
for pkg in a b; do pkg=evil; apt-get install -y "$pkg"; done
