//! The reader's monotone states.
//!
//! Bytes are bounded, then located, then checked, then parsed — in that order, with no path
//! that reaches a later state without passing every earlier one. A partial result never
//! converts to a complete one, in either direction.
//!
//! # What a checked document claims, and what it does not
//!
//! It claims exactly one thing: the signature is VALID under the material the resolver held for
//! the provider the document names. It does not claim the material is this controller's own —
//! nothing here can know that, and a state that said so would be saying whatever the resolver
//! said. Local authentication is the composition root's statement, made where a validated
//! keyset is held, and it travels in that seat's own envelope rather than in a type parameter
//! any crate could fill.

use core::marker::PhantomData;

use crate::capability::VerificationKeyResolver;
use crate::format::{self, LocatedReceiptEnvelope, RefusalReason, Skeleton};
use crate::ids::{SigningKeyId, from_hex_32};
use crate::limits::ReceiptLimits;
use crate::model::{Projection, Species, payload_type};

/// Input that has passed the aggregate bound, before anything is parsed or allocated from
/// a value the document declared.
#[derive(Debug)]
pub struct BoundedReceiptBytes {
    bytes: Vec<u8>,
}

impl BoundedReceiptBytes {
    /// Take the bytes back, consuming the bound.
    ///
    /// The seam between a store that read a file under its own ceiling and a reader that bounds
    /// again from scratch. Independent re-bounding is required — a writer's cap never proves a
    /// pre-existing file is bounded — so handing the bytes across gives up nothing: the reader's
    /// first act is to measure them itself.
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }

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

/// A document whose signature checked under the material the resolver held.
///
/// Still unparsed, and not a statement that anything it says is so — nor that the key belongs to
/// anybody in particular.
#[derive(Debug)]
pub struct ReceiptSignatureChecked {
    body: Vec<u8>,
    skeleton: Vec<u8>,
    armor: Option<String>,
}

