//! The three artifact FORMS, and the selector that chooses between them (`30I` §7.1
//! `rul-plan-emission-has-three-resting-forms`).
//!
//! # One structure, several forms
//!
//! The executable product is the Plan projection PLUS its generated files, never ad-hoc writes
//! beside a string-only plan (`30I:step-7-reify-plan-artifact-forms`). Everything below derives
//! from one already-settled [`Plan`](dorc_plan::Plan) and one already-resolved
//! [`BundleProjection`](crate::bundle::BundleProjection): this module resolves nothing, reads no
//! file, and decides nothing the plan already decided. A form READS the decision plane
//! (`plan/CLAUDE.md the-render-decides-nothing`) and does its own typesetting.
//!
//! # Why the dependency layout is the authored one
//!
//! `30I` §7.4 asks the planner to spend cwd analysis BEFORE scaffolding: emit a simple relative
//! dependency path where one suffices, and reach for a captured artifact root only when it does
//! not. A book's own `. "$ROOT/pkg/entry.oracle.sh"` already names a path RELATIVE to the load
//! cwd, and the artifact's execution begins in the artifact directory (§7.6), so MIRRORING that
//! relative layout under the artifact root makes every authored operand — the book's and every
//! nested one inside a copied file — resolve exactly as it did controller-side, with no rewritten
//! operand, no generated root variable, and no book byte moved. That is the cheapest correct
//! placement, and the honest availability question becomes a path question: a controller path
//! outside the load working directory cannot be mirrored, and the form is unavailable rather than
//! fudged (`need-controller-paths-never-cross-hosts`).
//!
//! The mirroring is therefore stated AGAINST THE LOAD CWD and never against a path's own spelling.
//! Every source a book `.` reaches is filed under its CANONICAL key, which is absolute whenever the
//! edge could answer where the run stands — so a seat that asked whether the stored spelling looked
//! relative would answer "unplaceable" for every real invocation while every in-process test, whose
//! modelled cwd is the flat virtual one, said otherwise. `Cwd::relativize` is the one rule both
//! worlds go through.
//!
//! # Why flattening REFUSES rather than inlining
//!
//! Textual inlining of a load-inert child at its `.` position is ARGUED sound and NOT MEASURED
//! (`30Ib` §5 row 8): a dot boundary is errexit-catching in a way pasted bytes are not, and
//! `fnd-loader-function-errexit-diverges` already refuted the obvious alternative. `30I` §7.1
//! sanctions refusing explicit single-stream intent the compiler cannot yet satisfy safely, so a
//! book with a dependency to inline makes the flattened form UNAVAILABLE until the floor cell is
//! minted. A book with nothing to inline is already one stream, and the flattened form is
//! available and byte-identical there — which is why the whole existing corpus is unaffected.

use dorc_core::loadpath::Cwd;
use dorc_core::{AstId, Span};
use dorc_plan::{
    EmittedName, ImportEdit, LoadSite, PlacedSources, Placement, PlacementDecision, PlacementReason,
};

use crate::bundle::{BundleProjection, BundleRootId};

/// The maximum dependency files one artifact set will place.
///
/// Bounds what an authored load graph can ask the edge to write, on
/// `rul-host-bytes-bounded-before-admission`'s reasoning one layer over: the inputs are the
/// operator's own, but a publication is still a write loop and a bound that exists is cheaper than
/// a bound that is argued.
const MAX_DEPENDENCIES: usize = 256;

/// Which semantic emission form an artifact set is in (`30I` §7.1, as widened by
/// `30Ng:rul-bundle-at-dorc-lang-boundaries`).
///
/// Ordered by FLATTENING, most first, because that is the order `auto` searches. The BUNDLE-POINT
/// axis is what the order is over, and both of its ends stay reachable by name: [`Flattened`] is one
/// emission and [`MirroredTree`] is none at all, with the default sitting between them
/// (`30Ng` §5, human-typed: both extremes fully supported, the default at neither).
///
/// Every spelling here is STRAWMAN and renames in place (`rul-strawman-formats-no-compat`); what is
/// ruled is the axis and its ends, never the words.
///
/// [`Flattened`]: Self::Flattened
/// [`MirroredTree`]: Self::MirroredTree
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactForm {
    /// One `plan.sh` and nothing else: every book-reached dorc-lang subgraph stands in the stream
    /// where its `.` stood. What a kept stdout takes, and what a byte-pipe transport needs.
    Flattened,
    /// `plan.sh` plus ONE bundle per book-sited dorc-lang root, with the plan's own imports naming
    /// them. The intended attention-preserving default.
    Multipart,
    /// `plan.sh` plus EVERY reached source, each mirrored at its own authored relative path, and no
    /// import re-said. The no-flatten end of the axis: the artifact is the author's own file tree,
    /// so an admin who wants to read the dependencies as their authors wrote them can ask for that
    /// and get exactly it.
    MirroredTree,
    /// The authored source boundaries survive untouched and the artifact set carries no
    /// dependencies: v0 could neither absorb them nor place them, so it miscompiles nothing and
    /// says so.
    PreservedBookTree,
}

impl ArtifactForm {
    /// The greppable name a disclosure prints — deliberately the variant's own word rather than
    /// prose, on `CollapseKind::class_name`'s footing.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Flattened => "flattened",
            Self::Multipart => "multipart",
            Self::MirroredTree => "mirrored-tree",
            Self::PreservedBookTree => "preserved-book-tree",
        }
    }
}

/// What the invocation asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormRequest {
    /// No form named: the selector chooses the most flattened SAFE one for the posture.
    Auto,
    /// A named form. Unavailable ⇒ a pre-network refusal, never a different form
    /// (`30I` §7.1's standing rule; §14 keeps that out of builder latitude).
    Explicit(ArtifactForm),
}

/// Whether a person is watching this run's stdout — the injected, non-hermetic EDGE fact
/// (`30Ng:rul-piped-stdout-carries-a-full-plan`, human-typed).
///
/// Injected rather than probed inward, so both cells are drivable deterministically and the kernel
/// stays a pure function of its inputs (`inv-determinism`). The real edge asks the terminal; every
/// test says which cell it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdoutPosture {
    /// A person is reading it: stdout carries the plan RENDER, and the artifact may live elsewhere.
    Interactive,
    /// It is going somewhere — a pager, an editor, a file kept for review. The user asked for a
    /// plan and for a meaningful output stream, so stdout carries the ARTIFACT, complete.
    NonInteractive,
}

/// Where this invocation's ARTIFACT goes, and what that obliges
/// (`30Ng:rul-piped-stdout-carries-a-full-plan`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamPosture {
    /// stdout IS the artifact and it is being kept: one stream, and a COMPLETE plan on it or a
    /// pre-network refusal. What the reader approves is exactly what executes, so an artifact that
    /// cannot run where it lands is not a smaller answer — it is the wrong one.
    PipedArtifact,
    /// stdout carries the plan render for a person at a terminal. `auto` may settle for a less
    /// flattened form and say so, because nothing here is being kept to run later.
    TerminalRender,
    /// The artifact stream is a directory the run may materialize.
    Materializable,
}

/// Which stream carries this run's artifact, or the pre-network refusal that says two things claimed
/// it (`30Ng` §4; `30I` §2.5's collapsed-resource rule, applied to the artifact itself).
///
/// The claimants are the point. A non-interactive stdout is an IMPLICIT claim — the user asked for a
/// plan and for a stream worth keeping — and `--artifact-dir` is an explicit one. Two claimants on
/// one collapsed resource refuse before network and name both, rather than a silent precedence rule
/// deciding which of two competing complete artifacts the user meant.
///
/// # Errors
/// Refuses when stdout and a named artifact directory both claim the artifact.
pub const fn artifact_stream(
    stdout: StdoutPosture,
    artifact_dir: bool,
) -> Result<StreamPosture, FormRefusal> {
    match (stdout, artifact_dir) {
        (StdoutPosture::NonInteractive, true) => Err(FormRefusal::TwoArtifactClaimants),
        (StdoutPosture::NonInteractive, false) => Ok(StreamPosture::PipedArtifact),
        (StdoutPosture::Interactive, true) => Ok(StreamPosture::Materializable),
        (StdoutPosture::Interactive, false) => Ok(StreamPosture::TerminalRender),
    }
}

