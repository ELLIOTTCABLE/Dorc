//! The replay runner (`282` §7): materialize a case's file sections to a temp
//! dir, run the `-- replay --` section's `$ ` command blocks SEQUENTIALLY in one
//! shared cwd (state flows between commands by design), and capture each block's
//! combined output for drift-checking or inline-on-bless.
//!
//! Environment is fully caller-injected ([`RunEnv`]) — errorloom provides the
//! mechanism (an exact env table + a `PATH` search list, `env -i`-style) and
//! never invents an environment; policy such as inert mocks is the consumer's
//! (`28A` §1). This is the crate's I/O edge; the transport kernel stays pure.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::command::{
    OutputRedirection, RedirectionTarget, ReplayChannel, ReplayCommand, ReplayInputTarget,
    ReplayParseError,
};
use crate::container::{Case, CaseError, MAX_CASE_BYTES, MAX_SECTION_COUNT};
use crate::{ConsumerKey, EditableRender, RenderComponent};

/// Maximum combined bytes captured while one generic replay process runs.
/// Read one additional byte so overflow is refused before result accumulation.
pub const MAX_CAPTURE_BYTES: usize = 64 * 1024;

/// The materialized state shared by every replay of one case.
///
/// Consumers receive this only while errorloom owns the materialization, so a
/// handled replay and the configured executor observe the same working tree.
#[derive(Debug)]
pub struct ReplayContext<'a> {
    block: usize,
    cwd: &'a Path,
    scratch: &'a Path,
    env: &'a RunEnv,
    inputs: &'a BTreeMap<String, String>,
}

impl ReplayContext<'_> {
    /// The zero-based replay block index.
    #[must_use]
    pub const fn block(&self) -> usize {
        self.block
    }

    /// The shared case working directory.
    #[must_use]
    pub fn cwd(&self) -> &Path {
        self.cwd
    }

    /// The per-case scratch directory shared by all replay handlers.
    #[must_use]
    pub fn scratch(&self) -> &Path {
        self.scratch
    }

    /// The exact injected environment for this case.
    #[must_use]
    pub fn env(&self) -> &RunEnv {
        self.env
    }

    /// Return the exact bounded contents of a file materialized for this case.
    #[must_use]
    pub fn materialized_input(&self, path: &str) -> Option<&str> {
        self.inputs.get(path).map(String::as_str)
    }

    /// Read the current bounded contents of a sandbox file.
    #[must_use]
    pub fn read_file(&self, path: &str) -> Option<String> {
        read_bounded_file(self.cwd, path).ok().flatten()
    }
}

/// One extra bounded, case-relative file supplied by an embedding consumer.
///
/// This is deliberately consumer-neutral: it lets a replay refer to an explicit
/// input that is not itself a txtar section without teaching errorloom its meaning.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplayInput {
    path: String,
    content: String,
}

impl ReplayInput {
    /// Construct a bounded materialized input.
    ///
    /// # Errors
    /// Returns a refusal when the path is unsafe or the contents exceed the case
    /// admission limit.
    pub fn new(path: impl Into<String>, content: impl Into<String>) -> Result<Self, RunError> {
        let path = path.into();
        let content = content.into();
        if !safe_relative_path(&path) {
            return Err(RunError::UnsafeReplayInput { path });
        }
        if content.len() > MAX_CASE_BYTES {
            return Err(RunError::ReplayInputTooLarge {
                path,
                limit: MAX_CASE_BYTES,
            });
        }
        Ok(Self { path, content })
    }
}

/// A retained command status. It is observable only through a later `echo $?` replay.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReplayStatus(i32);

impl ReplayStatus {
    /// Successful completion.
    pub const SUCCESS: Self = Self(0);

    /// Retain a driver's exact process-independent status code.
    #[must_use]
    pub const fn new(code: i32) -> Self {
        Self(code)
    }

    /// The retained integer status.
    #[must_use]
    pub const fn code(self) -> i32 {
        self.0
    }
}

/// One ordered direct-driver output emission.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplayEmission<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    channel: ReplayChannel,
    render: EditableRender<S, V>,
    carries_editability: bool,
}

impl<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> ReplayEmission<S, V> {
    /// Construct immutable bytes on one channel.
    #[must_use]
    pub fn bytes(channel: ReplayChannel, text: impl Into<String>) -> Self {
        Self {
            channel,
            render: EditableRender::new(vec![RenderComponent::Structure(text.into())]),
            carries_editability: false,
        }
    }

    /// Construct renderer-stamped output on one channel.
    #[must_use]
    pub fn editable(channel: ReplayChannel, render: EditableRender<S, V>) -> Self {
        Self {
            channel,
            render,
            carries_editability: true,
        }
    }

