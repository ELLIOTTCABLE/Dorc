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
use dorc_core::spine::{DecidePlane, InfluenceBearing, SurvivalDemote, SurvivalOutcome};

use crate::records::Admission;
use crate::{Disposition, Plan, Step, SurvivalReport};

/// The decide plane this engine instantiates (`309` §2 crate-home).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanPlane;

impl DecidePlane for PlanPlane {
    type Decision = Disposition;
    type Records = crate::records::AdmittedUnscopedHostRecords;
    type Narrative = CollapseNarrative;
    /// One authored region's shared decision is the same VERB set a site's is — replace, guard, or
    /// run the authored bytes — because `30L:rul-no-specialized-shell` leaves no third thing an edit
    /// could be. The identities differ (`ElisionRegion` vs `SiteId`), which is why the seam keeps
    /// them separate types even where they instantiate to one enum.
    type RegionDecision = Disposition;
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
///
/// Two witnesses by reference, and neither is decoration: `authority` is the intake's
/// (`306b:rul-report-only-output-cannot-plan`) and `_spent` is the certifier latch's
/// ([`TripSpent`](crate::certifier_trip::TripSpent)) — a plan that still elided past a tripped
/// certifier would be exactly the retained elision `30Md:fnd-discarded-trip-retains-elisions`
/// demonstrated, so the walk is a precondition of projecting rather than a call to remember.
///
/// The RENDER-time decisions are taken here too, and recorded before the plan is handed back
/// ([`record_render_decisions`]) — so a projection cannot exist whose render decisions nothing
/// wrote down, which is what `30F` §4.4 disclosed as still open.
#[must_use]
pub fn project_plan(
    spine: &mut Spine,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
    emission: crate::ArtifactEmission<'_>,
    _authority: &PlanAuthority,
    _spent: &crate::certifier_trip::TripSpent,
    world: dorc_core::influence::InfluenceAccount,
) -> Plan {
    let steps: Vec<Step> = spine
        .dispositions()
        .map(|record| Step {
            leaf: record.site().leaf,
            ast: record.ast(),
            sh: record.sh().to_owned(),
            disposition: record.decision().clone(),
        })
        .collect();
    let regions = spine
        .region_decisions()
        .iter()
        .map(|record| crate::RegionStep {
            region: record.region(),
            ast: record.ast(),
            sh: record.sh().to_owned(),
            disposition: record.decision().clone(),
            routes: record.routes().clone(),
        })
        .collect();
    // `306b:rul-projections-continue-influence-flow` — Spine finalization does not terminate
    // propagation. The fold starts at the PROJECTING RUN's own account rather than at ⊥, so a plan
    // over zero records answers where the run stands instead of reading pre-contact.
    let decided = spine
        .dispositions()
        .map(InfluenceBearing::account)
        .chain(
            spine
                .region_decisions()
                .iter()
                .map(InfluenceBearing::account),
        )
        .chain(
            spine
                .render_decisions()
                .iter()
                .map(InfluenceBearing::account),
        )
        .fold(world, dorc_core::influence::InfluenceAccount::join);
    let plan = Plan::decided(
        steps,
        regions,
        project_survival_report(spine),
        projected_defensive_emission(spine),
        emission,
        src,
        ast,
        decided,
    );
    record_render_decisions(spine, &plan, plan.account());
    plan
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
        match record.outcome() {
            SurvivalOutcome::Demoted(SurvivalDemote::MayAlias) => {
                report.may_alias_fires = report.may_alias_fires.saturating_add(1);
            }
            SurvivalOutcome::Demoted(SurvivalDemote::Poisoned) => {
                if let Some(kind) = record.poisoned_by() {
                    report.reach_poisonings.push((record.leaf(), kind));
                }
            }
            SurvivalOutcome::RederivationDisagreed { wall } => {
                report.rederivation_demotions.push((record.leaf(), wall));
            }
            // The report counts findings about the RESOLVERS and the reference model; a demotion
            // taken because a solve failed its own check is already the certifier's to report, and
            // counting it here would double-count one event under two instruments.
            SurvivalOutcome::Clean
            | SurvivalOutcome::SurvivedStandalone
            | SurvivalOutcome::SurvivedAggregate { .. }
            | SurvivalOutcome::Demoted(
                SurvivalDemote::TotalWall | SurvivalDemote::SolveInconsistent,
            ) => {}
        }
    }
    report
}

