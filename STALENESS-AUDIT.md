(WORKING REPORT — AI-generated, temporary. NOT human-authored, NOT authoritative. Safe to
delete. Rev. 3, pruned 2026-06-09 at human direction: strictly the input-sheet for the
DESIGN/IMPLEMENTATION rewrite now — ops/housekeeping and already-handled material removed
(spike-2 merge mechanics and the stash verdict were communicated in-chat; kSILO landed in
KNOBS.md; small doc-chores moved to TODO-ADDTL). Rev. 1 was the chat-log mining; rev. 2 added
the round-19/spike-2 rulings and the 2026-06-09 live adjudications.)

# DESIGN / IMPLEMENTATION rewrite — collected inputs

What this tracks: where the human-authored root docs drift from the human's own later
positions, plus rulings that have no home in any root doc yet. Anchor fidelity, descending:

- f-live — verbatim quotes from the 2026-06-09 main-session conversation, marked `> human:`.
  Actually verbatim; the highest-fidelity anchors in this file.
- f-spike — human rulings recorded inline in the round-19 notes (`notes/19x`). Recorded
  mid-session by agents; phrasing is usually the agent's ("treat as authoritative direction,
  though their phrasing here is mine" — 19A §3).
- f-mined — rev. 1's chat-log/archive mining. Round-15 proved LLM-extracted "verbatim" quotes
  can be confabulated; reverify any f-mined quote before writing it into DESIGN.

Doc-trust reminder while rewriting: ANALYZER-NEEDS is a fallible agent-written index, not
canonical (human ruling, 196 §6); KNOBS is human-authoritative on naming only.

Headline: the largest owed sections are the grounding three-axis model (drift-grounding), the
two-halves probe model (owed-halfb), the observable/rc model (owed-observable), and the
value-plane reframe (owed-valueplane) — all currently living only in worktree notes and this
file.

---

## Adjudicated 2026-06-09 (live)

### drift-grounding — RESOLVED in frame; residue is spelling + collision/evolution semantics
- Cite: DESIGN.md "Inference limitations" (~479–541, ends `(UNSETTLED, CONTINUE)`).
- Resolution (f-spike 19A §5, human-ruled; reconfirmed f-live): command-keyed vs effect-keyed was
  a layer-conflation, not an either/or. Three orthogonal axes: the check/probe invocation is
  command-keyed (only the oracle can argparse its own binary — C-1); cross-oracle identity is
  named-kind-keyed (a coordination vocabulary, not an alternative to command-keying); the elision
  license is fact-converged-keyed. Unified: command-keyed oracles declaring effects on named
  kinds. The `(kind, provider, verb)` triple dissolves into this — provider+verb IS the command —
  so the triplet is not a competing "effect-keying"; later documents that treat kpv as a settled
  contrary frame (plans/17N, 175 read uncharitably) should not be re-opened as authority.

> human: I'm confused why models keep siezing on the kpv triplet; I suspect there's some
> context-poisoning going on there. My *own* mental model just sees "a map of effects" as a ...
> fairly simple thing?

- Already-decided sub-points, restated by the human 06-09 (f-live) for the kind-coordination
  problem: "no central authority, for sure; fail-loudly-fail-fast explicitly allowed, under the
  'dorc introduced this concept so we're allow to vomit over it' logic that doesn't apply to
  almost any other user-input; structure-must-match-else-vomit; and
  structure-must-exist-for-some-concepts."
- Genuinely-open residue (small, concrete — not a re-litigation of the frame):
  - res-spelling: the annotation/kind-declaration spelling (`fork-annotation-spelling`, 19H §4 =
    the kTYANNOT decision; see drift-language).
  - res-collision: collision semantics between independent authors — namespacing convention,
    plus the already-designed equivalence-declaration mechanism (oracle-declared `apt≡dpkg`) as
    the only cross-provider unifier.
  - res-evolution: structure-evolution over time (oracle v2 adds a selector; does a v1
    shape-match vomit?) — exact-match vs subset-match is undecided and retrofit-relevant.
  - res-curation: who mints the well-known kinds for the bootstrap oracles (social/curation, not
    technical).
