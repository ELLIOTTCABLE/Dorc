//! The analyzed world a `dorc why` report is ABOUT, assembled from source alone.
//!
//! This is the harness half of the loom seam (`lib-target-is-a-loom-seam`): the binary builds its
//! world out of files, flags, a clock and a host's records, and every one of those is a QUERY that
//! stays on its side. What a loom case can hand across is SOURCE — a book, its oracles — so this
//! seat takes exactly that and runs the same kernel calls in the same order, which is what makes a
//! committed why transcript an honest render rather than a decoration
//! (`289:rul-worldless-route-honest-trigger`).
//!
//! MEASURED worlds arrive through the same intake a run uses. A case's own `dorc-records/1` bytes
//! are admitted by [`crate::results::admit_fixture_records`] — a second CONTROLLER of its own
//! hermetic in-process run, never a second scope and never an unframed side door
//! (`28L:rul-records-seam-approved`) — and the admitted [`SiteResults`] arrive here as a VALUE, so
//! this seat still opens nothing. With no records the fold is ⊤ everywhere and every site runs,
//! which is the honest unmeasured world rather than a scope cut.
//!
//! RESIDUAL SCOPE CUT, stated where it bites (`churn-avoidance-disclosure`): the wrapper PEEL is not
//! threaded (`peeled` is empty), so a book whose sites sit under a wrapper classifies here as an
//! ordinary run would classify them unwrapped. A case exercising wrapper adoption belongs on the
//! diagnostic plane (`crate::survival::survival_diagnostics`) until the peel is threaded through
//! this seat too.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::CollapseNarrative;
use dorc_aid::diag::Diag;
use dorc_core::{Interner, ProvArena, Symbol};

use crate::Receipt;
use crate::results::{SiteResults, probe_origins};
use dorc_analysis::load::{LoadControl, LoadStep, LoadTarget, TargetPart};

use crate::snapshot::StaticLoadSnapshot;
use crate::why::{
    CascadeAttribution, FirstWallHint, WallStep, WhyReport, collect_wall_steps, first_wall_hint,
};

/// Everything a why report reads, owned in one place so a caller can borrow a [`WhyReport`] out of
/// it without threading seventeen lifetimes of its own.
pub struct WhyWorld {
    /// The one immutable authored input this world was analysed from (`30I` §3.1). Held whole
    /// rather than shredded into four fields, so the why driver and the run cannot be handed
    /// different worlds (`one-definition-table-two-drivers`).
    snapshot: StaticLoadSnapshot,
    interner: Interner,
    arena: ProvArena,
    ast: dorc_syntax::ast::Ast,
    spine: dorc_plan::Spine,
    plan: dorc_plan::Plan,
    probe: dorc_plan::ProbePlan,
    narrative: Vec<CollapseNarrative>,
    why_diags: Vec<Diag>,
    refusals: Vec<Diag>,
    wall_steps: Vec<WallStep>,
    first_wall: Option<FirstWallHint>,
    cascades: BTreeMap<dorc_plan::LeafId, CascadeAttribution>,
}

impl std::fmt::Debug for WhyWorld {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WhyWorld")
            .field("book", &self.snapshot.book_path())
            .finish_non_exhaustive()
    }
}

impl WhyWorld {
    /// Analyze `book_src` against `oracle_srcs` with no measurements — every fact ⊤, every site runs.
    #[must_use]
    pub fn analyze(snapshot: &StaticLoadSnapshot) -> Self {
        Self::analyze_measured(snapshot, &SiteResults::default(), false)
    }

