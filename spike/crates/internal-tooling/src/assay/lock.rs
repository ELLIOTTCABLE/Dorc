//! The lock (`notes/30Y` § 2.7): one row per command the runner ran, compared in both directions.
//! Timing and translation size ride the report beside a row and never enter the lock, so a
//! slower machine can never move it.

use std::collections::{BTreeMap, BTreeSet};

use super::MODULE_SUFFIX;
use super::alloy::{self, Head};
use super::runner::Row as RunnerRow;
use crate::json::{Json, read_str};

/// One lock row. `premise` is the `<check>_premise` twin's result on a check, else its book's
/// run's result for a book line, else `absent` (`30Y` § 2.5 item 3); `None` on a run.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct LockRow {
    pub(super) module: String,
    pub(super) name: String,
    pub(super) kind: String,
    pub(super) scope: String,
    pub(super) result: String,
    pub(super) premise: Option<String>,
    pub(super) hash: String,
}

impl LockRow {
    pub(super) fn json(&self) -> Json {
        let mut fields = vec![
            ("module".to_owned(), Json::str(&self.module)),
            ("name".to_owned(), Json::str(&self.name)),
            ("kind".to_owned(), Json::str(&self.kind)),
            ("scope".to_owned(), Json::str(&self.scope)),
            ("result".to_owned(), Json::str(&self.result)),
        ];
        if let Some(premise) = &self.premise {
            fields.push(("premise".to_owned(), Json::str(premise)));
        }
        fields.push(("hash".to_owned(), Json::str(&self.hash)));
        Json::Obj(fields)
    }

    fn read(line: &str) -> Option<Self> {
        Some(Self {
            module: read_str(line, "module")?,
            name: read_str(line, "name")?,
            kind: read_str(line, "kind")?,
            scope: read_str(line, "scope")?,
            result: read_str(line, "result")?,
            premise: read_str(line, "premise"),
            hash: read_str(line, "hash")?,
        })
    }

    pub(super) fn is_red(&self) -> bool {
        !matches!(self.result.as_str(), "sat" | "no-counterexample")
    }
}

/// Lock rows from the runner's rows and the modules they ran, in the runner's order.
pub(super) fn rows(ran: &[RunnerRow], modules: &[(String, String)]) -> Vec<LockRow> {
    let result_of = |module: &str, name: &str| {
        ran.iter()
            .find(|r| r.module == module && r.command.as_deref() == Some(name))
            .map(|r| r.result.clone())
    };
    let mut ordinal: BTreeMap<&str, usize> = BTreeMap::new();
    ran.iter()
        .filter_map(|r| {
            let name = r.command.clone()?;
            let slot = ordinal.entry(r.module.as_str()).or_insert(0);
            let nth = *slot;
            *slot = slot.saturating_add(1);
            let text = modules
                .iter()
                .find(|(file, _)| file.strip_suffix(MODULE_SUFFIX) == Some(r.module.as_str()))
                .map_or("", |(_, text)| text.as_str());
            let kind = r.kind.clone().unwrap_or_default();
            let book_run = r
                .module
                .strip_prefix("book_")
                .map(|book| result_of(&r.module, book).or_else(|| result_of(&r.module, &r.module)));
            let premise = (kind == "check").then(|| {
                result_of(&r.module, &format!("{name}_premise"))
                    .or_else(|| book_run.flatten())
                    .unwrap_or_else(|| "absent".to_owned())
            });
            Some(LockRow {
                module: r.module.clone(),
                hash: digest(command_text(text, &name, nth).as_bytes()),
                name,
                kind,
                scope: r.scope.clone().unwrap_or_default(),
                result: r.result.clone(),
                premise,
            })
        })
        .collect()
}

/// A command's source in the module that ran it: by name, else by position, since Alloy labels
/// an unnamed command itself.
fn command_text(module: &str, name: &str, nth: usize) -> String {
    let commands: Vec<(Option<String>, String)> = alloy::items(module, 1)
        .into_iter()
        .filter_map(|item| match item.head() {
            Head::Command { name, .. } => Some((name, item.text)),
            Head::Sig { .. } | Head::Other => None,
        })
        .collect();
    commands
        .iter()
        .find(|(n, _)| n.as_deref() == Some(name))
        .or_else(|| commands.get(nth))
        .map(|(_, text)| text.clone())
        .unwrap_or_default()
}

