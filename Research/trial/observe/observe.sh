#!/usr/bin/env sh
# dorc-r25 comprehensive-state observer (P2) — throwaway round-25 field-trial tooling.
#
# The anti-self-greening eyeball: watches a trial box's ground-truth state INDEPENDENTLY
# of dorc's own model, so the differential (P4) can catch state dorc doesn't model. An
# observer that shared dorc's model could only ever confirm dorc — theatre. See recon.sh
# for the width; this driver captures, diffs, and governs the noise.
#
# Contract C-delta (plans/252 §2): a machine-delta = a set of typed changes
#   {source∈(fs-diff|state-probe|syscall), entity, kind∈(added|removed|changed),
#    before, after}, comparable across two runs. Emitted as JSONL (schema C-delta/1).
#
# Noise-governance (plans/252 §7, load-bearing — `∅` is fiction on a real box):
#   (a) the noise-envelope is derived dorc-INDEPENDENTLY via A/A runs — snapshot the
#       same box twice with NO change between; that diff IS the noise-floor to subtract
#       (`aa` / `envelope`). Values here carry no mtime/pid/uptime by construction, so
#       the floor is small and honest, not a giant subtract-list.
#   (b) a `world-drift` category tags between-run world changes (mirror moved a pkg,
#       kernel auto-update) — distinct from a real machine mutation.
#   (c) planted canaries: `canary-check` asserts the envelope does NOT mask a known
#       mutation — the subtraction is valid ONLY while canaries still show through it.
#
# Subcommands:
#   snapshot <host> [--out DIR] [--roots "…"] [--prunes "…"] [--local]
#   diff  <A.txt> <B.txt> [--out DIR] [--envelope FILE] [--label-a X] [--label-b Y]
#   envelope <A.txt> <B.txt> [--out FILE]        build a noise-envelope from an A/A pair
#   aa <host> [--out DIR]                        snapshot ×2 (no change) -> envelope
#   canary-check <envelope-FILE> <key>… | --file <keys-FILE>
#   selftest                                     hermetic; no network
#   help
#
# ssh reuses the apply-runner's usekeychain-free config + trial key (single source of
# truth). NOTHING here mutates the box.  Env: SSH_CONFIG SSH_KEY REMOTE_SH RECON OBS_ROOT.

set -eu

SELF="$0"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$SELF")" && pwd)"

SSH_CONFIG="${SSH_CONFIG:-$SCRIPT_DIR/../apply/ssh_config}"   # reuse P3's usekeychain-free config
SSH_KEY="${SSH_KEY:-$HOME/.ssh/dorc-r25}"
REMOTE_SH="${REMOTE_SH:-sh}"                                   # =dash on the Debian target
RECON="${RECON:-$SCRIPT_DIR/recon.sh}"
OBS_ROOT="${OBS_ROOT:-./dorc-r25-obs}"

log(){ printf '[observe] %s\n' "$*" >&2; }
die(){ log "ERROR: $*"; exit 2; }

[ -r "$RECON" ] || die "recon.sh not found beside observe.sh: $RECON"

