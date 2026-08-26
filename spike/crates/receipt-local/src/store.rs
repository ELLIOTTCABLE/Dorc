//! What a publication into the local store PROVED, and how a store can fail.
//!
//! # Why the proof is a set of properties and not a grade
//!
//! Windows has no operation matching a Unix directory synchronization, so the platforms do not
//! sit on one ladder and an `Ord` over grades would invent a comparison the world does not
//! support. The proof therefore records independent PROPERTIES, and a policy check asks whether
//! the ones it requires are present — never whether a number is big enough.
//!
//! The distinction that matters most, and the reason directory synchronization is one property
//! with two negative answers: "this platform has no such operation" and "the operation ran and
//! failed" are different facts about a publication, and collapsing them would let a real failure
//! read as an ordinary platform limit.

/// Whether one publication achieved one property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectorySync {
    /// The containing directory was synchronized.
    Synchronized,
    /// The platform exposes no meaningful operation. Recorded, never simulated.
    UnavailableOnPlatform,
}

/// What one publication actually did.
///
/// Private fields, no `Default`, no `Ord`: a proof exists because operations succeeded, and there
/// is no ordering between two platforms' proofs to derive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationProperties {
    exclusive_final_name_created: bool,
    complete_bytes_written: bool,
    file_synchronized: bool,
    directory: DirectorySync,
}

impl PublicationProperties {
    /// Record what a publication achieved.
    ///
    /// Every property is stated; none defaults. A caller that cannot say whether it synchronized
    /// has to answer the question rather than omit it.
    #[must_use]
    pub const fn of(
        exclusive_final_name_created: bool,
        complete_bytes_written: bool,
        file_synchronized: bool,
        directory: DirectorySync,
    ) -> Self {
        Self {
            exclusive_final_name_created,
            complete_bytes_written,
            file_synchronized,
            directory,
        }
    }

    /// Whether the file itself was exclusively created, fully written, and synchronized.
    #[must_use]
    pub const fn file_is_durable(self) -> bool {
        self.exclusive_final_name_created && self.complete_bytes_written && self.file_synchronized
    }

    /// What happened to the containing directory.
    #[must_use]
    pub const fn directory(self) -> DirectorySync {
        self.directory
    }

    /// Whether this satisfies the required local baseline for `platform`.
    ///
    /// A typed question over properties, never a numeric comparison. The Windows baseline is
    /// EXPLICITLY weaker and says so here rather than being quietly folded into the same answer:
    /// it demands everything Unix does except the directory synchronization the platform does not
    /// offer, and it is not equivalent.
    #[must_use]
    pub const fn meets_required_baseline(self, platform: PlatformBaseline) -> bool {
        if !self.file_is_durable() {
            return false;
        }
        match platform {
            PlatformBaseline::UnixLike => matches!(self.directory, DirectorySync::Synchronized),
            PlatformBaseline::Windows => {
                matches!(self.directory, DirectorySync::UnavailableOnPlatform)
            }
        }
    }
}

/// Which platform's honest baseline a proof is being read against.
///
/// Two arms, and no third for "whichever": the required properties differ, so a caller has to say
/// which world it is in rather than being handed a portable answer that is true nowhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformBaseline {
    /// Unix and macOS on a local filesystem: the directory synchronization is required.
    UnixLike,
    /// Windows: the file is flushed and the directory operation does not exist.
    Windows,
}

/// Why a publication did not happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishFailure {
    /// The bytes are larger than one receipt may be. Checked before the filesystem is touched.
    OverReceiptBound,
    /// A file already exists under the exact final name. Never replaced.
    NameAlreadyTaken,
    /// The store root is not a validated directory this process may write to.
    RootUnusable,
    /// The exclusive create did not happen.
    CreateFailed,
    /// Some bytes were written and some were not. No retry into another name.
    WriteIncomplete,
    /// Synchronization failed. Never retried: a second call can report success over pages the
    /// kernel already discarded.
    SyncFailed,
}

