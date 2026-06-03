# 092 — F2 corrected: flow-typing / tainting / the type-provider RAL (round 9, 2026-06-02)

> The human's AGENTS.md firming relocated this front: the contract is **not** an
> annotation layer — it is TypeScript-style *annotation-by-narrowing* (idiomatic
> code is the signal; meaning comes from contracted control-flow-tracing + tainting,
> never YAML/pragmas). So ACSL `assigns` / ShellCheck directives (read earlier this
> round) are the **wrong layer** — explicit annotation — and survive only as
> prior-art for the *minimal-OOB floor* (`kOOB-sidecar`) if idiomatic sh ever cannot
> carry a fact. The **right** prior-art is flow-typing. Terminology now firm
> (AGENTS): **oracle** = engineer-authored provider/library; **book** = admin's
> scrappy play; the two users = **admin** + **engineer**.

## Findings (lifted)

- **f14 — occurrence typing is the academic root of the TS-guard model** the human is
  hard-aiming at [A-tobin-hochstadt-logical-types-2010]. It is "a type discipline for
  exploiting the use of **data-type predicates in the test expression**" of a
  conditional to refine a variable's type *per branch*. Dorc's exact isomorphism:
  `if dpkg -s X` is a data-type predicate whose true-branch **narrows** the system-state
  type (`X ∈ package, installed`). +SURE this is the correct conceptual layer; ACSL/
  ShellCheck were right-church-wrong-pew (explicit annotation, not derived narrowing).
