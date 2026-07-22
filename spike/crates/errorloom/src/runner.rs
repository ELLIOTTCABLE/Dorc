//! The replay runner (`282` §7): materialize a case's file sections to a temp
//! dir, run the `-- replay --` section's `$ ` command blocks SEQUENTIALLY in one
//! shared cwd (state flows between commands by design), and capture each block's
//! combined output for drift-checking or inline-on-bless.
//!
//! Environment is fully caller-injected ([`RunEnv`]) — errorloom provides the
//! mechanism (an exact env table + a `PATH` search list, `env -i`-style) and
//! never invents an environment; policy such as inert mocks is the consumer's
//! (`28A` §1). This is the crate's I/O edge; the transport kernel stays pure.

use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::container::{Case, CaseError};
use crate::{ConsumerKey, EditableRender};

/// Maximum combined bytes captured while one generic replay process runs.
/// Read one additional byte so overflow is refused before result accumulation.
pub const MAX_CAPTURE_BYTES: usize = 64 * 1024;

/// The materialized state shared by every replay of one case.
///
/// Consumers receive this only while errorloom owns the materialization, so a
/// handled replay and the configured executor observe the same working tree.
#[derive(Debug)]
pub struct ReplayContext<'a> {
    cwd: &'a Path,
    scratch: &'a Path,
    env: &'a RunEnv,
}

impl ReplayContext<'_> {
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
}

/// The exact result of one consumer-driven replay.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplayResult<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    output: String,
    editable: Option<EditableRender<S, V>>,
}

impl<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> ReplayResult<S, V> {
    /// Construct a bytes-only result.
    #[must_use]
    pub fn bytes(output: String) -> Self {
        Self {
            output,
            editable: None,
        }
    }

    /// Construct a result whose renderer supplied exact editable provenance.
    #[must_use]
    pub fn editable(editable: EditableRender<S, V>) -> Self {
        let output = editable.text();
        Self {
            output,
            editable: Some(editable),
        }
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
}

/// A consumer's exact-shape replay driver.
pub trait ReplayDriver<S: ConsumerKey, V: Clone + Ord + std::fmt::Debug> {
    /// Declining is explicit: the embedding application decides whether to run a
    /// generic fallback.
    fn drive(&self, command: &str, context: &ReplayContext<'_>) -> Option<ReplayResult<S, V>>;
}

/// Drive every replay against one materialized context. The caller owns the
/// decline policy by supplying `fallback`; errorloom never selects it itself.
///
/// # Errors
/// Returns a materialization, execution, or caller-supplied replay error.
pub fn drive_case<S, V>(
    case: &Case,
    env: &RunEnv,
    mut drive: impl FnMut(&str, &ReplayContext<'_>) -> Result<ReplayResult<S, V>, RunError>,
) -> Result<Vec<ReplayResult<S, V>>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    let base = unique_base()?;
    let result = drive_in(case, env, &base, &mut drive);
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
pub fn execute_generic(command: &str, context: &ReplayContext<'_>) -> Result<String, RunError> {
    run_block(0, command, context.env, context.cwd)
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
    let work = base.join("work");
    fs::create_dir(&work)?;
    for (rel, content) in case.materialized_files() {
        let target = work.join(&rel);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, content)?;
    }

    let mut outputs: Vec<String> = Vec::new();
    for (index, block) in case.replay().blocks().iter().enumerate() {
        let output = run_block(index, block.command(), env, &work)?;
        outputs.push(output);
    }

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
    Ok(ReplayCapture { outputs })
}

fn drive_in<S, V>(
    case: &Case,
    env: &RunEnv,
    base: &Path,
    drive: &mut impl FnMut(&str, &ReplayContext<'_>) -> Result<ReplayResult<S, V>, RunError>,
) -> Result<Vec<ReplayResult<S, V>>, RunError>
where
    S: ConsumerKey,
    V: Clone + Ord + std::fmt::Debug,
{
    let work = base.join("work");
    let scratch = base.join("scratch");
    fs::create_dir(&work)?;
    fs::create_dir(&scratch)?;
    for (rel, content) in case.materialized_files() {
        let target = work.join(&rel);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, content)?;
    }
    let context = ReplayContext {
        cwd: &work,
        scratch: &scratch,
        env,
    };
    case.replay()
        .blocks()
        .iter()
        .map(|block| drive(block.command(), &context))
        .collect()
}

fn run_block(
    index: usize,
    command_line: &str,
    env: &RunEnv,
    work: &Path,
) -> Result<String, RunError> {
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
    let _status = child.wait()?;
    String::from_utf8(bytes).map_err(|error| RunError::NonUtf8Output {
        block: index,
        preview: String::from_utf8_lossy(&error.into_bytes()).into_owned(),
    })
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
