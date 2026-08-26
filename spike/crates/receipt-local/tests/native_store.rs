//! The receipt store against a real filesystem, on whichever platform is running.
//!
//! What this answers and the model cannot: real exclusive creation, real modes, real links, real
//! synchronization, and what a genuinely separate attempt finds. What it cannot answer and the
//! model can: every interruption, which is swept in `store_sweep.rs`.
//!
//! Neither layer substitutes for the other, so both exist.
//!
//! The sandbox is a uniquely named directory under the platform's own temporary location. On a
//! Unix leg that is a real Unix filesystem, which is what the mode assertions below require — a
//! Windows-mounted filesystem reached from a Unix process reports modes that are not the ones the
//! kernel is enforcing, so asserting against it would be asserting against a translation layer.
//! That premise is a loud assertion rather than an assumption.

// Only `expect_used`: every `panic!` here sits inside a `#[test]` function, which the central
// `allow-panic-in-tests` key does reach, so expecting that lint too would be an expectation
// nothing fulfils.
#![expect(
    clippy::expect_used,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use std::sync::atomic::{AtomicU32, Ordering};

use dorc_receipt::capability::ReceiptSigner as _;
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::ids::{PlanReceiptId, ReceiptId, ReceiptIdSource};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{Plain, PlanReceipt};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::writer::DraftReceipt;
use dorc_receipt_crypto::Ed25519Signer;
use dorc_receipt_local::store::{
    DirectorySync, EntryStanding, EnumerateFailure, LocalReceiptStoreV1, PublishFailure,
    PublishRefusal, StoreLimits, StoreOpenRefusal,
};
use dorc_receipt_local::{LocalLimits, NativeIo, RootInputs, RootPlatform};

/// Distinguishes concurrent cases inside one process; the process id distinguishes the runs.
static NEXT: AtomicU32 = AtomicU32::new(0);

const FIXTURE_SECRET: [u8; 32] = [13_u8; 32];

/// A sandbox that removes itself.
struct Sandbox {
    root: std::path::PathBuf,
}

impl Sandbox {
    fn new(name: &str) -> Self {
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "dorc-store-{name}-{}-{ordinal}",
            std::process::id()
        ));
        // A leftover from a killed run is removed rather than reused: the cases below assert what
        // a CLEAN profile does, and a stale directory would quietly make them assert something
        // else.
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("cfg")).expect("a sandbox configuration base");
        std::fs::create_dir_all(root.join("state")).expect("a sandbox state base");
        Self { root }
    }

    fn base(&self, which: &str) -> String {
        self.root.join(which).to_string_lossy().into_owned()
    }

    fn roots(&self) -> RootInputs {
        RootInputs::of(platform(), &self.base("cfg"), &self.base("state"))
            .expect("the sandbox bases are absolute")
    }

    fn store_dir(&self) -> std::path::PathBuf {
        self.root.join("state").join("dorc").join("receipts-v1")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

const fn platform() -> RootPlatform {
    if cfg!(windows) {
        RootPlatform::Windows
    } else {
        RootPlatform::OtherUnix
    }
}

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

fn document(id: PlanReceiptId, order: ReceiptOrderToken) -> DraftReceipt<PlanReceipt, Plain> {
    let key = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let row = SkeletonRecord::build(
        RecordKind::ProjectionOmission,
        ["observation", "0", "unminted", "authored-before-contact"]
            .iter()
            .map(|atom| (*atom).to_owned())
            .collect(),
    )
    .expect("a row the grammar admits");
    DraftReceipt::of(Skeleton {
        receipt_id: id.hex(),
        order,
        signing_key_id: key.signing_key_id().hex(),
        encryption_key_id: None,
        records: vec![row],
    })
}

fn signed_bytes(id: PlanReceiptId, order: ReceiptOrderToken) -> Vec<u8> {
    document(id, order)
        .serialize()
        .expect("a plain draft serializes")
        .sign(&Ed25519Signer::of_secret(FIXTURE_SECRET))
        .into_bytes()
}

/// One publication against the real filesystem, through a FRESH `NativeIo` — which is what makes
/// each call a separate attempt, owning only what it itself creates.
fn publish(sandbox: &Sandbox, seed: u8, millis: u64) -> Result<String, PublishRefusal> {
    let roots = sandbox.roots();
    let mut io = NativeIo::new();
    let store = LocalReceiptStoreV1::open_or_create(&roots, &mut io, StoreLimits::V1)
        .expect("the store opens");
    let id = plan_id(seed);
    let at = ReceiptOrderToken::of_controller_millis(millis);
    let policy = store.required_policy();
    let signed = document(id, at)
        .serialize()
        .expect("a plain draft serializes")
        .sign(&Ed25519Signer::of_secret(FIXTURE_SECRET));
    store
        .publish_required_v1::<PlanReceipt, Plain>(&mut io, at, id, signed, policy)
        .map(|proof| proof.file_name().spelled())
}

fn order(millis: u64) -> ReceiptOrderToken {
    ReceiptOrderToken::of_controller_millis(millis)
}

#[test]
fn a_clean_profile_publishes_on_this_platform_and_a_separate_attempt_reads_it_back() {
    // D3's exit, on whichever platform is running: the sequence completes against a real
    // filesystem, and a genuinely separate attempt — a fresh `NativeIo`, owning nothing —
    // enumerates and reads the exact bytes.
    let sandbox = Sandbox::new("publish");
    let id = plan_id(1);
    let at = order(1_700_000_000_000);
    let expected = signed_bytes(id, at);

    let name = publish(&sandbox, 1, 1_700_000_000_000).expect("the publication succeeds");
    let path = sandbox.store_dir().join(&name);
    assert!(path.is_file(), "{} was not written", path.display());
    assert_eq!(
        std::fs::read(&path).expect("it is readable"),
        expected,
        "the bytes on disk are the exact signed document"
    );

    let mut io = NativeIo::new();
    let store = LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, StoreLimits::V1)
        .expect("a later attempt opens the store");
    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 1);
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(entry.order(), at);
    let read = store.read(&mut io, entry).expect("it reads back");
    assert_eq!(read.standing(), EntryStanding::CompleteBytes);
    assert_eq!(read.byte_length(), expected.len());
}

