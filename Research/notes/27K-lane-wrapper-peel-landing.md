# 27K — block-context lane-wrapper-peel (né `24S` W1) landing + residue

AI-authored (Opus builder, r27 lane-wrapper-peel session, 2026-07-17). Records what landed for
`270:block-context`'s FIRST lane (`27J` §2.1). **This note is the durable deliverable** (the
session's conductor was rewound mid-turn; the final chat message was discarded — everything is
here). Authority: root docs + `spike/CLAUDE.md` rulings + `271`/`273`/`274`/`27C`/`27J`/`27D`/`27H`
outrank this. Companions: `27J` (the build spine / lane order), `273` (the wrapper surface spec),
`27H` (the positional-model charter this lane discharges).

## Branch / fold state (READ FIRST — the conductor must reconcile)

- Branch `ai/r27-wrapper-peel`, based on `ai/spike3-r27` @ **`7794838`** (the brief's stated base;
  verified at step-zero and at `HEAD~4`).
- **The lineage MOVED during the run.** `ai/spike3-r27` advanced `7794838 → 20598e9` (two
  DESIGN-only commits: `27Xn` context-entry crosscheck digest + its quarantine archive — the `27J`
  §3 Sol/codex crosscheck that ran PARALLEL to this lane). My four commits are based on `7794838`,
  NOT `20598e9`. **The fold must rebase these four onto the current `ai/spike3-r27` tip.** The two
  advancing commits are design docs (crosscheck digest); no code overlap with this lane is
  expected, so the rebase should be clean, but VERIFY.

## Commits (on `ai/r27-wrapper-peel`, oldest→newest)

1. `5bc7467` (AI new fix oracle) — **the oracle-side positional model** (`Word::PositionalArgs`).
2. `aa5396f` (AI new oracle) — **the wrapper model** (`oracle::wrapper`: peel detection, ρ-ladder,
   `lend_map`, `InnerContext`, dual-peel coherence). MODELS-only.
3. `0163db2` (AI new cli e2e) — **dual-peel coherence fail-fast** (CLI) + **four peel-tier e2e
   fixtures**.
4. `ba64aab` (AI test) — register the two new `DiagCode`s on the `diag_tidy` legacy allow-list.

## Acceptance summary (all green)

- **Four gates clean** on the whole workspace: `cargo fmt --check` · `clippy --workspace
  --all-targets` (0 warnings) · `cargo deny check licenses bans sources` · `typos spike`.
- **27 unit-test suites green** (+16 `oracle::wrapper` tests, +2 positional-model parser tests, +2
  wrapper e2e/coherence tests re-homed into the parser/verdict suites). No failures.
- **e2e: 80/80** (was 76). The **76 pre-existing cases are BYTE-STABLE** (rung-0 proof, below); 4
  new peel-tier cases.
- **Referendum did NOT fire** (`270` §2 / `273` §11 watch-item): no build contact forced a
  wrapper-aware arm into a TOOL oracle. The wrapper model lives entirely in `oracle::wrapper` and
  the cli edge; no tool oracle mentions a wrapper, no wrapper oracle mentions a kind, `command_effect`
  is untouched. `wrapper-law` holds.

## 1. The positional-model transition (report-ask #3 — both evaluators; founding-pin status)

The `27H` `finding-positional-oracle-side-couples-founding-pin` is DISCHARGED. The wrong-concrete
`Word::Literal("$@")` (which resolved to the literal text `$@`) is replaced by a position-aware
model:

- **New AST node** `Word::PositionalArgs` (`predict/ast.rs`) — the faithful positional LIST.
- **The lexer now tracks whole-word double-quoting** (`predict/lexer.rs`: `Tok::Word` gained a
  `double_quoted` flag, mirroring `single_quoted`). This is what makes `"$@"` REPRESENTABLE-distinct
  from bare `$@` — the parser previously decoded quotes away. Threaded through a new `WordQuoting`
  bundle in `parse_word_lexeme` (`predict/parser.rs`).
- **Parser routing** (`predict/parser.rs`): only double-quoted `"$@"` → `PositionalArgs`. bare
  `$@`, `$*`, `"$*"` (and `$#`/`$?`/… — no corpus use) → `Word::Unmodeled` (⊤ everywhere), the
  correctness fix — they word-split / IFS-join and so do NOT preserve the argument list
  (`271:rul-env-claim-inversion`: bare `"$@"` claims NOTHING).
