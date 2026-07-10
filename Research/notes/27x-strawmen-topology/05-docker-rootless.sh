# dorc-lang/v0.1

# ============================================================================
# 05 — Docker: rootless vs system daemon                (STRAWMAN, never run)
#
# THE REAL-WORLD FACTS. Which daemon a docker CLI call hits is decided by
# the CLIENT, per invocation: $DOCKER_HOST if set; else the user's selected
# context (~/.docker/config.json + contexts/); else the rootless socket
# /run/user/$UID/docker.sock where rootless is set up; else the system
# socket /var/run/docker.sock. So the "same" command is per-user state on a
# rootless host and SHARED SYSTEM STATE on a plain host — the topology
# depends on host configuration, per-user configuration, AND env.
#
# WHY NON-TRIVIAL: this is the case that is flatly INEXPRESSIBLE for Shape A,
# and it is also the case that VALIDATES the never-derive-separation carve
# with a live counterexample rather than a hypothetical.
# ============================================================================

# The book lines under analysis (run as alice):
docker ps --filter name=web --format '{{.Names}}' | grep -q web || docker compose up -d web
sudo docker system prune -f          # the wrapped mutation, drifted-day wall

# ---------------------------------------------------------------------------
# SHAPE A: pick a token, any token — and be wrong somewhere:
#    : sm.dorc.DockerEngine user=invariant   # WRONG on every rootless host
#    : sm.dorc.DockerEngine user=sensitive   # wrong-shaped on system-daemon
#                                            # hosts (alice and root DO share
#                                            # one daemon there)
# The flat token has no way to say "it depends on the host". A's honest move
# is `sensitive` (safe under the carve: keying without separation ⇒ walls,
# value-dead) — inexpressiveness priced as permanent value loss.
#
# SHAPE D: spell the client's actual selection logic:
sm_dorc_DockerEngine__lives_at() {
   if [ -n "${DOCKER_HOST-}" ]
   then printf '%s\n' "$DOCKER_HOST"                        # ρ/env-plane
   elif [ -S "/run/user/$(id -u)/docker.sock" ]             # host question!
   then printf '/run/user/%s/docker.sock\n' "$(id -u)"      # rootless: per-user
   else printf '/var/run/docker.sock\n'                     # system: shared
   fi
}
# (Deliberately ignoring tcp:// remotes: an off-host address is the host
# axis, someday-never — that branch would decline, rc 2 ⇒ floor.)
# ---------------------------------------------------------------------------

# DERIVATION WALKTHROUGH (v1, conservative). The body contains a user-axis
# input ($(id -u)) on a REACHABLE branch, and the branch condition is host
# state (the -S test) that static analysis cannot resolve. Sound v1 rule:
# any reachable user-axis dependence ⇒ NOT invariant ⇒ keyed-per-user,
# everywhere. On a plain system-daemon host this over-keys (alice's probe
# and root's site genuinely share /var/run/docker.sock, and we decline to
# bridge them) — conservative, value-lost, safe.
#
# THE CARVE, VALIDATED (the exercise's sharpest single result). Suppose the
# carve did NOT exist and sensitivity derived a separation license: on the
# plain host, alice's probed container-fact would be held "provably
# disjoint" from root's `sudo docker system prune` — and would SURVIVE that
# running wall under the trust flag, while the prune actually destroys the
# very state the fact rests on. Silent under-execution, in the most ordinary
# configuration docker has. That is no hypothetical aliasing hazard; it is
# the DEFAULT INSTALL. never-derive-separation graduates from precaution to
# demonstrated necessity.
#
# THE DEFERRED UPSIDE (finding f8, banked not built). D's body contains
# enough truth for a PER-HOST refinement: the probe lane could evaluate the
# `-S` branch on each host and classify per-host (rootless absent ⇒ the
# address is the shared socket ⇒ invariant ON THAT HOST ⇒ bridging returns).
# But a per-host classification is a fact ABOUT probe-time host state, so it
# inherits the chronology problem (a mid-book `apt-get install
# docker-rootless` upstream invalidates it) — the exact planes-meet-at-
# chronology cell the two-observation-planes note flagged for task 7.
# Explicitly deferred alongside it; v1 takes the conservative static answer.
#
# VERDICT. D expresses what A cannot (and its conservative projection is
# still ≥ A's best honest token); the carve is load-bearing on real hosts;
# and the per-host refinement is a named, deferred, chronology-priced
# future — not v1 scope creep.
