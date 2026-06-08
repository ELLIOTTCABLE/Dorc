# Round-17 adversarial-crosscheck strawmen — Dorc ELISION-DECISION inputs
# Each is valid sh + a comment stating the CORRECT Dorc behavior vs the failure. NOT self-asserting:
# the assertion is on Dorc's plan/elision verdict, not on a shell rc. Ledger: ../../17O-adversarial-crosscheck-findings.md

# ============================================================================
# R2-CHANGEDELTA  [CONVERGENT — the headline]  cross-kind, change-triggered effect.
# inc-7 carries per-KIND STATE transitions; it has no carrier for "a change to file: obligates a
# reload verb on service:". Dorc must couple leaf-Y (reload) elision to leaf-X (config-write) CHANGE.
# ============================================================================
webd_conf_matches() { [ -f /etc/webd/webd.conf ] && cmp -s /etc/webd/webd.conf "$1"; }  # file:#content probe
webd_active()       { systemctl is-active --quiet webd; }                                # service:#active probe

# book, change-gated reload (the idiomatic handler):
webd_conf_matches /opt/src/webd.conf.new || { cp /opt/src/webd.conf.new /etc/webd/webd.conf; cfg_changed=1; }
[ "${cfg_changed:-}" ] && systemctl reload webd
#   ASSERT(Dorc): eliding the cp (config converged) must CASCADE to not firing the reload (its
#   `cfg_changed` reader) — i.e. eliding a leaf removes its side-effects on downstream data-flow.
#   Firing the reload off a service#active==converged probe while the config is stale, OR eliding the
#   cp on a host where the config was hand-reverted, is priority-1 wrong. No host STATE distinguishes
#   "active running CURRENT config" from "active running STALE config".

# book, unconditional reload (the redis restart_service form):
#   cp /opt/src/webd.conf.new /etc/webd/webd.conf ; systemctl reload webd
#   ASSERT(Dorc): the reload is elidable ONLY via the (absent) cross-kind edge file:->service:.
#   Without it: run-every-bump (over-execute, value-prop loss) or wrong service#active elision (under-execute).

# ============================================================================
# R2-PROBEGATE  [adversarial]  a probe's RESULT gates a downstream probe/mutation subtree.
# The plan cannot be built leaf-independently. (The concrete example the human asked for.)
# ============================================================================
if getent group app >/dev/null 2>&1; then          # PROBE-1 (read-only): does group:app exist?
   if ! id -nG deploy | grep -qw app; then          # PROBE-2 (read-only): is deploy NOT already in app?
      systemctl is-active --quiet app \              # PROBE-3 — reached ONLY if PROBE-2 was true,
         || systemctl start app                      #          and its fallback `start` is a MUTATOR
      usermod -aG app deploy                          # MUTATOR — reached only if PROBE-1 ∧ PROBE-2
   fi
fi
#   THE PLAN PROBLEM (Dorc must present a coherent plan BEFORE any apply):
#   whether PROBE-3 / usermod are "live" depends on the RUNTIME results of PROBE-1, PROBE-2 — which
#   Dorc has not run yet. Three options, each costs:
#     (a) conservative: assume the whole block live => nothing elides => plan == script (no value,
#         exactly where guards nest — the careful-author target audience).
#     (b) speculate both branches: ship PROBE-3 even where PROBE-2 is false (a DEAD branch). If the
#         only precondition-probe is side-effecting (self-vouch / writes a lockfile) that is a mutation
#         in a dead branch => kFAIL-withhold breach; and PROBE-3's validity may need a not-yet-applied
#         mutation (the group existing) => wrong verdict.
#     (c) serialize: PROBE-1 -> result -> decide PROBE-2 -> result -> PROBE-3 => one network round-trip
#         PER GATE DEPTH => the cross-network big-O that dominates, defeating flat parallel probing (kFLATTEN).
#   Prior art: Ansible check-mode "will not generate output for tasks that use conditionals based on
#   registered variables" — it cannot predict past a result-gated `when:`. Dorc PROMISED a plan there;
#   17N (and fw-3) assume leaf-independent probes. It must state which of (a)/(b)/(c) it picks, and own it.

# ============================================================================
# R2-CONTEXT  [CONVERGENT; human: DEFER]  execution-context is part of a cell's identity.
# ssh/docker = eval-class pathological (defer/wontfix); sudo/become = future FIRST-CLASS (cf. Ansible become:).
# ============================================================================
deploy_in_app() { id -nG deploy 2>/dev/null | grep -qw app; }   # reads THIS host/user's resolved set
# same argv, DIFFERENT cells:
sudo -u deploy npm ls -g --depth=0 >/dev/null 2>&1 || sudo -u deploy npm install -g @acme/cli  # npm-of-USER:deploy
npm ls -g --depth=0 >/dev/null 2>&1 || npm install -g @acme/cli                                  # npm-of-USER:root
#   ASSERT(Dorc): a probe lifted WITHOUT the `sudo -u deploy` context reads root's cells and
#   mis-discharges deploy's => under-execute. The context must be part of the kind handle (sudo: deserves
#   first-class language support later). For `ssh host CMD` / `docker exec`, entering a not-yet-applied
#   context during the PROBE phase is a kFAIL-withhold network side-effect — eval-class, defer.

# ============================================================================
# F-ALGEBRA  [round 1]  the >=enum floor forces a STRUCTURED (not flat) entity key.
# ============================================================================
svc_enabled() { systemctl is-enabled --quiet -- "$1"; }   # service:#enabled
svc_active()  { systemctl is-active  --quiet -- "$1"; }    # service:#active
svc_enabled nginx || systemctl enable  -- nginx
svc_active  nginx || systemctl restart -- nginx
#   ASSERT(Dorc): a flat service:nginx key that lets is-enabled==true discharge is-active is priority-1
#   wrong on an enabled-but-crashed host. The key must be structured {enabled, active}.

# ============================================================================
# Oracle-QUALITY class  (human 2026-06-08: "good sh, sometimes hard; not too concerning — find a better,
# battle-tested spelling").  F-BLESSED, R2-IDCACHE, F-GETENT-HOSTS.
# ============================================================================
# F-BLESSED: the honest >=enum service probe is TWO commands (svc_enabled + svc_active above), not one
#   'read the kind off the command'. Some kinds (group-membership) have no single clean blessed probe.
# R2-IDCACHE: probe membership via the AUTHORITATIVE, hermetic source, never the cached `id -nG`:
member_authoritative() { getent group "$2" | cut -d: -f4 | tr ',' '\n' | grep -qx "$1"; }  # /etc/group truth
#   `id -nG` reads the resolved/cached set (stale until relogin; nscd/sssd) — must NOT license a skip.
# F-GETENT-HOSTS: getent passwd/group are file-backed (hermetic); getent hosts/ahosts hit DNS (non-hermetic)
#   — bless per-DATABASE, not by name-pattern (task #13).
host_user_exists() { getent passwd "$1" >/dev/null 2>&1; }   # OK: hermetic
# host_known()     { getent hosts  "$1" >/dev/null 2>&1; }   # NOT hermetic — live DNS; do not bless as read-only
