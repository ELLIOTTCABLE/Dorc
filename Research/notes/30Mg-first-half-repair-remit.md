# 30Mg — first-half review repairs: builder remit (just-fix-it tier only)

> Tier: builder remit distilled from the `30M` adjudication. EVERY item here is
> adjudicated obviously-wrong AND design-free — nothing in this document needs a design
> sitting, and nothing design-tier from the review is in it (those live in `30M`
> §§1,3,4,5,9 and are the human's). One builder, one lane, one worktree. The dispatching
> conductor supplies the base tip (post-sibling-fold `ai/main`), the safety block, and
> step-zero/step-one per `spike/CLAUDE.md`.
>
> Standing riders: verify each finding still reproduces at YOUR tip before fixing it
> (the four evidence tests make this mechanical; a finding already fixed by the sibling
> fold is reported as such, not re-fixed). Flag any `tc-*`-shaped judgment call UP,
> never resolve it. No steering-prose edits (`CLAUDE.md`s, `FORFEITS`,
> `ANALYZER-NEEDS` are the conductor's). No golden blessing (`BLESS` is
> orchestrator-only — prepare drift, enumerate it, request a scoped bless at fold).
> Builders author zero user-facing prose: new diagnostics render `[unwritten:]`.
> Commit granularly; completion contract `mise run both gate:full-quiet`.
>
> Evidence branches (cherry-pick sources; both die after this remit folds):
> `worktree-agent-aae734f66adb32de5` — `c304dc99` (pre-source replay test) ·
> `5e614861` (dot-locals test); `worktree-sol-adversarial-30M` — `1dbca1ab`
> (discarded-trip test) · `176e0818` (sentinel-literal test; NB its fix is NOT in this
> remit — cherry-pick it only if the human's `30M` §5 ruling directs the value conjunct
> built; otherwise leave it on the branch for the ruling's consumer).

## R1 — pre-source dependencies stop replaying as roots

Defect (`30Mc:finding-transitive-pre-source-replays-as-root`, CRITICAL, demonstrated):
acquisition (`cli/src/main.rs:372–403`) promotes transitively-acquired files to
`SourceRole::NamedLoad` roots; `run_ambient_prefix` then re-executes their load programs
after the authored program finished, which can restore definitions the author removed
(`unset -f`) — engine-created vouch authority.

Fix (`30Mc:required-root-occurrence-identity`): acquisition retains the explicit
ordered pre-source ROOTS separately from files acquired for their load programs;
`push_ambient` receives only invocation roots; dependencies are reached at their
authored `.` positions by the root's own `LoadProgram` evaluation. This entails actually
evaluating the pre-source roots' load programs (include-guard control flow included) at
the `Entry` transfer — the fix `30Ib:dev-ambient-include-guards-are-not-evaluated`
already names. Default posture for an unresolvable act inside a prelude (conductor
default, human veto invited at fold): the WHOLE prelude floors from that point
(funcenv ⊤ onward) — sh-parity-shaped, strictly conservative.

Accept: cherry-pick `c304dc99`, un-ignore; it greens (`Withheld`). Add the reverse
cell (a definition made after a dependency's source stays positionally later). Existing
corpus byte-identical (its dependencies are idempotent — any movement is a finding, not
churn). Do NOT absorb `30Ib` §5.2/§5.3's variable-rooted-custody work; only unblock it.

## R2 — sourced top-level assignments reach the caller

Defect (`30Mc:finding-dot-locals-are-discarded`, demonstrated): `run_program` hands the
loaded program `&mut locals.clone()` (`analysis/src/funcenv.rs:1501–1531`), discarding
its top-level assignments — contrary to POSIX `.`, to
`30I:rul-dot-resolves-as-sh` [TYPED], and to the ambient-prefix path's own shared-map
design. Conservative today; breaks possible-load completeness for the bundle projection.

Fix: thread the loaded program's post-state back to its caller (the ambient path's
shared-locals design, applied to nested loads). CAUTION: funcenv precision work is
license-review-tier forever (`28Q` §1 — every precision change is winner-shifting);
keep the change minimal, and add the negative cell: assignments inside a
subshell-scoped source still die at the closing paren.

Accept: cherry-pick `5e614861`, un-ignore; it greens (`vendored/common.sh` in the taken
set, the role bound). Subshell negative cell green. Enumerate any corpus movement.

## R3 — the certifier-trip terminal cleanup runs in every plan producer

Defect (`30Md:fnd-discarded-trip-retains-elisions` + `30Mf` F1, convergent,
demonstrated): `plan::build_plan` (`lib.rs:3729–3733`), `hostsim` (`lib.rs:1511–1515`),
`coverage` (`lib.rs:592–596`), and `sweep` (`drive.rs:213–217`) discard their
`CertifierTrip` and project without `demote_on_trip`; `analysis::effect::classify`'s
convenience path discards a latch upstream. Violates `plan/CLAUDE.md
certifier-trip-cleanup-runs-in-every-driver` / `302:rul-certifier-trip-guard-only`.

Fix: every producer threads ONE latch through classification and settlement and runs the
terminal demotion before projection (a shared seat that makes this non-optional is
preferred over four copied calls, if it falls out naturally — but do NOT attempt the
typed-witness redesign; that is `30M:rec-dissolve-trip-must-remember-structurally`,
design-tier, not yours). Correct the false doc-comment on `demote_on_trip`
(`certifier_trip.rs:96–99`): the must-remember surface is NOT dissolved by the record;
say what is true.

Accept: cherry-pick `1dbca1ab`, un-ignore; it greens through a genuinely-threaded latch
(never by hand-constructing a tripped latch at the projection seat — anti-masking). Add
the fence: a test that enumerates plan-producing paths (or an assertion at the
projection seat) so a fifth producer cannot forget silently.

## R4 — the members-path certifier floor becomes a typed cause

Defect (`30Mb` §1 `fnd-members-floor-is-a-sentinel-not-a-type`; unreachable today,
double-contained, verified): a members-site solo-solve certification failure substitutes
a sentinel wall (`settle.rs:415–423`) whose unresolvability is an accident of the
current footprint lift, instead of the typed floor the standalone path takes; and it
narrates as `TotalWall` where the truth is our own solver defect (inverting `302` §5's
admin-honesty). No test covers the path.

Fix: thread the solo `SolveConsistency` into members freshness the way the standalone
seat threads (`floor_uncertified`-style); the floor is
`Freshness::Stale(StaleCause::SolveInconsistent)`; delete the sentinel substitution;
narration names the solver failure.

Accept: a new pin (members-solo-inconsistent ⇒ Stale(SolveInconsistent), never
FreshSurvived under any footprint population) that goes RED when the sentinel is
restored (mutation-check it once); corpus byte-identical (the path is unreachable).

## R5 — redirect-refused guards are disclosed like heredoc-refused ones

Defect (`30Mf` F2, conductor-verified): `collect_edits` drops a guard edit on
`leaf_has_heredoc || (is_guard && leaf_has_blocking_output_redirect)` (`lib.rs:4792–4794`)
but `refused_render_steps` (`lib.rs:4740`) checks ONLY heredoc, so the three disclosure
consumers (`render_refusal_diagnostics`, `refused_render_leaves`,
`render_refusal_narratives` — which also hardcodes `RenderRefusalTag::Heredoc` at
`:4687`) miss redirect-refused guards entirely; only `guard_refused_asts` uses the full
`guard_render_refused` predicate. The mutator runs verbatim (correct) with no
disclosure — contradicting the "ONE guard-refusal definition, kept in lockstep" contract
(`lib.rs:5250–5253`).

Fix: `refused_render_steps` adopts the full predicate for Guard steps; the cause rides a
`RenderRefusalTag` variant (a reason arm, never a sibling code —
`28L:rul-reason-enums-not-sibling-codes`; extending the existing diag payload with the
cause is fine pre-publication, `rul-strawman-formats-no-compat`); all four consumers
flow from the one seat again. Prose explicitly unwritten.

Accept: a case with a vouched guard site carrying `>>log` shows the refusal diagnostic,
narrative, and decision-plane record, and the why-lens does not claim "guarded";
X-heredoc byte-identical.

## R6 — Spine fields stop stating falsehoods

Defect (`30Mc` F3, all +SURE): `SpineSiteClassification.invalidator` is written from
`kills` alone (`main.rs:1766, 2432–2536`) against its documented "gens into reach"
meaning — false for every establish and opaque leaf; the `InlineCall` record maps its
ordered member account to empty `cells` (`main.rs:2527`); `SpineInvocation.mode` is
hard-coded `"whylog-replay"` from a writer unreachable on the actual replay branch
(`main.rs:2095–2119, 2552–2566`).

Fix: populate `invalidator` from the real final invalidator set; populate `InlineCall`
cells from the sites vector; make `mode` record the actual producing invocation (or
remove the unreachable writer). Where a field CANNOT truthfully mean what its doc says
(e.g. non-leaf invalidators have no site record), make the doc say the narrower truth —
and flag the representation question up rather than widening the record.

Accept: one unit pin per corrected field; the species census unaffected; no durable
contents change (`rul-durable-contents-reviewed-before-design` — if a fix would touch
what the `.whylog` persists, STOP and report).

## R7 — hygiene batch (each small; one commit each)

1. `pin28-variable-resolved-source-loads.loom`: re-spell per `30Ib`'s own named fix
   (`. "./$PKG.oracle.sh"`) and re-head the prose to argue what the case now proves
   (`30Mb` §4). Goldens: prepare, enumerate, request scoped bless at fold.
2. `prove_inline_replaceable` doc-comment (`plan/src/lib.rs:710`): remove
   `EstablishProbeWritten` from the blocker list; state the freshness-is-the-caller's-
   conjunct truth (`30Mb` §3).
3. `AbstractRc` doc (`plan/src/fold.rs:34–41`): "from a probed observable" is wrong for
   the static rc-0 mints — fix the doc, and add the negative pins for
   `erasure-is-records-grounded-only`: empty-list / funcdef / bare-assignment
   controllers never mint a `DeadBranchProof` (red-first against a future
   `subtree_leaves_all` widening). Attempt `30Me` F2's residual cell (a
   false-cond-`if`-no-`else` as controller); if unreachable, land it as a
   documented-unreachable test or drop with a note in your report.
4. `synthetic_cross_generator_consumer_map_holds` (`plan/src/survival.rs:2004–2054`):
   delete the self-asserting local closures; keep the two production assertions; rename
   to what it actually pins (`30Me` F3).
5. Red-first pin for `funcenv-reads-source-literal-plane-only` (`30Ib` §5.4): the wall
   is vacuously held today; pin it so `variable_before` gaining a `ValueGrade` gate is
   load-bearing, not a sentence (`30Mb` §9).
6. Env-exported-sentinel containment cell (`30Mb` §9): one cheap e2e/DST cell pinning
   that a host-environment sentinel value never moves the license plane (worst case
   rc-127/unknown ⇒ run).

## Sequencing and close

R1→R2 are one funcenv/acquisition territory — do them serially, first (they gate the
sibling's bundle-shape work). R3–R6 are independent. R7 rides last. On completion:
report per `spike/CLAUDE.md` (deviations OPEN, never self-endorsed; every reproducer's
before/after status; exact golden drift enumerated; both-leg gate results), and
propose — do not apply — any steering-prose sentences your fixes make stale.
