#!/bin/sh
# Oracle EXTRACTED from books/onservice-harden.book.sh (onServiceTeam @5d1f2c6). [STRAWMAN]
# A HARDENING book — much state-change, comparatively LITTLE per-kind guarding (the honest worst case).
# Kinds: file:/swapfile · user:deploy · membership:deploy@sudo · service:{unattended-upgrades,fail2ban,ssh}
#        · port:{22,80,443}/tcp · sshd-config-valid
#
# (A) LIFTED — the human's only real guards (two), plus one precondition:
swapfile()    { [ -f /swapfile ]; }                  # book L94: if [ ! -f /swapfile ]
deploy_user() { id deploy >/dev/null 2>&1; }         # book L105: if ! id deploy
sshd_valid()  { sshd -t 2>/dev/null; }               # book L75 — PRECONDITION (validate), not idempotency
#
# (B) SUPPLIED — these fire UNCONDITIONALLY in the book; getent-pattern/port probes COULD elide:
deploy_in_sudo() { id -nG deploy 2>/dev/null | grep -qw sudo; }              # book L107 usermod -aG sudo (unguarded)
f2b_svc()        { systemctl is-enabled --quiet fail2ban 2>/dev/null; }      # book L55 enable --now (unguarded)
ufw_allow() {                                                                # book L37-39 ufw allow 22|80|443 (unguarded)
   ufw status 2>/dev/null | grep -q "$1[/ ].*ALLOW"   # ufw_allow 22 / 80 / 443
}
#
# Coverage WARNING: a hardening book is the worst case for guard-extraction — most lines are unconditional
# `ufw allow` / `systemctl enable --now`. Port-kind IS probe-able but the ufw parse is FRAGILE — this very
# `grep` carries the 15x ufw.straw.sh `.`-as-regex hazard (a rule "10.0.0.1" matches "10X0X0X1"). Port
# elision here wants a real ORACLE, not the book's (absent) guard. ~2 of ~9 state-changes are author-guarded.