/// Why an enumeration did not produce a bounded listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumerateFailure {
    /// The store root is not a validated directory.
    RootUnusable,
    /// The walk found more entries than the bound admits.
    ///
    /// A fact the walk OBSERVED — it goes to the bound plus one — rather than a silence at the
    /// boundary that would read as a complete short listing.
    OverEntryBound,
    /// The platform refused the walk.
    WalkFailed,
}

/// Why one entry could not be read back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreReadFailure {
    /// The entry is no longer there. Enumeration is not a snapshot.
    Vanished,
    /// The entry is not a regular file, or is a link or reparse point. Never followed.
    NotARegularFile,
    /// The entry is larger than one receipt may be, measured independently of whatever wrote it.
    OverReceiptBound,
    /// The platform refused the read.
    ReadFailed,
}

/// What a recognized entry turned out to be, once read.
///
/// `IncompletePublication` is the one that earns its place: publication creates the final name
/// directly, so a crash can leave a prefix on disk, and such a file can never parse as complete —
/// it has no signature trailer. What it CANNOT say is which side of the crash it is on, and the
/// arm's own payload says so rather than guessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryStanding {
    /// The entry read back whole and bounded. Whether it is a valid receipt is the reader's
    /// question, not the store's.
    CompleteBytes,
    /// The entry is a prefix of a document. Presence alone cannot say whether a writer is still
    /// working or stopped.
    IncompletePublication {
        /// The one honest reading.
        state: IncompleteState,
    },
    /// The entry's name is not a V1 receipt name. It counts against the walk and mints nothing.
    UnrecognizedName,
}

/// What an incomplete entry can be said to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncompleteState {
    /// Either a publication still running or one that stopped. Presence cannot distinguish them,
    /// and this arm is the refusal to pretend otherwise.
    InProgressOrAbandoned,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn everything(directory: DirectorySync) -> PublicationProperties {
        PublicationProperties::of(true, true, true, directory)
    }

    #[test]
    fn the_two_platform_baselines_are_not_interchangeable() {
        // The point of refusing an ordering. A Windows proof does not satisfy the Unix baseline,
        // and — just as load-bearing — a Unix proof does not satisfy the Windows one either: a
        // proof claiming a directory synchronization on a platform that has none is describing
        // some other machine.
        let unix = everything(DirectorySync::Synchronized);
        let windows = everything(DirectorySync::UnavailableOnPlatform);
        assert!(unix.meets_required_baseline(PlatformBaseline::UnixLike));
        assert!(windows.meets_required_baseline(PlatformBaseline::Windows));
        assert!(!windows.meets_required_baseline(PlatformBaseline::UnixLike));
        assert!(!unix.meets_required_baseline(PlatformBaseline::Windows));
    }

    #[test]
    fn every_missing_file_property_defeats_the_baseline_on_both_platforms() {
        // Exhaustive over the three file properties rather than one representative: each is
        // separately required, and a check that read only the last of them would pass a
        // publication that never created its own name.
        for (which, properties) in [
            (
                "no exclusive create",
                PublicationProperties::of(false, true, true, DirectorySync::Synchronized),
            ),
            (
                "incomplete write",
                PublicationProperties::of(true, false, true, DirectorySync::Synchronized),
            ),
            (
                "no file sync",
                PublicationProperties::of(true, true, false, DirectorySync::Synchronized),
            ),
        ] {
            assert!(
                !properties.meets_required_baseline(PlatformBaseline::UnixLike),
                "{which}"
            );
            assert!(
                !properties.meets_required_baseline(PlatformBaseline::Windows),
                "{which}"
            );
        }
    }

    #[test]
    fn a_platform_without_the_operation_is_not_a_platform_that_failed_it() {
        // There is no `DirectorySync::Failed` arm, and that is deliberate: a failed
        // synchronization fails the whole publication (`PublishFailure::SyncFailed`) rather than
        // being recorded as a weaker proof. Only the platform-limit answer survives into a proof.
        assert_ne!(
            DirectorySync::Synchronized,
            DirectorySync::UnavailableOnPlatform
        );
        assert_ne!(PublishFailure::SyncFailed, PublishFailure::WriteIncomplete);
    }
}
