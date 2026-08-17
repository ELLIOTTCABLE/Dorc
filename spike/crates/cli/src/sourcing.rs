//! `sourcing` — the include-tree over the loaded sources: which file `.`-sources which
//! (`28Q` §2 `syn-closure-is-the-speaker`; the contract at `30C:§1`).
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
//! (`30C:rul-inertness-is-contract-never-engine-fact`).
//!
//! # Path resolution is relative to the SOURCING FILE — flagged, and the human's to overturn
//!
//! A target is resolved against the directory of the file that spells it, and a slash-less target
//! resolves NOWHERE (a `PATH` search reads the ambient environment, which the kernel may not do —
//! `inv-determinism` · `hermeticity-precondition` — and POSIX leaves the not-found case
//! implementation-defined besides, so the construct is outside the two-binary floor).
//!
//! Sourcing-file-relative is NOT what a running shell does: POSIX resolves a slash-bearing `.`
//! operand against the WORKING DIRECTORY, and `rul-unsure-falls-toward-sh-parity` binds name
//! resolution by name. The argument for diverging, stated so it can be judged and reversed:
//!
//! **An oracle's top-level `.` never executes.** Oracle bodies reach the artifact by TRANSPLANT —
//! the emitted preamble carries the declarations inline, and `dorc strip` erases the marked file's
//! own text — so no shell ever evaluates this line and there is no runtime behaviour to match. The
//! construct is a LOADER directive, read once at analysis time to answer "whose declarations are
//! these", and every loader in the world resolves an include against the including file.
//!
//! **Working-directory-relative would make the ruled deliverable unreachable.** An admin naming
//! `-o /some/pkg/entry.oracle.sh` from anywhere else has a working directory with no `helpers.sh`
//! in it, so `28M` §7's helpers-plus-thin-entrypoints package — the shape the human named
//! community-critical, and the whole payoff of the sourcing build — would compose only when the
//! admin happened to `cd` into the package first. Measured on this tree: the e2e runner drives
//! every case from a throwaway sandbox with absolute oracle paths, so under the working-directory
//! rule no corpus cell could exercise the feature at all.
//!
//! Neither reading is a correctness fork. The engine ships declarations only from a file it
//! actually read and contract-checked, so the two rules differ in WHICH file answers, never in
//! whether an unread one might: an unresolved target SUSPENDS. If the human prefers strict sh
//! parity, [`resolve_against`] is the single function to change.

use std::collections::BTreeSet;

use dorc_syntax::ast::NodeKind;

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

/// Derive the include-tree from the loaded sources. `book` names the book's index, when there is
/// one — a book contributes no edges (`rul-book-sourcing-mints-no-speaker`).
#[must_use]
pub fn include_tree(paths: &[String], srcs: &[&str], book: Option<usize>) -> IncludeTree {
    let mut tree = IncludeTree::default();
    for (file, src) in srcs.iter().enumerate() {
        let Some(here) = paths.get(file) else {
            continue;
        };
        if book == Some(file) || !dorc_oracle::marker::has_marker(src) {
            continue;
        }
        for target in top_level_load_targets(src) {
            match resolve(here, &target, paths, srcs) {
                Some(sourced) => tree.edges.push((file, sourced)),
                None => drop(tree.unresolved.insert(file)),
            }
        }
    }
    tree
}

/// Where a `.` target lands, given the file that spells it: the target joined onto that file's
/// directory, in lexical normal form. `None` for a slash-less target, which names a `PATH` search
/// this seat may not perform.
///
/// THE seat the module doc's resolution ruling lives in — one function to change if strict
/// working-directory parity is ever preferred.
#[must_use]
pub fn resolve_against(sourcer_path: &str, target: &str) -> Option<String> {
    if !target.contains('/') {
        return None;
    }
    let here = normalize(sourcer_path);
    let dir = match here.rsplit_once('/') {
        // A sourcer at the filesystem root: its directory IS the root, and dropping to the
        // empty string here would silently re-root the target as relative.
        Some(("", _)) => "/",
        Some((parent, _)) => parent,
        None => "",
    };
    Some(if dir.is_empty() {
        normalize(target)
    } else {
        normalize(&format!("{dir}/{target}"))
    })
}

