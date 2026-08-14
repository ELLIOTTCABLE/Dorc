# spike-lean-sparing

A Lean 4 formalization of the sparing algebra — `notes/277` **as written**
(coordinate, ternary compare + consumer map, selector dialects, set-lifting /
universal meet) and `notes/272` §4. A de-risking probe, throwaway-grade except
for the theorem statements; the full account (per-theorem ledger, SPEC-GAPS,
toolchain notes) is `REPORT.md`.

Status: **CHECKED** — `lake build` green on Lean 4 v4.33.0 (core only, no
mathlib); zero `sorry`; every law's axiom profile is `[propext, Quot.sound]`.

Layout: `SparingAlgebra/{Coordinate,Dialect,Compare,Sparing,Laws}.lean` —
definitions in the first four, all theorems in `Laws.lean`.

Build: toolchain rides the repo `mise.toml` (elan under mise; Lean version
pinned by `./lean-toolchain`). One-time on a fresh machine:
`mise exec -- elan-init -y --no-modify-path --default-toolchain none`; then
`mise exec -- lake build` from this directory.
