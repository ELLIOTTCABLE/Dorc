//! The receipt family's ONE state-only home: state, exits, and cross-process relations.
//!
//! `30X` §3 splits the receipt batteries by assertion kind. A byte-exact render belongs in a loom
//! always — the loom is the prose-authoring surface, and nothing else can be edited into prose.
//! Structural and relational assertions — a keyset that exists or not, a document count, an
//! inequality across runs, a refusal code, two invocations byte-identical — belong in Rust, in this
//! one named home, and only where the bytes are un-goldenable.
//!
//! # What only this home can answer
//!
//! These drive a binary across a process boundary and ask what it DID: a keyset on disk, a second
//! process reading what the first wrote, a refusal that fired before the transport. Most drive the
//! seam-seeded `dorc-harness` twin; the liveness witness alone drives the shipped `dorc` with `Os`
//! seams, the one claim `30X:inv-division-at-the-narrowest-edge` keeps over the shipped binary.
//!
//! # The sandbox is the platform's own variables
//!
//! There is deliberately no Dorc-specific variable selecting a fixture provider, key, store, or
//! weaker policy. A sandbox is made by pointing the PLATFORM's standard roots somewhere
//! throwaway, so the resolution under test is the resolution that ships.
//!
//! # The sentinel
//!
//! A plan document's readable skeleton carries a site's ORDINAL, DISPOSITION and GRADE; the site's
//! own shell TEXT lives in the encrypted region. So a second process rendering the book's own
//! command has verified a signature and opened a region with the keyset the first process created —
//! which skeleton-only output could not fake.

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
    // The seam-driven twin: determinism from the seams, not the harness-shaped env the shipped
    // binary no longer reads. A DISTINCT seed per invocation (two runs mint different identities,
    // as the OS source did — the store-selection cases turn on it); the clock is pinned separately
    // ([`plan_at`]), so two runs can still share one recorded moment.
    let mut command = Command::new(env!("CARGO_BIN_EXE_dorc-harness"));
    command.current_dir(at);
    sandbox::scrub_harness_env(&mut command, sandbox.root());
    command.env("DORC_SEED", u64::from(next_ordinal()).to_string());
    command.env("DORC_SEAM_SOURCE_MATCH", "pinned:off");
    command
}

/// Run a plan over [`BOOK`], feeding it the records above on stdin.
fn plan(sandbox: &ProfileSandbox, scratch: &Scratch) -> String {
    plan_at(sandbox, scratch, None)
}

/// As [`plan`], with the controller clock reading this run records its document under.
fn plan_at(sandbox: &ProfileSandbox, scratch: &Scratch, clock: Option<&str>) -> String {
    plan_with(sandbox, scratch, clock, &[])
}

/// As [`plan_at`], with extra argv — the seat every store-selection case drives through.
fn plan_with(
    sandbox: &ProfileSandbox,
    scratch: &Scratch,
    clock: Option<&str>,
    extra: &[&str],
) -> String {
    let stdin = scratch.path.join("records.txt");
    std::fs::write(&stdin, records()).expect("write the records");
    let input = std::fs::File::open(&stdin).expect("re-open the records");
    let mut command = dorc(sandbox, &scratch.path);
    if let Some(millis) = clock {
        command.env("DORC_SEAM_CLOCK", format!("pinned:{millis}"));
    }
    let out = command
        .args(["plan", "--book=book.sh", "--results", "-"])
        .args(extra)
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
    why_streams(sandbox, scratch, args).0
}

