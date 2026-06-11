# 21Xa — resumption sweep ledger (running read-log + quarantine-danger ratings)

> Context: the round-21 conductor session died to a harness failure mid-round (uncommitted
> work in `spike/crates/`, untracked 21K/21Y). This session resumed via 210 (human-authorized
> read) + the level-1 orientation list, passed the 210 GATE, and is sweeping the >211 corpus
> one document per inference-step, appending an entry here immediately after each read.
>
> PURPOSE FOR A FUTURE SURVIVOR: if this session also dies, pick up here. Every file this
> session ingested is logged below with a quarantine-danger rating; the last sweep entry
> bounds the suspect range (the killer is the first unlogged document in sweep order, or the
> one a dangling "reading next: X" line names). Resumption path that worked for this session:
> human hands you 210 → its level-1 orientation list (root docs, spike/CLAUDE.md, 20K/20U/20V)
> → this ledger → continue the sweep where it stops.
>
> Rating scale (content-based judgment + this-session experience):
> - `none` — ordinary engineering content; no security-adjacent material.
> - `low` — isolated security-adjacent vocabulary or adversarial sh examples in clearly
>   defensive/engineering context.
> - `elevated` — sustained security-domain content (e.g. hardening-guide material) or
>   agent-addressed instruction-shaped text; read only with defensive-context briefing.
> - `HOT` — do not read; segregate for the human / an explicitly-briefed agent.
> - `unread` — not ingested by this session (deliberately deferred or quarantined).
>
> Sweep order: 212–21H numeric, then 220/221/222, then 21K, then 21Y (prime suspect, dead
> last — it preceded one opaque crash already, per the human). Zero harness incidents so far.

## Pre-sweep reads (level-1 orientation, this session, in order) — all uneventful

- `Research/notes/quarantine-DO-NOT-READ/210-spike3-round21-priming-prompt.md` — rating:
  none. Round-21 priming prompt (human-authored charter; its own safety block + quarantine
  rule; orientation list; GATE; arcs arch-1..7; north star; open forks; process rules).
- `README.md` — rating: none. `DESIGN.md` — rating: none (taint/soundness vocabulary is
  domain-inherent, not hot). `IMPLEMENTATION.md` — rating: none. `KNOBS.md` — rating: none.
  `TODO.md` — rating: none (mentions a deferred seccomp/security-dive item; trivial).
  `STALENESS-AUDIT.md` — rating: none (AI-generated rulings ledger w/ verbatim human quotes).
- `AGENTS.md` (harness-injected) — rating: none. Root `CLAUDE.md` → AGENTS.md only.
- `spike/CLAUDE.md` — rating: none (working agreement, invariants, gates, BLESS rule).
- `Research/plans/20K-round20-take3-report.md` — rating: none.
- `Research/plans/20U-round20-overnight-addendum.md` — rating: none.
- `Research/plans/20V-errexit-doors.md` — rating: none.

## Known-unread (deliberate)

- `Research/notes/quarantine-DO-NOT-READ/1A0-hts-target-priming-prompt.md` — unread. Seed
  prompt for the r1A side-quest (H2SaLS hardening-guide rewrite, per 210/212 references);
  ~SUSPECT elevated-by-topic but human-authored; human decides who reads it.
- `Research/notes/quarantine-DO-NOT-READ/200-spike3-priming-prompt.md` — unread. Round-20
  seed; superseded inheritance.
- `Research/corpora/H2SaLS/*` (sibling worktree `ai-r1A-H2SALS`) — unread. Hardening-guide
  corpus; rate elevated-by-default per 212's own briefing rule (defensive rewrite of a public
  server-hardening guide; brief that context before any read).
- Step-5 per-need round-20 notes (20M/20S/20O/20T/20E/206/207/209/20A, ANALYZER-NEEDS.md) —
  unread this session by 210's "per-need, not prospectively."
- The uncommitted `spike/crates/` diff (+329/−2 across analysis/cfg.rs, analysis/effect.rs,
  analysis/tests/cfg.rs, core/diag.rs, coverage/lib.rs) — unread, pending sweep completion.

## Sweep entries (one per document, appended immediately after each read)

- **211 — round-21 plan-of-attack** — rating: none. (orchestrator, round-open):
  quarantine-free charter restatement (arch-1..7 + north star); green baseline verified at
  `f09ebd7` (66/66 e2e, six gates); wave plan w-0..w-6 + stretch (door-3 solo first, arch-1
  solo, crosschecks paired with next builds, door-4/2 in w-5, arch-6 in w-6); slug
  reservations 212–21C (explains gaps — 217/218/21A/21C reserved for crosscheck-
  reconciliations and door-4/arch-5 builds that later renumbered or didn't happen as
  planned); crosscheck budget ~25–30% of build spend, mandatory on arch-1/arch-2/door-4/
  arch-5; five pre-registered arch-1 bite predictions (heredoc refuse-class; golden
  adjudication bottleneck; StatusRenderFloor narrows-not-vanishes; floor-test re-homing at
  observable_matrix.rs:765–907 + analysis/tests/cfg.rs:1136+; in-loop/Members subtle churn);
  dq-surface ledger opened with run-evidence as a candidate second dq-1 cost-species (apt
  history.log/auditd/atime — elision removes evidence) and the door-4 mint-policy fork
  {m-a probed-converged-only (default) · m-b declared-kind-always · m-c even-unprobed}, both
  raised to the human at round-open.

- **212 — mid-round rulings batch-1 + r1A arrival** — rating: low (references the
  hardening-guide corpus and carries the briefing rule; itself plain rulings prose).
  (2026-06-10): dq-errexit-1 stays OPEN, adjudication evidence-driven (candidates arrive as
  constructed strawmen/corpus shapes, never assertions; run-evidence is ledger entry #1; the
  dashboard walk should collect more). dq-errexit-2 LEANING-yes DOWNGRADED to genuinely-open
  (kSILO-in-ownership-terms: oracle-default ownership "sharpens the cliff…"; consequence: the
  precedence seam keeps all three ownership models live; dashboard reports r-2/r-4-reachable
  separately from r-3-only so numbers decide). dq-errexit-3 directional ruling: door-4 is
  CLI-flag-gated for-sure (trust-boundary taxonomy: a bad oracle must never cause novel
  apply-phase actions — door-4 breaks boundary-3; plus the stacked-correlated-failure frame),
  so the spike builds door-4 LAST behind a seam defaulting `Never` (provably zero
  transforms), hostile pass mandatory; the product hard-defers regardless. Priority
  reshuffle: arch-6 dashboard BEFORE door-4 (revised tail: arch-1 xchk ∥ arch-2 → door-1
  cases + arch-2 xchk → arch-6 + arch-4 note → arch-5 → arch-3c flagged default-off →
  stretch). r1A complete: corpus at sibling worktree `Research/corpora/H2SaLS/` (harden.sh,
  census TSVs, 11 oracle seeds); NOT merged (dashboard reads sibling path read-only); census
  numbers must be verified before weighting (handed to a lower-capability agent partway).
  Standing human style rule: agent-readable material justifies by evidence/artifacts only,
  no characterizations of people.

