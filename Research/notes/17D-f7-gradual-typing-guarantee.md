# 17D — f7 gather: gradual typing + the gradual guarantee (round 17, 2026-06-07)

> Charter `170` front f7. Read in full + graded A: [A-siek-refined-criteria-gradual-typing-2015] (the
> gradual guarantee). Read targeted (abstract/intro/method) + graded A:
> [A-takikawa-sound-gradual-typing-dead-2016] (the performance death). Siek-Taha 2006 (the GTLC origin) not
> separately graded — its consistency-relation + cast-insertion content is recapped in full inside the
> Refined-Criteria paper, which I read. Closes f7's include/kill rows; the f6+f7 through-line is the headline.

## Findings (lifted)
- **f7-1 · inc-2 CONFIRMED but ASYMMETRIC — the gradual guarantee transfers to Dorc only one direction.**
  The guarantee: removing annotations from a well-typed program keeps it well-typed and value-equivalent;
  adding *correct* annotations preserves behavior; adding a *wrong* annotation can only cause a (loud)
  trapped error. Mapping onto Dorc's no-cliff:
  - **REMOVE direction → sound for Dorc.** Drop an oracle ⇒ less precise ⇒ ⊤ ⇒ degrade to the apply-1
    floor ⇒ the book still runs, equivalently. This *is* the off-ramp / no-cliff, and it genuinely holds.
  - **ADD direction → does NOT transfer as a safety property.** GT's "wrong annotation ⇒ loud trapped
    error, never silent corruption" is enforced *by runtime casts*. Dorc is cast-free ⇒ a wrong oracle is
    **not** caught ⇒ it can cause a silent wrong-skip (`under-execute`). That gap *is* the T12
    unverifiable-oracle hazard, and *is* why the oracle must own the safety-direction (17B / oq-1).
  ⇒ Adopt the gradual guarantee as the no-cliff law, but explicitly: only its remove-half is a Dorc
  *guarantee*; its add-half is *best-effort* because we omit casts. +SURE.
- **f7-2 · kill-2 CONFIRMED — take the consistency *relation*, kill the cast/blame *machinery*.** The
  static consistency relation (`unknown ~ anything`; symmetric, not transitive) = Dorc's
  `⊤`-consistent-with-all. But GT's *dynamic* semantics is cast-insertion into an internal cast calculus
  (injections/projections, wrapped functions, `AppCast`, blame tracking) — all **runtime enforcement**.
  Dorc never checks consistency to reject and inserts no casts ⇒ kill the entire cast/coercion/blame
  apparatus; keep only the `⋆`-relates-to-all idea (which we already have as ⊤). +SURE.
- **f7-3 · kill-3 CONFIRMED, sharply — the "death" is boundary-cast cost, a dimension Dorc rejects by
  design.** Takikawa: soundness requires run-time checks at the **typed/untyped boundary** (macro-level
  behavioral contracts / micro-level casts); their granularity = the overhead; mixed partially-typed
  configs hit ~2 orders of magnitude; fully-typed & fully-untyped are both fine — *the boundaries are the
  cost*. Dorc is cast-free (probe = one-time host observation, not a per-value cast; oracle = behavioral
  no-op, not a contract/proxy) and does **not** enforce soundness at runtime (optimizer-not-checker,
  `kVERIFY`). So the mechanism that kills sound GT is structurally absent — not merely N/A, *opposite by
  design*. +SURE.
