//! `snapshot` — the immutable authored input every load answer is a function of
//! (`30I` §3.1 `StaticLoadSnapshot`).
//!
//! One structure carries the source bytes, their identities, the modeled working directory, and
//! which of them load "before line 1". Analysis, custody, emission, and the why driver consume
//! THESE bytes; a local file race between analysis and emission is structurally excluded rather
//! than detected after producing a plan, because there is nothing to re-read.
//!
//! # Pure, with the reading at the edge
//!
//! Building a snapshot opens nothing: the binary's edge acquires `(path, bytes)` pairs and hands
//! them in whole, which is what lets [`WhyWorld`](crate::world::WhyWorld) build the same snapshot
//! from a case's own sections and answer over the run's world rather than a cousin of it
//! (`lib-target-is-a-loom-seam` · `one-definition-table-two-drivers`).
//!
//! # The ambient split is a load-POSITION fact, not a provenance one
//!
//! Sources the invocation NAMED load before the book's first line, so their definitions are
//! ambient. Everything else loads at the `.` that reaches it — a book's, or a named root's own.
//! Making either ambient would let it license sites ABOVE its load point AND replay its program
//! after the authored one finished, restoring definitions the author removed
//! (`30Mc:required-root-occurrence-identity`).
//!
//! # Role is CARRIED, never derived from position
//!
//! A source is a book because it was named as one, not because it sorts last. The ordering that
//! puts the book at the end is real and load-bearing for the `SourceFileId` space
//! (`28K` §2a: CLI files are the ambient prefix, the book's own text executes after), but reading
//! ROLE off that ordering fossilizes "exactly one book, at the end" into every consumer that
//! re-derives it. [`SourceRole`] is on the source, so widening to several independently classified
//! roots is a change to how this vector is BUILT rather than to everything that reads it.
//!
//! **The named cut that remains** (`churn-avoidance-disclosure`): several `--book` operands are
//! still `\n`-CONCATENATED into one text at the edge, and the whole analysis below — one `Ast`,
//! one `Cfg`, one `ValueFlow`, and `Plan::render_apply`'s span edits over one string — assumes
//! that. Undoing it is an arc, not a slice; what this type does is stop the assumption spreading.

use dorc_core::loadpath::Cwd;

/// The role vocabulary now lives in `dorc_core`, because a durable projection asks the same
/// question this snapshot does. Re-exported so every consumer keeps one spelling.
pub use dorc_core::SourceRole;

/// Which acquired sources do NOT load before the book's first line, and why — what
/// [`StaticLoadSnapshot::over`] turns into [`SourceRole`]s.
///
/// One value rather than two `BTreeSet<usize>` parameters side by side: those are swappable without
/// a type error, and the constructor must DEMAND both answers — a dependency silently defaulting to
/// root is `30Mc:finding-transitive-pre-source-replays-as-root` itself.
#[derive(Debug, Default, Clone)]
pub struct LoadPositions {
    book_sourced: std::collections::BTreeSet<usize>,
    dependencies: std::collections::BTreeSet<usize>,
}

impl LoadPositions {
    /// Every named source is a prelude root and nothing loads positionally.
    #[must_use]
    pub fn roots_only() -> Self {
        Self::default()
    }

    /// The sources a book `.` reaches, transitively ([`book_reached`]).
    #[must_use]
    pub fn book_sourced(files: std::collections::BTreeSet<usize>) -> Self {
        Self {
            book_sourced: files,
            ..Self::default()
        }
    }

    /// The sources acquired for a named root's own load program, never named themselves.
    #[must_use]
    pub fn with_dependencies(mut self, files: std::collections::BTreeSet<usize>) -> Self {
        self.dependencies = files;
        self
    }

