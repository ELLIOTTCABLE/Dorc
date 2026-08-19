//! Narrow staging persistence edge. Packet construction and validation stay pure.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use crate::staging::parse as parse_staging;
use crate::{MAX_STAGING_BYTES, StagingError};

const STAGING_DIRECTORY: &str = "dorc-loom";
const STAGING_FILE: &str = "staged.publication";
const STAGING_BACKUP_FILE: &str = ".staged.publication.backup";
const TEMP_ATTEMPTS: u8 = 16;

/// Reports whether a stored packet still has cleanup work to retry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StagingWriteOutcome {
    /// The packet was stored and no stale backup remains.
    Published,
    /// The packet was stored, but a validated stale backup could not be removed.
    CleanupPending,
}

trait StagingFileOperations: Send + Sync {
    fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()>;
    /// Windows-only: the backup dance below is the sole caller (see [`FsStagingStore::publish`]).
    #[cfg(windows)]
    fn remove_file(&self, path: &Path) -> std::io::Result<()>;
}

struct NativeStagingFileOperations;

impl StagingFileOperations for NativeStagingFileOperations {
    fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        fs::rename(from, to)
    }

    #[cfg(windows)]
    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        fs::remove_file(path)
    }
}

/// The filesystem boundary a refusing publish stages through, and `--verbatim` reads back.
pub trait StagingStore {
    /// Store an already encoded and validated packet.
    ///
    /// # Errors
    ///
    /// Returns an I/O or staging-validation refusal without publishing partial bytes.
    fn publish(&self, packet: &[u8]) -> Result<StagingWriteOutcome, String>;
    /// Read one bounded, grammar-validated stored packet, or `None` when none is
    /// stored. Absence is a STATE, not a failure — it is the ordinary "nothing has been
    /// staged" case, and the caller owes its user a different sentence for it
    /// than for a corrupt or unreadable store.
    ///
    /// # Errors
    ///
    /// Returns an I/O, unsafe-path, size, or staging-validation refusal.
    fn read(&self) -> Result<Option<Vec<u8>>, String>;
    /// Drop the stored packet, if any. An applied interpretation has been spent: leaving it would
    /// let a second `--verbatim` re-confirm a loss the author already accepted once.
    ///
    /// # Errors
    ///
    /// Returns an I/O or unsafe-path refusal. Absence is success — the goal is that nothing is
    /// staged, and it already is not.
    fn discard(&self) -> Result<(), String>;
}

/// Worktree-local staging storage under one validated ignored target root.
#[derive(Clone)]
pub struct FsStagingStore {
    target_root: PathBuf,
    operations: Arc<dyn StagingFileOperations>,
}

impl PartialEq for FsStagingStore {
    fn eq(&self, other: &Self) -> bool {
        self.target_root == other.target_root
    }
}

impl Eq for FsStagingStore {}

impl std::fmt::Debug for FsStagingStore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FsStagingStore")
            .field("target_root", &self.target_root)
            .finish_non_exhaustive()
    }
}

impl FsStagingStore {
    /// Bind the fixed staging location below an existing trusted target root.
    ///
    /// # Errors
    ///
    /// Refuses a missing, linked, reparse-point, or non-directory target root.
    pub fn new(target_root: impl Into<PathBuf>) -> Result<Self, String> {
        let target_root = target_root.into();
        validate_directory_tree(&target_root, "staging target root")?;
        Ok(Self {
            target_root,
            operations: Arc::new(NativeStagingFileOperations),
        })
    }

    /// Where a stored packet lands, so a refusing publish can name the durable state it left behind.
    #[must_use]
    pub fn path(&self) -> PathBuf {
        Self::final_path(&self.target_root.join(STAGING_DIRECTORY))
    }

    fn staging_directory(&self, create: bool) -> Result<PathBuf, String> {
        validate_directory_tree(&self.target_root, "staging target root")?;
        let directory = self.target_root.join(STAGING_DIRECTORY);
        if create {
            ensure_directory(&directory)?;
        } else {
            validate_directory_tree(&directory, "staging directory")?;
        }
        validate_directory_tree(&self.target_root, "staging target root")?;
        validate_directory_tree(&directory, "staging directory")?;
        Ok(directory)
    }

