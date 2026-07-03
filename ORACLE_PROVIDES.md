ORACLE_PROVIDES: the ledger of what an oracle hands Dorc
=========================================================

> Document tier: AI-written, heavily user-audited (the USER_STORY.md / ANALYZER-NEEDS.md
> class — every word human-reviewed before it counts; trumps the Research/ planning-ocean;
> changes rarely). Minted 2026-07-03 (round 24) from a design dialogue; pending its first
> full human audit.
>
> **What this is:** a living ledger of the distinct *shapes of information* an oracle-author
> can hand Dorc — the moving parts slowly accrued across rounds that will eventually be
> firmed into "the oracle contract." It is NOT the contract. The eventual contract is a
> composite construct implemented across a disparate, non-uniform union: documentation ∪
> static checks ∪ promises Dorc pushes out to *admins* ∪ best-effort hints and lints. This
> ledger exists so the assumptions feeding that composite don't get lost in the churn.
>
> **The standing obligation every entry owes (unpaid; tracked here so it stays owed):**
> each shape, individually, eventually needs a fully-monotonic gradual-degradation story of
> the blessed form — *every partial-effort-increase yields vaguely-corresponding value; no
> partial-effort-increase removes previously-won value.* Where an entry sketches one below,
> it is a sketch, not a settled story.
>
> All sh in this document is inline idiomatic illustration, not settled spelling; each
> entry carries its own spelling status.

The frame: a book line Dorc cannot read (`foobar sync-certs "$CERTS"` — token soup) plus an
author who knows the tool. Everything below is something that author can *choose* to hand
over, roughly in dependency order — later shapes lean on earlier ones. Two shapes at the
end are categorically unlike the rest: one is an *answer*, one is a *liability
speech-act*, and keeping them distinct is this ledger's founding reason.

provides-decoding — "here is how to read my command's invocations"
------------------------------------------------------------------
The argparse: flag-stripping, operand extraction, dispatch — ordinary authored control-flow
(`verb="$1"; shift; while [ "${1#-}" != "$1" ]; do shift; done`) that Dorc constant-props a
site's argv through. Not a claim about the world; a decoder ring for the command's
*surface*. It defines **reached-path scoping** for every other shape: the engine's one
scoping concept is "which span does this site's resolved argv reach" (never token-matching,
never a "verb" concept — verbs are author-plane idiom; inv-referent-agnostic).

- Consumed by: entity-resolution; per-site keying; the path-scoping of every later shape.
- Trust-shape: near self-defeating when wrong — an argparse that resolves nothing yields ⊤
  and the site runs; gate-5 cross-checks resolved argv against dash ground truth.
- Spelling: settled idiom (the lifted function bodies' own control-flow; no dedicated
  syntax).
- Degradation sketch: no decoder ⇒ opaque command ⇒ wall + run (the floor). A partial
  decoder buys value on exactly the decoded paths. Monotone.

provides-vocabulary — "these are the named pieces of world-state my tool deals in"
----------------------------------------------------------------------------------
Kinds and entities: the author mints (or reuses) a named kind and binds operands as
entities in it — `dest : fb.Certs = "$1"` — yielding the coordinate vocabulary
(`kind:entity.prop` cells) everything state-shaped is expressed in. Nobody approves kind
names; there is no registry; a kind only has to agree with itself (the round-17
symbol-grounding settlement: identity is *declared*, never inferred).

This shape went conversationally invisible during the round-23/24 verdict-and-license churn
— it rides silently inside every bind and mark — which is part of why this ledger exists.
It is also the design's one acknowledged loudly-non-sh construct (the typed-sh surface):
its final concrete syllable remains gated behind the reserved dq-kOOB / kTYANNOT decision
(inline annotation ↔ eol-comment; the off-ramp trade).

- Consumed by: the fact-plane's cells; cross-oracle collaboration (shared kinds; the
  contribution model); wall/footprint/backing coordinates; probe-time dangling-reference
  detection.
- Trust-shape: identity claims. The failure mode is within-kind incoherence — one real
  referent under two names (the synonym cell) — the design's residual naked spot, owned by
  the kind's one author-of-identity; not directly a skip-liability.
- Spelling: strawman-frozen pending dq-kOOB.
- Degradation sketch: no kinds ⇒ no cross-site reasoning, everything still runs safely;
  per-kind adoption buys per-kind value. Monotone.

provides-reading — "here is how to measure that state, read-only" (+ the inertness vouch)
------------------------------------------------------------------------------------------
Executable measurement recipes: `dpkg-query -W "$pkg"`, `foobar status --certs-current --
"$dest"` — arbitrary sh, shipped strip-only into the probe lane and (as the guard tier
lands) into apply-lane guard bodies, byte-identical.

