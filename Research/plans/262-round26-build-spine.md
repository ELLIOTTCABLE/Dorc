# 262 — round 26 build-spine: the shared substrate under 260 (fleet) and 261 (read-concurrency)

AI-authored (Fable, design-synthesis session), 2026-07-07. Extracted at human direction from
`plans/260` + `plans/261`: the two tracks share a build-spine — the records lane, the emission
locus, the order-independence invariant, the policy ports, and the determinism rig — and this
document IS that spine. 260 and 261 keep the shape/reasoning/tradeoffs/gotchas of their halves
and now reference this doc for the shared substrate. **Build order: this doc's S0/S1 first;
the two tracks decouple after S1.**

**The organizing ruling (human, 2026-07-07):** bake in early the *ability* to re-order reads
for performance; defer everything about *how* to re-order (tuning, cost models, feedback,
caches). Endorsed, with the one guard that makes the deferral safe (§1): ordering policy is
bolt-on-able later *precisely because* it is observationally invisible — and that invisibility
is a maintained invariant, not a given. Corollary ruling, same date: **cross-run re-ingest of
timing (or anything else) is DEFER, full stop** — whatever someday re-orders things can change
later and must stay invisible to the user, so it bolts onto the §4 cost port without touching
the spine. (The rec-5/kSTATE fence question that a timing cache would raise is thereby parked
un-asked; `plans/261` §9 keeps the precise wording for whenever the punt is lifted.)

---

## §1. The central invariant — spine-inv-order-free

> **No scheduling, interleaving, arrival-order, width, or placement choice may change the
> CONTENT of any record, fact, plan-verdict, plan, or artifact — only its timing.**

This is the load-bearing sentence of the whole round. Everything the two plans defer
(LPT-vs-anything, cost tiers, width defaults, speculation policy, pools, caches, serial/canary
admission) is deferrable *because* this invariant makes every such policy observationally
invisible: swap the policy, byte-identical plans fall out, only the wall-clock moves. The
moment anything reads *meaning* from order, the bolt-on-later license dies silently.

What already guarantees it (existing law, now consumed as spine):
- the fold's same-cell merge is commutative + idempotent (`22H` §3) — accumulation order is
  free within a host;
- per-host accumulators are disjoint (`260` §3 s3-1) — accumulation order is free across hosts;
- records are leafid-keyed self-describing lines — no positional meaning;
- terminal determinism (`260` law-determinism-acceptance): incremental-final ≡ single-shot.

