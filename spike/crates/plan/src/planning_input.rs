//! The planner's input identity (`quarantine/30Rb:receipt-identity-map`).
//!
//! [`PlanningInputId`] names the complete decision-relevant VALUE presented to the planner,
//! authority scope included. It is a content identity, not an event identity: two invocations
//! whose scoped inputs are genuinely identical share one, and that is correct. `ReceiptId` is
//! what tells two events apart.
//!
//! The members divide in two. AUTHORED planning state — the ordered acquired-source table with
//! its exact digests, roles and named paths; controller semantics; target and attempt scope; and
//! every parsed policy value that can move analysis or settlement. ADMITTED world state — the
//! intake outcome and every bounded, typed, controller-attributed record the planner consumed,
//! in the planner's own order and keeping its duplicates. Both halves are required: without the
//! second, a converged host and a drifted host present the same identity, which is not an
//! identity of the planner's inputs at all.
//!
//! Outputs and narration are excluded. Dispositions, render decisions, narratives and artifact
//! bytes are what the planner PRODUCED; `PresentedPlanId` is the identity that binds this one to
//! that finished surface.

use dorc_core::spine::{AdmissionOutcome, SpineAdmission, SpineInvocation};
use dorc_receipt::ids::PlanningInputId;

use crate::records::{AdmittedHostRecord, AdmittedUnscopedHostRecords};

/// The format tag the encoding is domain-separated under.
pub const ENCODING: &str = "dorc-planning-inputs/1";

/// The line closing it, so a truncation is a different value rather than a shorter complete one.
pub const TERMINATOR: &str = "inputs-end";

/// The top-level tags this encoding writes, in order — the census every member is checked
/// against.
///
/// A member added to `PlanningInputs::encode` without a row here fails
/// `the_encoding_writes_exactly_the_census`, and a row without a perturbation case fails
/// `every_census_member_is_load_bearing`. The two together are what make "decision-relevant"
/// mechanical rather than asserted.
pub const CENSUS: [&str; 9] = [
    "semantics",
    "host",
    "attempt",
    "generation",
    "policy-risk-faultless-skips",
    "policy-mode",
    "sources",
    "admission",
    "records",
];

/// Which surface an invocation asked for. Closed, and mapped from the edge's own vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningMode {
    /// The pure oracle-bundle projection.
    Bundle,
    /// The read-only probe artifact alone.
    Probe,
    /// The human-facing preview.
    Plan,
    /// The shippable artifact.
    Apply,
    /// Probe then apply, with full disclosure.
    RoundTrip,
    /// The why-query surface.
    Why,
}

impl PlanningMode {
    const fn token(self) -> &'static str {
        match self {
            Self::Bundle => "bundle",
            Self::Probe => "probe",
            Self::Plan => "plan",
            Self::Apply => "apply",
            Self::RoundTrip => "round-trip",
            Self::Why => "why",
        }
    }
}

/// The parsed policy values that can move analysis or settlement.
///
/// One constructor, private fields, and no `Default`: this is the census point for policy. A new
/// flag that can change what the analyzer concludes or what settlement licenses adds a field
/// here and a row to [`CENSUS`], or it is silently absent from an identity that claims to be
/// complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanningPolicy {
    risk_faultless_skips: bool,
    mode: PlanningMode,
}

impl PlanningPolicy {
    /// Bind the analysis- and settlement-affecting policy of one invocation.
    #[must_use]
    pub const fn of(mode: PlanningMode, risk_faultless_skips: bool) -> Self {
        Self {
            risk_faultless_skips,
            mode,
        }
    }
}

/// The complete decision-relevant value presented to the planner.
///
/// Borrowed rather than owned: every member already exists on the Spine the run wrote, and
/// copying them here would open a second place for them to drift from what was decided.
#[derive(Debug, Clone, Copy)]
pub struct PlanningInputs<'a> {
    semantics: &'static str,
    invocation: &'a SpineInvocation,
    admission: Option<&'a SpineAdmission>,
    records: Option<&'a AdmittedUnscopedHostRecords>,
    policy: PlanningPolicy,
}

