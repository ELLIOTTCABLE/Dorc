# 30Md — Round-30 first-half adversarial review (Sol)

> Scope: commit range `68709783..3011daae`, reviewed at `3011daae`.
> This is an engineering-correctness review of the analysis kernel and its
> planned seams, not a security review. Certainty grades are `+SURE`,
> `~SUSPECT`, `-GUESS`, and `--WONDER`.

## 30Md:fnd-discarded-trip-retains-elisions

Severity: High. Confidence: +SURE about the violated authority floor and the
conditional wrong-elision outcome; +SURE that the primary CLI and `WhyWorld`
paths are not affected by this call-order defect.

Locations:

- `spike/crates/plan/src/lib.rs:3695` (`build_plan`), especially lines
  3729–3733;
- `spike/crates/hostsim/src/lib.rs:1500`, especially lines 1511–1515;
- `spike/crates/coverage/src/lib.rs:579`, especially lines 592–596;
- `spike/crates/sweep/src/drive.rs:196`, especially lines 213–217; and
- the earlier independent loss at `spike/crates/analysis/src/effect.rs:1764`,
  especially line 1788.

Binding law attacked, `spike/crates/plan/CLAUDE.md:129`:

> `certifier_trip::demote_on_trip` runs immediately after
> `build_plan_walled` in EVERY plan-producing driver; a NEW driver MUST call
> it. On a tripped run, Replace and Omit demote to run.

The [TYPED] parent rule is equally direct (`Research/plans/302-solve-certifier-spec.md:200`):
one boolean is shared by the analysis spine, and a final cleanup immediately
before plan emission demotes every elision-family outcome because a certifier
disagreement disqualifies both solver and shared substrate.

Each listed producer passes a freshly-created temporary `CertifierTrip` into
`build_plan_walled` and then projects the returned Spine. The temporary is
dropped at the call boundary, so no code can observe `tripped()` and the
mandatory terminal demotion never runs. The `analysis::effect::classify`
convenience path independently throws away another latch before these producers
build a plan. This is therefore neither one boolean per analysis spine nor a
terminal guard-only floor.

The primary binary driver is the control: it creates one latch at
`spike/crates/cli/src/main.rs:1039`, threads it through analysis and settlement,
calls `demote_on_certifier_trip` at line 1839, and only then projects at line
1843. `WhyWorld` does the same at `spike/crates/cli/src/world.rs:195` and
484–488. The finding is not that ordinary `dorc plan` currently omits a command
after a trip. It is that the public kernel entry and three correctness
instruments silently retain exactly the licenses the certifier rejected.

Failing world and committed demonstration:

1. A converged, reached, vouched `apt-get install -y nginx` gives the real plan
   predicate enough evidence to mint `Replace`.
2. A solver returns an inconsistent fixed point: on a one-node self-loop it
   claims bottom while its transfer returns `Elem(1)`. The real
   `certify_solution` rejects it and trips the real latch.
3. A listed producer follows its current order: build Spine, discard the latch,
   project. The `Replace` survives, so the needed install can be absent even
   though the checker detected that the analysis licensing it was inconsistent.

Commit `1dbca1ab` adds the ignored red test
`a_tripped_plan_projected_without_cleanup_must_not_retain_elision` at
`spike/crates/plan/src/certifier_trip.rs:385`. It uses both the genuine checker
disagreement and an end-to-end plan-minted `Replace`. Running
`mise exec -- cargo test -p dorc-plan a_tripped_plan_projected_without_cleanup_must_not_retain_elision -- --ignored`
fails because the projected plan still contains an elision. Normal gates do not
run ignored review reproducers.

This also poisons the gates' witnesses: host simulation and sweep are intended to
detect plan/bare divergence, but their own plan generator suppresses the alarm
that says its analysis is untrustworthy. A green instrument run is therefore not
evidence against this class.

## 30Md:fnd-sentinel-literal-never-participates

Severity: Medium. Confidence: +SURE that the implemented route disagrees with
POSIX sh and the [TYPED] rule; ~SUSPECT that this mismatch alone can license a
wrong command in the current product, because exact live-helper resolution is a
separate downstream fence.

Locations:

- `spike/crates/analysis/src/funcenv.rs:1599`, where
  `LoadCondition::Value { name, equals, .. }` discards the literal;
