# guard23-explicit-rc-consumers-run (the narrowest, uncontested slice of the rc-consumer
# question — a PASSING floor). Three converged, VOUCHED sites whose exit status the admin's
# own sh explicitly reads: an `if` condition, a `||` left operand with a written fallback,
# and a `$?` reader. The admin's spelled intent wins, always: a guard's pass-direction
# would swap the tool's rc for the predict's rc under the reader (a wrong-vouch suppressing
# a written `|| fallback` is the stacked-failure disaster, notes/218a hunt-B), so NONE of
# these sites may ever mint a guard — they run, vouch or no vouch, converged or not.
# (Deliberately narrow: whether errexit-IMPLICIT consumption also blocks guarding is
# DEFERRED by human ruling 2026-07-02 pending experimentation — no book here sets -e, and
# no pin in the guard23 set asserts either answer to that question; Research/notes/23A.)
if apt-get install -y nginx; then echo ok; fi
apt-get install -y curl || echo fallback
apt-get install -y vim; rc=$?
echo "rc was $rc"
