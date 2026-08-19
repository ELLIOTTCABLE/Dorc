//! `dorc-loom` is the read-only transcript-template inspection command.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use dorc_aid::prose::Mint;
use dorc_loom::invocation::{ALL, Breadth, PublishArgs, THIS, Verb};
use dorc_loom::usage::{self, PROGRAM, Reading};
use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, FsStagingStore, GitRepository, Repository, Roots,
    SectionKey, SectionVariableId, StagedPublication, StagedReplay, StagingStore, accept_staged,
    build_publication, classify_prose_changes, compile_preview, corpus_ownership,
    load_arrangement_corpus, load_corpus_by_slug, refuse_foreign_components, render_publish_diff,
    replay_case_with_inputs, stage_publication,
};
use errorloom::{
    Case, CaseRenderer, ReplayInput, ReplayResult, RunEnv, RunError, execute_generic, read_case,
    read_case_text,
};

/// The `{{name}}` mechanism has no other trace: every committed case is fully rendered, so a
/// reader who has only ever seen transcripts has no way to learn that a value can be typed at all.
/// Both inventory surfaces say it, once, at the top.
const VALUE_SYNTAX_NOTE: &str = "type {{name}} in a sentence to insert or move one of these values; omitting one bakes it to \
     literal text";

/// Deep enough for the corpus's deliberately over-nested books to clear a recursive-descent parse.
const WORKER_STACK_BYTES: usize = 64 * 1024 * 1024;

/// Warnings, notes and progress go to STDERR through `tracing`, so STDOUT carries only what a
/// reader or a pipe is meant to parse. The subscriber supplies the per-line attribution leader;
/// nothing here formats.
///
/// `--this` silences the stream: that spelling runs INSIDE a loom replay, where the only correct
/// amount of commentary beside a transcript is none. It never reaches this binary today (a terminal
/// is not inside a case, and `parse_argv` says so), so this is the guard for the day it does.
fn install_diagnostics(argv: &[String]) {
    let level = if argv.iter().any(|word| word == THIS) {
        tracing_subscriber::filter::LevelFilter::OFF
    } else {
        tracing_subscriber::filter::LevelFilter::INFO
    };
    let _ = tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(level)
        .without_time()
        .try_init();
}

/// A worker thread with an explicit stack, because the main thread's is whatever the platform gave
/// it: on Windows the nesting-bound case overflowed it, and since the bare invocation is the whole
/// corpus, ONE case took every publish down with it — an overflow, not a diagnostic.
fn main() -> ExitCode {
    install_diagnostics(&std::env::args().skip(1).collect::<Vec<_>>());
    let outcome = std::thread::Builder::new()
        .stack_size(WORKER_STACK_BYTES)
        .spawn(run)
        .map_err(|error| format!("start the worker thread: {error}"))
        .and_then(|worker| {
            worker
                .join()
                .map_err(|_| "the worker thread panicked".to_owned())
        })
        .and_then(|result| result);
    match outcome {
        Ok(code) => code,
        Err(message) => {
            let _ = writeln!(io::stderr(), "{PROGRAM}: {message}");
            ExitCode::from(2)
        }
    }
}

enum Command {
    Publish(Publication),
    Vars {
        breadth: Breadth,
        cases: Vec<PathBuf>,
    },
    Scaffold {
        slug: String,
    },
    AddRegister {
        case: PathBuf,
        register: String,
    },
    Sections {
        cases: Vec<PathBuf>,
    },
    Keys,
    /// The index, or one verb's own page when the reader had already chosen a verb.
    Help(&'static str),
}

/// One resolved `publish` invocation.
struct Publication {
    cases: Vec<PathBuf>,
    /// The case list as the author spelled it, so a refusal can name the re-run they would type.
    spelled: String,
    env: RunEnv,
    quiet: bool,
    accept_metadata: bool,
    provenance: Provenance,
    verbatim: bool,
}

/// What the author said about provenance — the `--human` / `--slop` pair. `Default` and `Slop`
/// mint the same tier, and differ only over an edit landing on a human-written register.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Provenance {
    Default,
    Human,
    Slop,
}

impl Provenance {
    fn mint(self) -> Mint {
        match self {
            Provenance::Human => Mint::Human,
            Provenance::Default | Provenance::Slop => Mint::Slop,
        }
    }
}

type SelectedCase = (String, PathBuf);

/// Refusals carry their count so the closing status line can match the exit code it explains.
enum Inspected {
    Ready(Interpretation),
    Refused { cases: usize },
}

/// Everything one publish computed before it decided whether it may write.
struct Interpretation {
    publication: StagedPublication,
    consumer: DorcConsumer,
    losses: Vec<HoleLoss>,
}

/// One hole a publication gives up, with enough address to point at it and the transport's reasons.
struct HoleLoss {
    case: String,
    section: String,
    hole: String,
    reappears: bool,
    shared: bool,
}

struct GatedCases {
    repository: GitRepository,
    paths: Vec<SelectedCase>,
    touched: std::collections::BTreeSet<String>,
    staged: std::collections::BTreeSet<String>,
}

fn run() -> Result<ExitCode, String> {
    let Invoked { roots, command } = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match command {
        Command::Publish(publication) => publish_cases(&roots, &publication, &mut out),
        Command::Vars { breadth, cases } => print_variables(breadth, &cases, &mut out),
        Command::Scaffold { slug } => scaffold_case(&roots, &slug),
        Command::AddRegister { case, register } => add_register(&roots, &case, &register),
        Command::Sections { cases } => print_sections(&cases, &mut out),
        Command::Keys => print_keys(&mut out),
        Command::Help(page) => writeln!(out, "{page}")
            .map_err(|error| error.to_string())
            .map(|()| ExitCode::SUCCESS),
    }
}

/// Resolve one CASE argument to a real file.
///
/// Four spellings resolve, because a reader who has only ever seen a case's SLUG has no way to know
/// where the collection lives, and the previous single spelling (a path relative to `spike/`) was
/// nowhere stated. In order: the canonical collection by slug, the canonical collection by
/// filename, the path as given, and the path against the workspace root.
fn resolve_case(roots: &Roots, arg: &str) -> Result<PathBuf, String> {
    let slug = arg.strip_suffix(".loom").unwrap_or(arg);
    let tried = [
        roots.corpus().join(format!("{slug}.loom")),
        PathBuf::from(arg),
        roots.base().join(arg),
    ];
    if let Some(found) = tried.iter().find(|path| path.is_file()) {
        return Ok(found.clone());
    }
    Err(format!(
        "no case `{arg}`. A CASE is its bare slug (`whylog-unwritten`), its filename, or a path \
         relative to the current directory or to `spike/`; these were tried, in order: {}. \
         The collection is {} — `dorc-loom sections` with no arguments lists every case in it",
        tried
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", "),
        roots.corpus().display()
    ))
}

/// One parsed command line: the world it acts on, and what to do in it.
struct Invoked {
    roots: Roots,
    command: Command,
}

fn parse_args() -> Result<Invoked, String> {
    parse_argv(&std::env::args().skip(1).collect::<Vec<_>>())
}

