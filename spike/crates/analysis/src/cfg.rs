//! `analysis::cfg` — lower a [`dorc_syntax::Ast`] into a control-flow graph the
//! dataflow framework ([`crate::solve`]) runs over.
//!
//! Design + the why: `Research/notes/163-analysis-engine-design-spa-grounded.md`
//! §3 (CFG construction + the hazard set) and `notes/160-analyzer-chord-synthesis.md`
//! §2 (the hazard set / ⊤-trigger set). This module owns the sh-specific modeling;
//! [`Cfg`] implements the analysis-agnostic [`crate::solve::Graph`] trait so the
//! same worklist solves forward (may-mutate, ambient-gate, ShellEnvState) and
//! backward (apply-minimization slice) over it.
//!
//! Five properties are load-bearing (the spike invariants, `spike/CLAUDE.md`):
//!
//! * **`inv-no-throw`** — [`build`] is **total**: any AST (including
//!   [`NodeKind::Unsupported`] ⊤-nodes and pathologically deep nesting) yields a
//!   [`Cfg`] and never panics. Errors are data ([`Carrier`] diagnostics).
//! * **`inv-determinism`** — pure function of the AST: no clock/RNG/IO, and no
//!   `HashMap`-iteration into output. Node order is the deterministic order in
//!   which the AST walk allocates them.
//! * **`inv-top-reject`** — an [`NodeKind::Unsupported`] AST node becomes a
//!   [`CfgNodeKind::Top`] node, **never silently skipped**. The analyzer treats it
//!   as absorbing ⊤ (un-probeable AND un-skippable).
//! * **`haz-redir-as-mutation`** — redirections are first-class effect nodes
//!   ([`CfgNodeKind::Redir`]), independent of the command word.
//! * **`haz-seterr`** — `set -e`/errexit edges are partly an analysis *output*
//!   (note 163 §3): [`build`] runs a small forward errexit pass and materialises
//!   **precise** conditional failure→`exit` edges (note 166; see [`build`]).

use std::collections::BTreeMap;

use dorc_core::{AstId, Carrier, DiagCode, Diagnostic, Span};
use dorc_syntax::{
    ast::{CaseArm, ElseIf},
    Ast, NodeKind, WordPart,
};

/// Diagnostic codes this module emits (greppable; `ch-catalog`).
const CFG_TOP: DiagCode = DiagCode("cfg-top-node");
const CFG_ERREXIT_TOP: DiagCode = DiagCode("cfg-errexit-unknown");

// ===========================================================================
// Public types
// ===========================================================================

/// Index of a node in a [`Cfg`]'s arena. A newtype, never a bare `usize`
/// (`make illegal states unrepresentable`); [`index`](CfgNodeId::index) projects
/// to the `usize` the [`Graph`](crate::solve::Graph) trait speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CfgNodeId(pub u32);

impl CfgNodeId {
    #[must_use]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// What a [`CfgNode`] represents. The finite set of control-flow node roles the
/// modeled sh subset needs (note 163 §3). Effect-bearing kinds (`Command`,
/// `Redir`, `Top`) carry an [`AstId`] back to the source construct (provenance,
/// `dac-B`); structural kinds (`Entry`/`Exit`/`Merge`/scope boundaries) are
/// synthetic join/sequencing points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfgNodeKind {
    /// Synthetic program entry. The whole-script flow starts here; the solver's
    /// boundary state is seeded at this node (it has no predecessors).
    Entry,
    /// Synthetic program exit. Every terminating path (fall-off-the-end,
    /// `exit`/`return`, an errexit failure-edge) reaches here.
    Exit,
    /// A simple command (`NodeKind::Simple`) — the primary effect site. Its oracle
    /// effect-class (next subagent) is the transfer function's input here.
    Command,
    /// A redirection effect site (`haz-redir-as-mutation`): `> f` mutates `f`
    /// regardless of the command word. Sequenced *before* its owning command so a
    /// terminating command (`exit > f`) never orphans the redirection's effect.
    Redir,
    /// A no-op merge node at a branch join (SPA Exercise 5.19: bounds `|pred|`/
    /// `|succ| ≤ 2` per branch, keeping the worklist cheap). Carries no effect.
    Merge,
    /// Enter a subshell `( )` / command-substitution `$( )` scope
    /// (`haz-concurrency`): the next subagent's ShellEnvState pass pushes a frame
    /// here. env/var/cwd mutations inside DO NOT escape; FS mutations DO.
    ScopeEnter,
    /// Leave a subshell/`$( )` scope: pop the frame, projecting out env/var/cwd
    /// mutations (the inverse-transient). FS effects already escaped.
    ScopeExit,
    /// An absorbing ⊤ node for an `NodeKind::Unsupported` construct
    /// (`inv-top-reject`): un-probeable AND un-skippable. The analyzer must fold
    /// this to ⊤ for its phase, never silently best-effort past it.
    Top,
}

/// One CFG node: its role plus the [`AstId`] it derives from (provenance). For
/// synthetic nodes (`Entry`/`Exit`/`Merge`/scope) the `ast` points at the nearest
/// meaningful construct (the enclosing compound, or the script root) so a
/// diagnostic can still locate it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CfgNode {
    pub ast: AstId,
    pub kind: CfgNodeKind,
}

