//! The shared-region acceptance battery (`plans/30L` §11, the pins this stage owns).
//!
//! Two halves, deliberately. The CENSUS half drives the REAL analyzer over real books — parse,
//! build, census — so what it pins is what the engine actually enumerates. The MEET half
//! synthesizes route admissions, because the bridge from a site's private conclusion into the
//! region plane is the settlement stage's one-line match and does not exist yet; every instance a
//! proof is built around still comes from the real census, so no test invents an identity.
//!
//! Where a cell's target behaviour needs the settlement/render stages, it is pinned RED-FIRST
//! through `internal_tooling::xfail` with the greening lane named, never asserted as if it worked.

#![expect(
    clippy::panic,
    clippy::arithmetic_side_effects,
    reason = "test helpers: panic-based require(), index arithmetic over small fixture vectors"
)]

use dorc_analysis::cfg::build;
use dorc_core::influence::Influenced;
use dorc_core::region::{ElisionRegion, RegionUniverse};
use dorc_core::{AstId, EntityRef, FactKey, Interner, KindId, SelectorId, SourceFileId};
use dorc_plan::StandIn;
use dorc_plan::region::{
    RegionCensus, RouteAdmission, RouteConclusion, RouteInstance, RoutePopulation,
    RouteRegionProof, SharedOutcome, SharedRegionAct, SharedStandIn, census, decide_region,
};
use dorc_syntax::parse;

const BOOK: SourceFileId = SourceFileId(0);

fn book_universe() -> RegionUniverse {
    RegionUniverse::of_book_custody_files([BOOK])
}

/// Census over `src`, with the book admitted to the region universe.
fn census_of(src: &str) -> RegionCensus {
    let parsed = parse(src);
    let built = build(&parsed.value);
    census(
        &parsed.value,
        &built.value,
        &built.diags,
        &book_universe(),
        BOOK,
    )
}

/// Census over `src` with an EMPTY universe — the world where no file is book-custody.
fn census_with_no_universe(src: &str) -> RegionCensus {
    let parsed = parse(src);
    let built = build(&parsed.value);
    census(
        &parsed.value,
        &built.value,
        &built.diags,
        &RegionUniverse::default(),
        BOOK,
    )
}

#[track_caller]
fn require<T>(opt: Option<T>, msg: &str) -> T {
    match opt {
        Some(v) => v,
        None => panic!("{msg}"),
    }
}

/// The census's regions, in order, paired with their populations.
fn regions(c: &RegionCensus) -> Vec<(ElisionRegion, RoutePopulation)> {
    c.regions()
        .map(|(region, population)| (*region, population.clone()))
        .collect()
}

/// The sole region of a census that is expected to hold exactly one.
#[track_caller]
fn sole_region(c: &RegionCensus) -> (ElisionRegion, RoutePopulation) {
    let all = regions(c);
    assert_eq!(all.len(), 1, "expected exactly one region: {all:?}");
    all.into_iter().next().unwrap_or_else(|| unreachable!())
}

#[track_caller]
fn closed_routes(population: &RoutePopulation) -> Vec<RouteInstance> {
    match population {
        RoutePopulation::Closed(routes) => routes.routes().copied().collect(),
        RoutePopulation::Open => panic!("expected a closed population"),
    }
}

fn fact(selector: &str) -> FactKey {
    let mut interner = Interner::default();
    FactKey::cell(
        KindId(interner.intern("sm.dorc.Package")),
        EntityRef::Singleton,
        SelectorId(interner.intern(selector)),
    )
}

fn guard_conclusion(bytes: &str) -> RouteConclusion {
    RouteConclusion::Guard {
        fact: fact("installed"),
        canonical: bytes.to_owned(),
    }
}

fn proofs_of(routes: &[RouteInstance], conclusions: &[RouteConclusion]) -> Vec<RouteRegionProof> {
    assert_eq!(routes.len(), conclusions.len(), "one conclusion per route");
    routes
        .iter()
        .zip(conclusions.iter())
        .map(|(route, conclusion)| {
            RouteRegionProof::new(*route, RouteAdmission::project(conclusion), None)
        })
        .collect()
}

// ===========================================================================
// The census half — over the real analyzer
// ===========================================================================

/// A book with no eligible calls yields NO regions at all. This is the byte-identity floor
/// (`30L:pin-empty-function-world-parity`): with nothing to group, the region plane has nothing to
/// say, so the engine behaves exactly as it did before it existed.
#[test]
fn a_book_with_no_calls_has_no_regions() {
    assert!(census_of("apt-get install -y nginx\nufw allow 443/tcp\n").is_empty());
}

