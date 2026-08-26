//! The keyset against a real filesystem, on whichever platform is running.
//!
//! What this answers and the model cannot: real exclusive creation, real modes, real links, real
//! synchronization, and what a genuinely separate process finds. What it cannot answer and the
//! model can: every interruption, which is swept in `keyset_sweep.rs`.
//!
//! Neither layer substitutes for the other, so both exist.
//!
//! The sandbox is a uniquely named directory under the platform's own temporary location. On a
//! Unix leg that is a real Unix filesystem, which is what the mode assertions below require — a
//! Windows-mounted filesystem reached from a Unix process reports modes that are not the ones the
//! kernel is enforcing, so asserting against it would be asserting against a translation layer.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use std::sync::atomic::{AtomicU32, Ordering};

use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy};
use dorc_receipt_local::keyset::{
    KeyAvailability, LocalReadOpenV1, LocalWriteOpenV1, StorePresence, open_for_read,
    open_or_initialize_for_write,
};
use dorc_receipt_local::{LocalLimits, NativeIo, RootInputs, RootPlatform, RootRole};

/// Distinguishes concurrent cases inside one process; the process id distinguishes the runs.
static NEXT: AtomicU32 = AtomicU32::new(0);

/// A sandbox that removes itself.
struct Sandbox {
    root: std::path::PathBuf,
}

