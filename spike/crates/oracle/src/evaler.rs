//! `evaler` — the eval'er surface: reentry detection inside `cmd__predict()`
//! (`274` · `271:rul-evaler-merge-no-structure-member`).
//!
//! An **eval'er** is a command that hands a STRING (or a stream) to a child shell — `sh -c 'STR'`,
//! `bash -c`, `dash -c`, `su -c`. Its oracle is an ORDINARY `<cmd>__predict` (there is NO separate
//! structure member — `271:rul-evaler-merge-no-structure-member`): the predict argparses which
//! operand is code and DELEGATES to the reentry form. That delegation is an ACTUAL COMMAND — the
//! blessed reentry primitive `dorc:sh` — never `eval` (`274` §0; `dialect-quality-law`: authored
//! `eval` never). Its command-position head being `dorc:sh` is what makes the payload analyzable and
//! chooses the room (`core::room`); the env-idiom on that head is the guest's ρ-claim
//! (`271:rul-env-claim-inversion`, `274` §2 the reentry ρ-split).
//!
//! This module reads the reentry off the authored predict body, NEVER by running it
//! (`static-lift-only`), exactly as `crate::wrapper` reads the peel:
//!
//! 1. **reentry detection + head room** ([`detect_evaler`]) — the first reachable command whose head
//!    (optionally through `env … ρ-args`) is a `dorc:sh` (invited) or `dorc-sh` (runtime object)
//!    reentry primitive. A bare-`sh` head is NOT a recognized reentry — it is the escape hatch (the
//!    payload rides the vouch + hint-descent, `274` §12 finding-scope-clarification), so it minted no
//!    reentry to detect.
//! 2. **the payload shape** ([`EvalerShape`]) — which-arg-is-code (`-c <word>`), stdin-code (`-s`),
//!    or a file/bare-stdin form; plus whether the reentry binds a positional tail (`"$@"` ⇒
//!    `sh -c CODE NAME ARGS`, POSIX §2 positional assignment).
//! 3. **the reentry ρ-claim** ([`crate::wrapper::RhoClaim`]) — read off the reentry's env-head, the
//!    same ladder the peel uses (`274` §2: the ρ CONTENT the guest sees is the standing env-idiom
//!    ladder; owning the token defines the child-context MECHANICS, not the ρ content).
//!
//! # Scope fence (`27J` §2.3, MODELS only)
//!
//! Like `crate::wrapper`, this lane builds the model and mints NO license: nothing here is consumed
//! by `analysis`/`plan` yet, so an eval'er book site with no eval'er oracle loaded walls opaquely
//! (`silence-licenses-nothing`) and the corpus stays byte-stable (`empty-world-byte-identical`). The
//! book-site resolution (running the argparse to read the concrete payload, then the nested parse +
//! whole-line fold) is the next slice; this settles what the reentry IS.
//!
//! `inv-referent-agnostic`: the walk reads the body's STRUCTURE (the reentry head literal, the `-c`
//! flag, the `env` syllable) — never what a payload operand MEANS.

use crate::predict::{Command, Predict, Stmt, Word};
use crate::wrapper::RhoClaim;
use dorc_core::RoomTag;
use dorc_syntax::sem::{EvalerHead, classify_evaler_head};

/// A detected eval'er reentry: the room its payload is analyzed in, the ρ-claim the guest is born
/// into, and the payload shape. Detection is by the reentry primitive in command position (`274`
/// §0/§3) — a predict body that delegates to `dorc:sh`/`dorc-sh` IS an eval'er oracle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaler {
    /// The room the payload is analyzed in, from the reentry HEAD spelling (`274` §1): `dorc:sh` ⇒
    /// [`RoomTag::Invited`] (full license); `dorc-sh` (row 3) ⇒ `None` — a runtime-object reentry
    /// grants no analysis license (an unusual oracle spelling, recorded not rejected).
    pub room: Option<RoomTag>,
    /// The ρ-claim the guest payload's environment carries, read off the reentry env-head
    /// (`271:rul-env-claim-inversion`; `274` §2). Bare `dorc:sh …` ⇒ [`RhoClaim::Nothing`];
    /// `env dorc:sh …` ⇒ full ambient; `env -i … dorc:sh …` ⇒ exactly-these.
    pub rho: RhoClaim,
    /// Which operand is code (or the stdin/file form) — the eval'er argparse's product.
    pub shape: EvalerShape,
}

