# 24U — Round-24 close-out: the abandonment accounting, and where every owed item moved

AI-authored (Fable close-out conductor), 2026-07-10. Round 24 closes here **by
reshuffle, not by completion**: its charter (`plans/240`) is mostly discharged, but the
design ground moved out from under the live work twice, and the remaining
reimplementation/respell obligations were re-arced into **round 27**
(`plans/270-round27-charter.md`) rather than finished in-round. This note is the frozen
accounting — what was planned, what landed, what was absorbed beyond the charter, what
was NOT completed and where each such item now lives. Build evidence and residue detail
stay in `notes/24C` (the landing/residue ledger — this note does not duplicate it);
history and settled law stay in `notes/23O`; the forward plan is `plans/270`.

Naming note: this document follows the naming discipline ruled at close (hyphenated
full-word slugs; outside-document references as `docID:slug`; old labels subscripted
once as "né …"). The discipline itself is registered in `plans/270` §1.

---

## §1 — The round in one paragraph

Round 24 opened as the empirical build round ("head off 233 by building something and
seeing what happens", `plans/240`): climb a six-stage ladder to elide-past-a-running-wall
on strawmen, differential-verified, and measure whether the converged-vouch's adequacy
gap actually bites. Stages 1–5 all landed, plus a large halo of unchartered work. The
round was then intercepted twice: first by the language crosscheck arc (`notes/24Kc` →
`plans/24L` → the binding rulings in `notes/24M`), which converted "finish the ladder"
into "respell the corpus under the settled dialect first"; then, late, by three intensive
research rounds — `plans/24R` (secondary positions), `plans/24S` (wrapper contexts / the
context algebra), `plans/24T` (payload decomposition) — whose keystones reshape the
coordinate representation, the probe machinery, and the stdlib's correctness story
fundamentally enough that continuing the old queue would have authored hundreds of files
against a spelling known to be moving. Meanwhile round-numbering went non-monotonic:
r25 (field trial) and r26 (multi-host/wire) both started — as plans, protocols, and
trial tooling — and both are now tabled behind the consolidation round. Round 24 is
therefore closed as **abandoned-forward**: nothing was lost, but the finish line moved
to `plans/270`.

## §2 — Charter discharge (against `plans/240`'s ladder)

- **Stage 1 (yardstick + honest baseline)** — DONE. Silence=wall repaired; the
  differential is the net. Evidence: `24C`.
- **Stage 2 (frame-rule machine: authored footprints × backing × disjointness)** — DONE
  (`24C` §Stage-2, §Stage-2b: the chronology net / `dorc-sweep` with lying-topology
  teeth).
- **Stage 3 (guard tier + claim-tier algebra + elide-weld)** — DONE, in two parts; the
  vouchless-elide gap closed ("a full skip DEMANDS a reached vouch"); 50 fixtures gained
  verdict-functions.
- **Stage 4 (derived footprints, host-run `touches()`)** — DONE; pipes-in-dialect landed
  with it.
- **Stage 5 (grounding + collaboration: `resolve()`/`reaches()`/owncoord; the aliasing
  closure)** — DONE for resolver-bearing kinds; the cross-kind co-reference residue is
  recorded (`24C:strain-coreference-crosskind`), banked post-trial.
- **Stage 6 (maximize, measure, conclude)** — SPLIT. The extract/conclude half is DONE
  (`notes/24O`, 25 dispositions, no 077 constraint retired). The **measure/maximize half
  was tabled and never re-opened in-round** — it moves to `270:block-stdlib` as
  `yardstick-measurement`.
- **The charter's sharpest empirical deliverable — the adequacy bite-rate
  (converged≠no-op)** — **NOT answered.** It was re-homed to the field-trial
  differential (`252` A1), and the field trial is now indefinitely tabled, so the
  answer-date slid substantially. Interim honesty: the in-memory lying-nets
  (static/derived/alias/reach) are the only adequacy-adjacent instrument in existence;
  they test *attribution under lies*, not real-world adequacy. This is the largest
  single open risk carried forward, and `plans/270` names it in the field-trial revival
  conditions.

## §3 — Absorbed beyond the charter (each one line; evidence in `24C` unless noted)

- The **first-contact polish pass** (`24H`: exit-code family, caret frames, `dorc why`,
  positional books, elision-render).
- The **pipe-guard lift** (`24J`: connected read-only check-pipes as one probe; both
  Stage-6-parked forks resolved, per-line dead, filter-blessing a phantom).
- **e2e de-graduation batches 1–2** (154→126 cases, twin-verified; `24I`/`24N`).
- **Wave-1** (Lcg-thinning root-fix; first-wall hint answering `252` B2;
  gating-verification errand; return-decline-inert; munge-reservation lint).
- The **language arc** (`24Kc` → `24L` → `24M` binding rulings: version-comment,
  typeless-floor law, bare `__role` names, reverse-DNS kinds, rungs default,
  governance) and the **respell specimens** (`24P` + the shebang adjudication riding
  `24Q`) — spec-by-example staged, d1–d9 all human-acked (d8 typed 2026-07-10).
- The **kind-owner family design round** (`24G`) and USER_STORY stages 6–7.
- **`24C:rul-ditch-is-diverged`** (né rul24-ditch-is-diverged): the role family is now
  predict / is_converged / touches / resolve / reaches; removal rides the
  corpus-respell.
- The **r25 field-trial package** (charter `250`, protocol `252`, adjudication `254`,
  validated book + dry-run `255`, recon `256`, plus real trial tooling on
  `ai/spike3-r25`: Vultr substrate, ssh-runner, observer, HHHF).
- The **r26 planning package** (`260` multi-host, `261` read-concurrency, `262` build
  spine, `26A` adjudication — architecture cleared, records-lane wire spec rewritten
  per `26A:stop-1`). **Zero build commits exist on `ai/spike3-r26`** (verified at
  close: every r26 commit is design/adjudication and already in r23's history).
- The **three late research keystones**: `24R` (dotfiles + ops siblinghood; the why-run
  impossibility ledger; the sudo-weight flag), `24S` (context algebra — provisionally
  acked for spike experimentation), `24T` (payload decomposition — research-phase
  closed, firming deferred).

## §4 — Not completed → where it moved (the reshuffle map)

Every item below has exactly one new home; the map is the point of this note.

- **corpus-respell** (né LIVING_STATUS queue-item-2; brief = `24P` §2/§3/§5/§7/§8/§9 +
  the `24C` deferral entries) → **`270:block-rebuild`, first stage.** Status change at
  close: it was "dispatch-ready", but it now deliberately WAITS on
  `270:block-settle` — the entity-algebra design note may move the authored mark
  grammar, and the fixture sweep must happen once, against final spellings. The four
  declared-XFAIL failing-spec specimen cases at tip are the staged spec and stay as-is.
- **typeless-floor build** (né queue-item-3; touch-point map `24C` §24L-gating-errand;
  folded riders: quoted-`"$@"` tracer modeling + the founding-one-liner pin from
  `24C:fd-headline-oneliner-gap`; fold `24C:resid-return-arity`) →
  **`270:block-rebuild`.**
- **entity-algebra-rebuild** (né queue-item-3b; ruled by `24C:rul-selector-pre-stdlib`)
  → design note in **`270:block-settle`** (must reserve the `24S:A7` seams:
  context-qualifier in the coordinate representation + per-kind-per-axis declaration
  room); rebuild in **`270:block-rebuild`.**
- **value-recipe-reshape** (né 219 tc-fork-ii / `24T:P-A5`; fragment-preserving,
  cause-tagged ValueOf) → **`270:block-rebuild`**, slotted into the same fact-domain
  churn window as the entity-algebra-rebuild (one merge-pain instead of two; this
  resolves the sequencing `24T` left "deliberately unspecified").
- **wire-records-v1-import** (NEW at close, human-acked 2026-07-10): the bare minimum
  of `262` §2 (framing, terminal token, coordinate-last-to-token, deriv-family
  end-records, additive-keys) pulled forward from r26, because reading host data back
  into the analysis framing is a must for the read-value lane → **`270:block-rebuild`.**
- **e2e-degraduation-remainder** (né `24I` batches 3–4; the ~50 in-memory twins +
  the one-shot `dash -n` net + the `24C:st-1` must-cover; the guard23 no-mint floor
  conversions) → **`270:block-rebuild`, tail** (twins authored once, post-everything).
- **yardstick-measurement** (né Stage-6 measure/maximize) → **`270:block-stdlib`**
  (also produces the may-alias sensitivity number `24O` item-25 waits on).
- **stdlib-authoring** (né P5, ~40 bootstrap oracles) → **`270:block-stdlib`**, with a
  NEW human-ruled blocker added at close: it additionally waits on the wrapper-context
  stages (`270:block-context`) — correct, derived, principled elision of sudo-gated
  commands via the real mechanism is a stdlib precondition, both because a sudo-blind
  stdlib mis-measures the felt product and because `24S` §2c's no-wrapper-awareness
  property is unproven against build contact (authoring 40 oracles before that
  referendum survives is the author-once mistake `24C:rul-selector-pre-stdlib` names,
  one axis over).
- **read-value-lane** (the `$(hostname)` capture fold, `notes/219` q-3; previously
  HOMELESS — fenced out of `24T`, `24S`, and r26 alike, yet existential for the
  dotfiles position per `24R` §1b, the killer of the r25 book's elision, and required
  by the stdlib coverage note) → minted as an owned work-item in
  **`270:block-context`**, gated on the `270:adj-capture-claim` ruling.
- **wrapper-context spike** (`24S` §8 staging W1–W2) and **payload-v1** (`24T` §8, R0)
  → **`270:block-context`.** W3–W4 float behind `270:adj-survival-flag-outcome` and
  `270:adj-trichotomy-spelling`.
- **Smaller residues** — all stay ledgered in `24C` (kill-coherence sweep scenario;
  resolve-derived second round-trip; hint-lane e2e needle; guard23 stale comments,
  which ride the corpus-respell; counter-drift read as landing-time snapshots) and in
  `24O`'s CARRY column (seccomp backstop → real-executor era; pipe residuals → trial /
  post-trial). Nothing was retired at close that `24O` had not already dispositioned.

## §5 — Discharged human-obligations at close (do not re-ask)

- The Stage-6 **spike-health cruft-verdict**: ruled fine 2026-07-07, no death-criteria
  concern.
- The **22H reactivity decision** (né TODO-ADDTL item-1's "DECISION DUE at r24 close"):
  subsumed — r26 was commissioned to build exactly that engine (`260` §2); the plans
  stand even though the build is tabled.
- **KNOBS dated markers** (kOOB closed-set-of-one, kTYANNOT containment,
  kCONTRACT-RUNGS default-pin): landed via the human's own commit `3e76518`.
- **Specimen micro-decisions d1–d9** (`24P` §5): all acked; d8's one-word confirm typed
  2026-07-10 (the improvised otelcol empty-entity bind stays faithful until the
  typeless-floor retires it).
- `24T:P-A4` (carrier quality bar): soft-acked 2026-07-10 (turn08
  opaques8-ack5-quality-bar), with the human's correction on record that in-body guards
  do NOT obsolete the outside-conjunction in general.

## §6 — The tabled rounds (state at close, and what revives them)

- **r25, the field trial** (`ai/spike3-r25`): shape UNCHANGED — `252`'s protocol,
  guardrails, and pre-registered questions all stand. Tabled because the stdlib gates it
  hard and the trial book's two sharpest walls (`su - postgres -c '…'`; the
  `case "$(hostname)"` host-guard) are exactly what `270:block-context` builds. Owed on
  revival, banked here so they survive: fold `255` §5's corrected predictions into
  `252` §4/§8; the human's `_assert_tagged` eyeball before any bulk destroy; the HHHF
  zsh smoke-test; the P4-differential lying-check must-cover
  (`24C` §rc-passthrough-taxonomy); the throwaway ssh key at `~/.ssh/dorc-r25{,.pub}`
  is still on disk (inert; human deletes at will); P2's observer harness lives on the
  r25 branch (salvaged, selftest 8/8).
- **r26, multi-host + wire** (`ai/spike3-r26`): plans + `26A` adjudication only; zero
  build commits (verified). Formally resumes AFTER the field trial, by rebasing onto
  the then-current r23 lineage, minus whatever `270:block-rebuild` already imported
  (the records-lane minimum). Two of its extractables ride round 27 per `262` §7's own
  handoff note: the gate-1 order-insensitive record-compare (cheapest during the
  respell's golden churn) and the additive-keys discipline for any machine-shaped
  surface the round mints.

## §7 — Pointers

The forward arc, block structure, adjudication agenda, and revival conditions:
**`plans/270`**. Build evidence + residue: `notes/24C`. Dispositions: `notes/24O`.
The respell spec-by-example: `notes/24P`. The design keystones: `plans/24R`/`24S`/`24T`.
Settled law: `notes/23O` §2 + `spike/CLAUDE.md`. Live state: `Research/LIVING_STATUS.md`.