#[test]
fn a_second_publication_under_one_name_is_refused_by_the_platform_itself() {
    // Exclusive creation IS the atomicity here, so this measures the platform's answer rather
    // than a check this crate performs before asking.
    let sandbox = Sandbox::new("collision");
    let name = publish(&sandbox, 1, 5_000).expect("the first succeeds");
    let before = std::fs::read(sandbox.store_dir().join(&name)).expect("it is readable");

    match publish(&sandbox, 1, 5_000) {
        Err(refusal) => {
            assert_eq!(refusal.reason(), PublishFailure::NameAlreadyTaken);
            assert!(refusal.into_incomplete().is_none());
        }
        Ok(second) => panic!("a taken name published again as {second}"),
    }
    assert_eq!(
        std::fs::read(sandbox.store_dir().join(&name)).expect("still readable"),
        before,
        "the first document is byte-identical after the refusal"
    );
}

#[test]
fn asking_to_read_a_clean_profile_creates_nothing() {
    // `dorc why` reaches the store only through the read-only open. A missing store is a report
    // state, not a directory to bring into being so an answer can be read out of it.
    let sandbox = Sandbox::new("readclean");
    let mut io = NativeIo::new();
    match LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, StoreLimits::V1) {
        Err(StoreOpenRefusal::NotInitialized) => {}
        other => panic!("a clean profile read as {other:?}"),
    }
    let entries: Vec<_> = std::fs::read_dir(sandbox.root.join("state"))
        .expect("the base exists")
        .flatten()
        .collect();
    assert!(
        entries.is_empty(),
        "asking created {:?}",
        entries
            .iter()
            .map(std::fs::DirEntry::path)
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_walk_past_a_narrowed_entry_bound_refuses_on_a_real_directory() {
    // The bound is lowered rather than filling a directory with four thousand files. What this
    // adds over the modelled case is that a REAL walk observes its own overflow: the production
    // implementation stops at the bound plus one rather than reading the whole directory.
    let sandbox = Sandbox::new("overflow");
    for (seed, millis) in [(1_u8, 10_u64), (2, 20), (3, 30)] {
        publish(&sandbox, seed, millis).expect("a publication");
    }
    let narrow = StoreLimits {
        receipt: ReceiptLimits::V1,
        local: LocalLimits {
            store_entries: 2,
            ..LocalLimits::V1
        },
    };
    let mut io = NativeIo::new();
    let store = LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, narrow)
        .expect("the store is there");
    assert_eq!(
        store.enumerate(&mut io),
        Err(EnumerateFailure::OverEntryBound)
    );

    let mut io = NativeIo::new();
    let store = LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, StoreLimits::V1)
        .expect("the store is there");
    assert_eq!(
        store.enumerate(&mut io).expect("the walk answers").walked(),
        3,
        "and the ordinary bound sees all three"
    );
}

