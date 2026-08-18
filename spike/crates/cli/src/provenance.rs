//! `provenance` — locators built from what the run's own loader did (`30I` §9).
//!
//! The representation is `aid::locator`'s arbitrary DAG; this is the seat that FILLS it from the
//! load model, so a consumer asking "where did this come from" is answered by the same structure
//! the engine loaded through rather than by a second account of it.
//!
//! # What this can say today, and what it cannot
//!
//! Two stages: the authored bytes, and the `.` act that brought their file into the unit. That is
//! the whole chain the tree currently has — nothing generates artifacts with copied ranges yet, so
//! the `Copied`/`Generated`/`Claimed` stages the representation carries have no producer here.
//! When the bundle projection lands, its segments compose ONTO these edges rather than replacing
//! them (`30I` §9.2 item 6), which is exactly why the representation is a DAG before the compiler
//! that needs one exists.
//!
//! **Disclosed cut** (`churn-avoidance-disclosure`): only BOOK-level load acts are recorded. A
//! package's own nested `.` is followed by the loader but not yet reported as an edge, so a
//! dependency-of-a-dependency resolves to two stages rather than three. Closing it means the
//! interpreter reporting the edges it walked; `30Ib` §5 carries it.

use dorc_aid::locator::{Locator, SourceLocus, Stage, StageId};
use dorc_analysis::cfg::Cfg;
use dorc_analysis::funcenv::FuncEnv;
use dorc_core::{SourceFileId, Span};

use crate::snapshot::StaticLoadSnapshot;

/// The load ACTS a book performed, by the source each brought in.
///
/// Built once per run from the settled environment. A file the invocation named directly appears
/// in no entry: nothing loaded it, it was simply there before line 1, and saying otherwise would
/// invent a line for a reader to go and look at.
#[derive(Debug, Clone, Default)]
pub struct LoadActs {
    /// Loaded source index → the span of the `.` item that named it, in the BOOK.
    by_source: std::collections::BTreeMap<usize, Span>,
}

impl LoadActs {
    /// Read the acts off a settled environment.
    #[must_use]
    pub fn of(
        snapshot: &StaticLoadSnapshot,
        cfg: &Cfg,
        book: &dorc_syntax::Ast,
        env: &FuncEnv,
    ) -> Self {
        let mut by_source = std::collections::BTreeMap::new();
        for (&node, key) in env.sourced_paths() {
            let Some(file) = (0..snapshot.source_paths().len())
                .find(|&file| snapshot.key_of(file).as_deref() == Some(key.as_str()))
            else {
                continue;
            };
            // FIRST act wins: a file loaded at two positions has two lines a reader could go to,
            // and picking the earliest is the one choice that does not depend on map order.
            let span = book.node(cfg.node(node).ast).span;
            by_source
                .entry(file)
                .and_modify(|first: &mut Span| {
                    if span.lo.0 < first.lo.0 {
                        *first = span;
                    }
                })
                .or_insert(span);
        }
        Self { by_source }
    }

    /// The locator for a range of one loaded source: the authored bytes, and the load act that
    /// brought their file in when there was one.
    ///
    /// The head is the OUTERMOST stage — the act, where there is one — so a consumer rendering the
    /// chain in order shows the reader the line they would edit to change WHICH file is involved
    /// before the bytes that say what it does.
    #[must_use]
    pub fn locator_for(
        &self,
        snapshot: &StaticLoadSnapshot,
        at: SourceLocus,
    ) -> (Locator, StageId) {
        let mut locator = Locator::default();
        let authored = locator.push(Stage::Authored(at), &[]);
        let file = at.file.0 as usize;
        let Some(&span) = self.by_source.get(&file) else {
            return (locator, authored);
        };
        let book = SourceFileId(u32::try_from(snapshot.book_index()).unwrap_or(u32::MAX));
        let act = locator.push(Stage::Loaded(SourceLocus::at(book, span)), &[authored]);
        (locator, act)
    }
}

