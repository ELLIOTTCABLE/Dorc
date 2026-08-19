//! The resolution seat's two properties: a `-C` world confines every path, and nothing outside the
//! seat can name an absolute anchor to escape it with.
//!
//! The second is what makes the first worth having. A root flag honoured by three of four
//! consumers is worse than no flag at all, because the write that ignores it lands somewhere
//! nobody thinks to look — under `cargo test`, that is a developer's in-progress loom edit.

#![expect(
    clippy::expect_used,
    reason = "a gate over this crate's own sources; the no-panic lints guard untrusted input"
)]

use std::path::{Path, PathBuf};

use dorc_loom::Roots;

/// Every location the seat answers, so a new one has to be added HERE to be confined — and the
/// lexical gate below is what says a new one cannot be added anywhere else.
fn every_location(roots: &Roots) -> Vec<(&'static str, PathBuf)> {
    vec![
        ("corpus", roots.corpus()),
        ("catalog lock", roots.catalog_lock()),
        ("arrangement lock", roots.arrangement_lock()),
        ("staging root", roots.staging_root()),
    ]
}

fn scratch(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("dorc-loom-roots-{label}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch dir");
    path
}

/// This crate's production sources — `#[cfg(test)]` modules truncated away, because a test may
/// legitimately name the tree it is running inside.
fn production_sources() -> Vec<(PathBuf, String)> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = Vec::new();
    let mut pending = vec![src];
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(&dir).expect("the crate's sources are readable") {
            let path = entry.expect("a readable dir entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|kind| kind == "rs") {
                let text = std::fs::read_to_string(&path).expect("a source file is UTF-8");
                let production = text
                    .split_once("#[cfg(test)]")
                    .map_or(text.clone(), |(before, _)| before.to_owned());
                found.push((path, production));
            }
        }
    }
    found.sort_by(|left, right| left.0.cmp(&right.0));
    assert!(!found.is_empty(), "the source walk found nothing to check");
    found
}

/// A `-C` world confines every location, so the Part-2 write-path tests can drive a real publish
/// without a real corpus being reachable from it.
#[test]
fn every_location_a_given_root_answers_is_inside_it() {
    let base = scratch("confined");
    let roots = Roots::at(base.to_str().expect("the scratch path is UTF-8")).expect("-C resolves");
    for (what, path) in every_location(&roots) {
        assert!(
            path.starts_with(roots.base()),
            "the {what} escaped the given root: {}",
            path.display()
        );
    }
    let _ = std::fs::remove_dir_all(&base);
}

/// An absent `-C` is the tree this binary was built in, and that tree is the workspace root — the
/// same answer four separate sites used to derive for themselves.
#[test]
fn an_absent_root_is_the_tree_this_was_built_in() {
    let built = Roots::resolve(None).expect("the built-in anchor resolves");
    assert_eq!(built, Roots::built_in().expect("the built-in anchor"));
    assert!(
        built.corpus().is_dir() && built.catalog_lock().is_file(),
        "the default world is the committed one: {}",
        built.base().display()
    );
}

/// The gate that makes the confinement above a property of the CRATE rather than of one type: an
/// absolute anchor spelled anywhere else is a location that would ignore `-C` in silence.
///
/// Lexical because the property is "no other seat can even spell it", which no type bound
/// expresses — the same shape as `aid`'s `spanless_mint_allow_list_is_exact`.
#[test]
fn only_the_resolution_seat_names_an_absolute_anchor() {
    const ANCHOR: &str = "CARGO_MANIFEST_DIR";
    for (path, production) in production_sources() {
        let seat = path.file_name().is_some_and(|name| name == "roots.rs");
        let count = production.matches(ANCHOR).count();
        assert_eq!(
            count,
            usize::from(seat),
            "{} names `{ANCHOR}` {count} time(s); every location resolves through `Roots`, whose \
             one anchor is its own default. A second one silently ignores -C",
            path.display()
        );
    }
}

/// `-C` names a directory, and a typo that resolves to nothing is told so rather than answered
/// with an empty corpus — the failure mode a root flag adds if it is quiet about it.
#[test]
fn a_root_that_is_not_a_directory_refuses() {
    let base = scratch("not-a-dir");
    let file = base.join("a-file");
    std::fs::write(&file, "").expect("write the file");
    let refusal = Roots::at(file.to_str().expect("the scratch path is UTF-8"))
        .expect_err("a file is not a world");
    assert!(refusal.contains("-C"), "{refusal}");

    let absent = Roots::at(
        base.join("nowhere")
            .to_str()
            .expect("the scratch path is UTF-8"),
    )
    .expect_err("an absent directory is not a world");
    assert!(absent.contains("-C"), "{absent}");
    let _ = std::fs::remove_dir_all(&base);
}
