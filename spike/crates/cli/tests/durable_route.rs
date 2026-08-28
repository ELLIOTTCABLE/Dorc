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
    clippy::expect_used,
    reason = "the fixture helpers sit beside the cases, where the in-tests allowance does not reach them"
)]

use std::path::{Path, PathBuf};
use std::process::Command;

mod sandbox;

use sandbox::ProfileSandbox;

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

/// The plan an apply ships: a null command, so the dispatch carries inert bytes.
///
/// Deliberately not a book, and deliberately doing nothing. What this battery observes is the
/// ORCHESTRATION — that an intent was published, that a session carried the bytes, and that the
/// outcome names the intent that authorized it — not what any command does on the far side.
const INERT_PLAN: &str = "#!/bin/sh\n:\n";

/// The destination an apply addresses. It is never resolved: the local driver stands in for the
/// remote shell over the production code path, so this is the name the run attributes to.
const DESTINATION: &str = "web9.example.net";

/// Point one apply at the local shell instead of ssh, over the production adapter.
///
/// Debug-only in the binary, and it announces itself on stderr — a run must never quietly say
/// "host" and mean "here" (`271:rul-sin-ordering` puts mis-attribution at the top).
fn through_a_local_shell(command: &mut Command, scratch: &Scratch) {
    let posix = internal_tooling::Posix::find().expect("this corpus needs a POSIX shell");
    command.env("DORC_TRANSPORT", format!("local:{}", posix.shell.display()));
    command.env(
        "DORC_TRANSPORT_INTERPRETER",
        if cfg!(windows) {
            format!("/usr/bin/{}", posix.name)
        } else {
            posix.shell.display().to_string()
        },
    );
    // Nothing ambient is reachable from the shipped bytes. The marker protocol uses `printf`
    // alone, which every shell in the floor carries as a builtin.
    command.env("PATH", scratch.path.join("no-such-tools"));
}

/// The store's documents, by species stem.
fn published_of(sandbox: &ProfileSandbox, stem: &str) -> Vec<String> {
    entries(&store_root(sandbox))
        .into_iter()
        .filter(|name| name.starts_with(stem))
        .collect()
}

/// The receipt identity a V1 filename carries: `<species>-v1-<order>-<id>.dorc-receipt`.
fn receipt_id_of(name: &str) -> String {
    name.trim_end_matches(".dorc-receipt")
        .rsplit('-')
        .next()
        .expect("a name splits")
        .to_owned()
}

#[test]
fn the_default_apply_publishes_its_intent_then_dispatches_and_records_what_it_reached() {
    let sandbox = ProfileSandbox::new("apply-why");
    let scratch = Scratch::new("apply-why");
    std::fs::write(scratch.path.join("plan.sh"), INERT_PLAN).expect("write the plan");

    let mut command = dorc(&sandbox, &scratch.path);
    through_a_local_shell(&mut command, &scratch);
    let out = command
        .args(["apply", "--host", DESTINATION, "--plan", "plan.sh"])
        .output()
        .expect("the built binary runs");
    assert!(
        out.status.success(),
        "the apply must complete for its receipts to mean anything; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let intents = published_of(&sandbox, "apply-intent-v1-");
    let outcomes = published_of(&sandbox, "apply-outcome-v1-");
    assert_eq!(intents.len(), 1, "one apply publishes one intent");
    assert_eq!(outcomes.len(), 1, "and records one outcome");
    let intent_id = receipt_id_of(intents.first().expect("one intent"));

    // The keyset the apply route created on its own first use — this profile never planned, so
    // the write path reached initialization from the apply side.
    assert!(
        keyset_dir(&sandbox)
            .join("keyset-manifest-v1.txt")
            .is_file(),
        "an apply on a clean profile initializes its own keyset"
    );

    let listing = why(&sandbox, &scratch, &["--all"]);
    assert!(
        listing.contains(&format!("answers-intent {intent_id}")),
        "the outcome must name the intent that authorized it; got:\n{listing}"
    );
    assert!(
        listing
            .lines()
            .any(|line| line.starts_with("edge apply-intent ") && line.contains(&intent_id)),
        "a second process must correlate the two species it read; got:\n{listing}"
    );
}

#[test]
fn an_apply_that_cannot_publish_its_intent_never_reaches_the_transport() {
    // THE PRE-DISPATCH BOUNDARY, pinned on WHICH refusal fires rather than on the exit status.
    //
    // The transport is pointed at a shell that does not exist, so a run that reached it would
    // report a transport failure and exit in the 13/14 family. A run that refuses FIRST exits on
    // the invocation path naming the durable. Asserting only "it failed" would pass either way,
    // which is the vacuous shape this arc keeps finding after the fact.
    let sandbox = ProfileSandbox::new("apply-refuse");
    let scratch = Scratch::new("apply-refuse");
    std::fs::write(scratch.path.join("plan.sh"), INERT_PLAN).expect("write the plan");
    // A FILE where the configuration root belongs: the keyset cannot be created under it, so the
    // write open refuses before anything is signed.
    let blocked = sandbox.config_root().join("dorc");
    std::fs::write(&blocked, "not a directory").expect("occupy the product root");

    let mut command = dorc(&sandbox, &scratch.path);
    command.env(
        "DORC_TRANSPORT",
        format!("local:{}", scratch.path.join("no-such-shell").display()),
    );
    let out = command
        .args(["apply", "--host", DESTINATION, "--plan", "plan.sh"])
        .output()
        .expect("the built binary runs");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("apply-plan-not-dispatchable"),
        "the refusal must name the durable; got: {stderr}"
    );
    // THE DISCRIMINATOR. A run that reached the transport announces the local driver and then
    // reports a transport failure, so neither word may appear — without this the case would pass
    // on the strength of a refusal that fired somewhere else entirely, which is the vacuous shape
    // an exit-status assertion has on every refusal.
    assert!(
        !stderr.contains("DORC_TRANSPORT") && !stderr.contains("transport"),
        "nothing transport-shaped may precede a refused publication; got: {stderr}"
    );
    assert!(
        !store_root(&sandbox).exists(),
        "an apply that published nothing left no store behind"
    );
}