/// Split from [`parse_args`] so the grammar is reachable from a test without a process.
///
/// What to SAY to a command line is `usage::read`'s, shared with the in-loom driver so a
/// transcript cannot teach words a terminal never says; what is left here is what only a terminal
/// can decide.
fn parse_argv(words: &[String]) -> Result<Invoked, String> {
    let words: Vec<&str> = words.iter().map(String::as_str).collect();
    let invocation = match usage::read(&words) {
        // A page reads nothing, so it never has to resolve the world the rest of the argv named.
        Reading::Help(page) => {
            return Ok(Invoked {
                roots: Roots::built_in()?,
                command: Command::Help(page),
            });
        }
        Reading::Refused(refusal) => return Err(refusal),
        Reading::Runs(invocation) => *invocation,
    };
    // A terminal is never inside a case, so this binary is the one seat `--this` can never resolve
    // at. Falling back to the bare form's every-case meaning would dump the whole corpus at
    // somebody who asked for exactly one (`30C:rul-this-is-a-global-flag`).
    if invocation.this {
        return Err(usage::with_page(
            &format!(
                "{THIS} names the case this invocation is running inside, so it resolves only \
                 where the command is a replay line in a case -- a terminal is not inside one. \
                 Name the case: `dorc-loom {} <slug>`",
                invocation.verb_name()
            ),
            &words,
        ));
    }
    let roots = Roots::resolve(invocation.root.as_deref())?;
    let command = match invocation.verb {
        Verb::Publish(args) => Command::Publish(Publication {
            cases: resolve_cases(&roots, &args.cases)?,
            spelled: if args.cases.is_empty() {
                ALL.to_owned()
            } else {
                args.cases.join(" ")
            },
            env: run_env(&args)?,
            quiet: args.quiet,
            accept_metadata: args.accept_metadata,
            provenance: provenance_of(&args)?,
            verbatim: args.verbatim,
        }),
        Verb::Vars(args) => Command::Vars {
            breadth: args.breadth(),
            cases: resolve_cases(&roots, &args.cases)?,
        },
        Verb::Scaffold { slug } => Command::Scaffold { slug },
        Verb::AddRegister { case, register } => Command::AddRegister {
            case: resolve_case(&roots, &case)?,
            register,
        },
        Verb::Sections(args) => Command::Sections {
            cases: resolve_cases(&roots, &args.cases)?,
        },
        Verb::Keys => Command::Keys,
    };
    Ok(Invoked { roots, command })
}

/// The `--human`/`--slop` pair, which say opposite things about the same registers.
fn provenance_of(args: &PublishArgs) -> Result<Provenance, String> {
    match (args.human, args.slop) {
        (true, true) => Err(format!(
            "{HUMAN} and {SLOP} say opposite things about the same registers; pass one\n{}",
            usage::usage_for("publish")
        )),
        (true, false) => Ok(Provenance::Human),
        (false, true) => Ok(Provenance::Slop),
        (false, false) => Ok(Provenance::Default),
    }
}

fn run_env(args: &PublishArgs) -> Result<RunEnv, String> {
    let mut env = RunEnv::new().path_dir(binary_dir()?);
    if let Some(shell) = &args.shell {
        env = env.shell(shell);
    }
    for path in &args.path {
        env = env.path_dir(path);
    }
    Ok(env)
}

/// Resolve a verb's CASE list; an empty one is the whole collection.
///
/// Which invocations may ARRIVE here empty is the grammar's question, not this function's
/// (`invocation::Invocation::target` — a bare read-only verb may, a bare `publish` refuses).
fn resolve_cases(roots: &Roots, cases: &[String]) -> Result<Vec<PathBuf>, String> {
    if cases.is_empty() {
        return corpus_cases(roots);
    }
    cases.iter().map(|case| resolve_case(roots, case)).collect()
}

/// `dorc-loom add-register CASE help` — mint a code's help register so the ordinary transcript loop
/// can fill it (`28L:rul-help-affordance-is-scaffold`).
///
/// The register is a CATALOG fact, so this publishes through the same generator a publish uses: the
/// lock gains `HelpRegister::Unwritten` and the case's transcript grows the
/// `= help: [unwritten: <slug>.help]` line the author then overtypes. Nothing here writes prose.
fn add_register(roots: &Roots, path: &Path, register: &str) -> Result<ExitCode, String> {
    if register != "help" {
        return Err(format!(
            "`help` is the only register that can be added; `message` exists on every code and \
             `{register}` is not a register"
        ));
    }
    let case = load(path)?;
    let slug = case
        .frontmatter()
        .scalar("code")
        .ok_or_else(|| {
            format!(
                "{} declares no `code`, so it owns no catalog registers",
                path.display()
            )
        })?
        .to_owned();
    let gated = gate_touched_set(roots, std::slice::from_ref(&path.to_path_buf()))?;
    if !gated.touched.is_empty() {
        return Err(format!(
            "{} has a prose edit that is not promoted yet, and adding a register rewrites the \
             case; run `dorc-loom publish {0}` first",
            path.display()
        ));
    }
    let mut consumer = DorcConsumer::new();
    consumer.seed_help_register(&slug).map_err(|refusal| match refusal {
        dorc_loom::SeedRefusal::MissingCode(slug) => format!(
            "no catalog row for `{slug}`; publish its defining case first: `dorc-loom publish {}`",
            path.display()
        ),
        dorc_loom::SeedRefusal::AlreadyPresent(slug) => format!(
            "`{slug}` already has a help register; edit its `= help:` line in {}, then \
             `dorc-loom publish {0}`",
            path.display()
        ),
    })?;
    publish(
        roots,
        &consumer,
        &std::collections::BTreeMap::from([(slug.clone(), case)]),
    )?;
    tracing::info!(
        "next: rebuild, then overtype `[unwritten: {slug}.help]` in {} with the remediation words",
        path.display()
    );
    tracing::info!("then: dorc-loom publish {0}", path.display());
    Ok(ExitCode::SUCCESS)
}

/// Write the empty defining-case skeleton for a freshly-minted code
/// (`288` §4 prop-scaffold-explicit-command). An EXPLICIT command, never a build or test
/// side-effect: tests never write source, and concurrent builders never race over the collection.
///
/// Everything the skeleton omits is deliberately red. Empty `when-fires`/`why` fail
/// `required_metadata_is_non_empty`; an empty replay output fails the same-slug coherence gate
/// (`check_hygiene`) until a genuinely-firing world is authored and blessed — the scaffold-and-forget
/// guard. `message` is never written, so the code renders `[unwritten: <slug>]` at every seat:
/// builders author zero user-facing prose (`error-authorship-tier`).
fn scaffold_case(roots: &Roots, slug: &str) -> Result<ExitCode, String> {
    if slug.is_empty()
        || !slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(format!(
            "slug {slug:?} is not a code slug (lowercase letters, digits, and hyphens)"
        ));
    }
    let path = roots.corpus().join(format!("{slug}.loom"));
    if path.exists() {
        return Err(format!(
            "{} already exists; scaffold never overwrites an authored case",
            path.display()
        ));
    }
    // The inventory block ships EMPTY like the replay above it; one `DORC_LOOM_DUMP` run fills
    // both. NEW cases only — it re-churns whenever its own values move, so an existing case
    // carries one at its author's judgment. It names no slug: `--this` is what keeps a rename
    // from stranding the block on a case that no longer answers to it.
    let skeleton = format!(
        "---\ncode: {slug}\nwhen-fires:\nwhy:\n---\n-- replay --\n\
         $ dorc plan --book=book.sh\n\n$ dorc-loom {THIS} vars\n"
    );
    std::fs::write(&path, skeleton)
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    tracing::info!("wrote {}", path.display());
    tracing::info!(
        "next: author `when-fires`/`why`, then replace the replay with a command that really fires `{slug}`"
    );
    tracing::info!(
        "then: dorc-loom publish {} (orchestrator-only, on a freshly verified binary)",
        path.display()
    );
    Ok(ExitCode::SUCCESS)
}

/// Yes, replace the committed metadata.
const ACCEPT_METADATA: &str = "--accept-metadata";
const HUMAN: &str = "--human";
const SLOP: &str = "--slop";

/// Yes, publish the reading I was just shown, holes and all
/// (`30C:rul-flag-names-the-act-not-the-history` — it names what you want done now, never a prior
/// interaction, which is why it is not spelled `--confirm`).
const VERBATIM: &str = "--verbatim";

/// Environment variables an agent harness announces itself with. One line to extend.
const AGENT_MARKERS: [&str; 2] = ["CLAUDECODE", "CLAUDE_CODE_ENTRYPOINT"];

/// The human-at-the-keyboard escape, shared verbatim with `.githooks/commit-msg`.
const HUMAN_ESCAPE: &str = "DORC_HUMAN_COMMIT";

