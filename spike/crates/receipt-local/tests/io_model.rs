//! The failure-schedule skeleton, driven over the modelled disk.
//!
//! This is the instrument the keyset and store sweeps will be built on, exercised on its own
//! before there is anything to sweep. What it has to get right is the property every later case
//! rests on: failing BEFORE a durable operation and failing AFTER it leave DIFFERENT disks, and a
//! later process restarting from either lands somewhere closed.
//!
//! It is deliberately not a filesystem, and it cannot answer what a native test answers: real
//! permissions, real links, real synchronization, real sharing. What it can do is enumerate every
//! interruption, which a native test cannot economically do.

use dorc_receipt_local::io::{FailureSchedule, IoFault, LocalIo, Op, Side};
use dorc_receipt_local::model::{ModelIo, Node};
use dorc_receipt_local::store::DirectorySync;

/// The keyset's own write order, as `30Rd` fixes it: the directory first, the two key documents
/// next, and the manifest LAST because its presence is what makes the keyset complete.
const KEYSET_SEQUENCE: &[(Op, &str)] = &[
    (Op::CreateDirectoryExclusive, "/cfg/receipt-keys-v1"),
    (
        Op::CreateDirectoryExclusive,
        "/cfg/receipt-keys-v1/keyset-v1",
    ),
    (
        Op::CreateFileExclusive,
        "/cfg/receipt-keys-v1/keyset-v1/signing-private-v1.pk8",
    ),
    (
        Op::WriteAll,
        "/cfg/receipt-keys-v1/keyset-v1/signing-private-v1.pk8",
    ),
    (
        Op::SyncFile,
        "/cfg/receipt-keys-v1/keyset-v1/signing-private-v1.pk8",
    ),
    (
        Op::CreateFileExclusive,
        "/cfg/receipt-keys-v1/keyset-v1/encryption-private-v1.age",
    ),
    (
        Op::WriteAll,
        "/cfg/receipt-keys-v1/keyset-v1/encryption-private-v1.age",
    ),
    (
        Op::SyncFile,
        "/cfg/receipt-keys-v1/keyset-v1/encryption-private-v1.age",
    ),
    (
        Op::CreateFileExclusive,
        "/cfg/receipt-keys-v1/keyset-v1/keyset-manifest-v1.txt",
    ),
    (
        Op::WriteAll,
        "/cfg/receipt-keys-v1/keyset-v1/keyset-manifest-v1.txt",
    ),
    (
        Op::SyncFile,
        "/cfg/receipt-keys-v1/keyset-v1/keyset-manifest-v1.txt",
    ),
    (Op::SyncDirectory, "/cfg/receipt-keys-v1/keyset-v1"),
];

/// The manifest's path — the completion marker every assertion below reads for.
const MANIFEST: &str = "/cfg/receipt-keys-v1/keyset-v1/keyset-manifest-v1.txt";

/// Drive the sequence until something faults, answering where it stopped.
fn drive(io: &mut ModelIo) -> Option<(Op, IoFault)> {
    for (op, path) in KEYSET_SEQUENCE {
        if let Err(fault) = io.perform(*op, path) {
            return Some((*op, fault));
        }
    }
    None
}

#[test]
fn an_uninterrupted_sequence_completes_with_the_manifest_last() {
    let mut io = ModelIo::new(FailureSchedule::intact(), DirectorySync::Synchronized);
    assert_eq!(drive(&mut io), None);
    assert_eq!(
        io.at(MANIFEST),
        Some(Node::File {
            whole: true,
            synced: true
        })
    );
}

/// Durable operations the initialization sequence does not perform.
///
/// Enumerated rather than merely skipped: `RemoveOwned` belongs to CLEANUP, which is a different
/// sequence with its own cases (one of them below), and a loop that silently passed over an
/// operation it could not reach would be a sweep with a hole in it.
const NOT_IN_THE_INITIALIZATION_SEQUENCE: [Op; 1] = [Op::RemoveOwned];

