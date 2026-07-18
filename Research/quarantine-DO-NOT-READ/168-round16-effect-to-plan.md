# 168 — round-16 build: the effect → plan elision path (network-free slice)

> **Status (2026-06-05): spike, round-16 build summary.** This round took the
> analyzer from "CFG + oracle types" to a **complete network-free vertical slice**:
> a lazy book's sh → a plan that elides already-converged mutations, behind hard
> type-locks. A standalone, frozen record — notes are append-only; this does NOT
> edit 164 (whose live-state §7 was a one-off; left as historical). Commits on
> `ai/spike-impl`, slice complete at HEAD `38885fa`. Confidence-marked.

## 0. The slice now runs end-to-end
`book.sh` → `syntax::parse` → `cfg::build` → `effect::classify` (per-command
`SkipClass`) → inject a host `PhasedVerdict` per fact → `plan::prove_skippable` →
`plan::render_sh`. No clock / network / disk anywhere (`inv-determinism`); the host
verdict is the single injected seam (the real host / `hostsim` is next). It
demonstrates the core value-prop: analyze a scrappy book, decide skips, emit a
plan-as-sh — e.g. a lone `apt-get install -y nginx` with a Converged probe renders
as `# skip[0]: apt-get install -y nginx` + a provenance line, the rest verbatim.

## 1. What landed (commits)
- **`dcbd03e` effect integrated** — `analysis::effect`: per-command effect lookup
  (`command_effect`) + the **ambient∧invariant gate** (forward reaching-defs over
  the oracle effect-map; the W5 `purge X;…;install X` wrong-skip guard, note 162
  O-1). `classify` → `MustRun` / `EstablishAmbient(fact)` / `EstablishWritten(fact)`.
- **`18ad386` two adversarial fixes** (note 167) — find-A (detached funcdef-body
  establish read as ambient → entry-reachability gate) + find-B (release trusting a
  non-converged solve → fold to `MustRun`).
- **`9d3ee73` engine-wide `meet`** — `Lattice` gained ⊓; a `BoundedLattice` split
  (adds ⊤) makes "must-analysis over a bare `Powerset`" a *compile error* (no
  finite universal set — the ⊤-representability asymmetry note 165 predicted).
  join/meet/absorption laws property-tested across all combinators.
- **`3e927dc` May/Must order-dual wrappers** — `May<L>` (identity) / `Must<L>` (the
  order-dual: ⊥↔⊤, ⊔↔⊓). A *must* analysis is the **unchanged** solver run over
  `Must<L>` — validated meet-at-merges; no engine rewrite (preserves the
  adversarially-validated may path).
- **`03a91f7` plan locks** (note 165 L1/L2) — new `plan` crate: `PhasedVerdict<P>` +
  `Bias` (phase in the type; `Unknown` can't fold to a skip in either phase) + the
  `SkipLicense` witness (private fields ⇒ `prove_skippable` is the *only* mint,
  gated on `EstablishAmbient ∧ Must ∧ Converged`).
- **`38885fa` plan-as-sh** — `build_plan` + `Plan::render_sh`; the leaf-seam (dn-3:
  each leaf a separate `Step` with `LeafId→AstId`, never one `sh -c`).

## 2. Decisions taken this round (the human, via AskUserQuestion 2026-06-05)
- **Build target = the probe/plan elision path** (host verdicts injected; `hostsim`
  later).
- **Orientation lock = engine-wide `meet`** — the maximal-machinery option, chosen
  explicitly as state-space exploration (note 165 §3 calibrate-UP: "my prior lean
  is the FLOOR, not the cap"). Hence `Must<L>` wrappers + a must-capable solver,
  not just elision-path-local tags.

## 3. Surfaced design problems (the deliverable)
- **DP-8 / DP-9 (note 167, from the effect adversarial review):** half-modeling a
  construct (its definition but not its call/handler edges) is *more* dangerous
  than ⊤-rejecting it — every effect-consumer needs the reachability gate until
  Tier-B (DP-8); and solve-convergence is a **per-consumer** obligation, not just a
  producer return value (DP-9).
- **fs-4 on the real fixture (plan):** `apt-get update` is un-oracled ⇒ Opaque ⇒
  poisons the `apt-get install` it precedes (→ `EstablishWritten`), so even a
  Converged probe can't license the skip. Recovering it needs the oracle to model
  `apt-get update` as package-state-*pure* — a precision burden the spike's package
  oracle doesn't carry. The fs-4 precision cost (note 167) made concrete on real code.
- **plan flattening (`render_sh` limitation):** leaves are emitted source-ordered
  without reproducing their `if`/`case` guards — the plan shows mutator dispositions,
  not a runnable control-flow rewrite. The leaf-seam / wo-1 provenance tension made
  concrete; a faithful in-place rewrite is a later refinement.
- **the ⊤-representability asymmetry (lattice):** a *may* domain needs only ⊥; a
  *must* domain needs a representable ⊤, which a powerset/map over unbounded
  elements lacks. Now a type-level fact (`BoundedLattice`). A real constraint on
  which must-analyses are expressible without an explicit-top domain.

## 4. Crate state (network-free kernel; every crate green + clippy-clean)
`core` (vocabulary) · `syntax` (parser) · `analysis::{lattice (meet + May/Must),
solve (hardened + must-runnable via the dual), cfg (precise errexit, note 166),
effect}` · `oracle` (the lift) · `plan` (the elision locks + plan-as-sh). Test
counts: core 5 · syntax 2+16 · analysis 18+23 · oracle 8 · plan 8.

## 5. Next (still network-free)
`hostsim` (DST) — a **seeded state-machine host** that answers fact-probes
deterministically against a modeled system-state (replacing the injected
`verdict_of`) AND **detects a probe attempting a modeled mutation** = the
kFAIL-withhold check (note 162 DP-4; the spike stand-in for the real seccomp /
sandbox). No async; a seeded PRNG, injected, is the one place nondeterminism is
allowed (`inv-determinism`). Then a thin `cli` wiring the pipeline and printing the
plan.

**NOTES INDEX:** 160 chord-synthesis · 161 dn-1 strawman · 162 dn-1 reconciliation
(fact-centric pivot) · 163 SPA engine · 164 (frozen checkpoint; do not edit) · 165
orientation-lockdown · 166 CFG errexit · 167 effect adversarial review (DP-8/DP-9)
· 168 (this — round-16 build summary).