    /// The channel this emission used.
    #[must_use]
    pub const fn channel(&self) -> ReplayChannel {
        self.channel
    }

    /// The exact emitted text.
    #[must_use]
    pub fn text(&self) -> String {
        self.render.text()
    }

    fn from_components(
        channel: ReplayChannel,
        components: Vec<RenderComponent<S, V>>,
        carries_editability: bool,
    ) -> Self {
        Self {
            channel,
            render: EditableRender::new(components),
            carries_editability,
        }
    }
}

/// The exact result of one consumer-driven replay.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplayResult<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    output: String,
    editable: Option<EditableRender<S, V>>,
    emissions: Vec<ReplayEmission<S, V>>,
    status: ReplayStatus,
    routing: Routing,
}

impl<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> ReplayResult<S, V> {
    /// Construct a bytes-only result.
    #[must_use]
    pub fn bytes(output: String) -> Self {
        Self::emitted(
            ReplayStatus::SUCCESS,
            vec![ReplayEmission::bytes(ReplayChannel::Stdout, output)],
        )
    }

    /// Construct a result whose renderer supplied exact editable provenance.
    #[must_use]
    pub fn editable(editable: EditableRender<S, V>) -> Self {
        Self::emitted(
            ReplayStatus::SUCCESS,
            vec![ReplayEmission::editable(ReplayChannel::Stdout, editable)],
        )
    }

    /// Construct an ordered direct-driver result.
    #[must_use]
    pub fn emitted(status: ReplayStatus, emissions: Vec<ReplayEmission<S, V>>) -> Self {
        Self::projected(status, emissions, Routing::Controlled)
    }

    /// Change the retained status without changing any output.
    #[must_use]
    pub fn with_status(mut self, status: ReplayStatus) -> Self {
        self.status = status;
        self
    }

    /// Exact output bytes represented as transcript text.
    #[must_use]
    pub fn output(&self) -> &str {
        &self.output
    }

    /// Renderer-stamped edit authority for this exact result, if any.
    #[must_use]
    pub fn editable_render(&self) -> Option<&EditableRender<S, V>> {
        self.editable.as_ref()
    }

    /// The retained status for a later `echo $?` replay.
    #[must_use]
    pub const fn status(&self) -> ReplayStatus {
        self.status
    }

    fn opaque(output: String, status: ReplayStatus) -> Self {
        Self::projected(
            status,
            vec![ReplayEmission::bytes(ReplayChannel::Stdout, output)],
            Routing::Opaque,
        )
    }

    fn projected(
        status: ReplayStatus,
        emissions: Vec<ReplayEmission<S, V>>,
        routing: Routing,
    ) -> Self {
        let output = emissions.iter().map(ReplayEmission::text).collect();
        let carries_editability = emissions
            .iter()
            .any(|emission| emission.carries_editability);
        let components = emissions
            .iter()
            .flat_map(|emission| emission.render.components().iter().cloned())
            .collect();
        Self {
            output,
            editable: carries_editability.then(|| EditableRender::new(components)),
            emissions,
            status,
            routing,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Routing {
    Controlled,
    Opaque,
}

/// A consumer's exact-shape replay driver.
pub trait ReplayDriver<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    /// Declining is explicit: the embedding application decides whether to run a
    /// generic fallback.
    fn drive(
        &self,
        command: &ReplayCommand,
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<S, V>>;
}

/// Drive every replay against one materialized context. The caller owns the
/// decline policy by supplying `fallback`; errorloom never selects it itself.
///
/// # Errors
/// Returns a materialization, execution, or caller-supplied replay error.
pub fn drive_case<S, V>(
    case: &Case,
    env: &RunEnv,
    drive: impl FnMut(&ReplayCommand, &ReplayContext<'_>) -> Result<ReplayResult<S, V>, RunError>,
) -> Result<Vec<ReplayResult<S, V>>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    drive_case_with_inputs(case, env, &[], drive)
}

/// Drive every replay after materializing caller-supplied bounded inputs beside
/// the case sections.
///
/// # Errors
/// Returns a materialization, execution, or caller-supplied replay error.
pub fn drive_case_with_inputs<S, V>(
    case: &Case,
    env: &RunEnv,
    inputs: &[ReplayInput],
    mut drive: impl FnMut(&ReplayCommand, &ReplayContext<'_>) -> Result<ReplayResult<S, V>, RunError>,
) -> Result<Vec<ReplayResult<S, V>>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    let base = unique_base()?;
    let result = drive_in(case, env, inputs, &base, &mut drive);
    let _ = fs::remove_dir_all(&base);
    result
}

/// Execute a declined replay with the controlled generic executor.
///
/// This is deliberately separate from [`ReplayDriver`]: consumers must choose
/// this fallback explicitly and its bytes never carry editable provenance.
///
/// # Errors
/// Returns a controlled-executor failure.
pub fn execute_generic<S, V>(
    command: &ReplayCommand,
    context: &ReplayContext<'_>,
) -> Result<ReplayResult<S, V>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    let (output, status) = run_block(context.block, command.original(), context.env, context.cwd)?;
    Ok(ReplayResult::opaque(output, status))
}

/// A caller-injected execution environment: an explicit shell, exact env table,
/// and `PATH` search list. Nothing ambient leaks in (`env -i`-style).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RunEnv {
    path: Vec<PathBuf>,
    vars: BTreeMap<String, String>,
    shell: Option<PathBuf>,
}

impl RunEnv {
    /// An empty environment (no PATH dirs, no vars).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a directory to the `PATH` search list (where argv[0] is resolved and
    /// what the child sees as `PATH`).
    #[must_use]
    pub fn path_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.path.push(dir.into());
        self
    }

