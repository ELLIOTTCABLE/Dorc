//! Resolving a recorded source against the world a reader stands in.
//!
//! A document records what its run acquired: a role, an ordinal, a content digest, a byte
//! length, and — where the projection collected them — a path and a bounded excerpt. A reader
//! comes along later, in a tree that has moved on, and wants to show the source a conclusion was
//! drawn from.
//!
//! Two questions fall out of that, and they are NOT the same question:
//!
//! * how does the recorded source stand against the world now ([`SourceStanding`]), and
//! * what material may a report actually put on screen ([`SourceMaterial`])?
//!
//! Keeping them apart is the point. A file that drifted AND has a recorded excerpt has a true
//! answer to both — show the excerpt, and say the current file no longer matches — and a single
//! flat state would have to drop one of them. Which one it dropped would then depend on an
//! ordering decision nobody would ever see again.
//!
//! Everything here is pure. The reader's edge owns the file access and reports what it SAW; this
//! module owns the classification and reaches for nothing.

use crate::plan::RecordedSource;
use crate::tokens::OpaqueState;

/// What a reader's edge found when it went looking for one recorded source.
///
/// The edge answers only what it observed. It draws no conclusion — in particular it never
/// decides that bytes it read are "the same" source, because that comparison is a digest check
/// and belongs beside the recorded digest rather than beside the file handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentSource<'bytes> {
    /// The edge read something at the recorded path; these are its exact bytes.
    Read(&'bytes [u8]),
    /// Nothing is at the recorded path.
    Absent,
    /// Something is at the recorded path and the edge could not read it.
    Unreadable,
    /// The document does not carry a path, so there was nowhere to look.
    ///
    /// Distinct from [`Absent`](Self::Absent): a document that withheld the path never claimed
    /// the file was gone, and reporting one as the other would blame the world for a projection
    /// decision.
    Unlocated,
}

/// How a recorded source stands against the world the reader is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceStanding {
    /// The file at the recorded path still hashes to the recorded digest.
    CurrentDigestMatch,
    /// A file is there and its content has moved since the run.
    Drifted,
    /// Nothing is at the recorded path.
    Absent,
    /// Something is there and could not be read.
    Unreadable,
    /// The document carries no path, so the world was never consulted.
    Unlocated,
}

impl SourceStanding {
    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::CurrentDigestMatch => "current-digest-match",
            Self::Drifted => "drifted",
            Self::Absent => "absent",
            Self::Unreadable => "unreadable",
            Self::Unlocated => "unlocated",
        }
    }

    /// Whether the current file reproduces what the run read.
    ///
    /// The ONE positive answer, and it is deliberately narrow: everything else — drifted, absent,
    /// unreadable, never located — means a report must not present current bytes as the ones a
    /// conclusion was drawn from.
    #[must_use]
    pub const fn reproduces_the_run(self) -> bool {
        matches!(self, Self::CurrentDigestMatch)
    }
}

/// What material a report may actually show for one recorded source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceMaterial {
    /// The current file, which the digest proves is the one the run read.
    CurrentBytes,
    /// The bounded exact excerpt the document carries.
    RecordedExcerpt,
    /// A bound stopped the excerpt being carried, so the document has nothing to fall back to.
    OmittedByLimit,
    /// Neither the world nor the document offers bytes; only recorded conclusions remain.
    RecordedConclusionsOnly,
}

impl SourceMaterial {
    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::CurrentBytes => "current-bytes",
            Self::RecordedExcerpt => "recorded-excerpt",
            Self::OmittedByLimit => "omitted-by-limit",
            Self::RecordedConclusionsOnly => "recorded-conclusions-only",
        }
    }
}

/// One recorded source, resolved: how it stands, and what can be shown for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedSource {
    standing: SourceStanding,
    material: SourceMaterial,
}

impl ResolvedSource {
    /// How the recorded source stands against the world.
    #[must_use]
    pub const fn standing(self) -> SourceStanding {
        self.standing
    }

    /// What a report may show.
    #[must_use]
    pub const fn material(self) -> SourceMaterial {
        self.material
    }
}

