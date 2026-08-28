//! Every interruption of keyset initialization, and what a later process finds.
//!
//! Driven through the REAL state machine, never through a sequence written out here: a sequence
//! spelled twice agrees with itself while quietly disagreeing with the code, and the ordering
//! this file exists to prove is the code's.
//!
//! The shape of every case is the same. Interrupt one durable operation on one side, assert the
//! run actually REACHED that interruption, then restart from the disk it left and ask both entry
//! points what they find. Every restart has to land in exactly one closed state, and no restart
//! may generate, overwrite, or expose a capability the material on disk does not support.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy};
use dorc_receipt_local::io::{FailureSchedule, IoFault, LocalIo as _, Op, Side};
use dorc_receipt_local::keyset::{
    KeyAvailability, KeysetLocation, LocalReadOpenV1, LocalWriteOpenV1, PermissionSubject,
    StorePresence, open_for_read, open_or_initialize_for_write,
};
use dorc_receipt_local::manifest::{KeyRole, KeysetManifest};
use dorc_receipt_local::model::{ModelIo, Node, NodeKind};
use dorc_receipt_local::names::PRODUCT_DIR;
use dorc_receipt_local::store::DirectorySync;
use dorc_receipt_local::{LocalLimits, RootInputs, RootPlatform};

const CONFIG_BASE: &str = "/cfg";
const STATE_BASE: &str = "/state";
const PRODUCT: &str = "/cfg/dorc";
const KEYS: &str = "/cfg/dorc/receipt-keys-v1";
const KEYSET: &str = "/cfg/dorc/receipt-keys-v1/keyset-v1";
const SIGNING: &str = "/cfg/dorc/receipt-keys-v1/keyset-v1/signing-private-v1.pk8";
const ENCRYPTION: &str = "/cfg/dorc/receipt-keys-v1/keyset-v1/encryption-private-v1.age";
const MANIFEST: &str = "/cfg/dorc/receipt-keys-v1/keyset-v1/keyset-manifest-v1.txt";
const STORE: &str = "/state/dorc/receipts-v1";

/// The durable operations the initialization sequence itself performs.
///
/// Removal is NOT among them, and that is a ruling rather than an omission: V1 does not remove or
/// repair an incomplete keyset, because leaving one is safer than deleting an object whose
/// identity is uncertain. Enumerated here rather than filtered out silently, and checked against
/// the vocabulary's own classification below.
const INITIALIZATION_DURABLE_OPS: [Op; 5] = [
    Op::CreateDirectoryExclusive,
    Op::CreateFileExclusive,
    Op::WriteAll,
    Op::SyncFile,
    Op::SyncDirectory,
];

/// A source answering fixed bytes, so two runs of one case produce one signing identity.
struct FixedSecret(u8);

impl KeySecretEntropy for FixedSecret {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        raw.fill(self.0);
        true
    }
}

fn generator(seed: u8) -> EntropyKeysetGenerator<FixedSecret> {
    EntropyKeysetGenerator::over(FixedSecret(seed))
}

fn roots() -> RootInputs {
    RootInputs::of(RootPlatform::OtherUnix, CONFIG_BASE, STATE_BASE).expect("absolute bases")
}

/// A clean disk: the platform bases exist and nothing of this project's does.
fn clean(schedule: FailureSchedule) -> ModelIo {
    ModelIo::new(schedule, DirectorySync::Synchronized)
        .planting(CONFIG_BASE, Node::private_directory())
        .planting(STATE_BASE, Node::private_directory())
}

/// Run a write open over `io`, probing the store first exactly as a caller would.
fn write_open(io: &mut ModelIo, seed: u8) -> LocalWriteOpenV1 {
    let roots = roots();
    let store = StorePresence::probe(&roots, io, &LocalLimits::V1);
    let mut generator = generator(seed);
    open_or_initialize_for_write(&roots, io, &LocalLimits::V1, store, &mut generator)
}

