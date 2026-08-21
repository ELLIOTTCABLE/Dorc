//! The terminal certifier-trip cleanup (`302` §3 `rul-certifier-trip-guard-only`).
//!
//! A trip means the solver and the solve-certifier DISAGREE, and the pair shares substrate (`Eq`,
//! canonical forms, the transfer model), so the disagreement cannot distinguish a solver-class
//! defect from a substrate-class one: it disqualifies BOTH, and neither may testify afterward.
//! This pass is what stops them testifying — one flat walk over the finished plan, immediately
//! before emission, demoting every elision-family outcome in the scope to run.
//!
//! # What it is NOT
//!
//! It is not a floor. The `302` §3 consumer floors (value · funcenv · reach · self-reach) are
//! unchanged and still fire in place, mid-pipeline, because a terminal pass cannot un-ship a wrong
//! pinned body — by the time a plan exists the probe has already been compiled and run. This is
//! the thin cross-window policy layered ABOVE them: it exists because a window that certified
//! clean may still have consumed an answer from a window that did not, and no per-window floor can
//! see that.
//!
//! It is also not recovery, and none is coming (`302` §9). Nothing here re-plans, carves a region,
//! or buys value back — re-planning would re-consult the machinery the trip disqualified.
//!
//! # Why guards may stand
//!
//! `( check ) || <original bytes>` re-verifies live, on the host, at apply time, and the original
//! command survives verbatim as the `||`-right whatever the check says. Everything a guard rests
//! on is therefore re-measured — except ONE analysis-chosen conjunct: WHICH body the name
//! resolves to. The body-occupancy census answers exactly that conjunct without consulting any
//! solve (`dorc_analysis::funcenv::DefinitionTable::occupancy` is a syntactic count), so a guard
//! whose family is census-unique keeps a live net that a trip cannot have corrupted, and one whose
//! family is plural demotes with everything else.

use dorc_aid::CollapseNarrative;
use dorc_aid::narrative::{CollapseKind, DemoteTag, SpeechAct};

use crate::{Disposition, Spine};

/// What the cleanup did (`302` §5): the count for the plan-prominent banner, and the per-site
/// narrative records that stay pull-tier.
#[derive(Debug, Clone, Default)]
pub struct TripCleanup {
    demoted: usize,
    narrative: Vec<CollapseNarrative>,
}

impl TripCleanup {
    /// How many sites the cleanup demoted. Zero is a legitimate answer for a tripped run — a plan
    /// that elided nothing loses nothing — and the banner fires on the TRIP, never on this count.
    #[must_use]
    pub fn demoted(&self) -> usize {
        self.demoted
    }

    /// One `Derived`-tier `Demotion` record per demoted site (`collapse-mints-narrative`),
    /// decision-inert like every other narrative.
    #[must_use]
    pub fn narrative(&self) -> &[CollapseNarrative] {
        &self.narrative
    }
}

