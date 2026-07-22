//! Narrow receipt persistence edge. Packet construction and validation stay pure.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use crate::receipt::parse as parse_receipt;
use crate::{MAX_RECEIPT_BYTES, ReceiptError};

const RECEIPT_DIRECTORY: &str = "dorc-loom";
const RECEIPT_FILE: &str = "compile.receipt";
const RECEIPT_BACKUP_FILE: &str = ".compile.receipt.backup";
const TEMP_ATTEMPTS: u8 = 16;

trait ReceiptFileOperations: Send + Sync {
    fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()>;
    fn remove_file(&self, path: &Path) -> std::io::Result<()>;
}

struct NativeReceiptFileOperations;

impl ReceiptFileOperations for NativeReceiptFileOperations {
    fn rename(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        fs::rename(from, to)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        fs::remove_file(path)
    }
}

/// The filesystem boundary used by compile and promote.
pub trait ReceiptStore {
    /// Publish an already encoded and validated receipt.
    ///
    /// # Errors
    ///
    /// Returns an I/O or receipt-validation refusal without publishing partial bytes.
    fn publish(&self, packet: &[u8]) -> Result<(), String>;
    /// Read one bounded, grammar-validated current receipt.
    ///
    /// # Errors
    ///
    /// Returns an I/O, unsafe-path, size, or receipt-validation refusal.
    fn read(&self) -> Result<Vec<u8>, String>;
}

/// Worktree-local receipt storage under one validated ignored target root.
#[derive(Clone)]
pub struct FsReceiptStore {
    target_root: PathBuf,
    operations: Arc<dyn ReceiptFileOperations>,
}

impl PartialEq for FsReceiptStore {
    fn eq(&self, other: &Self) -> bool {
        self.target_root == other.target_root
    }
}

impl Eq for FsReceiptStore {}

impl std::fmt::Debug for FsReceiptStore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FsReceiptStore")
            .field("target_root", &self.target_root)
            .finish_non_exhaustive()
    }
}

impl FsReceiptStore {
    /// Bind the fixed receipt location below an existing trusted target root.
    ///
    /// # Errors
    ///
    /// Refuses a missing, linked, reparse-point, or non-directory target root.
    pub fn new(target_root: impl Into<PathBuf>) -> Result<Self, String> {
        let target_root = target_root.into();
        validate_directory_tree(&target_root, "receipt target root")?;
        Ok(Self {
            target_root,
            operations: Arc::new(NativeReceiptFileOperations),
        })
    }

    fn receipt_directory(&self, create: bool) -> Result<PathBuf, String> {
        validate_directory_tree(&self.target_root, "receipt target root")?;
        let directory = self.target_root.join(RECEIPT_DIRECTORY);
        if create {
            ensure_directory(&directory)?;
        } else {
            validate_directory_tree(&directory, "receipt directory")?;
        }
        validate_directory_tree(&self.target_root, "receipt target root")?;
        validate_directory_tree(&directory, "receipt directory")?;
        Ok(directory)
    }

    fn final_path(directory: &Path) -> PathBuf {
        directory.join(RECEIPT_FILE)
    }

    fn backup_path(directory: &Path) -> PathBuf {
        directory.join(RECEIPT_BACKUP_FILE)
    }

    #[cfg(test)]
    fn with_operations(
        target_root: impl Into<PathBuf>,
        operations: Arc<dyn ReceiptFileOperations>,
    ) -> Result<Self, String> {
        let target_root = target_root.into();
        validate_directory_tree(&target_root, "receipt target root")?;
        Ok(Self {
            target_root,
            operations,
        })
    }
}