/// A control-flow graph over [`CfgNodeId`]s. Adjacency is stored as sorted,
/// de-duplicated successor/predecessor lists kept mutually consistent
/// (`w ∈ succ(v) ⟺ v ∈ pred(w)`), so iteration into any analysis output is
/// deterministic (`inv-determinism`) and the [`Graph`](crate::solve::Graph)
/// contract holds.
#[derive(Debug, Clone)]
pub struct Cfg {
    nodes: Vec<CfgNode>,
    entry: CfgNodeId,
    exit: CfgNodeId,
    succ: Vec<Vec<usize>>,
    pred: Vec<Vec<usize>>,
}

impl Cfg {
    #[must_use]
    pub fn entry(&self) -> CfgNodeId {
        self.entry
    }

    #[must_use]
    pub fn exit(&self) -> CfgNodeId {
        self.exit
    }

    /// Resolve a node id minted for *this* graph.
    #[must_use]
    pub fn node(&self, id: CfgNodeId) -> &CfgNode {
        &self.nodes[id.index()]
    }

    /// All nodes paired with their ids (whole-graph passes / provenance).
    pub fn iter(&self) -> impl Iterator<Item = (CfgNodeId, &CfgNode)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (CfgNodeId(i as u32), n))
    }

    /// Successors of `id` as typed ids (a view over the [`Graph`] adjacency).
    pub fn succ_ids(&self, id: CfgNodeId) -> impl Iterator<Item = CfgNodeId> + '_ {
        self.succ[id.index()].iter().map(|&w| CfgNodeId(w as u32))
    }

    /// Predecessors of `id` as typed ids.
    pub fn pred_ids(&self, id: CfgNodeId) -> impl Iterator<Item = CfgNodeId> + '_ {
        self.pred[id.index()].iter().map(|&w| CfgNodeId(w as u32))
    }
}

impl crate::solve::Graph for Cfg {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn succ(&self, node: usize) -> &[usize] {
        &self.succ[node]
    }
    fn pred(&self, node: usize) -> &[usize] {
        &self.pred[node]
    }
}

// ===========================================================================
// Builder
// ===========================================================================

/// Lower an [`Ast`] into a [`Cfg`] (`pub fn build`). **Total + pure** — never
/// panics on any AST (`inv-no-throw`), including ⊤-nodes and deep nesting, and
/// performs no I/O (`inv-determinism`).
///
/// Two-phase, because `set -e` couples construction and dataflow (note 163 §3,
/// `haz-seterr`):
///
/// 1. **Base CFG** — a structural walk of the AST emitting effect/structural
///    nodes and the ordinary sequential / branch / short-circuit edges.
/// 2. **errexit materialisation (precise — note 166)** — a small forward pass
///    tracks `errexit ∈ {on, off, ⊤}` per node (toggled by `set -e`/`set +e`;
///    `set "$dyn"` ⇒ ⊤; subshell toggles do not leak — find-4). Where errexit
///    *may* be on at a fallible `Command` **or** `Redir` (find-5), an implicit
///    failure→`exit` edge is added. The edge set is **precise**, not
///    over-approximate: it is *not* added where the shell never aborts — a
///    `!`-negated pipeline (find-1), anywhere in an `if`/`while`/`until` test or a
///    `&&`/`||` left operand (the whole region — find-2/3), or a `|| true` swallow
///    (`haz-swallow`). The single remaining conservative direction is `set "$dyn"`
///    ⇒ ⊤ ⇒ add the edge (dynamic option; genuinely unknown).
///
/// Why precise matters (note 166 find-8): a *spurious* `cmd→exit` edge is unsound
/// **backward** — the apply-minimization slice would see a post-`cmd` mutation as
/// conditionally-bypassed and could skip an always-reached mutation
/// (`kFAIL-perform` violation). A *missing* edge is unsound **forward** (a wrong
/// skip). Both directions are now fixed.
///
/// *Known residue (flagged):* special-built-in redirection-error abort (`: > /bad`
/// aborts *unconditionally*, not via `set -e` — see note 166 follow-up); `pipefail`
/// interaction; cross-call errexit into function bodies (find-7) —
/// `fork-errexit-semantics` (note 160 §9).
#[must_use]
pub fn build(ast: &Ast) -> Carrier<Cfg> {
    let mut b = Builder::new(ast);
    let entry = b.fresh(ast.root(), CfgNodeKind::Entry);
    let exit = b.fresh(ast.root(), CfgNodeKind::Exit);
    b.entry = entry;
    b.exit = exit;

    // Phase 1: structural walk. The script body flows entry → … → exit; a path
    // that runs off the end (no terminator) falls through to `exit`.
    let tail = b.lower_node(ast.root(), entry);
    b.add_edge(tail, exit);

    // Phase 2: errexit failure-edges (the haz-seterr coupling).
    b.materialise_errexit_edges();

    b.finish()
}

/// `errexit` abstract value — a height-2 flat lattice (off ⊑ {on}, ⊤ above),
/// matching `Flat` in the framework but kept local (the builder's pass is
/// hand-rolled, not run through `solve`, so `build` stays self-contained).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ErrExit {
    Off,
    On,
    /// `set "$dyn"` or a join of On and Off — don't know ⇒ over-approximate
    /// (assume the failure-edge may exist).
    Top,
}

impl ErrExit {
    /// Join two errexit facts (the forward pass merges at every predecessor).
    fn join(self, other: ErrExit) -> ErrExit {
        match (self, other) {
            (a, b) if a == b => a,
            (ErrExit::Top, _) | (_, ErrExit::Top) => ErrExit::Top,
            // On ⊔ Off — disagreement ⇒ ⊤ (may be on; add the edge).
            _ => ErrExit::Top,
        }
    }

