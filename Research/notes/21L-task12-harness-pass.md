# 21L — task #12: the harness pass (build record + DST position)

> Orchestrator note distilled from the harness builder's report (Opus, worktree-built
> on `e512cc3`, four granular slices, each gated; harvested by cherry-pick as
> `deadd3a` / `a5c1bef` / `59bb8b7` / `bf07208` plus the orchestrator's follow-up
> protocol patch for the three `omitsafe21-*` cases that landed from a concurrent
> builder after the sweep's base). Charter: 21Y task card #12 + the close-session
> contracts (mock-log grammar, EXIT_RC semantics, dual-rail judge semantics, dc-1
> pinning set — pre-spelled by the conductor after reading run.sh/21D/221). AI-built;
> process evidence, never proof.

## slice-1 — `EXIT_RC=<n>` case marker (`deadd3a`; closes tc-exec-nonzero-exit)

A per-case marker FILE (the `RAN_ORDER=lax`/`PROBE_RESULTS=authored` idiom): present ⇒
`exec_check` asserts the APPLY artifact's real exit status == n exactly (0-when-n≠0
also fails — equality, not tolerance); absent ⇒ rc==0 as before. Probe artifacts are
never expected nonzero; BLESS never creates/consults the marker; >1 marker or a
non-integer suffix refuses loudly. `door1-and-form` (215 §3's analysis-only case)
CONVERTED to exec-asserted: mocks + hand-derived empty `expected.ran` + `EXIT_RC=1` —
the faithful `set -e; false && {…}` rc-1 artifact is now executed, not just parsed.
Negative-tested four ways (marker removed / wrong n / non-int / duplicate ⇒ each FAILs).
Door-4's env-sick four-world pole (218 §6's sequencing dependency) now has its
harness prerequisite.

## slice-2 — the determinism rail (`a5c1bef`; 221 dc-1, msys-honest)

All three artifact-execution sites (`exec_check`, `probe_exec_check`, gate-5's bare
run) now execute under `env -i PATH=<mocks> DORC_LOG=<log> LC_ALL=C TZ=UTC` with
`umask 022` set in the execution subshell. `env -i` verified portable on this
msys/dash box in all three invocation forms; no unset-fallback needed. ZERO golden
churn — which doubles as evidence no corpus case depended on ambient env.

**The DST position (the documented residual lax-set, in run.sh's header):** the rail
pins the shell-exec environment (PATH/locale/TZ/umask/env-scrub) and the log encoding;
it deliberately does NOT pin: filesystem state beyond the fresh sandbox dir; mktemp
path names (never asserted on); the checker binary's identity/version (first
`dash`/`sh` on PATH); kernel/msys-vs-POSIX syscall differences; host facts reachable
by an artifact's own reads (`hostname`/`id`/`pwd` — none in the corpus today); and
wall-clock/RNG via shims calling `date`/`$RANDOM` (the shims don't). Kernel-level
determinism remains the Rust hostsim seam (21D); this rail fixes the e2e corpus's
shell-exec layer only.

## slice-3 — gate-6, the dual-rail license judge (`59bb8b7`; cm-1 at corpus tier)

The deferred cm-1 product-gate (20K §3/§4), msys-tier: for every mocks-bearing,
non-`PROBE_RESULTS=authored` case, rail-1 runs the BARE book under mocks, rail-2 the
eliding APPLY; the judge asserts, conservatively and one-directionally: (i) apply's
ran-lines ⊆ bare's (apply never runs anything new — a comment marks where door-4-era
work amends this); (ii) every bare-only line is license-attributable to a
`replace`/`omit` site from the engine's own `--debug-argv` ledger, with TOP treated as
a position-wildcard (necessary — strict TOP-skipping false-fails every converged
loop; this is 21D's `argv_matches` ported). The replace/omit filter lives in the
judge, so the self-test can prove `run` is NOT a license.

**The confound battery (a judge that can't scream is worse than no judge):**
`dual_rail_selftest` runs at harness start and aborts the whole harness (exit 3) on
failure — cf-1 apply-only line screams · cf-2 unattributable bare-only line screams ·
cf-3 a `run`-disposition does NOT attribute · cf-PASS the TOP-wildcard DOES license
converged members. The battery earned its keep before first commit: it caught the
judge splitting argv under the caller's inherited `IFS=newline` (fixed with a
subshell `unset IFS`) — a lying-judge bug that would have silently passed everything.

Validated: 0 false-fails across all eligible cases pre-wiring; disabling a marker
flips a real case to FAIL (load-bearing). Honest non-coverage, documented in-header:
host-state variation and two-directional branch-divergence attribution stay 21D's
domain; the TOP-wildcard has no independent convergence cross-check here (21D's
`removed_line_is_converged` has no corpus-tier analogue).

Exclusion markers (honesty over reach): `DUAL_RAIL=inlined` ×3 — the engine's ledger
reports CALL-site surface argv while the bare run logs inlined-BODY argv
(inv-leaf-seam's non-injectivity at the corpus tier; tc-gate6-inlining, the hostsim
differential attributes these in-process); `DUAL_RAIL=multiline-argv` ×2 — see
slice-4's finding.

## slice-4 — the newline-safe mock-log protocol (`bf07208`; the all-cases sweep)

A shared per-mocks-dir DOTFILE helper `mocks/.log` (invisible to the `ls`-derived
shimset) sourced by every shim (`. "${0%/*}/.log"; _dorc_logged "$@"`), encoding each
arg with shell builtins only (PATH is mocks-only): backslash → `\\` first, then
newline → the two chars `\n`. A clean argv encodes to itself, so the per-arg
space-join is byte-identical to the old `"$*"` format — zero golden churn, verified
across all 74 swept cases (157 shims rewritten; two non-logging mocks files correctly
untouched). The orchestrator applied the same idiom to the three `omitsafe21-*` cases
that post-dated the sweep's base (6 shims + 3 helpers, follow-up commit).

**The sweep's finding (tc-gate6-multiline-argv):** two multiline-argv cases had been
PASSING gate-6 only by accident — the un-encoded newline split the bare log so its
first fragment coincidentally matched the engine's first-line-only `--debug-argv`
rendering. The encoding removed the accident; honest disposition = the
`DUAL_RAIL=multiline-argv` exclusion markers, carried until the ledger's argv
rendering is newline-faithful. A coupling note for history: those markers live in the
sweep commit because the hazard only exists post-encoding.

## Carried tc-flags

tc-gate6-inlining · tc-gate6-multiline-argv (both above; both honest corpus-tier
limitations, not engine findings — the engine held on every case the harness could
judge) · the EXIT_RC marker governs apply only (probe-nonzero unsupported,
deliberately) · 221 dc-1's full env-allowlist ambition is satisfied at the
"scrub-everything, allow-four" tier; anything finer is r22's.
