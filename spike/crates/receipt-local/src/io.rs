//! The private I/O vocabulary, and the deterministic model that stands in for it.
//!
//! # One operation per security-relevant act
//!
//! The vocabulary is deliberately not a filesystem. It is the list of acts this crate PROMISES —
//! exclusive creation, ownership inspection, bounded reads, synchronization, a bounded walk — and
//! nothing else. There is no `write_file(path, bytes)` convenience, because a convenience that
//! collapses create, ownership, write, and sync into one call is one that no failure schedule can
//! interrupt between them, and every interesting interruption is exactly there.
//!
//! Conversely it does not model every syscall. The boundaries modelled are the ones the crate
//! makes a claim about; anything finer would be a second filesystem with its own bugs.
//!
//! # Why the trait is sealed
//!
//! Production and the model implement the SAME trait, so the failure sweep exercises the code
//! that ships rather than a parallel copy of it. [`LocalIo`] is SEALED by a private supertrait,
//! so no type outside this crate can implement it — a production route cannot be handed a
//! filesystem from somewhere else, and that is a property of the type rather than of a
//! convention. The vocabulary it speaks in is public, because naming an act is not performing
//! one.

use crate::store::DirectorySync;

/// The seal. Private, so naming [`LocalIo`] from outside is possible and implementing it is not.
mod sealed {
    /// Implemented for this crate own filesystem surfaces and nothing else.
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

/// One act this crate performs against a filesystem.
///
/// Closed and named for the act rather than for the call: the schedule below faults by operation,
/// so what the vocabulary distinguishes is exactly what a test can interrupt between.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    /// Create a directory exclusively, with the private policy this platform can honour.
    CreateDirectoryExclusive,
    /// Create a file exclusively, with the private policy this platform can honour.
    CreateFileExclusive,
    /// Open an existing file without following a final-component redirect.
    OpenExistingNoFollow,
    /// Read bytes under a bound the caller supplied.
    ReadBounded,
    /// Write every byte, or fail.
    WriteAll,
    /// Synchronize a file.
    SyncFile,
    /// Synchronize a directory, or answer that the platform has no such operation.
    SyncDirectory,
    /// Walk a directory to a bound.
    EnumerateBounded,
    /// Inspect an already-open object: its type, identity, owner, and mode.
    InspectOpened,
    /// Remove an object this attempt created and still owns.
    RemoveOwned,
}

impl Op {
    /// Every operation, in one order — the schedule's own iteration order.
    pub const ALL: [Self; 10] = [
        Self::CreateDirectoryExclusive,
        Self::CreateFileExclusive,
        Self::OpenExistingNoFollow,
        Self::ReadBounded,
        Self::WriteAll,
        Self::SyncFile,
        Self::SyncDirectory,
        Self::EnumerateBounded,
        Self::InspectOpened,
        Self::RemoveOwned,
    ];

    /// Whether this operation can change what a later process finds on disk.
    ///
    /// The property the failure sweep is organized around: for a DURABLE operation, failing
    /// before it and failing after it leave different disks, and both have to land in a closed
    /// outcome. For the rest the two are the same interruption.
    #[must_use]
    pub const fn is_durable(self) -> bool {
        match self {
            Self::CreateDirectoryExclusive
            | Self::CreateFileExclusive
            | Self::WriteAll
            | Self::SyncFile
            | Self::SyncDirectory
            | Self::RemoveOwned => true,
            Self::OpenExistingNoFollow
            | Self::ReadBounded
            | Self::EnumerateBounded
            | Self::InspectOpened => false,
        }
    }
}

/// How an operation failed.
///
/// Not `std::io::Error`: this crate's callers branch on these distinctions, and an errno would
/// make every one of them a string comparison. A platform error is mapped into one of these at
/// the production edge, which is the only place that knows what an errno meant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoFault {
    /// The object was already there. Never a licence to replace it.
    AlreadyExists,
    /// The object was not there.
    NotFound,
    /// The platform refused on permissions or ownership.
    Denied,
    /// The object was a link, junction, or reparse point where one is not followed.
    Redirect,
    /// The object was not the kind expected — a directory where a file was, or the reverse.
    WrongKind,
    /// The bound was reached.
    OverBound,
    /// Some bytes moved and some did not.
    Partial,
    /// The platform refused for a reason this vocabulary does not distinguish.
    ///
    /// The one open arm, and it is deliberately unhelpful: a caller cannot branch usefully on it,
    /// so a distinction that matters has to be given its own name above rather than being read
    /// out of this one.
    Platform,
}