- **f7-4 · THROUGH-LINE (f6+f7, the round's spine) — the cast calculus is one hinge wearing three hats.**
  The same runtime-cast machinery is (a) what gives GT the gradual guarantee's *add-direction* safety
  (f7-1), (b) what Takikawa's *performance death* kills (f7-3), and (c) what Dorc *structurally lacks*
  (cast-free). So Dorc trades boundary-safety for boundary-freedom and recovers safety only via the
  apply-1 floor + the **oracle-declared** direction. This is the same shape as f6-3 (Dialyzer bakes its
  direction only because Erlang's runtime tags are a uniform free backstop). Two independent forgiving-
  lineage sources converge on: *runtime enforcement is the thing Dorc omits, and its omission is exactly
  why direction must come from the oracle, not the engine.* ~SUSPECT→SURE.
- **f7-5 · BONUS — kill-6 (structural) + inc-3 (no-op-not-proxy) corroborated by the guarantee's own
  failure cases.** §5.5: structural type-tests (`e is T`) break the gradual guarantee under *every* tested
  semantics (optimistic/pessimistic/top-constructor) — supports **kill-6** (structural identity is where
  no-cliff breaks; stay nominal, inc-8). §5.7: proxies/wrappers interfere with object identity (Reticulated
  Python) — supports **inc-3** (the oracle must be a behavioral no-op, *not* an identity-changing proxy).
- **f7-6 · BONUS — the precision lattice = the kBURDEN gradient, formalized.** The partial order on type
  precision (`T ⊑ ⋆`) over differently-annotated versions of one program (Fig 5) is exactly the
  we-infer↔user-declares gradient; the gradual guarantee is the formal statement that moving along it has
  no cliff. Good vocabulary for the map.

## Citations
> [A-siek-refined-criteria-gradual-typing-2015]:p6 (relevance: +1:SURE)
> "The gradual guarantee says that if a gradually typed program is well typed, then removing type
> annotations always produces a program that is still well typed. Further, if a gradually typed program
> evaluates to a value, then removing type annotations always produces a program that evaluates to an
> equivalent value."

> [A-siek-refined-criteria-gradual-typing-2015]:p13 (relevance: +1:SURE)
> "When adding type annotations, if the program remains well typed, the only possible change in behavior is
> a trapped error due to a mistaken annotation." (the add-direction safety — enforced by casts.)

> [A-siek-refined-criteria-gradual-typing-2015]:p6 (relevance: +1:SURE)
> "the consistency relation, written T1 ∼ T2 … is more liberal when it comes to the unknown type: it
> relates any type to the unknown type … In contrast to subtyping, consistency is symmetric but not
> transitive."

> [A-siek-refined-criteria-gradual-typing-2015]:p4 (relevance: -0:SUSPECT)
> "the runtime system protects the static typing assumptions by casting values as they flow between
> statically and dynamically typed code … gradual typing ensures that statically typed regions of code are
> free of runtime type errors." (the cast machinery = kill-2; the backstop Dorc lacks.)

> [A-takikawa-sound-gradual-typing-dead-2016]:p1 (relevance: +1:SURE)
> "Realizing type soundness in this world requires run-time checks that watch out for potential impedance
> mismatches between the typed and untyped portions of the programs. The granularity of these checks
> determine the peformance overhead of gradual typing."

> [A-takikawa-sound-gradual-typing-dead-2016]:p1 (relevance: +1:SURE)
> "We find that Typed Racket's cost of soundness is not tolerable. If applying our method to other gradual
> type system implementations yields similar results, then sound gradual typing is dead."

## Carry-forward
- Siek-Taha 2006 (GTLC origin) remains ungraded; pull + grade only if the map needs the original
  consistency/cast formalism beyond the Refined-Criteria recap (-GUESS: it won't).
- f7-4 + f6-3 should be stated as a single named result in the map ("runtime enforcement is what Dorc omits
  → direction is the oracle's"); feeds oq-1 and #7.
- Efficient Gradual Typing (arXiv 2018) + later "fast sound GT" work surfaced but not pulled — only
  relevant if we ever reconsider runtime enforcement (we don't; welded cast-free). Note, don't gather.

## Correction (human steer, turn 8) — runtime enforcement is OPTIONAL/lint-like; defense-in-depth tenet-0
The "cast-free / no runtime enforcement" framing in f7-3/f7-4 is too absolute for the durable record.
Refinement (human; recorded for accuracy, NOT a new research thrust — do not over-weight):
- Dorc *may* have runtime enforcement, but it is **optional** and functions effectively **as lints** — it
  is NOT load-bearing for any soundness floor. Takikawa's death is the cost of *mandatory* boundary checks;
  optional lints don't incur it, so **kill-3 stands** (sharpened: Dorc rejects *mandatory* runtime
  enforcement, not all of it).
- A runtime catch's intended value is a **back-propagation loop**: the user (manually) lifts what the
  runtime backstop caught *into the statically-analyzable corpus* (a guard/oracle) ⇒ next run is
  faster/correcter, the catch is shared with other users, and the human is trained toward
  defensive-by-default authoring (catching subtle/dynamic cases runtime backstops never could). This feeds
  the corpus-growth / `effort-allocation` story (f8) and the `kBURDEN` gradient.
- **Defense-in-depth tenet-0 (governing constraint):** *no layer may depend on the correctness of another
  layer* — else it isn't depth (redundant layers), just one layer smeared across levels, each leaving holes
  it expects the others to fill. The layers — static analysis · the apply-1 floor · optional runtime lints ·
  oracle-declared direction · human defensive-authoring — must each be sound **standalone**. So restate
  f7-4 NOT as "recovers safety *only* via floor+oracle" (interlocking), but as *independent redundant
  layers*. The floor alone is sound; the oracle-direction alone is sound; lints + human-habit are additive.
