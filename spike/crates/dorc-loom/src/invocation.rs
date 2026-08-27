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
    /// Resolve the corpus, both generated locks and the staging store relative to this directory
    /// instead of the tree this binary was built in.
    ///
    /// Global for the same reason `--this` is, and spelled as git's and make's: it says which
    /// WORLD the command acts on, which is not one verb's business.
    #[arg(short = 'C', value_name = "DIR")]
    pub root: Option<String>,
    /// What to do.
    #[command(subcommand)]
    pub verb: Verb,
}

/// The verbs, each with its own arguments.
#[derive(Subcommand, Clone, PartialEq, Eq, Debug)]
pub enum Verb {
    /// Compile the prose edits back into template form and publish both locks and the cases.
    Publish(PublishArgs),
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
    /// Render the current case's explicitly authorized internal-defect diagnostic.
    Defect,
}

/// `publish`' own arguments.
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
    /// Apply an interpretation that gives up holes, exactly as it was already shown.
    #[arg(long)]
    pub verbatim: bool,
    /// Every committed case — the explicit, dangerous whole-corpus target.
    #[arg(long)]
    pub all: bool,
    /// The cases; `--all` is how the whole collection is asked for.
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

/// What every seat asks of a parsed verb.
struct Shape<'a> {
    name: &'static str,
    cases: &'a [String],
    takes_selector: bool,
    /// Whether a bare, targetless invocation legally means the whole collection. The READ-ONLY
    /// verbs say yes; `publish` MUTATES, so it makes the reader spell the whole corpus out
    /// (`--all`) rather than reaching it by omission.
    bare_means_everything: bool,
}