impl ReceiptStore for FsReceiptStore {
    fn publish(&self, packet: &[u8]) -> Result<(), String> {
        parse_receipt(packet).map_err(|error| error.to_string())?;
        let directory = self.receipt_directory(true)?;
        let final_path = Self::final_path(&directory);
        let backup_path = Self::backup_path(&directory);

        for attempt in 0..TEMP_ATTEMPTS {
            let temp_path = directory.join(format!(".compile.receipt.{attempt}.tmp"));
            let mut file = match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp_path)
            {
                Ok(file) => file,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(format!("create receipt temporary: {error}")),
            };
            if let Err(error) = write_and_sync(&mut file, packet) {
                return cleanup_owned_temp(&temp_path, error);
            }
            drop(file);

            if let Err(error) = validate_existing_final(&final_path) {
                return cleanup_owned_temp(&temp_path, error);
            }
            match self.operations.rename(&temp_path, &final_path) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    #[cfg(windows)]
                    if final_path.exists() {
                        let old_packet =
                            match read_valid_receipt(&final_path, "receipt final target") {
                                Ok(Some(packet)) => packet,
                                Ok(None) => {
                                    return cleanup_owned_temp(
                                        &temp_path,
                                        format!("publish receipt: {error}"),
                                    );
                                }
                                Err(validation) => {
                                    return cleanup_owned_temp(&temp_path, validation);
                                }
                            };
                        if let Err(validation) = validate_backup(&backup_path, &old_packet) {
                            return cleanup_owned_temp(&temp_path, validation);
                        }
                        if let Err(clear) =
                            clear_expected_backup(&*self.operations, &backup_path, &old_packet)
                        {
                            return cleanup_owned_temp(&temp_path, clear);
                        }
                        if let Err(move_old) = self.operations.rename(&final_path, &backup_path) {
                            return cleanup_owned_temp(
                                &temp_path,
                                format!("replace receipt backup old final: {move_old}"),
                            );
                        }
                        if let Err(validation) = validate_backup(&backup_path, &old_packet) {
                            return Err(format!(
                                "receipt backup is unsafe after move: {validation}"
                            ));
                        }
                        if let Err(publish) = self.operations.rename(&temp_path, &final_path) {
                            return match self.operations.rename(&backup_path, &final_path) {
                                Ok(()) => cleanup_owned_temp(
                                    &temp_path,
                                    format!(
                                        "publish receipt after backup: {publish}; restored prior receipt"
                                    ),
                                ),
                                Err(restore) => Err(format!(
                                    "publish receipt after backup: {publish}; restore prior receipt failed: {restore}; validated backup retained"
                                )),
                            };
                        }
                        if let Err(cleanup) =
                            remove_validated_backup(&*self.operations, &backup_path, &old_packet)
                        {
                            return Err(format!(
                                "published receipt; backup cleanup failed: {cleanup}"
                            ));
                        }
                        return Ok(());
                    }
                    return cleanup_owned_temp(&temp_path, format!("publish receipt: {error}"));
                }
            }
        }
        Err("receipt temporary names exhausted".to_owned())
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        let directory = self.receipt_directory(false)?;
        let final_path = Self::final_path(&directory);
        match read_valid_receipt(&final_path, "receipt final target")? {
            Some(packet) => Ok(packet),
            None => read_valid_receipt(&Self::backup_path(&directory), "receipt backup target")?
                .ok_or_else(|| "receipt is absent".to_owned()),
        }
    }
}

fn ensure_directory(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(_) => validate_directory_tree(path, "receipt directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|create| format!("create receipt directory: {create}"))?;
            validate_directory_tree(path, "receipt directory")
        }
        Err(error) => Err(format!("read receipt directory: {error}")),
    }
}

