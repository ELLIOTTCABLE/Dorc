use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::{Jvm, adapter_digest, adapter_source};
use crate::json::{Json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Budget {
    pub(crate) cpu_s: u64,
    pub(crate) wall_s: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Machine {
    pub(crate) heap_mb: u64,
    pub(crate) procs: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Size {
    pub(crate) primary_vars: u64,
    pub(crate) vars: u64,
    pub(crate) clauses: u64,
}

impl Size {
    fn read(v: &Value) -> Option<Self> {
        Some(Self {
            primary_vars: v.u64("primary_vars")?,
            vars: v.u64("vars")?,
            clauses: v.u64("clauses")?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Exceeded {
    Cpu(u64),
    Wall(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Phase {
    Translating,
    Solving,
}

impl Phase {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Translating => "translating",
            Self::Solving => "solving",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandInfo {
    pub(crate) index: u64,
    pub(crate) label: String,
    pub(crate) check: bool,
    pub(crate) expects: Option<u64>,
    pub(crate) scope: String,
    pub(crate) bitwidth: u64,
    pub(crate) unbounded_steps: bool,
    pub(crate) synthesized: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Parsed {
    pub(crate) module: String,
    pub(crate) loaded: Vec<(String, String)>,
    pub(crate) commands: Vec<CommandInfo>,
    pub(crate) options: Value,
    pub(crate) sigs: Vec<(String, Vec<String>)>,
    pub(crate) warnings: Vec<Warning>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Refusal {
    pub(crate) message: String,
    pub(crate) file: Option<String>,
    pub(crate) line: Option<u64>,
    pub(crate) column: Option<u64>,
}

/// What Alloy's type checker said about a module it still accepted. An always-empty join or a
/// formula two others were silently conjoined into often means the model is not the one meant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Warning {
    pub(crate) message: String,
    pub(crate) file: Option<String>,
    pub(crate) line: Option<u64>,
    pub(crate) column: Option<u64>,
}

#[derive(Debug, Clone)]
pub(crate) enum Solved {
    Found {
        sat: bool,
        solve_ms: u64,
        size: Option<Size>,
        options: Value,
        xml: Option<String>,
        text: Option<String>,
    },
    Timeout {
        exceeded: Exceeded,
        phase: Phase,
        size: Option<Size>,
        translated_ms: Option<u64>,
    },
    OutOfMemory(String),
    Error(String),
}

struct Live {
    proc: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
    stderr: Arc<Mutex<String>>,
}

#[derive(Debug)]
pub(crate) struct Adapter {
    java: PathBuf,
    classpath: String,
    machine: Machine,
    live: Option<LiveHandle>,
}

struct LiveHandle(Live);

impl std::fmt::Debug for LiveHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiveHandle(pid {})", self.0.proc.id())
    }
}

const STDERR_TAIL: usize = 16 * 1024;
const POLL: Duration = Duration::from_millis(200);
const ALIVE_EVERY: Duration = Duration::from_mins(5);

/// `41.3s`, `12m08s`, `2h14m05s`: read by a person, never parsed.
pub(crate) fn human(d: Duration) -> String {
    let s = d.as_secs();
    let (h, m, s) = (s / 3600, (s / 60) % 60, s % 60);
    match (h, m) {
        (0, 0) => format!("{:.1}s", d.as_secs_f64()),
        (0, _) => format!("{m}m{s:02}s"),
        _ => format!("{h}h{m:02}m{s:02}s"),
    }
}

/// sat4j's own counters, which a tick carries only while a sat4j solve is in flight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Effort {
    conflicts: u64,
    restarts: u64,
    learned: u64,
    decisions: u64,
}

impl Effort {
    fn read(tick: &Value) -> Option<Self> {
        Some(Self {
            conflicts: tick.u64("conflicts")?,
            restarts: tick.u64("restarts")?,
            learned: tick.u64("learned")?,
            decisions: tick.u64("decisions")?,
        })
    }
}

fn still_alive_due(waited: Duration, last_said: Duration) -> bool {
    waited.saturating_sub(last_said) >= ALIVE_EVERY
}

/// Reassurance, not progress: CPU of budget is the only ratio it may show, and before the child
/// reports its start there is no CPU to show.
fn still_alive_line(
    waited: Duration,
    cpu_ms: Option<u64>,
    budget: Budget,
    translated: Option<&Value>,
    effort: Option<Effort>,
) -> String {
    let Some(cpu_ms) = cpu_ms else {
        return format!(
            "alive, {} waited, the child has not started solving",
            human(waited)
        );
    };
    // The solver exists before translation ends, so its counts read zero until then.
    let phase = match (translated.and_then(|t| t.u64("clauses")), effort) {
        (Some(clauses), Some(e)) => format!(
            "solving {clauses} clauses, {} conflicts, {} restarts, {} learned, {} decisions",
            e.conflicts, e.restarts, e.learned, e.decisions
        ),
        (Some(clauses), None) => format!("solving {clauses} clauses"),
        (None, _) => "translating".to_owned(),
    };
    format!(
        "alive, {} waited, cpu {} of {}s, {phase}",
        human(waited),
        human(Duration::from_millis(cpu_ms)),
        budget.cpu_s
    )
}

impl Adapter {
    pub(crate) fn new(jvm: &Jvm, machine: Machine) -> Result<Self, String> {
        let classes = compiled(jvm)?;
        let sep = if cfg!(windows) { ";" } else { ":" };
        Ok(Self {
            java: jvm.java.clone(),
            classpath: format!("{}{sep}{}", jvm.jar.display(), classes.display()),
            machine,
            live: None,
        })
    }

    fn spawn(&mut self) -> Result<&mut Live, String> {
        if self.live.is_none() {
            let mut proc = Command::new(&self.java)
                .arg(format!("-Xmx{}m", self.machine.heap_mb))
                .arg(format!("-XX:ActiveProcessorCount={}", self.machine.procs))
                .arg("-XX:+ExitOnOutOfMemoryError")
                .arg("-cp")
                .arg(&self.classpath)
                .arg("AlloyAdapter")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("{}: {e}", self.java.display()))?;
            let stdin = proc.stdin.take().ok_or("no adapter stdin")?;
            let stdout = proc.stdout.take().ok_or("no adapter stdout")?;
            let mut err = proc.stderr.take().ok_or("no adapter stderr")?;
            let (tx, lines) = channel();
            std::thread::spawn(move || {
                for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                    if tx.send(line).is_err() {
                        return;
                    }
                }
            });
            let stderr = Arc::new(Mutex::new(String::new()));
            let sink = Arc::clone(&stderr);
            std::thread::spawn(move || {
                let mut buf = [0u8; 4096];
                while let Ok(n) = err.read(&mut buf) {
                    if n == 0 {
                        return;
                    }
                    if let (Ok(mut tail), Some(bytes)) = (sink.lock(), buf.get(..n)) {
                        tail.push_str(&String::from_utf8_lossy(bytes));
                        let excess = tail.len().saturating_sub(STDERR_TAIL);
                        let cut = (excess..tail.len())
                            .find(|i| tail.is_char_boundary(*i))
                            .unwrap_or(0);
                        tail.drain(..cut);
                    }
                }
            });
            self.live = Some(LiveHandle(Live {
                proc,
                stdin,
                lines,
                stderr,
            }));
        }
        self.live
            .as_mut()
            .map(|l| &mut l.0)
            .ok_or_else(|| "adapter did not start".to_owned())
    }

    pub(crate) fn kill(&mut self) {
        if let Some(LiveHandle(mut live)) = self.live.take() {
            let _ = live.proc.kill();
            let _ = live.proc.wait();
        }
    }

    fn call(&mut self, request: &Json, budget: Option<Budget>, speaks: bool) -> Call {
        let live = match self.spawn() {
            Ok(live) => live,
            Err(e) => return Call::Died(e),
        };
        let framed = format!("{}\n", request.line());
        if live
            .stdin
            .write_all(framed.as_bytes())
            .and_then(|()| live.stdin.flush())
            .is_err()
        {
            return self.died();
        }
        let sent = Instant::now();
        let mut base_cpu: Option<u64> = None;
        let mut translated: Option<Value> = None;
        let mut cpu_used: Option<u64> = None;
        let mut effort: Option<Effort> = None;
        let mut last_said = Duration::ZERO;
        loop {
            let Some(LiveHandle(live)) = self.live.as_mut() else {
                return Call::Died("adapter vanished".to_owned());
            };
            match live.lines.recv_timeout(POLL) {
                Ok(text) => {
                    let Some(value) = Value::parse(&text) else {
                        continue;
                    };
                    match value.str("event") {
                        // std reads no child's CPU portably, so the child reports its own.
                        Some("start") => base_cpu = value.u64("cpu_ms"),
                        Some("translated") => {
                            tracing::info!(
                                "translated in {}, {} clauses, {} primary vars",
                                human(Duration::from_millis(value.u64("ms").unwrap_or(0))),
                                value.u64("clauses").unwrap_or(0),
                                value.u64("primary_vars").unwrap_or(0),
                            );
                            translated = Some(value);
                        }
                        Some(_) => {
                            effort = Effort::read(&value);
                            if let (Some(b), Some(now), Some(budget)) =
                                (base_cpu, value.u64("cpu_ms"), budget)
                            {
                                let used = now.saturating_sub(b);
                                cpu_used = Some(used);
                                if used >= budget.cpu_s.saturating_mul(1000) {
                                    self.kill();
                                    return Call::Exceeded(Exceeded::Cpu(budget.cpu_s), translated);
                                }
                            }
                        }
                        None => return Call::Reply(value),
                    }
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return self.died(),
            }
            // Driven by the poll, not by the child's ticks, so a frozen child still gets a line.
            if let Some(budget) = budget
                && speaks
            {
                let waited = sent.elapsed();
                if still_alive_due(waited, last_said) {
                    last_said = waited;
                    tracing::info!(
                        "{}",
                        still_alive_line(waited, cpu_used, budget, translated.as_ref(), effort)
                    );
                }
            }
            if let Some(budget) = budget
                && sent.elapsed() >= Duration::from_secs(budget.wall_s)
            {
                self.kill();
                return Call::Exceeded(Exceeded::Wall(budget.wall_s), translated);
            }
        }
    }

    fn died(&mut self) -> Call {
        let Some(LiveHandle(mut live)) = self.live.take() else {
            return Call::Died("adapter vanished".to_owned());
        };
        let status = live.proc.wait().ok();
        std::thread::sleep(Duration::from_millis(50));
        let tail = live.stderr.lock().map(|t| t.clone()).unwrap_or_default();
        if tail.contains("OutOfMemoryError") {
            return Call::OutOfMemory(tail.trim().to_owned());
        }
        Call::Died(format!(
            "adapter exited {}: {}",
            status
                .and_then(|s| s.code())
                .map_or_else(|| "on a signal".to_owned(), |c| c.to_string()),
            tail.trim()
        ))
    }

    pub(crate) fn parse(
        &mut self,
        root: &Path,
        opens: &[(String, PathBuf)],
        solver: &str,
    ) -> Result<Parsed, Refusal> {
        let request = request("parse", root, opens, &[("solver", Json::str(solver))]);
        let reply = match self.call(&request, None, false) {
            Call::Reply(v) => v,
            other => return Err(other.refusal()),
        };
        if reply.bool("ok") != Some(true) {
            return Err(refusal(&reply));
        }
        Ok(Parsed {
            module: reply.str("module").unwrap_or_default().to_owned(),
            loaded: reply
                .arr("loaded")
                .iter()
                .filter_map(|f| Some((f.str("path")?.to_owned(), f.str("text")?.to_owned())))
                .collect(),
            commands: reply
                .arr("commands")
                .iter()
                .filter_map(command_info)
                .collect(),
            options: reply.get("options").cloned().unwrap_or(Value::Null),
            sigs: reply
                .arr("sigs")
                .iter()
                .filter_map(|s| {
                    let fields = s
                        .arr("fields")
                        .iter()
                        .filter_map(|f| match f {
                            Value::Str(f) => Some(f.clone()),
                            _ => None,
                        })
                        .collect();
                    Some((s.str("label")?.to_owned(), fields))
                })
                .collect(),
            warnings: warnings(&reply),
        })
    }

    pub(crate) fn parse_only(
        &mut self,
        root: &Path,
        opens: &[(String, PathBuf)],
    ) -> Result<(String, Vec<Warning>), Refusal> {
        match self.call(&request("parse-only", root, opens, &[]), None, false) {
            Call::Reply(v) if v.bool("ok") == Some(true) => {
                Ok((v.str("module").unwrap_or_default().to_owned(), warnings(&v)))
            }
            Call::Reply(v) => Err(refusal(&v)),
            other => Err(other.refusal()),
        }
    }

    pub(crate) fn solve(
        &mut self,
        root: &Path,
        opens: &[(String, PathBuf)],
        ask: &Ask<'_>,
        budget: Budget,
    ) -> Solved {
        let extra = [
            ("index", Json::Num(ask.index)),
            ("solver", Json::str(ask.solver)),
            ("xml", Json::Bool(ask.xml)),
            ("text", Json::Bool(ask.text)),
        ];
        match self.call(&request("solve", root, opens, &extra), Some(budget), true) {
            Call::Reply(v) => match v.str("result") {
                Some(r @ ("sat" | "unsat")) => Solved::Found {
                    sat: r == "sat",
                    solve_ms: v.u64("solve_ms").unwrap_or(0),
                    size: v.get("size").and_then(Size::read),
                    options: v.get("options").cloned().unwrap_or(Value::Null),
                    xml: v.str("instance_xml").map(str::to_owned),
                    text: v.str("instance").map(str::to_owned),
                },
                Some("out-of-memory") => {
                    self.kill();
                    Solved::OutOfMemory(v.str("message").unwrap_or_default().to_owned())
                }
                _ => Solved::Error(v.str("message").unwrap_or_default().to_owned()),
            },
            Call::Exceeded(exceeded, translated) => Solved::Timeout {
                exceeded,
                phase: if translated.is_some() {
                    Phase::Solving
                } else {
                    Phase::Translating
                },
                size: translated.as_ref().and_then(Size::read),
                translated_ms: translated.as_ref().and_then(|t| t.u64("ms")),
            },
            Call::OutOfMemory(m) => Solved::OutOfMemory(m),
            Call::Died(m) => Solved::Error(m),
        }
    }

    pub(crate) fn eval(
        &mut self,
        root: &Path,
        index: u64,
        xml: &str,
        budget: Budget,
    ) -> Option<bool> {
        let extra = [
            ("index", Json::Num(index)),
            ("instance_xml", Json::str(xml)),
        ];
        match self.call(&request("eval", root, &[], &extra), Some(budget), false) {
            Call::Reply(v) if v.bool("ok") == Some(true) => v.bool("value"),
            _ => None,
        }
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        self.kill();
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ask<'a> {
    pub(crate) index: u64,
    pub(crate) solver: &'a str,
    pub(crate) xml: bool,
    pub(crate) text: bool,
}

enum Call {
    Reply(Value),
    Exceeded(Exceeded, Option<Value>),
    OutOfMemory(String),
    Died(String),
}

impl Call {
    fn refusal(self) -> Refusal {
        let message = match self {
            Self::Reply(v) => return refusal(&v),
            Self::Exceeded(..) => "the adapter exceeded its budget".to_owned(),
            Self::OutOfMemory(m) | Self::Died(m) => m,
        };
        Refusal {
            message,
            file: None,
            line: None,
            column: None,
        }
    }
}

fn refusal(v: &Value) -> Refusal {
    Refusal {
        message: v.str("message").unwrap_or("the adapter refused").to_owned(),
        file: v.str("file").map(str::to_owned),
        line: v.u64("line"),
        column: v.u64("column"),
    }
}

fn warnings(v: &Value) -> Vec<Warning> {
    v.arr("warnings")
        .iter()
        .map(|w| Warning {
            message: w.str("message").unwrap_or_default().to_owned(),
            file: w.str("file").map(str::to_owned),
            line: w.u64("line"),
            column: w.u64("column"),
        })
        .collect()
}

fn command_info(v: &Value) -> Option<CommandInfo> {
    Some(CommandInfo {
        index: v.u64("index")?,
        label: v.str("label")?.to_owned(),
        check: v.bool("check")?,
        expects: v.u64("expects"),
        scope: v.str("scope")?.to_owned(),
        bitwidth: v.u64("bitwidth")?,
        unbounded_steps: v.bool("unbounded_steps")?,
        synthesized: v.bool("synthesized")?,
    })
}

fn request(verb: &str, root: &Path, opens: &[(String, PathBuf)], extra: &[(&str, Json)]) -> Json {
    let mut fields = vec![
        ("verb".to_owned(), Json::str(verb)),
        ("root".to_owned(), Json::str(root.display().to_string())),
    ];
    if !opens.is_empty() {
        fields.push((
            "opens".to_owned(),
            Json::Obj(
                opens
                    .iter()
                    .map(|(m, f)| (m.clone(), Json::str(f.display().to_string())))
                    .collect(),
            ),
        ));
    }
    fields.extend(extra.iter().map(|(k, v)| ((*k).to_owned(), v.clone())));
    Json::Obj(fields)
}

fn compiled(jvm: &Jvm) -> Result<PathBuf, String> {
    let digest = adapter_digest()?;
    let root = internal_tooling::target_dir().join("alloy");
    let dir = root.join(format!("adapter-{}", digest.get(..16).unwrap_or(&digest)));
    if dir.join("AlloyAdapter.class").is_file() {
        return Ok(dir);
    }
    let scratch = root.join(format!(
        "adapter-{}.{}",
        digest.get(..16).unwrap_or(&digest),
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).map_err(|e| format!("{}: {e}", scratch.display()))?;
    tracing::info!("compiling the adapter with javac into {}", dir.display());
    let out = Command::new(&jvm.javac)
        .arg("-nowarn")
        .arg("-cp")
        .arg(&jvm.jar)
        .arg("-d")
        .arg(&scratch)
        .arg(adapter_source())
        .output()
        .map_err(|e| format!("{}: {e}", jvm.javac.display()))?;
    if !out.status.success() {
        let _ = std::fs::remove_dir_all(&scratch);
        return Err(format!(
            "javac refused the adapter: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    if std::fs::rename(&scratch, &dir).is_err() {
        let _ = std::fs::remove_dir_all(&scratch);
    }
    if dir.join("AlloyAdapter.class").is_file() {
        Ok(dir)
    } else {
        Err(format!(
            "{}: the compiled adapter did not land",
            dir.display()
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Budget, Effort, human, still_alive_due, still_alive_line};
    use crate::json::Value;

    #[test]
    fn still_alive_speaks_once_per_five_minutes_from_the_last_line() {
        // The poll runs five times a second for hours; the throttle alone keeps that to one line
        // per interval, measured from the last line said rather than from each poll.
        let s = Duration::from_secs;
        assert!(!still_alive_due(s(0), s(0)));
        assert!(!still_alive_due(Duration::from_millis(299_900), s(0)));
        assert!(still_alive_due(s(300), s(0)));
        let said = Duration::from_millis(300_200);
        assert!(!still_alive_due(s(301), said));
        assert!(!still_alive_due(s(600), said));
        assert!(still_alive_due(Duration::from_millis(600_200), said));
    }

    #[test]
    fn still_alive_adds_solver_effort_only_when_a_tick_carries_it() {
        // Only the adapter's sat4j factory puts counters on a tick; any other tick must leave the
        // line exactly as it was, and a counted one must carry each count after the phase, but
        // only once translation is over, since the counts read zero until then.
        let budget = Budget {
            cpu_s: 1800,
            wall_s: 3600,
        };
        let translated = Value::parse(r#"{"event":"translated","clauses":2381046}"#);
        let plain = Value::parse(r#"{"event":"tick","cpu_ms":5}"#).expect("json");
        assert_eq!(Effort::read(&plain), None);
        let without = still_alive_line(
            Duration::from_mins(10),
            Some(420_000),
            budget,
            translated.as_ref(),
            None,
        );
        assert!(without.ends_with("solving 2381046 clauses"));

        let counted = Value::parse(
            r#"{"event":"tick","cpu_ms":5,"conflicts":1204331,"restarts":88,"learned":40213,"decisions":9912345}"#,
        )
        .expect("json");
        let with = still_alive_line(
            Duration::from_mins(10),
            Some(420_000),
            budget,
            translated.as_ref(),
            Effort::read(&counted),
        );
        let tail = with
            .strip_prefix(&without)
            .expect("the counts follow the phase");
        for count in ["1204331", "88", "40213", "9912345"] {
            assert!(tail.contains(count), "{count} missing from {with}");
        }

        let translating =
            |effort| still_alive_line(Duration::from_mins(10), Some(420_000), budget, None, effort);
        assert_eq!(translating(Effort::read(&counted)), translating(None));

        let unstarted = still_alive_line(
            Duration::from_mins(10),
            None,
            budget,
            None,
            Effort::read(&counted),
        );
        assert!(!unstarted.contains("1204331"));
    }

    #[test]
    fn durations_render_for_a_person() {
        // The human's own examples; seconds past a minute are what a person cannot read.
        let s = Duration::from_secs;
        assert_eq!(human(Duration::from_millis(41_300)), "41.3s");
        assert_eq!(human(s(60)), "1m00s");
        assert_eq!(human(s(728)), "12m08s");
        assert_eq!(human(s(8045)), "2h14m05s");
        assert_eq!(human(s(31_244)), "8h40m44s");
    }
}
