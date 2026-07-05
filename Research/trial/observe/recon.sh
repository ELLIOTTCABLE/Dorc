#!/bin/sh
# dorc-r25 on-box state recon (P2) — throwaway round-25 field-trial tooling.
#
# Runs ON the trial box (piped in over ssh: `ssh host sh -s < recon.sh`), as root,
# and dumps a COMPREHENSIVE, deterministic, dorc-INDEPENDENT snapshot of machine
# state to stdout — one sorted `source <TAB> entity <TAB> value` record per line.
# `#`-prefixed lines are provenance meta (the diff engine ignores them).
#
# WIDER-THAN-DORC is the whole point (plans/252 §1 P2, load-bearing): dorc models
# only what its oracles probe — dpkg presence, systemctl *enablement*, a `[ -f ]`
# file guard, ufw rules. This snapshot deliberately also captures the dimensions
# dorc has NO concept of, where converged≠no-op hides:
#   · service HEALTH via is-active/sub-state — NOT mere presence. `systemctl
#     enable --now` returns rc0 on a crash-loop (ops-val found this live); an rc-
#     trusting observer misses a dead service. active=failed / sub=auto-restart
#     is the tell.
#   · listening ports — a unit can be enabled+active yet not actually bound.
#   · docker container RUN-state + image ids — the book's `docker run` is non-
#     idempotent (errors on 2nd apply); state=exited vs running catches it.
#   · broad content-hashes — the book's cert guard checks presence not expiry;
#     a sha over /etc catches a present-but-stale file dorc's `[ -f ]` waves past.
#   · sysctl / kernel modules / users / groups / cron — no oracle touches these.
#   · postgres in-db roles/dbs — behind the book's own `su - postgres` opaque
#     wrapper; structurally invisible to dorc.
#
# Every source is guarded: a missing tool degrades to a `#`-warning + zero records,
# never a crash (comprehensiveness over fail-fast — this is read-only observation).
# NOTHING here mutates the box. Deterministic: no mtimes/pids/uptimes in values
# (they are "was it touched", not "did state change" — pure noise for the
# differential; see plans/252 §7 noise-governance). Final output is LC_ALL=C sorted.
#
# Config via positional args (observe.sh passes them):  $1=scan-roots  $2=prune-paths
# Defaults below. Env DORC_RECON_NO_PG=1 skips the postgres probe.

set -u

ROOTS="${1:-/etc /usr/local /srv /opt /root /var/lib}"
# subtrees whose CONTENTS churn or are better observed structurally — recorded as a
# node (exists+mode+owner) but never descended-into for hashing.
PRUNES="${2:-/var/lib/docker /var/lib/containerd /var/lib/postgresql /var/lib/prometheus /var/lib/apt/lists /var/lib/systemd /var/lib/private /var/cache /var/tmp}"

have(){ command -v "$1" >/dev/null 2>&1; }
warn(){ printf '# WARN: %s\n' "$*"; }
rec(){ # rec <source> <entity> <value...>  — tabs in fields collapsed to spaces
   s=$1; e=$2; shift 2
   printf '%s\t%s\t%s\n' "$s" "$e" "$*" | tr -d '\r'
}

emit_all(){
   # --- provenance (meta; diff ignores `#`) --------------------------------
   printf '# dorc-r25 recon snapshot\n'
   printf '# host: %s\n' "$(hostname 2>/dev/null)"
   printf '# when: %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null)"
   printf '# kernel: %s\n' "$(uname -rms 2>/dev/null)"
   printf '# roots: %s\n' "$ROOTS"
   printf '# prunes: %s\n' "$PRUNES"

   src_fs
   src_pkg
   src_svc
   src_port
   src_docker
   src_firewall
   src_kmod
   src_ident
   src_sysctl
   src_cron
   [ "${DORC_RECON_NO_PG:-0}" = 1 ] || src_pg
}