/// Which side of an operation a fault was injected on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    /// The operation never happened. For a durable operation, the disk is unchanged.
    Before,
    /// The operation happened and then the attempt failed. For a durable operation, the disk
    /// carries its effect and the caller was told it did not succeed.
    After,
}

/// One scheduled interruption: fault operation `op` on side `side`, the `nth` time it is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScheduledFault {
    /// Which operation.
    pub op: Op,
    /// Which side of it.
    pub side: Side,
    /// Which occurrence, counting from zero.
    pub occurrence: usize,
    /// What the caller is told.
    pub fault: IoFault,
}

/// A deterministic plan of interruptions, consumed as the operations are reached.
///
/// The failure sweep's skeleton: a schedule is built once, driven, and the resulting modelled
/// disk is restarted from. Nothing here is random, and nothing reads a clock — two runs of one
/// schedule interrupt at exactly the same places.
#[derive(Debug, Clone, Default)]
pub struct FailureSchedule {
    faults: Vec<ScheduledFault>,
    reached: Vec<(Op, Side)>,
}

impl FailureSchedule {
    /// A schedule that interrupts nothing.
    #[must_use]
    pub fn intact() -> Self {
        Self::default()
    }

    /// A schedule that faults `op` on `side` the first time it is reached.
    ///
    /// The shape the sweep is built from: for every durable operation, one schedule failing
    /// before it and one failing after it.
    #[must_use]
    pub fn faulting(op: Op, side: Side, fault: IoFault) -> Self {
        Self::faulting_occurrence(op, side, 0, fault)
    }

    /// A schedule that faults `op` on `side` the `occurrence`-th time it is reached.
    ///
    /// One operation reached several times in one sequence is several different interruptions —
    /// the trailing directory synchronization is not the one beside a create — so a sweep that
    /// could only name the first would leave the later ones untested.
    #[must_use]
    pub fn faulting_occurrence(op: Op, side: Side, occurrence: usize, fault: IoFault) -> Self {
        Self {
            faults: vec![ScheduledFault {
                op,
                side,
                occurrence,
                fault,
            }],
            reached: Vec::new(),
        }
    }

    /// Answer whether `op` is faulted on `side` now, counting this arrival.
    ///
    /// Consulted TWICE per operation by an implementation — once before doing the work and once
    /// after — which is what makes `Before` and `After` different disks rather than two names for
    /// one interruption.
    pub fn arrive(&mut self, op: Op, side: Side) -> Option<IoFault> {
        let occurrence = self
            .reached
            .iter()
            .filter(|(seen_op, seen_side)| *seen_op == op && *seen_side == side)
            .count();
        self.reached.push((op, side));
        self.faults
            .iter()
            .find(|scheduled| {
                scheduled.op == op && scheduled.side == side && scheduled.occurrence == occurrence
            })
            .map(|scheduled| scheduled.fault)
    }

    /// Every arrival this schedule has seen, in order.
    ///
    /// The sweep reads it to assert that a run reached the operation it meant to interrupt — a
    /// schedule naming an operation the code never performs would otherwise be a case that
    /// interrupts nothing and passes.
    #[must_use]
    pub fn arrivals(&self) -> &[(Op, Side)] {
        &self.reached
    }
}

