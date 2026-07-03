# 236b — the 233 frame-problem crisis: failure-catalog of the guard-tier proposal, and the paths not taken

> Commissioned adversarial-alternatives pass (2026-07-01/02). Input scope, per the
> commissioning prompt: human-authored root docs + the corpus **through end of r22**
> (incl. `plans/230`, the r22-close seed) + `plans/233` only; all other 23x materials
> deliberately unread, clean-slate. AI-authored; process evidence, never proof.
> Confidence marks +SURE / ~SUSPECT / -GUESS / --WONDER throughout. Slugs:
> `236b-failN` (failures of 233's proposal), `236b-altN` (alternative designs),
> `236b-oqN` (open questions), `236b-fdN` (cross-cutting findings).

## §0 Orientation — what the product is, restated in one paragraph (so the value-loss is measurable)

Dorc's headline user story (DESIGN "Dorc's approach" #3, README): the repo-full-of-sh
equivalent of an Ansible play reduces, via probe-phase facts, to a *short plan* — "one
or two shell commands, directly narrowed to the state-mutators relevant to the user's
current goals." 233's own update names this the **attention-product** ("a
sanity-retaining tool in a messy ops world... as much as, or moreso than, a
performance-optimizer") and distinguishes it from the **performance-product** (fewer /
cheaper executions). Priorities (DESIGN): bounded correctness > low user-effort >
cross-network wallclock > invisibility. Death-scenario named in DESIGN §Sensitivities:
if realistic user habit defeats the analysis, Dorc degrades to "as much work as a
strict tool, for an unsound one" — worthless to nearly everyone.

## §1 The crisis (233 §0–§4), compressed

- The effect-model has a **frame problem**: an oracle declaring only its headline
  effects (`apt-get install ⇒ apt.Package:$pkg.installed`) implicitly *licenses* every
  cell it stays silent about; `apt-get install nginx` also writes
  `fs.Path:/etc/nginx/nginx.conf`, so a downstream `[ -f /etc/nginx/nginx.conf ]`
  guard stays "ambient" and wrongly elides. Under-execute = priority-1 violation.
- Per the 234-verified correction (233 end): the spike is "**safe floor, dangerous
  middle**" — a fully-unmodeled command poisons all (safe); a *modeled* command
  poisons only declared cells (unsound).
- The sound fix (approach-1, poison-undeclared) forces every oracle to enumerate the
  non-effect universe; unbounded, retroactively invalidated whenever any new kind
  enters the loaded world ("install `widget` and all elisions stop"), bites worst for
  trivial oracles. Approaches 2–4 trade enumeration for silence-mints-trust footguns.
- hard-1: mutation is fundamentally un-analyzable from sh; only oracle testimony
  reconciles it. hard-2: even perfect oracles rot on their own.

+SURE the crisis is real and correctly diagnosed *as stated*; see §3 for the premise
I think is wrong (`236b-fd1`).

## §2 What the end-of-233 proposal actually is

Ternary verdict `{elide, guard, run}`. Local oracle claims buy `guard` (the oracle's
read-only convergence predicate compiled **in-sequence** into the apply artifact,
`check || cmd`, 218a/door-4 mechanics re-based as the default middle verdict); only
"family-participation" (every retained upstream command vouched w.r.t. my state) buys
`elide`; silence means nothing (neither vouches nor collapses). The guard-license is
the **converged-vouch** (a fallible per-path judgment mark, claimed-tier, attributed,
site-local-only, fenced out of the fact-plane). Conceded upfront: "**give up the
attention-product to save the performance-product, wherever the world is
undescribed**"; a 100-command book with an unmodeled 3rd command yields a ~97-guard
artifact *forever*; the only path back is an open fork (human vouches for opaques)
that "may resolve to no."

## §3 Failure-catalog of that proposal

