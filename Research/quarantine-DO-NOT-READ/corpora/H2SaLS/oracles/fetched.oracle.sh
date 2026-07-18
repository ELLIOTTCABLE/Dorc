# ============================================================================
# LLM-GENERATED ORACLE SEED — NOT REAL OPS CODE. Part of an intentionally
# quality-varied artificial testing corpus for the Dorc static-analysis
# project; it cannot expose the truth of real-world ops-code and must never be
# run. FROZEN EVIDENCE: the probe body below names a real read-only command
# (test) but is NEVER executed, under any flag or fragment. Validation is
# `dash -n` plus reading. See Research/corpora/H2SaLS/README.md.
# ============================================================================
#
# fetched — a file fetched from the network by wget. Models harden.sh L627
# (`wget -P /etc/audit/rules.d/ <url>` → audit.rules) and the `get_url` helper
# L642-647 (`wget -O "$2" "$1"` → the cisofy key, called L646).
#
# THE CONVERGENCE-IDENTITY FINDING (1A9 §fetched). Two halves, opposite
# difficulty:
#  - PRESENCE is trivially probeable: `[ -e <path> ]`. After a successful
#    fetch the file exists; absence means the fetch failed or never ran.
#  - CONTENT / VERSION is NOT probeable from what the book provides. The book
#    re-fetches EVERY run (L627 is unguarded; `get_url` L642 is unguarded) with
#    NO version pin, NO checksum, NO etag, NO conditional-GET. So the local
#    file carries no identity tying it to a known-good upstream image. To claim
#    "converged" (== up-to-date with intended upstream) honestly, the oracle
#    would need a CONTENT IDENTITY the book never establishes: a pinned sha256
#    to `cmp`/`sha256sum -c` against, or an upstream version the probe could
#    compare. Lacking that, the strongest HONEST claim is PRESENCE — and
#    presence explicitly does NOT mean current (a stale or truncated prior
#    download is "present" too). This is the finding: an unpinned network fetch
#    is presence-probeable but NOT convergence-probeable; the book's
#    re-fetch-every-run is the only "freshness" mechanism, and it is mutative,
#    not a probe.
#
# (Contrast: the `creates:`-guarded gpg --dearmor at L650-653 — `[ ! -e ...gpg ]`
# — IS cleanly presence-probeable, because the book itself gates on existence.
# That guard is the author handing us the probe. Recorded as um-file-gpg-1.)

oracle_kind=fetched

# Probe: existence of the fetched file. `[ -e <path> ]` — read-only QUERY, the
# ONLY honest claim (presence, not currency; see header). A re-fetch still runs
# to refresh content the probe cannot vouch for.
oracle_probe_fetched() { [ -e "$1" ]; }

# A content-pinned selector EXISTS in shape but is UNUSED by this book: if a
# fetch were checksum-pinned (`sha256sum -c <sums>` or `cmp` vs a vendored
# image), THAT would be the convergence probe. The book pins nothing, so this
# selector has no reference to check and the resolver never binds it — present
# here to DOCUMENT the missing identity, not because the corpus exercises it.
oracle_probe_fetched_pinned() { sha256sum -c "$2" >/dev/null 2>&1; }

# Effects: the fetcher is `wget` — it DOES establish the file. But wget's rc is
# a MUTATOR's rc (a network action with effects), and wget is generic (fetches
# any URL to any path). The (provider,verb) key would be `wget '' establish
# present` — BUT declaring `establish` invites the apply to treat a present
# file as discharging the fetch, which is exactly the unsound currency
# assumption above (present != current). So we declare only the read-side
# Query; the fetch's effect is intentionally NOT modeled as a convergence-
# discharging establish. (A presence establish would be sound for the
# `creates:`-guarded case but unsound for the unguarded re-fetch — and the book
# uses BOTH, so the conservative choice is to declare neither, 1A9 §fetched.)
oracle_effect test '' query present

# command-keyed check(): `[ -e <path> ]` / `test -e <path>` — the presence
# idiom, same resolver shape as crond. Strip the existence flag, bind the path
# as the entity, refuse compound tests (bind nothing ⇒ run).
test__check() {
   case $1 in -e|-f|-s) shift ;; esac
   path : fetched = "$1"; shift
   case $1 in
      ''|']') [ -e "$path" ] ;;
   esac
}
