# 23G — the round-23 pin-repair pass (executing the 23F adjudication + rulings)

AI-authored (Opus-class repair agent, isolated worktree, 2026-07-02). Executes the repair
dispatch at the foot of `23F` under the human's h1–h5 rulings. Never-vouch applies: this note
and the changes it records are process-evidence, not proof — the guard tier is still unbuilt, so
every new pin asserts a DESIRED behaviour that HEAD cannot yet satisfy. Confidence marked
+SURE / ~SUSPECT / -GUESS / --WONDER throughout. Inputs, precedence order: `23F` (adjudication +
rulings addendum) · `23A` (pin register, conventions) · `23B`/`23C` (the review pair) ·
`spike/CLAUDE.md` round-23 rulings block.

## §0 Tally (before → after), all at HEAD

- e2e round-trips: `118 → 123` (5 new cases). xfail: `6 → 9` (+var-namespace, +nounset,
  +redirect). XPASS `0 → 0`; red `0 → 0`. +SURE (re-run clean, `DORC_E2E_QUIET` both ways).
- Gate set green: `cargo fmt --check` · `clippy -D warnings` · `cargo deny check licenses bans
  sources` (bans/licenses/sources ok) · `typos`. No Rust source touched (see jc-rust-diff).
- Every new XFAIL lens-lifted to its DESIGNED failure (below); the flagship re-lens-lifted to
  confirm the harness edits left its `ap-2-exec + gate-1 parity` signature unchanged.

## §1 The five new cases (23F task-1) — argument + HEAD failure-mode per pin

Mechanism-neutrality (ruling h3) governs all guard-shaped goldens: the `expected.out` guard
forms are ILLUSTRATIVE (subshell-wrap shown), the load-bearing pins are `expected.ran`
(behaviour) + `head-expected.ran` (two-sided) + the gates. jc-golden-mechanism.

