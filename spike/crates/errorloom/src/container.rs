//! The case-file container: txtar-with-flat-YAML-frontmatter (`282` §2 /
//! `282:rul-frontmatter-txtar-container`).
//!
//! A case file is one txtar archive with a `---`-fenced flat-YAML frontmatter
//! head: structured metadata, then file/CLI-state sections, then a final
//! `-- replay --` section of `$ `-prefixed command blocks. errorloom treats the
//! frontmatter as an OPAQUE flat map (`28A` §1) — the schema belongs to
//! consumers. txtar is hand-rolled here (~100 lines) to keep the crate
//! dependency-free and off the `cargo deny` surface, matching its
//! dependency-free kernel posture.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::path::PathBuf;

/// Limits owned by the case/replay boundary (`282` §2 / §7). The file reader
/// admits at most [`MAX_CASE_BYTES`] before UTF-8 decoding; parser limits keep
/// individual stored values bounded for callers of [`Case::parse`].
pub const MAX_CASE_BYTES: usize = 256 * 1024;
/// Maximum txtar sections, including the final replay section.
pub const MAX_SECTION_COUNT: usize = 64;
/// Maximum bytes in one txtar section, including replay before it is parsed.
pub const MAX_SECTION_BYTES: usize = 128 * 1024;
/// Maximum replay blocks in one case.
pub const MAX_REPLAY_BLOCKS: usize = 32;
/// Maximum bytes in a replay command line.
pub const MAX_REPLAY_COMMAND_BYTES: usize = 8 * 1024;
/// Maximum committed output bytes in one replay block.
pub const MAX_REPLAY_OUTPUT_BYTES: usize = 64 * 1024;

/// A parsed case file: opaque frontmatter, verbatim file sections, and the final
/// replay section (`282` §2). Round-trips byte-identically modulo the LF pin: the
/// frontmatter block and file sections are preserved verbatim; only replay output
/// is rewritten at bless.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Case {
    frontmatter: Frontmatter,
    preamble: String,
    sections: Vec<Section>,
    replay: ReplaySection,
}

/// Byte layout of the mutable replay-output islands in one parsed case.
///
/// The container owns these spans only; consumers decide whether an output is
/// editable. Every byte outside them remains structural transcript text.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CaseLayout {
    replay_outputs: Vec<Range<usize>>,
}

impl CaseLayout {
    /// Replay output byte spans in source order.
    #[must_use]
    pub fn replay_outputs(&self) -> &[Range<usize>] {
        &self.replay_outputs
    }

    /// Compare all container bytes other than replay outputs exactly.
    #[must_use]
    pub fn same_non_replay_output_bytes(&self, text: &str, other: &Self, other_text: &str) -> bool {
        self.replay_outputs.len() == other.replay_outputs.len()
            && non_output_chunks(text, &self.replay_outputs)
                .eq(non_output_chunks(other_text, &other.replay_outputs))
    }
}

/// The opaque flat frontmatter map: `key: value` scalars and `key:` + `- item`
/// lists (`282:lean-flat-frontmatter-subset`). Nested structures refuse. The raw
/// text is preserved for byte-exact round-trip; the parsed entries drive only the
/// consumer-facing accessors and the required-token gate.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Frontmatter {
    raw: String,
    entries: BTreeMap<String, FrontmatterValue>,
}

/// A flat frontmatter value: a scalar or a list of scalars.
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum FrontmatterValue {
    /// A single `key: value` scalar.
    Scalar(String),
    /// A `key:` header with `- item` list entries.
    List(Vec<String>),
}

impl Frontmatter {
    /// The value for `key`, if present.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&FrontmatterValue> {
        self.entries.get(key)
    }

    /// The scalar value for `key`, if present and scalar.
    #[must_use]
    pub fn scalar(&self, key: &str) -> Option<&str> {
        match self.entries.get(key) {
            Some(FrontmatterValue::Scalar(s)) => Some(s.as_str()),
            _ => None,
        }
    }
}

/// One txtar file section: a name and its verbatim LF content.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Section {
    name: String,
    content: String,
}

impl Section {
    /// The section name (a relative, `/`-joined path when materialized).
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The verbatim section content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// The final `-- replay --` section: an ordered list of command blocks.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ReplaySection {
    blocks: Vec<ReplayBlock>,
}

impl ReplaySection {
    /// The command blocks in order.
    #[must_use]
    pub fn blocks(&self) -> &[ReplayBlock] {
        &self.blocks
    }
}

