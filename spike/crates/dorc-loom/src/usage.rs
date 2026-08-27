//! What `dorc-loom` SAYS to a command line before it acts on one: the help pages, and the single
//! decision between "print a page", "refuse", and "run this".
//!
//! It lives beside the grammar rather than in the binary for the reason the grammar does
//! (`invocation`'s header): two seats read this surface — a terminal's argv, and a replay line
//! lifted out of a case — and a page or a refusal spelled once in each is how a transcript comes
//! to teach words a terminal never says. The committed transcript is the authoring surface
//! (`282:rul-transcript-is-the-authoring-surface`); text no case can drive is text nobody rereads.
//!
//! This is INTERNAL-TOOL text, not product prose: it is a const here rather than a registry entry,
//! and `aid/CLAUDE.md`'s registry law does not reach it.

use crate::invocation::{self, Invocation};

/// The name this tool's own failures wear, so the driver and `main` frame them identically.
pub const PROGRAM: &str = "dorc-loom";

/// What one command line resolves to before any file is read.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Reading {
    /// A help request: the page to print, and nothing else to do.
    Help(&'static str),
    /// The grammar refused, with the chosen verb's page underneath.
    Refused(String),
    /// It parses; here is what it asked for.
    Runs(Box<Invocation>),
}

/// Read one command line: `argv` without the program name.
///
/// `--help`/`-h` are FLAGS and are read anywhere, because a reader who has already typed a verb
/// asks THE VERB for help. The bare word `help` is a VERB, and is read only in verb position: a
/// subcommand's own positional argument is never a global request. That distinction is
/// load-bearing rather than tidy — `add-register CASE help` ends in the literal token `help`, so
/// while the scan took the bare word anywhere, the verb's only legal invocation was
/// indistinguishable from a help request: usage page, exit 0, nothing minted.
#[must_use]
pub fn read(words: &[&str]) -> Reading {
    let asks_for_help = words.iter().any(|word| matches!(*word, "--help" | "-h"))
        || words.first().is_some_and(|word| *word == "help");
    if asks_for_help {
        return Reading::Help(usage_for(chosen_verb(words).unwrap_or_default()));
    }
    match invocation::parse(std::iter::once(PROGRAM).chain(words.iter().copied())) {
        Ok(invocation) => Reading::Runs(Box::new(invocation)),
        Err(refusal) => Reading::Refused(with_page(&refusal, words)),
    }
}

/// A refusal with the page of whichever verb the reader had already chosen underneath it — the
/// non-refusing half of `28L:rul-refusals-name-the-next-command`.
#[must_use]
pub fn with_page(refusal: &str, words: &[&str]) -> String {
    format!(
        "{refusal}\n{}",
        usage_for(chosen_verb(words).unwrap_or_default())
    )
}

/// The verb this invocation is ABOUT, for the help and misuse pages: the leading verb, or the one
/// after a leading `help`. `None` means the reader has not chosen one and wants the index.
///
/// Global flags are skipped first, because they legitimately precede the verb: a reader who typed
/// `dorc-loom --this vars --help` has chosen `vars`.
#[must_use]
pub fn chosen_verb<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut rest = words.iter().copied();
    let leading = loop {
        let word = rest.next()?;
        // `-C` takes a value, which is the one word after it that is never a verb; its attached
        // spellings (`-CDIR`, `-C=DIR`) carry their own.
        if word == invocation::ROOT {
            rest.next()?;
        } else if word != invocation::THIS && !word.starts_with(invocation::ROOT) {
            break word;
        }
    };
    if leading == "help" {
        return rest.next().filter(|next| VERBS.contains(next));
    }
    VERBS.contains(&leading).then_some(leading)
}

/// Each verb's own page — what it does, what its flags mean, and the command that follows it.
///
/// The index below answers "which verb", and answers nothing else: it cannot say what `--verbatim`
/// is for, or which of two spellings of a flag is which. A reader who has already chosen a verb and
/// typed `--help` is asking the VERB, so that is what they get.
#[must_use]
pub fn usage_for(verb: &str) -> &'static str {
    match verb {
        "publish" => PUBLISH_USAGE,
        "vars" => VARS_USAGE,
        "sections" => SECTIONS_USAGE,
        "scaffold" => SCAFFOLD_USAGE,
        "add-register" => ADD_REGISTER_USAGE,
        "keys" => KEYS_USAGE,
        "defect" => DEFECT_USAGE,
        _ => USAGE,
    }
}

