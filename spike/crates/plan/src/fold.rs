//! The apply-phase **fold**: abstract-interpretation of the apply-CFG over the
//! probed observables (`19A §5` / `19B` build-1). Given a leaf's *concrete* observed
//! exit status, constant-fold the known shell control constructs — `&&`, `||`, `if`,
//! `!`, `case` — and omit branches that *provably cannot run*.
//!
//! This is the **apply collapse** of `inv-superposition`: the engine (`analysis`)
//! emits phase-/orientation-agnostic facts; this pass — driven by the apply caller's
//! *injected* observations — collapses them into per-leaf liveness. It must NOT live
//! in the engine, and the probe-phase collapse is a *separate* thing (a probe never
//! folds an apply branch). It is intentionally *concrete* partial-evaluation where a
//! status is known, ⊤ where unknown (SPA ch.12 made concrete, not a fixpoint over
//! abstractions).
//!
//! Soundness (`inv-kfail` / `kFAIL-perform`): a branch is folded dead ONLY from a
//! KNOWN controlling status. An unknown / ⊤ status never folds — the branch stays
//! live ⇒ its leaves run. We hold the rc *value* opaquely (`9`) and replay the
//! shell's own `&&`/`||`/`!` semantics over it; we never interpret what `9` *means*
//! (`inv-referent-agnostic`-adjacent — the author already encoded the meaning by
//! choosing the operator).
//!
//! The fold walks the **AST**, not the CFG: which leaf controls which branch is a
//! syntactic fact (`AndOr{op,left,right}`, `If{cond,then,…}`, `Pipeline{negated}`)
//! that the flattened CFG blurs. `node_rc` exposes each node's abstract status so
//! the renderer can compute a fully-folded line's value-preserving stand-in.

use std::collections::BTreeMap;

use dorc_core::{AstId, Observable, Predicted, Rc};
use dorc_syntax::ast::{AndOrOp, Ast, NodeKind};

/// A node's abstract exit status: a concrete value, or ⊤ (unknown). The fold's
/// lattice is the flat one (`Flat<i32>`-shaped) — two distinct knowns never need to
/// join here (a node has one status), so ⊤ is the only non-concrete element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractRc {
    /// A statically-known exit status (from a probed observable, replayed through the
    /// shell's own operator semantics).
    Known(Rc),
    /// Unknown / not-probed / unmodeled ⇒ ⊤. No fold through this (`inv-kfail`).
    Top,
}

impl AbstractRc {
    fn known(n: i32) -> Self {
        AbstractRc::Known(Rc(n))
    }

    /// Did this status *succeed* (rc 0)? `None` when ⊤ (can't tell ⇒ no fold).
    fn is_success(self) -> Option<bool> {
        match self {
            AbstractRc::Known(rc) => Some(rc.0 == 0),
            AbstractRc::Top => None,
        }
    }
}

/// The result of folding a script: per-leaf liveness + per-node abstract status.
#[derive(Debug, Clone, Default)]
pub struct FoldResult {
    /// Leaves the fold proved **dead** (unreachable), each mapped to the `AstId` of the
    /// controlling leaf whose known status short-circuited past it (provenance, and
    /// the render's neutralised-controller gate). A `BTreeMap` for `inv-determinism`.
    dead: BTreeMap<AstId, AstId>,
    /// Each AST node's abstract exit status (so the renderer can reproduce a
    /// fully-folded *line*'s value, and tests can assert the fold's reasoning).
    node_rc: BTreeMap<AstId, AbstractRc>,
}

impl FoldResult {
    /// Was this leaf proved dead by the fold? (Its controlling leaf, if so.)
    #[must_use]
    pub fn dead_controller(&self, leaf: AstId) -> Option<AstId> {
        self.dead.get(&leaf).copied()
    }

    /// Is this leaf in a provably-dead branch?
    #[must_use]
    pub fn is_dead(&self, leaf: AstId) -> bool {
        self.dead.contains_key(&leaf)
    }

