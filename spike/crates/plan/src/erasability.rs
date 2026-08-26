//! The **erasability partition** + decision-digest — the identity/exempt plane split the
//! erasability gate enforces (`Research/plans/22A` concl-2/3, `notes/229` finding-4 +
//! `§0-partition-language`; the round-22 arch-1 contract). The gate itself lives in
//! `tests/erasability.rs`; this module supplies the canonical projection it compares and the
//! always-on digest it emits.
//!
//! # The partition (concl-2, human-ratified ru-12)
//!
//! The analyzer's decision output splits into two planes:
//! * the **identity plane** — must be byte-identical across a normal run and a
//!   receipts-stripped/varied run: the per-site dispositions (`Run`/`Replace`/`Omit` + the
//!   license's decision-relevant fields), the rendered probe/apply artifacts INCLUDING
//!   comments (the ru-12 floor), and Error-class diagnostics keyed by `(code, site,
//!   severity)`;
//! * the **exempt plane** — a CLOSED enumeration of named reasons ([`Exempt`]) a field may
//!   legitimately differ: explanation text, receipt ids, origin ordering, timing.
//!
//! # The mechanism: exhaustive destructuring (include-by-default, the safe direction)
//!
//! LLVM's bias (`notes/229` §0-partition-language): *"do not apply an exempt reason if it
//! isn't clear which is appropriate — an absent location can be detected and fixed, an
//! incorrectly annotated one is much harder."* So a new field must be **included by default**
//! and *deliberately* exempted. We get that without proc-macros (forbidden) by canonicalizing
//! each identity-plane type with an **exhaustive struct/enum destructure — no `..`**: add a
//! field and the canon fn stops compiling until the author classifies it (identity ⇒ fold it
//! into the bytes; exempt ⇒ drop it WITH a named [`Exempt`] reason in a comment). The compiler
//! is the "fails on any unassigned field" gate. Prefer *canonicalize-don't-exempt* (clamp a
//! legitimately-varying field to a deterministic form) over exempting it.
//!
//! # The identity (concl-3, `mechanism-decision-digest`)
//!
//! [`presented_plan_id`] hashes the canonical identity plane into the approval surface's content
//! identity, emitted on every analyzer run (cheap always-on drift signal — Zephyr's per-build
//! checksum). It hashes ONLY the identity plane (via the same canon path the gate uses), so a
//! receipt change never moves it. It is SHA-256, domain-separated, through the receipt crate's
//! one implementation — the same value a receipt records, so a printed identity and a recorded
//! one can never disagree about which surface they name.

use dorc_aid::Diag;

use crate::{Derivation, Disposition, GuardLicense, LicenseVia, Plan, ProbePlan, StandIn, Step};

/// The format tag the canonical decision is domain-separated under.
///
/// Opener and terminator both, because a truncation has to be a different value rather than a
/// shorter complete one — the sibling planner-input encoding, byte for byte.
pub const ENCODING: &str = "dorc-presented-plan/1";

/// See [`ENCODING`].
pub const TERMINATOR: &str = "decision-end";

/// The CLOSED set of reasons a field is on the **exempt** plane (`22A` concl-2 / ru-12;
/// modelled on LLVM's named `DebugLoc` absence-reasons). Extend DELIBERATELY: a new reason is
/// a conscious widening of what may differ between a normal and a receipts-varied run. Used as
/// documentation-at-the-definition-site in the canon fns below (each exempted field cites its
/// reason); it is not stored, because the canonical bytes simply OMIT exempt fields.
///
/// The governing bias (`notes/229` §0-partition-language): when unsure, a field is NOT exempt
/// (included-by-default — a spurious identity-diff is loud-but-fixable; a wrongly-exempted
/// leak is silent). Prefer canonicalize-don't-exempt where a field legitimately varies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exempt {
    /// Human-facing explanation text (a diagnostic `message`, a why-lens string). Varies with
    /// receipts by design; never a decision.
    Explanation,
    /// A receipt id ([`dorc_core::ProvId`]) or a structure reachable only through one. The
    /// WELD's payload — exempt by definition.
    ReceiptId,
    /// The ORDER of an origin set / join-parent witness. Receipts may reorder; decisions must
    /// not depend on it (`22A` concl-4).
    OriginOrdering,
    /// A timing / counter / arena-size value (wall-clock, a monotonic id, the arena's `len`).
    Timing,
}