    /// Analyze `book_src` against `oracle_srcs` and build the plan a why report explains.
    ///
    /// The same call sequence the binary runs — lift, parse, CFG, value-flow, classify, vouch,
    /// compile the probe, fold the records, lift the survival footprints, build the plan. Stage
    /// diagnostics are DROPPED rather than printed: this seat has no stderr, and a case that wants a
    /// diagnostic rendered drives the diagnostic route instead.
    ///
    /// `results` is the run's ADMITTED records; an empty one is the unmeasured world (⊤ everywhere ⇒
    /// run), which is what a case with no `< results` redirect asks for. `consented` is
    /// `--risk-faultless-skips`, and with it off the survival half is not merely quiet but ABSENT:
    /// no `touches()` is lifted, no footprint exists, and every running mutator is the honest
    /// Stage-1 total wall (`empty-world-byte-identical`).
    ///
    /// The validity fixpoint runs here, the binary's own rounds over the binary's own frozen model
    /// (`crate::fixpoint`), so a cascaded elision can be explained with the round that caused it
    /// rather than merely reported.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "one linear pipeline in the binary's own order; splitting it would let the two orders drift, which is the whole thing this seat exists to prevent"
    )]
    pub fn analyze_measured(
        snapshot: &StaticLoadSnapshot,
        results: &SiteResults,
        consented: bool,
    ) -> Self {
        let mut interner = Interner::default();
        let mut arena = ProvArena::new();
        let book_src = snapshot.book_src();
        let oracle_srcs = snapshot.oracle_srcs();
        let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();
        // SOURCE-WIDE, exactly as the binary's `source_table` builds it: the oracles in load order,
        // then the book, which is an ordinary definition source
        // (`the-book-is-a-definition-source`). This seat used to lift oracle-only vectors and site
        // the book one PAST them, so a site a book definition owned withheld here while the run
        // answered it — safe, but a why report that explains a different world than the run is a
        // decoration, which is the failure `one-definition-table-two-drivers` exists to prevent.
        let source_srcs = snapshot.source_srcs();
        let source_refs: Vec<&str> = snapshot.source_refs();
        let source_path_refs: Vec<&str> =
            snapshot.source_paths().iter().map(String::as_str).collect();

        let parsed = dorc_syntax::parse(book_src);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);

        let mut degrades = BTreeMap::new();
        let mut verdict_lane = BTreeMap::new();
        let peeled = BTreeMap::new();
        let definitions = definition_table(snapshot, &parsed.value);
        let env = {
            let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
            dorc_analysis::funcenv::analyze(&parsed.value, &cfg.value, &definitions, &plane)
        };
        let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &definitions);
        // THE EDGE, mirrored: the widening above is exactly what
        // `withdrawal-is-applied-once-never-consulted` requires to route through here first, so the
        // contested fact is minted from the same two calls in the same order the binary uses and
        // applied ONCE to every lifted set below.
        let contested = {
            let shadows =
                dorc_analysis::funcenv::contests(&parsed.value, &cfg.value, &definitions, &env);
            let unprovable =
                dorc_analysis::funcenv::unprovable(&definitions, &env, cfg.value.exit());
            dorc_core::ContestedFamilies::new(
                shadows
                    .iter()
                    .map(|c| c.name.as_str())
                    .chain(unprovable.iter().map(String::as_str))
                    .filter_map(|name| {
                        dorc_oracle::reserved::role_family(name).map(|(base, _)| base.to_owned())
                    }),
            )
        };
        let never_live = dorc_analysis::funcenv::never_live(&definitions, &env);

        // One non-role-declaration index per unit, shared by the ship seams and the vouch lift. The
        // book is the LAST source here too (`one-definition-table-two-drivers`), so the why driver's
        // custody predicate sees the same census the run's does.
        // The include-tree, derived from the same vectors by the same rule the binary uses, so the
        // why driver's custody predicate answers over the run's own closures rather than a
        // singleton world that would explain suspensions the run never made.
        let book_index = Some(snapshot.book_index());
        let include_tree = crate::sourcing::include_tree(snapshot, &env);
        let helpers = dorc_oracle::closure::HelperIndex::build(&source_refs, book_index)
            .with_include_tree(
                dorc_core::CustodyClosures::from_edges(source_refs.len(), &include_tree.edges),
                include_tree.unresolved,
            )
            .with_selection(dorc_core::CustodyClosures::from_edges(
                source_refs.len(),
                &include_tree.selected,
            ));
        let checks: Vec<dorc_oracle::predict::PredictSet> = source_refs
            .iter()
            .map(|src| {
                dorc_oracle::predict::lift_predicts(&mut interner, src)
                    .value
                    .withdrawing(&contested, &interner)
            })
            .collect();
        let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = source_refs
            .iter()
            .map(|src| {
                dorc_oracle::verdict::VerdictSet::lift(&mut interner, src)
                    .value
                    .withdrawing(&contested, &interner)
            })
            .collect();
        let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);
        let dead_predicts = never_live_predict_rows(&never_live, &checks, &interner);
        let idx = dorc_oracle::lift_from_sets(&mut interner, &checks, |file, provider| {
            !dead_predicts.contains(&(file, provider))
        })
        .value
        .withdrawing(&contested, &interner);
        // The run's own latch (`302:rul-certifier-trip-guard-only`), threaded through the same
        // rounds the binary threads it through: a why report built over an un-demoted plan would
        // explain elisions the run would never have emitted — a decoration, which is exactly what
        // `one-definition-table-two-drivers` exists to prevent.
        let mut trip = dorc_analysis::certify::CertifierTrip::default();
        record_pre_network_trip(&mut trip, &value, &env);
        let frozen = crate::fixpoint::FrozenModel {
            cfg: &cfg.value,
            value: &value,
            ast: &parsed.value,
            idx: &idx,
            checks: &checks,
            verdicts: &verdicts,
            peeled: &peeled,
            live,
        };
        let origin = crate::fixpoint::classify_round(
            &frozen,
            &dorc_analysis::erase::ErasedSites::none(),
            &mut interner,
            &mut arena,
            &mut degrades,
            &mut verdict_lane,
            &mut trip,
        );
        let classes = origin.classes.clone();
        let (kills, kill_coords) = (origin.kills.clone(), origin.kill_coords.clone());

        let (vouch_lift, vouch_aid) = dorc_plan::build_vouches(
            &source_refs,
            &source_path_refs,
            &helpers,
            &classes,
            &value,
            &mut interner,
            live,
        );
        let vouches = vouch_lift.value;

        let ship = |node: dorc_analysis::cfg::CfgNodeId, provider: Symbol, argv: &[Symbol]| {
            ship_predict_body(
                source_srcs,
                &helpers,
                &checks,
                &interner,
                provider,
                argv,
                node,
                live,
            )
        };
        let ship_auto = |node: dorc_analysis::cfg::CfgNodeId,
                         subjects: &[dorc_core::FactKey],
                         provider: Symbol,
                         _: &[Symbol]| {
            verdict_lane
                .get(&node)
                .is_some_and(|measurement| measurement.subjects() == subjects)
                .then(|| {
                    ship_verdict_body(
                        source_srcs,
                        &helpers,
                        &verdict_sets,
                        &interner,
                        provider,
                        node,
                        live,
                    )
                })
                .flatten()
        };
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg.value,
            &value,
            &classes,
            &dorc_plan::WrappedProbes::new(),
            &dorc_plan::ConnectedPipes::default(),
            ship,
            ship_auto,
            |node, fact| vouches.get(node, fact).is_some(),
        )
        .with_unresolvable_causes(&parsed.value, &cfg.value, &classes, &degrades);

        // The survival tier, flag-gated exactly as a run is (`rul24-mode-gate`, TC-1): unflagged,
        // the footprint data does not exist at all, so a running mutator walls totally.
        // Withdrawn at the edge like every other lifted set. The VECTORS stay oracle-only here and
        // in the binary alike — the kind-owner trio loads from the ambient prefix by design
        // (`vocabulary-acts-stay-ambient`), and widening the survival lane's own reach is a
        // separate dispatch (`one-helper-index-two-lanes`) — but oracle-only is a question about
        // WHICH files, never about whether the contested fact applies to them.
        let touches_paired =
            crate::survival::pair_touches_sets(&oracle_refs, &mut interner, &contested);
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        let coord_kinds = crate::survival::collect_coord_kinds(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &mut interner,
            live,
        );
        let kind_resolvers = crate::kinds::build_kind_resolvers(
            oracle_srcs,
            &checks,
            &touches_paired,
            &coord_kinds,
            &mut interner,
        )
        .value;
        let resolver_kinds: BTreeSet<Symbol> = kind_resolvers.resolver_kinds().collect();
        let kind_reaches = crate::kinds::build_kind_reaches(
            oracle_srcs,
            &checks,
            &touches_paired,
            &coord_kinds,
            &mut interner,
        )
        .value;
        let reach_kinds: BTreeSet<Symbol> = kind_reaches.reach_kinds().collect();

        let survival = consented.then(|| {
            let derivations = {
                let derive = |n, p, a: &[Symbol]| {
                    crate::survival::ship_touches_body(
                        &touches_paired,
                        &helpers,
                        &interner,
                        p,
                        a,
                        n,
                        live,
                    )
                };
                dorc_plan::compile_derivations(
                    &parsed.value,
                    &cfg.value,
                    &value,
                    &classes,
                    &kills,
                    derive,
                )
            };
            let mut footprints = crate::survival::build_survival_footprints(
                &touches_sets,
                &classes,
                &kills,
                &kill_coords,
                &value,
                &cfg.value,
                &parsed.value,
                &mut interner,
                live,
            )
            .value;
            let derived_node_spans: BTreeMap<_, _> = derivations
                .derivations
                .iter()
                .map(|d| (d.node, parsed.value.node(cfg.value.node(d.node).ast).span))
                .collect();
            let _ = crate::survival::merge_derived_footprints(
                &mut footprints,
                &derivations,
                results,
                &classes,
                &kill_coords,
                &derived_node_spans,
                &mut interner,
            );
            // The reach EXPANSION must not be skipped: a footprint is an AT-MOST claim, so an
            // un-widened one looks disjoint from more cells than it is — the under-execute
            // direction (`inv-kfail`).
            let reach_node_spans: BTreeMap<_, _> = footprints
                .nodes()
                .map(|n| (n, parsed.value.node(cfg.value.node(n).ast).span))
                .collect();
            let _ = crate::survival::expand_footprints_via_reaches(
                &mut footprints,
                &kind_reaches,
                &reach_kinds,
                results,
                &reach_node_spans,
                &mut interner,
            );
            footprints
        });

        let resolver_coords = if consented && !resolver_kinds.is_empty() {
            crate::survival::collect_resolver_coords(
                &classes,
                &kills,
                &value,
                &touches_sets,
                &resolver_kinds,
                &mut interner,
                live,
            )
        } else {
            BTreeSet::new()
        };
        let mut resolutions = crate::survival::build_resolutions(
            &resolver_coords,
            &resolver_kinds,
            results,
            &mut interner,
        );
        // `fence-no-disjoint` (`24L` §7): every verdict provider's auto-cell kind is registered so
        // the survival tier reads an auto coordinate as MAY-touch. Dropping this would let a
        // synthetic singleton read as provably-disjoint — a wrong survival.
        // Whole-unit and deliberately file-blind: registering a kind only makes coordinates read as
        // MAY-touch, so covering every file's verdict providers errs toward less sparing.
        let verdict_names: Vec<String> = verdicts
            .providers()
            .map(|(_, p)| interner.resolve(p.0).to_owned())
            .collect();
        for name in verdict_names {
            let kind = dorc_core::auto_fact(&mut interner, &name).kind;
            resolutions.add_auto_kind(kind);
        }

        // The dialect the survival tier compares selectors within, hoisted so the policy can
        // borrow it for the whole settlement.
        let dialect = dorc_oracle::build_dialect(&idx);
        let policy = match survival.as_ref() {
            Some(footprints) if consented => dorc_plan::WallPolicy::RiskAccepted {
                footprints,
                resolutions: &resolutions,
                dialect: &dialect,
            },
            _ => dorc_plan::WallPolicy::Honest,
        };
        // The SAME census the run builds (`one-definition-table-two-drivers`).
        let region_universe = dorc_core::region::RegionUniverse::of_book_custody_files(
            source_refs
                .iter()
                .enumerate()
                .filter(|(_, src)| !dorc_oracle::marker::has_marker(src))
                .map(|(index, _)| dorc_analysis::funcenv::source_file_of_index(index)),
        );
        let string_execution = dorc_plan::region::StringExecutionSites::of_unit(&parsed.value);
        let definition_vectors = dorc_oracle::closure::definition_vectors(&source_refs);
        let regions = dorc_plan::region::census(
            &parsed.value,
            &cfg.value,
            &cfg.diags,
            dorc_plan::region::CensusOpeners::of(
                &region_universe,
                env.unresolvable_loads(),
                &definition_vectors,
                &string_execution,
            ),
            snapshot.book_file(),
        );
        let plan_inputs = dorc_plan::SettleInputs {
            src: book_src,
            ast: &parsed.value,
            cfg: &cfg.value,
            vouches: &vouches,
            connected: &dorc_plan::ConnectedPipes::default(),
            policy,
            regions: &regions,
            // A why world reads results somebody already admitted, and reaching for host bytes at
            // all is what makes what follows influenced — so it widens through the one named seat
            // rather than holding a carrier (`307a:dis-phase-by-free-widening`).
            minted_at: Some(crate::results::influence_after_reaching_for_host_bytes()),
        };
        // The settlement, to quiescence, over the frozen origin — the binary's own rounds
        // (`the-fixpoint-owns-the-rounds-and-builds-nothing-else`). Its product beyond the settled
        // decisions is the round-tagged cascade attribution, which is the only way a why report can
        // answer for an elision that only became legal once something upstream was proven dead.
        // The ledger holds CFG SITES (leaves and non-leaves alike) and grows by at least one per
        // non-quiescent round, so the bound is the node count plus the settling round.
        let cap =
            u32::try_from(dorc_analysis::solve::Graph::node_count(&cfg.value).saturating_add(1))
                .unwrap_or(u32::MAX)
                .max(1);
        let settled = crate::fixpoint::settle_world(
            &frozen,
            &probe,
            results,
            &plan_inputs,
            cap,
            &mut interner,
            &mut arena,
            &mut trip,
        );
        let cascades = crate::fixpoint::attribute_cascades(
            &cfg.value,
            &parsed.value,
            book_src,
            &settled.round.classes,
            &settled.ledger,
            &settled.validity,
            &settled.origin_validity,
        );
        let round = settled.round;
        let classes = round.classes;
        let (why_diags, classify_narrative) = (round.why_diags, round.classify_narrative);
        let (by_fact, merge_narrative) = (settled.by_fact, settled.merge_narrative);
        let _ = by_fact;
        let probe_attributions = probe_origins(&probe, results, &mut arena);
        let mut spine = settled.spine;
        dorc_plan::attach_spine_probe_provenance(
            &mut spine,
            &parsed.value,
            &probe_attributions,
            &mut arena,
        );
        // The same whole-artifact emission decision the binary makes, by the same rule: a why report
        // that explained an artifact with different bindings than the run's would be a decoration
        // (`one-definition-table-two-drivers`).
        spine.push_render_decision(dorc_core::spine::SpineRenderDecision {
            site: None,
            decision: dorc_core::spine::RenderDecision::DefensiveEmission {
                defensive: !dorc_oracle::closure::definition_vectors(&source_refs).is_empty()
                    || !env.unresolvable_loads().is_empty(),
            },
            grade: None,
        });
        let (_trip_banner, trip_narrative) =
            demote_on_certifier_trip(&mut spine, trip, &definitions);
        // This world is handed results somebody else already decided about, so the intake authority
        // is the DRIVER's to hold and the driver's refused path never reaches a why world
        // (`the_driver_takes_its_authority_from_its_admission`).
        let plan = dorc_plan::project_plan(&spine, &dorc_plan::PlanAuthority::without_intake());
        dorc_plan::spine::record_render_decisions(&mut spine, &plan, book_src, &parsed.value);
        let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
        let narrative: Vec<CollapseNarrative> = classify_narrative
            .into_iter()
            .chain(vouch_aid.narrative)
            .chain(merge_narrative)
            .chain(plan.survival_report.collapse_narrative().iter().cloned())
            .chain(trip_narrative)
            .chain(plan.render_refusal_narratives(&parsed.value))
            .collect();
        let wall_steps = collect_wall_steps(
            &plan,
            &probe,
            &classes,
            &cfg.value,
            &kills,
            &parsed.value,
            book_src,
        );
        let first_wall = first_wall_hint(&wall_steps);

        WhyWorld {
            snapshot: snapshot.clone(),
            interner,
            arena,
            ast: parsed.value,
            spine,
            plan,
            probe,
            narrative,
            why_diags,
            refusals,
            wall_steps,
            first_wall,
            cascades,
        }
    }

    /// What this world decided, for the receipt's tally row.
    #[must_use]
    pub fn disposition_counts(&self) -> dorc_plan::DispositionCounts {
        self.plan.disposition_counts()
    }

    /// The may-alias fire-rate this world's survival pass recorded (`24F` §3a).
    #[must_use]
    pub fn may_alias_fires(&self) -> u32 {
        self.plan.survival_report.may_alias_fires()
    }

    /// This world's decision digest — the same stable hash the binary prints, from the same inputs.
    #[must_use]
    pub fn decision_digest(&self) -> String {
        let identity: Vec<Diag> = self
            .why_diags
            .iter()
            .cloned()
            .chain(self.refusals.iter().cloned())
            .collect();
        dorc_plan::erasability::decision_digest(
            &self.plan,
            &self.probe,
            self.snapshot.book_src(),
            &self.ast,
            &self.interner,
            &identity,
        )
    }

    /// The plan this world built, the AST its spans index into, and the interner that minted its
    /// symbols.
    ///
    /// MIGRATION SCAFFOLDING (`309` §4, build-to-kill): the decision-state baseline walks the plan
    /// directly rather than through [`report`](Self::report), which would demand a fabricated
    /// [`Receipt`] — inventing a tally and a risk-profile to read a disposition. Dies with the
    /// smoke-diff; nothing in the product reads it.
    ///
    /// The interner rides along because it MUST: a `Symbol` resolves only against the interner that
    /// minted it, and this world mints its own. Handing back the plan alone invites a caller to
    /// resolve its facts against some other interner, which indexes out of bounds if you are lucky
    /// and silently names the wrong entity if you are not.
    #[must_use]
    pub fn plan_ast_and_interner(&self) -> (&dorc_plan::Plan, &dorc_syntax::ast::Ast, &Interner) {
        (&self.plan, &self.ast, &self.interner)
    }

    /// The Spine this world's decisions live on — everything the plan is a projection OF.
    ///
    /// Exposed for the migration smoke-diff (`309` §4), which reads the decision plane DIRECTLY so
    /// that byte-identity against the frozen baseline proves the reification rather than proving one
    /// projection agrees with itself. Build-to-kill, like its consumer.
    #[must_use]
    pub const fn spine(&self) -> &dorc_plan::Spine {
        &self.spine
    }

    /// Borrow this world as the report context.
    #[must_use]
    pub fn report<'a>(&'a self, address: Option<&'a str>, receipt: &'a Receipt) -> WhyReport<'a> {
        WhyReport {
            address,
            plan: &self.plan,
            probe: &self.probe,
            first_wall: self.first_wall.as_ref(),
            wall_steps: &self.wall_steps,
            why_diags: &self.why_diags,
            refusals: &self.refusals,
            arena: &self.arena,
            ast: &self.ast,
            book_src: self.snapshot.book_src(),
            filename: self.snapshot.book_path(),
            interner: &self.interner,
            // THE DISCLOSED CUT (`churn-avoidance-disclosure`; `28P:res-why-world-lifts-no-book-
            // definitions`): the binary fills these SOURCE-wide, this seat ORACLE-only, and the
            // name/value mismatch IS the disclosure. It agrees today only because a book-sited
            // definition is invisible here, so it withholds where the binary answers — safe, and a
            // coincidence. Closing it means re-lifting this seat's world: a dispatch, not a rename.
            source_paths: self.snapshot.oracle_paths(),
            source_srcs: self.snapshot.oracle_srcs(),
            narrative: &self.narrative,
            cascades: &self.cascades,
            receipt,
        }
    }
}

