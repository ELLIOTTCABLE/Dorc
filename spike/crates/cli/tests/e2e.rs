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

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use libtest_mimic::{Arguments, Failed, Trial};

use support::{E2eCase, E2eKind, LoomCase, case_roots, discover_e2e, discover_looms, spike_root};

/// This crate's own `tests/` dir — the home of the round-trip collection, and the anchor
/// the pre-flight batteries resolve their specimens against.
fn own_cases() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// The fixed spike nonce and terminal token of the `dorc-records/1` framing (`262` §2).
/// These MIRROR `plan::records::{DEFAULT_NONCE, TERMINAL_TOKEN}` — keep the two in sync.
const RECORDS_NONCE: &str = "dorc";
/// The per-record terminal token (see [`RECORDS_NONCE`]).
const RECORDS_TOKEN: &str = "@@dorc@@";

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

/// Resolve `name` on `PATH`, honouring the platform's executable extensions.
fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exts: Vec<String> = std::env::var("PATHEXT")
        .map(|raw| raw.split(';').map(str::to_ascii_lowercase).collect())
        .unwrap_or_default();
    for dir in std::env::split_paths(&path) {
        let bare = dir.join(name);
        if bare.is_file() {
            return Some(bare);
        }
        for ext in &exts {
            let with_ext = dir.join(format!("{name}{ext}"));
            if with_ext.is_file() {
                return Some(with_ext);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// the harness's shared, immutable context

/// Binaries, the syntax checker, and the run mode — resolved once, shared by every trial.
/// The engine's harness clock pin, and the instant every case's transcript is dated by
/// (`rul-fixture-identity-never-production`). A round number in 2026 so a reader of a committed
/// transcript can tell at a glance that the date is fixture, not a real morning.
const FIXTURE_CLOCK_ENV: &str = "DORC_FIXTURE_CLOCK_MS";
const FIXTURE_CLOCK_MS: u64 = 1_769_306_437_000;

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
    /// The throwaway per-user state root every invocation is pointed at, so default-on receipts
    /// land here instead of in the developer's real profile directory.
    state_root: PathBuf,
}

impl Drop for Harness {
    /// Take the throwaway state root with us. Default-on means every case leaves receipts, and a
    /// suite that grows a new litter of them per run is a suite nobody wants to keep running.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.state_root);
    }
}

impl Harness {
    /// Resolve the harness context, aborting loudly when the `-n` gate has no shell.
    fn resolve() -> Self {
        let (checker_name, checker) = ["dash", "sh"]
            .iter()
            .find_map(|name| which(name).map(|path| ((*name).to_owned(), path)))
            .unwrap_or_else(|| {
                eprintln!(
                    "no POSIX shell (dash/sh) for the ap-2 syntax gate — cannot validate runnability"
                );
                std::process::exit(2);
            });
        let state_root =
            std::env::temp_dir().join(format!("dorc-e2e-state-{}", std::process::id()));
        std::fs::create_dir_all(&state_root).expect("create the harness state root");
        Self {
            dorc: PathBuf::from(env!("CARGO_BIN_EXE_dorc")),
            dorc_sh: PathBuf::from(env!("CARGO_BIN_EXE_dorc-sh")),
            checker,
            checker_name,
            bless: std::env::var("BLESS").as_deref() == Ok("1"),
            state_root,
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
    fn dorc(&self) -> Command {
        let mut command = Command::new(&self.dorc);
        command.env(FIXTURE_CLOCK_ENV, FIXTURE_CLOCK_MS.to_string());
        for key in ["XDG_STATE_HOME", "LOCALAPPDATA"] {
            command.env(key, &self.state_root);
        }
        command.env_remove("HOME");
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
        let mut command = Command::new(&self.checker);
        command
            .current_dir(sandbox)
            .env_clear()
            .env("PATH", path)
            .env("DORC_LOG", log)
            .env("LC_ALL", "C")
            .env("TZ", "UTC");
        if cfg!(unix) {
            let checker = self.checker.display().to_string();
            match script {
                Some(script) => {
                    command
                        .arg("-c")
                        .arg("umask 022; exec \"$0\" \"$1\"")
                        .arg(&checker)
                        .arg(script);
                }
                None => {
                    command
                        .arg("-c")
                        .arg("umask 022; exec \"$0\"")
                        .arg(&checker);
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
    fn capture_run(&self, payload: Payload<'_>, mocks: &Path) -> String {
        let scratch = Scratch::new("run");
        let log = scratch.path.join("dorc.log");
        std::fs::write(&log, "").expect("seed log");
        let sandbox = scratch.path.join("sand");
        std::fs::create_dir_all(&sandbox).expect("create sandbox");

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
    let ledger: Vec<&str> = lines_of(disp)
        .iter()
        .filter_map(|line| {
            let rest = line.strip_prefix("argv ")?;
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
            .dorc()
            .arg("probe")
            .arg(format!("--book={}", dir.join("book.sh").display()))
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;

    let header = probe
        .lines()
        .find(|line| line.contains("dorc-records/1"))
        .and_then(|line| line.split('\'').nth(1))
        .map(|field| field.strip_suffix("\\n").unwrap_or(field).to_owned());

    let mut sites: Vec<String> = Vec::new();
    for line in probe.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        for pair in fields.windows(2) {
            let (key, value) = (pair[0], pair[1]);
            if key == "site" && is_site_key(value) && !sites.iter().any(|seen| seen == value) {
                sites.push(value.to_owned());
            }
        }
    }

    let Some(header) = header else {
        return String::new();
    };

    let raw = read_or_empty(&dir.join("probe-results.txt"));
    let wanted: BTreeSet<&str> = sites.iter().map(String::as_str).collect();
    let mut body: Vec<String> = Vec::new();
    let mut deriv_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut deriv_order: Vec<String> = Vec::new();
    let mut deriv_closed: BTreeSet<String> = BTreeSet::new();
    for raw_line in raw.lines() {
        if raw_line.starts_with("dorc-records/1 ") || raw_line.starts_with("dorc-records-end/1 ") {
            continue;
        }
        let stripped = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let stripped = stripped.strip_prefix("dorc ").unwrap_or(stripped);
        let line = stripped
            .strip_suffix(&format!(" {RECORDS_TOKEN}"))
            .unwrap_or(stripped)
            .to_owned();
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.first().copied() {
            Some("site") if !fields.get(1).is_some_and(|id| wanted.contains(id)) => continue,
            Some("deriv") => {
                if let Some(site) = fields.get(1) {
                    let site = (*site).to_owned();
                    *deriv_counts.entry(site.clone()).or_default() += 1;
                    if !deriv_order.contains(&site) {
                        deriv_order.push(site);
                    }
                }
            }
            Some("deriv-end") => {
                if let Some(site) = fields.get(1) {
                    deriv_closed.insert((*site).to_owned());
                }
            }
            _ => {}
        }
        body.push(line);
    }
    for site in &deriv_order {
        if !deriv_closed.contains(site) {
            body.push(format!(
                "deriv-end {site} n={}",
                deriv_counts.get(site).copied().unwrap_or_default()
            ));
        }
    }

    let mut out = String::new();
    out.push_str(&header);
    out.push('\n');
    for line in body {
        let line = if line.trim_start().starts_with("site ") && !line.contains(" rc=") {
            format!("{line} rc=0")
        } else {
            line
        };
        let _ = writeln!(out, "{RECORDS_NONCE} {line} {RECORDS_TOKEN}");
    }
    for site in &sites {
        if !out
            .lines()
            .any(|line| line.starts_with(&format!("{RECORDS_NONCE} site {site} ")))
        {
            let _ = writeln!(
                out,
                "{RECORDS_NONCE} site {site} effect=cant-tell rc=0 {RECORDS_TOKEN}"
            );
        }
    }
    let _ = writeln!(
        out,
        "dorc-records-end/1 nonce={RECORDS_NONCE} {RECORDS_TOKEN}"
    );
    out
}

/// A site key is `N` or, for an in-loop Members member, `N.M`.
fn is_site_key(value: &str) -> bool {
    let mut parts = value.split('.');
    let head = parts.next().unwrap_or_default();
    let tail = parts.next();
    parts.next().is_none()
        && !head.is_empty()
        && head.chars().all(|c| c.is_ascii_digit())
        && tail.is_none_or(|t| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()))
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
const LOOM_KEYS: [&str; 13] = [
    "run",
    "fixpoint",
    "flags",
    "exit",
    "apply-exit",
    "tolerate",
    "probe-results",
    "dual-rail",
    "why-addr",
    "expect-diagnostic",
    "expect-why",
    "expect-hint",
    "expect-why-chain",
];

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
    let known: BTreeSet<&str> = LOOM_KEYS.iter().copied().collect();
    if let Some(unknown) = parsed.frontmatter().keys().find(|key| !known.contains(key)) {
        return Err(format!(
            "unread frontmatter key `{unknown}` — the key vocabulary is closed, and a key no gate reads is an assertion the author only believes they made"
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
        let _ = write!(command, " -o {oracle}");
    }
    if let Ok(Some(flags)) = marker(dir, "DORC_FLAGS") {
        let _ = write!(command, " {flags}");
    }
    if dir.join("probe-results.txt").is_file() {
        command.push_str(" < probe-results.txt");
    }
    command
}

/// Drive one loom-form case: materialize, run the dir-form battery, then fold any blessed
/// bytes back into the `.loom` (the loom, not the scratch dir, is what is committed).
fn run_loom(harness: &Harness, spec: &LoomCaseSpec) -> Result<(), Failed> {
    let scratch = Scratch::new("loom");
    let dir = scratch.path.join(&spec.name);
    std::fs::create_dir_all(&dir).expect("create loom case dir");
    materialize_loom(spec, &dir)
        .map_err(|error| Failed::from(format!("FAIL  {}  [loom: {error}]", spec.name)))?;

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
    let outcome = match spec.run {
        LoomRun::RoundTrip => run_round_trip(harness, &case),
        LoomRun::Lint => run_lint(harness, &case),
    };
    let (extra, extra_failures) = drive_extra_replays(harness, spec, &dir);
    if harness.bless {
        bless_loom(spec, &dir, &extra)?;
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
) -> (Vec<String>, Vec<String>) {
    let blocks = spec.case.replay().blocks();
    if blocks.len() < 2 {
        return (Vec::new(), Vec::new());
    }
    let name = &spec.name;
    let mut failures: Vec<String> = Vec::new();
    let args = match shared_args(dir) {
        Ok(args) => args,
        Err(message) => {
            return (
                Vec::new(),
                vec![format!("FAIL  {name}  [replay: {message}]")],
            );
        }
    };
    let scratch = Scratch::new("replay");
    let framed_path = scratch.path.join("framed.txt");
    std::fs::write(&framed_path, framed_results(harness, dir, &args)).expect("write framed");

    let mut outputs: Vec<String> = Vec::new();
    for (index, block) in blocks.iter().enumerate().skip(1) {
        match run_replay_block(harness, dir, &framed_path, block.command()) {
            Ok(got) if scratch_path_leaked(&got, dir) => failures.push(format!(
                "FAIL  {name}  [replay {index}: `{}` echoed the throwaway materialization path — a transcript carrying a machine-specific absolute path is not committable (`282` §7); spell the invocation with case-relative paths]",
                block.command()
            )),
            Ok(got) => {
                if !harness.bless && got != block.output() {
                    failures.push(format!(
                        "FAIL  {name}  [replay {index}: `{}` no longer reproduces its committed transcript]\n{}",
                        block.command(),
                        divergence(
                            &strip_trailing_newlines(block.output()),
                            &strip_trailing_newlines(&got)
                        )
                    ));
                }
                outputs.push(got);
            }
            Err(message) => failures.push(format!(
                "FAIL  {name}  [replay {index}: `{}` — {message}]",
                block.command()
            )),
        }
    }
    if failures.is_empty() {
        (outputs, failures)
    } else {
        (Vec::new(), failures)
    }
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

/// Execute one committed replay command and return its stdout as transcript bytes.
///
/// The accepted shape is deliberately tiny: a `dorc` invocation, optionally reading the case's
/// `probe-results.txt` (which resolves to the framed stream the battery feeds, exactly as block
/// 0's committed `< probe-results.txt` does) and optionally discarding stdout. stderr is dropped
/// — `stdout-contract` makes stdout the product surface — so a non-zero exit is the only
/// tripwire a silently-broken invocation leaves, and it is fatal here.
fn run_replay_block(
    harness: &Harness,
    dir: &Path,
    framed: &Path,
    command: &str,
) -> Result<String, String> {
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
    // does, and for the same reason: the case's authored `probe-results.txt` is the EXPECTATION
    // the record gates compare against, never the bytes dorc is fed. Since the `plans/28G` W3 fold
    // a `dorc why` must NAME its record source (an unnamed one reads the stored receipt instead),
    // so the flag form is now the spelling these transcripts carry.
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
            let mut child = harness.dorc();
            child
                .current_dir(dir)
                .args(rest)
                .stderr(Stdio::null())
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
                return Err(format!("exited rc={}", out.code));
            }
            let got = strip_trailing_newlines(&strip_cr(&out.stdout));
            Ok(if got.is_empty() {
                String::new()
            } else {
                format!("{got}\n")
            })
        }
        _ => Err(String::from("not a `dorc` invocation")),
    }
}

/// Fold the freshly-blessed `expected.out` / `expected.ran` back into the committed `.loom`.
/// `extra` carries blocks 1..N's captured outputs; empty means either a single-block case or a
/// failed drive, and `set_replay_outputs` then leaves those blocks' committed bytes untouched
/// rather than overwriting them with broken output.
fn bless_loom(spec: &LoomCaseSpec, dir: &Path, extra: &[String]) -> Result<(), Failed> {
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
        args.push("-o".to_owned());
        args.push(oracle.display().to_string());
    }
    if let Some(flags) = marker(dir, "DORC_FLAGS")? {
        args.push(flags);
    }
    Ok(args)
}

/// Drive one round-trip case through every gate, then apply the XFAIL/BLESS lens.
fn run_round_trip(harness: &Harness, case: &E2eCase) -> Result<(), Failed> {
    let dir = &case.dir;
    let name = &case.name;
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

    let book = dir.join("book.sh");
    let out = capture(
        harness
            .dorc()
            .arg(format!("--shim-dir={}", shim_dir.display()))
            .arg(format!("--book={}", book.display()))
            .args(&args)
            .stdin(Stdio::from(std::fs::File::open(&framed_path).unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
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
    if run.failures.is_empty() && mocks.is_dir() {
        exec_check(harness, name, dir, &mocks, &apply_art, &mut run.failures);
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
                    &mut run.failures,
                );
            }
        }
    }

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

    if xfail_active && head_ran_drifted(harness, dir, &mocks, &apply_art) {
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

/// A compact first-divergence window (the sh harness printed `diff -u`).
fn divergence(want: &str, got: &str) -> String {
    let want_lines: Vec<&str> = want.lines().collect();
    let got_lines: Vec<&str> = got.lines().collect();
    let at = (0..want_lines.len().max(got_lines.len()))
        .find(|i| want_lines.get(*i) != got_lines.get(*i))
        .unwrap_or(0);
    let from = at.saturating_sub(3);
    let to = (at + 4).min(want_lines.len().max(got_lines.len()));
    let mut out = format!(
        "      first divergence at line {} (want {} lines, got {} lines)\n",
        at + 1,
        want_lines.len(),
        got_lines.len()
    );
    for i in from..to {
        let _ = writeln!(
            out,
            "      -{}\n      +{}",
            want_lines.get(i).copied().unwrap_or("<eof>"),
            got_lines.get(i).copied().unwrap_or("<eof>")
        );
    }
    out.trim_end().to_owned()
}

// ---------------------------------------------------------------------------
// the per-case gates

/// The ap-2 EXECUTABLE acceptance: run the rendered apply under the inert shims and
/// assert the exact set of commands that ran, plus the declared exit rc.
fn exec_check(
    harness: &Harness,
    name: &str,
    dir: &Path,
    mocks: &Path,
    artifact: &str,
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
    let sandbox = scratch.path.join("sand");
    std::fs::create_dir_all(&sandbox).expect("create sandbox");
    let payload = scratch.path.join("apply.sh");
    std::fs::write(&payload, format!("{artifact}\n")).expect("write apply");
    let out = capture(
        harness
            .rail(&sandbox, &log, mocks.as_os_str(), None)
            .stdin(Stdio::from(std::fs::File::open(&payload).unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    );
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
    failures: &mut Vec<String>,
) {
    let debug = debug_argv(harness, dir, args, framed);
    let engine: Vec<&str> = debug
        .lines()
        .filter(|line| line.starts_with("argv "))
        .collect();
    let logged = harness.capture_run(Payload::File(&dir.join("book.sh")), mocks);
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
            .dorc()
            .arg("--debug-argv")
            .arg(format!("--book={}", dir.join("book.sh").display()))
            .args(args)
            .stdin(Stdio::from(std::fs::File::open(framed).unwrap()))
            .stdout(Stdio::null())
            .stderr(Stdio::piped()),
    )
    .stderr
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
    failures: &mut Vec<String>,
) {
    let debug = debug_argv(harness, dir, args, framed);
    let disp = debug
        .lines()
        .filter(|line| line.starts_with("argv "))
        .collect::<Vec<_>>()
        .join("\n");
    let guard_cmds = debug
        .lines()
        .filter_map(|line| line.strip_prefix("guardcmd "))
        .collect::<Vec<_>>()
        .join("\n");
    let bare = harness.capture_run(Payload::File(&dir.join("book.sh")), mocks);
    let apply_out = capture(
        harness
            .dorc()
            .arg(format!("--book={}", dir.join("book.sh").display()))
            .args(args)
            .stdin(Stdio::from(std::fs::File::open(framed).unwrap()))
            .stdout(Stdio::piped())
            .stderr(Stdio::null()),
    )
    .stdout;
    let (_, apply_art) = split_artifacts(&apply_out);
    let apply = harness.capture_run(Payload::Text(&apply_art), mocks);
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
/// (`288:prop-structural-needles-only`): the needle `error[<slug>]` is DERIVED, and every slug
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
    let unfired: Vec<String> = slugs
        .iter()
        .filter(|slug| {
            !errors
                .iter()
                .any(|line| line.contains(&format!("error[{slug}]")))
        })
        .map(|slug| format!("declared but never emitted: error[{slug}]"))
        .collect();
    if !unfired.is_empty() {
        failures.push(format!(
            "FAIL  {name}  [gate-3: a declared error-severity diagnostic did not fire — the declaration is an assertion, not a mute]\n{}",
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
    // `--results` rather than a stdin redirect: since the `plans/28G` W3 fold, naming a record
    // source is what tells `dorc why` to answer from records instead of from the stored receipt.
    let live = capture(
        harness
            .dorc()
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
            .dorc()
            .arg(&book)
            .args(args)
            .arg(format!("--whylog-dir={}", whylog.display()))
            .stdin(Stdio::from(std::fs::File::open(framed).unwrap()))
            .stdout(Stdio::null())
            .stderr(Stdio::null()),
    );
    let replay = capture(
        harness
            .dorc()
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
fn head_ran_drifted(harness: &Harness, dir: &Path, mocks: &Path, apply: &str) -> bool {
    let pin = dir.join("head-expected.ran");
    if !pin.is_file() || !mocks.is_dir() {
        return false;
    }
    let tolerated = tolerances(dir).unwrap_or_default();
    let got = canonicalize(
        &harness.capture_run(Payload::Text(apply), mocks),
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

/// The `DORC_FLAGS` plumbing confound: run the flagship with and without
/// `--trust-footprints` and assert the elision count DIFFERS. If it matches, the flag is
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
                .dorc()
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
    let flagged = elide(&[
        "-o".to_owned(),
        oracle.clone(),
        "--trust-footprints".to_owned(),
    ]);
    let plain = elide(&["-o".to_owned(), oracle]);
    (flagged == plain).then(|| format!(
        "dorc_flags_selftest FAILED — --trust-footprints did not change the flagship's elision count ({flagged} flagged vs {plain} plain); the flag is not reaching the engine, so a flagged survival case's gate-6 attribution would lie."
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
    let sh_dir = which("sh")
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("/bin"));
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

// ---------------------------------------------------------------------------

fn main() {
    let mut args = Arguments::from_args();
    if args.format.is_none() && std::env::var("DORC_E2E_QUIET").as_deref() == Ok("1") {
        args.format = Some(libtest_mimic::FormatSetting::Terse);
    }
    let harness = Arc::new(Harness::resolve());
    let discovered = discover_e2e(&case_roots());
    let looms = discover_looms(&case_roots());
    preflight(&harness, discovered.len());

    let mut trials: Vec<Trial> = Vec::new();
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
                run_round_trip(&harness, &case)
            })),
            E2eKind::Lint => trials.push(Trial::test(case.name.clone(), move || {
                run_lint(&harness, &case)
            })),
            E2eKind::LintReal => {
                real_fixtures.insert(case.name.clone(), case.dir);
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

    libtest_mimic::run(&args, trials).exit();
}
