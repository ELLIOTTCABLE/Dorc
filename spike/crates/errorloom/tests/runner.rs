//! Replay-runner self-tests (`282` §7). These drive the `loom-mock-tool` helper
//! binary (located via `CARGO_BIN_EXE_*`) so the runner is exercised cross-
//! platform with no shell and no real tools, through the public API only.

use std::path::PathBuf;

use errorloom::{
    Case, MAX_CAPTURE_BYTES, MAX_CASE_BYTES, ReplayInput, RunEnv, RunError, bless_structure,
    check_run, drive_case_with_inputs, run_case,
};

/// The directory holding the built `loom-mock-tool`, so it resolves on the
/// injected PATH by name (exercising `resolve_program`). A plain helper (not a
/// `#[test]` fn), so it avoids `expect` to stay lint-clean without a
/// crate-top `#![expect]`.
fn mock_dir() -> PathBuf {
    let exe = PathBuf::from(env!("CARGO_BIN_EXE_loom-mock-tool"));
    match exe.parent() {
        Some(parent) => parent.to_path_buf(),
        None => exe,
    }
}

fn env() -> RunEnv {
    RunEnv::new()
        .path_dir(mock_dir())
        .shell(env!("CARGO_BIN_EXE_loom-mock-tool"))
}

#[test]
fn captures_argv_stdout_and_combined_stderr() {
    // out: goes to stdout, err: to stderr; combined 2>&1 capture interleaves both.
    let case = Case::parse(
        "---\n---\n-- replay --\n$ loom-mock-tool out:alpha err:beta out:gamma\nplaceholder\n",
    )
    .expect("valid case");
    let capture = run_case(&case, &env()).expect("runs");
    assert_eq!(capture.outputs().len(), 1);
    assert_eq!(capture.outputs()[0], "alpha\nbeta\ngamma\n");
}

#[test]
fn injects_environment() {
    let case =
        Case::parse("---\n---\n-- replay --\n$ loom-mock-tool env:LOOM_MODE\nx\n").expect("valid");
    let capture = run_case(&case, &env().var("LOOM_MODE", "verbose")).expect("runs");
    assert_eq!(capture.outputs()[0], "verbose\n");
}

#[test]
fn state_flows_between_commands_in_the_shared_cwd() {
    // Block one saves stdin (from a materialized section) to a file; block two
    // reads it back — proving the shared cwd persists between sequential commands.
    let case = Case::parse(
        "---\n---\n-- seed.txt --\ncarried-state\n-- replay --\n$ loom-mock-tool write:passed.txt < seed.txt\n$ loom-mock-tool read:passed.txt\ncarried-state\n",
    )
    .expect("valid");
    let capture = run_case(&case, &env()).expect("runs");
    assert_eq!(capture.outputs()[1], "carried-state\n");
}

#[test]
fn structure_bless_inlines_actual_output() {
    let mut case =
        Case::parse("---\n---\n-- replay --\n$ loom-mock-tool out:fresh\nstale placeholder\n")
            .expect("valid");
    bless_structure(&mut case, &env(), None).expect("bless");
    assert_eq!(case.replay().blocks()[0].output(), "fresh\n");
    // Re-running against the just-blessed transcript is byte-stable (the run gate).
    assert!(check_run(&case, &env()).expect("re-run").is_clean());
}

#[test]
fn check_run_reports_drift() {
    let case =
        Case::parse("---\n---\n-- replay --\n$ loom-mock-tool out:actual\nwrong committed\n")
            .expect("valid");
    let report = check_run(&case, &env()).expect("runs");
    assert!(!report.is_clean());
    assert_eq!(report.drifts()[0].actual(), "actual\n");
    assert_eq!(report.drifts()[0].expected(), "wrong committed\n");
}

#[test]
fn sandbox_path_leak_refuses() {
    let case =
        Case::parse("---\n---\n-- replay --\n$ loom-mock-tool cwd\nplaceholder\n").expect("valid");
    let err = run_case(&case, &env()).unwrap_err();
    assert!(matches!(err, RunError::SandboxPathLeak { block: 0, .. }));
}

#[test]
fn no_shell_refuses_before_execution() {
    let case =
        Case::parse("---\n---\n-- replay --\n$ loom-mock-tool out:unused\nx\n").expect("valid");
    let err = run_case(&case, &RunEnv::new().path_dir(mock_dir())).unwrap_err();
    assert!(matches!(err, RunError::ShellNotConfigured));
}

