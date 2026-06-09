# fold-oror-guard-omits (round-19, the canonical idempotency idiom): a read-only
# guard short-circuits the mutator. `command -v nginx` (rc 0 when nginx is on PATH)
# proves the `|| apt-get install` branch DEAD. The apply fold reads the guard's known
# rc 0 and OMITS the install, collapsing the whole line to its value-preserving
# stand-in `true` (the guard's rc). Nothing runs (expected.ran is empty). The DESIGN
# `dpkg -s nginx || apt-get install nginx` idiom, with `command -v` as the guard.
command -v nginx >/dev/null 2>&1 || apt-get install -y nginx
