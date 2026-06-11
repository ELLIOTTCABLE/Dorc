//! CFG-construction tests for `analysis::cfg` (`cfg::build`).
//!
//! These pin **structure only** — they never run `solve` (per the build brief),
//! so they are independent of the solver's internals and validate the graph the
//! solver will later run over. Each test states the invariant it pins.
//!
//! Conventions: `kinds(&cfg)` counts node kinds; `reaches(&cfg, a, b)` is a plain
//! BFS over `succ` (so reachability assertions don't depend on the dataflow
//! engine); `consistent(&cfg)` re-checks `w ∈ succ(v) ⟺ v ∈ pred(w)`.

// An integration-test crate is a separate crate to clippy, so the `allow-*-in-tests`
// clippy.toml keys cover `#[test]` bodies but NOT module-level test helpers
// (`require`, `kind_counts`). These are the "tests may panic/index/cast" allowances
// the policy intends, spelled at the file top because the keys can't reach helpers.
#![expect(
    clippy::panic,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    reason = "test helpers: panic-based require(), a counter increment, and count-to-u32 — the in-tests allowances the policy intends"
)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use dorc_analysis::cfg::{Cfg, CfgNodeId, CfgNodeKind, build};
use dorc_analysis::lattice::Powerset;
use dorc_analysis::solve::Graph;
use dorc_core::Channel;
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

// ===========================================================================
// ⊤-node (inv-top-reject): an Unsupported AST input must NOT be skipped.
// ===========================================================================