/// The loaded-oracle index a threaded span belongs to (`law-lineno-identity`).
#[must_use]
pub fn source_file_id(idx: usize) -> dorc_core::SourceFileId {
    dorc_core::SourceFileId(u32::try_from(idx).unwrap_or(u32::MAX))
}

/// Latch the run-wide trip on the two PRE-NETWORK solve seats
/// (`302:rul-certifier-trip-guard-only`).
///
/// The license-plane twin of the cli's `solve_consistency_reports`, and deliberately not derived
/// from it: a policy that read the DIAGNOSTICS would be the narrative plane feeding a decision,
/// which `two-plane-aid-law` forbids in that direction. It also differs where it must — the report
/// seat suppresses the funcenv line when the failure is a value-plane CASCADE, because only
/// root-cause is reported (`271:rul-sin-ordering`), while the latch takes any real `Inconsistent`
/// it is handed and a cascade (`EnvFloor::ValuePlaneUntrusted`) is not one.
///
/// On the lib seam for the same reason [`definition_table`] is: both drivers must latch by ONE
/// rule, or the why report answers over a plan the run would not have emitted.
pub fn record_pre_network_trip(
    trip: &mut dorc_analysis::certify::CertifierTrip,
    value: &dorc_analysis::value::ValueFlow,
    env: &dorc_analysis::funcenv::FuncEnv,
) {
    use dorc_analysis::funcenv::EnvFloor;

    trip.record(value.consistency());
    if let Some(EnvFloor::SolverInconsistent(consistency)) = env.floor() {
        trip.record(consistency.as_ref());
    }
}

