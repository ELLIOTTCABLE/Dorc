//! The derived-definitions pipeline's non-charon halves.
//!
//! `spike/verify/aeneas/mise.toml` owns the shadowing toolchain pin and the two translator
//! invocations; everything after them lives here, in Rust, because a mise task body carries no
//! shell syntax (`task-bodies-are-shell-free`) and these steps are file moves, text scans and a
//! staged build.
//!
//! # Why the externals are materialized rather than hand-filled
//!
//! aeneas emits `FunsExternal_Template.lean` / `TypesExternal_Template.lean` — the axioms
//! standing in for std functions it does not model. Copying them VERBATIM is the honest
//! reading of an unmodelled function, and it is what makes the count of axioms a measurement
//! rather than a chore. Filling a hole with a hand-written body would be asserting something
//! about std that nothing checks.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::unit;

/// Files aeneas writes that this step turns into the committed tree.
const EXTERNAL_TEMPLATES: [(&str, &str); 2] = [
    ("FunsExternal_Template.lean", "FunsExternal.lean"),
    ("TypesExternal_Template.lean", "TypesExternal.lean"),
];

/// What one materialize pass found.
#[derive(Debug)]
pub struct Materialized {
    /// Files written or moved.
    pub written: Vec<String>,
    /// Proof holes across the whole generated tree — the sorry census.
    pub holes: usize,
    /// Axioms in the external files: the trusted-base entries this translation rests on.
    pub axioms: usize,
}

/// Turn a raw aeneas emission under `minispec/Generated/` into the committed shape.
///
/// # Errors
/// When the emission is missing or a file cannot be written.
pub fn materialize(repo_root: &Path) -> Result<Materialized, String> {
    let generated = repo_root.join("minispec").join("Generated");
    if !generated.is_dir() {
        return Err(format!(
            "{} does not exist — run `mise run verify:translate` first",
            generated.display()
        ));
    }
    let mut written = Vec::new();

    // aeneas writes the lib entry INSIDE `-dest`, while lake wants it one level above the
    // directory it names. Moving it is the whole reason this step exists as a step.
    let emitted_entry = generated.join("Generated.lean");
    if emitted_entry.is_file() {
        let target = repo_root.join("minispec").join("Generated.lean");
        let text = read(&emitted_entry)?;
        write(&target, &text)?;
        std::fs::remove_file(&emitted_entry)
            .map_err(|e| format!("{}: {e}", emitted_entry.display()))?;
        written.push("minispec/Generated.lean".to_owned());
    }

    for (template, target) in EXTERNAL_TEMPLATES {
        let from = generated.join(template);
        if from.is_file() {
            let text = read(&from)?;
            write(&generated.join(target), &text)?;
            written.push(format!("minispec/Generated/{target}"));
        }
    }

    let (holes, axioms) = census(&generated)?;
    Ok(Materialized {
        written,
        holes,
        axioms,
    })
}

/// The sorry census plus the axiom count over a generated tree.
///
/// # Errors
/// When the tree cannot be read.
pub fn census(generated: &Path) -> Result<(usize, usize), String> {
    let mut holes = 0usize;
    let mut axioms = 0usize;
    for path in lean_files(generated)? {
        let text = read(&path)?;
        if unit::contains_hole(&text) {
            holes = holes.saturating_add(1);
        }
        axioms = axioms.saturating_add(
            text.lines()
                .filter(|line| line.trim_start().starts_with("axiom "))
                .count(),
        );
    }
    Ok((holes, axioms))
}

/// Every `.lean` file directly under `dir`, sorted.
///
/// # Errors
/// When the directory cannot be listed.
pub fn lean_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "lean") {
            out.push(path);
        }
    }
    out.sort();
    Ok(out)
}

/// Stage `minispec/` to an ext4 build root and typecheck it with lake.
///
/// The staging is not tidiness. lake's dependency store for this package is a mathlib
/// checkout plus gigabytes of oleans, and unpacking that onto drvfs (`/mnt/c`) is
/// minutes-to-hours slower than ext4 — measured, and the reason the research spike's first
/// attempt was killed at five minutes still cloning. Sources stay committed in-tree; only the
/// build moves.
///
/// # Errors
/// When staging fails, lake is absent, or the build does not succeed. Absence is reported as
/// absence rather than as a failure the caller might read as a broken proof.
pub fn lean_build(repo_root: &Path, build_root: &Path) -> Result<(), String> {
    if cfg!(windows) {
        return Err(
            "the Lean leg is Linux/WSL only (upstream publishes no Windows aeneas asset); \
             run it from the WSL leg"
                .to_owned(),
        );
    }
    stage(repo_root, build_root)?;
    // mathlib's olean cache first: without it lake compiles mathlib from source, which is
    // hours rather than minutes and looks like a hang.
    run(build_root, "lake", &["exe", "cache", "get"])?;
    run(build_root, "lake", &["build"])
}

fn stage(repo_root: &Path, build_root: &Path) -> Result<(), String> {
    let source = repo_root.join("minispec");
    std::fs::create_dir_all(build_root).map_err(|e| format!("{}: {e}", build_root.display()))?;
    copy_tree(&source, build_root)
}

/// Copy everything but the build artefacts lake owns.
fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    let entries = std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if name == ".lake" {
            continue;
        }
        if path.is_dir() {
            copy_tree(&path, &to.join(&name))?;
        } else {
            std::fs::copy(&path, to.join(&name)).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(())
}

fn run(cwd: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .map_err(|e| {
            format!("{program}: {e} (is elan on PATH? `mise run verify:lean-bootstrap`)")
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} {} failed: {status}", args.join(" ")))
    }
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}
