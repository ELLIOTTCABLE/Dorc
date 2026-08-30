//! Every interruption of a receipt publication, what a later process finds, and the selection
//! the store does — and does not — offer.
//!
//! Driven through the REAL store, never through a sequence written out here: a sequence spelled
//! twice agrees with itself while quietly disagreeing with the code.
//!
//! The shape of most cases is the sweep's: interrupt one durable operation on one side, assert
//! the run actually REACHED that interruption, then restart from the disk it left and ask what a
//! later attempt finds. No interruption may mint a publication proof, and no restart may turn
//! bytes that happen to be on disk back into one.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use dorc_receipt::capability::ReceiptSigner as _;
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId, ReceiptId, ReceiptIdSource};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::writer::{DraftReceipt, SignedReceipt};
use dorc_receipt_crypto::Ed25519Signer;
use dorc_receipt_local::io::{FailureSchedule, GroupAndOtherAccess, IoFault, Op, Side};
use dorc_receipt_local::model::{ModelIo, Node, NodeKind};
use dorc_receipt_local::names::{NamedSpecies, ReceiptFileName};
use dorc_receipt_local::store::{
    CleanupFailure, DirectorySync, EntryStanding, EnumerateFailure, HeaderClaims, IncompleteState,
    LocalReceiptStoreV1, NameComponent, PublishFailure, PublishRefusal, StoreLimits,
    StoreOpenRefusal, StoreReadFailure, StoredSpecies,
};
use dorc_receipt_local::{LocalLimits, RootInputs, RootPlatform, RootRole};

const CONFIG_BASE: &str = "/cfg";
const STATE_BASE: &str = "/state";
const PRODUCT: &str = "/state/dorc";
const STORE: &str = "/state/dorc/receipts-v1";

/// A fixed secret, so every document in this file is reproducible.
const FIXTURE_SECRET: [u8; 32] = [11_u8; 32];

/// The durable operations ONE publication performs, in the order it performs them.
///
/// Enumerated here and checked against the vocabulary's own classification below, so an
/// operation that becomes durable without joining this list fails rather than going unswept.
const PUBLICATION_DURABLE_OPS: [Op; 4] = [
    Op::CreateFileExclusive,
    Op::WriteAll,
    Op::SyncFile,
    Op::SyncDirectory,
];

fn roots() -> RootInputs {
    RootInputs::of(RootPlatform::OtherUnix, CONFIG_BASE, STATE_BASE).expect("absolute bases")
}

/// A deterministic identity source. The production edge supplies one backed by the operating
/// system; nothing here reaches for either.
struct CountingIds(u8);

impl ReceiptIdSource for CountingIds {
    fn next_receipt_id(&mut self) -> ReceiptId {
        let mut raw = [0_u8; 32];
        if let Some(slot) = raw.first_mut() {
            *slot = self.0;
        }
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes(raw)
    }
}

fn plan_id(seed: u8) -> PlanReceiptId {
    PlanReceiptId::mint(&mut CountingIds(seed))
}

fn intent_id(seed: u8) -> ApplyIntentId {
    ApplyIntentId::mint(&mut CountingIds(seed))
}

fn outcome_id(seed: u8) -> ApplyOutcomeId {
    ApplyOutcomeId::mint(&mut CountingIds(seed))
}

/// One minimal signed document of the given species, carrying exactly the identity and order it
/// will be filed under.
fn document<D: StoredSpecies>(id_hex: &str, order: ReceiptOrderToken) -> SignedReceipt<D, Plain> {
    let key = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let row = SkeletonRecord::build(
        RecordKind::ProjectionOmission,
        ["observation", "0", "unminted", "authored-before-contact"]
            .iter()
            .map(|atom| (*atom).to_owned())
            .collect(),
    )
    .expect("a row the grammar admits");
    DraftReceipt::<D, Plain>::of(Skeleton {
        receipt_id: id_hex.to_owned(),
        order,
        signing_key_id: key.signing_key_id().hex(),
        encryption_key_id: None,
        records: vec![row],
    })
    .serialize()
    .expect("a plain draft serializes")
    .sign(&key)
}

/// The exact bytes of one such document, for planting a prefix of it.
fn document_bytes(id_hex: &str, order: ReceiptOrderToken) -> Vec<u8> {
    document::<PlanReceipt>(id_hex, order).into_bytes()
}

fn order(millis: u64) -> ReceiptOrderToken {
    ReceiptOrderToken::of_controller_millis(millis)
}

/// A clean disk: the platform bases exist and nothing of this project's does.
fn clean(schedule: FailureSchedule) -> ModelIo {
    ModelIo::new(schedule, DirectorySync::Synchronized)
        .planting(CONFIG_BASE, Node::private_directory())
        .planting(STATE_BASE, Node::private_directory())
}

/// A disk whose store already exists, under an intact schedule.
///
/// The sweep opens this READ-ONLY, so the only durable operations a case performs are the
/// publication's own — which is what lets a schedule name an occurrence without counting the
/// store open's operations first.
fn seeded() -> ModelIo {
    let mut io = clean(FailureSchedule::intact());
    LocalReceiptStoreV1::open_or_create(&roots(), &mut io, StoreLimits::V1)
        .expect("a clean profile opens a store");
    io
}

