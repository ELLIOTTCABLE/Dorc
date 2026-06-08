# 194 — Type-discipline: ⊤-contagion must be *structural*, not emergent (the strain-8 lesson)

> Round-19 standalone note. The human asked (directive #1) to reason about whether `notes/193`
> strain-8 — the `EntityRef::Singleton` wrong-elision the adversarial-crosscheck found — could have
> been caught by stronger "make-illegal-states-unrepresentable" type-discipline, and to implement the
> result. This is that reasoning + the hardening it produced, kept separate from 193's keystone
> strain-log because it is a *cross-cutting discipline finding*, not a keystone strain. AI-authored,
> confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust root
> `DESIGN`/`KNOBS`/`IMPLEMENTATION`/`AGENTS` + `plans/191` over this.

## 1. The failure, precisely (from `193` strain-8)

`analysis::effect::command_effect` built a command's target cell from the post-verb words via
`words[2..].filter_map(word_literal)` → a `Vec<&str>` of *literal* operands, then read "empty ⇒
`EntityRef::Singleton`". `word_literal($PKG)` returns `None` ("not a static literal" — a ⊤ signal),
and `filter_map` **silently discards** it. So three structurally-different word-lists became
indistinguishable:
- `apt-get update` (zero operand words) → empty → `Singleton` ✓
- `apt-get update -y` (one flag) → empty → `Singleton` ✓
- `apt-get install $PKG` (one **⊤ word**) → empty → `Singleton` ✗ — must be ⊤/`Opaque`

The destroyed information is *"there was an operand word I could not resolve."* That is a ⊤, and the
operand is part of the **cell identity**, so a ⊤ operand means *unknown which cell* — eliding it elides
a mutation whose target is unknown (`kFAIL-perform` / never-under-execute violation; +SURE priority-1).
And a **regression**: the flat baseline `357efdd` returned `Opaque` here.

## 2. Could stronger types have caught it? (layered, honest — the human's question)

- **L0 — no, and don't overclaim.** A *logic error inside a classifier* is not type-excludable; no type
  forbids `_ => Singleton`. "Unrepresentable" is the wrong frame for a value-level decision.
- **L1 — yes, the real lever: the bug was *enabled* by a lossy reduction that broke a rule the codebase
  already states.** `spike/CLAUDE.md`: "enums for finite choices; no stringly-typed anything";
  `inv-top-reject`: ⊤ is *rejected loudly, never silently best-effort'd*. The `filter_map(word_literal)`
  did the forbidden thing — best-effort'd a ⊤ word into *absence*. A **total** classification (every
  word → a named variant, nothing droppable) makes ⊤ a case you must consciously handle. That is the
  realistic "exclude incorrect states" win here: not *unrepresentable*, but **un-writable on the easy
  path** (you'd have to explicitly map `⊤ → Singleton`, which reads as obviously unsound).
- **L2 — the invariant the bug actually violated (the durable bit; §3).** ⊤-contagion must be
  *structural*, not emergent.
- **L3 — it's the witness / `May`-`Must` pattern (note-165) one level down.** The codebase's answer to
  "a silent wrong-orientation" was: concentrate the catastrophic decision in one typed reviewable place
  (`ReplaceLicense::prove_replaceable`) and make orientation type-carried (`May`/`Must`). Entity
  resolution had neither — inlined and ⊤-leaky. strain-8 is evidence the discipline was applied to
  *verdicts* (`PhasedVerdict`/`Bias`) and *orientation* (`May`/`Must`) but **not** to *entity-extraction*.

## 3. The principle (forward-load-bearing): ⊤-contagion is structural

A `FactKey` is assembled from three ⊤-sources — **provider**, **verb**, **operand**. The invariant:

> Every path from a ⊤-source to a constructed `FactKey` must terminate in `Opaque`; a ⊤ may never
> reach a built cell.

+SURE the engine *already* enforced this at the provider and verb boundaries (each has an explicit
`else { return Opaque }` on a non-literal word). The **operand** boundary alone let it go *emergent* —
a silent `filter_map` drop instead of a return. strain-8 is exactly the one boundary where
`inv-top-reject` was not structural. The fix restores parity across all three ⊤-sources.

## 4. The hardening (committed, behavior-preserving)

`effect::resolve_entity(ast, post_verb, interner) -> Option<EntityRef>` (None = ⊤ ⇒ caller emits
`Opaque`). It is:
- **total** — every post-verb word matches exactly one arm: flag (literal `-…`) · literal operand · ⊤
  (non-literal). Nothing is silently dropped.
- **⊤-contagious** — a non-literal operand, or a second operand, `return None` immediately; a ⊤ can
  never fall through to the `Singleton` arm.
- **concentrated + unit-testable** — one place, isolable from the oracle-lookup plumbing (the existing
  `command_effect_resolves_operand_singleton_and_top` pins `update ⇒ Singleton`, `update -y ⇒
  Singleton`, `install $PKG ⇒ Opaque`, `install nginx ⇒ Operand`, `install nginx curl ⇒ Opaque`).
- mirrors the verb/provider guards; `command_effect`'s doc now just points at it.

Replaces the K1 inline band-aid (a cryptic `Option<Option<&str>>` match). ~SUSPECT the *cleanest*
long-term fix is still an oracle-*declared* cardinality/nullary bit (`dec-cardinality-deferred`,
`193` strain-1) so `Singleton`-ness is never *inferred* at all — that lands in R-strongweak (T5), and
this note is the argument that T5 should subsume the inference, not just sit beside it.

## 5. Standing recommendation — audit ⊤-sources as the entity-algebra grows

The richness phases *multiply* the ⊤-sources. Today a cell is `(kind, entity, selector)` = three
literal-derived fields. The recursive kind-typed algebra (`ch-entity-algebra`, R-recursion) adds
**kind-handle fields and nested structs** — each a new word/value that may be ⊤. -GUESS the highest-
value preventive discipline for those phases: **never reduce a ⊤-bearing field list with a dropping
combinator** (`filter_map`/`flatten` that discards `None`); classify totally and let ⊤ be contagious to
the constructed key. Consider promoting "⊤-contagion is structural" to a named `inv-*` (an
`inv-top-contagious`, a corollary of `inv-top-reject`) if a second instance shows up — one instance is
a fix, two is an invariant. (--WONDER whether a `Resolved<T>` newtype wrapper — a value provably free of
⊤ in every contributing position, the only thing a `FactKey` field accepts — is worth it once fields
nest; it is the `Grounded<T>`-style taint wrapper `16P T12` sketched, aimed at ⊤ rather than oracle-
trust. Deferred; flagged as the type-level escalation if hand-discipline proves leaky again.)