#[test]
fn two_attempts_racing_one_name_produce_one_document_and_one_refusal() {
    // Serialized here rather than threaded, because what is being measured is the platform's
    // exclusive create — the same act either way — and a threaded race would make the assertion
    // about scheduling instead.
    let sandbox = Sandbox::new("race");
    let first = publish(&sandbox, 7, 1_234).expect("one attempt wins");
    let second = publish(&sandbox, 7, 1_234);
    assert!(second.is_err(), "the other attempt did not lose");

    let mut io = NativeIo::new();
    let store = LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, StoreLimits::V1)
        .expect("the store is there");
    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(walk.recognized().len(), 1);
    assert_eq!(
        walk.recognized()
            .first()
            .expect("one entry")
            .name()
            .spelled(),
        first
    );
}

#[cfg(unix)]
#[test]
fn every_object_the_store_creates_is_reachable_only_by_its_owner() {
    // The Unix half of the platform guarantee, on a real Unix filesystem. The mode rides the same
    // call that makes each object visible, so there is no window in which a receipt exists group-
    // or other-readable.
    use std::os::unix::fs::PermissionsExt as _;

    let sandbox = Sandbox::new("modes");
    // The premise, measured rather than assumed. A Windows-mounted filesystem reached from a Unix
    // process reports modes the kernel is not enforcing, so a sandbox that landed on one would
    // make every assertion below pass while proving nothing.
    assert!(
        !sandbox.root.starts_with("/mnt/"),
        "the sandbox landed at {}, which is not a real Unix filesystem",
        sandbox.root.display()
    );
    let name = publish(&sandbox, 1, 42).expect("a publication");

    let mode = |path: &std::path::Path| {
        std::fs::metadata(path)
            .expect("it exists")
            .permissions()
            .mode()
            & 0o777
    };
    for directory in [sandbox.root.join("state").join("dorc"), sandbox.store_dir()] {
        assert_eq!(mode(&directory), 0o700, "{}", directory.display());
    }
    assert_eq!(mode(&sandbox.store_dir().join(&name)), 0o600);
}

#[cfg(unix)]
#[test]
fn a_store_root_anyone_may_write_is_refused_and_narrowing_it_back_opens() {
    // The store's own permission rule against a real mode. The positive control matters: without
    // it the refusal would be satisfied by anything at all having gone wrong with this sandbox.
    use std::os::unix::fs::PermissionsExt as _;

    let sandbox = Sandbox::new("writable");
    assert!(!sandbox.root.starts_with("/mnt/"), "a real Unix filesystem");
    publish(&sandbox, 1, 1).expect("a publication");

    std::fs::set_permissions(sandbox.store_dir(), std::fs::Permissions::from_mode(0o777))
        .expect("the mode is ours to widen");
    let mut io = NativeIo::new();
    match LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, StoreLimits::V1) {
        Err(StoreOpenRefusal::PermissionRefused) => {}
        other => panic!("a world-writable store root answered {other:?}"),
    }

    // Group- and other-READABLE is not refused: receipts are created owner-only, and a readable
    // containing directory is not a place another account can plant entries.
    std::fs::set_permissions(sandbox.store_dir(), std::fs::Permissions::from_mode(0o755))
        .expect("the mode is ours to narrow");
    let mut io = NativeIo::new();
    LocalReceiptStoreV1::open_for_read(&sandbox.roots(), &mut io, StoreLimits::V1)
        .expect("a readable-but-not-writable store root opens");
}