/// The dorc-lang exclusion, reached through the census rather than the mint
/// (`30L:pin-region-universe-excludes-dorc-lang`): with the book outside the region universe, the
/// same book that yields a region yields none. The call site's own disposition is untouched either
/// way — the census reads no disposition at all.
#[test]
fn a_book_outside_the_universe_yields_no_regions() {
    let src = "helper() { apt-get install -y nginx; }\nhelper\n";
    assert_eq!(census_of(src).len(), 1);
    assert!(census_with_no_universe(src).is_empty());
}

/// Two calls to one definition are TWO routes over ONE region — the whole mechanism, in its
/// smallest form. The region is the authored span; the routes are the clones.
#[test]
fn two_calls_to_one_definition_are_two_routes_over_one_region() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (_, population) = sole_region(&census);
    let routes = closed_routes(&population);
    assert_eq!(routes.len(), 2, "two invocations, two routes");
    assert_eq!(
        routes[0].region(),
        routes[1].region(),
        "both routes edit ONE authored region"
    );
    assert_ne!(
        routes[0].cfg_node(),
        routes[1].cfg_node(),
        "each instance is its own clone"
    );
    assert_ne!(
        routes[0].invocation(),
        routes[1].invocation(),
        "each instance names its own invocation"
    );
}

/// The motivating wrapped factoring, censused: `main → task_fn → helper` yields one region per body
/// command, each with its own closed population. The depth-3 shape is what the re-sized splice
/// budget bought (`30L:req-census-admits-the-wrapped-book`), and this is the census reading of it.
#[test]
fn the_wrapped_factoring_censuses_every_body_region() {
    let census = census_of(
        "helper() { apt-get install -y nginx; }\n\
         task_fn() { helper; systemctl enable --now nginx; }\n\
         main() { task_fn; ufw allow 443/tcp; }\n\
         main\n",
    );
    assert_eq!(
        census.len(),
        3,
        "one region per authored body command: the install, the enable, the allow"
    );
    for (region, population) in census.regions() {
        assert_eq!(
            closed_routes(population).len(),
            1,
            "each region has exactly one invocation in this book: {region:?}"
        );
    }
}

/// `30L:pin-definition-not-name` at census tier: two same-named definitions never pool. The engine
/// refuses to inline a redefined name at all, so the population is also Open — but the REGIONS stay
/// distinct, which is the property that must survive the day inlining admits a redefinition.
#[test]
fn same_named_definitions_never_share_a_population() {
    let census =
        census_of("p() { apt-get install -y nginx; }\np\np() { apt-get install -y curl; }\np\n");
    let all = regions(&census);
    let distinct: std::collections::BTreeSet<_> =
        all.iter().map(|(region, _)| region.definition()).collect();
    assert_eq!(
        distinct.len(),
        all.len(),
        "each region's definition is its own: {all:?}"
    );
}

/// `30L:rul-call-census-must-be-closed`: a refused inline OPENS the population of every region in
/// that function, because some execution of it is not in the enumerated set. Here the opener is
/// recursion.
#[test]
fn a_refused_inline_opens_the_population() {
    let census = census_of("p() { apt-get install -y nginx; p; }\np\n");
    let all = regions(&census);
    assert_eq!(
        all.len(),
        2,
        "the install and the recursive call are both regions"
    );
    assert!(
        all.iter()
            .all(|(_, population)| *population == RoutePopulation::Open),
        "a recursion refusal opens every region of that function: {all:?}"
    );
}

/// An unmodeled EXTERNAL command never opens a census: external commands cannot invoke shell
/// functions, so silence about `hork` says nothing about who calls `p`
/// (`30L:rul-call-census-must-be-closed`'s opener list, stated as an exclusion).
#[test]
fn an_unmodeled_external_command_does_not_open_the_census() {
    let census = census_of("p() { apt-get install -y nginx; }\nhork tune-packages\np\n");
    let (_, population) = sole_region(&census);
    assert!(
        matches!(population, RoutePopulation::Closed(_)),
        "an opaque external command is not a shell-level dynamic construct"
    );
}

/// A shell-level DYNAMIC construct opens every census in the unit: an `eval` can name any function,
/// and silence never means "no other calls" (`silence-licenses-nothing`).
#[test]
fn a_dynamic_execution_construct_opens_every_census() {
    let census = census_of("p() { apt-get install -y nginx; }\np\neval \"$cmd\"\n");
    let (_, population) = sole_region(&census);
    assert_eq!(population, RoutePopulation::Open);
}

