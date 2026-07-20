//! A real-`git` smoke test for [`SubprocessGit`] (`282` §6): proves the
//! `head_version_of` / `dirty_paths` porcelain parsing against an actual git in a
//! throwaway temp repo. The orchestration itself is proven hermetically with
//! `FakeGit` (`toy_consumer.rs`); this is the edge test for the subprocess adapter,
//! and skips cleanly when git is unavailable.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use errorloom::{Git, GitError, SubprocessGit};

fn git(repo: &std::path::Path, args: &[&str]) -> Option<bool> {
    let status = Command::new("git")
        .arg("-c")
        .arg("user.email=loom@example.invalid")
        .arg("-c")
        .arg("user.name=loom")
        .arg("-c")
        .arg("commit.gpgsign=false")
        .arg("-c")
        .arg("core.autocrlf=false")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .ok()?;
    Some(status.status.success())
}

#[test]
fn head_version_and_dirty_paths_over_real_git() {
    let repo = std::env::temp_dir().join(format!("errorloom-git-{}", std::process::id()));
    let _ = fs::remove_dir_all(&repo);
    fs::create_dir_all(&repo).expect("temp repo dir");

    // Skip cleanly if git is unavailable or init fails (edge-tool posture).
    let Some(true) = git(&repo, &["init", "-q", "-b", "main"]) else {
        let _ = fs::remove_dir_all(&repo);
        return;
    };

    fs::write(repo.join("case.txt"), "committed body\n").expect("write");
    assert_eq!(git(&repo, &["add", "case.txt"]), Some(true));
    assert_eq!(git(&repo, &["commit", "-q", "-m", "seed"]), Some(true));

    let facade = SubprocessGit::new(&repo);
    assert_eq!(
        facade
            .head_version_of(&PathBuf::from("case.txt"))
            .expect("query"),
        Some("committed body\n".to_owned())
    );
    assert_eq!(
        facade
            .head_version_of(&PathBuf::from("absent.txt"))
            .expect("query"),
        None
    );
    assert!(facade.dirty_paths().expect("clean").is_empty());

    fs::write(repo.join("case.txt"), "edited body\n").expect("edit");
    assert_eq!(
        facade.dirty_paths().expect("dirty"),
        vec![PathBuf::from("case.txt")]
    );

    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn genuine_git_failure_surfaces_a_nonzero_exit() {
    // An unborn HEAD is a genuine git failure, no longer swallowed as the
    // "path not in HEAD" None (swe-F2) → NonZeroExit. The repo's own fresh `.git`
    // dodges any ancestor-repo the temp dir sits in.
    let repo = std::env::temp_dir().join(format!("errorloom-unborn-{}", std::process::id()));
    let _ = fs::remove_dir_all(&repo);
    fs::create_dir_all(&repo).expect("temp repo dir");

    let Some(true) = git(&repo, &["init", "-q", "-b", "main"]) else {
        let _ = fs::remove_dir_all(&repo);
        return;
    };

    let facade = SubprocessGit::new(&repo);
    assert!(matches!(
        facade.head_version_of(&PathBuf::from("case.txt")),
        Err(GitError::NonZeroExit { .. })
    ));

    let _ = fs::remove_dir_all(&repo);
}
