//! The `dorc-apply-artifact-image/1` container: every stream and file one apply uses, by value.
//!
//! This container may ENCODE an apply image and may never CHANGE it. Nothing here bundles,
//! flattens, relocates, rewrites an import, normalizes a path, deduplicates, or alters a byte;
//! re-materializing reproduces every entry, path, mode, root, edge, entrypoint and byte.
//!
//! Framing is by declared length, never by scanning for a delimiter, so an entry's content may
//! hold any byte — including a run spelling this container's own terminator.
//!
//! Identity is minted, never accepted: the two constructors validate, encode, hash and store in
//! one operation, and [`ApplyArtifactImage::parse`] recomputes over the exact input span and
//! compares before an image exists. No constructor takes an [`ApplyArtifactImageId`].

use crate::grammar::canonical_u64;
use crate::ids::ApplyArtifactImageId;
use crate::limits::{CountLimit, ReceiptLimits};

/// The container's version line.
pub const IMAGE_VERSION_LINE: &str = "dorc-apply-artifact-image/1";

/// The token closing the container.
pub const IMAGE_END: &str = "image-end";

/// The most bytes one path component may carry: a filesystem fact, not a policy value.
pub const MAX_PATH_COMPONENT_BYTES: usize = 255;

/// The greatest mode a four-octal-digit field can spell.
pub const MAX_MODE_BITS: u16 = 0o7777;

/// Why a path is not a legal recorded apply path. One condition each; none stands in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathRefusal {
    /// No bytes at all.
    Empty,
    /// Past the policy's path bound.
    TooLong,
    /// A component past [`MAX_PATH_COMPONENT_BYTES`].
    ComponentTooLong,
    /// A leading separator: the path is not target-relative.
    LeadingSeparator,
    /// A trailing separator.
    TrailingSeparator,
    /// Two adjacent separators.
    EmptyComponent,
    /// A `.` component.
    DotComponent,
    /// A `..` component: traversal is refused, never resolved.
    DotDotComponent,
    /// A byte outside the portable set.
    IllegalByte {
        /// The offending byte.
        byte: u8,
    },
    /// A component ending in a space.
    ComponentTrailingSpace,
    /// A component ending in a dot.
    ComponentTrailingDot,
    /// A reserved device stem, with or without an extension.
    DeviceStem,
}

/// Why an image was not accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageRefusal {
    /// Past a declared bound.
    OverBound {
        /// Which bound.
        what: &'static str,
    },
    /// A required line was absent, out of order, or misspelled.
    Structure {
        /// Which line.
        what: &'static str,
    },
    /// A collection the model requires to be non-empty was empty.
    Empty {
        /// Which collection.
        what: &'static str,
    },
    /// An identifier not unique and contiguous from zero, in ascending order.
    Identity {
        /// Which domain.
        what: &'static str,
    },
    /// A path this image already carries, under exact or case-folded comparison.
    DuplicatePath,
    /// One path names a directory another path also names as a file.
    PathContainsPath,
    /// A path outside the recorded grammar.
    Path(PathRefusal),
    /// An entry in a shape its kind does not admit.
    EntryShape {
        /// What disagreed.
        what: &'static str,
    },
    /// A second path-less entry: two would be indistinguishable at materialization.
    SecondStream,
    /// A root, entrypoint, or edge endpoint naming an entry that does not exist.
    Dangling {
        /// Which reference.
        what: &'static str,
    },
    /// An entry no root, entrypoint, or edge names.
    UnaccountedEntry,
    /// The same edge twice, or edges out of canonical order.
    EdgeOrder,
    /// A load chain deeper than the policy admits.
    TopologyDepth,
    /// A token outside its closed set.
    UnknownToken {
        /// Which field.
        what: &'static str,
    },
    /// A declared byte length the input does not supply.
    LengthMismatch,
    /// The recomputed identity is not the one that was asked for.
    IdentityMismatch,
    /// Bytes after the terminator, or a terminator that never arrived.
    TrailingBytes,
}

impl From<PathRefusal> for ImageRefusal {
    fn from(refusal: PathRefusal) -> Self {
        Self::Path(refusal)
    }
}

/// The shape the artifact was published in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedArtifactForm {
    /// One stream, every reached subgraph standing where its load stood.
    Flattened,
    /// A plan plus one bundle per book-sited root.
    Multipart,
    /// A plan plus every reached source at its own authored relative path.
    MirroredTree,
    /// Authored boundaries untouched, no dependencies carried.
    PreservedBookTree,
    /// Bytes the apply was handed rather than ones Dorc emitted.
    ExternalStream,
}

impl RecordedArtifactForm {
    /// Every form, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Flattened,
        Self::Multipart,
        Self::MirroredTree,
        Self::PreservedBookTree,
        Self::ExternalStream,
    ];

    /// The literal word in the `form` line.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Flattened => "flattened",
            Self::Multipart => "multipart",
            Self::MirroredTree => "mirrored-tree",
            Self::PreservedBookTree => "preserved-book-tree",
            Self::ExternalStream => "external-stream",
        }
    }

    /// The form a literal word names.
    #[must_use]
    pub fn of_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|form| form.token() == token)
    }
}

/// One entry's ordinal, unique and contiguous from zero within an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyEntryId(u32);