    /// Book-sourced wins where a source is both: it is the more specific act, and the two agree on
    /// the only question any consumer asks — neither is ambient.
    ///
    /// A book-sourced source splits again on the AUTHOR'S OWN CLAIM: a `# dorc-lang` marker says
    /// "model me", and its absence says this is ordinary sh Dorc acquired to MIRROR
    /// ([`SourceRole::PlainInclusion`]). Derived from the bytes rather than carried as a fourth
    /// index set, because that is one rule with nothing to keep in step — and because the marker is
    /// exactly the claim `marker-gates-syntax-only` already makes the discriminator everywhere else.
    fn role_of(&self, file: usize, src: &str) -> SourceRole {
        if self.book_sourced.contains(&file) {
            if dorc_oracle::marker::has_marker(src) {
                SourceRole::BookSourced
            } else {
                SourceRole::PlainInclusion
            }
        } else if self.dependencies.contains(&file) {
            SourceRole::LoadDependency
        } else {
            SourceRole::NamedLoad
        }
    }
}

/// Every source this run analyses, in load order.
#[derive(Debug, Clone)]
pub struct StaticLoadSnapshot {
    cwd: Cwd,
    /// Paths as the invocation named them — display, diagnostics, and the durable's record of what
    /// was loaded. The CANONICAL form is derived through [`Cwd`] on demand rather than stored, so
    /// there is one spelling of the identity rule and nothing to keep in step with it.
    paths: Vec<String>,
    srcs: Vec<String>,
    /// What each source is, positionally matching the two vectors above.
    ///
    /// A source that is BOTH invocation-named and book-sourced is classified [`BookSourced`], so
    /// it loads positionally rather than ambiently. That is the withholding direction and it is
    /// the disclosed cut: the sharper answer is "named wins", and it needs a boundary a replay can
    /// recover, which today it cannot without the durable learning a field
    /// (`rul-durable-contents-reviewed-before-design`).
    ///
    /// [`BookSourced`]: SourceRole::BookSourced
    roles: Vec<SourceRole>,
}

impl StaticLoadSnapshot {
    /// Build over sources the edge already read, in load order. `positions` names the ones that
    /// load at a `.` rather than before the book's first line; the rest are the invocation's own
    /// prelude roots.
    ///
    /// The book is APPENDED here, and that ordering is load-bearing for the `SourceFileId` space
    /// (`28K` §2a): every id already minted keeps its value, and the book joining the space moves
    /// no existing span. What is NOT load-bearing is reading role off that ordering — each source
    /// carries its own [`SourceRole`], so widening past one book is a change to this constructor
    /// rather than to every consumer.
    #[must_use]
    pub fn over(
        cwd: Cwd,
        oracle_paths: Vec<String>,
        oracle_srcs: Vec<String>,
        positions: &LoadPositions,
        book_path: &str,
        book_src: &str,
    ) -> Self {
        let mut roles: Vec<SourceRole> = (0..oracle_paths.len())
            .map(|file| positions.role_of(file, oracle_srcs.get(file).map_or("", String::as_str)))
            .collect();
        let mut paths = oracle_paths;
        let mut srcs = oracle_srcs;
        paths.push(book_path.to_owned());
        srcs.push(book_src.to_owned());
        roles.push(SourceRole::Book);
        Self {
            cwd,
            paths,
            srcs,
            roles,
        }
    }

    /// The modeled working directory every `.` operand in this unit resolves against.
    #[must_use]
    pub const fn cwd(&self) -> &Cwd {
        &self.cwd
    }

    /// Every source, in load order (`the-book-is-a-definition-source`).
    #[must_use]
    pub fn source_paths(&self) -> &[String] {
        &self.paths
    }

    /// What source `file` is. An index past the end is no source at all, and answering
    /// [`SourceRole::BookSourced`] for it would be the withholding direction by accident rather
    /// than by rule — so this says nothing instead.
    #[must_use]
    pub fn role_of(&self, file: usize) -> Option<SourceRole> {
        self.roles.get(file).copied()
    }

    /// Every source with the role it plays, in load order.
    pub fn sources(&self) -> impl Iterator<Item = (usize, &str, SourceRole)> {
        self.roles
            .iter()
            .enumerate()
            .filter_map(|(file, &role)| Some((file, self.srcs.get(file)?.as_str(), role)))
    }

