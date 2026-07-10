# 240 — Round-24 charter: "head off 233 by building something and seeing what happens"

**The prospective, prescriptive plan for round 24 — an EMPIRICAL build round.** Companion to the
historical record `notes/23O` (read that first for *why* we are here). Plans-tier: keep scanned
for currency; annotate, don't rewrite, when superseded.

> **⚠ ROUND CLOSED (2026-07-10, by reshuffle — full accounting `notes/24U`):** ladder Stages
> 1–5 LANDED (evidence `24C`); Stage 6 SPLIT — the extract/conclude half is DONE (`24O`), the
> measure/maximize half moved to `270:block-stdlib` as `270:yardstick-measurement`. The
> charter's sharpest deliverable (the converged≠no-op adequacy bite-rate) was NOT answered
> in-round — it re-homed to the field-trial differential (`252` A1), itself tabled; `24U` §2
> carries the honesty note. Successor arc: `plans/270`.

---

## Why this round is a build, not a design round

Round 23 established (fully, in `notes/23O`) three things that *converge on building*:
1. **The elide-half's core problem is permanent.** 233 is the frame problem; no further theory
   makes past-wall elision sound. The answer is "static proof or disclosed residue," final.
2. **The prior-art pass says so.** `23N` §10: "the work is design-and-adoption, not literature;
   do not re-open the search."
3. **The one live risk is empirical-only.** The irreducible danger is the converged-vouch's
   *adequacy* (converged≠no-op) — calibrated-never-proven, measurable *only* by running against
   real oracles and books.

The enabling move: the elide-half is **fail-dangerous** (a wrong elision is a silent
under-execution, the cardinal sin), so it needs a correctness *net* to build empirically. **In the
spike, the DST exec-differential is that net** — run the elided plan under inert mocks, diff the
end-state against the bare book; a wrong elision changes the state and goes red, deterministically.
The *guard* is the *production* net (production has no bare-book to diff against); it is
deferrable. Therefore the elide-half — the golden hill, the thing Dorc was set out to build — can
be built **first**, safely, in the spike, without waiting on a design it can only learn by
building.

---

## The yardstick — because a measurement requires one

The north-star metric, CLI-executable at every stage and the spine of this whole round:

> **elision frequency on a family of hand-authored strawman books** — a book plus oracles of
> varying quality — with correctness enforced by the differential.
>
> `dorc plan strawman.sh -o *.oracle.sh` → *(count of elided lines; differential-verified correct)*

Every stage must move this number, **visibly, from the CLI.** Building the yardstick is Stage 1.
Real-corpus measurement stays behind the standing quarantine; strawman measurement is the
sanctioned instrument, and it is sufficient to learn whether the mechanism *works* (it does not
answer "does it work on *real* ops sh" — that is a later, quarantine-gated step).

---

## Goals / success-state

The round succeeds when:
- the spike **elides past a running command** on strawmen, at maximum frequency given
  limited-correctness input, with **every elision differential-verified**;
