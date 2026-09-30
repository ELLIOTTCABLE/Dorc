//! Where a solving pass's JSON report goes, and the plain summary stdout carries in its place
//! (`notes/30Y` § 2.7): an official pass runs for hours, so its report is a file by default rather
//! than terminal scrollback someone has to save by hand.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{FormatTime, SystemTime};

use crate::alloy_jvm::adapter::human;
use crate::json::Value;

/// `--json`'s destination; a directory gets one timestamped file per document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Sink {
    Stdout,
    Dir(PathBuf),
    File(PathBuf),
}

impl Sink {
    /// `--json <to>`: `-` is stdout; with several documents one path can only be a directory.
    pub(super) fn named(to: &str, documents: usize) -> Self {
        match to {
            "-" => Self::Stdout,
            _ if documents > 1 => Self::Dir(super::resolve(to)),
            _ => Self::File(super::resolve(to)),
        }
    }
}

pub(super) fn default_dir() -> PathBuf {
    dorc_testbed::repo_root().join(".tmp").join("assay")
}

/// `2026-09-29T12:34:56.123456Z` rendered as `2026-09-29T123456Z`: Windows refuses `:` in names.
fn utc_stamp() -> String {
    let mut rfc3339 = String::new();
    let _ = SystemTime.format_time(&mut Writer::new(&mut rfc3339));
    let whole = rfc3339.split(['.', 'Z']).next().unwrap_or_default();
    format!("{}Z", whole.replace(':', ""))
}

fn in_dir(dir: &Path, stem: &str) -> PathBuf {
    dir.join(format!("{stem}-{}.json", utc_stamp()))
}

/// What the summary knows that the report does not carry.
#[derive(Debug)]
pub(super) struct Context<'a> {
    pub(super) stem: &'a str,
    pub(super) tier: &'a str,
    pub(super) wall: Duration,
    /// On `--write`: the rows the new lock changed, `+`, `~`, or `-` first.
    pub(super) changes: &'a [String],
}

pub(super) fn deliver(report: &str, sink: &Sink, ctx: &Context<'_>) {
    let value = Value::parse(report).unwrap_or(Value::Null);
    let path = match sink {
        Sink::Stdout => {
            print!("{report}");
            eprint!("{}", findings(&value, ctx));
            return;
        }
        Sink::File(path) => path.clone(),
        Sink::Dir(dir) => in_dir(dir, ctx.stem),
    };
    let written = path
        .parent()
        .map_or(Ok(()), std::fs::create_dir_all)
        .and_then(|()| std::fs::write(&path, report));
    match written {
        Ok(()) => print!("{}", summary(&value, ctx, &path.display().to_string())),
        Err(e) => {
            // Losing hours of solving to a bad path is worse than a loud stdout.
            eprintln!("assay: writing {}: {e}", path.display());
            print!("{report}");
        }
    }
}

