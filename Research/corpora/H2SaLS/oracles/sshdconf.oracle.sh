# ============================================================================
# LLM-GENERATED ORACLE SEED — NOT REAL OPS CODE. Part of an intentionally
# quality-varied artificial testing corpus for the Dorc static-analysis
# project; it cannot expose the truth of real-world ops-code and must never be
# run. FROZEN EVIDENCE: the probe bodies below name real read-only commands
# (sshd -T, grep) but are NEVER executed, under any flag or fragment.
# Validation is `dash -n` plus reading. See Research/corpora/H2SaLS/README.md.
# ============================================================================
#
# sshdconf — the PER-SURFACE showcase: a kind for sshd's effective
# configuration, probed SEMANTICALLY (by option), not textually. This is the
# opposite end of the spectrum from `confline` (path×line, textual): here the
# entity is a config SURFACE with named, typed options, and the probe knows
# what an option MEANS. Models harden.sh §3 (L160-307): the `set_sshd_line`
# loop (L262-273), the Port branches (L281-294), the mozilla block (L194-259).
#
# THE FILE-VS-EFFECTIVE GAP IS THE RESTART GAP (the headline finding — 1A9
# §sshdconf). Two candidate truths a probe could report:
#   (a) `grep` on /etc/ssh/sshd_config — the FILE text as written.
#   (b) `sshd -T`                      — the EFFECTIVE config sshd computes by
#       PARSING that file (merged with compiled-in defaults, keys lowercased).
# CRITICAL: `sshd -T` reports the config the file WOULD load — it re-parses the
# file; it does NOT introspect the RUNNING daemon's in-memory config (sshd has
# no such live query). So BOTH (a) and (b) read the FILE, not the running
# service. The book edits the file (L262…) then defers `service ssh restart` to
# an end-of-run change-flag (L685). Between edit and restart, the FILE says the
# new value but the DAEMON still serves the old. A probe on either (a) or (b)
# would therefore report CONVERGED the instant the file is edited — BEFORE the
# restart that actually makes it true. The host-truth a probe CANNOT see is
# "what the live daemon loaded"; convergence of sshdconf is a FILE claim, and
# any apply-elision built on it silently assumes the restart already happened.
# This kind chooses (b) `sshd -T` and DOCUMENTS that it is a file-parse claim,
# not a running-daemon claim. (See um-file-restart-1 for the run-delta side.)
#
# WHY `sshd -T` OVER `grep` (the per-surface payoff): the book writes
# `PermitRootLogin no` but a value may also arrive via Match blocks, Include
# files, or a compiled default — `grep '^PermitRootLogin'` misses all of those
# and reports absent-when-effectively-set. `sshd -T` gives the MERGED truth, so
# the probe answers the real question ("is rootlogin effectively no?") rather
# than the textual one ("is that literal line in that file?"). That semantic
# lift is the entire reason a per-surface kind beats the generic `confline`.

oracle_kind=sshdconf

# Kind-default probe: validate the config parses at all. `sshd -t` (test mode)
# exits non-zero on a syntactically broken sshd_config — a read-only QUERY of
# the surface's well-formedness. Not "converged", just "loadable"; the
# per-option selectors below carry the actual value checks.
oracle_probe_sshdconf() { sshd -t >/dev/null 2>&1; }

