# 094 — conversation findings: the idempotency-guard carrier, spec-mining, the grounding chicken-and-egg (round 9, 2026-06-02)

> **Provenance: design-reasoning from conversation, NOT yet source-grounded.** These emerged in
> dialogue (the "best-case script" challenge + the `wombat` chicken-and-egg), *ahead* of their
> research fronts — so they are **hypotheses to source-ground, not graded findings**. Fronts
> **F8 (grounding/type-discovery)** and **F7 (specification mining)** exist to verify or break
> them (plan 090 §4). All ~SUSPECT pending that grounding. Strawmen kept minimal/idiomatic per
> AGENTS (the elaborate best-case script lives in chat, deliberately un-persisted, so it can't
> become an accidental design-prompt).

## Findings (lifted)

- **g1 — the idempotency guard is the spec-carrier.** A best-case idiomatic ops script, written
  purely for its own idempotency, *incidentally* carries a recoverable spec. The unit is the
  **idempotency guard** — `if ! PROBE; then ESTABLISH; fi` and `PROBE || ESTABLISH`. Within it:
  the **shared literal argument** is the entity-link; **guard polarity** marks probe-vs-establisher;
  `set -e` + textual order incidentally specify the dependency DAG; a `cmp SRC DST` guard before
  `cp SRC DST` reveals a **data-dependency** (`DST \from SRC`) via the shared source-arg.
  Idempotency-checking *is* the probe/establish/link specification.
- **g2 — consumer-guard and oracle are the same spec, in two places.** The guard a careful `book`
  author writes incidentally *equals* the spec an `oracle` publishes deliberately. A **bare**
  command (`apt-get install nginx`, no guard) carries only the mutation → the oracle is "the
  published, reusable idempotency-guard" supplying the missing probe. Reconciles the archive's
  "consumer-guard-lifting is a minor bonus, not the model" (`archive/ansible-conversation-text.md:770`)
  with the TS-narrowing framing: the guard is the unit either way; the oracle just makes *bare*
  books skippable too.
- **g3 — guards give PROPAGATION, not GROUNDING (the "too simple" correction).** The guard is only
  meaningful once you already know `dpkg -s` is a probe, `apt-get install` an establisher, and the
  shared token an entity *of a kind*. Strip that — `if widget wombat; then hork_an snuffler wombat; fi`
  — and the structure grounds nothing.
- **g4 — the grounding chicken-and-egg = Rice at the bootstrap (the symbol-grounding problem).** You
  cannot *soundly* co-infer {what kind `wombat` is} and {what `widget`/`hork_an` do to it} from
  structure alone — mutually defining, no fixed point without an anchor. The shared token gives
  **co-reference** ("same string"), never **grounding** ("…and it is mutable kind-K state, safe to
  probe at rest"). Co-reference-by-token is sound only *within one script*; **across two oracles a
  shared token may be a name-collision**, so cross-oracle kind-identity must bind to a *named kind*
  (the Puppet RAL — note 092 `f17`), never be inferred from the argument-string.
- **g5 — resolution = the q-floor's *shape*: ≥1 grounded anchor per kind.** Tell Dorc *one* grounding
  (this command probes kind-K / this entity is kind-K) and the guard structure *propagates* the rest
  soundly across the script; anything with no reachable anchor is ⊤ → just run it (the safe floor).
  The chicken-and-egg is the *proof the floor is non-empty* and the statement of its *minimal shape*.
  [contract · q-floor]
- **g6 — the distributional escape is unsound.** Across a large corpus, `widget X` … `hork_an …X`
  co-occurrence can *cluster* a latent kind ("wombats are whatever widget+hork_an operate on" —
  Firth, "know a word by the company it keeps"), but that buys *hints*, not grounded elision-safety.
  The **sound (anchored) vs unsound (mined/distributional) boundary** is the central question for F7.

## Naming (candidates to verify in the fronts, not banked from memory)
- **specification mining / spec inference** — the SE/PLT name for deriving a spec from un-annotated
  idiomatic code (seed: Ammons et al., POPL 2002). **Human added it to the core README** → F7 must
  confirm it is *very* relevant, not merely adjacent.
- **symbol grounding problem** (Harnad 1990) — the impossibility behind g4.
- design word: **grounding** (type-discovery) vs the guards' **narrowing/propagation** — the two
  halves; F8 owns grounding.

## Minimal motivating snippet (idiomatic; NOT a Dorc-design artifact)
`if ! dpkg -s nginx; then apt-get install -y nginx; fi` — shared `nginx` links probe↔establisher;
but `if widget wombat; then hork_an snuffler wombat; fi` (all-unknown) shows the structure needs a
grounded anchor to mean anything (g3/g4).

## Open / next
- Conversation-hypotheses; **F8 then F7** source-ground or break them (plan 090 §4 / tasks #3, #4).
- Load-bearing outputs to carry to synthesis: **g5 (q-floor = anchor-per-kind)** and **g6 (the
  sound/unsound boundary)**.