    /// The abstract exit status the fold computed for an AST node (⊤ if it never
    /// reasoned about it — e.g. a node it didn't visit).
    #[must_use]
    pub fn rc_of(&self, node: AstId) -> AbstractRc {
        self.node_rc.get(&node).copied().unwrap_or(AbstractRc::Top)
    }
}

/// Run the fold over `ast`, using `observe` to get each *leaf*'s predicted
/// [`Observable`]. The fold reads only the **Status** channel (`observe(leaf).status`):
/// `Predicted::Value(rc)` is the fold input, `Predicted::Top` (or no observation) is ⊤ ⇒
/// no fold through it. (The Status channel is the only one the rc-fold consumes; Effect
/// gates the license elsewhere, Stdout/Stderr are liveness-only.)
///
/// Deterministic + total (`inv-determinism` / `inv-no-throw`): a pure recursion over
/// the arena, ordered maps, never panics (the same `MAX_DEPTH`-style safety is the
/// AST's own — the tree is already depth-bounded by the parser/cfg).
#[must_use]
pub(crate) fn fold(ast: &Ast, observe: impl Fn(AstId) -> Option<Observable>) -> FoldResult {
    let mut f = Folder {
        ast,
        observe: &observe,
        out: FoldResult::default(),
    };
    let _ = f.eval(ast.root(), true);
    f.out
}

struct Folder<'a> {
    ast: &'a Ast,
    observe: &'a dyn Fn(AstId) -> Option<Observable>,
    out: FoldResult,
}