/// Why `auto` settled for a less flattened form than it aimed at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormFallback {
    /// The book loads dependencies that would have to be inlined, and that inlining is not yet
    /// floor-measured.
    InliningUnproven {
        /// How many book-sited load occurrences would need inlining.
        loads: usize,
    },
    /// A dependency's authored path cannot be mirrored under an artifact root.
    DependencyUnplaceable {
        /// How many dependencies could not be placed.
        loads: usize,
    },
}

impl FormFallback {
    /// The greppable cause word.
    #[must_use]
    pub const fn cause(self) -> &'static str {
        match self {
            Self::InliningUnproven { .. } => "inlining-unproven",
            Self::DependencyUnplaceable { .. } => "dependency-unplaceable",
        }
    }

    /// How many load occurrences the cause counted.
    #[must_use]
    pub const fn loads(self) -> usize {
        match self {
            Self::InliningUnproven { loads } | Self::DependencyUnplaceable { loads } => loads,
        }
    }
}

/// Why this run can produce no artifact of the shape it was asked for — always pre-network, never a
/// silent swap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormRefusal {
    /// The named form is not available for this book.
    Unavailable {
        /// The form the invocation named.
        form: ArtifactForm,
        /// What blocked it.
        because: FormFallback,
    },
    /// A multipart artifact was requested with nowhere to put it: one stream cannot carry a
    /// dependency tree distinguishably (`30I` §2.5's collapsed-resource rule).
    NoArtifactStream {
        /// The form the invocation named.
        form: ArtifactForm,
    },
    /// A kept stdout stream and a named artifact directory both claim this run's ONE artifact
    /// (`30Ng` §4). Neither is wrong on its own; together they ask for two competing complete
    /// artifacts, and ranking them silently is what the collapsed-resource rule forbids.
    TwoArtifactClaimants,
    /// stdout is being kept and no complete plan can be put on it: the book loads dorc-lang the
    /// stream cannot carry in place. Distinct from [`Unavailable`](Self::Unavailable) because the
    /// invocation named no form — what it asked for was a plan worth keeping
    /// (`30Ng:rul-piped-stdout-carries-a-full-plan`).
    IncompleteSingleStream {
        /// How many book-sited loads could not be carried in the stream.
        loads: usize,
    },
}

impl FormRefusal {
    /// The form word the invocation asked for, or `auto` where it named none.
    #[must_use]
    pub const fn form(self) -> &'static str {
        match self {
            Self::Unavailable { form, .. } | Self::NoArtifactStream { form } => form.name(),
            Self::TwoArtifactClaimants | Self::IncompleteSingleStream { .. } => "auto",
        }
    }

    /// The greppable cause word.
    #[must_use]
    pub const fn cause(self) -> &'static str {
        match self {
            Self::Unavailable { because, .. } => because.cause(),
            Self::NoArtifactStream { .. } => "no-artifact-stream",
            Self::TwoArtifactClaimants => "two-artifact-claimants",
            Self::IncompleteSingleStream { .. } => "incomplete-single-stream",
        }
    }

    /// How many book-sited load occurrences the cause counted; zero where the cause counts none.
    #[must_use]
    pub const fn loads(self) -> usize {
        match self {
            Self::Unavailable { because, .. } => because.loads(),
            Self::IncompleteSingleStream { loads } => loads,
            Self::NoArtifactStream { .. } | Self::TwoArtifactClaimants => 0,
        }
    }
}

/// One file the artifact set publishes, at a path relative to the artifact root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactFile {
    /// Relative, separator-normalised, traversal-free by construction ([`placeable`]).
    pub path: String,
    /// Exact bytes.
    pub bytes: String,
}

/// The published product: one plan projection and the generated files it needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSet {
    form: ArtifactForm,
    fallback: Option<FormFallback>,
    primary: ArtifactFile,
    dependencies: Vec<ArtifactFile>,
}

impl ArtifactSet {
    /// Which form this set is in.
    #[must_use]
    pub const fn form(&self) -> ArtifactForm {
        self.form
    }

    /// Why `auto` did not reach a more flattened form, where it did not.
    #[must_use]
    pub const fn fallback(&self) -> Option<FormFallback> {
        self.fallback
    }

    /// The plan projection itself — the bytes every form puts on the artifact stream.
    #[must_use]
    pub const fn primary(&self) -> &ArtifactFile {
        &self.primary
    }

    /// The generated dependencies, in a deterministic order. EMPTY for every form but
    /// [`ArtifactForm::Multipart`].
    #[must_use]
    pub fn dependencies(&self) -> &[ArtifactFile] {
        &self.dependencies
    }

    /// Every file to publish, primary first.
    pub fn files(&self) -> impl Iterator<Item = &ArtifactFile> {
        std::iter::once(&self.primary).chain(self.dependencies.iter())
    }
}

/// One book-sited load the emission planner must account for.
///
/// ROOT occurrences only: a nested `.` inside a copied dependency is already inside that
/// dependency's bytes and is handled by the bundle, not by the planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookLoad {
    /// The `.` command's own node in the book.
    pub command: AstId,
    /// The `.` command's span in the book's own bytes.
    pub span: Span,
    /// The operand WORD's node, where the command has one. Absent means there is nothing to
    /// re-point, so no form that depends on re-pointing is available for this load.
    pub operand: Option<AstId>,
    /// The root bundle the occurrence opened.
    pub root: BundleRootId,
    /// Is this `.` a top-level simple command standing alone on its own line, with neither a
    /// redirect nor a leading assignment — the ONE shape `floor30-inline-dot-boundary` measured
    /// a bundle may stand in for?
    ///
    /// The assignment clause is load-bearing: an `Inline` replaces the WHOLE command node, so
    /// `MODE=prod . ./entry.oracle.sh` would lose `MODE=prod` and the absorbed bytes would read a
    /// different environment than the `.` they stand for. `Repoint` moves only the operand and
    /// stays eligible for that shape.
    pub absorbable: bool,
    /// Does the operand name its target EXPLICITLY — the precondition every rewrite of this line
    /// derives from (`30P:rul-rewrite-permission-is-derived`, human-typed)?
    ///
    /// Permission to re-point, inline or hoist an import comes from the AUTHOR having written what
    /// the line loads, never from the load plane's EXACT-ness, which answers a different question
    /// (can the controller say which file this resolves to). A computed operand stays verbatim in
    /// every form; where a form places files at all its target is mirrored at the authored relative
    /// path, and the author's own line finds it there.
    pub explicit: bool,
}

/// Does this operand name its target explicitly (`30P:rul-rewrite-permission-is-derived`)?
///
/// The ruling's two admitted spellings are a LITERAL word and a literal-assigned BOOK-SET ROOT
/// (`30I` §2.1) — the author writing what the line loads, in one piece or in two. A simple `$name`
/// therefore passes: the load plane admits a word only where its value is program text
/// (`funcenv-reads-source-literal-plane-only`), so an operand that resolved through a `$name` at all
/// resolved through a literal the book itself assigns.
///
/// Refused, and each for its own reason: an operator-bearing expansion and a command substitution
/// are what a `$0`-relative operand is spelled with, arithmetic is dynamic, and `$0` itself is
/// SELF-LOCATION rather than a named target — the engine knowing which file it resolves to is the
/// load plane's EXACT-ness, which this predicate exists to keep from becoming permission.
///
/// SEAM: this reads the shape off the AST word. The load plane is landing an evaluator under which
/// a `${0%/*}`-headed operand resolves EXACT, and the typed explicitness marker it carries on that
/// resolution replaces this seat at that fold — the predicate stays, its answer moves.
fn operand_is_explicit(book: &dorc_syntax::Ast, operand: AstId) -> bool {
    use dorc_syntax::ast::WordPart;
    fn parts_are_explicit(parts: &[WordPart]) -> bool {
        parts.iter().all(|part| match part {
            WordPart::Literal(_) | WordPart::SingleQuoted(_) => true,
            WordPart::DoubleQuoted(inner) => parts_are_explicit(inner),
            WordPart::Param { name } => name != "0",
            WordPart::CommandSubst(_) | WordPart::Arithmetic | WordPart::ParamComplex { .. } => {
                false
            }
        })
    }
    match &book.node(operand).kind {
        dorc_syntax::ast::NodeKind::Word { parts } => parts_are_explicit(parts),
        _ => false,
    }
}

