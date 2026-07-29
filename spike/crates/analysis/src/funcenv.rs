//! `analysis::funcenv` — the FUNCTION ENVIRONMENT as a first-class abstractly-interpreted domain
//! (`28K` §2). The answer to "which oracle answers this site" is no longer a Dorc resolution rule;
//! it is whatever definition a shell would have live at that program point.
//!
//! The domain is positional and scope-aware, exactly like [`value`](crate::value), and rides the
//! same worklist: per program point, a STACK of frames mapping a function NAME to the definition
//! bound to it. `.`-sourcing applies a file's definitions, `unset -f` removes one, a subshell
//! pushes a frame that its exit pops — which is how `28K` §3's regional-preference idiom
//! (`( . better-yum.sh; … )`) gets its scoping for free rather than by special case.
//!
//! # Two things here are wrong-elision tripwires; both have hard test tables
//!
//! **`Undefined` is an ELEMENT, never map-absence.** [`MapL`] is canonical-no-⊥ (absent ≡ ⊥), and
//! `⊥` is the join identity — so a name left ABSENT on one branch of `if c; then f() {…}; fi`
//! would join as `Elem(d) ⊔ ⊥ = Elem(d)`, claiming a definition is live on a path that never
//! defined it. That is a wrong-elision: licensure would read a body the shell would not call. The
//! entry state therefore seeds EVERY name in the unit's universe to an explicit
//! `Elem(Binding::Undefined)`, so the same join is `Elem(d) ⊔ Elem(Undefined) = Top` — can't-say,
//! which walls. This mirrors [`value`](crate::value)'s entry ⊤-seed and exists for the same
//! reason.
//!
//! **Non-convergence folds to ⊤ everywhere.** [`solve`](crate::solve)'s termination preconditions
//! are caller-upheld and un-type-enforceable; a capped solve is an under-approximation, and an
//! under-approximated function environment is precisely a set of confident wrong answers about
//! whose body runs. `converged == false` ⇒ every query answers ⊤ (`16P` DP-9, the same bargain
//! `value` strikes).
//!
//! # What this module may NOT see
//!
//! It reads the SOURCE-LITERAL plane only — see [`SourceLiteralPlane`]. It also names no records,
//! effect-vector, erasure, or verdict type, by signature: the environment is computed ONCE from
//! the origin model, before the validity fixpoint, and nothing a later round learns may flow back
//! into which definition was live (`cli/CLAUDE.md` the-fixpoint-owns-the-rounds; the fold's ratchet
//! erases EFFECTS and has no authority over BINDINGS).

use std::collections::{BTreeMap, BTreeSet};

use dorc_core::{AstId, Interner, Span, Symbol, ValueGrade};
use dorc_syntax::ast::{Ast, NodeKind};

use crate::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use crate::lattice::{Flat, Lattice, MapL};
use crate::solve::{Direction, Graph, solve};
use crate::value::{ValueFlow, ValueOf};

// ===========================================================================
// The value-plane seam — funcenv-reads-source-literal-plane-only
// ===========================================================================

/// The ONE window this module has onto resolved values, and it is deliberately narrow: a word
/// counts only when it is a literal whose provenance is [`ValueGrade::ProgramText`] — program
/// text, not a belief about the world (`275` §1).
///
/// Today the restriction is trivially total (every non-⊤ word is `ProgramText` at this stage), and
/// that is exactly why the door is closed NOW. `core`'s `seam-re-bind` will fold probe-captured
/// values back into the value plane, and when it does, a `WorldSpoken` value must not be able to
/// site a load, resolve a source target, or answer an env construct: which oracle answers a site
/// would then depend on what a host said, making oracle loading world-dependent and the plan
/// unreproducible from its inputs. The wall must already be standing when that lands, not be
/// remembered afterwards.
#[derive(Debug, Clone, Copy)]
pub struct SourceLiteralPlane<'a> {
    value: &'a ValueFlow,
    interner: &'a Interner,
}

impl<'a> SourceLiteralPlane<'a> {
    #[must_use]
    pub fn new(value: &'a ValueFlow, interner: &'a Interner) -> Self {
        Self { value, interner }
    }

    /// Word `index` of `node`'s argv as a source-literal symbol, or `None` when it is ⊤, absent,
    /// or carries any provenance weaker than program text.
    #[must_use]
    pub fn literal_word(&self, node: CfgNodeId, index: usize) -> Option<Symbol> {
        let grades = self.value.argv_word_grades(node);
        if grades.get(index) != Some(&ValueGrade::ProgramText) {
            return None;
        }
        match self.value.argv_values(node).get(index) {
            Some(&ValueOf::Literal(sym)) => Some(sym),
            _ => None,
        }
    }

