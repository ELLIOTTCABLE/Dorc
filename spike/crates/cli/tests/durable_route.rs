//! The production durable route, through the shipped binary, across process boundaries.
//!
//! # What only this battery can answer
//!
//! Every other receipt test injects its capabilities. This one links none: it runs
//! `CARGO_BIN_EXE_dorc` in a throwaway profile and asks what the binary itself did — which is the
//! whole of D4's claim, and a claim no in-process battery can make. A run that published nothing
//! would satisfy every injected-capability case in the tree and fail here.
//!
//! # The sandbox is the platform's own variables
//!
//! There is deliberately no Dorc-specific variable selecting a fixture provider, key, store, or
//! weaker policy. A sandbox is made by pointing the PLATFORM's standard roots somewhere
//! throwaway, so the resolution under test is the resolution that ships.
//!
//! # The sentinel
//!
//! A plan document's readable skeleton carries a source's ORDINAL, ROLE, DIGEST and LENGTH; its
//! PATH lives in the encrypted region. So a second process printing the book's own path has
//! verified a signature and opened a region with the keyset the first process created — which
//! skeleton-only output could not fake.

#![expect(
    clippy::panic,
    clippy::expect_used,
    reason = "the fixture helpers sit beside the cases, where the in-tests allowance does not reach them"
)]

use std::path::{Path, PathBuf};
use std::process::Command;

mod support;

use support::ProfileSandbox;

/// A book with one unmodeled command, so the run decides one site and records one.
const BOOK: &str = "#!/bin/sh\nhork tune --profile web\n";

/// A scratch directory that removes itself.
struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "dorc-durable-{name}-{}-{}",
            std::process::id(),
            next_ordinal()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch directory");
        std::fs::write(path.join("book.sh"), BOOK).expect("write the book");
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn next_ordinal() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static NEXT: AtomicU32 = AtomicU32::new(0);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// The records stream a run of [`BOOK`] admits, framed against the book's own digest.
///
/// Spelled here rather than measured, because the intake CHECKS this digest against the book it
/// read: a stream framed against anything else refuses, which would make the case green for
/// the wrong reason.
fn records() -> String {
    let digest = dorc_plan::invocation::book_digest(BOOK);
    format!(
        "dorc-records/1 nonce=dorc attempt=1 host=localhost book={digest} sites=1 @@dorc@@\n\
         dorc site 0 effect=holds rc=0 @@dorc@@\n\
         dorc-records-end/1 nonce=dorc @@dorc@@\n"
    )
}

/// One invocation of the shipped binary, in `sandbox`'s profile and `at`'s directory.
fn dorc(sandbox: &ProfileSandbox, at: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_dorc"));
    command.current_dir(at);
    sandbox.apply(&mut command);
    // No transcript here depends on a repository, and resolving one would make the case flip with
    // where a developer's temp directory sits.
    command.env("DORC_FIXTURE_SOURCE_MATCH", "off");
    command
}

/// Run a plan over [`BOOK`], feeding it the records above on stdin.
fn plan(sandbox: &ProfileSandbox, scratch: &Scratch) -> String {
    let stdin = scratch.path.join("records.txt");
    std::fs::write(&stdin, records()).expect("write the records");
    let input = std::fs::File::open(&stdin).expect("re-open the records");
    let out = dorc(sandbox, &scratch.path)
        .args(["plan", "--book=book.sh", "--results", "-"])
        .stdin(std::process::Stdio::from(input))
        .output()
        .expect("the built binary runs");
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the plan must complete for its receipt to mean anything; stderr: {stderr}"
    );
    stderr
}

