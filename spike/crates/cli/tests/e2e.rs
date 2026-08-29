//! The central e2e acceptance runner — a faithful port of `e2e/run.sh` (`288` §7).
//!
//! Every gate below is the sh harness's, moved verbatim in behaviour: the `dash -n`
//! runnability floor, exec-under-inert-mocks with `PATH=<case>/mocks` only, the ordered
//! run-set compare, gate-1(a)–(d) on the executed probe, the
//! gate-2 redirect scan, the gate-3 stderr-severity floor, gate-5's argv-echo
//! differential, gate-6's dual-rail license judge, gate-7/gate-hint/gate-8 needles, the
//! guard-shape floor, the XFAIL/XPASS lens and its two-sided `head-expected.ran` pin,
//! and BLESS. The per-gate rationale lives in `crates/cli/CLAUDE.md`'s harness contract
//! and in each gate's doc comment here; the sh source is git history.
//!
//! DELIBERATE DEVIATIONS from the sh original, all noted at their site:
//! - the `env -i` scrub is `Command::env_clear`, and `umask 022` rides an
//!   `sh -c 'umask 022; exec …'` shim (Rust cannot set a umask without FFI);
//! - `framed_results` is computed ONCE per case instead of once per gate (it is a pure
//!   function of the case dir and the shared arg vector, so the gates see identical bytes);
//! - the content golden mismatch prints a first-divergence window, not a unified diff;
//! - `DORC_E2E_QUIET=1` selects libtest's terse format rather than suppressing `ok` lines;
//! - `RAN_ORDER=lax` is RETIRED in favour of the declared normalizer vocabulary
//!   (`tolerate=<class>`, below), which is applied on the capture at bless AND at check.

#![expect(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::print_stderr,
    clippy::too_many_lines,
    reason = "test harness over the committed corpus: a malformed fixture is a loud abort, and the ported gates keep the sh original's shape"
)]

mod sandbox;
mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use libtest_mimic::{Arguments, Failed, Trial};

use sandbox::ProfileSandbox;
use support::{
    E2eCase, E2eKind, LoomCase, RECORDS_NONCE, RECORDS_TOKEN, Selection, case_from_path,
    case_roots, discover_e2e, discover_looms, report_path_selection, resolve_selection, spike_root,
    split_path_selectors,
};

/// This crate's own `tests/` dir — the home of the round-trip collection, and the anchor
/// the pre-flight batteries resolve their specimens against.
fn own_cases() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

// ---------------------------------------------------------------------------
// process + filesystem plumbing

/// A finished child process, decoded lossily (the corpus is ASCII by construction).
struct Output {
    /// Captured stdout.
    stdout: String,
    /// Captured stderr.
    stderr: String,
    /// The exit status, or -1 for a signal death.
    code: i32,
}

/// Run `command` to completion, capturing both streams.
fn capture(command: &mut Command) -> Output {
    let done = command
        .output()
        .unwrap_or_else(|error| panic!("spawn {command:?}: {error}"));
    Output {
        stdout: String::from_utf8_lossy(&done.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&done.stderr).into_owned(),
        code: done.status.code().unwrap_or(-1),
    }
}

/// A throwaway directory, removed on drop (`mktemp -d`).
struct Scratch {
    /// The created dir.
    path: PathBuf,
}

