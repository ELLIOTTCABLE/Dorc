//! Report-shape tests (`27R` §7): rung-book analysis-diagnostics + unmodeled-inventory findings, the
//! JSONL envelope + coverage block, source-subset selection, the clean sentence, and the
//! severity-threshold counting the cli's exit trichotomy consumes.

use dorc_core::Severity;
use dorc_lint::{
    LintInput, LintOptions, LintReport, NoToolsRunner, json, lint, list_sources, render,
};

fn file(path: &str, src: &str) -> LintInput {
    LintInput {
        path: path.to_owned(),
        src: src.to_owned(),
    }
}

fn only(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| (*s).to_owned()).collect()
}

/// A book with a ⊤-wall: `eval` is `syntax-unsupported` + `cfg-top-node`, then a downstream command.
const EVAL_BOOK: &str = "eval \"apt-get install -y nginx\"\napt-get install -y curl\n";

/// A clean book (no diagnostics, no walls).
const CLEAN_BOOK: &str = "#!/bin/sh\napt-get install -y nginx\nsystemctl enable nginx\n";

fn run_native(files: &[LintInput], only_names: Option<&[String]>) -> LintReport {
    lint(
        files,
        &[],
        LintOptions::default(),
        &NoToolsRunner,
        only_names,
    )
}

#[test]
fn analysis_diagnostics_surface_eval_wall_errors() {
    let report = run_native(
        &[file("book.sh", EVAL_BOOK)],
        Some(&only(&["analysis-diagnostics"])),
    );
    let codes: Vec<&str> = report.findings.iter().map(|f| f.code.as_str()).collect();
    assert!(
        codes.contains(&"syntax-unsupported"),
        "eval trips syntax-unsupported: {codes:?}"
    );
    assert!(
        codes.contains(&"cfg-top-node"),
        "and the downstream cfg ⊤-node: {codes:?}"
    );
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.source == "analysis-diagnostics"),
        "all tagged to this source"
    );
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.severity == Severity::Error),
        "the wall errors are Error-severity"
    );
}

#[test]
fn unmodeled_inventory_reports_one_wall_summary() {
    let report = run_native(
        &[file("book.sh", EVAL_BOOK)],
        Some(&only(&["unmodeled-inventory"])),
    );
    assert_eq!(report.findings.len(), 1, "one per-book summary finding");
    let f = &report.findings[0];
    assert_eq!(f.code, "unmodeled-wall-inventory");
    assert_eq!(f.severity, Severity::Note, "an advisory hint, never gates");
    assert_eq!(f.line, Some(1), "the first wall is the eval on line 1");
}

#[test]
fn clean_book_is_silent_with_a_positive_sentence() {
    let report = run_native(&[file("book.sh", CLEAN_BOOK)], None);
    let native: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.code != "tool-absent")
        .collect();
    assert!(
        native.is_empty(),
        "a clean book has no native findings: {native:?}"
    );
    let clean = run_native(
        &[file("book.sh", CLEAN_BOOK)],
        Some(&only(&["analysis-diagnostics", "unmodeled-inventory"])),
    );
    assert!(clean.findings.is_empty());
    let human = render::render_human(&clean);
    assert!(
        human.contains("clean — nothing found"),
        "positive clean sentence: {human}"
    );
}

#[test]
fn jsonl_envelope_carries_format_and_coverage_block() {
    let report = run_native(
        &[file("book.sh", EVAL_BOOK)],
        Some(&only(&["analysis-diagnostics", "unmodeled-inventory"])),
    );
    let out = render::render_jsonl(&report);
    let mut lines = out.lines();
    let envelope = lines.next().expect("an envelope line");
    let v = json::parse(envelope).expect("the envelope is valid JSON");
    assert_eq!(
        v.get("format").and_then(json::Json::as_str),
        Some(render::JSONL_FORMAT),
        "versioned format name"
    );
    let cov = v.get("coverage").expect("a coverage block");
    assert_eq!(
        cov.get("files")
            .and_then(json::Json::as_array)
            .map(<[_]>::len),
        Some(1),
        "the lintable-file list"
    );
    let sources = cov
        .get("sources")
        .and_then(json::Json::as_array)
        .expect("per-source status");
    assert_eq!(
        sources.len(),
        2,
        "two selected sources, each with a status row"
    );
    assert!(
        sources
            .iter()
            .all(|s| s.get("status").and_then(json::Json::as_str) == Some("ran")),
        "both native sources ran"
    );
    let counts = cov.get("counts").expect("a counts block");
    assert!(
        counts
            .get("errors")
            .and_then(json::Json::as_u32)
            .unwrap_or(0)
            >= 2
    );
    for line in lines {
        let f = json::parse(line).unwrap_or_else(|| panic!("finding line is valid JSON: {line}"));
        assert!(
            f.get("severity").is_some(),
            "finding carries severity: {line}"
        );
        assert!(
            f.get("remap").is_some(),
            "finding carries a remap tag: {line}"
        );
    }
}

#[test]
fn source_subset_selection_runs_only_named() {
    let report = run_native(
        &[file("book.sh", EVAL_BOOK)],
        Some(&only(&["unmodeled-inventory"])),
    );
    assert_eq!(
        report.coverage.sources.len(),
        1,
        "only the one named source ran"
    );
    assert_eq!(report.coverage.sources[0].name, "unmodeled-inventory");
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.source == "unmodeled-inventory"),
        "no other source's findings leak in"
    );
}

#[test]
fn severity_threshold_counting_drives_the_exit_trichotomy() {
    let report = run_native(
        &[file("book.sh", EVAL_BOOK)],
        Some(&only(&["analysis-diagnostics", "unmodeled-inventory"])),
    );
    let (errors, _warns, infos) = report.severity_counts();
    assert!(
        errors >= 2 && infos >= 1,
        "errors from the wall, an info from the inventory"
    );
    assert_eq!(
        report.count_at_or_above(Some(Severity::Error)),
        errors,
        "fail-on=error"
    );
    assert!(
        report.count_at_or_above(Some(Severity::Warning)) >= errors,
        "fail-on=warn includes errors"
    );
    assert_eq!(
        report.count_at_or_above(None),
        0,
        "fail-on=never never gates"
    );
}

#[test]
fn verdict_body_flags_a_terminal_pipeline() {
    let oracle = "# dorc-lang/v0.1\nfoo__is_converged() {\nfoo --status \"$1\" | grep -q ok\n}\n";
    let report = lint(
        &[],
        &[file("foo.oracle.sh", oracle)],
        LintOptions::default(),
        &NoToolsRunner,
        Some(&only(&["verdict-body"])),
    );
    assert_eq!(
        report.findings.len(),
        1,
        "the one pipeline-tailed verdict body is flagged"
    );
    let f = &report.findings[0];
    assert_eq!(f.code, "verdict-terminal-pipeline");
    assert_eq!(f.severity, Severity::Warning);
    assert_eq!(f.source, "verdict-body");
    assert_eq!(f.line, Some(3), "the pipeline command is on line 3");
}

#[test]
fn list_sources_enumerates_the_registry() {
    let sources = list_sources();
    let names: Vec<&str> = sources.iter().map(|s| s.name).collect();
    assert!(names.contains(&"analysis-diagnostics"));
    assert!(names.contains(&"shellcheck"));
    assert!(names.contains(&"checkbashisms"));
    assert!(names.contains(&"verdict-body"));
    assert!(names.contains(&"unmodeled-inventory"));
}