impl Folder<'_> {
    /// Evaluate node `id`'s abstract exit status. `live` is whether this node is on a
    /// reachable path; when `false`, every leaf beneath it is recorded dead (with the
    /// controller threaded down). Records `node_rc[id]` and returns the status.
    fn eval(&mut self, id: AstId, live: bool) -> AbstractRc {
        let rc = self.eval_inner(id, live);
        self.out.node_rc.insert(id, rc);
        rc
    }

    fn eval_inner(&mut self, id: AstId, live: bool) -> AbstractRc {
        match &self.ast.node(id).kind {
            NodeKind::Script { items } | NodeKind::List { items } => {
                // A sequence's status is its LAST item's; every item is on the same
                // reachability as the sequence (no short-circuit between `;`-items).
                let items = items.clone();
                let mut last = AbstractRc::known(0); // empty list ⇒ rc 0
                for item in &items {
                    last = self.eval(*item, live);
                }
                last
            }
            NodeKind::Simple { .. } => {
                if !live {
                    // A dead leaf: it has no controller of its own; the caller that
                    // killed the branch records the controller (see `&&`/`||`/`if`).
                    // Reaching here with `live=false` and no recorded controller means
                    // an ancestor killed it — mark it dead with a self-controller as a
                    // defensive fallback (the real controller is set at the kill site).
                    self.out.dead.entry(id).or_insert(id);
                    return AbstractRc::Top;
                }
                // A live leaf's status is its predicted Status channel, or ⊤.
                match (self.observe)(id) {
                    Some(Observable {
                        status: Predicted::Value(rc),
                        ..
                    }) => AbstractRc::Known(rc),
                    _ => AbstractRc::Top,
                }
            }
            NodeKind::Pipeline { stages, negated } => {
                // The pipeline's status is its LAST stage's (POSIX, no `pipefail`
                // modeled). `!` negates it: 0 ⇒ 1, non-0 ⇒ 0 (a known status only).
                let (stages, negated) = (stages.clone(), *negated);
                let mut last = AbstractRc::known(0);
                for s in &stages {
                    last = self.eval(*s, live);
                }
                if negated { negate(last) } else { last }
            }
            NodeKind::AndOr { op, left, right } => {
                let (op, left, right) = (*op, *left, *right);
                self.eval_and_or(op, left, right, live)
            }
            NodeKind::If {
                cond,
                then_body,
                elifs,
                else_body,
            } => {
                let cond = *cond;
                let then_body = *then_body;
                let elifs: Vec<_> = elifs.iter().map(|e| (e.cond, e.body)).collect();
                let else_body = *else_body;
                self.eval_if(cond, then_body, &elifs, else_body, live)
            }
            NodeKind::Case { word, arms } => {
                // Scrutinee is a STRING value, not an rc — the rc-fold can't resolve
                // which arm matches (string-value abstract-interp is out of scope,
                // `19B` build-1 / charter). So every arm stays live; the construct's
                // status is ⊤. (`word` may carry a `$(…)` effect, but its rc is not
                // the fold's concern.) Recorded as a strain in `19C`.
                let word = *word;
                let arms: Vec<AstId> = arms.iter().map(|a| a.body).collect();
                let _ = self.eval(word, live);
                for body in &arms {
                    self.eval(*body, live);
                }
                AbstractRc::Top
            }
            NodeKind::Subshell { body, .. } | NodeKind::Group { body, .. } => {
                // A `( )`/`{ }`'s status is its body's; redirs don't change the rc the
                // fold reads (a failing redir is the errexit pass's concern, not here).
                let body = *body;
                self.eval(body, live)
            }
            NodeKind::FuncDef { .. } => {
                // Defining a function has no runtime status effect (rc 0) and its body
                // is detached (not on this path) — don't descend (no call modeled).
                AbstractRc::known(0)
            }
            NodeKind::Unsupported { .. } => {
                // ⊤ by construction (`inv-top-reject`): an unmodeled construct's status
                // is unknown ⇒ no fold. Its (dead-or-live) leaves are handled by the
                // disposition layer's existing ⊤-containment, not here.
                AbstractRc::Top
            }
            // Leaf word/assign/redir never head a statement (parser nests them); a
            // bare assignment is rc 0. Treat as rc 0 (no command status).
            NodeKind::Word { .. } | NodeKind::Assign { .. } | NodeKind::Redir { .. } => {
                AbstractRc::known(0)
            }
        }
    }

    /// `left && right` / `left || right` — the short-circuit fold. `left` always runs
    /// (same reachability as the construct); `right` runs conditionally on `left`'s
    /// KNOWN status. The construct's status is the operand that actually determines it
    /// (the short-circuit value, or `right`'s when `right` runs).
    fn eval_and_or(&mut self, op: AndOrOp, left: AstId, right: AstId, live: bool) -> AbstractRc {
        let left_rc = self.eval(left, live);
        // Does `right` run? Known iff `left`'s success is known.
        let right_runs: Option<bool> = match (op, left_rc.is_success()) {
            // `&&`: right runs iff left succeeded.
            (AndOrOp::And, Some(s)) => Some(s),
            // `||`: right runs iff left FAILED.
            (AndOrOp::Or, Some(s)) => Some(!s),
            // left status unknown ⇒ can't tell ⇒ right is live (no fold).
            (_, None) => None,
        };
        match right_runs {
            Some(true) => {
                // `right` runs (same reachability as the construct); the construct's
                // status is `right`'s.
                self.eval(right, live)
            }
            Some(false) => {
                // `right` is provably DEAD — short-circuited past. Record every leaf
                // beneath it dead, controlled by `left`. The construct's status is
                // `left`'s short-circuit value (the operand that decided it).
                self.kill(right, left);
                left_rc
            }
            None => {
                // Unknown: `right` is live (run). The construct's status is ⊤ unless
                // both operands agree on a known value — conservatively ⊤.
                let _ = self.eval(right, live);
                AbstractRc::Top
            }
        }
    }

    /// `if C; then T; [elif Ci; then Ti;]* [else E;] fi`. Fold the first branch whose
    /// condition is KNOWN-true; everything past a known-true condition, and any branch
    /// reached only past a known-false-then-known condition chain, is resolved. A ⊤
    /// condition stops the fold: from there every remaining branch is live.
    fn eval_if(
        &mut self,
        cond: AstId,
        then_body: AstId,
        elifs: &[(AstId, AstId)],
        else_body: Option<AstId>,
        live: bool,
    ) -> AbstractRc {
        let cond_rc = self.eval(cond, live);
        match cond_rc.is_success() {
            Some(true) => {
                // Condition KNOWN true ⇒ `then_body` runs; every other branch's body
                // is dead, controlled by this condition's last leaf.
                let then_rc = self.eval(then_body, live);
                self.kill_if_rest(elifs, else_body, cond);
                then_rc
            }
            Some(false) => {
                // Condition KNOWN false ⇒ `then_body` is DEAD; recurse into the rest
                // with the same reachability (the rest is still live).
                self.kill(then_body, cond);
                match elifs.split_first() {
                    Some(((ec, eb), rest)) => self.eval_if(*ec, *eb, rest, else_body, live),
                    None => match else_body {
                        // No more conditions ⇒ `else` runs (status = else's), or (no
                        // else) the `if` is rc 0 (false condition, nothing ran).
                        Some(eb) => self.eval(eb, live),
                        None => AbstractRc::known(0),
                    },
                }
            }
            None => {
                // ⊤ condition ⇒ stop folding: `then` and the entire rest are live.
                let _ = self.eval(then_body, live);
                for (ec, eb) in elifs {
                    let _ = self.eval(*ec, live);
                    let _ = self.eval(*eb, live);
                }
                if let Some(eb) = else_body {
                    let _ = self.eval(eb, live);
                }
                AbstractRc::Top
            }
        }
    }

    /// Mark every elif-condition + body and the else-body dead (the branches reached
    /// only when an earlier condition was true), controlled by `controller`.
    fn kill_if_rest(
        &mut self,
        elifs: &[(AstId, AstId)],
        else_body: Option<AstId>,
        controller: AstId,
    ) {
        for (ec, eb) in elifs {
            self.kill(*ec, controller);
            self.kill(*eb, controller);
        }
        if let Some(eb) = else_body {
            self.kill(eb, controller);
        }
    }

    /// Record every leaf in `id`'s subtree as dead, controlled by `controller`. Walks
    /// with `live=false` so nested leaves are collected; `node_rc` is still recorded
    /// (⊤ for dead subtrees) for render/debug symmetry.
    fn kill(&mut self, id: AstId, controller: AstId) {
        self.kill_rec(id, controller);
    }

    fn kill_rec(&mut self, id: AstId, controller: AstId) {
        match &self.ast.node(id).kind {
            NodeKind::Simple { .. } => {
                self.out.dead.insert(id, controller);
                self.out.node_rc.insert(id, AbstractRc::Top);
            }
            NodeKind::Script { items } | NodeKind::List { items } => {
                for item in items.clone() {
                    self.kill_rec(item, controller);
                }
            }
            NodeKind::Pipeline { stages, .. } => {
                for s in stages.clone() {
                    self.kill_rec(s, controller);
                }
            }
            NodeKind::AndOr { left, right, .. } => {
                let (left, right) = (*left, *right);
                self.kill_rec(left, controller);
                self.kill_rec(right, controller);
            }
            NodeKind::If {
                cond,
                then_body,
                elifs,
                else_body,
            } => {
                let cond = *cond;
                let then_body = *then_body;
                let elifs: Vec<_> = elifs.iter().map(|e| (e.cond, e.body)).collect();
                let else_body = *else_body;
                self.kill_rec(cond, controller);
                self.kill_rec(then_body, controller);
                for (ec, eb) in &elifs {
                    self.kill_rec(*ec, controller);
                    self.kill_rec(*eb, controller);
                }
                if let Some(eb) = else_body {
                    self.kill_rec(eb, controller);
                }
            }
            NodeKind::Case { word, arms } => {
                let word = *word;
                let bodies: Vec<AstId> = arms.iter().map(|a| a.body).collect();
                self.kill_rec(word, controller);
                for b in bodies {
                    self.kill_rec(b, controller);
                }
            }
            NodeKind::Subshell { body, .. } | NodeKind::Group { body, .. } => {
                self.kill_rec(*body, controller);
            }
            // funcdef body is detached, words/assigns/redirs/unsupported carry no leaf.
            _ => {}
        }
    }
}

