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

// ── The value-plane seam: funcenv-reads-source-literal-plane-only ──

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

// ── Definitions, and the loaded unit ──

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
    /// The name token's span — where a diagnostic's caret lands, and the operand a shadow
    /// narrative carries (the whole-def span would frame a dozen lines to say one thing).
    pub name_span: Span,
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

    /// Whether the unit holds ANY definition of `name` — the positional gate's APPLICABILITY test
    /// (see [`LiveDefinitions::answers_at`]).
    ///
    /// The environment's universe is exactly these names, so a name outside it has no positional
    /// answer to give and the gate must not manufacture one. In production the table records every
    /// role funcdef `dorc_syntax` sees in every input, so the only names outside the universe are
    /// the ones the two parsers disagree about — a class `reserved.rs` refuses at Error severity
    /// before it can ship (`28O:fnd-two-parsers-disagree-on-funcdefs`).
    #[must_use]
    pub fn knows(&self, name: &str) -> bool {
        self.defs.iter().any(|d| d.name == name)
    }

    fn definitions_of_path(&self, path: &str) -> Option<&[DefId]> {
        self.by_path.get(path).map(Vec::as_slice)
    }
}

// ── The domain ──

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

    /// What the INNERMOST frame alone says about `name` — ⊥ when it says nothing.
    ///
    /// The shadow refusal's whole scope rule (`28K` §1 `rul-scope-by-subshell-resource`): a write
    /// lands in the innermost frame, so it REPLACES only what that frame already held. A binding
    /// in an outer frame is not replaced but shadowed, bounded by the subshell — which is the
    /// sanctioned regional-preference idiom and must never trip the refusal.
    #[must_use]
    pub fn innermost(&self, name: &str) -> Flat<Binding> {
        match self {
            EnvStack::Bottom => Flat::Bottom,
            EnvStack::Top => Flat::Top,
            EnvStack::Frames(frames) => frames
                .last()
                .map_or(Flat::Bottom, |frame| frame.get(&name.to_owned())),
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
            // The outermost frame is the script itself; this only guards a malformed pair.
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
                // Unequal depth = a merge across a scope boundary: no honest pointwise answer.
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

// ── The analysis ──

/// The solved function environment: per program point, which definition each name is bound to.
#[derive(Debug, Clone)]
pub struct FuncEnv {
    states: Vec<EnvStack>,
    converged: bool,
    /// Nodes whose transfer havoc'd the environment because a load could not be resolved
    /// (`28K` §1 rul-unloadable-is-unlicensed). Reported by the caller; recorded here as data so
    /// the kernel mints no diagnostics of its own.
    unresolvable_loads: BTreeSet<CfgNodeId>,
    /// Per RESOLVED `.`/`source` site, the loadable path it names — so the shadow pass can replay
    /// which definitions that statement bound without re-reading the value plane.
    sourced_paths: BTreeMap<CfgNodeId, String>,
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

// ── The positional visibility oracle (`28K` §2 rul-visibility-is-full-positional) ──

/// What every SITE-KEYED consuming act — verdict, predict-at-site, probe-ship, vouch, guard
/// eligibility — reads instead of the lifted sets' own load order (`28K` §2
/// `rul-visibility-is-full-positional`, ACKED spike-tier, human-typed 2026-07-31).
///
/// The rule in one sentence: an act answers at a site only if the definition it would answer FROM
/// is the one a shell executing the book top-to-bottom would have live AT THAT LINE. The naive
/// mental model this preserves is Dorc as a stupid guard-inserter whose inserted text cannot see a
/// definition loaded below it — now applied uniformly, not only to guards. Its named consequence:
/// a definition introduced late in a book licenses NOTHING above itself, no elision, no guard, no
/// vouch.
///
/// VOCABULARY acts (the kind-owner families — `resolve` / `disturbance_reaches_only` /
/// `state_stored_only_in`) are deliberately NOT routed through here: they load from the ambient
/// prefix, single-occupancy, and an in-book vocabulary role refuses with a notice instead
/// (`28M:obl-in-book-vocabulary-role-notice`).
#[derive(Debug, Clone, Copy, Default)]
pub struct LiveDefinitions<'a> {
    bound: Option<(&'a FuncEnv, &'a DefinitionTable)>,
}

impl<'a> LiveDefinitions<'a> {
    /// The positional oracle over a solved environment.
    #[must_use]
    pub fn new(env: &'a FuncEnv, defs: &'a DefinitionTable) -> Self {
        Self {
            bound: Some((env, defs)),
        }
    }

    /// The UNSOLVED unit: no name is known, so every act falls back to the lifted sets' own load
    /// order.
    ///
    /// This is for kernels driven WITHOUT a definition table — the crate's own unit tests, which
    /// build a [`KindIndex`](dorc_oracle::KindIndex) by hand from no source text at all. It is not
    /// a production posture: both drivers construct a real one, and
    /// `both_drivers_solve_a_real_function_environment` fails if either stops.
    #[must_use]
    pub fn unsolved() -> Self {
        Self { bound: None }
    }

    /// The input file whose definition of `name` a shell would have live immediately before
    /// `node` — `None` when nothing is live there (`Undefined`), when the environment cannot say
    /// (⊤), or when the point is unreached (⊥).
    #[must_use]
    pub fn source_before(&self, node: CfgNodeId, name: &str) -> Option<dorc_core::SourceFileId> {
        let (env, defs) = self.bound?;
        match env.binding_before(node, name) {
            Flat::Elem(Binding::Defined(def)) => defs.get(def).map(|d| d.file),
            Flat::Elem(Binding::Undefined) | Flat::Top | Flat::Bottom => None,
        }
    }

    /// Whether source index `file`'s definition of `name` is the one live at `node` — the gate an
    /// act consults before answering from that file's lifted set.
    ///
    /// `file` is an index into the source-ordered vectors, which IS the [`dorc_core::SourceFileId`]
    /// value (`28O:dec-load-order-is-the-id-order`); `source_index_is_the_file_id` pins that.
    ///
    /// **The one permissive answer, stated loudly:** a name the unit has NO definition of answers
    /// `true`, because the environment holds no opinion about it and inventing one would wall every
    /// hand-built index in the workspace. In production that reaches only names the sh parser and
    /// the dialect parser disagree about, which `reserved.rs` refuses before they can ship
    /// (`28O:fnd-two-parsers-disagree-on-funcdefs`); `an_unknown_name_is_not_gated_and_a_known_one_is`
    /// pins both halves.
    #[must_use]
    pub fn answers_at(&self, node: CfgNodeId, name: &str, file: usize) -> bool {
        let Some((_, defs)) = self.bound else {
            return true;
        };
        if !defs.knows(name) {
            return true;
        }
        self.source_before(node, name)
            == Some(dorc_core::SourceFileId(
                u32::try_from(file).unwrap_or(u32::MAX),
            ))
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
    // A capped VALUE solve makes every word ⊤, so nothing could be read: refuse wholesale.
    if !literals.converged() {
        return FuncEnv {
            states: vec![EnvStack::Top; cfg.node_count()],
            converged: false,
            unresolvable_loads: BTreeSet::new(),
            sourced_paths: BTreeMap::new(),
        };
    }
    // Its own pass: independent of the environment, so threading it would buy only interior
    // mutability in a kernel.
    let (unresolvable_loads, sourced_paths) = load_sites(cfg, defs, literals);
    let solution = solve(cfg, Direction::Forward, |node, incoming: &EnvStack| {
        transfer(ast, cfg, defs, literals, &universe, node, incoming)
    });
    FuncEnv {
        states: solution.states,
        converged: solution.converged,
        unresolvable_loads,
        sourced_paths,
    }
}

// ── The cross-unit shadow refusal (`28K` §1 rul-silent-shadowing-refuses) ──

/// One PROVEN cross-unit shadow: `shadowing`'s definition replaced `prior`'s, in the same scope,
/// with no intervening `unset -f` of the name.
///
/// "Same scope" is the whole subtlety, and it is why this is computed over the environment rather
/// than over the text: a definition arriving in an INNER frame (the `( . better-yum.sh; … )`
/// regional-preference idiom) shadows nothing — sh discards it at subshell exit, the outer unit's
/// definition survives intact, and the boundedness IS the spelled intent (`28K` §1
/// `rul-scope-by-subshell-resource`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contest {
    /// The role function name both definitions bind.
    pub name: String,
    /// The definition that was overridden.
    pub prior: DefId,
    /// The definition that overrode it.
    pub shadowing: DefId,
}

/// Every role name whose live binding at the unit's exit is ⊤ — the environment cannot say which
/// definition (if any) a shell would have.
///
/// **Rider 1, `⊤-licenses-nothing`.** This is the load-bearing half of ruling (ii)
/// (`28O:res-polyfill-binding-tops-pending-fold`): the shadow refusal is allowed to UNDER-fire
/// only because an unprovable binding grants nothing either. A name here is withheld exactly as a
/// contested one is — silently, since ⊤ never complains — so the two halves together mean a
/// license requires a PROVEN, uncontested definition and nothing weaker.
///
/// Reached by the half-defining branch, the define-if-absent polyfill the domain cannot fold yet,
/// an unequal-depth join, an unresolvable load, and a non-converged solve alike: they are one
/// world-state ("we cannot say") with one consequence.
#[must_use]
pub fn unprovable(defs: &DefinitionTable, env: &FuncEnv, exit: CfgNodeId) -> BTreeSet<String> {
    let at_exit = env.before(exit);
    defs.names()
        .into_iter()
        .filter(|name| at_exit.lookup(name) == Flat::Top)
        .collect()
}

/// Every proven cross-unit shadow in the unit, in a deterministic order (ambient prefix first,
/// then CFG node order).
///
/// **Ruling (ii), the binding one** (`28O:res-polyfill-binding-tops-pending-fold`): the refusal
/// fires only on a PROVABLE shadow. A ⊤ prior binding — a half-defining branch, a guarded
/// define-if-absent whose condition the domain cannot fold, a capped solve — complains NOT, and
/// (this is the load-bearing half) licenses NOT either: ⊤ reaches no consumer as a definition, so
/// under-firing here grants nothing. A same-file redefinition is NOT a contest: that is the
/// pre-existing within-file refusal (`216` e-1), and minting a second code for it would
/// mis-attribute one world-state to two remediations.
#[must_use]
pub fn contests(ast: &Ast, cfg: &Cfg, defs: &DefinitionTable, env: &FuncEnv) -> Vec<Contest> {
    let mut out = Vec::new();
    if !env.converged() {
        return out;
    }
    // PROVABILITY, read-side: only a shadow whose winner the environment can name at the unit's
    // exit complains. A CONDITIONAL definition joins to ⊤ there — it provably shadowed nothing,
    // and rider 1 withholds it anyway. The write-side half is [`contest_at`]'s.
    let at_exit = env.before(cfg.exit());
    let proven = |name: &str| matches!(at_exit.lookup(name), Flat::Elem(Binding::Defined(_)));
    // The ambient prefix loads inside the ENTRY transfer, so no CFG node witnesses it; walk the
    // same ordered list the transfer applies (`28K` §2: CLI files load "before line 1").
    let mut ambient: BTreeMap<&str, DefId> = BTreeMap::new();
    for &def in &defs.ambient {
        let Some(d) = defs.get(def) else { continue };
        if let Some(prior) = ambient.insert(d.name.as_str(), def)
            && proven(&d.name)
        {
            record(&mut out, defs, &d.name, prior, def);
        }
    }
    for node in 0..cfg.node_count() {
        let id = CfgNodeId(u32::try_from(node).unwrap_or(u32::MAX));
        let cfg_node = cfg.node(id);
        let mut incoming = env.before(id);
        match cfg_node.kind {
            CfgNodeKind::Merge => {
                let NodeKind::FuncDef { name, .. } = &ast.node(cfg_node.ast).kind else {
                    continue;
                };
                let Some(&def) = defs.by_ast.get(&cfg_node.ast) else {
                    continue;
                };
                if proven(name) {
                    contest_at(&mut out, defs, &incoming, name, def);
                }
            }
            CfgNodeKind::Command => {
                // Walking the RUNNING environment (not the entry state) is what makes two
                // same-name definitions in one sourced file read as a within-file redefinition.
                for &def in sourced_definitions(defs, env, id) {
                    let Some(d) = defs.get(def) else { continue };
                    if proven(&d.name) {
                        contest_at(&mut out, defs, &incoming, &d.name, def);
                    }
                    incoming.bind(&d.name, Flat::Elem(Binding::Defined(def)));
                }
            }
            _ => {}
        }
    }
    out
}

/// The definitions a `.`/`source` command at `node` contributes, or empty for any other command.
/// An unresolvable target contributes nothing HERE (it havocs the environment instead, so every
/// name reads ⊤ afterwards and nothing downstream is provable).
fn sourced_definitions<'a>(
    defs: &'a DefinitionTable,
    env: &FuncEnv,
    node: CfgNodeId,
) -> &'a [DefId] {
    env.sourced_paths
        .get(&node)
        .and_then(|path| defs.definitions_of_path(path))
        .unwrap_or(&[])
}

