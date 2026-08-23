//! `load_inert` — the marked-file load-inertness gate (`28K` §2a
//! `rul-marked-file-is-load-inert`; the implementation of this crate's standing
//! `declarations-only-files` law).
//!
//! A `# dorc-lang/v0.2`-marked file's top level holds function definitions, bare assignments, and
//! `.`-sources — never commands. Two consumers rest on that. `28K` §2's abstract
//! function-environment domain models a `.`-source as "apply this file's definitions here", which
//! is only a total model when the file cannot also *run* something the domain would have to
//! interpret; and `28K` §3's regional-preference idiom (`( . better-yum.sh; … )`) re-sources a file
//! for real, at apply time, inside a subshell — so a top-level command there is a mutation nobody
//! licensed.
//!
//! # This is a CONTRACT, not an engine proof
//!
//! [TYPED 2026-08-17, `307:§ack-implementation-open`] Load-inertness is HYGIENE and CONTRACT, never
//! an engine fact. What this gate does is hold an author to the dorc-lang promise their marker
//! makes — a marked file declares and sources, and runs nothing at load. Nothing downstream may
//! claim the engine PROVED a file inert, and a refusal here attributes to the contract the author
//! signed by marking the file, not to an analysis that came up short.
//!
//! The distinction is load-bearing because the admission below now includes `.`
//! (`28Q:pin-oracle-side-sourcing-amendment`), which is exactly a construct whose inertness the
//! engine cannot see: whether the sourced file runs anything is that file's own contract to keep,
//! checked the same way, one level down.
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

/// Is one top-level item admissible in a marked file? Three shapes are: a function DEFINITION
/// (defining binds a name and runs nothing), a BARE ASSIGNMENT whose value expands statically — the
/// AST spells the latter as a `Simple` with no `words` (`syntax::ast`'s own doc) — and a statically
/// spelled `.` ([`item_is_static_load`]). A redirection makes even a wordless command a write
/// (`: > /etc/x` is the standing example), so it disqualifies too. Everything else — a command, a
/// pipeline, a conditional, a loop, a subshell, a ⊤-rejected construct — is refused: `inv-top-reject`
/// biases the unknown toward refusal, and relaxing this later is cheap where re-tightening would not
/// be (`271:rul-posix-in-spirit-defaults`).
pub(crate) fn item_is_load_inert(ast: &Ast, item: AstId) -> bool {
    match &ast.node(item).kind {
        NodeKind::FuncDef { .. } => true,
        NodeKind::Simple {
            assigns,
            words,
            redirs,
        } => {
            redirs.is_empty()
                && assigns.iter().all(|&a| assign_value_is_static(ast, a))
                && (words.is_empty()
                    || (assigns.is_empty()
                        && (static_load_target(ast, words)
                            || unset_functions(ast, words).is_some())))
        }
        NodeKind::If { .. } => include_guard(ast, item).is_some(),
        _ => false,
    }
}

/// The ONE conditional shape a marked file's top level may carry: an INCLUDE GUARD
/// (`30I:rul-include-guards-are-load-semantics`, TYPED — "mandatory language surface, not optional
/// polish").
///
/// Independent oracle authors share dependencies with it, and it says: use the higher-quality live
/// implementation when it is present, otherwise load this fallback. The function environment,
/// custody, emission, and the bundle compiler all have to agree on that branch, so the shape is
/// closed rather than general.
///
/// # Why the branches may not DEFINE
///
/// A role funcdef inside a conditional branch is a MEASURED wrong-elision route, not a taste
/// (`oracle/CLAUDE.md only-load-inert-sources-contribute`, "INERTNESS IS DYING IN LITERAL"): the
/// dialect lift recognizes a role header only as a TOP-LEVEL ITEM, so a definition nested in a
/// branch is registered by `dorc_syntax` and by the definition table while producing ZERO lifted
/// rows and zero detected headers — silently, with the marks-lost backstop quiet about it too. The
/// pins are `sh_parity.rs`'s `a_host_conditional_oracle_definition_licenses_nothing` and its
/// expected-fail twin, and widening the allow-list WITHOUT making that binding `May` is exactly
/// what they forbid. So a guard's branches carry loads and removals; declarations stay at top
/// level, where they are seen.
#[must_use]
pub fn include_guard(ast: &Ast, item: AstId) -> Option<IncludeGuard> {
    let NodeKind::If {
        cond,
        then_body,
        elifs,
        else_body,
    } = &ast.node(item).kind
    else {
        return None;
    };
    // `elif` is a second condition, and the loader models ONE. An author wanting two guards nests
    // them, which this gate does admit.
    if !elifs.is_empty() {
        return None;
    }
    let (condition, negated) = guard_condition(ast, *cond)?;
    let then_ = guard_branch(ast, *then_body)?;
    let else_ = match else_body {
        Some(branch) => guard_branch(ast, *branch)?,
        None => Vec::new(),
    };
    Some(IncludeGuard {
        condition,
        negated,
        then_,
        else_,
    })
}

