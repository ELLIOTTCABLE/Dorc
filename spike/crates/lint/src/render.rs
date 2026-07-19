//! The two renderers over the ONE finding model (`27R` §5 dir-two-renders-one-model). Human:
//! unstable-by-declaration, grouped per file, source-tagged, quiet-on-clean with a POSITIVE clean
//! sentence (`27R` §8 delta-positive-clean-sentence). Machine: JSONL, a versioned additive-only
//! envelope line carrying the coverage block (`27R` §5, §8b), then one finding per line. Both are
//! pure String producers (color/tty is a cli-edge concern, kept out of the deterministic crate).

use std::fmt::Write as _;

use crate::finding::{Coverage, Finding, LintReport, RemapFidelity, severity_token};
use crate::json::escape_into;

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
    let (errors, warns, infos) = report.severity_counts();
    let file_count = report.coverage.files.len();
    let source_count = report.coverage.sources.len();
    if report.findings.is_empty() {
        return format!(
            "dorc lint: clean — nothing found across {file_count} file(s), {source_count} source(s).\n"
        );
    }
    let mut out = String::new();
    out.push_str(
        "dorc lint findings (advisory; the machine format `--format=jsonl` is the stable surface):\n",
    );
    let mut current_group: Option<&str> = None;
    for f in &report.findings {
        let group = if f.path.is_empty() {
            "(run)"
        } else {
            f.path.as_str()
        };
        if current_group != Some(group) {
            let _ = write!(out, "\n{group}:\n");
            current_group = Some(group);
        }
        out.push_str(&render_finding_line(f));
    }
    let _ = write!(
        out,
        "\ndorc lint: {errors} error(s), {warns} warning(s), {infos} info(s) across {file_count} file(s).\n"
    );
    out
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