What must be actively KEPT true (the spine's pins — S1 deliverables):
- **pin-fold-permutation** — `fold(any permutation of records) ≡ fold(book order)`, property-
  tested on generated record sets (was `261` §7; lives here — both tracks rely on it).
- **pin-terminal-determinism** — per host, arrival-incremental final plan == single-shot plan
  (was `260` acc-terminal-determinism; both tracks rely on it).
- **pin-no-order-keyed-consumers** — an audit obligation, not a test: any consumer that
  byte-compares record *sequences* must either fix order at the producer (width=1) or compare
  order-insensitively. Known instance: e2e gate-1 parity (byte-compares probe output against
  authored `probe-results.txt`) — resolved by running harness probe executions at width=1
  (the flag default) plus the dedicated jitter family (§5) that asserts set-equality at
  width>1. Any future consumer of the records lane inherits this obligation.
- **pin-late-and-alien-records** — records after the sentinel (escaped grandchildren) and
  non-nonce lines (freeform leakage) are discarded + counted + warned, never folded — so
  hostile/sloppy bytes cannot convert into order- or content-effects.

The failure mode to police in review, forever: "first record wins", "last write wins",
order-keyed goldens, or any tie-break that consults arrival sequence instead of leafid/content.

---

## §2. The records-lane contract — `dorc-records/1` (one spec, all consumers)

The complete wire spec both tracks build against (supersedes the split across `260` §4 and
`261` §5/§6; rationale and tradeoff records stay in those docs — `260` dec-26-wire-v1,
`261` dec-261-ms-field).

```
dorc-records/1 nonce=<nonce> host=<hostid> book=<sha256-of-book-bytes> sites=<N>
<nonce> site 0 <inner record grammar, unchanged, owned by the existing emitter/parser>
<nonce> deriv 0 coord=<kind:entity>
...
dorc-records-end/1 nonce=<nonce> seen=<K>
```

- **Framing:** header line first; end-sentinel last, emitted **after the artifact's final
  `wait`** — the drain NEVER keys on EOF (`notes/141` g5, the fd-inheritance hazard). Every
  record line carries the run-nonce prefix; the nonce is minted at the controller edge
  (DI'd — never ambient RNG in kernel code).
- **Integrity keys:** `book=` binds the stream to the exact analyzed book bytes (mismatch ⇒
  refuse fold — discharges the r22 `tc-probe-no-digest`/`tc-probe-results-roundtrip` items);
  `host=` must equal the session's expected HostId (mis-plumbed streams refuse — the
  partition law's wire tripwire); `sites=` declares the expected census so truncation is a
  *computable range*, not a guess.
- **Truncation semantics (consumer rule):** absent sentinel, or `seen < sites` ⇒ received
  sites keep their facts; the un-received site set folds Unknown ⇒ run, and the render marks
  the boundary (`plans/128` fc-2 at the probe lane).
- **Line atomicity (producer rule):** one record = ONE `printf` = one line, well under
  `PIPE_BUF` (≥512 bytes POSIX-atomic on pipes — `notes/141` g2). This is what lets
  concurrent within-host lanes and any future channel-mux share the lane without tearing.
  Stated in the artifact header comment; it is load-bearing, not style.
- **Additive-keys policy (versioning discipline, declared while the surface is one round
  old — the `24Kc` cluster-compat lesson):** parsers of `dorc-records/1` MUST ignore unknown
  `key=val` pairs on any line. New fields (e.g. `261`'s `ms=`) are additive within /1;
  a /2 bump is reserved for structural changes (framing, keying, semantics of existing keys).
- **Alien/late lines:** per pin-late-and-alien-records (§1) — discard, count, one aggregated
  warning per host.
- The **inner** `site N effect=… rc=…` / `deriv N coord=…` grammar is deliberately NOT owned
  here — it belongs to the existing emitter/parser pair and is expected to move with the
  entity re-key (`cli/CLAUDE.md` ap-1); the framing is agnostic to it by design.

---

## §3. The emission locus (the one kernel-adjacent edit, shared by both tracks)

All spine changes to the probe artifact land as **one bounded edit site** at probe-artifact
assembly, flag-gated:

- **framing** — the header/sentinel/nonce-prefix `printf`s (260's need);
- **per-task subshell isolation** — each probe-task becomes `( <invocation>; <record printf> ) &`
  granting private `_rc`/`_e`/oracle-locals/cwd/umask (dissolves `261` §2 h4 — the shared-
  scratch race verified in the incumbent);
- **wave barriers** — tasks launch per-wave with `&`, bare POSIX `wait` between waves;
  sentinel after the final wait;
- **the width flag** — `--probe-width` / `DORC_PROBE_WIDTH`; **width=1 emits today's
  byte-identical serial artifact** (modulo framing lines once S1 lands); `--faithful` forces
  width 1 + book order (`kFIDELITY`; `plans/142`).

Everything cognitive stays OUTSIDE this locus in new pure modules: the scheduler
(`261`/port-schedule), the classifier (`261` t1), the fleet kernel (`260`). The locus itself
is a dumb renderer of a schedule handed to it.

Golden posture (updated per human, 2026-07-07): r24's language work will churn goldens
massively anyway, so byte-stability-vs-the-sibling is no longer the *primary* motive; the
width=1 default is RETAINED regardless as (a) determinism insurance for the harness, (b) the
`--faithful` floor, and (c) a clean A/B lever for the `261` P4 yardstick. The width-default
flip remains a deliberate later decision package with its one-time re-bless.

---

## §4. The policy ports (where all deferred tuning bolts on — the "ability, not tuning" made structural)

Three signatures, stable from S0, each with a trivial v1 implementation. Swapping any
implementation is invisible by §1 — that is the whole design.

- **port-schedule** — `schedule(tasks, edges, atomic_units, K, cost) → waves×lanes`. Pure,
  deterministic (ties by leafid). v1: single antichain (today's probe set — the `261` §1
  finding), constant cost ⇒ arbitrary-deterministic placement. Later: LPT, class segregation,
  resource-keys, maintain-cfg subgraphs — all inside this function (`261` §3–§4).
- **port-cost** — `cost_estimate(task) → rank`. v1: constant. Later: t1 body-inferred class →
  t2 measured `ms=` → (deferred, human-gated) cross-run profile. The re-ingest DEFER ruling
  lives at this port: nothing upstream of it knows where estimates come from.
- **port-admit** — fleet-side `admit(phase, state) → Vec<HostId>` (`260` §3 s3-5). v1:
  open-cap + width-cap. Later: serial/rolling/canary (`kSCHEDULE` seam) — same signature.

Port discipline: no port implementation may read anything but its declared inputs (in
particular: no clock, no ambient config, no cross-run state — the DI law); every port
implementation is exercised under the §5 rig by construction.

---

## §5. The determinism rig (one harness discipline, both tracks' tests ride it)

- **Seeded variation, two axes:** the fleet-event interleaver (`22H` §2's arrival-ordering
  seam — a seeded logical clock over per-host event streams; `260` §7) and mock-duration
  jitter (`DORC_MOCK_JITTER_SEED` in the e2e mock shims — `261` §7). Both vary *timing only*;
  §1 says outcomes must not move — which is exactly what the rig asserts.
- **The guard:** rerun any seed ⇒ bit-identical command trace + final plans (`24B`
  C-determinism-guard; `260` acc-seed-bit-identical). Proven at S0 by deliberately breaking
  it once (inject an order-observable HashMap iteration; watch red; remove).
- **an-sometimes-assert** on every injected fault path (`plans/128` fc-5; hostsim
  discipline) — reachability half only; coverage humility inherited.
- **The jitter e2e family:** width>1 probe artifacts under several jitter seeds ⇒ identical
  record SET + identical final plan (order-insensitivity made executable at the artifact
  tier, complementing the in-memory permutation pin).

---

## §6. Spine build stages (before either track proceeds)

Gates as `260` §8 (fresh build · clippy `-D warnings` · suites · existing e2e byte-stable ·
the stage's pins green).

- **S0 — skeleton + rig + ports.** Crates `fleet` + `transport` exist (workspace members);
  event/command vocabulary compiles; the three port signatures exist with v1 trivial
  implementations; `schedule()` property pins (every task exactly once; atomic units intact;
  no consumer at-or-before its producer's wave; deterministic; faithful ⇒ book order — from
  `261` §7); the determinism guard proven. *(Absorbs `260` stage-26-0 and `261` P0.)*
- **S1 — the records contract + the emission locus.** §2 implemented end-to-end: framing
  emission + parser (refusals: book/host mismatch; truncation range; alien/late discipline);
  the §3 locus with subshell isolation + waves behind the width flag (default 1); gate-1
  harness answer (width=1) + the jitter family at width 4; pin-fold-permutation +
  pin-terminal-determinism landed as tests. *(Absorbs `260` stage-26-2's wire half and
  `261` P1.)*

**After S1 the tracks decouple:** `260` stages 26-1/26-2(transport half)/26-3/26-4/26-5
(fleet kernel semantics, ssh drivers, apply fan-out, verify, measurement) and `261` P2/P3/P4
(classifier + LPT, `ms=` telemetry, the makespan yardstick + decision packages) proceed in
either order or in parallel — they meet only at the ports and the records lane, both frozen
by S1.

---

## §7. Merge-disjointness (shared rules; per-track specifics stay in 260 §10 / 261 §10)

- New cognitive code = new files/modules only (fleet, transport, scheduler, classifier).
- The §3 emission locus is the ONE shared kernel-adjacent edit — both tracks' emission needs
  land there together, once, flag-gated.
- Parser changes are additive (framing wrapper + discard rules; inner grammar untouched).
- Existing e2e cases + goldens byte-stable while width defaults 1 (now insurance, not the
  primary fence — §3 golden posture).
- `core` untouched; fleet-level types live in `fleet` until after the merge.
- **Handoff note for the r24 implementor sibling (human is passing these plans over):** the
  spine pieces most plausibly extractable or collision-prone for language-side work are (a)
  the §2 additive-keys/versioning discipline — the same channel-stability pattern `24Kc`
  cluster-compat demands for the `# dorc:` trailer and any machine-shaped surface r24
  touches; (b) the §3 emission-locus discipline (one bounded edit site, cognition outside) if
  r24 restructures artifact rendering; (c) gate-1's order-sensitivity (pin-no-order-keyed-
  consumers) if r24's golden churn rewrites the parity harness anyway — cheapest moment to
  make the compare order-insensitive is during that churn.

## Cross-refs
`plans/260` (fleet/transport: kernel semantics §3, ssh §5, CLI §6, fleet DST §7, dec-26-*) ·
`plans/261` (read-concurrency: dependence theory §2, schedule model §3, cost tiers §4,
dec-261-*) · `plans/142`/`notes/140`/`141` (transport law; PIPE_BUF; EOF hazard) ·
`plans/128` (Seam-1, fc-2, fc-5) · `notes/22H` (fold reuse, arrival seam, terminal
determinism) · `notes/24B` §5 (determinism guard; coordination-DST reservation) · `24Kc`
(cluster-compat → §2 versioning policy) · KNOBS: `kFIDELITY` · `kFAIL` · `kSCHEDULE` ·
rec-5/`kSTATE` (the deferred re-ingest's eventual fence).
