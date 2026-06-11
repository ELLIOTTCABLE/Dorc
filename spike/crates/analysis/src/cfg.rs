//! `analysis::cfg` — lower a [`dorc_syntax::Ast`] into a control-flow graph the
//! dataflow framework ([`crate::solve`]) runs over.
//!
//! Design + the why: `Research/notes/163-analysis-engine-design-spa-grounded.md`
//! §3 (CFG construction + the hazard set) and `notes/160-analyzer-chord-synthesis.md`
//! §2 (the hazard set / ⊤-trigger set). This module owns the sh-specific modeling;
//! [`Cfg`] implements the analysis-agnostic [`crate::solve::Graph`] trait so the
//! same worklist solves forward (may-mutate, ambient-gate, `ShellEnvState`) and
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

use std::collections::{BTreeMap, BTreeSet};

use dorc_core::{AstId, BytePos, Carrier, Channel, DiagCode, Diagnostic, Span};
use dorc_syntax::{
    Ast, NodeKind, WordPart,
    ast::{CaseArm, ElseIf, RedirOp, RedirTarget},
};

use crate::lattice::Powerset;

/// Diagnostic codes this module emits (greppable; `ch-catalog`).
const CFG_TOP: DiagCode = DiagCode("cfg-top-node");
const CFG_ERREXIT_TOP: DiagCode = DiagCode("cfg-errexit-unknown");
/// arch-2 (brk-2): a call to an eligible-but-over-budget / refused funcdef stays an
/// ordinary unmodeled command (`Opaque`), but the refusal is surfaced (never silent —
/// proportional degradation, `211` §1). Each variant names *why* the splice was refused.
const CFG_INLINE_REFUSED: DiagCode = DiagCode("cfg-inline-refused");

/// arch-2 inlining budgets (`211` §1 / `209` brk-2; pre-spelled in the round-21 charter).
/// Over-budget ⇒ the call stays `Opaque` WITH a [`CFG_INLINE_REFUSED`] diagnostic naming the
/// exceeded budget — proportional degradation, never a silent cliff.
mod inline_budget {
    /// Maximum inline-splice depth (a call inside an inlined body inside an inlined body is
    /// depth 2; deeper ⇒ refuse). Keeps the recursion-stack shallow and the spliced-node
    /// count bounded.
    pub(super) const MAX_DEPTH: u32 = 2;
    /// Maximum spliced CFG nodes for ONE call site (the body's whole lowering, recursively).
    pub(super) const MAX_NODES_PER_SITE: usize = 64;
    /// Maximum spliced CFG nodes across the WHOLE book (all call sites summed).
    pub(super) const MAX_NODES_PER_BOOK: usize = 1024;
}

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
    /// (`haz-concurrency`): the next subagent's `ShellEnvState` pass pushes a frame
    /// here. env/var/cwd mutations inside DO NOT escape; FS mutations DO.
    ScopeEnter,
    /// Leave a subshell/`$( )` scope: pop the frame, projecting out env/var/cwd
    /// mutations (the inverse-transient). FS effects already escaped.
    ScopeExit,
    /// An absorbing ⊤ node for an `NodeKind::Unsupported` construct
    /// (`inv-top-reject`): un-probeable AND un-skippable. The analyzer must fold
    /// this to ⊤ for its phase, never silently best-effort past it.
    Top,
    /// The structural head of a loop (`for`/`while`/`until`, task-L1): the join point
    /// where the loop's entry edge and its **back-edge** converge — the first real
    /// cyclic CFG the worklist ever sees (`209` brk-1 (a)). Carries the loop's
    /// [`AstId`] so the value-plane can read a `for`'s iteration variable + list words
    /// and bind that variable to the JOIN of the literal words at body entry. Effect-
    /// free and never a plan/apply leaf (the loop *construct* is structural; only the
    /// body's [`Command`](CfgNodeKind::Command) nodes are leaves — `inv-leaf-seam`).
    LoopHead,
}

// The per-leaf consumption vocabulary is `core::Channel` (`inv-one-observable`, `19F`):
// the CFG records, per leaf, which channels the leaf's *context* consumes in a way the
// `true`-stub's default would NOT vouch — an un-collapsed fact (`inv-superposition`): the
// engine names *what is consumed*; the phased caller (`plan`) collapses it. `Effect` is
// vouched by convergence and never enters the set; `Stdout`/`Stderr` are vouched by
// nothing (always block, 16F §3). The status-consumers split by what reproduces the read
// (`206` §3 + arch-1 / note 214: the leaf-exact render retired the render-expressibility
// `StatusRenderFloor`): `StatusRelaxable` (a KNOWN rc reproduces the decision — a &&/||
// operand, an errexit-region command's rc, a `$?`-reader's predecessor, and an `if`/`elif`
// guard), `StatusInvariant` (the consumer decides nothing — `cmd || true`), and
// `StatusIterated` (a `while`/`until` condition — the per-pass SEQUENCE no single rc
// reproduces, an unconditional block). See `core::Channel`.

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
    /// Per-node: lowered inside a command-substitution `$( … )` body (find-cli-1).
    /// Such commands are effect-bearing but NOT plan/apply leaves.
    expansion_internal: Vec<bool>,
    /// Per-node: lowered inside a loop BODY or CONDITION (task-L1, `209` brk-1). A
    /// loop-body leaf is a structural render-floor block this round — the
    /// line-granular render cannot elide a single iteration, so an in-loop leaf never
    /// mints a `Replace`/`Omit` license (`plan::disposition_for`). The recorded floor
    /// the member-elision slice (`209` brk-1 (b)) later lifts.
    in_loop: Vec<bool>,
    /// Per-node: lowered as part of a function-body SPLICE at a call site (arch-2, brk-2).
    /// Such a `Command` is effect-bearing (its mutations gen into reaching-defs exactly as
    /// inline code would — `i-5`), reachable from entry (un-detaching the body — the find-7
    /// fix), and probe-resolvable as a sub-record OF THE CALL (`site N.M`, `i-4`); but it is
    /// NOT a plan/apply Step LEAF of its own (`i-3`): its source span belongs to the SHARED
    /// definition, which every call site reuses, so editing it for one call would rewrite the
    /// others (and the definition). The render/substitution unit is the CALL leaf; a spliced
    /// body site is excluded from the leaf list exactly like an expansion-internal `$()` body
    /// command. (`spliced_internal ⊆ ¬is_expansion_internal`: a `$()` inside a spliced body is
    /// flagged BOTH; both exclude it from the leaf set.)
    spliced_internal: Vec<bool>,
    /// arch-2 (`i-1`/`i-3`/`i-4`): a CALL [`CfgNodeKind::Command`] node → the ordered list of
    /// body LEAF [`Command`](CfgNodeKind::Command) CFG nodes the splice produced for it (its
    /// own spliced copy; a transitively-inlined body's leaves are flattened in here too — the
    /// call's WHOLE effect-bearing leaf set). Empty for a non-call command. The all-or-nothing
    /// CALL license (`plan`) aggregates these sites' classifications; the probe emits one
    /// `site N.M` sub-record per body site (M = the index into this list). `BTreeMap` for
    /// `inv-determinism`.
    call_body_sites: BTreeMap<CfgNodeId, Vec<CfgNodeId>>,
    /// Per-node: the unvouched output observables this node's *context* consumes
    /// (note 16J, `inv-superposition`). Computed during lowering — the single
    /// exhaustive structural traversal — so it is **total over nodes**: an empty
    /// set means "visited, nothing consumes it" (provably quiet), never "not
    /// examined". The phased caller wraps it `May<_>` and collapses it.
    consumed: Vec<Powerset<Channel>>,
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

    /// Was this node lowered inside a command-substitution `$( … )` body? Such
    /// commands run during word expansion — they are effect-bearing (kept in the
    /// dataflow) but are NOT plan/apply leaves (find-cli-1, the dn-3 leaf-seam).
    /// Subshell `( )` and group `{ }` body commands are real leaves (not flagged).
    #[must_use]
    pub fn is_expansion_internal(&self, id: CfgNodeId) -> bool {
        self.expansion_internal[id.index()]
    }

    /// Was this node lowered inside a loop body or condition (task-L1, `209` brk-1)?
    /// An in-loop leaf is the structural render-floor block: the line-granular render
    /// cannot substitute a single iteration, so `plan` never mints a license for it
    /// (`MustRun`-floor this round; the member-elision slice lifts it). A POST-loop
    /// leaf is NOT in-loop, so the value below a converged loop unlocks normally.
    #[must_use]
    pub fn in_loop_body(&self, id: CfgNodeId) -> bool {
        self.in_loop[id.index()]
    }

    /// Was this node lowered as part of a function-body SPLICE at a call site (arch-2,
    /// brk-2)? A spliced body `Command` is effect-bearing and probe-resolvable (a `site N.M`
    /// sub-record OF THE CALL), but is NOT a plan/apply Step LEAF of its own — its span
    /// belongs to the shared definition, so the CALL leaf is the render/substitution unit
    /// (`i-3`). Excluded from the leaf set exactly like [`is_expansion_internal`]. A POST-call
    /// command is NOT spliced-internal, so the value below an eliding call unlocks normally.
    #[must_use]
    pub fn is_spliced_internal(&self, id: CfgNodeId) -> bool {
        self.spliced_internal[id.index()]
    }

    /// The ordered body LEAF [`CfgNodeKind::Command`] nodes a function-call splice produced for
    /// the CALL node `id` (arch-2, brk-2; `i-3`/`i-4`), or `None` if `id` is not an inlined
    /// call. The all-or-nothing CALL license (`plan`) aggregates these sites' classifications;
    /// the probe ships one `site N.M` sub-record per entry (M = the list index). The list is
    /// the call's WHOLE effect-bearing-and-probeable leaf set (a transitively-inlined body's
    /// own leaves are flattened in, depth-bounded). In source order (`inv-determinism`).
    #[must_use]
    pub fn call_body_sites(&self, id: CfgNodeId) -> Option<&[CfgNodeId]> {
        self.call_body_sites.get(&id).map(Vec::as_slice)
    }

    /// Every inlined-CALL node paired with its body-leaf list (arch-2; the whole splice map).
    /// Deterministic key order (`BTreeMap`). Consumers: `effect::classify` (builds the call's
    /// aggregate `SkipClass`), `plan::compile_probe`/`build_plan` (per-call sub-records + the
    /// CALL Step). A call with an EMPTY body-leaf list (a wrapper of pure builtins only) is
    /// still present — its call elides trivially (nothing effect-bearing to gate).
    pub fn inlined_calls(&self) -> impl Iterator<Item = (CfgNodeId, &[CfgNodeId])> {
        self.call_body_sites
            .iter()
            .map(|(&id, sites)| (id, sites.as_slice()))
    }

    /// The unvouched output observables (`Stdout`/`Stderr`) this node's context
    /// consumes — the **un-collapsed** consumption fact (`inv-superposition`, note
    /// 16J). Empty ⇒ provably quiet (the lowering is the single total traversal, so
    /// "empty" means examined-and-quiet, never un-examined). The phased caller
    /// (`plan`) wraps this in `May<_>` and collapses it; per `inv-must-may` a `May`
    /// consumption can only **block** a replacement, never license one.
    #[must_use]
    pub fn consumed_observables(&self, id: CfgNodeId) -> &Powerset<Channel> {
        &self.consumed[id.index()]
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

    // Phase 2: errexit failure-edges (the haz-seterr coupling) — and, as a by-product,
    // errexit-region commands' rc is marked `StatusRelaxable`-consumed (19A C-3 / 205 §2).
    b.materialise_errexit_edges();

    // Phase 3: `$?`-readers mark their CFG-predecessor command(s)' rc consumed (C-3's
    // second consumer). Runs after the structural walk so `pred` is populated; the
    // errexit edges added in phase 2 do not change a command's predecessors.
    b.mark_dollar_question_predecessors();

    b.finish()
}