/// Build the canonical identity-plane STRING of a whole decision (`plan` + `probe` + their
/// rendered artifacts + diagnostics) — the single source the digest hashes and the gate
/// compares. Two runs are decision-identical iff their canonical strings are byte-equal.
///
/// LENGTH-FRAMED, on the sibling planner-input encoding: every component declares its byte length,
/// so no component can reach across into a neighbour whatever bytes it carries. That matters here
/// more than anywhere, because two components are the rendered artifacts and the apply render is
/// the BOOK verbatim — under a separator-delimited encoding a book line spelling a component tag
/// moved the boundary, and two different decisions could present one identity. This identity is
/// never authority, but it IS recompute-and-compare: a collision is a confident wrong answer to
/// "is this the same plan", which is the one thing `271:rul-sin-ordering` ranks worst.
///
/// Deterministic by construction (`inv-determinism`): every component is appended in a fixed
/// order; nothing iterates a hashed collection. `src`/`ast` resolve the rendered artifacts (the
/// ru-12 byte-floor, comments included); `diags` are the analyzer's accumulated diagnostics
/// (only Error-class ones, by `(code, site, severity)`, are identity — see [`canon_diag`]).
#[must_use]
pub fn canonical_decision(
    plan: &Plan,
    probe: &ProbePlan,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
    interner: &dorc_core::Interner,
    diags: &[Diag],
) -> String {
    // EXHAUSTIVE, as [`canon_step`] is: field-by-field reads caught a new `Plan` field only when it
    // happened to move rendered bytes (`30Nd:fnd-the-canon-does-not-destructure-plan`).
    let Plan {
        steps,
        regions,
        // EXEMPT (Exempt::Timing): 24F §3a instrumentation, never a decision.
        survival_report: _,
        // EXEMPT as DERIVED: a pure function of the fields above plus `(src, ast)` and the
        // defensive-emission input, all of which reach the byte-exact `render.apply` at (3).
        render: _,
        // EXEMPT: influence is causal accounting ORTHOGONAL to authority (`306b` §10), ruled
        // non-durable at `ExcludedContent::InfluenceGrade`. Two runs that decided identically from
        // differently-standing inputs DO reproduce identically, which is what the digest asks.
        account: _,
    } = plan;
    // (1) the per-site dispositions (the structured decision).
    let mut sites = String::new();
    for step in steps {
        sites.push_str(&canon_step(step));
        sites.push('\n');
    }
    // (1a) the SHARED region decisions, ABSENT rather than empty where the book has none — so a
    // book with no eligible calls stays distinguishable from one whose regions all vanished
    // (`30L:pin-empty-function-world-parity`).
    let shared = if regions.is_empty() {
        None
    } else {
        let mut text = String::new();
        for region in regions {
            text.push_str(&canon_region(region));
            text.push('\n');
        }
        Some(text)
    };
    // (2) the probe plan (site-keyed checks + unresolvable list).
    let probe_plane = canon_probe(probe);
    // (3) the rendered artifacts — byte-exact, comments included (the ru-12 floor). These
    //     subsume much of (1)/(2) but are compared directly: a render bug that left the
    //     structured plane intact would still be caught. A canonical differential form: the
    //     framing is fixed (spike default) so two renders of the same probe compare
    //     byte-identically; the digest is irrelevant to the comparison.
    let rendered_probe = probe.render_sh(&crate::records::Framing::spike(String::new()), interner);
    let rendered_apply = plan.render_apply(src, ast);
    // (4) Error-class diagnostics by (code, site, severity) — sorted for order-independence.
    let mut diag_lines: Vec<String> = diags.iter().filter_map(canon_diag).collect();
    diag_lines.sort();
    let mut diagnostics = String::new();
    for line in diag_lines {
        diagnostics.push_str(&line);
        diagnostics.push('\n');
    }

    let mut out = String::new();
    out.push_str(ENCODING);
    out.push('\n');
    put_str(&mut out, "plan", &sites);
    match &shared {
        Some(text) => put_str(&mut out, "regions", text),
        None => put_absent(&mut out, "regions"),
    }
    put_str(&mut out, "probe", &probe_plane);
    put_str(&mut out, "render.probe", &rendered_probe);
    put_str(&mut out, "render.apply", &rendered_apply);
    put_str(&mut out, "diags", &diagnostics);
    out.push_str(TERMINATOR);
    out.push('\n');
    out
}