    /// Word `index` as source-literal TEXT. Resolving here is ordinary sh modeling — the words
    /// this module matches are shell builtins (`.`, `source`, `unset`) and load paths, never an
    /// oracle token or kind whose text `inv-referent-agnostic` forbids decoding (the precedent is
    /// `effect`'s builtin recognition).
    #[must_use]
    pub fn literal_text(&self, node: CfgNodeId, index: usize) -> Option<&'a str> {
        self.literal_word(node, index)
            .map(|sym| self.interner.resolve(sym))
    }

    /// Whether the underlying value analysis converged; a capped value solve makes every word ⊤,
    /// and this domain must not read confident answers off it.
    #[must_use]
    pub fn converged(&self) -> bool {
        self.value.converged()
    }
}

// ===========================================================================
// Definitions, and the loaded unit
// ===========================================================================

/// A function definition's identity: an index into the unit's [`DefinitionTable`]. Opaque and
/// `Copy` so it can ride the lattice; the bytes it names are resolved by the caller for emission
/// and display only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefId(pub u32);

/// One definition, as DATA the kernel was handed — never something this module read from a disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// Which input file defines it (`28K` §2a: one id space over book and oracles alike).
    pub file: dorc_core::SourceFileId,
    /// The function name as authored.
    pub name: String,
    /// The whole `name() { … }` span within that file.
    pub span: Span,
}

/// Every definition in the analysis unit, plus the load structure over them.
///
/// Built at the cli edge (the only place allowed to read files) and handed in whole, so this
/// module stays a pure function of its inputs (`inv-determinism`).
#[derive(Debug, Clone, Default)]
pub struct DefinitionTable {
    defs: Vec<Definition>,
    /// Per loadable path, the definitions that file contributes IN FILE ORDER (so applying them
    /// left-to-right reproduces sh's last-wins).
    by_path: BTreeMap<String, Vec<DefId>>,
    /// The ambient prefix: the CLI-named sources' definitions, in command-line then file order
    /// (`28K` §2 — they load "before line 1").
    ambient: Vec<DefId>,
    /// For a definition sited in the BOOK, the `FuncDef` AST node that writes it. The book's
    /// definitions execute positionally, so the transfer needs to go from "this definition
    /// statement just ran" to "which definition that is".
    by_ast: BTreeMap<AstId, DefId>,
}

impl DefinitionTable {
    /// Record a definition and return its id.
    pub fn add(&mut self, def: Definition) -> DefId {
        let id = DefId(u32::try_from(self.defs.len()).unwrap_or(u32::MAX));
        self.defs.push(def);
        id
    }

    /// Declare that `path`, when sourced, contributes `defs` in that order.
    pub fn set_loadable(&mut self, path: String, defs: Vec<DefId>) {
        self.by_path.insert(path, defs);
    }

    /// Append to the ambient prefix (a CLI-named source's definitions, in order).
    pub fn extend_ambient(&mut self, defs: impl IntoIterator<Item = DefId>) {
        self.ambient.extend(defs);
    }

    /// Bind a BOOK definition to the `FuncDef` AST node that writes it.
    pub fn set_book_site(&mut self, ast: AstId, def: DefId) {
        self.by_ast.insert(ast, def);
    }

    #[must_use]
    pub fn get(&self, id: DefId) -> Option<&Definition> {
        self.defs.get(id.0 as usize)
    }

    /// Every NAME the unit could ever bind — the universe the entry state seeds to an explicit
    /// `Undefined`. Deterministic order.
    #[must_use]
    pub fn names(&self) -> BTreeSet<String> {
        self.defs.iter().map(|d| d.name.clone()).collect()
    }

    fn definitions_of_path(&self, path: &str) -> Option<&[DefId]> {
        self.by_path.get(path).map(Vec::as_slice)
    }
}

// ===========================================================================
// The domain
// ===========================================================================

/// What a name is bound to at a program point. `Undefined` is a first-class ELEMENT — see the
/// module doc; encoding it as map-absence is a wrong-elision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binding {
    /// Provably not defined as a function here.
    Undefined,
    /// Bound to exactly this definition.
    Defined(DefId),
}

/// One lexical scope's bindings. Absent ≡ ⊥ ≡ "this frame says nothing", which is what lets an
/// inner frame shadow without copying the outer one.
type Frame = MapL<String, Flat<Binding>>;

