//! dorc-loom — the Dorc↔errorloom adapter (`282` §4 · `28A` §1).
//!
//! Dorc's render seat emits a core-owned tagged render
//! ([`dorc_core::tagged::TaggedRender`]); dorc-core takes NO dependency on
//! errorloom, so this crate is the ordinary consumer shape that maps those
//! core spans 1:1 onto errorloom's generic span schema, keyed by Dorc's
//! `(code, field)` ([`FieldKey`]). The mapping is validated through
//! `errorloom::TaggedRender::new`, which enforces the gap-free, non-overlapping
//! total cover (`28A:rul-span-cover-stays-total`).
//!
//! Instance ids are stamped ALWAYS on the template/param regions
//! (`28A:rul-tagged-render-emits-instance-ids`) — the per-key all-or-nothing
//! floor. The prose-promote flow itself lives in errorloom; this adapter only
//! produces the tagged baseline it consumes.

use dorc_core::tagged::{self, Region as CoreRegion};
use errorloom::{
    ArrangementSlug, InstanceId, ParamName, Region, Span, TaggedRender, TaggedRenderError,
};

/// The opaque consumer key errorloom groups prose fields by: Dorc's
/// `(code, field)` (`28A` §1). errorloom compares/sorts it but never inspects it;
/// the derives satisfy `errorloom::ConsumerKey` (`Clone + Ord + Debug`).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FieldKey {
    /// The diagnostic code slug the field belongs to.
    pub code: String,
    /// Which prose register (`message`/`help`).
    pub field: &'static str,
}

/// Map a core-owned tagged render onto an `errorloom::TaggedRender`, keyed by
/// [`FieldKey`], validating the gap-free total cover through
/// `errorloom::TaggedRender::new`.
///
/// # Errors
/// Returns [`TaggedRenderError`] when the core spans are not a gap-free,
/// non-overlapping cover of exactly the render bytes — a core-emitter bug,
/// caught fail-fast (the `inv-top-reject` posture).
pub fn to_errorloom(
    tagged: &tagged::TaggedRender,
) -> Result<TaggedRender<FieldKey>, TaggedRenderError> {
    let spans = tagged.spans().iter().map(map_span).collect();
    TaggedRender::new(tagged.text().to_owned(), spans)
}

fn map_span(span: &tagged::Span) -> Span<FieldKey> {
    Span {
        range: span.range.clone(),
        region: map_region(&span.region),
    }
}

fn map_region(region: &CoreRegion) -> Region<FieldKey> {
    match *region {
        CoreRegion::TemplateLiteral {
            code,
            field,
            paragraph,
            instance,
        } => Region::TemplateLiteral {
            key: key(code, field),
            paragraph,
            instance: Some(InstanceId::new(instance)),
        },
        CoreRegion::ParamValue {
            code,
            field,
            param,
            instance,
        } => Region::ParamValue {
            key: key(code, field),
            param: ParamName::new(param),
            instance: Some(InstanceId::new(instance)),
        },
        CoreRegion::ForeignText { param } => Region::ForeignText {
            param: ParamName::new(param),
        },
        CoreRegion::Arrangement { slug } => Region::Arrangement {
            slug: ArrangementSlug::new(slug),
        },
    }
}

fn key(code: &str, field: tagged::Field) -> FieldKey {
    FieldKey {
        code: code.to_owned(),
        field: field.as_str(),
    }
}