- The blessed relational frame from rev. 1 stands unchanged: grounding is relational,
  referent-agnostic, declared-not-inferred; self-identity ("all invocations of `systemctl` share
  an arguments-shape") is the one hard anchor; co-reference is a weak hint (the
  `dpkg -s conflicting_package` counterexample); channel=instance / type=namespace-over-
  shared-global-state. (notes/095, human-adjudicated.)

### drift-applypar — ruling: no intra-host apply parallelization or reordering, ever
- Cite: nothing in DESIGN/IMPLEMENTATION promises it (the four-outcome lattice is silent on
  ordering); the overreach is in the LLM corpus — plans/076 §4, notes/073, and ANALYZER-NEEDS §I
  scheduling rows.

> human: I have *no* expectations of apply-phase parallelization; the value-add is supposed to be
> speeding the user up by apply-phase *elision*, and the way apply-phase *elision* can become
> faster than just-run-everything depends on *probe*-phase parallelization ... but never do I
> wish to enter the nightmare-category of trying to balance imperfect-mutation-oracles,
> global-state-tracking-analyses-for-mutual-shared-mutation, and
> user-expected-behaviour-(i.e. "the script they wrote")-modification. yikes.

- Consequences for the rewrite: state the ordering guarantee explicitly — within a host the
  book's order is preserved; apply-phase speed comes from elision only. IMPLEMENTATION's "no
  worse than just running the script blind" floor therefore holds without a reordering caveat.
  Scoping (AI-reading, unconfirmed): cross-host fan-out/batching (`serial:`-style rolling apply)
  and read-only probe-hoisting (kFLATTEN) are presumably unaffected.

