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
use crate::load::LoadAccount;
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

/// May a value of this provenance site a load (`funcenv-reads-source-literal-plane-only`)? ONLY
/// program text — `Register` is the tempting one to admit and is refused with the rest, because
/// which oracle answers a site would then rest on something outside the program text.
const fn admits_a_load(grade: ValueGrade) -> bool {
    matches!(grade, ValueGrade::ProgramText)
}

/// The provenance the VARIABLE plane carries whole-hog: `ValueEnv` records no per-variable grade,
/// so the wall rests on its writers being source literals. A constant rather than a sentence — when
/// `seam-re-bind` folds a captured value in, this line is what stops being true and the gate above
/// then refuses everything.
const VARIABLE_PLANE_GRADE: ValueGrade = ValueGrade::ProgramText;

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
        if !grades.get(index).copied().is_some_and(admits_a_load) {
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

    /// The source-literal value of shell variable `name` immediately before `node`, or `None` when
    /// the plane cannot say.
    ///
    /// THE seat the static loader expands a sourced file's own `.` operand through
    /// (`30I:force-root-value-flow`): `SM_ORACLE_ROOT` is assigned in the book and read inside the
    /// package, so the operand has no CFG node of its own and the argv accessors above cannot
    /// reach it. Same window, same trust gate, same wall — see
    /// [`ValueFlow::variable_before`](crate::value::ValueFlow::variable_before) for the grade
    /// obligation this inherits when captured values land.
    #[must_use]
    pub fn variable_text(&self, node: CfgNodeId, name: &str) -> Option<String> {
        if !admits_a_load(VARIABLE_PLANE_GRADE) {
            return None;
        }
        match self.value.variable_before(node, name) {
            Flat::Elem(text) => Some(text),
            Flat::Top | Flat::Bottom => None,
        }
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

/// The authored book path `$0` names, and the invocation spellings the analysis must hold for
/// (`30P:model-symbolic-dollar-zero`).
///
/// NEVER realpath'd — sh-parity under symlinks — and never read from a shell. `$0` is a fact about
/// how Dorc was invoked and how Dorc will invoke what it ships
/// (`30P:rul-dorc-invokes-in-a-modelled-live-spelling`), so no host answer has a route into one.
///
/// It rides [`DefinitionTable`] beside its cwd for that type's own reason: the load answer and the
/// definitions it binds must be ONE fact. Both spellings evaluate against the SAME modelled cwd —
/// the spelling varies `$0`'s string and nothing else.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScriptSpellings {
    slash_bearing: Option<String>,
    slashless: Option<String>,
}

/// Which invocation named the book (`30P:model-symbolic-dollar-zero`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Spelling {
    /// `sh /srv/book.sh` / `sh ./plan.sh` — the spelling Dorc itself invokes.
    SlashBearing,
    /// `sh book.sh`. Live ONLY when the book's own directory is the modelled load cwd: elsewhere
    /// that command could not have found the book at all, so it is not a possible invocation.
    Slashless,
}

impl ScriptSpellings {
    /// The spellings a book at `path` has under `cwd`.
    ///
    /// The path is put through [`dorc_core::loadpath::normalize`] first, so the two development
    /// platforms' spellings of one path answer alike — a `\`-bearing controller path has no `/` to
    /// trim and would otherwise make `${0%/*}` the whole word on one leg only.
    #[must_use]
    pub fn of(path: &str, cwd: &dorc_core::loadpath::Cwd) -> Self {
        let normalized = dorc_core::loadpath::normalize(path);
        if normalized.is_empty() {
            return Self::default();
        }
        let base = normalized
            .rsplit('/')
            .next()
            .unwrap_or(&normalized)
            .to_owned();
        let here = cwd.resolve_operand(&base);
        let slashless =
            (here.is_some() && here == cwd.resolve_operand(&normalized)).then_some(base);
        let slash_bearing = if normalized.contains('/') {
            normalized
        } else {
            format!("./{normalized}")
        };
        Self {
            slash_bearing: Some(slash_bearing),
            slashless,
        }
    }

    /// What `$0` holds under `spelling`, or `None` when that spelling is not live here.
    #[must_use]
    pub fn text(&self, spelling: Spelling) -> Option<&str> {
        match spelling {
            Spelling::SlashBearing => self.slash_bearing.as_deref(),
            Spelling::Slashless => self.slashless.as_deref(),
        }
    }

    /// Every live spelling, deterministically ordered, invoking-spelling first.
    pub fn live(&self) -> impl Iterator<Item = Spelling> {
        [Spelling::SlashBearing, Spelling::Slashless]
            .into_iter()
            .filter(|&s| self.text(s).is_some())
    }
}

/// One source the invocation named to load: a pre-source, whose whole top-level program runs
/// before the book's first line.
#[derive(Debug, Clone)]
struct AmbientRoot {
    /// The canonical key its program is filed under, when the modeled cwd could name one.
    key: Option<String>,
    /// Its declarations in file order — the binding this root contributes when no program is filed
    /// for it, which is every unmarked source and any unit whose path would not canonicalize.
    defs: Vec<DefId>,
}

/// Every definition in the analysis unit, plus the load structure over them.
///
/// Built at the cli edge (the only place allowed to read files) and handed in whole, so this
/// module stays a pure function of its inputs (`inv-determinism`).
#[derive(Debug, Clone, Default)]
pub struct DefinitionTable {
    defs: Vec<Definition>,
    /// The modeled working directory every `.` operand in this unit resolves against
    /// (`30I:rul-dot-resolves-as-sh`). Carried on the table rather than passed per query because
    /// the load answer and the definitions it binds must be one fact: a caller that could supply a
    /// different cwd to the resolver than to the loader is a caller that can make them disagree.
    cwd: dorc_core::loadpath::Cwd,
    /// What `$0` names in this unit, per live invocation spelling — carried here for the same
    /// reason as [`Self::cwd`], and beside it because an operand built from `$0` resolves through
    /// that cwd.
    spellings: ScriptSpellings,
    /// Per loadable path — in CANONICAL form, keyed through [`Self::cwd`] — what the controller
    /// holds there ([`crate::load::Loadable`]): a dorc-lang file's own top level as the closed
    /// program the loader interprets at each load site, or an ordinary sh file acquired for its
    /// BYTES alone. A file whose top level is a flat list of declarations is the degenerate
    /// program, and applying it left-to-right reproduces sh's last-wins exactly as before.
    by_path: BTreeMap<String, crate::load::Loadable>,
    /// The ambient prefix: the CLI-named sources, in command-line order (`28K` §2 — they load
    /// "before line 1"; `30I:rul-pre-source-is-dot-prelude` — each one is an ordinary `.`).
    ambient: Vec<AmbientRoot>,
    /// For a definition sited in the BOOK, the `FuncDef` AST node that writes it. The book's
    /// definitions execute positionally, so the transfer needs to go from "this definition
    /// statement just ran" to "which definition that is".
    by_ast: BTreeMap<AstId, DefId>,
    /// Every variable name the BOOK assigns, anywhere in its text.
    ///
    /// A NAME census, never a value: the sentinel recognition needs to know whether any unit
    /// OUTSIDE a package could have populated the value a guard tests, and the value plane cannot
    /// answer that — an assignment whose value it reads as ⊤ is invisible to it, and an assignment
    /// below the load point is invisible to it at the load point (`30I` §3.4).
    book_assigns: BTreeSet<String>,
}

impl DefinitionTable {
    /// An empty table whose loads resolve against `cwd`, in a unit whose `$0` is `spellings`.
    ///
    /// The parameter is DEMANDED rather than defaulted so a table carrying a cwd but no `$0` is
    /// unrepresentable: the two are one fact, and a caller free to supply only the first could make
    /// an operand's directory and its file disagree.
    #[must_use]
    pub fn rooted_at(cwd: dorc_core::loadpath::Cwd, spellings: ScriptSpellings) -> Self {
        Self {
            cwd,
            spellings,
            ..Self::default()
        }
    }

    /// The modeled working directory this unit's loads resolve against.
    #[must_use]
    pub const fn cwd(&self) -> &dorc_core::loadpath::Cwd {
        &self.cwd
    }

    /// What `$0` names in this unit, per live invocation spelling.
    #[must_use]
    pub const fn spellings(&self) -> &ScriptSpellings {
        &self.spellings
    }

    /// Record a definition and return its id.
    pub fn add(&mut self, def: Definition) -> DefId {
        let id = DefId(u32::try_from(self.defs.len()).unwrap_or(u32::MAX));
        self.defs.push(def);
        id
    }

    /// Declare that `path`, when sourced, runs `program`. `path` is the spelling the invocation
    /// used; it is filed under its canonical form, so the same file named relatively here and
    /// sourced absolutely from a book is ONE entry.
    pub fn set_loadable(&mut self, path: &str, program: crate::load::LoadProgram) {
        if let Some(key) = self.cwd.resolve_operand(path) {
            self.by_path
                .insert(key, crate::load::Loadable::Program(program));
        }
    }

    /// Declare that `path` is an ordinary sh file the controller READ and does not model
    /// (`30P:principle-book-code-source-is-inclusion`, r30's acquire-and-ship slice).
    ///
    /// It runs no program: the site havocs exactly as an unread one does, and this entry exists
    /// only so the load account can carry the OCCURRENCE the artifact mirrors.
    pub fn set_included(&mut self, path: &str) {
        if let Some(key) = self.cwd.resolve_operand(path) {
            self.by_path.insert(key, crate::load::Loadable::Included);
        }
    }

    /// Append a CLI-named source to the ambient prefix, in invocation order.
    ///
    /// `path` is the spelling the invocation used; the entry transfer runs the program filed under
    /// its canonical form, so a pre-source behaves as the `.` it is
    /// (`30I:rul-pre-source-is-dot-prelude`) — its include guard decides, its own `.` loads, its
    /// `unset -f` removes. `defs` is the flat declaration list, which is what binds when this unit
    /// has no program on file (an unmarked source, or a cwd that could name no key).
    pub fn push_ambient(&mut self, path: &str, defs: Vec<DefId>) {
        self.ambient.push(AmbientRoot {
            key: self.cwd.resolve_operand(path),
            defs,
        });
    }

    /// Bind a BOOK definition to the `FuncDef` AST node that writes it.
    pub fn set_book_site(&mut self, ast: AstId, def: DefId) {
        self.by_ast.insert(ast, def);
    }