- the empirical questions are answered *with evidence*: does the frame-rule mechanism deliver
  elisions? **how often does the converged≠no-op adequacy risk actually bite** (the round's most
  valuable output — it is the elide-half's whole residual danger)? are footprints and grounding
  *authorable by hand* without misery?
- those conclusions are extracted into the human-authored docs — the precondition for **killing
  the spike**, which is the real end-state this round moves toward.

---

## The ladder — six stages, each raising the yardstick, each teaching only the next

The golden hill lights up at **Stage 2**, not the end. The guard is a Stage-3 production-net
side-quest, off the critical path.

**Stage 1 — the yardstick + an honest baseline.** Build the measurement mode (elision count +
differential-verified correctness). *And* land the one repair that makes the baseline honest —
the spike still carries the "dangerous middle" (a partially-modeled command under-poisons; the
`fd10` hole from the repair pass), so silence does not yet mean wall. Fix it, and the baseline
reads: *pre-wall converged lines elide; everything past a wall runs; post-wall elisions = 0.* Now
there is a number to beat and an instrument to beat it with. *Teaches: exactly where zero is.*

**Stage 2 — the frame-rule machine: the first line vanishes past a wall.** Authored footprint (an
oracle declares "I touch entity X") + backing (the probe's read-set, self-framed) + the
disjointness test (footprint ∩ backing = ∅) + `elide-when-disjoint-else-run` — **no guard, `run`
is the fallback; the differential is the net.** The yardstick goes 0 → N; a line disappears from
the plan past `hork`, and the differential proves it safe. *Teaches, and only teaches: does
disjointness fire, are footprints authorable by hand, do the elisions stay correct — the three
things you must know before it is worth deriving anything.*

**Stage 3 — the guard tier (the production net).** Build the ternary guard the 9 `guard23-*`
XFAIL cases already pin (emitter, GuardLicense witness, gate-6 widening) — the fail-safe
production form of past-wall reasoning, and the shippability layer the elide machine will
eventually need where no differential exists. Slotted here (after the first elision, before
derivation) per the human's ordering. *Teaches: the converged-vouch's calibration under the safe
form; the render.* Off the critical path to the golden hill, but it is where the two halves share
machinery.

**Stage 4 — derived footprints: elide past the payload-bound tools.** Probe-time footprint
derivation (`dpkg -L`), so apt-class commands get a footprint computed on the host. Yardstick
climbs — elide past installs, not just fixed-footprint tools. *Teaches: does derivation work, and
does the residue (maintainer-scripts, the cross-kind escape) get professed honestly.* Only worth
doing once Stage 2 proved authored footprints produce correct elisions.

**Stage 5 — grounding + collaboration: the `scan_cve` story, live.** Coordinate-kinds,
grounding-bridges (the expansion bridge for cross-kind effects; co-reference for cross-namespace
sameness), cross-oracle contribution. Yardstick climbs *when oracles collaborate*. *Teaches:
entity-identity / synonym-as-aliasing in practice (must-not-alias-or-wall / dynamic points-to).*
Only worth doing once real derived footprints exist to collaborate over.

**Stage 6 — maximize, measure, conclude.** Tune to maximum elision across a strawman *family* at
varying oracle-quality — the ~80%-coverage north-star question answered *on strawmen*. Extract
what the machine taught into the human docs. *That* is when the spike has earned its death.

---

## Boundaries — explicitly NOT attempted this round

- **The barrier / placement-spectrum (the performance product, `236b`, task #11).** Parked. The
  differential is the net, not a runtime barrier; and by the consent-wall a post-mutation barrier
  buys performance, never attention. Do not build it into the elide path.
- **The real corpus.** Quarantined; strawman inputs only.
- **The elide *contract* is NOT designed up front.** Footprint spelling, grounding-bridge syntax,
  the vouch spelling — all built against *strawman* spellings *to learn what the design should be*.
  Do not stop to design the contract; muddle through against a strawman and let the yardstick and
  the differential teach. (This is the whole point of an empirical round; `23M` is deliberately
  "terminology and landmines, not a design.")
- **The deferred work** (`23O` §5): 22H reactivity (round-25), provenance-DAG (reorderable — may
  ride this spike for legible "why did/didn't this elide", but not a blocker), MH2 versioning, the
  language + `unsafe` hatch, kSTATE, DX tooling, `.diff`, the deferred surfaces. All tabled.
- **Re-opening the settled law** (`23O` §2). Build within it.

---

## The settled law this build MUST honor (from `23O` §2 — cite when you rely on one)

- **consent-wall** — attention is conserved ONLY by pre-mutation static analysis. Never build a
  post-mutation attention-elision; it is DOA. (This is why the barrier is out and why elision must
  be decided at plan time.)
- **silence = wall** — silence never licenses; the trusted claim is opt-in via an explicit
  grounding act.
- **the converged-vouch / adequacy is the CALIBRATION TARGET** — deliberately seed the strawman
  family with converged≠no-op cases (`dpkg -s` holds yet `apt-get install` would upgrade) and
  *measure how often adequacy bites*. This is the round's sharpest empirical deliverable.
- **kFAIL** (probe never mutates; apply never under-executes), **order-sacred** (no intra-host
  reorder/parallelize; apply speed is elision only), **the atomic-command axiom** (no command
  disassembly; whole-line), the **role-split** (`predict` / `is_converged` / `is_diverged`) +
  **rc-partition** (0/1/≥2), **strip-fidelity** (bare marks deleted whole).

---

## Method

- **Build on Opus.** The build is mechanical-ish and does not need Fable's design-judgment;
  reserve the cheap-Fable window for breadth and the round-25 reactivity design. The spike is
  freshly realigned (`23O` §1.5) — build now, before it drifts again.
- **The differential is the gate.** No elision lands without *elided-plan end-state == bare-book
  end-state* across the strawman family, under inert mocks (`sh e2e/run.sh` discipline).
- **xfail-first where it helps** (Stage 3 already has its 9 pins).
- **Per stage:** raise the yardstick, verify the differential, write what-it-taught into a fresh
  round-24 note, and decide the next stage *from the evidence* — the ladder is a default, not a
  contract; let a stage's findings reshape the next.
- **When the mechanism strains** (adequacy bites too often; footprints feel unauthorable; grounding
  is miserable), that strain *is the deliverable* — record it, it is the design conclusion the
  spike exists to produce.

---

## Pointers

History and settled law: `notes/23O`. The mechanism's terminology + landmines: `notes/23M`. The
academic grounding + reading list: `notes/23N`. The de-centered performance alternative:
`notes/236b`. Live state + the task list: `Research/LIVING_STATUS.md`.