fn open_read(io: &mut ModelIo) -> LocalReceiptStoreV1 {
    LocalReceiptStoreV1::open_for_read(&roots(), io, StoreLimits::V1).expect("the store is there")
}

/// Publish one plan document, answering the refusal or the proof's filename.
fn publish_plan(
    store: &LocalReceiptStoreV1,
    io: &mut ModelIo,
    seed: u8,
    at: ReceiptOrderToken,
) -> Result<String, PublishRefusal> {
    let id = plan_id(seed);
    let policy = store.required_policy();
    store
        .publish_required_v1::<PlanReceipt, Plain>(io, at, id, document(&id.hex(), at), policy)
        .map(|proof| proof.file_name().spelled())
}

fn refusal(outcome: Result<String, PublishRefusal>) -> PublishRefusal {
    match outcome {
        Ok(name) => panic!("the publication was expected to fail and placed {name}"),
        Err(refusal) => refusal,
    }
}

#[test]
fn a_clean_profile_publishes_and_a_later_attempt_reads_the_exact_bytes_back() {
    let mut io = clean(FailureSchedule::intact());
    let store = LocalReceiptStoreV1::open_or_create(&roots(), &mut io, StoreLimits::V1)
        .expect("a clean profile opens a store");

    let id = plan_id(1);
    let at = order(1_700_000_000_000);
    let expected = document_bytes(&id.hex(), at);
    let policy = store.required_policy();
    let proof = store
        .publish_required_v1::<PlanReceipt, Plain>(&mut io, at, id, document(&id.hex(), at), policy)
        .expect("the publication succeeds");

    // The proof is bound to the exact document rather than reporting a grade.
    assert_eq!(proof.receipt_id(), id);
    assert_eq!(proof.order(), at);
    assert_eq!(proof.file_name().species(), NamedSpecies::Plan);
    assert_eq!(proof.file_name().receipt_id(), id.hex());
    assert!(proof.properties().file_is_durable());
    assert_eq!(proof.properties().directory(), DirectorySync::Synchronized);

    // A genuinely later attempt, from the disk this one left, finds the exact bytes.
    let mut later = io.restart(FailureSchedule::intact());
    let store = open_read(&mut later);
    let walk = store.enumerate(&mut later).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 1);
    assert!(walk.unrecognized().is_empty());
    let entry = walk.recognized().first().expect("one entry");
    let read = store.read(&mut later, entry).expect("it reads back");
    assert_eq!(read.standing(), EntryStanding::CompleteBytes);
    assert_eq!(read.byte_length(), expected.len());
}

#[test]
fn a_second_document_under_one_name_is_refused_and_the_first_is_untouched() {
    // Direct final-name creation IS the atomicity, so a taken name has to be a refusal rather
    // than a replacement. The whole store's immutability rests on this one answer.
    let mut io = seeded();
    let store = open_read(&mut io);
    let at = order(1_700_000_000_000);
    let name = publish_plan(&store, &mut io, 1, at).expect("the first succeeds");
    let placed = io
        .at(&format!("{STORE}/{name}"))
        .and_then(Node::bytes)
        .map(<[u8]>::to_vec)
        .expect("the first document is on disk");

    let second = refusal(publish_plan(&store, &mut io, 1, at));
    assert_eq!(second.reason(), PublishFailure::NameAlreadyTaken);
    assert!(
        second.into_incomplete().is_none(),
        "a refusal before the create left nothing to own"
    );
    assert_eq!(
        io.at(&format!("{STORE}/{name}")).and_then(Node::bytes),
        Some(placed.as_slice()),
        "the first document's bytes are byte-identical after the refusal"
    );
}

#[test]
fn the_durable_operations_a_publication_performs_are_the_ones_this_file_sweeps() {
    // Two-way against the vocabulary's own classification. An operation that becomes durable
    // without joining the list would go unswept, and the interruption that leaves a half-written
    // receipt is exactly the one that would go missing.
    for op in PUBLICATION_DURABLE_OPS {
        assert!(op.is_durable(), "{op:?}");
    }
    let mut io = seeded();
    let store = open_read(&mut io);
    publish_plan(&store, &mut io, 1, order(5)).expect("one publication");
    let reached: Vec<Op> = io
        .schedule()
        .arrivals()
        .iter()
        .filter(|(op, side)| op.is_durable() && *side == Side::Before)
        .map(|(op, _)| *op)
        .collect();
    for op in PUBLICATION_DURABLE_OPS {
        assert!(
            reached.contains(&op),
            "the publication never reached {op:?}; the sweep below would interrupt nothing"
        );
    }
}