    /// Record every variable name the book assigns — see [`Self::book_assigns`].
    pub fn set_book_assigns(&mut self, names: impl IntoIterator<Item = String>) {
        self.book_assigns = names.into_iter().collect();
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

    /// Whether the unit holds ANY definition of `name` — the frame lookup's APPLICABILITY test
    /// (see [`LiveDefinitions::definition_before`], whose `NoOpinion` arm this decides).
    ///
    /// The environment's universe is exactly these names, so a name outside it has no positional
    /// answer to give and the lookup must not manufacture one. In production the table records every
    /// role funcdef `dorc_syntax` sees in every input, so the only names outside the universe are
    /// the ones the two parsers disagree about (`28O:fnd-two-parsers-disagree-on-funcdefs`) — a class
    /// `reserved.rs` MARKS at Error severity without refusing the run, so such a row still ships and
    /// still answers on its own provenance (`307c:fnd-reserved-name-error-does-not-refuse`; the
    /// row simply carries its own `DefinitionId` like any other).
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

    /// The program a canonical key names, for a nested load already resolved to one.
    fn program_at_key(&self, key: &str) -> Option<&crate::load::LoadProgram> {
        self.by_path.get(key)?.program()
    }

    /// Is this key an ordinary sh file the controller acquired but does not model?
    ///
    /// The ONE seat that tells an acquired inclusion from a target nobody read: both answer `None`
    /// to every program question above — which is what keeps the wall and the havoc identical —
    /// and only this one has bytes for the artifact to mirror.
    fn included_at_key(&self, key: &str) -> bool {
        matches!(self.by_path.get(key), Some(crate::load::Loadable::Included))
    }

    /// What a path OPERAND names — the `[ -f <path> ]` half of the decidable set, where the word
    /// is a filesystem operand rather than a `.` target and so carries no slash-less refusal.
    fn program_of_path_operand(&self, path: &str) -> Option<&crate::load::LoadProgram> {
        self.by_path
            .get(&self.cwd.resolve_operand(path)?)?
            .program()
    }

    /// Is a NON-FINAL component of `key` a file this unit holds — the DEAD reading
    /// (`30P:rul-dead-spelling-is-not-unsound`)?
    ///
    /// `${0%/*}` of a slashless `$0` is the whole word, so the operand becomes
    /// `book.sh/helpers.sh`, and a `.` under a path whose directory is a FILE cannot succeed. The
    /// book's own path counts as much as any loadable does — it is the one the trap is spelled
    /// over.
    fn a_non_final_component_is_a_file(&self, key: &str) -> bool {
        let book = self
            .spellings
            .text(Spelling::SlashBearing)
            .and_then(|path| self.cwd.resolve_operand(path));
        let mut probe = key;
        while let Some((parent, _)) = probe.rsplit_once('/') {
            if parent.is_empty() {
                return false;
            }
            if self.by_path.contains_key(parent) || book.as_deref() == Some(parent) {
                return true;
            }
            probe = parent;
        }
        false
    }

    /// The EXACT TARGET CLOSURE of `key`: that program plus everything it transitively loads,
    /// by canonical key (`30I` §3.4). `None` when the table does not hold `key` at all.
    ///
    /// Operands expand against the guard's own loading context, which is the same expansion the
    /// loader performs — so this closure is the set of files that load if the guard's fallback
    /// runs, and nothing else. An operand this context cannot read contributes NOTHING rather than
    /// a guess, which shrinks the closure and therefore only ever withholds
    /// ([`Self::sole_populator`] reads it as "the value was populated somewhere I cannot see").
    fn load_closure_of(
        &self,
        key: &str,
        locals: &BTreeMap<String, String>,
        ambient: &impl Fn(&str) -> Option<String>,
    ) -> Option<BTreeSet<String>> {
        self.program_at_key(key)?;
        let mut closure = BTreeSet::new();
        let mut frontier = vec![key.to_owned()];
        while let Some(next) = frontier.pop() {
            let Some(program) = self.program_at_key(&next) else {
                continue;
            };
            if !closure.insert(next) {
                continue;
            }
            for (target, _) in program.load_targets() {
                if let Some(reached) = target
                    .expand(locals, ambient)
                    .and_then(|text| self.cwd.resolve_dot(&text))
                {
                    frontier.push(reached);
                }
            }
        }
        Some(closure)
    }

    /// Is `closure` the ONLY thing in the authored world that assigns `name` — and does it assign
    /// it at all (`30I` §3.4's two `Must` questions, the value half)?
    ///
    /// Both halves, because either alone is forgeable. A copied sentinel assignment with no load
    /// makes the reuse arm reachable without the package; an assignment nowhere at all makes the
    /// condition vacuous. Together they mean the only way the tested value can be live is that this
    /// exact package really loaded.
    ///
    /// The book counts as an outside unit ([`Self::book_assigns`]), and so does any loadable the
    /// closure does not contain.
    fn sole_populator(&self, name: &str, closure: &BTreeSet<String>) -> bool {
        if self.book_assigns.contains(name) {
            return false;
        }
        let mut inside = false;
        for (key, loadable) in &self.by_path {
            if !loadable
                .program()
                .is_some_and(|program| program.assigns(name))
            {
                continue;
            }
            if closure.contains(key) {
                inside = true;
            } else {
                return false;
            }
        }
        inside
    }

    /// The value `closure` assigns to `name` — the live constant a sentinel guard's comparison
    /// really reads (`30I:rul-load-semantics-stay-full-fidelity`).
    ///
    /// Asked only where [`Self::sole_populator`] already proved this closure is the world's only
    /// writer of `name`, so "what the closure assigns" IS what a shell would hold once it ran.
    /// Withholds where two files INSIDE the closure both write it: which one wins is a load-order
    /// question this seat may not answer (`28K` §6), and the value is exactly what the guard
    /// compares.
    fn sentinel_value(&self, name: &str, closure: &BTreeSet<String>) -> Option<String> {
        let mut writers = closure
            .iter()
            .filter_map(|key| self.program_at_key(key))
            .filter(|program| program.assigns(name));
        let program = writers.next()?;
        if writers.next().is_some() {
            return None;
        }
        program.last_literal_assignment(name)
    }

    /// Does any loadable program `unset -f` a name `closure` declares?
    ///
    /// One of the named ways the sentinel shape can mislead (`30I` §3.4's dynamism list): a removal
    /// and redefine elsewhere in the loaded world means the reuse arm's binding is not the target's
    /// after all. Its presence withholds rather than being modelled, because modelling it exactly
    /// is the general load-order question the door is deliberately narrow about.
    fn anything_removes(&self, closure: &BTreeSet<String>) -> bool {
        let declared: BTreeSet<&str> = closure
            .iter()
            .filter_map(|key| self.program_at_key(key))
            .flat_map(crate::load::LoadProgram::declarations)
            .filter_map(|def| self.get(def).map(|d| d.name.as_str()))
            .collect();
        self.by_path
            .values()
            .filter_map(crate::load::Loadable::program)
            .any(|program| program.removes_any(&declared))
    }

    /// The unit-wide identity of `id` — the key every derived row this definition produced is
    /// filed under (`28Q` §1.1; [`dorc_core::DefinitionId`]).
    #[must_use]
    pub fn identity_of(&self, id: DefId) -> Option<dorc_core::DefinitionId> {
        self.get(id)
            .map(|d| dorc_core::DefinitionId::at(d.file, d.span))
    }

    /// Whether the table holds a definition at exactly this identity — the SPAN AGREEMENT the
    /// definition-grade keying rests on (`28Q` §1.1).
    ///
    /// Nothing in production resolution calls this: a derived row carries its own
    /// [`dorc_core::DefinitionId`], minted from the span its own lift recorded, so there is no join
    /// to perform. What the census calls it for is the property underneath that — the dialect
    /// parser and `dorc_syntax` recording one funcdef at one byte range — which is measured rather
    /// than constructed and would otherwise fail silently
    /// (`28O:fnd-two-parsers-disagree-on-funcdefs` is the one class where they legitimately differ,
    /// and it differs by NAME, so the table simply does not know it and the frame holds no opinion).
    #[must_use]
    pub fn holds(&self, id: dorc_core::DefinitionId) -> bool {
        self.defs
            .iter()
            .any(|d| d.file == id.file() && d.span == id.span())
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

    /// Every name the unit knows becomes unknown HERE and no further
    /// (`30P:principle-unknown-source-is-a-point-havoc`).
    ///
    /// POINTWISE, and in the INNERMOST frame, because that is where a write lands and where a
    /// later definition overwrites it — which is the whole delta: sh's `.` may define anything,
    /// and sh's next top-level `f() { … }` re-binds `f` to those bytes whatever ran before it.
    /// [`EnvStack::Top`] stays what it honestly is: a stack whose SHAPE is unknown too.
    ///
    /// Two consequences the tests pin. `ScopeExit`'s pop discards this, so a `.` inside `( … )`
    /// binds nothing outside — sh, exactly. And a name OUTSIDE the universe is untouched: an
    /// unknown file redefining a TOOL as a function is the same cell as the host's `PATH`
    /// resolving that tool to anything, which `30P:rul-guard-resolves-like-its-mutation` places on
    /// the admin's side of the horizon.
    ///
    /// MONOTONE (the worklist's caller-upheld precondition): the operation raises a FIXED set of
    /// keys to ⊤ in one frame. For `x ⊑ y` both `Frames` of equal depth, raising the same keys in
    /// both preserves the pointwise order; `Frames(_) ⊑ Top` is unchanged since `Top` has no frame
    /// to write into; unequal depths are incomparable and join to `Top`, so the obligation is
    /// vacuous there. No new lattice value exists, so the finite height is unchanged.
    fn havoc_names(&mut self, universe: &BTreeSet<String>) {
        for name in universe {
            self.bind(name, Flat::Top);
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
    /// The two load-head LINT populations, recorded as data for the same reason: sites whose head
    /// is dead under a slashless invocation, and sites whose operand is a `PATH` search.
    dies_slashless: BTreeSet<CfgNodeId>,
    searches_path: BTreeSet<CfgNodeId>,
    /// Per havoc'd site, WHY — minted where the reason is known rather than reconstructed at a
    /// diagnostic seat, which is the whole point of [`HavocCause`] carrying its operands.
    havoc_causes: BTreeMap<CfgNodeId, HavocCause>,
    /// Per RESOLVED `.`/`source` site, the loadable path it names — so the shadow pass can replay
    /// which definitions that statement bound without re-reading the value plane.
    resolved_loads: BTreeMap<CfgNodeId, ResolvedHead>,
    /// THE ONE LOAD ACCOUNT (`30I:rul-one-load-account-separate-projections`): every statically
    /// possible resolved load occurrence the settled walk followed, with its locus and positional
    /// context, from which every consumer derives its own projection.
    ///
    /// The binary's acquisition loop reads its wanted set and re-solves; the include-tree reads its
    /// SPEAKER edges; the cross-custody narrative reads its SELECTION edges; the bundle projection
    /// will read the occurrences whole. There is no second resolver at any edge to drift from this
    /// one (`30I:rul-one-loader-many-projections`).
    loads: LoadAccount,
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

    /// Sites whose head names its file under the spelling Dorc invokes and provably cannot succeed
    /// under a slashless one — an OFF-RAMP lint (`30P:rul-dead-spelling-is-not-unsound`): nothing
    /// this run does is wrong, and `sh book.sh` would die at that line.
    #[must_use]
    pub fn dies_slashless(&self) -> &BTreeSet<CfgNodeId> {
        &self.dies_slashless
    }

    /// Sites whose operand carries no `/`, which POSIX makes a `PATH` search.
    #[must_use]
    pub fn searches_path(&self) -> &BTreeSet<CfgNodeId> {
        &self.searches_path
    }

    /// Why each havoc'd site havoc'd — the operand a specific hint is built from, rather than a
    /// reason reconstructed at the diagnostic seat.
    #[must_use]
    pub fn havoc_causes(&self) -> &BTreeMap<CfgNodeId, HavocCause> {
        &self.havoc_causes
    }

    /// The ONE load account every projection is derived from.
    #[must_use]
    pub fn loads(&self) -> &LoadAccount {
        &self.loads
    }

    /// Per RESOLVED `.`/`source` site, the head it named — the load ACT, which is what a
    /// locator points at when it says which line brought a file into the unit
    /// (`30I:rul-source-maps-are-rich-and-early`), plus whether the AUTHOR named it, which is what
    /// decides whether an emitter may rewrite that line (`30P:rul-rewrite-permission-is-derived`).
    #[must_use]
    pub fn resolved_loads(&self) -> &BTreeMap<CfgNodeId, ResolvedHead> {
        &self.resolved_loads
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

    /// The loaded-source INDEX whose definition of `name` is live immediately before `node`
    /// (`28O:dec-load-order-is-the-id-order`), or `None` where the environment names none.
    ///
    /// The index-space twin of [`source_before`](Self::source_before), for a seat that keys on the
    /// ordered source vector rather than on `SourceFileId` — `dorc_oracle::closure::SiteFrame` is
    /// the one consumer, and doing the crossing here is what keeps `oracle` free of the id
    /// vocabulary it would otherwise have to re-spell.
    #[must_use]
    pub fn source_index_before(&self, node: CfgNodeId, name: &str) -> Option<usize> {
        self.source_before(node, name).map(|file| file.0 as usize)
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

/// The identity a derived row carries: the source index its lift read, and the funcdef span that
/// lift recorded (`28Q` §1.1 — definition-grade row identity).
///
/// ONE seat for the whole crossing, so no consumer re-spells either half. The seats hold a
/// positional index and their own row; this turns the pair into the key
/// [`dorc_core::answering_row`] compares, and it is the only way a row's id is ever minted.
#[must_use]
pub fn row_definition(file: usize, span: Span) -> dorc_core::DefinitionId {
    dorc_core::DefinitionId::at(source_file_of_index(file), span)
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
    let sites = load_sites(ast, cfg, defs, literals);
    let (unresolvable_loads, resolved_loads) = (sites.unresolvable.clone(), sites.resolved.clone());
    let solve_pruned = |folded: &BTreeSet<(CfgNodeId, CfgNodeId)>| {
        let graph = PrunedCfg::new(cfg, folded);
        solve_certified(&graph, Direction::Forward, |node, incoming: &EnvStack| {
            transfer(ast, cfg, defs, literals, &sites, &universe, node, incoming)
        })
    };

    match fold_to_environment(solve_pruned, |states| {
        dead_edges(ast, cfg, defs, literals, states, true)
    }) {
        Ok((states, folded_edges)) => {
            let loads = settled_account(
                defs,
                literals,
                cfg.entry(),
                &states,
                &resolved_loads,
                &sites.named,
            );
            FuncEnv {
                states,
                floor: None,
                unresolvable_loads,
                dies_slashless: sites.dies_slashless.clone(),
                searches_path: sites.searches_path.clone(),
                havoc_causes: sites.causes.clone(),
                resolved_loads,
                folded_edges,
                loads,
            }
        }
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
        dies_slashless: BTreeSet::new(),
        searches_path: BTreeSet::new(),
        havoc_causes: BTreeMap::new(),
        resolved_loads: BTreeMap::new(),
        folded_edges: BTreeSet::new(),
        loads: LoadAccount::default(),
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
    /// `command -v <literal ROLE name>`, where the name is one the unit DEFINES somewhere.
    ///
    /// Contracted to function-definedness within the analysis unit
    /// (`28M:rul-command-v-reads-fn-definedness`, human-restated: "for analysis, `command -v`
    /// will never check for a binary named `cmd__is_converged`"). A PATH executable of the same
    /// name is pathological-by-construction and is `28K:bitem8`'s reserved differential case.
    ///
    /// **ROLE-SHAPED, not merely known** — and the difference became load-bearing the moment the
    /// definition table widened past role names. The ruling's whole warrant is that nobody ships a
    /// BINARY called `apt_get__is_converged`; it says nothing about `jq`, and a unit carrying a
    /// `jq()` polyfill is exactly a unit where `command -v jq` is a genuine, host-dependent PATH
    /// question. Deciding it by function-definedness would model the polyfill as bound on a host
    /// where the real binary wins the lookup and the guard's right operand never runs — the engine
    /// then holds a body no execution reached, which is winner-shifting in the worst direction.
    /// So membership in the table is necessary and NOT sufficient: the name must also be a role
    /// name, which is what keeps the ordinary `command -v yum` out (`28M:dec-decidable-set-v0`
    /// grows by NAME only, and this is not a growth).
    FunctionDefined(String),
    /// `[ -f <literal> ]` / `test -f <literal>` naming a path the CONTROLLER resolved as a
    /// loadable source.
    ///
    /// Decides TRUE only. Absence from the load set is not filesystem absence — the driver knows
    /// only what it was told to read — so an unrecognized path stays ⊤ and
    /// `28K:res-host-conditional-loading` is untouched. Deciding a RESOLVED path true adds no
    /// assumption the loading model did not already make: `. ./lib.sh` already binds the
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
            if !defs.knows(name) || dorc_oracle::reserved::role_family(name).is_none() {
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
    defs.program_of_path_operand(path)
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
    for &def in defs.ambient.iter().flat_map(|root| &root.defs) {
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
                for def in sourced_definitions(defs, env, id) {
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

/// How deep a chain of nested loads the domain follows before answering ⊤.
///
/// A real package tree is a handful of levels; the cap is a backstop of [`solve`](crate::solve)'s
/// own flavour, and running out LOSES PRECISION rather than misbinding — an over-deep chain reads
/// ⊤, which withholds. A load already on the stack is refused by the same rule, which is what
/// makes a source CYCLE terminate; a DIAMOND (two entrypoints reaching one dependency) is not a
/// cycle and is followed both times, as `floor30-diamond-source-binds-once` measures a shell doing.
const LOAD_DEPTH_CAP: usize = 16;

/// Interpret one loadable file's top level against the environment at the load site
/// (`30I:rul-static-loading-is-the-whole-model`).
///
/// # The guard is decided in ONE direction and joined in the other
///
/// `command -v <name>` is not a question about this unit alone: a shell answers it from functions,
/// then builtins, then `PATH`. So the two directions are not symmetric, and the asymmetry is the
/// whole safety argument.
///
/// A frame that names a LIVE definition decides the guard TRUE unconditionally, because a live
/// function shadows every builtin and every binary — nothing on the host can make that query fail.
///
/// A frame that proves the name UNDEFINED decides FALSE only where `dec-decidable-set-v0`'s
/// existing warrant reaches: a ROLE-shaped name, which nobody ships a binary called
/// (`28M:rul-command-v-reads-fn-definedness`). For any other name a host binary could still answer
/// the query, so deciding FALSE would model a fallback as loaded on a host that took the other
/// branch — the engine then attributing calls to a body no execution ran, which is the
/// mis-attributed class (`271:rul-sin-ordering`). Undecided, both branches JOIN, which is
/// can't-say and withholds.
///
/// FLAGGED, not settled (`inv-superposition`): `30I:rul-include-guards-are-load-semantics` is
/// typed and reads as wanting the FALSE direction for ordinary helper names too. Whether the
/// existing role-shaped fence may be widened there is a licensure question with an owner above
/// this component; `30Ib` §4 carries it as an open deviation.
/// The loading context one program is interpreted under: everything constant across the walk.
#[derive(Clone, Copy)]
struct Loading<'a, 's> {
    defs: &'a DefinitionTable,
    literals: &'a SourceLiteralPlane<'a>,
    node: CfgNodeId,
    /// The canonical key of the file whose program this is, when the program belongs to a loaded
    /// file. `None` at a book `.` and at an invocation-named root, neither of which mints a
    /// speaker edge (`30I:rul-books-load-but-do-not-speak`; CLI co-loading composes no custody).
    sourcer: Option<&'s str>,
    /// Whether a `.` reached from here is one the engine knows really runs.
    ///
    /// False inside an UNDECIDED guard's speculative branch walks, and STICKY downward: an
    /// authored `.` mints its author's speaker edge, but only where the engine can say the load
    /// happened. On a guard's reuse route no `.` ran at all, so minting from a branch nobody
    /// decided would rest a licence on somebody else's utterance
    /// (`rul-speaker-minting-is-oracle-sourcing-only`). The occurrence is recorded EITHER WAY —
    /// absence from the speaker projection is not absence from the possible-load one
    /// (`30I:rul-one-load-account-separate-projections`).
    certain: bool,
    /// The occurrence whose descent we are inside, if any — what a nested load names as its parent.
    within: Option<usize>,
    depth: usize,
}

impl Loading<'_, '_> {
    /// The route an occurrence reached from here sits on.
    fn route(self, taken: crate::load::LoadRoute) -> crate::load::LoadRoute {
        if self.certain {
            taken
        } else {
            crate::load::LoadRoute::Speculative
        }
    }

    /// Who a `.` spelled from here belongs to. A book `.` and an invocation-named root are answered
    /// by their own root records, so anything reaching this arm with no sourcer is descending
    /// through a file the caller could not key.
    fn spelled_by(self) -> crate::load::LoadSourcer {
        self.sourcer.map_or(crate::load::LoadSourcer::Book, |key| {
            crate::load::LoadSourcer::File(key.to_owned())
        })
    }
}

fn run_program(
    ctx: Loading<'_, '_>,
    program: &crate::load::LoadProgram,
    incoming: &EnvStack,
    locals: &mut BTreeMap<String, String>,
    visiting: &mut BTreeSet<String>,
    account: &mut LoadAccount,
) -> EnvStack {
    run_steps(ctx, program.steps(), incoming, locals, visiting, account)
}

fn run_steps(
    ctx: Loading<'_, '_>,
    steps: &[crate::load::LoadStep],
    incoming: &EnvStack,
    locals: &mut BTreeMap<String, String>,
    visiting: &mut BTreeSet<String>,
    account: &mut LoadAccount,
) -> EnvStack {
    use crate::load::LoadStep;

    let ambient = |name: &str| ctx.literals.variable_text(ctx.node, name);
    let mut env = incoming.clone();
    for step in steps {
        match step {
            LoadStep::Define(def) => {
                if let Some(d) = ctx.defs.get(*def) {
                    env.bind(&d.name, Flat::Elem(Binding::Defined(*def)));
                }
            }
            LoadStep::Assign { name, value } => match value.expand(locals, &ambient) {
                Some(text) => drop(locals.insert(name.clone(), text)),
                // An unreadable constant does not poison the environment — it only makes any
                // operand built from it unresolvable, which the load step below answers as ⊤.
                None => drop(locals.remove(name)),
            },
            LoadStep::Control(control) => {
                env = run_control(ctx, control, &env, locals, visiting, account);
            }
        }
    }
    env
}

fn run_control(
    ctx: Loading<'_, '_>,
    control: &crate::load::LoadControl,
    incoming: &EnvStack,
    locals: &mut BTreeMap<String, String>,
    visiting: &mut BTreeSet<String>,
    account: &mut LoadAccount,
) -> EnvStack {
    use crate::load::LoadControl;

    let ambient = |name: &str| ctx.literals.variable_text(ctx.node, name);
    let mut env = incoming.clone();
    match control {
        LoadControl::UnsetFunctions(names) => {
            for name in names {
                env.bind(name, Flat::Elem(Binding::Undefined));
            }
            env
        }
        LoadControl::Load { target, span } => {
            let suspend_the_sourcer = |account: &mut LoadAccount| {
                if let Some(sourcer) = ctx.sourcer {
                    account.suspend(sourcer.to_owned());
                }
            };
            // ABSORBING where the book plane is pointwise: the `30Mg` R1 prelude floor. Pointwise
            // here would let a LATER prelude root license sites — a widening owed a ruling.
            let Some(next) = target
                .expand(locals, &ambient)
                .and_then(|text| ctx.defs.cwd.resolve_dot(&text))
            else {
                suspend_the_sourcer(account);
                return EnvStack::Top;
            };
            let Some(program) = ctx.defs.program_at_key(&next) else {
                account.want(next);
                suspend_the_sourcer(account);
                return EnvStack::Top;
            };
            let here = account.record(crate::load::LoadOccurrence {
                sourcer: ctx.spelled_by(),
                target: next.clone(),
                locus: Some(*span),
                at: ctx.node,
                within: ctx.within,
                route: ctx.route(crate::load::LoadRoute::Taken),
            });
            if ctx.depth == 0 || !visiting.insert(next.clone()) {
                return EnvStack::Top;
            }
            let inner = Loading {
                sourcer: Some(&next),
                within: Some(here),
                depth: ctx.depth.saturating_sub(1),
                ..ctx
            };
            // `locals` itself, never a copy: a `.` runs in the caller's own shell, so what the
            // loaded file assigns at its top level is live for everything the sourcer does next
            // (`30I:rul-dot-resolves-as-sh`).
            let loaded = run_program(inner, program, &env, locals, visiting, account);
            visiting.remove(&next);
            loaded
        }
        LoadControl::Guard {
            condition,
            negated,
            then_,
            else_,
        } => {
            let branch = |controls: &[LoadControl],
                          decided: bool,
                          visiting: &mut BTreeSet<String>,
                          account: &mut LoadAccount| {
                let inner_ctx = Loading {
                    certain: ctx.certain && decided,
                    ..ctx
                };
                let mut inner = env.clone();
                for control in controls {
                    inner = run_control(
                        inner_ctx,
                        control,
                        &inner,
                        &mut locals.clone(),
                        visiting,
                        account,
                    );
                }
                inner
            };
            match decide_guard(
                ctx, condition, *negated, then_, else_, &env, locals, account,
            ) {
                Some(taken) => branch(taken, true, visiting, account),
                // Undecided walks BOTH, so the acquisition sees every file the guard could reach:
                // reading one the run does not bind is harmless, missing one it does bind is not.
                // Neither walk mints a speaker: an undecided guard is exactly the world where the
                // engine cannot say whose `.` really ran (`rul-speaker-minting-is-oracle-sourcing-only`).
                None => branch(then_, false, visiting, account)
                    .join(&branch(else_, false, visiting, account)),
            }
        }
    }
}

/// Which branch of a guard the engine can say is taken, or `None` for "cannot say".
///
/// The two species answer for different reasons and neither generalizes to the other; see
/// [`command_v_decides`] and [`sentinel_decides`].
#[expect(
    clippy::too_many_arguments,
    reason = "a predicate over the WHOLE guard: its condition, its polarity, both branches, and the loading context that resolves its target. Bundling them would hide which condition a caller is answering."
)]
fn decide_guard<'a>(
    ctx: Loading<'_, '_>,
    condition: &crate::load::LoadCondition,
    negated: bool,
    then_: &'a [crate::load::LoadControl],
    else_: &'a [crate::load::LoadControl],
    env: &EnvStack,
    locals: &BTreeMap<String, String>,
    account: &mut LoadAccount,
) -> Option<&'a [crate::load::LoadControl]> {
    use crate::load::LoadCondition;
    match condition {
        LoadCondition::CommandV { function } => {
            command_v_decides(env, function).map(|held| if held == negated { else_ } else { then_ })
        }
        LoadCondition::Value {
            name,
            literal,
            equals,
        } => sentinel_decides(
            ctx, name, literal, *equals, negated, then_, else_, env, locals, account,
        ),
    }
}

/// `command -v <name>`, decided in ONE direction and joined in the other.
///
/// The asymmetry is the whole safety argument. A frame that names a LIVE definition decides TRUE
/// unconditionally, because a live function shadows every builtin and every binary — nothing on
/// the host can make that query fail. A frame that proves the name UNDEFINED decides FALSE only
/// where `28M:dec-decidable-set-v0`'s warrant reaches: a ROLE-shaped name, which nobody ships a
/// binary called. For any other name a host binary could still answer, so deciding FALSE would
/// model a fallback as loaded on a host that took the other branch — the mis-attributed class
/// (`271:rul-sin-ordering`).
///
/// This is why `command -v` is not the exact-package guard: its answer space is neither
/// floor-identical nor package identity (`notes/30Ic`; `30I:pin-command-v-load-model`). It stays a
/// supported, idiomatic route that conservatively withholds.
fn command_v_decides(env: &EnvStack, function: &str) -> Option<bool> {
    match env.lookup(function) {
        Flat::Elem(Binding::Defined(_)) => Some(true),
        Flat::Elem(Binding::Undefined)
            if dorc_oracle::reserved::role_family(function).is_some() =>
        {
            Some(false)
        }
        Flat::Elem(Binding::Undefined) | Flat::Top | Flat::Bottom => None,
    }
}

/// The exact package sentinel (`30I:rul-guarded-source-mints-exact-speaker-edge`).
///
/// # This is RECOGNITION, never a licensing widening
///
/// The idiom is a method, spelled in sh, by which an author says "reuse THIS exact package when
/// its own load value says it is present; otherwise source it". It LOOKS like a fork; where this
/// fires it is not one, because both arms land on the same exact foreign speech: either a prior
/// oracle already loaded that exact target, or this file loads it now. The engine's job is to SEE
/// that there is no analysis-time choice between SPEAKERS and decline to drive to ⊤.
///
/// Nothing extra is trusted. The reuse arm is reachable at all only if the target really loaded,
/// because [`DefinitionTable::sole_populator`] has proved that target's own closure is the only
/// thing in the authored world that could have written the tested value; and the ARM taken is read
/// off the environment rather than assumed, because "the package loaded earlier" and "the package
/// loads here" are different worlds whenever anything since has shadowed a name.
///
/// Whatever cannot be aligned exactly withholds: no decision, both branches join, no speaker edge.
///
/// # The conditions, and why each is load-bearing
///
/// 1. **The shape is the idiom**: one branch loads exactly one target and the other is EMPTY.
///    Anything richer — a second load, a removal, a nested guard — is a fork the engine has not
///    been shown is not one.
/// 2. **The polarity is the idiom**: the branch that LOADS is the one taken when the sentinel does
///    NOT match. A guard that loads when the value DOES match says something else entirely.
/// 3. **The target resolves exactly**, from authored-before-contact input, to a program the
///    controller holds.
/// 4. **The target's closure is the value's sole populator** — both halves: at least one
///    assignment inside it, and none anywhere else, the book included. A same-valued assignment
///    from any other unit is exactly what makes the reuse arm forgeable, and demanding both means
///    the only way to satisfy the guard is that the package really loaded.
/// 5. **Nothing removes what the target declares.** An `unset -f` and redefine elsewhere in the
///    loaded world is one of the named ways the shape can mislead: the removal would leave the
///    sentinel set with the package's names gone, so the two arms would no longer agree.
/// 6. **The environment says which arm** ([`sentinel_arm`]).
///
/// The reached-vouch-path half of `30I` §3.4 is deliberately NOT here: it is the EXISTING custody
/// machinery (`oracle::closure::HelperIndex::resolve` gating on the closures this edge feeds, plus
/// the frame lookup's `Must`-grade requirement). A same-named helper from another unit withholds
/// there, where it already did.
#[expect(
    clippy::too_many_arguments,
    reason = "see decide_guard: the recognition reads the whole guard, and hiding one of its six conditions behind a bundle is exactly what this door must not do"
)]
fn sentinel_decides<'a>(
    ctx: Loading<'_, '_>,
    name: &str,
    literal: &str,
    equals: bool,
    negated: bool,
    then_: &'a [crate::load::LoadControl],
    else_: &'a [crate::load::LoadControl],
    env: &EnvStack,
    locals: &BTreeMap<String, String>,
    account: &mut LoadAccount,
) -> Option<&'a [crate::load::LoadControl]> {
    use crate::load::LoadControl;

    // An EMPTY compared literal cannot tell "the package never loaded" from "it loaded and set the
    // value to nothing": `"${name-}"` is the empty string in both worlds, so the comparison the
    // shell makes is not the one the arm below reads. Withhold rather than pick.
    if literal.is_empty() {
        return None;
    }

    // Conditions 1 and 2, together: which branch loads, and does the branch NOT taken mean the
    // sentinel matched? `then_` runs when the comparison's own sense agrees with the `!`.
    let then_runs_when_equal = equals != negated;
    let (source, reuse, target, span) = match (then_, else_) {
        ([LoadControl::Load { target, span }], []) if !then_runs_when_equal => {
            (then_, else_, target, span)
        }
        ([], [LoadControl::Load { target, span }]) if then_runs_when_equal => {
            (else_, then_, target, span)
        }
        _ => return None,
    };

    // Condition 3.
    let ambient = |var: &str| ctx.literals.variable_text(ctx.node, var);
    let key = target
        .expand(locals, &ambient)
        .and_then(|text| ctx.defs.cwd.resolve_dot(&text))?;
    let closure = ctx.defs.load_closure_of(&key, locals, &ambient)?;

    // Conditions 4 and 5.
    if !ctx.defs.sole_populator(name, &closure) || ctx.defs.anything_removes(&closure) {
        return None;
    }
    // Condition 6.
    let arm = match sentinel_arm(ctx.defs, env, &closure)? {
        SentinelArm::Source => return Some(source),
        SentinelArm::Reuse => reuse,
    };
    // Condition 7, and it is a COMPARISON rather than a census: the arm above says only that the
    // target's names are bound to the target's own definitions. Whether the guard MATCHES is what
    // the shell actually tests, so a package that assigns `v1` under a guard testing `v2` is
    // SOURCED AGAIN — modelling it as reused is a load-semantic the full model must keep
    // (`30I:rul-load-semantics-stay-full-fidelity`). Nothing here reaches the lossy speech
    // projection, which asks the NAME question and must never gain this one
    // (`30I:rul-guarded-source-speech-is-lossy`).
    if ctx.defs.sentinel_value(name, &closure)? != literal {
        return Some(source);
    }
    // THE REUSE-ARM OCCURRENCE, and it is the whole ruling: the guard mints the same speaker edge
    // as a direct source even where no `.` runs at all (`30I` §3.4 case 2 — "even when another
    // package loaded the exact target first"). It is recorded HERE because the reuse arm has no
    // `.` to hang it on; the SOURCE arm returns above, where the branch walk's own `.` records it.
    account.record(crate::load::LoadOccurrence {
        sourcer: ctx.spelled_by(),
        target: key,
        locus: Some(*span),
        at: ctx.node,
        within: ctx.within,
        route: ctx.route(crate::load::LoadRoute::Reused),
    });
    Some(arm)
}

/// Which arm of a recognized sentinel guard this environment is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SentinelArm {
    /// The package is not live here, so the fallback `.` runs.
    Source,
    /// The package is already live, and live AS ITSELF, so nothing runs.
    Reuse,
}

/// Read the arm off the environment: is every name the target's closure declares still bound to
/// that closure's own definition, or is none of them bound at all?
///
/// Those two worlds are the guard's two arms, and nothing between them is decidable. A name live
/// from ANOTHER unit is the case that matters: a book's own hand-written function over a package
/// name means the sentinel may well be set while the binding is somebody else's, so both arms no
/// longer land on the same speech and the recognition must decline. A ⊤ or a mixed set says the
/// same thing more obviously.
///
/// A closure that declares NOTHING answers `None` too: there is no environment evidence to read,
/// and a guard whose target contributes no binding has nothing for this recognition to be about.
fn sentinel_arm(
    defs: &DefinitionTable,
    env: &EnvStack,
    closure: &BTreeSet<String>,
) -> Option<SentinelArm> {
    let declared: Vec<DefId> = closure
        .iter()
        .filter_map(|key| defs.program_at_key(key))
        .flat_map(crate::load::LoadProgram::declarations)
        .collect();
    let files: BTreeSet<dorc_core::SourceFileId> = declared
        .iter()
        .filter_map(|&def| defs.get(def).map(|d| d.file))
        .collect();
    if declared.is_empty() {
        return None;
    }
    let (mut any_absent, mut any_present) = (false, false);
    for def in declared {
        let name = &defs.get(def)?.name;
        match env.lookup(name) {
            Flat::Elem(Binding::Undefined) => any_absent = true,
            Flat::Elem(Binding::Defined(live)) if files.contains(&defs.get(live)?.file) => {
                any_present = true;
            }
            Flat::Elem(Binding::Defined(_)) | Flat::Top | Flat::Bottom => return None,
        }
    }
    match (any_absent, any_present) {
        (true, false) => Some(SentinelArm::Source),
        (false, true) => Some(SentinelArm::Reuse),
        _ => None,
    }
}

/// The settled environment's whole load account: what its loads still WANT, which file sourced
/// which, and which sourcers named nothing loadable (`30I:rul-one-loader-many-projections`).
///
/// A post-pass rather than an accumulation inside the transfer, for [`load_sites`]' reason: the
/// transfer is asked once per worklist iteration and an intermediate round's account would carry
/// paths the settled answer never names. Run against the SETTLED states, this asks the same
/// interpreter the same questions one final time.
///
/// Two consumers, one walk. The binary's acquisition loop reads `wanted` and re-solves until
/// nothing new appears; the include-tree reads `edges`/`unresolved`. Both therefore answer from
/// the engine that really followed the loads — no second resolver exists to drift from this one,
/// which is what lets a dependency sited through a caller-set root take custody at all.
fn settled_account(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    entry: CfgNodeId,
    states: &[EnvStack],
    resolved_loads: &BTreeMap<CfgNodeId, ResolvedHead>,
    unresolved_targets: &BTreeMap<CfgNodeId, ResolvedHead>,
) -> LoadAccount {
    let mut account = LoadAccount::default();
    // An acquired INCLUSION is no longer wanted — the controller holds its bytes — but it is still
    // an unresolvable load everywhere else, because acquiring bytes is not modelling them.
    account.want_all(
        unresolved_targets
            .values()
            .filter(|head| !defs.included_at_key(&head.key))
            .map(|head| head.key.clone()),
    );
    // ...and its OCCURRENCE is recorded here, where the resolved-and-programmed loads are, because
    // that is what the artifact's placement keys to (`30I:rul-bundles-key-to-load-occurrences`).
    // It runs no program: `30P:principle-book-code-source-is-inclusion`'s r30 slice is
    // acquire-and-ship, and the splice stays forfeited
    // (`FORFEITS:forfeit-plain-sh-inclusion-analysis`).
    for (&node, head) in unresolved_targets {
        if defs.included_at_key(&head.key) {
            account.record(crate::load::LoadOccurrence {
                sourcer: crate::load::LoadSourcer::Book,
                target: head.key.clone(),
                locus: None,
                at: node,
                within: None,
                route: crate::load::LoadRoute::Taken,
            });
        }
    }
    let universe = defs.names();
    let mut frame = Frame::default();
    for name in &universe {
        frame.insert(name.clone(), Flat::Elem(Binding::Undefined));
    }
    drop(run_ambient_prefix(
        defs,
        literals,
        entry,
        EnvStack::Frames(vec![frame]),
        &mut account,
    ));
    for (&node, head) in resolved_loads {
        let key = head.key();
        let Some(program) = defs.program_at_key(key) else {
            continue;
        };
        let Some(incoming) = states.get(node.index()) else {
            continue;
        };
        // The BOOK's own act is a root occurrence: it is what a root bundle keys to
        // (`30I:rul-bundles-key-to-load-occurrences`), and it mints no speaker
        // (`30I:rul-books-load-but-do-not-speak`) — which the sourcer TYPE says rather than a
        // filter downstream having to remember it.
        let root = account.record(crate::load::LoadOccurrence {
            sourcer: crate::load::LoadSourcer::Book,
            target: head.key.clone(),
            locus: None,
            at: node,
            within: None,
            route: crate::load::LoadRoute::Taken,
        });
        drop(run_program(
            Loading {
                defs,
                literals,
                node,
                sourcer: Some(key),
                certain: true,
                within: Some(root),
                depth: LOAD_DEPTH_CAP,
            },
            program,
            incoming,
            &mut BTreeMap::new(),
            &mut BTreeSet::from([head.key.clone()]),
            &mut account,
        ));
    }
    account
}

/// The definitions a `.`/`source` command at `node` contributes, or empty for any other command.
/// An unresolvable target contributes nothing HERE (it havocs the environment instead, so every
/// name reads ⊤ afterwards and nothing downstream is provable).
fn sourced_definitions(defs: &DefinitionTable, env: &FuncEnv, node: CfgNodeId) -> Vec<DefId> {
    // A cwd-havoc'd site binds nothing, so it declares nothing: a contest off it holds no binding.
    if matches!(
        env.havoc_causes.get(&node),
        Some(HavocCause::CwdUnknown { .. })
    ) {
        return Vec::new();
    }
    env.resolved_loads
        .get(&node)
        .and_then(|head| defs.program_at_key(head.key()))
        .map_or_else(Vec::new, crate::load::LoadProgram::declarations)
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

// ── The load-head evaluator (`30P:rul-load-head-is-exact-or-havoc`) ──

/// What a `.` operand answers under ONE invocation spelling.
///
/// There is no state between [`Resolves`](OperandAnswer::Resolves) and a havoc: a head Dorc cannot
/// evaluate over controller-held inputs claims no authority at all, and no engine SELECTION may
/// launder into one (`30Pb:fnd-possible-singleton-is-not-exact-selection`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperandAnswer {
    /// Names this canonical key in the snapshot's key space, and says whether the AUTHOR named it.
    Resolves(ResolvedHead),
    /// Provably fatal: a NON-FINAL component of the resolved key is a file this unit holds
    /// (`${0%/*}` of a slashless `$0` gives `book.sh/helpers.sh`). Narrow on purpose — a `.` that
    /// cannot succeed runs nothing below it, so it is DEAD rather than unsound
    /// (`30P:rul-dead-spelling-is-not-unsound`).
    Dead,
    /// The HOST picks: a slashless operand (a `PATH` search) or a relative one with no cwd to
    /// stand in.
    HostChosen,
    /// The word could not be evaluated over controller-known inputs.
    Unevaluable(HavocCause),
}

/// A load head that names a file, and whether the AUTHOR named it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedHead {
    key: String,
    explicitness: Explicitness,
}

impl ResolvedHead {
    /// The canonical key the head names.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Whether an emitter may REWRITE the `.` line that names it.
    #[must_use]
    pub const fn explicitness(&self) -> Explicitness {
        self.explicitness
    }
}

/// Whether the author NAMED a load's target or Dorc COMPUTED it
/// (`30P:rul-rewrite-permission-is-derived`, human-typed 2026-08-22).
///
/// Two different permissions ride two different questions, and merging them is the hazard this
/// type exists to stop. EXACT governs AUTHORITY — bindings below the line, vouch lift, shipping —
/// and an evaluated head can be perfectly EXACT. EXPLICITNESS governs REWRITING: re-pointing a `.`
/// at a bundle, or pasting one, edits a line the author wrote, and Dorc may only do that where the
/// author spelled the target it is replacing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Explicitness {
    /// The operand is a plain literal word, or a root the value plane folded from literals: the
    /// author spelled the target, so an emitter may re-point or paste this line.
    Literal,
    /// The operand went through the head evaluator — any `$0`, any parameter expansion. Dorc knows
    /// WHICH file it names; the author did not write that name, so no emitter may rewrite it.
    Evaluated,
}

/// Why a load head could not be named — an AID type, minted where the reason is KNOWN rather than
/// reconstructed at a diagnostic seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HavocCause {
    /// Two live spellings of `$0` resolve to DIFFERENT files, so no one file is the target.
    SpellingsDisagree,
    /// A variable read the source-literal plane cannot answer (an env read, a captured value).
    DynamicValue,
    /// A command substitution or arithmetic expansion — a value only the host can produce
    /// (`30P:rul-static-predict-sites-loads` is the sanctioned route, and needs a stdlib).
    ComputedSubstitution,
    /// A parameter-expansion operator this evaluator does not model.
    UnmodelledOperator,
    /// The operand evaluated, and named no file the controller can identify: a `PATH` search, a
    /// relative operand with no cwd, or a provably fatal one.
    NotInSnapshot,
    /// A relative operand below a line that may have moved the working directory. The clobbering
    /// node is carried so the hint can name the CAUSE rather than the symptom.
    CwdUnknown { clobbered_at: CfgNodeId },
}

