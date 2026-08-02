# 28R — context-kernel (28Q) review round: shared home

> Tier: LLM-authored conductor notes (r28 review arc); subordinate to `28Q`, root docs,
> and `spike/CLAUDE.md`. SHARED NOTE: multiple conductors home 28Q-review work here
> (an adversarial-review lane runs in parallel with the prior-art lane). Append your
> lane as a new top-level section; never restructure or edit another lane's section.

## §prior-art-round-takeaways — round-general findings (lane: prior-art full-reads, 2026-08-01)

Evidence base: the `context-kernel-prior-art` round's turn01 ledger (graded + durable;
currently at `.claude/research/context-kernel-prior-art/turn01-2026-08-01-notes.md` in
that round's worktree, `agent-a869c632c2df22892`) — consult it for per-source
elaboration and verbatim quotes. This section is the conductor-filtered residue:
findings judged worth carrying into implementation-conductor behaviour. Omissions are
judgments, not oversights.

Confidence discipline for the whole section: most PLT sources in this round match Dorc
on *shape* — everyone doing dataflow over programs-that-mutate converges on similar
patterns — and a shape-match transfers vocabulary and trap-lists, never theorems. The
ops-ecosystem cluster is the exception (same domain, not merely same shape) and is
weighted accordingly.

- **fnd-pessimistic-pass-shape** (+SURE — a mathematical property, not an analogy) — an
  optimistic fixpoint (assume-good, demote on evidence) yields *wrong* answers if
  halted early; a pessimistic one merely poor ones. Conductor rule: any kernel pass
  that could ever be budget-capped, timed out, or read mid-fixpoint must be
  pessimistic-shaped (start at walls, prove survival). Check stage-i/stage-iii briefs
  against this. [A-wegman-zadeck-sccp-1991]
- **fnd-key-reachability-to-edges** (+SURE on the source; ~SUSPECT it matters at Dorc's
  scale) — executability/reachability flags belong on CFG *edges*, not nodes;
  node-keying provably loses facts. Cheap to honor from day one, annoying to retrofit.
- **fnd-load-plane-bet-priced** (~SUSPECT overall) — corpus numbers favor P1 (`.` is
  mainstream, ~5k occurrences in the Debian corpus; `eval` vestigial at 42), and the
  one empirical pricing study found whole-entry-context resolution ~3500× slower than
  file-local while buying nothing in 2 of 5 systems. Conductor rule: size stage-i to
  the one-frame common case; frames are an O(env-mutating-statements) bounded
  structure, never a context-sensitivity dial. Plus a coverage-honesty warning from
  the JS/eval literature: a `.`-resolution-rate metric that omits the other
  computed-code vectors (command substitution into a sourced path, computed
  interpreter dispatch) overstates true coverage — count the denominator.
- **fnd-corroborated-literal-plane-only** (+SURE as corroboration) — three independent
  systems (Sorbet, PHP AiR, the phase-distinction literature) each derived
  `funcenv-reads-source-literal-plane-only`'s restriction for their own reasons
  (soundness vs reflection; no-logical-cycle in definition discovery; decidability).
  Standing invariant, now citable when challenged.
- **fnd-dedup-keys-to-resolution** (~SUSPECT, but specific and actionable) — the one
  source that adjudicates dedup-vs-refuse head-on (Backpack) keys module identity to
  source *plus what its own imports resolved to* — explicitly not to bytes. 28Q §2's
  dedup clause keys to byte-identity alone; two byte-identical helpers whose own
  `.`-lines resolve differently would silently merge two speakers. CHECK-ITEM for the
  stage-ii brief: restate dedup over closure-identity, or show sh's resolution rules
  make the divergence unrepresentable.
- **fnd-diamond-fires-the-fence** (~SUSPECT) — two entry files each sourcing one shared
  helper = two closures, two speakers; the fence's own motivating literature shipped
  exactly this false-positive unresolved, for lack of a manifest — and no-manifest is
  our chosen posture too. 28Q §2 asserts sibling/cousin-edge-only binding; CHECK-ITEM:
  a fixture proving the shared-helper diamond lands on the intended side.