/// Run the terminal certifier-trip cleanup and mint its plan-prominent banner
/// (`302:rul-certifier-trip-guard-only`). A no-op — no walk, no banner — when nothing tripped.
///
/// THE CENSUS FORK, answered: a guard stands iff its verdict funcname has exactly one definition
/// in the loaded unit, which [`dorc_analysis::funcenv::DefinitionTable::occupancy`] answers by
/// counting. The table is the same one the environment was solved OVER, built by a syntactic walk
/// with no solve in it, so a trip — which disqualifies the solver and the certifier together —
/// cannot have corrupted the answer. That is what makes it admissible here, and it is the whole
/// argument: a lookup that itself depended on a solve would be no census at all.
///
/// The narrative is per-demoted-site and stays pull-tier; the banner is one line for the run.
pub fn demote_on_certifier_trip(
    spine: &mut dorc_plan::Spine,
    trip: dorc_analysis::certify::CertifierTrip,
    definitions: &dorc_analysis::funcenv::DefinitionTable,
) -> (Vec<Diag>, Vec<CollapseNarrative>) {
    use dorc_aid::diag::{DiagCode, SolverConsistencyPlanDemoted};

    if !trip.tripped() {
        return (Vec::new(), Vec::new());
    }
    let cleanup = dorc_plan::certifier_trip::demote_on_trip(spine, |fn_name| {
        definitions.occupancy(fn_name) == 1
    });
    let banner = Diag::new_spanless_site(DiagCode::SolverConsistencyPlanDemoted(
        SolverConsistencyPlanDemoted {
            demoted: cleanup.demoted().to_string(),
        },
    ));
    (vec![banner], cleanup.narrative().to_vec())
}

