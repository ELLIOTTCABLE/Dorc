//! The filesystem edge for the posthoc-why durable -- the `28D` hardening bill's write half.
//!
//! `28D:must-default-durable-lands-with-its-hardening` bills a durable that is written on the
//! product's own initiative for exclusive creation, a restrictive mode, atomic replacement, bounded
//! reads, a trusted-directory rule, visible persistence failure, and a stated sensitivity contract.
//! The `28F` W3 ruling took the gate's OPT-IN branch -- the whylog stays behind `--whylog-dir` -- and
//! landed the bill anyway, because five of those seven were owed on the opt-in path regardless: two
//! concurrent runs silently truncated each other's durable, and every failure returned quietly.
//!
//! # Why this duplicates `dorc-loom`'s `FsReceiptStore` (`28F:rul-safe-store-is-cli-local`)
//!
//! `crates/dorc-loom/src/receipt_store.rs` already solves the same shapes, better-tested, and it is
//! unreachable from here: `dorc-loom` depends on `dorc-cli` (the worldless-route parser seat), so a
//! `cli -> dorc-loom` edge is a dependency cycle. `churn-avoidance-disclosure`: this module is a
//! deliberate ~100-line duplication of well-understood shapes rather than a shared crate, and a
//! third consumer is the trigger to extract one. Read `FsReceiptStore` before changing anything
//! here -- it is the reference, and it is stricter where it can afford to be.
//!
//! # Atomic replacement, and why none appears (`28D`'s third item)
//!
//! Nothing is ever replaced. Each run publishes a NEW `whylog-<NNNN>.txt`, so the operation is a
//! creation, and [`publish`]'s exclusive create IS its atomicity. `FsReceiptStore` pays ~150 lines
//! for a temp-then-rename dance (and a whole `#[cfg(windows)]` backup arm) only because it
//! republishes one stable name. Should the durable ever gain a stable `latest` entry point, that
//! cost arrives with it.

use std::path::{Component, Path, PathBuf};

/// The bounded window of candidate names one publish may try before giving up.
///
/// A taken name means a concurrent run won the race for that index; walking forward a few slots
/// resolves it. Unbounded retry would spin against a directory someone is filling deliberately.
const NAME_ATTEMPTS: u64 = 16;

/// The most `whylog-<NNNN>.txt` entries one directory scan will collect.
///
/// `rul-host-bytes-bounded-before-admission` bounds what a managed host produced; this bounds what
/// the local filesystem hands us, on the same reasoning -- the scan runs on every plan, apply, and
/// `why`, and a directory someone filled should cost a refusal rather than an allocation.
const MAX_ENTRIES: usize = 4096;

/// Why a durable was not persisted. Closed, and each arm is separately reportable.
///
/// `28F:rul-write-failure-is-error-floor`: these reach the user rather than vanishing. The write
/// path used to answer every one of them with a bare `return`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PersistRefusal {
    /// The named directory could not be created, or is not a directory we will write into.
    Directory,
    /// Every candidate name in the attempt window was already taken.
    NamesExhausted,
    /// The durable exceeded the retention byte cap.
    Oversize,
    /// Creating, writing, or flushing the durable failed.
    Write,
}

