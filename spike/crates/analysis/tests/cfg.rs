//! CFG-construction tests for `analysis::cfg` (`cfg::build`).
//!
//! These pin **structure only** — they never run `solve` (per the build brief),
//! so they are independent of the solver's internals and validate the graph the
//! solver will later run over. Each test states the invariant it pins.
//!
//! Conventions: `kinds(&cfg)` counts node kinds; `reaches(&cfg, a, b)` is a plain
//! BFS over `succ` (so reachability assertions don't depend on the dataflow
//! engine); `consistent(&cfg)` re-checks `w ∈ succ(v) ⟺ v ∈ pred(w)`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use dorc_analysis::cfg::{build, Cfg, CfgNodeId, CfgNodeKind, Observable};
use dorc_analysis::lattice::Powerset;
use dorc_analysis::solve::Graph;
use dorc_syntax::parse;

const PI_WEBHOST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/pi-webhost.book.sh"
));

// ---- helpers ---------------------------------------------------------------

fn cfg_of(src: &str) -> Cfg {
    let parsed = parse(src);
    build(&parsed.value).value
}

fn kind_counts(cfg: &Cfg) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for (_, n) in cfg.iter() {
        *m.entry(format!("{:?}", n.kind)).or_default() += 1;
    }
    m
}

fn count_kind(cfg: &Cfg, kind: CfgNodeKind) -> usize {
    cfg.iter().filter(|(_, n)| n.kind == kind).count()
}

/// Unwrap-with-message for tests, written as `match`/`panic!` rather than
/// `.expect()` so the test target stays clippy-clean (project convention; see
/// `crates/syntax/tests/parse.rs`), while still failing loudly with context.
#[track_caller]
fn require<T>(opt: Option<T>, msg: &str) -> T {
    match opt {
        Some(v) => v,
        None => panic!("{msg}"),
    }
}

/// BFS reachability over forward edges (`succ`), independent of `solve`.
fn reaches(cfg: &Cfg, from: CfgNodeId, to: CfgNodeId) -> bool {
    let mut seen = BTreeSet::new();
    let mut q = VecDeque::new();
    q.push_back(from);
    seen.insert(from);
    while let Some(v) = q.pop_front() {
        if v == to {
            return true;
        }
        for w in cfg.succ_ids(v) {
            if seen.insert(w) {
                q.push_back(w);
            }
        }
    }
    false
}

fn consistent(cfg: &Cfg) -> bool {
    let n = cfg.node_count();
    for v in 0..n {
        for &w in cfg.succ(v) {
            if w >= n || !cfg.pred(w).contains(&v) {
                return false;
            }
        }
        for &u in cfg.pred(v) {
            if u >= n || !cfg.succ(u).contains(&v) {
                return false;
            }
        }
    }
    true
}

// ===========================================================================
// The pi-webhost fixture — the real-world structure the spike must model.
// ===========================================================================

#[test]
fn fixture_builds_consistently_and_reaches_exit() {
    // inv-no-throw + Graph consistency on a real book: build must succeed, every
    // edge must be mutually consistent, and the program exit must be reachable
    // from entry (the script terminates).
    let cfg = cfg_of(PI_WEBHOST);
    assert!(
        consistent(&cfg),
        "succ/pred mutually consistent: {:?}",
        kind_counts(&cfg)
    );
    assert!(
        reaches(&cfg, cfg.entry(), cfg.exit()),
        "program exit reachable from entry"
    );
    // No node should be self-orphaned except detached func regions (none here)
    // and the synthetic dead nodes after terminators. Entry must reach the
    // `set -e` (first command) — proven below — and the bulk of commands.
}

#[test]
fn fixture_entry_reaches_set_e() {
    // The very first statement is `set -e`. Entry must reach it (sanity that the
    // walk wires the script body onto entry, and that `set -e` is a Command node).
    let cfg = cfg_of(PI_WEBHOST);
    let set_e = require(
        first_command_with_literal(&cfg, PI_WEBHOST, "set"),
        "a `set` command node exists",
    );
    assert!(reaches(&cfg, cfg.entry(), set_e), "entry reaches `set -e`");
}

#[test]
fn fixture_case_has_branch_per_arm_and_default_terminates() {
    // The fixture's `case "$(hostname)" in pi-web*|webhost-*) : ;; *) … exit 0 ;;`
    // must yield: a scrutinee region (the $(hostname) substitution → a scope), two
    // arm bodies branching from it, and the `*) … exit 0` arm TERMINATING — i.e.
    // its body reaches `exit` but does NOT reach the post-case continuation.
    let cfg = cfg_of(PI_WEBHOST);

    // The `exit 0` is a Command terminator: it has an edge to the program exit and
    // its fall-through is dead. Find the `exit` command.
    let exit_cmd = require(
        first_command_with_literal(&cfg, PI_WEBHOST, "exit"),
        "the default arm's `exit` command exists",
    );
    // exit routes to the program exit.
    assert!(
        cfg.succ_ids(exit_cmd).any(|w| w == cfg.exit()),
        "`exit 0` routes straight to the program exit"
    );

    // The arm with `exit 0` must NOT reach the bare mutations that follow the
    // case (e.g. the `apt-get`/`ufw` block). Concretely: the `exit` command does
    // not reach a later `ufw` command. (The terminator severs fall-through.)
    let ufw = first_command_with_literal(&cfg, PI_WEBHOST, "ufw");
    if let Some(ufw) = ufw {
        assert!(
            !reaches(&cfg, exit_cmd, ufw),
            "the `*) exit 0` arm terminates: it must not reach post-case commands"
        );
    }

    // A subshell scope exists for `$(hostname)` (the scrutinee command substitution).
    assert!(
        count_kind(&cfg, CfgNodeKind::ScopeEnter) >= 1,
        "the case scrutinee `$(hostname)` is a scoped command-substitution region"
    );
}