    /// Set an environment variable for every command.
    #[must_use]
    pub fn var(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.vars.insert(name.into(), value.into());
        self
    }

    /// Select the shell used for replay execution. The caller owns this policy;
    /// errorloom never discovers a shell from the ambient environment.
    #[must_use]
    pub fn shell(mut self, shell: impl Into<PathBuf>) -> Self {
        self.shell = Some(shell.into());
        self
    }
}

/// The combined per-block output captured by one replay run (`282` §7), in block
/// order.
#[derive(Clone, PartialEq, Eq, Debug)]
#[must_use = "a capture holds the outputs to compare or inline"]
pub struct ReplayCapture {
    outputs: Vec<String>,
    statuses: Vec<ReplayStatus>,
}

impl ReplayCapture {
    /// The captured combined outputs, one per block, in order.
    #[must_use]
    pub fn outputs(&self) -> &[String] {
        &self.outputs
    }

    /// Consume the capture, yielding the outputs (for inline-on-bless).
    #[must_use]
    pub fn into_outputs(self) -> Vec<String> {
        self.outputs
    }

    /// The retained command statuses, one per block.
    #[must_use]
    pub fn statuses(&self) -> &[ReplayStatus] {
        &self.statuses
    }
}

/// One block whose captured output diverged from the committed transcript.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Drift {
    block: usize,
    command: String,
    expected: String,
    actual: String,
}

impl Drift {
    /// The zero-based block index.
    #[must_use]
    pub fn block(&self) -> usize {
        self.block
    }

    /// The command whose output diverged.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// The committed (expected) output.
    #[must_use]
    pub fn expected(&self) -> &str {
        &self.expected
    }

    /// The freshly-captured (actual) output.
    #[must_use]
    pub fn actual(&self) -> &str {
        &self.actual
    }
}

/// The result of a drift check: the blocks whose output moved (`282` §7 — byte
/// stability under re-execution is the run gate).
#[derive(Clone, PartialEq, Eq, Debug)]
#[must_use = "a run report must be inspected for drift"]
pub struct RunReport {
    drifts: Vec<Drift>,
}

impl RunReport {
    /// The drifted blocks (empty ⇒ byte-stable).
    #[must_use]
    pub fn drifts(&self) -> &[Drift] {
        &self.drifts
    }

    /// Whether every block reproduced its committed output.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.drifts.is_empty()
    }
}