/// The abstract environment: a stack of frames, innermost last.
///
/// The stack IS the subshell model. `( … )` pushes; its exit pops, and the outer frames were never
/// touched, so the restore is exact and free — where a clobber-on-exit approximation (the shape
/// [`value`](crate::value) uses for variables, correctly, since a subshell's variable writes are
/// genuinely lost) would ⊤ every name the region re-sourced and poison every later site in the
/// book. That would make `28K` §3's regional-preference idiom actively harmful instead of useful.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvStack {
    /// The join identity — an unreached program point.
    Bottom,
    /// Frames, outermost first.
    Frames(Vec<Frame>),
    /// Everything unknown, at every depth. Reached by havoc (an unmodeled construct, an
    /// unresolvable load) or by joining stacks of different depths.
    Top,
}

impl EnvStack {
    /// The binding for `name`, innermost frame first. A frame that says nothing about the name
    /// (⊥) defers outward; running out of frames is ⊥ — which only happens before the entry seed,
    /// since the seed makes every name of the universe explicit.
    #[must_use]
    pub fn lookup(&self, name: &str) -> Flat<Binding> {
        match self {
            EnvStack::Bottom => Flat::Bottom,
            EnvStack::Top => Flat::Top,
            EnvStack::Frames(frames) => {
                for frame in frames.iter().rev() {
                    let here = frame.get(&name.to_owned());
                    if here != Flat::Bottom {
                        return here;
                    }
                }
                Flat::Bottom
            }
        }
    }

    /// Bind `name` in the innermost frame.
    fn bind(&mut self, name: &str, to: Flat<Binding>) {
        if let EnvStack::Frames(frames) = self
            && let Some(top) = frames.last_mut()
        {
            top.insert(name.to_owned(), to);
        }
    }

    fn push(&self) -> Self {
        match self {
            EnvStack::Frames(frames) => {
                let mut next = frames.clone();
                next.push(Frame::default());
                EnvStack::Frames(next)
            }
            other => other.clone(),
        }
    }

    fn pop(&self) -> Self {
        match self {
            // Popping the LAST frame would leave no seat to bind into; the outermost scope is the
            // script itself and its exit is the program's, so this only guards a malformed pair.
            EnvStack::Frames(frames) if frames.len() > 1 => {
                let mut next = frames.clone();
                next.pop();
                EnvStack::Frames(next)
            }
            other => other.clone(),
        }
    }
}

impl Lattice for EnvStack {
    fn bottom() -> Self {
        EnvStack::Bottom
    }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (EnvStack::Bottom, x) | (x, EnvStack::Bottom) => x.clone(),
            (EnvStack::Top, _) | (_, EnvStack::Top) => EnvStack::Top,
            (EnvStack::Frames(a), EnvStack::Frames(b)) => {
                // Well-nested scopes always meet at equal depth; unequal depth means the CFG
                // merged across a scope boundary, and there is no honest pointwise answer — ⊤ is
                // the safe one (every name can't-say ⇒ every family walls).
                if a.len() != b.len() {
                    return EnvStack::Top;
                }
                EnvStack::Frames(a.iter().zip(b).map(|(x, y)| x.join(y)).collect())
            }
        }
    }

    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (EnvStack::Top, x) | (x, EnvStack::Top) => x.clone(),
            (EnvStack::Bottom, _) | (_, EnvStack::Bottom) => EnvStack::Bottom,
            (EnvStack::Frames(a), EnvStack::Frames(b)) => {
                if a.len() != b.len() {
                    return EnvStack::Bottom;
                }
                EnvStack::Frames(a.iter().zip(b).map(|(x, y)| x.meet(y)).collect())
            }
        }
    }
}

// ===========================================================================
// The analysis
// ===========================================================================

/// The solved function environment: per program point, which definition each name is bound to.
#[derive(Debug, Clone)]
pub struct FuncEnv {
    states: Vec<EnvStack>,
    converged: bool,
    /// Nodes whose transfer havoc'd the environment because a load could not be resolved
    /// (`28K` §1 rul-unloadable-is-unlicensed). Reported by the caller; recorded here as data so
    /// the kernel mints no diagnostics of its own.
    unresolvable_loads: BTreeSet<CfgNodeId>,
}