/// One replay block: a `$ `-prefixed command line and its inlined output.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplayBlock {
    command: String,
    output: String,
}

impl ReplayBlock {
    /// The command text (the bytes after the `$ ` prefix).
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// The inlined output for this block (verbatim, may be empty).
    #[must_use]
    pub fn output(&self) -> &str {
        &self.output
    }
}

/// Why a case failed to parse or a hygiene gate refused (`282` §2 / `28A` §1).
/// Blunt by design (`282:rul-internal-tool-sharp-edges`).
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum CaseError {
    /// A parsed component exceeded its owning case boundary.
    LimitExceeded {
        /// The bounded component.
        component: &'static str,
        /// The maximum accepted byte or item count.
        limit: usize,
    },
    /// The file did not open with a `---` frontmatter fence.
    MissingFrontmatter,
    /// The frontmatter fence was never closed.
    UnterminatedFrontmatter,
    /// A frontmatter line was indented where the flat subset forbids nesting.
    FrontmatterNotFlat {
        /// Zero-based line index within the frontmatter block.
        line: usize,
    },
    /// A frontmatter line was neither `key: value` nor a `- item`.
    FrontmatterSyntax {
        /// Zero-based line index within the frontmatter block.
        line: usize,
    },
    /// The raw bytes contain a carriage return; sections are LF-only.
    ContainsCrlf,
    /// A section name escapes the materialization dir (absolute or `..`).
    UnsafeSectionName {
        /// The offending name.
        name: String,
    },
    /// Two sections share a name (materialize would clobber).
    DuplicateSection {
        /// The repeated name.
        name: String,
    },
    /// No `-- replay --` section was present.
    NoReplaySection,
    /// The replay section was not the final section.
    ReplayNotLast,
    /// The replay section held no `$ ` command block.
    EmptyReplay,
    /// The replay section had non-command text before its first `$ `.
    ReplayPreamble,
    /// A replay output line parses as a txtar marker (no escaping exists).
    MarkerCollision {
        /// The offending line.
        line: String,
    },
    /// A replay output line leaked the sandbox's absolute path.
    SandboxPathLeak {
        /// The offending line.
        line: String,
    },
    /// A required-token block did not surface the configured key's value.
    MissingRequiredToken {
        /// Zero-based block index.
        block: usize,
        /// The token that had to appear.
        token: String,
    },
}

impl fmt::Display for CaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaseError::LimitExceeded { component, limit } => {
                write!(f, "case: {component} exceeds limit {limit}")
            }
            CaseError::MissingFrontmatter => f.write_str("case: missing `---` frontmatter fence"),
            CaseError::UnterminatedFrontmatter => {
                f.write_str("case: unterminated frontmatter (no closing `---`)")
            }
            CaseError::FrontmatterNotFlat { line } => {
                write!(
                    f,
                    "case: frontmatter line {line} is nested; the subset is flat"
                )
            }
            CaseError::FrontmatterSyntax { line } => {
                write!(
                    f,
                    "case: frontmatter line {line} is not `key: value` or `- item`"
                )
            }
            CaseError::ContainsCrlf => f.write_str("case: CRLF found; sections are LF-only"),
            CaseError::UnsafeSectionName { name } => {
                write!(
                    f,
                    "case: section name {name:?} escapes the materialization dir"
                )
            }
            CaseError::DuplicateSection { name } => {
                write!(f, "case: duplicate section {name:?}")
            }
            CaseError::NoReplaySection => f.write_str("case: no `-- replay --` section"),
            CaseError::ReplayNotLast => f.write_str("case: `-- replay --` is not the last section"),
            CaseError::EmptyReplay => f.write_str("case: the replay section has no `$ ` command"),
            CaseError::ReplayPreamble => {
                f.write_str("case: text before the first `$ ` in the replay section")
            }
            CaseError::MarkerCollision { line } => {
                write!(
                    f,
                    "case: replay output line parses as a txtar marker: {line:?}"
                )
            }
            CaseError::SandboxPathLeak { line } => {
                write!(f, "case: replay output leaked the sandbox path: {line:?}")
            }
            CaseError::MissingRequiredToken { block, token } => {
                write!(f, "case: replay block {block} does not surface {token:?}")
            }
        }
    }
}

impl std::error::Error for CaseError {}

