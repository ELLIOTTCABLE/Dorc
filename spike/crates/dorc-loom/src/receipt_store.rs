//! Narrow receipt persistence edge. Packet construction and validation stay pure.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::receipt::parse as parse_receipt;
use crate::{MAX_RECEIPT_BYTES, ReceiptError};

/// The filesystem boundary used by compile and promote.
pub trait ReceiptStore {
    /// Publish an already encoded and validated receipt.
    ///
    /// # Errors
    /// Returns an I/O or receipt-validation refusal without publishing partial bytes.
    fn publish(&self, packet: &[u8]) -> Result<(), String>;
    /// Read one bounded, grammar-validated current receipt.
    ///
    /// # Errors
    /// Returns an I/O, unsafe-path, size, or receipt-validation refusal.
    fn read(&self) -> Result<Vec<u8>, String>;
}

/// Worktree-local receipt storage under the ignored Cargo target directory.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FsReceiptStore {
    final_path: PathBuf,
}

impl FsReceiptStore {
    /// Construct a store at an explicit ignored receipt path.
    #[must_use]
    pub fn new(final_path: impl Into<PathBuf>) -> Self {
        Self {
            final_path: final_path.into(),
        }
    }

    fn checked_parent(&self) -> Result<&Path, String> {
        let parent = self
            .final_path
            .parent()
            .ok_or_else(|| "receipt has no parent".to_owned())?;
        if self.final_path.file_name().is_none() || parent.as_os_str().is_empty() {
            return Err("unsafe receipt path".to_owned());
        }
        Ok(parent)
    }
}

impl ReceiptStore for FsReceiptStore {
    fn publish(&self, packet: &[u8]) -> Result<(), String> {
        parse_receipt(packet).map_err(|error| error.to_string())?;
        let parent = self.checked_parent()?;
        fs::create_dir_all(parent).map_err(|error| format!("create receipt parent: {error}"))?;
        if fs::symlink_metadata(&self.final_path)
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err("receipt final path is a symlink".to_owned());
        }
        for attempt in 0..16u8 {
            let temp = parent.join(format!(".dorc-loom-receipt-{attempt}.tmp"));
            let file = match OpenOptions::new().write(true).create_new(true).open(&temp) {
                Ok(file) => file,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(format!("create receipt temporary: {error}")),
            };
            let written = write_and_flush(file, packet);
            if let Err(error) = written {
                let _ = fs::remove_file(&temp);
                return Err(error);
            }
            match fs::rename(&temp, &self.final_path) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    let _ = fs::remove_file(&temp);
                    return Err(format!("publish receipt: {error}"));
                }
            }
        }
        Err("receipt temporary names exhausted".to_owned())
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        let metadata = fs::symlink_metadata(&self.final_path)
            .map_err(|error| format!("read receipt metadata: {error}"))?;
        if metadata.file_type().is_symlink()
            || !metadata.is_file()
            || metadata.len() > MAX_RECEIPT_BYTES as u64
        {
            return Err("unsafe or oversized receipt".to_owned());
        }
        let mut file =
            File::open(&self.final_path).map_err(|error| format!("open receipt: {error}"))?;
        let mut bytes =
            Vec::with_capacity(usize::try_from(metadata.len()).unwrap_or(MAX_RECEIPT_BYTES));
        Read::by_ref(&mut file)
            .take(u64::try_from(MAX_RECEIPT_BYTES.saturating_add(1)).unwrap_or(u64::MAX))
            .read_to_end(&mut bytes)
            .map_err(|error| format!("read receipt: {error}"))?;
        parse_receipt(&bytes).map_err(|error: ReceiptError| error.to_string())?;
        Ok(bytes)
    }
}

fn write_and_flush(mut file: File, packet: &[u8]) -> Result<(), String> {
    file.write_all(packet)
        .map_err(|error| format!("write receipt: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("flush receipt: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_refuses_invalid_packets_before_creating_a_file() {
        let path = std::env::temp_dir().join("dorc-loom-receipt-invalid.txt");
        let _ = fs::remove_file(&path);
        let store = FsReceiptStore::new(&path);
        assert!(store.publish(b"not a receipt").is_err());
        assert!(!path.exists());
    }
}