impl ApplyEntryId {
    /// Name an ordinal. An ordinal is an index, never an authority.
    #[must_use]
    pub const fn of(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// One root's ordinal, in its own domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyRootId(u32);

impl ApplyRootId {
    /// Name an ordinal.
    #[must_use]
    pub const fn of(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Whether an entry is a stream the apply is handed or a file it materializes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyEntryKind {
    /// No path: the bytes arrive on a stream and are never named.
    Stream,
    /// Materialized at its recorded path.
    File,
}

impl ApplyEntryKind {
    /// The literal word in an `entry` line.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Stream => "stream",
            Self::File => "file",
        }
    }

    /// The kind a literal word names.
    #[must_use]
    pub fn of_token(token: &str) -> Option<Self> {
        match token {
            "stream" => Some(Self::Stream),
            "file" => Some(Self::File),
            _ => None,
        }
    }
}

/// A file mode, or a statement that mode is not an execution input for this entry.
///
/// There is deliberately no unknown arm. A caller that does not know whether an entry's mode
/// matters cannot record that state and must refuse at its own seat instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedMode {
    /// Mode is not an execution input for this entry.
    Unused,
    /// Exactly these permission bits, at most [`MAX_MODE_BITS`].
    Octal(u16),
}

impl RecordedMode {
    /// The literal field in an `entry` line: `unused`, or exactly four octal digits.
    #[must_use]
    pub fn token(self) -> String {
        match self {
            Self::Unused => "unused".to_owned(),
            Self::Octal(bits) => format!("{bits:04o}"),
        }
    }

    /// The mode a literal field names.
    ///
    /// Exactly four octal digits, leading zeros required — the one fixed-width numeric in this
    /// family, so the canonical-integer rule deliberately does not apply.
    #[must_use]
    pub fn of_token(token: &str) -> Option<Self> {
        if token == "unused" {
            return Some(Self::Unused);
        }
        if token.len() != 4 || !token.bytes().all(|byte| (b'0'..=b'7').contains(&byte)) {
            return None;
        }
        u16::from_str_radix(token, 8).ok().map(Self::Octal)
    }
}

/// One entry's exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyEntryBytes(Vec<u8>);

impl ApplyEntryBytes {
    /// Take ownership of the exact bytes.
    #[must_use]
    pub const fn of(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// The exact bytes.
    #[must_use]
    pub fn get(&self) -> &[u8] {
        &self.0
    }
}

/// A target-relative path, validated once and thereafter reproduced exactly.
///
/// Never a controller path, and never a capability to open anything.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedApplyPath(String);

/// The reserved device stems, refused case-insensitively with or without an extension.
const DEVICE_STEMS: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

const FORBIDDEN_PATH_BYTES: [u8; 8] = [b'\\', b':', b'<', b'>', b'"', b'|', b'?', b'*'];

impl RecordedApplyPath {
    /// Validate one path against the V1 grammar, applied identically at live construction,
    /// serialization, parsing, and materialization.
    ///
    /// # Errors
    /// Refuses anything outside the grammar. Accepted bytes are stored exactly, never cleaned.
    pub fn of(bytes: &[u8], limits: &ReceiptLimits) -> Result<Self, PathRefusal> {
        if bytes.is_empty() {
            return Err(PathRefusal::Empty);
        }
        let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if !limits.path_bytes.admits(measured) {
            return Err(PathRefusal::TooLong);
        }
        for byte in bytes {
            if !(0x20..=0x7e).contains(byte) || FORBIDDEN_PATH_BYTES.contains(byte) {
                return Err(PathRefusal::IllegalByte { byte: *byte });
            }
        }
        if bytes.first() == Some(&b'/') {
            return Err(PathRefusal::LeadingSeparator);
        }
        if bytes.last() == Some(&b'/') {
            return Err(PathRefusal::TrailingSeparator);
        }
        let text =
            core::str::from_utf8(bytes).map_err(|_| PathRefusal::IllegalByte { byte: 0x80 })?;
        for component in text.split('/') {
            Self::check_component(component)?;
        }
        Ok(Self(text.to_owned()))
    }

    fn check_component(component: &str) -> Result<(), PathRefusal> {
        if component.is_empty() {
            return Err(PathRefusal::EmptyComponent);
        }
        if component.len() > MAX_PATH_COMPONENT_BYTES {
            return Err(PathRefusal::ComponentTooLong);
        }
        if component == "." {
            return Err(PathRefusal::DotComponent);
        }
        if component == ".." {
            return Err(PathRefusal::DotDotComponent);
        }
        if component.ends_with(' ') {
            return Err(PathRefusal::ComponentTrailingSpace);
        }
        if component.ends_with('.') {
            return Err(PathRefusal::ComponentTrailingDot);
        }
        let stem = component.split('.').next().unwrap_or(component);
        if DEVICE_STEMS.contains(&stem.to_ascii_lowercase().as_str()) {
            return Err(PathRefusal::DeviceStem);
        }
        Ok(())
    }

    /// The exact accepted bytes, as text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.0
    }

    /// The exact accepted bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// One stream or file the apply uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyImageEntry {
    id: ApplyEntryId,
    path: Option<RecordedApplyPath>,
    kind: ApplyEntryKind,
    mode: RecordedMode,
    bytes: ApplyEntryBytes,
}

impl ApplyImageEntry {
    /// A file materialized at `path`.
    #[must_use]
    pub const fn file(
        id: ApplyEntryId,
        path: RecordedApplyPath,
        mode: RecordedMode,
        bytes: ApplyEntryBytes,
    ) -> Self {
        Self {
            id,
            path: Some(path),
            kind: ApplyEntryKind::File,
            mode,
            bytes,
        }
    }

    /// A stream, which carries no path and no mode by construction.
    ///
    /// One of two independent halves: the parser refuses a declared path or mode on a stream
    /// entry separately, because a document is not built through this constructor.
    #[must_use]
    pub const fn stream(id: ApplyEntryId, bytes: ApplyEntryBytes) -> Self {
        Self {
            id,
            path: None,
            kind: ApplyEntryKind::Stream,
            mode: RecordedMode::Unused,
            bytes,
        }
    }