/// The unit's function definitions, as DATA for the function-environment domain (`28K` §2).
///
/// Read through `dorc_syntax::parse` for EVERY input, book and oracle alike, so the environment
/// sees exactly the funcdefs the sh parser sees.
///
/// EVERY top-level funcdef is recorded, role-named or not (`28Q` §1, human-typed intent: ONE
/// resolution mechanism, with oracle/book differences as POLICY and never as mechanism). The
/// retired table held role names alone, which meant the engine had two unrelated answers to "which
/// body does this name bind here" — a solved environment for roles, and last-declaration-wins over
/// the loaded set for helpers (`oracle/CLAUDE.md only-load-inert-sources-contribute` names that
/// second one as an interim that dies here). Sh has one answer, and
/// `rul-unsure-falls-toward-sh-parity` makes sh's the one to have.
///
/// POLICY still differs, and lives entirely at the consumers: role FAMILIES are what the shadow
/// refusal withholds (`28K` §1 — the cli maps a contest to a family through
/// `oracle::reserved::role_family`, so a helper collision reaches no withholding), and the
/// decidable-condition fold's `command -v` arm reads ROLE names only, because its whole warrant is
/// that a role name is never a binary (`dec-decidable-set-v0`; widening the table is exactly what
/// made that fence load-bearing rather than incidental).
///
/// Load order is the id order (`28K` §2a): CLI-named sources are the AMBIENT PREFIX, applied
/// "before line 1" in command-line order, and each is also registered under its own path so a
/// book's `. oracles/yum.sh` binds the same definitions. The book's own definitions are POSITIONAL
/// — keyed by the `FuncDef` AST node that writes them, since they execute in the book's stream.
///
/// Lives on the lib seam so the binary and [`WhyWorld`] build ONE table by one rule: a why report
/// that answered from a different environment than the run would be a decoration
/// (`lib-target-is-a-loom-seam`).
#[must_use]
pub fn definition_table(
    snapshot: &StaticLoadSnapshot,
    book: &dorc_syntax::Ast,
) -> dorc_analysis::funcenv::DefinitionTable {
    use dorc_analysis::funcenv::{Definition, DefinitionTable};
    use dorc_syntax::ast::NodeKind;

    let book_file = snapshot.book_file();
    let mut table = DefinitionTable::rooted_at(snapshot.cwd().clone());
    for (idx, path) in snapshot.oracle_paths().iter().enumerate() {
        let Some(src) = snapshot.oracle_srcs().get(idx) else {
            continue;
        };
        let parsed = dorc_syntax::parse(src).value;
        let mut by_ast = BTreeMap::new();
        let mut ids = Vec::new();
        for (id, node) in parsed.iter() {
            let NodeKind::FuncDef {
                name, name_span, ..
            } = &node.kind
            else {
                continue;
            };
            let def = table.add(Definition {
                file: source_file_id(idx),
                name: name.clone(),
                span: node.span,
                name_span: *name_span,
            });
            by_ast.insert(id, def);
            ids.push(def);
        }
        // A file that signed the dorc-lang contract carries a real load PROGRAM — its guards, its
        // own `.`s, its removals — because that is what its top level means (`30I` §3.1). One that
        // did not is registered flat, exactly as before: it makes no dialect claim, so reading
        // control flow into it would be inventing a promise its author never made.
        table.set_loadable(
            path,
            if crate::sourcing::satisfies_the_contract(src) {
                load_program(&parsed, &by_ast)
            } else {
                dorc_analysis::load::LoadProgram::of(
                    ids.iter().copied().map(LoadStep::Define).collect(),
                )
            },
        );
        // Only the invocation-named prefix loads "before line 1". Everything else binds at the `.`
        // that reaches it — a book's, or a named root's own, whose program the prefix run above
        // already evaluates. Making one ambient would license sites above its load point
        // (`visibility-is-full-positional`) AND replay its program (`30Mc:required-root-occurrence-identity`).
        if snapshot.is_ambient(idx) {
            table.push_ambient(path, ids);
        }
    }
    let mut book_assigns = Vec::new();
    for (id, node) in book.iter() {
        if let NodeKind::Assign { name, .. } = &node.kind {
            book_assigns.push(name.clone());
        }
        let NodeKind::FuncDef {
            name, name_span, ..
        } = &node.kind
        else {
            continue;
        };
        let def = table.add(Definition {
            file: book_file,
            name: name.clone(),
            span: node.span,
            name_span: *name_span,
        });
        table.set_book_site(id, def);
    }
    // Every name the book writes, anywhere, however it writes it — the outside-unit half of the
    // sentinel recognition's sole-populator question (`30I` §3.4). A whole-AST walk rather than a
    // top-level scan, because a book is ordinary flowing sh and an assignment inside a branch
    // populates the same variable.
    table.set_book_assigns(book_assigns);
    table
}

/// A contract-satisfying file's top level as the loader's closed program
/// (`dorc_analysis::load::LoadProgram`).
///
/// The step vocabulary and the admission gate are the SAME reading, taken from the same seat:
/// `dorc_oracle::load_inert` decides what a marked top level may hold, and this turns exactly
/// those shapes into steps. An item the gate would refuse cannot appear — the caller only reaches
/// here for a file that passed it — so an unrecognized shape is skipped rather than guessed at.
use dorc_syntax::ast::NodeKind;

