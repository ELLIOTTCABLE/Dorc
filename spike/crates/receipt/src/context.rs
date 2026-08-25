//! What the `apply-context` slot of an apply-assignment row holds: the five axes a standup
//! resolved besides the destination.
//!
//! Named after the SLOT rather than after the live six-answer value, because the destination
//! is the assignment's own `target-name` slot — the name an operator recognizes, kept where a
//! reader looking for a host will look. Neither slot repeats the other, so each answer has one
//! place it lives. Both hang off the SAME record, so a reader recombining them cannot pair one
//! assignment's destination with another's axes: the record position IS the assignment.
//!
//! Length-framed rather than line-framed. A resolved answer is an arbitrary string from an edge
//! this crate does not own, and a newline inside one would make a line-framed block ambiguous
//! about where an axis ends — the shape where two axes transpose while the block still parses.

use crate::limits::ReceiptLimits;

/// The block's own version line.
const VERSION_LINE: &str = "dorc-apply-context/1";

/// The token closing the block.
const END_LINE: &str = "context-end";

/// The axis keys, in the one order this block is written and read.
///
/// The key words are what defend against a permutation: a reader accepting them in any order
/// would accept a block whose same-typed axes had been swapped.
const KEYS: [&str; 5] = [
    "account",
    "namespace",
    "working-directory",
    "environment-policy",
    "credential-scope",
];

/// Why an apply-context block did not decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextFault {
    /// The block is longer than one opaque field may be.
    OverBound,
    /// A required line was absent, out of order, or misspelled.
    Structure {
        /// Which line.
        what: &'static str,
    },
    /// A declared length was not a canonical integer, or ran past the block.
    Length {
        /// Which axis.
        what: &'static str,
    },
    /// An axis payload was not followed by its framing newline.
    Framing {
        /// Which axis.
        what: &'static str,
    },
    /// Bytes after the terminator.
    Trailing,
}

/// One assignment's resolved context, as a document carries it.
///
/// Named fields rather than an array, because the axes are same-typed and a positional
/// container is the shape a projection transposes without anything noticing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyContext {
    account: Vec<u8>,
    namespace: Vec<u8>,
    working_directory: Vec<u8>,
    environment_policy: Vec<u8>,
    credential_scope: Vec<u8>,
}

impl RecordedApplyContext {
    /// Take one standup's five non-destination answers.
    #[must_use]
    pub const fn of(
        account: Vec<u8>,
        namespace: Vec<u8>,
        working_directory: Vec<u8>,
        environment_policy: Vec<u8>,
        credential_scope: Vec<u8>,
    ) -> Self {
        Self {
            account,
            namespace,
            working_directory,
            environment_policy,
            credential_scope,
        }
    }

    /// The principal the session authenticated as.
    #[must_use]
    pub fn account(&self) -> &[u8] {
        &self.account
    }

    /// The namespace the session entered.
    #[must_use]
    pub fn namespace(&self) -> &[u8] {
        &self.namespace
    }

    /// Where the session stands.
    #[must_use]
    pub fn working_directory(&self) -> &[u8] {
        &self.working_directory
    }

    /// Which environment the session carries.
    #[must_use]
    pub fn environment_policy(&self) -> &[u8] {
        &self.environment_policy
    }

    /// What the session's credentials reach.
    #[must_use]
    pub fn credential_scope(&self) -> &[u8] {
        &self.credential_scope
    }

    /// The axis values, in the block's own order.
    fn axes(&self) -> [&[u8]; 5] {
        [
            &self.account,
            &self.namespace,
            &self.working_directory,
            &self.environment_policy,
            &self.credential_scope,
        ]
    }

    /// The exact bytes this context occupies in a region.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        line(&mut out, VERSION_LINE);
        for (key, value) in KEYS.into_iter().zip(self.axes()) {
            line(&mut out, &format!("{key} {}", value.len()));
            out.extend_from_slice(value);
            out.push(b'\n');
        }
        line(&mut out, END_LINE);
        out
    }

    /// Read one block back, refusing every departure from the exact form.
    ///
    /// # Errors
    /// Refuses a block past the opaque-field bound, a missing or misspelled line, an axis out
    /// of order, a length that is not canonical or runs past the block, a payload whose
    /// framing newline is absent, and trailing bytes.
    pub fn decode(bytes: &[u8], limits: &ReceiptLimits) -> Result<Self, ContextFault> {
        let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if !limits.opaque_field_bytes.admits(measured) {
            return Err(ContextFault::OverBound);
        }
        let mut cursor = Cursor::of(bytes);
        if cursor.line() != Some(VERSION_LINE) {
            return Err(ContextFault::Structure { what: "version" });
        }
        let mut axes: Vec<Vec<u8>> = Vec::with_capacity(KEYS.len());
        for key in KEYS {
            let declared = cursor
                .line()
                .and_then(|text| text.strip_prefix(key))
                .and_then(|rest| rest.strip_prefix(' '))
                .ok_or(ContextFault::Structure { what: key })?;
            let length = crate::grammar::canonical_u64(declared)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or(ContextFault::Length { what: key })?;
            let payload = cursor
                .exact(length)
                .ok_or(ContextFault::Length { what: key })?;
            axes.push(payload.to_vec());
            cursor
                .newline()
                .ok_or(ContextFault::Framing { what: key })?;
        }
        if cursor.line() != Some(END_LINE) {
            return Err(ContextFault::Structure { what: "terminator" });
        }
        if !cursor.done() {
            return Err(ContextFault::Trailing);
        }
        let mut taken = axes.into_iter();
        let mut next = || taken.next().unwrap_or_default();
        Ok(Self::of(next(), next(), next(), next(), next()))
    }
}