- **213 — door-3 build** — rating: low (one adversarial strawman `true() { rm -rf /; }; cmd
  || true` in hunt-4 — destructive-command *example* in an analysis context; otherwise pure
  engine prose). (w-1 builder): mechanism landed clean and small —
  `core::Channel::StatusInvariant` (third consumed-Status variant); marking in
  `lower_and_or` (`door3 = (op==Or) && right_is_bare_true(right)`; bare Simple, no
  assigns/redirs, literal `true`); mark-union gate in `plan::consumption_ok` (Invariant never
  blocks; any blocking mark wins); the existing `Predicted::Top => StandIn::True` arm reused
  with invariance-as-license (weld-5 intact). +7 cfg pins, +5 matrix pins, 4 `door3-*` e2e
  cases, zero churn on the existing 66 goldens. Load-bearing strain (pre-registered d-5
  checkpoint FIRED): the render wall — the plan-level mint works (converged
  `apt-get install -y nginx || true` flips Run→Replace, verified via --debug-argv) but the
  line-granular render cannot express it: the `true` right-leaf is Pure⇒Run, run_lines-wins
  renders the line verbatim. Payoff case `door3-or-true-elides` authored as XFAIL asserting
  the correct empty-run-set, to XPASS when arch-1 lands. d-4 checkpoint PASSED: errexit
  already exempts the `||`-left (`lower_condition_region` → `clear_fallible_range`), no
  double-mark. Residuals: res-2 `cmd || true; echo $?` conservatively blocked (rc provably
  always 0 — future invariant-read refinement); res-3 chain over-mark (disclosure
  imprecision only); res-4 `cmd || true || other` conservative block flagged --WONDER as
  possibly a missed unlock. Hunt-list (for the crosscheck to exceed): marks-union range
  granularity (highest), errexit composition shapes, chains, `right_is_bare_true`
  false-positives — including the redefined-`true()`-function hole shared by every
  true-stand-in engine-wide — false-negatives (`|| :` deferred-by-choice), and render-wall
  wrong-reason-pass. Proposed the inv-one-observable doc-delta (third variant on a different
  axis: continuation-identity), which current spike/CLAUDE.md carries — applied.

- **214 — arch-1 leaf-exact render build** — rating: none (pure engine prose; hostile
  hunt-list is span/render arithmetic, no security-adjacent content). (w-2 builder):
  `render_apply` is now span-edit application (collect `(Span, replacement)` per elided leaf;
  `normalise_edits` enforcing disjoint-or-contained with outer-wins; right-to-left per-line
  splice; one provenance-comment emitter disclosing flattened originals). The entire
  carve-out family (T14 `inline_arm_subst`, F2 `inline_scaffold_subst`, `commented_line`,
  `LineRender`/`classify_lines`/`emit_apply_lines`, both detection helpers) DELETED not
  bypassed; `StatusRenderFloor` deleted — if/elif guards became ordinary `StatusRelaxable`
  substitution sites, while/until conditions became NEW `StatusIterated` (unconditional
  block keyed on iteration; defense-in-depth since `in_loop_body` also floors the condition
  — the priming prompt's d-4 guess that the condition sat outside the floor was WRONG for
  this impl). door-3 payoff XFAIL deleted — `true || true`, empty run-set, passes for the
  right reason (Replace span, rhs verbatim, not a spurious fold). d-5 audit: ZERO existing
  if-guard cases flipped (corpus accident — they're Pure or establish-polarity guards; the
  capability is proven by new case `render21-if-guard-query-elides`). Strains: strain-1
  compound-controller omit-safety (`is_neutralised` generalised to walk `subtree_leaves_all`
  — riskiest new code, flagged for crosscheck esp. ⊤-leaf-in-cond composition); strain-2
  provenance-comment embedded-newline dash-n breakage (fixed by flattening); strain-3
  multi-line edit line-collapse bookkeeping; strain-4 heredoc-refusal diagnostic plumbed as
  separate `render_refusal_diagnostics` keeping render pure (tc-flagged). Churn table: 31
  text-only cases, 2 newly-expressible semantic run-set shrinks (exec-multileaf-line-mixed,
  headline-guarded-realistic — orchestrator must eyeball both `expected.ran` diffs), 1
  payoff, 5 new render21-* cases (if-guard both poles, while anti-pole terminating-by-mock,
  heredoc refusal, multi-line pin). Full test re-homing table (every deleted test → named
  successor). d-10 verdicts: 20O find-6 latents unfrozen-and-correct (except `done > file`
  stdout-consumption gap — orthogonal cfg gap, still open); 20S hunt-4 frozen Members shapes
  DISSOLVE under layout-agnostic span edits. 75 e2e ×2 zero-xfail, 361 tests, all gates.
  Doc-deltas proposed (CLAUDE.md trio still describing RenderFloor as live — since applied;
  an-render-modes tension resolved). Hunt-list: span overlap/abutment under parser bugs
  (highest), comment-safety boundaries (CRLF, quoted `<<`, `;&`), if-guard channel gating
  (real-sink redirect must block), StatusIterated anti-pole (no constant-condition artifact
  ever), door-3×span interactions, the strain-1 walk, splice arithmetic (EOF/CRLF/UTF-8
  mid-char panic), disclosure fidelity.

- **215 — door-1 cascade cases** — rating: none (engine/corpus prose; block examples use
  sed/systemctl/rm in ops context). (w-4 builder): headline — door-1's cascade is FULLY
  GENERAL AT BASE, zero engine edits, extend-reach budget unspent: `fold::kill_rec` was
  always a total recursion over NodeKind (Group/Subshell/AndOr/If/Case/loops/pipelines), so
  guard-dead blocks already cascade-kill every leaf beneath; arch-1's span render was the
  only missing half (the line-render couldn't express `:`-per-dead-leaf inside a group —
  213's render wall was the door-3 instance of the same wall). Deliverable = 7 hand-derived
  `door1-*` cases (suite 75→82, zero churn elsewhere): cascade-block-elides (payoff —
  `true || { :; :; }`, empty run-set, restart elides as unreachable with no rc-provenance of
  its own), cascade-diverged-runs (pole — same book, guard rc 1, whole block live: deadness
  is probe-keyed never structural), cascade-multistatement (nested inner-if killed whole,
  arms become `:` never empty), and-form (`&&` dual folds at base symmetrically; ANALYSIS-
  ONLY because `set -e; false && {dead}` faithfully exits 1 and the exec gate has no
  expected-nonzero-exit opt-out — tc-exec-nonzero-exit flagged, suggests an `EXIT_RC=`
  marker), and-form-runs (the `&&` live pole, exec-gated), guard-below-mutators-invalid
  (st-3 pristine-prefix negative pole: one upstream oracled write invalidates the Query,
  everything runs; the sharper unshipped variant — an *elided* upstream mutator still
  statically invalidates — noted for future), door1-door3-inner-elides (composition d×c
  cell: inner door-3 mints inside a live door-1 block, `false || { true || true; systemctl
  …; }` — marks independent, tc closed for d×c, d×d left to crosscheck). §4 carries the
  charter-mandated run-delta distinction: door-1 elides a DEAD restart (branch-deadness from
  a probed STATE query) vs R2-CHANGEDELTA's LIVE-but-conditional restart (gated on a
  run-delta observable, never elidable via state-probe, cross-kind edge never synthesized) —
  litmus: swapping the guard's rc flips door-1's restart entirely; no rc ever licenses
  R2's. Hunt-list: side-exit blocks under ⊤-guards (must stay live), inner-Query probe-site
  emission harmlessness, d×d cell, pristine-prefix vs `cd`-writes-PWD upstream (~SUSPECT
  surprise candidate), heredoc-refused leaf inside dead block, all-four-pole exit-rc
  faithfulness. Strain-provenance-string: `false`-substituted guards' comment says "already
  converged" — cosmetic imprecision, pre-existing, flagged tc-provenance-string-coverage.

