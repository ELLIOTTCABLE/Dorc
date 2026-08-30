//! The receipt-owned inert mirror of `aid::locator::Locator` (`30Ra:planning-book-bytes-and-durable-locators`).
//!
//! # Why a mirror and not serde
//!
//! The live `Locator` is a describe-plane value holding process-local `SourceFileId`s and
//! `StageId`s. Serializing it would put two things in a document that cannot mean anything there:
//! ids whose id-space died with the process, and a type whose deserialization would hand a reader
//! back a *live* locator. So this is a separate type with its own wire encoding, its own
//! validation, and — deliberately — no conversion back. A recorded stage is a report value; there
//! is no `From<DurableLocator> for Locator`, no accessor yielding a `SourceFileId`, and nothing
//! here reaches a `ProvId`, an arena, or an authority input.
//!
//! # What it preserves
//!
//! The closed stage vocabulary, source ordinals or generated-artifact identity, exact byte spans,
//! bounded ordered origin edges, and one head. That is the whole of what a consumer needs to say
//! where a byte came from, and it is what lets exact recorded source bytes plus an authored span
//! recover a historical physical line without reparsing into an identical syntax arena.
//!
//! # The byte domain
//!
//! Spans index the ACQUIRED bytes, unnormalized (`30Rb:book-content-and-locator-projection`). LF
//! indexes physical lines and a CR in CRLF is an input byte like any other, so a newline
//! conversion is source drift rather than invisible equivalence.

use crate::limits::ReceiptLimits;
use crate::rows::SourceOrdinal;
use crate::tokens::ClosedToken;

/// The region's own version line, so a payload names its shape before anything reads it.
pub const LOCATOR_VERSION_LINE: &str = "dorc-receipt-locator/1";

/// The closed stage vocabulary, mirroring `aid::locator::Stage` by NAME.
///
/// Closed and extended by new name only, exactly as its live counterpart is: a consumer that
/// renders stages exhaustively is what makes a new stage a visible edit rather than a silent
/// omission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordedStageKind {
    /// Bytes as an author wrote them, in a source the controller read.
    Authored,
    /// The `.` act that brought the next stage's source into the unit.
    Loaded,
    /// Bytes copied verbatim into generated output.
    Copied,
    /// Scaffolding the engine wrote itself, descending from no authored bytes.
    Generated,
    /// A generated artifact's own claim about where its bytes came from. Narrative, never identity.
    Claimed,
}

/// The wire tokens, in declaration order.
pub const STAGE_KIND: &[&str] = &["authored", "loaded", "copied", "generated", "claimed"];

impl ClosedToken for RecordedStageKind {
    const TOKENS: &'static [&'static str] = STAGE_KIND;
    const ALL: &'static [Self] = &[
        Self::Authored,
        Self::Loaded,
        Self::Copied,
        Self::Generated,
        Self::Claimed,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Loaded => "loaded",
            Self::Copied => "copied",
            Self::Generated => "generated",
            Self::Claimed => "claimed",
        }
    }
}

impl RecordedStageKind {
    /// Does this kind locate itself in a source the controller READ?
    ///
    /// The discriminator that decides which identity the stage must carry, so the two are never
    /// asked independently and cannot disagree.
    const fn names_a_source(self) -> bool {
        matches!(self, Self::Authored | Self::Loaded)
    }

    /// Does this kind carry a byte span?
    ///
    /// Every kind but `Claimed`, whose whole content is the claim text — a claim names no range
    /// because it names no document anyone can point at.
    const fn carries_a_span(self) -> bool {
        !matches!(self, Self::Claimed)
    }
}

/// Which destination a stage's free bytes are bound for, so a render encodes each correctly.
///
/// Carried in the wire form rather than inferred from the stage kind at the render seat: a
/// generated artifact's own label and a bundle's claim about its origin are different sink
/// questions (`sinv-sink-encoding`), and a reader that had to re-derive which was which would be
/// one edit away from encoding a claim as a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StageTextKind {
    /// No free bytes.
    None,
    /// A generated artifact's own label.
    Artifact,
    /// A generated artifact's claim about where its bytes came from.
    Claim,
}

