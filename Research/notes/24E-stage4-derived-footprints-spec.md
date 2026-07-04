# 24E — Stage 4 spec: derived footprints (host-executed `touches()`) — conductor spec

AI-authored (Opus conductor), 2026-07-04, round 24. The type-contract + design SPEC for Stage 4,
authored by the conductor per `rul24-overtype`. **Status: PENDING HUMAN REVIEW before any builder
dispatch** — Stage 4 was flagged "not blind-dispatchable" because it opens a NEW execution surface
in the probe phase (`kFAIL-withhold` territory), and the human asked to eye the inertness contract.
Confidence-marked. The three fork-decisions (`fork-4A`/`B`/`C`) were pre-cleared with the human in
conversation 2026-07-04; this note consolidates them + the pieces not yet discussed (hostsim
extension, `resid-kill-coherence` close, the Stage-4/5 boundary, the type-shapes, testing).

## §1 What Stage 4 builds, and why it must exist

Stage 2 built **authored** footprints: an oracle's `touches()` statically emits the entity-
coordinates a running command touches (`apt-get.touches()` → `package:nginx`), traced in-Rust by
`evaluate_touches`. That works only for **fixed-footprint** tools. It CANNOT work for
**payload-bound** tools — `apt-get install nginx` touches whichever files/services/users *that
package on this host at this version* lays down, which the author cannot enumerate statically. The
elide-goal for payload-bound commands therefore survives only by **deriving the footprint at probe
time — asking the tool** (`dpkg -L nginx`, `apt-get install -s`, `hork list-plugins`). This is the
separation-logic frame rule through a **dynamic frame** (Kassios 2006): the mod-set is computed at
runtime, not declared. (The licensing basis for payload-bound elision "moved from testimony to
derivation" — `233` closing-annotation correction-3.)

Yardstick effect: elide past *installs*, not just fixed-footprint tools. Teaches: does derivation
work; are derived footprints authorable by hand without misery; does the residue (maintainer-
scripts, the cross-kind escape) get professed honestly.

## §2 The seam (where it plugs in — traced in code, +SURE)

Today `cli::resolve_touches_footprint` (main.rs) calls `evaluate_touches(touches, argv)` — a pure
in-Rust tracer that walks the `touches()` body and ⊤-rejects on ANY non-`printf` command (so a body
containing `dpkg -L` currently ⊤s → no footprint → the site walls). Stage 4 lets such a body instead
**ship into the probe lane** (the same `compile_probe` path that already ships stripped
`predict()`/`is_converged()` bodies via its `ship_body` closure), run read-only on the host, and have
its **stdout** parsed into the footprint.

**Only the footprint SOURCE moves.** Reused verbatim (24C handoff, verified): the opaque
`kind:entity` split + interning (`resolve_touches_footprint`'s tail), the `Footprint`/`Backing`/
`disjoint`/`wall_verdict` survival machinery (`plan/survival.rs`), the coherence check (own-establish
⊆ footprint), and `--trust-footprints` gating. `resid-argparse-drift` (24C) dissolves entirely — the
tool emits its own footprint, so there is no second argparse to drift from `predict()`'s.

## §3 fork-4A — the probe-inertness contract (THE reviewable core)

The anchor principle (round-20 welded ruling): **Dorc never decides a command is read-only by
*analyzing* it.** Mutation-analysis is permanently impossible. Probe-inertness comes from three
layers, only the first load-bearing:

1. **Structural self-vouch (load-bearing).** Trust anchors to *authorship*. Writing
   `apt-get.touches() { …dpkg -L… }` IS the vouch that this body is a read-only footprint-derivation
   — same act as authoring `predict()`/`is_converged()` (`rul24-vouch-is-verdict-authoring`
   generalized; `touches()` is the third role-sibling, `rul24-threefunc-monotonic`). Dorc ships and
   trusts it; it does not verify it.
2. **The closure-check (cheap, structural, currently deferred/moot).** A body's every *call* must be
   {the oracle's own command, a declared read-only Query (`:?`), a blessed-pure builtin}. Checks
   declared structure, never inferred mutation. `touches()`'s only new demand: the derivation idioms
   land in "declared Query" — `dpkg -L` is a declared Query (base library ships it); `apt-get install
   -s` is covered by "own command".
3. **The rc-127 mocks net (live, dynamic).** The e2e harness runs probes under `PATH=mocks-only`; an
   unmocked command 127s and gate-1(c) screams. The live guarantee while layer-2 stays deferred.

**The load-bearing conclusion: NO NEW TRUST EDGE.** A host-run `touches()` sits at *exactly* the same
tier as a host-run `predict()` — same phase (probe), same self-vouch anchor, same worst case if the
author botches read-only-ness (a `kFAIL-withhold` breach, the one soundness never traded). It is
emphatically NOT the sharp-knife tier: that is Stage 5's footprint-*trust*, where a survived elision
under `--trust-footprints` silently deletes *someone else's* line. fork-4A is same-tier surface-
extension. The one quantitative difference: touches bodies reach for a host tool more often than a
convergence-check does (asking-the-tool is their whole job), so the "declared Query" path is
exercised harder — a frequency difference, not a new tier.

**The one caveat, professed (and `predict()` already has it):** the naked spot is one flag wide.
`apt-get install -s` reads; `apt-get install` mutates. `dpkg -L` reads; `dpkg -i` mutates. The self-
vouch trusts the author wrote the read-only form. `251` logged a real instance of this class ("an
apt-get `-o` flag leak makes the probe itself mutate"). Same tier as the existing predict-body
hazard, no worse — but it is the honest boundary and the note says so in those words.

**Spike-pragmatic stance:** rely on layer-1 (authored strawmen are read-only by construction) +
layer-3 (mocks net is live). Note layer-2's closure-check as owed-for-production; do NOT build it now
— mirroring exactly how predict-body inertness is handled at HEAD (closure-check moot, rc-127 live).
Stdout-vs-rc (fork-4C) is orthogonal to inertness: a read-only command emits stdout freely.

## §4 fork-4B — static-default, dynamic-escalate (framed degrade-not-kill)

The mirror-invariant (`kLANG` off-ramp weld; human correction 2026-07-04): **valid sh is NEVER
hard-killed — it degrades and warns; hard-kills are reserved for Dorc-only syntax boundaries.** This
splits `evaluate_touches`'s existing ⊤-reasons cleanly:

- **`NonPrintfCommand` ⊤** (the body calls `dpkg -L` — a real host tool): the EXPECTED escalation
  trigger. Instead of walling, escalate — ship the body to the probe lane, run it, parse stdout.
  Emit a provenanced advisory: `site N: touches() escalated to host-derivation (dpkg call at
  <oracle>:<line>)`. This warning is SPIKE INSTRUMENTATION — it makes the static→dynamic boundary
  visible in the yardstick/differential. Marked spike-only under `ru-26` (churn-avoidance disclosure;
  must not leak into greenfield as a permanent requirement).
- **Every other ⊤** (unmodeled printf directive, malformed coordinate, arg-count mismatch, non-
  concrete word, budget): degrade to WALL — the command RUNS (safe) + warn. NEVER a hard-kill; the
  book's sh is untouched. (The "refuse the unexpected" convenience the human blessed is SPIKE-LOCAL
  and applies ONLY to Dorc's own annotation-parsing — a malformed `:` mark, which is new Dorc syntax
  that isn't valid standalone sh — never to the sh underneath.)

Static-first is also the correct real-world instinct: don't pay a host round-trip to learn what a
plan-time trace can compute. So the escalation only fires where static genuinely can't resolve.

## §5 fork-4C — the stdout readback channel

`predict()` reports an *rc* (the verdict); `touches()` reports *stdout* (the coord lines). Per `142`
(executorless-OOB, settled): short gating signals ride the shared fast-lane; large non-live payloads
ride **per-leaf files, demuxed by leaf-id filename**. A derived footprint is large (unbounded — a
fat package's `dpkg -L` is hundreds of lines) and non-live (consumed at plan-construction after all
probes return), so it is a per-leaf-file payload, NOT a fast-lane verdict. In the spike there is no
FIFO/file infra — the cli reads probe results from stdin as the Seam-1 stand-in — so concretely this
is: **add a stdout-payload field to the site-keyed probe record** (`inv-site-keyed-results` already
carries one per-site datum; this is a second). The probe-result parse gains a per-site coord-blob
lane, demuxed by the same `site N` keying the rc lane uses. Minimal; maps onto the settled `142`
shape so nothing is committed-by-accident.

## §6 hostsim extension — the derivation-answer shape

`hostsim` is the in-process Seam-1 stand-in: it SYNTHESIZES host answers from a seed, never spawns
ssh/dpkg (`128 se-2`). Today `Host` answers `verdict(fact)` (set-membership: Converged iff the fact
holds). Stage 4's one extension: `Host` gains a **derivation-answer** — given a touches-site + argv,
return the modeled entity-set (the coords the derivation would emit). Modeled the same way as
`verdict`: deterministic, seed-driven, no ssh. **The host returns a DECLARED entity-set (scenario
data), it does NOT simulate dpkg's real behaviour** — this keeps the host-model-growth in check.

This lands cleanly on the sweep's existing declared-vs-true split (24B §3): the *declared* footprint
= what the (possibly-lying) derived `touches()` returns; the *true* effect = the `CellDelta` ground-
truth. So the Stage-2 lying-footprint net (24C `find-net-covers-what`) EXTENDS to derived footprints
with no new machinery — a too-narrow derived footprint → wrong survival → the end-state differential
goes RED. Strawman scenario:

```
honest:  Host models nginx's payload = {package:nginx, file:/etc/nginx/nginx.conf}; derived
         touches() returns it; a downstream cp to that file intersects → correctly does NOT survive.
lying:   derived footprint returns {package:nginx} only (misses the file it writes; CellDelta kills
         the cp's fact) → the cp wrongly survives → bare has the effect, plan elided it → RED, caught.
```

**Caveat (flag, keep minimal — `hostsim/CLAUDE.md`'s own tension):** teaching `Host` a derivation-
answer nudges it toward re-implementing a real host, a second source of modeling-bugs that can
mask/manufacture analyzer bugs. Keep it a declared entity-set, not a dpkg simulation. Surfaced, not
blocked.

## §7 resid-kill-coherence close (24C flagged Stage 3/4)

Establish-walls get the at-least ⊆ at-most coherence check; kill-walls skip it (no single establish
cell). A drifted kill `touches()` → a too-narrow kill footprint → a downstream fact on the really-
killed entity wrongly survives. Stage 4 threads the killed fact (the kill's own entity-coordinate,
available from the `SkipClass` kill classification) as the coherence comparand for kill-walls,
closing the narrow under-execute. Do this AS PART of the derived-footprint wiring (the kill's
coordinate is in hand at the same site the footprint is resolved).

## §8 The Stage-4 / Stage-5 boundary (name it, so scope doesn't creep)

`dpkg -L` appears in both stages; the boundary (~SUSPECT, to firm in the build):
- **Stage 4 (this note):** the `touches()` body, running on the host, emits WHATEVER coordinates it
  wants — including cross-kind ones it derives itself (an `apt-get.touches()` that runs `dpkg -L` and
  emits `file:…` lines is in-scope; the body does its own translation). The engine just captures and
  interns what the body prints.
- **Stage 5 (deferred):** ENGINE-mediated coordinate work — grounding-bridges (the engine expands one
  oracle's `package:` footprint into `file:` coords via an owner-provided `manifest()` so a DIFFERENT
  file-reasoning oracle can intersect), co-reference (cross-namespace sameness), and the
  `resid-aliasing` closure (dynamic points-to over *resolved* locations, so `nginx`/`nginx-full` and
  symlinked paths stop coming up wrongly-disjoint). None of that is Stage 4.

## §9 Type-shapes the builder implements (`rul24-overtype`; lighter than 24D)

Stage 4 is foundation-light — it REUSES Stage 2's `Footprint`/`Backing`/`disjoint`/`SurvivalWitness`
and adds no tier-algebra change. The shapes:
- **A provenance discriminant on a footprint's origin** — `Authored` (static `evaluate_touches`) vs
  `Derived` (host-run). Carries the escalation-site + the derivation call's locus (for the fork-4B
  spike warning + attribution: the why-lens must say "footprint DERIVED at probe from `dpkg -L`").
  Keep it a provenance tag on the existing `Footprint`, not a new type — the disjointness/survival
  consumers are origin-agnostic (a derived footprint intersects identically to an authored one).
- **The site-keyed derived-footprint payload** (fork-4C) — a per-site coord-blob lane on the
  probe-result record, demuxed by `site N` keying.
- **The touches-body probe-lane shipping** — reuse `compile_probe`'s `ship_body` seam shape; a
  touches-derivation ships as a third body-kind alongside the predict bodies. It ships STRIP-ONLY
  (strip-fidelity: `name.touches()` → `name_touches()`, marks deleted whole), same as predict.
- **Inertness is STRUCTURAL, not a type** — there is no "InertFootprint" witness; the self-vouch is
  the authoring act (§3). Do not invent a type that implies Dorc verified inertness (it can't — the
  `hostsim` withhold-monitor + mocks net are the DST stand-ins, never a proof; never-vouch).

## §10 What Stage 4 must NOT do (scope fence)

No grounding-bridges / co-reference / `resid-aliasing` closure (Stage 5). No new trust tier (the
survival flag `--trust-footprints` and its mint are unchanged; a derived footprint is consumed by the
SAME survival machinery). No closure-check build (owed-for-production, §3). No hostsim dpkg-simulation
(§6). No real ssh/executor (that is the early-25 real-machine arc — and the ADEQUACY-of-derivation-
to-reality question, whether `dpkg -L` on a real box lists everything the install touches, is
STRUCTURALLY un-spike-testable per `128 se-2`; the spike tests the mechanism, the field trial tests
reality). Do not relitigate settled law.

## §11 Testing (no ssh; in-memory DST + e2e-under-mocks)

- **In-memory DST (`hostsim` + `dorc-sweep`) — the soundness net.** The derivation-answer extension
  (§6) + the existing declared-vs-true `CellDelta` split → the lying-derived-footprint scenario goes
  RED. This is the primary net; it needs no ssh (the arch the human is NOT worried about).
- **e2e under `PATH=mocks-only` — the sh-execution + wiring net.** The stripped touches body ships as
  sh; a mock `dpkg` shim emits a canned `-L` manifest; dash runs the probe locally; the stdout
  readback (fork-4C) parses the footprint; the plan builds. Exercises strip-fidelity, the stdout
  channel, and the escalation path — locally, no ssh.
- A new `strawman24-derived-*` family member on the yardstick (differential-verified), sibling to the
  Stage-2 `strawman24-*` fixtures.

## §12 Confidence

+SURE: the seam (§2, code-traced); fork-4A's no-new-trust-edge argument (same phase/anchor/worst-case
as predict); the reuse of Stage-2 survival machinery; the hostsim declared-vs-true extension riding
the sweep's existing split. ~SUSPECT: the Stage-4/5 boundary (§8 — where body-emitted cross-kind ends
and engine-mediated bridging begins; firm it in the build); that the provenance-tag (§9) is boundary-
only and doesn't churn the survival consumers. -GUESS: the exact spike shape of the per-site coord-
blob lane (§5 — builder's call, low lock-in, maps onto `142`).

**CLOSURE-CHECK RESOLVED (human, 2026-07-04):** the §3 layer-2 static closure-check stays DEFERRED for
touches too — "excessive right now," parity with predict; the rc-127 mocks net is the live guarantee.
Do not build it.

## §13 — post-recon corrections + fork resolutions (conductor, 2026-07-04; supersede where they conflict)

A Stage-4 recon pass (a warm Opus that read the whole code surface) found §2/§7 imprecise and surfaced
five forks. All verified against code before adoption. These resolutions are BINDING on the build.

- **corr-§2 (the pipeline-stage undersell — the biggest correction).** "Only the footprint SOURCE
  moves" was wrong. Static footprints build purely statically (`cli/main.rs:308`, consuming no stdin).
  A DERIVED footprint requires a NEW `cli::run()` pipeline stage: (1) compile a derivation-probe for
  each escalated wall-candidate site; (2) emit it INTO the probe artifact (phase-1, before stdin);
  (3) read the per-site coord-blobs back from the results; (4) merge into `TrustedFootprints` BEFORE
  `build_plan_walled`. The `Footprint`/`disjoint`/`wall_verdict` survival machinery downstream is
  unchanged; the SOURCE *and the probe round-trip* are the new work. (This is why Stage 4 is not a
  drop-in swap of `evaluate_touches`.)
- **corr-§7 + fork-s4-killcoord (RESOLVED).** Verified: `SkipClass` has NO `Kill(FactKey)` variant —
  kills fold to `MustRun` (`analysis/effect.rs:1119`) and the killed `FactKey` is not threaded out of
  `classify` (`cli` receives a bare `kills: BTreeSet<CfgNodeId>`). Resolution: thread a **side-map
  `BTreeMap<CfgNodeId, FactKey>`** (kill-node → its killed coordinate) out of `classify` alongside the
  existing `kills` set — NOT a new `SkipClass::Kill` variant (least churn to the load-bearing
  classifier). Use it as the kill-wall coherence comparand (own-killed-coord ⊆ footprint).
- **fork-s4-compile (RESOLVED → parallel `compile_derivations`).** Do NOT extend `compile_probe`. A
  parallel builder: different site-set (escalated wall-candidates, not elision-candidates), different
  body-source (`touches` not `predict`), different readback (stdout-coords not rc). Reuse the
  `ship_body` seam-SHAPE, leave the load-bearing convergence-probe path unperturbed.
- **fork-s4-coordwire (RESOLVED → dedicated site-keyed record).** A footprint is multi-coordinate; the
  reserved single `stdout=`/`OutClaim` slot on the verdict record does not fit. Use a DEDICATED per-site
  record, e.g. `deriv <leafid> coord=k1:e1 coord=k2:e2 …`, demuxed separately so a derivation-blob never
  collides with the site's `effect=`/`rc=` verdict record (`inv-site-keyed-results`). Maps onto `142`'s
  per-leaf-file demux.
- **fork-s4-declaredtrue (RESOLVED → the declared-vs-true derived model).** The sweep generator invents,
  per escalated site, a TRUE mod-set (its `CellDelta`) AND a DECLARED derived footprint that
  `Host::derive` returns. A footprint is an *at-most* claim, so soundness needs it to be a SUPERSET of
  the true mod-set: honest ⇒ derived ⊇ true; lying/too-narrow ⇒ derived ⊂ true (misses a truly-touched
  cell) → wrong survival → end-state differential RED. Mirrors the static lying-footprint net exactly
  (24C `find-net-covers-what` — the lying scenarios are load-bearing).
- **§8 boundary (CONFIRMED → within-kind, body-emitted).** Stage 4: the `touches()` body computes and
  emits coordinates in whatever kinds its own sh computes (including a `dpkg -L`-derived `file:` set) —
  entity-granular; the engine interns + intersects but NEVER bridges. The moment engine-mediated
  cross-kind expansion (`package:X` REACHES `service:X`) or cross-namespace co-reference is needed, that
  is Stage 5. Slogan: **"body computes and emits; engine interns and intersects, never bridges."**
  (Matches `23M`:219–247's cross-kind landmine.)