/// Why bounded case-file admission failed before [`Case::parse`] can receive a
/// full text buffer (`282` §2).
#[derive(Debug)]
#[non_exhaustive]
pub enum CaseReadError {
    /// The file could not be opened or read.
    Io(std::io::Error),
    /// The file exceeded [`MAX_CASE_BYTES`] during the bounded read.
    TooLarge,
    /// The admitted bytes were not UTF-8, while case files remain text-only.
    NonUtf8(std::string::FromUtf8Error),
    /// The admitted text failed case parsing.
    Parse(CaseError),
}

impl fmt::Display for CaseReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaseReadError::Io(error) => write!(f, "case read: {error}"),
            CaseReadError::TooLarge => write!(f, "case read: file exceeds limit {MAX_CASE_BYTES}"),
            CaseReadError::NonUtf8(error) => write!(f, "case read: non-UTF-8 input: {error}"),
            CaseReadError::Parse(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CaseReadError {}

/// Read and parse a case without allocating or UTF-8-decoding beyond the
/// file-admission ceiling.
///
/// # Errors
/// Returns [`CaseReadError`] for I/O, file-size, UTF-8, or parse failures.
pub fn read_case(path: impl AsRef<Path>) -> Result<Case, CaseReadError> {
    let text = read_case_text(path)?;
    Case::parse(&text).map_err(CaseReadError::Parse)
}

/// Read text admitted under [`MAX_CASE_BYTES`] without parsing it.
///
/// # Errors
/// Returns [`CaseReadError`] for I/O, file-size, or UTF-8 failures.
pub fn read_case_text(path: impl AsRef<Path>) -> Result<String, CaseReadError> {
    let mut bytes = Vec::with_capacity(MAX_CASE_BYTES.saturating_add(1));
    File::open(path)
        .map_err(CaseReadError::Io)?
        .take(u64::try_from(MAX_CASE_BYTES.saturating_add(1)).unwrap_or(u64::MAX))
        .read_to_end(&mut bytes)
        .map_err(CaseReadError::Io)?;
    if bytes.len() > MAX_CASE_BYTES {
        return Err(CaseReadError::TooLarge);
    }
    String::from_utf8(bytes).map_err(CaseReadError::NonUtf8)
}

/// The name every case's final section must carry to be recognized as replay.
pub const REPLAY_SECTION: &str = "replay";

impl Case {
    /// Parse a case file from its text. The caller owns the total-buffer bound;
    /// [`read_case`] is the bounded file-admission edge.
    ///
    /// # Errors
    /// Returns [`CaseError`] for a malformed frontmatter block, nested
    /// frontmatter, CRLF bytes, an unsafe or duplicate section name, or a
    /// missing / non-final / empty replay section.
    pub fn parse(text: &str) -> Result<Self, CaseError> {
        if text.contains('\r') {
            return Err(CaseError::ContainsCrlf);
        }
        let (frontmatter, body) = split_frontmatter(text)?;
        let (preamble, raw_sections) = parse_txtar(body)?;

        let mut sections: Vec<Section> = Vec::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut replay_raw: Option<String> = None;
        let section_count = raw_sections.len();
        for (index, (name, content)) in raw_sections.into_iter().enumerate() {
            if name == REPLAY_SECTION {
                if index != section_count.saturating_sub(1) {
                    return Err(CaseError::ReplayNotLast);
                }
                replay_raw = Some(content);
                continue;
            }
            if safe_relative(&name).is_none() {
                return Err(CaseError::UnsafeSectionName { name: name.clone() });
            }
            if !seen.insert(name.clone()) {
                return Err(CaseError::DuplicateSection { name: name.clone() });
            }
            sections.push(Section {
                name,
                content: strip_trailing_separator(&content),
            });
        }

        let Some(replay_raw) = replay_raw else {
            return Err(CaseError::NoReplaySection);
        };
        let replay = parse_replay(&replay_raw)?;

        Ok(Case {
            frontmatter,
            preamble,
            sections,
            replay,
        })
    }

    /// Parse the generic container and identify only its replay-output spans.
    ///
    /// This intentionally exposes raw container layout without assigning any
    /// consumer-specific meaning to the output bytes.
    ///
    /// # Errors
    ///
    /// Returns the same malformed-container refusal as [`Case::parse`].
    pub fn raw_layout(text: &str) -> Result<CaseLayout, CaseError> {
        let case = Self::parse(text)?;
        let replay_header = format!("-- {REPLAY_SECTION} --\n");
        let replay_start = text
            .rfind(&replay_header)
            .map(|offset| offset.saturating_add(replay_header.len()))
            .ok_or(CaseError::NoReplaySection)?;
        let replay = text.get(replay_start..).ok_or(CaseError::NoReplaySection)?;
        let mut commands = Vec::new();
        let mut offset = replay_start;
        for line in replay.split_inclusive('\n') {
            if line.strip_suffix('\n').unwrap_or(line).starts_with("$ ") {
                commands.push(offset);
            }
            offset = offset.saturating_add(line.len());
        }
        if commands.len() != case.replay.blocks.len() {
            return Err(CaseError::ReplayPreamble);
        }
        let mut replay_outputs = Vec::new();
        for (index, command_start) in commands.iter().copied().enumerate() {
            let command_end = text
                .get(command_start..)
                .and_then(|rest| {
                    rest.find('\n')
                        .map(|end| command_start.saturating_add(end.saturating_add(1)))
                })
                .ok_or(CaseError::EmptyReplay)?;
            let raw_end = commands
                .get(index.saturating_add(1))
                .copied()
                .unwrap_or(text.len());
            let raw = text
                .get(command_end..raw_end)
                .ok_or(CaseError::EmptyReplay)?;
            let end = command_end.saturating_add(strip_trailing_separator(raw).len());
            replay_outputs.push(command_end..end);
        }
        Ok(CaseLayout { replay_outputs })
    }

    /// The opaque frontmatter map.
    #[must_use]
    pub fn frontmatter(&self) -> &Frontmatter {
        &self.frontmatter
    }

    /// The file sections (everything but replay), in order.
    #[must_use]
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    /// The replay section.
    #[must_use]
    pub fn replay(&self) -> &ReplaySection {
        &self.replay
    }

    /// Replace every replay block's output with the freshly-captured bytes
    /// (inline-on-bless, `282` §7). One output per block, in order; a length
    /// mismatch is a caller bug and leaves extra blocks untouched.
    pub fn set_replay_outputs(&mut self, outputs: Vec<String>) {
        for (block, out) in self.replay.blocks.iter_mut().zip(outputs) {
            block.output = out;
        }
    }

    /// Serialize back to case-file text (LF pinned). CANONICAL form (`282` §12 items 3–4): a blank
    /// line separates each header from the body above it and each replay block from the one above it.
    /// Round-trips byte-identically because parse STRIPS those separators (emit-and-tolerate); a
    /// source lacking them normalizes TO them.
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("---\n");
        out.push_str(self.frontmatter.raw.as_str());
        out.push_str("---\n");
        out.push_str(&self.preamble);
        // A blank line before every header that follows a body (never before the first).
        let mut after_body = false;
        for section in &self.sections {
            ensure_trailing_lf(&mut out);
            if after_body {
                out.push('\n');
            }
            out.push_str("-- ");
            out.push_str(&section.name);
            out.push_str(" --\n");
            out.push_str(&section.content);
            after_body = true;
        }
        ensure_trailing_lf(&mut out);
        if after_body {
            out.push('\n');
        }
        out.push_str("-- ");
        out.push_str(REPLAY_SECTION);
        out.push_str(" --\n");
        for (index, block) in self.replay.blocks.iter().enumerate() {
            ensure_trailing_lf(&mut out);
            if index > 0 {
                out.push('\n');
            }
            out.push_str("$ ");
            out.push_str(&block.command);
            out.push('\n');
            out.push_str(&block.output);
        }
        out
    }

    /// The safe relative paths + contents to write when materializing this case's
    /// file sections (`282` §7). The runner writes these under a temp dir; kept
    /// pure here so the layout is unit-testable without disk.
    #[must_use]
    pub fn materialized_files(&self) -> Vec<(PathBuf, &str)> {
        self.sections
            .iter()
            .filter_map(|s| safe_relative(&s.name).map(|p| (p, s.content.as_str())))
            .collect()
    }

    /// The static hygiene gates over the current replay blocks (`28A` §1): every
    /// output line that parses as a txtar marker refuses; when `required_key` is
    /// set and present in frontmatter, every block's output must surface its
    /// scalar value.
    ///
    /// # Errors
    /// Returns [`CaseError::MarkerCollision`] or [`CaseError::MissingRequiredToken`].
    pub fn check_hygiene(&self, required_key: Option<&str>) -> Result<(), CaseError> {
        for block in &self.replay.blocks {
            for line in block.output.lines() {
                if marker_name(line).is_some() {
                    return Err(CaseError::MarkerCollision {
                        line: line.to_owned(),
                    });
                }
            }
        }
        if let Some(key) = required_key
            && let Some(token) = self.frontmatter.scalar(key)
            && !token.is_empty()
        {
            for (block_index, block) in self.replay.blocks.iter().enumerate() {
                if !block.output.contains(token) {
                    return Err(CaseError::MissingRequiredToken {
                        block: block_index,
                        token: token.to_owned(),
                    });
                }
            }
        }
        Ok(())
    }
}