    /// Could errexit be on here? (`On` or `Top` ⇒ materialise the failure-edge.)
    fn may_be_on(self) -> bool {
        matches!(self, ErrExit::On | ErrExit::Top)
    }
}

struct Builder<'a> {
    ast: &'a Ast,
    nodes: Vec<CfgNode>,
    succ: Vec<Vec<usize>>,
    pred: Vec<Vec<usize>>,
    diags: Vec<Diagnostic>,
    entry: CfgNodeId,
    exit: CfgNodeId,
    /// Per-node: does this node toggle errexit, and is it fallible /
    /// in a condition context? Populated during the walk; consumed by the
    /// errexit pass so the two phases share one source of truth (haz-seterr).
    /// `fallible` is set on both `Command` and `Redir` nodes (find-5): a failing
    /// redirection aborts under `set -e` just as a failing command does.
    fallible: Vec<bool>,
    toggle: Vec<Option<ErrExit>>,
    /// `ScopeExit` node → its matching `ScopeEnter` (find-4): the errexit forward
    /// pass restores the *pre-subshell* state at the exit, so a `set -e`/`set +e`
    /// toggle inside `( )`/`$( )` never leaks out. Both directions are kept so the
    /// worklist can re-queue the exit when its enter's inflow changes (keeping the
    /// fixed point correct despite there being no enter→exit control edge).
    /// `BTreeMap` (not `HashMap`) for `inv-determinism`.
    exit_to_enter: BTreeMap<usize, usize>,
    enter_to_exit: BTreeMap<usize, usize>,
    /// Recursion-depth guard mirroring the parser's (`inv-no-throw`): an AST that
    /// is pathologically deep (despite the parser's own bound) cannot blow the
    /// native stack here either.
    depth: u32,
}

/// Mirror of the parser's depth bound; the AST is already bounded by it, but the
/// CFG walk guards independently so `build` is total against *any* `Ast` value
/// (even a hand-constructed one), not only parser output.
const MAX_DEPTH: u32 = 512;

impl<'a> Builder<'a> {
    fn new(ast: &'a Ast) -> Self {
        Builder {
            ast,
            nodes: Vec::new(),
            succ: Vec::new(),
            pred: Vec::new(),
            diags: Vec::new(),
            entry: CfgNodeId(0),
            exit: CfgNodeId(0),
            fallible: Vec::new(),
            toggle: Vec::new(),
            exit_to_enter: BTreeMap::new(),
            enter_to_exit: BTreeMap::new(),
            depth: 0,
        }
    }

    // ---- node + edge primitives ----------------------------------------------

    fn fresh(&mut self, ast: AstId, kind: CfgNodeKind) -> CfgNodeId {
        let id = CfgNodeId(u32::try_from(self.nodes.len()).unwrap_or(u32::MAX));
        self.nodes.push(CfgNode { ast, kind });
        self.succ.push(Vec::new());
        self.pred.push(Vec::new());
        self.fallible.push(false);
        self.toggle.push(None);
        id
    }