#[test]
fn unsupported_loop_becomes_top_node_with_diagnostic() {
    // task-L1: a literal-list loop now PARSES (no ⊤), but a no-`in` `for` (iterates
    // runtime "$@") is still outside the subset → the parser emits an
    // `Unsupported{Loop}` node. cfg::build MUST surface a ⊤ CfgNode (never silently
    // skip it — inv-top-reject), carry a diagnostic, and keep the surrounding
    // structure (entry/exit) intact.
    let parsed = parse("for x; do x; done");
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
    // commands before and after it stay present and on the path. (A no-`in` `for`
    // still ⊤-rejects post-L1; a literal-list loop would NOT be ⊤ here.)
    let src = "echo before\nfor x; do x; done\necho after";
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
// Loops (task-L1, `209` brk-1): the first REAL cyclic CFG — back-edge present,
// no subshell scope, body commands are in-loop leaves.
// ===========================================================================

#[test]
fn for_loop_lowers_a_back_edge_not_a_top_node() {
    // The structural heart of L1: a literal-list `for` lowers to a LoopHead with a
    // genuine BACK-EDGE (body_exit → head) and an exit edge (head → merge) — a cyclic
    // CFG, NOT a ⊤ node. This is the first real cycle the worklist sees (the dataflow
    // tests prove it converges; this pins the graph it runs over).
    let cfg = cfg_of("for f in a b; do echo \"$f\"; done");
    assert_eq!(
        count_kind(&cfg, CfgNodeKind::Top),
        0,
        "a parsed loop is NOT ⊤"
    );
    assert_eq!(count_kind(&cfg, CfgNodeKind::LoopHead), 1, "one loop head");

    let head = require(
        cfg.iter()
            .find(|(_, n)| n.kind == CfgNodeKind::LoopHead)
            .map(|(id, _)| id),
        "LoopHead present",
    );
    let body = require(
        cfg.iter()
            .find(|(_, n)| n.kind == CfgNodeKind::Command)
            .map(|(id, _)| id),
        "the body `echo` command",
    );
    // The back-edge: the head is reachable FROM the body (a cycle), and the body is
    // reachable from the head — i.e. they are mutually reachable.
    assert!(reaches(&cfg, head, body), "head reaches the body (enter)");
    assert!(
        reaches(&cfg, body, head),
        "body reaches the head again (THE back-edge — a real cycle)"
    );
    // The head also has the exit edge to a merge (ran-zero-times / list-exhausted),
    // so the post-loop continuation is reachable.
    assert!(
        cfg.succ_ids(head)
            .any(|s| cfg.node(s).kind == CfgNodeKind::Merge),
        "head has an exit edge to a merge (loop can end / run zero times)"
    );
    assert!(
        consistent(&cfg),
        "succ/pred stay mutually consistent over the cycle"
    );
}

#[test]
fn while_loop_lowers_condition_region_and_back_edge() {
    // `while`: a condition region between head and body, plus the back-edge. The
    // condition command is in-loop and consumes its status per-iteration
    // (StatusIterated — pinned in the consumed-observable tests); here we pin the
    // cyclic shape + the body reachability.
    let src = "while dpkg -s nginx; do apt-get install -y nginx; done";
    let cfg = cfg_of(src);
    assert_eq!(
        count_kind(&cfg, CfgNodeKind::Top),
        0,
        "a parsed while is NOT ⊤"
    );
    assert_eq!(count_kind(&cfg, CfgNodeKind::LoopHead), 1);
    let head = require(
        cfg.iter()
            .find(|(_, n)| n.kind == CfgNodeKind::LoopHead)
            .map(|(id, _)| id),
        "LoopHead",
    );
    // The install body command is mutually reachable with the head (back-edge).
    let install = require(
        command_nodes_with_literal(&cfg, src, "apt-get")
            .first()
            .copied(),
        "the install body command",
    );
    assert!(reaches(&cfg, head, install));
    assert!(
        reaches(&cfg, install, head),
        "the back-edge closes the cycle"
    );
    // The condition `dpkg` is also in the cycle (it runs every iteration).
    let dpkg = require(
        command_nodes_with_literal(&cfg, src, "dpkg")
            .first()
            .copied(),
        "the while-condition command",
    );
    assert!(cfg.in_loop_body(dpkg), "the while condition is in-loop");
    assert!(consistent(&cfg));
}

#[test]
fn loop_body_commands_are_in_loop_post_loop_is_not() {
    // item-2(d) leaf-seam + item-3 in-loop floor: a body command is an in-loop leaf
    // (the structural render floor), but a command AFTER the loop is NOT in-loop, so
    // the value below a converged loop unlocks normally (the brk-1 value-unlock). The
    // two installs are command-word `apt-get`; exactly one is in-loop (the body) and
    // one is not (post-loop) — counted via `in_loop_body`.
    let src = "for f in a b; do apt-get install -y \"$f\"; done\napt-get install -y curl";
    let cfg = cfg_of(src);
    let installs = command_nodes_with_literal(&cfg, src, "apt-get");
    assert_eq!(installs.len(), 2, "two apt-get command leaves");
    let in_loop = installs.iter().filter(|&&id| cfg.in_loop_body(id)).count();
    let post_loop = installs.iter().filter(|&&id| !cfg.in_loop_body(id)).count();
    assert_eq!(in_loop, 1, "exactly the loop-body install is in-loop");
    assert_eq!(post_loop, 1, "exactly the post-loop install is not in-loop");
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
    assert_eq!(
        subst_internal, 1,
        "the `$(uname)` body command is expansion-internal"
    );
    assert_eq!(leaves, 1, "the `echo` command is a leaf");

    // A subshell body command is a real leaf (subshell bodies are NOT marked).
    let sub = cfg_of("( uname )");
    let mut sub_leaves = 0;
    for (id, node) in sub.iter() {
        if node.kind == CfgNodeKind::Command && !sub.is_expansion_internal(id) {
            sub_leaves += 1;
        }
    }
    assert_eq!(
        sub_leaves, 1,
        "a subshell `( uname )` body command IS a leaf"
    );
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
        !carried
            .diags
            .iter()
            .any(|d| d.code.0 == "cfg-errexit-unknown"),
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
        carried
            .diags
            .iter()
            .any(|d| d.code.0 == "cfg-errexit-unknown"),
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

    // `||` mirrors `&&`, including the `|| true` swallow idiom (haz-swallow): the
    // LEFT of a top-level `||` is a condition context ⇒ exempt. (Folded in from the
    // old standalone `swallow_suppresses_errexit_edge`.)
    let src3 = "set -e\nrisky || true\nafter";
    let cfg3 = cfg_of(src3);
    let swallowed = require(
        first_command_with_literal(&cfg3, src3, "risky"),
        "the `|| true`-swallowed left operand exists",
    );
    assert!(
        !has_exit_edge(&cfg3, swallowed),
        "the left of a top-level `||` (the `|| true` swallow) is errexit-exempt"
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
                [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s.clone()),
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
fn consumed_of(src: &str, lit: &str) -> Powerset<Channel> {
    let cfg = cfg_of(src);
    let id = require(first_command_with_literal(&cfg, src, lit), "command exists");
    cfg.consumed_observables(id).clone()
}

#[test]
fn consumed_lone_command_is_quiet() {
    // No pipe, no redirect, no enclosing capture ⇒ provably quiet (empty set).
    assert!(
        consumed_of("apt-get install -y nginx\n", "apt-get")
            .0
            .is_empty()
    );
}

#[test]
fn consumed_own_stdout_redirect() {
    // The leaf's OWN `> file` captures fd 1 ⇒ Stdout consumed.
    let c = consumed_of("apt-get install -y nginx > /etc/marker\n", "apt-get");
    assert!(c.contains(&Channel::Stdout));
    assert!(!c.contains(&Channel::Stderr));
}

#[test]
fn consumed_own_stderr_redirect() {
    // `2> file` captures fd 2 ⇒ Stderr consumed (fd 1 untouched).
    let c = consumed_of("apt-get install -y nginx 2> /tmp/err\n", "apt-get");
    assert!(c.contains(&Channel::Stderr));
    assert!(!c.contains(&Channel::Stdout));
}

#[test]
fn consumed_devnull_is_quiet() {
    // The `/dev/null` discard sink is exempt (the precision scalpel) ⇒ still quiet.
    assert!(
        consumed_of("apt-get install -y nginx > /dev/null\n", "apt-get")
            .0
            .is_empty()
    );
}

#[test]
fn consumed_nonlast_pipeline_stage() {
    // A non-last pipeline stage's stdout is piped onward ⇒ Stdout consumed.
    assert!(
        consumed_of("apt-get install -y nginx | tee log\n", "apt-get").contains(&Channel::Stdout)
    );
}

#[test]
fn consumed_enclosing_group_redirect_marks_inner_leaf() {
    // 16G kill-shot / def-5 regression lock: the redirect is on the GROUP, not the
    // leaf. The inner install must still be marked Stdout — proving consumption is
    // computed in the single lowering traversal (the old plan-side dual-traversal
    // missed exactly this enclosing case; here it cannot, the fact is born with the
    // node via the arena-range mark).
    let c = consumed_of(
        "{ apt-get install -y nginx; } > /tmp/out\ncat /tmp/out\n",
        "apt-get",
    );
    assert!(
        c.contains(&Channel::Stdout),
        "enclosing-group redirect must mark the inner leaf"
    );
}

#[test]
fn consumed_enclosing_subshell_pipe_marks_inner_leaf() {
    // 16G kill-shot: the `( … )` is a non-last pipeline stage; its inner install is
    // the producer. The enclosing-pipe context must reach the inner leaf.
    let c = consumed_of("( apt-get install -y nginx ) | grep -q nginx\n", "apt-get");
    assert!(
        c.contains(&Channel::Stdout),
        "enclosing-subshell pipe must mark the inner leaf"
    );
}

#[test]
fn consumed_enclosing_subshell_devnull_stays_quiet() {
    // The scalpel survives the enclosing case too: `( … ) > /dev/null` discards ⇒
    // the inner leaf stays quiet (the range-mark must keep /dev/null exempt).
    assert!(
        consumed_of("( apt-get install -y nginx ) > /dev/null\n", "apt-get")
            .0
            .is_empty()
    );
}

// --- branch-status (round-19 `notes/195`; arch-1 note 214): an `if`/`elif` guard's
// command consumes `Channel::StatusRelaxable` — the leaf-exact render retired the
// `StatusRenderFloor` expressibility block, so a guard is an ordinary single-shot
// substitution site (a probe-sourced KNOWN rc reproduces its branch decision; ⊤ blocks).
// A `while`/`until` condition consumes `Channel::StatusIterated` (the per-pass sequence no
// single rc reproduces — an unconditional block, the honest successor). These pin the
// engine-side fact directly (the `plan` collapse is in `observable_matrix.rs`). ---

#[test]
fn consumed_if_guard_marks_relaxable() {
    // The command in an `if` condition is a guard (a different branch runs on its rc); its
    // status is branch-consumed ⇒ `StatusRelaxable` in the set (arch-1: a guard is now an
    // ordinary substitution site, not the retired render floor). The `then`-body command is
    // NOT in the condition region ⇒ stays quiet.
    let guard = consumed_of(
        "if apt-get install -y nginx; then systemctl start nginx; fi\n",
        "apt-get",
    );
    assert!(
        guard.contains(&Channel::StatusRelaxable),
        "an if-condition command's status is branch-consumed (StatusRelaxable, arch-1)"
    );
    // The then-body command (distinct command word so the helper finds IT, not the
    // guard) is not in the condition region ⇒ no branch-status.
    let body = consumed_of(
        "if apt-get install -y nginx; then systemctl start nginx; fi\n",
        "systemctl",
    );
    assert!(
        !body.contains(&Channel::StatusRelaxable),
        "the then-body command is not a guard ⇒ no branch-status"
    );
}

#[test]
fn consumed_negated_if_guard_marks_relaxable() {
    // The Dorc idiom `if ! command -v X; then …` — the `!`-negated pipeline sits inside the
    // `if` condition, so its command's status is branch-consumed `StatusRelaxable` (arch-1).
    // `echo` is target-state-pure but the consumption fact is locus-based, not effect-based,
    // so it is still marked.
    let c = consumed_of(
        "if ! echo probe; then apt-get install -y nginx; fi\n",
        "echo",
    );
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "a negated if-guard command's status is branch-consumed (StatusRelaxable, arch-1)"
    );
}

#[test]
fn consumed_errexit_marks_relaxable_status_c3() {
    // 19A C-3 / 205 §2: `set -e` reads every command's rc (non-zero ⇒ abort), so an
    // errexit-region command IS a status-consumer — marked the value-relaxable
    // `StatusRelaxable`. The committed engine deliberately left this un-marked ("errexit
    // stays vouched"); that is the C-3 hole task-E closes (a converged ⊤-rc mutator under
    // `set -e` must RUN, not elide). A known/probe-sourced rc still folds — the relaxation
    // is what Query-guard rcs ride.
    let c = consumed_of("set -e\napt-get install -y nginx\n", "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "errexit-consumed status is marked StatusRelaxable (C-3: not special-cased-as-vouched)"
    );
    assert!(
        !c.contains(&Channel::StatusIterated),
        "errexit is not a loop condition ⇒ never StatusIterated"
    );
}

#[test]
fn consumed_errexit_off_does_not_mark_status() {
    // The dual: WITHOUT `set -e` a plain command has no status consumer (no failure-edge
    // ⇒ no StatusRelaxable from the errexit pass). Pins that the C-3 mark is errexit-gated,
    // not blanket — a lone establish stays quiet (and so stays elidable when converged).
    let c = consumed_of("apt-get install -y nginx\n", "apt-get");
    assert!(
        !c.contains(&Channel::StatusRelaxable) && !c.contains(&Channel::StatusIterated),
        "no errexit ⇒ no status consumer on a lone command"
    );
}

#[test]
fn consumed_errexit_mark_respects_precise_edge_pruning() {
    // EXCLUSION-CHECK (the precise-edge contract, note 166 + 205 §2; re-homed for arch-1):
    // the C-3 errexit mark reuses the errexit pass's failure-edge knowledge, so it is pruned
    // EXACTLY where the failure-edge is. An `if`-guard command under `set -e` is
    // errexit-exempt (a condition region, no failure-edge). Under arch-1 the if-guard mark is
    // ALSO `StatusRelaxable` (no longer the retired render floor), so the channel alone can no
    // longer distinguish "errexit mark" from "if-guard mark" — the precise-edge property is
    // pinned by the FAILURE-EDGE: an errexit-exempt guard has NO failure→exit edge (whereas a
    // bare errexit-region command does). The guard still carries `StatusRelaxable` (from the
    // if-guard mark), but it must NOT carry a failure-edge (errexit-exempt).
    let src = "set -e\nif apt-get install -y nginx; then echo done; fi\n";
    let c = consumed_of(src, "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "the if-guard carries StatusRelaxable (arch-1: an ordinary guard substitution site)"
    );
    let cfg = cfg_of(src);
    let guard_n = command_nodes_with_literal(&cfg, src, "apt-get")[0];
    assert!(
        !has_exit_edge(&cfg, guard_n),
        "the errexit-exempt if-guard has NO failure→exit edge (precise-edge pruning — the \
         real property, now that the channel no longer distinguishes guard-vs-errexit)"
    );
}

#[test]
fn while_condition_is_iterated_and_errexit_exempt() {
    // task-L1 item-2(a)+(b) (re-homed for arch-1), the dash-fidelity heart of the while
    // loop. Under `set -e`:
    //   * the CONDITION command (`dpkg -s nginx`) is errexit-EXEMPT (a failing while
    //     condition does NOT abort — dash: `set -e; while false; do :; done; echo ok`
    //     prints ok), so it has NO failure→exit edge, and its status is consumed
    //     per-iteration (`StatusIterated`, arch-1 — the per-pass rc-sequence no single
    //     predicted rc reproduces; the honest successor to the retired render floor, and a
    //     loop condition is still never substituted — now keyed on iteration); whereas
    //   * the BODY command (`apt-get install`) is in the errexit region (a failing body
    //     command DOES abort), so it has a failure-edge AND is `StatusRelaxable`-consumed
    //     (C-3). The two split exactly as loop-condition vs errexit-region command do.
    let src = "set -e\nwhile dpkg -s nginx; do apt-get install -y nginx; done\n";
    let cond = consumed_of(src, "dpkg");
    assert!(
        cond.contains(&Channel::StatusIterated),
        "the while CONDITION is StatusIterated-consumed (per-iteration sequence): {cond:?}"
    );
    assert!(
        !cond.contains(&Channel::StatusRelaxable),
        "the errexit-EXEMPT while condition must NOT pick up the errexit StatusRelaxable mark"
    );
    let body = consumed_of(src, "apt-get");
    assert!(
        body.contains(&Channel::StatusRelaxable),
        "the errexit-region BODY command is StatusRelaxable-consumed (C-3): {body:?}"
    );

    // The failure-edge split, on the same source.
    let cfg = cfg_of(src);
    let cond_n = command_nodes_with_literal(&cfg, src, "dpkg")[0];
    let body_n = command_nodes_with_literal(&cfg, src, "apt-get")[0];
    assert!(
        !has_exit_edge(&cfg, cond_n),
        "the while condition has NO failure→exit edge (errexit-exempt)"
    );
    assert!(
        has_exit_edge(&cfg, body_n),
        "the errexit-region body command HAS a failure→exit edge (set -e aborts on it)"
    );
}

#[test]
fn consumed_dollar_question_marks_predecessor_c3() {
    // 19A C-3 / 205 §2: `$?` reads the PREVIOUS command's rc, so the consumer is the
    // predecessor. `apt-get install …` then `[ $? -ne 0 ] && echo recover`: the install
    // is marked StatusRelaxable (its rc is read), so a converged ⊤-rc mutator there RUNS.
    let c = consumed_of(
        "apt-get install -y nginx\n[ $? -ne 0 ] && echo recover\n",
        "apt-get",
    );
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "a command whose rc `$?` reads is marked StatusRelaxable (C-3 second consumer)"
    );
    // The `$?`-reader itself is NOT the consumer of its own rc — only the predecessor is.
    // Use a `$?`-reader that is NOT also a `&&`/`||` operand (which would mark it from a
    // different source): a plain `echo $?` statement after the install. `echo` reads
    // `$?` ⇒ its predecessor (the install) is marked, but `echo` itself is not.
    let reader = consumed_of("apt-get install -y nginx\necho $?\n", "echo");
    assert!(
        !reader.contains(&Channel::StatusRelaxable),
        "the `$?`-reader marks its predecessor, not itself"
    );
    // And the install IS the marked predecessor in that plain-statement shape too.
    let pred = consumed_of("apt-get install -y nginx\necho $?\n", "apt-get");
    assert!(
        pred.contains(&Channel::StatusRelaxable),
        "the predecessor of a plain `echo $?` reader is marked"
    );
}

#[test]
fn consumed_dollar_question_first_command_marks_nothing() {
    // Boundary: `$?` with no command predecessor (the first statement reads it) marks
    // nothing — the pred-walk reaches only Entry. No panic, no spurious mark.
    let c = consumed_of("[ $? -ne 0 ]\napt-get install -y nginx\n", "apt-get");
    assert!(
        !c.contains(&Channel::StatusRelaxable),
        "a `$?`-reader with no command predecessor marks nothing (walk hits Entry)"
    );
}

#[test]
fn consumed_dollar_question_in_assignment_marks_predecessor() {
    // The canonical idiom `cmd; rc=$?; …`: `$?` in the assignment RHS (not the argv)
    // still marks the predecessor. `rc=$?` is an assignment-only Simple; the install
    // before it is the consumed command.
    let c = consumed_of(
        "apt-get install -y nginx\nrc=$?\ntest \"$rc\" -eq 0\n",
        "apt-get",
    );
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "`$?` in an assignment RHS marks the predecessor command too (rc=$? idiom)"
    );
}

