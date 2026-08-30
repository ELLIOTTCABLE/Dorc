//! The recorded-facts seat, fed by the REAL receipt-reading path.
//!
//! The binary publishes documents into a sandbox store; this battery reopens that store the way
//! `dorc why` does — standard roots, real keyset, real store, real signature check, real region —
//! and derives `RecordedWhyFacts` for each of the three root selectors.
//!
//! What it establishes is that the model is produced from material a real run wrote, rather than
//! from a fixture that agrees with itself. What it deliberately does NOT touch is user output: the
//! listing is unchanged, and joining these facts to a rendered surface is the next conductor's.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "fixture helpers beside the cases, where the in-tests allowance does not reach them"
)]

use std::path::{Path, PathBuf};
use std::process::Command;

use dorc_cli::durable::{LocalReceiptEdgeV1, NamedSpecies, NativeIo, ReadEdge};
use dorc_cli::recorded_facts::{ObservedSource, SelectedRoot, facts_for};
use dorc_receipt::ids::PlanReceiptId;
use dorc_receipt::report::{
    AuthenticationState, ClosureCompleteness, CurrentSourceReading, CurrentSourceState,
    DetailState, MaterialState, ReDerivationState, RecordedDocumentId, RecordedSpecies,
    RequestedAddress,
};

mod sandbox;

use sandbox::ProfileSandbox;

/// A book with one unmodeled command, so a run decides one site and records one.
const BOOK: &str = "#!/bin/sh\nhork tune --profile web\n";

struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("dorc-facts-{name}-{}", std::process::id()));
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

fn records() -> String {
    let digest = dorc_plan::invocation::book_digest(BOOK);
    format!(
        "dorc-records/1 nonce=dorc attempt=1 host=localhost book={digest} sites=1 @@dorc@@\n\
         dorc site 0 effect=holds rc=0 @@dorc@@\n\
         dorc-records-end/1 nonce=dorc @@dorc@@\n"
    )
}

