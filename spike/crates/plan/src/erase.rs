//! The **erasure ledger** — the validity fixpoint's sole cross-round accumulator, and the
//! one transform where W-C can be wrong (`26H` §4/§4¾).
//!
//! `26G:fnd-dead-branch-still-invalidates`: "a command that will not run cannot invalidate
//! anything" (`USER_STORY` st.3) is implemented twice — by the wall predicate, which honours
//! it, and by the query-validity bit, which does not. Validity is
//! `reach.states[i].is_pristine()`, computed statically in a records-blind crate, so a
//! mutator PROVEN DEAD by the records still invalidates every guard below it and the ladder
//! caps at one rung. The fix is a bounded, same-records, deterministic fixpoint: erase a
//! proven-dead site's invalidator-hood from the analyzer model, re-derive, repeat.
//!
//! This module owns the "why" half of the overlay model. [`dorc_analysis::erase`] owns the
//! model half; the two meet at an [`ErasureLicense`], which carries a node id and nothing
//! else — `analysis` never learns why a site shrank.
//!
//! # The chain, and where each link is sealed
//!
//! `FoldResult` → [`DeadBranchProof`] → `world::NoExecutionLedger` → [`ErasureLicense`] →
//! overlay.
//!
//! The first link is type-sealed against the whole workspace: [`DeadBranchProof`] has private
//! fields and its sole mint is [`prove_dead_branches`], which folds internally, and
//! `FoldResult`'s `dead` map is private with no public inserter while `fold::fold` is
//! `pub(crate)` — so no crate outside `plan` can manufacture a proof, and the ledger demands
//! one BY VALUE.
//!
//! The last link is the acked weak seam: `analysis` cannot depend on `plan`, so
//! `ErasureLicense::for_site` must be public and the type system cannot prove every licence
//! traces to a proof. `licence_mint_has_exactly_one_caller` is the fence — a lexical census
//! that fails if any second caller appears anywhere in the workspace.
//!
//! # Why the predicate is stricter than "the fold said dead"
//!
//! A fold-`Omit`ted leaf whose controller is NOT neutralised renders VERBATIM and runs at
//! apply time, gated by the live guard (`is_neutralised`, and the committed
//! `omitsafe21-heredoc-guard-keeps-body` case). Erasing such a site would license downstream
//! elisions off a mutator the artifact still runs — a wrong yes. So the proof additionally
//! demands that the controller really will be substituted away, computed from analysis data
//! and the fold input alone. It never reads a [`Disposition`](crate::Disposition):
//! omitted-for-any-other-reason is not dead, and an outcome must never become a premise
//! (`pin-no-outcome-as-generator`).

use std::collections::{BTreeMap, BTreeSet};

use dorc_analysis::cfg::{Cfg, CfgNodeId};
use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_analysis::erase::ErasureLicense;
use dorc_analysis::lattice::May;
use dorc_core::{AstId, Observable, Predicted, Rc};
use dorc_syntax::ast::Ast;

use crate::fold::AbstractRc;
use crate::{
    has_top_successor, leaf_facts, leaf_has_heredoc, query_substitutes, subtree_leaves_all,
};

/// Which round of the validity fixpoint minted something. Round 1 runs against the origin
/// model; a round-2+ erasure or validity flip is the cascade the why-chain renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoundId(pub u32);

/// A records-proven-dead derivation for ONE site: the fold, driven by a measured, valid,
/// non-conflicted query rc, proved this site's branch cannot be reached.
///
/// Private fields, and [`prove_dead_branches`] is the sole mint. "Records-proven" is load
/// bearing and narrow: a statically-known controlling rc (an empty list, a bare assignment,
/// a funcdef — all rc 0 in the fold) is sound but is not a MEASUREMENT, so it does not erase
/// and those branches keep today's behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeadBranchProof {
    site: CfgNodeId,
    controller: AstId,
    controller_rc: Rc,
}

impl DeadBranchProof {
    /// A proof for a sibling module's TESTS, and reachable from nowhere else: the fields are
    /// private so `prove_dead_branches` stays the only production mint, and `#[cfg(test)]` is what
    /// keeps this from becoming a second one (`rul-fixture-identity-never-production` — the fence
    /// is the absence of the constructor, not a comment).
    #[cfg(test)]
    pub(crate) fn fixture(site: CfgNodeId) -> Self {
        DeadBranchProof {
            site,
            controller: AstId(0),
            controller_rc: Rc(0),
        }
    }

