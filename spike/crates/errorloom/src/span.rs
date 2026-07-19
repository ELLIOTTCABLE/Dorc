//! The span-map schema (`282` §4): errorloom owns the region vocabulary that
//! classifies every run of a baseline render, generic over an opaque consumer
//! key `K` (`28A` §1 — Dorc later emits core-owned spans and adapts onto these).
//!
//! The map is the attribution AUTHORITY for promote; the word-diff is only
//! alignment (`282` §5).

use std::cmp::Ordering;
use std::fmt;
use std::ops::Range;

use crate::prose::ParamName;

/// An explicit occurrence discriminator for a field's render.
///
/// A field may render more than once in a transcript (`282` §5 "two instances
/// of one template"). When a consumer stamps each render with an `InstanceId`,
/// `promote` groups spans into instances by exact identity
/// (`28A:rul-tagged-render-emits-instance-ids`); when absent, it falls back to
/// the d1 structural inference (paragraph/adjacency heuristic). Opting in is
/// per-key all-or-nothing: a key whose spans all carry an id is grouped exactly,
/// otherwise the whole key falls back to structural.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InstanceId(usize);

impl InstanceId {
    /// Wrap an occurrence index.
    #[must_use]
    pub fn new(index: usize) -> Self {
        InstanceId(index)
    }

    /// The occurrence index.
    #[must_use]
    pub fn get(self) -> usize {
        self.0
    }
}

/// An arrangement region's display slug: render-owned structure (numbering,
/// connectives, tier words, blank structure — `282` §4). Carried only so a
/// refusal can name what structure was touched.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArrangementSlug(String);

impl ArrangementSlug {
    /// Wrap a slug.
    pub fn new(slug: impl Into<String>) -> Self {
        ArrangementSlug(slug.into())
    }

    /// The slug's text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The classification of one byte-run of a baseline render (`282` §4). `K` is the
/// consumer-opaque field key (Dorc: `(code, field)`); errorloom groups and
/// compares by it but never inspects it.
///
/// `#[non_exhaustive]`: schema growth adds region kinds (e.g. the `282` §8
/// passthrough work), which must not be a breaking change for a published crate.
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum Region<K> {
    /// The field's own prose words for paragraph `paragraph`. The ONLY editable
    /// class — everything else refuses.
    TemplateLiteral {
        /// The field key.
        key: K,
        /// Zero-based paragraph index within the field.
        paragraph: usize,
        /// The occurrence this render belongs to, when the consumer stamps it
        /// (`28A:rul-tagged-render-emits-instance-ids`); `None` ⇒ structural
        /// inference groups the instances.
        instance: Option<InstanceId>,
    },
    /// Interpolated payload for `param`; editing it refuses (data, not prose).
    ParamValue {
        /// The field key.
        key: K,
        /// The hole this value fills.
        param: ParamName,
        /// The occurrence this value belongs to (see `TemplateLiteral::instance`).
        instance: Option<InstanceId>,
    },
    /// Passthrough foreign text riding a hole (`282:rul-passthrough-type-gated`):
    /// tainted bytes, never our prose. Editing it refuses.
    ForeignText {
        /// The hole the foreign text rode.
        param: ParamName,
    },
    /// Render-owned structure; edit it by structure-bless, not prose-bless.
    Arrangement {
        /// What structure this is.
        slug: ArrangementSlug,
    },
}

impl<K> Region<K> {
    /// The field key this region belongs to, if any (template/param regions).
    #[must_use]
    pub fn key(&self) -> Option<&K> {
        match self {
            Region::TemplateLiteral { key, .. } | Region::ParamValue { key, .. } => Some(key),
            Region::ForeignText { .. } | Region::Arrangement { .. } => None,
        }
    }

    /// The explicit occurrence id this region carries, if any.
    #[must_use]
    pub fn instance(&self) -> Option<InstanceId> {
        match self {
            Region::TemplateLiteral { instance, .. } | Region::ParamValue { instance, .. } => {
                *instance
            }
            Region::ForeignText { .. } | Region::Arrangement { .. } => None,
        }
    }
}

/// One classified byte-run of a baseline render.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span<K> {
    /// The byte range within the render this span covers.
    pub range: Range<usize>,
    /// How the run is classified.
    pub region: Region<K>,
}

/// Why a [`TaggedRender`] failed to validate. The span map must be a gap-free
/// cover of the render bytes so region lookup is total (a consumer bug, caught
/// fail-fast — the `inv-top-reject` posture).
///
/// `#[non_exhaustive]`: new validation kinds must not break a published consumer.
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum TaggedRenderError {
    /// Span `index` does not start where the previous span ended.
    NonContiguous {
        /// The offending span index.
        index: usize,
        /// The byte offset the span was expected to start at.
        expected: usize,
        /// The byte offset it actually started at.
        found: usize,
    },
    /// Span `index` has an empty or inverted range.
    EmptyRange {
        /// The offending span index.
        index: usize,
    },
    /// The spans stop short of (or overrun) the render's byte length.
    Uncovered {
        /// Bytes the spans cover.
        covered: usize,
        /// The render's byte length.
        len: usize,
    },
}

impl fmt::Display for TaggedRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaggedRenderError::NonContiguous {
                index,
                expected,
                found,
            } => write!(
                f,
                "span {index} is non-contiguous: expected start {expected}, found {found}"
            ),
            TaggedRenderError::EmptyRange { index } => {
                write!(f, "span {index} has an empty or inverted range")
            }
            TaggedRenderError::Uncovered { covered, len } => {
                write!(
                    f,
                    "spans cover {covered} bytes but the render is {len} bytes"
                )
            }
        }
    }
}

impl std::error::Error for TaggedRenderError {}

/// A baseline render plus its span map: the input the consumer's tagged renderer
/// hands `promote`. Validated on construction to be a gap-free, non-overlapping
/// cover of the render bytes (`282` §4 — "classifying every output run").
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TaggedRender<K> {
    text: String,
    spans: Vec<Span<K>>,
}

impl<K> TaggedRender<K> {
    /// Bundle a render with its span map, validating full coverage.
    ///
    /// # Errors
    /// Returns [`TaggedRenderError`] if the spans are not a gap-free,
    /// non-overlapping cover of exactly `0..text.len()`.
    pub fn new(text: String, spans: Vec<Span<K>>) -> Result<Self, TaggedRenderError> {
        let mut expected: usize = 0;
        for (index, span) in spans.iter().enumerate() {
            if span.range.start != expected {
                return Err(TaggedRenderError::NonContiguous {
                    index,
                    expected,
                    found: span.range.start,
                });
            }
            if span.range.end <= span.range.start {
                return Err(TaggedRenderError::EmptyRange { index });
            }
            expected = span.range.end;
        }
        if expected != text.len() {
            return Err(TaggedRenderError::Uncovered {
                covered: expected,
                len: text.len(),
            });
        }
        Ok(TaggedRender { text, spans })
    }

    /// The rendered text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The span map.
    #[must_use]
    pub fn spans(&self) -> &[Span<K>] {
        &self.spans
    }

    /// The index of the span covering `byte`, or `None` if out of range.
    pub(crate) fn span_index_at(&self, byte: usize) -> Option<usize> {
        self.spans
            .binary_search_by(|s| {
                if byte < s.range.start {
                    Ordering::Greater
                } else if byte >= s.range.end {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()
    }
}