/// The verbs [`usage_for`] has a page for — also what makes `dorc-loom <verb> --help` route to it.
pub const VERBS: [&str; 7] = [
    "publish",
    "vars",
    "sections",
    "scaffold",
    "add-register",
    "keys",
    "defect",
];

/// The index.
pub const USAGE: &str = "usage: dorc-loom [--this] [-C DIR] <publish [--verbatim] [--all] [--quiet] [--accept-metadata] [--human|--slop] [--shell=PATH] [--path=DIR]... [CASE...]|vars [--used|--all] [CASE...]|scaffold SLUG|add-register CASE help|sections [CASE...]|keys|defect>\n       a CASE is a bare slug (`whylog-unwritten`), a filename, or a path; for the read-only verbs an omitted list means every crates/aid/tests/*.loom\n       --this comes BEFORE the verb and names the case a replay line is running inside; it resolves only there, never from a terminal\n       -C also comes before the verb and names the tree to resolve the corpus, both locks and the staging store under; without it, the tree this binary was built in\n       edit a sentence in a case's transcript, then publish it; type {{name}} to insert or move one of its values\n       `dorc-loom <subcommand> --help` explains one verb; this page is only the index";

const PUBLISH_USAGE: &str = "usage: dorc-loom [-C DIR] publish [--verbatim] [--all] [--quiet] [--accept-metadata] [--human|--slop] [--shell=PATH] [--path=DIR]... [CASE...]
  Drive every selected case's replays, compile the prose you edited back into template form, print
  what that does to each register in {{hole}} spelling, and publish it: both generated locks
  (crates/aid/src/catalog_lock.rs and arrangement_lock.rs) plus every affected case. In-process
  renders only -- no binary is run and no fixture is executed. Every byte and both fixpoints are
  computed before the first write, so a failure leaves the tree byte-identical. Nothing is staged or
  committed; the diff is yours -- `git diff --word-diff` is how prose reads.
  A publish that gives up a hole writes NOTHING, says which holes and why, and exits nonzero. The
  transcript renders values, so that loss is invisible in the case diff; the printed one is where
  you see it. Re-run with --verbatim once you have.
  --verbatim  publish an interpretation that gives up holes, exactly as it was just shown to you
  --all       every committed case. This verb rewrites files, so the whole corpus is spelled out
  --quiet     drop the header of every case that has nothing to report
  --accept-metadata  acknowledge that a case's when-fires / when-used / why REPLACES the committed
                     registry entry; without it a metadata change refuses before any write
  --human     mark every register this publishes as written by a person. Refuses in a session that
              announces itself as an agent; DORC_HUMAN_COMMIT=1 says a person is at the keyboard.
              Unflagged, a register is marked slop, whoever is driving.
  --slop      yes, re-mark a human-written register as slop. Unflagged, that refuses for a person
              (the forgotten --human) and proceeds with a note for an agent.
  --shell=P   lend the generic executor a shell, for a replay the in-process driver declines
  --path=D    prepend a directory to the replay PATH (repeatable)
  -C DIR      resolve the corpus, both locks and the staging store under DIR instead of the tree
              this binary was built in. It comes BEFORE the verb, git-style; no file outside DIR is
              read or written, and the repository asked what changed is the one containing DIR.
  next: mise run test -- a publish republishes shared locks, so its blast radius is wider than the
        case in front of you";

const VARS_USAGE: &str = "usage: dorc-loom [--this] [-C DIR] vars [--used|--all] [CASE...]
  Print each case's named template variables and their currently-rendered values, driven from the
  same render an edit compiles against. A case with no variables prints no row at all; stderr
  carries how many there were.
  --used   only the variables some rendered section actually consumes (the default)
  --all    the whole typed payload, including values no sentence spends yet
  --this   the case this invocation is running inside -- a replay line's spelling, so a case never
           has to name itself. It comes before the verb and resolves nowhere else.
  -C DIR   the tree to resolve the collection under; before the verb, git-style
  next: type {{name}} into a sentence in the transcript to insert or move that value, then
        dorc-loom publish";

const SECTIONS_USAGE: &str = "usage: dorc-loom [--this] [-C DIR] sections [CASE...]
  Per replay, print each editable section's key and its ordered Text|Variable fragment series,
  alongside the computed (immutable) spans around it. The answer to `which bytes in this transcript
  are mine to edit` -- read-only, and driven from the published baseline rather than from your
  worktree.
  --this   the case this invocation is running inside -- a replay line's spelling, so a case never
           has to name itself. It comes before the verb and resolves nowhere else.
  -C DIR   the tree to resolve the collection under; before the verb, git-style
  next: edit an editable section in the case, then dorc-loom publish";