#[test]
fn every_interruption_before_the_completion_marker_leaves_no_complete_keyset() {
    // The sweep's shape, over the whole vocabulary rather than a chosen operation. For each
    // durable operation the sequence performs, on each side, interrupt there and RESTART from the
    // resulting disk. The manifest is written LAST, so a run stopped anywhere up to and including
    // its own synchronization cannot have left one — which is the whole reason the write order is
    // what it is.
    //
    // The trailing directory synchronization is the one exception, and it is a real state rather
    // than a gap in the rule: by then the manifest is already whole on disk. It gets its own case
    // below.
    let mut failures: Vec<String> = Vec::new();
    let mut interrupted = 0_usize;

    for op in Op::ALL {
        if !op.is_durable()
            || NOT_IN_THE_INITIALIZATION_SEQUENCE.contains(&op)
            || op == Op::SyncDirectory
        {
            continue;
        }
        for side in [Side::Before, Side::After] {
            let mut io = ModelIo::new(
                FailureSchedule::faulting(op, side, IoFault::Platform),
                DirectorySync::Synchronized,
            );
            let stopped = drive(&mut io);

            // A schedule naming an operation the sequence never performs would interrupt nothing
            // and pass. Measure that it actually bit.
            let reached = io.schedule().arrivals().iter().any(|(seen, _)| *seen == op);
            if !reached {
                failures.push(format!("{op:?}/{side:?}: the sequence never reached it"));
                continue;
            }
            if stopped.is_none() {
                failures.push(format!(
                    "{op:?}/{side:?}: the schedule did not stop the run"
                ));
                continue;
            }
            interrupted = interrupted.saturating_add(1);

            // Restart: a later process sees only the disk.
            let restarted = io.restart(FailureSchedule::intact());
            if restarted.at(MANIFEST)
                == Some(Node::File {
                    whole: true,
                    synced: true,
                })
            {
                failures.push(format!(
                    "{op:?}/{side:?}: an interrupted run left a COMPLETE keyset"
                ));
            }
        }
    }

    assert!(interrupted > 0, "no interruption was exercised at all");
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn an_interrupted_ancestry_synchronization_leaves_a_manifest_a_later_writer_must_re_synchronize() {
    // The one interruption that leaves a keyset LOOKING complete, and the reason a write open
    // cannot be a file-existence check. By the time the ancestry is synchronized the manifest is
    // already whole, so a later process reading the disk finds every member there — and what it
    // does NOT have is any evidence that the directory entry making them reachable survived.
    //
    // `30Rd` answers it by requiring a later WRITE open to validate and successfully
    // re-synchronize the whole keyset before exposing a signing capability, while a later READ
    // open may inspect what the filesystem presents. This case pins the disk that requirement is
    // about; a run that treated the manifest's presence as sufficient would be reading it wrong.
    for side in [Side::Before, Side::After] {
        let mut io = ModelIo::new(
            FailureSchedule::faulting(Op::SyncDirectory, side, IoFault::Platform),
            DirectorySync::Synchronized,
        );
        assert_eq!(
            drive(&mut io),
            Some((Op::SyncDirectory, IoFault::Platform)),
            "the run stopped at the ancestry synchronization"
        );
        let restarted = io.restart(FailureSchedule::intact());
        assert_eq!(
            restarted.at(MANIFEST),
            Some(Node::File {
                whole: true,
                synced: true
            }),
            "the completion marker is on disk"
        );
    }
}

#[test]
fn failing_before_a_create_leaves_no_object_and_failing_after_it_leaves_one() {
    // The distinction the whole schedule exists for, at the one operation where it is most
    // visible. If these two produced the same disk there would be no reason for the schedule to
    // have two sides, and every "what does a later process find" assertion would be vacuous.
    let path = "/cfg/receipt-keys-v1";

    let mut before = ModelIo::new(
        FailureSchedule::faulting(Op::CreateDirectoryExclusive, Side::Before, IoFault::Denied),
        DirectorySync::Synchronized,
    );
    assert!(drive(&mut before).is_some());
    assert_eq!(before.at(path), None, "nothing was created");

    let mut after = ModelIo::new(
        FailureSchedule::faulting(Op::CreateDirectoryExclusive, Side::After, IoFault::Denied),
        DirectorySync::Synchronized,
    );
    assert!(drive(&mut after).is_some());
    assert_eq!(
        after.at(path),
        Some(Node::Directory),
        "the act happened and the caller was told it did not"
    );
}

#[test]
fn a_restart_over_an_existing_keyset_directory_never_replaces_what_is_there() {
    // The rule an incomplete keyset rests on: exclusive creation, and no repair. A second run
    // over a disk carrying a half-written keyset is refused at the directory rather than
    // continuing into it, which is what stops a resumed attempt mixing its own key with the
    // winner's.
    let mut first = ModelIo::new(
        FailureSchedule::faulting(Op::CreateFileExclusive, Side::After, IoFault::Platform),
        DirectorySync::Synchronized,
    );
    assert!(drive(&mut first).is_some());
    assert!(first.at(MANIFEST).is_none(), "no completion marker");

    let mut second = first.restart(FailureSchedule::intact());
    assert_eq!(
        second.perform(
            Op::CreateDirectoryExclusive,
            "/cfg/receipt-keys-v1/keyset-v1"
        ),
        Err(IoFault::AlreadyExists),
        "a second attempt does not win a directory that is already there"
    );
    assert!(
        second.at(MANIFEST).is_none(),
        "and it did not complete somebody else's keyset"
    );
}

#[test]
fn a_platform_without_directory_synchronization_does_not_fail_the_operation() {
    // The property `PublicationProperties` records rather than grades: Windows has no such call,
    // so the sequence does not stall there — it completes, and the proof says the operation was
    // unavailable rather than that it succeeded.
    let mut io = ModelIo::new(
        FailureSchedule::intact(),
        DirectorySync::UnavailableOnPlatform,
    );
    assert_eq!(drive(&mut io), None);
    assert_eq!(io.directory_sync(), DirectorySync::UnavailableOnPlatform);
}

#[test]
fn a_fault_while_handling_a_fault_is_expressible() {
    // The compound shape: an act fails, and the cleanup that answers it fails too. A schedule
    // that could only interrupt one operation per run could not describe it, and that is the
    // interleaving a real filesystem produces under a full disk.
    let mut io = ModelIo::new(
        FailureSchedule::faulting(Op::WriteAll, Side::After, IoFault::Partial),
        DirectorySync::Synchronized,
    );
    assert_eq!(
        drive(&mut io),
        Some((Op::WriteAll, IoFault::Partial)),
        "the write reported partial"
    );
    // The cleanup this attempt would attempt — and a second schedule faults it too.
    let mut cleanup = io.restart(FailureSchedule::faulting(
        Op::RemoveOwned,
        Side::Before,
        IoFault::Denied,
    ));
    assert_eq!(
        cleanup.perform(
            Op::RemoveOwned,
            "/cfg/receipt-keys-v1/keyset-v1/signing-private-v1.pk8"
        ),
        Err(IoFault::Denied)
    );
    assert!(
        cleanup
            .at("/cfg/receipt-keys-v1/keyset-v1/signing-private-v1.pk8")
            .is_some(),
        "a cleanup that failed removed nothing, and leaving the object is the safe answer"
    );
}