/// What a recognized include guard ASKS. Two species, and the difference is the whole of
/// `30I` §2.2: one asks the host what it would resolve under a name, the other asks this shell
/// what a variable holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardCondition {
    /// `command -v <name>` — what a shell would resolve under that name. Idiomatic, supported,
    /// and NOT exact package identity: its answer space spans functions, aliases, builtins,
    /// reserved words and `PATH` utilities, and the pinned floors do not even agree on the
    /// classification (`notes/30Ic`; `30I:pin-command-v-load-model`).
    CommandV {
        /// The function name the guard asks about.
        function: String,
    },
    /// `[ "${name-}" = 'literal' ]` — the package sentinel a library populates when it loads
    /// (`30I` §2.2). No `PATH` exposure, no builtin classification, and the floors agree on all
    /// three states (`30Ic:obs-sentinel-floor-semantics-agree`).
    ///
    /// The name and the literal are the AUTHOR's own package interface. Dorc recognizes no
    /// `_LOADED` suffix, namespace, version grammar, or distinguished value.
    Value {
        /// The variable the test reads.
        name: String,
        /// The literal it is compared against.
        literal: String,
        /// `=` rather than `!=`.
        equals: bool,
    },
}

/// A recognized include guard, decomposed for the loader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeGuard {
    /// What the guard asks.
    pub condition: GuardCondition,
    /// Whether the condition is `!`-negated, so a consumer reads the branches the right way round.
    pub negated: bool,
    /// The branch taken when the guard's condition SUCCEEDS, in source order.
    pub then_: Vec<AstId>,
    /// The branch taken when it fails. Empty for a guard with no `else`.
    pub else_: Vec<AstId>,
}

/// The guard's question, with the `!`-negation the shape carries and whatever redirections the
/// author silenced it with.
///
/// Redirections are permitted HERE and nowhere else in this gate: `>/dev/null 2>&1` on a query is
/// the idiom every author writes, and a redirect on a QUERY writes nothing the way a redirect on a
/// no-op command does (`: > /etc/x`, the standing `haz-redir-as-mutation` example). Every operand
/// must be one plain literal — a computed one would ask the loader a question it cannot answer.
fn guard_condition(ast: &Ast, cond: AstId) -> Option<(GuardCondition, bool)> {
    let mut id = cond;
    let mut negated = false;
    for _ in 0..GUARD_UNWRAP_CAP {
        match &ast.node(id).kind {
            NodeKind::Script { items } | NodeKind::List { items } => {
                let [only] = items[..] else { return None };
                id = only;
            }
            NodeKind::Pipeline {
                negated: flip,
                stages,
            } => {
                let [only] = stages[..] else { return None };
                negated ^= flip;
                id = only;
            }
            NodeKind::Simple { assigns, words, .. } => {
                if !assigns.is_empty() {
                    return None;
                }
                return command_v(ast, words)
                    .or_else(|| sentinel_test(ast, words))
                    .map(|condition| (condition, negated));
            }
            _ => return None,
        }
    }
    None
}

