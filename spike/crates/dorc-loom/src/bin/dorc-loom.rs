//! `dorc-loom` is the read-only transcript-template inspection command.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use dorc_aid::prose::Mint;
use dorc_loom::invocation::{Breadth, PublishArgs, THIS, Verb};
use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, FsReceiptStore, GitRepository, InspectedCompilation,
    InspectedReplay, Repository, SectionKey, SectionVariableId, build_publication,
    classify_prose_changes, compile_preview, compile_receipt, corpus_ownership,
    load_arrangement_corpus, load_corpus_by_slug, promote_receipt, refuse_foreign_components,
    render_compile_preview, replay_case_with_inputs,
};
use errorloom::{
    Case, CaseRenderer, ReplayInput, ReplayResult, RunEnv, RunError, execute_generic, read_case,
    read_case_text,
};

const USAGE: &str = "usage: dorc-loom [--this] <compile|promote [--quiet] [--accept-metadata] [--human|--slop] [--shell=PATH] [--path=DIR]... [CASE...]|vars [--used|--all] [CASE...]|scaffold SLUG|add-register CASE help|sections [CASE...]|keys>\n       a CASE is a bare slug (`whylog-unwritten`), a filename, or a path; an omitted list means every crates/aid/tests/*.loom\n       --this comes BEFORE the verb and names the case a replay line is running inside; it resolves only there, never from a terminal\n       edit a sentence in a case's transcript, then compile and promote it; type {{name}} to insert or move one of its values\n       `dorc-loom <subcommand> --help` explains one verb; this page is only the index";

/// Each verb's own page — what it does, what its flags mean, and the command that follows it.
///
/// The index above answers "which verb", and answers nothing else: it cannot say what a receipt is,
/// why `promote` wants the same CASE list `compile` got, or which of two spellings of a flag is
/// which. A reader who has already chosen a verb and typed `--help` is asking the VERB, so that is
/// what they get (`28L:rul-refusals-name-the-next-command`, in its non-refusing register).
fn usage_for(verb: &str) -> &'static str {
    match verb {
        "compile" => COMPILE_USAGE,
        "promote" => PROMOTE_USAGE,
        "vars" => VARS_USAGE,
        "sections" => SECTIONS_USAGE,
        "scaffold" => SCAFFOLD_USAGE,
        "add-register" => ADD_REGISTER_USAGE,
        "keys" => KEYS_USAGE,
        _ => USAGE,
    }
}

/// The verbs `usage_for` has a page for — also what makes `dorc-loom <verb> --help` route to it.
const VERBS: [&str; 7] = [
    "compile",
    "promote",
    "vars",
    "sections",
    "scaffold",
    "add-register",
    "keys",
];

const COMPILE_USAGE: &str =
    "usage: dorc-loom compile [--quiet] [--shell=PATH] [--path=DIR]... [CASE...]
  Drive every selected case's replays, compile the prose you edited back into template form, print
  what it understood, and record the whole inspection as a receipt under spike/target. Writes no
  source file. Bare, it takes the whole corpus and narrows to the prose-changed cases itself.
  --quiet     drop the header of every case that has nothing to report
  --shell=P   lend the generic executor a shell, for a replay the in-process driver declines
  --path=D    prepend a directory to the replay PATH (repeatable)
  next: dorc-loom promote <the same CASE list> -- the receipt refuses a different one";