impl<'a> PlanningInputs<'a> {
    /// Bind one invocation's inputs.
    ///
    /// `semantics` is the controller's own version string: two builds that could analyse the same
    /// bytes differently must not present one identity.
    #[must_use]
    pub const fn of(
        semantics: &'static str,
        invocation: &'a SpineInvocation,
        admission: Option<&'a SpineAdmission>,
        records: Option<&'a AdmittedUnscopedHostRecords>,
        policy: PlanningPolicy,
    ) -> Self {
        Self {
            semantics,
            invocation,
            admission,
            records,
            policy,
        }
    }

    /// The identity of these inputs.
    #[must_use]
    pub fn identity(&self) -> PlanningInputId {
        PlanningInputId::of_canonical_inputs(&self.encode())
    }

    /// The canonical encoding: length-framed values, explicit counts, explicit absence.
    ///
    /// Nothing here is derived from a generic serializer. Every member is written by name, so a
    /// field this crate gains is absent from the identity until someone writes it down — which is
    /// what [`CENSUS`] and its two tests exist to catch.
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(ENCODING.as_bytes());
        out.push(b'\n');

        put_str(&mut out, "semantics", self.semantics);

        // Target scope. The nonce and the start instant are DELIBERATELY absent: they distinguish
        // events, and folding them in would give every run its own inputs identity, which is the
        // one thing this value must not do.
        let identity = self.invocation.identity();
        put_str(&mut out, "host", &identity.host);
        put_u64(&mut out, "attempt", u64::from(identity.attempt));
        // No generation is minted at plan time. The tag is written anyway so its arrival is a
        // visible change of encoding rather than a silent widening.
        put_absent(&mut out, "generation");

        put_bool(
            &mut out,
            "policy-risk-faultless-skips",
            self.policy.risk_faultless_skips,
        );
        put_str(&mut out, "policy-mode", self.policy.mode.token());

        let sources = self.invocation.sources();
        put_count(&mut out, "sources", sources.len());
        for (ordinal, claim) in sources.iter().enumerate() {
            put_count(&mut out, "source-ordinal", ordinal);
            put_str(&mut out, "source-role", role_token(claim.role));
            put_str(&mut out, "source-path", &claim.path);
            put_str(&mut out, "source-digest", &claim.digest);
            put_u64(&mut out, "source-bytes", claim.bytes);
        }

        match self.admission.map(SpineAdmission::outcome) {
            None => put_absent(&mut out, "admission"),
            Some(AdmissionOutcome::Admitted) => put_str(&mut out, "admission", "admitted"),
            Some(AdmissionOutcome::NoObservation) => {
                put_str(&mut out, "admission", "no-observation");
            }
            Some(AdmissionOutcome::Refused) => put_str(&mut out, "admission", "refused"),
        }

        match self.records {
            None => put_absent(&mut out, "records"),
            Some(records) => {
                let all: Vec<AdmittedHostRecord<'_>> = records.iter().collect();
                put_count(&mut out, "records", all.len());
                for record in &all {
                    put_record(&mut out, record);
                }
            }
        }

        out.extend_from_slice(TERMINATOR.as_bytes());
        out.push(b'\n');
        out
    }
}

const fn role_token(role: dorc_core::SourceRole) -> &'static str {
    match role {
        dorc_core::SourceRole::Book => "book",
        dorc_core::SourceRole::NamedLoad => "named-load",
        dorc_core::SourceRole::BookSourced => "book-sourced",
        dorc_core::SourceRole::LoadDependency => "load-dependency",
        dorc_core::SourceRole::PlainInclusion => "plain-inclusion",
    }
}

