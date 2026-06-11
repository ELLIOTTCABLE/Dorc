# 21F — The r1A direction report (imp-1..6), adjudicated into round-21

> Orchestrator note. The human relayed the r1A orchestrator's close-out direction
> report mid-round. Numbering decode: r1A read a PRE-DOORS charter draft (no errexit
> arc; its arch-3/4/5 = our arch-4/5/6; its "207 routes elision through Query-guard
> folds" framing predates `plans/20V`). Substance adjudicated below; commit deferred
> to next tree-quiesce.

## imp-1 — redirect-effects (y-1): the round's missing task, and a candidate P1-AT-HEAD

r1A's chain: the H2SaLS helpers' `printf … >> file` edits are invisible (printf
blessed-pure, Redir⇒Pure) but accidentally MASKED today because the helper calls are
Opaque poison; arch-2's inlining lifts the poison ⇒ downstream grep-guard Queries of
the just-edited files compute `valid=true` ⇒ QueryGuard mints stale-guard elisions —
inlining-without-redirect-cells MANUFACTURES wrong-elisions in the round's primary
mechanism.

Orchestrator adjudication, two parts:

- a-1 The arch-2 containment already briefed (tc-M2 honor: a body containing a
  real-sink write-redirect ⇒ the call REFUSES) fences the inlining-armed form — IF
  refusal lands as effect-Opaque (poison-preserving). Harvest checklist + the arch-2
  crosscheck get a mandatory construction: helper `printf >> "$CONF"` + downstream
  grep-guard of `$CONF` ⇒ no QueryGuard mint, validity must NOT compute.
- a-2 ~SUSPECT (design-read, construction pending — calibration rule: no +SURE on an
  untraced mechanism): the BOOK-LEVEL form needs no inlining at all. `set -e;
  printf 'x' >> f; grep -q x f || mutator` — printf is a pure-builtin, and st-3
  validity exempts pure-builtins from invalidation (only oracled mutators or Opaque
  invalidate), so a grep-keyed query oracle would compute valid against a file the
  book itself just wrote; probe-rc folds the guard stale. Nothing in the corpus can
  hit it (no file/confline-keyed oracle is loaded in our e2e set; r1A's confline seed
  is exactly the arming vocabulary). Disposition: mandatory construction item on the
  arch-2 crosscheck (which will have a built binary and the seeds available), and
  task y-1 created NOW — the cheap poison-correctness-only fix (resolve
  RedirTarget::Word through the value plane; gen a per-path file cell; no
  probe/elision story) — sequenced immediately after arch-2 lands (same effect.rs
  surface; serializing avoids a deep merge). If a-2 confirms, y-1 is the fix for a
  latent P1-at-HEAD, not an enhancement.

## imp-2 — tc-M1/h-1 becomes arch-5's ENTRY GATE

r1A: `self_reach_holds` demands an EMPTY suppressed in-state ⇒ any nonempty preamble
refuses every Members license ⇒ partial-member rewriting (arch-5) would polish
machinery the dashboard immediately reports as zero-coverage; the existing L2 green
cases owe their pass to pristine fixture preambles. Adjudication: 20S §3.1's
DOCUMENTED design is cell-family-scoped ("the only writers reaching this site its own
per-member establishes"); if the implementation is global-pristine, that is an
impl-stricter-than-design divergence (safe direction, value-killing). Arch-5's task
now opens with: verify the code; surface the evidence + the cell-scoped lean to the
human; absorb the ruling; re-run the 20S adversarial pins (sibling-writer via
back-edge must still refuse) under whatever scoping lands — BEFORE any list-rewrite
build. (20T's crosscheck validated the rationale possibly against the stricter impl —
the re-scope needs its own hostile eyes, already chartered for arch-5.)

## imp-3 — dashboard adjudication aids (harvest checklist)

(a) A sea-of-red first H2SaLS run is CORRECT behavior (⊤-wall at L38, ~zero honest
elision at HEAD-with-seeds) — grade the dashboard on honesty, not on green.
(b) Two seed pieces are deliberately inert (crond's commented-out `test` resolver;
`test`-keyed query cells dead behind the blessed-pure gate) — must NOT count as
oracle coverage; verify at harvest (the in-flight builder doesn't know this).
(c) The 1AA matrix doubles as a hand-derived golden for the H2SaLS rollup — use it as
the cross-check during harvest. Also verify the dashboard can attribute
"poisoned-by-upstream-refusal" distinctly (imp-5 needs it measurable); if it can't
from public surfaces, that's the top seam-wishlist item.

## imp-4 — cmdsub requirements, corpus-grounded (graft at harvest)

The y-4 partition of H2SaLS's 11 `$()` sites: 4 scratch-path mktemps (ownership
story, not value-prediction) · 3 Query-shaped reads (incl. L38 `$(id -u)` — the
poison wall's ORIGIN, so arch-4's prize addresses the single highest-leverage site) ·
2 pure transforms that should STAY ⊤ · 1 statically foldable. General
substitution-folding is NOT needed for this corpus class. Weigh the in-flight design
note's q-5 recommendation against this partition at harvest.

## imp-5 — refusal-poison vs the north-star ceiling (dq-ledger item)

Every HONEST refusal in a loaded oracle costs all downstream sites their ambience
(refusals land Opaque); until a declare-without-verify cell shape exists (r1A h-3),
loading MORE oracles can REGRESS the dashboard number. Surface to the human alongside
the doors/207 fork discussion once the dashboard makes it concrete — this bounds the
"ceiling = oracle coverage × declaration coverage" claim from below in a way 20V §7
doesn't state.

## imp-6 — small items

L685-695's four if-guarded restarts are real-corpus poles for arch-1's guard class
(the dashboard run exercises them) · the y-7↔y-2 weld dissolves for whatever arch-2
inlines (constrains only havoc-narrowing done without inlining) · the dashboard is
the first consumer loading ALL seeds simultaneously — watch h-2 (provider-collision)
behavior at harvest · cfg.rs:~140-145/198-202 stale "member-elision is future"
comments (r1A h-6): fix in a passing harvest commit.

## meta — fb-7 adopted

"Reconcile by source, not by vote": r1A's crosscheck pair had one direct conflict
where the neutral reading was literally-correct-but-wrong-by-omission, and the
headline finding was adversarial-only — convergence-counting would have mis-resolved
both. Adopted into this round's reconciliation protocol: every applied correction
gets re-verified against the cited source by the reconciler (me), regardless of how
many agents agree.

## Process echo

r1A close state: D1–D4 shipped/finalized on `ai/r1A-H2SALS` (unmerged; our no-merge
decision stands — adapter reads the sibling path). Its pending human-flags (h-1, h-2,
h-5, two welds) are now threaded through the tasks above.
