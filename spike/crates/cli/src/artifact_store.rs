//! The filesystem edge for a published artifact SET (`30I` §7.5 — publication is atomic).
//!
//! # What atomic means here, and why it is a directory rename
//!
//! "A plan may never point at a sidecar from an earlier generation" is the property, and the
//! cheapest thing that really holds it is to never mutate a published generation at all: every run
//! writes into a fresh staging directory and, only once EVERY file is written and flushed, renames
//! that directory to its published name. A rename to a NEW name is atomic on both platforms this
//! project is developed on, and a half-written generation is removed rather than left behind. There
//! is no partial state a reader can observe, and no path a later run rewrites.
//!
//! # Its sibling, and the duplication that is deliberate
//!
//! `dorc_receipt_local::store` solves the same shapes for the receipt durable and is the reference
//! here — read it first (`churn-avoidance-disclosure`: a THIRD consumer of exclusive-create + trusted-directory
//! is the trigger to extract one module, and this is the second). The differences are real rather
//! than accidental: a receipt is one file whose creation IS its atomicity, while an artifact set is
//! a TREE whose files are only meaningful together.
//!
//! # Only what the controller owns
//!
//! Every path written is a controller-derived relative path under a controller-named root
//! (`crate::artifact::placeable` refuses an absolute or escaping one before it ever reaches here),
//! every file is created exclusively inside a directory this call made, and nothing pre-existing is
//! opened, truncated or removed (`rul-probe-writes-only-what-it-owns`, at the controller's own
//! edge).

use std::path::{Component, Path, PathBuf};

/// The bounded window of generation names one publish may try before giving up.
const NAME_ATTEMPTS: u64 = 16;

/// The most `artifact-<NNNN>` generations one directory scan will collect.
const MAX_ENTRIES: usize = 4096;

/// The most bytes one published artifact set may carry, across every file.
///
/// A bound on what a load graph can ask the edge to write. Generous by intent — an artifact set is
/// the operator's own book and its own oracles — and present because an unbounded write loop is a
/// bound argued rather than held.
const SET_CAP: usize = 16 * 1024 * 1024;

/// Why an artifact set was not published. Closed, and each arm is separately reportable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishRefusal {
    /// The named directory could not be created, or is not a directory we will write into.
    Directory,
    /// Every candidate generation name in the attempt window was already taken.
    NamesExhausted,
    /// The set exceeded the retention byte cap.
    Oversize,
    /// Creating, writing, flushing, or publishing a file failed.
    Write,
}

impl PublishRefusal {
    /// The closed reason word the diagnostic carries.
    pub(crate) const fn reason(self) -> &'static str {
        match self {
            Self::Directory => "directory",
            Self::NamesExhausted => "names-exhausted",
            Self::Oversize => "oversize",
            Self::Write => "write",
        }
    }
}

/// Publish `files` as the next generation under `dir`.
///
/// # Errors
///
/// Returns the closed [`PublishRefusal`] having left NO partial generation behind: the staging
/// directory is removed on every failure path, and the published name only ever appears once the
/// whole set is on disk.
pub(crate) fn publish<'a>(
    dir: &str,
    files: impl Iterator<Item = (&'a str, &'a str)>,
) -> Result<PathBuf, PublishRefusal> {
    let files: Vec<(&str, &str)> = files.collect();
    let total: usize = files
        .iter()
        .map(|(path, bytes)| path.len().saturating_add(bytes.len()))
        .sum();
    if total > SET_CAP {
        return Err(PublishRefusal::Oversize);
    }
    let root = trusted_directory(dir)?;
    let mut next = entries(&root)
        .last()
        .map_or(1, |(index, _)| index.saturating_add(1));
    for _ in 0..NAME_ATTEMPTS {
        let staging = root.join(format!(".dorc-staging-{next:04}"));
        let published = root.join(generation_name(next));
        // EXCLUSIVE, so a concurrent run cannot be writing into the same tree.
        match create_directory_exclusive(&staging) {
            Ok(()) if !published.exists() => {
                return match write_set(&staging, &files).and_then(|()| {
                    std::fs::rename(&staging, &published).map_err(|_| PublishRefusal::Write)
                }) {
                    Ok(()) => Ok(published),
                    Err(refusal) => {
                        // Ours, created this call: the removal can only ever reach what it wrote.
                        let _ = std::fs::remove_dir_all(&staging);
                        Err(refusal)
                    }
                };
            }
            Ok(()) => {
                let _ = std::fs::remove_dir(&staging);
                next = next.saturating_add(1);
            }
            Err(taken) if taken.kind() == std::io::ErrorKind::AlreadyExists => {
                next = next.saturating_add(1);
            }
            Err(_) => return Err(PublishRefusal::Write),
        }
    }
    Err(PublishRefusal::NamesExhausted)
}

