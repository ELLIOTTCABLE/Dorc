//! The Spine's one instantiation, and the projections that read it (`plans/309` §0/§3).
//!
//! `dorc_core::spine` owns the structure, the keying, the census, and the operand caps; this module
//! names the payload types `core` may not (the license-bearing decision, the admitted record
//! buffer, the narration) and derives the products.
//!
//! # Plan is a projection, not a product
//!
//! [`Plan`] used to be assembled directly by the kernel and then poked at by the driver. It is now
//! DERIVED from the Spine the kernel wrote, lossily and in one place ([`project_plan`]). Two things
//! fall out. A field the projection drops is visibly dropped, at one seat, rather than being a
//! field nobody happened to copy; and a run whose intake integrity was lost cannot produce one at
//! all, because the projection demands a [`PlanAuthority`] and there is no way to make one from a
//! `Refused` admission (`306b:rul-report-only-output-cannot-plan`).

use dorc_aid::narrative::CollapseNarrative;
use dorc_core::spine::{DecidePlane, SurvivalDemote, SurvivalOutcome};

use crate::records::Admission;
use crate::{Disposition, Plan, Step, SurvivalReport};

/// The decide plane this engine instantiates (`309` §2 crate-home).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanPlane;

impl DecidePlane for PlanPlane {
    type Decision = Disposition;
    type Records = crate::records::AdmittedUnscopedHostRecords;
    type Narrative = CollapseNarrative;
}

/// The engine's Spine: one structure, every decision (`309` §0).
pub type Spine = dorc_core::spine::Spine<PlanPlane>;

/// Proof that this run may produce an authority-bearing projection
/// (`306b:rul-report-only-output-cannot-plan`; `309` §3's authority-exit enumeration).
///
/// **A type, not a flag** (`306b` §4b): the value here is that the plan-producing conversion is
/// ABSENT rather than guarded. A boolean eventually goes unchecked; a missing witness cannot.
///
/// The field is private and there is no `Default`, no `Clone` from thin air, and no public
/// constructor beyond the two named mints below — so the only ways to hold one are to have an
/// admission that did not refuse, or to be a run with no intake at all.
///
/// Scope, stated precisely because it is easy to over-read: this gates the PROJECTION. It does not
/// change what a refused run prints today, and it is not the mechanism by which a refused run would
/// go on to render a complete report — that is render work `306b` §6c defers.
#[derive(Debug, Clone, Copy)]
pub struct PlanAuthority(());

/// An [`Admission`] with its [`PlanAuthority`] attached to the arms that carry one
/// (`PlanAuthority::authorise`).
///
/// The shape is the mechanism: the witness is ATTACHED to the two continuing arms rather than
/// returned beside the admission, so a driver reads its authority out of the same `match` that
/// tells it what the intake answered. There is no arm where a caller must remember to check, and no
/// arm where one is missing but the value continues.
#[derive(Debug)]
pub enum Authorised<T> {
    /// Usable facts arrived.
    Admitted(T, PlanAuthority),
    /// A well-owned attempt that produced no usable fact: ordinary conservative planning, and the
    /// affected sites simply run. It carries the authority — a quiet world is not a broken channel.
    NoObservation(PlanAuthority),
    /// Framing, bounds, attribution, or integrity failure. NO authority: this is not uncertainty
    /// about the world but not knowing whether we are still talking to the world we think we are,
    /// and it withholds mutation rather than rounding up to a universal "run"
    /// (`rul-integrity-failure-withholds-mutation`).
    Refused(crate::records::AdmissionRefusal),
}

impl PlanAuthority {
    /// Attach the authority an admission carries, arm by arm.
    ///
    /// Total and closed: every admission converts, and the only arm without a witness is the one
    /// that must not plan.
    #[must_use]
    pub fn authorise<T>(admission: Admission<T>) -> Authorised<T> {
        match admission {
            Admission::Admitted(value) => Authorised::Admitted(value, Self(())),
            Admission::NoObservation => Authorised::NoObservation(Self(())),
            Admission::Refused(reason) => Authorised::Refused(reason),
        }
    }