#[test]
fn no_interruption_of_a_publication_mints_a_proof_and_none_is_reconstructed_on_restart() {
    // The sweep. Every durable operation, both sides, driven through the real store.
    let base = seeded();
    let at = order(1_700_000_000_000);
    for op in PUBLICATION_DURABLE_OPS {
        for side in [Side::Before, Side::After] {
            let mut io = base.restart(FailureSchedule::faulting(op, side, IoFault::Platform));
            let store = open_read(&mut io);
            let outcome = publish_plan(&store, &mut io, 1, at);
            assert!(
                outcome.is_err(),
                "{op:?}/{side:?} produced a proof despite being interrupted"
            );
            assert!(
                io.schedule()
                    .arrivals()
                    .iter()
                    .any(|(seen, seen_side)| *seen == op && *seen_side == side),
                "{op:?}/{side:?} was never reached, so the case interrupted nothing"
            );

            // And from the disk it left: whatever is there, a later attempt cannot turn it into
            // a publication. The name is either free or taken, and a taken one is refused.
            let mut later = io.restart(FailureSchedule::intact());
            let store = open_read(&mut later);
            let walk = store.enumerate(&mut later).expect("the walk answers");
            let retry = publish_plan(&store, &mut later, 1, at);
            match walk.recognized().len() {
                0 => assert!(
                    retry.is_ok(),
                    "{op:?}/{side:?} left no entry, so the name is free"
                ),
                1 => assert_eq!(
                    refusal(retry).reason(),
                    PublishFailure::NameAlreadyTaken,
                    "{op:?}/{side:?} left an entry, and no later writer replaces one"
                ),
                other => panic!("{op:?}/{side:?} left {other} entries"),
            }
        }
    }
}

#[test]
fn an_interruption_before_the_create_leaves_nothing_and_one_after_it_leaves_an_unowned_object() {
    // The two sides of the create are different disks, and they are also different OWNERSHIPS. A
    // fault before it leaves no object; a fault reported after it leaves one this attempt was
    // told it does not have — so the refusal hands back no ownership, and what is on disk stays
    // as bounded partial evidence rather than being removed by name.
    let base = seeded();
    let at = order(9);

    let mut before = base.restart(FailureSchedule::faulting(
        Op::CreateFileExclusive,
        Side::Before,
        IoFault::Denied,
    ));
    let store = open_read(&mut before);
    let refused = refusal(publish_plan(&store, &mut before, 1, at));
    assert_eq!(refused.reason(), PublishFailure::RootUnusable);
    assert!(refused.into_incomplete().is_none());
    let walk = store
        .enumerate(&mut before)
        .expect("the walk answers over an empty store");
    assert!(walk.recognized().is_empty() && walk.unrecognized().is_empty());

    let mut after = base.restart(FailureSchedule::faulting(
        Op::CreateFileExclusive,
        Side::After,
        IoFault::Platform,
    ));
    let store = open_read(&mut after);
    let refused = refusal(publish_plan(&store, &mut after, 1, at));
    assert_eq!(refused.reason(), PublishFailure::CreateFailed);
    assert!(
        refused.into_incomplete().is_none(),
        "an attempt told its create failed cannot claim to own what it finds"
    );
    let store = open_read(&mut after);
    let walk = store.enumerate(&mut after).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 1, "the object is there");
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(
        store.read(&mut after, entry).expect("it reads").standing(),
        EntryStanding::IncompletePublication {
            state: IncompleteState::InProgressOrAbandoned
        },
        "and an empty file is never a whole document"
    );
}

#[test]
fn a_write_that_reached_the_disk_and_reported_failure_still_mints_no_proof() {
    // The sharpest arm of the sweep: the bytes are all there and complete, and the attempt was
    // told the write failed. A later reader finds a document that reads back whole — and there
    // is no proof, because a required publication is a runtime fact rather than something
    // reconstructed from finding complete bytes on disk.
    let at = order(1_700_000_000_000);
    let mut io = seeded().restart(FailureSchedule::faulting(
        Op::WriteAll,
        Side::After,
        IoFault::Platform,
    ));
    let store = open_read(&mut io);
    let refused = refusal(publish_plan(&store, &mut io, 1, at));
    assert_eq!(refused.reason(), PublishFailure::WriteIncomplete);
    let owned = refused
        .into_incomplete()
        .expect("a failure after the create owns what it made");

    let store = open_read(&mut io);
    let walk = store.enumerate(&mut io).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(
        store.read(&mut io, entry).expect("it reads").standing(),
        EntryStanding::CompleteBytes,
        "the bytes are whole, which is exactly why nothing may promote them"
    );

    // The ownership is the caller's to spend or drop. Spending it removes what this attempt
    // made, and nothing in the crate did that on its own.
    assert_eq!(store.remove_owned(&mut io, owned), Ok(()));
    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert!(walk.recognized().is_empty());
}

#[test]
fn a_synchronization_failure_is_reported_once_and_never_retried() {
    // Retrying a failed synchronization can report success over pages the kernel already
    // discarded, so the count is the assertion: one arrival, one answer, one refusal.
    for op in [Op::SyncFile, Op::SyncDirectory] {
        let mut io = seeded().restart(FailureSchedule::faulting(
            op,
            Side::After,
            IoFault::Platform,
        ));
        let store = open_read(&mut io);
        let refused = refusal(publish_plan(&store, &mut io, 1, order(3)));
        assert_eq!(refused.reason(), PublishFailure::SyncFailed, "{op:?}");
        assert!(refused.into_incomplete().is_some(), "{op:?}");
        let attempts = io
            .schedule()
            .arrivals()
            .iter()
            .filter(|(seen, side)| *seen == op && *side == Side::Before)
            .count();
        assert_eq!(attempts, 1, "{op:?} was attempted {attempts} times");
    }
}

