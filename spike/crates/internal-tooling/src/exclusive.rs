//! `exclusive --task <name> -- <command> [args…]`: the machine-global heavy-work lock
//! (`notes/30Y` § 3, runner). A second heavy task on one machine is REFUSED, naming the holder,
//! rather than queued: parallel builders and worktrees share one box, and a caller that waits in
//! silence sleeps for as long as the holder's solver runs.
//!
//! The lock lives in the user's cache directory, not the project, so every worktree sees it; each
//! platform leg keeps its own (WSL does not see `%LOCALAPPDATA%`).

use std::io::{ErrorKind, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::json::{Json, read_num, read_str};

/// `EX_TEMPFAIL`: try again later.
const REFUSED: u8 = 75;
/// A lock file is a few hundred bytes; anything larger is not ours and reads as no holder.
const LOCK_READ_CAP: u64 = 64 * 1024;

#[derive(Debug)]
struct Holder {
    pid: u32,
    task: String,
    started: u64,
    cwd: String,
}

pub(crate) fn run(args: &[String]) -> ExitCode {
    let (task, command) = match args {
        [flag, task, dashes, command @ ..]
            if flag == "--task" && dashes == "--" && !command.is_empty() =>
        {
            (task, command)
        }
        _ => {
            eprintln!("exclusive: usage: exclusive --task <name> -- <command> [args…]");
            return ExitCode::from(2);
        }
    };
    let Some(lock) = lock_path() else {
        eprintln!(
            "exclusive: no user cache directory (LOCALAPPDATA, XDG_CACHE_HOME, or HOME) to hold the lock"
        );
        return ExitCode::from(2);
    };
    match acquire(&lock, task) {
        Ok(()) => {}
        Err(Acquire::Held(holder)) => {
            refuse(&lock, &holder);
            return ExitCode::from(REFUSED);
        }
        Err(Acquire::Io(e)) => {
            eprintln!("exclusive: {}: {e}", lock.display());
            return ExitCode::from(2);
        }
    }
    let (program, rest) = command.split_first().unwrap_or((task, &[]));
    let status = Command::new(program).args(rest).status();
    release(&lock);
    match status {
        Ok(status) => ExitCode::from(
            status
                .code()
                .and_then(|c| u8::try_from(c).ok())
                .unwrap_or(1),
        ),
        Err(e) => {
            eprintln!("exclusive: {program}: {e}");
            ExitCode::from(127)
        }
    }
}

enum Acquire {
    Held(Holder),
    Io(std::io::Error),
}

/// Create-new, or read the holder; a dead holder's lock is taken over, once.
fn acquire(lock: &Path, task: &str) -> Result<(), Acquire> {
    if let Some(dir) = lock.parent() {
        std::fs::create_dir_all(dir).map_err(Acquire::Io)?;
    }
    for attempt in 0..2 {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(lock)
        {
            Ok(mut file) => {
                let body = Json::obj([
                    ("pid", Json::Num(u64::from(std::process::id()))),
                    ("task", Json::str(task)),
                    ("started", Json::Num(now())),
                    (
                        "cwd",
                        Json::str(
                            std::env::current_dir()
                                .map(|d| d.display().to_string())
                                .unwrap_or_default(),
                        ),
                    ),
                ]);
                return file
                    .write_all(body.render().as_bytes())
                    .map_err(Acquire::Io);
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                let holder = read_holder(lock);
                match holder {
                    Some(h) if attempt > 0 || alive(h.pid) => return Err(Acquire::Held(h)),
                    stale => {
                        let who = stale.map_or_else(
                            || "an unreadable holder".to_owned(),
                            |h| format!("`{}` (pid {}, no longer running)", h.task, h.pid),
                        );
                        eprintln!("exclusive: taking over {} from {who}", lock.display());
                        match std::fs::remove_file(lock) {
                            Ok(()) => {}
                            Err(e) if e.kind() == ErrorKind::NotFound => {}
                            Err(e) => return Err(Acquire::Io(e)),
                        }
                    }
                }
            }
            Err(e) => return Err(Acquire::Io(e)),
        }
    }
    Err(Acquire::Held(read_holder(lock).unwrap_or(Holder {
        pid: 0,
        task: String::new(),
        started: 0,
        cwd: String::new(),
    })))
}

fn release(lock: &Path) {
    if read_holder(lock).is_some_and(|h| h.pid == std::process::id()) {
        let _ = std::fs::remove_file(lock);
    }
}

fn refuse(lock: &Path, h: &Holder) {
    let ago = now().saturating_sub(h.started);
    eprintln!(
        "exclusive: REFUSED. This machine runs one heavy task at a time, and `{}` holds the lock \
         (pid {}, started {}m{}s ago, in {}; lock file {}). Do other work and retry later; do not \
         wait on it in a loop. If that process is not really running, delete the lock file.",
        h.task,
        h.pid,
        ago / 60,
        ago % 60,
        h.cwd,
        lock.display()
    );
}

fn read_holder(lock: &Path) -> Option<Holder> {
    let mut text = String::new();
    std::fs::File::open(lock)
        .ok()?
        .take(LOCK_READ_CAP)
        .read_to_string(&mut text)
        .ok()?;
    Some(Holder {
        pid: u32::try_from(read_num(&text, "pid")?).ok()?,
        task: read_str(&text, "task").unwrap_or_default(),
        started: read_num(&text, "started").unwrap_or(0),
        cwd: read_str(&text, "cwd").unwrap_or_default(),
    })
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// `%LOCALAPPDATA%` on Windows; `$XDG_CACHE_HOME`, else `~/.cache`, elsewhere.
fn lock_path() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
    }?;
    Some(base.join("dorc").join("heavy-work.lock"))
}

