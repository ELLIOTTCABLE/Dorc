//! One grammar for a session `$` line, backed by our OWN parser
//! (`30X:loom-in-process-driver-is-a-closed-grammar`, `30X:dogfood-the-sh-engine`).
//!
//! A loom's replay is a POSIX shell session (`30X:loom-is-a-shell-session`); the process driver
//! hands each `$` line to a real `sh`, and the in-process driver must understand the same lines
//! well enough to run the ones it can express and decline the rest with a typed reason. This reads
//! a line's AST through `dorc_syntax::parse` — the ONE parse entry — and yields a typed session
//! command, so the in-process driver's understanding of a line comes from our own parser, never a
//! hand grammar.
//!
//! errorloom's `ReplayCommand::parse` stays the OUTER gate (`30Xa:rul-minimize-errorloom-changes`):
//! a line it refuses never reaches this reading at all. Where BOTH accept a line but the two
//! readings disagree on argv, stdin routing, or stdout routing, that is a decline with the
//! disagreement as its reason ([`SessionReading::disagrees_with`]), never a silent preference. The
//! expressible set only ever SHRINKS (`30X` §9); no roster or lexical meta-test polices it — this
//! enum IS the set.

use dorc_syntax::{Ast, NodeKind, RedirOp, RedirTarget, WordPart};

use crate::consumer::LoomDecline;

/// A session line's typed reading: an expressible command, or the reason the in-process driver
/// cannot express it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SessionReading {
    /// A line the in-process driver runs (`dorc`/`dorc-loom`/`dorc-sh`, `export`, `cd`, and the two
    /// errorloom builtins `echo $?` / `cat <literal>`).
    Line(SessionLine),
    /// A line only the shell can express; why.
    Decline(LoomDecline),
}

/// One expressible session line: its head, its statically-fixed argv, and its stdin/stdout routing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SessionLine {
    /// What the line IS, structurally recognized (`30X:session-grammar-is-dorc-syntax`).
    pub head: SessionHead,
    /// The simple command's argv, every word statically fixed (`const_literal_text`; `SingleQuoted`
    /// counts) plus the ONE `echo $?` shape rendered as the literal `$?`.
    pub argv: Vec<String>,
    /// The stdin routing a `< target` (fd 0) selects, or [`SessionInput::Inherit`] with no `<`.
    pub input: SessionInput,
    /// The stdout/stderr routing the ruled redirect set selects.
    pub output: OutputRouting,
}

/// The structurally-recognized head of a session line.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SessionHead {
    /// A `dorc` / `dorc-loom` / `dorc-sh` invocation — the engine runs it.
    Invocation,
    /// `export NAME=word` / `export NAME` — a session environment mutation (dash semantics:
    /// `export NAME=word` sets and marks; `export NAME` marks; a shell-local `NAME=word` never
    /// reaches a child).
    Export(ExportOp),
    /// `cd <literal>` — a session cwd move.
    Cd(String),
    /// `echo $?` — the previous block's status; errorloom's builtin renders it.
    EchoStatus,
    /// `cat <literal path>` — errorloom's builtin.
    Cat(String),
}

/// An `export` operand: `NAME=word` (set and mark) or a bare `NAME` (mark only).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExportOp {
    /// `export NAME=word`.
    Assign {
        /// The variable name (a plain POSIX NAME).
        name: String,
        /// The value word (statically fixed).
        value: String,
    },
    /// `export NAME`.
    Mark {
        /// The variable name (a plain POSIX NAME).
        name: String,
    },
}

/// A session line's stdin routing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SessionInput {
    /// No `<` redirect — stdin is the session's own (the framed controller stream, in-process).
    Inherit,
    /// `< /dev/null` — an empty stdin.
    Null,
    /// `< path` — the file at `path`, resolved through the session cwd by the caller.
    File(String),
}

