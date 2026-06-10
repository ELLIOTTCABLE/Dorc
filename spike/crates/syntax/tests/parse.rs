//! Integration tests for `dorc-syntax::parse`. Brutal + targeted over exhaustive
//! (spike/CLAUDE.md testing policy): each test pins a specific invariant the
//! analyzer downstream depends on. Repetition is intentional — no DRY ceremony.

// An integration-test crate is a separate crate to clippy, so the `allow-*-in-tests`
// clippy.toml keys cover `#[test]` bodies but NOT module-level test helpers
// (`script_items`, `assert_all_ids_resolve`) or `#[test]`-fn length. These are
// the "tests may panic/index/cast" allowances the policy intends, spelled at the
// file top because the keys can't reach this crate's helpers.
#![expect(
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    reason = "test helpers/asserts: panic-on-bad-fixture, count-to-u32, and a long shape-assertion test — the in-tests allowances the policy intends"
)]

use dorc_core::AstId;
use dorc_syntax::ast::Ast;
use dorc_syntax::{NodeKind, RedirOp, RedirTarget, UnsupportedReason, Word, WordPart, parse};

// --- tiny read helpers (keep assertions legible) ---------------------------

fn script_items(ast: &Ast) -> &[AstId] {
    match &ast.node(ast.root()).kind {
        NodeKind::Script { items } => items,
        other => panic!("root is not Script: {other:?}"),
    }
}

fn kind(ast: &Ast, id: AstId) -> &NodeKind {
    &ast.node(id).kind
}

/// A discriminant label for a node, for order/shape assertions.
fn label(ast: &Ast, id: AstId) -> &'static str {
    match kind(ast, id) {
        NodeKind::Script { .. } => "Script",
        NodeKind::List { .. } => "List",
        NodeKind::Simple { .. } => "Simple",
        NodeKind::Pipeline { .. } => "Pipeline",
        NodeKind::AndOr { .. } => "AndOr",
        NodeKind::Subshell { .. } => "Subshell",
        NodeKind::Group { .. } => "Group",
        NodeKind::If { .. } => "If",
        NodeKind::Case { .. } => "Case",
        NodeKind::FuncDef { .. } => "FuncDef",
        NodeKind::ForLoop { .. } => "ForLoop",
        NodeKind::WhileLoop { .. } => "WhileLoop",
        NodeKind::Word { .. } => "Word",
        NodeKind::Assign { .. } => "Assign",
        NodeKind::Redir { .. } => "Redir",
        NodeKind::Unsupported { .. } => "Unsupported",
    }
}

/// The literal text of a word node, if it is a single fixed literal. (Mirrors
/// `Word::as_literal` but borrows directly from the arena so the `&str` outlives
/// the call — the `Word` wrapper is `Copy` and a temporary, which the borrow
/// checker won't let return a reference through.)
fn word_literal(ast: &Ast, id: AstId) -> Option<&str> {
    match kind(ast, id) {
        NodeKind::Word { parts } => match parts.as_slice() {
            [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s),
            _ => None,
        },
        _ => None,
    }
}

/// The literal text of a command's first word, if statically fixed.
fn first_word_literal(ast: &Ast, simple: AstId) -> Option<&str> {
    match kind(ast, simple) {
        NodeKind::Simple { words, .. } => word_literal(ast, *words.first()?),
        _ => None,
    }
}

// ===========================================================================
// Fixture: the pi-webhost book must parse to its exact top-level shape.
//
// Why this test: this is the do-4 dogfood book and the analyzer's first real
// fixture. If the top-level item order/kinds drift, every downstream phase that
// keys off the CFG built from this tree silently mis-analyzes. We pin the whole
// top-level sequence and the load-bearing internals (negated-pipeline cond,
// heredoc capture + quoted flag, the `[ ]`-as-command, the && change-signal).
// ===========================================================================