impl Invocation {
    fn shape(&self) -> Shape<'_> {
        let shape = |name, cases, takes_selector| Shape {
            name,
            cases,
            takes_selector,
            bare_means_everything: true,
        };
        match &self.verb {
            Verb::Publish(args) => Shape {
                bare_means_everything: args.all,
                ..shape("publish", &args.cases, false)
            },
            Verb::Vars(args) => shape("vars", &args.cases, true),
            Verb::Sections(args) => shape("sections", &args.cases, true),
            Verb::Scaffold { .. } => shape("scaffold", &[], false),
            Verb::AddRegister { .. } => shape("add-register", &[], false),
            Verb::Keys => shape("keys", &[], false),
            Verb::Defect => Shape {
                bare_means_everything: false,
                ..shape("defect", &[], true)
            },
        }
    }

    /// The verb this invocation spells, for a caller that wants its usage page.
    #[must_use]
    pub fn verb_name(&self) -> &'static str {
        self.shape().name
    }

    /// Where this invocation's cases come from.
    ///
    /// # Errors
    /// Refuses `--this` on a verb that takes no target, `--this` alongside a named case (both are a
    /// caller saying two different things about which case they mean), and a bare mutating verb.
    pub fn target(&self) -> Result<Target<'_>, String> {
        let Shape {
            name,
            cases,
            takes_selector,
            bare_means_everything,
        } = self.shape();
        if matches!(self.verb, Verb::Defect) && !self.this {
            return Err(format!(
                "`defect` is a loom-bound harness route and requires {THIS}: `dorc-loom {THIS} defect`"
            ));
        }
        if !self.this {
            if cases.is_empty() && !bare_means_everything {
                return Err(format!(
                    "`{name}` rewrites the generated locks and every case it touches, so it takes \
                     the cases it may rewrite: `dorc-loom {name} <slug>`. The whole corpus is \
                     {ALL}, spelled out."
                ));
            }
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

/// The global world flag, spelled once.
pub const ROOT: &str = "-C";

/// The flags that live before the verb, each with the invocation a reader who misplaced it should
/// type instead (`28L:rul-refusals-name-the-next-command`).
const GLOBAL_FLAGS: [(&str, &str); 2] = [
    (THIS, "dorc-loom --this vars"),
    (ROOT, "dorc-loom -C DIR publish <slug>"),
];

/// `publish`' whole-corpus opt-in, spelled once.
pub const ALL: &str = "--all";

const SELECTOR_VERBS: &str = "`vars`, `sections`, and the loom-only `defect` route";

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

/// The verbs that declare `flag`, asked of the grammar rather than of a table beside it — a
/// hand-listed table drifts the moment a verb gains a flag, and the drift is invisible.
fn verbs_declaring(flag: &str) -> Vec<String> {
    let name = flag.trim_start_matches('-');
    <Invocation as clap::CommandFactory>::command()
        .get_subcommands()
        .filter(|verb| verb.get_arguments().any(|arg| arg.get_long() == Some(name)))
        .map(|verb| verb.get_name().to_owned())
        .collect()
}

fn explain(error: &clap::Error) -> String {
    // ONLY an unknown argument is a misplacement. A conflict names its OTHER operand in the same
    // context slot, so consulting the grammar on every error kind turns `--used --all` into advice
    // about where `--used` goes.
    let unknown = (error.kind() == clap::error::ErrorKind::UnknownArgument)
        .then(|| error.get(clap::error::ContextKind::InvalidArg))
        .flatten()
        .map(ToString::to_string);
    let Some(flag) = unknown
        .as_deref()
        .map(|arg| arg.split_once('=').map_or(arg, |(name, _)| name))
    else {
        return error.render().to_string().trim_end().to_owned();
    };
    if let Some((global, example)) = GLOBAL_FLAGS.into_iter().find(|(global, _)| flag == *global) {
        return format!("{global} comes before the verb, git-style: `{example}`");
    }
    let verbs = verbs_declaring(flag);
    match verbs.first() {
        None => error.render().to_string().trim_end().to_owned(),
        Some(first) => format!(
            "{flag} belongs to {} and goes after the verb; the global flags {THIS} and {ROOT} are \
             the only ones that come before one: `dorc-loom {first} {flag}`",
            verbs.join(" and ")
        ),
    }
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
    /// say "unexpected", which a reader who believes they typed a real flag cannot act on. Two
    /// verbs declare `--all`, and the refusal names both rather than picking one.
    #[test]
    fn a_subcommand_flag_in_the_global_position_names_its_verb() {
        let refusal = parse_words(&["dorc-loom", "--all", "vars"]).expect_err("refuses");
        assert!(refusal.contains("--all"), "{refusal}");
        assert!(refusal.contains("vars"), "{refusal}");
        assert!(refusal.contains("publish"), "{refusal}");

        let published =
            parse_words(&["dorc-loom", "--human", "publish", "a-case"]).expect_err("refuses");
        assert!(published.contains("publish"), "{published}");

        let inverted = parse_words(&["dorc-loom", "vars", "--this"]).expect_err("refuses");
        assert!(inverted.contains("dorc-loom --this vars"), "{inverted}");
    }

    /// `publish` MUTATES, so omitting the target is a misuse rather than a shorthand; `--all` is
    /// how the whole corpus is asked for, and the refusal says so.
    #[test]
    fn a_bare_publish_refuses_and_names_the_whole_corpus_flag() {
        let bare = parse_words(&["dorc-loom", "publish"]).expect_err("a bare publish refuses");
        assert!(bare.contains("--all"), "{bare}");
        assert!(bare.contains("dorc-loom publish <slug>"), "{bare}");

        let named = parse_words(&["dorc-loom", "publish", "a-case"]).expect("a named case parses");
        assert_eq!(
            named.target().expect("a target"),
            Target::Named(&["a-case".to_owned()])
        );

        let whole = parse_words(&["dorc-loom", "publish", "--all"]).expect("--all parses");
        assert_eq!(whole.target().expect("a target"), Target::Named(&[]));

        // A read-only verb keeps bare-means-everything, and this is what says the two did not
        // drift into one rule.
        assert_eq!(
            parse_words(&["dorc-loom", "vars"])
                .expect("a bare vars parses")
                .target()
                .expect("a target"),
            Target::Named(&[])
        );
    }

    /// `-C` is git-shaped: before the verb, on every verb (the world a command acts on is not one
    /// verb's business), and misplaced it names where it goes rather than clap's "unexpected".
    #[test]
    fn the_world_flag_is_global_and_reaches_every_verb() {
        for spelling in [
            vec!["dorc-loom", "-C", "/tmp/world", "publish", "--all"],
            vec!["dorc-loom", "-C", "/tmp/world", "vars"],
            vec!["dorc-loom", "-C", "/tmp/world", "keys"],
        ] {
            let invocation = parse_words(&spelling).expect("the git shape parses");
            assert_eq!(
                invocation.root.as_deref(),
                Some("/tmp/world"),
                "{spelling:?}"
            );
        }
        assert_eq!(
            parse_words(&["dorc-loom", "vars"])
                .expect("the bare form parses")
                .root,
            None,
            "an absent -C is the tree this binary was built in"
        );

        let inverted =
            parse_words(&["dorc-loom", "vars", "-C", "/tmp/world"]).expect_err("refuses");
        assert!(inverted.contains("dorc-loom -C DIR"), "{inverted}");
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
        assert!(matches!(
            parse_words(&["dorc-loom", "--this", "defect"])
                .expect("the loom-only route parses")
                .verb,
            Verb::Defect
        ));
        assert!(parse_words(&["dorc-loom", "defect"]).is_err());
        let Verb::Publish(args) = parse_words(&[
            "dorc-loom",
            "publish",
            "--quiet",
            "--verbatim",
            "--shell=/bin/sh",
            "--path",
            "mocks",
            "a-case",
        ])
        .expect("parses")
        .verb
        else {
            panic!("publish parses as publish")
        };
        assert!(args.quiet);
        assert!(args.verbatim);
        assert_eq!(args.shell.as_deref(), Some("/bin/sh"));
        assert_eq!(args.path, vec!["mocks".to_owned()]);
        assert_eq!(args.cases, vec!["a-case".to_owned()]);
    }
}