/// Where the ruled output redirects send each stream: `> /dev/null` discards stdout, `> <file>`
/// captures it to a case-relative file (errorloom's own `RoutingPlan` writes it, and a later
/// `cat <file>` reads it back), and `2>/dev/null` discards stderr. `2>&1` (stderr joins stdout)
/// needs no field — the in-process render already interleaves both streams in engine order, which is
/// exactly what `2>&1` produces.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct OutputRouting {
    /// `> /dev/null` — the block's stdout is discarded.
    pub stdout_to_null: bool,
    /// `> <file>` — the block's stdout is captured to this case-relative file (errorloom routes it;
    /// the driver just emits stdout un-suppressed for that routing to reach).
    pub stdout_to_file: Option<String>,
    /// `2>/dev/null` — the block's stderr is discarded.
    pub stderr_to_null: bool,
}

impl OutputRouting {
    /// Whether stdout no longer reaches the terminal — errorloom's `To{File}`/`To{Null}` for stdout,
    /// as seen through its public `stdout_is_terminal()`. The cross-check requires this to agree with
    /// errorloom before a `>`-bearing line runs in-process.
    fn stdout_redirected(&self) -> bool {
        self.stdout_to_null || self.stdout_to_file.is_some()
    }
}

impl SessionReading {
    /// Read a `$` line through our own parser.
    #[must_use]
    pub fn read(src: &str) -> Self {
        let carrier = dorc_syntax::parse(src);
        if carrier.has_errors() {
            return decline(src);
        }
        let ast = &carrier.value;
        let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
            return decline(src);
        };
        let [item] = items.as_slice() else {
            return decline(src);
        };
        let NodeKind::Simple {
            assigns,
            words,
            redirs,
        } = &ast.node(*item).kind
        else {
            return decline(src);
        };
        if !assigns.is_empty() {
            return decline(src);
        }
        let Some(argv) = literal_argv(ast, words) else {
            return decline(src);
        };
        let Some((input, output)) = routing(ast, redirs) else {
            return decline(src);
        };
        match head(&argv, src) {
            Ok(head) => Self::Line(SessionLine {
                head,
                argv,
                input,
                output,
            }),
            Err(reason) => Self::Decline(reason),
        }
    }

    /// Whether this reading DISAGREES with errorloom's on the fields both can see: argv, the stdin
    /// target, and whether stdout still points at the terminal. A disagreement is a decline
    /// (`30Xa:rul-minimize-errorloom-changes`): the two grammars must read one accepted line the
    /// same way or the line is not run in-process. `stdout_is_terminal` is errorloom's own reading of
    /// its `OutputRedirection::To{File}`/`To{Null}` on stdout, so comparing it against our
    /// [`OutputRouting::stdout_redirected`] cross-checks a `> <file>` capture against errorloom's.
    #[must_use]
    pub fn disagrees_with(
        &self,
        argv: &[String],
        input: Option<&errorloom::ReplayInputTarget>,
        stdout_is_terminal: bool,
    ) -> bool {
        let Self::Line(line) = self else {
            return false;
        };
        let input_matches = match (&line.input, input) {
            (SessionInput::Inherit, None)
            | (SessionInput::Null, Some(errorloom::ReplayInputTarget::Null)) => true,
            (SessionInput::File(ours), Some(errorloom::ReplayInputTarget::File(theirs))) => {
                ours == theirs
            }
            _ => false,
        };
        line.argv != argv || !input_matches || line.output.stdout_redirected() == stdout_is_terminal
    }
}