#[test]
fn fixture_pi_webhost_top_level_shape() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/pi-webhost.book.sh"
    ));
    let parsed = parse(src);
    let ast = &parsed.value;

    // A well-formed book parses with ZERO diagnostics — it is entirely inside the
    // modeled subset. Any diagnostic here is a regression (over- or under-reject).
    assert!(
        parsed.diags.is_empty(),
        "fixture must parse clean; got diags: {:?}",
        parsed.diags
    );

    let items = script_items(ast);
    let labels: Vec<&str> = items.iter().map(|&i| label(ast, i)).collect();
    assert_eq!(
        labels,
        [
            "Simple", // set -e
            "Case",   // case "$(hostname)" in …
            "If",     // if ! command -v nginx …
            "Simple", // ufw allow 80/tcp
            "Simple", // ufw allow 443/tcp
            "Simple", // systemctl enable --now nginx
            "If",     // if [ ! -f … ]; then cat > … <<'EOF' …
            "AndOr",  // nginx -t && systemctl reload nginx
            "Simple", // touch …
            "Simple", // echo "pi-web up"
        ],
        "top-level item kinds/order"
    );

    // set -e
    assert_eq!(first_word_literal(ast, items[0]), Some("set"));

    // Case scrutinee is the command-substitution `$(hostname)` (double-quoted).
    match kind(ast, items[1]) {
        NodeKind::Case { word, arms } => {
            match kind(ast, *word) {
                NodeKind::Word { parts } => match parts.as_slice() {
                    [WordPart::DoubleQuoted(inner)] => {
                        assert!(
                            matches!(inner.as_slice(), [WordPart::CommandSubst(_)]),
                            "case word should be \"$(hostname)\" → DQ[CommandSubst]"
                        );
                    }
                    other => panic!("case word parts: {other:?}"),
                },
                other => panic!("case word: {other:?}"),
            }
            assert_eq!(arms.len(), 2, "two case arms");
            // First arm has two alternation patterns (pi-web* | webhost-*).
            assert_eq!(arms[0].patterns.len(), 2, "first arm: pi-web*|webhost-*");
            // Second arm is the `*` catch-all.
            assert_eq!(arms[1].patterns.len(), 1);
            assert_eq!(
                word_literal(ast, arms[1].patterns[0]),
                Some("*"),
                "second arm pattern is `*`"
            );
        }
        other => panic!("items[1] should be Case: {other:?}"),
    }

    // First If: condition is a NEGATED single-stage pipeline (`! command -v nginx`),
    // and the staged command carries the two redirs `>/dev/null 2>&1`.
    match kind(ast, items[2]) {
        NodeKind::If { cond, .. } => {
            let cond_items = match kind(ast, *cond) {
                NodeKind::List { items } => items.clone(),
                other => panic!("if cond not a List: {other:?}"),
            };
            assert_eq!(cond_items.len(), 1);
            match kind(ast, cond_items[0]) {
                NodeKind::Pipeline { negated, stages } => {
                    assert!(*negated, "cond pipeline must be `!`-negated");
                    assert_eq!(stages.len(), 1);
                    match kind(ast, stages[0]) {
                        NodeKind::Simple { words, redirs, .. } => {
                            assert_eq!(word_literal(ast, words[0]), Some("command"));
                            assert_eq!(redirs.len(), 2, "`>/dev/null` + `2>&1`");
                            // First redir: write to /dev/null.
                            assert!(matches!(
                                kind(ast, redirs[0]),
                                NodeKind::Redir {
                                    op: RedirOp::Write,
                                    ..
                                }
                            ));
                            // Second redir: 2>&1 (dup fd 2 → fd 1).
                            match kind(ast, redirs[1]) {
                                NodeKind::Redir {
                                    op: RedirOp::Dup,
                                    fd,
                                    target,
                                } => {
                                    assert_eq!(*fd, Some(2));
                                    assert!(matches!(target, RedirTarget::Fd(1)));
                                }
                                other => panic!("second redir: {other:?}"),
                            }
                        }
                        other => panic!("pipeline stage: {other:?}"),
                    }
                }
                other => panic!("cond item not Pipeline: {other:?}"),
            }
        }
        other => panic!("items[2] should be If: {other:?}"),
    }

    // Second If: then-body contains a `cat > file <<'EOF'` with a HEREDOC redir
    // whose delimiter was quoted (no-expansion) and whose body was captured.
    match kind(ast, items[6]) {
        NodeKind::If { then_body, .. } => {
            let body_items = match kind(ast, *then_body) {
                NodeKind::List { items } => items.clone(),
                other => panic!("{other:?}"),
            };
            // first body command: `cat > … <<'EOF'`
            match kind(ast, body_items[0]) {
                NodeKind::Simple { words, redirs, .. } => {
                    assert_eq!(word_literal(ast, words[0]), Some("cat"));
                    assert_eq!(redirs.len(), 2, "redirect-to-file + heredoc");
                    let heredoc = redirs.iter().find_map(|&r| match kind(ast, r) {
                        NodeKind::Redir {
                            op: RedirOp::HereDoc,
                            target,
                            ..
                        } => Some(target),
                        _ => None,
                    });
                    let Some(heredoc) = heredoc else {
                        panic!("a heredoc redir is present");
                    };
                    match heredoc {
                        RedirTarget::HereDoc { body, quoted } => {
                            assert!(*quoted, "<<'EOF' delimiter is quoted ⇒ no expansion");
                            assert!(
                                body.contains("server_name _;"),
                                "heredoc body captured verbatim, got: {body:?}"
                            );
                        }
                        other => panic!("expected HereDoc target: {other:?}"),
                    }
                }
                other => panic!("then-body[0]: {other:?}"),
            }
        }
        other => panic!("items[6] should be If: {other:?}"),
    }

    // The change-signal idiom `nginx -t && systemctl reload nginx`.
    match kind(ast, items[7]) {
        NodeKind::AndOr { op, left, right } => {
            assert_eq!(*op, dorc_syntax::AndOrOp::And);
            assert_eq!(first_word_literal(ast, *left), Some("nginx"));
            assert_eq!(first_word_literal(ast, *right), Some("systemctl"));
        }
        other => panic!("items[7] should be AndOr: {other:?}"),
    }
}

