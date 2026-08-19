//! `custody` — the whole-unit census of cross-custody dependencies no author announced
//! (`30I:rul-unannounced-cross-custody-fails-before-network`).
//!
//! `30I` §3.4 keeps three cases apart, and only the third is this module's subject. A voucher may
//! rest on a dependency it `.`-sourced (`dependency-explicitly-sourced`), or on one a recognized
//! include guard names under §3.4's exact proof (`dependency-guarded-source-exact`) — both are
//! authorship, and [`dorc_oracle::closure::HelperIndex`] already lets them through. The third is a
//! role body calling a cross-custody name with NEITHER: the declaration a shell binds simply
//! happened to be live, because somebody named two files on one command line
//! (`dependency-merely-happened-to-be-live`).
//!
//! # Why this is a refusal rather than the suspension it used to be
//!
//! The suspension is still there and still correct per site — a vouch whose composition is
//! somebody else's does not attach, so the site runs. What the suspension cannot say is that the
//! ORACLE SET is invalid contracted input: Dorc cannot tell an intended dependency injection from
//! an accidental function shadow, and guessing either way is the mis-attributed class
//! (`271:rul-sin-ordering`). So v0 refuses the whole run before any host is contacted and before
//! any mutation-authorizing plan exists, and says which call and which live declaration it means.
//!
//! This is a whole-unit question, deliberately: it is asked of every role definition in every
//! loaded source, whether or not a book site happens to reach one. A run whose book touches none
//! of the affected families would otherwise ship happily today and refuse tomorrow when a line
//! moved, which is the worst moment to learn that the packages never composed.
//!
//! # What "unannounced" means, and the judgment call inside it (OPEN)
//!
//! `30I` §3.4 carries two sentences that pull against each other. Case 3 defines the refusal's
//! subject as "a voucher calls a cross-custody function without the exact source-bearing
//! acceptance in case 1 or 2", which reads as: any inexact alignment refuses. Its closing
//! paragraph then says "whatever the engine cannot align exactly withholds vouch, licensure, and
//! speaker status under the existing collapse accounting" — the ordinary suspension, which runs
//! the site and continues. Taken literally together, the second sentence would be dead letter.
//!
//! This module reads them the narrow way: the refusal's subject is a call its author announced
//! NOWHERE — no `.`, no include guard naming the dependency, no `command` routing. An author who
//! DID write an acceptance act, and whose act the engine then could not align exactly, keeps the
//! ordinary suspension. Two reasons, and neither is decisive on its own:
//!
//! - **Attribution.** A package whose sentinel a stranger also assigns, or whose helper a later
//!   file shadows, did nothing wrong; the cause is a third file. Refusing the run in that author's
//!   name points the reader at the wrong repair, which is the pope-sin direction
//!   (`271:rul-sin-ordering`).
//! - **The remediation list.** The ruling's own suggestions — `command`, explicit sourcing, a
//!   guarded fallback source, renaming — are the acts a file with NO acceptance act is missing.
//!   They are not advice you can give an author who already wrote the guard.
//!
//! It is a `tc-*`-shaped call about how two sentences of a typed ruling compose, so it is FLAGGED
//! rather than settled (`inv-superposition`). The broad reading is one predicate away, and it is
//! the corpus that shows what it would cost: `load30-speaker-minting-is-observable`'s two
//! counterfactual worlds are announced-but-unaligned by construction, and under the broad reading
//! the whole run refuses and no run set can be observed at all.
//!
//! # Not a licensure widening in either direction
//!
//! Nothing here mints, resolves, or relaxes anything: the answer is read off the same
//! `closure_for` the vouch lift consults, so the refusal and the suspension can never disagree
//! about what custody reaches. Only [`DenialReason::ResolvedOutsideCustody`] is this refusal's
//! subject — the book-side denials, the within-custody contest and the unenumerable call keep
//! their own withholding behaviour, because each of those is a world the engine CAN describe.
//!
//! [`DenialReason::ResolvedOutsideCustody`]: dorc_oracle::closure::DenialReason::ResolvedOutsideCustody

use dorc_core::Span;
use dorc_oracle::closure::{DenialReason, HelperIndex};
use dorc_syntax::ast::NodeKind;

use crate::snapshot::StaticLoadSnapshot;

/// One role definition reaching a name whose live declaration is outside its custody.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnannouncedDependency {
    /// The loaded source whose role definition makes the call.
    pub file: usize,
    /// That definition's span, in `file` — where the diagnostic points.
    pub span: Span,
    /// The name whose resolution left the definition's custody.
    pub name: String,
    /// Where the declaration a shell would bind is authored, `path:line`-shaped. Empty when the
    /// denial named no site, which cannot arise for this reason but is not worth a panic.
    pub live: String,
}