- **Both evaluators**: `resolve_word` (`predict/eval.rs`, the shared exhaustive `Word` match) returns
  `Err` for `PositionalArgs` — ⊤ in VALUE position (annotation RHS, `[ ]` operand, `case`
  scrutinee), a multi-value list is not one value. The verdict `run_command` (`verdict.rs:367`) and
  the predict `Command` handler SKIP `PositionalArgs` — command-position `"$@"` is
  concrete-by-construction (the traced positional list), so it must NOT ⊤ the check. (The predict
  `Evaluator`'s Command handler never resolved words, so it needed no change beyond the exhaustive
  match; `touches.rs` ⊤s on it via the shared `resolve_word` — the safe direction.)
- **Founding-pin status: GREEN.** `typeless-floor-oneliner` (`mycmd__is_converged() { mycmd
  --dry-run "$@" ;}`) still Vouches — `"$@"` now resolves via the CORRECT model (`PositionalArgs`
  skipped in `run_command`) instead of the wrong-concrete literal. `typeless-floor-converged-elides`
  green. The strip is span-based (verbatim), so the shipped probe bytes are unchanged — the AST
  change affects only whether the vouch RESOLVES, not what ships.
- **Churn containment**: only ONE corpus oracle uses `$@`/`$*` (the founding one-liner). All 76 e2e
  byte-stable confirms no other file churned.

## 2. Peel / region / context-population shapes (report-ask #2 — types; where FactKey stands)

All in `oracle::wrapper` (new module). MODELS-only — nothing here is consumed by `analysis`/`plan`
yet (the entry/dial/vouch machinery is `lane-context-entry`).

- **`detect_peel(&Predict) -> Option<Peel>`** — wrapper-ness by tautology (`273` §1): the first
  reachable command whose guest `"$@"` is in EXECUTING position. The two modeled transparent
  executors: bare `"$@"` (head IS the guest) and `env … "$@"` (env execs its trailing operand). A
  trailing `"$@"` in ARGUMENT position (`grep "$@"`) is NOT a peel (walls). `Peel { rho: RhoClaim }`.
- **`RhoClaim`** (the ρ-ladder, `271:rul-env-claim-inversion` + `274` §12 r1–r6): `Nothing` (bare
  `"$@"` = ⊤, never "isolation") · `FullAmbient { overrides }` (`env "$@"`) · `ExactlyThese { vars }`
  (`env -i VAR=v "$@"`; `env -` = `-i`, r2). Unrecognized env flag (`-u`, `-S`/`-C`/`-P`, dynamic
  head) ⇒ `Nothing` (claims-nothing + hint, r1/r6). Path-qualified `/usr/bin/env` ⇒ not recognized
  as the env-executor ⇒ not a peel (walls, r1-safe).
- **`Dimension`** (engine-owned closed set): `{ User, FsView, Netns }` + `Dimension::ALL`,
  `as_token`/`from_token`. `fs-view` kept DISTINCT from the `fs` substrate token (`273` §8). ρ is
  NOT a lend_map dimension (it rides the predict env-idioms).
- **`LendEntry`** (`271:rul-lend-map`): `Full` (colon-line `:  : user`) · `Mapped` (`printf … : user`)
  · `Top` (a MISSING dimension = ⊤/walls — the enumerate-every-dimension law;
  absent-key-means-full-lend REJECTED).
- **`LendMap` + `derive_lend_map(&Predict)`** — per-dimension entries + `peels` (terminal `"$@"`).
  `lend(dim)` returns `Top` for any dimension the body did not answer. `missing_dimensions()`
  iterates `Dimension::ALL` (a newly-minted dialect dimension auto-reads ⊤ against an old member —
  `273` §3 version story). An unknown mark token on a lend_map line ⇒ a LOUD `lend-map-unknown-
  dimension` Warning (`inv-top-reject`), mints no lend. `FnRole::LendMap` + `lift_lend_maps` added to
  the shared role parser.