/// Whether this invocation looks like an agent's. The lookup is a PARAMETER because this is the
/// one non-hermetic input a provenance decision reads, and `forbid(unsafe_code)` puts
/// `set_var` out of a test's reach.
fn looks_like_an_agent(var: &impl Fn(&str) -> Option<String>) -> bool {
    if var(HUMAN_ESCAPE).is_some_and(|value| value == "1") {
        return false;
    }
    AGENT_MARKERS
        .iter()
        .any(|marker| var(marker).is_some_and(|value| !value.is_empty()))
}

fn process_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// Refuse a publish that would rewrite committed `when-fires`/`when-used`/`why` unless the caller
/// said so (`28L:fnd-case-frontmatter-overwrites-lock-metadata`).
///
/// Before any write, not after: the suite gate that also holds this property only fires once the
/// files are already rewritten, which turns an accident into a revert ceremony. Both texts are
/// shown because the reader is holding the case and cannot see the entry.
fn refuse_metadata_drift(roots: &Roots, accepted: bool) -> Result<(), String> {
    let cases_dir = roots.corpus();
    let drift = dorc_loom::metadata_drift(
        &load_corpus_by_slug(&cases_dir)?,
        &load_arrangement_corpus(&cases_dir)?,
    );
    if drift.is_empty() {
        return Ok(());
    }
    if accepted {
        for item in &drift {
            tracing::warn!(
                "`{}` {}: {:?} replaces {:?}",
                item.slug,
                item.key,
                item.declared,
                item.committed
            );
        }
        return Ok(());
    }
    let listed: Vec<String> = drift
        .iter()
        .map(|item| {
            format!(
                "\n  `{}` {}:\n    case:      {:?}\n    committed: {:?}",
                item.slug, item.key, item.declared, item.committed
            )
        })
        .collect();
    Err(format!(
        "this publish would replace committed metadata that no prose edit asked it to. One slug's \
         several registry entries all read one case's frontmatter, so an unnoticed edit reaches \
         every one of them at once.{} \nOmit the key from the case to keep the committed words, \
         or say you mean it: add {ACCEPT_METADATA} to this publish.",
        listed.join("")
    ))
}

fn binary_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|error| format!("locate built tools: {error}"))?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "locate built tools: executable has no parent".to_owned())
}

/// Every committed defining case, in a stable order.
///
/// Sorted because a `read_dir` order is not guaranteed and the receipt is order-sensitive.
fn corpus_cases(roots: &Roots) -> Result<Vec<PathBuf>, String> {
    let dir = roots.corpus();
    let read =
        std::fs::read_dir(&dir).map_err(|error| format!("read {}: {error}", dir.display()))?;
    let mut cases: Vec<PathBuf> = read
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("read {}: {error}", dir.display()))?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|kind| kind == "loom"))
        .collect();
    if cases.is_empty() {
        return Err(format!("no .loom cases under {}", dir.display()));
    }
    cases.sort();
    Ok(cases)
}

/// `dorc-loom publish CASE...` — the one authoring verb.
///
/// The shape is: compute EVERYTHING, print what it does to the registers, then decide whether this
/// run may write. There is no decision to make between those steps, which is why there is no second
/// verb; the decision that does exist is whether a hole was lost, and the gate fires exactly there
/// (`30C:rul-any-hole-loss-confirms`).
fn publish_cases(
    roots: &Roots,
    publication: &Publication,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(&publication.cases)?;
    let agent = looks_like_an_agent(&process_env);
    refuse_human_mint_from_an_agent(publication.provenance, agent)?;
    refuse_metadata_drift(roots, publication.accept_metadata)?;
    let gated = gate_touched_set(roots, &publication.cases)?;
    let total = gated.paths.len();
    let interpretation = match inspect_cases(roots, &gated, publication, out)? {
        Inspected::Ready(interpretation) => interpretation,
        Inspected::Refused { cases } => {
            tracing::info!("{total} cases, {cases} refused");
            return Ok(ExitCode::from(1));
        }
    };
    if let Some(note) = report_demotions(
        interpretation.consumer.demoted(),
        publication.provenance,
        agent,
    )? {
        tracing::info!("{note}");
    }

    // Byte-identical on both paths by construction: one census, emitted before either branch can
    // add a word of its own (`30C:rul-flag-names-the-act-not-the-history`).
    if let Some(detail) = hole_loss_detail(&interpretation.losses) {
        tracing::warn!("{detail}");
    }
    let store = staging_store(roots)?;
    let disposition = disposition(!interpretation.losses.is_empty(), publication.verbatim);
    if disposition == Disposition::Refuse {
        stage_refusal(&store, &interpretation.publication, &publication.spelled)?;
        return Ok(ExitCode::from(1));
    }
    if disposition == Disposition::Confirm {
        accept_staged(&store, &interpretation.publication, &publication.spelled)?;
        tracing::info!("{VERBATIM}: publishing this interpretation as it stands");
    }

    let affected = touched_cases(&gated)?;
    let before = staged_bytes(&gated)?;
    let wrote = publish(roots, &interpretation.consumer, &affected)?;
    if disposition == Disposition::Confirm {
        store.discard()?;
    }
    warn_each(staged_case_notes(
        &gated.staged,
        &rewritten_staged(&gated, &before)?,
    ));
    warn_each(nothing_moved_note(!wrote, &gated.paths));
    Ok(ExitCode::SUCCESS)
}

/// What a publish may do with the interpretation it just printed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Disposition {
    /// Nothing was given up: write, and the staging is not consulted at all.
    Write,
    /// Holes were given up and the author confirmed: match the staging, write, spend it.
    Confirm,
    /// Holes were given up and nobody has confirmed: stage, write nothing, exit nonzero.
    Refuse,
}

/// The whole gate, in one place (`30C:rul-any-hole-loss-confirms`).
///
/// The asymmetry is deliberate: a CLEAN `--verbatim` writes without consulting the staging, because
/// there is no loss to confirm and demanding one would be the prior-interaction ceremony
/// `30C:rul-flag-names-the-act-not-the-history` refused. And dropping every hole in a section is
/// this same table, not a louder one (`30C:rul-no-special-case-for-dropping-all`).
fn disposition(lost_holes: bool, verbatim: bool) -> Disposition {
    match (lost_holes, verbatim) {
        (false, _) => Disposition::Write,
        (true, true) => Disposition::Confirm,
        (true, false) => Disposition::Refuse,
    }
}

/// Hold the computed interpretation for a `--verbatim` and say what the author must do to it.
fn stage_refusal(
    store: &FsStagingStore,
    computed: &StagedPublication,
    spelled: &str,
) -> Result<(), String> {
    if matches!(
        stage_publication(store, computed)?,
        dorc_loom::StagingWriteOutcome::CleanupPending
    ) {
        tracing::warn!(
            "staged; retained backup requires deliberate resolution; subsequent writes refuse"
        );
    }
    tracing::warn!(
        "nothing was written. Re-type the {{{{name}}}} markers where those values belong and \
         re-run, or publish this reading as it stands: `dorc-loom publish {VERBATIM} {spelled}`"
    );
    Ok(())
}

/// The census of what a publication gives up, and the transport's reason for each.
///
/// One text, emitted identically whether this run refuses or applies, because the two runs are the
/// same interpretation and an author comparing them should find nothing moved but the outcome.
/// Returns the note rather than emitting it, so its wording stays testable without a subscriber.
fn hole_loss_detail(losses: &[HoleLoss]) -> Option<String> {
    if losses.is_empty() {
        return None;
    }
    let mut lines = vec![format!(
        "this publish gives up {} hole(s); the transcript renders values, so this is the only \
         place that difference is visible:",
        losses.len()
    )];
    lines.extend(losses.iter().map(|loss| {
        format!(
            "      {{{{{}}}}} in {} ({}) -- {}",
            loss.hole,
            loss.case,
            loss.section,
            hole_loss_reason(loss)
        )
    }));
    Some(lines.join("\n"))
}