impl Scratch {
    /// Create a fresh scratch dir under the system temp root.
    fn new(tag: &str) -> Self {
        static SEQ: AtomicUsize = AtomicUsize::new(0);
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("dorc-e2e-{}-{tag}-{seq}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create scratch dir");
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Read a file, or the empty string when it is absent.
fn read_or_empty(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// A file exists and is non-empty (`[ -f x ] && [ -s x ]`).
fn nonempty_file(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|meta| meta.is_file() && meta.len() > 0)
}

/// Emulate `$(…)`: a command substitution strips every trailing newline.
fn strip_trailing_newlines(text: &str) -> String {
    text.trim_end_matches('\n').to_owned()
}

/// Emulate `sed 's/\r$//'` over a whole text.
fn strip_cr(text: &str) -> String {
    text.lines()
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The lines of a shell-style string value (an empty value is ONE empty line, exactly as
/// `printf '%s\n' "$v"` produces).
fn lines_of(value: &str) -> Vec<&str> {
    value.split('\n').collect()
}

/// `LC_ALL=C sort` over a shell-style string value.
fn sort_lines(value: &str) -> String {
    let mut lines: Vec<&str> = lines_of(value);
    lines.sort_unstable();
    lines.join("\n")
}

/// PATH lookup and POSIX-shell discovery live in `internal-tooling`, the repo's
/// cross-platform tooling crate, so this runner and the mise tasks answer "where is a
/// shell" identically. Native Windows has none on PATH; that crate derives one from git.
use internal_tooling::{Posix, which};

// ---------------------------------------------------------------------------
// the harness's shared, immutable context

/// Binaries, the syntax checker, and the run mode — resolved once, shared by every trial.
/// The engine's harness clock pin, and the instant every case's transcript is dated by
/// (`rul-fixture-identity-never-production`). A round number in 2026 so a reader of a committed
/// transcript can tell at a glance that the date is fixture, not a real morning.
const FIXTURE_CLOCK_ENV: &str = "DORC_FIXTURE_CLOCK_MS";
const FIXTURE_CLOCK_MS: u64 = 1_769_306_437_000;

/// The engine's stdout-posture pin (`30Ng:rul-piped-stdout-carries-a-full-plan`), on the clock
/// pin's own footing: a real, non-hermetic edge fact a subprocess battery has to be able to state.
const STDOUT_POSTURE_ENV: &str = "DORC_STDOUT_POSTURE";

/// Where a case that owns its own per-user profile keeps it, inside its materialization.
///
/// A directory rather than a flag threaded through four drive seats: the profile a run resolves is
/// a property of the world the case was materialized into, and the one seat that builds every
/// command is the one seat that should have to know.
const OWN_PROFILE_DIR: &str = ".dorc-own-profile";

struct Harness {
    /// The `dorc` binary cargo just built for this test target.
    dorc: PathBuf,
    /// The `dorc-sh` sibling (the strip-and-exec off-ramp runner).
    dorc_sh: PathBuf,
    /// Absolute path of the strict-POSIX syntax checker (`dash`, else `sh`).
    checker: PathBuf,
    /// Its bare name, for gate messages.
    checker_name: String,
    /// `BLESS=1` — regenerate goldens from the current engine output.
    bless: bool,
    /// `BLESS_FLOOR=1` — the floor lane's own write authority (see [`FLOOR_BLESS_ENV`]).
    bless_floor: bool,
    /// The floor binaries gate-9 measures under, in the order named
    /// (`DORC_E2E_FLOOR_SHELLS`); empty ⇒ the lane does not fire.
    floor_shells: Vec<String>,
    /// The throwaway per-user profile every invocation is pointed at, so default-on keys and
    /// receipts land here instead of in the developer's real profile directory.
    ///
    /// BOTH roles, and that is the fix rather than a tidy-up: pointing only the state root at a
    /// sandbox left the CONFIGURATION root inherited, so a suite run minted a real keyset in
    /// whoever's profile ran it (measured r30, on the run that first made the binary publish).
    profile: ProfileSandbox,
}

impl Harness {
    /// Resolve the harness context, aborting loudly when the `-n` gate has no shell.
    ///
    /// A shell is a HARD dependency of this corpus, not a convenience: Dorc's product is
    /// sh, and these gates syntax-check and then execute rendered artifacts. Saying so in
    /// the refusal matters — the bare "no POSIX shell" this used to print read as a
    /// missing nicety, and every "Windows green" before 2026-07-26 was really git-bash
    /// silently supplying one.
    fn resolve() -> Self {
        let posix = Posix::find().unwrap_or_else(|why| {
            eprintln!(
                "e2e: no POSIX shell for the -n gate, so runnability cannot be validated — {why}.\n\
                 This corpus executes the sh it renders; a shell is a dependency, not a nicety.\n\
                 Windows takes it from git's own userland (no PATH setup needed); elsewhere,\n\
                 put dash or sh on PATH."
            );
            std::process::exit(2);
        });
        // `sh` on Windows is git's bash-in-sh-mode, which accepts `[[ ]]`/`<<<` and the
        // rest the dialect floor bans — a weaker check, so say which one ran.
        if posix.name != "dash" {
            eprintln!(
                "e2e: -n gate running under {} (not dash: a weaker dialect check)",
                posix.name
            );
        }
        let (checker_name, checker) = (posix.name.to_owned(), posix.shell);
        let bless = std::env::var("BLESS").as_deref() == Ok("1");
        let bless_floor = std::env::var(FLOOR_BLESS_ENV).as_deref() == Ok("1");
        let floor_shells: Vec<String> = std::env::var(FLOOR_SHELLS_ENV)
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .collect();
        // The mint's double opt-in, checked HERE so a half-spelled one refuses before any case
        // runs rather than reading as an ordinary green run that wrote nothing.
        if bless_floor && !bless {
            eprintln!(
                "e2e: {FLOOR_BLESS_ENV}=1 without BLESS=1 — the floor mint is a bless, and this run writes nothing."
            );
            std::process::exit(2);
        }
        if bless_floor && floor_shells.is_empty() {
            eprintln!(
                "e2e: {FLOOR_BLESS_ENV}=1 without {FLOOR_SHELLS_ENV} — `expected.emitted` is the shells' own answer, and no shell was named to ask."
            );
            std::process::exit(2);
        }
        Self {
            dorc: PathBuf::from(env!("CARGO_BIN_EXE_dorc")),
            dorc_sh: PathBuf::from(env!("CARGO_BIN_EXE_dorc-sh")),
            checker,
            checker_name,
            bless,
            bless_floor,
            floor_shells,
            profile: ProfileSandbox::new("e2e"),
        }
    }

    /// A bare `dorc` invocation. Every call site appends the case's shared
    /// `-o oracle … [DORC_FLAGS]` argv in the sh harness's own position — argument order
    /// is load-bearing for the mode-dispatching parser.
    ///
    /// The clock pin rides here rather than at the call sites: the why surface dates its output
    /// (receipt header, run-instants on `reported` rows), so an unpinned clock would make every
    /// transcript carrying one a non-fixpoint by construction.
    /// A `dorc` invocation with the harness's pinned clock and a throwaway state root.
    ///
    /// The state root is re-pointed rather than the durable disabled, so the corpus exercises the
    /// REAL default-destination resolver instead of a bypass of it. It must be re-pointed at all:
    /// since `28F:rul-w3-default-on-aim-high` every plan/apply/round-trip writes a receipt, and
    /// inheriting the developer's environment would have the suite depositing them in a real
    /// profile directory — outside the worktree, which no test may touch.
    fn dorc(&self, at: &Path) -> Command {
        let mut command = Command::new(&self.dorc);
        // A case that DEFINES a code asserts that its own drives emitted it, and a suite-wide
        // profile makes that a claim about every other case too: the durable store the harness
        // shares accumulates a document per drive, so a run reading it back is reading the suite.
        // Such a case therefore gets the profile its materialization laid down beside it, and
        // every other case keeps the shared one it has always had.
        let own = at.join(OWN_PROFILE_DIR);
        if own.is_dir() {
            sandbox::apply_roots_under(&mut command, &own);
        } else {
            self.profile.apply(&mut command);
        }
        // THE ANALYSIS CWD (`30I:rul-dot-resolves-as-sh`), and it is the CASE DIRECTORY — the shape
        // an admin gets by running `dorc` where their book and oracles are. Pinned rather than
        // inherited: cargo sets a test process.s cwd to the PACKAGE root, under which no case.s
        // `. ./helpers.sh` names anything, and a loom case materialized into a scratch directory
        // would resolve differently again. It is a separate question from the EXECUTION cwd, which
        // stays the throwaway sandbox `rail` supplies.
        command.current_dir(at);
        command.env(FIXTURE_CLOCK_ENV, FIXTURE_CLOCK_MS.to_string());
        // The battery reads a RENDER while the artifact SET goes to `--artifact-dir`, which is the
        // TERMINAL cell; left to the true answer it would be the kept-stream one, where naming a
        // directory claims the artifact twice and the run refuses before rendering anything.
        command.env(STDOUT_POSTURE_ENV, "interactive");
        // `real-tools-lane-opt-in`: zero external invocations; and no transcript may flip with
        // whether the developer.s TMPDIR sits inside a repository.
        command.env("DORC_FIXTURE_SOURCE_MATCH", "off");
        command
    }

    /// Syntax-check one artifact: `printf '%s\n' "$art" | $checker -n`.
    fn syntax_error(&self, artifact: &str) -> Option<String> {
        let scratch = Scratch::new("syn");
        let script = scratch.path.join("artifact.sh");
        std::fs::write(&script, format!("{artifact}\n")).expect("write artifact");
        let out = capture(
            Command::new(&self.checker)
                .arg("-n")
                .stdin(Stdio::from(std::fs::File::open(&script).unwrap()))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped()),
        );
        (out.code != 0).then(|| strip_trailing_newlines(&out.stderr))
    }

    /// Build an artifact-execution command under the determinism rail: a fixed
    /// environment (`env -i` ⇒ `env_clear`, then exactly `PATH`/`DORC_LOG`/`LC_ALL`/`TZ`),
    /// cwd = a throwaway sandbox, and `PATH` = the case's mocks alone.
    ///
    /// The `umask 022` pin rides an `sh -c 'umask 022; exec …'` shim, because Rust cannot
    /// set a process umask without FFI and `unsafe` is `forbid`-den workspace-wide. That
    /// shim is UNIX-ONLY: Windows has no umask to pin, and an msys shell cannot `exec` the
    /// Windows-form interpreter path this process resolved.
    fn rail(
        &self,
        sandbox: &Path,
        log: &Path,
        path: &std::ffi::OsStr,
        script: Option<&Path>,
    ) -> Command {
        Self::rail_under(&self.checker.clone(), sandbox, log, path, script)
    }

    /// [`Self::rail`] under a NAMED shell rather than the harness's own checker — the floor
    /// differential's seat (gate-9), where which binary runs the text IS the measurement.
    fn rail_under(
        shell: &Path,
        sandbox: &Path,
        log: &Path,
        path: &std::ffi::OsStr,
        script: Option<&Path>,
    ) -> Command {
        let mut command = Command::new(shell);
        command
            .current_dir(sandbox)
            .env_clear()
            .env("PATH", path)
            .env("DORC_LOG", log)
            .env("LC_ALL", "C")
            .env("TZ", "UTC");
        if cfg!(unix) {
            let shell = shell.display().to_string();
            match script {
                Some(script) => {
                    command
                        .arg("-c")
                        .arg("umask 022; exec \"$0\" \"$1\"")
                        .arg(&shell)
                        .arg(script);
                }
                None => {
                    command.arg("-c").arg("umask 022; exec \"$0\"").arg(&shell);
                }
            }
        } else if let Some(script) = script {
            command.arg(script);
        }
        command
    }

    /// Run a payload under the determinism rail and echo the shims' logged argvs, one per
    /// line, in execution order (`capture_run`). `payload` is either a script path or the
    /// artifact text; `mocks` becomes the child's ENTIRE `PATH`.
    ///
    /// `run_root` is the cwd the payload runs in. `None` is an EMPTY throwaway sandbox — the
    /// flattened form's world, where the artifact is the whole product and nothing beside it
    /// exists. A multipart case names its PUBLISHED generation instead, which is where `30I` §7.6
    /// says a multipart artifact executes (`cd <artifact> && sh ./plan.sh`). Copying the case's own
    /// authored sources into the sandbox would be the third option and is deliberately not offered:
    /// it would green a case against controller-side files the target never receives, which proves
    /// nothing about what the artifact ships.
    fn capture_run(&self, payload: Payload<'_>, mocks: &Path, run_root: Option<&Path>) -> String {
        let scratch = Scratch::new("run");
        let log = scratch.path.join("dorc.log");
        std::fs::write(&log, "").expect("seed log");
        let own = scratch.path.join("sand");
        std::fs::create_dir_all(&own).expect("create sandbox");
        let sandbox = run_root.unwrap_or(&own).to_path_buf();

        let piped = match payload {
            Payload::File(_) => None,
            Payload::Text(text) => {
                let piped = scratch.path.join("payload.sh");
                std::fs::write(&piped, format!("{text}\n")).expect("write payload");
                Some(piped)
            }
        };
        let script = match payload {
            Payload::File(script) => Some(script),
            Payload::Text(_) => None,
        };
        let mut command = self.rail(&sandbox, &log, mocks.as_os_str(), script);
        if let Some(file) = &piped {
            command.stdin(Stdio::from(std::fs::File::open(file).unwrap()));
        }
        let _ = capture(command.stdout(Stdio::null()).stderr(Stdio::null()));

        let logged = read_or_empty(&log);
        strip_trailing_newlines(
            &logged
                .lines()
                .map(|line| line.strip_prefix("ran: ").unwrap_or(line))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

/// Run `text` under one NAMED floor shell on the determinism rail and return its STDOUT
/// (gate-9). The sibling of [`Harness::capture_run`], which reads the mock run-LOG: a sentinel
/// manifest's whole product is what it printed, and nothing it prints comes from a shim.
///
/// The sandbox is a COPY of the case's own top-level files, because a load-order manifest's
/// whole subject is `. ./defs.sh` and a manifest that cannot find what it sources measures
/// nothing. A copy rather than the case dir itself: the throwaway-cwd rule is what keeps a
/// misbehaving manifest from writing into the corpus.
///
/// PATH is the mocks PLUS the floor binary's own userland, and it has to be: measured, `printf` is
/// a BUILTIN in dash 0.5.12 and an EXTERNAL command in posh 0.14.1, so under the corpus's ordinary
/// mocks-only PATH a posh manifest emits nothing at all and every case would read as a floor
/// disagreement. This is the opt-in real-binary lane, so the widening is in character — but it is
/// this lane's alone, and the rest of the rail (cleared env, sandbox cwd) is intact.
fn capture_floor_stdout(shell: &Path, text: &str, case: &Path, mocks: &Path) -> String {
    {
        let scratch = Scratch::new("floor");
        let log = scratch.path.join("dorc.log");
        std::fs::write(&log, "").expect("seed log");
        let sandbox = scratch.path.join("sand");
        std::fs::create_dir_all(&sandbox).expect("create sandbox");
        for entry in std::fs::read_dir(case).into_iter().flatten().flatten() {
            if entry.file_type().is_ok_and(|t| t.is_file()) {
                let _ = std::fs::copy(entry.path(), sandbox.join(entry.file_name()));
            }
        }
        let script = scratch.path.join("manifest.sh");
        std::fs::write(&script, format!("{text}\n")).expect("write manifest");
        let path = shell.parent().map_or_else(
            || mocks.as_os_str().to_owned(),
            |dir| {
                std::env::join_paths([mocks.to_path_buf(), dir.to_path_buf()])
                    .unwrap_or_else(|_| mocks.as_os_str().to_owned())
            },
        );
        let out = capture(
            Harness::rail_under(shell, &sandbox, &log, &path, Some(&script))
                .stdout(Stdio::piped())
                .stderr(Stdio::null()),
        );
        strip_trailing_newlines(&strip_cr(&out.stdout))
    }
}

/// What [`Harness::capture_run`] executes.
#[derive(Clone, Copy)]
enum Payload<'a> {
    /// A script path (the bare book).
    File(&'a Path),
    /// Artifact text fed on stdin.
    Text(&'a str),
}

// ---------------------------------------------------------------------------
// gate-2: the redirection scanner (port of `scan_redirects.awk`)

/// Strip an unquoted trailing `#` comment so provenance text cannot trip the scan.
fn strip_comment(line: &str) -> &str {
    let (mut in_single, mut in_double, mut prev_blank) = (false, false, true);
    for (index, ch) in line.char_indices() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '#' if !in_single && !in_double && prev_blank => return &line[..index],
            _ => {}
        }
        prev_blank = ch == ' ' || ch == '\t';
    }
    line
}

/// Is `word` an unsafe redirect target (absolute, dynamic, or sandbox-escaping)?
fn unsafe_target(word: &str) -> bool {
    if word == "/dev/null" || word.starts_with('&') {
        return false;
    }
    if word.contains('$') || word.contains('`') || word.starts_with('/') {
        return true;
    }
    word.split('/').any(|segment| segment == "..")
}

/// Lines carrying a redirect the exec sandbox refuses (`gate-2`, `20B` §3).
fn scan_redirects(artifact: &str) -> Vec<String> {
    let mut bad = Vec::new();
    for raw in lines_of(artifact) {
        let line = strip_comment(raw);
        let bytes: Vec<char> = line.chars().collect();
        let (mut in_single, mut in_double) = (false, false);
        let mut i = 0;
        while i < bytes.len() {
            let ch = bytes[i];
            if ch == '\'' && !in_double {
                in_single = !in_single;
                i += 1;
                continue;
            }
            if ch == '"' && !in_single {
                in_double = !in_double;
                i += 1;
                continue;
            }
            if in_single || in_double {
                i += 1;
                continue;
            }
            if ch == '>' || ch == '<' {
                let mut j = i + 1;
                if bytes.get(j) == Some(&ch) {
                    j += 1;
                }
                while matches!(bytes.get(j), Some(' ' | '\t')) {
                    j += 1;
                }
                let mut word = String::new();
                while let Some(&cc) = bytes.get(j) {
                    if matches!(cc, ' ' | '\t' | ';' | '|' | ')' | '(' | '>' | '<') {
                        break;
                    }
                    if cc == '&' && !word.is_empty() {
                        break;
                    }
                    word.push(cc);
                    j += 1;
                }
                if !word.is_empty() && unsafe_target(&word) {
                    bad.push(raw.to_owned());
                }
                i = j;
                continue;
            }
            i += 1;
        }
    }
    bad
}

// ---------------------------------------------------------------------------
// gate-1 parity normalizer (port of `norm_parity.awk`)

/// The sites whose AUTHORED record carries an `rc=` (pass 1 of `norm_parity.awk`).
fn rc_bearing_sites(authored: &str) -> BTreeSet<String> {
    let mut sites = BTreeSet::new();
    for line in authored.lines() {
        let mut fields = line.split_whitespace();
        if fields.next() != Some("site") {
            continue;
        }
        let Some(site) = fields.next() else { continue };
        if let Some(tail) = line.rsplit(' ').next()
            && tail.strip_prefix("rc=").is_some_and(|n| {
                !n.is_empty()
                    && n.trim_start_matches('-')
                        .chars()
                        .all(|c| c.is_ascii_digit())
                    && n.matches('-').count() <= 1
                    && !n.starts_with("--")
            })
        {
            sites.insert(site.to_owned());
        }
    }
    sites
}

/// Strip a trailing `rc=<n>` from every `site` record whose site is not rc-bearing.
fn norm_parity(records: &str, rc_sites: &BTreeSet<String>) -> String {
    lines_of(records)
        .iter()
        .map(|line| {
            let mut fields = line.split_whitespace();
            if fields.next() != Some("site") {
                return (*line).to_owned();
            }
            let Some(site) = fields.next() else {
                return (*line).to_owned();
            };
            if rc_sites.contains(site) {
                return (*line).to_owned();
            }
            match line.rsplit_once(" rc=") {
                Some((head, tail))
                    if !tail.is_empty()
                        && tail
                            .trim_start_matches('-')
                            .chars()
                            .all(|c| c.is_ascii_digit())
                        && !tail.trim_start_matches('-').is_empty() =>
                {
                    head.to_owned()
                }
                _ => (*line).to_owned(),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// needle files (gate-3 / gate-7 / gate-hint / gate-8)

/// Is `slug` a live code in the generated catalog? The structural needle's validator
/// (`288:prop-structural-needles-only`) — the same table `dorc-loom` regenerates, so a case
/// and the catalog cannot drift apart in silence.
fn catalog_has_slug(slug: &str) -> bool {
    dorc_aid::catalog::owned_catalog()
        .iter()
        .any(|entry| entry.slug == slug)
}

/// Does one haystack line carry every ` && `-conjoined needle of `pattern`?
fn needle_lands(haystack: &[&str], pattern: &str) -> bool {
    pattern
        .split(" && ")
        .filter(|needle| !needle.is_empty())
        .fold(haystack.to_vec(), |candidates, needle| {
            candidates
                .into_iter()
                .filter(|line| line.contains(needle))
                .collect()
        })
        .iter()
        .any(|_| true)
}

/// The patterns of `decl` that no `haystack` line satisfies. Blank and `#`-comment lines
/// are skipped (`needles_missing`).
fn needles_missing(haystack: &str, decl: &Path) -> Vec<String> {
    let text = read_or_empty(decl);
    let lines = lines_of(haystack);
    text.lines()
        .filter(|pattern| !pattern.is_empty() && !pattern.starts_with('#'))
        .filter(|pattern| !needle_lands(&lines, pattern))
        .map(str::to_owned)
        .collect()
}

// ---------------------------------------------------------------------------
// gate-6: the dual-rail judge (pure — driven by the confound battery on fabricated input)

/// Ledger entry (may carry `TOP` wildcards) vs a concrete logged argv, word-by-word.
fn argv_words_match(ledger: &str, concrete: &str) -> bool {
    let ledger: Vec<&str> = ledger.split_whitespace().collect();
    let concrete: Vec<&str> = concrete.split_whitespace().collect();
    ledger.len() == concrete.len()
        && ledger
            .iter()
            .zip(&concrete)
            .all(|(want, got)| *want == "TOP" || want == got)
}

/// Every violation of the two dual-rail directions (empty ⇒ pass). `disp` is the RAW
/// `--debug-argv` readout; the replace/omit/guard license filter lives HERE so the
/// confound battery can prove a `run` disposition never attributes an elision.
fn dual_rail_judge(
    bare: &str,
    apply: &str,
    disp: &str,
    shims: &str,
    guard_cmds: &str,
) -> Vec<String> {
    let mut violations = Vec::new();
    // TWO rails, two decision identities: `argv` attributes a SITE, `region` an AUTHORED REGION.
    let ledger: Vec<&str> = lines_of(disp)
        .iter()
        .filter_map(|line| {
            let rest = line
                .strip_prefix("argv ")
                .or_else(|| line.strip_prefix("region "))?;
            let (id, rest) = rest.split_once(' ')?;
            if id.is_empty() || !id.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            let (verb, words) = rest.split_once(' ')?;
            matches!(verb, "replace" | "omit" | "guard").then_some(words)
        })
        .collect();
    let bare_lines = lines_of(bare);
    let apply_lines = lines_of(apply);
    let guard_lines = lines_of(guard_cmds);

    for line in &apply_lines {
        if line.is_empty() || bare_lines.contains(line) {
            continue;
        }
        let cmd0 = line.split(' ').next().unwrap_or_default();
        if guard_lines.contains(&cmd0) {
            continue;
        }
        violations.push(format!("apply-only (ran in apply, not in bare): {line}"));
    }
    for line in &bare_lines {
        if line.is_empty() || apply_lines.contains(line) {
            continue;
        }
        let cmd0 = line.split(' ').next().unwrap_or_default();
        if !shims.contains(&format!(" {cmd0} ")) {
            continue;
        }
        if !ledger
            .iter()
            .any(|entry| !entry.is_empty() && argv_words_match(entry, line))
        {
            violations.push(format!(
                "unattributable bare-only (elided with no replace/omit license): {line}"
            ));
        }
    }
    violations
}

// ---------------------------------------------------------------------------
// the guard-shape floor (pure)

/// Every guarded line violating `rul-ternary-verdict`'s artifact-shape law: a guard must
/// carry a `<check> || <original>` fall-through (never-1), and the bytes after the FIRST
/// ` || ` must be a verbatim book line (bytes-survive-verbatim).
fn guard_shape_violations(artifact: &str, book: &str) -> Vec<String> {
    let book_lines: Vec<&str> = book.lines().map(str::trim).collect();
    let mut out = Vec::new();
    for line in artifact.lines().filter(|line| line.contains("dorc: guard")) {
        let code = match line
            .find("# dorc: guard")
            .or_else(|| line.find("#dorc: guard"))
        {
            Some(at) => line[..at].trim_end(),
            None => line,
        };
        let Some((_, original)) = code.split_once(" || ") else {
            out.push(format!(
                "thin guard (no '|| <original>' fall-through — never-1: engine-synthesized sh in guard position): {code}"
            ));
            continue;
        };
        if !book_lines.contains(&original.trim()) {
            out.push(format!(
                "fall-through bytes not verbatim from book.sh (mutated original — e.g. a dropped flag): {original}"
            ));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// framed_results — the probe-results fixture, framed as the engine's own records stream

/// Build the `dorc-records/1` stream the round-trip consumes on stdin, from the case's
/// authored `probe-results.txt` plus the site-set the compiled probe self-reports.
///
/// Port of `run.sh`'s `framed_results`; computed once per case (it is a pure function of
/// the case dir and the shared argv, so every gate consuming it sees identical bytes).
fn framed_results(harness: &Harness, dir: &Path, args: &[String]) -> String {
    let probe = capture(
        harness
            .dorc(dir)
            .arg("probe")
            .arg(format!("--book={}", dir.join("book.sh").display()))
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;

    // The re-framing itself lives in `support` so the `309` §4 baseline re-frames IDENTICALLY;
    // this seat keeps the probe invocation, which is the half that differs per driver.
    support::frame_records(&probe, dir)
}

// ---------------------------------------------------------------------------
// per-case markers

/// Resolve a `NAME=<value>` marker file, refusing more than one (an ambiguous marker is
/// an authoring error, never a silently-picked one).
fn marker(dir: &Path, prefix: &str) -> Result<Option<String>, String> {
    let mut found: Vec<String> = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(None);
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(value) = name.strip_prefix(&format!("{prefix}=")) {
            found.push(value.to_owned());
        }
    }
    found.sort();
    match found.len() {
        0 => Ok(None),
        1 => Ok(found.into_iter().next()),
        _ => Err(format!("multiple {prefix}=<value> markers")),
    }
}

/// Is a bare presence-marker file (`XFAIL`, `PROBE_RESULTS=authored`, …) present?
fn has_marker(dir: &Path, name: &str) -> bool {
    dir.join(name).exists()
}

/// The one generation an `ARTIFACT_SET` case's run published under `root`.
///
/// EXACTLY one, and the exactness is the assertion: only the round-trip's own drive is given the
/// artifact stream, so a second generation would mean a second publication nobody asked for, and
/// none at all means the run took a form that materializes nothing — which is precisely the silent
/// fallback a case declaring an artifact set must not be allowed to pass under.
fn published_generation(root: &Path) -> Result<PathBuf, String> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("artifact-"))
        })
        .collect();
    found.sort();
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err(String::from(
            "the run published no artifact generation — it took a form that materializes nothing, so every exec gate below would have measured the plan alone",
        )),
        n => Err(format!(
            "the run published {n} artifact generations; exactly one drive is given the artifact stream"
        )),
    }
}

/// The world the COUNTERFACTUAL rails (gate-5's argv echo, gate-6's dual rail) run in when a case
/// publishes an artifact set: the case's own authored top-level files, with the published generation
/// laid over them.
///
/// Both halves are needed because those rails compare an ARTIFACT against the BOOK, and since
/// `30Ng:rul-bundle-at-dorc-lang-boundaries` the two no longer resolve their imports in one place:
/// the book names its author's files and the artifact names the bundles this run generated. Running
/// the pair in the generation alone silently reduced the bare rail to "the shell exited at the first
/// `.`", which reads as an apply that ran MORE than the book — a false finding about the engine.
///
/// It is NOT a relaxation of `an-artifact-set-runs-from-its-own-generation`: `exec_check` still runs
/// the PUBLISHED plan from the generation alone, and that is where the self-containment question is
/// asked. This world exists so a delta between two run-sets is a delta rather than a crash.
fn counterfactual_root(dir: &Path, generation: &Path, into: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(into)?;
    for source in [dir, generation] {
        for entry in std::fs::read_dir(source).into_iter().flatten().flatten() {
            if entry.file_type().is_ok_and(|kind| kind.is_file()) {
                std::fs::copy(entry.path(), into.join(entry.file_name()))?;
            }
        }
    }
    Ok(())
}

/// Every LITERAL relative `.`/`source` operand in a published plan that the generation does not
/// carry — the assertion that makes an artifact set observe its own tree
/// (`an-artifact-set-runs-from-its-own-generation`, widened past "exactly one generation exists").
///
/// LITERAL operands only, and the cut is the honest one: an operand the plan builds from a variable
/// is resolved by the target's own shell against values the artifact sets, which this seat holds no
/// model of. What it does cover is exactly the class an import REWRITE produces — a controller-decided
/// relative path naming a file the same run published — so a rewrite pointing at nothing, or a bundle
/// the publication dropped, reddens here instead of passing on stdout bytes alone
/// (`30Nf:fnd-multipart-never-placed-anything-in-production` is the burn this widens against).
fn unresolved_generated_imports(generation: &Path, plan: &str) -> Vec<String> {
    plan.lines()
        .filter_map(|line| {
            let rest = line
                .trim_start()
                .strip_prefix(". ")
                .or_else(|| line.trim_start().strip_prefix("source "))?;
            let operand = rest.split_whitespace().next()?;
            let operand = operand
                .strip_prefix('\'')
                .and_then(|inner| inner.strip_suffix('\''))
                .unwrap_or(operand);
            let relative = operand.strip_prefix("./")?;
            (!relative.contains(['$', '"', '\'', '`']) && !generation.join(relative).is_file())
                .then(|| operand.to_owned())
        })
        .collect()
}

// ---------------------------------------------------------------------------
// the tolerated-nondeterminism normalizers

/// The CLOSED, engine-owned normalizer vocabulary (`288:prop-normalizer-closed-vocabulary`).
///
/// A case DECLARES which named class of nondeterminism it tolerates; the named normalizer is
/// then applied IDENTICALLY at bless-capture and at check, so the committed bytes are the
/// canonical form and the declaration is the honesty disclosure. One named normalizer per
/// named class — never a free regex, and never a check-only relaxation (the shape the old
/// `RAN_ORDER=lax` marker had, which blessed raw bytes and compared sorted ones, so the
/// committed file recorded an interleaving nothing asserted).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Normalizer {
    /// Pipeline stages race each other into the run-log: the SET of argvs a pipeline runs is
    /// determined by the artifact, their interleaving is the kernel's scheduling. Canonical
    /// form: the run-log sorted (`strawman24-pipe-guard-floor` is the specimen).
    PipeStageOrder,
}

impl Normalizer {
    /// Resolve a declared name, refusing anything outside the vocabulary.
    fn parse(name: &str) -> Result<Self, String> {
        match name {
            "pipe-stage-order" => Ok(Normalizer::PipeStageOrder),
            _ => Err(format!(
                "unknown tolerated-nondeterminism class `{name}` — the vocabulary is CLOSED and engine-owned; mint a named normalizer, never a per-case relaxation"
            )),
        }
    }

    /// Canonicalize a captured run-log under this class.
    fn apply(self, log: &str) -> String {
        match self {
            Normalizer::PipeStageOrder => sort_lines(log),
        }
    }
}

/// What a case's `artifact-set:` frontmatter may say — a CLOSED vocabulary, on
/// [`Normalizer::parse`]'s precedent (`288:prop-normalizer-closed-vocabulary`).
///
/// The dir form spells this as a bare `ARTIFACT_SET` presence file, which a `.loom` cannot carry:
/// a loom's fixture space is txtar sections, and a marker with no content has no section. So the
/// loom form declares a VALUE, and the value is checked rather than ignored — an unread declaration
/// is exactly the silence `30Nf:dev-artifact-set-is-dir-form-only` left open, on the very case
/// minted to demonstrate the capability.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ArtifactSetDeclaration {
    /// The run gets its own artifact directory and must publish exactly one generation there.
    Published,
}

impl ArtifactSetDeclaration {
    /// Resolve a declared value, refusing anything outside the vocabulary.
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "published" => Ok(ArtifactSetDeclaration::Published),
            _ => Err(format!(
                "unknown `artifact-set: {value}` — the vocabulary is CLOSED; `published` is the \
                 only value, and it means the run's artifact set is published to a directory of \
                 its own and the exec gates run from that generation"
            )),
        }
    }
}

/// The normalizers a case declares, from its `tolerate=<comma-list>` marker.
fn tolerances(dir: &Path) -> Result<Vec<Normalizer>, String> {
    let declared = marker(dir, "tolerate")?;
    declared
        .as_deref()
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(Normalizer::parse)
        .collect()
}

/// Apply every declared normalizer, in vocabulary order.
fn canonicalize(log: &str, tolerated: &[Normalizer]) -> String {
    tolerated
        .iter()
        .fold(log.to_owned(), |log, class| class.apply(&log))
}

// ---------------------------------------------------------------------------
// loom-form cases (`288` §7 — the whole-product transcript)

/// What a loom-form case's `run:` frontmatter selects.
#[derive(Clone, Copy, PartialEq, Eq)]
enum LoomRun {
    /// The whole-pipeline round-trip; the replay output is the two rendered artifacts.
    RoundTrip,
    /// A `dorc lint` case; the replay output is the lint render.
    Lint,
}

/// A `.loom` whose frontmatter declares an executable shape (`288` §7).
///
/// The loom is the AUTHORING surface — one file, sections instead of a dir of fixtures,
/// frontmatter instead of `NAME=value` marker files, and the committed transcript instead of
/// `expected.out`. It is not a second harness: the case MATERIALIZES into exactly the dir
/// shape every gate below already speaks, so the whole battery (the `dash -n` floor,
/// exec-under-mocks, gate-1(a)–(d), gate-2, gate-3, gate-5, gate-6, the needle gates, the
/// guard-shape floor) runs over it unchanged and the conversion cannot quietly drop a check.
///
/// The looms runner (`tests/looms.rs`) parses and hygiene-checks the same file; a round-trip
/// loom declares `fixpoint: executed`, because its transcript is proven by running the real
/// binary HERE rather than by the in-process renderer there.
struct LoomCaseSpec {
    /// The trial name (the loom's slug).
    name: String,
    /// The `.loom` path, for bless write-back.
    path: PathBuf,
    /// The parsed case.
    case: errorloom::Case,
    /// Which driver owns it.
    run: LoomRun,
}

/// The frontmatter keys a loom-form case may carry, and the dir-form artifact each becomes.
/// Anything else is refused — an unread key is a silently-ineffective assertion.
///
/// A PROJECTION of `dorc_loom::FRONTMATTER_KEYS` rather than a second list: the looms runner sees
/// whole-product cases too, so a key this runner accepted and that one did not would refuse the
/// same file from the other side. `owns` is read by neither runner — it is the prose-ownership
/// resolver's (`dorc_loom::corpus_ownership`), which scans EVERY collection — and is in the subset
/// because THIS runner's refusal is what a whole-product case's author meets, and refusing the key
/// here left a prose-component rendered only by this collection with no authoring home at all
/// (`28L:rul-ownership-declaration-adopted`).
fn loom_keys() -> Vec<&'static str> {
    dorc_loom::run_lane_key_names()
}

/// Scalar-or-list frontmatter items (an absent key is the empty list).
fn loom_items(case: &errorloom::Case, key: &str) -> Vec<String> {
    match case.frontmatter().get(key) {
        Some(errorloom::FrontmatterValue::Scalar(one)) => vec![one.clone()],
        Some(errorloom::FrontmatterValue::List(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// Read and classify one `.loom`, or `Ok(None)` when it carries no `run:` key (an ordinary
/// catalog case, the looms runner's).
fn loom_spec(case: &LoomCase) -> Result<Option<LoomCaseSpec>, String> {
    let text = std::fs::read_to_string(&case.path)
        .map_err(|error| format!("read {}: {error}", case.path.display()))?;
    let parsed = errorloom::Case::parse(&text).map_err(|error| format!("{error}"))?;
    let declared = parsed.frontmatter().scalar("run").map(str::to_owned);
    let executed = parsed.frontmatter().scalar("fixpoint") == Some("executed");
    let Some(declared) = declared else {
        return if executed {
            // Without a `run:` key nothing executes this case, so `fixpoint: executed` would
            // hand its transcript to a driver that never runs — no proof at all.
            Err(format!(
                "`{}` declares `fixpoint: executed` but no `run:` — nothing would ever execute it",
                case.name
            ))
        } else {
            Ok(None)
        };
    };
    let run = match declared.as_str() {
        "round-trip" => LoomRun::RoundTrip,
        "lint" => LoomRun::Lint,
        other => return Err(format!("unknown `run: {other}`")),
    };
    if let Some(unknown) = parsed
        .frontmatter()
        .keys()
        .find(|key| !dorc_loom::is_run_lane_key(key))
    {
        return Err(format!(
            "unread frontmatter key `{unknown}` — the key vocabulary is closed, and a key no gate \
             reads is an assertion the author only believes they made. A whole-product case reads \
             {}; `dorc-loom keys` says what each one is read by",
            loom_keys().join(", ")
        ));
    }
    Ok(Some(LoomCaseSpec {
        name: case.name.clone(),
        path: case.path.clone(),
        case: parsed,
        run,
    }))
}

/// Materialize a loom-form case into the dir shape the gates read.
fn materialize_loom(spec: &LoomCaseSpec, into: &Path) -> Result<(), String> {
    for (rel, content) in spec.case.materialized_files() {
        let target = into.join(&rel);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| format!("{error}"))?;
        }
        std::fs::write(&target, content).map_err(|error| format!("{error}"))?;
        make_executable(&target);
    }
    let write = |name: &str, body: String| -> Result<(), String> {
        std::fs::write(into.join(name), body).map_err(|error| format!("{error}"))
    };
    let scalar = |key: &str| spec.case.frontmatter().scalar(key).map(str::to_owned);
    if let Some(flags) = scalar("flags") {
        write(&format!("DORC_FLAGS={flags}"), String::new())?;
    }
    if let Some(value) = scalar("tolerate") {
        write(&format!("tolerate={value}"), String::new())?;
    }
    if let Some(value) = scalar("artifact-set") {
        match ArtifactSetDeclaration::parse(&value)? {
            ArtifactSetDeclaration::Published => write("ARTIFACT_SET", String::new())?,
        }
    }
    if let Some(value) = scalar("probe-results") {
        write(&format!("PROBE_RESULTS={value}"), String::new())?;
    }
    if let Some(value) = scalar("dual-rail") {
        write(&format!("DUAL_RAIL={value}"), String::new())?;
    }
    if let Some(value) = scalar("why-addr") {
        write(&format!("WHY_ADDR={value}"), String::new())?;
    }
    if let Some(value) = scalar("apply-exit") {
        write(&format!("EXIT_RC={value}"), String::new())?;
    }
    match spec.run {
        LoomRun::RoundTrip => {
            if let Some(value) = scalar("exit") {
                write(&format!("DORC_EXIT={value}"), String::new())?;
            }
        }
        LoomRun::Lint => {
            write(
                "expected-rc",
                format!("{}\n", scalar("exit").unwrap_or_default()),
            )?;
            write("cmd", format!("{}\n", lint_flags(spec)?))?;
        }
    }
    for (key, file) in [
        ("expect-diagnostic", "expected-diagnostics"),
        ("expect-why", "expected-why"),
        ("expect-hint", "expected-hint"),
        ("expect-why-chain", "expected-why-chain"),
    ] {
        let items = loom_items(&spec.case, key);
        if !items.is_empty() {
            write(file, format!("{}\n", items.join("\n")))?;
        }
    }
    let transcript = spec
        .case
        .replay()
        .blocks()
        .first()
        .ok_or_else(|| String::from("no replay block"))?;
    write("expected.out", transcript.output().to_owned())?;
    Ok(())
}

/// Give a materialized mock the execute bit `PATH` resolution needs on unix (txtar carries no
/// mode). A no-op on Windows, where `PATH` resolution does not consult it.
#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    if let Ok(meta) = std::fs::metadata(path) {
        let mut perms = meta.permissions();
        perms.set_mode(perms.mode() | 0o111);
        let _ = std::fs::set_permissions(path, perms);
    }
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

/// The lint flag vector a `run: lint` loom's replay command carries, minus the `dorc lint`
/// head and the trailing book path the driver appends itself. Deriving the `cmd` file from the
/// COMMAND is what keeps the committed transcript's first line honest: the flags a reader sees
/// are the flags the case runs.
fn lint_flags(spec: &LoomCaseSpec) -> Result<String, String> {
    let command = spec
        .case
        .replay()
        .blocks()
        .first()
        .ok_or_else(|| String::from("no replay block"))?
        .command();
    let words: Vec<&str> = command.split_whitespace().collect();
    match words.split_first() {
        Some((&"dorc", rest)) if rest.first() == Some(&"lint") => {}
        _ => return Err(format!("`{command}` is not a `dorc lint` invocation")),
    }
    if words.last() != Some(&"book.sh") {
        return Err(format!(
            "`{command}` must end in the book path the driver appends (`book.sh`)"
        ));
    }
    let mut flags: Vec<&str> = Vec::new();
    for word in words.iter().skip(2).take(words.len().saturating_sub(3)) {
        if !word.starts_with('-') {
            return Err(format!("`{command}` carries a non-flag word `{word}`"));
        }
        flags.push(word);
    }
    Ok(flags.join(" "))
}

/// T1, the closed loop (`26D` §5): probe REALLY executed, its REAL captured output admitted, a
/// plan built from it — with no authored `probe-results.txt` anywhere in that chain.
///
/// Both halves existed already and had never been joined: `probe_exec_check` runs a real probe but
/// only COMPARES its output against an authored fixture, and `exec_check` runs an apply built from
/// that same authored fixture. Captured-real-probe-output feeding a plan build existed nowhere, so
/// nothing proved the fixtures describe what a probe actually emits.
///
/// The closing assertion is the byte-comparison: the plan built from REAL captured output must
/// equal the plan the ordinary fixture-fed run produces. A divergence means the corpus has been
/// grading itself against a description of the world rather than the world, and IS the finding.
///
/// Hermetic throughout — the local driver is a shell, the shipped probe sees only the case's inert
/// mocks, and no socket is opened.
fn run_closed_loop(harness: &Harness, dir: &Path, mocks: &Path) -> Result<(), Failed> {
    let scratch = Scratch::new("loop");
    let sandbox = scratch.path.join("sand");
    std::fs::create_dir_all(&sandbox).expect("create sandbox");

    let args = shared_args(dir).map_err(Failed::from)?;
    let shim_dir = scratch.path.join("shims");
    std::fs::create_dir_all(&shim_dir).expect("create shim dir");
    let probe_path = std::env::join_paths([mocks, shim_dir.as_path()]).expect("join probe PATH");

    let mut shipped = harness.dorc(dir);
    shipped
        .current_dir(&sandbox)
        .env(
            "DORC_TRANSPORT",
            format!("local:{}", harness.checker.display()),
        )
        .env(
            "DORC_TRANSPORT_INTERPRETER",
            if cfg!(windows) {
                format!("/usr/bin/{}", harness.checker_name)
            } else {
                harness.checker.display().to_string()
            },
        )
        .env("PATH", &probe_path)
        .arg("plan")
        .arg(format!("--host={CLOSED_LOOP_HOST}"))
        .arg(format!("--shim-dir={}", shim_dir.display()))
        .arg(format!("--book={}", dir.join("book.sh").display()))
        .args(&args);
    let shipped = capture(shipped.stdout(Stdio::piped()).stderr(Stdio::piped()));
    if shipped.code != 0 {
        return Err(Failed::from(format!(
            "closed loop: `dorc plan --host` exited {} — a real probe never made it back through admission\n{}",
            shipped.code,
            shipped.stderr.trim()
        )));
    }
    if shipped.stdout.trim().is_empty() {
        return Err(Failed::from(
            "closed loop: no apply artifact — a real probe ran but produced no admissible plan"
                .to_owned(),
        ));
    }
    if let Some(error) = harness.syntax_error(&shipped.stdout) {
        return Err(Failed::from(format!(
            "closed loop: the plan built from REAL probe output is not runnable\n{}",
            error.trim()
        )));
    }

    let framed = scratch.path.join("framed-results.txt");
    std::fs::write(&framed, framed_results(harness, dir, &args)).expect("write framed");
    let results = std::fs::File::open(&framed).expect("open framed results");

    let mut fixture_fed = harness.dorc(dir);
    fixture_fed
        .current_dir(&sandbox)
        .arg("plan")
        .arg(format!("--shim-dir={}", shim_dir.display()))
        .arg(format!("--book={}", dir.join("book.sh").display()))
        .args(args_reading_stdin_records(&args));
    let fixture_fed = capture(
        fixture_fed
            .stdin(Stdio::from(results))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
    if fixture_fed.code != 0 {
        return Err(Failed::from(format!(
            "closed loop: the fixture-fed comparison run exited {} — the loop cannot be compared \
             against a run that did not happen\n{}",
            fixture_fed.code,
            fixture_fed.stderr.trim()
        )));
    }

    if strip_cr(&shipped.stdout) != strip_cr(&fixture_fed.stdout) {
        return Err(Failed::from(format!(
            "closed loop: the plan from REAL probe output differs from the plan the authored \
             fixture produces\n--- from a real probe ---\n{}\n--- from the fixture ---\n{}",
            shipped.stdout.trim(),
            fixture_fed.stdout.trim()
        )));
    }
    Ok(())
}

/// The destination the closed loop names. Never resolved — the local driver replaces the ssh
/// invocation wholesale, and `.invalid` is reserved by RFC 2606, so a regression that reached the
/// network would fail rather than contact anything.
const CLOSED_LOOP_HOST: &str = "closed-loop.invalid";

/// The case the closed loop drives. Chosen for shape, not verdicts: it carries inert mocks AND an
/// authored fixture, which is what makes the real-vs-fixture comparison possible at all.
const CLOSED_LOOP_CASE: &str = "context-entry-babby-elides";

/// The canonical round-trip invocation for a materialized case: exactly what the runner drives,
/// rendered as the transcript's command line. The committed command must EQUAL this, so a
/// transcript can never show one invocation while the gates run another.
fn round_trip_command(dir: &Path) -> String {
    let mut command = String::from("dorc --book=book.sh");
    let mut oracles: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".oracle.sh"))
        .collect();
    oracles.sort();
    for oracle in oracles {
        let _ = write!(command, " --pre-source {oracle}");
    }
    if let Ok(Some(flags)) = marker(dir, "DORC_FLAGS") {
        let _ = write!(command, " {flags}");
    }
    // DISCLOSED: naming an artifact stream decides which FORM the run takes. A shell VARIABLE
    // rather than the scratch path, because a committed transcript must be a fixpoint.
    if has_marker(dir, "ARTIFACT_SET") {
        command.push_str(" --artifact-dir=$ARTIFACT_DIR");
    }
    // `owed-no-flag-defaults-to-stdin`: the records lane no longer takes stdin unless a `-` names
    // it, and the runner always feeds a framed stream -- so the invocation says so, always.
    command.push_str(" --results -");
    if dir.join("probe-results.txt").is_file() {
        command.push_str(" < probe-results.txt");
    }
    command
}

/// Drive one loom-form case: materialize, run the dir-form battery, then fold any blessed
/// bytes back into the `.loom` (the loom, not the scratch dir, is what is committed).
fn run_loom(harness: &Harness, spec: &LoomCaseSpec) -> Result<(), Failed> {
    // Before materialization, so a refused floor cell leaves the committed `.loom` untouched:
    // `bless_loom` below writes the whole file back, gates or no gates.
    let carries_manifest = spec
        .case
        .sections()
        .iter()
        .any(|section| section.name() == "expected.emitted");
    if let Some(line) = floor_bless_refusal(
        harness.bless,
        harness.bless_floor,
        carries_manifest,
        &spec.name,
    ) {
        return Err(line.into());
    }
    let scratch = Scratch::new("loom");
    let dir = scratch.path.join(&spec.name);
    std::fs::create_dir_all(&dir).expect("create loom case dir");
    materialize_loom(spec, &dir)
        .map_err(|error| Failed::from(format!("FAIL  {}  [loom: {error}]", spec.name)))?;
    if spec.case.frontmatter().scalar("code").is_some() {
        for role in ["config", "state", "home"] {
            std::fs::create_dir_all(dir.join(OWN_PROFILE_DIR).join(role))
                .expect("create the case's own profile");
        }
    }

    let case = E2eCase {
        name: spec.name.clone(),
        dir: dir.clone(),
        kind: match spec.run {
            LoomRun::RoundTrip => E2eKind::RoundTrip,
            LoomRun::Lint => E2eKind::Lint,
        },
    };
    if spec.run == LoomRun::RoundTrip {
        let want = round_trip_command(&dir);
        let got = spec.case.replay().blocks()[0].command();
        if got != want {
            return Err(format!(
                "FAIL  {}  [loom: the committed replay command is not the invocation the gates drive]\n      committed: {got}\n      drives:    {want}",
                spec.name
            )
            .into());
        }
    }
    let mut stderr = String::new();
    let outcome = match spec.run {
        LoomRun::RoundTrip => run_round_trip(harness, &case, &mut stderr),
        LoomRun::Lint => run_lint(harness, &case),
    };
    let (extra, mut extra_failures) = drive_extra_replays(harness, spec, &dir);
    stderr.push_str(&extra.stderr);
    extra_failures.extend(defined_code_fired(spec, &stderr));
    // A FAILING case folds nothing. Safe because no bless workflow depends on partial folding:
    // every gate that compares against a bless-WRITTEN golden is already bless-aware and cannot
    // fail on staleness — the content diff and the extra-replay compare are `!bless`-guarded,
    // `exec_check`/`run_lint`/gate-9 write-and-return before theirs. What remains reachable under
    // bless is structural (`-n`, crash/empty, guard-shape, redirects), authored-fixture (gate-1's
    // `probe-results.txt`, the needle declarations), or environmental — none of it healable by a
    // write, and gate-1 says so in its own message. Ungated, `exec_check`'s early `expected.ran`
    // write would fold into a case whose transcript a later gate had just failed. XFAIL is
    // untouched: the lens returns `Ok` above, so its deliberate golden-text-blindness survives.
    if harness.bless && outcome.is_ok() {
        bless_loom(spec, &dir, &extra.outputs, harness.bless_floor)?;
    }
    match (outcome, extra_failures.is_empty()) {
        (Ok(()), true) => Ok(()),
        (Ok(()), false) => Err(extra_failures.join("\n").into()),
        (Err(failed), true) => Err(failed),
        (Err(failed), false) => Err(format!(
            "{}\n{}",
            failed.message().unwrap_or_default(),
            extra_failures.join("\n")
        )
        .into()),
    }
}

/// Drive replay blocks 1..N — the SAME input-state seen through other invocations
/// (`282:rul-multi-replay-per-case`) — sequentially in the one materialized dir that block 0's
/// battery just ran in. Sequential-in-one-dir is the whole point: a `--whylog-dir` run and the
/// `dorc why … --last` that replays it are two blocks sharing scratch state.
///
/// Each command is driven with `cwd` = the case dir and its committed words verbatim, so the
/// invocation a reader sees IS the one that ran, and every path a render echoes back stays
/// case-relative (an absolute host path in a transcript is not committable).
///
/// Returns the captured outputs for blocks 1..N (empty when anything failed, so bless leaves
/// the committed bytes alone) and the failure lines.
fn drive_extra_replays(
    harness: &Harness,
    spec: &LoomCaseSpec,
    dir: &Path,
) -> (ExtraReplays, Vec<String>) {
    let blocks = spec.case.replay().blocks();
    if blocks.len() < 2 {
        return (ExtraReplays::default(), Vec::new());
    }
    let name = &spec.name;
    let mut failures: Vec<String> = Vec::new();
    let args = match shared_args(dir) {
        Ok(args) => args,
        Err(message) => {
            return (
                ExtraReplays::default(),
                vec![format!("FAIL  {name}  [replay: {message}]")],
            );
        }
    };
    let scratch = Scratch::new("replay");
    let framed_path = scratch.path.join("framed.txt");
    std::fs::write(&framed_path, framed_results(harness, dir, &args)).expect("write framed");

    let mut outputs: Vec<String> = Vec::new();
    let mut stderr = String::new();
    for (index, block) in blocks.iter().enumerate().skip(1) {
        match run_replay_block(harness, dir, &framed_path, block.command()) {
            Ok(got) if scratch_path_leaked(&got.transcript, dir) => failures.push(format!(
                "FAIL  {name}  [replay {index}: `{}` echoed the throwaway materialization path — a transcript carrying a machine-specific absolute path is not committable (`282` §7); spell the invocation with case-relative paths]",
                block.command()
            )),
            Ok(got) => {
                if !harness.bless && got.transcript != block.output() {
                    failures.push(format!(
                        "FAIL  {name}  [replay {index}: `{}` no longer reproduces its committed transcript]\n{}",
                        block.command(),
                        divergence(
                            &strip_trailing_newlines(block.output()),
                            &strip_trailing_newlines(&got.transcript)
                        )
                    ));
                }
                stderr.push_str(&got.stderr);
                outputs.push(got.transcript);
            }
            Err(message) => failures.push(format!(
                "FAIL  {name}  [replay {index}: `{}` — {message}]",
                block.command()
            )),
        }
    }
    let extra = ExtraReplays { outputs, stderr };
    if failures.is_empty() {
        (extra, failures)
    } else {
        (ExtraReplays::default(), failures)
    }
}

/// A whole-product case's `code:` is an ASSERTION that one of its own drives emitted that code.
///
/// This is what lets a diagnostic whose world only a real run can build own its catalog row: the
/// key that MINTS the row is the key checked here, so the owner and the proof have one source and
/// a slug coincidence cannot stand in for either. The declaration is validated against the
/// generated catalog exactly as `expected-diagnostics` is, so a dead slug is refused rather than
/// asserting nothing forever.
///
/// Any severity counts — severity is registry data a case does not restate — and the whole case's
/// stderr is the haystack, because which drive provokes a diagnostic is the case's business.
fn defined_code_fired(spec: &LoomCaseSpec, stderr: &str) -> Vec<String> {
    let name = &spec.name;
    let Some(slug) = spec.case.frontmatter().scalar("code") else {
        return Vec::new();
    };
    if !catalog_has_slug(slug) {
        return vec![format!(
            "FAIL  {name}  [code: `{slug}` is not a code in the generated catalog — a whole-product case defines a live code, or it defines nothing]"
        )];
    }
    if ["error", "warning", "note"]
        .iter()
        .any(|severity| stderr.contains(&format!("{severity}[{slug}]")))
    {
        return Vec::new();
    }
    // The stderr comes with the refusal because it is the whole diagnosis: an author looking at
    // this is asking which diagnostic their world DID produce, and every gate above this one has
    // already thrown that stream away.
    vec![format!(
        "FAIL  {name}  [code: `{slug}` is what this case DEFINES, and no drive of it emitted that code — a defining case whose own run does not fire it defines a row nothing proves]\n{}",
        indent(&stderr.lines().map(str::to_owned).collect::<Vec<_>>())
    )]
}

/// What blocks 1..N produced, across the drives.
#[derive(Default)]
struct ExtraReplays {
    /// Per-block stdout, for the bless fold; empty when anything failed.
    outputs: Vec<String>,
    /// Every drive's stderr, concatenated — a diagnostic belongs to the CASE, not to whichever
    /// block happened to provoke it.
    stderr: String,
}

/// What one replay drive produced: the bytes its block commits, and the bytes only a gate reads.
struct ReplayCapture {
    /// Stdout, as the committed transcript spells it.
    transcript: String,
    /// Stderr, which no transcript carries and the code-fired gate needs.
    stderr: String,
}

/// Did a render echo back the per-run materialization dir? Renders that quote an argv path
/// (the `why` heading does) turn an absolute invocation into bytes no other machine reproduces,
/// so `282` §7 refuses them at capture rather than committing them. Both separator spellings are
/// checked: Windows argv paths come back with the one the caller supplied.
fn scratch_path_leaked(output: &str, dir: &Path) -> bool {
    let native = dir.display().to_string();
    let slashed = native.replace('\\', "/");
    output.contains(&native) || output.contains(&slashed)
}

/// Execute one committed replay command and return its stdout as transcript bytes, beside the
/// stderr it wrote.
///
/// The accepted shape is deliberately tiny: a `dorc` invocation, optionally reading the case's
/// `probe-results.txt` (which resolves to the framed stream the battery feeds, exactly as block
/// 0's committed `< probe-results.txt` does) and optionally discarding stdout. Only STDOUT is
/// transcript — `stdout-contract` makes it the product surface — but stderr is kept rather than
/// dropped, because a diagnostic is a thing a later drive can be the only one to emit, and a gate
/// that cannot see it is a gate the case can never satisfy.
fn run_replay_block(
    harness: &Harness,
    dir: &Path,
    framed: &Path,
    command: &str,
) -> Result<ReplayCapture, String> {
    let mut words: Vec<&str> = command.split_whitespace().collect();
    let mut stdin_framed = false;
    let mut discard = false;
    while words.len() >= 2 {
        let (redirect, target) = (words[words.len() - 2], words[words.len() - 1]);
        match redirect {
            "<" if target == "probe-results.txt" => stdin_framed = true,
            "<" => {
                return Err(format!(
                    "only `< probe-results.txt` is supported, not `{target}`"
                ));
            }
            ">" if target == "/dev/null" => discard = true,
            ">" => return Err(format!("only `> /dev/null` is supported, not `{target}`")),
            _ => break,
        }
        words.truncate(words.len().saturating_sub(2));
    }
    // `--results=probe-results.txt` resolves to the framed stream exactly as `< probe-results.txt`
    // does -- the authored file is the gates. EXPECTATION, never the bytes dorc is fed.
    let framed_flag = format!("--results={}", framed.display());
    let words: Vec<&str> = words
        .into_iter()
        .map(|word| {
            if word == "--results=probe-results.txt" {
                framed_flag.as_str()
            } else {
                word
            }
        })
        .collect();
    match words.split_first() {
        Some((&"dorc", rest)) if !rest.is_empty() => {
            let mut child = harness.dorc(dir);
            child
                .current_dir(dir)
                .args(rest)
                .stderr(Stdio::piped())
                .stdout(if discard {
                    Stdio::null()
                } else {
                    Stdio::piped()
                });
            child.stdin(if stdin_framed {
                Stdio::from(std::fs::File::open(framed).map_err(|error| format!("{error}"))?)
            } else {
                Stdio::null()
            });
            let out = capture(&mut child);
            if out.code != 0 {
                return Err(format!("exited rc={}\n{}", out.code, out.stderr));
            }
            let got = strip_trailing_newlines(&strip_cr(&out.stdout));
            Ok(ReplayCapture {
                transcript: if got.is_empty() {
                    String::new()
                } else {
                    format!("{got}\n")
                },
                stderr: out.stderr,
            })
        }
        _ => Err(String::from("not a `dorc` invocation")),
    }
}

/// Fold the freshly-blessed `expected.out` / `expected.ran` — and, under the floor mint, the
/// re-measured `expected.emitted` — back into the committed `.loom`.
///
/// `extra` carries blocks 1..N's captured outputs; empty means either a single-block case or a
/// failed drive, and `set_replay_outputs` then leaves those blocks' committed bytes untouched
/// rather than overwriting them with broken output.
///
/// `mint_manifest` is [`Harness::bless_floor`]. Folding the manifest HERE, in the same write that
/// commits the transcript, is what makes the mint one coherent act: the `book=<sha256>` a reader
/// sees in the transcript and the manifest bytes beside it come from the same materialized book,
/// so neither can be hand-computed and neither can drift from the other.
fn bless_loom(
    spec: &LoomCaseSpec,
    dir: &Path,
    extra: &[String],
    mint_manifest: bool,
) -> Result<(), Failed> {
    let mut case = spec.case.clone();
    let mut outputs = vec![read_or_empty(&dir.join("expected.out"))];
    outputs.extend_from_slice(extra);
    case.set_replay_outputs(outputs);
    let ran = dir.join("expected.ran");
    if ran.is_file() && !case.set_section_content("expected.ran", &read_or_empty(&ran)) {
        return Err(format!(
            "FAIL  {}  [bless: the case runs under mocks but has no `expected.ran` section to bless into]",
            spec.name
        )
        .into());
    }
    let emitted = dir.join("expected.emitted");
    if mint_manifest
        && emitted.is_file()
        && !case.set_section_content("expected.emitted", &read_or_empty(&emitted))
    {
        return Err(format!(
            "FAIL  {}  [bless:floor: the floor lane measured a manifest but the case has no `expected.emitted` section to mint into]",
            spec.name
        )
        .into());
    }
    std::fs::write(&spec.path, case.to_text())
        .map_err(|error| Failed::from(format!("FAIL  {}  [bless: {error}]", spec.name)))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// the round-trip case driver

/// Everything a single round-trip case accumulates; `case_ok` is `failures.is_empty()`.
struct CaseRun {
    /// Failure lines, in gate order (the sh harness prints each and keeps going).
    failures: Vec<String>,
    /// A malformed guard artifact — RED even under XFAIL.
    guard_shape_bad: bool,
    /// An XFAIL case's run-set drifted from its pinned HEAD signature.
    head_ran_drifted: bool,
}

/// The shared argv every dorc invocation of a case reads: `-o <oracle>` (glob-sorted) plus the
/// optional `DORC_FLAGS` marker. Single-source threading makes a flag MISMATCH between gates
/// structurally impossible — load-bearing for gate-6's attribution, and for the extra replay
/// blocks, whose framed stdin must be the one the battery itself consumed.
fn shared_args(dir: &Path) -> Result<Vec<String>, String> {
    let mut args: Vec<String> = Vec::new();
    let mut oracles: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("read case dir")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".oracle.sh"))
        })
        .collect();
    oracles.sort();
    for oracle in oracles {
        args.push("--pre-source".to_owned());
        args.push(oracle.display().to_string());
    }
    if let Some(flags) = marker(dir, "DORC_FLAGS")? {
        args.push(flags);
    }
    Ok(args)
}

