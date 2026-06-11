# 21D — arch-7: seeded-random differential testing at scale (the cm-1 local approximation)

> Build agent strain note, round-21 arch-7 (the stretch arc). Append-only. The deliverable
> is the harness + this note; design-as-built, the 500-seed sweep, finding triage, the
> generator-coverage map, and the adversarial hunt-list. Confidence-marked
> (+SURE/~SUSPECT/-GUESS/--WONDER). Trust the human-authored root docs over this.

## §0 What was built (paths)

- **`spike/crates/hostsim/src/differential.rs`** (~1.88k lines incl. tests) — the harness
  library: a seeded generator, a shell-driver that drives the REAL `dorc` binary + `dash`
  (mirroring `e2e/run.sh`'s working Windows/MSYS invocation), the soundness judge, a
  deterministic minimizer, and a finding-emitter. 10 `#[cfg(test)]` unit pins.
- **`spike/crates/hostsim/examples/differential.rs`** (~210 lines) — the thin CLI:
  `--seed N` (one reproducible verbose trial), `--sweep COUNT --start-seed N` (summary),
  `--max-secs S` (budget), `--emit-findings` (write case-dir drafts).
- **`spike/crates/hostsim/src/lib.rs`** — one line added: `pub mod differential;`.
- e2e corpus: **untouched** (verified `git status e2e/` clean throughout).

Run it: `mise exec -- cargo run --example differential -- --sweep 500 --start-seed 0`
(from `spike/`). Sub-second-ish per trial (~0.27s on this box); bounded memory; streaming.

## §1 The property tested (cm-1, locally)

cm-1 (the deferred product-gate, `20K` §3/§4) is "the one gate that observes elided sites":
bare-vs-apply run-set against the license ledger, exhaustive over modeled host-states. Its
ONLY deferral reason was the human's isolation-tier pricing — tens of thousands of seeds are
expensive per isolation-tier but cheap in-process. arch-7 is that cheap local approximation.

Per a seeded `(book, oracle-set, host-state)` triple, the harness asserts: **the apply
artifact's execution trace == the bare book's trace MODULO licensed elisions** — every
command absent from the apply trace is covered by a minted license (an `omit`/`replace`
disposition the engine emitted via `--debug-argv`), and NO command the four-outcome lattice
required to run is absent (an under-execute = disaster class; over/unnecessary-execute grade
lower). The engine's own dispositions ARE the license ledger (the stabler choice over
artifact-comment parsing — `--debug-argv` is one structured line per site, see §2).

## §2 Design-as-built — the faithful flow (no site-number guessing)

The keystone realization (+SURE, verified live): **the probe artifact, executed under
host-state-aware probe-mocks, self-reports correctly-keyed `site N effect=… rc=…` records.**
So the harness never predicts the engine's `LeafId` numbering (which shifts with `set -e`,
multi-operand, loop members `N.M`). The per-trial pipeline:

1. GENERATE book + oracles + mocks into a scratch dir (auto-removed; `DIFF_KEEP=1` retains
   for debugging).
2. PASS A: `dorc --book -o… < /dev/null` → capture the probe artifact (1st shebang block).
3. Execute the probe under PROBE mocks (exit per host-state) → the `site …` records =
   probe-results.txt. (This is the real Dorc probe→apply flow, not a fixture stand-in.)
4. PASS B: `dorc --debug-argv --book -o… < probe-results` → the apply artifact (2nd block)
   + the disposition ledger on stderr (`argv <leafid> <run|replace|omit> <words>`).
5. `dash -n` both artifacts (the ap-2 parse gate).
6. Execute BARE book (file arg) + APPLY artifact (stdin heredoc) under the SAME apply-mocks
   (`PATH=mocks-only`, sandbox cwd, absolute `DORC_LOG`) → two `ran: …` traces.
7. JUDGE.

**The mock taxonomy (the anti-drift discipline, +SURE load-bearing):**
- *Mutator apply-mocks* (`instpkg`/`allowfw`/`enablesvc`): log argv, exit 0, host-state-
  INDEPENDENT — IDENTICAL in the bare and apply runs. A mutator's exit never differs between
  the two runs, so a bare/apply trace difference can ONLY come from elision, never mock drift.