/// As [`why`], keeping BOTH streams.
///
/// They carry different species: stdout is the rendered surface, and every report ABOUT the store
/// — unreadable, or a greatest order naming a cohort — is a typed diagnostic on stderr. A case
/// asserting one cannot see the other, which is how the ambiguity seat went untested while its
/// store primitive did not.
fn why_streams(sandbox: &ProfileSandbox, scratch: &Scratch, args: &[&str]) -> (String, String) {
    let out = dorc(sandbox, &scratch.path)
        .arg("why")
        .args(args)
        .output()
        .expect("the built binary runs");
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "asking why is a read; stderr: {stderr}"
    );
    (String::from_utf8_lossy(&out.stdout).into_owned(), stderr)
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
    let rendered = why(&sandbox, &scratch, &["--receipt-last"]);
    assert!(
        rendered.contains(&receipt_id_of(name)),
        "the surface names the document a second process read; got:\n{rendered}"
    );
    assert!(
        rendered.contains("Trusted"),
        "and says the signature checked under material this controller's policy named; got:\n{rendered}"
    );

    // THE SENTINEL. A site's own shell text lives in the encrypted region and nowhere in the
    // readable skeleton, so a second process rendering it has verified the signature and opened the
    // region with the keyset the FIRST process created. Skeleton-only output cannot reach this line.
    assert!(
        rendered.contains("hork tune"),
        "the region must yield the book's own recorded shell; got:\n{rendered}"
    );

    // Non-vacuity: a document recording no decision would satisfy the sentinel while proving the
    // route carried nothing.
    assert!(
        rendered.contains("site 0"),
        "the run decided one site and the surface is about it; got:\n{rendered}"
    );
}

/// The signing identity one published document names, read from the document's own skeleton.
///
/// Read from the FILE rather than from a render: the readable skeleton is a product goal
/// (`30R:ruled-product-shape`), and the receipt-rooted surface reports what verification SAID
/// rather than which key said it — so the question "did two profiles mint two identities" is asked
/// where the answer lives.
fn signing_identity(sandbox: &ProfileSandbox) -> String {
    let published = entries(&store_root(sandbox));
    let name = published.first().expect("at least one document");
    let bytes =
        std::fs::read_to_string(store_root(sandbox).join(name)).expect("the document reads");
    bytes
        .lines()
        .find_map(|line| line.strip_prefix("signing-key-id "))
        .expect("the skeleton names its signing identity")
        .to_owned()
}