/// Why a replay run failed (`282` §7). Blunt (`282:rul-internal-tool-sharp-edges`).
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum RunError {
    /// An I/O failure (materialize, spawn, capture). Keeps the source
    /// [`std::io::ErrorKind`] (`taste-F1`) so consumers can distinguish
    /// `NotFound` / `PermissionDenied` without string-matching the message.
    Io {
        /// The classified I/O error kind.
        kind: std::io::ErrorKind,
        /// The source error's rendered message.
        message: String,
    },
    /// A replay block had an empty command line.
    EmptyCommand {
        /// Zero-based block index.
        block: usize,
    },
    /// A replay command line had an unterminated single/double quote (`swe-F3`).
    UnterminatedQuote {
        /// Zero-based block index.
        block: usize,
    },
    /// A command's program was not found on the injected `PATH`.
    CommandNotFound {
        /// Zero-based block index.
        block: usize,
        /// The unresolved program name.
        program: String,
    },
    /// No shell was injected for an arbitrary replay command.
    ShellNotConfigured,
    /// A command produced non-UTF-8 output; transcripts are text-only. Keeps a
    /// lossy preview of the offending bytes (`taste-F3`) for diagnosis.
    NonUtf8Output {
        /// Zero-based block index.
        block: usize,
        /// A lossy (`U+FFFD`-substituted) preview of the captured bytes.
        preview: String,
    },
    /// A child exceeded the live combined-output capture ceiling and was reaped
    /// before an unbounded capture can accumulate.
    OutputTooLarge {
        /// Zero-based block index.
        block: usize,
        /// The maximum accepted captured bytes.
        limit: usize,
    },
    /// Captured output leaked the sandbox's absolute path (`282` §7).
    SandboxPathLeak {
        /// Zero-based block index.
        block: usize,
        /// The offending line.
        line: String,
    },
    /// The inlined output failed a case-hygiene gate.
    Hygiene(CaseError),
    /// An embedding-supplied input path was not a safe case-relative path.
    UnsafeReplayInput {
        /// The rejected path.
        path: String,
    },
    /// An embedding-supplied input exceeded the bounded case-input limit.
    ReplayInputTooLarge {
        /// The rejected path.
        path: String,
        /// The maximum accepted byte count.
        limit: usize,
    },
    /// An embedding-supplied input would overwrite a case section or another input.
    DuplicateReplayInput {
        /// The duplicate path.
        path: String,
    },
    /// Embedding-supplied inputs exceeded the case section-count ceiling.
    ReplayInputCountExceeded {
        /// The maximum number of extra inputs for this case.
        limit: usize,
    },
    /// A replay command was outside the closed grammar.
    UnsupportedReplayGrammar {
        /// Zero-based block index.
        block: usize,
        /// The parser's closed refusal.
        error: ReplayParseError,
    },
    /// A generic process ended without a portable integer status.
    ProcessStatusUnavailable {
        /// Zero-based block index.
        block: usize,
    },
    /// A sandbox file exceeded the bounded replay-file ceiling.
    SandboxFileTooLarge {
        /// The case-relative path.
        path: String,
        /// The maximum accepted bytes.
        limit: usize,
    },
    /// A sandbox file was not UTF-8.
    NonUtf8SandboxFile {
        /// The case-relative path.
        path: String,
        /// A lossy preview of its bytes.
        preview: String,
    },
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::Io { kind, message } => write!(f, "run: io error ({kind}): {message}"),
            RunError::EmptyCommand { block } => write!(f, "run: block {block} has no command"),
            RunError::UnterminatedQuote { block } => {
                write!(f, "run: block {block} has an unterminated quote")
            }
            RunError::CommandNotFound { block, program } => {
                write!(f, "run: block {block} program {program:?} not on PATH")
            }
            RunError::ShellNotConfigured => f.write_str("run: no controlled shell configured"),
            RunError::NonUtf8Output { block, preview } => {
                write!(
                    f,
                    "run: block {block} produced non-UTF-8 output: {preview:?}"
                )
            }
            RunError::OutputTooLarge { block, limit } => {
                write!(
                    f,
                    "run: block {block} exceeded captured-output limit {limit}"
                )
            }
            RunError::SandboxPathLeak { block, line } => {
                write!(f, "run: block {block} leaked the sandbox path: {line:?}")
            }
            RunError::Hygiene(inner) => write!(f, "run: {inner}"),
            RunError::UnsafeReplayInput { path } => {
                write!(f, "run: unsafe replay input path {path:?}")
            }
            RunError::ReplayInputTooLarge { path, limit } => {
                write!(f, "run: replay input {path:?} exceeds limit {limit}")
            }
            RunError::DuplicateReplayInput { path } => {
                write!(f, "run: duplicate replay input {path:?}")
            }
            RunError::ReplayInputCountExceeded { limit } => {
                write!(f, "run: replay input count exceeds limit {limit}")
            }
            RunError::UnsupportedReplayGrammar { block, error } => {
                write!(
                    f,
                    "run: block {block} is outside the replay grammar: {error}"
                )
            }
            RunError::ProcessStatusUnavailable { block } => {
                write!(f, "run: block {block} ended without a portable status")
            }
            RunError::SandboxFileTooLarge { path, limit } => {
                write!(f, "run: sandbox file {path:?} exceeds limit {limit}")
            }
            RunError::NonUtf8SandboxFile { path, preview } => {
                write!(f, "run: sandbox file {path:?} is not UTF-8: {preview:?}")
            }
        }
    }
}

impl std::error::Error for RunError {}

impl From<std::io::Error> for RunError {
    fn from(error: std::io::Error) -> Self {
        RunError::Io {
            kind: error.kind(),
            message: error.to_string(),
        }
    }
}