# --- filesystem: content-hash + metadata over ROOTS, churny subtrees pruned --
src_fs(){
   have find || { warn "no find — skipping fs"; return; }
   have sha256sum || warn "no sha256sum — fs content-hashes will be MISSING (metadata only)"
   # a find -path prune expression from PRUNES
   pex=""; for p in $PRUNES; do pex="$pex -path $p -o"; done
   pex="${pex% -o}"
   hashes=$(mktemp 2>/dev/null) || hashes=/tmp/.dorc_recon_hashes.$$
   : > "$hashes"
   if have sha256sum; then
      # shellcheck disable=SC2086
      find $ROOTS \( $pex \) -prune -o -type f -print0 2>/dev/null \
         | xargs -0 sha256sum 2>/dev/null > "$hashes"
   fi
   # shellcheck disable=SC2086
   find $ROOTS \( $pex \) -prune -o -printf '%y\t%m\t%u\t%g\t%s\t%l\t%p\n' 2>/dev/null \
   | awk -F'\t' -v H="$hashes" '
        BEGIN{ while((getline line < H)>0){ h=substr(line,1,64); p=substr(line,index(line,"  ")+2); hash[p]=h } }
        {
           type=$1; mode=$2; u=$3; g=$4; sz=$5; lnk=$6; path=$7;
           for(i=8;i<=NF;i++) path=path "\t" $i;      # rejoin the (rare) tabbed path
           val="type=" type " mode=" mode " own=" u ":" g;
           if(type=="f"){ val=val " size=" sz " sha=" (path in hash ? hash[path] : "UNREADABLE") }
           else if(type=="l"){ val=val " -> " lnk }
           gsub(/\t/," ",path);
           print "fs\t" path "\t" val;
        }'
   rm -f "$hashes" 2>/dev/null
   # the pruned subtrees, as bare nodes (so their creation still shows)
   for p in $PRUNES; do
      [ -e "$p" ] || continue
      m=$(stat -c 'type=%F mode=%a own=%U:%G' "$p" 2>/dev/null) || continue
      rec fs "$p" "$m (contents-pruned)"
   done
}

# --- dpkg package db: version + install-status per package -------------------
src_pkg(){
   have dpkg-query || { warn "no dpkg-query — skipping pkg"; return; }
   dpkg-query -W -f '${Package}\t${Version}\t${db:Status-Abbrev}\n' 2>/dev/null \
   | awk -F'\t' '{ print "pkg\t" $1 "\tver=" $2 " st=" $3 }'
}

# --- systemd: HEALTH (load/active/sub) + enablement per service unit ---------
# The load-bearing width: active/sub is what catches a crash-loop that rc=0 hides.
src_svc(){
   have systemctl || { warn "no systemctl — skipping svc"; return; }
   enf=$(mktemp 2>/dev/null) || enf=/tmp/.dorc_recon_enabled.$$
   systemctl list-unit-files --type=service --no-legend --no-pager 2>/dev/null \
      | awk '{ print $1 "\t" $2 }' > "$enf"
   systemctl list-units --type=service --all --plain --no-legend --no-pager 2>/dev/null \
   | awk -v E="$enf" '
        BEGIN{ while((getline line < E)>0){ split(line,a,"\t"); en[a[1]]=a[2] } }
        { unit=$1; load=$2; active=$3; sub=$4;
          e=(unit in en ? en[unit] : "?");
          print "svc\t" unit "\tload=" load " active=" active " sub=" sub " enabled=" e }'
   rm -f "$enf" 2>/dev/null
   # top-level health rollup — one crash-loop flips this to `degraded` (cheap wide net)
   rec svc "@system-state" "value=$(systemctl is-system-running 2>/dev/null)"
   rec svc "@failed-count" "value=$(systemctl list-units --state=failed --plain --no-legend --no-pager 2>/dev/null | grep -c .)"
}