/// One `.` site's head, as the settled answer every consumer reads.
#[derive(Debug, Clone)]
struct LoadHead {
    /// The head this site EXACTLY names, or why no file could be named.
    exact: Result<ResolvedHead, HavocCause>,
    /// The operand is RELATIVE, so which file it names depends on where the run stands — the
    /// question the cwd-clobber closure answers.
    cwd_relative: bool,
    /// The SLASHLESS spelling proved fatal — the off-ramp lint's whole trigger. It never denies
    /// EXACT: Dorc invokes the slash-bearing spelling and bakes the decision into the bytes it
    /// ships (`30P:rul-dorc-invokes-in-a-modelled-live-spelling`).
    dies_slashless: bool,
}

/// Evaluate one `.` site's operand into the key it names.
///
/// Delegating to [`SourceLiteralPlane::literal_text`] FIRST is what preserves the positional
/// overlay and constant folding the value plane already performs, so a spliced body and a `$1` in
/// an operand behave exactly as before this evaluator existed.
///
/// EXACT is decided by the spelling Dorc INVOKES; a second live spelling contributes exactly two
/// things — a `Dead` answer, which mints an off-ramp lint, and a `Resolves` to a DIFFERENT key,
/// which denies EXACT (a genuine authorship hazard, cheap to catch).
fn load_head(
    ast: &Ast,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    site: AstId,
    index: usize,
) -> LoadHead {
    let dead = LoadHead {
        exact: Err(HavocCause::DynamicValue),
        cwd_relative: false,
        dies_slashless: false,
    };
    // The author's own bytes named this, so an emitter may rewrite the line.
    if let Some(text) = literals.literal_text(node, index) {
        return LoadHead {
            exact: named_key(key_of(defs, text, Explicitness::Literal)),
            cwd_relative: !dorc_core::loadpath::is_absolute(text),
            dies_slashless: false,
        };
    }
    let NodeKind::Simple { words, .. } = &ast.node(site).kind else {
        return dead;
    };
    let Some(&word) = words.get(index) else {
        return dead;
    };
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return dead;
    };
    // Below here Dorc COMPUTED the name, whatever EXACT answers.
    let evaluated = |spelling| match evaluate_word(defs, literals, node, parts, spelling) {
        Err(cause) => (OperandAnswer::Unevaluable(cause), false),
        Ok(text) => (
            key_of(defs, &text, Explicitness::Evaluated),
            !dorc_core::loadpath::is_absolute(&text),
        ),
    };
    let (invoked, cwd_relative) = defs.spellings.text(Spelling::SlashBearing).map_or(
        (OperandAnswer::Unevaluable(HavocCause::DynamicValue), false),
        |_| evaluated(Spelling::SlashBearing),
    );
    let other = defs
        .spellings
        .text(Spelling::Slashless)
        .map(|_| evaluated(Spelling::Slashless).0);
    let exact = match named_key(invoked) {
        Ok(head) => match other.as_ref() {
            Some(OperandAnswer::Resolves(elsewhere)) if elsewhere.key != head.key => {
                Err(HavocCause::SpellingsDisagree)
            }
            _ => Ok(head),
        },
        denied => denied,
    };
    LoadHead {
        exact,
        cwd_relative,
        dies_slashless: other == Some(OperandAnswer::Dead),
    }
}