fn read_open(io: &mut ModelIo) -> LocalReadOpenV1 {
    open_for_read(&roots(), io, &LocalLimits::V1)
}

fn refusal(outcome: LocalWriteOpenV1) -> KeyAvailability {
    match outcome {
        LocalWriteOpenV1::Ready(_) => panic!("the attempt was expected to refuse"),
        LocalWriteOpenV1::Refused(state) => state,
    }
}

fn is_ready(outcome: &LocalWriteOpenV1) -> bool {
    matches!(outcome, LocalWriteOpenV1::Ready(_))
}

#[test]
fn a_clean_profile_initializes_a_complete_keyset_and_reopens_it_unchanged() {
    let mut io = clean(FailureSchedule::intact());
    let first = write_open(&mut io, 1);
    assert!(is_ready(&first), "first use succeeds on a clean profile");

    for path in [PRODUCT, KEYS, KEYSET] {
        assert!(io.at(path).is_some_and(Node::is_directory), "{path}");
    }
    let signing = io.at(SIGNING).and_then(Node::bytes).map(<[u8]>::to_vec);
    let encryption = io.at(ENCRYPTION).and_then(Node::bytes).map(<[u8]>::to_vec);
    assert!(signing.is_some() && encryption.is_some());
    assert!(
        io.at(MANIFEST).is_some_and(Node::whole),
        "the manifest is whole"
    );
    assert!(
        io.at(KEYSET).is_some_and(Node::synced),
        "and the directory that makes it reachable is synchronized"
    );

    // A second process finds the same material and neither replaces nor regenerates it.
    let mut later = io.restart(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut later, 99)));
    assert_eq!(
        later.at(SIGNING).and_then(Node::bytes).map(<[u8]>::to_vec),
        signing,
        "a reopen with a DIFFERENT generator seed left the existing key untouched"
    );
    assert_eq!(
        later
            .at(ENCRYPTION)
            .and_then(Node::bytes)
            .map(<[u8]>::to_vec),
        encryption
    );
}

#[test]
fn the_removal_operation_is_not_part_of_initialization() {
    // Two-way against the vocabulary's own classification, so the sweep below cannot silently
    // stop covering an operation the sequence starts performing.
    let mut durable: Vec<Op> = Op::ALL.into_iter().filter(|op| op.is_durable()).collect();
    durable.retain(|op| !INITIALIZATION_DURABLE_OPS.contains(op));
    assert_eq!(
        durable,
        vec![Op::RemoveOwned],
        "the only durable operation initialization does not perform is removal"
    );

    let mut io = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut io, 2)));
    assert!(
        !io.schedule()
            .arrivals()
            .iter()
            .any(|(op, _)| *op == Op::RemoveOwned),
        "initialization removes nothing, so a failure cannot delete an object it does not own"
    );
}