    /// Add a directed edge, keeping `succ`/`pred` mutually consistent and free of
    /// duplicates (a node can be reached two ways — e.g. a merge — but the edge
    /// itself is recorded once, so the worklist does not double-count).
    fn add_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        let (f, t) = (from.index(), to.index());
        if !self.succ[f].contains(&t) {
            self.succ[f].push(t);
        }
        if !self.pred[t].contains(&f) {
            self.pred[t].push(f);
        }
    }

    // ---- the structural walk --------------------------------------------------

    /// Lower `id`'s subtree into CFG nodes, wiring `entry_pred` into the region's
    /// first node. Returns the region's **exit node** — the single node a caller
    /// sequences the continuation onto. A region that always terminates (every
    /// path `exit`s/`return`s) returns a fresh unreachable [`CfgNodeKind::Merge`]
    /// (zero in-edges), so the caller's follow-on code is correctly dead.
    fn lower_node(&mut self, id: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        if self.depth >= MAX_DEPTH {
            // Over-deep: a ⊤ node, wired in, so we neither panic nor silently drop.
            let top = self.fresh(id, CfgNodeKind::Top);
            self.diags.push(Diagnostic::warning(
                CFG_TOP,
                Some(self.span(id)),
                "CFG nesting bound hit; construct treated as ⊤ (un-probeable)",
            ));
            self.add_edge(entry_pred, top);
            return top;
        }
        self.depth += 1;
        let out = self.lower_node_inner(id, entry_pred);
        self.depth -= 1;
        out
    }

    fn lower_node_inner(&mut self, id: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        match &self.ast.node(id).kind {
            NodeKind::Script { items } | NodeKind::List { items } => {
                self.lower_sequence(&items.clone(), entry_pred)
            }
            NodeKind::Simple {
                assigns,
                words,
                redirs,
            } => {
                let (assigns, words, redirs) = (assigns.clone(), words.clone(), redirs.clone());
                self.lower_simple(id, &assigns, &words, &redirs, entry_pred)
            }
            NodeKind::Pipeline { stages, negated } => {
                let (stages, negated) = (stages.clone(), *negated);
                self.lower_pipeline(&stages, negated, entry_pred)
            }
            NodeKind::AndOr { op, left, right } => {
                let (op, left, right) = (*op, *left, *right);
                self.lower_and_or(op, left, right, entry_pred)
            }
            NodeKind::If {
                cond,
                then_body,
                elifs,
                else_body,
            } => {
                let (cond, then_body) = (*cond, *then_body);
                let (elifs, else_body) = (elifs.clone(), *else_body);
                self.lower_if(id, cond, then_body, &elifs, else_body, entry_pred)
            }
            NodeKind::Case { word, arms } => {
                let (word, arms) = (*word, arms.clone());
                self.lower_case(id, word, &arms, entry_pred)
            }
            NodeKind::Subshell { body, redirs } => {
                let (body, redirs) = (*body, redirs.clone());
                self.lower_scoped(id, body, &redirs, entry_pred)
            }
            NodeKind::Group { body, redirs } => {
                // A brace-group runs in the *current* shell (no scope boundary):
                // its env/var mutations DO escape. Model it as the body plus any
                // group-level redirections, with no ScopeEnter/Exit.
                let (body, redirs) = (*body, redirs.clone());
                let after_body = self.lower_node(body, entry_pred);
                self.attach_redirs(id, &redirs, after_body)
            }
            NodeKind::FuncDef { body, .. } => self.lower_funcdef(id, *body, entry_pred),
            NodeKind::Unsupported { .. } => self.lower_top(id, entry_pred),
            // Leaf word/assign/redir nodes never appear as a *statement* head here
            // (the parser nests them inside Simple/Redir); if one does, treat it as
            // a no-op pass-through so the walk stays total.
            NodeKind::Word { .. } | NodeKind::Assign { .. } | NodeKind::Redir { .. } => {
                let m = self.fresh(id, CfgNodeKind::Merge);
                self.add_edge(entry_pred, m);
                m
            }
        }
    }

    /// Sequence a list of statements: each region's exit feeds the next region's
    /// entry. Empty list ⇒ a pass-through merge node.
    fn lower_sequence(&mut self, items: &[AstId], entry_pred: CfgNodeId) -> CfgNodeId {
        if items.is_empty() {
            // Preserve a single sequencing point even for an empty body.
            let m = self.fresh(self.ast.root(), CfgNodeKind::Merge);
            self.add_edge(entry_pred, m);
            return m;
        }
        let mut cur = entry_pred;
        for &item in items {
            cur = self.lower_node(item, cur);
        }
        cur
    }

    /// A simple command. Redirections come first (their effects are established
    /// before the command body runs — so `exit > f` still records `> f`), then any
    /// command-substitution regions in the assignments/words (their subshells run
    /// during expansion, before the command word), then the command node. A
    /// terminating command (`exit`/`return`) routes to the program exit with no
    /// fall-through.
    ///
    /// errexit + command substitution (find-6): a `$( … )` failure aborts under
    /// `set -e` *only when its status becomes the host's* — i.e. an assignment-only
    /// command `x=$(false)` (`[RAN]` aborts; note 166), `a=1 b=$(false)` (`[RAN]`
    /// aborts). When a command word is present, `echo $(false)` takes `echo`'s
    /// status and does NOT abort (`[RAN]`). We need no special fallibility wiring
    /// for either: the host `Simple` is one `Command` node already flagged fallible,
    /// so the assignment-only case is covered, and the command-word case correctly
    /// attributes fallibility to the command word (not the masked subst). What was
    /// missing was the subst *regions* themselves — lowered here so the downstream
    /// effect analysis sees their effects; find-4 keeps any `set -e` they contain
    /// from leaking out.
    fn lower_simple(
        &mut self,
        id: AstId,
        assigns: &[AstId],
        words: &[AstId],
        redirs: &[AstId],
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        let mut cur = entry_pred;
        for &r in redirs {
            let rn = self.fresh(r, CfgNodeKind::Redir);
            // A failing redirection aborts under `set -e` regardless of the command
            // word (find-5): mark it fallible so phase 2 gives it a failure-edge.
            // A condition-context / negated-pipeline clear (clear_fallible_range)
            // will later unset this where the shell exempts it.
            self.fallible[rn.index()] = true;
            self.add_edge(cur, rn);
            cur = rn;
        }
        // Command-substitution subshells in the assignment values and command words
        // run during expansion (find-6). Assignment RHS is a `Word`; each word is a
        // `Word`. Lower their `$( … )` bodies as scoped regions, sequenced here.
        for &a in assigns {
            if let NodeKind::Assign { value: Some(v), .. } = &self.ast.node(a).kind {
                cur = self.lower_word_substs(*v, cur);
            }
        }
        for &w in words {
            cur = self.lower_word_substs(w, cur);
        }
        let cmd = self.fresh(id, CfgNodeKind::Command);
        self.add_edge(cur, cmd);

        // Record errexit-relevant facts for phase 2 (single source of truth).
        if let Some(t) = self.errexit_toggle(words) {
            self.toggle[cmd.index()] = Some(t);
        }
        // A command is "fallible" (a candidate for the errexit failure-edge) unless
        // it is a `set` toggle we just classified; we still mark normal commands.
        self.fallible[cmd.index()] = true;

        if self.is_terminator(words) {
            // `exit`/`return`: edge to program exit, NO fall-through. The caller's
            // continuation becomes dead (a fresh, unreachable merge).
            self.add_edge(cmd, self.exit);
            let dead = self.fresh(id, CfgNodeKind::Merge);
            return dead;
        }
        cmd
    }

    /// A pipeline `a | b | c`: stages run left-to-right; the *last* stage's status
    /// is what `set -e`/`pipefail` key on (so only the last stage carries the
    /// fallibility for the errexit pass). Stages run in subshell environments
    /// (`haz-concurrency`), but v1 models the pipeline as a simple sequence of its
    /// stage-regions; per-stage env-scoping is a flagged refinement (the next
    /// subagent may wrap each stage in a scope when it needs env-isolation).
    ///
    /// A `!`-negated pipeline never fires errexit (find-1): POSIX exempts a command
    /// whose status is negated with `!` from `set -e`, *even* `! true`
    /// (`[RAN]` dash 0.5 — note 166). This is also Dorc's own guard idiom. So a
    /// negated pipeline's governing (last) stage is cleared of fallibility, the
    /// same as a condition-context command.
    fn lower_pipeline(
        &mut self,
        stages: &[AstId],
        negated: bool,
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        let first = self.nodes.len();
        let mut cur = entry_pred;
        let last = stages.len().saturating_sub(1);
        for (i, &stage) in stages.iter().enumerate() {
            cur = self.lower_node(stage, cur);
            if i != last {
                // Only the last stage governs pipeline status; clear the
                // fallibility flag on a non-last stage's command exit node.
                if let CfgNodeKind::Command = self.nodes[cur.index()].kind {
                    self.fallible[cur.index()] = false;
                }
            }
        }
        if negated {
            // The whole pipeline's status is negated ⇒ errexit cannot fire on it.
            // Clear fallibility across every node the pipeline produced (covering
            // the last stage's command *and* its redirections — find-1 + find-5).
            self.clear_fallible_range(first, self.nodes.len());
        }
        cur
    }

    /// `left && right` / `left || right` — **short-circuit** edges. `left` always
    /// runs; `right` runs conditionally on `left`'s status; both reach the
    /// continuation (a merge). The whole `left` region is a *condition context*, so
    /// nothing in it gets an errexit failure-edge (errexit is suppressed in
    /// `&&`/`||` left operands — `[RAN]` `false && echo X`, `true && false && …`;
    /// note 166 find-3). For `||`, `left || true`-style swallowing falls out of the
    /// same rule. The `right` operand is NOT exempt at top level (`[RAN]`
    /// `true && false` aborts); left-associative chains nest so each inner left is
    /// covered by *its* enclosing `lower_and_or`.
    fn lower_and_or(
        &mut self,
        _op: dorc_syntax::ast::AndOrOp,
        left: AstId,
        right: AstId,
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        let left_exit = self.lower_condition_region(left, entry_pred);

        let right_exit = self.lower_node(right, left_exit);
        // Hang the join's provenance on the left operand (a reasonable locator).
        let merge = self.fresh(left, CfgNodeKind::Merge);
        // Short-circuit edge: left may skip right and go straight to the merge.
        self.add_edge(left_exit, merge);
        // Fall-through edge: right's exit reaches the merge.
        self.add_edge(right_exit, merge);
        merge
    }

    /// `if cond; then …; [elif …;]* [else …;] fi`. The condition region flows into
    /// a branch: `then` on success, the next `elif`/`else`/merge on failure. All
    /// branch exits converge on one merge node (the join). The `cond` is a
    /// condition context (no errexit failure-edge on its trailing command).
    fn lower_if(
        &mut self,
        id: AstId,
        cond: AstId,
        then_body: AstId,
        elifs: &[ElseIf],
        else_body: Option<AstId>,
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        let merge = self.fresh(id, CfgNodeKind::Merge);
        self.lower_if_chain(cond, then_body, elifs, else_body, entry_pred, merge);
        merge
    }

    /// The recursive spine of `if`/`elif`: evaluate `cond`, branch to `then_body`
    /// (success) or the rest (failure), converge both on `merge`.
    fn lower_if_chain(
        &mut self,
        cond: AstId,
        then_body: AstId,
        elifs: &[ElseIf],
        else_body: Option<AstId>,
        entry_pred: CfgNodeId,
        merge: CfgNodeId,
    ) {
        // The WHOLE condition is errexit-exempt, not just its tail (find-2): every
        // command/redir in the test region is cleared of fallibility (`[RAN]`
        // `if false; true; then …` runs the body, no abort; note 166).
        let cond_exit = self.lower_condition_region(cond, entry_pred);

        // Success path: then_body.
        let then_exit = self.lower_node(then_body, cond_exit);
        self.add_edge(then_exit, merge);

        // Failure path: the next elif, else, or (no else) straight to the merge.
        match elifs.split_first() {
            Some((head, rest)) => {
                self.lower_if_chain(head.cond, head.body, rest, else_body, cond_exit, merge);
            }
            None => match else_body {
                Some(eb) => {
                    let else_exit = self.lower_node(eb, cond_exit);
                    self.add_edge(else_exit, merge);
                }
                None => {
                    // No else: the condition's failure falls through to the merge.
                    self.add_edge(cond_exit, merge);
                }
            },
        }
    }

    /// `case word in (pat) body ;; … esac`. The scrutinee is evaluated (its word
    /// may contain a command substitution — lowered as a `Command` region), then
    /// one branch per arm, all converging on a merge. An arm whose body terminates
    /// (`*) … exit 0`) routes to the program exit and does NOT reach the merge.
    fn lower_case(
        &mut self,
        id: AstId,
        word: AstId,
        arms: &[CaseArm],
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        // Evaluate the scrutinee word (pulls in any `$(…)` substitution effects).
        let scrut = self.lower_word_effects(word, entry_pred);
        let merge = self.fresh(id, CfgNodeKind::Merge);
        for arm in arms {
            // Each arm is a separate branch from the scrutinee. Its body region's
            // exit reaches the merge (unless the body terminated — then its
            // returned exit is a fresh unreachable node with no path to merge).
            let body_exit = self.lower_node(arm.body, scrut);
            self.add_edge(body_exit, merge);
        }
        // No arm matched (no default `*`): control falls through to the merge.
        self.add_edge(scrut, merge);
        merge
    }

    /// Subshell `( body )` / the body of a `$( )` substitution: a scoped region.
    /// `ScopeEnter` → body → `ScopeExit`; the next subagent's ShellEnvState pass
    /// pushes/pops a frame at these so env/var/cwd mutations don't escape
    /// (`haz-concurrency`). Subshell-level redirections attach after the scope.
    fn lower_scoped(
        &mut self,
        id: AstId,
        body: AstId,
        redirs: &[AstId],
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        let enter = self.fresh(id, CfgNodeKind::ScopeEnter);
        self.add_edge(entry_pred, enter);
        let body_exit = self.lower_node(body, enter);
        let leave = self.fresh(id, CfgNodeKind::ScopeExit);
        self.add_edge(body_exit, leave);
        self.pair_scope(enter, leave);
        self.attach_redirs(id, redirs, leave)
    }

    /// A function definition's body is a **detached** sub-CFG (Tier-A is
    /// intraprocedural — note 163 §5): it is built (so its nodes/effects exist and
    /// a future Tier-B call edge can target them) but NOT wired into the main
    /// flow. Defining a function has no runtime effect; a *call* to it would be
    /// Tier-B → ⊤ (no call detection in the modeled subset). The definition is a
    /// pass-through in the enclosing flow.
    fn lower_funcdef(&mut self, id: AstId, body: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        // Detached region: its own entry, unreferenced by `entry_pred`.
        //
        // TODO(find-7): the body's `Entry` is pred-less, so the errexit forward pass
        // computes `Off` throughout the body and its commands get no failure-edges
        // even when the *call site* runs under `set -e`. Harmless today (calls
        // aren't modeled — the body is dead), but wrong once Tier-B call edges land:
        // the body's errexit inflow must then come from the call site, not a seeded
        // `Off`. Fix together with call-edge modeling, not piecemeal here (note 166).
        let func_entry = self.fresh(body, CfgNodeKind::Entry);
        let body_exit = self.lower_node(body, func_entry);
        let func_exit = self.fresh(body, CfgNodeKind::Exit);
        self.add_edge(body_exit, func_exit);

        // The *definition statement* is a no-op pass-through in the main flow.
        let m = self.fresh(id, CfgNodeKind::Merge);
        self.add_edge(entry_pred, m);
        m
    }

    /// An `NodeKind::Unsupported` construct → an absorbing ⊤ node (`inv-top-reject`).
    /// Wired into the flow (never skipped) and reported. Salvaged children are NOT
    /// lowered into the live flow: under ⊤ their control-flow is unknown-by-design;
    /// the absorbing node already forces every downstream conclusion to ⊤.
    fn lower_top(&mut self, id: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        let top = self.fresh(id, CfgNodeKind::Top);
        self.add_edge(entry_pred, top);
        self.diags.push(Diagnostic::error(
            CFG_TOP,
            Some(self.span(id)),
            "unsupported construct (⊤): un-probeable and un-skippable",
        ));
        top
    }

    /// Lower the effect of evaluating a *word* as a standalone region (a `case`
    /// scrutinee): its command substitutions become scoped sub-regions (it runs in
    /// a subshell). A pure-expansion scrutinee (no `$( … )`) yields a single
    /// pass-through merge so the `case` arms always have one node to branch from.
    fn lower_word_effects(&mut self, word_id: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        let after_substs = self.lower_word_substs(word_id, entry_pred);
        if after_substs == entry_pred {
            // No substitution region was emitted (pure expansion): keep the original
            // single-merge shape so the scrutinee has its own exit node.
            let m = self.fresh(word_id, CfgNodeKind::Merge);
            self.add_edge(entry_pred, m);
            return m;
        }
        after_substs
    }

    /// Lower just the command-substitution subshells inside a word, in source
    /// order, each a scoped region (subshell semantics). Returns `entry_pred`
    /// unchanged when the word has no `$( … )` (so a caller that wants no extra
    /// node — `lower_simple`, find-6 — gets none). Shared by the `case` scrutinee
    /// path ([`lower_word_effects`]) and assignment/argument expansion.
    fn lower_word_substs(&mut self, word_id: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        let mut cur = entry_pred;
        for body in self.command_substs(word_id) {
            let enter = self.fresh(word_id, CfgNodeKind::ScopeEnter);
            self.add_edge(cur, enter);
            let body_exit = self.lower_node(body, enter);
            let leave = self.fresh(word_id, CfgNodeKind::ScopeExit);
            self.add_edge(body_exit, leave);
            self.pair_scope(enter, leave);
            cur = leave;
        }
        cur
    }

    /// Record a `ScopeEnter`/`ScopeExit` pair (find-4) so the errexit forward pass
    /// can restore the pre-subshell errexit state at the exit. Both directions are
    /// stored: exit→enter for the restore, enter→exit so a change to the enter's
    /// inflow re-queues the exit during the worklist.
    fn pair_scope(&mut self, enter: CfgNodeId, leave: CfgNodeId) {
        self.exit_to_enter.insert(leave.index(), enter.index());
        self.enter_to_exit.insert(enter.index(), leave.index());
    }

    /// Append redirection effect nodes after `pred`, returning the new region exit.
    /// (Used for group/subshell-level redirs that follow the body.)
    fn attach_redirs(&mut self, _owner: AstId, redirs: &[AstId], pred: CfgNodeId) -> CfgNodeId {
        let mut cur = pred;
        for &r in redirs {
            let rn = self.fresh(r, CfgNodeKind::Redir);
            self.fallible[rn.index()] = true; // find-5: redir aborts under `set -e`
            self.add_edge(cur, rn);
            cur = rn;
        }
        cur
    }

    // ---- errexit (phase 2) ----------------------------------------------------

    /// Materialise the conditional failure→`exit` edges (`haz-seterr`). Runs a
    /// forward fixed-point over the *base* CFG computing `errexit ∈ {Off,On,⊤}` at
    /// each node, then adds a `node → exit` edge at every fallible `Command`/`Redir`
    /// where errexit may be on. **Precise** (note 166): the failure-edge set is
    /// pruned where the shell never aborts — negated pipelines (find-1), whole
    /// condition regions (find-2/3), `|| true` swallows — and *extended* where it
    /// does — failing redirections (find-5). Subshell `set -e`/`set +e` toggles do
    /// not leak out (find-4). `set "$dyn"` still over-approximates to ⊤ (add the
    /// edge), which is the one remaining conservative direction.
    fn materialise_errexit_edges(&mut self) {
        let n = self.nodes.len();
        // before[v] = join of predecessors' after-states; after[v] applies v's
        // toggle (a `set -e`/`set +e` command) to before[v], or restores the
        // pre-subshell state at a `ScopeExit` (find-4). Entry seeds Off (a script
        // starts with errexit off until `set -e`).
        let mut before = vec![ErrExit::Off; n];
        // Standard worklist to a fixed point (height-2 lattice ⇒ terminates fast).
        let mut work: Vec<usize> = (0..n).collect();
        let mut queued = vec![true; n];
        while let Some(v) = work.pop() {
            queued[v] = false;
            let inflow = self.errexit_inflow(v, &before);
            if inflow != before[v] {
                before[v] = inflow;
                // A `ScopeExit` restores its matching `ScopeEnter`'s inflow, but
                // there is no enter→exit control edge to propagate that change.
                // Re-queue the partner exit so its `after` (and successors) catch
                // up — keeping the fixed point correct (find-4).
                if let Some(&leave) = self.enter_to_exit.get(&v) {
                    if !queued[leave] {
                        queued[leave] = true;
                        work.push(leave);
                    }
                }
            }
            let after_v = self.errexit_after(v, &before);
            for &w in &self.succ[v] {
                let joined = before[w].join(after_v);
                if joined != before[w] {
                    before[w] = joined;
                    if !queued[w] {
                        queued[w] = true;
                        work.push(w);
                    }
                }
            }
        }

        // Materialise edges (collect first to avoid borrow conflict). Both
        // `Command` and `Redir` nodes abort under `set -e` when fallible (find-5).
        let exit = self.exit;
        let mut to_add: Vec<usize> = Vec::new();
        let mut saw_top = false;
        for (v, node) in self.nodes.iter().enumerate() {
            if matches!(node.kind, CfgNodeKind::Command | CfgNodeKind::Redir)
                && self.fallible[v]
                && self.errexit_after(v, &before).may_be_on()
            {
                to_add.push(v);
                if before[v] == ErrExit::Top {
                    saw_top = true;
                }
            }
        }
        for v in to_add {
            self.add_edge(CfgNodeId(v as u32), exit);
        }
        if saw_top {
            self.diags.push(Diagnostic::warning(
                CFG_ERREXIT_TOP,
                None,
                "errexit state is ⊤ at one or more commands; failure-edges \
                 added conservatively (over-approximate, sound)",
            ));
        }
    }

    /// `before[v]` recomputed = join over predecessors of their after-states.
    /// (Entry, with no predecessors, stays at its seed `Off`.)
    fn errexit_inflow(&self, v: usize, before: &[ErrExit]) -> ErrExit {
        if self.pred[v].is_empty() {
            return before[v]; // boundary: keep the seed (Off at entry)
        }
        let mut acc: Option<ErrExit> = None;
        for &p in &self.pred[v] {
            let after_p = self.errexit_after(p, before);
            acc = Some(match acc {
                Some(a) => a.join(after_p),
                None => after_p,
            });
        }
        acc.unwrap_or(ErrExit::Off)
    }

    /// Apply node `v`'s effect on errexit. A `set -e`/`set +e` toggle overrides; a
    /// `ScopeExit` *restores* its matching `ScopeEnter`'s inflow so a toggle inside
    /// `( )`/`$( )` does not leak out (find-4, `[RAN]` note 166); any other node
    /// passes its incoming state through.
    fn errexit_after(&self, v: usize, before: &[ErrExit]) -> ErrExit {
        if let Some(t) = self.toggle[v] {
            return t;
        }
        if let Some(&enter) = self.exit_to_enter.get(&v) {
            return before[enter];
        }
        before[v]
    }

    // ---- sh classification helpers (pure) -------------------------------------

    /// If `words` is a `set` command toggling errexit, return the new errexit
    /// state. `set -e`/`set -euo …` ⇒ On; `set +e` ⇒ Off; `set "$dyn"` (a
    /// non-literal option) ⇒ ⊤. A `set` with options not touching `e` returns
    /// `None` (no errexit change). Non-`set` commands return `None`.
    fn errexit_toggle(&self, words: &[AstId]) -> Option<ErrExit> {
        let first = self.word_literal(*words.first()?)?;
        if first != "set" {
            return None;
        }
        let mut result: Option<ErrExit> = None;
        for &w in &words[1..] {
            match self.word_literal(w) {
                Some(opt) if is_set_minus_e(opt) => result = Some(ErrExit::On),
                Some(opt) if is_set_plus_e(opt) => result = Some(ErrExit::Off),
                Some(_) => {} // an option not touching `e` (e.g. `-u`, `-o pipefail`)
                None => return Some(ErrExit::Top), // dynamic option ⇒ unknown
            }
        }
        result
    }

    /// Is this a path-terminating command (`exit`/`return`)? Such a command routes
    /// to the program exit with no fall-through (the fixture's `*) … exit 0` arm).
    fn is_terminator(&self, words: &[AstId]) -> bool {
        matches!(
            words.first().and_then(|&w| self.word_literal(w)),
            Some("exit" | "return")
        )
    }

    /// The statically-fixed literal of a word (the only case treated as a known
    /// token — command names, sub-verbs; mirrors `ast::Word::as_literal`). A word
    /// that may word-split / is an expansion is NOT a fixed literal.
    fn word_literal(&self, id: AstId) -> Option<&'a str> {
        // The only statically-fixed-string case (`ast::Word::as_literal`): a lone
        // unquoted or single-quoted literal part. Anything with an expansion that
        // may word-split is not a fixed token.
        match &self.ast.node(id).kind {
            NodeKind::Word { parts } => match parts.as_slice() {
                [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    /// Command-substitution bodies inside a word (each a nested `List`/`Script`
    /// AstId), in source order. Only top-level and double-quoted parts are walked
    /// (the lossless-quoting model); a `$( … )` runs in a subshell either way.
    fn command_substs(&self, word_id: AstId) -> Vec<AstId> {
        let mut out = Vec::new();
        if let NodeKind::Word { parts } = &self.ast.node(word_id).kind {
            collect_substs(parts, &mut out);
        }
        out
    }

    // ---- condition-context tagging --------------------------------------------

    /// Lower a *condition-context* region (the test of an `if`/`elif`, or the left
    /// operand of `&&`/`||`) and clear errexit-fallibility across **every** node it
    /// produces, returning the region's exit. errexit is suppressed throughout such
    /// a region, not merely at its tail (find-2), and reaches the inner non-final
    /// operands of a compound test (find-3) — both of which the old tail-only
    /// `mark_condition_context` missed when the region exit was a `Merge`.
    ///
    /// Implemented as a node-range clear: CFG nodes are arena-allocated in walk
    /// order, so the half-open range `[before, after)` is exactly the region's
    /// nodes (`inv-determinism` makes this range stable).
    fn lower_condition_region(&mut self, cond: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        let first = self.nodes.len();
        let exit = self.lower_node(cond, entry_pred);
        self.clear_fallible_range(first, self.nodes.len());
        exit
    }

    /// Clear the fallibility flag on every node in the half-open arena range
    /// `[from, to)` (a condition region, or a negated pipeline — find-1/2/3/5).
    /// Clearing a non-fallible node is a no-op, so over-broad ranges are harmless.
    fn clear_fallible_range(&mut self, from: usize, to: usize) {
        for f in self.fallible[from..to].iter_mut() {
            *f = false;
        }
    }

    // ---- misc -----------------------------------------------------------------

    fn span(&self, id: AstId) -> Span {
        self.ast.node(id).span
    }

    fn finish(self) -> Carrier<Cfg> {
        let cfg = Cfg {
            nodes: self.nodes,
            entry: self.entry,
            exit: self.exit,
            succ: self.succ,
            pred: self.pred,
        };
        debug_assert!(
            cfg_is_consistent(&cfg),
            "succ/pred must be mutually consistent"
        );
        Carrier::new(cfg, self.diags)
    }
}

// ===========================================================================
// Free helpers (pure)
// ===========================================================================

/// Recursively collect `$( … )` substitution body ids from word parts (walking
/// into double-quoted nesting, since `"$(cmd)"` still runs the command).
fn collect_substs(parts: &[WordPart], out: &mut Vec<AstId>) {
    for p in parts {
        match p {
            WordPart::CommandSubst(id) => out.push(*id),
            WordPart::DoubleQuoted(inner) => collect_substs(inner, out),
            _ => {}
        }
    }
}

/// `-e` set in a `set` short-option cluster (`-e`, `-eu`, `-euo`, …). A leading
/// `-` followed by option letters that include `e`.
fn is_set_minus_e(opt: &str) -> bool {
    opt.starts_with('-') && !opt.starts_with("--") && opt[1..].contains('e')
}

/// `+e` clears errexit (`+e`, `+eu`, …).
fn is_set_plus_e(opt: &str) -> bool {
    opt.starts_with('+') && opt[1..].contains('e')
}

/// Verify the [`Graph`](crate::solve::Graph) consistency invariant for a built
/// [`Cfg`]: `w ∈ succ(v) ⟺ v ∈ pred(w)`. Used in a `debug_assert!` and by tests.
#[must_use]
pub(crate) fn cfg_is_consistent(cfg: &Cfg) -> bool {
    use crate::solve::Graph;
    let n = cfg.node_count();
    let mut counts = BTreeMap::<(usize, usize), i32>::new();
    for v in 0..n {
        for &w in cfg.succ(v) {
            if w >= n {
                return false;
            }
            *counts.entry((v, w)).or_default() += 1;
        }
        for &u in cfg.pred(v) {
            if u >= n {
                return false;
            }
            *counts.entry((u, v)).or_default() -= 1;
        }
    }
    counts.values().all(|&c| c == 0)
}