#[test]
fn consumed_andand_left_operand_marks_relaxable_status() {
    // `19D`: a `&&`/`||` left operand IS branch-consumed, so the engine marks it
    // `Channel::StatusRelaxable` (a KNOWN rc reproduces the operand's branch decision —
    // the same channel an `if`/`elif` guard now uses, arch-1). The phased caller collapses
    // it rc-conditionally (`prove_replaceable`): undeclared rc ⇒
    // block (the `useradd[9] || mkdir` under-execute floor), declared rc ⇒ relax
    // (`install && start`'s rc-0 post-condition stays replaceable). Pins the engine-side
    // fact; the rc-conditional collapse is in `plan`'s `observable_matrix.rs`.
    let c = consumed_of(
        "apt-get install -y nginx && systemctl enable nginx\n",
        "apt-get",
    );
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "a `&&`/`||` left operand's status is branch-consumed (marked StatusRelaxable, 19D)"
    );
    // It is the value-relaxable variant, NOT the loop-condition unconditional block.
    assert!(
        !c.contains(&Channel::StatusIterated),
        "a `&&`/`||` operand is StatusRelaxable, never the loop-condition StatusIterated block"
    );
}

#[test]
fn consumed_oror_left_operand_marks_relaxable_status() {
    // The `||` dual (the under-execute side): `useradd deploy || mkdir` — the left
    // operand's status gates the `mkdir` fallback, so it is marked `StatusRelaxable`. With
    // no declared rc the caller blocks ⇒ `useradd` runs ⇒ the `|| mkdir` fallback runs
    // (`19D` — the proven `kFAIL-perform` fix).
    let c = consumed_of("useradd deploy || mkdir /srv/app\n", "useradd");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "a `||` left operand's status is branch-consumed (marked StatusRelaxable, 19D)"
    );
}

