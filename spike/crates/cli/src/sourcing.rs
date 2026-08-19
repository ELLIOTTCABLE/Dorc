//! `sourcing` — the include-tree over the loaded sources: which file `.`-sources which
//! (`28Q` §2 `syn-closure-is-the-speaker`; the contract at `30G:§1`).
//!
//! Everything here is a PURE function of the loaded `(path, source)` vectors and the settled
//! function environment, so both drivers — the binary and `WhyWorld` — derive one include-tree from
//! one rule and cannot disagree about whose utterance a licence rests on
//! (`one-definition-table-two-drivers`, the custody half). The filesystem half lives at the
//! binary's edge, which is the only place allowed to open a file (`io-at-edges-only`): it READS
//! what a `.` names; this module decides what that means.
//!
//! # What is admitted, and why it is narrow
//!
//! An edge exists when a MARKED, non-book source's own load PROGRAM loaded another loaded source
//! that itself satisfies the dorc-lang contract — marked, and declaring rather than running. That
//! is the whole of `rul-only-oracle-sourcing-mints-speakers`, and the two exclusions carry its
//! weight: CLI co-loading mints no edge, and a BOOK mints none either.
//!
//! Nothing here proves a file inert. The contract is the AUTHOR's promise, made by marking the
//! file; a target that fails it is refused with that attribution and no edge forms
//! (`30G:rul-inertness-is-contract-never-engine-fact`).
//!
//! # Which load happened is the LOADER's answer, not this seat's
//!
//! The edges arrive from `funcenv`'s settled account. This seat holds no loading context, so it
//! could not expand `. "$ROOT/dep.sh"` even in principle — and re-resolving literals here would
//! suspend exactly the package shape `30I` §2.1 makes canonical while looking like it worked. A
//! target resolves against the modeled WORKING DIRECTORY, exactly as the floor shells resolve it,
//! at the one seat both kernels share ([`dorc_core::loadpath`] · `30I:rul-dot-resolves-as-sh`).
//!
//! Paths are matched in canonical form (`Cwd::resolve_operand`), so an oracle named relatively on
//! the command line and the same file sourced through a book variable are ONE entry. An
//! unresolved target SUSPENDS the sourcer rather than degrading: the engine ships declarations
//! only from a file it actually read and contract-checked.

use std::collections::BTreeSet;

use dorc_syntax::ast::NodeKind;

use crate::snapshot::StaticLoadSnapshot;

/// The include-tree, as the three things its consumers need from it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IncludeTree {
    /// The speaker-minting `(sourcer, sourced)` edges, in source then item order.
    pub edges: Vec<(usize, usize)>,
    /// THE NARRATIVE PROJECTION (`30I:rul-one-load-account-separate-projections`): the
    /// `(sourcer, sourced)` pairs a loaded file's own program NAMED, aligned or not — a superset of
    /// `edges` carrying an undecided guard's fallback and anything else custody withheld.
    ///
    /// Decision-inert, and it must never be handed to a custody consumer: its whole job is telling
    /// `30I` §3.4's selected-but-unaligned case from its ambient sibling in the aid plane.
    pub selected: Vec<(usize, usize)>,
    /// Sources holding a top-level `.` that named nothing admissible.
    ///
    /// These SUSPEND rather than degrade. A file whose environment the engine could not reconstruct
    /// is one whose helper calls may resolve to nothing, and a body that ships without a helper it
    /// calls is not reliably an rc-127 decline — a body ignoring the helper's status and answering
    /// 0 from a later test reports converged off a helper that never ran, which is the priority-1
    /// under-execute (`execution-priority-order`).
    pub unresolved: BTreeSet<usize>,
}

