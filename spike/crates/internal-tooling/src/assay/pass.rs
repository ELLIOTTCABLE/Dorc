use std::path::{Path, PathBuf};

use super::drive::{self, Computed, Job, Target};
use super::lock::{self, Row, Standing};
use super::{
    Ask, CONTENTION, DOC_SUFFIX, Finding, Inputs, LAWS_HALF, LOCK_SUFFIX, Lint, Mode, RED, REFUSED,
    RUNNER_FAILED, SHARED_HALF, Source, UNMEASURED, compile, display_path, emit, lints_json,
    write_modules,
};
use crate::alloy_jvm::Jvm;
use crate::alloy_jvm::adapter::{Adapter, Machine, Progress, Refusal, Watch};
use crate::json::Json;

type Fields = Vec<(String, Json)>;

fn emit_report(fields: Fields, code: u8) -> u8 {
    print!("{}", Json::Obj(fields).render());
    code
}

fn staged(path: &Path) -> Option<String> {
    let rel = display_path(path);
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dorc_testbed::repo_root())
        .arg("show")
        .arg(format!(":{rel}"))
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

#[expect(
    clippy::too_many_lines,
    reason = "one document's pass, in the order the modes branch"
)]
pub(super) fn one(spec: &Path, out: Option<PathBuf>, ask: &Ask) -> u8 {
    let read = |path: &Path| {
        if ask.mode == Mode::Staged {
            staged(path)
        } else {
            std::fs::read_to_string(path).ok()
        }
    };
    let Some(doc) = read(spec) else {
        eprintln!("assay: {}: unreadable", spec.display());
        return REFUSED;
    };
    let (shared, laws) = (
        read(&spec.with_file_name(SHARED_HALF)),
        read(&spec.with_file_name(LAWS_HALF)),
    );
    let lossy = |s: Option<&std::ffi::OsStr>| {
        s.map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    };
    let doc_name = lossy(spec.file_name());
    let stem = doc_name
        .strip_suffix(DOC_SUFFIX)
        .map_or_else(|| lossy(spec.file_stem()), str::to_owned);
    let out = out.unwrap_or_else(|| {
        let dir = if ask.mode == Mode::Staged {
            format!("{stem}.staged")
        } else {
            stem.clone()
        };
        internal_tooling::target_dir().join("alloy").join(dir)
    });
    let spec_path = display_path(spec);
    let inputs = Inputs {
        doc: Source {
            name: &doc_name,
            text: &doc,
        },
        shared: shared.as_deref().map(|text| Source {
            name: SHARED_HALF,
            text,
        }),
        laws: laws.as_deref().map(|text| Source {
            name: LAWS_HALF,
            text,
        }),
    };
    let mut fields: Fields = vec![
        ("spec".to_owned(), Json::str(&spec_path)),
        ("out".to_owned(), Json::str(out.display().to_string())),
    ];
    let compiled = match compile(&inputs) {
        Ok(compiled) => compiled,
        Err(findings) => {
            fields.push(("modules".to_owned(), Json::Arr(Vec::new())));
            fields.push(("lints".to_owned(), lints_json(&findings)));
            return emit_report(fields, REFUSED);
        }
    };
    match write_modules(&out, &compiled.modules) {
        Ok(paths) => fields.push((
            "modules".to_owned(),
            Json::Arr(paths.into_iter().map(Json::Str).collect()),
        )),
        Err(e) => {
            eprintln!("assay: writing {}: {e}", out.display());
            return RED;
        }
    }
    fields.extend(
        compiled
            .report
            .iter()
            .map(|(k, v)| ((*k).to_owned(), v.clone())),
    );
    if ask.mode == Mode::Compile {
        return emit_report(fields, 0);
    }
    let jvm = match Jvm::from_env() {
        Ok(jvm) => jvm,
        Err(why) => return emit_report(runner_failed(fields, &why), RUNNER_FAILED),
    };
    let lock_path = spec.with_file_name(format!("{stem}{LOCK_SUFFIX}"));
    let committed_text = if ask.mode == Mode::Staged {
        staged(&lock_path)
    } else {
        std::fs::read_to_string(&lock_path).ok()
    };
    let committed = match committed_text.as_deref().map(lock::parse) {
        None => Vec::new(),
        Some(Some(rows)) => rows,
        Some(None) => {
            fields.push((
                "lock".to_owned(),
                Json::obj([
                    ("path", Json::str(display_path(&lock_path))),
                    ("status", Json::str("unreadable")),
                ]),
            ));
            return emit_report(fields, RED);
        }
    };
    let mut caps = match super::tier::with_overrides(ask.tier.caps(), &ask.caps) {
        Ok(caps) => caps,
        Err(why) => {
            eprintln!("assay: {why}");
            return REFUSED;
        }
    };
    let target = match resolve_target(&ask.target, &compiled.modules) {
        Ok(t) => t,
        Err(why) => {
            eprintln!("assay: {why}");
            return REFUSED;
        }
    };

    if matches!(ask.mode, Mode::Parse | Mode::Staged) {
        caps.machine = Machine {
            heap_mb: caps.machine.heap_mb.min(1024),
            procs: caps.machine.procs,
        };
        let job = Job {
            out: &out,
            modules: &compiled.modules,
            conjunctions: &compiled.conjunctions,
            committed: &committed,
            tier: ask.tier,
            caps,
            target: &target,
            stem: &stem,
            progress: Progress::Silent,
        };
        let mut adapter = match Adapter::new(&jvm, caps.machine, Watch::SILENT) {
            Ok(a) => a,
            Err(why) => return emit_report(runner_failed(fields, &why), RUNNER_FAILED),
        };
        let refusals = if ask.mode == Mode::Parse {
            compiled
                .modules
                .iter()
                .filter_map(|(name, _)| {
                    adapter
                        .parse_only(&out.join(name), &[])
                        .err()
                        .map(|r| (name.clone(), r))
                })
                .collect()
        } else {
            match drive::survey(&mut adapter, &job) {
                Ok(survey) => {
                    if survey.refusals.is_empty() {
                        fields.push((
                            "key_diff".to_owned(),
                            key_diff(&survey.entries, &committed, &spec_path),
                        ));
                    }
                    survey.refusals
                }
                Err(why) => return emit_report(runner_failed(fields, &why), RUNNER_FAILED),
            }
        };
        if refusals.is_empty() {
            return emit_report(fields, 0);
        }
        set(
            &mut fields,
            "lints",
            lints_json(&unparsed(&refusals, &compiled.modules, &doc_name, &out)),
        );
        return emit_report(fields, REFUSED);
    }

    if !crate::preflight::gate("alloy") {
        return emit_report(runner_failed(fields, "preflight refused"), RUNNER_FAILED);
    }
    let _hold = match crate::exclusive::hold("assay") {
        Ok(hold) => hold,
        Err(code) => {
            let code = if code == CONTENTION {
                CONTENTION
            } else {
                RUNNER_FAILED
            };
            return emit_report(runner_failed(fields, "the heavy-work lock"), code);
        }
    };
    let job = Job {
        out: &out,
        modules: &compiled.modules,
        conjunctions: &compiled.conjunctions,
        committed: &committed,
        tier: ask.tier,
        caps,
        target: &target,
        stem: &stem,
        progress: ask.progress,
    };
    let (rows, refusals) = match drive::run(&jvm, &job) {
        Ok(ran) => ran,
        Err(why) => return emit_report(runner_failed(fields, &why), RUNNER_FAILED),
    };
    if !refusals.is_empty() {
        set(
            &mut fields,
            "lints",
            lints_json(&unparsed(&refusals, &compiled.modules, &doc_name, &out)),
        );
        return emit_report(fields, REFUSED);
    }
    let judged: Vec<(Standing, Option<&Row>)> = rows
        .iter()
        .map(|c| {
            let prior = committed
                .iter()
                .find(|r| r.module == c.row.module && r.name == c.row.name);
            (lock::judge(prior, &c.row), prior)
        })
        .collect();
    let gone: Vec<&Row> = if target == Target::All {
        committed
            .iter()
            .filter(|r| {
                !rows
                    .iter()
                    .any(|c| c.row.module == r.module && c.row.name == r.name)
            })
            .collect()
    } else {
        Vec::new()
    };
    fields.push((
        "commands".to_owned(),
        Json::Arr(
            rows.iter()
                .zip(&judged)
                .map(|(c, (s, _))| command_json(c, *s, &rows))
                .collect(),
        ),
    ));
    let mismatched = judged
        .iter()
        .any(|(s, _)| matches!(s, Standing::Mismatch(_)))
        || !gone.is_empty();
    let unmeasured = judged
        .iter()
        .any(|(s, _)| matches!(s, Standing::Unmeasured | Standing::Owed));
    let mut summary: Fields = vec![
        ("path".to_owned(), Json::str(display_path(&lock_path))),
        ("tier".to_owned(), Json::str(ask.tier.name())),
    ];
    for (heading, standings) in [
        (
            "mismatches",
            &[
                Standing::Mismatch(lock::Why::New),
                Standing::Mismatch(lock::Why::Moved),
            ][..],
        ),
        ("unmeasured", &[Standing::Unmeasured][..]),
        ("owed", &[Standing::Owed][..]),
        ("deferred", &[Standing::Deferred][..]),
        ("accepted_reds", &[Standing::AcceptedRed][..]),
        ("accepted_unmeasured", &[Standing::AcceptedUnmeasured][..]),
        ("accepted_errors", &[Standing::AcceptedError][..]),
        ("carried", &[Standing::Carried][..]),
    ] {
        let listed: Vec<Json> = rows
            .iter()
            .zip(&judged)
            .filter(|(_, (s, _))| standings.contains(s))
            .map(|(c, _)| Json::str(format!("{}.{}", c.row.module, c.row.name)))
            .collect();
        summary.push((heading.to_owned(), Json::Arr(listed)));
    }
    summary.push((
        "gone".to_owned(),
        Json::Arr(
            gone.iter()
                .map(|r| Json::str(format!("{}.{}", r.module, r.name)))
                .collect(),
        ),
    ));
    let computed: Vec<Row> = rows.iter().map(|c| c.row.clone()).collect();
    let code = if ask.mode == Mode::Write {
        let next = lock::next(
            &committed,
            &computed,
            ask.tier == super::tier::Tier::Official,
        );
        print_diff(&committed, &next, &lock_path);
        match std::fs::write(&lock_path, lock::render(&next)) {
            Ok(()) => {
                summary.push(("status".to_owned(), Json::str("written")));
                0
            }
            Err(e) => {
                eprintln!("assay: writing {}: {e}", lock_path.display());
                summary.push(("status".to_owned(), Json::str("unwritten")));
                RED
            }
        }
    } else {
        let (status, code) = match (committed_text.is_some(), mismatched, unmeasured) {
            (false, _, _) => ("missing", RED),
            (true, true, _) => ("mismatch", RED),
            (true, false, true) => ("unmeasured", UNMEASURED),
            (true, false, false) => ("matches", 0),
        };
        summary.push(("status".to_owned(), Json::str(status)));
        code
    };
    let disagreeing = if ask.tier == super::tier::Tier::Official {
        disagreements(&rows, &compiled.conjunctions)
    } else {
        Vec::new()
    };
    if !disagreeing.is_empty() {
        eprintln!(
            "assay: the conjunction disagrees with its lines' own verdicts in: {}",
            disagreeing.join(", ")
        );
    }
    let code = if disagreeing.is_empty() { code } else { RED };
    summary.push((
        "construction_disagreements".to_owned(),
        Json::Arr(disagreeing.into_iter().map(Json::Str).collect()),
    ));
    fields.push(("lock".to_owned(), Json::Obj(summary)));
    emit_report(fields, code)
}