# Per-option selectors. `sshd -T` prints `keyword value` lines, keyword
# LOWERCASED, one effective value per option. Each selector greps the dump for
# its option's converged value. `$1` is the host/config entity (unused in the
# argv — `sshd -T` reads the local sshd_config; a remote probe would ssh first,
# but the kind entity identity is still "this host's sshd surface"). These
# model the loop options the book SETS (L262-273) + Port (L281-294).
#
# THE HONEST SUBSET — only options whose converged value this book fixes to a
# single literal AND that `sshd -T` reports as a clean scalar. Each greps for
# the exact effective line `sshd -T` would emit when converged.
oracle_probe_sshdconf_permitrootlogin() { sshd -T 2>/dev/null | grep -qx 'permitrootlogin no'; }
oracle_probe_sshdconf_passwordauthentication() { sshd -T 2>/dev/null | grep -qx 'passwordauthentication no'; }
oracle_probe_sshdconf_x11forwarding() { sshd -T 2>/dev/null | grep -qx 'x11forwarding no'; }
oracle_probe_sshdconf_permitemptypasswords() { sshd -T 2>/dev/null | grep -qx 'permitemptypasswords no'; }
oracle_probe_sshdconf_maxauthtries() { sshd -T 2>/dev/null | grep -qx 'maxauthtries 5'; }
oracle_probe_sshdconf_logingracetime() { sshd -T 2>/dev/null | grep -qx 'logingracetime 30'; }
oracle_probe_sshdconf_clientaliveinterval() { sshd -T 2>/dev/null | grep -qx 'clientaliveinterval 300'; }
oracle_probe_sshdconf_clientalivecountmax() { sshd -T 2>/dev/null | grep -qx 'clientalivecountmax 0'; }
oracle_probe_sshdconf_maxsessions() { sshd -T 2>/dev/null | grep -qx 'maxsessions 2'; }
oracle_probe_sshdconf_allowtcpforwarding() { sshd -T 2>/dev/null | grep -qx 'allowtcpforwarding no'; }
oracle_probe_sshdconf_port() { sshd -T 2>/dev/null | grep -qx 'port 55899'; }
#
# RECORDED-AS-REFUSED options this book sets that do NOT get an honest selector
# (1A9 §sshdconf un-modelable tail):
# - Subsystem (L273): `sshd -T` emits `subsystem sftp ...` with the command
#   path; the book's value embeds `-f AUTHPRIV -l INFO` — exact-match brittle
#   across OpenSSH versions; value comparison is not a clean scalar.
# - Protocol 2 (L219): removed in modern OpenSSH; `sshd -T` does not emit it.
#   A converged-value probe would assert a key that never appears → always
#   "absent". Honestly unprobeable on current sshd; left out.
# - HostKey (L201-203): multi-valued (three lines); `sshd -T` lists all host
#   keys. Set-equality, not scalar-equality — outside the one-line selector
#   shape. Recorded.
# - AllowGroups/ListenAddress/MaxStartups/PermitUserEnvironment/etc.: probeable
#   in principle (same shape) but OMITTED here to keep the seed a focused,
#   HONEST subset rather than an exhaustive transcription. The shape generalizes
#   trivially; the finding is the shape, not the row count.

# Effects: the value-SETTING side. The book sets sshd options via `sed -i` /
# `printf >>` on the file (through `set_sshd_line`), and the change takes effect
# via `service ssh restart`. NEITHER is an sshd-specific command:
# - `sed`/`printf`/`>>` are generic file mutators (see confline — no provider
#   token to key, the mutation is a redirect or a generic editor).
# - `service ssh restart` IS the establish-the-effective-config action, but it
#   is the `service` provider's verb, not sshdconf's — it belongs to a service
#   oracle, and it establishes "running with current file", not any specific
#   option value.
# So sshdconf declares ONLY the read side: `sshd -T` is a Query of the surface.
# There is NO sound `establish` effect to declare on this kind — the per-option
# convergence is set by generic editors the analyzer cannot bind here. This is
# the per-surface kind's structural limit: it can READ a surface semantically
# but cannot OWN the writes, because the writes have no surface-specific verb.
oracle_effect sshd '' query effective

# command-keyed check(): `sshd -T` / `sshd -t` bind NO verb (verbless probe of
# the local surface). There is no operand to annotate as the entity in the
# common `sshd -T` form — the surface IS the local host's sshd_config, an
# implicit singleton. We annotate the conventional `-f <file>` operand when
# present (an explicit config path), else bind the implicit surface. REFUSE any
# form we don't recognize (bind nothing ⇒ run).
sshd__check() {
   mode=T
   conf=''
   while [ "${1#-}" != "$1" ]; do
      case $1 in
         -t) mode=t ;;
         -T) mode=T ;;
         -f) shift; conf=$1 ;;
      esac
      shift
   done
   # the entity is the sshd surface; `-f`'s path names it explicitly, else the
   # implicit local surface. Bind the path, then annotate it as `sshdconf`
   # (plain positional-style binding — no parameter-default expansion, to stay
   # inside the constrained check() dialect the exemplars demonstrate).
   if [ "$conf" = "" ]; then conf=/etc/ssh/sshd_config; fi
   surface : sshdconf = "$conf"
   case $mode in
      t) sshd -t >/dev/null 2>&1 ;;
      T) sshd -T >/dev/null 2>&1 ;;
   esac
}
