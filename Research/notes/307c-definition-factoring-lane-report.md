# 307c — The definition-factoring conversion: lane report

> Tier: builder lane report (Opus executors, 2026-08-16). COMPLETE for `notes/305` §2 and
> `notes/305a` §§2–3, with ONE item held open at a conductor checkpoint (§6).
> Authority: `notes/305` (work order) as amended by `notes/305a` (rulings); `plans/28Q`
> §1/§7/§8 rules both. The lane ran under three executors; this document is the whole
> lane's account, not any one seat's.

## §1 — What landed

Branch `ai/r30-lane-definition-factoring`, rebased onto `ai/r30-conduct`. Seventeen commits
beyond the lane's base: seven from the checkpoint executor, three from the first §2 executor
(`559178a1` · `ec0b6888` · `3e01c87b`), seven from the §3 executor (`f70890dd` · `84343853` ·
`9809a249` · `7808c3e5` · `d6400d96` · `3ef82607` · `c21d5652`).

### The seat table

The six ROLE-lane seats `oracle/CLAUDE.md` enumerates are converted, and so is the survival
lane's. Seat numbering follows `305` §1's enumeration.

| Seat | Before | After |
|---|---|---|
| 1 — effect-map lift (`lift_from_sets`) | `live_source` filter kept ONLY the whole-unit winner's rows | filter dropped; every file's rows survive, keyed `(file, provider, verb)` |
| 2 — `VerdictIndex::from_sets` | `live_source` filter kept one body per provider | filter dropped; keyed `(usize, ProviderId)` |
| 3 — `analysis::effect` predict lane (`live_predict_source`) | `live_source` scan + positional agreement gate + `idx.source_of == live` third condition | one `answering_file` call; third condition DELETED as structurally unnecessary |
| 4 — `analysis::effect` verdict lane (`verdict_cell_or_auto`) | `visible.role_answers(verdicts.source_of(p), …)` | `answering_file` over `verdicts.contains(file, p)` |
| 5 — the three cli ship closures (`shipping_source`) | `live_source(...).filter(answers_at)` | `answering_file` |
| 6 — `plan::build_vouches_from_sets` + `build_wrapped_vouches` | `live_source(...).filter(answers_at)` | `answering_file` |
| 9 — survival/footprint `disturbs` lane | three forward scans, first-file-wins AND first-that-RESOLVES | one shared `touches_answering_source` over `shipping_source`; a decline by the resolved definition is a decline |
| 9b — `sweep`'s duplicate resolver | its own forward scan | routes through `dorc_core::answering_file` with the honest `LiveDefinitions::unsolved()` posture |

A FOURTH consumer of the footprint resolution that `307c`'s original scoping map did not
list turned up during the conversion — `main::collect_reach_probes` (the reach-probe compile)
— and is threaded on the same terms.

Consequences, as `28Q` §1 predicted:

- **The veto is gone.** `LiveDefinitions::answers_at` had zero production callers once seats
  1–6 resolved, and is deleted. Its three tests are repinned onto
  `definition_before`/`provenance_of`.
- **`live_source` has one caller left**: `build_dialect`'s minting scan, reached through
  `lift_from_sets`. That is why the seat-law edit reads true today rather than aspirationally.
- **The chimera is unrepresentable, not gated.** A site's argparse and its cells are addressed
  by the same file index, so "identity through one author's arms, cells another author
  declared" (`271:rul-sin-ordering`, pope-sin tier) cannot be spelled. This is the whole point
  of the factoring and the reason seat 3's third condition could be deleted rather than kept
  as a belt.
- **The plural arm is now exercised end to end.** Four new corpus cells (§4) and the frame
  differential (§3) cover the world the byte-identity gate is silent about.

### `dec-dialect-keeps-a-whole-unit-fold`

`build_dialect` mints only from `KindIndex::dialect_minting_source` — per provider, the
pre-conversion whole-unit winner. Now that every file's rows live in the index, minting from
all of them would ENLARGE the dialect, and a larger dialect spares MORE: a silent
liberalization of the design's one naked-trust tier, in the dangerous direction (`28Q` §9
`pin-two-position-sparing`). The field is named for aggregation, not resolution, and its doc
says so; the seat law carries the parenthetical.

### `dec-the-dialect-fold-keeps-its-liveness-input` (NEW — the never-live retirement's shape)

`305a` §3 asks for "the `never_live` retirement end-to-end". Executed literally that SHIFTS
the sparing dialect, so the retirement landed as follows and the divergence is disclosed here
for ratification rather than assumed.