/// `<tag> <byte-length> <exact bytes>\n` — the sibling planner-input encoding, byte for byte.
///
/// The declared length is the whole point: the rendered artifacts carry the BOOK verbatim, so a
/// component that ended at a separator would end wherever a book chose to spell one, and the split
/// between one component and the next would stop being recoverable from the bytes.
fn put_str(out: &mut String, tag: &str, value: &str) {
    out.push_str(tag);
    out.push(' ');
    out.push_str(&value.len().to_string());
    out.push(' ');
    out.push_str(value);
    out.push('\n');
}

/// `<tag> absent\n` — never an empty value standing in for absence.
fn put_absent(out: &mut String, tag: &str) {
    out.push_str(tag);
    out.push_str(" absent\n");
}

/// Mint the approval surface's content identity from the canonical identity plane
/// (`quarantine/30Rb:receipt-identity-map`).
///
/// This is the ONE identity path: the drift signal the analyzer prints and the identity a
/// receipt records are the same value, so a printed identity and a recorded one can never
/// disagree about which surface they name.
///
/// The seat is what licenses the mint. Every input reaching [`canonical_decision`] is final —
/// the `Plan` comes from its one constructor, after settlement has quiesced and the certifier
/// latch is spent, and the canon reads the rendered artifacts, so the human view, the executable
/// bytes, and every site and region decision are settled before a byte is hashed.
///
/// Receipt changes cannot move it: the canon plane omits the exempt fields by construction.
#[must_use]
pub fn presented_plan_id(
    plan: &Plan,
    probe: &ProbePlan,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
    interner: &dorc_core::Interner,
    diags: &[Diag],
) -> dorc_receipt::ids::PresentedPlanId {
    let canon = canonical_decision(plan, probe, src, ast, interner, diags);
    dorc_receipt::ids::PresentedPlanId::of_canonical_decision(canon.as_bytes())
}

/// Canonicalize one plan [`Step`] — EXHAUSTIVE destructure (no `..`): a new field stops this
/// compiling until classified identity-or-exempt. `leaf`/`ast`/`sh` are identity (the stable
/// back-map + the verbatim leaf text); `disposition` is projected by [`canon_disposition`].
fn canon_step(step: &Step) -> String {
    let Step {
        leaf,
        ast,
        sh,
        disposition,
    } = step;
    format!(
        "leaf={} ast={} sh={sh:?} {}",
        leaf.0,
        ast.0,
        canon_disposition(disposition)
    )
}

