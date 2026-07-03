# 23B — guard-tier pin-set review (independent assessment of the 23A set)

AI-authored (review agent, own worktree, 2026-07-02). Scope: the 19 `spike/e2e/cases/guard23-*`
cases (6 XFAIL + 13 floors) landed in `e5bdbf9`, judged against the round-23 rulings block
(`spike/CLAUDE.md`), stamped `plans/233` (+ end-annotation), `plans/239`, `notes/23Z`, and the
authors' register `notes/23A`. Never-vouch applies: this is process-evidence, not proof. Method
was empirical where possible — harness re-run, XFAIL lenses lifted, golden artifacts EXECUTED
under their own mocks, wrong-build artifacts hand-crafted and executed to test floor
discrimination. Confidence marked +SURE / ~SUSPECT / -GUESS throughout. (A sibling review runs
in parallel under 23C; this note was written without sight of it.)

## §1 Verdict in one paragraph

+SURE the set is faithful to the signed rulings, honestly argued, and satisfiable: every
hand-authored xfail golden, executed under its own mocks, reproduces its pinned ran-set and exit
code exactly; every lens-lifted failure signature matches 23A §1's claims; the floors I
attack-tested discriminate the exact disasters they claim (live wrong-build artifacts produced
the silent wrong-elision each floor exists to catch — hz-refusepath is real and demonstrable).
The register's jc-*/np-*/hz-* disclosure is genuinely candid. Two moderate findings temper this:
the build window has NO floor against the single most dangerous regression class
(vouch-softens-the-wall at the elide tier — every vouch+wall fixture is XFAIL, and the XFAIL
lens is one-sided, review-fd1); and the welded strip rule's name-rewrite half is both
unexercised by any pin and, at its letter, produces non-runnable sh for hyphenated command
names (review-fd2, a ruling-text gap the fixtures dodge rather than expose). Neither blocks the
build; review-fd1 deserves a cheap fix BEFORE the build slice starts.

## §2 What was verified (the confirmation ledger)

- review-conf1 · baseline: fresh harness run reproduces `all 118 round-trips passed`, 6 xfail,
  0 XPASS, 0 red (binary from the sibling worktree; the r23 arc carries zero Rust changes, so
  the sibling build is byte-faithful source-wise). +SURE.
- review-conf2 · lens-lift: all six XFAIL failure signatures at HEAD match 23A §1's per-pin
  claims exactly (flagship: ap-2-exec + gate-1 parity; drift/canttell/mutfail: ap-2-exec;
  X-why: gate-7 all three patterns; X-heredoc: gate-7 `refus` + gate-1 parity). +SURE.
- review-conf3 · satisfiability (the load-bearing check): each mocked xfail's GOLDEN apply
  artifact, executed under the case's own mocks in run.sh's exec environment, reproduces
  expected.ran byte-for-byte and exits 0; golden probe halves reproduce the authored records
  (flagship + heredoc parity-clean at promotion; the drift trio's deliberate probe/mock
  contradiction properly declared via PROBE_RESULTS=authored). The pins are not
  self-contradictory; a correct build can flip them. +SURE.
- review-conf4 · floor discrimination, attack-tested live:
  - P-rundelta: hand-crafted provider-bleed artifact (`systemctl__check restart nginx ||
    systemctl restart nginx`) yields an EMPTY ran-set — the case-statement's no-match rc-0
    silently suppresses the restart — the floor's expected.ran catches it. hz-refusepath
    demonstrated, not just argued.
  - P-multiop: unlicensed whole-body ship (`apt_get__check install -y nginx curl || …`) yields
    an EMPTY ran-set (the `[ "$2" = "" ]` refuse-path rc-0) — caught.
  - P-novouch: a wrong mint adds `dpkg-query -W curl` and suppresses the install — caught.
  - P-reingest: guard accretion doubles the `dpkg-query -W nginx` line — caught by exec.
  - P-pair: engine run on -a vs -b inputs produces stdout identical modulo the case-name
    comment; stderr differs only in the input decision-digest. The vouch is byte-inert at HEAD
    as claimed. +SURE.
- review-conf5 · rulings compliance: strip-only preamble (one annotation form, `pkg : package =
  "$1"` → `pkg="$1"`); original bytes verbatim as the `||`-right; nothing engine-synthesized
  beyond invocation + `||` + comments; GuardInsert mints no values; whole book, original order,
  elided lines present-as-comments; site 0 elide never downgraded (two-halves); m-a mint
  (flagship site 3 bare); no `set -e`/`set -u` anywhere (np-errexit honored — grep-verified);
  no partial-member pins; refuse-homes pinned (X-heredoc loud, others RUN-half only, flagged).
  +SURE on each.
- review-conf6 · jc-body-source's invariance claim verified: probe-wrapper and check-body
  sourcings both reduce to the same `dpkg-query -W <pkg>` argv under these fixtures, so the
  behavioural pins are sourcing-independent; only golden bytes commit to the check-body shape.