const PROMOTE_USAGE: &str = "usage: dorc-loom promote [--quiet] [--accept-metadata] [--human|--slop] [--shell=PATH] [--path=DIR]... [CASE...]
  Verify against the compile receipt, then publish: both generated locks
  (crates/aid/src/catalog_lock.rs and arrangement_lock.rs) plus every affected case. In-process
  renders only -- no binary is run and no fixture is executed. Every byte and both fixpoints are
  computed before the first write, so a validation failure leaves the tree byte-identical. Nothing
  is staged or committed; the diff is yours -- `git diff --word-diff` is how prose reads.
  --accept-metadata  acknowledge that a case's when-fires / when-used / why REPLACES the committed
                     registry entry; without it a metadata change refuses before any write
  --human     mark every register this publishes as written by a person. Refuses in a session that
              announces itself as an agent; DORC_HUMAN_COMMIT=1 says a person is at the keyboard.
              Unflagged, a register is marked slop, whoever is driving.
  --slop      yes, re-mark a human-written register as slop. Unflagged, that refuses for a person
              (the forgotten --human) and proceeds with a note for an agent.
  the other flags are compile's, and the CASE list must be the one compile saw
  next: mise run test -- a promote republishes shared locks, so its blast radius is wider than the
        case in front of you";

const VARS_USAGE: &str = "usage: dorc-loom [--this] vars [--used|--all] [CASE...]
  Print each case's named template variables and their currently-rendered values, driven from the
  same render an edit compiles against. A case with no variables prints no row at all; stderr
  carries how many there were.
  --used   only the variables some rendered section actually consumes (the default)
  --all    the whole typed payload, including values no sentence spends yet
  --this   the case this invocation is running inside -- a replay line's spelling, so a case never
           has to name itself. It comes before the verb and resolves nowhere else.
  next: type {{name}} into a sentence in the transcript to insert or move that value, then
        dorc-loom compile";

const SECTIONS_USAGE: &str = "usage: dorc-loom [--this] sections [CASE...]
  Per replay, print each editable section's key and its ordered Text|Variable fragment series,
  alongside the computed (immutable) spans around it. The answer to `which bytes in this transcript
  are mine to edit` -- read-only, and driven from the published baseline rather than from your
  worktree.
  --this   the case this invocation is running inside -- a replay line's spelling, so a case never
           has to name itself. It comes before the verb and resolves nowhere else.
  next: edit an editable section in the case, then dorc-loom compile";

const SCAFFOLD_USAGE: &str = "usage: dorc-loom scaffold SLUG
  Write the empty defining-case skeleton for a freshly-minted code slug to
  crates/aid/tests/SLUG.loom. Never overwrites an authored case, and never writes prose: the message
  register stays unwritten, and everything the skeleton omits is deliberately red until a world that
  really fires the code is authored.
  next: the two-step follow-up the command prints";

const KEYS_USAGE: &str = "usage: dorc-loom keys
  Print the closed frontmatter-key vocabulary a case may declare, and which gate reads each key --
  including which of `code:` and `arrangement:` a case wants. Takes no arguments and reads no case.
  next: declare the key in the case's frontmatter; anything outside this set is refused by the
        runners, because a key no gate reads is an assertion you only believe you made";

const ADD_REGISTER_USAGE: &str = "usage: dorc-loom add-register CASE help
  Mint a code's `help` register, so the ordinary transcript loop can fill it. The CASE spelling is
  every other verb's -- a bare slug, a filename, or a path. `help` is the only addable register --
  `message` exists on every code already. Refuses when the case carries an unpromoted prose edit,
  or when the register is already there.
  next: rebuild, overtype the printed [unwritten: SLUG.help] placeholder, then compile and promote";

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
/// corpus, ONE case took every `compile` and `promote` down with it — an overflow, not a diagnostic.
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
            let _ = writeln!(io::stderr(), "dorc-loom: {message}");
            ExitCode::from(2)
        }
    }
}

enum Command {
    Compile {
        cases: Vec<PathBuf>,
        env: RunEnv,
        quiet: bool,
    },
    Promote {
        cases: Vec<PathBuf>,
        env: RunEnv,
        quiet: bool,
        accept_metadata: bool,
        provenance: Provenance,
    },
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
    Help {
        verb: Option<String>,
    },
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
    Ready(InspectedCompilation, DorcConsumer),
    Refused { cases: usize },
}

struct GatedCases {
    repository: GitRepository,
    paths: Vec<SelectedCase>,
    touched: std::collections::BTreeSet<String>,
    staged: std::collections::BTreeSet<String>,
}

