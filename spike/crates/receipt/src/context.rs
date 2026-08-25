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

/// The word introducing an axis a standup entered, ahead of its declared length.
const ESTABLISHED: &str = "established";

/// The word standing alone where a standup entered nothing.
const NOT_ESTABLISHED: &str = "not-established";

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
    /// An axis line carried neither of the two state words.
    ///
    /// Separate from [`Self::Structure`] because the repair differs: the key was found and the
    /// claim beside it was not one this block can make.
    State {
        /// Which axis.
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

/// What a document records for one axis.
///
/// [`crate::dispatch::ResolvedAxis`]'s recorded twin, and it keeps the same two arms for the same
/// reason: a zero-length value is an axis that WAS entered, so absence needs a word of its own or
/// a document would read "no context was established" as "established as nothing".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedAxis {
    /// The standup entered this axis; these are the exact bytes it resolved to.
    Established(Vec<u8>),
    /// Nothing was entered on this axis.
    NotEstablished,
}

/// One assignment's resolved context, as a document carries it.
///
/// Named fields rather than an array, because the axes are same-typed and a positional
/// container is the shape a projection transposes without anything noticing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyContext {
    account: RecordedAxis,
    namespace: RecordedAxis,
    working_directory: RecordedAxis,
    environment_policy: RecordedAxis,
    credential_scope: RecordedAxis,
}

