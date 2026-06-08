# What Dorc's PROBE phase ships to a host — the COMPILED PROBE, lifted from a book's CFG by static analysis.
# Sanitized/cleaned from the human's sketch (2026-06-08). Grounds 17O R2-PROBEGATE + the probe model;
# inlined (tightened) into Research/plans/17N §3.
#
# Properties it MUST have:
#  - read-only w.r.t. the MANAGED system: every potentially-mutating command is either an oracle-vouched
#    read-only check (an INTERCEPTOR) or omitted. The book's mutators do NOT appear.
#  - lifted from the CFG; independent leaves are dispatched CONCURRENTLY (the point of read-only probes).
#  - output OUT-OF-BAND (kCOMMS / plans/142): per-leaf freeform -> scratch files demuxed by filename; short
#    gating verdicts -> a SEPARATE lane (never mixed with freeform — the GitHub set-output injection CVE; P6).
#  - lifts the oracle BODIES + (only where needed) minimal CFG, NOT the book's contents — so it never inherits
#    the book's `trap`s (17O R2-TRAP).
#
# ----------------------------------------------------------------------------
# SOURCE book it is compiled FROM (the R2-PROBEGATE shape — a probe-gated subtree):
#   if getent group app >/dev/null 2>&1; then
#      if ! id -nG deploy | grep -qw app; then
#         systemctl is-active --quiet app || systemctl start app   # MUTATOR fallback
#         usermod -aG app deploy                                    # MUTATOR
#      fi
#   fi
# ----------------------------------------------------------------------------

D=${DORC_SCRATCH:?run-scoped writable scratch dir on the target — NOT managed state}
V=${DORC_VERDICT:?the gating-verdict fast-lane — a channel separate from freeform output}

# --- oracle interceptors: shipped BECAUSE id.check() exists; replaces the `id` invocation in the probe ---
# (the real oracle body asserts arg-shape + sanitizes + vouches read-only; stub just calls the real binary.)
id__check() { command id "$@"; }

emit() { printf '%s\trc=%d\n' "$1" "$2" >>"$V"; }   # verdict on its own lane; %d (the rc), never %n

# === variant A: FLAT — independent leaves (the dispatcher runs these concurrently across the connection) ===
getent group app                >"$D"/p1.out 2>&1; emit p1 "$?"
id__check -nG deploy            >"$D"/p2.out 2>&1; emit p2 "$?"
systemctl is-active --quiet app >"$D"/p3.out 2>&1; emit p3 "$?"

# === variant B: CFG PRESERVED — p2/p3 are valid/inert ONLY under p1's guard ===
# (Dorc reserves the right to replicate a guard when a downstream probe is meaningless or non-inert unless an
#  upstream check held — e.g. probing membership before the group is known to exist.)
getent group app >"$D"/p1.out 2>&1; rc1=$?; emit p1 "$rc1"
if [ "$rc1" = 0 ]; then
   id__check -nG deploy            >"$D"/p2.out 2>&1; emit p2 "$?"
   systemctl is-active --quiet app >"$D"/p3.out 2>&1; emit p3 "$?"
fi

# The controller reads $V (verdicts demuxed by leaf-id) + the $D/*.out files, and builds the apply-plan from
# the ACTUAL results. A probe-gated branch (R2-PROBEGATE) is resolved by RUNNING the read-only probes for
# real — Dorc is NOT blind past a result-gated branch the way Ansible check-mode is, precisely because its
# probes are read-only and DO execute. The only residue is a gate on a MUTATION's result = the run-delta
# class (R2-CHANGEDELTA): see r17-crosscheck-dorc-inputs.straw.sh.