fn run() -> Result<ExitCode, String> {
    let command = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match command {
        Command::Compile { cases, env, quiet } => compile_cases(&cases, &env, quiet, &mut out),
        Command::Promote {
            cases,
            env,
            quiet,
            accept_metadata,
            provenance,
        } => promote_cases(&cases, &env, quiet, accept_metadata, provenance, &mut out),
        Command::Vars { breadth, cases } => print_variables(breadth, &cases, &mut out),
        Command::Scaffold { slug } => scaffold_case(&slug),
        Command::AddRegister { case, register } => add_register(&case, &register),
        Command::Sections { cases } => print_sections(&cases, &mut out),
        Command::Keys => print_keys(&mut out),
        Command::Help { verb } => writeln!(
            out,
            "{}",
            verb.as_deref().map_or(USAGE, |verb| usage_for(verb))
        )
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
fn resolve_case(arg: &str) -> Result<PathBuf, String> {
    let slug = arg.strip_suffix(".loom").unwrap_or(arg);
    let tried = [
        cases_dir().join(format!("{slug}.loom")),
        PathBuf::from(arg),
        spike_dir()?.join(arg),
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
        cases_dir().display()
    ))
}

/// The verb this invocation is ABOUT, for the help and misuse pages: the leading verb, or the one
/// after a leading `help`. `None` means the reader has not chosen one and wants the index.
///
/// Global flags are skipped first, because `--this` legitimately precedes the verb: a reader who
/// typed `dorc-loom --this vars --help` has chosen `vars`.
fn chosen_verb(argv: &[String]) -> Option<&str> {
    let mut rest = argv.iter().skip_while(|arg| *arg == THIS);
    let leading = rest.next().map(String::as_str)?;
    if leading == "help" {
        return rest
            .next()
            .map(String::as_str)
            .filter(|next| VERBS.contains(next));
    }
    VERBS.contains(&leading).then_some(leading)
}

fn parse_args() -> Result<Command, String> {
    parse_argv(&std::env::args().skip(1).collect::<Vec<_>>())
}

/// Split from [`parse_args`] so the grammar is reachable from a test without a process.
///
/// `--help`/`-h` are FLAGS and are read anywhere, because a reader who has already typed a verb
/// asks THE VERB for help. The bare word `help` is a VERB, and is read only in verb position: a
/// subcommand's own positional argument is never a global request. That distinction is
/// load-bearing rather than tidy — `add-register CASE help` ends in the literal token `help`, so
/// while the scan took the bare word anywhere, the verb's only legal invocation was
/// indistinguishable from a help request: usage page, exit 0, nothing minted.
fn parse_argv(words: &[String]) -> Result<Command, String> {
    let asks_for_help = words
        .iter()
        .any(|word| matches!(word.as_str(), "--help" | "-h"))
        || words.first().is_some_and(|word| word == "help");
    if asks_for_help {
        return Ok(Command::Help {
            verb: chosen_verb(words).map(str::to_owned),
        });
    }
    let page = || usage_for(chosen_verb(words).unwrap_or_default());
    let invocation = dorc_loom::invocation::parse(
        std::iter::once("dorc-loom".to_owned()).chain(words.iter().cloned()),
    )
    .map_err(|refusal| format!("{refusal}\n{}", page()))?;
    // A terminal is never inside a case, so this binary is the one seat `--this` can never resolve
    // at. Falling back to the bare form's every-case meaning would dump the whole corpus at
    // somebody who asked for exactly one (`30C:rul-this-is-a-global-flag`).
    if invocation.this {
        return Err(format!(
            "{THIS} names the case this invocation is running inside, so it resolves only where \
             the command is a replay line in a case -- a terminal is not inside one. Name the \
             case: `dorc-loom {} <slug>`\n{}",
            invocation.verb_name(),
            page()
        ));
    }
    match invocation.verb {
        Verb::Compile(args) => {
            if args.accept_metadata {
                return Err(format!(
                    "{ACCEPT_METADATA} is a promote-time acknowledgement; compile writes nothing \
                     to acknowledge\n{COMPILE_USAGE}"
                ));
            }
            if provenance_of(&args)? != Provenance::Default {
                return Err(format!(
                    "{HUMAN}/{SLOP} decide how a published register is MARKED; compile publishes \
                     nothing, so it marks nothing. Pass it to the promote instead\n{COMPILE_USAGE}"
                ));
            }
            Ok(Command::Compile {
                cases: resolve_cases(&args.cases)?,
                env: run_env(&args)?,
                quiet: args.quiet,
            })
        }
        Verb::Promote(args) => Ok(Command::Promote {
            cases: resolve_cases(&args.cases)?,
            env: run_env(&args)?,
            quiet: args.quiet,
            accept_metadata: args.accept_metadata,
            provenance: provenance_of(&args)?,
        }),
        Verb::Vars(args) => Ok(Command::Vars {
            breadth: args.breadth(),
            cases: resolve_cases(&args.cases)?,
        }),
        Verb::Scaffold { slug } => Ok(Command::Scaffold { slug }),
        Verb::AddRegister { case, register } => Ok(Command::AddRegister {
            case: resolve_case(&case)?,
            register,
        }),
        Verb::Sections(args) => Ok(Command::Sections {
            cases: resolve_cases(&args.cases)?,
        }),
        Verb::Keys => Ok(Command::Keys),
    }
}

/// The `--human`/`--slop` pair, which say opposite things about the same registers.
fn provenance_of(args: &PublishArgs) -> Result<Provenance, String> {
    match (args.human, args.slop) {
        (true, true) => Err(format!(
            "{HUMAN} and {SLOP} say opposite things about the same registers; pass one\n{PROMOTE_USAGE}"
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

/// Resolve a verb's CASE list, defaulting to the whole collection.
///
/// The default is the WHOLE corpus rather than an error because `compile` and `promote` must see
/// the same list for the receipt to match, and the tool already narrows to the prose-changed subset
/// itself (`gate_touched_set`). So "all of them" reads as "publish what I edited", not as a
/// blunderbuss, and spares every caller from keeping two lists in sync.
fn resolve_cases(cases: &[String]) -> Result<Vec<PathBuf>, String> {
    if cases.is_empty() {
        return corpus_cases();
    }
    cases.iter().map(|case| resolve_case(case)).collect()
}

/// `dorc-loom add-register CASE help` — mint a code's help register so the ordinary transcript loop
/// can fill it (`28L:rul-help-affordance-is-scaffold`).
///
/// The register is a CATALOG fact, so this publishes through the same generator promote uses: the
/// lock gains `HelpRegister::Unwritten` and the case's transcript grows the
/// `= help: [unwritten: <slug>.help]` line the author then overtypes. Nothing here writes prose.
fn add_register(path: &Path, register: &str) -> Result<ExitCode, String> {
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
    let gated = gate_touched_set(std::slice::from_ref(&path.to_path_buf()))?;
    if !gated.touched.is_empty() {
        return Err(format!(
            "{} has a prose edit that is not promoted yet, and adding a register rewrites the \
             case; run `dorc-loom compile {0}` then `dorc-loom promote {0}` first",
            path.display()
        ));
    }
    let mut consumer = DorcConsumer::new();
    consumer.seed_help_register(&slug).map_err(|refusal| match refusal {
        dorc_loom::SeedRefusal::MissingCode(slug) => format!(
            "no catalog row for `{slug}`; promote its defining case first: `dorc-loom promote {}`",
            path.display()
        ),
        dorc_loom::SeedRefusal::AlreadyPresent(slug) => format!(
            "`{slug}` already has a help register; edit its `= help:` line in {}, then \
             `dorc-loom compile {0}` and `dorc-loom promote {0}`",
            path.display()
        ),
    })?;
    publish(
        &consumer,
        &std::collections::BTreeMap::from([(slug.clone(), case)]),
    )?;
    tracing::info!(
        "next: rebuild, then overtype `[unwritten: {slug}.help]` in {} with the remediation words",
        path.display()
    );
    tracing::info!(
        "then: dorc-loom compile {0} && dorc-loom promote {0}",
        path.display()
    );
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
fn scaffold_case(slug: &str) -> Result<ExitCode, String> {
    if slug.is_empty()
        || !slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(format!(
            "slug {slug:?} is not a code slug (lowercase letters, digits, and hyphens)"
        ));
    }
    let path = cases_dir().join(format!("{slug}.loom"));
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
        "then: dorc-loom promote {} (orchestrator-only, on a freshly verified binary)",
        path.display()
    );
    Ok(ExitCode::SUCCESS)
}

/// The one acknowledgement `promote` takes: yes, replace the committed metadata.
const ACCEPT_METADATA: &str = "--accept-metadata";
const HUMAN: &str = "--human";
const SLOP: &str = "--slop";

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

/// Refuse a promote that would rewrite committed `when-fires`/`when-used`/`why` unless the caller
/// said so (`28L:fnd-case-frontmatter-overwrites-lock-metadata`).
///
/// Before any write, not after: the suite gate that also holds this property only fires once the
/// files are already rewritten, which turns an accident into a revert ceremony. Both texts are
/// shown because the reader is holding the case and cannot see the entry.
fn refuse_metadata_drift(accepted: bool) -> Result<(), String> {
    let cases_dir = cases_dir();
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
        "this promote would replace committed metadata that no prose edit asked it to. One slug's \
         several registry entries all read one case's frontmatter, so an unnoticed edit reaches \
         every one of them at once.{} \nOmit the key from the case to keep the committed words, \
         or say you mean it: add {ACCEPT_METADATA} to this promote.",
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
fn corpus_cases() -> Result<Vec<PathBuf>, String> {
    let dir = cases_dir();
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

fn compile_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    quiet: bool,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(cases)?;
    let gated = gate_touched_set(cases)?;
    let total = gated.paths.len();
    let (inspection, _consumer) = match inspect_cases(&gated, env, quiet, Mint::Slop, out)? {
        Inspected::Ready(inspection, consumer) => (inspection, consumer),
        Inspected::Refused { cases } => {
            tracing::info!("{total} cases, {cases} refused");
            return Ok(ExitCode::from(1));
        }
    };
    let store = receipt_store()?;
    let outcome = compile_receipt(&store, &inspection)?;
    if matches!(outcome, dorc_loom::ReceiptWriteOutcome::CleanupPending) {
        tracing::warn!(
            "receipt published; retained backup requires deliberate resolution; subsequent writes refuse"
        );
    }
    warn_each(staged_case_notes(
        &gated.staged,
        &std::collections::BTreeSet::new(),
    ));
    // A compile changes no tracked file, so without this its only trace is a receipt under
    // `target/` that nothing announces.
    tracing::info!(
        "{total} cases, {} touched, receipt {}",
        gated.touched.len(),
        store.path().display()
    );
    warn_each(nothing_moved_note(
        gated.touched.is_empty(),
        "compile",
        &gated.paths,
    ));
    Ok(ExitCode::SUCCESS)
}

/// The warning a reader who lost an hour to a silent run needed (`30C` item 6).
///
/// Both verbs can do exactly nothing and exit 0 — the wrong worktree, the wrong file, an edit
/// already promoted — and the ordinary summary line reads the same either way, because "0 touched"
/// is a number in a sentence rather than an answer to the question the reader is holding.
/// The one seat that turns a note-producing function's answer into stderr lines.
fn warn_each(notes: impl IntoIterator<Item = String>) {
    for note in notes {
        tracing::warn!("{note}");
    }
}

///
/// Returns the note rather than emitting it, so its wording stays testable without a subscriber.
fn nothing_moved_note(
    nothing_moved: bool,
    verb: &str,
    selected: &[SelectedCase],
) -> Option<String> {
    if !nothing_moved {
        return None;
    }
    let scope = match selected {
        [(only, _)] => format!("`{only}`"),
        many => format!("{} selected cases", many.len()),
    };
    Some(format!(
        "this {verb} changed nothing: {scope} carry no unpromoted prose edit against HEAD. If you \
         expected one, check that you edited the transcript in THIS worktree and that the case you \
         edited is the one you named."
    ))
}

fn promote_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    quiet: bool,
    accept_metadata: bool,
    provenance: Provenance,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(cases)?;
    let agent = looks_like_an_agent(&process_env);
    refuse_human_mint_from_an_agent(provenance, agent)?;
    refuse_metadata_drift(accept_metadata)?;
    let gated = gate_touched_set(cases)?;
    let Inspected::Ready(inspection, consumer) =
        inspect_cases(&gated, env, quiet, provenance.mint(), out)?
    else {
        return Ok(ExitCode::from(1));
    };
    if let Some(note) = report_demotions(consumer.demoted(), provenance, agent)? {
        tracing::info!("{note}");
    }
    promote_receipt(&receipt_store()?, &inspection)?;
    let affected = touched_cases(&gated)?;
    let before = staged_bytes(&gated)?;
    let wrote = publish(&consumer, &affected)?;
    warn_each(staged_case_notes(
        &gated.staged,
        &rewritten_staged(&gated, &before)?,
    ));
    warn_each(nothing_moved_note(!wrote, "promote", &gated.paths));
    Ok(ExitCode::SUCCESS)
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

/// What to say when this promote re-marks a human-written register as slop. An AGENT is told the
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
            "this promote re-marks {count} register(s) as slop that were marked \
             human-written: {listed}\n      Reworking prose through the loom is what re-marks it, \
             so this is the expected outcome of the edit.\n      No action is necessary."
        )));
    }
    Err(format!(
        "this promote would re-mark {count} human-written register(s) as slop: {listed}\nRe-run \
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
/// on their own pre-promote bytes.
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
/// promote do something".
fn publish(
    consumer: &DorcConsumer,
    affected: &std::collections::BTreeMap<String, Case>,
) -> Result<bool, String> {
    let cases_dir = cases_dir();
    let corpus = load_corpus_by_slug(&cases_dir)?;
    let arrangements = load_arrangement_corpus(&cases_dir)?;
    let publication = build_publication(consumer, &corpus, &arrangements, affected)?;

    let mut wrote = false;
    for (path, bytes) in [
        (catalog_path(), &publication.lock),
        (arrangement_path(), &publication.arrangement_lock),
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
/// `test:looms` went red much later, by which time nothing connected the failure to the promote
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
         component this promote reworded; `mise run test:looms` is where their stale transcripts \
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

fn cases_dir() -> PathBuf {
    crates_dir().join("aid").join("tests")
}

/// `spike/crates`, so every path this tool prints reads as a real location rather than as a
/// traversal out of whichever crate happens to host the binary.
fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| PathBuf::from("crates"), Path::to_path_buf)
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
    gated: &GatedCases,
    env: &RunEnv,
    quiet: bool,
    mint: Mint,
    out: &mut impl Write,
) -> Result<Inspected, String> {
    let (mut consumer, mut refused, mut selected) =
        (DorcConsumer::new().minting(mint), 0usize, Vec::new());
    let ownership = corpus_ownership(&cases_dir())?;
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
            writeln!(body, "refusal in replay {index}: {}", error.explain(path))
                .map_err(|write| write.to_string())?;
            writeln!(body, "class: {error:?}").map_err(|write| write.to_string())?;
            writeln!(body, "baseline: exact renderer provenance")
                .map_err(|write| write.to_string())?;
            writeln!(body, "edited:\n{}", bounded_evidence(&dirty))
                .map_err(|write| write.to_string())?;
            emit_case(out, path, &body, quiet)?;
            continue;
        }
        let mut compiled = emit_previews(&mut consumer, previews, path, &mut body)?;
        let replays = inspected_replays
            .into_iter()
            .map(|(index, command, routed)| match routed.editable_render() {
                Some(render) => InspectedReplay::editable(
                    index,
                    command,
                    routed.output().to_owned(),
                    render,
                    &compiled.remove(&index).into_iter().collect::<Vec<_>>(),
                ),
                None => InspectedReplay::bytes(index, command, routed.output().to_owned()),
            })
            .collect();
        emit_case(out, path, &body, quiet)?;
        let is_touched = gated.touched.contains(&relative_path);
        inspected_cases.push((relative_path, source, is_touched, replays));
    }
    if refused > 0 {
        return Ok(Inspected::Refused { cases: refused });
    }
    let catalog = std::fs::read_to_string(catalog_path())
        .map_err(|error| format!("read catalog input: {error}"))?;
    let touched_cases = inspected_cases
        .iter()
        .filter(|(_, _, touched, _)| *touched)
        .map(|(path, _, _, _)| path.clone())
        .collect();
    InspectedCompilation::new(catalog, selected, touched_cases, inspected_cases)
        .map(|inspection| Inspected::Ready(inspection, consumer))
        .map_err(|error| error.to_string())
}

/// Emit each compiled preview, apply it to the mirror (the promote edited-mirror seam), and collect
/// the previews keyed by replay index for receipt inspection.
fn emit_previews(
    consumer: &mut DorcConsumer,
    previews: Vec<(usize, dorc_loom::CompilePreview)>,
    path: &Path,
    out: &mut impl Write,
) -> Result<std::collections::BTreeMap<usize, dorc_loom::CompilePreview>, String> {
    let mut compiled = std::collections::BTreeMap::new();
    for (index, preview) in previews {
        writeln!(out, "replay: {index}").map_err(|error| error.to_string())?;
        let rendered = render_compile_preview(&preview);
        consumer
            .apply_preview(&preview)
            .map_err(|error| format!("{}: {}", path.display(), error.explain(path)))?;
        warn_each(baked_value_warnings(&preview));
        compiled.insert(index, preview);
        writeln!(out, "{rendered}").map_err(|error| error.to_string())?;
    }
    Ok(compiled)
}

/// One warning per variable this edit removed while leaving its rendered value behind as text
/// (`30C` item 2). Never a refusal: an author may genuinely mean to freeze a value, and no evidence
/// available here can tell the two apart.
fn baked_value_warnings(preview: &dorc_loom::CompilePreview) -> Vec<String> {
    preview
        .sections()
        .iter()
        .flat_map(|section| {
            section.baked().iter().map(|name| {
                format!(
                    "`{{{{{0}}}}}` looks baked in: this edit removed the variable and its rendered \
                     value is still there as literal text, frozen at whatever this render happened \
                     to say. Type `{{{{{0}}}}}` where the value should go to keep it a variable; \
                     leave it as text only if you meant to.",
                    name.0
                )
            })
        })
        .collect()
}

fn receipt_store() -> Result<FsReceiptStore, String> {
    // No `..` components — the store's directory-tree check rejects them (`spike/target`).
    let target = spike_dir()?.join("target");
    FsReceiptStore::new(target)
}

fn catalog_path() -> PathBuf {
    crates_dir().join("aid").join("src").join("catalog_lock.rs")
}

fn arrangement_path() -> PathBuf {
    crates_dir()
        .join("aid")
        .join("src")
        .join("arrangement_lock.rs")
}

fn spike_dir() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "locate spike dir".to_owned())
}