### var-namespace-isolated (XFAIL) — 23C-fd1, the material variable-capture find
Book: `pkg=vim; hork wombat; apt-get install -y curl; apt-get install -y "$pkg"`. The corpus
predict body assigns `pkg`/`verb` bare; POSIX functions run in the caller namespace, so a
whole-body-ship guard on the converged-past-wall curl site clobbers the book's `pkg` to "curl"
⇒ the final line installs curl (which the guard just suppressed) and VIM NEVER INSTALLS. Pin
(behaviour): `expected.ran`'s final `apt-get install -y vim`. Both isolation mechanisms
(subshell-wrap OR predict-body `local`) produce the SAME ran-set here (the check runs fine either
way; both isolate the write) ⇒ genuinely mechanism-neutral. HEAD failure (lens-lifted): ap-2-exec
ran-mismatch (HEAD runs curl bare) + gate-1 parity (past-wall establish-probe doesn't ship).

### nounset-book-survives (XFAIL) — 23C-fd2, the set-u crash
Book: `set -u; hork wombat; apt-get install -y curl; apt-get install -y vim`. The check's
unconditional `[ "$2" = "" ]` read is fatal under set-u on the single-operand curl invocation;
a naive whole-body guard dies rc 2 at the guard, under-executing curl+vim. Verified empirically
(scratch, inert mocks): subshell-wrapped guard → the crash is contained in the subshell → `||`
falls through → curl BARE → vim runs → exit 0; naive whole-body guard → dies after only `hork`,
rc 2. So `expected.ran = hork, apt-get install -y curl, apt-get install -y vim` (== the bare book
== `head-expected.ran`). HEAD failure (lens-lifted): gate-1 parity ONLY (exec_check is HEAD-true —
the bare book already survives set-u). The CRASH DISASTER is caught NOT by exec_check but by the
two-sided `head-expected.ran` pin (a naive-crash regression drifts hork-only+rc2 ⇒ RED). This is a
deliberately different xfail-driver from var-namespace — jc-nounset-desired below.

### cross-oracle-vouch-scoped (floor) — 23C-fd9
Book: `apt-get install -y nginx` (package oracle A VOUCHED, converged before the wall ⇒ elides);
`hork wombat`; `systemctl enable foo` (service oracle B UNVOUCHED, past the wall ⇒ runs bare).
Pin: B's site RUNS; A's vouch is inadmissible in B's reasoning (rul-guard-license — the vouch
marks a path through THIS oracle's predict-body; provider-set membership is not a license). Verified
a wrong B-guard adds `systemctl is-enabled -- foo` to the run-set (or suppresses the enable) ⇒
exec_check reds. Clean green floor: parity holds on `{site 0}` alone (the past-wall service record
never ships at HEAD), gate-6 active, no marker. jc-cross-oracle-verdict.

### canttell-plan-runs (floor) — 23C-fd8, ruling h1 (converged-only mint)
Book: nginx (before wall, elides); hork; `apt-get install -y curl` (vouched past wall, PLAN-TIME
verdict CANT-TELL). Pin: curl RUNS bare — a cant-tell mint would guard a line the approved plan
showed as RUN (front-load breach) even though runtime-safe-ish. `PROBE_RESULTS=authored` (the
past-wall establish-probe can't reproduce a plan-time cant-tell — the drift-trio pattern); gate-1
parity + gate-6 off, exec_check carries the pin. Stable across promotion (cant-tell never mints).

### redirect-line-runs (XFAIL) — 23C-fd10, ruling h4 (refuse-home)
Book: `hork wombat; apt-get install -y nginx >>log`. A vouched converged-past-wall site whose
non-devnull redirect a guard's pass-direction would suppress (the append never happens on a
converged pass). Mirrors `guard23-heredoc-refuses-loudly`'s split EXACTLY: the RUN half is a floor
riding along (`expected.ran` = HEAD's + `head-expected.ran`), the loud-refusal-disclosure half is
the xfail. Confirmed HEAD emits NO refusal surface for the redirect line (⇒ the disclosure half is
XFAIL-shaped, per the task's "iff no disclosure surface exists at HEAD"). gate-2 accepts the bare
relative `>>log` (lands in the sandbox). HEAD failure (lens-lifted): gate-7 `refus` + gate-1 parity.
`expected-diagnostics` pre-declares `guard` (inert at HEAD; covers an error-severity refusal at
promotion).

## §2 Harness repairs (23F tasks 2/4/6/8), all in `spike/e2e/run.sh`

- **two-sided xfails (task 2, 23B-fd1/23C-fd4).** New optional `head-expected.ran` marker; while
  an XFAIL is present, `head_ran_check` asserts the case's CURRENT apply run-set still equals the
  pinned HEAD signature — consulted ONLY while case_ok=0 (jc-head-ran-gating). Markers on all 8
  mocks-bearing xfails (flagship + fall-through trio + heredoc + the 3 new). Verified it bites: a
  simulated disaster-drift on the flagship went RED with the designed message, then restored.
- **guard-shape grep-floor (task 6b, 23C-fd4).** `guard_shape_check`/`_violations` assert every
  guarded line (keyed on the `dorc: guard` disposition comment) has the shape
  `<check-invocation> || <original book bytes>`, the fall-through byte-identical (mod whitespace)
  to a book.sh line. INERT until guards appear (HEAD emits none — verified 0 matches); LOUD even
  under XFAIL (a malformed guard is a disaster regardless). `guard_shape_selftest` (runs at start,
  aborts on failure) drives the two 23C-fd4 shapes: gf-1 (engine-synthesized thin guard, no `||`)
  + gf-2 (`-y`-dropped fall-through) + gf-PASS. Discrimination exercised standalone.
- **dual_rail_selftest cf-5 (task 6a, 23C-fd5).** A `guard` disposition never licenses an
  UNRELATED apply-only line — passes the current (un-widened) judge trivially (`guard` isn't in the
  replace/omit filter), forward-locks the deferred gate-6 widening against over-broad whitelisting.
  jc-cf6-deferred.
- **why-attribution conjunction (task 4, 23C-fd13).** `scan_why` gains an opt-in ` && ` per-line
  conjunction (progressive `grep -F` filter): needles must co-occur in ONE why-line. why-attribution's
  `expected-why` → `guard && vouch && package`; book/golden/XFAIL comments updated in lockstep.
  Verified the fd13 three-separate-notes attack now MISSING; `exec-opaque-var-runs` (no ` && `) unaffected.
- **diff-before-bless rule (task 8).** Written into the run.sh header (PROMOTION DISCIPLINE block):
  diff engine stdout vs the hand-authored `expected.out` and inspect BEFORE deleting an XFAIL or
  running BLESS — never bless-first (the XPASS/bless path is blind to golden text).

## §3 Smaller repairs (23F tasks 3/5/7)

- **dpkg-query tripwire shims (task 3, conv-2/23C-fd6).** exit-1 (nginx-absent, world-consistent)
  `dpkg-query` mocks added to `already-hand-guarded-runs` + `top-argv-runs`. Inert at HEAD (handguard
  probe uses `dpkg -s`; top-argv probe is empty). Verified a wrong provider-keyed guard on top-argv
  now logs `dpkg-query -W nginx` into the run-set ⇒ reds, instead of 127-ing invisibly. (Committed as
  git mode 100644, matching every sibling dpkg-query shim — the MSYS quirk where dash execs by shebang.)
- **cmdsub exec upgrade (task 5, 23B-fd6).** `cmdsub-position-runs` gained a `mocks/` dir (apt-get +
  shared `.log`) + `expected.ran`; the cmdsub body now executes under the shim and logs, so a wrong
  guard/elision reds the exec gate. `expected.out` unchanged (engine stdout identical).
- **conflict artifact (task 7, conv-3).** `git rm` the tracked
  `guard23-ternary-flagship/head-output.sync-conflict-…-PHNHRER.txt` (a scratch HEAD-output capture;
  its content cross-validated the flagship head-expected.ran). No other conflicted twins in the tree.

## §4 Judgment calls flagged (jc-*) — made-for-now, NOT settled, cheap to reverse

- **jc-nounset-desired** (the sharpest): nounset's `expected.ran` pins the BARE-survival ran-set
  (curl runs bare via subshell fall-through under set-u), mechanism-neutral across subshell-wrap and
  refuse-to-guard. But a SANITARY-oracle mechanism (`${2:-}`, which h3 "leans into") produces a
  DIFFERENT, WORKING ran-set (curl suppressed, a `dpkg-query -W curl` line appears) — because under
  set-u the subshell CRASHES before reaching the check's `dpkg-query`, while the hygiene form doesn't.
  So nounset's ran-set is mechanism-SENSITIVE where var-namespace's is not. I chose bare-survival (the
  "complete exactly as the bare book" reading, catchable by the two-sided pin); if the builder picks
  hygiene, `expected.ran` churns at promotion — visibly — surfacing that subshell-wrap under set-u is a
  degenerate guard (no attention saved). ~SUSPECT bare-survival is the right floor; the crosscheck/human
  should rule whether the desired set-u end-state is "survive-only" or "survive-AND-work".
- **jc-rust-diff**: the Rust `crates/hostsim/src/differential.rs` judge (cm-1 local approximation) knows
  only `run`/`replace`/`omit` and its generator emits NO guards — so it will need the SAME guard-disposition
  widening as gate-6 when the tier lands, plus a planted-guard-confound analogue beside its existing
  `judge_screams_on_planted_under_execute` battery. I did NOT touch it (premature: no guard emission to test
  against, no guard-license semantics to confound). The mission's "Rust dual-rail selftest" parenthetical is
  read as permission, not obligation; the concrete 23C-fd4/fd5 confounds live in run.sh (where XPASS/bless is).
  Flagged for the build round. +SURE it needs the widening; -GUESS the shape.
- **jc-cf6-deferred**: cf-6 (a guard disposition licenses its OWN suppressed mutator ⇒ must NOT scream) is
  deferred to the gate-6 widening — the current judge has no guard-license semantics, so a non-scream
  assertion would fire the FATAL selftest now. Breadcrumb left in `dual_rail_selftest`.
- **jc-head-ran-gating**: the two-sided pin is consulted only while case_ok=0. A drift with case_ok=1 is the
  designed behaviour LANDING (⇒ XPASS), not a disaster — so gating on case_ok=0 preserves XPASS on a legit
  promotion. This deviates from the literal "assert current == head-expected.ran always" (which would red a
  correct promotion). ~SUSPECT correct; it is the only reading that keeps XPASS meaningful.
- **jc-guard-shape-key**: the shape floor keys on the `dorc: guard` disposition comment (jc-guard-comment,
  whose wording 23A calls the builder's). If the builder changes that marker's wording, the floor goes inert
  until re-keyed — a jc-why-wording-style visible promotion adjustment. Its `||`-split assumes the
  check-invocation carries no literal ` || ` (true for every guardable site by construction).
- **jc-cross-oracle-verdict**: cross-oracle's service site ships no convergence record (past-wall establish
  doesn't ship at HEAD). The provider-membership bug is baited via vouch-EXISTENCE (A's vouch in the -o set),
  not a service probe-verdict; a "careful" bug that demanded a service verdict first wouldn't bite. Defensible
  — an UNVOUCHED oracle must short-circuit to run BEFORE any verdict question, so vouch-existence is the right
  bait — but noted as a coverage edge the crosscheck should weigh.
- **jc-canttell-authored**: canttell uses `PROBE_RESULTS=authored` (disables gate-1 parity + gate-6), so the
  cant-tell verdict isn't mock-reproduced; exec_check carries the pin. The alternative (ship a cant-tell mock
  + real parity) needs an rc-2 dpkg-query shim past a wall, which the past-wall-establish asymmetry defeats.

## §5 What strained (process notes for the next derivation)

- **nounset needed empirical grounding, not reasoning.** The subshell-under-set-u fall-through behaviour
  (crash contained in the subshell ⇒ `||` runs ⇒ curl bare, book survives) is easy to get wrong on paper; I
  ran it under the inert mocks before authoring. The `set -u` fatality exits the SUBSHELL, not the parent —
  that is the whole mechanism, and it flips the desired ran-set. Author set-u/errexit cases against dash, always.
- **expected.ran vs expected.out division held.** Every new xfail's load-bearing pin is expected.ran/EXIT
  (behavioural, mechanism-invariant); the goldens carry mechanism-illustrative bytes that WILL churn at
  promotion (jc-golden-mechanism). The two-sided head-expected.ran pin is what gives the latent composition
  hazards (var-namespace clobber, nounset crash) teeth DURING the build window — without it a naive-guard
  regression is a silent `xfail`, exactly 23B-fd1's camouflage.
- **lens-lift verification is mv-fragile.** Backgrounded `mv XFAIL /tmp; run; mv back` chains can strand the
  XFAIL file if the run is slow enough to background mid-chain. Switched to `cp .bak; rm; run; cp back` in the
  foreground (idempotent restore). No case tree was left modified — re-verified 123 green + XFAIL files intact.
- **the guard-shape floor and two-sided pin are BOTH inert at HEAD by construction** (no guards emitted; no
  ran-set drift), so they cost nothing now and only bite in the build window — the intended shape (23C-fd4's
  teeth live on the promotion path, not the HEAD suite).
