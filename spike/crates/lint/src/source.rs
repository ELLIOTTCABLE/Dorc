//! The dumb source registry (`27R` §1 dir-registry-stays-dumb): a thin [`LintSource`] trait + a
//! static `Vec<Box<dyn LintSource>>`. No dynamic discovery, no config, no plugin loading —
//! "pluggable" means "adding source N+1 is a small, local diff", nothing more
//! (`tension-registry-abstraction-cost`, directive: dumb registry, the trait boundary is the only
//! reserve).

use crate::finding::{Finding, SourceStatus};
use crate::runner::ExternalToolRunner;

/// One file handed to lint: its ORIGINAL path + its bytes. (`27R` §8b rung-file: sources that need
/// no book/world work on ANY file handed to them.)
#[derive(Debug, Clone)]
pub struct LintInput {
    /// The user's original path (the one findings name — `27R` §4 dir-paths-stay-yours).
    pub path: String,
    /// The file's source bytes.
    pub src: String,
}

/// Run-wide options a source may consult (`27R` §5). Deliberately small: the `--fail-on`/`--format`
/// knobs are exit/render concerns owned by the cli edge, not by a source's finding production.
#[derive(Debug, Clone, Copy)]
pub struct LintOptions {
    /// Whether external tools may run (`--no-tools` sets this false ⇒ external sources report `Off`).
    pub tools_enabled: bool,
}

impl Default for LintOptions {
    fn default() -> Self {
        Self {
            tools_enabled: true,
        }
    }
}

/// Everything a source reads (`27R` §8b ladder). The files to lint, the loaded oracle sources (for
/// the oracle-body rung-file lints and any lift), the options, and the injected runner. NO interner
/// is shared: each source that needs one mints its own local [`dorc_core::Interner`] (the sources are
/// independent; nothing keys symbols across them), which keeps this context immutable and the
/// crate a pure function of its inputs given the runner.
pub struct LintContext<'a> {
    /// The lintable files (rung-file/rung-book operate over these).
    pub files: &'a [LintInput],
    /// The loaded oracle sources (rung-file item-3 verdict-body lints; a future lift-with-oracles).
    pub oracles: &'a [LintInput],
    /// Run options.
    pub options: LintOptions,
    /// The injected external-tool runner (`dir-runner-is-the-di-seam`).
    pub runner: &'a dyn ExternalToolRunner,
}

impl std::fmt::Debug for LintContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LintContext")
            .field("files", &self.files)
            .field("oracles", &self.oracles)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

/// A pluggable lint source (`27R` §1 dir-registry-stays-dumb). The trait boundary IS the whole
/// pluggability story: implement it, add one line to [`registry`]. Advisory-only — a source pushes
/// [`Finding`]s and returns its [`SourceStatus`]; it never mints a claim/license (`dir-no-license-
/// plane-contact`).
pub trait LintSource {
    /// The source's stable name (`27R` §8 delta-named-sources-selectable): the `--list-sources` entry
    /// and the `Finding::source` tag. Append-only, never re-read to mean something else.
    fn name(&self) -> &'static str;

    /// One-line description for `--list-sources`.
    fn describe(&self) -> &'static str;

    /// Which input-availability rung this source needs (`27R` §8b nit-functionality-ladder). Purely
    /// informational here (shown by `--list-sources`); every registered source is a rung-file or
    /// rung-book source that runs with no probe/world.
    fn rung(&self) -> Rung;

    /// Run the source over `ctx`, pushing findings into `out`; return whether it actually ran
    /// (`27R` §8b dir-envelope-carries-coverage). An external source returns `Absent`/`Off` per the
    /// `27R` §4 ladder; a dorc-native source always returns `Ran`.
    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus;
}

/// The input-availability rung a source sits on (`27R` §8b nit-functionality-ladder). Only the two
/// no-world rungs are buildable this round; `rung-probe`/`rung-oracle-solo` are named-not-built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rung {
    /// Per-file, no book and no world (external tools, parse-tier diagnostics, oracle-body lints).
    File,
    /// Book(s) present, connection DENIED — the no-world pipeline prefix (analysis diagnostics,
    /// unmodeled-wall inventory).
    Book,
}

impl Rung {
    /// A short label for `--list-sources`.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Rung::File => "file",
            Rung::Book => "book",
        }
    }
}

/// The static source registry (`27R` §1 dir-registry-stays-dumb) — a plain `Vec`, built fresh each
/// call, in a fixed, deterministic order. Adding a source is one line here.
#[must_use]
pub fn registry() -> Vec<Box<dyn LintSource>> {
    vec![
        Box::new(crate::source_analysis::AnalysisDiagnostics),
        Box::new(crate::source_unmodeled::UnmodeledInventory),
        Box::new(crate::source_verdict::VerdictBodyFlattening),
        Box::new(crate::source_external::Shellcheck),
        Box::new(crate::source_external::Checkbashisms),
    ]
}