/// One admitted record, by kind, exhaustively. Order and duplicates are the planner's: this walks
/// its sequence and never sorts or dedupes it.
fn put_record(out: &mut Vec<u8>, record: &AdmittedHostRecord<'_>) {
    match *record {
        AdmittedHostRecord::Site {
            key,
            effect,
            rc,
            stdout,
            stderr,
            inert,
        } => {
            put_str(out, "record-kind", "site");
            put_str(out, "site-key", key);
            put_str(out, "site-effect", effect);
            put_i64(out, "site-rc", i64::from(rc));
            put_opt_str(out, "site-stdout", stdout);
            put_opt_str(out, "site-stderr", stderr);
            put_count(out, "site-inert", inert.len());
            for (name, value) in inert {
                put_str(out, "inert-name", name);
                put_str(out, "inert-value", value);
            }
        }
        AdmittedHostRecord::Derivation { site, coord } => {
            put_str(out, "record-kind", "derivation");
            put_u64(out, "derivation-site", u64::from(site));
            put_str(out, "derivation-coord", coord);
        }
        AdmittedHostRecord::DerivationEnd {
            site,
            count,
            body_rc,
        } => {
            put_str(out, "record-kind", "derivation-end");
            put_u64(out, "derivation-end-site", u64::from(site));
            put_u64(out, "derivation-end-count", u64::from(count));
            put_u64(out, "derivation-end-body-rc", u64::from(body_rc));
        }
        AdmittedHostRecord::Resolution { coord, canonical } => {
            put_str(out, "record-kind", "resolution");
            put_str(out, "resolution-coord", coord);
            put_opt_str(out, "resolution-canonical", canonical);
        }
        AdmittedHostRecord::Reach { coord, arm, entity } => {
            put_str(out, "record-kind", "reach");
            put_str(out, "reach-coord", coord);
            put_count(out, "reach-arm", arm);
            put_str(out, "reach-entity", entity);
        }
        AdmittedHostRecord::ReachEnd {
            coord,
            arm,
            count,
            body_rc,
        } => {
            put_str(out, "record-kind", "reach-end");
            put_str(out, "reach-end-coord", coord);
            put_count(out, "reach-end-arm", arm);
            put_u64(out, "reach-end-count", u64::from(count));
            put_u64(out, "reach-end-body-rc", u64::from(body_rc));
        }
        AdmittedHostRecord::Report { body } => {
            put_str(out, "record-kind", "report");
            put_str(out, "report-body", body);
        }
    }
}

/// `<tag> <byte-length> <exact bytes>` — framed, so no value can be mistaken for a separator.
fn put_str(out: &mut Vec<u8>, tag: &str, value: &str) {
    out.extend_from_slice(tag.as_bytes());
    out.push(b' ');
    out.extend_from_slice(value.len().to_string().as_bytes());
    out.push(b' ');
    out.extend_from_slice(value.as_bytes());
    out.push(b'\n');
}

/// `<tag> absent`, or the framed present form — never an empty value standing for absence.
fn put_opt_str(out: &mut Vec<u8>, tag: &str, value: Option<&str>) {
    match value {
        None => put_absent(out, tag),
        Some(text) => put_str(out, tag, text),
    }
}

fn put_absent(out: &mut Vec<u8>, tag: &str) {
    out.extend_from_slice(tag.as_bytes());
    out.extend_from_slice(b" absent\n");
}

fn put_u64(out: &mut Vec<u8>, tag: &str, value: u64) {
    put_scalar(out, tag, &value.to_string());
}

fn put_i64(out: &mut Vec<u8>, tag: &str, value: i64) {
    put_scalar(out, tag, &value.to_string());
}

fn put_count(out: &mut Vec<u8>, tag: &str, value: usize) {
    put_scalar(out, tag, &value.to_string());
}

fn put_bool(out: &mut Vec<u8>, tag: &str, value: bool) {
    put_scalar(out, tag, if value { "yes" } else { "no" });
}

