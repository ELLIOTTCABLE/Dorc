//! `sh` fences: load files and books, read line by line (`notes/30Y` § 2.1, § 2.4). Words come
//! from the product's own parser, so a map line and its command split the same way.

use dorc_syntax::{Ast, NodeKind};

use super::alloy::is_plain_identifier;
use super::{Finding, Lint};

/// One concrete command line of a book and everything written under it.
#[derive(Debug, Clone)]
pub(super) struct Command {
    /// 1-based among the book's concrete lines.
    pub(super) number: usize,
    pub(super) line: usize,
    pub(super) text: String,
    /// Whether a `#}` line followed it.
    pub(super) mapped: bool,
    /// `(literal, component)` pairs, one per word.
    pub(super) words: Vec<(String, Component)>,
    pub(super) stmts: Vec<(usize, String)>,
}

/// What a map component says about the literal beneath it (`notes/30Y` § 2.1): a bare component
/// names it, a braced one classes it and leaves it named after itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Component {
    Name(String),
    Class(String),
}

#[derive(Debug, Clone)]
pub(super) enum Fence {
    Load {
        name: String,
        line: usize,
        stems: Vec<(String, usize)>,
    },
    Book {
        name: String,
        line: usize,
        loads: Vec<(String, usize)>,
        commands: Vec<Command>,
    },
}

/// Read one `sh` fence whose body lines start at file line `first_line`.
pub(super) fn read(
    file: &str,
    first_line: usize,
    lines: &[&str],
    findings: &mut Vec<Finding>,
) -> Option<Fence> {
    let mut rows = lines
        .iter()
        .enumerate()
        .map(|(i, l)| (first_line.saturating_add(i), *l));
    let (header_line, header) = rows.next()?;
    let Some(name) = header
        .strip_prefix("# ")
        .and_then(|h| h.strip_suffix(".sh"))
        .filter(|n| is_plain_identifier(n))
    else {
        findings.push(Finding::new(Lint::FenceHeader, file, header_line).with("header", header));
        return None;
    };

    let mut loads = Vec::new();
    let mut commands: Vec<Command> = Vec::new();
    let mut pending: Option<(usize, String)> = None;
    for (line, raw) in rows {
        let trimmed = raw.trim();
        if let Some((start, mut joined)) = pending.take() {
            joined.push(' ');
            joined.push_str(trimmed.strip_suffix('\\').unwrap_or(trimmed).trim());
            if trimmed.ends_with('\\') {
                pending = Some((start, joined));
            } else {
                commands.push(command(commands.len().saturating_add(1), start, joined));
            }
            continue;
        }
        if trimmed.is_empty() {
            continue;
        }
        if let Some(stem) = trimmed
            .strip_prefix(". ./")
            .and_then(|s| s.strip_suffix(".sh"))
        {
            loads.push((stem.to_owned(), line));
        } else if let Some(map) = trimmed.strip_prefix("#}") {
            attach_map(file, line, map, commands.last_mut(), findings);
        } else if let Some(stmt) = trimmed.strip_prefix("#=") {
            match commands.last_mut() {
                Some(cmd) if cmd.mapped => cmd.stmts.push((line, stmt.trim().to_owned())),
                _ => findings.push(
                    Finding::new(Lint::MapLineFollowsCommand, file, line).with("text", trimmed),
                ),
            }
        } else if trimmed.starts_with('#') {
            findings.push(Finding::new(Lint::NoStrayComments, file, line).with("text", trimmed));
        } else if let Some(open) = trimmed.strip_suffix('\\') {
            pending = Some((line, open.trim().to_owned()));
        } else {
            commands.push(command(
                commands.len().saturating_add(1),
                line,
                trimmed.to_owned(),
            ));
        }
    }
    if let Some((start, joined)) = pending {
        commands.push(command(commands.len().saturating_add(1), start, joined));
    }
    for cmd in commands.iter().filter(|c| !c.mapped) {
        findings.push(
            Finding::new(Lint::MapLineFollowsCommand, file, cmd.line).with("text", &cmd.text),
        );
    }

    Some(if commands.is_empty() {
        Fence::Load {
            name: name.to_owned(),
            line: header_line,
            stems: loads,
        }
    } else {
        Fence::Book {
            name: name.to_owned(),
            line: header_line,
            loads,
            commands,
        }
    })
}