    /// The staging directory, or `None` where nothing has ever been staged here.
    ///
    /// An absent directory is the ordinary first-run state, and the readers below owe their caller
    /// "nothing is staged" for it rather than a raw `os error 2` — which is what a `--verbatim`
    /// typed before any publish would otherwise be answered with.
    fn existing_directory(&self) -> Result<Option<PathBuf>, String> {
        validate_directory_tree(&self.target_root, "staging target root")?;
        let directory = self.target_root.join(STAGING_DIRECTORY);
        if !staging_path_exists(&directory, "staging directory")? {
            return Ok(None);
        }
        validate_directory_tree(&directory, "staging directory")?;
        Ok(Some(directory))
    }

    fn final_path(directory: &Path) -> PathBuf {
        directory.join(STAGING_FILE)
    }

    fn backup_path(directory: &Path) -> PathBuf {
        directory.join(STAGING_BACKUP_FILE)
    }

    #[cfg(all(test, windows))]
    fn with_operations(
        target_root: impl Into<PathBuf>,
        operations: Arc<dyn StagingFileOperations>,
    ) -> Result<Self, String> {
        let target_root = target_root.into();
        validate_directory_tree(&target_root, "staging target root")?;
        Ok(Self {
            target_root,
            operations,
        })
    }
}

impl StagingStore for FsStagingStore {
    fn publish(&self, packet: &[u8]) -> Result<StagingWriteOutcome, String> {
        parse_staging(packet).map_err(|error| error.to_string())?;
        let directory = self.staging_directory(true)?;
        let final_path = Self::final_path(&directory);
        let backup_path = Self::backup_path(&directory);

        refuse_retained_backup_before_publish(&final_path, &backup_path)?;

        for attempt in 0..TEMP_ATTEMPTS {
            let temp_path = directory.join(format!(".staged.publication.{attempt}.tmp"));
            let mut file = match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp_path)
            {
                Ok(file) => file,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(format!("create staging temporary: {error}")),
            };
            if let Err(error) = write_and_sync(&mut file, packet) {
                return cleanup_owned_temp(&temp_path, error);
            }
            drop(file);

            if let Err(error) = validate_existing_final(&final_path) {
                return cleanup_owned_temp(&temp_path, error);
            }
            match self.operations.rename(&temp_path, &final_path) {
                Ok(()) => return Ok(StagingWriteOutcome::Published),
                Err(error) => {
                    // POSIX `rename(2)` replaces an existing destination atomically, so the whole
                    // backup-and-restore dance below exists ONLY for Windows, where renaming onto
                    // an existing file fails. Its five helpers carry the same `cfg(windows)` for a
                    // reason worth stating: an `allow(dead_code)` would keep them compiling on
                    // Linux, where a cross-platform caller could then reach machinery that answers
                    // a Windows-only question. Gated, that call fails to resolve — loudly, at
                    // compile time, on the platform that must not have it.
                    #[cfg(windows)]
                    if final_path.exists() {
                        let old_packet =
                            match read_valid_staging(&final_path, "staged final target") {
                                Ok(Some(packet)) => packet,
                                Ok(None) => {
                                    return cleanup_owned_temp(
                                        &temp_path,
                                        format!("publish staging: {error}"),
                                    );
                                }
                                Err(validation) => {
                                    return cleanup_owned_temp(&temp_path, validation);
                                }
                            };
                        if let Err(validation) = validate_backup(&backup_path, &old_packet) {
                            return cleanup_owned_temp(&temp_path, validation);
                        }
                        if let Err(validation) = ensure_absent_backup(&backup_path) {
                            return cleanup_owned_temp(&temp_path, validation);
                        }
                        if let Err(move_old) = self.operations.rename(&final_path, &backup_path) {
                            return cleanup_owned_temp(
                                &temp_path,
                                format!("replace staged backup old final: {move_old}"),
                            );
                        }
                        if let Err(validation) = validate_backup(&backup_path, &old_packet) {
                            return Err(format!(
                                "staged backup is unsafe after move: {validation}"
                            ));
                        }
                        if let Err(publish) = self.operations.rename(&temp_path, &final_path) {
                            return match self.operations.rename(&backup_path, &final_path) {
                                Ok(()) => cleanup_owned_temp(
                                    &temp_path,
                                    format!(
                                        "publish staging after backup: {publish}; restored prior staging"
                                    ),
                                ),
                                Err(restore) => Err(format!(
                                    "publish staging after backup: {publish}; restore prior staging failed: {restore}; validated backup retained"
                                )),
                            };
                        }
                        if remove_validated_backup(&*self.operations, &backup_path, &old_packet)
                            .is_err()
                        {
                            return Ok(StagingWriteOutcome::CleanupPending);
                        }
                        return Ok(StagingWriteOutcome::Published);
                    }
                    return cleanup_owned_temp(&temp_path, format!("publish staging: {error}"));
                }
            }
        }
        Err("staging temporary names exhausted".to_owned())
    }

    fn read(&self) -> Result<Option<Vec<u8>>, String> {
        let Some(directory) = self.existing_directory()? else {
            return Ok(None);
        };
        let final_path = Self::final_path(&directory);
        match read_valid_staging(&final_path, "staged final target")? {
            Some(packet) => Ok(Some(packet)),
            None => read_valid_staging(&Self::backup_path(&directory), "staged backup target"),
        }
    }

    /// Remove both seats a `read` consults, each validated as a safe regular file first — the
    /// store never unlinks a path it has only NAMED (`rul-probe-writes-only-what-it-owns`'s
    /// posture, at a far smaller boundary).
    fn discard(&self) -> Result<(), String> {
        let Some(directory) = self.existing_directory()? else {
            return Ok(());
        };
        for (path, label) in [
            (Self::final_path(&directory), "staged final target"),
            (Self::backup_path(&directory), "staged backup target"),
        ] {
            if read_valid_staging(&path, label)?.is_some() {
                fs::remove_file(&path)
                    .map_err(|error| format!("discard {}: {error}", path.display()))?;
            }
        }
        Ok(())
    }
}