Hidden inside this shape is a liability taken *implicitly by authorship*, older than any
license question: writing sh in an oracle body at all **vouches its probe-position
inertness** (the structural self-vouch — a command inside its own oracle's body). This
guards Dorc's hardest promise: *the plan phase mutates nothing*. Per IMPLEMENTATION.md,
this promise has no gradient — a mutating probe is full contract-collapse, not degraded
service.

- Consumed by: the probe lane (per-site, stripped); guard bodies (same bytes).
- Trust-shape: the single sharpest non-license liability an author carries; enforcement is
  structural (self-vouch scope) + harness (mocks-only exec; the rc-127 vouch-closure
  check), never inference ("no analysis-confidence threshold ever makes a probe safe").
- Spelling: settled (arbitrary sh; strip-only; bare annotation-marks delete whole per the
  strip-fidelity ruling).
- Degradation sketch: no recipe ⇒ can't-probe ⇒ can't-elide (value floor, safety intact);
  cheap-but-shallow recipes buy shallow verdicts. Monotone.

provides-binding — "this measurement MEANS that cell"
------------------------------------------------------
The semantic bridge from recipes to vocabulary: trailing establish-marks
(`probe-cmd : fb.Certs:"$dest".synced` — this exit status establishes that cell), the `!`
exit-code-inversion plumbing, the `:?` observe/depends-upon mark, and the surviving bare
per-cell marks (ACK, POISON). The engine never interprets what "synced" *means* — opaque
values, no polarity; the binding just names whose probe carries which cell.

- Consumed by: the fact-plane (ambience, cell-convergence, walls' same-cell reasoning);
  display; **backing** (a fact's backing = the union of its probe's *marked* reads — never
  a claim about unmarked syscalls).
- Trust-shape: descriptive. Wrong bindings mislead reasoning; *unmarked* side-reads are
  invisible to backing — the coordinate-plane honesty obligation, riding
  provides-vocabulary's coherence.
- Spelling: settled-strawman (the mark grammar; strip-fidelity applies; the two-level
  tilde vouch-mark is dead — see provides-license).
- Degradation sketch: unmarked ⇒ unmodeled ⇒ conservative ⊤-ish handling; each mark buys
  its cell. Monotone.

provides-behavior — "here is what running it would DO"
-------------------------------------------------------
Three descriptive sub-shapes about the command's own action, all fact-tier, none of which
licenses skipping a mutation (the proviso-read-erasure weld: predicted values may back
value-reproduction of *reads* and branch-fold decisions; only the license-shape may skip a
*write*):

1. **Predicted observables** — the rc a site would exit with (aggregate status of the
   describing function; someday declared stdout/stderr values). Feeds branch-folds
   (`&&`/`||`, errexit regions), the value-substitution of read-only guards, status
   relaxation.
2. **At-least effects** — the establish-marks on mutator paths: "install establishes
   `package:X#installed`." The gen-set; feeds same-cell kills and the honest wall.
3. **At-most footprint** — the write-set bound: "this invocation touches at most these
   entities" (the `touches()` strawman: emit entity-coordinates on reached paths;
   emission = the claim; no emission = no claim = wall). Feeds survival/disjointness past
   running walls — MODE-GATED behind the admin's explicit flag (rul24-mode-gate), because
   it is the sharpest descriptive claim in the design: a wrong footprint silently
   under-executes *someone else's* line (rul24-divergence-is-the-game — the license-site /
   elision-site divergence plus the claim-subject / blast-subject divergence).

- Trust-shape: graded within the family — predictions are substantially self-checking
  (parity gates); at-least is plain description; at-most is the traveled completeness-claim
  (opt-in, attributed, horizon-priced).
- Spelling: predict() settled (rul-role-split); touches() ratified strawman (24A §1b);
  footprint = write-set only (reads don't kill facts; the reads-spelling already exists as
  the `:?`/establish marks).
- Degradation sketch: each sub-shape independently optional; footprint-less oracles keep
  every pre-footprint behaviour. Monotone by construction (rul24-threefunc-monotonic).

