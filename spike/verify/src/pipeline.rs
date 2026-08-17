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

    // Recorded HERE because this step is the pipeline's own last act, on the stable toolchain:
    // the digest is a statement about what this translation read, and nothing else is in a
    // position to make it truthfully.
    let lock = crate::derivation::compute(repo_root)?;
    write(&crate::derivation::path(repo_root), &lock)?;
    written.push("spike/verify/aeneas/derivation.lock".to_owned());

    let (holes, axioms) = census(&generated)?;
    Ok(Materialized {
        written,
        holes,
        axioms,
    })
}

/// The sorry census plus the axiom count over a generated tree.
///
/// Counts the MATERIALIZED tree only. Each `*External_Template.lean` is committed beside the
/// byte-identical copy `materialize` made of it, and only the copy is imported — counting both
/// reported every axiom twice, which reads as twice the trusted base there really is.
///
/// # Errors
/// When the tree cannot be read.
pub fn census(generated: &Path) -> Result<(usize, usize), String> {
    let mut holes = 0usize;
    let mut axioms = 0usize;
    for path in lean_files(generated)? {
        if is_external_template(&path) {
            continue;
        }
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

/// Whether `path` is one of aeneas's raw external templates, keyed off the same table
/// `materialize` copies from — so a renamed template stops being skipped and stops being
/// copied in one edit, never one without the other.
fn is_external_template(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        EXTERNAL_TEMPLATES
            .iter()
            .any(|(template, _)| name == *template)
    })
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
pub fn lean_build(repo_root: &Path, build_root: &Path) -> Result<Built, String> {
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
    let output = capture(build_root, "lake", &["build"])?;
    if !output.ok {
        return Err(format!("lake build failed:\n{}", output.text));
    }
    Ok(Built {
        dependency_holes: output
            .text
            .lines()
            .filter(|line| line.contains("declaration uses"))
            .count(),
    })
}

/// What a green lake build reports beyond "it built".
#[derive(Debug)]
pub struct Built {
    /// `declaration uses 'sorry'` warnings across the WHOLE build, dependencies included.
    ///
    /// The `Generated/` census cannot see these: aeneas's own Lean library ships holes
    /// (`Aeneas/Std/Slice.lean`, `StringIter.lean` — recorded upstream), and anything proved
    /// through a holed declaration is not proved. Lean says so in one line of a 1700-job
    /// build, which is exactly how a trusted-base entry becomes invisible, so the number is
    /// lifted into the report rather than left in scrollback.
    pub dependency_holes: usize,
}

/// A command's combined output plus whether it succeeded.
struct Captured {
    ok: bool,
    text: String,
}

fn capture(cwd: &Path, program: &str, args: &[&str]) -> Result<Captured, String> {
    let out = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| {
            format!("{program}: {e} (is elan on PATH? `mise run verify:lean-bootstrap`)")
        })?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    Ok(Captured {
        ok: out.status.success(),
        text,
    })
}

/// The staged source subtrees, cleared before every copy (see [`stage`]).
const STAGED_SOURCES: [&str; 2] = ["Minispec", "Generated"];

/// Mirror `minispec/` into the build root — a MIRROR, which is why the sources are cleared first.
///
/// `copy_tree` only ever adds, so a unit deleted or renamed in the repo went on satisfying its
/// stale import in the staged tree forever: that is how a root module naming a unit the
/// repository does not contain survived a green `lake build`
/// (`30B:fnd-lean-staging-never-removes-stale-files`).
///
/// Everything else in the root SURVIVES, and both survivors are load-bearing: `.lake` holds the
/// olean store this whole staging exists to keep warm, and `lake-manifest.json` is untracked
/// build-root state whose loss would send lake back to the network to re-resolve pins.
fn stage(repo_root: &Path, build_root: &Path) -> Result<(), String> {
    let source = repo_root.join("minispec");
    std::fs::create_dir_all(build_root).map_err(|e| format!("{}: {e}", build_root.display()))?;
    for subtree in STAGED_SOURCES {
        let staged = build_root.join(subtree);
        if staged.exists() {
            std::fs::remove_dir_all(&staged).map_err(|e| format!("{}: {e}", staged.display()))?;
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_census_counts_the_materialized_tree_and_not_its_templates() {
        // The trusted base is what the Lean build IMPORTS, and it imports the materialized
        // copy alone. Counting the template beside it reported a trusted base twice the size
        // of the real one — the numbers in a coverage report are the whole product here, so a
        // doubled one is a wrong answer, not a cosmetic one.
        let dir = std::env::temp_dir().join("dorc-verify-census-pin");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let one_axiom = "axiom core_fmt_write : Unit\n";
        std::fs::write(dir.join("FunsExternal_Template.lean"), one_axiom).unwrap();
        std::fs::write(dir.join("FunsExternal.lean"), one_axiom).unwrap();

        assert_eq!(census(&dir).unwrap(), (0, 1), "the pair counts once");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn staging_drops_a_unit_the_repository_no_longer_has_and_keeps_lakes_own_state() {
        let root = std::env::temp_dir().join("dorc-verify-stage-pin");
        let (repo, build) = (root.join("repo"), root.join("build"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(repo.join("minispec").join("Minispec")).unwrap();
        std::fs::write(repo.join("minispec").join("Minispec.lean"), "import X\n").unwrap();
        std::fs::write(
            repo.join("minispec").join("Minispec").join("Live.lean"),
            "-- live\n",
        )
        .unwrap();
        // The staged tree as a previous run left it: one unit since deleted from the repo, plus
        // the two things a stage must never eat.
        std::fs::create_dir_all(build.join("Minispec")).unwrap();
        std::fs::create_dir_all(build.join(".lake").join("build")).unwrap();
        std::fs::write(build.join("Minispec").join("Deleted.lean"), "-- stale\n").unwrap();
        std::fs::write(build.join(".lake").join("build").join("olean"), "cached\n").unwrap();
        std::fs::write(build.join("lake-manifest.json"), "{}\n").unwrap();

        stage(&repo, &build).unwrap();

        assert!(
            !build.join("Minispec").join("Deleted.lean").exists(),
            "a deleted unit that survives staging keeps satisfying its stale import"
        );
        assert!(build.join("Minispec").join("Live.lean").exists());
        assert!(build.join(".lake").join("build").join("olean").exists());
        assert!(build.join("lake-manifest.json").exists());
        std::fs::remove_dir_all(&root).unwrap();
    }
}
