//! The one seat that answers "where is my world".
//!
//! The corpus, both generated locks, the staging store and the repository this tool asks about
//! all hang off ONE directory. Four separate sites used to re-derive it from the crate's own
//! compile-time anchor, which is how the second copy of a resolution silently rots
//! (`spike/CLAUDE.md` one-shell-answer says the same thing about interpreters). It is also what
//! made the `publish` write path untestable: reaching it wrote REAL sources, so three specified
//! tests could not be written and a developer's in-progress loom edit could be published by
//! `cargo test`.
//!
//! Nothing outside this module may name that anchor —
//! `only_the_resolution_seat_names_an_absolute_anchor` in `tests/roots.rs` is the gate, and it
//! reads this crate's sources lexically because the property is "no other seat can even spell
//! it", which no type bound expresses.

use std::path::{Path, PathBuf};

/// Every location `dorc-loom` reads or writes, derived from one base directory.
///
/// The base is `spike/` — the tree holding `crates/` and `target/`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Roots {
    base: PathBuf,
}

impl Roots {
    /// Where this invocation's world is: the tree named by the global `-C`, or the one this
    /// binary was built inside.
    ///
    /// # Errors
    /// Returns a refusal when `-C` names something that is not a readable directory, or when the
    /// built-in anchor cannot be walked back to `spike/`.
    pub fn resolve(named: Option<&str>) -> Result<Self, String> {
        match named {
            Some(dir) => Self::at(dir),
            None => Self::built_in(),
        }
    }

    /// The tree this binary's own sources live in — the default, and the only absolute anchor in
    /// the crate.
    ///
    /// Deliberately NOT canonicalized: the compile-time anchor is already a plain absolute path,
    /// and canonicalizing it would move every path this tool prints (on Windows into `\\?\`
    /// spelling) for no gain.
    ///
    /// # Errors
    /// Returns a refusal when the anchor has no grandparent, which would mean the crate has been
    /// re-homed out of `spike/crates/<c>`.
    pub fn built_in() -> Result<Self, String> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .map(|base| Self {
                base: base.to_path_buf(),
            })
            .ok_or_else(|| "locate spike dir".to_owned())
    }

    /// The tree named by `-C`.
    ///
    /// Canonicalized, because this one is typed by a person: a relative spelling, a `..`, or a
    /// symlinked parent would otherwise reach the staging store's directory-tree check as an
    /// "unsafe" refusal that names nothing the caller typed.
    ///
    /// # Errors
    /// Returns a refusal when the path does not resolve to a readable directory.
    pub fn at(dir: &str) -> Result<Self, String> {
        let base = std::fs::canonicalize(dir)
            .map_err(|error| format!("-C {dir}: {error}"))
            .and_then(|base| {
                base.is_dir()
                    .then_some(base)
                    .ok_or_else(|| format!("-C {dir}: not a directory"))
            })?;
        Ok(Self { base })
    }

    /// The tree everything below hangs off.
    #[must_use]
    pub fn base(&self) -> &Path {
        &self.base
    }

    /// The primary loom collection (`aid/CLAUDE.md` cases-live-here).
    #[must_use]
    pub fn corpus(&self) -> PathBuf {
        self.under("crates/aid/tests")
    }

    /// The generated catalog lock.
    #[must_use]
    pub fn catalog_lock(&self) -> PathBuf {
        self.under("crates/aid/src/catalog_lock.rs")
    }

    /// The generated arrangement lock.
    #[must_use]
    pub fn arrangement_lock(&self) -> PathBuf {
        self.under("crates/aid/src/arrangement_lock.rs")
    }

    /// The trusted directory a refusing publish stages its interpretation under.
    #[must_use]
    pub fn staging_root(&self) -> PathBuf {
        self.under("target")
    }

    /// Component-wise, so a joined answer carries the platform's own separator throughout: a
    /// `base.join("crates/aid/tests")` prints `…\spike\crates/aid/tests` on Windows, and these
    /// paths are printed at every refusal.
    fn under(&self, relative: &str) -> PathBuf {
        relative
            .split('/')
            .fold(self.base.clone(), |path, segment| path.join(segment))
    }
}