/// Every unannounced cross-custody dependency the loaded sources carry, in load then source order.
///
/// The BOOK is not a subject: an admin's own file is under no oracle contract, and its funcdefs
/// answer for its own sites only (`30I:rul-books-load-but-do-not-speak`). Nor is a source that
/// signs no dorc-lang contract — an unmarked file makes no dialect claim, so reading a
/// package-composition promise into it would be inventing one its author never made.
#[must_use]
pub fn unannounced_cross_custody(
    snapshot: &StaticLoadSnapshot,
    helpers: &HelperIndex,
) -> Vec<UnannouncedDependency> {
    let mut found: Vec<UnannouncedDependency> = Vec::new();
    for (file, src, _) in snapshot.sources() {
        if file == snapshot.book_index() || !crate::sourcing::satisfies_the_contract(src) {
            continue;
        }
        let ast = dorc_syntax::parse(src).value;
        let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
            continue;
        };
        for &item in items {
            let node = ast.node(item);
            let NodeKind::FuncDef { name, .. } = &node.kind else {
                continue;
            };
            if dorc_oracle::reserved::role_family(name).is_none() {
                continue;
            }
            let Some(body) = src.get(node.span.lo.0 as usize..node.span.hi.0 as usize) else {
                continue;
            };
            let Err(denial) = helpers.closure_for(file, body) else {
                continue;
            };
            if denial.reason != DenialReason::ResolvedOutsideCustody {
                continue;
            }
            // THE NARROWING (flagged OPEN — see the module doc): only a call its author announced
            // NOWHERE is this refusal's subject. If any source the author's own acceptance acts
            // reach DECLARES this name, the dependency was announced — a later file shadowing it,
            // or a stranger defeating the guard's recognition, is somebody else's doing and keeps
            // the ordinary suspension.
            let accepted = announced(snapshot, file);
            if denial.sites.iter().any(|&(at, _)| accepted.contains(&at)) {
                continue;
            }
            // One sentence per (definition, name): a body calling the same stranger twice is one
            // composition to repair, and N sentences would be a correlated cascade
            // (`28O:dec-one-diagnostic-per-file-not-per-item`).
            if found
                .iter()
                .any(|prior| prior.file == file && prior.name == denial.name)
            {
                continue;
            }
            found.push(UnannouncedDependency {
                file,
                span: node.span,
                name: denial.name.clone(),
                live: denial
                    .sites
                    .last()
                    .and_then(|&(at, span)| locus(snapshot, at, span))
                    .unwrap_or_default(),
            });
        }
    }
    found
}

/// Every loaded source the file at `from` ACCEPTS, transitively — the acceptance acts its author
/// actually wrote, whether or not the engine could align any of them exactly.
///
/// This is deliberately NOT the custody relation. Custody answers "may this author's vouch rest on
/// that utterance", and it withholds whenever the alignment is inexact — a package whose sentinel a
/// stranger also assigns, or whose helper a later file shadows, takes no custody even though its
/// author wrote a perfectly ordinary guarded source. The refusal's subject is narrower than that:
/// an author who wrote NOTHING. Where an acceptance act exists and merely could not be verified,
/// the suspension already says so and the site runs, which is the behaviour
/// `30I` §3.4's own "whatever the engine cannot align exactly withholds" sentence describes.
///
/// **Disclosed cut** (`churn-avoidance-disclosure`): the walk is LITERAL-only, because the loader
/// reports no unfiltered edge set (`30Ib` §15). An announcement spelled through a caller-set root
/// (`. "$ROOT/dep.sh"`) is therefore invisible here. That costs nothing in the ordinary case —
/// such a load RESOLVES, so custody reaches and no denial arises at all — and bites only in the
/// cell where a third file then shadows the helper: the variable-rooted package refuses where its
/// literal-rooted twin suspends. The direction is loud and conservative (a refusal emits no plan,
/// so nothing under-executes), and closing it is the same accessor step 5 has to open with.
fn announced(snapshot: &StaticLoadSnapshot, from: usize) -> std::collections::BTreeSet<usize> {
    let mut reached = std::collections::BTreeSet::new();
    let mut frontier = vec![from];
    while let Some(file) = frontier.pop() {
        let Some(src) = snapshot.source_srcs().get(file) else {
            continue;
        };
        for target in crate::sourcing::top_level_load_targets(src) {
            if let Some(next) = snapshot.source_at_dot_target(&target)
                && reached.insert(next)
            {
                frontier.push(next);
            }
        }
    }
    reached
}
/// `path:line` for a span in one loaded source.
fn locus(snapshot: &StaticLoadSnapshot, file: usize, span: Span) -> Option<String> {
    let path = snapshot.source_paths().get(file)?;
    let src = snapshot.source_srcs().get(file)?;
    let (line, _) = dorc_aid::diag::line_col(src, span.lo.0 as usize);
    Some(format!("{path}:{line}"))
}