/// One stage, as a document carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableStage {
    kind: RecordedStageKind,
    source: Option<SourceOrdinal>,
    span: Option<(u64, u64)>,
    text_kind: StageTextKind,
    text: Vec<u8>,
    origins: Vec<u32>,
}

/// Why a locator could not be built or read back.
///
/// Closed, and every arm is a different fact about the payload. A locator that fails any of them
/// is unavailable AS A WHOLE — a partially-trusted provenance chain is worse than none, because a
/// reader cannot tell which half they are looking at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocatorRefusal {
    /// The version line was absent or unrecognized.
    Version,
    /// A structural line did not parse.
    Malformed,
    /// A stage named a kind outside the closed vocabulary.
    UnknownStage,
    /// A stage carried an identity its kind does not take, or omitted one it requires.
    StageShape,
    /// A span ran backwards or overflowed.
    Span,
    /// An origin named a stage that does not exist, or one at or after itself.
    Origin,
    /// The head named a stage that does not exist.
    Head,
    /// A declared count disagreed with what followed.
    Count,
    /// The payload, its stages, or one stage's origins exceeded the bound.
    OverLimit,
}

impl DurableStage {
    /// One authored or loaded stage: a range of a source the controller read.
    ///
    /// # Errors
    /// Refuses a backwards span.
    pub fn in_source(
        kind: RecordedStageKind,
        source: SourceOrdinal,
        span: (u64, u64),
        origins: Vec<u32>,
    ) -> Result<Self, LocatorRefusal> {
        if !kind.names_a_source() {
            return Err(LocatorRefusal::StageShape);
        }
        if span.0 > span.1 {
            return Err(LocatorRefusal::Span);
        }
        Ok(Self {
            kind,
            source: Some(source),
            span: Some(span),
            text_kind: StageTextKind::None,
            text: Vec::new(),
            origins,
        })
    }

    /// One copied or generated stage: a range of an artifact the engine wrote.
    ///
    /// # Errors
    /// Refuses a backwards span or a kind that names a read source instead.
    pub fn in_artifact(
        kind: RecordedStageKind,
        artifact: Vec<u8>,
        span: (u64, u64),
        origins: Vec<u32>,
    ) -> Result<Self, LocatorRefusal> {
        if !matches!(
            kind,
            RecordedStageKind::Copied | RecordedStageKind::Generated
        ) {
            return Err(LocatorRefusal::StageShape);
        }
        if span.0 > span.1 {
            return Err(LocatorRefusal::Span);
        }
        Ok(Self {
            kind,
            source: None,
            span: Some(span),
            text_kind: StageTextKind::Artifact,
            text: artifact,
            origins,
        })
    }

    /// One claimed stage: text a generated artifact wrote about itself, interpreted by nothing.
    #[must_use]
    pub const fn claimed(claim: Vec<u8>, origins: Vec<u32>) -> Self {
        Self {
            kind: RecordedStageKind::Claimed,
            source: None,
            span: None,
            text_kind: StageTextKind::Claim,
            text: claim,
            origins,
        }
    }

    /// Which stage this is.
    #[must_use]
    pub const fn kind(&self) -> RecordedStageKind {
        self.kind
    }

    /// The source this stage names, where it names one.
    #[must_use]
    pub const fn source(&self) -> Option<SourceOrdinal> {
        self.source
    }

    /// The byte range this stage names, where it names one.
    #[must_use]
    pub const fn span(&self) -> Option<(u64, u64)> {
        self.span
    }

    /// What the free bytes are, so a render sends them to the right encoder.
    #[must_use]
    pub const fn text_kind(&self) -> StageTextKind {
        self.text_kind
    }