    /// The authority a REPLAY carries. The durable passed its own admission upstream, whose refusal
    /// returns before any analysis; a replay then re-derives a decision that was already made and
    /// opens no channel to a host, so there is no live intake integrity for this seat to answer for.
    #[must_use]
    pub const fn of_admitted_replay() -> Self {
        Self(())
    }

    /// A run with no intake at all — the kernel entries, `hostsim`, and DST, which analyse the
    /// unmeasured world (every fact ⊤ ⇒ every site runs).
    ///
    /// Not a bypass: a run that never opened an intake has no integrity to have lost, and the
    /// refusal this witness exists to enforce is about a channel that broke, not about one that was
    /// never opened. `the_driver_takes_its_authority_from_its_admission` is the fence — the binary
    /// driver, the one place a live intake is answered, reaches its authority through
    /// [`of_admission`](Self::of_admission) or [`of_admitted_replay`](Self::of_admitted_replay), so
    /// a refused intake can never be re-authorised by reaching for this instead.
    #[must_use]
    pub const fn without_intake() -> Self {
        Self(())
    }
}

/// Project the [`Plan`] from the Spine (`309` §0: "render Plan from Spine as a lossy
/// transformation").
///
/// Every field is derived, and the derivation is total over what the Spine holds:
/// * `steps` — one per [`SpineDisposition`](dorc_core::spine::SpineDisposition), in site order,
///   which IS span order because leaf ids are assigned by span;
/// * `survival_report` — [`project_survival_report`];
/// * `defensive_emission` — the whole-artifact render decision the Spine records.
///
/// The `SiteId` key's member index is DROPPED here, because a `Step` is leaf-granular: today
/// exactly one disposition exists per leaf, so the projection is currently injective, and the Spine
/// keying is what makes a future per-member step a widening of the projection rather than a re-key
/// of the whole engine (`30E:stop-siteid-digest-rekey`).
#[must_use]
pub fn project_plan(spine: &Spine, _authority: &PlanAuthority) -> Plan {
    let steps: Vec<Step> = spine
        .dispositions()
        .map(|record| Step {
            leaf: record.site.leaf,
            ast: record.ast,
            sh: record.sh.clone(),
            disposition: record.decision.clone(),
        })
        .collect();
    Plan {
        steps,
        survival_report: project_survival_report(spine),
        defensive_emission: projected_defensive_emission(spine),
    }
}

/// Derive the survival-tier instrumentation from the Spine's survival records and the narration
/// minted beside them (`24F` §3a).
///
/// Faithful by construction rather than by copying: each field is a filter over one record set, so
/// a survival outcome that stops being recorded stops being reported, instead of the two drifting.
#[must_use]
pub fn project_survival_report(spine: &Spine) -> SurvivalReport {
    let mut report = SurvivalReport {
        collapse_narrative: spine.narratives().to_vec(),
        ..SurvivalReport::default()
    };
    for record in spine.survivals() {
        match record.outcome {
            SurvivalOutcome::Demoted(SurvivalDemote::MayAlias) => {
                report.may_alias_fires = report.may_alias_fires.saturating_add(1);
            }
            SurvivalOutcome::Demoted(SurvivalDemote::Poisoned) => {
                if let Some(kind) = record.poisoned_by {
                    report.reach_poisonings.push((record.leaf, kind));
                }
            }
            SurvivalOutcome::RederivationDisagreed { wall } => {
                report.rederivation_demotions.push((record.leaf, wall));
            }
            SurvivalOutcome::Clean
            | SurvivalOutcome::Survived
            | SurvivalOutcome::Demoted(SurvivalDemote::TotalWall) => {}
        }
    }
    report
}