/// Record a contest iff the innermost frame provably held a DIFFERENT unit's definition.
fn contest_at(
    out: &mut Vec<Contest>,
    defs: &DefinitionTable,
    incoming: &EnvStack,
    name: &str,
    shadowing: DefId,
) {
    let Flat::Elem(Binding::Defined(prior)) = incoming.innermost(name) else {
        return; // Undefined ⇒ a free slot (the `unset -f` blessing); ⊥ ⇒ an outer, bounded
        // scope; ⊤ ⇒ unprovable, and ⊤ licenses nothing either.
    };
    record(out, defs, name, prior, shadowing);
}

fn record(
    out: &mut Vec<Contest>,
    defs: &DefinitionTable,
    name: &str,
    prior: DefId,
    shadowing: DefId,
) {
    let (Some(a), Some(b)) = (defs.get(prior), defs.get(shadowing)) else {
        return;
    };
    if a.file == b.file {
        return; // the pre-existing within-file redefinition refusal owns this cell
    }
    out.push(Contest {
        name: name.to_owned(),
        prior,
        shadowing,
    });
}

/// Split every `.`/`source` site into the resolvable and the unresolvable.
///
/// Unresolvable — a dynamic path, a path the driver never read, or a target word carrying anything
/// weaker than source-literal provenance — havocs the environment (`28K` §1
/// rul-unloadable-is-unlicensed); the caller discloses them, since silence licenses nothing. The
/// resolvable half is kept so the shadow pass can replay each statement's bindings.
fn load_sites(
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
) -> (BTreeSet<CfgNodeId>, BTreeMap<CfgNodeId, String>) {
    let mut unresolvable = BTreeSet::new();
    let mut resolved = BTreeMap::new();
    for node in 0..cfg.node_count() {
        let id = CfgNodeId(u32::try_from(node).unwrap_or(u32::MAX));
        if cfg.node(id).kind != CfgNodeKind::Command {
            continue;
        }
        if !matches!(literals.literal_text(id, 0), Some("." | "source")) {
            continue;
        }
        match literals
            .literal_text(id, 1)
            .filter(|target| defs.definitions_of_path(target).is_some())
        {
            Some(target) => drop(resolved.insert(id, target.to_owned())),
            None => drop(unresolvable.insert(id)),
        }
    }
    (unresolvable, resolved)
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
        // Unparsed: may define/unset/source invisibly. Half-modeling it is the DP-8 trap.
        CfgNodeKind::Top => EnvStack::Top,
        // `cfg::lower_funcdef` lowers a definition STATEMENT to a pass-through `Merge` carrying
        // the `FuncDef`'s AstId — the seat where the binding takes effect in the main flow.
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
            // `28K` §1: we cannot know WHICH names an unloaded file defines, so all of it is ⊤.
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
        Binding, DefId, Definition, DefinitionTable, EnvStack, FuncEnv, LiveDefinitions,
        SourceLiteralPlane, analyze,
    };
    use crate::cfg::{self, Cfg, CfgNodeId, CfgNodeKind};
    use crate::lattice::{Flat, Lattice, MapL};
    use dorc_core::{BytePos, Interner, SourceFileId, Span};
    use dorc_syntax::ast::NodeKind;
    use std::collections::{BTreeMap, BTreeSet};

    fn add_def(table: &mut DefinitionTable, file: u32, name: &str) -> DefId {
        table.add(Definition {
            file: SourceFileId(file),
            name: name.to_owned(),
            span: Span::new(BytePos(0), BytePos(1)),
            name_span: Span::new(BytePos(0), BytePos(1)),
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

    // ── TABLE 1: `Undefined` is an ELEMENT, never map-absence ──

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

    // ── TABLE 2: non-convergence folds to ⊤ everywhere ──

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
            sourced_paths: BTreeMap::new(),
        };
        assert_eq!(solved.before(CfgNodeId(0)), EnvStack::Top);
        assert_eq!(solved.binding_before(CfgNodeId(0), "f"), Flat::Top);
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

    // ── The frame stack ──

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

    // ── Containment: the two parsers disagree only where nothing can ship ──

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
        assert!(
            matches!(solved.before(exit), EnvStack::Frames(_)),
            "a meaningless-but-balanced scope pair must not collapse the environment to ⊤"
        );
    }

    // ── Structural enforcement: the pre-pass property, made mechanical ──

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

    // ── TABLE 3: the cross-unit shadow refusal's cells (`28K` §1) ──

    const ROLE: &str = "yum__is_converged";

    /// A unit shaped like a real run: one CLI-named oracle FILE (id 0) in the ambient prefix,
    /// registered under its own path so a book's `. lib.sh` binds the same definition, plus the
    /// book's own role funcdefs (id 1) keyed positionally by their `FuncDef` node.
    fn unit(book: &str, oracle_names: &[&str]) -> (DefinitionTable, DefId) {
        let mut table = DefinitionTable::default();
        let ids: Vec<DefId> = oracle_names
            .iter()
            .map(|name| add_def(&mut table, 0, name))
            .collect();
        table.set_loadable("lib.sh".to_owned(), ids.clone());
        table.extend_ambient(ids.iter().copied());
        let loaded = ids[0];
        for (id, node) in dorc_syntax::parse(book).value.iter() {
            if let NodeKind::FuncDef { name, .. } = &node.kind {
                let def = add_def(&mut table, 1, name);
                table.set_book_site(id, def);
            }
        }
        (table, loaded)
    }

    fn contests_of(book: &str, table: &DefinitionTable) -> Vec<super::Contest> {
        let mut interner = Interner::default();
        let ast = dorc_syntax::parse(book).value;
        let cfg = cfg::build(&ast).value;
        let value = crate::value::analyze(&cfg, &ast, &mut interner);
        let plane = SourceLiteralPlane::new(&value, &interner);
        let env = analyze(&ast, &cfg, table, &plane);
        super::contests(&ast, &cfg, table, &env)
    }

    /// CELL 1 — the unblessed cross-unit shadow: a book redefines a role the loaded oracle already
    /// bound. This is the case the whole refusal exists for, and the corpus's live instance is
    /// `guard23-reingest-collision-verbatim`.
    #[test]
    fn a_book_definition_shadowing_a_loaded_oracle_is_contested() {
        let book = "yum__is_converged() { :; }\nyum install -y nginx\n";
        let (table, loaded) = unit(book, &[ROLE]);
        let found = contests_of(book, &table);
        assert_eq!(found.len(), 1, "one shadow: {found:?}");
        assert_eq!(found[0].name, ROLE);
        assert_eq!(
            found[0].prior, loaded,
            "the OVERRIDDEN definition is the loaded oracle's, not the book's"
        );
    }

    /// CELL 2 — blessed by an intervening `unset -f`, textually between the two definitions
    /// (`28K` §9 rat-blessing-vocabulary-v0: that is the whole blessing set). The book's
    /// definition lands in a free slot, so nothing was silently overridden and the family keeps
    /// its licenses.
    #[test]
    fn an_intervening_unset_f_blesses_the_override() {
        let book = "unset -f yum__is_converged\nyum__is_converged() { :; }\n";
        let (table, _) = unit(book, &[ROLE]);
        assert!(
            contests_of(book, &table).is_empty(),
            "`unset -f` between the definitions is the spelled intent"
        );
    }

    /// CELL 3 — a guarded define-if-absent incoming definition draws NO complaint.
    ///
    /// Written against the OBSERVABLE, not the mechanism, deliberately: today the abstention is
    /// join-⊤ (the domain cannot fold a `command -v` condition, so neither arm is proven and the
    /// binding is ⊤, which never complains); when the decidable-condition fold lands the mechanism
    /// becomes a PROVABLE exemption — the guard is dead, the loaded definition survives — with the
    /// identical outcome, and this test must not move.
    #[test]
    fn a_guarded_define_if_absent_draws_no_complaint() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        assert!(contests_of(book, &table).is_empty());
    }

    /// CELL 4 — two definitions of one name in ONE file are not a cross-unit shadow: that world
    /// state has its own pre-existing refusal (`216` e-1) and its own remediation, and minting a
    /// second code for it would point the author at the wrong repair (`271:rul-sin-ordering`).
    #[test]
    fn a_within_file_redefinition_is_not_this_refusals_business() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let first = add_def(&mut table, 0, ROLE);
        let second = add_def(&mut table, 0, ROLE);
        table.extend_ambient([first, second]);
        assert!(contests_of(book, &table).is_empty());
    }

    /// CELL 5 — the sanctioned regional-preference idiom (`28K` §1
    /// `rul-scope-by-subshell-resource`) does NOT trip the refusal. The re-source binds in an
    /// INNER frame, so sh discards it at subshell exit and the outer unit's definition survives
    /// untouched: the boundedness IS the spelled intent, and complaining about it would tax the
    /// one selection idiom the design offers.
    #[test]
    fn a_subshell_scoped_re_source_does_not_trip_the_refusal() {
        let book = "( . lib.sh; yum install -y nginx )\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.extend_ambient([outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("lib.sh".to_owned(), vec![inner]);
        assert!(contests_of(book, &table).is_empty());
    }

    /// CELL 6a — a TOP-LEVEL cross-unit shadow arriving by the book's own sourcing DOES trip: the
    /// override is unbounded, so appending one `.`-source line would otherwise silently reassign
    /// whose judgment governs the family (`28K` §6 rej-load-order-as-trust-adjudicator).
    #[test]
    fn a_top_level_re_source_trips_the_refusal() {
        let book = ". lib.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.extend_ambient([outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("lib.sh".to_owned(), vec![inner]);
        let found = contests_of(book, &table);
        assert_eq!(found.len(), 1, "the same collision, unbounded: {found:?}");
        assert_eq!(found[0].prior, outer);
        assert_eq!(found[0].shadowing, inner);
    }

    /// CELL 6b — and the same collision arriving by CLI LOAD ORDER, which no CFG node witnesses
    /// (the ambient prefix loads inside the entry transfer). Two named oracle files describing one
    /// family is exactly the plurality `28K` §3 refuses to resolve by the mere act of loading.
    #[test]
    fn two_cli_named_oracles_defining_one_family_trip_the_refusal() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let first = add_def(&mut table, 0, ROLE);
        let second = add_def(&mut table, 1, ROLE);
        table.extend_ambient([first, second]);
        let found = contests_of(book, &table);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!((found[0].prior, found[0].shadowing), (first, second));
    }

    /// RIDER 1, `⊤-licenses-nothing` — the load-bearing half of ruling (ii). A binding the
    /// environment cannot prove is named as unprovable, so the driver withholds the family's
    /// licenses; that is what makes the refusal's permitted UNDER-firing sound, since an
    /// under-caught shadow can then grant nothing either.
    ///
    /// The negative half is on the same test on purpose: an ordinary, unconditional definition is
    /// NOT unprovable, so the withholding cannot be vacuously always-on.
    #[test]
    fn an_unprovable_binding_is_named_and_a_proven_one_is_not() {
        let guarded = "if [ -f /etc/x ]; then yum__is_converged() { :; }; fi\n";
        let (table, _) = unit(guarded, &[ROLE]);
        let mut interner = Interner::default();
        let ast = dorc_syntax::parse(guarded).value;
        let cfg = cfg::build(&ast).value;
        let value = crate::value::analyze(&cfg, &ast, &mut interner);
        let plane = SourceLiteralPlane::new(&value, &interner);
        let env = analyze(&ast, &cfg, &table, &plane);
        assert!(
            super::unprovable(&table, &env, cfg.exit()).contains(ROLE),
            "a half-defining branch leaves the binding ⊤, and ⊤ licenses nothing"
        );

        let plain = "yum install -y nginx\n";
        let (plain_table, _) = unit(plain, &[ROLE]);
        let mut i2 = Interner::default();
        let ast2 = dorc_syntax::parse(plain).value;
        let cfg2 = cfg::build(&ast2).value;
        let value2 = crate::value::analyze(&cfg2, &ast2, &mut i2);
        let plane2 = SourceLiteralPlane::new(&value2, &i2);
        let env2 = analyze(&ast2, &cfg2, &plain_table, &plane2);
        assert!(
            super::unprovable(&plain_table, &env2, cfg2.exit()).is_empty(),
            "an ambient, unconditional definition is provable — otherwise this withholds \
             everything and pins nothing"
        );
    }

    // ── TABLE 4: the full-positional regime (`28K` §2 rul-visibility-is-full-positional) ──

    /// Solve `book` and hand back the pieces a POSITIONAL query needs: the environment, the CFG
    /// (to name a site), and the parsed book.
    fn solve_positional(
        book: &str,
        table: &DefinitionTable,
    ) -> (FuncEnv, Cfg, dorc_syntax::ast::Ast) {
        let mut interner = Interner::default();
        let ast = dorc_syntax::parse(book).value;
        let cfg = cfg::build(&ast).value;
        let value = crate::value::analyze(&cfg, &ast, &mut interner);
        let plane = SourceLiteralPlane::new(&value, &interner);
        let env = analyze(&ast, &cfg, table, &plane);
        (env, cfg, ast)
    }

    /// The `Command` node whose source text is exactly `needle` — named by its bytes rather than
    /// by an ordinal, because a funcdef's own body contributes command nodes too and an ordinal
    /// silently slides onto one of those.
    fn command_at(cfg: &Cfg, ast: &dorc_syntax::ast::Ast, book: &str, needle: &str) -> CfgNodeId {
        use crate::solve::Graph as _;
        (0..cfg.node_count())
            .map(|i| CfgNodeId(u32::try_from(i).unwrap_or(u32::MAX)))
            .find(|id| {
                let node = cfg.node(*id);
                let span = ast.node(node.ast).span;
                node.kind == CfgNodeKind::Command
                    && book.get(span.lo.0 as usize..span.hi.0 as usize) == Some(needle)
            })
            .expect("the book carries that command site")
    }

    /// THE SHARPENED CONSEQUENCE CELL (`28K` §2, human-ACKED 2026-07-31): a definition introduced
    /// LATE in a book licenses NOTHING above itself. The site above it is answered by no file at
    /// all, so every act — elide, guard, vouch, probe-ship — declines there.
    ///
    /// The site BELOW is asserted on the same test so the cell cannot pass by the machinery simply
    /// never answering: the identical definition DOES answer once it is above the site.
    #[test]
    fn a_late_book_definition_answers_below_itself_and_never_above() {
        let book = "yum install -y nginx\nyum__is_converged() { :; }\nyum install -y curl\n";
        let mut table = DefinitionTable::default();
        let def = add_def(&mut table, 1, ROLE);
        table.set_book_site(first_funcdef(book), def);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let above = command_at(&cfg, &ast, book, "yum install -y nginx");
        let below = command_at(&cfg, &ast, book, "yum install -y curl");
        assert_eq!(
            live.source_before(above, ROLE),
            None,
            "a shell reaching line 1 has no such function yet — so nothing may be licensed there"
        );
        assert_eq!(
            live.source_before(below, ROLE),
            Some(SourceFileId(1)),
            "and below the definition it answers, or the cell above pins nothing"
        );
    }

    /// The overwhelmingly common shape, pinned so the conversion cannot quietly wall the world: a
    /// CLI-named oracle loads "before line 1", so its definition is live at EVERY book site and
    /// the positional answer equals the ambient one everywhere.
    #[test]
    fn an_ambient_prefix_definition_answers_at_every_site() {
        let book = "yum install -y nginx\nyum install -y curl\n";
        let (table, loaded) = unit(book, &[ROLE]);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let file = table.get(loaded).expect("the loaded definition").file;
        for site in ["yum install -y nginx", "yum install -y curl"] {
            assert_eq!(
                live.source_before(command_at(&cfg, &ast, book, site), ROLE),
                Some(file)
            );
        }
    }

    /// The regional-preference idiom (`28K` §1 `rul-scope-by-subshell-resource`) read POSITIONALLY:
    /// the re-sourced definition answers INSIDE the subshell and the outer one answers after it.
    /// This is the cell where a positional read and the ambient last-in-load-order read genuinely
    /// disagree, and it is the reason the query is per-site rather than per-unit.
    #[test]
    fn a_subshell_re_source_answers_only_within_its_scope() {
        let book = "( . lib.sh; yum install -y nginx )\nyum install -y curl\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.extend_ambient([outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("lib.sh".to_owned(), vec![inner]);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let inside = command_at(&cfg, &ast, book, "yum install -y nginx");
        let after = command_at(&cfg, &ast, book, "yum install -y curl");
        assert_eq!(
            live.source_before(inside, ROLE),
            Some(SourceFileId(1)),
            "inside the subshell the re-sourced definition is live"
        );
        assert_eq!(
            live.source_before(after, ROLE),
            Some(SourceFileId(0)),
            "and the pop restores the outer one EXACTLY — an ambient last-in-load-order read \
             would answer the inner file at both sites"
        );
    }

    /// The gate's applicability rule, both halves. A name the unit has no definition of is NOT
    /// gated (the environment holds no opinion, and walling it would take out every hand-built
    /// index); a name it DOES know is gated by position.
    #[test]
    fn an_unknown_name_is_not_gated_and_a_known_one_is() {
        let book = "yum install -y nginx\nyum__is_converged() { :; }\n";
        let mut table = DefinitionTable::default();
        let def = add_def(&mut table, 1, ROLE);
        table.set_book_site(first_funcdef(book), def);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let site = command_at(&cfg, &ast, book, "yum install -y nginx");
        assert!(
            live.answers_at(site, "apt_get__predict", 0),
            "an unrecorded name defers to the lifted sets' own load order"
        );
        assert!(
            !live.answers_at(site, ROLE, 1),
            "a recorded name is answered by POSITION — file 1 defines it below this site"
        );
    }

    /// An UNSOLVED oracle answers permissively for every name, which is what lets a kernel unit
    /// test drive `classify` with a hand-built index and no source text. Pinned so the fallback
    /// stays a deliberate, named posture rather than something a caller discovers by accident.
    #[test]
    fn an_unsolved_oracle_gates_nothing() {
        let live = LiveDefinitions::unsolved();
        assert!(live.answers_at(CfgNodeId(0), ROLE, 0));
        assert_eq!(live.source_before(CfgNodeId(0), ROLE), None);
    }

    /// The identity the whole gate rests on (`28O:dec-load-order-is-the-id-order`): a source's
    /// INDEX in the ordered vectors IS its [`SourceFileId`] value. Every consumer converts one to
    /// the other by hand, so a drift here would silently mis-key every positional answer.
    #[test]
    fn source_index_is_the_file_id() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let second = add_def(&mut table, 1, ROLE);
        table.extend_ambient([second]);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let site = command_at(&cfg, &ast, book, "yum install -y nginx");
        assert_eq!(live.source_before(site, ROLE), Some(SourceFileId(1)));
        assert!(live.answers_at(site, ROLE, 1));
        assert!(!live.answers_at(site, ROLE, 0));
    }
}
