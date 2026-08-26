//! The store-selection ordering token, and the seam a controller clock reaches it through.
//!
//! [`ReceiptOrderToken`] is a hint about WHEN a document was written, spelled inside the signed
//! body so a local filename claiming an order can be checked against one. It is not an identity,
//! not a freshness claim, not a graph edge, and there is no conversion between it and a receipt
//! identity in either direction — which is why it lives here and not in [`crate::ids`].
//!
//! Its whole consumer is local store selection: greatest token wins, equal tokens are an
//! ambiguity rather than a tie-break. A clock that moves backwards therefore changes which
//! document a local selection offers first and changes nothing else about what any document says.

/// The exact spelled width. Fixed rather than minimal, so a lexical comparison over the digits
/// is the numeric comparison over the value, and a filename and a header line can be compared
/// as bytes.
pub const ORDER_DIGITS: usize = 20;

/// One document's store-selection order: exactly [`ORDER_DIGITS`] decimal digits.
///
/// Private fixed-width digits, and the derived `Ord` is over those digits — which is the numeric
/// order precisely because the width is fixed and leading zeroes are required. A variable-width
/// spelling would sort `9` above `10`, so the padding is load-bearing rather than cosmetic.
///
/// Deliberately absent: any conversion to or from [`crate::ids::ReceiptId`] or its species
/// newtypes, any arithmetic, and any comparison against a live clock reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptOrderToken([u8; ORDER_DIGITS]);

impl ReceiptOrderToken {
    /// The lowest token there is: every real reading sorts at or above it.
    ///
    /// A FIRST-CLASS value, not a degenerate one. An undated receipt is what a stable-format
    /// artifact wants — a run asking "did this change" cannot have a clock reading in the bytes it
    /// diffs — so the library, the store, and the reader all carry it natively.
    ///
    /// What must not happen is an undated document reaching a store that selects by order: it
    /// would sort oldest, and a user who just ran something and asked why would be shown the
    /// PREVIOUS run own answer with nothing saying so. That is refused where a document is
    /// EMITTED, at the production composition root, rather than by making this unrepresentable.
    pub const UNDATED: Self = Self([b'0'; ORDER_DIGITS]);

    /// Mint from a controller-observed instant, in milliseconds.
    ///
    /// Total: every `u64` fits the fixed width, `u64::MAX` included.
    #[must_use]
    pub fn of_controller_millis(millis: u64) -> Self {
        let mut digits = [b'0'; ORDER_DIGITS];
        let mut value = millis;
        for slot in digits.iter_mut().rev() {
            *slot = b'0'.wrapping_add(u8::try_from(value % 10).unwrap_or(0));
            value /= 10;
        }
        Self(digits)
    }

    /// Read a token back from its exact spelling.
    ///
    /// Exactly [`ORDER_DIGITS`] ASCII digits and nothing else: no sign, no shorter spelling of the
    /// same value, no wider one. A value above `u64::MAX` is still admitted, because the grammar
    /// fixes the WIDTH rather than a numeric range, and narrowing a reviewed wire field to the
    /// range of the type that happens to mint it would be this reader inventing a rule.
    #[must_use]
    pub fn of_spelling(text: &str) -> Option<Self> {
        let bytes: [u8; ORDER_DIGITS] = text.as_bytes().try_into().ok()?;
        if !bytes.iter().all(u8::is_ascii_digit) {
            return None;
        }
        Some(Self(bytes))
    }

    /// The one spelling, exactly [`ORDER_DIGITS`] digits wide.
    #[must_use]
    pub fn spelled(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

/// Where a document's order comes from. Injected, so this crate reads no clock and a
/// deterministic clock drives every test.
pub trait ControllerClock {
    /// The order to stamp on the next document.
    fn order_token(&mut self) -> ReceiptOrderToken;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_is_always_exactly_the_fixed_width() {
        for millis in [0, 1, 9, 10, 1_700_000_000_000, u64::MAX] {
            let spelled = ReceiptOrderToken::of_controller_millis(millis).spelled();
            assert_eq!(spelled.len(), ORDER_DIGITS, "{millis}");
            assert!(spelled.bytes().all(|b| b.is_ascii_digit()), "{millis}");
        }
        assert_eq!(
            ReceiptOrderToken::of_controller_millis(u64::MAX).spelled(),
            "18446744073709551615",
            "the widest value a controller clock can present still fits the field"
        );
    }

    #[test]
    fn the_fixed_width_is_what_makes_the_byte_order_the_numeric_order() {
        // The reason leading zeroes are required rather than tolerated. Without the padding
        // these two sort the other way round as text, and a store selecting the greatest
        // filename would answer with the older document.
        let earlier = ReceiptOrderToken::of_controller_millis(9);
        let later = ReceiptOrderToken::of_controller_millis(10);
        assert!(earlier < later);
        assert!(earlier.spelled() < later.spelled());
        assert!(ReceiptOrderToken::UNDATED < earlier);
    }

    #[test]
    fn a_spelling_round_trips_and_a_departure_from_the_exact_width_does_not() {
        let token = ReceiptOrderToken::of_controller_millis(1_700_000_000_000);
        assert_eq!(
            ReceiptOrderToken::of_spelling(&token.spelled()),
            Some(token)
        );
        // Each of these is a vector in the committed corpus; pinned here too so the type's own
        // refusal cannot drift away from the grammar's.
        assert_eq!(
            ReceiptOrderToken::of_spelling("1700000000000"),
            None,
            "short"
        );
        assert_eq!(
            ReceiptOrderToken::of_spelling("000000000000000000000"),
            None,
            "wide"
        );
        assert_eq!(
            ReceiptOrderToken::of_spelling("-0000000000000000001"),
            None,
            "signed"
        );
        assert_eq!(
            ReceiptOrderToken::of_spelling("0000000000000000000a"),
            None,
            "not a digit"
        );
        assert_eq!(ReceiptOrderToken::of_spelling(""), None, "empty");
    }

    #[test]
    fn a_value_past_the_minting_type_is_still_a_well_spelled_token() {
        // The grammar fixes the width, not a range. Refusing this would be the reader narrowing
        // a reviewed field to the type that happens to mint it — and the refusal would then be
        // indistinguishable from a malformed one.
        assert!(ReceiptOrderToken::of_spelling("99999999999999999999").is_some());
        assert!(
            ReceiptOrderToken::of_spelling("99999999999999999999")
                > ReceiptOrderToken::of_spelling("18446744073709551615")
        );
    }
}
