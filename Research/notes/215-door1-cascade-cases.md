# 215 — door-1 (guard folds): cascade verification + reach. Strain + decisions.

> Round-21 w-4, builder note (slug 215 reserved per 211 §4). Charter: 211 §1 arch-3(b)
> + `plans/20V` §4 door-1. Append-only; confidence-marked (+SURE/~SUSPECT/-GUESS/--WONDER).
> NO engine edits — door-1's cascade FOLDS AT BASE (the headline finding, §2). Deliverable:
> 7 corpus cases (`door1-*`), all hand-derived; the gate set green ×2 (suite 75→82); zero churn
> to existing goldens. HEAD before task: `92162f1` (post arch-1 leaf-exact render `140c303`).

## §0 Headline (what shipped, and what did NOT)

+SURE (traced + empirically confirmed via `--debug-argv` and exec-under-mocks, gate-set green
×2): **door-1's cascade is fully general at the BASE engine — no extension was needed, the
extend-reach budget is unspent.** The charter's payoff semantic
(`grep -qx … || { sed -i …; systemctl restart sshd; }` — guard holds ⇒ the whole guarded
BLOCK is dead control-flow ⇒ fold; the edit AND the restart elide as unreachable, needing no
rc-provenance of their own) works for EVERY chartered shape AND every adversarial shape I
probed, because the apply fold's `kill_rec` (`plan/src/fold.rs` ~350) already recurses through
**every** compound construct: `Group`/`Subshell` (the brace-group and `( )` RHS), `AndOr`
(nested `&&`/`||`), `If`/`Case`, loops, pipelines. When `eval_and_or` proves a `||`/`&&`
branch dead (a KNOWN guard rc short-circuits past it), it calls `kill(right, guard)`, and
`kill_rec` marks **every** `Simple` leaf beneath `right` dead, controlled by the guard. The
leaf-exact render (arch-1, `140c303`/note 214) then substitutes each dead leaf's byte-span to
`:` (omit-safety: the controller guard is itself neutralised to `true`/`false`), and the guard
to its probe-sourced stand-in. The Ansible handler/notify semantic falls out of plain
control-flow analysis, exactly as `plans/20V` §4 door-1 stated.

**No `plan`/`fold` edit. No render edit. No new test in the engine** (the fold already carries
unit pins for the `||`/`&&`/`if` deadness — `fold.rs` tests `oror_known_success_kills_right_operand`
etc.; the new value is the END-TO-END corpus pins that the CASCADE through a multi-statement
group renders dash-clean with the right run-set). The deliverable is the 7 cases + this note.

## §1 The 7 cases (all `door1-*`, all hand-derived; file:disposition)

Each case's `book.sh` carries a verbose case-comment (corpus house style — the existing
`fold-oror-guard-omits` / `exec-query-guard-composition` set the precedent) stating the
semantic, the fold mechanism, and the run-delta distinction. The guard is `dpkg -s <pkg>` (the
`pkgstate` Query, DESIGN's canonical idiom) — chosen over `command -v` because `dpkg` is an
EXTERNAL command, so its probe is mock-reproducible (full gate-1 parity, NO
`PROBE_RESULTS=authored` opt-out needed; contrast `fold-oror-guard-omits`, whose `command -v`
builtin forces the opt-out). The block commands (`sed`/`systemctl`/`rm`/`cp`) are un-oracled —
correct, because in the converged case they are DEAD (folded, no oracle consulted), and in the
diverged case they just RUN as opaque mutators.

- **door1-cascade-block-elides** (PAYOFF, `||`, guard holds rc 0): mocks+exec. `set -e; dpkg -s
  nginx … || { sed …; systemctl restart sshd; }`, nginx installed ⇒ guard holds ⇒ block DEAD.
  Renders `true || { :; :; }`, run-set EMPTY, dash-`n` clean. argv: `1 replace`, `2 omit` (sed),
  `3 omit` (systemctl). The headline: the restart elides as UNREACHABLE, no rc-provenance of its
  own.
- **door1-cascade-diverged-runs** (POLE, `||`, guard absent rc 1): mocks+exec. SAME book, nginx
  absent ⇒ guard fails ⇒ `||` fires ⇒ block LIVE. Renders `false || { sed …; systemctl …; }`,
  run-set `[sed, systemctl]` (book order). Proves deadness is PROBE-KEYED, never structural: the
  identical shape that folded whole now runs whole. The guard substitutes to `false` (its
  probe-sourced rc 1).
- **door1-cascade-multistatement** (NESTED REGION, `||`, guard holds): mocks+exec. The block has
  3+ statements incl. an inner `if [ -f … ]; then cp …; fi`. The cascade kills the WHOLE nested
  region — sed, BOTH inner-`if` leaves (its `[` test AND its `cp` body), systemctl. Renders
  `true || { :; if :; then :; fi; :; }`, dash-`n` clean (the inner `if` arms become `:`, NEVER
  empty — the span-edit substitutes each leaf, never deletes a clause). Run-set EMPTY. This is
  the `kill_rec` Group→If→Simple recursion proven end-to-end.
- **door1-and-form** (`&&` DUAL, guard absent rc 1 ⇒ block dead): **ANALYSIS-ONLY** (no mocks —
  see strain-and-exit §3). `set -e; dpkg -s nginx … && { systemctl stop nginx; rm …; }`, nginx
  absent ⇒ guard FAILS ⇒ `&&` short-circuits ⇒ block DEAD. Renders `false && { :; :; }`. Pins the
  charter's EXTEND-REACH question for `&&`: it FOLDS AT BASE (the `eval_and_or` `(And, Some(s))`
  arm handles it symmetrically to `||`), no extension. The `-n` + exact golden structurally prove
  the empty run-set (every leaf is `:`/`false`).
