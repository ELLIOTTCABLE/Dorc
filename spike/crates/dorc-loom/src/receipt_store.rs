//! Narrow receipt persistence edge. Packet construction and validation stay pure.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use crate::receipt::parse as parse_receipt;
use crate::{MAX_RECEIPT_BYTES, ReceiptError};

const RECEIPT_DIRECTORY: &str = "dorc-loom";
const RECEIPT_FILE: &str = "compile.receipt";
const TEMP_ATTEMPTS: u8 = 16;

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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FsReceiptStore {
    target_root: PathBuf,
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
        Ok(Self { target_root })
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
}

impl ReceiptStore for FsReceiptStore {
    fn publish(&self, packet: &[u8]) -> Result<(), String> {
        parse_receipt(packet).map_err(|error| error.to_string())?;
        let directory = self.receipt_directory(true)?;
        let final_path = Self::final_path(&directory);

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
            match fs::rename(&temp_path, &final_path) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    #[cfg(windows)]
                    if final_path.exists() {
                        if let Err(validation) = validate_existing_final(&final_path) {
                            return cleanup_owned_temp(&temp_path, validation);
                        }
                        if let Err(remove) = fs::remove_file(&final_path) {
                            return cleanup_owned_temp(
                                &temp_path,
                                format!("replace receipt remove: {remove}"),
                            );
                        }
                        if let Err(rename) = fs::rename(&temp_path, &final_path) {
                            return cleanup_owned_temp(
                                &temp_path,
                                format!("publish receipt after replacement remove: {rename}"),
                            );
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
        read_valid_receipt(&final_path)?.ok_or_else(|| "receipt is absent".to_owned())
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
    match read_valid_receipt(path)? {
        Some(_) | None => Ok(()),
    }
}

fn read_valid_receipt(path: &Path) -> Result<Option<Vec<u8>>, String> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("read receipt final target: {error}")),
    };
    if unsafe_metadata(&metadata) || !metadata.is_file() {
        return Err("unsafe receipt final target".to_owned());
    }
    if metadata.len() > MAX_RECEIPT_BYTES as u64 {
        return Err("receipt exceeds size limit".to_owned());
    }
    let mut file = File::open(path).map_err(|error| format!("open receipt: {error}"))?;
    let opened = file
        .metadata()
        .map_err(|error| format!("read opened receipt: {error}"))?;
    if unsafe_metadata(&opened) || !opened.is_file() || opened.len() > MAX_RECEIPT_BYTES as u64 {
        return Err("unsafe receipt final target".to_owned());
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