fn counts<'v>(values: impl Iterator<Item = &'v str>) -> String {
    let mut by: BTreeMap<&str, usize> = BTreeMap::new();
    for v in values {
        let n = by.entry(v).or_default();
        *n = n.saturating_add(1);
    }
    by.into_iter()
        .map(|(k, n)| format!("{k} {n}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn listed(items: &[Value]) -> String {
    items
        .iter()
        .filter_map(|v| match v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The lines a `--quiet` never silences: the official tier's construction finding and the rows a
/// `--write` changed.
fn findings(report: &Value, ctx: &Context<'_>) -> String {
    let mut out = String::new();
    let lock = report.get("lock");
    let disagreeing = lock.map_or(&[][..], |l| l.arr("construction_disagreements"));
    if !disagreeing.is_empty() {
        let _ = writeln!(out, "construction disagreements: {}", listed(disagreeing));
    }
    if !ctx.changes.is_empty() {
        let path = lock.and_then(|l| l.str("path")).unwrap_or("the lock");
        let _ = writeln!(out, "{path} changed:");
        for change in ctx.changes {
            let _ = writeln!(out, "  {change}");
        }
    }
    out
}

pub(super) fn summary(report: &Value, ctx: &Context<'_>, report_path: &str) -> String {
    let mut out = String::new();
    let commands = report.arr("commands");
    let wall = human(ctx.wall);
    let _ = if commands.is_empty() {
        writeln!(out, "{}: {} tier, no rows, {wall}", ctx.stem, ctx.tier)
    } else {
        writeln!(
            out,
            "{}: {} tier, {} rows in {wall}",
            ctx.stem,
            ctx.tier,
            commands.len()
        )
    };
    if let Some(commit) = report.str("commit") {
        let _ = writeln!(out, "commit: {commit}");
    }
    if !commands.is_empty() {
        let field = |key: &'static str| commands.iter().filter_map(move |c| c.str(key));
        let _ = writeln!(out, "results: {}", counts(field("result")));
        let _ = writeln!(out, "standings: {}", counts(field("standing")));
    }
    if let Some(why) = report.str("runner") {
        let _ = writeln!(out, "runner: {why}");
    }
    let lints: Vec<String> = match report.get("lints") {
        Some(Value::Obj(by_lint)) => by_lint
            .iter()
            .filter_map(|(name, hits)| match hits {
                Value::Arr(hits) if !hits.is_empty() => Some(format!("{name} {}", hits.len())),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    };
    if !lints.is_empty() {
        let _ = writeln!(out, "lints: {}", lints.join(", "));
    }
    if let Some(lock) = report.get("lock") {
        if let Some(status) = lock.str("status") {
            let _ = writeln!(out, "lock: {status}");
        }
        let gone = lock.arr("gone");
        if !gone.is_empty() {
            let _ = writeln!(out, "gone: {}", listed(gone));
        }
    }
    out.push_str(&findings(report, ctx));
    let _ = writeln!(out, "report: {report_path}");
    out
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Context, Value, default_dir, in_dir, summary};

    #[test]
    fn the_summary_carries_the_counts_the_changed_rows_and_the_report_path_last() {
        // The summary is what a person reads after an hours-long pass instead of the report, so
        // every fact it names must come out of the report it points at, and the pointer must be
        // the line a terminal leaves at the bottom.
        let report = Value::parse(
            r#"{"spec": "specs/w.assay.md", "commit": "0123abcd",
                "commands": [
                  {"module": "laws", "name": "a", "result": "no-counterexample", "standing": "green"},
                  {"module": "laws", "name": "b", "result": "no-counterexample", "standing": "mismatch-new"},
                  {"module": "laws", "name": "c", "result": "timeout", "standing": "unmeasured"}
                ],
                "lock": {"path": "specs/w.lock.json", "tier": "hot", "status": "written",
                         "gone": ["laws.d"], "construction_disagreements": []}}"#,
        )
        .expect("the fixture should parse");
        let changes = [
            "+ laws.b no-counterexample".to_owned(),
            "- laws.d sat".to_owned(),
        ];
        let ctx = Context {
            stem: "w",
            tier: "hot",
            wall: Duration::from_secs(75),
            changes: &changes,
        };
        let text = summary(&report, &ctx, "/r/.tmp/assay/w-x.json");
        let lines: Vec<&str> = text.lines().collect();
        assert!(
            lines.first().is_some_and(|l| l.starts_with("w:")
                && l.contains("3 rows")
                && l.contains("1m15s")),
            "{text}"
        );
        assert!(
            text.contains("no-counterexample 2") && text.contains("timeout 1"),
            "{text}"
        );
        assert!(
            text.contains("mismatch-new 1") && text.contains("written"),
            "{text}"
        );
        assert!(text.contains("gone: laws.d"), "{text}");
        assert!(text.contains("0123abcd"), "{text}");
        let under_header: Vec<&str> = lines
            .iter()
            .skip_while(|l| !l.contains("specs/w.lock.json"))
            .skip(1)
            .take(2)
            .copied()
            .collect();
        assert_eq!(
            under_header,
            ["  + laws.b no-counterexample", "  - laws.d sat"],
            "{text}"
        );
        assert_eq!(text.matches("specs/w.lock.json").count(), 1, "{text}");
        assert_eq!(
            lines.last(),
            Some(&"report: /r/.tmp/assay/w-x.json"),
            "{text}"
        );
    }

    #[test]
    fn the_default_report_lands_under_the_repository_tmp_with_the_stem_in_its_name() {
        let path = in_dir(&default_dir(), "widgets");
        assert!(
            path.starts_with(dorc_testbed::repo_root().join(".tmp").join("assay")),
            "{}",
            path.display()
        );
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        assert!(
            name.starts_with("widgets-") && name.ends_with("Z.json"),
            "{name}"
        );
        assert!(!name.contains(':'), "{name}");
    }
}