    /// The free bytes, exactly as recorded.
    #[must_use]
    pub fn text(&self) -> &[u8] {
        &self.text
    }

    /// The stages this one descends from, in order.
    #[must_use]
    pub fn origins(&self) -> &[u32] {
        &self.origins
    }
}

/// A locator DAG, as a document carries it: stages in index order, and one head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableLocator {
    stages: Vec<DurableStage>,
    head: u32,
}

impl DurableLocator {
    /// Bind stages and a head, validating the whole graph before anything holds one.
    ///
    /// Validation is at the CONSTRUCTOR rather than at the render, so an invalid locator is
    /// unrepresentable rather than merely unrendered. Four things are checked and each is its own
    /// refusal: the head exists; every origin exists; every origin is EARLIER than the stage
    /// citing it (which is what makes acyclicity structural rather than a walk); and the counts
    /// sit inside the bound.
    ///
    /// # Errors
    /// Refuses a dangling or forward origin, a missing head, or a graph over the bound.
    pub fn of(
        stages: Vec<DurableStage>,
        head: u32,
        limits: &ReceiptLimits,
    ) -> Result<Self, LocatorRefusal> {
        if u64::try_from(stages.len()).unwrap_or(u64::MAX) > limits.locator_stages.get() {
            return Err(LocatorRefusal::OverLimit);
        }
        let count = u32::try_from(stages.len()).map_err(|_| LocatorRefusal::OverLimit)?;
        if head >= count {
            return Err(LocatorRefusal::Head);
        }
        for (index, stage) in stages.iter().enumerate() {
            if u64::try_from(stage.origins.len()).unwrap_or(u64::MAX) > limits.locator_origins.get()
            {
                return Err(LocatorRefusal::OverLimit);
            }
            let own = u32::try_from(index).map_err(|_| LocatorRefusal::OverLimit)?;
            for origin in &stage.origins {
                // Strictly earlier, never merely different: `aid::locator::Locator::push` can only
                // cite ids it already minted, so a forward or self edge is a payload that did not
                // come from one — and admitting it would be admitting a cycle.
                if *origin >= own {
                    return Err(LocatorRefusal::Origin);
                }
            }
            if stage.kind.names_a_source() != stage.source.is_some() {
                return Err(LocatorRefusal::StageShape);
            }
            if stage.kind.carries_a_span() != stage.span.is_some() {
                return Err(LocatorRefusal::StageShape);
            }
        }
        Ok(Self { stages, head })
    }

    /// Every stage, in index order.
    #[must_use]
    pub fn stages(&self) -> &[DurableStage] {
        &self.stages
    }

    /// The stage the chain starts from.
    #[must_use]
    pub const fn head(&self) -> u32 {
        self.head
    }

    /// The stage `index` names.
    #[must_use]
    pub fn stage(&self, index: u32) -> Option<&DurableStage> {
        self.stages.get(index as usize)
    }

    /// Every stage reachable from the head, head first and origins outward, deduplicated.
    ///
    /// The same order the live locator renders in — most-generated first, most-editable last — so
    /// a recorded chain reads the way a live one does. Total over any graph this type admits,
    /// because origins are strictly earlier and the visited set reports fan-in once.
    #[must_use]
    pub fn chain(&self) -> Vec<u32> {
        let mut seen = std::collections::BTreeSet::new();
        let mut order = Vec::new();
        let mut queue = std::collections::VecDeque::from([self.head]);
        while let Some(index) = queue.pop_front() {
            let Some(stage) = self.stage(index) else {
                continue;
            };
            if !seen.insert(index) {
                continue;
            }
            order.push(index);
            queue.extend(stage.origins.iter().copied());
        }
        order
    }