const SCAFFOLD_USAGE: &str = "usage: dorc-loom [-C DIR] scaffold SLUG
  Write the empty defining-case skeleton for a freshly-minted code slug to
  crates/aid/tests/SLUG.loom. Never overwrites an authored case, and never writes prose: the message
  register stays unwritten, and everything the skeleton omits is deliberately red until a world that
  really fires the code is authored.
  next: the two-step follow-up the command prints";

const KEYS_USAGE: &str = "usage: dorc-loom keys
  Print the closed frontmatter-key vocabulary a case may declare, and which gate reads each key --
  including which of `code:` and `arrangement:` a case wants. Takes no arguments and reads no case.
  next: declare the key in the case's frontmatter; anything outside this set is refused by the
        runners, because a key no gate reads is an assertion you only believe you made";

const ADD_REGISTER_USAGE: &str = "usage: dorc-loom [-C DIR] add-register CASE help
  Mint a code's `help` register, so the ordinary transcript loop can fill it. The CASE spelling is
  every other verb's -- a bare slug, a filename, or a path. `help` is the only addable register --
  `message` exists on every code already. Refuses when the case carries an unpromoted prose edit,
  or when the register is already there.
  next: rebuild, overtype the printed [unwritten: SLUG.help] placeholder, then publish";

const DEFECT_USAGE: &str = "usage: dorc-loom --this defect
  Loom-only harness route for the three explicitly authorized correctness-critical internal
  failures that cannot be induced by an external production scenario. It resolves the current
  case's code through a closed typed list and uses the production diagnostic event renderer.
  next: use this only in the defining loom for an authorized internal-defect code";

#[cfg(test)]
mod tests {
    use super::*;

    /// Every verb has a page of its own, and every page ends where the reader goes next. The index
    /// answers "which verb" and nothing else, so a verb that falls through to it silently teaches a
    /// reader who already knew the one thing it says.
    #[test]
    fn every_verb_has_its_own_page_ending_in_a_next_command() {
        for verb in VERBS {
            let page = usage_for(verb);
            assert_ne!(page, USAGE, "`{verb}` falls through to the index");
            // A synopsis may carry the global flags before the verb, so the pin is that it names
            // THIS verb, not that it opens on an exact prefix.
            let synopsis = page.lines().next().unwrap_or_default();
            assert!(
                synopsis.starts_with("usage: dorc-loom ") && synopsis.contains(verb),
                "`{verb}` page opens on someone else's synopsis: {synopsis}"
            );
            assert!(
                page.contains("\n  next: "),
                "`{verb}` page has no next: {page}"
            );
        }
        assert_eq!(usage_for("nonesuch"), USAGE);
    }

    /// `--help` after a verb asks the VERB; before one, or with no verb at all, it asks the index.
    /// A global flag before the verb is transparent, and `-C`'s VALUE is never mistaken for one.
    #[test]
    fn help_routes_to_the_verb_the_reader_already_chose() {
        assert_eq!(chosen_verb(&["publish", "--help"]), Some("publish"));
        assert_eq!(chosen_verb(&["help", "vars"]), Some("vars"));
        assert_eq!(chosen_verb(&["--this", "vars", "--help"]), Some("vars"));
        assert_eq!(chosen_verb(&["-C", "keys", "vars"]), Some("vars"));
        assert_eq!(chosen_verb(&["-C/tmp/world", "publish"]), Some("publish"));
        assert_eq!(chosen_verb(&["help"]), None);
        assert_eq!(chosen_verb(&["--help"]), None);
        assert_eq!(chosen_verb(&["nonesuch", "-h"]), None);
    }

    /// The three readings, and the one that carries a page: a refusal lands on the page of the verb
    /// the reader had already chosen, never on the index.
    #[test]
    fn a_refusal_carries_the_chosen_verbs_page() {
        assert!(matches!(read(&["keys"]), Reading::Runs(_)));
        assert!(matches!(read(&["--help"]), Reading::Help(USAGE)));

        let Reading::Refused(refusal) = read(&["publish"]) else {
            panic!("a bare publish refuses")
        };
        assert!(
            refusal.contains("usage: dorc-loom [-C DIR] publish"),
            "{refusal}"
        );
        assert!(refusal.contains("--all"), "{refusal}");
    }
}
