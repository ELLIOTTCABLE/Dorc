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

use dorc_core::Span;
use dorc_core::loadpath::Cwd;

use crate::bundle::{BundleProjection, BundleRootId};

/// The maximum dependency files one artifact set will place.
///
/// Bounds what an authored load graph can ask the edge to write, on
/// `rul-host-bytes-bounded-before-admission`'s reasoning one layer over: the inputs are the
/// operator's own, but a publication is still a write loop and a bound that exists is cheaper than
/// a bound that is argued.
const MAX_DEPENDENCIES: usize = 256;

/// Which of the three semantic emission forms an artifact set is in (`30I` §7.1).
///
/// Ordered by FLATTENING, most first, because that is the order `auto` searches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactForm {
    /// One `plan.sh` and nothing else — what a byte pipe and explicit single-stream intent take.
    Flattened,
    /// `plan.sh` plus its contracted dorc-lang dependencies, mirrored at their authored relative
    /// paths under the artifact root. The intended attention-preserving default.
    Multipart,
    /// The authored source boundaries survive untouched and the artifact set carries no
    /// dependencies: v0 could neither inline them nor place them, so it miscompiles nothing and
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

/// Where this invocation's ARTIFACT goes — the injected, non-hermetic edge fact
/// (`30I:rul-piped-stdout-implies-one-flat-plan`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamPosture {
    /// The artifact stream is stdout: one stream, so one flat plan or nothing.
    SingleStream,
    /// The artifact stream is a directory the run may materialize.
    Materializable,
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

/// Why an explicitly requested form cannot be served — always pre-network, never a silent swap.
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
}

impl FormRefusal {
    /// The form the invocation named.
    #[must_use]
    pub const fn form(self) -> ArtifactForm {
        match self {
            Self::Unavailable { form, .. } | Self::NoArtifactStream { form } => form,
        }
    }

