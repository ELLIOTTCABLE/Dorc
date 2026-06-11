//! `dorc-coverage` — the analyzer-coverage dashboard binary (round-21 arch-6).
//!
//! Reads a book + oracle files (+ optional probe-results), drives the engine through
//! [`dorc_coverage::build_report`], and prints a human-readable aligned per-site table
//! plus the per-door / dq-2-rung rollups to stdout; optionally writes a stable-column
//! TSV. A measuring INSTRUMENT, never a gate: it never fails a build (see
//! `spike/tools/coverage.sh`).
//!
//! ```text
//! usage: dorc-coverage --book=<book.sh> [-o <oracle.sh>]... [--probe-results=<file>]
//!                      [--tsv=<out.tsv>] [--no-table]
//! ```
//!
//! I/O edge: `inv-determinism` exempts the binary; the analyzer kernel it drives is
//! pure. The table/TSV are a pure function of the inputs (BTree-ordered, no clock/RNG).

#![forbid(unsafe_code)]
// The dashboard binary is an I/O edge (workspace Cargo.toml: "I/O-edge crates may
// `#[expect]` these at the crate root, with reason"): the report + table go to
// stdout, diagnostics to stderr. The kernel it drives stays print-free.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "dorc-coverage is the I/O edge: the coverage table/TSV to stdout, errors to stderr"
)]

use std::fmt::Write as _;
use std::process::ExitCode;

use dorc_coverage::weights::Weights;
use dorc_coverage::{
    Analyzable, BlockReason, DOOR_COLUMNS, Door, Inputs, RUNG_COLUMNS, Report, Rung, SiteRow,
    build_report,
};

const USAGE: &str = "usage: dorc-coverage --book=<book.sh> [-o <oracle.sh>]... \
                     [--probe-results=<file>] [--tsv=<out.tsv>] [--no-table]";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("dorc-coverage: {msg}");
            ExitCode::FAILURE
        }
    }
}

struct Args {
    book: String,
    oracles: Vec<String>,
    probe_results: Option<String>,
    tsv: Option<String>,
    no_table: bool,
}

/// Hand-rolled arg parsing (no `clap` dep, cli parity): `--book=PATH`/`--book PATH`,
/// `-o PATH`/`-oPATH`/`--oracle PATH` (repeatable), `--probe-results=PATH`,
/// `--tsv=PATH`, `--no-table`.
fn parse_args() -> Result<Args, String> {
    let mut book: Option<String> = None;
    let mut oracles = Vec::new();
    let mut probe_results: Option<String> = None;
    let mut tsv: Option<String> = None;
    let mut no_table = false;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        if let Some(p) = arg.strip_prefix("--book=") {
            book = Some(p.to_string());
        } else if arg == "--book" {
            book = Some(it.next().ok_or("--book needs a path")?);
        } else if arg == "-o" || arg == "--oracle" {
            oracles.push(it.next().ok_or("-o needs a path")?);
        } else if let Some(p) = arg.strip_prefix("-o").filter(|p| !p.is_empty()) {
            oracles.push(p.to_string());
        } else if let Some(p) = arg.strip_prefix("--probe-results=") {
            probe_results = Some(p.to_string());
        } else if arg == "--probe-results" {
            probe_results = Some(it.next().ok_or("--probe-results needs a path")?);
        } else if let Some(p) = arg.strip_prefix("--tsv=") {
            tsv = Some(p.to_string());
        } else if arg == "--tsv" {
            tsv = Some(it.next().ok_or("--tsv needs a path")?);
        } else if arg == "--no-table" {
            no_table = true;
        } else if arg == "-h" || arg == "--help" {
            return Err(USAGE.to_string());
        } else {
            return Err(format!("unexpected argument {arg:?}; {USAGE}"));
        }
    }
    Ok(Args {
        book: book.ok_or(USAGE)?,
        oracles,
        probe_results,
        tsv,
        no_table,
    })
}

