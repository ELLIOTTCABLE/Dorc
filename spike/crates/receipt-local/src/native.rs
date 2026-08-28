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
//! A file this crate opens is RETAINED, and its inspection and its read both go through that
//! handle rather than through its name a second time. That is what makes `30Rd`'s
//! inspect-the-open-handle-before-reading real rather than nominal: between the inspection and
//! the read there is no name to swap.
//!
//! It does not close every race. Opening by name is still one lookup — reaching a genuinely
//! non-following open needs a platform call this crate's dependency set does not carry — so a
//! swap BETWEEN the redirect check and the open remains possible. Creation is not affected:
//! exclusive creation refuses an existing name, a dangling link included, in one act.
//!
//! Directories are addressed by name on both platforms, because a directory handle is not
//! portable. On Windows that is the explicitly weaker baseline `30Rd` describes, and it must
//! never be rendered as equivalent to what Unix answers.

use std::collections::BTreeMap;
use std::fs::File;

use crate::io::{
    Answer, BoundedEntries, GroupAndOtherAccess, IoFault, LocalIo, ObjectFacts, ObjectKind,
    OpenIntent, OwnerCheck, Request, Sealed,
};
use crate::store::DirectorySync;

/// The real filesystem.
///
/// Holds the handles this attempt opened and the names it created. Both are per-attempt, which is
/// what lets ownership be answered honestly: an object this attempt made is one it can say it
/// owns, and an object it merely found is not.
#[derive(Debug, Default)]
pub struct NativeIo {
    open: BTreeMap<String, File>,
    created: Vec<String>,
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
}

impl Sealed for NativeIo {}

impl LocalIo for NativeIo {
    fn perform(&mut self, request: Request<'_>, path: &str) -> Result<Answer, IoFault> {
        match request {
            Request::CreateDirectoryExclusive => {
                create_directory(path)?;
                self.created.push(path.to_owned());
                Ok(Answer::Done)
            }
            Request::CreateFileExclusive => {
                let file = create_file(path)?;
                self.open.insert(path.to_owned(), file);
                self.created.push(path.to_owned());
                Ok(Answer::Done)
            }
            Request::OpenExistingNoFollow { intent } => {
                let facts = symlink_facts(path)?;
                if facts.redirected() {
                    return Err(IoFault::Redirect);
                }
                if facts.kind() == ObjectKind::RegularFile {
                    let file = open_existing(path, intent)?;
                    self.open.insert(path.to_owned(), file);
                }
                Ok(Answer::Done)
            }
            Request::InspectOpened => {
                let created = self.created.iter().any(|made| made == path);
                // Through the retained handle where there is one, so a name cannot be swapped
                // between this answer and the read that follows it.
                let found = if let Some(file) = self.open.get(path) {
                    let metadata = file.metadata().map_err(|error| fault_of(&error))?;
                    ObjectFacts::of(
                        kind_of(&metadata),
                        false,
                        access_of(&metadata),
                        owner_of(&metadata, created),
                    )
                } else {
                    let metadata =
                        std::fs::symlink_metadata(path).map_err(|error| fault_of(&error))?;
                    ObjectFacts::of(
                        kind_of(&metadata),
                        is_redirect(&metadata),
                        access_of(&metadata),
                        owner_of(&metadata, created),
                    )
                };
                Ok(Answer::Facts(found))
            }
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
            Request::SyncDirectory => sync_directory(path),
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
            // Only an object this attempt created and still names. A removal by pathname alone is
            // how a failure handler deletes somebody else's work.
            Request::RemoveOwned => {
                if !self.created.iter().any(|made| made == path) {
                    return Err(IoFault::Denied);
                }
                self.open.remove(path);
                std::fs::remove_file(path).map_err(|error| fault_of(&error))?;
                self.created.retain(|made| made != path);
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

/// What a non-following look at `path` says, without opening it.
fn symlink_facts(path: &str) -> Result<ObjectFacts, IoFault> {
    let metadata = std::fs::symlink_metadata(path).map_err(|error| fault_of(&error))?;
    Ok(ObjectFacts::of(
        kind_of(&metadata),
        is_redirect(&metadata),
        access_of(&metadata),
        // This look precedes an open, so no attempt owns what it found yet; the ownership
        // question is asked of the RETAINED handle, where a name cannot be swapped underneath it.
        OwnerCheck::NotEstablished,
    ))
}

/// Who owns an object, as far as this platform will say.
///
/// An object this attempt CREATED belongs to whoever this process is, whatever else is true —
/// exclusive creation is one act and nothing stood between it and this answer.
///
/// For anything else, Unix compares the object's owner against the process's own effective user.
/// `std` answers the first and not the second, which is the whole reason this crate carries a
/// syscall dependency at all; the call is safe and the workspace still forbids `unsafe`.
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

#[cfg(unix)]
fn is_redirect(metadata: &std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
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

/// The mode rides the SAME creation call, so the object never exists group- or other-reachable.
#[cfg(unix)]
fn create_directory(path: &str) -> Result<(), IoFault> {
    use std::os::unix::fs::DirBuilderExt as _;
    std::fs::DirBuilder::new()
        .mode(0o700)
        .create(path)
        .map_err(|error| fault_of(&error))
}

#[cfg(windows)]
fn create_directory(path: &str) -> Result<(), IoFault> {
    std::fs::create_dir(path).map_err(|error| fault_of(&error))
}

#[cfg(unix)]
fn create_file(path: &str) -> Result<File, IoFault> {
    use std::os::unix::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|error| fault_of(&error))
}

/// Windows has no mode, so the object inherits the per-user profile's access. Exclusive creation
/// still holds: `create_new` refuses an existing name, a dangling link included.
#[cfg(windows)]
fn create_file(path: &str) -> Result<File, IoFault> {
    std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(path)
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

#[cfg(unix)]
fn sync_directory(path: &str) -> Result<Answer, IoFault> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| fault_of(&error))?;
    Ok(Answer::Done)
}

/// Windows has no operation matching this one. It is recorded unavailable rather than simulated,
/// and the publication proof carries that as a property rather than as a weaker grade.
#[cfg(windows)]
#[expect(
    clippy::unnecessary_wraps,
    reason = "the shape is the Unix twin's, where the operation genuinely fails; one signature \n              across both platforms is what keeps the caller from branching on the platform"
)]
const fn sync_directory(_: &str) -> Result<Answer, IoFault> {
    Ok(Answer::Done)
}

/// Open an existing file with exactly the access the caller declared it needs.
///
/// The distinction is load-bearing rather than tidy: flushing a handle opened only for reading is
/// permitted on one platform and refused on the other, so a route that will synchronize has to
/// say so at the open — and a read-only route acquires a handle that could not.
fn open_existing(path: &str, intent: OpenIntent) -> Result<File, IoFault> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    if intent == OpenIntent::ReadAndSynchronize {
        options.write(true);
    }
    options.open(path).map_err(|error| fault_of(&error))
}