fn non_output_chunks<'a>(
    text: &'a str,
    spans: &'a [Range<usize>],
) -> impl Iterator<Item = &'a str> {
    let mut chunks = Vec::with_capacity(spans.len().saturating_add(1));
    let mut start = 0;
    for span in spans {
        let Some(chunk) = text.get(start..span.start) else {
            return Vec::new().into_iter();
        };
        chunks.push(chunk);
        start = span.end;
    }
    match text.get(start..) {
        Some(chunk) => chunks.push(chunk),
        None => return Vec::new().into_iter(),
    }
    chunks.into_iter()
}

/// Insert an LF before an about-to-be-written marker/command when the text does
/// not already end with one (`swe-F6`): a captured output lacking a trailing
/// newline (or a section content) must never fuse the following `-- name --` /
/// `$ ` onto its last line, or the case no longer round-trips through parse.
fn ensure_trailing_lf(out: &mut String) {
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
}

/// Split leading `---`-fenced frontmatter from the txtar body.
fn split_frontmatter(text: &str) -> Result<(Frontmatter, &str), CaseError> {
    let mut offset = 0;
    let mut fm_end: Option<(usize, usize)> = None;
    let mut first = true;
    let mut opened = false;
    for line in text.split_inclusive('\n') {
        let bare = line.strip_suffix('\n').unwrap_or(line);
        if first {
            if bare != "---" {
                return Err(CaseError::MissingFrontmatter);
            }
            opened = true;
            first = false;
        } else if bare == "---" {
            fm_end = Some((offset, offset.saturating_add(line.len())));
            break;
        }
        offset = offset.saturating_add(line.len());
    }
    if !opened {
        return Err(CaseError::MissingFrontmatter);
    }
    let Some((raw_end, body_start)) = fm_end else {
        return Err(CaseError::UnterminatedFrontmatter);
    };
    let raw_start = text.find('\n').map_or(raw_end, |i| i.saturating_add(1));
    let raw = text.get(raw_start..raw_end).unwrap_or_default().to_owned();
    let body = text.get(body_start..).unwrap_or_default();
    let entries = parse_frontmatter_entries(&raw)?;
    Ok((Frontmatter { raw, entries }, body))
}

