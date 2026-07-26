//! Compile/promote receipt boundary; the binary supplies repository and filesystem edges.

use crate::{
    InspectedCompilation, ReceiptStore, ReceiptWriteOutcome, encode_receipt, validate_receipt,
};

/// Persist one inspected compilation only after the caller completed every read-only check.
///
/// # Errors
///
/// Returns encoding or storage failures without changing source inputs.
pub fn compile(
    store: &impl ReceiptStore,
    inspection: &InspectedCompilation,
) -> Result<ReceiptWriteOutcome, String> {
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
        .map_err(|error| format!("promote receipt: {error}"))?
        .ok_or_else(|| {
            "promote refused: no compile receipt is stored. Run `dorc-loom compile` over the \
             same cases first -- promote publishes only an interpretation you have already \
             seen (`282:rul-promote-requires-fresh-compilation`)"
                .to_owned()
        })?;
    validate_receipt(&packet, inspection).map_err(|error| format!("promote refused: {error}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::receipt::tests::{inspection, inspection_mutations};

    #[derive(Default)]
    struct MemoryStore(RefCell<Option<Vec<u8>>>);

    impl ReceiptStore for MemoryStore {
        fn publish(&self, packet: &[u8]) -> Result<ReceiptWriteOutcome, String> {
            *self.0.borrow_mut() = Some(packet.to_vec());
            Ok(ReceiptWriteOutcome::Published)
        }

        fn read(&self) -> Result<Option<Vec<u8>>, String> {
            Ok(self.0.borrow().clone())
        }
    }

    #[derive(Default)]
    struct FailingStore {
        packet: RefCell<Option<Vec<u8>>>,
        fail_read: bool,
        fail_publish: bool,
        publishes: RefCell<usize>,
    }

    impl ReceiptStore for FailingStore {
        fn publish(&self, packet: &[u8]) -> Result<ReceiptWriteOutcome, String> {
            let next = (*self.publishes.borrow())
                .checked_add(1)
                .expect("test store publish counter overflow");
            *self.publishes.borrow_mut() = next;
            if self.fail_publish {
                return Err("publish failed".to_owned());
            }
            *self.packet.borrow_mut() = Some(packet.to_vec());
            Ok(ReceiptWriteOutcome::Published)
        }

        fn read(&self) -> Result<Option<Vec<u8>>, String> {
            if self.fail_read {
                return Err("read failed".to_owned());
            }
            Ok(self.packet.borrow().clone())
        }
    }

    /// A promote with no prior compile is the single most likely first-use mistake, and it used to
    /// surface as the store's raw "receipt is absent". The message must name the step that is
    /// missing, since nothing else in the output says a compile was ever expected.
    #[test]
    fn promoting_without_a_compile_names_the_missing_step() {
        let error = promote(&MemoryStore::default(), &inspection("current"))
            .expect_err("no receipt is stored");
        assert!(error.contains("dorc-loom compile"), "{error}");
        assert!(!error.contains("receipt is absent"), "{error}");
    }

    #[test]
    fn compile_then_promote_requires_the_same_inspection() {
        let store = MemoryStore::default();
        let current = inspection("current");
        assert_eq!(
            compile(&store, &current),
            Ok(ReceiptWriteOutcome::Published)
        );
        assert!(promote(&store, &current).is_ok());
        assert!(promote(&store, &inspection("stale")).is_err());
    }

    #[test]
    fn promote_refuses_every_changed_bound_dimension_without_writing() {
        let store = FailingStore::default();
        let (original, variants) = inspection_mutations();
        compile(&store, &original).expect("compile stores receipt");
        let packet = store.packet.borrow().clone();
        let publishes = *store.publishes.borrow();
        for changed in variants {
            assert!(promote(&store, &changed).is_err());
            assert_eq!(*store.publishes.borrow(), publishes);
            assert_eq!(*store.packet.borrow(), packet);
        }
        assert!(promote(&store, &original).is_ok());
        assert_eq!(*store.publishes.borrow(), publishes);
    }

    #[test]
    fn workflow_propagates_store_failures_and_preserves_prior_receipts() {
        let current = inspection("current");
        let source_before = current.clone();
        let prior = encode_receipt(&inspection("prior")).expect("prior receipt");
        let store = FailingStore {
            packet: RefCell::new(Some(prior.clone())),
            fail_publish: true,
            ..FailingStore::default()
        };
        assert!(compile(&store, &current).is_err());
        assert_eq!(current, source_before);
        assert_eq!(*store.packet.borrow(), Some(prior));
        assert_eq!(*store.publishes.borrow(), 1);

        let store = FailingStore {
            fail_read: true,
            ..FailingStore::default()
        };
        assert!(promote(&store, &current).is_err());
        assert_eq!(*store.publishes.borrow(), 0);
        assert!(store.packet.borrow().is_none());

        let absent = FailingStore::default();
        assert!(promote(&absent, &current).is_err());
        assert_eq!(*absent.publishes.borrow(), 0);
    }

    #[test]
    fn malformed_receipts_cannot_mint_a_witness_or_change_the_store() {
        let current = inspection("current");
        let valid = encode_receipt(&current).expect("valid receipt");
        let mut wrong_schema = valid.clone();
        let schema = wrong_schema
            .windows(b"schema: 1".len())
            .position(|window| window == b"schema: 1")
            .expect("schema header");
        wrong_schema[schema + b"schema: ".len()] = b'2';
        for packet in [
            Vec::new(),
            valid[..valid.len() - 1].to_vec(),
            [valid.clone(), b"trailing 0\n\n".to_vec()].concat(),
            wrong_schema,
        ] {
            let store = FailingStore {
                packet: RefCell::new(Some(packet)),
                ..FailingStore::default()
            };
            assert!(promote(&store, &current).is_err());
            assert_eq!(*store.publishes.borrow(), 0);
        }
    }
}
