# 225 — research: the four unreachable primaries behind the receipts plane

> Retrieval-and-read note, 2026-06-11, PHASE-R of round-22 (error/provenance round).
> Serves note 220 §7, which leaned on four primaries it marked NOT directly read
> (220-corpus `B-zdancewic-myers-2001`-via-survey, `B-green-tannen-2007`,
> `B-livshits-chong-2013`, `B-carata-primer-2014`). Each underwrites a load-bearing
> claim of a refuse/explain-ONLY provenance ("receipts") plane: a per-value
> `value × set-of-contributing-origins` that may only refuse-or-explain, never grant
> permission (the only licenses to elide a command come from author-declared oracle
> claims, never from receipt inspection). This note reads all four in full, grades
> them, and extracts the load-bearing material as VERBATIM excerpts with page
> locators, each followed by its read for the receipts plane.
>
> Findings slugged `finding-N` / descriptive slugs; sources `[grade-slug-year]`,
> graded list at the foot. Confidence marks per project convention
> (+SURE / ~SUSPECT / -GUESS / --WONDER). Page locators are the PDF's own printed
> page numbers unless noted "sheet" (extractor sheet index).
>
> Provenance of this note: ZM01 and Green–Tannen were read+graded full by a
> predecessor agent (its turn-1 scratch banked verbatim below, re-verified against
> the cited pages where thin); Livshits–Chong and Carata were read full from local
> PDF by this agent. Grades assigned by gathering subagent; conductor re-verification
> pending.
>
> Cross-reference convention: bracketed `[slug]` = a source with a Graded-sources row
> in THIS note. Back-references to note 220 use a non-bracket form — `220:vp-N` for
> its findings.

## §0 Conclusions up front

