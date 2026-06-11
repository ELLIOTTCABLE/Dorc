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
  Tannen's why-vs-how passage (p32): two output tuples can share why-provenance `{r,s}`
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