    /// The first authored stage on the chain, which is where a source address resolves.
    ///
    /// `Authored` only, never `Loaded`: a load act names the line that pulled a file in, and
    /// answering an address with it would point a reader at the `.` rather than at the command
    /// they asked about.
    #[must_use]
    pub fn authored_origin(&self) -> Option<(SourceOrdinal, (u64, u64))> {
        self.chain()
            .into_iter()
            .filter_map(|index| self.stage(index))
            .find(|stage| stage.kind == RecordedStageKind::Authored)
            .and_then(|stage| Some((stage.source?, stage.span?)))
    }

    /// The exact payload bytes for one overlay slot.
    ///
    /// ASCII structural lines with length-prefixed raw runs, so an artifact label or a claim
    /// carrying any byte — a newline, invalid UTF-8, a control sequence — round-trips exactly
    /// without escaping. Escaping is what the length prefix exists to avoid: an escape alphabet is
    /// a second grammar to get wrong, and the values here are somebody else's bytes.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(LOCATOR_VERSION_LINE.as_bytes());
        out.push(b'\n');
        out.extend_from_slice(format!("stages {}\n", self.stages.len()).as_bytes());
        out.extend_from_slice(format!("head {}\n", self.head).as_bytes());
        for stage in &self.stages {
            let origins = if stage.origins.is_empty() {
                "-".to_owned()
            } else {
                stage
                    .origins
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let (lo, hi) = stage.span.map_or_else(
                || ("-".to_owned(), "-".to_owned()),
                |(lo, hi)| (lo.to_string(), hi.to_string()),
            );
            out.extend_from_slice(
                format!(
                    "stage {} {} {lo} {hi} {} {} {origins}\n",
                    stage.kind.token(),
                    stage
                        .source
                        .map_or_else(|| "-".to_owned(), |ordinal| ordinal.get().to_string()),
                    text_kind_token(stage.text_kind),
                    stage.text.len(),
                )
                .as_bytes(),
            );
            out.extend_from_slice(&stage.text);
            out.push(b'\n');
        }
        out.extend_from_slice(b"locator-end\n");
        out
    }

    /// Read one payload back, validating it whole before releasing anything.
    ///
    /// # Errors
    /// Refuses an unrecognized version, a malformed line, an unknown stage, a shape mismatch, a
    /// backwards span, a dangling or forward origin, a missing head, a count disagreement, or a
    /// payload over the bound.
    pub fn decode(bytes: &[u8], limits: &ReceiptLimits) -> Result<Self, LocatorRefusal> {
        if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > limits.locator_bytes.get() {
            return Err(LocatorRefusal::OverLimit);
        }
        let mut rest = bytes;
        let version = take_line(&mut rest)?;
        if version != LOCATOR_VERSION_LINE.as_bytes() {
            return Err(LocatorRefusal::Version);
        }
        let declared = parse_count(take_field(&mut rest, "stages")?)?;
        let head = u32::try_from(parse_count(take_field(&mut rest, "head")?)?)
            .map_err(|_| LocatorRefusal::OverLimit)?;
        if u64::try_from(declared).unwrap_or(u64::MAX) > limits.locator_stages.get() {
            return Err(LocatorRefusal::OverLimit);
        }
        let mut stages = Vec::with_capacity(declared.min(1024));
        for _ in 0..declared {
            stages.push(decode_stage(&mut rest, limits)?);
        }
        // The closing token is what makes a truncated payload a refusal rather than a shorter
        // graph that happens to parse: a reader that stopped at `declared` would accept a
        // document whose tail was cut off mid-stage.
        if take_line(&mut rest)? != b"locator-end" || !rest.is_empty() {
            return Err(LocatorRefusal::Count);
        }
        Self::of(stages, head, limits)
    }
}

const fn text_kind_token(kind: StageTextKind) -> &'static str {
    match kind {
        StageTextKind::None => "none",
        StageTextKind::Artifact => "artifact",
        StageTextKind::Claim => "claim",
    }
}

