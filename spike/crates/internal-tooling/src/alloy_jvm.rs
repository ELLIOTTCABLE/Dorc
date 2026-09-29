pub(crate) mod adapter;
pub(crate) mod runner;

use std::path::{Path, PathBuf};

use crate::json::{Json, Value};
use crate::sha256;

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
        let cache = internal_tooling::target_dir()
            .join("alloy")
            .join("jvm-paths.json");
        let pins = pins();
        if std::env::var_os(JAVA_HOME_ENV).is_none()
            && let Some(jvm) = std::fs::read_to_string(&cache)
                .ok()
                .and_then(|t| Value::parse(&t))
                .filter(|v| v.str("pins") == pins.as_deref())
                .and_then(|v| {
                    Some(Self {
                        java: PathBuf::from(v.str("java")?),
                        javac: PathBuf::from(v.str("javac")?),
                        jar: PathBuf::from(v.str("jar")?),
                    })
                })
                .filter(|j| j.java.is_file() && j.javac.is_file() && j.jar.is_file())
        {
            return Ok(jvm);
        }
        let jvm = Self::resolve()?;
        if let Some(pins) = pins {
            let record = Json::obj([
                ("pins", Json::str(pins)),
                ("java", Json::str(jvm.java.display().to_string())),
                ("javac", Json::str(jvm.javac.display().to_string())),
                ("jar", Json::str(jvm.jar.display().to_string())),
            ]);
            let _ = std::fs::create_dir_all(cache.parent().unwrap_or(&cache));
            let _ = std::fs::write(&cache, record.render());
        }
        Ok(jvm)
    }

    fn resolve() -> Result<Self, String> {
        let var = |name: &str, tool: &'static str| {
            std::env::var_os(name).map_or_else(
                || std::thread::spawn(move || mise_where(tool)),
                |v| std::thread::spawn(move || Ok(PathBuf::from(v))),
            )
        };
        let (home, dist) = (var(JAVA_HOME_ENV, "java"), var(JAR_ENV, "http:alloy"));
        let joined = |h: std::thread::JoinHandle<Result<PathBuf, String>>| {
            h.join()
                .unwrap_or_else(|_| Err("mise where panicked".to_owned()))
        };
        let home = joined(home)?;
        let dist = joined(dist)?;
        let exe = |name: &str| {
            home.join("bin")
                .join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
        };
        let jar = if dist.is_dir() {
            dist.join("org.alloytools.alloy.dist.jar")
        } else {
            dist
        };
        Ok(Self {
            java: exe("java"),
            javac: exe("javac"),
            jar,
        })
    }
}

fn pins() -> Option<String> {
    let text = std::fs::read_to_string(dorc_testbed::repo_root().join("mise.toml")).ok()?;
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim_start)
        .filter(|l| {
            l.starts_with("java ") || l.starts_with("java=") || l.starts_with("\"http:alloy\"")
        })
        .collect();
    (lines.len() == 2).then(|| lines.join("\n"))
}

fn mise_where(tool: &str) -> Result<PathBuf, String> {
    let out = std::process::Command::new("mise")
        .args(["where", tool])
        .current_dir(dorc_testbed::repo_root())
        .output()
        .map_err(|e| format!("mise where {tool}: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "mise where {tool}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(PathBuf::from(String::from_utf8_lossy(&out.stdout).trim()))
}

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