- *Query-guard apply-mocks* (`havetool`): log argv AND exit per host-state — the guard's rc
  drives `||`/`&&`/`if` control flow in the BARE book, so it must report the SAME convergence
  the probe does (else bare control-flow and the engine's fold disagree). Its probe-mock
  (`dorcprobe_tool`) consults the SAME converged set, so they never disagree.
- *Probe-mocks* (`dorcprobe_<kind>`): exit per host-state, no log (probe execution only).

**Why drive the binary, not the in-process API** (+SURE this was the right call): mirrors the
pre-solved `run.sh` invocation pattern (the prompt's mandate — a prior attempt got tangled in
Windows-vs-POSIX path translation). The harness logic stays dependency-light (only
`std::process` + string work — NO new prod deps on syntax/oracle/plan; they stay dev-deps the
example doesn't even use), and it exercises the true end-to-end pipeline — strictly stronger
than the in-memory DST tests already in `hostsim/src/lib.rs`.

**Determinism** (`inv-determinism`, +SURE): a `u64` seed fully determines a trial. The
generator's randomness flows through a seeded LCG (`Rng`, independent of `Host`'s LCG so the
draw-order is local). No OS randomness, no clock. The only disk touched is the disposable
scratch tree; the only processes are `dorc`/`dash`. hostsim is the sanctioned DI seam.

## §3 The 500-seed sweep (authoritative, post-fixes)

```
sweep: 500 trials · 500 clean · 0 findings · 0 generator-rejects · 134.4s   (seeds 0–499)
sweep: 100 trials · 100 clean · 0 findings · 0 generator-rejects ·  24.6s   (seeds 5000–5099)
```

NOT vacuous (+SURE): across seeds 0–99, **59/100 trials exercise real elision**
(`replace`/`omit` in the ledger) which the judge validates as sound. The judge is PROVEN
able to scream — the mandatory planted-under-execute pin (`judge_screams_on_planted_under_
execute`) corrupts a clean apply trace by deleting a required `run` command and asserts
`UnderExecute`; a judge blind to the disaster class is worse than none.

Per-shape elision (seeds 0–59, post PRNG fix): OrTrue 8/9, AndGuard 8/11, IfGuard 7/8,
OrGuard 7/9, ForLoop 3/11, StraightLine 3/8, TopControl 0/4 (correct — ⊤ ⇒ run-everything).

## §4 Finding triage — every finding was a HARNESS bug; the engine held

The sweep surfaced THREE bug classes across iterations. **All three were MY bugs (generator
or judge), not engine under-executes.** The engine's degrade-spine (⊤⇒run) and
fork-mutator-rc discipline held under every attack — consistent with `20A`'s diagnosis ("the
degrade spine held under every attack; every break was in precision-ADDING code").

- **find-1 — generator: query oracle missing `oracle_effect … query …`** (initial run).
  A guard `havetool india` resolved to Opaque (empty probe-results, all sites unresolvable),
  because my generated `tool.oracle.sh` lacked the `oracle_effect havetool '' query present`
  marker. The effect-map keys on the (ε-)verb; without the marker the guard is not a Query.
  FIX: emit the marker (the `Polarity::Query` field). +SURE this was a generator bug —
  `fold-oror-guard-omits`'s real `tool.oracle.sh` carries `oracle_effect command '' query
  present`; I had omitted it.

- **find-2 — generator: double `__check` suffix** (the empty-probe-results root cause).
  My `check_fn_name(provider)` returned `instpkg__check`, then the format string appended
  `__check` again → `instpkg__check__check`. The engine keys the check by `<provider>__check`
  so it found none → every site `skip-unresolvable`. FIX: drop the redundant suffix. +SURE.

- **find-3 — judge: gate-5 `argv ⊆ bare` false-positive on guarded sites** (seed 13).
  A `run`-disposition mutator behind a short-circuiting `if`/`||`/`&&` guard legitimately
  never executes in the bare book (the guard succeeded), but my gate-5-style check demanded
  every `run` argv appear in the bare trace. FIX: REMOVED the gate-5 check entirely —
  gate-5/cm-2 already runs on the e2e corpus; the bare-vs-apply differential IS cm-1, and a
  value-flow mis-resolution that licenses a wrong elision surfaces as under-/over-execute
  anyway. ~SUSPECT this loses no cm-1 coverage; it removes redundant noise.

- **find-4 — generator: `echo "$(echo sub)"` is NOT a hard ⊤** (seed 29, `top-control-elided`).
  I had classed cmdsubst-in-ARGUMENT as a runs-everything ⊤-control. But `echo "$(echo sub)"`
  is a localized dynamic-arg — it does NOT ⊤-reject and does NOT poison an independent
  downstream converged install (verified: no `syntax-unsupported`/`cfg-top-node`; the engine
  correctly elided the independent install). Only `eval` and `break`-in-loop are hard ⊤s that
  havoc downstream (both verified to ⊤-reject AND force a following converged install to run).
  FIX: trigger list = `{eval, break-in-loop}` only. +SURE this was a generator bug.

- **find-5 — judge: loop-member argv mismatch (TOP-wildcard)** (seed 6, the most subtle —
  surfaced only after the PRNG fix improved shape distribution). A fully-converged loop
  `for x in foxtrot golf; do enablesvc enable "$x"; done` is correctly `replace`d whole, with
  the ledger reporting `replace enablesvc enable TOP` (loop var unresolved = `TOP`). But the
  bare trace has the concrete `enablesvc enable foxtrot`/`golf`. My judge did exact-argv
  license-matching → no license matched the concrete member → FALSE under-execute. FIX: the
  judge now treats `TOP` as a position-wildcard (`argv_matches`) AND cross-checks the removed
  member's host-state convergence INDEPENDENTLY (`removed_line_is_converged`) — so a wildcard
  cannot mask eliding a DIVERGED member (the adversarial guard, §6 risk-1). +SURE the engine
  was correct: a partially-diverged loop disposes `run`, never `replace` (verified seed-6
  loop-2: `site 1.1 absent` ⇒ `argv 1 run instpkg install TOP`, both members in the apply).

- **find-6 — generator: PRNG low-bit correlation** (a generator-quality finding, not a
  soundness finding). The first 100-seed run showed `set -e` present in 9/9 StraightLine
  trials (p≈0.2% if independent) — the LCG's lowest bit (used by `chance(1,2)` via
  `below(2)`) correlates with seed parity (a power-of-two LCG's low bits cycle short). The
  generated shape and the `set -e` coin were thus correlated, and StraightLine/ForLoop
  elision was silently 0 (all had `set -e`, under which converged mutators don't elide —
  fork-mutator-rc). FIX: `below` now uses the HIGH 32 bits (`>> 32`). +SURE — post-fix `set
  -e` is ~50% per shape and StraightLine/ForLoop elide. This is exactly the "silent caps read
  as coverage" trap the prompt warned of; the PRNG fix turned a silent cap into real coverage
  AND immediately surfaced find-5 (the loop-member case that the bad PRNG had been hiding).

Each fix is pinned: find-1/2/4/6 by `generator_emits_only_dash_n_clean_books` (200 seeds) +
the determinism pins; find-3 by the judge no longer carrying the check; find-5 by
`judge_top_wildcard_licenses_converged_loop_members` AND
`judge_top_wildcard_does_not_mask_diverged_member_under_execute` (the adversarial guard).

## §5 Generator-coverage map (which modeled shapes it CAN / CANNOT emit)

CAN emit (inside `inv-top-reject`'s modeled subset), all exercised in the sweep:
- straight-line mutators (`<provider> <verb> -y <entity>`; verbed establish).
- `if <query>; then <mutator>; fi`; `<query> || <mutator>`; `<query> && <mutator>` (the
  query-guard idempotency idiom — `command -v`-shaped, verbless `query` polarity).
- literal-list `for x in e1 e2; do <mutator>; done` (member sites `N.M`; whole-loop replace
  when all members converged, run when any diverged — both observed).
- `<mutator> || true` (door-3 rc-deadness; `replace` when converged — observed).
- `set -e` on/off (~50% post PRNG fix).
- deliberate ⊤-controls: `eval`, `break`-in-loop (hard ⊤s that havoc a following converged
  mutator → assert runs-everything).

CANNOT emit (silent caps — documented so they don't read as coverage, per the prompt):
- **cross-file flow** (`. /path` sourcing; a book calling a command another file's oracle
  defines across files). The `seam-interproc` surface is untested here. ~SUSPECT this is the
  biggest coverage gap relative to the modeled subset.
- **function inlining / wrapper-pun** (arch-2 territory). Not emitted.
- **`case` statements**, **nested loops**, **`while`/`until`** loops. The generator emits
  single-level `for` only. (`while`-cond is `StatusIterated`, a distinct floor — untested.)
- **heredocs** (the arch-1 render-refusal class — `render21-heredoc-refusal`). Not emitted,
  so the refuse-at-render path is not exercised by this harness.
- **multi-operand mutators** (`install a b` — the `[ "$2" = "" ]` guard). Not emitted; the
  single-operand annotation is always satisfied.
- **`$()` Query sites** (arch-4). Not emitted as a probe-able site.
- **consumed stdout/stderr** (the observable-liveness blocks). Not emitted — the generated
  guards redirect to `/dev/null` (exempt) and mutators don't pipe.
- **mixed-state same-entity** across sites (the `removed_line_is_converged` `.any` would say
  converged if ANY site has it so; ~SUSPECT a latent imprecision, mooted by distinct entities
  in practice — see §6 risk-2).

The North-star coverage number (`211` §1) is NOT what this harness measures — it measures
SOUNDNESS over a bounded shape-space, not elision-coverage of H2SALS. Complementary, not
overlapping, with arch-6's dashboard.

## §6 Adversarial hunt-list (judge-blindness risks, ranked)

- **risk-1 (highest) — TOP-wildcard license masking a real under-execute.** A loop body's
  `replace … TOP` is a wildcard; naively it could license eliding ANY member, masking a
  wrongly-elided diverged member (the disaster). MITIGATED: the wildcard is honored ONLY when
  the removed member's host-state is converged (`removed_line_is_converged`, an INDEPENDENT
  cross-check against `trial.entity_states`, not the engine's claim). Pinned by
  `judge_top_wildcard_does_not_mask_diverged_member_under_execute`. RESIDUAL (~SUSPECT): if
  the same entity name appears both converged and diverged across the book, the `.any(conv)`
  is lenient — but the generator uses mostly-distinct entities, and the engine wouldn't
  whole-replace a mixed loop anyway. Tighten to `.all(conv)` if mixed-entity books are added.

- **risk-2 — trace-capture gaps.** A command that fails to log (un-shimmed external) would
  silently drop from a trace. MITIGATED: `PATH=mocks-only`, so an un-shimmed external ⇒
  `command not found` ⇒ the bare/apply run errors (swallowed, but the trace would then
  diverge and surface as a finding). The shims log `${0##*/} $*`; whitespace is normalized on
  both ledger and trace sides (`normalize_argv`) so a trailing-empty-arg space can't cause a
  spurious mismatch. ~SUSPECT a probe whose `dorcprobe_*` is un-shimmed would yield empty
  probe-results (⇒ Unknown ⇒ run) — safe direction, but a silent coverage loss, not a false
  pass.

- **risk-3 — license-ledger misparse.** `parse_ledger` keys on the `argv ` prefix and splits
  `<leafid> <disposition> <words>`. A disposition string the engine adds beyond
  `run`/`replace`/`omit` would be treated as "not a license" ⇒ conservative (a removal under
  an unrecognized disposition fires under-execute, the safe direction). The leafid is parsed
  but unused by the judge (the argv is the matching key). +SURE the parse is conservative.

- **risk-4 — mock-behavior drift between bare and apply runs.** The CENTRAL soundness
  precondition. MITIGATED by construction: mutator apply-mocks are host-state-INDEPENDENT
  (identical exit 0 in both runs), and the SAME `mocks/` dir + SAME `DORC_LOG`-discipline
  drive both. The only host-state-dependent apply-mock is the query-guard, and it consults
  the same converged set as its probe-mock. ~SUSPECT the one place drift could hide: if a
  generated guard's apply-mock and probe-mock keyed different entities — but they're stamped
  from the same `entity_states` rows, so they can't.

- **risk-5 — judge too weak (all-clean is vacuous).** Countered by: (a) the planted-under-
  execute pin proves the judge screams; (b) 59/100 seeds exercise real elision the judge
  passes; (c) the ⊤-control + over-execute + under-execute checks each have a pinned
  hand-built positive. ~SUSPECT residual: the harness only tests the SHAPES it generates — a
  bug in an UN-generated construct (sourcing, `case`, heredoc) is invisible here. That is a
  coverage gap (§5), not a judge weakness.

## §7 What this buys, and what it doesn't (the honest frame)

+SURE: 500 deterministic, reproducible, sub-second trials found ZERO engine under-executes
within the modeled subset — corroborating the round-20/21 soundness story from a third angle
(neither tests nor crosscheck nor the e2e corpus). The harness IS the cheapest cm-1 stand-in;
everything cm-1 needs (runnable probes, ordered run-sets, dispositions in `--debug-argv`)
exists, and this proves the loop closes locally.

NOT a substitute for cm-1 proper (~SUSPECT): cm-1's value is EXHAUSTIVE host-states over the
REAL corpus at the product isolation-tier; this is seeded-random over a GENERATED bounded
shape-space. The coverage gaps in §5 (sourcing, function-inlining, `case`, `while`, heredoc,
`$()`-Query, consumed-output, multi-operand) are real — a soundness bug living only in those
constructs is invisible to this harness. The right next step is to grow the generator toward
those shapes (each is a `re-eval-trigger`-style increment on the same substrate), with this
harness's judge as the safety net the roadmap previously lacked.

--WONDER whether the generator should be seeded from the e2e corpus's actual book shapes
(mutating real cases) rather than synthesizing from a kind-vocabulary — that would close the
"only tests what it generates" gap faster than hand-growing each construct, at the cost of
less control over host-state coverage. Flagged, not decided.