impl Sandbox {
    fn new(name: &str) -> Self {
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "dorc-keyset-{name}-{}-{ordinal}",
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

    fn keyset_dir(&self) -> std::path::PathBuf {
        self.root
            .join("cfg")
            .join("dorc")
            .join("receipt-keys-v1")
            .join("keyset-v1")
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

struct FixedSecret(u8);

impl KeySecretEntropy for FixedSecret {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        raw.fill(self.0);
        true
    }
}

/// One write open against the real filesystem, with a FRESH `NativeIo` — which is what makes each
/// call a separate attempt, owning only what it itself creates.
fn write_open(sandbox: &Sandbox, seed: u8) -> LocalWriteOpenV1 {
    let roots = sandbox.roots();
    let mut io = NativeIo::new();
    let store = StorePresence::probe(&roots, &mut io, &LocalLimits::V1);
    let mut generator = EntropyKeysetGenerator::over(FixedSecret(seed));
    open_or_initialize_for_write(&roots, &mut io, &LocalLimits::V1, store, &mut generator)
}

fn read_open(sandbox: &Sandbox) -> LocalReadOpenV1 {
    let mut io = NativeIo::new();
    open_for_read(&sandbox.roots(), &mut io, &LocalLimits::V1)
}

fn ready(outcome: LocalWriteOpenV1, what: &str) -> dorc_receipt_local::LocalWriteKeysV1 {
    match outcome {
        LocalWriteOpenV1::Ready(keys) => keys,
        LocalWriteOpenV1::Refused(state) => panic!("{what}: refused with {state:?}"),
    }
}

#[test]
fn a_clean_profile_initializes_on_this_platform_and_a_second_attempt_reopens_it() {
    // D2's exit, on whichever platform is running: the sequence completes against a real
    // filesystem, and a genuinely separate attempt finds the same identities rather than making
    // new ones. The second attempt uses a DIFFERENT generator seed, so an identity that matched
    // by coincidence is not a way for this to pass.
    let sandbox = Sandbox::new("firstuse");
    let first = ready(write_open(&sandbox, 1), "first use");
    let signing = first.signer().signing_key_id().hex();
    let encryption = first.encryption().encryption_key_id().hex();
    drop(first);

    for name in [
        "signing-private-v1.pk8",
        "encryption-private-v1.age",
        "keyset-manifest-v1.txt",
    ] {
        let path = sandbox.keyset_dir().join(name);
        assert!(path.is_file(), "{} was not written", path.display());
    }

    let second = ready(write_open(&sandbox, 2), "reopen");
    assert_eq!(second.signer().signing_key_id().hex(), signing);
    assert_eq!(second.encryption().encryption_key_id().hex(), encryption);

    // And the read path answers the same identity without creating anything.
    match read_open(&sandbox) {
        LocalReadOpenV1::Ready(keys) => {
            assert_eq!(keys.status(), &KeyAvailability::RichReadReady);
            assert!(keys.opener().is_some());
        }
        LocalReadOpenV1::Unavailable(state) => panic!("the read path answered {state:?}"),
    }
}

#[test]
fn two_clean_profiles_produce_two_different_keysets() {
    // Nothing about a keyset is derived from a fixed value, so two profiles that generated
    // independently must not agree. The signing halves are seeded identically ON PURPOSE, so what
    // this measures is the ENCRYPTION half's independence from anything the process controls.
    let first = Sandbox::new("distinct-a");
    let second = Sandbox::new("distinct-b");
    let one = ready(write_open(&first, 7), "profile a");
    let two = ready(write_open(&second, 7), "profile b");
    assert_ne!(
        one.encryption().encryption_key_id().hex(),
        two.encryption().encryption_key_id().hex()
    );
}

#[test]
fn asking_why_on_a_clean_profile_creates_nothing() {
    // The mutation-free promise, against a real filesystem: after a read open of a profile that
    // has no keyset, the configuration base is still empty.
    let sandbox = Sandbox::new("whyclean");
    match read_open(&sandbox) {
        LocalReadOpenV1::Unavailable(KeyAvailability::NotInitialized) => {}
        other => panic!("a clean profile read as {other:?}"),
    }
    let entries: Vec<_> = std::fs::read_dir(sandbox.root.join("cfg"))
        .expect("the base exists")
        .flatten()
        .collect();
    assert!(
        entries.is_empty(),
        "asking why created {:?}",
        entries
            .iter()
            .map(std::fs::DirEntry::path)
            .collect::<Vec<_>>()
    );
}

#[test]
fn an_existing_store_stops_a_missing_keyset_from_being_replaced() {
    // The same refusal the sweep proves, measured through real enumeration.
    let sandbox = Sandbox::new("storeoccupied");
    let store = sandbox.root.join("state").join("dorc").join("receipts-v1");
    std::fs::create_dir_all(&store).expect("a store directory");
    std::fs::write(store.join("something.dorc-receipt"), b"bytes").expect("an entry");

    match write_open(&sandbox, 3) {
        LocalWriteOpenV1::Refused(KeyAvailability::KeysetMissingWithExistingStore) => {}
        other => panic!("a store with history answered {other:?}"),
    }
    assert!(!sandbox.keyset_dir().exists(), "nothing was created");
}

#[cfg(unix)]
#[test]
fn every_object_this_creates_is_reachable_only_by_its_owner() {
    // The Unix half of the platform guarantee, on a real Unix filesystem. The mode rides the same
    // call that makes each object visible, so there is no window in which a key document exists
    // group- or other-readable.
    use std::os::unix::fs::PermissionsExt as _;

    let sandbox = Sandbox::new("modes");
    // The premise, measured rather than assumed. A Windows-mounted filesystem reached from a Unix
    // process reports modes the kernel is not enforcing, so a sandbox that landed on one would
    // make every assertion below pass while proving nothing. It is where the temporary directory
    // happens to be, which is exactly the kind of thing that changes without anyone noticing.
    assert!(
        !sandbox.root.starts_with("/mnt/"),
        "the sandbox landed at {}, which is not a real Unix filesystem",
        sandbox.root.display()
    );
    let _ = ready(write_open(&sandbox, 4), "first use");

    let mode = |path: &std::path::Path| {
        std::fs::metadata(path)
            .expect("it exists")
            .permissions()
            .mode()
            & 0o777
    };
    let keys = sandbox
        .root
        .join("cfg")
        .join("dorc")
        .join("receipt-keys-v1");
    for directory in [
        sandbox.root.join("cfg").join("dorc"),
        keys.clone(),
        sandbox.keyset_dir(),
    ] {
        assert_eq!(mode(&directory), 0o700, "{}", directory.display());
    }
    for name in [
        "signing-private-v1.pk8",
        "encryption-private-v1.age",
        "keyset-manifest-v1.txt",
    ] {
        let path = sandbox.keyset_dir().join(name);
        assert_eq!(mode(&path), 0o600, "{}", path.display());
    }
}

#[cfg(unix)]
#[test]
fn a_key_document_anyone_can_read_is_refused_before_its_bytes_are_fetched() {
    use dorc_receipt_local::keyset::PermissionSubject;
    use dorc_receipt_local::manifest::KeyRole;
    use std::os::unix::fs::PermissionsExt as _;

    let sandbox = Sandbox::new("permissive");
    let _ = ready(write_open(&sandbox, 5), "first use");
    let document = sandbox.keyset_dir().join("signing-private-v1.pk8");
    std::fs::set_permissions(&document, std::fs::Permissions::from_mode(0o644))
        .expect("the mode is ours to widen");

    match write_open(&sandbox, 5) {
        LocalWriteOpenV1::Refused(KeyAvailability::PermissionRefused {
            subject:
                PermissionSubject::KeyDocument {
                    role: KeyRole::Signing,
                },
        }) => {}
        other => panic!("a world-readable key document answered {other:?}"),
    }

    // The positive control: narrowed back, the same keyset opens. Without it the refusal above
    // would be satisfied by anything at all having gone wrong with this sandbox.
    std::fs::set_permissions(&document, std::fs::Permissions::from_mode(0o600))
        .expect("and ours to narrow");
    let _ = ready(write_open(&sandbox, 5), "after narrowing");
}

#[cfg(unix)]
#[test]
fn a_keyset_reached_through_a_link_is_refused_without_being_followed() {
    use dorc_receipt_local::keyset::PermissionSubject;

    let sandbox = Sandbox::new("linked");
    let elsewhere = sandbox.root.join("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("a real directory");
    let keys = sandbox
        .root
        .join("cfg")
        .join("dorc")
        .join("receipt-keys-v1");
    std::fs::create_dir_all(&keys).expect("the versioned key directory");
    std::os::unix::fs::symlink(&elsewhere, keys.join("keyset-v1")).expect("a link");

    match write_open(&sandbox, 6) {
        LocalWriteOpenV1::Refused(KeyAvailability::PermissionRefused {
            subject: PermissionSubject::Directory,
        }) => {}
        other => panic!("a linked keyset directory answered {other:?}"),
    }
    let leaked: Vec<_> = std::fs::read_dir(&elsewhere)
        .expect("it exists")
        .flatten()
        .collect();
    assert!(
        leaked.is_empty(),
        "the link was followed and written through: {:?}",
        leaked
            .iter()
            .map(std::fs::DirEntry::path)
            .collect::<Vec<_>>()
    );
}

#[cfg(windows)]
#[test]
fn the_windows_baseline_initializes_and_reports_the_operation_it_does_not_have() {
    // The Windows half. There is no mode to assert, so what is measured is that the sequence
    // completes under the inherited per-user access and that the platform's missing directory
    // synchronization is RECORDED rather than simulated as a success of a stronger kind.
    use dorc_receipt_local::io::LocalIo as _;
    use dorc_receipt_local::store::DirectorySync;

    let sandbox = Sandbox::new("windows");
    let _ = ready(write_open(&sandbox, 8), "first use");
    assert!(
        sandbox
            .keyset_dir()
            .join("keyset-manifest-v1.txt")
            .is_file(),
        "the completion marker was written"
    );
    assert_eq!(
        NativeIo::new().directory_sync(),
        DirectorySync::UnavailableOnPlatform
    );
}

#[test]
fn the_two_roles_land_under_their_own_role_typed_roots() {
    // Keys under configuration, receipts under state. On a platform where the two coincide the
    // versioned subdirectories still keep the roles apart; here they are separate bases, so the
    // separation is visible as a path.
    let sandbox = Sandbox::new("rolesplit");
    let _ = ready(write_open(&sandbox, 9), "first use");
    let roots = sandbox.roots();
    assert!(
        sandbox
            .keyset_dir()
            .starts_with(roots.base(RootRole::Configuration)),
        "the keyset sits under the configuration base"
    );
    assert!(
        !sandbox
            .keyset_dir()
            .starts_with(roots.base(RootRole::State)),
        "and not under the state base"
    );
    assert!(
        !sandbox.root.join("state").join("dorc").exists(),
        "and initializing a keyset created nothing under the state base at all"
    );
}