- **fnd-collision-posture-spread** (+SURE — same-domain evidence, the round's
  best-grounded cluster) — across shipped config-management, collision handling sorts
  cleanly: hard-error-at-declaration (Puppet; Debian file-ownership policy) shows no
  recorded regret; total precedence orders (Chef's 15 slots, Ansible's 22) show
  documented regret, including implementation-vs-documentation drift that went
  undetected until someone built a differential harness; silent last-wins merge was
  repudiated in writing (Chef resource cloning), sharpest stated reason being loss of
  *addressability by name*. Dorc's refuse-loudly posture sits in the no-regret cell.
  Conductor rule: when a collision shortcut tempts (precedence rank, quiet merge),
  re-read this table first.
- **fnd-coherence-underpins-elision** (+SURE on the in-source mechanism; ~SUSPECT on
  transfer weight) — GHC's optimizer *assumes* instance coherence and observably
  miscompiles when it is violated; and the refined trigger is that resolution picks
  *locally, silently, differently at different sites* — not that two candidates are
  simultaneously visible somewhere. This is the chimera class 28Q §1's
  definition-factoring makes unrepresentable; useful as the failure-story when the
  fence or the factoring is challenged.
- **fnd-bridging-oracles-foreclosed** (~SUSPECT; watch-item, not action-item) —
  single-occupancy rules structurally forbid third-party glue (nobody may publish an
  oracle teaching family A about family B's vocabulary), and both ecosystems that
  shipped such a rule needed an escape valve almost immediately. No action now; expect
  this pressure to arrive with the stdlib/community arc.
- **fnd-incarnation-correlation-shapes** (mixed grades) — for the
  `28Q:res-incarnation-correlation-door` sitting, three concrete shapes worth bringing:
  Raft (old-term evidence never ripens on its own accumulation, only in conjunction
  with a current-term fact — the correlate-don't-divorce precedent); NFS volatile
  filehandles (identity comparison licensed to improve *performance, never
  correctness*; plus the two-error split "object is gone" vs "your handle expired, the
  object may well exist"); Kubernetes (uid = which incarnation / generation = which
  revision of intent / observedGeneration = how far caught up). ⚠ K8s currently
  contradicts itself across two live documents on version-string opacity; pin the
  document if cited.
- **fnd-identity-signals-decay** (+SURE — plain ops facts) — machine-id survives most
  churn and must be *actively blanked* in golden images, so host identity is a property
  of someone else's provisioning discipline, not of the machine; boot-id discriminates
  lifetimes only at the host's actual reboot rate and silently acquires
  machine-id-like properties on long-uptime hosts. Neither is sufficient alone as an
  incarnation signal. systemd's per-service invocation-ID is the nearest shipped
  finer-grained marker, unexplored this round.
- **fnd-lifecycle-contagion-vs-statelessness** (~SUSPECT; CHECK-ITEM; full-read
  verified 2026-08-01) — Terraform's one lifecycle flag that *must* persist to state is
  exactly the one that propagates across the dependency graph (create_before_destroy),
  because selectively reversing edges makes cycles (maintainer: overriding it on a
  subset "amounts to reversing the dependencies"). Full-read carves: the persistence
  pressure routes through delete-by-undeclare (destroy of a thing whose config is
  gone must remember its creation semantics), a product surface Dorc structurally
  refuses — so the *persistence* half likely dissolves for us; and the cycles arise
  because Terraform *derives* ordering, which no-reorder-ever forecloses. What
  survives as the CHECK-ITEM: declared per-object lifecycle/correlation policy that
  propagates along dependency/reach edges, where a per-object opt-out is incoherent —
  the coherent postures are derive-the-propagation-and-refuse-overrides-loudly, or
  don't offer the per-object knob. Full-read rider (+SURE, in-source): the propagated
  flag also *suppresses destroy-time provisioners* on nodes that never opted in —
  implicit propagation of a semantics-bearing mark changed behaviour on a node whose
  author never chose it, with no attribution; any propagating Dorc mark must carry
  whose declaration forced it. [B-create-before-destroy-propagates-2026] ·
  [B-cycle-argument-maintainer-explanation-2019]
- **fnd-reify-guard-correlation** (~SUSPECT; book-analysis flavor) — two systems two
  decades apart reject correlated-but-separate guards (`if t; then …; fi` … later …
  `if ! t; then …; fi`) and prescribe the same repair: reify the correlation into a
  value. The sh spelling is native (a variable holding a test's result), so if
  book-analysis ever grows flow-refinement, prefer recognizing the reified idiom over
  solving the unreified pattern.

## §full-read-verdicts — the two deep-read sources (lane: prior-art full-reads, 2026-08-01)

Full reads of [A-lifetimes-as-program-points-2017] (Rust RFC 2094, "non-lexical
lifetimes") and [A-access-permissions-modular-typestate-2007] (Bierhoff & Aldrich,
OOPSLA'07), graded against 28Q. Same framing discipline: unless marked otherwise a
finding transfers vocabulary and trap-lists, not correctness arguments. Only items
that survived a skeptical pass are recorded.

### NLL / RFC 2094

- **vd-window-is-point-set** (+SURE; the strongest single transfer of the round — and
  not by shape-analogy but as an experimental data point) — the same team first
  shipped validity-regions-over-a-CFG as *continuous* regions (RFC 396), found
  continuity made real cases unfixable, and rebuilt on point-sets-with-gaps. The gap
  case is structurally the re-creation (a context available, then departed, then
  available again as a new incarnation). Stage-iii type-shape consequence: an
  availability window is a position-set (or interval *list*), never a (start, end)
  pair; the cheap encoding silently over-approximates every incarnation boundary.
- **vd-conservatism-sign-flips** (+SURE; trap-warning) — their safe-to-be-sloppy
  direction is over-*extending* validity (rejects good programs, stays sound); ours is
  over-*killing* facts (runs unneeded commands, never under-executes). Opposite signs.
  Any approximation shortcut borrowed from borrow-checker literature must be re-derived
  for rounding direction; never port one as-is.
- **vd-wall-machinery-is-loan-dataflow** (~SUSPECT on value — likely convergent shape) —
  their in-scope-loans pass (gen at borrow; kill at region-end / path-overwrite;
  forward dataflow) is shape-identical to fact-survival across walls. Residual value:
  their transfer function is a tidy enumerated kill-condition checklist to diff ours
  against when stage-iii enumerates kill conditions; their path-prefix kill rules
  (shallow vs supporting prefixes) are the syntactic cousin of footprint×backing under
  `resolve`/`disturbance_reaches_only`.
- **vd-synthetic-exit-edges** (+SURE; cheap and concrete) — any backward pass
  ("consumed later?" is a reverse analysis) diverges on exitless regions; the standard
  fix is synthetic edges yielding a postdominating exit node. Books really contain
  `while true` supervisor loops and `exec` tails. If stage-iii runs any reverse
  analysis this belongs in the brief from day one; their motivating bug (a fact
  appearing immortal because teardown was unreachable) has a direct Dorc shape.
- **vd-destroy-tolerates-staleness** (-GUESS; ergonomic, not load-bearing) — their
  two-track liveness (use vs drop; may-dangle) suggests destroy-verbs legitimately
  need weaker freshness than read/mutate verbs. Park for the `28Q` §10 authored-surface
  dig.
- **vd-three-point-narrative-convergence** (+SURE as corroboration only) — their
  prescribed error form (borrow point / invalidating action / later use; blame the
  middle action, justified by the later use) independently matches the why-chain's
  measured / ran-above+claimed / decided shape. No action; citable when the
  why-arrangement is challenged.
- Downgraded after the full read: the exclusivity/borrow-conflict half (no Dorc
  referent — serial apply, sacred order; only the read-killed-by-write half maps), and
  the named-lifetime `end('r)` machinery (its Dorc analogue would be static claims
  across the plan→apply gap or across runs — deliberately refused territory).

### Access permissions / Bierhoff & Aldrich

- **vd-checked-versus-vouched-seam** (+SURE; the most conductor-actionable item from
  this source) — every guarantee in their system is typechecker-verified over all
  participating code; Dorc's cousins (`disturbs` at-most claims,
  `undivided-by-transit-across`) are human claims. Consequence for all prior-art use:
  a transferred mechanism lands in the license plane only where Dorc's own proofs back
  it (read-set closure, the structural self-vouch); everywhere else it lands as
  attribution structure. When a future brief leans on literature soundness, first ask
  which side of this seam the load-bearing guarantee sits on.
- **vd-verdict-is-dynamic-state-test** (+SURE on the structural identity; the one
  terminology-steal candidate) — their "dynamic state test" (read-only body; the
  boolean result spec-correlated with abstract state via a disjunction of
  conjunctions) is member-for-member the verdict function + rc-partition
  (0 = named sense ⊕ 1 = complement ⊕ ≥2 = nothing claimed). The term is precise and
  established; worth adopting in design prose where "check" is overloaded.
- **vd-weak-half-lens** (~SUSPECT as anything more than an organizing lens) — sorting
  permissions by what-others-may-do-behind-you puts Dorc's probe in their `pure` cell
  and its apply in `share`; the exclusive cells never occur on a host. Under the lens,
  three Dorc rules read as one discipline: temporary knowledge dies at unaccounted
  effects (walls), knowledge is re-acquired by a runtime test (guards), and coarse
  knowledge survives under a declared bound (at-most claims). Lens only; their
  soundness does not come along (previous item).
- **vd-freeze-forget-fork-closed** (+SURE on the recorded history; value is
  door-closing) — their predecessor (Fugue) froze state knowledge permanently at the
  first unaccounted alias; they name that as the over-restriction and rebuild on
  forget-then-retest. In Dorc's permanently-aliased world, freeze degenerates to
  guard-tier-forever after the first wall. Standing counter-argument on file against
  any future drift toward "once poisoned, stop re-checking." The price they name —
  runtime tests must be first-class in the model — is already paid
  (`inv-one-observable`, the rc-partition).
- **vd-orthogonality-is-owner-indexed** (+SURE it is in the formalism; ~SUSPECT on how
  deliberate the parallel is) — their cell-disjointness judgment is indexed by the
  class that *defined* the state space: "these cells are orthogonal" is decidable only
  inside the minting vocabulary. Independent corroboration of the sparing algebra's
  same-family-dialect restriction; cite-when-challenged.
- **vd-effects-bit-mechanism** (~SUSPECT) — their checker derives a
  may-have-changed-state bit from the callee's *declared* permission kinds, and
  pure-only callees provably preserve callers' temporary knowledge. Mechanism
  suggestion for 28Q's wall-transparency corollary: derive transparency from
  participants' claimed effect surfaces, rather than as a special-cased delay-loop
  rule.
- Negative results, recorded so nobody imports them: **fractions** (rejoin-to-
  exclusivity accounting) have no Dorc referent — Dorc never claims exclusivity over
  host state, and its one exclusive object (controller scratch) gets exclusivity from
  create-semantics, not accounting. And the **specification logic** (MALL with
  quantified fractions; NEXPTIME-hard) must not enter the kernel; every mapped item
  above needs only set operations over declared coordinates (`an-flat-domain` stands).

## §crosscheck-adjudication — the six-lane review, adjudicated (lane: conductor, 2026-08-01)

Inputs: `28Ra`–`28Rf` (Fable ×2 · Sol ×2 · DeepSeek ×2, clean contexts, worktrees at
`356e3948` with doctrine files deactivated) + the §prior-art sections above. Weighting per
the calibration ladder (Fable > Sol > DeepSeek; cross-lineage convergence = the strongest
signal available, still not proof; adversarial-only findings suspect-until-checked). Lane
hygiene note: BOTH DeepSeek reports ran the adversarial lens (`28Re` opens "the
adversarial review" despite its `-neutral` filename), so that lineage contributed two
hostile passes and no neutral one — its solo findings are discounted accordingly. Every
CONFIRMED verdict below was re-verified by this conductor against the in-context corpus
(`28P`/`28O`/`28M`/`28K`/`27C`/steering law) or against the tree by hand; two claims were
re-checked in code before adjudication (stage-0 is BUILT — the verdict-lane ship tests
exist and `plan/CLAUDE.md` carries the built-at-stage-0 law; and
`oracle::load_inert::item_is_load_inert` really admits only funcdefs + static assigns,
grounding `28Rb` F2).

**Round verdict.** No kill-shot. The three-pillar direction, the stage-0 verdict-primacy
re-cut, the P1 factoring argument (chimera-unrepresentable), the flat-domain
reconciliation, and P3's fail-safe floor survived six lanes of review including two
genuinely hostile frontier passes — each was attacked and each attack was withdrawn as
did-not-hold in at least one hostile lane. What did NOT survive is narrower and
concentrated: P1's helper-closure story is semantically wrong as written; P2's closure
identity is undefined at every edge that matters (and unspellable today); the stage gates
verify almost nothing the refactor changes; P3's availability domain is stated too small;
and a cluster of overclaims would mis-steer stage briefs if ratified as written. All are
repairable inside the plan's own conservative postures; none demands a direction change.

### Confirmed findings (adjudicated; disposition attached)

- **adj-helper-closure-frame-plurality** (CONFIRMED; stage-i BLOCKER; sources: `28Rc` #1 +
  `28Rb` F1, independently convergent cross-lineage) — sh resolves a body's calls against
  the environment AT INVOCATION; once role members answer positionally, helpers can be
  positionally plural (blessed-override supplying a same-named helper; regional
  re-source), and §1's "closure … computed once, whole-unit / no index multiplication"
  re-creates the chimera one level down: a positionally-correct role body shipping a
  helper binding no shell at that site would resolve. `28Rb`'s emission half also holds:
  frame-plural helpers cannot be hash-munged without rewriting call sites inside AUTHORED
  bodies (violates `28K:rul-pin-by-definition-bytes` + strip's no-in-body-renames), so
  the frame-keyed-closure "fix" has no legal emission. Disposition: rule the conservative
  option in §1 — helpers stay whole-unit-contested (`helper-declaration-contested`
  stands); a frame whose live role definition closes over a helper name that is plural-
  with-differing-bytes across frames WITHHOLDS (disclosed value-loss); the plural idioms
  deliver only where helper namespaces do not collide. One sentence + one commissioned
  fixture; the constants analogue (`dec-constants-ride-per-contributing-file`) gets the
  same sentence.
- **adj-closure-identity-undefined** (CONFIRMED as a composite; stage-ii BLOCKER;
  sources: `28Rc` #2, `28Rd` #2, `28Ra` §1, `28Rb` F2, §prior-art
  fnd-dedup-keys-to-resolution + fnd-diamond-fires-the-fence, both DeepSeek lanes on the
  unruled foundation) — four independent under-definitions: (1) no rooting rule — "one
  identity per entry file" contradicts the sibling/cousin fence carve as written, and the
  fence-dissolving misreading (book sources two strangers ⇒ one speaker) is the one a
  builder would implement; (2) overlap/diamond identity undefined (a shared helper file
  belongs to two closures; byte-dedup doesn't pick a speaker — and §prior-art's Backpack
  finding says byte-identity is the wrong dedup key outright); (3) NO LEGAL SPELLING:
  marked files refuse top-level `.` (verified in code) and book-level `.` walls, so every
  closure today is a singleton and §9's "complete list" omits the load-inert amendment
  P2 needs; (4) the bitem6-proven package shape is CLI-sibling-loaded — two closures
  under P2's definition — so "custody flows to the closure" vs `28M` §7's
  helpers-ride-under-the-CALLING-entrypoint is caller-keyed-vs-closure-keyed ambiguity,
  and kind-owner single-occupancy silently narrows for split-file owner packages.
  Disposition: §2 gains a definitional paragraph (closure-identity as a function: what
  roots a custody unit; sibling edges fence, ancestor edges take custody; the five
  truth-table cells from `28Rd` — two entries × shared helper · entry-sourcing-entry ·
  diamond · ambient+subshell re-source · book-over-package-helpers); membership semantics
  decided (sourcing-only vs sourcing-plus-CLI-co-naming); dedup re-keyed to
  resolution-identity or divergence shown unrepresentable; §9 gains the oracle-side
  load-inert `.` amendment as an owed ruling.
- **adj-book-closure-runtime-prereq** (CONFIRMED; source: `28Rd` #3) — stage-ii
  schedules analysis-side custody but not `28K:res-book-ships-its-load-closure` (named,
  unbuilt), and no executing corpus case can even carry a book-level `.` (`28P`). When
  the dot-blessing lands, analysis-green/runtime-dead is the false-green shape.
  Disposition: runtime closure materialization + an executing e2e cell join stage-ii's
  scope and gate.
- **adj-gates-vacuous-where-machinery-acts** (CONFIRMED; sources: `28Ra` §2, `28Rb` F5,
  `28Rd` #1, `28Re` #6) — the corpus is single-definition/define-before-use, so
  byte-identity gates are satisfied trivially by the new machinery; the lane's own best
  catches came from fixtures commissioned BEFORE the ruling (bitem6, item0), and the plan
  inverts that order. Also stage-0's outcome gate cannot see lost measurements (a named
  predict cell silently becoming an auto-cell changes records, backings, why-chains —
  not outcomes), and stage-iii's gate lacks the carve for the behavior it itself
  introduces. Disposition: stage-i commissions the plural-idiom fixtures FIRST and as
  DIFFERENTIAL cells (sentinel bodies under the two-binary floor — the bitem8 pattern);
  stage-0's fold gains a records/fact-set diff (already-built ⇒ run it retroactively at
  the stage-i brief); stage-iii's gate reads "no lifecycle events, no host-denoting
  lines, no local-exec". Rider: §1 restates `28P:adj-never-live-exactness-accepted`'s
  standing consequence for the WHOLE frame solver — under true resolution every funcenv
  precision bug is winner-shifting, so funcenv work is license-review-tier forever; the
  "dissolves" wording drops.
- **adj-availability-domain-understated** (CONFIRMED; sources: `28Rc` #3 + §prior-art
  vd-window-is-point-set + fnd-pessimistic-pass-shape; `28Rf` #3 and `28Ra` §7 for the
  wait-loop corollary) — §3's four statuses cannot represent conditional/looped
  lifecycle events; the needed statement is small and conservative: entry consumes only
  MUST-available; a MAY-run begin never licenses entry and a MAY-run end conservatively
  distrusts (the wall machinery's existing direction); windows are position-SETS, never
  (start,end) pairs (the NLL result); loops/joins land on unknown ⇒ guard/run. The
  wait-loop transparency sentence as compressed inverts silence-licenses-nothing
  (absence-of-events ⇒ transparent); the sound form routes through positively-MODELED
  purity ("modeled-pure bodies; an unmodeled body walls as ever" — the `26K`
  ratification's own wording). Disposition: one availability-domain paragraph in §3 +
  the two-word fix to the wait-loop sentence.
- **adj-host-asymmetries** (CONFIRMED composite; sources: `28Rb` F4, `28Rd` #4, `28Ra`
  §3/§8) — four sub-items, all real: (1) "entry is uniform" requires the ambient-host
  carve (the CLI-named target is consent-by-invocation; without it the default dial
  would forbid baseline probing — reductio); (2) host aliasing must NOT ride the kind-
  resolver contract: for hosts a wrong MERGE is not conservative (it licenses cross-host
  fact reuse — the inverted failure direction vs `USER_STORY`'s resolver pricing), and
  `260`'s HostId-verbatim law + `an-host-as-adversary` both cut against it; v0 posture:
  no host merging, duplicate probing is the price, controller-authenticated identity is
  the only future route; (3) the ssh entry form cannot be `"$@"`-verbatim across the
  remote re-parse — `rul-host-entry-is-ordinary-entry` [ACKED] therefore quietly
  requires un-punting `27C`'s only-entry-shape ruling or minting ssh a carved shape, and
  the re-parse is the `24T` payload-decomposition problem wearing a network hat (cite it
  at §10, not a vouch rider); (4) probe-time side-host entry is a new
  latency/footprint class on the plan critical path (perf-doctrine's one watched cost)
  and needs a stated policy (lazy/opt-in/timeout budget) plus a host coordinate in
  `27C:render-authority-disclosure`. Disposition: all four named in §3/§6/§7; the
  entry-shape conflict and side-host policy become §9 owed rulings; stage-iii's
  dependency on the stdlib revival (ssh oracle) stated in §8.
- **adj-blessing-vs-keep-verdict** (CONFIRMED as a tension needing one typed line;
  source: `28Rb` F3) — best-caller elevation makes verdict-word enrollment reachable via
  a thin delegating predict (the keep-verdict's own softener sanctioned that half), but
  grade-by-reachability is refactor-sensitive call-graph topology, not a speech act
  (`prn-vocabulary-is-output-side`), and the reverse-flow cell (a judgment-tier helper
  later reached by predict through a shared utility, elevated everywhere) sits inside
  one closure and one family, where `pin-blessing-keying`'s "only bites" clause does not
  reach. Not a stage-i/ii blocker; bites at vocab-minting. Disposition: routed to the
  human as a ruling ask — either re-affirm keep-for-now knowing the exclusion is now
  ceremony-tier, fold it into the spike-end instrument explicitly, or key elevation to
  marks-reached-from-predict rather than blanket text-elevation.
- **adj-sparing-two-position-rule** (CONFIRMED gap; source: `28Ra` §5) — the sparing
  meet reads ACROSS sites (claim minted at p, backing at q); once liveness/custody/
  dialects are frame-relative, no text says which position's environment governs.
  Disposition: one PROPOSED conservative line for the human's cheap ack — "collide
  unless both positions agree on the backing family's closure and dialect" — placed
  before stage-ii.
- **adj-for-free-claims-repriced** (CONFIRMED; sources: `28Ra` §4, `28Rb` F6) — the
  whyworld/survival unification was priced by `28P` as a dispatch ("re-lifting that
  seat's whole world"), not a coincidence-deletion; and
  `tc-wrapped-lane-drops-a-case-bodied-in-book-verdict` was measured-NOT-diagnosed, so
  §1's asserted cause is a hypothesis. Disposition: stage-i carries `28P`'s price tag;
  the case-bodied wrapped fixture rides stage-i as an EXPECTED-TO-FLIP cell so the
  asserted cause is tested rather than trusted.
- **adj-zero-new-spellings-scoped** (CONFIRMED; sources: all six lanes in some form) —
  the slug is true of admin/sh acts and false of oracle vocabulary: P3's load-bearing
  input (how an oracle says "begins/ends") has no existing spelling and is exactly what
  §10 reserves. Disposition: re-scope the slug ("no new ADMIN surface; the oracle-member
  vocabulary is §10's priced budget") so it cannot be cited against the additions
  stage-iii cannot proceed without.
- **adj-small-batch** (CONFIRMED, cheap; various lanes) — the `272` §6 "time" row is the
  Linux time-NAMESPACE, not a lifetime slot (mint the lifetime axis as a NEW axis value;
  leave the timens row); the epoch/pivot/transit retirement needs the three-line mapping
  table (transit splits across TWO new rows); the umbrella "three-pillar direction as a
  whole [ACKED]" line gets scoped "direction, not mechanics";
  `res-survival-lanes-still-ship-closure-less` enters §5's ledger (in or explicitly
  out); the per-host-frames eventual-story sentence names itself an AMENDMENT of
  `funcenv-reads-source-literal-plane-only`; §6 gains the two-planes fence sentence (the
  load plane never grows a probe-data input) and the PRE/available-expressions citation;
  USER_STORY's stage-2 "predict body" guard-anatomy wording joins the human-edit-someday
  list.

### Reframed or partially held

- `28Rf` #5 (incarnation "softly defined" vs hard-partition default): the delivered
  default IS distrust-walls; "softly" correctly describes the held-open correlation
  door, not delivered semantics. One clarifying clause at most; no design change.
- `28Re` #9 (converged-creator theorem preconditions): the divergence cells land on
  dynamic entry failure ⇒ can't-say ⇒ guard/run; the floor holds. A converged (elided)
  creator mints NO event and hence no fresh incarnation — worth one clarifying sentence
  in §3, not a fault.
- `28Re` #7 / `28Rf` #7 (subsumption/dissolves rhetoric): the mechanism-replacement
  reading is right and is absorbed into adj-gates-vacuous's rider; the rest is wording.
- Stage-ii/-iii structure (`28Rc` #4/#5, `28Re` #6): absorbed as staging edits — split
  custody INFRASTRUCTURE (buildable now, license-inert) from policy consumers (fence
  permanence, blessing keying — human-gated), and stage-iii into definite-availability
  mechanics vs §10-gated authoring vs the correlation door. The "independently green"
  phrasing weakens to name what each stage's green actually asserts.

### Dismissed (so nobody re-litigates)

Attacks run and withdrawn by the hostile lanes themselves, concurred: P1-is-k-CFA (the
frames-are-intervals reconciliation is real); lifecycle-availability circularity (the
dependency is forward; pre-creation contexts conservatively guard/run); plan-scale vs
gradual-enhancement (staging + byte-gates are the mitigation); the frame-relative fence
(arguably the MORE correct reading of `28M` §4); stage-0's soundness (repairs a measured
two-author license; the ruling stands). Conductor dismissals: `28Re` #8
(gains-story/v0-refused is not a contradiction); `28Rf` #10 (plan-edit TOCTOU — the
plan/consent model already owns edits; value-loss only); `28Rf` #6 (record-format churn
is `rul-strawman-formats-no-compat` territory, not a gate breach); `28Re` #5's
consent-conflation reading (availability computation runs nothing; entry remains the
consent-gated act — the gate-then-27C order is coherent, per `28Rf`'s own did-not-hold).

### The amendment set (proposed; awaiting human ack)

1. §1: the helper-plurality ruling (conservative option) + constants clause.
2. §2: the closure-identity definitional paragraph (rooting function; overlap truth
   table; membership semantics; resolution-keyed dedup) — and §9 + the oracle-side
   load-inert `.` amendment.
3. §8: gates rebuilt — plural-idiom differential fixtures commissioned FIRST (stage-i);
   records-diff at the stage-0 fold; runtime closure materialization + e2e in stage-ii;
   stage-iii gate carve + stdlib-revival dependency named; stage-ii split
   infrastructure/policy; stage-iii split mechanics/authoring/correlation.
4. §3: availability-domain paragraph (must/may; point-set windows; loop/join
   conservatism); wait-loop "modeled-pure" clause; ambient-host carve; host-identity
   failure-direction annotation; side-host entry policy + disclosure host coordinate.
5. §6/§7/§9/§10: ssh-entry-shape conflict named as owed ruling (cite `24T`);
   two-position sparing rule (PROPOSED conservative line); blessing/keep-verdict
   reconciliation routed to the human; zero-new-spellings re-scoped.
6. The small batch (timens; mapping table; umbrella-ACK scoping; §5 ledger addition;
   literal-plane amendment sentence; two-planes fence + PRE citation; USER_STORY
   wording list).