/// Publish one plan receipt through the shipped binary, into `sandbox`'s own profile.
fn publish(sandbox: &ProfileSandbox, scratch: &Scratch) {
    let stdin = scratch.path.join("records.txt");
    std::fs::write(&stdin, records()).expect("write the records");
    let input = std::fs::File::open(&stdin).expect("re-open the records");
    let mut command = Command::new(env!("CARGO_BIN_EXE_dorc"));
    command.current_dir(&scratch.path);
    sandbox.apply(&mut command);
    command.env("DORC_FIXTURE_SOURCE_MATCH", "off");
    let out = command
        .args(["plan", "--book=book.sh", "--results", "-"])
        .stdin(std::process::Stdio::from(input))
        .output()
        .expect("the built binary runs");
    assert!(
        out.status.success(),
        "the plan must complete for its receipt to mean anything; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Reopen the sandbox store the way `dorc why` does, and hand back the edge.
fn reopen(sandbox: &ProfileSandbox) -> (LocalReceiptEdgeV1, NativeIo) {
    let roots = dorc_cli::durable::standard_roots(
        dorc_cli::durable::host_platform(),
        &SandboxEnvironment(sandbox.root_for_env()),
    )
    .expect("the sandbox names both roots");
    (LocalReceiptEdgeV1::of(roots), NativeIo::new())
}

/// The sandbox's own environment, as the root resolver's one query.
struct SandboxEnvironment(PathBuf);

impl dorc_cli::durable::RootEnvironment for SandboxEnvironment {
    fn var(&self, name: &str) -> Option<String> {
        let leaf = match name {
            "APPDATA" | "XDG_CONFIG_HOME" => "config",
            "LOCALAPPDATA" | "XDG_STATE_HOME" => "state",
            "HOME" => "home",
            _ => return None,
        };
        Some(self.0.join(leaf).to_string_lossy().into_owned())
    }
}

/// Every plan document in the store, decoded, with what the edge learned about each.
fn selected_roots(open: &ReadEdge, io: &mut NativeIo) -> Vec<SelectedRoot> {
    let store = open.store();
    let entries = store.enumerate(io).expect("the walk answers");
    let mut budget = store.graph_budget();
    let mut found = Vec::new();
    for entry in entries.recognized() {
        if entry.species() != NamedSpecies::Plan {
            continue;
        }
        let bytes = store
            .read_into_budget(io, entry, &mut budget)
            .expect("the document reads");
        let Ok(receipt) = open.read_plan(bytes.into_bytes().into_vec()) else {
            panic!("a document this profile wrote reads back under its own keyset");
        };
        let model = receipt
            .document()
            .model()
            .expect("its records close over themselves");
        let id = PlanReceiptId::of_hex(entry.name().receipt_id()).expect("a well-formed identity");
        found.push(SelectedRoot {
            receipt,
            model,
            identity: RecordedDocumentId::Plan(id),
            order: entry.name().order().spelled(),
            // The edge established both: the read came back inside the local-authentication
            // envelope, and it came back at all only because the region validated.
            authentication: AuthenticationState::Trusted,
            detail: DetailState::Available,
        });
    }
    found
}

/// THE INTEGRATION: a document the binary wrote, reopened, derives a model with the run's own
/// site and source in it.
///
/// The three root SELECTORS pick which document a question is rooted at, and each produces a model
/// through this one seat. Driven over a single-document store, because what varies between the
/// selectors is which root reaches the seat — not what the seat then does with it — and a store
/// with one document makes all three name the same root, which is what lets the assertion be about
/// the model rather than about the selection.
#[test]
fn a_document_the_binary_published_derives_recorded_facts_when_reopened() {
    let sandbox = ProfileSandbox::new("facts-route");
    let scratch = Scratch::new("facts-route");
    publish(&sandbox, &scratch);

    let (edge, mut io) = reopen(&sandbox);
    let open = edge.open_for_read(&mut io).expect("the store reopens");
    let roots = selected_roots(&open, &mut io);
    assert_eq!(roots.len(), 1, "one run published one plan document");
    let root = &roots[0];

    let facts = facts_for(root, Vec::new(), Vec::new(), Vec::new(), None);

    assert_eq!(facts.root().species(), RecordedSpecies::Plan);
    assert_eq!(facts.root().authentication(), AuthenticationState::Trusted);
    assert_eq!(
        facts.closure().completeness(),
        ClosureCompleteness::Complete
    );
    assert_eq!(
        facts.closure().reached().len(),
        1,
        "a rooted question with no siblings reached its own root and nothing else"
    );
    assert_eq!(
        facts.rederivation(),
        ReDerivationState::PendingKernelSupport,
        "nothing was re-derived, and the model says so rather than staying silent"
    );

    // NON-VACUITY: the run decided one site and acquired one general-sh book, and both reached the
    // model. A model that carried neither would satisfy every state assertion above.
    assert_eq!(
        facts.sites().len(),
        1,
        "the run's own site reached the model"
    );
    assert!(
        facts
            .sources()
            .iter()
            .any(|source| source.content() == MaterialState::Held),
        "and the book's exact bytes did too"
    );
}

/// An address resolves against the real document when the current book is the recorded one, and
/// refuses when the line moved — through the same seat, over the same store.
#[test]
fn an_address_resolves_or_refuses_against_the_real_recorded_source() {
    let sandbox = ProfileSandbox::new("facts-address");
    let scratch = Scratch::new("facts-address");
    publish(&sandbox, &scratch);

    let (edge, mut io) = reopen(&sandbox);
    let open = edge.open_for_read(&mut io).expect("the store reopens");
    let roots = selected_roots(&open, &mut io);
    let root = &roots[0];

    let book_source = facts_for(root, Vec::new(), Vec::new(), Vec::new(), None)
        .sources()
        .iter()
        .find(|source| source.content() == MaterialState::Held)
        .map(dorc_receipt::report::SourceFacts::ordinal)
        .expect("the book's bytes are in the document");

    let unchanged = facts_for(
        root,
        Vec::new(),
        Vec::new(),
        vec![ObservedSource {
            ordinal: book_source,
            reading: CurrentSourceReading::Read(BOOK.as_bytes().to_vec()),
            matches_digest: true,
        }],
        Some(RequestedAddress::of(book_source, 2)),
    );
    let address = unchanged.address().expect("the question named an address");
    assert_eq!(address.current(), CurrentSourceState::Matching);

    // THE MOVED LINE. The same command, one line further down. Nothing may find it there.
    let moved = format!("#!/bin/sh\necho inserted\n{}", &BOOK[10..]);
    let drifted = facts_for(
        root,
        Vec::new(),
        Vec::new(),
        vec![ObservedSource {
            ordinal: book_source,
            reading: CurrentSourceReading::Read(moved.into_bytes()),
            matches_digest: false,
        }],
        Some(RequestedAddress::of(book_source, 2)),
    );
    let address = drifted.address().expect("the question named an address");
    assert_eq!(address.current(), CurrentSourceState::Drifted);
    assert_eq!(
        address.resolved_site(),
        None,
        "a moved line never answers the address it moved away from"
    );
}

/// `dorc why`'s own output is unchanged by any of this.
///
/// The seat exists and is fed by the real path; joining it to a rendered surface is the next
/// conductor's work, and this case is what says the current surface has not moved under them.
#[test]
fn the_listing_surface_is_untouched_by_the_facts_seat() {
    let sandbox = ProfileSandbox::new("facts-listing");
    let scratch = Scratch::new("facts-listing");
    publish(&sandbox, &scratch);

    let mut command = Command::new(env!("CARGO_BIN_EXE_dorc"));
    command.current_dir(&scratch.path);
    sandbox.apply(&mut command);
    command.env("DORC_FIXTURE_SOURCE_MATCH", "off");
    let out = command
        .args(["why", "--receipt-last"])
        .output()
        .expect("the built binary runs");

    let listing = String::from_utf8_lossy(&out.stdout);
    assert!(
        listing.lines().any(|line| line.starts_with("receipt ")),
        "the recorded listing still names its document; got:\n{listing}"
    );
    assert!(
        listing.lines().any(|line| line.starts_with("sites ")),
        "and still counts its sites; got:\n{listing}"
    );
}

/// The sandbox's root, for the environment shim above.
trait RootForEnv {
    fn root_for_env(&self) -> PathBuf;
}

impl RootForEnv for ProfileSandbox {
    fn root_for_env(&self) -> PathBuf {
        self.config_root()
            .parent()
            .map(Path::to_path_buf)
            .expect("the sandbox config root sits under the sandbox")
    }
}