    /// This entry's ordinal.
    #[must_use]
    pub const fn id(&self) -> ApplyEntryId {
        self.id
    }

    /// Where it materializes; absent for a stream.
    #[must_use]
    pub const fn path(&self) -> Option<&RecordedApplyPath> {
        self.path.as_ref()
    }

    /// Whether it is a stream or a file.
    #[must_use]
    pub const fn kind(&self) -> ApplyEntryKind {
        self.kind
    }

    /// Its mode.
    #[must_use]
    pub const fn mode(&self) -> RecordedMode {
        self.mode
    }

    /// Its exact bytes.
    #[must_use]
    pub const fn bytes(&self) -> &ApplyEntryBytes {
        &self.bytes
    }
}

/// One top-level authored unit the artifact covers, and the entry that materializes it.
///
/// Several roots may name one entry, which is what a flattened artifact looks like.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyRoot {
    id: ApplyRootId,
    entry: ApplyEntryId,
}

impl ApplyRoot {
    /// Name a root and the entry it lands in.
    #[must_use]
    pub const fn of(id: ApplyRootId, entry: ApplyEntryId) -> Self {
        Self { id, entry }
    }

    /// This root's ordinal.
    #[must_use]
    pub const fn id(self) -> ApplyRootId {
        self.id
    }

    /// The entry it lands in.
    #[must_use]
    pub const fn entry(self) -> ApplyEntryId {
        self.entry
    }
}

/// What one dependency edge means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApplyEdgeKind {
    /// The parent loads the child at run time.
    Loads,
    /// The parent's bytes contain the child's.
    Contains,
}

impl ApplyEdgeKind {
    /// The literal word in an `edge` line.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Loads => "loads",
            Self::Contains => "contains",
        }
    }

    /// The kind a literal word names.
    #[must_use]
    pub fn of_token(token: &str) -> Option<Self> {
        match token {
            "loads" => Some(Self::Loads),
            "contains" => Some(Self::Contains),
            _ => None,
        }
    }
}

/// One dependency edge. Ordered by endpoint then kind, which is the container's canonical order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApplyEdge {
    parent: ApplyEntryId,
    child: ApplyEntryId,
    kind: ApplyEdgeKind,
}

impl ApplyEdge {
    /// Name an edge.
    #[must_use]
    pub const fn of(parent: ApplyEntryId, child: ApplyEntryId, kind: ApplyEdgeKind) -> Self {
        Self {
            parent,
            child,
            kind,
        }
    }

    /// The depending entry.
    #[must_use]
    pub const fn parent(self) -> ApplyEntryId {
        self.parent
    }

    /// The depended-upon entry.
    #[must_use]
    pub const fn child(self) -> ApplyEntryId {
        self.child
    }

    /// What the edge means.
    #[must_use]
    pub const fn kind(self) -> ApplyEdgeKind {
        self.kind
    }
}

/// The dependency edges. An image sorts them into canonical order and refuses a repeat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyTopology {
    edges: Vec<ApplyEdge>,
}

impl ApplyTopology {
    /// Take a set of edges. Endpoints are checked against entries when an image is minted.
    #[must_use]
    pub const fn of(edges: Vec<ApplyEdge>) -> Self {
        Self { edges }
    }

    /// The edges, in canonical order once an image owns them.
    #[must_use]
    pub fn edges(&self) -> &[ApplyEdge] {
        &self.edges
    }
}

/// One apply image: exact bytes, exact paths, exact topology, and the identity over them.
///
/// The canonical encoding is minted once and stored, so the bytes an identity was computed over
/// and the bytes a consumer reads are the same bytes rather than two runs of an encoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyArtifactImage {
    id: ApplyArtifactImageId,
    canonical: Vec<u8>,
    form: RecordedArtifactForm,
    entrypoints: Vec<ApplyEntryId>,
    roots: Vec<ApplyRoot>,
    entries: Vec<ApplyImageEntry>,
    topology: ApplyTopology,
}

impl ApplyArtifactImage {
    /// Mint an image from an emitted artifact.
    ///
    /// # Errors
    /// Refuses [`RecordedArtifactForm::ExternalStream`], which has its own constructor, and
    /// every structural condition [`ApplyArtifactImage::parse`] refuses.
    pub fn of_parts(
        form: RecordedArtifactForm,
        entries: Vec<ApplyImageEntry>,
        roots: Vec<ApplyRoot>,
        entrypoints: Vec<ApplyEntryId>,
        topology: ApplyTopology,
        limits: &ReceiptLimits,
    ) -> Result<Self, ImageRefusal> {
        if form == RecordedArtifactForm::ExternalStream {
            return Err(ImageRefusal::EntryShape {
                what: "external-stream-needs-its-own-constructor",
            });
        }
        let mut edges = topology.edges;
        edges.sort_unstable();
        Self::assemble(form, entries, roots, entrypoints, edges, limits)
    }

    /// Mint an image for bytes the apply was handed rather than ones Dorc emitted.
    ///
    /// One entry that is its own root and its own entrypoint. No bundle root is invented,
    /// because there is no bundle.
    ///
    /// # Errors
    /// Refuses content past the per-entry or aggregate bound.
    pub fn of_external_stream(
        bytes: ApplyEntryBytes,
        limits: &ReceiptLimits,
    ) -> Result<Self, ImageRefusal> {
        let entry = ApplyImageEntry::stream(ApplyEntryId::of(0), bytes);
        Self::assemble(
            RecordedArtifactForm::ExternalStream,
            vec![entry],
            vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
            vec![ApplyEntryId::of(0)],
            Vec::new(),
            limits,
        )
    }