/// TRANSCRIBE the decided render onto the decision plane (`30E` §3's audit).
///
/// Three of the five audited decisions used to be made INSIDE the render, where only a diagnostic
/// stood between them and the structured plane — and each is license-relevant.
/// `dec-pinned-definitions` decides which body a guard invokes and under what name, where a
/// misalignment swaps WHOSE judgment executes (pope-sin tier, `271:rul-sin-ordering`).
/// `dec-render-refusal` is a leaf the disposition layer LICENSED that the span render refuses, so
/// the record and the artifact disagree by design. `dec-omit-neutralisation` is the wrong-yes fence
/// of `erasure-demands-a-proof-and-a-rendered-death`.
///
/// This walk now RE-DERIVES NOTHING: `Plan::decided` took every one of them, the render prints
/// them, and this copies the same values onto the Spine. That is the difference from the shape
/// `30F` §4.4 disclosed — a record that cannot disagree with the artifact because it is not a
/// second computation of the same question.
///
/// A refused REGION reaches a row on the species' REGION axis
/// (`30N:rul-region-refusal-discloses-region-keyed`): the same record, keyed by the identity a
/// region has, never by one of the invocations that share the edit.
pub fn record_render_decisions(
    spine: &mut Spine,
    plan: &Plan,
    world: dorc_core::influence::InfluenceAccount,
) {
    use dorc_core::spine::{RefusalCause, RenderDecision, SpineRenderDecision};

    let decided = world;
    let pinned = plan.pinned_definitions();
    for step in plan.steps() {
        if let Some(invoked) = pinned.invoked(step.ast) {
            spine.push_render_decision(SpineRenderDecision::minted(
                Some(dorc_core::SiteId::leaf(step.leaf)),
                None,
                RenderDecision::PinnedBinding {
                    invoked: invoked.to_owned(),
                },
                decided,
            ));
        }
    }
    for refusal in plan.render_plane().refused() {
        if refusal.leaf.is_none() && refusal.region.is_none() {
            continue;
        }
        spine.push_render_decision(SpineRenderDecision::minted(
            refusal.leaf.map(dorc_core::SiteId::leaf),
            refusal.region,
            // The REAL cause. Hard-coding `Heredoc` made the record state a falsehood for every
            // redirect-refused guard — the class `30Mf` F2 had just made reachable.
            RenderDecision::Refused {
                cause: match refusal.cause {
                    dorc_aid::narrative::RenderRefusalTag::Heredoc => RefusalCause::Heredoc,
                    dorc_aid::narrative::RenderRefusalTag::OutputRedirect => {
                        RefusalCause::BlockingRedirect
                    }
                },
            },
            decided,
        ));
    }
    // SITE-LESS and REGION-LESS, on `DefensiveEmission`'s precedent: an import edit belongs to a
    // book line, and a new identity gets a new axis (`a-second-key-axis-never-widens-siteid`).
    for import in plan.import_edits() {
        spine.push_render_decision(SpineRenderDecision::minted(
            None,
            None,
            RenderDecision::ImportRewritten {
                verb: import.verb(),
                names: import.names().to_owned(),
            },
            decided,
        ));
    }
    for (leaf, neutralised) in plan.omit_neutralisations() {
        spine.push_render_decision(SpineRenderDecision::minted(
            Some(dorc_core::SiteId::leaf(leaf)),
            None,
            RenderDecision::OmitNeutralised { neutralised },
            decided,
        ));
    }
}