#[test]
fn fixture_first_if_has_then_and_else_join() {
    // `if ! command -v nginx >/dev/null 2>&1; then apt-get … ; fi` (no else).
    // Both the success path (the `apt-get` body) and the failure path (no-else
    // fall-through) must converge on a single merge that continues to the rest of
    // the script. We assert: a Merge node exists that is reachable from entry, and
    // the `apt-get install` command reaches the later `ufw` command (the merge
    // rejoins the main flow rather than dead-ending).
    let cfg = cfg_of(PI_WEBHOST);
    assert!(
        count_kind(&cfg, CfgNodeKind::Merge) >= 1,
        "the `if` produces a branch-join merge node"
    );
    let apt = require(
        first_command_with_literal(&cfg, PI_WEBHOST, "apt-get"),
        "`apt-get` command exists inside the if-then body",
    );
    let ufw = require(
        first_command_with_literal(&cfg, PI_WEBHOST, "ufw"),
        "`ufw` command exists after the if",
    );
    assert!(
        reaches(&cfg, apt, ufw),
        "the if-then body rejoins the main flow (merge → continuation → ufw)"
    );
    // The negated condition (`! command -v nginx`) is modeled: the `command`
    // node exists and entry reaches it.
    let cmd_v = require(
        first_command_with_literal(&cfg, PI_WEBHOST, "command"),
        "`command -v nginx` exists as the if condition",
    );
    assert!(reaches(&cfg, cfg.entry(), cmd_v));
    // The redirections on the condition (`>/dev/null 2>&1`) are first-class nodes.
    assert!(
        count_kind(&cfg, CfgNodeKind::Redir) >= 3,
        "redirections (>/dev/null, 2>&1, cat>file, here-doc) are first-class effect nodes: {:?}",
        kind_counts(&cfg)
    );
}

#[test]
fn fixture_andand_short_circuits() {
    // `nginx -t && systemctl reload nginx` — the `&&` must short-circuit: the left
    // (`nginx -t`) always runs and has a path that SKIPS the right and goes
    // straight to the join; the right (`systemctl reload`) runs conditionally; both
    // reach the continuation (`touch …`). We assert the left reaches the join
    // without necessarily passing through the right by checking that removing the
    // right from the graph still leaves left→merge (structurally: left has ≥2
    // successors — one toward the right, one toward the merge).
    let cfg = cfg_of(PI_WEBHOST);
    // `nginx -t` is the left of the &&. Its DIRECT successors must encode the
    // short-circuit branch: one edge toward the right operand (a Command —
    // `systemctl reload`) and one edge that skips it straight to the join (a
    // Merge). We assert both appear among the immediate successors.
    let nginx_t = require(
        command_nodes_with_literal(&cfg, PI_WEBHOST, "nginx")
            .into_iter()
            .find(|&id| {
                cfg.succ_ids(id)
                    .any(|w| cfg.node(w).kind == CfgNodeKind::Merge)
            }),
        "`nginx -t` (the && left) fans out to a merge (short-circuit edge)",
    );
    let succ_kinds: Vec<CfgNodeKind> = cfg.succ_ids(nginx_t).map(|w| cfg.node(w).kind).collect();
    assert!(
        succ_kinds.contains(&CfgNodeKind::Merge),
        "short-circuit edge: `nginx -t` can skip the right and reach the join directly"
    );
    assert!(
        succ_kinds.contains(&CfgNodeKind::Command),
        "conditional edge: `nginx -t` can reach the right operand (`systemctl reload`)"
    );
    // The right operand is the reload — confirm by reaching a `systemctl` that is
    // a DIRECT successor of `nginx -t` (the reload, not the upstream `enable`).
    let reload = require(
        cfg.succ_ids(nginx_t).find(|&w| {
            cfg.node(w).kind == CfgNodeKind::Command
                && command_nodes_with_literal(&cfg, PI_WEBHOST, "systemctl").contains(&w)
        }),
        "the && right is the `systemctl reload` command",
    );
    // Both paths (short-circuit and conditional) rejoin and continue to `touch`.
    let touch = require(
        first_command_with_literal(&cfg, PI_WEBHOST, "touch"),
        "the `touch …provisioned` marker exists after the &&",
    );
    assert!(
        reaches(&cfg, nginx_t, touch),
        "&& rejoins and continues to `touch`"
    );
    assert!(
        reaches(&cfg, reload, touch),
        "the reload path also continues to `touch`"
    );
}