// ===========================================================================
// Totality (inv-no-throw): a table of hostile inputs must all return without
// panic. This is THE invariant the whole pipeline rests on — `parse` is called
// on untrusted scripts, so a panic anywhere is a crash of the orchestrator.
// We assert only "it returned and the root resolves"; shape is irrelevant here.
// ===========================================================================

#[test]
fn totality_hostile_inputs_never_panic() {
    let hostile = [
        "",                             // empty
        " \t\n\n  \t ",                 // only whitespace
        "\u{0}\u{0}\u{0}",              // NUL bytes
        "echo \u{1f600}\u{1f4a9} café", // non-ASCII / multibyte
        "$(((",                         // truncated arithmetic
        "$( $( $( ",                    // nested unterminated cmd-subst
        "'unterminated single",         // unterminated single quote
        "\"unterminated $x double",     // unterminated double quote
        "`unterminated backtick",       // unterminated backtick
        "${unterminated",               // unterminated brace param
        "<<EOF\nbody no terminator",    // unterminated heredoc
        "if then fi |||",               // garbage operators
        "case in esac",                 // degenerate case
        "case x in",                    // unterminated case
        "if if if if",                  // repeated keyword, no bodies
        ";;;;;;",                       // only terminators
        "| | |",                        // only pipes
        "& & &",                        // only async
        "{ { { {",                      // unbalanced groups
        "( ( ( (",                      // unbalanced subshells
        ")))) }}}} ;;;;",               // unbalanced closers
        "for for for do do done done",  // malformed loop
        "for x in a b",                 // unterminated for (no `do`/`done`)
        "for x in a b; do echo $x",     // for body, no `done`
        "while",                        // bare loop keyword at EOF
        "do echo hi; done",             // `done` without `for`/`while` (misplaced)
        "until",                        // bare until at EOF
        "for x in $(",                  // for-list with truncated cmd-subst
        &"for i in a; do ".repeat(400), // deep loop nesting (stack safety, MAX_DEPTH)
        "a=$(b=$(c=$(echo)))",          // deeply nested subst assigns
        "\\",                           // lone backslash
        "echo \\",                      // trailing backslash
        "2>&",                          // truncated dup
        "<<<",                          // here-string (unmodeled triadic)
        &"(".repeat(2000),              // deep nesting (stack safety)
        &"echo a | ".repeat(1000),      // long pipeline
        &"a && ".repeat(1000),          // long and-or chain
    ];
    for src in hostile {
        let parsed = parse(src);
        // Root must resolve (no panic, arena consistent).
        let _ = ast_root_label(&parsed.value);
        // Every node id in the arena must resolve, and every child id must point
        // within the arena (no dangling ids from the absorb/shift logic).
        assert_all_ids_resolve(&parsed.value);
    }
}

fn ast_root_label(ast: &Ast) -> &'static str {
    label(ast, ast.root())
}

/// Walk every node and assert each referenced child id is in-bounds. A dangling
/// id would panic on resolve downstream — this catches arena-corruption bugs the
/// totality table would otherwise mask (since we only print labels).
fn assert_all_ids_resolve(ast: &Ast) {
    let count = ast.iter().count() as u32;
    let check = |id: AstId| assert!(id.0 < count, "dangling AstId {id:?} (arena has {count})");
    for (_, node) in ast.iter() {
        match &node.kind {
            NodeKind::Script { items } | NodeKind::List { items } => {
                for &i in items {
                    check(i);
                }
            }
            NodeKind::Simple {
                assigns,
                words,
                redirs,
            } => {
                assigns
                    .iter()
                    .chain(words)
                    .chain(redirs)
                    .for_each(|&i| check(i));
            }
            NodeKind::Pipeline { stages, .. } => stages.iter().for_each(|&i| check(i)),
            NodeKind::AndOr { left, right, .. } => {
                check(*left);
                check(*right);
            }
            NodeKind::Subshell { body, redirs } | NodeKind::Group { body, redirs } => {
                check(*body);
                for &i in redirs {
                    check(i);
                }
            }
            NodeKind::If {
                cond,
                then_body,
                elifs,
                else_body,
            } => {
                check(*cond);
                check(*then_body);
                for e in elifs {
                    check(e.cond);
                    check(e.body);
                }
                if let Some(e) = else_body {
                    check(*e);
                }
            }
            NodeKind::Case { word, arms } => {
                check(*word);
                for arm in arms {
                    arm.patterns.iter().for_each(|&i| check(i));
                    check(arm.body);
                }
            }
            NodeKind::FuncDef { body, .. } => check(*body),
            NodeKind::ForLoop { words, body, .. } => {
                for &i in words {
                    check(i);
                }
                check(*body);
            }
            NodeKind::WhileLoop { cond, body, .. } => {
                check(*cond);
                check(*body);
            }
            NodeKind::Word { parts } => check_parts(parts, &check),
            NodeKind::Assign { value, .. } => {
                if let Some(v) = value {
                    check(*v);
                }
            }
            NodeKind::Redir { target, .. } => {
                if let RedirTarget::Word(w) = target {
                    check(*w);
                }
            }
            NodeKind::Unsupported { salvaged, .. } => salvaged.iter().for_each(|&i| check(i)),
        }
    }
}

