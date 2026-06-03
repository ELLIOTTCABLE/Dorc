# 096 — F7 (specification mining): the field, and Engler's MUST/MAY = Dorc's sound/unsound boundary (round 9, 2026-06-02)

> Front F7 (task #4) — the human added "specification mining" to the core README, so this is a *hard
> relevance check*, not a glance. **Keystone read: [A-engler-deviant-behavior-2001] (core, full).**
> Ammons et al. (POPL 2002) named as the field-origin (abstract-level; full read deferred — follow-up).
> Read with the *relational* lens fixed by the 095 human-adjudication (Dorc keeps relational contracts
> about referent-agnostic symbols; it does not "ground" them).

## Findings (lifted)
- **f29 — spec-mining IS the right field-name, but it is *statistical/unsound by nature*.** Ammons,
  Bodík & Larus, "Mining Specifications" (POPL 2002) coined it: "a machine-learning approach to
  discovering formal specifications of the protocols that code must obey." Its mechanism is a *vote*:
  "the miner can collect and summarize API protocols followed by various programmers (some of them
  wrong), effectively **taking their vote on which protocol is correct**." Consequence (Le Goues &
  Weimer, "Specification Mining With Few False Positives"): mined specs have **high false-positive
  rates** and need **trustworthiness** filtering. So mining *proposes*, it does not *certify*. → README
  term **justified** — as the name for "derive the relational contract from un-annotated code" — *with
  the caveat that mined ≠ trusted.*
- **f30 — Engler's MUST vs MAY belief = the exact sound/unsound boundary (keystone).**
  [A-engler-deviant-behavior-2001] §1–2: "**MUST beliefs are directly implied by the code, and there is
  no doubt** … A pointer dereference implies the programmer must believe the pointer is non-null. **MAY
  beliefs are cases where we observe code features that suggest a belief but may instead be a
  coincidence** … A call to 'a' followed by 'b' implies the programmer *may* believe they must be paired,
  but it could be a coincidence." Handling: MUST → "any contradiction implies an error" (sound); MAY →
  "assume all MAY are MUST … then use **statistical analysis to rank each error by probability** (999/1000
  ⇒ valid; once ⇒ coincidence)" (unsound). This maps onto Dorc *exactly*:
  - **MUST = Dorc's elision-safe relational contract** = *directly implied by idiomatic structure* (the
    idempotency guard → occurrence-typing narrowing, note 092/094 g1) **or** *oracle/anchor-declared*
    (the q-floor, 094 g5 / 095-relational). Sound; elision may rely on it.
  - **MAY = the distributional/mined guess** = corpus co-occurrence (095 f27). Statistical, ranked,
    **never elision-safe**.
- **f31 — the verdict (the relevance check the human asked for).** Specification mining is
  *adjacent-and-useful*, **not** Dorc's elision-grounding mechanism. It is predominantly a **MAY-belief
  generator**, so its real roles for Dorc are: (a) the **oracle-bootstrap ranking** — which protocols are
  common across the corpus ⇒ which ~40-50 oracles to write first (the network-effect bootstrap, DESIGN
  component #4/#5); and (b) donating the **MUST/MAY vocabulary**. Dorc's *elision* rests on **MUST**
  (implied-by-structure or declared); mining supplies **MAY** (ranked hints). **Mining proposes; the
  contract must be implied-or-declared to be elision-safe.**

## Net (the boundary, named)
The sound/unsound line the whole round circled is Engler's **MUST vs MAY**: a relational contract is
elision-safe only if it is **directly implied by idiomatic structure or declared by an oracle** (MUST);
everything *mined from co-occurrence* is **MAY** — a ranked hint that bootstraps the oracle library but
can never license a skip. Spec-mining is the bootstrap engine, not the elision engine.

## Citations
> [A-engler-deviant-behavior-2001]:§1-2 (relevance: +1:SURE)
> "MUST beliefs are directly implied by the code, and there is no doubt that the programmer has that
> belief. A pointer dereference implies that a programmer must believe the pointer is non-null … MAY
> beliefs are cases where we observe code features that suggest a belief but may instead be a coincidence.
> A call to 'a' followed by a call to 'b' implies the programmer may believe they must be paired, but it
> could be a coincidence."
> "For a set of MUST beliefs, we look for contradictions. Any contradiction implies the existence of an
> error … For a set including MAY beliefs … we start by assuming all MAY beliefs are MUST beliefs and look
> for violations … then use a statistical analysis to rank each error by the probability of its beliefs."

## Open / next
- Follow-up (optional): full-read Ammons POPL'02 to confirm the definition first-hand (currently
  abstract-level). Verdict is unlikely to move.
- Carry to synthesis: **MUST/MAY is the named sound/unsound boundary**; spec-mining = bootstrap engine
  (oracle-ranking), not elision engine.