- **`InnerContext` + `inner_context(&LendMap)`** — THE context-population design (report-ask #2):
  `HostDefault` (all dimensions Full, or an identity wrapper — the inner sits in the caller's world)
  vs `Shifted { shifts: BTreeMap<Dimension, LendEntry> }` (a mapped-or-⊤ dimension shifts the inner
  out; a ⊤ dimension is a wall recorded for the next lane's guard/run degrade). This is the "begins
  to be populated" computation.

### Where `FactKey` stands (the seam decision)

**FactKey is UNTOUCHED this lane; `core::Context` gained NO new variant.** Per `27J` §4
("fact-plane context keying → lane-context-entry; FactKey widening decided at that brief") and the
standing **`tc-context-slot-on-coord-not-factkey`** (from `27G`/`27D`, still open). The reasoning:
threading `InnerContext` into `dorc_core::FactKey` touches FactKey and its ~47-site map so two
same-cell facts in different contexts do not collide — a cross-cutting decision I could not bound
inside this lane without risking rung-0. So `inner_context()` is the DESIGN of the population (the
computation that WOULD key the slot), delivered as a standalone descriptor that `lane-context-entry`
consumes. `core::coord::compare` is already ready: it answers `Unknown` on a context mismatch
(`never-derive-separation`), so the survival/transport consumers work the moment the slot is
populated. **Recommendation for lane-context-entry**: mint `Context::Wrapped(_)` (or similar) from
`InnerContext::Shifted`, thread it through FactKey's constructors, and add the two-same-cell-
different-context collision pin.

## 3. Dual-peel coherence + the fail-fast (`273` §5)

- **`check_peel_coherence(predict, lend_map, argv) -> Option<Incoherence>`** — both members'
  argparse must reach `"$@"` after consuming the same number of leading argv tokens (the guest
  starts at the same depth). A dedicated `PeelTracer` runs the shared argparse primitives
  (`eval_test`/`resolve_word`/`pattern_matches` — the 24A §1b vocabulary fence) and records
  positionals consumed by `shift`. A non-peeling member declines (coherent, adds no license).
- **CLI wiring** (`cli/main.rs::check_wrapper_peel_coherence`): at oracle-load, for every provider
  authoring BOTH a peeling `__predict` and a `__lend_map`, run the check over three canonical probe
  argvs (`["g"]`, `["-a","g"]`, `["-a","-b","g","x"]` — exercise the flag-strip loops + guest/operand
  positions). Any disagreement ⇒ a loud `wrapper-peel-incoherent` Error + fail-fast:
  `RunOutcome::WrapperIncoherent` → `EXIT_WRAPPER_INCOHERENT = 11` (second of the reserved 10..=19
  dorc-semantic fast-fail range). The artifact STILL ships (like `BookUnmodeled`); mints NO license
  (an error is the safe direction).
- **SCOPE-CUT (flag, not a tc-\*)**: the load-time check uses a canonical-argv heuristic, which
  catches the shift-count / flag-count class of incoherence. The PER-SITE check over real book argvs
  (`273` §5 "the same book-invocation") is `lane-context-entry`'s refinement — it needs the
  wrapper/inner split wired into the book pipeline, which this lane does not build.

## 4. Rung-0 byte-stability proof (report-ask #4)

- Baseline captured BEFORE any edit: **76/76 e2e** (`b0as8jmha`).
- After the positional-model commit: **76/76 e2e** (`bw1hfsw9u`) — the one corpus `"$@"` (the
  founding one-liner) re-vouches via the correct model; every other case byte-identical.
- Final: **80/80 e2e** (`bo7snbn5p`) — the 76 pre-existing cases unchanged, 4 new peel-tier cases
  added. The wrapper machinery is invisible until a wrapper oracle loads
  (`empty-world-byte-identical`): a wrapper predict carries no effect marks (no cells, no dialect
  minting), and `command_effect` is untouched, so a wrapped book site walls EXACTLY as an opaque
  command did.
- **Zero-new-trust cross-check**: `wrapper-modeled-peel-coheres-walls` (a coherent sudo oracle
  loaded) vs `wrapper-unmodeled-peel-walls` (no oracle) — the wrapped site `sudo hork setup` RUNS
  verbatim in the apply of BOTH (the modeled wrapper mints no elision; the peel is modeled, not
  consumed). User bytes never rewritten (`24S` §8 invariant, pinned by the e2e apply artifact).

## 5. e2e tally (report-ask #6, verbatim)

```
all 80 e2e round-trips passed (ap-2 dash -n + apply/probe exec gates, redirect sandbox,
ordered run-set, stderr floor, argv-echo differential, dual-rail license judge, why-lens emission)
```

Four new cases (corpus idiom, inert mocks under `PATH=mocks-only`):

- `wrapper-dual-peel-incoherent-fails-fast` — `DORC_EXIT=11` + an `expected-diagnostics` file
  declaring `wrapper: error[wrapper-peel-incoherent]` (gate-3 requires undeclared error-diagnostics
  be declared; discovered + fixed during the run). Artifact ships; the book's own `hork setup` walls.
- `wrapper-unmodeled-peel-walls` — `sudo hork setup`, no oracle ⇒ opaque command, walls.
- `wrapper-modeled-peel-coheres-walls` — coherent sudo oracle loads + coheres; wrapped site still
  walls (zero new trust).
- `wrapper-identity-lend-map-coheres` — an identity (nice) wrapper (bare-`"$@"` predict +
  all-full-lend lend_map) coheres.

## 6. `24S` §8 invariants landed as tests (report-ask, "where cheap")

- **rung-0 behavior byte-identical to HEAD goldens** — the 76-case e2e byte-stability (§4).
- **silence never identifies / ⊤ identifies with nothing** — `lend_map_full_and_missing` (a missing
  dimension ⇒ `LendEntry::Top` / walls, not full lend) + `identity_wrapper_bare_at_claims_nothing`
  (bare `"$@"` ⇒ `RhoClaim::Nothing`, never isolation). Core-side `compare` context-mismatch ⇒
  `Unknown` is already pinned in `core::coord` tests.
- **user bytes never rewritten** — the e2e apply artifacts (§4 zero-new-trust cross-check).
- **probes never escalate** / **every cross-context elision renders its chain** — N/A this lane (no
  probe or elision machinery built; `lane-context-entry`).

## 7. tc-\* flags carried forward (NEVER resolved here)

- **`tc-context-slot-on-coord-not-factkey`** (open, from `27G`/`27D`): FactKey widening for the
  inner-node context. `inner_context()` is the population design; the FactKey threading + the
  two-same-cell-different-context collision pin are `lane-context-entry`'s (see §2 recommendation).
- The REFERENDUM (`273` §11 no-wrapper-awareness) did NOT fire — reported clean, not flagged.
- `inv-superposition` — nothing needed flagging UP (the wrapper model is phase-agnostic data; no
  phase-baking).

## 8. Modeling gaps / churn-avoidance disclosures (`ru-26`)

- **The bare prefix-assignment ρ rung `VAR=x "$@"` is NOT modeled this lane.** The predict parser
  splits a leading `VAR=x` into a script-scoped `Stmt::Assign` (not a command-scoped prefix), so
  `VAR=x "$@"` (per-variable, rest ⊤) parses as an assign + a separate bare-`"$@"` command. The
  ρ-ladder's env-headed rungs (`env "$@"`, `env -i … "$@"`) ARE modeled (env is a real command word,
  so its assignment args stay in the command). Authors get exactly-these via `env -i VAR=v "$@"`,
  which IS recognized. Extending the parser to fold a leading assignment into the following command
  was deliberately deferred (corpus-churn risk against the founding pin); flag for
  `lane-context-entry` or the stdlib wrapper-authoring brief if the bare per-variable rung is wanted.
- **`FullAmbient { overrides }`** records `env VAR=v "$@"` per-variable overrides but the consumer
  (next lane) treats FullAmbient as full ambient regardless; the overrides field is representation-
  open for a future refinement.

## 9. Where the next lane picks up (`lane-context-entry`, `27J` §2.2)

The `cmd__enter()` member (`27C` §3), the escalation dial + `tolerates:` vouch, per-(host,context)
probe composition, the degrade ladder — all consume the shapes landed here: `detect_peel`/`RhoClaim`
for the wrapper predict, `LendMap`/`inner_context` for the context keying, `check_peel_coherence`
for the per-site coherence refinement. The one hard hand-off is the FactKey context threading (§2 /
§7) — mint the `Context` variant there and thread it, then the whole survival/transport algebra
lights up on the already-ready `compare` chokepoint.
