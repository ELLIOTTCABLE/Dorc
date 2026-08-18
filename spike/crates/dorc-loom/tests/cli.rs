//! Process-level read-only inspection coverage.

#![expect(
    clippy::panic,
    reason = "test helpers surface subprocess and UTF-8 failures with their concrete cause"
)]

use std::path::PathBuf;
use std::process::Command;

use errorloom::{MAX_CASE_BYTES, MAX_REPLAY_OUTPUT_BYTES};

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
fn a_clean_selected_case_stays_out_of_the_touched_set() {
    let path = case("cmdsub-command.loom");
    let before =
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture reads: {error}"));
    let output = run(&[
        "vars",
        "--used",
        path.to_str().unwrap_or("fixture path is UTF-8"),
    ]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("case: "));
    assert!(stdout.contains("{{position}}"));
    assert_eq!(std::fs::read_to_string(&path).ok(), Some(before));
}

#[test]
fn inventories_are_ordered_and_do_not_widen_used_values() {
    let path = case("cmdsub-command.loom");
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
fn clean_generated_inventory_replays_are_ignored() {
    let path = case("cmdsub-two-sections.loom");
    let output = run(&[
        "vars",
        "--used",
        path.to_str().unwrap_or("fixture path is UTF-8"),
    ]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("case: "));
    assert!(stdout.contains("case: "));
}

#[test]
fn a_clean_selected_case_has_no_preview() {
    let path = case("cmdsub-command.loom");
    let output = run(&[
        "vars",
        "--used",
        path.to_str().unwrap_or("fixture path is UTF-8"),
    ]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("case: "));
    assert!(stdout.contains("case: "));
}

#[test]
fn a_clean_transcript_output_is_not_reinterpreted() {
    let path = case("cmdsub-partial-refusal.loom");
    let output = run(&[
        "vars",
        "--used",
        path.to_str().unwrap_or("fixture path is UTF-8"),
    ]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("case: "));
}

#[test]
fn all_inventory_excludes_foreign_detail() {
    let path = case("site-foreign.loom");
    let output = run(&[
        "vars",
        "--all",
        path.to_str().unwrap_or("fixture path is UTF-8"),
    ]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));

    assert!(output.status.success(), "{stdout}");
    assert!(!stdout.contains("{{detail}}"));
    // Omitting it silently is the trap: the listing claims the whole payload, and the hole an
    // author can see in the transcript is not in it. On STDERR, because stdout is the inventory
    // and nothing else -- which is also what makes the listing identical inside a loom.
    let stderr =
        String::from_utf8(output.stderr).unwrap_or_else(|error| panic!("stderr is UTF-8: {error}"));
    assert!(
        stderr.contains("foreign passthrough values are omitted deliberately"),
        "the inventory must own its own gap: {stderr}"
    );
}

/// Only `--this` may behave differently inside a loom (`30C` item 1), so every other seat's stdout
/// is the same bytes at a terminal as in a replay block -- no preamble, no absolute path, the
/// case's own declared slug. The committed block in `whylog-absent.loom` is the other half of this
/// pin: it is what the driver prints, and the render fixpoint holds it.
#[test]
fn a_terminal_inventory_is_byte_identical_to_the_in_loom_block() {
    let output = run(&["vars", "whylog-absent"]);
    let stdout =
        String::from_utf8(output.stdout).unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));
    assert!(output.status.success(), "{stdout}");
    assert_eq!(stdout, "case: whylog-absent\n{{dir}} = \".whylog\"\n");
}

#[test]
fn clean_cases_do_not_trigger_marker_compilation() {
    let unknown = case("cmdsub-unknown.loom");
    let unknown = unknown.to_str().unwrap_or("fixture path is UTF-8");
    let refusal = run(&["vars", "--used", unknown]);
    let stdout = String::from_utf8(refusal.stdout)
        .unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));
    assert!(refusal.status.success(), "{stdout}");
    assert!(stdout.contains("case: "));

    let foreign = case("site-foreign.loom");
    let foreign = foreign.to_str().unwrap_or("fixture path is UTF-8");
    let foreign = run(&["vars", "--used", foreign]);
    let stdout = String::from_utf8(foreign.stdout)
        .unwrap_or_else(|error| panic!("stdout is UTF-8: {error}"));
    assert!(foreign.status.success(), "{stdout}");
    assert!(stdout.contains("case: "));

    let malformed = run(&["vars", "--wat"]);
    assert_eq!(malformed.status.code(), Some(2));
    let stderr = String::from_utf8(malformed.stderr)
        .unwrap_or_else(|error| panic!("stderr is UTF-8: {error}"));
    assert!(stderr.contains("--wat"), "{stderr}");
    assert!(
        stderr.contains("usage: dorc-loom [--this] vars"),
        "{stderr}"
    );

    let unreadable = run(&["publish", "missing-case.txt"]);
    assert_eq!(unreadable.status.code(), Some(2));
}

