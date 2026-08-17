//! The one `dorc-loom` argv grammar (`30C:rul-this-is-a-global-flag`).
//!
//! Two seats read this surface: the binary reads a terminal's argv, and the in-loom replay driver
//! reads the command TEXT lifted out of a case. They go through this module so they cannot disagree
//! by construction — a `--this` the binary accepts in one position and the driver claims in another
//! would render one thing in a transcript and another in a terminal, which is the whole failure the
//! committed-transcript surface exists to prevent (`282:rul-transcript-is-the-authoring-surface`).
//!
//! Nothing here touches the filesystem, prints, or exits: a positional is a raw string the binary
//! resolves afterwards, and every refusal is returned. The driver's contract is a clean decline for
//! shapes it does not claim, and a parser that printed or exited would take a whole loom run with it.

use clap::{Args, Parser, Subcommand};

/// Which cases a verb was pointed at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target<'a> {
    /// `--this`: the case this invocation is running inside. Resolvable only by the in-loom
    /// driver, which is the one seat that holds a bound case.
    This,
    /// The cases named on the command line; empty means the whole collection.
    Named(&'a [String]),
}

/// How much of a case's typed payload `vars` lists.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Breadth {
    /// Only the variables some rendered section actually consumes — the default breadth
    /// (`30C:rul-used-is-the-default-breadth`).
    Used,
    /// The whole typed payload, including values no sentence spends yet.
    All,
}

/// One parsed `dorc-loom` command line: the global layer, then a verb and its own arguments.
#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[command(
    name = "dorc-loom",
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
pub struct Invocation {
    /// The case this invocation is running inside, in place of a named one.
    ///
    /// Global and git-shaped — it comes BEFORE the verb, because it selects a target rather than
    /// tuning one verb's behaviour.
    #[arg(long)]
    pub this: bool,
    /// What to do.
    #[command(subcommand)]
    pub verb: Verb,
}

/// The verbs, each with its own arguments.
#[derive(Subcommand, Clone, PartialEq, Eq, Debug)]
pub enum Verb {
    /// Drive the selected cases and compile the prose edits back into template form.
    Compile(PublishArgs),
    /// Verify against the compile receipt, then publish both locks and the affected cases.
    Promote(PublishArgs),
    /// Print each case's named template variables and their currently-rendered values.
    Vars(VarsArgs),
    /// Print each replay's editable sections and their fragment series.
    Sections(SectionsArgs),
    /// Write the empty defining-case skeleton for a freshly-minted code slug.
    Scaffold {
        /// The new code slug.
        slug: String,
    },
    /// Mint a code's `help` register so the ordinary transcript loop can fill it.
    AddRegister {
        /// The defining case: a bare slug, a filename, or a path.
        case: String,
        /// The register to add; only `help` exists today.
        register: String,
    },
    /// Print the closed frontmatter-key vocabulary a case may declare.
    Keys,
}

/// The arguments `compile` and `promote` share. `compile` refuses the publish-only three itself,
/// with words that say why a verb which writes nothing marks nothing.
#[derive(Args, Clone, PartialEq, Eq, Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "a flag set is independent booleans by construction; folding them into a state \
              machine would hide which spellings exist from the derive that defines them"
)]
pub struct PublishArgs {
    /// Drop the header of every case that has nothing to report.
    #[arg(long)]
    pub quiet: bool,
    /// Lend the generic executor a shell, for a replay the in-process driver declines.
    #[arg(long)]
    pub shell: Option<String>,
    /// Prepend a directory to the replay PATH (repeatable).
    #[arg(long)]
    pub path: Vec<String>,
    /// Acknowledge that a case's metadata REPLACES the committed registry entry.
    #[arg(long)]
    pub accept_metadata: bool,
    /// Mark every register this publishes as written by a person.
    #[arg(long)]
    pub human: bool,
    /// Re-mark a human-written register as slop, deliberately.
    #[arg(long)]
    pub slop: bool,
    /// The cases; empty means the whole collection.
    pub cases: Vec<String>,
}

/// `vars`' own arguments. Mode is OPTIONAL: `--used` is the default breadth and `--all` the
/// explicit widening (`30C:rul-used-is-the-default-breadth`).
#[derive(Args, Clone, PartialEq, Eq, Debug)]
pub struct VarsArgs {
    /// Only the variables some rendered section actually consumes (the default).
    #[arg(long)]
    pub used: bool,
    /// The whole typed payload, including values no sentence spends yet.
    #[arg(long, conflicts_with = "used")]
    pub all: bool,
    /// The cases; empty means the whole collection.
    pub cases: Vec<String>,
}