fn load_program(
    ast: &dorc_syntax::Ast,
    by_ast: &BTreeMap<dorc_core::AstId, dorc_analysis::funcenv::DefId>,
) -> dorc_analysis::load::LoadProgram {
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return dorc_analysis::load::LoadProgram::default();
    };
    dorc_analysis::load::LoadProgram::of(load_steps(ast, by_ast, items))
}

fn load_steps(
    ast: &dorc_syntax::Ast,
    by_ast: &BTreeMap<dorc_core::AstId, dorc_analysis::funcenv::DefId>,
    items: &[dorc_core::AstId],
) -> Vec<LoadStep> {
    let mut steps = Vec::new();
    for &item in items {
        if let Some(&def) = by_ast.get(&item) {
            steps.push(LoadStep::Define(def));
            continue;
        }
        if let Some(control) = load_control(ast, item) {
            steps.push(LoadStep::Control(control));
            continue;
        }
        let NodeKind::Simple { assigns, .. } = &ast.node(item).kind else {
            continue;
        };
        for &assign in assigns {
            let NodeKind::Assign { name, value, .. } = &ast.node(assign).kind else {
                continue;
            };
            steps.push(LoadStep::Assign {
                name: name.clone(),
                value: value.map_or_else(LoadTarget::default, |word| load_target(ast, word)),
            });
        }
    }
    steps
}

/// One item as LOAD CONTROL — a `.`, an `unset -f`, or a guard over either — or `None` when it is
/// something else.
///
/// A guard's branches recurse HERE rather than through [`load_steps`], which is the type doing the
/// work: `LoadControl` has no declaring variant, so a definition cannot land in a branch even if
/// the admission gate were widened to admit one (`dorc_analysis::load::LoadControl` carries the
/// measured reason). A no-op branch item (`then :`) simply contributes nothing.
fn load_control(ast: &dorc_syntax::Ast, item: dorc_core::AstId) -> Option<LoadControl> {
    use dorc_oracle::load_inert::{include_guard, item_is_static_load, unset_functions};

    if let Some(word) = item_is_static_load(ast, item) {
        return Some(LoadControl::Load {
            target: load_target(ast, word),
            span: ast.node(item).span,
        });
    }
    if let Some(guard) = include_guard(ast, item) {
        let branch = |items: &[dorc_core::AstId]| {
            items
                .iter()
                .filter_map(|&nested| load_control(ast, nested))
                .collect()
        };
        return Some(LoadControl::Guard {
            condition: match guard.condition {
                dorc_oracle::load_inert::GuardCondition::CommandV { function } => {
                    dorc_analysis::load::LoadCondition::CommandV { function }
                }
                dorc_oracle::load_inert::GuardCondition::Value {
                    name,
                    literal,
                    equals,
                } => dorc_analysis::load::LoadCondition::Value {
                    name,
                    literal,
                    equals,
                },
            },
            negated: guard.negated,
            then_: branch(&guard.then_),
            else_: branch(&guard.else_),
        });
    }
    let NodeKind::Simple { words, .. } = &ast.node(item).kind else {
        return None;
    };
    unset_functions(ast, words).map(LoadControl::UnsetFunctions)
}

/// A word as a load operand: literal fragments kept, variable reads left for the loading context
/// to answer (`30I:force-root-value-flow`).
///
/// A fragment this seat cannot read — a command substitution, an operator expansion the lexer
/// collapsed — yields an EMPTY target, which expands to the empty string and resolves nowhere. The
/// admission gate already refuses those shapes, so this is the belt to its braces.
fn load_target(ast: &dorc_syntax::Ast, word: dorc_core::AstId) -> LoadTarget {
    use dorc_syntax::ast::WordPart;

    fn walk(parts: &[WordPart], out: &mut Vec<TargetPart>) -> bool {
        for part in parts {
            match part {
                WordPart::Literal(text) | WordPart::SingleQuoted(text) => {
                    out.push(TargetPart::Literal(text.clone()));
                }
                WordPart::Param { name, .. } => out.push(TargetPart::Param(name.clone())),
                WordPart::DoubleQuoted(inner) => {
                    if !walk(inner, out) {
                        return false;
                    }
                }
                WordPart::CommandSubst(_)
                | WordPart::Arithmetic
                | WordPart::ParamComplex { .. } => {
                    return false;
                }
            }
        }
        true
    }

    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return LoadTarget::default();
    };
    let mut out = Vec::new();
    if walk(parts, &mut out) {
        LoadTarget::of(out)
    } else {
        LoadTarget::default()
    }
}

/// The `(file, provider)` predict rows whose defining funcdef the environment proves binds at NO
/// program point ([`dorc_analysis::funcenv::never_live`]).
///
/// The ONE consumer is `dorc_oracle::build_dialect`'s whole-unit minting fold, reached through
/// `lift_from_sets`' `binds_somewhere`. Every SITE-KEYED consumer already declines such a row by
/// resolution — the frame names a definition and a dead one is named at no frame — so this exists
/// solely because the dialect asks a question no frame answers: which tokens the unit's authors
/// minted AT ALL. A dead polyfill body's tokens are not among them, and letting them in would
/// enlarge or shift the sparing dialect, which spares MORE (`28Q` §9 `pin-two-position-sparing`).
/// That is why "finishing" the never-live retirement by deleting this is WRONG: the withdrawal it
/// used to drive is gone, the liveness it computes is not.
///
/// Keyed by the PREDICT member specifically, not the family: the dialect mints from predict-derived
/// cells alone, and the family-wide reading the contest withdrawal uses would take a live sibling
/// member down with a dead one.
///
/// On the lib seam because both drivers must reach it (`one-definition-table-two-drivers`).
#[must_use]
pub fn never_live_predict_rows(
    never_live: &BTreeSet<(String, dorc_core::SourceFileId)>,
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
) -> BTreeSet<(usize, Symbol)> {
    let mut out = BTreeSet::new();
    for (file, set) in checks.iter().enumerate() {
        for provider in set.providers() {
            let name = format!(
                "{}{}",
                dorc_oracle::to_funcname_segment(interner.resolve(provider)),
                dorc_oracle::predict::PREDICT_SUFFIX
            );
            if never_live.contains(&(name, source_file_id(file))) {
                out.insert((file, provider));
            }
        }
    }
    out
}