/// `errexit` abstract value — a small lattice: `Bottom` (⊥: no information /
/// unreached) below the two **incomparable** atoms `Off`/`On`, below `Top` (⊤).
/// `Off` and `On` do NOT compare (a path that is explicitly off and one that is on
/// genuinely conflict), so their join is ⊤ — which is exactly why merges must be
/// seeded ⊥, not `Off` (else `Off(seed) ⊔ On = ⊤` spuriously). Kept local (the
/// builder's pass is hand-rolled, not run through `solve`, so `build` stays
/// self-contained).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ErrExit {
    /// No information yet — the join identity and the init for non-entry nodes.
    /// Never itself materialises a failure-edge (an unreached node cannot abort).
    Bottom,
    Off,
    On,
    /// `set "$dyn"` or a join of On and Off — don't know ⇒ over-approximate
    /// (assume the failure-edge may exist).
    Top,
}

impl ErrExit {
    /// Join two errexit facts (the forward pass merges at every predecessor).
    #[expect(
        clippy::match_same_arms,
        reason = "On⊔Off=Top is kept a distinct, commented arm from Top-absorption: same value, different lattice reason"
    )]
    fn join(self, other: ErrExit) -> ErrExit {
        match (self, other) {
            (a, b) if a == b => a,
            // ⊥ is the identity (so `⊥ ⊔ On = On`, not ⊤ — the merge-seed fix).
            (ErrExit::Bottom, x) | (x, ErrExit::Bottom) => x,
            (ErrExit::Top, _) | (_, ErrExit::Top) => ErrExit::Top,
            // On ⊔ Off — genuine disagreement ⇒ ⊤ (may be on; add the edge).
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
    /// Per-node: is this command lowered *inside* a command-substitution `$( … )`
    /// body? Those run during word expansion, not as standalone leaves, so they
    /// stay in the effect dataflow but are excluded from the plan/apply leaf set
    /// (find-cli-1 / dn-3). Subshell `( )` and group `{ }` bodies are NOT marked —
    /// their commands are real leaves.
    expansion_internal: Vec<bool>,
    /// Per-node: lowered inside a loop body/condition (task-L1). Marked by an
    /// arena-range pass in `lower_for`/`lower_while` (the same range trick
    /// `expansion_internal` uses); emitted on the [`Cfg`] for `plan`'s in-loop floor.
    in_loop: Vec<bool>,
    /// Per-node: lowered as part of a function-body splice (arch-2). Set by an arena-range
    /// pass in [`splice_funcdef_body`](Builder::splice_funcdef_body); emitted on the [`Cfg`]
    /// so the leaf set excludes a spliced body command (the CALL is the render unit, `i-3`).
    spliced_internal: Vec<bool>,
    /// arch-2: the funcdef registry — name ↦ each definition's `(body AstId, def-start
    /// BytePos)`, in source order. Built once in [`new`](Builder::new) over the AST. A name
    /// with `len() > 1` is REDEFINED ⇒ every call to it ⊤-rejects (out of slice, `i-1`); a
    /// call resolves to the LAST definition strictly BEFORE the call site (so a call before any
    /// definition stays an ordinary unmodeled command). `BTreeMap` for `inv-determinism`.
    funcdefs: BTreeMap<String, Vec<(AstId, BytePos)>>,
    /// arch-2: the ACTIVE inline stack — the body-AstIds currently being spliced (call-stack
    /// order). A call whose resolved body is already on this stack is direct/transitive
    /// RECURSION ⇒ ⊤-reject naming the cycle (`i-1`). Pushed before splicing a body, popped
    /// after.
    inline_stack: Vec<AstId>,
    /// arch-2: spliced CFG nodes minted across the WHOLE book so far (the per-book budget,
    /// `i-1`). Each splice adds its node count; a splice that would exceed
    /// [`inline_budget::MAX_NODES_PER_BOOK`] is refused (the call stays `Opaque`).
    spliced_node_total: usize,
    /// arch-2: the CALL node → its ordered body-leaf list, accumulated as splices complete
    /// (`i-3`/`i-4`). Emitted on the [`Cfg`] as [`Cfg::call_body_sites`].
    call_body_sites: BTreeMap<CfgNodeId, Vec<CfgNodeId>>,
    /// Per-node: the unvouched output observables this node's context consumes
    /// (note 16J). Populated in lowering by `mark_consumed_range` (the enclosing
    /// pipeline-stage / redirected-group context propagated to inner leaves, the
    /// same arena-range trick `expansion_internal` uses). Emitted on the [`Cfg`]
    /// un-collapsed (`inv-superposition`).
    consumed: Vec<Powerset<Channel>>,
    /// `ScopeExit` node → its matching `ScopeEnter` (find-4): the errexit forward
    /// pass restores the *pre-subshell* state at the exit, so a `set -e`/`set +e`
    /// toggle inside `( )`/`$( )` never leaks out. Both directions are kept so the
    /// worklist can re-queue the exit when its enter's inflow changes (keeping the
    /// fixed point correct despite there being no enter→exit control edge).
    /// `BTreeMap` (not `HashMap`) for `inv-determinism`.
    exit_to_enter: BTreeMap<usize, usize>,
    enter_to_exit: BTreeMap<usize, usize>,
    /// A `while`/`until` loop's exit `Merge` node → its BODY-exit node (20O find-6a /
    /// task-L2 item-6a). dash's post-loop `$?` is the BODY's last command rc (when the
    /// loop ran ≥1) or 0 (ran 0 times) — NEVER the condition's rc. But a `while` loop's
    /// only control edge into the post-loop flow is `cond_exit → merge`, so a backward
    /// `$?`-predecessor walk from a post-loop reader stops at the CONDITION command and
    /// never reaches the body. This map lets [`mark_dollar_question_predecessors`] also
    /// reach the body-exit when the walk passes the loop's exit `merge`, so the body's
    /// last command is correctly marked `StatusRelaxable`-consumed. (A `for` loop needs
    /// no entry here: its exit edge is `head → merge`, and `head`'s back-edge pred IS the
    /// body-exit, so the walk reaches it already — `temp`-verified, item-6a "for verified
    /// correct, leave".) `BTreeMap` for `inv-determinism`.
    while_exit_to_body: BTreeMap<usize, usize>,
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
            expansion_internal: Vec::new(),
            in_loop: Vec::new(),
            spliced_internal: Vec::new(),
            funcdefs: collect_funcdefs(ast),
            inline_stack: Vec::new(),
            spliced_node_total: 0,
            call_body_sites: BTreeMap::new(),
            consumed: Vec::new(),
            exit_to_enter: BTreeMap::new(),
            enter_to_exit: BTreeMap::new(),
            while_exit_to_body: BTreeMap::new(),
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
        self.expansion_internal.push(false);
        self.in_loop.push(false);
        self.spliced_internal.push(false);
        self.consumed.push(Powerset::default());
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
                let body_start = self.nodes.len();
                let after_body = self.lower_node(body, entry_pred);
                // A group-level output redirect captures the body's output ⇒ every
                // inner leaf consumes it (note 16J / 16G kill-shot: `{ install; } > f`).
                let obs = output_redir_observables(self.ast, &redirs);
                self.mark_consumed_range(body_start, self.nodes.len(), &obs);
                self.attach_redirs(id, &redirs, after_body)
            }
            NodeKind::FuncDef { body, .. } => self.lower_funcdef(id, *body, entry_pred),
            NodeKind::ForLoop { body, .. } => {
                let body = *body;
                self.lower_for(id, body, entry_pred)
            }
            NodeKind::WhileLoop { cond, body, .. } => {
                let (cond, body) = (*cond, *body);
                self.lower_while(id, cond, body, entry_pred)
            }
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

        // Output-consumption (note 16J): this command's OWN redirections that
        // capture fd 1/2 to a real (non-`/dev/null`) sink consume that observable.
        // Enclosing pipeline-stage / redirected-group context is marked by the
        // callers (`mark_consumed_range`); the leaf-local part is here.
        let own = output_redir_observables(self.ast, redirs);
        self.mark_consumed_range(cmd.index(), cmd.index() + 1, &own);

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

        // arch-2 (brk-2): if this command's word resolves to a funcdef DEFINED EARLIER in
        // this file, SPLICE a fresh lowering of the body's AST right after the CALL node, so
        // the body becomes reachable (un-detaching it — the find-7 fix) and its effects flow
        // downstream exactly as inline code would (`i-1`/`i-5`). The CALL `cmd` node stays the
        // render/substitution LEAF (`i-3`); the spliced body commands are `spliced_internal`
        // (not Step leaves) and become the call's per-site probe sub-records (`site N.M`,
        // `i-4`). Eligibility (`i-1`) is strict + every exclusion loud; a non-eligible call
        // stays an ordinary unmodeled command (`Opaque`, status quo). The argv (for positional
        // binding) is resolved by the value plane (`i-2`), not here.
        if let Some(call_exit) = self.try_inline_call(id, words, cmd) {
            return call_exit;
        }
        cmd
    }

    /// arch-2 (brk-2): try to splice a function-body inline at a call site. Returns
    /// `Some(region-exit)` when the call was inlined (the body spliced after `cmd`, the body
    /// leaves recorded against `cmd` for the plan/probe), or `None` when the command is not an
    /// eligible call (it stays an ordinary unmodeled command — `Opaque`, status quo). Every
    /// refusal is loud (a [`CFG_INLINE_REFUSED`] diagnostic) so the proportional degradation
    /// is never silent (`211` §1). `i-1` eligibility, in order:
    ///
    /// * the command word is a fixed literal that names a funcdef with a definition strictly
    ///   BEFORE this call (a call before any definition, or a non-funcdef word, ⇒ `None`: it
    ///   might be a PATH binary — ordinary `Opaque`, no diagnostic);
    /// * the name is defined exactly ONCE (`> 1` ⇒ ⊤-reject with a specific diagnostic —
    ///   redefinition tracking is out of slice);
    /// * the resolved body is not already on the active inline stack (direct/transitive
    ///   RECURSION ⇒ ⊤-reject naming the cycle);
    /// * inline depth `≤ MAX_DEPTH` (the stack depth);
    /// * the body uses none of `$@`/`$*`/`shift`/`local` (out of slice — the diagnostic names
    ///   the construct);
    /// * no body WRITE-redirect (`>`/`>>`/`<>`) to anything but `/dev/null` (tc-M2: a body
    ///   file-write is an unmodeled effect (y-1) inlining would EXPOSE as wrong-ambience);
    /// * the splice fits the node budgets (`≤ MAX_NODES_PER_SITE` for this site, and the
    ///   running total `≤ MAX_NODES_PER_BOOK`) — over-budget ⇒ `Opaque` WITH a diagnostic
    ///   naming the exceeded budget.
    fn try_inline_call(&mut self, id: AstId, words: &[AstId], cmd: CfgNodeId) -> Option<CfgNodeId> {
        // The command word must be a fixed literal naming a defined function. A ⊤/expansion
        // word, or a name no funcdef declares, is an ordinary unmodeled command (no diagnostic
        // — it might be a PATH binary; `i-1`).
        let name = self.word_literal(*words.first()?)?;
        let defs = self.funcdefs.get(name)?;

        let call_lo = self.span(id).lo;
        // A name DEFINED MORE THAN ONCE ⇒ every call ⊤-rejects (redefinition tracking is out of
        // slice). A specific, loud diagnostic; the call stays Opaque.
        if defs.len() > 1 {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "function `{name}` is defined more than once; the call is not inlined \
                     (redefinition tracking is out of the modeled subset) — it runs as an \
                     ordinary unmodeled command"
                ),
            ));
            return None;
        }
        // Resolve to the LAST definition strictly BEFORE the call. A definition AFTER the call
        // (or none) ⇒ not a same-file-earlier call ⇒ ordinary unmodeled command (no diagnostic:
        // a forward-call might legitimately be a PATH binary at that program point).
        let body = defs
            .iter()
            .filter(|(_, def_lo)| def_lo.0 < call_lo.0)
            .max_by_key(|(_, def_lo)| def_lo.0)
            .map(|(body, _)| *body)?;

        // RECURSION (direct or transitive within the active inline stack) ⇒ ⊤-reject naming the
        // cycle. The call stays Opaque (the poison stays — nothing downstream wrongly elides).
        if self.inline_stack.contains(&body) {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "recursive call to `{name}` (direct or transitive within the active inline \
                     stack); not inlined — it runs as an ordinary unmodeled command"
                ),
            ));
            return None;
        }
        // Inline DEPTH budget (`i-1`): the active stack is the inline depth so far.
        if self.inline_stack.len() as u32 >= inline_budget::MAX_DEPTH {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "call to `{name}` exceeds the inline-depth budget ({}); not inlined — it \
                     runs as an ordinary unmodeled command",
                    inline_budget::MAX_DEPTH
                ),
            ));
            return None;
        }
        // Out-of-slice body constructs (`i-1`): `$@`/`$*`/`shift`/`local` (the positional-array
        // and dynamic-scope forms the value plane does not model). The diagnostic names the
        // construct.
        if let Some(construct) = self.body_uses_unmodeled_positional(body) {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "call to `{name}` not inlined: its body uses `{construct}` (out of the \
                     modeled subset) — it runs as an ordinary unmodeled command"
                ),
            ));
            return None;
        }
        // tc-M2 weld (the r1A capability-matrix crosscheck): a body containing a WRITE-shaped
        // redirect (`>`/`>>`/`<>`) to anything but `/dev/null` ⇒ refuse. Inlining alone would
        // EXPOSE an invisible body file-write as wrong-ambience (redirect-effects are unmodeled,
        // y-1). `>/dev/null`, `2>&1`, fd-dup forms stay exempt (the devnull-exemption vocabulary).
        if let Some(detail) = self.body_has_unmodeled_write_redirect(body) {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "call to `{name}` not inlined: its body has an unmodeled write-redirect \
                     ({detail}) — it runs as an ordinary unmodeled command (tc-M2)"
                ),
            ));
            return None;
        }

        // Eligible. Splice the body after the CALL node, depth-bounding the node count.
        self.splice_funcdef_body(id, name, body, cmd)
    }

    /// arch-2: splice a fresh lowering of `body` (the funcdef body's AST) right after the CALL
    /// node `cmd`, returning the spliced region's exit (so the caller sequences the
    /// continuation onto the BODY's last node — POSIX: the call's continuation follows the
    /// body, and the call's rc is the body's last rc, `i-5`). Records the body's effect-bearing
    /// LEAF [`Command`](CfgNodeKind::Command) nodes against `cmd` (`call_body_sites`, for the
    /// `plan` aggregate + `site N.M` probe records, `i-3`/`i-4`) and marks every spliced node
    /// `spliced_internal` (excluded from the Step leaf set; the CALL is the render unit).
    ///
    /// Budgets (`i-1`, proportional degradation): the per-site budget is checked from the
    /// body's AST-subtree node count (a conservative pre-estimate — AST descendants ≥ the CFG
    /// leaf nodes the body lowers to; over-estimating is the safe refuse direction), and the
    /// per-book budget from the running tally of actually-spliced nodes. Over either ⇒ refuse
    /// WITH a diagnostic naming the exceeded budget; the call stays `Opaque` (no splice happens
    /// — the estimate is checked BEFORE any node is allocated, so there is nothing to roll
    /// back). `inv-determinism`: the arena is append-only and the body-leaf collection is in
    /// arena (== source) order.
    fn splice_funcdef_body(
        &mut self,
        id: AstId,
        name: &str,
        body: AstId,
        cmd: CfgNodeId,
    ) -> Option<CfgNodeId> {
        // Per-site budget: a conservative AST-subtree estimate (never under-counts the CFG
        // leaves a body lowers to). Checked BEFORE allocation so refusal needs no rollback.
        let estimate = subtree_node_count(self.ast, body);
        if estimate > inline_budget::MAX_NODES_PER_SITE {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "call to `{name}` exceeds the per-call inline-node budget ({} > {}); not \
                     inlined — it runs as an ordinary unmodeled command",
                    estimate,
                    inline_budget::MAX_NODES_PER_SITE
                ),
            ));
            return None;
        }
        // Per-book budget: the running tally of actually-spliced nodes, plus this body's
        // estimate, must not exceed the whole-book cap. (A transitively-inlined inner call adds
        // to the tally when ITS splice runs, so this composes across depths.)
        if self.spliced_node_total.saturating_add(estimate) > inline_budget::MAX_NODES_PER_BOOK {
            self.diags.push(Diagnostic::warning(
                CFG_INLINE_REFUSED,
                Some(self.span(id)),
                format!(
                    "call to `{name}` exceeds the per-book inline-node budget ({} spliced + {} \
                     more > {}); not inlined — it runs as an ordinary unmodeled command",
                    self.spliced_node_total,
                    estimate,
                    inline_budget::MAX_NODES_PER_BOOK
                ),
            ));
            return None;
        }

        // Splice the body. Push the body onto the active inline stack so a recursive call
        // INSIDE the body ⊤-rejects (the cycle guard, `i-1`); a deeper eligible call splices
        // (depth-bounded). `lower_node` re-enters `lower_simple` for body commands, so a
        // transitively-inlined call recurses through here naturally.
        let body_start = self.nodes.len();
        self.inline_stack.push(body);
        let body_exit = self.lower_node(body, cmd);
        self.inline_stack.pop();
        let body_end = self.nodes.len();

        // Mark every spliced node `spliced_internal` (the CALL is the render unit; a body leaf
        // is never an independent Step, `i-3`). A nested `$()` inside the body is already
        // `expansion_internal`; flagging it `spliced_internal` too is harmless (both exclude
        // it from the leaf set).
        for v in body_start..body_end {
            self.spliced_internal[v] = true;
        }
        self.spliced_node_total = self
            .spliced_node_total
            .saturating_add(body_end - body_start);

        // Collect the body's effect-bearing/probeable LEAF Command nodes (the call's per-site
        // sub-record set, `i-4`): a spliced `Command` that is NOT itself an inner inlined CALL
        // (an inner call's OWN body leaves are the call's sites; the inner call node is a
        // pass-through aggregate, not a leaf — flatten its leaves in instead) and is NOT a
        // `$()`-internal non-leaf. In arena (source) order (`inv-determinism`).
        let mut sites: Vec<CfgNodeId> = Vec::new();
        for v in body_start..body_end {
            let node = CfgNodeId(v as u32);
            if self.nodes[v].kind != CfgNodeKind::Command || self.expansion_internal[v] {
                continue;
            }
            // An inner inlined CALL node is itself NOT a site — its body leaves are already in
            // `call_body_sites[inner]` and were appended to `sites` via the flatten below. The
            // inner call node has its OWN body-site entry, so skip it here to avoid double
            // counting (a body command that is a plain mutator IS a site).
            if let Some(inner_sites) = self.call_body_sites.get(&node) {
                sites.extend(inner_sites.iter().copied());
            } else {
                sites.push(node);
            }
        }
        self.call_body_sites.insert(cmd, sites);
        Some(body_exit)
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
            let stage_start = self.nodes.len();
            cur = self.lower_node(stage, cur);
            if i != last {
                // A non-last stage's stdout is piped into the next ⇒ consumed (note
                // 16J). Mark every command in the stage's region: the stage may be a
                // `( … )`/`{ … }` whose inner leaf is the real producer (16G
                // kill-shot, `( install ) | grep`), not just `cur`.
                self.mark_consumed_range(
                    stage_start,
                    self.nodes.len(),
                    &Powerset::singleton(Channel::Stdout),
                );
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
        op: dorc_syntax::ast::AndOrOp,
        left: AstId,
        right: AstId,
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        // A `&&`/`||` left operand's status is **branch-consumed** (the right operand's
        // reachability turns on it). `lower_condition_region(_, None)` clears its
        // errexit-fallibility (a `&&`/`||` left operand is errexit-exempt) but marks NO
        // channel itself; this site then marks `Channel::StatusRelaxable` (a KNOWN rc
        // reproduces the operand's branch decision). The phased caller
        // (`plan`) collapses it **rc-conditionally** (`prove_replaceable`): an undeclared
        // rc blocks the license (eliding to a fabricated `true`/rc-0 would suppress a
        // `cmd || fallback` — the `kFAIL-perform` under-execute the round-19 adversarial
        // trace proved); a *declared* rc relaxes it (the value-preserving stand-in
        // reproduces the exact status, so `install && start`'s rc-0 post-condition stays
        // replaceable, and `useradd[9] || mkdir` keeps `mkdir` live). This dissolves the
        // former `tc-mint` ambiguity (`notes/198` §1.3): the engine no longer guesses
        // post-condition-vs-guard — it emits the un-collapsed status fact, and the
        // *declared rc* (the fold's input) decides, per `inv-superposition`/`19A §5`.
        //
        // door-3 (charter `20V` §4 / note 213): `lhs || true` marks the left operand
        // `Channel::StatusInvariant` (never blocks, even at ⊤) instead of `StatusRelaxable` —
        // the rc is consumed-in-form, dead-in-fact (see the variant doc for why). Only the
        // EXACT bare-`true` rhs qualifies ([`right_is_bare_true`]); `|| :`/`|| false`/
        // `&& true`/`|| true >/dev/null`/`|| { …; }` keep `StatusRelaxable` (a deliberately
        // narrow license-widening, `20V` §4 d-2). The mark covers the whole left arena range;
        // in a chain `a || b || true` (left-assoc) that range also spans `a`, but `a` already
        // carries `StatusRelaxable` from the inner `||` (its rc gates whether `b` runs), and
        // that blocking mark wins over the inert Invariant over-mark — so only `b` (the direct
        // `|| true` left) unlocks. The d-3 asymmetry falls out of mark-union composition.
        let door3 = matches!(op, dorc_syntax::ast::AndOrOp::Or) && self.right_is_bare_true(right);
        let left_status = if door3 {
            Channel::StatusInvariant
        } else {
            Channel::StatusRelaxable
        };
        let before_left = self.nodes.len();
        let left_exit = self.lower_condition_region(left, entry_pred, None);
        self.mark_consumed_range(
            before_left,
            self.nodes.len(),
            &Powerset::singleton(left_status),
        );

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
        // `if false; true; then …` runs the body, no abort; note 166). It is also an
        // UNAMBIGUOUS guard — a different branch runs on the rc — so its commands' status is
        // branch-consumed `Channel::StatusRelaxable` (arch-1 / note 214: the leaf-exact
        // render retired the `StatusRenderFloor` block, so an if/elif guard is an ordinary
        // single-shot substitution site — a probe-sourced KNOWN rc reproduces its branch
        // decision; ⊤ blocks, the `19D`/`kFAIL-perform` floor).
        let cond_exit =
            self.lower_condition_region(cond, entry_pred, Some(Channel::StatusRelaxable));

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
    /// `ScopeEnter` → body → `ScopeExit`; the next subagent's `ShellEnvState` pass
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
        let body_start = self.nodes.len();
        let body_exit = self.lower_node(body, enter);
        // A subshell-level output redirect captures the body's output ⇒ every inner
        // leaf consumes it (note 16J / 16G kill-shot: `( install ) > f`).
        let obs = output_redir_observables(self.ast, redirs);
        self.mark_consumed_range(body_start, self.nodes.len(), &obs);
        let leave = self.fresh(id, CfgNodeKind::ScopeExit);
        self.add_edge(body_exit, leave);
        self.pair_scope(enter, leave);
        self.attach_redirs(id, redirs, leave)
    }

    /// A function DEFINITION's body is a **detached** sub-CFG (Tier-A is intraprocedural —
    /// note 163 §5): it is built (so its nodes/effects exist) but NOT wired into the main flow.
    /// Defining a function has no runtime effect; the body runs only when CALLED, and a call is
    /// now a CFG-level body SPLICE (arch-2, brk-2 — [`try_inline_call`](Builder::try_inline_call)).
    ///
    /// The detached definition body's command nodes are marked `spliced_internal` (`i-3`): a
    /// definition's body commands are NEVER independent plan/apply Step LEAVES — the body text
    /// renders verbatim inside the `name() { … }` definition, and its effects reach the analysis
    /// only via the per-call splices. Pre-arch-2 these detached commands surfaced as
    /// `MustRun`/`skip-unresolvable` leaves of their own (unreachable, harmless, but noisy and
    /// double-counting the spliced copy); marking them non-leaf collapses the definition to its
    /// proper "no runnable leaf of its own" shape.
    ///
    /// errexit residue (the old find-7 TODO): the detached body's `Entry` is pred-less, so the
    /// forward errexit pass computes `Off` throughout and its commands get no failure-edges.
    /// This is now MOOT for the body's runtime semantics — the SPLICED copy at each call site
    /// gets its errexit inflow from the call (it is wired in, `i-5`), and the detached copy is
    /// a non-leaf island that never runs. The detached body remains only so its nodes exist for
    /// arena consistency; it is not consulted for any apply/probe decision.
    fn lower_funcdef(&mut self, id: AstId, body: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        // Detached region: its own entry, unreferenced by `entry_pred`.
        let func_entry = self.fresh(body, CfgNodeKind::Entry);
        let body_start = self.nodes.len();
        let body_exit = self.lower_node(body, func_entry);
        // The definition's body commands are not independent leaves (`i-3`): the body renders
        // inside the `name() { … }` definition and runs only via the per-call splices.
        for v in body_start..self.nodes.len() {
            self.spliced_internal[v] = true;
        }
        let func_exit = self.fresh(body, CfgNodeKind::Exit);
        self.add_edge(body_exit, func_exit);

        // The *definition statement* is a no-op pass-through in the main flow.
        let m = self.fresh(id, CfgNodeKind::Merge);
        self.add_edge(entry_pred, m);
        m
    }

    /// `for NAME in WORD…; do body; done` (task-L1, `209` brk-1). The control-flow:
    ///
    /// ```text
    ///   entry_pred ─► head(LoopHead) ─► body ─► body_exit ─┐
    ///                   ▲   │                              │  (back-edge)
    ///                   └───┼──────────────────────────────┘
    ///                       └─► merge (exit: list exhausted / ran 0 times)
    /// ```
    ///
    /// The [`CfgNodeKind::LoopHead`] is the join of the entry edge and the **back-edge**
    /// — the first real cyclic CFG the worklist sees (the back-edge join is what makes
    /// a body reassignment reach the next iteration, and what lets "ran 0 times" fall
    /// straight to `merge`). The list WORDS are pure expansion (any `$(…)`/arith in them
    /// ⊤-rejected at parse), so they mint no CFG node; the value-plane reads the
    /// iteration variable + words off this node's `ForLoop` AST and binds the variable
    /// to the JOIN of the words at body entry. Loops do NOT create a subshell scope —
    /// body assignments persist (`{ }`-like, item-2(c)) — so there is NO `ScopeEnter`/
    /// `Exit`. Body commands are real leaves (NOT expansion-internal); they are marked
    /// **in-loop** so `plan` floors them to run this round (`Cfg::in_loop_body`).
    fn lower_for(&mut self, id: AstId, body: AstId, entry_pred: CfgNodeId) -> CfgNodeId {
        let head = self.fresh(id, CfgNodeKind::LoopHead);
        self.add_edge(entry_pred, head);
        let body_start = self.nodes.len();
        let body_exit = self.lower_node(body, head);
        self.add_edge(body_exit, head); // the back-edge
        self.mark_in_loop_range(body_start, self.nodes.len());
        let merge = self.fresh(id, CfgNodeKind::Merge);
        self.add_edge(head, merge); // exit edge (list exhausted, or ran zero times)
        merge
    }

    /// `while LIST; do body; done` / `until LIST; do body; done` (task-L1). The
    /// control-flow mirrors [`lower_for`] but with a real CONDITION region between the
    /// head and the body:
    ///
    /// ```text
    ///   entry_pred ─► head ─► [cond] ─► cond_exit ─► body ─► body_exit ─┐
    ///                   ▲                   │                            │
    ///                   └───────────────────┼────────────────────────────┘ (back-edge)
    ///                                        └─► merge (cond ends the loop)
    /// ```
    ///
    /// dash-fidelity (analysis/CLAUDE.md T9 / item-2(a)): a failing command in the
    /// `while`/`until` CONDITION region does NOT abort under `set -e` (the same
    /// errexit-exemption as an `if`/`elif` test — extended here via
    /// [`lower_condition_region`]); a failing BODY command DOES abort (its failure-edge
    /// is materialised in phase 2, and it is `StatusRelaxable`-consumed per C-3). The
    /// condition CONSUMES its last command's status (it decides whether the body or the exit
    /// runs) as `Channel::StatusIterated` (arch-1 / note 214): a loop condition is
    /// re-evaluated per pass, so its consumed value is a SEQUENCE no single predicted rc can
    /// reproduce, and replacing it with a constant is an infinite/zero-iteration disaster —
    /// so it blocks unconditionally, the honest successor to the retired `StatusRenderFloor`
    /// (keyed on iteration, not the line-granular render the leaf-exact render replaced).
    /// `until` vs `while` only flips the runtime continuation sense, not the CFG shape
    /// (both a body-entry and an exit edge exist either way); the value/effect planes
    /// are continuation-sense-agnostic, so one lowering serves both.
    fn lower_while(
        &mut self,
        id: AstId,
        cond: AstId,
        body: AstId,
        entry_pred: CfgNodeId,
    ) -> CfgNodeId {
        let head = self.fresh(id, CfgNodeKind::LoopHead);
        self.add_edge(entry_pred, head);
        let loop_start = self.nodes.len();
        // The condition is an errexit-exempt guard whose status is consumed per-iteration —
        // `Channel::StatusIterated` (arch-1): a single predicted rc can never reproduce the
        // condition's per-pass sequence, and a constant-substituted loop condition is an
        // infinite/zero-iteration disaster, so it blocks unconditionally. (NB: `loop_start`
        // is captured BEFORE the condition is lowered, so the condition nodes are ALSO
        // flagged `in_loop` below — the structural floor independently forces them to run;
        // this mark records the honest *reason* regardless.)
        let cond_exit = self.lower_condition_region(cond, head, Some(Channel::StatusIterated));
        let body_exit = self.lower_node(body, cond_exit);
        self.add_edge(body_exit, head); // the back-edge
        self.mark_in_loop_range(loop_start, self.nodes.len());
        let merge = self.fresh(id, CfgNodeKind::Merge);
        self.add_edge(cond_exit, merge); // exit edge (condition ends the loop)
        // item-6a (20O find-6a): record this loop's body-exit against its exit `merge`, so
        // a post-loop `$?`-predecessor walk reaches the BODY (dash's post-loop `$?` source),
        // not just the condition the `cond_exit → merge` edge leads back to.
        self.while_exit_to_body
            .insert(merge.index(), body_exit.index());
        merge
    }

    /// Mark every node in the half-open arena range `[from, to)` as lowered inside a
    /// loop (task-L1). Mirrors the `expansion_internal` / `mark_consumed_range` range
    /// idiom (`inv-determinism` makes the range stable). A NESTED loop's nodes fall
    /// inside the outer range too — correct (they are in *a* loop), and a nested
    /// loop's own call marks them again (idempotent).
    fn mark_in_loop_range(&mut self, from: usize, to: usize) {
        for v in from..to {
            self.in_loop[v] = true;
        }
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
            // Commands lowered inside this `$( … )` body run during word expansion,
            // so they are NOT plan/apply leaves (find-cli-1) — mark the whole body
            // range (covering nested substitutions too). They remain in the effect
            // dataflow; only their leaf-status changes.
            let body_start = self.nodes.len();
            let body_exit = self.lower_node(body, enter);
            for v in body_start..self.nodes.len() {
                self.expansion_internal[v] = true;
            }
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
        // pre-subshell state at a `ScopeExit` (find-4). Non-entry nodes init to ⊥
        // (no-info, the join identity) so a merge of an On path with an as-yet-
        // unreached path yields On, not a spurious ⊤; only the program entry seeds
        // `Off` (a script starts with errexit off until `set -e`), so a genuine
        // `set +e` vs `set -e` split still joins to ⊤.
        let mut before = vec![ErrExit::Bottom; n];
        before[self.entry.index()] = ErrExit::Off;
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
                if let Some(&leave) = self.enter_to_exit.get(&v)
                    && !queued[leave]
                {
                    queued[leave] = true;
                    work.push(leave);
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
            // 19A C-3 / 205 §2: a command errexit might abort on is a status-consumer
            // (`set -e` reads every rc; non-zero ⇒ abort). Mark it the value-relaxing
            // `StatusRelaxable` — a known/probe-sourced rc still folds/substitutes exactly,
            // but a ⊤ rc (every mutator under `fork-mutator-rc`) blocks the license, so a
            // converged non-conforming establish under `set -e` RUNS (it does NOT stay
            // vouched). Redir failure-edges abort too, but a `Redir` is never a plan leaf
            // consulted for consumption, so only `Command` nodes are marked.
            if self.nodes[v].kind == CfgNodeKind::Command {
                self.consumed[v].0.insert(Channel::StatusRelaxable);
            }
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

    /// Mark each `$?`-reading command's CFG-**predecessor** command(s)' rc as consumed
    /// (19A C-3 / 205 §2: `$?` is the second un-marked status-consumer the committed
    /// engine missed). `$?` reads the *previous* command's exit status, so the consumer
    /// is the predecessor, not the reader itself. Walk back over `pred`, skipping
    /// structural nodes (`Entry`/`Merge`/scope/`Redir`/`Top`) until the first `Command`
    /// on each incoming path; at a merge, every reaching command pred is marked. A walk
    /// that reaches only structural nodes (e.g. `$?` as the first command) marks
    /// nothing. The mark is `StatusRelaxable` (value-relaxable): a known rc still
    /// folds/substitutes, a ⊤ rc blocks the license.
    ///
    /// Conservative by construction (`inv-kfail`-safe): at a pipeline (`a | b` with `$?`
    /// in `b`) or across a subshell boundary the "predecessor" is whatever command the
    /// pred-edges reach — marking more can only *block* more, never license more, so the
    /// marking-more direction is taken without resolving those ambiguities precisely.
    ///
    /// `while`/`until` post-loop `$?` (20O find-6a / task-L2 item-6a): dash's post-loop
    /// `$?` is the BODY's last command rc (loop ran ≥1) or 0 (ran 0) — never the
    /// condition's. A `while`'s only exit edge is `cond_exit → merge`, so the bare
    /// pred-walk would stop at the condition command and miss the body. When the walk
    /// reaches such a loop-exit `merge` ([`while_exit_to_body`]), it also seeds the
    /// recorded body-exit, so the body's last command is marked too (the condition keeps
    /// its mark — already `StatusIterated`-blocked, so the over-mark is inert). A `for`
    /// loop needs no special case: its exit is `head → merge` and `head`'s back-edge pred
    /// is the body-exit, so the body is already reached.
    fn mark_dollar_question_predecessors(&mut self) {
        let readers: Vec<usize> = (0..self.nodes.len())
            .filter(|&v| {
                self.nodes[v].kind == CfgNodeKind::Command
                    && self.command_reads_dollar_question(self.nodes[v].ast)
            })
            .collect();
        for reader in readers {
            // Backward walk: first `Command` on each path; structural nodes recurse.
            let mut visited: BTreeSet<usize> = BTreeSet::new();
            let mut stack: Vec<usize> = self.pred[reader].clone();
            while let Some(p) = stack.pop() {
                if !visited.insert(p) {
                    continue;
                }
                if self.nodes[p].kind == CfgNodeKind::Command {
                    self.consumed[p].0.insert(Channel::StatusRelaxable);
                } else {
                    // A `while`/`until` exit `merge`: also reach the body-exit (item-6a),
                    // so the post-loop `$?` marks the body's last command, not only the
                    // condition the `cond_exit → merge` edge leads back to.
                    if let Some(&body_exit) = self.while_exit_to_body.get(&p) {
                        stack.push(body_exit);
                    }
                    stack.extend(self.pred[p].iter().copied());
                }
            }
        }
    }

    /// Does this `Simple`'s argv or assignment values read `$?` (the special status
    /// parameter, lexed as `WordPart::Param { name: "?" }`)? Walks double-quoted nesting
    /// (`"$?"` reads it too). Assignment RHS is included — `rc=$?` is the canonical idiom
    /// — alongside the words; marking either way is the conservative direction.
    fn command_reads_dollar_question(&self, id: AstId) -> bool {
        let NodeKind::Simple { assigns, words, .. } = &self.ast.node(id).kind else {
            return false;
        };
        let assign_values = assigns
            .iter()
            .filter_map(|&a| match &self.ast.node(a).kind {
                NodeKind::Assign { value, .. } => *value,
                _ => None,
            });
        words
            .iter()
            .copied()
            .chain(assign_values)
            .any(|w| self.word_reads_dollar_question(w))
    }

    /// Does a `Word` node contain the `$?` special parameter (recursing into
    /// double-quoted parts)?
    fn word_reads_dollar_question(&self, word_id: AstId) -> bool {
        let NodeKind::Word { parts } = &self.ast.node(word_id).kind else {
            return false;
        };
        parts_read_dollar_question(parts)
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

    /// Is `id` the EXACT bare-`true` rhs that licenses door-3 (`20V` §4 d-2)? A
    /// [`NodeKind::Simple`] whose argv is exactly the single literal word `true` —
    /// zero arguments, zero redirections, zero assignment-prefixes, and (because
    /// [`word_literal`](Self::word_literal) only matches a lone `Literal`/`SingleQuoted`
    /// part) no command-substitution. This is a deliberately narrow slice: `true x`,
    /// `X=1 true`, `true >/dev/null`, and `$(echo true)` all fail it and keep the
    /// blocking `StatusRelaxable` mark. `:` does NOT qualify (it is semantically
    /// identical, but every license-surface widening is a disaster-class-bug locus —
    /// `20V` §4 widens deliberately, candidate-by-candidate; the `|| :` deferral is
    /// pinned as a unit test). `false` does not qualify (it changes the list rc).
    fn right_is_bare_true(&self, id: AstId) -> bool {
        let NodeKind::Simple {
            assigns,
            words,
            redirs,
        } = &self.ast.node(id).kind
        else {
            return false;
        };
        assigns.is_empty()
            && redirs.is_empty()
            && matches!(words.as_slice(), [w] if self.word_literal(*w) == Some("true"))
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
                [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    /// Command-substitution bodies inside a word (each a nested `List`/`Script`
    /// `AstId`), in source order. Only top-level and double-quoted parts are walked
    /// (the lossless-quoting model); a `$( … )` runs in a subshell either way.
    fn command_substs(&self, word_id: AstId) -> Vec<AstId> {
        let mut out = Vec::new();
        if let NodeKind::Word { parts } = &self.ast.node(word_id).kind {
            collect_substs(parts, &mut out);
        }
        out
    }

    // ---- arch-2 funcdef-body scanners (pure; `i-1` eligibility) ----------------

    /// arch-2 (`i-1`): does the funcdef `body`'s AST subtree use an out-of-slice positional
    /// construct (`$@`/`$*`/`shift`/`local`)? Returns the offending construct's spelling for
    /// the refusal diagnostic, or `None`. These are out of the modeled subset (the
    /// positional-array forms `$@`/`$*` the value plane folds to ⊤; `shift`/`local` mutate the
    /// positional/scope state the splice's positional overlay does not model). Span-contained
    /// scan over the AST (cheap; corpus bodies are tiny). A `$@`/`$*` is a
    /// `WordPart::Param { name }` whose name is the special `@`/`*`; `shift`/`local` are command
    /// words.
    fn body_uses_unmodeled_positional(&self, body: AstId) -> Option<&'static str> {
        for (aid, node) in self.ast.iter() {
            if !node_within(self.ast, aid, body) {
                continue;
            }
            match &node.kind {
                NodeKind::Word { parts } => {
                    if let Some(p) = word_parts_special_positional(parts) {
                        return Some(p);
                    }
                }
                NodeKind::Simple { words, .. } => {
                    match words.first().and_then(|&w| self.word_literal(w)) {
                        Some("shift") => return Some("shift"),
                        Some("local") => return Some("local"),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// arch-2 (tc-M2): does the funcdef `body`'s AST subtree carry a WRITE-shaped redirect
    /// (`>`/`>>`/`<>`) to anything OTHER than `/dev/null`? Returns a short detail for the
    /// refusal diagnostic, or `None`. A body file-write is an unmodeled effect (y-1) that
    /// inlining alone would EXPOSE as wrong-ambience — so the call refuses (the wrapper-pun
    /// population redirects only to `/dev/null`, which stays exempt, keeping it alive). `2>&1`
    /// fd-dups and here-docs are NOT write-to-file redirects (a `Dup`/`HereDoc` op), so they
    /// are exempt; only `Write`/`Append`/`<>`-class file targets fence. Span-contained scan.
    fn body_has_unmodeled_write_redirect(&self, body: AstId) -> Option<String> {
        for (aid, node) in self.ast.iter() {
            if !node_within(self.ast, aid, body) {
                continue;
            }
            let NodeKind::Redir { op, target, .. } = &node.kind else {
                continue;
            };
            // Only file-write redirects fence (`>`, `>>`). A read (`<`), an fd-dup (`2>&1`,
            // `>&3`), and a here-doc (`<<EOF`) are not a write-to-file effect.
            if !matches!(op, RedirOp::Write | RedirOp::Append) {
                continue;
            }
            let RedirTarget::Word(w) = target else {
                continue; // fd-dup / here-doc target: not a file write
            };
            match word_text(self.ast, *w) {
                Some("/dev/null") => {} // the discard sink stays exempt (devnull-exemption)
                Some(path) => return Some(format!("`> {path}`")),
                None => return Some("`>` to a dynamic/unresolved target".to_string()),
            }
        }
        None
    }

    // ---- condition-context tagging --------------------------------------------

    /// Lower a *condition-context* region (the test of an `if`/`elif`, or the left
    /// operand of `&&`/`||`) and clear errexit-fallibility across **every** node it
    /// produces, returning the region's exit. errexit is suppressed throughout such
    /// a region, not merely at its tail (find-2), and reaches the inner non-final
    /// operands of a compound test (find-3) — both of which the old tail-only
    /// `mark_condition_context` missed when the region exit was a `Merge`.
    ///
    /// `mark_status`, when `Some(channel)`, additionally marks every command in the region
    /// as consuming `channel` — the test of an `if`/`elif`, or a `while`/`until` condition,
    /// is an **unambiguous guard** (a *different* branch runs on its rc: the then/else, or
    /// the body/exit), so its status is branch-consumed. The CHANNEL differs by what can
    /// reproduce the read (arch-1 / note 214; the round-21 `StatusRenderFloor` render-floor
    /// is retired now that the leaf-exact render substitutes a guard's byte-span in-situ):
    /// * an `if`/`elif` condition is marked `Channel::StatusRelaxable` — a single-shot guard
    ///   a probe-sourced KNOWN rc reproduces exactly (`if (exit 1); then` is dash-clean), ⊤
    ///   blocks. The guard became an ordinary substitution site;
    /// * a `while`/`until` condition is marked `Channel::StatusIterated` — the condition is
    ///   re-evaluated per pass, so the consumed value is a SEQUENCE no single rc reproduces,
    ///   and substituting a loop condition with a constant is an infinite/zero-iteration
    ///   disaster, so it blocks UNCONDITIONALLY (the honest successor to the old floor,
    ///   keyed on iteration not render capability).
    ///
    /// A `&&`/`||` left operand is marked the value-relaxable `Channel::StatusRelaxable` (or
    /// `StatusInvariant` for `|| true`) at its OWN call site (`lower_and_or`), not here —
    /// it passes `None`. So the `StatusRelaxable` channel has these sources (`206` §3 +
    /// arch-1): a `&&`/`||` operand, an errexit-region command (`materialise_errexit_edges`),
    /// a `$?`-reader's predecessor (`mark_dollar_question_predecessors`), and an `if`/`elif`
    /// guard (here); `StatusIterated` has the lone `while`/`until` condition (here). errexit
    /// is not special-cased-as-vouched (19A C-3 / 205 §2): a converged conforming establish
    /// under `set -e` still folds via a known rc, but a ⊤-rc mutator runs.
    ///
    /// Implemented as a node-range mark/clear: CFG nodes are arena-allocated in walk
    /// order, so the half-open range `[before, after)` is exactly the region's
    /// nodes (`inv-determinism` makes this range stable).
    fn lower_condition_region(
        &mut self,
        cond: AstId,
        entry_pred: CfgNodeId,
        mark_status: Option<Channel>,
    ) -> CfgNodeId {
        let first = self.nodes.len();
        let exit = self.lower_node(cond, entry_pred);
        self.clear_fallible_range(first, self.nodes.len());
        if let Some(channel) = mark_status {
            self.mark_consumed_range(first, self.nodes.len(), &Powerset::singleton(channel));
        }
        exit
    }

    /// Clear the fallibility flag on every node in the half-open arena range
    /// `[from, to)` (a condition region, or a negated pipeline — find-1/2/3/5).
    /// Clearing a non-fallible node is a no-op, so over-broad ranges are harmless.
    fn clear_fallible_range(&mut self, from: usize, to: usize) {
        for f in &mut self.fallible[from..to] {
            *f = false;
        }
    }

    /// Mark every `Command` node in the half-open arena range `[from, to)` as having
    /// each of `obs` consumed by an enclosing context (note 16J). Mirrors the
    /// `expansion_internal` / `clear_fallible_range` arena-range idiom: a construct's
    /// context applies to every leaf it lexically contains — exactly the enclosing
    /// case the old leaf-local gate missed (16G kill-shot). Conservative: it also
    /// marks nested / already-captured leaves, but over-marking ⇒ over-run ⇒ sound
    /// (`kFAIL` / `kPRECISION`). Empty `obs` is a no-op (a `> /dev/null` discard).
    fn mark_consumed_range(&mut self, from: usize, to: usize, obs: &Powerset<Channel>) {
        if obs.0.is_empty() {
            return;
        }
        for v in from..to {
            if self.nodes[v].kind == CfgNodeKind::Command {
                for &o in &obs.0 {
                    self.consumed[v].0.insert(o);
                }
            }
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
            expansion_internal: self.expansion_internal,
            in_loop: self.in_loop,
            spliced_internal: self.spliced_internal,
            call_body_sites: self.call_body_sites,
            consumed: self.consumed,
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

/// Which unvouched output observables a redirection list captures (note 16J): a
/// `Write`/`Append` of fd 1 (or the default) ⇒ `Stdout`, fd 2 ⇒ `Stderr` — UNLESS
/// the target is `/dev/null` (the discard sink, the precision scalpel; 16F §5 /
/// 16G). fd-dups (`2>&1`, `>&3`) are deliberately NOT resolved (a deferred
/// refinement — 16G); the structural floor already runs any file-redirected leaf.
fn output_redir_observables(ast: &Ast, redirs: &[AstId]) -> Powerset<Channel> {
    let mut out = BTreeSet::new();
    for &r in redirs {
        let NodeKind::Redir { op, fd, target } = &ast.node(r).kind else {
            continue;
        };
        if !matches!(op, RedirOp::Write | RedirOp::Append) {
            continue; // input / fd-dup / here-doc: not an output-write sink
        }
        let RedirTarget::Word(w) = target else {
            continue;
        };
        if word_text(ast, *w) == Some("/dev/null") {
            continue; // discard sink ⇒ not consumed (the scalpel)
        }
        match fd {
            None | Some(1) => {
                out.insert(Channel::Stdout);
            }
            Some(2) => {
                out.insert(Channel::Stderr);
            }
            _ => {}
        }
    }
    Powerset(out)
}

/// The single-literal text of a word node, if it is exactly one literal fragment
/// (mirrors `effect::word_literal`): used to recognise the `/dev/null` discard sink.
fn word_text(ast: &Ast, w: AstId) -> Option<&str> {
    match &ast.node(w).kind {
        NodeKind::Word { parts } => match parts.as_slice() {
            [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
}

/// arch-2: build the funcdef registry — every `name() { … }` definition's `(body AstId,
/// def-start BytePos)`, grouped by name in source order. A name with more than one entry is
/// REDEFINED ⇒ every call ⊤-rejects (`i-1`); a call resolves to the latest definition strictly
/// before it. `BTreeMap` for `inv-determinism`. Pure over the AST (the `FuncDef` node carries
/// `name`, `name_span`, `body`; the definition's start is the node's own span `lo`).
fn collect_funcdefs(ast: &Ast) -> BTreeMap<String, Vec<(AstId, BytePos)>> {
    let mut out: BTreeMap<String, Vec<(AstId, BytePos)>> = BTreeMap::new();
    for (aid, node) in ast.iter() {
        if let NodeKind::FuncDef { name, body, .. } = &node.kind {
            out.entry(name.clone())
                .or_default()
                .push((*body, ast.node(aid).span.lo));
        }
    }
    out
}

/// arch-2: a conservative node-count estimate for a funcdef body — the number of AST nodes in
/// its subtree (`i-1` per-site budget). AST descendants are ≥ the CFG leaf nodes the body
/// lowers to (each `Simple` ⇒ ≥1 `Command`; structural nodes add more CFG nodes but the AST
/// count over-estimates conservatively for the budget's purpose — over-estimating refuses
/// MORE, the safe direction). Pure span-containment count over the AST.
fn subtree_node_count(ast: &Ast, body: AstId) -> usize {
    ast.iter()
        .filter(|(aid, _)| node_within(ast, *aid, body))
        .count()
}

/// Is node `inner` within node `outer`'s subtree, by span containment? AST spans nest by
/// construction (a child's `[lo,hi)` lies within its parent's), so a byte-range containment
/// test is a sound subtree-membership check. `inner == outer` counts as within. (Mirrors
/// `value::node_within`; arch-2's body scanners + budget use it.)
fn node_within(ast: &Ast, inner: AstId, outer: AstId) -> bool {
    let i = ast.node(inner).span;
    let o = ast.node(outer).span;
    o.lo.0 <= i.lo.0 && i.hi.0 <= o.hi.0
}

/// arch-2 (`i-1`): if any word part is the special positional-array parameter `$@`/`$*`,
/// return its spelling (`"$@"`/`"$*"`) for the refusal diagnostic; else `None`. Recurses into
/// double-quoted nesting (`"$@"` is the common form). The lexer keeps `$@`/`$*` as
/// `WordPart::Param { name: "@" | "*" }`.
fn word_parts_special_positional(parts: &[WordPart]) -> Option<&'static str> {
    for part in parts {
        match part {
            WordPart::Param { name } if name == "@" => return Some("$@"),
            WordPart::Param { name } if name == "*" => return Some("$*"),
            WordPart::DoubleQuoted(inner) => {
                if let Some(p) = word_parts_special_positional(inner) {
                    return Some(p);
                }
            }
            _ => {}
        }
    }
    None
}

/// Does this list of word parts contain the `$?` special parameter (the lexer keeps
/// it as `Param { name: "?" }`)? Recurses into double-quoted nesting (`"$?"` reads it).
/// `ParamComplex` (`${...}` operator-forms) is opaque ⇒ conservatively NOT matched as
/// `$?` (such a form is already ⊤-ward; this pass need not over-reach into it).
fn parts_read_dollar_question(parts: &[WordPart]) -> bool {
    parts.iter().any(|p| match p {
        WordPart::Param { name } => name == "?",
        WordPart::DoubleQuoted(inner) => parts_read_dollar_question(inner),
        _ => false,
    })
}

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