// ===========================================================================
// door-3 (`20V` §4 / note 213): the `cmd || true` left operand is marked
// `Channel::StatusInvariant` (never blocks) instead of `StatusRelaxable` (blocks at
// ⊤). The classification pins live here (engine-side fact); the consumption-gate
// collapse + disposition flip live in `plan`'s `observable_matrix.rs`.
// ===========================================================================

#[test]
fn consumed_oror_true_left_operand_marks_invariant_not_relaxable() {
    // The door-3 mark: `cmd || true` — both `||` continuations rejoin with identical
    // observables, so the consumed left rc is dead-in-fact. The left operand is marked
    // `StatusInvariant` (never blocks, even at ⊤), NOT the blocking `StatusRelaxable`.
    let c = consumed_of("apt-get install -y nginx || true\n", "apt-get");
    assert!(
        c.contains(&Channel::StatusInvariant),
        "a `cmd || true` left operand is marked StatusInvariant (door-3): {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusRelaxable),
        "the `|| true` mark REPLACES StatusRelaxable (else the blocking mark would still block): {c:?}"
    );
}

#[test]
fn consumed_oror_false_keeps_relaxable_not_invariant() {
    // The `|| false` pole: `false` CHANGES the list rc (cmd rc=0 ⇒ short-circuit, list rc
    // 0; cmd rc≠0 ⇒ `false` runs ⇒ list rc 1), so the continuations do NOT rejoin
    // identically — the left rc is genuinely load-bearing. It keeps the blocking
    // `StatusRelaxable` mark; door-3 must NOT widen to it.
    let c = consumed_of("apt-get install -y nginx || false\n", "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "`|| false` keeps the blocking StatusRelaxable (the list rc differs by the left rc): {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusInvariant),
        "`|| false` is NOT door-3 (it is not the bare-true shape): {c:?}"
    );
}