- **door1-and-form-runs** (`&&` POLE, guard holds rc 0 ⇒ block runs): mocks+exec. `set -e; dpkg -s
  nginx … && { systemctl reload nginx; }`, nginx installed ⇒ guard holds ⇒ `&&` proceeds ⇒ block
  LIVE. Renders `true && { systemctl reload nginx; }`, run-set `[systemctl reload nginx]`, exits 0
  (so the exec gate accepts it). The `&&` companion-pole (dual of door1-cascade-diverged-runs).
- **door1-guard-below-mutators-invalid** (NEGATIVE pole, st-3): mocks+exec. `set -e; apt-get
  install -y curl; dpkg -s nginx … || { sed …; systemctl …; }`. The upstream `apt-get install
  curl` is an oracled WRITE ⇒ the downstream `dpkg -s nginx` Query fails rule-query-validity (the
  pristine-prefix rule, st-3 / 20A §4 / 205 §2: ANY upstream write makes the Query's resting rc
  stale ⇒ `valid: false`) ⇒ the fold can NEVER prove the `||` branch dead ⇒ EVERYTHING runs (the
  install, the guard verbatim, the block). Renders fully verbatim. Run-set `[apt-get install curl,
  dpkg -s nginx, sed, systemctl]` (nginx absent at the mock host ⇒ the runtime guard fails ⇒ block
  runs too). Two probe sites: site 1 the install's ESTABLISH probe (`dpkg-query -W`, rc firewalled
  ⇒ probe-results omits rc), site 2 the Query guard (`dpkg -s`, rc carried but NOT fold-used —
  invalid).
- **door1-door3-inner-elides** (door-1 × door-3 COMPOSITION, hunt-C made concrete): mocks+exec.
  `set -e; dpkg -s nginx … || { apt-get install -y curl || true; systemctl restart sshd; }`,
  nginx ABSENT (outer guard fails ⇒ block LIVE) × curl CONVERGED (inner `apt-get install || true`
  is a door-3 site that mints). The OUTER door-1 keeps the block live (guard diverged); the INNER
  door-3 (StatusInvariant) elides the converged install INSIDE the live block. Renders `false ||
  { true || true; systemctl restart sshd; }`, run-set `[systemctl restart sshd]` (the inner
  install elided to `true`, `true` is a builtin). Proves the two doors compose without
  interference (door-1 deadness is per-leaf via the fold; door-3 StatusInvariant is per-`||`-left
  via cfg — orthogonal). Closes tc-door1-door3-composition.

## §2 Folded-at-base vs extended (the charter's required accounting)

**ALL chartered shapes folded at base; ZERO extensions added.** Per-shape:

| shape | folds at base? | mechanism |
|---|---|---|
| `\|\| { mutator; mutator; }` (group RHS) | +SURE YES | `kill_rec` `Group{body}` arm recurses the brace group |
| `\|\| ( mutator; mutator; )` (subshell RHS) | +SURE YES | `kill_rec` `Subshell{body}` arm (probed: `true \|\| ( :; : )`) |
| multi-statement block w/ inner `if` | +SURE YES | `kill_rec` Group→If→Simple recursion (door1-cascade-multistatement) |
| `&&` direction (`query && { … }`) | +SURE YES | `eval_and_or` `(And, Some(s)) => Some(s)` — symmetric to `\|\|` (door1-and-form) |
| compound guard (`A && B \|\| { … }`) | +SURE YES | `eval_and_or` evaluates the `(A && B)` rc, then folds the outer `\|\|` (probed: `true && true \|\| { :; }`) |
| block w/ side-exit (`\|\| { …; exit 1; }`) | +SURE YES | `exit 1` is a Simple leaf in the dead block ⇒ omitted; the diverged pole RUNS it (§5 hunt-A) |
| block containing a Query of another kind | +SURE YES | the inner Query's leaf is dead ⇒ omitted; inner `\|\|` collapses to `: \|\| :` (§5 hunt-B) |
| block whose inner cmd is `\|\| true` (door-3) | +SURE YES | door-3's `\|\| true` and its left both dead ⇒ `: \|\| :` (§5 hunt-C) |

The extend-reach budget (~150 lines, fold/plan layer) is UNSPENT. -GUESS the reason this came in
"free" is that the fold was DESIGNED right in `19B` build-1 / task-L1: `kill_rec` was always a
total recursion over `NodeKind`, so the brace-group/subshell/nested-if arms existed before door-1
was a named door — door-1 is the OBSERVATION that this recursion already IS the Ansible-handler
cascade. The leaf-exact render (arch-1) was the missing half (the line-render could not express a
`:`-per-dead-leaf inside a group — note 213 §1 res-1 is the door-3 instance of exactly that wall);
once arch-1 landed, the cascade renders with no further work.

## §3 What strained

- **strain-and-exit (the only real one) — a `set -e … && {dead}` book exits NON-ZERO, and the
  exec gate has no opt-out.** +SURE, dash-confirmed: `set -e; false && { … }` does NOT abort (the
  failed left of `&&` is errexit-exempt, and a short-circuited AND-OR list's non-zero result does
  not trigger errexit — `dash -c 'set -e; false && { echo x; }; echo reached'` prints `reached`),
  but the SCRIPT'S final rc is 1 (the AND-OR list's rc = the failed left's). The BARE book exits 1
  too (I confirmed under mocks) — so the artifact is FAITHFUL. But `exec_check` (run.sh ~140)
  treats ANY non-zero exit as `[ap-2-exec: rendered apply errored when run]` — there is no
  expected-nonzero-exit opt-out (the suite's books all happen to end rc 0, via `|| true` /
  `true ||` / a trailing converged mutator). So a faithful `false && { :; :; }` artifact
  FALSE-fails the exec gate. **Disposition**: made door1-and-form ANALYSIS-ONLY (no mocks/) — the
  `-n` + exact golden structurally prove the empty run-set (no runnable command survives), and the
  `&&` RUN pole's run-set IS exec-asserted by door1-and-form-runs (its live block ends in a
  logging command, exits 0). This is honest (the case-comment says so) and avoids touching run.sh
  (out of scope — the harness). ~SUSPECT a future harness refinement could add an
  `EXIT_RC=<n>`-style marker (mirroring `RAN_ORDER=lax` / `PROBE_RESULTS=authored`) so a
  legitimately-non-zero artifact gets the exec gate; flagged as a **tc-exec-nonzero-exit** for the
  orchestrator. Not built (charter: confine to fold/plan; do NOT extend render/harness).
- **strain-provenance-string (cosmetic) — the diverged/`false`-substituted guard's comment reads
  "already converged / dead branch".** The fixed provenance string `provenance_comment` appends
  (note 214 §1) is `# dorc: elided [<orig>] (already converged / dead branch)`. On a guard
  substituted to `false` (door1-cascade-diverged-runs, door1-and-form), "already converged" is
  imprecise (the guard is a known-rc Query substitution, NOT a converged elision). +SURE harmless
  (it's inside a `#` comment, dash-`n`-inert; identical imprecision already ships on
  `headline-guarded-realistic`'s `false`-substituted guard) and PRE-EXISTING (note 214's shared
  render, not introduced here). Flagged as **tc-provenance-string-coverage** in case disclosure
  fidelity is later weighted; I did NOT touch the render (zero churn).

## §4 The run-delta distinction (the charter's MANDATORY statement)

+SURE, stated in every cascade case's comment and restated here for the durable record: **door-1
is NOT the run-delta / notify-handler class (`TODO.md` R2-CHANGEDELTA).** The two look identical
in sh (`mutator || { …; service-restart; }` ≈ "restart after a config change"), but the licensing
is OPPOSITE:

- **R2-CHANGEDELTA** ("restart-iff-changed"): `cp config … && changed=1; [ "$changed" ] &&
  systemctl restart` — the restart is gated on a CONSUMED OBSERVABLE (`changed=1`) that an
  upstream effect produced. It is **un-probeable** (no state-probe can vouch for a future run's
  delta) and **never elidable via a state-probe** — eliding the config-write would REMOVE its
  `changed=1` side-effect (a `q1-interproc` hazard, `plan/CLAUDE.md` R2-CHANGEDELTA). The
  cross-kind `file:`→`service:` edge must NEVER be synthesized.
- **door-1** ("guard folds its block"): `query-guard || { …; service-restart; }` — the restart
  elides because **the BRANCH IS DEAD**, proved by the guard's probe-observed rc, NOT because any
  state-probe vouched FOR the restart. The restart has NO rc-provenance of its own and needs none
  — it is unreachable. The **diverged pole makes this visible**: when the guard fails
  (door1-cascade-diverged-runs / door1-and-form-runs), the WHOLE block runs, restart INCLUDED. The
  restart's disposition is 100% a function of the guard's branch-deadness, 0% a function of any
  claim about the restart itself.

The litmus: in door-1, swap the guard's probed rc and the restart's disposition flips entirely
(elide↔run); in R2-CHANGEDELTA, no state-probe rc can ever license eliding the restart, because
the gating observable is a run-delta, not a resting state. door-1 elides a DEAD restart;
R2-CHANGEDELTA must run a LIVE-but-conditional restart. **They must never be conflated** — a fold
that treated R2-CHANGEDELTA's `[ "$changed" ] && restart` as a foldable guard would wrong-elide
(the `changed` flag is ⊤ at apply time unless the write ran, and the write may have elided). door-1
is safe precisely because the guard is a probe-able STATE Query (pristine-prefix-validated, st-3),
not a run-delta flag.

## §5 Adversarial hunt-list (WRITE-IT-YOURSELF — ranked; a hostile crosscheck must EXCEED this)

Hostile-identity framing: "a builder I distrust claims door-1's cascade 'just works at base' and
shipped only e2e fixtures, no engine change — the lazy path. Find a guarded-block shape where the
cascade WRONG-ELIDES (a leaf that survives in some world is killed) or produces a BROKEN artifact
(dash-`n` dirty / wrong run-set), or where the 'folds at base' claim hides a gap the fixtures
dodged." Construct every probe against dash (the semantic oracle), not the engine's self-report.

- **hunt-A (HIGHEST) — block-with-side-exit, both poles.** `query || { …; exit 1; }` and `… ||
  { …; return; }`. I probed the converged pole: the `exit 1` is killed (`argv 3 omit`), rendering
  `true || { :; :; }` — CORRECT, because the block (incl. the exit) is genuinely unreachable when
  the guard holds, exactly when the `exit` shouldn't fire. The diverged pole RUNS the `exit 1`
  (`argv 3 run`, dash-confirmed). +SURE this is right, but ATTACK the COMPOSITION: a side-exit
  block where the exit is CONDITIONAL on something inside the block (`|| { cmd; [ -f x ] && exit
  1; }`) — does killing the inner `&&`'s leaves stay sound? And `return` OUTSIDE a function (a
  syntax/semantic error in dash — does the fixture/engine handle it, or does it ⊤-reject?). And the
  disaster shape: a guard whose probed rc is ⊤ (NOT known) + a side-exit block — the block must
  stay LIVE (kFAIL-perform), the `exit 1` must NOT be killed. ~SUSPECT the kill is correctly gated
  on KNOWN guard rc (fold.rs `is_success() => None ⇒ no kill`), but verify a ⊤-guard side-exit runs
  the whole block.
- **hunt-B — block containing a Query of ANOTHER kind.** I probed `|| { command -v curl ||
  apt-get install; systemctl; }` (guard holds): the inner `command -v curl` AND the install both
  omit, rendering `: || :` for the inner `||` — dash-`n` clean. ATTACK: an inner Query that is
  ITSELF foldable in isolation (a `dpkg -s curl || …` with curl's OWN probe site) — when the OUTER
  guard kills the whole block, does the inner Query's probe site still get EMITTED (a probe for a
  command that will never run)? The probe is phase-separate (it probes the BARE book's sites, not
  the apply's), so +SURE the inner site is probed (the probe doesn't know the outer fold will kill
  it) — but confirm that's HARMLESS (a wasted read, not a wrong elision). And the dual: does the
  inner Query's site-keyed result accidentally feed the OUTER fold? It must not — they are distinct
  sites. ~SUSPECT clean (site-keyed, inv-site-keyed-results), but a hostile pass should trace the
  site IDs.