/// Derive the include-tree from the loads the ENGINE followed (`30I:rul-one-loader-many-projections`).
///
/// The edges come from `funcenv`'s own settled account rather than from a second walk over literal
/// `.` operands here. That is what closes custody for a dependency sited through a caller-set root
/// (`. "$ROOT/dep.sh"`): this seat holds no loading context and could never expand one, so a
/// re-resolution here would silently suspend exactly the package shape `30I` §2.1 makes canonical.
/// The BOOK contributes no edges however it is spelled (`rul-book-sourcing-mints-no-speaker`), and
/// neither does CLI co-loading — the loader records an edge only from a file whose own program
/// spelled the load.
///
/// What stays HERE is the dorc-lang CONTRACT check, which the loader has no business making: a
/// target the engine bound but that does not satisfy the contract mints no edge and suspends its
/// sourcer, exactly as an unresolvable one does.
#[must_use]
pub fn include_tree(
    snapshot: &StaticLoadSnapshot,
    env: &dorc_analysis::funcenv::FuncEnv,
) -> IncludeTree {
    let mut tree = IncludeTree::default();
    let srcs = snapshot.source_srcs();
    let at =
        |key: &str| (0..srcs.len()).find(|&file| snapshot.key_of(file).as_deref() == Some(key));
    let admissible = |file: usize| {
        file != snapshot.book_index()
            && srcs
                .get(file)
                .is_some_and(|src| satisfies_the_contract(src))
    };
    for (sourcer, target) in &env.loads().speaker_edges() {
        let Some(from) = at(sourcer).filter(|&file| admissible(file)) else {
            continue;
        };
        match at(target).filter(|&file| admissible(file)) {
            Some(to) => tree.edges.push((from, to)),
            None => drop(tree.unresolved.insert(from)),
        }
    }
    // The SELECTION half: the same walk over the wider projection, and deliberately WITHOUT the
    // target-end contract check. What the author selected is a fact about their own act; whether
    // the target signed the dorc-lang contract is a separate refusal that already suspends them
    // (`UnresolvedLoad`), and folding it in here would silently retitle that world.
    for (sourcer, target) in &env.loads().selection_edges() {
        if let (Some(from), Some(to)) = (
            at(sourcer).filter(|&file| admissible(file)),
            at(target).filter(|&file| file != snapshot.book_index()),
        ) {
            tree.selected.push((from, to));
        }
    }
    for sourcer in env.loads().unresolved() {
        if let Some(from) = at(sourcer).filter(|&file| admissible(file)) {
            tree.unresolved.insert(from);
        }
    }
    tree.edges.sort_unstable();
    tree.edges.dedup();
    tree.selected.sort_unstable();
    tree.selected.dedup();
    tree
}

/// Does this source satisfy the dorc-lang contract a `.` may name — marked, and declaring rather
/// than running? The binary asks before admitting a file it read; [`include_tree`] asks again at
/// both ends of every edge, so a source that reached the vector some other way — named on the
/// command line, say — cannot become an edge by accident.
#[must_use]
pub fn satisfies_the_contract(src: &str) -> bool {
    dorc_oracle::marker::has_marker(src) && dorc_oracle::load_inert::lint_load_inert(src).is_empty()
}

/// Every LITERALLY spelled `.` target in `src`, top level and include-guard branches alike, in
/// source order.
///
/// The binary calls this to learn what to READ, before there is any environment to ask.
///
/// Guard branches are walked because that is where the healthy shared-dependency shape puts its
/// load (`30I` §2.2) — a dependency the engine refused to READ because it sat behind a guard would
/// leave every guarded package suspended.
///
/// **LITERAL only, and the cut is disclosed** (`churn-avoidance-disclosure`): an operand built from
/// a variable the CALLER set has no value here, so it is skipped. Nothing is lost — the real loader
/// names it through `FuncEnv::loads`' wanted set, which the acquisition loop reads and re-solves — and
/// LINKING it is [`include_tree`]'s job, which asks the loader rather than this walk.
#[must_use]
pub fn top_level_load_targets(src: &str) -> Vec<String> {
    fn walk(ast: &dorc_syntax::ast::Ast, items: &[dorc_core::AstId], out: &mut Vec<String>) {
        for &item in items {
            if let Some(word) = dorc_oracle::load_inert::item_is_static_load(ast, item) {
                out.extend(literal_text(ast, word));
            } else if let Some(guard) = dorc_oracle::load_inert::include_guard(ast, item) {
                walk(ast, &guard.then_, out);
                walk(ast, &guard.else_, out);
            }
        }
    }

    let ast = dorc_syntax::parse(src).value;
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk(&ast, items, &mut out);
    out
}

