# 30F — lane-spine-reification: builder lane report

> Tier: **LLM-authored, builder (Opus-class)**, working `ai/r30-lane-spine-reification` from
> `ai/main@fa046a37`. Executes `plans/309` §5 stages 2–3 against the census `notes/30E`, whose §8
> rulings are what the build followed. Everything here is as-built and measured; confidence markers
> where it is not.

## §1 — What landed, per `309` §5's stage structure

### stage-spine-transition (`309` §5.2)

- **`core::spine`** — the one in-memory decision structure. Pure data, dependency-clean, `SiteId`-keyed,
  `Ord`-deterministic; fifteen record species; the no-wildcard `census_arm` classification (4 durable /
  0 excluded / 11 new, count-pinned by test); `ExcludedContent` enumerating the four CONTENT-tier
  exclusions; `Account<T>` as the k-capped operand account (`law-spine-operands-capped`).
- **`Plan` is a projection.** `build_plan_walled` now returns a `Spine`; `plan::spine::project_plan`
  derives `Plan` from it, `project_survival_report` derives the survival instrumentation from
  `SpineSurvival` records + the narration minted beside them. The driver's two post-construction field
  pokes (`defensive_emission`, the certifier-trip demotion) are Spine writes.
- **The durable is a projection through per-species Views.** `plan::whylog::view` +
  `DurableProjection::project(&Spine)` is the ONE route from decisions to disk. `WhylogV2Write::of_projection`
  is how the driver reaches the writer.
- **Drop-accounting** — `CollapseKind::ProjectionDrop { projection, species, dropped }`, minted per
  non-durable species the Spine actually held, walking `SpineSpecies::ALL` so a new species is
  accounted for the moment the census classifies it.
- **The `30E` §3 hidden decisions are hoisted**: `dec-pinned-definitions`, `dec-render-refusal`,
  `dec-omit-neutralisation`, `dec-defensive-emission`, `dec-certifier-trip-cleanup` are all
  `SpineRenderDecision` records, computed from the render's own seats.
- **`flg-v1-durable-is-fixture-only` discharged** — the v1 grammar (`WhylogDoc`, `serialize`, `parse`,
  `WHYLOG_TAG`/`WHYLOG_END` and their helpers) is DELETED; `whylog::inspect` is re-cut onto the v2
  admission with a small structural NAMER for the four `WhylogCorruptReason` conditions, and every
  other refusal keeps the admission's own named diagnostic rather than being flattened to "corrupt".

### stage-306-accounting (`309` §5.3)

- **Grade-stamping** — carried by CONSTRUCTION rather than per-mint discipline: `Spine::minted_at(grade)`
  stamps every record it receives, and `build_plan_walled` takes the grade as a parameter. The driver
  passes its intake's `InfluencePhase`; the intakeless entries (kernel, `hostsim`, sweep, coverage)
  pass `None`. A new mint site cannot forget, and `306b` §1c's gradation axis keeps its room.
- **Report-only state + typed projection-absence** — `PlanAuthority` is a private-field witness with no
  `Default` and no public constructor beyond three named mints. `PlanAuthority::authorise(Admission<T>)`
  returns `Authorised<T>`, which ATTACHES the witness to the two continuing arms and gives the `Refused`
  arm none. `project_plan` demands one, so a run whose intake integrity is lost has no plan-producing
  conversion available — a typed absence, per `306b` §4b's "a type, not a flag".
- **`pin-debug-dump-gating`** — `Spine::debug_dump()` names no sink (no path, no writer, no
  destination, none addable), returns a `String`, and is held by the lexical non-empty-walk gate
  `the_new_arm_debug_dump_has_no_production_caller` over five crates' `src/`.

## §2 — The end-gate verdicts

- **Smoke-diff baseline: BYTE-IDENTICAL.** `mise run spine:baseline` reproduces
  `spike/spine-baseline-before.txt` exactly, over all 232 cases. The instrument was RE-POINTED at the
  decision plane (`WhyWorld::spine()`), so byte-identity now proves the reification itself rather than
  proving one projection agrees with itself.
- **`.whylog` durable bytes: BYTE-IDENTICAL** [measured, not argued]. A binary built from `fa046a37` in
  a throwaway worktree and the lane's binary were run over the same case, same framed records, same
  fixed clock (`DORC_FIXTURE_CLOCK_MS`), into separate durable dirs. The only differing lines are the
  two `argv value=` rows naming the two different binaries and their two different output dirs; header,
  book digest, oracle digests + ordinals, decision digest, instants, apply rows, results block and
  sentinel are identical. The throwaway worktree is reaped and pruned.
- **Suites**: Windows `gate:full-quiet` 2173/2173 (2 skipped); WSL leg 2169/2169 (2 skipped) — the
  ordinary platform-gated delta. `bless:dry` green with a clean tree afterwards (zero golden writes).
