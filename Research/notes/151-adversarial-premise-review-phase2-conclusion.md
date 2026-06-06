# 151 — adversarial premise-review: phase 2 (cross-cuts) + round conclusion

> **Status (2026-06-04): round-15 conclusion.** Phase 2 of the adversarial review-round
> (phase 1 = `150`). Four clean-context cross-cutting subagents read *across* the 14 rounds
> (digested with phase-1's findings as OOS, so they synthesize emergent/systemic faults, not
> re-derive) — feasibility-retuned per the human steer: go/no-go is YOLO-GO, so `M1`/`M2`
> (self-killing, value-band) were OOS'd; the target was **"meta-poor planning that wastes
> engineering effort."** X1 soundness-coherence · X2 MVP-entanglement · X3 oracle-mechanism
> feasibility (cold-start OOS'd per the triangle reframe) · X4 the Xpbd strawmen worktree.
>
> **Not truth — wider coverage.** Convergence across the four (and with phase-1) is the
> signal; the verbatim quotes below are the agents' words, preserved for fidelity per the
> human's request, *not* endorsed as fact. Several findings were **empirically run** (X4
> executed the strawman sh under dash/bash) — those are marked and are the most trustworthy.

## THE CONVERGENCE — all four cross-cuts point at ONE unwritten artifact

The dominant phase-2 result: **the entire analyzer build hinges on a single contract that
has never been spelled — the sh idiom by which an oracle (a) NAMES the kind its predicate
serves, (b) anchors a sound skip, and (c) reports its verdict — and four independent rounds
reach it, depend on it, and defer it.** This is the highest-leverage feasibility item: it is
cheap to settle on a whiteboard and ruinous to discover mid-substrate-build.

- **X1 (soundness):** *"three load-bearing preconditions (entity-uniqueness, anchor-spelling/
  completeness-recognition, verdict-trust-per-phase) were each established in one round and
  consumed in another, with no round holding both ends. All three converge on one unwritten
  artifact. Write that artifact first."*
- **X3 (oracle-mechanism):** *"the plans never spell the sh idiom by which an oracle NAMES the
  kind its predicate serves — and that one unspelled token is the hinge the entire
  cross-oracle/m×n mechanism hangs on … The corpus has named this gap four times
  (DESIGN:199, 090:377, 099 §9, 09A:102) and deferred it four times. Deferring a hinge is the
  meta-poverty: every round adds machinery … that assumes a grounded named-kind exists, while
  the one mechanism that would produce a named kind from author-written sh remains a TODO."*
- **X4 (strawmen):** the strawmen are the first attempt to spell it; they *"cleanly spell the
  guard/host/change families and cleanly refuse to fake the effect/kind/entity families"* —
  i.e. the easy half is expressible, the load-bearing half is the unwritten hinge.

**The de-risk (X3, the most important positive of the whole round):** the contract is **not
proven impossible** — phase-1 `fN-NOTATION` stated it as a possibly-unsatisfiable
constraint-conjunction, but X3 shows the conjunction is **over-tight**:
> *"kOOB's redline is 'user-configuration form, not metadata transport' … an analyzer that
> lifts provide/equivalence facts from oracle ASTs into an internal map keyed by leaf-id does
> NOT violate kOOB … The registry the design 'rejects' is a baked-in maintainer-arbitrated
> list of Every Thing; an analyzer-built index of user-authored declarations is a different
> object and is permitted."*
So a buildable path reopens: author-written sh declarations, lifted statically into an
**engine-internal, kOOB-legal kind-index**. X3 is ~SUSPECT (not SURE) that this is what the
human *intends* — "the permission exists, the decision doesn't." **Recommended action: a
half-day strawman settling the kind-naming idiom, before engine code.** (X1: this same
strawman also answers "what licenses a strong update?" — SF-1/SF-2 below collapse to it.)

---

## M3 · ACCRETION IS OUTRUNNING RECONCILIATION  (frontloaded; verbatim)

*The human asked for fidelity + bandwidth here.* Later rounds silently overturn earlier
"welded / day-1" decisions; the "decide-now retrofit-hostile" list is internally inconsistent,
so building to it means building to conflicting specs. Phase 2 sharpened this from "three
contradictions" to a **single physical knot**.

- **X2's core image:** *"The retrofit-hostile commitments do not form a clean DAG — they form
  a **knot at one physical point**: the controller↔host leaf-execution session. Five 'day-1'
  seams all land on or pass through it"* (kFIDELITY leaf-wrap · DST L0 transport seam ·
  provenance marker reconstruction · kCOMMS transport · async executor shape).
