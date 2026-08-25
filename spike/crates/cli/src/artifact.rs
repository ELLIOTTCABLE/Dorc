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

use crate::bundle::{BundleFileId, BundleProjection, BundleRootId};

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
    account: dorc_core::influence::InfluenceAccount,
    topology: ArtifactTopology,
}

impl ArtifactSet {
    /// Which form this set is in.
    #[must_use]
    pub const fn form(&self) -> ArtifactForm {
        self.form
    }

    /// Where this published product stands: the selection's own account JOINED with the plan's
    /// (`Selection::with_plan`).
    #[must_use]
    pub const fn account(&self) -> dorc_core::influence::InfluenceAccount {
        self.account
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

    /// The generated dependencies, in a deterministic order. Populated for
    /// [`ArtifactForm::Multipart`] and [`ArtifactForm::MirroredTree`]; empty for the two forms
    /// that place no file beside the plan.
    #[must_use]
    pub fn dependencies(&self) -> &[ArtifactFile] {
        &self.dependencies
    }

    /// Every file to publish, primary first.
    pub fn files(&self) -> impl Iterator<Item = &ArtifactFile> {
        std::iter::once(&self.primary).chain(self.dependencies.iter())
    }

    /// Which published paths this set materializes as roots, and how its files reach one another —
    /// carried from the placement seat that chose each destination.
    #[must_use]
    pub const fn topology(&self) -> &ArtifactTopology {
        &self.topology
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
    /// What Dorc may do to this `.` line, on the two axes that govern it.
    pub permits: LoadPermission,
}

/// What Dorc may do to one `.` line — and the two axes are NOT interchangeable.
///
/// EXPLICITNESS asks whether the AUTHOR named the target, so a rewrite of that line is Dorc's to
/// make (`30P:rul-rewrite-permission-is-derived`, human-typed). EXACTNESS asks whether the
/// CONTROLLER can say which file the line loads, so anything may rest on it at all
/// (`30P:rul-load-head-is-exact-or-havoc`). Below a BLIND ACT — a line whose effect on the shell
/// Dorc cannot see — the second fails while the first still holds, and that cell is exactly where
/// re-pointing a reference changes which file the host loads
/// (`30P:law-no-unsoundness-below-a-blind-act`).
///
/// One value with private fields rather than two bools side by side: those are swappable without a
/// type error, and the questions they answer reach different seats. Three cells, and the third is
/// the one the law adds — an EXACT explicit load may be re-pointed or pasted; an EXACT inexplicit
/// one stays verbatim and is mirrored so the author's own operand finds it; a non-EXACT one stays
/// verbatim and nothing is shipped for it at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadPermission {
    explicit: bool,
    exact: bool,
}

impl LoadPermission {
    /// The permission a load carries, from the two questions answered independently.
    #[must_use]
    pub const fn of(explicit: bool, exact: bool) -> Self {
        Self { explicit, exact }
    }

    /// May an emitter re-point, inline, hoist or paste this line? The author named the target AND
    /// Dorc knows which file that is — explicitness alone never licenses a rewrite.
    #[must_use]
    pub const fn may_rewrite(self) -> bool {
        self.explicit && self.exact
    }

    /// May any form carry a copy of this line's target? A copy of a file Dorc cannot prove the
    /// author referenced is engine selection, which is the thing the law strikes.
    #[must_use]
    pub const fn may_ship(self) -> bool {
        self.exact
    }

    /// Did the author name the target — the REWRITE axis alone, for the one seat that owes the
    /// admin a sentence about which of the two failed.
    #[must_use]
    pub const fn is_explicit(self) -> bool {
        self.explicit
    }
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
            WordPart::CommandSubst(_) | WordPart::Arithmetic | WordPart::ParamExpansion { .. } => {
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
/// `env` answers the EXACT axis: `funcenv::FuncEnv::load_certainty` is the ONE seat that composes
/// the analysis's own two maps, so no second derivation of "does Dorc know which file this loads"
/// exists here to drift from it (`30P:rul-load-head-is-exact-or-havoc`).
#[must_use]
pub fn book_loads(
    cfg: &dorc_analysis::cfg::Cfg,
    book: &dorc_syntax::Ast,
    book_src: &str,
    projection: &BundleProjection,
    env: &dorc_analysis::funcenv::FuncEnv,
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
                permits: LoadPermission::of(
                    operand.is_some_and(|word| operand_is_explicit(book, word)),
                    env.load_certainty(occurrence.load().at).is_ok(),
                ),
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

/// One file-to-file reach inside a published artifact, named by the paths the artifact publishes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CarriedEdge {
    /// The published path whose bytes reach the child.
    pub parent: String,
    /// The published path they reach.
    pub child: String,
}

/// Which published paths a settled form materializes as roots, and how its files reach one another.
///
/// Built beside the placement that chose each destination, because that is the only seat holding
/// the correspondence between a projected file and the path carrying its bytes: the bundle
/// projection answers which entry a root materializes and the load account answers which occurrence
/// encloses which, and both are gone by the time an artifact set exists. A consumer downstream can
/// restore neither, so this is carriage rather than derivation — where the correspondence is
/// missing the form refuses instead of composing a plausible one
/// (`30Ng:rul-bundle-at-dorc-lang-boundaries`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactTopology {
    roots: Vec<String>,
    edges: Vec<CarriedEdge>,
}

impl ArtifactTopology {
    /// The published path each shipped book-sited load materializes, deduplicated: two load points
    /// naming one entrypoint compose one file and therefore one root.
    #[must_use]
    pub fn roots(&self) -> &[String] {
        &self.roots
    }