fn put_scalar(out: &mut Vec<u8>, tag: &str, value: &str) {
    out.extend_from_slice(tag.as_bytes());
    out.push(b' ');
    out.extend_from_slice(value.as_bytes());
    out.push(b'\n');
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::SourceRole;
    use dorc_core::influence::InfluenceAccount;
    use dorc_core::spine::{InvocationMode, RunIdentity, SourceClaim};

    use crate::records::{
        Admission, Framing, HostEvidenceLimits, admit_unscoped_host_records, read_host_evidence,
    };

    /// Everything one identity is computed from, owned so a case can perturb one member.
    #[derive(Clone)]
    struct Fixture {
        semantics: &'static str,
        sources: Vec<SourceClaim>,
        identity: RunIdentity,
        admission: Option<AdmissionOutcome>,
        records: Option<String>,
        policy: PlanningPolicy,
    }

    fn baseline() -> Fixture {
        Fixture {
            semantics: "dorc/test",
            sources: vec![
                SourceClaim {
                    path: String::from("ufw.oracle.sh"),
                    digest: "a".repeat(64),
                    role: SourceRole::NamedLoad,
                    bytes: 40,
                },
                SourceClaim {
                    path: String::from("webhost.sh"),
                    digest: "b".repeat(64),
                    role: SourceRole::Book,
                    bytes: 12,
                },
            ],
            identity: RunIdentity {
                nonce: String::from("dorc"),
                attempt: 1,
                host: String::from("web1.example.net"),
                started_at: None,
            },
            admission: Some(AdmissionOutcome::Admitted),
            records: Some(wire(&["site 0 effect=holds rc=0"])),
            policy: PlanningPolicy::of(PlanningMode::Plan, false),
        }
    }

    /// A framed record stream the real intake accepts — built through the production reader and
    /// admission path, never hand-assembled into the container.
    fn wire(inners: &[&str]) -> String {
        let framing = Framing::spike(String::from("bk"));
        // The header declares SITE facts, not records: a report line is not a fact, and declaring
        // one truncates the stream at intake.
        let sites = inners
            .iter()
            .filter(|inner| inner.starts_with("site "))
            .count();
        let header = unprintf(&crate::records::header_line(&framing, sites));
        let sentinel = unprintf(&crate::records::sentinel_line(framing.nonce()));
        let body = inners
            .iter()
            .map(|inner| format!("{}\n", crate::records::frame(framing.nonce(), inner)))
            .collect::<Vec<_>>()
            .concat();
        format!("{header}\n{body}{sentinel}\n")
    }

    /// The emitters produce the `printf '…\n'` line a probe would SHIP; the intake reads what that
    /// line would PRINT. Unwrapping here keeps the fixture tied to the production emitters rather
    /// than hand-spelling a second copy of the framing.
    fn unprintf(line: &str) -> String {
        line.trim_end()
            .trim_start_matches("printf '")
            .trim_end_matches("\\n'")
            .to_owned()
    }

    fn admitted(raw: &str) -> AdmittedUnscopedHostRecords {
        let limits = HostEvidenceLimits::spike_default();
        let framing = Framing::spike(String::from("bk"));
        let bytes = match read_host_evidence(raw.as_bytes(), limits) {
            Admission::Admitted(bytes) => bytes,
            other => panic!("the fixture stream must pass the byte reader: {other:?}"),
        };
        match admit_unscoped_host_records(&bytes, &framing, limits) {
            Admission::Admitted(graded) => graded.into_read().0,
            other => panic!("the fixture stream must be admitted: {other:?}"),
        }
    }

    fn identity_of(fixture: &Fixture) -> PlanningInputId {
        let invocation = SpineInvocation::minted(
            InvocationMode::Unstated,
            vec![String::from("dorc"), String::from("plan")],
            fixture.sources.clone(),
            fixture.identity.clone(),
            InfluenceAccount::authored_before_contact(),
        );
        let admission = fixture.admission.map(|outcome| {
            SpineAdmission::minted(outcome, None, InfluenceAccount::authored_before_contact())
        });
        let records = fixture.records.as_deref().map(admitted);
        PlanningInputs::of(
            fixture.semantics,
            &invocation,
            admission.as_ref(),
            records.as_ref(),
            fixture.policy,
        )
        .identity()
    }

    /// Every census member, with the smallest change that must move the identity.
    ///
    /// Two-way against [`CENSUS`]: a member with no case here, or a case naming no member, fails
    /// `every_census_member_is_load_bearing`.
    /// One member, and the smallest change to it that must move the identity.
    type Perturbation = (&'static str, fn(&mut Fixture));

    fn perturbations() -> Vec<Perturbation> {
        vec![
            ("semantics", |f| f.semantics = "dorc/other"),
            ("host", |f| {
                f.identity.host = String::from("web2.example.net");
            }),
            ("attempt", |f| f.identity.attempt = 2),
            ("policy-risk-faultless-skips", |f| {
                f.policy = PlanningPolicy::of(PlanningMode::Plan, true);
            }),
            ("policy-mode", |f| {
                f.policy = PlanningPolicy::of(PlanningMode::Apply, false);
            }),
            ("sources", |f| {
                f.sources[1].digest = "c".repeat(64);
            }),
            ("admission", |f| {
                f.admission = Some(AdmissionOutcome::NoObservation);
            }),
            ("records", |f| {
                f.records = Some(wire(&["site 0 effect=absent rc=1"]));
            }),
        ]
    }

    #[test]
    fn one_value_presents_one_identity() {
        // The property the whole thing rests on: identical scoped inputs share an identity, and
        // that is correct rather than a collision.
        assert_eq!(identity_of(&baseline()), identity_of(&baseline()));
    }

    /// Census members with no value to perturb in V1, each with the reason.
    ///
    /// Enumerated rather than merely missing from the table: a member lands here by ruling, and
    /// the moment it gains a value it has to move to the perturbation table or the union check
    /// below fails. Absence stays a stated fact, never an oversight.
    const ABSENT_BY_CONSTRUCTION: [&str; 1] = [
        // No generation is minted at plan time; the encoding writes the tag as `absent`, and the
        // encoding census is what keeps that visible.
        "generation",
    ];

    #[test]
    fn every_census_member_is_load_bearing() {
        let base = identity_of(&baseline());
        let cases = perturbations();
        let mut named: Vec<&str> = cases.iter().map(|(name, _)| *name).collect();
        named.extend(ABSENT_BY_CONSTRUCTION);
        named.sort_unstable();
        let mut expected = CENSUS.to_vec();
        expected.sort_unstable();
        assert_eq!(
            named, expected,
            "every census member is either perturbable or absent by construction, and no case may \
             name a member the census does not have"
        );
        for (name, perturb) in cases {
            let mut fixture = baseline();
            perturb(&mut fixture);
            assert_ne!(
                identity_of(&fixture),
                base,
                "changing `{name}` did not move the identity, so it is not part of it"
            );
        }
    }

    #[test]
    fn the_encoding_writes_exactly_the_census() {
        // The other direction: a member written into the encoding without a census row, or a row
        // whose tag nothing writes, fails here rather than being caught by nobody.
        let fixture = baseline();
        let invocation = SpineInvocation::minted(
            InvocationMode::Unstated,
            vec![String::from("dorc")],
            fixture.sources.clone(),
            fixture.identity.clone(),
            InfluenceAccount::authored_before_contact(),
        );
        let admission = fixture.admission.map(|outcome| {
            SpineAdmission::minted(outcome, None, InfluenceAccount::authored_before_contact())
        });
        let records = fixture.records.as_deref().map(admitted);
        let encoded = PlanningInputs::of(
            fixture.semantics,
            &invocation,
            admission.as_ref(),
            records.as_ref(),
            fixture.policy,
        )
        .encode();
        let text = String::from_utf8(encoded).expect("the encoding is ASCII-framed");
        for member in CENSUS {
            assert!(
                text.lines()
                    .any(|line| line.starts_with(&format!("{member} "))),
                "the census names `{member}` and the encoding never writes it"
            );
        }
        assert!(text.starts_with(ENCODING), "the domain tag opens the value");
        assert!(text.ends_with("inputs-end\n"), "the value is terminated");
    }

    #[test]
    fn the_world_the_planner_saw_is_part_of_the_identity() {
        // The sharp one. A converged host and a drifted host differ ONLY in admitted records; if
        // those did not reach the identity, both would present the same one, and the value would
        // not be an identity of the planner's inputs at all.
        let mut converged = baseline();
        converged.records = Some(wire(&["site 0 effect=holds rc=0"]));
        let mut drifted = baseline();
        drifted.records = Some(wire(&["site 0 effect=absent rc=1"]));
        assert_ne!(identity_of(&converged), identity_of(&drifted));
    }

    #[test]
    fn the_multiplicity_the_intake_presents_is_the_multiplicity_encoded() {
        // "Preserve duplicates wherever the planner does" — so this pins BOTH halves of what the
        // planner actually does, rather than asserting a multiplicity the intake never delivers.
        //
        // MEASURED, both for site facts and for report bodies: the intake folds an exact repeat
        // before the planner sees anything, so there is no second record for the encoding to
        // preserve and matching identities are correct. The encoding walks the admitted sequence
        // and never dedupes on its own — where a multiplicity survives intake, it survives here.
        //
        // Pinned rather than assumed, in both directions, so the day intake stops folding is a red
        // test here instead of a silent widening of what an identity covers.
        for repeated_record in [
            "site 0 effect=holds rc=0",
            "report site=0 decline unsound host detail",
        ] {
            let mut once = baseline();
            once.records = Some(wire(&[repeated_record]));
            let mut twice = baseline();
            twice.records = Some(wire(&[repeated_record, repeated_record]));
            assert_eq!(
                identity_of(&once),
                identity_of(&twice),
                "`{repeated_record}` repeated exactly folds at intake; if that changes, this \
                 identity must change with it"
            );
        }

        // Order, by contrast, is NOT folded — pinned on its own by
        // `record_order_is_part_of_the_identity`, which is what makes the fold above a fold rather
        // than the encoding losing the sequence.
    }

    #[test]
    fn record_order_is_part_of_the_identity() {
        let mut forward = baseline();
        forward.records = Some(wire(&[
            "site 0 effect=holds rc=0",
            "site 1 effect=absent rc=1",
        ]));
        let mut reversed = baseline();
        reversed.records = Some(wire(&[
            "site 1 effect=absent rc=1",
            "site 0 effect=holds rc=0",
        ]));
        assert_ne!(identity_of(&forward), identity_of(&reversed));
    }

    #[test]
    fn the_event_identity_is_not_folded_in() {
        // A nonce and a start instant distinguish EVENTS. Folding either in would give every run
        // its own inputs identity and destroy the property `one_value_presents_one_identity` pins.
        let mut other_event = baseline();
        other_event.identity.nonce = String::from("different");
        other_event.identity.started_at = Some(dorc_core::RunInstant(1_234));
        assert_eq!(identity_of(&other_event), identity_of(&baseline()));
    }

    #[test]
    fn source_order_is_part_of_the_identity() {
        let mut swapped = baseline();
        swapped.sources.swap(0, 1);
        assert_ne!(identity_of(&swapped), identity_of(&baseline()));
    }

    #[test]
    fn a_framed_value_cannot_impersonate_a_neighbouring_field() {
        // Length framing earns its keep here: a path carrying what looks like the next tag must
        // not produce the same bytes as a shorter path plus that field.
        let mut spoofed = baseline();
        spoofed.sources[1].path = String::from("webhost.sh\nsource-digest 64 ");
        assert_ne!(identity_of(&spoofed), identity_of(&baseline()));
    }
}