    /// The site whose invalidator-hood this proof licenses erasing.
    #[must_use]
    pub fn site(&self) -> CfgNodeId {
        self.site
    }

    /// The controlling node whose known rc short-circuited past the site.
    #[must_use]
    pub fn controller(&self) -> AstId {
        self.controller
    }

    /// The controller's measured rc — the number the whole derivation rests on.
    #[must_use]
    pub fn controller_rc(&self) -> Rc {
        self.controller_rc
    }
}

/// Mint a [`DeadBranchProof`] for every site this round's model proves records-dead.
///
/// THE transform (`26H` §4.6 "concentrated danger, by design"). Its four conditions, in the
/// order they are checked:
///
/// 1. **the fold proved the site unreachable** — `fold.dead_controller`, minted only from a
///    KNOWN controlling status (`inv-kfail`). The fold runs here, over the same `leaf_facts`
///    mapping and the same `observe` the plan will use, so the ledger and the artifact
///    cannot disagree about what was measured.
/// 2. **the site actually invalidates** — membership in `invalidators` (an Establish, a Kill,
///    or an Opaque, from the RESIDUAL model). Erasing anything else is a no-op that would
///    make the monotone-growth pin vacuous, and `SkipClass` cannot answer this: a kill, an
///    opaque, and a blessed pure builtin all classify `MustRun`.
/// 3. **the site is not floored into running anyway** — not in a loop body (the render
///    cannot elide one iteration) and no ⊤ successor (the execution context is unmodeled).
/// 4. **the controller is records-grounded and render-expressible** — every `Simple` leaf
///    under the controller is a valid `QueryResolvable` whose measured rc substitutes
///    ([`query_substitutes`], the shared seat), is itself unfloored, and carries no heredoc
///    or blocking output redirect that would make the render keep it verbatim.
///
/// Condition 4 is the wrong-yes fence: without it a dead site behind a KEPT live guard
/// (`omitsafe21-heredoc-guard-keeps-body`) would be erased while the artifact still runs it.
/// It re-derives "this controller will be substituted away" from analysis data alone — never
/// from a [`Disposition`](crate::Disposition), so no outcome becomes a premise.
///
/// Pure and deterministic: ordered maps throughout, no clock, no arena.
#[must_use]
pub fn prove_dead_branches(
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    invalidators: &BTreeSet<CfgNodeId>,
    effective_valid: &BTreeMap<CfgNodeId, bool>,
    observe: impl Fn(FactKey) -> Observable,
) -> Vec<DeadBranchProof> {
    let leaf_fact = leaf_facts(cfg, classes);
    let fold = crate::fold::fold(ast, |leaf| leaf_fact.get(&leaf).map(|f| observe(*f)));

    // Non-injective AstId-ward under inlining (`inv-leaf-seam`): an ambiguous leaf is DROPPED.
    let mut node_of_ast: BTreeMap<AstId, Option<CfgNodeId>> = BTreeMap::new();
    for (node, _) in classes {
        node_of_ast
            .entry(cfg.node(*node).ast)
            .and_modify(|slot| *slot = None)
            .or_insert(Some(*node));
    }
    let class_of_node: BTreeMap<CfgNodeId, &SkipClass> =
        classes.iter().map(|(node, class)| (*node, class)).collect();

    let mut proofs = Vec::new();
    for (node, _) in classes {
        let site_ast = cfg.node(*node).ast;
        let Some(controller) = fold.dead_controller(site_ast) else {
            continue;
        };
        if !invalidators.contains(node) {
            continue;
        }
        if cfg.in_loop_body(*node) || has_top_successor(cfg, *node) {
            continue;
        }
        let AbstractRc::Known(controller_rc) = fold.rc_of(controller) else {
            continue;
        };
        if !controller_substitutes_away(
            ast,
            cfg,
            controller,
            &node_of_ast,
            &class_of_node,
            &leaf_fact,
            effective_valid,
            &observe,
        ) {
            continue;
        }
        proofs.push(DeadBranchProof {
            site: *node,
            controller,
            controller_rc,
        });
    }
    proofs
}

