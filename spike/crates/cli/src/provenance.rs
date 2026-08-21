//! `provenance` — locators built from what the run's own loader did (`30I` §9).
//!
//! The representation is `aid::locator`'s arbitrary DAG; this is the seat that FILLS it from the
//! load model, so a consumer asking "where did this come from" is answered by the same structure
//! the engine loaded through rather than by a second account of it.
//!
//! # What this can say today, and what it cannot
//!
//! Bundle copies compose onto the authored/load chain without rebuilding identity. Nested load
//! acts come directly from the loader's occurrence account, and root book loci are the account's
//! `CfgNodeId` resolved through the already-built CFG.

use dorc_aid::locator::{BundleOriginClaim, GeneratedLocus, Locator, SourceLocus, Stage, StageId};
use dorc_analysis::cfg::Cfg;
use dorc_analysis::load::{LoadAccount, LoadSourcer};
use dorc_core::{BytePos, SourceFileId, Span};

use crate::bundle::{BundleFile, BundleFileId, BundleProjection};
use crate::snapshot::StaticLoadSnapshot;

/// The loader-owned locus of every occurrence, without target or source collapse.
///
/// Built once per run from the settled environment. A file the invocation named directly appears
/// in no entry: nothing loaded it, it was simply there before line 1, and saying otherwise would
/// invent a line for a reader to go and look at.
#[derive(Debug, Clone, Default)]
pub struct LoadActs {
    by_occurrence: Vec<Option<SourceLocus>>,
}

impl LoadActs {
    /// Read the acts off a settled environment.
    #[must_use]
    pub fn of(
        snapshot: &StaticLoadSnapshot,
        cfg: &Cfg,
        book: &dorc_syntax::Ast,
        account: &LoadAccount,
    ) -> Self {
        let book_file = SourceFileId(u32::try_from(snapshot.book_index()).unwrap_or(u32::MAX));
        let by_occurrence = account
            .occurrences()
            .iter()
            .map(|occurrence| match &occurrence.sourcer {
                LoadSourcer::Invocation => None,
                LoadSourcer::Book => Some(SourceLocus::at(
                    book_file,
                    book.node(cfg.node(occurrence.at).ast).span,
                )),
                LoadSourcer::File(key) => {
                    let file = snapshot.source_at_key(key)?;
                    let file = SourceFileId(u32::try_from(file).ok()?);
                    Some(SourceLocus::at(file, occurrence.locus?))
                }
            })
            .collect();
        Self { by_occurrence }
    }

    /// Compose one generated bundle range through its exact occurrence to authored source.
    #[must_use]
    pub fn locator_for_bundle(
        &self,
        snapshot: &StaticLoadSnapshot,
        projection: &BundleProjection,
        file: BundleFileId,
        original_span: Span,
    ) -> Option<(Locator, StageId)> {
        let mut locator = Locator::default();
        let occurrence = projection.file(file)?.occurrence();
        let head = self.push_bundle_range(
            &mut locator,
            snapshot,
            projection,
            occurrence,
            original_span,
        )?;
        Some((locator, head))
    }

    fn push_bundle_range(
        &self,
        locator: &mut Locator,
        snapshot: &StaticLoadSnapshot,
        projection: &BundleProjection,
        occurrence: usize,
        original_span: Span,
    ) -> Option<StageId> {
        let projected = projection.occurrences().get(occurrence)?;
        let file = projection.file(projected.file())?;
        let authored = locator.push(
            Stage::Authored(SourceLocus::at(file.copied().source(), original_span)),
            &[],
        );
        let mut origins = Vec::new();
        if let Some(load_locus) = self.by_occurrence.get(occurrence).copied().flatten() {
            let load_origins = projected
                .load()
                .within
                .and_then(|parent| {
                    self.push_bundle_range(locator, snapshot, projection, parent, load_locus.span)
                })
                .into_iter()
                .collect::<Vec<_>>();
            origins.push(locator.push(Stage::Loaded(load_locus), &load_origins));
        }
        origins.push(authored);
        let src = snapshot
            .source_srcs()
            .get(file.copied().source().0 as usize)?;
        let generated_span = generated_span(file, src, original_span)?;
        Some(locator.push(
            Stage::Copied(GeneratedLocus::at(file.storage_path(), generated_span)),
            &origins,
        ))
    }
}