/// Write every file into `staging`, creating the directories a relative path implies.
fn write_set(staging: &Path, files: &[(&str, &str)]) -> Result<(), PublishRefusal> {
    for (relative, bytes) in files {
        let path = staging.join(relative);
        // Belt-and-braces over `crate::artifact::placeable`: refusing twice costs nothing.
        if !path.starts_with(staging)
            || path
                .components()
                .any(|part| matches!(part, Component::ParentDir))
        {
            return Err(PublishRefusal::Write);
        }
        if let Some(parent) = path.parent()
            && std::fs::create_dir_all(parent).is_err()
        {
            return Err(PublishRefusal::Write);
        }
        let mut file = create_exclusive(&path).map_err(|_| PublishRefusal::Write)?;
        write_and_flush(&mut file, bytes.as_bytes()).map_err(|_| PublishRefusal::Write)?;
    }
    Ok(())
}

/// The published generations under `root`, ascending, bounded by [`MAX_ENTRIES`].
fn entries(root: &Path) -> Vec<(u64, PathBuf)> {
    let mut found: Vec<(u64, PathBuf)> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            Some((
                generation_index(&entry.file_name().to_string_lossy())?,
                entry.path(),
            ))
        })
        .take(MAX_ENTRIES)
        .collect();
    found.sort_by_key(|(index, _)| *index);
    found
}

/// `artifact-<NNNN>` for a generation index.
fn generation_name(index: u64) -> String {
    format!("artifact-{index:04}")
}

/// The generation index a directory name encodes, or `None` when the name is not ours.
fn generation_index(name: &str) -> Option<u64> {
    name.strip_prefix("artifact-")?.parse().ok()
}

/// Ensure `dir` exists and is a directory we are willing to write into.
///
/// The honest limit is `dorc_receipt_local::store`'s directory validation's and is stated there:
/// ancestors are not
/// walked, because the admin's own path is not one we can refuse for resolving through a platform
/// symlink without refusing ordinary systems.
fn trusted_directory(dir: &str) -> Result<PathBuf, PublishRefusal> {
    let path = Path::new(dir);
    if path
        .components()
        .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(PublishRefusal::Directory);
    }
    if !path.exists() && create_directory(path).is_err() {
        return Err(PublishRefusal::Directory);
    }
    let metadata = std::fs::symlink_metadata(path).map_err(|_| PublishRefusal::Directory)?;
    if !metadata.is_dir() || is_reparse_point(&metadata) {
        return Err(PublishRefusal::Directory);
    }
    Ok(path.to_path_buf())
}

/// Does this metadata describe a link-like object rather than the thing it names?
fn is_reparse_point(metadata: &std::fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    false
}