    /// Every reach between published files, in a deterministic order.
    #[must_use]
    pub fn edges(&self) -> &[CarriedEdge] {
        &self.edges
    }
}

/// Restore the load topology over the paths a placement seat actually chose.
///
/// `sited` maps every projected file a form carried to the published path holding its bytes — for
/// an ABSORBED file that is the path of the bundle it was inlined into, which is what makes an
/// absorbed dependency produce no edge without a special case: parent and child name one path, and
/// a file does not reach itself.
///
/// Answers `None` on any gap. The enclosure index, the root's entry, and every carried file must
/// all resolve, because an edge set that is complete except where it was hard to look is exactly
/// the false record this container exists to refuse.
fn topology_of(
    projection: &BundleProjection,
    loads: &[BookLoad],
    sited: &std::collections::BTreeMap<BundleFileId, String>,
) -> Option<ArtifactTopology> {
    let shipped: std::collections::BTreeSet<BundleRootId> = loads
        .iter()
        .filter(|load| load.permits.may_ship())
        .map(|load| load.root)
        .collect();
    let mut roots: Vec<String> = Vec::new();
    for id in &shipped {
        let root = projection.roots().iter().find(|root| root.id() == *id)?;
        let destination = sited.get(&root.entry())?;
        if !roots.contains(destination) {
            roots.push(destination.clone());
        }
    }

    let occurrences = projection.occurrences();
    let mut edges: std::collections::BTreeSet<CarriedEdge> = std::collections::BTreeSet::new();
    for occurrence in occurrences {
        if !shipped.contains(&occurrence.root()) {
            continue;
        }
        let child = sited.get(&occurrence.file())?;
        // A root act is reached by the BOOK's own `.`, which the plan projection carries — under
        // every form, whether the import was re-said to name a generated bundle or left naming the
        // author's own mirrored path.
        let parent = match occurrence.load().within {
            None => PRIMARY_NAME.to_owned(),
            Some(enclosing) => sited.get(&occurrences.get(enclosing)?.file())?.clone(),
        };
        if parent == *child {
            continue;
        }
        edges.insert(CarriedEdge {
            parent,
            child: child.clone(),
        });
    }

    Some(ArtifactTopology {
        roots,
        edges: edges.into_iter().collect(),
    })
}

/// A form's placements, and the correspondence between a projected file and the published path
/// holding its bytes.
///
/// One value rather than three locals, because the correspondence is only true if it is recorded
/// by the same act that chooses the destination: a seat that placed bytes and forgot to site them
/// leaves an artifact whose record of itself is short, which the topology walk then reads as a gap.
#[derive(Default)]
struct Placer {
    placed: std::collections::BTreeMap<String, String>,
    sited: std::collections::BTreeMap<BundleFileId, String>,
    unplaceable: usize,
}

impl Placer {
    /// Put `bytes` at `destination`, siting every projected file those bytes carry.
    ///
    /// Two DIFFERENT byte-sets claiming one destination is unplaceable rather than last-wins — an
    /// artifact whose dependency depends on which occurrence was walked last is not a projection
    /// of anything.
    fn place(&mut self, carried: &[BundleFileId], destination: &str, bytes: String) {
        match self.placed.get(destination) {
            Some(existing) if *existing != bytes => {
                self.miss();
                return;
            }
            Some(_) => {}
            None => drop(self.placed.insert(destination.to_owned(), bytes)),
        }
        for &file in carried {
            self.sited.insert(file, destination.to_owned());
        }
    }

    /// Note one file the form could not place.
    fn miss(&mut self) {
        self.unplaceable = self.unplaceable.saturating_add(1);
    }