- **`empty-world-byte-identical`** holds by construction and by the corpus: 158 e2e cases and 266 loom
  cases pass with no golden churn beyond §3.

## §3 — The loom re-cut, classified

Four cases and two generated rows moved, all of it the v1→v2 grammar deletion and nothing else. Every
`message:` register is BYTE-IDENTICAL — no user-facing prose moved, so `error-authorship-tier` holds.

| case | what moved | why |
|---|---|---|
| `whylog-corrupt` | `.whylog` header v1 → v2 | one grammar; still renders `EndSentinelMissing` |
| `whylog-corrupt-results-block-overruns` | `.whylog` header v1 → v2 | still renders `ResultsBlockOverruns` |
| `whylog-version-refused` | refused version `/2` → `/3`; transcript + derived `example` | v2 is now the reader, so `/2` is no longer a refusable version |
| `whylog-book-desync` | `.whylog` re-cut to a valid v2 durable; case moved to the ORACLE arm; `why:` metadata re-sourced | see `fnd-book-arm-is-shadowed-by-the-degraded-receipt` below |
| `catalog_lock.rs` | two rows (`whylog-version-refused.example`, `whylog-book-desync.{why,example}`) | derived from the above; republished via `promote --accept-metadata` |
| `dorc-loom/tests/consumer.rs` | `{which}` pin `book` → `oracle firewall.oracle.sh` | follows the case's arm; a variable-value pin, not a prose pin |

### `fnd-book-arm-is-shadowed-by-the-degraded-receipt` [verified]

The old `whylog-book-desync` case reached its diagnostic ONLY because its v1 durable failed v2
admission, so the drifted-receipt route declined and the case fell through to `inspect`. That is
exactly the rot `30E` §1 named. Under one grammar, an admitted book-drifted durable takes the DEGRADED
RECEIPT route (`28F:rul-drift-replay-d1`) — which the case's own `why:` already described — so the
`book` arm is no longer reachable through `--last` and the case now renders the ORACLE arm, which still
refuses outright. This required `inspect` to resolve current oracle sources through a caller-supplied
resolver; before this, the loom consumer passed an empty oracle slice, so that arm was unreachable from
any transcript at all.

## §4 — Deviations, under the deviation-litmus