/// Materialize a case and run its replay blocks, returning the combined output of
/// each in order.
///
/// Blocking contract (`swe-F5`; churn-avoidance-disclosure): each block runs to
/// completion with NO timeout at v1 — a case whose command hangs hangs the run.
/// Cases are trusted internal fixtures, so a wall-clock seam is deferred, not a
/// silent gap.
///
/// # Errors
/// Returns [`RunError`] for an I/O failure, an empty or unresolvable command, an
/// unterminated quote, non-UTF-8 output, or a sandbox-path leak.
pub fn run_case(case: &Case, env: &RunEnv) -> Result<ReplayCapture, RunError> {
    let base = unique_base()?;
    let result = run_in(case, env, &base);
    let _ = fs::remove_dir_all(&base);
    result
}

fn run_in(case: &Case, env: &RunEnv, base: &Path) -> Result<ReplayCapture, RunError> {
    let mut generic = |command: &ReplayCommand, context: &ReplayContext<'_>| {
        execute_generic::<String, String>(command, context)
    };
    let results = drive_in(case, env, &[], base, &mut generic)?;
    let outputs: Vec<String> = results.iter().map(|result| result.output.clone()).collect();
    let statuses: Vec<ReplayStatus> = results.iter().map(|result| result.status).collect();

    let base_str = base.to_string_lossy();
    for (index, output) in outputs.iter().enumerate() {
        for line in output.lines() {
            if line.contains(base_str.as_ref()) {
                return Err(RunError::SandboxPathLeak {
                    block: index,
                    line: line.to_owned(),
                });
            }
        }
    }
    Ok(ReplayCapture { outputs, statuses })
}

fn drive_in<S, V>(
    case: &Case,
    env: &RunEnv,
    inputs: &[ReplayInput],
    base: &Path,
    drive: &mut impl FnMut(&ReplayCommand, &ReplayContext<'_>) -> Result<ReplayResult<S, V>, RunError>,
) -> Result<Vec<ReplayResult<S, V>>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    let work = base.join("work");
    let scratch = base.join("scratch");
    let input_limit = MAX_SECTION_COUNT.saturating_sub(case.sections().len().saturating_add(1));
    if inputs.len() > input_limit {
        return Err(RunError::ReplayInputCountExceeded { limit: input_limit });
    }
    fs::create_dir(&work)?;
    fs::create_dir(&scratch)?;
    let mut materialized = BTreeMap::new();
    let mut tracked = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for (rel, content) in case.materialized_files() {
        let name = rel.to_string_lossy().into_owned();
        paths.insert(name.clone());
        let target = work.join(&rel);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, content)?;
        materialized.insert(name, content.to_owned());
        tracked.insert(
            rel.to_string_lossy().into_owned(),
            TrackedFile::from_bytes(content),
        );
    }
    for input in inputs {
        if !paths.insert(input.path.clone()) {
            return Err(RunError::DuplicateReplayInput {
                path: input.path.clone(),
            });
        }
        let target = work.join(&input.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, &input.content)?;
        materialized.insert(input.path.clone(), input.content.clone());
        tracked.insert(input.path.clone(), TrackedFile::from_bytes(&input.content));
    }
    let mut last_status = ReplayStatus::SUCCESS;
    let mut results = Vec::new();
    for (block_index, block) in case.replay().blocks().iter().enumerate() {
        let command = ReplayCommand::parse(block.command()).map_err(|error| {
            RunError::UnsupportedReplayGrammar {
                block: block_index,
                error,
            }
        })?;
        require_input(&command, &work)?;
        let mut routing = RoutingPlan::prepare(&command, &work, &mut tracked)?;
        let context = ReplayContext {
            block: block_index,
            cwd: &work,
            scratch: &scratch,
            env,
            inputs: &materialized,
        };
        let result = match builtin(&command, &work, &tracked, last_status)? {
            Some(result) => result,
            None => drive(&command, &context)?,
        };
        let result = if result.routing == Routing::Opaque {
            for file in tracked.values_mut() {
                file.components = None;
                file.carries_editability = false;
            }
            result
        } else {
            routing.apply(result, &work, &mut tracked)?
        };
        last_status = result.status;
        results.push(result);
    }
    Ok(results)
}

#[derive(Clone)]
struct TrackedFile<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    components: Option<Vec<RenderComponent<S, V>>>,
    carries_editability: bool,
}

impl<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> TrackedFile<S, V> {
    fn from_bytes(bytes: &str) -> Self {
        Self {
            components: Some(vec![RenderComponent::Structure(bytes.to_owned())]),
            carries_editability: false,
        }
    }
}

#[derive(Clone)]
enum Destination {
    Terminal(ReplayChannel),
    File(usize),
    Null,
}

struct PendingFile<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    path: String,
    components: Vec<RenderComponent<S, V>>,
    carries_editability: bool,
}