impl VarsArgs {
    /// The breadth this invocation asked for.
    #[must_use]
    pub fn breadth(&self) -> Breadth {
        if self.all {
            Breadth::All
        } else {
            Breadth::Used
        }
    }
}

/// `sections`' own arguments.
#[derive(Args, Clone, PartialEq, Eq, Debug)]
pub struct SectionsArgs {
    /// The cases; empty means the whole collection.
    pub cases: Vec<String>,
}

impl Invocation {
    /// The verb's own name, its case list, and whether it takes a target selector — the three
    /// things every seat asks of a parsed verb.
    fn shape(&self) -> (&'static str, &[String], bool) {
        match &self.verb {
            Verb::Compile(args) => ("compile", &args.cases, false),
            Verb::Promote(args) => ("promote", &args.cases, false),
            Verb::Vars(args) => ("vars", &args.cases, true),
            Verb::Sections(args) => ("sections", &args.cases, true),
            Verb::Scaffold { .. } => ("scaffold", &[], false),
            Verb::AddRegister { .. } => ("add-register", &[], false),
            Verb::Keys => ("keys", &[], false),
        }
    }

    /// The verb this invocation spells, for a caller that wants its usage page.
    #[must_use]
    pub fn verb_name(&self) -> &'static str {
        self.shape().0
    }

    /// Where this invocation's cases come from.
    ///
    /// # Errors
    /// Refuses `--this` on a verb that takes no target, and `--this` alongside a named case: both
    /// are a caller saying two different things about which case they mean.
    pub fn target(&self) -> Result<Target<'_>, String> {
        let (name, cases, takes_selector) = self.shape();
        if !self.this {
            return Ok(Target::Named(cases));
        }
        if !takes_selector {
            return Err(format!(
                "{THIS} selects one case, and {SELECTOR_VERBS} are the verbs that take a \
                 selector; `{name}` does not. Drop it: `dorc-loom {name}`"
            ));
        }
        if cases.is_empty() {
            return Ok(Target::This);
        }
        Err(format!(
            "{THIS} means the case this invocation is running inside, and the case list names \
             another; pass one or the other: `dorc-loom {name} {}`",
            cases.join(" ")
        ))
    }
}

/// The global selector flag, spelled once.
pub const THIS: &str = "--this";

const SELECTOR_VERBS: &str = "`vars` and `sections`";

/// Parse a `dorc-loom` command line — the program name included, as argv arrives.
///
/// # Errors
/// Returns the refusal text, with no usage page attached: a caller that knows which verb the reader
/// had already chosen appends that verb's own page (`28L:rul-refusals-name-the-next-command`).
pub fn parse<I, T>(argv: I) -> Result<Invocation, String>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let invocation = Invocation::try_parse_from(argv).map_err(|error| explain(&error))?;
    invocation.target()?;
    Ok(invocation)
}

/// Which verb owns a flag, for the reader who put it in the wrong place.
///
/// A flag in the global position is the misuse worth naming (`dorc-loom --all vars`): clap can say
/// only that the argument was unexpected, and "unexpected" is exactly what a reader who believes
/// they typed a real flag cannot act on.
const FLAG_OWNERS: [(&str, &str); 8] = [
    ("--all", "vars"),
    ("--used", "vars"),
    ("--quiet", "compile and promote"),
    ("--shell", "compile and promote"),
    ("--path", "compile and promote"),
    ("--accept-metadata", "promote"),
    ("--human", "promote"),
    ("--slop", "promote"),
];

