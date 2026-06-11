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