    /// The greppable cause word.
    #[must_use]
    pub const fn cause(self) -> &'static str {
        match self {
            Self::Unavailable { because, .. } => because.cause(),
            Self::NoArtifactStream { .. } => "no-artifact-stream",
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
/// dependency's bytes and is placed by mirroring, not by the planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookLoad {
    /// The `.` command's span in the book's own bytes.
    pub span: Span,
    /// The root bundle the occurrence opened.
    pub root: BundleRootId,
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
    projection: &BundleProjection,
) -> Vec<BookLoad> {
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
        .map(|occurrence| BookLoad {
            span: book.node(cfg.node(occurrence.load().at).ast).span,
            root: occurrence.root(),
        })
        .collect()
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

/// The dependency files a multipart set would place, or the count that made it impossible.
///
/// Deduplicated by DESTINATION: a diamond reaches one file through two occurrences, and both
/// copies are the same stripped bytes of the same source. Two DIFFERENT byte-sets claiming one
/// destination is unplaceable rather than last-wins — an artifact whose dependency depends on
/// which occurrence was walked last is not a projection of anything.
fn dependency_files(
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
        // Unplaceable, never silently skipped: omitting a file the runtime `.` will look for is
        // what the possible-load projection exists to prevent (`30I` §6.1).
        let Some(root) = projection.roots().iter().find(|root| root.id() == *id) else {
            unplaceable = unplaceable.saturating_add(1);
            continue;
        };
        for &id in root.files() {
            let Some(file) = projection.file(id) else {
                continue;
            };
            let authored = snapshot_paths
                .get(file.copied().source().0 as usize)
                .map_or("", String::as_str);
            let Some(destination) = mirrored(cwd, authored) else {
                unplaceable = unplaceable.saturating_add(1);
                continue;
            };
            let bytes = file.copied().text().to_owned();
            match placed.get(&destination) {
                Some(existing) if *existing != bytes => {
                    unplaceable = unplaceable.saturating_add(1);
                }
                Some(_) => {}
                None => {
                    placed.insert(destination, bytes);
                }
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
    // A book with nothing to load is ALREADY one stream: the flattened form is available there.
    let inline_debt = loads.len();
    let flat = inline_debt == 0;
    let multipart = match posture {
        StreamPosture::SingleStream => Err(None),
        StreamPosture::Materializable => {
            dependency_files(cwd, snapshot_paths, projection, loads).map_err(Some)
        }
    };

    let preserved = |fallback: Option<FormFallback>| Selection {
        form: ArtifactForm::PreservedBookTree,
        fallback,
        dependencies: Vec::new(),
    };
    let flattened = Selection {
        form: ArtifactForm::Flattened,
        fallback: None,
        dependencies: Vec::new(),
    };

    match request {
        FormRequest::Explicit(ArtifactForm::Flattened) => {
            flat.then_some(flattened).ok_or(FormRefusal::Unavailable {
                form: ArtifactForm::Flattened,
                because: FormFallback::InliningUnproven { loads: inline_debt },
            })
        }
        FormRequest::Explicit(ArtifactForm::Multipart) => match multipart {
            Ok(dependencies) => Ok(Selection {
                form: ArtifactForm::Multipart,
                fallback: None,
                dependencies,
            }),
            Err(None) => Err(FormRefusal::NoArtifactStream {
                form: ArtifactForm::Multipart,
            }),
            Err(Some(unplaceable)) => Err(FormRefusal::Unavailable {
                form: ArtifactForm::Multipart,
                because: FormFallback::DependencyUnplaceable { loads: unplaceable },
            }),
        },
        FormRequest::Explicit(ArtifactForm::PreservedBookTree) => Ok(preserved(None)),
        // One stream holds only the flat form; a materializable one aims at mode 2 (`30I` §7.1).
        FormRequest::Auto => Ok(match posture {
            StreamPosture::SingleStream if flat => flattened,
            StreamPosture::SingleStream => {
                preserved(Some(FormFallback::InliningUnproven { loads: inline_debt }))
            }
            StreamPosture::Materializable => match multipart {
                Ok(dependencies) => Selection {
                    form: ArtifactForm::Multipart,
                    fallback: None,
                    dependencies,
                },
                Err(unplaceable) => preserved(Some(FormFallback::DependencyUnplaceable {
                    loads: unplaceable.unwrap_or(inline_debt),
                })),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactForm, ArtifactSet, FormFallback, FormRefusal, FormRequest, StreamPosture, mirrored,
        placeable, select,
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

    fn one_load() -> super::BookLoad {
        super::BookLoad {
            span: dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
            root: crate::bundle::BundleRootId::first(),
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
            StreamPosture::SingleStream,
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
            StreamPosture::SingleStream,
        )
        .expect_err("v0 cannot inline a load-inert child safely yet");
        assert_eq!(refusal.form(), ArtifactForm::Flattened);
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
            StreamPosture::SingleStream,
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
            StreamPosture::SingleStream,
        )
        .expect_err("a directory-shaped artifact cannot ride one pipe");
        assert_eq!(refusal.cause(), "no-artifact-stream");
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
        book_sourced_at(Cwd::default(), book, paths, srcs, request, posture)
    }

    /// The same, with the modelled working directory named — the axis production and the in-process
    /// drivers differ on, and therefore the axis a placement rule must be measured across.
    fn book_sourced_at(
        cwd: Cwd,
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
        request: FormRequest,
        posture: StreamPosture,
    ) -> Result<super::Selection, FormRefusal> {
        let reached = crate::snapshot::book_reached(&cwd, &paths, &srcs, book);
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
        let loads = super::book_loads(&cfg, &ast, &projection);
        select(
            &cwd,
            snapshot.source_paths(),
            &projection,
            &loads,
            request,
            posture,
        )
    }

    /// THE MULTIPART PLACEMENT, end to end over a real load: the dependency lands at the SAME
    /// relative path the book's own operand names, carrying STRIPPED bytes.
    ///
    /// Both halves matter. Mirroring is what lets the book's `. ./wombat.oracle.sh` — and every
    /// nested operand inside a copied file — resolve on the target with no rewritten byte and no
    /// generated root variable (`30I` §7.4). Stripping is what lets a stock shell source it at all
    /// (`30Ib` §15: a bundle ships `dorc strip`'s output, which is still pure erasure, so the byte
    /// floor holds).
    #[test]
    fn a_multipart_dependency_lands_at_its_authored_relative_path() {
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
        let set = selection.with_plan("#!/bin/sh\n".to_owned());
        let paths: Vec<&str> = set.files().map(|file| file.path.as_str()).collect();
        assert_eq!(paths, ["plan.sh", "wombat.oracle.sh"]);
        let dependency = &set.dependencies()[0].bytes;
        assert!(
            dependency.contains("wombat__is_converged()") && !dependency.contains(" : sm.dorc.W"),
            "the mirrored dependency is the STRIPPED body a stock shell can source:\n{dependency}"
        );
    }

    /// The same world with stdout as the artifact stream: one stream cannot carry the tree, the
    /// book has a load to inline, and inlining is not floor-measured — so `auto` lands on the
    /// preserved tree and SAYS SO. This is the cell every `--pre-source`-free corpus case with a
    /// book `.` sits in today.
    #[test]
    fn the_same_world_on_one_stream_preserves_the_tree_and_explains() {
        let selection = book_sourced(
            ". ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec!["# dorc-lang/v0.2\nwombat__is_converged() { :; }\n".to_owned()],
            FormRequest::Auto,
            StreamPosture::SingleStream,
        )
        .expect("auto always lands somewhere");
        assert_eq!(selection.form(), ArtifactForm::PreservedBookTree);
        assert_eq!(
            selection.fallback(),
            Some(FormFallback::InliningUnproven { loads: 1 })
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
            Cwd::at("/ops/case"),
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
            ["plan.sh", "wombat.oracle.sh"],
            "the destination is the operand's own spelling, recovered through the cwd"
        );
    }

    /// A dependency the book reached OUTSIDE the load working directory has no relative spelling
    /// under an artifact root, so the form is unavailable rather than fudged — the half of
    /// `need-controller-paths-never-cross-hosts` that a cwd-relative rule must not lose.
    #[test]
    fn a_dependency_outside_the_load_cwd_is_unplaceable() {
        let selection = book_sourced_at(
            Cwd::at("/ops/case"),
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