    fn assemble(
        form: RecordedArtifactForm,
        entries: Vec<ApplyImageEntry>,
        roots: Vec<ApplyRoot>,
        entrypoints: Vec<ApplyEntryId>,
        edges: Vec<ApplyEdge>,
        limits: &ReceiptLimits,
    ) -> Result<Self, ImageRefusal> {
        validate(form, &entries, &roots, &entrypoints, &edges, limits)?;
        let canonical = encode(form, &entries, &roots, &entrypoints, &edges);
        let measured = u64::try_from(canonical.len()).unwrap_or(u64::MAX);
        if !limits.image_bytes.admits(measured) {
            return Err(ImageRefusal::OverBound {
                what: "image-bytes",
            });
        }
        Ok(Self {
            id: ApplyArtifactImageId::over(&canonical),
            canonical,
            form,
            entrypoints,
            roots,
            entries,
            topology: ApplyTopology { edges },
        })
    }

    /// Read one container, recompute its identity over the exact input span, and compare.
    ///
    /// The container carries no identity of its own, so `expected` comes from the document that
    /// names it. There is deliberately no unchecked variant: an image nothing compared would be
    /// a claim rather than a binding.
    ///
    /// # Errors
    /// Refuses every departure from the exact grammar, every structural condition the mint
    /// refuses, and an identity that is not the one asked for.
    pub fn parse(
        bytes: &[u8],
        expected: ApplyArtifactImageId,
        limits: &ReceiptLimits,
    ) -> Result<Self, ImageRefusal> {
        let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if !limits.image_bytes.admits(measured) {
            return Err(ImageRefusal::OverBound {
                what: "image-bytes",
            });
        }
        let parsed = decode(bytes, limits)?;
        validate(
            parsed.form,
            &parsed.entries,
            &parsed.roots,
            &parsed.entrypoints,
            &parsed.edges,
            limits,
        )?;
        let id = ApplyArtifactImageId::over(bytes);
        if id != expected {
            return Err(ImageRefusal::IdentityMismatch);
        }
        Ok(Self {
            id,
            canonical: bytes.to_vec(),
            form: parsed.form,
            entrypoints: parsed.entrypoints,
            roots: parsed.roots,
            entries: parsed.entries,
            topology: ApplyTopology {
                edges: parsed.edges,
            },
        })
    }

    /// This image's identity.
    #[must_use]
    pub const fn id(&self) -> ApplyArtifactImageId {
        self.id
    }

    /// The exact canonical bytes, as minted or as read. Never a second run of the encoder.
    #[must_use]
    pub fn encode(&self) -> &[u8] {
        &self.canonical
    }

    /// The shape the artifact was published in.
    #[must_use]
    pub const fn form(&self) -> RecordedArtifactForm {
        self.form
    }

    /// What the apply executes, in the order the artifact supplied.
    #[must_use]
    pub fn entrypoints(&self) -> &[ApplyEntryId] {
        &self.entrypoints
    }

    /// The top-level units this artifact covers.
    #[must_use]
    pub fn roots(&self) -> &[ApplyRoot] {
        &self.roots
    }

    /// Every stream and file, in ordinal order.
    #[must_use]
    pub fn entries(&self) -> &[ApplyImageEntry] {
        &self.entries
    }

    /// The dependency edges.
    #[must_use]
    pub const fn topology(&self) -> &ApplyTopology {
        &self.topology
    }
}

fn encode(
    form: RecordedArtifactForm,
    entries: &[ApplyImageEntry],
    roots: &[ApplyRoot],
    entrypoints: &[ApplyEntryId],
    edges: &[ApplyEdge],
) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    line(&mut out, IMAGE_VERSION_LINE);
    line(&mut out, &format!("form {}", form.token()));
    line(&mut out, &format!("entrypoints {}", entrypoints.len()));
    for id in entrypoints {
        line(&mut out, &format!("entrypoint {}", id.get()));
    }
    line(&mut out, &format!("roots {}", roots.len()));
    for root in roots {
        line(
            &mut out,
            &format!("root {} {}", root.id().get(), root.entry().get()),
        );
    }
    line(&mut out, &format!("entries {}", entries.len()));
    for entry in entries {
        let path = entry.path().map_or(&[][..], RecordedApplyPath::bytes);
        let content = entry.bytes().get();
        line(
            &mut out,
            &format!(
                "entry {} {} {} {} {}",
                entry.id().get(),
                entry.kind().token(),
                entry.mode().token(),
                path.len(),
                content.len()
            ),
        );
        out.extend_from_slice(path);
        out.push(b'\n');
        out.extend_from_slice(content);
        out.push(b'\n');
    }
    line(&mut out, &format!("edges {}", edges.len()));
    for edge in edges {
        line(
            &mut out,
            &format!(
                "edge {} {} {}",
                edge.parent().get(),
                edge.child().get(),
                edge.kind().token()
            ),
        );
    }
    line(&mut out, IMAGE_END);
    out
}

fn line(out: &mut Vec<u8>, text: &str) {
    out.extend_from_slice(text.as_bytes());
    out.push(b'\n');
}

struct Decoded {
    form: RecordedArtifactForm,
    entrypoints: Vec<ApplyEntryId>,
    roots: Vec<ApplyRoot>,
    entries: Vec<ApplyImageEntry>,
    edges: Vec<ApplyEdge>,
}

/// A byte cursor. Header lines are consumed one literal space at a time; content blocks are
/// consumed by declared length and never scanned.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    const fn of(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    fn line(&mut self) -> Option<&'a str> {
        let rest = self.bytes.get(self.at..)?;
        let end = rest.iter().position(|byte| *byte == b'\n')?;
        let text = core::str::from_utf8(rest.get(..end)?).ok()?;
        self.at = self.at.checked_add(end)?.checked_add(1)?;
        Some(text)
    }

    fn exact(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(count)?;
        let out = self.bytes.get(self.at..end)?;
        self.at = end;
        Some(out)
    }

    fn newline(&mut self) -> Option<()> {
        if self.bytes.get(self.at)? != &b'\n' {
            return None;
        }
        self.at = self.at.checked_add(1)?;
        Some(())
    }

    const fn done(&self) -> bool {
        self.at == self.bytes.len()
    }
}