/// The leading Simple's argv words as SOURCE text, up to the first redirect — the ONE reading both
/// drivers classify a block from (`30X:loom-gates-attach-by-kind`; `block-argv-classifier-is-read-only`).
///
/// Source text, not `const_literal_text`, so a word the engine expands (`--artifact-dir=$ARTIFACT_DIR`)
/// still reaches the product arg parser for classification; a line the grammar cannot read as a
/// leading Simple yields no argv (no such line exists in the corpus, and a classifier over an empty
/// argv produces no artifacts, the safe answer).
#[must_use]
pub fn block_argv(src: &str) -> Vec<String> {
    let carrier = dorc_syntax::parse(src);
    let ast = &carrier.value;
    let Some(simple) = first_simple(ast, ast.root()) else {
        return Vec::new();
    };
    let NodeKind::Simple { words, .. } = &ast.node(simple).kind else {
        return Vec::new();
    };
    words
        .iter()
        .map(|word| {
            let span = ast.node(*word).span;
            src.get(span.lo.0 as usize..span.hi.0 as usize)
                .unwrap_or_default()
                .to_owned()
        })
        .collect()
}

fn decline(src: &str) -> SessionReading {
    SessionReading::Decline(LoomDecline::Unexpressible(src.to_owned()))
}

/// The first `Simple` reachable by descending Script/Pipeline/AndOr/List left spines — the command
/// whose argv classifies the block. Returns `None` for a line with no simple command.
fn first_simple(ast: &Ast, id: dorc_core::AstId) -> Option<dorc_core::AstId> {
    match &ast.node(id).kind {
        NodeKind::Simple { .. } => Some(id),
        NodeKind::Script { items } | NodeKind::List { items } => {
            items.first().and_then(|&head| first_simple(ast, head))
        }
        NodeKind::Pipeline { stages, .. } => {
            stages.first().and_then(|&head| first_simple(ast, head))
        }
        NodeKind::AndOr { left, .. } => first_simple(ast, *left),
        _ => None,
    }
}

/// Every argv word statically fixed (`const_literal_text`; `SingleQuoted` counts), plus the ONE
/// `echo $?` shape: a word whose parts are exactly `[Param { name: "?" }]` in `echo`'s argument
/// position, rendered as the literal `$?` so it matches errorloom's argv. Any other non-literal
/// word declines the whole line.
fn literal_argv(ast: &Ast, words: &[dorc_core::AstId]) -> Option<Vec<String>> {
    let mut argv: Vec<String> = Vec::with_capacity(words.len());
    for word in words {
        let NodeKind::Word { parts } = &ast.node(*word).kind else {
            return None;
        };
        if let Some(literal) = dorc_syntax::sem::const_literal_text(parts) {
            argv.push(literal);
        } else if argv.first().map(String::as_str) == Some("echo") && is_dollar_question(parts) {
            argv.push("$?".to_owned());
        } else {
            return None;
        }
    }
    Some(argv)
}

fn is_dollar_question(parts: &[WordPart]) -> bool {
    matches!(parts, [WordPart::Param { name }] if name == "?")
}

/// The stdin/stdout routing the ruled redirect set selects, or `None` for any redirect outside it:
/// `< word` on fd 0 · `> /dev/null` or `> <file>` (fd 1) · `2>/dev/null` · `2>&1`. Append, here-docs,
/// and any other fd algebra decline. A `> <file>` target is captured by errorloom's own routing (the
/// outer gate already vetted its safety), so this reading only has to recognize it and not decline.
fn routing(ast: &Ast, redirs: &[dorc_core::AstId]) -> Option<(SessionInput, OutputRouting)> {
    let mut input = SessionInput::Inherit;
    let mut output = OutputRouting::default();
    for redir in redirs {
        let NodeKind::Redir { op, fd, target } = &ast.node(*redir).kind else {
            return None;
        };
        match (op, fd) {
            (RedirOp::Read, None | Some(0)) => {
                let path = literal_target(ast, target)?;
                input = if path == "/dev/null" {
                    SessionInput::Null
                } else {
                    SessionInput::File(path)
                };
            }
            (RedirOp::Write, None | Some(1)) => match literal_target(ast, target)?.as_str() {
                "/dev/null" => output.stdout_to_null = true,
                path => output.stdout_to_file = Some(path.to_owned()),
            },
            (RedirOp::Write, Some(2)) if literal_target(ast, target)? == "/dev/null" => {
                output.stderr_to_null = true;
            }
            (RedirOp::Dup, Some(2)) if matches!(target, RedirTarget::Fd(1)) => {}
            _ => return None,
        }
    }
    Some((input, output))
}