fn line(out: &mut Vec<u8>, text: &str) {
    out.extend_from_slice(text.as_bytes());
    out.push(b'\n');
}

/// A byte cursor. Header lines are read as text; payloads are consumed by declared length and
/// never scanned, so a payload spelling the terminator's own bytes is still a payload.
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

#[cfg(test)]
mod tests {
    use super::{ContextFault, KEYS, RecordedApplyContext};
    use crate::limits::{ByteLimit, ReceiptLimits};

    /// Five DISTINCT values, which is what lets the round trip fail: with one value in every
    /// axis, a transposed writer and a transposed reader would agree.
    fn distinct() -> RecordedApplyContext {
        RecordedApplyContext::of(
            b"deploy".to_vec(),
            b"netns-blue".to_vec(),
            b"/srv/app".to_vec(),
            b"inherited-minus-ssh".to_vec(),
            b"agent-forwarded".to_vec(),
        )
    }

    #[test]
    fn a_context_round_trips_with_every_axis_holding_its_own_value() {
        let context = distinct();
        let decoded = RecordedApplyContext::decode(&context.encode(), &ReceiptLimits::V1)
            .expect("its own encoding decodes");
        assert_eq!(decoded, context);
        assert_eq!(decoded.account(), b"deploy");
        assert_eq!(decoded.credential_scope(), b"agent-forwarded");
    }

    #[test]
    fn an_axis_carrying_a_newline_survives_because_payloads_are_length_framed() {
        // The reason this block is not line-framed. A resolved answer comes from an edge this
        // crate does not own, and a line-framed reader would end the axis mid-value and read
        // the remainder as the next axis's key.
        let context = RecordedApplyContext::of(
            b"deploy\nroot".to_vec(),
            b"context-end\n".to_vec(),
            b"/srv".to_vec(),
            b"none".to_vec(),
            b"none".to_vec(),
        );
        let decoded = RecordedApplyContext::decode(&context.encode(), &ReceiptLimits::V1)
            .expect("a payload is bytes, not lines");
        assert_eq!(decoded, context);
    }

    #[test]
    fn an_empty_axis_is_legal_and_an_absent_one_is_not() {
        let context = RecordedApplyContext::of(
            Vec::new(),
            b"x".to_vec(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        assert_eq!(
            RecordedApplyContext::decode(&context.encode(), &ReceiptLimits::V1),
            Ok(context)
        );
        let without = "dorc-apply-context/1\ncontext-end\n";
        assert_eq!(
            RecordedApplyContext::decode(without.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::Structure { what: KEYS[0] })
        );
    }

    #[test]
    fn a_permuted_block_refuses_at_the_axis_whose_key_it_expected() {
        // The failure this exists for: two same-typed axes swapped. Pinned to the EXACT key
        // the reader wanted, because "it was rejected" is satisfied by a truncation too.
        let text = String::from_utf8(distinct().encode()).expect("the fixture is text");
        let swapped = text.replacen("account 6\n", "namespace 6\n", 1);
        assert_eq!(
            RecordedApplyContext::decode(swapped.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::Structure { what: "account" })
        );
    }

    #[test]
    fn a_declared_length_past_the_block_refuses_as_a_length_and_not_as_a_terminator() {
        let text = String::from_utf8(distinct().encode()).expect("the fixture is text");
        let overlong = text.replacen("account 6\n", "account 600\n", 1);
        assert_eq!(
            RecordedApplyContext::decode(overlong.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::Length { what: "account" })
        );
    }

    #[test]
    fn a_missing_framing_newline_refuses_separately_from_a_wrong_length() {
        // Under-declaring by one leaves the payload's own last byte where the framing newline
        // belongs, so the two failures are adjacent and must stay distinguishable.
        let text = String::from_utf8(distinct().encode()).expect("the fixture is text");
        let short = text.replacen("account 6\n", "account 5\n", 1);
        assert_eq!(
            RecordedApplyContext::decode(short.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::Framing { what: "account" })
        );
    }

    #[test]
    fn bytes_after_the_terminator_refuse_rather_than_being_ignored() {
        let mut bytes = distinct().encode();
        bytes.extend_from_slice(b"more\n");
        assert_eq!(
            RecordedApplyContext::decode(&bytes, &ReceiptLimits::V1),
            Err(ContextFault::Trailing)
        );
    }

    #[test]
    fn a_block_past_the_opaque_field_bound_refuses_before_it_is_read() {
        let narrow = ReceiptLimits {
            opaque_field_bytes: ByteLimit::of(4),
            ..ReceiptLimits::V1
        };
        assert_eq!(
            RecordedApplyContext::decode(&distinct().encode(), &narrow),
            Err(ContextFault::OverBound)
        );
    }
}
