//! The ONE source-comparison packet this crate releases (`30Vd:the-source-comparison-seat`).
//!
//! # What this is, and what it deliberately is not
//!
//! This crate validates and classifies recorded source material. It does not compare anything, does
//! not touch a filesystem, and does not decide what "the same source" means — every one of those is
//! POLICY, and policy that lives here would be policy nobody could change without reopening a
//! sealed crate. So the split is: this file releases one narrowly typed packet per recorded source,
//! and exactly one seat in the CLI owns path rehydration, bounded reads, comparison, the
//! same-physical-line rule, and destination encoding.
//!
//! It is SOURCE-SPECIFIC on purpose. A generic "give me the raw detail" accessor would have been
//! smaller and would have been the widening `inv-report-is-the-public-read-boundary` exists to
//! refuse: what leaves here is a source's own material, under its own types, to one caller.
//!
//! # Nothing here formats
//!
//! Every byte-carrying member is a [`RecordedValue`], whose only exit is a `ValueEncoder`. Nothing
//! in this file gains `Display`, a revealing `Debug`, serde, or an ambient conversion, and the
//! visit hands out borrows rather than owned copies so a consumer cannot accumulate a second store
//! of somebody's source.

use super::states::{AuthenticationState, MaterialState};
use super::value::RecordedValue;
use super::{SiteFacts, SourceFacts};
use crate::rows::RecordedSite;
use crate::tokens::RecordedSourceClass;

/// Every recorded source of one document, as the comparison seat may see them.
///
/// A borrow of the model rather than a copy: the packet exists to be walked once, at the seat, and
/// an owned form would be a second place a document's sources live.
#[derive(Debug)]
pub struct RecordedSourceMaterial<'a> {
    pub(crate) sources: &'a [SourceFacts],
    pub(crate) sites: &'a [SiteFacts],
    pub(crate) authentication: AuthenticationState,
}

impl RecordedSourceMaterial<'_> {
    /// Release each recorded source to the one comparison consumer, in the document's own order.
    ///
    /// A VISIT rather than an accessor returning a collection: what a consumer gets is scoped to
    /// one call, so the borrowed material cannot outlive the walk and be filed away somewhere the
    /// encoder obligation does not reach.
    pub fn visit_for_comparison(&self, consumer: &mut dyn SourceComparisonConsumer) {
        for source in self.sources {
            consumer.accept(&SourceComparison {
                source,
                placements: self.placements_naming(source.ordinal),
                authentication: self.authentication,
            });
        }
    }

    /// Every authored placement whose stage names `ordinal`, in site order.
    ///
    /// The AUTHORED stage only: a generated or loaded stage's span is stated against bytes the
    /// author never typed, so offering one here would let a line number be computed against the
    /// wrong text (`30Rb:book-content-and-locator-projection`).
    fn placements_naming(&self, ordinal: u32) -> Vec<SourcePlacement> {
        self.sites
            .iter()
            .filter_map(|site| {
                let authored = site.authored_origin()?;
                if authored.source() != Some(ordinal) {
                    return None;
                }
                Some(SourcePlacement {
                    site: site.site(),
                    span: authored.span()?,
                })
            })
            .collect()
    }
}

/// One recorded site's authored placement inside one source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePlacement {
    /// Which recorded site.
    pub site: RecordedSite,
    /// Its authored byte span, in the acquired byte domain.
    pub span: (u64, u64),
}

/// One recorded source, as the comparison seat receives it.
#[derive(Debug)]
pub struct SourceComparison<'a> {
    source: &'a SourceFacts,
    placements: Vec<SourcePlacement>,
    authentication: AuthenticationState,
}

impl SourceComparison<'_> {
    /// Where the source sat in the acquired-source table.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.source.ordinal()
    }

    /// Which dialect the run accepted it as — the boundary that decided byte custody.
    #[must_use]
    pub const fn class(&self) -> RecordedSourceClass {
        self.source.class()
    }

    /// Its content digest at load time, as spelled. A controller-minted digest, never host material.
    #[must_use]
    pub fn digest(&self) -> &str {
        self.source.digest()
    }

    /// Whether its exact bytes are in the document.
    #[must_use]
    pub const fn content_state(&self) -> MaterialState {
        self.source.content()
    }

    /// Whether its path is in the document.
    #[must_use]
    pub const fn path_state(&self) -> MaterialState {
        self.source.path()
    }

    /// The exact recorded path, where the document carries it. Encoder-mediated like every value.
    #[must_use]
    pub const fn path(&self) -> Option<&RecordedValue> {
        self.source.path_text()
    }

    /// The exact recorded content, where the document carries it. Encoder-mediated.
    #[must_use]
    pub const fn content(&self) -> Option<&RecordedValue> {
        self.source.text()
    }

    /// Every authored placement inside this source.
    #[must_use]
    pub fn placements(&self) -> &[SourcePlacement] {
        &self.placements
    }

    /// What outer verification said about the DOCUMENT this source was recorded in.
    ///
    /// It rides the source rather than being looked up beside it because the consumer's read policy
    /// turns on it: a receipt-provided path is followed only for material this controller
    /// authenticated, and a consumer that had to remember to ask would be one edit from forgetting.
    #[must_use]
    pub const fn authentication(&self) -> AuthenticationState {
        self.authentication
    }

    /// The byte offset each physical line of the recorded content starts at, first line first.
    ///
    /// OFFSETS, never content: this is what lets a seat turn a locator span into a line NUMBER
    /// without the bytes leaving through anything but an encoder. Empty where the document carries
    /// no content, which is the honest answer — a line number computed against absent text would be
    /// a number about nothing.
    #[must_use]
    pub fn line_starts(&self) -> Vec<u64> {
        self.source
            .text()
            .map(RecordedValue::line_starts)
            .unwrap_or_default()
    }
}

/// The obligation the ONE comparison seat satisfies.
///
/// Implemented OUTSIDE this crate, on [`super::ValueEncoder`]'s footing and for the same reason:
/// everything the implementation does — resolving a path on a platform, reading a file, deciding
/// what correspondence means — is exactly what this crate must not do. There is one implementation
/// and one call site, and because Rust cannot say that about a public trait, the roster that says
/// it is `receipt/tests/crate_boundary.rs`'s.
pub trait SourceComparisonConsumer {
    /// Receive one recorded source.
    fn accept(&mut self, source: &SourceComparison<'_>);
}
