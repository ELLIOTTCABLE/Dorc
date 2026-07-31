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
        let frozen = crate::fixpoint::FrozenModel {
            cfg: &cfg.value,
            value: &value,
            ast: &parsed.value,
            idx: &idx,
            checks: &checks,
            verdicts: &verdicts,
            peeled: &peeled,
        };
        let origin = crate::fixpoint::classify_round(
            &frozen,
            &dorc_analysis::erase::ErasedSites::none(),
            &mut interner,
            &mut arena,
            &mut degrades,
            &mut verdict_lane,
        );
        let classes = origin.classes.clone();

        let (vouch_lift, decline_narrative) =
            dorc_plan::build_vouches(&oracle_refs, &classes, &value, &mut interner);
        let vouches = vouch_lift.value;

        let ship = |provider: Symbol, argv: &[Symbol]| {
            ship_predict_body(oracle_srcs, &checks, &interner, provider, argv)
        };
        let ship_auto = |node: dorc_analysis::cfg::CfgNodeId, provider: Symbol, _: &[Symbol]| {
            verdict_lane
                .contains(&node)
                .then(|| ship_verdict_body(oracle_srcs, &verdict_sets, &interner, provider))
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
            crate::survival::expand_footprints_via_reaches(
                &mut footprints,
                &kind_reaches,
                &reach_kinds,
                results,
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

        let plan = dorc_plan::build_plan_walled(
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
        let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
        let narrative: Vec<CollapseNarrative> = classify_narrative
            .into_iter()
            .chain(decline_narrative)
            .chain(merge_narrative)
            .chain(plan.survival_report.collapse_narrative().iter().cloned())
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
            oracle_paths: &self.oracle_paths,
            oracle_srcs: &self.oracle_srcs,
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

/// R3 (23D §1 — the check IS the oracle): the stripped `<provider>__predict` a probe site ships.
#[must_use]
pub fn ship_predict_body(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::{Resolution, evaluate, map_provider_name, strip_predict};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    for (idx, (src, cs)) in oracle_srcs.iter().zip(checks).enumerate().rev() {
        for cp in cs.providers() {
            if map_provider_name(interner.resolve(cp)) != want {
                continue;
            }
            let Some(check) = cs.get(cp) else { continue };
            if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                return Some(dorc_plan::ShippedCheck::predict(
                    strip_predict(src, check, interner),
                    Some((check.name_span, source_file_id(idx))),
                ));
            }
        }
    }
    None
}

/// `24L` §2 — the stripped `<provider>__is_converged` a typeless-floor auto-cell probe ships.
#[must_use]
pub fn ship_verdict_body(
    oracle_srcs: &[String],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &Interner,
    provider: Symbol,
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    let want = map_provider_name(interner.resolve(provider));
    for (idx, (src, set)) in oracle_srcs.iter().zip(verdict_sets).enumerate().rev() {
        for vp in set.providers() {
            if map_provider_name(interner.resolve(vp)) != want {
                continue;
            }
            let Some(verdict) = set.get(vp) else { continue };
            let emits_report = dorc_oracle::report::emits_report(verdict);
            return Some(dorc_plan::ShippedCheck::verdict(
                strip_verdict(src, verdict, interner),
                Some((verdict.name_span, source_file_id(idx))),
                emits_report,
            ));
        }
    }
    None
}
