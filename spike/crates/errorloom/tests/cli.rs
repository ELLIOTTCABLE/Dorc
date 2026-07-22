//! CLI self-tests (`282` §5): drive the built `errorloom` binary as the cram
//! tool, with `loom-mock-tool` as the case command on the injected PATH. Proves
//! `bless` re-inlines and `run` exits nonzero on drift, end to end.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// The directory holding both built bins (they share a target dir).
fn bin_dir() -> PathBuf {
    let exe = PathBuf::from(env!("CARGO_BIN_EXE_errorloom"));
    match exe.parent() {
        Some(parent) => parent.to_path_buf(),
        None => exe,
    }
}

const CASE: &str =
    "---\ncode: the-slug\n---\n-- replay --\n$ loom-mock-tool out:the-slug\nstale placeholder\n";

#[test]
fn bless_inlines_then_run_detects_drift() {
    let dir = std::env::temp_dir().join(format!("errorloom-cli-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir");
    let case_path = dir.join("case.txt");
    fs::write(&case_path, CASE).expect("write case");

    let errorloom = env!("CARGO_BIN_EXE_errorloom");
    let path_flag = format!("--path={}", bin_dir().display());
    let shell_flag = format!("--shell={}", env!("CARGO_BIN_EXE_loom-mock-tool"));
    let case_arg = case_path.display().to_string();

    let blessed_run = Command::new(errorloom)
        .args([
            "bless",
            &path_flag,
            &shell_flag,
            "--require-token=code",
            &case_arg,
        ])
        .output()
        .expect("spawn bless");
    assert!(blessed_run.status.success(), "bless should succeed");

    let blessed = fs::read_to_string(&case_path).expect("read blessed");
    assert!(
        blessed.contains("$ loom-mock-tool out:the-slug\nthe-slug\n"),
        "the block output was re-inlined; got:\n{blessed}"
    );

    let clean = Command::new(errorloom)
        .args(["run", &path_flag, &shell_flag, &case_arg])
        .output()
        .expect("spawn run");
    assert!(clean.status.success(), "a just-blessed case runs clean");

    // Tamper with only the committed OUTPUT line (leaving the command intact);
    // `run` must exit nonzero on the drift.
    let tampered = blessed.replace("out:the-slug\nthe-slug\n", "out:the-slug\ntampered\n");
    fs::write(&case_path, tampered).expect("tamper");
    let drift = Command::new(errorloom)
        .args(["run", &path_flag, &shell_flag, &case_arg])
        .output()
        .expect("spawn run");
    assert_eq!(drift.status.code(), Some(1), "drift exits 1");

    let _ = fs::remove_dir_all(&dir);
}
