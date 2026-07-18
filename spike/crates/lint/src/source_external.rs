//! `source-external-shellcheck` + `source-external-checkbashisms` (`27R` §2 item-4, §4): garner value
//! from the universe. Each marked file is stripped through the REAL parser-backed strip WITH a line-map
//! (`dir-strip-then-lint`; never a parallel regex-strip), the stripped bytes are fed to the tool via
//! the injected runner (the tool sees stdin/`-`, never a temp path), and its output is parsed down the
//! `27R` §4 degradation ladder: (a) machine format (shellcheck `-f json1`) → (b) tolerant text remap of
//! "looks like a line number" → (c) raw passthrough as one opaque finding. Upstream drift may cost
//! precision, NEVER a crash and NEVER silence. Findings ALWAYS name the user's original path + original
//! line (`dir-paths-stay-yours`); the tool's reported filename is discarded, the line remapped.
//!
//! rc DISCIPLINE (`27R` §8 delta-exit-trichotomy-sharpened): the adapter NEVER interprets a foreign
//! tool-rc beyond zero/nonzero (checkbashisms' ADDITIVE 1|2|4 codes are the named trap). PARSED
//! findings govern: findings present ⇒ just findings (rc ignored); zero findings + nonzero rc ⇒ one
//! warn operational finding; unrecognized output ⇒ raw passthrough.

use dorc_core::Interner;

use crate::finding::{Finding, LintSeverity, RemapFidelity, SourceStatus};
use crate::json;
use crate::runner::ToolRun;
use crate::source::{LintContext, LintSource, Rung};

/// shellcheck: reads stdin (`-`) as POSIX sh (`-s sh` — dorc's dialect IS POSIX sh, so this is the
/// correct portability lens for both stripped-marked oracle text and unmarked books; latitude per
/// §4, rationale in `27S`), emits `-f json1`. Falls to the tolerant text ladder on any parse failure.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Shellcheck;

impl LintSource for Shellcheck {
    fn name(&self) -> &'static str {
        "shellcheck"
    }
    fn describe(&self) -> &'static str {
        "shellcheck -f json1 -s sh over each stripped file"
    }
    fn rung(&self) -> Rung {
        Rung::File
    }
    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        run_external(
            ctx,
            self.name(),
            &["-f", "json1", "-s", "sh", "-"],
            true,
            out,
        )
    }
}

/// checkbashisms: prefer the `--lint` form (`file:line:1: warning: …`; `27R` §8
/// delta-checkbashisms-lint-flag) over its prose default. No machine format exists, so this adapter
/// starts at the tolerant text tier of the ladder.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Checkbashisms;

impl LintSource for Checkbashisms {
    fn name(&self) -> &'static str {
        "checkbashisms"
    }
    fn describe(&self) -> &'static str {
        "checkbashisms --lint over each stripped file"
    }
    fn rung(&self) -> Rung {
        Rung::File
    }
    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        run_external(ctx, self.name(), &["--lint", "-"], false, out)
    }
}

/// The shared external-tool driver. `try_json` = start the ladder at shellcheck's json1 tier;
/// `false` = go straight to the tolerant text tier (checkbashisms). Returns the coverage status
/// (`Off` under `--no-tools`, `Absent` when the tool is not on PATH, else `Ran`).
fn run_external(
    ctx: &LintContext<'_>,
    tool: &'static str,
    args: &[&str],
    try_json: bool,
    out: &mut Vec<Finding>,
) -> SourceStatus {
    if !ctx.options.tools_enabled {
        return SourceStatus::Off;
    }
    if !ctx.runner.available(tool) {
        // `27R` §4 dir-absent-is-info: ONE info finding per run (not per file). A run-level finding
        // carries an empty path (the human render groups it as a run note; JSONL carries "").
        out.push(Finding {
            path: String::new(),
            line: None,
            col: None,
            severity: LintSeverity::Info,
            source: tool,
            code: "tool-absent".to_owned(),
            message: format!(
                "`{tool}` was not found on PATH — its checks were skipped. Install it, pass \
                 --no-tools to silence this, or --require-tools to make it a hard CI error."
            ),
            remap: RemapFidelity::None,
        });
        return SourceStatus::Absent;
    }
    for file in ctx.files {
        lint_one_file(ctx, tool, args, try_json, file, out);
    }
    SourceStatus::Ran
}