/// `30L:pin-loop-population-open-until-proven` — the EXPECTED-OPEN literal-loop cell. A call inside
/// an authored `for` over a LITERAL list is many evaluations, and today the census cannot enumerate
/// them, so the population is Open and the region runs. That is the CURRENT truth; the target is
/// the next test.
#[test]
fn a_literal_loop_population_is_open_today() {
    let census = census_of(
        "install_pkg() { apt-get install -y nginx; }\nfor pkg in nginx curl; do install_pkg; done\n",
    );
    let (_, population) = sole_region(&census);
    assert_eq!(
        population,
        RoutePopulation::Open,
        "no loop-specific optimistic default exists anywhere"
    );
}

/// The TARGET for the literal-loop cell, red-first: propagation closes the population into one
/// route per ordered member, and the universal meet then runs over both. The greening trigger is
/// the loop-propagation lane, NOT this stage — `30L` §7 stages the representation and defers the
/// value-plane work that fills it.
#[test]
fn p_x_loop_population_closes_over_literal_members() {
    let census = census_of(
        "install_pkg() { apt-get install -y nginx; }\nfor pkg in nginx curl; do install_pkg; done\n",
    );
    let (_, population) = sole_region(&census);
    internal_tooling::xfail::xfail_until("p-x-loop-population-closes-over-literal-members", || {
        let routes = match &population {
            RoutePopulation::Closed(routes) => routes.routes().copied().collect::<Vec<_>>(),
            RoutePopulation::Open => Vec::new(),
        };
        assert_eq!(routes.len(), 2, "one route per ordered loop member");
        assert_ne!(
            routes[0].iteration(),
            routes[1].iteration(),
            "the members are told apart by the iteration axis, not by CFG node"
        );
    });
}

/// `30L:pin-census-is-execution-not-scope`. The census quantifies over what may EXECUTE, so a book
/// whose two invocations sit on opposite sides of a poison wall — one of them past a wall that no
/// mode would bother checking — still enumerates BOTH. Nothing in the census reads a probe result,
/// a records fold, or any selection of what was checked.
#[test]
fn the_census_counts_every_executing_invocation_whatever_was_checked() {
    let census = census_of(
        "install_pkg() { apt-get install -y nginx; }\ninstall_pkg\nhork tune-packages\ninstall_pkg\n",
    );
    let (_, population) = sole_region(&census);
    assert_eq!(
        closed_routes(&population).len(),
        2,
        "the invocation past the wall is a route exactly like the one before it"
    );
}

/// `30L:pin-probe-site-identity-unchanged`, in the form this stage can hold: route identity is a
/// pure function of the analysis, and it carries no dispatch, batch, or attempt dimension — there
/// is no field for one. Censusing the same book twice yields identical instances, so nothing a
/// later dispatch could vary is in the key.
#[test]
fn route_identity_is_stable_and_carries_no_dispatch_dimension() {
    let src = "install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n";
    let first = closed_routes(&sole_region(&census_of(src)).1);
    let second = closed_routes(&sole_region(&census_of(src)).1);
    assert_eq!(
        first, second,
        "route identity is a function of the book alone"
    );
}

// ===========================================================================
// The meet half
// ===========================================================================

/// `30L:pin-open-route-runs` — one unenumerated invocation forces Run for every region it may
/// execute, WITHOUT consulting a proof. Even handed a proof that would otherwise replace.
#[test]
fn an_open_population_runs_whatever_the_proofs_say() {
    let census = census_of("p() { apt-get install -y nginx; p; }\np\n");
    for (region, population) in regions(&census) {
        let decision = decide_region(region, &population, &[]);
        assert_eq!(*decision.outcome(), SharedOutcome::Run);
        assert_eq!(decision.act(), SharedRegionAct::MayMutateEveryInstance);
    }
}