fn expect(cursor: &mut Cursor<'_>, want: &str, what: &'static str) -> Result<(), ImageRefusal> {
    match cursor.line() {
        Some(text) if text == want => Ok(()),
        _ => Err(ImageRefusal::Structure { what }),
    }
}

fn value<'a>(cursor: &mut Cursor<'a>, key: &'static str) -> Result<&'a str, ImageRefusal> {
    let text = cursor.line().ok_or(ImageRefusal::Structure { what: key })?;
    let rest = text
        .strip_prefix(key)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or(ImageRefusal::Structure { what: key })?;
    if rest.is_empty() {
        return Err(ImageRefusal::Structure { what: key });
    }
    Ok(rest)
}

fn count(
    cursor: &mut Cursor<'_>,
    key: &'static str,
    bound: CountLimit,
) -> Result<u64, ImageRefusal> {
    let declared = value(cursor, key)?;
    let measured = canonical_u64(declared).ok_or(ImageRefusal::Structure { what: key })?;
    if !bound.admits(measured) {
        return Err(ImageRefusal::OverBound { what: key });
    }
    Ok(measured)
}

fn ordinal(text: &str, what: &'static str) -> Result<u32, ImageRefusal> {
    canonical_u64(text)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(ImageRefusal::Identity { what })
}

fn split1<'a>(text: &'a str, what: &'static str) -> Result<(&'a str, &'a str), ImageRefusal> {
    text.split_once(' ').ok_or(ImageRefusal::Structure { what })
}

fn decode(bytes: &[u8], limits: &ReceiptLimits) -> Result<Decoded, ImageRefusal> {
    let mut cursor = Cursor::of(bytes);
    expect(&mut cursor, IMAGE_VERSION_LINE, "version")?;
    let form = RecordedArtifactForm::of_token(value(&mut cursor, "form")?)
        .ok_or(ImageRefusal::UnknownToken { what: "form" })?;

    let declared = count(&mut cursor, "entrypoints", limits.image_entries)?;
    let mut entrypoints: Vec<ApplyEntryId> = Vec::new();
    for _ in 0..declared {
        let text = value(&mut cursor, "entrypoint")?;
        entrypoints.push(ApplyEntryId::of(ordinal(text, "entrypoint")?));
    }

    let declared = count(&mut cursor, "roots", limits.image_entries)?;
    let mut roots: Vec<ApplyRoot> = Vec::new();
    for _ in 0..declared {
        let text = value(&mut cursor, "root")?;
        let (id, entry) = split1(text, "root")?;
        roots.push(ApplyRoot::of(
            ApplyRootId::of(ordinal(id, "root")?),
            ApplyEntryId::of(ordinal(entry, "root")?),
        ));
    }

    let declared = count(&mut cursor, "entries", limits.image_entries)?;
    let mut entries: Vec<ApplyImageEntry> = Vec::new();
    for _ in 0..declared {
        entries.push(decode_entry(&mut cursor, limits)?);
    }

    let declared = count(&mut cursor, "edges", limits.topology_edges)?;
    let mut edges: Vec<ApplyEdge> = Vec::new();
    for _ in 0..declared {
        let text = value(&mut cursor, "edge")?;
        let (parent, rest) = split1(text, "edge")?;
        let (child, kind) = split1(rest, "edge")?;
        edges.push(ApplyEdge::of(
            ApplyEntryId::of(ordinal(parent, "edge")?),
            ApplyEntryId::of(ordinal(child, "edge")?),
            ApplyEdgeKind::of_token(kind).ok_or(ImageRefusal::UnknownToken { what: "edge" })?,
        ));
    }

    expect(&mut cursor, IMAGE_END, "image-end")?;
    if !cursor.done() {
        return Err(ImageRefusal::TrailingBytes);
    }
    Ok(Decoded {
        form,
        entrypoints,
        roots,
        entries,
        edges,
    })
}

fn decode_entry(
    cursor: &mut Cursor<'_>,
    limits: &ReceiptLimits,
) -> Result<ApplyImageEntry, ImageRefusal> {
    let text = value(cursor, "entry")?;
    let (id, rest) = split1(text, "entry")?;
    let (kind, rest) = split1(rest, "entry")?;
    let (mode, rest) = split1(rest, "entry")?;
    let (path_len, content_len) = split1(rest, "entry")?;

    let id = ApplyEntryId::of(ordinal(id, "entry")?);
    let kind = ApplyEntryKind::of_token(kind).ok_or(ImageRefusal::UnknownToken { what: "kind" })?;
    let mode = RecordedMode::of_token(mode).ok_or(ImageRefusal::UnknownToken { what: "mode" })?;

    let path_len = canonical_u64(path_len).ok_or(ImageRefusal::Structure { what: "path-bytes" })?;
    let content_len = canonical_u64(content_len).ok_or(ImageRefusal::Structure {
        what: "content-bytes",
    })?;
    // Both declared lengths are checked against policy before a single byte is taken, so a
    // declaration cannot drive a read or a reservation on its own.
    if !limits.path_bytes.admits(path_len) {
        return Err(ImageRefusal::OverBound { what: "path-bytes" });
    }
    if !limits.image_entry_bytes.admits(content_len) {
        return Err(ImageRefusal::OverBound {
            what: "image-entry-bytes",
        });
    }
    let path_len = usize::try_from(path_len).map_err(|_| ImageRefusal::LengthMismatch)?;
    let content_len = usize::try_from(content_len).map_err(|_| ImageRefusal::LengthMismatch)?;

    let path_bytes = cursor
        .exact(path_len)
        .ok_or(ImageRefusal::LengthMismatch)?
        .to_vec();
    cursor.newline().ok_or(ImageRefusal::Structure {
        what: "path-framing",
    })?;
    let content = cursor
        .exact(content_len)
        .ok_or(ImageRefusal::LengthMismatch)?
        .to_vec();
    cursor.newline().ok_or(ImageRefusal::Structure {
        what: "content-framing",
    })?;

    match kind {
        ApplyEntryKind::Stream => {
            if !path_bytes.is_empty() {
                return Err(ImageRefusal::EntryShape {
                    what: "stream-path",
                });
            }
            if mode != RecordedMode::Unused {
                return Err(ImageRefusal::EntryShape {
                    what: "stream-mode",
                });
            }
            Ok(ApplyImageEntry::stream(id, ApplyEntryBytes::of(content)))
        }
        ApplyEntryKind::File => {
            if path_bytes.is_empty() {
                return Err(ImageRefusal::EntryShape { what: "file-path" });
            }
            let path = RecordedApplyPath::of(&path_bytes, limits)?;
            Ok(ApplyImageEntry::file(
                id,
                path,
                mode,
                ApplyEntryBytes::of(content),
            ))
        }
    }
}