/// The shared argv PLUS the stdin claim, for a drive that really feeds framed records on stdin
/// (`owed-no-flag-defaults-to-stdin`: no flag acquires stdin implicitly any more, so a drive that
/// pipes records has to name the stream).
///
/// Deliberately NOT folded into [`shared_args`]: the same vector also reaches drives that must
/// NOT claim stdin — `dorc plan --host`, where `--results` and `--host` are mutually exclusive,
/// and the why-chain scan, which names its own `--results=<file>` and answers from the receipt.
fn args_reading_stdin_records(args: &[String]) -> Vec<String> {
    let mut out = args.to_vec();
    out.push("--results".to_owned());
    out.push("-".to_owned());
    out
}

/// Drive one round-trip case through every gate, then apply the XFAIL/BLESS lens.
fn run_round_trip(
    harness: &Harness,
    case: &E2eCase,
    drive_stderr: &mut String,
) -> Result<(), Failed> {
    let dir = &case.dir;
    let name = &case.name;
    // The dir-form seat of the same refusal `run_loom` makes on sections. Loom cases never reach
    // it (they are refused before materializing); a dir-form case carrying the manifest as a FILE
    // would otherwise walk straight into the discard this whole lane exists to close.
    if let Some(line) = floor_bless_refusal(
        harness.bless,
        harness.bless_floor,
        dir.join("expected.emitted").is_file(),
        name,
    ) {
        return Err(line.into());
    }
    let mut run = CaseRun {
        failures: Vec::new(),
        guard_shape_bad: false,
        head_ran_drifted: false,
    };

    let args = shared_args(dir)
        .map_err(|message| Failed::from(format!("FAIL  {name}  [DORC_FLAGS: {message}]")))?;
    let expected_dorc_exit: i32 = match marker(dir, "DORC_EXIT") {
        Ok(Some(value)) => value.parse().map_err(|_| {
            Failed::from(format!(
                "FAIL  {name}  [DORC_EXIT: `{value}` is not an integer]"
            ))
        })?,
        Ok(None) => 0,
        Err(message) => return Err(format!("FAIL  {name}  [DORC_EXIT: {message}]").into()),
    };

    let scratch = Scratch::new("case");
    let framed_path = scratch.path.join("framed.txt");
    std::fs::write(&framed_path, framed_results(harness, dir, &args)).expect("write framed");
    let shim_dir = scratch.path.join("shims");
    std::fs::create_dir_all(&shim_dir).expect("create shim dir");

    // The artifact STREAM this case's product goes to. Absent, the artifact is stdout and the run
    // sits in the single-stream cell every case has always sat in; present, the run may materialize
    // a directory, and the artifact SET rather than the plan alone is what the exec gates measure.
    let artifact_root = has_marker(dir, "ARTIFACT_SET").then(|| {
        let root = scratch.path.join("artifacts");
        std::fs::create_dir_all(&root).expect("create artifact root");
        root
    });

    let book = dir.join("book.sh");
    let mut command = harness.dorc(dir);
    command
        .arg(format!("--shim-dir={}", shim_dir.display()))
        .arg(format!("--book={}", book.display()));
    if let Some(root) = &artifact_root {
        command.arg(format!("--artifact-dir={}", root.display()));
    }
    let out = capture(
        command
            .args(args_reading_stdin_records(&args))
            .stdin(Stdio::from(std::fs::File::open(&framed_path).unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
    // Handed back through the parameter rather than the return, because this function exits at a
    // dozen gate verdicts and the caller's code-fired question is about the DRIVE, not the verdict:
    // a case whose declared code fired and whose golden then diverged must still say so.
    drive_stderr.push_str(&out.stderr);
    let got = strip_trailing_newlines(&strip_cr(&out.stdout));
    if out.code != expected_dorc_exit || got.is_empty() {
        return Err(format!(
            "FAIL  {name}  [dorc exited rc={} (expected {expected_dorc_exit}) / produced no output — a dead engine, or a wrong exit-code contract, is never green]\n{}",
            out.code, out.stderr
        )
        .into());
    }

    let (probe_art, apply_art) = split_artifacts(&got);
    let xfail_reason = std::fs::read_to_string(dir.join("XFAIL"))
        .ok()
        .map(|text| text.lines().next().unwrap_or_default().to_owned());
    let xfail_active = xfail_reason.is_some();

    // Opt-in implies require: a case that declares an artifact SET and did not get one has measured
    // the flattened world under a multipart name, which is exactly the false green this marker
    // exists to close.
    let published = match &artifact_root {
        None => None,
        Some(root) => match published_generation(root) {
            Ok(generation) => Some(generation),
            Err(why) => {
                return Err(format!("FAIL  {name}  [ARTIFACT_SET: {why}]").into());
            }
        },
    };
    if let Some(generation) = &published {
        let planned =
            strip_trailing_newlines(&strip_cr(&read_or_empty(&generation.join("plan.sh"))));
        if planned != apply_art {
            run.failures.push(format!(
                "FAIL  {name}  [ARTIFACT_SET: the published plan.sh and the apply block on stdout are not the same bytes — the two surfaces must READ one artifact set, never assemble it twice]\n{}",
                divergence(&apply_art, &planned)
            ));
        }
        for missing in unresolved_generated_imports(generation, &planned) {
            run.failures.push(format!(
                "FAIL  {name}  [ARTIFACT_SET: the published plan sources `{missing}`, which the generation does not contain — a generated plan's own imports name files the artifact carries, or the artifact is not one]"
            ));
        }
    }

    if let Some(error) = harness.syntax_error(&probe_art) {
        run.failures.push(format!(
            "FAIL  {name}  [ap-2: rendered probe is not {} -n clean]\n      {error}",
            harness.checker_name
        ));
    }
    if let Some(error) = harness.syntax_error(&apply_art) {
        run.failures.push(format!(
            "FAIL  {name}  [ap-2: rendered apply is not {} -n clean]\n      {error}",
            harness.checker_name
        ));
    }

    let mocks = dir.join("mocks");
    let run_root = published.as_deref();
    // The counterfactual rails' own world (`counterfactual_root`), built only where a generation
    // exists so every other case keeps the empty throwaway sandbox it has always had.
    let counterfactual = published.as_deref().and_then(|generation| {
        let into = scratch.path.join("counterfactual");
        counterfactual_root(dir, generation, &into)
            .ok()
            .map(|()| into)
    });
    let counterfactual_run_root = counterfactual.as_deref().or(run_root);
    if run.failures.is_empty() && mocks.is_dir() {
        exec_check(
            harness,
            name,
            dir,
            &mocks,
            &apply_art,
            run_root,
            &mut run.failures,
        );
        probe_exec_check(
            harness,
            name,
            dir,
            &mocks,
            &shim_dir,
            &probe_art,
            &mut run.failures,
        );
        if !harness.bless {
            let shimset = shimset(&mocks);
            argv_echo_check(
                harness,
                name,
                dir,
                &mocks,
                &shimset,
                &args,
                &framed_path,
                counterfactual_run_root,
                &mut run.failures,
            );
            if !has_marker(dir, "PROBE_RESULTS=authored")
                && !has_marker(dir, "DUAL_RAIL=inlined")
                && !has_marker(dir, "DUAL_RAIL=multiline-argv")
            {
                dual_rail_check(
                    harness,
                    name,
                    dir,
                    &mocks,
                    &shimset,
                    &args,
                    &framed_path,
                    counterfactual_run_root,
                    &mut run.failures,
                );
            }
        }
    }

    floor_differential(harness, name, dir, &mocks, &mut run.failures);

    scan_diagnostics(name, &out.stderr, dir, &mut run.failures);
    scan_why(name, &out.stderr, dir, &mut run.failures);
    scan_hint(name, &out.stderr, dir, &mut run.failures);
    scan_why_chain(harness, name, dir, &args, &framed_path, &mut run.failures);

    let guard_violations = guard_shape_violations(&apply_art, &read_or_empty(&book));
    if !guard_violations.is_empty() {
        run.guard_shape_bad = true;
        run.failures.push(format!(
            "FAIL  {name}  [guard-shape: a guarded line violates rul-ternary-verdict's artifact-shape law (never-1 / bytes-verbatim); the shape floor screams even under XFAIL — 23C-fd4]\n{}",
            indent(&guard_violations)
        ));
    }

    if xfail_active && head_ran_drifted(harness, dir, &mocks, &apply_art, run_root) {
        run.head_ran_drifted = true;
    }

    if run.failures.is_empty() && !harness.bless && !xfail_active {
        let want = strip_trailing_newlines(&strip_cr(&read_or_empty(&dir.join("expected.out"))));
        if got != want {
            run.failures.push(format!(
                "FAIL  {name}  [content diff]\n{}",
                divergence(&want, &got)
            ));
        }
    }

    // The XFAIL / BLESS / ok lens.
    if let Some(reason) = xfail_reason {
        if run.failures.is_empty() {
            return Err(format!(
                "XPASS {name}  [known defect appears FIXED — promote this case: {reason}]"
            )
            .into());
        }
        if run.guard_shape_bad {
            return Err(run.failures.join("\n").into());
        }
        if run.head_ran_drifted {
            return Err(format!(
                "FAIL  {name}  [head-expected.ran: current run-set drifted from the pinned HEAD signature while still XFAIL — a disaster-shaped behaviour change is hiding as an ordinary xfail (two-sided pin, 23B-fd1/23C-fd4)]"
            )
            .into());
        }
        return Ok(()); // `xfail <name> [<reason>]`
    }
    if harness.bless {
        if run.failures.is_empty() {
            std::fs::write(dir.join("expected.out"), format!("{got}\n"))
                .expect("bless expected.out");
            return Ok(());
        }
        run.failures
            .push(format!("FAIL  {name}  [gate failed; not blessed]"));
    }
    if run.failures.is_empty() {
        Ok(())
    } else {
        Err(run.failures.join("\n").into())
    }
}

/// Split dorc's stdout into the probe (first `#!/bin/sh` block) and the eliding apply
/// (from the second shebang on).
fn split_artifacts(stdout: &str) -> (String, String) {
    let (mut probe, mut apply) = (Vec::new(), Vec::new());
    let mut shebangs = 0usize;
    for line in stdout.lines() {
        if line.starts_with("#!/bin/sh") {
            shebangs += 1;
        }
        if shebangs == 1 {
            probe.push(line);
        }
        if shebangs >= 2 {
            apply.push(line);
        }
    }
    (probe.join("\n"), apply.join("\n"))
}

/// `" name1 name2 "` — the case's shim set, the way gate-5/gate-6 test membership.
fn shimset(mocks: &Path) -> String {
    let mut names: Vec<String> = std::fs::read_dir(mocks)
        .expect("read mocks")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| !name.starts_with('.'))
        .collect();
    names.sort();
    format!(" {} ", names.join(" "))
}

/// Indent detail lines the way the sh harness does.
fn indent(lines: &[String]) -> String {
    lines
        .iter()
        .flat_map(|line| line.lines())
        .map(|line| format!("      {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// An aligned first-divergence window (the sh harness printed `diff -u`).
fn divergence(want: &str, got: &str) -> String {
    let mut out = String::new();
    for line in errorloom::describe_divergence(want, got)
        .unwrap_or_else(|| String::from("byte-identical (the check and the report disagree)"))
        .lines()
    {
        let _ = writeln!(out, "      {line}");
    }
    out.trim_end().to_owned()
}

// ---------------------------------------------------------------------------
// the per-case gates

/// The ap-2 EXECUTABLE acceptance: run the rendered apply under the inert shims and
/// assert the exact set of commands that ran, plus the declared exit rc.
///
/// `run_root` names a PUBLISHED artifact generation for a case that declared one, and the gate then
/// runs that generation's own `plan.sh` from inside it — the product an operator receives, at the
/// cwd `30I` §7.6 gives it. Without one the rendered text runs alone in an empty sandbox, which is
/// the flattened form's honest world and every existing case's.
fn exec_check(
    harness: &Harness,
    name: &str,
    dir: &Path,
    mocks: &Path,
    artifact: &str,
    run_root: Option<&Path>,
    failures: &mut Vec<String>,
) {
    let unsafe_lines = scan_redirects(artifact);
    if !unsafe_lines.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-2: rendered apply has an unsafe redirect target (absolute/dynamic/escaping) — refused before exec]\n{}",
            indent(&unsafe_lines)
        ));
        return;
    }
    let expected_rc: i32 = match marker(dir, "EXIT_RC") {
        Ok(Some(value)) => {
            let Ok(rc) = value.parse() else {
                failures.push(format!(
                    "FAIL  {name}  [ap-2-exec: EXIT_RC marker value '{value}' is not a non-negative integer]"
                ));
                return;
            };
            rc
        }
        Ok(None) => 0,
        Err(_) => {
            failures.push(format!(
                "FAIL  {name}  [ap-2-exec: multiple EXIT_RC=<n> markers — exactly one expected-exit is permitted]"
            ));
            return;
        }
    };

    let scratch = Scratch::new("exec");
    let log = scratch.path.join("dorc.log");
    std::fs::write(&log, "").expect("seed log");
    let own = scratch.path.join("sand");
    std::fs::create_dir_all(&own).expect("create sandbox");
    let sandbox = run_root.unwrap_or(&own).to_path_buf();
    let payload = scratch.path.join("apply.sh");
    std::fs::write(&payload, format!("{artifact}\n")).expect("write apply");
    let mut command = if run_root.is_some() {
        harness.rail(
            &sandbox,
            &log,
            mocks.as_os_str(),
            Some(Path::new("plan.sh")),
        )
    } else {
        let mut piped = harness.rail(&sandbox, &log, mocks.as_os_str(), None);
        piped.stdin(Stdio::from(std::fs::File::open(&payload).unwrap()));
        piped
    };
    let out = capture(command.stdout(Stdio::piped()).stderr(Stdio::piped()));
    if out.code != expected_rc {
        failures.push(format!(
            "FAIL  {name}  [ap-2-exec: rendered apply exited rc={}, expected {expected_rc}]\n      {}",
            out.code,
            out.stderr.trim_end()
        ));
        return;
    }

    let tolerated = match tolerances(dir) {
        Ok(tolerated) => tolerated,
        Err(message) => {
            failures.push(format!("FAIL  {name}  [tolerate: {message}]"));
            return;
        }
    };
    // The declared normalizers run on the CAPTURE, on both paths — so what bless commits is
    // already the canonical form, and the check compares canonical to canonical.
    let got_ran = canonicalize(&strip_trailing_newlines(&read_or_empty(&log)), &tolerated);
    if harness.bless {
        std::fs::write(dir.join("expected.ran"), format!("{got_ran}\n"))
            .expect("bless expected.ran");
        return;
    }
    let expected = dir.join("expected.ran");
    if !expected.is_file() {
        failures.push(format!(
            "FAIL  {name}  [ap-2-exec: mocks/ present but expected.ran missing — author or bless it]"
        ));
        return;
    }
    let want_ran = strip_trailing_newlines(&read_or_empty(&expected));
    if got_ran != want_ran {
        failures.push(format!(
            "FAIL  {name}  [ap-2-exec: apply ran the wrong commands or wrong order]\n{}",
            indent(&[format!("want:\n{want_ran}\ngot:\n{got_ran}")])
        ));
    }
}

/// gate-1: execute the rendered probe under the inert shims and assert (a) site-
/// completeness + grammar, (c) vouch-closure (no rc=127), (b) record parity against the
/// authored fixture, and (d) deriv-coord parity. (b)/(c)/(d) are disabled by a
/// `PROBE_RESULTS=authored` marker; (a) always holds.
fn probe_exec_check(
    harness: &Harness,
    name: &str,
    dir: &Path,
    mocks: &Path,
    shim_dir: &Path,
    artifact: &str,
    failures: &mut Vec<String>,
) {
    let unsafe_lines = scan_redirects(artifact);
    if !unsafe_lines.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-2: rendered probe has an unsafe redirect target (absolute/dynamic/escaping) — refused before exec]\n{}",
            indent(&unsafe_lines)
        ));
        return;
    }
    // Shim-materialization last mile (`274` §5): mocks FIRST, so mocked tools keep
    // winning and the shim adds only the disjoint oracle-check names.
    let shims_present =
        std::fs::read_dir(shim_dir).is_ok_and(|entries| entries.flatten().next().is_some());
    let probe_path = if shims_present {
        std::env::join_paths([mocks, shim_dir]).expect("join probe PATH")
    } else {
        mocks.as_os_str().to_owned()
    };

    let emit_ids = sort_lines(&emitted_site_ids(artifact).join("\n"));

    let scratch = Scratch::new("probe");
    let log = scratch.path.join("dorc.log");
    std::fs::write(&log, "").expect("seed log");
    let sandbox = scratch.path.join("sand");
    std::fs::create_dir_all(&sandbox).expect("create sandbox");
    let payload = scratch.path.join("probe.sh");
    std::fs::write(&payload, format!("{artifact}\n")).expect("write probe");
    let out = capture(
        harness
            .rail(&sandbox, &log, &probe_path, None)
            .stdin(Stdio::from(std::fs::File::open(&payload).unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    );

    // Deframe the `dorc-records/1` stream to the inner records the gate understands.
    let recs = strip_trailing_newlines(
        &strip_cr(&out.stdout)
            .lines()
            .filter_map(|line| {
                line.strip_prefix(&format!("{RECORDS_NONCE} "))?
                    .strip_suffix(&format!(" {RECORDS_TOKEN}"))
                    .map(str::to_owned)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    let rec_lines: Vec<&str> = lines_of(&recs)
        .into_iter()
        .filter(|line| line.starts_with("site "))
        .collect();
    let good_ids = sort_lines(
        &rec_lines
            .iter()
            .filter_map(|line| valid_site_record(line))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    if good_ids != emit_ids {
        failures.push(format!(
            "FAIL  {name}  [gate-1: probe records not site-complete/grammar-valid (every resolvable site must emit exactly one valid record)]\n      emitters: {}\n      valid records: {}\n      raw records:\n{}",
            emit_ids.replace('\n', " "),
            good_ids.replace('\n', " "),
            indent(std::slice::from_ref(&recs))
        ));
        return;
    }
    if has_marker(dir, "PROBE_RESULTS=authored") {
        return;
    }

    let not_found: Vec<String> = rec_lines
        .iter()
        .filter(|line| line.ends_with("rc=127"))
        .map(|line| (*line).to_owned())
        .collect();
    if !not_found.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-1: probe invoked an un-shimmed command (rc=127) — vouch-closure: a probe command has no mock (add a probe shim, or mark PROBE_RESULTS=authored)]\n{}",
            indent(&not_found)
        ));
        return;
    }

    let authored_text = read_or_empty(&dir.join("probe-results.txt"));
    let rc_sites = rc_bearing_sites(&authored_text);
    let produced = sort_lines(&norm_parity(
        &lines_of(&recs)
            .into_iter()
            .filter(|line| line.starts_with("site "))
            .collect::<Vec<_>>()
            .join("\n"),
        &rc_sites,
    ));
    let authored = sort_lines(&norm_parity(
        &authored_text
            .lines()
            .filter(|line| line.starts_with("site "))
            .collect::<Vec<_>>()
            .join("\n"),
        &rc_sites,
    ));
    if produced != authored {
        failures.push(format!(
            "FAIL  {name}  [gate-1: mocked probe records diverge from authored probe-results.txt — re-author the fixture, add probe shims, or mark PROBE_RESULTS=authored (do NOT silently re-bless)]\n{}",
            indent(&[format!("authored:\n{authored}\nproduced:\n{produced}")])
        ));
        return;
    }

    let authored_deriv = sort_lines(
        &authored_text
            .lines()
            .filter(|line| line.starts_with("deriv "))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    if authored_deriv.is_empty() {
        return;
    }
    let produced_deriv = sort_lines(
        &lines_of(&recs)
            .into_iter()
            .filter(|line| {
                line.strip_prefix("deriv ")
                    .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
    if produced_deriv != authored_deriv {
        failures.push(format!(
            "FAIL  {name}  [gate-1: mocked probe's DERIV coords diverge from authored probe-results.txt — the derivation lane is not reproducing its footprint (an empty produced set means the shipped def and its invocation disagree, or a derivation command has no shim)]\n{}",
            indent(&[format!("authored:\n{authored_deriv}\nproduced:\n{produced_deriv}")])
        ));
    }
}

/// The site keys the probe self-reports (one `printf 'dorc site <key> effect=…` emitter).
fn emitted_site_ids(artifact: &str) -> Vec<String> {
    let needle = format!("printf '{RECORDS_NONCE} site ");
    let mut ids = Vec::new();
    for line in artifact.lines() {
        let Some(at) = line.rfind(&needle) else {
            continue;
        };
        let rest = &line[at + needle.len()..];
        let id: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        if !id.is_empty() && rest[id.len()..].starts_with(" effect=") {
            ids.push(id);
        }
    }
    ids
}

/// The site id of a grammar-valid record (`site <id> effect=<word> rc=<int>`).
fn valid_site_record(line: &str) -> Option<String> {
    let rest = line.strip_prefix("site ")?;
    let (id, rest) = rest.split_once(' ')?;
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return None;
    }
    let rest = rest.strip_prefix("effect=")?;
    let (effect, rc) = rest.split_once(' ')?;
    if !matches!(effect, "holds" | "absent" | "cant-tell") {
        return None;
    }
    let digits = rc
        .strip_prefix("rc=")?
        .strip_prefix('-')
        .unwrap_or_else(|| rc.strip_prefix("rc=").unwrap_or_default());
    (!digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())).then(|| id.to_owned())
}

/// gate-5: the engine's per-site resolved argv, cross-checked against ground truth from
/// the bare book under dash. One-directional and conservative:
/// engine-resolved-and-shimmed-and-`run` ⊆ logged.
#[expect(
    clippy::too_many_arguments,
    reason = "the sh gate's parameter set, moved intact"
)]
fn argv_echo_check(
    harness: &Harness,
    name: &str,
    dir: &Path,
    mocks: &Path,
    shims: &str,
    args: &[String],
    framed: &Path,
    run_root: Option<&Path>,
    failures: &mut Vec<String>,
) {
    let debug = debug_argv(harness, dir, args, framed);
    let engine: Vec<&str> = debug
        .lines()
        .filter(|line| line.starts_with("argv "))
        .collect();
    let logged = harness.capture_run(Payload::File(&dir.join("book.sh")), mocks, run_root);
    let logged_lines = lines_of(&logged);
    let mut bad = Vec::new();
    for line in engine {
        let Some((disposition, words)) = split_debug_argv(line) else {
            continue;
        };
        if disposition != "run" || words.is_empty() {
            continue;
        }
        if format!(" {words} ").contains(" TOP ") {
            continue;
        }
        let cmd0 = words.split(' ').next().unwrap_or_default();
        if !shims.contains(&format!(" {cmd0} ")) {
            continue;
        }
        if !logged_lines.contains(&words) {
            bad.push(line.to_owned());
        }
    }
    if !bad.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-5: engine-resolved argv not in the bare book's executed argvs (dash disagrees with value-flow)]\n{}",
            indent(&bad)
        ));
    }
}