fn explain(error: &clap::Error) -> String {
    // ONLY an unknown argument is a misplacement. A conflict names its OTHER operand in the same
    // context slot, so reading the table on every error kind turns `--used --all` into advice
    // about where `--used` goes.
    let unknown = (error.kind() == clap::error::ErrorKind::UnknownArgument)
        .then(|| error.get(clap::error::ContextKind::InvalidArg))
        .flatten()
        .map(ToString::to_string);
    let flag = unknown
        .as_deref()
        .map(|arg| arg.split_once('=').map_or(arg, |(name, _)| name));
    if flag == Some(THIS) {
        return format!("{THIS} comes before the verb, git-style: `dorc-loom {THIS} vars`");
    }
    if let Some((flag, owner)) = flag.and_then(|flag| {
        FLAG_OWNERS
            .iter()
            .find(|(name, _)| *name == flag)
            .map(|(name, owner)| (*name, *owner))
    }) {
        return format!(
            "{flag} belongs to {owner} and goes after the verb; {THIS} is the only flag that comes \
             before one: `dorc-loom {} {flag}`",
            owner.split(' ').next().unwrap_or(owner)
        );
    }
    error.render().to_string().trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_words(words: &[&str]) -> Result<Invocation, String> {
        parse(words.iter().copied())
    }

    /// The git shape, both verbs, both breadths: `--this` before the verb, mode after it.
    #[test]
    fn the_selector_is_global_and_the_breadth_belongs_to_vars() {
        let vars = parse_words(&["dorc-loom", "--this", "vars"]).expect("the git shape parses");
        assert!(vars.this);
        assert_eq!(vars.target().expect("a target"), Target::This);
        let Verb::Vars(args) = &vars.verb else {
            panic!("vars parses as vars")
        };
        assert_eq!(args.breadth(), Breadth::Used);

        let widened =
            parse_words(&["dorc-loom", "--this", "vars", "--all"]).expect("the widening parses");
        let Verb::Vars(args) = &widened.verb else {
            panic!("vars parses as vars")
        };
        assert_eq!(args.breadth(), Breadth::All);

        let sections =
            parse_words(&["dorc-loom", "--this", "sections"]).expect("sections takes it too");
        assert_eq!(sections.target().expect("a target"), Target::This);
    }

    /// Mode is optional and `--used` is the default, so the bare form and the explicit one are the
    /// same invocation.
    #[test]
    fn a_bare_vars_means_used() {
        for spelling in [
            ["dorc-loom", "vars", "whylog-absent"],
            ["dorc-loom", "vars", "--used"],
        ] {
            let Verb::Vars(args) = parse_words(&spelling).expect("parses").verb else {
                panic!("vars parses as vars")
            };
            assert_eq!(args.breadth(), Breadth::Used);
        }
        assert!(
            parse_words(&["dorc-loom", "vars", "--used", "--all"])
                .is_err_and(|refusal| refusal.contains("--all")),
            "the two breadths contradict each other"
        );
    }

    /// A subcommand flag in the global position must name the verb it belongs to: clap can only
    /// say "unexpected", which a reader who believes they typed a real flag cannot act on.
    #[test]
    fn a_subcommand_flag_in_the_global_position_names_its_verb() {
        let refusal = parse_words(&["dorc-loom", "--all", "vars"]).expect_err("refuses");
        assert!(refusal.contains("--all"), "{refusal}");
        assert!(refusal.contains("vars"), "{refusal}");
        assert!(refusal.contains("dorc-loom vars --all"), "{refusal}");

        let promote = parse_words(&["dorc-loom", "--human", "promote"]).expect_err("refuses");
        assert!(promote.contains("promote"), "{promote}");

        let inverted = parse_words(&["dorc-loom", "vars", "--this"]).expect_err("refuses");
        assert!(inverted.contains("dorc-loom --this vars"), "{inverted}");
    }

    /// `--this` and a case list say two different things about which case is meant, and a verb
    /// with no target has nothing for a selector to select.
    #[test]
    fn the_selector_refuses_a_second_target_and_a_targetless_verb() {
        let both = parse_words(&["dorc-loom", "--this", "vars", "whylog-absent"])
            .expect_err("two targets refuse");
        assert!(both.contains("whylog-absent"), "{both}");

        let keys = parse_words(&["dorc-loom", "--this", "keys"]).expect_err("keys takes no target");
        assert!(keys.contains("vars"), "{keys}");
    }

    /// Every other verb still parses, so the driver's shared grammar is the binary's whole grammar
    /// rather than a second one that happens to overlap.
    #[test]
    fn the_remaining_verbs_keep_their_own_arguments() {
        assert!(matches!(
            parse_words(&["dorc-loom", "scaffold", "new-slug"]).expect("parses").verb,
            Verb::Scaffold { slug } if slug == "new-slug"
        ));
        assert!(matches!(
            parse_words(&["dorc-loom", "add-register", "some-case", "help"])
                .expect("parses")
                .verb,
            Verb::AddRegister { register, .. } if register == "help"
        ));
        assert!(matches!(
            parse_words(&["dorc-loom", "keys"]).expect("parses").verb,
            Verb::Keys
        ));
        let Verb::Promote(args) = parse_words(&[
            "dorc-loom",
            "promote",
            "--quiet",
            "--shell=/bin/sh",
            "--path",
            "mocks",
            "a-case",
        ])
        .expect("parses")
        .verb
        else {
            panic!("promote parses as promote")
        };
        assert!(args.quiet);
        assert_eq!(args.shell.as_deref(), Some("/bin/sh"));
        assert_eq!(args.path, vec!["mocks".to_owned()]);
        assert_eq!(args.cases, vec!["a-case".to_owned()]);
    }
}
