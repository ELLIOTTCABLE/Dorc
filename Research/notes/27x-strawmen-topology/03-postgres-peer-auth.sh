# dorc-lang/v0.1

# ============================================================================
# 03 — PostgreSQL: peer auth, PGDATA, the r25 wall      (STRAWMAN, never run)
#
# THE REAL-WORLD FACTS. The r25 field-trial book's permanent wall:
# `su - postgres -c 'psql -c "CREATE ROLE app LOGIN"'`. Cluster state lives
# in the data directory (${PGDATA:-/var/lib/postgresql/16/main}); the FACT
# "role app exists" is rows in catalog tables inside it. Reading via psql is
# peer-auth-gated (you must BE postgres). But cluster METADATA is not:
# `pg_lsclusters` is world-readable (reads /etc/postgresql + /var/lib
# listings).
#
# WHY NON-TRIVIAL: state that is genuinely one referent (user-invariant!)
# yet unreadable from outside — the case that looks like it should break the
# invariance license.
# ============================================================================

# The book lines under analysis (run as alice):
su - postgres -c 'psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='"'"'app'"'"'" | grep -q 1 || createuser app'
sudo pg_ctlcluster 16 main start

# ---------------------------------------------------------------------------
# SHAPE A:   : sm.dorc.PgCluster user=invariant
# SHAPE D:
sm_dorc_PgCluster__lives_at() {
   printf '%s\n' "${PGDATA:-/var/lib/postgresql/16/main}"
   printf '/etc/postgresql\n'
}
# Derivation: no user-axis-owned input (PGDATA is a ρ VARIABLE, env-plane,
# not the user axis) ⇒ INVARIANT. And that is TRUE: the cluster is one
# referent no matter who asks. Both shapes land the same classification;
# D additionally keys correctly under `env PGDATA=/srv/pg …` for free
# (the address resolves per-site-ρ — the opaques7-finding17 behavior).
# ---------------------------------------------------------------------------

# THE APPARENT PARADOX, RESOLVED (finding f6: referent-topology ≠ access).
# Invariance grants the probe-outside LICENSE. For the catalog fact behind
# line 1, the licensed probe (psql as alice) runs and is REFUSED — peer auth
# denied, rc ≥ 2 ⇒ can't-say ⇒ the site runs, guarded by the admin's own
# `| grep -q 1 ||` shape. No escalation happened, no wrong verdict landed:
# LICENSE ≠ ABILITY, and imp-1 is enforced by rc-reality per-probe, not by
# the topology declaration. The two concerns never needed to be one
# mechanism.
#
# AND THE LICENSE STILL PAYS, on the same kind: pg_lsclusters-backed facts
# (cluster exists, version, port, online) are world-readable — probed as
# alice, they bridge via the SAME invariance to root-context sites like
# line 2 (`sudo pg_ctlcluster 16 main start` elides when the cluster is
# already online). Access-gating is per-PROBE; topology is per-kind; the
# split is load-bearing and both shapes respect it identically.
#
# THE WALL-BOUNDING DIVIDEND needs no topology at all: when line 1 really
# runs (drifted day), its footprint claims key to PgCluster cells, which are
# CROSS-KIND disjoint-by-construction from the book's ufw/file/service facts
# — the wall is kind-granular with or without any user-axis declaration.
#
# VERDICT. Ties. Both shapes say "invariant" and both are right; the
# interesting machinery (per-probe access failure, cross-kind bounding,
# ρ-keying of PGDATA) all lives OUTSIDE the trichotomy spelling. Postgres
# stresses the DESIGN and both shapes pass; it does not separate them —
# recorded as evidence that the imp-1 composition is graceful, which was a
# standing worry (`24S` §2c class-3).