/// `argv <leafid> <disposition> <words…>` → `(disposition, words)`.
fn split_debug_argv(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix("argv ")?;
    let (id, rest) = rest.split_once(' ')?;
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    match rest.split_once(' ') {
        Some((verb, words)) if verb.chars().all(|c| c.is_ascii_lowercase()) => Some((verb, words)),
        None if rest.chars().all(|c| c.is_ascii_lowercase()) => Some((rest, "")),
        _ => None,
    }
}

/// The raw `--debug-argv` readout (stderr; stdout discarded).
fn debug_argv(harness: &Harness, dir: &Path, args: &[String], framed: &Path) -> String {
    capture(
        harness
            .dorc(dir)
            .arg("--debug-argv")
            .arg(format!("--book={}", dir.join("book.sh").display()))
            .args(args_reading_stdin_records(args))
            .stdin(Stdio::from(std::fs::File::open(framed).unwrap()))
            .stdout(Stdio::null())
            .stderr(Stdio::piped()),
    )
    .stderr
}

/// The lane that opts a run into executing the base-dialect floor binaries: a comma-list of names
/// (`DORC_E2E_FLOOR_SHELLS=dash,posh`). UNSET ⇒ zero external shell invocations beyond the ones the
/// harness already makes, exactly the `real-tools-lane-opt-in` default; listed-but-absent ⇒ a loud
/// failure, because opt-in implies require-tools and a differential answered by fewer shells than
/// the operator asked for is a differential that measured something else.
const FLOOR_SHELLS_ENV: &str = "DORC_E2E_FLOOR_SHELLS";