/// What an act needs in order to happen, beyond the path it happens to.
///
/// Paired one-for-one with [`Op`], which stays the CLASSIFICATION the schedule faults on: the
/// payload rides here so a new operation cannot arrive without a durability answer and a place
/// in the sweep.
#[derive(Debug, Clone, Copy)]
pub enum Request<'a> {
    /// Create a directory exclusively. The platform's most private policy is applied by the SAME
    /// call that makes the directory visible, never by a later adjustment.
    CreateDirectoryExclusive,
    /// Create a file exclusively, under the same rule.
    CreateFileExclusive,
    /// Open an existing object without following a final-component redirect.
    OpenExistingNoFollow {
        /// What the opened handle must be able to do.
        intent: OpenIntent,
    },
    /// Read at most `limit` bytes, refusing rather than truncating past it.
    ReadBounded {
        /// The bound.
        limit: usize,
    },
    /// Write every byte, or fail.
    WriteAll {
        /// The bytes.
        bytes: &'a [u8],
    },
    /// Synchronize a file.
    SyncFile,
    /// Synchronize a directory, or answer that the platform has no such operation.
    SyncDirectory,
    /// Walk a directory, collecting at most `limit` plus one so overflow is observed.
    EnumerateBounded {
        /// The bound.
        limit: usize,
    },
    /// Inspect an already-open object.
    InspectOpened,
    /// Remove an object this attempt created and still owns.
    RemoveOwned,
}

impl Request<'_> {
    /// Which operation this is, for the schedule.
    #[must_use]
    pub const fn op(&self) -> Op {
        match self {
            Self::CreateDirectoryExclusive => Op::CreateDirectoryExclusive,
            Self::CreateFileExclusive => Op::CreateFileExclusive,
            Self::OpenExistingNoFollow { .. } => Op::OpenExistingNoFollow,
            Self::ReadBounded { .. } => Op::ReadBounded,
            Self::WriteAll { .. } => Op::WriteAll,
            Self::SyncFile => Op::SyncFile,
            Self::SyncDirectory => Op::SyncDirectory,
            Self::EnumerateBounded { .. } => Op::EnumerateBounded,
            Self::InspectOpened => Op::InspectOpened,
            Self::RemoveOwned => Op::RemoveOwned,
        }
    }
}

/// What an opened handle must be able to do.
///
/// Stated at the OPEN rather than discovered at the synchronization, because the platforms
/// disagree: flushing a handle opened only for reading is permitted on one and refused on the
/// other. A caller that will synchronize says so when it opens, and a read-only route has no way
/// to acquire a handle that could.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenIntent {
    /// Read the object. This handle cannot synchronize.
    Read,
    /// Read the object and synchronize it.
    ReadAndSynchronize,
}

/// What an act answered.
///
/// One shape per act. A caller reaches these through the typed helpers below rather than by
/// matching, so an implementation answering the wrong shape is a fault at the seam instead of a
/// surprise at the call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    /// The act happened and says nothing further.
    Done,
    /// Bytes, within the bound the request named.
    Bytes(Vec<u8>),
    /// A bounded listing.
    Entries(BoundedEntries),
    /// What an opened object turned out to be.
    Facts(ObjectFacts),
}

/// A directory walk, and whether it ran out of room.
///
/// Overflow is a fact the walk OBSERVED — it collects to the bound plus one — rather than a
/// silence at the boundary that would read as a complete short listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedEntries {
    names: Vec<String>,
    over_bound: bool,
}

impl BoundedEntries {
    /// Record a walk that collected `names`, having asked for at most `limit`.
    #[must_use]
    pub fn of(mut names: Vec<String>, limit: usize) -> Self {
        names.sort_unstable();
        let over_bound = names.len() > limit;
        Self { names, over_bound }
    }

    /// The entries, in one order.
    #[must_use]
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Whether the walk found more than the bound admits.
    #[must_use]
    pub const fn over_bound(&self) -> bool {
        self.over_bound
    }

    /// Whether the walk found nothing at all, which is different from finding nothing it
    /// recognized.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty() && !self.over_bound
    }
}

/// What an opened object is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKind {
    /// A directory.
    Directory,
    /// A regular file.
    RegularFile,
    /// Something else — a device, a socket, a pipe.
    Other,
}

/// Whether anyone but the owner can reach an object.
///
/// Three answers rather than a boolean, because "the platform says nobody else can" and "this
/// platform does not answer the question" are different facts and the second must never be read
/// as the first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupAndOtherAccess {
    /// The platform reports no group or other access.
    None,
    /// The platform reports some.
    Present,
    /// The platform exposes no comparable answer. Recorded, never simulated.
    NotInspectable,
}