# ── snapshot: ssh in, run recon, save the sorted state records ───────────────
cmd_snapshot(){
   DIR=""; ROOTS=""; PRUNES=""; LOCAL=0; HOST=""
   while [ $# -gt 0 ]; do
      case "$1" in
         --out)    DIR="${2:?--out needs a DIR}"; shift 2 ;;
         --roots)  ROOTS="${2:-}"; shift 2 ;;
         --prunes) PRUNES="${2:-}"; shift 2 ;;
         --local)  LOCAL=1; shift ;;
         --)       shift ;;
         -*)       die "unknown flag: $1" ;;
         *)        [ -z "$HOST" ] || die "unexpected extra arg: $1"; HOST="$1"; shift ;;
      esac
   done
   [ -n "$HOST" ] || [ "$LOCAL" -eq 1 ] || die "usage: $SELF snapshot <host> [--out DIR] [--local]"
   [ -n "$HOST" ] || HOST="local"

   if [ -z "$DIR" ]; then
      mkdir -p "$OBS_ROOT"
      stamp="$(date -u +%Y%m%dT%H%M%SZ)"
      slug="$(printf '%s' "$HOST" | tr -c 'A-Za-z0-9._-' '_')"
      DIR="$(mktemp -d "$OBS_ROOT/${stamp}-${slug}-XXXXXX")"
   else
      mkdir -p "$DIR"
   fi
   DIR="$(CDPATH= cd -- "$DIR" && pwd)"
   SNAP="$DIR/snapshot.txt"; ERR="$DIR/recon.stderr"

   if [ "$LOCAL" -eq 1 ]; then
      log "snapshot LOCAL  roots='${ROOTS:-<default>}'  dir=$DIR"
      sh "$RECON" "$ROOTS" "$PRUNES" >"$SNAP" 2>"$ERR" || die "local recon failed (see $ERR)"
   else
      [ -r "$SSH_CONFIG" ] || die "ssh config not found: $SSH_CONFIG"
      [ -r "$SSH_KEY" ]    || die "trial ssh key not found: $SSH_KEY"
      log "snapshot SSH   host=$HOST  cfg=$SSH_CONFIG  dir=$DIR"
      # recon.sh IS the remote script (stdin to `sh -s`); roots/prunes ride as $1/$2.
      ssh -F "$SSH_CONFIG" -i "$SSH_KEY" -T "$HOST" "$REMOTE_SH" -s -- "$ROOTS" "$PRUNES" \
         <"$RECON" >"$SNAP" 2>"$ERR" \
         || die "remote recon failed on $HOST (see $ERR) — HALT (unknown state is not safe to trust)"
   fi

   recs="$(grep -cv '^#' "$SNAP" 2>/dev/null || echo 0)"
   [ "$recs" -gt 0 ] || die "snapshot has 0 records — recon produced nothing (see $ERR)"
   when="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
   log "snapshot done  records=$recs  -> $SNAP"
   printf '{"schema":"C-snap/1","host":"%s","dir":"%s","snapshot":"%s","records":%s,"when":"%s"}\n' \
      "$HOST" "$DIR" "$SNAP" "$recs" "$when"
}

# ── the pure diff core: two snapshots -> raw typed deltas (sorted TSV) ────────
# Key = source<TAB>entity; `#`-meta ignored. Emits: kind \t source \t entity \t before \t after
_raw_diff(){
   awk -F'\t' '
      /^#/ || NF<2 { next }
      FNR==NR { A[$1 SUBSEP $2]=$3; KA[$1 SUBSEP $2]=$1"\t"$2; next }
                { B[$1 SUBSEP $2]=$3; KB[$1 SUBSEP $2]=$1"\t"$2 }
      END{
         for(k in A){ if(k in B){ if(A[k]!=B[k]) print "changed\t" KA[k] "\t" A[k] "\t" B[k] }
                      else print "removed\t" KA[k] "\t" A[k] "\t" }
         for(k in B){ if(!(k in A)) print "added\t" KB[k] "\t\t" B[k] }
      }' "$1" "$2" | LC_ALL=C sort
}

