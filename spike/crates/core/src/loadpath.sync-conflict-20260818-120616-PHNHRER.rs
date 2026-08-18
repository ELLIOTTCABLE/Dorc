//! `core::loadpath` — where a `.` operand lands (`30I:rul-dot-resolves-as-sh`).
//!
//! ONE seat, because the answer is asked from three places that must not drift: the CLI edge that
//! decides which file to OPEN, the function-environment domain that decides which definitions a
//! load BINDS, and the bundle projection that decides which bytes a load CARRIES. Two of those are
//! kernels, so the rule is spelled here — pure text, no filesystem, no environment.
//!
//! # The rule is sh's, not a loader's
//!
//! A supported operand resolves exactly as the floor shells resolve it in the modeled working
//! directory: an absolute operand names that path; a relative slash-BEARING operand resolves
//! against the cwd at that load position; a slash-LESS operand asks for a `PATH` search, which is
//! outside the v0 model and resolves nowhere.
//!
//! The rejected alternative — resolving against the directory of the file that spells the `.` —
//! reads naturally for a loader and is wrong for sh: it gives one authored line a different
//! referent under Dorc than under `dash`, so a package composes here and not on the off-ramp, and
//! no erasure-only strip can repair the difference. `rul-unsure-falls-toward-sh-parity` binds name
//! resolution by name, and this is name resolution.
//!
//! # Matching is lexical
//!
//! [`normalize`] is textual: `\` folds to `/`, `.` segments drop, `..` pops a real predecessor, a
//! leading separator survives. It resolves no symlink and asks the filesystem nothing, because the
//! answer must be reproducible from the snapshot alone (`inv-determinism`). Two spellings that do
//! not normalize alike simply do not match, which WITHHOLDS — the safe direction.

/// Where `target` lands when a shell spells `. <target>` with `cwd` current.
///
/// `None` for a slash-less operand: that is a `PATH` search, which reads the ambient environment a
/// kernel may not touch and which POSIX leaves implementation-defined when it misses, so the
/// construct sits outside the two-binary floor and outside v0 (`30I` §4.2 names it as owed).
#[must_use]
pub fn resolve_against_cwd(cwd: &str, target: &str) -> Option<String> {
    (target.contains('/') || target.contains('\\')).then(|| against_cwd(cwd, target))
}

/// Where a path OPERAND lands: the same join, without the `.`-operand's slash-less refusal.
///
/// A path the invocation names (`--book=x.sh`, `-o pkg.oracle.sh`) is a filesystem operand the
/// shell would have resolved against the cwd whether or not it carries a separator, so the two
/// rules genuinely differ and are spelled apart. This is also the CANONICAL KEY every loaded
/// source is filed under — a file named relatively on the command line and sourced absolutely
/// from a book must be one entry, not two.
#[must_use]
pub fn against_cwd(cwd: &str, path: &str) -> String {
    if is_absolute(path) {
        return normalize(path);
    }
    let cwd = normalize(cwd);
    if cwd.is_empty() {
        normalize(path)
    } else {
        normalize(&format!("{cwd}/{path}"))
    }
}

/// Is this operand already rooted? A POSIX leading separator, or a Windows drive prefix — the
/// latter because the controller's own invocation cwd is spelled that way on one of the two
/// platforms this project is developed on, and a drive-lettered cwd joined onto again would
/// produce a path that matches nothing.
#[must_use]
pub fn is_absolute(path: &str) -> bool {
    if path.starts_with('/') || path.starts_with('\\') {
        return true;
    }
    let mut chars = path.chars();
    matches!(
        (chars.next(), chars.next(), chars.next()),
        (Some(letter), Some(':'), Some('/' | '\\')) if letter.is_ascii_alphabetic()
    )
}