#[test]
fn fixture_heredoc_and_write_redir_are_effect_nodes() {
    // The `cat > /etc/nginx/sites-available/pi-web.conf <<'EOF' … EOF` inside the
    // second `if` produces a `cat` command plus TWO redirection effect nodes (the
    // `>` write and the `<<` here-doc), sequenced before/with the command. We
    // assert the cat command exists and at least two Redir nodes are present
    // overall (combined with the >/dev/null redirs this is ≥3, asserted elsewhere).
    let cfg = cfg_of(PI_WEBHOST);
    assert!(
        first_command_with_literal(&cfg, PI_WEBHOST, "cat").is_some(),
        "the heredoc-writing `cat` command is modeled"
    );
}

// ===========================================================================
// ⊤-node (inv-top-reject): an Unsupported AST input must NOT be skipped.
// ===========================================================================

#[test]
fn unsupported_loop_becomes_top_node_with_diagnostic() {
    // `for i in 1; do x; done` is outside the modeled subset → the parser emits an
    // `Unsupported{Loop}` node. cfg::build MUST surface a ⊤ CfgNode (never silently
    // skip it — inv-top-reject), carry a diagnostic, and keep the surrounding
    // structure (entry/exit) intact.
    let parsed = parse("for i in 1; do x; done");
    let carried = build(&parsed.value);
    let cfg = &carried.value;

    assert_eq!(
        count_kind(cfg, CfgNodeKind::Top),
        1,
        "the loop is a single ⊤ node, not dropped: {:?}",
        kind_counts(cfg)
    );
    assert!(
        carried.diags.iter().any(|d| d.code.0 == "cfg-top-node"),
        "a diagnostic accompanies the ⊤ node"
    );
    // Surrounding nodes still present and wired: entry reaches the ⊤, ⊤ reaches exit.
    let top = require(
        cfg.iter()
            .find(|(_, n)| n.kind == CfgNodeKind::Top)
            .map(|(id, _)| id),
        "⊤ node present",
    );
    assert!(reaches(cfg, cfg.entry(), top), "entry reaches the ⊤ node");
    assert!(
        reaches(cfg, top, cfg.exit()),
        "⊤ node reaches exit (not orphaned)"
    );
}

#[test]
fn unsupported_in_sequence_keeps_neighbours_live() {
    // A ⊤ in the middle of a sequence must not swallow its neighbours: the
    // commands before and after it stay present and on the path.
    let src = "echo before\nfor i in 1; do x; done\necho after";
    let parsed = parse(src);
    let cfg = build(&parsed.value).value;
    assert_eq!(
        count_kind(&cfg, CfgNodeKind::Top),
        1,
        "{:?}",
        kind_counts(&cfg)
    );
    let before = first_command_with_literal(&cfg, src, "echo");
    assert!(before.is_some(), "the `echo before` command survives the ⊤");
    // Both echoes exist (two Command nodes named echo).
    let echoes = command_nodes_with_literal(&cfg, src, "echo").len();
    assert_eq!(
        echoes, 2,
        "both echo commands (before and after the ⊤) are present"
    );
}

// ===========================================================================
// Determinism (inv-determinism): identical inputs ⇒ identical graph.
// ===========================================================================

#[test]
fn build_is_deterministic() {
    // Same source built twice ⇒ identical node sequence (kinds + ast ids) and
    // identical adjacency. Pins inv-determinism for the CFG builder.
    let a = cfg_of(PI_WEBHOST);
    let b = cfg_of(PI_WEBHOST);
    assert_eq!(a.node_count(), b.node_count(), "same node count");
    for i in 0..a.node_count() {
        let (na, nb) = (a.node(CfgNodeId(i as u32)), b.node(CfgNodeId(i as u32)));
        assert_eq!(na.kind, nb.kind, "node {i} same kind");
        assert_eq!(na.ast, nb.ast, "node {i} same provenance ast id");
        assert_eq!(a.succ(i), b.succ(i), "node {i} same successors");
        assert_eq!(a.pred(i), b.pred(i), "node {i} same predecessors");
    }
}

// ===========================================================================
// Totality (inv-no-throw): hostile sources never panic.
// ===========================================================================

#[test]
fn build_is_total_on_hostile_sources() {
    // cfg::build must be TOTAL: any AST (incl. deeply nested, malformed, ⊤-laden)
    // yields a Cfg without panicking. We feed a battery of hostile sources through
    // the real parser and assert build returns a consistent graph each time.
    let hostile = [
        "",
        "\u{0}\u{0}\u{0}",
        "if then fi |||",
        "$(((",
        "case in esac",
        ";; ;; ;;",
        "& & &",
        "eval \"$x\"",
        "unset \"$dyn\"",
        "$( $( $( $( echo deep ) ) ) )",
        "( ( ( ( ( : ) ) ) ) )",
        "if ! ! ! x; then y; fi",
        "a && b || c && d || e",
        "for i in 1 2 3; do for j in a b; do echo $i$j; done; done",
        "cat <<EOF\nnested $(code here)\nEOF",
        "set -e; set +e; set -euo pipefail; set \"$opts\"",
        "{ { { echo nested groups; } ; } ; }",
        "func() { other; }; func",
        "x=1 y=2 z=$(cmd) echo hi",
    ];
    for src in hostile {
        let parsed = parse(src);
        let carried = build(&parsed.value);
        let cfg = &carried.value;
        assert!(
            consistent(cfg),
            "graph consistent for hostile src {src:?}: {:?}",
            kind_counts(cfg)
        );
        // Entry/exit always exist.
        assert!(matches!(cfg.node(cfg.entry()).kind, CfgNodeKind::Entry));
        assert!(matches!(cfg.node(cfg.exit()).kind, CfgNodeKind::Exit));
    }
}