impl ReceiptSignatureChecked {
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

/// Check a located document against the material the resolver holds for its named provider.
///
/// One answer. There used to be two, sorted by whether the resolver called its own material
/// "trusted", and the sorting was worth nothing: the resolver said which. What is left is the
/// question this crate can decide from the bytes in front of it.
///
/// # Errors
/// Refuses a misshaped signature, a failed check, or material the resolver does not hold.
pub fn check_signature<D: Species, P: Projection>(
    located: &LocatedReceiptEnvelope,
    resolver: &dyn VerificationKeyResolver,
) -> Result<ReceiptSignatureChecked, RefusalReason> {
    let id = SigningKeyId::of_hex(&located.signing_key_id).ok_or(RefusalReason::SignatureShape)?;
    let signature = signature_bytes(&located.signature_hex)?;
    let input = crate::ids::pae(&payload_type::<D, P>(), &located.body);

    let key = resolver.material(id).ok_or(RefusalReason::KeyUnavailable)?;
    if !key.verify(&input, &signature) {
        return Err(RefusalReason::SignatureCheck);
    }
    Ok(ReceiptSignatureChecked {
        body: located.body.clone(),
        skeleton: located.skeleton.clone(),
        armor: located.armor.clone(),
    })
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
pub struct ParsedReceiptSkeleton<D: Species, P: Projection> {
    skeleton: Skeleton,
    armor: Option<String>,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: Species, P: Projection> ParsedReceiptSkeleton<D, P> {
    /// Parse the checked span. The species and projection parsed out must equal the ones the
    /// signature domain was selected with, or the document is refused rather than coerced.
    ///
    /// # Errors
    /// Refuses a grammar departure or a region presence the projection disagrees with.
    pub fn of(
        checked: &ReceiptSignatureChecked,
        limits: &ReceiptLimits,
    ) -> Result<Self, RefusalReason> {
        let skeleton = format::parse_skeleton_span::<D, P>(checked.skeleton(), limits)?;
        if P::HAS_OVERLAY != checked.armor().is_some() {
            return Err(RefusalReason::OverlayPresence);
        }
        Ok(Self {
            skeleton,
            armor: checked.armor().map(str::to_owned),
            species: PhantomData,
            projection: PhantomData,
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
/// Complete means the document parsed whole under its own grammar, and its signature checked
/// under material somebody supplied. It does not mean what the document says is so, that it is
/// current, that it may be shared, or that the key was this controller's.
#[derive(Debug)]
pub struct Receipt<D: Species, P: Projection> {
    skeleton: Skeleton,
    region: P::Region,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: Species> Receipt<D, crate::model::Plain> {
    /// Complete a plain document. A plain projection carries no region, so there is nothing
    /// further to validate.
    #[must_use]
    pub fn of_plain(parsed: ParsedReceiptSkeleton<D, crate::model::Plain>) -> Self {
        Self {
            skeleton: parsed.skeleton,
            region: crate::model::NoOpaqueOverlay,
            species: PhantomData,
            projection: PhantomData,
        }
    }
}

impl<D: Species, P: Projection> Receipt<D, P> {
    /// The document's records.
    #[must_use]
    pub const fn skeleton(&self) -> &Skeleton {
        &self.skeleton
    }
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
) -> Result<crate::reingested::Reingested<Receipt<D, crate::model::Plain>>, PartialReceipt> {
    let bounded = BoundedReceiptBytes::of(bytes, limits).map_err(PartialReceipt::of)?;
    let located = bounded.locate(limits).map_err(PartialReceipt::of)?;
    if located.armor.is_some() {
        return Err(PartialReceipt::of(RefusalReason::OverlayPresence));
    }
    let checked = check_signature::<D, crate::model::Plain>(&located, resolver)
        .map_err(PartialReceipt::of)?;
    let parsed = ParsedReceiptSkeleton::<D, crate::model::Plain>::of(&checked, limits)
        .map_err(PartialReceipt::of)?;
    Ok(crate::reingested::Reingested::seal(Receipt::of_plain(
        parsed,
    )))
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

impl<D: Species> Receipt<D, crate::model::Rich> {
    /// Complete a rich document from a region that has already validated.
    ///
    /// Private to the crate and reachable only from [`read_rich`], so a rich receipt cannot
    /// exist without a region that was checked against this document's own skeleton.
    fn of_rich(
        parsed: ParsedReceiptSkeleton<D, crate::model::Rich>,
        region: crate::overlay::ValidatedOpaqueOverlay,
    ) -> Self {
        Self {
            skeleton: parsed.skeleton,
            region,
            species: PhantomData,
            projection: PhantomData,
        }
    }

    /// The bytes filling one slot of one record.
    ///
    /// Crate-private, and the two readers below are the whole of what reaches the region: the
    /// public exit is `Reingested::recorded_details`, which seals every value under its slot's
    /// class. A public `&[u8]` here was the easier of two routes out (`30Ri`).
    pub(crate) fn detail(
        &self,
        record: u64,
        tag: crate::projection::OpaqueFieldTag,
    ) -> Option<&[u8]> {
        self.region.value(record, tag)
    }

    /// The validated region.
    pub(crate) const fn region(&self) -> &crate::overlay::ValidatedOpaqueOverlay {
        &self.region
    }
}

/// Read one rich document end to end: bound, locate, check, parse, open, validate, seal.
///
/// The order is the point and it is not negotiable at a call site. Opening cannot begin until
/// the outer signature has checked, and no opened byte reaches a report until the region has
/// validated completely against the skeleton that was signed. A failure at any step answers a
/// partial receipt and releases nothing — never a partial enrichment.
///
/// # Errors
/// Answers a partial receipt for every condition that stops the read.
pub fn read_rich<D: Species>(
    bytes: Vec<u8>,
    limits: &ReceiptLimits,
    resolver: &dyn VerificationKeyResolver,
    opener: &dyn crate::capability::OverlayOpener,
) -> Result<crate::reingested::Reingested<Receipt<D, crate::model::Rich>>, PartialReceipt> {
    let bounded = BoundedReceiptBytes::of(bytes, limits).map_err(PartialReceipt::of)?;
    let located = bounded.locate(limits).map_err(PartialReceipt::of)?;
    if located.armor.is_none() {
        return Err(PartialReceipt::of(RefusalReason::OverlayPresence));
    }
    let checked =
        check_signature::<D, crate::model::Rich>(&located, resolver).map_err(PartialReceipt::of)?;
    let parsed = ParsedReceiptSkeleton::<D, crate::model::Rich>::of(&checked, limits)
        .map_err(PartialReceipt::of)?;
    let region = open_and_validate::<D>(&checked, &parsed, limits, opener)?;
    Ok(crate::reingested::Reingested::seal(Receipt::of_rich(
        parsed, region,
    )))
}

/// Open the region and validate it against the skeleton that was signed.
///
/// Takes the checked state by reference so the span it digests is the span the signature
/// covered, not a re-read of anything.
fn open_and_validate<D: Species>(
    checked: &ReceiptSignatureChecked,
    parsed: &ParsedReceiptSkeleton<D, crate::model::Rich>,
    limits: &ReceiptLimits,
    opener: &dyn crate::capability::OverlayOpener,
) -> Result<crate::overlay::ValidatedOpaqueOverlay, PartialReceipt> {
    let armor = parsed
        .armor()
        .ok_or_else(|| PartialReceipt::of(RefusalReason::OverlayPresence))?;
    let plaintext = opener
        .open(armor, limits.overlay_bytes.get())
        .ok_or_else(|| PartialReceipt::of(RefusalReason::RegionUnopenable))?;
    crate::overlay::DecryptedOpaqueOverlay::of(plaintext)
        .validate(parsed.skeleton(), checked.skeleton(), D::TOKEN, limits)
        .map_err(|fault| PartialReceipt::of(RefusalReason::Overlay(fault)))
}
