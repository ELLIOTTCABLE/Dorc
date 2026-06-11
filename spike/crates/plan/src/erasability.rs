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
//! # The digest (concl-3, `mechanism-decision-digest`)
//!
//! [`decision_digest`] is a one-line hash of the canonical identity plane, emitted on every
//! analyzer run (cheap always-on drift signal — Zephyr's per-build checksum). It hashes ONLY
//! the identity plane (via the same canon path the gate uses), so a receipt change never moves
//! it. The hash is a hand-rolled FNV-1a (`core` is dependency-free and `DefaultHasher` is not
//! a stable cross-version function); its job is drift-detection, not cryptographic strength.

use dorc_core::Diagnostic;

use crate::{Derivation, Disposition, LicenseVia, Plan, ProbePlan, StandIn, Step};

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

/// A FNV-1a hasher state — a small, fixed, dependency-free, deterministic hash (`core` forbids
/// deps; `std`'s `DefaultHasher` output is explicitly NOT a stable cross-version function, so
/// it cannot back a digest other tools compare). Drift-detection strength, not cryptographic.
struct Fnv1a(u64);

impl Fnv1a {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

/// Build the canonical identity-plane STRING of a whole decision (`plan` + `probe` + their
/// rendered artifacts + diagnostics) — the single source the digest hashes and the gate
/// compares. Two runs are decision-identical iff their canonical strings are byte-equal.
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
    diags: &[Diagnostic],
) -> String {
    let mut out = String::new();
    // (1) the per-site dispositions (the structured decision).
    out.push_str("== plan ==\n");
    for step in &plan.steps {
        out.push_str(&canon_step(step));
        out.push('\n');
    }
    // (2) the probe plan (site-keyed checks + unresolvable list).
    out.push_str("== probe ==\n");
    out.push_str(&canon_probe(probe));
    // (3) the rendered artifacts — byte-exact, comments included (the ru-12 floor). These
    //     subsume much of (1)/(2) but are compared directly: a render bug that left the
    //     structured plane intact would still be caught.
    out.push_str("== render.probe ==\n");
    out.push_str(&probe.render_sh(interner));
    out.push_str("\n== render.apply ==\n");
    out.push_str(&plan.render_apply(src, ast));
    // (4) Error-class diagnostics by (code, site, severity) — sorted for order-independence.
    out.push_str("\n== diags ==\n");
    let mut diag_lines: Vec<String> = diags.iter().filter_map(canon_diag).collect();
    diag_lines.sort();
    for line in diag_lines {
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// The one-line decision digest (`mechanism-decision-digest`): a stable hash of the canonical
/// identity plane, emitted on every analyzer run as a cheap drift signal. Receipt changes
/// cannot move it (the canon plane omits the exempt fields by construction).
#[must_use]
pub fn decision_digest(
    plan: &Plan,
    probe: &ProbePlan,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
    interner: &dorc_core::Interner,
    diags: &[Diagnostic],
) -> String {
    let canon = canonical_decision(plan, probe, src, ast, interner, diags);
    let mut h = Fnv1a::new();
    h.write(canon.as_bytes());
    format!("{:016x}", h.finish())
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
    }
}

/// Canonicalize a [`Derivation`] — EXHAUSTIVE destructure. Today every field is decision-state
/// (fact/via/ambient/grade/verdict), so all are identity. The arch-1 receipt fields land here
/// in the k-cap/witness slice; when they do, each is dropped WITH a named [`Exempt`] reason at
/// this site (`Exempt::ReceiptId` for a `ProvId`/witness, `Exempt::OriginOrdering` for its
/// order). The exhaustive destructure is the gate: a new field will not compile until
/// classified here.
fn canon_derivation(d: &Derivation) -> String {
    let Derivation {
        fact,
        via,
        ambient,
        grade,
        verdict,
    } = d;
    // All identity (decision state). NB the receipt fields (arch-1 k-cap/witness slice) attach
    // here as `Exempt::ReceiptId`/`Exempt::OriginOrdering` and will be OMITTED from this string.
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
    } = f;
    let entity = match entity {
        dorc_core::EntityRef::Operand(t) => format!("op{}", t.0.as_u32()),
        dorc_core::EntityRef::Singleton => "singleton".to_string(),
    };
    format!("k{}#{}@{}", kind.0.as_u32(), selector.0.as_u32(), entity)
}