- **216 — arch-2 budget-bounded inlining build** — rating: none (pure engine prose). (w-3
  builder): same-file-earlier funcdef calls are now CFG-level subgraph splices at the call
  site — body AST freshly lowered AFTER the CALL node per call (un-detaching find-7),
  `$1..$9`/`$#` bound from the call's resolved argv via a separate post-solve `inline_pass`
  side-channel (Members precedent; never through the lattice; `PosLit`/`PosSplit` frags;
  bounded-iterated ×3 for depth-2 nesting); CALL stays the render/substitution unit (body
  sites `spliced_internal`, never render-edited, ship as `site N.M` sub-records of the
  call); all-or-nothing `LicenseVia::InlineCall` mints iff every EstablishAmbient body fact
  Converged, no blocker site, ≥1 establish (pure wrapper runs), call's consumed channels
  pass (call status ALWAYS ⊤ — no mini-fold needed, the i-5 stop-flag resolved: licensing is
  on body establish FACTS not body control-flow deadness; per-call fold impossible anyway
  since body AstIds are shared). Eligibility refusals all loud `cfg-inline-refused` warnings
  (tc-inline-refusal-severity flagged): forward-call silent, redefinition, recursion (stack
  guard + textual-order makes transitive cycles structurally unreachable), depth ≥2, body
  `$@`/`$*`/`shift`/`local`, tc-M2 non-devnull write-redirect, per-call 64-node AST estimate
  (deliberate over-refusing proxy — no rollback needed), per-book 1024. Two bugs caught
  in-flight: strain-1 detached-body double-count (fix: definition's detached body also
  `spliced_internal`); strain-2 call node's Opaque poisoning its own spliced body (fix:
  inlined CALL gens Pure — body carries effects). strain-3: the literal 207 pun's internal
  `dpkg -s` guard misclassifies MustRun due to an oracle-authoring verb-key gap (orthogonal;
  payoff case uses devnull-establish wrapper instead; flag arch2-pun-internal-query).
  Late addition post-self-crosscheck: EXPLICIT in-loop floor in `inline_disposition` (it
  runs before `disposition_for` like Members and could have bypassed the floor) + e2e
  `inline21-in-loop-call-floored` + unit pin. 7 new inline21-* cases + exec-detached-fn
  re-homed; 81 e2e ×2, 388 tests. Doc-deltas: analysis/CLAUDE.md detached-funcdef reality ×3
  (since applied per commit 5589bbe), inv-leaf-seam non-injectivity nuance (AstId-ward
  shared body, Step-level stays injective — applied to spike/CLAUDE.md). Hunt-list:
  back-map non-injectivity consumer audit (done by builder, all benign — re-attack),
  call-status-⊤ lost-fold-never-wrong-elision proof, assignment-leak scope edges
  (prefix-env must not leak), splice determinism, door-3×call chains (subshell-wrapped,
  piped-stdout-consumed), aggregate completeness (two-cell bodies, Members-precision ×
  inlined-call composition unbuilt), budget boundary AST-vs-CFG inversion, scanner
  completeness (`$@` in nested `$()`, dynamic redirect targets refused-safe).

- **219 — arch-4 cmdsub design note (design-FIRST, nothing built)** — rating: none (pure
  engine/design prose). Headline: current `$()` handling is structurally complete but
  SILENTLY degrading — parser lowers `$()` to real sub-ASTs (not a ⊤-trigger; only the
  for-list-word case ⊤-rejects); CFG marks bodies `expansion_internal` (effect-bearing
  non-leaves: they poison/establish but never become Steps); value plane collapses any
  `$()`-bearing word to `Recipe::Top` unconditionally (quoted or not; assignments make the
  var ⊤ downstream); and NO diagnostic fires on any of these paths — a live find-3
  (no-silent-phantoms) violation, the one common construct degrading silently. q-2 floor
  (recommended build-now): 2-3 honest diagnostics (`dq-cmdsub-operand-top` generic-⊤ form at
  `effect.rs::command_effect` which already has the diags sink; `dq-cmdsub-inner-nonleaf`
  disclosure at classify's leaf-drop; `dq-site-unresolvable` cli stderr loop over
  probe.unresolvable) — pure-additive, zero golden churn, unit-test-only. q-3 prize (the
  Query-shaped `$()`, `v=$(getent group docker)` value-by-probe): real but a multi-wave
  future-round keystone, NOT this round — needs (1) probe-exec gate as hard prereq (fm-1:
  fixture-authored captures = the 19I §3 trap with teeth), (2) a value-carriage wire
  surviving multi-line/binary stdout (token-shaped `stdout=` key breaks; options
  base64-with-probe-dependency / length-prefix-parser-rearch / refuse-non-text floor —
  lean refuse-non-text single-line first slice covering `$(hostname)`-class), (3) classify
  promotion of vouched inner commands from expansion_internal to a new CaptureResolvable
  leaf class, (4) a value-plane ← probe-record BACK-EDGE against current pipeline order
  (second post-probe value pass or fold-time substitution), (5) value-provenance tracking
  for inv-probe-sourced-values (ValueOf::Literal is provenance-erased today; lean
  route-through-site-keyed-class over tagging every ValueOf). Seams verdict: site-keying +
  `OutClaim`/`stdout=`/`ProbeSiteKind` BEND (deliberately reserved for exactly this);
  record grammar + non-leaf→leaf + the back-edge RESIST. Four human forks surfaced, argued
  both ways, unsettled: fork-capture-claim-type (NEW claim-type à la door-2 counterfactual
  vs ordinary fourth-channel observation — the load-bearing one), fork-cmdsub-top-cause
  (generic vs cause-tagged ⊤), fork-capture-probe-body (inner-command-verbatim vs declared
  probe body), fork-capture-wire (base64 vs hex vs refuse-non-text). Recommendation: q-2
  only this round; q-3 designed here, deferred.