/// The book's own root load occurrences, paired with the bundle roots they opened.
///
/// A `--pre-source` root is deliberately absent: it is not in the book's bytes, so no artifact
/// form has anything to place for it — the guard preamble already carries whatever of it the
/// artifact needs (`pinned-definitions-are-the-artifact's-binding`).
#[must_use]
pub fn book_loads(
    cfg: &dorc_analysis::cfg::Cfg,
    book: &dorc_syntax::Ast,
    book_src: &str,
    projection: &BundleProjection,
) -> Vec<BookLoad> {
    use dorc_syntax::ast::NodeKind;

    let top_level: std::collections::BTreeSet<AstId> = match &book.node(book.root()).kind {
        NodeKind::Script { items } => items
            .iter()
            .copied()
            .filter(|&id| matches!(book.node(id).kind, NodeKind::Simple { .. }))
            .collect(),
        _ => std::collections::BTreeSet::new(),
    };
    projection
        .occurrences()
        .iter()
        .filter(|occurrence| {
            occurrence.load().within.is_none()
                && matches!(
                    occurrence.load().sourcer,
                    dorc_analysis::load::LoadSourcer::Book
                )
        })
        .map(|occurrence| {
            let command = cfg.node(occurrence.load().at).ast;
            let span = book.node(command).span;
            let (operand, bare) = match &book.node(command).kind {
                NodeKind::Simple {
                    assigns,
                    words,
                    redirs,
                } => (
                    words.get(1).copied(),
                    redirs.is_empty() && assigns.is_empty(),
                ),
                _ => (None, false),
            };
            BookLoad {
                command,
                span,
                operand,
                root: occurrence.root(),
                absorbable: top_level.contains(&command) && bare && alone_on_line(book_src, span),
                explicit: operand.is_some_and(|word| operand_is_explicit(book, word)),
            }
        })
        .collect()
}

/// Is this span the whole of its own line, bar whitespace and a trailing comment?
///
/// The same question `plan`'s commented-original render asks of an elided leaf, asked here for the
/// same reason: bytes standing in for a command that shares its line either swallow a sibling
/// statement or land inside one.
fn alone_on_line(src: &str, span: Span) -> bool {
    let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
    let start = src
        .get(..lo)
        .and_then(|s| s.rfind('\n'))
        .map_or(0, |i| i.saturating_add(1));
    let end = src
        .get(hi..)
        .and_then(|s| s.find('\n'))
        .map_or(src.len(), |i| hi.saturating_add(i));
    let leading = src.get(start..lo).unwrap_or("").trim();
    let trailing = src.get(hi..end).unwrap_or("").trim();
    leading.is_empty() && (trailing.is_empty() || trailing.starts_with('#'))
}

