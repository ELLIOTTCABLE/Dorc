# 093 — the impossibility dive: decidability ceilings & floors (round 9, 2026-06-02)

> Per the human's steer: hunt *proved-impossible* results (qualitative ceilings/floors),
> NOT big-O complexity — "the big-O of the analyzer alone is massively dominated by the
> twelve SSH-tunneled connections that follow" (AGENTS); the only complexity that bites
> crosses a network boundary. And mine them **query-planner-style**: planners live above
> undecidability (optimal join order NP-hard; perfect cardinality estimation impossible)
> yet ship best-effort-good plans from poorly-annotated SQL — Dorc's exact posture
> (`kVERIFY-calibrate`, not prove). Ceilings tell us *where the contract/probe must take
> over*; floors tell us *where we can be sound for free*.

## Findings (lifted)

- **f18 — Rice's theorem is the master ceiling AND names the escape hatch.** (Rice 1953,
  canonical.) *All non-trivial **semantic** properties of programs are undecidable; **syntactic**
  properties are decidable.* "Does `do_y` need `x`?" / "is this command already-satisfied?" are
  semantic → undecidable in general. **The escape is in the theorem:** push every question to the
  *syntactic/contracted* side. Dorc never decides behaviour; it (a) **recognizes contracted
  syntax** (does this match a known oracle-idiom — decidable), (b) **probes** (runtime
  observation, not a decision), (c) **delegates** the residue to the human contract. +SURE this
  is the cleanest single framing of the whole architecture: *Rice forbids the semantic question,
  so Dorc refuses to ask it.* [wall · the master ceiling → the three-way split]
- **f19 — precise footprint is undecidable (the precision ceiling).** [A-ramalingam-undecidability-aliasing-1994] (on Landi 1992): it is *impossible to compute statically precise alias information — may- or
  must-alias — with if/loops/dynamic-storage/recursion.* → "exactly which shared-state does this
  command touch" (precise frame/footprint) is undecidable. So Dorc **must over-approximate the
  footprint** (⊤-on-unknown) and lean on the **oracle's declared frame**; precise inference is not
  on the table even in principle. Formal backing for `kFAIL` (⊤ is the safe default) and for *why
  the contract exists at all*. [wall · justifies `kFAIL` + the declared frame]
- **f20 — the decidable floor: finite + distributive (IFDS).** [A-reps-horwitz-sagiv-ifds-popl-1995] (in-corpus): a large class of interprocedural dataflow problems is *precise and polynomial*
  provided **the fact-set is finite** and **transfer functions distribute** over meet. → The island
  in the undecidable sea: if Dorc keeps state-kind facts **finite** (bounded by the script's literal
  commands/paths — note 054 confirms this holds) and its effects **gen/kill-distributive**, it gets
  precise, poly-time, *decidable* analysis. The design constraint falls out: **stay finite +
  distributive**; the moment a fact-domain goes infinite or non-distributive you've stepped off the
  floor (→ ⊤). Pairs exactly against f19: precise *aliasing* is undecidable, but precise
  *finite-distributive dataflow* is not. [floor · the island to stay on]
- **f21 — the frame problem: non-effects can't be enumerated (the philosophical ceiling).**
  [B-sep-frame-problem-2004] (on McCarthy & Hayes 1969): you cannot write out everything an action
  *doesn't* change. → Dorc cannot enumerate a command's frame (what it leaves untouched); it must
  **assume-unchanged-unless-declared** (a *frame axiom* / closed-world assumption). That closed-world
  move is the unsound-but-necessary core of the whole skip-thesis — and the frame problem says it is
  *fundamental, not a Dorc shortcut*. The oracle declares the small **footprint** (what it touches);
  Dorc assumes the vast **frame** (everything else). [wall · roots footprint-declared + frame-assumed]
- **f22 — the query-planner posture (synthesis, per AGENTS).** Planners are the proof that living
  *above* these ceilings is normal engineering: NP-hard/undecidable in the limit, best-effort-excellent
  in practice, on poorly-annotated input. Dorc adopts the same: accept Rice/Ramalingam/frame-problem as
  *given*, stay on the IFDS floor, let the **probe** supply what static analysis can't decide and the
  **contract** supply what neither can — and *calibrate* (test), never *prove*. [confirms `kVERIFY`,
  and the DESIGN "RDBMS query-planner" prior-art lens]

## The ceiling/floor map (one picture)
| Ceiling (proved-impossible) | Dorc's forced response |
|---|---|
| **Rice** — semantic properties undecidable (f18) | refuse semantic Qs; recognize syntax + probe + delegate |
| **Ramalingam** — precise footprint undecidable (f19) | over-approximate frame (⊤); oracle declares footprint |
| **Frame problem** — non-effects unenumerable (f21) | closed-world frame axiom: assume-unchanged-unless-declared |
| **Floor — IFDS** finite+distributive = precise/poly/decidable (f20) | keep facts finite + gen/kill-distributive; stay on the island |

All four are *consistent with*, and *only navigable by*, the best-effort query-planner posture (f22):
the ceilings are why Dorc is contract-and-probe-shaped rather than analyzer-alone, and the floor is the
exact discipline (finite + distributive) that keeps the analyzer half sound and cheap.

## Citations
> Rice's theorem (canonical, Rice 1953; standard statement) (relevance: +1:SURE)
> "all non-trivial semantic properties of programs are undecidable. A semantic property is one about
> the program's behavior … unlike a syntactic property." → the master ceiling + the syntactic escape.

> [A-ramalingam-undecidability-aliasing-1994]:abstract (relevance: +1:SURE)
> "Landi [1992] recently established that it is impossible to compute statically precise alias
> information — either may-alias or must-alias — in languages with if statements, loops, dynamic
> storage, and recursive data structures."

> [A-reps-horwitz-sagiv-ifds-popl-1995]:abstract (relevance: +1:SURE)
> "a large class of interprocedural dataflow-analysis problems can be solved precisely in polynomial
> time … The only restrictions are that the set of dataflow facts must be a finite set, and that the
> dataflow functions must distribute over the confluence operator."

> [B-sep-frame-problem-2004]:§"The Frame Problem in Logic" (relevance: +1:SURE)
> "how is it possible to write formulae that describe the effects of actions without having to write a
> large number of accompanying formulae that describe the mundane, obvious non-effects of those
> actions?"

## Open thread
- f20's "finite + distributive" floor is a genuine **design constraint to surface at synthesis**: it is
  the formal version of `kCONTEXT`'s "keep the abstract domain flat" + the gen/kill effect-lattice
  (notes 052/055). Worth stating plainly as *the* soundness-floor discipline.

## seq-1 confirmation — ceilings read-from-source (2026-06-02)
- **f19 sharpened (Ramalingam, full-read):** confirmed verbatim, and *stronger* than first cited —
  **even the *intraprocedural* may/must-alias problem is undecidable** (reduction from Post's
  Correspondence Problem; root cause: "deciding if an arbitrary path in a program is executable is
  undecidable"). So the precision ceiling is *intrinsic*, not an interprocedural artifact — staying
  intraprocedural buys no precise footprint. Strengthens the f19→`kFAIL`/declared-frame argument.
- **f21 confirmed (SEP frame-problem):** the "non-effects" statement verified against the archived
  text; source correctly re-graded to `B-sep-frame-problem-2004` (an encyclopedia = secondary).
