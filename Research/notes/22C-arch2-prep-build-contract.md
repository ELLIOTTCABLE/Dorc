# 22C — arch-2 prep + build contract (the emit-at-origin consumer)

> Round-22 conductor, arch-2 prep (2026-06-13). Written after a full-corpus read
> (README/DESIGN/IMPLEMENTATION/KNOBS/ANALYZER-NEEDS/22B + the TODO trio) and a
> read of the live seams arch-2 extends (`core/src/prov.rs` arena, `plan/src/
> erasability.rs` gate, the `None` cause hooks). This is the BUILD CONTRACT for
> the arch-2 dispatch — design-only, no code landed. It is the artifact the
> adversarial review-pair (XC-2) pressure-tests before any builder is dispatched.
> AI-authored; process evidence, never proof. Confidence marks +SURE/~SUSPECT/
> -GUESS/--WONDER.

## §0 The thesis — arch-2 makes the receipts/why machinery LOAD-BEARING

arch-1 built the ProvId arena + `Top(cause)` + the erasability gate; arch-3 built
the typed diagnostics catalog + the three render lanes. Both are real but the gate
is VACUOUS-AT-HEAD (x-1, 224 §11): nothing in the decision pipeline reads a
receipt, so "strip receipts ⇒ decisions identical" is proving the inertness of a
thing with no consumers. The triad that IS the vacuity: `top_cause()` has zero
callers; the diag `cause: Option<ProvId>` fields are `None` at the give-up sites
(`tc-cmdsub-cause`, deferred here); the canary asserts only "witness non-empty,"
not "witness DIFFERS yet decisions identical."

arch-2 wires the FIRST consumer that reads a receipt — the **why-lens** — and in
doing so de-vacuums the gate. The why-lens is the concrete realization of the
human's `dir-soundiness-ux` direction (STALENESS-AUDIT): frontload the
unsoundness via *per-line, at-decision-point* disclosure — "why was this command
forced to run (never elided)?" — rendered on the plan-render surface, reading the
⊤-cause, classed by remediation. +SURE this is the right consumer: it is
exempt-plane by construction (a *rendering*, never a decision), so it reads
receipts WITHOUT violating ru-11's one-way weld, which is exactly the shape the
gate needs to become non-vacuous (receipts demonstrably read, decisions
demonstrably unaffected).

## §1 The live seams arch-2 extends (grounded, read at HEAD)

- `core/src/prov.rs`: `Arena::{mint, leaf(kind, site), join(inbound)}`,
  `OriginKind::{… TopCause …}` (closed), `Witness` (the uncapped full granted
  license witness, vp-17/18). The cause-minting machinery EXISTS — arch-2 CALLS
  it, doesn't build it.
- `plan/src/erasability.rs`: `canonical_decision(...)` builds the identity-plane
  string (plan + probe + rendered artifacts + diagnostics — only Error-class
  diags by `(code, site, severity)` are identity); `decision_digest`; the exempt
  enum `{Explanation, ReceiptId, OriginOrdering, Timing}`; run-B already injects
  witness sentinels + reorders origins; the canon destructures exhaustively (no
  `..`). The GATE EXISTS — arch-2 upgrades its CANARY and adds a why-lens-reading
  fixture.
- `core/src/diag.rs`: `CmdsubOperandTop{ …, cause: Option<ProvId> }` and siblings
  carry the hook, `None` at emit. `render_cli` / `render_artifact_comment` /
  `project_oob` are the three lanes (22B §4). The diag value carries the slot —
  arch-2 fills the cause and adds the why-lens render.

## §2 The build, in dependency order

- **mvs-A — emit-at-origin cause-wiring** (concl-5 / 228 dc-1; resolves
  `tc-cmdsub-cause`). At each ⊤-ORIGIN give-up site (start with
  `CmdsubOperandTop`; the worked-3 example), mint a `ProvId`
  (`arena.leaf(OriginKind::TopCause, Some(span))`) at the moment the ⊤ is created
  and put it in the diag's `cause` field. NEVER reconstruct the cause after the
  fact (the Clang NoteTag lesson — generate where the info already exists). The
  mint is at ⊤-creation in the effect pass; the diag carries the handle.
- **mvs-B — the why-lens consumer** (THE de-vacuuming consumer; dir-soundiness-ux).
  A render/query that READS the cause and produces a per-line "why forced to run"
  explanation, classed by `RemediationClass` (ru-6), on the plan-render / `why`
  surface ONLY — never the artifact (rec-1). This is the first receipt-reader.
  Minimal shape: a `why(site) -> Explanation` or an inline plan-render annotation;
  §6-fork-surface decides which.
