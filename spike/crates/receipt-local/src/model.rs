//! The modelled filesystem: what the deterministic sweep drives instead of a disk.
//!
//! It is not a filesystem. It is a record of the acts this crate performed and what they left
//! behind, which is exactly enough to answer the question the sweep asks — RESTART FROM THIS
//! DISK, and see which state the keyset lands in. Anything more would be a second filesystem with
//! its own bugs, and a native test is what answers the questions this cannot: real permissions,
//! real links, real synchronization, real sharing.
//!
//! It implements the same sealed trait the production edge does, so the sweep exercises the code
//! that ships rather than a copy of it.
//!
//! Objects a test PLANTS carry the facts an inspection would report, so a permissive mode, a
//! redirect, and a wrong kind are things a case states rather than things this file guesses.

use std::collections::BTreeMap;

use crate::io::{
    Answer, BoundedEntries, FailureSchedule, GroupAndOtherAccess, IoFault, LocalIo, ObjectFacts,
    ObjectKind, OpenIntent, OwnerCheck, Request, Sealed, Side,
};
use crate::store::DirectorySync;

/// What lives at one modelled path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    kind: NodeKind,
    /// Whether the object was created by the attempt currently running.
    created_here: bool,
    /// What an inspection would say about group and other access.
    group_and_other: GroupAndOtherAccess,
    /// Whether the name is a link, junction, or reparse point.
    redirected: bool,
    /// Whether the object has been synchronized since its last write.
    synced: bool,
}

/// What kind of object a node is, and what a file holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// A directory.
    Directory,
    /// A file, its bytes, and whether every byte it was given arrived.
    File {
        /// What it holds.
        bytes: Vec<u8>,
        /// Whether the write completed.
        whole: bool,
    },
    /// Something a keyset may not contain.
    Other,
}

impl Node {
    /// A directory as this crate would have created it.
    #[must_use]
    pub fn private_directory() -> Self {
        Self::of(NodeKind::Directory, GroupAndOtherAccess::None)
    }

    /// A file holding `bytes`, as this crate would have written it.
    #[must_use]
    pub fn private_file(bytes: &[u8]) -> Self {
        Self::of(
            NodeKind::File {
                bytes: bytes.to_vec(),
                whole: true,
            },
            GroupAndOtherAccess::None,
        )
    }

    /// An object of `kind` whose group and other access is `group_and_other`.
    #[must_use]
    pub fn of(kind: NodeKind, group_and_other: GroupAndOtherAccess) -> Self {
        Self {
            kind,
            created_here: false,
            group_and_other,
            redirected: false,
            synced: true,
        }
    }

    /// The same object, reached through a link, junction, or reparse point.
    #[must_use]
    pub fn redirected(mut self) -> Self {
        self.redirected = true;
        self
    }

    /// What it holds, if it is a file.
    #[must_use]
    pub fn bytes(&self) -> Option<&[u8]> {
        match &self.kind {
            NodeKind::File { bytes, .. } => Some(bytes),
            NodeKind::Directory | NodeKind::Other => None,
        }
    }

    /// Whether the last write completed.
    #[must_use]
    pub const fn whole(&self) -> bool {
        match &self.kind {
            NodeKind::File { whole, .. } => *whole,
            NodeKind::Directory | NodeKind::Other => true,
        }
    }

    /// Whether it has been synchronized since its last write.
    #[must_use]
    pub const fn synced(&self) -> bool {
        self.synced
    }

    /// Whether it is a directory.
    #[must_use]
    pub const fn is_directory(&self) -> bool {
        matches!(self.kind, NodeKind::Directory)
    }

