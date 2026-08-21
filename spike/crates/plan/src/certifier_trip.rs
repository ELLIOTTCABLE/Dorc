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
use dorc_analysis::certify::CertifierTrip;

use crate::{Disposition, Plan, PlanAuthority, Spine};

/// Proof that this run's [`CertifierTrip`] reached the terminal cleanup before anything projected
/// (`30M:rec-dissolve-trip-must-remember-structurally`).
///
/// **A type, not a roster** — the same shape [`PlanAuthority`] wears, for the same reason. The
/// cleanup used to be a must-remember-to-ask surface: `plan/CLAUDE.md` said "EVERY plan-producing
/// driver", and four producers had already forgotten (`30Md:fnd-discarded-trip-retains-elisions`).
/// The reification moved the cleanup's RESULT into the decision plane but left the ACT a call
/// somebody had to remember; this dissolves the act. [`crate::project_plan`] demands one by value,
/// the only mint is [`spend_certifier_trip`], and that mint cannot be reached without a
/// [`CertifierTrip`] in hand — so a producer that never spent its latch has no projection to call.
///
/// There is deliberately no intakeless-style escape (contrast [`PlanAuthority::without_intake`]):
/// every producer HAS a latch, even an untripped default one, so nothing legitimately needs a
/// witness minted beside the walk rather than by it.
///
/// The two-way lexical roster (`every_plan_producer_spends_its_certifier_trip`) stays as
/// belt-and-braces: it also covers the producers that reach the settlement without projecting here.
///
/// ```compile_fail
/// # use dorc_plan::{PlanAuthority, Spine};
/// let mut spine = Spine::new();
/// // No witness in hand ⇒ no projection: this is the dissolution, spelled. Falsified by hand at
/// // authoring time — threading a real `spend_certifier_trip` witness makes this block COMPILE and
/// // reddens the assertion, so the refusal is the arity and not some unrelated error.
/// let _ = dorc_plan::project_plan(&spine, &PlanAuthority::without_intake());
/// ```
#[derive(Debug)]
pub struct TripSpent(());

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
///
/// AUTHORED REGIONS take the same walk over the same verbs, spelled rather than implied: a shared
/// elision is a whole family of instances at once, so a tripped run that kept one would keep more
/// mutations un-run than any single site could (`30Md:fnd-discarded-trip-retains-elisions`, at the
/// region grain). Its narration keys by the contributing routes' invocation leaves, since a region
/// owns no leaf of its own.
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
    for record in spine.region_decisions_mut() {
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
        for route in record.routes.shown() {
            out.narrative.push(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::Demotion {
                    site: dorc_aid::diag::SiteId::leaf(route.invocation.leaf),
                    reason: DemoteTag::CertifierTripped,
                },
            ));
        }
    }
    // `dec-certifier-trip-cleanup` (`30E` §3) lands in the decision plane rather than staying a
    // post-construction mutation nobody records. The must-remember surface the reification left
    // behind — the RESULT moved, the ACT did not — is now dissolved too: the only way to reach a
    // projection is through `spend_certifier_trip`'s `TripSpent`
    // (`30M:rec-dissolve-trip-must-remember-structurally`).
    for site in demoted_sites {
        spine.push_render_decision(dorc_core::spine::SpineRenderDecision {
            site: Some(site),
            decision: dorc_core::spine::RenderDecision::CertifierTripDemote,
            grade: None,
        });
    }
    out
}

/// SPEND a run's certifier latch — THE one mint of [`TripSpent`].
///
/// Total over both latch states, and that totality is what makes the witness unforgeable: an
/// untripped run does no walk and still comes away with the proof, so nothing is ever tempted to
/// mint one beside the walk. A tripped run demotes first
/// (`302:rul-certifier-trip-guard-only`), and `census_unique` is the guard fork —
/// `FORFEITS:forfeit-certifier-trip-demotes-guards` is what answering `false` costs.
pub fn spend_certifier_trip(
    spine: &mut Spine,
    trip: CertifierTrip,
    census_unique: impl Fn(&str) -> bool,
) -> (TripCleanup, TripSpent) {
    let cleanup = if trip.tripped() {
        demote_on_trip(spine, census_unique)
    } else {
        TripCleanup::default()
    };
    (cleanup, TripSpent(()))
}