/// Canonicalize the probe plan — EXHAUSTIVE destructure of [`ProbePlan`] and each [`ProbeCheck`].
/// Every field is decision-identity (the probe artifact's shape): the site key, member index,
/// resolved fact, site-kind firewall discriminant, and the probe-body sh.
fn canon_probe(probe: &ProbePlan) -> String {
    use std::fmt::Write;
    let ProbePlan {
        checks,
        unresolvable,
    } = probe;
    let mut out = String::new();
    for c in checks {
        let crate::ProbeCheck {
            site,
            member,
            fact,
            site_kind,
            sh,
        } = c;
        let _ = writeln!(
            out,
            "check site={} member={member:?} fact={} kind={} sh={sh:?}",
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
fn canon_diag(d: &Diagnostic) -> Option<String> {
    let Diagnostic {
        severity,
        code,
        span,
        message: _, // EXEMPT: Exempt::Explanation — human text, may embed receipt-rendered provenance.
    } = d;
    // Only Error-class diagnostics are on the identity plane (ru-12). A Warning/Note is a
    // disclosure that a receipt-prompted change may legitimately add or vary ⇒ dropped.
    if *severity != dorc_core::Severity::Error {
        return None;
    }
    let site = match span {
        Some(s) => format!("@{}:{}", s.lo.0, s.hi.0),
        None => "@?".to_string(),
    };
    Some(format!("error[{}] {site}", code.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_deterministic_and_stable_for_empty() {
        // The digest is a pure function of the canonical plane; an empty plan/probe/diags
        // hashes to a fixed value (drift-detection baseline). Two calls agree.
        let plan = Plan { steps: vec![] };
        let probe = ProbePlan::default();
        let ast = dorc_syntax::parse("").value;
        let interner = dorc_core::Interner::default();
        let d1 = decision_digest(&plan, &probe, "", &ast, &interner, &[]);
        let d2 = decision_digest(&plan, &probe, "", &ast, &interner, &[]);
        assert_eq!(d1, d2, "digest is deterministic");
        assert_eq!(d1.len(), 16, "16 hex chars (u64)");
    }

    #[test]
    fn canon_drops_non_error_diagnostics() {
        // Only Error-class diagnostics are identity (ru-12). A Note (the common
        // receipt-prompted disclosure shape) is dropped, so a why-lens Note can vary freely
        // without the gate forbidding it.
        let note = Diagnostic::note(dorc_core::DiagCode("x-note"), None, "a disclosure");
        let warn = Diagnostic::warning(dorc_core::DiagCode("x-warn"), None, "a warning");
        let err = Diagnostic::error(
            dorc_core::DiagCode("x-err"),
            Some(dorc_core::Span::new(
                dorc_core::BytePos(1),
                dorc_core::BytePos(2),
            )),
            "an error",
        );
        assert_eq!(canon_diag(&note), None, "Note dropped");
        assert_eq!(canon_diag(&warn), None, "Warning dropped");
        assert_eq!(
            canon_diag(&err),
            Some("error[x-err] @1:2".to_string()),
            "Error keyed by (code, site, severity); message exempt"
        );
    }

    #[test]
    fn canon_diag_message_is_exempt() {
        // Two errors identical in (code, site, severity) but DIFFERENT in message canonicalize
        // identically — the message is Exempt::Explanation (a receipt-rendered why-string must
        // not move the identity plane).
        let span = Some(dorc_core::Span::new(
            dorc_core::BytePos(3),
            dorc_core::BytePos(7),
        ));
        let a = Diagnostic::error(dorc_core::DiagCode("e"), span, "message A (receipt foo)");
        let b = Diagnostic::error(dorc_core::DiagCode("e"), span, "message B (receipt bar)");
        assert_eq!(
            canon_diag(&a),
            canon_diag(&b),
            "differing messages must not perturb the identity plane"
        );
    }
}