#[test]
fn shell_reports_an_unknown_command_as_captured_output() {
    let case =
        Case::parse("---\n---\n-- replay --\n$ no-such-tool out:x\nplaceholder\n").expect("valid");
    let capture = run_case(&case, &env()).expect("shell runs");
    assert!(capture.outputs()[0].contains("unsupported shell command"));
}

#[test]
fn nonzero_exit_still_captures_output() {
    // A faithful command may exit nonzero (`282` §7); the runner keeps its trace.
    let case = Case::parse("---\n---\n-- replay --\n$ loom-mock-tool out:before rc:3\nx\n")
        .expect("valid");
    let capture = run_case(&case, &env()).expect("runs despite nonzero exit");
    assert_eq!(capture.outputs()[0], "before\n");
}

#[test]
fn bless_then_required_token_gate() {
    // The blessed output must surface the frontmatter `code` value, or the gate
    // refuses at bless (`28A` §1 required-token coherence).
    let mut case = Case::parse(
        "---\ncode: the-slug\n---\n-- replay --\n$ loom-mock-tool out:the-slug\nplaceholder\n",
    )
    .expect("valid");
    bless_structure(&mut case, &env(), Some("code")).expect("token present");

    let mut missing = Case::parse(
        "---\ncode: the-slug\n---\n-- replay --\n$ loom-mock-tool out:nothing\nplaceholder\n",
    )
    .expect("valid");
    let err = bless_structure(&mut missing, &env(), Some("code")).unwrap_err();
    assert!(matches!(err, RunError::Hygiene(_)));
}

#[test]
fn capture_limit_is_enforced_while_the_child_runs() {
    let at_limit = Case::parse(&format!(
        "---\n---\n-- replay --\n$ loom-mock-tool repeat:{MAX_CAPTURE_BYTES}\nplaceholder\n"
    ))
    .expect("valid");
    assert_eq!(
        run_case(&at_limit, &env())
            .expect("boundary capture")
            .outputs()[0]
            .len(),
        MAX_CAPTURE_BYTES
    );

    let over_limit = Case::parse(&format!(
        "---\n---\n-- replay --\n$ loom-mock-tool repeat:{}\nplaceholder\n",
        MAX_CAPTURE_BYTES.saturating_add(1)
    ))
    .expect("valid");
    assert!(matches!(
        run_case(&over_limit, &env()),
        Err(RunError::OutputTooLarge {
            block: 0,
            limit: MAX_CAPTURE_BYTES
        })
    ));
}

#[test]
fn replay_input_rejects_every_unsafe_path_class() {
    for path in [
        "/absolute.txt",
        "../parent.txt",
        "nested/../parent.txt",
        "./current.txt",
        "nested//empty.txt",
        "C:/drive.txt",
        "C:\\drive.txt",
        "\\\\server\\share.txt",
    ] {
        assert_eq!(
            ReplayInput::new(path, "input"),
            Err(RunError::UnsafeReplayInput {
                path: path.to_owned(),
            })
        );
    }
}

#[test]
fn replay_input_content_limit_is_exact() {
    assert!(ReplayInput::new("at-limit.txt", "x".repeat(MAX_CASE_BYTES)).is_ok());
    assert_eq!(
        ReplayInput::new(
            "over-limit.txt",
            "x".repeat(MAX_CASE_BYTES.saturating_add(1))
        ),
        Err(RunError::ReplayInputTooLarge {
            path: "over-limit.txt".to_owned(),
            limit: MAX_CASE_BYTES,
        })
    );
}

#[test]
fn replay_input_refusals_precede_driver_execution() {
    let case = Case::parse("---\n---\n-- existing.txt --\ncase bytes\n-- replay --\n$ ignored\n")
        .expect("valid case");
    let inputs = [
        ReplayInput::new("first.txt", "first bytes").expect("safe input"),
        ReplayInput::new("existing.txt", "conflicting bytes").expect("safe input"),
    ];
    let mut drove = false;

    let error = drive_case_with_inputs(
        &case,
        &RunEnv::new(),
        &inputs,
        |_, _| -> Result<errorloom::ReplayResult<String, String>, RunError> {
            drove = true;
            Err(RunError::EmptyCommand { block: usize::MAX })
        },
    )
    .unwrap_err();

    assert!(!drove);
    assert_eq!(
        error,
        RunError::DuplicateReplayInput {
            path: "existing.txt".to_owned(),
        }
    );
}