/// The whole TAIL of a plan producer holding no body-occupancy census: spend the run's latch, then
/// project (`302:rul-certifier-trip-guard-only`).
///
/// ONE seat, because the two are not independent acts: `build_plan_walled` hands back a Spine AND a
/// latch its settlement may have raised, and a cross-window trip is invisible to the mid-pipeline
/// floors, so this walk is the only thing that evicts it (`30Md:fnd-discarded-trip-retains-elisions`).
/// Censusless is honest for every instrument here — none holds a `DefinitionTable` — so every guard
/// demotes too (`FORFEITS:forfeit-certifier-trip-demotes-guards`: safe, merely poorer).
#[must_use]
pub fn project_censusless(
    spine: &mut Spine,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
    trip: CertifierTrip,
    authority: &PlanAuthority,
) -> Plan {
    let (_cleanup, spent) = spend_certifier_trip(spine, trip, |_| false);
    crate::project_plan(spine, src, ast, authority, &spent)
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

    use super::TripCleanup;
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

    /// The book every fixture spine's dispositions were decided over — the render plane is decided
    /// against the same tree, exactly as a real producer decides it against its own.
    const FIXTURE_BOOK: &str = "apt-get install -y nginx\n";

    /// The projected plan, which is what every consumer of the cleanup actually reads. The latch
    /// is a parameter because it MUST be: there is no projection without one.
    fn projected(spine: &mut Spine, trip: CertifierTrip) -> Plan {
        let (_, spent) = super::spend_certifier_trip(spine, trip, |_| true);
        project(spine, &spent)
    }

    /// Project a spine whose latch the caller already spent itself.
    fn project(spine: &mut Spine, spent: &super::TripSpent) -> Plan {
        let ast = dorc_syntax::parse(FIXTURE_BOOK).value;
        crate::project_plan(
            spine,
            FIXTURE_BOOK,
            &ast,
            &crate::PlanAuthority::without_intake(),
            spent,
        )
    }

    fn tags(cleanup: &TripCleanup) -> Vec<dorc_aid::narrative::CollapseKind> {
        cleanup
            .narrative()
            .iter()
            .map(|n| n.kind().clone())
            .collect()
    }

    /// `30L` §10's last exclusion, at the REGION grain: a run whose certifier tripped keeps no
    /// shared elision either.
    ///
    /// The stakes are strictly higher than a site's. One region decision covers EVERY invocation of
    /// its definition, so a tripped run that kept one would leave more mutations un-run than any
    /// single site could — the `30Md:fnd-discarded-trip-retains-elisions` shape, one abstraction
    /// level up. The demotion narrates against the region's contributing INVOCATIONS, because a
    /// region owns no leaf of its own to be blamed at.
    #[test]
    fn a_real_trip_evicts_a_shared_region_elision_too() {
        let elide = a_real_elide_plan();
        let Some(Disposition::Replace(license, stand_in)) =
            elide.steps.first().map(|step| step.disposition.clone())
        else {
            panic!("the fixture must really carry a licensed replacement");
        };
        let mut spine = Spine::new();
        spine.push_region_decision(dorc_core::spine::SpineRegionDecision {
            region: dorc_core::region::ElisionRegion::mint(
                &dorc_core::region::RegionUniverse::of_book_custody_files([SourceFileId(0)]),
                dorc_core::DefinitionId::at(
                    SourceFileId(0),
                    dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(40)),
                ),
                dorc_core::Span::new(dorc_core::BytePos(4), dorc_core::BytePos(24)),
            )
            .expect("the book surface admits the region"),
            ast: AstId(1),
            sh: "apt-get install -y nginx".to_string(),
            decision: Disposition::Replace(license, stand_in),
            routes: dorc_core::spine::Account::capped([dorc_core::spine::RegionRoute {
                invocation: dorc_core::SiteId::leaf(LeafId(7)),
                ast: AstId(9),
            }]),
            grade: None,
        });

        let (cleanup, spent) = super::spend_certifier_trip(&mut spine, a_real_trip(), |_| true);
        let plan = project(&mut spine, &spent);

        assert!(
            matches!(
                plan.regions.first().map(|r| &r.disposition),
                Some(Disposition::Run)
            ),
            "a shared replacement demotes to run exactly as a site's does"
        );
        assert_eq!(cleanup.demoted(), 1, "and the demotion is accounted for");
        assert!(
            matches!(
                tags(&cleanup).first(),
                Some(dorc_aid::narrative::CollapseKind::Demotion {
                    site,
                    reason: dorc_aid::narrative::DemoteTag::CertifierTripped,
                }) if site.leaf == LeafId(7)
            ),
            "and it narrates against the invocation that would have executed it: {:?}",
            tags(&cleanup)
        );
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

        let (cleanup, spent) = super::spend_certifier_trip(&mut spine, a_real_trip(), |_| true);
        let plan = project(&mut spine, &spent);

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

    /// The producer TAIL over a REAL elision and a REAL trip: a censusless driver spending its
    /// latch through the shared seat cannot ship a tripped plan that still elides. The latch is the
    /// genuine checker's, threaded as a producer threads it, never hand-applied at the projection
    /// (`anti-masking-tests`); the clean-latch half is what makes it non-vacuous.
    #[test]
    fn a_censusless_producer_spends_its_trip_before_projecting() {
        let elided = a_real_elide_plan().steps;
        let mut tripped_spine = spine_of(elided.clone());
        let mut clean_spine = spine_of(elided);
        let authority = crate::PlanAuthority::without_intake();

        let ast = dorc_syntax::parse(FIXTURE_BOOK).value;
        let tripped = super::project_censusless(
            &mut tripped_spine,
            FIXTURE_BOOK,
            &ast,
            a_real_trip(),
            &authority,
        );
        let clean = super::project_censusless(
            &mut clean_spine,
            FIXTURE_BOOK,
            &ast,
            a_real_clean_latch(),
            &authority,
        );

        assert!(
            tripped
                .steps
                .iter()
                .all(|step| matches!(step.disposition, Disposition::Run)),
            "a genuine certifier disagreement reaches the terminal demotion before projection"
        );
        assert!(
            clean
                .steps
                .iter()
                .any(|step| matches!(step.disposition, Disposition::Replace(..))),
            "...and an untripped latch spends nothing, or the assertion above proves only that the \
             seat always demotes"
        );
    }

    /// THE PRODUCER FENCE — belt-and-braces since the typed witness landed.
    ///
    /// Four producers projected without spending their latch, silently
    /// (`30Md:fnd-discarded-trip-retains-elisions`), so the set of files that may build a walled
    /// plan is ENUMERATED here and checked both ways: one that stops spending fails, and a FIFTH
    /// producer is a deliberate act with a diff. `TripSpent` now makes the omission uncompilable
    /// at the PROJECTION, and this stays because it binds a different thing: a producer that builds
    /// a walled plan and hands the Spine somewhere else entirely never reaches that seat.
    #[test]
    fn every_plan_producer_spends_its_certifier_trip() {
        // Split so this scan does not find ITSELF — the fence is about production call sites.
        let builds = concat!("build_plan", "_walled(");
        let spends = [
            concat!("project_", "censusless("),
            concat!("spend_certifier_", "trip("),
            concat!("demote_on_", "trip("),
        ];
        let expected = [
            "coverage/src/lib.rs",
            "hostsim/src/lib.rs",
            "plan/src/lib.rs",
            "sweep/src/drive.rs",
        ];

        let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates dir");
        let mut producers: Vec<String> = Vec::new();
        let mut stack = vec![crates.to_path_buf()];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir)
                .expect("readable crates dir")
                .flatten()
            {
                let path = entry.path();
                if path.is_dir() {
                    if path.file_name().is_some_and(|n| n == "target") {
                        continue;
                    }
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let src = std::fs::read_to_string(&path).unwrap_or_default();
                    if !src.contains(builds) {
                        continue;
                    }
                    let shown = path.display().to_string().replace('\\', "/");
                    assert!(
                        spends.iter().any(|needle| src.contains(needle)),
                        "{shown} builds a walled plan and spends no certifier trip"
                    );
                    producers.push(shown);
                }
            }
        }
        producers.sort();
        assert!(
            !producers.is_empty(),
            "discovery floor: the walk found no plan producers, so this fence proves nothing"
        );
        let found: Vec<&str> = producers
            .iter()
            .map(|path| {
                expected
                    .iter()
                    .find(|tail| path.ends_with(*tail))
                    .copied()
                    .unwrap_or(path.as_str())
            })
            .collect();
        assert_eq!(
            found, expected,
            "the plan-producer roster moved; adding one is a governed act, not a local edit"
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

        let (cleanup, spent) = super::spend_certifier_trip(&mut spine, a_real_trip(), |fn_name| {
            fn_name == "apt_get__is_converged"
        });
        let plan = project(&mut spine, &spent);

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

        let (_, spent) = super::spend_certifier_trip(&mut spine, a_real_trip(), |_| false);
        let plan = project(&mut spine, &spent);

        assert!(matches!(plan.steps[0].disposition, Disposition::Run));
    }

    /// THE STRUCTURAL DISSOLUTION (`30M:rec-dissolve-trip-must-remember-structurally`) — the
    /// original adversarial shape, now green because the defect it demonstrated is unspellable.
    ///
    /// A real trip, a spine that really elides, and NOTHING between them but the projection. Filed
    /// red (`1dbca1ab`) against a `project_plan` a producer could reach with its latch still in
    /// hand; the reshaped `a_censusless_producer_spends_its_trip_before_projecting` proved the SEAT
    /// spends, which is a weaker claim — it says the one tail that exists is correct, not that no
    /// other tail can be written. This says the second thing: the witness `project_plan` demands is
    /// minted only by the walk, so a plan cannot exist while a tripped run's elisions do. The
    /// `compile_fail` doctest on `TripSpent` is the half no runtime assertion can carry.
    #[test]
    fn a_tripped_plan_cannot_be_projected_while_it_still_elides() {
        let mut spine = spine_of(a_real_elide_plan().steps);
        let trip = a_real_trip();
        assert!(trip.tripped());

        let plan = projected(&mut spine, trip);

        assert!(
            plan.steps
                .iter()
                .all(|step| matches!(step.disposition, Disposition::Run)),
            "a genuine certifier disagreement must reach the terminal demotion before projection"
        );
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