#[test]
fn asking_why_creates_nothing_and_says_what_it_found() {
    // `why` NEVER initializes. A profile with no keyset and no store must come back with a report
    // rather than a fresh identity — one that could not open a receipt written under the old one.
    let sandbox = ProfileSandbox::new("why-only");
    let scratch = Scratch::new("why-only");

    let (listing, stderr) = why_streams(&sandbox, &scratch, &["--receipt-last"]);
    assert!(
        stderr.contains("warning[durable-receipt-unreadable]"),
        "an empty profile must report WHICH state it found, by code; got: {stderr}"
    );
    // The closed reason word rides the payload and is NOT assertable here: with the register
    // unwritten the render is the greppable placeholder, which interpolates no parameter. It
    // becomes visible - and worth pinning - the day this code has prose.
    assert!(
        listing.is_empty(),
        "a store with nothing to say puts no listing on stdout; got: {listing}"
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
    let one = signing_identity(&first);
    let two = signing_identity(&second);
    assert_ne!(
        one, two,
        "two clean profiles minted the same signing identity, so it is not being generated"
    );

    plan(&first, &scratch);
    // A REOPEN means both documents still verify under one keyset: replaced material would leave
    // the earlier one unreadable, and `why` would answer nothing rather than a surface.
    assert!(
        !why(&first, &scratch, &["--receipt-last"]).is_empty(),
        "a second run in one profile must reopen the keyset it found, never replace it"
    );
    assert_eq!(
        signing_identity(&first),
        one,
        "and the identity the documents name is the one the first run minted"
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
    let posix = dorc_transport::Posix::find().expect("this corpus needs a POSIX shell");
    let interpreter = if cfg!(windows) {
        format!("/usr/bin/{}", posix.name)
    } else {
        posix.shell.display().to_string()
    };
    command.env(
        "DORC_SEAM_TRANSPORT",
        format!("local:{};{interpreter}", posix.shell.display()),
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

    // ROOTED at the newest document, not enumerated: `--all` no longer selects store entries
    // (`30R:receipt-rooted-attention-and-cli`), so what reaches this listing is the outcome the
    // last-selection derived plus the typed edges the graph correlated. That is the stronger
    // statement — a union of every history would have shown the same lines without proving the
    // outcome was reachable FROM anything.
    let outcome_id = receipt_id_of(outcomes.first().expect("one outcome"));
    let rendered = why(&sandbox, &scratch, &["--receipt-last"]);
    assert!(
        rendered.contains(&intent_id),
        "the outcome must name the intent that authorized it; got:\n{rendered}"
    );
    // The CORRELATION is asked of the machine register: a terminal render wraps a 129-character
    // pair across lines, so a text needle for it would be asserting a layout rather than a fact.
    let machine = why(&sandbox, &scratch, &["--receipt-last", "--json"]);
    assert!(
        machine.contains(&format!("{intent_id} {outcome_id}")),
        "a second process must correlate the two species it read; got:\n{machine}"
    );
    assert!(
        machine.contains("complete"),
        "and the outcome's own terminal state reaches the surface; got:\n{machine}"
    );

    // THE FILE ROOT over a non-plan document, with `--receipts` doing its orthogonal job: the store
    // is still walked, so an outcome named by PATH reaches the intent it answers — the
    // bounded-discovery half of `30R:receipt-rooted-attention-and-cli`.
    let outcome_file = store_root(&sandbox).join(outcomes.first().expect("one outcome"));
    let named = why(
        &sandbox,
        &scratch,
        &["--receipt", &outcome_file.display().to_string(), "--json"],
    );
    assert!(
        named.contains(&format!("{intent_id} {outcome_id}")),
        "an explicitly-named outcome still reaches the intent it answers; got:\n{named}"
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
        "DORC_SEAM_TRANSPORT",
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
        !stderr.contains("DORC_SEAM_TRANSPORT") && !stderr.contains("transport"),
        "nothing transport-shaped may precede a refused publication; got: {stderr}"
    );
    assert!(
        !store_root(&sandbox).exists(),
        "an apply that published nothing left no store behind"
    );
}

#[test]
fn a_remote_apply_declining_its_receipt_touches_nothing_at_all() {
    // THE PRE-I/O BOUNDARY, measured rather than argued. A remote apply's intent publication IS
    // its dispatch authority (`30R:publication-and-dispatch-boundary`), so `--no-receipt` is
    // refused there — and the refusal has to land before anything is opened, which is a claim only
    // an untouched profile can make. Before this repair the flag parsed, the store was created,
    // and rich intent AND outcome receipts were written under a flag saying none would be.
    let sandbox = ProfileSandbox::new("apply-no-receipt");
    let scratch = Scratch::new("apply-no-receipt");
    std::fs::write(scratch.path.join("plan.sh"), INERT_PLAN).expect("write the plan");

    let mut command = dorc(&sandbox, &scratch.path);
    // Pointed at a shell that does not exist, so a run that got as far as the transport would say
    // so — the same discriminator the publication-refusal case above uses.
    command.env(
        "DORC_SEAM_TRANSPORT",
        format!("local:{}", scratch.path.join("no-such-shell").display()),
    );
    let out = command
        .args([
            "apply",
            "--host",
            DESTINATION,
            "--plan",
            "plan.sh",
            "--no-receipt",
        ])
        .output()
        .expect("the built binary runs");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("apply-receipt-not-optional"),
        "the refusal names the incompatibility; got: {stderr}"
    );
    assert!(
        !stderr.contains("DORC_SEAM_TRANSPORT") && !stderr.contains("transport"),
        "nothing transport-shaped may precede it; got: {stderr}"
    );
    assert!(
        !store_root(&sandbox).exists() && !keyset_dir(&sandbox).exists(),
        "a refused invocation created neither a store nor a keyset"
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
        .env("DORC_SEAM_CLOCK", "absent")
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

    let rendered = why(&sandbox, &scratch, &["--receipt-id", &first]);
    assert!(
        rendered.contains(&first),
        "the named document must be the one answered; got:\n{rendered}"
    );
    assert!(
        !rendered.contains(&second),
        "and no other document may ride along; got:\n{rendered}"
    );

    // An identity nothing carries answers about nothing rather than falling back to whatever was
    // nearest — the property the one-selection rule exists to keep.
    let absent = why(&sandbox, &scratch, &["--receipt-id", &"0".repeat(64)]);
    assert!(
        absent.trim().is_empty(),
        "an identity nothing carries must answer about no document; got:\n{absent}"
    );
}

/// Every file that spawns the shipped binary points its per-user roots somewhere throwaway.
///
/// A lexical census, because no type can say "this `Command` had its environment set". It exists
/// because the shape it catches was found twice by hand in one lane: the corpus harness pointed
/// only its STATE root at a sandbox and minted a real keyset in the runner's profile, and the
/// deterministic differential sweep pointed neither. Both were invisible to a green suite — the
/// runs passed, and the residue was somewhere no assertion looked.
///
/// One direction only, and that is the useful one: a file that drives the binary and names no
/// sandbox seat fails. The walk asserts it found spawners, so a census looking in the wrong place
/// fails rather than passing over an empty set.
#[test]
fn every_seat_that_drives_the_binary_sandboxes_the_profile_it_writes_into() {
    /// How far after a spawn the sandbox must be spelled.
    ///
    /// POSITIONAL, not per-file. Measured: a per-file census passes on the mere presence of the
    /// helper's own DEFINITION, so removing every CALL leaves it green — the shape that makes a
    /// lexical check satisfiable by something that is not the property.
    const WITHIN: usize = 1200;
    /// How a file reaches the built binary.
    ///
    /// The three spellings the tree uses. A fourth would be a new shape somebody wrote
    /// deliberately, and the non-empty floor below is what says this list still finds the ones
    /// that exist rather than silently matching nothing.
    const SPAWNS: [&str; 3] = [
        "Command::new(env!(\"CARGO_BIN_EXE_dorc\"))",
        "Command::new(&self.dorc)",
        "Command::new(&tools.dorc)",
    ];
    /// How a spawner points that binary's roots somewhere throwaway.
    ///
    /// Spelled as a CALL taking the command by mutable reference, never as a bare name: a name
    /// alone is satisfied by the helper's own definition sitting elsewhere in the file.
    const SANDBOXES: [&str; 2] = [".apply(&mut ", "sandbox_profile(&mut "];

    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/");
    let mut spawners = 0_usize;
    let mut unsandboxed: Vec<String> = Vec::new();

    let mut stack = vec![crates.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|ext| ext != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for needle in SPAWNS {
                for (at, _) in text.match_indices(needle) {
                    spawners = spawners.saturating_add(1);
                    let window = text
                        .get(at..text.len().min(at.saturating_add(WITHIN)))
                        .unwrap_or_default();
                    if !SANDBOXES.iter().any(|seat| window.contains(seat)) {
                        unsandboxed.push(format!("{} near byte {at}", path.display()));
                    }
                }
            }
        }
    }
    assert!(
        spawners > 2,
        "the census found {spawners} seats driving the binary; it is looking in the wrong place"
    );
    assert!(
        unsandboxed.is_empty(),
        "these drive the shipped binary without pointing its per-user roots anywhere throwaway, \
         so a run of them writes keys and receipts into whoever is running the suite: \
         {unsandboxed:?}"
    );
}

/// The instant two runs are pinned to share, so their documents land at one store order.
///
/// Pinned rather than raced: the order IS the controller's clock reading, so at real wall-clock
/// two runs differ by however long a process takes, and a case built on that would pass or fail
/// with the machine. A round number in 2026, on the corpus harness's own footing, so a reader can
/// tell at a glance that the moment is fixture.
const ONE_MOMENT_MS: &str = "1769306437000";

#[test]
fn two_runs_at_one_recorded_moment_leave_a_last_the_store_cannot_name() {
    // THE AMBIGUITY SEAT, through the shipped binary. The store's ONE selection is its
    // maximum-order cohort, and its order is when the run was recorded — so two runs recorded at
    // one moment are a store that genuinely cannot say which of them is last. What must happen is
    // that it SAYS so and explains NOTHING: a tie-break would pick by the value least related to
    // when a document was written, and answering about both would be the whole-store union
    // `30R:receipt-rooted-attention-and-cli` refuses. A cohort is not something a surface roots at.
    let sandbox = ProfileSandbox::new("ambiguous-last");
    let scratch = Scratch::new("ambiguous-last");
    plan_at(&sandbox, &scratch, Some(ONE_MOMENT_MS));
    plan_at(&sandbox, &scratch, Some(ONE_MOMENT_MS));

    let published = entries(&store_root(&sandbox));
    assert_eq!(published.len(), 2, "two runs publish two documents");
    let first = receipt_id_of(published.first().expect("two documents"));
    let second = receipt_id_of(published.get(1).expect("two documents"));
    assert_ne!(first, second, "two documents never share one identity");

    let (listing, stderr) = why_streams(&sandbox, &scratch, &["--receipt-last"]);
    assert!(
        stderr.contains("warning[durable-receipt-ambiguous]"),
        "an unnameable last must be reported by code; got: {stderr}"
    );
    // REPORTED, never resolved — and never half-resolved either: neither member is explained,
    // because an explanation is rooted at a document and no document was selected.
    assert!(
        listing.trim().is_empty(),
        "a cohort is not a root, so nothing is explained; got:\n{listing}"
    );

    // THE FAILING DIRECTION. Naming one identity is retrieval, not a ranking, so there is no
    // greatest-order question left to be ambiguous about — and a case that only ever saw the
    // warning fire could not tell this seat from one that reports it unconditionally.
    let (named, quiet) = why_streams(&sandbox, &scratch, &["--receipt-id", &first]);
    assert!(
        !quiet.contains("durable-receipt-ambiguous"),
        "retrieval by identity asks no ordering question; got: {quiet}"
    );
    assert!(
        named.contains(&first) && !named.contains(&second),
        "and answers about the named document alone; got:\n{named}"
    );
}

// ---------------------------------------------------------------------------
// `--receipts <folder>`: the exact store root, and what it must NOT move

/// The folder the admin names IS the store root, with nothing appended beneath it.
///
/// The whole point of the flag is that an operator can say where their receipts go and then find
/// them there. A silent `receipts-v1` component beneath the named folder would put them one level
/// down from the directory the operator was looking at.
#[test]
fn an_explicit_store_receives_the_receipt_at_exactly_the_named_folder() {
    let sandbox = ProfileSandbox::new("explicit-store");
    let scratch = Scratch::new("explicit-store");
    let named = scratch.path.join("elsewhere");

    plan_with(
        &sandbox,
        &scratch,
        None,
        &[&format!("--receipts={}", named.display())],
    );

    let published = entries(&named);
    assert_eq!(published.len(), 1, "the named folder holds the document");
    assert!(
        published
            .first()
            .is_some_and(|name| name.starts_with("plan-v1-")),
        "and it is a typed receipt name: {published:?}"
    );
    assert!(
        !named.join("receipts-v1").exists(),
        "no second component is appended beneath the folder the admin named"
    );

    // THE OTHER HALF, and the one a passing first half would hide: the default store did not also
    // receive it. A run that wrote to both would satisfy every assertion above.
    assert!(
        !store_root(&sandbox).exists(),
        "an explicit store is the store, not an additional one"
    );

    // KEYS STAY STANDARD (`30Rd`: no custom key root). The store moved; custody did not.
    assert!(
        keyset_dir(&sandbox)
            .join("keyset-manifest-v1.txt")
            .is_file(),
        "the keyset is still under the standard configuration root"
    );
}

/// A second process reads the same explicit store, and the two stores never cross-read.
#[test]
fn a_second_process_reads_the_explicit_store_and_neither_store_sees_the_other() {
    let sandbox = ProfileSandbox::new("explicit-read");
    let scratch = Scratch::new("explicit-read");
    let named = scratch.path.join("elsewhere");
    let flag = format!("--receipts={}", named.display());

    // One document in the DEFAULT store, one in the explicit store, from two runs.
    plan(&sandbox, &scratch);
    plan_with(&sandbox, &scratch, None, &[&flag]);

    let explicit = why(&sandbox, &scratch, &["--receipt-last", &flag]);
    let default = why(&sandbox, &scratch, &["--receipt-last"]);

    let elsewhere = receipt_id_of(entries(&named).first().expect("the explicit store has one"));
    let here = receipt_id_of(
        entries(&store_root(&sandbox))
            .first()
            .expect("the default store has one"),
    );
    assert!(
        explicit.contains(&elsewhere) && !explicit.contains(&here),
        "the explicit store answered about its own document alone; got:\n{explicit}"
    );
    assert!(
        default.contains(&here) && !default.contains(&elsewhere),
        "and the default store about its own; got:\n{default}"
    );
}

/// Read-only `why` creates nothing, including an explicit folder that is not there.
///
/// `dorc why` must never bring a store into being for the answer to be read out of, and an
/// admin-named folder is exactly where that is easiest to get wrong — the path is right there in
/// argv, and creating it would look like helpfulness.
#[test]
fn a_read_only_why_never_creates_the_explicit_store() {
    let sandbox = ProfileSandbox::new("explicit-read-only");
    let scratch = Scratch::new("explicit-read-only");
    let absent = scratch.path.join("never-made");

    let (_, stderr) = why_streams(
        &sandbox,
        &scratch,
        &[
            "--receipt-last",
            &format!("--receipts={}", absent.display()),
        ],
    );

    assert!(
        !absent.exists(),
        "asking why created the store it was asked about"
    );
    assert!(
        stderr.contains("durable-receipt-unreadable"),
        "and reports the store it could not read; got: {stderr}"
    );
}

/// `--no-receipt` suppresses publication under either store selection.
///
/// Both arms, because the gate and the location are independent answers: a gate that consulted the
/// location would let naming a folder turn publication back on.
#[test]
fn no_receipt_suppresses_publication_under_either_store_selection() {
    let sandbox = ProfileSandbox::new("suppressed");
    let scratch = Scratch::new("suppressed");
    let named = scratch.path.join("elsewhere");

    plan_with(&sandbox, &scratch, None, &["--no-receipt"]);
    assert!(
        !store_root(&sandbox).exists(),
        "a refused receipt writes nothing to the default store"
    );

    plan_with(
        &sandbox,
        &scratch,
        None,
        &["--no-receipt", &format!("--receipts={}", named.display())],
    );
    assert!(
        !named.exists(),
        "and nothing to a named one: the refusal is the only thing that decides"
    );
}

/// One `dorc plan` over [`BOOK`] through the SHIPPED binary (`env!("CARGO_BIN_EXE_dorc")`), in
/// `sandbox`'s profile. The harness seams are set in its environment on purpose: the shipped binary
/// constructs only the production row (`Seams::os()`) and reads none of them
/// (`30X:inv-fixture-state-never-typeable-into-main`), so setting them must not change what it mints.
fn shipped_plan(sandbox: &ProfileSandbox, scratch: &Scratch) {
    let stdin = scratch.path.join("records.txt");
    std::fs::write(&stdin, records()).expect("write the records");
    let input = std::fs::File::open(&stdin).expect("re-open the records");
    let mut command = Command::new(env!("CARGO_BIN_EXE_dorc"));
    sandbox.apply(&mut command);
    command.current_dir(&scratch.path);
    command.env("DORC_SEED", "7");
    command.env("DORC_SEAM_RECEIPT_IDS", "seeded");
    command.env("DORC_SEAM_CLOCK", "pinned:1769306437000");
    let out = command
        .args(["plan", "--book=book.sh", "--results", "-"])
        .stdin(std::process::Stdio::from(input))
        .output()
        .expect("the built binary runs");
    assert!(
        out.status.success(),
        "the shipped plan must complete for its receipt to mean anything; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// The shipped binary draws its identities from the live OS, not a seam — the one witness this arc
/// keeps over `30X:inv-division-at-the-narrowest-edge` after the harness twin took the corpus.
///
/// Two `dorc plan` runs share one sandbox store and mint two DIFFERENT receipt identities, which a
/// seeded binary could not: it would compute one name twice and collide. And the keyset lands under
/// the sandboxed platform root, so the production roots really resolved there. The harness seams are
/// set in the environment throughout, so the run proves the shipped binary ignores them
/// (`30X:inv-fixture-state-never-typeable-into-main`) — a regression that read the seed would mint
/// one identity twice and fail this relation.
#[test]
fn the_shipped_binary_draws_live_os_identities_and_ignores_harness_seams() {
    let sandbox = ProfileSandbox::new("shipped-liveness");
    let scratch = Scratch::new("shipped-liveness");
    shipped_plan(&sandbox, &scratch);
    shipped_plan(&sandbox, &scratch);

    let published = entries(&store_root(&sandbox));
    assert_eq!(
        published.len(),
        2,
        "two runs into one store publish two documents"
    );
    assert_ne!(
        receipt_id_of(&published[0]),
        receipt_id_of(&published[1]),
        "the shipped binary mints from live OS entropy: two runs never share one identity"
    );
    assert!(
        keyset_dir(&sandbox)
            .join("keyset-manifest-v1.txt")
            .is_file(),
        "the production roots resolved to the sandboxed platform variables, where the keyset landed"
    );
}

/// Asking a plan-producing mode for a stored durable refuses, by naming the flag, through the
/// binary's own argv handling.
///
/// Driven through the real binary on purpose. The parser seat is already pinned beside
/// `reads_the_receipt` in `dorc_cli`'s own tests — but a guard proven at one seat, in one
/// direction, is the shape this arc keeps finding after the fact. No loom can express this cell:
/// its replay blocks require rc 0 and discard stderr, and its lint lane fixes both the subcommand
/// and the book, so a refusing invocation has nowhere to sit there.
///
/// MEASURED in its failing direction: with the refusal disabled `dorc plan --last book.sh` still
/// exits non-zero, on `cli-file-not-found`. So the exit status alone proves NOTHING — the slug
/// assertion is the whole test, and a rc check would leave a guard that passes whatever the parser
/// does. The `why` leg is not decoration: without it the case would pass just as happily if every
/// invocation refused for every reason, which would prove the flag unusable rather than confined.
#[test]
fn asking_a_plan_producing_mode_for_a_stored_durable_refuses_through_the_binary() {
    const SLUG: &str = "cli-flag-requires-mode";

    // A throwaway profile: these drive the REAL binary, which writes a durable by default, and an
    // inherited environment would deposit keys and receipts in whoever ran the suite.
    let sandbox = ProfileSandbox::new("plan-mode-refuses");
    for mode in ["plan", "apply", "probe", "round-trip", "bundle"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_dorc"));
        sandbox.apply(&mut command);
        let refused = command
            .args([mode, "--receipt-last", "book.sh"])
            .output()
            .expect("the built binary runs");
        assert!(
            !refused.status.success(),
            "`dorc {mode} --receipt-last` must not proceed: a stored record stream would stand \
             where a live measurement belongs"
        );
        let stderr = String::from_utf8_lossy(&refused.stderr);
        assert!(
            stderr.contains(SLUG) && stderr.contains("--receipt-last"),
            "`dorc {mode} --receipt-last` must refuse by naming the flag and the mode it belongs \
             to, rather than by any other refusal that happens to fire first; got: {stderr}"
        );
    }

    // The control: the same flag, on the surface that owns it, is not refused for this reason.
    let mut control = Command::new(env!("CARGO_BIN_EXE_dorc"));
    sandbox.apply(&mut control);
    let explained = control
        .args(["why", "--receipt-last", "--receipts=no-such-directory"])
        .output()
        .expect("the built binary runs");
    let stderr = String::from_utf8_lossy(&explained.stderr);
    assert!(
        !stderr.contains(SLUG),
        "`dorc why --receipt-last` is the one invocation the flag is for; got: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// The receipt-rooted `why` surface: relations and structure the loom shows but cannot ASSERT.
// The byte-exact renders are goldened by `why30-receipt-rooted-surface.loom`; these pin the
// cross-render relations (byte-identity across flags and invocations) and the address-resolution
// structure (a refusal reason present, a drifted source detected) that a single golden cannot state.

/// `--all` is a labelled synonym for the default on the receipt route, byte for byte.
///
/// DEPTH ONLY (`30R:receipt-rooted-attention-and-cli`): the total surface already renders everything
/// the reconstruction holds, so there is nothing deeper for the flag to reach. The loom SHOWS both
/// renders; this machine-ENFORCES they stay equal, which two independent goldens do not
/// (`30X` §4: byte-identity across invocations is Rust's honest home).
#[test]
fn all_is_byte_identical_to_the_default_on_the_receipt_route() {
    let sandbox = ProfileSandbox::new("all-identity");
    let scratch = Scratch::new("all-identity");
    plan(&sandbox, &scratch);

    assert_eq!(
        why(&sandbox, &scratch, &["--receipt-last", "--all"]),
        why(&sandbox, &scratch, &["--receipt-last"]),
    );
}

/// An address naming a file no recorded source reproduces REFUSES, and says so as a datum.
///
/// The refusal is in the answer rather than instead of it (`30R`: one unanswerable address is not a
/// reason to stop explaining the rest), so the surface still renders and carries the typed reason.
#[test]
fn an_unmatched_address_refuses_inside_the_answer() {
    let sandbox = ProfileSandbox::new("address-refusal");
    let scratch = Scratch::new("address-refusal");
    plan(&sandbox, &scratch);
    std::fs::write(scratch.path.join("other.sh"), "#!/bin/sh\ntrue\n").expect("write a stranger");

    let rendered = why(&sandbox, &scratch, &["--receipt-last", "other.sh:2"]);
    assert!(
        rendered.contains("address-unplaceable NoRecordedSourceMatches"),
        "a file the document never recorded is unplaceable, by name; got:\n{rendered}"
    );
    assert!(
        rendered.contains("[unwritten: why-total-section-data]"),
        "and every unrelated fact still renders; got:\n{rendered}"
    );

    let missing = why(&sandbox, &scratch, &["--receipt-last", "nowhere.sh:2"]);
    assert!(
        missing.contains("address-unplaceable CurrentSourceUnreadable"),
        "a file that is not there is a different refusal; got:\n{missing}"
    );
    let shapeless = why(&sandbox, &scratch, &["--receipt-last", "book.sh"]);
    assert!(
        shapeless.contains("address-unplaceable NotAFileAndLine"),
        "and an address that is not `<file>:<line>` is a third; got:\n{shapeless}"
    );
}

/// `--receipt <file>` roots at the named document rather than answering nothing.
///
/// The regression this closes by name: the `File` arm matched no store entry, so an explicit file
/// selected nothing at all and the route emitted a store-unreadable report over a store that had
/// just been written into.
#[test]
fn an_explicit_receipt_file_roots_the_question() {
    let sandbox = ProfileSandbox::new("file-root");
    let scratch = Scratch::new("file-root");
    plan(&sandbox, &scratch);

    let name = published_of(&sandbox, "plan-v1-")
        .into_iter()
        .next()
        .expect("the run published a plan document");
    let file = store_root(&sandbox).join(name);

    let rooted = why(
        &sandbox,
        &scratch,
        &["--receipt", &file.display().to_string()],
    );
    assert_eq!(
        rooted,
        why(&sandbox, &scratch, &["--receipt-last"]),
        "the one document in this store answers the same whether it is named by path or derived"
    );
}

/// An AUTHENTICATED receipt's sources are compared without the user naming a file.
///
/// The asymmetry the comparison packet rules: for material this controller authenticated, the one
/// seat rehydrates the recorded path and reads it. Both directions, because a comparison that
/// always said one thing would be worth nothing — the second half rewrites the book under the
/// receipt and the same question comes back with the other answer.
#[test]
fn an_authenticated_receipt_compares_its_own_sources_unasked() {
    let sandbox = ProfileSandbox::new("implicit-compare");
    let scratch = Scratch::new("implicit-compare");
    plan(&sandbox, &scratch);

    let unchanged = why(&sandbox, &scratch, &["--receipt-last"]);
    assert!(
        unchanged.contains("Matching"),
        "the recorded book is still on disk and the seat read it; got:\n{unchanged}"
    );

    std::fs::write(
        scratch.path.join("book.sh"),
        "#!/bin/sh\nhork tune --profile web\necho drifted\n",
    )
    .expect("rewrite the book");
    let drifted = why(&sandbox, &scratch, &["--receipt-last"]);
    assert!(
        drifted.contains("Drifted"),
        "and a book that moved under the receipt says so; got:\n{drifted}"
    );
}
