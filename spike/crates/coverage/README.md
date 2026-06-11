# `dorc-coverage` — the analyzer-coverage dashboard (round-21 arch-6)

A re-runnable report over any **book + oracle set** (+ optional probe-results) that
decomposes the round's north star — "~80% criticality-weighted non-trivial elision
coverage on a converged host" — into per-site, per-door, separately-ownable terms.

It is a measuring **instrument**, not part of the analyzer kernel and not a gate: it
reads the engine's outputs and *attributes* them; it never makes an analysis decision
of its own, and it never fails a build.

## Invocation

```sh
# from spike/ — build first
cargo build -p dorc-coverage

# one book + its oracles (+ optional probe-results), full per-site table + rollups
target/debug/dorc-coverage --book=book.sh -o pkg.oracle.sh -o svc.oracle.sh \
    --probe-results=probe-results.txt

# rollups only (no per-site table); write a machine-readable TSV alongside
target/debug/dorc-coverage --book=book.sh -o pkg.oracle.sh --no-table --tsv=cov.tsv
```

Flags (mirrors the `dorc` cli's arg style):
- `--book=<path>` / `--book <path>` — the book sh (required).
- `-o <path>` / `-o<path>` / `--oracle <path>` — an oracle file (repeatable).
- `--probe-results=<path>` — the site-keyed `site <id> effect=… rc=…` records. Without
  it, c3 is unknown and every convergence-gated site shows `runs/unprobed` (the honest
  host-less shape).
- `--tsv=<path>` — write the per-site + rollup TSV (stable column order).
- `--no-table` — suppress the per-site table (keep the header + rollups).

### The gate-set wrapper

`spike/tools/coverage.sh` runs the dashboard over the whole e2e corpus (rollup per
case), or a one-off book. It is **NOT** wired into `e2e/run.sh` and never fails a
build (exits 0 even when the binary is missing):

```sh
sh tools/coverage.sh                         # rollup over every e2e case
sh tools/coverage.sh --full                  # ... with per-site tables
sh tools/coverage.sh book.sh oracle.sh ...   # a one-off book+oracles
```

## What it reports (per command-site)

- **c1 analyzable** — `yes` (a fact resolved) / `n/a` (a `MustRun` — the public
  surface cannot tell opaque-⊤ from a pure builtin).
- **c2 oracled** — did an oracle `check()` + effect-map resolve a fact.
- **c3 probed** — the host's Effect verdict (`holds`/`absent`/`?`) when probe-results
  are supplied.
- **c4 door** — through which door the disposition was reached (below), with a
  `why` (the dominant blocking reason for a `runs` site).
- **wt** — criticality weight (the site's line-count, a stand-in for the future 1A
  matrix; see `weights.rs`).
- **rung** — the dq-2 effort-rung (`r2/4` readable-idiom / `r3` needs-declaration / `-`).

## The door vocabulary (full-elisions and guard-transforms NEVER blurred)

| door | meaning |
|---|---|
| `fold` | door-1: a leaf in a provably-dead branch under a *measured* guard rc (`Disposition::Omit`) |
| `dead-invariant` | door-3: the `cmd \|\| true` shape — a `Replace` whose status is `StatusInvariant`-consumed |
| `replace-converged` | a plain converged-establish elision (`Replace` via `ConvergedEstablish`/`MembersLoop`) |
| `query-substituted` | a read-only Query guard value-substituted to its probed rc (`Replace` via `QueryGuard`) |
| `guard-transform` | door-4 (not built yet) — **reports 0**; a guard-insertion license |
| `static-declared` | door-2 (not built yet) — **reports 0**; a declared converged-run elision |
| `runs` | the leaf runs verbatim; the `why` names the dominant blocking reason |
| `unattributed` | a disposition shape this build does not recognise (a new engine variant) — surfaced loudly, never silently mis-bucketed |

The **north-star** block reports full-elision (the run-set shrinks) and guard-transform
(door-4) as SEPARATE numbers, never summed — per the doors-program charter. The
criticality-weighted full-elision fraction is the ~80% question's measurable form.

The **dq-2 rung split** answers "how do we degrade gracefully": which sites already pay
off from readable idioms (`guard-readable`) vs which await an oracle declaration
(`needs-declaration`, the door-2/door-4 population).

## Design notes

- Pure kernel (`inv-determinism`): `BTreeMap` throughout, sites sorted by id, no
  clock/RNG. The binary is the only I/O edge.
- **Evolution-survival**: every match over an engine enum ends `_ =>` into an honest
  `unattributed` bucket, so the crate survives new engine variants
  (`SkipClass::InlineCall`, new `Channel`s/`LicenseVia`s) and reports its own blind
  spots instead of failing to compile or miscounting.
- It consumes the other crates as libraries; it never re-implements engine logic. The
  design-as-built, the H2SaLS rollup, census discrepancies, and the seam wishlist are
  in `Research/notes/21B-arch6-coverage-dashboard.md`.
