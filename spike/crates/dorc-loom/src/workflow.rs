//! Compile/promote receipt boundary; the binary supplies repository and filesystem edges.

use crate::{InspectedCompilation, ReceiptStore, encode_receipt, validate_receipt};

/// Persist one inspected compilation only after the caller completed every read-only check.
///
/// # Errors
///
/// Returns encoding or storage failures without changing source inputs.
pub fn compile(store: &impl ReceiptStore, inspection: &InspectedCompilation) -> Result<(), String> {
    let packet = encode_receipt(inspection).map_err(|error| error.to_string())?;
    store.publish(&packet)
}

/// Require the stored receipt to exactly match the caller's fresh inspection.
///
/// # Errors
///
/// Returns a refusal without writing either the receipt or source inputs.
pub fn promote(store: &impl ReceiptStore, inspection: &InspectedCompilation) -> Result<(), String> {
    let packet = store
        .read()
        .map_err(|error| format!("promote receipt: {error}"))?;
    validate_receipt(&packet, inspection).map_err(|error| format!("promote refused: {error}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::receipt::tests::inspection;

    #[derive(Default)]
    struct MemoryStore(RefCell<Option<Vec<u8>>>);

    impl ReceiptStore for MemoryStore {
        fn publish(&self, packet: &[u8]) -> Result<(), String> {
            *self.0.borrow_mut() = Some(packet.to_vec());
            Ok(())
        }

        fn read(&self) -> Result<Vec<u8>, String> {
            self.0
                .borrow()
                .clone()
                .ok_or_else(|| "absent receipt".to_owned())
        }
    }

    #[test]
    fn compile_then_promote_requires_the_same_inspection() {
        let store = MemoryStore::default();
        let current = inspection("current");
        compile(&store, &current).expect("compile stores receipt");
        assert!(promote(&store, &current).is_ok());
        assert!(promote(&store, &inspection("stale")).is_err());
    }
}