- **X2 THE-ONE (the concrete core of M3):** *"kCOMMS transport is the undecided producer of a
  marker/metadata protocol that four 'day-1, retrofit-hostile' seams are already defined to
  consume … 142 (round 14, 2026-06-04 — the newest document in the corpus) is explicitly a
  'plan-and-gate, not a settled question.'"*
- **The unreconciled contradiction, located:** *"128 rob-1 asserts 'banking L0 presupposes
  neither kCOMMS pole,' which is true only for the DST mock-point and **false** for the three
  other consumers that must agree on the protocol's shape … a wrong placeholder protocol
  re-shapes the leaf-runner, the provenance parser, the DST synthesizer, and the executor
  framing together (~every component except the analyzer kernel)."*
- **The driver is itself parked (X2):** *"whether you can stay executorless is driven by kCONC
  (intra-host probe concurrency), which is parked and un-sized … 140 f15: 'The executor is
  forced only by live AND concurrent together — precisely the probe's now-stated want.' So the
  transport pole is downstream of a corpus measurement nobody has run."*

The three phase-1 contradictions that feed this (verbatim from the per-round agents):
- **Provenance vs optimizer (R11):** `plans/077:20` — *"the probe optimizer … **destroys the
  1:1 leaf↔execution mapping** … a batched `dpkg -l` covering 40 checks, or an elided leaf,
  can't be attributed back to one source line."* Yet `plans/111` bills rich provenance as
  recoverable from the optimized stream. ⇒ rich attribution is welded to the *slow* probe.
- **DST cost (R12):** *"the 'DST is cheap' verdict rests on two mutually-exclusive cost-dodges
  … the cited state-machine sidestep (Polar Signals) works precisely because it is **not**
  async ('no async keyword … no runaway futures'). You can bank one, not both. Banking L0
  therefore forces an irreversible async-vs-state-machine kernel decision now."*
- **Transport vs agentless (R13):** *"agentless ∧ dumb-host ∧ live-concurrent-probe are
  mutually inconsistent … the note is being driven toward an executor it elsewhere swore off
  … without acknowledging it overturns the welded kAGENTLESS + plans/111's dumb-host carrier."*

**Effort-blast-radius:** highest of any finding. **De-risk:** resolve `kCONC` + the
`ax-executor` fork *before* (or as the first act of) the `do-4` spike — X2 notes *"the spike's
thin executor and its marker emission already commit a kCOMMS shape by accident, and 088 §4
doesn't flag that it's doing so."*

---

## M4 · CITATION INTEGRITY  (frontloaded; verbatim)

*The human asked for fidelity + bandwidth here.* Load-bearing citation/evidence faults the
2026-06-04 self-audit structurally missed (it deprioritized interpretive/analogical claims +
several source tiers — exactly where these sit). Building on a fabricated or misread premise
wastes effort, so this is feasibility-relevant, not mere hygiene.

- **Fabricated quote (sharpest; R5, verbatim):** *"`plans/deferred/078:9` attributes 'the
  mechanism finding that **shapes everything**' to a verbatim rattle README quote — 'if
  `fsatrace` runs a binary that goes through a server process not spawned by the build system,
  it won't be visible.' That sentence is in **neither** the current rattle README **nor** the
  fsatrace README; it does not surface in web search … The phrasing was fabricated or heavily
  paraphrased and then dressed with the strongest epistemic marker ('confirmed verbatim'){.}"*
  And: *"the corpus's own source-audit … never audited this 078 quote … the self-correcting
  machinery structurally missed the recovery round's two most load-bearing factual claims."*
  (The *conclusion* — daemons defeat per-process tracing — is independently true; the
  *evidentiary anchor* is invented.)
- **User-study read against source (R3, verbatim):** the "drop to a shell script" claim —
  *"the practitioner escapes to shell to test/debug in isolation, then brings it back into
  Ansible … the paper's named exemplars (Pulumi, Nix, Jetporch) are the opposite design pole
  from 'POSIX sh is the substrate.' Dorc reads the paper's 'go beyond config languages' as
  endorsement, but … takes a debugging-workaround and an argument-for-real-languages and reads
  both as endorsement of shell-as-platform."*