fn text_kind_of(token: &str) -> Option<StageTextKind> {
    match token {
        "none" => Some(StageTextKind::None),
        "artifact" => Some(StageTextKind::Artifact),
        "claim" => Some(StageTextKind::Claim),
        _ => None,
    }
}

/// One LF-terminated line, consumed from the front. An unterminated tail is malformed.
fn take_line<'a>(rest: &mut &'a [u8]) -> Result<&'a [u8], LocatorRefusal> {
    let end = rest
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or(LocatorRefusal::Malformed)?;
    let line = &rest[..end];
    *rest = &rest[end + 1..];
    Ok(line)
}

/// One `<key> <value>` line, refusing any other key so a reordered payload cannot parse.
fn take_field<'a>(rest: &mut &'a [u8], key: &str) -> Result<&'a str, LocatorRefusal> {
    let line = std::str::from_utf8(take_line(rest)?).map_err(|_| LocatorRefusal::Malformed)?;
    line.strip_prefix(key)
        .and_then(|tail| tail.strip_prefix(' '))
        .ok_or(LocatorRefusal::Malformed)
}

fn parse_count(text: &str) -> Result<usize, LocatorRefusal> {
    text.parse().map_err(|_| LocatorRefusal::Malformed)
}

/// One `stage` line plus its length-prefixed free bytes.
fn decode_stage(rest: &mut &[u8], limits: &ReceiptLimits) -> Result<DurableStage, LocatorRefusal> {
    let line = std::str::from_utf8(take_line(rest)?).map_err(|_| LocatorRefusal::Malformed)?;
    let tail = line
        .strip_prefix("stage ")
        .ok_or(LocatorRefusal::Malformed)?;
    let atoms: Vec<&str> = tail.split(' ').collect();
    let [kind, source, lo, hi, text_kind, text_len, origins] = atoms[..] else {
        return Err(LocatorRefusal::Malformed);
    };
    let kind = RecordedStageKind::ALL
        .iter()
        .copied()
        .find(|candidate| candidate.token() == kind)
        .ok_or(LocatorRefusal::UnknownStage)?;
    let source = match source {
        "-" => None,
        text => Some(SourceOrdinal::of(
            text.parse().map_err(|_| LocatorRefusal::Malformed)?,
        )),
    };
    let span = match (lo, hi) {
        ("-", "-") => None,
        (lo, hi) => Some((
            lo.parse::<u64>().map_err(|_| LocatorRefusal::Malformed)?,
            hi.parse::<u64>().map_err(|_| LocatorRefusal::Malformed)?,
        )),
    };
    if span.is_some_and(|(lo, hi)| lo > hi) {
        return Err(LocatorRefusal::Span);
    }
    let text_kind = text_kind_of(text_kind).ok_or(LocatorRefusal::Malformed)?;
    let text_len = parse_count(text_len)?;
    if u64::try_from(text_len).unwrap_or(u64::MAX) > limits.locator_bytes.get() {
        return Err(LocatorRefusal::OverLimit);
    }
    if rest.len() < text_len + 1 {
        return Err(LocatorRefusal::Malformed);
    }
    let text = rest[..text_len].to_vec();
    *rest = &rest[text_len..];
    if take_line(rest)? != b"" {
        return Err(LocatorRefusal::Malformed);
    }
    let origins = match origins {
        "-" => Vec::new(),
        text => text
            .split(',')
            .map(|atom| atom.parse::<u32>().map_err(|_| LocatorRefusal::Malformed))
            .collect::<Result<Vec<_>, _>>()?,
    };
    // Shape is checked HERE as well as in `of`, because a stage that carried the wrong identity
    // would otherwise be built and then rejected — and the refusal a reader gets should name the
    // stage's own defect rather than the graph's.
    if kind.names_a_source() != source.is_some() || kind.carries_a_span() != span.is_some() {
        return Err(LocatorRefusal::StageShape);
    }
    Ok(DurableStage {
        kind,
        source,
        span,
        text_kind,
        text,
        origins,
    })
}