    /// Every source as a `(path, bytes, role)` triple, in LOAD order.
    ///
    /// The three vectors are built together by [`Self::over`], so this is the seat that owns their
    /// positional agreement. A consumer indexing two of them separately would carry that invariant
    /// wherever it went, and answer with a defaulted empty path where it slipped.
    pub fn source_claims(&self) -> impl Iterator<Item = (&str, &str, SourceRole)> {
        self.paths
            .iter()
            .zip(&self.srcs)
            .zip(&self.roles)
            .map(|((path, src), &role)| (path.as_str(), src.as_str(), role))
    }

    /// Every source's bytes, positionally matching [`Self::source_paths`].
    #[must_use]
    pub fn source_srcs(&self) -> &[String] {
        &self.srcs
    }

    /// The same bytes as `&str` — the REAL text, which is what mirroring, placement, diagnostics,
    /// and the durable read.
    #[must_use]
    pub fn source_refs(&self) -> Vec<&str> {
        self.srcs.iter().map(String::as_str).collect()
    }

    /// Every source's text AS THE ENGINE MODELS IT: a [`SourceRole::PlainInclusion`] reads EMPTY,
    /// because Dorc acquired its bytes to ship and models nothing in it
    /// (`FORFEITS:forfeit-plain-sh-inclusion-analysis`).
    ///
    /// THE ONE SELECTION SEAT for that, and it is positional rather than filtered: the index IS the
    /// `SourceFileId` (`28O:dec-load-order-is-the-id-order`), so dropping an entry would renumber
    /// every source after it and silently re-aim every derived row's identity. An empty file lifts
    /// nothing, declares nothing, and indexes nothing, so every lift seat gets the right answer by
    /// construction rather than by remembering a role check.
    ///
    /// Every LIFT and INDEX seat takes this. [`source_refs`](Self::source_refs) is for the seats
    /// that want the bytes themselves.
    #[must_use]
    pub fn modelled_refs(&self) -> Vec<&str> {
        self.sources()
            .map(|(_, src, role)| if role.is_modelled() { src } else { "" })
            .collect()
    }

    /// The LOADED sources as the engine models them — [`modelled_refs`](Self::modelled_refs)'
    /// book-free prefix, for the oracle-only lanes.
    #[must_use]
    pub fn modelled_oracle_refs(&self) -> Vec<&str> {
        let mut refs = self.modelled_refs();
        refs.truncate(self.book_index());
        refs
    }

    /// The LOADED sources alone — everything that is not a book. The receipt's record of what was
    /// loaded is load-only coherently, and so is `validate`.
    ///
    /// A contiguous slice because the constructor appends the book last, which is what the
    /// `SourceFileId` ordering rests on; the SLICE is a consequence of that ordering, never the
    /// definition of the role.
    #[must_use]
    pub fn oracle_paths(&self) -> &[String] {
        self.paths.get(..self.book_index()).unwrap_or(&[])
    }

    /// The loaded sources' bytes.
    #[must_use]
    pub fn oracle_srcs(&self) -> &[String] {
        self.srcs.get(..self.book_index()).unwrap_or(&[])
    }

    /// The book's index in the source vectors.
    #[must_use]
    pub fn book_index(&self) -> usize {
        self.roles
            .iter()
            .position(|&role| role == SourceRole::Book)
            .unwrap_or_else(|| self.paths.len().saturating_sub(1))
    }

    /// The book's own `SourceFileId`.
    #[must_use]
    pub fn book_file(&self) -> dorc_core::SourceFileId {
        dorc_analysis::funcenv::source_file_of_index(self.book_index())
    }

    /// The book's display path.
    #[must_use]
    pub fn book_path(&self) -> &str {
        self.paths
            .get(self.book_index())
            .map_or("book.sh", String::as_str)
    }

    /// The book's bytes.
    #[must_use]
    pub fn book_src(&self) -> &str {
        self.srcs.get(self.book_index()).map_or("", String::as_str)
    }

    /// Does source `file` load before the book's first line?
    #[must_use]
    pub fn is_ambient(&self, file: usize) -> bool {
        self.role_of(file).is_some_and(SourceRole::is_ambient)
    }