- **k-CFA term-equivocation (R2, verbatim):** *"the round repeatedly says 'flat-fact-map →
  OO-like → polynomial,' conflating 'flat environment representation' (the paper's mechanism)
  with 'flat domain of facts' (Dorc's data model) — these are not the same thing … you get
  polynomiality precisely by collapsing the precision the dial was supposed to buy."*
- **SHA-pin self-contradiction (R10, verbatim):** *"the team has imported a source
  ([B-nesbitt-quacks-package-manager-2026]) whose central supply-chain lesson is 'SHA-pinning
  the top level buys you little against transitive compromise,' and then prescribed top-level
  SHA-pinning as the integrity mitigation without engaging the transitive hole."*
- **PlanBouquet non-transferable bound (R11, verbatim):** *"the bound's discovery mechanism is
  repeated re-execution of the query … That is precisely what Dorc's kFAIL-withhold forbids
  the probe from doing. So the mechanism that earns the bound is illegal in Dorc's probe
  phase."*
- **v1 contradicts its own source (R7-8, verbatim):** `do-2` ships git-diff elision as v1, but
  `notes/075:27` (the source it cites) says *"the git-diff should **never** be the skip
  authority … the probe is."*

**Effort-blast-radius:** medium, but compounding — a fabricated/misread anchor that survived
the self-audit means the audit's "0 EGREGIOUS" is a *floor over a scoped set*, not a clean
bill; any of these becoming load-bearing in a build decision wastes the effort spent on it.
**De-risk:** a second citation-integrity pass over the interpretive/analogical tier + the
docker/tracing/transport source tiers the audit skipped; treat `078`'s tracing rationale as
unsourced until re-grounded.

---

## Other sharp cross-cut findings (feasibility-framed)

- **X1 · SF-1 (strong-update keystone, SURE):** *"strong-update requires proving two
  state-mutations target the same unique entity … Under the referent-agnostic model, the
  analyzer holds only opaque tokens whose identity is the very symbol-grounding it has
  disclaimed … The keystone that makes the floor useful is the one operation whose misfire
  makes the floor unsound, and its precondition is welded-undecidable."* Effort: high — it's
  the engine substrate (kFACTS/kCONTEXT, high lock-in); building elision on token-equality
  strong-updates risks a substrate re-pour. Collapses to the same strawman as THE CONVERGENCE.
- **X1 · SF-3 (three-valued verdict crosses the kFAIL boundary, SUSPECT):** *"kFAIL is welded
  phase-keyed (two opposite ⊤s) — but the three-valued verdict is a single artifact crossing
  both phases, and a single three-valued lattice cannot carry two opposite fail-orientations
  simultaneously. The weld that 'holds in the abstract' develops a hole the moment a third
  value crosses the phase boundary it was keyed to separate."* + the R10 host-as-adversary
  closure (forged verdicts → silent skip) lands on the same un-owned seam.
- **X4 · empirically-run oracle bugs (SURE — actually executed):**
  - *ufw `.`-as-regex:* *"rule `10.0.0.1` passes the sanitiser, and against a status line
    `10X0X0X1 ALLOW IN Anywhere` the `.`s match the `X`s → false-positive → return 0
    (converged) → Dorc WRONGLY skips adding the firewall rule"* — a silent `kFAIL-perform`
    violation from a *paranoid* author's defensive sanitiser.
  - *apt-get `-o` guard leak:* *"`--option=Foo=bar` LEAKS THROUGH … could defeat the very
    simulation the oracle relies on for non-mutation"* — a `kFAIL-withhold` (never-mutate-in-
    probe) violation.
  - *oracles aren't POSIX:* all four `.straw.sh` oracles **fail `dash -n`** ("Bad function
    name" at the `cmd.check()` line); the "dash-clean / runs as-is" claim is false until a
    mechanical rename. The off-ramp survives "as a rename, not as-is" — on the oracle side.
  - **The lesson (X4):** *"hand-rolled sh arg-parsing in oracles is a recurring soundness hole;
    the fN-NOTATION contract cannot be carried by author-written `case` globs alone … the
    contract needs machine-enforcement, not author-discipline."*