/// The forward CWD-CLOBBER closure: per node, the line above it that may have moved the working
/// directory, when one may have (`30P`'s cwd domain of the point havoc).
///
/// A `.` the loader cannot follow runs arbitrary sh in the caller's own shell, and a `cd` persists
/// out of a sourced file (floor-measured) — so below either, a RELATIVE operand names a file the
/// controller cannot identify. This says WHERE that is true, and which line to blame.
///
/// A `cd` inside `( … )` clobbers NOTHING outside the paren, which is the idiom books are full of.
/// The state is therefore the SCOPE DEPTH a live clobber sits at, and a `ScopeExit` discards
/// everything deeper than the scope it leaves; keeping only the SHALLOWEST live clobber is exact
/// for that test (the shallowest is the one that survives) and is what the blame names.
///
/// Direction: strictly withholding — a clobber makes fewer operands identify, never more. So both
/// approximations here fail safe: joining "a clobber on ANY incoming path" and taking the MINIMUM
/// depth each keep clobbers alive longer than a path-exact answer would.
fn cwd_clobbers(cfg: &Cfg, clobbering: &BTreeSet<CfgNodeId>) -> BTreeMap<CfgNodeId, CfgNodeId> {
    let count = cfg.node_count();
    // `u32::MAX` = not yet reached; both vectors move monotonically DOWN and are bounded.
    let mut depth = vec![u32::MAX; count];
    let mut live: Vec<Option<(u32, CfgNodeId)>> = vec![None; count];
    depth[cfg.entry().index()] = 0;
    let mut work: Vec<usize> = (0..count).collect();
    let mut queued = vec![true; count];
    while let Some(index) = work.pop() {
        queued[index] = false;
        let here = depth[index];
        if here == u32::MAX {
            continue;
        }
        let id = CfgNodeId(u32::try_from(index).unwrap_or(u32::MAX));
        let below = match cfg.node(id).kind {
            CfgNodeKind::ScopeEnter => here.saturating_add(1),
            CfgNodeKind::ScopeExit => here.saturating_sub(1),
            _ => here,
        };
        let mut after = live[index].filter(|&(at_depth, _)| at_depth <= below);
        if clobbering.contains(&id) {
            let mine = (below, id);
            after = Some(after.map_or(mine, |current| current.min(mine)));
        }
        for successor in cfg.succ_ids(id).map(CfgNodeId::index) {
            let mut moved = below < depth[successor];
            depth[successor] = depth[successor].min(below);
            let joined = match (live[successor], after) {
                (None, other) | (other, None) => other,
                (Some(a), Some(b)) => Some(a.min(b)),
            };
            if joined != live[successor] {
                live[successor] = joined;
                moved = true;
            }
            if moved && !queued[successor] {
                queued[successor] = true;
                work.push(successor);
            }
        }
    }
    live.into_iter()
        .enumerate()
        .filter_map(|(index, at)| {
            let (_, blame) = at?;
            Some((CfgNodeId(u32::try_from(index).unwrap_or(u32::MAX)), blame))
        })
        .collect()
}

/// The head an answer names, or the cause that denies one.
fn named_key(answer: OperandAnswer) -> Result<ResolvedHead, HavocCause> {
    match answer {
        OperandAnswer::Resolves(head) => Ok(head),
        OperandAnswer::Unevaluable(cause) => Err(cause),
        OperandAnswer::Dead | OperandAnswer::HostChosen => Err(HavocCause::NotInSnapshot),
    }
}

/// Which canonical key an already-evaluated operand names.
///
/// The slashless refusal and the unknown-cwd refusal are sh's own and belong to
/// [`dorc_core::loadpath::Cwd::resolve_dot`]; what is added here is the DEAD reading, which is a
/// question about the unit's own files rather than about the path rule. Membership in the loaded
/// set is deliberately NOT asked — a key nothing is loaded under is what the acquisition loop goes
/// and reads (`30I:rul-one-loader-many-projections`).
fn key_of(defs: &DefinitionTable, text: &str, explicitness: Explicitness) -> OperandAnswer {
    let Some(key) = defs.cwd.resolve_dot(text) else {
        return OperandAnswer::HostChosen;
    };
    if defs.a_non_final_component_is_a_file(&key) {
        return OperandAnswer::Dead;
    }
    OperandAnswer::Resolves(ResolvedHead { key, explicitness })
}

/// Evaluate a `.` operand's word over controller-known inputs alone
/// (`30P:principle-load-operands-evaluate-over-controller-known-inputs`).
///
/// Structure comes from the AST; every VARIABLE read routes through
/// [`SourceLiteralPlane::variable_text`], so `funcenv-reads-source-literal-plane-only` still holds.
/// `$0` is not a variable read at all — it is a controller-held constant on the definition table.
fn evaluate_word(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    parts: &[WordPart],
    spelling: Spelling,
) -> Result<String, HavocCause> {
    let mut out = String::new();
    for part in parts {
        match part {
            WordPart::Literal(text) | WordPart::SingleQuoted(text) => out.push_str(text),
            WordPart::DoubleQuoted(inner) => {
                out.push_str(&evaluate_word(defs, literals, node, inner, spelling)?);
            }
            WordPart::Param { name } => {
                out.push_str(&read_parameter(defs, literals, node, name, spelling)?);
            }
            WordPart::ParamExpansion { base, op } => {
                out.push_str(&apply_operator(defs, literals, node, base, op, spelling)?);
            }
            WordPart::CommandSubst(_) | WordPart::Arithmetic => {
                return Err(HavocCause::ComputedSubstitution);
            }
        }
    }
    Ok(out)
}

/// One parameter's value: `$0` from the table's spellings, everything else through the plane.
fn read_parameter(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    name: &str,
    spelling: Spelling,
) -> Result<String, HavocCause> {
    if name == "0" {
        return defs
            .spellings
            .text(spelling)
            .map(str::to_owned)
            .ok_or(HavocCause::DynamicValue);
    }
    literals
        .variable_text(node, name)
        .ok_or(HavocCause::DynamicValue)
}

/// Apply one decoded parameter-expansion operator to its base's value.
///
/// Every arm requires the base's value to be KNOWN: a substitution over a base the plane cannot
/// read is undecidable in BOTH directions (set-and-empty and unset are different answers), so it
/// is a havoc rather than a guess.
fn apply_operator(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    base: &str,
    op: &dorc_syntax::ast::ParamOp,
    spelling: Spelling,
) -> Result<String, HavocCause> {
    use dorc_syntax::ast::{ParamOp, SubstituteKind};

    let value = read_parameter(defs, literals, node, base, spelling)?;
    match op {
        ParamOp::EmptyDefault { colon } => Ok(if *colon && value.is_empty() {
            String::new()
        } else {
            value
        }),
        ParamOp::Substitute { kind, colon, word } => {
            let absent = *colon && value.is_empty();
            let word = || evaluate_word(defs, literals, node, word, spelling);
            match kind {
                // The plane answered ⇒ SET: all three yield the value unless `:` rejects the empty.
                SubstituteKind::Default | SubstituteKind::Assign | SubstituteKind::Error => {
                    if absent {
                        word()
                    } else {
                        Ok(value)
                    }
                }
                SubstituteKind::Alternate => {
                    if absent {
                        Ok(String::new())
                    } else {
                        word()
                    }
                }
            }
        }
        ParamOp::Trim {
            end,
            greedy,
            pattern,
        } => Ok(trim(&value, &pattern_of(pattern)?, *end, *greedy)),
        ParamOp::Length | ParamOp::Unmodelled => Err(HavocCause::UnmodelledOperator),
    }
}

/// One atom of a trim pattern — the sh globbing subset this evaluator models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternAtom {
    Char(char),
    AnyOne,
    AnyRun,
}

/// A trim pattern's atoms, or ⊤ when a fragment is not source-literal or uses a bracket class.
///
/// Bracket expressions are refused rather than modelled: their collating and equivalence classes
/// are the target's business, and `posix-in-spirit-default` says a character granted here could
/// never be clawed back.
fn pattern_of(parts: &[WordPart]) -> Result<Vec<PatternAtom>, HavocCause> {
    let mut out = Vec::new();
    for part in parts {
        match part {
            WordPart::Literal(text) => {
                for ch in text.chars() {
                    out.push(match ch {
                        '*' => PatternAtom::AnyRun,
                        '?' => PatternAtom::AnyOne,
                        '[' => return Err(HavocCause::UnmodelledOperator),
                        other => PatternAtom::Char(other),
                    });
                }
            }
            WordPart::SingleQuoted(text) => out.extend(text.chars().map(PatternAtom::Char)),
            WordPart::DoubleQuoted(inner) => out.extend(pattern_of(inner)?),
            _ => return Err(HavocCause::UnmodelledOperator),
        }
    }
    Ok(out)
}