1. **`Spine` is generic over a `DecidePlane` seam** [shape decision]. `30E` §5 puts Spine in `core` as
   pure dependency-clean data, but a `SpineDisposition` is license-bearing, a `SpineRecordStream` holds
   admitted host bytes, and a narrative is describe-plane — all three are minted in crates `core` may
   not depend on. One associated-type seam names them; `plan` implements it once and aliases
   `Spine<PlanPlane>`, so the parameter is invisible downstream. Alternatives considered and rejected:
   moving the licenses into `core` (breaks `plan`'s sole-mint law and `aid-is-the-describe-plane`), or
   splitting the disposition record out of Spine (defeats "one structure", and `30E` §2 explicitly wants
   the record license-bearing so `DurableView` is vindicated).
2. **The report-only state is STRUCTURE, not new behaviour.** `309` §5.3 asks for "the report-only Spine
   state + typed projection-absences"; `306c` §3a additionally wanted a refused target to get a complete
   analysis and a full report. The typed absence is built; the refused path still reports-and-returns
   exactly as before, so no golden moved. Widening it to render a full report on refusal is render work
   `306b` §6c defers, and it would have churned the `records30-glued-line-refuses-the-attempt` transcript
   — outside the brief's golden-identity gate. **Owed, named.**
3. **The forgiving-parser re-home (`306c` §3b steps 2–3) is NOT built.** Its destination is the
   report-only CONSUMER, which deviation 2 leaves unbuilt, so the re-home has nowhere to land; and its
   step 3 is a governed allow-list act. `307a:flg-allow-list-entry-not-added` stays open, unchanged.
4. **The render still computes its own decisions rather than reading them back.** `record_render_decisions`
   writes the audited three from the render's own seats, so a record cannot disagree with the artifact,
   but the render is not yet a pure Spine consumer. Making it one is the arrangement-home round's.
5. **Four `new` species are classified but not yet minted**, each with its seat named in the recorder's
   doc-comment: `SpineVouch` (the `Vouches` map exposes no iteration), `SpineObservation` (the `by_fact`
   merge is consumed by closure, not collection), `SpineValidityRound` (recording a round means deciding
   what a never-survives round may leave behind — `the-fixpoint-owns-the-rounds-and-builds-nothing-else`),
   and `SpineLoadDecision`'s custody column (contested/unprovable are recorded; custody is not).
6. **`WhyWorld`'s Spine carries dispositions and render decisions only** — the `new`-arm recorder runs in
   the binary driver. A why report's Spine is therefore narrower than a run's. Disclosed here rather than
   silently.

## §5 — Fences held

- NOTHING entered the durable arm: the census still reads 4/0/11, `apply[].leaf: u32` is unchanged, and
  the durable's bytes are measured identical. `lift-durable-siteid-keying` and
  `lift-durable-drop-accounting` (`30E` §9) stay OUT — drop-accounting is in-memory/render-only.
- The parallel custody lane's surfaces were never touched: `analysis::funcenv` keying,
  `spike/crates/oracle/` custody seats, `core::DefinitionCustody`, `verdict_cell_or_auto`'s slot.
- rec-5 (`probe-tape-not-a-cache`), `law-whylog-is-sensitive`, `two-plane-aid-law`,
  `operands-are-pure-and-capped`, and `306b` §6b (no influenced value gates engine control flow — the
  grade is `Option<InfluencePhase>` whose payload is `()`) are untouched.
- `309` §4's two smoke-diff fences hold: the dump is never the whylog, and never the `new`-arm debug
  dump — the latter is now a separate mechanism with its own lifetime and its own gate.

## §6 — Findings worth a conductor's attention

- **`fnd-classification-was-keyed-by-the-wrong-id-space`** [found and fixed in-lane]. The first cut of
  the `new`-arm recorder keyed `SpineSiteClassification` by `LeafId(cfg.node(node).ast.0)` — an AST id
  read as a leaf id. Those are unrelated integer spaces, so records would have keyed to other sites'
  identities. Fixed by bridging through the plan's own `ast → leaf` back-map. Worth recording because
  it is exactly the class of error `inv-site-keyed-results` exists to forbid, and no gate would have
  caught it: the record is `new`-arm, so nothing reads it yet.
- **`fnd-wsl-invocation-needs-native-cd`** [tooling, measured]. Driving the WSL leg as
  `wsl.exe -- bash -lc 'cd <path> && …'` silently runs in the PRIMARY CHECKOUT: the login profile execs
  zsh, `$0` reads `/bin/zsh`, `cd` returns 0 without moving, and `git` then answers for `ai/main`. A gate
  run that way is a false green against another branch's tree. `wsl.exe --cd '<Windows path>' -- bash -c …`
  lands correctly. Any lane driving WSL by hand wants the `--cd` form plus a branch assertion before the
  gate — `mise run both` is unaffected, this is the hand-rolled invocation only.

## §7 — Proposed steering-prose (conductor's to place; not edited by this lane)

- `spike/CLAUDE.md`, invariants — **`spine-is-the-one-decision-structure`**: every apparent product is a
  PROJECTION of Spine × the input files. `Plan`, the durable, and the artifact are derived, never
  assembled twice. A new product reads Spine; it does not accumulate its own fields.
- `spike/CLAUDE.md` — **`durable-census-is-closed`**: `SpineSpecies::census_arm` is a no-wildcard
  classification and ENTERING `CensusArm::Durable` IS the durable tripwire firing. A durable species
  reaches disk only through its `DurableView`, whose fields are the durable subset; a field no View names
  cannot be written.
- `crates/core/CLAUDE.md` — **`spine-grade-is-minted-not-remembered`**: `Spine::minted_at` stamps every
  record; a mint site never fills the grade in. v0's flip is positional and global, so per-record
  gradation stays open room rather than a decision anyone has already made.
- `crates/plan/CLAUDE.md` — **`plan-projection-demands-authority`**: `project_plan` takes a
  `PlanAuthority`, minted only by `PlanAuthority::authorise` over a non-refused admission (plus the
  named replay and intakeless mints). The binary driver must take its authority from its admission;
  `the_driver_takes_its_authority_from_its_admission` is the fence.
- `crates/plan/CLAUDE.md` — **`one-durable-grammar`**: v1 is deleted. `inspect` and the replay path read
  the same v2 the writer writes; a permissive second reader is what let a fixture grammar drift away from
  the product's.

## §8 — Successor context

- The smoke-diff (`crates/cli/tests/spine_baseline.rs`, `spike/spine-baseline-before.txt`, the
  `spine:baseline` task) is BUILD-TO-KILL and has now served its purpose: it is byte-identical. It dies at
  the fold review, together with `WhyWorld::spine()`'s stated reason for being public.
- The `new` arm has no accretion instrument (`309` §7 `pin-census-new-arm-hygiene`, still undecided).
  `Spine::population` and `debug_dump` are what an instrument would read.
- The four unminted species and the render-consumer residue (§4.4–4.5) are the natural next increment and
  need no new design.
- `306b` §2c's owed doc correction (timings are persisted TODAY, not a suggested addition — `30E`
  `fnd-timings-are-already-durable`) is still routed to the conductor and untouched here.
