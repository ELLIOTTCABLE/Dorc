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
- **fnd-lifecycle-contagion-vs-statelessness** (~SUSPECT; CHECK-ITEM) — Terraform's one
  lifecycle flag that *must* persist to state is exactly the one that propagates across
  the dependency graph (create_before_destroy), because selectively reversing edges
  makes cycles. If any Dorc incarnation-sensitivity marking turns out to propagate
  object-to-object, it will generate pressure against the nothing-persists posture.
  Stage-iii should look for this failure-shape deliberately rather than discover it.
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