/// Parse the flat frontmatter subset (`282:lean-flat-frontmatter-subset`).
fn parse_frontmatter_entries(raw: &str) -> Result<BTreeMap<String, FrontmatterValue>, CaseError> {
    let lines: Vec<&str> = raw.lines().collect();
    let mut entries: BTreeMap<String, FrontmatterValue> = BTreeMap::new();
    let mut i = 0;
    while i < lines.len() {
        let Some(raw_line) = lines.get(i) else { break };
        if raw_line.trim().is_empty() {
            i = i.saturating_add(1);
            continue;
        }
        if raw_line.starts_with(char::is_whitespace) {
            return Err(CaseError::FrontmatterNotFlat { line: i });
        }
        let Some((key, rest)) = raw_line.split_once(':') else {
            return Err(CaseError::FrontmatterSyntax { line: i });
        };
        let key = key.trim();
        let rest = rest.trim();
        if key.is_empty() {
            return Err(CaseError::FrontmatterSyntax { line: i });
        }
        if rest.is_empty() {
            let (items, next) = parse_list_items(&lines, i.saturating_add(1))?;
            if items.is_empty() {
                entries.insert(key.to_owned(), FrontmatterValue::Scalar(String::new()));
                i = i.saturating_add(1);
            } else {
                entries.insert(key.to_owned(), FrontmatterValue::List(items));
                i = next;
            }
        } else {
            entries.insert(key.to_owned(), FrontmatterValue::Scalar(rest.to_owned()));
            i = i.saturating_add(1);
        }
    }
    Ok(entries)
}

