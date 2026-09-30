//! Which committed text a solving pass answers for (`notes/30Y` § 2.7): the report and a written
//! lock carry `HEAD`, and `--write` refuses while its own inputs differ from it.

use std::path::Path;
use std::process::Command;

fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

pub(super) fn head(dir: &Path) -> Option<String> {
    git(dir, &["rev-parse", "HEAD"]).map(|h| h.trim().to_owned())
}

/// `git status --porcelain -z` as `(XY, path)` pairs; a rename's or copy's source is dropped.
fn entries(z: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut fields = z.split('\0').filter(|f| !f.is_empty());
    while let Some(field) = fields.next() {
        let (xy, path) = (field.get(..2).unwrap_or(""), field.get(3..).unwrap_or(""));
        if matches!(xy.chars().next(), Some('R' | 'C')) {
            let _ = fields.next();
        }
        out.push((xy.to_owned(), path.to_owned()));
    }
    out
}

/// The inputs git reports anything about: modified, staged, deleted, untracked, or ignored are
/// all text the lock would not be with reference to. Everything else in the tree is not asked.
fn uncommitted(status: &[(String, String)], inputs: &[String]) -> Vec<String> {
    inputs
        .iter()
        .filter(|input| {
            status.iter().any(|(_, path)| {
                path == *input || (path.ends_with('/') && input.starts_with(path.as_str()))
            })
        })
        .cloned()
        .collect()
}

/// The named files beside `dir` that differ from `HEAD`, repository-relative; `Err` outside git.
pub(super) fn dirty_inputs(dir: &Path, names: &[&str]) -> Result<Vec<String>, &'static str> {
    const NOT_A_REPO: &str = "not inside a git repository with a commit";
    head(dir).ok_or(NOT_A_REPO)?;
    let prefix = git(dir, &["rev-parse", "--show-prefix"]).ok_or(NOT_A_REPO)?;
    let mut args = vec![
        "status",
        "--porcelain=v1",
        "-z",
        "--untracked-files=all",
        // `matching` names an ignored directory rather than the file inside it.
        "--ignored=traditional",
        "--",
    ];
    args.extend(names);
    let status = git(dir, &args).ok_or(NOT_A_REPO)?;
    let inputs: Vec<String> = names
        .iter()
        .map(|n| format!("{}{n}", prefix.trim()))
        .collect();
    Ok(uncommitted(&entries(&status), &inputs))
}

#[cfg(test)]
mod tests {
    use super::{entries, uncommitted};

    #[test]
    fn only_the_inputs_that_differ_from_the_commit_refuse_a_write() {
        // The lock beside the spec is rewritten between writes and other files say nothing about
        // the spec's text, so only the document and its shared halves may stop a --write, and
        // every way of differing from HEAD (staged, unstaged, untracked, ignored) counts.
        let z = [
            " M specs/w.lock.json",
            "M  specs/w.assay.md",
            "?? specs/shared.assay.md",
            "!! specs/shared-laws.assay.md",
            "?? notes/unrelated.md",
            "R  specs/renamed.md",
            "specs/old.md",
        ]
        .join("\0");
        let inputs = [
            "specs/w.assay.md".to_owned(),
            "specs/shared.assay.md".to_owned(),
            "specs/shared-laws.assay.md".to_owned(),
        ];
        assert_eq!(uncommitted(&entries(&z), &inputs), inputs.to_vec());

        let clean_inputs = [" M specs/w.lock.json", "?? notes/unrelated.md"].join("\0");
        assert!(uncommitted(&entries(&clean_inputs), &inputs).is_empty());

        // git may name an untracked or ignored directory instead of the files in it.
        let whole_dir = "?? specs/".to_owned();
        assert_eq!(uncommitted(&entries(&whole_dir), &inputs), inputs.to_vec());
    }
}