    /// The placed files and their topology, or the count that made the form unavailable.
    ///
    /// A topology that cannot be stated exactly fails the form HERE, pre-network, exactly as an
    /// unplaceable file does.
    fn finish(
        self,
        projection: &BundleProjection,
        loads: &[BookLoad],
    ) -> Result<(Vec<ArtifactFile>, ArtifactTopology), usize> {
        if self.unplaceable > 0 || self.placed.len() > MAX_DEPENDENCIES {
            return Err(self
                .unplaceable
                .max(self.placed.len().saturating_sub(MAX_DEPENDENCIES)));
        }
        let topology = topology_of(projection, loads, &self.sited).ok_or(1_usize)?;
        Ok((
            self.placed
                .into_iter()
                .map(|(path, bytes)| ArtifactFile { path, bytes })
                .collect(),
            topology,
        ))
    }
}

/// The files a root's bundled text absorbed — its entry and every nested file it inlined.
fn absorbed_files(root: &crate::bundle::BundleRoot) -> Vec<BundleFileId> {
    let separate: std::collections::BTreeSet<BundleFileId> =
        root.separate().iter().copied().collect();
    root.files()
        .iter()
        .copied()
        .filter(|id| !separate.contains(id))
        .collect()
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
    snapshot: &crate::snapshot::StaticLoadSnapshot,
    projection: &BundleProjection,
    loads: &[BookLoad],
) -> Result<(Vec<ArtifactFile>, Vec<ImportEdit>, ArtifactTopology), usize> {
    let cwd = snapshot.cwd();
    let snapshot_paths = snapshot.source_paths();
    let mut out = Placer::default();
    let mut imports: Vec<ImportEdit> = Vec::new();
    let authored_of = |file: &crate::bundle::BundleFile| {
        snapshot_paths
            .get(file.copied().source().0 as usize)
            .map_or("", String::as_str)
    };
    for load in loads {
        // NOTHING IS SHIPPED FOR A NON-EXACT LOAD (`30P:law-no-unsoundness-below-a-blind-act`): a
        // copy of a file Dorc cannot prove the author's own line will read is engine selection.
        // Not `unplaceable` — the form is available and this dependency is deliberately absent
        // from it, which is a different answer from one the form could not carry.
        if !load.permits.may_ship() {
            continue;
        }
        // Unplaceable, never silently skipped: omitting a file the runtime `.` will look for is
        // what the possible-load projection exists to prevent (`30I` §6.1).
        let Some(root) = projection
            .roots()
            .iter()
            .find(|root| root.id() == load.root)
        else {
            out.miss();
            continue;
        };
        // AN INCLUSION IS MIRRORED, NEVER BUNDLED. Its bytes are BOOK-CLASS
        // (`30P:principle-book-code-source-is-inclusion`): Dorc composed nothing out of them and
        // has nothing to re-say, so the file lands at the author's own relative path and the
        // author's own `.` finds it there — no generated name, no import edit, `two-surfaces`'
        // byte floor intact.
        if let Some(entry) = projection.file(root.entry())
            && is_included(snapshot, entry)
        {
            match mirrored(cwd, authored_of(entry)) {
                Some(beside) => {
                    out.place(&[root.entry()], &beside, entry.copied().text().to_owned());
                }
                None => out.miss(),
            }
            continue;
        }
        // A line Dorc may not rewrite stays verbatim, so every file under it is mirrored at the
        // authored relative path the author's own operand will resolve to. Two ways to land here:
        // the author did not NAME the target, or Dorc cannot say WHICH file the line loads.
        if !load.permits.may_rewrite() {
            for &id in root.files() {
                let Some(file) = projection.file(id) else {
                    out.miss();
                    continue;
                };
                match mirrored(cwd, authored_of(file)) {
                    Some(beside) => out.place(&[id], &beside, file.copied().text().to_owned()),
                    None => out.miss(),
                }
            }
            continue;
        }
        let Some((entry, operand)) = projection.file(root.entry()).zip(load.operand) else {
            out.miss();
            continue;
        };
        let Some(destination) = mirrored(cwd, authored_of(entry)).map(|path| bundle_name(&path))
        else {
            out.miss();
            continue;
        };
        // The bundle's bytes carry the entry AND every nested file it absorbed, so all of them are
        // sited at this one destination. That is what makes an absorbed dependency cast no edge:
        // its `.` was replaced by the bytes, and a file does not reach itself.
        out.place(
            &absorbed_files(root),
            &destination,
            root.bundled().to_owned(),
        );
        // SEAM (`30P:rul-rewrite-permission-is-derived`): an import edit may mint only for an
        // EXPLICIT operand — a literal word, or one built from a literal-assigned book-set root.
        // Today the resolution is `literal_text`-only, so every operand that reaches here is
        // explicit and the fence is vacuous; when the load-head evaluator lands its typed
        // explicitness marker, THIS is the seat that reads it before minting.
        imports.push(ImportEdit::Repoint {
            ast: operand,
            path: format!("./{destination}"),
            reason: kept_in_place_reason(load),
        });
        for &id in root.separate() {
            let Some(file) = projection.file(id) else {
                out.miss();
                continue;
            };
            match mirrored(cwd, authored_of(file)) {
                Some(beside) => out.place(&[id], &beside, file.copied().text().to_owned()),
                None => out.miss(),
            }
        }
    }
    let (dependencies, topology) = out.finish(projection, loads)?;
    Ok((dependencies, imports, topology))
}

/// Every reached source at its OWN authored relative path — the no-flatten end of the bundle-point
/// axis (`30Ng` §5), and the placement machinery this arc inherited, kept reachable by name.
///
/// No import is re-said, because none has to be: a file mirrored at the spelling its sourcer used
/// resolves on the target exactly as it did controller-side, which is the cwd-analysis answer `30I`
/// §7.4 asks for. That is the whole difference from the default — the artifact is the author's tree
/// rather than the engine's composition of it.
fn mirrored_files(
    snapshot: &crate::snapshot::StaticLoadSnapshot,
    projection: &BundleProjection,
    loads: &[BookLoad],
) -> Result<(Vec<ArtifactFile>, ArtifactTopology), usize> {
    let cwd = snapshot.cwd();
    let snapshot_paths = snapshot.source_paths();
    // A non-EXACT load carries nothing here either — the mirror is a shipped copy like any other
    // (`30P:law-no-unsoundness-below-a-blind-act`).
    let wanted: std::collections::BTreeSet<BundleRootId> = loads
        .iter()
        .filter(|load| load.permits.may_ship())
        .map(|load| load.root)
        .collect();
    let mut out = Placer::default();
    for id in &wanted {
        let Some(root) = projection.roots().iter().find(|root| root.id() == *id) else {
            out.miss();
            continue;
        };
        // NOTHING IS ABSORBED IN THIS FORM: every file stands at its own authored path, which is
        // exactly why its edges cannot be assumed. The moment one dependency sources another the
        // plan is not what loads the inner file, and the siting below is what says so.
        for &id in root.files() {
            let Some(file) = projection.file(id) else {
                out.miss();
                continue;
            };
            let authored = snapshot_paths
                .get(file.copied().source().0 as usize)
                .map_or("", String::as_str);
            match mirrored(cwd, authored) {
                Some(destination) => {
                    out.place(&[id], &destination, file.copied().text().to_owned());
                }
                None => out.miss(),
            }
        }
    }
    out.finish(projection, loads)
}

/// The in-place substitutions a single-stream set needs, or `None` when one of its loads cannot be
/// served by the measured shape.
///
/// One stream carries no file beside the plan, so every book-sited dorc-lang root has to stand in
/// the stream itself. That is exactly `floor30-inline-dot-boundary`'s cell 1, and only that cell:
/// a `.` that shares its line, carries a redirect, or is not a top-level command is outside what was
/// measured, so the FORM is unavailable rather than the substitution being attempted anyway.
fn inline_imports(
    snapshot: &crate::snapshot::StaticLoadSnapshot,
    projection: &BundleProjection,
    loads: &[BookLoad],
) -> Option<Vec<ImportEdit>> {
    loads
        .iter()
        .map(|load| {
            let root = projection
                .roots()
                .iter()
                .find(|root| root.id() == load.root)
                .filter(|_| load.absorbable && load.permits.may_rewrite())?;
            // AN INCLUSION MAKES THE FORM UNAVAILABLE. Pasting an ordinary sh file into one stream
            // is `mech-paste-plain-sh-single-stream`, which is forfeited behind a closed exclusion
            // set nobody has welded (`30P:principle-book-code-source-is-inclusion` tier 3;
            // `FORFEITS:forfeit-plain-sh-inclusion-analysis`) — top-level `return` alone would
            // change what the surrounding plan does. Refusing by NAME is `KNOBS:kBACKFLIPS`'s
            // verbatim-or-refuse weld; there is no compile-to-fit here.
            let entry = projection.file(root.entry())?;
            if is_included(snapshot, entry) {
                return None;
            }
            Some(ImportEdit::Inline {
                ast: load.command,
                sh: root.bundled().to_owned(),
                reason: kept_in_place_reason(load),
            })
        })
        .collect()
}

/// Which ladder condition decided that this load's bundle stands where the author's `.` stands.
///
/// ONE seat, read by the placement account and by the import edit alike, so the reason a plan
/// DISCLOSES and the reason the placement RECORDS cannot say different things about one line.
///
/// It reads the EXPLICITNESS axis alone, deliberately: a non-EXACT load reaches no import edit and
/// carries no bytes anywhere, so it never arrives here at all. Answering
/// `KeptInPlaceOperandNotExplicit` for a line that IS explicit and merely non-EXACT would point the
/// author at the wrong repair, which is the top of `271:rul-sin-ordering`.
const fn kept_in_place_reason(load: &BookLoad) -> PlacementReason {
    if !load.permits.is_explicit() {
        PlacementReason::KeptInPlaceOperandNotExplicit
    } else if load.absorbable {
        PlacementReason::KeptInPlaceLadderUnconsulted
    } else {
        PlacementReason::KeptInPlaceShapeUnmeasured
    }
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
            let reaches = load.permits.may_ship()
                && match carriage {
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
                        kept_in_place_reason(load),
                    ),
                );
            } else {
                placed.uncarried(source);
            }
        }
    }
    placed
}

