//! Seat citation resolution (`301` §5 anchors).
//!
//! v0's one anchor kind is `fn-seat`: the cited chokepoint function. A seat is not minted
//! here — it is the chokepoint the house architecture already mandates per behaviour
//! (`relational-compare-chokepoint`, `selector-chokepoint`, one-named-seat-per-invariant),
//! cited by this catalogue. Resolving it is a CHECKED CONFIRMATION rather than new
//! information, which is exactly why the check is worth having: it is the thing that goes red
//! when a rename moves the chokepoint out from under a law.
//!
//! The seat's three consumers are each simple because a seat is one function — the reach
//! check (one boolean region-hit), the mutant scope (one filtered function), and the rustdoc
//! backlink. Only resolution is built at v0; the other two are named seams.

use std::path::{Path, PathBuf};

/// Where the cited seat was found.
#[derive(Clone, Debug)]
pub struct Resolved {
    /// Repo-relative path to the file declaring it.
    pub file: String,
}

/// Resolve `seat`, spelled `dorc_<crate>::<module>::…::<fn>`.
///
/// # Errors
/// When the crate, the module file, or the function cannot be found. The message names which
/// of the three failed, because "seat unresolved" alone sends a reader to the wrong place.
pub fn resolve(seat: &str, repo_root: &Path) -> Result<Resolved, String> {
    let mut segments = seat.split("::");
    let crate_seg = segments
        .next()
        .ok_or_else(|| format!("{seat}: empty citation"))?;
    let rest: Vec<&str> = segments.collect();
    let (module, function) = match rest.as_slice() {
        [] => return Err(format!("{seat}: names a crate but no function")),
        [.., function] => (rest.first().copied().unwrap_or(""), *function),
    };
    let crate_dir = crate_dir(crate_seg, repo_root)
        .ok_or_else(|| format!("{seat}: no crate directory for `{crate_seg}`"))?;
    let module_file = crate_dir.join("src").join(format!("{module}.rs"));
    if !module_file.is_file() {
        return Err(format!(
            "{seat}: no module file {}",
            relative(repo_root, &module_file)
        ));
    }
    let text = std::fs::read_to_string(&module_file)
        .map_err(|e| format!("{}: {e}", module_file.display()))?;
    if declares_fn(&text, function) {
        Ok(Resolved {
            file: relative(repo_root, &module_file),
        })
    } else {
        Err(format!(
            "{seat}: {} declares no `fn {function}`",
            relative(repo_root, &module_file)
        ))
    }
}

/// `dorc_core` → `spike/crates/core`. The `dorc_` prefix is the crate-name convention, not part
/// of the directory.
fn crate_dir(crate_seg: &str, repo_root: &Path) -> Option<PathBuf> {
    let bare = crate_seg.strip_prefix("dorc_").unwrap_or(crate_seg);
    let dir = repo_root
        .join("spike")
        .join("crates")
        .join(bare.replace('_', "-"));
    dir.is_dir().then_some(dir)
}

/// Whether `text` declares `fn <name>` — trait signature or inherent definition alike, since a
/// seat may be either.
fn declares_fn(text: &str, name: &str) -> bool {
    text.lines().any(|line| {
        line.split_whitespace()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|w| w.first() == Some(&"fn") && w.get(1).is_some_and(|n| starts_call(n, name)))
    })
}

fn starts_call(spelled: &str, name: &str) -> bool {
    spelled
        .strip_prefix(name)
        .is_some_and(|tail| tail.is_empty() || tail.starts_with('(') || tail.starts_with('<'))
}

fn relative(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_declaration_is_recognized_in_both_seat_shapes() {
        // A seat is either a trait's signature or an inherent definition; a checker that saw
        // only one would silently pass a citation naming the other.
        assert!(declares_fn(
            "    fn join(&self, other: &Self) -> Self;",
            "join"
        ));
        assert!(declares_fn(
            "    pub fn insert(&mut self, value: T) -> bool {",
            "insert"
        ));
        assert!(declares_fn(
            "fn from_iter<I: IntoIterator<Item = T>>(i: I)",
            "from_iter"
        ));
        assert!(!declares_fn("    let joined = out.get(k).join(v);", "join"));
        assert!(!declares_fn("    fn joinery(&self) {}", "join"));
    }
}
