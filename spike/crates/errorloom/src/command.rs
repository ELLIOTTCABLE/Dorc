//! The closed, portable replay-command grammar.

use std::collections::BTreeSet;

/// A command's observable output channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReplayChannel {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

/// A portable redirection target.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RedirectionTarget {
    /// A case-relative sandbox file.
    File(String),
    /// The platform-independent `/dev/null` spelling.
    Null,
}

/// The input selected for a replay command.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReplayInputTarget {
    /// A case-relative sandbox file.
    File(String),
    /// Empty input, spelled `/dev/null`.
    Null,
}

/// One output-routing action, applied left to right.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OutputRedirection {
    /// Route one channel to a file or `/dev/null`.
    To {
        /// The channel being changed.
        channel: ReplayChannel,
        /// Its new destination.
        target: RedirectionTarget,
    },
    /// Copy stdout's current destination onto stderr (`2>&1`).
    StderrToStdout,
}

/// A parsed simple command and its conservative trailing redirections.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplayCommand {
    original: String,
    argv: Vec<String>,
    input: Option<ReplayInputTarget>,
    outputs: Vec<OutputRedirection>,
}

impl ReplayCommand {
    /// Parse one replay command without invoking a shell.
    ///
    /// # Errors
    /// Refuses every spelling outside the closed grammar.
    pub fn parse(command: &str) -> Result<Self, ReplayParseError> {
        let words: Vec<&str> = command.split_ascii_whitespace().collect();
        if words.is_empty() {
            return Err(ReplayParseError::Empty);
        }

        let mut argv = Vec::new();
        let mut input = None;
        let mut outputs = Vec::new();
        let mut redirected = false;
        let mut stdout_set = false;
        let mut stderr_set = false;
        let mut opened_paths = BTreeSet::new();
        let mut index = 0;
        while let Some(word) = words.get(index).copied() {
            if word.contains(">>") || word.contains("<<") || (word.contains('&') && word != "2>&1")
            {
                return Err(ReplayParseError::UnsupportedSyntax {
                    word: word.to_owned(),
                });
            }
            if word == "2>&1" {
                redirected = true;
                if stderr_set {
                    return Err(ReplayParseError::DescriptorRepeated { descriptor: 2 });
                }
                stderr_set = true;
                outputs.push(OutputRedirection::StderrToStdout);
                index = index.saturating_add(1);
                continue;
            }

            let redirection = redirection_word(word);
            if let Some((kind, attached)) = redirection {
                redirected = true;
                let target = if let Some(path) = attached {
                    path
                } else {
                    index = index.saturating_add(1);
                    words
                        .get(index)
                        .copied()
                        .ok_or(ReplayParseError::MissingRedirectionTarget)?
                };
                match kind {
                    RedirectionKind::Input => {
                        if input.is_some() {
                            return Err(ReplayParseError::DescriptorRepeated { descriptor: 0 });
                        }
                        input = Some(parse_input_target(target)?);
                    }
                    RedirectionKind::Stdout => push_output_redirection(
                        ReplayChannel::Stdout,
                        &mut stdout_set,
                        target,
                        &mut opened_paths,
                        &mut outputs,
                    )?,
                    RedirectionKind::Stderr => push_output_redirection(
                        ReplayChannel::Stderr,
                        &mut stderr_set,
                        target,
                        &mut opened_paths,
                        &mut outputs,
                    )?,
                }
                index = index.saturating_add(1);
                continue;
            }

            if redirected {
                return Err(ReplayParseError::RedirectionNotTrailing {
                    word: word.to_owned(),
                });
            }
            if unsupported_word(word) {
                return Err(ReplayParseError::UnsupportedSyntax {
                    word: word.to_owned(),
                });
            }
            argv.push(word.to_owned());
            index = index.saturating_add(1);
        }

        if argv.is_empty() {
            return Err(ReplayParseError::Empty);
        }
        if argv.iter().any(|word| word.contains('$'))
            && !matches!(argv.as_slice(), [echo, status] if echo == "echo" && status == "$?")
        {
            return Err(ReplayParseError::UnsupportedSyntax {
                word: "$ expansion".to_owned(),
            });
        }
        Ok(Self {
            original: command.to_owned(),
            argv,
            input,
            outputs,
        })
    }

    /// The exact replay line after `$ `.
    #[must_use]
    pub fn original(&self) -> &str {
        &self.original
    }

    /// The simple command argv, excluding redirections.
    #[must_use]
    pub fn argv(&self) -> &[String] {
        &self.argv
    }

    /// The selected stdin source, if any.
    #[must_use]
    pub fn input(&self) -> Option<&ReplayInputTarget> {
        self.input.as_ref()
    }

    /// Whether stdout still points at the terminal after all redirections.
    #[must_use]
    pub fn stdout_is_terminal(&self) -> bool {
        !self.outputs.iter().any(|redirection| {
            matches!(
                redirection,
                OutputRedirection::To {
                    channel: ReplayChannel::Stdout,
                    ..
                }
            )
        })
    }

    pub(crate) fn output_redirections(&self) -> &[OutputRedirection] {
        &self.outputs
    }
}