/// Canonicalize one [`RegionStep`] — EXHAUSTIVE destructure, same rule as [`canon_step`].
///
/// `region`/`ast`/`sh` are edit identity; `disposition` is the one shared decision. `routes` is the
/// contributing-invocation ACCOUNT: attribution the why plane reads, and already narrower than the
/// census by construction (`SpineRegionDecision::routes`), so it is exempt on the same footing as a
/// license's witness — it names WHO licensed the edit, never WHAT the edit is.
fn canon_region(region: &crate::RegionStep) -> String {
    let crate::RegionStep {
        region,
        ast,
        sh,
        disposition,
        routes: _, // EXEMPT: Exempt::ReceiptId — contributing-route attribution, output-only.
    } = region;
    format!(
        "region={:?} ast={} sh={sh:?} {}",
        region.span(),
        ast.0,
        canon_disposition(disposition)
    )
}

/// Canonicalize a [`Disposition`] — EXHAUSTIVE match (every variant): the run/replace/omit
/// decision and its decision-relevant payload are identity; the license's [`Derivation`] is
/// the exempt receipt holder (see [`canon_derivation`]).
fn canon_disposition(d: &Disposition) -> String {
    match d {
        Disposition::Run => "Run".to_string(),
        Disposition::Replace(license, stand_in) => {
            // The license's fact + via are decision-identity; its `Derivation` is projected
            // (its receipt-bearing fields are EXEMPT). The stand-in is the value-preserving
            // substitution — identity (it is rendered into the artifact).
            format!(
                "Replace via={} fact={} standin={} {}",
                canon_via(license.derivation().via),
                canon_fact(license.fact()),
                canon_standin(*stand_in),
                canon_derivation(license.derivation()),
            )
        }
        Disposition::Omit { controller } => format!("Omit controller={}", controller.0),
        // A guard's identity is its fact + the emitter code (funcname/invocation/sense/preamble);
        // its attribution overlay is EXEMPT (`GuardInsert::canonical` drops it), so a plan
        // differing only in guard attribution digests identically (24D §2).
        Disposition::Guard(license) => {
            let GuardLicense {
                fact,
                insert,
                // EXEMPT (Exempt::ReceiptId + Exempt::Timing): a why-chain row, decided-without.
                probe: _,
                // EXEMPT, on the `Plan::account` reasoning above.
                account: _,
            } = license;
            format!("Guard fact={} {}", canon_fact(*fact), insert.canonical())
        }
    }
}

/// Canonicalize a [`Derivation`] — EXHAUSTIVE destructure. `fact`/`via`/`ambient`/`grade`/
/// `verdict` are decision-state ⇒ identity. `witness` is the arch-1 full granted witness
/// (`vp-17`/`vp-18`): EXEMPT — [`Exempt::ReceiptId`] (the origins themselves) compounded with
/// [`Exempt::OriginOrdering`] (their order) — so it is OMITTED here. This is the load-bearing
/// exemption the gate's adversarial run-B exercises: run-B's witness holds different sentinel
/// `ProvId`s in reversed order, and because this fn drops it, the canonical decision is
/// unchanged. The exhaustive destructure is the gate: a future field will not compile until
/// classified identity-or-exempt.
fn canon_derivation(d: &Derivation) -> String {
    let Derivation {
        fact,
        via,
        ambient,
        grade,
        verdict,
        witness: _, // EXEMPT: Exempt::ReceiptId + Exempt::OriginOrdering — the full granted
        // witness is output-only provenance; the adversarial gate proves it inert.
        survival: _, // EXEMPT (Stage 2 / TC-3): the survival witness is render-surface
        // attribution (the why-lens), NEVER woven into the byte-floored artifact
        // (rec-1). A survived elision's ARTIFACT bytes are identical to any other
        // elision's (both render the StandIn); only its disposition (Replace vs
        // Run) is identity, and that is already hashed. So the attribution detail
        // is output-only, like `witness`.
        vouch_span: _, // EXEMPT (C7): the vouch's `file:line` is why-lens attribution only (a
        // vouch informs, never becomes a fact — TC-tier-3), like `witness`/`survival`.
        establish_vouches: _, // EXEMPT: aggregate vouch receipts are narration-only.
        probe: _, // EXEMPT (Exempt::ReceiptId + Exempt::Timing): a why-chain row. Its tool-rc is
                  // CARRIED, never consumed — the fold read its own admissible copy pre-mint.
    } = d;
    format!(
        "deriv(fact={} via={} ambient={ambient} grade={grade:?} verdict={verdict:?})",
        canon_fact(*fact),
        canon_via(*via),
    )
}

