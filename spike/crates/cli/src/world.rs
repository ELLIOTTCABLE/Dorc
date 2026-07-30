//! The analyzed world a `dorc why` report is ABOUT, assembled from source alone.
//!
//! This is the harness half of the loom seam (`lib-target-is-a-loom-seam`): the binary builds its
//! world out of files, flags, a clock and a host's records, and every one of those is a QUERY that
//! stays on its side. What a loom case can hand across is SOURCE — a book, its oracles — so this
//! seat takes exactly that and runs the same kernel calls in the same order, which is what makes a
//! committed why transcript an honest render rather than a decoration
//! (`289:rul-worldless-route-honest-trigger`).
//!
//! RESIDUAL SCOPE CUT, stated where it bites (`churn-avoidance-disclosure`): this world carries NO
//! probe records, so every fact is ⊤ and every site RUNS. Feeding records is not a missing
//! parameter — host evidence is admitted under controller-minted attribution
//! (`rul-attribution-is-controller-minted`), and those scope types are deliberately private to the
//! binary. A world with measured facts therefore stays the binary's, and the chain families that
//! only a measured fact reaches (survival, guard) are unreachable from here BY CONSTRUCTION.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::CollapseNarrative;
use dorc_aid::diag::Diag;
use dorc_core::{Interner, Observable, ProvArena, Symbol, Verdict};

use crate::Receipt;
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
    /// Analyze `book_src` against `oracle_srcs` and build the plan a why report explains.
    ///
    /// The same call sequence the binary runs — lift, parse, CFG, value-flow, classify, vouch,
    /// compile the probe, build the plan — minus the record-fed fold. Stage diagnostics are
    /// DROPPED rather than printed: this seat has no stderr, and a case that wants a diagnostic
    /// rendered drives the diagnostic route instead.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "one linear pipeline in the binary's own order; splitting it would let the two orders drift, which is the whole thing this seat exists to prevent"
    )]
    pub fn analyze(
        filename: &str,
        book_src: &str,
        oracle_paths: &[String],
        oracle_srcs: &[String],
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
        let (classified, why_diags, _kills, _kill_coords, _backings, classify_narrative, _inval) =
            dorc_analysis::effect::classify_with_why_diags(
                &cfg.value,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &verdicts,
                &BTreeMap::new(),
                &dorc_analysis::erase::ErasedSites::none(),
                &mut interner,
                &mut arena,
                &mut degrades,
                &mut verdict_lane,
            );
        let classes = classified.value;

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

        let plan = dorc_plan::build_plan(
            book_src,
            &parsed.value,
            &cfg.value,
            &classes,
            &vouches,
            |_| Observable::verdict_only(Verdict::Unknown),
            &mut arena,
        );
        let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
        let narrative: Vec<CollapseNarrative> = classify_narrative
            .into_iter()
            .chain(decline_narrative)
            .chain(plan.survival_report.collapse_narrative().iter().cloned())
            .chain(plan.render_refusal_narratives(&parsed.value))
            .collect();
        let wall_steps = collect_wall_steps(
            &plan,
            &probe,
            &classes,
            &cfg.value,
            &BTreeSet::new(),
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
            // No fixpoint runs here: with nothing measured, no round-2 validity flip exists to
            // attribute, so an empty map is the honest answer rather than a missing one.
            cascades: BTreeMap::new(),
        }
    }

    /// What this world decided, for the receipt's tally row.
    #[must_use]
    pub fn disposition_counts(&self) -> dorc_plan::DispositionCounts {
        self.plan.disposition_counts()
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
fn oracle_file_id(idx: usize) -> dorc_core::OracleFileId {
    dorc_core::OracleFileId(u32::try_from(idx).unwrap_or(u32::MAX))
}

/// R3 (23D §1 — the check IS the oracle): the stripped `<provider>__predict` a probe site ships.
fn ship_predict_body(
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
    for (idx, (src, cs)) in oracle_srcs.iter().zip(checks).enumerate() {
        for cp in cs.providers() {
            if map_provider_name(interner.resolve(cp)) != want {
                continue;
            }
            let Some(check) = cs.get(cp) else { continue };
            if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                return Some(dorc_plan::ShippedCheck::predict(
                    strip_predict(src, check, interner),
                    Some((check.name_span, oracle_file_id(idx))),
                ));
            }
        }
    }
    None
}

/// `24L` §2 — the stripped `<provider>__is_converged` a typeless-floor auto-cell probe ships.
fn ship_verdict_body(
    oracle_srcs: &[String],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &Interner,
    provider: Symbol,
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    let want = map_provider_name(interner.resolve(provider));
    for (idx, (src, set)) in oracle_srcs.iter().zip(verdict_sets).enumerate() {
        for vp in set.providers() {
            if map_provider_name(interner.resolve(vp)) != want {
                continue;
            }
            let Some(verdict) = set.get(vp) else { continue };
            let emits_report = dorc_oracle::report::emits_report(verdict);
            return Some(dorc_plan::ShippedCheck::verdict(
                strip_verdict(src, verdict, interner),
                Some((verdict.name_span, oracle_file_id(idx))),
                emits_report,
            ));
        }
    }
    None
}
