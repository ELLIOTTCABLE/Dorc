//! External-adapter tests over a FAKE runner (`27R` §7 deliverable; `anti-masking-tests`: the fake
//! feeds RAW stdout/stderr bytes, NEVER a pre-parsed finding). Exercises the `27R` §4 degradation
//! ladder (good json1 → corrupt-json text fallback → prose raw passthrough), the checkbashisms
//! rc-summing trap, tool-absent, `--no-tools`, and strip-then-remap over a marked file.

use std::collections::{BTreeMap, BTreeSet};

use dorc_lint::{
    ExternalToolRunner, LintInput, LintOptions, RemapFidelity, SourceStatus, ToolRun, lint,
};

/// A runner with canned per-tool availability + output. Feeds RAW bytes only (anti-masking).
struct FakeRunner {
    available: BTreeSet<String>,
    runs: BTreeMap<String, ToolRun>,
}

impl FakeRunner {
    fn new() -> Self {
        Self {
            available: BTreeSet::new(),
            runs: BTreeMap::new(),
        }
    }
    fn with(mut self, tool: &str, rc: i32, stdout: &str, stderr: &str) -> Self {
        self.available.insert(tool.to_owned());
        self.runs.insert(
            tool.to_owned(),
            ToolRun {
                rc,
                stdout: stdout.as_bytes().to_vec(),
                stderr: stderr.as_bytes().to_vec(),
            },
        );
        self
    }
    /// Mark a tool present but producing empty clean output (rc 0).
    fn present_clean(mut self, tool: &str) -> Self {
        self.available.insert(tool.to_owned());
        self.runs.insert(
            tool.to_owned(),
            ToolRun {
                rc: 0,
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
        );
        self
    }
}

impl ExternalToolRunner for FakeRunner {
    fn available(&self, tool: &str) -> bool {
        self.available.contains(tool)
    }
    fn run(&self, tool: &str, _args: &[&str], _stdin: &[u8]) -> ToolRun {
        self.runs.get(tool).cloned().unwrap_or(ToolRun {
            rc: 0,
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
    }
}

fn file(path: &str, src: &str) -> LintInput {
    LintInput {
        path: path.to_owned(),
        src: src.to_owned(),
    }
}

fn only(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| (*s).to_owned()).collect()
}

/// A plain (unmarked) book — strip is the identity, so the line-map is `n ↦ n`.
const BOOK: &str = "#!/bin/sh\napt-get update\necho $UNQUOTED\nls\n";

#[test]
fn shellcheck_good_json1_maps_severity_code_and_line() {
    let json = r#"{"comments":[{"file":"-","line":3,"column":6,"level":"warning","code":2086,"message":"Double quote to prevent globbing."}]}"#;
    let runner = FakeRunner::new().with("shellcheck", 1, json, "");
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["shellcheck"])),
    );
    assert_eq!(report.findings.len(), 1, "one comment ⇒ one finding");
    let f = &report.findings[0];
    assert_eq!(f.code, "SC2086");
    assert_eq!(f.line, Some(3), "identity line-map over an unmarked book");
    assert_eq!(f.col, Some(6));
    assert_eq!(f.severity, dorc_lint::LintSeverity::Warn);
    assert_eq!(f.remap, RemapFidelity::Exact, "json1 tier is exact");
    assert_eq!(f.source, "shellcheck");
    assert_eq!(f.path, "book.sh", "the original path, never `-`");
}

#[test]
fn shellcheck_corrupt_json_degrades_to_tolerant_text() {
    // Not valid JSON, but carries a `-:NN:CC:`-shaped diagnostic — the tolerant text tier remaps it.
    let text = "-:2:1: warning: apt-get update output not checked [SC2069]";
    let runner = FakeRunner::new().with("shellcheck", 1, text, "");
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["shellcheck"])),
    );
    assert_eq!(report.findings.len(), 1);
    let f = &report.findings[0];
    assert_eq!(f.line, Some(2), "the text tier recovered the line");
    assert_eq!(
        f.remap,
        RemapFidelity::Approximate,
        "text tier ⇒ approximate fidelity"
    );
    assert_eq!(f.code, "external-text");
}

#[test]
fn shellcheck_unstructured_output_is_raw_passthrough() {
    // Non-empty output with no line-number shape ⇒ one opaque raw finding (`27R` §4(c)).
    let blather = "shellcheck: something went sideways and there is no structure here";
    let runner = FakeRunner::new().with("shellcheck", 2, blather, "");
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["shellcheck"])),
    );
    assert_eq!(report.findings.len(), 1);
    let f = &report.findings[0];
    assert_eq!(f.code, "external-raw");
    assert_eq!(f.line, None);
    assert_eq!(f.remap, RemapFidelity::None);
    assert!(f.message.contains("no structure"), "raw output rides along");
}

