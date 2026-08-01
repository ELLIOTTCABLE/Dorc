//! Internal repo tooling — NOT part of Dorc. See `Cargo.toml` for why this crate exists.
//!
//! Its one export is [`Posix`]: the answer to "where is a POSIX shell on this machine",
//! computed once, explicitly, from a source we already depend on.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The POSIX shell this repo's tooling and test corpus drive.
///
/// Dorc's product is sh, so the corpus cannot avoid needing one: it syntax-checks and
/// executes rendered artifacts. Native Windows ships no POSIX shell, which is why every
/// "Windows green" result before 2026-07-26 was really measured from git-bash.
#[derive(Debug, Clone)]
pub struct Posix {
    /// Absolute path to the shell. Never re-resolved from `PATH` at the point of use.
    pub shell: PathBuf,
    /// `dash` or `sh`. On Windows `sh` is git's bash-in-sh-mode, a materially weaker
    /// dialect check — it accepts `[[ ]]`, `<<<` and the rest of what the floor bans.
    pub name: &'static str,
    /// Windows only: git's `usr/bin`. A child needing `sed`/`awk`/`grep` gets this
    /// prepended to *its own* PATH — git's shells do not resolve their own siblings
    /// (measured), and nothing here ever touches the ambient PATH.
    pub utils_dir: Option<PathBuf>,
}

impl Posix {
    /// Locate a shell, or explain what to install.
    ///
    /// # Errors
    /// When no POSIX shell can be found: on Windows if git's userland is absent or its
    /// layout is unrecognized, elsewhere if neither `dash` nor `sh` is on `PATH`. The
    /// message names the remedy; callers are expected to print it and stop.
    ///
    /// Windows resolves through git — already a hard dependency, and the only POSIX
    /// userland guaranteed present. Deriving it structurally is also what disarms the
    /// `bash.exe` trap: on native Windows `bash` on PATH is `%SystemRoot%\System32\bash.exe`,
    /// the WSL launcher, which would silently run Linux binaries against Linux paths. We
    /// never PATH-search for a shell here, so it can never be selected.
    pub fn find() -> Result<Self, String> {
        if cfg!(windows) {
            let dir = git_usr_bin()?;
            for (name, file) in [("dash", "dash.exe"), ("sh", "sh.exe")] {
                let candidate = dir.join(file);
                if candidate.is_file() {
                    return Ok(Self {
                        shell: candidate,
                        name,
                        utils_dir: Some(dir),
                    });
                }
            }
            Err(format!(
                "git ships no dash.exe or sh.exe in {}",
                dir.display()
            ))
        } else {
            for name in ["dash", "sh"] {
                if let Some(shell) = which(name) {
                    return Ok(Self {
                        shell,
                        name,
                        utils_dir: None,
                    });
                }
            }
            Err("no dash or sh on PATH".to_owned())
        }
    }

    /// Resolve one NAMED base-dialect floor binary (`two-binary-floor`: `posh 0.14.1` ∩
    /// `dash 0.5.12`) for the differential lane, or say why it is unavailable.
    ///
    /// It sits beside [`Self::find`] rather than in the e2e runner because `one-shell-answer` is
    /// about there being ONE place that answers "where is a shell", and a second copy is how the
    /// first one silently rotted. What it does NOT share with `find` is the fallback: `find` wants
    /// any POSIX shell and will take `sh`, while the floor lane wants a SPECIFIC binary and an
    /// absence is a refusal, never a substitution — a differential answered by the wrong shell is
    /// worse than one not run.
    ///
    /// Windows resolves through git's own userland exactly as `find` does (never a PATH search:
    /// there, PATH order can hand you `%SystemRoot%\System32\bash.exe`, the WSL launcher). `posh`
    /// is not part of git's userland, so on Windows this lane resolves `dash` alone and the caller
    /// reports the half-floor honestly.
    ///
    /// # Errors
    /// When the named binary is not present in the platform's resolution seat.
    pub fn floor(name: &str) -> Result<PathBuf, String> {
        if cfg!(windows) {
            let dir = git_usr_bin()?;
            let candidate = dir.join(format!("{name}.exe"));
            return candidate
                .is_file()
                .then_some(candidate)
                .ok_or_else(|| format!("git ships no {name}.exe in {}", dir.display()));
        }
        which(name).ok_or_else(|| format!("no {name} on PATH"))
    }

    /// The PATH a child of this shell needs, given the caller's own environment.
    ///
    /// Scoped to one child by construction: the return value goes into a single
    /// `Command::env`, never into the process or the ambient environment. That is what
    /// keeps git's 249 utilities from shadowing `find`, `sort`, `tar` and MSVC's
    /// `link.exe` for everything else the gate runs.
    #[must_use]
    pub fn child_path(&self) -> std::ffi::OsString {
        let ambient = std::env::var_os("PATH").unwrap_or_default();
        match &self.utils_dir {
            Some(dir) => std::env::join_paths(
                std::iter::once(dir.clone()).chain(std::env::split_paths(&ambient)),
            )
            .unwrap_or(ambient),
            None => ambient,
        }
    }
}

/// Git's bundled POSIX userland, derived from the one path git will tell us about.
///
/// `git --exec-path` reports `<root>/mingw64/libexec/git-core` (or `mingw32`); the
/// userland is `<root>/usr/bin`. Derived rather than hardcoded so a non-default install
/// location works, and checked so a layout change fails loudly instead of silently.
fn git_usr_bin() -> Result<PathBuf, String> {
    let out = Command::new("git")
        .arg("--exec-path")
        .output()
        .map_err(|e| format!("could not run git: {e}"))?;
    let exec_path = String::from_utf8_lossy(&out.stdout)
        .trim()
        .replace('\\', "/");
    let root = [
        "/mingw64/libexec/git-core",
        "/mingw32/libexec/git-core",
        "/libexec/git-core",
    ]
    .iter()
    .find_map(|suffix| exec_path.strip_suffix(suffix))
    .ok_or_else(|| format!("unrecognized git layout: {exec_path}"))?;
    // Back to native separators: git answers in forward slashes, and a path printed as
    // `C:/Program Files/Git\usr\bin` in a refusal reads as a bug in the tool reporting it.
    let dir = PathBuf::from(root.replace('/', std::path::MAIN_SEPARATOR_STR))
        .join("usr")
        .join("bin");
    if dir.is_dir() {
        Ok(dir)
    } else {
        Err(format!(
            "git's POSIX userland is missing: {}",
            dir.display()
        ))
    }
}

/// Resolve `name` on `PATH`, honouring the platform's executable extensions.
#[must_use]
pub fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exts: Vec<String> = std::env::var("PATHEXT")
        .map(|raw| raw.split(';').map(str::to_ascii_lowercase).collect())
        .unwrap_or_default();
    std::env::split_paths(&path).find_map(|dir| {
        let bare = dir.join(name);
        if bare.is_file() {
            return Some(bare);
        }
        exts.iter()
            .map(|ext| dir.join(format!("{name}{ext}")))
            .find(|p| p.is_file())
    })
}

/// The worktree root, from this crate's compile-time location (`<root>/spike/crates/…`).
#[must_use]
pub fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap_or(Path::new("."))
}
