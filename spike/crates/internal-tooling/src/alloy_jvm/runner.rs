use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use super::adapter::{Adapter, Ask, Budget, Exceeded, Machine, Solved};
use super::{Jvm, expand};
use crate::json::Json;

const USAGE: u8 = 2;

#[derive(Debug)]
struct Opts {
    wall_s: u64,
    cpu_s: Option<u64>,
    batch_s: u64,
    machine: Machine,
    instances: bool,
    parse_only: bool,
    only: Option<String>,
    solver: String,
    opens: Vec<(String, PathBuf)>,
    files: Vec<(PathBuf, String)>,
}

fn usage(why: &str) -> ExitCode {
    eprintln!(
        "alloy runner: {why}\nusage: alloy [--timeout <seconds>] [--cpu <seconds>] [--heap <MB>] [--procs <n>] [--batch-timeout <seconds>] [--parse-only] [--command <name>] [--instances] [--solver <id>] [--open <module>=<file.als>]... <file.als>..."
    );
    ExitCode::from(USAGE)
}

fn resolve(arg: &str) -> PathBuf {
    let path = PathBuf::from(arg);
    let path = match std::env::var_os("MISE_ORIGINAL_CWD") {
        Some(cwd) if path.is_relative() => PathBuf::from(cwd).join(path),
        _ => path,
    };
    std::path::absolute(&path).unwrap_or(path)
}

fn opts(args: &[String]) -> Result<Opts, String> {
    let mut o = Opts {
        wall_s: 120,
        cpu_s: None,
        batch_s: 540,
        machine: Machine {
            heap_mb: 2048,
            procs: 2,
        },
        instances: false,
        parse_only: false,
        only: None,
        solver: "sat4j".to_owned(),
        opens: Vec::new(),
        files: Vec::new(),
    };
    let mut it = args.iter();
    let number = |v: Option<&String>, flag: &str| {
        v.and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| format!("{flag} takes a number"))
    };
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--timeout" => o.wall_s = number(it.next(), arg)?,
            "--cpu" => o.cpu_s = Some(number(it.next(), arg)?),
            "--batch-timeout" => o.batch_s = number(it.next(), arg)?,
            "--heap" => o.machine.heap_mb = number(it.next(), arg)?,
            "--procs" => {
                o.machine.procs =
                    u32::try_from(number(it.next(), arg)?).map_err(|_| "--procs is too large")?;
            }
            "--instances" => o.instances = true,
            "--parse-only" => o.parse_only = true,
            "--solver" => o
                .solver
                .clone_from(it.next().ok_or("--solver takes an id")?),
            "--command" => o.only = Some(it.next().ok_or("--command takes a name")?.clone()),
            "--open" => {
                let spec = it
                    .next()
                    .filter(|s| s.contains('='))
                    .ok_or("--open takes <module>=<file.als>")?;
                let (module, file) = spec.split_once('=').unwrap_or_default();
                o.opens.push((module.to_owned(), resolve(file)));
            }
            flag if flag.starts_with('-') => return Err(format!("unknown option {flag}")),
            path => {
                let resolved = resolve(path);
                if resolved.is_dir() {
                    for file in expand(&resolved) {
                        let given = file.display().to_string();
                        o.files.push((file, given));
                    }
                } else {
                    o.files.push((resolved, path.to_owned()));
                }
            }
        }
    }
    if o.files.is_empty() {
        return Err("no .als files given".to_owned());
    }
    Ok(o)
}

pub(crate) fn run(args: &[String]) -> ExitCode {
    let o = match opts(args) {
        Ok(o) => o,
        Err(why) => return usage(&why),
    };
    let jvm = match Jvm::from_env() {
        Ok(jvm) => jvm,
        Err(why) => return usage(&why),
    };
    let _hold = match crate::exclusive::hold("alloy") {
        Ok(hold) => hold,
        Err(code) => return ExitCode::from(code),
    };
    let mut adapter = match Adapter::new(&jvm, o.machine) {
        Ok(a) => a,
        Err(why) => {
            eprintln!("alloy runner: {why}");
            return ExitCode::from(1);
        }
    };
    if o.parse_only {
        return parse_only(&mut adapter, &o);
    }
    solve_all(&mut adapter, &o)
}

