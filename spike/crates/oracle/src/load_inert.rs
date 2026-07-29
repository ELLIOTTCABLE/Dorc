//! `load_inert` — the marked-file load-inertness gate (`28K` §2a
//! `rul-marked-file-is-load-inert`; the implementation of this crate's standing
//! `declarations-only-files` law).
//!
//! A `# dorc-lang/v0.2`-marked file must be provably no-op to LOAD: its top level holds function
//! definitions and bare assignments, never commands. Two consumers rest on that. `28K` §2's
//! abstract function-environment domain models a `.`-source as "apply this file's definitions
//! here", which is only a total model when the file cannot also *run* something the domain would
//! have to interpret; and `28K` §3's regional-preference idiom (`( . better-yum.sh; … )`)
//! re-sources a file for real, at apply time, inside a subshell — so a top-level command there is
//! a mutation nobody licensed.
//!
//! Marker-gated, per `marker-gates-syntax-only`: an UNMARKED `.sh` is ordinary shell that happens
//! to be loadable, makes no dialect claim, and is nobody's oracle.

use dorc_aid::Diag;
use dorc_aid::diag::{DiagCode, OracleFileNotLoadInert};
use dorc_core::AstId;
use dorc_syntax::ast::{Ast, NodeKind, WordPart};

/// Refuse a marked `src` whose top level is not provably inert to load.
///
/// AT MOST ONE diagnostic, spanned at the FIRST offending item. Load-inertness is a property of
/// the FILE — one claim, one world-state, one remediation ("make this file definitions-only") —
/// so reporting it per-item would be a correlated cascade, and a book mislabelled as an oracle
/// would bury every other finding under one line per command (`AGENTS.md` fail-fast: only
/// root-cause is reported; `AID-NEEDS:law-codes-vary-by-world-not-grammar`). Total
/// (`inv-no-throw`): an unparseable file yields whatever the fail-soft parse salvaged, and its
/// own parse diagnostics are the caller's separate concern.
#[must_use]
pub fn lint_load_inert(src: &str) -> Vec<Diag> {
    if !crate::marker::has_marker(src) {
        return Vec::new();
    }
    let ast = dorc_syntax::parse(src).value;
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return Vec::new();
    };
    items
        .iter()
        .find(|&&item| !item_is_load_inert(&ast, item))
        .map(|&item| {
            Diag::new(
                DiagCode::OracleFileNotLoadInert(OracleFileNotLoadInert),
                ast.node(item).span,
            )
        })
        .into_iter()
        .collect()
}

/// Is one top-level item a no-op to load? Only two shapes are: a function DEFINITION (defining
/// binds a name and runs nothing) and a BARE ASSIGNMENT whose value expands statically — the AST
/// spells the latter as a `Simple` with no `words` (`syntax::ast`'s own doc). A redirection makes
/// even a wordless command a write (`: > /etc/x` is the standing example), so it disqualifies too.
/// Everything else — a command, a pipeline, a conditional, a loop, a subshell, a ⊤-rejected
/// construct — is refused: `inv-top-reject` biases the unknown toward refusal, and relaxing this
/// later is cheap where re-tightening would not be (`271:rul-posix-in-spirit-defaults`).
fn item_is_load_inert(ast: &Ast, item: AstId) -> bool {
    match &ast.node(item).kind {
        NodeKind::FuncDef { .. } => true,
        NodeKind::Simple {
            assigns,
            words,
            redirs,
        } => {
            words.is_empty()
                && redirs.is_empty()
                && assigns.iter().all(|&a| assign_value_is_static(ast, a))
        }
        _ => false,
    }
}

/// Does an assignment's value expand without running anything? An absent value (`FOO=`) trivially
/// does; otherwise every fragment of the value word must be static.
fn assign_value_is_static(ast: &Ast, assign: AstId) -> bool {
    let NodeKind::Assign { value, .. } = &ast.node(assign).kind else {
        return false;
    };
    let Some(word) = value else { return true };
    let NodeKind::Word { parts } = &ast.node(*word).kind else {
        return false;
    };
    parts.iter().all(word_part_is_static)
}

/// A word fragment that cannot run a command. `ParamComplex` is refused alongside the obvious
/// two because the lexer collapses EVERY operator form to that one opaque part and discards the
/// body, so `${x:-$(hostname)}` is indistinguishable from `${x:-literal}` here — accepting it
/// would accept a hidden command substitution, which is exactly the claim this gate makes.
fn word_part_is_static(part: &WordPart) -> bool {
    match part {
        WordPart::Literal(_) | WordPart::SingleQuoted(_) | WordPart::Param { .. } => true,
        WordPart::DoubleQuoted(inner) => inner.iter().all(word_part_is_static),
        WordPart::CommandSubst(_) | WordPart::Arithmetic | WordPart::ParamComplex => false,
    }
}

