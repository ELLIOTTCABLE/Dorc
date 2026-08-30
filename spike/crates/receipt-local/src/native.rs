//! The production filesystem, behind the same sealed vocabulary the deterministic model
//! implements.
//!
//! # One implementation, two drivers
//!
//! The sweep and the shipped binary run the SAME state machine; only this file differs from the
//! model beneath it. That is the whole reason the vocabulary is one sealed trait rather than a
//! convenience API — a second code path for production is a second thing to be wrong, and the
//! interruptions the sweep enumerates would be enumerating the wrong sequence.
//!
//! # What a handle buys, and what it does not
//!
//! A file or directory this crate opens is RETAINED, and its inspection, its read, its write and
//! its synchronization all go through that handle rather than through its name a second time.
//! That is what makes `30Rd`'s inspect-the-open-handle-before-reading real rather than nominal:
//! between the inspection and the read there is no name to swap.
//!
//! On Unix the open ITSELF is non-following and, wherever this attempt already holds the
//! containing directory, HANDLE-RELATIVE. So the final component cannot be a link, and no
//! ancestor is re-walked between the directory this attempt validated and the entry it reaches
//! inside it. Enumeration still reads names by path, and that is sound because a name a walk
//! produces carries no authority: every entry is re-opened, non-following and relative to the
//! retained root, before a byte of it is read.
//!
//! Windows is the explicitly weaker baseline `30Rd` describes, and it is unchanged: a redirect is
//! refused by inspecting the name before the open, ownership is not answered at all, and neither
//! is ever rendered as equivalent to what Unix answers.
//!
//! # Why Unix removes nothing
//!
//! Every removal this crate can express names an object by its PATH, and no Unix in this
//! dependency set can condition the unlink on which object that path currently holds. Re-opening
//! the name and re-reading its device and inode first does not close that: the answer is stale
//! the instant it is read, and the unlink that follows still removes whatever holds the name a
//! moment later. So the Unix removal DECLINES — it reports [`IoFault::Unavailable`] without
//! touching anything, and the incomplete file stays where it is.
//!
//! This is a DEFECT that is being carried, not a resting state anybody wants. An interrupted
//! publication now strands a partial file that nothing collects, in a store with no retention,
//! forever. It is carried because the only other option on offer is unlinking a name, which is
//! how a failure handler deletes somebody else's work, and `30Rd` rules that worse than the
//! litter. No remedy is scheduled; whoever schedules one is choosing between a platform-specific
//! capability and a store that can sweep its own leavings, not restoring something that was lost.
//!
//! The identity a check-then-unlink would have compared is not recorded at all, because recording
//! it would say this crate binds a removal it cannot bind.
//!
//! Windows removes by name under the explicitly weaker baseline, unchanged and never rendered as
//! equivalent: it answers no ownership, retains no directory handle, and its `remove_file` reaches
//! an object this attempt created inside a directory the per-user profile owns.

use std::collections::BTreeMap;
use std::fs::File;
#[cfg(unix)]
use std::path::Path;

use crate::io::{
    Answer, BoundedEntries, GroupAndOtherAccess, IoFault, LocalIo, ObjectFacts, ObjectKind,
    OpenIntent, OwnerCheck, Request, Sealed,
};
use crate::store::DirectorySync;

/// The real filesystem.
///
/// Holds the handles this attempt opened and the KIND it created at each path it created. Both
/// are per-attempt, which is what lets ownership be answered honestly: an object this attempt
/// made is one it can say it owns, and an object it merely found is not. The kind is what is
/// remembered because the two creates are different acts and only one of them is ever removable:
/// a request naming a directory this attempt made is refused rather than reaching a call that
/// would fail obscurely.
#[derive(Debug, Default)]
pub struct NativeIo {
    open: BTreeMap<String, File>,
    created: BTreeMap<String, ObjectKind>,
}

