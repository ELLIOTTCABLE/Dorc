# 23N — Prior-art grounding for the golden-hill (elide) mechanism

*AI-authored synthesis; NOT welded; confidence-marked (`+SURE / ~SUSPECT / -GUESS`). The durable map of how
Dorc's elide-past-a-running-command mechanism sits in the research literature: the canonical name for each
golden-hill concept, a graded reading-list, and the directional conclusions about what the literature does
and does not offer. It re-connects prior art already gathered in rounds 9 + 17 (`plans/090`, `plans/17N`,
`sources.json`); the spine citations (frame rule, dynamic frames, Bernstein) were verified against source
this session. **Read it** when you need the standard term for a mechanism, or to decide whether a literature
search is worth spending. **Honest weighting:** most of this pass is vocabulary for territory the design
already understood; the net-new is modest and mostly defensive — one mis-filed name recovered (§3), three
tempting search-directions closed with reasons (§4, §6, §10), and confirmation that the literature offers no
mechanism for the hard part. It found no new route, and argues none exists. Do not read it as a breakthrough.
**Reliability note:** the AGT/GES relevance verdicts are prose-grounded (their 2D
formalism did not survive text extraction); this doc names and maps — it does not vouch for any formal proof.*

---

## §1 — The thesis, in one paragraph

Dorc's golden-hill move — *footprint × backing × disjointness ⇒ elide-past-a-running-command* — is the
**separation-logic frame rule** (O'Hearn–Reynolds–Yang) applied through a **dynamic frame** (Kassios: a
footprint computed per-run); the disjointness test is **Bernstein's independence condition**; "poison" is the
compiler's **kill** set; the whole move is **(partial) redundancy elimination** over an abstract system-state
store. One line: **elision is a compiler-optimization problem; guarding is a gradual-typing problem** — the
must/may split, mapped onto two disciplines. The consequential conclusion (§4): the elide-half's *soundiness
cannot be borrowed.* Every discipline that elides under uncertainty (JITs, speculative execution, OCC,
adaptive query planners) stays sound via a **runtime net** — deopt, rollback, re-optimization — that Dorc's
plan/apply consent firewall structurally forbids. So Dorc's elide-soundiness must come from **plan-time proof**
or **disclosed residue**; no paper supplies a third option. The literature grounds vocabulary and confirms
walls; it does not hand Dorc a new mechanism. The net contribution is naming and wall-confirmation, not machinery.

---

## §2 — The terminology table (the spine)

*Dorc term (23M) → the canonical name → primary citation → already in our corpus? → what adopting the name
buys.* Confidence marks the mapping, not the citation.