/// Strip one file (with the line-map), run the tool over the stripped bytes, and lower its output to
/// findings against the user's ORIGINAL path/line.
fn lint_one_file(
    ctx: &LintContext<'_>,
    tool: &'static str,
    args: &[&str],
    try_json: bool,
    file: &crate::source::LintInput,
    out: &mut Vec<Finding>,
) {
    let mut interner = Interner::default();
    let mapped = dorc_oracle::strip::strip_file_with_map(&mut interner, &file.src).value;
    let run = ctx.runner.run(tool, args, mapped.text.as_bytes());
    match parse_output(try_json, &run) {
        ParseResult::Findings(raws) if !raws.is_empty() => {
            for raw in raws {
                out.push(remap_finding(&file.path, &mapped.line_map, tool, raw));
            }
        }
        // Parsed cleanly, zero findings: clean UNLESS the tool tripped a nonzero rc — then one warn
        // operational finding (never masquerading as clean; `27R` §8 delta-exit-trichotomy-sharpened).
        ParseResult::Findings(_) => {
            if run.rc != 0 {
                out.push(operational_finding(&file.path, tool, run.rc));
            }
        }
        // Total confusion: emit the raw output as ONE opaque finding block (`27R` §4(c)).
        ParseResult::Unparsable(raw) => {
            out.push(Finding {
                path: file.path.clone(),
                line: None,
                col: None,
                severity: LintSeverity::Warn,
                source: tool,
                code: "external-raw".to_owned(),
                message: format!(
                    "unrecognized `{tool}` output (raw): {}",
                    one_line_truncated(&raw)
                ),
                remap: RemapFidelity::None,
            });
        }
    }
}

/// A raw (pre-remap) finding at a STRIPPED-file line, before it is mapped back to the original.
struct RawFinding {
    /// The 1-based STRIPPED line the tool reported, if any.
    line: Option<u32>,
    col: Option<u32>,
    severity: LintSeverity,
    code: String,
    message: String,
    /// The best fidelity this finding can carry once remapped — `Exact` from a machine format,
    /// `Approximate` from the tolerant text tier.
    base: RemapFidelity,
}

/// The outcome of parsing a tool's output.
enum ParseResult {
    /// Parsed successfully (the vec may be empty = a clean run).
    Findings(Vec<RawFinding>),
    /// Non-empty output that no tier recognized — the raw-passthrough case.
    Unparsable(String),
}

/// Run the `27R` §4 ladder: json1 (if `try_json`) → tolerant text → raw passthrough.
fn parse_output(try_json: bool, run: &ToolRun) -> ParseResult {
    if try_json {
        let stdout = String::from_utf8_lossy(&run.stdout);
        // A valid json1 with a `comments` array wins; anything else falls through to the text tier.
        if let Some(v) = json::parse(stdout.trim())
            && let Some(comments) = v.get("comments").and_then(json::Json::as_array)
        {
            return ParseResult::Findings(comments.iter().map(parse_shellcheck_comment).collect());
        }
    }
    parse_text(run)
}

/// One shellcheck json1 `comments[]` entry → a raw finding (`Exact` base). Missing fields degrade to
/// sane defaults (no line, `SC` code, empty message) rather than dropping the comment.
fn parse_shellcheck_comment(c: &json::Json) -> RawFinding {
    let line = c.get("line").and_then(json::Json::as_u32);
    let col = c.get("column").and_then(json::Json::as_u32);
    let level = c
        .get("level")
        .and_then(json::Json::as_str)
        .unwrap_or("warning");
    let code = c
        .get("code")
        .and_then(json::Json::as_u32)
        .map_or_else(|| "SC".to_owned(), |n| format!("SC{n}"));
    let message = c
        .get("message")
        .and_then(json::Json::as_str)
        .unwrap_or("")
        .to_owned();
    RawFinding {
        line,
        col,
        severity: shellcheck_level(level),
        code,
        message,
        base: RemapFidelity::Exact,
    }
}

/// shellcheck `level` → lint severity. `error ⇒ Error`, `warning ⇒ Warn`, `info`/`style ⇒ Info`.
fn shellcheck_level(level: &str) -> LintSeverity {
    match level {
        "error" => LintSeverity::Error,
        "warning" => LintSeverity::Warn,
        _ => LintSeverity::Info,
    }
}