- review-conf7 · promotion story coherent: drift trio + X-heredoc + X-why can XPASS on
  engine work alone; the flagship additionally REQUIRES the gate-6 widening (23A §5 flags it,
  with a sketch and a confound-battery extension) — the one xfail with a harness-work
  dependency. gate-5's disposition filter needs no change (any `[a-z]+` word parses; `guard`
  is skipped by the run-filter automatically). +SURE.
- review-conf8 · donor derivations (P-inloop ← loop-members-all-converged-elides;
  P-background ← background-amp-runs) are clean deltas: comment blocks + vouch (+ a `wait`);
  diagnostics identical. +SURE.

## §3 Findings (severity-ordered)

### review-fd1 · MODERATE — no floor guards the wall against the vouch during the build window
The set's own door-3 family (license leaking beyond its fence) misses its most direct member:
a build that lets the vouch soften the POISON-WALL at the elide tier — i.e. wrongly ELIDES a
vouched, converged, past-wall site — is caught by NOTHING while the guard tier is being built.
Every (vouch + wall) fixture is XFAIL; the harness's XFAIL lens is ONE-SIDED (content-diff
skipped, any gate failure prints as plain `xfail`), so this regression keeps the xfails red
with a *changed* failure signature and flips nothing visible: red-because-unbuilt and
red-because-disaster are mechanically indistinguishable. P-pair cannot see it (its book has no
wall — there is no poison for the vouch to soften; the differential tests inertness only
against an all-elide baseline, a sensitivity limit the register does not name). P-novouch
cannot (it is the no-vouch control). 23A's own claim that floors "discriminate CONTINUOUSLY
during the build" is exactly what fails here. +SURE of the gap; ~SUSPECT it was invisible to
the authors because pre-crisis corpus habits equate floors with HEAD-true cases, and
"vouch+wall runs bare at HEAD" is HEAD-true but not promotion-stable (it flips to `guard` at
promotion), so it fits neither the floor nor the xfail template.
Cheap fixes, either/both:
- (a) a TEMPORARY floor (flagship-shaped book: vouch + wall + converged site, no XFAIL,
  expected = HEAD's bare run), documented as deleted-at-flagship-promotion; ~10 minutes.
- (b) the stronger, reusable harness nicety: an optional `head-expected.ran` (and/or
  `head-expected.out`) marker asserted WHILE an XFAIL file is present — turning every xfail
  into a two-sided pin (asserts the current safe behaviour until the designed behaviour
  lands, then the marker is deleted with the XFAIL). Fits the existing marker idiom; the
  flagship's accidentally-committed head-output conflict file (review-fd4) shows the authors
  already generated exactly this artifact while verifying.

### review-fd2 · MODERATE (ruling-text, not pin) — the strip's name-rewrite half is unexercised and letter-broken for hyphenated names
rul-ternary-verdict welds: strip = "annotations removed, `name.check()` → `name_check()`,
nothing else changed; the strip's output is runnable sh". Empirically (dash 0.5.x, this box):
`apt-get.check() { …; }` is a SYNTAX ERROR (the authored period-form is not parseable sh —
tension with the 23Z oracle-ground-truth "oracles are JUST SH" at its letter), and
`apt-get_check() { …; }` is ALSO rejected ("Bad function name" — dash forbids `-` in function
names), so the rule's literal output for every hyphenated tool is non-runnable, contradicting
the ruling's own runnability clause. The fixtures dodge both by pre-spelling `apt_get__check`
(hyphen→underscore + the spike's `__check` convention — transforms the ruling's "nothing else
changed" does not authorize), and no pin exercises any `.check()`-form strip. hz-strip-scope
flags the annotation-form half of this but misses the name half entirely. Not a pin defect —
the pins are consistent within the fixtures' world — but the welded rule needs an
identifier-munge clause before the strip function is built, and the golden preambles' own
pedagogy comments (which quote the `name.check()` rule while shipping `apt_get__check`) are
mildly self-inconsistent. Surface to the human as ruling-text debt. +SURE of the dash
behaviour (tested); ~SUSPECT the resolution is a one-line munge-spec amendment.

### review-fd3 · MODERATE (visibility, not defect) — delta-1's safety rationale rests on a sameness the pinned goldens do not realize
239 delta-1's rationale: apply-lane guard code is "bounded to the SAME trust-object the probe
lane already ships — same bytes, both lanes, no new code-class crosses the boundary." The
pinned end-state does NOT deliver that sameness: golden probe halves ship `oracle_probe_*`
wrapper bodies (HEAD's st-2 shape), golden guard preambles ship stripped `__check` bodies —
different bytes, two lanes, so at promotion the guard lane executes code the probe lane never
exercised (exactly the drift the human's one-body-two-lanes candidate invariant worries
about: "tested in the probe so it can be trusted at apply"). 23A flags the sourcing question
(jc-body-source) and defers byte-identity (np-onebody → task #2) honestly, but never names
that the SIGNED RATIONALE leans on the deferred property. Consequence for reviewers: until
the st-2 reconciliation or task #2 lands, delta-1's "same bytes both lanes" is design-intent,
not pinned truth, and the trust-boundary argument for the posture shift is correspondingly
weaker than 239's text reads. Recommend a one-line acknowledgment wherever the build slice is
chartered. ~SUSPECT this is worth surfacing to the human before promotion rather than after.

### review-fd4 · MINOR — committed SyncThing conflict artifact
`spike/e2e/cases/guard23-ternary-flagship/head-output.sync-conflict-20260702-032713-PHNHRER.txt`
is tracked (landed in `e5bdbf9`). A stray authoring-scratch conflict copy (its base
`head-output.txt` was never committed). Harmless to the harness (matches no marker/golden
glob) and incidentally useful (it documents HEAD's flagship output), but it is landed repo
noise from the PHNHRER box; SyncThing-conflict cleanup is human-owned — flag for removal (or
conscious adoption under a non-conflict name per review-fd1(b)).

### review-fd5 · MINOR — P-handguard's run-set regression signature is inoperative as fixtured
23A claims stacking would show as "preamble + rewritten line (content diff) and/or a second
check invocation in the run-set". Tested live: a machine-stacked guard's check invokes
`dpkg-query`, which has NO shim in this case's mocks → command-not-found is swallowed by the
body's own `>/dev/null 2>&1` → fall-through → the ran-set is byte-IDENTICAL to expected
(hork, dpkg -s, apt-get). Only the content-diff catches stacking. Cheap hardening: add a
`dpkg-query` mock (exit 1, world-consistent with the absent-nginx `dpkg` shim) so the
signature also fires in the exec gate. Content-diff is a real gate for floors, so this is
belt-and-suspenders, not a hole. +SURE (executed).

### review-fd6 · MINOR — P-cmdsub is analysis-only where an exec pin was available
`out=$(apt-get …); echo "$out"` is exec-able under mocks (the cmdsub body's apt-get would
log); the case ships no mocks, so the RUN half rides content-diff alone. Defensible economy;
noted for cheap hardening.

### review-fd7 · design observations, no action needed from the builder
- m-a mint policy (converged-only) is pinned via flagship site 3 while the ruling's letter
  ("matching (call-site, reached converged-vouch, probe-verdict) witness") does not literally
  say converged — 23A flags this ~SUSPECT, correctly. m-a is the strictly-safer reading: it
  confines the vacuous-pass hazard (hz-refusepath) to sites where suppression is the intended
  outcome. Endorse; suggest the human ratify "matching = probe-verdict converged" in a line.
- The P-pair goldens encode the KNOWN 233 §0 ambient-elision hole (documented-not-endorsed,
  loudly, in the book comments) — reviewed and accepted; the differential is the pin, and the
  lockstep-flip obligation when the elide-tier fix lands is clearly stated. Note the
  sensitivity limit from review-fd1: the differential can catch vouch-CHANGES-something only
  in a book where something is changeable; pairing it with a walled book is the real fence.
- Unpinnable-today items I hunted and judged acceptably absent, which the register does NOT
  list: (i) book-defined function SHADOWING a shipped check-body's callee (book defines
  `dpkg-query()` fn → hijacks the guard's internals; the 218a in-environment hazard family —
  no ruling settles detection/refusal, so neither direction is pinnable; deserves a line
  beside hz-setu in any future register); (ii) `cmd || true` (StatusInvariant) + vouch + wall
  — the interim rc-posture (any explicit `||` operand refuses) implies run-bare, but pinning
  it would pin an interim answer (np-errexit-adjacent; defensible either way); (iii)
  guard-invocation quoting fidelity for resolved-but-space-containing operands (no fixture;
  builder-churn item — guard sites are constant-argv by construction, so the surface is
  small). -GUESS none of these three bites before task #13/#2 resolve.
- The `name.check()` spelling question aside (review-fd2), the vouch strawman's
  (provider, verb)-vs-reached-path coincidence is correctly disclosed in 23A §2 and none of
  the scope-policing pins depend on the difference — verified by reading each (they turn on
  entity-resolution/constprop, which the strawman cannot influence).

## §4 Bottom line for the conductor

Build against this set. Before the build slice starts: land review-fd1's fix (option (b)
preferred), delete review-fd4's stray, consider review-fd5/6's cheap mock additions, and
surface review-fd2 + review-fd3 to the human as ruling-text/rationale debt (neither gates the
guard-half build; both gate how much the promotion is allowed to claim). The register 23A is
trustworthy as a map — every checkable claim in it that I tested held, with the two
overstatements named above (the handguard run-set signature; floors-discriminate-continuously
as applied to the vouch+wall class).