#[test]
fn consumed_oror_colon_keeps_relaxable_deliberate_deferral() {
    // The `|| :` deferral: `:` is the null command — semantically IDENTICAL to `true`
    // (rc 0, no observable), so this shape is extensionally a door-3 candidate. We
    // deliberately DECLINE to widen to it this slice: every license-surface widening is a
    // disaster-class-bug locus (`20V` §4 d-2 widens candidate-by-candidate). `:` keeps the
    // blocking `StatusRelaxable`. This is a CANDIDATE-WIDENING marker — a future slice that
    // adds `:` (and audits it) flips this pin.
    let c = consumed_of("apt-get install -y nginx || :\n", "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "`|| :` stays StatusRelaxable-blocked (deliberate deferral, not yet a door-3 shape): {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusInvariant),
        "`|| :` is deliberately NOT widened into door-3 this slice (the colon is a candidate): {c:?}"
    );
}

#[test]
fn consumed_oror_true_with_redirect_keeps_relaxable() {
    // `|| true >/dev/null`: the redirect makes the rhs not the BARE `true` (`right_is_bare_true`
    // requires zero redirs). A redirect is its own effect node and could change observables,
    // so door-3's identity-of-continuations argument does not hold — keep the blocking mark.
    let c = consumed_of("apt-get install -y nginx || true >/dev/null\n", "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "`|| true >/dev/null` keeps StatusRelaxable (a redirect disqualifies the bare-true slice): {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusInvariant),
        "a redirected `true` is not the door-3 bare-true shape: {c:?}"
    );
}

#[test]
fn consumed_andand_true_keeps_relaxable_not_invariant() {
    // `cmd && true`: door-3 is `||`-only. Under `&&`, cmd's FAILURE short-circuits past
    // `true` and the LIST rc is cmd's non-zero rc (which fires errexit), so the
    // continuations differ — the left rc is load-bearing. Keep the blocking
    // `StatusRelaxable`; the `op == Or` guard in `lower_and_or` must exclude `&&`.
    let c = consumed_of("apt-get install -y nginx && true\n", "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "`&& true` keeps StatusRelaxable (failure short-circuits, list rc ≠ 0): {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusInvariant),
        "door-3 is `||`-only — `&& true` is not it: {c:?}"
    );
}

#[test]
fn consumed_oror_handler_group_keeps_relaxable() {
    // `cmd || { …; }`: the rhs is a `Group`, not a bare `true` Simple — its continuations
    // can differ observably (the handler prints, exits, etc.). Keep the blocking mark; only
    // the EXACT bare-`true` Simple qualifies.
    let c = consumed_of(
        "apt-get install -y nginx || { printf 'recovering\\n'; }\n",
        "apt-get",
    );
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "`|| {{ …; }}` keeps StatusRelaxable (a handler group is not the bare-true shape): {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusInvariant),
        "an `|| {{ …; }}` handler is not door-3: {c:?}"
    );
}

#[test]
fn consumed_oror_chain_true_marks_only_outer_left_invariant() {
    // d-3 asymmetry: `a || b || true` parses left-assoc as `(a || b) || true`. `a`'s rc is
    // read by the INNER `||` (it controls whether `b` runs — genuinely live), so `a` keeps
    // `StatusRelaxable` (blocks at ⊤). `b`'s consumer is the OUTER `|| true`, so `b` is
    // marked `StatusInvariant` (unlocks). The mark-union composition produces this: the inner
    // call marks `a` Relaxable; the outer call's `StatusInvariant` over the whole left range
    // ALSO lands on `a` (verified: `a` = {StatusRelaxable, StatusInvariant}), but is INERT
    // there — its Relaxable still blocks. We assert the CONTRACT (a blocks via Relaxable; b
    // unlocks via Invariant-without-Relaxable), NOT the incidental over-mark on `a` (a future
    // precision pass could stop over-marking `a` without changing the contract).
    let src = "apt-get install -y nginx || apt-get install -y curl || true\n";
    let cfg = cfg_of(src);
    let nodes = command_nodes_with_literal(&cfg, src, "apt-get");
    assert_eq!(
        nodes.len(),
        2,
        "two apt-get sites (a=nginx, b=curl): {nodes:?}"
    );
    let a = cfg.consumed_observables(nodes[0]).clone(); // source-order first: nginx
    let b = cfg.consumed_observables(nodes[1]).clone(); // second: curl
    assert!(
        a.contains(&Channel::StatusRelaxable),
        "a (inner-`||` left) keeps the blocking StatusRelaxable — its rc controls whether b runs: {a:?}"
    );
    assert!(
        b.contains(&Channel::StatusInvariant) && !b.contains(&Channel::StatusRelaxable),
        "b (outer `|| true` left) is StatusInvariant, not Relaxable — its rc is dead-in-fact: {b:?}"
    );
}

#[test]
fn consumed_oror_true_with_true_funcdef_keeps_relaxable() {
    // find-I (note 213 §5 hunt-4): a book-defined `true() { … }` shadows the builtin in
    // dash (a function wins over a regular builtin), so the `|| true` rhs is NOT the inert
    // builtin and door-3's rejoin-identical premise fails. The left operand must keep the
    // blocking `StatusRelaxable` — at HEAD-before-fix it was marked `StatusInvariant`
    // purely syntactically, minting a Replace whose stand-in `true` would RUN the
    // function's mutator body unconditionally (the verified wrong-elision).
    let src = "true() { systemctl restart sshd; }\napt-get install -y nginx || true\n";
    let c = consumed_of(src, "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable),
        "with a `true` funcdef in the book, `|| true` keeps the blocking StatusRelaxable: {c:?}"
    );
    assert!(
        !c.contains(&Channel::StatusInvariant),
        "a book-defined `true()` disqualifies door-3 (the rhs is not the inert builtin): {c:?}"
    );
}