- **hunt-C — block whose inner command is `|| true` (door-3 composition). NOW PINNED
  (door1-door3-inner-elides).** I probed `|| { apt-get install || true; systemctl; }` both ways: in
  the CONVERGED outer case the inner `|| true` is DEAD (omitted) so door-3 never fires (`: || :`) —
  fine. In the DIVERGED outer case (guard fails ⇒ block LIVE), the inner `apt-get install || true`
  is a LIVE door-3 site, and +SURE (now exec-gated): with curl CONVERGED it door-3-elides correctly
  (`false || { true || true; systemctl …; }`, run-set `[systemctl]`), proving the marks are
  INDEPENDENT (door-1's deadness per-leaf via fold; door-3's StatusInvariant per-`||`-left via cfg;
  when the block is live, the inner door-3 leaf is reachable and its mark applies). The shipped case
  pins the d×c cell (the interesting one — outer-dead × anything is trivially all-omitted). A
  crosscheck should still EXCEED this with the d×d cell (outer live + inner install DIVERGED ⇒ inner
  install RUNS) and a `&& { … || true }` mix; +SURE-via-mechanism but only the d×c cell is
  exec-pinned. **tc-door1-door3-composition closed for d×c; d×d left for crosscheck.**
- **hunt-D — the pristine-prefix boundary (st-3), re-attacked.** door1-guard-below-mutators-invalid
  pins ONE write (apt-get install) upstream. ATTACK: (i) an OPAQUE (un-oracled) command upstream
  (`ufw allow … ; dpkg -s nginx || { … }`) — `query_after_opaque_is_invalid` (effect.rs test) says
  Opaque⇒Top⇒invalid, so the block must run; verify end-to-end. (ii) A PURE builtin upstream (`:`
  or `set -e` only) — must STAY valid (the block folds); door1-cascade-* all have `set -e` upstream
  and DO fold, so +SURE `set -e` is pristine, but a hostile pass should add a `cd`/`export`
  upstream (do those gen a write? `cd` writes PWD/OLDPWD per note 20T — does that invalidate a
  downstream Query? ~SUSPECT it might, which would be a SURPRISE; worth a probe). (iii) An upstream
  QUERY (not a write) — `query_after_query_stays_valid_st3` says a guard-stack stays valid; a
  `dpkg -s A || { … }; dpkg -s B || { … }` double-cascade should fold BOTH — not pinned, worth a
  case.
- **hunt-E — render arithmetic on the multi-statement / multi-line group.** door1-cascade-
  multistatement renders a 4-line block with a per-line provenance comment. ATTACK the span-edit
  splice (note 214 hunt-1/hunt-7 territory) on a group: a block with TWO leaves SHARING a line
  (`|| { sed …; systemctl … ; }` all on one line is the block-elides case — clean; but `do sed
  …; rm …` style siblings inside a multi-line group), a leaf whose span includes a trailing
  redirect inside the group, a heredoc-bearing leaf INSIDE a dead block (the d-6 refusal — does a
  REFUSED heredoc leaf inside a dead block leave the block half-edited / dash-`n` dirty?). +SURE the
  single-line and 4-line forms are clean (pinned); ~SUSPECT a heredoc-in-dead-block is the corner
  (the leaf refuses the `:` edit ⇒ runs verbatim ⇒ but it's in a dead block whose guard is `true`
  ⇒ never reached ⇒ safe, just not elided — confirm it's dash-`n` clean and the run-set is still
  empty because the guard short-circuits).
- **hunt-F (lower) — the `&&`/`||` exit-rc faithfulness across poles.** strain-and-exit found
  `false && {dead}` exits 1. ATTACK every pole's exit rc vs dash: `true || {dead}` (exits 0 ✓),
  `false || {live}` (exits the block's last rc), `true && {live}` (exits the block's last rc),
  `false && {dead}` (exits 1 — the analysis-only one). Confirm the engine's artifact exit rc MATCHES
  the bare book's under dash for ALL FOUR, so the analysis-only carve-out is the ONLY non-zero-exit
  case (if another pole also exits non-zero, it too needs the carve-out or a trailing no-op).

## §6 What surprised me