/// Collect the indented `- item` lines starting at `start`; returns the items and
/// the index of the first line past them.
fn parse_list_items(lines: &[&str], start: usize) -> Result<(Vec<String>, usize), CaseError> {
    let mut items: Vec<String> = Vec::new();
    let mut j = start;
    while let Some(line) = lines.get(j) {
        if !line.starts_with(char::is_whitespace) {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(item) = trimmed.strip_prefix("- ") {
            items.push(item.trim().to_owned());
        } else if trimmed == "-" {
            items.push(String::new());
        } else {
            return Err(CaseError::FrontmatterNotFlat { line: j });
        }
        j = j.saturating_add(1);
    }
    Ok((items, j))
}

/// Parse a txtar body into an optional comment/preamble and its named sections.
fn parse_txtar(body: &str) -> Result<(String, Vec<(String, String)>), CaseError> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut preamble = String::new();
    let mut current: Option<(String, String)> = None;
    for line in body.split_inclusive('\n') {
        let bare = line.strip_suffix('\n').unwrap_or(line);
        if let Some(name) = marker_name(bare) {
            if let Some(section) = current.take() {
                sections.push(section);
            }
            if sections.len() >= MAX_SECTION_COUNT {
                return Err(CaseError::LimitExceeded {
                    component: "section count",
                    limit: MAX_SECTION_COUNT,
                });
            }
            current = Some((name, String::new()));
        } else if let Some((_, content)) = current.as_mut() {
            if content.len().saturating_add(line.len()) > MAX_SECTION_BYTES {
                return Err(CaseError::LimitExceeded {
                    component: "section bytes",
                    limit: MAX_SECTION_BYTES,
                });
            }
            content.push_str(line);
        } else {
            preamble.push_str(line);
        }
    }
    if let Some(section) = current.take() {
        sections.push(section);
    }
    Ok((preamble, sections))
}

/// The txtar marker name for a line, or `None` if it is not a `-- name --` marker.
fn marker_name(line: &str) -> Option<String> {
    let inner = line.strip_prefix("-- ")?.strip_suffix(" --")?;
    let name = inner.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_owned())
    }
}

/// Drop a single trailing blank line — the canonical separator [`Case::to_text`] inserts before a
/// header / replay block (`282` §12 items 3–4, the "tolerate" half). Idempotent with the emit: a
/// content already ending in a blank line loses exactly one `\n`, which `to_text` restores.
fn strip_trailing_separator(content: &str) -> String {
    match content.strip_suffix('\n') {
        Some(rest) if rest.is_empty() || rest.ends_with('\n') => rest.to_owned(),
        _ => content.to_owned(),
    }
}

/// Parse a replay section's content into `$ `-prefixed command blocks.
fn parse_replay(content: &str) -> Result<ReplaySection, CaseError> {
    let mut blocks: Vec<ReplayBlock> = Vec::new();
    let mut current: Option<ReplayBlock> = None;
    for line in content.split_inclusive('\n') {
        let bare = line.strip_suffix('\n').unwrap_or(line);
        if let Some(command) = bare.strip_prefix("$ ") {
            if let Some(mut block) = current.take() {
                block.output = strip_trailing_separator(&block.output);
                blocks.push(block);
            }
            if blocks.len() >= MAX_REPLAY_BLOCKS {
                return Err(CaseError::LimitExceeded {
                    component: "replay block count",
                    limit: MAX_REPLAY_BLOCKS,
                });
            }
            if command.len() > MAX_REPLAY_COMMAND_BYTES {
                return Err(CaseError::LimitExceeded {
                    component: "replay command bytes",
                    limit: MAX_REPLAY_COMMAND_BYTES,
                });
            }
            current = Some(ReplayBlock {
                command: command.to_owned(),
                output: String::new(),
            });
        } else if let Some(block) = current.as_mut() {
            if block.output.len().saturating_add(line.len()) > MAX_REPLAY_OUTPUT_BYTES {
                return Err(CaseError::LimitExceeded {
                    component: "committed replay output bytes",
                    limit: MAX_REPLAY_OUTPUT_BYTES,
                });
            }
            block.output.push_str(line);
        } else if !bare.trim().is_empty() {
            return Err(CaseError::ReplayPreamble);
        }
    }
    if let Some(mut block) = current.take() {
        block.output = strip_trailing_separator(&block.output);
        blocks.push(block);
    }
    if blocks.is_empty() {
        return Err(CaseError::EmptyReplay);
    }
    Ok(ReplaySection { blocks })
}

/// A safe, relative, `/`-joined materialization path for a section name, or
/// `None` if it is absolute, empty, or climbs out with `..` / drive syntax.
fn safe_relative(name: &str) -> Option<PathBuf> {
    let mut path = PathBuf::new();
    let mut any = false;
    for component in name.split('/') {
        if component.is_empty()
            || component == "."
            || component == ".."
            || component.contains('\\')
            || component.contains(':')
        {
            return None;
        }
        path.push(component);
        any = true;
    }
    any.then_some(path)
}
