//! Process-level read-only inspection coverage.

#![expect(
    clippy::panic,
    reason = "test helpers surface subprocess and UTF-8 failures with their concrete cause"
)]

use std::path::PathBuf;
use std::process::Command;

fn case(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_dorc-loom"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("dorc-loom starts: {error}"))
}

#[test]
fn compile_previews_a_flagship_command_marker_without_writing_the_case() {
    let path = case("cmdsub-command.txt");
    let before =
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture reads: {error}"));
    let output = run(&["compile", path.to_str().unwrap_or("fixture path is UTF-8")]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("section: cmdsub-operand-top.message#0:"));
    assert!(stdout.contains("Variable({{command}})"));
    assert!(stdout.contains("{{command}} = \"apt-get\""));
    assert!(stdout.contains("concrete:\n"));
    assert_eq!(std::fs::read_to_string(&path).ok(), Some(before));
}

#[test]
fn inventories_are_ordered_and_do_not_widen_used_values() {
    let path = case("cmdsub-command.txt");
    let path = path.to_str().unwrap_or("fixture path is UTF-8");
    let used = run(&["vars", "--used", path]);
    let all = run(&["vars", "--all", path]);
    let used =
        String::from_utf8(used.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));
    let all =
        String::from_utf8(all.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(used.contains("{{position}} = \"operand 3\""));
    assert!(used.contains("{{cause}} = "));
    assert!(!used.contains("{{command}}"));
    assert!(all.contains("{{command}} = \"apt-get\""));
    assert!(!all.contains("{{detail}}"));
}

#[test]
fn compile_ignores_generated_inventory_and_reports_each_changed_section() {
    let path = case("cmdsub-two-sections.txt");
    let output = run(&["compile", path.to_str().unwrap_or("fixture path is UTF-8")]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("replay: 1"));
    let message = stdout.find("section: cmdsub-operand-top.message#0:");
    let help = stdout.find("section: cmdsub-operand-top.help#0:");
    assert!(message < help, "sections retain renderer order: {stdout}");
    assert_eq!(stdout.matches("{{command}} = \"apt-get\"").count(), 2);
}

#[test]
fn compile_reports_changed_sections_in_renderer_order() {
    let path = case("cmdsub-command.txt");
    let output = run(&["compile", path.to_str().unwrap_or("fixture path is UTF-8")]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("replay: 0"));
    let message = stdout.find("section: cmdsub-operand-top.message#0:");
    assert!(message.is_some());
}

#[test]
fn compile_withholds_earlier_previews_when_a_later_replay_refuses() {
    let path = case("cmdsub-partial-refusal.txt");
    let output = run(&["compile", path.to_str().unwrap_or("fixture path is UTF-8")]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert_eq!(output.status.code(), Some(1), "{stdout}");
    assert!(stdout.contains("refusal in replay 1"));
    assert!(
        !stdout.contains("replay: 0"),
        "partial previews are withheld: {stdout}"
    );
}

#[test]
fn all_inventory_excludes_foreign_detail() {
    let path = case("site-foreign.txt");
    let output = run(&[
        "vars",
        "--all",
        path.to_str().unwrap_or("fixture path is UTF-8"),
    ]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(!stdout.contains("{{detail}}"));
}

#[test]
fn compile_refuses_unknown_markers_and_bad_invocations() {
    let unknown = case("cmdsub-unknown.txt");
    let unknown = unknown.to_str().unwrap_or("fixture path is UTF-8");
    let refusal = run(&["compile", unknown]);
    let stdout = String::from_utf8(refusal.stdout)
        .unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));
    assert_eq!(refusal.status.code(), Some(1));
    assert!(stdout.contains("refusal in replay 0: UnknownVariable"));
    assert!(stdout.contains("baseline:"));

    let foreign = case("site-foreign.txt");
    let foreign = foreign.to_str().unwrap_or("fixture path is UTF-8");
    let foreign = run(&["compile", foreign]);
    let stdout = String::from_utf8(foreign.stdout)
        .unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));
    assert_eq!(foreign.status.code(), Some(1));
    assert!(stdout.contains("refusal in replay 0: MarkerOutsideEditableSection"));

    let malformed = run(&["vars", "--wat"]);
    assert_eq!(malformed.status.code(), Some(2));
    let stderr = String::from_utf8(malformed.stderr)
        .unwrap_or_else(|error| panic!("stderr is UTF-8: {error}"));
    assert!(stderr.contains("unknown vars mode"));

    let unreadable = run(&["compile", "missing-case.txt"]);
    assert_eq!(unreadable.status.code(), Some(2));
}