/// Read the whole-artifact emission regime off the Spine (`dec-defensive-emission`, hoisted out of
/// the driver's post-construction field poke — `30E` §3).
fn projected_defensive_emission(spine: &Spine) -> bool {
    spine.render_decisions().iter().any(|record| {
        matches!(
            record.decision(),
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
        let plan = Plan::decided(
            vec![
                Step {
                    leaf: LeafId(0),
                    ast: AstId(0),
                    sh: "apt-get install curl".to_owned(),
                    disposition: Disposition::Guard(
                        crate::GuardLicense::mint(
                            fact,
                            vouch,
                            Verdict::Converged,
                            &dorc_analysis::lattice::May(
                                dorc_analysis::lattice::Powerset::default(),
                            ),
                            dorc_core::influence::InfluenceAccount::authored_before_contact(),
                        )
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
            Vec::new(),
            SurvivalReport::default(),
            false,
            crate::NO_ARTIFACT_FORM,
            src,
            &ast,
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );

        let mut spine = Spine::new();
        record_render_decisions(
            &mut spine,
            &plan,
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );

        let binding = spine.render_decisions().iter().find(|record| {
            matches!(record.decision(), RenderDecision::PinnedBinding { .. })
                && record.site() == Some(dorc_core::SiteId::leaf(LeafId(0)))
        });
        assert!(
            binding.is_some(),
            "the guard's binding — whose judgment executes — must be readable beside its site"
        );
        assert_eq!(
            spine
                .render_decisions()
                .iter()
                .filter_map(|record| match record.decision() {
                    RenderDecision::OmitNeutralised { neutralised } =>
                        Some((record.site(), *neutralised)),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            vec![(Some(dorc_core::SiteId::leaf(LeafId(1))), false)],
            "the omit's wrong-yes fence answers `false` behind a guard, and says so on the record"
        );
    }

    #[test]
    fn the_new_arm_debug_dump_has_no_production_caller() {
        // `309:pin-debug-dump-gating`'s second half. The signature already cannot name a sink; this
        // is the part no type can hold — that no shipping path CALLS it. A non-empty walk, so a
        // wrong root cannot pass by finding nothing (the discovery-floor lesson).
        let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates/");
        // Assembled rather than written, so this gate cannot match its own source — the lesson
        // `aid`'s spanless gate records ("keep examples needle-free").
        let needle = format!(".{}()", "debug_dump");
        let mut walked = 0usize;
        let mut offenders = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = ["core", "plan", "cli", "oracle", "analysis"]
            .iter()
            .map(|name| crates.join(name).join("src"))
            .collect();
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_some_and(|ext| ext == "rs") {
                    walked += 1;
                    let text = std::fs::read_to_string(&path).unwrap_or_default();
                    // The definition itself lives in `core::spine`; a CALL is what would ship it.
                    if text.contains(&needle) {
                        offenders.push(path.display().to_string());
                    }
                }
            }
        }
        assert!(
            walked > 0,
            "the walk found no production sources, so it proves nothing"
        );
        assert!(
            offenders.is_empty(),
            "the `new`-arm dump must stay project-internal: {offenders:?}"
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

    /// `309:rul-spine-preserves-never-stamps`: a Spine STORES the account a record's own semantic
    /// mint joined — it never computes one, never applies an object-global grade, and never fills
    /// an absent field.
    ///
    /// The shape exercised: two records on ONE Spine belonging to a run that read host bytes, whose
    /// mints answered differently. Before the stamp went, every setter assigned the run-wide scalar
    /// over whatever the mint supplied, so a per-object account was not merely absent but
    /// unobservable — the two inputs were indistinguishable at the reader. What it buys is that a
    /// pre-contact decision stops wearing a post-contact run's phase
    /// (`30I:rul-load-decisions-are-authored-before-contact`).
    ///
    /// Promoted from `p-x-spine-record-keeps-its-mints-account`; `core::spine`'s
    /// `the_spine_stamps_the_grade_so_a_mint_site_cannot_forget_it` pinned the forbidden behaviour
    /// and was rewritten into its opposite in the same commit rather than left passing beside this.
    #[test]
    fn a_spine_record_keeps_the_account_its_mint_supplied() {
        use dorc_core::influence::{InfluenceAccount, Influenced};
        use dorc_core::spine::InfluenceBearing;
        use dorc_core::{AstId, LeafId, SiteId};

        let phase = Influenced::authored_before_contact(()).widen();
        let mut spine = Spine::new();
        for (leaf, account) in [
            (LeafId(0), InfluenceAccount::authored_before_contact()),
            (LeafId(1), InfluenceAccount::of_phase(phase)),
        ] {
            spine.set_disposition(dorc_core::spine::SpineDisposition::minted(
                SiteId::leaf(leaf),
                AstId(leaf.0),
                String::new(),
                Disposition::Run,
                account,
            ));
        }
        let stored: Vec<InfluenceAccount> = spine
            .dispositions()
            .map(InfluenceBearing::account)
            .collect();
        assert_eq!(
            stored,
            [
                InfluenceAccount::authored_before_contact(),
                InfluenceAccount::of_phase(phase)
            ],
            "each record must answer what its own mint joined, not one run-wide phase"
        );
    }

    /// Every workspace source naming `needle`, as `crate/dir/file.rs x<count>`, sorted, EXCLUDING
    /// the module that defines all three needles — plus how many files were walked, so a fence
    /// aimed at a wrong root cannot pass by finding nothing (the discovery-floor lesson).
    fn sources_naming(needle: &str) -> (Vec<String>, usize) {
        let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates/");
        let mut hits: Vec<String> = Vec::new();
        let mut walked = 0usize;
        let mut stack = vec![crates.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if path.file_name().is_some_and(|name| name == "target") {
                        continue;
                    }
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|ext| ext != "rs") {
                    continue;
                }
                walked += 1;
                let text = std::fs::read_to_string(&path).unwrap_or_default();
                let count = text.matches(needle).count();
                if count > 0 {
                    let full = path.display().to_string().replace('\\', "/");
                    let short = full
                        .rsplit_once("/crates/")
                        .map_or(full.clone(), |(_, tail)| tail.to_owned());
                    if short != "core/src/influence.rs" {
                        hits.push(format!("{short} x{count}"));
                    }
                }
            }
        }
        hits.sort();
        (hits, walked)
    }

    /// `tc-accounting-reads-are-not-gating`'s NARROW WINDOW, mechanized.
    ///
    /// Accounting is the one exempt consumer of `306b` §6b (influenced values never gate engine
    /// control flow), and the exemption is affordable only because it is one transition wide: the
    /// phase marker becomes an account at exactly the two driver seats `cli/src/results.rs` owns
    /// (`fnd-two-drivers-compute-one-fact-twice` ruled them two seats, one vocabulary), and every
    /// other seat in the engine only joins accounts it was handed. A new PRODUCTION caller is not a
    /// refactor; it is the window widening.
    ///
    /// The other entries are inline test modules and fixture seats, which a lexical walk cannot
    /// tell from production code. They are listed rather than filtered because a filter that
    /// guessed would be this fence's own blind spot; the file names say which is which.
    #[test]
    fn the_phase_to_account_transition_lives_at_one_seat() {
        let (callers, walked) = sources_naming(concat!("InfluenceAccount", "::of_phase("));
        assert!(
            walked > 0,
            "the walk found no sources, so it proves nothing"
        );
        let files: Vec<&str> = callers
            .iter()
            .filter_map(|hit| hit.split_once(" x").map(|(file, _)| file))
            .collect();
        assert_eq!(
            files,
            [
                "cli/src/results.rs",
                "core/src/spine.rs",
                "plan/src/spine.rs",
                "plan/tests/region.rs",
            ],
            "the phase→account transition must stay at the ruled driver seats; found {callers:?}"
        );
    }

    /// The authored posture is an ASSERTION, so every seat that spells one is enumerated.
    ///
    /// No affine clean-of-host witness is built this round (human-typed), which means nothing
    /// structurally stops a seat claiming authored while reading influenced material. This census
    /// is the whole of what does: the list is two-way, so both a new claimant and a stale entry are
    /// a diff somebody looks at.
    ///
    /// The needle deliberately covers BOTH `Influenced::authored_before_contact` (the value
    /// wrapper) and `InfluenceAccount::authored_before_contact` (the account) — they are the same
    /// hazard wearing two types, and a needle that split them would repeat
    /// `fnd-one-mint-fence-misses-a-qualified-spelling`. Counts are shown but not asserted: they
    /// move with ordinary test churn inside an already-listed file, and a fence people re-bless
    /// every commit stops being read.
    #[test]
    fn every_authored_before_contact_posture_is_enumerated() {
        let (claimants, walked) = sources_naming(concat!("authored_before", "_contact("));
        assert!(
            walked > 0,
            "the walk found no sources, so it proves nothing"
        );
        let files: Vec<&str> = claimants
            .iter()
            .filter_map(|hit| hit.split_once(" x").map(|(file, _)| file))
            .collect();
        assert_eq!(
            files,
            [
                "cli/src/artifact.rs",
                "cli/src/main.rs",
                "cli/src/results.rs",
                "cli/src/world.rs",
                "core/src/spine.rs",
                "coverage/src/lib.rs",
                "hostsim/src/lib.rs",
                "plan/src/certifier_trip.rs",
                "plan/src/erasability.rs",
                "plan/src/lib.rs",
                "plan/src/placement.rs",
                "plan/src/region.rs",
                "plan/src/settle.rs",
                "plan/src/spine.rs",
                "plan/src/whylog.rs",
                "plan/tests/erasability.rs",
                "plan/tests/region.rs",
                "plan/tests/render_corpus.rs",
                "sweep/src/drive.rs",
            ],
            "a new authored claim is a design act; found {claimants:?}"
        );
    }

    /// The staging INVENTORY (`306b:rul-untracked-is-not-authored`) — not a ban.
    ///
    /// Gradation is deliberately not built, and the stated purpose of the threading is to force the
    /// type discipline and then WATCH whether unconverted seams accumulate over later churn. So an
    /// `untracked` adapter is LEGAL and expected; what it is not is silent. A seat that mints one
    /// joins the list below in the same commit, which is what turns "we are staged here" into
    /// something a reader can count rather than something they must go looking for.
    ///
    /// The list is empty only because nothing has needed an adapter yet.
    #[test]
    fn every_untracked_adapter_is_enumerated() {
        /// Every seat that deliberately carries an explicit `untracked`, and what it stages.
        ///
        /// * `plan/src/region.rs` — a region whose route population the census could not close, or
        ///   whose proofs do not correspond to it: the routes nobody enumerated may have been
        ///   decided from host-reported material (`30L:pin-open-route-runs`).
        /// * `plan/tests/region.rs` — that arm's own assertion.
        /// * `plan/src/whylog.rs` — a durable's account read back ABSENT, unrecognised, or
        ///   malformed (`306b:rul-missing-influence-grade-reads-highest`). Not a staged hole in the
        ///   same sense: it is the ruled REHYDRATION floor, and it stays after the export is
        ///   enabled.
        const INVENTORY: &[&str] = &[
            "plan/src/region.rs",
            "plan/src/whylog.rs",
            "plan/tests/region.rs",
        ];

        let (adapters, walked) = sources_naming(concat!("InfluenceAccount", "::untracked("));
        assert!(
            walked > 0,
            "the walk found no sources, so it proves nothing"
        );
        let files: Vec<&str> = adapters
            .iter()
            .filter_map(|hit| hit.split_once(" x").map(|(file, _)| file))
            .collect();
        assert_eq!(
            files, INVENTORY,
            "an untracked seam is legal and STAGED — enumerate it here in the same commit; \
             found {adapters:?}"
        );
    }
}