/// FNV-1a, 64-bit, over LF-normalized bytes: a drift alarm on the command's text, never an
/// identity (`spike/verify/CLAUDE.md` `rul-derivation-digest-is-an-alarm-not-trust`).
fn digest(bytes: &[u8]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for byte in bytes.iter().filter(|b| **b != b'\r') {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

/// The lock file: a JSON array, one row per line, so its git diff is a row diff.
pub(super) fn render(rows: &[LockRow]) -> String {
    let lines: Vec<String> = rows
        .iter()
        .map(|r| format!("  {}", r.json().line()))
        .collect();
    format!("[\n{}\n]\n", lines.join(",\n"))
}

/// Rows of a lock file this module wrote; `None` when a row line does not read back.
pub(super) fn parse(text: &str) -> Option<Vec<LockRow>> {
    text.lines()
        .map(|l| l.trim().trim_end_matches(','))
        .filter(|l| l.starts_with('{'))
        .map(LockRow::read)
        .collect()
}

/// The red rows the committed lock records exactly as the run produced them: the residue a
/// passing `--check` carries, acked by whoever committed the lock.
pub(super) fn accepted_reds(committed: &[LockRow], computed: &[LockRow]) -> Vec<LockRow> {
    computed
        .iter()
        .filter(|r| r.is_red() && committed.contains(r))
        .cloned()
        .collect()
}

/// Rows the run produced that the lock lacks, and rows the lock holds that the run did not.
pub(super) fn compare(committed: &[LockRow], computed: &[LockRow]) -> (Vec<LockRow>, Vec<LockRow>) {
    let committed_set: BTreeSet<&LockRow> = committed.iter().collect();
    let computed_set: BTreeSet<&LockRow> = computed.iter().collect();
    (
        computed
            .iter()
            .filter(|r| !committed_set.contains(r))
            .cloned()
            .collect(),
        committed
            .iter()
            .filter(|r| !computed_set.contains(r))
            .cloned()
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::{LockRow, RunnerRow, accepted_reds, compare, parse, render, rows};

    fn ran(module: &str, command: &str, kind: &str, result: &str) -> RunnerRow {
        RunnerRow {
            module: module.to_owned(),
            command: Some(command.to_owned()),
            kind: Some(kind.to_owned()),
            scope: Some("3".to_owned()),
            result: result.to_owned(),
            wall_ms: Some(1234),
            ..RunnerRow::default()
        }
    }

    const LAWS: &str =
        "module laws\ncheck k { a } for 3\nrun k_premise { b } for 3\ncheck lone_law { c } for 3\n";

    #[test]
    fn a_check_carries_its_twin_and_a_lone_check_says_absent() {
        // `30Y` § 2.5: a check is only as good as its witnessed premise, so the twin's result
        // sits on the check's own row, and a check without one is marked rather than left blank.
        let got = rows(
            &[
                ran("laws", "k", "check", "no-counterexample"),
                ran("laws", "k_premise", "run", "unsat"),
                ran("laws", "lone_law", "check", "no-counterexample"),
            ],
            &[("laws.als".to_owned(), LAWS.to_owned())],
        );
        let premises: Vec<Option<&str>> = got.iter().map(|r| r.premise.as_deref()).collect();
        assert_eq!(premises, vec![Some("unsat"), None, Some("absent")]);
        assert!(got.iter().all(|r| r.hash.len() == 16));
        assert_ne!(got[0].hash, got[2].hash, "each row hashes its own command");
        assert!(got[1].is_red(), "an unsat run is red");
    }

    #[test]
    fn a_book_line_check_is_witnessed_by_its_books_run() {
        // `30Y` § 2.5 item 3: a book line has no twin of its own; the book's run, which asserts
        // every outcome together, is what shows its premise has a world.
        let book = "module book_twirls\ncheck line_2 { a } for 3\nrun twirls { a } for 3\n";
        let corpus = "module book_corpus\ncheck law { b } for 3\nrun book_corpus {} for 3\n";
        let got = rows(
            &[
                ran("book_twirls", "line_2", "check", "no-counterexample"),
                ran("book_twirls", "twirls", "run", "unsat"),
                ran("book_corpus", "law", "check", "no-counterexample"),
                ran("book_corpus", "book_corpus", "run", "sat"),
            ],
            &[
                ("book_twirls.als".to_owned(), book.to_owned()),
                ("book_corpus.als".to_owned(), corpus.to_owned()),
            ],
        );
        let premises: Vec<Option<&str>> = got.iter().map(|r| r.premise.as_deref()).collect();
        assert_eq!(premises, vec![Some("unsat"), None, Some("sat"), None]);
    }

    #[test]
    fn timing_never_reaches_the_lock_and_the_file_round_trips() {
        // A slower machine must never move the lock (`30Y` § 2.7): the same verdicts at another
        // wall-clock are the same rows, byte for byte.
        let module = [("laws.als".to_owned(), LAWS.to_owned())];
        let fast = rows(&[ran("laws", "k", "check", "no-counterexample")], &module);
        let mut slow_row = ran("laws", "k", "check", "no-counterexample");
        slow_row.wall_ms = Some(99_999);
        slow_row.message = Some("translated in 9s: 12 clauses".to_owned());
        let slow = rows(&[slow_row], &module);
        assert_eq!(render(&fast), render(&slow));
        assert_eq!(parse(&render(&fast)), Some(fast));
    }

    #[test]
    fn a_mismatch_is_reported_in_both_directions() {
        // A missing lock row and a stale one are both drift; one direction alone would let a
        // deleted law or a new command slip through.
        let row = |name: &str, result: &str| LockRow {
            module: "laws".to_owned(),
            name: name.to_owned(),
            kind: "check".to_owned(),
            scope: "3".to_owned(),
            result: result.to_owned(),
            premise: Some("sat".to_owned()),
            hash: "0".repeat(16),
        };
        let committed = vec![
            row("kept", "no-counterexample"),
            row("moved", "no-counterexample"),
        ];
        let computed = vec![
            row("kept", "no-counterexample"),
            row("moved", "counterexample"),
            row("new", "no-counterexample"),
        ];
        let (not_in_lock, not_in_run) = compare(&committed, &computed);
        let names = |rs: &[LockRow]| {
            rs.iter()
                .map(|r| (r.name.clone(), r.result.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            names(&not_in_lock),
            vec![
                ("moved".into(), "counterexample".into()),
                ("new".into(), "no-counterexample".into())
            ]
        );
        assert_eq!(
            names(&not_in_run),
            vec![("moved".into(), "no-counterexample".into())]
        );
        assert_eq!(compare(&committed, &committed), (vec![], vec![]));
    }

    #[test]
    fn a_red_the_lock_records_is_accepted_and_a_moved_premise_is_not() {
        // The human's ruling: a set of reds fully acked by the committed lock passes. The ack is
        // row for row, so a red whose premise moved is drift, not the red that was acked.
        let row = |name: &str, result: &str, premise: &str| LockRow {
            module: "laws".to_owned(),
            name: name.to_owned(),
            kind: "check".to_owned(),
            scope: "3".to_owned(),
            result: result.to_owned(),
            premise: Some(premise.to_owned()),
            hash: "0".repeat(16),
        };
        let committed = vec![
            row("green", "no-counterexample", "sat"),
            row("known_red", "counterexample", "sat"),
        ];
        assert_eq!(compare(&committed, &committed), (vec![], vec![]));
        assert_eq!(
            accepted_reds(&committed, &committed),
            vec![row("known_red", "counterexample", "sat")]
        );

        let moved = vec![
            row("green", "no-counterexample", "sat"),
            row("known_red", "counterexample", "unsat"),
        ];
        let (not_in_lock, not_in_run) = compare(&committed, &moved);
        assert_eq!(
            not_in_lock,
            vec![row("known_red", "counterexample", "unsat")]
        );
        assert_eq!(not_in_run, vec![row("known_red", "counterexample", "sat")]);
        assert!(accepted_reds(&committed, &moved).is_empty());
    }
}