fn literal_target(ast: &Ast, target: &RedirTarget) -> Option<String> {
    match target {
        RedirTarget::Word(word) => match &ast.node(*word).kind {
            NodeKind::Word { parts } => dorc_syntax::sem::const_literal_text(parts),
            _ => None,
        },
        RedirTarget::HereDoc { .. } | RedirTarget::Fd(_) => None,
    }
}

/// Classify a statically-fixed argv into its head. A RECOGNIZED head whose operands the driver
/// cannot express (`export` with a flag, `cd` without exactly one operand) is `Unexpressible`; a head
/// that is not one of ours (a shell builtin outside the set, or an external tool) is `ShellOrExternal`.
fn head(argv: &[String], src: &str) -> Result<SessionHead, LoomDecline> {
    let unexpressible = || LoomDecline::Unexpressible(src.to_owned());
    match argv.first().map(String::as_str) {
        Some("dorc" | "dorc-loom" | "dorc-sh") => Ok(SessionHead::Invocation),
        Some("export") => export_op(argv)
            .map(SessionHead::Export)
            .ok_or_else(unexpressible),
        Some("cd") => match argv {
            [_, target] => Ok(SessionHead::Cd(target.clone())),
            _ => Err(unexpressible()),
        },
        Some("echo") if argv == ["echo", "$?"] => Ok(SessionHead::EchoStatus),
        Some("cat") => match argv {
            [_, path] => Ok(SessionHead::Cat(path.clone())),
            _ => Err(unexpressible()),
        },
        _ => Err(LoomDecline::ShellOrExternal(src.to_owned())),
    }
}