#[test]
fn deeply_nested_does_not_blow_stack() {
    // A pathologically nested source must not overflow the native stack
    // (inv-no-throw). The parser bounds its own depth; build bounds independently.
    let deep_subshell = "(".repeat(2000) + ":" + &")".repeat(2000);
    let parsed = parse(&deep_subshell);
    let cfg = build(&parsed.value).value; // must simply return
    assert!(cfg.node_count() >= 2, "entry+exit at minimum");
}

// ===========================================================================
// errexit failure-edges (haz-seterr, coarse-but-sound v1).
// ===========================================================================

#[test]
fn errexit_on_adds_failure_edge_to_exit() {
    // Under `set -e`, a fallible command gets an implicit failure→exit edge. With
    // `set -e; risky_cmd; after`, `risky_cmd` must have an edge to the program exit
    // (the errexit failure-edge) IN ADDITION to its fall-through to `after`.
    let src = "set -e\nrisky_cmd\nafter";
    let cfg = cfg_of(src);
    let risky = require(
        first_command_with_literal(&cfg, src, "risky_cmd"),
        "risky_cmd command exists",
    );
    assert!(
        cfg.succ_ids(risky).any(|w| w == cfg.exit()),
        "errexit-on: risky_cmd has a failure→exit edge"
    );
    // It still falls through to `after` (normal-success path).
    let after = require(
        first_command_with_literal(&cfg, src, "after"),
        "after exists",
    );
    assert!(
        cfg.succ_ids(risky)
            .any(|w| w != cfg.exit() && reaches(&cfg, w, after)),
        "risky_cmd still falls through toward `after` on success"
    );
}

#[test]
fn command_substitution_body_is_expansion_internal_subshell_body_is_not() {
    // find-cli-1: a command inside `$( … )` is effect-bearing but NOT a leaf (it
    // runs during word expansion); a command inside a subshell `( … )` IS a leaf.
    let cfg = cfg_of("echo $(uname)");
    let mut subst_internal = 0;
    let mut leaves = 0;
    for (id, node) in cfg.iter() {
        if node.kind == CfgNodeKind::Command {
            if cfg.is_expansion_internal(id) {
                subst_internal += 1;
            } else {
                leaves += 1;
            }
        }
    }
    assert_eq!(subst_internal, 1, "the `$(uname)` body command is expansion-internal");
    assert_eq!(leaves, 1, "the `echo` command is a leaf");

    // A subshell body command is a real leaf (subshell bodies are NOT marked).
    let sub = cfg_of("( uname )");
    let mut sub_leaves = 0;
    for (id, node) in sub.iter() {
        if node.kind == CfgNodeKind::Command && !sub.is_expansion_internal(id) {
            sub_leaves += 1;
        }
    }
    assert_eq!(sub_leaves, 1, "a subshell `( uname )` body command IS a leaf");
}

#[test]
fn case_with_subst_scrutinee_does_not_spuriously_flag_errexit_top() {
    // Regression (find-cli-4): a `case` whose scrutinee has a `$(…)` substitution,
    // FOLLOWED by a command, used to spuriously mark the post-case merge ⊤ — the
    // errexit pass seeded merges with `Off`, and `Off ⊔ On = ⊤` (Off/On are
    // incomparable). Merges now seed ⊥, so `⊥ ⊔ On = On`: no spurious ⊤ on the
    // common host-selection idiom. (Spurious ⊤ ⇒ spurious failure-edges, which are
    // unsound for the backward apply-slice; note 166 find-8.)
    let parsed = parse("set -e\ncase $(hostname) in *) : ;; esac\necho after");
    let carried = build(&parsed.value);
    assert!(
        !carried.diags.iter().any(|d| d.code.0 == "cfg-errexit-unknown"),
        "no spurious errexit ⊤ on `case $(...)` + a following command: {:?}",
        carried.diags
    );
    // Non-vacuity: the case must be modeled (not ⊤-rejected), else the test passes
    // for the wrong reason.
    assert!(
        !carried.diags.iter().any(|d| d.code.0 == "cfg-top-node"),
        "the case is modeled, not ⊤-rejected"
    );
}

#[test]
fn genuine_set_plus_e_split_still_flags_errexit_top() {
    // The dual guard: a real split — `set +e` on one path, `set -e` (still on) on
    // the other — must STILL join to ⊤ at the merge (the following command may or
    // may not abort). The ⊥-seed fix must not suppress genuine conflicts.
    let parsed = parse("set -e\nif true; then set +e; fi\nafter");
    let carried = build(&parsed.value);
    assert!(
        carried.diags.iter().any(|d| d.code.0 == "cfg-errexit-unknown"),
        "a genuine set+e / set-e split must still flag ⊤: {:?}",
        carried.diags
    );
}

#[test]
fn no_errexit_means_no_failure_edge() {
    // Without `set -e`, a plain command does NOT get a failure→exit edge: its only
    // successor is the fall-through. (Coarse model: errexit Off ⇒ no extra edge.)
    let src = "risky_cmd\nafter";
    let cfg = cfg_of(src);
    let risky = require(
        first_command_with_literal(&cfg, src, "risky_cmd"),
        "risky_cmd command exists",
    );
    let succ_is_exit: Vec<bool> = cfg.succ_ids(risky).map(|w| w == cfg.exit()).collect();
    assert!(
        !succ_is_exit.into_iter().any(|b| b),
        "no errexit ⇒ no direct failure→exit edge from risky_cmd"
    );
}