#[test]
fn the_windows_shaped_store_publishes_and_records_the_operation_it_does_not_have() {
    // The platform posture, end to end. The proof records the missing directory synchronization
    // as a property rather than simulating a success of a stronger kind, and the required
    // baseline it is judged against is the one this store validated its own root under.
    let roots =
        RootInputs::of(RootPlatform::Windows, "C:\\Roaming", "C:\\Local").expect("absolute bases");
    let mut io = ModelIo::windows_shaped(FailureSchedule::intact())
        .planting("C:\\Roaming", Node::private_directory())
        .planting("C:\\Local", Node::private_directory());
    let store = LocalReceiptStoreV1::open_or_create(&roots, &mut io, StoreLimits::V1)
        .expect("a clean Windows profile opens a store");

    let id = plan_id(4);
    let at = order(77);
    let policy = store.required_policy();
    let proof = store
        .publish_required_v1::<PlanReceipt, Plain>(&mut io, at, id, document(&id.hex(), at), policy)
        .expect("the Windows baseline publishes");
    assert_eq!(
        proof.properties().directory(),
        DirectorySync::UnavailableOnPlatform
    );
    assert!(proof.properties().file_is_durable());

    // And the walk sees it, which is the half a backslash-blind model would have called empty.
    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 1);
}

#[test]
fn unknown_and_conflict_names_count_against_the_walk_and_mint_no_receipt() {
    // Everything in the directory costs budget, recognized or not. A sync client's conflict name
    // is retained as a finding and is never deleted, repaired, or normalized into a receipt.
    let mut io = seeded();
    let store = open_read(&mut io);
    let name = publish_plan(&store, &mut io, 1, order(20)).expect("one real receipt");

    let conflict = name.replace(".dorc-receipt", ".sync-conflict-20260825.dorc-receipt");
    let mut io = io
        .restart(FailureSchedule::intact())
        .planting(&format!("{STORE}/{conflict}"), Node::private_file(b"x"))
        .planting(&format!("{STORE}/README.txt"), Node::private_file(b"y"))
        .planting(
            &format!("{STORE}/plan-v1-0000-short.dorc-receipt"),
            Node::private_file(b"z"),
        );
    let store = open_read(&mut io);
    let walk = store.enumerate(&mut io).expect("the walk answers");

    assert_eq!(walk.walked(), 4, "everything in the directory was counted");
    assert_eq!(walk.recognized().len(), 1, "and exactly one is a receipt");
    assert_eq!(walk.unrecognized().len(), 3);
    assert!(
        walk.unrecognized()
            .iter()
            .any(|entry| entry.name() == conflict),
        "the conflict name is retained as a finding: {:?}",
        walk.unrecognized()
    );
    assert!(
        io.at(&format!("{STORE}/{conflict}")).is_some(),
        "and nothing removed it"
    );
}

#[test]
fn a_walk_past_the_entry_bound_refuses_rather_than_answering_a_short_listing() {
    // A truncated listing would be indistinguishable from a complete one, and could hide the
    // very entry a selection wanted. The bound is lowered here rather than filling a directory
    // with four thousand files: lowering a limit is a legitimate local act.
    let narrow = StoreLimits {
        receipt: ReceiptLimits::V1,
        local: LocalLimits {
            store_entries: 2,
            ..LocalLimits::V1
        },
    };
    let mut io = seeded();
    let store = open_read(&mut io);
    for (seed, at) in [(1_u8, 10_u64), (2, 20)] {
        publish_plan(&store, &mut io, seed, order(at)).expect("a publication");
    }
    let narrow_store =
        LocalReceiptStoreV1::open_for_read(&roots(), &mut io, narrow).expect("the store is there");
    assert!(
        narrow_store.enumerate(&mut io).is_ok(),
        "two entries are within a bound of two"
    );

    publish_plan(&store, &mut io, 3, order(30)).expect("a third publication");
    assert_eq!(
        narrow_store.enumerate(&mut io),
        Err(EnumerateFailure::OverEntryBound),
        "and the third is one past it, which the walk SAW rather than went quiet at"
    );
}

#[test]
fn the_newest_candidate_being_partial_never_selects_an_older_complete_one() {
    // The property `30Rd` is most emphatic about, and the store answers it structurally: the ONE
    // selection is the maximum-order cohort, so there is no call that could return the older
    // complete document. A caller finding this member partial has to report it.
    let mut io = seeded();
    let store = open_read(&mut io);
    let older = order(1_000);
    let complete = publish_plan(&store, &mut io, 1, older).expect("an older complete receipt");

    let newer = order(2_000);
    let newer_id = plan_id(9);
    let whole = document_bytes(&newer_id.hex(), newer);
    let partial = whole.get(..whole.len() / 2).expect("a prefix").to_vec();
    let newer_name = ReceiptFileName::of(NamedSpecies::Plan, newer, &newer_id.hex())
        .expect("a mintable name")
        .spelled();
    let mut io = io.restart(FailureSchedule::intact()).planting(
        &format!("{STORE}/{newer_name}"),
        Node::private_file(&partial),
    );

    let store = open_read(&mut io);
    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 2);
    let cohort = walk.maximum_order_cohort().expect("a cohort");
    assert_eq!(cohort.order(), newer);
    assert!(!cohort.is_ambiguous());
    let member = cohort.members().first().expect("one member");
    assert_eq!(member.name().spelled(), newer_name);
    assert_eq!(
        store.read(&mut io, member).expect("it reads").standing(),
        EntryStanding::IncompletePublication {
            state: IncompleteState::InProgressOrAbandoned
        }
    );

    // The older complete document is still there, and reaching it took naming it: no selection
    // handed it over as the answer to "the last one".
    let older_entry = walk
        .recognized()
        .iter()
        .find(|entry| entry.name().spelled() == complete)
        .expect("the older entry is enumerable");
    assert_eq!(older_entry.order(), older);
}