/// Why a replay command is outside the closed grammar.
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ReplayParseError {
    /// No command word was present.
    Empty,
    /// Quoting, expansion, a compound operator, a pipeline, append, or fd algebra was present.
    UnsupportedSyntax {
        /// The token that forced refusal.
        word: String,
    },
    /// A redirection was followed by another command argument.
    RedirectionNotTrailing {
        /// The unexpected argument.
        word: String,
    },
    /// A redirection had no target.
    MissingRedirectionTarget,
    /// The same descriptor was assigned more than once.
    DescriptorRepeated {
        /// The repeated descriptor.
        descriptor: u8,
    },
    /// Two independent file opens named one path.
    DuplicateOutputPath {
        /// The ambiguously opened path.
        path: String,
    },
    /// A redirection path was not a safe case-relative path or `/dev/null`.
    UnsafePath {
        /// The rejected path.
        path: String,
    },
}

impl std::fmt::Display for ReplayParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ReplayParseError {}

#[derive(Clone, Copy)]
enum RedirectionKind {
    Input,
    Stdout,
    Stderr,
}

fn redirection_word(word: &str) -> Option<(RedirectionKind, Option<&str>)> {
    for (prefix, kind) in [
        ("2>", RedirectionKind::Stderr),
        ("1>", RedirectionKind::Stdout),
        (">", RedirectionKind::Stdout),
        ("<", RedirectionKind::Input),
    ] {
        if let Some(target) = word.strip_prefix(prefix) {
            return Some((kind, (!target.is_empty()).then_some(target)));
        }
    }
    None
}

fn parse_input_target(path: &str) -> Result<ReplayInputTarget, ReplayParseError> {
    if path == "/dev/null" {
        return Ok(ReplayInputTarget::Null);
    }
    safe_path(path)?;
    Ok(ReplayInputTarget::File(path.to_owned()))
}

fn parse_output_target(path: &str) -> Result<RedirectionTarget, ReplayParseError> {
    if path == "/dev/null" {
        return Ok(RedirectionTarget::Null);
    }
    safe_path(path)?;
    Ok(RedirectionTarget::File(path.to_owned()))
}

fn push_output_redirection(
    channel: ReplayChannel,
    assigned: &mut bool,
    path: &str,
    opened: &mut BTreeSet<String>,
    outputs: &mut Vec<OutputRedirection>,
) -> Result<(), ReplayParseError> {
    if *assigned {
        let descriptor = match channel {
            ReplayChannel::Stdout => 1,
            ReplayChannel::Stderr => 2,
        };
        return Err(ReplayParseError::DescriptorRepeated { descriptor });
    }
    *assigned = true;
    let target = parse_output_target(path)?;
    remember_open(&target, opened)?;
    outputs.push(OutputRedirection::To { channel, target });
    Ok(())
}

fn remember_open(
    target: &RedirectionTarget,
    opened: &mut BTreeSet<String>,
) -> Result<(), ReplayParseError> {
    let RedirectionTarget::File(path) = target else {
        return Ok(());
    };
    if opened.insert(path.clone()) {
        Ok(())
    } else {
        Err(ReplayParseError::DuplicateOutputPath { path: path.clone() })
    }
}

fn safe_path(path: &str) -> Result<(), ReplayParseError> {
    let safe = !path.is_empty()
        && !path.starts_with('/')
        && !path.contains([
            '\\', ':', '\'', '"', '`', ';', '|', '&', '(', ')', '$', '<', '>',
        ])
        && path
            .split('/')
            .all(|component| !matches!(component, "" | "." | ".."));
    if safe {
        Ok(())
    } else {
        Err(ReplayParseError::UnsafePath {
            path: path.to_owned(),
        })
    }
}

fn unsupported_word(word: &str) -> bool {
    word.contains(['\'', '"', '`', ';', '|', '&', '(', ')', '\\', '\n', '\r'])
        || word.contains("<<")
        || word.contains(">>")
        || (word.contains('<') && !word.starts_with('<'))
        || (word.contains('>')
            && !word.starts_with('>')
            && !word.starts_with("1>")
            && !word.starts_with("2>"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_closed_redirection_set() {
        let command =
            ReplayCommand::parse("dorc plan --book=book.sh < input.txt >plan.txt 2>errors.txt")
                .expect("closed grammar");
        assert_eq!(command.argv(), ["dorc", "plan", "--book=book.sh"]);
        assert_eq!(
            command.input(),
            Some(&ReplayInputTarget::File("input.txt".to_owned()))
        );
        assert_eq!(command.output_redirections().len(), 2);

        let merged = ReplayCommand::parse("dorc plan --book=book.sh 1>/dev/null 2>&1")
            .expect("null and descriptor copy");
        assert_eq!(merged.output_redirections().len(), 2);
    }

    #[test]
    fn accepts_attached_targets_and_the_status_builtin() {
        assert!(ReplayCommand::parse("cat output.txt").is_ok());
        assert!(ReplayCommand::parse("echo $?").is_ok());
        assert!(ReplayCommand::parse("tool <input >output 2>/dev/null").is_ok());
    }

    #[test]
    fn refuses_every_open_ended_shell_form() {
        for command in [
            "tool 'quoted'",
            "tool \"quoted\"",
            "tool | other",
            "tool && other",
            "tool; other",
            "tool >>log",
            "tool 2>>log",
            "tool 1>&2",
            "tool $(other)",
            "tool >out trailing",
            "tool >same 2>same",
            "tool >one >two",
            "tool >/absolute",
            "tool >../escape",
        ] {
            assert!(
                ReplayCommand::parse(command).is_err(),
                "accepted {command:?}"
            );
        }
    }
}
