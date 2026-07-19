//! The per-code **defining cases** (`27V` §3 · `AID-NEEDS:law-one-defining-case-per-code`): every
//! catalog code has ONE defining case pinning its colocated TRIPLE render — machine line · terse
//! line · prose registers — byte-for-byte, so a template/payload change fails loud and re-blesses
//! consciously (`goldens-churn-freely`; the particulars ride `27V:rul-output-form-unwelded`).
//!
//! # Two-half regime (`tc-defining-case-triple-render-siting`, conductor-accepted d4b)
//!
//! A defining case is TWO colocated halves:
//! * **fires** — that the code can actually be EMITTED is pinned by
//!   `diag_tidy::every_catalog_variant_is_constructed` (the production emit-site grep): delete a
//!   code's sole emit and that gate fails. This is the "a defining case that stops triggering fails
//!   loud" half, at emit-site granularity — no per-code trigger book is needed here.
//! * **renders** — THIS harness byte-compares the three renders of a canonical payload against the
//!   colocated goldens under `tests/defining_cases/<slug>.{machine,terse,prose}`.
//!
//! Siting rationale (one line, `27V` §3 build-graph hygiene): unit-tier, NOT 22 new e2e dirs — the
//! e2e `expected.out` golden captures the PLAN render, not the isolated per-code triple, and new e2e
//! dirs would need an orchestrator BLESS this builder cannot run; the unit harness is self-contained,
//! bless-free (`DORC_DEFINING_BLESS=1` regenerates the goldens), and byte-asserts the real deliverable.
//!
//! # Coverage: RATCHET (`tc-defining-case-coverage-ratchet`, conductor-accepted)
//!
//! The corpus asserts only ~5 codes by identity today; a full 52-case sweep is disproportionate for
//! one dispatch (the expensive tail is the `records-*` corruption fixtures + the `whylog-*` durables).
//! [`DEFINING_CASE_RATCHET`] is the SHRINK-ONLY allowlist of not-yet-covered codes; the completeness
//! gate is `covered ∪ ratchet == every catalog slug`. Every ratchet entry carries a one-line
//! corruption-injection surface so future coverage is mechanical, not re-derived (conductor rider).

use dorc_core::diag::{
    self, CarriedAcrossSubstrateAxis, CmdsubOperandTop, DanglingReference, Diag, DiagCode,
    EscalationPolicy, MissingDialectMarker, MungeNameInvalid, OperandPosition,
    RecordsFactTruncated, RenderHeredocRefused, SiteId, SiteUnresolvable, SyntaxUnsupported,
    ToleratesUnknownDimension, WhylogAbsent, WhylogBookDesync, WhylogCorrupt, WhylogVersionRefused,
    WrapperPeelIncoherent,
};
use dorc_core::{BytePos, Interner, LeafId, Severity, Span, TopCause};

/// A defining case: the code's stable slug + a constructor for its CANONICAL payload (fixed values so
/// the renders are deterministic — `inv-determinism`).
struct DefiningCase {
    slug: &'static str,
    build: fn() -> DiagCode,
}