/// Whether `pid` is running. A check that cannot run answers yes: a false refusal names its
/// holder and costs a retry, a false takeover runs two solvers at once.
#[cfg(windows)]
fn alive(pid: u32) -> bool {
    let exe = std::env::var_os("SystemRoot").map_or_else(
        || PathBuf::from("tasklist.exe"),
        |r| PathBuf::from(r).join("System32").join("tasklist.exe"),
    );
    let filter = format!("PID eq {pid}");
    let pid = pid.to_string();
    Command::new(exe)
        .args(["/FI", &filter, "/NH"])
        .output()
        .map_or(true, |out| {
            String::from_utf8_lossy(&out.stdout)
                .split_whitespace()
                .any(|word| word == pid)
        })
}

#[cfg(not(windows))]
fn alive(pid: u32) -> bool {
    Command::new("ps")
        .args(["-p", &pid.to_string()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_or(true, |s| s.success())
}

#[cfg(test)]
mod tests {
    use super::{Acquire, acquire, alive, read_holder, release};

    #[test]
    fn a_live_holder_refuses_and_a_dead_one_is_taken_over() {
        // The whole contract: this process is alive, so its own lock refuses a second taker; a
        // lock naming a pid that cannot be running is stale and taken over.
        let dir = std::env::temp_dir().join(format!("dorc-exclusive-test-{}", std::process::id()));
        let lock = dir.join("heavy-work.lock");
        let _ = std::fs::remove_dir_all(&dir);

        assert!(acquire(&lock, "first").is_ok());
        match acquire(&lock, "second") {
            Err(Acquire::Held(h)) => {
                assert_eq!((h.pid, h.task.as_str()), (std::process::id(), "first"));
            }
            _ => panic!("a live holder must refuse"),
        }
        release(&lock);
        assert!(!lock.exists(), "release removes our own lock");

        let dead = u32::MAX - 7;
        assert!(!alive(dead), "a pid this large is not running");
        std::fs::create_dir_all(&dir).expect("scratch dir");
        std::fs::write(
            &lock,
            format!("{{\"pid\": {dead}, \"task\": \"gone\", \"started\": 0, \"cwd\": \"x\"}}"),
        )
        .expect("stale lock");
        assert!(
            acquire(&lock, "third").is_ok(),
            "a dead holder's lock is taken over"
        );
        assert_eq!(read_holder(&lock).map(|h| h.task), Some("third".to_owned()));
        release(&lock);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