/// The floor lane's own WRITE authority — the second half of the mint's double opt-in, alongside
/// [`FLOOR_SHELLS_ENV`] (`mise run bless:floor`; `spike/CLAUDE.md`
/// emitted-is-measure-once-ground-truth).
///
/// `expected.emitted` is what the floor BINARIES said, so an ordinary `BLESS=1` has no authority
/// over it and [`floor_bless_refusal`] says so. This flag is the one path that may write it, and
/// therefore the one way to mint or amend a floor case at all.
const FLOOR_BLESS_ENV: &str = "BLESS_FLOOR";

/// Why a bless run refuses this case, or `None` when it may proceed. Pure: the policy is three
/// booleans, and this seat has now bitten three lanes, so it is worth stating exhaustively
/// (`floor_bless_selftest`).
fn floor_bless_refusal(
    bless: bool,
    bless_floor: bool,
    emitted: bool,
    name: &str,
) -> Option<String> {
    (bless && !bless_floor && emitted).then(|| {
        format!(
            "FAIL  {name}  [bless: this case carries `expected.emitted`, which is MEASURED ground truth (the floor shells' own answer) and never an engine render — a default bless would rewrite its transcript around a manifest it did not re-measure. Mint or amend it with `mise run bless:floor -- {name}` ({FLOOR_BLESS_ENV}=1 + {FLOOR_SHELLS_ENV}), which re-measures and writes both in one act]"
        )
    })
}