/// `command -v <NAME>`.
fn command_v(ast: &Ast, words: &[AstId]) -> Option<GuardCondition> {
    let [head, flag, name] = words[..] else {
        return None;
    };
    if literal_word(ast, head)? != "command" || literal_word(ast, flag)? != "-v" {
        return None;
    }
    Some(GuardCondition::CommandV {
        function: literal_word(ast, name)?.to_owned(),
    })
}

/// `[ "${name-}" = 'literal' ]` / `test "${name-}" != 'literal'`, in either operand order.
///
/// The variable side must be the NOUNSET-SAFE spelling (`${name-}` or `${name:-}`) — the one form
/// whose body cannot hide a command substitution, and the one an author writing for a `set -u`
/// caller writes anyway. A bare `"$name"` is deliberately NOT admitted: it aborts the loading
/// shell under `set -u`, so admitting it would bless an idiom the floor breaks on.
fn sentinel_test(ast: &Ast, words: &[AstId]) -> Option<GuardCondition> {
    let operands = match (words, words.first().and_then(|&w| literal_word(ast, w))) {
        ([_, a, op, b, close], Some("[")) if literal_word(ast, *close)? == "]" => [*a, *op, *b],
        ([_, a, op, b], Some("test")) => [*a, *op, *b],
        _ => return None,
    };
    let [left, op, right] = operands;
    let equals = match literal_word(ast, op)? {
        "=" => true,
        "!=" => false,
        _ => return None,
    };
    // Either order: the variable may sit on either side of the comparison, and a test with a
    // variable on BOTH sides compares nothing this pass can read.
    let (name, literal) = match (sentinel_name(ast, left), sentinel_name(ast, right)) {
        (Some(name), None) => (name, plain_text(ast, right)?),
        (None, Some(name)) => (name, plain_text(ast, left)?),
        _ => return None,
    };
    Some(GuardCondition::Value {
        name,
        literal,
        equals,
    })
}

/// The variable a `"${name-}"` operand reads, or `None` for any other word.
fn sentinel_name(ast: &Ast, word: AstId) -> Option<String> {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return None;
    };
    let inner = match parts.as_slice() {
        [WordPart::DoubleQuoted(inner)] => inner.as_slice(),
        other => other,
    };
    match inner {
        [WordPart::ParamExpansion { base, op }] if op.default_word_is_empty() => Some(base.clone()),
        _ => None,
    }
}

/// A word that is one plain literal, quoted or not — the value side of a sentinel test.
fn plain_text(ast: &Ast, word: AstId) -> Option<String> {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return None;
    };
    match parts.as_slice() {
        [WordPart::Literal(text) | WordPart::SingleQuoted(text)] => Some(text.clone()),
        [WordPart::DoubleQuoted(inner)] => match inner.as_slice() {
            [WordPart::Literal(text)] => Some(text.clone()),
            [] => Some(String::new()),
            _ => None,
        },
        [] => Some(String::new()),
        _ => None,
    }
}

/// Every item of a guard branch, when all of them are load control the loader models: a `.`, an
/// `unset -f`, a no-op, or a nested guard. `None` the moment one is not.
fn guard_branch(ast: &Ast, branch: AstId) -> Option<Vec<AstId>> {
    let (NodeKind::Script { items } | NodeKind::List { items }) = &ast.node(branch).kind else {
        return None;
    };
    items
        .iter()
        .all(|&item| item_is_load_control(ast, item))
        .then(|| items.clone())
}

/// Is this item something a guard branch may hold? Loads, function removals, no-ops, and nested
/// guards — never a declaration (see [`include_guard`]) and never an assignment, whose conditional
/// value the load plane would then have to model.
fn item_is_load_control(ast: &Ast, item: AstId) -> bool {
    match &ast.node(item).kind {
        NodeKind::Simple {
            assigns,
            words,
            redirs,
        } => {
            redirs.is_empty()
                && assigns.is_empty()
                && (static_load_target(ast, words)
                    || unset_functions(ast, words).is_some()
                    || matches!(
                        words.first().and_then(|&w| literal_word(ast, w)),
                        Some(":" | "true")
                    ))
        }
        NodeKind::If { .. } => include_guard(ast, item).is_some(),
        _ => false,
    }
}