/// An owned frame ready for the production diagnostic renderer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatorFrame {
    /// Display path or generated archive path.
    pub filename: String,
    /// Exact bytes whose coordinates `span` uses.
    pub source: String,
    /// Byte range in `source`.
    pub span: Span,
}

/// Resolve generated and load stages into renderable frames, leaving the diagnostic's authored
/// primary frame to the ordinary diagnostic seat.
#[must_use]
pub fn locator_frames(
    locator: &Locator,
    head: StageId,
    snapshot: &StaticLoadSnapshot,
    projection: &BundleProjection,
) -> Vec<LocatorFrame> {
    locator
        .resolve(head)
        .into_iter()
        .filter_map(|stage| match stage {
            Stage::Authored(_) | Stage::Claimed(_) => None,
            Stage::Loaded(locus) => source_frame(snapshot, *locus),
            Stage::Copied(at) | Stage::Generated(at) => projection
                .files()
                .iter()
                .find(|file| file.storage_path() == at.artifact)
                .map(|file| LocatorFrame {
                    filename: at.artifact.clone(),
                    source: file.render_sh(true),
                    span: at.span,
                }),
        })
        .collect()
}

fn source_frame(snapshot: &StaticLoadSnapshot, locus: SourceLocus) -> Option<LocatorFrame> {
    let file = locus.file.0 as usize;
    Some(LocatorFrame {
        filename: snapshot.source_paths().get(file)?.clone(),
        source: snapshot.source_srcs().get(file)?.clone(),
        span: locus.span,
    })
}

fn generated_span(file: &BundleFile, original: &str, span: Span) -> Option<Span> {
    let (original_line, _) = dorc_aid::diag::line_col(original, span.lo.0 as usize);
    let output_line = file
        .copied()
        .line_map()
        .iter()
        .position(|&line| line as usize == original_line)?;
    let (line_start, line_len) = line_bounds(file.copied().text(), output_line)?;
    // The strip map proves line identity, not columns; frame the line rather than invent a span.
    let prefix = file.copied_offset(true);
    let lo = prefix.saturating_add(line_start);
    let hi = lo.saturating_add(line_len);
    Some(Span::new(
        BytePos(u32::try_from(lo).ok()?),
        BytePos(u32::try_from(hi).ok()?),
    ))
}

fn line_bounds(text: &str, wanted: usize) -> Option<(usize, usize)> {
    let mut start = 0usize;
    for (line, part) in text.split_inclusive('\n').enumerate() {
        let len = part.strip_suffix('\n').map_or(part.len(), str::len);
        if line == wanted {
            return Some((start, len));
        }
        start = start.saturating_add(part.len());
    }
    (wanted == 0 && text.is_empty()).then_some((0, 0))
}

/// Result of checking a comment-origin suggestion against caller-supplied current bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginResolution {
    /// No candidate bytes were supplied for the claim.
    Absent,
    /// Candidate bytes existed but did not produce this exact copied segment.
    Mismatch,
    /// Exact captured content identity agreed; the source locus may be shown as resolved.
    Matching(SourceLocus),
}

/// Caller-supplied bytes for resolving a generated comment without performing I/O here.
#[derive(Debug, Clone, Copy)]
pub struct OriginCandidate<'a> {
    /// Path the claim suggested.
    pub path: &'a str,
    /// Aid-plane identity assigned to the candidate by the caller.
    pub source: SourceFileId,
    /// Bytes captured by the bundle-generating run.
    pub bundled_bytes: &'a str,
    /// Bytes available at the later read.
    pub current_bytes: &'a str,
}

/// Resolve an aid-only origin claim using injected bytes, never an ambient file read.
#[must_use]
pub fn resolve_origin_claim(
    claim: &BundleOriginClaim,
    file: &BundleFile,
    candidate: Option<OriginCandidate<'_>>,
) -> OriginResolution {
    let Some(candidate) = candidate else {
        return OriginResolution::Absent;
    };
    if candidate.path != claim.as_claimed() {
        return OriginResolution::Absent;
    }
    if candidate.current_bytes != candidate.bundled_bytes {
        return OriginResolution::Mismatch;
    }
    let stripped = dorc_oracle::strip::strip_file_with_map(
        &mut dorc_core::Interner::default(),
        candidate.bundled_bytes,
    );
    if stripped.value.text != file.copied().text() {
        return OriginResolution::Mismatch;
    }
    OriginResolution::Matching(SourceLocus::at(
        candidate.source,
        Span::new(
            BytePos(0),
            BytePos(u32::try_from(candidate.bundled_bytes.len()).unwrap_or(u32::MAX)),
        ),
    ))
}

