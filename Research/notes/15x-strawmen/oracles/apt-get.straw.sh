# ═══════════════════════════════════════════════════════════════════════
# oracle: apt-get — Debian package management      [STRAWMAN, for discussion]
# ═══════════════════════════════════════════════════════════════════════
# An oracle WRAPS the upstream command and is NAMED after it (apt-get) so Dorc can
# bind a bare `apt-get install …` in a book to it and SILENTLY transmute it — the
# book author writes nothing special:
#
#   book writes:        apt-get install -y nginx
#   Dorc (probe phase): apt-get.check install -y nginx   # iff .check is defined,
#                       then hoists to global, parallelises, ships to host.
#   no .check defined:  the call + its control-flow stay, run at apply, every time.
#
# So a verb is a defensive dry-run of the SAME call: delegate to apt-get's own
# non-mutating mode, pass the real args ("$@"), add only sanitisation + a hard
# non-mutation guarantee (fail loud, kFAIL-withhold). Useful standalone too.
#
# Naming: ONLY the verb-ladder postfix carries the sanctioned dot [sp-verbladder];
# helpers are plain POSIX (dash-clean). Rename the 3 dotted fns -> all dash.
#   apt-get.check <args>   would it change anything?  rc [sp-checkrc]: 0 conv·1 chg·2 refuse
#   apt-get.diff  <args>   stream the would-change lines
#   apt-get.version        the wrapped tool's own version
# Streaming, never out=$(…) collect-all [sp-nobuffer]. Corpus: apt-get -s plan,
# command -v preflight [src-W1-06]. HOLE(...) = static-extraction gap only; every
# verb below fully executes.

_apt_get_safe_subcommand() {
   case $1 in install|remove|purge|reinstall) return 0 ;; *) return 1 ;; esac
}

_apt_get_guard() {
   command -v apt-get >/dev/null 2>&1 || { echo "apt-get oracle: no apt-get" >&2; return 2; }
   _apt_get_safe_subcommand "$1" || { echo "apt-get oracle: won't vouch for non-mutation of 'apt-get $1'" >&2; return 2; }
   for a in "$@"; do
      case $a in -o|--option|-o?*)
         # a WARNING the admin needs but has no clean channel for yet (notes:
         # q-warn-channel). >&2 is the strawman stand-in:
         echo "apt-get oracle: refusing to probe '$a' (could defeat --simulate); this call stays unoptimised" >&2
         return 2 ;;
      esac
   done
}

apt-get.check() {
   _apt_get_guard "$@" || return 2
   if apt-get --simulate "$@" 2>/dev/null | grep -qE '^(Inst|Conf|Remv) '; then
      return 1
   fi
   return 0
   # NOTE(q-buffering): streaming via grep avoids collect-all, but conflates
   # "tool failed" with "converged" (empty stream -> rc 0); no POSIX pipefail.
}

apt-get.diff() {
   _apt_get_guard "$@" || return 2
   if apt-get --simulate "$@" 2>/dev/null | grep -E '^(Inst|Conf|Remv) '; then
      return 1
   fi
   return 0
}

apt-get.version() {
   command -v apt-get >/dev/null 2>&1 || return 2
   apt-get --version 2>/dev/null
}

# ── HOLEs (static-extraction gaps; runtime logic above is complete) ─────
# HOLE(reserved): KIND BINDING — nothing tells Dorc apt-get manages the same
#   `package` kind a yum/brew oracle would (cross-oracle identity needs a named
#   anchor, the RAL). Explicitly NOT being solved now.
# HOLE(reserved): EFFECT DECLARATION — "apt-get install mutates pkg:<x>" is in
#   prose only; in-band effect-class spelling is the kBURDEN/kOOB open knob.
# HOLE(ceiling): apt-get drags in DEPENDENCIES — precise footprint undecidable
#   [093 f19]; over-approximate or declare.