- **mvs-C — the suppression rule-set** (228 mvs-1..5, as tested code): carry-cause
  at origin · interestingness-from-sink pruning · same-fact tie-break by speaker
  priority · observe-THAT-⊤-never-WHY (the why-lens shows the cause-site; pure
  consumers never re-emit — f-3b is already this shape) · flush-or-trip net. Dedup
  is in RENDERING (collapse by `coarse_key`), never destroyed at capture (Sentry's
  scar; 228 dc-4).
- **mvs-D — the gate-obligation canary upgrade** (THE load-bearing test work; see
  §5). Now that the why-lens reads the cause, upgrade the canary from "witness
  non-empty" to "witness DIFFERS across run-A/run-B yet the identity plane is
  byte-identical," and land a fixture that drives the why-lens read under run-B's
  variance.
- **mvs-E — secondary-span / span-bridge** (tier-2/3; 22B worked-3): the cause-site
  (primary) + the poisoned-consumer site (secondary `SpanLabel`) in ONE diag — the
  multi-span model that lets one authored diagnostic replace N scattered notes.
- **mvs-F — ValueOf::Top cause** (deferred from arch-1): the value-plane ⊤ gains a
  cause too (today only `Reach::Top` carries one). ~SUSPECT this is small but it
  is a SECOND ⊤ plane — §6-fork-valuetop asks whether to wire it now or defer to
  keep arch-2 tight.

## §3 The contract lines arch-2 MUST honor (the welds)

- **ru-11 one-way weld (the load-bearing one).** Receipts influence NOTHING in
  allow/reject. The why-lens is a RENDERING; it reads the cause to EXPLAIN, never
  to DECIDE. Any decision keyed on receipt content is a weld violation. The gate
  (§5) is what proves we held this. +SURE this is the single most important line —
  the entire arch-2 design rests on "the consumer is a renderer, not a decider."
- **rec-1 two-surfaces (ru-12 + ru-20).** The why-lens output renders on the
  plan-render surface (CLI / `why` / future dashboard), OVERLAID on the artifact
  bytes, NEVER embedded in the byte-floored `.sh` artifact. `render_artifact_
  comment` stays fact-plane (the existing disposition/provenance comment); the
  cause/remediation/prose are exempt-plane (render-surface only). The erasability
  gate asserts the artifact is byte-identical with receipts stripped.
- **dir-soundiness-ux (the user-facing purpose).** Per-line, at-decision-point
  disclosure — claimed-vs-proven taint surfaced in the plan render, why-elided /
  why-probed per leaf, blame landing on the specific oracle line. The phase
  asymmetry (STALENESS-AUDIT AI-caveat): apply-side residual failures are educable
  (the why-lens teaches); probe-side mutation is trust-catastrophic (engineering
  backstops, not pedagogy) — the why-lens is apply-side education, correctly
  scoped.
- **emit-at-origin (concl-5).** Mint the cause where the ⊤ is created; pure-
  propagation consumers inherit silently and never have standing to emit. Never
  emit-N-then-dedup.
