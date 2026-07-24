//! The two renderers over the ONE finding model (`27R` §5 dir-two-renders-one-model). Human:
//! unstable-by-declaration, grouped per file, source-tagged, quiet-on-clean with a POSITIVE clean
//! sentence (`27R` §8 delta-positive-clean-sentence). Machine: JSONL, a versioned additive-only
//! envelope line carrying the coverage block (`27R` §5, §8b), then one finding per line. Both are
//! pure String producers (color/tty is a cli-edge concern, kept out of the deterministic crate).

use std::fmt::Write as _;

use crate::finding::{Coverage, Finding, LintReport, RemapFidelity, severity_token};
use crate::json::escape_into;
use dorc_core::tagged::{RenderPart, RenderParts};

/// The versioned machine-format name (`27R` §5 dir-stability-split): the ENVELOPE/field schema is
/// stable and additive-only (`27R` §8 delta-additive-only-format-policy); a breaking change mints a
/// NEW name rather than mutating this one.
pub const JSONL_FORMAT: &str = "dorc-lint-format/1";

/// The human render (`27R` §5). Quiet-on-clean: no findings ⇒ a single positive sentence, nothing
/// else. With findings: a one-time advisory preamble (`27R` §8 delta-advisory-preamble-taste), then
/// findings grouped per file (run-level findings — empty path — under a `(run)` header first), then a
/// summary line. Unstable by declaration; never parse this.
#[must_use]
pub fn render_human(report: &LintReport) -> String {
    render_human_parts(report).text()
}