/// Is this projected file an ordinary sh INCLUSION — acquired for its bytes, modelled not at all?
///
/// One seat, because three placement decisions turn on it and each would otherwise re-spell the
/// role check (`30P:principle-book-code-source-is-inclusion`).
fn is_included(
    snapshot: &crate::snapshot::StaticLoadSnapshot,
    file: &crate::bundle::BundleFile,
) -> bool {
    snapshot.role_of(file.copied().source().0 as usize)
        == Some(crate::snapshot::SourceRole::PlainInclusion)
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
    topology: ArtifactTopology,
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

    /// Everything this settled form answers before the plan exists: where it stands every source a
    /// book `.` reaches, and what each of those imports says.
    ///
    /// One value, handed whole to `Plan::decided`, because a definition cannot stand anywhere its
    /// own file's bytes do not: a form that carries a package at the author's `.` must not ALSO
    /// hoist that package's definitions above the whole book, and a form that carries it nowhere
    /// places nothing (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`).
    #[must_use]
    pub const fn emission(&self) -> dorc_plan::ArtifactEmission<'_> {
        dorc_plan::ArtifactEmission::of(&self.placements, self.imports.as_slice())
    }

    /// Where this selection stands — RESTRICTED to authored-before-contact, and structurally so:
    /// nothing here needs a probe, a host, or a rendered plan, which is the same property that
    /// makes the refusal pre-network.
    #[must_use]
    pub fn account(&self) -> dorc_core::influence::InfluenceAccount {
        self.emission().account()
    }

    /// Bind the settled form to the plan projection it describes.
    ///
    /// This is the JOIN seat: the artifact's bytes ARE the plan's, so the set stands wherever the
    /// plan does, however pre-contact the form's own answers were
    /// (`306b:rul-projections-continue-influence-flow`).
    #[must_use]
    pub fn with_plan(
        self,
        plan_sh: String,
        plan_account: dorc_core::influence::InfluenceAccount,
    ) -> ArtifactSet {
        ArtifactSet {
            form: self.form,
            fallback: self.fallback,
            account: self.account().join(plan_account),
            primary: ArtifactFile {
                path: PRIMARY_NAME.to_owned(),
                bytes: plan_sh,
            },
            dependencies: self.dependencies,
            topology: self.topology,
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
pub fn select_for_terminal_render(
    snapshot: &crate::snapshot::StaticLoadSnapshot,
    projection: &BundleProjection,
    loads: &[BookLoad],
) -> Selection {
    let inline_debt = loads
        .iter()
        .filter(|load| !(load.absorbable && load.permits.may_rewrite()))
        .count()
        .max(usize::from(
            inline_imports(snapshot, projection, loads).is_none(),
        ));
    match inline_imports(snapshot, projection, loads) {
        Some(imports) => Selection {
            form: ArtifactForm::Flattened,
            fallback: None,
            dependencies: Vec::new(),
            imports,
            placements: placements(projection, loads, Carriage::AbsorbedOnly),
            // EMPTY IS EXACT HERE, not unknown: a form that places no file beside the plan has one
            // published path, so its every relation is vacuously absent rather than unrecorded.
            topology: ArtifactTopology::default(),
        },
        None => Selection {
            form: ArtifactForm::PreservedBookTree,
            fallback: Some(FormFallback::InliningUnproven { loads: inline_debt }),
            dependencies: Vec::new(),
            imports: Vec::new(),
            placements: placements(projection, loads, Carriage::Nothing),
            topology: ArtifactTopology::default(),
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
    snapshot: &crate::snapshot::StaticLoadSnapshot,
    projection: &BundleProjection,
    loads: &[BookLoad],
    request: FormRequest,
    posture: StreamPosture,
) -> Result<Selection, FormRefusal> {
    // A book with nothing to load is ALREADY one stream; one whose every bundle can stand where its
    // `.` stands becomes one (`floor30-inline-dot-boundary`'s measured cell).
    let inlined = inline_imports(snapshot, projection, loads);
    let inline_debt = loads
        .iter()
        .filter(|load| !(load.absorbable && load.permits.may_rewrite()))
        .count()
        .max(usize::from(inlined.is_none()));
    let multipart = match posture {
        StreamPosture::PipedArtifact | StreamPosture::TerminalRender => Err(None),
        StreamPosture::Materializable => bundle_files(snapshot, projection, loads).map_err(Some),
    };

    let preserved = |fallback: Option<FormFallback>| Selection {
        form: ArtifactForm::PreservedBookTree,
        fallback,
        dependencies: Vec::new(),
        imports: Vec::new(),
        placements: placements(projection, loads, Carriage::Nothing),
        topology: ArtifactTopology::default(),
    };
    let flattened = |imports: Vec<ImportEdit>| Selection {
        form: ArtifactForm::Flattened,
        fallback: None,
        dependencies: Vec::new(),
        imports,
        placements: placements(projection, loads, Carriage::AbsorbedOnly),
        topology: ArtifactTopology::default(),
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
            Ok((dependencies, imports, topology)) => Ok(Selection {
                form: ArtifactForm::Multipart,
                fallback: None,
                dependencies,
                imports,
                placements: whole_root(),
                topology,
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
            StreamPosture::Materializable => match mirrored_files(snapshot, projection, loads) {
                Ok((dependencies, topology)) => Ok(Selection {
                    form: ArtifactForm::MirroredTree,
                    fallback: None,
                    dependencies,
                    imports: Vec::new(),
                    placements: whole_root(),
                    topology,
                }),
                Err(unplaceable) => Err(FormRefusal::Unavailable {
                    form: ArtifactForm::MirroredTree,
                    because: FormFallback::DependencyUnplaceable { loads: unplaceable },
                }),
            },
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
            StreamPosture::TerminalRender => {
                Ok(select_for_terminal_render(snapshot, projection, loads))
            }
            StreamPosture::Materializable => Ok(match multipart {
                Ok((dependencies, imports, topology)) => Selection {
                    form: ArtifactForm::Multipart,
                    fallback: None,
                    dependencies,
                    imports,
                    placements: whole_root(),
                    topology,
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
        ArtifactForm, ArtifactSet, ArtifactTopology, FormFallback, FormRefusal, FormRequest,
        ImportEdit, Placement, PlacementReason, Selection, StdoutPosture, StreamPosture,
        artifact_stream, mirrored, placeable, select,
    };
    use crate::bundle::BundleProjection;
    use dorc_core::loadpath::Cwd;

    /// A topology naming a file the set does not publish refuses AS ITSELF, rather than as one of
    /// the image model's structural refusals.
    ///
    /// Unreachable from [`select`], which builds both halves in one act — so this pins the
    /// defensive arm, and pins that it stays distinguishable. The two halves failing closed in
    /// ways that look alike is what would make "it was rejected" satisfied by several different
    /// bugs.
    #[test]
    fn a_topology_naming_an_unpublished_path_refuses_as_itself() {
        let set = Selection {
            form: ArtifactForm::Multipart,
            fallback: None,
            dependencies: Vec::new(),
            imports: Vec::new(),
            placements: dorc_plan::PlacedSources::all_ambient(),
            topology: ArtifactTopology {
                roots: vec!["never-published.sh".to_owned()],
                edges: Vec::new(),
            },
        }
        .with_plan(
            "#!/bin/sh\n:\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );

        assert_eq!(
            crate::apply::image_of_artifact_set(&set, &dorc_receipt::limits::ReceiptLimits::V1),
            Err(
                crate::apply::ImageCarriageRefusal::TopologyNamesUnpublishedPath {
                    path: "never-published.sh".to_owned(),
                }
            ),
            "the refusal names the path, so a reader is not sent looking for a malformed entry"
        );
    }

    fn empty() -> BundleProjection {
        BundleProjection::default()
    }

    fn assemble(
        plan_sh: &str,
        loads: &[super::BookLoad],
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<ArtifactSet, FormRefusal> {
        let snapshot = crate::snapshot::StaticLoadSnapshot::over(
            Cwd::default(),
            Vec::new(),
            Vec::new(),
            &crate::snapshot::LoadPositions::roots_only(),
            "book.sh",
            "",
        );
        select(&snapshot, &empty(), loads, request, posture).map(|selection| {
            selection.with_plan(
                plan_sh.to_owned(),
                dorc_core::influence::InfluenceAccount::authored_before_contact(),
            )
        })
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
            permits: super::LoadPermission::of(true, true),
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
        loads[0].permits = super::LoadPermission::of(false, true);
        let settled = |request, posture| select(&snapshot, &projection, &loads, request, posture);
        let multipart = settled(FormRequest::Auto, StreamPosture::Materializable)
            .expect("the dependency is placeable at its authored path");
        assert_eq!(multipart.form(), ArtifactForm::Multipart);
        assert!(
            multipart.imports().is_empty(),
            "an inexplicit operand is not ours to re-say: {:?}",
            multipart.imports()
        );
        let published = multipart.with_plan(
            String::new(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
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

    /// The disclosure half of the ladder: a kept-in-place bundle says WHICH condition kept it, and
    /// the import edit and the placement account are minted from one seat so they cannot disagree.
    ///
    /// CFG SHAPE: two books, one top-level `.` each — one whole-line and redirect-free (inside the
    /// measured absorbable cell, so what keeps it is that the ladder has not answered), one as an
    /// `||` right operand (outside that cell, so the shape is what keeps it).
    #[test]
    fn a_kept_in_place_bundle_says_which_condition_kept_it() {
        let oracle = || {
            (
                vec!["wombat.dorc.sh".to_owned()],
                vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            )
        };
        let (paths, srcs) = oracle();
        let absorbable = book_sourced(
            ". ./wombat.dorc.sh\nwombat sync a.conf\n",
            paths,
            srcs,
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        assert_eq!(
            absorbable.imports().first().map(ImportEdit::reason),
            Some(&PlacementReason::KeptInPlaceLadderUnconsulted)
        );

        let (paths, srcs) = oracle();
        let outside = book_sourced(
            "false || . ./wombat.dorc.sh\nwombat sync a.conf\n",
            paths,
            srcs,
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("multipart does not need the absorbable cell");
        assert_eq!(
            outside.imports().first().map(ImportEdit::reason),
            Some(&PlacementReason::KeptInPlaceShapeUnmeasured)
        );
    }

    /// TARGET (`30Ng` §7 T1, human-typed; `30P:rul-front-lift-is-the-planners-first-consumer`): a
    /// bundle nothing in the book contends with joins the LIFTED section ahead of the book, by pure
    /// code motion, so the attention-preserving partition the single stream owes its reader is
    /// partition-by-LAYOUT rather than an oracle ocean interleaved with their own mutative lines.
    ///
    /// CFG SHAPE: a top-level, whole-line, redirect-free, assignment-free `.` with NOTHING above it —
    /// so no book statement observes or mutates any name the bundle binds before the load, the
    /// bundle's own top level reads no book variable, and the unit carries no dynamism opener at all.
    /// Every T1 condition holds and the placement is still `InPlace`.
    #[test]
    fn a_clean_bundle_hoists_ahead_of_the_book() {
        let selection = book_sourced(
            ". ./wombat.dorc.sh\nwombat sync a.conf\n",
            vec!["wombat.dorc.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        let decided = carried(&selection);
        internal_tooling::xfail::xfail_until("p-x-front-hoist-lifts-a-clean-bundle", || {
            assert_eq!(
                decided
                    .iter()
                    .map(|decision| (decision.placement().clone(), decision.why().clone()))
                    .collect::<Vec<_>>(),
                vec![(Placement::Hoist, PlacementReason::HoistedAsIs)],
                "nothing contends with this bundle, so the lift is pure code motion"
            );
        });
    }

    /// TARGET (`30Ng` §7 T2, human-typed, as narrowed by `30Qb:tc-t2-is-narrower-than-the-ladder-says`
    /// and ruled to T2a): where the ONLY collision is a ROLE function — whose every reference is
    /// engine-emitted, so the rename stays header-only (`28R:rul-munge-oracle-names-only`) — the
    /// bundle still lifts, under a munged name. A helper or a file-level constant would need
    /// alpha-rename, which is RESERVED (`d-alpha-rename-equivalence`), and falls to T3 instead.
    ///
    /// CFG SHAPE: a top-level funcdef binding the package's own role name, ABOVE a top-level
    /// whole-line `.` of the package that binds it too, and the described site below both.
    #[test]
    fn a_colliding_role_name_hoists_under_a_munge() {
        let selection = book_sourced(
            "wombat__is_converged() { hork ;}\n. ./wombat.dorc.sh\nwombat sync a.conf\n",
            vec!["wombat.dorc.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("auto always lands somewhere");
        let decided = carried(&selection);
        internal_tooling::xfail::xfail_until(
            "p-x-front-hoist-munges-a-colliding-role-name",
            || {
                assert!(
                    decided.iter().all(|decision| matches!(
                        (decision.placement(), decision.why()),
                        (Placement::Hoist, PlacementReason::HoistedMunged { .. })
                    )),
                    "the one collision is a role name, so the lift survives under a munge: {decided:?}"
                );
            },
        );
    }

    /// Every placement decision this selection took over a book-reached source, in source order.
    fn carried(selection: &Selection) -> Vec<dorc_plan::PlacementDecision> {
        (0..8_u32)
            .filter_map(|index| {
                match selection
                    .emission()
                    .placements()
                    .of(dorc_core::SourceFileId(index))
                {
                    dorc_plan::SourcePlacement::Carried(decision) => Some(decision.clone()),
                    _ => None,
                }
            })
            .collect()
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

    /// THE VERSION-MISMATCH CELL (`30I` §2.2's guarded-source idiom; promoted from the pin
    /// `p-x-sentinel-value-conjunct`).
    ///
    /// The world: `common` assigns `sm_common_loaded='v1'`, and `alpha`'s include guard tests for
    /// `'v2'`. A real shell compares the VALUES, finds them different, and takes the SOURCE arm —
    /// so common is loaded a SECOND time. The recognition reads the target closure's own assigned
    /// value against the compared literal to say so (`30I:rul-load-semantics-stay-full-fidelity`);
    /// reading only whether the closure's NAMES are bound answered `Reused`, because common really
    /// was pre-sourced first, and recorded a load that never runs where sh runs one.
    ///
    /// Why it belongs to the artifact forms: a form asks the account "what does this program load",
    /// and an account that answers `Reused` where sh sources is an account a flattened artifact
    /// could act on by omitting the re-source. The disposition was safe only because flattening
    /// refuses to inline at all, and that is not a thing to leave holding a corner.
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
        assert_eq!(
            routes,
            vec![dorc_analysis::load::LoadRoute::Taken],
            "sh compares 'v1' against 'v2' and takes the SOURCE arm, so the guarded `.` really \
             runs — whatever the environment's names say about the target's closure"
        );
    }

    /// Build a whole world over a BOOK-sourced tree, and settle a form over it.
    fn book_sourced(
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<Selection, FormRefusal> {
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
    ) -> Result<Selection, FormRefusal> {
        let (snapshot, projection, loads) = world_at(cwd, book, paths, srcs);
        select(&snapshot, &projection, &loads, request, posture)
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
        let loads = super::book_loads(&cfg, &ast, book, &projection, &env);
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
        let set = selection.with_plan(
            "#!/bin/sh\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
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
        let set = selection.with_plan(
            "#!/bin/sh\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(
            paths,
            ["plan.sh", "wombat.oracle.dorc-bundle.sh"],
            "the destination is the operand's own spelling, recovered through the cwd"
        );
    }

    /// AN ORDINARY SH INCLUSION IS MIRRORED AT ITS AUTHORED PATH, AND ITS `.` IS NEVER RE-SAID
    /// (`30P:principle-book-code-source-is-inclusion`, r30's `mech-acquire-and-ship-plain-sh`).
    ///
    /// Three answers, and each is a separate ruling. The file lands under the SPELLING ITS AUTHOR
    /// USED, not a generated bundle name, because Dorc composed nothing out of it and has nothing
    /// to re-say. Its bytes are VERBATIM, because they are book-class and the strip's job is
    /// erasing a dialect this file never claimed (`two-surfaces`). And no import edit mints,
    /// because the authored `.` already names the file correctly at that path — which is also the
    /// conservative side of `30P:rul-rewrite-permission-is-derived`, whose fence the import-edit
    /// seat carries.
    #[test]
    fn a_plain_sh_inclusion_is_mirrored_verbatim_and_never_re_said() {
        let selection = book_sourced(
            ". ./helpers.sh\nplain_helper_step\n",
            vec!["helpers.sh".to_owned()],
            vec!["plain_helper_step() {\n   wombat note done\n}\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("a relative inclusion is placeable");
        assert_eq!(selection.form(), ArtifactForm::Multipart);
        assert!(
            selection.imports().is_empty(),
            "the authored `.` already names the file at the path it lands on: {:?}",
            selection.imports()
        );
        let set = selection.with_plan(
            "#!/bin/sh\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(
            paths,
            ["plan.sh", "helpers.sh"],
            "the author's own spelling, never a generated bundle name"
        );
        assert_eq!(
            set.dependencies()[0].bytes,
            "plain_helper_step() {\n   wombat note done\n}\n",
            "book-class bytes are mirrored verbatim"
        );
    }

    /// …and the CONTROL for the ruling that governs the rewrite: a dorc-lang dependency named
    /// through a literal-assigned book-set ROOT is EXPLICIT, so its import IS re-said
    /// (`30P:rul-rewrite-permission-is-derived`: EXACT governs authority, EXPLICITNESS governs
    /// rewriting).
    ///
    /// The pair is what makes the rewrite permission observable rather than incidental: the same
    /// artifact form re-points one and leaves the other verbatim, and the discriminator is what the
    /// operand is, never whether the placement happened to be convenient. Today every operand that
    /// reaches the import-edit seat resolved through controller-known TEXT, so the fence there is
    /// vacuous; the seat carries it in a comment so a widened load-head evaluator has to visit it.
    #[test]
    fn a_book_set_root_is_explicit_enough_to_re_point() {
        let selection = book_sourced(
            "OPS_LIB=.\n. \"$OPS_LIB/wombat.oracle.sh\"\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::Materializable,
        )
        .expect("a root-relative dependency is placeable");
        assert_eq!(selection.form(), ArtifactForm::Multipart);
        assert!(
            matches!(
                selection.imports(),
                [ImportEdit::Repoint { path, .. }] if path == "./wombat.oracle.dorc-bundle.sh"
            ),
            "an explicit operand's import names the published bundle: {:?}",
            selection.imports()
        );
    }

    /// A book that includes ordinary sh cannot be given the FLATTENED form, and the refusal names
    /// it rather than quietly publishing a smaller answer.
    ///
    /// Pasting an inclusion into one stream is `mech-paste-plain-sh-single-stream`, forfeited
    /// behind an exclusion set nobody has welded (`FORFEITS:forfeit-plain-sh-inclusion-analysis`):
    /// a top-level `return` in the included file leaves the file under a `.` and would leave the
    /// PLAN under a paste. `KNOBS:kBACKFLIPS` is welded to verbatim-or-refuse, so the most a
    /// construct may cost its author is single-stream — never support.
    #[test]
    fn a_plain_sh_inclusion_refuses_the_flattened_form_by_name() {
        let refusal = book_sourced(
            ". ./helpers.sh\nplain_helper_step\n",
            vec!["helpers.sh".to_owned()],
            vec!["plain_helper_step() {\n   wombat note done\n}\n".to_owned()],
            FormRequest::Explicit(ArtifactForm::Flattened),
            StreamPosture::TerminalRender,
        )
        .expect_err("one stream cannot carry an inclusion");
        assert_eq!(refusal.form(), "flattened");
        assert_eq!(refusal.cause(), "inlining-unproven");
        assert_eq!(refusal.loads(), 1);

        let kept = book_sourced(
            ". ./helpers.sh\nplain_helper_step\n",
            vec!["helpers.sh".to_owned()],
            vec!["plain_helper_step() {\n   wombat note done\n}\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::PipedArtifact,
        )
        .expect_err("a KEPT stream carries a complete plan or the run stops");
        assert_eq!(kept.cause(), "incomplete-single-stream");
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
        let set = mirrored_tree.with_plan(
            "#!/bin/sh\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
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
        let set = selection.with_plan(
            "#!/bin/sh\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
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

    // ── No unsoundness below a blind act (`30P:law-no-unsoundness-below-a-blind-act`) ──

    /// The world both cells below settle a form over: a BLIND ACT — a `.` of a file the controller
    /// holds no bytes for, which runs arbitrary sh in the book's own shell — standing above an
    /// ordinary literal dorc-lang load.
    ///
    /// The second operand is LITERAL, so `operand_is_explicit` answers true and every rewrite tier
    /// is open to it. What it is not is EXACT: the blind act may have moved the working directory,
    /// so which file `./wombat.dorc.sh` names on the host is unknown, and rewriting a reference
    /// whose resolution is unknown changes which file the host loads.
    fn below_a_blind_act(
        blind: &str,
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<Selection, FormRefusal> {
        book_sourced(
            &format!("{blind}\n. ./wombat.dorc.sh\nwombat sync a.conf\n"),
            vec!["wombat.dorc.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            request,
            posture,
        )
    }

    /// A blind act whose OPERAND the controller cannot evaluate. The engine already reads this one
    /// as a cwd clobber, so the load below it is already non-EXACT — which is what makes it the
    /// direct assertion beside each pin: the rewrite gate is observable here TODAY, and the pin
    /// beside it is waiting on the seed widening alone, never on the gate.
    const DYNAMIC_BLIND_ACT: &str = ". \"$SITE_PROFILE/rc\"";

    /// The same act with a LITERAL operand — the law's own example. It evaluates perfectly and the
    /// controller holds no bytes for it, which is exactly the species the clobber seed misses.
    const LITERAL_BLIND_ACT: &str = ". /etc/os-release";

    /// TARGET: the line stays VERBATIM in every form — not re-pointed at a bundle, not replaced by
    /// one (`30P:the-load-plane-stays-correct`: "a literal `.` below a `cd`/havoc is NOT pasted and
    /// NOT re-pointed … explicitness alone never licenses a rewrite — the resolution must be EXACT
    /// too").
    ///
    /// Both blind-act SPELLINGS are asserted, and they discharge at different commits. The dynamic
    /// operand already reads as a cwd clobber, so it is what makes the REWRITE GATE observable on
    /// its own; the literal one waits on the clobber seed, which is a separate question about which
    /// acts are blind at all.
    ///
    /// CFG SHAPE: two straight-line top-level `.`s, each the whole of its own line with neither a
    /// redirect nor a leading assignment (so the second is inside `floor30-inline-dot-boundary`'s
    /// measured absorbable cell), and the described mutator below both.
    #[test]
    fn a_load_below_a_blind_act_is_never_re_pointed() {
        let settled = |blind| {
            (
                below_a_blind_act(blind, FormRequest::Auto, StreamPosture::Materializable)
                    .expect("a relative dependency is placeable"),
                below_a_blind_act(blind, FormRequest::Auto, StreamPosture::TerminalRender)
                    .expect("auto always lands somewhere"),
            )
        };
        for blind in [DYNAMIC_BLIND_ACT, LITERAL_BLIND_ACT] {
            let (multipart, one_stream) = settled(blind);
            assert!(
                multipart.imports().is_empty(),
                "multipart below `{blind}`: the author's own operand is what finds the file: {:?}",
                multipart.imports()
            );
            assert_eq!(
                one_stream.form(),
                ArtifactForm::PreservedBookTree,
                "one stream below `{blind}`: absorbing the bundle would replace the line outright"
            );
        }

        let (control, control_stream) = settled(":");
        assert_eq!(
            control.imports().len(),
            1,
            "control: with nothing blind above it the same line IS re-pointed, so the refusals \
             above are the act's doing: {:?}",
            control.imports()
        );
        assert_eq!(
            control_stream.form(),
            ArtifactForm::Flattened,
            "control: and one stream absorbs it"
        );
    }

    /// TARGET: and nothing is SHIPPED for it — no bundle, no mirror
    /// (`30P:law-no-unsoundness-below-a-blind-act`, the nothing-shipped clause: a copy of a file
    /// Dorc cannot prove the author referenced is engine selection). Under the `30Q` §3 D2 re-cut
    /// a cwd-⊤ load kept its acquisition and its mirror and lost only its binding authority; the
    /// law reverses that, and this is the veto-eligible half.
    ///
    /// Its own cell rather than an assertion on the one above, because the two green at different
    /// seats: the rewrite gate is `BookLoad`'s, the carriage gate is `bundle_files`/`mirrored_files`.
    ///
    /// Both spellings again, and here they discharge TOGETHER: refusing the rewrite still mirrors
    /// the target, so neither is green until the carriage gate lands.
    ///
    /// CFG SHAPE: as above.
    #[test]
    fn a_load_below_a_blind_act_ships_no_copy() {
        let placed = |blind| {
            below_a_blind_act(blind, FormRequest::Auto, StreamPosture::Materializable)
                .expect("a relative dependency is placeable")
        };
        for blind in [DYNAMIC_BLIND_ACT, LITERAL_BLIND_ACT] {
            let settled = placed(blind);
            assert!(
                settled.dependencies.is_empty(),
                "below `{blind}` nothing is shipped on a guess about where the run stands: {:?}",
                settled
                    .dependencies
                    .iter()
                    .map(|file| file.path.as_str())
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(
            placed(":").dependencies.len(),
            1,
            "control: with nothing blind above it the same package IS published"
        );
    }
}
