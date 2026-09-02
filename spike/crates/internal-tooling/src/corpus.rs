//! The prose corpus's own file walk and docID-token primitives, shared by the `docids` dangle
//! lint and the `slugs` index so the two cannot drift on WHICH files are the corpus or on what a
//! docID looks like. A naive recursive walk is wrong here: the primary checkout carries sibling
//! worktrees under `.tmp/` and `.claude/`, so the walk is anchored at `Research/`, `spike/**/CLAUDE.md`
//! and the root `*.md` files alone, and a `quarantine-DO-NOT-READ` directory is harvested for its
//! filenames but never opened for content.

use std::path::{Path, PathBuf};

/// Never descended into for content, wherever it appears; its filenames alone answer existence.
/// Quarantined material is off-limits to content reads, and neither instrument is the exception.
pub(crate) const QUARANTINE_DIR: &str = "quarantine-DO-NOT-READ";

/// One character of a line, by index — `None` past the end rather than a panic.
pub(crate) fn ch(chars: &[char], i: usize) -> Option<char> {
    chars.get(i).copied()
}

pub(crate) fn starts_with(chars: &[char], i: usize, want: &str) -> bool {
    want.chars()
        .enumerate()
        .all(|(off, c)| ch(chars, i.saturating_add(off)) == Some(c))
}

/// A docID token: 1–3 digits then up to 2 ASCII letters (`307`, `306b`, `27Xf`).
pub(crate) fn take_id(chars: &[char], i: usize) -> Option<(String, usize)> {
    let mut end = i;
    let mut digits = 0_u8;
    while digits < 3 && ch(chars, end).is_some_and(|c| c.is_ascii_digit()) {
        end = end.saturating_add(1);
        digits = digits.saturating_add(1);
    }
    if digits == 0 {
        return None;
    }
    let mut letters = 0_u8;
    while letters < 2 && ch(chars, end).is_some_and(|c| c.is_ascii_alphabetic()) {
        end = end.saturating_add(1);
        letters = letters.saturating_add(1);
    }
    Some((chars.get(i..end)?.iter().collect(), end))
}

/// The ID a corpus filename encodes: everything before the first hyphen, if it starts with a digit
/// (`271-block-….md` → `271`). Root docs and steering files have no such prefix and fall to a stem.
pub(crate) fn id_of(name: &str) -> Option<&str> {
    let id = name.split('-').next()?;
    id.starts_with(|c: char| c.is_ascii_digit()).then_some(id)
}

/// A Syncthing conflict copy (`271-foo.sync-conflict-20260101-….md`), which lands beside the real
/// file in this synced tree. Never a corpus document — indexing one would mint duplicate definition
/// sites for the same slugs (and a phantom dangling docID for `docids`).
fn is_sync_conflict(name: &str) -> bool {
    name.contains(".sync-conflict-")
}

fn dir_entries(dir: &Path) -> Vec<(String, bool)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            if is_sync_conflict(&name) {
                return None;
            }
            Some((name, entry.file_type().is_ok_and(|kind| kind.is_dir())))
        })
        .collect()
}

/// Every entry name under `dir`, files and directories alike, all the way down — gathered by NAME
/// ONLY, so nothing under a quarantine is ever opened.
pub(crate) fn names_under(dir: &Path, out: &mut Vec<String>) {
    for (name, is_dir) in dir_entries(dir) {
        out.push(name.clone());
        if is_dir {
            names_under(&dir.join(name), out);
        }
    }
}

/// Every child ID under `dir`, files and directories alike (directory names count: `plans/deferred/078-…`
/// and `notes/28G-…/` are both real corpus documents).
pub(crate) fn doc_ids(dir: &Path, out: &mut std::collections::BTreeSet<String>) {
    for (name, is_dir) in dir_entries(dir) {
        if let Some(id) = id_of(&name) {
            out.insert(id.to_owned());
        }
        if is_dir {
            doc_ids(&dir.join(name), out);
        }
    }
}

/// Markdown under `dir`. A quarantine is harvested for names (into `quarantined`) and never
/// descended into for content, wherever in the tree it turns up.
fn markdown_under(dir: &Path, files: &mut Vec<PathBuf>, quarantined: &mut Vec<String>) {
    for (name, is_dir) in dir_entries(dir) {
        let path = dir.join(&name);
        if is_dir {
            if name == QUARANTINE_DIR {
                names_under(&path, quarantined);
            } else {
                markdown_under(&path, files, quarantined);
            }
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
}

/// The three scanned surfaces, sorted: the `Research/` corpus, the `spike/**/CLAUDE.md` steering
/// files, and the root docs. `quarantined` receives the quarantine filenames the walk skipped.
pub(crate) fn scanned(root: &Path, quarantined: &mut Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    markdown_under(&root.join("Research"), &mut files, quarantined);
    let mut spike = Vec::new();
    markdown_under(&root.join("spike"), &mut spike, quarantined);
    files.extend(
        spike
            .into_iter()
            .filter(|path| path.file_name().is_some_and(|name| name == "CLAUDE.md")),
    );
    files.extend(
        dir_entries(root)
            .iter()
            .map(|(name, _)| root.join(name))
            .filter(|path| path.extension().is_some_and(|ext| ext == "md")),
    );
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::is_sync_conflict;

    #[test]
    fn it_skips_syncthing_conflict_copies() {
        assert!(is_sync_conflict(
            "271-block-settle-rulings-ledger.sync-conflict-20260101-123456-ABCDEF.md"
        ));
        assert!(!is_sync_conflict("271-block-settle-rulings-ledger.md"));
    }
}
