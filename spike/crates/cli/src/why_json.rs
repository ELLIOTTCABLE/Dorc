//! The `--json` sibling of the total surface (`30V` §5): the SAME reconstruction, mechanically
//! serialized.
//!
//! # Version-unstable, loudly
//!
//! Pre-user, nothing here is a contract. The envelope says so in its own first field rather than in
//! documentation nobody reads with a pipe attached: promising an additive envelope now would be the
//! `stability-ledger`'s plan-as-API failure-mode wearing a machine format's clothes.
//!
//! # Withholds are markers, never absent keys
//!
//! Every slot carries BOTH keys. A consumer that had to tell "the run held no value" from "the key
//! is missing this version" by probing for absence would be inferring our schema history from their
//! own data, and the four absences `30V` §3 keeps apart would collapse into one.
//!
//! # Keys are hardcoded and out of the registry
//!
//! The arrangement registry is a RENDER-plane home (`aid/CLAUDE.md artifact-plane-strings-stay-out`):
//! a machine format's key is not chrome anybody edits. The two surfaces share their VALUE spellings
//! (`why_total`'s text seats) so a token cannot read one way here and another there.

use std::fmt::Write as _;

use dorc_lint::json::escape_into;
use dorc_receipt::report::{ValueClass, ValueEncoder};
use dorc_why::known::Known;
use dorc_why::{Carrier, CarrierRole, Datum, DatumId, Delivery, Reconstruction, VoiceSet};

use crate::why_total::{
    Coverage, absence_word, correlation_text, flag_text, identity_text, locus_text, state_text,
    subject_text, token_text,
};

/// The envelope's own name, and the whole of its stability promise.
const FORMAT: &str = "dorc-why-json/unstable";

/// The destination encoder for a JSON sink: the display seat, then the string escape.
///
/// COMPOSED rather than invented (`sinv-sink-encoding` — one centralized encoder per destination):
/// `encode_foreign` is what makes the bytes printable and countable, and `escape_into` is what makes
/// them a well-formed JSON string body. Its output is already escaped, so the serializer wraps it in
/// quotes and never escapes it twice.
#[derive(Debug, Clone, Copy)]
pub struct JsonValues {
    cap: usize,
}

impl Default for JsonValues {
    fn default() -> Self {
        Self {
            cap: dorc_aid::said::WHY_VALUE_CAP,
        }
    }
}

impl ValueEncoder for JsonValues {
    fn encode(&mut self, class: ValueClass, bytes: &[u8]) -> String {
        let shown = match class {
            ValueClass::ShellText
            | ValueClass::SourceText
            | ValueClass::SourcePath
            | ValueClass::ArtifactLabel
            | ValueClass::OriginClaim
            | ValueClass::Argv
            | ValueClass::TargetName
            | ValueClass::HostOutput
            | ValueClass::Coordinate
            | ValueClass::EncodedStructure
            | ValueClass::DiagnosticDetail => {
                dorc_aid::display::encode_foreign(&String::from_utf8_lossy(bytes), self.cap)
            }
        };
        let mut out = String::new();
        escape_into(&mut out, &shown);
        out
    }
}

/// Serialize one reconstruction, appending each datum to the coverage ledger as it is written.
///
/// The ledger is the total surface's, and appended at the same kind of seat, so the two surfaces
/// make the SAME totality claim rather than two claims that happen to agree.
#[must_use]
pub fn why_json(
    reconstruction: &Reconstruction,
    encoder: &mut dyn ValueEncoder,
) -> (String, Coverage) {
    let mut coverage = Coverage::default();
    let mut out = String::new();
    out.push_str("{\"format\":");
    push_quoted(&mut out, FORMAT);

    out.push_str(",\"carriers\":[");
    for (index, carrier) in reconstruction.carriers().iter().enumerate() {
        push_separator(&mut out, index);
        push_carrier(&mut out, carrier);
    }

    out.push_str("],\"data\":[");
    for (index, datum) in reconstruction.data().iter().enumerate() {
        push_separator(&mut out, index);
        coverage.saw(DatumId::of(index));
        push_datum(&mut out, datum, encoder);
    }

    out.push_str("],\"correlations\":[");
    for (index, correlation) in reconstruction.structure().receipts().iter().enumerate() {
        push_separator(&mut out, index);
        push_quoted(&mut out, &correlation_text(correlation));
    }

    let loci = reconstruction.structure().loci();
    out.push_str("],\"loci\":{\"nodes\":[");
    for (index, locus) in loci.nodes().iter().enumerate() {
        push_separator(&mut out, index);
        out.push_str("{\"locus\":");
        push_quoted(&mut out, &locus_text(locus));
        out.push_str(",\"address\":");
        push_slot(&mut out, &locus.address, |address| {
            format!(
                "{} {}..{}",
                address.source.get(),
                address.span.0,
                address.span.1
            )
        });
        out.push('}');
    }
    out.push_str("],\"edges\":[");
    for (index, edge) in loci.edges().iter().enumerate() {
        push_separator(&mut out, index);
        let _ = write!(out, "{{\"from\":{},\"to\":{}}}", edge.from, edge.to);
    }

    out.push_str("]},\"audit\":[");
    for (index, hole) in reconstruction.audit().iter().enumerate() {
        push_separator(&mut out, index);
        out.push_str("{\"family\":");
        push_quoted(&mut out, hole.family.token());
        out.push_str(",\"cause\":");
        push_quoted(&mut out, &format!("{:?}", hole.cause));
        out.push('}');
    }
    out.push_str("]}");
    (out, coverage)
}

fn push_separator(out: &mut String, index: usize) {
    if index > 0 {
        out.push(',');
    }
}