    /// The canonical key source `file` is filed under, when it has one.
    #[must_use]
    pub fn key_of(&self, file: usize) -> Option<String> {
        self.cwd.resolve_operand(self.paths.get(file)?)
    }

    /// Which source is filed under an already-resolved canonical key.
    ///
    /// Bundle projection consumes the loader's resolved target directly; resolving that target
    /// again would create a second loader. Source operands are normalized through the existing
    /// identity seat, and the last match wins exactly as [`crate::world::definition_table`] does.
    #[must_use]
    pub fn source_at_key(&self, key: &str) -> Option<usize> {
        (0..self.paths.len())
            .rev()
            .find(|&file| self.key_of(file).as_deref() == Some(key))
    }

    /// Which loaded source a `.` operand names — the RESOLUTION half of the load answer, with no
    /// contract check and no admission (those are `sourcing`'s and the edge's).
    #[must_use]
    pub fn source_at_dot_target(&self, target: &str) -> Option<usize> {
        let wanted = self.cwd.resolve_dot(target)?;
        self.source_at_key(&wanted)
    }
}

/// Every `.`/`source` operand a book spells, anywhere in its control flow, that resolves to
/// program text.
///
/// A CFG walk rather than a top-level item scan, because a book's loads are ordinary flowing sh
/// and the committed specimens put one inside a subshell. Resolution rides `SourceLiteralPlane`,
/// the same narrow window `funcenv` reads, so the file this names and the file the environment
/// binds are the same file by construction (`funcenv-reads-source-literal-plane-only`).
///
/// The value plane runs on a throwaway interner: this answers which FILES are involved, and
/// interning into the run's own symbol space would reorder every symbol id downstream for no
/// analytic gain.
#[must_use]
pub fn book_load_targets(book_src: &str) -> Vec<String> {
    use dorc_analysis::cfg::CfgNodeKind;

    let mut interner = dorc_core::Interner::default();
    let ast = dorc_syntax::parse(book_src).value;
    let cfg = dorc_analysis::cfg::build(&ast).value;
    let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
    let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
    let mut out = Vec::new();
    for (id, node) in cfg.iter() {
        if node.kind != CfgNodeKind::Command
            || !matches!(plane.literal_text(id, 0), Some("." | "source"))
        {
            continue;
        }
        if let Some(target) = plane.literal_text(id, 1) {
            out.push(target.to_owned());
        }
    }
    out
}

/// Which of the loaded sources a book `.` reaches, transitively — the PURE half of the load
/// acquisition (`30I:rul-books-load-but-do-not-speak`).
///
/// The binary's edge reads files first and then asks this; the in-process why driver holds every
/// source already and asks the same question of the same rule. That is what keeps a replay's
/// partition identical to its original run's without the durable learning a field
/// (`rul-durable-contents-reviewed-before-design`).
///
/// Only a MARKED, contract-satisfying target is reached: a book sourcing ordinary shell is outside
/// the load model entirely, and its site walls exactly as it always has (`30I` §7.2).
#[must_use]
pub fn book_reached(
    cwd: &Cwd,
    paths: &[String],
    srcs: &[String],
    book_src: &str,
) -> std::collections::BTreeSet<usize> {
    let at = |target: &str| -> Option<usize> {
        let wanted = cwd.resolve_dot(target)?;
        paths
            .iter()
            .position(|path| cwd.resolve_operand(path).as_deref() == Some(wanted.as_str()))
            .filter(|&file| {
                // REACHED is a question about the book's own act, not about what the target
                // signed: an ordinary sh file a book `.` names is reached, and MODELLING it is
                // what [`LoadPositions::role_of`] answers separately
                // (`30P:principle-book-code-source-is-inclusion`). What stays out is a file that
                // CLAIMS the dialect and fails its own contract — the acquisition refuses those
                // outright, and treating one as reached here would let the two drivers disagree
                // about a file only one of them holds.
                srcs.get(file).is_some_and(|src| {
                    crate::sourcing::satisfies_the_contract(src)
                        || !dorc_oracle::marker::has_marker(src)
                })
            })
    };
    let mut reached: std::collections::BTreeSet<usize> = book_load_targets(book_src)
        .iter()
        .filter_map(|target| at(target))
        .collect();
    // Transitive: what a book-reached package sources is book-reached too. Terminates because the
    // frontier only ever grows and is bounded by the loaded set.
    let mut frontier: Vec<usize> = reached.iter().copied().collect();
    while let Some(file) = frontier.pop() {
        let Some(src) = srcs.get(file) else { continue };
        for target in crate::sourcing::top_level_load_targets(src) {
            if let Some(next) = at(&target)
                && reached.insert(next)
            {
                frontier.push(next);
            }
        }
    }
    reached
}

