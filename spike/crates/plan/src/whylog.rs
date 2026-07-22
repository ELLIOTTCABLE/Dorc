//! `plan::whylog` — the thin posthoc-why durable (`27V` Lane B · `22A:concl-10` ·
//! `whylog-write-only-replay`). The durable is THIN — only what cannot be recomputed — and the
//! full narration is a rendering of a RE-RUN: `dorc why --last` replays it through the SAME kernel
//! (determinism is the license). rec-5 (`probe-tape-not-a-cache`): write-only, replay-driven,
//! nothing re-ingests it across runs for any decision.
//!
//! # Format (`tc-whylog-serialization`, conductor-ACCEPTED line-framed)
//!
//! Hand-rolled line-framed on the [`crate::records`] discipline (version-tagged header + the
//! `@@dorc@@` terminal token + free content last-to-token), NOT serde/JSON — zero deps
//! (`inv-determinism`), proven tear/glue tolerance, and the format's DECLARED byte-instability
//! (additive-only fields, no cross-version byte-stability) makes a rich schema worthless. The one
//! twist: the records stream is stored AS-RECEIVED (`tc-whylog-stores-raw-buffer`) and itself
//! contains `@@dorc@@` tokens, so it rides a BYTE-COUNT-prefixed opaque block (`results bytes=N`)
//! rather than a token-scanned region — replay then re-deframes it through the identical path (the
//! strongest determinism guarantee).
//!
//! # `inv-no-throw`
//!
//! [`parse`] is total: a truncated / clobbered / wrong-version durable yields diagnostics
//! ([`DiagCode::WhylogCorrupt`] / [`DiagCode::WhylogVersionRefused`]), never a panic.

use dorc_core::diag::{Diag, DiagCode, WhylogCorrupt, WhylogVersionRefused};

use crate::records::TERMINAL_TOKEN;

/// The durable's version tag — the format's identity (`27V` §2; the `report-lane-versioned-entry`
/// posture). Recognized once published; a new grammar mints a new tag. NO byte-stability within a
/// version (additive-only fields).
pub const WHYLOG_TAG: &str = "dorc-whylog/1";
/// The end sentinel (a truncated write is detected by its absence — `inv-no-throw` ⇒
/// [`DiagCode::WhylogCorrupt`]).
pub const WHYLOG_END: &str = "dorc-whylog-end/1";

/// One apply-report line (`27V` §2). SPIKE (`tc-apply-report-is-prediction`,
/// `churn-avoidance-disclosure`): there is NO apply executor (`cli/CLAUDE.md` scope-boundary), so
/// `disposition` is the PREDICTED disposition and [`predicted`](Self::predicted) is ALWAYS `true`.
/// The reader must never render a prediction in a measurement's clothes (ruling: a prediction must
/// not wear a measurement's clothes — `law-trust-tier-is-syntax`'s cousin). The field shape stays
/// additive so a real executor later fills genuine ran/guard-passed/guard-fell-through/replaced
/// outcomes + divergence flags + apply-rcs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyLine {
    /// The plan leaf id (`inv-site-keyed-results`).
    pub leaf: u32,
    /// The predicted disposition tag (`run` / `replace` / `guard` / `omit`).
    pub disposition: String,
    /// `true` ⇒ a PREDICTED disposition, not a measured apply outcome (spike: always `true`).
    pub predicted: bool,
}

/// The assembled thin durable (`27V` §2). Reused for [`serialize`] (write) and [`parse`] (replay).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhylogDoc {
    /// The invocation mode (`plan` / `apply` / `roundtrip`).
    pub mode: String,
    /// The full argv, one word per element (consent flags are chain-links, kept verbatim).
    pub argv: Vec<String>,
    /// The book path + its content digest (re-read from disk on replay; the digest is verified —
    /// a mismatch is [`DiagCode::WhylogBookDesync`]).
    pub book: (String, String),
    /// Each oracle path + its content digest (re-read + verified on replay).
    pub oracles: Vec<(String, String)>,
    /// The per-attempt run nonce (the deterministic replay "seed"; `27V` §2 — no RNG/clock).
    pub nonce: String,
    /// The attempt serial (`26A` retry-hygiene).
    pub attempt: u32,
    /// The session host id.
    pub host: String,
    /// The decision digest at write time — a drift signal; the re-derived digest must match on
    /// replay (else [`DiagCode::WhylogBookDesync`]).
    pub decision_digest: String,
    /// The records stream AS-RECEIVED — the raw deframed buffer (site records + the report lane +
    /// deriv/resolv/reach), replayed through the identical deframe path.
    pub raw_results: String,
    /// The apply report (per-leaf PREDICTED outcomes; spike prediction-only — see [`ApplyLine`]).
    pub apply: Vec<ApplyLine>,
}