- ~SUSPECT-turned-+SURE: I expected to spend the extend-reach budget on the brace-group or `&&`
  form (the charter framed them as likely-to-fail: "if the `&&` form fails to cascade, that is your
  extend-reach work"). Both folded at base on the FIRST probe. The fold's `kill_rec` total-recursion
  (a `19B`/task-L1 design choice) had already solved the cascade; door-1 was waiting only on
  arch-1's render, which landed in w-2 (`140c303`). The round's wave-ordering (door-1 cases in w-4,
  AFTER arch-1 in w-2) was exactly right — had this run before arch-1, it would have hit note 213's
  render wall (the door-3 instance of the same `:`-in-a-group problem).
- +SURE: the SHARP demonstration of st-3 is the NO-`set -e` variant (probed, not shipped):
  `apt-get install curl` (converged, ELIDES to `true`) + `dpkg -s nginx || { … }` — the upstream
  mutator ELIDES, yet STILL invalidates the downstream guard (its STATIC presence as a write is what
  poisons the pristine-prefix, independent of its apply-time disposition). I shipped the `set -e`
  variant instead (cleaner "everything runs" story — the install runs via fork-mutator-rc), but the
  no-errexit variant is the more pointed proof that invalidation is a STATIC property. Noted for a
  possible future case.
- -GUESS: the exec gate's no-non-zero-exit-opt-out (strain-and-exit) is a latent that the corpus
  never hit because its books are all rc-0-terminating by happenstance. door-1's `&&`-dead pole is
  the first natural rc-non-zero artifact. A `RAN_ORDER=lax`-style `EXIT_RC=` marker would close it
  cleanly; out of scope this task.

## §7 Flags (tc-*/doc-deltas — surfaced, not resolved)

- **tc-exec-nonzero-exit (strain-and-exit):** the exec harness FALSE-fails a faithful artifact that
  legitimately exits non-zero (`set -e; guard && {dead}`). door1-and-form is analysis-only as the
  workaround. A future `EXIT_RC=<n>` marker (mirroring the existing opt-out markers) would let such
  a case get the exec gate. Orchestrator/human call (touches run.sh — out of my scope).
- **tc-door1-door3-composition (hunt-C) — PARTLY CLOSED:** the d×c cell (outer-diverged × inner-
  converged-`|| true`) is now exec-pinned by door1-door3-inner-elides (the inner door-3 elides
  inside a live door-1 block — `false || { true || true; systemctl …; }`). The mechanisms are
  independent (fold deadness vs cfg StatusInvariant), confirmed end-to-end. The d×d cell (inner
  install diverged ⇒ runs) is left for a crosscheck to exceed.
- **tc-provenance-string-coverage (strain-provenance-string):** the `false`-substituted guard's
  comment says "already converged" — imprecise for a known-rc Query substitution. Pre-existing
  (shared render), cosmetic, dash-inert. Flagged for disclosure-fidelity weighting; not touched.
- **NOT a tc, for the orchestrator's eye:** 7 cases added, the suite goes 75→82 (run.sh count),
  ZERO churn to existing goldens or source (`git status`: 7 new `door1-*/` dirs, nothing modified).
  No BLESS used — every golden generated from the verified binary's stdout AND independently
  hand-traced (the fold dispositions via `--debug-argv`, the render `:`/stand-in rules, dash-`n`
  cleanliness, the run-set under mocks); the binary output MATCHED the trace for every case.

## §8 Confidence summary

- +SURE: door-1's cascade folds at base for all chartered/probed shapes; no engine edit; gate set
  green ×2 (fmt/clippy-`D`/test workspace all-pass/e2e 82 ×2/typos); zero churn to existing goldens.
- +SURE: the run-delta distinction (§4) — door-1 elides a DEAD restart (probe-keyed branch
  deadness), NOT R2-CHANGEDELTA's LIVE-conditional restart; the diverged pole proves it.
- +SURE: the `&&` dual folds symmetrically (`eval_and_or` `(And, Some(s))`); the extend-reach
  budget is unspent.
- +SURE (now pinned): the door-1 × door-3 composition's d×c cell (door1-door3-inner-elides) — the
  inner door-3 elides inside a LIVE door-1 block; the marks are independent. The d×d cell is left
  for a crosscheck.
- ~SUSPECT: the exec-gate non-zero-exit limitation (strain-and-exit) is the only place a faithful
  door-1 artifact can't get the exec gate; the analysis-only workaround is honest but a marker would
  be cleaner.