/// The names an `unset -f NAME…` removes, or `None` for any other command. `unset NAME` without
/// `-f` is a VARIABLE and is not load control.
#[must_use]
pub fn unset_functions(ast: &Ast, words: &[AstId]) -> Option<Vec<String>> {
    let [head, flag, rest @ ..] = words else {
        return None;
    };
    if literal_word(ast, *head)? != "unset" || literal_word(ast, *flag)? != "-f" || rest.is_empty()
    {
        return None;
    }
    rest.iter()
        .map(|&word| literal_word(ast, word).map(str::to_owned))
        .collect()
}

/// How far a guard condition is unwrapped looking for one simple command. Bounded for the reason
/// every other walk here is: a malformed shape must lose precision, never spin.
const GUARD_UNWRAP_CAP: usize = 8;

/// The statically spelled target of a top-level `.`, or `None` for any other item.
///
/// The driver reads this to learn what a marked file sources, and the answer is the whole input to
/// the include-tree (`core::custody`). It is deliberately the SPELLING only: whether the target
/// exists, and whether it satisfies the dorc-lang contract, are questions for the edge that can
/// open files.
#[must_use]
pub fn item_is_static_load(ast: &Ast, item: AstId) -> Option<AstId> {
    let NodeKind::Simple {
        assigns,
        words,
        redirs,
    } = &ast.node(item).kind
    else {
        return None;
    };
    (redirs.is_empty() && assigns.is_empty() && static_load_target(ast, words)).then(|| words[1])
}

/// Is this word list a `.` whose operand this pass can read off the text?
///
/// **`source` is NOT admitted, and that is a floor fact rather than a taste.** `dash` has no
/// `source` builtin (measured: `source: not found`), so a marked file using it would fail the
/// `two-binary-floor` the whole dialect is defined by — the construct is outside the base dialect,
/// and admitting it here would mint text `dorc strip` leaves behind and the off-ramp cannot run. A
/// BOOK may still spell it (`funcenv`'s load sites accept both), because book text is not
/// floor-bound.
///
/// The operand must expand without running anything, by the same predicate the assignment arm uses
/// and for the same reason: `${x:-$(hostname)}` lexes identically to `${x:-literal}`, so accepting
/// the operator form would accept a hidden command substitution deciding which file loads. Exactly
/// two words, because `. a b` passes positional parameters in some shells and is outside what this
/// admission was ruled for.
fn static_load_target(ast: &Ast, words: &[AstId]) -> bool {
    let ([_, target], Some(".")) = (words, words.first().and_then(|&w| literal_word(ast, w)))
    else {
        return false;
    };
    let NodeKind::Word { parts } = &ast.node(*target).kind else {
        return false;
    };
    parts.iter().all(word_part_is_static)
}