/// Resolve one recorded source against what the reader's edge found.
///
/// The two answers are computed independently, which is what keeps a drifted file with a
/// recorded excerpt able to say both things at once.
///
/// Material follows the documented preference — the current file when its digest still matches,
/// then the document's own bounded excerpt — and the archive tier that would sit between them is
/// deliberately absent rather than stubbed: an arm no caller can reach is a promise the format
/// has not made.
///
/// A digest comparison is exact and case-sensitive: the writer spells lowercase hexadecimal, so
/// anything else is not a spelling difference to be normalized away but a document this reader
/// should not be claiming agreement with.
#[must_use]
pub fn resolve_source(
    recorded: &RecordedSource,
    current: CurrentSource<'_>,
    current_digest: impl FnOnce(&[u8]) -> String,
) -> ResolvedSource {
    let standing = match current {
        CurrentSource::Read(bytes) => {
            if current_digest(bytes) == recorded.digest() {
                SourceStanding::CurrentDigestMatch
            } else {
                SourceStanding::Drifted
            }
        }
        CurrentSource::Absent => SourceStanding::Absent,
        CurrentSource::Unreadable => SourceStanding::Unreadable,
        CurrentSource::Unlocated => SourceStanding::Unlocated,
    };

    let material = if standing.reproduces_the_run() {
        SourceMaterial::CurrentBytes
    } else {
        match recorded.excerpt() {
            OpaqueState::Captured => SourceMaterial::RecordedExcerpt,
            OpaqueState::OmittedLimit => SourceMaterial::OmittedByLimit,
            // Withheld by the projection, never held by the run, or simply not collected: three
            // different reasons the document has no bytes, and none of them is a bound. A report
            // that called them all `omitted-by-limit` would be inventing a cap that never fired.
            OpaqueState::WithheldPlain | OpaqueState::Unavailable | OpaqueState::Uncollected => {
                SourceMaterial::RecordedConclusionsOnly
            }
        }
    };

    ResolvedSource { standing, material }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::SourceSlots;
    use crate::reingested::RecordedInfluence;
    use crate::rows::SourceOrdinal;
    use crate::tokens::{RecordedSourceClass, RecordedSourceRole};

    const RECORDED: &str = "aa";

    fn source(excerpt: OpaqueState) -> RecordedSource {
        RecordedSource::of(
            SourceOrdinal::of(0),
            RecordedSourceRole::Book,
            RECORDED.to_owned(),
            2,
            SourceSlots {
                path: OpaqueState::Captured,
                excerpt,
                content: OpaqueState::Captured,
            },
            RecordedSourceClass::GeneralSh,
            RecordedInfluence::AuthoredBeforeContact,
        )
    }

    /// A stand-in digest: the bytes ARE the spelling, so a test states agreement and drift
    /// directly instead of hard-coding hashes that say nothing to a reader.
    fn spelled(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).into_owned()
    }

    #[test]
    fn a_matching_digest_is_the_one_state_that_shows_current_bytes() {
        let resolved = resolve_source(
            &source(OpaqueState::Uncollected),
            CurrentSource::Read(RECORDED.as_bytes()),
            spelled,
        );
        assert_eq!(resolved.standing(), SourceStanding::CurrentDigestMatch);
        assert!(resolved.standing().reproduces_the_run());
        assert_eq!(resolved.material(), SourceMaterial::CurrentBytes);
    }

    #[test]
    fn drift_and_a_recorded_excerpt_are_both_reported() {
        // The reason standing and material are separate values. A single flat state would have
        // to drop one of these, and a reader shown an excerpt without being told the file moved
        // is being quietly misled about which world they are looking at.
        let resolved = resolve_source(
            &source(OpaqueState::Captured),
            CurrentSource::Read(b"bb"),
            spelled,
        );
        assert_eq!(resolved.standing(), SourceStanding::Drifted);
        assert!(!resolved.standing().reproduces_the_run());
        assert_eq!(resolved.material(), SourceMaterial::RecordedExcerpt);
    }

    #[test]
    fn every_way_of_not_matching_withholds_current_bytes() {
        // The safety direction: only an exact digest agreement may present the file on disk as
        // the one a conclusion was drawn from.
        for current in [
            CurrentSource::Read(b"bb"),
            CurrentSource::Absent,
            CurrentSource::Unreadable,
            CurrentSource::Unlocated,
        ] {
            let resolved = resolve_source(&source(OpaqueState::Uncollected), current, spelled);
            assert!(!resolved.standing().reproduces_the_run());
            assert_ne!(resolved.material(), SourceMaterial::CurrentBytes);
        }
    }

    #[test]
    fn the_world_and_the_document_answer_separately() {
        for (current, standing) in [
            (CurrentSource::Absent, SourceStanding::Absent),
            (CurrentSource::Unreadable, SourceStanding::Unreadable),
            (CurrentSource::Unlocated, SourceStanding::Unlocated),
        ] {
            assert_eq!(
                resolve_source(&source(OpaqueState::Captured), current, spelled).standing(),
                standing,
                "the document's excerpt must not change what the world says"
            );
            assert_eq!(
                resolve_source(&source(OpaqueState::Captured), current, spelled).material(),
                SourceMaterial::RecordedExcerpt,
                "nor the world what the document holds"
            );
        }
    }

    #[test]
    fn only_a_bound_reads_as_a_bound() {
        // Three different reasons a document holds no bytes, and none of them is a cap firing.
        for absent in [
            OpaqueState::WithheldPlain,
            OpaqueState::Unavailable,
            OpaqueState::Uncollected,
        ] {
            assert_eq!(
                resolve_source(&source(absent), CurrentSource::Absent, spelled).material(),
                SourceMaterial::RecordedConclusionsOnly
            );
        }
        assert_eq!(
            resolve_source(
                &source(OpaqueState::OmittedLimit),
                CurrentSource::Absent,
                spelled
            )
            .material(),
            SourceMaterial::OmittedByLimit
        );
    }

    #[test]
    fn a_digest_comparison_is_exact() {
        // Upper-case hexadecimal is not this writer's spelling, so agreeing with it would be
        // agreeing with a document produced by something else.
        let resolved = resolve_source(
            &source(OpaqueState::Uncollected),
            CurrentSource::Read(b"AA"),
            spelled,
        );
        assert_eq!(resolved.standing(), SourceStanding::Drifted);
    }

    #[test]
    fn the_state_words_are_distinct() {
        let standings = [
            SourceStanding::CurrentDigestMatch,
            SourceStanding::Drifted,
            SourceStanding::Absent,
            SourceStanding::Unreadable,
            SourceStanding::Unlocated,
        ];
        for (index, one) in standings.iter().enumerate() {
            for other in standings.iter().skip(index.saturating_add(1)) {
                assert_ne!(one.token(), other.token());
            }
        }
        let materials = [
            SourceMaterial::CurrentBytes,
            SourceMaterial::RecordedExcerpt,
            SourceMaterial::OmittedByLimit,
            SourceMaterial::RecordedConclusionsOnly,
        ];
        for (index, one) in materials.iter().enumerate() {
            for other in materials.iter().skip(index.saturating_add(1)) {
                assert_ne!(one.token(), other.token());
            }
        }
    }
}
