//! The failure schedule's own mechanics, exercised on the modelled disk.
//!
//! What this file asserts is the instrument rather than the subject: that failing BEFORE a
//! durable operation and failing AFTER it leave DIFFERENT disks, that a schedule interrupts the
//! occurrence it named and no other, and that the arrivals record can tell a case which
//! operations it actually reached. The keyset's own interruptions are swept in `keyset_sweep.rs`,
//! driven by the real state machine rather than by a sequence written out here — a sequence
//! spelled twice is one that agrees with itself while disagreeing with the code.
//!
//! It is deliberately not a filesystem, and it cannot answer what a native test answers: real
//! permissions, real links, real synchronization, real sharing. What it can do is enumerate every
//! interruption, which a native test cannot economically do.

use dorc_receipt_local::io::{
    Answer, FailureSchedule, GroupAndOtherAccess, IoFault, LocalIo, ObjectKind, Op, Request, Side,
};
use dorc_receipt_local::model::{ModelIo, Node, NodeKind};
use dorc_receipt_local::store::DirectorySync;

const DIR: &str = "/cfg/keyset-v1";
const FILE: &str = "/cfg/keyset-v1/signing-private-v1.pk8";

fn empty(schedule: FailureSchedule) -> ModelIo {
    ModelIo::new(schedule, DirectorySync::Synchronized)
}

#[test]
fn failing_before_a_create_leaves_nothing_and_failing_after_it_leaves_an_object() {
    // The distinction the whole schedule exists for, measured on the disk rather than on the
    // return value: both attempts are told the act failed, and only one of them is telling the
    // truth about what a later process will find.
    let mut before = empty(FailureSchedule::faulting(
        Op::CreateDirectoryExclusive,
        Side::Before,
        IoFault::Denied,
    ));
    assert_eq!(
        before.perform(Request::CreateDirectoryExclusive, DIR),
        Err(IoFault::Denied)
    );
    assert!(before.at(DIR).is_none(), "nothing was created");

    let mut after = empty(FailureSchedule::faulting(
        Op::CreateDirectoryExclusive,
        Side::After,
        IoFault::Denied,
    ));
    assert_eq!(
        after.perform(Request::CreateDirectoryExclusive, DIR),
        Err(IoFault::Denied)
    );
    assert!(
        after.at(DIR).is_some(),
        "the object survives an attempt that was told it failed"
    );
}

#[test]
fn a_schedule_interrupts_the_occurrence_it_named_and_no_other() {
    // Directory synchronization is reached three times in one keyset sequence, so a schedule that
    // could only name the first would leave the trailing one — the interesting one — untested.
    let mut io = empty(FailureSchedule::faulting_occurrence(
        Op::SyncDirectory,
        Side::After,
        2,
        IoFault::Platform,
    ));
    assert_eq!(
        io.perform(Request::CreateDirectoryExclusive, DIR),
        Ok(Answer::Done)
    );
    assert_eq!(io.perform(Request::SyncDirectory, DIR), Ok(Answer::Done));
    assert_eq!(io.perform(Request::SyncDirectory, DIR), Ok(Answer::Done));
    assert_eq!(
        io.perform(Request::SyncDirectory, DIR),
        Err(IoFault::Platform),
        "the third arrival is the one the schedule named"
    );
}

#[test]
fn a_write_leaves_the_bytes_it_was_given_and_a_read_bound_refuses_rather_than_truncating() {
    let mut io = empty(FailureSchedule::intact());
    assert_eq!(
        io.perform(Request::CreateDirectoryExclusive, DIR),
        Ok(Answer::Done)
    );
    assert_eq!(
        io.perform(Request::CreateFileExclusive, FILE),
        Ok(Answer::Done)
    );
    assert_eq!(
        io.perform(Request::WriteAll { bytes: b"abcdef" }, FILE),
        Ok(Answer::Done)
    );
    assert_eq!(
        io.at(FILE).and_then(Node::bytes),
        Some(b"abcdef".as_slice())
    );

    assert_eq!(
        io.perform(Request::ReadBounded { limit: 6 }, FILE),
        Ok(Answer::Bytes(b"abcdef".to_vec())),
        "at the bound"
    );
    assert_eq!(
        io.perform(Request::ReadBounded { limit: 5 }, FILE),
        Err(IoFault::OverBound),
        "one byte over the bound is refused, never handed back short"
    );
}