/// Does this source satisfy the dorc-lang contract a `.` may name — marked, and declaring rather
/// than running? The binary asks before admitting a file it read; [`resolve`] asks again, so a
/// source that reached the vector some other way cannot become an edge by accident.
#[must_use]
pub fn satisfies_the_contract(src: &str) -> bool {
    dorc_oracle::marker::has_marker(src) && dorc_oracle::load_inert::lint_load_inert(src).is_empty()
}

/// Every statically spelled top-level `.` target in `src`, in source order.
///
/// The binary calls this to learn what to READ; [`include_tree`] calls it to learn what to LINK.
/// One reader, so the file the driver opened and the file the tree links are the same file by
/// construction.
#[must_use]
pub fn top_level_load_targets(src: &str) -> Vec<String> {
    let ast = dorc_syntax::parse(src).value;
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|&item| dorc_oracle::load_inert::item_is_static_load(&ast, item))
        .filter_map(|word| literal_text(&ast, word))
        .collect()
}

/// Which loaded source a `.` target names, or `None` when nothing admissible answers.
///
/// Matching is LEXICAL — the resolved target against `normalize`d paths — because the answer must
/// be reproducible from the vectors alone, with no filesystem read (`inv-determinism`). Two
/// spellings of one file that do not normalize alike simply do not match, which withholds.
fn resolve(sourcer_path: &str, target: &str, paths: &[String], srcs: &[&str]) -> Option<usize> {
    let wanted = resolve_against(sourcer_path, target)?;
    paths
        .iter()
        .position(|path| normalize(path) == wanted)
        .filter(|&file| {
            srcs.get(file)
                .is_some_and(|src| satisfies_the_contract(src))
        })
}