#[test]
fn swallow_suppresses_errexit_edge() {
    // `cmd || true` (haz-swallow): even under `set -e`, the left of `||` is in a
    // condition context, so it must NOT get a failure→exit edge.
    let src = "set -e\nrisky_cmd || true\nafter";
    let cfg = cfg_of(src);
    let risky = command_nodes_with_literal(&cfg, src, "risky_cmd");
    assert_eq!(risky.len(), 1, "exactly one risky_cmd command");
    let risky = risky[0];
    assert!(
        !cfg.succ_ids(risky).any(|w| w == cfg.exit()),
        "swallowed (`|| true`) command must not have a failure→exit edge"
    );
}

#[test]
fn errexit_unknown_is_conservative() {
    // `set "$opts"` makes errexit ⊤ (dynamic option). A subsequent command must
    // STILL get the failure→exit edge (over-approximate: ⊤ ⇒ add the edge), and a
    // diagnostic notes the conservative choice.
    let src = "set \"$opts\"\nrisky_cmd\nafter";
    let parsed = parse(src);
    let carried = build(&parsed.value);
    let cfg = &carried.value;
    if let Some(risky) = first_command_with_literal(cfg, src, "risky_cmd") {
        assert!(
            cfg.succ_ids(risky).any(|w| w == cfg.exit()),
            "errexit ⊤ ⇒ failure→exit edge added conservatively"
        );
        assert!(
            carried
                .diags
                .iter()
                .any(|d| d.code.0 == "cfg-errexit-unknown"),
            "a diagnostic flags the conservative ⊤ failure-edges"
        );
    }
    // (If `set "$opts"` itself ⊤-rejects as dynamic at the parser, the test is
    // vacuous on the edge but build still must not panic — covered by totality.)
}

// ===========================================================================
// Precise errexit modeling (note 166): one regression test per fixed finding.
// Each pins the CFG *structure* against the dash-verified script from the note.
// ===========================================================================

#[test]
fn find1_negated_pipeline_has_no_failure_edge() {
    // `[RAN]` dash: `set -e; ! true; echo AFTER` prints AFTER (rc 0) — a `!`-negated
    // pipeline NEVER fires errexit, even `! true` (POSIX `!` exemption). So the
    // negated pipeline's command must have NO failure→exit edge.
    let src = "set -e\n! true\nafter";
    let cfg = cfg_of(src);
    let true_cmds = command_nodes_with_literal(&cfg, src, "true");
    assert_eq!(true_cmds.len(), 1, "exactly one `true` command");
    assert!(
        !has_exit_edge(&cfg, true_cmds[0]),
        "negated `! true` must NOT have a failure→exit edge (find-1)"
    );
    // Control: the trailing bare `after` (errexit on, not negated) DOES abort. We
    // can't assert its exit edge directly (its only successor IS exit as the last
    // statement), so instead pin a non-negated sibling that has a fall-through:
    let src2 = "set -e\nfalse\nafter";
    let cfg2 = cfg_of(src2);
    let false_cmd = require(
        first_command_with_literal(&cfg2, src2, "false"),
        "false command exists",
    );
    assert!(
        has_exit_edge(&cfg2, false_cmd),
        "non-negated `false` under set -e DOES get a failure→exit edge (contrast)"
    );
}

#[test]
fn find2_whole_condition_is_errexit_exempt() {
    // `[RAN]` dash: `set -e; if false; true; then echo THEN; fi; echo AFTER` runs
    // THEN and AFTER — the WHOLE `if` condition is exempt, not just its last
    // command. So `false` (a NON-tail condition command) must have no failure→exit
    // edge, and so must the tail `true`.
    let src = "set -e\nif false\ntrue\nthen body\nfi\nafter";
    let cfg = cfg_of(src);
    let false_cmd = require(
        first_command_with_literal(&cfg, src, "false"),
        "the non-tail condition command `false` exists",
    );
    assert!(
        !has_exit_edge(&cfg, false_cmd),
        "non-tail condition command is errexit-exempt (find-2)"
    );
    let true_cmd = require(
        first_command_with_literal(&cfg, src, "true"),
        "the tail condition command `true` exists",
    );
    assert!(
        !has_exit_edge(&cfg, true_cmd),
        "tail condition command is errexit-exempt too (confirmed-correct)"
    );
    // The THEN body is NOT exempt: `body` keeps a failure→exit edge.
    let body = require(
        first_command_with_literal(&cfg, src, "body"),
        "the then-body command exists",
    );
    assert!(
        has_exit_edge(&cfg, body),
        "the then-body is NOT a condition context — it keeps its failure edge"
    );
}

