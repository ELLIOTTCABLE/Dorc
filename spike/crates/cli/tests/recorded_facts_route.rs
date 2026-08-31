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

use dorc_aid::RenderCtx;
use dorc_cli::durable::{LocalReceiptEdgeV1, NamedSpecies, NativeIo, ReadEdge};
use dorc_cli::recorded_facts::{ObservedSource, SelectedRoot, facts_for};
use dorc_cli::why_json::{JsonValues, why_json};
use dorc_cli::why_total::{TerminalValues, why_total};
use dorc_receipt::graph::ReceiptGraph;
use dorc_receipt::ids::PlanReceiptId;
use dorc_receipt::report::{
    AuthenticationState, ClosureCompleteness, CurrentSourceReading, CurrentSourceState,
    DetailState, MaterialState, ReDerivationState, RecordedDocumentId, RecordedSpecies,
    RequestedAddress,
};
use dorc_why::recorded::{AddressStanding, Rooted, reconstruct};

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
            // Empty graph: a plan root reaches nothing further, and the walk is pinned in `receipt`.
            closure: ReceiptGraph::new().closure_from(&RecordedDocumentId::Plan(id)),
            order: entry.name().order(),
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

    let facts = facts_for(root, Vec::new(), Vec::new(), None);

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

    let book_source = facts_for(root, Vec::new(), Vec::new(), None)
        .sources()
        .iter()
        .find(|source| source.content() == MaterialState::Held)
        .map(dorc_receipt::report::SourceFacts::ordinal)
        .expect("the book's bytes are in the document");

    let unchanged = facts_for(
        root,
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

/// THE PRODUCTION ROUTE: the shipped binary answers `dorc why` with the TOTAL SURFACE.
///
/// Driven through the real binary in a second process, over a store the first one published into,
/// because that is the whole claim — the listing seats are gone, and what a reader gets is the
/// reconstruction the in-process cases above pin. The needles are structural (registry slugs and
/// the document's own identity), never prose: the words are `[unwritten:]` by design and re-blessing
/// them must not redden this file (`prose-pins-live-where-the-prose-does`).
#[test]
fn the_shipped_binary_answers_with_the_total_surface() {
    let sandbox = ProfileSandbox::new("facts-surface");
    let scratch = Scratch::new("facts-surface");
    publish(&sandbox, &scratch);

    let rendered = why(&sandbox, &scratch, &["--receipt-last"]);
    for section in [
        "why-total-section-carriers",
        "why-total-section-data",
        "why-total-section-loci",
    ] {
        assert!(
            rendered.contains(&format!("[unwritten: {section}]")),
            "the receipt-rooted surface renders its own sections; got:\n{rendered}"
        );
    }
    assert!(
        !rendered
            .lines()
            .any(|line| line.starts_with("signing-key ")),
        "the recorded LISTING is gone, not coexisting with the surface; got:\n{rendered}"
    );
    assert!(
        rendered.is_ascii(),
        "weft-ascii-forever binds the production route too"
    );
}

/// `--all` is a labelled synonym for the default on this route, byte for byte.
///
/// DEPTH ONLY (`30R:receipt-rooted-attention-and-cli`): the total surface already renders everything
/// the reconstruction holds, so there is nothing deeper for the flag to reach. Byte-identity is what
/// says so — a flag that changed one byte would be selecting.
#[test]
fn all_is_byte_identical_to_the_default_on_the_receipt_route() {
    let sandbox = ProfileSandbox::new("facts-all");
    let scratch = Scratch::new("facts-all");
    publish(&sandbox, &scratch);

    assert_eq!(
        why(&sandbox, &scratch, &["--receipt-last", "--all"]),
        why(&sandbox, &scratch, &["--receipt-last"]),
    );
}

/// `--json` is the same reconstruction, well-formed, with explicit withhold markers.
#[test]
fn the_json_register_parses_and_marks_its_withholds() {
    let sandbox = ProfileSandbox::new("facts-json");
    let scratch = Scratch::new("facts-json");
    publish(&sandbox, &scratch);

    let rendered = why(&sandbox, &scratch, &["--receipt-last", "--json"]);
    dorc_lint::json::parse(&rendered).expect("the envelope parses as JSON");
    assert!(
        rendered.contains("\"state\":\"present\"") && rendered.contains("\"value\":null"),
        "both slot spellings reach a real render; got:\n{rendered}"
    );
}

/// An address naming a file no recorded source reproduces REFUSES, and says so as a datum.
///
/// The refusal is in the answer rather than instead of it (`30R`: one unanswerable address is not a
/// reason to stop explaining the rest), so the surface still renders and carries the typed reason.
#[test]
fn an_unmatched_address_refuses_inside_the_answer() {
    let sandbox = ProfileSandbox::new("facts-address-refusal");
    let scratch = Scratch::new("facts-address-refusal");
    publish(&sandbox, &scratch);
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
    let sandbox = ProfileSandbox::new("facts-file-root");
    let scratch = Scratch::new("facts-file-root");
    publish(&sandbox, &scratch);

    let store = sandbox.state_root().join("dorc").join("receipts-v1");
    let file = std::fs::read_dir(&store)
        .expect("the store directory exists")
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("plan-v1-"))
        })
        .expect("the run published a plan document");

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