- **X4 · inference handle ⟂ input quality (SURE, the THE-ONE):** *"the scrappy `pi-webhost`
  book with bare top-level `systemctl enable --now nginx` is richly inferable, while the
  careful, correctness-heavy `deploy-widget.sh` — functions, `readonly` constant-folding,
  temp-staged atomic `cp`+`mv`, heredoc-sourced desired-state — drives the apt-get-style
  transmute straight to ⊤ … Every cheap inference handle is exactly what good structure
  removes. The team's stated highest-value audience is the engineer writing correctness-heavy
  oracles and the careful scripts they wrap — and that is precisely the input where the
  spelled inference no-ops."* Effort: very high — the cheap optimizer fires on the inputs that
  matter least; the careful inputs need the deferred interprocedural + constant-fold + heredoc
  path. (Note the tension with the triangle reframe: the "homelabber-to-author pipeline" lifts
  value via *better* scripts, but better-structured scripts are *harder* to infer from — the
  pipeline's success may erode the cheap-inference win unless the interprocedural path is built.)
- **X2 · verdict (thin slice):** there IS a clean go/no-go probe slice (`do-2` cheap git-diff
  elision needs only parser+diff; `do-4` is disposable-by-design) — but *"there is no safe
  intermediate"* executor between serial-one-box and the Graham-aware rich scheduler, and
  *"first durable value requires the leaf-seam + transport-protocol + hermetic-oracle-contract
  + non-naive-executor knot, and the most retrofit-hostile member of that knot (kCOMMS) is the
  least decided thing in the entire corpus."* Plus an unreconciled framing: *"a disposable
  artifact whose seams are all 'design-it-in-or-never' is a contradiction in scoping."*
- **X3 · footprint isn't a one-liner (SURE):** *"the footprint of `cp` is a function of parsed
  argv, not a command-keyed constant … each command's footprint oracle is a mini-parser
  tracking option semantics. It is authorable and stays on the IFDS floor, but the
  effort-estimate 'dumb one-liners' is wrong by a large constant factor."*
- **X3 · m×n naming is a 3-place relation in a 1-place namespace (SURE):** *"a naming
  convention `package__probe` gives each kind exactly one global function slot per process —
  sourcing apt's then brew's oracle silently clobbers the first … No sh naming scheme expresses
  a three-place relation (kind, provider, equivalence) in a one-place namespace."* → forces the
  analyzer-internal index (the de-risk above), not a naming convention.

## HELD-UP (de-risking — genuinely sound, don't burn effort here)

- **The within-script single-anchor narrowing contract IS buildable as pure no-op sh** (X3
  held-1, with running strawman): `pkg_present nginx || apt-get install -y nginx` — pure POSIX,
  behavioural no-op, machine-recognizable (occurrence-typing triple). The guard/host/change
  families are real, idiomatic, off-ramp-clean (X4 held-1/2; host-selection via
  `case $(hostname)` / `case $(uname)` is unambiguous).
- **Provenance is genuinely NOT lock-in-y** (X2 held): build it crude, improve freely —
  phase-1 over-counted it as a day-1-hostile item.
- **`kCONTEXT` has a free retrofit-safe default** (k=0); Tier-B analyzer + `kFACTS` substrate
  are genuinely deferrable; language choice is decoupled behind the serialization seam.
- **The strawmen's self-honesty is a feasibility asset** (X4 held-4): every `HOLE(ceiling)` is
  left empty and flagged, never faked — the team will not waste effort building toward
  provably-uninferable facts.
- (Carried from `150`) impossibility-ceilings used correctly; no soundness proof claimed;
  analyzer scalability + network-ceiling engineering solid; the `result × [diagnostics]`
  data-shape; `kLANG` "second input = new product."

## Recommended effort-ordering (feasibility actions, not design decisions)

1. **Write the kind-naming/anchor/verdict-channel contract strawman BEFORE engine code.** It
   is the hinge all four cross-cuts converge on; X3 shows the likely-buildable shape
   (analyzer-internal kОOB-legal index) and the human need only *decide* it. This one artifact
   simultaneously settles SF-1 (strong-update licensing), `fN-NOTATION`, the verdict channel
   (X4 find-3 rc-2 overload), and the modeling-completeness gate (SF-2).
2. **Resolve `kCONC` + the `ax-executor`/`kCOMMS` fork before the `do-4` spike** — the spike
   commits a transport shape by accident otherwise (M3).
3. **Add the dash/bash `set -e`/`trap` differential to the spike harness** — phase-1
   `fN-SILENT`: it's named in `087` §3c but absent from the `088` harness scope.
4. **Treat the oracle contract as machine-enforced, not author-disciplined** (X4: paranoid
   authors still ship silent wrong-skips); budget per-oracle effort above "one-liner" (X3).
5. **Second citation pass** over the interpretive + tracing/transport source tiers (M4);
   re-ground or retract `078`'s fabricated anchor.

## Caveats (standing)

- Adversarial-only; convergence is the signal, singletons (SF-3 verdict-boundary, R10
  host-adversary) are suspect-until-checked. The four agents were paid to be hostile; the
  arbiter (me) is Dorc-curious. X4's empirically-run findings (ufw regex, apt-get `-o`,
  `dash -n`) are the most trustworthy (executed, not argued).
- `M1`/`M2` (self-killing, value-band) remain recorded-but-de-prioritized per the YOLO-GO steer.
- Round artifacts: `150` (phase-1 per-round) · `151` (this). No `plans/` synthesis written
  (per human: no research-planning doc for this round).