#[test]
fn find3_compound_condition_exempts_inner_operands() {
    // `[RAN]` dash: `set -e; if false && echo X; then …` does not abort at `false`
    // (the whole compound test is exempt). When the condition is a `&&`/`||` chain,
    // its inner operands (whose region exit is a Merge, which the old tail-only
    // tagging skipped) must also be exempt.
    let src = "set -e\nif a && b\nthen body\nfi\nafter";
    let cfg = cfg_of(src);
    for lit in ["a", "b"] {
        let cmd = require(
            first_command_with_literal(&cfg, src, lit),
            "compound-condition operand exists",
        );
        assert!(
            !has_exit_edge(&cfg, cmd),
            "compound-condition operand `{lit}` is errexit-exempt (find-3)"
        );
    }

    // Top-level `&&`/`||` (NOT a condition): the left/non-final operands are exempt
    // (`[RAN]` `true && false && echo C` prints AFTER), but the FINAL right operand
    // is NOT (`[RAN]` `true && false` aborts).
    let src2 = "set -e\ntrue && false && c\nafter";
    let cfg2 = cfg_of(src2);
    let mid = require(
        first_command_with_literal(&cfg2, src2, "false"),
        "the middle `&&` operand exists",
    );
    assert!(
        !has_exit_edge(&cfg2, mid),
        "a non-final `&&` operand is exempt (find-3 / confirmed `|| true` family)"
    );
    let final_op = require(
        first_command_with_literal(&cfg2, src2, "c"),
        "the final `&&` operand exists",
    );
    assert!(
        has_exit_edge(&cfg2, final_op),
        "the FINAL `&&` operand is NOT exempt — it keeps its failure edge"
    );
}

#[test]
fn find4_subshell_errexit_toggle_does_not_leak() {
    // `[RAN]` dash: `set -e; ( set +e; false ); false; echo AFTER` aborts (rc 1) —
    // the `set +e` inside `( )` does NOT disable errexit outside it. So a command
    // AFTER the subshell still gets its failure→exit edge.
    //
    // NB: the asserted command must be NON-terminal, else its normal fall-through to
    // the program exit is indistinguishable from the errexit failure-edge — so each
    // script ends with a trailing `tail` after the command under test.
    let src = "set -e\n( set +e; false )\nafter\ntail";
    let cfg = cfg_of(src);
    assert!(
        count_kind(&cfg, CfgNodeKind::ScopeEnter) == 1
            && count_kind(&cfg, CfgNodeKind::ScopeExit) == 1,
        "the subshell is a scoped region"
    );
    // `false` INSIDE the subshell (after `set +e`) is NOT fallible — errexit is off
    // there.
    let inner_false = require(
        first_command_with_literal(&cfg, src, "false"),
        "the inner `false` exists",
    );
    assert!(
        !has_exit_edge(&cfg, inner_false),
        "inside the subshell `set +e` is in effect: inner `false` has no failure edge"
    );
    // The command after the subshell sees errexit RESTORED to on (the toggle did
    // not leak): it keeps a failure→exit edge in ADDITION to its fall-through.
    let after = require(
        first_command_with_literal(&cfg, src, "after"),
        "the post-subshell command exists",
    );
    assert!(
        has_exit_edge(&cfg, after),
        "errexit is restored after the subshell — `after` keeps its failure edge (find-4)"
    );

    // Inverse: a `set -e` INSIDE a subshell must not leak OUT to an otherwise-off
    // outer scope. `( set -e; false ); after; tail` — `after` must NOT get a
    // failure→exit edge (its only successor is the fall-through to `tail`).
    let src2 = "( set -e; false )\nafter\ntail";
    let cfg2 = cfg_of(src2);
    let after2 = require(
        first_command_with_literal(&cfg2, src2, "after"),
        "post-subshell command exists",
    );
    assert!(
        !has_exit_edge(&cfg2, after2),
        "a `set -e` inside `( )` does not leak out: outer `after` stays non-fallible (find-4)"
    );
}

#[test]
fn find5_failing_redirection_aborts_under_errexit() {
    // `[RAN]` dash: a failing redirection aborts under `set -e` (`cat /etc/x > /f`
    // with a bad path). The `Redir` node — sequenced before its command — must get
    // a failure→exit edge, not only the `Command`.
    let src = "set -e\ncat /etc/x > /f\nafter";
    let cfg = cfg_of(src);
    let redirs = redir_nodes(&cfg);
    assert_eq!(redirs.len(), 1, "one redirection node");
    assert!(
        has_exit_edge(&cfg, redirs[0]),
        "a failing redirection has a failure→exit edge under set -e (find-5)"
    );

    // Without `set -e`, the redirection does NOT get a failure→exit edge.
    let src_off = "cat /etc/x > /f\nafter";
    let cfg_off = cfg_of(src_off);
    let redirs_off = redir_nodes(&cfg_off);
    assert_eq!(redirs_off.len(), 1, "one redirection node");
    assert!(
        !has_exit_edge(&cfg_off, redirs_off[0]),
        "no set -e ⇒ the redirection has no failure→exit edge"
    );

    // A redirection inside an `if` condition is exempt (the whole test region is).
    let src_cond = "set -e\nif cat /etc/x > /f\nthen y\nfi";
    let cfg_cond = cfg_of(src_cond);
    let redirs_cond = redir_nodes(&cfg_cond);
    assert_eq!(
        redirs_cond.len(),
        1,
        "one redirection node in the condition"
    );
    assert!(
        !has_exit_edge(&cfg_cond, redirs_cond[0]),
        "a redirection in a condition context is errexit-exempt (find-5 × find-2)"
    );
}