/// The COVERED codes (`tc-defining-case-coverage-ratchet`): one canonical payload each, chosen to
/// span the payload species (templated / passthrough / static / conductor-authored-prose). Add a code
/// here + drop it from [`DEFINING_CASE_RATCHET`] to grow coverage (the ratchet only shrinks).
#[expect(
    clippy::too_many_lines,
    reason = "one struct literal per covered code — the case table is inherently long and stays \
              scannable; splitting it would scatter the defining cases"
)]
fn covered() -> Vec<DefiningCase> {
    vec![
        DefiningCase {
            slug: "cmdsub-operand-top",
            build: || {
                DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                    site: SiteId::leaf(LeafId(3)),
                    position: OperandPosition::Operand(1),
                    cause: None,
                    top_cause: TopCause::UnmodeledExpansion,
                })
            },
        },
        DefiningCase {
            slug: "site-unresolvable",
            build: || {
                DiagCode::SiteUnresolvable(SiteUnresolvable {
                    site: SiteId::leaf(LeafId(4)),
                    detail: "2 sites run unprobed (no read-only check could be shipped): \
                             `make install`, `ldconfig`"
                        .to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "render-heredoc-refused",
            build: || {
                DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                    site: SiteId::leaf(LeafId(7)),
                    verb: "elide",
                    command: "cat <<EOF".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "syntax-unsupported",
            build: || {
                DiagCode::SyntaxUnsupported(SyntaxUnsupported {
                    detail: "process substitution `<(…)` is not modeled".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "missing-dialect-marker",
            build: || DiagCode::MissingDialectMarker(MissingDialectMarker),
        },
        DefiningCase {
            slug: "munge-name-invalid",
            build: || {
                DiagCode::MungeNameInvalid(MungeNameInvalid {
                    source: "9pkg".to_owned(),
                    funcname: "9pkg".to_owned(),
                    problem: "starts with a digit".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "tolerates-unknown-dimension",
            build: || {
                DiagCode::ToleratesUnknownDimension(ToleratesUnknownDimension {
                    token: "netns2".to_owned(),
                    expected: "user, netns, fs-view".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "records-fact-truncated",
            build: || {
                DiagCode::RecordsFactTruncated(RecordsFactTruncated {
                    received: 3,
                    declared: 5,
                    unseen: 2,
                })
            },
        },
        DefiningCase {
            slug: "dangling-reference",
            build: || {
                DiagCode::DanglingReference(DanglingReference {
                    coord: "sm.dorc.Package:nginx".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "escalation-policy",
            build: || {
                DiagCode::EscalationPolicy(EscalationPolicy {
                    detail: "escalation policy: probe re-uses connection authority for \
                             `tolerates:`-vouched functions only (default)"
                        .to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "carried-across-substrate-axis",
            build: || {
                DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                    detail: "elision carried across the fs-view axis: backing kind `sm_dorc_File` \
                             vouches `invariant:fs-view`; the verdict body is read-set-closed"
                        .to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "wrapper-peel-incoherent",
            build: || {
                DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
                    detail: "wrapper `sudo`: __predict and __lend_map disagree on the peel tail \
                             position (predict reaches \"$@\" after 1 argv token(s), lend_map after 0)"
                        .to_owned(),
                })
            },
        },
        // The conductor-authored-prose quartet (`27V` Lane B): their prose is the third catalog
        // state (unprefixed, rostered), so the defining case pins the AUTHORED render byte-for-byte.
        DefiningCase {
            slug: "whylog-version-refused",
            build: || {
                DiagCode::WhylogVersionRefused(WhylogVersionRefused {
                    found: "dorc-whylog/2".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "whylog-book-desync",
            build: || {
                DiagCode::WhylogBookDesync(WhylogBookDesync {
                    which: "book".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "whylog-absent",
            build: || {
                DiagCode::WhylogAbsent(WhylogAbsent {
                    dir: "./.dorc/whylog".to_owned(),
                })
            },
        },
        DefiningCase {
            slug: "whylog-corrupt",
            build: || {
                DiagCode::WhylogCorrupt(WhylogCorrupt {
                    detail: "no end-sentinel — a partial write?".to_owned(),
                })
            },
        },
    ]
}

/// The SHRINK-ONLY not-yet-covered allowlist (`tc-defining-case-coverage-ratchet`). Each entry's note
/// is the corruption/trigger surface a future defining case injects, so coverage is mechanical
/// (conductor rider). A code may leave this list ONLY by gaining a `covered()` case; adding a NEW code
/// without a defining case must add it here with its surface note (the completeness gate enforces the
/// partition, `ratchet_only_shrinks` guards the direction against the committed baseline).
const DEFINING_CASE_RATCHET: &[(&str, &str)] = &[
    // ── analyzer give-ups: trigger via a small book/oracle through the pipeline ──
    (
        "cmdsub-inner-nonleaf",
        "book: an effect command inside `$(…)`, e.g. `x=$(id -u)`",
    ),
    (
        "redir-target-top",
        "book: a write-redirect to a dynamic target, `echo x >\"$f\"`",
    ),
    (
        "depth-2-positional-unthreaded",
        "book: a depth-2 inlined call passing `$1` through two levels",
    ),
    (
        "syntax-malformed",
        "book: a parse error, e.g. an unterminated double-quote",
    ),
    (
        "cfg-top-node",
        "book: an unsupported construct that lowers to a CFG ⊤ node",
    ),
    (
        "cfg-errexit-unknown",
        "book: `set -e` with an unmodeled command in the errexit region",
    ),
    (
        "cfg-inline-refused",
        "book: a recursive/over-budget funcdef call the inliner refuses",
    ),
    (
        "cfg-builtin-shadowed",
        "book: a funcdef named `test`/`[` shadowing a relied-on builtin",
    ),
    (
        "effect-kind-disagreement",
        "oracle: a check annotation kind ≠ the effect-map kind for a verb",
    ),
    (
        "predict-out-of-dialect",
        "oracle: a check body using a backtick / `[[ ]]` (out of dialect)",
    ),
    (
        "predict-unterminated",
        "oracle: a check `case` body missing its `esac`",
    ),
    (
        "munge-name-collision",
        "oracle: two source names munging to one sh funcname",
    ),
    (
        "reserved-namespace-squat",
        "book: a funcdef named `nginx__predict` squatting the role namespace",
    ),
    (
        "tolerates-over-identity-dependence",
        "oracle: `tolerates:user` over a body reading `id`/`$USER`",
    ),
    (
        "heavy-context-no-tolerance",
        "oracle: an is_converged reading identity with no tolerance vouch",
    ),
    (
        "lend-map-unknown-dimension",
        "oracle: a `__lend_map` line with an unknown dimension token",
    ),
    (
        "carry-netns-on-net-kernel-forbidden",
        "oracle: `invariant:netns` on a per-netns net-kernel store",
    ),
    (
        "mark-brace-verdict-single-cell",
        "oracle: a brace-alternation `#{a,b}` on a verdict/observe mark",
    ),
    (
        "footprint-incoherent",
        "oracle: a touches() footprint omitting its own effect coordinate",
    ),
    (
        "touches-escalated",
        "book+flag: a payload-bound touches() escalating to host-derivation",
    ),
    (
        "deriv-family-incomplete",
        "probe-results: a derived footprint family missing its deriv-end record",
    ),
    (
        "wrapped-site-adoption-hint",
        "book: a `sudo`-wrapped site whose is_converged lacks tolerates",
    ),
    (
        "resolver-conflict",
        "oracle: two oracle files declaring one kind's resolver",
    ),
    (
        "resolver-provider-collision",
        "oracle: a resolver keyed to a known COMMAND provider name",
    ),
    (
        "reaches-conflict",
        "oracle: two oracle files declaring one kind's reach-function",
    ),
    (
        "reaches-provider-collision",
        "oracle: a reach-function keyed to a known COMMAND provider name",
    ),
    (
        "wrapper-entry-incoherent",
        "oracle: a wrapper whose __enter and __lend_map disagree on argv flow",
    ),
    // ── records deframer: inject a mangled `probe-results.txt` frame ──
    (
        "records-headerless-refused",
        "probe-results: a stream with NO `dorc-records/1` framing at all",
    ),
    (
        "records-glued-line",
        "probe-results: a record line with bytes after its terminal token",
    ),
    (
        "records-header-missing",
        "probe-results: a framed stream whose header line is torn/absent",
    ),
    (
        "records-sentinel-nonce",
        "probe-results: an end-sentinel carrying a foreign nonce",
    ),
    (
        "records-integrity-refused",
        "probe-results: a header failing an integrity key (host/book/attempt)",
    ),
    (
        "records-torn-line",
        "probe-results: a record fragment that lost its terminating write",
    ),
    (
        "records-alien-line",
        "probe-results: a non-nonce record line mixed into the stream",
    ),
    (
        "records-late-line",
        "probe-results: a record line after the end-sentinel",
    ),
];

/// The render triple for a canonical [`DiagCode`] (`27V` §3): machine line (the `project_oob`
/// wire projection) · terse line (the filled one-line message) · prose (message + help register).
fn triple(code: &DiagCode, interner: &Interner) -> (String, String, String) {
    let diag = Diag::new(code.clone(), Span::new(BytePos(0), BytePos(1)));
    let oob = diag::project_oob(&diag);
    let site = match oob.site {
        Some(s) => match s.member {
            Some(m) => format!("{}.{m}", s.leaf.0),
            None => s.leaf.0.to_string(),
        },
        None => "-".to_owned(),
    };
    let machine = format!(
        "code={} severity={} site={site}",
        oob.code,
        sev_word(oob.severity)
    );
    let terse = diag::render_message(code, interner);
    let prose = diag::render_body(&diag, interner);
    (machine, terse, prose)
}

fn sev_word(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    }
}

fn golden_path(slug: &str, register: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/defining_cases")
        .join(format!("{slug}.{register}"))
}

/// The defining case = byte-compare of the three renders (`27V` §3). `DORC_DEFINING_BLESS=1`
/// regenerates the colocated goldens (bless-free unit capture — NOT the orchestrator-only e2e BLESS).
#[test]
fn defining_cases_render_triples_byte_match() {
    let interner = Interner::default();
    let bless = std::env::var("DORC_DEFINING_BLESS").as_deref() == Ok("1");
    for case in covered() {
        let (machine, terse, prose) = triple(&(case.build)(), &interner);
        for (register, actual) in [("machine", &machine), ("terse", &terse), ("prose", &prose)] {
            let path = golden_path(case.slug, register);
            if bless {
                std::fs::create_dir_all(path.parent().expect("goldens dir")).expect("mkdir");
                std::fs::write(&path, actual).expect("write golden");
                continue;
            }
            let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| {
                panic!(
                    "defining case `{}` missing the `{register}` golden ({}) — run with \
                     DORC_DEFINING_BLESS=1 to generate it",
                    case.slug,
                    path.display()
                )
            });
            assert_eq!(
                *actual, expected,
                "defining case `{}` {register} render drifted from its golden (re-bless \
                 consciously with DORC_DEFINING_BLESS=1 — goldens-churn-freely, 27V:rul-output-\
                 form-unwelded)",
                case.slug
            );
        }
    }
}

/// COMPLETENESS (`AID-NEEDS:law-one-defining-case-per-code`, ratchet-tempered): every catalog slug is
/// EITHER covered by a defining case OR on the shrink-only [`DEFINING_CASE_RATCHET`] — never silently
/// uncovered. Also: no slug is in BOTH (a covered code must leave the ratchet).
#[test]
fn every_code_has_a_defining_case_or_is_ratcheted() {
    use std::collections::BTreeSet;
    let cases = covered();
    let covered_set: BTreeSet<&str> = cases.iter().map(|c| c.slug).collect();
    let ratchet: BTreeSet<&str> = DEFINING_CASE_RATCHET.iter().map(|(s, _)| *s).collect();
    assert_eq!(
        covered_set.len(),
        cases.len(),
        "a slug is listed twice in covered()"
    );
    if let Some(slug) = covered_set.intersection(&ratchet).next() {
        panic!("`{slug}` is BOTH covered and ratcheted — a covered code must leave the ratchet");
    }
    for e in dorc_core::catalog::CATALOG {
        assert!(
            covered_set.contains(e.slug) || ratchet.contains(e.slug),
            "catalog code `{}` has no defining case and is not on DEFINING_CASE_RATCHET — add a \
             covered() case or a ratchet entry with its trigger surface (silent partial coverage \
             is not acceptable, 27V §3)",
            e.slug
        );
    }
    // The ratchet may not name a retired code (keeps it honest as the catalog shrinks).
    let catalog: BTreeSet<&str> = dorc_core::catalog::CATALOG.iter().map(|e| e.slug).collect();
    for slug in &ratchet {
        assert!(
            catalog.contains(slug),
            "DEFINING_CASE_RATCHET names `{slug}`, which is not a catalog code (stale entry — remove it)"
        );
    }
}

/// The ratchet is SHRINK-ONLY (`tc-defining-case-coverage-ratchet`): it may never GROW past its
/// committed baseline (a new code must ship a covered case, not pad the ratchet). Best-effort against
/// `git show HEAD` — skipped when git is unavailable, like the `diag_tidy` retire-guard.
#[test]
fn ratchet_only_shrinks() {
    use std::process::Command;
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let rel = "crates/core/tests/catalog_defining_cases.rs";
    let spike = manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("crates/core -> crates -> spike");
    let committed = ["HEAD:spike/", "HEAD:"].iter().find_map(|prefix| {
        let out = Command::new("git")
            .arg("-C")
            .arg(spike)
            .arg("show")
            .arg(format!("{prefix}{rel}"))
            .output()
            .ok()?;
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
    });
    let Some(committed) = committed else {
        eprintln!("ratchet_only_shrinks: no committed baseline (new file / no git) — skipping");
        return;
    };
    let baseline = count_ratchet_entries(&committed);
    assert!(
        DEFINING_CASE_RATCHET.len() <= baseline,
        "DEFINING_CASE_RATCHET GREW ({} entries vs the committed {baseline}) — it is shrink-only; a \
         new code must ship a covered defining case, never pad the ratchet (27V §3)",
        DEFINING_CASE_RATCHET.len()
    );
}

/// Count the `("slug", "note")` rows inside the committed `DEFINING_CASE_RATCHET` literal by shape
/// (bounded to the `const DEFINING_CASE_RATCHET` block), so the guard reads the baseline without
/// importing the committed source. Robust to rustfmt wrapping: an entry opener is a line whose
/// trimmed form is either the wrapped `(` or the single-line `("…`.
fn count_ratchet_entries(src: &str) -> usize {
    let Some(start) = src.find("const DEFINING_CASE_RATCHET") else {
        return usize::MAX; // unreadable ⇒ never trips the <= assert (conservative)
    };
    let body = &src[start..];
    let end = body.find("];").map_or(body.len(), |i| i);
    body[..end]
        .lines()
        .filter(|l| {
            let t = l.trim();
            t == "(" || t.starts_with("(\"")
        })
        .count()
}

/// `[unwritten: <slug>]` renders GREPPABLY (`27V` §3): a catalog `message`/`help` that is still the
/// unwritten placeholder must be EXACTLY `[unwritten: <that entry's slug>]` (never a near-miss that
/// escapes a grep), and the count is pinned so it only shrinks as prose is authored.
#[test]
fn unwritten_renders_are_greppable_and_pinned() {
    let mut unwritten = Vec::new();
    for e in dorc_core::catalog::CATALOG {
        let placeholder = format!("[unwritten: {}]", e.slug);
        for (register, text) in [("message", Some(e.message)), ("help", e.help)] {
            let Some(text) = text else { continue };
            if text.contains("[unwritten") {
                assert_eq!(
                    text, placeholder,
                    "catalog `{}` {register} carries a MALFORMED unwritten placeholder — it must be \
                     exactly `{placeholder}` so `grep '\\[unwritten:'` finds every one",
                    e.slug
                );
                unwritten.push((e.slug, register));
            }
        }
    }
    // At the base tip the prose is `sm `-prefixed or conductor-authored — zero `[unwritten:]` yet.
    // This pin SHRINKS to accommodate new codes' placeholders and re-tightens as prose is authored;
    // it never silently grows unnoticed (a bump here is a conscious conductor act).
    assert!(
        unwritten.len() <= 1,
        "more `[unwritten:]` placeholders ({}) than the pinned ceiling — each is a conductor prose \
         debt; bump this ceiling consciously when a new code lands with empty prose: {unwritten:?}",
        unwritten.len()
    );
}