fn run() -> Result<(), String> {
    let args = parse_args()?;

    let book = std::fs::read_to_string(&args.book)
        .map_err(|e| format!("reading book {}: {e}", args.book))?;
    let oracle_srcs: Vec<String> = args
        .oracles
        .iter()
        .map(|p| std::fs::read_to_string(p).map_err(|e| format!("reading oracle {p}: {e}")))
        .collect::<Result<_, _>>()?;
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();
    let probe_src = match &args.probe_results {
        Some(p) => Some(
            std::fs::read_to_string(p).map_err(|e| format!("reading probe-results {p}: {e}"))?,
        ),
        None => None,
    };

    let weights = Weights::line_count_standin();
    let inputs = Inputs {
        book: &book,
        oracles: &oracle_refs,
        probe_results: probe_src.as_deref(),
        weights: &weights,
    };
    let report = build_report(&inputs);

    // `--no-table` suppresses only the per-site TABLE; the header + rollups (the
    // instrument's headline) always print so the wrapper's "rollup only" mode is useful.
    print!(
        "{}",
        render_report(&report, &args.book, probe_src.is_some(), !args.no_table)
    );
    if let Some(path) = &args.tsv {
        std::fs::write(path, render_tsv(&report))
            .map_err(|e| format!("writing tsv {path}: {e}"))?;
        eprintln!(
            "dorc-coverage: wrote {} site rows to {path}",
            report.total_sites()
        );
    }
    Ok(())
}

// ===========================================================================
// Human-readable aligned table
// ===========================================================================

/// Render the full human-readable report: an optional per-site table (gated by
/// `with_table`), then the count- and criticality-weighted per-door rollup, the
/// north-star split (full-elision vs guard-transform), and the dq-2 rung split. The
/// header + rollups always render (so `--no-table` still shows the headline).
/// Deterministic (site-id ordered).
fn render_report(report: &Report, book_path: &str, has_probe: bool, with_table: bool) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# dorc-coverage — analyzer coverage over {book_path}");
    let _ = writeln!(
        out,
        "# {} sites · probe-results: {}\n",
        report.total_sites(),
        if has_probe {
            "supplied"
        } else {
            "ABSENT (c3 unknown)"
        }
    );

    // 217 §5 obs-3 tier-1: a render-refusal the span-bridge could not attribute is a silently-
    // dropped demotion ⇒ a render-refused leaf re-counted as an elision (the heredoc OVER-count).
    // MUST be 0; surface any miss LOUDLY in the report itself, never let it regrow the lie.
    if report.bridge_suspect > 0 {
        let _ = writeln!(
            out,
            "!! WARNING: {} render-refusal(s) unattributed by the span bridge — elision counts \
             may OVER-count (217 §5 obs-3); the per-site doors for these leaves are SUSPECT.\n",
            report.bridge_suspect,
        );
    }

    if with_table {
        // Per-site rows. Columns: site·line·c1·c2·c3·door·why·wt·rung·cmd.
        let rows: Vec<Vec<String>> = report.rows.iter().map(site_columns).collect();
        let headers = vec![
            "site".into(),
            "line".into(),
            "c1·ok".into(),
            "c2·orc".into(),
            "c3·probe".into(),
            "door".into(),
            "why".into(),
            "wt".into(),
            "rung".into(),
            "command".into(),
        ];
        // The last column (command) is free-text — leave it unpadded.
        out.push_str(&align_table(&headers, &rows, 9));
        out.push('\n');
    }

    out.push_str(&render_door_rollup(report));
    out.push('\n');
    out.push_str(&render_northstar(report));
    out.push('\n');
    out.push_str(&render_rung_rollup(report));
    out
}

/// The ten display columns for one site row.
fn site_columns(r: &SiteRow) -> Vec<String> {
    vec![
        r.site.0.to_string(),
        r.line.to_string(),
        analyzable_word(r.analyzable).into(),
        if r.oracled { "yes".into() } else { "no".into() },
        match r.probed {
            Some(v) => verdict_word(v).to_string(),
            None => "-".into(),
        },
        r.door.label().to_string(),
        why_cell(&r.door),
        r.weight.to_string(),
        rung_short(r.rung).to_string(),
        truncate(&r.command, 52),
    ]
}

/// The "why" cell: the block-reason slug for a `runs` site, the unattributed repr,
/// else empty (the door itself is the explanation).
fn why_cell(door: &Door) -> String {
    match door {
        Door::Runs(reason) => reason.label().to_string(),
        Door::Unattributed(repr) => truncate(repr, 28),
        _ => String::new(),
    }
}