fn check_parts(parts: &[WordPart], check: &impl Fn(AstId)) {
    for p in parts {
        match p {
            WordPart::CommandSubst(id) => check(*id),
            WordPart::DoubleQuoted(inner) => check_parts(inner, check),
            _ => {}
        }
    }
}

// ===========================================================================
// ⊤-reject (inv-top-reject): unmodeled constructs collapse to Unsupported with
// the CORRECT reason variant + an Error diagnostic — loud, never silent. The
// analyzer treats these as absorbing ⊤; the wrong reason mislabels why.
// ===========================================================================

/// Find the first `Unsupported` node anywhere in the tree and return its reason.
fn first_unsupported(ast: &Ast) -> Option<UnsupportedReason> {
    ast.iter().find_map(|(_, n)| match &n.kind {
        NodeKind::Unsupported { reason, .. } => Some(reason.clone()),
        _ => None,
    })
}

fn assert_rejects(src: &str, want: UnsupportedReason) {
    let parsed = parse(src);
    assert!(
        parsed.has_errors(),
        "`{src}` must emit an Error diagnostic (loud ⊤-reject)"
    );
    assert_eq!(
        first_unsupported(&parsed.value),
        Some(want),
        "`{src}` ⊤-reason"
    );
}

#[test]
fn loops_with_literal_lists_parse_to_real_nodes() {
    // task-L1 (brk-1): the ⊤-trigger shrank — `for`/`while`/`until` over an
    // enumerable list now PARSE to real AST nodes (body + words captured), no
    // diagnostic. This is the pin-flip from the old loops-as-⊤ posture.
    let f = parse("for i in 1 2; do echo $i; done");
    assert!(
        !f.has_errors(),
        "literal-list for must parse clean: {:?}",
        f.diags
    );
    match kind(&f.value, script_items(&f.value)[0]) {
        NodeKind::ForLoop {
            var, words, body, ..
        } => {
            assert_eq!(var, "i", "iteration variable captured");
            assert_eq!(words.len(), 2, "both list words captured (1 2)");
            assert!(
                matches!(kind(&f.value, *body), NodeKind::List { .. }),
                "body is a List"
            );
        }
        other => panic!("expected ForLoop: {other:?}"),
    }

    let w = parse("while true; do :; done");
    assert!(!w.has_errors(), "while must parse clean: {:?}", w.diags);
    assert!(matches!(
        kind(&w.value, script_items(&w.value)[0]),
        NodeKind::WhileLoop { until: false, .. }
    ));

    let u = parse("until false; do :; done");
    assert!(!u.has_errors(), "until must parse clean: {:?}", u.diags);
    assert!(matches!(
        kind(&u.value, script_items(&u.value)[0]),
        NodeKind::WhileLoop { until: true, .. }
    ));
}

#[test]
fn loop_shapes_outside_the_subset_stay_unsupported_loop() {
    // task-L1: what STAYS ⊤-rejected, with the honest reason pinned (the report's
    // "what stays ⊤" list). Each must be a loud `UnsupportedReason::Loop`.
    // no-`in` `for` iterates runtime "$@" (not a static list):
    assert_rejects("for x; do echo $x; done", UnsupportedReason::Loop);
    // `break`/`continue` — un-modeled early exit breaks the back-edge fixpoint's
    // reaching-uses soundness (the body would otherwise parse fine):
    assert_rejects("for x in a b; do break; done", UnsupportedReason::Loop);
    assert_rejects("while true; do continue; done", UnsupportedReason::Loop);
    // a list word with a command-substitution / arithmetic — effect-bearing
    // expansion in word position, deferred per HOLE#1:
    assert_rejects(
        "for f in $(ls /etc); do echo $f; done",
        UnsupportedReason::Loop,
    );
    assert_rejects(
        "for n in $((1+1)); do echo $n; done",
        UnsupportedReason::Loop,
    );
    // A `break`/`continue` NESTED in an if/group/case WITHIN the body still binds to the
    // loop ⇒ still ⊤-rejects (the body walk descends through non-loop compounds).
    assert_rejects(
        "for x in a b; do if true; then break; fi; done",
        UnsupportedReason::Loop,
    );
    assert_rejects("while c; do { continue; }; done", UnsupportedReason::Loop);
    assert_rejects(
        "for x in a b; do case $x in a) break ;; esac; done",
        UnsupportedReason::Loop,
    );
}

#[test]
fn break_or_continue_as_an_argument_is_not_a_jump() {
    // The dual: `break`/`continue` in ARGUMENT position (not the command word) is an
    // ordinary word, NOT a loop jump ⇒ the loop parses clean. Pins that the jump
    // detection keys on the command-word position only (`body_has_loop_jump`).
    assert!(
        !parse("for x in a b; do echo break; done").has_errors(),
        "`break` as an argument to `echo` is not a loop jump"
    );
    assert!(
        !parse("for x in a b; do apt-get install continue; done").has_errors(),
        "`continue` as an operand is not a loop jump"
    );
}