#[cfg(unix)]
#[test]
fn a_receipt_name_occupied_by_a_link_is_refused_without_being_written_through() {
    // Exclusive creation refuses an existing name, a DANGLING link included — which is what
    // stops a planted link from redirecting a publication out of the store.
    let sandbox = Sandbox::new("linkedname");
    let name = publish(&sandbox, 1, 100).expect("a first publication mints a real name");
    let second_id = plan_id(2);
    let second_at = order(200);
    let second_name = name
        .replace(&plan_id(1).hex(), &second_id.hex())
        .replace(&order(100).spelled(), &second_at.spelled());

    let elsewhere = sandbox.root.join("elsewhere");
    std::fs::write(&elsewhere, b"someone else's file").expect("a real file");
    std::os::unix::fs::symlink(&elsewhere, sandbox.store_dir().join(&second_name))
        .expect("a link at the name a publication would take");

    match publish(&sandbox, 2, 200) {
        Err(refusal) => assert_eq!(refusal.reason(), PublishFailure::NameAlreadyTaken),
        Ok(placed) => panic!("a linked name published as {placed}"),
    }
    assert_eq!(
        std::fs::read(&elsewhere).expect("it exists"),
        b"someone else's file",
        "the link was followed and written through"
    );
}

#[cfg(unix)]
#[test]
fn a_store_entry_that_is_a_link_is_not_followed_on_read() {
    // The read half of the same rule: a recognized NAME whose object is a redirect is refused
    // rather than opened, so nothing outside the store can be read as a receipt.
    let sandbox = Sandbox::new("linkedentry");
    let elsewhere = sandbox.root.join("elsewhere");
    std::fs::write(&elsewhere, b"not a receipt").expect("a real file");

    let mut io = NativeIo::new();
    let store = LocalReceiptStoreV1::open_or_create(&sandbox.roots(), &mut io, StoreLimits::V1)
        .expect("the store opens");
    let id = plan_id(3);
    let at = order(300);
    let name = dorc_receipt_local::names::ReceiptFileName::of(
        dorc_receipt_local::names::NamedSpecies::Plan,
        at,
        &id.hex(),
    )
    .expect("a mintable name")
    .spelled();
    std::os::unix::fs::symlink(&elsewhere, sandbox.store_dir().join(&name)).expect("a link");

    let walk = store.enumerate(&mut io).expect("the walk answers");
    assert_eq!(
        walk.recognized().len(),
        1,
        "the NAME is recognized; the object behind it is a separate question"
    );
    let entry = walk.recognized().first().expect("one entry");
    assert_eq!(
        store.read(&mut io, entry),
        Err(dorc_receipt_local::store::StoreReadFailure::NotARegularFile)
    );
}

#[cfg(windows)]
#[test]
fn the_windows_baseline_publishes_and_reports_the_operation_it_does_not_have() {
    // The Windows half. There is no mode to assert, so what is measured is that the publication
    // completes under the inherited per-user access and that the platform's missing directory
    // synchronization is RECORDED rather than simulated as a success of a stronger kind.
    use dorc_receipt_local::io::LocalIo as _;

    let sandbox = Sandbox::new("windows");
    let roots = sandbox.roots();
    let mut io = NativeIo::new();
    let store =
        LocalReceiptStoreV1::open_or_create(&roots, &mut io, StoreLimits::V1).expect("a store");
    let id = plan_id(1);
    let at = order(4_000);
    let policy = store.required_policy();
    let signed = document(id, at)
        .serialize()
        .expect("a plain draft serializes")
        .sign(&Ed25519Signer::of_secret(FIXTURE_SECRET));
    let proof = store
        .publish_required_v1::<PlanReceipt, Plain>(&mut io, at, id, signed, policy)
        .expect("the Windows baseline publishes");
    assert_eq!(
        proof.properties().directory(),
        DirectorySync::UnavailableOnPlatform
    );
    assert_eq!(
        NativeIo::new().directory_sync(),
        DirectorySync::UnavailableOnPlatform
    );
    assert!(
        sandbox
            .store_dir()
            .join(proof.file_name().spelled())
            .is_file()
    );
}