/// The payload SHAPE an eval'er reentry declares — which-arg-is-code (`274` §3; `24T` pin1
/// licensed-code-carriers). Read off the reentry's own argv (the child sh's flags), after the
/// reentry head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalerShape {
    /// `-c <payload>` — the payload is the operand after `-c` (the reentry's `"$code"`). Carries the
    /// payload word so the book-site resolution reads which operand it traces to (usually a
    /// [`Word::Var`] the argparse bound; a literal is the trivially-resolved case). `binds_argv` is
    /// true when a trailing `"$@"` follows — the POSIX `sh -c CODE NAME ARGS` positional binding
    /// (`24T:L3`; `$0`=NAME, `$@`=ARGS).
    DashC {
        /// The payload word (`"$code"` ⇒ [`Word::Var`]).
        payload: Word,
        /// A trailing `"$@"` positional binding follows the payload.
        binds_argv: bool,
    },
    /// `-s` — stdin-fed code (`sh -s args`): the operands bind as positionals, the code arrives on
    /// stdin. Modeled as a distinct shape; the stdin value-carriage is the capture lane's coupling
    /// (`274` §11 task-7).
    DashS {
        /// A trailing `"$@"` positional binding follows.
        binds_argv: bool,
    },
    /// Neither `-c` nor `-s` after the reentry head — a file operand (`sh FILE args`) or bare stdin
    /// (`sh` alone). The local-stream cell (`24T` imp-P4 in-scope half); recorded, resolved later.
    FileOrStdin,
}

/// Detect whether `check` (a `<cmd>__predict` body) is an eval'er — i.e. it delegates to the
/// reentry primitive (`274` §0/§3). Returns `None` for an ordinary predict (no reentry). A pre-order
/// walk finds the FIRST reachable reentry command (a `case`-arm reentry counts). Pure/total
/// (`inv-no-throw`/`inv-determinism`).
#[must_use]
pub fn detect_evaler(check: &Predict) -> Option<Evaler> {
    let cmd = first_reentry_command(&check.body)?;
    let (room, after_head) = reentry_head(cmd)?;
    Some(Evaler {
        room,
        rho: reentry_rho(cmd),
        shape: reentry_shape(after_head),
    })
}

/// The first reachable command that is a reentry (`dorc:sh`/`dorc-sh` in command position), walking
/// control flow in source order (a reentry in a `case`/`if`/`while` arm counts). `None` ⇒ not an
/// eval'er. Mirrors `crate::wrapper::first_peel_command`.
fn first_reentry_command(body: &[Stmt]) -> Option<&Command> {
    for stmt in body {
        match stmt {
            Stmt::Command(c) if reentry_head(c).is_some() => return Some(c),
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    if let Some(c) = first_reentry_command(&arm.body) {
                        return Some(c);
                    }
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                if let Some(c) =
                    first_reentry_command(then_body).or_else(|| first_reentry_command(else_body))
                {
                    return Some(c);
                }
            }
            Stmt::While { body, .. } => {
                if let Some(c) = first_reentry_command(body) {
                    return Some(c);
                }
            }
            _ => {}
        }
    }
    None
}

/// If `cmd` is a reentry command, return `(room, words-after-the-reentry-head)`. A reentry head is a
/// `dorc:`-prefixed literal (invited) or exactly `dorc-sh` (runtime object), optionally preceded by
/// an `env` head + ρ-args. `None` ⇒ this command is not a reentry (an ordinary body command, or a
/// bare-`sh` escape — not a recognized reentry primitive).
fn reentry_head(cmd: &Command) -> Option<(Option<RoomTag>, &[Word])> {
    // A pipeline never reentries in the modeled sense (`inv-top-reject`).
    if cmd.pipeline {
        return None;
    }
    let words = cmd.words.as_slice();
    // Locate the reentry head: skip a leading `env` + its ρ-args (literals) until a `dorc:`/`dorc-sh`
    // head. Without a leading `env`, the head must be words[0].
    let head_idx = reentry_head_index(words)?;
    let head = classify_word_head(&words[head_idx])?;
    match head {
        // Invited (dorc:sh) or the runtime object (dorc-sh) are reentries; the room follows the head.
        EvalerHead::Invited { .. } | EvalerHead::RuntimeObject => {
            Some((head.analysis_room(), &words[head_idx.saturating_add(1)..]))
        }
        // A bare head reached here only via the words[0] branch of `reentry_head_index`, which never
        // returns a bare head — so this arm is unreachable, but stays exhaustive (`inv-no-throw`).
        EvalerHead::Bare { .. } => None,
    }
}

