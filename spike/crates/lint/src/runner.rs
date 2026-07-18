//! The ONE external-tool DI seam (`27R` §1 dir-runner-is-the-di-seam). External-tool invocation
//! (shellcheck, checkbashisms) is non-hermetic, so it goes behind this single injected trait: the
//! `lint` crate stays a pure function of its inputs (`inv-determinism` posture, same shape as
//! hostsim's seam), and the REAL subprocess impl lives at the cli edge only (io-at-edges-only).
//! Unit tests inject a fake that feeds RAW stdout/stderr bytes (`anti-masking-tests`: never a
//! pre-parsed finding).

/// The outcome of running one external tool: its raw process bytes. The adapters
/// (`crate::source_external`) NEVER interpret [`rc`](Self::rc) beyond zero/nonzero (`27R` §8
/// delta-exit-trichotomy-sharpened — checkbashisms' additive exit codes are the named trap);
/// PARSED findings govern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRun {
    /// The tool's process exit code. Read only for zero/nonzero, never decoded further.
    pub rc: i32,
    /// The tool's raw stdout bytes (where machine formats and most linter output land).
    pub stdout: Vec<u8>,
    /// The tool's raw stderr bytes (kept for the raw-passthrough tier / operational diagnostics).
    pub stderr: Vec<u8>,
}

/// The injected external-tool runner (`27R` §1 dir-runner-is-the-di-seam). Two operations: probe
/// availability, and run bytes-in → bytes-out. Deterministic callers depend ONLY on this trait; the
/// nondeterminism (real `std::process`) is confined to the cli-edge impl.
pub trait ExternalToolRunner {
    /// Is `tool` invocable (on PATH)? (`27R` §4 dir-absent-is-info: absence ⇒ one info finding per
    /// run, or a `--require-tools` hard-fail — decided at the cli edge, not here.)
    fn available(&self, tool: &str) -> bool;

    /// Run `tool args…` with `stdin` piped in, returning its raw bytes. The lint sources feed the
    /// STRIPPED source on stdin (the tool sees `-`/stdin, never a temp path — `27R` §4
    /// dir-paths-stay-yours), so nothing about the caller's real path reaches the tool.
    fn run(&self, tool: &str, args: &[&str], stdin: &[u8]) -> ToolRun;
}

/// A runner that reports every tool absent and runs nothing — the `--no-tools` / airgap stand-in and
/// a safe default. (`--no-tools` is actually handled one layer up by disabling the external sources,
/// so this is mostly a convenience for callers/tests that want "no external world" without a real
/// subprocess impl.)
#[derive(Debug, Default, Clone, Copy)]
pub struct NoToolsRunner;

impl ExternalToolRunner for NoToolsRunner {
    fn available(&self, _tool: &str) -> bool {
        false
    }

    fn run(&self, _tool: &str, _args: &[&str], _stdin: &[u8]) -> ToolRun {
        ToolRun {
            rc: 0,
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }
}
