# 26M — pivot-book language-surface sitting (live ledger)

> AI-authored (Fable, live design sitting WITH the human, 2026-08-31); the human
> adjudicates in-chat. Conversation-tier EXCEPT the ack-ledger — only what the human
> TYPED counts. Continues the 26L/26Lb meta-orchestration saga toward the concrete
> feature surface: analysis of pivot/conductor books (controller-scoped books whose
> lines denote other machines). Governing: root docs · `26K` §0b (the chartered
> kernel sitting this partially discharges) · `plans/27C` · `plans/30I`/`30P` ·
> `plans/24T` · `notes/274`. Grounding read this sitting:
> `.claude/research/ops-glue-residue/` turn03 + round-charter (§epoch, §turn-B/C) ·
> `notes/061`/`062` · `plans/09A` · the `r26-glue-strawmen` companion notes.

## Ack-ledger (human-TYPED, 2026-08-31)

- **ack-one-scope-per-book** — the `26K` interim ruling ACKED outright: a pivot is
  sh-spelled or it is nothing; a native non-sh ordering/pivot/orchestrator-control
  construct is ~nope. NOT a weld against LIFTING sh-spelled orchestration; the
  human expects native-orchestration and lifted-sh-orchestration to meld.
- **ack-cross-host-facts-scoping** — the gate-never-license lean is SCOPED to
  native orchestration features (which include per-host *different* books). Within
  a pivot/local book, cross-host dataflow is ordinary sh and largely
  un-preventable; punt. A security/threat-model review (opaque-review-tier) of
  pivot books' cross-host influence is OWED and out-of-scope for this design work.