/// Record the render-time decisions the audit found hiding (`30E` §3), onto the decision plane.
///
/// Three of the five audited decisions are made INSIDE the render, where only a diagnostic stands
/// between them and the structured plane — and each is license-relevant. `dec-pinned-definitions`
/// decides which body a guard invokes and under what name, where a misalignment swaps WHOSE
/// judgment executes (pope-sin tier, `271:rul-sin-ordering`). `dec-render-refusal` is a leaf the
/// disposition layer LICENSED that the span render refuses, so the record and the artifact disagree
/// by design. `dec-omit-neutralisation` is the wrong-yes fence of
/// `erasure-demands-a-proof-and-a-rendered-death`, evaluated at render time.
///
/// They are computed HERE from the render's own seats — `Plan::pinned_definitions` and
/// `Plan::refused_render_steps` — rather than re-derived, so the record cannot drift from what the
/// artifact does. RESIDUE, stated where it bites (`churn-avoidance-disclosure`): the render still
/// computes them for itself rather than reading them back, so this makes the decisions VISIBLE and
/// diffable without yet making the render a pure consumer. Closing that is the arrangement-home
/// round's, and `render_decisions_agree_with_the_render` is what holds the two together meanwhile.
pub fn record_render_decisions(
    spine: &mut Spine,
    plan: &Plan,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
) {
    use dorc_core::spine::{RefusalCause, RenderDecision, SpineRenderDecision};

    let pinned = plan.pinned_definitions(src, ast);
    for step in &plan.steps {
        if let Some(invoked) = pinned.invoked(step.ast) {
            spine.push_render_decision(SpineRenderDecision {
                site: Some(dorc_core::SiteId::leaf(step.leaf)),
                decision: RenderDecision::PinnedBinding {
                    invoked: invoked.to_owned(),
                },
                grade: None,
            });
        }
    }
    for (leaf, _verb) in plan.refused_render_leaves(ast) {
        spine.push_render_decision(SpineRenderDecision {
            site: Some(dorc_core::SiteId::leaf(leaf)),
            decision: RenderDecision::Refused {
                cause: RefusalCause::Heredoc,
            },
            grade: None,
        });
    }
    for (leaf, neutralised) in plan.omit_neutralisations(ast) {
        spine.push_render_decision(SpineRenderDecision {
            site: Some(dorc_core::SiteId::leaf(leaf)),
            decision: RenderDecision::OmitNeutralised { neutralised },
            grade: None,
        });
    }
}