#[test]
fn equal_greatest_orders_stay_an_ambiguity_rather_than_being_tie_broken() {
    // Choosing one by receipt identity would be picking a document by the value least related to
    // when it was written. The cohort carries the ambiguity instead.
    let mut io = seeded();
    let store = open_read(&mut io);
    let at = order(4_242);
    let policy = store.required_policy();

    let plan = plan_id(1);
    store
        .publish_required_v1::<PlanReceipt, Plain>(
            &mut io,
            at,
            plan,
            document(&plan.hex(), at),
            policy,
        )
        .expect("a plan at the order");
    let intent = intent_id(2);
    store
        .publish_required_v1::<ApplyIntent, Plain>(
            &mut io,
            at,
            intent,
            document(&intent.hex(), at),
            policy,
        )
        .expect("an intent at the same order");
    let outcome = outcome_id(3);
    store
        .publish_required_v1::<ApplyOutcome, Plain>(
            &mut io,
            at,
            outcome,
            document(&outcome.hex(), at),
            policy,
        )
        .expect("an outcome at the same order");

    let walk = store.enumerate(&mut io).expect("the walk answers");
    let cohort = walk.maximum_order_cohort().expect("a cohort");
    assert_eq!(cohort.order(), at);
    assert!(cohort.is_ambiguous());
    assert_eq!(cohort.members().len(), 3);

    // Deterministic for display: two walks of one store answer in one order.
    let again = store.enumerate(&mut io).expect("the walk answers again");
    let first: Vec<String> = cohort
        .members()
        .iter()
        .map(|entry| entry.name().spelled())
        .collect();
    let second: Vec<String> = again
        .maximum_order_cohort()
        .expect("a cohort")
        .members()
        .iter()
        .map(|entry| entry.name().spelled())
        .collect();
    assert_eq!(first, second);
}

#[test]
fn an_undated_receipt_sorts_lowest_and_is_stored_and_read_like_any_other() {
    // Clocklessness is a capability, not a test artifact: the store handles an all-zeroes order
    // rather than refusing it. What must never happen — an undated document silently causing an
    // older one to be selected — is prevented by it sorting LOWEST, so it never leads a cohort
    // that has any dated member.
    let mut io = seeded();
    let store = open_read(&mut io);
    publish_plan(&store, &mut io, 1, ReceiptOrderToken::UNDATED).expect("an undated publication");
    let dated = publish_plan(&store, &mut io, 2, order(1)).expect("a dated one");

    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 2);
    let cohort = walk.maximum_order_cohort().expect("a cohort");
    assert_eq!(cohort.members().len(), 1);
    assert_eq!(
        cohort
            .members()
            .first()
            .expect("one member")
            .name()
            .spelled(),
        dated,
        "the undated document sorts below the dated one"
    );

    // And on its own it is an ordinary readable receipt.
    let mut alone = clean(FailureSchedule::intact());
    let store = LocalReceiptStoreV1::open_or_create(&roots(), &mut alone, StoreLimits::V1)
        .expect("a store");
    publish_plan(&store, &mut alone, 1, ReceiptOrderToken::UNDATED).expect("an undated one");
    let walk = store.enumerate(&mut alone).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(entry.order(), ReceiptOrderToken::UNDATED);
    assert_eq!(
        store.read(&mut alone, entry).expect("it reads").standing(),
        EntryStanding::CompleteBytes
    );
}

#[test]
fn a_filename_disagreeing_with_its_document_is_a_finding_and_decides_nothing() {
    // A filename is a selection hint. The authenticated header is what counts, and the store's
    // job is to say WHERE they differ rather than to prefer either.
    let mut io = seeded();
    let store = open_read(&mut io);
    let at = order(500);
    let id = plan_id(1);
    let policy = store.required_policy();
    store
        .publish_required_v1::<PlanReceipt, Plain>(&mut io, at, id, document(&id.hex(), at), policy)
        .expect("a publication");
    let walk = store.enumerate(&mut io).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry");

    let truthful = HeaderClaims {
        version: "dorc-receipt/1",
        species: "plan",
        order: at,
        receipt_id: &id.hex(),
    };
    assert!(entry.agreement(&truthful).agrees());

    let wrong_everything = HeaderClaims {
        version: "dorc-receipt/2",
        species: "apply-intent",
        order: order(501),
        receipt_id: &plan_id(2).hex(),
    };
    let finding = entry.agreement(&wrong_everything);
    assert!(!finding.agrees());
    assert_eq!(
        finding.disagreements(),
        [
            NameComponent::Version,
            NameComponent::Species,
            NameComponent::Order,
            NameComponent::ReceiptId
        ],
        "every disagreeing component is named, not just the first"
    );

    // And the entry is unchanged by having been asked: a finding is not a rejection.
    assert_eq!(
        store.read(&mut io, entry).expect("it reads").standing(),
        EntryStanding::CompleteBytes
    );
}