/// `${x%p}` / `${x%%p}` / `${x#p}` / `${x##p}` over a known value and a modelled pattern.
///
/// Shortest-match scans the candidate lengths in increasing order and longest in decreasing, which
/// is the definition rather than an optimisation — the two differ exactly on which candidate the
/// scan meets first.
fn trim(
    value: &str,
    pattern: &[PatternAtom],
    end: dorc_syntax::ast::TrimEnd,
    greedy: bool,
) -> String {
    use dorc_syntax::ast::TrimEnd;

    let chars: Vec<char> = value.chars().collect();
    let mut lengths: Vec<usize> = (0..=chars.len()).collect();
    if greedy {
        lengths.reverse();
    }
    for len in lengths {
        let (kept, candidate) = match end {
            TrimEnd::Suffix => (&chars[..chars.len() - len], &chars[chars.len() - len..]),
            TrimEnd::Prefix => (&chars[len..], &chars[..len]),
        };
        if pattern_matches(pattern, candidate) {
            return kept.iter().collect();
        }
    }
    value.to_owned()
}

/// Does `pattern` match the whole of `text`? A plain backtracking walk — patterns and paths are
/// both tiny, and `perf-doctrine` puts every network round-trip above this.
fn pattern_matches(pattern: &[PatternAtom], text: &[char]) -> bool {
    match pattern.split_first() {
        None => text.is_empty(),
        Some((PatternAtom::AnyRun, rest)) => {
            (0..=text.len()).any(|split| pattern_matches(rest, &text[split..]))
        }
        Some((PatternAtom::AnyOne, rest)) => !text.is_empty() && pattern_matches(rest, &text[1..]),
        Some((PatternAtom::Char(want), rest)) => {
            text.first() == Some(want) && pattern_matches(rest, &text[1..])
        }
    }
}

/// Split every `.`/`source` site into the resolvable and the unresolvable.
///
/// Unresolvable — a head no live spelling names, a path the driver never read, or a target word
/// carrying anything weaker than source-literal provenance — havocs the environment (`28K` §1
/// rul-unloadable-is-unlicensed); the caller discloses them, since silence licenses nothing. The
/// resolvable half is kept so the shadow pass can replay each statement's bindings.
///
/// THE ONE RESOLVER: the transfer reads this map rather than re-evaluating a head per worklist
/// iteration, so no second answer to "which file is this" exists to drift from it
/// (`30I:rul-one-loader-many-projections`).
///
/// # The cwd pass, and what it costs
///
/// The clobber SEED is a `.` whose head could not be evaluated (it may `cd` in the caller's own
/// shell) plus every `cd`. EXECUTE-B's `Included` plain-sh target arrives through that same door
/// with no edit here (`30Qc:rul-included-is-as-opaque-as-unresolvable`): once `program_at_key`
/// stops answering for it, it is an unevaluated head like any other. NOT the merely-unread bucket
/// — a book-sourced dorc-lang dependency is named-but-unloaded in acquisition round 1 and
/// resolvable in round 2, so seeding clobbers from it would stop the acquisition fixpoint growing.
///
/// Below a clobber a relative head costs BINDING AUTHORITY and NOTHING else (`30P`, ruled
/// 2026-08-22). The file is still acquired and still mirrored at its authored relative path,
/// because cwd-parity is what keeps the shipped tree faithful to the author's and a plan that dies
/// at the `.` on the host is a worse answer than one that runs the line. What it loses is the
/// vouch: the site havocs and takes no custody, exactly as an unresolvable one does — which is why
/// such a site sits in BOTH `resolved` and `unresolvable`.
fn load_sites(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
) -> LoadSites {
    let mut sites = LoadSites::default();
    let mut heads = Vec::new();
    let mut clobbering = BTreeSet::new();
    for node in 0..cfg.node_count() {
        let id = CfgNodeId(u32::try_from(node).unwrap_or(u32::MAX));
        let cfg_node = cfg.node(id);
        if cfg_node.kind != CfgNodeKind::Command {
            continue;
        }
        if literals.literal_text(id, 0) == Some("cd") {
            clobbering.insert(id);
            continue;
        }
        if !matches!(literals.literal_text(id, 0), Some("." | "source")) {
            continue;
        }
        if literals
            .literal_text(id, 1)
            .is_some_and(|text| !text.contains('/') && !text.contains('\\'))
        {
            sites.searches_path.insert(id);
        }
        let head = load_head(ast, defs, literals, id, cfg_node.ast, 1);
        if head.dies_slashless {
            sites.dies_slashless.insert(id);
        }
        if head.exact.is_err() {
            clobbering.insert(id);
        }
        heads.push((id, head));
    }
    let clobbers = cwd_clobbers(cfg, &clobbering);
    for (id, head) in heads {
        let clobbered = head
            .cwd_relative
            .then(|| clobbers.get(&id).copied())
            .flatten();
        match head.exact {
            // A name the controller never read is still a name: the acquisition reads exactly these
            // and re-solves, which is how a book-sourced package joins the loaded set at all.
            Ok(resolved) => {
                if let Some(clobbered_at) = clobbered {
                    sites
                        .causes
                        .insert(id, HavocCause::CwdUnknown { clobbered_at });
                    sites.cwd_havoc.insert(id);
                    sites.unresolvable.insert(id);
                }
                if defs.program_at_key(resolved.key()).is_some() {
                    sites.resolved.insert(id, resolved);
                } else {
                    sites.named.insert(id, resolved);
                    sites.unresolvable.insert(id);
                }
            }
            Err(cause) => {
                sites.causes.insert(id, cause);
                sites.unresolvable.insert(id);
            }
        }
    }
    sites
}

/// What one pass over the `.`/`source` sites found.
#[derive(Debug, Default, Clone)]
struct LoadSites {
    /// Sites whose load the environment could not follow — they havoc.
    unresolvable: BTreeSet<CfgNodeId>,
    /// Sites whose target the loaded set holds.
    resolved: BTreeMap<CfgNodeId, ResolvedHead>,
    /// Sites whose target the operand NAMED but the loaded set does not hold.
    named: BTreeMap<CfgNodeId, ResolvedHead>,
    /// Why an unresolvable site could not be named — the hint's operand.
    causes: BTreeMap<CfgNodeId, HavocCause>,
    /// Sites that name their file but bind NOTHING, because a line above may have moved the
    /// working directory. They stay in `resolved` so the file is still acquired and mirrored.
    cwd_havoc: BTreeSet<CfgNodeId>,
    /// Sites whose head is EXACT for the spelling Dorc invokes and provably fatal for the other
    /// (`30P:rul-dead-spelling-is-not-unsound`) — an off-ramp lint, never a refusal.
    dies_slashless: BTreeSet<CfgNodeId>,
    /// Sites whose operand is a plain slash-less literal, which POSIX makes a `PATH` search rather
    /// than a cwd lookup — a host read, so the controller names no file.
    searches_path: BTreeSet<CfgNodeId>,
}

/// The per-node transfer.
///
/// An UNREACHED node produces ⊥. Havoc is what an EXECUTED unmodeled construct does to the
/// environment; a node no path reaches executes nothing, so reading ⊤ off one would let a
/// provably-dead branch poison the join it never reaches — exactly what the fold masks edges to
/// prevent. `Entry` is exempt because minting the boundary state out of ⊥ is its whole job.
#[expect(
    clippy::too_many_arguments,
    reason = "the whole analysis unit a per-node transfer is a pure function OF: its two programs, its loaded set, its value window, its pre-pass load answers, its name universe, and the node with its inflow. Bundling them would hide which of them a given arm reads."
)]
fn transfer(
    ast: &Ast,
    cfg: &Cfg,
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    sites: &LoadSites,
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
            // The transfer discards the account, exactly as `command_transfer` does: it is asked
            // once per worklist iteration, and the settled answer is what a caller may act on.
            run_ambient_prefix(
                defs,
                literals,
                id,
                EnvStack::Frames(vec![frame]),
                &mut LoadAccount::default(),
            )
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
        CfgNodeKind::Command => command_transfer(defs, literals, sites, universe, id, incoming),
        _ => incoming.clone(),
    }
}