#[test]
fn consumed_oror_true_with_later_true_funcdef_keeps_relaxable() {
    // The funcdef-check is FILE-WIDE, not positional ("defined before this point"): a
    // textually-later `true() { … }` can still be live when the site executes (re-entry
    // under a loop; and the minted stand-in inherits the site's program point). Over-
    // refusing a pathological book is the safe direction — no door-3 mark anywhere in it.
    let src = "apt-get install -y nginx || true\ntrue() { systemctl restart sshd; }\n";
    let c = consumed_of(src, "apt-get");
    assert!(
        c.contains(&Channel::StatusRelaxable) && !c.contains(&Channel::StatusInvariant),
        "a later `true()` funcdef still disqualifies door-3 (file-wide, conservative): {c:?}"
    );
}

#[test]
fn consumed_post_while_dollar_question_marks_body_not_only_condition() {
    // task-L2 item-6a (20O find-6a), dash-verified: post-loop `$?` after a `while` is the
    // BODY's last command rc (loop ran ≥1) or 0 (ran 0) — NEVER the condition's. So a
    // post-loop `$?`-reader must mark the BODY's last command `StatusRelaxable`-consumed.
    // The bug this pins: a `while`'s only exit edge is `cond_exit → merge`, so the bare
    // pred-walk stopped at the condition (`dpkg`) and the body (`apt-get`) was left
    // UNMARKED — under task-L2's member-license, a converged in-loop body establish whose
    // rc a post-loop `$?` reads would then be wrongly elidable. (dash: `n=0; while [ $n
    // -lt 2 ]; do n=$((n+1)); (exit 7); done; echo $?` ⇒ 7, the body-last, not the
    // condition's `[ ]` false rc.)
    let src = "while dpkg -s nginx; do apt-get install -y nginx; done\nrc=$?\n";
    let body = consumed_of(src, "apt-get");
    assert!(
        body.contains(&Channel::StatusRelaxable),
        "the while BODY's last command is marked (post-loop $? = body-last rc): {body:?}"
    );
    // The condition is ALSO marked (the walk still reaches it), but that mark is inert: a
    // while condition is `StatusIterated`-blocked unconditionally regardless (arch-1).
    let cond = consumed_of(src, "dpkg");
    assert!(
        cond.contains(&Channel::StatusIterated),
        "the while condition stays StatusIterated-blocked (its StatusRelaxable over-mark is inert): {cond:?}"
    );
}

#[test]
fn consumed_post_for_dollar_question_marks_body() {
    // The `for` half of item-6a (verified-correct, left unchanged): a `for` loop's exit
    // edge is `head → merge`, and `head`'s back-edge pred IS the body-exit, so a post-loop
    // `$?`-walk reaches the body's last command already. (dash: `for x in a b; do (exit 5);
    // done; echo $?` ⇒ 5, the body-last.) Pinned so a future change to `lower_for` that
    // drops the back-edge regresses loudly here.
    let src = "for f in a b; do apt-get install -y nginx; done\nrc=$?\n";
    let body = consumed_of(src, "apt-get");
    assert!(
        body.contains(&Channel::StatusRelaxable),
        "the for BODY's last command is marked (post-loop $? = body-last rc): {body:?}"
    );
}

// ===========================================================================
// arch-2 (brk-2): budget-bounded function inlining — the CFG splice + eligibility.
// `cfg_of`/`build` only; structural pins (no `solve`), plus the loud refusal
// diagnostics. The call resolution + budgets are `i-1`; the back-map is `i-6`.
// ===========================================================================

/// Did `build(src)` emit a `cfg-inline-refused` diagnostic mentioning `needle`?
fn inline_refused_for(src: &str, needle: &str) -> bool {
    let parsed = parse(src);
    build(&parsed.value)
        .diags
        .iter()
        .any(|d| d.code.0 == "cfg-inline-refused" && d.message.contains(needle))
}

/// The body-leaf list (CFG node ids) of the FIRST inlined call whose call-word is `lit`.
fn call_body_sites_of(cfg: &Cfg, src: &str, lit: &str) -> Option<Vec<CfgNodeId>> {
    let call = first_command_with_literal(cfg, src, lit)?;
    cfg.call_body_sites(call).map(<[CfgNodeId]>::to_vec)
}

#[test]
fn earlier_funcdef_call_splices_body_and_is_an_inlined_call() {
    // The base case (`i-1`): a call to a same-file-earlier funcdef splices the body. The CALL
    // node carries a body-site list (the install), and the spliced body command is reachable
    // from entry (un-detached — the find-7 fix) and is `spliced_internal` (not its own leaf).
    let src = "p() { apt-get install -y nginx; }\np\n";
    let cfg = cfg_of(src);
    let sites = require(call_body_sites_of(&cfg, src, "p"), "`p` is an inlined call");
    assert_eq!(sites.len(), 1, "one body leaf (the install)");
    assert!(
        cfg.is_spliced_internal(sites[0]),
        "the spliced body command is spliced_internal (not an independent leaf, i-3)"
    );
    assert!(
        reaches(&cfg, cfg.entry(), sites[0]),
        "the spliced body command IS reachable from entry (un-detached)"
    );
}

#[test]
fn forward_call_before_definition_is_not_inlined() {
    // `i-1`: a call BEFORE any definition of the name stays an ordinary unmodeled command (it
    // might be a PATH binary at that program point) — NOT inlined, and NO diagnostic (a
    // forward call is legitimate, not a refusal). The later definition is detached/non-leaf.
    let src = "p\np() { apt-get install -y nginx; }\n";
    let cfg = cfg_of(src);
    assert!(
        call_body_sites_of(&cfg, src, "p").is_none(),
        "a call before the definition is not an inlined call"
    );
    let parsed = parse(src);
    assert!(
        !build(&parsed.value)
            .diags
            .iter()
            .any(|d| d.code.0 == "cfg-inline-refused"),
        "a forward call is silent (no refusal diagnostic — it might be a PATH binary)"
    );
}

#[test]
fn redefined_function_call_refuses_with_diagnostic() {
    // `i-1`: a name defined MORE THAN ONCE ⇒ every call ⊤-rejects (redefinition tracking is
    // out of slice). Loud diagnostic; the call is not inlined.
    let src = "p() { apt-get install -y nginx; }\np() { apt-get install -y curl; }\np\n";
    let cfg = cfg_of(src);
    assert!(
        call_body_sites_of(&cfg, src, "p").is_none(),
        "a call to a redefined function is not inlined"
    );
    assert!(
        inline_refused_for(src, "defined more than once"),
        "the redefinition refusal is loud"
    );
}