- RETIRED: the per-file WITHDRAWAL. `never_live_withdrawals` and the two `.withdrawing(&dead, …)`
  calls are deleted. Every site-keyed seat now declines a never-live definition BY RESOLUTION —
  the frame names a definition and a dead one is named at no frame — so subtracting its rows
  bought nothing the lookup was not already doing.
- KEPT: the liveness itself, as an input to the ONE whole-unit fold resolution does not cover.
  `lift_from_sets` gained `binds_somewhere`, and `build_dialect`'s minting scan runs over the
  definitions that bind.

Why the second half is not optional: the fold's decidable set makes `never_live` fire on the
polyfill idiom, where the LAST file to DECLARE a provider is precisely the dead body. Dropping
its liveness would hand that dead body the minting seat — vocabulary no execution could have
uttered, enlarging or shifting the sparing dialect. `305a` §1's dialect ruling states the
property it protects as "preserves the minting SET, not a resolution"; dropping never_live from
its input breaks that ruled property, so keeping it is that ruling applied consistently rather
than a new exception. ~SUSPECT this is what the ruling intended; it wants an explicit ack.

Pinned by `dorc-oracle`'s `a_never_live_definition_mints_no_dialect_tokens`, which asserts both
directions from one fixture. `305a` §1's rider — the funcenv certifier floor (`folded_edges = ∅`)
must survive on its own merits — is DISCHARGED by the pre-existing
`the_fold_breaks_to_its_floor_at_the_failing_round`, whose assertions (`!trusted()`,
`folded_edges().is_empty()`) name no consumer; its doc-comment's reason was widened, because
under true resolution EVERY environment answer is winner-shifting, not only this one.

### Law edits landed

- `oracle/CLAUDE.md`: `live-source-is-the-only-resolution-seat` →
  **`the-frame-lookup-is-the-only-resolution-seat`**, carrying the `dialect_minting_source`
  carve and the winner-shifting rider.
- `analysis/CLAUDE.md` `visibility-is-full-positional`: mechanism restated as RESOLUTION rather
  than agreement; the retired `source_of` carriage recorded.
- `analysis/CLAUDE.md` + `plan/CLAUDE.md`: the two "built at stage-0" claims → "RULED at stage-0
  and NOT YET BUILT", citing `fnd-stage-zero-is-not-built`.
- `analysis/CLAUDE.md` `never-live-subtracts-from-the-whole-unit-answer` →
  **`never-live-feeds-the-dialect-fold-only`**; `the-fold-decides-conditions-never-shapes` and
  `floors-are-whole-window-and-demote-only` re-grounded on the general winner-shifting property.
- `core/CLAUDE.md` `contested-is-write-once`: the never-live second use recorded as retired,
  with the write-once property kept for its own sake.

The winner-shifting rider is propagated into doc-comments at `core::definition` (module header),
`VisibleRole::answering`, `world::shipping_source`, and `survival::resolve_touches_footprint`.

## §2 — Findings

- **`fnd-stage-zero-is-not-built`** — stage-0 was never built; the commit that read as its
  landing restated LAW ONLY. `305` §2 item 8 (the stage-0 retroactive audit) is consequently
  STRUCK for this lane (`305a` §1) and stage-0 is its own queued lane. Full statement in
  `307` §2.
- **`fnd-reserved-name-error-does-not-refuse`** — the reserved-name error is MARKED, not
  REFUSED. +SURE for the lint trace; the ~SUSPECT can-answer-at-sites gap is disclosed and
  unproven by design. Landed as `5f9e5bc6`. This is why the authored/munged join-miss maps to
  `Unkeyed` (the ruled permissive arm) rather than to `Ambiguous`.
- **`fnd-survival-footprint-lane-scans-forward`** — a live wrong-elision route: the `disturbs`
  lane resolved by first-file-wins AND first-that-RESOLVES, so a declining live body fell
  THROUGH into a shadowed one's arms. A footprint answered by the wrong body can NARROW an
  at-most claim, and narrow SPARES MORE. **FIXED** (`f70890dd`), pinned by
  `the_footprint_answers_from_the_definition_the_frame_names` (verified red under the retired
  scan: it answered `first` where the frame names `second`).
- **`fnd-ship-predict-stage-is-not-in-world`** — DISCHARGED. Seat 6's composed-stage path routes
  through the shared `shipping_source` seat instead of an open-coded twin.