/// A path's lexical normal form: `\` folded to `/`, `.` segments dropped, `..` popped where a real
/// segment precedes it, and a LEADING separator preserved. Purely textual — it never touches the
/// filesystem, so it cannot resolve a symlink and does not pretend to.
///
/// The leading separator is load-bearing rather than cosmetic, and dropping it was a real bug this
/// tree caught only on its Linux leg (`one-platform-green-is-not-cross-platform-green`): an
/// absolute POSIX oracle path came back RELATIVE, so the sourced file was looked for under the
/// working directory and never found. Windows paths hid it — a drive letter has no leading
/// separator to lose.
#[must_use]
pub fn normalize(path: &str) -> String {
    let rooted = path.starts_with('/') || path.starts_with('\\');
    let mut out: Vec<&str> = Vec::new();
    for segment in path.split(['/', '\\']) {
        match segment {
            "" | "." => {}
            ".." if out.last().is_some_and(|last| *last != "..") => drop(out.pop()),
            other => out.push(other),
        }
    }
    let joined = out.join("/");
    if rooted { format!("/{joined}") } else { joined }
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
    use super::{IncludeTree, include_tree, normalize, top_level_load_targets};

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn marked(body: &str) -> String {
        format!("{MARKER}{body}")
    }

    /// The package shape `28M` §7 calls community-critical: a thin entrypoints file sourcing the
    /// helpers file that carries the bulk. One edge, and it points the way custody flows.
    #[test]
    fn an_entrypoints_file_sourcing_its_helpers_mints_one_edge() {
        let entry = marked(". ./helpers.sh\nwombat__is_converged() { _same \"$1\" ;}\n");
        let helpers = marked("_same() { wombat cmp -- \"$1\" ;}\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = include_tree(&paths, &[&entry, &helpers], None);
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
            include_tree(&paths, &[&entry, &helpers], None),
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
            include_tree(&paths, &[&helpers, &book], Some(1)),
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
            include_tree(&paths, &[unmarked_sourcer, &marked(helpers)], None),
            IncludeTree::default()
        );

        let marked_sourcer = marked(". ./helpers.sh\n");
        let tree = include_tree(&paths, &[&marked_sourcer, helpers], None);
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
        let tree = include_tree(&paths, &[&entry, &helpers], None);
        assert_eq!(tree.edges, Vec::new());
        assert_eq!(tree.unresolved, [0].into());
    }

    /// A target nothing loaded answers SUSPENDS the sourcer rather than passing silently. The
    /// silent reading would ship a body whose helper calls resolve to nothing, which is not
    /// reliably a safe rc-127 decline.
    #[test]
    fn an_unloadable_target_suspends_its_sourcer() {
        let entry = marked(". ./missing.sh\n");
        let tree = include_tree(&["entry.sh".to_owned()], &[&entry], None);
        assert_eq!(tree.unresolved, [0].into());
    }

    /// A slash-less target is a `PATH` search — non-hermetic to answer here, and unspecified at the
    /// floor when `PATH` misses. It resolves nowhere and suspends.
    #[test]
    fn a_path_searched_target_resolves_nowhere() {
        let entry = marked(". helpers.sh\n");
        let helpers = marked("_same() { :; }\n");
        let paths = vec!["entry.sh".to_owned(), "helpers.sh".to_owned()];
        let tree = include_tree(&paths, &[&entry, &helpers], None);
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
        let tree = include_tree(&paths, &[&top, &mid, &leaf], None);
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
            include_tree(&paths, &[&a, &b, &shared], None).edges,
            vec![(0, 2), (1, 2)]
        );
    }

    /// Spellings that differ only in `./` noise or separator name the same file. Matching is
    /// lexical, so it stops exactly there: nothing here resolves a symlink or asks the filesystem.
    #[test]
    fn matching_is_lexical_normalization() {
        assert_eq!(normalize("./oracles/../oracles/h.sh"), "oracles/h.sh");
        assert_eq!(normalize("oracles\\h.sh"), "oracles/h.sh");
        assert_eq!(normalize("../h.sh"), "../h.sh");

        let entry = marked(". ./oracles/h.sh\n");
        let helpers = marked("_h() { :; }\n");
        let paths = vec!["e.sh".to_owned(), "oracles//h.sh".to_owned()];
        assert_eq!(
            include_tree(&paths, &[&entry, &helpers], None).edges,
            vec![(0, 1)]
        );
    }

    /// A target resolves against the SOURCING FILE's directory, not the working directory — the
    /// module doc's flagged divergence, pinned so overturning it is a visible edit rather than a
    /// silent drift. This is what lets an admin name one absolute entrypoint path from anywhere and
    /// still get the package, which is the shape the whole sourcing build exists to deliver.
    #[test]
    fn a_target_resolves_against_its_sourcers_directory() {
        assert_eq!(
            super::resolve_against("/pkg/entry.oracle.sh", "./helpers.sh").as_deref(),
            Some("/pkg/helpers.sh"),
            "an ABSOLUTE sourcer keeps its root — dropping the leading separator turned every \
             POSIX oracle path relative, and only the Linux leg saw it"
        );
        assert_eq!(
            super::resolve_against("/entry.sh", "./helpers.sh").as_deref(),
            Some("/helpers.sh"),
            "...including a sourcer sitting at the root itself"
        );
        assert_eq!(normalize("/mnt/c/pkg/../pkg/h.sh"), "/mnt/c/pkg/h.sh");
        assert_eq!(
            super::resolve_against("pkg/sub/entry.sh", "../shared/h.sh").as_deref(),
            Some("pkg/shared/h.sh")
        );
        assert_eq!(
            super::resolve_against("entry.sh", "./h.sh").as_deref(),
            Some("h.sh"),
            "a bare sourcer name has no directory, so the target stands alone"
        );
        assert_eq!(
            super::resolve_against("pkg/entry.sh", "h.sh"),
            None,
            "slash-less is a PATH search, which this seat may not perform"
        );

        let entry = marked(". ./helpers.sh\n");
        let helpers = marked("_h() { :; }\n");
        let paths = vec![
            "pkg/entry.oracle.sh".to_owned(),
            "pkg/helpers.sh".to_owned(),
        ];
        assert_eq!(
            include_tree(&paths, &[&entry, &helpers], None).edges,
            vec![(0, 1)]
        );

        let elsewhere = vec!["pkg/entry.oracle.sh".to_owned(), "helpers.sh".to_owned()];
        assert_eq!(
            include_tree(&elsewhere, &[&entry, &helpers], None).unresolved,
            [0].into(),
            "a helpers file that is not beside its entrypoint is not the one it named"
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