impl NativeIo {
    /// A fresh attempt against the real filesystem.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn handle(&self, path: &str) -> Result<&File, IoFault> {
        self.open.get(path).ok_or(IoFault::Platform)
    }

    /// The handle this attempt holds for `path`'s containing directory, where it holds one.
    ///
    /// The seat that makes an act handle-relative. Absence is ordinary — nothing has opened the
    /// parent — and the caller falls back to the absolute name, which still refuses a redirected
    /// final component.
    ///
    /// Unix-gated at the MEMBER rather than allowed as dead code: Windows retains no directory
    /// handle to be relative to, and a cross-platform caller reaching for one should fail to
    /// resolve loudly rather than compile into a question the platform cannot answer
    /// (`spike/CLAUDE.md one-platform-green-is-not-cross-platform-green`).
    #[cfg(unix)]
    fn parent_handle(&self, path: &str) -> Option<&File> {
        let parent = Path::new(path).parent()?.to_str()?;
        self.open.get(parent)
    }

    /// The final component of `path`, for a handle-relative act.
    #[cfg(unix)]
    fn leaf(path: &str) -> Option<&str> {
        Path::new(path).file_name()?.to_str()
    }

    /// What the object at `path` turns out to be.
    ///
    /// Through the RETAINED handle where there is one, so a name cannot be swapped between this
    /// answer and the read that follows it. The name-shaped branch is what a platform that
    /// retains no handle for the object gets — Windows, for a directory — and it is the only
    /// branch where the redirect answer is a fact about the name rather than about the object.
    fn inspect(&self, path: &str) -> Result<ObjectFacts, IoFault> {
        let created = self.created.contains_key(path);
        if let Some(file) = self.open.get(path) {
            let metadata = file.metadata().map_err(|error| fault_of(&error))?;
            return Ok(ObjectFacts::of(
                kind_of(&metadata),
                false,
                access_of(&metadata),
                owner_of(&metadata, created),
            ));
        }
        let metadata = std::fs::symlink_metadata(path).map_err(|error| fault_of(&error))?;
        Ok(ObjectFacts::of(
            kind_of(&metadata),
            is_redirect(&metadata),
            access_of(&metadata),
            owner_of(&metadata, created),
        ))
    }
}

impl Sealed for NativeIo {}

impl LocalIo for NativeIo {
    fn perform(&mut self, request: Request<'_>, path: &str) -> Result<Answer, IoFault> {
        match request {
            Request::CreateDirectoryExclusive => {
                create_directory(self, path)?;
                self.created.insert(path.to_owned(), ObjectKind::Directory);
                Ok(Answer::Done)
            }
            Request::CreateFileExclusive => {
                let file = create_file(self, path)?;
                self.open.insert(path.to_owned(), file);
                self.created
                    .insert(path.to_owned(), ObjectKind::RegularFile);
                Ok(Answer::Done)
            }
            Request::OpenExistingNoFollow { intent } => {
                // `None` only on Windows, for a directory: no handle to retain
                if let Some(file) = open_existing(self, path, intent)? {
                    self.open.insert(path.to_owned(), file);
                }
                Ok(Answer::Done)
            }
            Request::InspectOpened => Ok(Answer::Facts(self.inspect(path)?)),
            Request::ReadBounded { limit } => {
                use std::io::Read as _;
                let file = self.handle(path)?;
                let ceiling = u64::try_from(limit).unwrap_or(u64::MAX).saturating_add(1);
                let mut bytes = Vec::new();
                file.take(ceiling)
                    .read_to_end(&mut bytes)
                    .map_err(|error| fault_of(&error))?;
                if bytes.len() > limit {
                    return Err(IoFault::OverBound);
                }
                Ok(Answer::Bytes(bytes))
            }
            Request::WriteAll { bytes } => {
                use std::io::Write as _;
                let mut file = self.handle(path)?;
                file.write_all(bytes).map_err(|error| fault_of(&error))?;
                Ok(Answer::Done)
            }
            Request::SyncFile => {
                self.handle(path)?
                    .sync_all()
                    .map_err(|error| fault_of(&error))?;
                Ok(Answer::Done)
            }
            Request::SyncDirectory => sync_directory(self, path),
            Request::EnumerateBounded { limit } => {
                let mut names = Vec::new();
                for entry in std::fs::read_dir(path).map_err(|error| fault_of(&error))? {
                    let entry = entry.map_err(|error| fault_of(&error))?;
                    names.push(entry.file_name().to_string_lossy().into_owned());
                    if names.len() > limit {
                        break;
                    }
                }
                Ok(Answer::Entries(BoundedEntries::of(names, limit)))
            }
            // The ownership questions are asked before the platform is, so a platform that
            // declines outright still refuses an unowned path for the reason it is unowned.
            Request::RemoveOwned => {
                let Some(made) = self.created.get(path).copied() else {
                    return Err(IoFault::Denied);
                };
                if made != ObjectKind::RegularFile {
                    return Err(IoFault::WrongKind);
                }
                remove_created(self, path)?;
                self.created.remove(path);
                Ok(Answer::Done)
            }
        }
    }