/// The relative destination a controller-side path mirrors to, or `None` when it cannot be
/// mirrored.
///
/// Refused: an absolute path, a Windows drive-qualified path, an empty one, and anything whose
/// normalised form still leaves the artifact root. `.` segments collapse; `..` is never resolved
/// against the preceding segment, because doing so would let `a/../../x` normalise into an
/// escape that looks clean.
#[must_use]
pub fn placeable(authored: &str) -> Option<String> {
    let normalised = authored.replace('\\', "/");
    if normalised.is_empty()
        || normalised.starts_with('/')
        || normalised.chars().nth(1) == Some(':')
    {
        return None;
    }
    let mut parts: Vec<&str> = Vec::new();
    for part in normalised.split('/') {
        match part {
            "" | "." => {}
            ".." => return None,
            other => parts.push(other),
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

/// Where a dependency the run resolved to `authored` is MIRRORED under the artifact root, or `None`
/// when it cannot be placed there.
///
/// Two refusals compose, and they refuse different things: [`Cwd::relativize`] refuses a path that
/// stands OUTSIDE the load working directory (there is no relative spelling of it to mirror), and
/// [`placeable`] refuses a shape that could not be a destination under a root. The second is
/// belt-and-braces over the first — `relativize` already yields a normalized relative path — and
/// refusing twice costs nothing on a write path.
#[must_use]
pub fn mirrored(cwd: &Cwd, authored: &str) -> Option<String> {
    placeable(&cwd.relativize(authored)?)
}

/// The artifact-relative name a bundle takes: the entry's own mirrored spelling, with its `.sh`
/// suffix replaced by `.dorc-bundle.sh`.
///
/// STRAWMAN, and renameable in place under `rul-strawman-formats-no-compat`. What the name has to do
/// is not be the authored path: the file is GENERATED — one author's dorc-lang subgraph, stripped and
/// composed — and publishing it under the authored spelling would put bytes on the target under a
/// name that promises to be somebody's file. A `.dorc` segment ahead of `.sh` is dropped so the
/// common `alpha.dorc.sh` does not become `alpha.dorc.dorc-bundle.sh`.
#[must_use]
pub fn bundle_name(mirrored_path: &str) -> String {
    let stem = mirrored_path.strip_suffix(".sh").unwrap_or(mirrored_path);
    let stem = stem.strip_suffix(".dorc").unwrap_or(stem);
    format!("{stem}.dorc-bundle.sh")
}

/// What a multipart set would place, and which import each book load then names — or the count that
/// made it impossible.
///
/// Each book-sited root becomes ONE bundle at the dep-graph point where the book's dependencies
/// become dorc-lang (`30Ng:rul-bundle-at-dorc-lang-boundaries`), plus a mirrored file for every
/// nested load the bundle could not absorb, whose authored `.` still names it.
///
/// Deduplicated by DESTINATION: two load points naming one entrypoint compose the same bytes, and
/// both imports may name the one file. Two DIFFERENT byte-sets claiming one destination is
/// unplaceable rather than last-wins — an artifact whose dependency depends on which occurrence was
/// walked last is not a projection of anything.
fn bundle_files(
    cwd: &Cwd,
    snapshot_paths: &[String],
    projection: &BundleProjection,
    loads: &[BookLoad],
) -> Result<(Vec<ArtifactFile>, Vec<ImportEdit>), usize> {
    let mut placed: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut imports: Vec<ImportEdit> = Vec::new();
    let mut unplaceable = 0_usize;
    let mut place = |destination: String, bytes: String, unplaceable: &mut usize| match placed
        .get(&destination)
    {
        Some(existing) if *existing != bytes => *unplaceable = unplaceable.saturating_add(1),
        Some(_) => {}
        None => drop(placed.insert(destination, bytes)),
    };
    let authored_of = |file: &crate::bundle::BundleFile| {
        snapshot_paths
            .get(file.copied().source().0 as usize)
            .map_or("", String::as_str)
    };
    for load in loads {
        // Unplaceable, never silently skipped: omitting a file the runtime `.` will look for is
        // what the possible-load projection exists to prevent (`30I` §6.1).
        let Some(root) = projection
            .roots()
            .iter()
            .find(|root| root.id() == load.root)
        else {
            unplaceable = unplaceable.saturating_add(1);
            continue;
        };
        // A computed operand is not ours to re-say (`30P:rul-rewrite-permission-is-derived`): the
        // line stays verbatim, so every file under it is mirrored at the authored relative path the
        // author's own operand will resolve to.
        if !load.explicit {
            for file in root.files().iter().filter_map(|&id| projection.file(id)) {
                match mirrored(cwd, authored_of(file)) {
                    Some(beside) => {
                        place(beside, file.copied().text().to_owned(), &mut unplaceable)
                    }
                    None => unplaceable = unplaceable.saturating_add(1),
                }
            }
            continue;
        }
        let Some((entry, operand)) = projection.file(root.entry()).zip(load.operand) else {
            unplaceable = unplaceable.saturating_add(1);
            continue;
        };
        let Some(destination) = mirrored(cwd, authored_of(entry)).map(|path| bundle_name(&path))
        else {
            unplaceable = unplaceable.saturating_add(1);
            continue;
        };
        place(
            destination.clone(),
            root.bundled().to_owned(),
            &mut unplaceable,
        );
        imports.push(ImportEdit::Repoint {
            ast: operand,
            path: format!("./{destination}"),
        });
        for &id in root.separate() {
            let Some(file) = projection.file(id) else {
                continue;
            };
            let Some(beside) = mirrored(cwd, authored_of(file)) else {
                unplaceable = unplaceable.saturating_add(1);
                continue;
            };
            place(beside, file.copied().text().to_owned(), &mut unplaceable);
        }
    }
    if unplaceable > 0 || placed.len() > MAX_DEPENDENCIES {
        return Err(unplaceable.max(placed.len().saturating_sub(MAX_DEPENDENCIES)));
    }
    Ok((
        placed
            .into_iter()
            .map(|(path, bytes)| ArtifactFile { path, bytes })
            .collect(),
        imports,
    ))
}

/// Every reached source at its OWN authored relative path — the no-flatten end of the bundle-point
/// axis (`30Ng` §5), and the placement machinery this arc inherited, kept reachable by name.
///
/// No import is re-said, because none has to be: a file mirrored at the spelling its sourcer used
/// resolves on the target exactly as it did controller-side, which is the cwd-analysis answer `30I`
/// §7.4 asks for. That is the whole difference from the default — the artifact is the author's tree
/// rather than the engine's composition of it.
fn mirrored_files(
    cwd: &Cwd,
    snapshot_paths: &[String],
    projection: &BundleProjection,
    loads: &[BookLoad],
) -> Result<Vec<ArtifactFile>, usize> {
    let wanted: std::collections::BTreeSet<BundleRootId> =
        loads.iter().map(|load| load.root).collect();
    let mut placed: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut unplaceable = 0_usize;
    for id in &wanted {
        let Some(root) = projection.roots().iter().find(|root| root.id() == *id) else {
            unplaceable = unplaceable.saturating_add(1);
            continue;
        };
        for &file in root.files() {
            let Some(file) = projection.file(file) else {
                continue;
            };
            let authored = snapshot_paths
                .get(file.copied().source().0 as usize)
                .map_or("", String::as_str);
            let (Some(destination), bytes) = (mirrored(cwd, authored), file.copied().text()) else {
                unplaceable = unplaceable.saturating_add(1);
                continue;
            };
            match placed.get(&destination) {
                Some(existing) if existing != bytes => {
                    unplaceable = unplaceable.saturating_add(1);
                }
                Some(_) => {}
                None => drop(placed.insert(destination, bytes.to_owned())),
            }
        }
    }
    if unplaceable > 0 || placed.len() > MAX_DEPENDENCIES {
        return Err(unplaceable.max(placed.len().saturating_sub(MAX_DEPENDENCIES)));
    }
    Ok(placed
        .into_iter()
        .map(|(path, bytes)| ArtifactFile { path, bytes })
        .collect())
}

/// The in-place substitutions a single-stream set needs, or `None` when one of its loads cannot be
/// served by the measured shape.
///
/// One stream carries no file beside the plan, so every book-sited dorc-lang root has to stand in
/// the stream itself. That is exactly `floor30-inline-dot-boundary`'s cell 1, and only that cell:
/// a `.` that shares its line, carries a redirect, or is not a top-level command is outside what was
/// measured, so the FORM is unavailable rather than the substitution being attempted anyway.
fn inline_imports(projection: &BundleProjection, loads: &[BookLoad]) -> Option<Vec<ImportEdit>> {
    loads
        .iter()
        .map(|load| {
            let root = projection
                .roots()
                .iter()
                .find(|root| root.id() == load.root)
                .filter(|_| load.absorbable && load.explicit)?;
            Some(ImportEdit::Inline {
                ast: load.command,
                sh: root.bundled().to_owned(),
            })
        })
        .collect()
}

/// What a settled form does with one book-reached bundle's bytes.
///
/// One value rather than two bools side by side: those are swappable without a type error, and the
/// difference between them decides whether a package's definitions may be hoisted above the book.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Carriage {
    /// Every file under the root reaches the artifact — the bundle at the `.`, and each nested file
    /// the bundle could not absorb beside it.
    WholeRoot,
    /// The bundle's absorbed bytes stand at the `.`, and a nested file it could not absorb reaches
    /// nothing: one stream carries no file beside the plan.
    AbsorbedOnly,
    /// Nothing reaches the artifact: the form places no dependency at all.
    Nothing,
}

/// Where a settled form stands every source a book `.` reaches
/// (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`).
///
/// Keyed by SOURCE rather than by bundle, because that is the question a definition asks: it
/// inherits the placement of the file it was authored in, and a bundle is several files.
fn placements(
    projection: &BundleProjection,
    loads: &[BookLoad],
    carriage: Carriage,
) -> PlacedSources {
    let mut placed = PlacedSources::all_ambient();
    for load in loads {
        let Some(root) = projection
            .roots()
            .iter()
            .find(|root| root.id() == load.root)
        else {
            continue;
        };
        let separate: std::collections::BTreeSet<_> = root.separate().iter().copied().collect();
        for &id in root.files() {
            let Some(file) = projection.file(id) else {
                continue;
            };
            let source = file.copied().source();
            let reaches = match carriage {
                Carriage::WholeRoot => true,
                Carriage::AbsorbedOnly => !separate.contains(&id),
                Carriage::Nothing => false,
            };
            if reaches {
                placed.carried(
                    source,
                    PlacementDecision::new(
                        Placement::InPlace(LoadSite(load.command)),
                        EmittedName::Authored,
                        if !load.explicit {
                            PlacementReason::KeptInPlaceOperandNotExplicit
                        } else if load.absorbable {
                            PlacementReason::KeptInPlaceLadderUnconsulted
                        } else {
                            PlacementReason::KeptInPlaceShapeUnmeasured
                        },
                    ),
                );
            } else {
                placed.uncarried(source);
            }
        }
    }
    placed
}

/// The artifact-set filename every form puts its plan projection under.
const PRIMARY_NAME: &str = "plan.sh";

/// A settled form and the files it will carry, decided BEFORE the plan exists.
///
/// The split from [`ArtifactSet`] is what makes the refusal pre-network: nothing here needs a
/// probe, a host, or a rendered plan, so an unservable request is answered while the run has still
/// touched nothing (`30I` §10: every invalid emission world is detected before network contact).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    form: ArtifactForm,
    fallback: Option<FormFallback>,
    dependencies: Vec<ArtifactFile>,
    imports: Vec<ImportEdit>,
    placements: PlacedSources,
}

impl Selection {
    /// Which form was chosen.
    #[must_use]
    pub const fn form(&self) -> ArtifactForm {
        self.form
    }

    /// Why `auto` did not reach a more flattened form, where it did not.
    #[must_use]
    pub const fn fallback(&self) -> Option<FormFallback> {
        self.fallback
    }

    /// What each book-sited import says in the GENERATED plan under this form
    /// (`30Ng:rul-bundle-at-dorc-lang-boundaries`).
    ///
    /// Handed to `Plan::decided` as an input, never applied afterwards: the form is settled from
    /// authored inputs before the plan exists, and what a plan's import line says is a decision like
    /// any other (`the-render-decides-nothing`).
    #[must_use]
    pub fn imports(&self) -> &[ImportEdit] {
        &self.imports
    }

    /// Where this form stands every source a book `.` reaches
    /// (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`).
    ///
    /// Handed to `Plan::decided` beside the imports, because a definition cannot stand anywhere its
    /// own file's bytes do not: a form that carries a package at the author's `.` must not ALSO
    /// hoist that package's definitions above the whole book, and a form that carries it nowhere
    /// places nothing.
    #[must_use]
    pub const fn placements(&self) -> &PlacedSources {
        &self.placements
    }

    /// Bind the settled form to the plan projection it describes.
    #[must_use]
    pub fn with_plan(self, plan_sh: String) -> ArtifactSet {
        ArtifactSet {
            form: self.form,
            fallback: self.fallback,
            primary: ArtifactFile {
                path: PRIMARY_NAME.to_owned(),
                bytes: plan_sh,
            },
            dependencies: self.dependencies,
        }
    }
}

/// The form a TERMINAL RENDER settles — `auto` on a stdout a person is watching.
///
/// TOTAL by construction, and that is why it is its own entry point: this is the one cell with no
/// refusal, because nothing is being kept to run later, so `auto` may settle for a less flattened
/// form and say so. The why driver mirrors the run's carriage through here rather than through a
/// refusal path it could never take (`one-definition-table-two-drivers`), and [`select`]'s own
/// `auto`-at-a-terminal arm calls it, so the two cannot drift.
#[must_use]
pub fn select_for_terminal_render(projection: &BundleProjection, loads: &[BookLoad]) -> Selection {
    let inline_debt = loads
        .iter()
        .filter(|load| !(load.absorbable && load.explicit))
        .count()
        .max(usize::from(inline_imports(projection, loads).is_none()));
    match inline_imports(projection, loads) {
        Some(imports) => Selection {
            form: ArtifactForm::Flattened,
            fallback: None,
            dependencies: Vec::new(),
            imports,
            placements: placements(projection, loads, Carriage::AbsorbedOnly),
        },
        None => Selection {
            form: ArtifactForm::PreservedBookTree,
            fallback: Some(FormFallback::InliningUnproven { loads: inline_debt }),
            dependencies: Vec::new(),
            imports: Vec::new(),
            placements: placements(projection, loads, Carriage::Nothing),
        },
    }
}

/// Choose a form.
///
/// # Errors
/// A named form that cannot be served refuses HERE — before any network contact, and before any
/// file is created — rather than returning a different form (`30I` §14: whether explicit
/// single-stream intent may silently return multipart output is not builder latitude).
pub fn select(
    cwd: &Cwd,
    snapshot_paths: &[String],
    projection: &BundleProjection,
    loads: &[BookLoad],
    request: FormRequest,
    posture: StreamPosture,
) -> Result<Selection, FormRefusal> {
    // A book with nothing to load is ALREADY one stream; one whose every bundle can stand where its
    // `.` stands becomes one (`floor30-inline-dot-boundary`'s measured cell).
    let inlined = inline_imports(projection, loads);
    let inline_debt = loads
        .iter()
        .filter(|load| !(load.absorbable && load.explicit))
        .count()
        .max(usize::from(inlined.is_none()));
    let multipart = match posture {
        StreamPosture::PipedArtifact | StreamPosture::TerminalRender => Err(None),
        StreamPosture::Materializable => {
            bundle_files(cwd, snapshot_paths, projection, loads).map_err(Some)
        }
    };

    let preserved = |fallback: Option<FormFallback>| Selection {
        form: ArtifactForm::PreservedBookTree,
        fallback,
        dependencies: Vec::new(),
        imports: Vec::new(),
        placements: placements(projection, loads, Carriage::Nothing),
    };
    let flattened = |imports: Vec<ImportEdit>| Selection {
        form: ArtifactForm::Flattened,
        fallback: None,
        dependencies: Vec::new(),
        imports,
        placements: placements(projection, loads, Carriage::AbsorbedOnly),
    };
    let whole_root = || placements(projection, loads, Carriage::WholeRoot);

    match request {
        FormRequest::Explicit(ArtifactForm::Flattened) => {
            inlined.map(flattened).ok_or(FormRefusal::Unavailable {
                form: ArtifactForm::Flattened,
                because: FormFallback::InliningUnproven { loads: inline_debt },
            })
        }
        FormRequest::Explicit(ArtifactForm::Multipart) => match multipart {
            Ok((dependencies, imports)) => Ok(Selection {
                form: ArtifactForm::Multipart,
                fallback: None,
                dependencies,
                imports,
                placements: whole_root(),
            }),
            Err(None) => Err(FormRefusal::NoArtifactStream {
                form: ArtifactForm::Multipart,
            }),
            Err(Some(unplaceable)) => Err(FormRefusal::Unavailable {
                form: ArtifactForm::Multipart,
                because: FormFallback::DependencyUnplaceable { loads: unplaceable },
            }),
        },
        FormRequest::Explicit(ArtifactForm::MirroredTree) => match posture {
            StreamPosture::Materializable => {
                match mirrored_files(cwd, snapshot_paths, projection, loads) {
                    Ok(dependencies) => Ok(Selection {
                        form: ArtifactForm::MirroredTree,
                        fallback: None,
                        dependencies,
                        imports: Vec::new(),
                        placements: whole_root(),
                    }),
                    Err(unplaceable) => Err(FormRefusal::Unavailable {
                        form: ArtifactForm::MirroredTree,
                        because: FormFallback::DependencyUnplaceable { loads: unplaceable },
                    }),
                }
            }
            _ => Err(FormRefusal::NoArtifactStream {
                form: ArtifactForm::MirroredTree,
            }),
        },
        // A preserved tree needs files beside it, so naming it on a KEPT stream asks for an
        // artifact that cannot run where it lands.
        FormRequest::Explicit(ArtifactForm::PreservedBookTree) => match posture {
            StreamPosture::PipedArtifact if !loads.is_empty() => {
                Err(FormRefusal::IncompleteSingleStream { loads: loads.len() })
            }
            _ => Ok(preserved(None)),
        },
        // One stream holds only the flat form; a materializable one aims at mode 2 (`30I` §7.1).
        FormRequest::Auto => match posture {
            // The one cell with no fallback: a stream the user is keeping carries a COMPLETE plan
            // or the run stops (`30Ng:rul-piped-stdout-carries-a-full-plan`, human-typed).
            StreamPosture::PipedArtifact => inlined
                .map(flattened)
                .ok_or(FormRefusal::IncompleteSingleStream { loads: inline_debt }),
            StreamPosture::TerminalRender => Ok(select_for_terminal_render(projection, loads)),
            StreamPosture::Materializable => Ok(match multipart {
                Ok((dependencies, imports)) => Selection {
                    form: ArtifactForm::Multipart,
                    fallback: None,
                    dependencies,
                    imports,
                    placements: whole_root(),
                },
                Err(unplaceable) => preserved(Some(FormFallback::DependencyUnplaceable {
                    loads: unplaceable.unwrap_or(loads.len()),
                })),
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactForm, ArtifactSet, FormFallback, FormRefusal, FormRequest, ImportEdit,
        StdoutPosture, StreamPosture, artifact_stream, mirrored, placeable, select,
    };
    use crate::bundle::BundleProjection;
    use dorc_core::loadpath::Cwd;

    fn empty() -> BundleProjection {
        BundleProjection::default()
    }

    fn assemble(
        plan_sh: &str,
        loads: &[super::BookLoad],
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<ArtifactSet, FormRefusal> {
        select(&Cwd::default(), &[], &empty(), loads, request, posture)
            .map(|selection| selection.with_plan(plan_sh.to_owned()))
    }

    /// A book load naming a root NO projection holds — so nothing can be composed for it, under any
    /// form. The unservable cell, spelled without a world.
    fn one_load() -> super::BookLoad {
        super::BookLoad {
            command: dorc_core::AstId(0),
            span: dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
            operand: Some(dorc_core::AstId(1)),
            root: crate::bundle::BundleRootId::first(),
            absorbable: true,
            explicit: true,
        }
    }

    /// The whole existing corpus's shape: a book that loads nothing is ALREADY one stream, so the
    /// flattened form is available and its bytes are the plan projection unchanged. If this ever
    /// stops holding, every committed golden moves — which is the tripwire, not a side effect.
    #[test]
    fn a_book_with_no_loads_flattens_to_the_plan_projection_unchanged() {
        let set = assemble(
            "#!/bin/sh\napt-get install -y nginx\n",
            &[],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("nothing to place");
        assert_eq!(set.form(), ArtifactForm::Flattened);
        assert_eq!(set.fallback(), None);
        assert!(set.dependencies().is_empty());
        assert_eq!(set.primary().bytes, "#!/bin/sh\napt-get install -y nginx\n");
        assert_eq!(set.files().count(), 1);
    }

    /// Explicit single-stream intent the compiler cannot satisfy REFUSES rather than returning a
    /// different form (`30I` §7.1's standing rule, and §14 keeps it out of builder latitude). The
    /// refusal names the form asked for AND the cause, because "no" without either is unactionable.
    #[test]
    fn explicit_flattening_refuses_when_a_dependency_would_have_to_be_inlined() {
        let refusal = assemble(
            "",
            &[one_load()],
            FormRequest::Explicit(ArtifactForm::Flattened),
            StreamPosture::TerminalRender,
        )
        .expect_err("v0 cannot inline a load-inert child safely yet");
        assert_eq!(refusal.form(), "flattened");
        assert_eq!(refusal.cause(), "inlining-unproven");
        assert!(matches!(
            refusal,
            FormRefusal::Unavailable {
                because: FormFallback::InliningUnproven { loads: 1 },
                ..
            }
        ));
    }

    /// The same world under AUTO: no refusal, a less flattened form, and an EXPLANATION. The pair
    /// with the test above is the whole selector contract — explicit intent is never silently
    /// downgraded, and auto never fails for a reason it declined to state.
    #[test]
    fn auto_falls_back_and_explains_rather_than_refusing() {
        let set = assemble(
            "",
            &[one_load()],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        assert_eq!(set.form(), ArtifactForm::PreservedBookTree);
        assert_eq!(
            set.fallback(),
            Some(FormFallback::InliningUnproven { loads: 1 })
        );
        assert!(set.dependencies().is_empty());
    }

    /// A multipart artifact asked for with nowhere to put it refuses BEFORE network, naming the
    /// stream problem rather than the book: one stream cannot carry a dependency tree
    /// distinguishably (`30I` §2.5).
    #[test]
    fn explicit_multipart_on_one_stream_refuses_naming_the_stream() {
        let refusal = assemble(
            "",
            &[],
            FormRequest::Explicit(ArtifactForm::Multipart),
            StreamPosture::TerminalRender,
        )
        .expect_err("a directory-shaped artifact cannot ride one pipe");
        assert_eq!(refusal.cause(), "no-artifact-stream");
    }

    /// THE WHOLE POSTURE TABLE (`30Ng:rul-piped-stdout-carries-a-full-plan`, human-typed), cell by
    /// cell, because the ruling is about all four at once and any one of them read alone is a
    /// plausible-looking mistake.
    ///
    /// The load-bearing cell is the first: a stdout the user is KEEPING and a named artifact
    /// directory both claim this run's one artifact, and ranking them silently is what would decide
    /// on the user's behalf which of two competing complete artifacts they meant.
    #[test]
    fn which_stream_carries_the_artifact_is_a_closed_table() {
        assert_eq!(
            artifact_stream(StdoutPosture::NonInteractive, true),
            Err(FormRefusal::TwoArtifactClaimants)
        );
        assert_eq!(
            artifact_stream(StdoutPosture::NonInteractive, false),
            Ok(StreamPosture::PipedArtifact)
        );
        assert_eq!(
            artifact_stream(StdoutPosture::Interactive, true),
            Ok(StreamPosture::Materializable)
        );
        assert_eq!(
            artifact_stream(StdoutPosture::Interactive, false),
            Ok(StreamPosture::TerminalRender)
        );
    }

    /// A KEPT stream that cannot be given a complete plan REFUSES, where the same book at a terminal
    /// falls back and explains.
    ///
    /// The pair is the ruling's own shape: what the reviewer approves on a kept stream is exactly
    /// what executes, so an artifact that cannot run where it lands is the wrong answer rather than
    /// a smaller one — while a render nobody is keeping loses nothing by being less flattened.
    #[test]
    fn a_kept_stream_refuses_where_a_terminal_render_falls_back() {
        let world = || {
            (
                "false || . ./wombat.oracle.sh\nwombat sync a.conf\n",
                vec!["wombat.oracle.sh".to_owned()],
                vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            )
        };
        let (book, paths, srcs) = world();
        let refusal = book_sourced(
            book,
            paths,
            srcs,
            FormRequest::Auto,
            StreamPosture::PipedArtifact,
        )
        .expect_err("a kept stream carries a complete plan or nothing");
        assert_eq!(refusal.cause(), "incomplete-single-stream");
        assert_eq!(refusal.form(), "auto", "the invocation named no form");
        assert_eq!(refusal.loads(), 1);

        let (book, paths, srcs) = world();
        let rendered = book_sourced(
            book,
            paths,
            srcs,
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("a render is allowed to be less flattened");
        assert_eq!(rendered.form(), ArtifactForm::PreservedBookTree);
    }

    /// …and NAMING the incomplete form does not buy it either: the posture semantics are constant
    /// across every flag-form, which is the half of the ruling that keeps them semantics rather than
    /// a default.
    #[test]
    fn naming_the_preserved_tree_does_not_override_a_kept_stream() {
        let refusal = book_sourced(
            "false || . ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Explicit(ArtifactForm::PreservedBookTree),
            StreamPosture::PipedArtifact,
        )
        .expect_err("an incomplete form on a kept stream is still incomplete");
        assert_eq!(refusal.cause(), "incomplete-single-stream");
    }

    /// A `.` whose operand does not name its target EXPLICITLY is never re-pointed, inlined or
    /// hoisted by any tier, in any form (`30P:rul-rewrite-permission-is-derived`, human-typed).
    ///
    /// CFG SHAPE: five top-level `Simple`s carrying one operand word each, one per admitted or
    /// refused spelling. The head is a plain command rather than a `.` because the predicate reads
    /// the WORD and nothing else, and because a `$( … )` inside a `.` operand is a parse-tier
    /// refusal today (`30P:fnd-computed-dot-is-a-whole-book-refusal`) — which would take the whole
    /// fixture with it. Today none of the refused three resolves at all, so the rewrite question
    /// never arises through a selection; the load lane's evaluator makes the `$0` forms resolve
    /// EXACT, and EXACT must not become permission — which is why the PRECONDITION is pinned at its
    /// own seat rather than through a selection that cannot yet reach it.
    #[test]
    fn a_computed_operand_is_never_explicit_enough_to_rewrite() {
        use dorc_syntax::ast::NodeKind;
        let book = concat!(
            "hork \"${0%/*}/wombat.dorc.sh\"\n",
            "hork \"$(dirname \"$0\")/wombat.dorc.sh\"\n",
            "hork \"$0\"\n",
            "hork './wombat.dorc.sh'\n",
            "hork \"./$PKG.dorc.sh\"\n",
        );
        let ast = dorc_syntax::parse(book).value;
        let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
            panic!("a script parses to a script");
        };
        let operands: Vec<dorc_core::AstId> = items
            .iter()
            .filter_map(|&id| match &ast.node(id).kind {
                NodeKind::Simple { words, .. } => words.get(1).copied(),
                _ => None,
            })
            .collect();
        assert_eq!(operands.len(), 5, "five `.` lines, five operands");
        let explicit: Vec<bool> = operands
            .iter()
            .map(|&word| super::operand_is_explicit(&ast, word))
            .collect();
        assert_eq!(
            explicit,
            vec![false, false, false, true, true],
            "self-location names no target the engine may re-say; a literal word and a \
             literal-assigned book-set root are the author naming one"
        );
    }

    /// The same rule at the FORM seat: with the operand inexplicit, one stream is unavailable (there
    /// is no line the bundle may stand in for) and multipart carries the dependency at its AUTHORED
    /// relative path with no import re-said, so the author's own operand finds it there.
    #[test]
    fn an_inexplicit_operand_mirrors_rather_than_re_saying_its_import() {
        let cwd = Cwd::default();
        let (snapshot, projection, mut loads) = world_at(
            &cwd,
            ". ./wombat.dorc.sh\nwombat sync a.conf\n",
            vec!["wombat.dorc.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
        );
        assert_eq!(loads.len(), 1, "one book-sited load");
        loads[0].explicit = false;
        let settled = |request, posture| {
            select(
                &cwd,
                snapshot.source_paths(),
                &projection,
                &loads,
                request,
                posture,
            )
        };
        let multipart = settled(FormRequest::Auto, StreamPosture::Materializable)
            .expect("the dependency is placeable at its authored path");
        assert_eq!(multipart.form(), ArtifactForm::Multipart);
        assert!(
            multipart.imports().is_empty(),
            "an inexplicit operand is not ours to re-say: {:?}",
            multipart.imports()
        );
        let published = multipart.with_plan(String::new());
        assert_eq!(
            published
                .dependencies()
                .iter()
                .map(|file| file.path.as_str())
                .collect::<Vec<_>>(),
            vec!["wombat.dorc.sh"],
            "the target mirrors where the author's own operand resolves"
        );
        let rendered = settled(FormRequest::Auto, StreamPosture::TerminalRender)
            .expect("a render is allowed to be less flattened");
        assert_eq!(rendered.form(), ArtifactForm::PreservedBookTree);
        assert_eq!(
            rendered.fallback(),
            Some(FormFallback::InliningUnproven { loads: 1 })
        );
    }

    /// Build a whole world the way the corpus does, over CLI-named prelude roots, and hand back
    /// the loader's complete occurrence account.
    fn account_of(
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
    ) -> dorc_analysis::load::LoadAccount {
        let cwd = Cwd::default();
        let snapshot = crate::snapshot::StaticLoadSnapshot::over(
            cwd,
            paths,
            srcs,
            &crate::snapshot::LoadPositions::roots_only(),
            "book.sh",
            book,
        );
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut interner = dorc_core::Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane)
            .loads()
            .clone()
    }

    /// THE VERSION-MISMATCH CELL, red-first (`30I` §2.2's guarded-source idiom, under the human's
    /// pending `rule-sentinel-value-conjunct` ruling).
    ///
    /// The world: `common` assigns `sm_common_loaded='v1'`, and `alpha`'s include guard tests for
    /// `'v2'`. A real shell compares the VALUES, finds them different, and takes the SOURCE arm —
    /// so common is loaded a SECOND time. The recognition instead reads whether the target
    /// closure's names are bound, and they are (common was pre-sourced first), so the engine
    /// selects the REUSE arm and records a load that never runs where sh runs one.
    ///
    /// Why it belongs to the artifact forms: a form asks the account "what does this program load",
    /// and an account that answers `Reused` where sh sources is an account a flattened artifact
    /// could act on by omitting the re-source. The disposition is safe TODAY only because
    /// flattening refuses to inline at all; the corner must not be golden-promoted while that is
    /// the only thing holding it.
    #[test]
    fn a_version_mismatched_sentinel_takes_the_source_arm() {
        const COMMON: &str = "# dorc-lang/v0.2\nsm_common_query() { :; }\nsm_common_loaded='v1'\n";
        const ALPHA: &str = concat!(
            "# dorc-lang/v0.2\n",
            "if [ \"${sm_common_loaded-}\" = 'v2' ]; then\n",
            "   :\n",
            "else\n",
            "   . ./common.oracle.sh\n",
            "fi\n",
            "alpha__is_converged() { sm_common_query \"$1\" ;}\n",
        );
        // Outside the closure: a panic HERE would read as the target still failing.
        let account = account_of(
            "alpha sync\n",
            vec!["common.oracle.sh".to_owned(), "alpha.oracle.sh".to_owned()],
            vec![COMMON.to_owned(), ALPHA.to_owned()],
        );
        let routes: Vec<dorc_analysis::load::LoadRoute> = account
            .occurrences()
            .iter()
            .filter(|occurrence| {
                occurrence.target.ends_with("common.oracle.sh") && occurrence.within.is_some()
            })
            .map(|occurrence| occurrence.route)
            .collect();
        internal_tooling::xfail::xfail_until("p-x-sentinel-value-conjunct", || {
            assert_eq!(
                routes,
                vec![dorc_analysis::load::LoadRoute::Taken],
                "sh compares 'v1' against 'v2' and takes the SOURCE arm, so the guarded `.` really \
                 runs — whatever the environment's names say about the target's closure"
            );
        });
    }

    /// Build a whole world over a BOOK-sourced tree, and settle a form over it.
    fn book_sourced(
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<super::Selection, FormRefusal> {
        book_sourced_at(&Cwd::default(), book, paths, srcs, request, posture)
    }

    /// The same, with the modelled working directory named — the axis production and the in-process
    /// drivers differ on, and therefore the axis a placement rule must be measured across.
    fn book_sourced_at(
        cwd: &Cwd,
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<super::Selection, FormRefusal> {
        let (snapshot, projection, loads) = world_at(cwd, book, paths, srcs);
        select(
            cwd,
            snapshot.source_paths(),
            &projection,
            &loads,
            request,
            posture,
        )
    }

    /// The analysed world a form is settled over, handed back whole so a test can perturb the loads
    /// before selecting.
    fn world_at(
        cwd: &Cwd,
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
    ) -> (
        crate::snapshot::StaticLoadSnapshot,
        BundleProjection,
        Vec<super::BookLoad>,
    ) {
        let reached = crate::snapshot::book_reached(cwd, &paths, &srcs, book);
        let snapshot = crate::snapshot::StaticLoadSnapshot::over(
            cwd.clone(),
            paths,
            srcs,
            &crate::snapshot::LoadPositions::book_sourced(reached),
            "book.sh",
            book,
        );
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut interner = dorc_core::Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        let projection = crate::bundle::project(&snapshot, env.loads())
            .map(crate::bundle::BundleProjectionOutput::into_projection)
            .expect("one closed occurrence forest");
        let loads = super::book_loads(&cfg, &ast, book, &projection);
        (snapshot, projection, loads)
    }

    /// THE MULTIPART PLACEMENT, end to end over a real load: the book's dorc-lang dependency is
    /// published as a BUNDLE, and the plan's own import names it
    /// (`30Ng:rul-bundle-at-dorc-lang-boundaries`, human-typed).
    ///
    /// Three halves, and all three are the ruling rather than styling. The file is GENERATED, so it
    /// does not sit under the author's own spelling; the plan is a generated durable, so its import
    /// may be re-said to name what the artifact actually carries; and the bytes are `dorc strip`
    /// output, so a stock shell can source them at all and the byte floor still holds
    /// (`strip-is-pure-erasure`).
    #[test]
    fn a_multipart_dependency_is_published_as_the_bundle_its_import_names() {
        let selection = book_sourced(
            ". ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec![
                "# dorc-lang/v0.2\nwombat__is_converged() { wombat status : sm.dorc.W:@ok ;}\n"
                    .to_owned(),
            ],
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("a relative dependency is placeable");
        assert_eq!(selection.form(), ArtifactForm::Multipart);
        assert_eq!(selection.fallback(), None);
        assert!(
            matches!(
                selection.imports(),
                [ImportEdit::Repoint { path, .. }] if path == "./wombat.oracle.dorc-bundle.sh"
            ),
            "the plan's import names the published bundle: {:?}",
            selection.imports()
        );
        let set = selection.with_plan("#!/bin/sh\n".to_owned());
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(paths, ["plan.sh", "wombat.oracle.dorc-bundle.sh"]);
        let dependency = &set.dependencies()[0].bytes;
        assert!(
            dependency.contains("wombat__is_converged()") && !dependency.contains(" : sm.dorc.W"),
            "the published bundle is the STRIPPED body a stock shell can source:\n{dependency}"
        );
    }

    /// The same world with stdout as the artifact stream: one stream carries no file beside the
    /// plan, so the bundle stands where the `.` stood and the stream is a COMPLETE plan
    /// (`30Ng:rul-piped-stdout-carries-a-full-plan` — what the reviewer approves is what executes).
    ///
    /// The substitution is exactly `floor30-inline-dot-boundary`'s measured cell, and the `.` here is
    /// exactly its shape: a top-level command alone on its line.
    #[test]
    fn the_same_world_on_one_stream_carries_the_bundle_in_the_stream() {
        let selection = book_sourced(
            ". ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        assert_eq!(selection.form(), ArtifactForm::Flattened);
        assert_eq!(selection.fallback(), None);
        assert!(
            selection.dependencies.is_empty(),
            "one stream carries no file beside the plan"
        );
        assert!(
            matches!(
                selection.imports(),
                [ImportEdit::Inline { sh, .. }] if sh.contains("wombat__is_converged()")
            ),
            "the bundle's own bytes stand where the load did: {:?}",
            selection.imports()
        );
    }

    /// …and a book `.` OUTSIDE that measured shape leaves the single stream unable to carry a
    /// complete plan, so `auto` falls back and SAYS SO rather than substituting anyway.
    ///
    /// The `.` here is an `||` right operand, which is the cell the floor manifest measured a
    /// difference at: a `.` is one command and the bytes it loads are N, so the operator covers
    /// different things in the two shapes.
    #[test]
    fn a_load_outside_the_measured_shape_leaves_one_stream_incomplete() {
        let selection = book_sourced(
            "false || . ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        assert_eq!(selection.form(), ArtifactForm::PreservedBookTree);
        assert_eq!(
            selection.fallback(),
            Some(FormFallback::InliningUnproven { loads: 1 })
        );
        assert!(selection.imports().is_empty());
    }

    /// …and so does a `.` carrying a LEADING ASSIGNMENT, for a different reason than the operator
    /// cell above: `Inline` replaces the whole command node, and the assignment is part of that node.
    ///
    /// CFG shape: one top-level `Simple` whose `assigns` is non-empty and whose word 1 is the load
    /// operand — `MODE=prod . ./wombat.oracle.sh`. Substituting the bundle's bytes for that node
    /// drops `MODE=prod`, so the absorbed top level reads an environment the authored `.` would have
    /// had. The multipart half is the acceptance's other end: `Repoint` moves the OPERAND only, so
    /// the assignment survives byte-for-byte and that form stays available.
    #[test]
    fn a_load_carrying_a_leading_assignment_is_not_absorbable() {
        let world = || {
            (
                "MODE=prod . ./wombat.oracle.sh\nwombat sync a.conf\n",
                vec!["wombat.oracle.sh".to_owned()],
                vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            )
        };

        let (book, paths, srcs) = world();
        let one_stream = book_sourced(
            book,
            paths,
            srcs,
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        assert_eq!(one_stream.form(), ArtifactForm::PreservedBookTree);
        assert_eq!(
            one_stream.fallback(),
            Some(FormFallback::InliningUnproven { loads: 1 })
        );
        assert!(
            one_stream.imports().is_empty(),
            "no edit at all beats an edit that drops the assignment: {:?}",
            one_stream.imports()
        );

        let (book, paths, srcs) = world();
        let multipart = book_sourced(
            book,
            paths,
            srcs,
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("a relative dependency is placeable");
        assert_eq!(multipart.form(), ArtifactForm::Multipart);
        assert!(
            matches!(
                multipart.imports(),
                [ImportEdit::Repoint { path, .. }] if path == "./wombat.oracle.dorc-bundle.sh"
            ),
            "the operand still re-points, so the assignment ahead of it is untouched: {:?}",
            multipart.imports()
        );
    }

    /// THE PRODUCTION CWD, which is the shape every real invocation has and no other test here had.
    ///
    /// `invocation_cwd()` answers an ABSOLUTE directory whenever the platform can say where the run
    /// stands, and every book-sourced dependency is then filed under an absolute canonical key. A
    /// placement rule keyed on the stored spelling therefore answered "unplaceable" for every real
    /// book while every test above, whose modelled cwd is the flat virtual one, said the opposite —
    /// so `dorc plan --artifact-dir out` on a book that sourced a package silently fell back to the
    /// preserved tree and published its plan with no dependency beside it. Mirroring is stated
    /// against the load cwd for exactly this reason, and this is the cell that measures it.
    #[test]
    fn a_dependency_under_an_absolute_load_cwd_still_mirrors() {
        let selection = book_sourced_at(
            &Cwd::at("/ops/case"),
            ". ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["/ops/case/wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("a dependency inside the load cwd is placeable");
        assert_eq!(selection.form(), ArtifactForm::Multipart);
        assert_eq!(selection.fallback(), None);
        let set = selection.with_plan("#!/bin/sh\n".to_owned());
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(
            paths,
            ["plan.sh", "wombat.oracle.dorc-bundle.sh"],
            "the destination is the operand's own spelling, recovered through the cwd"
        );
    }

    /// BOTH ENDS of the bundle-point axis, over one world (`30Ng` §5, human-typed: both extremes
    /// fully supported, and the default at neither).
    ///
    /// The pair is the axis made observable. Most-flattened is ONE emission with the subgraph in the
    /// stream; no-flatten is the author's own tree, file for file, with no import re-said — and the
    /// default sits between them, composing one bundle per root and saying where it points.
    #[test]
    fn both_ends_of_the_bundle_point_axis_are_reachable_by_name() {
        let world = || {
            (
                ". ./alpha.oracle.sh\nwombat sync a.conf\n",
                vec![
                    "/ops/case/common.oracle.sh".to_owned(),
                    "/ops/case/alpha.oracle.sh".to_owned(),
                ],
                vec![
                    "# dorc-lang/v0.2\nsm_common_query() { :; }\n".to_owned(),
                    "# dorc-lang/v0.2\n. ./common.oracle.sh\nalpha__is_converged() { :; }\n"
                        .to_owned(),
                ],
            )
        };
        let at = Cwd::at("/ops/case");

        let (book, paths, srcs) = world();
        let flattened = book_sourced_at(
            &at,
            book,
            paths,
            srcs,
            FormRequest::Explicit(ArtifactForm::Flattened),
            StreamPosture::PipedArtifact,
        )
        .expect("the one load is absorbable");
        assert!(
            flattened.dependencies.is_empty() && flattened.imports().len() == 1,
            "one emission, and the subgraph is IN it"
        );

        let (book, paths, srcs) = world();
        let mirrored_tree = book_sourced_at(
            &at,
            book,
            paths,
            srcs,
            FormRequest::Explicit(ArtifactForm::MirroredTree),
            StreamPosture::Materializable,
        )
        .expect("every source is inside the load cwd");
        assert!(
            mirrored_tree.imports().is_empty(),
            "nothing is re-said, because a mirrored file needs no re-pointing"
        );
        let set = mirrored_tree.with_plan("#!/bin/sh\n".to_owned());
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(
            paths,
            ["plan.sh", "alpha.oracle.sh", "common.oracle.sh"],
            "no flattening at all: each source at the spelling its own sourcer used"
        );
    }

    /// THE DIAMOND, at the placement seat: two book load points reach one shared dependency, and
    /// each bundle ABSORBS it — so the artifact carries two files, not four, and no file is a
    /// dependency of another.
    ///
    /// `rul-bundles-key-to-load-occurrences` keeps the two occurrences apart on purpose, which is
    /// what lets each root compose its own subgraph; the shared file appearing inside both is not
    /// duplication the artifact has to reconcile, because a dorc-lang top level is idempotent to load
    /// and the authors' own include guards decide whether it runs twice.
    #[test]
    fn two_roots_over_one_shared_dependency_compose_two_bundles() {
        let selection = book_sourced_at(
            &Cwd::at("/ops/case"),
            ". ./alpha.oracle.sh\n. ./beta.oracle.sh\nwombat sync a.conf\n",
            vec![
                "/ops/case/common.oracle.sh".to_owned(),
                "/ops/case/alpha.oracle.sh".to_owned(),
                "/ops/case/beta.oracle.sh".to_owned(),
            ],
            vec![
                "# dorc-lang/v0.2\nsm_common_query() { :; }\n".to_owned(),
                "# dorc-lang/v0.2\n. ./common.oracle.sh\nalpha__is_converged() { :; }\n".to_owned(),
                "# dorc-lang/v0.2\n. ./common.oracle.sh\nbeta__is_converged() { :; }\n".to_owned(),
            ],
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("every dependency is inside the load cwd");
        assert_eq!(selection.form(), ArtifactForm::Multipart);
        let set = selection.with_plan("#!/bin/sh\n".to_owned());
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(
            paths,
            [
                "plan.sh",
                "alpha.oracle.dorc-bundle.sh",
                "beta.oracle.dorc-bundle.sh"
            ],
            "one bundle per book-sited root, and the shared file inside each rather than beside them"
        );
        assert!(
            set.dependencies()
                .iter()
                .all(|file| file.bytes.contains("sm_common_query()")),
            "each bundle absorbed the dependency it reached"
        );
    }

    /// A dependency the book reached OUTSIDE the load working directory has no relative spelling
    /// under an artifact root, so the form is unavailable rather than fudged — the half of
    /// `need-controller-paths-never-cross-hosts` that a cwd-relative rule must not lose.
    #[test]
    fn a_dependency_outside_the_load_cwd_is_unplaceable() {
        let selection = book_sourced_at(
            &Cwd::at("/ops/case"),
            ". /opt/shared/wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["/opt/shared/wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("auto always lands somewhere");
        assert_eq!(selection.form(), ArtifactForm::PreservedBookTree);
        assert_eq!(
            selection.fallback(),
            Some(FormFallback::DependencyUnplaceable { loads: 1 })
        );
    }

    /// Placement is the availability question, so its refusals are load-bearing: a path outside the
    /// load working directory, or one whose shape could not be a destination, cannot be mirrored
    /// under an artifact root — and a mirrored tree is exactly what lets every authored operand,
    /// the book's and every nested one, resolve unchanged (`30I` §7.4).
    #[test]
    fn only_a_path_inside_the_load_cwd_can_be_mirrored() {
        let case = Cwd::at("/ops/case");
        assert_eq!(
            mirrored(&case, "/ops/case/oracles/alpha.sh"),
            Some("oracles/alpha.sh".into())
        );
        assert_eq!(mirrored(&case, "/opt/alpha.sh"), None);
        assert_eq!(mirrored(&case, "/ops/alpha.sh"), None);
        assert_eq!(mirrored(&case, "./alpha.sh"), Some("alpha.sh".into()));
        assert_eq!(
            mirrored(&Cwd::default(), "alpha.sh"),
            Some("alpha.sh".into())
        );
        assert_eq!(mirrored(&Cwd::default(), "/etc/alpha.sh"), None);
        assert_eq!(mirrored(&Cwd::unknown(), "alpha.sh"), None);
    }

    /// The path-SHAPE half, independent of any cwd: what could be a destination under a root at all.
    #[test]
    fn only_a_relative_traversal_free_path_can_be_mirrored() {
        assert_eq!(
            placeable("oracles/alpha.sh"),
            Some("oracles/alpha.sh".into())
        );
        assert_eq!(placeable("./alpha.sh"), Some("alpha.sh".into()));
        assert_eq!(placeable(".\\alpha.sh"), Some("alpha.sh".into()));
        assert_eq!(placeable("/etc/alpha.sh"), None);
        assert_eq!(placeable("C:/tmp/alpha.sh"), None);
        assert_eq!(placeable("../alpha.sh"), None);
        assert_eq!(placeable("a/../../escape.sh"), None);
        assert_eq!(placeable(""), None);
        assert_eq!(placeable("."), None);
    }
}
