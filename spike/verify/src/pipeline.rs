//! Derived-definitions pipeline.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::unit;

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

/// Materialize under an injected output root.
///
/// # Errors
/// When generated inputs, derivation inputs, or output files cannot be read, written, moved, or
/// inspected.
pub fn materialize(source_root: &Path, output_root: &Path) -> Result<Materialized, String> {
    let generated = output_root.join("minispec").join("Generated");
    if !generated.is_dir() {
        return Err(format!(
            "{} does not exist — run `mise run verify:translate` first",
            generated.display()
        ));
    }
    let mut written = Vec::new();

    let emitted_entry = generated.join("Generated.lean");
    if emitted_entry.is_file() {
        let target = output_root.join("minispec").join("Generated.lean");
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

    let lock = crate::derivation::compute(source_root)?;
    write(&crate::derivation::path(output_root), &lock)?;
    written.push("spike/verify/aeneas/derivation.lock".to_owned());

    let (holes, axioms) = census(&generated)?;
    Ok(Materialized {
        written,
        holes,
        axioms,
    })
}

/// Materialize under the committed root.
///
/// # Errors
/// When generated inputs, derivation inputs, or output files cannot be read, written, moved, or
/// inspected.
pub fn materialize_production(repo_root: &Path) -> Result<Materialized, String> {
    materialize(repo_root, repo_root)
}

/// Strictly compare output trees byte-for-byte.
///
/// # Errors
/// When either output tree cannot be inspected or read.
pub fn compare_outputs(
    committed_root: &Path,
    candidate_root: &Path,
) -> Result<Vec<String>, String> {
    let outputs = [
        PathBuf::from("minispec/Generated"),
        PathBuf::from("minispec/Generated.lean"),
        PathBuf::from("spike/verify/aeneas/derivation.lock"),
    ];
    let mut differences = Vec::new();
    for output in outputs {
        compare_path(
            &committed_root.join(&output),
            &candidate_root.join(&output),
            &output,
            &mut differences,
        )?;
    }
    Ok(differences)
}

fn compare_path(
    committed: &Path,
    candidate: &Path,
    relative: &Path,
    differences: &mut Vec<String>,
) -> Result<(), String> {
    match (committed.try_exists(), candidate.try_exists()) {
        (Ok(false), Ok(false)) => Ok(()),
        (Ok(true), Ok(false)) => {
            differences.push(format!("missing {}", display_path(relative)));
            Ok(())
        }
        (Ok(false), Ok(true)) => {
            differences.push(format!("extra {}", display_path(relative)));
            Ok(())
        }
        (Ok(true), Ok(true)) => {
            let committed_meta = std::fs::metadata(committed)
                .map_err(|e| format!("{}: {e}", committed.display()))?;
            let candidate_meta = std::fs::metadata(candidate)
                .map_err(|e| format!("{}: {e}", candidate.display()))?;
            if committed_meta.is_file() && candidate_meta.is_file() {
                if std::fs::read(committed).map_err(|e| format!("{}: {e}", committed.display()))?
                    != std::fs::read(candidate)
                        .map_err(|e| format!("{}: {e}", candidate.display()))?
                {
                    differences.push(format!("changed {}", display_path(relative)));
                }
                return Ok(());
            }
            if committed_meta.is_dir() && candidate_meta.is_dir() {
                let mut names = std::collections::BTreeSet::new();
                for entry in std::fs::read_dir(committed)
                    .map_err(|e| format!("{}: {e}", committed.display()))?
                {
                    let entry = entry.map_err(|e| format!("{}: {e}", committed.display()))?;
                    names.insert(entry.file_name());
                }
                for entry in std::fs::read_dir(candidate)
                    .map_err(|e| format!("{}: {e}", candidate.display()))?
                {
                    let entry = entry.map_err(|e| format!("{}: {e}", candidate.display()))?;
                    names.insert(entry.file_name());
                }
                for name in names {
                    compare_path(
                        &committed.join(&name),
                        &candidate.join(&name),
                        &relative.join(name),
                        differences,
                    )?;
                }
                return Ok(());
            }
            differences.push(format!("changed {}", display_path(relative)));
            Ok(())
        }
        (Err(e), _) => Err(format!("{}: {e}", committed.display())),
        (_, Err(e)) => Err(format!("{}: {e}", candidate.display())),
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
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
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
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

    #[test]
    fn output_comparison_reports_changed_missing_and_extra_paths_without_touching_inputs() {
        let root = std::env::temp_dir().join("dorc-verify-output-comparison-pin");
        let (committed, candidate) = (root.join("committed"), root.join("candidate"));
        let _ = std::fs::remove_dir_all(&root);
        for tree in [&committed, &candidate] {
            std::fs::create_dir_all(tree.join("minispec").join("Generated")).unwrap();
            std::fs::create_dir_all(tree.join("spike").join("verify").join("aeneas")).unwrap();
            std::fs::write(tree.join("minispec").join("Generated.lean"), "entry\n").unwrap();
            std::fs::write(
                tree.join("spike")
                    .join("verify")
                    .join("aeneas")
                    .join("derivation.lock"),
                "lock\n",
            )
            .unwrap();
        }
        assert!(
            compare_outputs(&committed, &candidate).unwrap().is_empty(),
            "the identical candidate is an idempotent pass"
        );
        std::fs::write(
            committed
                .join("minispec")
                .join("Generated")
                .join("same.lean"),
            "same\n",
        )
        .unwrap();
        std::fs::write(
            committed
                .join("minispec")
                .join("Generated")
                .join("changed.lean"),
            "old\n",
        )
        .unwrap();
        std::fs::write(
            committed
                .join("minispec")
                .join("Generated")
                .join("missing.lean"),
            "gone\n",
        )
        .unwrap();
        std::fs::write(
            candidate
                .join("minispec")
                .join("Generated")
                .join("same.lean"),
            "same\n",
        )
        .unwrap();
        std::fs::write(
            candidate
                .join("minispec")
                .join("Generated")
                .join("changed.lean"),
            "new\n",
        )
        .unwrap();
        std::fs::write(
            candidate
                .join("minispec")
                .join("Generated")
                .join("extra.lean"),
            "extra\n",
        )
        .unwrap();

        assert_eq!(
            compare_outputs(&committed, &candidate).unwrap(),
            vec![
                "changed minispec/Generated/changed.lean",
                "extra minispec/Generated/extra.lean",
                "missing minispec/Generated/missing.lean",
            ]
        );
        assert_eq!(
            std::fs::read_to_string(
                committed
                    .join("minispec")
                    .join("Generated")
                    .join("changed.lean")
            )
            .unwrap(),
            "old\n"
        );
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn materialize_places_every_output_under_the_injected_root() {
        let root = std::env::temp_dir().join("dorc-verify-materialize-root-pin");
        let (source, output) = (root.join("source"), root.join("output"));
        let _ = std::fs::remove_dir_all(&root);
        let aeneas = source.join("spike").join("verify").join("aeneas");
        std::fs::create_dir_all(aeneas.join("src")).unwrap();
        std::fs::write(aeneas.join("Cargo.toml"), "[package]\nname = \"fixture\"\n").unwrap();
        std::fs::write(aeneas.join("src").join("lib.rs"), "pub mod fixture;\n").unwrap();

        let generated = output.join("minispec").join("Generated");
        std::fs::create_dir_all(&generated).unwrap();
        std::fs::write(generated.join("Generated.lean"), "entry\n").unwrap();
        std::fs::write(
            generated.join("FunsExternal_Template.lean"),
            "axiom funs_external : Unit\n",
        )
        .unwrap();
        std::fs::write(
            generated.join("TypesExternal_Template.lean"),
            "axiom types_external : Unit\n",
        )
        .unwrap();

        let done = materialize(&source, &output).unwrap();

        assert_eq!(done.written.len(), 4);
        assert_eq!(done.holes, 0);
        assert_eq!(done.axioms, 2);
        assert_eq!(
            std::fs::read_to_string(output.join("minispec").join("Generated.lean")).unwrap(),
            "entry\n"
        );
        assert_eq!(
            std::fs::read_to_string(generated.join("FunsExternal.lean")).unwrap(),
            "axiom funs_external : Unit\n"
        );
        assert!(
            output
                .join("spike")
                .join("verify")
                .join("aeneas")
                .join("derivation.lock")
                .is_file()
        );
        assert!(!source.join("minispec").exists());
        std::fs::remove_dir_all(root).unwrap();
    }
}