- **21B — arch-6 H2SaLS coverage dashboard build** — rating: low (operates over the
  hardening-guide corpus and names its commands — ufw/rkhunter/lynis/visudo etc. — in
  census tables; itself plain engineering prose). New crate `crates/coverage` (binary
  `dorc-coverage` + `tools/coverage.sh` wrapper, never wired as a failing gate): per-site
  rows {analyzable tri-state, oracled?, probed-converged?, disposition+DOOR}, door enum
  {fold, dead-invariant, replace-converged, query-substituted, guard-transform(0, unbuilt),
  static-declared(0, unbuilt), runs(+dominant BlockReason), unattributed} — door-3
  discriminated from plain replace-converged by consumed-set-contains-StatusInvariant
  checked BEFORE via-dispatch (subtlest call, validated against door3-or-true-elides);
  query-substituted its own column (counted in full-elision fraction, reported separately);
  dq-2 rung split {guard-readable, needs-declaration, not-applicable}; criticality = line-
  count stand-in behind `weights::from_line_scores` adapter seam for the 1A matrix; pure
  kernel, BTreeMaps, every match ends `_ => Unattributed` with `#[expect(unreachable_
  patterns)]` so new engine variants scream at compile time (priority-tension vs -D
  warnings resolved toward the charter). HEADLINE: H2SaLS rollup = 195 sites, 0.0% elision,
  IDENTICAL with no-probe and all-converged-probe — and the 0% decomposes into four
  correct, separately-ownable causes: (1) only 4/195 sites oracled (book deliberately
  un-annotated; 11 seeds vs 33 distinct commands); (2) the 3 oracled installs are
  written-upstream — `apt-get update` intentionally unmodeled ⇒ Opaque ⇒ poisons every
  downstream install; (3) the lone converged establish (`groupadd`) blocks on
  consumed-⊤-status under `set -eu` — the ONE needs-declaration site, the doors program's
  most pointed signal; (4) all 5 `|| true` door-3 sites wrap UN-ORACLED tools (rkhunter/
  wget/lynis…) ⇒ door-3 is "free" only once the wrapped tool is oracled — flagged the
  highest-leverage corpus lesson. Unattributed = 0 on H2SaLS AND all 75 e2e (corpus-wide:
  fold=4, dead-inv=1, replace-conv=37, query-subst=6, runs=128). Cross-validated against
  the cli byte-for-byte (0 elisions both; headline case exact per-door match). Census
  spot-verify: 6 commands EXACT; getent/sed divergences are granularity (cmdsub-internal
  non-leaves; ⊤-rejected while-read-EOF loops — loud, pinned) not census errors. Seam
  wishlist: seam-1 public per-site ⊤-reason readout (highest value, c1 can't split
  genuine-⊤ from pure-builtin); seam-2 public facts_from_sites equivalent (dashboard
  reconstructs Query rc from verdict — errs safe, Unknown⇒Top); seam-3 refusal-reason
  enum. Top residual lie-risk FLAGGED: render-refusal demotion unread (a heredoc-bearing
  converged Replace counts elided but runs verbatim — over-count; no corpus case exercises
  it yet; fix = consult `render_refusal_diagnostics`, demote to runs(render-refusal)).

- **21D — arch-7 seeded-random differential harness (cm-1 local approximation)** — rating:
  none (pure engine prose). `hostsim/src/differential.rs` (~1.9k lines) + example CLI:
  seeded generator → drives the REAL dorc binary + dash per trial (probe artifact executed
  under host-state-aware probe-mocks self-reports correctly-keyed `site N` records — no
  LeafId guessing; `--debug-argv` dispositions ARE the license ledger), judge asserts
  apply-trace == bare-trace MODULO licensed elisions with under-execute as the disaster
  class; deterministic (u64 seed → LCG, high-32-bits after find-6), scratch-dir only, no
  new prod deps. Mock taxonomy is the anti-drift keystone: mutator apply-mocks host-state-
  INDEPENDENT (trace diffs can only come from elision), query-guard mocks consult the same
  converged set as their probe-mocks. Authoritative sweep: 500/500 clean + 100/100
  (seeds 5000+), 0 findings, ~0.27s/trial; NOT vacuous (59/100 seeds exercise real elision;
  judge proven able to scream via planted-under-execute pin). All six in-flight findings
  were HARNESS bugs, engine held every time (consistent with 20A's degrade-spine
  diagnosis): find-1 generator missing `oracle_effect … query` marker; find-2 doubled
  `__check` suffix; find-3 judge's gate-5-style check false-positive on short-circuited
  guards (removed — redundant with the differential itself); find-4 cmdsub-in-argument
  wrongly classed hard-⊤ (only `eval` + break-in-loop havoc downstream — engine verified
  correctly eliding past `echo "$(echo sub)"`); find-5 TOP-wildcard loop-member matching —
  judge now wildcard-matches AND independently cross-checks the removed member's host-state
  convergence so a wildcard can't mask eliding a diverged member (the adversarial guard,
  both directions pinned); find-6 LCG low-bit/seed-parity correlation silently capping
  StraightLine/ForLoop elision at 0 under always-on `set -e` — exactly the
  silent-cap-reads-as-coverage trap, fix surfaced find-5 immediately. Coverage map honest:
  CAN emit straight-line/if/||/&&-guards/literal-for/`|| true`/set-e-coin/⊤-controls;
  CANNOT emit sourcing (biggest gap), inlining, case, while/until, heredocs, multi-operand,
  `$()`-Query, consumed-output, mixed-state same-entity (risk-2 `.any` leniency — tighten
  to `.all` if added). Measures SOUNDNESS over bounded shape-space, complementary to
  arch-6's coverage number. --WONDER flagged: seed the generator from real e2e books
  (mutate) vs hand-grow constructs.