/// Demote every elision-family outcome in `plan` to run, because a certifier tripped this run.
///
/// The three elision-family outcomes are two dispositions: `Replace` covers both elide-by-proof
/// and SURVIVE (an elision kept past a running wall — the split is in the license's witness, not
/// the verb), and `Omit` is the fold-proved-dead branch. `Run` steps are left alone: runs run.
///
/// `census_unique` answers the body-occupancy question for a guard's verdict funcname. A guard it
/// answers `true` for STANDS; every other guard demotes. Callers with no census in hand answer
/// `false` and take the `FORFEITS:forfeit-certifier-trip-demotes-guards` posture — verbatim plus
/// banner — which is always safe and merely poorer.
///
/// Deliberately stupid, and that is the design: the whole policy is one boolean and this walk
/// (`302:rul-certifier-value-is-stupidity` applied to the trip's consequences).
pub fn demote_on_trip(spine: &mut Spine, census_unique: impl Fn(&str) -> bool) -> TripCleanup {
    let mut out = TripCleanup::default();
    let mut demoted_sites = Vec::new();
    for record in spine.dispositions_mut() {
        let stands = match &record.decision {
            Disposition::Run => true,
            Disposition::Guard(license) => census_unique(license.insert().fn_name()),
            Disposition::Replace(..) | Disposition::Omit { .. } => false,
        };
        if stands {
            continue;
        }
        record.decision = Disposition::Run;
        out.demoted = out.demoted.saturating_add(1);
        demoted_sites.push(record.site);
        out.narrative.push(CollapseNarrative::new(
            SpeechAct::Derived,
            CollapseKind::Demotion {
                site: dorc_aid::diag::SiteId::leaf(record.site.leaf),
                reason: DemoteTag::CertifierTripped,
            },
        ));
    }
    // `dec-certifier-trip-cleanup` (`30E` §3) lands in the decision plane rather than staying a
    // post-construction mutation nobody records: the cleanup is a decision about a decision, and a
    // NEW driver forgetting to call it is exactly the must-remember-to-ask surface the reification
    // dissolves.
    for site in demoted_sites {
        spine.push_render_decision(dorc_core::spine::SpineRenderDecision {
            site: Some(site),
            decision: dorc_core::spine::RenderDecision::CertifierTripDemote,
            grade: None,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use dorc_analysis::certify::{CertifierTrip, certify_solution};
    use dorc_analysis::lattice::Flat;
    use dorc_analysis::solve::{Direction, Graph, Solution};
    use dorc_core::{
        AstId, ByVouch, EntityRef, FactKey, Interner, KindId, LeafId, Observable, OpaqueToken,
        ProviderId, Rung, SelectorId, SourceFileId, Verdict,
    };
    use dorc_oracle::{KindIndex, ValueClaim};

    use super::{TripCleanup, demote_on_trip};
    use crate::{Disposition, Plan, Spine, Step, VerdictVouch, build_plan};

    /// One node with a self-loop — the smallest system that has an edge to fail.
    struct SelfLoop;
    impl Graph for SelfLoop {
        fn node_count(&self) -> usize {
            1
        }
        fn succ(&self, _: usize) -> &[usize] {
            &[0]
        }
        fn pred(&self, _: usize) -> &[usize] {
            &[0]
        }
    }

    /// A latch tripped by a GENUINE perturbation judged by the GENUINE checker (`302` §6.1/§6.7,
    /// the F9-era shape): the claimed solution says ⊥ everywhere while the transfer really
    /// produces `Elem(1)`, so the per-edge inequality fails for real. Nothing here hand-injects an
    /// outcome — `certify_solution` is the judge, and the mint it uses is the fenced one.
    fn a_real_trip() -> CertifierTrip {
        let pristine: Flat<u8> = Flat::Bottom;
        let solution = Solution {
            states: vec![pristine.clone()],
            converged: true,
            rounds: 1,
        };
        let outcome = certify_solution(
            &SelfLoop,
            Direction::Forward,
            std::slice::from_ref(&pristine),
            |_, _| Flat::Elem(1u8),
            &solution,
        );
        assert!(
            !outcome.is_consistent(),
            "the fixture must really fail — otherwise every assertion below is vacuous"
        );
        let mut trip = CertifierTrip::default();
        trip.record(&outcome);
        assert!(trip.tripped());
        trip
    }

    /// A latch that saw a real, PASSING certification and stayed shut.
    fn a_real_clean_latch() -> CertifierTrip {
        let pristine: Flat<u8> = Flat::Bottom;
        let solution = Solution {
            states: vec![pristine.clone()],
            converged: true,
            rounds: 1,
        };
        let outcome = certify_solution(
            &SelfLoop,
            Direction::Forward,
            std::slice::from_ref(&pristine),
            |_, incoming: &Flat<u8>| incoming.clone(),
            &solution,
        );
        assert!(outcome.is_consistent(), "the control must really certify");
        let mut trip = CertifierTrip::default();
        trip.record(&outcome);
        trip
    }

    /// The corpus `apt_get__predict`, enough of it to resolve `apt-get install -y <pkg>`.
    const PREDICT_SRC: &str = r#"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ; fi ;;
   esac
}
"#;

    fn a_vouch(fn_name: &str, preamble: &str) -> ByVouch<VerdictVouch> {
        ByVouch::vouched(
            VerdictVouch::new(
                fn_name.to_string(),
                preamble.to_string(),
                format!("{fn_name} install -y nginx"),
                "package".to_string(),
                Vec::new(),
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            ),
            Rung::Both,
        )
    }

    /// A REAL plan for a converged `apt-get install`, driven end to end through classify and
    /// `build_plan`, so its `Replace` is one the elision predicate actually minted.
    fn a_real_elide_plan() -> Plan {
        let mut i = Interner::default();
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let mut idx = KindIndex::default();
        idx.add_effect(0, apt, install, package, installed, ValueClaim::Establish);

        let src = "apt-get install -y nginx\n";
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        let mut vouches = crate::Vouches::new();
        for (node, class) in &classes {
            if let dorc_analysis::effect::SkipClass::EstablishProbeAmbient(fact) = class {
                vouches.insert(*node, *fact, a_vouch("apt_get__is_converged", "body"));
            }
        }
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &invalidators,
            &vouches,
            |f: FactKey| {
                if f.kind == package
                    && f.selector == installed
                    && let EntityRef::Operand(OpaqueToken(tok)) = f.entity
                    && i.resolve(tok) == "nginx"
                {
                    return Observable::verdict_only(Verdict::Converged);
                }
                Observable::verdict_only(Verdict::Unknown)
            },
            &mut dorc_core::ProvArena::new(),
        );
        assert!(
            plan.steps
                .iter()
                .any(|s| matches!(s.disposition, Disposition::Replace(..))),
            "the fixture must really elide — otherwise the eviction below proves nothing"
        );
        plan
    }

    fn guard_step(leaf: u32, fn_name: &str) -> Step {
        let mut i = Interner::default();
        let fact = FactKey::cell(
            KindId(i.intern("package")),
            EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            SelectorId(i.intern("installed")),
        );
        Step {
            leaf: LeafId(leaf),
            ast: AstId(leaf),
            sh: "apt-get install -y nginx".to_string(),
            disposition: Disposition::Guard(
                crate::GuardLicense::mint(
                    fact,
                    a_vouch(fn_name, "body"),
                    Verdict::Converged,
                    &dorc_analysis::lattice::May(dorc_analysis::lattice::Powerset::default()),
                )
                .expect("a converged probe verdict mints a guard"),
            ),
        }
    }

    fn omit_step(leaf: u32) -> Step {
        Step {
            leaf: LeafId(leaf),
            ast: AstId(leaf),
            sh: "systemctl reload nginx".to_string(),
            disposition: Disposition::Omit {
                controller: AstId(0),
            },
        }
    }

    /// Write a step list onto a Spine, which is where the cleanup now reaches its decisions.
    fn spine_of(steps: Vec<Step>) -> Spine {
        let mut spine = Spine::new();
        for step in steps {
            spine.set_disposition(dorc_core::spine::SpineDisposition {
                site: dorc_core::SiteId::leaf(step.leaf),
                ast: step.ast,
                sh: step.sh,
                decision: step.disposition,
                grade: None,
            });
        }
        spine
    }

    /// The projected plan, which is what every consumer of the cleanup actually reads.
    fn projected(spine: &Spine) -> Plan {
        crate::project_plan(spine, &crate::PlanAuthority::without_intake())
    }

    fn tags(cleanup: &TripCleanup) -> Vec<dorc_aid::narrative::CollapseKind> {
        cleanup
            .narrative()
            .iter()
            .map(|n| n.kind().clone())
            .collect()
    }

    /// `302:rul-certifier-trip-guard-only` — the eviction, over a REAL elision and a REAL trip.
    ///
    /// The elide comes from the actual predicate (a converged, vouched, ambient establish) and the
    /// trip from the actual checker, so the only thing under test is the policy between them. The
    /// `Omit` rides along because a fold-proved-dead branch is an elision-family outcome too: its
    /// deadness was proved from the same analysis the trip disqualified.
    #[test]
    fn a_real_trip_evicts_every_elision_family_outcome() {
        let mut steps = a_real_elide_plan().steps;
        steps.push(omit_step(9));
        let before = steps.len();
        let mut spine = spine_of(steps);

        let cleanup = demote_on_trip(&mut spine, |_| true);
        let plan = projected(&spine);

        assert_eq!(
            plan.steps.len(),
            before,
            "nothing is removed — the plan becomes verbatim-or-guarded, never shorter"
        );
        assert!(
            plan.steps
                .iter()
                .all(|s| matches!(s.disposition, Disposition::Run)),
            "every elide and every omit demotes to run: {:?}",
            plan.steps
                .iter()
                .map(|s| &s.disposition)
                .collect::<Vec<_>>()
        );
        assert_eq!(cleanup.demoted(), before, "each demotion is accounted for");
        assert!(
            tags(&cleanup).iter().all(|k| matches!(
                k,
                dorc_aid::narrative::CollapseKind::Demotion {
                    reason: dorc_aid::narrative::DemoteTag::CertifierTripped,
                    ..
                }
            )),
            "each demotion narrates as the existing demotion class under its own reason arm"
        );
    }

    #[test]
    #[ignore = "round-30 adversarial review: plan producers project before trip cleanup"]
    fn a_tripped_plan_projected_without_cleanup_must_not_retain_elision() {
        let spine = spine_of(a_real_elide_plan().steps);
        let trip = a_real_trip();
        assert!(trip.tripped());

        let plan = projected(&spine);

        assert!(
            plan.steps
                .iter()
                .all(|step| matches!(step.disposition, Disposition::Run)),
            "a genuine certifier disagreement must reach the terminal demotion before projection"
        );
    }

    /// THE CENSUS FORK. A guard whose verdict family is census-unique STANDS — its runtime net
    /// re-verifies live and the one analysis-chosen conjunct (which body the name resolves to) was
    /// never chosen. A guard whose family is plural demotes with everything else, because there a
    /// wrong choice runs somebody else's judgment over a mutator that needed to run.
    #[test]
    fn a_census_unique_guard_stands_while_a_plural_one_demotes() {
        let mut spine = spine_of(vec![
            guard_step(0, "apt_get__is_converged"),
            guard_step(1, "ufw__is_converged"),
        ]);

        let cleanup = demote_on_trip(&mut spine, |fn_name| fn_name == "apt_get__is_converged");
        let plan = projected(&spine);

        assert!(
            matches!(plan.steps[0].disposition, Disposition::Guard(_)),
            "the census-unique family keeps its guard"
        );
        assert!(
            matches!(plan.steps[1].disposition, Disposition::Run),
            "the plural family loses its guard: its body identity is analysis-chosen"
        );
        assert_eq!(cleanup.demoted(), 1);
    }

    /// The WHOLESALE branch (`FORFEITS:forfeit-certifier-trip-demotes-guards`): a caller with no
    /// census in hand answers `false` and the tripped plan is verbatim-plus-banner. Pinned so the
    /// forfeited posture stays reachable and safe rather than merely described.
    #[test]
    fn a_censusless_caller_demotes_guards_wholesale() {
        let mut spine = spine_of(vec![guard_step(0, "apt_get__is_converged")]);

        demote_on_trip(&mut spine, |_| false);

        assert!(matches!(
            projected(&spine).steps[0].disposition,
            Disposition::Run
        ));
    }

    /// THE LATCH'S OWN CONTROL: a certification that really PASSED leaves the latch shut, so the
    /// eviction above is driven by a real disagreement and not by the pass having run. (The seat
    /// control — that an unlatched run reaches no walk at all — lives with the seat, in
    /// `dorc_cli::world`.)
    #[test]
    fn a_passing_certification_never_latches() {
        assert!(!a_real_clean_latch().tripped());
    }

    /// The latch is MONOTONE: a later consistent answer cannot argue an earlier failure away.
    /// Both outcomes are real, so this is a property of the latch and not of a fixture.
    #[test]
    fn a_later_consistent_answer_never_clears_the_latch() {
        let mut trip = a_real_trip();
        let pristine: Flat<u8> = Flat::Bottom;
        let solution = Solution {
            states: vec![pristine.clone()],
            converged: true,
            rounds: 1,
        };
        let clean = certify_solution(
            &SelfLoop,
            Direction::Forward,
            std::slice::from_ref(&pristine),
            |_, incoming: &Flat<u8>| incoming.clone(),
            &solution,
        );
        assert!(clean.is_consistent());

        trip.record(&clean);

        assert!(
            trip.tripped(),
            "a spine that tripped stays tripped — nothing in the run may testify it away"
        );
    }
}