/// Why one hole went. Two independent facts, so a hole can carry both: what happened to its BYTES,
/// and whether WHICH occurrence went was settled by the edit or selected by the transport.
fn hole_loss_reason(loss: &HoleLoss) -> String {
    let mut reasons = Vec::new();
    if loss.reappears {
        reasons.push(
            "its rendered value is still there as literal text, frozen at whatever this render \
             happened to say",
        );
    }
    if loss.shared {
        reasons.push(
            "another hole in that section renders the same text, so which of them this edit \
             dropped is the reading I picked, not something the bytes settle",
        );
    }
    if reasons.is_empty() {
        reasons.push("this edit no longer interpolates it");
    }
    reasons.join("; and ")
}

/// The one seat that turns a note-producing function's answer into stderr lines.
fn warn_each(notes: impl IntoIterator<Item = String>) {
    for note in notes {
        tracing::warn!("{note}");
    }
}

/// The warning a reader who lost an hour to a silent run needed (`30C` item 6).
///
/// A publish can do exactly nothing and exit 0 — the wrong worktree, the wrong file, an edit
/// already published — and the ordinary summary line reads the same either way, because "0 touched"
/// is a number in a sentence rather than an answer to the question the reader is holding.
///
/// Returns the note rather than emitting it, so its wording stays testable without a subscriber.
fn nothing_moved_note(nothing_moved: bool, selected: &[SelectedCase]) -> Option<String> {
    if !nothing_moved {
        return None;
    }
    let scope = match selected {
        [(only, _)] => format!("`{only}`"),
        many => format!("{} selected cases", many.len()),
    };
    Some(format!(
        "this publish changed nothing: {scope} carry no unpublished prose edit against HEAD. If \
         you expected one, check that you edited the transcript in THIS worktree and that the case \
         you edited is the one you named."
    ))
}

/// `--human` claims who typed the words; the one environment that can falsify it wins.
fn refuse_human_mint_from_an_agent(provenance: Provenance, agent: bool) -> Result<(), String> {
    if provenance != Provenance::Human || !agent {
        return Ok(());
    }
    Err(format!(
        "{HUMAN} marks a register as written by a person, and this session announces itself as an \
         agent ({}). A person at this keyboard says so with {HUMAN_ESCAPE}=1; an agent's edits are \
         published without the flag, which is the ordinary path and needs nothing else.",
        AGENT_MARKERS.join(" / ")
    ))
}

/// What to say when this publish re-marks a human-written register as slop. An AGENT is told the
/// truth about a consequence of its own work and asked for nothing; a PERSON has most likely
/// forgotten `--human` mid-sprint, and losing their mark to a missing flag is worth a stop.
/// Returns the note rather than emitting it, so its wording stays testable without a subscriber.
fn report_demotions(
    demoted: &[String],
    provenance: Provenance,
    agent: bool,
) -> Result<Option<String>, String> {
    if demoted.is_empty() {
        return Ok(None);
    }
    let listed = demoted.join(", ");
    let count = demoted.len();
    if provenance == Provenance::Slop || agent {
        return Ok(Some(format!(
            "this publish re-marks {count} register(s) as slop that were marked \
             human-written: {listed}\n      Reworking prose through the loom is what re-marks it, \
             so this is the expected outcome of the edit.\n      No action is necessary."
        )));
    }
    Err(format!(
        "this publish would re-mark {count} human-written register(s) as slop: {listed}\nRe-run \
         with {HUMAN} to keep them marked as yours, or with {SLOP} to re-mark them deliberately."
    ))
}

/// The touched defining cases (dirty on-disk bytes) keyed by their `code` slug — the only cases a
/// prose edit re-renders and republishes.
fn touched_cases(gated: &GatedCases) -> Result<std::collections::BTreeMap<String, Case>, String> {
    let mut cases = std::collections::BTreeMap::new();
    for (relative_path, path) in &gated.paths {
        if !gated.touched.contains(relative_path) {
            continue;
        }
        let case = load(path)?;
        let slug = case
            .frontmatter()
            .scalar("code")
            .or_else(|| case.frontmatter().scalar("arrangement"))
            .ok_or_else(|| {
                format!(
                    "touched case {} declares neither `code` nor `arrangement`",
                    path.display()
                )
            })?
            .to_owned();
        cases.insert(slug, case);
    }
    Ok(cases)
}

/// Naming a staged case IS the remedy: dorc-loom mutates no index
/// (`282:rul-promote-is-one-atomic-act`), so a rewrite otherwise strands the author's `git add`
/// on their own pre-publish bytes.
///
/// Returns the notes rather than emitting them, so their wording stays testable without a
/// subscriber.
fn staged_case_notes(
    staged: &std::collections::BTreeSet<String>,
    rewritten: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    staged
        .iter()
        .map(|path| {
            if rewritten.contains(path) {
                format!(
                    "{path} was staged and has been rewritten; `git add` it again before \
                     committing -- dorc-loom never touches your index"
                )
            } else {
                format!("{path} is staged; dorc-loom read your worktree and never the index")
            }
        })
        .collect()
}

/// Read before publication, so the note can name exactly the cases a rewrite left stale.
fn staged_bytes(gated: &GatedCases) -> Result<std::collections::BTreeMap<String, Vec<u8>>, String> {
    gated
        .staged
        .iter()
        .map(|path| {
            gated
                .repository
                .current_bytes(path)
                .map(|bytes| (path.clone(), bytes))
        })
        .collect()
}

fn rewritten_staged(
    gated: &GatedCases,
    before: &std::collections::BTreeMap<String, Vec<u8>>,
) -> Result<std::collections::BTreeSet<String>, String> {
    let mut rewritten = std::collections::BTreeSet::new();
    for (path, bytes) in before {
        if gated.repository.current_bytes(path)? != *bytes {
            rewritten.insert(path.clone());
        }
    }
    Ok(rewritten)
}

/// Compute the entire preflighted candidate set from the edited mirror, then publish the changed
/// files — the lock first, then affected cases in lexical order — by per-target temp-file-and-rename
/// (`282:rul-promote-is-one-atomic-act`). All bytes and both fixpoints precede every write, so a
/// validation failure leaves committed files byte-identical; a mid-publication interruption is loud
/// in git and repaired by rerun. No journal, staging, rollback, or index mutation.
/// Returns whether anything was actually written, which is the only honest answer to "did this
/// publish do something".
fn publish(
    roots: &Roots,
    consumer: &DorcConsumer,
    affected: &std::collections::BTreeMap<String, Case>,
) -> Result<bool, String> {
    let cases_dir = roots.corpus();
    let corpus = load_corpus_by_slug(&cases_dir)?;
    let arrangements = load_arrangement_corpus(&cases_dir)?;
    let publication = build_publication(consumer, &corpus, &arrangements, affected)?;

    let mut wrote = false;
    for (path, bytes) in [
        (roots.catalog_lock(), &publication.lock),
        (roots.arrangement_lock(), &publication.arrangement_lock),
    ] {
        if file_differs(&path, bytes) {
            publish_file(&path, bytes)?;
            tracing::info!("wrote {}", path.display());
            wrote = true;
        }
    }
    for (slug, bytes) in &publication.cases {
        let path = cases_dir.join(format!("{slug}.loom"));
        if !path.is_file() {
            return Err(format!("defining case `{slug}` is not `{slug}.loom`"));
        }
        if file_differs(&path, bytes) {
            publish_file(&path, bytes)?;
            tracing::info!("wrote {}", path.display());
            wrote = true;
        }
    }
    if let Some(note) = stale_siblings_note(consumer, &corpus, &arrangements, affected) {
        tracing::warn!("{note}");
    }
    Ok(wrote)
}