#[cfg(test)]
mod tests {
    use super::lint_load_inert;

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn slugs(body: &str) -> Vec<&'static str> {
        lint_load_inert(&format!("{MARKER}{body}"))
            .iter()
            .map(|d| d.code.slug())
            .collect()
    }

    /// The shapes an oracle file is BUILT from: role funcdefs, helper funcdefs, and the
    /// file-global constants `rul-marked-file-is-load-inert` calls a must-have. A false positive
    /// here would refuse the whole corpus, so this is the load-bearing negative pin.
    #[test]
    fn definitions_and_bare_assignments_are_inert() {
        let body = "\
CERTS=/etc/nginx/certs
EMPTY=
QUOTED=\"$CERTS/live\"
apt_get__is_converged() { dpkg-query -W \"$1\"; }
_helper() { printf '%s\\n' \"$1\"; }
";
        assert!(slugs(body).is_empty(), "{:?}", slugs(body));
    }

    /// A top-level command is the thing the gate exists to refuse — loading the file would run it.
    #[test]
    fn a_top_level_command_is_refused() {
        assert_eq!(slugs("apt-get update\n"), ["oracle-file-not-load-inert"]);
    }

    /// ONE diagnostic however many items offend, and it points at the FIRST. Load-inertness is a
    /// property of the file, so per-item reporting would be a correlated cascade with a single
    /// remediation — and a book mislabelled as an oracle would bury every neighbouring finding
    /// under one error per command (measured: five, on the `unmodeled-wall-inventory` case).
    #[test]
    fn the_whole_file_reports_once_at_the_first_offender() {
        let body = "hork tune\nf() { :; }\nwombat sync\napt-get install -y nginx\n";
        let diags = lint_load_inert(&format!("{MARKER}{body}"));
        assert_eq!(diags.len(), 1, "{diags:?}");
        let span = diags[0].primary.span().expect("spanned at an item");
        let head = format!("{MARKER}hork tune");
        assert_eq!(
            (span.lo.0 as usize, span.hi.0 as usize),
            (MARKER.len(), head.len()),
            "the span covers the FIRST offending item"
        );
    }

    /// `293` §4's first named edge: an assignment is a command in disguise when its value runs
    /// one. Arithmetic expansion is the same shape at a different spelling.
    #[test]
    fn an_assignment_that_runs_a_command_is_refused() {
        assert_eq!(slugs("CERTS=$(hostname)\n"), ["oracle-file-not-load-inert"]);
        assert_eq!(
            slugs("HOST=\"prefix-$(hostname)\"\n"),
            ["oracle-file-not-load-inert"],
            "a substitution nested inside double quotes is still a command"
        );
        assert_eq!(slugs("N=$((1 + 1))\n"), ["oracle-file-not-load-inert"]);
    }

    /// The operator form of parameter expansion is refused because the lexer cannot see inside it
    /// — `${x:-$(hostname)}` lexes identically to `${x:-lit}`, so accepting the shape would accept
    /// the substitution. Conservative-first; relaxing needs the lexer to keep the body.
    #[test]
    fn an_operator_parameter_expansion_is_refused() {
        assert_eq!(
            slugs("ROOT=${DORC_ROOT:-/etc}\n"),
            ["oracle-file-not-load-inert"]
        );
        assert!(
            slugs("ROOT=${DORC_ROOT}\n").is_empty(),
            "the simple braced form stays inert — the lexer resolves it to a plain Param"
        );
    }

    /// `293` §4's second named edge: `export`/`readonly` are commands by AST shape, and stay
    /// refused for now rather than being special-cased into the permitted set.
    #[test]
    fn export_and_readonly_are_refused() {
        assert_eq!(slugs("export FOO=bar\n"), ["oracle-file-not-load-inert"]);
        assert_eq!(slugs("readonly FOO=bar\n"), ["oracle-file-not-load-inert"]);
    }

    /// A redirection writes whatever the command word does — the standing `haz-redir-as-mutation`
    /// example is a no-op command that truncates a file.
    #[test]
    fn a_wordless_redirect_is_refused() {
        assert_eq!(slugs(": > /etc/x\n"), ["oracle-file-not-load-inert"]);
    }

    /// Control flow at top level is refused whole: a conditional load is exactly the
    /// host-dependent shape `28K` §1 rul-unloadable-is-unlicensed walls, and a subshell or loop
    /// can run anything.
    #[test]
    fn top_level_control_flow_is_refused() {
        for body in [
            "if [ -f /etc/x ]; then f() { :; }; fi\n",
            "( f() { :; } )\n",
            "for x in a b; do echo $x; done\n",
            ". ./other.sh\n",
        ] {
            assert_eq!(slugs(body).len(), 1, "{body}");
        }
    }

    /// Marker-gated (`marker-gates-syntax-only`): an unmarked file makes no dialect claim, so the
    /// gate says nothing about it however it is written. Without this the check would fire on
    /// every ordinary shell script the tool is ever pointed at.
    #[test]
    fn an_unmarked_file_is_out_of_scope() {
        assert!(lint_load_inert("apt-get update\nexport FOO=bar\n").is_empty());
    }

    /// The degenerate inputs stay silent rather than panicking (`inv-no-throw`).
    #[test]
    fn empty_and_marker_only_files_are_silent() {
        assert!(lint_load_inert("").is_empty());
        assert!(lint_load_inert(MARKER).is_empty());
    }
}
