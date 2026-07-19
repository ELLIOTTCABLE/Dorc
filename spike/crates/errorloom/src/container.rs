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
use std::path::PathBuf;

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

/// The name every case's final section must carry to be recognized as replay.
pub const REPLAY_SECTION: &str = "replay";

impl Case {
    /// Parse a case file from its text.
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
        let (preamble, raw_sections) = parse_txtar(body);

        let mut sections: Vec<Section> = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut replay_raw: Option<String> = None;
        for (index, (name, content)) in raw_sections.iter().enumerate() {
            if name == REPLAY_SECTION {
                if index != raw_sections.len().saturating_sub(1) {
                    return Err(CaseError::ReplayNotLast);
                }
                replay_raw = Some(content.clone());
                continue;
            }
            if safe_relative(name).is_none() {
                return Err(CaseError::UnsafeSectionName { name: name.clone() });
            }
            if !seen.insert(name.as_str()) {
                return Err(CaseError::DuplicateSection { name: name.clone() });
            }
            sections.push(Section {
                name: name.clone(),
                content: content.clone(),
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

    /// Serialize back to case-file text (LF pinned). Byte-identical to the
    /// source when no output was rewritten (`282` §7 round-trip gate).
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("---\n");
        out.push_str(self.frontmatter.raw.as_str());
        out.push_str("---\n");
        out.push_str(&self.preamble);
        for section in &self.sections {
            out.push_str("-- ");
            out.push_str(&section.name);
            out.push_str(" --\n");
            out.push_str(&section.content);
        }
        out.push_str("-- ");
        out.push_str(REPLAY_SECTION);
        out.push_str(" --\n");
        for block in &self.replay.blocks {
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
fn parse_txtar(body: &str) -> (String, Vec<(String, String)>) {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut preamble = String::new();
    let mut current: Option<(String, String)> = None;
    for line in body.split_inclusive('\n') {
        let bare = line.strip_suffix('\n').unwrap_or(line);
        if let Some(name) = marker_name(bare) {
            if let Some(section) = current.take() {
                sections.push(section);
            }
            current = Some((name, String::new()));
        } else if let Some((_, content)) = current.as_mut() {
            content.push_str(line);
        } else {
            preamble.push_str(line);
        }
    }
    if let Some(section) = current.take() {
        sections.push(section);
    }
    (preamble, sections)
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

/// Parse a replay section's content into `$ `-prefixed command blocks.
fn parse_replay(content: &str) -> Result<ReplaySection, CaseError> {
    let mut blocks: Vec<ReplayBlock> = Vec::new();
    let mut current: Option<ReplayBlock> = None;
    for line in content.split_inclusive('\n') {
        let bare = line.strip_suffix('\n').unwrap_or(line);
        if let Some(command) = bare.strip_prefix("$ ") {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            current = Some(ReplayBlock {
                command: command.to_owned(),
                output: String::new(),
            });
        } else if let Some(block) = current.as_mut() {
            block.output.push_str(line);
        } else if !bare.trim().is_empty() {
            return Err(CaseError::ReplayPreamble);
        }
    }
    if let Some(block) = current.take() {
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