/// The cases this publication does not rewrite, but does invalidate.
///
/// Promote republishes only what it was handed, and that is the right blast radius to WRITE. It is
/// the wrong one to stay silent about: a reworded shared component moves every render that spends
/// it, and one such edit left 37 sibling transcripts stale with nothing to say so until
/// `test:looms` went red much later, by which time nothing connected the failure to the publish
/// that caused it. Naming them keeps the write narrow and the cause attached.
///
/// Returns the note rather than emitting it, so its wording stays testable without a subscriber.
fn stale_siblings_note(
    consumer: &DorcConsumer,
    corpus: &std::collections::BTreeMap<String, Case>,
    arrangements: &std::collections::BTreeMap<String, Case>,
    affected: &std::collections::BTreeMap<String, Case>,
) -> Option<String> {
    let published = DorcConsumer::new();
    let stale: Vec<&str> = corpus
        .iter()
        .chain(arrangements)
        .filter(|(slug, _)| !affected.contains_key(slug.as_str()))
        .filter(|(_, case)| {
            // A case that will not render at all is somebody else's red, not this note's.
            matches!(
                (published.render_case(case), consumer.render_case(case)),
                (Ok(before), Ok(after)) if before != after
            )
        })
        .map(|(slug, _)| slug.as_str())
        .collect();
    if stale.is_empty() {
        return None;
    }
    Some(format!(
        "{} other case(s) now render differently and were NOT republished: {}\n  they spend a \
         component this publish reworded; `mise run test:looms` is where their stale transcripts \
         surface",
        stale.len(),
        stale.join(", ")
    ))
}

fn file_differs(path: &Path, bytes: &str) -> bool {
    std::fs::read_to_string(path).map_or(true, |current| current != bytes)
}

/// Replace one target by writing a sibling temp file and renaming over it (same directory, so the
/// rename does not cross a mount point). Not a crash-atomic transaction; the preflight above is where
/// atomicity lives.
fn publish_file(path: &Path, bytes: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("no parent dir for {}", path.display()))?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("no filename for {}", path.display()))?;
    let tmp = parent.join(format!(".{name}.dorc-loom-tmp"));
    std::fs::write(&tmp, bytes).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("rename into {}: {e}", path.display()))
}

/// Drive one case's replays through the Dorc adapter, routing declines to the generic executor.
type DrivenReplays = Vec<ReplayResult<SectionKey, SectionVariableId>>;

fn drive_replays(
    case: &Case,
    consumer: &DorcConsumer,
    env: &RunEnv,
    path: &Path,
    source: &str,
) -> Result<DrivenReplays, String> {
    let input = ReplayInput::new(case_name(path)?, source.to_owned())
        .map_err(|error| format!("{}: {error}", path.display()))?;
    catch_arity_panic(path, || {
        replay_case_with_inputs(case, consumer, env, &[input], |command, context| {
            execute_generic(command, context).map(ReplayResult::bytes)
        })
        .map_err(|error| match error {
            // The raw refusal names neither the flag that supplies a shell nor the decline that
            // needed one.
            RunError::ShellNotConfigured => format!(
                "{}: a replay declined the in-process Dorc driver and would need the generic \
                 executor, which has no shell. Rerun with `--shell=PATH` (e.g. `--shell=/bin/sh`), \
                 or make the replay a shape the driver handles",
                path.display()
            ),
            other => format!("{}: {other}", path.display()),
        })
    })
}

/// Traps a hand-seeded row's arity-mismatch panic (`dorc_aid::arrangement::sentence_words`'s
/// `debug_assert!`) as a typed refusal instead of crashing the process. Hook suppressed for the
/// call only; sound because `drive` is read-only.
fn catch_arity_panic<T>(
    path: &Path,
    drive: impl FnOnce() -> Result<T, String> + std::panic::UnwindSafe,
) -> Result<T, String> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(drive);
    std::panic::set_hook(previous_hook);
    match outcome {
        Ok(result) => result,
        Err(payload) => Err(format!(
            "{}: rendering this case panicked ({}). A hand-seeded arrangement row's `words` list \
             must carry exactly one more word than the values its seat interleaves; fix the named \
             row's word count in crates/aid/src/arrangement_lock.rs to match, then rebuild",
            path.display(),
            panic_message(&payload)
        )),
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_owned()
    } else {
        "no panic message available".to_owned()
    }
}

/// A case with nothing to report costs a `--quiet` caller nothing: the header is written only once
/// its body is, so the untouched majority of the corpus falls silent while every refusal,
/// interpretation, and note survives verbatim.
fn emit_case(out: &mut impl Write, path: &Path, body: &[u8], quiet: bool) -> Result<(), String> {
    if quiet && body.is_empty() {
        return Ok(());
    }
    writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
    out.write_all(body).map_err(|error| error.to_string())
}

fn inspect_cases(
    roots: &Roots,
    gated: &GatedCases,
    publication: &Publication,
    out: &mut impl Write,
) -> Result<Inspected, String> {
    let (env, quiet) = (&publication.env, publication.quiet);
    let (mut consumer, mut refused, mut selected) = (
        DorcConsumer::new().minting(publication.provenance.mint()),
        0usize,
        Vec::new(),
    );
    let ownership = corpus_ownership(&roots.corpus())?;
    let mut losses = Vec::new();
    let mut inspected_cases = Vec::new();
    for (relative_path, path) in &gated.paths {
        let relative_path = relative_path.clone();
        let (case, source) = load_with_text(path)?;
        let head = gated.repository.head_bytes(&relative_path)?;
        let head = std::str::from_utf8(&head)
            .map_err(|_| format!("HEAD case is not UTF-8: {relative_path}"))?;
        let head_case = Case::parse(head)
            .map_err(|error| format!("parse HEAD case {relative_path}: {error}"))?;
        selected.push(relative_path.clone());
        let mut body = Vec::new();
        let mut previews = Vec::new();
        let mut case_refusal = None;
        let results = drive_replays(&case, &consumer, env, path, &source)?;
        let mut inspected_replays = Vec::new();
        for (index, ((block, head_block), routed)) in case
            .replay()
            .blocks()
            .iter()
            .zip(head_case.replay().blocks())
            .zip(results)
            .enumerate()
        {
            let changed_from_head = block.output() != head_block.output();
            // The committed bytes ARE the render's bytes, so an edit compiles against them
            // directly (`28L:rul-editability-is-stamped-never-re-derived`).
            let dirty = block.output().to_owned();
            if let Some(render) = routed.editable_render().cloned() {
                let baseline = consumer
                    .baseline_from_render(&case, render)
                    .map_err(|error| format!("{}: {error}", path.display()))?;
                if changed_from_head {
                    match compile_preview(&baseline, &dirty).and_then(|preview| {
                        refuse_foreign_components(&ownership, path, &preview).map(|()| preview)
                    }) {
                        Ok(preview) => previews.push((index, preview)),
                        Err(error) => case_refusal = Some((index, error, dirty)),
                    }
                }
            } else {
                if block.output() != routed.output() || head_block.output() != routed.output() {
                    case_refusal = Some((
                        index,
                        DorcSectionEditRefusal::Unchanged,
                        "bytes-only replay changed".to_owned(),
                    ));
                }
                // Structure, not a change: a bytes-only replay that actually diverged took the
                // refusal branch above, so quiet loses nothing by dropping the inventory line.
                if !quiet {
                    writeln!(body, "replay: {index} bytes-only")
                        .map_err(|error| error.to_string())?;
                }
            }
            inspected_replays.push((index, block.command().to_owned(), routed));
        }
        if let Some((index, error, dirty)) = case_refusal {
            refused = refused.saturating_add(1);
            write_refusal(&mut body, path, index, &error, &dirty)?;
            emit_case(out, path, &body, quiet)?;
            continue;
        }
        let compiled = emit_previews(
            &mut consumer,
            previews,
            path,
            &relative_path,
            &mut losses,
            &mut body,
        )?;
        let replays = staged_replays(inspected_replays, compiled);
        emit_case(out, path, &body, quiet)?;
        let is_touched = gated.touched.contains(&relative_path);
        inspected_cases.push((relative_path, source, is_touched, replays));
    }
    if refused > 0 {
        return Ok(Inspected::Refused { cases: refused });
    }
    let catalog = std::fs::read_to_string(roots.catalog_lock())
        .map_err(|error| format!("read catalog input: {error}"))?;
    let touched_cases = inspected_cases
        .iter()
        .filter(|(_, _, touched, _)| *touched)
        .map(|(path, _, _, _)| path.clone())
        .collect();
    StagedPublication::new(catalog, selected, touched_cases, inspected_cases)
        .map(|publication| {
            Inspected::Ready(Interpretation {
                publication,
                consumer,
                losses,
            })
        })
        .map_err(|error| error.to_string())
}