| 23M term | Canonical name | Citation | In corpus? | What the name buys |
|---|---|---|---|---|
| **footprint** | footprint / mod-set / write-set / effect READ·WRITE | sep. logic (O'Hearn–Reynolds–Yang CSL'01); modifies-clause (JML/Dafny); effects [A-lucassen-gifford-effect-systems-popl-1988] | **YES** — the word is literally theirs (090 §D2, 099) | nothing new — already borrowed correctly |
| **backing** (a fact's verdict-probe read-set) | the **frame** `R` / an assertion's footprint / read-set / "self-framing" support | frame rule (below); implicit dynamic frames (Smans'09) | PARTIAL — 099 §8 has "the frame falls out of the effect-set," never named `R` | the ~SUSPECT "is backing a fresh completeness-universal?" worry is answered by **self-framing** — but only for *survival* of F, not for probe-adequacy (§5) |
| **disjointness ⇒ F's proof survives X** | the **FRAME RULE** `{P}C{Q} ⊢ {P∗R}C{Q∗R}`, side-cond. `mod(C) ∩ free(R)=∅`; = **stability** under interference (rely-guarantee) | Reynolds LICS'02; O'Hearn CACM'19; Jones TOPLAS'83 | gathered (090 §D2) then dropped (§3) | the exact licensing THEOREM the elide-work keeps re-deriving nameless |
| **footprint DERIVED at probe-time** (`dpkg -L`) | **dynamic frame** (a spec-var whose value is a computed set of locations); the infer pole = **bi-abduction** | Kassios FM'06; [A-biabduction-popl-2009] (Infer) | gathered (090 §D2/F2) then dropped | the precise name for 238-claim-5 "per-run derived footprint" + the industrial existence-proof (Infer) |
| **poison** (entity-granular) | **KILL** (gen/kill dataflow); invalidation | Kildall POPL'73; CS153 notes: "kills any available expr e2 s.t. x ∈ variables(e2)" | PARTIAL — gen/kill in the spike + 052/055; "poison = kill" not named | the safe-direction rule: **kill must be OVER-approximated (may-mod)** = "silence = wall" |
| **elide-past-a-running-command** (the whole move) | **(partial) redundancy elimination** / CSE, over an abstract state-store | Morel–Renvoise CACM'79; Knoop–Rüthing–Steffen "Lazy Code Motion" PLDI'92 | **NO** — never connected | the headline move is a 45-year-old optimization: the establish is elided because its precondition is still available and not killed |
| **the ternary {elide, guard, run}** | full-redundant / partially-redundant-⇒-insert-a-check / not-available | PRE + speculation-with-guard | **NO** | `guard = check‖cmd` is PRE's *insertion* step (make a partial redundancy total); elide = full redundancy; run = not available |
| **horizon** (professed liability boundary) | the typed/untyped **boundary** where **BLAME** is assigned | Wadler–Findler "Well-Typed Programs Can't Be Blamed" ESOP'09 | **NO** — blame never named | the formal home for "attribution of wrong elisions" |
| **the 3 failure-directions** (dangerous / value / disclosed) | soundness **direction** of an over-/under-approximation; **may (⊇) vs must (⊆)** | abstract interp. (Cousot); [A-engler-deviant-behavior-2001] MUST/MAY | PARTIAL — MUST/MAY in 096/099 | "name the quantifier, name its failure-direction" = state which way your abstraction is sound; under-approximating KILL is *the* unsound sin |
| **233-permanence** (can't ground soundness in a completeness-claim) | the **frame problem** ⇒ modular verification *requires* explicit frame specs; closed-world assumption | McCarthy–Hayes 1969 ([B-sep-frame-problem-2004]); Reiter CWA'78 | **YES** (093 f21) — filed as a *wall*, not as "why everyone declares frames" | the whole modifies-clause literature exists *because 233 is true*; you re-derived the field's founding lesson (§7) |
| **synonym** — within-kind, OBSERVABLE (fs, proc) | **aliasing** / may-must-alias / **points-to**; fix = must-not-alias via canonicalization | Andersen'94; Steensgaard POPL'96; Doop; CQual [A-foster-flow-sensitive-qualifiers-2002] | **YES** for aliasing (092/093/099 W2); points-to-as-synonym-dissolver not connected | the "measurement crack" = **dynamic points-to**: disjointness over *locations* not *names* (§6) |
| **synonym** — cross-PROVIDER sameness (dns.Zone, cloud, cross-manager pkg) | **entity resolution** / **record linkage** / **ontology alignment** / owl:sameAs | Fellegi–Sunter'69; [A-halpin-owl-sameas-2010]; [A-cisa-software-id-ecosystem-2023] | **YES** (17N/175) | **CLOSED by design** — reverse-DNS was chosen to bypass this; not open work (§6) |

---

## §3 — Why elision is compiler-theory and guarding is gradual-typing

The standing framing that organizes everything below: **PLT is the theory of guarding; compiler-engineering
is the theory of elision.** Gradual typing/effects, soundiness, and blame are all about *accepting* under
partial information and inserting a *runtime check* where you cannot prove safety — the guard half, which
Dorc mostly already had. The elide half is *computation-elimination* — dropping the mutating command whose
effect is already present — and its home is compiler-optimization theory. This is the **may/must split** by
discipline: `guard = MAY-accept (∃ a safe reading)`, `elide = MUST-eliminate (∀ readings safe)`.

Concretely, the golden-hill mechanism is **redundancy elimination through the frame rule**:
- The downstream fact F was established (the probe proved it) → F is *available* (GEN).
- The interfering command X *kills* F iff `footprint(X) ∩ backing(F) ≠ ∅`.
- F survives to its site (elides) iff F is available and not killed along the way.
- Soundness = **over-approximate the kill** (may-mod) and under-approximate availability (must). That
  asymmetry *is* "silence = wall."

The footprint side is **dynamic frames** (Kassios): a footprint is a function returning a set of coordinates
evaluated in the current state — exactly Dorc's probe-time-derived `manifest(){ dpkg -L "$1"; }`. The
infer-vs-declare tension the elide-work keeps re-litigating is the `kBURDEN` knob, poles named:
**bi-abduction / Infer** (derive) ↔ **modifies-clauses / dynamic frames** (declare).

**Why this was hard to see (the filing error).** None of this is new to the corpus. `plans/090` §D2 (round 9)
wrote the frame rule out verbatim — `{P}C{Q} ⊢ {P∗R}C{Q∗R}`, footprints, dynamic frames (Kassios),
bi-abduction (Infer), the infer-vs-declare knob. But §0.5's anti-over-connection guard tagged the whole
domain *"vocabulary-only, don't chase the separation-logic proving,"* and `plans/099` then folded separation
logic **into the impossibility walls** (Ramalingam "precise footprint undecidable," the frame problem) and
dropped the positive name. So the *frame rule* — the licensing condition, which is design-constraint
vocabulary, not a prover — was swept out with the proof-machinery, and every round since re-derives it in
private words (`238` "derived per-run footprint" = a dynamic frame; `23M` "disjointness ⇒ proof survives" =
the frame rule). The fix is a re-filing: **separation logic is positive prior art (frame rule + dynamic
frames), alongside — not inside — the impossibility walls.** The walls say "you can't infer the footprint
soundly"; the frame rule + dynamic frames + bi-abduction say "…so here is the declare-or-derive discipline
the field built in response," which is the discipline the elide-work independently reached.

---

## §4 — The two products, and why the elide-half's soundiness can't be borrowed

The sharpest structural fact is the **consent-wall**: the *attention product* (elide lines from the plan
BEFORE the human consents) needs a **static** proof; the *performance product* (skip work at apply) can use
**dynamic** re-observation. Prior art sorts cleanly on this line, and the two must not be re-tangled.

**Static-proof family → the attention product (the golden hill):** the frame rule; dynamic frames +
bi-abduction; Bernstein's disjointness *test* (borrow the test, not the reorder — Dorc forbids reordering);
PRE / available-expressions (the elide *action*); rely-guarantee **stability** ("is F stable under X's
interference?"); occurrence typing (guard-lifting) + the forgiving gradual/success/soft-typing posture.

**Dynamic / re-observation family → the performance product (task #11), FORBIDDEN for the attention
product:** OCC / snapshot-isolation validation; generation-probes / verifying-trace rebuilders (Mokhov,
ninja `restat`, early-cutoff); TOCTOU re-check; build-hazard speculate-then-fallback (Rattle); and the whole
JIT / speculative-optimization + deoptimization lineage.

**The load-bearing conclusion — the elide-half's soundiness is not borrowable.** The tempting move is "mine
JITs: they elide under an open, changing, imperfectly-known world." They do — but their soundiness mechanism
is **deoptimization**, and deopt is architecturally forbidden here. A deopt is a *dynamic, post-consent
runtime decision*: discover a violated assumption at runtime, then run a fallback that was never in the plan.
The plan/apply consent-ack means *what the human approved is what runs*; a runtime-added fallback breaches
the firewall. Dorc's guard (`check‖cmd`) is **not** a deopt — the whole conditional is *present in the
consented plan*, so at apply the check merely selects a pre-approved branch. **Present-and-consented (guard)
vs absent-then-added (deopt)** is the entire line.

So every elide-under-uncertainty discipline — JIT deopt, speculative rollback, adaptive re-optimization,
OCC-abort, self-adjusting recomputation — stays sound via a runtime net **Dorc refuses by construction** to
preserve the attention product. Dorc's elide-soundiness therefore has only two sources: **plan-time
certainty** (a static proof — the frame-rule/dynamic-frames vein) or **disclosed residue** (the horizon;
soundiness). The moving-target literature **confirms this wall; it does not scale it.** From those
disciplines Dorc can borrow the *static* half (which speculations are worth attempting — cost-model input,
already touched in `074`) but never the dynamic soundiness net.

---

## §5 — Survival vs adequacy: the two layers of an elision

An elision rests on two independent claims; conflating them is a live hazard.

- **Survival** — "does F, once observed, outlast the interfering command?" This is backing/disjointness, and
  it is *settled*: define `backing(F)` = the verdict-probe's read-set, and F is **self-framing** (implicit
  dynamic frames, Smans–Jacobs–Piessens ECOOP'09) — it cannot claim more than the probe reads, *by
  construction*, so the completeness question dissolves. `-SUSPECT` retired.
- **Adequacy** — "does the probe faithfully stand in for the ELIDED COMMAND's full effect?" Eliding command M
  on probe P substitutes P's verdict for M's behaviour, and self-framing does **not** license that: it makes
  F complete about P's *own reads*, never equal to M's *effect*. This is the **oracle contract** —
  calibrated, never proven — and it under-executes *silently* in the **converged ≠ no-op** case (`dpkg -s
  nginx` holds, yet `apt-get install nginx` would upgrade; = 236b hunt-A / door-2 converged-vouch).

**No literature touches the adequacy layer** — it is Dorc's own design risk, and it is the *more* suspect of
the two. The survival layer is where the prior art applies; the adequacy layer is where the design work is.

---

## §6 — The synonym cell, scoped (an adoption item, not open research)

23M flags synonym/entity-identity as "the one genuinely-unresearched cell, dig hardest here." That over-states
the openness *and* mis-files the danger. It is three problems across two literatures:

1. **Cross-provider sameness** (`apt.Package:nginx` ≡ another manager's nginx) — the owl:sameAs / ontology-
   alignment / record-linkage quagmire — is **closed by design**. Reverse-DNS per-provider naming was chosen
   (`17N`/`175`) *specifically to bypass* it: identity rides a declared named kind, co-reference is a may-grade
   hint only, cross-provider sameness is simply not offered. The linked-data field's own verdict ("binary
   identity is too strong; grouping is unsolved") is *why* reverse-DNS was chosen. Spent; do not re-open.
2. **Within-kind, observable aliasing** (symlinks / mounts / normalization — the *actually-dangerous* cell:
   one referent under two real keys; `/etc/nginx` and a bind-mount of it are one inode) — is **alias analysis,
   not owl:sameAs**. 23M mis-files it against the abstract literature. The fix is 23M's own "measurement
   crack," whose name is **dynamic points-to**: test disjointness over *resolved locations*, not names (two
   names are must-not-alias iff their points-to sets are disjoint). In-corpus already (Andersen/Steensgaard,
   Doop, CQual strong/weak-update = 092/099 W2; 236b-F7 reached for `realpath`-canonicalization). Sound rule:
   **disjointness is safe only as MUST-not-alias; absent that proof, assume may-alias ⇒ wall.** This is the
   one genuine adoption item — a design thread, not a literature gap.
3. **Within-kind, abstract referents** (dns.Zone, cloud entities — no inode to resolve to) — reduces to a
   discipline: the kind-owner **declares** its entity-identity semantics; where it can't, entities stay
   distinct (**wall on un-pinned aliasing**). Safe, over-verifying, the honest floor.

Net: once split, the cell is ~fully answered — the "open frontier" was an artifact of conflating the two
literatures.

---

## §7 — Why 233 is permanent (the field agrees), and why this was never built

`+SURE.` "233 — grounding soundness in a fallible completeness-claim is unsound — is permanent; cage it, don't
fix it" is *the frame problem* (McCarthy–Hayes 1969; 093 f21) plus its settled engineering consequence. The
entire modular-verification field reached this decades ago: JML, Dafny, separation logic, dynamic frames, and
region logic all make you *write* a modifies-clause/footprint **because** you cannot infer the frame soundly
(Ramalingam) or enumerate the non-effects (frame problem) — so the only honest move is an explicit,
declared-or-derived frame with a conservative default (undeclared ⇒ clobbered), which *is* "silence = wall."
233's CONCENTRATE/OPT-IN/PRICE re-derives the modifies-clause discipline; convergence with 40 years of
practice is the strongest signal the spine is right, not a Dorc tragedy. Adopt **blame** (Wadler–Findler) as
the term for the horizon's who-is-at-fault attribution.

**Why nobody built this (road-not-taken, not road-that-failed).** Redundancy elimination ships everywhere
*except* this cell because the ops world took the **convergence** fork — CFEngine et al. "avoid unnecessary
operations" via *runtime, per-resource* idempotence (= the guard tier), correctness without a short
pre-consent plan — and the shell-analysis world took the **lint** (ShellCheck) / **verify** (CoLiS) fork,
never elision. Static-redundancy-elimination + effect-oracles + probes for ops sh is unbuilt because the
payoff (a short pre-consent plan) wasn't what convergence optimised for — not because it was tried and failed.

---

## §8 — Reading list (graded; grouped by what it serves)

`[core]` = read for the golden hill · `[frame]` = specification vocabulary · `[perf]` = performance product
only (do **not** open for the attention/elide half) · `[context]` = already-mapped · `[closed]`/`[adopt]` =
the synonym cell. `[in-corpus]` = already in `sources.json`/vendor; don't re-acquire. Venue confidence marked.

**Tier 1 — the missing spine of the elide mechanism (highest value):**
- `[core][frame]` O'Hearn–Reynolds–Yang, *Local Reasoning…* (CSL 2001); Reynolds, *Separation Logic* (LICS
  2002); O'Hearn, *Separation Logic* (CACM 62(2), 2019 — the readable retrospective). `+SURE(verified)`. The
  frame rule = the licensing theorem. Start with CACM'19.
- `[core][frame]` Kassios, *Dynamic Frames* (FM 2006, LNCS 4085). `+SURE(verified)`. = the probe-time-derived
  footprint.
- `[core]` Calcagno–Distefano–O'Hearn–Yang, *Compositional Shape Analysis by Bi-Abduction* (POPL 2009)
  [A-biabduction-popl-2009]. The "derive the footprint" pole, shipped at scale (Infer) — architecture, not
  algorithm, transfers (tool-metadata vs program-shape).
- `[core]` Bernstein, *Analysis of Programs for Parallel Processing* (IEEE TEC EC-15, 1966). `+SURE(verified)`.
  The disjointness/independence *test*.
- `[core]` Kildall, *A Unified Approach…* (POPL 1973); Morel–Renvoise, *…Partial Redundancies* (CACM 1979);
  Knoop–Rüthing–Steffen, *Lazy Code Motion* (PLDI 1992). `+SURE(content), -GUESS(venues)`. Available-
  expressions is [in-corpus] via `learning-path/harvard-cs153-cfg-dataflow.txt`. elide = redundancy
  elimination; poison = kill.

**Tier 2 — how a footprint is specified (declaration vocabulary):**
- `[frame]` Smans–Jacobs–Piessens, *Implicit Dynamic Frames* (ECOOP 2009). `~SUSPECT(venue)`. Gives
  **self-framing** — settles the *survival* half of the backing worry (not adequacy; §5).
- `[frame]` Banerjee–Naumann–Rosenberg, *Regional Logic* (2008/2013). Sibling of dynamic frames; regions =
  footprint-as-set.
- `[frame]` Jones, *…Interfering Programs* (TOPLAS 1983); Vafeiadis–Parkinson, *RG + Separation Logic* (CONCUR
  2007); O'Hearn, *Resources, Concurrency, and Local Reasoning* (TCS 2007). `~SUSPECT(venues)`. **Stability
  under interference** = "proof survives the running command."
- `[frame]` Dafny (Leino; any modern tutorial). Dynamic frames in a working tool — how declared footprints
  actually check. `-GUESS(cite)`.

**Tier 3 — attribution + the forgiving posture (mostly already mapped):**
- `[core]` Wadler–Findler, *Well-Typed Programs Can't Be Blamed* (ESOP 2009). `+SURE`. Blame =
  horizon-attribution.
- `[context]` The gradual-typing lineage — Lindahl–Sagonas success typings (PPDP'06); Siek–Taha gradual +
  Siek et al. gradual guarantee (2006/2015); Cartwright–Fagan soft typing (PLDI'91); Tobin-Hochstadt–
  Felleisen occurrence typing (ICFP'10) [A-tobin-hochstadt-logical-types-2010, in-corpus]; Bracha pluggable
  types. Already in `learning-path/gradual-success-typing.ai-pointers.md`; validate, don't re-gather.
  - **Abstracting Gradual Typing** — Garcia–Clark–Tanter (POPL 2016) [A-garcia-abstracting-gradual-typing-popl-2016].
    **MARGINAL** — its "consistency" is the optimistic ∃/may lifting (= the accept/guard side Dorc has); it
    names ⊤/no-cliff but builds no ∀/must mechanism; wrong polarity for the elide half. Full grade: `sources.json`.
  - **A Theory of Gradual Effect Systems** — Bañados Schwerter–Garcia–Tanter (ICFP 2014) [A-banados-gradual-effect-systems-icfp-2014].
    **LOW-MEDIUM** — the more relevant of the two: it *does* build the ∀/must dual (`strict-check`) and uses
    strict-vs-consistent to drive check-elision, and its **minimal-guard** idea (verify only the minimal
    residual) is an adoptable guard-tier refinement. But it elides the *check*, not the mutating *command*,
    and assumes effects are *declared* — so it vindicates structure without touching the info-layer/233. Full
    grade: `sources.json`.
- `[context]` Lucassen–Gifford effect systems (POPL'88) [in-corpus]; Gordon effect quantale (order-sensitive)
  [cand. A-gordon-effect-quantale-2021]. `~SUSPECT`.

**Tier 4 — the synonym cell (§6):**
- `[adopt]` Andersen (DIKU 1994) & Steensgaard, *Points-to Analysis in Almost Linear Time* (POPL 1996); Doop
  [in-corpus]; CQual [A-foster-flow-sensitive-qualifiers-2002, in-corpus]. The one real adoption item: within-kind observable
  aliasing = must-not-alias over disjoint points-to; the "measurement crack" = dynamic points-to.
- `[closed]` Halpin et al., *When owl:sameAs isn't the Same* (ISWC 2010) [A-halpin-owl-sameas-2010,
  in-corpus]; CISA software-id [A-cisa-software-id-ecosystem-2023, in-corpus]; Fellegi–Sunter record linkage
  (JASA 1969). The *rationale* for reverse-DNS, not open work. Do not re-open.

**Tier 5 — performance product only (NOT the elide half; listed so they're not mis-opened):**
- OCC (Kung–Robinson 1981); Mokhov et al. *Build Systems à la Carte* (075/076, in-corpus); Spall–Mitchell
  *Rattle* [A-spall-mitchell-rattle-perfect-dependencies-2020, in-corpus]; Papadimitriou serializability (JACM 1979).
- The JIT/deopt lineage — Hölzle–Chambers–Ungar dynamic deoptimization (PLDI 1992); CHA + guarded
  devirtualization (Dean–Grove–Chambers, ECOOP 1995); adaptive query re-optimization; self-adjusting
  computation. **Read to understand the wall (§4), not to scale it** — their soundiness mechanism (the runtime
  net) is forbidden here.

**Deferred / tangential (not the golden hill):**
- Cross-host "distributed PRE" (fleet arc, `22H`) — if it un-parks: choreographic programming / endpoint
  projection (Montesi; multiparty session types); the **CALM theorem** (Hellerstein–Alvaro, CACM 2020:
  coordination-free ⟺ monotone); predicate/operator pushdown. Session *types* are protocol-conformance,
  adjacent-not-it.
- Frame-problem AI-side (enrichment only): Shanahan 1997 + Lifschitz 2015 (event-calculus / circumscription)
  — a second solution-shape for "what persists across an action," sibling to the verification-side answer.

---

## §9 — Honest caveats (the confidence boundary)

- The frame rule **names** the licensing condition; it does **not** make it decidable. Sep. logic assumes the
  footprint is given; Dorc must derive/declare it and can't infer it soundly (Ramalingam/233). The frame rule
  structures the check, it doesn't dissolve 233 — which is the point (§7).
- Bernstein licenses **reordering**; Dorc forbids reorder (STALENESS-AUDIT). Borrow the disjointness *test*,
  not the parallelize/reorder *action*.
- Classical PRE has **syntactic** kill (a variable redefinition); Dorc's kill is a *semantic footprint
  intersection over an abstract system-state store* — "PRE generalised to effectful operations," which is real
  mod/ref work, not free.
- Self-framing retires the *universal-quantifier* worry, not the *measurement-completeness* worry (did I trace
  the probe's reads) and — the bigger one — not the *probe-adequacy* worry (§5). Both stay as residual risks.
- Blame calculus assumes a boundary with runtime contract enforcement; Dorc's horizon is a *professed* (prose)
  boundary with none. Borrow the attribution vocabulary; the enforcement half doesn't transfer (elision
  bypasses guards).
- The AGT/GES verdicts are **prose-grounded** — their 2D formalism did not survive text extraction, so the
  claims rest on the authors' English, not on parsed proofs. Not vouched as formal fact.

---

## §10 — Direction: what to do with this (design, not more reading)

The literature's *generative* routes are exhausted for the elide half, and we can say why: the hard part —
sound plan-time elision over a moving, imperfectly-specified target — is provably not inferable (the frame
problem), and the runtime net every other discipline uses to stay sound is architecturally refused (§4). So
the forward work is **design and adoption, not literature**:

1. **Adopt the vocabulary into the human-authored docs.** The single highest-value action: put "frame rule /
   dynamic frame / stability / kill / blame / self-framing" where the design lives, so the team stops
   re-deriving them every round (the filing error, §3). This doc is AI-authored notes; the vocabulary only
   stops the churn once it lands somewhere authoritative.
2. **Adopt the alias-analysis frame** for within-kind observable synonyms (§6) — the one genuine
   implementation thread the prior art hands over (must-not-alias / dynamic points-to / `realpath`-canon).
3. **Point design attention at the adequacy layer** (§5) — the oracle contract / converged≠no-op — which is
   the live, more-suspect risk and which *no* paper addresses.
4. **The elide-soundiness wall is confirmed (§4): do not re-open the JIT / moving-target search.** It confirms
   the wall, it does not scale it. If revisited, the useful question is narrow — "sound elision under a moving
   target with a *static-only* net" — and the honest expected answer is that it doesn't exist, because the net
   everyone relies on is the one Dorc gives up to keep the attention product.

Bottom line (weighted honestly — a modest, mostly-defensive haul): the prior art grounded the mechanism (elision = frame-rule-through-a-dynamic-frame), confirmed
the wall (the naked residue is the whole field's residue, not Dorc's mistake), and closed three tempting
search directions with reasons. It did not, and by construction cannot, hand over a mechanism that makes the
elide-half sound without either plan-time proof or disclosed residue. The value of this pass is real but defensive — fewer wrong turns, better names — not a new road forward.