    fn directory_sync(&self) -> DirectorySync {
        directory_sync_standing()
    }
}

/// What a platform error means in this crate's vocabulary.
fn fault_of(error: &std::io::Error) -> IoFault {
    match error.kind() {
        std::io::ErrorKind::AlreadyExists => IoFault::AlreadyExists,
        std::io::ErrorKind::NotFound => IoFault::NotFound,
        std::io::ErrorKind::PermissionDenied => IoFault::Denied,
        std::io::ErrorKind::WriteZero => IoFault::Partial,
        _ => IoFault::Platform,
    }
}

fn kind_of(metadata: &std::fs::Metadata) -> ObjectKind {
    if metadata.is_dir() {
        ObjectKind::Directory
    } else if metadata.is_file() {
        ObjectKind::RegularFile
    } else {
        ObjectKind::Other
    }
}

/// Who owns an object, as far as this platform will say.
///
/// An object this attempt CREATED belongs to whoever this process is, whatever else is true —
/// exclusive creation is one act and nothing stood between it and this answer.
///
/// For anything else, Unix compares the object's owner against the process's own effective user.
/// `std` answers the first and not the second, which is one of the two reasons this crate carries
/// a syscall dependency at all; the call is safe and the workspace still forbids `unsafe`.
#[cfg(unix)]
fn owner_of(metadata: &std::fs::Metadata, created: bool) -> OwnerCheck {
    use std::os::unix::fs::MetadataExt as _;
    if created {
        return OwnerCheck::CreatedByThisAttempt;
    }
    if metadata.uid() == rustix::process::geteuid().as_raw() {
        OwnerCheck::EffectiveUser
    } else {
        OwnerCheck::AnotherUser
    }
}

/// Windows answers nothing comparable, so it says so.
///
/// The baseline there is the per-user profile's inherited access plus the refusal of redirects,
/// explicitly weaker than what Unix answers and never rendered as equivalent. Reconstructing a
/// DACL policy would need a maintained safe ACL implementation this crate does not carry.
#[cfg(windows)]
fn owner_of(_: &std::fs::Metadata, created: bool) -> OwnerCheck {
    if created {
        OwnerCheck::CreatedByThisAttempt
    } else {
        OwnerCheck::NotEstablished
    }
}