- **ack-carrier-form-neutrality** — adjacent-book dispatch (`ssh h <web.sh`) and
  inline payload descent (`ssh h 'cmd; cmd'`, heredoc forms) get EQUAL priority
  and are built together; the form is authoring comfort, not a correctness
  boundary, and neither may be artificially imposed. Both reduce to ONE engine
  core: the payload-declaration speech-act (a refag tool-oracle saying "this
  input of mine is also sh, running <somewhere>, with <properties> letting you
  roll it into analysis as a book-component"). The prior rung-2/rung-3 ranking is
  DEAD.
- **ack-hosts-blessed-someday-fenced-now** — hosts are expected to become the
  SECOND engine-blessed type with special semantics (sibling of File/redirects:
  File is forced by sh semantics, Host by security/transport semantics). FENCED
  off the language-design surface for now: pivot design proceeds fully
  agnostically, without "knowing" what hosts are — while not precluding the later
  blessing.
- **ack-authored-host-sameness-parallel** — hostname sameness/resolution wants
  resolve-class authored machinery: offload what we can to the knowledgeable
  admin in plain sh, even once refag is later breached for probe emission and
  host authorization. Conductor rider (NOT acked): the knife-direction INVERTS
  vs kind-resolve — a wrong host-MERGE under-executes (cardinal-sin direction), a
  wrong split only duplicates probing — so prefer authenticated witnesses
  (host-key equality is a measurement) over authored merges, and treat an
  authored merge as a survival-grade claim. `forfeit-no-host-merging` stands.
- **nack-containment-nesting** — the conductor's "havoc propagates down
  entry-chain containment for free" is NACKED pending opaque-review;
  out-of-band reasons may INVERT refag leans on some multi-host territory. Do
  not bake nesting assumptions into the language surface.
- **ack-terminology-no-locus** — the 'locus' mint is withdrawn (near-content):
  the pair is host × epoch; traditional terms (incarnation / generation)
  preferred for the temporal index. Standing kin:
  `28Q:rule-incarnation-continuity-semantics`.
- **suspicion-transit-reach-subsumption** — the conductor claim "a fired
  transit's invalidation is entirely carried by existing reach machinery" is
  SUSPECTED incomplete (human: requires counterexamples, not vibes; defers to
  the granularity item). Conductor pocket counterexample for that hunt:
  self-transit — a transit of the world the executing artifact itself runs in
  (controller reboot under local-exec; `reboot` inside a host's own dispatched
  payload; the nixos-anywhere kexec specimen) is session/continuity territory
  that reach never models.
- **ack-command-position-constant-prop-owed** — `$SSH`/`$SUDO`-style
  prefix-variable head resolution is owed engineering; set aside as too narrow
  for the sitting.
- (The apply-side-transport exploration is NOT ack-ledger material — nothing
  ruled; see the tabled section below.)

## Sitting model-state (conductor synthesis, human-read; cite nothing as ruled)

- Epoch-identity model (presented, unobjected): epoch identity is a
  comparison-WITNESS now (host-key continuity at `260` §5 is the built instance;
  boot-id the finer future one; unplanned churn ⇒ withhold, per the
  integrity carve) · narrative always · a coordinate slot ONLY when
  invariance-carry (`undivided-by-transit-across`) arrives · a hard precondition
  for any future cross-run state. Not a licensing key at this tier.
- Probe/apply transport asymmetry (~SUSPECT, load-bearing if it holds): the
  APPLY never re-transports — the author's own dispatch line executes the
  (in-place transformed) payload; the byte-floor holds. The PROBE constructs
  entries (the `27C` pattern: the oracle's authored entry recipe) to measure in
  the denoted world. Entry cost therefore lands probe-side, plus on inserted
  guards riding inside the payload.
- Who-holds-the-bytes line (~SUSPECT, clean): analysis descends only into
  CONTROLLER-HELD authored bytes (book text, adjacent files, heredocs);
  target-resident payloads (`ssh h sh /remote/path`) are world-state, reachable
  only as facts (the `FORFEITS` content-facts rows), never as source.

## Explored & TABLED (2026-08-31): apply-side transport participation

Explored in-chat at the human's direction ("prove me wrong"), then gently tabled.
**NOTHING RULED.** The human's standing lean is, at minimum, economic: no further
machinery is buildable now or soon. Revival wants that constraint lifted plus the
owed pivot security review in hand.

- **The question:** does "the apply never re-transports" hold as a weld, or do
  ssh/scp-class wrappers graduate to the `.`-tier (engine-modifiable under
  derived permission; bytes-ship-unmodified relaxed for those sites)?
- **The buys-list landed stronger than expected** (human: "concerning to turn
  down"; the correctness/safety half "very nearly shifts my position alone"):
  guard-parity at dispatch sites (form-neutrality × `kHALVES` — the inline form
  loses the guard tier entirely without at least argv-writes) · per-leg
  fail-direction (sh structurally cannot distinguish rc-255 sever from remote
  failure) · per-leg identity continuity (the surrender pattern is the wild
  norm) · handshake/touch economics at hundreds-of-dispatches scale · transfer
  (scp) write-if-changed · relay/quoting construction. Conductor concession: the
  pivot strawman's own §2 ("a contiguous run denoting one scope is one artifact
  on one connection") had already silently assumed re-transport.
- **Human-typed cost-side nacks** (durable, verbatim-tier): (1) *the middle is
  the worst place software can stand* — interposition makes both learning
  burdens (users-learn-us AND we-learn-their-space) critically owed; (2)
  *`kBACKFLIPS` IS authorship's strongest argument* — correct generation and
  composition of sh/ssh-arguments from arbitrary input, code-motion between ssh
  inner bodies, is nightmare-tier Rust engineering demanding deep ops-ocean
  knowledge, with none of the oracle lane's extendability/attribution/
  fix-it-yourself; (3) *refag dies at mechanization* — the moment the engine
  applies tool semantics (add/remove `-o ControlMaster`) it is locked to
  specifically-`ssh` (not mosh, not serial-over-comport), and the semantic
  cannot be pushed down into user-authored abstractions.
- **The inflection point** (conductor formulation, un-ruled): the meaningful
  yes/no is whether the engine ever writes anything that is not
  *payload-text-through-a-declared-fidelity-function*. Below the line, writes
  are licensed by the taught surface (oracle-carried, attributable,
  reconstruction-differential-checkable); above it, writes enter the tool's own
  semantic space (flags, connections, topology) — engine-baked tool knowledge,
  where all three nacks bite at once.
- **The witness lane** (human-sketched, conductor-developed): a SECOND
  connection — the existing probe machinery granted an apply-time observational
  seat — while the author's own invocation/tunnel stays sacrosanct.
  Re-measurement-as-truth does most of the fail-direction work (an apply's rc is
  not state-truth — `260` §4); continuity tokens re-check on Dorc's own channel;
  the whole lane is aid/integrity-plane, lawful by construction under
  `law-two-planes-opposite-fail`. One reserved corner: witness-detected
  incarnation change mid-apply × the integrity-withhold carve = possible
  kill-the-book authority — wants its own ruling. Economics stay unpurchased and
  displace to hints-in-the-user's-idiom (ControlMaster is the admin's own
  spelled mechanism; hint, never do).
- **The hole the tabling landed on** (human, load-bearing): the witness lane
  quietly presumes SYNCHRONIZATION — "after it failed", "when it comes up".
  Correlating a parallel channel with book-position needs either injected
  markers — and ~DREP-writes at transit boundaries may be the smallest
  user-byte-mutation that is still user-byte-mutation-shaped — or coarse
  granularity. So rung-3-ish wrapping is ~necessary for the PARALLEL witness,
  and the live question becomes whether the parallel channel keeps value over
  injecting direct actions at those same now-being-edited positions. Conductor
  split, un-adjudicated: the lane bifurcates into a MARKER-FREE half (artifact
  boundaries + the driver's own child-exit/stall as the sync events — standup
  checks, post-mortem diagnosis, final verify; cheap, still real) and a
  MARKER-NEEDING half (mid-book granularity), which re-opens the byte question
  exactly as stated.
- Guard-tier-at-dispatch-sites: explicitly OPEN, punted (the human considers it
  orthogonal to the transport/topology question that drove this exploration).

## The road to `plans/30W` (second half of the sitting, 2026-08-31)

The mechanism's design-of-record is **`plans/30W`**; this section keeps only the
trail — the adjudications and superseded strawmen that got there, so the corpses
stay visibly buried.

- **The necessity adjudication.** The conductor's v1-posture ("transits are total
  honest walls; the C-kit ships cheap-plane") was challenged for counterexamples.
  Taxonomy found: Class A — the routinely-firing mid-book reboot inside the
  patch-day everything-book (structurally the stale-index morning that bought
  stages 5–7, firing more often, havocking more); Class B — the transport
  self-transit (ssh-port/firewall change; near-empty real footprint, maximal
  tail); Class C — staged-state-crossing-by-design (grub/cmdline, RunOnce,
  kexec; walls sever the book's own causal spine); Class D — data transits
  (pg_upgrade class). Human dispositions: Class C most motivating
  ("product-appearance murder" if unmodeled); Class A softened (reboots are
  guarded and authors expect multi-invocation; a transit only bites BETWEEN
  sites in a HIGHER context — a script cannot outlive its own world's reboot);
  Class B held open. The B/C user stories surfaced the **license-vs-narration
  split** (B wants elisions kept = license-plane; C wants witnessed causal
  narration across the havoc = aid/integrity-plane, receipt-narration legal) —
  human nit accepted: that split is economics, not principle, and the two
  analyses converge in shape. Human CORE argument, typed: expressiveness — the
  ability to write correct purpose-ordered pivot books (transport-standup
  FIRST, whole book below it) while extracting any value "seems fairly locked
  behind survival"; unsold on deferring the license plane. The conductor's
  sequencing case (C-kit as substrate + sizing instrument first) was then
  largely OVERTAKEN by the 30W collapse shrinking the license build itself.
- **The spelling iterations, each superseded in turn:** axis-as-new-grounding
  (danger: the symbol-grounding problem replayed over havoc-classes; enum-baking
  named the trap) → transit-class naming ("axis" conceded a bad mint) →
  region/incarnation recast with an `outlives` kind-member — naming rulings
  banked: `survives` REJECTED (collides with the survive mechanism), `outlives`
  per the Rust/region-calculus relation, GC "generation" flagged a false friend,
  "incarnation" acceptable (kind = region-class, entity = the incarnation) →
  the human's kinds-rhyme insight (coordinate/grounding reuse;
  entities-as-incarnations; continuity as generic state-assertion) → the
  cohabitation exercise (findings: DUAL SPEAKERS, not shared verbs;
  exits-never-inferred, forced by every-dispatch-ends-a-session; Session's
  value is cell-scoping not havoc; File-as-region is real-but-unblessed — the
  reload phenomenon) → the human-directed collapse experiment: rebuild on
  EXISTING verbs only. Result: `disturbs` + `state_stored_only_in` + `reaches` +
  finished-definitions suffice; the one lift is the abstract **Store role**;
  even expected-sever derives (a footprint touching the store backing your own
  entry path). Every interim spelling above (`: transits <axis>`, `outlives`,
  region-declarations) is SUPERSEDED by `30W`'s form.
- **The two adversarial checks (human):** (1) namespacing → names are
  view-relative, identity is chain-rooted, minted-unique entities bridge;
  view-aliasing (chroot/bind-mount: same cell, two names) split from
  store-relativity (two chains, one string); resolvers inherit
  measurement-in-denoted-context. (2) "time is not in the model" — RETRACTED as
  overclaim; the honest form: event-time dissolved into identity+order, METRIC
  time authored-or-parked with named reentry points (`30W` law-event-time-only).

## Queue (live)

1. `q-payload-declaration-speech-act` — OPEN, the sitting's standing focus:
   custody-over-there drill · carrier-geometry survey · inline-render posture ·
   the fidelity-normalizer spelling (human-directed: an authored, pre-applied
   normalizer function in the closed dialect, NOT a closed token class).
2. `q-store-algebra-ratification` — `plans/30W`'s open asks: the stored-in
   coverage burden · the two engine extensions' pricing · the stdlib store-kind
   roster + incarnation-cell reads · the §5 law-forms.
3. `q-entry-economics` — check-tax through constructed entries; probe batching
   per (world, context); feeds `slow-planner-cost-model`.
4. `q-prefix-head-resolution` — parked (owed, narrow).

(`q-granularity-partial-havoc` DISCHARGED into `30W` — the reach-subsumption
rider's residue is the self-transit/expected-sever cell, now derived at `30W`
§2.4; `q-entry-spelling-strawmen`'s havoc half likewise discharged, its
payload-declaration half absorbed into item 1.)
