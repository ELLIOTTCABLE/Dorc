//! assay's self-test (`notes/30Y` § 3): the Dorc-agnostic fixture compiles to exactly the modules
//! under `assay_fixture/expected/`, and a join-key conflict refuses. No solver runs here.
#![expect(
    clippy::expect_used,
    reason = "a test asserts by panicking, naming the file that differed"
)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const MODULE_SUFFIX: &str = ".als";

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assay_fixture")
}

fn assay(spec: &Path, out: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_internal-tooling"))
        .arg("assay")
        .arg(spec)
        .arg("--out")
        .arg(out)
        .env_remove("MISE_ORIGINAL_CWD")
        .output()
        .expect("internal-tooling should spawn")
}

fn fresh_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn module_files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("directory should list")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(MODULE_SUFFIX))
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .collect();
    names.sort();
    names
}

/// A line diff good enough to point at the first divergence; the regeneration command is the real
/// remedy, reviewed as a git diff.
fn diff(want: &str, got: &str) -> String {
    let (w, g): (Vec<&str>, Vec<&str>) = (want.lines().collect(), got.lines().collect());
    let mut out = String::new();
    for (number, i) in (1..).zip(0..w.len().max(g.len())) {
        match (w.get(i), g.get(i)) {
            (Some(a), Some(b)) if a == b => {}
            (a, b) => {
                let _ = writeln!(out, "@@ line {number}");
                if let Some(a) = a {
                    let _ = writeln!(out, "-{a}");
                }
                if let Some(b) = b {
                    let _ = writeln!(out, "+{b}");
                }
            }
        }
    }
    out
}

#[test]
fn the_fixture_compiles_to_exactly_the_expected_modules() {
    // Every mechanism the compiler has is exercised by the fixture, so a changed byte in any
    // generated module is a changed behaviour, reviewed here as a diff rather than found later
    // as an Alloy error in someone's spec.
    let out = fresh_dir("assay_fixture");
    let spec = fixture().join("widgets.md");
    let run = assay(&spec, &out);
    assert!(
        run.status.success(),
        "assay refused the fixture:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let expected = fixture().join("expected");
    let regenerate = "mise run assay -- spike/crates/internal-tooling/tests/assay_fixture/widgets.md \
                      --out spike/crates/internal-tooling/tests/assay_fixture/expected";
    assert_eq!(
        module_files(&out),
        module_files(&expected),
        "module set differs; regenerate with `{regenerate}`"
    );
    for name in module_files(&expected) {
        let want =
            std::fs::read_to_string(expected.join(&name)).expect("expected module should read");
        let got = std::fs::read_to_string(out.join(&name)).expect("generated module should read");
        assert!(
            want == got,
            "{name} differs from expected:\n{}\nregenerate with `{regenerate}` and review the git diff",
            diff(&want, &got)
        );
    }
}

#[test]
fn one_literal_under_two_names_refuses_before_writing_anything() {
    // The join key (`30Y` § 2.1): a literal is one atom, so two bare names for it would silently
    // mint two words for one thing.
    let out = fresh_dir("assay_join_key_conflict");
    let run = assay(
        &fixture().join("negative").join("join_key_conflict.md"),
        &out,
    );
    let report = String::from_utf8_lossy(&run.stdout);
    assert_eq!(run.status.code(), Some(2), "{report}");
    let json: serde_json::Value = serde_json::from_str(&report).expect("the report should be JSON");
    let hits = json["lints"]["join-key-coherence"]
        .as_array()
        .expect("the lint should be listed");
    assert_eq!(hits.len(), 1, "{report}");
    assert_eq!(hits[0]["literal"], "/tmp/sprocket", "{report}");
    assert!(!out.exists(), "a refusal must write no module");
}

#[test]
fn words_name_themselves_as_the_human_pictured() {
    // The ruling, verbatim (`30Yc` § 6): a bare component names its literal, a repeated one
    // leaves it named after itself, and a braced one classes it and leaves it self-named.
    let out = fresh_dir("assay_self_naming");
    let run = assay(&fixture().join("self_naming.md"), &out);
    let report = String::from_utf8_lossy(&run.stdout);
    assert!(run.status.success(), "{report}");
    let json: serde_json::Value = serde_json::from_str(&report).expect("the report should be JSON");
    let words: Vec<(String, String)> = json["words"]
        .as_array()
        .expect("the words table should be listed")
        .iter()
        .filter(|w| !w["literal"].is_null())
        .map(|w| (w["name"].to_string(), w["literal"].to_string()))
        .collect();
    let pair = |n: &str, l: &str| (format!("{n:?}"), format!("{l:?}"));
    assert_eq!(
        words,
        vec![
            pair("widget", "foo"),
            pair("bar", "bar"),
            pair("baz", "baz"),
            pair("bum", "bum"),
            pair("assay_colon", ":"),
        ],
        "{report}"
    );
    assert_eq!(
        json["classes"]["c"],
        serde_json::json!(["baz", "bum"]),
        "{report}"
    );
}

#[test]
fn a_self_named_literal_whose_atom_another_literal_holds_refuses() {
    // `-c` names itself, and its atom `w__dash_c` joins the key space every bare name lives in,
    // where `-d` already holds it: one name over two literals.
    let out = fresh_dir("assay_self_name_conflicts");
    let run = assay(
        &fixture().join("negative").join("self_name_conflicts.md"),
        &out,
    );
    let report = String::from_utf8_lossy(&run.stdout);
    assert_eq!(run.status.code(), Some(2), "{report}");
    let json: serde_json::Value = serde_json::from_str(&report).expect("the report should be JSON");
    let clash = json["lints"]["join-key-coherence"]
        .as_array()
        .expect("the lint should be listed");
    assert_eq!(clash.len(), 1, "{report}");
    assert_eq!(clash[0]["name"], "w__dash_c", "{report}");
    assert!(!out.exists(), "a refusal must write no module");
}