/// Canonicalize a [`LicenseVia`] — EXHAUSTIVE match (the substitution path is decision-identity).
fn canon_via(via: LicenseVia) -> &'static str {
    match via {
        LicenseVia::ConvergedEstablish => "ConvergedEstablish",
        LicenseVia::QueryGuard => "QueryGuard",
        LicenseVia::MembersLoop => "MembersLoop",
        LicenseVia::InlineCall => "InlineCall",
        LicenseVia::SharedRegion => "SharedRegion",
    }
}

/// Canonicalize a [`StandIn`] — EXHAUSTIVE match (the rendered substitution bytes are identity).
fn canon_standin(s: StandIn) -> String {
    match s {
        StandIn::True => "true".to_string(),
        StandIn::False => "false".to_string(),
        StandIn::Exit(n) => format!("exit{n}"),
    }
}

/// Canonicalize a [`dorc_core::FactKey`] — EXHAUSTIVE destructure. The whole cell coordinate is
/// decision-identity (it is what a license keys on). Interned symbols are rendered by their raw
/// id (stable within one run's interner; the gate's two runs share an interner per run, and the
/// comparison is intra-pair — the symbol-id space is identical for run-A and run-B because the
/// interner is fed identically). Referent-agnostic: the id is provenance/identity, not decoded.
fn canon_fact(f: dorc_core::FactKey) -> String {
    let dorc_core::FactKey {
        kind,
        entity,
        selector,
        context,
    } = f;
    let entity = match entity {
        dorc_core::EntityRef::Operand(t) => format!("op{}", t.0.as_u32()),
        dorc_core::EntityRef::Singleton => "singleton".to_string(),
    };
    // The context is decision-identity too (`27C` §3): a wrapped fact is a DIFFERENT cell-in-world,
    // so it must digest distinctly. `HostDefault` renders empty — a wrapper-free run's digest is
    // byte-identical to the pre-`27C` three-place form (`empty-world-byte-identical`).
    let ctx = match context {
        dorc_core::Context::HostDefault => String::new(),
        dorc_core::Context::Wrapped(k) => format!("~ctx{}", k.0.as_u32()),
    };
    format!(
        "k{}@{}@{}{ctx}",
        kind.0.as_u32(),
        selector.0.as_u32(),
        entity
    )
}

/// Canonicalize the probe plan — EXHAUSTIVE destructure of [`ProbePlan`] and each [`ProbePredict`].
/// Every field is decision-identity (the probe artifact's shape): the site key, member index,
/// resolved fact, site-kind firewall discriminant, and the probe-body sh.
fn canon_probe(probe: &ProbePlan) -> String {
    use std::fmt::Write;
    let ProbePlan {
        checks,
        unresolvable,
        // EXEMPT (Exempt::Explanation): WHY a site could not be probed. `unresolvable` above is the
        // identity — the same sites, the same `unresolvable-no-probe` comments — and the cause is
        // read only by the stderr note. Digesting it would make a reason-wording change read as an
        // artifact change, the inverse of what this canon exists to prove.
        unresolvable_causes: _,
    } = probe;
    let mut out = String::new();
    for c in checks {
        let crate::ProbePredict {
            site,
            member,
            fact,
            site_kind,
            provider,
            argv,
            sh,
            defining_span: _, // EXEMPT (Exempt::Explanation): a speaker label. The BODY (`sh`) is
            // identity above; where it was sliced from changes no emitted byte.
            connected,
            verdict,
            entry,
            emits_report,
        } = c;
        let _ = writeln!(
            out,
            "check site={} member={member:?} fact={} kind={} provider={provider:?} argv={argv:?} sh={sh:?} connected={connected:?} verdict={verdict:?} entry={entry:?} emits_report={emits_report:?}",
            site.0,
            canon_fact(*fact),
            canon_site_kind(*site_kind),
        );
    }
    for u in unresolvable {
        let _ = writeln!(out, "unresolvable site={}", u.0);
    }
    out
}

