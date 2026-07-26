//! The analyzer-coverage rollup over the e2e corpus, ported from `tools/coverage.sh`.
//!
//! An INSTRUMENT, read by a human: it rides no gate and never fails a build. `dorc-coverage`
//! does the analysis; this only knows where the corpus lives, which is repo plumbing rather
//! than anything the instrument should have to learn.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use internal_tooling::which;

/// `internal-tooling coverage [--full] [<book.sh> <oracle.sh>...]`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    let Some(binary) = locate() else {
        // Deliberately SUCCESS: an instrument that fails a build is a gate, and this is not
        // one. Say what to do and get out of the way.
        eprintln!(
            "dorc-coverage not built — `cargo build -p dorc-coverage` (or set DORC_COVERAGE)"
        );
        return ExitCode::SUCCESS;
    };
    let full = args.iter().any(|a| a == "--full");
    let files: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();

    if let Some((book, oracles)) = files.split_first() {
        let mut command = Command::new(&binary);
        command.arg(format!("--book={book}"));
        for oracle in oracles {
            command.args(["-o", oracle]);
        }
        let _ = command.status();
        return ExitCode::SUCCESS;
    }

    let cases = internal_tooling::repo_root()
        .join("spike")
        .join("crates")
        .join("cli")
        .join("tests");
    let Ok(entries) = std::fs::read_dir(&cases) else {
        eprintln!("no case dir at {}", cases.display());
        return ExitCode::SUCCESS;
    };
    // Sorted so two runs of an instrument are comparable; `read_dir` order is not stable.
    let mut dirs: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    dirs.sort();

    for dir in dirs.iter().filter(|d| d.join("book.sh").is_file()) {
        let name = dir.file_name().unwrap_or_default().to_string_lossy();
        println!("######## {name} ########");
        let mut command = Command::new(&binary);
        command.arg(format!("--book={}", dir.join("book.sh").display()));
        for oracle in oracles_in(dir) {
            command.arg("-o").arg(oracle);
        }
        let probe = dir.join("probe-results.txt");
        if probe.is_file() {
            command.arg(format!("--probe-results={}", probe.display()));
        }
        if !full {
            command.arg("--no-table");
        }
        let _ = command.status();
        println!();
    }
    ExitCode::SUCCESS
}

/// `<case>/*.oracle.sh`, sorted — the order the e2e runner assembles them in.
fn oracles_in(dir: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".oracle.sh"))
        .collect();
    found.sort();
    found
}

/// `$DORC_COVERAGE`, else whichever build profile has one.
fn locate() -> Option<PathBuf> {
    if let Some(explicit) = std::env::var_os("DORC_COVERAGE") {
        let path = PathBuf::from(explicit);
        return path.is_file().then_some(path);
    }
    let target = internal_tooling::repo_root().join("spike/target");
    ["debug", "release"]
        .iter()
        .flat_map(|profile| {
            ["dorc-coverage", "dorc-coverage.exe"].map(|bin| target.join(profile).join(bin))
        })
        .find(|candidate| candidate.is_file())
        .or_else(|| which("dorc-coverage"))
}
