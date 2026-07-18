# 16J — superposition: collapse at the caller, not the engine (the def-4 spec)

> **Status (2026-06-05): spike, round-16 — design note + spec for the rewrite it
> describes (built in the same/following rounds).** Triggered by the human catching
> `def-1` (a phase-collapse baked into the gate) and naming "the May/Meet lift
> again." Re-grounded in the PRIMARY docs (DESIGN/KNOBS) after a compaction had me
> running on the `16x` notes + an apply→probe analogy. Append-only (round 16: …16I →
> 16J). Confidence-marked; the human asked that nothing here be overstated as
> for-sure. **This is the design-of-record for the def-4 rewrite — read it first.**

## 0. Method correction (why this note exists)
A compaction dropped the human-authored `DESIGN.md`/`KNOBS.md` from my context; I'd
been reasoning from the `16x` notes (AGENTS calls these "unreviewed LLM-generated
planning-slop", trust LESS than the human docs) and an apply→probe analogy. The
"open honest question" of the prior rounds — *what does the probe phase do with the
observable-consumption fact?* — is answered outright by the primary docs (§5).
Not-knowing it was a **sourcing failure, not a real gap**. Standing lesson: re-read
`DESIGN.md`/`KNOBS.md` in full after any compaction. (Corollary discovered the same
way: `spike/CLAUDE.md` and its `inv-*` slugs are **AI-authored**, a working
registry — not first-party ground truth. The first-party docs are
`README`/`DESIGN`/`KNOBS`/`TODO`.)

## 1. The principle, from the primary docs (not analogy)
- DESIGN §"Dorc's approach": the **probe** phase **under-approximates** ("better to
  not ship a probe at all … than … something that may be mutative", :79); the
  **apply** phase **over-approximates** (:83); and they are **"Same analysis,
  different fail-safe posture"** (:83).
- KNOBS `kFAIL` (welded, phase-keyed): probe⇒`kFAIL-withhold`, apply⇒`kFAIL-perform`
  — opposite safe directions, "the one thing never traded for performance."
- KNOBS `kPRECISION`: cutting precision "costs probes/runs, never correctness, while
  `kFAIL` holds" ⇒ an over-conservative consumption analysis is **sound**.
- DESIGN :215: minimality/elision is optional (always free to fall back to running)
  ⇒ **under-elision is always safe.**

Read together: "is this leaf's output consumed?" is **one** analysis; the **phase
chooses the posture** (under vs over). The engine must therefore NOT bake a posture
— it emits the un-collapsed fact, and the phased caller collapses it.

Proposed name (human-ratified the name): **`inv-superposition`** — *the analyzer
kernel emits phase-/orientation-agnostic lattice facts; only the phased caller
collapses, by arguing the phase.* It is note-165-L1 ("the merge picked by the type")
**generalized** from the verdict to every phase-sensitive fact. ~SUSPECT this is the
right general principle; +SURE it is what DESIGN :83 says for *this* fact.
(Recording in `spike/CLAUDE.md`'s invariant list is proposed to the human, not done
autonomously — that file is special even if AI-authored.)

## 2. Prior art already in the tree (use it; do not reinvent)
- `analysis/lattice.rs`: `May<L>` (identity wrapper, over-approx, ⊥-start/⊔-merge)
  and `Must<L>` (the order-dual, under-approx). Its doc: *"one engine, both
  orientations, the merge picked by the type … kills the union-where-you-needed-
  intersection bug."* The `solve` engine runs ONE direction; the **wrapper type**
  picks may/must. That IS engine-superposition + caller-collapse, for orientation.
- `plan/lib.rs`: `PhasedVerdict<P: Bias>` + `Bias::on_unknown` — the **verdict**
  already got the lift (phase in the type; `Unknown`-fold per phase; one impl-site).
- The gap: the **observable gate** never got it. `ObservableUse::forbids_stub` is a
  phase-blind `bool`; `prove_replaceable` is hard-wired `<Probe>` (:212 — currently
  benign ONLY because both `Bias` impls return `Run` on `Unknown`; a latent collapse
  the moment they diverge). `def-1` (default-MustRun) would have baked MORE collapse
  in — **withdrawn**.

## 3. What def-4 builds (the spec)
Relocate output-consumption from the hand-rolled `plan` walk INTO the engine, as an
un-collapsed fact, collapsed at `plan` by `(phase, orientation)`.

- **Engine (`analysis`):** consumption as a lattice fact per leaf —
  `MapL<AstId, Powerset<Observable>>` (the observables that MAY reach a consumer).
  `Powerset` does **not** `impl BoundedLattice` (`lattice.rs`), so `Must<Powerset<_>>`
  does not type-check — a must-read of consumption is a **compile error**, not a
  wrong default. Emit it un-collapsed alongside `classify`, phase-agnostic. Only
  `Stdout`/`Stderr` populate it today (effect/status are vouched elsewhere), but key
  it by `Observable` so the vouching model stays nameable in the type.
  - Form (~SUSPECT, decide when building): a **backward** pass over the CFG via the
    direction-generic `solve`, so the *fixpoint* owns path-completeness instead of a
    hand traversal (the whole point — the 16I kill-shot was a hand traversal missing
    a scope). The conservative structural surrogate (pipe-position + non-`/dev/null`
    output redir, **enclosing-context-aware**) is the transfer floor; true file/var
    liveness is an OPTIONAL precision add (`kPRECISION` — defer if it's a lift).
    -GUESS the minimal-correct version may be "attach during CFG lowering" (`def-3`,
    single-source-of-truth) rather than a separate fixpoint, since consumption is
    largely structural. Either way it lives in the engine and is emitted un-collapsed.
- **Caller (`plan`):** `prove_replaceable<P: Bias>(…, consumed: May<Powerset<Observable>>)`
  — generic over the phase (no hard `<Probe>`); consumption arrives as a `May<_>`
  that, per `inv-must-may` (`lattice.rs:244`), can only **block** a license, never
  authorize one. The license still needs its positive grounds (ambient + `Must`-grade
  + `Converged`). The phase-direction of "consumed ⇒ ?" moves into `Bias` (one method
  per phase), so wiring the probe phase later **forces** a conscious per-phase answer
  — it cannot inherit apply's, cannot be forgotten (the spotlight/exclude win).

### Constraints (from KNOBS — do not violate while building)
- `kCONTEXT` (redline, high lock-in): keep the pass **context-insensitive** (no k-CFA).
- `kFACTS` (high lock-in, **UNSETTLED**): do NOT weld materialized-vs-demand. Reuse
  `solve` + the lattice combinators; don't smuggle in a Datalog layer or a demand
  cache as a side effect of this rewrite.
- `kPRECISION`: the conservative structural floor is sound; any precision add must
  only ever **shrink runs, never licenses** (stay in the `kFAIL` direction).
- `kFIDELITY` (high lock-in): the leaf-seam stays wrappable + provenance-preserving
  (`LeafId→AstId`); the consumption fact keys by that leaf identity.
- `inv-determinism`: ordered collections only (`MapL`/`Powerset` are `BTree`-backed).

## 4. Prereqs folded into the rewrite
- **`def-2`** — kill the `_ => {}` catch-all: the floor-computing traversal must be
  **exhaustive over `NodeKind`** so a future construct is a compile error, not a
  silent under-approximation (the 16I kill-shot's root cause-class).
- **`def-3`** — single source of structural truth: compute the floor from the SAME
  structure `classify` uses (the CFG), not a parallel AST scan (that parallel scan
  WAS 16I bug-c).
- **`def-5`** (human-confirmed hard-yes): a DST/property test pinning the **domain
  invariant** — every leaf `classify` can mark `EstablishAmbient` is in the
  consumption fact's domain (no leaf invisible to the gate). Detection for the
  residue types can't reach (a mis-threaded transfer).

## 5. The probe-phase answer (the "open question", from primary docs — marked)
- +SURE (DESIGN :79/:83 + `kFAIL`): probe under-approximates and folds doubt to
  **withhold** — don't ship a probe ⇒ fact `Unknown` ⇒ apply runs it — NOT to "run
  during probe" (that would violate `kFAIL-withhold`). Apply over-approximates and
  folds doubt to **run**. Same consumption fact, opposite postures.
- -GUESS (genuinely unbuilt): whether the probe *projection* reads THIS exact fact
  or a sibling mutation-containment fact, and its precise mechanism — the probe
  projection is not built. `inv-superposition`'s payoff is that it **forces** this to
  be answered per-phase when the probe phase lands, instead of silently inheriting
  apply's posture. (Which is exactly why `def-1`'s single hard default was dangerous:
  it would have answered an unbuilt phase's open question by analogy.)

**NOTES INDEX:** …16H gate landed · 16I core attack (enclosing gate + ⊤-containment)
· 16J (this — superposition / collapse-at-caller; the def-4 spec).