/// gate-9: the two-binary-floor DIFFERENTIAL (`28K` §5 model-calibration; `276:rul-spec-two-binary-
/// floor`'s own prescription — strip-then-run-under-both IS the executable off-ramp test).
///
/// A case opts in by carrying an `expected.emitted` section: its book is then a SENTINEL MANIFEST,
/// a which-am-I emitter whose stdout says which definition a real shell actually had live at each
/// point. The gate strips the book to stock POSIX sh (the off-ramp cleaner, so no dialect construct
/// reaches the floor binaries), runs the result under every named floor shell on the determinism
/// rail, and requires them to agree with each other AND with the committed bytes.
///
/// What it measures is the half the corpus cannot otherwise reach. Dorc's own answer for the same
/// shape is the committed transcript beside it — the analyzer half, proven on every platform by the
/// ordinary run. This gate proves the SHELL half, so a divergence between the two is a measured
/// fact rather than an argument, and the `command -v` case can pin its documented divergence
/// instead of asserting its absence.
fn floor_differential(
    harness: &Harness,
    name: &str,
    dir: &Path,
    mocks: &Path,
    failures: &mut Vec<String>,
) {
    let want_path = dir.join("expected.emitted");
    if !want_path.is_file() || harness.floor_shells.is_empty() {
        return;
    }
    let stripped = capture(
        harness
            .dorc(dir)
            .arg("strip")
            .arg(dir.join("book.sh"))
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;
    let mut emitted: Vec<(String, String)> = Vec::new();
    for shell_name in &harness.floor_shells {
        match Posix::floor(shell_name) {
            Ok(shell) => emitted.push((
                shell_name.clone(),
                capture_floor_stdout(&shell, &stripped, dir, mocks),
            )),
            Err(why) => failures.push(format!(
                "FAIL  {name}  [gate-9: floor shell `{shell_name}` named in {FLOOR_SHELLS_ENV} is absent — {why}]"
            )),
        }
    }
    if emitted.is_empty() {
        return;
    }
    // Disagreement BETWEEN the floor binaries is the dialect's own answer: the construct is outside
    // it (`276:rul-spec-two-binary-floor`), and no committed byte can be right for both.
    if let Some((first_name, first)) = emitted.first()
        && let Some((other_name, other)) = emitted.iter().find(|(_, text)| text != first)
    {
        failures.push(format!(
            "FAIL  {name}  [gate-9: the floor binaries disagree, so this construct is OUTSIDE the base dialect]\n{}",
            indent(&[format!("{first_name}: {first}"), format!("{other_name}: {other}")])
        ));
        return;
    }
    let got = &emitted[0].1;
    if harness.bless_floor {
        // A mint answered by ONE binary never asks the differential's question, and the answer to
        // that question is the whole point of the section being committed. Windows has no `posh`
        // in git's userland, so this is where a Windows mint stops and the WSL leg takes over.
        if emitted.len() < 2 {
            failures.push(format!(
                "FAIL  {name}  [bless:floor: minting `expected.emitted` from ONE binary ({}) would commit ground truth no differential ever agreed to — name two floor shells; on Windows that means running the mint from WSL]",
                emitted[0].0
            ));
            return;
        }
        std::fs::write(&want_path, format!("{got}\n")).expect("bless expected.emitted");
        return;
    }
    let want = strip_trailing_newlines(&strip_cr(&read_or_empty(&want_path)));
    if got != &want {
        failures.push(format!(
            "FAIL  {name}  [gate-9: the floor shells emit something other than the committed manifest]\n{}",
            divergence(&want, got)
        ));
    }
}

/// gate-6: the apply/bare run-set delta must be covered by the engine's own
/// replace/omit/guard license ledger.
#[expect(
    clippy::too_many_arguments,
    reason = "the sh gate's parameter set, moved intact"
)]
fn dual_rail_check(
    harness: &Harness,
    name: &str,
    dir: &Path,
    mocks: &Path,
    shims: &str,
    args: &[String],
    framed: &Path,
    run_root: Option<&Path>,
    failures: &mut Vec<String>,
) {
    let debug = debug_argv(harness, dir, args, framed);
    // BOTH rails: dropping the region one makes every region elision read as unattributed.
    let disp = debug
        .lines()
        .filter(|line| line.starts_with("argv ") || line.starts_with("region "))
        .collect::<Vec<_>>()
        .join("\n");
    let guard_cmds = debug
        .lines()
        .filter_map(|line| line.strip_prefix("guardcmd "))
        .collect::<Vec<_>>()
        .join("\n");
    let bare = harness.capture_run(Payload::File(&dir.join("book.sh")), mocks, run_root);
    let apply_out = capture(
        harness
            .dorc(dir)
            .arg(format!("--book={}", dir.join("book.sh").display()))
            .args(args_reading_stdin_records(args))
            .stdin(Stdio::from(std::fs::File::open(framed).unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;
    let (_, apply_art) = split_artifacts(&apply_out);
    let apply = harness.capture_run(Payload::Text(&apply_art), mocks, run_root);
    let violations = dual_rail_judge(&bare, &apply, &disp, shims, &guard_cmds);
    if !violations.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-6: apply/bare run-set delta not covered by the license ledger (cm-1 dual-rail)]\n{}",
            indent(&violations)
        ));
    }
}

/// gate-3: an undeclared error-severity diagnostic on dorc's stderr fails the case.
///
/// `expected-diagnostics` is a list of code SLUGS, one per line
/// (`288:prop-structural-needles-only`): the needle `<severity>[<slug>]` is DERIVED, and every slug
/// is validated against the generated catalog. That kills the two ways the old free-text form
/// rotted — a needle carrying migrated `sm ` prose stops matching the moment phase 8 rewrites
/// that prose, and a needle naming a deleted code silently declares nothing forever
/// (`288:nit-needles-rot`). A dead slug is now REFUSED, so the file cleans itself.
fn scan_diagnostics(name: &str, stderr: &str, dir: &Path, failures: &mut Vec<String>) {
    let decl = dir.join("expected-diagnostics");
    let slugs: Vec<String> = if nonempty_file(&decl) {
        read_or_empty(&decl)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_owned)
            .collect()
    } else {
        Vec::new()
    };
    let dead: Vec<String> = slugs
        .iter()
        .filter(|slug| !catalog_has_slug(slug))
        .map(|slug| format!("`{slug}` is not a code in the generated catalog"))
        .collect();
    if !dead.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-3: expected-diagnostics names a code that does not exist — declare the live slug, or drop the line]\n{}",
            indent(&dead)
        ));
        return;
    }

    let errors: Vec<&str> = stderr
        .lines()
        .filter(|line| {
            line.split_once(": error[").is_some_and(|(stage, _)| {
                !stage.is_empty() && stage.chars().all(|c| c.is_ascii_lowercase())
            })
        })
        .collect();
    // Declared-must-fire reads the SLUG at any severity (only the undeclared-noise half below
    // stays error-keyed): severity is registry data a case does not restate.
    let fired = |slug: &str| {
        ["error", "warning", "note"]
            .iter()
            .any(|severity| stderr.contains(&format!("{severity}[{slug}]")))
    };
    let unfired: Vec<String> = slugs
        .iter()
        .filter(|slug| !fired(slug))
        .map(|slug| format!("declared but never emitted: [{slug}]"))
        .collect();
    if !unfired.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-3: a declared diagnostic did not fire — the declaration is an assertion, not a mute]\n{}",
            indent(&unfired)
        ));
    }
    if errors.is_empty() {
        return;
    }
    let undeclared: Vec<String> = errors
        .iter()
        .filter(|line| {
            !slugs
                .iter()
                .any(|slug| line.contains(&format!("error[{slug}]")))
        })
        .map(|line| (*line).to_owned())
        .collect();
    if !undeclared.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-3: undeclared error-severity diagnostic on stderr — fix the cause, or declare it in an expected-diagnostics file]\n{}",
            indent(&undeclared)
        ));
    }
}