#[test]
fn nested_break_continue_binds_to_inner_loop_only() {
    // A `break`/`continue` in a NESTED loop binds to the INNER loop, so it does NOT
    // ⊤-reject the OUTER loop — only the inner one. The outer parses; the inner is the
    // ⊤ node. (`body_has_loop_jump` does not descend into a nested loop.)
    let p = parse("for o in a b; do for i in 1 2; do break; done; done");
    // The outer is a ForLoop (NOT rejected); the inner break-bearing loop is the ⊤.
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::ForLoop { body, .. } => {
            // Somewhere inside the outer body there is exactly one Unsupported (the inner).
            assert!(
                first_unsupported(&p.value).is_some(),
                "the inner break-loop is ⊤-rejected"
            );
            let _ = body;
        }
        other => panic!("outer must parse as ForLoop, not be rejected: {other:?}"),
    }
    assert!(p.has_errors(), "the inner loop's ⊤-reject is loud");
}

// ===========================================================================
// fix-3 (`20O` find-4): the for-LIST is a wordlist that ends ONLY at `;`/newline
// (dash/POSIX) — a reserved word in list position is an ORDINARY word, not a
// terminator. Both demonstrated directions must be dash-faithful.
// ===========================================================================

#[test]
fn for_list_reserved_words_are_ordinary_list_words() {
    // `for f in do done; do echo "$f"; done` — `do` and `done` in LIST position are
    // literal list words (dash iterates `do`,`done`). Must parse clean to a ForLoop with
    // both words captured (before fix-4 the wordlist wrongly terminated at the first `do`).
    let p = parse(r#"for f in do done; do echo "$f"; done"#);
    assert!(
        !p.has_errors(),
        "`do`/`done` as for-list words must parse clean: {:?}",
        p.diags
    );
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::ForLoop { var, words, .. } => {
            assert_eq!(var, "f");
            assert_eq!(
                words.len(),
                2,
                "both literal list words `do`,`done` captured"
            );
            assert_eq!(word_literal(&p.value, words[0]), Some("do"));
            assert_eq!(word_literal(&p.value, words[1]), Some("done"));
        }
        other => panic!("expected ForLoop iterating `do`,`done`: {other:?}"),
    }
}

#[test]
fn for_list_unterminated_before_do_is_loud_top_reject() {
    // `for f in a b do c; done` — the wordlist consumes `a b do c` (ending only at `;`),
    // then finds `done` where dash requires `do` ⇒ a syntax error in dash
    // (`"done" unexpected (expecting "do")`). We must ⊤-reject the whole loop loudly,
    // never silently build a malformed ForLoop (`inv-top-reject`).
    assert_rejects("for f in a b do c; done", UnsupportedReason::Loop);
    // The fully-`do`-less list form: `for f in a b; echo c; done` (no `do` anywhere).
    assert_rejects("for f in a b; echo c; done", UnsupportedReason::Loop);
}

#[test]
fn for_var_can_be_a_reserved_word_in_agreement() {
    // The agreement case: `for in in a b; do echo "$in"; done` — the FIRST `in` is the
    // iteration VARIABLE name (a valid POSIX name), the SECOND `in` is the list keyword.
    // dash accepts this and iterates `a`,`b`. Must parse clean with var=`in`, words=[a,b].
    let p = parse(r#"for in in a b; do echo "$in"; done"#);
    assert!(
        !p.has_errors(),
        "`for in in a b` must parse clean (first `in` is the var): {:?}",
        p.diags
    );
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::ForLoop { var, words, .. } => {
            assert_eq!(var, "in", "the iteration variable is the first `in`");
            assert_eq!(words.len(), 2, "the list is `a b`");
            assert_eq!(word_literal(&p.value, words[0]), Some("a"));
            assert_eq!(word_literal(&p.value, words[1]), Some("b"));
        }
        other => panic!("expected ForLoop with var `in`: {other:?}"),
    }
}

// ===========================================================================
// fix-4 (`20O` find-5): `break`/`continue` in a `while`/`until` CONDITION region
// ⊤-rejects the loop, like a body jump (dash runs a condition `break` and it DOES
// exit the loop). The for-LIST is a wordlist, so a `break` there is a literal word.
// ===========================================================================

#[test]
fn break_or_continue_in_while_condition_is_top_reject() {
    // A condition-position jump is an un-modeled early exit ⇒ ⊤-reject (dash:
    // `while … && break; do …` exits the loop). Both while and until conditions.
    assert_rejects("while break; do :; done", UnsupportedReason::Loop);
    assert_rejects("until continue; do :; done", UnsupportedReason::Loop);
    // A condition jump nested in the condition's own control flow still binds here.
    assert_rejects("while x && break; do :; done", UnsupportedReason::Loop);
    assert_rejects(
        "while if y; then continue; fi; do :; done",
        UnsupportedReason::Loop,
    );
}

