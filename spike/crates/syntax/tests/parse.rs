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
fn reject_loops_are_unsupported_loop() {
    // Why: for/while/until are outside the modeled subset; mis-parsing one as a
    // simple command named "for" would let the analyzer walk a phantom CFG.
    assert_rejects("for i in 1 2; do echo $i; done", UnsupportedReason::Loop);
    assert_rejects("while true; do :; done", UnsupportedReason::Loop);
    assert_rejects("until false; do :; done", UnsupportedReason::Loop);
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
    // analysis proceeds. We reject the loop but still see the trailing `echo`.
    let parsed = parse("echo before\nfor i in 1; do x; done\necho after");
    let items = script_items(&parsed.value);
    let labels: Vec<&str> = items.iter().map(|&i| label(&parsed.value, i)).collect();
    assert_eq!(
        labels,
        ["Simple", "Unsupported", "Simple"],
        "reject is isolated"
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