#[test]
fn every_interruption_of_initialization_lands_in_exactly_one_closed_state() {
    // The sweep. For every durable operation the sequence performs, and for every occurrence of
    // it, interrupt before and after, then restart and ask what a later process finds.
    let mut failures: Vec<String> = Vec::new();
    let mut exercised: Vec<(Op, Side)> = Vec::new();
    for op in INITIALIZATION_DURABLE_OPS {
        for occurrence in 0..4 {
            for side in [Side::Before, Side::After] {
                let schedule =
                    FailureSchedule::faulting_occurrence(op, side, occurrence, IoFault::Platform);
                let mut io = clean(schedule);
                let outcome = write_open(&mut io, 3);

                let reached = io
                    .schedule()
                    .arrivals()
                    .iter()
                    .filter(|(seen, seen_side)| *seen == op && *seen_side == side)
                    .count();
                if reached <= occurrence {
                    // The sequence performs this operation fewer times than the case names, so
                    // this case interrupted nothing. Skipped rather than counted, and the
                    // occurrence-0 cases below assert the coverage that matters.
                    continue;
                }
                exercised.push((op, side));
                if is_ready(&outcome) {
                    failures.push(format!("{op:?}/{side:?}#{occurrence} completed anyway"));
                    continue;
                }

                // The manifest is the completion act, so it can never be the thing that exists
                // without the documents beside it — whatever was interrupted.
                if io.at(MANIFEST).is_some()
                    && (io.at(SIGNING).is_none() || io.at(ENCRYPTION).is_none())
                {
                    failures.push(format!("{op:?}/{side:?}#{occurrence} left a lone manifest"));
                }

                let mut restarted = io.restart(FailureSchedule::intact());
                let after = write_open(&mut restarted, 4);
                // The question is about the disk the RESTART left, not the one it found. A run
                // interrupted before anything durable happened leaves a clean profile, and the
                // restart is then an ordinary first use that legitimately completes.
                let closed = match &after {
                    LocalWriteOpenV1::Ready(_) => {
                        restarted.at(MANIFEST).is_some_and(Node::whole)
                            && restarted.at(SIGNING).is_some()
                            && restarted.at(ENCRYPTION).is_some()
                    }
                    LocalWriteOpenV1::Refused(state) => {
                        !state.exposes_write_capability() && !state.licenses_first_use_generation()
                    }
                };
                if !closed {
                    failures.push(format!(
                        "{op:?}/{side:?}#{occurrence} restarted into {after:?}"
                    ));
                }
                // Whatever happened, a restart that found key material never replaced it.
                if io.at(SIGNING).is_some()
                    && restarted.at(SIGNING).and_then(Node::bytes)
                        != io.at(SIGNING).and_then(Node::bytes)
                {
                    failures.push(format!("{op:?}/{side:?}#{occurrence} rewrote a key"));
                }
            }
        }
    }
    // The coverage floor. Cases naming an occurrence the sequence never reaches are skipped
    // above, so without this every one of them could be skipped and the sweep would pass having
    // interrupted nothing. Stated per operation and side rather than as a total, because a total
    // drifts while the property — every durable act of the sequence was interrupted from both
    // sides — does not.
    for op in INITIALIZATION_DURABLE_OPS {
        for side in [Side::Before, Side::After] {
            assert!(
                exercised.contains(&(op, side)),
                "the sweep never interrupted {op:?} on its {side:?} side"
            );
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn a_failure_before_the_keyset_directory_exists_leaves_no_keyset_path() {
    // The first assertion `30Rd` names. Generation happens before this act, so a run that stops
    // here has produced key material and committed none of it.
    let mut io = clean(FailureSchedule::faulting_occurrence(
        Op::CreateDirectoryExclusive,
        Side::Before,
        2,
        IoFault::Denied,
    ));
    assert_eq!(
        refusal(write_open(&mut io, 5)),
        KeyAvailability::PermissionRefused {
            subject: PermissionSubject::Directory
        }
    );
    assert!(io.at(KEYSET).is_none(), "no keyset path");
    assert!(io.at(SIGNING).is_none() && io.at(ENCRYPTION).is_none());

    let mut restarted = io.restart(FailureSchedule::intact());
    assert!(
        is_ready(&write_open(&mut restarted, 6)),
        "and a later run initializes cleanly, because nothing was left behind"
    );
}

#[test]
fn a_failure_after_the_directory_and_before_the_manifest_restarts_as_incomplete() {
    // The second assertion. Whatever partial material is there, the missing manifest means
    // nothing was ever licensed for publication, and V1 neither repairs it nor treats it as
    // first use.
    for (op, occurrence) in [
        (Op::CreateFileExclusive, 0),
        (Op::WriteAll, 0),
        (Op::SyncFile, 0),
        (Op::CreateFileExclusive, 1),
        (Op::WriteAll, 1),
        (Op::SyncFile, 1),
    ] {
        let mut io = clean(FailureSchedule::faulting_occurrence(
            op,
            Side::After,
            occurrence,
            IoFault::Platform,
        ));
        assert!(!is_ready(&write_open(&mut io, 7)), "{op:?}#{occurrence}");
        assert!(io.at(KEYSET).is_some(), "{op:?}#{occurrence}");
        assert!(io.at(MANIFEST).is_none(), "{op:?}#{occurrence}");

        let mut restarted = io.restart(FailureSchedule::intact());
        assert_eq!(
            refusal(write_open(&mut restarted, 8)),
            KeyAvailability::IncompleteOrInProgress,
            "{op:?}#{occurrence}"
        );
        match read_open(&mut restarted) {
            LocalReadOpenV1::Unavailable(KeyAvailability::IncompleteOrInProgress) => {}
            other => panic!("{op:?}#{occurrence} read as {other:?}"),
        }
    }
}

#[test]
fn no_incomplete_state_exposes_a_capability_and_no_restart_rewrites_a_key() {
    // The third and fourth assertions together, because they are one property: an interrupted
    // attempt neither hands out a signer nor lets the next attempt replace what it left.
    let mut io = clean(FailureSchedule::faulting_occurrence(
        Op::WriteAll,
        Side::After,
        0,
        IoFault::Platform,
    ));
    assert!(!is_ready(&write_open(&mut io, 9)));
    let left = io.at(SIGNING).and_then(Node::bytes).map(<[u8]>::to_vec);
    assert!(left.is_some(), "a partial document is on the disk");

    let mut restarted = io.restart(FailureSchedule::intact());
    assert_eq!(
        refusal(write_open(&mut restarted, 10)),
        KeyAvailability::IncompleteOrInProgress
    );
    assert_eq!(
        restarted
            .at(SIGNING)
            .and_then(Node::bytes)
            .map(<[u8]>::to_vec),
        left,
        "a run with a different generator did not overwrite what it found"
    );
    assert!(
        restarted.at(ENCRYPTION).is_none(),
        "and did not add the member the interrupted run never reached"
    );
}

#[test]
fn an_interruption_at_the_trailing_synchronization_leaves_a_keyset_that_looks_complete() {
    // The interruption a manifest cannot report. By the time the ancestry is synchronized the
    // manifest is already whole, so a write open that read completeness off its presence would
    // publish into a keyset whose directory entry may not survive. What makes it safe is that a
    // write open RE-SYNCHRONIZES, and only a synchronization that succeeds mints readiness.
    let mut io = clean(FailureSchedule::faulting_occurrence(
        Op::SyncDirectory,
        Side::Before,
        0,
        IoFault::Platform,
    ));
    assert_eq!(
        refusal(write_open(&mut io, 11)),
        KeyAvailability::TemporarilyUnavailable
    );
    assert!(
        io.at(MANIFEST).is_some_and(Node::whole),
        "the manifest IS whole — that is the whole difficulty"
    );

    // Read-only inspection works on what the filesystem presents.
    let mut restarted = io.restart(FailureSchedule::intact());
    match read_open(&mut restarted) {
        LocalReadOpenV1::Ready(keys) => {
            assert_eq!(keys.status(), &KeyAvailability::RichReadReady);
        }
        other @ LocalReadOpenV1::Unavailable(_) => {
            panic!("a complete-looking keyset did not read: {other:?}")
        }
    }

    // And a write open re-synchronizes before it will publish. The arrivals record is what says
    // the re-synchronization HAPPENED rather than that the answer was merely agreeable.
    let mut writer = io.restart(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut writer, 12)));
    let syncs = writer
        .schedule()
        .arrivals()
        .iter()
        .filter(|(op, side)| *op == Op::SyncDirectory && *side == Side::Before)
        .count();
    assert!(
        syncs >= 3,
        "a write open re-synchronizes the keyset directory and its ancestry; it reached {syncs}"
    );
    let file_syncs = writer
        .schedule()
        .arrivals()
        .iter()
        .filter(|(op, side)| *op == Op::SyncFile && *side == Side::Before)
        .count();
    assert!(
        file_syncs >= 3,
        "and both documents and the manifest; it reached {file_syncs}"
    );

    // The falsifying half: with that re-synchronization faulted, no write capability appears,
    // even though the material on disk is exactly the same as in the run above.
    let mut blocked = io.restart(FailureSchedule::faulting_occurrence(
        Op::SyncDirectory,
        Side::Before,
        0,
        IoFault::Platform,
    ));
    assert_eq!(
        refusal(write_open(&mut blocked, 13)),
        KeyAvailability::TemporarilyUnavailable,
        "identical bytes, and no readiness, because the synchronization is what was missing"
    );
}

#[test]
fn a_keyset_that_is_absent_beside_a_store_never_generates() {
    // The fifth assertion, over every shape of store that is not provably empty. Whole-keyset
    // loss with receipts still on disk would otherwise become an unannounced new key era, and
    // every one of those receipts would stop being readable without anyone being told.
    let occupied = [
        (
            "a recognized receipt",
            vec![(
                format!(
                    "{STORE}/plan-v1-00000000000000000001-{}.dorc-receipt",
                    "a".repeat(64)
                ),
                Node::private_file(b"whatever"),
            )],
        ),
        (
            "an unknown entry",
            vec![(format!("{STORE}/notes.txt"), Node::private_file(b"x"))],
        ),
        (
            "a nested directory",
            vec![(format!("{STORE}/old"), Node::private_directory())],
        ),
        (
            "a sync-client conflict name",
            vec![(
                format!("{STORE}/plan-v1.sync-conflict.dorc-receipt"),
                Node::private_file(b"x"),
            )],
        ),
    ];
    for (what, entries) in occupied {
        let mut io = clean(FailureSchedule::intact()).planting(STORE, Node::private_directory());
        for (path, node) in entries {
            io = io.planting(&path, node);
        }
        assert_eq!(
            refusal(write_open(&mut io, 14)),
            KeyAvailability::KeysetMissingWithExistingStore,
            "{what}"
        );
        assert!(io.at(KEYSET).is_none(), "{what}: nothing was created");
    }

    // A store directory that exists and is EMPTY does not block first use — the gate is the
    // presence of history, not the presence of a directory.
    let mut empty_store =
        clean(FailureSchedule::intact()).planting(STORE, Node::private_directory());
    assert!(is_ready(&write_open(&mut empty_store, 15)));

    // Nor does a store root that is a redirect get followed to decide the question: it cannot be
    // shown empty, so it refuses.
    let mut redirected =
        clean(FailureSchedule::intact()).planting(STORE, Node::private_directory().redirected());
    assert_eq!(
        refusal(write_open(&mut redirected, 16)),
        KeyAvailability::KeysetMissingWithExistingStore
    );
}

#[test]
fn a_manifest_disagreeing_with_a_key_refuses_and_never_regenerates() {
    // The eighth assertion. A manifest claiming an identity the document beside it does not
    // derive is a keyset that half works, and refusing is the only honest answer: the material
    // is intact, and which of the two is wrong is not knowable from here.
    let mut io = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut io, 17)));
    let real = io.at(MANIFEST).and_then(Node::bytes).map(<[u8]>::to_vec);
    let parsed = KeysetManifest::parse(&real.clone().expect("bytes"), &LocalLimits::V1)
        .expect("the manifest this run wrote parses");

    let forged = KeysetManifest::of(&"a".repeat(64), parsed.claimed(KeyRole::Encryption))
        .expect("a well-formed manifest naming another signing identity");
    let mut swapped = io
        .restart(FailureSchedule::intact())
        .planting(MANIFEST, Node::private_file(forged.serialize().as_bytes()));
    assert_eq!(
        refusal(write_open(&mut swapped, 18)),
        KeyAvailability::ManifestMismatch {
            role: KeyRole::Signing
        }
    );
    assert_eq!(
        swapped
            .at(SIGNING)
            .and_then(Node::bytes)
            .map(<[u8]>::to_vec),
        io.at(SIGNING).and_then(Node::bytes).map(<[u8]>::to_vec),
        "the key was neither replaced nor regenerated"
    );
}