- **finding-1 (the one-way rule's true source is the survey, not ZM01) — +SURE.**
  220:vp-26 attributes "untrusted influence must not control what gets released" to
  Zdancewic–Myers. That phrasing is the Sabelfeld–Sands *survey's* downstream gloss,
  NOT a verbatim ZM01 claim. ZM01's own statement of robust declassification is
  operational/information-theoretic (Def 4.2): an *active* attacker who modifies the
  system learns no more than a *passive* observer. The Dorc design should cite the
  survey for the slogan and ZM01 for the semantic property. ~SUSPECT the survey's
  gloss is faithful; the chain is sound, just mis-attributed.

- **finding-2 (ZM01 gives the property, not the enforcement) — +SURE.** ZM01 studies
  a *richer* object than Dorc needs — information leaked under declassification, a
  confidentiality × integrity duality proven via a least-fixed-point over an
  information lattice. The mechanism Dorc actually wants — a one-way *type/dataflow*
  rule — is downstream work (the later "enforcing robust declassification" type-system
  papers), not this one. Cite ZM01 for *why the rule is the right shape*, not for *how
  to enforce it*.

- **finding-3 (ZM01 carries a self-correction the secondhand read missed) — +SURE.**
  Footnote 4 (p7): the proceedings Theorem 4.2 was "incorrect as stated"; this copy is
  the corrected, weaker version, valid only when `S^ω[≈A] = S[≈A]`, and even then the
  bound "is not tight." Any Dorc claim that the formalism gives a tight info-flow bound
  would over-reach.

- **finding-4 (ZM01's attacker model and blame construction both transfer) — +SURE on
  the quotes, ~SUSPECT on the mapping.** The "fair environment assumption" (Def 4.1:
  the attacker must not already know the secret nor be able to learn it except via the
  system) maps to "receipts must not mint authority they did not already hold." ZM01
  §2.3's set `D` of domains that *could* explain an observed declassification, with
  `glb(D)` pinpointing the least one responsible under a distributive lattice, is a
  blame/provenance construction directly analogous to Dorc's ⊤-blame ("who
  contributed", 220:vp-16).

- **finding-5 (semirings: coarsening is principled, not lossy-by-accident) — +SURE.**
  Green–Tannen prove the *factorization theorem*: RA+ (and datalog) on K-relations for
  any commutative ω-continuous semiring factors uniquely through the provenance
  polynomial `N[X]` (resp. `N^∞[[X]]`) via the evaluation homomorphism. The paper
  itself frames lineage/bag/set as deliberate coarsenings of the universal polynomial,
  and Prop 3.5 makes the homomorphism the exact *license to coarsen safely*: a coarse
  answer is the homomorphic image of the fine one iff the coarsening map is a semiring
  homomorphism. This is the formal grounding for 220:vp-19 (use flat lineage on
  values, the full how-polynomial answers no Dorc question) — the coarsening is sound,
  not accidental loss. NB: the *named* `lineage ⊂ why ⊂ how` hierarchy is the
  Cheney/Karvounarakis–Tan survey's packaging (220-corpus `B-cheney-whw-2009`); THIS
  paper proves the underlying math.

- **finding-6 (why-provenance is too coarse exactly at licenses) — +SURE.** Green–
  Tannen's why-vs-how passage (sheet 4 §4): two output tuples can share why-provenance `{r,s}`
  yet be computed `from s alone` and `from r alone` respectively; "in a provenance
  application in which one of r or s is perhaps less trusted or less usable than the
  other the effect can be different … and this cannot be detected by why-provenance."
  This is the verbatim grounding for 220:vp-17 — store the *minimal witness the license
  was actually granted on*, at licenses only, not flat origin sets. For 220:vp-16
  ("who contributed to ⊤") flat lineage remains the paper-sanctioned coarsening.

- **finding-7 (distributive-lattice collapse corroborates avoiding how-polynomials) —
  ~SUSPECT.** Thm 9.2: if K is a distributive lattice, query containment under
  K-relation semantics is identical to plain set semantics. Dorc's abstract domains are
  lattices; for lattice-shaped provenance the polynomial machinery buys nothing on the
  containment question — a hard-theorem corroboration of 220:vp-19, not merely an
  absence-of-consumer argument. ~SUSPECT because the mapping from "query containment"
  to Dorc's actual queries is an analogy, not a proof.

- **finding-8 (Livshits–Chong: humans misplace permit-points, demonstrated) — +SURE.**
  The paper's thesis is that sanitizer/declassifier placement "in large-scale
  applications is difficult, and developers are likely to make errors, and thus create
  security vulnerabilities" — so much so that "developers are better off leaving out
  sanitizers entirely" and letting placement be inferred. Developers "are heavily
  discouraged from writing their own sanitizers … because most of the time, they get
  them wrong." This is the empirical/argumentative support for 220:vp-27's design
  translation: make permit-points few, explicit, owned, and machine-checked — in Dorc,
  the License type is constructible only from an oracle-claim (capability-style), never
  from receipt inspection.

- **finding-9 (Livshits–Chong: where licenses sit is computed, never the consumer's
  burden, and the analysis is orthogonal to graph precision) — +SURE on quotes.** The
  placement is the deliverable; the precision/soundness of the underlying dataflow
  graph is *orthogonal* — better graphs only improve results. Their edge-based
  placement reduces instrumentation points by 6.19× on average (up to 27× on sparse
  synthetic graphs) vs naive taint-everything (which would instrument 60+% of nodes).
  -GUESS the transferable shape for Dorc: licenses are few and computed at specific
  edges; a precise analyzer makes them fewer, but correctness does not depend on
  precision — exactly Dorc's best-effort posture.

- **finding-10 (Carata: capture-without-consumer is the recurring drowning, and
  granularity trades against noise) — +SURE.** The previously-missing tail confirms
  220:vp-23. Overhead is NOT the killer — every shipping system lands in a tolerable
  band (workflow/disclosed ~1%; PASSv2 1–23%; SPADEv2 <10%; spatial ~20%). The killers
  are noise and unbounded capture: finer granularity raises overhead AND noise (the
  Python-interpreter example links an output to every module loaded); "heuristics are
  needed to determine which entities are important and which should be ignored"; the
  n-by-m black-box problem manufactures false-positive dependencies; and pruning/
  querying/visualization are still listed as open research. Inoculation for Dorc: build
  receipts against the closed consumer list, refuse speculative capture, and treat
  pruning/suppression as first-class.

- **finding-11 (Carata: secure provenance wants different access policy than the data,
  and disclosed-vs-observed is the trust axis) — ~SUSPECT.** Provenance must be
  "managed under different access policies than those of the data," with security
  defined as confidentiality + integrity. Disclosed systems (author attests via API)
  give better semantics but their trustworthiness "is a concern when running in
  untrusted environments"; observed systems lose semantics by black-boxing each
  process. ~SUSPECT mapping: Dorc's oracle-claims are the "disclosed" pole (author
  attests, better semantics, trust-on-the-author); inferred receipts are the "observed"
  pole (cheap, semantics-poor, never authoritative) — which is exactly why receipts
  may only refuse/explain and licenses come only from claims.

- **fetch-requests (for the human): NONE strictly blocking.** All four primaries were
  read in full from local copies. One *optional* follow-up is listed in the
  fetch-requests section below (the ZM01 type-system sequel that supplies the
  enforcement finding-2 says ZM01 itself lacks); not required for this round.

---

## Zdancewic–Myers, "Robust Declassification" (CSFW 2001) — FULL READ (predecessor), author-posted Cornell copy

Read+graded full by the predecessor agent; banked here verbatim from its turn-1
scratch, spot-re-verified against the cited pages (the Def 4.1 / Def 4.2 / fn4 locators
hold). Source: `[A-zdancewic-myers-robust-declassification-2001]`.

What this paper is, for the receipts plane: it is the *named origin* of robust
declassification, but it studies a richer object than Dorc needs — *information leaked*
under declassification (a confidentiality × integrity duality), proven via a
least-fixed-point `S^ω` over an information lattice. It supplies the **semantic
property** (the rule's right shape) and an **attacker model** and a **blame
construction** that transfer; it does NOT supply an enforcement mechanism (that is the
downstream type-system work). See finding-1..4.

Load-bearing excerpts:

> [A-zdancewic-myers-robust-declassification-2001]:p2 §1 — the informal statement of
> the property (relevance +SURE):
> "[the system] is robust with respect to a class of active attackers if these
> attackers can learn no more about the confidential information through active attacks
> than they can through passive observation. Equivalently, a system is robust if the
> intentional information leaks that it contains cannot be exploited through active
> attack to learn more than was intended."

> [A-zdancewic-myers-robust-declassification-2001]:p6 §4.2 Def 4.2 — the precise
> definition (relevance +SURE):
> "Definition 4.2 (Robust Declassification) A system S = ⟨Σ, ↦⟩ is robust with
> respect to the class B ⊆ A(≈A) of attacks if for all attacks A = ⟨Σ, ↦A⟩ in B, it is
> the case that (S ∪ A)[≈A] ⊑I S[≈A]. … This says formally that observing the attacked
> system S ∪ A reveals no more information than watching the original system S."

> [A-zdancewic-myers-robust-declassification-2001]:p6 §4.1 Def 4.1 — the attacker model
> / fair-environment assumption (relevance +SURE):
> "Definition 4.1 (≈A-Attack) An ≈A-attack is a system A = ⟨Σ, ↦A⟩ such that
> A |= SP(≈A). … the requirement that A |= SP(≈A) is essentially the fair environment
> assumption: The attacker must not know the secret already (or be able to learn it
> from means other than the system in question)."

> [A-zdancewic-myers-robust-declassification-2001]:p7 §4.2 Thm 4.1 — the
> conservativity-adjacent result (relevance ~SUSPECT):
> "If S |= SP(≈A) then S |= R(A(≈A))."
> [i.e. a noninterference-secure system is automatically robust to all attacks from
> that view.]

> [A-zdancewic-myers-robust-declassification-2001]:p7 fn4 — the self-correction
> (relevance ~SUSPECT, but load-bearing as a caveat):
> "In the proceedings version of this paper, Theorem 4.2 was claimed to be a
> generalization of Theorem 4.1, and was incorrect as stated. The version presented
> here is weaker in that it does not define a class of attacks against which S is robust
> unless S^ω[≈A] = S[≈A]."

> [A-zdancewic-myers-robust-declassification-2001]:p4–5 §2.3 — the blame/responsibility
> construction (relevance ~SUSPECT):
> "[blame construction] D = {≈ℓ | S[≈] ⊑I (≈ℓ ⊔I ≈)} … If the lattice LC is
> distributive, we can pinpoint the least security domain that could have been
> responsible for the declassification by simply taking the greatest lower bound on the
> members of D."

Read for the receipts plane:
- The slogan in 220:vp-26 ("untrusted influence must not control what gets released")
  is the *survey's* synthesis, not ZM01 verbatim — cite accordingly (finding-1).
- The fair-environment assumption (Def 4.1) is the sharp version of "receipts must not
  mint authority": an inferred receipt is the attacker's influence; it must not be able
  to construct a dependency on information it could not observe — i.e. it must not grant
  a license it did not already hold (finding-4). This is sharper than note 220's gloss.
- §2.3's `glb(D)` blame is a prior-art anchor for ⊤-blame (220:vp-16): the *least*
  domain that could explain an observation, recoverable only when the lattice is
  distributive — a caveat worth carrying, since Dorc's domains are lattices but not
  always distributive.
- Use the corrected, weaker Thm 4.2 and do not claim a tight info-flow bound
  (finding-3).

---

## Green / Karvounarakis / Tannen, "Provenance Semirings" (PODS 2007) — FULL READ (predecessor), UC Davis author copy

Read+graded full by the predecessor agent; banked here verbatim from its turn-1
scratch. Source: `[A-green-karvounarakis-tannen-provenance-semirings-2007]`. (Housekeeping
for the conductor: the predecessor first registered this paper at a `B-` slug by
erroneous carry-over of note-220's access-grade, then re-registered it correctly at the
`A-` slug after the full read; the stale `B-green-karvounarakis-tannen-provenance-semirings-2007`
key + its archived PDF remain in the r22-rqA `sources.json` as an append-only artifact,
cited nowhere — prune that one duplicate key + file.)

What this paper is, for the receipts plane: the seminal source of the semiring
provenance framework. It proves the mathematical fact the named `lineage ⊂ why ⊂ how`
hierarchy rests on — the **factorization theorem** — and frames coarser provenance forms
as principled, homomorphic coarsenings of a universal polynomial. It directly grounds
the "coarsen safely" argument for Dorc's flat-lineage-on-values choice (220:vp-16/19) and
the "store the minimal witness at licenses" choice (220:vp-17). See finding-5..7.

Load-bearing excerpts:

Locator note: the local copy is the author preprint — it carries the PODS'07 copyright
block but NOT ACM printed folios (pp.31–40 in the proceedings). Locators below are
extractor *sheet* indices (10 sheets total), verified by content search; the
predecessor's "pN" were the same sheet indices. Conductor: if a folio-accurate cite is
wanted, map sheet→folio as sheet N → p(30+N).

> [A-green-karvounarakis-tannen-provenance-semirings-2007]:sheet 9 §Conclusions — the
> paper's own framing of coarser-forms-as-coarsenings (relevance +SURE):
> "Beyond the technical results, this paper can be regarded also as arguing that various
> forms of K-relations, even multisets, provide coarser forms of provenance while the
> polynomial and formal power series annotations are, by virtue of their 'universality'
> (as illustrated by the factorization theorems) the most general form of annotation
> possible with[in] the boundaries of semiring structures."

> [A-green-karvounarakis-tannen-provenance-semirings-2007]:sheet 4 §4 — the why-vs-how
> limitation, verbatim motivation for storing the minimal witness (relevance +SURE;
> exact text re-verified on sheet 4):
> "in Figure 5(b) (f, e) and (d, e) have the same why-provenance … However, the query
> can also calculate (f, e) from s alone and (d, e) from r alone. In a provenance
> application in which one of r or s is perhaps less trusted or less usable than the
> other the effect can be different on (f, e) than on (d, e) and this cannot be detected
> by why-provenance … we need to know not just which input tuples contribute but also
> how they contribute."

> [A-green-karvounarakis-tannen-provenance-semirings-2007]:sheet 4 §3 Prop 3.5 — the
> algebraic license to coarsen (relevance +SURE):
> "Let h : K → K' … The transformation given by h from K-relations to K'-relations
> commutes with any RA+ query … q(h(R)) = h(q(R)) if and only if h is a semiring
> homomorphism."
> [a coarse view is the homomorphic image of the fine one exactly when the coarsening
> map is a semiring homomorphism — this IS the safety condition for coarsening.]

> [A-green-karvounarakis-tannen-provenance-semirings-2007]:sheet 4 §4 Thm 4.3 — the
> factorization theorem (relevance ~SUSPECT):
> "For any RA+ query q we have q(R) = Eval_v ∘ q(R̄)"
> [any-semiring semantics factors through the universal provenance polynomial N[X];
> stated sheet 4, the N^∞[[X]] datalog/power-series version on sheets 7–8.]

> [A-green-karvounarakis-tannen-provenance-semirings-2007]:sheets 8–9 §9 Thm 9.2 — the
> distributive-lattice containment collapse (relevance ~SUSPECT):
> "If K is a distributive lattice then for any q1, q2 unions of conjunctive queries we
> have q1 ⊑K q2 iff q1 ⊑B q2."
> [for lattice-shaped annotation the finer structure adds nothing to containment —
> corroborates avoiding how-polynomials in Dorc's lattice setting.]

> [A-green-karvounarakis-tannen-provenance-semirings-2007]:sheet 4 fn — where the paper
> coins the term (relevance ~SUSPECT):
> "In contrast to why-provenance, the notion of provenance we propose could justifiably
> be called how-provenance."

Read for the receipts plane:
- The Conclusion passage + Prop 3.5 together say: pick the cheapest representation that
  is a homomorphic image of the universal one, and the coarsening is *provably*
  information-preserving up to that image. Flat lineage on values (220:vp-16) is a
  sanctioned coarsening, not accidental loss (finding-5).
- The why-vs-how passage is the precise reason flat lineage is too coarse *at licenses*:
  it cannot tell "licensed because of r alone" from "licensed because of s alone." Store
  the minimal witness the license was granted on, at licenses only (220:vp-17,
  finding-6).
- Thm 9.2 is a hard-theorem corroboration that, in a lattice setting, the polynomial
  machinery buys nothing on containment — reinforcing "don't reach for how-polynomials"
  (220:vp-19, finding-7) beyond the mere absence-of-consumer argument.
- Recursion handling (formal power series `N^∞[[X]]`, finite iff no cycle of unit rules
  through the tuple) is the formalism behind Soufflé's "min-proof-height picks one of
  infinitely many proofs" trick — -GUESS the height-annotation is an engineering shortcut
  around the power-series machinery (relevant to 220:vp-18's recompute-on-retraction).

---

## Livshits & Chong, "Towards Fully Automatic Placement of Security Sanitizers and Declassifiers" (POPL 2013) — FULL READ, Chong/Harvard author copy

Read full from local PDF by this agent. Source: `[A-livshits-chong-automatic-placement-2013]`.
Printed folios coincide with extractor sheets (pp.1–14); locators are those page
numbers. Footnote 1 (p1) makes the sanitizer↔declassifier identification explicit, so
everything below applies to declassifier placement equally.

What this paper is, for the receipts plane: the load-bearing evidence for 220:vp-27
("permits are capabilities, never inferences; make permit-points few/explicit/owned").
It argues — from large-application experience — that humans systematically *misplace*
permit-points (sanitizers/declassifiers), to the degree that the authors advocate
removing the human from placement entirely and inferring it. It also shows placement is
a *computed* property whose correctness is independent of analyzer precision, and that
the count of permit-points is small relative to a naive instrument-everything baseline.
See finding-8..9.

Load-bearing excerpts:

> [A-livshits-chong-automatic-placement-2013]:p1 Abstract — the thesis (relevance
> +SURE):
> "However, in pretty much all work thus far, the burden of sanitizer placement has
> fallen on the developer. However, sanitizer placement in large-scale applications is
> difficult, and developers are likely to make errors, and thus create security
> vulnerabilities. … This paper advocates a radically different approach: we aim to
> fully automate the placement of sanitizers by analyzing the flow of tainted data in
> the program. We argue that developers are better off leaving out sanitizers entirely
> instead of trying to place them."

> [A-livshits-chong-automatic-placement-2013]:p2 §1.1 — why developers should not own
> the placement (relevance +SURE):
> "Developers are heavily discouraged from writing their own sanitizers. This is in part
> because most of the time, they get them wrong [4, 15]."

> [A-livshits-chong-automatic-placement-2013]:p1 §1 — the scale evidence (relevance
> +SURE):
> "Additional motivation for exploring run-time techniques comes from the complexity of
> large-scale web applications with multiple, potentially nested sanitizers, which
> recent assessments [38, 39] suggest is well beyond the ability of developers to
> address using static reasoning and code reviews."
> [the cited assessments are Samuel et al. CCS'11 and Saxena–Molnar–Livshits ScriptGard
> CCS'11 — both empirical studies of real placement failure; see fetch-requests.]

> [A-livshits-chong-automatic-placement-2013]:p4 §3.1 — placement correctness is
> orthogonal to analyzer precision (relevance +SURE):
> "This work is not directly concerned with the precision or soundness of the analysis
> used to produce the dataflow graph: improvements to the precision and soundness of
> analyses for dataflow graph construction will seamlessly improve the quality and
> soundness of our results."

> [A-livshits-chong-automatic-placement-2013]:p12 §5.1.2 / Figure 14 — the
> instrument-fewer-points payoff (relevance +SURE):
> "Savings in terms of the number of instrumentation points in the last column of
> Figure 14 are 6.19× on average."
> [Figure 13 / p11 context: the naive taint-everything baseline must instrument as many
> as "60+% of nodes"; the purely-static node-based placement instruments far fewer but
> "in most cases it fails to provide sanitization on all paths" — so few-and-correct
> requires the edge-based analysis.]

> [A-livshits-chong-automatic-placement-2013]:p12 §6 — they INFER permit-points rather
> than require annotation (relevance +SURE):
> "Hammer et al. require certain nodes in a program dependence [graph] to be annotated
> as declassifiers, whereas we seek to infer where to insert declassifiers and
> sanitizers."

> [A-livshits-chong-automatic-placement-2013]:p13 §7 — the honest limitation: no notion
> of optimal placement (relevance ~SUSPECT):
> "However, we do not offer a formal notion of optimality of our algorithms. This is in
> part because it is unclear what we should aim to optimize. Possible candidates include
> reducing the number of instrumentation points or run-time overhead for the worst-case
> or average-case workloads."

Read for the receipts plane:
- finding-8: the paper is direct empirical/argumentative support for the "humans
  misplace permit-points" premise behind 220:vp-27. Note the *direction* of their
  conclusion differs from Dorc's: they remove the human and INFER permit-points; Dorc
  keeps the human but makes the permit-point an explicit, owned, capability-style
  oracle-claim (the License type constructible only from a claim). Both share the core
  finding — *implicit, developer-scattered* permit-points are the failure mode. ~SUSPECT
  the contrast is actually complementary: Dorc's oracle-author is exactly the
  "specialist who gets it right" that Livshits–Chong say the average developer is not.
- finding-9: the orthogonality quote (p4) is a clean prior-art anchor for Dorc's
  best-effort posture — placement/licensing correctness does not depend on analyzer
  precision; a more precise analyzer just yields fewer/tighter licenses. The 6.19×
  reduction is the quantitative shape of "permit-points are FEW relative to the values
  that flow through them."
- The no-optimality admission (p7) is a caution: Dorc should likewise NOT claim its
  license placement is optimal, only correct-and-few. There is no agreed objective to
  optimize against, which matches Dorc's "we like extra work on non-human timescales"
  stance — minimizing license count is not obviously the right objective.
- Their edge-based "spill to run time only when static placement can't decide" mirrors
  Dorc's probe-then-apply staging (static where possible, measure live when necessary) —
  -GUESS a structural echo worth noting, though their run-time taint-tracking is not a
  Dorc mechanism.

---

## Carata et al., "A Primer on Provenance" (ACM Queue, 2014) — FULL READ, Seltzer author copy

Read full from local PDF by this agent (the previously-403'd full text). Source:
`[A-carata-primer-2014]`. Printed folios 1–14 coincide with extractor sheets; locators
are those page numbers. This read closes the gap a prior round flagged: note 220's
`B-carata-primer-2014` had only the front material; the load-bearing TAIL (the overhead
section, the system-properties table, SPADEv2's fusion mechanism, the
granularity/noise/n-by-m tradeoffs, and pruning-as-open-problem) is captured below.

What this paper is, for the receipts plane: a practitioner survey of eight provenance
systems (PASS/PASSv2, SPADE/SPADEv2, VisTrails, ZOOM, Burrito, SPROV, Lipstick, RAMP)
along five axes — what's captured (granularity, layering), integration effort, querying,
overhead, security. It is the corroborating evidence for 220:vp-23 (capture-without-
consumer drowns systems) and supplies the overhead numbers that show payload size is NOT
the killer — noise and unbounded capture are. See finding-10..11.

Load-bearing excerpts — the overhead tail (the part the prior round was missing):

> [A-carata-primer-2014]:p9 §Understanding overhead / Time overhead — the headline
> numbers (relevance +SURE):
> "Both ZOOM and VisTrails, for example, report an approximately 1 percent increase in
> execution time. … Kernel-based system-call interception mechanisms such as in PASSv2
> have a 1 to 23 percent overhead on workloads representative of real-world applications.
> Similarly, SPADEv2, which uses kernel auditing infrastructure for provenance capture,
> reports less than a 10 percent overhead on Windows, Linux, and OS X for production
> Apache runs. … For I/O-heavy workloads, however, provenance capture may impose larger
> runtime overheads. PASS, for example, reports up to a 230 percent overhead on small
> file benchmarks, even though the absolute increase in execution times remains small."

> [A-carata-primer-2014]:p9 — the interception-mechanism sensitivity, and fine-grained
> cost (relevance +SURE):
> "SPADEv2, for example, supports operation interception via the kernel auditing
> mechanisms on OS X, while on Windows it requires a file-system filter driver that
> relays operations to the provenance collector. As a consequence, provenance-enabled
> Apache builds are 50 percent slower on Windows but only 5 percent slower on OS X. …
> it is common for the cost of provenance capture to equal or exceed the cost of the
> recorded operation, leading to slowdowns exceeding 100 percent. For example, in the
> Lipstick system, operator-level provenance is reported to lead to a slowdown of two to
> three times, while in the RAMP system … it is common to observe a temporal overhead of
> up to 75 percent."

> [A-carata-primer-2014]:p9 §Spatial overhead — the storage numbers (relevance +SURE):
> "The general-purpose PASSv2 system requires, on average, approximately 20 percent
> additional space overhead (as compared with the original output size) to log all the
> operations for a workload representative of real-world applications. … The Burrito
> system, running on a real user workload, required 800 MB for provenance storage and
> 2 GB for file versions over a two-month period. … These results indicate that storage
> overhead should not be prohibitive for most cases."

> [A-carata-primer-2014]:p10 §Overhead tradeoffs — granularity-vs-overhead, and delayed
> construction (relevance +SURE):
> "Generally speaking, there is a direct tradeoff between capture granularity and
> provenance overhead. … Most systems also delay provenance construction in order to
> minimize capture overhead. PASSv2, for example, captures raw operation records,
> converting them to their final representation via an asynchronous user-space daemon. …
> Other systems delay provenance collection to query time to avoid wasting resources
> computing provenance that will never be accessed. For example, Lipstick carries out
> provenance construction only when a query is made."

The noise / n-by-m / granularity tradeoffs (the second part the prior round wanted):

> [A-carata-primer-2014]:p4 §What can it capture? / Granularity — the noise problem and
> the heuristics admission (relevance +SURE):
> "Consider a Python script that copies one file to another. When running the script,
> the Python interpreter will first read and load any required modules from disk. Thus,
> beyond the dependency on the actual input, the final provenance graph will link the
> output file to all the Python modules used by the interpreter. This extra data can make
> it difficult to sift through the provenance graph as an end user, so, generally,
> heuristics are needed to determine which entities are important and which should be
> ignored."

> [A-carata-primer-2014]:p4–5 §The n-by-m problem (relevance +SURE):
> "the n-by-m problem, where a program reads n input files and writes m output files.
> Even when tracing system calls for individual reads and writes, it's not possible to
> infer which reads affected a particular write, so the provenance graph has to link each
> output file to all of the inputs. A system that is unaware of the semantics of
> individual data transformations within a process will always present a number of such
> false-positive relationships. Both PASS and VisTrails have this problem, as they treat
> the process or each workflow step as a black box."

> [A-carata-primer-2014]:p6 §Cooperation between layers — SPADEv2's fusion/composition
> filters (relevance ~SUSPECT, the SPADEv2 figure the prior round cited):
> "SPADEv2, for example, uses a multisource fusion filter (with process ID as a tag) to
> combine provenance data from multiple sources describing the same event and working at
> the same level of abstraction. When provenance is reported at different levels of
> abstraction, SPADEv2 uses a cross-layer composition filter that has the same purpose."

> [A-carata-primer-2014]:p7 §Integrating provenance — disclosed vs observed, the trust
> axis (relevance +SURE):
> "As a group, the literature refers to these as disclosed provenance systems, and they
> are recognized for their ability to offer improved semantic descriptions of provenance.
> The trustworthiness of the provenance captured in this way, however, is a concern when
> running in untrusted environments. … [observed systems] tend to have the lowest
> intrusiveness. … Observed provenance systems have their own shortcomings, however,
> mostly because of the loss of semantic information when treating each process as a
> black box."

> [A-carata-primer-2014]:p11 §Research challenges — pruning/querying still open
> (relevance +SURE, the pruning-as-open-problem point):
> "Despite the research carried out so far toward querying and visualizing provenance,
> these are still challenging problems. … even small provenance graphs can easily contain
> thousands of nodes." [§How do you answer questions, p8] And: "Moving beyond human
> queries, provenance should be made available to applications, allowing automated
> validation of inputs, limiting error propagation, or self-diagnosing changes in output
> quality." [§Computing with provenance, p11]

> [A-carata-primer-2014]:p10 §Security issues — provenance needs its own access policy
> (relevance ~SUSPECT):
> "It is imperative for provenance data to be secured against unauthorized access and not
> to leak any information about the data against which it is collected. Fundamentally,
> this requires provenance to be managed under different access policies than those of
> the data. … the security aspects of provenance are defined as its confidentiality (only
> authorized parties can read it) and its integrity (it cannot be forged or altered)."

Read for the receipts plane:
- finding-10: the overhead numbers settle the "is per-value capture affordable" worry —
  every shipping system lands in a tolerable band (disclosed ~1%; observed system-call
  1–23%; SPADEv2 <10%; spatial ~20%); the I/O-heavy and fine-grained outliers (PASS 230%,
  Lipstick 2–3×, RAMP 75%) are exactly the *finest* granularities. Corroborates 220:vp-23
  and the broader note-220 r-4 thesis ("payload bloat per se killed almost nobody"): the
  killer is noise + capture-without-consumer, not bytes-per-value.
- The Python-modules noise example + "heuristics are needed to determine which entities
  are important" is the canonical capture-without-consumer pathology; the n-by-m problem
  is the structural source of false-positive edges. Dorc inoculation (already in 220:vp-23):
  build receipts against the *closed* consumer list (⊤-blame; license-witness;
  refusal-delta; dashboard why-not; erasability gate) and refuse speculative capture —
  Dorc's origin sets are NOT a black-box n-by-m link-everything, they are computed from
  the analyzer's own dataflow, which is precisely the semantic information PASS/VisTrails
  lack.
- finding-11: disclosed-vs-observed is the trust axis that maps cleanly onto Dorc's
  claim-vs-receipt split — disclosed = author-attested = better semantics but trust-on-
  the-author (Dorc's oracle-claims); observed = cheap, inferred, semantics-poor, "a
  concern in untrusted environments" (Dorc's inferred receipts, which may only refuse/
  explain). The primer independently arrives at the same reason receipts must not grant:
  observed/inferred provenance is not trustworthy enough to be authoritative.
- The "different access policies than the data" point (p10) is a -GUESS prior-art anchor
  for keeping the receipts plane separate from the value/where plane (220:vp-20) and for
  the erasability gate — provenance can leak about its subject and may warrant its own
  visibility rules. Weaker relevance; flagged ~SUSPECT.
- "Computing with provenance" as future work (p11) — provenance made available to
  *applications* for "automated validation of inputs, limiting error propagation" — is
  exactly Dorc's machine-consumer stance (receipts consumed by the analyzer's license
  check, not just a human dashboard). The primer lists this as aspirational in 2014;
  Dorc's closed-consumer design is the concrete instance.

---

## Fetch-requests for the human

Nothing in the four target primaries was unreachable — all read in full from local
copies. The items below are OPTIONAL follow-ups surfaced in the reference chains, none
blocking this round:

- **fetch-opt-1 (the ZM01 enforcement sequel).** finding-2 notes ZM01 gives the
  *semantic property* of robust declassification but not an *enforcement* (type/dataflow)
  mechanism. The canonical sequel is Myers, Sabelfeld & Zdancewic, "Enforcing Robust
  Declassification" (CSFW 2004) / its JCS 2006 journal version. If round-22 wants the
  one-way rule as an actually-enforceable typing discipline (not just a property to cite),
  that paper is the source. Best URL: `https://www.cs.cornell.edu/andru/papers/` (Myers's
  author page lists csfw04/jcs copies). Why needed: only if the design moves from "cite
  the property" to "adopt an enforcement rule." Not required now.

- **fetch-opt-2 (the two placement-failure assessments Livshits–Chong leans on).** The
  "well beyond the ability of developers" claim (p1) cites [38] Samuel/Saxena/Song,
  "Context-sensitive auto-sanitization …" (CCS 2011) and [39] Saxena/Molnar/Livshits,
  "ScriptGard …" (CCS 2011). If a primary-source measurement of *human misplacement rate*
  is wanted (rather than Livshits–Chong's secondhand summary of it), those two are the
  empirical origin. Both are ACM DL; ScriptGard has a Microsoft Research author copy.
  Why needed: only to harden finding-8 from "asserted by L–C" to "measured in [38,39]."
  Low priority — note 220 already treats vp-27 as well-supported.

I did not fight any paywall; no archive copies were created (per the saved local copies
in `.claude/research/r22-rqA/staging/`, which are scratch, not committed archives).

---

## Graded sources

Grades assigned by gathering subagent; conductor re-verification pending. Read-depth:
full = whole paper read; targeted = specific sections; snippet = abstract/excerpt only.
All four were read full. Grade letter encodes source quality (A = peer-reviewed primary,
canonical author copy, no rot); the letter lives in the slug, so a slug change ⇒ a
grade change.

- `[A-zdancewic-myers-robust-declassification-2001]` · Zdancewic & Myers, "Robust
  Declassification" · `https://www.cs.cornell.edu/andru/papers/csfw01.pdf` · published
  IEEE CSFW 2001 · read-depth full · grading: A not B because it is the peer-reviewed
  CSFW primary in the canonical author-posted Cornell copy with no rot, AND notably a
  *corrected* version (fn4 fixes a proceedings-version error) — strictly better provenance
  than note 220's secondhand survey-read it replaces; would only be B if access were
  degraded/secondhand, which it is not. · Relevance: the named origin of robust
  declassification, closest formal statement of Dorc's one-way receipts rule (220:vp-26);
  attacker model + the `glb(D)` who-declassified blame construction (220:vp-16) both
  transfer. · Via: rq-A list / predecessor's turn-1 read.

- `[A-green-karvounarakis-tannen-provenance-semirings-2007]` · Green, Karvounarakis &
  Tannen, "Provenance Semirings" · `https://web.cs.ucdavis.edu/~green/papers/pods07.pdf` ·
  published ACM PODS 2007 · read-depth full · grading: A not B because it is the seminal
  peer-reviewed PODS'07 primary (origin of the semiring provenance framework, thousands of
  citations) in the canonical author-posted UC Davis copy, no rot, fully read; note 220
  held it at B only by access-grade ("not directly read"), which no longer applies. The
  local copy is the author preprint (no ACM folios) — a quality-neutral packaging detail,
  not a grade reduction. · Relevance: formal source for 220:vp-3/16..19 — factorization
  theorem (polynomials universal, coarser forms are homomorphic images), why-vs-how
  limitation (motivates minimal-witness-at-licenses, vp-17), distributive-lattice
  containment collapse (corroborates avoiding how-polynomials, vp-19). · Via: rq-A list /
  predecessor's turn-1 read. (Conductor housekeeping: the predecessor's stale
  `B-green-karvounarakis-tannen-provenance-semirings-2007` duplicate key + archived PDF in
  the r22-rqA sources.json should be pruned; cited nowhere.)

- `[A-livshits-chong-automatic-placement-2013]` · Livshits & Chong, "Towards Fully
  Automatic Placement of Security Sanitizers and Declassifiers" ·
  `https://people.seas.harvard.edu/~chong/pubs/popl13-automatic-placement.pdf` · published
  ACM POPL 2013 · read-depth full · grading: A not B because it is the peer-reviewed POPL
  primary in the canonical author-posted Harvard copy, no rot, fully read in main context;
  note 220's `B-livshits-chong-2013` was the access-grade for a not-directly-read source,
  now superseded. Directly on-point and foundational for the permit-point-placement
  argument. · Relevance: the load-bearing evidence for 220:vp-27 (humans misplace
  permit-points → make them few/explicit/owned); also supplies the placement-is-orthogonal-
  to-precision result (Dorc's best-effort posture) and the 6.19× few-points payoff. · Via:
  rq-A list (downloaded by predecessor, read by this agent).

- `[A-carata-primer-2014]` · Carata, Akoush, Balakrishnan, Bytheway, Sohan, Seltzer,
  Hopper, "A Primer on Provenance" · `https://www.seltzer.com/assets/publications/A-Primer-on-Provenance.pdf` ·
  published ACM Queue 12(3) / CACM 2014 · read-depth full · grading: A not B because it is
  a peer-reviewed ACM Queue article in the canonical author-posted (Seltzer) copy, no rot,
  now fully read including the previously-403'd tail; note 220 held it at B with only the
  front material accessible, which no longer applies. It is a survey rather than a results
  primary, but an authoritative, peer-reviewed one by the PASS/Burrito principals — an A
  for what it is (a practitioner survey), not over-graded as if it were a primary results
  paper. · Relevance: corroborates 220:vp-23 (capture-without-consumer) with concrete
  overhead numbers (payload size is not the killer; noise + unbounded capture are), the
  n-by-m and granularity/noise tradeoffs, SPADEv2's fusion mechanism, and disclosed-vs-
  observed as the trust axis mapping onto Dorc's claim-vs-receipt split. · Via: rq-A list
  (downloaded by predecessor, tail read by this agent).
