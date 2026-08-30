//! The one way bytes a document carried leave [`RecordedWhyFacts`](super::RecordedWhyFacts).
//!
//! # Why there is no accessor
//!
//! Everything in this family is a managed host's bytes, an author's own shell text, or a path off
//! somebody's filesystem. `sinv-sink-encoding` says every such value passes through the centralized
//! encoder for its DESTINATION before it is shown, and the way to make that true by construction
//! rather than by review is to leave no other exit: [`RecordedValue`] has no `as_bytes`, no
//! `as_str`, no `into_inner`, and its `Debug` says how much it is holding rather than what.
//!
//! It carries its own [`ValueClass`] because the encoder needs to know which sink question it is
//! answering — a path, a line of shell, and a bundle's claim about its own origin are three
//! different encodings — and a caller that had to remember which was which would be one edit away
//! from encoding a claim as a path.
//!
//! # Why the encoder is a trait this crate does not implement
//!
//! `aid` owns the destination encoders and this crate must not depend on it. So the direction is
//! inverted: the receipt crate states the OBLIGATION as a trait, and the CLI adapter satisfies it
//! with the real `aid` encoders. Nothing here can render, which is the point.

use core::fmt;

/// What KIND of value a byte run is, so an encoder can answer the right sink question.
///
/// Closed, and it grows by new name only: an encoder matching exhaustively is what makes a new
/// class a visible edit rather than a value quietly taking some other class's encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueClass {
    /// A line or span of an author's own shell.
    ShellText,
    /// A whole acquired source, or a span of one.
    SourceText,
    /// A path off the controller's filesystem, as the run recorded it.
    SourcePath,
    /// A generated artifact's own label.
    ArtifactLabel,
    /// What a generated artifact CLAIMED about where its bytes came from. Narrative, never
    /// identity — the claim is somebody else's text and is interpreted by nothing.
    OriginClaim,
}

/// Bytes a document carried, with no exit but an encoder.
///
/// Deliberately implements none of `Display`, `PartialEq`, `Eq`, `Ord`, `Hash`, or serde: each
/// would be a side channel that reads the bytes without an encoder ever being consulted. Equality
/// is the subtle one — comparing two values against a caller-supplied probe leaks them a byte at a
/// time — so the comparison this model needs is done INSIDE the crate, on the private field, and
/// answers a typed verdict rather than a bool a caller can drive.
#[derive(Clone)]
pub struct RecordedValue {
    class: ValueClass,
    bytes: Vec<u8>,
}

impl RecordedValue {
    /// Seal one byte run under its class.
    #[must_use]
    pub(crate) fn sealed(class: ValueClass, bytes: Vec<u8>) -> Self {
        Self { class, bytes }
    }

    /// Which sink question this value poses.
    #[must_use]
    pub const fn class(&self) -> ValueClass {
        self.class
    }

    /// How many bytes are held. A length is not the content, and a report that cannot say how big
    /// a thing is cannot explain why a bound refused it.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the run recorded nothing here.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The value, as `encoder` renders it for its destination. THE ONLY EXIT.
    #[must_use]
    pub fn render(&self, encoder: &mut dyn ValueEncoder) -> String {
        encoder.encode(self.class, &self.bytes)
    }
}

/// Says its class and its size, and nothing about its content.
///
/// Hand-written rather than derived because a derived `Debug` would print the bytes — into a panic
/// message, a log line, or a test failure, all of which are places `sinv-sink-encoding` says host
/// bytes may not arrive unencoded.
impl fmt::Debug for RecordedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RecordedValue({:?}, {} bytes)",
            self.class,
            self.bytes.len()
        )
    }
}

/// The obligation a caller satisfies to see any recorded value.
///
/// Implemented OUTSIDE this crate, by the seat that owns destination encoders. The `&mut` receiver
/// is deliberate: a real encoder accumulates (a width budget, a truncation count, a diagnostic
/// frame), and a `&self` signature would push that state somewhere a caller has to thread by hand.
pub trait ValueEncoder {
    /// Render `bytes` for the destination this encoder speaks, treating them as `class`.
    fn encode(&mut self, class: ValueClass, bytes: &[u8]) -> String;
}

/// How one recorded byte run stands against a current one.
///
/// The comparison happens INSIDE the crate, on private fields, and answers this rather than
/// handing out a bool a caller could drive against probe bytes. Byte-exact and nothing else: no
/// normalization, no trimming, no newline folding, because `30R` rules every one of those a source
/// CHANGE rather than an equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteAgreement {
    /// The two runs are the same bytes.
    Identical,
    /// They are not. Which bytes differ is not said, because saying it is the leak.
    Differing,
}

impl RecordedValue {
    /// Whether this value's bytes are exactly `other`.
    pub(crate) fn agrees_with(&self, other: &[u8]) -> ByteAgreement {
        if self.bytes == other {
            ByteAgreement::Identical
        } else {
            ByteAgreement::Differing
        }
    }

    /// The physical line `line` (1-indexed) of this value, as its own sealed value.
    ///
    /// LF indexes lines and a CR in CRLF stays in the line's bytes, because the acquired byte
    /// domain is what every span in a durable locator is stated against
    /// (`30Rb:book-content-and-locator-projection`). A final line with no terminator counts.
    pub(crate) fn physical_line(&self, line: u32) -> Option<Self> {
        let wanted = usize::try_from(line).ok()?.checked_sub(1)?;
        let mut start = 0_usize;
        for (index, run) in self
            .bytes
            .split_inclusive(|byte| *byte == b'\n')
            .enumerate()
        {
            if index == wanted {
                let end = start.checked_add(run.len())?;
                let text = self.bytes.get(start..end)?.to_vec();
                return Some(Self::sealed(self.class, text));
            }
            start = start.checked_add(run.len())?;
        }
        None
    }

    /// The byte range physical line `line` (1-indexed) occupies, terminator included.
    pub(crate) fn line_span(&self, line: u32) -> Option<(u64, u64)> {
        let wanted = usize::try_from(line).ok()?.checked_sub(1)?;
        let mut start = 0_usize;
        for (index, run) in self
            .bytes
            .split_inclusive(|byte| *byte == b'\n')
            .enumerate()
        {
            let end = start.checked_add(run.len())?;
            if index == wanted {
                return Some((u64::try_from(start).ok()?, u64::try_from(end).ok()?));
            }
            start = end;
        }
        None
    }
}