#[test]
fn a_permissive_or_redirected_member_is_refused_before_its_secret_is_parsed() {
    // The ninth assertion, and the ordering inside it is the point: the handle is inspected
    // BEFORE anything is read, so a document anyone could read is refused rather than parsed and
    // then complained about.
    let mut base = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut base, 19)));
    let signing = base
        .at(SIGNING)
        .and_then(Node::bytes)
        .map(<[u8]>::to_vec)
        .expect("a document was written");

    for (what, node, subject) in [
        (
            "group or other can reach it",
            Node::of(
                NodeKind::File {
                    bytes: signing.clone(),
                    whole: true,
                },
                dorc_receipt_local::io::GroupAndOtherAccess::Present,
            ),
            PermissionSubject::KeyDocument {
                role: KeyRole::Signing,
            },
        ),
        (
            "it is reached through a redirect",
            Node::private_file(&signing).redirected(),
            PermissionSubject::KeyDocument {
                role: KeyRole::Signing,
            },
        ),
        (
            // The residual the owner comparison closes. `0700` plus a successful read is already
            // transitive proof of ownership for a non-root process on a mode-enforcing
            // filesystem, so this is the DAC-override case: a private document belonging to
            // somebody else, which the mode answer alone would admit.
            "it belongs to somebody else",
            Node::private_file(&signing).owned_by_another(),
            PermissionSubject::KeyDocument {
                role: KeyRole::Signing,
            },
        ),
    ] {
        let mut io = base
            .restart(FailureSchedule::intact())
            .planting(SIGNING, node);
        assert_eq!(
            refusal(write_open(&mut io, 20)),
            KeyAvailability::PermissionRefused { subject },
            "{what}"
        );
        let reads = io
            .schedule()
            .arrivals()
            .iter()
            .filter(|(op, side)| *op == Op::ReadBounded && *side == Side::Before)
            .count();
        assert_eq!(
            reads, 1,
            "{what}: only the manifest was read, so the private document's bytes were never \
             fetched at all"
        );
    }

    // A permissive keyset DIRECTORY is refused on the same rule, before any member is opened.
    let mut permissive = base.restart(FailureSchedule::intact()).planting(
        KEYSET,
        Node::of(
            NodeKind::Directory,
            dorc_receipt_local::io::GroupAndOtherAccess::Present,
        ),
    );
    assert_eq!(
        refusal(write_open(&mut permissive, 21)),
        KeyAvailability::PermissionRefused {
            subject: PermissionSubject::Directory
        }
    );
}