/// A compact rung tag for the per-site table.
fn rung_short(rung: Rung) -> &'static str {
    match rung {
        Rung::GuardReadable => "r2/4",
        Rung::NeedsDeclaration => "r3",
        Rung::NotApplicable => "-",
    }
}

fn verdict_word(v: dorc_core::Verdict) -> &'static str {
    match v {
        dorc_core::Verdict::Converged => "holds",
        dorc_core::Verdict::Diverged => "absent",
        dorc_core::Verdict::Unknown => "?",
    }
}

/// c1 tri-state word: `yes` (fact resolved), `n/a` (`MustRun` — opaque-⊤ or
/// pure/kill, the public surface cannot tell). NEVER asserts a bare `TOP`.
fn analyzable_word(a: Analyzable) -> &'static str {
    match a {
        Analyzable::Yes => "yes",
        Analyzable::Indeterminate => "n/a",
    }
}

/// The count- + criticality-weighted per-door rollup (the `20V` §7 decomposition).
/// Every door column is present (door-4/door-2 show 0 — the stable instrument shape).
fn render_door_rollup(report: &Report) -> String {
    let mut out = String::from("## per-door rollup (count | criticality-weight)\n");
    let rows: Vec<Vec<String>> = DOOR_COLUMNS
        .iter()
        .map(|label| {
            vec![
                (*label).to_string(),
                report
                    .by_door_count
                    .get(*label)
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
                report
                    .by_door_weight
                    .get(*label)
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ]
        })
        .collect();
    let headers = [
        "door".to_string(),
        "count".to_string(),
        "crit-wt".to_string(),
    ];
    out.push_str(&align_table(&headers, &rows, 99));
    out
}

/// The north-star split: FULL elision (run-set shrinks) vs guard-TRANSFORM (door-4),
/// NEVER blurred (`20V` §7). Both count- and criticality-weighted, with the
/// criticality-weighted full-elision fraction (the ~80% question's measurable form).
fn render_northstar(report: &Report) -> String {
    let total_w = report.total_weight();
    let full_w = report.full_elided_weight();
    let trans_w = report.transform_weight();
    let frac = if total_w == 0 {
        0.0
    } else {
        f64::from(full_w) / f64::from(total_w)
    };
    let transform_sites = report
        .rows
        .iter()
        .filter(|r| matches!(r.door, Door::GuardTransform))
        .count();
    let mut out = String::from("## north-star (full-elision vs guard-transform — kept separate)\n");
    let _ = writeln!(
        out,
        "   full-elision     : {:>3} sites · crit-wt {full_w:>4}",
        report.full_elided_count(),
    );
    let _ = writeln!(
        out,
        "   guard-transform  : {transform_sites:>3} sites · crit-wt {trans_w:>4}   \
         (door-4; 0 until it lands)",
    );
    let _ = writeln!(
        out,
        "   total sites      : {:>3} sites · crit-wt {total_w:>4}",
        report.total_sites(),
    );
    let _ = writeln!(
        out,
        "   crit-weighted full-elision coverage : {:.1}%   (north-star target ~80%)",
        frac * 100.0,
    );
    out
}

/// The dq-2 rung-population split (`20V` §6): guard-readable (already paying off from
/// idioms) vs needs-declaration (the door-2/door-4 work would move it) vs n/a.
fn render_rung_rollup(report: &Report) -> String {
    let mut out =
        String::from("## dq-2 rung split (elision reachability: readable-idiom vs declaration)\n");
    let rows: Vec<Vec<String>> = RUNG_COLUMNS
        .iter()
        .map(|label| {
            vec![
                (*label).to_string(),
                report
                    .rung_count
                    .get(label)
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
                report
                    .rung_weight
                    .get(label)
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ]
        })
        .collect();
    let headers = [
        "rung".to_string(),
        "count".to_string(),
        "crit-wt".to_string(),
    ];
    out.push_str(&align_table(&headers, &rows, 99));
    out
}

