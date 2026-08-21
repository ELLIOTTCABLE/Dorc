I have everything needed. Below is the report. Per the task brief's strict read-only rule ("DO NOT edit, create, or delete any file") I have not written it to `Research/notes/30Mf-first-half-review-deepseek-adversarial.md`; the report text is the final message, ready to file.

---

# 30Mf first-half review — deepseek adversarial (read-only)

Scope: the round-30 effective-world-reach kernel rework — `plan` (settle, world, decision seats, certifier_trip, fold, erase), `analysis` (effect, cfg, load, funcenv, solve, certify, erase), `core` (custody, claim, loadpath, influence), and their drivers/tests. Confidence words per the mandated scale. Every finding cites `file:line` and the law text it rests on.

## Finding 1 — `demote_on_trip` is absent from three plan-producing drivers and the `build_plan` entry, violating `certifier-trip-cleanup-runs-in-every-driver` (severity: medium)

**Law** (`spike/crates/plan/CLAUDE.md:129-137`):
> **certifier-trip-cleanup-runs-in-every-driver** (`302:rul-certifier-trip-guard-only`) — `certifier_trip::demote_on_trip` runs immediately after `build_plan_walled` in EVERY plan-producing driver; a NEW driver MUST call it. On a tripped run, Replace and Omit demote to run (`DemoteTag::CertifierTripped`, …); guards STAND only on the syntactic occupancy-1 census over `DefinitionTable` — the census consults NO solve, and that independence IS its admissibility (a trip disqualifies solver and certifier together; nothing either touched may testify).

**What the code does.** `demote_on_trip` (`plan/src/certifier_trip.rs:73-108`) mutates the spine: `Run` stands (line 78), `Guard` stands only if `census_unique(license.insert().fn_name())` (line 79), `Replace(..) | Omit { .. }` demote to `Run` (line 80). A grep for the symbol across the whole `spike/` tree shows it is called from exactly one driver:

- `cli/src/world.rs:667` — inside `demote_on_certifier_trip`, gated on `trip.tripped()` (line 664). This is the only correct caller.

It is *not* called by any of the other three plan-producing drivers, all of which construct a fresh trip, discard it, and project immediately:

- `hostsim/src/lib.rs:1500-1516` — `build_plan_walled(..., &mut CertifierTrip::default(), ...)` (1511), then `project_plan` (1515). No `demote_on_trip`.
- `sweep/src/drive.rs:196-217` — same shape (trip default at 213, project at 217).
- `coverage/src/lib.rs:579-596` — same shape (trip default at 592, project at 596).
- `plan/src/lib.rs:3717-3733` — the non-walled `build_plan` compatibility entry: `build_plan_walled(..., &mut CertifierTrip::default(), ...)` (3729), then `project_plan` (3733). No `demote_on_trip`.

I'm +SURE of the mechanical fact (the grep and the four reads confirm it). The law's "every driver / a NEW driver MUST call it" is plainly contradicted in three existing drivers plus the entry point.

**Sharper than a missed call.** The doc on `demote_on_trip` itself claims the must-remember surface was dissolved by reification (`certifier_trip.rs:96-99`):
> `dec-certifier-trip-cleanup` (`30E` §3) lands in the decision plane rather than staying a post-construction mutation nobody records: … a NEW driver forgetting to call it is exactly the must-remember-to-ask surface the reification dissolves.

But the reification only moved the *record* (`push_render_decision`, `certifier_trip.rs:100-106`) into the decision plane; the demotion itself (`record.decision = Disposition::Run`, line 85) is still a separate post-`build_plan_walled` mutation that drivers must remember to invoke. The must-remember surface is not dissolved — it is intact, and three drivers have already forgotten it. I'm +SURE the demotion is a separate call, not folded into `build_plan_walled`.

**What actually breaks.** I traced the interaction with the mid-pipeline §3 floors (which the law says still fire in place): on a tripped run `settle.rs` latches `SolveInconsistent`, making `consistency.is_consistent()` false, so `valid` is false everywhere (settle.rs:434) and `floor_uncertified` returns `Freshness::Stale(StaleCause::SolveInconsistent)` (settle.rs:560-569). `establish_disposition` then turns a Stale-but-licensed site into a *guard*. So on a trip:

- Replace is already floored to run by the freshness path — +SURE.
- The fold-`Omit` body is independently held verbatim by the render's `is_neutralised(controller)` gate (a tripped controller is not a `Replace`, so `is_neutralised` is false and the Omit body renders behind the live guard) — ~SUSPECT, verified in the prior session.
- The residue is **guards standing on a trip whose verdict funcname occupancy ≠ 1**. In the CLI, `demote_on_trip` demotes those to `Run` (the census is the only admissible testifier after a trip, per the law). In hostsim/sweep/coverage/`build_plan`, they stand.

A concrete wrong-elision world (~SUSPECT, not +SURE because it needs the trip plus an occupancy-ambiguous binding plus a guard, and it does not reach a production artifact): an engineer ships two same-named `nginx__is_converged` definitions (e.g. an oracle body shadowed by a book-sited one), the solver/certifier disagree and trip, and a guard was minted off a vouch for the establish. The CLI demotes that guard to run; the instrument drivers leave it standing, and the runtime frame resolution binds `nginx__is_converged` to a body whose rc-0 "converged" skips a mutator that should have run.