#[test]
fn a_concurrent_loser_discards_its_keys_and_never_mixes_them_into_the_winner() {
    // The tenth assertion. The keyset directory's exclusive creation is the arbitration point:
    // exactly one process makes it, and the one that does not walks away from what it generated.
    let mut winner = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut winner, 30)));
    let winning_signing = winner
        .at(SIGNING)
        .and_then(Node::bytes)
        .map(<[u8]>::to_vec)
        .expect("the winner wrote one");

    // A loser arriving at a keyset directory that exists but has no manifest yet.
    let mut mid_flight = clean(FailureSchedule::intact())
        .planting(KEYS, Node::private_directory())
        .planting(KEYSET, Node::private_directory());
    assert_eq!(
        refusal(write_open(&mut mid_flight, 31)),
        KeyAvailability::IncompleteOrInProgress,
        "no waiting, no deleting, no adding"
    );
    assert!(
        mid_flight.at(SIGNING).is_none() && mid_flight.at(ENCRYPTION).is_none(),
        "the loser wrote none of its own material into the winner's directory"
    );

    // And a loser arriving after the winner finished loads the WINNER's keyset, not its own.
    let mut late = winner.restart(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut late, 32)));
    assert_eq!(
        late.at(SIGNING).and_then(Node::bytes).map(<[u8]>::to_vec),
        Some(winning_signing),
        "the material on disk is the winner's, whatever the loser generated"
    );
}