#[test]
fn direct_recursion_refuses_with_cycle_diagnostic() {
    // `i-1`: a direct self-call inside the body ⇒ ⊤-reject naming the cycle. The OUTER call
    // still inlines, but the inner recursive call is NOT inlined (Opaque) — so the body-site
    // list does NOT recurse infinitely, and a diagnostic names the recursion.
    let src = "p() { apt-get install -y nginx; p; }\np\n";
    let cfg = cfg_of(src);
    assert!(
        call_body_sites_of(&cfg, src, "p").is_some(),
        "the outer call still inlines (the recursion guard stops the inner, not the outer)"
    );
    assert!(
        inline_refused_for(src, "recursive call to `p`"),
        "the recursion refusal names the cycle"
    );
}

#[test]
fn mutual_recursion_terminates_no_infinite_splice() {
    // `i-1` recursion safety — TWO guards compose: the textual "definition strictly before the
    // call" rule (a body call inside a definition can only resolve to a function defined before
    // THAT definition's position, so the resolvable-call relation is a strict partial order
    // with NO cycle), AND the active-inline-stack guard (the belt-and-suspenders for direct
    // recursion). A mutual pair `a(){ b; } b(){ a; }` calls `b` (which inlines, its body calls
    // `a` — `a` is defined before, so it inlines, its body calls `b` — but `b` is defined AFTER
    // `a`'s definition position, so that body-call is a FORWARD call, refused-silently — the
    // chain terminates). The key property: build TERMINATES (no infinite splice) and produces a
    // consistent CFG. (A true body-call cycle is structurally unreachable; the stack guard is
    // exercised by `direct_recursion_refuses_with_cycle_diagnostic`.)
    let src = "a() { b; }\nb() { a; }\nb\n";
    let cfg = cfg_of(src); // must not hang / overflow
    assert!(
        consistent(&cfg),
        "the mutual-recursion splice stays consistent"
    );
    assert!(
        reaches(&cfg, cfg.entry(), cfg.exit()),
        "build terminates, exit reachable"
    );
}

#[test]
fn depth_budget_refuses_a_fourth_level() {
    // `i-1`: inline depth <= 2. A chain `a`->`b`->`c` inlines `b` (depth 1) and `c` (depth 2);
    // a fourth level `a`->`b`->`c`->`d` refuses `d` at depth 2 (the stack is already 2 deep).
    let src = "d() { apt-get install -y nginx; }\nc() { d; }\nb() { c; }\na() { b; }\na\n";
    assert!(
        inline_refused_for(src, "inline-depth budget"),
        "the 4th-level call refuses at the depth budget"
    );
}

#[test]
fn body_using_shift_refuses() {
    // `i-1`: a body using `shift` (positional-array mutation, out of slice) ⇒ refuse naming it.
    let src = "p() { shift; apt-get install -y nginx; }\np x\n";
    assert!(call_body_sites_of(&cfg_of(src), src, "p").is_none());
    assert!(
        inline_refused_for(src, "shift"),
        "the shift refusal names it"
    );
}

#[test]
fn body_using_local_refuses() {
    let src = "p() { local x; apt-get install -y nginx; }\np\n";
    assert!(
        inline_refused_for(src, "local"),
        "the local refusal names it"
    );
}

#[test]
fn body_using_dollar_at_refuses() {
    // `i-1`: the positional array is out of slice — refuse naming it. (The value plane cannot
    // bind the whole operand list this slice.)
    let src = "p() { apt-get install -y \"$@\"; }\np nginx\n";
    assert!(
        inline_refused_for(src, "$@"),
        "the positional-array refusal names the construct"
    );
}

#[test]
fn body_write_redirect_to_real_file_refuses_but_devnull_is_exempt() {
    // tc-M2: a body write-redirect to a REAL file ⇒ refuse (an unmodeled effect inlining would
    // expose). A `>/dev/null` body write stays EXEMPT (the wrapper-pun population redirects
    // there) ⇒ that body DOES inline.
    let fences = "p() { apt-get install -y nginx >> /etc/motd; }\np\n";
    assert!(call_body_sites_of(&cfg_of(fences), fences, "p").is_none());
    assert!(
        inline_refused_for(fences, "write-redirect"),
        "a real-file body write-redirect refuses (tc-M2)"
    );

    let exempt = "p() { apt-get install -y nginx >/dev/null 2>&1; }\np\n";
    assert!(
        call_body_sites_of(&cfg_of(exempt), exempt, "p").is_some(),
        "a `>/dev/null` body write-redirect is EXEMPT => the call still inlines"
    );
    assert!(
        !inline_refused_for(exempt, "write-redirect"),
        "no write-redirect refusal for the devnull-exempt body"
    );
}

#[test]
fn at_budget_body_inlines_over_budget_refuses() {
    // `i-1` budget boundary: a tiny body inlines (well under the 64-node budget); a large body
    // (100 `:` commands, well over) refuses with a budget diagnostic. The estimate is the
    // AST-subtree node count (a conservative proxy); the test finds the boundary by extremes so
    // it does not hard-code the per-node AST cost.
    let body_of = |n: usize| {
        let mut b = String::from("p() { ");
        for _ in 0..n {
            b.push_str(": ; ");
        }
        b.push_str("apt-get install -y nginx; }\np\n");
        b
    };
    let small = body_of(1);
    assert!(call_body_sites_of(&cfg_of(&small), &small, "p").is_some());
    assert!(!inline_refused_for(&small, "budget"));
    let big = body_of(100);
    assert!(
        call_body_sites_of(&cfg_of(&big), &big, "p").is_none(),
        "an over-budget body is not inlined"
    );
    assert!(
        inline_refused_for(&big, "per-call inline-node budget"),
        "the over-budget refusal names the per-call budget"
    );
}

#[test]
fn two_calls_to_one_function_produce_distinct_leaf_nodes_sharing_one_ast() {
    // `i-6` back-map non-injectivity: two calls to one function splice DISTINCT body CFG nodes
    // (their own LeafIds downstream) that map to the SAME body AstId. Verify the two calls'
    // body sites are different CFG nodes but their provenance AstId is shared.
    let src = "p() { apt-get install -y nginx; }\np\np\n";
    let cfg = cfg_of(src);
    let calls = command_nodes_with_literal(&cfg, src, "p");
    assert_eq!(calls.len(), 2, "two call sites");
    let s0 = require(cfg.call_body_sites(calls[0]), "call 0 inlined").to_vec();
    let s1 = require(cfg.call_body_sites(calls[1]), "call 1 inlined").to_vec();
    assert_eq!(s0.len(), 1);
    assert_eq!(s1.len(), 1);
    assert_ne!(
        s0[0], s1[0],
        "the two calls' body sites are DISTINCT CFG nodes"
    );
    assert_eq!(
        cfg.node(s0[0]).ast,
        cfg.node(s1[0]).ast,
        "but they map to the SAME shared body AstId (the back-map is non-injective, i-6)"
    );
}

