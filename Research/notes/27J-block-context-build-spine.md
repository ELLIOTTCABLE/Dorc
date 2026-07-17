# 27J — block-context build spine (implementation-planning; the lane order + the payload-pins record)

AI-authored (Fable conductor, r27-impl session, 2026-07-17) — the
`270:block-context` implementation-planning artifact. Authority: root docs,
`spike/CLAUDE.md`, `plans/271`, `plans/27C` (consumed whole), `notes/273`/`274`/
`275`, and the human-typed rulings in `notes/27D` outrank this. This note fixes
the LANE ORDER, each lane's brief scope, the formal `270:adj-payload-pins`
record, and the routing of `27D`'s forward seam-list.

## §1 — The formal adj-payload-pins record (discharging `270` §3's owed act)

Human-typed 2026-07-17, in-session (transcribed at `27D`, governing here):

- **`27D:rul-payload-pins-near-weld`** — the human reviewed `270:adj-payload-pins`
  thoroughly; nothing new; "very close to weld, but not quite." The `24T` pins
  (né P-A1) proceed as reviewed; conservative-proceed is authorized.
- **`27D:rul-synthesized-payload-render-stays-unwelded`** (né "R2"/`24T:P-A2`) —
  rendering `sh -c "$SOMECONSTANT"` via value-propagation into an
  engine-synthesized payload string is REFUSED at v1 and deliberately
  just-barely-unwelded: "an *ocean* of downsides… a completely
  unknown-size-of-upside"; "we won't know if we need it until we discover that
  we need it." Build obligation on payload-v1: nothing may FORECLOSE a future
  un-refusal; re-entry trigger = discovered need.

With these, block-context has NO remaining human-presence gates. (The
`27C:law-perfect-overlap` promotion to the standing-rulings surface remains a
non-blocking human act, listed in LIVING_STATUS.)

## §2 — Lane order (serialized, one lineage, same conduct protocol as block-rebuild)

1. **lane-wrapper-peel** (né `24S` W1, surfaces per `273`): peel DETECTION inside
   `cmd__predict()` (command-position `"$@"` runs the argument-slot ⇒ peeling
   wrapper by tautology); `cmd__lend_map()` with the enumerate-every-dimension
   law; wrapper/inner node split + context regions; identity wrappers
   (nice/nohup; the `env "$@"` one-syllable ρ-claim); dual-peel coherence
   fail-fast; ρ-claim ladder (`271:rul-env-claim-inversion` + the `274` §12
   env riders as build obligations); zero new trust surface; rung-0 regression =
   wrapper-free corpus byte-stable. CARRIES the oracle-side positional model
   from `27D` (`Word::PositionalArgs` across both evaluators; quoted-vs-bare
   representable; the founding-one-liner pin guards the transition) — peel
   detection IS command-position `"$@"` modeling, one churn.
   **Referendum watch-item (STANDING, stop-the-block):** if build contact
   forces a wrapper-aware arm into a TOOL oracle, STOP and report — never
   work around (`270` §2; `273` §11).