fn ensure_directory(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(_) => validate_directory_tree(path, "staging directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|create| format!("create staging directory: {create}"))?;
            validate_directory_tree(path, "staging directory")
        }
        Err(error) => Err(format!("read staging directory: {error}")),
    }
}

fn validate_directory_tree(path: &Path, label: &str) -> Result<(), String> {
    if !path.is_absolute() {
        return Err(format!("unsafe {label}"));
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        match component {
            // A Windows drive/UNC prefix is not a filesystem object and cannot be a link; stat'ing
            // it alone answers about the drive-relative cwd on `C:` and outright FAILS on the
            // `\\?\C:` verbatim spelling `fs::canonicalize` hands back. The walk starts at the root.
            Component::Prefix(prefix) => {
                current.push(prefix.as_os_str());
                continue;
            }
            Component::RootDir => current.push(component.as_os_str()),
            Component::Normal(segment) => current.push(segment),
            Component::CurDir | Component::ParentDir => return Err(format!("unsafe {label}")),
        }
        let metadata =
            fs::symlink_metadata(&current).map_err(|error| format!("read {label}: {error}"))?;
        if unsafe_metadata(&metadata) || !metadata.is_dir() {
            return Err(format!("unsafe {label}"));
        }
    }
    Ok(())
}

fn validate_existing_final(path: &Path) -> Result<(), String> {
    match read_valid_staging(path, "staged final target")? {
        Some(_) | None => Ok(()),
    }
}

#[cfg(windows)]
fn validate_backup(path: &Path, expected: &[u8]) -> Result<(), String> {
    match read_valid_staging(path, "staged backup target")? {
        Some(packet) if packet == expected => Ok(()),
        Some(_) => Err("staged backup does not match prior staging".to_owned()),
        None => Ok(()),
    }
}