/// What a refused case says for itself: the refusal, its class, and the bytes that produced it.
fn write_refusal(
    body: &mut Vec<u8>,
    path: &Path,
    index: usize,
    error: &DorcSectionEditRefusal,
    dirty: &str,
) -> Result<(), String> {
    writeln!(body, "refusal in replay {index}: {}", error.explain(path))
        .and_then(|()| writeln!(body, "class: {error:?}"))
        .and_then(|()| writeln!(body, "baseline: exact renderer provenance"))
        .and_then(|()| writeln!(body, "edited:\n{}", bounded_evidence(dirty)))
        .map_err(|write| write.to_string())
}

/// One case's replays in staging form: an editable one carries its render and whatever the edit
/// compiled to, a bytes-only one carries neither.
fn staged_replays(
    replays: Vec<(usize, String, ReplayResult<SectionKey, SectionVariableId>)>,
    mut compiled: std::collections::BTreeMap<usize, dorc_loom::CompilePreview>,
) -> Vec<StagedReplay> {
    replays
        .into_iter()
        .map(|(index, command, routed)| match routed.editable_render() {
            Some(render) => StagedReplay::editable(
                index,
                command,
                routed.output().to_owned(),
                render,
                &compiled.remove(&index).into_iter().collect::<Vec<_>>(),
            ),
            None => StagedReplay::bytes(index, command, routed.output().to_owned()),
        })
        .collect()
}

/// Emit each preview's register diff, apply it to the mirror (the edited-mirror seam publication is
/// computed from), collect what it gives up, and key the previews by replay index for staging.
fn emit_previews(
    consumer: &mut DorcConsumer,
    previews: Vec<(usize, dorc_loom::CompilePreview)>,
    path: &Path,
    case: &str,
    losses: &mut Vec<HoleLoss>,
    out: &mut impl Write,
) -> Result<std::collections::BTreeMap<usize, dorc_loom::CompilePreview>, String> {
    let mut compiled = std::collections::BTreeMap::new();
    for (index, preview) in previews {
        writeln!(out, "replay: {index}").map_err(|error| error.to_string())?;
        let rendered = render_publish_diff(&preview);
        consumer
            .apply_preview(&preview)
            .map_err(|error| format!("{}: {}", path.display(), error.explain(path)))?;
        losses.extend(preview.sections().iter().flat_map(|section| {
            let key = section.section();
            section.dropped().iter().map(move |hole| HoleLoss {
                case: case.to_owned(),
                section: format!(
                    "{}.{}#{}:{}",
                    key.owner, key.field, key.instance, key.segment
                ),
                hole: hole.name.0.clone(),
                reappears: hole.value_reappears_as_text,
                shared: hole.value_shared_with_another_occurrence,
            })
        }));
        compiled.insert(index, preview);
        writeln!(out, "{rendered}").map_err(|error| error.to_string())?;
    }
    Ok(compiled)
}

fn staging_store(roots: &Roots) -> Result<FsStagingStore, String> {
    FsStagingStore::new(roots.staging_root())
}

/// The receipt may bind only transcript-prose edits. Repository reads are isolated
/// in `GitRepository`; this command owns only selection and inspection orchestration.
fn gate_touched_set(roots: &Roots, cases: &[PathBuf]) -> Result<GatedCases, String> {
    let repository = GitRepository::open_at(roots.base())?;
    let mut paths: Vec<_> = cases
        .iter()
        .map(|path| {
            repository
                .repository_path(path)
                .map(|relative| (relative, path.clone()))
        })
        .collect::<Result<_, _>>()?;
    paths.sort_by(|left, right| left.0.cmp(&right.0));
    let selected = paths.iter().map(|(path, _)| path.clone()).collect();
    let catalog = repository.repository_path(&roots.catalog_lock())?;
    let arrangement = repository.repository_path(&roots.arrangement_lock())?;
    let classification = classify_prose_changes(&repository, selected, &catalog, &arrangement)?;
    Ok(GatedCases {
        repository,
        paths,
        touched: classification.touched().clone(),
        staged: classification.staged().clone(),
    })
}

const MAX_REFUSAL_EVIDENCE: usize = 4096;

fn bounded_evidence(text: &str) -> String {
    if text.len() <= MAX_REFUSAL_EVIDENCE {
        return text.to_owned();
    }
    let mut end = MAX_REFUSAL_EVIDENCE;
    while !text.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    format!("{}\n[truncated]", &text[..end])
}

fn print_variables(
    breadth: Breadth,
    cases: &[PathBuf],
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    let consumer = DorcConsumer::new();
    tracing::info!("{VALUE_SYNTAX_NOTE}");
    if breadth == Breadth::All {
        // "The whole typed payload" is not quite whole, and the gap is invisible from here: a
        // foreign-valued hole renders, so an author sees `{{name}}` in the transcript and then
        // fails to find it in the listing that claims to hold everything.
        tracing::info!(
            "foreign passthrough values are omitted deliberately — they render but cannot be \
             typed; `dorc-loom sections` shows them as computed spans"
        );
    }
    let mut variableless = 0usize;
    for path in cases {
        let case = load(path)?;
        let inventory = consumer
            .vars_inventory(&case, breadth)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if inventory.is_empty() {
            variableless = variableless.saturating_add(1);
            continue;
        }
        write!(out, "{inventory}").map_err(|error| error.to_string())?;
    }
    if variableless > 0 {
        tracing::info!("{variableless} case(s) have no variables at this breadth");
    }
    Ok(ExitCode::SUCCESS)
}

/// `dorc-loom sections CASE...` — per replay, print each editable section's key and its ordered
/// `Text | Variable` fragment series plus the computed spans around it. Drives like `vars`, just
/// without dropping every replay after the first.
fn print_sections(cases: &[PathBuf], out: &mut impl Write) -> Result<ExitCode, String> {
    let consumer = DorcConsumer::new();
    // Which bytes these describe is the one thing a reader can get wrong here.
    tracing::info!(
        "sections of the published baseline — the render your edit is attributed against; what \
         you have typed on disk is what `dorc-loom publish` reads"
    );
    tracing::info!("{VALUE_SYNTAX_NOTE}");
    for path in cases {
        let case = load(path)?;
        let listing = consumer
            .sections_inventory(&case)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        write!(out, "{listing}").map_err(|error| error.to_string())?;
    }
    Ok(ExitCode::SUCCESS)
}

/// `dorc-loom keys` — the closed frontmatter vocabulary, and which gate gives each key effect.
///
/// A listing rather than a refusal because the vocabulary was previously discoverable only by
/// declaring something outside it and reading what came back, which is a poor way to learn that
/// `code:` and `arrangement:` are alternatives at all.
fn print_keys(out: &mut impl Write) -> Result<ExitCode, String> {
    writeln!(out, "{}\n", dorc_loom::DEFINING_KEYS_NOTE).map_err(|error| error.to_string())?;
    let width = dorc_loom::FRONTMATTER_KEYS
        .iter()
        .map(|key| key.name.len())
        .max()
        .unwrap_or_default();
    for key in &dorc_loom::FRONTMATTER_KEYS {
        writeln!(out, "  {:width$}  {}", key.name, key.read_by).map_err(|e| e.to_string())?;
    }
    Ok(ExitCode::SUCCESS)
}

