//! The writer's affine states.
//!
//! Each transition consumes its predecessor, so a caller cannot sign a mutable object, add a
//! region after signing, publish unsigned bytes, or reuse a publication. The states hold
//! private fields and are not `Clone`.

use core::marker::PhantomData;

use crate::capability::{OverlaySealer, PublicationGrade, ReceiptSigner, ReceiptSink};
use crate::format::{self, RefusalReason, Skeleton};
use crate::ids::to_hex;
use crate::model::{Plain, Projection, Rich, Species, payload_type};
use crate::overlay::{self, OverlayEntry};

/// A receipt still being assembled.
#[derive(Debug)]
pub struct DraftReceipt<D: Species, P: Projection> {
    skeleton: Skeleton,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: Species, P: Projection> DraftReceipt<D, P> {
    /// Begin a draft from a complete skeleton.
    #[must_use]
    pub const fn of(skeleton: Skeleton) -> Self {
        Self {
            skeleton,
            species: PhantomData,
            projection: PhantomData,
        }
    }
}

impl<D: Species> DraftReceipt<D, Plain> {
    /// Serialize with no region, proving none is carried.
    ///
    /// # Errors
    /// Refuses a draft carrying region material.
    pub fn serialize(self) -> Result<SerializedReceipt<D, Plain>, RefusalReason> {
        if self.skeleton.encryption_key_id.is_some() {
            return Err(RefusalReason::OverlayPresence);
        }
        let text = format::serialize_skeleton::<D, Plain>(&self.skeleton)?;
        Ok(SerializedReceipt {
            skeleton: text,
            armor: None,
            species: PhantomData,
            projection: PhantomData,
        })
    }
}

impl<D: Species> DraftReceipt<D, Rich> {
    /// Serialize with exactly one region, sealing the plaintext once.
    ///
    /// The plaintext is consumed by value and sealed in the same act, so no caller retains it
    /// and no second region can be produced from it. Three things are checked before the
    /// region is stored: that the sealer answered at all, that it is the provider the skeleton
    /// names, and that what came back is a region this format's own reader can locate. The
    /// last is what stops a document being emitted that its own reader would refuse.
    ///
    /// # Errors
    /// Refuses a draft naming no encryption provider, a provider that is not the sealer's, a
    /// declined seal, or a region outside the stored armored shape.
    pub fn serialize(
        self,
        plaintext: OverlayPlaintext,
        sealer: &dyn OverlaySealer,
    ) -> Result<SerializedReceipt<D, Rich>, RefusalReason> {
        let Some(declared) = self.skeleton.encryption_key_id.clone() else {
            return Err(RefusalReason::OverlayPresence);
        };
        if declared != sealer.encryption_key_id().hex() {
            return Err(RefusalReason::ProviderMismatch);
        }
        let text = format::serialize_skeleton::<D, Rich>(&self.skeleton)?;
        let OverlayPlaintext { bytes } = plaintext;
        let armor = sealer.seal(&bytes).ok_or(RefusalReason::SealDeclined)?;
        format::check_armor_shape(&armor)?;
        Ok(SerializedReceipt {
            skeleton: text,
            armor: Some(armor),
            species: PhantomData,
            projection: PhantomData,
        })
    }
}

/// One region's plaintext, before sealing. Not `Clone`: it is sealed once or dropped.
#[derive(Debug)]
pub struct OverlayPlaintext {
    bytes: Vec<u8>,
}

impl OverlayPlaintext {
    /// Build one region's plaintext from its entries, in canonical form.
    ///
    /// The one public way to obtain a plaintext, so a caller supplies content and never
    /// layout: ordering, framing, and the document binding are written here.
    #[must_use]
    pub fn canonical(
        receipt_id: &str,
        species: &str,
        skeleton_span: &[u8],
        entries: &[OverlayEntry],
    ) -> Self {
        Self {
            bytes: overlay::serialize(receipt_id, species, skeleton_span, entries),
        }
    }

    /// How many bytes this region occupies once opened.
    ///
    /// The quantity a reader bounds, measured on the exact bytes about to be sealed rather than
    /// estimated from the entries — framing counts, and a writer that summed only payloads would
    /// pass its own check and emit a document its own reader refuses.
    #[must_use]
    pub fn opened_bytes(&self) -> u64 {
        u64::try_from(self.bytes.len()).unwrap_or(u64::MAX)
    }
}

/// A receipt whose bytes are fixed. No semantic value can change past this point.
#[derive(Debug)]
pub struct SerializedReceipt<D: Species, P: Projection> {
    skeleton: String,
    armor: Option<String>,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: Species, P: Projection> SerializedReceipt<D, P> {
    /// Sign the exact body span and append the trailer.
    ///
    /// The signing input is the envelope over the same bytes the reader will parse; the
    /// payload type comes from the type parameters and cannot be selected by a caller.
    pub fn sign(self, signer: &dyn ReceiptSigner) -> SignedReceipt<D, P> {
        let body = format::signed_body(&self.skeleton, self.armor.as_deref());
        let signature = signer.sign(&crate::ids::pae(&payload_type::<D, P>(), &body));
        let bytes = format::assemble(&self.skeleton, self.armor.as_deref(), &to_hex(&signature));
        SignedReceipt {
            bytes,
            species: PhantomData,
            projection: PhantomData,
        }
    }

    /// The literal skeleton span, for a caller computing a digest over it.
    #[must_use]
    pub fn skeleton_bytes(&self) -> &[u8] {
        self.skeleton.as_bytes()
    }
}

/// A complete, signed document. Immutable byte owner.
#[derive(Debug)]
pub struct SignedReceipt<D: Species, P: Projection> {
    bytes: Vec<u8>,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: Species, P: Projection> SignedReceipt<D, P> {
    /// The exact document bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Publish, consuming this document. Only a successful sink answer mints a publication.
    ///
    /// # Errors
    /// Answers a failure when the sink does not place the document.
    pub fn publish(
        self,
        name: &str,
        sink: &mut dyn ReceiptSink,
    ) -> Result<PublishedReceipt<D, P>, PublicationFailure> {
        match sink.publish(name, &self.bytes) {
            Some(grade) => Ok(PublishedReceipt {
                bytes: self.bytes,
                grade,
                species: PhantomData,
                projection: PhantomData,
            }),
            None => Err(PublicationFailure),
        }
    }
}

/// The sink did not place the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationFailure;

/// A published document and the grade its sink reported.
#[derive(Debug)]
pub struct PublishedReceipt<D: Species, P: Projection> {
    bytes: Vec<u8>,
    grade: PublicationGrade,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: Species, P: Projection> PublishedReceipt<D, P> {
    /// The grade the sink reported.
    #[must_use]
    pub const fn grade(&self) -> PublicationGrade {
        self.grade
    }

    /// The exact document bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