# churny families collapsed to one rule each — so an A/A envelope stays legible even
# when a whole log/cache subtree moved. (Prepended to every built envelope.)
_glob_prelude(){
   cat <<'EOF'
# --- dorc-r25 built-in noise prelude (churny families) ---
fs	/var/log/*
fs	/var/cache/*
fs	/var/lib/apt/lists*
fs	/var/lib/systemd/*
fs	/var/lib/private/*
fs	/etc/ld.so.cache
fs	/etc/.pwd.lock
fs	/root/.*history
svc	@failed-count
EOF
}

# ── envelope: an A/A diff's changed keys (+ the glob prelude) = the noise-floor ─
cmd_envelope(){
   OUT=""; A=""; B=""
   while [ $# -gt 0 ]; do
      case "$1" in
         --out) OUT="${2:?}"; shift 2 ;;
         --)    shift ;;
         -*)    die "bad flag $1" ;;
         *)     if [ -z "$A" ]; then A="$1"; elif [ -z "$B" ]; then B="$1"; else die "extra arg: $1"; fi; shift ;;
      esac
   done
   [ -n "$A" ] && [ -n "$B" ] || die "usage: $SELF envelope <A.txt> <B.txt> [--out FILE]"
   tmp="$(mktemp)"
   { _glob_prelude
     printf '# --- measured A/A residual (%s vs %s) ---\n' "$A" "$B"
     _raw_diff "$A" "$B" | awk -F'\t' '{ print $2 "\t" $3 }' | LC_ALL=C sort -u
   } > "$tmp"
   n="$(grep -cv '^#' "$tmp" || echo 0)"
   if [ -n "$OUT" ]; then mv "$tmp" "$OUT"; log "envelope: $n rules -> $OUT"; else cat "$tmp"; rm -f "$tmp"; log "envelope: $n rules"; fi
}

# ── diff: raw deltas, minus the envelope, with world-drift tagged ────────────
cmd_diff(){
   OUT=""; ENVF=""; LA="A"; LB="B"; A=""; B=""
   while [ $# -gt 0 ]; do
      case "$1" in
         --out)      OUT="${2:?}"; shift 2 ;;
         --envelope) ENVF="${2:?}"; shift 2 ;;
         --label-a)  LA="${2:?}"; shift 2 ;;
         --label-b)  LB="${2:?}"; shift 2 ;;
         --)         shift ;;
         -*)         die "unknown flag: $1" ;;
         *)          if [ -z "$A" ]; then A="$1"; elif [ -z "$B" ]; then B="$1"; else die "extra arg: $1"; fi; shift ;;
      esac
   done
   [ -n "$A" ] && [ -n "$B" ] || die "usage: $SELF diff <A.txt> <B.txt> [--envelope FILE] [--out DIR]"
   [ -r "$A" ] || die "no such snapshot: $A"
   [ -r "$B" ] || die "no such snapshot: $B"
   [ -z "$ENVF" ] || [ -r "$ENVF" ] || die "no such envelope: $ENVF"

   if [ -z "$OUT" ]; then mkdir -p "$OBS_ROOT"; OUT="$(mktemp -d "$OBS_ROOT/delta-XXXXXX")"; else mkdir -p "$OUT"; fi
   OUT="$(CDPATH= cd -- "$OUT" && pwd)"
   JSONL="$OUT/delta.jsonl"; TXT="$OUT/delta.txt"
   : > "$JSONL"                                    # awk appends; guarantee a clean, existing file

   _raw_diff "$A" "$B" | awk -F'\t' \
      -v JSONL="$JSONL" -v TXT="$TXT" -v ENVF="${ENVF:-}" -v LA="$LA" -v LB="$LB" -v A="$A" -v B="$B" '
      function jstr(s){ gsub(/\\/,"\\\\",s); gsub(/"/,"\\\"",s); gsub(/\t/," ",s); gsub(/\n/," ",s); return "\"" s "\"" }
      function glob2re(p,   r,i,c){ r="^";
         for(i=1;i<=length(p);i++){ c=substr(p,i,1);
            if(c=="*") r=r".*"; else if(c=="?") r=r".";
            else if(index(".^$+(){}[]|\\/",c)) r=r"\\" c; else r=r c }
         return r "$" }
      function masked(key,   i){ for(i=1;i<=ne;i++) if(key ~ ere[i]) return 1; return 0 }
      # world-drift signatures: world moved between snapshots, not the box being built.
      function is_drift(src,ent,before,after,   b,a){
         if(src=="kernel" && ent=="release") return 1;                 # auto kernel update
         if(src=="pkg" && before ~ /st=ii/ && after ~ /st=ii/){        # installed->installed,
            b=before; a=after; sub(/ver=[^ ]+/,"",b); sub(/ver=[^ ]+/,"",a);
            if(b==a) return 1 }                                        # only the version moved (mirror)
         if(src=="fs" && ent ~ /\/var\/lib\/apt\//) return 1;          # mirror cache
         return 0 }
      BEGIN{
         ne=0; if(ENVF!=""){ while((getline line < ENVF)>0){ if(line ~ /^#/ || line=="") continue; ere[++ne]=glob2re(line) } }
         printf "== dorc-r25 machine-delta (%s -> %s) ==\n", LA, LB > TXT
         printf "A: %s\nB: %s\nenvelope: %s\n\n", A, B, (ENVF=="" ? "(none)" : ENVF) >> TXT
      }
      {
         kind=$1; src=$2; ent=$3; before=$4; after=$5;
         key=src "\t" ent;
         if(masked(key)){ nnoise++; next }                             # (a) A/A noise subtracted
         csrc=(src=="fs" ? "fs-diff" : "state-probe");                 # C-delta contract source
         cat=(is_drift(src,ent,before,after) ? "world-drift" : "change");  # (b)
         # C-delta/1 JSONL
         printf "{\"schema\":\"C-delta/1\",\"source\":%s,\"probe\":%s,\"entity\":%s,\"kind\":%s,\"before\":%s,\"after\":%s,\"category\":%s}\n",
            jstr(csrc), jstr(src), jstr(ent), jstr(kind), jstr(before), jstr(after), jstr(cat) >> JSONL;
         if(cat=="world-drift"){ drift[++nd]=sprintf("  [%s/%s] %s   %s -> %s", csrc, src, ent, before, after); ndrift++; next }
         n[kind]++; ntot++;
         line=sprintf("  [%s/%s] %s", csrc, src, ent);
         if(kind=="changed") line=line sprintf("\n      %s\n   -> %s", before, after);
         else if(kind=="added") line=line "   " after;
         else line=line "   " before;
         bucket[kind]=bucket[kind] line "\n";
      }
      END{
         printf "totals: +%d  -%d  ~%d    world-drift: %d    noise-subtracted: %d\n\n",
            n["added"]+0, n["removed"]+0, n["changed"]+0, ndrift+0, nnoise+0 >> TXT;
         for(k in bucket){ printf "--- %s ---\n%s\n", k, bucket[k] >> TXT }
         if(ndrift>0){ printf "--- world-drift (informational; not a machine mutation) ---\n" >> TXT;
                       for(i=1;i<=nd;i++) print drift[i] >> TXT; printf "\n" >> TXT }
         # stdout: a one-line JSON summary (the machine-facing verdict shape)
         printf "{\"schema\":\"C-delta-summary/1\",\"added\":%d,\"removed\":%d,\"changed\":%d,\"world_drift\":%d,\"noise_subtracted\":%d,\"jsonl\":%s,\"txt\":%s}\n",
            n["added"]+0, n["removed"]+0, n["changed"]+0, ndrift+0, nnoise+0, jstr(JSONL), jstr(TXT);
      }'
   log "delta -> $TXT  (jsonl: $JSONL)"
}

# ── canary-check: the envelope MUST NOT mask a known planted mutation ────────
# (plans/252 §7c) the noise-subtraction is valid ONLY while canaries show through it.
cmd_canary_check(){
   [ $# -ge 1 ] || die "usage: $SELF canary-check <envelope-FILE> <key>… | <envelope> --file <keys-FILE>"
   ENVF="$1"; shift
   [ -r "$ENVF" ] || die "no such envelope: $ENVF"
   keys=""
   if [ "${1:-}" = "--file" ]; then
      [ -r "${2:?--file needs a path}" ] || die "no such keys file: $2"
      keys="$(grep -v '^#' "$2" | grep .)"
   else
      [ $# -ge 1 ] || die "no canary keys given"
      keys="$(printf '%s\n' "$@")"
   fi
   masked="$(printf '%s\n' "$keys" | awk -F'\t' -v ENVF="$ENVF" '
      function glob2re(p,   r,i,c){ r="^"; for(i=1;i<=length(p);i++){ c=substr(p,i,1);
         if(c=="*") r=r".*"; else if(c=="?") r=r".";
         else if(index(".^$+(){}[]|\\/",c)) r=r"\\" c; else r=r c } return r "$" }
      BEGIN{ ne=0; while((getline line < ENVF)>0){ if(line ~ /^#/ || line=="") continue; ere[++ne]=glob2re(line) } }
      { key=$0; for(i=1;i<=ne;i++) if(key ~ ere[i]){ print key; next } }')"
   if [ -n "$masked" ]; then
      log "CANARY MASKED — envelope $ENVF would HIDE these known mutations (envelope INVALID, §7c):"
      printf '%s\n' "$masked" | sed 's/^/  /' >&2
      return 3                                      # return, not exit: composes inside `if` (selftest, P4)
   fi
   n="$(printf '%s\n' "$keys" | grep -c .)"
   log "canary-check PASS — all $n canaries show through the envelope (subtraction valid)"
}

# ── aa: the dorc-independent noise derivation (snapshot ×2, no change) ────────
cmd_aa(){
   DIR=""; HOST=""
   while [ $# -gt 0 ]; do
      case "$1" in
         --out) DIR="${2:?}"; shift 2 ;;
         --)    shift ;;
         -*)    die "bad flag $1" ;;
         *)     [ -z "$HOST" ] || die "extra arg: $1"; HOST="$1"; shift ;;
      esac
   done
   [ -n "$HOST" ] || die "usage: $SELF aa <host> [--out DIR]"
   [ -n "$DIR" ] || { mkdir -p "$OBS_ROOT"; DIR="$(mktemp -d "$OBS_ROOT/aa-XXXXXX")"; }
   DIR="$(CDPATH= cd -- "$DIR" && pwd)"
   log "A/A noise derivation on $HOST (two snapshots, NO change between) -> $DIR"
   cmd_snapshot "$HOST" --out "$DIR/a1" >/dev/null
   cmd_snapshot "$HOST" --out "$DIR/a2" >/dev/null
   cmd_envelope "$DIR/a1/snapshot.txt" "$DIR/a2/snapshot.txt" --out "$DIR/envelope.txt"
   nres="$(_raw_diff "$DIR/a1/snapshot.txt" "$DIR/a2/snapshot.txt" | grep -c . || echo 0)"
   log "A/A measured residual: $nres changed keys (the honest noise-floor); envelope -> $DIR/envelope.txt"
   printf '{"schema":"C-aa/1","host":"%s","dir":"%s","a1":"%s","a2":"%s","envelope":"%s","residual_keys":%s}\n' \
      "$HOST" "$DIR" "$DIR/a1/snapshot.txt" "$DIR/a2/snapshot.txt" "$DIR/envelope.txt" "$nres"
}

# ── selftest: hermetic proof of the diff/envelope/canary/drift logic ─────────
cmd_selftest(){
   log "selftest: hermetic (no network) — exercising diff, A/A subtraction, canary, drift"
   T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT

   # A synthetic snapshot A: a fresh box.
   cat > "$T/a1.txt" <<'EOF'
# meta
fs	/etc/nginx/nginx.conf	type=f mode=644 own=root:root size=100 sha=aaa
fs	/var/log/syslog	type=f mode=640 own=root:adm size=10 sha=log1
pkg	nginx	ver=1.22.1-9 st=ii
pkg	libc6	ver=2.36-9 st=ii
svc	nginx.service	load=loaded active=active sub=running enabled=enabled
kernel	release	value=6.1.0-13-amd64
EOF
   # A/A twin (a2): ONLY the churny log moved — that is the whole noise-floor.
   cat > "$T/a2.txt" <<'EOF'
# meta (later)
fs	/etc/nginx/nginx.conf	type=f mode=644 own=root:root size=100 sha=aaa
fs	/var/log/syslog	type=f mode=640 own=root:adm size=880 sha=log2
pkg	nginx	ver=1.22.1-9 st=ii
pkg	libc6	ver=2.36-9 st=ii
svc	nginx.service	load=loaded active=active sub=running enabled=enabled
kernel	release	value=6.1.0-13-amd64
EOF
   # Post-apply-ish (b): a REAL mutation (a config edit = the canary), a crash-looping
   # new service (health width), PLUS the same log-churn noise AND a world-drift kernel bump.
   cat > "$T/b.txt" <<'EOF'
# meta (post)
fs	/etc/nginx/nginx.conf	type=f mode=644 own=root:root size=140 sha=CANARY
fs	/var/log/syslog	type=f mode=640 own=root:adm size=1200 sha=log3
pkg	nginx	ver=1.22.1-9 st=ii
pkg	libc6	ver=2.36-10 st=ii
svc	nginx.service	load=loaded active=active sub=running enabled=enabled
svc	grafana.service	load=loaded active=activating sub=auto-restart enabled=enabled
kernel	release	value=6.1.0-14-amd64
EOF

   fail=0

   # 1. envelope from the A/A pair should name the log key (and nothing structural).
   cmd_envelope "$T/a1.txt" "$T/a2.txt" --out "$T/env.txt" 2>/dev/null
   if grep -q '/var/log/syslog' "$T/env.txt" || grep -q '/var/log/\*' "$T/env.txt"; then
      log "  PASS: A/A envelope captures the log-churn noise"
   else log "  FAIL: A/A envelope missed the log churn"; fail=1; fi

   # 2. bare diff B-vs-A2 sees the canary, the crash-loop, AND the noise+drift.
   cmd_diff "$T/a2.txt" "$T/b.txt" --out "$T/d_bare" >/dev/null 2>&1
   grep -q '"sha=CANARY"\|CANARY' "$T/d_bare/delta.jsonl" && log "  PASS: canary (config edit) caught bare" || { log "  FAIL: canary missed bare"; fail=1; }
   grep -q 'auto-restart' "$T/d_bare/delta.jsonl" && log "  PASS: crash-loop (active=activating/auto-restart) caught — health width, not rc" || { log "  FAIL: crash-loop missed"; fail=1; }

   # 3. envelope-subtracted diff: canary+crash-loop SURVIVE, log-noise GONE, kernel=world-drift.
   cmd_diff "$T/a2.txt" "$T/b.txt" --envelope "$T/env.txt" --out "$T/d_net" >/dev/null 2>&1
   grep -q 'CANARY' "$T/d_net/delta.jsonl"    && log "  PASS: canary survives noise-subtraction" || { log "  FAIL: canary lost to subtraction"; fail=1; }
   if grep -q '/var/log/syslog' "$T/d_net/delta.jsonl"; then log "  FAIL: log-noise leaked past subtraction"; fail=1; else log "  PASS: log-noise subtracted"; fi
   grep -q '"category":"world-drift"' "$T/d_net/delta.jsonl" && log "  PASS: kernel bump tagged world-drift (not counted as mutation)" || { log "  FAIL: world-drift not tagged"; fail=1; }

   # 4. canary-check: an over-broad envelope that WOULD mask the config canary must be rejected.
   printf 'fs\t/etc/nginx/*\n' > "$T/bad_env.txt"
   if cmd_canary_check "$T/bad_env.txt" "$(printf 'fs\t/etc/nginx/nginx.conf')" 2>/dev/null; then
      log "  FAIL: canary-check accepted an envelope that masks the canary"; fail=1
   else log "  PASS: canary-check REJECTS an envelope that would hide the canary (§7c gate)"; fi

   # 5. canary-check: the good (log-only) envelope must NOT flag the config canary.
   if cmd_canary_check "$T/env.txt" "$(printf 'fs\t/etc/nginx/nginx.conf')" 2>/dev/null; then
      log "  PASS: canary-check accepts the honest log-only envelope"
   else log "  FAIL: canary-check wrongly rejected the honest envelope"; fail=1; fi

   [ "$fail" -eq 0 ] && { log "selftest: ALL PASS"; return 0; } || die "selftest: FAILURES above"
}

case "${1:-}" in
   snapshot)      shift; cmd_snapshot "$@" ;;
   diff)          shift; cmd_diff "$@" ;;
   envelope)      shift; cmd_envelope "$@" ;;
   canary-check)  shift; cmd_canary_check "$@" ;;
   aa)            shift; cmd_aa "$@" ;;
   selftest)      shift; cmd_selftest "$@" ;;
   ""|-h|help|--help)
      sed -n '2,45p' "$SELF" | sed 's/^# \{0,1\}//' >&2
      exit 1 ;;
   *) die "unknown subcommand: $1 (try: $SELF help)" ;;
esac