/// The authoritative human render retains core diagnostic parts rather than
/// attempting to recover editable prose from the completed text.
#[must_use]
pub fn render_human_parts(report: &LintReport) -> RenderParts {
    let (errors, warns, infos) = report.severity_counts();
    let file_count = report.coverage.files.len();
    let source_count = report.coverage.sources.len();
    if report.findings.is_empty() {
        return structure(format!(
            "dorc lint: clean — nothing found across {file_count} file{}, {source_count} source{}.\n",
            plural(file_count),
            plural(source_count)
        ));
    }
    let mut out = structure(String::from(
        "dorc lint findings (advisory; the machine format `--format=jsonl` is the stable surface):\n",
    ));
    let mut current_group: Option<&str> = None;
    for f in &report.findings {
        let group = if f.path.is_empty() {
            "(run)"
        } else {
            f.path.as_str()
        };
        if current_group != Some(group) {
            out.push(RenderPart::Arrangement {
                text: format!("\n{group}:\n"),
                slug: "lint-group",
            });
            current_group = Some(group);
        }
        append_finding_parts(&mut out, f);
    }
    out.push(RenderPart::Arrangement {
        text: format!(
            "\ndorc lint: {errors} error{}, {warns} warning{}, {infos} info{} across {file_count} file{}.\n",
            plural(errors),
            plural(warns),
            plural(infos),
            plural(file_count)
        ),
        slug: "lint-summary",
    });
    out
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn structure(text: String) -> RenderParts {
    let mut parts = RenderParts::new();
    parts.push(RenderPart::Arrangement {
        text,
        slug: "lint-structure",
    });
    parts
}

fn append_finding_parts(out: &mut RenderParts, finding: &Finding) {
    if let Some(provenance) = &finding.provenance {
        out.push(RenderPart::Arrangement {
            text: String::from("  "),
            slug: "lint-indent",
        });
        out.append(dorc_core::diag::render_cli_parts(
            &dorc_core::catalog::CONST_CATALOG,
            &provenance.diag,
            &provenance.source,
            &finding.path,
            &dorc_core::Interner::default(),
        ));
        out.push(RenderPart::Arrangement {
            text: String::from("\n"),
            slug: "lint-terminal-newline",
        });
    } else {
        out.push(RenderPart::Arrangement {
            text: render_finding_line(finding),
            slug: "lint-fixed-finding",
        });
    }
}

/// One human finding line: `  <line>:<col> <severity> [<source>:<code>] <message>` plus a
/// `(approximate)`/`(raw)` tag when the location fidelity is not exact (`27R` §4 remap-fidelity).
fn render_finding_line(f: &Finding) -> String {
    let loc = match (f.line, f.col) {
        (Some(l), Some(c)) => format!("{l}:{c}"),
        (Some(l), None) => format!("{l}"),
        (None, _) => "-".to_owned(),
    };
    let fidelity = match f.remap {
        RemapFidelity::Exact => "",
        RemapFidelity::Approximate => " (approximate location)",
        RemapFidelity::None => " (raw)",
    };
    format!(
        "  {loc} {} [{}:{}] {}{fidelity}\n",
        severity_token(f.severity),
        f.source,
        f.code,
        f.message
    )
}

/// The JSONL render (`27R` §5, §8b). Line 1 is the envelope (format name + coverage block); each
/// subsequent line is one finding object. Additive-only: consumers tolerate unknown fields/enum
/// values (`27R` §8 delta-additive-only-format-policy).
#[must_use]
pub fn render_jsonl(report: &LintReport) -> String {
    let mut out = String::new();
    out.push_str(&envelope_line(report));
    out.push('\n');
    for f in &report.findings {
        out.push_str(&finding_object(f));
        out.push('\n');
    }
    out
}

/// The envelope line: `{"format":…,"coverage":{…}}` with the `27R` §8b coverage block.
fn envelope_line(report: &LintReport) -> String {
    let (errors, warns, infos) = report.severity_counts();
    let mut s = String::from("{\"format\":");
    push_json_str(&mut s, JSONL_FORMAT);
    s.push_str(",\"coverage\":");
    push_coverage(
        &mut s,
        &report.coverage,
        report.findings.len(),
        errors,
        warns,
        infos,
    );
    s.push('}');
    s
}

/// The coverage object (`27R` §8b dir-envelope-carries-coverage): the lintable-file list, per-source
/// status, and counts. A CI policy diffs this to catch silent scope-shrinkage; dorc owns no state.
fn push_coverage(
    out: &mut String,
    cov: &Coverage,
    findings: usize,
    errors: usize,
    warns: usize,
    infos: usize,
) {
    out.push_str("{\"files\":[");
    for (i, f) in cov.files.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        push_json_str(out, f);
    }
    out.push_str("],\"sources\":[");
    for (i, s) in cov.sources.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"name\":");
        push_json_str(out, s.name);
        out.push_str(",\"status\":");
        push_json_str(out, s.status.token());
        out.push('}');
    }
    let _ = write!(
        out,
        "],\"counts\":{{\"findings\":{findings},\"errors\":{errors},\"warns\":{warns},\
         \"infos\":{infos},\"files\":{}}}}}",
        cov.files.len()
    );
}

/// One finding as a JSON object line (`27R` §5): `{path, line, col, severity, source, code, message,
/// remap}`. `line`/`col` are `null` when absent.
fn finding_object(f: &Finding) -> String {
    let mut s = String::from("{\"path\":");
    push_json_str(&mut s, &f.path);
    s.push_str(",\"line\":");
    push_opt_num(&mut s, f.line);
    s.push_str(",\"col\":");
    push_opt_num(&mut s, f.col);
    s.push_str(",\"severity\":");
    push_json_str(&mut s, severity_token(f.severity));
    s.push_str(",\"source\":");
    push_json_str(&mut s, f.source);
    s.push_str(",\"code\":");
    push_json_str(&mut s, &f.code);
    s.push_str(",\"message\":");
    push_json_str(&mut s, &f.message);
    s.push_str(",\"remap\":");
    push_json_str(&mut s, f.remap.token());
    s.push('}');
    s
}

/// Append a quoted, escaped JSON string.
fn push_json_str(out: &mut String, s: &str) {
    out.push('"');
    escape_into(out, s);
    out.push('"');
}

/// Append a JSON number or `null`.
fn push_opt_num(out: &mut String, n: Option<u32>) {
    match n {
        Some(v) => out.push_str(&v.to_string()),
        None => out.push_str("null"),
    }
}