#[test]
fn every_prefix_of_a_published_document_reads_back_short_of_complete() {
    // The property that makes direct final-name creation safe: an interrupted publication cannot
    // produce something a reader mistakes for whole. Exhaustive over prefixes rather than one
    // representative, because the interesting cuts are at the span boundaries.
    let at = order(1_700_000_000_000);
    let id = plan_id(1);
    let whole = document_bytes(&id.hex(), at);
    let name = ReceiptFileName::of(NamedSpecies::Plan, at, &id.hex())
        .expect("a mintable name")
        .spelled();
    let base = seeded();
    for cut in 0..whole.len() {
        let prefix = whole.get(..cut).expect("a prefix").to_vec();
        let mut io = base
            .restart(FailureSchedule::intact())
            .planting(&format!("{STORE}/{name}"), Node::private_file(&prefix));
        let store = open_read(&mut io);
        let walk = store.enumerate(&mut io).expect("the walk answers");
        let entry = walk.recognized().first().expect("one entry");
        let standing = store.read(&mut io, entry).expect("it reads").standing();
        // The strong form: a prefix is INCOMPLETE, never merely not-complete. Measured over this
        // document's shape — every cut lands either inside the opening line or at a span the
        // locator then cannot close — so a prefix reading as damage would mean the classifier had
        // started calling interrupted publications foreign bytes.
        assert_eq!(
            standing,
            EntryStanding::IncompletePublication {
                state: IncompleteState::InProgressOrAbandoned
            },
            "a {cut}-byte prefix of {} bytes read as {standing:?}",
            whole.len()
        );
    }

    // The positive control: the whole document does read complete, so the assertion above is not
    // satisfied by every input whatsoever.
    let mut io = base
        .restart(FailureSchedule::intact())
        .planting(&format!("{STORE}/{name}"), Node::private_file(&whole));
    let store = open_read(&mut io);
    let walk = store.enumerate(&mut io).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(
        store.read(&mut io, entry).expect("it reads").standing(),
        EntryStanding::CompleteBytes
    );

    // And the other control: bytes that are WHOLE and are not this format read as damage rather
    // than as a publication somebody might still be finishing. Without this the incomplete arm
    // above would be satisfied by a classifier that never says anything else.
    let mut foreign = base.restart(FailureSchedule::intact()).planting(
        &format!("{STORE}/{name}"),
        Node::private_file(b"some-other-format/9\nwhatever\n"),
    );
    let store = open_read(&mut foreign);
    let walk = store.enumerate(&mut foreign).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(
        store
            .read(&mut foreign, entry)
            .expect("it reads")
            .standing(),
        EntryStanding::Damaged
    );
}

#[test]
fn an_entry_is_only_readable_through_the_store_that_walked_it() {
    // A name is never enough. An entry carries the root it came from, so one store's handle
    // cannot be spent against another's directory even where the spelling would resolve.
    let mut io = seeded();
    let store = open_read(&mut io);
    publish_plan(&store, &mut io, 1, order(1)).expect("a publication");
    let walk = store.enumerate(&mut io).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry").clone();

    let other_roots =
        RootInputs::of(RootPlatform::OtherUnix, CONFIG_BASE, "/elsewhere").expect("absolute bases");
    let mut other = io
        .restart(FailureSchedule::intact())
        .planting("/elsewhere", Node::private_directory());
    let other_store =
        LocalReceiptStoreV1::open_or_create(&other_roots, &mut other, StoreLimits::V1)
            .expect("a second store");
    match other_store.read(&mut other, &entry) {
        Err(StoreReadFailure::NotThisStore) => {}
        other => panic!("another store's entry read as {:?}", other.map(|_| "bytes")),
    }
}

#[test]
fn an_entry_that_vanished_between_the_walk_and_the_read_is_not_a_receipt() {
    // POSIX leaves concurrent additions and removals during a walk unspecified, so a listing is
    // never a snapshot. The read answers what it found rather than what the walk promised.
    let mut io = seeded();
    let store = open_read(&mut io);
    publish_plan(&store, &mut io, 1, order(1)).expect("a publication");
    let walk = store.enumerate(&mut io).expect("the walk answers");
    let entry = walk.recognized().first().expect("one entry").clone();

    let mut emptied = seeded();
    let store = open_read(&mut emptied);
    match store.read(&mut emptied, &entry) {
        Err(StoreReadFailure::Vanished) => {}
        other => panic!("a vanished entry read as {:?}", other.map(|_| "bytes")),
    }
}

#[test]
fn a_removal_refuses_an_object_this_attempt_did_not_create() {
    // The compound shape: a publication fails, and the cleanup that follows fails too. What must
    // NOT happen is the failure broadening into removal by pathname — so an attempt that no
    // longer owns the object is refused, and the object stays.
    let at = order(88);
    let mut io = seeded().restart(FailureSchedule::faulting(
        Op::SyncFile,
        Side::After,
        IoFault::Platform,
    ));
    let store = open_read(&mut io);
    let owned = refusal(publish_plan(&store, &mut io, 1, at))
        .into_incomplete()
        .expect("the failure owns what it made");
    let name = owned.file_name().spelled();

    // A restart is what genuinely loses the ownership: a later process finds the object and did
    // not create it.
    let mut later = io.restart(FailureSchedule::intact());
    let store = open_read(&mut later);
    assert_eq!(
        store.remove_owned(&mut later, owned),
        Err(CleanupFailure::NotOwned)
    );
    assert!(
        later.at(&format!("{STORE}/{name}")).is_some(),
        "and the object is still there"
    );
}