/// The receipt may bind only transcript-prose edits. Repository reads are isolated
/// in `GitRepository`; this command owns only selection and inspection orchestration.
fn gate_touched_set(cases: &[PathBuf]) -> Result<GatedCases, String> {
    let repository = GitRepository::open()?;
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
    let catalog = repository.repository_path(&catalog_path())?;
    let arrangement = repository.repository_path(&arrangement_path())?;
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
         you have typed on disk is what `dorc-loom compile` reads"
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

    /// Every verb has a page of its own, and every page ends where the reader goes next. The index
    /// answers "which verb" and nothing else, so a verb that falls through to it silently teaches a
    /// reader who already knew the one thing it says.
    #[test]
    fn every_verb_has_its_own_page_ending_in_a_next_command() {
        for verb in VERBS {
            let page = usage_for(verb);
            assert_ne!(page, USAGE, "`{verb}` falls through to the index");
            // A synopsis may carry the global selector slot before the verb, so the pin is that it
            // names THIS verb, not that it opens on an exact prefix.
            let synopsis = page.lines().next().unwrap_or_default();
            assert!(
                synopsis.starts_with("usage: dorc-loom ") && synopsis.contains(verb),
                "`{verb}` page opens on someone else's synopsis: {synopsis}"
            );
            assert!(
                page.contains("\n  next: "),
                "`{verb}` page has no next: {page}"
            );
        }
        assert_eq!(usage_for("nonesuch"), USAGE);
    }

    /// `--help` after a verb asks the VERB; before one, or with no verb at all, it asks the index.
    #[test]
    fn help_routes_to_the_verb_the_reader_already_chose() {
        let argv = |args: &[&str]| args.iter().map(|a| (*a).to_owned()).collect::<Vec<_>>();
        assert_eq!(chosen_verb(&argv(&["promote", "--help"])), Some("promote"));
        assert_eq!(chosen_verb(&argv(&["help", "vars"])), Some("vars"));
        assert_eq!(chosen_verb(&argv(&["help"])), None);
        assert_eq!(chosen_verb(&argv(&["--help"])), None);
        assert_eq!(chosen_verb(&argv(&["nonesuch", "-h"])), None);
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
                Ok(Command::Help { verb: Some(verb) }) if verb == "add-register"
            ),
            "`--help` after a verb still asks the verb"
        );
        assert!(matches!(
            parse_argv(&argv(&["help"])),
            Ok(Command::Help { verb: None })
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

    /// A promote-time MARKING decision: compile publishes nothing so it takes neither, and the two
    /// together are a contradiction rather than a last-one-wins.
    #[test]
    fn the_provenance_flags_belong_to_promote_and_exclude_each_other() {
        let argv = |args: &[&str]| args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/cmdsub-command.loom")
            .to_str()
            .expect("the fixture path is UTF-8")
            .to_owned();

        assert!(matches!(
            parse_argv(&argv(&["promote", &fixture, HUMAN])),
            Ok(Command::Promote {
                provenance: Provenance::Human,
                ..
            })
        ));
        assert!(matches!(
            parse_argv(&argv(&["promote", &fixture, SLOP])),
            Ok(Command::Promote {
                provenance: Provenance::Slop,
                ..
            })
        ));
        assert!(matches!(
            parse_argv(&argv(&["promote", &fixture])),
            Ok(Command::Promote {
                provenance: Provenance::Default,
                ..
            })
        ));
        assert!(
            parse_argv(&argv(&["promote", &fixture, HUMAN, SLOP]))
                .is_err_and(|error| error.contains("opposite things"))
        );
        assert!(
            parse_argv(&argv(&["compile", &fixture, HUMAN]))
                .is_err_and(|error| error.contains("marks nothing"))
        );
    }

    /// Under-naming is the failure that matters: a rewritten case's staged bytes are the author's
    /// own pre-promote text, so a bare `git commit` would take those and drop the promotion.
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
        let note = nothing_moved_note(true, "compile", &one).expect("a no-op run says so");
        assert!(note.contains("this compile changed nothing"), "{note}");
        assert!(note.contains("whylog-absent.loom"), "{note}");
        assert!(note.contains("worktree"), "{note}");

        let many = selected(&["a.loom", "b.loom", "c.loom"]);
        let note = nothing_moved_note(true, "promote", &many).expect("a no-op run says so");
        assert!(note.contains("this promote changed nothing"), "{note}");
        assert!(note.contains("3 selected cases"), "{note}");

        assert_eq!(nothing_moved_note(false, "promote", &many), None);
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

    /// The other half: an unedited consumer stales nothing, so an ordinary promote stays quiet.
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