/// gate-7: opt-in `expected-why` substring (and ` && `-conjoined) assertions over the
/// `why:` stderr lines. Unlike the other needle gates, `#` lines are NOT comments here —
/// the sh original reads every non-empty line as a pattern.
fn scan_why(name: &str, stderr: &str, dir: &Path, failures: &mut Vec<String>) {
    let decl = dir.join("expected-why");
    if !nonempty_file(&decl) {
        return;
    }
    let whys: Vec<&str> = stderr
        .lines()
        .filter(|line| line.starts_with("why: "))
        .collect();
    let missing: Vec<String> = read_or_empty(&decl)
        .lines()
        .filter(|pattern| !pattern.is_empty())
        .filter(|pattern| !needle_lands(&whys, pattern))
        .map(str::to_owned)
        .collect();
    if !missing.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-7: expected why-lens line(s) not emitted on stderr — fix the cause, or update expected-why]\n{}",
            missing
                .iter()
                .map(|pattern| format!("      missing: {pattern}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
}

/// gate-hint: opt-in `expected-hint` assertions over the `hint:` stderr lines.
fn scan_hint(name: &str, stderr: &str, dir: &Path, failures: &mut Vec<String>) {
    let decl = dir.join("expected-hint");
    if !nonempty_file(&decl) {
        return;
    }
    let hints = stderr
        .lines()
        .filter(|line| line.starts_with("hint: "))
        .collect::<Vec<_>>()
        .join("\n");
    let missing = needles_missing(&hints, &decl);
    if !missing.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-hint: expected first-wall hint line(s) not emitted on stderr — fix the cause, or update expected-hint]\n{}",
            missing
                .iter()
                .map(|pattern| format!("      missing: {pattern}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
}

/// gate-8: the why-chain PAIR — `dorc why <n>` live and `dorc why <n> --last` replayed
/// through the whylog must both land the same needles.
fn scan_why_chain(
    harness: &Harness,
    name: &str,
    dir: &Path,
    args: &[String],
    framed: &Path,
    failures: &mut Vec<String>,
) {
    let decl = dir.join("expected-why-chain");
    if !nonempty_file(&decl) {
        return;
    }
    let Ok(Some(addr)) = marker(dir, "WHY_ADDR") else {
        failures.push(format!(
            "FAIL  {name}  [gate-8: expected-why-chain present but no WHY_ADDR=<n> marker]"
        ));
        return;
    };
    let book = format!("--book={}", dir.join("book.sh").display());
    let live = capture(
        harness
            .dorc(dir)
            .arg("why")
            .arg(&addr)
            .arg(&book)
            .arg(format!("--results={}", framed.display()))
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;

    let scratch = Scratch::new("whylog");
    let whylog = scratch.path.join("log");
    std::fs::create_dir_all(&whylog).expect("create whylog dir");
    let _ = capture(
        harness
            .dorc(dir)
            .arg(&book)
            .args(args_reading_stdin_records(args))
            .arg(format!("--whylog-dir={}", whylog.display()))
            .stdin(Stdio::from(std::fs::File::open(framed).unwrap()))
            .stdout(Stdio::null())
            .stderr(Stdio::null()),
    );
    let replay = capture(
        harness
            .dorc(dir)
            .arg("why")
            .arg(&addr)
            .arg("--last")
            .arg(&book)
            .args(args)
            .arg(format!("--whylog-dir={}", whylog.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;

    let missing_live = needles_missing(&strip_trailing_newlines(&live), &decl);
    let missing_replay = needles_missing(&strip_trailing_newlines(&replay), &decl);
    if missing_live.is_empty() && missing_replay.is_empty() {
        return;
    }
    let mut detail = String::new();
    for pattern in &missing_live {
        let _ = writeln!(detail, "      missing (live): {pattern}");
    }
    for pattern in &missing_replay {
        let _ = writeln!(detail, "      missing (replay): {pattern}");
    }
    failures.push(format!(
        "FAIL  {name}  [gate-8: why-chain needle(s) missing — fix the walker, or update expected-why-chain]\n{}",
        detail.trim_end()
    ));
}

/// The two-sided XFAIL pin: has an XFAIL case's current apply run-set drifted from the
/// `head-expected.ran` signature captured when the pin was authored?
fn head_ran_drifted(
    harness: &Harness,
    dir: &Path,
    mocks: &Path,
    apply: &str,
    run_root: Option<&Path>,
) -> bool {
    let pin = dir.join("head-expected.ran");
    if !pin.is_file() || !mocks.is_dir() {
        return false;
    }
    let tolerated = tolerances(dir).unwrap_or_default();
    let got = canonicalize(
        &harness.capture_run(Payload::Text(apply), mocks, run_root),
        &tolerated,
    );
    let want = strip_trailing_newlines(
        &read_or_empty(&pin)
            .lines()
            .map(|line| line.strip_prefix("ran: ").unwrap_or(line))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    got != want
}

// ---------------------------------------------------------------------------
// the lint case drivers

/// A `dorc lint` case: the flags in `cmd`, run from INSIDE the case dir under a SCRUBBED
/// `PATH` so the external linters are deterministically absent.
fn run_lint(harness: &Harness, case: &E2eCase) -> Result<(), Failed> {
    let dir = &case.dir;
    let name = &case.name;
    let flags: Vec<String> = read_or_empty(&dir.join("cmd"))
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let want_rc: i32 = strip_trailing_newlines(&read_or_empty(&dir.join("expected-rc")))
        .parse()
        .unwrap_or(0);
    let empty = Scratch::new("lintpath");
    let out = capture(
        Command::new(&harness.dorc)
            .current_dir(dir)
            .env("PATH", &empty.path)
            .arg("lint")
            .args(&flags)
            .arg("book.sh")
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    );
    if out.code != want_rc {
        return Err(format!(
            "FAIL  {name}  [lint: exit rc={}, expected {want_rc}]",
            out.code
        )
        .into());
    }
    let got = strip_trailing_newlines(&strip_cr(&out.stdout));
    // A lint case's golden is written HERE, like every other gate's: without this the only route
    // to re-blessing one was a hand edit, which is a golden nothing mechanically produced.
    if harness.bless {
        std::fs::write(dir.join("expected.out"), format!("{got}\n")).expect("bless expected.out");
        return Ok(());
    }
    let want = strip_trailing_newlines(&strip_cr(&read_or_empty(&dir.join("expected.out"))));
    if got != want {
        return Err(format!(
            "FAIL  {name}  [lint: stdout content diff]\n{}",
            divergence(&want, &got)
        )
        .into());
    }
    Ok(())
}

/// The opt-in real-external-tools lint lane (`spike/CLAUDE.md` real-tools-lane-opt-in).
/// Registered ONLY when `DORC_E2E_REAL_TOOLS` is set; the LIST is the coverage assertion,
/// so a listed tool with no fixture, or an absent tool, fails loudly.
fn run_lint_real(
    harness: &Harness,
    tool: &str,
    fixture: Option<&PathBuf>,
    extra_path: &str,
) -> Result<(), Failed> {
    let Some(dir) = fixture.filter(|dir| dir.join("book.sh").is_file()) else {
        return Err(format!(
            "FAIL  lint-real/{tool}  [lint-real: listed tool has no fixture (a `lint-real-{tool}/book.sh` case dir)]"
        )
        .into());
    };
    let (coverage, finding): (&str, &[&str]) = match tool {
        "shellcheck" => (
            "\"name\":\"shellcheck\",\"status\":\"ran\"",
            &[
                "\"path\":\"book.sh\",\"line\":6,",
                "\"source\":\"shellcheck\",\"code\":\"SC2086\",",
                "\"remap\":\"exact\"",
            ],
        ),
        "checkbashisms" => (
            "\"name\":\"checkbashisms\",\"status\":\"ran\"",
            &[
                "\"path\":\"book.sh\",\"line\":6,",
                "\"source\":\"checkbashisms\",\"code\":\"external-text\",",
                "\"remap\":\"approximate\"",
            ],
        ),
        _ => {
            return Err(format!(
                "FAIL  lint-real/{tool}  [lint-real: unknown tool in DORC_E2E_REAL_TOOLS (no adapter/fixture)]"
            )
            .into());
        }
    };
    let out = capture(
        Command::new(&harness.dorc)
            .current_dir(dir)
            .env("PATH", extra_path)
            .args([
                "lint",
                "--format=jsonl",
                "--source",
                tool,
                "--require-tools",
                "book.sh",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    );
    // The lint exit trichotomy (`27R` §5) separates ran-with-findings from broken: 0 clean,
    // 1 findings, 3 OPERATIONAL — and 3 is where `--require-tools` reports an absent tool.
    // Reading `rc != 0` as absence read the healthiest outcome as the sickest
    // (`289:rider-real-tools-lane-rc-bitrot`).
    if !matches!(out.code, 0 | 1) {
        return Err(format!(
            "FAIL  lint-real/{tool}  [lint-real: dorc exited {} — a LISTED tool is absent/unrunnable (opt-in requires it)]\n{}",
            out.code,
            indent(&[out.stdout])
        )
        .into());
    }
    let mut bad = Vec::new();
    if !out.stdout.contains(coverage) {
        bad.push("coverage did not report the tool as ran".to_owned());
    }
    if !out
        .stdout
        .lines()
        .any(|line| finding.iter().all(|needle| line.contains(needle)))
    {
        bad.push(
            "expected-tier finding missing (stable code + original remapped line + fidelity)"
                .to_owned(),
        );
    }
    if !bad.is_empty() {
        return Err(format!(
            "FAIL  lint-real/{tool}  [lint-real: {}]\n{}",
            bad.join("; "),
            indent(&[out.stdout])
        )
        .into());
    }
    Ok(())
}

/// The `PATH` the real-tools lane runs under: the pinned shellcheck's dir, plus the
/// provisioned checkbashisms launcher, ahead of the ambient `PATH`.
fn real_tools_path(tools: &[String]) -> String {
    let ambient = std::env::var("PATH").unwrap_or_default();
    let mut prefix: Vec<String> = Vec::new();
    if which("mise").is_some() {
        let found = capture(
            Command::new("mise")
                .args(["which", "shellcheck"])
                .stdout(Stdio::piped())
                .stderr(Stdio::null()),
        );
        let located = strip_trailing_newlines(&found.stdout);
        if !located.is_empty()
            && let Some(parent) = Path::new(&located).parent()
        {
            prefix.push(parent.display().to_string());
        }
    }
    if tools.iter().any(|tool| tool == "checkbashisms") {
        let setup = spike_root().join("e2e/lint-real-tools-setup.sh");
        let out = capture(
            Command::new("sh")
                .arg(&setup)
                .stdout(Stdio::piped())
                .stderr(Stdio::null()),
        );
        if out.code == 0 {
            prefix.push(strip_trailing_newlines(&out.stdout));
        }
    }
    prefix.push(ambient);
    prefix.join(if cfg!(windows) { ";" } else { ":" })
}

// ---------------------------------------------------------------------------
// the pre-flight confound batteries (a lying judge is worse than no judge)

/// Drive the dual-rail judge on FABRICATED fixtures proving it screams on each failure
/// mode, and does NOT scream on the two negative controls.
fn dual_rail_selftest() -> Vec<String> {
    let shims = " instpkg systemctl ";
    let mut fails = Vec::new();
    let screams = |result: &[String], needle: &str| result.iter().any(|line| line.contains(needle));

    let cf1 = dual_rail_judge(
        "instpkg install nginx",
        "instpkg install nginx\nsystemctl restart sshd",
        "argv 1 run instpkg install nginx",
        shims,
        "",
    );
    if !screams(&cf1, "apply-only") {
        fails.push("cf-1 (apply-only line not caught)".to_owned());
    }
    let cf2 = dual_rail_judge(
        "instpkg install nginx\nsystemctl restart sshd",
        "instpkg install nginx",
        "argv 1 run instpkg install nginx",
        shims,
        "",
    );
    if !screams(&cf2, "unattributable") {
        fails.push("cf-2 (unattributable bare-only not caught)".to_owned());
    }
    let cf3 = dual_rail_judge(
        "systemctl restart sshd",
        "",
        "argv 7 run systemctl restart sshd",
        shims,
        "",
    );
    if !screams(&cf3, "unattributable") {
        fails.push("cf-3 (a `run`-disposition entry wrongly attributed an elided line)".to_owned());
    }
    let cf_pass = dual_rail_judge(
        "instpkg install nginx\ninstpkg install curl",
        "",
        "argv 0 replace instpkg install TOP",
        shims,
        "",
    );
    if !cf_pass.is_empty() {
        fails.push(format!(
            "cf-PASS (TOP-wildcard failed to license a converged member: {cf_pass:?})"
        ));
    }
    let cf5 = dual_rail_judge(
        "instpkg install nginx",
        "instpkg install nginx\nsystemctl restart sshd",
        "argv 2 guard instpkg install curl",
        shims,
        "",
    );
    if !screams(&cf5, "apply-only") {
        fails.push(
            "cf-5 (a guard disposition wrongly licensed an unrelated apply-only line)".to_owned(),
        );
    }
    let cf6 = dual_rail_judge(
        "instpkg install nginx",
        "dpkg-query nginx",
        "argv 2 guard instpkg install nginx",
        shims,
        "dpkg-query",
    );
    if !cf6.is_empty() {
        fails.push(format!(
            "cf-6 (a guard's own suppressed mutator + check-command was wrongly screamed: {cf6:?})"
        ));
    }
    fails
}

/// Drive the artifact-shape floor on the two demonstrated violation shapes plus a pass
/// control (`23C-fd4`).
fn guard_shape_selftest() -> Vec<String> {
    let book = "apt-get install -y nginx\napt-get install -y curl";
    let mut fails = Vec::new();
    let pass = guard_shape_violations(
        "apt_get__predict install -y curl || apt-get install -y curl   # dorc: guard [package converged-vouch; probe: holds]",
        book,
    );
    if !pass.is_empty() {
        fails.push(format!(
            "gf-PASS (a well-formed guard was wrongly flagged: {pass:?})"
        ));
    }
    let thin = guard_shape_violations(
        "dorc_guard curl   # dorc: guard [synthesized — no oracle body]",
        book,
    );
    if !thin.iter().any(|line| line.contains("thin")) {
        fails.push("gf-1 (an engine-synthesized thin guard was not caught)".to_owned());
    }
    let mutated = guard_shape_violations(
        "apt_get__predict install -y curl || apt-get install curl   # dorc: guard [package converged-vouch; probe: holds]",
        book,
    );
    if !mutated.iter().any(|line| line.contains("verbatim")) {
        fails.push("gf-2 (a mutated fall-through — dropped -y — was not caught)".to_owned());
    }
    fails
}

/// Drive the path-selection floor on FABRICATED name sets, proving it screams for a name no case
/// root answers to and stays quiet for one that is really there.
///
/// Both directions are load-bearing and they pull against each other. Screaming is what stops a
/// mistyped or stale hook path from buying a green no-op; staying quiet is what stops a commit
/// touching a real non-case path under `tests/` — an `.rs` test's fixture dir, an `aid` catalog
/// loom — from aborting a hook that had nothing to do. Collapsing either into the other is the
/// bug this battery exists to catch. The `.loom` mapping is here too, because the hook's
/// single-file-loom glob rides on it.
fn selection_floor_selftest() -> Vec<String> {
    let mut fails = Vec::new();
    let present: BTreeSet<String> = ["headline-partial", "cli-help-page", "golden"]
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    let minted: BTreeSet<&str> = ["headline-partial"].into_iter().collect();

    if case_from_path("spike/crates/cli/tests/whygallery-webhost-whole.loom").as_deref()
        != Some("whygallery-webhost-whole")
    {
        fails.push("sf-0 (a single-file loom path does not resolve to its slug)".to_owned());
    }
    for (name, want) in [
        ("headline-partial", Selection::Runs),
        ("cli-help-page", Selection::NoTrial),
        ("does-not-exist", Selection::Unknown),
    ] {
        let got = resolve_selection(name, &minted, &present);
        if got != want {
            fails.push(format!("sf-{name} (want {want:?}, got {got:?})"));
        }
    }
    fails
}

/// Drive the artifact-set gate's own requirement, in all three directions.
///
/// The failure this closes is a SILENT one: a case declaring `ARTIFACT_SET` whose run published
/// nothing would have every exec gate below it measure the plan alone in an empty sandbox — the
/// flattened world, under a multipart name — and pass. So "exactly one generation" has to be a
/// refusal rather than an `unwrap_or_default`, and a refusal nothing exercises is a refusal that
/// rots.
fn artifact_set_selftest() -> Vec<String> {
    let mut fails = Vec::new();
    let scratch = Scratch::new("artifactset");
    let root = scratch.path.join("artifacts");
    std::fs::create_dir_all(&root).expect("create artifact root");
    if published_generation(&root).is_ok() {
        fails.push("as-none (an empty artifact root answered a generation)".to_owned());
    }
    std::fs::create_dir_all(root.join("artifact-0001")).expect("create generation");
    match published_generation(&root) {
        Ok(one) if one.ends_with("artifact-0001") => {}
        other => fails.push(format!(
            "as-one (the sole generation did not answer: {other:?})"
        )),
    }
    std::fs::create_dir_all(root.join("artifact-0002")).expect("create second generation");
    if published_generation(&root).is_ok() {
        fails.push("as-two (two generations answered as if one drive published)".to_owned());
    }

    if ArtifactSetDeclaration::parse("published") != Ok(ArtifactSetDeclaration::Published) {
        fails.push("as-decl (the one legal declaration did not resolve)".to_owned());
    }
    if ArtifactSetDeclaration::parse("yes").is_ok() {
        fails.push("as-decl-open (a value outside the vocabulary was accepted)".to_owned());
    }

    let generation = root.join("artifact-0001");
    std::fs::write(generation.join("here.dorc-bundle.sh"), "true\n").expect("write bundle");
    let plan = ". './here.dorc-bundle.sh'\n. './gone.dorc-bundle.sh'\n. \"$ROOT/x.sh\"\n";
    if unresolved_generated_imports(&generation, plan) != vec!["./gone.dorc-bundle.sh".to_owned()] {
        fails.push(
            "as-imports (the import check did not name exactly the absent literal operand)"
                .to_owned(),
        );
    }
    fails
}

/// Drive the case-shape classifier over the three `book.sh` dirs it must tell apart
/// (`30Qa:fnd-missing-expected-out-hides-a-case`).
///
/// The middle answer is why this exists: a dir carrying `mocks/` and `probe-results.txt` but no
/// `expected.out` used to classify as a real-tools fixture, and a real-tools fixture is only ever
/// looked up by the name `lint-real-<tool>` — so the case left the suite silently, which is the
/// one thing a discovery floor may not do (`count-drifts`' residual, made loud for this shape).
fn case_shape_selftest() -> Vec<String> {
    let mut fails = Vec::new();
    let scratch = Scratch::new("caseshape");
    let root = scratch.path.join("cases");
    for (name, extra) in [
        ("shape-round-trip", vec!["expected.out"]),
        ("shape-missing-out", vec!["probe-results.txt"]),
        ("shape-real-tools", vec![]),
    ] {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).expect("create specimen case");
        std::fs::write(dir.join("book.sh"), "true\n").expect("write book");
        for file in extra {
            std::fs::write(dir.join(file), "").expect("write specimen file");
        }
    }
    let kinds: BTreeMap<String, E2eKind> = discover_e2e(&[root])
        .into_iter()
        .map(|case| (case.name, case.kind))
        .collect();
    for (name, want) in [
        ("shape-round-trip", E2eKind::RoundTrip),
        ("shape-missing-out", E2eKind::MissingExpectedOut),
        ("shape-real-tools", E2eKind::LintReal),
    ] {
        let got = kinds.get(name).copied();
        if got != Some(want) {
            fails.push(format!("cs-{name} (want {want:?}, got {got:?})"));
        }
    }
    fails
}

/// Drive the floor-transcript WRITE policy on a throwaway case: the refusal in both directions,
/// and the mint's fold plus its byte-stability. No committed golden and no floor binary take part
/// — gate-9 owns the measurement; what is proven here is who may commit it. Both directions are
/// load-bearing: a refusal that also caught ordinary cases would just be a broken bless.
fn floor_bless_selftest() -> Vec<String> {
    let mut fails = Vec::new();
    for (bless, mint, manifest, want) in [
        (true, false, true, true),
        (true, true, true, false),
        (true, false, false, false),
        (false, false, true, false),
        (false, true, true, false),
    ] {
        let got = floor_bless_refusal(bless, mint, manifest, "specimen").is_some();
        if got != want {
            fails.push(format!(
                "fb-refusal (bless={bless} mint={mint} manifest={manifest}: want refuse={want}, got {got})"
            ));
        }
    }

    let scratch = Scratch::new("floorbless");
    let dir = scratch.path.join("case");
    std::fs::create_dir_all(&dir).expect("create specimen dir");
    std::fs::write(dir.join("expected.out"), "fresh transcript\n").expect("write transcript");
    std::fs::write(dir.join("expected.emitted"), "live\n").expect("write manifest");
    std::fs::write(dir.join("expected.ran"), "").expect("write run-set");
    let source = "---\nrun: round-trip\nfixpoint: executed\n---\n-- book.sh --\n#!/bin/sh\nprintf 'live\\n'\n\n-- expected.emitted --\nstale\n\n-- expected.ran --\n\n-- replay --\n$ dorc --book=book.sh\nstale transcript\n";
    let loom = scratch.path.join("specimen.loom");
    let spec = |text: &str| LoomCaseSpec {
        name: "specimen".to_owned(),
        path: loom.clone(),
        case: errorloom::Case::parse(text).expect("parse specimen"),
        run: LoomRun::RoundTrip,
    };

    if bless_loom(&spec(source), &dir, &[], false).is_err() {
        fails
            .push("fb-default (the default fold refused a case it should have blessed)".to_owned());
    }
    let defaulted = read_or_empty(&loom);
    if !defaulted.contains("-- expected.emitted --\nstale\n") {
        fails.push("fb-measure-once (a default bless rewrote the measured manifest)".to_owned());
    }
    if !defaulted.contains("fresh transcript") {
        fails.push(
            "fb-transcript (the default fold did not commit the fresh transcript)".to_owned(),
        );
    }

    if bless_loom(&spec(source), &dir, &[], true).is_err() {
        fails.push("fb-mint (the mint fold refused a well-formed case)".to_owned());
    }
    let minted = read_or_empty(&loom);
    if !minted.contains("-- expected.emitted --\nlive\n") {
        fails.push(
            "fb-fold (the mint did not fold the re-measured manifest into the case)".to_owned(),
        );
    }
    // Idempotence: minting an already-correct case must move no byte, or every mint of one drifted
    // cell would carry its neighbours' whitespace along for the ride.
    if bless_loom(&spec(&minted), &dir, &[], true).is_err() {
        fails.push("fb-remint (a second mint over its own output refused)".to_owned());
    }
    if read_or_empty(&loom) != minted {
        fails.push("fb-stable (minting an already-correct case is not byte-stable)".to_owned());
    }
    fails
}

/// Drive the FOLD-ONLY-ON-PASS rule on two throwaway cases under a synthetic bless harness: a
/// case whose gates fail must leave its committed bytes exactly as authored, and a passing one
/// must still fold. Both looms are written NON-canonically (no blank line before a header), which
/// `Case::to_text` normalizes — so "was it written at all" is a byte question, not a content one.
///
/// Ungated, `exec_check` writes `expected.ran` and returns before the later gates, so a case that
/// failed one of THOSE still had its run-set folded while its transcript stayed stale.
fn bless_folds_only_on_pass_selftest(harness: &Harness) -> Vec<String> {
    let mut fails = Vec::new();
    let scratch = Scratch::new("foldpass");
    // Its own state root: `Harness::drop` removes that dir, and sharing the real one would take
    // the live harness's receipts down with this specimen.
    let bless = Harness {
        dorc: harness.dorc.clone(),
        dorc_sh: harness.dorc_sh.clone(),
        checker: harness.checker.clone(),
        checker_name: harness.checker_name.clone(),
        bless: true,
        bless_floor: false,
        floor_shells: Vec::new(),
        // Its OWN profile: each sandbox removes itself on drop, and sharing the live harness's
        // would take its receipts down with this specimen.
        profile: ProfileSandbox::new("foldpass"),
    };

    // `if true` with no `fi` is a parse error, so dorc exits non-zero and the crash/empty guard
    // fails the case before any golden is consulted.
    for (tag, book, want_written) in [
        ("fold-pass-failing", "#!/bin/sh\nif true", false),
        ("fold-pass-passing", "#!/bin/sh\nhork tune", true),
    ] {
        let path = scratch.path.join(format!("{tag}.loom"));
        let source = format!(
            "---\nrun: round-trip\nfixpoint: executed\n---\n-- book.sh --\n{book}\n-- expected.ran --\n-- replay --\n$ dorc --book=book.sh --results -\nplaceholder\n"
        );
        std::fs::write(&path, &source).expect("write specimen loom");
        let spec = LoomCaseSpec {
            name: tag.to_owned(),
            path: path.clone(),
            case: errorloom::Case::parse(&source).expect("parse specimen"),
            run: LoomRun::RoundTrip,
        };
        let passed = run_loom(&bless, &spec).is_ok();
        if passed != want_written {
            fails.push(format!(
                "bf-{tag} (want the case to {}, but it did not)",
                if want_written {
                    "pass"
                } else {
                    "fail its gates"
                }
            ));
        }
        if (read_or_empty(&path) != source) != want_written {
            fails.push(format!(
                "bf-{tag}-fold (a {} case {} its committed bytes)",
                if want_written { "passing" } else { "failing" },
                if want_written {
                    "did not fold"
                } else {
                    "folded into"
                }
            ));
        }
    }
    fails
}

/// The `DORC_FLAGS` plumbing confound: run the flagship with and without
/// `--risk-faultless-skips` and assert the elision count DIFFERS. If it matches, the flag is
/// inert and a flagged survival case's gate-6 attribution would lie.
fn dorc_flags_selftest(harness: &Harness) -> Option<String> {
    let dir = own_cases().join("strawman24-survive-multiwall");
    if !dir.is_dir() {
        // The sh original skipped a missing anchor. Since the collection moved, a missing
        // anchor now most likely means the runner is looking in the wrong place — the one
        // failure that would otherwise disable this battery in silence.
        return Some(format!(
            "dorc_flags_selftest: the flagship anchor is missing ({}) — the case collection is not where the runner looks.",
            dir.display()
        ));
    }
    let oracle = dir.join("package.oracle.sh").display().to_string();
    let book = format!("--book={}", dir.join("book.sh").display());
    let elide = |args: &[String]| -> String {
        let scratch = Scratch::new("flagself");
        let framed = scratch.path.join("framed.txt");
        std::fs::write(&framed, framed_results(harness, &dir, args)).expect("write framed");
        let out = capture(
            harness
                .dorc(&dir)
                .arg(&book)
                .args(args)
                .stdin(Stdio::from(std::fs::File::open(&framed).unwrap()))
                .stdout(Stdio::null())
                .stderr(Stdio::piped()),
        );
        // `grep -oE 'elide=[0-9]+'` — the count, not whatever punctuation trails it.
        out.stderr
            .match_indices("elide=")
            .map(|(at, _)| {
                let digits: String = out.stderr[at + "elide=".len()..]
                    .chars()
                    .take_while(char::is_ascii_digit)
                    .collect();
                format!("elide={digits}")
            })
            .filter(|found| found != "elide=")
            .collect::<Vec<_>>()
            .join("\n")
    };
    // `--results -` is the stdin claim this battery's own pipe now needs spelling
    // (`owed-no-flag-defaults-to-stdin`); without it the run measures nothing and BOTH arms read
    // elide=0, which is a false equality rather than a real one.
    let flagged = elide(&[
        "--pre-source".to_owned(),
        oracle.clone(),
        "--risk-faultless-skips".to_owned(),
        "--results".to_owned(),
        "-".to_owned(),
    ]);
    let plain = elide(&[
        "--pre-source".to_owned(),
        oracle,
        "--results".to_owned(),
        "-".to_owned(),
    ]);
    (flagged == plain).then(|| format!(
        "dorc_flags_selftest FAILED — --risk-faultless-skips did not change the flagship's elision count ({flagged} flagged vs {plain} plain); the flag is not reaching the engine, so a flagged survival case's gate-6 attribution would lie."
    ))
}

/// The strip-and-exec off-ramp smoke (`27D` rider-dorc-sh-unbuilt): `dorc strip` leaves a
/// `-n`-clean, dialect-free text, and `dorc-sh` strips-and-execs a marked script. The
/// smoke script is BUILTIN-ONLY, so nothing mutating can run whatever the `PATH`.
fn dorc_sh_smoke(harness: &Harness) -> Option<String> {
    if !harness.dorc_sh.is_file() {
        return Some(format!(
            "dorc_sh_smoke: dorc-sh binary not found next to dorc ({})",
            harness.dorc.display()
        ));
    }
    let oracle = own_cases().join("strawman24-alias-provides/package.oracle.sh");
    if !oracle.is_file() {
        return Some(format!(
            "dorc_sh_smoke: the strip anchor is missing ({}) — the case collection is not where the runner looks.",
            oracle.display()
        ));
    }
    {
        let stripped = capture(
            Command::new(&harness.dorc)
                .arg("strip")
                .arg(&oracle)
                .stdout(Stdio::piped())
                .stderr(Stdio::null()),
        )
        .stdout;
        if harness
            .syntax_error(&strip_trailing_newlines(&stripped))
            .is_some()
        {
            return Some(format!(
                "dorc_sh_smoke: 'dorc strip' output is not {} -n clean",
                harness.checker_name
            ));
        }
        for dialect in ["env dorc-sh", ": sm.", "invariant:"] {
            if stripped.contains(dialect) {
                return Some(
                    "dorc_sh_smoke: 'dorc strip' left a dialect construct (dorc-sh shebang / mark / bare-mark)".to_owned(),
                );
            }
        }
    }
    let scratch = Scratch::new("dorcsh");
    let marked = scratch.path.join("marked.sh");
    std::fs::write(
        &marked,
        "#!/usr/bin/env dorc-sh\n# dorc-lang/v0.2\nsmoke__predict() {\n   pkg : sm.dorc.Package = \"$1\"\n   printf 'dorc-sh-smoke ran: %s\\n' \"$pkg\"\n}\nsmoke__predict nginx\n",
    )
    .expect("write smoke script");
    // The harness's own resolved shell, not a fresh PATH search: on Windows there is
    // nothing named `sh` on PATH to find.
    let sh_dir = harness
        .checker
        .parent()
        .map_or_else(|| PathBuf::from("/bin"), Path::to_path_buf);
    let out = capture(
        Command::new(&harness.dorc_sh)
            .current_dir(&scratch.path)
            .env_clear()
            .env("PATH", &sh_dir)
            .env("LC_ALL", "C")
            .env("TZ", "UTC")
            .arg(&marked)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
    if out.code != 0 {
        return Some(format!(
            "dorc_sh_smoke: dorc-sh exited {} on a marked script (expected 0): {}{}",
            out.code, out.stdout, out.stderr
        ));
    }
    if !out.stdout.contains("dorc-sh-smoke ran: nginx") {
        return Some(format!(
            "dorc_sh_smoke: the stripped body did not run as expected (got: {})",
            out.stdout
        ));
    }
    gate_differential(harness, &sh_dir)
}

/// The and-or GATE differential: the tracer's static answer for `[ … ] || return 2` must be the
/// one a real shell gives. The tracer says a second operand trips the gate and a single operand
/// does not; here the SAME body runs under the harness's POSIX shell, through `dorc-sh` (which
/// strips and execs it), and its stdout/rc say which path the shell actually took.
///
/// Builtin-only by construction — the body is `[`, `printf`, and `return`, and `PATH` is the shell's
/// own directory — so this runs no tool whatever the environment, exactly as the smoke above does.
/// This is the closest sanctioned execution differential the corpus has: it rides the one runner
/// allowed to execute fixture material rather than opening a second execution lane.
fn gate_differential(harness: &Harness, sh_dir: &Path) -> Option<String> {
    let scratch = Scratch::new("dorcgate");
    let marked = scratch.path.join("gate.sh");
    std::fs::write(
        &marked,
        "#!/usr/bin/env dorc-sh\n\
         # dorc-lang/v0.2\n\
         gate__predict() {\n   \
            [ \"${2-}\" = \"\" ] || return 2\n   \
            pkg : sm.dorc.Package = \"$1\"\n   \
            printf 'gate cleared: %s\\n' \"$pkg\"\n\
         }\n\
         gate__predict \"$@\"\n\
         printf 'rc=%s\\n' \"$?\"\n",
    )
    .expect("write the gate script");
    // (argv, what the TRACER says: cleared, and the body's rc)
    for (argv, cleared, rc) in [
        (vec!["nginx"], true, "0"),
        (vec!["nginx", "curl"], false, "2"),
    ] {
        let mut command = Command::new(&harness.dorc_sh);
        command
            .current_dir(&scratch.path)
            .env_clear()
            .env("PATH", sh_dir)
            .env("LC_ALL", "C")
            .env("TZ", "UTC")
            .arg(&marked);
        for arg in &argv {
            command.arg(arg);
        }
        let out = capture(command.stdout(Stdio::piped()).stderr(Stdio::piped()));
        let ran = out.stdout.contains("gate cleared: nginx");
        if ran != cleared || !out.stdout.contains(&format!("rc={rc}")) {
            return Some(format!(
                "gate_differential: for argv {argv:?} the tracer says cleared={cleared} rc={rc}, \
                 but {} actually produced: {}{}",
                harness.checker_name, out.stdout, out.stderr
            ));
        }
    }
    None
}

/// Run every confound battery before any case. A failing battery ABORTS (exit 3) exactly
/// as the sh harness does: a lying judge is worse than no judge, so no case may report
/// green underneath one.
fn preflight(harness: &Harness, discovered: usize) {
    let mut fatal: Vec<String> = Vec::new();
    // The DISCOVERY FLOOR. Walking the wrong roots yields zero trials, and a suite of zero
    // trials EXITS GREEN — the one failure mode this runner's own path constants can cause
    // and not report. A count is deliberately not pinned (`count-drifts`); non-empty is.
    if discovered == 0 {
        fatal.push(format!(
            "FATAL  discovery floor: no cases found under any of {:?} — the collection is not where the runner looks, and an empty suite would otherwise pass.",
            case_roots()
        ));
    }
    let dual = dual_rail_selftest();
    if !dual.is_empty() {
        fatal.push(format!(
            "FATAL  dual_rail_selftest FAILED — the cm-1 judge does not scream as required:\n  {}",
            dual.join("\n  ")
        ));
    }
    let shape = guard_shape_selftest();
    if !shape.is_empty() {
        fatal.push(format!(
            "FATAL  guard_shape_selftest FAILED — the artifact-shape floor does not scream as required:\n  {}",
            shape.join("\n  ")
        ));
    }
    let selection = selection_floor_selftest();
    if !selection.is_empty() {
        fatal.push(format!(
            "FATAL  selection_floor_selftest FAILED — path selection does not sort real paths from absent ones:\n  {}",
            selection.join("\n  ")
        ));
    }
    let shapes = case_shape_selftest();
    if !shapes.is_empty() {
        fatal.push(format!(
            "FATAL  case_shape_selftest FAILED — a mis-authored round-trip case could leave the suite in silence:\n  {}",
            shapes.join("\n  ")
        ));
    }
    let artifact_set = artifact_set_selftest();
    if !artifact_set.is_empty() {
        fatal.push(format!(
            "FATAL  artifact_set_selftest FAILED — a declared artifact set could go unpublished and every exec gate would measure the plan alone:\n  {}",
            artifact_set.join("\n  ")
        ));
    }
    let floor_bless = floor_bless_selftest();
    if !floor_bless.is_empty() {
        fatal.push(format!(
            "FATAL  floor_bless_selftest FAILED — the floor-transcript write policy does not hold:\n  {}",
            floor_bless.join("\n  ")
        ));
    }
    let fold_pass = bless_folds_only_on_pass_selftest(harness);
    if !fold_pass.is_empty() {
        fatal.push(format!(
            "FATAL  bless_folds_only_on_pass_selftest FAILED — bless folds a case its own gates rejected:\n  {}",
            fold_pass.join("\n  ")
        ));
    }
    if let Some(message) = dorc_flags_selftest(harness) {
        fatal.push(format!("FATAL  {message}"));
    }
    if let Some(message) = dorc_sh_smoke(harness) {
        fatal.push(format!("FATAL  {message}"));
    }
    if !fatal.is_empty() {
        for message in fatal {
            eprintln!("{message}");
        }
        eprintln!("aborting.");
        std::process::exit(3);
    }
}

fn run_bundle_integration(harness: &Harness) -> Result<(), Failed> {
    let located = Scratch::new("bundle-locator");
    std::fs::write(located.path.join("book.sh"), ". ./entry.sh\n").expect("write bundle book");
    std::fs::write(
        located.path.join("entry.sh"),
        "# dorc-lang/v0.2\n. ./dep.sh\n",
    )
    .expect("write bundle entry");
    std::fs::write(
        located.path.join("dep.sh"),
        "# dorc-lang/v0.2\nrunas__lend_map() {\n   printf '%s\\n' \"$1\" : lends frobnicate\n}\n",
    )
    .expect("write bundle dependency");
    let output = capture(
        harness
            .dorc(&located.path)
            .args(["bundle", "book.sh"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
    if output.code != 0
        || !output.stdout.contains("# dorc-bundle-set/v0")
        || !output.stderr.contains("dep.sh:3:")
        || !output
            .stderr
            .contains("dorc-bundle/v0/root-00000000/occurrence-00000001.sh:3:")
        || !output.stderr.contains("book.sh:1:")
    {
        return Err(format!(
            "bundle locator integration failed (rc={}):\nstdout:\n{}\nstderr:\n{}",
            output.code, output.stdout, output.stderr
        )
        .into());
    }

    let refused = Scratch::new("bundle-refusal");
    std::fs::write(refused.path.join("book.sh"), "hork setup\n").expect("write refusal book");
    std::fs::write(
        refused.path.join("w.oracle.sh"),
        "# dorc-lang/v0.2\nw__predict() { verb=$1; shift; env \"$@\"; }\nw__lend_map() { : lends user; : lends fs-view; : lends netns; \"$@\"; }\n",
    )
    .expect("write incoherent wrapper");
    let output = capture(
        harness
            .dorc(&refused.path)
            .args(["bundle", "book.sh", "--pre-source", "w.oracle.sh"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
    if output.code != 11
        || !output.stdout.is_empty()
        || !output.stderr.contains("wrapper-peel-incoherent")
    {
        return Err(format!(
            "bundle incoherence refusal failed (rc={}):\nstdout:\n{}\nstderr:\n{}",
            output.code, output.stdout, output.stderr
        )
        .into());
    }
    Ok(())
}

/// The KEPT-STREAM refusal, driven for real: a stdout the user is keeping carries a COMPLETE plan
/// or the run stops before the network, with NOTHING on stdout
/// (`30Ng:rul-piped-stdout-carries-a-full-plan`, human-typed).
///
/// Native rather than declarative, and that is the point (`30Nh`, the named harness gap): the
/// round-trip battery hard-fails empty output before any lens, so a case whose whole behaviour is a
/// pre-network refusal with no artifact, no run-set and no transcript has no axis for it to compare.
/// Every key that battery owns would be absent, and what is left is a marker plus a negation.
///
/// CFG SHAPE: one top-level `.` of a dorc-lang package standing as an `||` RIGHT operand — outside
/// `floor30-inline-dot-boundary`'s measured cell, so the single stream cannot carry the bundle where
/// the load stands and no complete plan exists for a kept stream to hold.
fn run_kept_stream_refusal(harness: &Harness) -> Result<(), Failed> {
    let kept = Scratch::new("kept-stream-refusal");
    std::fs::write(
        kept.path.join("book.sh"),
        "false || . ./wombat.dorc.sh\nwombat sync a.conf\n",
    )
    .expect("write kept-stream book");
    std::fs::write(
        kept.path.join("wombat.dorc.sh"),
        "# dorc-lang/v0.2\nwombat__is_converged() { :; }\n",
    )
    .expect("write kept-stream package");
    let output = capture(
        harness
            .dorc(&kept.path)
            .args(["plan", "book.sh"])
            .env(STDOUT_POSTURE_ENV, "non-interactive")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
    if output.code == 0
        || !output.stdout.is_empty()
        || !output.stderr.contains("artifact-form-refused")
    {
        return Err(format!(
            "kept-stream refusal failed (rc={}):\nstdout:\n{}\nstderr:\n{}",
            output.code, output.stdout, output.stderr
        )
        .into());
    }
    Ok(())
}

// ---------------------------------------------------------------------------

/// The case a repo path belongs to, or `None` for an argument that is not a case path.
///
/// The case is the segment after `tests/`, so a file nested in a case's `mocks/` attributes to its
/// case rather than to `mocks`. Both separators are accepted because the caller is a git hook and
/// git reports forward slashes even on Windows.
fn main() {
    let (passthrough, changed) = split_path_selectors(std::env::args());
    let mut args = Arguments::from_iter(passthrough);
    if args.format.is_none() && std::env::var("DORC_E2E_QUIET").as_deref() == Ok("1") {
        args.format = Some(libtest_mimic::FormatSetting::Terse);
    }
    let harness = Arc::new(Harness::resolve());
    let discovered = discover_e2e(&case_roots());
    let looms = discover_looms(&case_roots());
    preflight(&harness, discovered.len());

    let closed_loop_dir = discovered
        .iter()
        .find(|case| {
            case.name == CLOSED_LOOP_CASE
                && case.dir.join("mocks").is_dir()
                && case.dir.join("probe-results.txt").is_file()
        })
        .map(|case| case.dir.clone());

    let mut trials: Vec<Trial> = Vec::new();
    {
        let harness = Arc::clone(&harness);
        trials.push(Trial::test("bundle-integration".to_owned(), move || {
            run_bundle_integration(&harness)
        }));
    }
    {
        let harness = Arc::clone(&harness);
        trials.push(Trial::test("kept-stream-refusal".to_owned(), move || {
            run_kept_stream_refusal(&harness)
        }));
    }
    for loom in looms {
        match loom_spec(&loom) {
            Ok(None) => {}
            Ok(Some(spec)) => {
                assert!(
                    !discovered.iter().any(|case| case.name == spec.name),
                    "`{}` exists in both dir and loom form — a half-finished conversion runs the case twice under one filter name",
                    spec.name
                );
                let harness = Arc::clone(&harness);
                let spec = Arc::new(spec);
                trials.push(Trial::test(spec.name.clone(), move || {
                    run_loom(&harness, &spec)
                }));
            }
            Err(message) => {
                let name = loom.name.clone();
                trials.push(Trial::test(name.clone(), move || {
                    Err(format!("FAIL  {name}  [loom: {message}]").into())
                }));
            }
        }
    }
    let mut real_fixtures: BTreeMap<String, PathBuf> = BTreeMap::new();
    for case in discovered {
        let harness = Arc::clone(&harness);
        match case.kind {
            E2eKind::RoundTrip => trials.push(Trial::test(case.name.clone(), move || {
                run_round_trip(&harness, &case, &mut String::new())
            })),
            E2eKind::Lint => trials.push(Trial::test(case.name.clone(), move || {
                run_lint(&harness, &case)
            })),
            E2eKind::LintReal => {
                real_fixtures.insert(case.name.clone(), case.dir);
            }
            E2eKind::MissingExpectedOut => {
                let name = case.name.clone();
                trials.push(Trial::test(name.clone(), move || {
                    let residue = support::round_trip_residue(&case.dir).join(", ");
                    Err(format!(
                        "FAIL  {name}  [a round-trip case needs `expected.out`; \
                         this dir also carries: {residue}. Mint it (an empty file is enough — \
                         `BLESS=1` fills it) or reduce the dir to `book.sh` alone for the \
                         real-tools lane.]"
                    )
                    .into())
                }));
            }
        }
    }
    if let Ok(list) = std::env::var("DORC_E2E_REAL_TOOLS")
        && !list.is_empty()
    {
        let tools: Vec<String> = list.split(',').map(str::to_owned).collect();
        let path = Arc::new(real_tools_path(&tools));
        let fixtures = Arc::new(real_fixtures);
        for tool in tools {
            let harness = Arc::clone(&harness);
            let path = Arc::clone(&path);
            let fixtures = Arc::clone(&fixtures);
            trials.push(Trial::test(format!("lint-real/{tool}"), move || {
                run_lint_real(
                    &harness,
                    &tool,
                    fixtures.get(&format!("lint-real-{tool}")),
                    &path,
                )
            }));
        }
    }

    if let Some(dir) = closed_loop_dir {
        let harness = Arc::clone(&harness);
        let mocks = dir.join("mocks");
        trials.push(Trial::test("closed-loop".to_owned(), move || {
            run_closed_loop(&harness, &dir, &mocks)
        }));
    }

    if !changed.is_empty() {
        let minted: BTreeSet<&str> = trials.iter().map(Trial::name).collect();
        if !report_path_selection(&changed, &minted, &case_roots()) {
            return;
        }
        trials.retain(|trial| changed.contains(trial.name()));
    }
    libtest_mimic::run(&args, trials).exit();
}