fn load(path: &Path) -> Result<Case, String> {
    read_case(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn load_with_text(path: &Path) -> Result<(Case, String), String> {
    let source = read_case_text(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let case = Case::parse(&source).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok((case, source))
}

fn validate_case_inputs(cases: &[PathBuf]) -> Result<(), String> {
    for path in cases {
        let _ = load_with_text(path)?;
    }
    Ok(())
}

fn case_name(path: &Path) -> Result<String, String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("{}: case file has no UTF-8 filename", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the grammar made of one command line, without the world it also resolved — the
    /// question every test below is asking.
    fn parse_argv(words: &[String]) -> Result<Command, String> {
        super::parse_argv(words).map(|invoked| invoked.command)
    }

    /// The bare word `help` is `add-register`'s own second positional, so a scan that read it
    /// anywhere made the verb's ONLY legal invocation indistinguishable from a help request — it
    /// printed the page and exited 0 having minted nothing, for as long as the command existed.
    #[test]
    fn a_verbs_own_positional_help_is_not_a_help_request() {
        let argv = |args: &[&str]| args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cmdsub-command.loom");
        let spelled = path.to_str().expect("the fixture path is UTF-8");

        match parse_argv(&argv(&["add-register", spelled, "help"])) {
            Ok(Command::AddRegister { case, register }) => {
                assert_eq!(case, path);
                assert_eq!(register, "help");
            }
            _ => panic!("`add-register CASE help` must parse as the verb it spells"),
        }

        // The flag spelling still asks the verb, from that same trailing position.
        assert!(
            matches!(
                parse_argv(&argv(&["add-register", spelled, "--help"])),
                Ok(Command::Help(page)) if page == usage::usage_for("add-register")
            ),
            "`--help` after a verb still asks the verb"
        );
        assert!(matches!(
            parse_argv(&argv(&["help"])),
            Ok(Command::Help(usage::USAGE))
        ));
    }

    fn env_of(pairs: &[(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        let pairs: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |name| {
            pairs
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.clone())
        }
    }

    /// Both markers answer; an empty one does not (the hook's self-test neutralises by emptying
    /// these same variables); the escape outranks every marker.
    #[test]
    fn an_agent_session_is_recognized_by_its_markers_and_overridden_by_the_escape() {
        assert!(!looks_like_an_agent(&env_of(&[])));
        assert!(!looks_like_an_agent(&env_of(&[("CLAUDECODE", "")])));
        for marker in AGENT_MARKERS {
            assert!(looks_like_an_agent(&env_of(&[(marker, "1")])), "{marker}");
        }
        assert!(!looks_like_an_agent(&env_of(&[
            ("CLAUDECODE", "1"),
            ("DORC_HUMAN_COMMIT", "1"),
        ])));
        assert!(looks_like_an_agent(&env_of(&[
            ("CLAUDECODE", "1"),
            ("DORC_HUMAN_COMMIT", "0"),
        ])));
    }

    /// `--human` is the ONE claim this tool takes on trust, so an environment that contradicts it
    /// wins; the refusal names the escape, since a person hitting it has no other way past.
    #[test]
    fn the_human_mint_refuses_only_from_an_agent_session() {
        let refusal = refuse_human_mint_from_an_agent(Provenance::Human, true)
            .expect_err("an agent session may not claim a human mint");
        assert!(refusal.contains(HUMAN_ESCAPE), "{refusal}");
        assert!(refuse_human_mint_from_an_agent(Provenance::Human, false).is_ok());
        assert!(refuse_human_mint_from_an_agent(Provenance::Default, true).is_ok());
        assert!(refuse_human_mint_from_an_agent(Provenance::Slop, true).is_ok());
    }

    /// Both demotion branches, and the wording law over the agent one: a NOTICE about a
    /// consequence may not read as a failure, and must say that nothing is owed.
    #[test]
    fn a_demotion_notifies_an_agent_and_stops_a_person() {
        let demoted = vec!["site-unresolvable".to_owned()];
        let note = report_demotions(&demoted, Provenance::Default, true)
            .expect("an agent proceeds")
            .expect("an agent is told what moved");
        assert!(note.contains("site-unresolvable"), "{note}");
        assert!(note.contains("No action is necessary."), "{note}");
        assert!(note.contains("expected outcome"), "{note}");
        for forbidden in ["error", "refus", "fail"] {
            assert!(
                !note.to_lowercase().contains(forbidden),
                "the agent notice must not read as a failure ({forbidden}): {note}"
            );
        }

        let refusal = report_demotions(&demoted, Provenance::Default, false)
            .expect_err("a person is stopped");
        assert!(refusal.contains(HUMAN), "{refusal}");
        assert!(refusal.contains(SLOP), "{refusal}");

        assert!(
            report_demotions(&demoted, Provenance::Slop, false)
                .expect("a deliberate demotion proceeds")
                .is_some(),
            "a deliberate demotion still says what moved"
        );
        assert!(
            report_demotions(&[], Provenance::Default, false)
                .expect("no demotion")
                .is_none(),
            "nothing to say"
        );
    }

    /// A MARKING decision about the registers this publishes; the two flags together are a
    /// contradiction rather than a last-one-wins.
    #[test]
    fn the_provenance_flags_exclude_each_other() {
        let argv = |args: &[&str]| args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/cmdsub-command.loom")
            .to_str()
            .expect("the fixture path is UTF-8")
            .to_owned();

        for (flag, expected) in [
            (Some(HUMAN), Provenance::Human),
            (Some(SLOP), Provenance::Slop),
            (None, Provenance::Default),
        ] {
            let mut words = vec!["publish", &fixture];
            words.extend(flag);
            assert!(
                matches!(
                    parse_argv(&argv(&words)),
                    Ok(Command::Publish(Publication { provenance, .. })) if provenance == expected
                ),
                "{words:?}"
            );
        }
        assert!(
            parse_argv(&argv(&["publish", &fixture, HUMAN, SLOP]))
                .is_err_and(|error| error.contains("opposite things"))
        );
    }

    /// `publish` MUTATES, so it never takes the whole corpus by omission: a bare invocation is a
    /// misuse that lands on the verb's own page, and `--all` is the spelled-out whole-corpus target.
    #[test]
    fn a_bare_publish_lands_on_its_own_usage_page() {
        let argv = |args: &[&str]| args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        let Err(refusal) = parse_argv(&argv(&["publish"])) else {
            panic!("a bare publish must refuse")
        };
        assert!(refusal.contains(usage::usage_for("publish")), "{refusal}");
        assert!(refusal.contains("--all"), "{refusal}");

        assert!(matches!(
            parse_argv(&argv(&["publish", "--all"])),
            Ok(Command::Publish(_))
        ));
        // The read-only verbs keep bare-means-everything; only the mutating one asks.
        assert!(matches!(
            parse_argv(&argv(&["vars"])),
            Ok(Command::Vars { .. })
        ));
    }

    /// Under-naming is the failure that matters: a rewritten case's staged bytes are the author's
    /// own pre-publish text, so a bare `git commit` would take those and drop the publication.
    #[test]
    fn only_a_rewritten_staged_case_is_told_to_restage() {
        let staged =
            std::collections::BTreeSet::from(["kept.loom".to_owned(), "rewritten.loom".to_owned()]);
        let rewritten = std::collections::BTreeSet::from(["rewritten.loom".to_owned()]);
        let notes = staged_case_notes(&staged, &rewritten).join("\n");
        assert!(
            notes.contains("rewritten.loom was staged and has been rewritten"),
            "{notes}"
        );
        assert!(notes.contains("kept.loom is staged;"), "{notes}");
        assert!(!notes.contains("kept.loom was staged"), "{notes}");
    }

    /// A run that did nothing exits 0 either way, so the only thing separating "nothing to do" from
    /// "wrong worktree, wrong file" is this line. It names the case when there is one to name,
    /// because a reader who mistyped a path needs to see which path the tool actually read.
    #[test]
    fn a_run_that_moved_nothing_says_so_and_names_what_it_looked_at() {
        let selected = |paths: &[&str]| -> Vec<SelectedCase> {
            paths
                .iter()
                .map(|path| ((*path).to_owned(), PathBuf::from(path)))
                .collect()
        };

        let one = selected(&["crates/aid/tests/whylog-absent.loom"]);
        let note = nothing_moved_note(true, &one).expect("a no-op run says so");
        assert!(note.contains("this publish changed nothing"), "{note}");
        assert!(note.contains("whylog-absent.loom"), "{note}");
        assert!(note.contains("worktree"), "{note}");

        let many = selected(&["a.loom", "b.loom", "c.loom"]);
        let note = nothing_moved_note(true, &many).expect("a no-op run says so");
        assert!(note.contains("3 selected cases"), "{note}");

        assert_eq!(nothing_moved_note(false, &many), None);
    }

    /// The census is ONE text, shared byte-for-byte by the run that refuses and the `--verbatim`
    /// that applies (`30C:rul-flag-names-the-act-not-the-history` — the flag names the act, so the
    /// accounting beside it may not change its story between the two). It names every hole, and the
    /// two reasons compose on one hole rather than one winning.
    #[test]
    fn the_hole_loss_census_names_every_hole_and_all_of_its_reasons() {
        let loss = |hole: &str, reappears, shared| HoleLoss {
            case: String::from("crates/aid/tests/x.loom"),
            section: String::from("x.message#0:0"),
            hole: String::from(hole),
            reappears,
            shared,
        };
        let detail = hole_loss_detail(&[
            loss("command", true, false),
            loss("to", false, true),
            loss("both", true, true),
            loss("plain", false, false),
        ])
        .expect("four losses are a census");

        assert!(detail.contains("gives up 4 hole(s)"), "{detail}");
        for hole in ["{{command}}", "{{to}}", "{{both}}", "{{plain}}"] {
            assert!(detail.contains(hole), "{hole} unnamed: {detail}");
        }
        assert!(detail.contains("still there as literal text"), "{detail}");
        assert!(detail.contains("the reading I picked"), "{detail}");
        assert!(
            detail.contains("frozen at whatever this render happened to say; and another hole"),
            "both reasons compose on one hole: {detail}"
        );
        assert_eq!(hole_loss_detail(&[]), None, "a clean publish says nothing");
    }

    /// The whole gate. A clean run writes whether or not `--verbatim` was typed — there is nothing
    /// to confirm, and demanding a confirmation anyway would make the flag a statement about a
    /// prior interaction, which is exactly what it is not.
    #[test]
    fn only_a_hole_loss_needs_confirming() {
        assert_eq!(disposition(false, false), Disposition::Write);
        assert_eq!(disposition(false, true), Disposition::Write);
        assert_eq!(disposition(true, false), Disposition::Refuse);
        assert_eq!(disposition(true, true), Disposition::Confirm);
    }

    /// Quiet may drop a header, never a report — the corpus is ~50 cases and all but the edited one
    /// have nothing to say, but a refusal buried in that listing is the whole reason to look.
    #[test]
    fn quiet_drops_only_a_case_that_said_nothing() {
        let mut silent = Vec::new();
        emit_case(&mut silent, Path::new("silent.loom"), b"", true).expect("emit");
        assert!(silent.is_empty());

        let mut speaking = Vec::new();
        emit_case(&mut speaking, Path::new("loud.loom"), b"replay: 0\n", true).expect("emit");
        let speaking = String::from_utf8(speaking).expect("notes are utf-8");
        assert!(speaking.contains("case: "), "{speaking}");
        assert!(speaking.contains("replay: 0"), "{speaking}");

        let mut verbose = Vec::new();
        emit_case(&mut verbose, Path::new("silent.loom"), b"", false).expect("emit");
        assert!(!verbose.is_empty());
    }

    /// The incident this exists for, reproduced on the corpus's own shared component: eleven
    /// invocation-error cases render the usage synopsis, `cli-no-book-given` homes it, and a
    /// reword through that home moves every other one's committed bytes without republishing any
    /// of them.
    #[test]
    fn a_reworded_component_names_the_cases_it_stales() {
        let read = |slug: &str| {
            let text = std::fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../aid/tests/{slug}.loom")),
            )
            .expect("read the case");
            Case::parse(&text).expect("case parses")
        };
        let home = read("cli-no-book-given");
        let borrower = read("cli-unknown-flag");

        let mut consumer = DorcConsumer::new();
        let words = consumer
            .arrangements()
            .iter()
            .find(|entry| entry.slug == "cli-usage-synopsis")
            .and_then(|entry| entry.words.as_ref())
            .map(|tier| tier.text().clone())
            .expect("the synopsis component has words");
        // Same arity, different bytes: an arity change is a different failure with its own refusal.
        let mut reworded = words.clone();
        reworded[0] = format!("{}, really", words[0]);
        consumer.set_arrangement_words(
            "cli-usage-synopsis",
            Some(dorc_aid::prose::ProseTier::Slop(reworded)),
        );

        let corpus = std::collections::BTreeMap::from([
            ("cli-no-book-given".to_owned(), home.clone()),
            ("cli-unknown-flag".to_owned(), borrower),
        ]);
        let affected = std::collections::BTreeMap::from([("cli-no-book-given".to_owned(), home)]);
        let note = stale_siblings_note(
            &consumer,
            &corpus,
            &std::collections::BTreeMap::new(),
            &affected,
        )
        .expect("the reword stales a sibling");
        assert!(note.contains("cli-unknown-flag"), "{note}");
        assert!(
            !note.contains("cli-no-book-given"),
            "the republished case is not stale: {note}"
        );
        assert!(
            note.contains("test:looms"),
            "the note names where it surfaces: {note}"
        );
    }

    /// The other half: an unedited consumer stales nothing, so an ordinary publish stays quiet.
    #[test]
    fn an_unreworded_promote_names_nothing() {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests/cli-unknown-flag.loom"),
        )
        .expect("read the case");
        let case = Case::parse(&text).expect("case parses");
        let corpus = std::collections::BTreeMap::from([("cli-unknown-flag".to_owned(), case)]);
        let note = stale_siblings_note(
            &DorcConsumer::new(),
            &corpus,
            &std::collections::BTreeMap::new(),
            &std::collections::BTreeMap::new(),
        );
        assert_eq!(note, None);
    }

    /// A hand-seeded row's arity mismatch panics deep inside the shared renderer
    /// (`dorc_aid::arrangement::sentence_words`'s own `debug_assert!`) the first time some case's
    /// render reaches it — a whole-PAGE entry's arity is always "exactly one word", so seeding a
    /// second one reproduces the wiring defect without needing a value-bearing seat. This proves
    /// dorc-loom's own driving boundary catches that panic instead of taking the whole process
    /// down, and reports the row, the diagnosis, and the fix.
    #[test]
    fn a_hand_seeded_arity_mismatch_refuses_instead_of_crashing_the_process() {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests/cli-help-page.loom"),
        )
        .expect("read fixture case");
        let case = Case::parse(&text).expect("case parses");
        let mut consumer = DorcConsumer::new();
        consumer.set_arrangement_words(
            "cli-help-page",
            Some(dorc_aid::prose::ProseTier::Slop(vec![
                "one word".to_owned(),
                "an extra word a page never takes".to_owned(),
            ])),
        );
        let error = drive_replays(
            &case,
            &consumer,
            &RunEnv::new(),
            Path::new("crates/aid/tests/cli-help-page.loom"),
            &text,
        )
        .expect_err("a bad-arity row must refuse, not panic");
        assert!(error.contains("cli-help-page"), "{error}");
        assert!(error.contains("panicked"), "{error}");
        assert!(error.contains("arrangement_lock.rs"), "{error}");
    }
}