#[test]
fn break_as_a_for_list_word_is_not_a_jump() {
    // The for-LIST is a wordlist (fix-3), so `break`/`continue` appearing as a LIST word
    // is a literal iteration value, NOT a loop jump ⇒ the loop parses clean (dash iterates
    // the literal `break`). Pins that the jump detector does not over-claim the for-list.
    let p = parse(r#"for x in break continue; do echo "$x"; done"#);
    assert!(
        !p.has_errors(),
        "`break`/`continue` as for-list words are literals, not jumps: {:?}",
        p.diags
    );
    assert!(matches!(
        kind(&p.value, script_items(&p.value)[0]),
        NodeKind::ForLoop { .. }
    ));
}

// ===========================================================================
// fix-2 (`20O` find-3): a CONSTRUCT-TRAILING redirection (`done < file`, `fi > log`,
// `esac > log`) currently mis-parses into a phantom empty-argv command with ZERO
// diagnostics — a silent ⊤. It must ⊤-reject the construct LOUDLY (honest interim;
// full construct-redirection modeling is a recorded later slice).
// ===========================================================================

#[test]
fn construct_trailing_redirection_is_loud_top_reject() {
    // The idiomatic `while read … done < file` shape (find-3's headline) — now loud.
    assert_rejects(
        "while read line; do echo \"$line\"; done < input",
        UnsupportedReason::Unmodeled("construct-trailing redirection"),
    );
    // `for … done > file`, `if … fi > log`, `case … esac > log` — every construct family.
    assert_rejects(
        "for x in a b; do echo \"$x\"; done > out",
        UnsupportedReason::Unmodeled("construct-trailing redirection"),
    );
    assert_rejects(
        "if true; then echo x; fi > log",
        UnsupportedReason::Unmodeled("construct-trailing redirection"),
    );
    assert_rejects(
        "case x in x) echo hi ;; esac > log",
        UnsupportedReason::Unmodeled("construct-trailing redirection"),
    );
    // An append (`>>`) and an input (`<`) redirection both count.
    assert_rejects(
        "until false; do :; done >> out",
        UnsupportedReason::Unmodeled("construct-trailing redirection"),
    );
}

#[test]
fn construct_trailing_redirection_salvages_the_construct() {
    // The construct is salvaged (so unrelated sibling analysis proceeds, `dn-7`), and the
    // ⊤ is loud. Pins the salvage so a later modeling slice can recover the inner tree.
    let p = parse("while read l; do echo \"$l\"; done < input");
    match first_unsupported_node(&p.value) {
        Some(NodeKind::Unsupported { salvaged, .. }) => {
            assert_eq!(salvaged.len(), 1, "the while-loop construct is salvaged");
            assert!(
                matches!(kind(&p.value, salvaged[0]), NodeKind::WhileLoop { .. }),
                "the salvaged child is the parsed WhileLoop"
            );
        }
        other => panic!("expected an Unsupported construct-redirect node: {other:?}"),
    }
}

#[test]
fn construct_without_trailing_redirection_still_parses_clean() {
    // The non-regression control: a loop/if/case WITHOUT a trailing redirection is
    // unaffected (still parses to its real node, no ⊤). Proves fix-2 fires only on the
    // trailing-redirection shape, not on every construct.
    assert!(!parse("while read l; do echo \"$l\"; done").has_errors());
    assert!(!parse("for x in a b; do echo \"$x\"; done").has_errors());
    assert!(!parse("if true; then echo x; fi").has_errors());
    // A redirection INSIDE the body is fine (it attaches to the inner command, not the
    // construct): `done` is not immediately followed by a redir here.
    assert!(!parse("while read l; do echo \"$l\" > /dev/null; done").has_errors());
}

#[test]
fn reject_eval_is_dynamic_execution() {
    // Why: eval runs constructed code — unanalyzable; the canonical ⊤-trigger.
    assert_rejects("eval \"$x\"", UnsupportedReason::DynamicExecution);
    assert_rejects("eval ls", UnsupportedReason::DynamicExecution);
}

#[test]
fn reject_dynamic_command_name_is_dynamic_execution() {
    // Why: a command whose NAME is an expansion (`"$cmd" a`) cannot be resolved to
    // an effect/oracle statically — referent-agnostic engine can't even start.
    assert_rejects("\"$cmd\" a", UnsupportedReason::DynamicExecution);
    assert_rejects("$cmd a", UnsupportedReason::DynamicExecution);
    assert_rejects("${prog} run", UnsupportedReason::DynamicExecution);
}

#[test]
fn reject_dynamic_source_target_but_allow_literal() {
    // Why: `. "$x"` pulls in code chosen at runtime (⊤); `. /literal` is a fixed
    // include the analyzer could in principle follow, so it must NOT reject.
    assert_rejects(". \"$x\"", UnsupportedReason::DynamicExecution);
    assert_rejects("source $f", UnsupportedReason::DynamicExecution);

    let ok = parse(". /etc/profile");
    assert!(!ok.has_errors(), "literal-target source must parse clean");
    assert!(first_unsupported(&ok.value).is_none());
}

#[test]
fn reject_arith_as_command_but_allow_arith_in_word() {
    // Why: `$(( … ))` as a command is dynamic arithmetic-driven dispatch (⊤,
    // ArithmeticExpansion); but `echo $((x))` is just an arg expansion — flagged
    // opaque in the word, NOT a command-level reject. This boundary is exact.
    assert_rejects("$(( 1 + 2 ))", UnsupportedReason::ArithmeticExpansion);

    let ok = parse("echo $(( x + 1 ))");
    assert!(!ok.has_errors(), "arith inside a word must not reject");
    // and the arg word carries an Arithmetic part (opaque, lossless).
    match kind(&ok.value, script_items(&ok.value)[0]) {
        NodeKind::Simple { words, .. } => {
            let has_arith = words.iter().any(|&w| match kind(&ok.value, w) {
                NodeKind::Word { parts } => parts.iter().any(|p| matches!(p, WordPart::Arithmetic)),
                _ => false,
            });
            assert!(has_arith, "expected an Arithmetic word-part");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn reject_lvalue_builtins_are_dynamic_lvalue() {
    // Why: these write/inspect a *variable* lvalue (indirect state the engine does
    // not model); the ⊤-set lists them explicitly.
    assert_rejects("unset \"$x\"", UnsupportedReason::DynamicLValue);
    assert_rejects("printf -v out '%s' y", UnsupportedReason::DynamicLValue);
    assert_rejects("test -v FOO", UnsupportedReason::DynamicLValue);
    assert_rejects("[ -v FOO ]", UnsupportedReason::DynamicLValue);

    // …but ordinary forms of the same builtins stay modeled.
    assert!(!parse("unset FOO").has_errors(), "literal unset is fine");
    assert!(
        !parse("printf '%s\\n' hi").has_errors(),
        "plain printf is fine"
    );
    assert!(
        !parse("[ -f /etc/x ]").has_errors(),
        "ordinary test is fine"
    );
}

#[test]
fn reject_over_deep_nesting_is_loud() {
    // inv-top-reject for the depth bound: nesting past the parser cap (MAX_DEPTH=256)
    // must ⊤-reject LOUDLY with the right reason, never silently truncate. This is the
    // one ⊤-trigger whose reason was otherwise unpinned — `totality_hostile_inputs…`
    // only checks no-panic, `deeply_nested…` (cfg) only checks node_count.
    let deep = "(".repeat(300);
    let parsed = parse(&deep);
    assert!(
        parsed.has_errors(),
        "over-deep nesting must emit an Error diagnostic"
    );
    assert!(
        parsed.value.iter().any(|(_, n)| matches!(
            &n.kind,
            NodeKind::Unsupported {
                reason: UnsupportedReason::Unmodeled("nesting too deep"),
                ..
            }
        )),
        "a `nesting too deep` ⊤-node must be present (loud depth reject)"
    );
}

#[test]
fn reject_keeps_going_and_salvages() {
    // Why (dn-7 / inv-top-reject salvage): a reject in the middle of a script must
    // not abort the rest, and salvageable children are retained so unrelated
    // analysis proceeds. The no-`in` `for` still ⊤-rejects (task-L1); the ⊤-node is
    // consumed to its matching `done`, so the trailing `echo` is still seen.
    let parsed = parse("echo before\nfor x; do x; done\necho after");
    let items = script_items(&parsed.value);
    let labels: Vec<&str> = items.iter().map(|&i| label(&parsed.value, i)).collect();
    assert_eq!(
        labels,
        ["Simple", "Unsupported", "Simple"],
        "reject is isolated (no-`in` for stays ⊤, consumed to `done`)"
    );
    assert!(parsed.has_errors());

    // And the eval reject salvages its argument words (non-empty salvaged set).
    let ev = parse("eval some args here");
    match first_unsupported_node(&ev.value) {
        Some(NodeKind::Unsupported { salvaged, .. }) => {
            assert!(!salvaged.is_empty(), "eval args should be salvaged");
        }
        other => panic!("expected Unsupported: {other:?}"),
    }
}

fn first_unsupported_node(ast: &Ast) -> Option<&NodeKind> {
    ast.iter()
        .map(|(_, n)| &n.kind)
        .find(|k| matches!(k, NodeKind::Unsupported { .. }))
}

// ===========================================================================
// Quoting / word-splitting (haz-unquoted): the analyzer decides a command's
// arity and effect-target set from Word::may_split. `echo "$x"` (quoted, one
// field) and `echo $x` (unquoted, may split into many) MUST differ, or an
// unquoted expansion silently looks arity-fixed → unsound effect set.
// ===========================================================================

#[test]
fn quoting_unquoted_param_may_split_quoted_does_not() {
    let unq = parse("echo $x");
    let q = parse("echo \"$x\"");

    let arg_word = |ast: &Ast| -> Vec<WordPart> {
        match kind(ast, script_items(ast)[0]) {
            NodeKind::Simple { words, .. } => match kind(ast, words[1]) {
                NodeKind::Word { parts } => parts.clone(),
                other => panic!("{other:?}"),
            },
            other => panic!("{other:?}"),
        }
    };

    let unq_parts = arg_word(&unq.value);
    let q_parts = arg_word(&q.value);

    assert!(
        Word { parts: &unq_parts }.may_split(),
        "unquoted $x must report may_split=true"
    );
    assert!(
        !Word { parts: &q_parts }.may_split(),
        "quoted \"$x\" must report may_split=false"
    );

    // Structural witness of the losslessness: unquoted is a bare Param; quoted is
    // a DoubleQuoted wrapping the Param.
    assert!(matches!(unq_parts.as_slice(), [WordPart::Param { .. }]));
    assert!(matches!(q_parts.as_slice(), [WordPart::DoubleQuoted(_)]));
}

#[test]
fn quoting_single_quotes_are_literal_no_split() {
    // Why: `'$x'` must be a literal `$x`, never an expansion — single-quote is the
    // one context with zero expansion; getting this wrong invents a phantom param.
    let p = parse("echo '$x *'");
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::Simple { words, .. } => match kind(&p.value, words[1]) {
            NodeKind::Word { parts } => {
                assert!(matches!(parts.as_slice(), [WordPart::SingleQuoted(s)] if s == "$x *"));
                assert!(!Word { parts }.may_split(), "single-quoted never splits");
            }
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }
}

// ===========================================================================
// Oracle idioms the task calls out beyond the fixture.
// ===========================================================================

#[test]
fn idiom_standalone_assignment_statement() {
    // Why (dn-1 anchor): `oracle_kind=package` is how an oracle declares its kind —
    // a bare assignment with NO command word. Must be an Assign, value present.
    let p = parse("oracle_kind=package");
    assert!(!p.has_errors());
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::Simple { assigns, words, .. } => {
            assert!(words.is_empty(), "assignment-only command has no words");
            assert_eq!(assigns.len(), 1);
            match kind(&p.value, assigns[0]) {
                NodeKind::Assign { name, value, .. } => {
                    assert_eq!(name, "oracle_kind");
                    assert!(value.is_some(), "value `package` present");
                }
                other => panic!("{other:?}"),
            }
        }
        other => panic!("{other:?}"),
    }

    // `x=` is an explicit-empty assignment (value None) — distinct from `x=""`.
    let e = parse("x=");
    match kind(&e.value, script_items(&e.value)[0]) {
        NodeKind::Simple { assigns, .. } => match kind(&e.value, assigns[0]) {
            NodeKind::Assign { value, .. } => assert!(value.is_none()),
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }
}

#[test]
fn idiom_function_definition() {
    // Why: `name() { … }` is the oracle/library packaging idiom. Body must be the
    // group, and the name captured.
    let p = parse("install_nginx() { apt-get update; apt-get install -y nginx; }");
    assert!(!p.has_errors());
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::FuncDef { name, body, .. } => {
            assert_eq!(name, "install_nginx");
            assert!(matches!(kind(&p.value, *body), NodeKind::Group { .. }));
        }
        other => panic!("expected FuncDef: {other:?}"),
    }
}

#[test]
fn idiom_grep_pipeline() {
    // Why: `cmd | grep -qE '…'` is the probe-shaped idiom. Two-stage pipeline; the
    // grep pattern stays a single-quoted literal (no expansion, lossless).
    let p = parse("dpkg -s nginx | grep -qE '^Status: .*installed'");
    assert!(!p.has_errors());
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::Pipeline { negated, stages } => {
            assert!(!negated);
            assert_eq!(stages.len(), 2);
            assert_eq!(first_word_literal(&p.value, stages[0]), Some("dpkg"));
            assert_eq!(first_word_literal(&p.value, stages[1]), Some("grep"));
        }
        other => panic!("expected Pipeline: {other:?}"),
    }
}

#[test]
fn andor_is_left_associative() {
    // Why: `a && b || c` must nest as `(a && b) || c` (POSIX equal precedence,
    // left assoc). The CFG short-circuit edges depend on this nesting.
    let p = parse("a && b || c");
    match kind(&p.value, script_items(&p.value)[0]) {
        NodeKind::AndOr { op, left, right } => {
            assert_eq!(
                *op,
                dorc_syntax::AndOrOp::Or,
                "outermost op is the last `||`"
            );
            assert!(matches!(
                kind(&p.value, *left),
                NodeKind::AndOr {
                    op: dorc_syntax::AndOrOp::And,
                    ..
                }
            ));
            assert_eq!(first_word_literal(&p.value, *right), Some("c"));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn determinism_same_input_same_arena() {
    // Why (inv-determinism): the kernel is a pure function of its input. Two parses
    // of the same bytes must yield byte-identical arenas (no HashMap iteration, no
    // address-dependent ordering leaking into output).
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/pi-webhost.book.sh"
    ));
    let a = parse(src);
    let b = parse(src);
    assert_eq!(a.value.iter().count(), b.value.iter().count(), "node count");
    for ((ia, na), (ib, nb)) in a.value.iter().zip(b.value.iter()) {
        assert_eq!(ia, ib, "id sequence");
        assert_eq!(na.span, nb.span, "spans must match deterministically");
        assert_eq!(label(&a.value, ia), label(&b.value, ib), "node kind per id");
    }
    assert_eq!(a.diags.len(), b.diags.len());
}
