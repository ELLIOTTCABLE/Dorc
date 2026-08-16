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
//! **An untrusted solve folds to ⊤ everywhere.** [`solve`](crate::solve)'s termination
//! preconditions are caller-upheld and un-type-enforceable, and an under-approximated function
//! environment is precisely a set of confident wrong answers about whose body runs. The gate is
//! the solve's CERTIFICATION, never the advisory `converged` flag (`302` §1): a cap-tripped
//! answer that still certifies is the least fixpoint and is used, while an uncertified one ⇒
//! every query answers ⊤ (`16P` DP-9, the same bargain `value` strikes).
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
use dorc_syntax::ast::{Ast, NodeKind, WordPart};

use crate::certify::{SolveConsistency, solve_certified};
use crate::cfg::{Branch, Cfg, CfgNodeId, CfgNodeKind};
use crate::lattice::{Flat, Lattice, MapL};
use crate::solve::{Direction, Graph, Solution};
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

    /// How many argv words `node` carries. Read by the fold's arity checks, which must
    /// distinguish "no fourth word" from "a fourth word this plane cannot resolve" — the latter
    /// is a different command and decides nothing.
    #[must_use]
    pub fn argv_len(&self, node: CfgNodeId) -> usize {
        self.value.argv_values(node).len()
    }

    /// Whether the underlying value analysis may be trusted; an untrusted value solve makes every
    /// word ⊤, and this domain must not read confident answers off it. The gate is that solve's
    /// CERTIFICATION, never the advisory `converged` flag (`302` §1).
    #[must_use]
    pub fn trusted(&self) -> bool {
        self.value.trusted()
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

    /// How many definitions of `name` the loaded unit holds — the BODY-OCCUPANCY CENSUS
    /// (`302:rul-certifier-trip-guard-only`).
    ///
    /// Occupancy 1 is the whole question the certifier-trip cleanup asks of a guard, and the
    /// reason it can be asked at all is that this table is built by a plain syntactic walk over
    /// the parsed inputs (`dorc_cli::world::definition_table`) with no solve anywhere in it — so a
    /// trip, which disqualifies the solver and the certifier together, leaves it standing. At
    /// occupancy 1 a guard's body identity was never analysis-CHOSEN: the positional gate can
    /// still withhold (⇒ no vouch ⇒ no guard) or, at worst, name a body the shell has not defined
    /// yet, and a guard that cannot run its check exits non-zero and falls through to the
    /// author's own bytes. At occupancy ≥2 a wrong choice runs somebody ELSE's judgment and can
    /// answer 0 over a mutator that needed to run, which is the under-execute direction.
    ///
    /// Declarations, not distinct bodies: content-dedup would be sharper and is deliberately not
    /// done here, because the conservative miscount costs a guard (over-execute, priority 2) while
    /// the sharp one costs a lookup its triviality.
    #[must_use]
    pub fn occupancy(&self, name: &str) -> usize {
        self.defs.iter().filter(|d| d.name == name).count()
    }

    fn definitions_of_path(&self, path: &str) -> Option<&[DefId]> {
        self.by_path.get(path).map(Vec::as_slice)
    }

    /// The unit-wide identity of `id` — the key every derived row this definition produced is
    /// filed under (`28Q` §1.1; [`dorc_core::DefinitionId`]).
    #[must_use]
    pub fn identity_of(&self, id: DefId) -> Option<dorc_core::DefinitionId> {
        self.get(id)
            .map(|d| dorc_core::DefinitionId::at(d.file, d.span))
    }

    /// What a derived row lifted from `file`'s definition of `name` may be keyed by — the JOIN
    /// between the two parsers that read every source (`28Q` §1.1).
    ///
    /// The dialect parser produces the ROW; `dorc_syntax` produces the DEFINITION this table holds;
    /// and they are joined on `(file, name)`, the only thing both spell identically
    /// (`28O:fnd-two-parsers-disagree-on-funcdefs`). The span therefore rides in from HERE and is
    /// never reconstructed on the row side, which is what keeps the disagreement from becoming a
    /// silent corpus-wide withhold.
    ///
    /// Three answers, each ruled: no definition ⇒ [`Unkeyed`](dorc_core::DefinitionProvenance::Unkeyed)
    /// (the ruled permissive arm — `28P:dec-the-gate-applies-only-to-names-the-unit-knows`); one ⇒
    /// [`Keyed`](dorc_core::DefinitionProvenance::Keyed); more than one ⇒
    /// [`Ambiguous`](dorc_core::DefinitionProvenance::Ambiguous), because the lift keeps ONE row per
    /// `(file, role)` and which of the file's definitions spoke is then unrecoverable.
    #[must_use]
    pub fn provenance_of(
        &self,
        file: dorc_core::SourceFileId,
        name: &str,
    ) -> dorc_core::DefinitionProvenance {
        let mut found = None;
        for def in &self.defs {
            if def.file != file || def.name != name {
                continue;
            }
            if found.is_some() {
                return dorc_core::DefinitionProvenance::Ambiguous;
            }
            found = Some(dorc_core::DefinitionId::at(def.file, def.span));
        }
        found.map_or(
            dorc_core::DefinitionProvenance::Unkeyed,
            dorc_core::DefinitionProvenance::Keyed,
        )
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

/// Why an environment is floored, when it is (`302` §3) — one gate, and it NAMES its cause
/// rather than leaving two independent flags a consumer could read apart.
#[derive(Debug, Clone)]
pub enum EnvFloor {
    /// The value plane is untrusted, so no word could be read: the cascade along the real
    /// dependency (`302` §3.1). No environment solve was attempted.
    ValuePlaneUntrusted,
    /// A round's solve failed its own post-fixpoint check (`302` §3.2). Carries the verdict.
    SolverInconsistent(Box<SolveConsistency<EnvStack>>),
}

/// The solved function environment: per program point, which definition each name is bound to.
#[derive(Debug, Clone)]
pub struct FuncEnv {
    states: Vec<EnvStack>,
    /// `Some` ⇒ every answer is ⊤ and every license is withheld. The ONE gate: `trusted()` is
    /// its absence, so no second flag can disagree with it.
    floor: Option<EnvFloor>,
    /// Nodes whose transfer havoc'd the environment because a load could not be resolved
    /// (`28K` §1 rul-unloadable-is-unlicensed). Reported by the caller; recorded here as data so
    /// the kernel mints no diagnostics of its own.
    unresolvable_loads: BTreeSet<CfgNodeId>,
    /// Per RESOLVED `.`/`source` site, the loadable path it names — so the shadow pass can replay
    /// which definitions that statement bound without re-reading the value plane.
    sourced_paths: BTreeMap<CfgNodeId, String>,
    /// The edges the decidable-condition fold proved dead (`28M` §9). Kept as data so a pin can
    /// assert WHICH condition folded rather than only its downstream effect on a binding — an
    /// empty set is the honest statement that nothing was decidable, and the corpus cell that
    /// must never fold (`contest28-top-licenses-nothing`) is checkable directly.
    folded_edges: BTreeSet<(CfgNodeId, CfgNodeId)>,
}

impl FuncEnv {
    /// The environment IMMEDIATELY BEFORE `node` — the positional regime's query (`28K` §2:
    /// anything standing in for text in the execution stream reads sh execution order).
    ///
    /// Answers ⊤ for everything when the environment is floored.
    #[must_use]
    pub fn before(&self, node: CfgNodeId) -> EnvStack {
        if !self.trusted() {
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
        binding_in(&self.states, self.trusted(), node, name)
    }

    /// May this environment's answers be trusted? Named for what consumers ASK rather than for
    /// the solver flag that used to answer it (`302` §1; see [`crate::value::ValueFlow::trusted`]).
    #[must_use]
    pub fn trusted(&self) -> bool {
        self.floor.is_none()
    }

    /// Why this environment is floored, if it is — the scalars a consumer's degrade record is
    /// built from (`302` §5), and the failing evidence for a pull surface.
    #[must_use]
    pub fn floor(&self) -> Option<&EnvFloor> {
        self.floor.as_ref()
    }

    /// The sites whose load could not be resolved, for the caller to disclose.
    #[must_use]
    pub fn unresolvable_loads(&self) -> &BTreeSet<CfgNodeId> {
        &self.unresolvable_loads
    }

    /// The control-flow edges the decidable-condition fold proved dead (`28M` §9), in
    /// deterministic order. Empty whenever no condition in the unit was decidable.
    #[must_use]
    pub fn folded_edges(&self) -> &BTreeSet<(CfgNodeId, CfgNodeId)> {
        &self.folded_edges
    }
}

/// The binding `name` has immediately before `node`, read off a raw solution — the shared body
/// of [`FuncEnv::binding_before`] and the fold's own per-round queries, so an intermediate round
/// answers by exactly the rule the finished environment answers by (the ⊤-on-non-convergence
/// fold included).
fn binding_in(states: &[EnvStack], converged: bool, node: CfgNodeId, name: &str) -> Flat<Binding> {
    if !converged {
        return Flat::Top;
    }
    states
        .get(node.index())
        .map_or(Flat::Top, |state| state.lookup(name))
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

    /// **The frame lookup** (`28Q` §1.2/§1.3): which DEFINITION a shell would have live for `name`
    /// immediately before `node`.
    ///
    /// This is the only per-frame structure the conversion needs — the environment already computes
    /// it, positionally and scope-stacked — and it is what every resolution seat asks before reading
    /// any derived row. Feed the answer to [`dorc_core::answering_file`] together with the seat's own
    /// candidate rows; that function, not this one, holds the rule.
    ///
    /// Its three answers are the three the seats must tell apart. A definition is
    /// [`Live`](dorc_core::LiveDefinition::Live). `Undefined`, ⊤, and unreached all collapse to
    /// [`Withheld`](dorc_core::LiveDefinition::Withheld) — they differ in cause and agree completely
    /// in consequence. And a name the table does not know, or an unsolved environment, is
    /// [`NoOpinion`](dorc_core::LiveDefinition::NoOpinion): the environment's universe IS the table's
    /// names, so manufacturing an opinion outside it would wall every hand-built index in the
    /// workspace (`28P:dec-the-gate-applies-only-to-names-the-unit-knows`, preserved verbatim).
    #[must_use]
    pub fn definition_before(&self, node: CfgNodeId, name: &str) -> dorc_core::LiveDefinition {
        let Some((env, defs)) = self.bound else {
            return dorc_core::LiveDefinition::NoOpinion;
        };
        if !defs.knows(name) {
            return dorc_core::LiveDefinition::NoOpinion;
        }
        match env.binding_before(node, name) {
            Flat::Elem(Binding::Defined(def)) => defs
                .identity_of(def)
                .map_or(dorc_core::LiveDefinition::Withheld, |id| {
                    dorc_core::LiveDefinition::Live(id)
                }),
            Flat::Elem(Binding::Undefined) | Flat::Top | Flat::Bottom => {
                dorc_core::LiveDefinition::Withheld
            }
        }
    }

    /// What a row lifted from source index `file`'s definition of `name` may be keyed by — the
    /// table's own answer ([`DefinitionTable::provenance_of`]), reached through the oracle a seat
    /// already holds so no seat grows a second parameter.
    ///
    /// Takes the positional INDEX because that is what every seat holds and the index IS the
    /// [`dorc_core::SourceFileId`] (`28O:dec-load-order-is-the-id-order`); doing the crossing here
    /// keeps it in ONE place, exactly as [`custody_of_source_index`] does for custody.
    ///
    /// An unsolved unit answers [`Unkeyed`](dorc_core::DefinitionProvenance::Unkeyed): there is no
    /// table to join against, which is the same "no opinion" posture
    /// [`definition_before`](Self::definition_before) takes from the other side.
    #[must_use]
    pub fn provenance_of(&self, file: usize, name: &str) -> dorc_core::DefinitionProvenance {
        self.bound
            .map_or(dorc_core::DefinitionProvenance::Unkeyed, |(_, defs)| {
                defs.provenance_of(source_file_of_index(file), name)
            })
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

    /// The CUSTODY of `name`'s live definition immediately before `node` — whose utterance an act
    /// answering here would be resting on (`28M` §8; [`dorc_core::DefinitionCustody`]).
    #[must_use]
    pub fn custody_before(
        &self,
        node: CfgNodeId,
        name: &str,
    ) -> Option<dorc_core::DefinitionCustody> {
        self.source_before(node, name)
            .map(dorc_core::DefinitionCustody::of_defining_file)
    }
}

/// The custody a source-ordered vector index denotes (`28O:dec-load-order-is-the-id-order`: the
/// index IS the [`dorc_core::SourceFileId`]). The ONE place the engine crosses from a positional
/// index into the custody vocabulary — every seat that used to compare bare indices routes here,
/// so `28M` §10's possible re-key has one crossing to inspect rather than five.
#[must_use]
pub fn custody_of_source_index(file: usize) -> dorc_core::DefinitionCustody {
    dorc_core::DefinitionCustody::of_defining_file(source_file_of_index(file))
}

/// The [`dorc_core::SourceFileId`] a source-ordered vector index denotes
/// (`28O:dec-load-order-is-the-id-order`). Its twin above crosses into the CUSTODY vocabulary; this
/// one stops at the file id, which is what the definition join needs.
#[must_use]
pub fn source_file_of_index(file: usize) -> dorc_core::SourceFileId {
    dorc_core::SourceFileId(u32::try_from(file).unwrap_or(u32::MAX))
}

/// Solve the function environment over `cfg`.
///
/// Pure: `defs` is the whole loaded unit as data and `literals` is the narrow source-literal
/// window; nothing here reads a clock, a file, or a host answer.
///
/// Two passes, not one — the decidable-condition fold (`28M` §9) is pessimistic conditional
/// constant propagation over this domain: solve, decide whatever conditions the solved
/// environment makes decidable, mask the edges those decisions prove dead, re-solve. See
/// [`FOLD_ROUNDS_CAP`] for why it terminates and why every intermediate state is sound.
#[must_use]
pub fn analyze(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
) -> FuncEnv {
    let universe = defs.names();
    // A capped VALUE solve makes every word ⊤, so nothing could be read: refuse wholesale.
    if !literals.trusted() {
        return funcenv_floor(cfg, EnvFloor::ValuePlaneUntrusted);
    }
    // Its own pass: independent of the environment, so threading it would buy only interior
    // mutability in a kernel.
    let (unresolvable_loads, sourced_paths) = load_sites(cfg, defs, literals);
    let solve_pruned = |folded: &BTreeSet<(CfgNodeId, CfgNodeId)>| {
        let graph = PrunedCfg::new(cfg, folded);
        solve_certified(&graph, Direction::Forward, |node, incoming: &EnvStack| {
            transfer(ast, cfg, defs, literals, &universe, node, incoming)
        })
    };

    match fold_to_environment(solve_pruned, |states| {
        dead_edges(ast, cfg, defs, literals, states, true)
    }) {
        Ok((states, folded_edges)) => FuncEnv {
            states,
            floor: None,
            unresolvable_loads,
            sourced_paths,
            folded_edges,
        },
        Err(consistency) => funcenv_floor(cfg, EnvFloor::SolverInconsistent(consistency)),
    }
}

/// The decidable-condition fold's round loop, over a caller-supplied round-solver.
///
/// Parameterized so `302` §6.8's obligation is OBSERVABLE: a test can hand it a solver whose
/// answer really does fail the REAL certifier and watch the fold break, without any way for a
/// production path to reach a faked verdict.
///
/// `Err` is the grant-shifting guard (`302` §3.2, `303:fnd-never-live-is-the-grant-shifting-
/// consumer`): `never_live` subtracts EXACTLY and so SHIFTS WINNERS, which means a round whose
/// states did not certify must reach neither `dead` nor the next fold. Breaking out is not the
/// same as stopping — stopping would keep both the unchecked states AND the edges already folded
/// from them, and those edges are precisely what would grant.
fn fold_to_environment(
    solve_round: impl Fn(&FoldedEdges) -> (Solution<EnvStack>, SolveConsistency<EnvStack>),
    dead: impl Fn(&[EnvStack]) -> FoldedEdges,
) -> Result<SettledFold, Box<SolveConsistency<EnvStack>>> {
    let mut folded_edges = FoldedEdges::new();
    let (mut solution, mut consistency) = solve_round(&folded_edges);
    for _ in 0..FOLD_ROUNDS_CAP {
        if !consistency.is_consistent() {
            return Err(Box::new(consistency));
        }
        let found = dead(&solution.states);
        if found.is_subset(&folded_edges) {
            break;
        }
        folded_edges.extend(found);
        (solution, consistency) = solve_round(&folded_edges);
    }
    if !consistency.is_consistent() {
        return Err(Box::new(consistency));
    }
    Ok((solution.states, folded_edges))
}

/// The control-flow edges the decidable-condition fold has masked.
type FoldedEdges = BTreeSet<(CfgNodeId, CfgNodeId)>;

/// A fold that settled: the solved states, plus the edges it masked to get there.
type SettledFold = (Vec<EnvStack>, FoldedEdges);

/// The FUNCTION-ENVIRONMENT FLOOR (`302` §3.2) — the one seat every un-trusted environment lands
/// on, and the sharpest floor in the lane.
///
/// All-⊤ states, `trusted()` false, and — the hard rider — **`folded_edges` EMPTY**. The fold
/// must arrive here by BREAKING at the failing round rather than by stopping: `never_live`
/// subtracts exactly and shifts winners (`28P:adj-never-live-exactness-accepted`), so a floor
/// that still carried edges folded from unchecked states would convert a detected engine defect
/// into a LICENSE. Everything the environment can say is withheld: `before` ⇒ ⊤,
/// `unprovable` names every role, `never_live` ⇒ ∅, `contests` ⇒ ∅.
#[must_use]
pub fn funcenv_floor<G: Graph>(graph: &G, floor: EnvFloor) -> FuncEnv {
    FuncEnv {
        states: vec![EnvStack::Top; graph.node_count()],
        floor: Some(floor),
        unresolvable_loads: BTreeSet::new(),
        sourced_paths: BTreeMap::new(),
        folded_edges: BTreeSet::new(),
    }
}

// ── The decidable-condition fold (`28M` §9) ──

/// How many times the fold may re-solve.
///
/// **Termination.** The masked-edge set only GROWS and is bounded by the graph's edge count, so
/// the loop settles after at most one round per maskable edge; the cap is a backstop of
/// [`solve`](crate::solve)'s own flavour, not the real bound. A round is needed only where
/// deciding one condition is what makes the NEXT one decidable — stacked define-if-absent guards
/// — so the practical bound is guard-nesting depth.
///
/// **Why running out is safe** (`28M:dec-pessimistic-iteration`, "always — pessimism is what we
/// do here"): every intermediate state is independently sound. Round *n*'s decisions are taken
/// against an environment solved under round *n-1*'s mask, and masking edges can only remove
/// paths — so a name whose binding was `Defined(d)` keeps binding `d` or becomes unreached, and
/// one that was `Undefined` stays `Undefined` or becomes unreached. A decided condition
/// therefore never flips; running out of rounds loses precision (⊤ ⇒ withhold) and nothing else.
const FOLD_ROUNDS_CAP: usize = 8;

/// How far the fold will unwrap a condition looking for one simple command. Bounded for the same
/// reason every other walk here is: a malformed shape must lose precision, never spin.
const CONDITION_UNWRAP_CAP: usize = 8;

/// A condition this domain can answer — `28M:dec-decidable-set-v0`, CLOSED, and it grows by NAME
/// only. Anything not in this set is ⊤ and folds nothing (`inv-top-reject`).
#[derive(Debug, Clone, PartialEq, Eq)]
enum DecidableTest {
    /// `command -v <literal name>`, where the name is one the unit DEFINES somewhere.
    ///
    /// Contracted to function-definedness within the analysis unit
    /// (`28M:rul-command-v-reads-fn-definedness`, human-restated: "for analysis, `command -v`
    /// will never check for a binary named `cmd__is_converged`"). A PATH executable of the same
    /// name is pathological-by-construction and is `28K:bitem8`'s reserved differential case.
    /// The universe restriction is what keeps the ordinary `command -v yum` — a genuine,
    /// host-dependent PATH question — out: a name the unit never defines has no binding to read.
    FunctionDefined(String),
    /// `[ -f <literal> ]` / `test -f <literal>` naming a path the CONTROLLER resolved as a
    /// loadable source.
    ///
    /// Decides TRUE only. Absence from the load set is not filesystem absence — the driver knows
    /// only what it was told to read — so an unrecognized path stays ⊤ and
    /// `28K:res-host-conditional-loading` is untouched. Deciding a RESOLVED path true adds no
    /// assumption the loading model did not already make: `. lib.sh` already binds the
    /// definitions the controller read from that path.
    LoadableExists,
}

/// The edges every decidable condition in `cfg` proves dead, against the environment `states`.
fn dead_edges(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    states: &[EnvStack],
    converged: bool,
) -> BTreeSet<(CfgNodeId, CfgNodeId)> {
    let mut out = BTreeSet::new();
    for branch in cfg.branches() {
        let Some(held) = decide(ast, cfg, defs, literals, states, converged, branch) else {
            continue;
        };
        for dead in branch.dead_successors(cfg, held) {
            out.insert((branch.decided_at, dead));
        }
    }
    out
}

/// Whether `branch`'s condition provably succeeds (`Some(true)`) or provably fails
/// (`Some(false)`) at its own position. `None` is every other case, which is most of them.
fn decide(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    states: &[EnvStack],
    converged: bool,
    branch: &Branch,
) -> Option<bool> {
    let (test, negated) = decidable_test(ast, cfg, defs, literals, branch)?;
    let holds = match test {
        DecidableTest::FunctionDefined(name) => {
            match binding_in(states, converged, branch.decided_at, &name) {
                Flat::Elem(Binding::Defined(_)) => true,
                Flat::Elem(Binding::Undefined) => false,
                // ⊤ cannot say; ⊥ means the condition itself is unreached, so it decides nothing.
                Flat::Top | Flat::Bottom => return None,
            }
        }
        DecidableTest::LoadableExists => true,
    };
    Some(holds != negated)
}

/// Classify `branch`'s condition against [`DecidableTest`], with the `!`-negation the shape
/// carries. `None` unless the condition is exactly one simple command in the closed set.
///
/// Two structural refusals earn their place. The decisive CFG node must BE the simple command the
/// AST names — where it is not (a compound condition, a pipeline, an `&&` chain whose left arm is
/// itself a branch) the decision would be keyed to somebody else's status. And redirections are
/// their own CFG nodes, so `>/dev/null 2>&1` never reaches the argv: `dec-decidable-set-v0`'s
/// "rc-irrelevant redirects ignored" falls out rather than being special-cased.
fn decidable_test(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    branch: &Branch,
) -> Option<(DecidableTest, bool)> {
    let (simple, negated) = condition_shape(ast, branch.cond)?;
    let node = branch.decided_at;
    if cfg.node(node).kind != CfgNodeKind::Command || cfg.node(node).ast != simple {
        return None;
    }
    let test = match command_head(ast, simple)? {
        "command" if literals.argv_len(node) == 3 && literals.literal_text(node, 1)? == "-v" => {
            let name = literals.literal_text(node, 2)?;
            if !defs.knows(name) {
                return None;
            }
            DecidableTest::FunctionDefined(name.to_owned())
        }
        "test" if literals.argv_len(node) == 3 => file_test(defs, literals, node, None)?,
        "[" if literals.argv_len(node) == 4 => file_test(defs, literals, node, Some("]"))?,
        _ => return None,
    };
    Some((test, negated))
}

/// The command word of `simple`, when it is one statically-fixed literal
/// ([`Word::as_literal`](dorc_syntax::ast::Word::as_literal), the analyzer's standing rule for
/// command names).
///
/// Read as PROGRAM TEXT rather than through [`SourceLiteralPlane`] for one reason: `[` carries a
/// glob metacharacter, so the value plane holds every `[ … ]` head at ⊤ — its correct,
/// conservative pathname-expansion posture at a use site — and `[` is the spelling this test is
/// overwhelmingly written in, so honouring only `test` would leave the U-shaped middle the
/// dialect rules warn about. The narrowness is the safety: only the HEAD comes from here, and
/// only to name which builtin the command is. Every OPERAND — the role name, the load path —
/// still resolves through the plane, so `funcenv-reads-source-literal-plane-only`'s actual
/// subject (a value a HOST spoke siting a load) is untouched.
fn command_head(ast: &Ast, simple: AstId) -> Option<&str> {
    let NodeKind::Simple { words, .. } = &ast.node(simple).kind else {
        return None;
    };
    let NodeKind::Word { parts } = &ast.node(*words.first()?).kind else {
        return None;
    };
    match parts.as_slice() {
        [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s.as_str()),
        _ => None,
    }
}

/// The `-f <loadable path>` half of the decidable set, shared by the `test` and `[` spellings.
fn file_test(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    closer: Option<&str>,
) -> Option<DecidableTest> {
    if literals.literal_text(node, 1)? != "-f" || literals.literal_text(node, 3) != closer {
        return None;
    }
    let path = literals.literal_text(node, 2)?;
    defs.definitions_of_path(path)
        .map(|_| DecidableTest::LoadableExists)
}

/// Peel a condition down to the single simple command whose status decides it, accumulating
/// `!`-negation. Anything with two commands in it decides nothing: the fold reads ONE rc.
fn condition_shape(ast: &Ast, cond: AstId) -> Option<(AstId, bool)> {
    let mut id = cond;
    let mut negated = false;
    for _ in 0..CONDITION_UNWRAP_CAP {
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
            NodeKind::Simple { .. } => return Some((id, negated)),
            _ => return None,
        }
    }
    None
}

/// The CFG with the fold's proven-dead edges removed — the graph the environment is actually
/// solved over.
///
/// A view rather than a rewrite of the [`Cfg`]: the graph the rest of the engine sees is
/// untouched, so the fold's reach is exactly this one domain and no consumer inherits a pruned
/// CFG it did not ask for.
struct PrunedCfg {
    succ: Vec<Vec<usize>>,
    pred: Vec<Vec<usize>>,
}

impl PrunedCfg {
    fn new(cfg: &Cfg, folded: &BTreeSet<(CfgNodeId, CfgNodeId)>) -> Self {
        let n = cfg.node_count();
        let node_id = |v: usize| CfgNodeId(u32::try_from(v).unwrap_or(u32::MAX));
        let succ: Vec<Vec<usize>> = (0..n)
            .map(|v| {
                cfg.succ(v)
                    .iter()
                    .copied()
                    .filter(|&w| !folded.contains(&(node_id(v), node_id(w))))
                    .collect()
            })
            .collect();
        let mut pred = vec![Vec::new(); n];
        for (v, targets) in succ.iter().enumerate() {
            for &w in targets {
                if let Some(preds) = pred.get_mut(w) {
                    preds.push(v);
                }
            }
        }
        PrunedCfg { succ, pred }
    }
}

impl Graph for PrunedCfg {
    fn node_count(&self) -> usize {
        self.succ.len()
    }
    fn succ(&self, node: usize) -> &[usize] {
        &self.succ[node]
    }
    fn pred(&self, node: usize) -> &[usize] {
        &self.pred[node]
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
/// Reached by the half-defining branch, a guard whose condition falls outside the fold's closed
/// decidable set, an unequal-depth join, an unresolvable load, and a non-converged solve alike:
/// they are one world-state ("we cannot say") with one consequence.
#[must_use]
pub fn unprovable(defs: &DefinitionTable, env: &FuncEnv, exit: CfgNodeId) -> BTreeSet<String> {
    let at_exit = env.before(exit);
    defs.names()
        .into_iter()
        .filter(|name| at_exit.lookup(name) == Flat::Top)
        .collect()
}

/// Every `(role name, source file)` whose definition the environment proves binds at NO program
/// point — a definition no execution of this unit could call, however late it sits in load order.
///
/// **Why anything still needs this.** Every SITE-KEYED act now resolves through
/// [`LiveDefinitions::definition_before`], and a never-live definition is named at no frame, so no
/// resolution seat can reach its rows: the withdrawal this used to drive retired with the
/// conversion (`28Q` §1). What did NOT retire is the ONE whole-unit fold resolution does not cover
/// — `dorc_oracle::build_dialect`'s minting scan, which asks which selector tokens the unit's
/// authors minted AT ALL (`28Q` §9 `pin-two-position-sparing`). That question has no frame to ask
/// from, and a define-if-absent body the fold proved dead would otherwise mint vocabulary no
/// execution could have uttered — enlarging or shifting the sparing dialect, which spares MORE.
///
/// The exactness matters, because removal SHIFTS the minting winner rather than merely
/// withholding: this is not a conservative filter, it must be RIGHT. It is — a definition no
/// program point binds is one no execution can call. Empty when the solve did not converge, since
/// every binding is then ⊤ and [`unprovable`] withholds those families outright.
///
/// Grouped per `(name, file)` because a file may hold two definitions of one name — the
/// within-file redefinition the `216` e-1 refusal owns — and the pair is dead only when every one
/// of them is.
#[must_use]
pub fn never_live(
    defs: &DefinitionTable,
    env: &FuncEnv,
) -> BTreeSet<(String, dorc_core::SourceFileId)> {
    let mut out = BTreeSet::new();
    if !env.trusted() {
        return out;
    }
    let mut live: BTreeSet<DefId> = BTreeSet::new();
    for name in defs.names() {
        for state in &env.states {
            if let Flat::Elem(Binding::Defined(def)) = state.lookup(&name) {
                live.insert(def);
            }
        }
    }
    let mut per_key: BTreeMap<(String, dorc_core::SourceFileId), Vec<DefId>> = BTreeMap::new();
    for (index, def) in defs.defs.iter().enumerate() {
        per_key
            .entry((def.name.clone(), def.file))
            .or_default()
            .push(DefId(u32::try_from(index).unwrap_or(u32::MAX)));
    }
    for (key, ids) in per_key {
        if ids.iter().all(|id| !live.contains(id)) {
            out.insert(key);
        }
    }
    out
}

/// Every proven cross-unit shadow in the unit, in a deterministic order (ambient prefix first,
/// then CFG node order).
///
/// **Ruling (ii), the binding one** (`28O:res-polyfill-binding-tops-pending-fold`): the refusal
/// fires only on a PROVABLE shadow. A ⊤ prior binding — a half-defining branch, a guard whose
/// condition falls outside the fold's decidable set, a capped solve — complains NOT, and (this is
/// the load-bearing half) licenses NOT either: ⊤ reaches no consumer as a definition, so
/// under-firing here grants nothing. A same-file redefinition is NOT a contest: that is the
/// pre-existing within-file refusal (`216` e-1), and minting a second code for it would
/// mis-attribute one world-state to two remediations.
///
/// The fold narrows what stays ⊤ in BOTH directions, deliberately: a define-if-absent guard the
/// fold proves dead now draws no complaint by PROOF rather than by abstention, and a
/// define-if-PRESENT override it proves live now draws the complaint the rule always meant.
#[must_use]
pub fn contests(ast: &Ast, cfg: &Cfg, defs: &DefinitionTable, env: &FuncEnv) -> Vec<Contest> {
    let mut out = Vec::new();
    if !env.trusted() {
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
///
/// An UNREACHED node produces ⊥. Havoc is what an EXECUTED unmodeled construct does to the
/// environment; a node no path reaches executes nothing, so reading ⊤ off one would let a
/// provably-dead branch poison the join it never reaches — exactly what the fold masks edges to
/// prevent. `Entry` is exempt because minting the boundary state out of ⊥ is its whole job.
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
    if matches!(incoming, EnvStack::Bottom) && cfg_node.kind != CfgNodeKind::Entry {
        return EnvStack::Bottom;
    }
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
            floor: Some(super::EnvFloor::ValuePlaneUntrusted),
            unresolvable_loads: BTreeSet::new(),
            sourced_paths: BTreeMap::new(),
            folded_edges: BTreeSet::new(),
        };
        assert_eq!(solved.before(CfgNodeId(0)), EnvStack::Top);
        assert_eq!(solved.binding_before(CfgNodeId(0), "f"), Flat::Top);
        assert_eq!(solved.binding_before(CfgNodeId(99), "f"), Flat::Top);
    }

    /// `302` §6.8 — the FOLD BREAKS at the failing round, and the floor it lands on carries
    /// **`folded_edges = ∅`**.
    ///
    /// This is the lane's sharpest correctness obligation, and it stands INDEPENDENT of any one
    /// consumer: under true resolution every environment answer SHIFTS WINNERS (`28Q` §1 — it
    /// selects whose judgment governs a site, with no agreement veto behind it), and `never_live`
    /// shifts the dialect's minting winner besides. Edges folded from states that never certified
    /// would therefore GRANT on unchecked evidence — a detected engine defect converted into a
    /// license. Merely stopping the fold would leave exactly those edges behind, so the test
    /// asserts the break, not the stop.
    ///
    /// The certifier is REAL and unmocked (anti-masking): the round-solver hands back a genuinely
    /// perturbed solution and `certify_solution` is what judges it. Only the SOLVER is faulted —
    /// which is precisely `302` §6.1's fault-injection shape, applied one layer up.
    #[test]
    fn the_fold_breaks_to_its_floor_at_the_failing_round() {
        use super::{EnvFloor, fold_to_environment, funcenv_floor};
        use crate::certify::certify_solution;
        use crate::solve::{Direction, Graph, Solution};

        struct OneNode;
        impl Graph for OneNode {
            fn node_count(&self) -> usize {
                1
            }
            fn succ(&self, _: usize) -> &[usize] {
                &[0]
            }
            fn pred(&self, _: usize) -> &[usize] {
                &[0]
            }
        }
        let rounds = std::cell::Cell::new(0usize);
        let solve_round = |_: &BTreeSet<(CfgNodeId, CfgNodeId)>| {
            rounds.set(rounds.get().saturating_add(1));
            let mut solution = Solution {
                states: vec![EnvStack::Bottom],
                converged: true,
                rounds: 1,
            };
            // Raise the state above what the transfer can justify: `transfer(0, ⊥) = ⊥`, and
            // `⊥ ⊑ Top` holds, so we instead lower the STATE below its own transferred output by
            // seeding Top and transferring Top — the edge check then compares Top ⊑ Bottom.
            solution.states = vec![EnvStack::Bottom];
            let perturbed_transfer = |_: usize, _: &EnvStack| EnvStack::Top;
            let consistency = certify_solution(
                &OneNode,
                Direction::Forward,
                &[EnvStack::Bottom],
                perturbed_transfer,
                &solution,
            );
            (solution, consistency)
        };

        let outcome = fold_to_environment(solve_round, |_| BTreeSet::new());
        let consistency = outcome.expect_err("a non-certifying round must break the fold");
        assert!(!consistency.is_consistent());
        assert_eq!(rounds.get(), 1, "the fold broke at the FIRST failing round");

        let floored = funcenv_floor(&OneNode, EnvFloor::SolverInconsistent(consistency));
        assert!(!floored.trusted(), "the floor withholds every answer");
        assert!(
            floored.folded_edges().is_empty(),
            "THE RIDER: a floor carrying folded edges would grant on unchecked states"
        );
        assert_eq!(floored.before(CfgNodeId(0)), EnvStack::Top);
        // What the environment can say is exactly nothing.
        assert!(floored.unresolvable_loads().is_empty());
        assert!(matches!(
            floored.floor(),
            Some(EnvFloor::SolverInconsistent(_))
        ));
    }

    /// A converged environment with an out-of-range node still answers ⊤ — the same reasoning,
    /// on the other side of the convergence flag.
    #[test]
    fn an_out_of_range_node_answers_top_even_when_converged() {
        let (solved, _) = solve_book("true\n", &DefinitionTable::default());
        assert!(solved.trusted());
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
            solved.trusted(),
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

    /// CELL 3 — a guarded define-if-absent incoming definition draws NO complaint. The DECIDABLE
    /// subcase, which is now exempt by PROOF rather than by abstention: the fold reads the
    /// condition false at its own position, the guarded arm's edge is dead, and no shadow ever
    /// occurred. `28K` §1's "guarded incoming definitions are exempt as a consequence, not a
    /// blessing" is delivered as written.
    ///
    /// Written against the OBSERVABLE, not the mechanism, deliberately — it did not move when the
    /// mechanism under it changed, which is the property that made it worth writing that way.
    #[test]
    fn a_guarded_define_if_absent_draws_no_complaint() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        assert!(contests_of(book, &table).is_empty());
    }

    /// CELL 3, the UNDECIDABLE subcase, pinned beside its twin because the exemption now has two
    /// independent sources and losing either would be a silent regression. Here the condition is
    /// outside the decidable set, so nothing folds and the binding joins to ⊤ — which complains
    /// not, and (rider 1) licenses not either.
    #[test]
    fn an_undecidable_guard_draws_no_complaint_by_joining_to_top() {
        let book = "if [ -f /etc/dorc/prefer-local ]; then\nyum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        assert!(contests_of(book, &table).is_empty());
        let (solved, exit) = solve_book(book, &table);
        assert_eq!(
            solved.binding_before(exit, ROLE),
            Flat::Top,
            "and it is ⊤ that is doing the work here, not a dead edge"
        );
    }

    /// The refusal's UNDER-complaint half closing (`28O:res-polyfill-binding-tops-pending-fold`,
    /// gap one): a define-if-PRESENT guard over a loaded oracle really does override it, and the
    /// fold makes that shadow PROVABLE — so a cross-unit override that previously slipped past
    /// under ⊤ now draws the complaint the rule always meant it to.
    #[test]
    fn a_define_if_present_guard_proves_the_shadow() {
        let book = "if command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\n";
        let (table, loaded) = unit(book, &[ROLE]);
        let found = contests_of(book, &table);
        assert_eq!(found.len(), 1, "the override is now proven: {found:?}");
        assert_eq!(found[0].prior, loaded);
    }

    /// The whole-unit resolution's half of the fold: a definition the environment proves binds
    /// nowhere is named, so `dorc_oracle::live_source` stops counting it as this file DECLARING
    /// the role. Without this the P1 cure reaches the binding and stops there — the guard file
    /// still wins the ambient answer by being last, and every site withholds on disagreement.
    ///
    /// The negative half rides along: the definition that IS live is never named, so the
    /// subtraction cannot be vacuously everything.
    #[test]
    fn a_dead_guard_is_named_never_live_and_the_live_one_is_not() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\nyum install -y nginx\n";
        let (table, _) = unit(book, &[ROLE]);
        let (solved, _) = solve_book(book, &table);
        let dead = super::never_live(&table, &solved);
        assert!(
            dead.contains(&(ROLE.to_owned(), SourceFileId(1))),
            "the book's guarded definition binds at no program point: {dead:?}"
        );
        assert!(
            !dead.contains(&(ROLE.to_owned(), SourceFileId(0))),
            "the loaded oracle's definition is live everywhere and must survive"
        );
    }

    /// And its containment: with nothing folded, NOTHING is named never-live. A subtraction that
    /// fired on an ordinary single-definition unit would silently re-key every site in the corpus.
    #[test]
    fn an_ordinary_unit_names_nothing_never_live() {
        let book = "yum install -y nginx\n";
        let (table, _) = unit(book, &[ROLE]);
        let (solved, _) = solve_book(book, &table);
        assert!(super::never_live(&table, &solved).is_empty());
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

    /// The universe rule, both halves. A name the unit has no definition of gets NO OPINION (the
    /// environment holds none, and walling it would take out every hand-built index); a name it
    /// DOES know is answered by POSITION, and a definition below the site withholds.
    #[test]
    fn an_unknown_name_is_not_gated_and_a_known_one_is() {
        let book = "yum install -y nginx\nyum__is_converged() { :; }\n";
        let mut table = DefinitionTable::default();
        let def = add_def(&mut table, 1, ROLE);
        table.set_book_site(first_funcdef(book), def);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let site = command_at(&cfg, &ast, book, "yum install -y nginx");
        assert_eq!(
            live.definition_before(site, "apt_get__predict"),
            dorc_core::LiveDefinition::NoOpinion,
            "an unrecorded name gets no manufactured opinion — the row answers on its own \
             provenance alone"
        );
        assert_eq!(
            live.definition_before(site, ROLE),
            dorc_core::LiveDefinition::Withheld,
            "a recorded name is answered by POSITION — file 1 defines it below this site"
        );
    }

    /// An UNSOLVED oracle answers permissively for every name, which is what lets a kernel unit
    /// test drive `classify` with a hand-built index and no source text. Pinned so the fallback
    /// stays a deliberate, named posture rather than something a caller discovers by accident.
    #[test]
    fn an_unsolved_oracle_gates_nothing() {
        let live = LiveDefinitions::unsolved();
        assert_eq!(live.source_before(CfgNodeId(0), ROLE), None);
        assert_eq!(
            live.definition_before(CfgNodeId(0), ROLE),
            dorc_core::LiveDefinition::NoOpinion,
            "no environment ⇒ no opinion, never a withhold — a withhold would wall every \
             hand-built index in the workspace"
        );
    }

    // ── TABLE 4b: the frame lookup and the two-parser join (`28Q` §1) ──

    /// A definition at a caller-chosen span, so two definitions of one name in one file are
    /// DISTINCT — which `add_def`'s fixed span cannot express and the `Ambiguous` join needs.
    fn add_def_spanned(table: &mut DefinitionTable, file: u32, name: &str, lo: u32) -> DefId {
        table.add(Definition {
            file: SourceFileId(file),
            name: name.to_owned(),
            span: Span::new(BytePos(lo), BytePos(lo)),
            name_span: Span::new(BytePos(lo), BytePos(lo)),
        })
    }

    /// The frame lookup answers with the DEFINITION, and a subshell re-source moves that answer —
    /// the same world `a_subshell_re_source_answers_only_within_its_scope` reads through the
    /// file-shaped accessor, now read through the identity the derived rows are keyed by. Two
    /// spellings of one question, and this is the one the conversion consumes.
    #[test]
    fn the_frame_lookup_names_the_definition_live_at_each_site() {
        let book = "( . lib.sh; yum install -y nginx )\nyum install -y curl\n";
        let mut table = DefinitionTable::default();
        let outer = add_def_spanned(&mut table, 0, ROLE, 10);
        table.extend_ambient([outer]);
        let inner = add_def_spanned(&mut table, 1, ROLE, 20);
        table.set_loadable("lib.sh".to_owned(), vec![inner]);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let inside = command_at(&cfg, &ast, book, "yum install -y nginx");
        let after = command_at(&cfg, &ast, book, "yum install -y curl");
        assert_eq!(
            live.definition_before(inside, ROLE),
            dorc_core::LiveDefinition::Live(table.identity_of(inner).expect("inner id"))
        );
        assert_eq!(
            live.definition_before(after, ROLE),
            dorc_core::LiveDefinition::Live(table.identity_of(outer).expect("outer id")),
            "the pop restores the outer DEFINITION exactly, not merely its file"
        );
    }

    /// A site above the only definition WITHHOLDS, and a name the table never heard of holds NO
    /// OPINION. Both halves matter and they are opposite answers: withhold licenses nothing, while
    /// no-opinion defers to whatever provenance the row itself carries. Collapsing them would
    /// either wall every hand-built index or license a site the shell would not answer at.
    #[test]
    fn withheld_and_no_opinion_are_told_apart() {
        let book = "yum install -y nginx\nyum__is_converged() { :; }\n";
        let mut table = DefinitionTable::default();
        let def = add_def_spanned(&mut table, 1, ROLE, 21);
        table.set_book_site(first_funcdef(book), def);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let site = command_at(&cfg, &ast, book, "yum install -y nginx");
        assert_eq!(
            live.definition_before(site, ROLE),
            dorc_core::LiveDefinition::Withheld,
            "the definition sits BELOW this site, so nothing is live here"
        );
        assert_eq!(
            live.definition_before(site, "apt_get__predict"),
            dorc_core::LiveDefinition::NoOpinion,
            "a name outside the environment's universe gets no manufactured opinion"
        );
    }

    /// The join's three answers, over one table. `Keyed` is the ordinary case; `Unkeyed` is the
    /// two-parser disagreement (a row whose munged funcname the sh parser never recorded) and the
    /// hand-built posture; `Ambiguous` is a file holding TWO definitions of one role, where the
    /// lift keeps one row and which definition spoke is unrecoverable.
    #[test]
    fn the_join_tells_keyed_unkeyed_and_ambiguous_apart() {
        let mut table = DefinitionTable::default();
        let sole = add_def_spanned(&mut table, 0, ROLE, 10);
        add_def_spanned(&mut table, 1, ROLE, 20);
        add_def_spanned(&mut table, 1, ROLE, 40);
        assert_eq!(
            table.provenance_of(SourceFileId(0), ROLE),
            dorc_core::DefinitionProvenance::Keyed(table.identity_of(sole).expect("sole id"))
        );
        assert_eq!(
            table.provenance_of(SourceFileId(1), ROLE),
            dorc_core::DefinitionProvenance::Ambiguous,
            "two definitions of one role in one file: the surviving row cannot name its author"
        );
        assert_eq!(
            table.provenance_of(SourceFileId(2), ROLE),
            dorc_core::DefinitionProvenance::Unkeyed,
            "a file the table records nothing for holds no opinion about the row"
        );
    }

    // ── TABLE 5: the decidable-condition fold (`28M` §9) ──

    /// The exit binding of `ROLE`, plus how many edges the fold proved dead. Every cell below
    /// reads both: the binding is the deliverable, and an empty fold set is what distinguishes
    /// "the lattice was read" from "the answer came out right for some other reason".
    fn folded(book: &str, table: &DefinitionTable) -> (Flat<Binding>, usize) {
        let (solved, exit) = solve_book(book, table);
        assert!(solved.trusted(), "the fold must still reach a fixed point");
        (
            solved.binding_before(exit, ROLE),
            solved.folded_edges().len(),
        )
    }

    /// A book plus a `lib.sh` the book may source, both defining `ROLE`: file 0 is the loadable,
    /// file 1 the book. Unlike [`unit`] the loadable is NOT ambient — the book decides when it
    /// loads, which is what the conditional-sourcing cells need.
    fn sourceable(book: &str) -> (DefinitionTable, DefId) {
        let mut table = DefinitionTable::default();
        let lib = add_def(&mut table, 0, ROLE);
        table.set_loadable("lib.sh".to_owned(), vec![lib]);
        for (id, node) in dorc_syntax::parse(book).value.iter() {
            if let NodeKind::FuncDef { name, .. } = &node.kind {
                let def = add_def(&mut table, 1, name);
                table.set_book_site(id, def);
            }
        }
        (table, lib)
    }

    /// THE P1 CELL (`28O:res-polyfill-binding-tops-pending-fold`, half two — the larger half):
    /// a polite author's guarded default, loaded AFTER a real oracle. Before the fold this
    /// joined `Defined(oracle) ⊔ Defined(guard)` to ⊤ and the family went silently sparing-inert
    /// — "the polite author's file quietly poisons the family it deferred to". The condition is
    /// decidable-FALSE at its own position, so the guarded edge is dead and the loaded oracle
    /// survives intact.
    #[test]
    fn a_guard_loaded_after_a_real_oracle_leaves_that_oracle_live() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\nyum install -y nginx\n";
        let (table, loaded) = unit(book, &[ROLE]);
        let (binding, folds) = folded(book, &table);
        assert_eq!(
            binding,
            Flat::Elem(Binding::Defined(loaded)),
            "the unconditional definition wins — the poison is cured, not merely conservative"
        );
        assert_eq!(folds, 1, "exactly the guarded arm's edge");
    }

    /// THE P2 CELL: the same guard with NO prior definition. The condition is decidable-TRUE, so
    /// the fall-through edge is the dead one and the guard's own definition binds concretely
    /// where it used to join to ⊤. Without masking the fall-through this reads
    /// `Defined(guard) ⊔ Undefined = ⊤`, which is the pre-fold answer.
    #[test]
    fn a_fresh_polyfill_binds_its_own_definition() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let guard = add_def(&mut table, 1, ROLE);
        table.set_book_site(first_funcdef(book), guard);
        let (binding, folds) = folded(book, &table);
        assert_eq!(binding, Flat::Elem(Binding::Defined(guard)));
        assert_eq!(folds, 1);
    }

    /// The lattice's other textual order, which the fold must not disturb: guard FIRST, real
    /// oracle sourced after. Specific still beats default — here because the later unconditional
    /// binding overwrites whatever the guard left, fold or no fold.
    #[test]
    fn a_guard_above_the_real_oracle_still_loses_to_it() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\n. lib.sh\nyum install -y nginx\n";
        let (table, lib) = sourceable(book);
        assert_eq!(folded(book, &table).0, Flat::Elem(Binding::Defined(lib)));
    }

    /// Guards among themselves resolve FIRST-wins (`28K` §3): the second guard's condition is
    /// decidable-FALSE because the first one already bound the name.
    #[test]
    fn two_guards_resolve_first_wins() {
        let book = "if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\n\
                    if ! command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { true ;}\nfi\n";
        let parsed = dorc_syntax::parse(book).value;
        let mut table = DefinitionTable::default();
        let mut first = None;
        for (id, node) in parsed.iter() {
            if let NodeKind::FuncDef { name, .. } = &node.kind {
                let def = add_def(&mut table, 1, name);
                table.set_book_site(id, def);
                first.get_or_insert(def);
            }
        }
        let first = first.expect("two definitions");
        let (binding, folds) = folded(book, &table);
        assert_eq!(
            binding,
            Flat::Elem(Binding::Defined(first)),
            "the FIRST guard's body is the live one — the second guard is dead"
        );
        assert_eq!(folds, 2, "one edge per guard, in opposite senses");
    }

    /// The conditional-sourcing idiom (`28K` §3's admin toolkit) folds on the same rail: with the
    /// name already live, the `|| . backup.sh` operand is dead.
    #[test]
    fn conditional_sourcing_does_not_load_when_the_name_is_already_live() {
        let book = "command -v yum__is_converged >/dev/null 2>&1 || . lib.sh\n";
        let (mut table, lib) = sourceable(book);
        let ambient = add_def(&mut table, 2, ROLE);
        table.extend_ambient([ambient]);
        let (binding, folds) = folded(book, &table);
        assert_eq!(
            binding,
            Flat::Elem(Binding::Defined(ambient)),
            "the ambient definition survives; the backup file never loads"
        );
        assert_ne!(binding, Flat::Elem(Binding::Defined(lib)));
        assert_eq!(folds, 1);
    }

    /// And its live half: with nothing bound, the same line DOES load the backup.
    #[test]
    fn conditional_sourcing_loads_when_the_name_is_absent() {
        let book = "command -v yum__is_converged >/dev/null 2>&1 || . lib.sh\n";
        let (table, lib) = sourceable(book);
        assert_eq!(folded(book, &table).0, Flat::Elem(Binding::Defined(lib)));
    }

    /// `[ -f <loadable> ] && . <loadable>` — the file-test half of the decidable set. The path is
    /// one the CONTROLLER resolved, so the test is decidable-TRUE and the load is certain.
    #[test]
    fn a_file_test_on_a_resolved_loadable_decides_true() {
        let book = "[ -f lib.sh ] && . lib.sh\n";
        let (table, lib) = sourceable(book);
        let (binding, folds) = folded(book, &table);
        assert_eq!(binding, Flat::Elem(Binding::Defined(lib)));
        assert_eq!(folds, 1, "the short-circuit-to-merge edge is the dead one");
    }

    /// THE CELL THAT MUST NOT MOVE (`contest28-top-licenses-nothing`): a live FILESYSTEM test on
    /// a path the controller never resolved. Absence from the load set is not absence from the
    /// disk — the driver knows only what it was told to read — so this stays ⊤ forever and keeps
    /// pinning that ⊤ licenses nothing. If this ever folds, the decidable set has been widened.
    #[test]
    fn a_file_test_on_an_unresolved_path_never_folds() {
        let book = "if [ -f /etc/dorc/prefer-local ]; then\nyum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        let (binding, folds) = folded(book, &table);
        assert_eq!(binding, Flat::Top);
        assert_eq!(folds, 0, "nothing about this condition is decidable");
    }

    /// `command -v` on a name the unit never DEFINES is a genuine, host-dependent PATH question
    /// and stays ⊤ (`res-host-conditional-loading` untouched). This is the containment on
    /// `28M:rul-command-v-reads-fn-definedness`: the contract binds role names the unit binds,
    /// never an arbitrary word.
    #[test]
    fn command_v_on_a_name_outside_the_unit_decides_nothing() {
        let book = "if ! command -v yum >/dev/null 2>&1; then\nyum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        let (binding, folds) = folded(book, &table);
        assert_eq!(binding, Flat::Top, "a PATH probe decides no branch");
        assert_eq!(folds, 0);
    }

    /// The negation is read, not assumed: define-if-PRESENT is the same test with the `!` gone,
    /// and its branch is decidable-TRUE where the polyfill's is decidable-FALSE.
    #[test]
    fn dropping_the_negation_flips_which_arm_dies() {
        let book = "if command -v yum__is_converged >/dev/null 2>&1; then\n\
                    yum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        let (binding, folds) = folded(book, &table);
        let book_def = match binding {
            Flat::Elem(Binding::Defined(d)) => d,
            other => panic!("the book's definition is live, not {other:?}"),
        };
        assert_eq!(
            table.get(book_def).map(|d| d.file),
            Some(SourceFileId(1)),
            "the guarded body really does run when its condition holds"
        );
        assert_eq!(folds, 1);
    }

    /// A condition with TWO commands in it decides nothing — the fold reads ONE rc, and a
    /// compound condition's status is somebody else's. Pessimism, per `dec-pessimistic-iteration`.
    #[test]
    fn a_compound_condition_decides_nothing() {
        let book = "if true; ! command -v yum__is_converged; then\n\
                    yum__is_converged() { :; }\nfi\n";
        let (table, _) = unit(book, &[ROLE]);
        assert_eq!(folded(book, &table).1, 0);
    }

    // ── TABLE 5: value-flow source targets (`28K` §1 rul-unloadable-is-unlicensed, the
    // richness half; `28K` §10 bitem9) ──

    /// THE ITEM, as an A/B pair one spelling apart. `LIB=./oracles; . "$LIB/lib.sh"` binds
    /// exactly what `. ./oracles/lib.sh` binds — same definition, same positional answer at a
    /// site below it — because the target resolves through the SAME
    /// [`SourceLiteralPlane`] window every other operand already used. No second resolver
    /// exists, and this test fails if one is ever introduced beside it: the two spellings
    /// would then be free to disagree.
    #[test]
    fn a_variable_resolved_source_target_binds_what_the_literal_spelling_binds() {
        let spellings = [
            ". ./oracles/lib.sh\nyum install -y nginx\n",
            "LIB=./oracles\n. \"$LIB/lib.sh\"\nyum install -y nginx\n",
        ];
        for book in spellings {
            let mut table = DefinitionTable::default();
            let lib = add_def(&mut table, 0, ROLE);
            table.set_loadable("./oracles/lib.sh".to_owned(), vec![lib]);
            let (env, cfg, ast) = solve_positional(book, &table);
            assert_eq!(
                env.binding_before(cfg.exit(), ROLE),
                Flat::Elem(Binding::Defined(lib)),
                "{book:?} must bind the loaded definition"
            );
            let live = LiveDefinitions::new(&env, &table);
            let site = command_at(&cfg, &ast, book, "yum install -y nginx");
            assert_eq!(
                live.source_before(site, ROLE),
                Some(SourceFileId(0)),
                "{book:?}: the whole positional regime applies — nothing is special-cased for \
                 the variable spelling, so the resolved path carries the SAME SourceFileId \
                 provenance a literal one does"
            );
        }
    }

    /// The other half, and the one that must never move: a target the value plane cannot
    /// resolve stays ⊤ and is disclosed as an unresolvable load. Here `LIB` is never assigned,
    /// so `"$LIB/lib.sh"` is ⊤ at the site — the richness cut widens what RESOLVES, never what
    /// is decidable, and an unresolved target walls exactly as it did before (`28K` §1
    /// rul-unloadable-is-unlicensed).
    #[test]
    fn an_unresolvable_variable_source_target_still_tops_the_family() {
        let book = ". \"$LIB/lib.sh\"\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let lib = add_def(&mut table, 0, ROLE);
        table.set_loadable("./oracles/lib.sh".to_owned(), vec![lib]);
        let (env, cfg, ast) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), ROLE), Flat::Top);
        assert!(
            super::unprovable(&table, &env, cfg.exit()).contains(ROLE),
            "⊤ licenses nothing, and the family must be named so the driver withholds it"
        );
        assert_eq!(
            env.unresolvable_loads(),
            &BTreeSet::from([command_at(&cfg, &ast, book, ". \"$LIB/lib.sh\"")]),
            "and the site is disclosed rather than silently walling"
        );
    }

    /// A resolvable target the CONTROLLER never read is the same ⊤ by a different route, and it
    /// is the cell that keeps the richness cut honest: resolving a PATH is not learning what
    /// lives at it. Absence from the load set is not filesystem absence
    /// (`28K:res-host-conditional-loading` untouched).
    #[test]
    fn a_resolved_target_the_controller_never_read_is_an_unresolvable_load() {
        let book = "LIB=/etc/hork\n. \"$LIB/env\"\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let lib = add_def(&mut table, 0, ROLE);
        table.set_loadable("./oracles/lib.sh".to_owned(), vec![lib]);
        let (env, cfg, ast) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), ROLE), Flat::Top);
        assert_eq!(
            env.unresolvable_loads(),
            &BTreeSet::from([command_at(&cfg, &ast, book, ". \"$LIB/env\"")]),
        );
    }

    /// The shadow refusal reads a variable-resolved load exactly as it reads a literal one —
    /// the regime applies whole, so a cross-unit override arriving through `"$LIB/lib.sh"`
    /// draws the same complaint `a_top_level_re_source_trips_the_refusal` pins for `. lib.sh`.
    /// Without this, widening the resolvable set would have widened the SILENT set with it.
    #[test]
    fn a_variable_resolved_load_trips_the_shadow_refusal() {
        let book = "LIB=.\n. \"$LIB/lib.sh\"\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.extend_ambient([outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("./lib.sh".to_owned(), vec![inner]);
        let found = contests_of(book, &table);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!((found[0].prior, found[0].shadowing), (outer, inner));
    }

    /// The fence, asserted rather than argued (`28M:dec-decidable-set-v0`, CLOSED): the file
    /// test's operand ALREADY resolved through the plane, so a variable-spelled `[ -f … ]`
    /// decides exactly what a literal-spelled one decides, and bitem9 widened nothing here.
    /// The book carries no `.` statement at all, which is what makes that claim checkable —
    /// this cell is untouched by the source-target cut and must read identically to
    /// `a_file_test_on_a_resolved_loadable_decides_true`.
    #[test]
    fn a_variable_spelled_file_test_decides_what_the_literal_one_decides() {
        let book = "LIB=.\n[ -f \"$LIB/lib.sh\" ] && yum__is_converged() { :; }\n";
        let mut table = DefinitionTable::default();
        let lib = add_def(&mut table, 0, ROLE);
        table.set_loadable("./lib.sh".to_owned(), vec![lib]);
        for (id, node) in dorc_syntax::parse(book).value.iter() {
            if let NodeKind::FuncDef { name, .. } = &node.kind {
                let def = add_def(&mut table, 1, name);
                table.set_book_site(id, def);
            }
        }
        assert_eq!(
            folded(book, &table).1,
            1,
            "the short-circuit edge folds, exactly as the literal spelling's does"
        );
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
        // The same identity read through the definition join the seats consume: a row lifted from
        // index 1 keys to the definition the frame names, and index 0's does not.
        assert_eq!(
            live.definition_before(site, ROLE),
            dorc_core::LiveDefinition::Live(table.identity_of(second).expect("second id"))
        );
        assert_eq!(
            live.provenance_of(1, ROLE),
            dorc_core::DefinitionProvenance::Keyed(table.identity_of(second).expect("second id"))
        );
        assert_eq!(
            live.provenance_of(0, ROLE),
            dorc_core::DefinitionProvenance::Unkeyed,
            "index 0 records no definition of this role, so it can key no row"
        );
    }
}