/// A path's lexical normal form.
///
/// The leading separator is load-bearing rather than cosmetic, and dropping it was a real bug this
/// tree caught only on its Linux leg (`one-platform-green-is-not-cross-platform-green`): an
/// absolute POSIX oracle path came back RELATIVE, so the sourced file was looked for under the
/// working directory and never found. Windows paths hid it — a drive letter has no leading
/// separator to lose.
#[must_use]
pub fn normalize(path: &str) -> String {
    let rooted = path.starts_with('/') || path.starts_with('\\');
    let mut out: Vec<&str> = Vec::new();
    for segment in path.split(['/', '\\']) {
        match segment {
            "" | "." => {}
            ".." if out.last().is_some_and(|last| *last != "..") => drop(out.pop()),
            other => out.push(other),
        }
    }
    let joined = out.join("/");
    if rooted { format!("/{joined}") } else { joined }
}

#[cfg(test)]
mod tests {
    use super::{against_cwd, normalize, resolve_against_cwd};

    /// A command-line path operand is NOT subject to the slash-less refusal: `-o pkg.oracle.sh`
    /// names a file in the cwd, where `. pkg.oracle.sh` names a `PATH` search. Filing both under
    /// one key is what lets a relatively-named oracle and an absolutely-sourced one be one entry.
    #[test]
    fn a_path_operand_and_a_dot_operand_differ_only_on_slash_lessness() {
        assert_eq!(against_cwd("/ops", "pkg.oracle.sh"), "/ops/pkg.oracle.sh");
        assert_eq!(resolve_against_cwd("/ops", "pkg.oracle.sh"), None);
        assert_eq!(
            resolve_against_cwd("/ops", "./pkg.oracle.sh").as_deref(),
            Some(against_cwd("/ops", "pkg.oracle.sh").as_str())
        );
    }

    /// The whole rule, in the four cases that differ. The relative case is the one the landed
    /// sourcing-file-relative implementation answered differently, and it is the reason this seat
    /// exists at all.
    #[test]
    fn a_dot_operand_resolves_against_the_working_directory() {
        assert_eq!(
            resolve_against_cwd("/ops", "./oracles/alpha.sh").as_deref(),
            Some("/ops/oracles/alpha.sh"),
            "relative and slash-bearing ⇒ joined onto the cwd, NOT onto the sourcing file"
        );
        assert_eq!(
            resolve_against_cwd("/ops/pkg", "/etc/other.sh").as_deref(),
            Some("/etc/other.sh"),
            "absolute operands ignore the cwd"
        );
        assert_eq!(
            resolve_against_cwd("/ops", "helpers.sh"),
            None,
            "slash-less is a PATH search — outside v0, and never a guessed file"
        );
        assert_eq!(
            resolve_against_cwd("", "./h.sh").as_deref(),
            Some("h.sh"),
            "an empty modeled cwd leaves the operand standing alone"
        );
    }

    /// A sourcing file's own directory is NOT consulted: two entrypoints in different packages
    /// spelling `./helpers.sh` from one cwd name ONE file, exactly as a shell would.
    #[test]
    fn the_sourcing_files_directory_is_not_consulted() {
        assert_eq!(
            resolve_against_cwd("/ops", "./helpers.sh").as_deref(),
            resolve_against_cwd("/ops", "helpers/../helpers.sh").as_deref()
        );
    }

    /// Windows spellings reach the same normal form, and a drive-rooted operand counts as
    /// absolute — the controller's own cwd is spelled that way on one of the two development
    /// platforms, and joining a drive letter onto a drive letter would match nothing.
    #[test]
    fn windows_spellings_normalize_and_root() {
        assert_eq!(normalize("oracles\\h.sh"), "oracles/h.sh");
        assert_eq!(
            resolve_against_cwd("C:/ops", "./h.sh").as_deref(),
            Some("C:/ops/h.sh")
        );
        assert_eq!(
            resolve_against_cwd("C:/ops", "D:/other/h.sh").as_deref(),
            Some("D:/other/h.sh")
        );
    }

    /// Purely textual: `.` drops, `..` pops a real predecessor and survives where there is none,
    /// and a leading separator is kept.
    #[test]
    fn normalization_is_textual_only() {
        assert_eq!(normalize("./oracles/../oracles/h.sh"), "oracles/h.sh");
        assert_eq!(normalize("../h.sh"), "../h.sh");
        assert_eq!(normalize("/mnt/c/pkg/../pkg/h.sh"), "/mnt/c/pkg/h.sh");
        assert_eq!(normalize("oracles//h.sh"), "oracles/h.sh");
    }
}