/// Condition 4: will this controller be substituted away in the rendered artifact?
///
/// Mirrors `is_neutralised`'s compound arm — every `Simple` leaf under the controller must be
/// neutralised — but resolves each leaf through the license PREDICATE rather than through its
/// computed disposition, so no outcome becomes a premise. A controller with no command leaf
/// under it answers `false` (it reproduces nothing; the run-it direction).
///
/// The refusal check is `leaf_has_heredoc` and ONLY that, matching `is_neutralised`'s
/// `Replace` arm exactly. `leaf_has_blocking_output_redirect` is deliberately absent: it is
/// the GUARD tier's refusal, and a controller here is REPLACED, not guarded — its redirect is
/// span-elided along with the command that binds it, so it suppresses nothing. Including it
/// would not be merely conservative; `cmd >/dev/null 2>&1 || mutator` is the ladder idiom, and
/// refusing it would disable the fixpoint for the exact shape it exists to fix.
fn controller_substitutes_away(
    ast: &Ast,
    cfg: &Cfg,
    controller: AstId,
    node_of_ast: &BTreeMap<AstId, Option<CfgNodeId>>,
    class_of_node: &BTreeMap<CfgNodeId, &SkipClass>,
    leaf_fact: &BTreeMap<AstId, FactKey>,
    effective_valid: &BTreeMap<CfgNodeId, bool>,
    observe: &impl Fn(FactKey) -> Observable,
) -> bool {
    let mut any_leaf = false;
    let all = subtree_leaves_all(ast, controller, &mut any_leaf, &mut |leaf| {
        if leaf_has_heredoc(ast, leaf) {
            return false;
        }
        let Some(Some(node)) = node_of_ast.get(&leaf).copied() else {
            return false;
        };
        if cfg.in_loop_body(node) || has_top_successor(cfg, node) {
            return false;
        }
        let Some(SkipClass::QueryResolvable { .. }) = class_of_node.get(&node).copied() else {
            return false;
        };
        // The EFFECTIVE validity, not the class's origin bit (`30K` §5.2): the fold that produced
        // `observe` already ran under this view, so reading the frozen probe's bit here would judge
        // the controller against a different world than the one that measured it.
        let valid = effective_valid.get(&node).copied().unwrap_or(false);
        let status = leaf_fact
            .get(&leaf)
            .map_or(Predicted::Top, |f| observe(*f).status);
        query_substitutes(valid, &May(cfg.consumed_observables(node).clone()), status)
    });
    any_leaf && all
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{Interner, Verdict};

    /// A query oracle (`:?`, the guard) and a mutator oracle (`:`, the `||`-RHS) — the two
    /// halves of the `USER_STORY` stage-3 ladder idiom this fixpoint exists to make cascade.
    const LADDER_ORACLE: &str = r#"
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"@installed
}
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
      esac
   fi
}
"#;

    struct Model {
        ast: Ast,
        cfg: Cfg,
        classes: Vec<(CfgNodeId, SkipClass)>,
        invalidators: BTreeSet<CfgNodeId>,
    }

    /// Parse + classify `book` against [`LADDER_ORACLE`] — the origin model, nothing erased.
    fn model(book: &str) -> Model {
        let mut i = Interner::default();
        let idx = dorc_oracle::lift(&mut i, &[LADDER_ORACLE]).value;
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, LADDER_ORACLE).value];
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let (classified, _why, _kills, _coords, _backings, _narrative, invalidators) =
            dorc_analysis::effect::classify_with_why_diags(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &dorc_oracle::verdict::VerdictIndex::default(),
                &BTreeMap::new(),
                &dorc_analysis::erase::ErasedSites::none(),
                &mut i,
                &mut dorc_core::ProvArena::new(),
                &mut BTreeMap::new(),
                &mut BTreeMap::new(),
                &mut dorc_analysis::certify::CertifierTrip::default(),
                dorc_analysis::funcenv::LiveDefinitions::unsolved(),
            );
        Model {
            ast: parsed.value,
            cfg,
            classes: classified.value,
            invalidators,
        }
    }

    /// Answer every Query cell with a measured `rc`, every other cell ⊤ — the shape
    /// `facts_from_sites` produces for a valid, non-conflicted guard record.
    fn measured(model: &Model, rc: i32) -> impl Fn(FactKey) -> Observable + '_ {
        let queries: BTreeSet<FactKey> = model
            .classes
            .iter()
            .filter_map(|(_, class)| match class {
                SkipClass::QueryResolvable { fact, .. } => Some(*fact),
                _ => None,
            })
            .collect();
        move |f: FactKey| Observable {
            effect: Verdict::Converged,
            status: if queries.contains(&f) {
                Predicted::Value(Rc(rc))
            } else {
                Predicted::Top
            },
            stdout: Predicted::Top,
            stderr: Predicted::Top,
        }
    }

    /// Every origin Query is effectively valid in these models: nothing upstream mutates, which is
    /// the world each fixture is about.
    fn all_valid(model: &Model) -> BTreeMap<CfgNodeId, bool> {
        model
            .classes
            .iter()
            .filter(|(_, class)| matches!(class, SkipClass::QueryResolvable { .. }))
            .map(|(node, _)| (*node, true))
            .collect()
    }

    fn proofs_for(model: &Model, rc: i32) -> Vec<DeadBranchProof> {
        prove_dead_branches(
            &model.ast,
            &model.cfg,
            &model.classes,
            &model.invalidators,
            &all_valid(model),
            measured(model, rc),
        )
    }

    #[test]
    fn a_measured_guard_proves_its_or_right_dead() {
        let m = model("dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha\n");
        let proofs = proofs_for(&m, 0);
        assert_eq!(proofs.len(), 1, "exactly the install is proven dead");
        assert_eq!(
            proofs[0].controller_rc(),
            Rc(0),
            "on the guard's measured rc"
        );
    }

    #[test]
    fn a_failing_guard_proves_nothing_dead() {
        let m = model("dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha\n");
        assert!(
            proofs_for(&m, 1).is_empty(),
            "rc 1 left of `||` ⇒ the right operand is LIVE"
        );
    }

    #[test]
    fn an_unmeasured_guard_proves_nothing_dead() {
        let m = model("dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha\n");
        let proofs = prove_dead_branches(
            &m.ast,
            &m.cfg,
            &m.classes,
            &m.invalidators,
            &all_valid(&m),
            |_f: FactKey| Observable::verdict_only(Verdict::Converged),
        );
        assert!(proofs.is_empty(), "a ⊤ guard licenses no erasure");
    }

    #[test]
    fn a_heredoc_controller_proves_nothing_dead() {
        // THE wrong-yes fence: a render-REFUSED guard keeps its dead body verbatim behind it, so
        // that body may still run. Same book as the positive case bar the heredoc.
        let m = model(
            "dpkg -s alpha <<EOF >/dev/null 2>&1 || apt-get install -y alpha\npayload\nEOF\n",
        );
        assert!(
            proofs_for(&m, 0).is_empty(),
            "a render-refused controller erases nothing, however dead the fold says its body is"
        );
    }

    #[test]
    fn a_statically_known_controller_proves_nothing_dead() {
        // The fold DOES prove this dead, but off a static rc rather than a measurement.
        let m = model("FOO=bar || apt-get install -y alpha\n");
        assert!(
            proofs_for(&m, 0).is_empty(),
            "deadness with no measurement behind it is not a records-proven derivation"
        );
    }

    #[test]
    fn an_in_loop_site_proves_nothing_dead() {
        let m = model(
            "for p in a b; do dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha; done\n",
        );
        assert!(
            proofs_for(&m, 0).is_empty(),
            "an in-loop site runs despite the fold, so it may not be erased"
        );
    }

    #[test]
    fn a_pure_site_is_never_recorded() {
        let m = model("dpkg -s alpha >/dev/null 2>&1 || echo nothing\n");
        assert!(
            proofs_for(&m, 0).is_empty(),
            "a dead non-invalidator is not ledger material"
        );
    }

    #[test]
    fn proofs_are_deterministic_and_site_ordered() {
        let m = model(
            "dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha\n\
             dpkg -s beta >/dev/null 2>&1 || apt-get install -y beta\n",
        );
        let once = proofs_for(&m, 0);
        let twice = proofs_for(&m, 0);
        assert_eq!(once, twice, "same inputs, same proofs");
        let sites: Vec<u32> = once.iter().map(|p| p.site().0).collect();
        let mut sorted = sites.clone();
        sorted.sort_unstable();
        assert_eq!(sites, sorted, "proofs come out in site order");
    }

    #[test]
    fn licence_mint_has_exactly_one_caller() {
        // A second caller is not a refactor; it is an unproven route to shrinking the model.
        let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates dir");
        let mut callers: Vec<String> = Vec::new();
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
                    // Split so this scan does not find ITSELF: the fence is about production
                    // callers, and a needle spelled whole here would be a permanent false hit.
                    if src.contains(concat!("ErasureLicense", "::for_site(")) {
                        callers.push(path.display().to_string().replace('\\', "/"));
                    }
                }
            }
        }
        callers.sort();
        callers.retain(|p| !p.ends_with("analysis/src/erase.rs"));
        assert_eq!(
            callers.len(),
            1,
            "exactly one caller of the licence mint; found {callers:?}"
        );
        assert!(
            callers[0].ends_with("plan/src/world.rs"),
            "the sole caller is the no-execution ledger's overlay projection; found {callers:?}"
        );
    }
}