#[test]
fn a_missing_encryption_document_verifies_and_never_publishes() {
    // Role-specific read availability, and the all-or-nothing write rule beside it. A receipt
    // whose opaque half cannot be opened is still one whose authorship can be checked; a keyset
    // whose opaque half is gone can publish nothing.
    let mut base = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut base, 40)));

    let without = base.restart(FailureSchedule::intact());
    let carried: Vec<(String, Node)> = without
        .paths()
        .into_iter()
        .filter(|path| *path != ENCRYPTION)
        .map(str::to_owned)
        .filter_map(|path| without.at(&path).cloned().map(|node| (path, node)))
        .collect();
    let mut rebuilt = ModelIo::new(FailureSchedule::intact(), DirectorySync::Synchronized);
    for (path, node) in carried {
        rebuilt = rebuilt.planting(&path, node);
    }

    match read_open(&mut rebuilt) {
        LocalReadOpenV1::Ready(keys) => {
            assert_eq!(keys.status(), &KeyAvailability::VerificationReady);
            assert!(
                keys.opener().is_none(),
                "nothing can open a region without the material that opens it"
            );
        }
        other @ LocalReadOpenV1::Unavailable(_) => {
            panic!("the signing half alone did not read: {other:?}")
        }
    }
    assert_eq!(
        refusal(write_open(&mut rebuilt, 41)),
        KeyAvailability::MissingAfterInitialization {
            role: KeyRole::Encryption
        },
        "and write readiness is all-or-nothing"
    );
    assert!(
        rebuilt.at(ENCRYPTION).is_none(),
        "the missing member was not replaced"
    );
}

