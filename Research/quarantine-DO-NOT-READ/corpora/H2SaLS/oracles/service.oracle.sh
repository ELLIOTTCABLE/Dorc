# LLM-GENERATED ORACLE SEED — intentionally quality-varied artificial testing
# corpus for a static-analysis project (Dorc). NOT real ops code; FROZEN EVIDENCE,
# NEVER EXECUTE. An artificial oracle cannot expose the truth of real-world ops-code.
# Validation is `dash -n` (parse-only) plus reading.
#
# Provider is `service` (the SysV/`service(8)` wrapper) — the book NEVER uses
# `systemctl`. The book's ONLY verb is `restart` (§3 L188 ufw; §10 L628 auditd;
# end-of-play handlers L686/L689/L692/L695 ssh/ufw/psad/fail2ban).
#
# THE CENTRAL FINDING (um-svc-1): a `restart` is NOT host-state-probeable, so it is
# given NO effect cell and ALWAYS runs. Every restart in this book exists to make a
# daemon RE-READ a config file the script just edited (sshd_config, ufw rules,
# jail.local, audit.rules). The convergence target is "the LIVE process has loaded the
# CURRENT on-disk config" — and no read-only host fact exposes that:
#   - `service <svc> status` rc tells you the daemon is RUNNING (#active), NOT that its
#     loaded config matches disk. A daemon running STALE config still reads #active.
#   - So an `#active`-keyed elision of `restart` would be a priority-1 under-execute:
#     it would drop the very restart that applies a config change, exactly when the
#     daemon was already up. `inv-kfail`/Apply: when unsure, ACT.
# Hence there is deliberately NO `oracle_effect service restart …`. With no effect
# cell, every `service … restart` resolves ⊤ ⇒ runs — the correct, safe behaviour.
#
# The kind DOES declare an `#active` kind-default probe (read-only `service status`),
# for two honest reasons: it satisfies the "a declared kind needs a probe" contract,
# and it is the right probe for a `start` verb (#active is start's selector) SHOULD a
# future book use `service <svc> start`. It is NOT wired to `restart`. CAVEAT recorded
# in um-svc-1: `service status` rc is not cleanly three-outcome (running / stopped /
# unknown-or-no-such-service collapse), and #enabled (boot-persistence) has no
# `service`-native read at all — that is `systemctl is-enabled`/runlevel territory,
# un-modelable under this provider.
oracle_kind=service
oracle_probe_service() { service "$1" status >/dev/null 2>&1; }

# command-keyed check(): `service <name> <verb>` — note the operand-FIRST argv order
# (unlike systemctl's verb-first). Annotate the first operand `service`; bind the verb
# from the second. `restart` reaches no effect cell (um-svc-1) so it runs regardless;
# the resolution exists only to name the entity for diagnostics and to give a future
# start/stop arm a home.
service__check() {
   svc : service = "$1"
   verb=$2
   case $verb in
      restart) : ;;
   esac
}
