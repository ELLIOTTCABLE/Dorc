# 23C — adversarial review of the guard23 pin-set (independent red-team pass)

AI-written, 2026-07-02. Scope: the 19 `spike/e2e/cases/guard23-*` cases (6 XFAIL pins + 13
floors) against the signed round-23 rulings (spike/CLAUDE.md), plans/233 (incl. end
annotation), plans/239, and the harness (spike/e2e/run.sh). Explicitly NOT read: notes/23A,
23B, 23Z (per brief). Out of scope by brief: vouch sh-spelling, errexit-implicit consumption
(both directions), command-disassembly.

Method: full manual trace of every case (book → probe-results → goldens → mocks → gate path),
suite executed at HEAD (118/118 green, exactly the 6 guard23 xfails), plus four live
demonstrations in scratch + a fake-engine XPASS demonstration against a temporarily-widened
judge (run.sh restored afterward; worktree clean).

Verdict up front: the fall-through core is genuinely well-pinned (drift/canttell/mutator-fails
have real run-set teeth; the no-mint controls no-vouch/rundelta/multioperand catch wrong mints
at exec). The holes are (I) an environment-capture family the signed strip-only/whole-body law
itself creates, un-exercised by any pin — one member DEMONSTRATED as silent under-execution;
and (II) an enforcement gap: the artifact-shape law (two nevers, bytes-verbatim) lives only in
goldens the XFAIL/XPASS mechanism never reads.

## I. Under-execution reachable under the pinned law

### 23C-fd1 · DEMONSTRATED — guard-body variable capture silently drops a required install
The pinned preamble is the oracle check body verbatim (strip-only; signed). POSIX functions
share the caller's variable namespace; the fixture bodies assign `verb` and `pkg` bare.
Composition (all pieces individually pinned/licensed): book
`pkg=vim; hork wombat; apt-get install -y curl; apt-get install -y "$pkg"` — curl site vouched
+ converged-past-wall ⇒ guard; vim site diverged ⇒ run with ORIGINAL BYTES (`"$pkg"` spelling
survives verbatim, per rul-ternary-verdict; `pkg=vim` is propagatable per
exec-resolved-var-elides precedent so the site resolves + probes absent).
Bare book run-set: `hork wombat · apt-get install -y curl · apt-get install -y vim`.
Pinned-transformation artifact run-set (dash, inert mocks, harness exec discipline):
`hork wombat · dpkg-query -W curl · apt-get install -y curl` — the guard call's `pkg="$1"`
clobbers the book's `pkg`, so the final line installs curl (which the guard itself just
suppressed as converged) and VIM IS NEVER INSTALLED. Under-execute + unnecessary-execute in
one stroke. The engine cannot see this at analysis time (insertion happens at render, after
value-flow). 233's hazard-class prose names the book-environment risk; no pin composes a guard
with downstream variable reads. reingest-collision-verbatim pins the FUNCTION-name half and
declares renames unspellable under strip-only; the variable half is strictly worse (`pkg`,
`verb`, `svc`, `now` are ubiquitous book vars — the 233 §1 strawmen themselves use them) and
is unpinned. Mitigation shapes (not mine to pick): oracle-contract `local` discipline in check
bodies (dash ≥0.5.13 has `local`; contract-tier, pinnable as a lint/refusal), or
refuse-the-mint when the check body's assigned names intersect the book's live variable set
(the engine has both sets statically).

### 23C-fd2 · DEMONSTRATED — `set -u` book: the inserted guard kills the whole book tail
Fixture check bodies read `"$2"` unconditionally; a single-operand invocation leaves `$2`
unset. Book `set -u; hork wombat; apt-get install -y curl; apt-get install -y vim`: bare
completes rc 0 (3 commands); the pinned artifact dies rc 2 at the check's `"$2"` read
(`parameter not set`) — curl, vim, everything downstream under-executed by the insertion.
Distinct axis from the excluded errexit-consumption question (nounset is the inserted code's
own crash, not rc-consumption semantics); it is the other half of 218a's named
"`set -e`/`set -u`" environment hazard. No guard23 book sets -u. Same mitigation family as
fd1 (`${2:-}` spelling is contract-tier; strip-only forbids the engine adding it).

### 23C-fd3 · argued (mechanics empirically confirmed) — dynamic-path escape from the static vouch
rul-guard-license scopes the vouch by CONSTANT-PROPAGATION reachability (static); the shipped
whole body path-selects at RUNTIME (rul-ternary-verdict blesses this). A check body with a
world-dependent branch — 233's own "capability fallbacks" — can, after drift, terminate rc 0
via an UNVOUCHED path: strawman `if command -v newtool >/dev/null 2>&1; then newtool query
"$1"; else :; fi`, vouch on the newtool path, newtool removed by the wall-former between plan
and apply ⇒ else-branch ⇒ rc 0 ⇒ mutator suppressed with ZERO live convergence verification.
Both fixture check bodies already exit rc 0 on their refuse-paths (verified live: two-operand
apt check rc=0; unmodeled-verb systemctl check rc=0), so the corpus's own oracle idiom
defaults fail-OPEN. The drift pins (fallthrough-drift/canttell) cover only rc≠0 drift; nothing
pins drift-into-an-rc-0-unvouched-path. multioperand-atomic's comment says "the witness's
reached-path component is load-bearing exactly here" but pins only the no-mint side. This is
the sharpest design-adjacent hole: under-execution one storey below the witness, with an
unreliable-but-contract-compliant oracle. (One-body-two-lanes — the flagged, not-GO'd
candidate invariant — is what would at least exercise these bodies at probe time.)