/// A word's text when it is one plain literal — the command-position read `static_load_target` does.
fn literal_word(ast: &Ast, word: AstId) -> Option<&str> {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return None;
    };
    match parts.as_slice() {
        [WordPart::Literal(text)] => Some(text),
        _ => None,
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

/// A word fragment that cannot run a command. `ParamExpansion` is refused alongside the obvious
/// two: the lexer now decodes the body, so `${x:-$(hostname)}` and `${x:-literal}` ARE
/// distinguishable here — but admitting the inert half widens what becomes a load program, which
/// is a licensure act (`28Q` §1's winner-shifting rider) rather than a mechanical consequence of
/// the decode.
fn word_part_is_static(part: &WordPart) -> bool {
    match part {
        WordPart::Literal(_) | WordPart::SingleQuoted(_) | WordPart::Param { .. } => true,
        WordPart::DoubleQuoted(inner) => inner.iter().all(word_part_is_static),
        WordPart::CommandSubst(_) | WordPart::Arithmetic | WordPart::ParamExpansion { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{GuardCondition, include_guard, lint_load_inert};
    use dorc_syntax::ast::NodeKind;

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
        ] {
            assert_eq!(slugs(body).len(), 1, "{body}");
        }
    }

    /// THE amendment (`28Q:pin-oracle-side-sourcing-amendment`): a top-level `.` is legal oracle
    /// text. It is how sh spells composition, and `28M` §7 already sanctions explicitly-spelled
    /// composition as the community-critical package shape — a helpers file plus a thin
    /// entrypoints file that sources it.
    #[test]
    fn a_top_level_dot_is_admitted() {
        assert!(slugs(". ./helpers.sh\n").is_empty());
        assert!(slugs(". \"$DORC_HELPERS\"\n").is_empty());
        assert!(
            slugs(". ./helpers.sh\nf() { :; }\n").is_empty(),
            "and the file's own declarations keep contributing beside it — the second conjunct, \
             without which one blessed line costs the author every helper they wrote"
        );
    }

    /// `source` is NOT the same construct. `dash` has no such builtin (measured: `source: not
    /// found`), so a marked file spelling it fails the two-binary floor the dialect is defined by
    /// — the construct is outside the base dialect, and `dorc strip` would leave behind text the
    /// off-ramp cannot run.
    #[test]
    fn the_source_spelling_stays_outside_the_dialect() {
        assert_eq!(
            slugs("source ./helpers.sh\n"),
            ["oracle-file-not-load-inert"]
        );
    }

    /// A target the pass cannot read off the text is refused, for the assignment arm's reason: the
    /// lexer collapses operator forms to one opaque part, so `${x:-$(hostname)}` is
    /// indistinguishable from `${x:-lit}` and accepting it would let a hidden command substitution
    /// decide which file loads. `. a b` is refused as a shape nothing ruled on.
    #[test]
    fn a_dynamic_or_multi_word_load_is_refused() {
        assert_eq!(slugs(". \"$(pick)\"\n"), ["oracle-file-not-load-inert"]);
        assert_eq!(
            slugs(". \"${DORC_ROOT:-/etc}/h.sh\"\n"),
            ["oracle-file-not-load-inert"]
        );
        assert_eq!(slugs(". ./h.sh extra\n"), ["oracle-file-not-load-inert"]);
        assert_eq!(
            slugs(". ./h.sh >/dev/null\n"),
            ["oracle-file-not-load-inert"]
        );
    }

    /// THE INCLUDE GUARD (`30I:rul-include-guards-are-load-semantics`, TYPED): the canonical
    /// shared-dependency shape, and the whole of the conditional surface a marked file may carry.
    /// `30I` §2.2 spells it with the `else` arm doing the loading; the negated `!` spelling and a
    /// bare `then`-arm load are the same guard read the other way round.
    #[test]
    fn the_canonical_include_guard_is_admitted() {
        for body in [
            "if command -v _q >/dev/null 2>&1; then\n   :\nelse\n   . ./common.sh\nfi\n",
            "if ! command -v _q >/dev/null 2>&1; then\n   . ./common.sh\nfi\n",
            "if command -v _q; then unset -f _q; . ./better.sh; fi\n",
            "if command -v _q; then\n   :\nelse\n   if command -v _r; then :; else . ./c.sh; fi\nfi\n",
        ] {
            assert!(slugs(body).is_empty(), "{body} — {:?}", slugs(body));
        }
        assert!(
            slugs("if command -v _q; then :; fi\nq__is_converged() { _q ;}\n").is_empty(),
            "and the file's own declarations keep contributing beside it"
        );
    }

    /// A guard's branch may NOT define. The pins this protects are `sh_parity.rs`'s
    /// `a_host_conditional_oracle_definition_licenses_nothing` and its expected-fail twin: the
    /// dialect lift sees a role header only as a top-level ITEM, so a nested definition would be
    /// registered by the definition table while lifting ZERO rows — described nowhere, detected
    /// nowhere, and licensing off a body the lift never read. Widening the allow-list without
    /// making that binding `May` is exactly the wrong-elision route they forbid.
    #[test]
    fn a_guard_branch_may_not_define() {
        assert_eq!(
            slugs("if command -v _q; then :; else _q() { hork ;}; fi\n"),
            ["oracle-file-not-load-inert"]
        );
        assert_eq!(
            slugs("if command -v _q; then :; else SM_ROOT=./other; fi\n"),
            ["oracle-file-not-load-inert"],
            "nor assign — a conditional value is one the load plane would have to model"
        );
        assert_eq!(
            slugs("if command -v _q; then :; else apt-get update; fi\n"),
            ["oracle-file-not-load-inert"],
            "nor run anything at all"
        );
    }

    /// TWO conditions open a guard, and no others — a file test, a computed name, an `elif` chain
    /// all stay refused, because the loader models a CLOSED set of questions and `inv-top-reject`
    /// biases the unknown toward refusal.
    #[test]
    fn only_the_two_guard_conditions_open_a_guard() {
        for body in [
            "if [ -f ./common.sh ]; then . ./common.sh; fi\n",
            "if command -v \"$WANTED\"; then :; fi\n",
            "if command -v _q; then :; elif command -v _r; then . ./c.sh; fi\n",
            "if command -v _q && command -v _r; then :; else . ./c.sh; fi\n",
            "if [ \"${sm_loaded-}\" ]; then :; else . ./c.sh; fi\n",
            "if [ \"$sm_loaded\" != 'v1' ]; then . ./c.sh; fi\n",
        ] {
            assert_eq!(slugs(body), ["oracle-file-not-load-inert"], "{body}");
        }
    }

    /// THE PACKAGE SENTINEL (`30I` §2.2), in the spellings an author reaches for. The variable side
    /// is the NOUNSET-SAFE form and nothing else: a bare `"$x"` aborts the loading shell under
    /// `set -u`, so admitting it would bless an idiom the floor breaks on (the last case of
    /// [`only_the_two_guard_conditions_open_a_guard`] pins that refusal).
    #[test]
    fn the_package_sentinel_guard_is_admitted() {
        for body in [
            "if [ \"${sm_loaded-}\" != 'sm.common/v1' ]; then\n   . ./common.sh\nfi\n",
            "if [ \"${sm_loaded:-}\" != 'sm.common/v1' ]; then\n   . ./common.sh\nfi\n",
            "if [ \"${sm_loaded-}\" = 'sm.common/v1' ]; then\n   :\nelse\n   . ./common.sh\nfi\n",
            "if test \"${sm_loaded-}\" != 'sm.common/v1'; then . ./common.sh; fi\n",
            "if [ 'sm.common/v1' != \"${sm_loaded-}\" ]; then . ./common.sh; fi\n",
        ] {
            assert!(slugs(body).is_empty(), "{body} — {:?}", slugs(body));
        }
    }

    /// The decomposition a loader reads: which variable, which literal, and which sense — because
    /// the sense is what says which arm loads, and the arms are not interchangeable.
    #[test]
    fn a_sentinel_guard_decomposes_to_its_test() {
        let src = format!(
            "{MARKER}if [ \"${{sm_loaded-}}\" != 'sm.common/v1' ]; then\n   . ./common.sh\nfi\n"
        );
        let ast = dorc_syntax::parse(&src).value;
        let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
            panic!("expected a script")
        };
        let guard = include_guard(&ast, items[0]).expect("the guard is recognized");
        assert_eq!(
            guard.condition,
            GuardCondition::Value {
                name: "sm_loaded".to_owned(),
                literal: "sm.common/v1".to_owned(),
                equals: false,
            }
        );
        assert!(!guard.negated);
        assert_eq!(guard.then_.len(), 1, "the load is the taken arm");
        assert!(guard.else_.is_empty());
    }

    /// `unset -f` is v0 load surface (`30I:rul-oracle-loading-stays-load-safe`) — it removes a
    /// binding and runs nothing. `unset` without `-f` is a VARIABLE and stays refused, because the
    /// load plane does not model variable removal.
    #[test]
    fn unset_f_is_load_surface_and_bare_unset_is_not() {
        assert!(slugs("unset -f _q\n").is_empty());
        assert!(slugs("unset -f _q _r\n").is_empty());
        assert_eq!(slugs("unset SM_ROOT\n"), ["oracle-file-not-load-inert"]);
        assert_eq!(slugs("unset -f\n"), ["oracle-file-not-load-inert"]);
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
