//! The Alloy runner, reached exactly as an author reaches it: a nested `mise run alloy`, so its
//! preflight, its heavy-work lock, and its caps all apply, and assay adds nothing an author
//! could not type themselves (`notes/30Y` § 3).

use std::path::Path;
use std::process::{Command, Stdio};

use crate::json::{read_num, read_str};

/// What the runner printed for one command, or for one file under `--parse-only`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Row {
    pub(super) module: String,
    pub(super) command: Option<String>,
    pub(super) kind: Option<String>,
    pub(super) scope: Option<String>,
    pub(super) result: String,
    pub(super) wall_ms: Option<u64>,
    pub(super) solve_ms: Option<u64>,
    pub(super) message: Option<String>,
}

/// `mise run alloy -- <args> <dir>`, its rows, or why it did not run at all. Exit 1 is the
/// runner's own verdict on a red command and still carries rows; any other nonzero exit (a
/// refused heavy-work lock, a usage error, a missing JDK) is the runner failing to run.
pub(super) fn invoke(args: &[String], dir: &Path) -> Result<Vec<Row>, String> {
    let out = Command::new("mise")
        .args(["run", "alloy", "--"])
        .args(args)
        .arg(dir)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("mise run alloy: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    match out.status.code() {
        Some(0 | 1) => {
            let rows = rows(&stdout);
            if rows.is_empty() {
                Err("mise run alloy printed no rows".to_owned())
            } else {
                Ok(rows)
            }
        }
        code => {
            eprint!("{stdout}");
            Err(format!(
                "mise run alloy exited {}",
                code.map_or_else(|| "on a signal".to_owned(), |c| c.to_string())
            ))
        }
    }
}

/// The runner writes one JSON object per line; everything else it prints (the preflight line,
/// the array brackets) is not a row.
pub(super) fn rows(stdout: &str) -> Vec<Row> {
    stdout
        .lines()
        .filter_map(|line| {
            let row = line.trim_start_matches([' ', ',']).trim();
            let row = row.strip_prefix('{')?;
            Some(Row {
                module: read_str(row, "module")?,
                command: read_str(row, "command"),
                kind: read_str(row, "kind"),
                scope: read_str(row, "scope"),
                result: read_str(row, "result")?,
                wall_ms: read_num(row, "wall_ms"),
                solve_ms: read_num(row, "solve_ms"),
                message: read_str(row, "message"),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::rows;

    #[test]
    fn runner_rows_are_read_from_its_stdout_and_nothing_else_is() {
        // The nested task's stdout also carries preflight's line; a row reader that tripped on it
        // would turn every lock recomputation into a runner failure.
        let out = "preflight alloy: ok\n[\n  {\"module\": \"laws\", \"command\": \"k\", \"kind\": \"check\", \"scope\": \"3\", \"result\": \"timeout\", \"wall_ms\": 60012, \"solve_ms\": null, \"message\": \"exceeded 60s wall-clock; translated in 41ms: 9 clauses\"}\n, {\"module\": \"x.als\", \"command\": null, \"kind\": null, \"scope\": null, \"result\": \"error\", \"wall_ms\": null, \"solve_ms\": null}\n]\n";
        let got = rows(out);
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].result, "timeout");
        assert_eq!(got[0].wall_ms, Some(60012));
        assert_eq!(got[1].command, None);
    }
}