#[test]
fn asking_why_never_writes_anything() {
    // `dorc why` calls the read entry point and nothing else, so the property is measurable:
    // over an intact keyset, a damaged one, and a wholly absent one, the read path performs no
    // durable operation at all.
    let mut complete = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut complete, 50)));

    let cases: Vec<ModelIo> = vec![
        complete.restart(FailureSchedule::intact()),
        complete
            .restart(FailureSchedule::intact())
            .planting(MANIFEST, Node::private_file(b"dorc-receipt-keyset/9\n")),
        clean(FailureSchedule::intact()),
    ];
    for mut io in cases {
        let before = io
            .paths()
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let _ = read_open(&mut io);
        assert!(
            !io.schedule()
                .arrivals()
                .iter()
                .any(|(op, _)| op.is_durable()),
            "the read path reached a durable operation: {:?}",
            io.schedule().arrivals()
        );
        assert_eq!(
            io.paths()
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            before,
            "and left the disk exactly as it found it"
        );
    }
}

#[test]
fn a_root_that_admits_no_fixed_component_is_refused_rather_than_worked_around() {
    // The location type is what enforces `30Rd`'s single-component rule, so a base that cannot
    // carry this project's components produces no location at all rather than a path assembled
    // some other way.
    let roots = roots();
    assert!(KeysetLocation::under(&roots).is_some());
    assert_eq!(
        roots.product_root(dorc_receipt_local::RootRole::Configuration),
        Some(dorc_receipt_local::names::LocalPath::of_root(
            RootPlatform::OtherUnix,
            CONFIG_BASE
        ))
        .and_then(|root| root.child(PRODUCT_DIR))
    );
    for hostile in ["..", ".", "", "a/b", "a\\b", "c:"] {
        assert!(
            dorc_receipt_local::names::LocalPath::of_root(RootPlatform::OtherUnix, CONFIG_BASE)
                .child(hostile)
                .is_none(),
            "{hostile:?} is not one ordinary component"
        );
    }
}

#[test]
fn a_file_standing_where_the_keyset_directory_belongs_is_not_an_incomplete_keyset() {
    // The state that would otherwise be read as somebody's work in progress, inviting a wait for
    // a writer that does not exist.
    let mut io = clean(FailureSchedule::intact())
        .planting(KEYS, Node::private_directory())
        .planting(KEYSET, Node::private_file(b"not a keyset"));
    assert_eq!(
        refusal(write_open(&mut io, 60)),
        KeyAvailability::UnexpectedObject {
            subject: PermissionSubject::Directory
        }
    );
}

