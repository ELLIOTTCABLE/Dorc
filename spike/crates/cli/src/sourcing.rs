//! `sourcing` — the include-tree over the loaded sources: which file `.`-sources which
//! (`28Q` §2 `syn-closure-is-the-speaker`; the contract at `30G:§1`).
//!
//! Everything here is a PURE function of the loaded `(path, source)` vectors, so both drivers — the
//! binary and `WhyWorld` — derive one include-tree from one rule and cannot disagree about whose
//! utterance a licence rests on (`one-definition-table-two-drivers`, the custody half). The
//! filesystem half lives at the binary's edge, which is the only place allowed to open a file
//! (`io-at-edges-only`): it READS what a `.` names; this module decides what that means.
//!
//! # What is admitted, and why it is narrow
//!
//! An edge exists when a MARKED, non-book source spells a top-level `.` whose target resolves to
//! another loaded source that itself satisfies the dorc-lang contract — marked, and declaring
//! rather than running. That is the whole of `rul-only-oracle-sourcing-mints-speakers`, and the two
//! exclusions carry its weight: CLI co-loading mints no edge, and a BOOK mints none either.
//!
//! Nothing here proves a file inert. The contract is the AUTHOR's promise, made by marking the
//! file; a target that fails it is refused with that attribution and no edge forms
//! (`30G:rul-inertness-is-contract-never-engine-fact`).
//!
//! # Path resolution is the shell's
//!
//! A target resolves against the modeled WORKING DIRECTORY, exactly as the floor shells resolve
//! it, and the rule itself lives at one seat both kernels share
//! ([`dorc_core::loadpath`] · `30I:rul-dot-resolves-as-sh`). A slash-less target resolves NOWHERE
//! — that is a `PATH` search, which reads the ambient environment a kernel may not touch
//! (`inv-determinism` · `hermeticity-precondition`) and which POSIX leaves implementation-defined
//! when it misses, so the construct is outside the two-binary floor.
//!
//! Paths are matched in canonical form (`Cwd::resolve_operand`), so an oracle named relatively on
//! the command line and the same file sourced through a book variable are ONE entry. An
//! unresolved target SUSPENDS the sourcer rather than degrading: the engine ships declarations
//! only from a file it actually read and contract-checked.

use std::collections::BTreeSet;

use dorc_syntax::ast::NodeKind;

use crate::snapshot::StaticLoadSnapshot;

/// The include-tree, as the two things every consumer needs from it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IncludeTree {
    /// The speaker-minting `(sourcer, sourced)` edges, in source then item order.
    pub edges: Vec<(usize, usize)>,
    /// Sources holding a top-level `.` that named nothing admissible.
    ///
    /// These SUSPEND rather than degrade. A file whose environment the engine could not reconstruct
    /// is one whose helper calls may resolve to nothing, and a body that ships without a helper it
    /// calls is not reliably an rc-127 decline — a body ignoring the helper's status and answering
    /// 0 from a later test reports converged off a helper that never ran, which is the priority-1
    /// under-execute (`execution-priority-order`).
    pub unresolved: BTreeSet<usize>,
}

/// Derive the include-tree from one loaded snapshot. The BOOK contributes no edges however it
/// is spelled (`rul-book-sourcing-mints-no-speaker`), and every operand resolves against the
/// snapshot's own working directory — one input, so no caller can hand this a different world
/// than the run analysed.
#[must_use]
pub fn include_tree(snapshot: &StaticLoadSnapshot) -> IncludeTree {
    let mut tree = IncludeTree::default();
    let srcs = snapshot.source_refs();
    for (file, src) in srcs.iter().enumerate() {
        if file == snapshot.book_index() || !dorc_oracle::marker::has_marker(src) {
            continue;
        }
        for target in top_level_load_targets(src) {
            match resolve(snapshot, &target) {
                Some(sourced) => tree.edges.push((file, sourced)),
                None => drop(tree.unresolved.insert(file)),
            }
        }
    }
    tree
}

/// Does this source satisfy the dorc-lang contract a `.` may name — marked, and declaring rather
/// than running? The binary asks before admitting a file it read; [`resolve`] asks again, so a
/// source that reached the vector some other way cannot become an edge by accident.
#[must_use]
pub fn satisfies_the_contract(src: &str) -> bool {
    dorc_oracle::marker::has_marker(src) && dorc_oracle::load_inert::lint_load_inert(src).is_empty()
}

/// Every LITERALLY spelled `.` target in `src`, top level and include-guard branches alike, in
/// source order.
///
/// The binary calls this to learn what to READ; [`include_tree`] calls it to learn what to LINK.
/// One reader, so the file the driver opened and the file the tree links are the same file by
/// construction.
///
/// Guard branches are walked because that is where the healthy shared-dependency shape puts its
/// load (`30I` §2.2) — a dependency the engine refused to READ because it sat behind a guard would
/// leave every guarded package suspended.
///
/// **LITERAL only, and the cut is disclosed** (`churn-avoidance-disclosure`): an operand built from
/// a variable the CALLER set has no value here — this seat holds no loading context — so it is
/// skipped, and a sourcer whose dependency is spelled that way SUSPENDS rather than composing.
/// That is the withholding direction. The binary's acquisition loop reads such a file anyway,
/// because it drives the real loader; what is owed is CUSTODY for a variable-rooted dependency,
/// which needs the load site's own environment and is `30Ib` §5's first open question.
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

/// Which loaded source a `.` target names, or `None` when nothing admissible answers.
///
/// Matching is LEXICAL — the resolved target against canonicalized paths — because the answer must
/// be reproducible from the vectors alone, with no filesystem read (`inv-determinism`). Two
/// spellings of one file that do not canonicalize alike simply do not match, which withholds.
fn resolve(snapshot: &StaticLoadSnapshot, target: &str) -> Option<usize> {
    snapshot.source_at_dot_target(target).filter(|&file| {
        snapshot
            .source_srcs()
            .get(file)
            .is_some_and(|src| satisfies_the_contract(src))
    })
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

    /// Build the one snapshot the tree derives from. `book` names which source is the BOOK, which
    /// the snapshot always sorts last; `None` supplies an empty one.
    fn tree_at(cwd: &str, paths: &[String], srcs: &[&str], book: Option<usize>) -> IncludeTree {
        let mut paths: Vec<String> = paths.to_vec();
        let mut srcs: Vec<String> = srcs.iter().map(|s| (*s).to_owned()).collect();
        let (book_path, book_src) = match book {
            Some(at) => (paths.remove(at), srcs.remove(at)),
            None => ("book.sh".to_owned(), String::new()),
        };
        include_tree(&StaticLoadSnapshot::over(
            Cwd::at(cwd),
            paths,
            srcs,
            [].into(),
            &book_path,
            &book_src,
        ))
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

    /// The reader the binary shares: what a file says it sources, in source order, before anything
    /// has been read or resolved.
    #[test]
    fn the_target_reader_lists_what_a_file_sources() {
        let src = marked(". ./a.sh\nf() { . ./not-top-level.sh ;}\n. \"./b.sh\"\n");
        assert_eq!(top_level_load_targets(&src), ["./a.sh", "./b.sh"]);
    }
}
