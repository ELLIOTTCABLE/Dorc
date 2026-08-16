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
use dorc_core::{Interner, Observable, ProvArena, Symbol, Verdict};

use crate::Receipt;
use crate::results::{SiteResults, probe_origins};
use crate::why::{
    CascadeAttribution, FirstWallHint, WallStep, WhyReport, collect_wall_steps, first_wall_hint,
};

/// Everything a why report reads, owned in one place so a caller can borrow a [`WhyReport`] out of
/// it without threading seventeen lifetimes of its own.
pub struct WhyWorld {
    filename: String,
    book_src: String,
    oracle_paths: Vec<String>,
    oracle_srcs: Vec<String>,
    interner: Interner,
    arena: ProvArena,
    ast: dorc_syntax::ast::Ast,
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
            .field("filename", &self.filename)
            .finish_non_exhaustive()
    }
}

impl WhyWorld {
    /// Analyze `book_src` against `oracle_srcs` with no measurements — every fact ⊤, every site runs.
    #[must_use]
    pub fn analyze(
        filename: &str,
        book_src: &str,
        oracle_paths: &[String],
        oracle_srcs: &[String],
    ) -> Self {
        Self::analyze_measured(
            filename,
            book_src,
            oracle_paths,
            oracle_srcs,
            &SiteResults::default(),
            false,
        )
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
        filename: &str,
        book_src: &str,
        oracle_paths: &[String],
        oracle_srcs: &[String],
        results: &SiteResults,
        consented: bool,
    ) -> Self {
        let mut interner = Interner::default();
        let mut arena = ProvArena::new();
        let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();

        // One non-role-declaration index per unit, shared by the ship seams and the vouch lift.
        let helpers = dorc_oracle::closure::HelperIndex::build(&oracle_refs);
        let idx = dorc_oracle::lift(&mut interner, &oracle_refs).value;
        let checks: Vec<dorc_oracle::predict::PredictSet> = oracle_refs
            .iter()
            .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
            .collect();
        let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = oracle_refs
            .iter()
            .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
            .collect();
        let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);