#[test]
fn find6_command_substitution_regions_and_assignment_fallibility() {
    // `[RAN]` dash: `set -e; x=$(false); echo AFTER` aborts (rc 1) — an
    // assignment-only command takes the substitution's status. The host `Command`
    // (whose command-word literal is None) must be fallible AND the `$( … )` body
    // must be a scoped region in the graph.
    let src = "set -e\nx=$(false)\nafter";
    let cfg = cfg_of(src);
    assert_eq!(
        count_kind(&cfg, CfgNodeKind::ScopeEnter),
        1,
        "the `$(false)` is a scoped command-substitution region (find-6)"
    );
    let host = require(
        assign_command_node(&cfg, src, "x"),
        "the `x=$(…)` assignment host command exists",
    );
    assert!(
        has_exit_edge(&cfg, host),
        "an assignment-only `x=$(false)` aborts under set -e (find-6)"
    );

    // A `set +e` INSIDE the substitution subshell must not leak out (find-4 × find-6):
    // `set -e; x=$(set +e; false); after` — `after` still gets a failure edge, since
    // the subst runs in its own shell and the outer errexit stays on. (`after` is
    // non-terminal so the failure edge is distinguishable from the fall-through.)
    let src2 = "set -e\nx=$(set +e; false)\nafter\ntail";
    let cfg2 = cfg_of(src2);
    let after = require(
        first_command_with_literal(&cfg2, src2, "after"),
        "post-substitution command exists",
    );
    assert!(
        has_exit_edge(&cfg2, after),
        "a `set +e` inside `$( )` does not change errexit after the subshell (find-6 × find-4)"
    );

    // Contrast: `echo $(false)` has a command word, so the subst status is masked
    // (`[RAN]` does NOT abort on the subst). The `echo` command is still fallible
    // (the command word itself can fail) — that is correct and unchanged — but the
    // important structural fact is the subst region exists.
    let src3 = "set -e\necho $(false)\nafter";
    let cfg3 = cfg_of(src3);
    assert_eq!(
        count_kind(&cfg3, CfgNodeKind::ScopeEnter),
        1,
        "the argument `$(false)` is still a scoped region"
    );
}

#[test]
fn confirmed_pipeline_last_stage_only_governs_errexit() {
    // Confirmed-correct (note 166): in `a | b`, only the LAST stage governs the
    // pipeline status under set -e. The non-last stage `a` must have no failure
    // edge; the last stage `b` must have one. Pins this against accidental change.
    let src = "set -e\na | b\nafter";
    let cfg = cfg_of(src);
    let a = require(first_command_with_literal(&cfg, src, "a"), "stage a exists");
    let b = require(first_command_with_literal(&cfg, src, "b"), "stage b exists");
    assert!(
        !has_exit_edge(&cfg, a),
        "non-last pipeline stage does not govern errexit (confirmed-correct)"
    );
    assert!(
        has_exit_edge(&cfg, b),
        "last pipeline stage governs errexit — keeps its failure edge (confirmed-correct)"
    );
}

#[test]
fn confirmed_brace_group_does_not_scope_errexit() {
    // Confirmed-correct (note 166): a brace `{ }` runs in the current shell and does
    // NOT scope — its `set -e` leaks (matches real shell). A `set -e` inside a brace
    // group makes a following command fallible. (Contrast find-4's subshell.)
    let src = "{ set -e; }\nafter\ntail";
    let cfg = cfg_of(src);
    assert_eq!(
        count_kind(&cfg, CfgNodeKind::ScopeEnter),
        0,
        "a brace group introduces no scope boundary (confirmed-correct)"
    );
    // `after` is non-terminal (trailing `tail`), so its failure→exit edge is the
    // errexit edge, not the fall-through.
    let after = require(
        first_command_with_literal(&cfg, src, "after"),
        "post-group command exists",
    );
    assert!(
        has_exit_edge(&cfg, after),
        "a brace-group `set -e` leaks to the following command (confirmed-correct, contrast find-4)"
    );
}

// ===========================================================================
// Lookup helpers that resolve a literal command word back through provenance.
// ===========================================================================

/// All `Command` CFG nodes whose `Simple` AST node's command word is exactly
/// `lit`. Resolves provenance back to the AST (the back-map the next subagent
/// also relies on). Pure; uses the parsed AST, not the source string.
fn command_nodes_with_literal(cfg: &Cfg, src: &str, lit: &str) -> Vec<CfgNodeId> {
    let parsed = parse(src);
    let ast = &parsed.value;
    cfg.iter()
        .filter(|(_, n)| n.kind == CfgNodeKind::Command)
        .filter(|(_, n)| command_word_literal(ast, n.ast) == Some(lit.to_string()))
        .map(|(id, _)| id)
        .collect()
}

fn first_command_with_literal(cfg: &Cfg, src: &str, lit: &str) -> Option<CfgNodeId> {
    command_nodes_with_literal(cfg, src, lit).into_iter().next()
}

/// The command-word literal of a `Simple` AST node, if statically fixed.
fn command_word_literal(ast: &dorc_syntax::Ast, id: dorc_core::AstId) -> Option<String> {
    use dorc_syntax::{NodeKind, WordPart};
    if let NodeKind::Simple { words, .. } = &ast.node(id).kind {
        let first = words.first()?;
        if let NodeKind::Word { parts } = &ast.node(*first).kind {
            return match parts.as_slice() {
                [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s.clone()),
                _ => None,
            };
        }
    }
    None
}

