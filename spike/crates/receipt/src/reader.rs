//! The reader's monotone states.
//!
//! Bytes are bounded, then located, then checked, then parsed — in that order, with no path
//! that reaches a later state without passing every earlier one. A partial result never
//! converts to a complete one, in either direction.

use core::marker::PhantomData;

use crate::capability::VerificationKeyResolver;
use crate::format::{self, LocatedReceiptEnvelope, RefusalReason, Skeleton};
use crate::ids::{SigningKeyId, from_hex_32};
use crate::limits::ReceiptLimits;
use crate::model::{
    Projection, SelfAssertedReceiptSigner, SignerTrust, Species, TrustedReceiptSigner, payload_type,
};

/// Input that has passed the aggregate bound, before anything is parsed or allocated from
/// a value the document declared.
#[derive(Debug)]
pub struct BoundedReceiptBytes {
    bytes: Vec<u8>,
}

impl BoundedReceiptBytes {
    /// Accept input within the aggregate bound.
    ///
    /// # Errors
    /// Refuses input past the aggregate bound.
    pub fn of(bytes: Vec<u8>, limits: &ReceiptLimits) -> Result<Self, RefusalReason> {
        let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if !limits.outer_bytes.admits(measured) {
            return Err(RefusalReason::OverBound {
                what: "outer-bytes",
            });
        }
        Ok(Self { bytes })
    }

    /// Locate the exact spans, interpreting no field.
    ///
    /// # Errors
    /// Refuses whatever the locator refuses.
    pub fn locate(&self, limits: &ReceiptLimits) -> Result<LocatedReceiptEnvelope, RefusalReason> {
        format::locate(&self.bytes, limits)
    }
}

/// A document whose signature checked, under material whose provenance is `T`.
///
/// Still unparsed, and not a statement that anything it says is so.
#[derive(Debug)]
pub struct ReceiptSignatureChecked<T: SignerTrust> {
    body: Vec<u8>,
    skeleton: Vec<u8>,
    armor: Option<String>,
    trust: PhantomData<T>,
}

impl<T: SignerTrust> ReceiptSignatureChecked<T> {
    /// The exact span that was checked, which is the span parsing consumes.
    #[must_use]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// The literal skeleton span.
    #[must_use]
    pub fn skeleton(&self) -> &[u8] {
        &self.skeleton
    }

    /// The armored region, when the document carries one.
    #[must_use]
    pub fn armor(&self) -> Option<&str> {
        self.armor.as_deref()
    }
}

/// Check a located document against material the resolver supplies.
///
/// The provenance marker comes from which resolver answer was taken; a caller has no type
/// parameter with which to ask for one. Both concrete answers are tried in a fixed order,
/// and the trusted answer is preferred where policy names the provider.
///
/// # Errors
/// Refuses a misshaped signature, a failed check, or material the resolver does not hold.
pub fn check_signature<D: Species, P: Projection>(
    located: &LocatedReceiptEnvelope,
    resolver: &dyn VerificationKeyResolver,
) -> Result<Checked, RefusalReason> {
    let id = SigningKeyId::of_hex(&located.signing_key_id).ok_or(RefusalReason::SignatureShape)?;
    let signature = signature_bytes(&located.signature_hex)?;
    let input = crate::ids::pae(&payload_type::<D, P>(), &located.body);

    if let Some(key) = resolver.trusted(id) {
        return if key.verify(&input, &signature) {
            Ok(Checked::Trusted(ReceiptSignatureChecked {
                body: located.body.clone(),
                skeleton: located.skeleton.clone(),
                armor: located.armor.clone(),
                trust: PhantomData,
            }))
        } else {
            Err(RefusalReason::SignatureCheck)
        };
    }
    if let Some(key) = resolver.self_asserted(id) {
        return if key.verify(&input, &signature) {
            Ok(Checked::SelfAsserted(ReceiptSignatureChecked {
                body: located.body.clone(),
                skeleton: located.skeleton.clone(),
                armor: located.armor.clone(),
                trust: PhantomData,
            }))
        } else {
            Err(RefusalReason::SignatureCheck)
        };
    }
    Err(RefusalReason::KeyUnavailable)
}

/// Which provenance a check landed on. The two arms are separate types, never a flag.
#[derive(Debug)]
pub enum Checked {
    /// Checked under material controller policy names.
    Trusted(ReceiptSignatureChecked<TrustedReceiptSigner>),
    /// Checked under material controller policy does not name.
    SelfAsserted(ReceiptSignatureChecked<SelfAssertedReceiptSigner>),
}

fn signature_bytes(hex: &str) -> Result<[u8; 64], RefusalReason> {
    if hex.len() != 128 {
        return Err(RefusalReason::SignatureShape);
    }
    let (head, tail) = hex.split_at(64);
    let head = from_hex_32(head).ok_or(RefusalReason::SignatureShape)?;
    let tail = from_hex_32(tail).ok_or(RefusalReason::SignatureShape)?;
    let mut out = [0_u8; 64];
    for (slot, byte) in out.iter_mut().zip(head.iter().chain(tail.iter())) {
        *slot = *byte;
    }
    Ok(out)
}

/// A checked document parsed under its species and projection grammar.
#[derive(Debug)]
pub struct ParsedReceiptSkeleton<D: Species, P: Projection, T: SignerTrust> {
    skeleton: Skeleton,
    armor: Option<String>,
    species: PhantomData<D>,
    projection: PhantomData<P>,
    trust: PhantomData<T>,
}