        let parsed = dorc_syntax::parse(book_src);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);

        let mut degrades = BTreeMap::new();
        let mut verdict_lane = BTreeSet::new();
        let peeled = BTreeMap::new();
        // The book's id sits ONE PAST the vector this seat lifts (it does not feed the book to the
        // lifts — `28M` §7's rename rider names the gap), so a book-owned site withholds.
        let definitions = definition_table(
            oracle_paths,
            &oracle_refs,
            source_file_id(oracle_srcs.len()),
            &parsed.value,
        );
        let env = {
            let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
            dorc_analysis::funcenv::analyze(&parsed.value, &cfg.value, &definitions, &plane)
        };
        let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &definitions);
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

        let (vouch_lift, decline_narrative) = dorc_plan::build_vouches(
            &oracle_refs,
            &helpers,
            &classes,
            &value,
            &mut interner,
            live,
        );
        let vouches = vouch_lift.value;

        let ship = |node: dorc_analysis::cfg::CfgNodeId, provider: Symbol, argv: &[Symbol]| {
            ship_predict_body(
                oracle_srcs,
                &helpers,
                &checks,
                &interner,
                provider,
                argv,
                node,
                live,
            )
        };
        let ship_auto = |node: dorc_analysis::cfg::CfgNodeId, provider: Symbol, _: &[Symbol]| {
            verdict_lane
                .contains(&node)
                .then(|| {
                    ship_verdict_body(
                        oracle_srcs,
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
            |node| vouches.contains_site(node),
        )
        .with_unresolvable_causes(&parsed.value, &cfg.value, &classes, &degrades);

        // The validity fixpoint, to quiescence, over the frozen origin — the binary's own rounds
        // (`the-fixpoint-owns-the-rounds-and-builds-nothing-else`). Its product beyond the settled
        // fold is the round-tagged cascade attribution, which is the only way a why report can
        // answer for an elision that only became legal once something upstream was proven dead.
        let cap = u32::try_from(origin.classes.len())
            .unwrap_or(u32::MAX)
            .max(1);
        let settled = crate::fixpoint::settle_validity_fixpoint(
            &frozen,
            &probe,
            results,
            origin,
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
            &settled.origin_validity,
        );
        let round = settled.round;
        let classes = round.classes;
        let (kills, kill_coords, fact_backings) =
            (round.kills, round.kill_coords, round.fact_backings);
        let (why_diags, classify_narrative) = (round.why_diags, round.classify_narrative);
        let (by_fact, merge_narrative) = (settled.by_fact, settled.merge_narrative);
        let probe_attributions = probe_origins(&probe, results, &mut arena);

        // The survival tier, flag-gated exactly as a run is (`rul24-mode-gate`, TC-1): unflagged,
        // the footprint data does not exist at all, so a running mutator walls totally.
        let touches_paired: Vec<(&str, dorc_oracle::touches::TouchesSet)> = oracle_refs
            .iter()
            .map(|src| {
                (
                    *src,
                    dorc_oracle::touches::TouchesSet::lift(&mut interner, src).value,
                )
            })
            .collect();
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        let coord_kinds = crate::survival::collect_coord_kinds(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &mut interner,
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
                let derive = |p, a: &[Symbol]| {
                    crate::survival::ship_touches_body(&touches_paired, &interner, p, a)
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
        let verdict_names: Vec<String> = verdicts
            .providers()
            .map(|p| interner.resolve(p.0).to_owned())
            .collect();
        for name in verdict_names {
            let kind = dorc_core::auto_fact(&mut interner, &name).kind;
            resolutions.add_auto_kind(kind);
        }

        let mut plan = dorc_plan::build_plan_walled(
            book_src,
            &parsed.value,
            &cfg.value,
            &classes,
            &kills,
            survival.as_ref(),
            consented.then_some(&resolutions),
            &dorc_oracle::build_dialect(&idx),
            &fact_backings,
            &vouches,
            &dorc_plan::ConnectedPipes::default(),
            &probe_attributions,
            |f| {
                by_fact
                    .get(&f)
                    .copied()
                    .unwrap_or(Observable::verdict_only(Verdict::Unknown))
            },
            &mut arena,
        );
        let (_trip_banner, trip_narrative) =
            demote_on_certifier_trip(&mut plan, trip, &definitions);
        let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
        let narrative: Vec<CollapseNarrative> = classify_narrative
            .into_iter()
            .chain(decline_narrative)
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
            filename: filename.to_owned(),
            book_src: book_src.to_owned(),
            oracle_paths: oracle_paths.to_vec(),
            oracle_srcs: oracle_srcs.to_vec(),
            interner,
            arena,
            ast: parsed.value,
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
            &self.book_src,
            &self.ast,
            &self.interner,
            &identity,
        )
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
            book_src: &self.book_src,
            filename: &self.filename,
            interner: &self.interner,
            // THE DISCLOSED CUT (`churn-avoidance-disclosure`; `28P:res-why-world-lifts-no-book-
            // definitions`): the binary fills these SOURCE-wide, this seat ORACLE-only, and the
            // name/value mismatch IS the disclosure. It agrees today only because a book-sited
            // definition is invisible here, so it withholds where the binary answers — safe, and a
            // coincidence. Closing it means re-lifting this seat's world: a dispatch, not a rename.
            source_paths: &self.oracle_paths,
            source_srcs: &self.oracle_srcs,
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
    plan: &mut dorc_plan::Plan,
    trip: dorc_analysis::certify::CertifierTrip,
    definitions: &dorc_analysis::funcenv::DefinitionTable,
) -> (Vec<Diag>, Vec<CollapseNarrative>) {
    use dorc_aid::diag::{DiagCode, SolverConsistencyPlanDemoted};

    if !trip.tripped() {
        return (Vec::new(), Vec::new());
    }
    let cleanup = dorc_plan::certifier_trip::demote_on_trip(plan, |fn_name| {
        definitions.occupancy(fn_name) == 1
    });
    let banner = Diag::new_spanless_site(DiagCode::SolverConsistencyPlanDemoted(
        SolverConsistencyPlanDemoted {
            demoted: cleanup.demoted().to_string(),
        },
    ));
    (vec![banner], cleanup.narrative().to_vec())
}

/// The unit's role definitions, as DATA for the function-environment domain (`28K` §2).
///
/// Read through `dorc_syntax::parse` for EVERY input, book and oracle alike, so the environment and
/// the shadow refusal see exactly the funcdefs the sh parser sees. Only ROLE names are recorded:
/// the refusal is about role FAMILIES (`28K` §1), and an ordinary helper colliding across files
/// carries no license to withhold.
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
    oracle_paths: &[String],
    source_srcs: &[&str],
    book_file: dorc_core::SourceFileId,
    book: &dorc_syntax::Ast,
) -> dorc_analysis::funcenv::DefinitionTable {
    use dorc_analysis::funcenv::{Definition, DefinitionTable};
    use dorc_syntax::ast::NodeKind;

    let mut table = DefinitionTable::default();
    for (idx, path) in oracle_paths.iter().enumerate() {
        let Some(src) = source_srcs.get(idx) else {
            continue;
        };
        let parsed = dorc_syntax::parse(src).value;
        let mut ids = Vec::new();
        for (_, node) in parsed.iter() {
            let NodeKind::FuncDef {
                name, name_span, ..
            } = &node.kind
            else {
                continue;
            };
            if dorc_oracle::reserved::role_family(name).is_none() {
                continue;
            }
            ids.push(table.add(Definition {
                file: source_file_id(idx),
                name: name.clone(),
                span: node.span,
                name_span: *name_span,
            }));
        }
        table.set_loadable(path.clone(), ids.clone());
        table.extend_ambient(ids);
    }
    for (id, node) in book.iter() {
        let NodeKind::FuncDef {
            name, name_span, ..
        } = &node.kind
        else {
            continue;
        };
        if dorc_oracle::reserved::role_family(name).is_none() {
            continue;
        }
        let def = table.add(Definition {
            file: book_file,
            name: name.clone(),
            span: node.span,
            name_span: *name_span,
        });
        table.set_book_site(id, def);
    }
    table
}

/// The ONE index a site's role body ships from: sh's live definition ([`dorc_oracle::live_source`],
/// the single seat), narrowed to the one live AT this site (`28K` §2
/// rul-visibility-is-full-positional).
///
/// `has` asks only "does file `i` define this role for this provider" — never "does its body
/// answer this argv". That distinction is the point: a backwards scan for the first file that
/// RESOLVES falls through a declining live body into a shadowed one's arms, which is exactly
/// `28K` §6 rej-decline-fallthrough-cascade, and `analysis::effect` retired it at stage D. A
/// decline by the winner is a decline, in the ship lane too.
fn shipping_source(
    count: usize,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
    role_name: &str,
    has: impl Fn(usize) -> bool,
) -> Option<usize> {
    dorc_oracle::live_source(count, has).filter(|&i| live.answers_at(node, role_name, i))
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
    use dorc_oracle::predict::{
        PREDICT_SUFFIX, Resolution, evaluate, map_provider_name, strip_predict,
    };
    let want = map_provider_name(interner.resolve(provider));
    let named = |cs: &dorc_oracle::predict::PredictSet| {
        cs.providers()
            .find(|cp| map_provider_name(interner.resolve(*cp)) == want)
            .and_then(|cp| cs.get(cp).cloned())
    };
    let idx = shipping_source(
        checks.len(),
        node,
        live,
        &format!("{want}{PREDICT_SUFFIX}"),
        |i| checks.get(i).and_then(named).is_some(),
    )?;
    let check = checks.get(idx).and_then(named)?;
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
        format!("{}{body}", closure.sh),
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
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    use dorc_oracle::verdict::{VERDICT_SUFFIX, VerdictSet};
    let want = map_provider_name(interner.resolve(provider));
    let named = |set: &VerdictSet| {
        set.providers()
            .find(|vp| map_provider_name(interner.resolve(*vp)) == want)
            .and_then(|vp| set.get(vp).cloned())
    };
    let idx = shipping_source(
        verdict_sets.len(),
        node,
        live,
        &format!("{want}{VERDICT_SUFFIX}"),
        |i| verdict_sets.get(i).and_then(named).is_some(),
    )?;
    let verdict = verdict_sets.get(idx).and_then(named)?;
    let src = oracle_srcs.get(idx)?;
    let emits_report = dorc_oracle::report::emits_report(&verdict);
    let body = strip_verdict(src, &verdict, interner);
    let closure = helpers.closure_for(idx, &body).ok()?;
    Some(dorc_plan::ShippedCheck::verdict(
        format!("{}{body}", closure.sh),
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

    use super::{definition_table, demote_on_certifier_trip, source_file_id};

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
                    GuardLicense::mint(fact, vouch, Verdict::Converged)
                        .expect("a converged probe verdict mints a guard"),
                ),
            }],
            survival_report: SurvivalReport::default(),
        }
    }

    /// Build the REAL census input the seat reads: a definition table over parsed sources.
    fn table_over(oracles: &[&str]) -> dorc_analysis::funcenv::DefinitionTable {
        let paths: Vec<String> = (0..oracles.len()).map(|n| format!("o{n}.sh")).collect();
        let book = dorc_syntax::parse("apt-get install -y nginx\n").value;
        definition_table(&paths, oracles, source_file_id(oracles.len()), &book)
    }

    const ONE_DECLARATION: &str = "apt_get__is_converged() { return 0; }\n";
    const ANOTHER_DECLARATION: &str = "apt_get__is_converged() { return 1; }\n";

    /// THE CENSUS FORK, over the real lookup. One oracle declaring the verdict family ⇒ occupancy
    /// 1 ⇒ the guard stands, because no analysis ever chose which body its name resolves to. Two
    /// oracles declaring it ⇒ the choice was analysis's, the trip disqualified the analysis, and
    /// the guard goes with it.
    #[test]
    fn the_body_occupancy_census_decides_whether_a_guard_stands() {
        let mut sole = guarded_plan("apt_get__is_converged");
        demote_on_certifier_trip(
            &mut sole,
            latch_from_a_real_certification(true),
            &table_over(&[ONE_DECLARATION]),
        );
        assert!(
            matches!(sole.steps[0].disposition, Disposition::Guard(_)),
            "a census-unique family keeps its runtime net"
        );

        let mut plural = guarded_plan("apt_get__is_converged");
        demote_on_certifier_trip(
            &mut plural,
            latch_from_a_real_certification(true),
            &table_over(&[ONE_DECLARATION, ANOTHER_DECLARATION]),
        );
        assert!(
            matches!(plural.steps[0].disposition, Disposition::Run),
            "a plural family's guard could run somebody else's judgment — it demotes"
        );
    }

    /// The BANNER's structure (`302` §5): one plan-prominent line per tripped run, spanless,
    /// carrying the demoted count. Its prose is deliberately unwritten — the structure is the
    /// builder's, the words are not (`error-authorship-tier`).
    #[test]
    fn a_trip_mints_one_spanless_banner_carrying_the_demoted_count() {
        let mut plan = guarded_plan("apt_get__is_converged");

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
        let mut plan = guarded_plan("apt_get__is_converged");

        let (diags, narrative) = demote_on_certifier_trip(
            &mut plan,
            latch_from_a_real_certification(false),
            &table_over(&[ONE_DECLARATION, ANOTHER_DECLARATION]),
        );

        assert!(diags.is_empty(), "no trip, no banner");
        assert!(narrative.is_empty());
        assert!(
            matches!(plan.steps[0].disposition, Disposition::Guard(_)),
            "the plural census demotes NOTHING without a trip — the trip is the whole trigger"
        );
    }
}