/// An `export` with EXACTLY one operand that is a plain `NAME` or `NAME=word`. A flag, a bare
/// `export`, several operands, or a name that is not a plain NAME declines (the value word is
/// already literal by [`literal_argv`], so a non-literal operand never reaches here).
fn export_op(argv: &[String]) -> Option<ExportOp> {
    let [_, operand] = argv else {
        return None;
    };
    match operand.split_once('=') {
        Some((name, value)) if dorc_syntax::sem::is_name(name) => Some(ExportOp::Assign {
            name: name.to_owned(),
            value: value.to_owned(),
        }),
        None if dorc_syntax::sem::is_name(operand) => Some(ExportOp::Mark {
            name: operand.clone(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{ExportOp, SessionHead, SessionInput, SessionReading, block_argv};
    use crate::consumer::LoomDecline;

    fn line(src: &str) -> super::SessionLine {
        match SessionReading::read(src) {
            SessionReading::Line(line) => line,
            SessionReading::Decline(reason) => panic!("expected a line, got a decline: {reason:?}"),
        }
    }

    fn decline(src: &str) -> LoomDecline {
        match SessionReading::read(src) {
            SessionReading::Decline(reason) => reason,
            SessionReading::Line(line) => panic!("expected a decline, got {line:?}"),
        }
    }

    /// The exact shape the corpus's receipt-rooted why line takes: a literal argv (a `book.sh:8`
    /// address is ONE word, not a `:`-mark, because a `$` line carries no dorc-lang marker), a `<`
    /// stdin file, and a suppressed stderr — every part inside the ruled set.
    #[test]
    fn a_dorc_invocation_with_the_ruled_redirects_is_an_invocation() {
        let read = line("dorc why book.sh:8 --book=book.sh --results - < probe.txt 2>/dev/null");
        assert_eq!(read.head, SessionHead::Invocation);
        assert_eq!(read.argv[0], "dorc");
        assert_eq!(read.argv[2], "book.sh:8");
        assert_eq!(read.input, SessionInput::File("probe.txt".to_owned()));
        assert!(read.output.stderr_to_null);
        assert!(!read.output.stdout_to_null);
    }

    #[test]
    fn stdout_to_dev_null_is_read_and_dev_null_stdin_is_null() {
        let read = line("dorc plan --book=book.sh < /dev/null > /dev/null");
        assert_eq!(read.input, SessionInput::Null);
        assert!(read.output.stdout_to_null);
    }

    /// A `> <file>` capture is inside the ruled set (errorloom admits it and routes it to the file;
    /// a later `cat` reads it back), so it reads as a stdout-to-file line, not a decline.
    #[test]
    fn stdout_to_a_real_file_is_read_as_a_capture() {
        let read = line("dorc plan --book=book.sh > plan.sh");
        assert_eq!(read.head, SessionHead::Invocation);
        assert_eq!(read.output.stdout_to_file.as_deref(), Some("plan.sh"));
        assert!(!read.output.stdout_to_null);
    }

    #[test]
    fn export_assign_and_mark_are_recognized_dash_semantics() {
        assert_eq!(
            line("export DORC_SEAM_CLOCK=pinned:1769306437000").head,
            SessionHead::Export(ExportOp::Assign {
                name: "DORC_SEAM_CLOCK".to_owned(),
                value: "pinned:1769306437000".to_owned(),
            })
        );
        assert_eq!(
            line("export DORC_SEED").head,
            SessionHead::Export(ExportOp::Mark {
                name: "DORC_SEED".to_owned(),
            })
        );
    }

    #[test]
    fn cd_echo_status_and_cat_are_recognized_structurally() {
        assert_eq!(line("cd subdir").head, SessionHead::Cd("subdir".to_owned()));
        assert_eq!(line("echo $?").head, SessionHead::EchoStatus);
        assert_eq!(
            line("cat book.sh").head,
            SessionHead::Cat("book.sh".to_owned())
        );
        assert_eq!(line("echo $?").argv, ["echo", "$?"]);
    }

    /// A leading assignment, a non-literal word, an unruled redirect, a pipeline, a list, a second
    /// command, an unrecognized head, and an export flag — each is a decline, never a best-effort
    /// read (`inv-top-reject`, `rul-unsure-falls-toward-sh-parity`).
    #[test]
    fn everything_outside_the_ruled_shapes_declines() {
        assert!(matches!(
            decline("FOO=bar dorc plan --book=book.sh"),
            LoomDecline::Unexpressible(_)
        ));
        assert!(matches!(
            decline("dorc plan --artifact-dir=$ARTIFACT_DIR"),
            LoomDecline::Unexpressible(_)
        ));
        assert!(matches!(
            decline("dorc plan --book=book.sh >> log"),
            LoomDecline::Unexpressible(_)
        ));
        assert!(matches!(
            decline("dorc plan | grep x"),
            LoomDecline::Unexpressible(_)
        ));
        assert!(matches!(
            decline("dorc plan; echo hi"),
            LoomDecline::Unexpressible(_)
        ));
        assert!(matches!(
            decline("hork tune --profile web"),
            LoomDecline::ShellOrExternal(_)
        ));
        assert!(matches!(
            decline("export -n DORC_SEED"),
            LoomDecline::Unexpressible(_)
        ));
    }

    /// The classifier needs the raw argv up to the first redirect, so a word the engine expands
    /// survives verbatim even though the expressibility reading declines it.
    #[test]
    fn block_argv_keeps_raw_word_source_for_classification() {
        assert_eq!(
            block_argv("dorc plan --artifact-dir=$ARTIFACT_DIR > /dev/null"),
            ["dorc", "plan", "--artifact-dir=$ARTIFACT_DIR"]
        );
        assert_eq!(block_argv("export DORC_SEED=1"), ["export", "DORC_SEED=1"]);
    }
}