### closed-corpusmeasure — ruling: the real-world corpus-measurement avenue is closed
- Supersedes the round-6 corpus-acquisition/kill-criteria thread (plans/063, 083, 086, 087) as a
  validation strategy. (Distinct from the spike e2e "corpus", which is alive as the take-3
  acceptance stick — don't conflate the two senses in new prose.)

> human: even if we spend a lot of time and effort slurping and measuring, two hard blockers make
> all that effort nearly valueless: A. I'm not a scientist, this is a subtle study with
> metholodical caveats and constraints, and I can't meaningfully resolve them in a way that
> builds confidence in my results ("we're measuring badly"); and B. we can't access "real" ops
> code at any scale (most homelabbers/users never push their runbooks to GitHub, basically;
> organizations *definitely* don't) - so we're "measuring the wrong thing" (... badly.)

- Rewrite action: Sensitivities #2 should absorb this — the analyzability bet is
  accepted-as-unmeasurable-in-advance; validation comes from building and dogfooding. The
  standing coverage question is sharpened by spike-2 (see owed-halfb): elision is gated
  multiplicatively on upstream oracle coverage (one un-oracled reaching mutator poisons
  everything below it), so the binding constraint is oracle-coverage + guard-handling, not
  analyzer machinery (193 strain-5).

### dir-soundiness-ux — direction: frontload the unsoundness; provenance as pedagogy
- The human's resolution-direction for the whole "soundiness is not what users expect" category
  — bears on Sensitivities, Contract & DX, and README marketing:

> human: I want to do a *better job* of frontloading the unsoundness, in extremely direct and
> tracable ways, both in documentation, 'README marketing', *and* in realtime, runtime behaviour
> and UX/feedback.

> human: "I cannot make mutation-never-happen" u "I don't want to give up on the high life-value
> of a system that avoids 99% of mutation" u "users won't believe me when I say 'be careful,
> there will be 1% mutation'" -> "I must do an *amazing* job of teaching and reminding users
> about the 1% mutation, and showing them *why* it happens to help them build an intuition for
> how a soundy tool like this works."

- Mechanism already seamed: per-line, at-decision-point disclosure rather than ambient
  disclaimers — claimed-vs-proven taint surfaced in the plan render, why-elided/why-probed per
  leaf, blame landing on the specific oracle line.
- AI-caveat (argued in-session, not ruled): the education budget is phase-asymmetric. Apply-side
  residual failures are attributable and recoverable — educable. Probe-side mutation during a
  phase sold as free is trust-catastrophic and education-proof (Chef why-run, Terraform #12489);
  that side needs the engineering backstops (sandbox, reflexive-inertness) regardless of
  pedagogy. kFAIL already encodes the asymmetry; the prose should too.

### drift-selfvouch — carried, with a fidelity and scope caution
- Cite: DESIGN.md "Inference limitations" (~489–497: `mycmd.check() { mycmd --dry-run "$@" ;}`;
  "by existence, an oracle vouches for itself"); contract list ~444–476.
- Carried finding (f-mined): self-vouch became a load-bearing contract axiom — probe default is
  withhold; a command ships into the probe only via the carve-out that a command inside its own
  oracle's check() is the author's self-declaration of dry-run inertness. (chat 1bfc0c47, 06-07;
  5b67ec3d 06-09 "we can never choose a safety-direction — oracles' choice".)
- Caution (f-live): the strong tail of the mined quote ("'testing' applied to oracles is
  effectively DX/lint — it provides no new information") exists nowhere durable, and the human
  himself queried its strength on 06-09. When DESIGN absorbs self-vouch, split the two claims:
  self-vouch is the soundness boundary (correct, keep); testing/calibration still provides
  confidence and remains the primary oracle-quality lever (kVERIFY-calibrate, the differential/
  container harness, 088's "lever on the #1 existential risk"). Do not let the strong phrasing
  suppress the calibration harness.

---

## Owed-to-DESIGN from round-19 / spike-2 (live only in worktree notes until merged)

### owed-halfb — the two-halves probe model; DESIGN's pitch-idiom is the unbuilt half
- The single most consequential spike-2 finding (196 §2, human-surfaced, human concurs with the
  promotion). "Probe → converge" has two halves. Half A (built): the oracle's read-only
  fact-probes ship; a converged mutator is elided. Half B (unbuilt, the core): probe projection —
  compile the probe from the book's own CFG, lift the book's own check-then-execute guards
  (`command -v nginx`, `dpkg -s …`) as read-only interceptors, and subsume the guarded branch
  from the probe's answer. The guard is never "elided"; it is executed read-only, early, in
  parallel. Corrected framing (19A §5/C-1): the probe is the union of the book's ordering+args
  with the oracles' declared check-bodies — nothing un-oracled is lifted, and the oracle (not
  Dorc) argparses.
- Sharp parts to carry into the probe/apply section:
  - DESIGN's own ten-minute pitch ("add a `if dpkg -s ca-certificates; then` guard that Dorc can
    analyze, lift, and probe", ~64–68) is Half B. Spike-1+2 built Half A. The dominant real
    idiom is handled by the half that does not exist yet.
  - F1 (the guard-status wrong-elision) is the engine screaming this: with no guard/query
    category, a read-only check is either un-oracled → Opaque → poisons, or oracled-as-establish
    → treated as a mutator → wrongly stubbed when converged. Under the settled model the guard's
    check() is itself the read-only probe and its rc is a normal probed value the fold flows
    (19H §2.2; 19I group E); the genuine prerequisites are the value-plane and the structural
    render (C-5).
  - The entanglement (~SUSPECT, durable law): the poison wall was accidentally protecting
    guards; every increment of elision-power surfaces latent wrong-elisions the poison was
    masking (196 §2).
  - Keystone verdict, both halves honest: the selector re-key works (the `apt-get update`
    false-poison is dead; the bare-mutation headline book elides 6 mutations, proven by
    execution) AND the full guarded realistic book still does not elide (two un-oracled
    neighbours — `case "$(hostname)"` and `if ! command -v nginx` — re-poison it).
    Oracle-coverage, not analyzer machinery, is the binding constraint (193 strain-5, 19I
    group J).

### owed-observable — rc is opaque, fully modeled, oracle-declared; one Observable
- The round's other center of gravity (19A §3+§5, 19B, 19E/19F/19G — human-ruled at the decision
  points). The model DESIGN should state:
  - rc is opaque to Dorc: hold the value, never interpret its meaning. Which observable values
    mean "converged" is declared by the oracle per command (C-4 refined: the oracle contract is
    fact-state → observables, not rc → verdict-directionality). No fabricated defaults — the 19D
    under-execute (fabricated `rc=0` collapsing `useradd deploy || mkdir /srv/app`, dropping the
    fallback) is the cautionary specimen.
  - The probe→apply handoff is abstract interpretation over probed observables: the probe yields
    concrete observables; the apply-CFG is folded over them; whatever cannot be resolved is ⊤
    and runs (kFAIL-perform). The C-2/C-3 "track rc-polarity" framing is dissolved — anything
    describing polarity-tracking as the model is stale.
  - Verdicts travel out-of-band (the $DORC_VERDICT lane): no exit code can mean
    "unknown/can't-probe", because every code may be a meaningful observable; signalling never
    shares a lane with freeform output (19B §2).
  - One Observable representation, structurally enforced (inv-one-observable, 19F/19G): the
    round's failure was three incoherent representations of one concept; the unification is
    half-landed and priority-1 for take-3.
  - A mutator's converged-observables are a separate oracle declaration, not the check mimicking
    the mutator's rc (opt-B, 19B §1). No `.valid()` verb — the single check() self-guards.
  - C-1: the interceptor passes the command's full argument string; Dorc extracts nothing.

### owed-valueplane — value propagation (not synthesis) is the actual core
- 19H §1 (reference-quality forward synthesis): a real constant + argument/parameter propagation
  analysis, across files, books AND oracles ("no useful book/oracle distinction at this layer"),
  is what entity-resolution-before-probe and observable-flow-after-probe both reduce to. Crucial
  distinction for the inference-limits rewrite: value synthesis stays refuted (16C); value
  propagation is decidable and required. Soundness floor is apply-direction only: stop anywhere →
  ⊤ → command runs; probe-inertness and propagation-correctness are separate, harder guarantees
  (19H crosscheck correction).
- The settled-mechanism statement for grounding (19G §2, human-confirmed): the engine does ZERO
  argparse; an oracle writes a mini-argparse in a constrained oracle-contract dialect (not
  arbitrary sh) and inline-annotates the operand's kind; the engine flow-tracks the book's
  constant through that argparse to the annotation.

### drift-verbset — oracle verb-set & fail-fast contract: decided, still unwritten
- Carried (f-mined; archive L498/1204/1208/1265; chat c9fce2bc): check = the narrowest correct
  primitive (default; not list-all-then-grep); diff = strictly-stronger, opt-in, must-earn;
  version verbs for gradual rollup; check() receives the full verbatim arg-string; fail-fast on
  unparsable flag; byte-mechanics deferred to jq/diff/patch — only the convergence-predicate is
  Dorc's altitude.
- Round-19 additions to the same owed contract: opt-B (separate converged-observables
  declaration); no `.valid()`; C-1 full-argv; the 17O oracle-quality regression class
  (`command -v` resolves to an executable file, `getent group` field-4 not `id`, per-database
  hermeticity, refuse `|| true`-masked rc as a verdict) as shipped stdlib-oracle tests;
  "blessing" = a stdlib oracle shipped day-1, not a separate mechanism.
- Open fork the contract section can state with the lean: fork-mutator-rc (19H §2.3) — an
  un-probeable mutator's rc is ⊤ ⇒ it runs ("no defaults, no values; only what the probe gives
  us"; cost = one lost elision) vs an oracle-declared fact-state → observable. Undecided.

---

## Carried drifts from rev. 1 (still owed)

- drift-version — versioning is absent from DESIGN. The founding co-pillar was self-demoted to
  out-of-scope; the two undesigned survivors (cross-PM version lattice; binary-content-hash
  grounding gate) are carried in TODO-ADDTL's MH2 item with the human's continuing-to-defer
  note. The rewrite should state the deferral rather than stay silent.
- drift-skip — "skip" is banned; elision is observable-preserving substitution. DESIGN ~223
  ("the more Dorc can skip"), ~391, IMPLEMENTATION ~143 still use the word. Built form now
  exists: value-preserving substitution `StandIn{True,False,Exit(i32)}` ("emit `9`, not `1`";
  `true` not `:`) — 19C. AGENTS.md already firms the terminology; DESIGN/IMPL lag.
- drift-language — superset is day-one non-optional (kLANG "sh IS Dorc"); must-handle welded
  (const-prop, interprocedural, variables, heredocs; "the one and only construct I am punting on
  is `eval`" — b820ef5e); kWHICHSH sub-axis; the Rust-`unsafe`-equivalent escape-hatch (loses
  CFG-totality, taints its control-flow-subgraph un-elidable) is in no doc. The kTYANNOT
  spelling is the named live fork `fork-annotation-spelling` (19H §4) — inline
  `pkg : Kind = "$1"` breaks the off-ramp under stock dash (17O F-OFFRAMP, verified) and wants a
  strip/transpile pass; eol-comment re-opens kOOB's no-comment-config.
- drift-realtime — two distinct requirements, DESIGN states neither: streaming remote command
  output (the founding anti-Ansible motivator, archive L73) and the live plan TUI (apply-lines
  greying as probe results stream in). C-5 (19A §3, human) firms the render direction:
  AST-structural + reconstruction-metadata, two outputs (minimal by-AST print; full textual
  reconstruction with grey-out). 06-09, human-blessed: the TUI must degrade into
  do-one-thing-well plain-text controls, with why-elided/why-probed as first-class primitive
  queries every panel composes over.
- drift-transport — lean moved from "pluggable/deploy-over-Ansible" to executorless +
  own-the-transport, push (4efae104, 0d9ad3df; plans/142). Push is ergonomic, NOT a security
  claim (bastion-hopping reintroduces multi-hop trust) — owed to README/DESIGN. The 19B OOB
  verdict-lane decision is this architecture's first concrete piece.
- drift-state — DESIGN ~174–182 "may short-term-persist" vs the human's stateless-recompute lean
  (6b89f9ee: rust-analyzer-style, recompute from on-disk/host truth; kSTATE genuinely open).
  Decide the verdict-shape now even if built later.
- drift-depinfer — inference ceiling: explicit task-deps must be includable (archive L608-616;
  Ansible-style dumb dep-tree OK); static-derive + runtime-trace are complementary
  (kDEPS-accept-partial). 19H's value-plane gives this its precise statement — propagation in,
  synthesis out.

## Lower-firmness owed

- owed-delta — run-delta convergence class (restart/reload-iff-changed; notify/handler):
  convergence is on a run-delta, not host-state → un-probeable; elide only by preserving the
  author's own change-flag dataflow, else over-execute; never via state-probe; never synthesize
  the cross-kind edge. Human-dispositioned; already in TODO.md; belongs in the probe/apply
  model. (17O R2-CHANGEDELTA.)
- owed-prov — N-tier per-host-forking provenance DAG: human-seeded direction, mechanics never
  firmed; DESIGN ~417 concedes OOB error-metadata only. Name it without over-claiming.
  (notes/110/111.)
- owed-platform — tier-A/tier-B sh-precondition targets, CRLF-normalize-on-ship, kWINLOCAL
  (plans/139). An addition, not a contradiction; DESIGN/README silent.
- owed-dst — IMPLEMENTATION's DST section is current; postdated by: the single nondeterminism
  seam = controller↔host transport; "'best effort' isn't an escape-hatch … provable-algorithms-
  levels-of-best" (plans/128; 4efae104); and the acceptance discipline both spikes re-proved —
  execute or `sh -n`-check rendered artifacts, never text-diff them (the ap-2 trap shipped
  non-runnable output green twice). The last belongs in IMPLEMENTATION's correctness section.

## Verified non-drifts — don't over-rewrite these

The four-outcome execution lattice (IMPLEMENTATION ~128–186) is current and load-bearing
(round-19 invoked it for every priority-1 find). eBPF/ptrace correctly sits in DX-tooling, not
core. Rollback/undo is correctly absent (founding drop: a correct inverse is undecidable;
untested undo is worse than none). The two-users framing and the off-ramp passages are present
and faithful — available minor sharpening: the two contracts are duals, lossy-deployer
paid-for-by-strict-engineer. The five-minute-pitch passage stays, noting its ten-minute example
names the unbuilt half (owed-halfb).

---
Provenance: rev. 1 = 4-agent chat-log/archive mining + grep verification. Rev. 2 added an Opus
subagent full-read of the round-19 notes (193–19G, 19H, 19I) + main-session reads of 196/19H/19I
+ the 2026-06-09 live adjudication (f-live quotes verbatim). Rev. 3 = pruned to rewrite-inputs
only; removed material is preserved in git history.