# guard23-reingest-collision-verbatim (the off-ramp closure — a PASSING floor). This book
# IS a guarded apply artifact fed back to dorc: the preamble function (the oracle's check
# body, stripped) plus the `check || original` line — exactly what the guard tier will
# emit, hand-written today. Three pins in one:
#  (1) re-analysis is SAFE and quiet: the artifact re-parses as plain sh (off-ramp holds),
#      nothing crashes, and the line runs VERBATIM (the un-oracled function call is opaque
#      => conservative run; "safe, merely unimproved", notes/218a d4-6);
#  (2) NO ACCRETION: re-analysis must never stack `check || (check || cmd)` — the book
#      already carries the guard (an inserted guard is recognized the same way a hand-
#      written one is; plans/233 upsides list);
#  (3) NAME COLLISION: the book defines `apt_get__check` — the very name the guard tier
#      would ship. sh function redefinition is last-writer-wins, so a shipped preamble
#      after this definition would be hijacked (or hijack); and a collision-dodging RENAME
#      is unspellable under strip-only sourcing (rul-ternary-verdict: `name.check()` ->
#      `name_check()`, nothing else changed) — so the only lawful verdicts here are
#      refuse-and-run. On this mock host nginx is ABSENT (dpkg-query mock: rc 1), so at exec
#      the book carried guard falls through and the install runs, exactly as bare sh would.
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg="$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
}
apt_get__check install -y nginx || apt-get install -y nginx