/// Render an aligned monospace table: headers + rows, each column padded to its widest
/// cell. Columns `< left_col` are left-aligned and right-padded; columns `≥ left_col`
/// (the trailing free-text command) are emitted unpadded. Row width need not match
/// `headers` length (short rows pad with empties), so no fixed-array indexing.
fn align_table(headers: &[String], rows: &[Vec<String>], left_col: usize) -> String {
    let ncols = headers.len();
    let mut widths = vec![0usize; ncols];
    for (i, w) in widths.iter_mut().enumerate() {
        *w = headers.get(i).map_or(0, String::len);
    }
    for row in rows {
        for (i, w) in widths.iter_mut().enumerate() {
            *w = (*w).max(row.get(i).map_or(0, String::len));
        }
    }
    let mut out = String::new();
    push_row(&mut out, headers, &widths, left_col);
    let rule: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    push_row(&mut out, &rule, &widths, left_col);
    for row in rows {
        push_row(&mut out, row, &widths, left_col);
    }
    out
}

/// Push one padded table row (two-space column gaps; columns ≥ `left_col` unpadded).
fn push_row(out: &mut String, cells: &[String], widths: &[usize], left_col: usize) {
    let parts: Vec<String> = cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            if i >= left_col {
                cell.clone()
            } else {
                format!(
                    "{cell:<width$}",
                    width = widths.get(i).copied().unwrap_or(0)
                )
            }
        })
        .collect();
    out.push_str(parts.join("  ").trim_end());
    out.push('\n');
}

/// Truncate a string to `max` chars, appending `…` if cut (keeps the table aligned).
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let head: String = s.chars().take(max.saturating_sub(1)).collect();
    format!("{head}…")
}

// ===========================================================================
// TSV (stable column order, machine-readable)
// ===========================================================================

/// Render the per-site TSV with a STABLE column order (`inv-determinism`): one header
/// row, one row per site in site-id order, then blank-line-separated rollup blocks
/// (per-door, rung). Tabs separate columns; command text is tab/newline-cleaned. The
/// TSV is the durable artifact the gate-set wrapper and any future tooling consume.
fn render_tsv(report: &Report) -> String {
    let mut out =
        String::from("site\tline\tanalyzable\toracled\tprobed\tdoor\twhy\tweight\trung\tcommand\n");
    for r in &report.rows {
        let probed = match r.probed {
            Some(v) => verdict_word(v),
            None => "-",
        };
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{probed}\t{}\t{}\t{}\t{}\t{}",
            r.site.0,
            r.line,
            analyzable_word(r.analyzable),
            r.oracled,
            r.door.label(),
            why_full(&r.door),
            r.weight,
            rung_label_for(r.rung),
            tsv_clean(&r.command),
        );
    }
    let _ = write!(out, "\n# per-door (door\\tcount\\tcrit-weight)\n");
    for label in DOOR_COLUMNS {
        let _ = writeln!(
            out,
            "{label}\t{}\t{}",
            report.by_door_count.get(*label).copied().unwrap_or(0),
            report.by_door_weight.get(*label).copied().unwrap_or(0),
        );
    }
    let _ = write!(out, "\n# dq-2-rung (rung\\tcount\\tcrit-weight)\n");
    for label in RUNG_COLUMNS {
        let _ = writeln!(
            out,
            "{label}\t{}\t{}",
            report.rung_count.get(label).copied().unwrap_or(0),
            report.rung_weight.get(label).copied().unwrap_or(0),
        );
    }
    // 217 §5 obs-3: the span-bridge blind-spot count (MUST be 0) in the machine-readable artifact.
    let _ = write!(out, "\n# bridge-suspect (unattributed-render-refusals)\n");
    let _ = writeln!(out, "bridge-suspect\t{}", report.bridge_suspect);
    out
}

/// The TSV rung slug (mirrors the library's `rung_label`, kept local so the binary
/// owns its column vocabulary).
fn rung_label_for(rung: Rung) -> &'static str {
    match rung {
        Rung::GuardReadable => "guard-readable",
        Rung::NeedsDeclaration => "needs-declaration",
        Rung::NotApplicable => "not-applicable",
    }
}