provides-convergence — "for this invocation, taken whole: is the desired state true?"
--------------------------------------------------------------------------------------
The authored *aggregation* of reading+binding into one per-invocation answer:
yes / no / can't-say, spelled as the verdict-function family's fixed exit-status partition
(0 = the named sense holds; 1 = its complement; ≥2 = can't-say, always runs). Declining
per-path is ordinary control-flow (`*) return 2 ;;` or an unhandled path), never an
annotation.

**This shape is an ANSWER, not a permission — and it is not subsumed by the license.** Its
license-free consumers are real product: the plan as a read-only *drift report* ("show me
what's diverged," consumed at 3am before deciding anything); plan-shape and display
annotations; the hint machinery's coverage counts; can't-say routing. An author who
provides everything through this shape and *nothing further* has built something genuinely
useful that never skips a single command on their word.

- Consumed by (license-free): drift-reporting, display, hints. Consumed by (license-gated):
  guard bodies, elide decisions — see provides-license.
- Trust-shape: an answer whose *semantics* carry the adequacy question ("does my yes mean
  re-running is noise, or merely that some state holds?" — the converged≠no-op gap) — but
  *acting* on the answer at a distance is the next shape's business, not this one's.
- Spelling: the verdict-function family (sense-by-name; rc-partition welded).
- Degradation sketch: absent ⇒ no verdicts (facts may still display); per-path answers buy
  per-path verdicts. Monotone.

provides-license — "I accept the blame for acting on my answer" (THE LADDER — open)
------------------------------------------------------------------------------------
The ledger's odd one out: the only shape with **zero measurement content**. It says nothing
about the world. It is a human speech-act about liability: *"hi, I'm a human, I declare
throw-away-some-residue, come blame me when that's bad. okay bye."*

Laid against provides-convergence, it is a **ladder of rungs over one answer**, not a bit:

- **rung-0 — none.** My answer is for display and drift-reporting only. Never skip
  anything on my word.
- **rung-1 — in-position.** Skip on my answer *where it just ran*: the guard form
  (`check || original-bytes`), visible in the consented plan, co-signed by the admin who
  read it. Carries the adequacy exposure; carries no staleness exposure (frame-free by
  construction) and no invisible-line exposure.
- **rung-2 — carried.** Remove the line from the plan on my answer's strength — across
  time, and (behind the admin's flag) across other commands' runs. Invisible to the admin;
  the full causor-weight of a wrong skip lands on me; the converged≠no-op residue is
  accepted *here*, by name.

Future rungs graft onto the same ladder rather than becoming new shapes: the
"literally-a-no-op, fully state-precise" precision claim (the stronger cousin of rung-2's
judgment; 24A §1c proviso-family-open), the deferred `.diff`-style earned verbs.

**The prohibitive polarity — vetos (PARKED, someday-maybe; `notes/23M`).** The same
speech-act category run backwards: an owner-spelled, judgment-tier, veto-only list that
*withdraws* license from elisions the machinery would otherwise grant — aimed at a named
residue class (canonically the cross-kind escape: a footprint's professed hole). Inverted
failure-direction makes it the safe twin: a wrong or stale veto only over-verifies (it can
fail to prevent, never newly cause, a wrong elision; staleness degrades to the binary
floor). Not the rung-0/1 hatch restated: rungs withhold permission for acting on *my own
answer*; the veto protects *others' lines* from *my footprint's* residue. Parked verdict
(human, 2026-07-03): moves neither correctness nor value needle alone; tolerable only with
aggressive attribution + an admin off-switch — both of which Stage 2 is building for
independent reasons (attribution co-primary; the mode-gate flag), so the marginal price is
dropping. The unpark-bar stays: the Stage-5 expansion-bridge is the preferred non-blunt
alternative for the same hole; the veto earns life only on empirical evidence that bridges
don't suffice.

- Trust-shape: pure liability. Enforcement is the attribution machinery (the why-lens
  naming the licensor per elision/guard) — and, on the *admin's* side of the same
  coin, the mode-gate flag (rul24-mode-gate: the opt-in that is marketing at best, theatre
  at worst, and demanded anyway).
- Spelling: **OPEN — under active design (2026-07-03).** The current ruling
  (rul24-vouch-is-verdict-authoring, 24A §1c) welds authoring provides-convergence to
  rungs 1+2 in one act; this ledger's taxonomy pressures an un-weld (the wary engineer who
  wants to hand over answers while standing on rung-0 or rung-1 — the engineer's escape
  hatch, sibling to the admin's flag). Note that rung-selection is the least sh-native
  thing in the entire design — POSIX has no idiom for "blame me" — making it the one place
  that may *earn* a loud first-class construct under the native-or-break-loudly law.
  Function names, if names end up carrying rungs, fall out of this ledger later; the rungs
  come first.
- Degradation sketch (the ladder IS the story): each rung up trades author-liability for
  admin-value; declining a rung never removes lower-rung value. Monotone if and only if
  rung-selection stays independent per-path and per-shape — a constraint on whatever
  spelling lands.

provides-margins — abstentions, breadcrumbs, and the reserved seats
--------------------------------------------------------------------
- **Scoped refusal with attribution** (live): the `UNK` out-of-band report idiom — "I
  decline to answer here, and here is why" — feeding hints and plan-reasons. OOB is legal
  for facts, diagnostics, refusals; never for verdicts or licenses (rul-role-split).
- **Reserved future entries** (each will need its own ledger row when unparked):
  *cost-hints* (check-cost banding; parked pending a sanctioned data source);
  *grounding-bridges* (Stage-5: footprint-expansion across kinds vs co-reference across
  namespaces — vocabulary-relations, owner-spelled, provides-vocabulary's relatives);
  *version/context-pins* (the MH2 seed: "the binary you're eliding around is the binary I
  described" — the elision-site's mechanical check of a traveled claim's context, the
  missing tether under rul24-divergence-is-the-game).

----

How this ledger evolves: entries are appended or refined as rounds settle them — never
silently dropped; a shape that dies gets a dated tombstone with its reason. Every entry
eventually owes three payments before "the oracle contract" can be assembled from it:
(1) a settled spelling; (2) an enforcement home — which of {docs, static check/lint,
promise-to-admins, hint} carries it; (3) the monotonic gradual-degradation story in the
blessed form. As of minting, no entry has paid all three.