impl PersistRefusal {
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

/// Write `bytes` as the next durable in `dir`, then prune to the newest `keep`.
///
/// # Errors
///
/// Returns the closed [`PersistRefusal`] without leaving a partial durable behind.
pub(crate) fn publish(
    dir: &str,
    bytes: &[u8],
    cap: usize,
    keep: usize,
) -> Result<PathBuf, PersistRefusal> {
    if bytes.len() > cap {
        return Err(PersistRefusal::Oversize);
    }
    let dir = trusted_directory(dir)?;
    let mut next = entries(&dir)
        .last()
        .map_or(1, |(index, _)| index.saturating_add(1));
    for _ in 0..NAME_ATTEMPTS {
        let path = dir.join(durable_name(next));
        match create_exclusive(&path) {
            Ok(mut file) => {
                if write_and_flush(&mut file, bytes).is_err() {
                    drop(file);
                    // Our own partial byte-run, created this call and held until now: removing it
                    // is `rul-probe-writes-only-what-it-owns`'s ownership test passing, not an
                    // exception to it. A truncated durable that survived would replay as corrupt.
                    let _ = std::fs::remove_file(&path);
                    return Err(PersistRefusal::Write);
                }
                prune(&dir, keep);
                return Ok(path);
            }
            Err(taken) if taken.kind() == std::io::ErrorKind::AlreadyExists => {
                next = next.saturating_add(1);
            }
            Err(_) => return Err(PersistRefusal::Write),
        }
    }
    Err(PersistRefusal::NamesExhausted)
}

/// The durables in `dir`, ascending by run-index, bounded by [`MAX_ENTRIES`].
pub(crate) fn entries(dir: &Path) -> Vec<(u64, PathBuf)> {
    let mut found: Vec<(u64, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            Some((
                durable_index(&entry.file_name().to_string_lossy())?,
                entry.path(),
            ))
        })
        .take(MAX_ENTRIES)
        .collect();
    found.sort_by_key(|(index, _)| *index);
    found
}

/// The newest durable in `dir` (highest run-index), or `None`.
pub(crate) fn newest(dir: &str) -> Option<PathBuf> {
    entries(Path::new(dir)).pop().map(|(_, path)| path)
}

/// `whylog-<NNNN>.txt` for a run-index.
fn durable_name(index: u64) -> String {
    format!("whylog-{index:04}.txt")
}

/// The run-index a durable's filename encodes, or `None` when the name is not ours.
fn durable_index(name: &str) -> Option<u64> {
    name.strip_prefix("whylog-")?
        .strip_suffix(".txt")?
        .parse()
        .ok()
}

/// Ensure `dir` exists and is a directory we are willing to write into.
///
/// # The rule, and the honest limit of it (`28D`'s trusted-directory item)
///
/// Checked: the path names no `..` traversal, the directory exists as a REAL directory, and
/// [`symlink_metadata`](std::fs::symlink_metadata) -- never a following stat -- is what says so, so a
/// link planted where the directory belongs is refused rather than written through. The durable
/// itself is then created exclusively, which is what actually defeats a link planted at the FILE
/// name: `O_CREAT|O_EXCL` fails on an existing path even when it is a dangling symlink.
///
/// NOT checked: every ancestor up to the filesystem root. `FsReceiptStore::validate_directory_tree`
/// does walk them, and it can, because its root is a repository-relative path Git handed it. Ours
/// is whatever the admin typed, and a full ancestor walk refuses ordinary systems: macOS resolves
/// `std::env::temp_dir()` under `/var`, which IS a symlink to `/private/var`, so the walk would
/// reject the platform's own temp root -- and with it every loom case. A rule that must be disabled
/// to run is not a rule (`271:rul-net-quality-u-curve`).
fn trusted_directory(dir: &str) -> Result<PathBuf, PersistRefusal> {
    let path = Path::new(dir);
    if path
        .components()
        .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(PersistRefusal::Directory);
    }
    if !path.exists() && create_directory(path).is_err() {
        return Err(PersistRefusal::Directory);
    }
    let metadata = std::fs::symlink_metadata(path).map_err(|_| PersistRefusal::Directory)?;
    if !metadata.is_dir() || is_reparse_point(&metadata) {
        return Err(PersistRefusal::Directory);
    }
    Ok(path.to_path_buf())
}