# --- listening sockets: proto/addr:port -> process name (pid dropped = noise) -
src_port(){
   have ss || { warn "no ss — skipping port"; return; }
   ss -H -tulpn 2>/dev/null \
   | awk '{
        netid=$1; local=$5; proc="-";
        for(i=1;i<=NF;i++){ if($i ~ /^users:/){ m=$i; if(match(m,/"[^"]+"/)) proc=substr(m,RSTART+1,RLENGTH-2) } }
        print "port\t" netid "/" local "\tproc=" proc }'
}

# --- docker: container run-state + image ids (guarded on a live daemon) ------
src_docker(){
   have docker || { warn "no docker — skipping docker"; return; }
   docker info >/dev/null 2>&1 || { warn "docker present but daemon unreachable — skipping docker"; return; }
   docker ps -a --format '{{.Names}}\t{{.Image}}\t{{.State}}' 2>/dev/null \
   | awk -F'\t' '{ print "docker\tcontainer/" $1 "\timage=" $2 " state=" $3 }'
   docker images --no-trunc --format '{{.Repository}}:{{.Tag}}\t{{.ID}}' 2>/dev/null \
   | awk -F'\t' '{ print "docker\timage/" $1 "\tid=" $2 }'
}

# --- firewall: ufw rules (parsed) + a stateless nft/iptables ruleset hash ----
# The hash is wider than ufw's own view — catches ANY packet-filter change.
src_firewall(){
   if have ufw; then
      ufw status 2>/dev/null | awk '
         /^[Ss]tatus:/ { print "fw\tufw/status\tvalue=" $2; next }
         /(ALLOW|DENY|REJECT|LIMIT)/ {
            line=$0; gsub(/[ \t]+/," ",line); gsub(/^ | $/,"",line);
            print "fw\tufw-rule/" line "\tpresent" }'
   else warn "no ufw — skipping ufw rules"; fi
   # counter-free ruleset hash: nft -s (stateless) preferred; else iptables-save (no -c)
   if have nft; then
      h=$(nft -s list ruleset 2>/dev/null | sha256sum 2>/dev/null | cut -c1-64)
      [ -n "$h" ] && rec fw "nft/ruleset-sha" "value=$h"
   elif have iptables-save; then
      h=$(iptables-save 2>/dev/null | grep -v '^#' | sha256sum 2>/dev/null | cut -c1-64)
      [ -n "$h" ] && rec fw "iptables/ruleset-sha" "value=$h"
   fi
}

# --- kernel modules (names only; size/refcount are runtime-noisy) -----------
src_kmod(){
   have lsmod || { warn "no lsmod — skipping kmod"; return; }
   lsmod 2>/dev/null | awk 'NR>1{ print "kmod\t" $1 "\tloaded" }'
}

# --- system identity: users, groups, kernel release -------------------------
src_ident(){
   if have getent; then
      getent passwd 2>/dev/null | awk -F: '{ print "user\t" $1 "\tuid=" $3 " gid=" $4 " home=" $6 " shell=" $7 }'
      getent group 2>/dev/null  | awk -F: '{ print "group\t" $1 "\tgid=" $3 " members=" $4 }'
   else warn "no getent — skipping users/groups"; fi
   rec kernel "release" "value=$(uname -r 2>/dev/null)"
}

# --- a curated, non-noisy sysctl subset (security/networking posture) --------
src_sysctl(){
   have sysctl || { warn "no sysctl — skipping sysctl"; return; }
   for k in \
      net.ipv4.ip_forward \
      net.ipv6.conf.all.forwarding \
      net.ipv4.conf.all.rp_filter \
      net.ipv4.tcp_syncookies \
      kernel.unprivileged_userns_clone \
      kernel.kptr_restrict \
      kernel.randomize_va_space \
      fs.protected_hardlinks \
      fs.protected_symlinks
   do
      v=$(sysctl -n "$k" 2>/dev/null) && rec sysctl "$k" "value=$v"
   done
}

# --- root crontab + a note of /etc/cron* (dirs come via fs) ------------------
src_cron(){
   if have crontab; then
      c=$(crontab -l 2>/dev/null | grep -v '^[[:space:]]*#' | grep -c .)
      rec cron "root/entry-count" "value=$c"
      h=$(crontab -l 2>/dev/null | grep -v '^[[:space:]]*#' | sha256sum 2>/dev/null | cut -c1-64)
      [ -n "$h" ] && rec cron "root/sha" "value=$h"
   fi
}

# --- postgres in-db state: roles + databases (behind the book's su-wrapper) --
# The strongest wider-than-dorc probe: this state lives INSIDE postgres, opaque to
# any sh-level analysis. Read-only SELECTs, guarded on a reachable local server.
src_pg(){
   have psql || return                              # silent: not every box runs pg
   id postgres >/dev/null 2>&1 || return
   su - postgres -c 'psql -tAqc "SELECT 1"' >/dev/null 2>&1 || {
      warn "postgres user present but psql not reachable — skipping pg"; return; }
   su - postgres -c 'psql -tAqc "SELECT rolname FROM pg_roles ORDER BY 1"' 2>/dev/null \
      | while IFS= read -r r; do [ -n "$r" ] && rec pgrole "$r" "present"; done
   su - postgres -c 'psql -tAqc "SELECT datname FROM pg_database ORDER BY 1"' 2>/dev/null \
      | while IFS= read -r d; do [ -n "$d" ] && rec pgdb "$d" "present"; done
}

# `#`-meta sorts to the top (C locale); records follow, deterministically ordered.
emit_all | LC_ALL=C sort
