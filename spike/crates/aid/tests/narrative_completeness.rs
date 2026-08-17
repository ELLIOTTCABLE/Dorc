//! The collapse-class MINT COMPLETENESS gate (`289:rul-mint-hardening-package` items 1 and 2).
//!
//! `collapse-mints-narrative` says every safety-narrowing mints a decision-inert record. Nothing
//! mechanically held the schedule to that: `CollapseNarrative::new` is demanded at each collapse
//! CONSTRUCTOR, but no gate said "this class has a constructor call somewhere". Two instruments,
//! deliberately cheap:
//!
//! 1. a no-wildcard `match CollapseKind` mapping each variant to its expected census marker — the
//!    compiler forces every future collapse class to visit this file before it can land;
//! 2. a `diag_tidy`-style lexical census asserting each constructible class is minted at ≥1 site in
//!    the mint crates.
//!
//! # The unrendered eight (`289:seam-narrative-render-unconsumed`, dated 2026-07-24)
//!
//! Only `VerdictDecline` carrying an `authored_reason` reaches a user surface (one `why:` line via
//! `emit_static_decline_notes`). The why-chain is built from `SurvivalWitness` and license
//! derivations; `emit_why_lens` takes the narrative slice as `_collapse_narrative` and ignores it by
//! signature. So a MISSING narrative today omits SILENTLY — there is no `Unexplained` class and no
//! self-advertising render. The mint-side assertions below are therefore the whole instrument for
//! eight of nine classes; the render-side assertion exists for the one renderable class. Building a
//! narrative render to close that gap is the deferred d4 arrangement walker's work (phase 7's
//! arrangement-home sitting), not this gate's.
//!
//! # The escalation seam, deliberately UNBUILT
//!
//! If this census ever leaks a real under-narration, the priced next rung is value-carriage in the
//! lattice join (`289:rul-mint-hardening-package`). Nothing is built toward it.

#![expect(
    clippy::expect_used,
    reason = "tidy-gate harness over the repo's own source tree; the no-panic lints guard \
              untrusted-input paths"
)]

use std::path::{Path, PathBuf};

use dorc_aid::diag::SolvePass;
use dorc_aid::narrative::{
    ChannelCoverage, CollapseKind, DeclineGate, DefinitionSite, DemoteTag, EntryDegradeTag,
    EntryFailureTag, FailedCheck, MintSpan, Operands, SolverRounds, ValueOperand, WrapperPairTag,
};
use dorc_core::{BytePos, Channel, LeafId, SiteId, SourceFileId, Span};

/// The crates that MINT (`288` §2e): the nine sites live in `analysis` (1), `plan` (3), `cli` (5).
const MINT_CRATES: &[&str] = &["analysis", "plan", "cli"];

/// The census marker for one collapse class: the literal `CollapseKind::<Variant>` text a mint site
/// must spell. Both the bare and the `dorc_aid::`-qualified spellings appear in-tree, so the scan
/// matches the suffix.
fn census_marker(kind: &CollapseKind) -> &'static str {
    match kind {
        CollapseKind::FactMergeDisagreement { .. } => "CollapseKind::FactMergeDisagreement",
        CollapseKind::VerdictDecline { .. } => "CollapseKind::VerdictDecline",
        CollapseKind::WallFormation { .. } => "CollapseKind::WallFormation",
        CollapseKind::SubstitutionRefusal { .. } => "CollapseKind::SubstitutionRefusal",
        CollapseKind::EntryDenial { .. } => "CollapseKind::EntryDenial",
        CollapseKind::WrapperPairIncoherent { .. } => "CollapseKind::WrapperPairIncoherent",
        CollapseKind::EntryFailure { .. } => "CollapseKind::EntryFailure",
        CollapseKind::Demotion { .. } => "CollapseKind::Demotion",
        CollapseKind::RenderRefusal { .. } => "CollapseKind::RenderRefusal",
        CollapseKind::FixpointCapDegrade { .. } => "CollapseKind::FixpointCapDegrade",
        CollapseKind::RoleFamilyShadowed { .. } => "CollapseKind::RoleFamilyShadowed",
        CollapseKind::SolverConsistencyFailure { .. } => "CollapseKind::SolverConsistencyFailure",
        CollapseKind::CompositionSuspended { .. } => "CollapseKind::CompositionSuspended",
        CollapseKind::ProjectionDrop { .. } => "CollapseKind::ProjectionDrop",
        CollapseKind::Cancellation(reserved) => match *reserved {},
    }
}

fn site(leaf: u32) -> SiteId {
    SiteId::leaf(LeafId(leaf))
}

fn definition(file: u32) -> DefinitionSite {
    DefinitionSite {
        file: SourceFileId(file),
        name: MintSpan(Span::new(BytePos(0), BytePos(1))),
    }
}