/// One string, escaped through the shared escape seat.
fn push_quoted(out: &mut String, text: &str) {
    out.push('"');
    escape_into(out, text);
    out.push('"');
}

/// One string the ENCODER already escaped, wrapped without a second escape.
fn push_encoded(out: &mut String, encoded: &str) {
    out.push('"');
    out.push_str(encoded);
    out.push('"');
}

/// One wrapped slot: both keys, always. `state` is `present` or the absence's own machine word.
fn push_slot<T>(out: &mut String, known: &Known<T>, present: impl FnOnce(&T) -> String) {
    match (known.value(), absence_word(known)) {
        (Some(value), _) => {
            out.push_str("{\"state\":\"present\",\"value\":");
            push_quoted(out, &present(value));
            out.push('}');
        }
        (None, Some(word)) => {
            out.push_str("{\"state\":");
            push_quoted(out, word);
            out.push_str(",\"value\":null}");
        }
        // Unreachable by construction: a slot with no value has an absence word. Serialized rather
        // than panicked, because a machine surface refusing to close its own brace is worse than a
        // slot that says it could not say.
        (None, None) => out.push_str("{\"state\":\"unspellable\",\"value\":null}"),
    }
}

fn push_carrier(out: &mut String, carrier: &Carrier) {
    out.push_str("{\"document\":");
    push_quoted(out, &carrier.document.hex());
    out.push_str(",\"species\":");
    push_quoted(out, carrier.species.token());
    out.push_str(",\"role\":");
    push_quoted(
        out,
        &match &carrier.role {
            CarrierRole::Root => "root".to_owned(),
            CarrierRole::Reached => "reached".to_owned(),
            CarrierRole::Sibling(state) => format!("sibling {state:?}"),
        },
    );
    out.push_str(",\"authentication\":");
    push_slot(out, &carrier.authentication, |state| format!("{state:?}"));
    out.push_str(",\"projection\":");
    push_slot(out, &carrier.projection, |state| format!("{state:?}"));
    out.push_str(",\"detail\":");
    push_slot(out, &carrier.detail, |state| format!("{state:?}"));
    out.push('}');
}

fn push_datum(out: &mut String, datum: &Datum, encoder: &mut dyn ValueEncoder) {
    out.push_str("{\"subject\":");
    push_slot(out, datum.subject(), subject_text);
    out.push_str(",\"speaker\":");
    push_slot(out, datum.speaker(), |speaker| {
        format!(
            "{:?} {}",
            speaker.act(),
            match speaker.voices().value() {
                Some(VoiceSet::Mine) => "mine".to_owned(),
                Some(VoiceSet::One(_)) => "one".to_owned(),
                Some(VoiceSet::Committee { voices, .. }) => format!("committee {}", voices.len()),
                None => absence_word(speaker.voices())
                    .unwrap_or("unspellable")
                    .to_owned(),
            }
        )
    });

    out.push_str(",\"payload\":");
    match datum.payload().value() {
        Some(payload) => {
            out.push_str("{\"state\":\"present\",\"value\":");
            match payload {
                dorc_why::Payload::Text(value) => {
                    push_encoded(out, &value.render(encoder));
                }
                other => push_quoted(out, &payload_text(other)),
            }
            out.push('}');
        }
        None => push_slot(out, datum.payload(), |_| String::new()),
    }

    let world = datum.world();
    out.push_str(",\"world\":{\"moment\":");
    push_slot(out, world.moment(), |moment| match moment {
        dorc_why::Moment::Filed(order) => order.clone(),
        dorc_why::Moment::Undated => "undated".to_owned(),
    });
    out.push_str(",\"host\":");
    match world.host().value() {
        Some(host) => {
            out.push_str("{\"state\":\"present\",\"value\":");
            push_encoded(out, &host.value().render(encoder));
            out.push('}');
        }
        None => push_slot(out, world.host(), |_| String::new()),
    }
    out.push_str(",\"lineage\":");
    push_slot(out, world.lineage(), |lineage| {
        let dorc_why::AttemptLineage::Document(document) = lineage;
        document.hex()
    });

    out.push_str("},\"delivery\":");
    push_quoted(
        out,
        &match datum.delivery() {
            Delivery::Recorded(reference) => reference.get().to_string(),
            Delivery::Live => "live".to_owned(),
        },
    );
    out.push('}');
}

/// Every payload kind but the one that carries bytes, in the SAME spellings the text surface uses.
///
/// The byte-carrying arm is absent on purpose: it is the only one that must reach the caller's
/// encoder, and giving it a spelling here would be a second exit for recorded material.
fn payload_text(payload: &dorc_why::Payload) -> String {
    use dorc_why::Payload as P;
    match payload {
        P::Decision(disposition) => {
            dorc_receipt::tokens::ClosedToken::token(*disposition).to_owned()
        }
        P::Influence(grade) => grade.token().to_owned(),
        P::Identity(identity) => identity_text(identity),
        P::State(state) => state_text(*state),
        P::Correlation(correlation) => correlation_text(correlation),
        P::Collapse(kind) => dorc_receipt::tokens::ClosedToken::token(*kind).to_owned(),
        P::Token(token) => token_text(*token),
        P::Flag(flag) => flag_text(*flag),
        P::NegativeSpace(space) => format!(
            "{} {}",
            match space.kind {
                dorc_why::NegativeKind::ReportApiGap => "absent-report-api-lacks",
                dorc_why::NegativeKind::CarrierGap => "absent-run-held-no-value",
            },
            space.family.token()
        ),
        // Bytes leave through the encoder at the call site above and never through a spelling here.
        P::Text(_) => String::new(),
    }
}