#[cfg(test)]
mod tests {
    use dorc_core::loadpath::Cwd;

    use super::{UnannouncedDependency, unannounced_cross_custody};
    use crate::snapshot::StaticLoadSnapshot;

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn marked(body: &str) -> String {
        format!("{MARKER}{body}")
    }

    /// Build the whole world a run builds — snapshot, environment, include-tree, helper index —
    /// so the census answers off the SAME custody the vouch lift consults. A hand-built index
    /// would be a second answer to the one question this module exists not to re-derive.
    fn census(paths: &[&str], srcs: &[&str], book: &str) -> Vec<UnannouncedDependency> {
        let snapshot = StaticLoadSnapshot::over(
            Cwd::at(""),
            paths.iter().map(|p| (*p).to_owned()).collect(),
            srcs.iter().map(|s| (*s).to_owned()).collect(),
            &std::collections::BTreeSet::new(),
            "book.sh",
            book,
        );
        let mut interner = dorc_core::Interner::default();
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        let tree = crate::sourcing::include_tree(&snapshot, &env);
        let source_refs = snapshot.source_refs();
        let helpers =
            dorc_oracle::closure::HelperIndex::build(&source_refs, Some(snapshot.book_index()))
                .with_include_tree(
                    dorc_core::CustodyClosures::from_edges(source_refs.len(), &tree.edges),
                    tree.unresolved.clone(),
                );
        unannounced_cross_custody(&snapshot, &helpers)
    }

    /// THE REFUSAL'S SUBJECT (`30I` §3.4 case 3): two packages named on one command line, one
    /// calling the other's helper. Co-loading is ingestion and composes no custody, so the call
    /// reaches an utterance its author never made — and v0 cannot tell that from an intended
    /// injection, so it refuses rather than guessing.
    #[test]
    fn a_bare_cross_custody_call_is_unannounced() {
        let found = census(
            &["entry.oracle.sh", "helpers.oracle.sh"],
            &[
                &marked("wombat__is_converged() {\n   _wombat_dest \"$1\"\n}\n"),
                &marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n"),
            ],
            "wombat sync a.conf\n",
        );
        assert_eq!(found.len(), 1, "one composition, one sentence");
        assert_eq!(found[0].name, "_wombat_dest");
        assert_eq!(found[0].file, 0, "spanned at the CALLING definition's file");
        assert_eq!(
            found[0].live, "helpers.oracle.sh:2",
            "and naming where the declaration a shell binds is authored"
        );
    }

    /// `dependency-explicitly-sourced` (§3.4 case 1): the same two files, one `.` between them.
    /// The call is now inside its author's own custody and nothing is refused — which is the whole
    /// remediation the diagnostic points at.
    #[test]
    fn an_explicitly_sourced_dependency_is_announced() {
        let found = census(
            &["entry.oracle.sh", "helpers.oracle.sh"],
            &[
                &marked(
                    ". ./helpers.oracle.sh\nwombat__is_converged() {\n   _wombat_dest \"$1\"\n}\n",
                ),
                &marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n"),
            ],
            "wombat sync a.conf\n",
        );
        assert_eq!(found, Vec::new());
    }

    /// `dependency-guarded-source-exact` (§3.4 case 2): the recognized package sentinel mints the
    /// same speaker edge a direct `.` does, so the guarded shape composes and is not refused. The
    /// pair with the case above is what keeps the refusal from reading as "cross-file calls are
    /// banned" — it is bare, unannounced ones that are.
    #[test]
    fn a_recognized_sentinel_guard_announces_its_dependency() {
        let entry = marked(
            "if [ \"${_sm_helpers-}\" != 'sm.helpers/v1' ]; then\n   . ./helpers.oracle.sh\nfi\n\
             wombat__is_converged() {\n   _wombat_dest \"$1\"\n}\n",
        );
        let helpers =
            marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n_sm_helpers='sm.helpers/v1'\n");
        assert_eq!(
            census(
                &["entry.oracle.sh", "helpers.oracle.sh"],
                &[&entry, &helpers],
                "wombat sync a.conf\n"
            ),
            Vec::new()
        );
    }