impl RecordedApplyContext {
    /// Take one standup's five non-destination answers.
    #[must_use]
    pub const fn of(
        account: RecordedAxis,
        namespace: RecordedAxis,
        working_directory: RecordedAxis,
        environment_policy: RecordedAxis,
        credential_scope: RecordedAxis,
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
    pub const fn account(&self) -> &RecordedAxis {
        &self.account
    }

    /// The namespace the session entered.
    #[must_use]
    pub const fn namespace(&self) -> &RecordedAxis {
        &self.namespace
    }

    /// Where the session stands.
    #[must_use]
    pub const fn working_directory(&self) -> &RecordedAxis {
        &self.working_directory
    }

    /// Which environment the session carries.
    #[must_use]
    pub const fn environment_policy(&self) -> &RecordedAxis {
        &self.environment_policy
    }

    /// What the session's credentials reach.
    #[must_use]
    pub const fn credential_scope(&self) -> &RecordedAxis {
        &self.credential_scope
    }

    /// The axis values, in the block's own order.
    fn axes(&self) -> [&RecordedAxis; 5] {
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
        for (key, axis) in KEYS.into_iter().zip(self.axes()) {
            match axis {
                RecordedAxis::Established(value) => {
                    line(&mut out, &format!("{key} {ESTABLISHED} {}", value.len()));
                    out.extend_from_slice(value);
                    out.push(b'\n');
                }
                RecordedAxis::NotEstablished => {
                    line(&mut out, &format!("{key} {NOT_ESTABLISHED}"));
                }
            }
        }
        line(&mut out, END_LINE);
        out
    }

    /// Read one block back, refusing every departure from the exact form.
    ///
    /// # Errors
    /// Refuses a block past the opaque-field bound, a missing or misspelled line, an axis out
    /// of order, an axis claiming neither state, a length that is not canonical or runs past the
    /// block, a payload whose framing newline is absent, and trailing bytes.
    pub fn decode(bytes: &[u8], limits: &ReceiptLimits) -> Result<Self, ContextFault> {
        let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if !limits.opaque_field_bytes.admits(measured) {
            return Err(ContextFault::OverBound);
        }
        let mut cursor = Cursor::of(bytes);
        if cursor.line() != Some(VERSION_LINE) {
            return Err(ContextFault::Structure { what: "version" });
        }
        let mut axes: Vec<RecordedAxis> = Vec::with_capacity(KEYS.len());
        for key in KEYS {
            let claim = cursor
                .line()
                .and_then(|text| text.strip_prefix(key))
                .and_then(|rest| rest.strip_prefix(' '))
                .ok_or(ContextFault::Structure { what: key })?;
            if claim == NOT_ESTABLISHED {
                axes.push(RecordedAxis::NotEstablished);
                continue;
            }
            let declared = claim
                .strip_prefix(ESTABLISHED)
                .and_then(|rest| rest.strip_prefix(' '))
                .ok_or(ContextFault::State { what: key })?;
            let length = crate::grammar::canonical_u64(declared)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or(ContextFault::Length { what: key })?;
            let payload = cursor
                .exact(length)
                .ok_or(ContextFault::Length { what: key })?;
            axes.push(RecordedAxis::Established(payload.to_vec()));
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
        let mut next = || taken.next().unwrap_or(RecordedAxis::NotEstablished);
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
    use super::{ContextFault, KEYS, RecordedApplyContext, RecordedAxis};
    use crate::limits::{ByteLimit, ReceiptLimits};

    fn entered(text: &str) -> RecordedAxis {
        RecordedAxis::Established(text.as_bytes().to_vec())
    }

    /// Five DISTINCT values, which is what lets the round trip fail: with one value in every
    /// axis, a transposed writer and a transposed reader would agree.
    fn distinct() -> RecordedApplyContext {
        RecordedApplyContext::of(
            entered("deploy"),
            entered("netns-blue"),
            entered("/srv/app"),
            entered("inherited-minus-ssh"),
            entered("agent-forwarded"),
        )
    }

    /// What a session that entered no context records.
    fn nothing_entered() -> RecordedApplyContext {
        RecordedApplyContext::of(
            RecordedAxis::NotEstablished,
            RecordedAxis::NotEstablished,
            RecordedAxis::NotEstablished,
            RecordedAxis::NotEstablished,
            RecordedAxis::NotEstablished,
        )
    }

    #[test]
    fn a_context_round_trips_with_every_axis_holding_its_own_value() {
        let context = distinct();
        let decoded = RecordedApplyContext::decode(&context.encode(), &ReceiptLimits::V1)
            .expect("its own encoding decodes");
        assert_eq!(decoded, context);
        assert_eq!(decoded.account(), &entered("deploy"));
        assert_eq!(decoded.credential_scope(), &entered("agent-forwarded"));
    }

    #[test]
    fn an_axis_carrying_a_newline_survives_because_payloads_are_length_framed() {
        // The reason this block is not line-framed. A resolved answer comes from an edge this
        // crate does not own, and a line-framed reader would end the axis mid-value and read
        // the remainder as the next axis's key.
        let context = RecordedApplyContext::of(
            entered("deploy\nroot"),
            entered("context-end\n"),
            entered("/srv"),
            entered("none"),
            entered("none"),
        );
        let decoded = RecordedApplyContext::decode(&context.encode(), &ReceiptLimits::V1)
            .expect("a payload is bytes, not lines");
        assert_eq!(decoded, context);
    }

    /// THE FALSIFIER for the two-arm axis: a session that entered nothing must not encode as one
    /// that entered five empty strings.
    ///
    /// The two are the same length, the same shape, and differ only in the word each axis line
    /// carries — which is exactly why the word exists. A block that spelled absence as a
    /// zero-length payload would read back as an established answer of nothing, and the
    /// difference between "we did not look" and "we looked and it is empty" is the whole content
    /// of a thin session's claim.
    #[test]
    fn an_entered_empty_axis_and_an_unentered_one_are_different_documents() {
        let empty = RecordedApplyContext::of(
            entered(""),
            entered(""),
            entered(""),
            entered(""),
            entered(""),
        );
        assert_ne!(empty.encode(), nothing_entered().encode());
        for context in [empty, nothing_entered()] {
            assert_eq!(
                RecordedApplyContext::decode(&context.encode(), &ReceiptLimits::V1),
                Ok(context),
                "each round trips to itself and never to its neighbour"
            );
        }
    }

    #[test]
    fn an_absent_axis_line_refuses_where_an_unentered_one_is_recorded() {
        let without = "dorc-apply-context/1\ncontext-end\n";
        assert_eq!(
            RecordedApplyContext::decode(without.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::Structure { what: KEYS[0] }),
            "an axis nobody wrote is a broken block, not an unentered axis"
        );
    }

    #[test]
    fn an_axis_claiming_neither_state_refuses_as_a_state_and_not_as_a_length() {
        // Adjacent to the length failure and repaired differently: the key was found, and the
        // claim beside it was not one this block can make.
        let text = String::from_utf8(distinct().encode()).expect("the fixture is text");
        let unclaimed = text.replacen("account established 6\n", "account maybe 6\n", 1);
        assert_eq!(
            RecordedApplyContext::decode(unclaimed.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::State { what: "account" })
        );
    }

    #[test]
    fn a_permuted_block_refuses_at_the_axis_whose_key_it_expected() {
        // The failure this exists for: two same-typed axes swapped. Pinned to the EXACT key
        // the reader wanted, because "it was rejected" is satisfied by a truncation too.
        let text = String::from_utf8(distinct().encode()).expect("the fixture is text");
        let swapped = text.replacen("account established 6\n", "namespace established 6\n", 1);
        assert_eq!(
            RecordedApplyContext::decode(swapped.as_bytes(), &ReceiptLimits::V1),
            Err(ContextFault::Structure { what: "account" })
        );
    }

    #[test]
    fn a_declared_length_past_the_block_refuses_as_a_length_and_not_as_a_terminator() {
        let text = String::from_utf8(distinct().encode()).expect("the fixture is text");
        let overlong = text.replacen("account established 6\n", "account established 600\n", 1);
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
        let short = text.replacen("account established 6\n", "account established 5\n", 1);
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