/// The ONE index a site's role body ships from: the file whose definition of this role is the one
/// a shell would have live AT this site (`28Q` §1.3 — the frame lookup is the only resolution seat).
///
/// The whole-unit scan it replaces and the positional gate that narrowed the scan's answer are gone
/// together, because they were two readings of one environment and could disagree
/// (`28P:fnd-build-vouches-relifted-the-verdict-sets`). One question is asked once, and
/// [`dorc_core::answering_row`] holds the rule.
///
/// `declaration_at` answers the funcdef SPAN file `i` declares this role at, or `None` — presence
/// plus identity, never "does its body answer this argv". That distinction is the point: a scan for
/// the first file that RESOLVES falls through a declining live body into a shadowed one's arms,
/// which is exactly `28K` §6 rej-decline-fallthrough-cascade. A decline by the winner is a decline,
/// in the ship lane too.
#[must_use]
pub fn shipping_source(
    count: usize,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
    role_name: &str,
    declaration_at: impl Fn(usize) -> Option<dorc_core::Span>,
) -> Option<usize> {
    dorc_core::answering_row(live.definition_before(node, role_name), count, |i| {
        declaration_at(i).map(|span| dorc_analysis::funcenv::row_definition(i, span))
    })
}

/// The `<munged provider><suffix>` definition live at `node`, and the file that spells it.
///
/// One seat for every per-MEMBER resolution over a `PredictSet`-shaped vector — the probe ship's
/// `__predict`, and the wrapper lane's `__predict` / `__lend_map` / `__enter`. Each role member is
/// its own funcdef with its own frame answer, which is also what a shell does: names bind
/// independently, so the peel model, the lend map, and the entry form are three separate questions
/// asked at one site rather than one file's package deal.
///
/// The predicate is presence-only, deliberately: asking "does this file's body ANSWER this argv"
/// would resolve by first-that-succeeds, which is the retired decline-fallthrough cascade
/// (`28K` §6). A decline by the resolved definition is a decline.
#[must_use]
pub fn member_answering_at(
    sets: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    suffix: &str,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<(usize, dorc_oracle::predict::Predict)> {
    use dorc_oracle::predict::map_provider_name;
    let want = map_provider_name(interner.resolve(provider));
    let named = |set: &dorc_oracle::predict::PredictSet| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p).cloned())
    };
    let idx = shipping_source(sets.len(), node, live, &format!("{want}{suffix}"), |i| {
        sets.get(i).and_then(named).map(|p| p.span)
    })?;
    Some((idx, sets.get(idx).and_then(named)?))
}

/// The `<provider>__is_converged` definition live at `node`, and the file that spells it — the
/// [`member_answering_at`] twin for the verdict vector's own wrapper type.
///
/// Three acts consume ONE call of this per wrapped site (`308:rul-carry-proof-is-same-definition`):
/// the shipped inner check, the `safe-across` consent vouch, and pure-predicate carry's
/// read-set-closure proof. Resolving it once is what makes the proof and the measured body the same
/// definition by construction rather than by a checked coincidence.
#[must_use]
pub fn verdict_answering_at(
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &Interner,
    provider: Symbol,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<(usize, dorc_oracle::predict::Predict)> {
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::verdict::{VERDICT_SUFFIX, VerdictSet};
    let want = map_provider_name(interner.resolve(provider));
    let named = |set: &VerdictSet| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p).cloned())
    };
    let idx = shipping_source(
        verdict_sets.len(),
        node,
        live,
        &format!("{want}{VERDICT_SUFFIX}"),
        |i| verdict_sets.get(i).and_then(named).map(|p| p.span),
    )?;
    Some((idx, verdict_sets.get(idx).and_then(named)?))
}

/// R3 (23D §1 — the check IS the oracle): the stripped `<provider>__predict` a probe site ships,
/// preceded by its CLOSURE (`28K` §4 `rul-pin-by-definition-bytes`) — the helpers and file-level
/// constants the body needs, which do not travel with the funcdef span. A body whose closure the
/// loaded sources contest ships NOTHING (`None` ⇒ the site runs): the ambiguity resolves toward
/// run, and the load edge already named the collision.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "the shipped unit is now the definition PLUS its closure (`28K` §4), so the source \
              set, its non-role index, and the lifted checks all reach one seat by construction"
)]
pub fn ship_predict_body(
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::{PREDICT_SUFFIX, Resolution, evaluate, strip_predict};
    let (idx, check) = member_answering_at(checks, interner, provider, PREDICT_SUFFIX, node, live)?;
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    if !matches!(evaluate(&check, &arg_refs), Resolution::Resolved(_)) {
        return None;
    }
    let src = oracle_srcs.get(idx)?;
    let body = strip_predict(src, &check, interner);
    let closure = helpers.closure_for(idx, &body).ok()?;
    Some(dorc_plan::ShippedCheck::predict(
        format!("{}{body}", closure.sh()),
        Some((check.name_span, source_file_id(idx))),
    ))
}