2. **lane-context-entry** (né W2, re-cut by `plans/27C` — consumes it whole):
   the `cmd__enter()` member (non-interactive by construction;
   authoring-is-vouching for traversal); the ternary escalation dial, all four
   operational cells; the `tolerates:` per-function per-dimension vouch mark;
   per-(host,context) probe segments composed on
   `271:rul-only-oracle-bytes-ship` + its riders; the full degrade ladder
   (every failure ⇒ can't-say ⇒ guard/run); reuse-never-acquire traced through
   code; the authority-disclosure plan-header line; hostsim context-qualified
   verdict injection + two-context e2e fixtures; the §6 mined-idiom lints
   (recognize-never-license); guards-in-context + conditional tails
   (`27C:route-conditional-tail`, minimal mechanics — detailed placement
   belongs to the placement-spectrum round). Gated on the §3 crosscheck
   adjudication (below).
3. **lane-payload-v1** (né `24T` R0, per `274` + §1's record): the `dorc:sh`
   prefix mark; descend-don't-license enforced at TYPE level (this is where the
   invited-rooms compile-failure pin from `279f` §5 lands); the per-run PATH
   shim (+ its DST story per `274` §5); bare `sh -c` = hints only; eval'er
   argparse (which-arg-is-code, stdin shapes, argv-binding); nested parse at
   analysis time; whole-line fold {elide, guard-conjunction, run};
   derived-text locators; the composed acceptance shape
   `echo data | sudo sh -c 'cat >> /etc/f'` (pipeline ∘ context ∘ payload);
   synthesized-payload-render REFUSED with the door-open obligation (§1).
   Basic-forms exploration punt-empowered per `24T:P-A3`.
4. **lane-read-value-slice**: the `$(hostname)` capture fold, first slice —
   `275` §4's validity table + the reversible floor (world-spoken only;
   single-line via the landed wire stdout field); the post-probe re-bind seam
   BUILT (choosing between the second value-flow pass and the fold-time
   substitution channel — the stage-4 recipe representation kept both open);
   the `279f` hard gates: never elide a capture-binding with live apply-time
   consumers outside the folded region; the walls-patrol clarification
   (unmodeled interposers wall by default); the merged-streams capture fence
   checked AT CAPTURE; the nested-wrapper lend/ρ composition rule (pointwise,
   ⊤ propagates); DST must-covers (spaces, empty output, nonzero rc, merged
   stderr, hidden walls, probe/apply value disagreement).
5. **Fallback lane (`27C` §4): NOT BUILT this block beyond its floor** — the
   engine-warranted carried-by rows land where lane-context-entry needs them
   (substrate-borne, structural, unflagged); the invariance-line ×
   `--risk-faultless-skips` consumption ships behind the existing flag ONLY if
   it falls out nearly-free of lane 2's plumbing; otherwise
   honest-walls-for-worlds (the recorded v1-defer) stands and the lane waits
   for field pressure. Punt-empowered.

## §3 — The wrapper-design crosscheck (the `270` §6 obligation, sized to the
human's autonomous-run budget ruling)

`273` rode the 279-series crosscheck; `plans/27C` POSTDATES it and is the
least-reviewed load-bearing design (its §3 entry mechanics + §5 conditional
tails especially). One focused outside-lineage review: Sol (codex) only, via an
Opus manager per the foreign-models skill; exclusions-not-inclusions framing;
runs PARALLEL to lane-wrapper-peel (read-only, no collision); its findings
adjudicated under maximum skepticism (coherence + my own verification required
before any churn) BEFORE lane-context-entry dispatches. No DeepSeek, no
antigravity, no Fable (human-ruled 2026-07-17).

## §4 — Seam-list routing (from `27D`'s block-close)

fact-plane context keying → lane-context-entry (the context slot becomes real
there; FactKey widening decided at that brief) · oracle-side positional model →
lane-wrapper-peel (§2.1) · minting-LINE threading → a lane-context-entry rider
(must precede stdlib selector-bearing disturbs) · cross-kind backing members →
lane-read-value-slice (derive-model widening) · invited-rooms compile pin →
lane-payload-v1 (§2.3) · effect-check re-homing → conductor discretion at
block-stdlib · tc-stage-ship-triplication + batch-5 + E3 → unowned tails,
opportunistic.

## §5 — Acceptance for the block

Every lane: four gates, foreground e2e, isolated blesses with named delta
classes, granular commits, landing note per stage, conductor checkpoint per
landing (same protocol as block-rebuild; ledger stays `27D`). Block-level:
the `24S` §8 invariants list + `24T` §6 asserted-semantics ledger are
test-pinnable obligations (`270` §6); every cross-context elision renders its
four-link attribution chain from day one; wrapper-oracle + eval'er briefs carry
the quality-bar checklists (`24S:A6` + `24T:P-A4`).