/// Ask a SECOND process what the store holds.
fn why(sandbox: &ProfileSandbox, scratch: &Scratch, args: &[&str]) -> String {
    let out = dorc(sandbox, &scratch.path)
        .arg("why")
        .args(args)
        .output()
        .expect("the built binary runs");
    assert!(
        out.status.success(),
        "asking why is a read; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Every file directly under one directory, sorted.
fn entries(at: &Path) -> Vec<String> {
    let mut found: Vec<String> = std::fs::read_dir(at)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    found.sort();
    found
}

fn store_root(sandbox: &ProfileSandbox) -> PathBuf {
    sandbox.state_root().join("dorc").join("receipts-v1")
}

fn keyset_dir(sandbox: &ProfileSandbox) -> PathBuf {
    sandbox
        .config_root()
        .join("dorc")
        .join("receipt-keys-v1")
        .join("keyset-v1")
}

/// The one line of `listing` whose first word is `word`, if there is one.
fn line_starting(listing: &str, word: &str) -> Option<String> {
    listing
        .lines()
        .find(|line| line.split(' ').next() == Some(word))
        .map(str::to_owned)
}

#[test]
fn a_clean_profile_publishes_a_receipt_a_second_process_verifies_and_opens() {
    let sandbox = ProfileSandbox::new("plan-why");
    let scratch = Scratch::new("plan-why");
    plan(&sandbox, &scratch);

    // The keyset the write path created on genuine first use, all three members, and no fourth.
    assert_eq!(
        entries(&keyset_dir(&sandbox)),
        vec![
            "encryption-private-v1.age".to_owned(),
            "keyset-manifest-v1.txt".to_owned(),
            "signing-private-v1.pk8".to_owned(),
        ],
        "first use creates exactly the three V1 keyset members"
    );

    let published = entries(&store_root(&sandbox));
    assert_eq!(published.len(), 1, "one run publishes one document");
    let name = published.first().expect("one document");
    assert!(
        name.starts_with("plan-v1-") && name.ends_with(".dorc-receipt"),
        "the name carries its species and version: {name}"
    );

    // Everything the run wrote is INSIDE the sandbox — the assertion that makes this case a
    // statement about where a real invocation puts things rather than about a directory a test
    // happened to look in.
    assert!(
        !sandbox.state_root().join("dorc").join("whylog").is_dir(),
        "the old durable has its own destination and this route does not write one there"
    );

    let listing = why(&sandbox, &scratch, &["--last"]);
    let signing = line_starting(&listing, "signing-key").expect("the listing names its key");
    assert!(
        signing.len() > "signing-key ".len(),
        "a signing identity is spelled, not merely announced: {signing}"
    );

    // THE SENTINEL. A source's path lives in the encrypted region and nowhere in the readable
    // skeleton, so a second process printing it has verified the signature and opened the region
    // with the keyset the FIRST process created. Skeleton-only output cannot reach this line.
    let opaque: Vec<&str> = listing
        .lines()
        .filter(|line| line.starts_with("opaque "))
        .collect();
    assert!(
        opaque
            .iter()
            .any(|line| line.ends_with("source-path book.sh")),
        "the region must yield the book's own path; got {opaque:?}"
    );

    // Non-vacuity: a document recording no decision would satisfy the sentinel while proving the
    // route carried nothing.
    let sites = line_starting(&listing, "sites").expect("the listing counts its sites");
    assert_eq!(sites, "sites 1", "the run decided one site and recorded it");
}

#[test]
fn asking_why_creates_nothing_and_says_what_it_found() {
    // `why` NEVER initializes. A profile with no keyset and no store must come back with a report
    // rather than a fresh identity — one that could not open a receipt written under the old one.
    let sandbox = ProfileSandbox::new("why-only");
    let scratch = Scratch::new("why-only");

    let listing = why(&sandbox, &scratch, &["--last"]);
    assert!(
        listing.starts_with("store-unreadable "),
        "an empty profile answers with what it found: {listing}"
    );
    assert!(
        !sandbox.config_root().join("dorc").exists(),
        "asking why created a configuration root"
    );
    assert!(
        !sandbox.state_root().join("dorc").exists(),
        "asking why created a state root"
    );
}

#[test]
fn two_clean_profiles_mint_different_identities_and_reopening_one_preserves_them() {
    // The identities are GENERATED, not derived from anything a build shares: two clean profiles
    // must disagree. And a reopen must not quietly replace what it found — a new identity over an
    // existing store is the unannounced key era `30Rd` forbids.
    let first = ProfileSandbox::new("identity-a");
    let second = ProfileSandbox::new("identity-b");
    let scratch = Scratch::new("identity");

    plan(&first, &scratch);
    plan(&second, &scratch);
    let one = line_starting(&why(&first, &scratch, &["--last"]), "signing-key")
        .expect("the first profile names its key");
    let two = line_starting(&why(&second, &scratch, &["--last"]), "signing-key")
        .expect("the second profile names its key");
    assert_ne!(
        one, two,
        "two clean profiles minted the same signing identity, so it is not being generated"
    );

    plan(&first, &scratch);
    let reopened = line_starting(&why(&first, &scratch, &["--all"]), "signing-key")
        .expect("the reopened profile names its key");
    assert_eq!(
        one, reopened,
        "a second run in one profile must reopen the keyset it found, never replace it"
    );
    assert_eq!(
        entries(&store_root(&first)).len(),
        2,
        "two runs publish two documents, and neither replaces the other"
    );
}
