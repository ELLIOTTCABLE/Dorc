# ============================================================================
# LLM-GENERATED ORACLE SEED — NOT REAL OPS CODE. Part of an intentionally
# quality-varied artificial testing corpus for the Dorc static-analysis
# project; it cannot expose the truth of real-world ops-code and must never be
# run. FROZEN EVIDENCE: the probe body below names a real read-only command
# (test) but is NEVER executed, under any flag or fragment. Validation is
# `dash -n` plus reading. See Research/corpora/H2SaLS/README.md.
# ============================================================================
#
# crond — a cron.d drop-in. Models harden.sh L553-557:
#   cat > /etc/cron.d/clamav-daily <<EOF ... EOF ; chmod 0644 ...
#
# THE IDENTITY FINDING (1A9 §crond): a cron.d drop-in is JUST A FILE. The
# tempting abstraction is "the entity is the cron JOB" (schedule × command ×
# user), with a probe like "is this job scheduled?". But that abstraction does
# NOT survive contact with the system: cron has no clean query for "effective
# cron.d entries" — `/etc/cron.d/*` files are directory-scanned by crond with
# no `crontab -l`-style listing, and the running crond's loaded set is not
# host-queryable. (Contrast a USER crontab, where `crontab -l -u <user>` IS a
# real per-user query — that would be a different, genuinely job-keyed kind.)
# So for the cron.d-FILE form the book actually uses, "job" collapses to
# "file", and crond is structurally a thin alias of `confblock`: path-keyed,
# probed by existence/content. The drop-in's INTERNAL `MAILTO=$MAIL_TO` and the
# schedule line are runtime-$VAR-bearing content (L554) — same un-knowable-bytes
# wall as confblock's unquoted heredocs. We probe presence only.
#
# RECORDED as the finding, not engineered around: that a "cron job" is not a
# probeable entity here — only the file backing it is — is itself the lesson.

oracle_kind=crond

# Probe: existence of the drop-in file. `[ -e <path> ]` — a read-only QUERY.
# Honest floor: the drop-in is installed. NOT "the job is active" (crond's
# loaded set is unqueryable) and NOT "content is current" (runtime $VAR in the
# body, L554). Presence is the only sound claim.
oracle_probe_crond() { [ -e "$1" ]; }

# Effects: the writer is `cat > /etc/cron.d/... <<EOF` — `cat` to a `>`
# redirect, the same no-provider-token-to-key situation as confblock. The
# subsequent `chmod 0644` (L557) is a mode mutation on the same path, also not
# a crond-specific verb. So crond declares only the existence Query; there is
# no sound establish verb (the write is shell redirect syntax).
# ── COMMENTED OUT (orchestrator, tc-F2 adjudication — see notes 1A9 + the
# round's matrix). This kind's resolver claimed provider `test`, and so does
# fetched.oracle.sh's: two oracles cannot both own one provider (the
# (kind,provider,verb) index would clobber or refuse — cross-oracle
# coherence). That collision is exactly the category error 1A9 tc-F2 names:
# generic read-builtins (`test`/`[`/`grep`/`cmp`) as oracle-keyed providers do
# not scale past one claimant. fetched.oracle.sh keeps the live `test`
# resolver (the book's gpg-keyring `[ ! -e … ]` guard is a real site); crond
# has ZERO `[ -e ]` sites in this book (its cron.d write is unguarded), so its
# copy is preserved INERT below as evidence of the problem, not as code.
#
# oracle_effect test '' query present
#
# test__check() {
#    case $1 in -e|-f|-s) shift ;; esac
#    path : crond = "$1"; shift
#    case $1 in
#       ''|']') [ -e "$path" ] ;;
#    esac
# }