#[test]
fn a_cleanup_never_reaches_an_existing_receipt() {
    // One publication succeeds and a second fails. Spending the second's ownership removes the
    // second's object and cannot reach the first, because the token names one object rather than
    // a directory to tidy.
    let mut io = seeded();
    let store = open_read(&mut io);
    let kept = publish_plan(&store, &mut io, 1, order(10)).expect("a successful publication");

    let mut io = io.restart(FailureSchedule::faulting(
        Op::WriteAll,
        Side::Before,
        IoFault::Platform,
    ));
    let store = open_read(&mut io);
    let owned = refusal(publish_plan(&store, &mut io, 2, order(20)))
        .into_incomplete()
        .expect("the failure owns what it made");
    assert_eq!(store.remove_owned(&mut io, owned), Ok(()));

    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 1);
    assert_eq!(
        walk.recognized()
            .first()
            .expect("one entry")
            .name()
            .spelled(),
        kept
    );
}

#[test]
fn the_aggregate_budget_stops_a_graph_build_before_it_retains_the_next_document() {
    // The aggregate is checked BEFORE the next document is retained, and the read is bounded by
    // whichever is smaller — so nothing is allocated from a length a file declared.
    let mut io = seeded();
    let store = open_read(&mut io);
    for (seed, at) in [(1_u8, 10_u64), (2, 20)] {
        publish_plan(&store, &mut io, seed, order(at)).expect("a publication");
    }
    let walk = store.enumerate(&mut io).expect("the walk answers");
    let first = walk.recognized().first().expect("two entries");
    let one_document = store.read(&mut io, first).expect("it reads").byte_length();

    let narrow = StoreLimits {
        receipt: ReceiptLimits::V1,
        local: LocalLimits {
            graph_bytes: one_document as u64,
            ..LocalLimits::V1
        },
    };
    let narrow_store =
        LocalReceiptStoreV1::open_for_read(&roots(), &mut io, narrow).expect("the store is there");
    let walk = narrow_store.enumerate(&mut io).expect("the walk answers");
    let mut budget = narrow_store.graph_budget();
    let mut outcomes = Vec::new();
    for entry in walk.recognized() {
        outcomes.push(
            narrow_store
                .read_into_budget(&mut io, entry, &mut budget)
                .map(|read| read.byte_length()),
        );
    }
    assert_eq!(outcomes.first(), Some(&Ok(one_document)));
    assert_eq!(
        outcomes.get(1),
        Some(&Err(StoreReadFailure::OverGraphBudget)),
        "the second document is refused rather than read"
    );
    assert_eq!(budget.remaining(), 0);
}

#[test]
fn a_store_root_anyone_but_the_owner_may_write_is_refused() {
    // The store's own permission rule, and it is deliberately weaker than the keyset's: a root
    // others may READ is accepted, because receipts are created owner-only and readability of
    // the containing directory does not let another account plant entries. A root others may
    // WRITE does, and is refused.
    for (access, admitted) in [
        (GroupAndOtherAccess::None, true),
        (GroupAndOtherAccess::Present, true),
        (GroupAndOtherAccess::Writable, false),
        (GroupAndOtherAccess::NotInspectable, false),
    ] {
        let mut io = clean(FailureSchedule::intact())
            .planting(PRODUCT, Node::private_directory())
            .planting(STORE, Node::of(NodeKind::Directory, access));
        let outcome = LocalReceiptStoreV1::open_for_read(&roots(), &mut io, StoreLimits::V1);
        assert_eq!(
            outcome.is_ok(),
            admitted,
            "a Unix store root answering {access:?} was {outcome:?}"
        );
    }
}

#[test]
fn a_product_root_anyone_may_write_refuses_a_store_that_is_itself_fine() {
    // Both Dorc-owned components are validated, not just the last one. A store reached through a
    // product root another account may write is a store whose own directory could be replaced
    // between one attempt and the next, so the component above it is checked too — and the two
    // open paths check the same pair, or a read would accept what a write refuses.
    for open_or_create in [false, true] {
        let mut io = clean(FailureSchedule::intact())
            .planting(
                PRODUCT,
                Node::of(NodeKind::Directory, GroupAndOtherAccess::Writable),
            )
            .planting(STORE, Node::private_directory());
        let outcome = if open_or_create {
            LocalReceiptStoreV1::open_or_create(&roots(), &mut io, StoreLimits::V1)
        } else {
            LocalReceiptStoreV1::open_for_read(&roots(), &mut io, StoreLimits::V1)
        };
        assert_eq!(
            outcome,
            Err(StoreOpenRefusal::PermissionRefused),
            "open_or_create={open_or_create}"
        );
    }

    // The positive control: narrow the product root and the same store opens both ways.
    for open_or_create in [false, true] {
        let mut io = clean(FailureSchedule::intact())
            .planting(PRODUCT, Node::private_directory())
            .planting(STORE, Node::private_directory());
        let outcome = if open_or_create {
            LocalReceiptStoreV1::open_or_create(&roots(), &mut io, StoreLimits::V1)
        } else {
            LocalReceiptStoreV1::open_for_read(&roots(), &mut io, StoreLimits::V1)
        };
        assert!(
            outcome.is_ok(),
            "open_or_create={open_or_create}: {outcome:?}"
        );
    }
}