#[cfg(test)]
mod tests {
    use super::{Cwd, LoadPositions, SourceRole, StaticLoadSnapshot, book_reached};

    fn snapshot(book_sourced: &[usize]) -> StaticLoadSnapshot {
        positioned(&LoadPositions::book_sourced(
            book_sourced.iter().copied().collect(),
        ))
    }

    fn positioned(positions: &LoadPositions) -> StaticLoadSnapshot {
        StaticLoadSnapshot::over(
            Cwd::at("/ops"),
            vec!["pkg/entry.sh".to_owned(), "pkg/dep.sh".to_owned()],
            vec!["# entry\n".to_owned(), "# dep\n".to_owned()],
            positions,
            "book.sh",
            "# book\n",
        )
    }

    /// The book is the LAST source and the vectors stay positional — the shape every lift seat
    /// zips against, and the one a shorter oracle-only vector silently truncates
    /// (`the-book-is-a-definition-source`).
    #[test]
    fn the_book_is_the_last_source() {
        let snap = snapshot(&[]);
        assert_eq!(snap.book_index(), 2);
        assert_eq!(snap.book_path(), "book.sh");
        assert_eq!(snap.oracle_paths().len(), 2);
        assert_eq!(snap.source_paths().len(), 3);
        assert_eq!(snap.source_refs()[snap.book_index()], "# book\n");
    }

    /// A source a book `.` reaches loads AT that line, so it is not ambient — and the book itself
    /// never is, however the set is spelled: a book that loaded "before line 1" is a
    /// contradiction. Ambience is what decides whether a definition can license sites ABOVE its
    /// own load point (`visibility-is-full-positional`).
    #[test]
    fn only_what_no_book_line_loads_is_ambient() {
        assert!(snapshot(&[]).is_ambient(1));
        assert!(!snapshot(&[1]).is_ambient(1));
        assert!(snapshot(&[1]).is_ambient(0), "the sibling is untouched");
        let snap = snapshot(&[0, 1, 2]);
        assert!(!snap.is_ambient(snap.book_index()));
    }

    /// A source acquired for a named root's load program is NOT a root: it binds where its
    /// sourcer's `.` runs it, and a second ambient run would replay acts the author already wrote
    /// — restoring, among other things, a definition the root `unset -f`'d
    /// (`30Mc:required-root-occurrence-identity`).
    #[test]
    fn a_load_dependency_is_never_a_prelude_root() {
        let snap = positioned(&LoadPositions::roots_only().with_dependencies([1].into()));
        assert!(snap.is_ambient(0), "the invocation named this one");
        assert!(!snap.is_ambient(1));
        assert_eq!(snap.role_of(1), Some(SourceRole::LoadDependency));
    }

    /// Resolution is the shell's, against the snapshot's own cwd, and it matches canonically — so
    /// a relatively-named source and an absolutely-spelled `.` of it are ONE entry.
    #[test]
    fn a_dot_target_resolves_to_one_source() {
        let snap = snapshot(&[]);
        assert_eq!(snap.source_at_dot_target("./pkg/dep.sh"), Some(1));
        assert_eq!(snap.source_at_dot_target("/ops/pkg/dep.sh"), Some(1));
        assert_eq!(
            snap.source_at_dot_target("dep.sh"),
            None,
            "slash-less is a PATH search, outside v0"
        );
        assert_eq!(snap.source_at_dot_target("./pkg/missing.sh"), None);
    }

