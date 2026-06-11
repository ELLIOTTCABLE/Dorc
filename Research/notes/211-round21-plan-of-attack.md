# 211 — Round-21: plan-of-attack, slug reservations, baseline

> Orchestrator note, round-21 top-level. The charter is the human's round-21 priming
> prompt (quarantined; its arcs are restated here so no other agent needs to read it).
> Append-only; later chunks get later notes.

## §1 Charter restatement (arcs + acceptance, terse — the quarantine-free copy)

- **arch-1 leaf-exact render** (lands before anything render-touching): Replace
  substitutes the leaf's exact byte-span; the carve-out family (T14 case-arms, F2
  scaffolding lines, group-closer) and its detection machinery retire — deleted, not
  bypassed; `Channel::StatusRenderFloor` retires or demonstrably narrows; the lone
  if/elif guard becomes an ordinary guard-capable substitution = a NEW elision class,
  pinned at both poles. Acceptance: zero *semantic* golden churn outside
  newly-expressible cases; 20R/20M floor tests re-grounded; 20O find-6 latents +
  20S §10 hunt-4 re-checked against the new render.
- **arch-2 budget-bounded function inlining** (brk-2): bounded (size/depth budget, no
  recursion — recursion ⊤-rejects loudly), uniform books+oracles, positional-param
  binding through the existing value plane, over-budget ⇒ Opaque-with-diagnostic
  (proportional degradation, never a cliff). Unlocks 207 wrapper-pun mechanism; the
  207 policy fork stays the human's. Hostile crosscheck MANDATORY before dependents.
- **arch-3 errexit doors** (PRODUCT-VIABILITY keystone; charter `plans/20V`):
  (a) door-3 rc-deadness early and independent, both poles pinned; (b) door-1 cascade
  verification (guarded blocks fold whole, restart elides as dead control-flow;
  post-arch-2 the wrapper form); (c) door-4 guard-insertion as a NEW license category
  (guard-insertion, NOT observable-reproduction relaxation), mintable only with the
  kind's door-2 converged-run declaration AND non-Status channels passing existing
  gates; rides arch-1, lands after arch-2; declaration spelled acceptable-debt inline
  (kTYANNOT precedent); precedence policy (admin-explicit > oracle-default >
  engine-conservative) behind a policy-seam in ONE module — human rulings land
  mid-round and must hot-swap. Hostile crosscheck mandatory on (c). dq-errexit-1/2/3
  are the human's: surface, never settle.
- **arch-4 command-substitution first slice**: at minimum honest, specific
  ⊤-diagnostics everywhere `$()` appears (no silent phantoms); the prize, if the
  design holds, a Query-shaped `$()` (a `$()` site is a site in the probe-results
  lane). Design-first: a short note BEFORE building, flagged to the human if seams
  resist.
- **arch-5 partial-member list-rewriting** (tc-l2-member-list-not-rewritten): rewrite
  the for-list to diverged members only — first render that CHANGES a loop header;
  rides arch-1; own hostile pass (20T did-not-survive list = attack prior-art).
- **arch-6 H2SALS coverage dashboard** (exists by round-close): re-runnable report,
  per command-site: analyzable-without-⊤ (which triggers) · oracled? ·
  probed-converged? · elided THROUGH WHICH DOOR (fold / dead / guard-transform /
  static-declared / runs); count- AND criticality-weighted (1A matrix when it lands;
  line-count stand-in). Adapter seam for `.claude/worktrees/ai-r1A-H2SALS` artifacts;
  never edit that tree; never block on it. Runs in the gate set without being a gate.
- **arch-7 (stretch)**: hostsim DST at scale — seeded-random book/oracle generation
  through the gate-5 differential harness.
- North star (derived, never a target): ~80% criticality-weighted non-trivial elision
  coverage of the H2SALS rewrite on a converged host; guard-transforms count but are
  reported as their own column, never blurred with full elisions; ceiling = oracle
  coverage × declaration coverage, NOT guard-idiom density (must not game) and NOT
  raw engine quality. Per-door attribution is what makes the number decomposable.

## §2 Baseline at round-open