    fn facts(&self) -> ObjectFacts {
        let kind = match self.kind {
            NodeKind::Directory => ObjectKind::Directory,
            NodeKind::File { .. } => ObjectKind::RegularFile,
            NodeKind::Other => ObjectKind::Other,
        };
        let owner = if self.created_here {
            OwnerCheck::CreatedByThisAttempt
        } else {
            OwnerCheck::NotEstablished
        };
        ObjectFacts::of(kind, self.redirected, self.group_and_other, owner)
    }
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
    creates_privately: bool,
    /// What each open handle this attempt holds is able to do. The platforms disagree about
    /// flushing a read-only handle, so the model enforces the stricter rule and the sweep sees
    /// the divergence rather than a native run discovering it.
    handles: BTreeMap<String, OpenIntent>,
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
            creates_privately: directory_sync == DirectorySync::Synchronized,
            handles: BTreeMap::new(),
        }
    }

    /// The same disk on a platform that cannot report group and other access — the Windows
    /// posture, where the baseline is explicitly weaker and says so.
    #[must_use]
    pub fn windows_shaped(schedule: FailureSchedule) -> Self {
        Self {
            nodes: BTreeMap::new(),
            schedule,
            directory_sync: DirectorySync::UnavailableOnPlatform,
            creates_privately: false,
            handles: BTreeMap::new(),
        }
    }

    /// Place `node` at `path` as something that was already there when this attempt started.
    #[must_use]
    pub fn planting(mut self, path: &str, node: Node) -> Self {
        self.nodes.insert(path.to_owned(), node);
        self
    }

    /// Restart from the disk this one left, under a fresh schedule.
    ///
    /// The sweep's second half: an interruption is only interesting for what a LATER process
    /// finds, so the disk survives the schedule that made it and nothing else does — including
    /// the knowledge of which objects this attempt created, which a restart genuinely loses.
    #[must_use]
    pub fn restart(&self, schedule: FailureSchedule) -> Self {
        let nodes = self
            .nodes
            .iter()
            .map(|(path, node)| {
                let mut carried = node.clone();
                carried.created_here = false;
                (path.clone(), carried)
            })
            .collect();
        Self {
            nodes,
            schedule,
            directory_sync: self.directory_sync,
            creates_privately: self.creates_privately,
            handles: BTreeMap::new(),
        }
    }

    /// What is at `path`, if anything.
    #[must_use]
    pub fn at(&self, path: &str) -> Option<&Node> {
        self.nodes.get(path)
    }

    /// Every path on this disk, in order.
    #[must_use]
    pub fn paths(&self) -> Vec<&str> {
        self.nodes.keys().map(String::as_str).collect()
    }

    /// The schedule, for a case asserting which operations it actually reached.
    #[must_use]
    pub const fn schedule(&self) -> &FailureSchedule {
        &self.schedule
    }

    /// The direct children of `path`, by their own names.
    fn children(&self, path: &str) -> Vec<String> {
        let prefix = format!("{path}/");
        self.nodes
            .keys()
            .filter_map(|candidate| candidate.strip_prefix(&prefix))
            .filter(|rest| !rest.contains('/'))
            .map(str::to_owned)
            .collect()
    }

    /// The act itself, once the schedule has let it through.
    fn apply(&mut self, request: Request<'_>, path: &str) -> Result<Answer, IoFault> {
        match request {
            Request::CreateDirectoryExclusive => self.create(path, NodeKind::Directory),
            Request::CreateFileExclusive => self.create(
                path,
                NodeKind::File {
                    bytes: Vec::new(),
                    whole: false,
                },
            ),
            Request::OpenExistingNoFollow { intent } => match self.nodes.get(path) {
                Some(node) if node.redirected => Err(IoFault::Redirect),
                Some(_) => {
                    self.handles.insert(path.to_owned(), intent);
                    Ok(Answer::Done)
                }
                None => Err(IoFault::NotFound),
            },
            Request::InspectOpened => match self.nodes.get(path) {
                Some(node) => Ok(Answer::Facts(node.facts())),
                None => Err(IoFault::NotFound),
            },
            Request::ReadBounded { limit } => match self.nodes.get(path) {
                Some(node) => match node.bytes() {
                    Some(bytes) if bytes.len() > limit => Err(IoFault::OverBound),
                    Some(bytes) => Ok(Answer::Bytes(bytes.to_vec())),
                    None => Err(IoFault::WrongKind),
                },
                None => Err(IoFault::NotFound),
            },
            Request::WriteAll { bytes } => match self.nodes.get_mut(path) {
                Some(node) => match &mut node.kind {
                    NodeKind::File { bytes: held, whole } => {
                        held.clear();
                        held.extend_from_slice(bytes);
                        *whole = true;
                        node.synced = false;
                        Ok(Answer::Done)
                    }
                    NodeKind::Directory | NodeKind::Other => Err(IoFault::WrongKind),
                },
                None => Err(IoFault::NotFound),
            },
            // A handle opened only for reading cannot flush on every platform, so the model
            // refuses what the stricter one does rather than letting a native run find it.
            Request::SyncFile if self.handles.get(path) == Some(&OpenIntent::Read) => {
                Err(IoFault::Denied)
            }
            Request::SyncFile => match self.nodes.get_mut(path) {
                Some(node) if matches!(node.kind, NodeKind::File { .. }) => {
                    node.synced = true;
                    Ok(Answer::Done)
                }
                Some(_) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            // A platform without the operation does not FAIL it — it does not have it, and the
            // proof records that rather than a weaker success. So the answer there is `Done` for
            // every path, including one that is not there.
            Request::SyncDirectory
                if self.directory_sync == DirectorySync::UnavailableOnPlatform =>
            {
                Ok(Answer::Done)
            }
            Request::SyncDirectory => match self.nodes.get_mut(path) {
                Some(node) if node.is_directory() => {
                    node.synced = true;
                    Ok(Answer::Done)
                }
                Some(_) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Request::EnumerateBounded { limit } => match self.nodes.get(path) {
                Some(node) if node.is_directory() => Ok(Answer::Entries(BoundedEntries::of(
                    self.children(path),
                    limit,
                ))),
                Some(_) => Err(IoFault::WrongKind),
                None => Err(IoFault::NotFound),
            },
            Request::RemoveOwned => match self.nodes.remove(path) {
                Some(_) => Ok(Answer::Done),
                None => Err(IoFault::NotFound),
            },
        }
    }

    /// Exclusive creation, under the platform's private policy, in one act.
    fn create(&mut self, path: &str, kind: NodeKind) -> Result<Answer, IoFault> {
        if self.nodes.contains_key(path) {
            return Err(IoFault::AlreadyExists);
        }
        self.handles
            .insert(path.to_owned(), OpenIntent::ReadAndSynchronize);
        let group_and_other = if self.creates_privately {
            GroupAndOtherAccess::None
        } else {
            GroupAndOtherAccess::NotInspectable
        };
        self.nodes.insert(
            path.to_owned(),
            Node {
                kind,
                created_here: true,
                group_and_other,
                redirected: false,
                synced: false,
            },
        );
        Ok(Answer::Done)
    }
}

impl Sealed for ModelIo {}

impl LocalIo for ModelIo {
    fn perform(&mut self, request: Request<'_>, path: &str) -> Result<Answer, IoFault> {
        let op = request.op();
        if let Some(fault) = self.schedule.arrive(op, Side::Before) {
            return Err(fault);
        }
        let outcome = self.apply(request, path);
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