fn literal_text(ast: &dorc_syntax::ast::Ast, word: dorc_core::AstId) -> Option<String> {
    use dorc_syntax::ast::WordPart;
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return None;
    };
    let mut out = String::new();
    for part in parts {
        match part {
            WordPart::Literal(text) | WordPart::SingleQuoted(text) => out.push_str(text),
            WordPart::DoubleQuoted(inner) => match inner.as_slice() {
                [WordPart::Literal(text)] => out.push_str(text),
                [] => {}
                _ => return None,
            },
            _ => return None, // a parameter this seat cannot read is not a path it may claim.
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use dorc_core::loadpath::Cwd;

    use super::{IncludeTree, StaticLoadSnapshot, include_tree, top_level_load_targets};

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn marked(body: &str) -> String {
        format!("{MARKER}{body}")
    }

    /// Build the snapshot AND solve the environment the tree derives from — the whole driver path,
    /// because the edges are the loader's own account and a hand-built one would be a second
    /// resolver of exactly the kind this seat exists not to have. `book` names which source is the
    /// BOOK, which the snapshot always sorts last; `None` supplies an empty one.
    fn tree_at(cwd: &str, paths: &[String], srcs: &[&str], book: Option<usize>) -> IncludeTree {
        tree_reached(cwd, paths, srcs, book, &std::collections::BTreeSet::new())
    }

    /// [`tree_at`], with `reached` naming the sources a book `.` brings in rather than the
    /// invocation — the difference that decides whether a package's own operands see the book's
    /// variables at all.
    fn tree_reached(
        cwd: &str,
        paths: &[String],
        srcs: &[&str],
        book: Option<usize>,
        reached: &std::collections::BTreeSet<usize>,
    ) -> IncludeTree {
        let mut paths: Vec<String> = paths.to_vec();
        let mut srcs: Vec<String> = srcs.iter().map(|s| (*s).to_owned()).collect();
        let (book_path, book_src) = match book {
            Some(at) => (paths.remove(at), srcs.remove(at)),
            None => ("book.sh".to_owned(), String::new()),
        };
        let snapshot =
            StaticLoadSnapshot::over(Cwd::at(cwd), paths, srcs, reached, &book_path, &book_src);
        let mut interner = dorc_core::Interner::default();
        let ast = dorc_syntax::parse(&book_src).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        include_tree(&snapshot, &env)
    }

    /// Most cases spell their paths relative to ONE modeled working directory, which the empty
    /// string names — the shape an admin gets by running `dorc` where their files are.
    fn tree(paths: &[String], srcs: &[&str], book: Option<usize>) -> IncludeTree {
        tree_at("", paths, srcs, book)
    }

    /// The package shape `28M` §7 calls community-critical: a thin entrypoints file sourcing the
    /// helpers file that carries the bulk. One edge, and it points the way custody flows.
    #[test]
    fn an_entrypoints_file_sourcing_its_helpers_mints_one_edge() {
        let entry = marked(". ./helpers.sh\nwombat__is_converged() { _same \"$1\" ;}\n");
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = tree(&paths, &[&entry, &helpers], None);
        assert_eq!(
            tree,
            IncludeTree {
                edges: vec![(0, 1)],
                selected: vec![(0, 1)],
                unresolved: [].into()
            }
        );
    }

    /// Co-loading is INGESTION and mints nothing (`rul-cli-coloading-composes-nothing`). Without
    /// this the whole fence is decorative: any two strangers named on one command line would
    /// compose.
    #[test]
    fn co_loading_alone_mints_no_edge() {
        let entry = marked("wombat__is_converged() { _same \"$1\" ;}\n");
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        assert_eq!(
            tree(&paths, &[&entry, &helpers], None),
            IncludeTree::default()
        );
    }

    /// A BOOK's `.` mints no speaker, however it is spelled (`rul-book-sourcing-mints-no-speaker`).
    /// Its value is un-walling, which is a different lane entirely.
    #[test]
    fn a_books_source_line_mints_no_edge() {
        let book = marked(". ./helpers.sh\nwombat sync\n");
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let paths = vec!["helpers.sh".to_owned(), "book.sh".to_owned()];
        assert_eq!(
            tree(&paths, &[&helpers, &book], Some(1)),
            IncludeTree::default()
        );
    }

    /// An UNMARKED sourcer makes no dialect claim, so its `.` is ordinary shell and mints nothing
    /// (`marker-gates-syntax-only`); and a marked file naming an unmarked TARGET gets no edge
    /// either, because the target signed no contract.
    #[test]
    fn the_contract_binds_both_ends() {
        let helpers = "_same() { wombat cmp -- \"$1\" ;}\n";
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];

        let unmarked_sourcer = ". ./helpers.sh\n";
        assert_eq!(
            tree(&paths, &[unmarked_sourcer, &marked(helpers)], None),
            IncludeTree::default()
        );

        let marked_sourcer = marked(". ./helpers.sh\n");
        let tree = tree(&paths, &[&marked_sourcer, helpers], None);
        assert_eq!(tree.edges, Vec::new());
        assert_eq!(
            tree.unresolved,
            [0].into(),
            "an unmarked target is a refusal, not a silence — the sourcer suspends"
        );
    }

    /// A target that names a file carrying top-level COMMANDS fails the contract, so no edge forms
    /// and the sourcer suspends. Attribution is to the contract the target's author signed.
    #[test]
    fn a_target_that_runs_something_fails_the_contract() {
        let entry = marked(". ./helpers.sh\n");
        let helpers = marked("apt-get update\n_same() { :; }\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = tree(&paths, &[&entry, &helpers], None);
        assert_eq!(tree.edges, Vec::new());
        assert_eq!(tree.unresolved, [0].into());
    }

    /// A target nothing loaded answers SUSPENDS the sourcer rather than passing silently. The
    /// silent reading would ship a body whose helper calls resolve to nothing, which is not
    /// reliably a safe rc-127 decline.
    #[test]
    fn an_unloadable_target_suspends_its_sourcer() {
        let entry = marked(". ./missing.sh\n");
        let tree = tree(&["entry.sh".to_owned()], &[&entry], None);
        assert_eq!(tree.unresolved, [0].into());
    }

    /// A slash-less target is a `PATH` search — non-hermetic to answer here, and unspecified at the
    /// floor when `PATH` misses. It resolves nowhere and suspends.
    #[test]
    fn a_path_searched_target_resolves_nowhere() {
        let entry = marked(". helpers.sh\n");
        let helpers = marked("_same() { :; }\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = tree(&paths, &[&entry, &helpers], None);
        assert_eq!(tree.edges, Vec::new());
        assert_eq!(tree.unresolved, [0].into());
    }

    /// Transitivity is recorded as EDGES, not as a flattened reach — closing them is
    /// `core::custody`'s job, and keeping the two apart is what lets one relation answer sibling,
    /// cousin, and diamond questions from the same data.
    #[test]
    fn a_chain_records_one_edge_per_link() {
        let top = marked(". ./mid.sh\n");
        let mid = marked(". ./leaf.sh\n");
        let leaf = marked("_leaf() { :; }\n");
        let paths = vec![
            "top.sh".to_owned(),
            "mid.sh".to_owned(),
            "leaf.sh".to_owned(),
        ];
        let tree = tree(&paths, &[&top, &mid, &leaf], None);
        assert_eq!(tree.edges, vec![(0, 1), (1, 2)]);
    }

    /// Two entries sourcing one helpers file each get their own edge — the diamond, which the
    /// closed `pin-closure-membership-and-diamond` allows and `core::custody` resolves to custody
    /// for both.
    #[test]
    fn a_diamond_records_both_edges() {
        let a = marked(". ./shared.sh\n");
        let b = marked(". ./shared.sh\n");
        let shared = marked("_s() { :; }\n");
        let paths = vec!["a.sh".to_owned(), "b.sh".to_owned(), "shared.sh".to_owned()];
        assert_eq!(
            tree(&paths, &[&a, &b, &shared], None).edges,
            vec![(0, 2), (1, 2)]
        );
    }

    /// A guard the engine cannot decide mints NOTHING
    /// (`rul-speaker-minting-is-oracle-sourcing-only`, as amended by
    /// `30I:rul-guarded-source-mints-exact-speaker-edge`).
    ///
    /// The load is authored, but on the reuse route no `.` ran at all — so an edge minted from an
    /// undecided branch would rest this author's licence on whoever really loaded the target. The
    /// `command -v` shape over an ORDINARY helper name is exactly that world: a host binary could
    /// answer the query, so neither branch is decided (`notes/30Ic`).
    #[test]
    fn an_undecided_guard_mints_no_edge() {
        let entry = marked(
            "if command -v _same >/dev/null 2>&1; then\n   :\nelse\n   . ./helpers.sh\nfi\n\
             wombat__is_converged() { _same \"$1\" ;}\n",
        );
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = tree(&paths, &[&entry, &helpers], None);
        assert_eq!(tree.edges, Vec::new());
        assert_eq!(tree.unresolved, [].into());
    }

    /// …and the RECOGNIZED sentinel guard does mint it
    /// (`30I:rul-guarded-source-mints-exact-speaker-edge`): the package's own load value says
    /// whether the package is live, so both arms land on the same speech and there is no
    /// analysis-time choice between speakers to be made.
    ///
    /// The pair is the whole ruling in two cases: the same authored `.`, the same target, and the
    /// custody follows the engine's ability to say the load really happened.
    #[test]
    fn a_recognized_sentinel_guard_mints_its_edge() {
        let entry = marked(
            "if [ \"${_sm_helpers-}\" != 'sm.helpers/v1' ]; then\n   . ./helpers.sh\nfi\n\
             wombat__is_converged() { _same \"$1\" ;}\n",
        );
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n_sm_helpers='sm.helpers/v1'\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        assert_eq!(tree(&paths, &[&entry, &helpers], None).edges, vec![(0, 1)]);
    }

    /// THE NARRATIVE PROJECTION, beside the speaker one, over the SAME undecided guard
    /// (`30I:rul-one-load-account-separate-projections`). Nothing about the licence moves: the edge
    /// still does not mint. What the wider relation buys is the difference between telling this
    /// author their guarded dependency did not align and telling them they named none — and getting
    /// that backwards points them at a repair they already made (`271:rul-sin-ordering`).
    #[test]
    fn an_undecided_guard_is_selected_though_it_mints_no_edge() {
        let entry = marked(
            "if command -v _same >/dev/null 2>&1; then\n   :\nelse\n   . ./helpers.sh\nfi\n\
             wombat__is_converged() { _same \"$1\" ;}\n",
        );
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = tree(&paths, &[&entry, &helpers], None);
        assert_eq!(tree.edges, Vec::new());
        assert_eq!(tree.selected, vec![(0, 1)]);
    }

    /// A BOOK selects nothing on anyone's behalf, in either relation — the sourcer species keeps it
    /// out of both (`30I:rul-books-load-but-do-not-speak`), so a book's `.` can never make an
    /// oracle's ambient reach read as that oracle author's own selection.
    #[test]
    fn a_books_loads_select_nothing_either() {
        let book = marked(". ./alpha.sh\nwombat sync\n");
        let alpha = marked("_same() { :; }\n");
        let paths = vec!["alpha.sh".to_owned(), "book.sh".to_owned()];
        assert_eq!(
            tree(&paths, &[&alpha, &book], Some(1)),
            IncludeTree::default()
        );
    }

    /// The book half of `30I:rul-books-load-but-do-not-speak`, at every spelling a book has: a
    /// book's `.` changes which definitions are LIVE and contributes no speaker edge, guard-nested
    /// or not. Custody is asymmetric containment, and a book sourcing two packages does not make
    /// itself their shared author.
    #[test]
    fn a_books_loads_change_visibility_and_mint_no_speaker() {
        let book = marked(
            "if command -v _same >/dev/null 2>&1; then\n   :\nelse\n   . ./alpha.sh\nfi\n\
             . ./beta.sh\nwombat sync\n",
        );
        let alpha = marked("_same() { :; }\n");
        let beta = marked("_other() { :; }\n");
        let paths = vec![
            "alpha.sh".to_owned(),
            "beta.sh".to_owned(),
            "book.sh".to_owned(),
        ];
        assert_eq!(
            tree(&paths, &[&alpha, &beta, &book], Some(2)),
            IncludeTree::default(),
            "no edges and no suspension — a book's loads are simply not this relation's subject"
        );
    }

    /// Spellings that differ only in `./` noise or separator name the same file. Matching is
    /// lexical, so it stops exactly there: nothing here resolves a symlink or asks the filesystem.
    #[test]
    fn matching_is_lexical_normalization() {
        let entry = marked(". ./oracles/h.sh\n");
        let helpers = marked("_h() { :; }\n");
        let paths = vec!["e.sh".to_owned(), "oracles//h.sh".to_owned()];
        assert_eq!(tree(&paths, &[&entry, &helpers], None).edges, vec![(0, 1)]);
    }

    /// A target resolves against the modeled WORKING DIRECTORY, exactly as sh does
    /// (`30I:rul-dot-resolves-as-sh`) — never against the sourcing file's own directory. The
    /// difference is visible precisely where a package is not sitting under the cwd: an
    /// entrypoint in `pkg/` spelling `./helpers.sh` names `helpers.sh` beside the ADMIN, which is
    /// what a shell would do and what the off-ramp reproduces.
    #[test]
    fn a_target_resolves_against_the_working_directory() {
        let entry = marked(". ./helpers.sh\n");
        let helpers = marked("_h() { :; }\n");
        let sources: [&str; 2] = [&entry, &helpers];

        let beside_the_admin = vec![
            "/ops/pkg/entry.oracle.sh".to_owned(),
            "/ops/helpers.sh".to_owned(),
        ];
        let beside_the_entrypoint = vec![
            "/ops/pkg/entry.oracle.sh".to_owned(),
            "/ops/pkg/helpers.sh".to_owned(),
        ];

        assert_eq!(
            tree_at("/ops", &beside_the_admin, &sources, None).edges,
            vec![(0, 1)],
            "`./helpers.sh` names the file beside the ADMIN, which is what a shell would do"
        );
        assert_eq!(
            tree_at("/ops", &beside_the_entrypoint, &sources, None).unresolved,
            [0].into(),
            "a helpers file beside its ENTRYPOINT is not what that line names from here"
        );
        assert_eq!(
            tree_at("/ops/pkg", &beside_the_entrypoint, &sources, None).edges,
            vec![(0, 1)],
            "...and it is exactly what the same line names once the admin stands in the package"
        );
    }

    /// CUSTODY FOR A VARIABLE-ROOTED DEPENDENCY (`30I:force-root-value-flow`) — the canonical
    /// package shape sites its own dependency through a root ITS CALLER SET, so the operand has no
    /// value at all until the load actually happens.
    ///
    /// The edge exists because the engine that FOLLOWED the load is the engine that reports it.
    /// Under the literal walk this replaced, the same package suspended its own vouches: the
    /// operand was skipped as unreadable, the sourcer landed in `unresolved`, and the community
    /// shape `30I` §2.1 makes canonical could not compose at all.
    #[test]
    fn a_dependency_sited_through_the_callers_root_takes_custody() {
        let entry = marked(". \"$OPS_LIB/helpers.sh\"\nwombat__is_converged() { _same \"$1\" ;}\n");
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let book = "OPS_LIB=.\n. ./entry.sh\nwombat sync\n";
        let paths = vec![
            "entry.sh".to_owned(),
            "helpers.sh".to_owned(),
            "book.sh".to_owned(),
        ];
        let tree = tree_reached(
            "",
            &paths,
            &[&entry, &helpers, book],
            Some(2),
            &[0, 1].into(),
        );
        assert_eq!(
            tree,
            IncludeTree {
                edges: vec![(0, 1)],
                selected: vec![(0, 1)],
                unresolved: [].into()
            }
        );
    }

    /// The reader the binary shares: what a file says it sources, in source order, before anything
    /// has been read or resolved.
    #[test]
    fn the_target_reader_lists_what_a_file_sources() {
        let src = marked(". ./a.sh\nf() { . ./not-top-level.sh ;}\n. \"./b.sh\"\n");
        assert_eq!(top_level_load_targets(&src), ["./a.sh", "./b.sh"]);
    }
}