- **f15 — user-defined type guards (`x is T`) are the *shape* of the engineer's contract.**
  TypeScript's `function isFoo(x): x is Foo` and Flow's `param is T` let a *library
  author* mark an otherwise-ordinary predicate function as one the compiler may use to
  narrow. → The oracle-author's analog: mark an idiomatic check (`pkg_present() { dpkg -s
  "$1"; }`) as *the narrowing predicate for the `package` kind*. The **shape** is settled
  by this prior-art; the **open q-frontier** is how to spell that mark *idiomatically in
  sh* (not a DSL `x is T` keyword) so the script still runs after abandoning Dorc. [contract
  · the q-frontier]
- **f16 — flow-sensitive type qualifiers / CQual = the "tainting" the human named**
  [A-foster-flow-sensitive-qualifiers-2002]. Qualifiers are a *lightweight tag layered on
  a base type*, modelled **flow-sensitively** with **strong updates**, where "users annotate
  … and inference checks/fills." → Dorc's state-kind facts are exactly flow-sensitive
  *qualifiers* on the abstract system-state: a mutator strong-updates `package:X` from
  absent→present; a guard reads it; inference fills what the author didn't annotate (= the
  `kBURDEN` declare+infer gradient, *as one mechanism*). Practical checker, **not** a
  proving-minefield. [knob · `kBURDEN` realized as qualifier-infer]
- **f17 — the Puppet RAL (type/provider) is the state-kind + extensibility model**
  [internal: `Research/archive/ansible-conversation-text.md:178`; Puppet Resource API docs].
  *Type* = abstract capability ("package installed"); *provider* = per-platform impl
  (apt/yum/brew), "a named capability that multiple providers satisfy, selected by host
  facts," user-extensible via the Resource API. → `package` = kind; `dpkg`/`apt`/`mycmd`
  = providers (= oracles); this is the concrete answer to "how do dpkg and mycmd both
  declare they handle `package` so one's probe discharges the other's demand." Chef's
  resource/provider is the sibling. **This is the half-remembered reference.** [contract
  · the kind-universe; q-frontier = keeping it open/extensible]

## The corrected prior-art stack (how the four compose)
- **consumer (`book`, admin)** writes idiomatic sh → narrowed by **occurrence typing** (f14).
- **provider (`oracle`, engineer)** declares each command's check as a **narrowing predicate**
  for a **kind** — shape from **user-defined type guards** (f15), bound to kinds via the
  **RAL** (f17).
- **the engine** carries kind-facts as **flow-sensitive qualifiers**, strong-updated by
  mutators, inferred where undeclared (f16) — this *is* the "facts taint the CFG" mechanism.
- *Open q-frontier (the real question, per the human):* how the **kind-universe stays open
  and user-extensible** (RAL's lesson) while every mark stays **idiomatic-sh, not DSL**
  (the abandon-Dorc-and-it-still-runs test). Not resolved here — it is the design space.

## Citations
> [A-tobin-hochstadt-logical-types-2010]:abstract/§1 (relevance: +1:SURE)
> "its type system combines several preexisting elements … with the novel idea of occurrence
> typing, a type discipline for exploiting the use of data-type predicates in the test
> expression of [a conditional]."

> TypeScript Handbook, "Narrowing" (first-party docs; inline ref, not graded) (relevance: -0:SUSPECT)
> "This analysis of code based on reachability is called control flow analysis, and TypeScript
> uses this flow analysis to narrow types as it encounters type guards and assignments."
> (User-defined guards: `function isFoo(x): x is Foo` — the mark that makes an ordinary
> predicate a narrowing one.)

> [A-foster-flow-sensitive-qualifiers-2002]:abstract (relevance: +1:SURE)
> "We present a system for extending standard type systems with flow-sensitive type qualifiers.
> … only the type qualifiers are modeled flow-sensitively — the underlying standard types are
> unchanged … integrates flow-insensitive alias analysis, effect inference, and ideas from
> linear type systems to support strong updates."

> internal `Research/archive/ansible-conversation-text.md`:178 (relevance: +1:SURE)
> "Resource-type vs. provider (the RAL). Puppet separates the declarative interface ('package X
> installed') from the per-platform implementation (apt/yum/brew). … a named capability that
> multiple providers satisfy, selected by host facts."

## Demoted (recorded, not elevated)
- **ACSL `assigns` clause** (`assigns \nothing` = pure; absent = assigns-anything = ⊤; `\from`
  = data-dependency) and **ShellCheck directives** (in-comment, scoped to the *next
  command/compound structure*) are concrete **explicit-annotation** prior-art. Per AGENTS they
  are **not** the model; retain only as the `kOOB`-floor fallback shape for the irreducible bit
  that idiomatic sh cannot carry. Do not elevate.

## seq-1 full-read enrichment — occurrence typing's actual mechanism (2026-06-02)
Full-read of [A-tobin-hochstadt-logical-types-2010] §2–§3.5 (not the abstract); grade A stands
(+1:SURE), now from the mechanism. The machinery is sharper — and it **formally unifies the
narrowing half (this note) with the grounding half (note 094 g4/g5):**

- **latent proposition** — a predicate's *type* carries two propositions: what it proves when it
  returns true, and when false (`number?` : `y:τ —[N_y | ¬N_y]→ B`). The paper explicitly borrows
  "latent" from effect systems [A-lucassen-gifford-effect-systems-popl-1988, in-corpus] → lineage
  closes: latent *effects* → latent *propositions* → Dorc's latent *state-propositions on a command*.
  **This latent proposition IS the per-command grounding anchor the q-floor needs (094 g5).** An
  oracle grounds `has_pkg` by giving it "true ⟹ `package:X` installed; false ⟹ ¬installed." A command
  with *no* latent proposition cannot narrow → ⊤ → run. The chicken-and-egg (094 g4) dissolves
  precisely: **supply one latent proposition = the minimal anchor.**
- **object** — each expression derives an *object*: which part of the state it accesses (`x`; or the
  **selector** `car(p)`). → Dorc's "object" = the entity/access-path into shared state. **Selectors
  ⇒ kinds are *structured*** (a package has `installed` *and* `version`; a service `enabled` *and*
  `active` — the strawman's `is-enabled`/`is-active` are two selectors on `service(nginx)`), richer
  than a flat shared token.
- **narrowing = substitution** of the object into the latent proposition (`number? x` → sub `x` for
  `y` in `N_y` → `N_x`). Guard **negation/polarity** (`if ! …`) and `and`/`or` compose propositions →
  the then-branch of `if ! has_pkg X` carries `¬installed(X)` = "install is needed." Formalizes g1.

Net: occurrence typing is not merely the *analogy* — its **(latent-proposition, object,
substitution)** triple is a ready-made formal spine, and the latent-proposition is the exact locus
where grounding (F8) attaches. Carry to synthesis.

## seq-1 full-read enrichment — CQual's actual mechanism (2026-06-02)
Full-read of [A-foster-flow-sensitive-qualifiers-2002] §3.2–§3.4; grade A stands (+1:SURE), from the
mechanism. CQual is the **second formal spine** (complements occurrence typing) and it pins the
*precision* story directly to the ceilings:
- **state-facts = flow-sensitive *qualifiers*** decorating entities; "flow-sensitivity is restricted to
  the qualifiers — the underlying standard types are unchanged." → Dorc: the script *structure* is
  fixed/flow-insensitive; only the *state-facts* flow. (Efficient, and exactly Dorc's shape.)
- **strong vs weak update is gated by *linearity* (uniqueness):** linear/unique entity → **strong
  update** (fact becomes precisely V); non-linear/maybe-aliased → **weak update** (join → ⊤-ward).
  **This is the aliasing ceiling (note 093 f19) in operation:** you may strong-update only when you can
  *prove* the entity uniquely identified; Ramalingam says that is undecidable in general, so weak-update
  (precision loss) is the forced fallback. Strong/weak *is* where f19 trades precision.
- **effects = the frame:** the (App) rule gates flow by a function's *effect set* L — "functions do not
  act as join points for locations they do not use." Footprint = effect; everything outside L is
  framed/unchanged (note 093 f21). The frame falls out of the declared/inferred effect.
- **declare + infer (= `kBURDEN`):** qualifiers are either *constants specified by the user* (with a
  *user-supplied partial order* — the lattice) or *variables inferred*. → the user/oracle supplies the
  **kind lattice** (`package` + its states/order) + anchors; inference propagates. **The user-supplied
  qualifier lattice is a second grounding locus** (beside occurrence typing's latent proposition):
  type-discovery (F8) = "declare the kinds + their order," exactly CQual's constant-qualifier lattice.

Net: two complementary spines — *latent-proposition narrowing* (occurrence typing) and *flow-sensitive
qualifier update* (CQual) — both locating grounding in user-supplied structure, with strong-vs-weak
update as the precise seam where the aliasing ceiling (f19) bites.