/// On Windows a junction is not a symlink to `std`, and both are reparse points, so the attribute
/// is what answers the question a symlink test would miss.
#[cfg(windows)]
fn is_redirect(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt as _;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    metadata.file_type().is_symlink()
        || (metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0
}

/// Unix never reaches the name-inspecting branch for an object it opened, because the open itself
/// refuses a redirect. It stays for the unopened case, where a fact about the NAME is all there is.
#[cfg(unix)]
fn is_redirect(metadata: &std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(unix)]
fn access_of(metadata: &std::fs::Metadata) -> GroupAndOtherAccess {
    use std::os::unix::fs::PermissionsExt as _;
    /// Every read, write, and execute bit outside the owner's own.
    const GROUP_AND_OTHER: u32 = 0o077;
    /// The write bits alone, which are what a store root is refused for.
    const GROUP_AND_OTHER_WRITE: u32 = 0o022;
    let mode = metadata.permissions().mode();
    if (mode & GROUP_AND_OTHER_WRITE) != 0 {
        GroupAndOtherAccess::Writable
    } else if (mode & GROUP_AND_OTHER) != 0 {
        GroupAndOtherAccess::Present
    } else {
        GroupAndOtherAccess::None
    }
}

/// Windows exposes no comparable answer, and the proof records that rather than guessing one.
#[cfg(windows)]
fn access_of(_: &std::fs::Metadata) -> GroupAndOtherAccess {
    GroupAndOtherAccess::NotInspectable
}

/// What a `rustix` failure means in this crate's vocabulary.
///
/// `ELOOP` is the interesting one: with `O_NOFOLLOW` it is what a symlinked final component
/// answers, so the redirect refusal that used to be a separate metadata read is now the open's
/// own verdict. macOS spells the same condition `EMLINK` on some paths, hence both.
#[cfg(unix)]
fn errno_of(errno: rustix::io::Errno) -> IoFault {
    match errno {
        rustix::io::Errno::EXIST => IoFault::AlreadyExists,
        rustix::io::Errno::NOENT => IoFault::NotFound,
        rustix::io::Errno::ACCESS | rustix::io::Errno::PERM => IoFault::Denied,
        rustix::io::Errno::LOOP | rustix::io::Errno::MLINK => IoFault::Redirect,
        rustix::io::Errno::ISDIR | rustix::io::Errno::NOTDIR => IoFault::WrongKind,
        _ => IoFault::Platform,
    }
}

/// The mode rides the SAME creation call, so the object never exists group- or other-reachable.
#[cfg(unix)]
fn create_directory(io: &NativeIo, path: &str) -> Result<(), IoFault> {
    use rustix::fs::Mode;
    let mode = Mode::from_bits_truncate(0o700);
    match (io.parent_handle(path), NativeIo::leaf(path)) {
        (Some(parent), Some(name)) => rustix::fs::mkdirat(parent, name, mode),
        _ => rustix::fs::mkdir(path, mode),
    }
    .map_err(errno_of)
}

#[cfg(windows)]
fn create_directory(_: &NativeIo, path: &str) -> Result<(), IoFault> {
    std::fs::create_dir(path).map_err(|error| fault_of(&error))
}

/// Exclusive creation, non-following by construction: `O_EXCL` refuses an existing name, a
/// dangling link included, and it does so in the same call that makes the object.
#[cfg(unix)]
fn create_file(io: &NativeIo, path: &str) -> Result<File, IoFault> {
    use rustix::fs::{Mode, OFlags};
    let flags = OFlags::CREATE | OFlags::EXCL | OFlags::RDWR | OFlags::CLOEXEC;
    let mode = Mode::from_bits_truncate(0o600);
    match (io.parent_handle(path), NativeIo::leaf(path)) {
        (Some(parent), Some(name)) => rustix::fs::openat(parent, name, flags, mode),
        _ => rustix::fs::open(path, flags, mode),
    }
    .map(File::from)
    .map_err(errno_of)
}

/// Windows has no mode, so the object inherits the per-user profile's access. Exclusive creation
/// still holds: `create_new` refuses an existing name, a dangling link included.
#[cfg(windows)]
fn create_file(_: &NativeIo, path: &str) -> Result<File, IoFault> {
    std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(path)
        .map_err(|error| fault_of(&error))
}

/// Open an existing object without following its final component, with exactly the access the
/// caller declared it needs.
///
/// The access distinction is load-bearing rather than tidy: flushing a handle opened only for
/// reading is permitted on one platform and refused on the other, so a route that will
/// synchronize has to say so at the open — and a read-only route acquires a handle that could
/// not.
///
/// `O_NONBLOCK` rides along so a hostile FIFO in a store directory cannot park the reader on the
/// open. It changes nothing for the regular files and directories this crate opens, and the kind
/// is checked through the retained handle immediately afterwards either way.
#[cfg(unix)]
fn open_existing(io: &NativeIo, path: &str, intent: OpenIntent) -> Result<Option<File>, IoFault> {
    use rustix::fs::{Mode, OFlags};
    let access = if intent == OpenIntent::ReadAndSynchronize {
        OFlags::RDWR
    } else {
        OFlags::RDONLY
    };
    let flags = access | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK;
    match (io.parent_handle(path), NativeIo::leaf(path)) {
        (Some(parent), Some(name)) => rustix::fs::openat(parent, name, flags, Mode::empty()),
        _ => rustix::fs::open(path, flags, Mode::empty()),
    }
    .map(|fd| Some(File::from(fd)))
    .map_err(errno_of)
}

/// Windows keeps the pre-check-then-open shape, which is the weaker baseline and the reason this
/// arm is spelled separately rather than shared: `std` follows the final component, and this
/// crate's dependency set carries no safe non-following open for the platform.
///
/// A directory answers `None`: the platform has no handle this crate can open and retain for one,
/// so its inspection reads the NAME. That, too, is the weaker baseline stated rather than
/// simulated.
#[cfg(windows)]
fn open_existing(_: &NativeIo, path: &str, intent: OpenIntent) -> Result<Option<File>, IoFault> {
    let metadata = std::fs::symlink_metadata(path).map_err(|error| fault_of(&error))?;
    if is_redirect(&metadata) {
        return Err(IoFault::Redirect);
    }
    if metadata.is_dir() {
        return Ok(None);
    }
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    if intent == OpenIntent::ReadAndSynchronize {
        options.write(true);
    }
    options
        .open(path)
        .map(Some)
        .map_err(|error| fault_of(&error))
}

#[cfg(unix)]
const fn directory_sync_standing() -> DirectorySync {
    DirectorySync::Synchronized
}

#[cfg(windows)]
const fn directory_sync_standing() -> DirectorySync {
    DirectorySync::UnavailableOnPlatform
}

/// Synchronize a directory through the handle this attempt already holds for it, where it holds
/// one; otherwise open it non-following for the purpose.
#[cfg(unix)]
fn sync_directory(io: &NativeIo, path: &str) -> Result<Answer, IoFault> {
    if let Some(handle) = io.open.get(path) {
        handle.sync_all().map_err(|error| fault_of(&error))?;
        return Ok(Answer::Done);
    }
    let opened = open_existing(io, path, OpenIntent::Read)?.ok_or(IoFault::Platform)?;
    opened.sync_all().map_err(|error| fault_of(&error))?;
    Ok(Answer::Done)
}

/// Windows has no operation matching this one. It is recorded unavailable rather than simulated,
/// and the publication proof carries that as a property rather than as a weaker grade.
#[cfg(windows)]
#[expect(
    clippy::unnecessary_wraps,
    reason = "the shape is the Unix twin's, where the operation genuinely fails; one signature \n              across both platforms is what keeps the caller from branching on the platform"
)]
const fn sync_directory(_: &NativeIo, _: &str) -> Result<Answer, IoFault> {
    Ok(Answer::Done)
}

/// Decline, without touching anything.
///
/// Unix removes by NAME — `unlink` and `unlinkat` both do — and offers no way to say "only if this
/// path still holds the object I created". Re-opening the name and comparing its device and inode
/// first would answer about the instant of the comparison and not about the instant of the unlink,
/// so the removal it licenses is still a removal of whatever holds the name by then.
///
/// So this hands back a refusal and strands the incomplete file — the carried defect the module
/// note describes, taken over deleting an object whose identity is uncertain.
#[cfg(unix)]
fn remove_created(_: &mut NativeIo, _: &str) -> Result<(), IoFault> {
    Err(IoFault::Unavailable)
}

/// Windows removes by name under the weaker baseline, unchanged: no identity is available to bind
/// to, and the object is one this attempt created inside a directory the per-user profile owns.
///
/// The retained handle goes first, because a file this attempt still holds open is not one the
/// platform reliably unlinks.
#[cfg(windows)]
fn remove_created(io: &mut NativeIo, path: &str) -> Result<(), IoFault> {
    io.open.remove(path);
    std::fs::remove_file(path).map_err(|error| fault_of(&error))
}
