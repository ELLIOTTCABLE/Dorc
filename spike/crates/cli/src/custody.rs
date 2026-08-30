//! What the run already holds that a receipt may carry: exact general-sh bytes, and one locator
//! per decided site (`30Ra:planning-book-bytes-and-durable-locators`).
//!
//! # Nothing here reads anything
//!
//! Every byte this hands the projection came out of the snapshot the analysis already ran over.
//! There is no path argument and no file access, which is how "persistence expands no observation"
//! is kept true by construction rather than by review.
//!
//! # The class boundary is the marker, and only the marker
//!
//! `marker-gates-syntax-only` makes the `# dorc-lang/v0.2` marker the discriminator everywhere
//! else in the tree, and `snapshot::role_of` already splits `BookSourced` from `PlainInclusion` on
//! exactly this call. Reusing it is what keeps one answer to "which dialect is this file".
//! Deliberately NOT `sourcing::satisfies_the_contract`, which additionally demands load-inertness:
//! that is the contract a `.` operand must meet, a different question from which dialect a file was
//! accepted as, and folding the two would withhold the bytes of a marked file that merely runs
//! something at load time.

use std::collections::BTreeMap;

use dorc_plan::receipt::{RecordedInputs, SourceCustody};
use dorc_receipt::durable_locator::{DurableLocator, DurableStage, RecordedStageKind};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::rows::SourceOrdinal;

use crate::snapshot::StaticLoadSnapshot;

/// The custody of every acquired source, in the invocation's own source order.
///
/// Order is the whole of the correspondence: the projection walks
/// `SpineInvocation::sources()` and indexes this by position, so a vector built in a different
/// order would attach one file's bytes to another file's identity.
#[must_use]
pub fn source_custody(snapshot: &StaticLoadSnapshot) -> Vec<SourceCustody<'_>> {
    snapshot
        .source_srcs()
        .iter()
        .map(|src| {
            if dorc_oracle::marker::has_marker(src) {
                SourceCustody::dorc_lang()
            } else {
                SourceCustody::general_sh(src)
            }
        })
        .collect()
}

/// One locator per decided site, naming the authored bytes the decision was about.
///
/// V1 records the stage a `dorc why <line>` question actually resolves against: the site's own
/// authored span in the book. That is not a simplification of the DAG — it is the honest depth of
/// what the decision plane knows about a book site, which descends from nothing. A site whose
/// bytes arrived through a `.` composes a `Loaded` stage above its authored one, and the
/// representation already carries that shape; what is missing is the per-site source identity to
/// build it from, which lives in the loader rather than in `SpineDisposition`.
///
/// Sites whose span the book's own arena cannot answer are ABSENT from the map rather than
/// carrying a locator over a guessed range: the projection reads an absent entry as uncollected,
/// and an uncollected locator is a slot a reader knows to distrust. A fabricated span would be a
/// slot a reader would trust.
#[must_use]
pub fn site_locators(
    spine: &dorc_plan::Spine,
    book: &dorc_syntax::Ast,
    book_ordinal: usize,
    limits: &ReceiptLimits,
) -> BTreeMap<dorc_core::SiteId, DurableLocator> {
    let Ok(ordinal) = u32::try_from(book_ordinal) else {
        return BTreeMap::new();
    };
    let ordinal = SourceOrdinal::of(ordinal);
    spine
        .dispositions()
        .filter_map(|record| {
            // `Ast::node` indexes and panics past the arena, so the bounds check is the caller's
            // (`syntax`'s own documented contract for an id that may not belong to this arena).
            let ast = record.ast();
            let span = (usize::try_from(ast.0).ok()? < book.len()).then(|| book.node(ast).span)?;
            let stage = DurableStage::in_source(
                RecordedStageKind::Authored,
                ordinal,
                (u64::from(span.lo.0), u64::from(span.hi.0)),
                Vec::new(),
            )
            .ok()?;
            let locator = DurableLocator::of(vec![stage], 0, limits).ok()?;
            Some((record.site(), locator))
        })
        .collect()
}

/// Everything one publication may carry about the run's inputs.
#[must_use]
pub fn recorded_inputs<'a>(
    snapshot: &'a StaticLoadSnapshot,
    spine: &dorc_plan::Spine,
    book: &dorc_syntax::Ast,
    limits: &ReceiptLimits,
) -> RecordedInputs<'a> {
    RecordedInputs::of(
        source_custody(snapshot),
        site_locators(spine, book, snapshot.book_index(), limits),
    )
}