#[test]
fn a_run_with_no_clock_publishes_nothing_and_says_so() {
    // THE UNDATED REFUSAL, at the production composition root and nowhere lower.
    //
    // Clocklessness is a supported capability: the library emits an undated document happily,
    // which is what stable tests need and what a diffable artifact will need. What must never
    // happen is one reaching a store that SELECTS by order — a document sorting below every dated
    // one would make `--last` answer with older history. So the refusal lives at the placement,
    // and this drives the whole binary to prove it is there rather than asserting it at a seam.
    //
    // A malformed fixture clock is how a run reaches the edge with no reading at all.
    let sandbox = ProfileSandbox::new("undated");
    let scratch = Scratch::new("undated");
    let stdin = scratch.path.join("records.txt");
    std::fs::write(&stdin, records()).expect("write the records");
    let input = std::fs::File::open(&stdin).expect("re-open the records");
    let out = dorc(&sandbox, &scratch.path)
        .env("DORC_FIXTURE_CLOCK_MS", "not-a-reading")
        .args(["plan", "--book=book.sh", "--results", "-"])
        .stdin(std::process::Stdio::from(input))
        .output()
        .expect("the built binary runs");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("durable-receipt-unwritten"),
        "an undated run must report the durable it did not write; got: {stderr}"
    );
    assert!(
        entries(&store_root(&sandbox)).is_empty(),
        "and must leave nothing in a store that selects by order"
    );
    // The run itself is unaffected: a postmortem aid failing is loud, never fatal.
    assert!(
        out.status.success(),
        "the plan still completed; stderr: {stderr}"
    );
}

#[test]
fn a_receipt_identity_retrieves_its_own_document_and_prefers_nothing() {
    // RETRIEVAL, not a second ranking. The store offers one selection — its maximum-order cohort
    // — and a second way to PREFER a candidate is what would reopen the fallback past a damaged
    // newest one. An exact identity match prefers nothing: it answers about the document carrying
    // that identity, and about no other, and about none at all when none does.
    let sandbox = ProfileSandbox::new("by-identity");
    let scratch = Scratch::new("by-identity");
    plan(&sandbox, &scratch);
    plan(&sandbox, &scratch);

    let published = entries(&store_root(&sandbox));
    assert_eq!(published.len(), 2, "two runs, two documents");
    let first = receipt_id_of(published.first().expect("two documents"));
    let second = receipt_id_of(published.get(1).expect("two documents"));
    assert_ne!(first, second, "two documents never share one identity");

    let listing = why(&sandbox, &scratch, &["--receipt", &first]);
    assert!(
        listing.contains(&format!("receipt {first}")),
        "the named document must be the one answered; got:\n{listing}"
    );
    assert!(
        !listing.contains(&format!("receipt {second}")),
        "and no other document may ride along; got:\n{listing}"
    );

    // An identity nothing carries answers about nothing rather than falling back to whatever was
    // nearest — the property the one-selection rule exists to keep.
    let absent = why(&sandbox, &scratch, &["--receipt", &"0".repeat(64)]);
    assert!(
        !absent.contains("receipt "),
        "an identity nothing carries must answer about no document; got:\n{absent}"
    );
}
