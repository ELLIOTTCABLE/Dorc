# 16D — the meta-obligation: type "best-effort stays provably-best as the oracle degrades"

> **Status (2026-06-05): spike, CORE LENS (human-raised). Note-and-defer — the full
> solution is too hard to target now; we will return to it.** This is not a feature;
> it is a *stance* that must angle into every synthesis from here. Append-only
> (round 16: …16C → 16D). Confidence-marked.

## 0. The obligation, stated
We've been lowering **core-correctness** claims into the type system (May/Must,
`Verdict<Phase>`+`Bias`, the `SkipLicense` witness, `BoundedLattice` etc., notes 165
/ this round). Good. But there is a **meta-obligation along that same gradient**:

> As the **oracle's** correctness degrades, **our best-effort must stay
> provably-best** — and *that* property, not just core-correctness, is what belongs
> in the types.

The footgun this guards against (human's words, kept verbatim because they are the
whole point): *"This code looks like correctness code! that means we can depend on
provable correctness of things!!"* — **no.** We are writing correctness code
**specifically to be defensive** against how extremely *unprovable and unreliable*
our foundation (the oracle, the host, the sh) is. The heavy types do **not** prove
ground truth; they prove a **conditional** ("*if* the oracle's unverifiable claims
hold, the skip is safe") **and** a **floor** ("*when* they don't, we fail the
conservative way"). An implementation-agent — or a tired human — that reads the
witnesses as proofs-of-reality will over-trust exactly where the design is weakest.

## 1. Why this is THE concern, not a footnote
Dorc's soundness is **capped** by construction (DESIGN; 162 O-2 / note 167 DP-2/DP-9):
the oracle-grounding boundary is unverifiable — a frame-clean oracle can ship a
wrong fact-probe, a mutating "probe", a lying verdict, a garbled lift. The kernel's
internal logic can be type-proven sound (the adversarial pair confirmed the
reachable-flow gen/kill); but every *useful* conclusion rides an oracle claim we
cannot check. So the valuable, defensible thing the type system can do is **not**
"prove the skip is correct" (impossible) — it is:
1. make the **oracle-trust dependency visible in the type** (you cannot mistake an
   oracle-conditional conclusion for an analyzer-proven one — that conflation is the
   footgun, and it should be a *type error*); and
2. guarantee **every degradation folds conservatively** (the failure direction is
   always the phase-safe one, kFAIL); and
3. **bound the blast radius** (one lying oracle corrupts only its own skip, never the
   analyzer's integrity or an unrelated leaf).

## 2. We already have FRAGMENTS — the obligation is to name + systematize them
Several existing locks are *instances* of best-effort-under-degradation, built
ad-hoc per-site rather than as one stance:
- **`inv-top-reject`** — under-modeling ⇒ ⊤ ⇒ run (degradation: *we* don't model it ⇒ conservative).
- **`Bias`/kFAIL fold** — `Unknown` ⇒ conservative per phase (degradation: can't-tell ⇒ conservative).
- **must-may one-way coercion** (`May ↛ Must`, note 165 L1) — this is the *purest*
  existing instance: as belief quality degrades `Must → May`, the type **forbids**
  using it to license elision. That is exactly "best-effort stays provably-best as
  the input degrades," already typed. The obligation is to make *everything*
  oracle-derived behave like this, not just the grade.
- **`Carrier`/dn-7** — malformed input ⇒ data + diagnostics, never a crash.
- **oracle-lift** — garbled oracle file ⇒ diagnostic + no decl ⇒ the consumer sees
  ⊤ ⇒ run (degradation: bad oracle ⇒ conservative).

What's missing is the **uniform, typed** guarantee across the whole oracle-derived
surface, and the **visibility** that prevents the conflation footgun.

## 3. The degradation gradient (what "provably-best" must mean at each rung)
Oracle quality, roughly worst-helped to worst-harmful, and the floor the types
should make legible at each:
- *absent oracle* → ⊤ → run. (Have it.)
- *garbled lift / non-literal anchor* → diagnostic, no decl → run. (Have it.)
- *under-precise* (⊤-on-unknown-flag, 162 O-3) → fewer skips, never wrong. (Have it.)
- *wrong fact-probe* (says converged when diverged) → **a wrong skip.** Type-uncatchable
  (the unverifiable boundary). Floor must be: re-probe-before-apply (165 L4) + DST
  differential backstop + the lie is contained to *that* fact's skip.
- *mutating "probe"* (kFAIL-withhold breach) → not frame-enforceable (162 O-2); floor
  = hostsim detection now / sandbox later, and the type must not *pretend* it's proven.

The hard truth (why deferred): the worst rungs (confident lie, mutating probe) are
**type-uncatchable**. So "provably-best-effort" can never mean "provably-correct
despite lies." It means: **(a)** the trust-boundary is type-*visible* (no conflation),
**(b)** the failure-direction is provably conservative, **(c)** the blast-radius is
bounded, and **(d)** the unprovable rungs are *loudly* delegated to runtime backstops
(DST/re-probe/sandbox), never silently absorbed.

## 4. Candidate shape (sketch only — DEFERRED, ~SUSPECT)
Make oracle-derived values a **distinct type** from analyzer-proven ones, so they
cannot be silently conflated and the footgun becomes a compile error:
- a wrapper like `Grounded<T>` (or `OracleConditional<T>`) on *every* value that
  depends on an oracle claim — carrying *which* claim it's conditional on (its
  `Derivation`), analogous to how `May`/`Must` carries orientation;
- the only ways to *discharge* `Grounded<T>` into an irreversible action are the
  kFAIL-conservative gate (fold to run) or a *witnessed backstop* (re-probe /
  DST-confirmed) — never a bare unwrap;
- composition rule: combining a `Grounded` value with anything keeps it `Grounded`
  (taint-style), so the conditionality can't be laundered away mid-pipeline.
This is the `May/Must` discipline generalized from "belief grade" to "trust
provenance." It is real machinery (a meet-like obligation, possibly engine-wide) and
collides with ergonomics — exactly the calibrate-UP tension (165 §3), now one level
up. **Do not build yet** (human: "we'll return to it"); prototype it against one
concrete path (the skip-license) before generalizing.

## 5. The directive going forward (the lens)
Every synthesis from here — the skip-and-substitute bridge (16C), the apply executor
and the backward slice (16A §1), multi-host (16A §2) — must, as a first-class design
question, answer: *where does oracle-trust enter, is that entry type-visible, and does
every degradation of it fold conservatively with a bounded blast radius?* If a design
makes the analyzer "look more certain than the oracle warrants," it has the footgun
and must be re-cut. Treat this note as the standing review-criterion.

**NOTES INDEX:** …16A apply+multi-host · 16B leaf-seam · 16C skip-and-substitute ·
16D (this — typed best-effort-under-degradation, the core lens).