fn validate(
    form: RecordedArtifactForm,
    entries: &[ApplyImageEntry],
    roots: &[ApplyRoot],
    entrypoints: &[ApplyEntryId],
    edges: &[ApplyEdge],
    limits: &ReceiptLimits,
) -> Result<(), ImageRefusal> {
    if entries.is_empty() {
        return Err(ImageRefusal::Empty { what: "entries" });
    }
    if roots.is_empty() {
        return Err(ImageRefusal::Empty { what: "roots" });
    }
    if entrypoints.is_empty() {
        return Err(ImageRefusal::Empty {
            what: "entrypoints",
        });
    }
    bound(entries.len(), limits.image_entries, "image-entries")?;
    bound(roots.len(), limits.image_entries, "roots")?;
    bound(entrypoints.len(), limits.image_entries, "entrypoints")?;
    bound(edges.len(), limits.topology_edges, "topology-edges")?;

    let mut streams: u32 = 0;
    for (index, entry) in entries.iter().enumerate() {
        let want = u32::try_from(index).map_err(|_| ImageRefusal::OverBound {
            what: "image-entries",
        })?;
        if entry.id() != ApplyEntryId::of(want) {
            return Err(ImageRefusal::Identity { what: "entry" });
        }
        bound_bytes(
            entry.bytes().get().len(),
            limits,
            "image-entry-bytes",
            |limits| limits.image_entry_bytes,
        )?;
        if let RecordedMode::Octal(bits) = entry.mode()
            && bits > MAX_MODE_BITS
        {
            return Err(ImageRefusal::EntryShape { what: "mode-bits" });
        }
        match entry.kind() {
            ApplyEntryKind::Stream => {
                if entry.path().is_some() {
                    return Err(ImageRefusal::EntryShape {
                        what: "stream-path",
                    });
                }
                if entry.mode() != RecordedMode::Unused {
                    return Err(ImageRefusal::EntryShape {
                        what: "stream-mode",
                    });
                }
                streams = streams.saturating_add(1);
                if streams > 1 {
                    return Err(ImageRefusal::SecondStream);
                }
            }
            ApplyEntryKind::File => {
                if entry.path().is_none() {
                    return Err(ImageRefusal::EntryShape { what: "file-path" });
                }
            }
        }
    }
    check_paths(entries)?;

    if form == RecordedArtifactForm::ExternalStream {
        let single = entries.len() == 1 && streams == 1 && roots.len() == 1;
        if !single || entrypoints.len() != 1 || !edges.is_empty() {
            return Err(ImageRefusal::EntryShape {
                what: "external-stream-shape",
            });
        }
    }

    for (index, root) in roots.iter().enumerate() {
        let want = u32::try_from(index).map_err(|_| ImageRefusal::OverBound { what: "roots" })?;
        if root.id() != ApplyRootId::of(want) {
            return Err(ImageRefusal::Identity { what: "root" });
        }
        if !holds(entries, root.entry()) {
            return Err(ImageRefusal::Dangling { what: "root" });
        }
    }

    let mut seen: Vec<ApplyEntryId> = Vec::new();
    for id in entrypoints {
        if !holds(entries, *id) {
            return Err(ImageRefusal::Dangling { what: "entrypoint" });
        }
        if seen.contains(id) {
            return Err(ImageRefusal::Identity { what: "entrypoint" });
        }
        seen.push(*id);
    }

    // Strictly ascending: canonical order and the no-repeat rule in one pass.
    for pair in edges.windows(2) {
        match (pair.first(), pair.get(1)) {
            (Some(left), Some(right)) if left < right => {}
            _ => return Err(ImageRefusal::EdgeOrder),
        }
    }
    for edge in edges {
        if !holds(entries, edge.parent()) || !holds(entries, edge.child()) {
            return Err(ImageRefusal::Dangling { what: "edge" });
        }
    }

    check_accounted(entries, roots, entrypoints, edges)?;
    check_depth(entries, roots, entrypoints, edges, limits)
}

fn bound(measured: usize, limit: CountLimit, what: &'static str) -> Result<(), ImageRefusal> {
    if limit.admits(u64::try_from(measured).unwrap_or(u64::MAX)) {
        Ok(())
    } else {
        Err(ImageRefusal::OverBound { what })
    }
}