#[cfg(windows)]
fn remove_validated_backup(
    operations: &dyn StagingFileOperations,
    path: &Path,
    expected: &[u8],
) -> Result<(), String> {
    match read_valid_staging(path, "staged backup target")? {
        Some(packet) if packet == expected => operations
            .remove_file(path)
            .map_err(|error| format!("remove staged backup: {error}")),
        Some(_) => Err("staged backup does not match prior staging".to_owned()),
        None => Err("staged backup disappeared before cleanup".to_owned()),
    }
}

fn refuse_retained_backup_before_publish(
    final_path: &Path,
    backup_path: &Path,
) -> Result<(), String> {
    if read_valid_staging(final_path, "staged final target")?.is_some() {
        if staging_path_exists(backup_path, "staged backup target")? {
            return Err(
                "staging write refused: retained backup requires deliberate resolution".to_owned(),
            );
        }
    } else {
        let _ = read_valid_staging(backup_path, "staged backup target")?;
    }
    Ok(())
}

fn staging_path_exists(path: &Path, label: &str) -> Result<bool, String> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(format!("read {label}: {error}")),
    }
}

#[cfg(windows)]
fn ensure_absent_backup(path: &Path) -> Result<(), String> {
    match read_valid_staging(path, "staged backup target")? {
        Some(_) => Err("staged backup appeared before publication".to_owned()),
        None => Ok(()),
    }
}

fn read_valid_staging(path: &Path, label: &str) -> Result<Option<Vec<u8>>, String> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("read {label}: {error}")),
    };
    if unsafe_metadata(&metadata) || !metadata.is_file() {
        return Err(format!("unsafe {label}"));
    }
    if metadata.len() > MAX_STAGING_BYTES as u64 {
        return Err("staged publication exceeds size limit".to_owned());
    }
    let mut file = File::open(path).map_err(|error| format!("open staging: {error}"))?;
    let opened = file
        .metadata()
        .map_err(|error| format!("read opened staging: {error}"))?;
    if unsafe_metadata(&opened) || !opened.is_file() || opened.len() > MAX_STAGING_BYTES as u64 {
        return Err(format!("unsafe {label}"));
    }
    let capacity =
        usize::try_from(opened.len()).map_err(|_| "staged publication exceeds size limit")?;
    let mut bytes = Vec::with_capacity(capacity);
    Read::by_ref(&mut file)
        .take((MAX_STAGING_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("read staging: {error}"))?;
    if bytes.len() > MAX_STAGING_BYTES {
        return Err("staged publication exceeds size limit".to_owned());
    }
    parse_staging(&bytes).map_err(|error: StagingError| error.to_string())?;
    Ok(Some(bytes))
}

fn unsafe_metadata(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    false
}