/// Serialize a [`WhylogDoc`] to the line-framed durable. Deterministic (fixed field order; no
/// hashed-collection iteration — `inv-determinism`). The records stream rides a byte-count-prefixed
/// opaque block so its own `@@dorc@@` tokens never collide with the whylog framing.
#[must_use]
pub fn serialize(doc: &WhylogDoc) -> String {
    use std::fmt::Write;
    let t = TERMINAL_TOKEN;
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{WHYLOG_TAG} nonce={} attempt={} host={} {t}",
        doc.nonce, doc.attempt, doc.host
    );
    let _ = writeln!(out, "invoc mode={} {t}", doc.mode);
    // argv words + paths run LAST-TO-TOKEN (free content — embedded spaces survive).
    for a in &doc.argv {
        let _ = writeln!(out, "argv {a} {t}");
    }
    let _ = writeln!(out, "book digest={} path={} {t}", doc.book.1, doc.book.0);
    for (path, digest) in &doc.oracles {
        let _ = writeln!(out, "oracle digest={digest} path={path} {t}");
    }
    let _ = writeln!(out, "digest decision={} {t}", doc.decision_digest);
    for a in &doc.apply {
        let _ = writeln!(
            out,
            "apply leaf={} disposition={} predicted={} {t}",
            a.leaf,
            a.disposition,
            u8::from(a.predicted)
        );
    }
    // The records stream: byte-count-prefixed opaque block (its own tokens ride verbatim).
    let _ = writeln!(out, "results bytes={} {t}", doc.raw_results.len());
    out.push_str(&doc.raw_results);
    let _ = writeln!(out, "{WHYLOG_END} {t}");
    out
}

/// The result of parsing a durable (`inv-no-throw`): the reconstructed doc (or `None` when refused)
/// plus any refusal diagnostics.
#[derive(Debug, Default)]
pub struct WhylogParse {
    /// The reconstructed durable, or `None` when the parse refused (version/corrupt).
    pub doc: Option<WhylogDoc>,
    /// Refusal diagnostics (version-refused / corrupt). Spanless — the durable is not a source file.
    pub diagnostics: Vec<Diag>,
}

/// Current replay inputs supplied by an I/O edge.
#[derive(Debug, Clone, Copy)]
pub struct WhylogCurrent<'a> {
    /// Current book bytes.
    pub book: Option<&'a str>,
    /// Current oracle bytes by recorded path.
    pub oracles: &'a [(&'a str, &'a str)],
}

/// Inspection outcome for one durable.
#[derive(Debug)]
pub struct WhylogInspection {
    /// Parsed durable when replay remains valid.
    pub doc: Option<WhylogDoc>,
    /// Typed refusal diagnostics.
    pub diagnostics: Vec<Diag>,
}

/// Inspect one exact durable without filesystem or directory access.
///
/// Keeps CLI and transcript replay on one typed-refusal path.
#[must_use]
pub fn inspect(
    raw: Option<&str>,
    identity: &str,
    current: Option<WhylogCurrent<'_>>,
) -> WhylogInspection {
    let Some(raw) = raw else {
        return WhylogInspection {
            doc: None,
            diagnostics: vec![Diag::new_spanless_site(DiagCode::WhylogAbsent(
                dorc_core::diag::WhylogAbsent {
                    dir: identity.to_owned(),
                },
            ))],
        };
    };
    let parsed = parse(raw);
    let Some(doc) = parsed.doc else {
        return WhylogInspection {
            doc: None,
            diagnostics: parsed.diagnostics,
        };
    };
    if let Some(current) = current {
        let desync = current
            .book
            .filter(|book| book_digest(book) != doc.book.1)
            .map(|_| "book".to_owned())
            .or_else(|| {
                doc.oracles.iter().find_map(|(path, digest)| {
                    current.oracles.iter().find_map(|(current_path, source)| {
                        (*current_path == path && book_digest(source) != *digest)
                            .then(|| format!("oracle {path}"))
                    })
                })
            });
        if let Some(which) = desync {
            return WhylogInspection {
                doc: None,
                diagnostics: vec![Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                    dorc_core::diag::WhylogBookDesync { which },
                ))],
            };
        }
    }
    WhylogInspection {
        doc: Some(doc),
        diagnostics: Vec::new(),
    }
}