/// Build the honest later-read locator: generated bytes always remain primary, the comment remains
/// claimed, and authored source joins only after exact content identity agrees.
#[must_use]
pub fn locator_for_origin_claim(
    file: &BundleFile,
    claim: BundleOriginClaim,
    candidate: Option<OriginCandidate<'_>>,
) -> (Locator, StageId, OriginResolution) {
    let resolution = resolve_origin_claim(&claim, file, candidate);
    let mut locator = Locator::default();
    let claimed = locator.push(Stage::Claimed(claim), &[]);
    let mut origins = vec![claimed];
    if let OriginResolution::Matching(authored) = resolution {
        origins.push(locator.push(Stage::Authored(authored), &[]));
    }
    let lo = file.copied_offset(true);
    let hi = lo.saturating_add(file.copied().text().len());
    let generated = GeneratedLocus::at(
        file.storage_path(),
        Span::new(
            BytePos(u32::try_from(lo).unwrap_or(u32::MAX)),
            BytePos(u32::try_from(hi).unwrap_or(u32::MAX)),
        ),
    );
    let head = locator.push(Stage::Copied(generated), &origins);
    (locator, head, resolution)
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
    const LOCUS_CAP: usize = 4096;
    let source_locus = |locus: &SourceLocus| {
        let file = locus.file.0 as usize;
        let path = snapshot.source_paths().get(file)?;
        let src = snapshot.source_srcs().get(file)?;
        let (line, _) = dorc_aid::diag::line_col(src, locus.span.lo.0 as usize);
        Some(dorc_aid::display::encode_foreign(
            &format!("{path}:{line}"),
            LOCUS_CAP,
        ))
    };
    locator
        .resolve(head)
        .into_iter()
        .filter_map(|stage| match stage {
            Stage::Authored(locus) | Stage::Loaded(locus) => source_locus(locus),
            Stage::Copied(at) | Stage::Generated(at) => {
                Some(dorc_aid::display::encode_foreign(&at.artifact, LOCUS_CAP))
            }
            Stage::Claimed(claim) => Some(dorc_aid::display::encode_foreign(
                claim.as_claimed(),
                LOCUS_CAP,
            )),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use dorc_aid::locator::{BundleOriginClaim, Stage};
    use dorc_core::{BytePos, Interner, SourceFileId, Span};

    use super::{
        LoadActs, LocatorFrame, OriginCandidate, OriginResolution, locator_for_origin_claim,
        locator_frames, render_chain, resolve_origin_claim,
    };
    use crate::bundle::{BundleProjection, project};
    use crate::snapshot::{StaticLoadSnapshot, book_reached};

    const MARKER: &str = "# dorc-lang/v0.2\n";

    struct World {
        snapshot: StaticLoadSnapshot,
        acts: LoadActs,
        bundle: BundleProjection,
    }

    fn world(book: &str, paths: Vec<String>, srcs: Vec<String>) -> World {
        let cwd = dorc_core::loadpath::Cwd::default();
        let reached = book_reached(&cwd, &paths, &srcs, book);
        let snapshot = StaticLoadSnapshot::over(
            cwd,
            paths,
            srcs,
            &crate::snapshot::LoadPositions::book_sourced(reached),
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
        let acts = LoadActs::of(&snapshot, &cfg, &ast, env.loads());
        let bundle = project(&snapshot, env.loads())
            .expect("closed occurrence forest")
            .into_projection();
        World {
            snapshot,
            acts,
            bundle,
        }
    }

    #[test]
    fn a_book_load_maps_through_generated_storage_to_authored_bytes() {
        let package = format!("{MARKER}\nsm_q() {{ common \"$@\" ;}}\n");
        let world = world(
            "OPS_LIB=.\n. \"$OPS_LIB/pkg.sh\"\nsm_q first\n",
            vec!["pkg.sh".to_owned()],
            vec![package.clone()],
        );
        let body = package.find("sm_q()").expect("the package declares it");
        let span = Span::new(
            BytePos(u32::try_from(body).unwrap_or(0)),
            BytePos(u32::try_from(body).unwrap_or(0) + 4),
        );
        let file = world.bundle.roots()[0].entry();
        let (locator, head) = world
            .acts
            .locator_for_bundle(&world.snapshot, &world.bundle, file, span)
            .expect("the occurrence has a locator");
        let chain = render_chain(&locator, head, &world.snapshot);
        assert_eq!(chain[1..], ["book.sh:2", "pkg.sh:3"]);
        assert!(chain[0].starts_with("dorc-bundle/v0/root-00000000/"));
    }

    #[test]
    fn a_transitive_dependency_retains_every_generated_and_load_locus() {
        let dep = format!("{MARKER}\ndep_q() {{ common \"$@\" ;}}\n");
        let world = world(
            ". ./entry.sh\n",
            vec!["entry.sh".to_owned(), "dep.sh".to_owned()],
            vec![format!("{MARKER}. ./dep.sh\n"), dep.clone()],
        );
        let body = dep.find("dep_q").expect("dependency body");
        let span = Span::new(
            BytePos(u32::try_from(body).unwrap_or(0)),
            BytePos(u32::try_from(body).unwrap_or(0) + 5),
        );
        let dep_file = world.bundle.occurrences()[1].file();
        let (locator, head) = world
            .acts
            .locator_for_bundle(&world.snapshot, &world.bundle, dep_file, span)
            .expect("transitive locator");
        let resolved = locator.resolve(head);
        assert_eq!(
            resolved
                .iter()
                .filter(|stage| matches!(stage, Stage::Copied(_)))
                .count(),
            2
        );
        assert_eq!(
            resolved
                .iter()
                .filter(|stage| matches!(stage, Stage::Loaded(_)))
                .count(),
            2
        );
        assert!(render_chain(&locator, head, &world.snapshot).contains(&"dep.sh:3".to_owned()));
    }

    #[test]
    fn the_production_diagnostic_render_keeps_generated_and_original_frames() {
        let dep = format!(
            "{MARKER}\nrunas__lend_map() {{\n   printf '%s\\n' \"$1\" : lends frobnicate\n}}\n"
        );
        let world = world(
            ". ./entry.sh\n",
            vec!["entry.sh".to_owned(), "dep.sh".to_owned()],
            vec![format!("{MARKER}. ./dep.sh\n"), dep.clone()],
        );
        let mut validation_interner = Interner::default();
        let validation = dorc_oracle::validate::validate(&mut validation_interner, &[&dep]);
        let diag = validation
            .stages
            .iter()
            .flat_map(|stage| &stage.diags)
            .find(|diag| diag.code.slug() == "lend-map-unknown-dimension")
            .expect("the production validation diagnostic");
        let span = diag.primary.span().expect("the diagnostic is source-sited");
        let file = world.bundle.occurrences()[1].file();
        let (locator, head) = world
            .acts
            .locator_for_bundle(&world.snapshot, &world.bundle, file, span)
            .expect("transitive locator");
        let mut owned = locator_frames(&locator, head, &world.snapshot, &world.bundle);
        owned.push(LocatorFrame {
            filename: "hostile\nforged\u{202e}".to_owned(),
            source: "safe\u{1b}[31m forged\n".to_owned(),
            span: Span::new(BytePos(0), BytePos(4)),
        });
        let borrowed: Vec<_> = owned
            .iter()
            .map(|frame| dorc_aid::diag::DiagnosticFrame {
                filename: &frame.filename,
                source: &frame.source,
                span: frame.span,
            })
            .collect();
        let rendered = dorc_aid::diag::render_staged_cli_parts_with_frames(
            "bundle",
            &dorc_aid::RenderCtx::production(),
            diag,
            &dep,
            "dep.sh",
            &borrowed,
            &Interner::default(),
        )
        .text();
        assert!(rendered.contains("dep.sh:4:"));
        assert!(rendered.contains("dorc-bundle/v0/root-00000000/occurrence-00000001.sh:4:"));
        assert!(rendered.contains("entry.sh:2:"));
        assert!(rendered.contains("book.sh:1:"));
        assert!(!rendered.contains('\u{1b}'));
        assert!(!rendered.contains("\nforged"));
        assert!(rendered.contains("hostile\\x0aforged\\xe2\\x80\\xae"));
    }

    #[test]
    fn absent_and_changed_claim_candidates_never_resolve_as_current_source() {
        let source = format!(
            "{MARKER}thing__is_converged() {{\n   thing status : sm.dorc.Thing:@ready\n}}\n"
        );
        let changed = source.replace("@ready", "@changed");
        assert_eq!(
            dorc_oracle::strip::strip_file(&mut Interner::default(), &source).value,
            dorc_oracle::strip::strip_file(&mut Interner::default(), &changed).value,
            "the changed annotation is invisible in stripped bytes"
        );
        let world = world(
            ". ./thing.sh\n",
            vec!["thing.sh".to_owned()],
            vec![source.clone()],
        );
        let file = world.bundle.file(world.bundle.roots()[0].entry()).unwrap();
        let claim = BundleOriginClaim::of("thing.sh");
        assert_eq!(
            resolve_origin_claim(&claim, file, None),
            OriginResolution::Absent
        );
        assert_eq!(
            resolve_origin_claim(
                &claim,
                file,
                Some(OriginCandidate {
                    path: "thing.sh",
                    source: SourceFileId(7),
                    bundled_bytes: &source,
                    current_bytes: &changed,
                }),
            ),
            OriginResolution::Mismatch
        );
        assert!(matches!(
            resolve_origin_claim(
                &claim,
                file,
                Some(OriginCandidate {
                    path: "thing.sh",
                    source: SourceFileId(7),
                    bundled_bytes: &source,
                    current_bytes: &source,
                })
            ),
            OriginResolution::Matching(_)
        ));
        for candidate in [
            None,
            Some(OriginCandidate {
                path: "thing.sh",
                source: SourceFileId(7),
                bundled_bytes: &source,
                current_bytes: &changed,
            }),
        ] {
            let (locator, head, resolution) =
                locator_for_origin_claim(file, claim.clone(), candidate);
            assert!(!matches!(resolution, OriginResolution::Matching(_)));
            assert!(matches!(locator.resolve(head)[0], Stage::Copied(_)));
            assert!(
                locator
                    .resolve(head)
                    .iter()
                    .all(|stage| !matches!(stage, Stage::Authored(_)))
            );
        }
    }

    #[test]
    fn two_roots_and_diamond_occurrences_keep_distinct_chains() {
        let world = world(
            ". ./a.sh\n. ./b.sh\n",
            vec!["a.sh".into(), "b.sh".into(), "shared.sh".into()],
            vec![
                format!("{MARKER}. ./shared.sh\n"),
                format!("{MARKER}. ./shared.sh\n"),
                format!("{MARKER}shared() {{ :; }}\n"),
            ],
        );
        let shared: Vec<_> = world
            .bundle
            .occurrences()
            .iter()
            .filter(|occurrence| occurrence.load().target == "shared.sh")
            .collect();
        assert_eq!(shared.len(), 2);
        let shared_span = Span::new(
            BytePos(u32::try_from(MARKER.len()).unwrap_or(0)),
            BytePos(u32::try_from(MARKER.len()).unwrap_or(0).saturating_add(6)),
        );
        let chains: Vec<_> = shared
            .iter()
            .map(|occurrence| {
                let (locator, head) = world
                    .acts
                    .locator_for_bundle(
                        &world.snapshot,
                        &world.bundle,
                        occurrence.file(),
                        shared_span,
                    )
                    .expect("diamond locator");
                render_chain(&locator, head, &world.snapshot)
            })
            .collect();
        assert_ne!(chains[0][0], chains[1][0]);
        assert!(chains[0].contains(&"book.sh:1".to_owned()));
        assert!(chains[1].contains(&"book.sh:2".to_owned()));
    }

    #[test]
    fn bundle_claim_composition_names_no_authority_type() {
        let source = include_str!("provenance.rs");
        let body: String = source
            .split_once("mod tests {")
            .map_or(source, |(head, _)| head)
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect();
        for forbidden in [
            "ReplaceLicense",
            "GuardLicense",
            "VerdictVouch",
            "ByVouch",
            "ByObservation",
            "DefinitionId",
            "DefinitionCustody",
            "FactKey",
            "Verdict",
        ] {
            assert!(!body.contains(forbidden), "provenance names `{forbidden}`");
        }
    }
}