/// What is known about who owns an object.
///
/// Deliberately NOT a boolean, and deliberately carrying an unestablished arm: the effective
/// user's identity is not reachable from this crate's dependency set, so an object this attempt
/// did not create cannot be shown to belong to this user. Naming that keeps a reopened keyset
/// from reading as owner-verified when nothing verified it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerCheck {
    /// This attempt created the object, so it belongs to whoever this process is.
    CreatedByThisAttempt,
    /// Not established. The mode answer above stands on its own and this one does not.
    NotEstablished,
}

/// What an inspection of an already-open object found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFacts {
    kind: ObjectKind,
    redirected: bool,
    group_and_other: GroupAndOtherAccess,
    owner: OwnerCheck,
}

impl ObjectFacts {
    /// Record what an inspection found. Every field is stated; none defaults.
    #[must_use]
    pub const fn of(
        kind: ObjectKind,
        redirected: bool,
        group_and_other: GroupAndOtherAccess,
        owner: OwnerCheck,
    ) -> Self {
        Self {
            kind,
            redirected,
            group_and_other,
            owner,
        }
    }

    /// What it is.
    #[must_use]
    pub const fn kind(self) -> ObjectKind {
        self.kind
    }

    /// Whether the name is a link, junction, or reparse point.
    #[must_use]
    pub const fn redirected(self) -> bool {
        self.redirected
    }

    /// What the platform says about group and other access.
    #[must_use]
    pub const fn group_and_other(self) -> GroupAndOtherAccess {
        self.group_and_other
    }

    /// What is known about ownership.
    #[must_use]
    pub const fn owner(self) -> OwnerCheck {
        self.owner
    }

    /// Whether this attempt established who owns the object.
    ///
    /// Answers what was shown rather than what is hoped: an object this attempt did not create
    /// carries [`OwnerCheck::NotEstablished`], and no surface may read that as verified.
    #[must_use]
    pub const fn ownership_established(self) -> bool {
        matches!(self.owner, OwnerCheck::CreatedByThisAttempt)
    }
}