fn command(number: usize, line: usize, text: String) -> Command {
    Command {
        number,
        line,
        text,
        mapped: false,
        words: Vec::new(),
        stmts: Vec::new(),
    }
}

/// Pair a `#}` line with the command above it, word for word.
fn attach_map(
    file: &str,
    line: usize,
    map: &str,
    cmd: Option<&mut Command>,
    findings: &mut Vec<Finding>,
) {
    let Some(cmd) = cmd.filter(|c| !c.mapped) else {
        findings.push(
            Finding::new(Lint::MapLineFollowsCommand, file, line).with("text", format!("#}}{map}")),
        );
        return;
    };
    cmd.mapped = true;
    let literals = words(&cmd.text);
    let components = words(map);
    if literals.len() != components.len() {
        findings.push(
            Finding::new(Lint::MapWordCount, file, line)
                .with("command_words", literals.len().to_string())
                .with("map_components", components.len().to_string()),
        );
        return;
    }
    cmd.words = literals
        .into_iter()
        .zip(components)
        .map(|(literal, raw)| {
            let component = match raw.strip_prefix('{').and_then(|c| c.strip_suffix('}')) {
                Some(class) => Component::Class(class.to_owned()),
                None => Component::Name(raw),
            };
            (literal, component)
        })
        .collect();
}

/// A line's words as the syntax crate splits it: every simple command's assignments, words, and
/// redirections in source order, a redirection being one word with its target.
pub(super) fn words(line: &str) -> Vec<String> {
    let ast = dorc_syntax::parse(line).value;
    let mut spans = Vec::new();
    collect(&ast, &ast.node(ast.root()).kind, &mut spans);
    spans.sort_unstable();
    spans
        .into_iter()
        .filter_map(|(lo, hi)| line.get(lo..hi))
        .map(str::to_owned)
        .collect()
}

/// Walks command structure only, never into a word: a substitution's inner commands carry spans
/// relative to their own body (`syntax/CLAUDE.md` `tn-coarse-subst-provenance`).
fn collect(ast: &Ast, kind: &NodeKind, out: &mut Vec<(usize, usize)>) {
    let span = |node: &dorc_syntax::Node| {
        let at = |pos: u32| usize::try_from(pos).unwrap_or(usize::MAX);
        (at(node.span.lo.0), at(node.span.hi.0))
    };
    match kind {
        NodeKind::Script { items: children }
        | NodeKind::List { items: children }
        | NodeKind::Pipeline {
            stages: children, ..
        } => {
            for &child in children {
                collect(ast, &ast.node(child).kind, out);
            }
        }
        NodeKind::AndOr { left, right, .. } => {
            collect(ast, &ast.node(*left).kind, out);
            collect(ast, &ast.node(*right).kind, out);
        }
        NodeKind::Subshell { body, redirs } | NodeKind::Group { body, redirs } => {
            collect(ast, &ast.node(*body).kind, out);
            out.extend(redirs.iter().map(|&r| span(ast.node(r))));
        }
        NodeKind::Simple {
            assigns,
            words,
            redirs,
        } => {
            out.extend(
                assigns
                    .iter()
                    .chain(words)
                    .chain(redirs)
                    .map(|&w| span(ast.node(w))),
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::words;

    #[test]
    fn a_redirection_is_one_word_so_the_strawman_map_pairs_one_to_one() {
        // `30Ya-strawman-3`'s hork line: six map components over six command words.
        assert_eq!(
            words("hork tune --profile web >>/var/log/hork.log 2>&1"),
            vec![
                "hork",
                "tune",
                "--profile",
                "web",
                ">>/var/log/hork.log",
                "2>&1"
            ]
        );
        assert_eq!(
            words("hork tune dash_dash_profile web append_hork_log stderr_to_stdout").len(),
            6
        );
    }

    #[test]
    fn a_quoted_string_is_one_word_and_a_braced_component_survives() {
        assert_eq!(
            words("stat -c '%i %d' \"a b\""),
            vec!["stat", "-c", "'%i %d'", "\"a b\""]
        );
        assert_eq!(words("frob {gizmo}"), vec!["frob", "{gizmo}"]);
    }
}
