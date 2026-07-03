# strawman24-adequacy-seed (plans/240 Stage-1 — the CALIBRATION TARGET, 23O §2). The single most
# valuable seed in the family: a book that ELIDES at HEAD, correctly per the differential, yet
# whose elision is INADEQUATE.
#   apt-get install nginx — the probe (`dpkg-query -W nginx`) reports INSTALLED ⇒ converged ⇒ the
#   line elides. But `apt-get install` is converged≠no-op: on a host where nginx is installed BUT
#   a newer version is pending, the real command would UPGRADE (act) — a mutation the installed-
#   only probe cannot see. So the elision silently UNDER-EXECUTES (the converged-vouch's whole
#   residual danger; calibrated-never-proven — 240).
# WHY THE DIFFERENTIAL IS BLIND HERE, and what Stage-6 measurement will need: the mocks are
# STATE-INDEPENDENT — the dpkg-query shim answers "installed" and the apt-get shim just logs;
# neither models "a newer version exists", so the bare book and the elided apply reach the
# IDENTICAL mock end-state ⇒ the exec-differential goes green either way, seeing no bite. To make
# the bite VISIBLE, Stage-6 needs a STATE-BEARING mock (a host-state axis): a dpkg-query that
# distinguishes installed-version from candidate-version, an apt-get shim whose logged effect
# DEPENDS on that state, and a probe/vouch that observes VERSION, not just presence — then the
# bare book upgrades while the elided apply does not, and the differential finally goes red.
# Until then this case carries the adequacy semantics forward — GREEN, not xfail: the elision IS
# what HEAD does, and measuring how often it bites is the round's sharpest empirical deliverable.
apt-get install -y nginx