/// The agreeing-twin-calls cell: two invocations of one region, both admitting the SAME
/// observable-preserving replacement, meet to Replace. The cells they establish differ (`nginx` vs
/// `curl` in the motivating shape) and that is fine — each route carries its own license for its
/// own cell, and what must agree is the EDIT (`30L:rul-shared-edit-reproduces-every-route`).
#[test]
fn agreeing_twin_calls_meet_to_one_replacement() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let proofs = proofs_of(
        &routes,
        &[
            RouteConclusion::Replace(StandIn::True),
            RouteConclusion::Replace(StandIn::True),
        ],
    );
    let decision = decide_region(region, &population, &proofs);
    assert_eq!(
        *decision.outcome(),
        SharedOutcome::Replace(require(
            match decision.outcome() {
                SharedOutcome::Replace(stand_in) => Some(*stand_in),
                _ => None,
            },
            "the outcome is a replacement"
        ))
    );
    assert_eq!(decision.act(), SharedRegionAct::RetiresEveryInstance);
    assert_eq!(decision.contributing().len(), 2);
}

/// `30L:pin-no-singleton-special-case` — a one-route population takes the same path as a two-route
/// one. Nothing branches on cardinality, so a lone invocation is not a shortcut and cannot acquire
/// an answer the general meet would refuse.
#[test]
fn cardinality_one_falls_out_of_the_general_meet() {
    let single = census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\n");
    let (one_region, one_population) = sole_region(&single);
    let one = decide_region(
        one_region,
        &one_population,
        &proofs_of(
            &closed_routes(&one_population),
            &[RouteConclusion::Replace(StandIn::True)],
        ),
    );

    let twin = census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (twin_region, twin_population) = sole_region(&twin);
    let two = decide_region(
        twin_region,
        &twin_population,
        &proofs_of(
            &closed_routes(&twin_population),
            &[
                RouteConclusion::Replace(StandIn::True),
                RouteConclusion::Replace(StandIn::True),
            ],
        ),
    );
    assert_eq!(
        one.outcome(),
        two.outcome(),
        "one route and two agreeing routes reach the same outcome by the same rule"
    );
}

/// `30L:pin-every-route-meets` — mutating ONE route's property to failure reddens the shared
/// elision. The disagreeing route here is an ordinary Run, which is where every failed property
/// lands.
#[test]
fn one_failing_route_forces_run_for_the_whole_region() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let proofs = proofs_of(
        &routes,
        &[
            RouteConclusion::Replace(StandIn::True),
            RouteConclusion::Run,
        ],
    );
    assert_eq!(
        *decide_region(region, &population, &proofs).outcome(),
        SharedOutcome::Run
    );
}

/// `30L:pin-common-replacement-observables` — two routes that would each replace, but reproduce
/// DIFFERENT statuses, have no single stand-in and therefore Run. Equivalence is semantic: both
/// routes carry the tag `Replace`, and that is not enough.
#[test]
fn differing_reproduced_statuses_run() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let proofs = proofs_of(
        &routes,
        &[
            RouteConclusion::Replace(StandIn::True),
            RouteConclusion::Replace(StandIn::Exit(9)),
        ],
    );
    assert_eq!(
        *decide_region(region, &population, &proofs).outcome(),
        SharedOutcome::Run
    );
}

/// Two routes admitting the SAME parametric guard meet to Guard — and a Guard leaves the authored
/// bytes able to execute, so its act is may-mutate exactly as a Run's is
/// (`plan/CLAUDE.md only-a-proof-retires-a-wall`).
#[test]
fn agreeing_guards_meet_to_one_guard_that_still_walls() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let proofs = proofs_of(
        &routes,
        &[
            guard_conclusion("fn=p__is_converged inv=p__is_converged nginx preamble=BODY"),
            guard_conclusion("fn=p__is_converged inv=p__is_converged nginx preamble=BODY"),
        ],
    );
    let decision = decide_region(region, &population, &proofs);
    assert!(matches!(decision.outcome(), SharedOutcome::Guard(_)));
    assert_eq!(decision.act(), SharedRegionAct::MayMutateEveryInstance);
}

/// `30L:pin-guard-resolution-is-frame-live` — two instances whose live verdict definitions differ
/// ship different guard bytes, so the shared guard REFUSES. The comparison runs over the guard's
/// decision-relevant bytes rather than its defining span, which keeps a display-tier value out of
/// a licence decision while still separating two authors' bodies.
#[test]
fn divergent_live_guard_definitions_refuse_the_shared_guard() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let proofs = proofs_of(
        &routes,
        &[
            guard_conclusion("fn=p__is_converged inv=p__is_converged nginx preamble=BODY-A"),
            guard_conclusion("fn=p__is_converged inv=p__is_converged nginx preamble=BODY-B"),
        ],
    );
    assert_eq!(
        *decide_region(region, &population, &proofs).outcome(),
        SharedOutcome::Run
    );
}