fn row_start(first: &mut bool) -> &'static str {
    if std::mem::replace(first, false) {
        "  "
    } else {
        ", "
    }
}

fn parse_only(adapter: &mut Adapter, o: &Opts) -> ExitCode {
    let mut ok = true;
    let mut first = true;
    println!("[");
    for (file, given) in &o.files {
        let (module, message) = match adapter.parse_only(file, &o.opens) {
            Ok(module) => (module, None),
            Err(r) => {
                ok = false;
                (given.clone(), Some(r.message))
            }
        };
        let mut fields = vec![
            ("module".to_owned(), Json::str(module)),
            ("file".to_owned(), Json::str(given)),
            (
                "result".to_owned(),
                Json::str(if message.is_some() { "error" } else { "parsed" }),
            ),
        ];
        if let Some(m) = message {
            fields.push(("message".to_owned(), Json::str(m)));
        }
        println!("{}{}", row_start(&mut first), Json::Obj(fields).line());
    }
    println!("]");
    ExitCode::from(u8::from(!ok))
}

fn agrees(result: &str, expects: Option<u64>) -> bool {
    match expects {
        Some(1) => matches!(result, "sat" | "counterexample"),
        Some(0) => matches!(result, "unsat" | "no-counterexample"),
        _ => matches!(result, "sat" | "no-counterexample"),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "the runner's one pass, in the order it prints"
)]
fn solve_all(adapter: &mut Adapter, o: &Opts) -> ExitCode {
    let cpu_s = o.cpu_s.unwrap_or(o.wall_s);
    let mut parsed = Vec::new();
    let mut labels = Vec::new();
    let mut total = 0usize;
    let mut any_error = false;
    for (file, given) in &o.files {
        let p = adapter.parse(file, &o.opens, &o.solver);
        if let Err(r) = &p
            && r.message.contains("unknown solver")
        {
            return usage(&format!(
                "unknown solver {} (see `java -jar <alloy jar> solvers`)",
                o.solver
            ));
        }
        if let Ok(p) = &p {
            for c in p.commands.iter().filter(|c| !c.synthesized) {
                labels.push(c.label.clone());
                if o.only.as_ref().is_none_or(|only| *only == c.label) {
                    total = total.saturating_add(1);
                }
            }
        } else {
            any_error = true;
        }
        parsed.push((file, given, p));
    }
    if let Some(only) = &o.only
        && total == 0
        && !any_error
    {
        eprintln!(
            "alloy runner: --command {only} matches no command; available: {}",
            labels.join(", ")
        );
        return ExitCode::from(USAGE);
    }
    eprintln!(
        "alloy runner: {total} commands; caps: {}s wall and {cpu_s}s cpu per command, {} MB heap, {} processors, {}s for the batch",
        o.wall_s, o.machine.heap_mb, o.machine.procs, o.batch_s
    );
    let budget = Budget {
        cpu_s,
        wall_s: o.wall_s,
    };
    let mut red = false;
    let mut first = true;
    let batch = Instant::now();
    println!("[");
    for (file, given, p) in &parsed {
        let p = match p {
            Ok(p) => p,
            Err(r) => {
                red = true;
                let row = Json::Obj(vec![
                    ("module".to_owned(), Json::str(*given)),
                    ("command".to_owned(), Json::Null),
                    ("kind".to_owned(), Json::Null),
                    ("scope".to_owned(), Json::Null),
                    ("result".to_owned(), Json::str("error")),
                    ("wall_ms".to_owned(), Json::Null),
                    ("solve_ms".to_owned(), Json::Null),
                    ("message".to_owned(), Json::str(&r.message)),
                ]);
                println!("{}{}", row_start(&mut first), row.line());
                continue;
            }
        };
        for c in p
            .commands
            .iter()
            .filter(|c| !c.synthesized && o.only.as_ref().is_none_or(|only| *only == c.label))
        {
            let kind = if c.check { "check" } else { "run" };
            let mut fields = vec![
                ("module".to_owned(), Json::str(&p.module)),
                ("command".to_owned(), Json::str(&c.label)),
                ("kind".to_owned(), Json::str(kind)),
                ("scope".to_owned(), Json::str(&c.scope)),
            ];
            if let Some(e) = c.expects {
                fields.push(("expect".to_owned(), Json::Num(e)));
            }
            let (result, wall_ms, solve_ms, message, instance) =
                if batch.elapsed().as_secs() >= o.batch_s {
                    (
                        "not-run".to_owned(),
                        None,
                        None,
                        Some(format!(
                            "the batch cap of {}s was reached before this command started",
                            o.batch_s
                        )),
                        None,
                    )
                } else {
                    let started = Instant::now();
                    let ask = Ask {
                        index: c.index,
                        solver: &o.solver,
                        xml: false,
                        text: o.instances,
                    };
                    let solved = adapter.solve(file, &o.opens, &ask, budget);
                    let wall = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
                    match solved {
                        Solved::Found {
                            sat,
                            solve_ms,
                            text,
                            ..
                        } => {
                            let result = match (sat, c.check) {
                                (true, true) => "counterexample",
                                (true, false) => "sat",
                                (false, true) => "no-counterexample",
                                (false, false) => "unsat",
                            };
                            (result.to_owned(), Some(wall), Some(solve_ms), None, text)
                        }
                        Solved::Timeout {
                            exceeded,
                            size,
                            translated_ms,
                            ..
                        } => {
                            let exceeded = match exceeded {
                                Exceeded::Wall(s) => format!("{s}s wall-clock"),
                                Exceeded::Cpu(s) => format!("{s}s cpu"),
                            };
                            let progress = match (size, translated_ms) {
                                (Some(s), Some(ms)) => format!(
                                    "translated in {ms}ms: {} primary vars, {} vars, {} clauses",
                                    s.primary_vars, s.vars, s.clauses
                                ),
                                _ => "still translating".to_owned(),
                            };
                            (
                                "timeout".to_owned(),
                                Some(wall),
                                None,
                                Some(format!("exceeded {exceeded}; {progress}")),
                                None,
                            )
                        }
                        Solved::OutOfMemory(m) => (
                            "error".to_owned(),
                            Some(wall),
                            None,
                            Some(format!("out of memory: {m}")),
                            None,
                        ),
                        Solved::Error(m) => ("error".to_owned(), Some(wall), None, Some(m), None),
                    }
                };
            if !agrees(&result, c.expects) {
                red = true;
            }
            let ms = |n: Option<u64>| n.map_or(Json::Null, Json::Num);
            fields.push(("result".to_owned(), Json::str(result)));
            fields.push(("wall_ms".to_owned(), ms(wall_ms)));
            fields.push(("solve_ms".to_owned(), ms(solve_ms)));
            if let Some(m) = message {
                fields.push(("message".to_owned(), Json::str(m)));
            }
            if let Some(i) = instance {
                fields.push(("instance".to_owned(), Json::str(i)));
            }
            println!("{}{}", row_start(&mut first), Json::Obj(fields).line());
        }
    }
    println!("]");
    ExitCode::from(u8::from(red))
}

#[cfg(test)]
mod tests {
    use super::agrees;

    #[test]
    fn an_expect_is_judged_by_agreement_and_its_absence_by_greenness() {
        assert!(agrees("counterexample", Some(1)));
        assert!(!agrees("no-counterexample", Some(1)));
        assert!(agrees("unsat", Some(0)));
        assert!(!agrees("unsat", None));
        assert!(
            !agrees("timeout", Some(1)),
            "a cap is red whatever was expected"
        );
    }
}
