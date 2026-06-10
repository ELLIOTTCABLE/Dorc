# sourcing-degrades-safely (19I floor / 20B §3 boundary strawman): `. helper.sh` is a
# literal-target source — KEPT (not ⊤-rejected, unlike a DYNAMIC `. "$f"`), but UNMODELED:
# dorc cannot see what the sourced file does, so it is an OPAQUE command (a write-or-unknown
# that poisons downstream ambient-ness). The install BELOW it therefore runs VERBATIM despite
# the fixture reporting it converged (the bait) — the SAFE degrade (kFAIL-perform): an opaque
# effect never silently licenses a downstream elision. (helper.sh is shipped in mocks/ so the
# no-slash `.` finds it via PATH at exec time; it sources to a harmless no-op.) The run-set
# proves the install runs.
. helper.sh
apt-get install -y nginx