struct RoutingPlan<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    stdout: Destination,
    stderr: Destination,
    files: Vec<PendingFile<S, V>>,
}

impl<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> RoutingPlan<S, V> {
    fn prepare(
        command: &ReplayCommand,
        work: &Path,
        tracked: &mut BTreeMap<String, TrackedFile<S, V>>,
    ) -> Result<Self, RunError> {
        let mut plan = Self {
            stdout: Destination::Terminal(ReplayChannel::Stdout),
            stderr: Destination::Terminal(ReplayChannel::Stderr),
            files: Vec::new(),
        };
        for redirection in command.output_redirections() {
            match redirection {
                OutputRedirection::StderrToStdout => plan.stderr = plan.stdout.clone(),
                OutputRedirection::To { channel, target } => {
                    let destination = match target {
                        RedirectionTarget::Null => Destination::Null,
                        RedirectionTarget::File(path) => {
                            fs::File::create(work.join(path))?;
                            tracked.insert(path.clone(), TrackedFile::from_bytes(""));
                            let index = plan.files.len();
                            plan.files.push(PendingFile {
                                path: path.clone(),
                                components: Vec::new(),
                                carries_editability: false,
                            });
                            Destination::File(index)
                        }
                    };
                    match channel {
                        ReplayChannel::Stdout => plan.stdout = destination,
                        ReplayChannel::Stderr => plan.stderr = destination,
                    }
                }
            }
        }
        Ok(plan)
    }

    fn apply(
        &mut self,
        result: ReplayResult<S, V>,
        work: &Path,
        tracked: &mut BTreeMap<String, TrackedFile<S, V>>,
    ) -> Result<ReplayResult<S, V>, RunError> {
        let mut terminal = Vec::new();
        for emission in result.emissions {
            let destination = match emission.channel {
                ReplayChannel::Stdout => self.stdout.clone(),
                ReplayChannel::Stderr => self.stderr.clone(),
            };
            match destination {
                Destination::Terminal(channel) => terminal.push(ReplayEmission::from_components(
                    channel,
                    emission.render.components().to_vec(),
                    emission.carries_editability,
                )),
                Destination::File(index) => {
                    let Some(file) = self.files.get_mut(index) else {
                        continue;
                    };
                    file.components
                        .extend(emission.render.components().iter().cloned());
                    file.carries_editability |= emission.carries_editability;
                }
                Destination::Null => {}
            }
        }
        for file in &self.files {
            let text = EditableRender::new(file.components.clone()).text();
            fs::write(work.join(&file.path), text)?;
            tracked.insert(
                file.path.clone(),
                TrackedFile {
                    components: Some(file.components.clone()),
                    carries_editability: file.carries_editability,
                },
            );
        }
        Ok(ReplayResult::projected(
            result.status,
            terminal,
            Routing::Controlled,
        ))
    }
}

fn builtin<S, V>(
    command: &ReplayCommand,
    work: &Path,
    tracked: &BTreeMap<String, TrackedFile<S, V>>,
    last_status: ReplayStatus,
) -> Result<Option<ReplayResult<S, V>>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    if command.argv() == ["echo", "$?"] {
        return Ok(Some(ReplayResult::bytes(format!(
            "{}\n",
            last_status.code()
        ))));
    }
    let [program, path] = command.argv() else {
        return Ok(None);
    };
    if program != "cat" {
        return Ok(None);
    }
    if path == "/dev/null" {
        return Ok(Some(ReplayResult::bytes(String::new())));
    }
    if !safe_relative_path(path) {
        return Err(RunError::UnsafeReplayInput { path: path.clone() });
    }
    if let Some(file) = tracked.get(path)
        && let Some(components) = &file.components
    {
        return Ok(Some(ReplayResult::emitted(
            ReplayStatus::SUCCESS,
            vec![ReplayEmission::from_components(
                ReplayChannel::Stdout,
                components.clone(),
                file.carries_editability,
            )],
        )));
    }
    let bytes = read_bounded_file(work, path)?
        .ok_or_else(|| RunError::UnsafeReplayInput { path: path.clone() })?;
    Ok(Some(ReplayResult::bytes(bytes)))
}

fn require_input(command: &ReplayCommand, work: &Path) -> Result<(), RunError> {
    match command.input() {
        None | Some(ReplayInputTarget::Null) => Ok(()),
        Some(ReplayInputTarget::File(path)) => {
            let _ = read_bounded_file(work, path)?
                .ok_or_else(|| RunError::UnsafeReplayInput { path: path.clone() })?;
            Ok(())
        }
    }
}