/// The REACHABLE (from entry) `Command` nodes whose command word is `lit` — filters out the
/// detached funcdef-definition copies (which are non-leaf islands, unreachable).
fn reachable_command_nodes_with_literal(cfg: &Cfg, src: &str, lit: &str) -> Vec<CfgNodeId> {
    command_nodes_with_literal(cfg, src, lit)
        .into_iter()
        .filter(|&n| reaches(cfg, cfg.entry(), n))
        .collect()
}

#[test]
fn depth_2_positional_argument_refuses_inner_call_loudly() {
    // arch-2 wave-2 correction: depth-2 positional threading does NOT work — note 216 §1.2
    // claimed `a() { b "$1"; }` threads the outer `$1` into `b`'s body, but the overlay does
    // not bind it (the inner `$1` resolves ⊤). Instead of a SILENT safe MustRun, the inner
    // call `b "$1"` is REFUSED inlining with a catalogued Note (`dq-depth-2-positional-
    // unthreaded`), so the limitation is loud. The book still RUNS verbatim (the reachable
    // inner `b` stays Opaque/un-inlined — safe behavior unchanged).
    let src = "b() { apt-get install -y \"$1\"; }\na() { b \"$1\"; }\na nginx\n";
    let parsed = parse(src);
    let built = build(&parsed.value);
    assert!(
        built
            .diags
            .iter()
            .any(|d| d.code.0 == "dq-depth-2-positional-unthreaded"),
        "the un-threaded depth-2 positional inner call refuses LOUDLY (catalogued Note): {:?}",
        built.diags.iter().map(|d| d.code.0).collect::<Vec<_>>()
    );
    let cfg = &built.value;
    // The OUTER call `a` still inlines (its argument `nginx` is a literal, threads fine).
    assert!(
        call_body_sites_of(cfg, src, "a").is_some(),
        "the outer call `a` still inlines (its literal argument threads)"
    );
    // The REACHABLE inner `b` call (inside `a`'s splice) is refused — it is NOT an inlined call,
    // so it has no body-site list (it runs verbatim as an ordinary command). (The detached
    // definition copy of `a`'s body is a non-leaf island and is not the run-set's concern.)
    let reachable_b = reachable_command_nodes_with_literal(cfg, src, "b");
    assert_eq!(
        reachable_b.len(),
        1,
        "one reachable `b` call (inside a's splice)"
    );
    assert!(
        cfg.call_body_sites(reachable_b[0]).is_none(),
        "the reachable inner `b` call is refused (runs verbatim, Opaque)"
    );
}

#[test]
fn depth_2_literal_argument_still_inlines() {
    // The refusal is NARROW: a nested call with a LITERAL argument still inlines (only a
    // positional-referencing nested call is the un-threaded shape). `a() { b nginx; }` threads
    // `nginx` to `b`'s `$1` fine (one level), so the reachable `b` inlines and the install
    // resolves.
    let src = "b() { apt-get install -y \"$1\"; }\na() { b nginx; }\na\n";
    let parsed = parse(src);
    let built = build(&parsed.value);
    assert!(
        !built
            .diags
            .iter()
            .any(|d| d.code.0 == "dq-depth-2-positional-unthreaded"),
        "a literal-argument nested call is NOT refused (it threads one level fine)"
    );
    let reachable_b = reachable_command_nodes_with_literal(&built.value, src, "b");
    assert_eq!(reachable_b.len(), 1, "one reachable `b` call");
    assert!(
        built.value.call_body_sites(reachable_b[0]).is_some(),
        "the reachable inner call `b nginx` still inlines"
    );
}

#[test]
fn depth_2_inner_install_ships_exactly_one_site_record() {
    // `i-6` depth-2 splice double-count regression (crosscheck wave-2): a depth-2 transitive
    // splice (`b` called inside `a`'s body) must ship the inner install exactly ONCE under the
    // OUTER call's leaf, not twice (the `site 0.0` AND `site 0.1` bug). The inner install sits
    // both in the inner call's flattened site list AND directly in the outer arena range; the
    // direct scan must skip the already-flattened leaf.
    let src = "b() { apt-get install -y nginx >/dev/null 2>&1; }\na() { b; }\na\n";
    let cfg = cfg_of(src);
    let outer = require(call_body_sites_of(&cfg, src, "a"), "`a` is an inlined call");
    assert_eq!(
        outer.len(),
        1,
        "exactly one body-site record for the depth-2 install (no double-count): got {outer:?}"
    );
    // The single recorded site is the deepest install leaf, not the inner `b` call node.
    assert_eq!(
        command_word_literal(&parse(src).value, cfg.node(outer[0]).ast).as_deref(),
        Some("apt-get"),
        "the recorded site is the install leaf, not the `b` call"
    );
    assert!(
        cfg.is_spliced_internal(outer[0]),
        "the install is spliced_internal (the CALL is the render unit)"
    );
}

#[test]
fn splice_keeps_the_cfg_consistent_and_terminating() {
    // inv-no-throw + Graph consistency under the splice: an inlined book builds a consistent
    // graph whose exit is reachable (the spliced region is properly sequenced into the flow).
    // The post-call install is a real (non-spliced) leaf — the value below an eliding call
    // unlocks normally.
    let src = "p() { apt-get install -y nginx; }\np\napt-get install -y curl\n";
    let cfg = cfg_of(src);
    assert!(consistent(&cfg), "succ/pred consistent under the splice");
    assert!(reaches(&cfg, cfg.entry(), cfg.exit()), "exit reachable");
    let installs = command_nodes_with_literal(&cfg, src, "apt-get");
    assert!(
        installs.iter().any(|&n| !cfg.is_spliced_internal(n)),
        "the post-call install is a real (non-spliced) leaf"
    );
}