impl FuncEnv {
    /// The environment IMMEDIATELY BEFORE `node` — the positional regime's query (`28K` §2:
    /// anything standing in for text in the execution stream reads sh execution order).
    ///
    /// Answers ⊤ for everything when the solve did not converge.
    #[must_use]
    pub fn before(&self, node: CfgNodeId) -> EnvStack {
        if !self.converged {
            return EnvStack::Top;
        }
        self.states
            .get(node.index())
            .cloned()
            .unwrap_or(EnvStack::Top)
    }

    /// The binding `name` has immediately before `node`.
    #[must_use]
    pub fn binding_before(&self, node: CfgNodeId, name: &str) -> Flat<Binding> {
        self.before(node).lookup(name)
    }

    #[must_use]
    pub fn converged(&self) -> bool {
        self.converged
    }

    /// The sites whose load could not be resolved, for the caller to disclose.
    #[must_use]
    pub fn unresolvable_loads(&self) -> &BTreeSet<CfgNodeId> {
        &self.unresolvable_loads
    }
}

/// Solve the function environment over `cfg`.
///
/// Pure: `defs` is the whole loaded unit as data and `literals` is the narrow source-literal
/// window; nothing here reads a clock, a file, or a host answer.
#[must_use]
pub fn analyze(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
) -> FuncEnv {
    let universe = defs.names();
    // A capped VALUE solve makes every word ⊤, so no load target and no `unset -f` operand could
    // be read; rather than silently binding nothing, refuse the whole environment up front.
    if !literals.converged() {
        return FuncEnv {
            states: vec![EnvStack::Top; cfg.node_count()],
            converged: false,
            unresolvable_loads: BTreeSet::new(),
        };
    }
    // Which loads cannot be resolved is a pure function of (cfg, literals, defs) — it does not
    // depend on the environment at all — so it is its own pass rather than an out-param smuggled
    // through the transfer. That keeps the transfer a plain `Fn` and the kernel free of interior
    // mutability.
    let unresolvable_loads = unresolvable_load_sites(cfg, defs, literals);
    let solution = solve(cfg, Direction::Forward, |node, incoming: &EnvStack| {
        transfer(ast, cfg, defs, literals, &universe, node, incoming)
    });
    FuncEnv {
        states: solution.states,
        converged: solution.converged,
        unresolvable_loads,
    }
}

/// Every `.`/`source` site whose target this analysis cannot resolve to a loaded file — a dynamic
/// path, a path the driver never read, or a target word carrying anything weaker than source-literal
/// provenance. Each one havocs the environment (`28K` §1 rul-unloadable-is-unlicensed); the caller
/// discloses them, since silence licenses nothing.
fn unresolvable_load_sites(
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
) -> BTreeSet<CfgNodeId> {
    let mut out = BTreeSet::new();
    for node in 0..cfg.node_count() {
        let id = CfgNodeId(u32::try_from(node).unwrap_or(u32::MAX));
        if cfg.node(id).kind != CfgNodeKind::Command {
            continue;
        }
        if !matches!(literals.literal_text(id, 0), Some("." | "source")) {
            continue;
        }
        let resolved = literals
            .literal_text(id, 1)
            .is_some_and(|target| defs.definitions_of_path(target).is_some());
        if !resolved {
            out.insert(id);
        }
    }
    out
}

/// The per-node transfer.
fn transfer(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    universe: &BTreeSet<String>,
    node: usize,
    incoming: &EnvStack,
) -> EnvStack {
    let id = CfgNodeId(u32::try_from(node).unwrap_or(u32::MAX));
    let cfg_node = cfg.node(id);
    match cfg_node.kind {
        // The entry seeds the AMBIENT PREFIX: every name of the universe explicitly `Undefined`
        // (see the module doc — absence would join as ⊥ and claim a definition on a path that
        // never made one), then the CLI-named sources' definitions applied in load order.
        CfgNodeKind::Entry => {
            let mut frame = Frame::default();
            for name in universe {
                frame.insert(name.clone(), Flat::Elem(Binding::Undefined));
            }
            let mut env = EnvStack::Frames(vec![frame]);
            for &def in &defs.ambient {
                if let Some(d) = defs.get(def) {
                    env.bind(&d.name, Flat::Elem(Binding::Defined(def)));
                }
            }
            env
        }
        CfgNodeKind::ScopeEnter => incoming.push(),
        CfgNodeKind::ScopeExit => incoming.pop(),
        // A ⊤-rejected region is UNPARSED: its body may define, unset, or source anything,
        // invisibly. Half-modeling it as a no-op is the DP-8 trap.
        CfgNodeKind::Top => EnvStack::Top,
        // `cfg::lower_funcdef` lowers the DEFINITION STATEMENT to a pass-through `Merge` carrying
        // the `FuncDef`'s own AstId; that node is where the binding takes effect in the main flow.
        CfgNodeKind::Merge => match &ast.node(cfg_node.ast).kind {
            NodeKind::FuncDef { name, .. } => {
                let mut env = incoming.clone();
                env.bind(name, definition_at(defs, cfg_node.ast));
                env
            }
            _ => incoming.clone(),
        },
        CfgNodeKind::Command => command_transfer(defs, literals, id, incoming),
        _ => incoming.clone(),
    }
}