HEAD `f09ebd7`, verified by orchestrator (verify-don't-relay): `cargo fmt --check` ·
`clippy --all-targets -- -D warnings` · `cargo test --workspace` · `sh e2e/run.sh` ×2
(66/66, zero xfail, all six gates) · `typos` — all green. A recon agent reported
"62 e2e cases"; run.sh says 66 — the count-the-dirs rule reaffirmed; trust the
harness, not the survey.

## §3 Wave plan (adaptive; dependencies are the spine, not the calendar)

- **w-0 (done)**: baseline verify; two read-only recons — render+channel surface
  (arch-1 seed) and function/value surface (arch-2 seed). Reports held in
  orchestrator context; load-bearing facts fold into briefs.
- **w-1**: door-3 build SOLO (touches `analysis/cfg.rs` marking + `plan` consumption
  gate — small, lands first for the corpus's sake). Meanwhile orchestrator drafts
  arch-1 + arch-2 briefs.
- **w-2**: arch-1 build SOLO (render + Channel surface; too entangled with cfg.rs to
  pair with arch-2).
- **w-3**: arch-1 hostile crosscheck (read-only) ∥ arch-2 build (analysis surface).
- **w-4**: arch-2 hostile crosscheck ∥ door-1 cascade corpus cases ∥ arch-4 design
  note.
- **w-5**: door-4 + door-2 + precedence-seam build (needs arch-1 landed, arch-2
  crosschecked); then its mandatory hostile pass ∥ arch-5 build prep.
- **w-6**: arch-5 build + its hostile pass ∥ arch-6 dashboard build (disjoint
  surface).
- **stretch**: arch-7.
- Concurrency rules inherited (20U §5): disjointness must include `target/` and the
  e2e tree; BLESS exclusive, orchestrator-only, quiesced tree, diff inspected
  case-by-case; case-name prefixes reserved per task (door3-*, door1-*, render-*,
  inline-*, members-*, dash-*).

## §4 Note-slug reservations (reserved at dispatch, per protocol)

211 this note (orchestrator) · 212 reserved: recon-digest durables if needed ·
213 door-3 build (builder) · 214 arch-1 design+build · 215 door-1 cascade cases ·
216 arch-2 design+build · 217 crosscheck reconciliation, wave-1 (arch-1/arch-2) ·
218 door-4/door-2/precedence build · 219 arch-4 cmdsub design note · 21A arch-5 ·
21B arch-6 dashboard · 21C crosscheck reconciliation, wave-2 · 21D+ overflow.

## §5 Crosscheck budget

Mandatory targets: arch-1 (render rebuild = highest golden-churn risk), arch-2
(before dependents — charter), door-4 (four-world trace + disclosure floor —
charter), arch-5 (charter). Target ~25–30% of build spend (~200k/pass, 4–6 passes;
20U: two passes ≈ 410k bought every priority-1). One pass rotates to
harness/charter-adherence per the process rules. Builders write their own
adversarial hunt-lists; crosscheck briefs start from those and are told to exceed
them; hostile-identity briefing + engine-vs-dash construction discipline mandatory.

## §6 arch-1 bite predictions (pre-registered, to grade the round's calibration)

- bite-1 ~SUSPECT **span fidelity at the edges**: heredoc leaves are unsubstitutable
  at current AST granularity (span covers the `<<EOF` token; content is
  non-structural) ⇒ refuse-class, not fix; leaf-inside-redirected-group untested;
  multi-line-operand refusal untested under the new render.
- bite-2 +SURE **golden adjudication is the bottleneck**: most of 66 cases pin exact
  artifact bytes; span-render churns textually; BLESS exclusivity makes review
  orchestrator-serial. Mitigation: builder hand-derives; single orchestrator bless on
  a quiesced, freshly-verified tree, diff inspected case-by-case.
- bite-3 ~SUSPECT **StatusRenderFloor narrows rather than vanishes**: a substituted
  guard must reproduce consumed channels; stdout is unconsumed-by-default at HEAD
  (consumption = redirect-to-real-sink), so plain printing guards pass, but piped/
  redirected guards stay blocked (consumed-Stdout ⊤, correctly). The new guard class
  needs poles: known-rc single-line guard elides; ⊤-rc or consumed-output guard
  refuses.
- bite-4 +SURE **floor-test re-homing is real work**: observable_matrix.rs:765–907 +
  analysis/tests/cfg.rs:1136+ pin line-granular mechanics by name; "deleted not
  bypassed" means those pins move to span-render equivalents, and the retirement
  must hunt dead code (classify_lines' priority walk, inline_arm_subst,
  inline_scaffold_subst, commented_line).
- bite-5 ~SUSPECT **in-loop interactions**: 20M's in-loop render floor and 20S's
  Members body-substitution assume the line-granular form; arch-1 must keep the
  Members elision behavior semantically identical (its goldens are the
  newly-re-derived set most likely to churn subtly).

## §7 dq-surface ledger (standing; surface-never-settle; grows as the round teaches)

- **dq-errexit-1 candidate second cost-species — run-evidence**: a converged mutator
  run leaves third-party-observable evidence (apt history.log, auditd trail, atime,
  package-manager journals) that elision removes. Not an environment-health canary;
  not in the Observable tuple; shared with plain static elision (not
  errexit-specific) — but dq-1's frame ("is canary crash-fidelity the ONLY
  cost-species?") asks, so it's surfaced rather than self-dismissed. Raised to the
  human at round-open.
- **door-4 mint-policy fork**: {m-a probed-converged-only · m-b declared-kind-always
  · m-c even-unprobed}. All three correctness-safe (the guard re-measures live);
  they differ in wasted-read cost, artifact-diff size, and disclosure posture —
  dq-errexit-3-adjacent, so the seam takes all three; default m-a pending rulings.
  Raised to the human at round-open.
