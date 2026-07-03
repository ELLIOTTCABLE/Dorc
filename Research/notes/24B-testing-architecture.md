# 24B — the testing architecture for the round-24 arc (three flavours; the sweep's altitude)

AI-authored (Fable conductor), 2026-07-03, round 24. Design deliverable answering the
human's three testing-protocol questions (three-or-four flavours; is the huge-corpus
material filed under e2e or standalone; hostsim altitude) — decided under delegated
judgment while the human slept, so: confidence-marked, and the load-bearing calls are
flagged for a waking review. Binds tasks #9 (chronology net) and #10 (battle-oracle suite)
and every later stage's test coverage. Companion to `24A` (rulings/evidence);
`24A §1e`/`§1f` are the commissions this note architects.

## TL;DR (the three answers)

1. **Three flavours going forward** (+ one legacy item being subsumed). Named + bounded in
   §1. The number that matters is the *filing rule* (§1's "MUST NOT absorb" lines), not the
   count — the human's real worry is the e2e suite's cost leaking into the new coverage.
2. **The battle-oracle corpus is STANDALONE fixtures, not filed under e2e.** It lives as
   plain `.sh` fixture material driven PRIMARILY by the in-memory sweep (flavour C), and
   contributes only a SMALL handful of rows to the real-path corpus (flavour B). §2.
3. **The sweep is a NEW crate (a composition-root altitude), not more hostsim.** hostsim
   stays the host-*model*; the harness that drives the whole kernel against that model
   moves up one level. §3. ~SUSPECT this is the single highest-value structural call in the
   note.

## §1. The three flavours (going-forward), by essential properties

The honest 2×2 that generates the taxonomy: axis-1 **real-execution-path** (subprocess →
emitter → dash → artifact) vs **kernel-only** (in-process; apply dispositions to models,
never shell out); axis-2 **authored-narrow** vs **generated-broad**. Three cells are worth
maintaining; the fourth is legacy.

**Flavour A — KERNEL tests** (exists; keep). Kernel-only × authored-narrow, asserting
analyzer *structure* (this classifies `EstablishAmbient`; this lift resolves this argv;
this witness mints iff…) + the round-24 **type-contract properties** (the tier algebra's
compile-fail/constructor pins). Home: `#[test]` in each crate + `crates/<c>/tests/`. Fast,
pure, deterministic. *Sees:* internal invariants. *Can't see:* end-to-end behaviour, host
interaction over time. *MUST NOT absorb:* nothing — this is the floor.

**Flavour B — the CORPUS** (exists as `e2e/run.sh`; keep, do NOT grow beyond its purpose).
Real-path × authored-narrow. Subprocess-drives the real `dorc`, executes rendered artifacts
under inert mocks (`dash -n` + exec), gates the artifact's runnability, run-set, ordered
execution, the stderr surfaces (plan-summary/why-lens), the yardstick rows, and the gate-5
dash-differential (value-flow ↔ real dash agreement). *Sees:* the WHOLE real pipeline
including render/emit/dash-semantics — the failure classes the spike has actually hit
(ap-2 empty-clause, `${N#pat}`/prefix-env value-flow bugs). *Can't see:* END-STATE outcomes
(its mocks are state-independent — running a command changes nothing), interference over
evolving time, breadth. *Cost:* slow (~2 min at 135), file-per-case, structurally brittle —
**inherent to true e2e, and the reason it MUST NOT be the home for hostsim-shaped stories.**
Grows ONLY for what genuinely needs the real path: yardstick rows, emitter/artifact
honesty, and a small handful of state-bearing "tie-down" cases (§3).

**Flavour C — the SWEEP** (NEW; tasks #9/#10). Kernel-only × generated-broad. Per seed:
generate a scenario, run the real kernel pipeline *in-process*, evolve two host-model copies
(bare vs plan), assert **end-state equality** + **attribution-under-lies** across
interference topologies, at thousands of seeds/second. *Sees:* outcome-correctness across
evolving time + interference, at generated adversarial breadth, with the declared/ground-
truth split that makes lying footprints (and thus blame-assertion) expressible. *Can't
see:* the render/emit/dash path (it never shells out — it applies dispositions to models).
*MUST NOT absorb:* the real-path coverage — that stays flavour B's job, tied to C by §3's
handful of tie-down cases so C's kernel-only view is trusted, not assumed.

**Legacy — the SUBPROCESS DIFFERENTIAL** (`hostsim/differential.rs`; real-path ×
generated-broad; the 4th cell). It drove the real binary across seeds asserting *run-set*
soundness. Flavour C **subsumes** it (end-state equality strictly dominates run-set;
in-process is far faster) except on the real-emit/dash axis — which flavour B already pins
at authored breadth, and which generated-breadth rarely adds to. Disposition: **FREEZE**
(don't grow it, don't add topologies to it) and route all new generated-breadth to C;
**defer deletion to Stage-6 cleanup** (deleting a green harness mid-round is churn). ~SUSPECT
freeze-not-delete is right; a waking human may prefer delete-now — cheap either way.

So: **three flavours to maintain (A/B/C), one legacy to freeze.** "Three or four" resolves
to *three going forward.*

## §2. Filing the battle-oracle corpus (Q2): STANDALONE, not under e2e

The battle-oracle material (≥150-line oracle + hostile playbook + cooperating shims) is
BIG. Filed as e2e case-dirs it inherits exactly the file-per-case slowness/brittleness the
human flagged. So:

- **Primary home: standalone `.sh` fixtures** (a `fixtures/` tree of real oracle + book +
  shim files), consumed by flavour C — the sweep drives them across seeded world-states
  (fast, no per-case-dirs, the realism exercised at breadth over evolving chronology).
- **Secondary: a SMALL number of flavour-B rows** — its headline yardstick numbers (the
  most honest numbers in the family) + one artifact-runnability pin (prove the real emitter
  handles a 150-line oracle and a big real playbook's rendered artifact actually runs under
  dash). A *handful*, never dozens.
- The 17x/15x strawmen (census, `24A §1f`) seed the playbook (books nearly dialect-free —
  the expensive half, already written); the strawman oracles respell into the cooperating
  shims; the ONE battle oracle is authored fresh to current dialect. (H2SaLS lift stays
  human-keyed/non-blocking.)

## §3. The sweep's altitude (Q3): a new crate; hostsim stays the model

The load-bearing structural call. The sweep needs to compose {syntax, analysis, plan,
oracle, hostsim, core} — a **composition-root** altitude. Today `differential.rs` gets away
with living in hostsim only because it shells out (links no kernel crate). Going in-process
makes the composition explicit, and it wants its own home:

- **hostsim stays the host-MODEL** (its charter — "the sole home of nondeterminism," the
  Seam-1 stand-in). It keeps `Host`, the seeded `Lcg`, the kFAIL-withhold monitor, and
  seeded initial-state generation. One SMALL addition at the right altitude: a concrete
  **apply-a-cell-delta** op on `Host` (the model must be able to enact a ground-truth
  effect on its cells) — host-model-shaped, so it belongs here.
- **A NEW crate — suggested `sweep` (builder's call; low lock-in)** — is the harness. It
  depends on the kernel + hostsim and holds: `gen` (the **scenario generator** — book sh,
  oracle-set, initial state, AND the ground-truth effect-model, with declared-vs-true
  divergence controlled independently: the lying-footprint capability); `drive` (run the
  real kernel in-process; evolve the two `Host` copies — bare applies every mutator's
  ground-truth delta in book order, plan applies only `Run`-disposition sites' deltas);
  and the assertions (end-state equality; blame-under-lies). Reuses hostsim's `Lcg` for
  entropy (don't fork a second PRNG — keep nondeterminism single-homed). `inv-determinism`
  holds: same seed → same trial, replayable, seed surfaced on failure (L0).
- **The declared/ground-truth split's altitude** (the subtle bit): the generator invents,
  per generated command, a `(declared_claim, true_effect)` PAIR and the divergence between
  them (new-crate altitude — it's the scenario's invention); `Host` merely *applies* a
  concrete `true_effect` to its cells (model altitude). This split is what lets the same
  sweep test **soundness-under-honesty** (no divergence ⇒ end-states must match, else engine
  bug) and **attribution-under-lies** (divergence ⇒ end-states may differ ⇒ assert the
  witness/why-lens blames the *actual* liar) — the property the human's testing-refinement
  unlocked (`24A §1e` addendum).

Why not put the harness in hostsim: it would bloat the lean host-model into a heavy
composition crate and muddy the prized "kernel depends on none of hostsim; hostsim is just
the injected model" layering. A new crate keeps every altitude clean. (The frozen legacy
`differential.rs` may migrate up to the new crate at cleanup, or die — Stage-6 call.)

## §4. What each task implements against this

- **#9 (chronology net) = flavour C's core**: the new `sweep` crate (gen/drive/assert), the
  interference-topology axis with per-class sometimes-asserts, end-state + blame assertions,
  the `Host` cell-delta op. Plus the couple of flavour-B **tie-down** cases (state-bearing
  mocks) that cross-check C's in-memory model against real dash on a few chronology inputs.
- **#10 (battle-oracle suite) = §2's material**: standalone fixtures (sweep-driven) + a
  handful of flavour-B yardstick rows + the runnability pin. Built on #9's sweep.
- **Every later stage** inherits the filing rule: new coverage is flavour A or C by default;
  flavour B only when the real emit/dash path or a yardstick row IS the point.

## §5. Reconciliation against the round-12 DST conclusion (`plans/128`; human-prompted "do DST well")

Read the whole round-12 DST/CI round (`plans/128` + `notes/120`–`127`) before finalizing —
DST is powerful but a footgun, and this round already mapped the footguns. It is
authoritative and current (2026-06-03; cites the canonical sources — FDB, madsim, eatonphil,
Antithesis, TigerBeetle-VOPR, sled). **Judgment: no fresh external search warranted** — the
round covers the footguns, and the one thing it flags UNSOLVED (state-space coverage) is
unsolved industry-wide. Three corrections to §§1–4 above, and confirmations, fell out:

- **CORRECTION C-altitude (the load-bearing one). Two DST targets exist; the chronology net
  is the NARROWER one — name it honestly.** The round-12 canonical seam is
  `ship(host,unit) → results` — the controller↔host *transport STREAM* (Seam-1), tested by
  synthesizing the result stream from a seed under network faults
  (drop/timeout/partition/truncate/reorder), asserting the orchestrator records **Unknown**
  for leaves past the last marker, retries don't double-apply, provenance survives (`128`
  se-2 "CAN", fc-2/fc-4). **That is the multi-host COORDINATION story — it needs the
  orchestrator that does not exist in the spike yet (the round-25 reactivity/`22H` work),
  and rides the transport-stream seam.** The **chronology net is a DIFFERENT, narrower
  thing: ELISION-SOUNDNESS at the FACT-VERDICT seam** (does the elided plan reach the bare
  book's end-state), consuming `hostsim::Host` as the Seam-1 *verdict* stand-in (a level
  BELOW the result-stream — correct, because the analyzer consumes fact-verdicts). So: do
  NOT call the net "the DST"; it is elision-soundness DST. RESERVE-NOT-COLLIDE: the net
  rides Host as the verdict oracle; round-25 adds the transport-stream seam + fault
  injection ON TOP; they compose (shared Host), never collide. `24A`'s deferred-work ledger
  (22H reactivity) is where the coordination-DST lives.

- **CORRECTION C-determinism-guard (a real gap I'd have shipped). Add the "is it actually
  deterministic?" guard** (`128` fc-5; `notes/123` f20 — madsim `MADSIM_TEST_CHECK_
  DETERMINISM`, S2's rerun-seed-diff-TRACE "down to the last byte"). §1-C had the
  sometimes-asserts (the *reachability* half of the "is my DST working" defense) but MISSED
  the other half: rerun a sampled/failing seed, assert the trial is BIT-IDENTICAL
  (plan, S_bare, S_apply, witness). For a pure kernel any divergence is a real bug (an
  observable `HashMap` iteration, a non-`BTree` order leak — exactly `inv-determinism`'s
  ban). Cheap, and the single highest-value guard in an ~89%-agent-implemented codebase
  because agents refactor blind and "seeds break on code change" (`128` rg-1).

- **CORRECTION C-coverage-humility. The sometimes-asserts are the reachability half;
  coverage is UNSOLVED** ("resembles science more than engineering", `128` fc-5/rg-1). The
  net HUNTS bugs, never PROVES their absence (Jepsen's "presence not absence"). §1-C must
  not read as "the sweep proves elision soundness" — it raises confidence at breadth, no
  more. Inherit the humility.

Confirmations (my §§1–4 instincts, now grounded, not assumed):
- **Approach-#3 fit** (`notes/123` f18/f23): the net IS state-machine DST — a near-pure
  kernel has no transitive nondeterminism to police; the seeded `Lcg` is the sole entropy;
  divergence is injected by the generator (declared-vs-true), not by faulting a real socket.
  **Consequence worth stating so the builder doesn't cargo-cult: NO madsim, NO tokio-shim,
  NO libc-overrides — and therefore the net DODGES the `axis-platform` Mac/Windows
  determinism trap entirely** (`notes/123` f19). It is trivially cross-platform, unlike
  general Rust DST. (This is why se-3 calls DST "unusually cheap for Dorc.")
- **Rigor placement** (`128` rg-2): elision-soundness IS kernel correctness — the right DST
  target (kernel gets provable-grade rigor; the mocked edge stays oracle territory,
  best-effort). The net's focus is validated, not incidental.

Forward hooks (round-25, named so they aren't lost — explicitly NOT this arc): the
three-valued-verdict-under-truncation test (`128` fc-2) and retry-safety/no-double-apply
under Unknown (`128` fc-4, idempotency-as-test-oracle) live at the transport-stream seam
with the coordination orchestrator. Builder source pointers: `notes/123` f18 (state-machine
DST) + f20 (determinism guard); `128` se-1/se-3 (the seam + why cheap), rg-1/fc-5 (the
footguns).

## Confidence

+SURE: the 2×2 is the real taxonomy; B must not be the home for hostsim stories (the human's
explicit cost concern); the battle-oracle is standalone fixtures; the C-altitude and
C-determinism-guard corrections (grounded in `128`). ~SUSPECT: the new-crate altitude for
the sweep (vs extending hostsim) — high-value if right, cheap to relocate if the builder
finds a dependency knot. -GUESS: freeze-not-delete for the legacy differential (a waking
human may just delete it). The crate *name* `sweep` is a placeholder.