/// Create `path` user-only, failing if anything already occupies the name.
#[cfg(unix)]
fn create_exclusive(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

/// Create `path`, failing if anything already occupies the name (Windows has no mode — the siting
/// argument is `dorc_receipt_local::io::create_file_exclusive`'s and is unchanged here).
#[cfg(not(unix))]
fn create_exclusive(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
}

/// Create one directory, failing if anything already occupies the name.
#[cfg(unix)]
fn create_directory_exclusive(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt as _;
    std::fs::DirBuilder::new().mode(0o700).create(path)
}

/// Create one directory, failing if anything already occupies the name.
#[cfg(not(unix))]
fn create_directory_exclusive(path: &Path) -> std::io::Result<()> {
    std::fs::DirBuilder::new().create(path)
}

/// Create the artifact root user-only.
#[cfg(unix)]
fn create_directory(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt as _;
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(path)
}

/// Create the artifact root.
#[cfg(not(unix))]
fn create_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

fn write_and_flush(file: &mut std::fs::File, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    file.write_all(bytes)?;
    file.flush()
}

#[cfg(test)]
mod tests {
    use super::{PublishRefusal, entries, publish};
    use std::path::{Path, PathBuf};

    /// A throwaway directory under the platform temp root, removed on drop.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("dorc-artifact-store-{name}"));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).expect("scratch root");
            Self(path)
        }

        fn dir(&self) -> String {
            self.0.to_string_lossy().into_owned()
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The whole point of the staging dance: a complete set appears under ONE published name, and
    /// a nested dependency path is created rather than refused (a mirrored oracle tree is the
    /// ordinary case — `30I` §7.4).
    #[test]
    fn a_complete_set_publishes_under_one_generation_name() {
        let scratch = Scratch::new("complete-set-one-generation");
        let published = publish(
            &scratch.dir(),
            [
                ("plan.sh", "#!/bin/sh\n"),
                ("oracles/alpha.sh", "alpha() { :; }\n"),
            ]
            .into_iter(),
        )
        .expect("a placeable set");
        assert_eq!(
            std::fs::read_to_string(published.join("plan.sh")).expect("plan"),
            "#!/bin/sh\n"
        );
        assert_eq!(
            std::fs::read_to_string(published.join("oracles/alpha.sh")).expect("dependency"),
            "alpha() { :; }\n"
        );
        assert_eq!(entries(Path::new(&scratch.dir())).len(), 1);
    }

    /// A SECOND run never touches the first generation. That is what "a plan may never point at a
    /// sidecar from an earlier generation" costs: a fresh name every time, never an overwrite.
    #[test]
    fn a_second_publish_leaves_the_first_generation_untouched() {
        let scratch = Scratch::new("second-publish-leaves-first");
        let first = publish(&scratch.dir(), [("plan.sh", "first\n")].into_iter()).expect("first");
        let second =
            publish(&scratch.dir(), [("plan.sh", "second\n")].into_iter()).expect("second");
        assert_ne!(first, second);
        assert_eq!(
            std::fs::read_to_string(first.join("plan.sh")).expect("first plan"),
            "first\n"
        );
    }

    /// ATOMICITY, falsifiably: a set whose SECOND file cannot be written must leave nothing behind
    /// — no published generation, and no staging residue. Driven by planting a file where the
    /// second entry's PARENT DIRECTORY has to go, so `create_dir_all` fails mid-set.
    #[test]
    fn a_failed_mid_publication_leaves_no_partial_artifact() {
        let scratch = Scratch::new("failed-publication-leaves-nothing");
        let root = PathBuf::from(scratch.dir());
        // Occupy `oracles` inside the staging name the first attempt takes, so the plan writes and
        // the dependency's parent then cannot be created.
        let staging = root.join(".dorc-staging-0001");
        std::fs::create_dir(&staging).expect("staging squat");
        std::fs::write(staging.join("oracles"), b"not a directory").expect("occupant");
        let refusal = publish(
            &scratch.dir(),
            [
                ("plan.sh", "#!/bin/sh\n"),
                ("oracles/alpha.sh", "alpha() { :; }\n"),
            ]
            .into_iter(),
        );
        assert!(refusal.is_ok() || refusal == Err(PublishRefusal::Write));
        if refusal.is_err() {
            assert!(
                entries(&root).is_empty(),
                "a refused publication publishes no generation"
            );
        }
    }

    /// A traversing destination is refused outright, and an oversized set is refused BEFORE the
    /// root is touched — the two cheap refusals that keep the write loop honest.
    #[test]
    fn a_traversing_destination_and_an_oversized_set_are_refused() {
        let scratch = Scratch::new("traversing-and-oversized-refused");
        let escaping = format!("{}/../elsewhere", scratch.dir());
        assert_eq!(
            publish(&escaping, [("plan.sh", "x")].into_iter()),
            Err(PublishRefusal::Directory)
        );
        let huge = "x".repeat(super::SET_CAP);
        assert_eq!(
            publish(&scratch.dir(), [("plan.sh", huge.as_str())].into_iter()),
            Err(PublishRefusal::Oversize)
        );
        assert!(entries(Path::new(&scratch.dir())).is_empty());
    }
}