fn read_bounded_file(work: &Path, path: &str) -> Result<Option<String>, RunError> {
    if path == "/dev/null" {
        return Ok(Some(String::new()));
    }
    if !safe_relative_path(path) {
        return Ok(None);
    }
    let mut bytes = Vec::with_capacity(MAX_CAPTURE_BYTES.saturating_add(1));
    fs::File::open(work.join(path))?
        .take(u64::try_from(MAX_CAPTURE_BYTES.saturating_add(1)).unwrap_or(u64::MAX))
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_CAPTURE_BYTES {
        return Err(RunError::SandboxFileTooLarge {
            path: path.to_owned(),
            limit: MAX_CAPTURE_BYTES,
        });
    }
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|error| RunError::NonUtf8SandboxFile {
            path: path.to_owned(),
            preview: String::from_utf8_lossy(&error.into_bytes()).into_owned(),
        })
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains(['\\', ':'])
        && path
            .split('/')
            .all(|component| !matches!(component, "" | "." | ".."))
}

fn run_block(
    index: usize,
    command_line: &str,
    env: &RunEnv,
    work: &Path,
) -> Result<(String, ReplayStatus), RunError> {
    if command_line.trim().is_empty() {
        return Err(RunError::EmptyCommand { block: index });
    }
    let shell = env.shell.as_ref().ok_or(RunError::ShellNotConfigured)?;

    let mut command = Command::new(shell);
    // One pipe preserves v1's combined stream while it is bounded in memory.
    command.args(["-c", &format!("exec 2>&1\n{command_line}")]);
    command.current_dir(work);
    command.env_clear();
    for (name, value) in &env.vars {
        command.env(name, value);
    }
    // A path containing the platform separator cannot be joined; leaving PATH
    // unset then surfaces as a CommandNotFound rather than a silent misresolve
    // (taste-F10).
    if let Ok(joined) = std::env::join_paths(&env.path) {
        command.env("PATH", joined);
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::null());
    command.stdin(Stdio::null());

    let mut child = command.spawn()?;
    let mut stdout = child.stdout.take().ok_or_else(|| RunError::Io {
        kind: std::io::ErrorKind::BrokenPipe,
        message: "child stdout pipe unavailable".to_owned(),
    })?;
    let mut bytes = Vec::with_capacity(MAX_CAPTURE_BYTES.saturating_add(1));
    stdout
        .by_ref()
        .take(u64::try_from(MAX_CAPTURE_BYTES.saturating_add(1)).unwrap_or(u64::MAX))
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_CAPTURE_BYTES {
        let _ = child.kill();
        let _ = child.wait();
        return Err(RunError::OutputTooLarge {
            block: index,
            limit: MAX_CAPTURE_BYTES,
        });
    }
    let status = child.wait()?;
    let code = status
        .code()
        .ok_or(RunError::ProcessStatusUnavailable { block: index })?;
    let output = String::from_utf8(bytes).map_err(|error| RunError::NonUtf8Output {
        block: index,
        preview: String::from_utf8_lossy(&error.into_bytes()).into_owned(),
    })?;
    Ok((output, ReplayStatus::new(code)))
}

/// Run the case and report which blocks diverged from their committed output
/// (`282` §7). The committed transcript is the expectation.
///
/// # Errors
/// Propagates any [`RunError`] from [`run_case`].
pub fn check_run(case: &Case, env: &RunEnv) -> Result<RunReport, RunError> {
    let capture = run_case(case, env)?;
    let mut drifts: Vec<Drift> = Vec::new();
    for (index, (block, actual)) in case
        .replay()
        .blocks()
        .iter()
        .zip(capture.outputs())
        .enumerate()
    {
        if block.output() != actual {
            drifts.push(Drift {
                block: index,
                command: block.command().to_owned(),
                expected: block.output().to_owned(),
                actual: actual.clone(),
            });
        }
    }
    Ok(RunReport { drifts })
}