## II. Enforcement gaps (wrong pin gets built to / green for the wrong reason)

### 23C-fd4 · DEMONSTRATED — XPASS never reads expected.out; the artifact-shape law has no teeth
run.sh skips the content golden-diff for XFAIL cases (`-z "$xfail_reason"` guard); the XPASS
trigger is gates-only (run-set, probe records/parity, diagnostics, why-lens). Every pinned
artifact property that lives only in golden TEXT — strip-only preamble sourcing, `check ||
original-bytes` shape, never-engine-synthesized, never-testimony-as-output, bytes-survive-
verbatim — is documentation, not assertion. Demonstrated end-to-end: a fake engine emitting,
for the flagship, an artifact violating never-1 (engine-synthesized thin guard, no oracle
body) AND bytes-verbatim (`-y` dropped from the fall-through) XPASSes — the suite prints
"known defect appears FIXED — promote this case". Post-promotion the content diff would bite
once — unless the builder runs BLESS, which regenerates expected.out from engine output
(gates-pass is bless's only precondition). Cheap hardenings: (a) a harness assertion for
guard-bearing cases that the apply text between the two shebangs contains the literal
`<check-name> … || <original line bytes>` for each guarded site (a grep-level floor, not a
full diff); (b) a standing rule that promotion of a guard23 XFAIL must diff engine output
against the hand-authored golden BEFORE any bless (process, but write it down where the
builder will trip over it).

### 23C-fd5 · flagship/heredoc XPASS is gate-6-blocked; the forced widening is unpinned
gate-6 direction (i) (apply ⊆ bare) screams at any guard's apply-only check invocation
(`dpkg-query -W curl`), so the flagship literally cannot XPASS until the builder edits the
dual-rail judge — the in-file comments even invite it ("door-4-era … widen direction (i)
then"), and direction (ii) needs `guard` admitted as a license class. Neither widening has a
selftest confound: dual_rail_selftest's cf-1..3/cf-PASS none contain a guard disposition, so a
widening that simply skips direction (i) for guard-bearing cases (what I implemented for the
demo — two obvious lines) passes the battery while going blind to cf-1-class apply-only
mutations exactly where guards live. The pin-set forces a safety-gate edit and pins nothing
about the edited gate. Hardening: add cf-5 (a guard-disposition ledger + an apply-only line
that is NOT the guarded site's check ⇒ must still scream) and cf-6 (guard-disposition
attributes its own site's bare-only line ⇒ must not scream) to the battery BEFORE the build
round; they're fabricated-string tests, ~10 lines.

### 23C-fd6 · DEMONSTRATED — already-hand-guarded floor's claimed run-set rail is false
The case comment claims a stacked machine-guard would surface as "a second check invocation
appearing in the run-set". It cannot: mocks/ has NO dpkg-query shim, so the stacked check's
probe 127s invisibly (the check's own `2>/dev/null` swallows it), falls through to the hand
guard, and the run-set is byte-identical to expected.ran (demonstrated under the case's own
mocks; rc 0). Only the content diff holds the no-double-guard line — the exact gate BLESS
rewrites. One-file fix: add a dpkg-query shim (either rc: rc 0 ⇒ suppression changes the
run-set; rc 1 ⇒ the extra invocation appears; both red). Same shim-absence weakens
top-argv-runs (a wrong guard there 127s invisibly too).

### 23C-fd7 · no apply-lane 127 detector
gate-1(c) catches un-shimmed PROBE commands loudly (rc=127 ⇒ vouch-closure fail). Guards
import probe-code into the APPLY lane, where an un-shimmed check command silently 127s into
fall-through (fd6's mechanism) — safe-direction at runtime, but it makes exec_check blind to
whole classes of wrong-guard (and of missing-shim fixture rot). The gate-1(c) rationale
transfers verbatim; nothing implements it. Hardening: exec_check gains a (c)-analogue —
refuse any apply run whose stderr shows `not found` from the artifact, or shim-set-closure
over guard check bodies.

## III. Missing pins (forbidden-but-mintable, or decisions that will get built by accident)

- 23C-fd8 · vouched site past wall, probe = CANT-TELL (or record absent): may the witness
  mint? Flagship pins holds⇒guard and absent⇒run; the third verdict of the probe vocabulary
  is unpinned. A cant-tell mint guards a line the approved plan displayed as RUN — a
  front-load-doctrine breach (decision made post-approval) even though runtime-safe-ish.
  Either behaviour XPASSes today. One sibling case (mock rc 2 at PLAN time, i.e. authored
  cant-tell record + vouch) pins it.
- 23C-fd9 · cross-oracle vouch attribution: no case composes (oracle A vouched) with
  (oracle B's un-vouched site past a wall). A build keying "a vouch exists" / provider-set
  membership rather than THIS-site's-oracle's-reached-path mints B-guards off A's vouch;
  nothing reds. rundelta covers verb-scope within ONE oracle; no-vouch-runs covers zero-vouch
  only. Cheap: add a service site (unvouched service oracle) past the wall to a variant of
  no-vouch-runs whose package oracle IS vouched.
- 23C-fd10 · redirect-carrying guardable line: HEAD's elide tier refuses non-devnull
  redirects (enclosing-group-redir runs a CONVERGED `{ …; } > /var/log/…`; exec-devnull-exempt
  elides `>/dev/null`) — i.e. the elide law treats an admin-spelled file-redirect as a
  blocking side-effect. The guard-tier analogue is unpinned: a guard's pass-direction
  suppresses that same admin-spelled file side-effect (`cmd >>log` appends nothing on pass).
  The ratified refuse-homes list (background/subst/heredoc) is silent on redirects. Pin one
  direction consciously; suggest symmetric refuse (or vouch-covers-it, argued), either way a
  case.
- 23C-fd13 · why-attribution patterns are three independent substrings over all why-lines
  (`guard`/`vouch`/`package`). All three are absent at HEAD (verified: the case emits zero
  why-lines), so it is not one-bug-from-XPASS; but any build emitting a "vouch present but
  guard tier disabled" note plus ordinary package-kind lines XPASSes it. Cheap tightening:
  one conjoined per-line pattern (gate-7 is per-line substring matching).

## IV. Hygiene / drift

- 23C-fd12 · stray SyncThing conflict file committed inside the flagship case:
  `guard23-ternary-flagship/head-output.sync-conflict-20260702-032713-PHNHRER.txt` (a scratch
  HEAD-output capture). Harness-inert (no glob reads it), but it is repo clutter, evidence the
  pin-set was authored while the sync race was live, and device PHNHRER is the known conflict
  source. Delete; eyeball that no golden carries a conflicted twin.
- 23C-fd14 · lane-sourcing drift baked into the (unasserted) goldens: the pinned probe halves
  ship st-2 `oracle_probe_*` bodies while rul-ternary-verdict's sourcing sentence says guard
  bytes are "the same bytes the probe lane ships", and 233's end-annotation (human ground
  truth, correction 1) says the stripped CHECK body ships in BOTH lanes. Because of fd4 the
  discrepancy is documentation-tier, but it sits in the exact clause the builder will
  implement from; and the not-GO'd one-body-two-lanes candidate is what would have surfaced
  fd3's bodies at probe time. Flag to the human rather than resolve.
- The HEAD renderer's `# site:N skip-unresolvable` comment uses the banned word "skip"
  (pre-existing, visible throughout the new goldens).

## V. What held (checked and NOT broken)

- Every floor's expected.ran equals the licensed delta of its bare book at HEAD; gate-6
  attributes the only true elisions (inloop, no-vouch site 0, vouch-inert pair) mechanically.
- fallthrough-drift/canttell/mutator-fails: exec-level fall-through teeth are real; a
  suppressing build stays red (never wrongly green) — the single most safety-critical
  direction is soundly pinned, three ways.
- multioperand-atomic and rundelta have genuine two-sided exec teeth (wrong mint ⇒ run-set
  diverges in BOTH the extra-check and missing-mutator directions; verified rc-0 refuse-paths
  make the suppression visible as an EMPTY run-set).
- no-vouch-runs (the control) is the best-toothed floor: its dpkg-query shim (rc 0) makes a
  wrong mint doubly visible. vouch-inert-pair-a/b's differential is sound (no wall ⇒ tier
  lands with pair unchanged); its "goldens byte-identical forever" invariant is
  reviewer-discipline only (no mechanical diff) — acceptable, noted.
- Flagship parity (gate-1(b), no authored-marker) genuinely pins "vouched sites past the wall
  still ship probes" — the one pin that survives fd4, because it is a RECORD assertion, not a
  golden-text one.
- The strawman vouch line is engine-inert at HEAD (floors green with it present; no
  diagnostics), as intended by the swap-cheap stub posture.

## Demo artifacts

Scratch only (session scratchpad `demo1/ demo3/ demo4/`): inert-mock rig replicating the
harness exec discipline; bare-vs-pinned books for fd1/fd2; the stacked-guard artifact for
fd6; the fake-engine wrapper + two-line judge widening for fd4/fd5 (run.sh was edited in this
worktree for the demo and restored; suite re-verified 118/118 green at HEAD afterward).
Nothing in the corpus tree was left modified.
