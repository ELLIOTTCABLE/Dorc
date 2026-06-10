# gate-1 parity normalizer (task-P item-4 / tc-probe-parity-projection): strip the
# trailing `rc=<n>` from a `site N effect=W rc=<n>` record ONLY when that site's
# AUTHORED record carried no rc — so a site the fixture pins WITH an rc is compared
# with it (the fold-valid Query/pkgstate rc, 20E §2), and a site the fixture omits an
# rc for keeps the effect-only compare (the firewalled establish-site rc).
#
# Two inputs, in order: (1) the authored probe-results.txt (FNR==NR pass — record which
# sites carry rc); (2) the records to normalize (the produced records, or the authored
# file again). Applied identically to both comparison sides so the rc-bearing decision
# is the fixture's alone.
#
# POSIX awk only (the harness's existing dependency); deterministic line transform.
FNR == NR {
   # Pass 1: the authored file. A `site N … rc=<n>` record marks site N rc-bearing.
   if ($1 == "site" && $0 ~ /rc=-?[0-9]+$/) rc_site[$2] = 1
   next
}
{
   # Pass 2: emit each record, stripping a trailing rc= unless its site is rc-bearing.
   if ($1 == "site" && !($2 in rc_site)) sub(/ rc=-?[0-9]+$/, "")
   print
}
