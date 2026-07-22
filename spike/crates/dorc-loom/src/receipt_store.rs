//! Narrow receipt persistence edge. Packet construction and validation stay pure.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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
        validate_directory(&target_root, "receipt target root")?;
        Ok(Self { target_root })
    }

    fn receipt_directory(&self, create: bool) -> Result<PathBuf, String> {
        validate_directory(&self.target_root, "receipt target root")?;
        let directory = self.target_root.join(RECEIPT_DIRECTORY);
        if create {
            ensure_directory(&directory)?;
        } else {
            validate_directory(&directory, "receipt directory")?;
        }
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
        validate_final(&final_path)?;

        for attempt in 0..TEMP_ATTEMPTS {
            let temp_path = directory.join(format!(".compile.receipt.{attempt}.tmp"));
            validate_temp(&temp_path)?;
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
                return cleanup_failure(&temp_path, error);
            }
            drop(file);

            validate_final(&final_path)?;
            match fs::rename(&temp_path, &final_path) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    // Windows may require remove-then-rename replacement.
                    #[cfg(windows)]
                    if final_path.exists() {
                        validate_final(&final_path)?;
                        fs::remove_file(&final_path)
                            .map_err(|remove| format!("replace receipt remove: {remove}"))?;
                        if let Err(rename) = fs::rename(&temp_path, &final_path) {
                            return Err(format!(
                                "publish receipt after replacement remove: {rename}"
                            ));
                        }
                        return Ok(());
                    }
                    return cleanup_failure(&temp_path, format!("publish receipt: {error}"));
                }
            }
        }
        Err("receipt temporary names exhausted".to_owned())
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        let directory = self.receipt_directory(false)?;
        let final_path = Self::final_path(&directory);
        validate_final(&final_path)?;
        let mut file = File::open(&final_path).map_err(|error| format!("open receipt: {error}"))?;
        let mut limited = Vec::new();
        Read::by_ref(&mut file)
            .take((MAX_RECEIPT_BYTES + 1) as u64)
            .read_to_end(&mut limited)
            .map_err(|error| format!("read receipt: {error}"))?;
        if limited.len() > MAX_RECEIPT_BYTES {
            return Err("receipt exceeds size limit".to_owned());
        }
        parse_receipt(&limited).map_err(|error: ReceiptError| error.to_string())?;
        Ok(limited)
    }
}

fn ensure_directory(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(_) => validate_directory(path, "receipt directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|create| format!("create receipt directory: {create}"))?;
            validate_directory(path, "receipt directory")
        }
        Err(error) => Err(format!("read receipt directory: {error}")),
    }
}

fn validate_directory(path: &Path, label: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| format!("read {label}: {error}"))?;
    if unsafe_metadata(&metadata) || !metadata.is_dir() {
        return Err(format!("unsafe {label}"));
    }
    Ok(())
}

fn validate_final(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if unsafe_metadata(&metadata) || !metadata.is_file() => {
            Err("unsafe receipt final target".to_owned())
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("read receipt final target: {error}")),
    }
}

fn validate_temp(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(_) => Err("receipt temporary target already exists".to_owned()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("read receipt temporary target: {error}")),
    }
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

fn cleanup_failure(temp_path: &Path, error: String) -> Result<(), String> {
    fs::remove_file(temp_path)
        .map_err(|cleanup| format!("{error}; cleanup receipt temporary: {cleanup}"))?;
    Err(error)
}