/// The index of the reentry head word (`dorc:sh`/`dorc-sh`) within `words`, allowing a leading `env`
/// prefix (`env … dorc:sh …`). `None` if no reentry head is present. The `env`-prefix scan stops at
/// the first `dorc:`/`dorc-sh` literal; anything between is ρ-args (read by [`reentry_rho`]).
fn reentry_head_index(words: &[Word]) -> Option<usize> {
    match words.first() {
        Some(Word::Literal(h)) if h == "env" => words.iter().position(|w| {
            matches!(
                classify_word_head(w),
                Some(EvalerHead::Invited { .. } | EvalerHead::RuntimeObject)
            )
        }),
        Some(w)
            if matches!(
                classify_word_head(w),
                Some(EvalerHead::Invited { .. } | EvalerHead::RuntimeObject)
            ) =>
        {
            Some(0)
        }
        _ => None,
    }
}

/// Classify a command word as an [`EvalerHead`], if it is a plain literal (`dorc:sh` ⇒ `Invited`,
/// `dorc-sh` ⇒ `RuntimeObject`, else `Bare`). A non-literal word (`Var`, `"$@"`) is not a head we
/// classify. Bridges into `syntax::sem`'s recognition (`274` §1).
fn classify_word_head(word: &Word) -> Option<EvalerHead<'_>> {
    match word {
        Word::Literal(s) | Word::SingleQuotedLiteral(s) => Some(classify_evaler_head(s)),
        _ => None,
    }
}

/// The ρ-claim of a reentry, read off its env-head (`271:rul-env-claim-inversion`; `274` §2). Bare
/// `dorc:sh …` (no `env`) claims nothing; an `env … dorc:sh …` reentry parses env's ρ-args into the
/// ladder rung. Reuses the same enumerated grammar as the peel (`274` §12 r1–r6): assignments,
/// `-i`, `env -` (= `-i`); anything else ⇒ claims-nothing.
fn reentry_rho(cmd: &Command) -> RhoClaim {
    let words = cmd.words.as_slice();
    let Some(head_idx) = reentry_head_index(words) else {
        return RhoClaim::Nothing;
    };
    // Bare reentry (head at 0, no `env`) claims nothing.
    if head_idx == 0 {
        return RhoClaim::Nothing;
    }
    // `env <ρ-args…> <reentry-head>` — the ρ-args are words[1..head_idx].
    rho_of_env_args(&words[1..head_idx])
}

/// Parse the env ρ-args between an `env` head and a reentry head into a [`RhoClaim`] (`274` §12
/// r1–r6). Enumerated SYNTACTIC grammar: `VAR=v` assignments, `-i`/`-` (scrub base); any unrecognized
/// flag or dynamic arg ⇒ claims-nothing (safe + hint). Mirrors `crate::wrapper::rho_of_env`, but the
/// terminator is the reentry head, not a trailing `"$@"`.
fn rho_of_env_args(args: &[Word]) -> RhoClaim {
    let mut scrubbed = false;
    let mut assignments: Vec<String> = Vec::new();
    for w in args {
        let Word::Literal(tok) = w else {
            return RhoClaim::Nothing;
        };
        if tok == "-i" || tok == "-" {
            scrubbed = true;
        } else if let Some(name) = var_assignment_name(tok) {
            assignments.push(name);
        } else {
            return RhoClaim::Nothing;
        }
    }
    if scrubbed {
        RhoClaim::ExactlyThese { vars: assignments }
    } else {
        RhoClaim::FullAmbient {
            overrides: assignments,
        }
    }
}

/// A `VAR=value` assignment token's NAME (the value is runtime argv, resolved later), or `None` if
/// not a valid assignment (the name must be a sh NAME).
fn var_assignment_name(tok: &str) -> Option<String> {
    let (name, _value) = tok.split_once('=')?;
    dorc_syntax::sem::is_name(name).then(|| name.to_owned())
}

