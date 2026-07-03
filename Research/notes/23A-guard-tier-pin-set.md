# 23A — the guard-tier pin-set (task #7's unblocked half; back-to-earth step 3)

AI-authored (single Fable-tier subagent, dispatched 2026-07-02 per `23Z` step-3; human-acked
prompt). Never-vouch applies: this note and the case-set are process-evidence, not proof — the
adversarial crosscheck over the pin-set ("find the pin that licenses a wrong-elision; find the
licensing hole no pin covers") is the next consumer, and this note is written to be attacked.
Confidence marked +SURE / ~SUSPECT / -GUESS / --WONDER throughout.

Binding inputs, in precedence order: `spike/CLAUDE.md` round-23 rulings block
(rul-ternary-verdict · rul-guard-license · rul-attention-honesty · rul-divergence-proceed; the
TOCTOU identified-cause clarifier; the inv-probe-sourced-values GuardInsert carve-out) · the
THREE MID-RUN HUMAN RULINGS relayed 2026-07-02 while this set was being derived (rc-consumer
split DEFERRED → narrowest posture only; "the command is the atomic unit" → no partial-member
pins; refuse-loudly RATIFIED for awkward homes) — now also recorded in `23Z` §"Task-4 rulings" ·
stamped `plans/233` + its end-annotation · `plans/239` (closure, GO) · `notes/218a` (door-4
mechanics prior art) · `notes/237` (crosscheck adjudication + post-adjudication corrections) ·
the `23Z` ORACLE GROUND-TRUTH block.

Deliverable state: **19 new cases** under `spike/e2e/cases/guard23-*` — 6 XFAIL (desired,
unbuilt) + 13 passing floors (true today; must never regress). Harness green at HEAD:
`118 round-trips, 6 xfail, 0 XPASS, 0 red`. Every xfail's failing-gates were individually
verified to be the DESIGNED failure (XFAIL lens lifted case-by-case; reasons listed per-pin
below), not an authoring accident.

## §0 Shape of the set, and the one sin

Wrong-elision is the sin the set is built around: a command the world still needed, silently
not run. In the ternary reshape that sin has three doors, and the set posts a pin at each:

- door-1: a guard minted where none was licensed (no vouch / wrong scope / consumed reader) —
  pins P-novouch, P-topargv, P-rundelta, P-rcreaders, P-stdout, P-cmdsub, P-multiop, P-inloop,
  P-background;
- door-2: a licensed guard that fails to FALL THROUGH (the guard's own under-execute) — pins
  X-drift, X-canttell, X-mutfail; plus X-flagship's pass-direction run-set;
- door-3: the license leaking beyond its fence (vouch entering the fact-plane; double-guarding;
  values minted from a guard) — pins P-pair-a/b, P-handguard, P-reingest, and the flagship's
  golden bytes.

Passing floors deliberately outnumber xfails: most of the guard tier's safety story is
"HEAD already refuses this, and the build must keep refusing it" — a floor that discriminates
CONTINUOUSLY during the build, where an xfail only discriminates at promotion.

## §1 Pin register (slug · kind · invariant · argument · HEAD failure-mode)

### X-flagship — `guard23-ternary-flagship` (XFAIL)
The centerpiece: one book, all three verdicts. site 0 converged-before-wall ⇒ ELIDE (unchanged
from HEAD — the guard tier must never downgrade a provable elision; two-halves doctrine + rul-
attention-honesty make provable elision the only attention-saver); site 1 `hork` opaque ⇒ RUN +
poison-wall; site 2 converged-past-wall + vouch ⇒ GUARD; site 3 diverged-past-wall + vouch ⇒
RUN (mint is converged-only; jc-mint-policy below). Pins, concretely:
- the artifact shape (golden bytes): preamble = the oracle's check body shipped STRIP-ONLY
  (`pkg : package = "$1"` → `pkg="$1"`, nothing else changed — the annotation-strip is the only
  byte delta from the authored oracle); the guarded line is exactly
  `apt_get__check install -y curl || apt-get install -y curl` + a postfixed reason-comment —
  original bytes verbatim as the `||`-right (inv-g2 ancestry, now rul-ternary-verdict's "no
  code path removes them"); NO engine-synthesized sh anywhere (the two never-clauses, pinned
  negatively: nothing in the golden exists that the oracle author did not write, save the
  invocation, the `||`, and comments);
- the behaviour (expected.ran): apply under mocks runs `hork wombat`, then site 2's check-body
  command `dpkg-query -W curl` (rc 0 ⇒ short-circuit ⇒ NO `apt-get install -y curl`), then
  `apt-get install -y vim` bare. Book order preserved (gate-4 ordered compare);
- the probe half: vouched sites PAST the wall still ship their read-only probes (records for
  sites 2 and 3) — the witness triple needs the verdict, and "plan-prediction and apply-guard
  run the same code" (233 §guard-license) requires the plan-side prediction to exist. Site 1
  (hork, un-oracled) stays `skip-unresolvable` forever.
HEAD failure-mode (verified): ap-2-exec ran-mismatch + gate-1 parity (sites 2/3 records missing
from the HEAD probe). Content-diff is xfail-skipped; it activates at promotion.

### X-drift — `guard23-fallthrough-drift-runs` (XFAIL)
The single most safety-critical pin: a guard minted on plan-time-converged whose world DRIFTS
before apply (mock dpkg-query exits 1) must FALL THROUGH — check fails ⇒ `||`-right runs the
original mutator ⇒ downstream continues. A guard that trusts its plan verdict over its own live
read is wrong-elision one storey down, and the whole point of pushing "the given-but-incomplete
fact-base down into the apply phase" (the human's compression of the reshape) is that the guard
decides FRESH, at position, every time. This is also the TOCTOU-clarifier's "hork-catching is
in" made executable: the drift here has a named, in-book, potentially-responsible cause.
PROBE_RESULTS=authored (plan-time holds deliberately contradicts the mock host — the
contradiction IS the case; gate-1 parity is structurally impossible and honestly declared so).
HEAD failure-mode (verified): ap-2-exec ran-mismatch (no check invocation in HEAD's run-set).

### X-canttell — `guard23-fallthrough-canttell-runs` (XFAIL)
Sibling of X-drift, mock rc 2: ANY nonzero check answer falls through (holds(0) skips;
absent(1) runs; cant-tell(2+) RUNS — when unsure, act; inv-kfail apply-direction). The `||`
form gives this for free; the pin exists to catch a future rewrite (an `if`-form, an
rc-classifying wrapper, a "treat 2 as pass" erosion) that mishandles the third outcome. The two
sibling cases differ ONLY in the mock's exit code — that difference is the pin.
HEAD failure-mode (verified): ap-2-exec ran-mismatch.

### X-mutfail — `guard23-mutator-fails-book-continues` (XFAIL)
rul-divergence-proceed's mechanical half: after fall-through the mutator itself FAILS (apt-get
mock exits 1) and the un-errexit'd book CONTINUES (marker runs; artifact exits 0) — byte-faithful
to bare sh. The engine adds no second-guess layer: no wrapper may swallow, retry, abort-on, or
report-and-stop the mutator's failure in-line. (The errexit "natural loud stop" variant is
deliberately NOT authored — see §4 np-errexit.)
HEAD failure-mode (verified): ap-2-exec ran-mismatch.

### X-why — `guard23-why-attribution` (XFAIL, analysis-only)
The disclosure floor: a guard decision must reach the user's stderr why-lens naming (i) the
mechanism (guard), (ii) the license (the converged-vouch), (iii) the licensor (the oracle).
Attribution is the guard-license's entire enforcement story ("attribute, don't prevent" — the
human's intent-smuggling ruling), and rul-attention-honesty makes the disclosure load-bearing: a
guard the user can't trace to its licensor is hidden risk. gate-7 patterns are deliberately
loose substrings (`guard` / `vouch` / `package`); wording is the builder's (jc-why-wording).
HEAD failure-mode (verified): gate-7 (no why-lines at all for this book at HEAD).

### X-heredoc — `guard23-heredoc-refuses-loudly` (XFAIL)
The ratified refuse-home posture, pinned representatively on the heredoc case: a vouched,
converged-past-wall site whose leaf carries `<<EOF` stays RUN (span-edit would strand the
payload — the render21-heredoc-refusal precedent) and the refusal is LOUD (gate-7 pattern
`refus`) — never a silent downgrade. Behavioural floor rides along: expected.ran equals HEAD's
(hork + the install), so a build that wrongly guards the heredoc line fails the exec gate too.
An `expected-diagnostics` file pre-declares the substring `guard` so an error-severity refusal
does not trip gate-3 at promotion (inert at HEAD — declared-but-unseen patterns don't fail).
HEAD failure-mode (verified): gate-7 (`refus` unmatched) + gate-1 parity (the golden's probe
half ships the vouched site's record; HEAD doesn't).

### P-novouch — `guard23-no-vouch-runs` (floor)
"No vouch ⇒ run", the flagship's CONTROL: same wall shape, oracle WITHOUT the vouch line,
converged-past-wall site runs BARE forever. The tripwire against minting from anything other
than an explicit converged-vouch — from the effect-map headline ("install establishes
installed, so installed ⇒ skippable" is the claim-noop conflation: `dpkg -s nginx` passing does
not make `apt-get install nginx` skippable — the upgrade case, 218a hunt-A), from the probe
verdict alone, from kind-membership, from anything ambient. Silence stays meaningless (233:
neither vouches nor poisons; merely fails to upgrade). Its probe-results deliberately carry NO
records for the walled site, with an in-file note that the unvouched-hint-probe question is
OPEN and any re-author must be conscious (jc-probe-scope).

### P-topargv — `guard23-top-argv-runs` (floor)
The constprop half of the witness: `PKG=$(cat /etc/pkg); apt-get install -y "$PKG"` — ⊤
operand ⇒ the check is never evaluated ⇒ no reached path ⇒ no vouch ⇒ no witness ⇒ run, vouch
present or not (233: "Unpropagatable argv ⟹ no path reached ⟹ no vouch ⟹ run"). Tripwire
against provider-keyed vouch scoping. kFAIL-perform corollary: unknown identity never elides —
and never guards either (a guard needs an entity to check).

### P-rundelta — `guard23-rundelta-never-guards` (floor)
Run-delta verbs never guard; an oracle DECLINES by not vouching (rul-guard-license's own
sentence). `systemctl restart nginx` with a service oracle that models-and-vouches `enable`
only: restart reaches no vouched path (no effect row, no probe, no verdict), so no witness ever
forms. A state-guard on restart is the forbidden wrong-skip — "is-active" passing must never
eat a restart the book demanded (the command's value IS the run). Tripwire against
provider-level vouch bleed ("some systemctl path is vouched" licensing nothing-in-particular).

### P-rcreaders — `guard23-explicit-rc-consumers-run` (floor)
The narrowest, uncontested slice of the rc-consumer question (per the mid-run ruling): sites
whose status the admin's own sh EXPLICITLY reads — `if apt-get …; then`, `apt-get … || echo
fallback`, `apt-get …; rc=$?` — never guard, converged-and-vouched or not. The admin's spelled
intent wins; a wrong vouch suppressing a written `|| fallback` is the stacked-failure disaster
(218a hunt-B). All three shapes converged + vouched (maximal bait), all three run at HEAD
(consumed-⊤-status blocks the elide-tier) and must keep running. NO book in the set uses
`set -e`, and no pin asserts either answer to the errexit-implicit question (§4 np-errexit).

### P-stdout / P-cmdsub — `guard23-consumed-stdout-runs` / `guard23-cmdsub-position-runs` (floors)
Consumed-output positions never guard: the pipe (stdout feeds `tee`) and the substitution
position (`out=$(apt-get …)`). A guard's pass-direction replaces the tool's output with the
check's bytes — value corruption at the consumer. Both ride existing HEAD refusals (consumed-⊤
stdout blocks elide; cmdsub bodies are non-leaf) and pin that the vouch un-blocks neither. The
ratified loud-refusal disclosure for these two homes is NOT separately gate-7-pinned (wording-
risk economy; X-heredoc is the representative disclosure pin — §4 np-refusal-why-each).

### P-multiop — `guard23-multioperand-atomic-runs` (floor)
"The command is the atomic unit" (mid-run axiom): `apt-get install -y nginx curl`, vouched —
the check's own argparse refuses the second operand ⇒ no entity ⇒ no probe ⇒ no witness ⇒ the
WHOLE line runs. Case comment carries the axiom's forward half (whole-line all-or-nothing IF a
whole-line witness ever exists; one diverged member ⇒ whole line runs; per-member split
hard-deferred, enrichment path = the author rewrites to a loop) and the refuse-path-rc-0 hazard
(§6 hz-refusepath), which this floor is the standing tripwire for.

### P-handguard — `guard23-already-hand-guarded-runs` (floor)
No-double-guard: `dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx` past a wall, the
install's oracle vouched — the machine must never stack `handguard || (check || install)`
(218a d4-6 already-guarded refusal; admin-explicit wins in the guarded direction too). The
refusal precedes any verdict question. The hand-guard is a MODELED query (pkgstate oracle) so
the eventual same-fact recognition has a fact to recognize (218a hunt-C). Post-build regression
signature: preamble + rewritten line (content diff) and/or a second check invocation in the
run-set. Note: mock host has nginx ABSENT so the hand-guard falls through at exec in both rails
(gate-5 needs bare ⊇ engine-run; a short-circuiting hand-guard would hide the install from the
bare log — the same structural constraint run.sh documents for fold cases).

### P-pair — `guard23-vouch-inert-pair-a` / `-b` (floor pair, differential)
rul-guard-license's plane-fence, pinned differentially: two cases byte-identical except the
vouch lines in -b's oracles; goldens byte-identical apart from the case-name comment line, and
they must STAY in lockstep forever — the vouch may never change which sites elide, poison, or
run (a vouch softening poison launders a local skip-judgment into a global non-interference
claim — 233 §0 rebuilt one storey up). Also pins, in the -b half: the guard tier never STEALS a
provable elision (site 0 keeps eliding with the vouch present). HONESTY NOTE, load-bearing for
the crosscheck: both halves' service-site elision rides the KNOWN 233 §0 ambient hole (a
modeled-but-narrow apt oracle poisons nothing it doesn't declare, so the downstream converged
`systemctl enable` elides at HEAD). The pair pins the DIFFERENTIAL, not that absolute — the
elide-tier's wound is documented in the book comments, not endorsed; when its fix lands, both
goldens flip in lockstep and the pin survives. The harness cannot diff across cases (§3
jc-pair-mechanics); reviewers must.

### P-reingest — `guard23-reingest-collision-verbatim` (floor)
The off-ramp closure, three pins in one: a hand-written guarded artifact (preamble fn + `check
|| original`) fed back to dorc (1) re-parses and runs VERBATIM, safely ("safe, merely
unimproved" — the un-oracled fn call is opaque ⇒ conservative run); (2) never ACCRETES a second
guard; (3) the book defines `apt_get__check` — the very name the guard tier ships — and a
collision-dodging RENAME is unspellable under strip-only sourcing (rul-ternary-verdict:
`name.check()` → `name_check()`, nothing else changed), so refuse-and-run is the only lawful
verdict near this book. sh function redefinition is last-writer-wins; a preamble emitted after
the book's def would be hijacked — the 218a name-collision refusal, now derivable from the
strip-only ruling itself (+SURE of the derivation; the case comment carries it).

### P-inloop — `guard23-inloop-unchanged` (floor)
Donor: `loop-members-all-converged-elides` + vouch; goldens identical. In-loop sites are
outside the guard tier's initial reach (233's can't-serve list; 218a u-9 per-iteration
check-then-act deferred), and the vouch must not perturb the existing member-granular
elide machinery (task-L2, probe-fact-licensed). If a build reaches guards into loop bodies, the
case goes red and the reach must be argued consciously. (No partial-member pin exists anywhere
in the set — the atomic axiom; the pre-crisis `xf-partial-member-elide` family is PARKED per
`23Z` task-4 rulings.)

### P-background — `guard23-background-not-guarded` (floor)
`cmd &` is a ⊤-reject (inv-top-reject; declared error-diagnostics per the donor
`background-amp-runs`) — un-probeable, un-elidable, and un-guardable: no witness forms on a ⊤
site, and the vouch must not soften a ⊤-reject ("a build that uses the vouch to model its way
past the async wall has broken two fences at once" — the case comment). The ratified refuse-home
posture for backgrounded commands is thereby pinned at its floor (run + loud, the loudness
being the pre-existing ⊤-reject errors).

## §2 The strawman vouch spelling (and its deliberate wrongness)

`oracle_vouch_converged='<provider> <verb>'` — one inert sh assignment in the oracle file,
under a loud `STRAWMAN … NOT DESIGN` comment block at every use. Chosen because: (a) verified
byte-inert through the HEAD engine (no diagnostics, no output delta — the P-pair cases pin
exactly this inertness); (b) a single greppable line, trivially swappable (the ruling's
requirement); (c) reads as data, not as a fake mechanism. Two knowingly-wrong aspects, flagged:
- it keys (provider, verb) where the settled design keys REACHED PATHS through the check body
  ("per-verb is sloppy vocabulary" — 233 §guard-license). For every fixture in this set the two
  scopings coincide (each vouched verb-path is reached exactly by the sites the row names), so
  no pin's BEHAVIOUR depends on the difference; the pins that police scope (P-topargv,
  P-rundelta, P-multiop) are spelling-independent (they turn on constprop reachability, which
  is engine machinery). ~SUSPECT this is the least-bad available stand-in; the real spelling
  (the dq-kOOB vouch-surface family) replaces the assignment and the comment block wholesale.
- it cannot express path-grain vouches (two verbs one vouched, inside one case-arm, etc.) —
  behaviours needing that grain are NOT pinned (§4 np-pathgrain).

## §3 Judgment calls flagged (jc-*) — made-for-now, cheap to reverse, never silently

- **jc-body-source** (the big one): golden guard bytes follow the ORACLE GROUND-TRUTH reading —
  whole stripped CHECK body shipped as the preamble, invoked with the site's argv
  (`apt_get__check install -y curl`). The spike's probe lane at HEAD ships `oracle_probe_*`
  bodies instead (st-2), and 218a's inv-g3 pointed at THAT shape — the build-vs-design
  divergence `23Z` flags for reconciliation. I did NOT settle it: the behavioural pins
  (expected.ran) are IDENTICAL under either sourcing (both bodies reduce to `dpkg-query -W
  <pkg>` under mocks — verified), so only golden bytes commit; if the builder lands the
  probe-wrapper shape first, the goldens churn cosmetically at promotion, visibly, under
  review. The golden documents the ruling's letter ("the check IS the oracle"); the
  probe-half of the same goldens stays at HEAD's st-2 shape, so "same bytes both lanes" is
  NOT yet satisfied inside any single golden — deliberately, since reshaping the probe lane
  is the reconciliation's job, not the guard tier's. Flagged, not resolved.
- **jc-mint-policy**: the set pins m-a (mint at converged-past-wall only; 218a d4-4's default,
  task #7's sketch). Flagship site 3 (diverged+vouched ⇒ bare run) encodes it. If m-b
  (guard-wherever-vouched) is ever preferred — 218a argues its durability upside — the flagship
  golden changes consciously. The ruling's letter ("a matching (call-site, reached vouch,
  probe-verdict) witness") reads most naturally as m-a to me (~SUSPECT); the crosscheck should
  test that reading.
- **jc-silencing**: rul-ternary-verdict's form is bare `<check-invocation> || <original>`;
  218a d4-2 had call-site `>/dev/null 2>&1`. Goldens follow the ruling's letter (bare). Open
  consequence for the builder: a check body that PRINTS leaks onto the apply transcript —
  observable-preservation of the artifact's output argues for the 218a redirect; adding it
  later is one golden-visible change. Deliberately unpinned.
- **jc-probe-scope**: probes-past-wall pinned ONLY for vouched sites (the witness needs the
  verdict). Whether UNVOUCHED walled sites also ship hint-probes (the "expected: 1 change, 96
  no-op" plan-prediction wants them; the kill-agent's "guard-compiler with hint-probe" framing)
  is deliberately unpinned — P-novouch's results file carries the conscious-re-author note.
  Same posture for the un-consumed QUERY probe past a wall in P-handguard (HEAD ships it;
  golden follows HEAD).
- **jc-default-on**: the harness invokes dorc with no flags, so the xfail set can only promote
  if the guard verdict is ACTIVE in the default invocation. Delta-1's signing IS the door-4
  deferral reversal, and `23Z` step-4 says "default flipped only after the delta-1 re-weld is
  signed" — signed 2026-07-02 — so I read default-on as the end-state (+SURE of the reading,
  ~SUSPECT of the build sequencing). If the slice lands flag-gated-off first, the set stays
  xfail (safe, inert) until the flip or until the harness plumbs the flag; either is a
  conscious step, not a silent one.
- **jc-why-wording**: gate-7 patterns (`guard`, `vouch`, `package`, `refus`) assume disclosure
  wording. The pins' substance is the disclosure's EXISTENCE and attribution-content; if the
  built vocabulary differs, patterns adjust at promotion — visibly. A build could
  theoretically satisfy `guard`+`vouch`+`package` with a why-line that discloses without
  attributing blame-to-the-mark (-GUESS, low); the crosscheck should read the built wording.
- **jc-guard-comment**: the postfixed reason-comment on guarded lines (task-3 ruling: "guards
  appear inline as real code with postfixed reason-comments") is pinned by presence in the
  flagship golden; its wording (`# dorc: guard [package converged-vouch; probe: holds]`) is
  mine and incidental.
- **jc-pair-mechanics**: the harness has no cross-case assertion, so the P-pair byte-equality
  is enforced by documentation + review, not mechanically. A ~10-line harness extension (a
  PAIRED_WITH marker diffing two cases' goldens) would mechanize it; I did not modify the
  harness (my remit was cases; run.sh changes deserve their own review). Flagged as a gap the
  crosscheck should weigh.
- **jc-ran-vs-golden division**: wherever behaviour and bytes could disagree, I leaned on
  expected.ran/EXIT_RC (behavioural, sourcing-invariant) as the load-bearing pin and let
  expected.out carry shape. This is why every xfail with an executable surface fails the EXEC
  gate at HEAD, not merely a text diff.

## §4 Deliberately NOT pinned (np-*) — with the reason each is a decision, not an omission

- **np-errexit**: whether errexit-IMPLICIT status consumption blocks guarding — DEFERRED by
  the mid-run human ruling (→ task #13); both defaults suspected painful. NO book in the set
  uses `set -e`; no pin asserts either answer. Dropped from my draft set: an errexit-book
  "guard-fail must not crash the book" pin (the `||`-left exemption) and an errexit
  "mutator's failure is the natural loud stop" pin — both presuppose guards mint inside
  errexit regions, i.e. exactly the deferred question. The exemption mechanics (218a d4-2,
  213's errexit-region modeling) remain design-argued but unpinned; when task #13 resolves,
  BOTH directions need pins (mint-side and refuse-side), plus the set -u hazard (§6 hz-setu).
- **np-partialmember**: nothing anywhere pins per-member/partial-line behaviour — the atomic
  axiom; the pre-crisis `xf-partial-member-elide` family is parked upstream of me.
- **np-onebody**: one-body-two-lanes byte-identity (probe bytes ≡ guard bytes) — a CANDIDATE
  invariant the human explicitly flagged "bears more thought", routed to task #2; 239 says "if
  later welded, a byte-identity xfail pin follows". Pinning it now would weld it by test.
- **np-wave**: hoisted post-wall re-verification waves — gated on the task #11
  placement-spectrum round ("wave xfails" are named as gated in 239 §2). Per-site guards only.
- **np-costband**: check-cost banding (expensive checks earn their vouch or just-run) — task
  #8, parked, no sanctioned data source.
- **np-hatches**: the escape-hatch taxonomy (`notes/235`) — task #10, parked, human on-fence.
- **np-differentfact**: a hand-guard checking a DIFFERENT fact than the site's (218a hunt-C:
  same-kind-different-entity must NOT refuse-as-already-guarded) — open design nuance; pinning
  either way would settle it. P-handguard pins only the same-fact case.
- **np-pathgrain**: vouch-scope at path grain finer than a verb (one arm of a case vouched) —
  unpinnable without committing a spelling (the exact situation the prompt told me to set
  aside). Same for the vouch's own strip-fidelity (that the real spelling strips out of
  shipped bodies): my assignment-spelling never enters a body, so there is nothing to strip;
  the dash-n gates pin general runnability only.
- **np-declaredoutput**: the second never (declared/claimed output in guard-position) has no
  spelling at HEAD to build a temptation-fixture from; it is pinned only NEGATIVELY (no
  golden contains any output-producing insertion; flagship bytes are the evidence) and by the
  inv-probe-sourced-values carve-out's text. A positive "engine refuses a declared-output
  spelling" pin waits for any such spelling to exist.
- **np-refusal-why-each**: loud-refusal disclosure is gate-7-pinned once (X-heredoc); the
  cmdsub/background/pipe homes pin only the RUN half. Tripling the wording-risk surface for
  the same doctrine bought coverage-theater, not coverage (~SUSPECT; cheap to add later).
- **np-plan-render-tui**: rul-attention-honesty's render doctrine beyond the artifact (no
  fold-by-default, dimming-maybe) lives on the PLAN-RENDER surface (rec-1), which has no e2e
  harness surface; the artifact-plane half (elided lines present-as-comments, guards inline
  as real code, whole book, original order) IS pinned by the flagship golden.

## §5 What the builder must touch (the honest churn/coupling list)

- **gate-6 widening (REQUIRED before flagship promotion)**: the dual-rail judge's direction
  (i) "apply never runs anything NEW" false-fails a legitimate guard (apply-only check-body
  commands: `dpkg-query -W curl`), and direction (ii) has no license class for a
  guard-suppressed mutator (bare-only `apt-get install -y curl` with only replace/omit
  attributable). run.sh's own comments anticipate this ("door-4-era amends this"). Sketch,
  NOT settled: `--debug-argv` grows a `guard` disposition (gate-5's run-filter then skips
  guarded sites automatically — its ledger line carries the SITE argv, so no gate-5 change);
  direction (ii) accepts a `guard`-disposition ledger entry as license for its site's argv
  IFF the apply log shows the guard's check-command(s) ran before the suppression point
  (pass-observed attribution); direction (i) allowlists apply-only lines that match the
  shipped preamble's command set (the engine can emit a `guardcmd <argv0>` ledger line per
  preamble body command — strawman). The dual_rail_selftest MUST grow confounds proving the
  widened judge still screams (a `guard` entry must not license an UNRELATED elided line; an
  apply-only line not in the preamble set must still scream). Cases already marked
  PROBE_RESULTS=authored (the drift trio) stay gate-6-excluded regardless.
- **probe-past-wall shipping**: flagship + X-heredoc + X-why goldens require vouched walled
  sites to probe; P-novouch and P-handguard carry conscious-re-author notes for the unvouched
  halves. gate-1 parity then binds the new records (flagship's mocks already answer
  correctly: nginx/curl 0, vim 1).
- **promotion mechanics**: each xfail promotes by deleting its XFAIL file; content-diff then
  activates against my hand-authored goldens — cosmetic divergence (comment wording,
  whitespace, emitter forms) is EXPECTED and re-golden'd under review; the invariants listed
  per-pin in §1 are the non-negotiables. An XPASS before the builder intends promotion means
  a behaviour landed un-reviewed — investigate, never just delete the XFAIL.
- **the heredoc refusal's severity**: if error-severity, X-heredoc's expected-diagnostics
  (`guard` substring) covers it; if Note/Warning, gate-3 never sees it and only gate-7 pins
  the loudness. Builder's choice, u-6-adjacent (218a).

## §6 Hazards surfaced for the crosscheck (hz-*) — attack here first

- **hz-refusepath (found while authoring; the sharpest)**: the corpus-standard check bodies
  EXIT 0 ON THEIR REFUSE PATHS — `if [ "$2" = "" ]; then dpkg-query …; fi` returns 0 when a
  second operand makes the condition false, and `case` with no matching arm returns 0
  (service oracle, restart). A build that ships a check body as a guard WITHOUT proving the
  invocation constant-propagates to a VOUCHED path mints `check || install` where the check
  rc-0s vacuously ⇒ the mutator is suppressed on a path the author never vouched — silent
  wrong-elision. The witness's reached-path component is load-bearing precisely here;
  P-multiop and P-rundelta are the standing tripwires. The crosscheck should hunt more shapes
  of this class (fall-through `case` arms, functions whose last statement is a passing test,
  set-but-empty verbs).
- **hz-provider-bleed**: any vouch scoping keyed wider than the reached path (provider-level,
  kind-level, file-level). Tripwires: P-rundelta, P-topargv, P-novouch (the control).
- **hz-ambient-hole**: the P-pair goldens ENCODE the 233 §0 ambient elision (documented, not
  endorsed). A reader could mistake the golden for an endorsement; the book comments say
  otherwise, loudly. The elide-tier fix will churn those goldens in lockstep — verify the
  lockstep when it happens.
- **hz-correlated-lie**: a lying check body misleads the plan AND acts at apply (218a
  world-4; 239's "identical in kind to the residue HEAD already carries", narrowed by
  fall-through-to-run). Nothing in this set can pin it away — it is the accepted trust edge
  (a) of 239 §1. The set pins its CONTAINMENT (attribution, fall-through, no values minted),
  not its absence. The anti-masking discipline holds: no fixture hand-injects an observable
  its check should produce — the drift cases' contradiction is between PHASES (authored
  plan-verdict vs mock host), which is the modeled reality, not a mask (~SUSPECT this is the
  correct reading of the discipline; crosscheck should check it).
- **hz-setu**: a shipped check body under the book's `set -u` can die on an unset-parameter
  expansion (`[ "$2" = "" ]` with one arg — 218a u-11). No book in the set uses `set -u`;
  unpinned, unresolved, and it composes with np-errexit (both need the task #13-adjacent
  round). The corpus-standard check-body idiom is itself set-u-unsafe — worth a lint
  eventually (oracle-author tooling).
- **hz-strip-scope**: the strip is defined "annotations removed, `name.check()` →
  `name_check()`, nothing else" — my goldens strip exactly one annotation form (`var : kind =
  value` → `var=value`). Other annotation forms exist in the wild fixtures (`expr : T:i.p`
  punning, `!`-suffixes from 233's strawmen); none appear in guard23 oracles, so the strip's
  totality is UNPINNED beyond the one form (-GUESS this bites during the build; the
  strip-function needs its own unit surface).

## §7 What strained (process notes for the next derivation)

- gate-5's one-directional assertion (engine-run ⊆ bare-log) structurally forbids fixtures
  where a hand-guard SHORT-CIRCUITS its mutator under mocks (the mutator is engine-`run` but
  absent from the bare log). Both hand-guard floors therefore mock the ABSENT world
  (fall-through in both rails). The converged-hand-guard world is exec-pinnable only after
  gate-6/5 learn the guard disposition — noted in §5.
- HEAD ships QUERY probes past walls (their fold-validity dies, their shipping doesn't) —
  finer than my model predicted; P-handguard's fixture had to follow the machine, not my
  sketch. Establish probes do NOT ship past walls at HEAD; that asymmetry is what the
  flagship's probe-pin changes for vouched sites.
- The e2e goldens embed the book's comment block (comments flow through the apply render), so
  book headers are load-bearing fixture bytes: write the pedagogy FIRST, then generate. Two
  authoring traps hit and fixed: a nested-heredoc terminator ate a book mid-file (X-heredoc),
  and a `head -10` off-by-one duplicated a donor's loop line (P-inloop) — both caught by the
  harness itself (the exec gates screamed), which is the harness working as designed.
- The three mid-run rulings landed while the set was half-drafted; the errexit cases were
  redesigned to plain books (their fall-through/faithfulness content survived; the
  exemption-mechanics content moved to np-errexit), and the multi-operand pin was reframed
  from "never guards" to "no whole-line witness ⇒ runs" with the axiom's forward half in the
  case comment. Nothing in the landed set asserts against any of the three rulings.

## §8 Validation ledger

`cargo build` clean at f333db7+cases; `sh e2e/run.sh` → `all 118 e2e round-trips passed`,
6 xfail / 0 XPASS / 0 red (DORC_E2E_QUIET both on and off); every xfail's failing-gate set
verified individually with the XFAIL lens lifted (reasons recorded per-pin in §1); every
golden `dash -n` clean; every passing floor's golden generated from the HEAD binary and
inspected line-by-line for safe behaviour before adoption (no BLESS run was ever used —
BLESS is exclusive/orchestrator-only per spike/CLAUDE.md).