fn validate_directory_tree(path: &Path, label: &str) -> Result<(), String> {
    if !path.is_absolute() {
        return Err(format!("unsafe {label}"));
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
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
    match read_valid_receipt(path, "receipt final target")? {
        Some(_) | None => Ok(()),
    }
}

fn validate_backup(path: &Path, expected: &[u8]) -> Result<(), String> {
    match read_valid_receipt(path, "receipt backup target")? {
        Some(packet) if packet == expected => Ok(()),
        Some(_) => Err("receipt backup does not match prior receipt".to_owned()),
        None => Ok(()),
    }
}

fn remove_validated_backup(
    operations: &dyn ReceiptFileOperations,
    path: &Path,
    expected: &[u8],
) -> Result<(), String> {
    match read_valid_receipt(path, "receipt backup target")? {
        Some(packet) if packet == expected => operations
            .remove_file(path)
            .map_err(|error| format!("remove receipt backup: {error}")),
        Some(_) => Err("receipt backup does not match prior receipt".to_owned()),
        None => Err("receipt backup disappeared before cleanup".to_owned()),
    }
}

fn clear_expected_backup(
    operations: &dyn ReceiptFileOperations,
    path: &Path,
    expected: &[u8],
) -> Result<(), String> {
    match read_valid_receipt(path, "receipt backup target")? {
        Some(packet) if packet == expected => operations
            .remove_file(path)
            .map_err(|error| format!("clear validated receipt backup: {error}")),
        Some(_) => Err("receipt backup does not match prior receipt".to_owned()),
        None => Ok(()),
    }
}

fn read_valid_receipt(path: &Path, label: &str) -> Result<Option<Vec<u8>>, String> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("read {label}: {error}")),
    };
    if unsafe_metadata(&metadata) || !metadata.is_file() {
        return Err(format!("unsafe {label}"));
    }
    if metadata.len() > MAX_RECEIPT_BYTES as u64 {
        return Err("receipt exceeds size limit".to_owned());
    }
    let mut file = File::open(path).map_err(|error| format!("open receipt: {error}"))?;
    let opened = file
        .metadata()
        .map_err(|error| format!("read opened receipt: {error}"))?;
    if unsafe_metadata(&opened) || !opened.is_file() || opened.len() > MAX_RECEIPT_BYTES as u64 {
        return Err(format!("unsafe {label}"));
    }
    let capacity = usize::try_from(opened.len()).map_err(|_| "receipt exceeds size limit")?;
    let mut bytes = Vec::with_capacity(capacity);
    Read::by_ref(&mut file)
        .take((MAX_RECEIPT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("read receipt: {error}"))?;
    if bytes.len() > MAX_RECEIPT_BYTES {
        return Err("receipt exceeds size limit".to_owned());
    }
    parse_receipt(&bytes).map_err(|error: ReceiptError| error.to_string())?;
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
        .map_err(|error| format!("write receipt: {error}"))?;
    file.flush()
        .map_err(|error| format!("flush receipt: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("sync receipt: {error}"))
}

fn cleanup_owned_temp(temp_path: &Path, error: String) -> Result<(), String> {
    fs::remove_file(temp_path)
        .map_err(|cleanup| format!("{error}; cleanup receipt temporary: {cleanup}"))?;
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipt::tests::inspection;
    use crate::{compile_receipt, encode_receipt, promote_receipt};

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir()
                .join("dorc-loom-receipt-store-tests")
                .join(name);
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root");
            Self(path)
        }

        fn receipt_directory(&self) -> PathBuf {
            self.0.join(RECEIPT_DIRECTORY)
        }

        fn final_path(&self) -> PathBuf {
            self.receipt_directory().join(RECEIPT_FILE)
        }

        fn backup_path(&self) -> PathBuf {
            self.receipt_directory().join(RECEIPT_BACKUP_FILE)
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn packet(value: &str) -> Vec<u8> {
        encode_receipt(&inspection(value)).expect("valid receipt")
    }

    #[test]
    fn workflow_writes_then_reads_one_isolated_receipt() {
        let root = TestRoot::new("workflow-writes-then-reads-one-isolated-receipt");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
        let inspection = inspection("first");
        compile_receipt(&store, &inspection).expect("compile persists");
        promote_receipt(&store, &inspection).expect("promote reads exact receipt");
        assert_eq!(store.read().expect("receipt reads"), packet("first"));
    }

    #[test]
    fn valid_receipt_replaces_a_valid_receipt() {
        let root = TestRoot::new("valid-receipt-replaces-a-valid-receipt");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
        store.publish(&packet("first")).expect("first receipt");
        store
            .publish(&packet("second"))
            .expect("replacement receipt");
        assert_eq!(
            fs::read(root.final_path()).expect("final bytes"),
            packet("second")
        );
    }

    #[test]
    fn regular_readonly_final_remains_readable() {
        let root = TestRoot::new("regular-readonly-final-remains-readable");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
        let packet = packet("readonly");
        store.publish(&packet).expect("receipt");
        let final_path = root.final_path();
        let mut permissions = fs::metadata(&final_path)
            .expect("final metadata")
            .permissions();
        let original_permissions = permissions.clone();
        permissions.set_readonly(true);
        fs::set_permissions(&final_path, permissions).expect("readonly final");
        let read = store.read();
        fs::set_permissions(&final_path, original_permissions).expect("restore final permissions");
        assert_eq!(read.expect("readonly receipt reads"), packet);
    }

    #[test]
    fn malformed_and_oversized_finals_are_preserved() {
        for (name, bytes) in [
            ("malformed-final", b"not a receipt".to_vec()),
            ("oversized-final", vec![b'x'; MAX_RECEIPT_BYTES + 1]),
        ] {
            let root = TestRoot::new(name);
            fs::create_dir(root.receipt_directory()).expect("receipt directory");
            fs::write(root.final_path(), &bytes).expect("hostile final");
            let store = FsReceiptStore::new(&root.0).expect("trusted root");
            assert!(store.read().is_err());
            assert_eq!(fs::read(root.final_path()).expect("final bytes"), bytes);
            assert!(store.publish(&packet("next")).is_err());
            assert_eq!(fs::read(root.final_path()).expect("final bytes"), bytes);
        }
    }

    #[test]
    fn directory_final_and_non_directory_parent_refuse() {
        let root = TestRoot::new("directory-final-refuses");
        fs::create_dir(root.receipt_directory()).expect("receipt directory");
        fs::create_dir(root.final_path()).expect("final directory");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
        assert!(store.read().is_err());
        assert!(store.publish(&packet("next")).is_err());
        assert!(root.final_path().is_dir());

        let root = TestRoot::new("non-directory-parent-refuses");
        fs::write(root.receipt_directory(), b"not a directory").expect("hostile parent");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
        assert!(store.publish(&packet("next")).is_err());
        assert_eq!(
            fs::read(root.receipt_directory()).expect("parent bytes"),
            b"not a directory"
        );
    }

    #[test]
    fn hostile_temp_collisions_exhaust_without_changes() {
        let root = TestRoot::new("hostile-temp-collisions-exhaust-without-changes");
        fs::create_dir(root.receipt_directory()).expect("receipt directory");
        for attempt in 0..TEMP_ATTEMPTS {
            fs::write(
                root.receipt_directory()
                    .join(format!(".compile.receipt.{attempt}.tmp")),
                format!("hostile-{attempt}"),
            )
            .expect("hostile temp");
        }
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
        assert!(store.publish(&packet("next")).is_err());
        for attempt in 0..TEMP_ATTEMPTS {
            assert_eq!(
                fs::read(
                    root.receipt_directory()
                        .join(format!(".compile.receipt.{attempt}.tmp"))
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
        assert!(cleanup_owned_temp(&owned, "failure".to_owned()).is_err());
        assert!(!owned.exists());
        assert_eq!(fs::read(hostile).expect("hostile temp bytes"), b"hostile");
    }

    #[cfg(windows)]
    struct WindowsFailureOperations {
        rename_calls: std::sync::Mutex<usize>,
        fail_renames: Vec<usize>,
        fail_backup_cleanup: bool,
    }

    #[cfg(windows)]
    impl ReceiptFileOperations for WindowsFailureOperations {
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
                    .is_some_and(|name| name == RECEIPT_BACKUP_FILE)
            {
                return Err(std::io::Error::other("injected backup cleanup failure"));
            }
            fs::remove_file(path)
        }
    }

    #[cfg(windows)]
    fn windows_store(
        root: &TestRoot,
        fail_renames: Vec<usize>,
        fail_backup_cleanup: bool,
    ) -> FsReceiptStore {
        FsReceiptStore::with_operations(
            &root.0,
            Arc::new(WindowsFailureOperations {
                rename_calls: std::sync::Mutex::new(0),
                fail_renames,
                fail_backup_cleanup,
            }),
        )
        .expect("trusted root")
    }

    #[cfg(windows)]
    #[test]
    fn windows_replacement_publishes_and_removes_the_validated_backup() {
        let root = TestRoot::new("windows-replacement-publishes-and-removes-validated-backup");
        FsReceiptStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first receipt");

        windows_store(&root, vec![1], false)
            .publish(&packet("second"))
            .expect("replacement receipt");

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
        FsReceiptStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first receipt");

        assert!(
            windows_store(&root, vec![1, 3], false)
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
        FsReceiptStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first receipt");
        let store = windows_store(&root, vec![1, 3, 4], false);

        assert!(store.publish(&packet("second")).is_err());
        assert!(!root.final_path().exists());
        assert_eq!(
            fs::read(root.backup_path()).expect("retained backup"),
            packet("first")
        );
        assert_eq!(store.read().expect("backup recovery read"), packet("first"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_backup_cleanup_failure_keeps_the_new_final() {
        let root = TestRoot::new("windows-backup-cleanup-failure-keeps-new-final");
        FsReceiptStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first receipt");

        assert!(
            windows_store(&root, vec![1], true)
                .publish(&packet("second"))
                .is_err()
        );
        assert_eq!(
            fs::read(root.final_path()).expect("new final"),
            packet("second")
        );
        assert_eq!(
            fs::read(root.backup_path()).expect("retained backup"),
            packet("first")
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_hostile_backup_refuses_without_touching_the_final() {
        let root = TestRoot::new("windows-hostile-backup-refuses-without-touching-the-final");
        FsReceiptStore::new(&root.0)
            .expect("trusted root")
            .publish(&packet("first"))
            .expect("first receipt");
        fs::write(root.backup_path(), b"hostile backup").expect("hostile backup");

        assert!(
            windows_store(&root, vec![1], false)
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
        fs::create_dir(root.receipt_directory()).expect("receipt directory");
        fs::write(root.backup_path(), packet("prior")).expect("valid backup");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");

        assert_eq!(store.read().expect("backup recovery"), packet("prior"));
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
        assert!(FsReceiptStore::new(linked_parent).is_err());

        fs::create_dir(root.receipt_directory()).expect("receipt directory");
        let outside = root.0.join("outside-receipt");
        fs::write(&outside, b"outside").expect("outside receipt");
        symlink(&outside, root.final_path()).expect("final link");
        let store = FsReceiptStore::new(&root.0).expect("trusted root");
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
            assert!(FsReceiptStore::new(linked_parent).is_err());
        }
    }
}