fn write_and_sync(file: &mut File, packet: &[u8]) -> Result<(), String> {
    file.write_all(packet)
        .map_err(|error| format!("write staging: {error}"))?;
    file.flush()
        .map_err(|error| format!("flush staging: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("sync staging: {error}"))
}

fn cleanup_owned_temp<T>(temp_path: &Path, error: String) -> Result<T, String> {
    fs::remove_file(temp_path)
        .map_err(|cleanup| format!("{error}; cleanup staging temporary: {cleanup}"))?;
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::staging::tests::inspection;
    use crate::{accept_staged, encode_staging, stage_publication};

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir()
                .join("dorc-loom-staging-store-tests")
                .join(name);
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root");
            Self(path)
        }

        fn staging_directory(&self) -> PathBuf {
            self.0.join(STAGING_DIRECTORY)
        }

        fn final_path(&self) -> PathBuf {
            self.staging_directory().join(STAGING_FILE)
        }

        fn backup_path(&self) -> PathBuf {
            self.staging_directory().join(STAGING_BACKUP_FILE)
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn packet(value: &str) -> Vec<u8> {
        encode_staging(&inspection(value)).expect("a valid packet")
    }

    #[test]
    fn staging_writes_then_reads_one_isolated_packet() {
        let root = TestRoot::new("staging-writes-then-reads-one-isolated-packet");
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        let inspection = inspection("first");
        stage_publication(&store, &inspection).expect("the refusing run stages");
        accept_staged(&store, &inspection, "a-case").expect("verbatim reads the exact packet");
        assert_eq!(
            store
                .read()
                .expect("the store reads")
                .expect("a published packet is present"),
            packet("first")
        );
    }

    /// The `--verbatim` contract end to end at this seat: a staging binds the exact bytes it was
    /// computed from, so a re-edit invalidates it; and an APPLIED one is spent, or a second
    /// `--verbatim` would silently re-confirm a loss the author accepted once, against an
    /// interpretation nobody looked at this time.
    #[test]
    fn a_staged_interpretation_binds_its_bytes_and_is_spent_once_applied() {
        let root = TestRoot::new("a-staged-interpretation-binds-its-bytes");
        let store = FsStagingStore::new(&root.0).expect("trusted root");

        let refusal = accept_staged(&store, &inspection("first"), "a-case")
            .expect_err("nothing has been staged yet");
        assert!(refusal.contains("dorc-loom publish a-case"), "{refusal}");

        stage_publication(&store, &inspection("first")).expect("the refusing run stages");
        let stale = accept_staged(&store, &inspection("edited-since"), "a-case")
            .expect_err("a re-edit invalidates the staging");
        assert!(stale.contains("dorc-loom publish a-case"), "{stale}");
        assert!(
            store.read().expect("the store reads").is_some(),
            "a refusal spends nothing"
        );

        accept_staged(&store, &inspection("first"), "a-case").expect("the exact bytes apply");
        store.discard().expect("an applied staging is spent");
        assert!(store.read().expect("the store reads").is_none());
        // Idempotent: the goal is that nothing is staged, and it already is not.
        store.discard().expect("discarding nothing succeeds");
    }

    #[test]
    fn a_valid_packet_replaces_a_valid_packet() {
        let root = TestRoot::new("a-valid-packet-replaces-a-valid-packet");
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        store.publish(&packet("first")).expect("first packet");
        store
            .publish(&packet("second"))
            .expect("replacement packet");
        assert_eq!(
            fs::read(root.final_path()).expect("final bytes"),
            packet("second")
        );
    }

    #[test]
    fn regular_readonly_final_remains_readable() {
        let root = TestRoot::new("regular-readonly-final-remains-readable");
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        let packet = packet("readonly");
        store.publish(&packet).expect("publish");
        let final_path = root.final_path();
        let mut permissions = fs::metadata(&final_path)
            .expect("final metadata")
            .permissions();
        let original_permissions = permissions.clone();
        permissions.set_readonly(true);
        fs::set_permissions(&final_path, permissions).expect("readonly final");
        let read = store.read();
        fs::set_permissions(&final_path, original_permissions).expect("restore final permissions");
        assert_eq!(
            read.expect("the readonly store reads")
                .expect("a published packet is present"),
            packet
        );
    }

    #[test]
    fn malformed_and_oversized_finals_are_preserved() {
        for (name, bytes) in [
            ("malformed-final", b"not a staged packet".to_vec()),
            ("oversized-final", vec![b'x'; MAX_STAGING_BYTES + 1]),
        ] {
            let root = TestRoot::new(name);
            fs::create_dir(root.staging_directory()).expect("staging directory");
            fs::write(root.final_path(), &bytes).expect("hostile final");
            let store = FsStagingStore::new(&root.0).expect("trusted root");
            assert!(store.read().is_err());
            assert_eq!(fs::read(root.final_path()).expect("final bytes"), bytes);
            assert!(store.publish(&packet("next")).is_err());
            assert_eq!(fs::read(root.final_path()).expect("final bytes"), bytes);
        }
    }

    #[test]
    fn malformed_backup_without_a_final_refuses_without_touching_hostile_bytes() {
        let root = TestRoot::new("malformed-backup-without-a-final-refuses");
        fs::create_dir(root.staging_directory()).expect("staging directory");
        let hostile = b"not a staged packet";
        fs::write(root.backup_path(), hostile).expect("hostile backup");

        let store = FsStagingStore::new(&root.0).expect("trusted root");
        assert!(store.publish(&packet("next")).is_err());
        assert!(!root.final_path().exists());
        assert_eq!(
            fs::read(root.backup_path()).expect("hostile backup"),
            hostile
        );
    }

    #[test]
    fn identical_valid_backup_with_a_final_refuses_without_mutation() {
        let root = TestRoot::new("identical-valid-backup-with-a-final-refuses");
        let retained = packet("retained");
        fs::create_dir(root.staging_directory()).expect("staging directory");
        fs::write(root.final_path(), &retained).expect("final packet");
        fs::write(root.backup_path(), &retained).expect("backup packet");

        let store = FsStagingStore::new(&root.0).expect("trusted root");
        let error = store
            .publish(&packet("next"))
            .expect_err("retained backup refuses");
        assert!(error.contains("retained backup requires deliberate resolution"));
        assert_eq!(fs::read(root.final_path()).expect("final bytes"), retained);
        assert_eq!(
            fs::read(root.backup_path()).expect("backup bytes"),
            retained
        );
        for attempt in 0..TEMP_ATTEMPTS {
            assert!(
                !root
                    .staging_directory()
                    .join(format!(".staged.publication.{attempt}.tmp"))
                    .exists()
            );
        }
    }

    #[test]
    fn distinct_valid_backup_with_a_final_refuses_without_mutation() {
        let root = TestRoot::new("distinct-valid-backup-with-a-final-refuses");
        let final_packet = packet("final");
        let backup_packet = packet("hostile backup");
        fs::create_dir(root.staging_directory()).expect("staging directory");
        fs::write(root.final_path(), &final_packet).expect("final packet");
        fs::write(root.backup_path(), &backup_packet).expect("backup packet");

        let store = FsStagingStore::new(&root.0).expect("trusted root");
        let error = store
            .publish(&packet("next"))
            .expect_err("retained backup refuses");
        assert!(error.contains("retained backup requires deliberate resolution"));
        assert_eq!(
            fs::read(root.final_path()).expect("final bytes"),
            final_packet
        );
        assert_eq!(
            fs::read(root.backup_path()).expect("backup bytes"),
            backup_packet
        );
        for attempt in 0..TEMP_ATTEMPTS {
            assert!(
                !root
                    .staging_directory()
                    .join(format!(".staged.publication.{attempt}.tmp"))
                    .exists()
            );
        }
    }

    #[test]
    fn directory_final_and_non_directory_parent_refuse() {
        let root = TestRoot::new("directory-final-refuses");
        fs::create_dir(root.staging_directory()).expect("staging directory");
        fs::create_dir(root.final_path()).expect("final directory");
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        assert!(store.read().is_err());
        assert!(store.publish(&packet("next")).is_err());
        assert!(root.final_path().is_dir());

        let root = TestRoot::new("non-directory-parent-refuses");
        fs::write(root.staging_directory(), b"not a directory").expect("hostile parent");
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        assert!(store.publish(&packet("next")).is_err());
        assert_eq!(
            fs::read(root.staging_directory()).expect("parent bytes"),
            b"not a directory"
        );
    }

    #[test]
    fn hostile_temp_collisions_exhaust_without_changes() {
        let root = TestRoot::new("hostile-temp-collisions-exhaust-without-changes");
        fs::create_dir(root.staging_directory()).expect("staging directory");
        for attempt in 0..TEMP_ATTEMPTS {
            fs::write(
                root.staging_directory()
                    .join(format!(".staged.publication.{attempt}.tmp")),
                format!("hostile-{attempt}"),
            )
            .expect("hostile temp");
        }
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        assert!(store.publish(&packet("next")).is_err());
        for attempt in 0..TEMP_ATTEMPTS {
            assert_eq!(
                fs::read(
                    root.staging_directory()
                        .join(format!(".staged.publication.{attempt}.tmp"))
                )
                .expect("hostile temp bytes"),
                format!("hostile-{attempt}").as_bytes()
            );
        }
    }

    #[test]
    fn cleanup_removes_only_the_operation_owned_temp() {
        let root = TestRoot::new("cleanup-removes-only-the-operation-owned-temp");
        let owned = root.0.join("owned.tmp");
        let hostile = root.0.join("hostile.tmp");
        fs::write(&owned, b"owned").expect("owned temp");
        fs::write(&hostile, b"hostile").expect("hostile temp");
        assert!(cleanup_owned_temp::<()>(&owned, "failure".to_owned()).is_err());
        assert!(!owned.exists());
        assert_eq!(fs::read(hostile).expect("hostile temp bytes"), b"hostile");
    }

    #[cfg(windows)]
    struct WindowsFailureOperations {
        rename_calls: std::sync::Mutex<usize>,
        fail_renames: Vec<usize>,
        fail_backup_cleanup: bool,
        fail_final_removal: bool,
    }

    #[cfg(windows)]
    impl StagingFileOperations for WindowsFailureOperations {
        fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()> {
            let call = {
                let mut calls = self
                    .rename_calls
                    .lock()
                    .expect("test rename counter poisoned");
                let call = calls.checked_add(1).expect("test rename counter overflow");
                *calls = call;
                call
            };
            if self.fail_renames.contains(&call) {
                return Err(std::io::Error::other("injected rename failure"));
            }
            fs::rename(from, to)
        }

        fn remove_file(&self, path: &Path) -> std::io::Result<()> {
            if self.fail_backup_cleanup
                && path
                    .file_name()
                    .is_some_and(|name| name == STAGING_BACKUP_FILE)
            {
                return Err(std::io::Error::other("injected backup cleanup failure"));
            }
            if self.fail_final_removal && path.file_name().is_some_and(|name| name == STAGING_FILE)
            {
                return Err(std::io::Error::other("injected final removal failure"));
            }
            fs::remove_file(path)
        }
    }

    #[cfg(windows)]
    fn windows_store(
        root: &TestRoot,
        fail_renames: Vec<usize>,
        fail_backup_cleanup: bool,
        fail_final_removal: bool,
    ) -> FsStagingStore {
        FsStagingStore::with_operations(
            &root.0,
            Arc::new(WindowsFailureOperations {
                rename_calls: std::sync::Mutex::new(0),
                fail_renames,
                fail_backup_cleanup,
                fail_final_removal,
            }),
        )
        .expect("trusted root")
    }

    #[cfg(windows)]
    #[test]
    fn windows_replacement_publishes_and_removes_the_validated_backup() {
        let root = TestRoot::new("windows-replacement-publishes-and-removes-validated-backup");
        FsStagingStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first packet");

        windows_store(&root, vec![1], false, false)
            .publish(&packet("second"))
            .expect("replacement packet");

        assert_eq!(
            fs::read(root.final_path()).expect("final bytes"),
            packet("second")
        );
        assert!(!root.backup_path().exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_second_rename_failure_restores_the_prior_final() {
        let root = TestRoot::new("windows-second-rename-failure-restores-prior-final");
        FsStagingStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first packet");

        assert!(
            windows_store(&root, vec![1, 3], false, false)
                .publish(&packet("second"))
                .is_err()
        );
        assert_eq!(
            fs::read(root.final_path()).expect("restored final"),
            packet("first")
        );
        assert!(!root.backup_path().exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_failed_publication_and_restore_retains_a_recoverable_backup() {
        let root =
            TestRoot::new("windows-failed-publication-and-restore-retains-recoverable-backup");
        FsStagingStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first packet");
        let store = windows_store(&root, vec![1, 3, 4], false, false);

        assert!(store.publish(&packet("second")).is_err());
        assert!(!root.final_path().exists());
        assert_eq!(
            fs::read(root.backup_path()).expect("retained backup"),
            packet("first")
        );
        assert_eq!(
            store
                .read()
                .expect("backup recovery read")
                .expect("the backup is present"),
            packet("first")
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_cleanup_failure_retains_backup_and_refuses_later_writes() {
        let root = TestRoot::new(
            "windows-cleanup-failure-keeps-the-published-packet-and-refuses-later-writes",
        );
        FsStagingStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first packet");

        let store = windows_store(&root, vec![1], true, true);
        assert_eq!(
            store.publish(&packet("second")),
            Ok(StagingWriteOutcome::CleanupPending)
        );
        assert_eq!(
            fs::read(root.final_path()).expect("published final"),
            packet("second")
        );
        assert_eq!(
            fs::read(root.backup_path()).expect("retained stale backup"),
            packet("first")
        );
        assert_eq!(
            store
                .read()
                .expect("reader prefers final")
                .expect("the final is present"),
            packet("second")
        );

        assert!(store.publish(&packet("third")).is_err());
        assert_eq!(
            fs::read(root.final_path()).expect("unmodified final"),
            packet("second")
        );
        assert_eq!(
            fs::read(root.backup_path()).expect("unmodified stale backup"),
            packet("first")
        );
        for attempt in 0..TEMP_ATTEMPTS {
            assert!(
                !root
                    .staging_directory()
                    .join(format!(".staged.publication.{attempt}.tmp"))
                    .exists()
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn windows_hostile_backup_refuses_without_touching_the_final() {
        let root = TestRoot::new("windows-hostile-backup-refuses-without-touching-the-final");
        FsStagingStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first packet");
        fs::write(root.backup_path(), b"hostile backup").expect("hostile backup");

        assert!(
            windows_store(&root, vec![1], false, false)
                .publish(&packet("second"))
                .is_err()
        );
        assert_eq!(
            fs::read(root.final_path()).expect("prior final"),
            packet("first")
        );
        assert_eq!(
            fs::read(root.backup_path()).expect("hostile backup"),
            b"hostile backup"
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_reader_recovers_only_an_absent_final_from_a_valid_backup() {
        let root = TestRoot::new("windows-reader-recovers-only-an-absent-final-from-valid-backup");
        fs::create_dir(root.staging_directory()).expect("staging directory");
        fs::write(root.backup_path(), packet("prior")).expect("valid backup");
        let store = FsStagingStore::new(&root.0).expect("trusted root");

        assert_eq!(
            store
                .read()
                .expect("backup recovery")
                .expect("the backup is present"),
            packet("prior")
        );
        fs::write(root.final_path(), b"malformed final").expect("malformed final");
        assert!(store.read().is_err());
    }

    #[cfg(unix)]
    #[test]
    fn linked_parent_and_final_refuse_without_following_links() {
        use std::os::unix::fs::symlink;

        let root = TestRoot::new("linked-parent-and-final-refuse-without-following-links");
        let target = root.0.join("target");
        fs::create_dir(&target).expect("target directory");
        let linked_parent = root.0.join("linked-parent");
        symlink(&target, &linked_parent).expect("parent link");
        assert!(FsStagingStore::new(linked_parent).is_err());

        fs::create_dir(root.staging_directory()).expect("staging directory");
        let outside = root.0.join("outside-packet");
        fs::write(&outside, b"outside").expect("outside packet");
        symlink(&outside, root.final_path()).expect("final link");
        let store = FsStagingStore::new(&root.0).expect("trusted root");
        assert!(store.read().is_err());
        assert!(store.publish(&packet("next")).is_err());
        assert_eq!(fs::read(outside).expect("outside bytes"), b"outside");
    }

    #[cfg(windows)]
    #[test]
    fn linked_parent_refuses_when_link_creation_is_permitted() {
        use std::os::windows::fs::symlink_dir;

        let root = TestRoot::new("linked-parent-refuses-when-link-creation-is-permitted");
        let target = root.0.join("target");
        fs::create_dir(&target).expect("target directory");
        let linked_parent = root.0.join("linked-parent");
        if symlink_dir(&target, &linked_parent).is_ok() {
            assert!(FsStagingStore::new(linked_parent).is_err());
        }
    }
}