fn book_digest(source: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in source.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Parse a durable's bytes into a [`WhylogDoc`] (`27V` Lane B). Total (`inv-no-throw`): a
/// wrong-version durable refuses with [`DiagCode::WhylogVersionRefused`]; a truncated/clobbered one
/// with [`DiagCode::WhylogCorrupt`]. The records block is read by its byte-count prefix (robust to
/// the stream's embedded `@@dorc@@` tokens).
#[must_use]
pub fn parse(raw: &str) -> WhylogParse {
    let mut out = WhylogParse::default();
    let Some(header_end) = raw.find('\n') else {
        return corrupt(&mut out, "empty or headerless durable");
    };
    let header = strip_token(raw[..header_end].trim_end());
    // Version gate: a different `dorc-whylog/N` refuses politely; anything else ⇒ corrupt.
    let Some(rest_after_tag) = header.strip_prefix(WHYLOG_TAG) else {
        if let Some(found) = header
            .split_whitespace()
            .next()
            .filter(|w| w.starts_with("dorc-whylog/"))
        {
            out.diagnostics
                .push(Diag::new_spanless_site(DiagCode::WhylogVersionRefused(
                    WhylogVersionRefused {
                        found: found.to_owned(),
                    },
                )));
            return out;
        }
        return corrupt(&mut out, "missing dorc-whylog header tag");
    };

    let mut doc = WhylogDoc::default();
    read_header_keys(rest_after_tag, &mut doc);

    // A `results bytes=N` line switches to reading N raw bytes verbatim.
    let mut cursor = header_end + 1;
    let bytes = raw.as_bytes();
    let mut saw_end = false;
    while cursor < raw.len() {
        let line_end = raw[cursor..].find('\n').map_or(raw.len(), |i| cursor + i);
        let line = strip_token(raw[cursor..line_end].trim_end());
        cursor = line_end + 1;
        if line == WHYLOG_END || line.starts_with(&format!("{WHYLOG_END} ")) {
            saw_end = true;
            break;
        }
        if let Some(n) = line
            .strip_prefix("results bytes=")
            .and_then(|n| n.trim().parse::<usize>().ok())
        {
            let block_end = cursor.saturating_add(n);
            if block_end > bytes.len() {
                return corrupt(&mut out, "results block byte-count exceeds the file");
            }
            raw[cursor..block_end].clone_into(&mut doc.raw_results);
            cursor = block_end;
            continue;
        }
        absorb_line(line, &mut doc);
    }
    if !saw_end {
        return corrupt(&mut out, "no dorc-whylog-end sentinel (a truncated write?)");
    }
    out.doc = Some(doc);
    out
}

/// Push a [`DiagCode::WhylogCorrupt`] and return the (doc-less) parse. Centralised so every corrupt
/// exit carries the code the completeness gate demands.
fn corrupt(out: &mut WhylogParse, detail: &str) -> WhylogParse {
    out.diagnostics
        .push(Diag::new_spanless_site(DiagCode::WhylogCorrupt(
            WhylogCorrupt {
                detail: detail.to_owned(),
            },
        )));
    std::mem::take(out)
}

/// Read `nonce=`/`attempt=`/`host=` from the header tail (additive-key tolerant — unknown keys
/// ignored, `24Kc`).
fn read_header_keys(rest: &str, doc: &mut WhylogDoc) {
    for tok in rest.split_whitespace() {
        if let Some(v) = tok.strip_prefix("nonce=") {
            v.clone_into(&mut doc.nonce);
        } else if let Some(v) = tok.strip_prefix("attempt=") {
            doc.attempt = v.parse().unwrap_or(0);
        } else if let Some(v) = tok.strip_prefix("host=") {
            v.clone_into(&mut doc.host);
        }
    }
}

/// Absorb one non-header, non-results line into the doc. Unknown tags are ignored (additive-key
/// tolerant); a malformed value drops that line (never a panic — `inv-no-throw`).
fn absorb_line(line: &str, doc: &mut WhylogDoc) {
    let Some((tag, rest)) = line.split_once(' ') else {
        return;
    };
    match tag {
        "invoc" => {
            if let Some(m) = rest.strip_prefix("mode=") {
                m.trim().clone_into(&mut doc.mode);
            }
        }
        "argv" => doc.argv.push(rest.trim().to_owned()),
        "book" => {
            if let (Some(d), Some(p)) = split_digest_path(rest) {
                doc.book = (p, d);
            }
        }
        "oracle" => {
            if let (Some(d), Some(p)) = split_digest_path(rest) {
                doc.oracles.push((p, d));
            }
        }
        "digest" => {
            if let Some(d) = rest.strip_prefix("decision=") {
                d.trim().clone_into(&mut doc.decision_digest);
            }
        }
        "apply" => {
            if let Some(a) = parse_apply(rest) {
                doc.apply.push(a);
            }
        }
        _ => {}
    }
}

/// Split a `digest=<hex> path=<free-content>` body — `path=` is last-to-token so a space-bearing
/// path survives. Returns `(digest, path)`.
fn split_digest_path(rest: &str) -> (Option<String>, Option<String>) {
    let digest = rest
        .split_whitespace()
        .find_map(|t| t.strip_prefix("digest="))
        .map(str::to_owned);
    let path = rest
        .find("path=")
        .map(|at| rest[at + "path=".len()..].to_owned());
    (digest, path)
}

/// Parse an `apply leaf=<n> disposition=<tag> predicted=<0|1>` body.
fn parse_apply(rest: &str) -> Option<ApplyLine> {
    let mut leaf = None;
    let mut disposition = None;
    let mut predicted = true;
    for tok in rest.split_whitespace() {
        if let Some(v) = tok.strip_prefix("leaf=") {
            leaf = v.parse::<u32>().ok();
        } else if let Some(v) = tok.strip_prefix("disposition=") {
            disposition = Some(v.to_owned());
        } else if let Some(v) = tok.strip_prefix("predicted=") {
            predicted = v != "0";
        }
    }
    Some(ApplyLine {
        leaf: leaf?,
        disposition: disposition?,
        predicted,
    })
}

/// Strip a trailing ` @@dorc@@` (the whylog reuses the records terminal token as a tear-detector on
/// its own single-line records). A line without it is taken as-is (defensive).
fn strip_token(line: &str) -> &str {
    line.strip_suffix(TERMINAL_TOKEN)
        .map_or(line, |b| b.strip_suffix(' ').unwrap_or(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc() -> WhylogDoc {
        WhylogDoc {
            mode: "plan".to_owned(),
            argv: vec!["dorc".to_owned(), "plan".to_owned(), "--trust-footprints".to_owned()],
            book: ("web host.sh".to_owned(), "abc123".to_owned()), // a space-bearing path
            oracles: vec![("foobar.oracle.sh".to_owned(), "def456".to_owned())],
            nonce: "dorc".to_owned(),
            attempt: 1,
            host: "localhost".to_owned(),
            decision_digest: "0011223344556677".to_owned(),
            // raw_results carries its OWN terminal tokens (the byte-count block must survive them).
            raw_results: "dorc site 0 effect=holds rc=0 @@dorc@@\ndorc report site=1 decline unsound k @@dorc@@\n".to_owned(),
            apply: vec![
                ApplyLine { leaf: 0, disposition: "replace".to_owned(), predicted: true },
                ApplyLine { leaf: 1, disposition: "run".to_owned(), predicted: true },
            ],
        }
    }

    #[test]
    fn round_trips_including_embedded_tokens_and_spaced_paths() {
        let d = doc();
        let parsed = parse(&serialize(&d));
        assert!(
            parsed.diagnostics.is_empty(),
            "a clean durable refuses nothing: {:?}",
            parsed.diagnostics
        );
        assert_eq!(parsed.doc.as_ref(), Some(&d), "serialize→parse is identity");
    }

    #[test]
    fn wrong_version_refuses_politely_never_panics() {
        let raw =
            format!("dorc-whylog/2 nonce=dorc {TERMINAL_TOKEN}\n{WHYLOG_END} {TERMINAL_TOKEN}\n");
        let p = parse(&raw);
        assert!(p.doc.is_none());
        assert!(
            p.diagnostics
                .iter()
                .any(|d| d.code.slug() == "whylog-version-refused")
        );
    }

    #[test]
    fn truncated_durable_is_corrupt_not_a_panic() {
        let mut raw = serialize(&doc());
        raw.truncate(raw.len() / 2);
        let p = parse(&raw);
        assert!(p.doc.is_none());
        assert!(
            p.diagnostics
                .iter()
                .any(|d| d.code.slug() == "whylog-corrupt")
        );
    }

    #[test]
    fn headerless_bytes_are_corrupt() {
        let p = parse("this is not a whylog at all\n");
        assert!(p.doc.is_none());
        assert!(
            p.diagnostics
                .iter()
                .any(|d| d.code.slug() == "whylog-corrupt")
        );
    }

    #[test]
    fn exact_inspection_distinguishes_absence_parse_version_and_desync() {
        let absent = inspect(None, ".whylog", None);
        assert_eq!(absent.diagnostics[0].code.slug(), "whylog-absent");

        let corrupt = inspect(Some("dorc-whylog/1\n"), ".whylog", None);
        assert_eq!(corrupt.diagnostics[0].code.slug(), "whylog-corrupt");

        let version = inspect(
            Some("dorc-whylog/2 nonce=dorc @@dorc@@\ndorc-whylog-end/1 @@dorc@@\n"),
            ".whylog",
            None,
        );
        assert_eq!(version.diagnostics[0].code.slug(), "whylog-version-refused");

        let durable = serialize(&doc());
        let desync = inspect(
            Some(&durable),
            ".whylog",
            Some(WhylogCurrent {
                book: Some("changed book"),
                oracles: &[("foobar.oracle.sh", "changed oracle")],
            }),
        );
        assert_eq!(desync.diagnostics[0].code.slug(), "whylog-book-desync");
    }
}