/// The divergent-facts-one-guard cell, at its CURRENT behaviour: one converged route and one
/// diverged route meet to Run today, because the engine's guard tier is freshness-driven and a
/// diverged-but-vouched site concludes Run rather than admitting a guard. Named interim, per the
/// xfail seat's own rule that an interim assertion lives in its own test.
#[test]
fn interim_divergent_route_facts_run_rather_than_guarding() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let proofs = proofs_of(
        &routes,
        &[
            RouteConclusion::Replace(StandIn::True),
            RouteConclusion::Run,
        ],
    );
    assert_eq!(
        *decide_region(region, &population, &proofs).outcome(),
        SharedOutcome::Run
    );
}

/// The TARGET of that cell, red-first (`30L` §4.5): where route facts DIVERGE but every route
/// admits the SAME invocation-parametric guard, Guard absorbs what Replace cannot. Two things are
/// missing and both belong to later stages: a diverged route must be able to ADMIT a guard, and the
/// guard's argv must be the SOURCE-level expression rather than each site's resolved operands.
#[test]
fn p_x_divergent_routes_share_one_parametric_guard() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    internal_tooling::xfail::xfail_until("p-x-divergent-routes-share-one-parametric-guard", || {
        let proofs = proofs_of(
            &routes,
            &[
                RouteConclusion::Replace(StandIn::True),
                RouteConclusion::Run,
            ],
        );
        assert!(matches!(
            decide_region(region, &population, &proofs).outcome(),
            SharedOutcome::Guard(_)
        ));
    });
}

/// Agreeing source-level Omits meet to Omit. Controllers must agree: two clones of one body share
/// their controller's `AstId` by construction (the AST is the definition's), so this is near-free
/// where it is right and refuses where the shared render would have no single provenance.
#[test]
fn agreeing_omits_meet_to_one_omit() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let agreeing = proofs_of(
        &routes,
        &[
            RouteConclusion::Omit {
                controller: AstId(3),
            },
            RouteConclusion::Omit {
                controller: AstId(3),
            },
        ],
    );
    assert_eq!(
        *decide_region(region, &population, &agreeing).outcome(),
        SharedOutcome::Omit {
            controller: AstId(3)
        }
    );
    let disagreeing = proofs_of(
        &routes,
        &[
            RouteConclusion::Omit {
                controller: AstId(3),
            },
            RouteConclusion::Omit {
                controller: AstId(4),
            },
        ],
    );
    assert_eq!(
        *decide_region(region, &population, &disagreeing).outcome(),
        SharedOutcome::Run
    );
}

/// `30L:pin-influence-joins-most` — the shared decision carries influence when ANY contributing
/// route was host-influenced. One uninfluenced route never cleanses an influenced sibling, and the
/// grade cannot be lowered on the way out because `core::influence` has no lowering conversion at
/// all: the join is a carry, not an arithmetic.
#[test]
fn one_influenced_route_influences_the_shared_decision() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let influenced = Influenced::authored_before_contact(()).widen();
    let proofs = vec![
        RouteRegionProof::new(
            routes[0],
            RouteAdmission::project(&RouteConclusion::Replace(StandIn::True)),
            None,
        ),
        RouteRegionProof::new(
            routes[1],
            RouteAdmission::project(&RouteConclusion::Replace(StandIn::True)),
            Some(influenced),
        ),
    ];
    assert!(
        decide_region(region, &population, &proofs)
            .influence()
            .is_some(),
        "the uninfluenced route did not cleanse its influenced sibling"
    );
}

/// The meet quantifies over the population the CENSUS proved, never over whatever a caller handed
/// it. A short proof list — one route's answer standing in for two — is Run, which is the same
/// mistake-shape `pin-shared-witness-spans-instances` forbids one level up.
#[test]
fn proofs_that_do_not_cover_the_population_run() {
    let census =
        census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    let routes = closed_routes(&population);
    let short = proofs_of(&routes[..1], &[RouteConclusion::Replace(StandIn::True)]);
    assert_eq!(
        *decide_region(region, &population, &short).outcome(),
        SharedOutcome::Run
    );
}

