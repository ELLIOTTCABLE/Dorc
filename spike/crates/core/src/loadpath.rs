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

/// The modeled working directory a load resolves against, or the explicit absence of one.
///
/// A newtype rather than a bare string because "we do not know where this run stands" is a real
/// state with a ruled consequence — an unknown cwd yields an unresolvable load, never a guessed
/// file (`30I` §3.2) — and a bare `&str` has no spelling for it that a caller cannot mistake for a
/// directory name. The v0 profile has exactly one cwd for a whole run: marked oracle top level
/// cannot change directory, and full book cwd flow is owed rather than built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cwd(Option<String>);

impl Default for Cwd {
    /// The flat virtual directory — every relative operand stands alone. It is the default rather
    /// than [`Cwd::unknown`] because a table nobody sited is a table whose paths are already
    /// relative to one another, and defaulting to "we cannot say" would silently resolve nothing.
    fn default() -> Self {
        Self::at("")
    }
}

impl Cwd {
    /// The modeled cwd is `dir`. The empty string is legal and names the root of a virtual,
    /// flat working directory — the shape an in-process driver holding named sources has.
    #[must_use]
    pub fn at(dir: impl Into<String>) -> Self {
        Self(Some(normalize(&dir.into())))
    }

    /// The edge could not answer where the run stands. Every relative operand resolves nowhere.
    #[must_use]
    pub const fn unknown() -> Self {
        Self(None)
    }

    /// Where `target` lands when a shell spells `. <target>` here.
    ///
    /// `None` for a slash-less operand: that is a `PATH` search, which reads the ambient
    /// environment a kernel may not touch and which POSIX leaves implementation-defined when it
    /// misses, so the construct sits outside the two-binary floor and outside v0 (`30I` §4.2 names
    /// it as owed). `None` too when the cwd is unknown and the operand is relative.
    #[must_use]
    pub fn resolve_dot(&self, target: &str) -> Option<String> {
        (target.contains('/') || target.contains('\\')).then(|| self.resolve_operand(target))?
    }

    /// Where a path OPERAND lands: the same join, without the `.`-operand's slash-less refusal.
    ///
    /// A path the invocation names (`--book=x.sh`, `-o pkg.oracle.sh`) is a filesystem operand the
    /// shell would have resolved against the cwd whether or not it carries a separator, so the two
    /// rules genuinely differ and are spelled apart. This is also the CANONICAL KEY every loaded
    /// source is filed under — a file named relatively on the command line and sourced absolutely
    /// from a book must be one entry, not two.
    #[must_use]
    pub fn resolve_operand(&self, path: &str) -> Option<String> {
        if is_absolute(path) {
            return Some(normalize(path));
        }
        let here = self.0.as_deref()?;
        Some(if here.is_empty() {
            normalize(path)
        } else {
            normalize(&format!("{here}/{path}"))
        })
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
    use super::{Cwd, normalize};

    /// The whole rule, in the four cases that differ. The relative case is the one the landed
    /// sourcing-file-relative implementation answered differently, and it is the reason this seat
    /// exists at all.
    #[test]
    fn a_dot_operand_resolves_against_the_working_directory() {
        let ops = Cwd::at("/ops");
        assert_eq!(
            ops.resolve_dot("./oracles/alpha.sh").as_deref(),
            Some("/ops/oracles/alpha.sh"),
            "relative and slash-bearing ⇒ joined onto the cwd, NOT onto the sourcing file"
        );
        assert_eq!(
            ops.resolve_dot("/etc/other.sh").as_deref(),
            Some("/etc/other.sh"),
            "absolute operands ignore the cwd"
        );
        assert_eq!(
            ops.resolve_dot("helpers.sh"),
            None,
            "slash-less is a PATH search — outside v0, and never a guessed file"
        );
        assert_eq!(
            Cwd::at("").resolve_dot("./h.sh").as_deref(),
            Some("h.sh"),
            "an empty modeled cwd leaves the operand standing alone"
        );
    }

    /// A sourcing file's own directory is NOT consulted: two entrypoints in different packages
    /// spelling `./helpers.sh` from one cwd name ONE file, exactly as a shell would.
    #[test]
    fn the_sourcing_files_directory_is_not_consulted() {
        let ops = Cwd::at("/ops");
        assert_eq!(
            ops.resolve_dot("./helpers.sh").as_deref(),
            ops.resolve_dot("helpers/../helpers.sh").as_deref()
        );
    }

    /// A command-line path operand is NOT subject to the slash-less refusal: `-o pkg.oracle.sh`
    /// names a file in the cwd, where `. pkg.oracle.sh` names a `PATH` search. Filing both under
    /// one key is what lets a relatively-named oracle and an absolutely-sourced one be one entry.
    #[test]
    fn a_path_operand_and_a_dot_operand_differ_only_on_slash_lessness() {
        let ops = Cwd::at("/ops");
        assert_eq!(
            ops.resolve_operand("pkg.oracle.sh").as_deref(),
            Some("/ops/pkg.oracle.sh")
        );
        assert_eq!(ops.resolve_dot("pkg.oracle.sh"), None);
        assert_eq!(
            ops.resolve_dot("./pkg.oracle.sh"),
            ops.resolve_operand("pkg.oracle.sh")
        );
    }

    /// An unknown cwd resolves every RELATIVE operand nowhere rather than guessing a file, and
    /// still answers an absolute one — which is what keeps a run whose edge could not name its own
    /// directory honest instead of merely broken.
    #[test]
    fn an_unknown_cwd_resolves_only_absolutes() {
        let nowhere = Cwd::unknown();
        assert_eq!(nowhere.resolve_dot("./h.sh"), None);
        assert_eq!(nowhere.resolve_operand("h.sh"), None);
        assert_eq!(
            nowhere.resolve_dot("/pkg/h.sh").as_deref(),
            Some("/pkg/h.sh")
        );
    }

    /// Windows spellings reach the same normal form, and a drive-rooted operand counts as
    /// absolute — the controller's own cwd is spelled that way on one of the two development
    /// platforms, and joining a drive letter onto a drive letter would match nothing.
    #[test]
    fn windows_spellings_normalize_and_root() {
        assert_eq!(normalize("oracles\\h.sh"), "oracles/h.sh");
        let ops = Cwd::at("C:/ops");
        assert_eq!(ops.resolve_dot("./h.sh").as_deref(), Some("C:/ops/h.sh"));
        assert_eq!(
            ops.resolve_dot("D:/other/h.sh").as_deref(),
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