/// Canonicalize a [`crate::ProbeSiteKind`] — EXHAUSTIVE match (the firewall discriminant is identity).
fn canon_site_kind(k: crate::ProbeSiteKind) -> String {
    match k {
        crate::ProbeSiteKind::Establish => "Establish".to_string(),
        crate::ProbeSiteKind::Query { valid } => format!("Query(valid={valid})"),
    }
}

/// Canonicalize a [`Diagnostic`] to its identity tuple `(severity, code, site)` — or `None` to
/// DROP it from the identity plane. EXHAUSTIVE destructure: a new field must be classified here.
///
/// The partition (ru-12 / `22A` concl-2): only **Error-class** diagnostics are identity, keyed
/// by `(code, span, severity)`. The `message` is EXEMPT — [`Exempt::Explanation`] (it embeds
/// receipt-rendered text). Warnings/Notes are disclosures, not decisions, so they are dropped
/// entirely (a receipt-prompted Note must be free to appear/vary — the gate would otherwise
/// forbid the why-lens). A span is rendered by its byte coordinates (the stable site); `None`
/// span renders as `@?`.
fn canon_diag(d: &Diag) -> Option<String> {
    // Only Error-class diagnostics are on the identity plane (ru-12). A Warning/Note is a
    // disclosure that a receipt-prompted change may legitimately add or vary ⇒ dropped. The
    // catalog-rendered MESSAGE is EXEMPT (Exempt::Explanation): identity keys on the code slug,
    // the primary span, and the severity only.
    if d.severity() != dorc_aid::Severity::Error {
        return None;
    }
    let site = match d.primary.span() {
        Some(s) => format!("@{}:{}", s.lo.0, s.hi.0),
        None => "@?".to_string(),
    };
    Some(format!("error[{}] {site}", d.code.slug()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_deterministic_and_stable_for_empty() {
        // The digest is a pure function of the canonical plane; an empty plan/probe/diags
        // hashes to a fixed value (drift-detection baseline). Two calls agree.
        let probe = ProbePlan::default();
        let ast = dorc_syntax::parse("").value;
        let plan = Plan::decided(
            vec![],
            Vec::new(),
            crate::SurvivalReport::default(),
            false,
            crate::NO_ARTIFACT_FORM,
            "",
            &ast,
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
        let interner = dorc_core::Interner::default();
        let d1 = presented_plan_id(&plan, &probe, "", &ast, &interner, &[]);
        let d2 = presented_plan_id(&plan, &probe, "", &ast, &interner, &[]);
        assert_eq!(d1, d2, "one surface, one identity");
        assert_eq!(d1.hex().len(), 64, "SHA-256, hex-spelled");
    }

    #[test]
    fn canon_drops_non_error_diagnostics() {
        use dorc_aid::diag::{
            CfgBuiltinShadowed, DiagCode, RedirTargetTop, SiteId, SyntaxMalformed,
            SyntaxMalformedReason,
        };
        // Only Error-class diagnostics are identity (ru-12). A Note (RedirTargetTop) and a
        // Warning (CfgBuiltinShadowed) are dropped; the Error (SyntaxMalformed) keys on
        // (slug, span, severity), its rendered message exempt.
        let note = Diag::new(
            DiagCode::RedirTargetTop(RedirTargetTop {
                site: SiteId::leaf(dorc_core::LeafId(0)),
            }),
            dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
        );
        let warn = Diag::new(
            DiagCode::CfgBuiltinShadowed(CfgBuiltinShadowed {
                name: "cd".to_owned(),
            }),
            dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
        );
        let err = Diag::new(
            DiagCode::SyntaxMalformed(SyntaxMalformed {
                reason: SyntaxMalformedReason::ExpectedFiToCloseIf,
            }),
            dorc_core::Span::new(dorc_core::BytePos(1), dorc_core::BytePos(2)),
        );
        assert_eq!(canon_diag(&note), None, "Note dropped");
        assert_eq!(canon_diag(&warn), None, "Warning dropped");
        assert_eq!(
            canon_diag(&err),
            Some("error[syntax-malformed] @1:2".to_string()),
            "Error keyed by (code, site, severity); message exempt"
        );
    }

    #[test]
    fn canon_diag_message_is_exempt() {
        use dorc_aid::diag::{DiagCode, SyntaxMalformed, SyntaxMalformedReason};
        // Two errors identical in (code, span, severity) but DIFFERENT in payload detail (⇒
        // different rendered message) canonicalize identically — the message is
        // Exempt::Explanation (identity keys on slug/span/severity only).
        let span = dorc_core::Span::new(dorc_core::BytePos(3), dorc_core::BytePos(7));
        let a = Diag::new(
            DiagCode::SyntaxMalformed(SyntaxMalformed {
                reason: SyntaxMalformedReason::UnterminatedSubshell,
            }),
            span,
        );
        let b = Diag::new(
            DiagCode::SyntaxMalformed(SyntaxMalformed {
                reason: SyntaxMalformedReason::UnterminatedBraceGroup,
            }),
            span,
        );
        assert_eq!(
            canon_diag(&a),
            canon_diag(&b),
            "differing messages must not perturb the identity plane"
        );
    }
}

#[cfg(test)]
mod framing {
    use crate::{NO_ARTIFACT_FORM, Plan, ProbePlan, SurvivalReport};

    /// One canonical decision over a book carrying `body`.
    fn canon_over(body: &str) -> String {
        let ast = dorc_syntax::parse(body).value;
        let plan = Plan::decided(
            vec![],
            Vec::new(),
            SurvivalReport::default(),
            false,
            NO_ARTIFACT_FORM,
            body,
            &ast,
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
        super::canonical_decision(
            &plan,
            &ProbePlan::default(),
            body,
            &ast,
            &dorc_core::Interner::default(),
            &[],
        )
    }

    /// Walk a canonical decision by its DECLARED LENGTHS, answering each component's exact bytes.
    ///
    /// A reader rather than an assertion, because recoverability IS the property: if this can take
    /// the string apart into exactly the components that were put in, then no component reached
    /// into a neighbour, whatever bytes it carried.
    fn walk(canon: &str) -> Vec<(String, Option<String>)> {
        let mut out: Vec<(String, Option<String>)> = Vec::new();
        let body = canon
            .strip_prefix(super::ENCODING)
            .and_then(|rest| rest.strip_prefix('\n'))
            .expect("the encoding opens with its own tag");
        let mut cursor = body;
        loop {
            if let Some(rest) = cursor.strip_prefix(super::TERMINATOR) {
                assert_eq!(rest, "\n", "the terminator is followed by end of value");
                return out;
            }
            let (tag, rest) = cursor
                .split_once(' ')
                .expect("a component opens with its tag");
            if let Some(rest) = rest.strip_prefix("absent\n") {
                out.push((tag.to_owned(), None));
                cursor = rest;
                continue;
            }
            let (declared, rest) = rest
                .split_once(' ')
                .expect("a component declares its length");
            let length: usize = declared.parse().expect("a canonical decimal length");
            let value = rest.get(..length).expect("the declared length is present");
            let after = rest.get(length..).expect("bytes past the value");
            assert_eq!(
                after.as_bytes().first(),
                Some(&b'\n'),
                "the framing newline sits exactly at the declared length"
            );
            out.push((tag.to_owned(), Some(value.to_owned())));
            cursor = after.get(1..).expect("bytes past the framing newline");
        }
    }

    #[test]
    fn a_canonical_decision_takes_apart_into_exactly_the_components_the_table_names() {
        // The control for the case below: an ordinary book walks cleanly, so a failure there is
        // about the spoofing bytes rather than about the walk.
        let walked = walk(&canon_over("echo hi\n"));
        let tags: Vec<&str> = walked.iter().map(|(tag, _)| tag.as_str()).collect();
        let expected: Vec<&str> = crate::identity_table::PRESENTED_PLAN_COMPONENTS
            .iter()
            .map(|row| row.tag)
            .collect();
        assert_eq!(tags, expected);
        // The empty book has no shared regions, and that is spelled absent rather than empty.
        assert_eq!(walked.get(1).map(|(_, value)| value.clone()), Some(None));
    }

    #[test]
    fn a_book_line_spelling_a_component_cannot_move_a_boundary() {
        // The repair, measured. Before length framing this book put a SECOND `== diags ==` header
        // into the canon and the split between the apply render and the diagnostics stopped being
        // recoverable — two different decisions could present one identity, which for an identity
        // that is recompute-and-compared is a confident wrong answer to "is this the same plan".
        //
        // Now the spoofing bytes are inside the render component's declared length, and the walk
        // proves it: the component count is unchanged and the line is where the book put it.
        let spoofing = "echo hi\n== diags ==\ndiags 0 \n";
        let walked = walk(&canon_over(spoofing));
        assert_eq!(
            walked.len(),
            crate::identity_table::PRESENTED_PLAN_COMPONENTS.len(),
            "the spoofing bytes did not add a component"
        );
        let render = walked
            .iter()
            .find(|(tag, _)| tag == "render.apply")
            .and_then(|(_, value)| value.clone())
            .expect("the apply render is present");
        assert!(
            render.contains("== diags ==\n"),
            "the book's own line stayed inside the render"
        );
        assert!(
            render.contains("diags 0 \n"),
            "including a line spelling this encoding's own component form"
        );
        let diagnostics = walked
            .iter()
            .find(|(tag, _)| tag == "diags")
            .and_then(|(_, value)| value.clone())
            .expect("the diagnostics component is present");
        assert!(
            diagnostics.is_empty(),
            "and the diagnostics component is the empty one it should be, not the book's bytes"
        );
    }

    #[test]
    fn two_books_differing_only_where_a_boundary_would_have_fallen_are_two_identities() {
        // The consequence, stated as identities rather than as bytes. These two differ only in
        // whether the spoofing line is present, which is exactly the difference a delimiter-ended
        // component could have swallowed.
        assert_ne!(
            super::presented_plan_id(
                &Plan::decided(
                    vec![],
                    Vec::new(),
                    SurvivalReport::default(),
                    false,
                    NO_ARTIFACT_FORM,
                    "echo hi\n",
                    &dorc_syntax::parse("echo hi\n").value,
                    dorc_core::influence::InfluenceAccount::authored_before_contact(),
                ),
                &ProbePlan::default(),
                "echo hi\n",
                &dorc_syntax::parse("echo hi\n").value,
                &dorc_core::Interner::default(),
                &[],
            ),
            super::presented_plan_id(
                &Plan::decided(
                    vec![],
                    Vec::new(),
                    SurvivalReport::default(),
                    false,
                    NO_ARTIFACT_FORM,
                    "echo hi\n== diags ==\n",
                    &dorc_syntax::parse("echo hi\n== diags ==\n").value,
                    dorc_core::influence::InfluenceAccount::authored_before_contact(),
                ),
                &ProbePlan::default(),
                "echo hi\n== diags ==\n",
                &dorc_syntax::parse("echo hi\n== diags ==\n").value,
                &dorc_core::Interner::default(),
                &[],
            ),
        );
    }
}