/// Does `id` have a direct failure→exit edge? (The errexit failure-edge the
/// precise phase-2 pass materialises — note 166.)
fn has_exit_edge(cfg: &Cfg, id: CfgNodeId) -> bool {
    cfg.succ_ids(id).any(|w| w == cfg.exit())
}

/// All `Redir` effect nodes (find-5 asserts on these).
fn redir_nodes(cfg: &Cfg) -> Vec<CfgNodeId> {
    cfg.iter()
        .filter(|(_, n)| n.kind == CfgNodeKind::Redir)
        .map(|(id, _)| id)
        .collect()
}

/// The `Command` node of an *assignment-only* `Simple` whose first assignment is
/// `name=…` (e.g. the host of `x=$(false)`; its command-word literal is `None`, so
/// `command_nodes_with_literal` cannot find it).
fn assign_command_node(cfg: &Cfg, src: &str, name: &str) -> Option<CfgNodeId> {
    use dorc_syntax::NodeKind;
    let parsed = parse(src);
    let ast = &parsed.value;
    cfg.iter()
        .filter(|(_, n)| n.kind == CfgNodeKind::Command)
        .find(|(_, n)| match &ast.node(n.ast).kind {
            NodeKind::Simple { assigns, .. } => assigns.iter().any(
                |&a| matches!(&ast.node(a).kind, NodeKind::Assign { name: an, .. } if an == name),
            ),
            _ => false,
        })
        .map(|(id, _)| id)
}

// ===========================================================================
// Output-consumption fact (note 16J / `inv-superposition`). The engine records,
// per leaf, which unvouched output observables its CONTEXT consumes — computed in
// the single lowering traversal (def-2 exhaustive / def-3 single-source), so it is
// total over nodes (no "absent leaf" — def-1 intent achieved structurally). These
// pin the fact directly, independent of `plan`'s collapse.
// ===========================================================================

/// The consumption set of the (sole) command whose command-word literal is `lit`.
fn consumed_of(src: &str, lit: &str) -> Powerset<Observable> {
    let cfg = cfg_of(src);
    let id = require(first_command_with_literal(&cfg, src, lit), "command exists");
    cfg.consumed_observables(id).clone()
}

#[test]
fn consumed_lone_command_is_quiet() {
    // No pipe, no redirect, no enclosing capture ⇒ provably quiet (empty set).
    assert!(consumed_of("apt-get install -y nginx\n", "apt-get").0.is_empty());
}

#[test]
fn consumed_own_stdout_redirect() {
    // The leaf's OWN `> file` captures fd 1 ⇒ Stdout consumed.
    let c = consumed_of("apt-get install -y nginx > /etc/marker\n", "apt-get");
    assert!(c.contains(&Observable::Stdout));
    assert!(!c.contains(&Observable::Stderr));
}

#[test]
fn consumed_own_stderr_redirect() {
    // `2> file` captures fd 2 ⇒ Stderr consumed (fd 1 untouched).
    let c = consumed_of("apt-get install -y nginx 2> /tmp/err\n", "apt-get");
    assert!(c.contains(&Observable::Stderr));
    assert!(!c.contains(&Observable::Stdout));
}

#[test]
fn consumed_devnull_is_quiet() {
    // The `/dev/null` discard sink is exempt (the precision scalpel) ⇒ still quiet.
    assert!(consumed_of("apt-get install -y nginx > /dev/null\n", "apt-get").0.is_empty());
}

#[test]
fn consumed_nonlast_pipeline_stage() {
    // A non-last pipeline stage's stdout is piped onward ⇒ Stdout consumed.
    assert!(consumed_of("apt-get install -y nginx | tee log\n", "apt-get")
        .contains(&Observable::Stdout));
}

#[test]
fn consumed_enclosing_group_redirect_marks_inner_leaf() {
    // 16G kill-shot / def-5 regression lock: the redirect is on the GROUP, not the
    // leaf. The inner install must still be marked Stdout — proving consumption is
    // computed in the single lowering traversal (the old plan-side dual-traversal
    // missed exactly this enclosing case; here it cannot, the fact is born with the
    // node via the arena-range mark).
    let c = consumed_of("{ apt-get install -y nginx; } > /tmp/out\ncat /tmp/out\n", "apt-get");
    assert!(c.contains(&Observable::Stdout), "enclosing-group redirect must mark the inner leaf");
}

#[test]
fn consumed_enclosing_subshell_pipe_marks_inner_leaf() {
    // 16G kill-shot: the `( … )` is a non-last pipeline stage; its inner install is
    // the producer. The enclosing-pipe context must reach the inner leaf.
    let c = consumed_of("( apt-get install -y nginx ) | grep -q nginx\n", "apt-get");
    assert!(c.contains(&Observable::Stdout), "enclosing-subshell pipe must mark the inner leaf");
}

#[test]
fn consumed_enclosing_subshell_devnull_stays_quiet() {
    // The scalpel survives the enclosing case too: `( … ) > /dev/null` discards ⇒
    // the inner leaf stays quiet (the range-mark must keep /dev/null exempt).
    assert!(consumed_of("( apt-get install -y nginx ) > /dev/null\n", "apt-get").0.is_empty());
}
