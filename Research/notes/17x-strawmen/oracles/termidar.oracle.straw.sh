#!/bin/sh
# Oracle EXTRACTED from books/termidar-setup.book.sh (N-Erickson/termidar @040f0d6). [STRAWMAN]
# Kinds: service:firewalld · tool:go · user:termidar · service:termidar-ssh · port:2222/tcp · sshd-config
#
# (A) LIFTED:
firewalld_active() { systemctl is-active --quiet firewalld; }   # book L33
go_present()       { command -v go >/dev/null 2>&1; }           # book L49: if ! command -v go
termidar_user()    { id -u termidar >/dev/null 2>&1; }          # book L69: if ! id -u $TERMIDAR_USER
termidar_active()  { systemctl is-active --quiet termidar-ssh; }  # book L237
#
# (B) SUPPLIED — unguarded port + sshd edit:
fw_2222()   { firewall-cmd --query-port=2222/tcp >/dev/null 2>&1; }   # book L34 --add-port (unguarded) —
                                                                      #   firewalld HAS a clean query verb (getent-pattern!)
sshd_2222() { grep -q '^Port 2222' /etc/ssh/sshd_config 2>/dev/null; }  # book L29: echo "Port 2222" | tee -a
                                                                      #   (unguarded AND non-idempotent append)
#
# Provider note: termidar uses firewalld, so `firewall-cmd --query-port=` is a CLEAN port-kind probe — the
# port-kind IS getent-pattern-probe-able here. Contrast onservice's ufw (no clean query -> fragile grep).
# Port-probe-ability is PROVIDER-dependent: a key honest finding (the same kind, two providers, two answers).