- **21E — arch-1 P1 fix: adjacent multi-line span-edit orphan** — rating: none (pure engine
  prose). The hostile crosscheck of `140c303` found the P1 that 214 hunt-7 pre-flagged:
  two adjacent multi-line edits (second leaf STARTS on the first's closing line) — edit A's
  line-keyed splice consumed through line 1 and the walk jumped past it, orphaning edit B
  ⇒ rendered `true; apt-get install -y "c<LF>d"`-corrupted artifact: dash-n CLEAN BY QUOTE
  COINCIDENCE but the second converged install RAN with a corrupted operand (silent
  over-execute), and an odd-embedded-quote variant hard-broke dash -n (comment appended
  inside an open double-quote — `comment_safe` only knew trailing-`\` and `<<`). Three
  interacting emit-side defects (orphan / half-splice / comment-in-quote); the edit SET was
  always correct (normalise_edits untouched — spans genuinely disjoint; pure emission bug).
  Fix f-1: region-GROUPING — transitive closure of line-overlapping edits, one region per
  group, all members spliced right-to-left, one rendered line, provenance comment carries
  every member's original; `debug_assert_eq!(spliced_count, edits.len())` tripwire. Fix f-2
  defense-in-depth: `region_ends_in_quote` POSIX quote-state machine (single/double/
  backslash; no expansion nesting — an unclosed `$(` ⊤-rejects upstream); refuse ⇒ DROP THE
  COMMENT NEVER THE EDIT (disclosure stays in the OOB verdict lane; res-3 flags the honest
  artifact-text disclosure loss, future fix = own-line leading comment, deferred as it
  churns every golden). 8 new unit/integration pins + e2e `render21-adjacent-multiline-
  elides` (76 cases); gates green ×2; dispositions byte-identical (render-emission-only
  fix). Residuals verified: three-deep chains safe (O(n) exhaustive sweep), done-line
  scaffolding-prefix composition safe; --WONDER left open: keyword strictly interior to a
  multi-line consumed span (couldn't construct — quoted text can't be a token — not proven
  unreachable); crosscheck should probe rendered line ending inside `"$(…`.

- **21F — r1A direction report (imp-1..6) adjudicated into round-21** — rating: low
  (references the hardening corpus's helper idioms; plain rulings prose). imp-1/y-1 (the
  big one, a candidate P1-AT-HEAD): H2SaLS helpers' `printf … >> file` edits are invisible
  (printf blessed-pure, Redir⇒Pure) but MASKED today by Opaque helper-call poison; arch-2's
  inlining lifts the poison ⇒ downstream grep-guard Queries of just-edited files compute
  valid ⇒ stale-guard QueryGuard mints — inlining-without-redirect-cells manufactures
  wrong-elisions. Adjudication: a-1 the tc-M2 body-write-redirect refusal fences the
  inlining-armed form IF refusal lands effect-Opaque (mandatory arch-2-crosscheck
  construction: helper `printf >> "$CONF"` + downstream grep-guard ⇒ no mint); a-2
  ~SUSPECT the BOOK-LEVEL form needs no inlining at all (`set -e; printf 'x' >> f;
  grep -q x f || mutator` — st-3 exempts pure-builtins from invalidation, so a
  file/confline-keyed oracle would fold the guard stale; nothing in the e2e corpus can hit
  it because no such oracle is loaded — r1A's confline seed is exactly the arming
  vocabulary). Task y-1 created: resolve `RedirTarget::Word` through the value plane, gen
  per-path file cells, poison-correctness only, sequenced right after arch-2 (same
  effect.rs surface). imp-2: arch-5's ENTRY GATE — `self_reach_holds` may be implemented
  global-pristine vs 20S §3.1's documented cell-family scope (impl-stricter-than-design;
  safe but value-killing — L2's greens may owe their pass to pristine fixture preambles);
  verify code, surface evidence + cell-scoped lean to human, re-run 20S adversarial pins
  under whatever scoping lands BEFORE any list-rewrite build. imp-3: dashboard harvest
  aids — sea-of-red is CORRECT (grade honesty not green); two seed pieces deliberately
  inert (crond's commented-out test resolver; test-keyed cells dead behind blessed-pure)
  must not count as coverage; 1AA matrix doubles as hand-derived golden; verify
  "poisoned-by-upstream-refusal" is distinctly attributable. imp-4: H2SaLS's 11 `$()`
  sites partition 4 mktemp / 3 Query-shaped (incl. L38 `$(id -u)` — the poison wall's
  ORIGIN, arch-4's prize hits the highest-leverage site) / 2 stay-⊤ / 1 static — general
  substitution-folding not needed for this class. imp-5 (dq-ledger): every honest refusal
  in a loaded oracle costs downstream ambience ⇒ loading MORE oracles can REGRESS the
  number until a declare-without-verify cell exists (r1A h-3) — bounds the ceiling claim
  from below; surface to human with dashboard concreteness. imp-6 smalls (L685-695
  if-guarded restarts as real guard-class poles; y-7↔y-2 weld dissolves for inlined
  bodies; h-2 provider-collision watch; stale cfg.rs comments). fb-7 adopted: reconcile
  by source not by vote (neutral was literally-correct-but-wrong-by-omission once;
  headline find was adversarial-only — every applied correction re-verified against the
  cited source by the reconciler).

- **21G — direction batch-2: spike-4 shape, error-tooling intent, q-2 re-spec** — rating:
  none (rulings/direction prose). §1 spike-4 expectation (human): assuming round-21 closes
  with "real stuff really elided, sound modulo Perfect Oracle Competence," spike-4 forces
  that competence-bar's permeability DOWN to meet authorship (re-eliminate the cliff) via
  detailed provenance + error-handling; round-21 seed artifacts enumerated (disclosure
  floor, provenance comments, refusal diagnostics, site N.M, dashboard why-not, dq-2
  split). §2 error-tooling intent (human, self-labeled vague; recorded not designed):
  layer-1 dislocated error-message index (slug → catalog) + mechanical CI gate that every
  give-up path carries a slugged ID with catalog entry; layer-2 same posture for
  provenance (user-surface text must be provenance-typed; every transform extends the
  chain). Corpus reconciliation: layer-1 = [A-pottier-reachability-2016] (Menhir
  .messages-style completeness gate — the unbuilt piece); the Carrier/never-throw half =
  [A-bour-merlin-2018], already welded as inv-no-throw; layer-2 = owed-prov derivation-DAG
  + Racket transplantable-metadata as type-enforcement. Orchestrator observation: most of
  the self-flow-analyzer ambition approximates via make-bad-states-unrepresentable
  (DiagCode-only diagnostics; provenance-only user-text newtype), shrinking the mechanical
  residue to a catalog-completeness test + a no-raw-text lint; Pottier path-enumeration
  stays future. §3 in-round change: the queued q-2 cmdsub-diagnostics slice (lands
  post-arch-2 with y-1) gains rq-1 single-catalog/template-separated codes, rq-2
  completeness unit test (embryonic Pottier gate), rq-3 no free-text-only emissions. §4
  fork-leans supplied (reserve not weld): prefer cause-tagged-⊤ where cost-comparable;
  provenance via site-keyed-class; dashboard seam-1 same family, candidate post-arch-2
  task. §5 round-close obligation added: wrap report carries a spike-4 inventory of
  round-11 primitives (welded / embryonic / absent) as the r22 seeding document.

- **21H — y-1 redirect-write cells + q-2 diagnostics floor build** — rating: none (pure
  engine prose). Two pre-spelled slices, built uncommitted-for-orchestrator (the
  orchestrator committed as `0c48e07`). q-2: diagnostic CATALOG born in NEW `core/src/diag.rs`
  (per code: `DiagCode` const + template() arm + structured-param constructor + CATALOG
  registry; completeness test = the embryonic Pottier gate; rq-1/2/3 honored; existing
  scattered codes deliberately NOT retrofitted) wired to three formerly-silent `$()`→⊤
  sites as Note-severity disclosures (`dq-cmdsub-operand-top` in command_effect,
  `dq-cmdsub-inner-nonleaf` at classify's leaf-drop gated on non-Pure,
  `dq-site-unresolvable` cli-edge stderr loop) — gate-3 keys only on `error[` so Notes are
  invisible to it, zero golden churn (90 cases byte-identical, +3 new = 93). y-1:
  WRITE-shaped redirects (`>`/`>>`) to resolved non-devnull targets now gen per-path
  `file:<path>#written` cells via a `redir_pass` post-solve pass (Members/inline
  precedent; resolve_recipe against incoming env; expansion-hazard ⇒ ⊤ + `dq-redir-target-
  top`; devnull + fd-dups exempt; `<>` already parser-⊤-rejects) ⇒ writer ⇒ st-3
  invalidation ⇒ imp-1's stale-guard hole CLOSED — regression pin traced both ways
  (with the `printf >> nginx.conf`: guard withheld, install runs live; without: folds
  dead). Kind-vocabulary flagged for human ratification (§5): `file#written` distinct from
  `confline#present` — write does NOT discharge content-read, coordination only via
  pristine-prefix invalidation, intended gen-and-poison-nothing-licenses (doubly: Redir
  never a plan leaf). s-2 (dashboard seam-1 accessor) assessed and SKIPPED per the
  ≤20-lines gate — right future shape is a deliberate classify return-type change.
  Exclusion-check ran all four directions (notably: admin gets poison-correctness free,
  redirect IS the sh-spelled signal; no oracle dependency so unreliability can't weaken
  it). Hunt-list: var-resolved target pinned (hunt-1 says NOW PINNED; §9 still ~SUSPECTs a
  missing pin — minor internal inconsistency, verify which); gate-2 absolute-path
  interaction flagged for future exec cases; q-2 disclosure volume on large books
  (~SUSPECT acceptable, spike-4 wants more disclosure); span-provenance gap (None spans,
  invisible while report() ignores spans).

- **220 — research: value-provenance plane engineering (round-22 seeding)** — rating: low
  (dense academic taint/IFC citations — TaintDroid, FlowDroid, declassification, Flume,
  CHERI — all in defensive receipts-plane-design context; candidate contributor if the
  harness is vocabulary-sensitive, but content is benign). NOTE: file is 220 but its H1
  mis-titles itself "# 21H" — slug collision with the y-1/q-2 build note; flag for fixup.
  Serves 21G §2 layer-2; extends plans/111 without re-covering it. r-1 representation
  consensus (+SURE, five system families): constant-size per-value annotation pointing
  into an engine-owned append-only shared side-structure, richness recovered by lazy
  backward search (Soufflé (rule,height) 2 words/tuple 1.27×/1.45×; rustc 8-byte Span +
  interner, 4-byte was SLOWER — over-compression backfires, no-size-cliff rule; ProvSQL
  shared circuits 2–3×; Smoke capture-in-hot-loop or 10×+; Titian in-engine <1.3× vs Newt
  external-store 86× fatal; TaintDroid 32-bit set + boundary coarsening; Salsa durability
  tier-summary). r-2 UX habits: minimal witness first, fragment-and-expand never
  whole-graph, evidence-per-edge in the user's own artifact (nix --precise), category PLUS
  delta (Bazel/ninja failures), suppression heuristics are the shipped artifact (Clang SA
  10→170 note explosion); "why not" resists automation everywhere BUT Dorc's refusals are
  positive events (~SUSPECT a structural advantage). r-3 formalism mapping: flat lineage
  (k-capped origin set) for ⊤-blame; ONE stored minimal witness per license (CHERI-
  intentionality operationalized); retraction via re-derivation not per-value DNF
  (over-invalidation is kFAIL-perform-safe); full how-semiring answers no Dorc question —
  avoid; where-provenance = the existing Span plane, keep planes separate. r-4 drowning
  predictors: external stores, host-fork packaging (the Trio/Perm/Orion graveyard vs
  extension/library survivors), capture-without-consumer (PASS), unbounded precision
  (DroidSafe fails real apps; the shippers have hard caps), raw-dump UX. r-5 one-way rule
  prior art: IFC declassification (permits few/explicit/owned — oracle-claims are the
  declassifiers; conservativity = no-claims ⇒ never-elide, current behavior), Flume
  capability-held permits (License constructible ONLY from oracle-claim value), CHERI as
  the CONTRAST (provenance-as-authority demands a sound chain Dorc doesn't promise ⇒
  receipts stay refuse/explain-side), plus the ERASABILITY GATE (strip receipts, re-run,
  assert verdict-identical — CI from first commit). §6 r22 build order: ProvId arena +
  Top(cause) reshape → erasability gate → ONE consumer end-to-end (dashboard why-not as
  minimal-witness) before widening capture. Unreachable sources flagged for human:
  ACM Queue/CACM 403'd; Zdancewic–Myers primary unread.

- **221 — research: execution substrates & independent adjudicators (round-22 seeding)** —
  rating: none (DST/substrate research; ptrace/syscall mechanics only). NOTE: H1
  mis-titles itself "notes/21I" (drafted under provisional slug, renamed 22x — same
  pattern as 220); also written BEFORE 21D existed (it says 21D "was never written" —
  superseded by the actual arch-7 build note). Feeds arch-7 + the standing differential
  epistemology (real dash adjudicates; self-built simulated dash = correlated error).
  concl-r1 Smoosh: real, good, dormant (only shell with zero VSC-PCTS2016 failures; found
  bugs in dash/yash/the POSIX test suite/the spec itself; drop-in shell column; OCaml/Lem
  bitrot risk, Docker-pin; CRITICAL nuance: parses via libdash so it's an independent
  SEMANTICS adjudicator but dash-CORRELATED on parsing; its divergence catalog = audit
  list for our ⊤/refusal zones). concl-r2 Hermit/DetTrace/rr: ptrace-determinization
  proven on shell-shaped workloads (DetTrace: 12,130 Debian builds bit-reproducible) and
  abandoned as products (Hermit maintenance-mode, bot-only commits; needs PMU counters ⇒
  WSL2 likely out); Hermit's `--chaos --sched-seed=N` is the only found seeded-
  interleaving rail; josnyder: full userspace determinism provably unreachable (rdrand
  untrappable ⇒ Antithesis is a hypervisor); for OUR books a namespaces/env-freeze/1-cpu
  tier captures most determinism ~free. concl-r3 FDB/Antithesis lessons: seam
  architecture (real code above, simulated I/O below, model-fidelity hedged with hardware
  tier); dependency lesson (outside-the-boundary = untested ⇒ FDB deleted ZooKeeper —
  bring real dash INSIDE the boundary, never re-model it); BUGGIFY lesson (determinism
  alone unproductive — white-box sabotage points + knob randomization + recover-and-
  assert phase; WarpStream's 233s-vs-10k-CI-hours race). concl-r4 wasm/WASIX: dash-on-
  wasm exists but is a DIFFERENT ARTIFACT than target-host dash ⇒ contaminated
  adjudicator, avoid. concl-r5 Oils spec corpus = the richest reusable asset (~225
  Apache-2.0 test files with hand-ADJUDICATED per-shell divergence verdicts PASS/OK/N-I/
  BUG; mine the corpus, don't adopt the runner); ShellCheck never attempted evaluation
  semantics (corroborates our ⊤-reject posture); CoLiS interpreter 8/161 (not ring
  material; Morbig stays attractive as the only independent PARSE-layer referee — parser
  is our highest-risk surface); NO open-source POSIX-sh conformance suite exists;
  busybox-ash is dash-correlated (Almquist lineage) — prefer yash/mksh columns. Ranked
  table: adopt-r22 = harness determinism rail (env-freeze+unshare+cpu-pin, document the
  residual lax-set) + Oils corpus import (BUG-dash-annotated cases priority); evaluate =
  Smoosh column (Docker-pinned, disagreement = spec-ambiguity evidence not engine-bug),
  Hermit 1-day smoke spike (needs real Linux x86_64 — the rackmounted PC; h-2),
  Morbig parse-referee; defer VSC license; avoid wasm rail/Antithesis/hand-simulated
  dash. Human-pending: h-1 CoLiS STTT 2022 full text bot-walled+paywalled (HAL
  hal-03737886 — one-minute human browser fetch wanted).

- **222 — research: author-declared behavioral claims in real ecosystems (round-22
  seeding)** — rating: low (benign ops/build-systems literature; "cache poisoning"
  vocabulary recurs in Bazel/Nix context). H1 self-titles "21J" (same provisional-slug
  renumbering as 220/221). Serves door-2/door-4/dq-errexit. Conclusions: c-1 every
  ecosystem with author-declared dry-run/idempotence claims accumulated a
  lying-declaration bug class — they differ in WHERE the lie surfaces (silent wrong plan:
  Ansible check-mode; loud party-attributed error: Terraform apply cross-check, Nix FOD
  mismatch; symptom-far-from-cause: Bazel cache poisoning). c-2 the canonical failure is
  COMPOSITIONAL (Chef's "assumptions problem": forecasts conditioned on unexecuted prior
  mutations; Chef's vendor recommended against its own why-run and floated removal) —
  Dorc's probe-grounded elision dodges most of it EXCEPT door-2's consumed-stdout cell,
  which re-imports it (p-1: restrict door-2's sanctioned channels to rc + unconsumed-
  stdout for the spike; apt's "already newest version" text is locale/version-rot-prone,
  the most rot-prone claim-shape surveyed). c-3 what survived: predictions computed from
  MEASURED PRESENT (Puppet noop reads state; Terraform plan-after-refresh) or verified
  ex-ante in a harness (Hummer: 92/300 real Chef cookbooks non-idempotent ≈31% base rate
  for author-vouched idempotence) — door-4 is in the surviving family (T-1 verify-eagerly,
  self-correcting where Terraform can only error); door-2 is in the dying family (T-4
  never-verify) unless fenced by the disclosure floor + door-1-on-wrappers analytic
  pre-validation (name them load-bearing). c-4 the dry-run lane running author-shipped
  read-side code is UNIVERSAL accepted practice (Puppet onlyif/unless MUST run under
  noop) — dq-errexit-3's precedent is total: same-trust-extended, not a line crossed
  (~SUSPECT, human's call), conditional on disclosure + blame-routing-to-oracle (m-2
  Terraform-style "this is a bug in the provider" template). c-5 weld-5 convergently
  evolved everywhere (DeHaan: over-predict change rather than risk one). c-6 attribution
  quality is THE trust differentiator (Terraform names-the-liar best; Bazel unattributed
  cache-poisoning worst). c-7 contracts tighten by phased enforcement (legacy tolerated-
  WARN breadcrumbs) — the door-4 default-OFF flag matches precedent exactly. c-8
  trust-psych: false-alarm-prone automation damages compliance+reliance; one popular
  lying oracle ⇒ correlated identical failures fleet-wide ⇒ the reputational cliff (p-3:
  dashboard attribution must aggregate per-oracle across hosts so one rot event reads as
  one cause). Mechanisms to steal: m-1 tri-level support declarations with sh-spelled
  partial-conditions (Ansible attributes); m-3 checksum-pin declarations to oracle
  source-hash with validCheckSum-style re-vouch (FOD lesson: re-verifiers must not
  inherit the suppression); m-4 author-side Hummer harness (run mutator twice, diff
  declared-vs-observed); m-5 sampled door-4-under-door-2 live cross-check (Trustix
  shape, c-9); m-6 render the counterfactual TEXT in the plan comment (Chef's one good
  UX idea). p-4 the simulation lane WILL be asked to mutate (Ansible/Puppet both grew
  escape hatches) — keep refusing, answer = narrower-scope-sooner. p-5 read-side at
  fleet scale is its own hazard class (Chef's nightly systemd-lockup outage from a
  "non-mutating" cron why-run) — future per-kind probe concurrency/jitter knob. The
  deliberate inversion recorded: prior art degrades by not-predicting (silent gap, sim
  lane); Dorc degrades by executing (loud but safe, apply lane) — an unelided site is a
  missing declaration, never a wrong one.

- **21K — direction batch 3 (PROVISIONAL, human reserves reversal)** — rating: none
  (short direction ledger, benign). d-1 r22 lean = errors+provenance INCLUDING the
  voluminous-durable idea: a derivation-dump mode per run (postmortem + DST golden-TRACE
  fixtures, tiered — critical-tier pins traces, rest pins verdicts; `why` becomes a query
  over the dump — one producer, many lenses). d-2 light OTel integration early
  (controller→host trace-propagation; verdict lane as carrier candidate;
  import-ideas-not-machinery caveat stands). d-3 first-contact reframed: human has NO
  coherent existing book-set and his ops work is gated BEHIND this project; his hybrid
  probe = ~3 real todo items + cheap AI agents + cheap VPS + slopped oracles/books as a
  FEEL-test, attempt-cheaply-abandon-freely; orchestrator to propose pre-registered 3-5
  design questions + hard abandon criterion (converts whack-a-mole into a survey with an
  exit); the YOLO gate looms (strict-vs-YOLO-by-default; homelabber adoption is the
  game). d-4 SUBAGENT COMMIT POLICY CHANGED (human feedback): builders commit granularly
  (detached worktree bases, orchestrator harvests by cherry-pick with gates before each
  pick; main-tree builders may commit granularly, orchestrator verifies after) —
  no-commit briefs compress evidence. d-5 r-5 scoped-mutation stays deferred, rationale
  expanded (sanctions declared mutation-boundaries on the kFAIL-withhold side — trust-
  catastrophic phase, engine can never check the boundary, 222's c-2/c-3 evidence, zero
  measured population). d-6 arch-5 entry gate VERIFIED in code: `self_reach_holds`
  (effect.rs ~620) is GLOBAL-pristine — impl-matching-its-own-doc, ambiguity upstream in
  20S §3.1's wording; the authorized cell-family re-scope proceeds under EXPANDED
  obligations (deep-preamble pole, sibling-writer re-runs, two-leaf floor pin,
  value-plane pole, multi-effect hazard note, dedicated hostile pass — load-bearing, not
  ceremonial, since 20T validated the strict form).

- **21Y — conductor handoff (the prime suspect — READ CLEAN)** — rating: low (names the
  safety-gate situation explicitly and the H2SaLS defensive context; content is a
  well-constructed defensive handoff, nothing hot). Written by the "lower-capability
  caretaker" mid-#13 for exactly this resumption. Its own diagnosis, verbatim-in-
  substance: the round-21 orchestrator (or a subagent) hit the cybersecurity safety-gate;
  the H2SaLS corpus is a DEFENSIVE server-hardening rewrite; "if a sec-gate fires, the
  trigger is most likely the corpus's hardening vocabulary in reasoning, not a real
  security task. Keep the analyzer (not the workload's domain) as the subject in all
  reasoning and subagent briefs." Round state at handoff (HEAD 0c48e07): everything
  through y-1/q-2 landed (matches my git reconciliation exactly). IN-FLIGHT DRAINED: the
  uncommitted 5-file +329 diff IS task #13 (wave-2 fix slice) — dashboard heredoc
  over-count fix (wave-2 P2/m-6 = 21B's flagged residual), depth-2 transitive inlining
  fixes (P2/m-8: depth-2 was broken-but-SAFE — positional non-thread ⇒ runs, record
  double-count redundant — REFUTING 216 hunt-1's self-claim; correction belongs in note
  217, never edit 216), plus two new undecided flags (tc-fix3-severity Note-vs-warning;
  detached-funcdef-copy asymmetry). Agent-claimed gates green ×2 — RE-VERIFY before
  commit. Full #13 report + diff snapshot at %TEMP%/dorc-r21/handoff/ ("the resumer's
  first read after this file"). Wave-2 verdicts: arch-2 single-level SOUND-under-attack
  (process evidence); imp-1 stale-guard hole confirmed live → CLOSED by y-1; multiline
  exec flake = stale shared-target binary, closed. Remaining queue: harvest #13 (gates
  ×2, commit) → note 217 (wave-1+2 crosscheck reconciliation) → task #7 arch-5 under 21K
  d-6 expanded obligations → task #12 harness pass (newline-safe mock-log, EXIT_RC
  marker, dual-rail corpus harness) → task #5 door-4/2+precedence LAST behind default-
  OFF flag → round-close (20K-analogue report, plans/21Z living spike-4 inventory,
  seeding feedback). Standing rulings restated (never-vouch hard limit; dq-errexit
  forks are the human's; never harness isolation:worktree — wrong-base 5×; explicit-path
  worktrees under %TEMP%/dorc-r21/).

## Phase 2: %TEMP% handoff artifacts (the last material before the final crash — human flags HIGH suspicion)

- **status.txt (1:49:29 AM)** — rating: none (it is literally a `git status --short`
  snapshot, ten lines). What it led to: a TIMELINE reconstruction, not a contradiction.
  It lists only FOUR modified crate files — `effect.rs` is absent — while 21Y and the
  current working tree both say five. The mtimes order it: status.txt at 1:49 was a
  mid-landing snapshot taken before #13's effect.rs edit arrived; 13-report.md (2:00:32)
  and the diff snapshot (2:00:35) postdate it by 11 minutes and include all five. So
  status.txt is an intermediate, stale-by-eleven-minutes record — internally consistent
  with the rest of the handoff, useless as a file-list, and evidence the caretaker was
  snapshotting WHILE #13 landed rather than after. The authoritative artifact is the
  diff snapshot, to be checked against the live working tree by patch-id below.

- **13-report.md (2:00:32 AM)** — rating: none (engine prose; agent completion report,
  no instruction-shaped content beyond the orchestrator queue already known from 21Y).
  What it led to: the four-fix decomposition of the dirty diff, each cross-checking
  cleanly against an already-known flag — fix-1 implements 21B hunt-1's prescribed
  remedy VERBATIM (`BlockReason::RenderRefusal`, `render_refused_leaves` consulting
  `Plan::render_refusal_diagnostics`, demotion BEFORE elision-door dispatch, plus the
  unit pin 21B said was missing); fix-2/fix-3 are the two halves of wave-2 m-8
  (record double-count → `flattened_inner` BTreeSet dedupe in `splice_funcdef_body`;
  positional non-thread → LOUD refusal `dq-depth-2-positional-unthreaded`, threading
  deliberately NOT built, literal-arg depth-2 still inlines — broken-but-safe becomes
  refused-loudly-safe); fix-4 is test-only pins in effect.rs for the `: > f` / bare
  `> f` shapes the orchestrator hand-verified post-y-1. The effect.rs-is-test-only
  detail also retro-explains status.txt's four-file list (tests landed last, after
  1:49). Two standing flags confirmed mine-to-resolve with human override: tc-fix3-
  severity (Note because 21H's catalog carries a Note-only invariant TEST — widening it
  means per-code declared severity, the ~SUSPECT right shape but round-22-sized; lean:
  keep Note this round, record the sibling-warning inconsistency in 217) and the
  detached-copy asymmetry (harmless non-leaf island; lower_funcdef untouched
  judged-risky — agree, 217 material). Follow-ups match 21Y's queue exactly: gates ×2,
  commit per d-4, 217 carries the 216-hunt-1 depth-2 refutation.

- **inflight-13-uncommitted.diff (2:00:35 AM — the last artifact before the crash)** —
  rating: none (pure Rust diff + doc comments; nothing instruction-shaped). What it led
  to — a line-level review against 13-report's claims, all four fixes verified present
  and shaped as described, with three reviewer observations for 217/crosscheck: (1)
  fix-3's `word_parts_reference_positional` matches `Param{name}` digits + `$#` and
  recurses DoubleQuoted, but NOT `ParamComplex` — so a nested call spelled `b "${1}"`
  (brace form) would slip the refusal and fall back to the OLD silent-but-safe MustRun
  (body ⊤ ⇒ runs). Safe direction, but the disclosure is incomplete for that spelling —
  217 nit, not a blocker. (2) fix-2's dedupe correctness rests on arena ordering ("inner
  CALL precedes its body leaves; accumulate-as-you-go covers later direct hits") — +SURE
  for depth ≤2 given the splice lowers bodies after calls and the scan ascends; depth-3
  is budget-refused so the ordering argument never extends. (3) fix-1's span-keyed
  bridge (refusal-diagnostic span → step AstId span → LeafId, defensive-drop on miss)
  is sound under inv-leaf-seam span-disjointness; the `render_refused` override sits
  FIRST in `attribute_door`, before every elision arm — exactly 21B's prescribed
  demotion. File/line claims in 13-report all reconcile (minor line drift only); the
  +329/−2 stat matches; effect.rs is indeed test-only (two y-1 regression pins, `: > f`
  and bare `> f`). The new diag follows full catalog discipline (const + CATALOG entry +
  template + constructor + severity/fill tests — Note severity pinned in-test, which is
  what makes tc-fix3-severity a real invariant decision, not a one-liner). Conclusion:
  the snapshot matches the report; nothing in the last-written artifact looks
  crash-causal — it is ordinary engine code.

### Phase-2 verification results (2026-06-11, this session)

- Working tree ≡ snapshot: `git diff` is BYTE-IDENTICAL to `inflight-13-uncommitted.diff`
  — nothing moved since the crash; the reviewed diff is the live state.
- Gate set, re-verified by the resumer (verify-don't-relay): `cargo fmt --check` OK ·
  `clippy --workspace --all-targets -D warnings` clean (warm cache; fingerprint covers
  byte-identical sources) · `cargo test --workspace` 448 passed / 0 failed / 1
  pre-existing ignored · `sh e2e/run.sh` ×2 = 93/93 both runs, all six gates · `typos`
  clean. The #13 agent's green-claim is now VERIFIED, not relayed.
- %TEMP% verdict: all three handoff artifacts read clean under the sentinel discipline.
  Nothing in the last-written material looks crash-causal; the cumulative-vocabulary
  hypothesis stands unchallenged as the best explanation for the prior session deaths.
- Harvest: #13 committed; 21K + 21Y + this ledger committed at this quiesce.
  STALENESS-AUDIT.md left uncommitted (self-described temporary working report — human
  owns its lifecycle); quarantine seeds left untracked (human's call).
- Review-pass deliberately NOT run on #13's comments: the diff is harvested evidence
  from another agent, already line-reviewed; rewriting its comments would break
  snapshot≡tree equivalence and the d-4 evidence-preservation intent.

## SWEEP VERDICT (2026-06-11, this session)

Every document in the round-21 corpus plus the three round-22 research notes read clean
— ZERO harness incidents across 16 sweep documents + 12 pre-sweep documents. No single
"bomb" document exists in the swept set. Best current hypothesis (~SUSPECT, aligned with
21Y's own diagnosis and 212's pre-existing briefing rule): the prior crashes were
triggered by CUMULATIVE security-domain vocabulary in reasoning — most plausibly when a
context combined the H2SaLS hardening corpus material (un-swept here, in the sibling
worktree) with heavy security-flavored reasoning — or by the prior session's particular
single-block prompt; not by any one note's text. The un-swept residue, for a future
survivor: `Research/corpora/H2SaLS/*` (sibling worktree; elevated-by-topic, brief
defensive context first), the two quarantine seeds (1A0, 200), `%TEMP%/dorc-r21/handoff/`
artifacts (#13 report + diff), and the uncommitted working-tree diff itself. Discipline
that appears to work: 21Y's rule — keep the ANALYZER as the subject of every sentence;
treat the workload's domain as inert data; never enumerate/elaborate hardening content.

