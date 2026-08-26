//! The modelled filesystem: what the deterministic sweep drives instead of a disk.
//!
//! It is not a filesystem. It is a record of the acts this crate performed and what they left
//! behind, which is exactly enough to answer the question the sweep asks — RESTART FROM THIS
//! DISK, and see which state the keyset lands in. Anything more would be a second filesystem with
//! its own bugs, and a native test is what answers the questions this cannot: real permissions,
//! real links, real synchronization, real sharing.
//!
//! It implements the same sealed trait the production edge will, so the sweep exercises the code
//! that ships rather than a copy of it.

use std::collections::BTreeMap;

use crate::io::{FailureSchedule, IoFault, LocalIo, Op, Sealed, Side};
use crate::store::DirectorySync;

/// What lives at one modelled path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    /// A directory.
    Directory,
    /// A file, and whether every byte it was given arrived.
    File {
        /// Whether the write completed.
        whole: bool,
        /// Whether the file was synchronized after its last write.
        synced: bool,
    },
}

/// One modelled disk, plus the schedule interrupting the acts performed against it.
///
/// Ordered map, never a hash: the sweep's assertions read the disk, and an iteration order that
/// moved between runs would make a deterministic test report differently on two of them.
#[derive(Debug)]
pub struct ModelIo {
    nodes: BTreeMap<String, Node>,
    schedule: FailureSchedule,
    directory_sync: DirectorySync,
}

impl ModelIo {
    /// An empty disk, under `schedule`, on a platform whose directory synchronization is
    /// `directory_sync`.
    #[must_use]
    pub fn new(schedule: FailureSchedule, directory_sync: DirectorySync) -> Self {
        Self {
            nodes: BTreeMap::new(),
            schedule,
            directory_sync,
        }
    }

    /// Restart from the disk this one left, under a fresh schedule.
    ///
    /// The sweep's second half: an interruption is only interesting for what a LATER process
    /// finds, so the disk survives the schedule that made it and nothing else does.
    #[must_use]
    pub fn restart(&self, schedule: FailureSchedule) -> Self {
        Self {
            nodes: self.nodes.clone(),
            schedule,
            directory_sync: self.directory_sync,
        }
    }

    /// What is at `path`, if anything.
    #[must_use]
    pub fn at(&self, path: &str) -> Option<Node> {
        self.nodes.get(path).copied()
    }

    /// Every path on this disk, in order.
    #[must_use]
    pub fn paths(&self) -> Vec<&str> {
        self.nodes.keys().map(String::as_str).collect()
    }

    /// The schedule, for a case asserting which operations it actually reached.
    #[must_use]
    pub fn schedule(&self) -> &FailureSchedule {
        &self.schedule
    }
}

impl Sealed for ModelIo {}

impl LocalIo for ModelIo {
    fn perform(&mut self, op: Op, path: &str) -> Result<(), IoFault> {
        if let Some(fault) = self.schedule.arrive(op, Side::Before) {
            return Err(fault);
        }
        let outcome = self.apply(op, path);
        // The After side is consulted even when the act itself failed, so a schedule can describe
        // a cleanup that fails while handling a prior failure — the compound shape a real
        // filesystem produces and a single-fault model would never reach.
        if let Some(fault) = self.schedule.arrive(op, Side::After) {
            return Err(fault);
        }
        outcome
    }

    fn directory_sync(&self) -> DirectorySync {
        self.directory_sync
    }
}

impl ModelIo {
    /// The act itself, once the schedule has let it through.
    fn apply(&mut self, op: Op, path: &str) -> Result<(), IoFault> {
        match op {
            Op::OpenValidatedRoot => match self.nodes.get(path) {
                Some(Node::Directory) => Ok(()),
                Some(Node::File { .. }) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Op::CreateDirectoryExclusive => {
                if self.nodes.contains_key(path) {
                    return Err(IoFault::AlreadyExists);
                }
                self.nodes.insert(path.to_owned(), Node::Directory);
                Ok(())
            }
            Op::CreateFileExclusive => {
                if self.nodes.contains_key(path) {
                    return Err(IoFault::AlreadyExists);
                }
                self.nodes.insert(
                    path.to_owned(),
                    Node::File {
                        whole: false,
                        synced: false,
                    },
                );
                Ok(())
            }
            Op::OpenExistingNoFollow | Op::InspectOpened => match self.nodes.get(path) {
                Some(_) => Ok(()),
                None => Err(IoFault::NotFound),
            },
            Op::ReadBounded => match self.nodes.get(path) {
                Some(Node::File { .. }) => Ok(()),
                Some(Node::Directory) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Op::WriteAll => match self.nodes.get_mut(path) {
                Some(Node::File { whole, synced }) => {
                    *whole = true;
                    *synced = false;
                    Ok(())
                }
                Some(Node::Directory) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Op::SyncFile => match self.nodes.get_mut(path) {
                Some(Node::File { synced, .. }) => {
                    *synced = true;
                    Ok(())
                }
                Some(Node::Directory) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            // A platform without the operation does not FAIL it — it does not have it, and the
            // proof records that rather than a weaker success. So the answer there is `Ok` for
            // every path, including one that is not there.
            Op::SyncDirectory if self.directory_sync == DirectorySync::UnavailableOnPlatform => {
                Ok(())
            }
            Op::SyncDirectory => match self.nodes.get(path) {
                Some(Node::Directory) => Ok(()),
                Some(Node::File { .. }) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Op::EnumerateBounded => match self.nodes.get(path) {
                Some(Node::Directory) => Ok(()),
                Some(Node::File { .. }) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Op::RemoveOwned => match self.nodes.remove(path) {
                Some(_) => Ok(()),
                None => Err(IoFault::NotFound),
            },
        }
    }
}