    #[test]
    fn canonical_key_lookup_matches_the_definition_tables_last_wins_order() {
        let snap = StaticLoadSnapshot::over(
            Cwd::default(),
            vec!["same.sh".to_owned(), "./same.sh".to_owned()],
            vec!["first".to_owned(), "second".to_owned()],
            &LoadPositions::roots_only(),
            "book.sh",
            "",
        );
        assert_eq!(snap.source_at_key("same.sh"), Some(1));
    }

    const MARKER: &str = "# dorc-lang/v0.2\n";

    /// A book's own root variable flows into its `.` operand, the reached package's own `.` is
    /// reached too, and NOTHING that is merely co-loaded is (`30I:force-root-value-flow`). The
    /// engine recognizes no root name — `SM_ORACLE_ROOT` is a spike mnemonic and any ordinary
    /// variable does the same work.
    #[test]
    fn a_books_root_variable_reaches_its_package_transitively() {
        let paths = vec![
            "oracles/alpha.dorc.sh".to_owned(),
            "oracles/common.dorc.sh".to_owned(),
            "oracles/unrelated.dorc.sh".to_owned(),
        ];
        let srcs = vec![
            format!("{MARKER}. ./oracles/common.dorc.sh\nalpha() {{ :; }}\n"),
            format!("{MARKER}common() {{ :; }}\n"),
            format!("{MARKER}unrelated() {{ :; }}\n"),
        ];
        let book = "OPS_LIB=./oracles\n. \"$OPS_LIB/alpha.dorc.sh\"\nalpha\n";
        assert_eq!(
            book_reached(&Cwd::at(""), &paths, &srcs, book),
            [0, 1].into(),
            "the entrypoint AND its own dependency; the co-loaded stranger is not book-reached"
        );
    }

    /// A book sourcing ordinary shell REACHES it and models nothing in it: the reach is a fact
    /// about the book's own act, and the target's silence about the dialect makes it a
    /// [`SourceRole::PlainInclusion`] — acquired for its bytes, its site still walling
    /// (`30P:principle-book-code-source-is-inclusion`). That split is what keeps a book's
    /// non-dorc-lang material where its author put it while still letting the artifact carry it.
    ///
    /// The MARKED-but-not-inert cousin is the control: a file that claims the dialect and fails its
    /// own contract is not reached at all, because the acquisition refuses to hold one.
    #[test]
    fn an_unmarked_target_is_reached_as_an_inclusion() {
        let paths = vec!["child.sh".to_owned()];
        let srcs = vec!["f() { :; }\n".to_owned()];
        assert_eq!(
            book_reached(&Cwd::at(""), &paths, &srcs, ". ./child.sh\n"),
            [0].into(),
            "unmarked ⇒ reached, so the artifact has bytes to mirror"
        );
        let snapshot = StaticLoadSnapshot::over(
            Cwd::at(""),
            paths.clone(),
            srcs.clone(),
            &LoadPositions::book_sourced([0].into()),
            "book.sh",
            ". ./child.sh\n",
        );
        assert_eq!(snapshot.role_of(0), Some(SourceRole::PlainInclusion));
        assert_eq!(
            snapshot.modelled_refs(),
            ["", ". ./child.sh\n"],
            "and it reads EMPTY at every lift and index seat"
        );

        let claiming = vec![format!("{MARKER}f() {{ :; }}\nfalse\n")];
        assert!(
            book_reached(&Cwd::at(""), &paths, &claiming, ". ./child.sh\n").is_empty(),
            "a file that claims the dialect and fails its contract is held by nobody"
        );
    }

    /// A load inside a subshell is still a load — the walk is over the whole CFG, not the top
    /// level, because the committed specimens spell one exactly there.
    #[test]
    fn a_regional_load_is_reached_too() {
        let paths = vec!["fallback.dorc.sh".to_owned()];
        let srcs = vec![format!("{MARKER}pick() {{ :; }}\n")];
        assert_eq!(
            book_reached(
                &Cwd::at(""),
                &paths,
                &srcs,
                "(\n   . ./fallback.dorc.sh\n)\n"
            ),
            [0].into()
        );
    }
}