- **236b-fail1 · It concedes the headline product in the *majority* case.** +SURE.
  Real books ~always contain ≥1 early un-oracled command (curl|sh installers, vendor
  scripts, in-house tools) — the lazy-user segment the product targets *guarantees*
  low oracle coverage early (DESIGN §Sensitivities #2). So "wherever the world is
  undescribed" is not an edge regime; it is the default regime, indefinitely. The
  fast-path (full description) is exactly the "you did all the work anyway" death
  scenario. The proposal optimizes the corner and cedes the center.

- **236b-fail2 · It regresses to the pattern the product defines itself against.**
  +SURE. DESIGN's own problem statement: "it can be quite silly to actually *execute*
  a deep tree of these check-then-execute blocks... slow to interrogate serially and
  individually, over and over." Past a wall, the proposal re-instates precisely
  serial, per-site, every-apply check-then-execute — the *probe phase's* founding win
  (hoist checks, batch, parallelize, amortize across the plan/apply split) is quietly
  abandoned where the wall applies. In-sequence guards are structurally serial within
  the host stream; nothing amortizes; `an-equivalence-class` fleet-reuse (N·H→H) is
  dead for guard-tier sites.

- **236b-fail3 · It does not actually repair the headline unsoundness — it relocates
  it into an undesigned tier.** +SURE, and I consider this the damning one. The
  crisis is the *dangerous middle* (modeled-but-partial oracle under-poisons). The
  proposal's `elide` tier requires family-vouching — the very completeness machinery
  233 §1–§2 shows to be unaffordable — and that tier is explicitly deferred ("for
  when the vouch/elide tier gets designed"). So near-term the options are: (a) keep
  the spike's under-poisoning elide-license → the §0 unsoundness persists at
  elide-tier; or (b) empty the elide tier → everything is guard/run. (b) is what the
  text implies; then the analyzer's static-elision heart — "the thing" per DESIGN
  component-2 — is dead in practice, and Dorc ≈ an idempotence-guard inserter with a
  nice renderer. Either way the dilemma isn't dissolved; it's re-priced and deferred.

- **236b-fail4 · The incentive flywheel stalls (wish-E / kSILO / DESIGN sensitivity
  #1).** ~SUSPECT-leaning-SURE. Under guard-default, a *local* claim buys ≈ what a
  diligent admin writes by hand (`check || cmd`); the big payoff (elide) is gated
  behind the frame-problem-priced vouch tier nobody will reach. Effort→value goes
  flat exactly where the design needs it to compound; the community-library dynamic
  the tool "lives or dies on" (DESIGN §top) never spins up. Worse, machine-inserted
  guards make the admin's *own* defensive guards look redundant (anti-Half-B
  pressure, `kSILO` gradient in the bad direction).

- **236b-fail5 · Plan-fidelity and the approval contract degrade.** ~SUSPECT. The
  plan can no longer say what will happen — "expected: 1 change, 96 no-op" is hope,
  not the Terraform-grade "what you approve is what runs" DESIGN aims at. The guard
  artifact decides on-host, at run-time, with no per-site disclosure back-pressure
  (until after the fact). This is Ansible-check-mode-flavoured vagueness, the thing
  we positioned against.

- **236b-fail6 · Off-ramp/mindshare erosion at the realistic (fat) pole.** +SURE and
  conceded by 233 itself ("the mindshare cost is total" absent a uniform foldable
  render): realistic guards are shipped predict-body functions, so the artifact reads
  as machine-woven library calls threaded through your book. Runnable-without-Dorc
  survives; readable/ownable-without-Dorc (the actual off-ramp value, kLANG) erodes.

- **236b-fail7 · New mid-mutation failure class in the user's critical path.**
  ~SUSPECT. Guards run claimed-tier oracle code inside the book's shell env (233
  carries the 218a env-hazards). Probe-phase failures were pre-flight, isolated,
  reported; a hung/slow/stderr-noisy guard now fails *mid-apply, between mutations*.
  The `||`-form fixes errexit only; it doesn't fix hangs, latency, or attribution.

- **236b-fail8 · The check-tax compounds in loops, hot paths, and — worst — the
  steady state.** -GUESS on sizing, +SURE on shape: `for x in …; do guarded; done`
  pays per-iteration; 97 guards × 50–200ms serial is 5–20s per host per apply. And
  note *when* this bill lands: on a fully-converged host (the most common re-run —
  the tight develop/debug loop DESIGN's problem-statement centers), the apply IS the
  guard-tail — O(book) serial checks per run instead of the promised O(changed).
  233's "hiding in the shadow of whatever real mutation forced the wall" argument
  holds only when the wall-cause is itself expensive; a cheap opaque (a status call,
  a logger) casts no shadow, and the tail is O(book)-serial either way — vs the
  barrier form's O(crossing-facts)-parallel (or O(kinds) token reads). The exact
  "slow to interrogate serially and individually, over and over" pain, re-introduced
  by our own machinery.

- **236b-fail9 · Exclusion-check, "other user": the admin has no lever.** +SURE. The
  proposal is oracle-author-centric end to end. The admin — who *knows* "hork is my
  log-shipper; it doesn't touch nginx" — has no way to say so short of authoring an
  oracle (the cliff kBURDEN forbids). Every mainstream orchestrator gives the admin
  this lever (`changed_when: false`, `creates:`, `.PHONY`); its absence here is a
  design hole, not a principled refusal (see 236b-alt4).

Salvage list (what's *right* in 233 and should survive any redesign): the
blast-radius asymmetry (local-claims safe / global-vouches sharp — generalized in
236b-fd4); the converged-vouch's "fallible judgment, attributed, never a fact-claim"
framing; the refutation of universally-quantified no-op licenses; guard-insertion
itself as a *mechanism* (its sin is promotion to default verdict, not existence —
guards remain the right tactic for probe-less facts, declined generation-probes, canary
sites); the `check || cmd` errexit-exempt form; "one source of truth: plan-prediction
and apply-guard run the same code." Also note the r21 door-2 lineage (218a):
"declared converged-run" already worked out the (provider,verb)-keyed vouch
mechanics, the converged≠no-op hazard (hunt-A: `dpkg -s nginx` holds yet
`apt-get install nginx` upgrades an outdated package — live on the flagship
oracle; same distinction the human's 2026-07 TODO.md item draws), and the
fabricated-abort gate (d2-4a) — any redesign inherits those findings wholesale;
233's converged-vouch is recognizably door-2's claim path-scoped, and should say
so to avoid re-deriving it worse.

## §4 The reframe the proposal missed

- **236b-fd1 · The false premise: "proof is static-only."** The whole ternary rests
  on: interference across an opaque can only be (a) statically testified away
  (vouch), or (b) surrendered (guard/run). But the project's founding answer to
  "sh is unanalyzable" was never testimony — it was **the probe** ("given oracles,
  probe; then converge"). A fact whose license must survive a wall can be
  **re-observed after the wall**, at apply-time, read-only, batched. Observation is
  frame-free for exactly the reason 233 grants the in-sequence guard is frame-free —
  it happens *after* the interference — but unlike the guard it feeds the *fact
  plane*, so **elide survives as the outcome**. The question is not "elide vs guard
  vs run"; it is *"where and at what granularity do we refresh evidence during
  apply"* — a query-planner/cost decision (the project's own prior-art lens), not a
  product-visible verdict tier.

- **236b-fd2 · The design already contains this shape, four times.** +SURE:
  `an-toctou-window` (OCC re-validate the hoisted read-set at apply before mutating);
  `an-early-cutoff` (post-mutation fact unchanged ⇒ dependents still skip — ninja's
  restat, cited via the build-systems round `075`/`076`); `an-content-key` (the
  dependence-derived hash of the fact-slice a verdict reads); `an-freshness` (reuse
  only in-window). A wall mid-book is formally the *same epistemic hole* as the
  probe→apply gap the design already tolerates (minutes of cron/other-admin drift!)
  — only better-localized and more suspicious (the book is being applied precisely
  because that area of the system is under change, so opaque-interference is
  *correlated*, unlike background drift; it deserves *more* checking, not infinite
  poison). The machinery to bridge both holes is one mechanism.

- **236b-fd3 · The frame is not "the universe"; it's the crossing set.** +SURE. The
  elision question is never "does hork touch anything anywhere" — it is "does hork
  touch any of the ~O(10) facts established before it and consumed after it *in this
  book*" (`an-finite-domain`: the fact-set is bounded by the script's literals). The
  engine knows the crossing set exactly, per wall. Every 233 approach prices
  enumeration against the universe; demand-driven pricing against the crossing set
  is orders of magnitude smaller, and turns "the author must pre-declare
  everything" into "the engine asks finitely many concrete, payoff-ranked
  questions" (`an-enrichment-nudge` pointed at interference).

- **236b-fd4 · Blast-radius generalizes to claim-scope.** 233's asymmetry
  (effects-claims are socially-global; local claims risk only yourself) generalizes:
  every trust-tier should be *scoped and attributed* — book-local admin stubs risk
  the book-owner only; oracle-local footprints risk that oracle's users; kind-owner
  generation-probes risk the kind's users; curated-library family-vouches risk the library's
  users. The r23 trust-taint lattice (`plans/230` §2) is exactly the home for this
  gradient — the guard proposal *bypasses* trust (everything local) and thereby
  discards the machinery r23 was chartered to build.

## §5 Alternative designs

### 236b-alt1 · Wall-barriers: OCC revalidation of the crossing set (the load-bearing one)

Mechanism: apply is segmented at walls (opaque or under-modeled commands). At each
wall, immediately after the wall-command runs, execute a **batched, parallel,
read-only re-probe wave** of the crossing facts (the same compiled probe leaves the
probe phase already ships — one source of truth). Verdict unchanged ⇒ downstream
elisions stand (the fact was observed *after* the interference — sound under exactly
the same freshness assumptions the design already makes across the probe→apply gap).
Verdict flipped / probe failed / timed out ⇒ Unknown ⇒ the dependent sites run
(kFAIL-perform), per a contingency the plan disclosed.

- Plan/approval shape: the approved plan is a small contingent DAG — "1 will-change;
  96 converged (63 static-proven; 33 revalidate-at-wall-3); if revalidation fails,
  these run." Attention-product preserved: one collapsed disclosure group, not 97
  guard lines. This is *more* honest than today's plan, which silently bets on the
  probe→apply gap.
- Two carrier forms, both viable: (i) host-side — ship the conditional structure
  embedded in the (still static) apply artifact, no round-trips; per 22H §1 the
  human *leans static apply-scripts*, so this form is the aligned default; (ii)
  controller-mediated — stop at the wall, run the re-probe wave as a mini
  probe-phase, ship the next segment; O(walls) round-trips hidden in the shadow of
  the wall-command's own execution, per-host independent. (ii) becomes natural once
  the 22H live-plan engine exists (probe-results streaming and the plan re-folding
  *is* this, extended past apply-start) — the alternative *rides* the committed
  foundational arc instead of fighting it.
- Perf: re-probes are hoisted-to-wall and parallel (`kFLATTEN-hoist` scoped to the
  segment; the segment-local ambient analysis licenses hoisting) — vs 233's serial
  per-site tail. Same checks, better schedule, and the outcome is elide, not guard.
- +SURE this dominates the guard-tier on the attention axis; post-crosscheck the
  soundness claim is downgraded to "probe-fact soundness modulo effect-quiescence
  at validation time" (§9a F1 — a bound 233's guards share), and the mechanism
  carries four repairs: placement-range, bracket-tokens, unconditional-tripwire,
  precompiled revocation-closures (§9b). The price is engine work (segmented
  apply, contingent dispositions) — a subset of 22H, already top of TODO-ADDTL.
  Wall-waves also run *isolated from the book's shell env* (a differential
  advantage over inline guards — §9a F11).

### 236b-alt2 · Kind generation-probes: whole-kind revalidation in O(1)

*(Naming caution: earlier drafts said "kind-witness"; renamed because "witness" is
taken — the `ReplaceLicense` private-field witness pattern, 16P T6. Herein:
**generation-probe** producing a **generation token**.)*

Mechanism: the kind-owner's oracle defines a read-only **generation-probe** — a
digest of the *same observable substrate its own probes read*:

```sh
apt_Package__generation() { cksum </var/lib/dpkg/status; }   # illustrative, not design
```

Tokens are read as a *bracket pair around each wall* (before/after the opaque —
§9a F3 repair; a probe-phase-recorded baseline would be stale from the book's own
modeled pre-wall mutations). Bracket unchanged ⇒ *no* cell of that kind changed
across the wall ⇒ all crossing facts of the kind revalidate in one read. Changed ⇒
escalate: per-fact re-probes (alt1) ⇒ run. An escalation ladder, all read-only,
subject to the F1 quiescence bound (§9a); kind-owners can fold quiescence idioms
(`dpkg --audit` empty, `systemctl list-jobs` empty) into the token itself.

- The epistemics are the point: because the token digests the substrate the kind's
  probes *already read*, completeness is largely **by construction** (state
  supervenes on the substrate), not testimony. The residue ("my probes read only
  through F") is a per-kind, single-reviewable-function claim — auditable, and
  **CI-fuzzable** (container calibration harness, `an-calibration-delta`: mutate,
  assert the token moved — inc-5's machine-enforced-not-author-trusted, applied to
  interference). Contrast: 233's converged-vouch quantifier "ranges over exactly the
  observables the author never attended to"; the generation-probe quantifier ranges over a
  substrate the author *defined*.
- This inverts the frame problem's cost structure: instead of every command vouching
  non-interference with every kind (O(commands × kinds), authored by the wrong
  people), each *kind* declares once how to detect any change to itself (O(kinds),
  authored by exactly the person who knows — the kind-owner 17N already requires for
  coherence). Unknown commands are then *checked against* kinds, never trusted.
- Kills 233's retroactivity bomb: `widget` enters the universe *bringing its own
  generation-probe*; old oracles never change; crossing widget-facts revalidate at
  old-tool walls day-1. (Concrete test-case worth keeping.)
- Degradation is per-kind and graceful: kinds with no cheap generation-probe (live-daemon
  state, remote endpoints) simply stay at the per-fact-re-probe rung or at
  guard/run. ~SUSPECT most core ops kinds have almost-free generation-probes (dpkg status
  file; systemd unit-file dirs + `systemctl show` dumps; passwd/group; sysctl −
  careful; fs is per-instance stat, which is exactly the crossing set anyway).
- The generation token ≈ `an-content-key` materialized as a host-side read. The
  design reserved this seat in ANALYZER-NEEDS §G without noticing it bears on the
  frame problem. Two more internal anchors, both already-adopted design: `plans/076`
  adopts the Scheduler×**Rebuilder** factoring ("Rebuilder = Dorc's skip-decision")
  — in Mokhov's taxonomy the token-compare is exactly a **verifying-trace
  rebuilder**, applied at wall boundaries; and `076` names the "resumability
  dividend: **re-probe is the recovery** … cheap because compiled + parallel; the
  probe *is* the retry-file, derived not stored — a genuine differentiator." The
  barrier is that differentiator pointed at walls instead of crashes.

### 236b-alt3 · Positive footprints, incl. probe-computed manifests

Mechanism: an oracle may declare a command's effect-surface *positively* — and the
declaration can be **computed at probe-time from the system's own metadata** rather
than hand-enumerated:

```sh
apt_get__footprint() { dpkg -L "$1" 2>/dev/null; printf '%s\n' /var/lib/dpkg /var/cache/apt; }
```

The engine treats the footprint as the wall's *window*: crossing facts inside it are
poisoned/re-verified; facts outside cross freely. Claimed-tier, attributed,
**continuously verified** by alt1/alt2 machinery (a barrier contradiction revokes
the footprint loudly and runs the affected sites — trust-but-verify; rot becomes
*detectable* instead of silent).

- Positive-vs-negative is the entire economics: O(what-I-touch), delegated to
  ground truth the tool itself maintains (manifests), instance-precise (windows the
  fs.Path poison to `dpkg -L nginx`'s actual paths — the exact 233 §0 example),
  vs O(universe−what-I-touch) hand-lists that rot invisibly.
- Honest residue: package postinst scripts can touch ~anything (adduser, systemd
  enable...). A good apt oracle *models* those as its own cross-kind effects
  (installing nginx Establishes `systemd.Service:nginx.present` — that's ordinary
  effect-modeling, not frame-enumeration); the un-modeled residue is covered by
  *other kinds' generation-probes at the apt wall*. Composition, not completeness.

### 236b-alt4 · The admin's stub — the missing cheap rung (exclusion-check repair)

The degenerate footprint, book-local, admin-authored, one line:

```sh
hork__touches() { printf '%s\n' /var/hork; }   # book-side stub; illustrative
```

(or a Dorc-prelude wrapper-word, `unrelated hork nginx` — a real one-line sh
function, off-ramp-inert, the sudo/ssh command-wrapper axis 17N §7 already
first-classes; spelling open, `236b-oq3`). Scope: this book only; blast-radius: its
author; disclosed in the plan ("line 3 treated as non-interfering per the book's own
mark"); verified by barriers like any footprint. This is the industry-standard
answer to opaques (Ansible `changed_when`/`creates:`) recast in Dorc's
spelled-in-sh idiom, and it is the *only* rung cheap enough for the lazy-admin
center-of-market. Restores wish-B for real books without touching community
machinery.

### 236b-alt5 · Observed footprints: measure, ratify, verify (answers 233's open fork)

The open fork ("can a human vouch global-shaped knowledge about an opaque?") is
framed as pinkie-promise vs nothing. Third option: **measure it**. The deferred
tracing devtool (`plans/deferred/078`, eBPF/strace/container-snapshot diff — already
designed, quarantined as privileged-DX) plus even the barrier machinery itself
(which generation-tokens flipped across hork's wall over the last N applies) yields an
*empirical* footprint; the tool proposes the stub ("hork touched only /var/hork
across 5 runs — ratify?"); the human ratifies (one keystroke → the alt4 stub is
written into the book); the barrier keeps verifying it forever after (drift ⇒ revoke
⇒ run ⇒ report). Measurement proposes, human ratifies, runtime verifies: strictly
better epistemics than either side of 233's fork. MAY-grade evidence is never a
skip-license by itself (kill-1 / `an-must-may-grade` respected — ratification is
what mints the claim; the measurement is just its evidence).

### 236b-alt6 · Property invalidation-bases (dissolves story-1 without coordination)

story-1's burden was misassigned. The `cve_clean` *extender* — not the apt author —
knows what invalidates their property; and the extender by construction knows the
kind they're extending. So: an extension property must declare its
**invalidation-basis in terms of the kind's core state** ("killed by any change to
`.version`/`.installed`"; default if absent: killed by any kind-change, i.e.
generation-covered — safe + cheap). The apt oracle triggers the kill without ever
having heard of cve_clean. Asymmetric knowledge → asymmetric authoring; zero
pairwise coordination; the 17N kind-owner/extender structure already exists to hang
it on. +SURE this is the right assignment of story-1; -GUESS on how far it
generalizes cross-kind (the run-delta class stays forbidden territory, per 17O
R2-CHANGEDELTA).

### 236b-alt7 · Library-as-coherence-domain; curated family vouches

wish-C/D over-rotate toward "every oracle is a stranger." In practice books source a
few curated libraries (the ~40–50 bootstrap oracles, `effort-allocation`), and the
*library* is the natural coherence/vouching domain (DefinitelyTyped governance,
inc-5): maintainers — exactly the people who know — declare intra-family
non-interference once, machine-fuzzed in CI (containers: run apt-verbs, assert
docker's generation-tokens unmoved). A book using only family commands + alt4 stubs for its
weird tools gets full elide-tier with zero admin-side frame work. The O(n²) social
frame collapses to hub-and-spoke. Also bounds 233's "new kind enters the universe"
to "new library sourced into *this* book" — a local, visible act the engine can
re-price loudly.

### 236b-alt8 · Wall-aware *advice*, never engine reordering (demoted on re-check)

First drafted as planner reordering (cluster opaques tail-ward; hoist consumers) —
then **struck against drift-applypar** (STALENESS-AUDIT, human ruling: *no intra-host
apply parallelization or reordering, ever*; "within a host the book's order is
preserved; apply-phase speed comes from elision only"). What survives is
advisory-only: a lint/nudge telling the *author* "line 3's opaque forces re-checks of
12 facts; moving it after line 40 (if your intent permits) would clear them" — the
author reorders their own book, the engine never does. Kept only as a
demand-driven-nudge garnish on fd3.

## §6 Composition — the graded wall (how §5 assembles into one design)

Every command is a **graded wall**, uniformly (dissolving the safe-floor /
dangerous-middle split *by construction*, since silence never mints trust at any
tier — it only selects a more expensive evidence tactic):

per (wall, kind): transparent (that kind precisely modeled — earned per-(command,
kind), never per-command; a partially-modeled command is itself a graded wall for
every kind it doesn't precisely model, §9a F2) → windowed (footprint, alt3/alt4 —
which never suppresses the tripwire, §9a F6) → generation-checked (alt2) →
per-fact re-probed (alt1) → guarded (233's mechanism, demoted to tactic — for
probe-less facts, declined generation-probes, run-delta declines) → run (the
floor). Stub/footprint claims are admissible only over kinds with a live tripwire
(§9a F8).

The tactic per (wall, crossing-fact) is a **cost-model choice, invisible in the
verdict vocabulary** — the plan still says "converged/elided," with an evidence
footnote. kFAIL holds at every rung (all evidence tactics are read-only; every
failure direction lands on run). The trust-gradient (r23's charter, 230) is exactly
the scoping/attribution substrate for the claims (fd4). 233's own fence logic
(converged-vouch site-local; vouches never enter the fact-plane) carries over
unchanged for the guard rung.

Migration: v0 = alt4 stubs + alt3 footprints + guard-as-tactic (no engine
segmentation yet — footprint/stub simply *narrow the poison*, a fact-plane change
only); v1 = barriers + generation-probes (either carrier: controller-mediated is
least-new-machinery; host-side-embedded is the static-artifact-aligned form per 22H
§1); v2 = live-plan (22H) unification. Each stage independently shippable and
valuable. ~SUSPECT v0 alone already recovers most of the attention-product for
realistic books (the common opaques are precisely the ones an admin can stub in one
line).

## §7 Compliance check (wishes, hards, welds)

- wish-A correct: poison-undeclared restored as the *default*; every relaxation is a
  positive, scoped, attributed, runtime-verified claim. No silence-mints-trust
  anywhere. ✓
- wish-B gradual: a zero-metadata book gets probe-phase elision + barrier
  revalidation at walls (facts already have oracles or they wouldn't exist) — value
  *before* any completeness work. The 233 proposal cannot say this. ✓✓
- wish-C/D community/composable: generation-probes live with kind-owners; footprints
  self-scoped; invalidation-bases extender-side; no registry, no authority election
  beyond 17N's existing kind-owner. ✓
- wish-E accrual: a real ladder (stub → footprint → generation-probe → family-vouch), each
  rung locally-authored, machine-checkable, and *demand-driven* — the engine names
  the next rung with its payoff ("vouch hork w.r.t. fs.Path:/etc/nginx → +12
  elisions"), fd3. ✓
- wish-F / kFAIL-withhold: all new machinery is read-only probe-code, same
  self-vouch tier as the probe phase; apply-lane reads are the same posture-shift
  233 already concedes for guards. ✓
- hard-1 respected: we never analyze mutation; we *observe state around it*.
  Testimony is demoted from load-bearing to cost-optimizing. hard-2: rot becomes
  detectable (barrier contradiction ⇒ revoke+run+report) instead of silent. ✓
- Welds: `inv-probe-sourced-values` — barrier re-probes *are* probe-provenance (they
  strengthen the weld, replacing gap-stale values with fresher ones);
  `an-verdict-phase-keyed` — barrier verdicts are probe-oriented facts consumed by
  the apply-phase folder, same superposition discipline; ru-11 — barrier results are
  facts, not receipts; plan-approval — the contingent-DAG disclosure *must* be ruled
  on by the human (236b-oq1). `dorc plan`'s no-mutation promise: barriers run inside
  *apply*, never plan. ✓ (+SURE on all but the approval question.)

## §7b Standing-ruling interactions (be honest: two rulings are touched)

- **rul-toctou (201 §1, "deferred-to-actively-WONTFIX; no re-probe-before-apply, no
  freshness windows").** alt1/alt2 touch its *letter*. But (a) its *motivation* was
  "attacking [gap-staleness] would erode the last ounce of remaining value-prop" —
  i.e. paranoia-tax re-probing with no interference evidence; a wall barrier is the
  opposite case: a *named, book-caused, correlated* interference event, and the
  competing posture (233's poison→guards-everywhere) erodes the value-prop far more
  than a targeted barrier does. (b) The ruling's own direction-quote already floats
  the escape: "*maybe* someday … some sort of oracle tooling for this ('here's a
  super-cheap last-second check to run before the real thing')" — that is, nearly
  verbatim, the generation-probe/barrier shape. So: a deliberate human re-ruling is required
  (scope: walls only, never the ambient gap), exactly symmetric to how 233's own
  proposal requires reversing door-4's "build-last / product-hard-defers" posture
  (dq-errexit-3). Both candidate designs ask for one re-weld; neither is
  weld-clean. +SURE this must be surfaced as a ruling-request, not drifted into.
- **rul-mutation-impossible (201 §1).** Fully consistent, and arguably the
  alternatives take it *more* seriously than 233 does: generation-probes/barriers/footprints
  never analyze mutation — they observe state around it; testimony (the thing the
  ruling says rots in months) is demoted from load-bearing to cost-optimizing, and
  the calibration harness stays a confidence lever, never a proof.
- **22H §1 residue.** "Re-probe is apply-script-embedded, far-future, human leans
  STATIC apply-scripts" — noted; alt1's host-side carrier form is the static-script-
  compatible one, and "far-future" was priced against the pre-crisis value model
  (where static elision still worked without it). 233 changes that arithmetic.

## §8 Open questions

- **236b-oq1 · The approval contract for contingent plans — ESCALATED to the
  round's central human ruling (§9a F14).** Is "elided contingent on
  wall-revalidation; else runs" within what a user thinks they approved? The
  crosscheck's sharpest framing: contingency removes the human gate exactly when
  the world surprised. My lean: yes-iff-disclosed-as-a-grouped-contingency (the
  run-set is a subset of their own book, in their own order — the no-Dorc
  baseline), with render in future tense and per-mode posture
  (kOBJECTIVE-latency auto-continues; reconcile pauses for re-approval). But this
  is a product-honesty choice — guards' constant visible tax vs barriers'
  disclosed contingency — and only the human can rule it.
- **236b-oq2 · Generation-substrate coverage honesty.** Which core kinds genuinely have
  supervenience-clean substrates? dpkg yes; systemd enable yes (symlink dirs);
  systemd *active* is daemon-RAM (token = `systemctl show` dump — is that
  cheap/stable enough?); sysctl/proc volatile-adjacent. Needs a specimen pass
  (bench + fuzz) before the mechanism is load-bearing. Also mtime-granularity /
  same-second hazards ⇒ prefer content-digests over stat where cheap.
- **236b-oq3 · Spelling of the admin stub / wrapper-word** (alt4): wrapper-function
  vs book-side `__touches` stub vs both; naming that doesn't overclaim ("inert" is
  wrong; "unrelated"/"standalone" candidates). Same open vouch-surface family as
  every other mark (233's own note) — but the *mechanism* doesn't wait on the
  spelling.
- **236b-oq4 · Loop walls.** An opaque inside a loop = a wall per iteration;
  barrier-per-iteration may be unaffordable ⇒ validate post-loop for facts
  consumed *after* the loop (last-wall rule, §9a F12: a fact re-validates only at
  the last wall before its consumption; tokens still bracket every wall);
  intra-loop consumers degrade to guard/run — as they do under 233's guards,
  which pay per-iteration identically. Needs worked examples.
- **236b-oq5 · Fleet staggering.** Hosts cross walls at different times;
  controller-mediated v0 barriers must not convoy the fleet (per-host progression is
  already the 22H accumulator model; verify no hidden global barrier).
- **236b-oq6 · How much of alt1 lands in spike-3 vs the real build** — the
  fact-plane-only v0 (footprint/stub narrowing poison) is spike-sized; segmented
  apply is not. Sequencing call for the human.

## §9 Adversarial crosscheck of §5 (clean-context, disowned, no corpus access)

A hostile clean-context review of alt1–alt5 (mechanism statement handed
self-contained; reviewer allowed only the four root docs; no subagents). 18
findings (F1–F18), three staked as most damaging: F1 (async effects), F6+F8
(footprint blind-spot × stub incentives), F14 (approval-contract inversion).
Adjudicated below against the actual engine rules (the reviewer lacked corpus
access, so some findings attack licenses the design doesn't have); survivors
folded into §5/§6 as four **repairs**, one **honest downgrade**, and one
**human-ruling escalation**.

### §9a Adjudication

- **F1 (KILL claim) — "after the opaque exits" ≠ "after the interference": SURVIVES
  as a bound, symmetrically wounds 233 too.** Real opaques interfere after exit
  (dpkg triggers deferred into *later* invocations; enrollment daemons mutating
  config seconds later; `systemctl restart` returning at job-queue time; `&`/
  pipeline opaques have no sequencing point at all). Adjudication: (a) 233's
  in-sequence guard has the *identical* hole — its "frame-free: whatever the
  opaques did has already happened by the time it runs" premise is equally false
  under deferred effects; guards merely enjoy a wider time-cushion (they run at
  their site, later). So this is a shared bound, not a differential kill. (b)
  REPAIR-1, placement-range: a crossing fact's re-validation may run anywhere in
  [post-wall … immediately-pre-consumption], cost-model-chosen; at the late pole it
  equals the guard's cushion while still feeding the fact-plane (elide verdict,
  short plan). (c) Kind-specific quiescence belongs to the kind-owner: a dpkg
  generation-probe can *include* `dpkg --audit`-emptiness / pending-triggers;
  systemd's can include `list-jobs`-empty — ops practice has quiescence idioms, and
  putting them on the kind is exactly the alt2 assignment. (d) `&`/pipeline opaques:
  no barrier license — stay poison (an-concurrency-edge already refuses). HONEST
  DOWNGRADE: barrier soundness = probe-fact soundness **modulo effect-quiescence at
  validation time** — one new, named, disclosed assumption; the static-poison
  baseline does not need it, 233's guards need it equally.
- **F2 (SERIOUS) — post-wall *predicted* effects re-import the frame problem:
  MOSTLY A STRAWMAN of the license, one real clarification.** The elide-license
  never rides predicted effects (fact-centric weld; `inv-probe-sourced-values`;
  fork-mutator-rc: a mutator's rc is ⊤); probed guard-rcs consumed downstream ARE
  crossing facts by definition; written-stale forces MustRun after same-fact
  upstream mutation. The real residue: "transparent" status in §6 must be earned
  per-(command, kind) — a partially-modeled command is itself a graded wall for
  everything it doesn't precisely model, so its behaviour-perturbation by an
  earlier opaque is caught at *its own* wall. Now stated explicitly in §6.
- **F3 (SERIOUS) — probe-phase token baseline is stale from the book's own legit
  pre-wall mutations: CORRECT, mechanical spec bug.** REPAIR-2, bracket-tokens:
  generation-tokens are read as a *pair bracketing each wall* (immediately before /
  after the opaque, adjacent in the serial stream); equality means "the opaque
  didn't touch this kind," independent of modeled pre-wall mutations (which the
  fact-plane already tracks). Cheap (tokens are the cheap thing); residual sliver =
  background-drift class, accepted.
- **F4 (SERIOUS) — substrates lie both ways; kVOLATILES: PARTIALLY CORRECT →
  quality-bar, not kill.** False-unchanged (mtime-preserving writes, `cp -p`,
  same-second stat) ⇒ token bar: content-digests over stat where affordable; the
  17O-style probe-quality regression class extends to tokens; dpkg's
  status+journal dance means the flagship token digests status *plus* the updates/
  journal. False-changed (timestamps/counters in `systemctl show`, nft counters) ⇒
  canonicalization — which kVOLATILES-exclude *mandates* for any caching, it does
  not forbid the mechanism; cost is real, failure direction is run (safe, noisy →
  auto-demote that kind's token, PGO-style, an-cost-profiled). RAM-state kinds
  without clean substrates stay at the per-fact rung — already the ladder.
- **F5 (SERIOUS) — CI-fuzz tests sensitivity, not completeness: RIGHT, with a
  bounded answer.** Substrate-completeness ("my probes read only through F") is
  checkable by tracing the *probes'* reads — a closed, owned, per-kind set (strace
  a dozen probe bodies offline) — unlike tracing the world's writers (open set).
  The frame-quantifier genuinely shrinks; it does not vanish; stays claimed-tier
  under kVERIFY-calibrate. Folded into alt2.
- **F6 (KILL claim) — footprints are unfalsifiable outside their window: CORRECT
  as specified; REPAIR-3 resolves it structurally.** Unconditional-tripwire rule:
  generation-tokens/stat-sweeps for ALL crossing kinds run at EVERY wall,
  regardless of footprints. A footprint never suppresses evidence; it only (a)
  un-poisons at plan-time and (b) marks in-window token churn as *expected*. Out-of-
  window lies therefore ARE confronted (token flips → contradiction → loud revoke →
  safe re-execution). Economics restated honestly: footprints buy plan-time
  optimism + attribution, not wave-elimination; the tripwire is the constant.
- **F7 (SERIOUS) — aliasing/bind-mounts/daemon-mediation: real, bounded by
  REPAIR-3.** Per-host realpath-canonicalize footprint paths and cell paths at
  wall time; hardlink/daemon residue lands on the tripwire (crossing facts are all
  checked); measured footprints must come from fs-diff/eBPF (078 already
  established per-process tracing fails on daemon-mediated tools), never bare
  strace.
- **F8 (KILL-as-social-system) — the stub is a pressure-relief valve whose
  equilibrium is silent w2: THE serious social finding; bounded, not dissolved.**
  With REPAIR-3 stub lies are runtime-confronted (loud, next apply). NEW GATE
  (engine-enforced): a stub/footprint claim is *admissible only for kinds with a
  live tripwire* — a window claimed over a token-less kind is inert (stays
  poison/guard). The bypass switch works only where the alarm system is live.
  Residual honestly carried: incentive pressure toward cargo-culted stubs is real;
  attribution + plan disclosure + the nudge-ledger are mitigations, not proofs.
- **F9 (KILL for alt6 as drafted) — invalidation-bases can't express
  externally-grounded properties: CORRECT; grammar widened.** `cve_clean` is
  invalidated by an external feed + time, not host cells. alt6's basis grammar
  becomes: core-cell basis ∪ *own-substrate* basis (the extender's probe-read set —
  which the extender knows, it's their own probe) ∪ volatile/TTL (routes to
  `an-freshness`, already-designed). Default-if-absent stays killed-by-any-kind-
  change. Core-evolution coupling: coherence-CI direction (inc-5), conceded
  one-directional; default catches the gap safely.
- **F10 (SERIOUS) — revocation cascades ⇒ re-planning ⇒ v1 is a second engine:
  RIGHT problem, wrong conclusion; REPAIR-4.** Precompiled revocation-closures:
  because run-more is the safe direction, the engine statically over-approximates
  per-crossing-fact closures (flip F at wall W ⇒ run-set R(F,W) = union of
  transitive dependents), and ships them as a decision table (plain per-site
  run-flags). The host evaluates prepared booleans — no on-host fact-store, no
  second engine, hosts stay dumb (111). v0 (controller) may re-fold exactly
  (build_plan is pure/cheap, 22H §3); both carriers read the same table, so
  cross-carrier divergence reduces to F1's timing cushion. -GUESS artifact-size is
  fine (closures are unions over ≤ crossing-set, encodable as flags).
- **F11 (SERIOUS) — injected probe code inside the book's shell, mid-mutation:
  CORRECT — and it's a differential ADVANTAGE of barriers over 233's guards.**
  Wall-waves run *between* leaves and need none of the book's shell state ⇒ execute
  them as a scrubbed, isolated process (or a separate channel entirely; v0 does
  this naturally); the book's stream waits on one verdict read. 233's inline
  guards, by contrast, structurally sit inside the book's env (its own 218a
  hazard-list concession) — both contamination directions live. Folded into alt1.
- **F12 (SERIOUS) — loops and wall×fact multiplication: real; specified.**
  Last-wall rule: a fact re-validates only at the last wall before its consumption
  (tokens still bracket every wall — cheap). Intra-loop opaque: no elision of
  intra-loop-consumed facts (233's guards pay per-iteration identically);
  post-loop consumers validate once at loop exit. → oq4 updated.
- **F13 (SERIOUS) — waves inside author critical sections (stop/migrate/start)
  stretch outages: GOOD catch; REPAIR-1 (placement-range) is the mitigation** —
  defer validation toward consumption, escaping the stop/start span when consumers
  sit outside it; probe-time budgets bound wave latency (timeout ⇒ run, safe).
  Honest residual: some added mid-apply latency is intrinsic to any evidence
  refresh, including 233's guards (same checks, inline).
- **F14 (KILL on w3/UX) — contingent elision replaces approval with a forecast and
  removes the human exactly when the world surprised: THE central product finding;
  escalated to the human (oq1 is now the round's #1 ruling).** Softeners, honest:
  (a) render tense must be future ("will re-verify at apply"), never
  outcome-typeset; (b) the contingency run-set is a subset of the user's own book
  in their own order — the no-Dorc baseline, disclosed as a foldable group, so the
  floor ("no worse than running blind") holds; (c) revocation-budget /
  pause-and-ask postures per kOBJECTIVE mode (deployer auto-continues,
  reconcile-mode pauses) — F14's own suggested fix; (d) noise-triggered runs are
  bounded by token quality and always disclosed post-hoc (why-lens). Conceded
  crisply: 233's guard-default is honest-by-pessimism (it never claims contingent
  elision — it always pays what barriers pay only on revocation); the barrier
  design converts that constant tax into a disclosed contingency. Which honesty the
  product wants is the human's call.
- **F15 (SERIOUS) — noisy kinds ⇒ escalation storms / plan-flap:** bounded by
  canonicalization (F4), per-kind auto-demotion on observed flap (an-cost-profiled
  feedback), and the safe failure direction; cross-run flap only from
  nondeterministic probes (the volatile bar again). Carried as a token-quality
  obligation.
- **F16 (values) — dead stub-functions are pragmas wearing sh (w4 drift): FAIR on
  the `__touches` stub form; tilts oq3 to the wrapper-word.** kOOB's redline
  explicitly sanctions config-as-sh/library-code, but "scripts stop meaning what
  they say" is the real cost — prefer the *live* wrapper spelling (`unrelated hork
  nginx`: executes hork, reads as documentation, runnable anywhere) and the
  genuinely-executable footprint function; note 233's vouch-marks carry the
  identical cost (shared spelling-family problem, the same open vouch-surface).
- **F17 (edge spec-gaps):** specified — a wall's own claimed establishes never
  license (only probe-verdicts license; its establishes only mark written-stale);
  last-wall rule (F12); an opaque *consuming* an elided producer's observables is
  already governed by `inv-probe-sourced-values` (unreproducible-consumed ⇒
  producer runs).
- **F18 (resume):** resume = re-probe; the probe *is* the retry-file (076's
  resumability dividend); never trust a prior attempt's baselines. Cost
  multiplicative with failure rate — honest.

### §9b What the crosscheck changed (delta summary)

REPAIR-1 placement-range (F1/F13) · REPAIR-2 bracket-tokens (F3) · REPAIR-3
unconditional-tripwire + token-gated stub admissibility (F6/F7/F8) · REPAIR-4
precompiled revocation-closures, hosts stay dumb (F10) · alt6 grammar widened
(F9) · alt4 spelling tilted to wrapper-word (F16) · soundness claim downgraded to
"modulo effect-quiescence at validation time," shared with 233's guards (F1) ·
oq1 escalated to the round's central human ruling (F14). The comparative bottom
line survives adjudication: every KILL either hits 233's guard-default equally
(F1, F11, F12) or is repaired structurally (F3, F6, F10); the one asymmetric
finding (F14) is a *product-honesty choice* between a constant visible tax
(guards) and a disclosed contingency (barriers) — not an engineering refutation.
