#!/bin/sh
# Oracle EXTRACTED from books/plik-postinstall.book.sh (root-gg/plik @4fbe4fe). [STRAWMAN]
# Method: lifted from the book's OWN guards via grep — the mutative body was never read.
# Kinds: user:plik · group:plik · service:plikd · file:/etc/plik/plikd.cfg
#
# (A) LIFTED — the human already guarded these (faithful):
plik_group() { getent group  plik >/dev/null 2>&1; }   # book L5:  if ! getent group plik
plik_user()  { getent passwd plik >/dev/null 2>&1; }   # book L9:  if ! getent passwd plik
plik_cfg_unconfigured() {                              # book L23 — NB INVERSE polarity:
   grep -q 'WebappDirectory.*= "../webapp/dist"' /etc/plik/plikd.cfg 2>/dev/null
}                                                       # TRUE = default path still there = NOT yet configured.
#
# (B) SUPPLIED — book enables the unit (L34) but never probes its state; Dorc would:
plik_svc() { systemctl is-enabled --quiet plikd 2>/dev/null; }   # service:plikd
#
# Kind tags (getent-pattern, all blessed + cross-script-correlatable):
#   user:plik=getent passwd · group:plik=getent group · service:plikd=systemctl is-enabled.