/// The mixed-body cell, and the whole reason this stage exists: a body holding one converged region
/// and one live one decides them INDEPENDENTLY. Under the all-or-nothing call license the live
/// command forfeited the whole body; under region decisions it forfeits only itself.
#[test]
fn a_mixed_body_decides_its_regions_independently() {
    let census = census_of("task() { apt-get install -y nginx; hork tune-packages; }\ntask\n");
    let all = regions(&census);
    assert_eq!(all.len(), 2, "two authored regions in one body: {all:?}");
    let outcomes: Vec<SharedOutcome> = all
        .iter()
        .enumerate()
        .map(|(index, (region, population))| {
            let conclusion = if index == 0 {
                RouteConclusion::Replace(StandIn::True)
            } else {
                RouteConclusion::Run
            };
            let proofs = proofs_of(&closed_routes(population), &[conclusion]);
            decide_region(*region, population, &proofs)
                .outcome()
                .clone()
        })
        .collect();
    assert_eq!(
        outcomes,
        vec![
            SharedOutcome::Replace(SharedStandIn::of(StandIn::True)),
            SharedOutcome::Run
        ],
        "the live region's failure does not reach its converged neighbour"
    );
}

/// The branch-join cell (`30L` §3.1). Each arm of a branch inside a body is its own region with its
/// own population, so one arm's failure never travels to the other — while the JOINED region after
/// the branch still answers on its own route, whose per-invocation proof the CFG meet already
/// resolved before it ever reached here.
#[test]
fn branch_arms_are_separate_regions_and_one_arms_failure_stays_there() {
    let census = census_of(
        "task() { if hork check; then apt-get install -y nginx; else apt-get install -y curl; fi\n\
         systemctl enable --now nginx; }\ntask\n",
    );
    let all = regions(&census);
    assert!(
        all.len() >= 3,
        "the condition, both arms, and the joined command are their own regions: {all:?}"
    );
    for (region, population) in &all {
        assert_eq!(
            closed_routes(population).len(),
            1,
            "one invocation, so one route per region: {region:?}"
        );
    }
}

/// The consumed-vs-dead call cell: the SAME region serves an invocation whose status a branch reads
/// and one whose status nobody reads. Both are routes; what differs between them is a per-route
/// property (`30L` §4.3's status trichotomy), which lands in each route's own conclusion and meets
/// universally — so a region serving a status-consuming call is exactly as hard to transform as its
/// strictest route.
#[test]
fn one_region_serves_a_status_consuming_call_and_a_bare_one() {
    let census = census_of(
        "install_pkg() { apt-get install -y nginx; }\ninstall_pkg\nif install_pkg; then hork ok; fi\n",
    );
    let all = regions(&census);
    let body = require(
        all.iter().find(|(_, population)| {
            matches!(population, RoutePopulation::Closed(routes) if routes.count() == 2)
        }),
        "the body region serves both invocations",
    );
    let proofs = proofs_of(
        &closed_routes(&body.1),
        &[
            RouteConclusion::Replace(StandIn::True),
            RouteConclusion::Run,
        ],
    );
    assert_eq!(
        *decide_region(body.0, &body.1, &proofs).outcome(),
        SharedOutcome::Run,
        "the stricter route governs the shared region"
    );
}

/// `30L:rul-whole-helper-is-derived` at the level this stage reaches: when EVERY region of a
/// definition decides Replace, that is a property of the region decisions and nothing more. The
/// call's own elision is a SEPARATE, derived decision that additionally needs the call-level
/// consumed observables reproduced — the render stage's, never a primitive here.
#[test]
fn a_wholly_replaceable_helper_is_all_replace_regions_and_no_call_decision() {
    let census =
        census_of("task() { apt-get install -y nginx; systemctl enable --now nginx; }\ntask\n");
    let all = regions(&census);
    assert_eq!(all.len(), 2);
    for (region, population) in &all {
        let proofs = proofs_of(
            &closed_routes(population),
            &[RouteConclusion::Replace(StandIn::True)],
        );
        let decision = decide_region(*region, population, &proofs);
        assert_eq!(decision.act(), SharedRegionAct::RetiresEveryInstance);
    }
    assert!(
        census.population(all[0].0).is_some(),
        "the census speaks only about regions; no call-level decision is minted here"
    );
}

/// `30L:inv-closed-route-set-never-empty` reached from the other side: no proofs at all is Run, so
/// a universal quantifier is never satisfied vacuously. An unreached definition acquires no
/// authority.
#[test]
fn no_proofs_at_all_is_run_never_a_vacuous_yes() {
    let census = census_of("install_pkg() { apt-get install -y nginx; }\ninstall_pkg\n");
    let (region, population) = sole_region(&census);
    assert_eq!(
        *decide_region(region, &population, &[]).outcome(),
        SharedOutcome::Run
    );
}