/// Structure-bless (the generic cram mode, `282` §6): re-run and re-inline every
/// block's output, then apply the case-hygiene gates. `required_key` names the
/// frontmatter coherence key, if any.
///
/// # Errors
/// Propagates any [`RunError`], including a hygiene refusal after inlining.
pub fn bless_structure(
    case: &mut Case,
    env: &RunEnv,
    required_key: Option<&str>,
) -> Result<(), RunError> {
    let capture = run_case(case, env)?;
    case.set_replay_outputs(capture.into_outputs());
    case.check_hygiene(required_key)
        .map_err(RunError::Hygiene)?;
    Ok(())
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// The bound on `unique_base`'s retry (`taste-F5`): the counter is
/// process-monotonic, so only pre-existing stale dirs at `pid-nonce` collide —
/// a handful at most. A distinctive bounded error beats an unbounded spin.
const MAX_BASE_ATTEMPTS: u32 = 1024;

/// Create a fresh, uniquely-named session dir under the system temp dir. Unique
/// by pid + a process-monotonic counter, so concurrent runs never collide; the
/// non-deterministic path is exactly why the sandbox-leak gate exists.
fn unique_base() -> std::io::Result<PathBuf> {
    let pid = std::process::id();
    for _ in 0..MAX_BASE_ATTEMPTS {
        let nonce = COUNTER.fetch_add(1, Ordering::Relaxed);
        let candidate = std::env::temp_dir().join(format!("errorloom-{pid}-{nonce}"));
        match fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        format!("errorloom: no free session dir after {MAX_BASE_ATTEMPTS} attempts"),
    ))
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    fn case_with_sections(section_count: usize) -> Case {
        let mut text = String::from("---\n---\n");
        for index in 0..section_count {
            writeln!(text, "-- section-{index}.txt --\ncase {index}")
                .expect("writing to a String cannot fail");
        }
        text.push_str("-- replay --\n$ ignored\n");
        Case::parse(&text).expect("case within section limit")
    }

    fn unexpected_driver(
        _: &ReplayCommand,
        _: &ReplayContext<'_>,
    ) -> Result<ReplayResult<String, String>, RunError> {
        Err(RunError::EmptyCommand { block: usize::MAX })
    }

    fn test_base(name: &str) -> PathBuf {
        let base =
            std::env::temp_dir().join(format!("errorloom-runner-{name}-{}", std::process::id()));
        fs::create_dir(&base).expect("fresh test base");
        base
    }

    #[test]
    fn case_section_collision_preserves_case_bytes() {
        let base = test_base("case-collision");
        let case =
            Case::parse("---\n---\n-- existing.txt --\ncase bytes\n-- replay --\n$ ignored\n")
                .expect("valid case");
        let inputs = [ReplayInput::new("existing.txt", "conflicting bytes").expect("safe input")];

        let error = drive_in(
            &case,
            &RunEnv::new(),
            &inputs,
            &base,
            &mut unexpected_driver,
        )
        .unwrap_err();

        assert_eq!(
            error,
            RunError::DuplicateReplayInput {
                path: "existing.txt".to_owned(),
            }
        );
        assert_eq!(
            fs::read_to_string(base.join("work/existing.txt")).expect("case file"),
            "case bytes\n"
        );
        fs::remove_dir_all(base).expect("remove test base");
    }

    #[test]
    fn duplicate_explicit_input_preserves_first_input_bytes() {
        let base = test_base("input-collision");
        let case = case_with_sections(0);
        let inputs = [
            ReplayInput::new("duplicate.txt", "first bytes").expect("safe input"),
            ReplayInput::new("duplicate.txt", "conflicting bytes").expect("safe input"),
        ];

        let error = drive_in(
            &case,
            &RunEnv::new(),
            &inputs,
            &base,
            &mut unexpected_driver,
        )
        .unwrap_err();

        assert_eq!(
            error,
            RunError::DuplicateReplayInput {
                path: "duplicate.txt".to_owned(),
            }
        );
        assert_eq!(
            fs::read_to_string(base.join("work/duplicate.txt")).expect("first input"),
            "first bytes"
        );
        fs::remove_dir_all(base).expect("remove test base");
    }

    #[test]
    fn replay_input_count_accepts_remaining_capacity_and_refuses_one_over() {
        let case = case_with_sections(MAX_SECTION_COUNT.saturating_sub(2));
        let at_capacity = [ReplayInput::new("remaining.txt", "input").expect("safe input")];
        let base = test_base("count-at-capacity");
        let mut observed = None;

        let result = drive_in(
            &case,
            &RunEnv::new(),
            &at_capacity,
            &base,
            &mut |_, context| {
                observed = context
                    .materialized_input("remaining.txt")
                    .map(str::to_owned);
                Ok(ReplayResult::<String, String>::bytes(String::new()))
            },
        );

        assert!(result.is_ok());
        assert_eq!(observed.as_deref(), Some("input"));
        fs::remove_dir_all(base).expect("remove test base");

        let over_capacity = [
            ReplayInput::new("first.txt", "first").expect("safe input"),
            ReplayInput::new("second.txt", "second").expect("safe input"),
        ];
        let base = test_base("count-over-capacity");
        let error = drive_in(
            &case,
            &RunEnv::new(),
            &over_capacity,
            &base,
            &mut unexpected_driver,
        )
        .unwrap_err();

        assert_eq!(error, RunError::ReplayInputCountExceeded { limit: 1 });
        assert!(!base.join("work").exists());
        fs::remove_dir_all(base).expect("remove test base");
    }
}