/// `24L` §2 — the stripped `<provider>__is_converged` a typeless-floor auto-cell probe ships,
/// closure included on the same terms as [`ship_predict_body`]. Resolved through the same
/// [`shipping_source`] seat.
#[must_use]
pub fn ship_verdict_body(
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &Interner,
    provider: Symbol,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<dorc_plan::ShippedCheck> {
    let (idx, verdict) = verdict_answering_at(verdict_sets, interner, provider, node, live)?;
    ship_resolved_verdict(oracle_srcs, helpers, interner, idx, &verdict)
}

/// The emit half of [`ship_verdict_body`], over a definition the caller ALREADY resolved.
///
/// The wrapped lane holds its inner verdict resolved (`308:rul-carry-proof-is-same-definition` — one
/// definition feeds the shipped body, the entry tolerance, and the carry proof together), so it emits
/// through here rather than resolving a second time.
#[must_use]
pub fn ship_resolved_verdict(
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    interner: &Interner,
    idx: usize,
    verdict: &dorc_oracle::predict::Predict,
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::strip_verdict;
    let src = oracle_srcs.get(idx)?;
    let emits_report = dorc_oracle::report::emits_report(verdict);
    let body = strip_verdict(src, verdict, interner);
    let closure = helpers.closure_for(idx, &body).ok()?;
    Some(dorc_plan::ShippedCheck::verdict(
        format!("{}{body}", closure.sh()),
        Some((verdict.name_span, source_file_id(idx))),
        emits_report,
    ))
}

#[cfg(test)]
mod tests {
    use dorc_analysis::certify::{CertifierTrip, certify_solution};
    use dorc_analysis::lattice::Flat;
    use dorc_analysis::solve::{Direction, Graph, Solution};
    use dorc_core::{
        AstId, ByVouch, EntityRef, FactKey, Interner, KindId, LeafId, OpaqueToken, Rung,
        SelectorId, SourceFileId, Verdict,
    };
    use dorc_plan::{Disposition, GuardLicense, Plan, Step, SurvivalReport, VerdictVouch};

    use super::{StaticLoadSnapshot, definition_table, demote_on_certifier_trip};

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

    /// A latch driven by a GENUINE perturbation judged by the GENUINE checker (`302` §6.1/§6.7):
    /// the claimed solution says ⊥ while the transfer really produces `Elem(1)`, so the per-edge
    /// inequality fails for real. `raise` picks whether the fixture perturbs at all, so the
    /// control below is this same fixture with the defect taken out rather than a different one.
    fn latch_from_a_real_certification(raise: bool) -> CertifierTrip {
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
            |_, incoming: &Flat<u8>| {
                if raise {
                    Flat::Elem(1u8)
                } else {
                    incoming.clone()
                }
            },
            &solution,
        );
        assert_eq!(
            outcome.is_consistent(),
            !raise,
            "the fixture must really do what the case name says"
        );
        let mut trip = CertifierTrip::default();
        trip.record(&outcome);
        trip
    }

    /// The cleanup reaches its decisions through the Spine now, so the fixture writes one there and
    /// every assertion below reads the projection — the same path a real run takes.
    fn guarded_spine(fn_name: &str) -> dorc_plan::Spine {
        let plan = guarded_plan(fn_name);
        let mut spine = dorc_plan::Spine::new();
        for step in plan.steps {
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

    fn projected(spine: &dorc_plan::Spine) -> Plan {
        dorc_plan::project_plan(spine, &dorc_plan::PlanAuthority::without_intake())
    }

    fn guarded_plan(fn_name: &str) -> Plan {
        let mut i = Interner::default();
        let fact = FactKey::cell(
            KindId(i.intern("package")),
            EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            SelectorId(i.intern("installed")),
        );
        let vouch = ByVouch::vouched(
            VerdictVouch::new(
                fn_name.to_string(),
                format!("{fn_name}() {{ return 0; }}"),
                format!("{fn_name} install -y nginx"),
                "package".to_string(),
                Vec::new(),
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            ),
            Rung::Both,
        );
        Plan {
            steps: vec![Step {
                leaf: LeafId(0),
                ast: AstId(0),
                sh: "apt-get install -y nginx".to_string(),
                disposition: Disposition::Guard(
                    GuardLicense::mint(
                        fact,
                        vouch,
                        Verdict::Converged,
                        &dorc_analysis::lattice::May(dorc_analysis::lattice::Powerset::default()),
                    )
                    .expect("a converged probe verdict mints a guard"),
                ),
            }],
            regions: Vec::new(),
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        }
    }

    const BOOK: &str = "apt-get install -y nginx
";

    /// Build the REAL census input the seat reads: a definition table over parsed sources.
    fn table_over(oracles: &[&str]) -> dorc_analysis::funcenv::DefinitionTable {
        let paths: Vec<String> = (0..oracles.len()).map(|n| format!("o{n}.sh")).collect();
        let book = dorc_syntax::parse(BOOK).value;
        let snapshot = StaticLoadSnapshot::over(
            dorc_core::loadpath::Cwd::default(),
            paths,
            oracles.iter().map(|s| (*s).to_owned()).collect(),
            &crate::snapshot::LoadPositions::roots_only(),
            "book.sh",
            BOOK,
        );
        definition_table(&snapshot, &book)
    }

    const ONE_DECLARATION: &str = "apt_get__is_converged() { return 0; }\n";
    const ANOTHER_DECLARATION: &str = "apt_get__is_converged() { return 1; }\n";

    /// THE CENSUS FORK, over the real lookup. One oracle declaring the verdict family ⇒ occupancy
    /// 1 ⇒ the guard stands, because no analysis ever chose which body its name resolves to. Two
    /// oracles declaring it ⇒ the choice was analysis's, the trip disqualified the analysis, and
    /// the guard goes with it.
    #[test]
    fn the_body_occupancy_census_decides_whether_a_guard_stands() {
        let mut sole = guarded_spine("apt_get__is_converged");
        demote_on_certifier_trip(
            &mut sole,
            latch_from_a_real_certification(true),
            &table_over(&[ONE_DECLARATION]),
        );
        assert!(
            matches!(projected(&sole).steps[0].disposition, Disposition::Guard(_)),
            "a census-unique family keeps its runtime net"
        );

        let mut plural = guarded_spine("apt_get__is_converged");
        demote_on_certifier_trip(
            &mut plural,
            latch_from_a_real_certification(true),
            &table_over(&[ONE_DECLARATION, ANOTHER_DECLARATION]),
        );
        assert!(
            matches!(projected(&plural).steps[0].disposition, Disposition::Run),
            "a plural family's guard could run somebody else's judgment — it demotes"
        );
    }

    /// The BANNER's structure (`302` §5): one plan-prominent line per tripped run, spanless,
    /// carrying the demoted count. Its prose is deliberately unwritten — the structure is the
    /// builder's, the words are not (`error-authorship-tier`).
    #[test]
    fn a_trip_mints_one_spanless_banner_carrying_the_demoted_count() {
        let mut plan = guarded_spine("apt_get__is_converged");

        let (diags, narrative) = demote_on_certifier_trip(
            &mut plan,
            latch_from_a_real_certification(true),
            &table_over(&[ONE_DECLARATION, ANOTHER_DECLARATION]),
        );

        assert_eq!(diags.len(), 1, "ONE banner for the run, not one per pass");
        assert_eq!(diags[0].code.slug(), "solver-consistency-plan-demoted");
        assert!(
            diags[0].primary.span().is_none(),
            "spanless: a caret on a book line would blame the admin for our defect"
        );
        assert!(
            matches!(
                &diags[0].code,
                dorc_aid::diag::DiagCode::SolverConsistencyPlanDemoted(p) if p.demoted == "1"
            ),
            "the count is measured from the walk, never announced ahead of it"
        );
        assert_eq!(
            narrative.len(),
            1,
            "and one pull-tier demotion record beside it"
        );
    }

    /// THE SEAT CONTROL. A run whose certification really passed reaches no walk at all: the plan
    /// keeps every disposition it earned and no banner is minted. Same fixture, defect removed.
    #[test]
    fn an_untripped_run_is_left_entirely_alone() {
        let mut plan = guarded_spine("apt_get__is_converged");

        let (diags, narrative) = demote_on_certifier_trip(
            &mut plan,
            latch_from_a_real_certification(false),
            &table_over(&[ONE_DECLARATION, ANOTHER_DECLARATION]),
        );

        assert!(diags.is_empty(), "no trip, no banner");
        assert!(narrative.is_empty());
        assert!(
            matches!(projected(&plan).steps[0].disposition, Disposition::Guard(_)),
            "the plural census demotes NOTHING without a trip — the trip is the whole trigger"
        );
    }
}