/// Read the payload [`EvalerShape`] from the reentry's argv (the words after the reentry head): the
/// child sh's own flags (`274` §3). `-c <word>` ⇒ [`EvalerShape::DashC`] (the payload operand);
/// `-s` ⇒ [`EvalerShape::DashS`] (stdin-code); anything else ⇒ [`EvalerShape::FileOrStdin`].
fn reentry_shape(after_head: &[Word]) -> EvalerShape {
    match after_head {
        [Word::Literal(f), payload, rest @ ..] if f == "-c" => EvalerShape::DashC {
            payload: payload.clone(),
            binds_argv: rest.iter().any(|w| matches!(w, Word::PositionalArgs)),
        },
        [Word::Literal(f), rest @ ..] if f == "-s" => EvalerShape::DashS {
            binds_argv: rest.iter().any(|w| matches!(w, Word::PositionalArgs)),
        },
        _ => EvalerShape::FileOrStdin,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predict::lift_predicts;
    use dorc_core::Interner;

    /// Lift the sole predict funcdef from `src`. (The `while :` infinite-loop argparse idiom is
    /// out of the predict dialect — it needs a `[ ]` test — so these strawmen use the `case`-based
    /// argparse, which lifts; a build-surfaced dialect limitation, noted for the stdlib brief.)
    fn one_predict(src: &str) -> Predict {
        let mut i = Interner::default();
        let set = lift_predicts(&mut i, src);
        assert!(set.diags.is_empty(), "clean predict lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        set.value.get(p).expect("the predict").clone()
    }

    #[test]
    fn dash_c_reentry_invited_full_ambient() {
        // The workhorse sh oracle (`274` §4, `case`-argparse form): `-c) code="${2-}"; env dorc:sh
        // -c "$code" "$@"`. The reentry is INVITED (dorc:sh), ρ = full ambient (the `env` syllable),
        // shape = DashC binding the positional tail.
        let check = one_predict(
            "sh__predict() { case \"${1-}\" in \
             -c) code=\"${2-}\"; env dorc:sh -c \"$code\" \"$@\" ;; \
             *) return 2 ;; esac; }",
        );
        let e = detect_evaler(&check).expect("an eval'er reentry");
        assert_eq!(e.room, Some(RoomTag::Invited));
        assert_eq!(e.rho, RhoClaim::FullAmbient { overrides: vec![] });
        let EvalerShape::DashC { binds_argv, .. } = e.shape else {
            panic!("expected a -c shape, got {:?}", e.shape);
        };
        assert!(
            binds_argv,
            "the trailing `\"$@\"` is the positional binding"
        );
    }

    #[test]
    fn bare_dorcsh_reentry_claims_nothing() {
        // `dorc:sh -c "$code" "$@"` with NO `env` head — invited room, but ρ claims NOTHING (the
        // su-login arm's shape: `274` §2, login-file env is host state).
        let check = one_predict("su__predict() { dorc:sh -c \"$code\" \"$@\"; }");
        let e = detect_evaler(&check).expect("an eval'er reentry");
        assert_eq!(e.room, Some(RoomTag::Invited));
        assert_eq!(e.rho, RhoClaim::Nothing);
    }

    #[test]
    fn env_dash_i_reentry_exactly_these() {
        // `env -i TERM=x dorc:sh -c "$code" "$@"` — the scrubbed-base reentry (sudo-ish): exactly-these ρ.
        let check = one_predict("sudo__predict() { env -i TERM=x dorc:sh -c \"$code\" \"$@\"; }");
        let e = detect_evaler(&check).expect("an eval'er reentry");
        assert_eq!(
            e.rho,
            RhoClaim::ExactlyThese {
                vars: vec!["TERM".to_owned()]
            }
        );
    }

    #[test]
    fn dash_s_reentry_is_stdin_code() {
        // `-s` stdin-code shape: `env dorc:sh -s "$@"` (operands are positionals, code on stdin).
        let check = one_predict("sh__predict() { env dorc:sh -s \"$@\"; }");
        let e = detect_evaler(&check).expect("an eval'er reentry");
        assert_eq!(e.shape, EvalerShape::DashS { binds_argv: true });
    }

    #[test]
    fn dorc_sh_runtime_reentry_grants_no_room() {
        // A predict delegating to `dorc-sh` (row 3, runtime object) grants NO analysis room — recorded
        // (room = None), not rejected. An unusual oracle spelling; the payload is not analyzed.
        let check = one_predict("x__predict() { dorc-sh -c \"$code\" \"$@\"; }");
        let e = detect_evaler(&check).expect("a runtime-object reentry is still detected");
        assert_eq!(e.room, None, "row-3 reentry grants no analysis license");
    }

    #[test]
    fn ordinary_predict_is_not_an_evaler() {
        // A normal tool predict (no reentry primitive) is not an eval'er.
        let check =
            one_predict("apt_get__predict() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; }");
        assert_eq!(detect_evaler(&check), None);
    }

    #[test]
    fn bare_sh_reentry_is_not_recognized_as_a_reentry() {
        // A predict whose body runs bare `sh -c "$code"` is the ESCAPE HATCH — not a recognized
        // reentry primitive (`274` §12 finding-scope-clarification: bare `sh` rides the vouch +
        // hint-descent, it minted no reentry). `detect_evaler` returns None.
        let check = one_predict("weird__predict() { sh -c \"$code\" \"$@\"; }");
        assert_eq!(
            detect_evaler(&check),
            None,
            "a bare-sh delegation is the escape hatch, not a detected reentry"
        );
    }
}