/// Render one locator chain as `path:line` loci, outermost stage first.
///
/// Generated stages render by their artifact label and a claimed one by its own text, because a
/// generated artifact has no loaded-source path and a claim has no verified anything — saying
/// otherwise would be the conversion `rul-bundle-origin-is-aid-only` forbids.
#[must_use]
pub fn render_chain(
    locator: &Locator,
    head: StageId,
    snapshot: &StaticLoadSnapshot,
) -> Vec<String> {
    let source_locus = |locus: &SourceLocus| {
        let file = locus.file.0 as usize;
        let path = snapshot.source_paths().get(file)?;
        let src = snapshot.source_srcs().get(file)?;
        let (line, _) = dorc_aid::diag::line_col(src, locus.span.lo.0 as usize);
        Some(format!("{path}:{line}"))
    };
    locator
        .resolve(head)
        .into_iter()
        .filter_map(|stage| match stage {
            Stage::Authored(locus) | Stage::Loaded(locus) => source_locus(locus),
            Stage::Copied(at) | Stage::Generated(at) => Some(at.artifact.clone()),
            Stage::Claimed(claim) => Some(claim.as_claimed().to_owned()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use dorc_aid::locator::SourceLocus;
    use dorc_core::{Interner, SourceFileId};

    use super::{LoadActs, render_chain};
    use crate::snapshot::StaticLoadSnapshot;

    const MARKER: &str = "# dorc-lang/v0.2\n";

    /// Analyse a hand-built world exactly as a run does, and hand back what a locator needs.
    fn world(book: &str, paths: Vec<String>, srcs: Vec<String>) -> (StaticLoadSnapshot, LoadActs) {
        let snapshot = StaticLoadSnapshot::over(
            dorc_core::loadpath::Cwd::default(),
            paths,
            srcs,
            [].into(),
            "book.sh",
            book,
        );
        let mut interner = Interner::default();
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        let acts = LoadActs::of(&snapshot, &cfg, &ast, &env);
        (snapshot, acts)
    }

    /// THE CHAIN, FROM A REAL RUN (`30I` §9.1): a definition in a package, and the book line that
    /// brought that package in. Two stages, resolved from the environment the loader actually
    /// settled on rather than from a hand-built graph.
    ///
    /// What it forces: the outermost stage is the LOAD ACT, so a reader is shown the line they
    /// would edit to change WHICH file answers before the bytes that say what it does. A
    /// `(generated, original)` pair could carry this pair and not the next one; the DAG carries
    /// both, which is why it is the representation before the compiler that needs it exists.
    #[test]
    fn a_definition_in_a_loaded_package_resolves_through_its_load_act() {
        let package = format!("{MARKER}\nsm_q() {{ common \"$@\" ;}}\n");
        let (snapshot, acts) = world(
            "OPS_LIB=.\n. \"$OPS_LIB/pkg.sh\"\nsm_q first\n",
            vec!["pkg.sh".to_owned()],
            vec![package.clone()],
        );
        let body = package.find("sm_q()").expect("the package declares it");
        let at = SourceLocus::at(
            SourceFileId(0),
            dorc_core::Span::new(
                dorc_core::BytePos(u32::try_from(body).unwrap_or(0)),
                dorc_core::BytePos(u32::try_from(body).unwrap_or(0) + 4),
            ),
        );
        let (locator, head) = acts.locator_for(&snapshot, at);
        assert_eq!(
            render_chain(&locator, head, &snapshot),
            vec!["book.sh:2".to_owned(), "pkg.sh:3".to_owned()],
            "the book's `.` line first, the authored bytes it named behind it"
        );
    }

    /// A source the invocation NAMED has no load act, and the chain says so by being one stage
    /// rather than by pointing at a line nobody wrote. Inventing an act for an ambient oracle
    /// would send a reader to a line that does not exist.
    #[test]
    fn an_ambient_oracle_resolves_to_its_own_bytes_alone() {
        let (snapshot, acts) = world(
            "sm_q first\n",
            vec!["pkg.sh".to_owned()],
            vec![format!("{MARKER}sm_q() {{ common \"$@\" ;}}\n")],
        );
        let at = SourceLocus::at(
            SourceFileId(0),
            dorc_core::Span::new(dorc_core::BytePos(18), dorc_core::BytePos(22)),
        );
        let (locator, head) = acts.locator_for(&snapshot, at);
        assert_eq!(render_chain(&locator, head, &snapshot).len(), 1);
    }
}