- **`fnd-sweep-duplicates-the-footprint-resolution`** — `sweep/src/drive.rs` carried its own
  `resolve_touches_footprint` beside `cli::survival`'s. **ADDRESSED, disclosed shape**: the two
  return different products (the cli's carries selectors and the emitting arm's span; the
  sweep's does not), so they were not folded into one function. What was single-seated is the
  RULE — both now ask `dorc_core::answering_file`, and the sweep passes the honest
  `LiveDefinitions::unsolved()` posture because it loads exactly one oracle and solves no
  environment. The ceremony earns its keep: a second sweep oracle now WITHHOLDS rather than
  silently taking the first.
- **`fnd-corpus-carries-twelve-plural-families`** — twelve textual plural cases; seven never
  load the second file; FIVE (`contest28-*` ×4, `guard23-reingest-collision-verbatim`) are held
  byte-stable ONLY by the contested withdrawal. The tripwire note lives on `ContestedFamilies`'
  test. **This dependency is load-bearing for the byte-identity gate**: any change that weakens
  the withdrawal moves those five cases. **REFINED by the census** (`307c:fnd-two-blessed-plural-
  cases-already-reach-the-seats`, below).
- **`fnd-two-blessed-plural-cases-already-reach-the-seats`** (NEW) — the plurality census
  measured, rather than assumed, which plural families reach the resolution seats with licenses
  intact, and found TWO that pre-date this lane:
  `contest28-polyfill-guard-defers-to-the-oracle` and `contest28-unset-f-blesses-elision`. Both
  are `28K` §1's BLESSED shapes, so the contested withdrawal correctly leaves them alone and the
  frame lookup has to answer them. `305a` §1 expected the census to land empty; it does not, and
  the difference is good news — the corpus was already exercising the plural arm end to end
  before this lane added four more.
- **`fnd-pin30-did-not-flip`** (NEW; `305` §2 item 6's stop-and-report branch) —
  `pin30-wrapped-case-bodied-in-book-verdict` is byte-identical after the whole conversion. Its
  probe-results file is still empty and gate-1 is still green, which is the alarm NOT firing.
  Per the brief this REFUTES `28Q` §1's asserted cause and is reported rather than chased.
  Supporting observation, offered as a lead and not a diagnosis: the hypothesis was that the
  wrapped lane reads ORACLE-ONLY vectors, but the binary's `build_wrapped_analysis` already
  takes `source_srcs`/`source_refs`/`source_paths` (source-wide, book included), so the
  oracle-only reading cannot be the cause at that seat. -GUESS the drop is somewhere between the
  wrapped peel and the verdict lane rather than in the vector choice.
- **`fnd-render-fixtures-keyed-cells-to-the-wrong-file`** (NEW; pre-existing, REPAIRED) —
  `mise run both gate:full-quiet` had never been run since the `(file, provider, verb)` re-key
  (`307c` §6 recorded it as owed), and it caught eight red tests in
  `crates/plan/tests/render_corpus.rs`. The hand-built indices `service_index`, `seam_index`, and
  `query_index` added their SECOND oracle's cells at file 0 while the classifier was handed that
  oracle at source index 1, so the site resolved to file 1 and read an empty cell slice. Repaired
  by keying each row to the file that declares it (`3ef82607`). Two things this is worth saying
  plainly: the re-key's own gate did not cover these targets (`gate:quick-quiet` runs
  `--lib --bins`), and the failure mode was silent LICENCE LOSS, which is the safe direction —
  had it been the other way the byte-identity gate would have been the only thing between it and
  a wrong elision.
- **`fnd-written-establishes-in-a-region-ship-no-check`** (NEW, measured, NOT chased) — an
  attempt to mint a hash-munge ACTIVATION cell (two guard bodies under one name in one artifact)
  failed because the shapes that produce two guards produce no probe: a site inside a subshell
  that classifies `EstablishWritten` ships no check at all (`unresolvable-no-probe`), and so
  takes no guard either, while the same class at TOP LEVEL does ship. Measured twice, from two
  book shapes. `28Q` §1 predicts the munge "becomes reachable exactly as its ledger predicted";
  on this evidence it does not become reachable through subshell regions, and the cell was
  dropped rather than forced. Whether that asymmetry is intended is a question for the seat's
  owner; it is NOT obviously wrong (a written establish is conservative), but it is the reason
  bitem1's machinery is still unexercised.
- **`fnd-git-deny-hook-blind-to-midrebase`** (tooling) — `git-deny.mjs`'s `isAutonomous()` reads
  `git branch --show-current`, which returns EMPTY under a mid-rebase detached HEAD
  (**measured: `branch=[]`**). Neither autonomy clause can then match, so `git rebase --continue`
  is refused — and so is `git rebase --abort`, leaving a conflicted rebase resolvable only from
  outside the session. The same predicate gates `git commit`. Proposed fallback, unapplied
  (harness config is human-owned):

  ```js
  const rebaseBranch = () => {
     for (const p of ["rebase-merge/head-name", "rebase-apply/head-name"]) {
        const f = sh(`rev-parse --git-path ${p}`)
        if (f && existsSync(f))
           return readFileSync(f, "utf8").trim().replace(/^refs\/heads\//, "")
     }
     return ""
  }
  const branch = sh("branch --show-current") || rebaseBranch()
  ```

  `rev-parse --git-path` resolves per-worktree git dirs correctly. ~SUSPECT complete for the
  rebase case; a plain detached `git checkout <sha>` still reads empty.
- **`fnd-cargo-autodiscovery-ingests-sync-residue`** (tooling; FIXED in `ec0b6888`) —
  `spike/crates/plan` had no `autotests = false`, so cargo autodiscovered
  `crates/plan/tests/*.sync-conflict-*.rs` as test targets. Such a copy's name is not a legal
  crate identifier, so the workspace lint gate fails and **every commit in the tree** is blocked
  until a human clears the residue. Fixed by mirroring `crates/cli`'s declaration.
- **`fnd-hk-fix-stages-untracked-sync-residue`** (NEW, tooling; live hazard for every lane in
  this tree) — `mise run fmt` (`hk fix --all`, which re-enables fix mode for itself per
  `fmt-is-a-task-in-every-session`) **`git add`s files matching its step globs, including
  untracked `*.sync-conflict-*.rs`**. Measured once in this lane: a clean `mise run fmt` left
  nine sync-conflict copies STAGED, one `git commit -a`-shaped mistake away from committing
  somebody's conflict copies into the corpus. Recovered by an explicit-path `git reset`. The
  pathspec law ("`git add` uses EXPLICIT FILE paths only") does NOT protect against this,
  because the staging happens inside a mise task rather than at a `git add` call site. Two
  mitigations worth considering, neither applied: teach `hk`'s globs to exclude
  `*.sync-conflict-*` (the same exclusion the corpus walkers and now `crates/plan`'s target
  declaration already carry — it is becoming a repo-wide idiom that deserves ONE home), or have
  agent sessions verify staging after every `fmt`. The first is the real fix.

## §3 — The new battery (`crates/cli/tests/definition_frames.rs`)

Four properties, all ordinary-harness tests over the committed corpus. The reason they exist
rather than more goldens: `syn-single-frame-byte-identical` asks the corpus to be byte-stable,
and the corpus is very nearly single-frame, so the migration gate is silent EXACTLY where the
new machinery decides.

- `every_lifted_role_row_joins_to_a_parsed_definition` — the pre-existing join census (the
  conversion's precondition).
- `the_engine_names_the_definition_the_shells_ran` — THE FRAME DIFFERENTIAL. For each
  `floor30-*` cell it feeds the loom's sourced sections back as inputs under the exact path
  strings the book spells, solves the real environment, and asserts that at every site the file
  `LiveDefinitions::source_before` names is the one whose body printed that site's committed
  `expected.emitted` token. The token→author mapping is MECHANICAL (the section containing
  `printf '<token>\n'`), never a committed table, so a re-authored body cannot drift it. `gone`
  asserts the negative half: after `unset -f` the environment must name NOTHING. Three cells,
  eleven sites, plus structural coverage floors. Verified red under
  `LiveDefinitions::unsolved()`.
- `a_contested_helper_closure_withholds_the_role_body` — the two floor30 HELPER cells as
  WITHHOLD cells, per `305a` §3: where one helper name holds differing bodies across frames the
  load edge owes a `helper-declaration-contested` and `closure_for` on the role's own body
  REFUSES. Classification into role-cells and helper-cells is mechanical (does ≥2 inputs declare
  the role), not a name list.
- `every_reachable_plural_family_is_an_enumerated_plural_idiom` — THE PLURALITY CENSUS,
  allow-list-shaped per `305a` §1. Load-set-modeled (the `*.oracle.sh` glob the e2e runner
  itself turns into `-o` arguments, plus the book) and withholding-aware (it mints the contested
  set through the same two `funcenv` calls in the same order the cli edge uses). Two-way: a
  listed case that stops carrying a reachable plural family reddens as stale.

`task-verify-definition-vector-walls` (`28R:§snapshot` residue) is discharged in the same file
by `a_book_definition_vector_walls_its_own_call_site`: a book-level definition under an
oracle-described tool's name walls its own call site, with the unshadowed control asserting the
license was demonstrably there to lose, and both a mutating and an inert body asserted so the
wall is the VECTOR's rather than the spliced body's own opacity.

## §4 — Sanctioned goldens (named individually)

The byte-identity gate held: **no committed golden moved**, on either leg, at any point in this
lane. `git diff --stat f81609ef..HEAD -- 'spike/crates/*/tests/*'` touches six files, and four
of them are new:

- `frame30-subshell-body-answers-inside-only.loom` — NEW. Two sites, one role name, TWO live
  bodies; the probe artifact ships the book's regional body for the in-region site and the
  oracle's for the site after it. This is the conversion's payoff, pinned.
- `frame30-region-removal-dies-at-the-paren.loom` — NEW. An in-subshell `unset -f` is
  frame-local: the in-region site runs on its own bytes (nothing is live, silence licenses
  nothing) and the post-region site keeps its check.
- `frame30-nested-region-inherits-the-outer-body.loom` — NEW. Frames STACK: the nested site
  ships the OUTER region's body, the trailing site the oracle's.
- `frame30-a-regional-decline-is-a-decline.loom` — NEW. The retired decline-fallthrough cascade's
  end-to-end grave: the region's body declines every argv and the site RUNS rather than borrowing
  the oracle's answer.
- `definition_frames.rs` / `render_corpus.rs` — `.rs` tests, not goldens.

`pin30-wrapped-case-bodied-in-book-verdict` did NOT flip (`fnd-pin30-did-not-flip`); no record
was authored for it and no golden was re-blessed anywhere. Each new cell was blessed through the
ordinary scoped path (`BLESS=1 … --test e2e -- <substring>`), which the standing rider
pre-authorizes for exactly this class.

**Owed against `305a` §3's "six new cells": FOUR landed.** The two not minted are the
hash-munge activation cell and a helper-collision cell; both are blocked on measured obstacles,
not on budget — `fnd-written-establishes-in-a-region-ship-no-check` for the first, and for the
second the fact that `HelperIndex` indexes top-level declarations of LOADED SOURCES, so a helper
redefined inside a book region never enters it and the end-to-end world cannot be spelled today.
The closure floor is covered at the differential tier instead (§3), which `305a` §1 sanctions
for seat 9's coverage and which reads as the same trade here.

## §5 — Scope notes and disclosures

- **`tc-lift-diags-now-span-every-file`** (ENDORSED by the conductor) — dropping
  `lift_from_sets`' `live_source` filter means `derive_predict`'s diagnostics are now surfaced
  for EVERY file's rows. Aid-plane only, no license effect, and invisible on today's corpus.
- **`dec-plurality-withhold-repin`** — `effect.rs`'s
  `the_live_definitions_reason_is_the_one_reported` FAILED under the conversion: with no
  environment, two competing definitions now withhold instead of the last one winning. Repinned
  as `competing_definitions_without_an_environment_withhold_in_either_order` (symmetric, so it
  asserts an invariant rather than which expedient is in force), with
  `a_sole_definition_without_an_environment_still_answers` beside it so the pin stays a statement
  about PLURALITY. Reaches only hand-built indices and the instrument/hint lanes, which become
  more conservative — the safe direction.
- **`dis-solved-environment-cousin-dropped`** — the conditionally-authorized solved-environment
  cousin test was DROPPED, disclosed per its ruling; its coverage lives at the differential tier
  instead, which is strictly stronger.
- **`dis-survival-vectors-stay-oracle-only`** — seat 9's conversion threads the FRAME into the
  survival lane; it does NOT widen that lane's vectors, which stay oracle-only. A site whose
  `__disturbs` a BOOK defines therefore resolves to a definition the touches vector cannot hold
  and answers nowhere — no footprint, the site walls. That is the safe half of
  `cli/CLAUDE.md the-book-is-a-definition-source`, and widening is still its own dispatch.
- **Comment budget** (whole lane, `f81609ef..HEAD`; doc-comments counted separately):
  **424** added lines in `spike/crates/*/src/*.rs`, of which **21** are plain `//` (**4.95%**,
  under the ≤10% budget) and **114** are `///`/`//!` doc-comments. Commands:
  ```
  R=f81609ef..HEAD
  git diff $R -- 'spike/crates/*/src/*.rs' | grep -c '^+[^+]'
  git diff $R -- 'spike/crates/*/src/*.rs' | grep -cE '^\+\s*//([^/!]|$)'
  git diff $R -- 'spike/crates/*/src/*.rs' | grep -cE '^\+\s*//(/|!)'
  ```

## §6 — HELD AT A CHECKPOINT (the one item not landed)

Two items are gated on ONE conductor ruling, held per `305a`'s deviation praxis rather than
acted on with a justification.

**`ask-does-survival-route-through-the-withdrawal-edge`.** Seat 9's frame resolution landed;
its WITHDRAWAL half did not. The conductor's brief requires the exact law text plus the proposed
edit before any withdrawal-edge change, because getting it wrong reintroduces a half-withdrawal.

The law, verbatim (`spike/crates/cli/CLAUDE.md`, `withdrawal-is-applied-once-never-consulted`):

> Known seats OUTSIDE the edge (`28P:res-whyworld-and-survival-do-not-withdraw`): `WhyWorld` and
> `survival` lift their own ORACLE-only vectors and apply NEITHER withdrawal — benign only while
> those vectors stay oracle-only; widening either seat to the source-wide list must route it
> through this edge first.

The same file's `one-helper-index-two-lanes` prices the survival/kind lanes' widening as "its
own dispatch", and `plan/CLAUDE.md` is silent.

The two gated items:

1. **The survival touches sets are not withdrawn.** `lift_touches_sets` and `touches_paired` lift
   from `oracle_refs` and never see `contested`. Reachability assessment, offered as evidence and
   not as a licence to skip the ruling: a contested family's sites classify `Opaque ⇒ MustRun`, and
   `build_survival_footprints` only considers establish-bearing classes and kills, so a contested
   family's own sites are not footprint candidates and the gap looks UNREACHABLE today. ~SUSPECT,
   not +SURE — it rests on a class analysis, not on a type, and `withdrawing-drops-detected-too`
   exists precisely because half-withdrawals are found later.
2. **The whyworld frame-unification** (`305a` §1, in scope) is the SAME ruling. `28P:res-the-why-
   world-cut-is-now-visible` prices it as "re-lifting that seat's whole world" — i.e. widening
   `WhyWorld` to the source-wide vectors so it answers as the run does — and that widening is
   exactly what the quoted law says "must route it through this edge first".

Proposed edit, for the conductor to rule on before it lands:

> **withdrawal-is-applied-once-never-consulted** … Known seats OUTSIDE the edge: `WhyWorld` lifts
> its own ORACLE-only vectors and applies NEITHER withdrawal — benign only while those vectors stay
> oracle-only; widening that seat to the source-wide list must route it through this edge first.
> The SURVIVAL lane's `disturbs` vectors are withdrawn AT this edge like every other lifted set
> (`28K` §1's `TouchesSet::withdrawing` forwarder), so no seat downstream has to remember to ask;
> its vectors remain oracle-only, which is a separate question from whether they are withdrawn.

Ruling asked for, in one line each: (a) may the survival touches sets be withdrawn at the edge
under that wording, and (b) does the whyworld source-wide re-lift land in this lane or become its
own dispatch? A follow-up executor can land both in well under an hour with the ruling in hand;
the gates below must then be re-run.

## §7 — Gate evidence (per leg, FOREGROUND)

| Step | Leg | Result |
|---|---|---|
| `mise run check-quiet` | Windows | clean |
| `mise run both preflight gate` | both | clean |
| `mise run gate:full-quiet` | Windows | **2096 / 2096** passed, 1 skipped (incl. a `cargo clean` + full clippy + doctests) |
| `mise run gate:full-quiet` | WSL | **2092 / 2092** passed, 1 skipped |
| `mise run bless:dry` | Windows | clean, zero golden writes |
| `mise run test:floor` (`DORC_E2E_FLOOR_SHELLS=dash,posh`) | WSL | **145 / 145** |
| `mise run test:e2e` drift check | Windows | zero tracked modifications after every run |

The byte-identity gate (`syn-single-frame-byte-identical`) held throughout: after each e2e run
`git status --short` reported no modified tracked file, and the whole-lane case diff touches only
the four new cells and the two `.rs` test files (§4). No `expected.*` section was re-blessed, and
no certification `Refused` appeared on the new shapes.

Residue in the tree: eleven untracked `*.sync-conflict-*` files, human-owned, deliberately
untouched (`fnd-hk-fix-stages-untracked-sync-residue` is why they are worth a mitigation).
