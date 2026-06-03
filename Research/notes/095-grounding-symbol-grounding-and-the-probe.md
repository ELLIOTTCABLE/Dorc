# 095 — F8 (grounding / type-discovery): the symbol grounding problem, and the probe as Dorc's escape (round 9, 2026-06-02)

> Front F8 (task #3). Source-grounds the chicken-and-egg (094 g4) and the q-floor shape (094 g5).
> **Keystone read: [A-harnad-symbol-grounding-1990] (core, full).** Supporting named concepts
> (attributed, *not* separately full-read this turn → flagged to deepen): distributional hypothesis
> (Firth 1957 / Harris 1954) and type-class coherence (Wadler–Blott 1989). Confidence-marked.

## Findings (lifted)
- **f23 — the chicken-and-egg IS the symbol grounding problem (named, confirmed).**
  [A-harnad-symbol-grounding-1990] §2.2: meaning from symbols-defined-only-by-other-symbols is the
  "Chinese/Chinese Dictionary-Go-Round" — "passing endlessly from one meaningless symbol … to
  another … never coming to a halt on what anything meant." That is exactly
  `if widget wombat; then hork_an snuffler wombat` with every token unknown (094 g4). Learning it as a
  *first* language is the *impossible* variant; ours is only the *difficult* one — because Dorc has
  external grounding (f25).
- **f24 — meaning is *parasitic*, not intrinsic (why the analyzer can't self-bootstrap).** §2.1
  (Searle): shape-manipulated symbols have meaning "parasitic on the fact that the symbols have meaning
  for us … like the meanings of the symbols in a book." → A `book`'s tokens (`nginx`, `wombat`) carry
  nothing *intrinsic* to the analyzer; meaning is supplied by author/oracle. No amount of structure
  analysis grounds a symbol — grounding must come from outside the symbol system.
- **f25 — Dorc IS Harnad's hybrid solution; the PROBE is the grounding mechanism (keystone).** Harnad's
  escape (§2.3 + abstract): ground symbols *bottom-up in two kinds of nonsymbolic representation* —
  **(1) iconic** (raw sensory projection) and **(2) categorical** (a feature-detector picking out the
  invariant features that define a category). Dorc instantiates exactly this hybrid:
  - **probe = iconic/sensory grounding** — `dpkg -s nginx` *connects the symbol to real observed
    host-state* (Harnad's "connecting to the world," which he names the crux). **The probe is therefore
    not just an optimization — it is Dorc's escape from the regress: look at the actual machine instead
    of chasing the dictionary.**
  - **oracle = categorical grounding** — the declared check *is* the feature-detector that categorizes
    "X ∈ `package`, installed." The oracle supplies the category and its detector.
  +SURE the load-bearing reframe: probe (iconic) + oracle (categorical) = the two groundings Harnad
  prescribes; together they break the regress.
- **f26 — the q-floor = the *categorical* grounding minimum; the probe supplies the *iconic*.** (094 g5
  sharpened.) The irreducible declaration is "≥1 *categorical* anchor per kind" (oracle's kind +
  feature-detector); the probe grounds each instance *iconically* at run-time. No category ⇒ no
  detector ⇒ ⊤ ⇒ run.
- **f27 — the distributional escape is *principledly* unsound (counter-thesis, refuted for soundness).**
  Corpus co-occurrence kind-discovery (Firth 1957 "know a word by the company it keeps" / Harris 1954)
  is *symbol-to-symbol* — the Chinese/Chinese dictionary at scale, which Harnad shows never touches the
  world. So it yields *hints*, never *grounded* (sound) kinds. Confirms 094 g6 with a principle:
  **distributional = ungrounded by construction** → fine for ranking/seeding the oracle-bootstrap,
  never for elision-safety.
- **f28 — cross-oracle coherence (named, to deepen).** Type-class *coherence* (Wadler–Blott 1989):
  multiple instances of a class must yield consistent semantics. → multiple oracles grounding the same
  kind must be **coherent** (agree) or it is unsound. Sharpens 094 g4's "named-kind, not shared-token":
  the kind is the *class*, an oracle is an *instance*, and instances must cohere.

## The grounding picture (one line)
Tokens are *parasitic*; Dorc grounds them with **probe (iconic: observe the real host) + oracle
(categorical: declared kind + feature-detector)** — Harnad's hybrid — and the q-floor is the minimal
*categorical* declaration (≥1 anchor per kind). Corpus/distributional inference is the ungrounded
merry-go-round: hints, never sound grounding.

## Open / next
- Deepen if load-bearing (candidate follow-up tasks): Wadler–Blott *coherence* (the cross-oracle
  agreement rule, f28) and a distributional primary (Harris 1954).
- Carry to synthesis: **probe = grounding mechanism (f25)** · **q-floor = categorical minimum (f26)** ·
  **distributional = unsound-by-construction (f27)**.

## ⟢ Human adjudication (2026-06-02): the grounding lens is over-reach — Dorc is *relational*, not grounded
The human rejected this note's keystone. **The symbol-grounding/Harnad lens is the wrong frame for Dorc.**
A script "touches the world" trivially (execution does); that is not insight. And — the load-bearing point —
Dorc can be *categorically disconnected from reality* and that is *fine*: to enforce "it matters that foos
don't bar," we need not know what foos/bars *are* in the real world, **only keep the promise to the person who
does**. Corrections:
- **f25 RETRACTED (positive framing).** The probe is **not** "Dorc's grounding mechanism"; it merely
  *executes the oracle's promised check*. Dorc trusts the contract and stays **referent-agnostic**. The
  symbol-grounding problem is the *human's*, off Dorc's plate.
- **f23/f26 RE-CAST relationally.** The chicken-and-egg (094 g4) resolves not by *grounding* but by
  *relational contract*: the human declares relationships among (possibly meaningless) symbols ("A establishes
  F"; "foos don't bar"); Dorc **relays/propagates** them and never infers referents. The q-floor (094 g5 / f26)
  stands but is a **relational-contract minimum**, not a "categorical grounding" minimum — drop the
  iconic/categorical vocabulary.
- **f27 SURVIVES, stated relationally.** Corpus/distributional co-occurrence can't establish a *trusted
  contract*, only guess one → hints, never elision-safe. (Unchanged conclusion.)
- **Source re-grade (human-adjudicated):** the useful content here is already carried by *closer-to-domain
  first-party sources in hand* — the **type/provider RAL** (092 f17), **Burgess promise theory** (091 f12),
  **effect systems / occurrence-typing's relational narrowing** (092). [A-harnad-symbol-grounding-1990]
  relevance is downgraded to ~`-1:GUESS` (was `+1:SURE`); it **was** read, but it is **not** a keystone and
  not closer-domain. (Manifest is append-only; this is the authoritative re-grade record.)

**Net retained from F8:** the q-floor as a **relational-contract minimum** (human promises; Dorc relays,
referent-agnostic) + the distributional-unsound result (f27). The *grounding* theory is parked as not-ours.