fn bound_bytes(
    measured: usize,
    limits: &ReceiptLimits,
    what: &'static str,
    pick: impl Fn(&ReceiptLimits) -> crate::limits::ByteLimit,
) -> Result<(), ImageRefusal> {
    if pick(limits).admits(u64::try_from(measured).unwrap_or(u64::MAX)) {
        Ok(())
    } else {
        Err(ImageRefusal::OverBound { what })
    }
}

fn holds(entries: &[ApplyImageEntry], id: ApplyEntryId) -> bool {
    usize::try_from(id.get()).is_ok_and(|index| index < entries.len())
}

/// Exact and case-folded path uniqueness, and its sibling: one entry naming a directory another
/// entry also names as a file. Both refuse for the same reason — the pair cannot materialize.
fn check_paths(entries: &[ApplyImageEntry]) -> Result<(), ImageRefusal> {
    let mut folded: Vec<String> = entries
        .iter()
        .filter_map(|entry| entry.path().map(|path| path.text().to_ascii_lowercase()))
        .collect();
    folded.sort();
    for pair in folded.windows(2) {
        if pair.first() == pair.get(1) {
            return Err(ImageRefusal::DuplicatePath);
        }
    }
    for path in &folded {
        let mut prefix = path.clone();
        prefix.push('/');
        let at = folded.partition_point(|other| *other < prefix);
        if folded
            .get(at)
            .is_some_and(|other| other.starts_with(&prefix))
        {
            return Err(ImageRefusal::PathContainsPath);
        }
    }
    Ok(())
}

fn check_accounted(
    entries: &[ApplyImageEntry],
    roots: &[ApplyRoot],
    entrypoints: &[ApplyEntryId],
    edges: &[ApplyEdge],
) -> Result<(), ImageRefusal> {
    let mut accounted = vec![false; entries.len()];
    let mut mark = |id: ApplyEntryId| {
        if let Ok(index) = usize::try_from(id.get())
            && let Some(slot) = accounted.get_mut(index)
        {
            *slot = true;
        }
    };
    for root in roots {
        mark(root.entry());
    }
    for id in entrypoints {
        mark(*id);
    }
    for edge in edges {
        mark(edge.child());
    }
    if accounted.iter().any(|seen| !seen) {
        return Err(ImageRefusal::UnaccountedEntry);
    }
    Ok(())
}

