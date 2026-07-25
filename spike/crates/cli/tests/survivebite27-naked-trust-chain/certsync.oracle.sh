#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# LYING wall oracle (27V §4 flagship). `certsync push` rewrites a cert bundle AND — per its real
# post-push behaviour — reloads nginx. But its disturbs claims at most its OWN CertBundle cell,
# omitting the Service cell systemctl backs. The omission survives the own-coordinate canary (the
# CertBundle cell IS certsync's own effect ⇒ own ∈ footprint), so --trust-footprints spares the
# downstream `systemctl start` elision that truly collides. The frame problem, bought.
certsync__predict() {
   verb=$1; shift
   bundle : sm.dorc.CertBundle = "$1"
   case $verb in
      push) certsync status -- "$bundle" : sm.dorc.CertBundle:"$bundle"@synced ;;
   esac
}

certsync__disturbs() {                             # THE LIE: at-most its OWN CertBundle cell — the
   verb=$1; shift                                  # real push ALSO reloads nginx (Service:nginx#active),
   bundle : sm.dorc.CertBundle = "$1"              # UNCLAIMED and invisible to every coherence check.
   case $verb in
      push) printf '%s\n' "$bundle" : disturbs sm.dorc.CertBundle ;;
   esac
}

certsync__is_converged() {
   verb=$1; shift
   case $verb in
      push) certsync status -- "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