/// The definition bound by an in-book `FuncDef` statement.
///
/// A definition statement the caller's table does not know is ⊤, never a skipped binding: SOMETHING
/// bound this name here, and pretending otherwise would leave a stale earlier definition live and
/// license a body the shell would no longer call. `28K` §7's `guard23-reingest-collision-verbatim`
/// is exactly this shape — a book whose inlined definition shadows a loaded oracle's.
fn definition_at(defs: &DefinitionTable, ast: AstId) -> Flat<Binding> {
    defs.by_ast
        .get(&ast)
        .map_or(Flat::Top, |&def| Flat::Elem(Binding::Defined(def)))
}

/// `.`/`source`, `unset -f`, and nothing else. Every other command leaves the environment alone —
/// including `command -v`, which QUERIES the environment (its consumer is the branch fold, not a
/// binding change).
fn command_transfer(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    incoming: &EnvStack,
) -> EnvStack {
    let Some(head) = literals.literal_text(node, 0) else {
        return incoming.clone();
    };
    match head {
        "." | "source" => {
            let Some(target) = literals.literal_text(node, 1) else {
                return EnvStack::Top;
            };
            // A path the driver did not load: `28K` §1 rul-unloadable-is-unlicensed — the affected
            // names are ⊤, and since we cannot know WHICH names such a file defines, the whole
            // environment is. `unresolvable_load_sites` names these sites for disclosure.
            let Some(contributed) = defs.definitions_of_path(target) else {
                return EnvStack::Top;
            };
            let mut env = incoming.clone();
            for &def in contributed {
                if let Some(d) = defs.get(def) {
                    env.bind(&d.name, Flat::Elem(Binding::Defined(def)));
                }
            }
            env
        }
        "unset" => {
            // Only `unset -f NAME…` touches functions; `unset NAME` is a variable.
            if literals.literal_text(node, 1) != Some("-f") {
                return incoming.clone();
            }
            let mut env = incoming.clone();
            let mut index = 2;
            while let Some(name) = literals.literal_text(node, index) {
                env.bind(name, Flat::Elem(Binding::Undefined));
                index += 1;
            }
            env
        }
        _ => incoming.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Binding, DefId, Definition, DefinitionTable, EnvStack, FuncEnv, SourceLiteralPlane, analyze,
    };
    use crate::cfg::{self, CfgNodeId};
    use crate::lattice::{Flat, Lattice, MapL};
    use dorc_core::{BytePos, Interner, SourceFileId, Span};
    use dorc_syntax::ast::NodeKind;
    use std::collections::BTreeSet;

    fn add_def(table: &mut DefinitionTable, file: u32, name: &str) -> DefId {
        table.add(Definition {
            file: SourceFileId(file),
            name: name.to_owned(),
            span: Span::new(BytePos(0), BytePos(1)),
        })
    }

    /// Solve `book` against a caller-shaped table, returning the environment plus the program
    /// EXIT node. The exit's in-state is the join over every path's end-of-load environment,
    /// which is what "what would a shell have live here" means for a whole-unit query. (`solve`
    /// stores each node's INPUT state, so querying the entry would read ⊥ — its seed is what the
    /// entry's transfer produces, not what it receives.)
    fn solve_book(book: &str, table: &DefinitionTable) -> (FuncEnv, CfgNodeId) {
        let mut interner = Interner::default();
        let ast = dorc_syntax::parse(book).value;
        let cfg = cfg::build(&ast).value;
        let value = crate::value::analyze(&cfg, &ast, &mut interner);
        let plane = SourceLiteralPlane::new(&value, &interner);
        let exit = cfg.exit();
        (analyze(&ast, &cfg, table, &plane), exit)
    }

    /// The `FuncDef` AST node of the first function `book` defines.
    fn first_funcdef(book: &str) -> dorc_core::AstId {
        let ast = dorc_syntax::parse(book).value;
        ast.iter()
            .find(|(_, n)| matches!(&n.kind, NodeKind::FuncDef { .. }))
            .map(|(id, _)| id)
            .expect("the book defines a function")
    }

    // =======================================================================
    // TABLE 1 — `Undefined` is an ELEMENT, never map-absence
    // =======================================================================

    /// The bare lattice statement, independent of any CFG, and the single most important test in
    /// this module: an explicit `Undefined` joined with a definition is ⊤, where an ABSENT key
    /// joined with a definition is the definition. The second half is the trap — stated positively
    /// so the contrast is on the record — and it is why the entry state seeds every name.
    #[test]
    fn explicit_undefined_joins_to_top_where_absence_would_join_to_the_definition() {
        let d = Binding::Defined(DefId(0));
        let mut explicit: MapL<String, Flat<Binding>> = MapL::default();
        explicit.insert("f".to_owned(), Flat::Elem(Binding::Undefined));
        let mut defined: MapL<String, Flat<Binding>> = MapL::default();
        defined.insert("f".to_owned(), Flat::Elem(d));
        assert_eq!(
            explicit.join(&defined).get(&"f".to_owned()),
            Flat::Top,
            "explicit Undefined ⊔ Defined = ⊤, the safe answer"
        );
        let absent: MapL<String, Flat<Binding>> = MapL::default();
        assert_eq!(
            absent.join(&defined).get(&"f".to_owned()),
            Flat::Elem(d),
            "an absent key is ⊥ and ⊥ is the join identity — the WRONG answer, which is exactly \
             what the explicit seed exists to prevent"
        );
    }

    /// The entry seed, which is what makes the above hold end-to-end: a name the unit knows but
    /// nothing has loaded is explicitly `Undefined` at entry, never absent.
    #[test]
    fn the_entry_state_seeds_every_known_name_explicitly() {
        let mut table = DefinitionTable::default();
        let _ = add_def(&mut table, 0, "never_loaded__is_converged");
        let (solved, exit) = solve_book("true\n", &table);
        assert_eq!(
            solved.binding_before(exit, "never_loaded__is_converged"),
            Flat::Elem(Binding::Undefined),
            "a name the unit knows but nothing loaded reads explicitly Undefined, never absent"
        );
    }

    /// The half-defining branch, end-to-end: `f` defined on ONE arm of a conditional must read
    /// can't-say afterwards. Were undefinedness encoded as absence, this would confidently report
    /// the definition live on a path that never made it — a wrong-elision, since licensure would
    /// then read a body the shell would not call.
    #[test]
    fn a_half_defining_branch_joins_to_top_not_to_the_definition() {
        let book = "if [ -f /etc/x ]; then f__is_converged() { :; }; fi\n";
        let mut table = DefinitionTable::default();
        let d = add_def(&mut table, 9, "f__is_converged");
        table.set_book_site(first_funcdef(book), d);
        let (solved, exit) = solve_book(book, &table);
        assert_eq!(solved.binding_before(exit, "f__is_converged"), Flat::Top);
    }

    /// The contrast case that proves the previous test is not vacuous: an UNCONDITIONAL definition
    /// really does bind, so the ⊤ above comes from the branch and not from the machinery failing
    /// to bind at all.
    #[test]
    fn an_unconditional_definition_binds_concretely() {
        let book = "f__is_converged() { :; }\n";
        let mut table = DefinitionTable::default();
        let d = add_def(&mut table, 9, "f__is_converged");
        table.set_book_site(first_funcdef(book), d);
        let (solved, exit) = solve_book(book, &table);
        assert_eq!(
            solved.binding_before(exit, "f__is_converged"),
            Flat::Elem(Binding::Defined(d))
        );
    }

    // =======================================================================
    // TABLE 2 — non-convergence folds to ⊤ everywhere
    // =======================================================================

    /// The fold lives on the QUERY, not merely on the stored states, so a consumer that forgets to
    /// check `converged()` still cannot read a confident answer off a capped solve. An
    /// under-approximated function environment is precisely a set of confident wrong answers about
    /// whose body runs, which is why this is belt-and-braces rather than a caller obligation.
    #[test]
    fn every_query_answers_top_when_the_solve_did_not_converge() {
        let mut frame: MapL<String, Flat<Binding>> = MapL::default();
        frame.insert("f".to_owned(), Flat::Elem(Binding::Defined(DefId(7))));
        let solved = FuncEnv {
            states: vec![EnvStack::Frames(vec![frame])],
            converged: false,
            unresolvable_loads: BTreeSet::new(),
        };
        assert_eq!(solved.before(CfgNodeId(0)), EnvStack::Top);
        assert_eq!(solved.binding_before(CfgNodeId(0), "f"), Flat::Top);
        // Out-of-range is ⊤ too, never a silent ⊥ that would read as "no information".
        assert_eq!(solved.binding_before(CfgNodeId(99), "f"), Flat::Top);
    }

    /// A converged environment with an out-of-range node still answers ⊤ — the same reasoning,
    /// on the other side of the convergence flag.
    #[test]
    fn an_out_of_range_node_answers_top_even_when_converged() {
        let (solved, _) = solve_book("true\n", &DefinitionTable::default());
        assert!(solved.converged());
        assert_eq!(solved.before(CfgNodeId(9999)), EnvStack::Top);
    }

    // =======================================================================
    // The frame stack
    // =======================================================================

    /// Depth mismatch is ⊤, never a pointwise guess: a merge across a scope boundary has no honest
    /// per-frame answer, so every family walls rather than one being invented.
    #[test]
    fn stacks_of_unequal_depth_join_to_top() {
        let one = EnvStack::Frames(vec![MapL::default()]);
        let two = EnvStack::Frames(vec![MapL::default(), MapL::default()]);
        assert_eq!(one.join(&two), EnvStack::Top);
        assert_eq!(EnvStack::Bottom.join(&one), one, "⊥ is the join identity");
        assert_eq!(one.join(&EnvStack::Top), EnvStack::Top);
    }

    /// Lookup walks innermost-first, so an inner frame shadows without copying the outer one —
    /// and popping restores the outer binding exactly, which is the whole reason the domain is a
    /// stack rather than a flat map with a clobber-on-exit rule.
    #[test]
    fn an_inner_frame_shadows_and_the_pop_restores_exactly() {
        let outer_def = Binding::Defined(DefId(1));
        let inner_def = Binding::Defined(DefId(2));
        let mut outer: MapL<String, Flat<Binding>> = MapL::default();
        outer.insert("f".to_owned(), Flat::Elem(outer_def));
        let base = EnvStack::Frames(vec![outer]);

        let mut scoped = base.push();
        scoped.bind("f", Flat::Elem(inner_def));
        assert_eq!(
            scoped.lookup("f"),
            Flat::Elem(inner_def),
            "the inner frame shadows"
        );
        assert_eq!(
            scoped.pop().lookup("f"),
            Flat::Elem(outer_def),
            "and the pop restores the outer binding EXACTLY — a clobber-on-exit approximation \
             would ⊤ this name and poison every later site in the book"
        );
    }

    /// A frame that says nothing about a name defers outward rather than answering ⊥.
    #[test]
    fn an_empty_inner_frame_defers_to_the_enclosing_scope() {
        let d = Binding::Defined(DefId(3));
        let mut outer: MapL<String, Flat<Binding>> = MapL::default();
        outer.insert("f".to_owned(), Flat::Elem(d));
        let scoped = EnvStack::Frames(vec![outer]).push();
        assert_eq!(scoped.lookup("f"), Flat::Elem(d));
    }

    // =======================================================================
    // Containment — the two parsers disagree only where nothing can ship
    // =======================================================================

    /// CELL (a), the ROLE-shaped weird name. `dorc_syntax` and the dialect parser disagree about
    /// what a funcdef is: the dialect parser lifts `中pkg__predict` (which is why
    /// `munge-name-invalid` fires at all) while `dorc_syntax` garbles it into three unrelated
    /// items and says nothing. The containment is that `reserved.rs`'s charclass refusal rejects
    /// exactly that class at Error severity — so for every name that can legally SHIP, the two
    /// parsers agree.
    ///
    /// Pinned from this side: a charclass-refused name produces NO funcenv binding. The
    /// environment and the lifts therefore cannot disagree about any name that ships — one holds
    /// nothing, and the other is refused.
    #[test]
    fn a_charclass_refused_name_produces_no_binding() {
        let book = "\u{4e2d}pkg__predict() { :; }\n";
        assert!(
            dorc_syntax::parse(book)
                .value
                .iter()
                .all(|(_, n)| !matches!(&n.kind, NodeKind::FuncDef { .. })),
            "the sh parser yields no FuncDef for a name it cannot lex — the premise of this pin"
        );
        let mut table = DefinitionTable::default();
        let _ = add_def(&mut table, 0, "\u{4e2d}pkg__predict");
        let (solved, exit) = solve_book(book, &table);
        assert_eq!(
            solved.binding_before(exit, "\u{4e2d}pkg__predict"),
            Flat::Elem(Binding::Undefined),
            "nothing bound it, so it reads Undefined — never a definition the lifts would then \
             disagree with"
        );
    }

    /// CELL (b), the NON-role weird funcdef in a book. No role machinery is implicated and
    /// `reserved.rs` says nothing, so the only protection is that the silent mis-parse fails
    /// conservative. `dorc_syntax` garbles it into a bare command plus an empty `Subshell` plus a
    /// `Group`, and that phantom subshell injects a real (well-nested, meaningless) scope pair
    /// into the CFG — which the frame stack must simply tolerate.
    ///
    /// ANTI-MASKING: this fails if the cell ever starts licensing. It asserts the name stays
    /// UNBOUND, so no consumer can read a definition off garbage.
    #[test]
    fn a_garbled_non_role_funcdef_walls_and_its_phantom_scope_is_harmless() {
        let book = "\u{4e2d}foo() { :; }\n";
        let mut table = DefinitionTable::default();
        let _ = add_def(&mut table, 0, "\u{4e2d}foo");
        let (solved, exit) = solve_book(book, &table);
        assert!(
            solved.converged(),
            "the phantom scope pair is well-nested, so the solve still reaches a fixed point"
        );
        assert_eq!(
            solved.binding_before(exit, "\u{4e2d}foo"),
            Flat::Elem(Binding::Undefined),
            "a garbled definition binds NOTHING — if this ever reports a definition, the silent \
             mis-parse has started licensing and the cell is no longer failing safe"
        );
        // And the phantom `ScopeEnter`/`ScopeExit` did not corrupt the stack: the exit state is a
        // real frame stack, not the ⊤ a depth mismatch would have produced.
        assert!(
            matches!(solved.before(exit), EnvStack::Frames(_)),
            "a meaningless-but-balanced scope pair must not collapse the environment to ⊤"
        );
    }

    // =======================================================================
    // Structural enforcement — the pre-pass property, made mechanical
    // =======================================================================

    /// SIGNATURE ENFORCEMENT: this module names no records / effect-vector / erasure / verdict
    /// type, so a later fixpoint round has no channel by which to reach it.
    ///
    /// The environment is computed ONCE from the origin model and joins the frozen set. The
    /// fixpoint's ratchet erases EFFECTS; it has no authority over BINDINGS, and a license once
    /// withheld must never be regained by a later round. A lexical census rather than a type
    /// bound because the property is "cannot even be spelled here" — the same reasoning
    /// `licence_mint_has_exactly_one_caller` uses one crate over.
    #[test]
    fn this_module_names_no_fixpoint_reachable_type() {
        let src = include_str!("funcenv.rs");
        // Only the `use` block matters: a type this module cannot import, it cannot take.
        let imports: String = src
            .lines()
            .take_while(|l| !l.starts_with("// ====="))
            .filter(|l| l.trim_start().starts_with("use "))
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in [
            "records",
            "SiteResults",
            "erase",
            "Erasure",
            "verdict",
            "Verdict",
            "effect",
            "SkipClass",
            "Disposition",
        ] {
            assert!(
                !imports.contains(forbidden),
                "`{forbidden}` is reachable from funcenv's imports — the environment would stop \
                 being a pre-pass, and a later round could re-decide which definition was live"
            );
        }
    }

    /// A records-proven-dead branch containing a funcdef must NOT re-run env resolution. Stated
    /// as the property that makes it impossible: the environment is a pure function of
    /// (ast, cfg, defs, source-literals), none of which the fixpoint mutates, so re-solving it
    /// mid-run would be a no-op even if someone wired it — and the census above stops the wiring.
    /// This test pins the determinism half: the same inputs give the same answer, every time.
    #[test]
    fn re_solving_the_same_inputs_is_stable() {
        let book = "f__is_converged() { :; }\nunset -f f__is_converged\n";
        let mut table = DefinitionTable::default();
        let d = add_def(&mut table, 9, "f__is_converged");
        table.set_book_site(first_funcdef(book), d);
        let (first, exit) = solve_book(book, &table);
        let (again, _) = solve_book(book, &table);
        assert_eq!(
            first.binding_before(exit, "f__is_converged"),
            again.binding_before(exit, "f__is_converged")
        );
        assert_eq!(
            first.binding_before(exit, "f__is_converged"),
            Flat::Elem(Binding::Undefined),
            "and `unset -f` really did retire the definition — otherwise this pins nothing"
        );
    }
}