#[test]
fn a_created_object_reports_that_this_attempt_owns_it_and_a_restart_no_longer_does() {
    // The honest half of the ownership question. What a process can show is that IT made the
    // object; a later process finds the same bytes and has shown nothing about who owns them,
    // and the fact carries that difference rather than flattening it into a boolean.
    let mut io = empty(FailureSchedule::intact());
    assert_eq!(
        io.perform(Request::CreateDirectoryExclusive, DIR),
        Ok(Answer::Done)
    );
    let Ok(Answer::Facts(fresh)) = io.perform(Request::InspectOpened, DIR) else {
        panic!("a created directory inspects");
    };
    assert!(fresh.ownership_established());
    assert_eq!(fresh.kind(), ObjectKind::Directory);
    assert_eq!(fresh.group_and_other(), GroupAndOtherAccess::None);

    let mut later = io.restart(FailureSchedule::intact());
    let Ok(Answer::Facts(found)) = later.perform(Request::InspectOpened, DIR) else {
        panic!("the same directory inspects after a restart");
    };
    assert!(
        !found.ownership_established(),
        "a restart genuinely loses the one thing that established ownership"
    );
}

#[test]
fn a_planted_redirect_is_refused_without_being_followed() {
    let mut io =
        empty(FailureSchedule::intact()).planting(DIR, Node::private_directory().redirected());
    assert_eq!(
        io.perform(Request::OpenExistingNoFollow, DIR),
        Err(IoFault::Redirect)
    );
}

#[test]
fn a_platform_without_directory_synchronization_has_not_failed_it() {
    // Two negative answers that must never collapse: this one is a platform limit the proof
    // records, and a real failure is a refusal that stops a publication.
    let mut windows = ModelIo::windows_shaped(FailureSchedule::intact());
    assert_eq!(
        windows.directory_sync(),
        DirectorySync::UnavailableOnPlatform
    );
    assert_eq!(
        windows.perform(Request::SyncDirectory, "/nowhere"),
        Ok(Answer::Done),
        "an operation the platform does not have cannot fail on a path it never looked at"
    );

    let mut unix = empty(FailureSchedule::intact());
    assert_eq!(
        unix.perform(Request::SyncDirectory, "/nowhere"),
        Err(IoFault::NotFound),
        "and where the platform does have it, the path matters"
    );
}

#[test]
fn an_enumeration_observes_its_own_overflow_rather_than_going_quiet_at_the_bound() {
    let mut io = empty(FailureSchedule::intact()).planting(DIR, Node::private_directory());
    for name in ["a", "b", "c"] {
        assert_eq!(
            io.perform(Request::CreateFileExclusive, &format!("{DIR}/{name}")),
            Ok(Answer::Done)
        );
    }
    let Ok(Answer::Entries(within)) = io.perform(Request::EnumerateBounded { limit: 3 }, DIR)
    else {
        panic!("the walk answers");
    };
    assert_eq!(within.names(), ["a", "b", "c"]);
    assert!(!within.over_bound());

    let Ok(Answer::Entries(over)) = io.perform(Request::EnumerateBounded { limit: 2 }, DIR) else {
        panic!("the walk answers");
    };
    assert!(
        over.over_bound(),
        "the walk goes to the bound plus one, so overflow is something it SAW"
    );
    assert!(!over.is_empty());
}

#[test]
fn a_wrong_kind_object_is_not_a_missing_one() {
    let mut io = empty(FailureSchedule::intact())
        .planting(FILE, Node::of(NodeKind::Other, GroupAndOtherAccess::None));
    let Ok(Answer::Facts(facts)) = io.perform(Request::InspectOpened, FILE) else {
        panic!("it inspects");
    };
    assert_eq!(facts.kind(), ObjectKind::Other);
    assert_eq!(
        io.perform(Request::ReadBounded { limit: 16 }, FILE),
        Err(IoFault::WrongKind)
    );
}

#[test]
fn every_operation_is_classified_and_the_durable_ones_are_the_ones_that_change_a_disk() {
    // Two-way over the classification, because the sweep is built from it: an operation wrongly
    // called non-durable gets one schedule instead of two, and the interruption that leaves a
    // half-written keyset is the one that goes untested.
    let durable: Vec<Op> = Op::ALL.into_iter().filter(|op| op.is_durable()).collect();
    assert_eq!(durable.len(), 6, "{durable:?}");
    for op in [
        Op::CreateDirectoryExclusive,
        Op::CreateFileExclusive,
        Op::WriteAll,
        Op::SyncFile,
        Op::SyncDirectory,
        Op::RemoveOwned,
    ] {
        assert!(op.is_durable(), "{op:?}");
    }
    for op in [
        Op::ReadBounded,
        Op::InspectOpened,
        Op::EnumerateBounded,
        Op::OpenExistingNoFollow,
    ] {
        assert!(!op.is_durable(), "{op:?}");
    }
}