/// The longest load chain, measured in entries, ignoring edges that close a cycle.
///
/// Cycles are recorded rather than refused: the container reports what an apply uses and does
/// not adjudicate whether the book is sensible. Ignoring the closing edge is what keeps a cycle
/// from reading as unbounded depth. Deterministic for a given input: starts are walked in
/// ascending order and children in canonical edge order.
fn check_depth(
    entries: &[ApplyImageEntry],
    roots: &[ApplyRoot],
    entrypoints: &[ApplyEntryId],
    edges: &[ApplyEdge],
    limits: &ReceiptLimits,
) -> Result<(), ImageRefusal> {
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); entries.len()];
    for edge in edges {
        if let (Ok(parent), Ok(child)) = (
            usize::try_from(edge.parent().get()),
            usize::try_from(edge.child().get()),
        ) && let Some(slot) = children.get_mut(parent)
        {
            slot.push(child);
        }
    }

    let mut colour = vec![Colour::White; entries.len()];
    let mut longest = vec![0_u64; entries.len()];
    let mut starts: Vec<usize> = Vec::new();
    for id in entrypoints
        .iter()
        .copied()
        .chain(roots.iter().map(|root| root.entry()))
    {
        if let Ok(index) = usize::try_from(id.get()) {
            starts.push(index);
        }
    }
    starts.sort_unstable();
    starts.dedup();

    let mut deepest = 0_u64;
    for start in starts {
        longest_from(start, &children, &mut colour, &mut longest);
        deepest = deepest.max(longest.get(start).copied().unwrap_or(0));
    }
    if limits.topology_depth.admits(deepest) {
        Ok(())
    } else {
        Err(ImageRefusal::TopologyDepth)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Colour {
    White,
    Grey,
    Black,
}

/// Iterative three-colour search. A grey child is an ancestor on the current path, so the edge
/// reaching it closes a cycle and contributes nothing; a black child is already measured.
fn longest_from(start: usize, children: &[Vec<usize>], colour: &mut [Colour], longest: &mut [u64]) {
    if colour.get(start) == Some(&Colour::Black) {
        return;
    }
    let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
    if let Some(slot) = colour.get_mut(start) {
        *slot = Colour::Grey;
    }
    while let Some((node, cursor)) = stack.last_mut() {
        let node = *node;
        let next = children
            .get(node)
            .and_then(|list| list.get(*cursor))
            .copied();
        if let Some(child) = next {
            *cursor = cursor.saturating_add(1);
            if colour.get(child) == Some(&Colour::White) {
                if let Some(slot) = colour.get_mut(child) {
                    *slot = Colour::Grey;
                }
                stack.push((child, 0));
            }
        } else {
            let best = children
                .get(node)
                .into_iter()
                .flatten()
                .filter(|child| colour.get(**child) == Some(&Colour::Black))
                .filter_map(|child| longest.get(*child).copied())
                .max()
                .unwrap_or(0);
            if let Some(slot) = longest.get_mut(node) {
                *slot = best.saturating_add(1);
            }
            if let Some(slot) = colour.get_mut(node) {
                *slot = Colour::Black;
            }
            stack.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(text: &str) -> RecordedApplyPath {
        RecordedApplyPath::of(text.as_bytes(), &ReceiptLimits::V1).expect("a legal path")
    }

    fn refusal(text: &str) -> PathRefusal {
        RecordedApplyPath::of(text.as_bytes(), &ReceiptLimits::V1).expect_err("an illegal path")
    }

    #[test]
    fn a_legal_path_is_stored_exactly_and_never_cleaned() {
        // The grammar refuses; it does not repair. A path that needed normalizing to be legal
        // would mean the recorded bytes are not the applied bytes.
        for text in [
            "plan.sh",
            "lib/a.dorc-bundle.sh",
            "a/b/c/d.sh",
            "with space/x.sh",
            "UPPER.SH",
            "dotted.name.sh",
        ] {
            assert_eq!(ok(text).text(), text);
        }
    }

    #[test]
    fn traversal_and_absolute_shapes_refuse_rather_than_resolve() {
        assert_eq!(refusal("../x"), PathRefusal::DotDotComponent);
        assert_eq!(refusal("a/../b"), PathRefusal::DotDotComponent);
        assert_eq!(refusal("/a"), PathRefusal::LeadingSeparator);
        assert_eq!(refusal("a/"), PathRefusal::TrailingSeparator);
        assert_eq!(refusal("a//b"), PathRefusal::EmptyComponent);
        assert_eq!(refusal("./a"), PathRefusal::DotComponent);
        assert_eq!(refusal(""), PathRefusal::Empty);
    }

    #[test]
    fn the_portable_byte_set_is_exactly_what_the_grammar_names() {
        for (text, byte) in [
            ("a\\b", b'\\'),
            ("c:/x", b':'),
            ("a<b", b'<'),
            ("a>b", b'>'),
            ("a\"b", b'"'),
            ("a|b", b'|'),
            ("a?b", b'?'),
            ("a*b", b'*'),
            ("a\tb", b'\t'),
            ("a\rb", b'\r'),
            ("a\x7fb", 0x7f),
        ] {
            assert_eq!(
                refusal(text),
                PathRefusal::IllegalByte { byte },
                "{text:?} should refuse"
            );
        }
        assert!(matches!(
            RecordedApplyPath::of(&[b'a', 0x00, b'b'], &ReceiptLimits::V1),
            Err(PathRefusal::IllegalByte { byte: 0 })
        ));
        assert!(matches!(
            RecordedApplyPath::of(&[b'a', 0xff], &ReceiptLimits::V1),
            Err(PathRefusal::IllegalByte { byte: 0xff })
        ));
    }

    #[test]
    fn windows_hostile_component_shapes_refuse() {
        // A component ending in space or dot, and the reserved device stems, are silently
        // rewritten or rejected by one platform's filesystem and not the other's, so a path
        // carrying them could not materialize identically on both.
        assert_eq!(refusal("a /b.sh"), PathRefusal::ComponentTrailingSpace);
        assert_eq!(refusal("a./b.sh"), PathRefusal::ComponentTrailingDot);
        for text in [
            "CON", "con", "Con.txt", "aux/x.sh", "a/NUL", "com1.sh", "LPT9",
        ] {
            assert_eq!(refusal(text), PathRefusal::DeviceStem, "{text:?}");
        }
        // Not device stems: the list is exact, not a prefix rule.
        for text in ["console.sh", "com0.sh", "com10.sh", "auxiliary"] {
            assert_eq!(ok(text).text(), text);
        }
    }

    #[test]
    fn a_component_and_a_path_each_have_their_own_bound() {
        let long_component = "x".repeat(MAX_PATH_COMPONENT_BYTES);
        assert_eq!(ok(&long_component).text().len(), MAX_PATH_COMPONENT_BYTES);
        assert_eq!(
            refusal(&"x".repeat(MAX_PATH_COMPONENT_BYTES.saturating_add(1))),
            PathRefusal::ComponentTooLong
        );
        // Boundary-at and boundary-plus on the policy bound, built from legal components.
        let unit = format!("{}/", "y".repeat(63));
        let at = unit.repeat(64);
        let at = at.trim_end_matches('/');
        assert_eq!(at.len(), 4095);
        assert_eq!(ok(at).text().len(), 4095);
        let over = "z".repeat(200);
        let over = format!("{over}/").repeat(21);
        assert_eq!(
            refusal(over.trim_end_matches('/')),
            PathRefusal::TooLong,
            "past the policy bound"
        );
    }

    #[test]
    fn a_mode_field_is_unused_or_exactly_four_octal_digits() {
        assert_eq!(RecordedMode::Unused.token(), "unused");
        assert_eq!(RecordedMode::Octal(0o755).token(), "0755");
        assert_eq!(RecordedMode::Octal(0o0).token(), "0000");
        assert_eq!(RecordedMode::Octal(0o4755).token(), "4755");
        for token in ["unused", "0755", "0000", "4755"] {
            assert_eq!(
                RecordedMode::of_token(token)
                    .map(RecordedMode::token)
                    .as_deref(),
                Some(token)
            );
        }
        for token in ["755", "00755", "0o755", "0778", "", "UNUSED", "absent"] {
            assert_eq!(RecordedMode::of_token(token), None, "{token:?}");
        }
    }

    #[test]
    fn every_form_and_edge_token_round_trips_and_none_is_shared() {
        let mut tokens: Vec<&str> = RecordedArtifactForm::ALL
            .iter()
            .map(|f| f.token())
            .collect();
        let before = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(before, tokens.len());
        for form in RecordedArtifactForm::ALL {
            assert_eq!(RecordedArtifactForm::of_token(form.token()), Some(form));
        }
        assert_eq!(RecordedArtifactForm::of_token("bundled"), None);
        for kind in [ApplyEdgeKind::Loads, ApplyEdgeKind::Contains] {
            assert_eq!(ApplyEdgeKind::of_token(kind.token()), Some(kind));
        }
        assert_eq!(ApplyEdgeKind::of_token("includes"), None);
    }
}