**Severity is bounded and I'll say why.** None of the four non-CLI sites emits an executable apply artifact: `hostsim` is the DST harness (and `raced_plan` passes `Vouches::new()`, so it cannot even mint a guard), `sweep` is the yardstick elision-frequency table, `coverage` is the analyzer-coverage rollup, and `build_plan`'s only in-tree callers are `#[cfg(test)]` blocks and tests (`plan/tests/*`, `hostsim/src/lib.rs:1120/1257/1368`, `cli/src/why.rs:3034`, `cli/src/main.rs:6630`). So there is no production wrong-elision today; the cost is (a) a hard law violation, (b) instrument output that can disagree with the production disposition on a tripped run, and (c) the drift hazard the law names — the pattern a future driver will copy is already broken in three places. The repair direction the law implies (fold the demotion into `build_plan_walled` so the "forget" surface is actually dissolved, per `certifier_trip.rs:96-99`'s own claim) is the obvious fix; I'm flagging, not prescribing.

## Finding 2 — a guard refused for a blocking output redirect is dropped from the edit but not disclosed, contradicting `guard_render_refused`'s "ONE definition, kept in lockstep" claim (severity: low)

**Law/contract** (`plan/src/lib.rs:5250-5253`):
> Is this leaf's GUARD render refused (run verbatim + disclosed)? … The ONE guard-refusal definition, kept in lockstep across [`Plan::collect_edits`] (drop the edit), [`Plan::render_refusal_diagnostics`] (disclose it), [`Plan::guard_refused_asts`] (the why-lens suppresses the "guarded" claim), and the cli's guard why-lane.

Backed by the `collect_edits` comment (`plan/src/lib.rs:4787-4790`):
> A GUARD ALSO refuses a non-devnull output redirect (`>>log`) — the guard's pass-direction would suppress the admin-spelled side-effect (23C-fd10). Both run VERBATIM (kFAIL-perform; disclosed by `render_refusal_diagnostics` + the cli guard why-lane).

**What the code does.** `collect_edits` refuses the guard edit on heredoc *or* redirect (`plan/src/lib.rs:4792-4793`): `leaf_has_heredoc(...) || (is_guard && leaf_has_blocking_output_redirect(...))`. That refusal path is correct and safe (the mutator runs verbatim — the right direction).

But the disclosure machinery does not share that predicate. `guard_refused_asts` (`lib.rs:4531-4539`) correctly uses `guard_render_refused` (both conjuncts). The other three consumers all route through `refused_render_steps` (`lib.rs:4727-4750`), whose only refusal check is `leaf_has_heredoc(ast, step.ast)` at line 4740 — no `leaf_has_blocking_output_redirect`. The consumers:

- `render_refusal_diagnostics` (`lib.rs:4643-4645`) → `refused_render_steps`.
- `refused_render_leaves` (`lib.rs:4697-4702`) → `refused_render_steps`.
- `render_refusal_narratives` (`lib.rs:4677-4692`) → `refused_render_steps`, and additionally hardcodes `RenderRefusalTag::Heredoc` at line 4687.

I'm +SURE of each of these reads. The CLI's own disclosure path uses the gap: `world.rs:490` and `main.rs:1978` call `render_refusal_diagnostics`, and `world.rs:497`/`main.rs:1919` chain `render_refusal_narratives`.

**Consequence.** A guard whose edit is refused because guarding would suppress an admin-spelled `>>log` side-effect runs verbatim (correct), and the why-lens correctly stops claiming "guarded" for it (`guard_refused_asts`). But the *positive* disclosure — "this line was refused a guard, and here is why" — is silently missing: no `render_refusal_diagnostics` line, no `refused_render_leaves` record, no narrative. The admin sees an unchanged mutator line with no explanation that Dorc tried and declined to guard it. This is a disclosure/attention-honesty gap (`rul-attention-honesty`: a may-execute line is never hidden; `attention-honesty-here`: a refused license is refused "loudly"), not a wrong-elision — the mutator runs. It is also a plain contradiction of the doc's "ONE guard-refusal definition, kept in lockstep" — three of the four named consumers do not use that definition.

## did not hold:

- **`Derivation.ambient` is provenance-only; the `elision-predicate` law's `ambient` conjunct is now enforced elsewhere.** The law lists `ambient` as a live elision conjunct (`plan/CLAUDE.md` `elision-predicate`: `probe(L.fact) = Converged ∧ ambient ∧ Must ∧ …`). The field `Derivation.ambient` (`lib.rs:290`) is written at five construction sites (579, 627, 690, 779, 801), read only for display (`render.rs:647`), canonicalization (`erasability.rs:220,238`), and one test assertion (`lib.rs:6887`) — never read to decide a license. The ambient gate still exists, but as apply-time freshness (`Freshness`), per `lib.rs:490` ("its per-cell ambient-ness as apply-time freshness is exactly the split `30K` closed") and `lib.rs:5541-5542` ("the origin ambient/written split no longer gates the tier"). No wrong-elision; the law text is simply stale about which mechanism is the `ambient` conjunct and could be updated to point at `Freshness`.
- **Fold-`Omit` not validity-gated at the decision seat.** I initially suspected the fold-Omit branch in `site_conclusion` (gated only on `!has_top_successor` + `fold.dead_controller`, not `valid_at`) could elide past an invalid controller. Traced and closed: the render's `is_neutralised(controller)` gate holds the body verbatim behind the live guard whenever the controller is not a genuinely-substituted `Replace`, so an invalid controller renders verbatim rather than editing the dead body to `:`. Correct fail-safe, not a finding.

---

Nothing in this review contains memetic-hazard material. Two findings, ordered by severity: a hard "every driver" law violation with a guard-on-trip residue confined to test/instrument drivers (medium), and a guard-refusal disclosure gap contradicting a lockstep claim in the production why/diagnostics lane (low). The report was not written to disk, respecting the read-only rule; apply this text to `Research/notes/30Mf-first-half-review-deepseek-adversarial.md` if you want it filed.