/// The tolerant text tier (`27R` §4(b)): scan combined stdout+stderr for lines that "look like" a
/// diagnostic — a `<file>:<NN>:<CC>:` prefix (shellcheck gcc / checkbashisms `--lint`) or a bare
/// `line <NN>` (checkbashisms prose). A blank combined output is a clean run; non-blank output with
/// zero recognized lines is `Unparsable` (raw passthrough).
fn parse_text(run: &ToolRun) -> ParseResult {
    let mut combined = String::from_utf8_lossy(&run.stdout).into_owned();
    combined.push('\n');
    combined.push_str(&String::from_utf8_lossy(&run.stderr));
    if combined.trim().is_empty() {
        return ParseResult::Findings(Vec::new());
    }
    let mut findings = Vec::new();
    for raw_line in combined.lines() {
        if let Some(f) = parse_text_line(raw_line) {
            findings.push(f);
        }
    }
    if findings.is_empty() {
        ParseResult::Unparsable(combined)
    } else {
        ParseResult::Findings(findings)
    }
}

/// Parse one "looks like a diagnostic" text line, or `None`. Handles both the colon-delimited
/// `file:NN:CC: level: message` shape and the `… line NN …` prose shape.
fn parse_text_line(line: &str) -> Option<RawFinding> {
    // Colon shape: split into at most 4 pieces `[file, NN, CC, rest]`; accept if piece 1 is digits.
    let parts: Vec<&str> = line.splitn(4, ':').collect();
    if parts.len() >= 3
        && let Some(n) = parts.get(1).and_then(|s| s.trim().parse::<u32>().ok())
    {
        let col = parts.get(2).and_then(|s| s.trim().parse::<u32>().ok());
        let msg = parts.get(3).copied().unwrap_or("").trim().to_owned();
        return Some(RawFinding {
            line: Some(n),
            col,
            severity: text_severity(line),
            code: "external-text".to_owned(),
            message: if msg.is_empty() {
                line.trim().to_owned()
            } else {
                msg
            },
            base: RemapFidelity::Approximate,
        });
    }
    // Prose shape: `… line NN …` (checkbashisms default). Find `line ` then trailing digits.
    if let Some(n) = parse_line_keyword(line) {
        return Some(RawFinding {
            line: Some(n),
            col: None,
            severity: text_severity(line),
            code: "external-text".to_owned(),
            message: line.trim().to_owned(),
            base: RemapFidelity::Approximate,
        });
    }
    None
}

/// Extract `NN` from the first `line NN` occurrence in `s`, if any.
fn parse_line_keyword(s: &str) -> Option<u32> {
    let idx = s.find("line ")?;
    let after = s.get(idx.saturating_add(5)..)?;
    let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
    digits.parse::<u32>().ok()
}

/// Guess a severity from a text line's words (the text tier carries no structured level).
fn text_severity(line: &str) -> LintSeverity {
    let lower = line.to_ascii_lowercase();
    if lower.contains("error") {
        LintSeverity::Error
    } else {
        // Bashisms/portability findings are warnings; default there rather than Info so they surface.
        LintSeverity::Warn
    }
}

/// Remap a raw finding's STRIPPED line to the user's ORIGINAL line via the strip line-map, minting
/// the final finding against the original path (`27R` §4 dir-paths-stay-yours). An out-of-range line
/// keeps its number but degrades fidelity to `Approximate`.
fn remap_finding(path: &str, line_map: &[u32], tool: &'static str, raw: RawFinding) -> Finding {
    let (line, remap) = match raw.line {
        None => (None, raw.base),
        Some(stripped) => match stripped
            .checked_sub(1)
            .and_then(|i| line_map.get(i as usize))
            .copied()
        {
            Some(orig) => (Some(orig), raw.base),
            None => (Some(stripped), RemapFidelity::Approximate),
        },
    };
    Finding {
        path: path.to_owned(),
        line,
        col: raw.col,
        severity: raw.severity,
        source: tool,
        code: raw.code,
        message: raw.message,
        remap,
    }
}

/// The "tool exited nonzero with no findings" operational finding (`27R` §4 dir-absent-is-info's
/// sibling; §8 delta-exit-trichotomy-sharpened).
fn operational_finding(path: &str, tool: &'static str, rc: i32) -> Finding {
    Finding {
        path: path.to_owned(),
        line: None,
        col: None,
        severity: LintSeverity::Warn,
        source: tool,
        code: "external-operational".to_owned(),
        message: format!("`{tool}` exited with status {rc} but produced no parseable findings"),
        remap: RemapFidelity::None,
    }
}

/// Collapse a possibly-multiline blob to a single line, truncated, for a raw/opaque finding message.
fn one_line_truncated(s: &str) -> String {
    let flat: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() > 400 {
        let cut: String = flat.chars().take(400).collect();
        format!("{cut}…")
    } else {
        flat
    }
}