/// Read the whole-artifact emission regime off the Spine (`dec-defensive-emission`, hoisted out of
/// the driver's post-construction field poke — `30E` §3).
fn projected_defensive_emission(spine: &Spine) -> bool {
    spine.render_decisions().iter().any(|record| {
        matches!(
            record.decision,
            dorc_core::spine::RenderDecision::DefensiveEmission { defensive: true }
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::AdmissionRefusal;

    #[test]
    fn a_refused_admission_yields_no_authority_to_project_a_plan() {
        // `306b:rul-report-only-output-cannot-plan`, as a type rather than a guard: the refused arm
        // carries no witness, so a refused attempt cannot be walked forward into a plan by anyone
        // who forgets to check — there is nothing to forget.
        let refused: Admission<()> = Admission::Refused(AdmissionRefusal::Framing);
        assert!(matches!(
            PlanAuthority::authorise(refused),
            Authorised::Refused(_)
        ));
    }

    #[test]
    fn a_well_owned_attempt_with_no_usable_fact_still_plans() {
        // `NoObservation` is ordinary conservative planning — the affected sites simply run. Folding
        // it in with `Refused` would convert a quiet world into a broken channel.
        let none: Admission<()> = Admission::NoObservation;
        let some: Admission<u8> = Admission::Admitted(7);
        assert!(matches!(
            PlanAuthority::authorise(none),
            Authorised::NoObservation(_)
        ));
        assert!(matches!(
            PlanAuthority::authorise(some),
            Authorised::Admitted(7, _)
        ));
    }

    #[test]
    fn the_audited_render_decisions_reach_the_decision_plane_site_keyed() {
        // `30E` §3: before the reification these three were made inside the render with only a
        // diagnostic between them and the structured plane, which is exactly why the smoke-diff
        // exists. This pins that a diff over the decision plane now SEES them, keyed by site — a
        // guard's binding beside the guard, an omit's neutralisation answer beside the omit.
        use dorc_core::spine::RenderDecision;
        use dorc_core::{
            AstId, ByVouch, EntityRef, FactKey, Interner, KindId, LeafId, OpaqueToken,
        };
        use dorc_core::{Rung, SelectorId, SourceFileId, Verdict};

        let mut interner = Interner::default();
        let fact = FactKey::cell(
            KindId(interner.intern("package")),
            EntityRef::Operand(OpaqueToken(interner.intern("curl"))),
            SelectorId(interner.intern("installed")),
        );
        let vouch = ByVouch::vouched(
            crate::VerdictVouch::new(
                "apt_get__is_converged".to_owned(),
                "apt_get__is_converged() { return 0; }".to_owned(),
                "apt_get__is_converged install curl".to_owned(),
                "package".to_owned(),
                Vec::new(),
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            ),
            Rung::Both,
        );
        let src = "apt-get install curl\nsystemctl reload nginx\n";
        let ast = dorc_syntax::parse(src).value;
        let plan = Plan {
            steps: vec![
                Step {
                    leaf: LeafId(0),
                    ast: AstId(0),
                    sh: "apt-get install curl".to_owned(),
                    disposition: Disposition::Guard(
                        crate::GuardLicense::mint(fact, vouch, Verdict::Converged)
                            .expect("a converged probe verdict mints a guard"),
                    ),
                },
                Step {
                    leaf: LeafId(1),
                    ast: AstId(1),
                    sh: "systemctl reload nginx".to_owned(),
                    // A GUARD controller runs its check and MAY run the original, so its decision is
                    // not reproduced by a `:` body — the omit stays verbatim, the run-it direction.
                    disposition: Disposition::Omit {
                        controller: AstId(0),
                    },
                },
            ],
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        };

        let mut spine = Spine::new();
        record_render_decisions(&mut spine, &plan, src, &ast);

        let binding = spine.render_decisions().iter().find(|record| {
            matches!(record.decision, RenderDecision::PinnedBinding { .. })
                && record.site == Some(dorc_core::SiteId::leaf(LeafId(0)))
        });
        assert!(
            binding.is_some(),
            "the guard's binding — whose judgment executes — must be readable beside its site"
        );
        assert_eq!(
            spine
                .render_decisions()
                .iter()
                .filter_map(|record| match record.decision {
                    RenderDecision::OmitNeutralised { neutralised } =>
                        Some((record.site, neutralised)),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            vec![(Some(dorc_core::SiteId::leaf(LeafId(1))), false)],
            "the omit's wrong-yes fence answers `false` behind a guard, and says so on the record"
        );
    }

    #[test]
    fn the_driver_takes_its_authority_from_its_admission() {
        // The lexical half of the fence: no type can stop the binary driver reaching for the
        // intakeless mint after its intake refused, so this asserts it does not. A ZERO-caller
        // assertion over the one file that answers a live intake, rather than an allow-list — so it
        // needs no maintenance and cannot be widened by adding a row.
        let driver = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates/")
            .join("cli")
            .join("src")
            .join("main.rs");
        let text = std::fs::read_to_string(&driver).expect("the driver source is readable");
        assert!(
            text.contains("build_plan_walled"),
            "the walk found the wrong file: this must be the driver that plans"
        );
        assert!(
            !text.contains("without_intake"),
            "the driver must take its authority from its admission, never from the intakeless mint"
        );
    }
}