#[test]
fn checkbashisms_additive_rc_is_ignored_when_findings_parse() {
    // rc=3 (checkbashisms' additive 1+2) — MUST be ignored; the parsed finding governs, and there
    // is NO spurious operational finding (`27R` §8 delta-exit-trichotomy-sharpened).
    let lint_out = "-:3:1: warning: possible bashism; 'echo -n'";
    let runner = FakeRunner::new().with("checkbashisms", 3, lint_out, "");
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["checkbashisms"])),
    );
    assert_eq!(
        report.findings.len(),
        1,
        "just the bashism, not a bogus operational note"
    );
    let f = &report.findings[0];
    assert_eq!(f.line, Some(3));
    assert!(f.message.contains("bashism"));
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.code != "external-operational"),
        "additive rc must not mint an operational finding"
    );
}

#[test]
fn nonzero_rc_with_no_findings_is_one_operational_warn() {
    // Clean-parsing (empty) output but nonzero rc ⇒ exactly one warn operational finding.
    let runner = FakeRunner::new().with("checkbashisms", 2, "", "");
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["checkbashisms"])),
    );
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].code, "external-operational");
    assert_eq!(report.findings[0].severity, dorc_lint::LintSeverity::Warn);
}

#[test]
fn clean_tool_run_produces_no_findings() {
    let runner = FakeRunner::new().present_clean("checkbashisms");
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["checkbashisms"])),
    );
    assert!(
        report.findings.is_empty(),
        "clean rc-0 empty output ⇒ no findings"
    );
    assert_eq!(report.coverage.sources[0].status, SourceStatus::Ran);
}

#[test]
fn absent_tool_is_one_info_finding_and_absent_status() {
    let runner = FakeRunner::new(); // nothing available
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["shellcheck"])),
    );
    assert_eq!(
        report.findings.len(),
        1,
        "one info finding per RUN, not per file"
    );
    assert_eq!(report.findings[0].code, "tool-absent");
    assert_eq!(report.findings[0].severity, dorc_lint::LintSeverity::Info);
    assert_eq!(report.coverage.sources[0].status, SourceStatus::Absent);
}

#[test]
fn no_tools_option_disables_external_sources() {
    let runner = FakeRunner::new().with("shellcheck", 1, "{\"comments\":[]}", "");
    let opts = LintOptions {
        tools_enabled: false,
    };
    let report = lint(
        &[file("book.sh", BOOK)],
        &[],
        opts,
        &runner,
        Some(&only(&["shellcheck"])),
    );
    assert!(
        report.findings.is_empty(),
        "--no-tools ⇒ no external findings, not even tool-absent"
    );
    assert_eq!(report.coverage.sources[0].status, SourceStatus::Off);
}

#[test]
fn strip_line_map_remaps_a_marked_file_finding() {
    // A marked oracle: the marker line (orig 2) and the `invariant:` bare-mark line vanish, so a
    // finding on a STRIPPED line must be remapped to its ORIGINAL line (`27R` §4 dir-paths-stay-yours).
    // Stripped text lines: 1 `apt_get__predict() {` (orig 2 vanished the marker; wait orig1 is marker)…
    // The map is asserted precisely by the oracle-crate strip tests; here we assert the remap wiring:
    // a json finding at stripped line 1 maps to whatever original line 1 became.
    let marked = "# dorc-lang/v0.1\nfoo__state_stored_only_in() {\n:   : invariant:fs-view\nprintf 'x\\n'   : kernel\n}\n";
    // Stripped: line1 `foo__…() {` (orig 2), line2 `printf 'x\n'` (orig 4), line3 `}` (orig 5).
    let json = r#"{"comments":[{"file":"-","line":2,"column":1,"level":"info","code":2043,"message":"whatever"}]}"#;
    let runner = FakeRunner::new().with("shellcheck", 1, json, "");
    let report = lint(
        &[file("pkg.oracle.sh", marked)],
        &[],
        LintOptions::default(),
        &runner,
        Some(&only(&["shellcheck"])),
    );
    assert_eq!(report.findings.len(), 1);
    assert_eq!(
        report.findings[0].line,
        Some(4),
        "stripped line 2 (the printf) maps back to original line 4"
    );
    assert_eq!(report.findings[0].remap, RemapFidelity::Exact);
}