/// Negate a known status the way `!` does: success(0) ⇒ failure(1), failure ⇒ 0.
/// ⊤ stays ⊤.
fn negate(rc: AbstractRc) -> AbstractRc {
    match rc.is_success() {
        Some(true) => AbstractRc::known(1),
        Some(false) => AbstractRc::known(0),
        None => AbstractRc::Top,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::Verdict;
    use dorc_syntax::ast::{NodeKind, WordPart};

    /// The `AstId` of the `Simple` command whose first word is `name` (the leaf the
    /// fold reasons about). Tests inject observations keyed by these.
    fn leaf(ast: &Ast, name: &str) -> AstId {
        for (id, node) in ast.iter() {
            if let NodeKind::Simple { words, .. } = &node.kind
                && let Some(&w) = words.first()
                && let NodeKind::Word { parts } = &ast.node(w).kind
                && matches!(parts.as_slice(), [WordPart::Literal(s)] if s == name)
            {
                return id;
            }
        }
        panic!("no leaf command `{name}` in the parsed AST");
    }

    /// A known concrete observation (`rc=n`), the conforming converged shape.
    fn obs(rc: i32) -> Observable {
        Observable {
            effect: Verdict::Converged,
            status: Predicted::Value(Rc(rc)),
            stdout: Predicted::Top,
            stderr: Predicted::Top,
        }
    }

    /// Fold `src`, observing each named leaf with a concrete rc; all other leaves ⊤.
    fn fold_with(src: &str, observed: &[(&str, i32)]) -> (FoldResult, Ast) {
        let ast = dorc_syntax::parse(src).value;
        let want: BTreeMap<AstId, Observable> = observed
            .iter()
            .map(|(name, rc)| (leaf(&ast, name), obs(*rc)))
            .collect();
        let result = fold(&ast, |id| want.get(&id).copied());
        (result, ast)
    }

    #[test]
    fn oror_known_success_kills_right_operand() {
        // `dpkg -s nginx`(rc 0) `|| apt-get install` ⇒ `||` short-circuits ⇒ the
        // install is provably DEAD. The canonical idempotency idiom (DESIGN).
        let (f, ast) = fold_with("dpkg -s nginx || apt-get install nginx\n", &[("dpkg", 0)]);
        assert!(
            f.is_dead(leaf(&ast, "apt-get")),
            "rc-0 left of `||` ⇒ right operand dead"
        );
        assert!(!f.is_dead(leaf(&ast, "dpkg")), "the guard itself is live");
    }

    #[test]
    fn oror_known_failure_keeps_right_operand_live() {
        // `useradd`(rc 9) `|| mkdir` ⇒ 9≠0 ⇒ the `||` fires ⇒ `mkdir` is LIVE. The
        // round-19 under-execute fix: a non-conforming establish's fallback must run.
        let (f, ast) = fold_with("useradd deploy || mkdir /srv/app\n", &[("useradd", 9)]);
        assert!(
            !f.is_dead(leaf(&ast, "mkdir")),
            "rc≠0 left of `||` ⇒ right operand LIVE (the fallback runs)"
        );
    }

    #[test]
    fn andand_known_failure_kills_right_operand() {
        // `cmd`(rc 1) `&& other` ⇒ `&&` short-circuits on failure ⇒ `other` is DEAD.
        let (f, ast) = fold_with("dpkg -s nginx && apt-get purge nginx\n", &[("dpkg", 1)]);
        assert!(
            f.is_dead(leaf(&ast, "apt-get")),
            "rc≠0 left of `&&` ⇒ right operand dead"
        );
    }

    #[test]
    fn andand_known_success_keeps_right_operand_live() {
        // `install`(rc 0) `&& enable` ⇒ 0 ⇒ `enable` is LIVE (the post-condition).
        let (f, ast) = fold_with(
            "apt-get install nginx && systemctl enable nginx\n",
            &[("apt-get", 0)],
        );
        assert!(
            !f.is_dead(leaf(&ast, "systemctl")),
            "rc 0 left of `&&` ⇒ right operand LIVE"
        );
    }

    #[test]
    fn if_negated_guard_success_kills_then_body() {
        // `if ! cmd(rc 0); then body; fi` ⇒ `!0`=false ⇒ then-body DEAD. The Half-B
        // subsumption: the guard's known rc resolves the branch (the F1 case, at the
        // fold/disposition layer — the render keeps it via mark_status+omit-safety).
        let (f, ast) = fold_with(
            "if ! command -v nginx; then apt-get install nginx; fi\n",
            &[("command", 0)],
        );
        assert!(
            f.is_dead(leaf(&ast, "apt-get")),
            "`if ! (rc 0)` ⇒ then-body dead"
        );
    }

    #[test]
    fn if_guard_failure_kills_then_body_keeps_else() {
        // `if cmd(rc 1); then T; else E; fi` ⇒ T dead, E live (false condition).
        let (f, ast) = fold_with(
            "if dpkg -s nginx; then echo yes; else apt-get install nginx; fi\n",
            &[("dpkg", 1)],
        );
        assert!(
            f.is_dead(leaf(&ast, "echo")),
            "false guard ⇒ then-body dead"
        );
        assert!(
            !f.is_dead(leaf(&ast, "apt-get")),
            "false guard ⇒ else-body LIVE"
        );
    }

    #[test]
    fn unknown_guard_folds_nothing_kfail_perform() {
        // EXCLUSION-CHECK (`tc-reliability` / `inv-kfail`): a ⊤ (un-observed) guard
        // folds NOTHING — both operands stay live. An unreliable/un-probed oracle can
        // never cause an omission (the priority-1 under-execute floor). Here `dpkg` is
        // NOT observed ⇒ ⊤ ⇒ the install is LIVE.
        let (f, ast) = fold_with("dpkg -s nginx || apt-get install nginx\n", &[]);
        assert!(
            !f.is_dead(leaf(&ast, "apt-get")),
            "⊤ guard ⇒ no fold ⇒ right operand LIVE (kFAIL-perform)"
        );
        assert_eq!(
            f.rc_of(leaf(&ast, "dpkg")),
            AbstractRc::Top,
            "an un-observed leaf is ⊤"
        );
    }

    #[test]
    fn negation_inverts_known_status() {
        // `!`-fold: success ⇒ failure, failure ⇒ success, ⊤ ⇒ ⊤ (the bare arithmetic
        // the `if !` fold rests on; pinned directly).
        assert_eq!(negate(AbstractRc::known(0)), AbstractRc::known(1));
        assert_eq!(negate(AbstractRc::known(7)), AbstractRc::known(0));
        assert_eq!(negate(AbstractRc::Top), AbstractRc::Top);
    }

    #[test]
    fn case_does_not_fold_rc_all_arms_live() {
        // EXCLUSION-CHECK (the other construct): `case` switches on a STRING, not an
        // rc, so the rc-fold cannot resolve which arm matches — every arm stays live
        // (string-value abstract-interp is out of scope, recorded in 19C).
        let (f, ast) = fold_with(
            "case nginx in nginx) apt-get install nginx ;; *) echo no ;; esac\n",
            &[],
        );
        assert!(
            !f.is_dead(leaf(&ast, "apt-get")),
            "case arm body is never folded dead by the rc-fold"
        );
    }

    #[test]
    fn determinism_same_input_same_result() {
        // `inv-determinism`: the fold is a pure function — identical (ordered) output
        // from identical input, twice.
        let a = fold_with("useradd x || mkdir /y\n", &[("useradd", 9)]).0;
        let b = fold_with("useradd x || mkdir /y\n", &[("useradd", 9)]).0;
        assert_eq!(a.dead, b.dead);
        assert_eq!(a.node_rc, b.node_rc);
    }
}