    /// `deliberate-external-utility` (§3.4): a call routed through `command` asks for the TOOL by
    /// that name, never for whatever function happens to be live — so it reaches no helper and is
    /// nothing for this census to refuse. It is the escape hatch the diagnostic suggests, and it
    /// has to work for the suggestion to be honest.
    #[test]
    fn a_command_prefixed_utility_is_not_a_dependency() {
        let found = census(
            &["entry.oracle.sh", "helpers.oracle.sh"],
            &[
                &marked("wombat__is_converged() {\n   command _wombat_dest \"$1\"\n}\n"),
                &marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n"),
            ],
            "wombat sync a.conf\n",
        );
        assert_eq!(found, Vec::new());
    }

    /// THE NARROWING, pinned (the OPEN judgment call in the module doc): an author who DID write a
    /// guarded source naming the dependency, but whose sentinel a STRANGER also assigns, keeps the
    /// ordinary suspension rather than refusing the run. Nothing this author wrote is wrong; the
    /// third file is what defeated the recognition, and naming this author in a refusal would point
    /// the reader at the wrong repair.
    ///
    /// This is the shape `load30-speaker-minting-is-observable`'s first counterfactual carries, and
    /// it is why the whole-product case can still observe a RUN SET at all.
    #[test]
    fn an_announced_dependency_whose_guard_is_defeated_is_not_refused() {
        let entry = marked(
            "if [ \"${sm_beta_loaded-}\" != 'sm.beta/v1' ]; then\n   . ./dep.oracle.sh\nfi\n\
             beta__is_converged() {\n   sm_beta_check \"$1\"\n}\n",
        );
        let dep =
            marked("sm_beta_check() {\n   common cmp -- \"$1\"\n}\nsm_beta_loaded='sm.beta/v1'\n");
        let stranger = marked("sm_beta_loaded='sm.beta/v1'\n");
        assert_eq!(
            census(
                &["entry.oracle.sh", "dep.oracle.sh", "stranger.oracle.sh"],
                &[&entry, &dep, &stranger],
                "beta sync b.conf\n"
            ),
            Vec::new(),
            "announced but unaligned suspends; it does not refuse"
        );
    }

    /// The same narrowing from the other side: an acceptance act naming SOME OTHER file does not
    /// announce this dependency. The walk is per-name, not per-author — otherwise one `.` anywhere
    /// in a package would license every bare cross-custody call it makes.
    #[test]
    fn announcing_a_different_file_announces_nothing_here() {
        let entry = marked(
            ". ./unrelated.oracle.sh\nwombat__is_converged() {\n   _wombat_dest \"$1\"\n}\n",
        );
        let unrelated = marked("_unrelated() {\n   :\n}\n");
        let helpers = marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n");
        let found = census(
            &[
                "entry.oracle.sh",
                "unrelated.oracle.sh",
                "helpers.oracle.sh",
            ],
            &[&entry, &unrelated, &helpers],
            "wombat sync a.conf\n",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "_wombat_dest");
    }

    /// The BOOK is not a subject, at either end. Its own funcdefs answer for its own sites, and
    /// the admin signs no package contract (`30I:rul-books-load-but-do-not-speak`). A book calling
    /// an oracle's helper is the un-walling lane's question, never this one's.
    #[test]
    fn the_book_is_not_a_subject() {
        let found = census(
            &["helpers.oracle.sh"],
            &[&marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n")],
            "wombat__is_converged() {\n   _wombat_dest \"$1\"\n}\nwombat sync a.conf\n",
        );
        assert_eq!(found, Vec::new());
    }

    /// A NON-role definition is not a subject either: only a role member speaks for a site, so a
    /// package's private plumbing calling across files carries no license to withhold and nothing
    /// to refuse. Over-refusing here would make the rule about file layout rather than about
    /// whose judgment answers a site.
    #[test]
    fn a_private_helper_calling_across_files_is_not_a_subject() {
        let found = census(
            &["entry.oracle.sh", "helpers.oracle.sh"],
            &[
                &marked("_wombat_local() {\n   _wombat_dest \"$1\"\n}\n"),
                &marked("_wombat_dest() {\n   wombat cmp -- \"$1\"\n}\n"),
            ],
            "wombat sync a.conf\n",
        );
        assert_eq!(found, Vec::new());
    }
}