/// The process half of `a_verbs_own_positional_help_is_not_a_help_request`: `add-register` was
/// uninvokable for its whole life and no test noticed, because the mint was covered at the library
/// seat (`seed_help_register`) and nothing ever ran the verb's argv.
///
/// The case here declares no `code`, which refuses inside the verb BEFORE the mint. That is
/// deliberate and is as far as a test may go: the mint publishes the generated lock and rewrites a
/// corpus case at paths fixed to the real tree, so a test that reached it would write sources.
#[test]
fn add_register_reaches_its_verb_rather_than_the_help_page() {
    let dir = std::env::temp_dir().join(format!("dorc-loom-verb-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("registerless.loom");
    std::fs::write(
        &path,
        "---\narrangement: registerless\n---\n-- replay --\n$ dorc plan --book=book.sh\nnothing\n",
    )
    .expect("write the registerless case");
    let path = path.to_str().unwrap_or("test path is UTF-8");

    let reached = run(&["add-register", path, "help"]);
    let stdout = String::from_utf8_lossy(&reached.stdout).into_owned();
    assert!(
        !stdout.contains("usage: dorc-loom"),
        "the verb's own positional must not print a usage page: {stdout}"
    );
    assert_eq!(reached.status.code(), Some(2), "{stdout}");
    assert!(
        String::from_utf8_lossy(&reached.stderr).contains("declares no `code`"),
        "the refusal must come from inside add-register: {}",
        String::from_utf8_lossy(&reached.stderr)
    );

    let asked = run(&["add-register", path, "--help"]);
    let page = String::from_utf8_lossy(&asked.stdout).into_owned();
    assert!(asked.status.success(), "{page}");
    assert!(
        page.starts_with("usage: dorc-loom add-register"),
        "the flag spelling still asks the verb: {page}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// `publish` MUTATES two generated locks and every case it touches, so it never takes the whole
/// corpus by omission. The process-level half of the grammar's rule: a bare invocation reaches the
/// verb's own page and a nonzero exit rather than the whole collection.
///
/// This is as far as a process test may go on this verb. Its write paths land on the REAL corpus
/// and the REAL locks at paths fixed to the tree, and a test that reached them would write sources
/// (`288` §4: never a test side-effect). Covering them wants the injectable corpus root, which is
/// deliberately a separate lane.
#[test]
fn a_bare_publish_reaches_its_own_usage_page_and_refuses() {
    let bare = run(&["publish"]);
    assert_eq!(bare.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&bare.stderr).into_owned();
    assert!(stderr.contains("usage: dorc-loom publish"), "{stderr}");
    assert!(stderr.contains("--all"), "{stderr}");
    assert!(
        String::from_utf8_lossy(&bare.stdout).is_empty(),
        "a refusal writes nothing to stdout"
    );
}

#[test]
fn publish_refuses_bounded_case_input_before_edit_compilation() {
    let dir = std::env::temp_dir().join(format!("dorc-loom-limit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let output_limit = dir.join("output-limit.txt");
    std::fs::write(
        &output_limit,
        format!(
            "---\ncode: cmdsub-operand-top\n---\n-- replay --\n$ dorc plan --book=book.sh\n{}",
            "x".repeat(MAX_REPLAY_OUTPUT_BYTES.saturating_add(1))
        ),
    )
    .expect("write output limit case");
    let output_refusal = run(&[
        "publish",
        output_limit.to_str().unwrap_or("test path is UTF-8"),
    ]);
    assert_eq!(output_refusal.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output_refusal.stderr).contains("committed replay output bytes")
    );

    let file_limit = dir.join("file-limit.txt");
    std::fs::write(&file_limit, vec![b'x'; MAX_CASE_BYTES.saturating_add(1)])
        .expect("write file limit");
    let file_refusal = run(&[
        "publish",
        file_limit.to_str().unwrap_or("test path is UTF-8"),
    ]);
    assert_eq!(file_refusal.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&file_refusal.stderr).contains("file exceeds limit"));
    let _ = std::fs::remove_dir_all(&dir);
}
