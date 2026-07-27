#!/bin/sh
# guard26-classed-decline-demotes-guard — KNOWN DEFECT, pinned. The golden records what the engine
# does TODAY, which is NOT what it should do; do not read it as the specified answer.
#
# Same three command lines and the same `wombat` oracle as `guard26-unmodeled-wall-guards-below`.
# The only difference is the added `hork.oracle.sh`, which classes a decline for `hork` — and that
# one file takes both drops from GUARD to RUN.
#
# Why: an unmodeled `hork` is Opaque and WALLS, so the drops below it classify `EstablishWritten`
# and reach the guard tier. Once `hork` bears a verdict function it establishes a cell of its own
# instead of going Opaque, so it stops walling and the drops classify `EstablishAmbient`. `hork`
# still RUNS (it declined, so nothing vouches it), so the drops sit below a live mutator and their
# elision is correctly refused — but the guard tier, which exists for exactly that case ("the world
# may have moved, so re-check live"), is keyed to `EstablishWritten` and so is unreachable. Neither
# tier applies, and both drops run.
#
# The evidence each drop carries is identical across the pair: a reached vouch and a `holds` record
# on its own authored cell. Only the upstream site's classification differs. So an author who
# classes an honest decline gets a strictly worse plan than one who ships no oracle at all, which
# inverts what the contract asks for.
#
# EXPECTED once the tier boundary is repaired: both drops guard here exactly as in the sibling.
# That is a licensing-tier change (which class may reach the guard mint), deliberately NOT made
# here; when it lands, this golden moves and the two cases agree.
hork provision
wombat a.conf /etc/a.conf
wombat b.conf /etc/b.conf