/// Every CONSTRUCTIBLE class, one value each. `Cancellation` is uninhabited and cannot appear here;
/// the exhaustive match in [`census_marker`] is what keeps it (and every future class) accounted for.
fn constructible_classes() -> Vec<CollapseKind> {
    vec![
        CollapseKind::FactMergeDisagreement {
            cell: site(0),
            operands: Operands::<ValueOperand>::default(),
        },
        CollapseKind::VerdictDecline {
            site: site(0),
            arm: MintSpan(Span::new(BytePos(0), BytePos(1))),
            arm_file: SourceFileId(1),
            gate: DeclineGate::Return,
            authored_reason: None,
        },
        CollapseKind::WallFormation {
            participant: LeafId(0),
            channel: ChannelCoverage {
                channel: Channel::StatusRelaxable,
            },
        },
        CollapseKind::SubstitutionRefusal {
            site: site(0),
            top_channel: Channel::StatusRelaxable,
        },
        CollapseKind::EntryDenial {
            rung: EntryDegradeTag::NoCapability,
        },
        CollapseKind::WrapperPairIncoherent {
            class: WrapperPairTag::PeelDepth,
        },
        CollapseKind::EntryFailure {
            site: site(0),
            class: EntryFailureTag::Refused,
        },
        CollapseKind::Demotion {
            site: site(0),
            reason: DemoteTag::TotalWall,
        },
        CollapseKind::render_refusal_heredoc(site(0)),
        CollapseKind::FixpointCapDegrade {
            rounds: 1,
            discarded: 0,
        },
        CollapseKind::RoleFamilyShadowed {
            prior: definition(0),
            shadowing: definition(1),
        },
        CollapseKind::SolverConsistencyFailure {
            pass: SolvePass::ValueFlow,
            operands: Operands::<FailedCheck>::default(),
            shown: 0,
            total: 1,
            solves: 1,
            advisory: SolverRounds {
                converged: true,
                rounds: 4,
            },
        },
        CollapseKind::CompositionSuspended {
            site: site(0),
            vouching: MintSpan(Span::new(BytePos(0), BytePos(1))),
            vouching_file: SourceFileId(0),
            reason: dorc_aid::diag::VouchedCompositionReason::BookRedefinesHelper,
        },
        CollapseKind::ProjectionDrop {
            projection: "whylog",
            species: "SpineSurvival",
            dropped: 2,
        },
    ]
}

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/aid has a parent (crates/)")
        .to_path_buf()
}

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn mint_source() -> String {
    let crates = crates_dir();
    let mut files = Vec::new();
    for name in MINT_CRATES {
        rs_files(&crates.join(name).join("src"), &mut files);
    }
    let mut out = String::new();
    for file in files {
        if let Ok(text) = std::fs::read_to_string(&file) {
            out.push_str(&text);
            out.push('\n');
        }
    }
    out
}

/// Every constructible collapse class is minted somewhere in the mint crates.
///
/// Two disclosed limits, the same ones `diag_tidy` carries. NEEDLE-SHAPE: this greps the literal
/// `CollapseKind::<Variant>` text, so a class built into a variable and passed on is invisible.
/// `#[cfg(test)]`-BLINDNESS: `rs_files` walks every `.rs` under `src/`, test modules included, so a
/// test-only construction in a mint crate satisfies the census even if the production mint died. It
/// is a belt-and-braces backstop against a class that is never narrated AT ALL, not a liveness
/// proof — the per-class fault-injection pins are the liveness instrument.
#[test]
fn every_constructible_collapse_class_is_minted() {
    let source = mint_source();
    for kind in constructible_classes() {
        let marker = census_marker(&kind);
        assert!(
            source.contains(marker),
            "collapse class `{marker}` is never minted in analysis/plan/cli — every \
             safety-narrowing mints a decision-inert narrative carrying its operands \
             (AID-NEEDS:law-collapse-mints-narrative). Mint it at the collapse constructor, or \
             delete the class."
        );
    }
}

/// The exhaustive match is the real gate: adding a `CollapseKind` variant fails to compile here
/// until it is given a census marker, which forces the author past the census above. This test is
/// the marker's own sanity check (distinct, non-empty, correctly-prefixed).
#[test]
fn each_class_carries_a_distinct_census_marker() {
    let mut markers: Vec<&str> = constructible_classes().iter().map(census_marker).collect();
    let count = markers.len();
    markers.sort_unstable();
    markers.dedup();
    assert_eq!(markers.len(), count, "two classes share a census marker");
    for marker in markers {
        assert!(
            marker.starts_with("CollapseKind::"),
            "`{marker}` is not the literal mint spelling"
        );
    }
}