/// Apply the ambient prefix to the entry state: each CLI-named source in invocation order, as the
/// ordinary `.` a pre-source is (`30I:rul-pre-source-is-dot-prelude`).
///
/// Running the PROGRAM rather than applying a flat declaration list is what makes a pre-sourced
/// package's include guard decide at all, and what lets its own `.` reach a dependency the
/// invocation never named. A root with no program on file — an unmarked source, which makes no
/// dialect claim, or one whose path would not canonicalize — falls back to its declarations, which
/// is what the whole prefix used to do.
///
/// One `locals` map spans the prefix, because a shell's `.` leaves its assignments live for the
/// next one; `visiting` is per-root, because each is a separate load act.
fn run_ambient_prefix(
    defs: &DefinitionTable,
    literals: &SourceLiteralPlane<'_>,
    node: CfgNodeId,
    mut env: EnvStack,
    account: &mut LoadAccount,
) -> EnvStack {
    let mut locals = BTreeMap::new();
    for root in &defs.ambient {
        let program = root.key.as_deref().and_then(|key| defs.program_at_key(key));
        let Some(program) = program else {
            for &def in &root.defs {
                if let Some(d) = defs.get(def) {
                    env.bind(&d.name, Flat::Elem(Binding::Defined(def)));
                }
            }
            continue;
        };
        let mut visiting = root.key.iter().cloned().collect();
        // A pre-source is a root occurrence too, and its sourcer type is what says CLI co-loading
        // composes no custody (`rul-cli-coloading-composes-nothing`) without a downstream filter.
        let at_root = root.key.clone().map(|key| {
            account.record(crate::load::LoadOccurrence {
                sourcer: crate::load::LoadSourcer::Invocation,
                target: key,
                locus: None,
                at: node,
                within: None,
                route: crate::load::LoadRoute::Taken,
            })
        });
        env = run_program(
            Loading {
                defs,
                literals,
                node,
                sourcer: root.key.as_deref(),
                certain: true,
                within: at_root,
                depth: LOAD_DEPTH_CAP,
            },
            program,
            &env,
            &mut locals,
            &mut visiting,
            account,
        );
    }
    env
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
    sites: &LoadSites,
    universe: &BTreeSet<String>,
    node: CfgNodeId,
    incoming: &EnvStack,
) -> EnvStack {
    let Some(word) = literals.literal_text(node, 0) else {
        return incoming.clone();
    };
    // `28K` §1: every name an unloaded file COULD define is ⊤ — at this line, and no further.
    let havoc = || {
        let mut env = incoming.clone();
        env.havoc_names(universe);
        env
    };
    match word {
        "." | "source" => {
            // Names its file for acquisition and mirroring; binds nothing (a line above may have cd'd).
            if sites.cwd_havoc.contains(&node) {
                return havoc();
            }
            let Some(head) = sites.resolved.get(&node) else {
                return havoc();
            };
            let Some(program) = defs.program_at_key(head.key()) else {
                return havoc();
            };
            run_program(
                Loading {
                    defs,
                    literals,
                    node,
                    sourcer: Some(head.key()),
                    certain: true,
                    within: None,
                    depth: LOAD_DEPTH_CAP,
                },
                program,
                incoming,
                &mut BTreeMap::new(),
                &mut BTreeSet::from([head.key.clone()]),
                // The transfer discards the account: it is asked once per worklist iteration, and
                // the settled answer is what a caller may act on ([`settled_account`]).
                &mut LoadAccount::default(),
            )
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

    /// The degenerate load program: a file whose top level is a flat list of declarations.
    fn flat(defs: Vec<DefId>) -> LoadProgram {
        LoadProgram::of(defs.into_iter().map(LoadStep::Define).collect())
    }

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
            dies_slashless: BTreeSet::new(),
            searches_path: BTreeSet::new(),
            havoc_causes: BTreeMap::new(),
            resolved_loads: BTreeMap::new(),
            folded_edges: BTreeSet::new(),
            loads: crate::load::LoadAccount::default(),
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

    /// SIGNATURE ENFORCEMENT, the load-head evaluator's half of
    /// `funcenv-reads-source-literal-plane-only`: the evaluator reads the AST for STRUCTURE and
    /// routes every VARIABLE read through the grade-gated plane, never through the value plane
    /// directly. `$0` is not a variable read at all — it is a controller-held constant on the
    /// definition table, which is why it may site a load when a host-answered value may not.
    ///
    /// Lexical, over the evaluator's own region, because the module as a whole legitimately imports
    /// the plane's backing type — the property is "this code cannot spell it", which no signature
    /// expresses.
    #[test]
    fn the_load_head_evaluator_names_no_value_plane_accessor() {
        let src = include_str!("funcenv.rs");
        let start = src
            .find("// \u{2500}\u{2500} The load-head evaluator")
            .expect("the evaluator's section header");
        let end = start
            + src[start..]
                .find("/// Split every")
                .expect("the section ends where load_sites begins");
        let region = &src[start..end];
        assert!(region.contains("fn evaluate_word"), "a non-empty walk");
        for forbidden in [
            "ValueFlow",
            "variable_before",
            "argv_values",
            "argv_word_grades",
        ] {
            assert!(
                !region.contains(forbidden),
                "`{forbidden}` appears in the load-head evaluator — which oracle answers a site \
                 would then rest on something outside the program text"
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
    /// registered under its own path so a book's `. ./lib.sh` binds the same definition, plus the
    /// book's own role funcdefs (id 1) keyed positionally by their `FuncDef` node.
    fn unit(book: &str, oracle_names: &[&str]) -> (DefinitionTable, DefId) {
        let mut table = DefinitionTable::default();
        let ids: Vec<DefId> = oracle_names
            .iter()
            .map(|name| add_def(&mut table, 0, name))
            .collect();
        table.set_loadable("lib.sh", flat(ids.clone()));
        table.push_ambient("ambient.sh", ids.clone());
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
        table.push_ambient("ambient.sh", vec![first, second]);
        assert!(contests_of(book, &table).is_empty());
    }

    /// CELL 5 — the sanctioned regional-preference idiom (`28K` §1
    /// `rul-scope-by-subshell-resource`) does NOT trip the refusal. The re-source binds in an
    /// INNER frame, so sh discards it at subshell exit and the outer unit's definition survives
    /// untouched: the boundedness IS the spelled intent, and complaining about it would tax the
    /// one selection idiom the design offers.
    #[test]
    fn a_subshell_scoped_re_source_does_not_trip_the_refusal() {
        let book = "( . ./lib.sh; yum install -y nginx )\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.push_ambient("ambient.sh", vec![outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("lib.sh", flat(vec![inner]));
        assert!(contests_of(book, &table).is_empty());
    }

    /// CELL 6a — a TOP-LEVEL cross-unit shadow arriving by the book's own sourcing DOES trip: the
    /// override is unbounded, so appending one `.`-source line would otherwise silently reassign
    /// whose judgment governs the family (`28K` §6 rej-load-order-as-trust-adjudicator).
    #[test]
    fn a_top_level_re_source_trips_the_refusal() {
        let book = ". ./lib.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.push_ambient("ambient.sh", vec![outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("lib.sh", flat(vec![inner]));
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
        table.push_ambient("ambient.sh", vec![first, second]);
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
        let book = "( . ./lib.sh; yum install -y nginx )\nyum install -y curl\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.push_ambient("ambient.sh", vec![outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("lib.sh", flat(vec![inner]));
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
        let book = "( . ./lib.sh; yum install -y nginx )\nyum install -y curl\n";
        let mut table = DefinitionTable::default();
        let outer = add_def_spanned(&mut table, 0, ROLE, 10);
        table.push_ambient("ambient.sh", vec![outer]);
        let inner = add_def_spanned(&mut table, 1, ROLE, 20);
        table.set_loadable("lib.sh", flat(vec![inner]));
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

    /// Span agreement, both directions — the property definition-grade keying rests on, and the
    /// only thing a seat ever asks the table ABOUT a row's identity (production seats ask nothing:
    /// the row carries its own id).
    ///
    /// Two definitions of one role in ONE file are told apart here, which is exactly what the
    /// retired `(file, role name)` join could not do — it had to answer "ambiguous" and withhold at
    /// every frame, including the one whose definition was unambiguously live.
    #[test]
    fn the_table_holds_a_definition_at_each_distinct_span() {
        let mut table = DefinitionTable::default();
        let sole = add_def_spanned(&mut table, 0, ROLE, 10);
        let first = add_def_spanned(&mut table, 1, ROLE, 20);
        let second = add_def_spanned(&mut table, 1, ROLE, 40);
        for id in [sole, first, second] {
            assert!(
                table.holds(
                    table
                        .identity_of(id)
                        .expect("an added definition has an id")
                )
            );
        }
        assert_ne!(table.identity_of(first), table.identity_of(second));
        assert!(
            !table.holds(dorc_core::DefinitionId::at(
                SourceFileId(1),
                Span::new(BytePos(999), BytePos(1009))
            )),
            "a span the table never recorded is not held: this is the drift alarm, and it must \
             fire rather than shrug"
        );
    }

    // ── TABLE 5: the decidable-condition fold (`28M` §9) ──

    /// THE WIDENING'S FENCE: `command -v` decides function-definedness for ROLE names and for
    /// nothing else, however much the definition table knows.
    ///
    /// The table now records every top-level funcdef, helpers included (`28Q` §1 — one resolution
    /// mechanism). Membership was the fold's whole universe restriction, so without this fence the
    /// widening would silently promote every helper polyfill into the decidable set. That is
    /// unsound in a way the role case is not: `rul-command-v-reads-fn-definedness` rests on nobody
    /// shipping a binary named `apt_get__is_converged`, and `jq` is precisely a binary. On a host
    /// that HAS jq, the real `command -v jq` succeeds, the `||` right operand never runs, and the
    /// polyfill is never bound — so folding the guard would leave the engine holding a body no
    /// execution reached.
    ///
    /// Asserted as ⊤ AND zero folded edges: the binding alone could come out right for the wrong
    /// reason, and the edge count is what says the fold declined to decide.
    #[test]
    fn a_helper_polyfill_guard_stays_undecidable() {
        let book = "command -v jq >/dev/null 2>&1 || jq() { : ; }\nyum install -y nginx\n";
        let (table, _) = unit(book, &[ROLE]);
        assert!(
            table.knows("jq"),
            "precondition: the widened table records the helper, so membership cannot be what \
             holds the line"
        );
        let (solved, exit) = solve_book(book, &table);
        assert!(solved.trusted());
        assert_eq!(
            solved.binding_before(exit, "jq"),
            Flat::Top,
            "a host-dependent PATH question must leave both branches live"
        );
        assert_eq!(
            solved.folded_edges().len(),
            0,
            "and the fold must decline it rather than reach that ⊤ by another route"
        );
    }

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
        table.set_loadable("lib.sh", flat(vec![lib]));
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
                    yum__is_converged() { :; }\nfi\n. ./lib.sh\nyum install -y nginx\n";
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
        let book = "command -v yum__is_converged >/dev/null 2>&1 || . ./lib.sh\n";
        let (mut table, lib) = sourceable(book);
        let ambient = add_def(&mut table, 2, ROLE);
        table.push_ambient("ambient.sh", vec![ambient]);
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
        let book = "command -v yum__is_converged >/dev/null 2>&1 || . ./lib.sh\n";
        let (table, lib) = sourceable(book);
        assert_eq!(folded(book, &table).0, Flat::Elem(Binding::Defined(lib)));
    }

    /// `[ -f <loadable> ] && . <loadable>` — the file-test half of the decidable set. The path is
    /// one the CONTROLLER resolved, so the test is decidable-TRUE and the load is certain.
    #[test]
    fn a_file_test_on_a_resolved_loadable_decides_true() {
        let book = "[ -f lib.sh ] && . ./lib.sh\n";
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
            table.set_loadable("./oracles/lib.sh", flat(vec![lib]));
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
        table.set_loadable("./oracles/lib.sh", flat(vec![lib]));
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

    /// The load operand is resolved by SH's rule, against the modeled working directory
    /// (`30I:rul-dot-resolves-as-sh`) — so where the run stands decides which file a relative
    /// target names, and one that is not under it names nothing.
    ///
    /// The reversed reading this replaces resolved a target against the SOURCING FILE's own
    /// directory, which gave one authored line a different referent under Dorc than under `dash`;
    /// `rul-unsure-falls-toward-sh-parity` binds name resolution by name, and this is name
    /// resolution.
    #[test]
    fn a_relative_target_resolves_against_the_modeled_working_directory() {
        let book = ". ./lib.sh\nyum install -y nginx\n";
        for (cwd, loaded, bound) in [
            ("/ops", "/ops/lib.sh", true),
            ("/ops", "/ops/pkg/lib.sh", false),
            ("/ops/pkg", "/ops/pkg/lib.sh", true),
        ] {
            let mut table = DefinitionTable::rooted_at(
                dorc_core::loadpath::Cwd::at(cwd),
                super::ScriptSpellings::default(),
            );
            let lib = add_def(&mut table, 0, ROLE);
            table.set_loadable(loaded, flat(vec![lib]));
            let (env, cfg, _) = solve_positional(book, &table);
            let want = if bound {
                Flat::Elem(Binding::Defined(lib))
            } else {
                Flat::Top
            };
            assert_eq!(
                env.binding_before(cfg.exit(), ROLE),
                want,
                "standing in {cwd}, `. ./lib.sh` names {}",
                if bound { loaded } else { "nothing loaded" }
            );
        }
    }

    /// A `.` operand built by pure parameter expansion over `$0` names its file with no command
    /// run (`30P:model-symbolic-dollar-zero`), including at the two traps the atlas measured: a
    /// book at `/` trims to the EMPTY string rather than to "cwd", and a slashless `$0` has no
    /// slash to trim so `${0%/*}` is the whole word — which makes the operand name a path UNDER a
    /// file, dead rather than resolving, so the invoking spelling is what answers.
    ///
    /// CFG shape exercised: one top-level `.` of a single double-quoted word (expansion plus
    /// literal tail), straight-line, with the bound name read at the unit's exit.
    #[test]
    fn a_dollar_zero_trim_names_a_file_with_no_command_run() {
        for (book_path, cwd, loaded, operand) in [
            ("/ops/book.sh", "/ops", "/ops/lib.sh", "${0%/*}/lib.sh"),
            ("/book.sh", "/", "/lib.sh", "${0%/*}/lib.sh"),
            ("book.sh", "/ops", "/ops/lib.sh", "${0%/*}/lib.sh"),
            ("/ops/pkg/book.sh", "/ops", "/lib.sh", "${0%%/*}/lib.sh"),
        ] {
            let cwd = dorc_core::loadpath::Cwd::at(cwd);
            let mut table = DefinitionTable::rooted_at(
                cwd.clone(),
                super::ScriptSpellings::of(book_path, &cwd),
            );
            let lib = add_def(&mut table, 0, ROLE);
            table.set_loadable(loaded, flat(vec![lib]));
            let (env, cfg, _) =
                solve_positional(&format!(". \"{operand}\"\nyum install\n"), &table);
            assert_eq!(
                env.binding_before(cfg.exit(), ROLE),
                Flat::Elem(Binding::Defined(lib)),
                "`{operand}` from a book at {book_path} names {loaded}"
            );
        }
    }

    /// EXACT and EXPLICIT are DIFFERENT questions (`30P:rul-rewrite-permission-is-derived`,
    /// human-typed 2026-08-22), and this is the cell where they part.
    ///
    /// `${0%/*}/lib.sh` and `./lib.sh` name the same file with the same authority — bindings below
    /// the line, vouch lift, shipping. They differ in whether the AUTHOR wrote that name, which is
    /// what decides whether an emitter may re-point or paste the `.` line. Without the marker an
    /// artifact lane cannot tell them apart and starts rewriting a line nobody spelled.
    #[test]
    fn an_evaluated_head_is_exact_and_still_not_the_authors_own_name() {
        let cwd = dorc_core::loadpath::Cwd::at("/ops");
        for (operand, want) in [
            ("./lib.sh", super::Explicitness::Literal),
            ("${0%/*}/lib.sh", super::Explicitness::Evaluated),
        ] {
            let mut table = DefinitionTable::rooted_at(
                cwd.clone(),
                super::ScriptSpellings::of("/ops/book.sh", &cwd),
            );
            let lib = add_def(&mut table, 0, ROLE);
            table.set_loadable("/ops/lib.sh", flat(vec![lib]));
            let (env, cfg, _) =
                solve_positional(&format!(". \"{operand}\"\nyum install\n"), &table);
            assert_eq!(
                env.binding_before(cfg.exit(), ROLE),
                Flat::Elem(Binding::Defined(lib)),
                "`{operand}` is EXACT either way"
            );
            let heads: Vec<super::Explicitness> = env
                .resolved_loads()
                .values()
                .map(super::ResolvedHead::explicitness)
                .collect();
            assert_eq!(heads, vec![want], "`{operand}`");
        }
    }

    /// The four trims, over the pattern subset the evaluator models. Shortest-vs-longest is the
    /// whole reason `%` and `%%` are different operators, and a pattern that matches NOTHING
    /// leaves the value alone rather than answering empty.
    #[test]
    fn the_trims_match_shortest_and_longest_and_leave_a_miss_alone() {
        use dorc_syntax::ast::{TrimEnd, WordPart};

        let pattern = |text: &str| {
            super::pattern_of(&[WordPart::Literal(text.to_owned())]).expect("a modelled pattern")
        };
        let trim =
            |value: &str, pat: &str, end, greedy| super::trim(value, &pattern(pat), end, greedy);
        assert_eq!(
            trim("/ops/pkg/book.sh", "/*", TrimEnd::Suffix, false),
            "/ops/pkg"
        );
        assert_eq!(trim("/ops/pkg/book.sh", "/*", TrimEnd::Suffix, true), "");
        assert_eq!(
            trim("/ops/pkg/book.sh", "*/", TrimEnd::Prefix, false),
            "ops/pkg/book.sh"
        );
        assert_eq!(
            trim("/ops/pkg/book.sh", "*/", TrimEnd::Prefix, true),
            "book.sh"
        );
        assert_eq!(
            trim("book.sh", "/*", TrimEnd::Suffix, false),
            "book.sh",
            "no suffix matches, so the whole word survives — the slashless `$0` trap"
        );
        assert!(
            super::pattern_of(&[WordPart::Literal("[a-z]*".to_owned())]).is_err(),
            "a bracket expression is the TARGET's collation to answer, so it is ⊤ here"
        );
    }

    /// The two live spellings of `$0`, derived from the authored book path and the modeled cwd
    /// alone (`30P:model-symbolic-dollar-zero`) — never realpath'd, never asked of a shell.
    ///
    /// The slashless spelling is live only where `sh <basename>` could have FOUND the book, which
    /// is the only place that invocation is possible; and the `\`-spelled row is why the derivation
    /// normalizes first, since a `\`-bearing path has no `/` for `${0%/*}` to trim and would answer
    /// differently on this project's two development platforms
    /// (`one-platform-green-is-not-cross-platform-green`).
    #[test]
    fn the_dollar_zero_spellings_come_from_the_book_path_and_the_cwd() {
        for (cwd, book, bearing, slashless) in [
            ("/ops", "/ops/book.sh", "/ops/book.sh", Some("book.sh")),
            ("/ops", "book.sh", "./book.sh", Some("book.sh")),
            ("/ops", "/srv/book.sh", "/srv/book.sh", None),
            ("/ops", "pkg/book.sh", "pkg/book.sh", None),
            (
                "C:/ops",
                "C:\\ops\\book.sh",
                "C:/ops/book.sh",
                Some("book.sh"),
            ),
        ] {
            let spellings = super::ScriptSpellings::of(book, &dorc_core::loadpath::Cwd::at(cwd));
            assert_eq!(
                spellings.text(super::Spelling::SlashBearing),
                Some(bearing),
                "standing in {cwd}, `{book}`"
            );
            assert_eq!(
                spellings.text(super::Spelling::Slashless),
                slashless,
                "standing in {cwd}, `sh {book}` is reachable by bare name only from its own dir"
            );
        }
    }

    /// A SLASH-LESS operand is a `PATH` search, which is outside v0 and outside what a kernel may
    /// answer — so it resolves nowhere and havocs, even when a file of that name is loaded
    /// (`30I` §3.2). The command-line spelling is a different question and keeps working: `-o
    /// lib.sh` names a file in the cwd, because a path OPERAND carries no slash-less refusal.
    #[test]
    fn a_slash_less_target_is_a_path_search_and_resolves_nowhere() {
        let book = ". lib.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let lib = add_def(&mut table, 0, ROLE);
        table.set_loadable("lib.sh", flat(vec![lib]));
        let (env, cfg, ast) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), ROLE), Flat::Top);
        assert_eq!(
            env.unresolvable_loads(),
            &BTreeSet::from([command_at(&cfg, &ast, book, ". lib.sh")]),
            "disclosed rather than silently walling — silence licenses nothing"
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
        table.set_loadable("./oracles/lib.sh", flat(vec![lib]));
        let (env, cfg, ast) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), ROLE), Flat::Top);
        assert_eq!(
            env.unresolvable_loads(),
            &BTreeSet::from([command_at(&cfg, &ast, book, ". \"$LIB/env\"")]),
        );
    }

    /// The shadow refusal reads a variable-resolved load exactly as it reads a literal one —
    /// the regime applies whole, so a cross-unit override arriving through `"$LIB/lib.sh"`
    /// draws the same complaint `a_top_level_re_source_trips_the_refusal` pins for `. ./lib.sh`.
    /// Without this, widening the resolvable set would have widened the SILENT set with it.
    #[test]
    fn a_variable_resolved_load_trips_the_shadow_refusal() {
        let book = "LIB=.\n. \"$LIB/lib.sh\"\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let outer = add_def(&mut table, 0, ROLE);
        table.push_ambient("ambient.sh", vec![outer]);
        let inner = add_def(&mut table, 1, ROLE);
        table.set_loadable("./lib.sh", flat(vec![inner]));
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
        table.set_loadable("./lib.sh", flat(vec![lib]));
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
        table.push_ambient("ambient.sh", vec![second]);
        let (env, cfg, ast) = solve_positional(book, &table);
        let live = LiveDefinitions::new(&env, &table);
        let site = command_at(&cfg, &ast, book, "yum install -y nginx");
        assert_eq!(live.source_before(site, ROLE), Some(SourceFileId(1)));
        let named = table.identity_of(second).expect("second id");
        assert_eq!(
            live.definition_before(site, ROLE),
            dorc_core::LiveDefinition::Live(named)
        );
        // ...and the id a row lifted from index 1 mints for itself is that same identity, which is
        // what lets a seat compare the two without a join.
        assert_eq!(super::row_definition(1, named.span()), named);
        assert_ne!(super::row_definition(0, named.span()), named);
    }

    // ── TABLE 6: the healthy library — a package's own top level as a load PROGRAM (`30I` §3) ──

    use crate::load::{LoadControl, LoadProgram, LoadStep, LoadTarget, TargetPart};

    fn no_span() -> Span {
        Span::new(BytePos(0), BytePos(0))
    }

    fn loads(target: LoadTarget) -> LoadControl {
        LoadControl::Load {
            target,
            span: no_span(),
        }
    }

    fn rooted(leaf: &str) -> LoadTarget {
        LoadTarget::of(vec![
            TargetPart::Param("OPS_LIB".to_owned()),
            TargetPart::Literal(leaf.to_owned()),
        ])
    }

    /// An entrypoint that loads `target` unless `function` is already live — the canonical
    /// shared-dependency shape (`30I` §2.2), as the loader sees it.
    fn guarded(function: &str, target: LoadTarget) -> LoadProgram {
        LoadProgram::of(vec![LoadStep::Control(LoadControl::Guard {
            condition: crate::load::LoadCondition::CommandV {
                function: function.to_owned(),
            },
            negated: false,
            then_: Vec::new(),
            else_: vec![loads(target)],
        })])
    }

    /// The canonical shared-dependency package: an entrypoint whose include guard loads a
    /// dependency through the ROOT ITS CALLER SET (`30I:force-root-value-flow` ·
    /// `30I:force-guarded-fallback`).
    ///
    /// Two things are pinned together because they only mean something together. The operand
    /// `"$OPS_LIB/common.sh"` lives inside the PACKAGE and its root lives in the BOOK, so nothing
    /// could have resolved it when the package was read; and the guard is what decides the
    /// dependency loads at all. The engine recognizes neither the variable's name nor the
    /// function's — any ordinary variable and any role-shaped name do this.
    #[test]
    fn a_books_root_reaches_a_packages_guarded_dependency() {
        let book = "OPS_LIB=./oracles\n. ./oracles/entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 1, ROLE);
        table.set_loadable("./oracles/common.sh", flat(vec![dependency]));
        table.set_loadable("./oracles/entry.sh", guarded(ROLE, rooted("/common.sh")));
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(dependency)),
            "the guard answers absent, so the dependency loads — through the caller's root"
        );
        assert!(
            env.unresolvable_loads().is_empty(),
            "and nothing was left unresolvable along the way"
        );
    }

    /// A root the book never set leaves the package's operand unreadable, so the load is
    /// UNRESOLVABLE and everything that file could have bound reads ⊤ — never a guessed file
    /// (`30I` §3.2). The safe direction, and the one that keeps the richness honest.
    #[test]
    fn a_package_whose_root_is_unset_resolves_nowhere() {
        let book = ". ./oracles/entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 1, ROLE);
        table.set_loadable("./oracles/common.sh", flat(vec![dependency]));
        table.set_loadable(
            "./oracles/entry.sh",
            LoadProgram::of(vec![LoadStep::Control(loads(rooted("/common.sh")))]),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), ROLE), Flat::Top);
        assert!(super::unprovable(&table, &env, cfg.exit()).contains(ROLE));
    }

    /// THE GUARD'S TWO DIRECTIONS, which are not symmetric (see [`super::run_control`]).
    ///
    /// A frame that proves the name undefined decides the guard FALSE only where
    /// `dec-decidable-set-v0`'s role-shaped warrant reaches; the same guard over an ORDINARY
    /// helper name joins both branches instead, because a host binary could still answer the
    /// query and deciding would model a fallback as loaded on a host that took the other branch.
    #[test]
    fn only_a_role_shaped_name_decides_the_absent_direction() {
        let book = ". ./entry.sh\nyum install -y nginx\n";

        let mut table = DefinitionTable::default();
        let fallback = add_def(&mut table, 1, ROLE);
        table.set_loadable("./fallback.sh", flat(vec![fallback]));
        table.set_loadable(
            "./entry.sh",
            guarded(ROLE, LoadTarget::literal("./fallback.sh")),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(fallback)),
            "nobody ships a binary called `yum__is_converged`, so absent really is absent"
        );

        let helper = "_common_query";
        let mut table = DefinitionTable::default();
        let fallback = add_def(&mut table, 1, helper);
        table.set_loadable("./fallback.sh", flat(vec![fallback]));
        table.set_loadable(
            "./entry.sh",
            guarded(helper, LoadTarget::literal("./fallback.sh")),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), helper),
            Flat::Top,
            "an ordinary name could be answered by a host binary, so neither branch is decided"
        );
    }

    /// The other direction, and the one that is sound unconditionally: a definition the frame
    /// names LIVE shadows every builtin and binary, so the query cannot fail and the guarded
    /// dependency does not load.
    #[test]
    fn a_live_definition_decides_the_guard_true() {
        let book = ". ./base.sh\n. ./entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let live = add_def(&mut table, 1, ROLE);
        let fallback = add_def(&mut table, 2, ROLE);
        table.set_loadable("./base.sh", flat(vec![live]));
        table.set_loadable("./fallback.sh", flat(vec![fallback]));
        table.set_loadable(
            "./entry.sh",
            guarded(ROLE, LoadTarget::literal("./fallback.sh")),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(live)),
            "the live body survives; the fallback was never loaded over it"
        );
    }

    /// THE DIAMOND: two entrypoints sharing one guarded dependency. The first load brings it in,
    /// the second finds it live and does not re-load — one definition, reached through two
    /// independent entrypoints, which is what `floor30-diamond-source-binds-once` measures a shell
    /// doing.
    #[test]
    fn two_entrypoints_share_one_guarded_dependency() {
        let book = ". ./alpha.sh\n. ./beta.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let shared = add_def(&mut table, 1, ROLE);
        table.set_loadable("./common.sh", flat(vec![shared]));
        table.set_loadable(
            "./alpha.sh",
            guarded(ROLE, LoadTarget::literal("./common.sh")),
        );
        table.set_loadable(
            "./beta.sh",
            guarded(ROLE, LoadTarget::literal("./common.sh")),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(shared))
        );
    }

    /// ONE ENTRYPOINT AT TWO POSITIONS, under two frames
    /// (`30I:rul-bundles-key-to-load-occurrences`).
    ///
    /// The same package text answers differently at the two load points because the ENVIRONMENT
    /// differs: ambiently the guarded name is undefined so the fallback loads, while inside a
    /// region that has already loaded a better body the guard holds and that body survives. The
    /// region's binding dies at the closing parenthesis, so the ambient answer stands afterwards.
    #[test]
    fn one_entrypoint_answers_its_own_frame_at_each_position() {
        let book = ". ./entry.sh\n( . ./better.sh\n. ./entry.sh )\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let fallback = add_def(&mut table, 1, ROLE);
        let better = add_def(&mut table, 2, ROLE);
        table.set_loadable("./fallback.sh", flat(vec![fallback]));
        table.set_loadable("./better.sh", flat(vec![better]));
        table.set_loadable(
            "./entry.sh",
            guarded(ROLE, LoadTarget::literal("./fallback.sh")),
        );
        let (env, cfg, ast) = solve_positional(book, &table);

        let first = command_at(&cfg, &ast, book, ". ./entry.sh");
        assert_eq!(
            env.binding_before(first, ROLE),
            Flat::Elem(Binding::Undefined),
            "the FIRST position sees nothing live, which is what makes its guard load the fallback"
        );
        let after = command_at(&cfg, &ast, book, "yum install -y nginx");
        assert_eq!(
            env.binding_before(after, ROLE),
            Flat::Elem(Binding::Defined(fallback)),
            "and the region's better body died at the parenthesis, so the ambient answer stands"
        );
    }

    /// A source CYCLE terminates by answering ⊤ rather than by recursing: a file already on the
    /// load stack is refused, the same rule and the same withholding direction as the depth cap.
    #[test]
    fn a_source_cycle_answers_top() {
        let book = ". ./a.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let def = add_def(&mut table, 1, ROLE);
        table.set_loadable(
            "./a.sh",
            LoadProgram::of(vec![
                LoadStep::Define(def),
                LoadStep::Control(loads(LoadTarget::literal("./b.sh"))),
            ]),
        );
        table.set_loadable(
            "./b.sh",
            LoadProgram::of(vec![LoadStep::Control(loads(LoadTarget::literal(
                "./a.sh",
            )))]),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), ROLE), Flat::Top);
    }

    /// `unset -f` inside a package removes a binding exactly as it does in a book — the removal
    /// half of `30I:rul-oracle-loading-stays-load-safe`'s positive surface.
    ///
    /// Asked at the SITE below the two loads rather than at the unit's exit, which is where
    /// `visibility-is-full-positional` puts every consuming act anyway. The exit node also joins
    /// the errexit failure-edges a `.` now owes (its file may `set -e` in the caller's own shell),
    /// so it answers ⊤ — honestly, since a shell that aborted at the FIRST load really does end
    /// with the earlier binding live.
    #[test]
    fn a_package_may_remove_a_binding() {
        let book = ". ./base.sh\n. ./strip.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let base = add_def(&mut table, 1, ROLE);
        table.set_loadable("./base.sh", flat(vec![base]));
        table.set_loadable(
            "./strip.sh",
            LoadProgram::of(vec![LoadStep::Control(LoadControl::UnsetFunctions(vec![
                ROLE.to_owned(),
            ]))]),
        );
        let (env, cfg, ast) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(command_at(&cfg, &ast, book, "yum install -y nginx"), ROLE),
            Flat::Elem(Binding::Undefined)
        );
    }

    /// A package that sets its own constant reads its OWN value for a dependency it sites with it,
    /// even when the caller set the same name — the value a shell would have live once the
    /// assignment has run.
    #[test]
    fn a_package_constant_shadows_the_callers() {
        let book = "OPS_LIB=./oracles\n. ./oracles/entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let vendored = add_def(&mut table, 1, ROLE);
        table.set_loadable("./vendored/common.sh", flat(vec![vendored]));
        table.set_loadable(
            "./oracles/entry.sh",
            LoadProgram::of(vec![
                LoadStep::Assign {
                    name: "OPS_LIB".to_owned(),
                    value: LoadTarget::literal("./vendored"),
                },
                LoadStep::Control(loads(rooted("/common.sh"))),
            ]),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(vendored))
        );
    }

    // ── TABLE 7: the exact package sentinel (`30I:rul-guarded-source-mints-exact-speaker-edge`) ──

    /// A package: it declares `defs` and populates its own load value, which is what makes the
    /// sentinel a fact about THIS package rather than a variable anyone could set.
    fn package(defs: Vec<DefId>, sentinel: &str, value: &str) -> LoadProgram {
        let mut steps: Vec<LoadStep> = defs.into_iter().map(LoadStep::Define).collect();
        steps.push(LoadStep::Assign {
            name: sentinel.to_owned(),
            value: LoadTarget::literal(value),
        });
        LoadProgram::of(steps)
    }

    /// `30I` §2.2's canonical entrypoint: load `target` unless the sentinel already says it is live.
    fn sentinel_guarded(sentinel: &str, value: &str, target: LoadTarget) -> LoadProgram {
        LoadProgram::of(vec![LoadStep::Control(LoadControl::Guard {
            condition: crate::load::LoadCondition::Value {
                name: sentinel.to_owned(),
                literal: value.to_owned(),
                equals: false,
            },
            negated: false,
            then_: vec![loads(target)],
            else_: Vec::new(),
        })])
    }

    const SENTINEL: &str = "sm_common_loaded";
    const VERSION: &str = "sm.common/v1";
    const HELPER: &str = "sm_common_query";

    /// The canonical cross-author shared dependency, resolved (`30I` §2.2 · §3.4). Two independent
    /// entrypoints guard on ONE package's own load value; both resolve to that package's helper.
    ///
    /// Under `command -v` over this same ordinary helper name neither guard decides, both branches
    /// join, and the helper binds ⊤ — which is why the sentinel is the exact-package guard and
    /// `command -v` is not (`notes/30Ic`). Recognition is still the engine SEEING that both arms
    /// land on the same speech; the VALUE it reads decides only WHICH arm, and it may read it
    /// because the two `Must`s above already proved the package is the world's only writer.
    #[test]
    fn a_recognized_sentinel_resolves_the_shared_dependency() {
        let book =
            "OPS_LIB=./oracles\n. ./oracles/alpha.sh\n. ./oracles/beta.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable(
            "./oracles/common.sh",
            package(vec![helper], SENTINEL, VERSION),
        );
        for entry in ["./oracles/alpha.sh", "./oracles/beta.sh"] {
            table.set_loadable(
                entry,
                sentinel_guarded(SENTINEL, VERSION, rooted("/common.sh")),
            );
        }
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), HELPER),
            Flat::Elem(Binding::Defined(helper)),
            "both arms land on the same body, so there is no analysis-time choice to drive to ⊤"
        );
    }

    /// A package whose live value is not the one its consumer's guard compares against is SOURCED
    /// AGAIN (promoted from the pin `p-x-sentinel-value-conjunct`).
    ///
    /// The sh fact: `[ "${sm_common_loaded-}" = 'sm.common/v2' ]` is FALSE while the live value is
    /// `sm.common/v1`, so the fallback `.` runs and `common.sh` executes twice. Modelling it as
    /// reused is a lost load-semantic, and `30I:rul-load-semantics-stay-full-fidelity` keeps the
    /// live constant and the compared literal in the full model for exactly this.
    ///
    /// Why an engine choice depends on it: a reuse the engine believes and the shell does not is a
    /// load OCCURRENCE the account never records, so every projection over it — bundle keying,
    /// artifact placement, custody edges — is short one file the run really loads.
    #[test]
    fn a_mismatched_sentinel_literal_takes_the_source_arm() {
        let book = ". ./common.sh\n. ./alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable("./common.sh", package(vec![helper], SENTINEL, VERSION));
        table.set_loadable(
            "./alpha.sh",
            sentinel_guarded(SENTINEL, "sm.common/v2", LoadTarget::literal("./common.sh")),
        );

        let (env, _, _) = solve_positional(book, &table);

        assert_eq!(
            targets_of(&env, LoadRoute::Reused),
            Vec::<&str>::new(),
            "the live v1 assignment cannot satisfy alpha's v2 comparison"
        );
        assert_eq!(
            targets_of(&env, LoadRoute::Taken),
            ["common.sh", "alpha.sh", "common.sh"],
            "POSIX sh executes both source operations — the book's own `. ./alpha.sh` sits between \
             them, and alpha's fallback is the second"
        );
    }

    /// The control for the cell above: the SAME shape with agreeing literals really does reuse, so
    /// the difference measured there is the comparison and nothing else.
    #[test]
    fn a_matching_sentinel_literal_takes_the_reuse_arm() {
        let book = ". ./common.sh\n. ./alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable("./common.sh", package(vec![helper], SENTINEL, VERSION));
        table.set_loadable(
            "./alpha.sh",
            sentinel_guarded(SENTINEL, VERSION, LoadTarget::literal("./common.sh")),
        );

        let (env, _, _) = solve_positional(book, &table);

        assert_eq!(
            targets_of(&env, LoadRoute::Reused),
            ["common.sh"],
            "the live value IS the one the guard compares, so nothing re-sources"
        );
        assert_eq!(
            targets_of(&env, LoadRoute::Taken),
            ["common.sh", "alpha.sh"],
            "and the only executions are the book's own two"
        );
    }

    /// A same-valued assignment from ANOTHER unit withholds. That unit could make the reuse arm
    /// reachable without the package ever loading, which is exactly the forgery the two-`Must`
    /// proof exists to refuse (`30I` §3.4).
    #[test]
    fn a_sentinel_another_unit_also_populates_withholds() {
        let book = ". ./oracles/alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        let stranger = add_def(&mut table, 2, "_unrelated");
        table.set_loadable(
            "./oracles/common.sh",
            package(vec![helper], SENTINEL, VERSION),
        );
        table.set_loadable(
            "./oracles/stranger.sh",
            package(vec![stranger], SENTINEL, VERSION),
        );
        table.set_loadable(
            "./oracles/alpha.sh",
            sentinel_guarded(
                SENTINEL,
                VERSION,
                LoadTarget::literal("./oracles/common.sh"),
            ),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), HELPER), Flat::Top);
    }

    /// The BOOK counts as another unit, and the value plane could not have told us: an assignment
    /// it reads as ⊤, or one sited below the load, is invisible there. The census is over NAMES.
    #[test]
    fn a_book_assigned_sentinel_withholds() {
        let book = ". ./oracles/alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable(
            "./oracles/common.sh",
            package(vec![helper], SENTINEL, VERSION),
        );
        table.set_loadable(
            "./oracles/alpha.sh",
            sentinel_guarded(
                SENTINEL,
                VERSION,
                LoadTarget::literal("./oracles/common.sh"),
            ),
        );
        table.set_book_assigns([SENTINEL.to_owned()]);
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), HELPER), Flat::Top);
    }

    /// A sentinel NOTHING populates withholds too — the other half of the same `Must`. Without it
    /// the condition is vacuously satisfied by an author's typo, and the only thing that could ever
    /// select the reuse arm is the invocation environment.
    #[test]
    fn a_sentinel_the_target_never_populates_withholds() {
        let book = ". ./oracles/alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable("./oracles/common.sh", flat(vec![helper]));
        table.set_loadable(
            "./oracles/alpha.sh",
            sentinel_guarded(
                SENTINEL,
                VERSION,
                LoadTarget::literal("./oracles/common.sh"),
            ),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), HELPER), Flat::Top);
    }

    /// The POLARITY is the idiom: a guard that loads when the sentinel MATCHES says something else
    /// entirely, and the engine has been shown nothing about what.
    #[test]
    fn loading_on_the_matching_arm_is_not_the_idiom() {
        let book = ". ./oracles/alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable(
            "./oracles/common.sh",
            package(vec![helper], SENTINEL, VERSION),
        );
        table.set_loadable(
            "./oracles/alpha.sh",
            LoadProgram::of(vec![LoadStep::Control(LoadControl::Guard {
                condition: crate::load::LoadCondition::Value {
                    name: SENTINEL.to_owned(),
                    literal: VERSION.to_owned(),
                    equals: true,
                },
                negated: false,
                then_: vec![loads(LoadTarget::literal("./oracles/common.sh"))],
                else_: Vec::new(),
            })]),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), HELPER), Flat::Top);
    }

    /// The ARM is read off the environment, never assumed. A book's own hand-written function over
    /// a package name is `30I` §13's named mislead: the sentinel may well be set — the package
    /// loaded earlier — while the live binding is the book's, so the two arms no longer land on the
    /// same speech and the recognition declines rather than picking one.
    #[test]
    fn a_package_name_shadowed_from_outside_withholds() {
        let book = format!(
            "{HELPER}() {{ common high \"$@\" ;}}\n. ./oracles/alpha.sh\nyum install -y nginx\n"
        );
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable(
            "./oracles/common.sh",
            package(vec![helper], SENTINEL, VERSION),
        );
        table.set_loadable(
            "./oracles/alpha.sh",
            sentinel_guarded(
                SENTINEL,
                VERSION,
                LoadTarget::literal("./oracles/common.sh"),
            ),
        );
        for (id, node) in dorc_syntax::parse(&book).value.iter() {
            if let NodeKind::FuncDef { name, .. } = &node.kind {
                let def = add_def(&mut table, 0, name);
                table.set_book_site(id, def);
            }
        }
        let (env, cfg, _) = solve_positional(&book, &table);
        assert_eq!(env.binding_before(cfg.exit(), HELPER), Flat::Top);
    }

    /// A removal elsewhere in the loaded world withholds: an `unset -f` and redefine is one of the
    /// named ways the shape can mislead, and on the reuse arm it means the live body is not the
    /// target's after all.
    #[test]
    fn a_removal_of_the_targets_own_name_withholds() {
        let book = ". ./oracles/alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable(
            "./oracles/common.sh",
            package(vec![helper], SENTINEL, VERSION),
        );
        table.set_loadable(
            "./oracles/strip.sh",
            LoadProgram::of(vec![LoadStep::Control(LoadControl::UnsetFunctions(vec![
                HELPER.to_owned(),
            ]))]),
        );
        table.set_loadable(
            "./oracles/alpha.sh",
            sentinel_guarded(
                SENTINEL,
                VERSION,
                LoadTarget::literal("./oracles/common.sh"),
            ),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(env.binding_before(cfg.exit(), HELPER), Flat::Top);
    }

    /// A PRE-SOURCE IS A `.` (`30I:rul-pre-source-is-dot-prelude`): a source the invocation named
    /// runs its own top-level program before the book's first line, so its include guard decides
    /// and its own dependency loads.
    ///
    /// Under the flat declaration list this replaced, a CLI-named package's guard was never
    /// evaluated at all and its guarded dependency bound unconditionally — the same file, read as
    /// a bag of definitions rather than as the program its author wrote.
    #[test]
    fn a_pre_source_runs_its_own_program() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 1, ROLE);
        table.set_loadable("./common.sh", flat(vec![dependency]));
        table.set_loadable(
            "./entry.sh",
            guarded(ROLE, LoadTarget::literal("./common.sh")),
        );
        table.push_ambient("./entry.sh", Vec::new());
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(dependency)),
            "the prelude's guard answered absent, so its dependency loaded"
        );
    }

    /// The prelude is ORDERED, and one `.`'s variables are live for the next — a shell's own
    /// behaviour, and what lets one pre-source site the next one's dependency.
    #[test]
    fn one_pre_source_sites_the_next() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 2, ROLE);
        table.set_loadable("./vendored/common.sh", flat(vec![dependency]));
        table.set_loadable(
            "./root.sh",
            LoadProgram::of(vec![LoadStep::Assign {
                name: "OPS_LIB".to_owned(),
                value: LoadTarget::literal("./vendored"),
            }]),
        );
        table.set_loadable(
            "./entry.sh",
            LoadProgram::of(vec![LoadStep::Control(loads(rooted("/common.sh")))]),
        );
        table.push_ambient("./root.sh", Vec::new());
        table.push_ambient("./entry.sh", Vec::new());
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(dependency))
        );
    }

    /// A source with no program on file — an unmarked one, which makes no dialect claim — keeps
    /// contributing its flat declarations. Nothing about the prelude's richness may cost a plain
    /// file its binding.
    #[test]
    fn a_pre_source_with_no_program_still_binds_its_declarations() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let plain = add_def(&mut table, 0, ROLE);
        table.push_ambient("./plain.sh", vec![plain]);
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(plain))
        );
    }

    /// A `.` RUNS IN ITS CALLER'S SHELL (`30I:rul-dot-resolves-as-sh`): a nested load's top-level
    /// assignments are live for everything its sourcer does afterwards, so one file can site the
    /// next one's dependency. Handed a COPY, they died with it
    /// (`30Mc:finding-dot-locals-are-discarded`).
    #[test]
    fn a_nested_loads_assignment_sites_its_sourcers_next_load() {
        let book = ". ./entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 2, ROLE);
        table.set_loadable("./vendored/common.sh", flat(vec![dependency]));
        table.set_loadable(
            "./root.sh",
            LoadProgram::of(vec![LoadStep::Assign {
                name: "OPS_LIB".to_owned(),
                value: LoadTarget::literal("./vendored"),
            }]),
        );
        table.set_loadable(
            "./entry.sh",
            LoadProgram::of(vec![
                LoadStep::Control(loads(LoadTarget::literal("./root.sh"))),
                LoadStep::Control(loads(rooted("/common.sh"))),
            ]),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(dependency))
        );
        assert!(targets_of(&env, LoadRoute::Taken).contains(&"vendored/common.sh"));
    }

    /// THE SCOPE FLOOR: a `.` inside a subshell assigns inside it, so a load after the closing
    /// paren resolves nowhere. Held today for the broader reason that no book-level `.` propagates
    /// variables at all (`30Na:fnd-book-level-dot-locals-need-a-domain`); load-bearing when they do.
    #[test]
    fn a_subshell_scoped_sources_assignment_dies_at_the_closing_paren() {
        let book = "(\n   . ./root.sh\n)\n. ./entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 2, ROLE);
        table.set_loadable("./vendored/common.sh", flat(vec![dependency]));
        table.set_loadable(
            "./root.sh",
            LoadProgram::of(vec![LoadStep::Assign {
                name: "OPS_LIB".to_owned(),
                value: LoadTarget::literal("./vendored"),
            }]),
        );
        table.set_loadable(
            "./entry.sh",
            LoadProgram::of(vec![LoadStep::Control(loads(rooted("/common.sh")))]),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert!(!targets_of(&env, LoadRoute::Taken).contains(&"vendored/common.sh"));
        assert_ne!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Elem(Binding::Defined(dependency)),
            "and nothing the paren-scoped source reached can bind past it"
        );
    }

    /// CONTAINMENT (`30Mb` §9): a value only the HOST ENVIRONMENT could have supplied moves the
    /// license plane nowhere, on either door. A variable no assignment in the program populates
    /// reads ⊥, so a load built from it havocs; and the sentinel census counts ASSIGNMENTS, so an
    /// unpopulated name decides no guard. Both land on ⊤ — the run direction — so an exported
    /// `SM_COMMON_LOADED` cannot buy a reuse arm nor an exported root pick which file answers.
    #[test]
    fn a_host_environment_value_neither_sites_a_load_nor_decides_a_guard() {
        let mut sited = DefinitionTable::default();
        let dependency = add_def(&mut sited, 1, ROLE);
        sited.set_loadable("./vendored/common.sh", flat(vec![dependency]));
        let (env, cfg, _) = solve_positional(". \"$SM_ROOT/common.sh\"\ntrue\n", &sited);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Top,
            "a root only the environment could hold sites nothing"
        );
        assert!(targets_of(&env, LoadRoute::Taken).is_empty());

        let mut guarded = DefinitionTable::default();
        let helper = add_def(&mut guarded, 1, HELPER);
        // The dependency declares the helper and assigns NO sentinel: the only thing that could set
        // one is the host environment.
        guarded.set_loadable("./oracles/common.sh", flat(vec![helper]));
        guarded.set_loadable(
            "./oracles/alpha.sh",
            sentinel_guarded(
                SENTINEL,
                VERSION,
                LoadTarget::literal("./oracles/common.sh"),
            ),
        );
        let (env, cfg, _) = solve_positional(". ./oracles/alpha.sh\ntrue\n", &guarded);
        assert_eq!(
            env.binding_before(cfg.exit(), HELPER),
            Flat::Top,
            "an unpopulated sentinel decides nothing: both arms walk and the binding joins to ⊤"
        );
    }

    /// `funcenv-reads-source-literal-plane-only`, as a TABLE: exactly one of the five provenances
    /// may site a load, the other four being the value-prediction species (`275` §1). Vacuous in
    /// EFFECT today (nothing mints a middle grade) and deliberately not in FORM — this reddens when
    /// somebody widens the gate ahead of the licensure review that widening needs.
    #[test]
    fn only_program_text_may_site_a_load() {
        use dorc_core::ValueGrade;

        let admitted: Vec<ValueGrade> = [
            ValueGrade::Top,
            ValueGrade::AuthorComposed,
            ValueGrade::WorldSpoken,
            ValueGrade::Register,
            ValueGrade::ProgramText,
        ]
        .into_iter()
        .filter(|grade| super::admits_a_load(*grade))
        .collect();
        assert_eq!(admitted, [ValueGrade::ProgramText]);
        assert!(
            super::admits_a_load(super::VARIABLE_PLANE_GRADE),
            "and the variable plane's whole-hog grade is one the gate admits — the day it is not, \
             variable-sited loads stop resolving rather than resolving off a host's answer"
        );
    }

    /// THE PRELUDE FLOOR (conductor default at `30Mg` R1; human veto invited): an unresolvable act
    /// inside a prelude's load program floors the WHOLE prelude from that point. Sh ran that `.`
    /// and what it did is unknowable from there on, so ⊤ beats a per-subtree suspension that would
    /// let a later root's bindings license sites (`rul-unsure-falls-toward-sh-parity`).
    #[test]
    fn an_unresolvable_prelude_load_floors_the_rest_of_the_prelude() {
        let book = "yum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let later = add_def(&mut table, 1, ROLE);
        table.set_loadable(
            "./entry.sh",
            LoadProgram::of(vec![LoadStep::Control(loads(LoadTarget::literal(
                "./never-loaded.sh",
            )))]),
        );
        table.set_loadable("./later.sh", flat(vec![later]));
        table.push_ambient("./entry.sh", Vec::new());
        table.push_ambient("./later.sh", vec![later]);
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), ROLE),
            Flat::Top,
            "a root sequenced AFTER the unresolvable act binds nothing"
        );
    }

    // ── TABLE 8: the ONE load account and its three projections
    //    (`30I:rul-one-load-account-separate-projections`) ──

    use crate::load::{LoadRoute, LoadSourcer};

    fn targets_of(env: &FuncEnv, route: LoadRoute) -> Vec<&str> {
        env.loads()
            .occurrences()
            .iter()
            .filter(|occurrence| occurrence.route == route)
            .map(|occurrence| occurrence.target.as_str())
            .collect()
    }

    /// TARGET: two BOOK-level `.`s, the first assigning the root the second's operand is built
    /// from. Sh keeps that variable; the engine does not, because a book's `.` sites are separate
    /// CFG nodes each minting a fresh variable map. Closing it needs variables in this domain (or
    /// the value plane learning what a `.` assigns) — a winner-shifting change with its own
    /// monotonicity question, hence a pin (`30Na:fnd-book-level-dot-locals-need-a-domain`). The
    /// NESTED cell above was the design-free half and is fixed.
    #[test]
    fn a_sourced_assignment_sites_a_later_load() {
        let book = ". ./root.sh\n. ./entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let dependency = add_def(&mut table, 1, ROLE);
        table.set_loadable("./vendored/common.sh", flat(vec![dependency]));
        table.set_loadable(
            "./root.sh",
            LoadProgram::of(vec![LoadStep::Assign {
                name: "OPS_LIB".to_owned(),
                value: LoadTarget::literal("./vendored"),
            }]),
        );
        table.set_loadable(
            "./entry.sh",
            LoadProgram::of(vec![LoadStep::Control(loads(rooted("/common.sh")))]),
        );

        let (env, cfg, _) = solve_positional(book, &table);

        internal_tooling::xfail::xfail_until("p-x-book-level-dot-locals", || {
            assert_eq!(
                targets_of(&env, LoadRoute::Taken),
                ["root.sh", "entry.sh", "vendored/common.sh"]
            );
            assert_eq!(
                env.binding_before(cfg.exit(), ROLE),
                Flat::Elem(Binding::Defined(dependency))
            );
        });
    }

    /// THE BLOCKER THIS TABLE DISCHARGES (`30Ib:fnd-the-loader-reports-no-unfiltered-edge-set`):
    /// an undecided guard's fallback target is ABSENT from the speaker projection and PRESENT in
    /// the possible-load one.
    ///
    /// Both halves are load-bearing and they pull opposite ways. No authority may rest on a branch
    /// nobody decided (`rul-speaker-minting-is-oracle-sourcing-only`), so the edge must not mint;
    /// but a bundle built from the speaker edges alone would OMIT a file the runtime `.` really may
    /// load, which is an artifact that does not reproduce its own book. One account, two answers.
    #[test]
    fn an_undecided_guards_fallback_is_possible_but_never_a_speaker() {
        let book = ". ./entry.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable("./common.sh", flat(vec![helper]));
        table.set_loadable(
            "./entry.sh",
            guarded(HELPER, LoadTarget::literal("./common.sh")),
        );
        let (env, cfg, _) = solve_positional(book, &table);
        assert_eq!(
            env.binding_before(cfg.exit(), HELPER),
            Flat::Top,
            "an ordinary helper name decides neither way, which is what makes this the cell"
        );
        assert_eq!(
            targets_of(&env, LoadRoute::Speculative),
            ["common.sh"],
            "the fallback IS a possible load — a bundle omitting it would not reproduce the book"
        );
        assert_eq!(
            env.loads().speaker_edges(),
            BTreeSet::new(),
            "...and it mints nothing: an undecided branch rests no licence on anyone"
        );
        assert_eq!(
            env.loads().selection_edges(),
            BTreeSet::from([("entry.sh".to_owned(), "common.sh".to_owned())]),
            "the author SELECTED it all the same, which is the narrative projection's whole job"
        );
    }

    /// The recognized sentinel's REUSE arm records an occurrence with no `.` behind it, and mints.
    /// `30I` §3.4 case 2 in one assertion: the guard is one authored dependency act whichever arm
    /// the environment is in, so the edge exists even where another package loaded the target first.
    #[test]
    fn a_reuse_arm_records_its_occurrence_and_mints() {
        let book = ". ./common.sh\n. ./alpha.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable("./common.sh", package(vec![helper], SENTINEL, VERSION));
        table.set_loadable(
            "./alpha.sh",
            sentinel_guarded(SENTINEL, VERSION, LoadTarget::literal("./common.sh")),
        );
        let (env, _, _) = solve_positional(book, &table);
        assert_eq!(targets_of(&env, LoadRoute::Reused), ["common.sh"]);
        assert_eq!(
            env.loads().speaker_edges(),
            BTreeSet::from([("alpha.sh".to_owned(), "common.sh".to_owned())])
        );
    }

    /// OCCURRENCE IDENTITY IS NOT A TARGET PAIR (`30I` §6.1's insufficiency clause): two textual
    /// load points naming ONE entrypoint are two occurrences, each with its own locus and its own
    /// enclosing act, and a bundle keyed by pairs would collapse them into one
    /// (`rul-bundles-key-to-load-occurrences`).
    #[test]
    fn two_load_points_naming_one_target_are_two_occurrences() {
        let book = ". ./alpha.sh\n. ./beta.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let helper = add_def(&mut table, 1, HELPER);
        table.set_loadable("./common.sh", flat(vec![helper]));
        for entry in ["./alpha.sh", "./beta.sh"] {
            table.set_loadable(
                entry,
                LoadProgram::of(vec![LoadStep::Control(loads(LoadTarget::literal(
                    "./common.sh",
                )))]),
            );
        }
        let (env, _, _) = solve_positional(book, &table);
        let account = env.loads();
        let common: Vec<&crate::load::LoadOccurrence> = account
            .occurrences()
            .iter()
            .filter(|occurrence| occurrence.target == "common.sh")
            .collect();
        assert_eq!(
            common.len(),
            2,
            "one per textual load point, never one per file"
        );
        let parents: Vec<&LoadSourcer> = common
            .iter()
            .filter_map(|occurrence| occurrence.within)
            .filter_map(|parent| account.occurrences().get(parent))
            .map(|parent| &parent.sourcer)
            .collect();
        assert_eq!(
            parents,
            [&LoadSourcer::Book, &LoadSourcer::Book],
            "each names the ROOT act it descends from, which is what a locator composes onto"
        );
        assert_eq!(
            account.speaker_edges().len(),
            2,
            "and the pair set they collapse to has lost exactly that distinction"
        );
    }

    /// The root acts are recorded, and their SOURCER SPECIES is what says a book `.` and a
    /// pre-source mint nothing — by the type, rather than by a filter every consumer must remember
    /// (`30I:rul-books-load-but-do-not-speak` · `rul-cli-coloading-composes-nothing`).
    #[test]
    fn root_acts_carry_the_species_that_mints_nothing() {
        let book = ". ./sourced.sh\nyum install -y nginx\n";
        let mut table = DefinitionTable::default();
        let a = add_def(&mut table, 1, HELPER);
        let b = add_def(&mut table, 2, ROLE);
        table.set_loadable("./sourced.sh", flat(vec![a]));
        table.set_loadable("./named.sh", flat(vec![b]));
        table.push_ambient("./named.sh", Vec::new());
        let (env, _, _) = solve_positional(book, &table);
        let species: Vec<(&LoadSourcer, &str)> = env
            .loads()
            .occurrences()
            .iter()
            .map(|occurrence| (&occurrence.sourcer, occurrence.target.as_str()))
            .collect();
        assert_eq!(
            species,
            [
                (&LoadSourcer::Invocation, "named.sh"),
                (&LoadSourcer::Book, "sourced.sh"),
            ]
        );
        assert_eq!(env.loads().speaker_edges(), BTreeSet::new());
        assert_eq!(env.loads().selection_edges(), BTreeSet::new());
    }
}