/// Does this metadata describe a link-like object rather than the thing it names?
///
/// `is_dir()` alone is not enough on Windows: a directory junction reports as a directory and
/// redirects anyway, so the reparse-point attribute has to be read directly (`FsReceiptStore`'s
/// `unsafe_metadata`, same reasoning, same constant).
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
///
/// The mode rides the SAME `open(2)` as the creation, so the durable never exists group- or
/// world-readable for even an instant; a `set_permissions` afterwards would leave that window open.
#[cfg(unix)]
fn create_exclusive(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

/// Create `path`, failing if anything already occupies the name.
///
/// # The mode this cannot set, said plainly (`28D`'s restrictive-mode item)
///
/// Windows has no mode, and the durable therefore inherits its directory's ACL. `set_readonly`
/// looks like the answer and is not: it sets `FILE_ATTRIBUTE_READONLY`, which restricts nobody --
/// promising confidentiality with it would be exactly the unkeepable promise
/// `28D:must-split-the-bundled-entries` forbids. A real DACL needs `SetNamedSecurityInfo`, i.e. an
/// FFI dependency, and `inv-no-unsafe` forbids FFI workspace-wide over a graph with zero
/// third-party crates; lifting that is a design event, not a builder's call.
///
/// So the Windows posture is siting, not permissions: keep the durable under a per-user profile
/// root whose inherited ACL is already user-only, and state the limit in the sensitivity contract
/// rather than implying a protection that is not there (`AID-NEEDS:law-whylog-is-sensitive`).
#[cfg(not(unix))]
fn create_exclusive(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
}

/// Create the durable directory user-only.
#[cfg(unix)]
fn create_directory(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt as _;
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(path)
}

/// Create the durable directory (see [`create_exclusive`] for the mode Windows cannot carry).
#[cfg(not(unix))]
fn create_directory(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

fn write_and_flush(file: &mut std::fs::File, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    file.write_all(bytes)?;
    file.flush()
}

/// Drop all but the newest `keep` durables.
///
/// Failures are ignored deliberately: an unremoved durable RETAINS data, and the failure direction
/// that matters for a postmortem aid is losing it. `28D:must-retention-is-one-decision` owns the
/// question of what `keep` should be at all; this only enforces whatever it is told.
fn prune(dir: &Path, keep: usize) {
    let found = entries(dir);
    if let Some(excess) = found.len().checked_sub(keep) {
        for (_, path) in found.into_iter().take(excess) {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway directory under the platform temp root, removed on drop.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("dorc-whylog-store-{name}"));
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

    /// The bug exclusive creation exists to kill: the old writer computed `max index + 1` and then
    /// `fs::write`, so two runs racing on one directory both chose the same name and the loser's
    /// receipt was silently truncated away. Publishing twice must yield two distinct durables with
    /// both byte-runs intact -- the property a plain create-and-truncate cannot offer.
    #[test]
    fn a_second_publish_never_overwrites_the_first() {
        let scratch = Scratch::new("second-publish-never-overwrites");
        let first = publish(&scratch.dir(), b"first", 1024, 5).expect("first durable");
        let second = publish(&scratch.dir(), b"second", 1024, 5).expect("second durable");
        assert_ne!(first, second, "each run publishes its own name");
        assert_eq!(std::fs::read(&first).expect("first bytes"), b"first");
        assert_eq!(std::fs::read(&second).expect("second bytes"), b"second");
    }

    /// A name already taken must be stepped over rather than clobbered -- the concurrent-run case,
    /// simulated by planting the name the index scan would otherwise choose.
    #[test]
    fn a_taken_name_is_stepped_over_and_left_intact() {
        let scratch = Scratch::new("taken-name-stepped-over");
        let squatted = Path::new(&scratch.dir()).join("whylog-0001.txt");
        std::fs::write(&squatted, b"not ours").expect("squatted durable");
        let published = publish(&scratch.dir(), b"ours", 1024, 5).expect("durable");
        assert_ne!(published, squatted);
        assert_eq!(
            std::fs::read(&squatted).expect("squatted bytes"),
            b"not ours"
        );
    }

    /// Retention keeps the NEWEST durables. Getting the end wrong would delete the receipt for the
    /// run the admin is standing in front of, which is the only one they are certain to want.
    #[test]
    fn pruning_keeps_the_newest_durables() {
        let scratch = Scratch::new("pruning-keeps-newest");
        for run in 0..4_u8 {
            publish(&scratch.dir(), &[b'a', run], 1024, 2).expect("durable");
        }
        let remaining = entries(Path::new(&scratch.dir()));
        assert_eq!(remaining.len(), 2, "keep=2 leaves two");
        assert_eq!(
            remaining
                .iter()
                .map(|(index, _)| *index)
                .collect::<Vec<_>>(),
            vec![3, 4],
            "the two survivors are the newest indices"
        );
    }

    /// An oversized durable is refused before anything is created, so a cap breach cannot leave a
    /// half-file that would later replay as corrupt.
    #[test]
    fn an_oversized_durable_is_refused_before_creation() {
        let scratch = Scratch::new("oversized-refused-before-creation");
        assert_eq!(
            publish(&scratch.dir(), b"too long", 2, 5),
            Err(PersistRefusal::Oversize)
        );
        assert!(entries(Path::new(&scratch.dir())).is_empty());
    }

    /// A traversing path is refused outright: the durable's directory is an admin-typed value, and
    /// a `..` in it is either a mistake or an attempt to escape, neither of which we should serve.
    #[test]
    fn a_traversing_directory_is_refused() {
        let scratch = Scratch::new("traversing-directory-refused");
        let escaping = format!("{}/../elsewhere", scratch.dir());
        assert_eq!(
            publish(&escaping, b"payload", 1024, 5),
            Err(PersistRefusal::Directory)
        );
    }

    /// A file where the directory belongs is refused rather than being turned into one.
    #[test]
    fn a_non_directory_at_the_named_path_is_refused() {
        let scratch = Scratch::new("non-directory-refused");
        let occupied = Path::new(&scratch.dir()).join("occupied");
        std::fs::write(&occupied, b"a file, not a directory").expect("occupant");
        assert_eq!(
            publish(&occupied.to_string_lossy(), b"payload", 1024, 5),
            Err(PersistRefusal::Directory)
        );
        assert_eq!(
            std::fs::read(&occupied).expect("occupant bytes"),
            b"a file, not a directory"
        );
    }

    /// The durable and its directory are user-only on unix. Worth pinning rather than trusting the
    /// call site, because the failure is SILENT: a durable written 0644 reads exactly like one
    /// written 0600 to every test that only checks its bytes, and host metadata
    /// (`AID-NEEDS:law-whylog-is-sensitive`) would sit world-readable with nothing complaining.
    /// The umask cannot mask a pass here — it only ever REMOVES bits, so an over-permissive result
    /// still fails.
    #[cfg(unix)]
    #[test]
    fn the_durable_and_its_directory_are_user_only() {
        use std::os::unix::fs::PermissionsExt as _;

        let scratch = Scratch::new("durable-and-directory-are-user-only");
        let nested = Path::new(&scratch.dir()).join("made-by-us");
        let published = publish(&nested.to_string_lossy(), b"sensitive", 1024, 5).expect("durable");
        let mode = |path: &Path| {
            std::fs::metadata(path)
                .expect("metadata")
                .permissions()
                .mode()
                & 0o777
        };
        assert_eq!(mode(&published), 0o600, "the durable is user-only");
        assert_eq!(mode(&nested), 0o700, "so is the directory we created");
    }

    /// A symlinked durable directory is refused WITHOUT following it. The attack this closes is
    /// planting a link at a directory the admin will later name, so the receipt -- host metadata by
    /// `AID-NEEDS:law-whylog-is-sensitive` -- lands somewhere the planter can read.
    #[cfg(unix)]
    #[test]
    fn a_linked_directory_is_refused_without_being_followed() {
        let scratch = Scratch::new("linked-directory-refused");
        let real = Path::new(&scratch.dir()).join("real");
        std::fs::create_dir(&real).expect("real directory");
        let linked = Path::new(&scratch.dir()).join("linked");
        std::os::unix::fs::symlink(&real, &linked).expect("directory link");
        assert_eq!(
            publish(&linked.to_string_lossy(), b"payload", 1024, 5),
            Err(PersistRefusal::Directory)
        );
        assert!(entries(&real).is_empty(), "nothing was written through");
    }
}
