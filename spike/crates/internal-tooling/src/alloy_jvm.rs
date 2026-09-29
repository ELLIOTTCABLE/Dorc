//! Alloy 6 behind one Java adapter (`spike/verify/alloy/AlloyAdapter.java`, `notes/30Yf` § 9):
//! the adapter answers four verbs and decides nothing; every budget, key, and verdict rule is
//! here or in `assay`. `runner` is `mise run alloy`, the author's direct route to the solver.

pub(crate) mod adapter;
pub(crate) mod runner;

use std::path::{Path, PathBuf};

use crate::sha256;

/// The pinned JDK and Alloy jar, exported by the `alloy` and `assay` tasks from `mise where`.
#[derive(Debug, Clone)]
pub(crate) struct Jvm {
    java: PathBuf,
    javac: PathBuf,
    jar: PathBuf,
}

const JAVA_HOME_ENV: &str = "ALLOY_JAVA_HOME";
const JAR_ENV: &str = "ALLOY_JAR";

impl Jvm {
    pub(crate) fn from_env() -> Result<Self, String> {
        let var = |name: &str| {
            std::env::var_os(name).map(PathBuf::from).ok_or_else(|| {
                format!("{name} is unset; run through `mise run alloy` or `mise run assay`")
            })
        };
        let home = var(JAVA_HOME_ENV)?;
        let exe = |name: &str| {
            home.join("bin")
                .join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
        };
        Ok(Self {
            java: exe("java"),
            javac: exe("javac"),
            jar: var(JAR_ENV)?,
        })
    }
}

/// The adapter's source, and its digest (a key input: an adapter change is a semantics change).
pub(crate) fn adapter_source() -> PathBuf {
    dorc_testbed::repo_root()
        .join("spike")
        .join("verify")
        .join("alloy")
        .join("AlloyAdapter.java")
}

pub(crate) fn adapter_digest() -> Result<String, String> {
    let path = adapter_source();
    std::fs::read(&path)
        .map(|bytes| sha256::hex(&bytes))
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// The jar's identity is the checksum mise enforces at install, read from the root `mise.toml`
/// rather than re-hashed on every run.
pub(crate) fn jar_digest() -> Result<String, String> {
    let path = dorc_testbed::repo_root().join("mise.toml");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    jar_checksum(&text)
        .ok_or_else(|| format!("{}: no pinned checksum for http:alloy", path.display()))
}

fn jar_checksum(mise_toml: &str) -> Option<String> {
    let line = mise_toml
        .lines()
        .find(|l| l.trim_start().starts_with("\"http:alloy\""))?;
    let at = line.find("sha256:")?;
    let hex: String = line
        .get(at.saturating_add(7)..)?
        .chars()
        .take_while(char::is_ascii_hexdigit)
        .collect();
    (hex.len() == 64).then_some(hex)
}

/// `.als` files a path stands for: a directory stands for its own, sorted.
pub(crate) fn expand(path: &Path) -> Vec<PathBuf> {
    if !path.is_dir() {
        return vec![path.to_path_buf()];
    }
    let mut found: Vec<PathBuf> = std::fs::read_dir(path)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "als"))
        .collect();
    found.sort();
    found
}

/// The platform a measurement was made on (`notes/30Yf` § 3: recorded, never keyed).
pub(crate) fn platform() -> &'static str {
    match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "macos",
        _ => "linux",
    }
}

#[cfg(test)]
mod tests {
    use super::jar_checksum;

    #[test]
    fn the_jar_digest_is_the_pin_mise_enforces() {
        let toml = "\"http:alloy\" = { version = \"6.2.0\", url = \"x\", checksum = \"sha256:6b8c1cb5bc93bedfc7c61435c4e1ab6e688a242dc702a394628d9a9801edb78d\" }\n";
        assert_eq!(
            jar_checksum(toml).as_deref(),
            Some("6b8c1cb5bc93bedfc7c61435c4e1ab6e688a242dc702a394628d9a9801edb78d")
        );
        assert_eq!(
            jar_checksum("\"http:alloy\" = { checksum = \"sha256:abc\" }"),
            None
        );
    }
}