/// One invocation of the shipped binary's `why`, in this sandbox's profile.
fn why(sandbox: &ProfileSandbox, scratch: &Scratch, args: &[&str]) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_dorc"));
    command.current_dir(&scratch.path);
    sandbox.apply(&mut command);
    command.env("DORC_FIXTURE_SOURCE_MATCH", "off");
    let out = command
        .arg("why")
        .args(args)
        .output()
        .expect("the built binary runs");
    assert!(
        out.status.success(),
        "`dorc why {args:?}` exited {:?}; stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
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

/// THE TOTAL SURFACE over a document the binary published: every datum reaches the render exactly
/// once, and nothing is excluded.
///
/// A permutation over the population rather than a count (`count-drifts`): what must hold is that no
/// datum is dropped and none is doubled. The ledger is appended at the EMIT SITE, so a datum an
/// early return skipped would be missing here rather than merely uncounted.
#[test]
fn the_total_surface_reaches_every_datum_exactly_once() {
    let sandbox = ProfileSandbox::new("total-surface");
    let scratch = Scratch::new("total-surface");
    publish(&sandbox, &scratch);

    let (edge, mut io) = reopen(&sandbox);
    let open = edge.open_for_read(&mut io).expect("the store reopens");
    let roots = selected_roots(&open, &mut io);
    let facts = facts_for(&roots[0], Vec::new(), Vec::new(), None);
    let reconstruction = reconstruct(&Rooted::Plan(&facts), AddressStanding::AsRecorded);

    let ctx = RenderCtx::production();
    let mut encoder = TerminalValues::default();
    let (parts, coverage) = why_total(&reconstruction, &ctx, &mut encoder);

    assert!(
        !reconstruction.data().is_empty(),
        "a reconstruction over a real document is non-empty; an empty one satisfies the \
         permutation below vacuously"
    );
    let mut reached: Vec<usize> = coverage.reached().iter().map(|id| id.get()).collect();
    reached.sort_unstable();
    assert_eq!(
        reached,
        (0..reconstruction.data().len()).collect::<Vec<usize>>(),
        "every datum reaches the render exactly once, and no position is rendered twice"
    );
    assert!(
        coverage.excluded().is_empty(),
        "the total surface excludes nothing; its reason type is uninhabited"
    );

    let text = parts.text();
    assert!(text.is_ascii(), "weft-ascii-forever binds this surface too");
    for section in [
        "why-total-section-carriers",
        "why-total-section-data",
        "why-total-section-correlations",
        "why-total-section-loci",
    ] {
        assert!(
            text.contains(&format!("[unwritten: {section}]")),
            "the section renders its own registry row, unwritten until a conductor mints words: \
             {text}"
        );
    }
}

/// The render is a pure function of the reconstruction: two renders of one model are byte-equal.
///
/// `30V` §2 rul-stateful-narrowing-hard-gated turns on this — a user's multi-step dig re-enters by
/// ADDRESS rather than by session state, which is only true while an identical reinvocation
/// reproduces identical bytes.
#[test]
fn one_reconstruction_renders_identically_every_time() {
    let sandbox = ProfileSandbox::new("total-stable");
    let scratch = Scratch::new("total-stable");
    publish(&sandbox, &scratch);

    let (edge, mut io) = reopen(&sandbox);
    let open = edge.open_for_read(&mut io).expect("the store reopens");
    let roots = selected_roots(&open, &mut io);
    let facts = facts_for(&roots[0], Vec::new(), Vec::new(), None);
    let reconstruction = reconstruct(&Rooted::Plan(&facts), AddressStanding::AsRecorded);
    let ctx = RenderCtx::production();

    let first = why_total(&reconstruction, &ctx, &mut TerminalValues::default())
        .0
        .text();
    let second = why_total(&reconstruction, &ctx, &mut TerminalValues::default())
        .0
        .text();
    assert_eq!(first, second);
    assert!(!first.is_empty());
}

/// The `--json` sibling: the SAME reconstruction, well-formed, and total over the same population.
///
/// Parsed back through the workspace's own JSON reader rather than string-matched, because "the
/// bytes contain a brace" is not well-formedness. Version-unstable by open contract, which is why
/// the envelope names itself and nothing here pins a schema.
#[test]
fn the_json_sibling_is_well_formed_and_reaches_every_datum() {
    let sandbox = ProfileSandbox::new("total-json");
    let scratch = Scratch::new("total-json");
    publish(&sandbox, &scratch);

    let (edge, mut io) = reopen(&sandbox);
    let open = edge.open_for_read(&mut io).expect("the store reopens");
    let roots = selected_roots(&open, &mut io);
    let facts = facts_for(&roots[0], Vec::new(), Vec::new(), None);
    let reconstruction = reconstruct(&Rooted::Plan(&facts), AddressStanding::AsRecorded);

    let (text, coverage) = why_json(&reconstruction, &mut JsonValues::default());

    let parsed = dorc_lint::json::parse(&text).expect("the envelope parses as JSON");
    let dorc_lint::json::Json::Obj(fields) = parsed else {
        panic!("the envelope is an object");
    };
    let format = fields
        .iter()
        .find(|(key, _)| key == "format")
        .map(|(_, value)| value);
    assert!(
        matches!(format, Some(dorc_lint::json::Json::Str(name)) if name.contains("unstable")),
        "the envelope names its own instability in its first field"
    );

    let mut reached: Vec<usize> = coverage.reached().iter().map(|id| id.get()).collect();
    reached.sort_unstable();
    assert_eq!(
        reached,
        (0..reconstruction.data().len()).collect::<Vec<usize>>(),
        "the machine surface makes the same totality claim the text one does"
    );

    assert!(
        text.contains("\"state\":\"present\""),
        "a present slot says so rather than merely carrying a value"
    );
    assert!(
        text.contains("\"value\":null"),
        "a withheld slot keeps BOTH keys; a consumer must never infer an absence from a missing key"
    );
}