impl<D: Species, P: Projection, T: SignerTrust> ParsedReceiptSkeleton<D, P, T> {
    /// Parse the checked span. The species and projection parsed out must equal the ones the
    /// signature domain was selected with, or the document is refused rather than coerced.
    ///
    /// # Errors
    /// Refuses a grammar departure or a region presence the projection disagrees with.
    pub fn of(
        checked: &ReceiptSignatureChecked<T>,
        limits: &ReceiptLimits,
    ) -> Result<Self, RefusalReason> {
        let skeleton = format::parse_body::<D, P>(checked.body(), limits)?;
        if P::HAS_OVERLAY != checked.armor().is_some() {
            return Err(RefusalReason::OverlayPresence);
        }
        Ok(Self {
            skeleton,
            armor: checked.armor().map(str::to_owned),
            species: PhantomData,
            projection: PhantomData,
            trust: PhantomData,
        })
    }

    /// The parsed skeleton.
    #[must_use]
    pub const fn skeleton(&self) -> &Skeleton {
        &self.skeleton
    }

    /// The armored region, when the projection carries one.
    #[must_use]
    pub fn armor(&self) -> Option<&str> {
        self.armor.as_deref()
    }
}

/// A format-complete receipt for its projection.
///
/// Complete means the document parsed whole under its own grammar. It does not mean what the
/// document says is so, that it is current, or that it may be shared.
#[derive(Debug)]
pub struct Receipt<D: Species, P: Projection, T: SignerTrust> {
    skeleton: Skeleton,
    species: PhantomData<D>,
    projection: PhantomData<P>,
    trust: PhantomData<T>,
}

impl<D: Species, T: SignerTrust> Receipt<D, crate::model::Plain, T> {
    /// Complete a plain document. A plain projection carries no region, so there is nothing
    /// further to validate.
    #[must_use]
    pub fn of_plain(parsed: ParsedReceiptSkeleton<D, crate::model::Plain, T>) -> Self {
        Self {
            skeleton: parsed.skeleton,
            species: PhantomData,
            projection: PhantomData,
            trust: PhantomData,
        }
    }
}

impl<D: Species, P: Projection, T: SignerTrust> Receipt<D, P, T> {
    /// The document's records.
    #[must_use]
    pub const fn skeleton(&self) -> &Skeleton {
        &self.skeleton
    }

    /// The word a report renders for this document's provenance.
    #[must_use]
    pub const fn signer_provenance(&self) -> &'static str {
        T::TOKEN
    }
}

/// A completed plain read, carrying which provenance its material had.
///
/// Two arms rather than a flag, so a consumer that only accepts policy-named material
/// cannot be handed the other by mistake.
#[derive(Debug)]
pub enum ReadPlain<D: Species> {
    /// Read under material controller policy names.
    Trusted(crate::reingested::Reingested<Receipt<D, crate::model::Plain, TrustedReceiptSigner>>),
    /// Read under material controller policy does not name.
    SelfAsserted(
        crate::reingested::Reingested<Receipt<D, crate::model::Plain, SelfAssertedReceiptSigner>>,
    ),
}

/// Read one plain document end to end: bound, locate, check, parse, seal.
///
/// The one entry point, so the ordering cannot be taken out of order by a caller. Every
/// failure answers a [`PartialReceipt`] naming one condition; none of them yields a
/// [`Receipt`].
///
/// # Errors
/// Answers a partial receipt for every condition that stops the read.
pub fn read_plain<D: Species>(
    bytes: Vec<u8>,
    limits: &ReceiptLimits,
    resolver: &dyn VerificationKeyResolver,
) -> Result<ReadPlain<D>, PartialReceipt> {
    let bounded = BoundedReceiptBytes::of(bytes, limits).map_err(PartialReceipt::of)?;
    let located = bounded.locate(limits).map_err(PartialReceipt::of)?;
    if located.armor.is_some() {
        return Err(PartialReceipt::of(RefusalReason::OverlayPresence));
    }
    match check_signature::<D, crate::model::Plain>(&located, resolver)
        .map_err(PartialReceipt::of)?
    {
        Checked::Trusted(checked) => {
            let parsed = ParsedReceiptSkeleton::<D, crate::model::Plain, TrustedReceiptSigner>::of(
                &checked, limits,
            )
            .map_err(PartialReceipt::of)?;
            Ok(ReadPlain::Trusted(crate::reingested::Reingested::seal(
                Receipt::of_plain(parsed),
            )))
        }
        Checked::SelfAsserted(checked) => {
            let parsed =
                ParsedReceiptSkeleton::<D, crate::model::Plain, SelfAssertedReceiptSigner>::of(
                    &checked, limits,
                )
                .map_err(PartialReceipt::of)?;
            Ok(ReadPlain::SelfAsserted(
                crate::reingested::Reingested::seal(Receipt::of_plain(parsed)),
            ))
        }
    }
}

/// Why a document is being reported rather than used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialReceipt {
    reason: RefusalReason,
    bounded_structure: Option<String>,
}

impl PartialReceipt {
    /// Report a document that did not complete.
    #[must_use]
    pub const fn of(reason: RefusalReason) -> Self {
        Self {
            reason,
            bounded_structure: None,
        }
    }

    /// Attach one bounded structural view, rendered under a single status.
    #[must_use]
    pub fn with_structure(mut self, text: String) -> Self {
        self.bounded_structure = Some(text);
        self
    }

    /// Why the document did not complete.
    #[must_use]
    pub const fn reason(&self) -> &RefusalReason {
        &self.reason
    }

    /// The bounded structural view, where one was attached. Rendered whole under one status;
    /// no field of it is promoted on its own.
    #[must_use]
    pub fn bounded_structure(&self) -> Option<&str> {
        self.bounded_structure.as_deref()
    }
}