#[test]
fn a_store_root_that_is_a_redirect_or_the_wrong_kind_is_refused_without_being_followed() {
    let mut linked = clean(FailureSchedule::intact())
        .planting(PRODUCT, Node::private_directory())
        .planting(STORE, Node::private_directory().redirected());
    assert_eq!(
        LocalReceiptStoreV1::open_for_read(&roots(), &mut linked, StoreLimits::V1),
        Err(StoreOpenRefusal::PermissionRefused)
    );

    let mut file = clean(FailureSchedule::intact())
        .planting(PRODUCT, Node::private_directory())
        .planting(STORE, Node::private_file(b"not a store"));
    assert_eq!(
        LocalReceiptStoreV1::open_for_read(&roots(), &mut file, StoreLimits::V1),
        Err(StoreOpenRefusal::NotADirectory)
    );

    let mut absent = clean(FailureSchedule::intact());
    assert_eq!(
        LocalReceiptStoreV1::open_for_read(&roots(), &mut absent, StoreLimits::V1),
        Err(StoreOpenRefusal::NotInitialized),
        "and the read-only path answers absence rather than creating one"
    );
    assert!(
        absent.at(PRODUCT).is_none() && absent.at(STORE).is_none(),
        "asking created {:?}",
        absent.paths()
    );
}

// ---------------------------------------------------------------------------
// the explicit store root: the location moves, the contract does not

/// Where an admin-named folder sits in this model's world, and the roots that name it.
const NAMED: &str = "/elsewhere/receipts";

fn named_roots() -> RootInputs {
    roots()
        .with_store_root(NAMED)
        .expect("an absolute folder is a store root")
}

/// The named folder IS the root: one owned component, and nothing appended beneath it.
///
/// The default selection owns two components and the explicit one owns exactly itself — nothing
/// above an admin's folder is Dorc's to create. A `receipts-v1` beneath it would put the documents
/// one level below the directory the admin named.
#[test]
fn an_explicit_root_creates_exactly_the_named_folder() {
    let mut io = clean(FailureSchedule::intact()).planting("/elsewhere", Node::private_directory());
    let store = LocalReceiptStoreV1::open_or_create(&named_roots(), &mut io, StoreLimits::V1)
        .expect("the named folder opens");

    assert_eq!(store.root().as_str(), NAMED);
    assert!(io.at(NAMED).is_some(), "the folder was created");
    assert!(
        io.at("/elsewhere/receipts/receipts-v1").is_none(),
        "no component is appended beneath the folder the admin named"
    );
    assert!(
        io.at(STORE).is_none() && io.at(PRODUCT).is_none(),
        "and the standard store is untouched: {:?}",
        io.paths()
    );
}

/// The explicit root is validated exactly as the standard one is.
///
/// The point of routing both through one seat: an admin-named folder is not a weaker landing, so
/// a redirect, a non-directory, and a world-writable root are refused there for the same reasons
/// and with the same words.
#[test]
fn an_explicit_root_is_refused_for_every_reason_the_standard_one_is() {
    let mut linked = clean(FailureSchedule::intact())
        .planting("/elsewhere", Node::private_directory())
        .planting(NAMED, Node::private_directory().redirected());
    assert_eq!(
        LocalReceiptStoreV1::open_for_read(&named_roots(), &mut linked, StoreLimits::V1),
        Err(StoreOpenRefusal::PermissionRefused)
    );

    let mut file = clean(FailureSchedule::intact())
        .planting("/elsewhere", Node::private_directory())
        .planting(NAMED, Node::private_file(b"not a store"));
    assert_eq!(
        LocalReceiptStoreV1::open_for_read(&named_roots(), &mut file, StoreLimits::V1),
        Err(StoreOpenRefusal::NotADirectory)
    );
}

/// Read-only opening never creates the named folder.
///
/// `dorc why` reaches the store only through this path, and an admin-named folder is the easiest
/// place to get it wrong: the path is right there, and creating it would look like helpfulness.
#[test]
fn a_read_only_open_of_an_explicit_root_creates_nothing() {
    let mut io = clean(FailureSchedule::intact()).planting("/elsewhere", Node::private_directory());
    assert_eq!(
        LocalReceiptStoreV1::open_for_read(&named_roots(), &mut io, StoreLimits::V1),
        Err(StoreOpenRefusal::NotInitialized)
    );
    assert!(io.at(NAMED).is_none(), "asking created {:?}", io.paths());
}

/// A relative or empty folder is refused: the CLI edge resolves argv to an absolute path, so a
/// spelling that is absolute on neither family means that resolution did not happen.
#[test]
fn a_store_root_that_is_not_absolute_is_refused() {
    for folder in ["", "receipts", "./receipts", ".."] {
        assert!(
            roots().with_store_root(folder).is_err(),
            "`{folder}` would move with a process's working directory"
        );
    }
}

/// Naming a store never moves the KEY root (`30Rd`: no custom key root in V1).
#[test]
fn an_explicit_store_root_leaves_the_configuration_root_alone() {
    let named = named_roots();
    assert_eq!(
        named.base(RootRole::Configuration),
        roots().base(RootRole::Configuration),
        "custody stays where it was; only the store moved"
    );
    assert_eq!(named.base(RootRole::State), roots().base(RootRole::State));
}
