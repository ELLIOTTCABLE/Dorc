# strawman24-derived-survive (24E Stage 4 — the DERIVED-footprint flagship; sibling to
# strawman24-survive-simple, which uses an AUTHORED footprint). The apt oracle's touches() is now
# PAYLOAD-BOUND: it derives its footprint via the NATURAL pipe idiom `dpkg -L "$1" | sed 's|^|file:|'`
# (24E §14 — the parser ACCEPTS the pipe, the tracer ⊤s on it ⇒ escalate), so the footprint is
# DERIVED at probe time (24E §2) instead of authored statically.
#   site 0  apt-get install oldpkg — DIVERGED (absent) ⇒ RUNS. A running modeled mutator IS a
#           wall — but its touches() ESCALATED (NonPrintfCommand ⊤ on the pipeline), shipped to
#           the probe lane byte-exact, ran read-only, and its stdout derived {file:/etc/oldpkg.conf}
#           (`dpkg -L | sed` alone); the engine UNIONS the wall's own establish coord package:oldpkg
#           (24G §8) ⇒ hit-surface {package:oldpkg, file:/etc/oldpkg.conf}.
#   site 1  apt-get install nginx — CONVERGED (holds), PAST the running oldpkg wall. Its backing
#           (package:nginx) is DISJOINT from the DERIVED footprint (package:oldpkg — same kind,
#           different entity; and file:/etc/oldpkg.conf — a different kind) ⇒ under
#           --trust-footprints it ELIDES past the running wall (the frame rule, dynamic frame).
#           The yardstick goes 0->1 vs the honest baseline (the UNFLAGGED sibling); the why-lens
#           names the licensor AND its DERIVED provenance (24E §9). Differential: expected.ran +
#           the dual-rail judge pin the run-set; the elision is licensed by a probe-DERIVED footprint.
apt-get install -y oldpkg
apt-get install -y nginx