/// The full (untruncated) "why" for the TSV `why` column.
fn why_full(door: &Door) -> String {
    match door {
        Door::Runs(BlockReason::Unattributed(repr)) | Door::Unattributed(repr) => {
            format!("unattributed:{repr}")
        }
        Door::Runs(reason) => reason.label().to_string(),
        _ => String::new(),
    }
}

/// Replace any stray tab/newline in command text with a space (TSV safety).
fn tsv_clean(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

// ===========================================================================
// Smoke test: the binary's rendering is deterministic + stable-columned.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal book the engine fully models, with a converged install probed — the
    /// table/TSV smoke. Asserts the rollup columns are ALL present (the stable shape)
    /// and the run-set / coverage numbers render.
    fn report_for(book: &str, oracle: &str, probe: Option<&str>) -> Report {
        let weights = Weights::line_count_standin();
        let inputs = Inputs {
            book,
            oracles: &[oracle],
            probe_results: probe,
            weights: &weights,
        };
        build_report(&inputs)
    }

    const PKG_ORACLE: &str = r#"
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge kill installed
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
}
"#;

    #[test]
    fn tsv_has_all_stable_columns_and_is_deterministic() {
        // The TSV must carry every door + rung column in the fixed order, regardless
        // of which doors the book hit, and be byte-identical across two builds.
        let r = report_for(
            "apt-get install -y nginx\n",
            PKG_ORACLE,
            Some("site 0 effect=holds\n"),
        );
        let tsv = render_tsv(&r);
        for col in DOOR_COLUMNS {
            assert!(
                tsv.contains(&format!("\n{col}\t")),
                "door column `{col}` present"
            );
        }
        for col in RUNG_COLUMNS {
            assert!(
                tsv.contains(&format!("\n{col}\t")),
                "rung column `{col}` present"
            );
        }
        let r2 = report_for(
            "apt-get install -y nginx\n",
            PKG_ORACLE,
            Some("site 0 effect=holds\n"),
        );
        assert_eq!(
            tsv,
            render_tsv(&r2),
            "TSV is a deterministic function of inputs"
        );
    }

    #[test]
    fn table_renders_coverage_line() {
        // The human table includes the north-star coverage percentage line.
        let r = report_for(
            "apt-get install -y nginx\n",
            PKG_ORACLE,
            Some("site 0 effect=holds\n"),
        );
        let table = render_report(&r, "book.sh", true, true);
        assert!(
            table.contains("crit-weighted full-elision coverage"),
            "table:\n{table}"
        );
        // A converged lone install elides ⇒ replace-converged ≥ 1.
        assert!(table.contains("replace-converged"), "table:\n{table}");
    }

    #[test]
    fn columns_present_even_with_no_probe_results() {
        // Without probe-results c3 is `-` everywhere and the elidable site runs
        // `unprobed`; the table/TSV shape is unchanged (the instrument is stable).
        let r = report_for("apt-get install -y nginx\n", PKG_ORACLE, None);
        let tsv = render_tsv(&r);
        assert!(
            tsv.contains("\truns\t") || tsv.contains("unprobed"),
            "tsv:\n{tsv}"
        );
    }

    #[test]
    fn bridge_suspect_renders_loud_warning_only_when_nonzero() {
        // 217 §5 obs-3: the span-bridge blind-spot must surface LOUDLY in the binary's output.
        // A clean book renders no warning; a report carrying an unattributed refusal renders the
        // loud line. (The bridge-counting logic itself is unit-pinned in lib; here we pin the
        // user-visible signal — that the count is not computed-then-swallowed.)
        let clean = report_for(
            "apt-get install -y nginx\n",
            PKG_ORACLE,
            Some("site 0 effect=holds\n"),
        );
        assert_eq!(clean.bridge_suspect, 0, "the clean book has no blind spot");
        assert!(
            !render_report(&clean, "book.sh", true, false).contains("WARNING"),
            "no warning on a clean bridge"
        );

        let mut suspect = clean;
        suspect.bridge_suspect = 2;
        let out = render_report(&suspect, "book.sh", true, false);
        assert!(
            out.contains("WARNING") && out.contains("217 §5 obs-3"),
            "a non-zero bridge_suspect renders the loud warning: {out}"
        );
    }
}