- **The four priorities (DESIGN/IMPLEMENTATION).** correctness-within-contract >
  user-effort > performance > invisibility. The why-lens serves user-effort
  (#2 — less work to understand a forced-run) and invisibility (#4 — it reads like
  the admin's own sh). It must not cost correctness or add probe-phase work.
- **kFAIL-perform / inv-top-reject.** A ⊤ operand runs (the why-lens explains WHY
  it runs; it never licenses NOT running). inv-determinism (the arena/why-lens
  iterate ordered collections only) + inv-no-throw (the why-lens returns data,
  never panics).

## §4 Scope boundaries — explicitly NOT arch-2

- The FLEET rollup / `coarse_key` real scope-keying — stays stubbed
  (`22B-fork-scope-key`: coarse=fine this round). The trait slot exists; the
  rollup that consumes it does not.
- The DASHBOARD (render-4) — a future projection; arch-2 ships CLI/`why` + the OOB
  lane projection only.
- The DURABLE / replay / why-tape — that is arch-4 (wave-3). arch-2's why-lens
  reads the LIVE arena, not a persisted durable. (rec-5: the probe-tape is a
  separate, write-only durable; do not conflate.)
- `dorc fix` auto-apply (consuming `Applicability`) — out of scope (22B
  type-sketch-3 NOTE).
- ui-A / ui-B — separate (ru-25); the why-lens is a render consumed by them later.

## §5 The gate-obligation (mvs-D) — in detail, because it is the point

The erasability gate today proves "stripping receipts ⇒ identity-plane identical,"
but VACUOUSLY (no consumer reads a receipt, so run-A ≡ run-B because the perturbed
data is write-only). arch-2 makes it non-vacuous in TWO moves:

1. **Land a fixture where the why-lens READS the cause.** A book that produces a
   ⊤-origin (e.g. a `$(…)` operand) ⇒ a `CmdsubOperandTop` with a real
   `cause: Some(ProvId)` ⇒ the why-lens renders a cause-derived explanation. This
   is the first place run-A and run-B's receipt difference reaches a CONSUMER.
2. **Upgrade the canary.** From "witness non-empty / ≥1 Replace / nonzero arena"
   to: run-B's witness/cause DIFFERS from run-A's (the existing run-B variance
   already injects sentinels + reorders origins — confirm it reaches the cause
   the why-lens reads), AND the identity plane (`canonical_decision`: plan +
   probe + artifact bytes + Error-diags) is byte-identical. The assertion becomes
   "the why-lens output COULD differ (receipts differ) yet decisions DO NOT" —
   the actual non-vacuous inertness proof.

PROOF-OF-BITE obligation (arch-1's strain-1/strain-2 lesson): the builder must
inject a synthetic leak (make a DECISION read the cause — e.g. key a disposition
on `cause.is_some()`) and confirm the upgraded gate BITES (identity plane moves);
then revert. A gate that cannot be made to fail is the x-3 vacuity again.

The disposition for x-1's coverage-doc test (`ai/r22-xcheck1` @ `b68fc66`, which
DOCUMENTS the vacuity): SUPERSEDE, don't fold. It recorded "the gate is vacuous
at HEAD"; arch-2 makes that false. Re-derive its intent as the mvs-D upgrade
(the doc-test becomes the real variance fixture). Harvest nothing from the branch
verbatim; the branch is a tombstone for the vacuity it described.

## §6 Forks for the human (decide at dispatch, or let the builder propose)

- **22C-fork-origins** — which ⊤-origins get cause-wired in arch-2? `CmdsubOperandTop`
  is the worked example and the minimum (it de-vacuums the gate). The others
  (`RedirTargetTop`, the cfg-top-node depth site, the member-family ⊤) could ride
  along or defer. Conductor lean: wire `CmdsubOperandTop` + `RedirTargetTop` (the
  two value-ish ⊤ operands) for arch-2; defer the rest to a mechanical sweep —
  enough to prove the consumer + the gate, not a 23-site retrofit.
- **22C-fork-surface** — the why-lens surface: an inline per-line plan-render
  annotation, a `dorc why <site>` query, or both? dir-soundiness-ux leans
  per-line inline + why-elided/why-probed as primitive queries every panel
  composes over. Conductor lean: build the `why(site)->Explanation` primitive
  first (it is the data the gate reads + what ui-A/B later compose), render it
  inline as the minimal CLI surface; defer a standalone `dorc why` subcommand.
- **22C-fork-valuetop** — wire `ValueOf::Top` cause now (mvs-F) or defer to keep
  arch-2 tight? Conductor lean: defer — `Reach::Top` cause is enough to
  de-vacuum the gate; the value-plane ⊤ is a second plane that can ride a later
  mechanical pass (its consumer is the value why-lens, vp-23-aligned).
- **22C-fork-remediation** — the `RemediationClass` per code (which code →
  AuthorOracle / AddDeclaration / FixBookLine / Structural). A small ratification
  like the floor column; builder-proposes / human-disposes at the PR.

## §7 Prep mechanics — status

- x-1 coverage-doc test: disposition = SUPERSEDE (see §5). Recorded.
- arch-3-residual-1 must-emit audit: DONE (B8 all-23 pins + XC-1 closed the two
  unit gaps). No carry.
- 22B re-read in full (this prep). The `22B-fork-*` dispositions from arch-3 hold
  (payload=typed, scope-key=stub, wire-code=string-slug, floor=builder-proposes,
  severity-help=no).
- TODO.md noisy-harness line: addressed by B5's `DORC_E2E_QUIET` knob (human's to
  resolve/commit; not arch-2's).

## §8 The dispatch shape (post-XC-2, post-human-fork-rulings)

ONE Opus builder (arch-2 is design-surface-heavy; the gate-obligation needs
judgment), fb-19-clamped, in a conductor-created worktree at a verified base.
Brief = this contract's §2 (build order) + §3 (welds) + §5 (gate-obligation with
the proof-of-bite) + the human's §6 fork rulings. Granular commits per mvs-step
so partial completion is harvestable. The gate-obligation (mvs-D) is the
acceptance gate: arch-2 is not "done" until the canary is non-vacuous and
proof-of-bite is demonstrated. Crosscheck candidate after harvest: x-2
(over-suppression — does the why-lens's root-cause-only rendering hide a second
independent cause?), which is the natural adversarial pass on mvs-C and wants
fr-2 first.