/// Every act this crate performs against a filesystem, as one surface.
///
/// SEALED: the supertrait is private, so no type outside this crate can implement it. A
/// production route therefore cannot be handed a modelled filesystem, and the production and
/// modelled implementations cannot drift into two different vocabularies.
pub trait LocalIo: Sealed {
    /// Perform `request` against `path`, or answer the fault that stopped it.
    ///
    /// The one entry point, so the schedule sees every act. A richer surface — a method per
    /// operation — would be pleasanter to call and would let a new method skip the schedule.
    ///
    /// # Errors
    /// Answers the fault the platform or the schedule produced.
    fn perform(&mut self, request: Request<'_>, path: &str) -> Result<Answer, IoFault>;

    /// What this implementation can say about directory synchronization.
    fn directory_sync(&self) -> DirectorySync;
}

/// Perform `request` and require it to answer [`Answer::Done`].
fn done(io: &mut dyn LocalIo, request: Request<'_>, path: &str) -> Result<(), IoFault> {
    match io.perform(request, path)? {
        Answer::Done => Ok(()),
        _ => Err(IoFault::Platform),
    }
}

pub(crate) fn create_directory_exclusive(io: &mut dyn LocalIo, path: &str) -> Result<(), IoFault> {
    done(io, Request::CreateDirectoryExclusive, path)
}

pub(crate) fn create_file_exclusive(io: &mut dyn LocalIo, path: &str) -> Result<(), IoFault> {
    done(io, Request::CreateFileExclusive, path)
}

pub(crate) fn open_existing_no_follow(
    io: &mut dyn LocalIo,
    path: &str,
    intent: OpenIntent,
) -> Result<(), IoFault> {
    done(io, Request::OpenExistingNoFollow { intent }, path)
}

pub(crate) fn write_all(io: &mut dyn LocalIo, path: &str, bytes: &[u8]) -> Result<(), IoFault> {
    done(io, Request::WriteAll { bytes }, path)
}

pub(crate) fn sync_file(io: &mut dyn LocalIo, path: &str) -> Result<(), IoFault> {
    done(io, Request::SyncFile, path)
}

/// Synchronize a directory, answering what the platform was able to do.
///
/// A platform without the operation has not FAILED it, so the two answers are separate: this
/// returns the platform's standing, and only a real failure comes back as an error.
pub(crate) fn sync_directory(io: &mut dyn LocalIo, path: &str) -> Result<DirectorySync, IoFault> {
    let standing = io.directory_sync();
    done(io, Request::SyncDirectory, path)?;
    Ok(standing)
}

pub(crate) fn read_bounded(
    io: &mut dyn LocalIo,
    path: &str,
    limit: usize,
) -> Result<Vec<u8>, IoFault> {
    match io.perform(Request::ReadBounded { limit }, path)? {
        Answer::Bytes(bytes) if bytes.len() <= limit => Ok(bytes),
        Answer::Bytes(_) => Err(IoFault::OverBound),
        _ => Err(IoFault::Platform),
    }
}

pub(crate) fn enumerate_bounded(
    io: &mut dyn LocalIo,
    path: &str,
    limit: usize,
) -> Result<BoundedEntries, IoFault> {
    match io.perform(Request::EnumerateBounded { limit }, path)? {
        Answer::Entries(entries) => Ok(entries),
        _ => Err(IoFault::Platform),
    }
}

pub(crate) fn inspect_opened(io: &mut dyn LocalIo, path: &str) -> Result<ObjectFacts, IoFault> {
    match io.perform(Request::InspectOpened, path)? {
        Answer::Facts(facts) => Ok(facts),
        _ => Err(IoFault::Platform),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_operation_is_classified_and_the_durable_ones_are_the_ones_that_change_a_disk() {
        // Two-way over the classification, because the sweep is built from it: an operation
        // wrongly called non-durable gets one schedule instead of two, and the interruption that
        // leaves a half-written keyset is the one that goes untested.
        let durable: Vec<Op> = Op::ALL.into_iter().filter(|op| op.is_durable()).collect();
        assert_eq!(durable.len(), 6, "{durable:?}");
        for op in [
            Op::CreateDirectoryExclusive,
            Op::CreateFileExclusive,
            Op::WriteAll,
            Op::SyncFile,
            Op::SyncDirectory,
            Op::RemoveOwned,
        ] {
            assert!(op.is_durable(), "{op:?}");
        }
        for op in [Op::ReadBounded, Op::InspectOpened, Op::EnumerateBounded] {
            assert!(!op.is_durable(), "{op:?}");
        }
    }

    #[test]
    fn a_schedule_faults_the_named_occurrence_and_no_other() {
        let mut schedule = FailureSchedule::faulting(Op::SyncFile, Side::After, IoFault::Platform);
        assert_eq!(schedule.arrive(Op::SyncFile, Side::Before), None);
        assert_eq!(
            schedule.arrive(Op::SyncFile, Side::After),
            Some(IoFault::Platform)
        );
        // The SECOND arrival at the same operation and side is not faulted: a schedule naming one
        // interruption has to interrupt once, or a sweep case would be describing a platform that
        // fails forever rather than one that failed here.
        assert_eq!(schedule.arrive(Op::SyncFile, Side::After), None);
    }

    #[test]
    fn an_intact_schedule_faults_nothing_and_still_records_what_was_reached() {
        // The arrivals record is what lets a sweep case prove it interrupted the operation it
        // named rather than one the code never performs.
        let mut schedule = FailureSchedule::intact();
        for op in Op::ALL {
            assert_eq!(schedule.arrive(op, Side::Before), None);
        }
        assert_eq!(schedule.arrivals().len(), Op::ALL.len());
        assert_eq!(
            schedule.arrivals().first().map(|(op, _)| *op),
            Op::ALL.first().copied()
        );
    }

    #[test]
    fn the_two_sides_of_one_operation_are_different_interruptions() {
        // The distinction the whole schedule exists for. Failing before a create leaves no
        // object; failing after it leaves one nobody finished with.
        let mut before =
            FailureSchedule::faulting(Op::CreateFileExclusive, Side::Before, IoFault::Denied);
        assert_eq!(
            before.arrive(Op::CreateFileExclusive, Side::Before),
            Some(IoFault::Denied)
        );
        let mut after =
            FailureSchedule::faulting(Op::CreateFileExclusive, Side::Before, IoFault::Denied);
        assert_eq!(after.arrive(Op::CreateFileExclusive, Side::After), None);
    }
}