#[test]
fn the_windows_baseline_is_accepted_and_is_not_the_unix_one() {
    // The platform posture, both ways. A Windows-shaped disk answers nothing about group and
    // other access and initializes anyway, under the explicitly weaker baseline; a Unix-shaped
    // open of the SAME material refuses, because there the answer is required and absent.
    let roots =
        RootInputs::of(RootPlatform::Windows, "C:\\Roaming", "C:\\Local").expect("absolute bases");
    let mut io = ModelIo::windows_shaped(FailureSchedule::intact())
        .planting("C:\\Roaming", Node::private_directory())
        .planting("C:\\Local", Node::private_directory());
    let store = StorePresence::probe(&roots, &mut io, &LocalLimits::V1);
    let mut generator = generator(70);
    let outcome =
        open_or_initialize_for_write(&roots, &mut io, &LocalLimits::V1, store, &mut generator);
    assert!(
        is_ready(&outcome),
        "the Windows baseline initializes: {outcome:?}"
    );
    assert_eq!(
        io.directory_sync(),
        DirectorySync::UnavailableOnPlatform,
        "and records the operation it does not have rather than simulating one"
    );

    // The other direction, stated where it actually lives — at the posture, not at the path
    // spelling. An object that answers NOTHING about group and other access is accepted under the
    // Windows baseline and refused under the Unix one, where that answer is required; and a Unix
    // object answering "none" is refused under the Windows baseline, because a proof claiming a
    // mode on a platform that has none is describing some other machine.
    let mut base = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut base, 71)));
    let signing_bytes = base
        .at(SIGNING)
        .and_then(Node::bytes)
        .map(<[u8]>::to_vec)
        .expect("a document was written");

    let mut unanswerable = base.restart(FailureSchedule::intact()).planting(
        SIGNING,
        Node::of(
            NodeKind::File {
                bytes: signing_bytes,
                whole: true,
            },
            dorc_receipt_local::io::GroupAndOtherAccess::NotInspectable,
        ),
    );
    match read_open(&mut unanswerable) {
        LocalReadOpenV1::Unavailable(KeyAvailability::PermissionRefused { .. }) => {}
        other => panic!("a Unix open of an unanswerable object gave {other:?}"),
    }
    assert!(
        matches!(
            read_open(&mut base.restart(FailureSchedule::intact())),
            LocalReadOpenV1::Ready(_)
        ),
        "the unedited keyset must read, or the refusal above proves nothing"
    );
}

#[test]
fn the_read_path_cannot_acquire_a_handle_that_could_synchronize() {
    // The platforms disagree about flushing a handle opened only for reading — one permits it and
    // one refuses — so the access a handle carries is declared at the open rather than discovered
    // at the synchronization. The read entry point declares the weaker one everywhere, which is
    // what stops `dorc why` from holding something that could write, and the model enforces the
    // stricter platform's rule so this is caught here rather than on one native leg.
    let mut base = clean(FailureSchedule::intact());
    assert!(is_ready(&write_open(&mut base, 80)));

    let mut reader = base.restart(FailureSchedule::intact());
    assert!(matches!(read_open(&mut reader), LocalReadOpenV1::Ready(_)));
    assert!(
        !reader
            .schedule()
            .arrivals()
            .iter()
            .any(|(op, _)| *op == Op::SyncFile),
        "the read path synchronized nothing"
    );

    // And the write open, which does have to synchronize, re-opens first: with the very same
    // material, faulting its re-open refuses rather than quietly flushing a read handle.
    let mut writer = base.restart(FailureSchedule::faulting_occurrence(
        Op::OpenExistingNoFollow,
        Side::Before,
        6,
        IoFault::Denied,
    ));
    let outcome = write_open(&mut writer, 81);
    assert!(
        !is_ready(&outcome),
        "a write open whose re-open was refused must not publish: {outcome:?}"
    );
}