- `spike/crates/analysis/src/funcenv.rs:1674` (`sentinel_decides`), whose
  signature has no literal or compared value;
- `spike/crates/analysis/src/funcenv.rs:1708`, which asks only the name-keyed
  `sole_populator`; and
- `Research/notes/30Ib-static-loading-lane-report.md:532`, where the builder
  explicitly chose that "a value comparison never happens."

Human-ratified design attacked, `Research/plans/30I-static-loading-and-bundle-emission.md:152`:

> on the no-source/reuse route, both the guard-tested value that selected that
> route and every transitively load-bearing helper on the REACHED vouch path
> `Must`-originate inside the exact fallback target closure.

Section 3.4 repeats the requirement at lines 696–708 and assigns the
value/helper-unaligned case to `dependency-source-act-present-but-unaligned`,
which licenses nothing. The ruling ledger at lines 1287–1290 marks this exact
value-and-helper provenance as human-ratified.

The landed recognizer proves only that the fallback closure is the sole authored
unit assigning the variable name. `LoadProgram::assigns` is deliberately
name-only. It never checks that the closure assigns the literal appearing in the
guard, nor that the live assignment which selected reuse equals that literal.
The builder report's claim that the value question was "dissolved" is a
consequential implementation default contradicting an existing [TYPED] ruling,
not an authorized narrowing of it.

Failing world and committed demonstration:

```sh
. ./common.sh       # assigns sm_common_loaded=sm.common/v1
. ./alpha.sh        # guards against sm.common/v2
```

with `alpha.sh` containing the admitted shape:

```sh
[ "${sm_common_loaded-}" = sm.common/v2 ] || . ./common.sh
```

POSIX sh takes the source arm in `alpha.sh`, so `common.sh` runs twice. The
analyzer records the second occurrence as `LoadRoute::Reused`, meaning no `.` ran.
Commit `176e081` adds the ignored red test
`a_mismatched_sentinel_literal_must_take_the_source_arm` at
`spike/crates/analysis/src/funcenv.rs:4119`. Running
`mise exec -- cargo test -p dorc-analysis a_mismatched_sentinel_literal_must_take_the_source_arm -- --ignored`
fails with `left: ["common.sh"]`, `right: []` for the unexpected reuse route.

The current independent definition-resolution fence is why this report does not
claim a demonstrated wrong command elision from the specimen. The consequential
present defect is that the one load account already states the wrong branch, while
`30I` requires function environment, custody, emission, and bundle compilation to
agree on it. The not-yet-built bundle compiler is planned to consume those
occurrence-keyed decisions. Building that projection atop this seam would either
reproduce the false reuse result or require a second resolver forbidden by the
one-loader rule. This is accidental product-level lock-in unless corrected before
artifact work treats the account as firm.

## Did not hold

- `30Md:did-live-cli-drop-trip-cleanup` — +SURE false: `main` and `WhyWorld`
  both demote before projection; the trip finding is scoped to alternate/public
  producers and instruments.
- `30Md:did-aggregate-repair-remain-missing` — +SURE duplicate/false for this
  review: the known `30La` aggregate verdict-primacy defect is already recorded
  and under directed repair; the later universal member tests and settlements
  are present in the reviewed range.
- `30Md:did-effective-reach-read-public-disposition` — +SURE false after the
  repair: the settled implementation derives private semantic act and output
  disposition from one joint decision rather than feeding `Disposition` back
  into reach.
- `30Md:did-speculative-load-mint-speaker` — +SURE false: speculative
  occurrences remain in the possible-load projection and are excluded from the
  speaker projection.
- `30Md:did-loader-cycle-license-vouch` — +SURE false in the current
  composition: cycle/depth exhaustion reaches `EnvStack::Top`, and definition
  resolution withholds before a vouch can consume the recorded edge.
- `30Md:did-plan-leaf-projection-collapse-current-members` — +SURE false for
  the current population: one disposition per leaf still holds. `30L` already
  schedules the region/route identity widening before artifact forms; it remains
  a seam to preserve, not a present fault.
- `30Md:did-render-decision-residue-hide-itself` — +SURE false as a new
  finding: render decisions are still written back after projection, but `30F`
  explicitly records this as arrangement-home residue rather than silently
  claiming a pure Spine consumer.