fn disagreements(rows: &[Computed], conjunctions: &[(String, Vec<String>)]) -> Vec<String> {
    let result = |module: &str, name: &str| {
        rows.iter()
            .find(|c| c.row.module == module && c.row.name == name)
            .map(|c| c.row.result)
    };
    conjunctions
        .iter()
        .filter(|(module, members)| {
            let Some(every) = result(module, super::book::EVERY_LINE).filter(|o| o.definite())
            else {
                return false;
            };
            let lines: Vec<lock::Outcome> = members
                .iter()
                .filter_map(|m| result(module, m))
                .filter(|o| o.definite())
                .collect();
            lines.len() == members.len()
                && (every == lock::Outcome::NoCounterexample)
                    != lines.iter().all(|o| *o == lock::Outcome::NoCounterexample)
        })
        .map(|(module, _)| module.clone())
        .collect()
}

fn resolve_target(target: &Target, modules: &[(String, emit::Rendered)]) -> Result<Target, String> {
    let Target::Only { module, label } = target else {
        return Ok(target.clone());
    };
    if !module.is_empty() {
        return Ok(target.clone());
    }
    let holders: Vec<String> = modules
        .iter()
        .filter(|(_, r)| {
            super::alloy::items(&r.text, 1).iter().any(
                |i| matches!(i.head(), super::Head::Command { name: Some(n), .. } if n == *label),
            )
        })
        .map(|(n, _)| n.trim_end_matches(super::MODULE_SUFFIX).to_owned())
        .collect();
    match holders.as_slice() {
        [one] => Ok(Target::Only {
            module: one.clone(),
            label: label.clone(),
        }),
        [] => Err(format!("--only {label} names no command")),
        many => Err(format!(
            "--only {label} is ambiguous; name one of {}",
            many.iter()
                .map(|m| format!("{m}.{label}"))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn premise(c: &Computed, rows: &[Computed]) -> Option<String> {
    if c.row.kind != "check" {
        return None;
    }
    let find = |name: &str| {
        rows.iter()
            .find(|r| r.row.module == c.row.module && r.row.name == name)
            .map(|r| r.row.result.name().to_owned())
    };
    let book = c.row.module.strip_prefix("book_");
    Some(
        find(&format!("{}_premise", c.row.name))
            .or_else(|| book.and_then(find))
            .or_else(|| book.and_then(|_| find(&c.row.module)))
            .unwrap_or_else(|| "absent".to_owned()),
    )
}

fn command_json(c: &Computed, standing: Standing, rows: &[Computed]) -> Json {
    let mut json = c.row.json();
    if let Json::Obj(fields) = &mut json {
        if let Some(p) = premise(c, rows) {
            fields.push(("premise".to_owned(), Json::str(p)));
        }
        let ms = |n: Option<u64>| n.map_or(Json::Null, Json::Num);
        fields.push(("provenance".to_owned(), Json::str(c.provenance.name())));
        fields.push(("standing".to_owned(), Json::str(standing.name())));
        fields.push(("wall_ms".to_owned(), ms(c.wall_ms)));
        fields.push(("solve_ms".to_owned(), ms(c.solve_ms)));
        if let Some(note) = &c.note {
            fields.push(("note".to_owned(), Json::str(note)));
        }
    }
    json
}

fn key_diff(entries: &[drive::Entry], committed: &[Row], spec: &str) -> Json {
    let moved: Vec<String> = entries
        .iter()
        .filter(|e| {
            committed
                .iter()
                .find(|r| r.module == e.module && r.name == e.info.label)
                .is_none_or(|r| !(r.result.definite() && r.key.is_some() && r.key == e.key))
        })
        .map(|e| format!("{}.{}", e.module, e.info.label))
        .collect();
    if !moved.is_empty() {
        eprintln!(
            "assay: {} rows of {spec} go unmeasured with this commit (their keys moved from the lock's): {}",
            moved.len(),
            moved.join(", ")
        );
    }
    Json::Arr(moved.into_iter().map(Json::Str).collect())
}

fn print_diff(before: &[Row], after: &[Row], lock_path: &Path) {
    let id = |r: &Row| format!("{}.{}", r.module, r.name);
    for r in after {
        match before
            .iter()
            .find(|b| b.module == r.module && b.name == r.name)
        {
            None => eprintln!(
                "assay: {} + {} {}",
                lock_path.display(),
                id(r),
                r.result.name()
            ),
            Some(b) if b.result != r.result => eprintln!(
                "assay: {} ~ {} {} -> {}",
                lock_path.display(),
                id(r),
                b.result.name(),
                r.result.name()
            ),
            Some(_) => {}
        }
    }
    for b in before.iter().filter(|b| {
        !after
            .iter()
            .any(|r| r.module == b.module && r.name == b.name)
    }) {
        eprintln!(
            "assay: {} - {} {}",
            lock_path.display(),
            id(b),
            b.result.name()
        );
    }
}

fn unparsed(
    refusals: &[(String, Refusal)],
    modules: &[(String, emit::Rendered)],
    doc_name: &str,
    out: &Path,
) -> Vec<Finding> {
    type Group = (String, Vec<String>, Option<(String, usize)>);
    let mut grouped: Vec<Group> = Vec::new();
    for (module, r) in refusals {
        let origin = r.file.as_deref().and_then(|file| {
            let name = file.rsplit(['/', '\\']).next()?;
            let (_, rendered) = modules.iter().find(|(n, _)| n == name)?;
            let line = usize::try_from(r.line?).ok()?;
            let column = usize::try_from(r.column.unwrap_or(1)).ok()?;
            rendered
                .origin(line, column)
                .map(|s| (s.file.clone(), s.line))
        });
        let message = drive::portable(&r.message, out);
        match grouped.iter_mut().find(|(m, _, _)| *m == message) {
            Some((_, stopped, _)) => stopped.push(module.clone()),
            None => grouped.push((message, vec![module.clone()], origin)),
        }
    }
    grouped
        .into_iter()
        .map(|(message, stopped, origin)| {
            let (file, line) = origin.unwrap_or_else(|| (doc_name.to_owned(), 0));
            Finding::new(Lint::AlloyParses, &file, line)
                .with("modules", stopped.join(" "))
                .with("message", message)
        })
        .collect()
}

fn runner_failed(mut fields: Fields, why: &str) -> Fields {
    fields.push(("runner".to_owned(), Json::str(why)));
    fields
}

fn set(fields: &mut [(String, Json)], key: &str, value: Json) {
    if let Some((_, slot)) = fields.iter_mut().find(|(k, _)| k == key) {
        *slot = value;
    }
}

#[cfg(test)]
mod tests {
    use super::premise;
    use crate::assay::drive::{Computed, Provenance};
    use crate::assay::lock::Outcome;
    use crate::assay::lock::Row;

    fn computed(module: &str, name: &str, kind: &str, result: Outcome) -> Computed {
        Computed {
            row: Row {
                module: module.to_owned(),
                name: name.to_owned(),
                kind: kind.to_owned(),
                scope: String::new(),
                result,
                phase: None,
                budget: None,
                heap: None,
                size: None,
                key: None,
                platform: None,
                message: None,
            },
            provenance: Provenance::Fresh,
            wall_ms: None,
            solve_ms: None,
            note: None,
            index: 0,
        }
    }

    #[test]
    fn a_conjunction_disagreeing_with_its_individually_solved_lines_is_found() {
        let conj = vec![(
            "book_b".to_owned(),
            vec!["line_1".to_owned(), "line_2".to_owned()],
        )];
        let rows = |every: Outcome, second: Outcome| {
            vec![
                computed("book_b", "line_1", "check", Outcome::NoCounterexample),
                computed("book_b", "line_2", "check", second),
                computed("book_b", "every_line", "check", every),
            ]
        };
        let green = Outcome::NoCounterexample;
        assert!(super::disagreements(&rows(green, green), &conj).is_empty());
        assert!(
            super::disagreements(
                &rows(Outcome::Counterexample, Outcome::Counterexample),
                &conj
            )
            .is_empty()
        );
        assert_eq!(
            super::disagreements(&rows(green, Outcome::Counterexample), &conj),
            vec!["book_b"]
        );
        assert_eq!(
            super::disagreements(&rows(Outcome::Counterexample, green), &conj),
            vec!["book_b"]
        );
        assert!(
            super::disagreements(&rows(Outcome::Timeout, Outcome::Counterexample), &conj)
                .is_empty()
        );
    }

    #[test]
    fn a_premise_is_the_twin_else_the_books_run_else_absent() {
        let rows = vec![
            computed("laws", "k", "check", Outcome::NoCounterexample),
            computed("laws", "k_premise", "run", Outcome::Unsat),
            computed("laws", "lone", "check", Outcome::NoCounterexample),
            computed("book_b", "line_1", "check", Outcome::NoCounterexample),
            computed("book_b", "b", "run", Outcome::Sat),
            computed("book_corpus", "law", "check", Outcome::NoCounterexample),
            computed("book_corpus", "book_corpus", "run", Outcome::Deferred),
        ];
        let got: Vec<Option<String>> = rows.iter().map(|c| premise(c, &rows)).collect();
        let want = [
            Some("unsat"),
            None,
            Some("absent"),
            Some("sat"),
            None,
            Some("deferred"),
            None,
        ];
        assert_eq!(got, want.map(|w| w.map(str::to_owned)));
    }
}
