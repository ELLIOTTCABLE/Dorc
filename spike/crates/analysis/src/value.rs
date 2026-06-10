//! `analysis::value` — book-side value-flow analysis (`19H §1`, `202 §1` face-book):
//! flow-sensitive constant + variable propagation over the existing [`Cfg`], solved by the
//! existing monotone worklist ([`crate::solve`]).
//!
//! For each command-site it answers: *what are this command's argv words, resolved?* Each
//! word is either a known literal string (interned to a [`Symbol`]) or [`ValueOf::Top`]
//! (`⊤` — runtime-dynamic / unmodeled / lost to quoting). This is the input the
//! entity-resolution pass (`19H §1.2`) and the post-probe fold (`19H §1.5`) both need; this
//! module computes the propagation only — it resolves nothing about oracles, picks no phase,
//! and licenses no elision (`inv-superposition`: phase-/orientation-agnostic facts).
//!
//! # The domain, and why the entry is seeded ⊤
//!
//! Per program point the state is a map *variable name ↦ abstract string value*. The abstract
//! value is the textbook constant-propagation element [`Flat`] (height 2: ⊥ below the
//! literals below ⊤) carried over **owned text** (`Flat<String>`) so concatenation needs no
//! live interner; [`MapL`] supplies the pointwise join, deterministic ordered iteration
//! (`inv-determinism`), and the canonical no-⊥ form. The final literal text is interned to a
//! [`Symbol`] only at the public boundary, where [`analyze`] holds the [`Interner`].
//!
//! The one non-obvious move: the **entry node seeds every assigned variable to `⊤`**, not the
//! worklist's default ⊥. This is required for shell-correctness *and* to ride `MapL`
//! unmodified (the MapL-friction flagged in the round-20 note). In shell an unset variable is
//! not a constant, so "uninitialised ⇒ ⊤"; without the seed, the half-assigned branch
//! `if c; then pkg=a; fi` would wrongly resolve `pkg` to `a`, because the else-path leaves
//! `pkg` *absent* and `MapL`'s pointwise join treats absent as ⊥ (its canonical-form premise:
//! absent ≡ ⊥), so `Elem(a) ⊔ ⊥ = Elem(a)`. Seeding the else-path's `pkg` to an explicit `⊤`
//! at entry makes the join `Elem(a) ⊔ ⊤ = ⊤` — the correct "maybe-`a`, maybe-unset ⇒
//! unknown". The seed is monotone (a constant function at the pred-less entry) and preserves
//! the worklist's ⊥-identity, so the lattice laws and termination hold.
//!
//! # Soundness posture (`19H §1.3`)
//!
//! Wherever propagation cannot follow a value it degrades to `⊤`, and a `⊤` word means the
//! consumer must run the command with that argument unparsed — the apply-direction floor
//! (`kFAIL-perform`). Non-convergence of the worklist folds the **entire** result to all-`⊤`
//! (`16P` DP-9): a capped solve is an under-approximation we must not trust.

use std::collections::{BTreeMap, BTreeSet};

use dorc_core::{AstId, Interner, Symbol};
use dorc_syntax::ast::{Ast, NodeKind, WordPart};
use dorc_syntax::sem::{self, FragClass};

use crate::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use crate::lattice::{Flat, Lattice, MapL};
use crate::solve::{Direction, solve};

/// One resolved word: a statically-known literal string, or `⊤` (unknown).
///
/// The literal carries a specific guarantee the entity-resolution consumer relies on
/// (`19H §2.7`, flagged for the wiring task): a [`ValueOf::Literal`] is the **fully expanded,
/// non-word-splitting** string the shell would pass as a single argument — every part of the
/// originating word was a literal or a variable resolved to a literal, with no
/// command-substitution, no arithmetic, no operator-expansion (`${x:-y}`), and no unquoted
/// expansion that could word-split or glob. Anything weaker is [`ValueOf::Top`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueOf {
    /// A statically-known argument value (interned).
    Literal(Symbol),
    /// `⊤`: runtime-dynamic, unmodeled, or lost to quoting/splitting ⇒ the consumer must
    /// treat the argument as unknown (`kFAIL`: run, do not elide on it).
    Top,
}

/// The queryable result of the value-flow analysis: per command-site argv values.
///
/// Construct with [`analyze`]; query with [`ValueFlow::argv_values`] (keyed by the
/// [`CfgNodeId`] of a `Command` node). [`ValueFlow::converged`] mirrors the worklist's
/// convergence; when it is `false`, every query already returns all-`⊤` internally (`16P`
/// DP-9), so a consumer need not re-check it — but it is exposed for diagnostics.
#[derive(Debug, Clone)]
pub struct ValueFlow {
    /// Per `Command` node: its resolved argv (command word first, then args, in order).
    /// Absent for non-`Command` nodes. When `!converged`, populated entirely with `⊤`.
    argv: BTreeMap<CfgNodeId, Vec<ValueOf>>,
    converged: bool,
}

impl ValueFlow {
    /// The resolved argv of a command-site: the command word followed by every argument
    /// word, in source order, each a [`ValueOf`]. Empty for a bare assignment-only command
    /// (`pkg=nginx`, no command word) and for any non-`Command` node.
    ///
    /// Per-word independence is the contract (`202 §1`): `apt-get install -y "$dyn"` yields
    /// `[Literal, Literal, Literal, Top]` — the dynamic word is `⊤`, its literal neighbours
    /// are not. Collapsing a partially-`⊤` argv to a single verdict is the *consumer's* rule
    /// (202 §1's fully-concrete-argv scope), never imposed here.
    #[must_use]
    pub fn argv_values(&self, node: CfgNodeId) -> Vec<ValueOf> {
        self.argv.get(&node).cloned().unwrap_or_default()
    }

    /// Did the underlying worklist reach a fixed point? `false` ⇒ all queries are all-`⊤`
    /// (the non-convergence fold, `16P` DP-9).
    #[must_use]
    pub fn converged(&self) -> bool {
        self.converged
    }
}

/// The dataflow lattice element: shell variable name ↦ abstract string value (owned text, so
/// concatenation is interner-free; interned only at the public boundary).
type ValueEnv = MapL<String, Flat<String>>;

/// An abstract word value mid-analysis: known literal text, or `⊤`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Abstract {
    Lit(String),
    Top,
}

/// Read a variable's abstract value, treating **absent as `⊤`** (`19H`: unset-is-⊤;
/// `unset name` ⇒ ⊤). A variable present in the map carries its tracked `Flat`; one no
/// assignment anywhere touches is absent ⇒ unknown.
fn lookup(env: &ValueEnv, var: &str) -> Abstract {
    match env.get(&var.to_owned()) {
        Flat::Elem(s) => Abstract::Lit(s),
        Flat::Bottom | Flat::Top => Abstract::Top,
    }
}

/// Run the value-flow analysis over a built [`Cfg`] and its [`Ast`]. Total, deterministic,
/// never panics (`inv-no-throw`). The `interner` is threaded so final literal text resolves
/// into the same [`Symbol`] space the rest of the engine uses.
#[must_use]
pub fn analyze(cfg: &Cfg, ast: &Ast, interner: &mut Interner) -> ValueFlow {
    let prep = Prep::new(cfg, ast);

    // Forward constant/variable propagation. `transfer` is monotone over a finite-height
    // domain (keys ⊆ the script's assigned-variable set, values height-2 `Flat`), so the
    // worklist converges (`inv-monotonicity`); the entry seed is the only non-pass-through
    // boundary (see the module doc).
    let solution = solve(cfg, Direction::Forward, |i, incoming: &ValueEnv| {
        prep.transfer(CfgNodeId(u32::try_from(i).unwrap_or(u32::MAX)), incoming)
    });

    // `16P` DP-9 / `inv-probe-sourced-values`: a non-converged solve is an under-
    // approximation; fold every site to all-`⊤` rather than trust a partial fixed point.
    let mut argv = BTreeMap::new();
    for (id, node) in cfg.iter() {
        if node.kind != CfgNodeKind::Command {
            continue;
        }
        let words = prep.site_argv(id, &solution.states, solution.converged);
        argv.insert(id, intern_argv(words, interner));
    }

    ValueFlow {
        argv,
        converged: solution.converged,
    }
}

/// Intern a resolved-argv vector's literals into [`ValueOf`].
fn intern_argv(words: Vec<Abstract>, interner: &mut Interner) -> Vec<ValueOf> {
    words
        .into_iter()
        .map(|w| match w {
            Abstract::Lit(s) => ValueOf::Literal(interner.intern(&s)),
            Abstract::Top => ValueOf::Top,
        })
        .collect()
}

/// Precomputed, pure structure the transfer and resolution both read: the program's
/// assigned-variable set (for the entry ⊤-seed), per-`ScopeExit` "assigned-inside" sets (for
/// subshell containment), and per-node assignment recipes (so the transfer is interner-free).
struct Prep<'a> {
    cfg: &'a Cfg,
    ast: &'a Ast,
    /// Every variable name assigned anywhere (seeded ⊤ at entry). `BTreeSet` for
    /// deterministic seed-iteration (`inv-determinism`).
    assigned_vars: BTreeSet<String>,
    /// Per `ScopeExit` node index: the variable names assigned anywhere inside the matching
    /// scope. At the exit those are forced to `⊤` so a subshell binding cannot leak past
    /// `( )`/`$( )` (`an-leaf-scope`; the sound containment fallback of `19H`).
    scope_exit_clobbers: BTreeMap<usize, BTreeSet<String>>,
    /// Per assignment-bearing AST node: `(lhs-name, RHS-recipe)` per assignment, in source
    /// order. Built once so the transfer/resolution closures stay pure.
    assigns: BTreeMap<AstId, Vec<(String, Recipe)>>,
}

/// A flattened recipe for one word's value: the ordered fragments to concatenate. Any
/// fragment the analysis cannot turn into a literal makes the whole word `⊤` (`19H`: a word
/// containing a `⊤`-var or an unmodeled expansion is `⊤`).
#[derive(Debug, Clone)]
enum Recipe {
    /// Unconditionally `⊤` (held an unmodeled/dynamic part, or an unquoted splitting
    /// expansion): no point tracking fragments.
    Top,
    /// Concatenate these fragments left-to-right; if any resolves to `⊤`, the word is `⊤`.
    Parts(Vec<Frag>),
}

/// One concatenation fragment of a [`Recipe`].
#[derive(Debug, Clone)]
enum Frag {
    /// Literal text.
    Lit(String),
    /// A plain-variable reference, resolved against the per-point state at use.
    Var(String),
}

impl<'a> Prep<'a> {
    fn new(cfg: &'a Cfg, ast: &'a Ast) -> Self {
        let mut assigned_vars = BTreeSet::new();
        let mut assigns: BTreeMap<AstId, Vec<(String, Recipe)>> = BTreeMap::new();

        // Every Command node's source `Simple` carries assignments (both the bare
        // `pkg=nginx` form and the command-prefix `FOO=bar cmd` form sit in `Simple.assigns`).
        for (_, node) in cfg.iter() {
            if node.kind != CfgNodeKind::Command || assigns.contains_key(&node.ast) {
                continue;
            }
            let NodeKind::Simple { assigns: a_ids, .. } = &ast.node(node.ast).kind else {
                continue;
            };
            let mut list = Vec::new();
            for &a in a_ids {
                let NodeKind::Assign { name, value, .. } = &ast.node(a).kind else {
                    continue;
                };
                assigned_vars.insert(name.clone());
                let recipe = match value {
                    None => Recipe::Parts(Vec::new()), // `name=` ⇒ empty literal
                    Some(v) => recipe_of_word(ast, *v),
                };
                list.push((name.clone(), recipe));
            }
            assigns.insert(node.ast, list);
        }

        let scope_exit_clobbers = compute_scope_clobbers(cfg, ast);

        Prep {
            cfg,
            ast,
            assigned_vars,
            scope_exit_clobbers,
            assigns,
        }
    }

    /// The monotone per-node transfer. Forward; `incoming` is the join of predecessors'
    /// outputs (`solve` semantics).
    fn transfer(&self, id: CfgNodeId, incoming: &ValueEnv) -> ValueEnv {
        match self.cfg.node(id).kind {
            // Entry seeds every assigned variable to ⊤ (uninitialised-is-⊤; module doc).
            CfgNodeKind::Entry => {
                let mut env = ValueEnv::default();
                for v in &self.assigned_vars {
                    env.insert(v.clone(), Flat::Top);
                }
                env
            }
            // ScopeExit: a subshell `( )` / `$( )` does not leak var/env mutations
            // (`an-leaf-scope`). Force every variable assigned inside the scope to ⊤.
            CfgNodeKind::ScopeExit => {
                let mut env = incoming.clone();
                if let Some(clobbers) = self.scope_exit_clobbers.get(&id.index()) {
                    for v in clobbers {
                        env.insert(v.clone(), Flat::Top);
                    }
                }
                env
            }
            CfgNodeKind::Command => self.transfer_command(id, incoming),
            // A ⊤-rejected region (`eval`, a no-`in` `for`, `break`/`continue`, …) is
            // UNPARSED: its body may assign anything, invisibly — half-modeling it as a
            // no-op is the `DP-8` trap, and a stale literal surviving past it is a
            // confidently-wrong propagation (the no-floor class, `19H §1.3`). Havoc every
            // tracked variable; untracked ones are absent-as-⊤ already (`lookup`). NB:
            // PARSED loops are NOT `Top` (task-L1) — their body is visible, so the
            // back-edge JOIN (`LoopHead`) does the right thing without this havoc.
            CfgNodeKind::Top => {
                let mut env = incoming.clone();
                for v in &self.assigned_vars {
                    env.insert(v.clone(), Flat::Top);
                }
                env
            }
            // A `for` loop head binds its iteration variable to the JOIN of the list
            // words at body entry (task-L1, `209` brk-1: Flat — one word ⇒ that literal,
            // >1 distinct ⇒ ⊤; the Powerset precision is the later member-elision slice).
            // The binding overwrites whatever the back-edge carried for that var (the
            // loop var is reset each iteration), and is resolved against `incoming` (the
            // post-join state, so a body reassignment of a word-referenced var is seen).
            // A `while`/`until` head binds nothing (no loop var) ⇒ pass-through.
            CfgNodeKind::LoopHead => self.transfer_loop_head(id, incoming),
            // Merge / Redir / ScopeEnter / Exit carry state through unchanged (they bind no
            // variable in the modeled subset).
            _ => incoming.clone(),
        }
    }

    /// A `for`-loop [`CfgNodeKind::LoopHead`] transfer (task-L1): bind the iteration
    /// variable to the JOIN of the list words, resolved against `incoming`. One word
    /// ⇒ that literal (`for f in nginx` ⇒ `f = nginx`); ≥2 distinct, any ⊤/unresolvable
    /// word, or an empty list ⇒ `⊤` (the Flat join saturates — `for x in a b` ⇒ `x = ⊤`,
    /// the Powerset precision deferred to the member-elision slice). A `while`/`until`
    /// head (or a `for` whose AST we cannot read) binds nothing.
    fn transfer_loop_head(&self, id: CfgNodeId, incoming: &ValueEnv) -> ValueEnv {
        let NodeKind::ForLoop { var, words, .. } = &self.ast.node(self.cfg.node(id).ast).kind
        else {
            return incoming.clone(); // while/until head: no loop var
        };
        // JOIN the resolved words into one Flat. Empty list ⇒ ⊤ (the body's uses see an
        // unset/0-iteration var; never a stale literal). `Flat::join` saturates two
        // distinct literals to ⊤ — exactly the >1-element rule.
        let mut acc = Flat::Bottom;
        for &w in words {
            acc = acc.join(&flat_of(&resolve_recipe(
                &recipe_of_word(self.ast, w),
                incoming,
            )));
        }
        let bound = match (words.is_empty(), acc) {
            (true, _) => Flat::Top, // empty list ⇒ ⊤ (cannot enumerate / ran 0 times)
            (false, f) => f,
        };
        let mut env = incoming.clone();
        env.insert(var.clone(), bound);
        env
    }

    /// A `Command` node's transfer. A bare assignment-only command (`pkg=nginx`, no words)
    /// *persists* its bindings; a command with a prefix (`FOO=bar cmd`) does **not** — the
    /// prefix is command-scoped, so its bindings evaporate and the outgoing state is unchanged
    /// (`19H` adversarial case). `unset` clobbers: a stale literal surviving an `unset` is the
    /// same confidently-wrong class as the ⊤-region case (the shell passes empty/unset where
    /// the analysis would claim the old value).
    fn transfer_command(&self, id: CfgNodeId, incoming: &ValueEnv) -> ValueEnv {
        if let Some(env) = self.transfer_lvalue_builtin(id, incoming) {
            return env;
        }
        let Some(list) = self.assigns.get(&self.cfg.node(id).ast) else {
            return incoming.clone();
        };
        if list.is_empty() || self.has_command_word(id) {
            return incoming.clone();
        }
        let mut env = incoming.clone();
        apply_assigns(list, &mut env);
        env
    }

    /// The lvalue-mutating-builtin family: `unset`, `read`, `export`, `readonly`, `local`,
    /// `getopts` all mutate variables while being target-state-Pure at the effect layer (they
    /// touch no system state, so they rightly do not poison the ambient gate) — which is
    /// exactly why a stale concrete surviving one is the confidently-wrong no-floor class
    /// (round-20 crosscheck F-read/F-export: `PKG=nginx; read PKG; install "$PKG"` elided a
    /// runtime-determined install). Every clobber is to ⊤, never a modeled value — we degrade,
    /// we do not re-implement these builtins' semantics. Returns `None` for other commands.
    fn transfer_lvalue_builtin(&self, id: CfgNodeId, incoming: &ValueEnv) -> Option<ValueEnv> {
        let NodeKind::Simple { words, .. } = &self.ast.node(self.cfg.node(id).ast).kind else {
            return None;
        };
        let (&cmd_word, operands) = words.split_first()?;
        let cmd = literal_text(self.ast, cmd_word)?;
        let mut env = incoming.clone();
        let mut names: Vec<String> = Vec::new();
        let havoc_all = |env: &mut ValueEnv| {
            for v in &self.assigned_vars {
                env.insert(v.clone(), Flat::Top);
            }
        };
        match cmd.as_str() {
            // `unset [-fv] name…` / `read [-r] name…`: every literal non-flag operand is a
            // clobbered variable name. `-r` is read's one POSIX flag (value-irrelevant);
            // any OTHER flag or a dynamic operand ⇒ which var mutated is unknowable ⇒
            // havoc-all (sound, imprecise).
            "unset" | "read" => {
                for &w in operands {
                    match literal_text(self.ast, w) {
                        Some(t) if t == "-r" && cmd == "read" => {}
                        Some(t) if t.starts_with('-') => {
                            havoc_all(&mut env);
                            return Some(env);
                        }
                        Some(t) => names.push(t),
                        None => {
                            havoc_all(&mut env);
                            return Some(env);
                        }
                    }
                }
            }
            // `export NAME=v` / `readonly NAME=v` / `local NAME=v`: an operand WITH `=`
            // assigns (clobber the name — we do not model the value); a bare `NAME` operand
            // only marks/exports the existing binding (no value change in dash — leave it).
            // Dynamic operand ⇒ havoc-all.
            "export" | "readonly" | "local" => {
                for &w in operands {
                    match literal_text(self.ast, w) {
                        Some(t) if t.starts_with('-') => {
                            havoc_all(&mut env);
                            return Some(env);
                        }
                        Some(t) => {
                            if let Some((name, _)) = t.split_once('=') {
                                names.push(name.to_owned());
                            }
                        }
                        None => {
                            havoc_all(&mut env);
                            return Some(env);
                        }
                    }
                }
            }
            // `getopts optstring name [args…]`: clobbers `name` plus OPTIND/OPTARG, every
            // call. (Usually inside a ⊤-rejected loop anyway; this covers the bare form.)
            "getopts" => {
                names.push("OPTIND".to_owned());
                names.push("OPTARG".to_owned());
                match operands.get(1).and_then(|&w| literal_text(self.ast, w)) {
                    Some(t) if !t.starts_with('-') => names.push(t),
                    _ => {
                        havoc_all(&mut env);
                        return Some(env);
                    }
                }
            }
            _ => return None,
        }
        for n in names {
            env.insert(n, Flat::Top);
        }
        Some(env)
    }

    /// Does this command node have a command word (a real command, vs. assignment-only)?
    fn has_command_word(&self, id: CfgNodeId) -> bool {
        matches!(
            &self.ast.node(self.cfg.node(id).ast).kind,
            NodeKind::Simple { words, .. } if !words.is_empty()
        )
    }

    /// Resolve one command-site's argv. When `!converged`, the whole site is all-`⊤` (`16P`
    /// DP-9). Command-prefix assignments (`FOO=bar cmd "$FOO"`) are NOT visible to the
    /// command's own argv: POSIX §2.9.1 expands the non-assignment words FIRST (step 2) and
    /// performs the assignments after (step 4), so `"$FOO"` reads the *incoming* binding.
    /// (Round-20 crosscheck finding: the original transient application here resolved
    /// `pkg=nginx; pkg=apache apt-get install "$pkg"` to `apache` while dash passes `nginx` —
    /// a wrong concrete that licensed a wrong elision end-to-end. Argv resolves against
    /// `incoming` only; the prefix bindings affect the command's ENVIRONMENT, which we do not
    /// model, and correctly never persist downstream — see `transfer_command`.)
    fn site_argv(&self, id: CfgNodeId, states: &[ValueEnv], converged: bool) -> Vec<Abstract> {
        let NodeKind::Simple { words, .. } = &self.ast.node(self.cfg.node(id).ast).kind else {
            return Vec::new();
        };
        if !converged {
            return vec![Abstract::Top; words.len()];
        }
        let Some(incoming) = states.get(id.index()) else {
            return vec![Abstract::Top; words.len()];
        };
        words
            .iter()
            .map(|&w| resolve_recipe(&recipe_of_word(self.ast, w), incoming))
            .collect()
    }
}

/// The word's compile-time-constant text: `Some` iff every part is literal (no variable
/// references, no expansion of any kind). Used where a command's *shape* (e.g. `unset name`)
/// must be recognized statically, independent of any dataflow state. Delegates to
/// [`sem::const_literal_text`] — the single home of the "no-variables-at-all" rule.
fn literal_text(ast: &Ast, word: AstId) -> Option<String> {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return None;
    };
    sem::const_literal_text(parts)
}

/// Apply a node's assignments left-to-right, each RHS resolved against the running state.
fn apply_assigns(list: &[(String, Recipe)], env: &mut ValueEnv) {
    for (name, recipe) in list {
        let v = resolve_recipe(recipe, env);
        env.insert(name.clone(), flat_of(&v));
    }
}

/// [`Abstract`] ⇒ the per-variable `Flat`.
fn flat_of(v: &Abstract) -> Flat<String> {
    match v {
        Abstract::Lit(s) => Flat::Elem(s.clone()),
        Abstract::Top => Flat::Top,
    }
}

/// Resolve a [`Recipe`] against a state: concatenate its fragments; any `⊤` fragment, any
/// `⊤`-recipe, makes the whole word `⊤`. A concatenation of literals is the joined literal
/// (`19H`: `x=ng; y="${x}inx"` ⇒ `nginx` when the AST exposes the parts).
fn resolve_recipe(recipe: &Recipe, env: &ValueEnv) -> Abstract {
    let parts = match recipe {
        Recipe::Top => return Abstract::Top,
        Recipe::Parts(p) => p,
    };
    let mut buf = String::new();
    for frag in parts {
        match frag {
            Frag::Lit(s) => buf.push_str(s),
            Frag::Var(v) => match lookup(env, v) {
                Abstract::Lit(s) => buf.push_str(&s),
                Abstract::Top => return Abstract::Top, // ⊤ or absent-as-⊤ var ⇒ whole word ⊤
            },
        }
    }
    Abstract::Lit(buf)
}

/// Flatten an AST word into a [`Recipe`] via the shared quoting-class rules
/// ([`sem::classify_frag`]): a quoted plain variable is a trackable [`Frag::Var`]; a literal
/// is a [`Frag::Lit`]; and any ⊤-class fragment (a quoted positional/special/subst —
/// `FragClass::OpaqueValue` — or *any* unquoted expansion that may word-split —
/// `FragClass::SplitRisk`) collapses the whole word to [`Recipe::Top`]. The
/// arity/value-preservation split that was hand-rolled here now lives once in `sem`.
fn recipe_of_word(ast: &Ast, word: AstId) -> Recipe {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return Recipe::Top;
    };
    let mut frags = Vec::new();
    if collect_frags(parts, /* quoted = */ false, &mut frags) {
        Recipe::Parts(frags)
    } else {
        Recipe::Top
    }
}

/// Collect concatenation fragments from word-parts; returns `false` (⇒ whole word `⊤`) on the
/// first part that is not value-preserving as a tracked fragment. `quoted` tracks whether we
/// are inside a double-quote. The per-part decision is [`sem::classify_frag`]; this only maps
/// its [`FragClass`] onto the analysis's owned-text [`Frag`].
fn collect_frags(parts: &[WordPart], quoted: bool, out: &mut Vec<Frag>) -> bool {
    for part in parts {
        // Non-DoubleQuoted parts classify directly; a DoubleQuoted recurses at quoted=true.
        let WordPart::DoubleQuoted(inner) = part else {
            match sem::classify_frag(part, quoted) {
                Some(FragClass::Literal(s)) => out.push(Frag::Lit(s.to_owned())),
                Some(FragClass::Var(name)) => out.push(Frag::Var(name.to_owned())),
                // ⊤ classes collapse the word. `None` is only `DoubleQuoted` (handled
                // above); a defensive ⊤ otherwise.
                Some(FragClass::OpaqueValue | FragClass::SplitRisk) | None => return false,
            }
            continue;
        };
        if !collect_frags(inner, true, out) {
            return false;
        }
    }
    true
}

/// Compute, per `ScopeExit` node, the set of variable names assigned anywhere inside the
/// matching scope (`an-leaf-scope` containment). Scope boundaries nest by construction
/// (`cfg::build` allocates `ScopeEnter` before its body before `ScopeExit`), so the matching
/// enter for an exit is the nearest preceding still-open enter *by node index* — a
/// bracket-match over the arena order. Pure + deterministic.
fn compute_scope_clobbers(cfg: &Cfg, ast: &Ast) -> BTreeMap<usize, BTreeSet<String>> {
    // The names assigned by each node (Command nodes only bind names; others bind none).
    let assigned_at = |node_ast: AstId| -> Vec<String> {
        match &ast.node(node_ast).kind {
            NodeKind::Simple { assigns, .. } => assigns
                .iter()
                .filter_map(|&a| match &ast.node(a).kind {
                    NodeKind::Assign { name, .. } => Some(name.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    };

    // Walk nodes in arena (index) order, maintaining a stack of open ScopeEnter indices.
    // Each assignment is attributed to every currently-open scope (an inner assign clobbers
    // all enclosing subshell exits it sits within). On ScopeExit, pop and record.
    let nodes: Vec<(CfgNodeId, CfgNodeKind, AstId)> =
        cfg.iter().map(|(id, n)| (id, n.kind, n.ast)).collect();
    let mut open: Vec<BTreeSet<String>> = Vec::new();
    let mut out: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
    for (id, kind, node_ast) in nodes {
        match kind {
            CfgNodeKind::ScopeEnter => open.push(BTreeSet::new()),
            CfgNodeKind::ScopeExit => {
                let inside = open.pop().unwrap_or_default();
                out.insert(id.index(), inside);
            }
            CfgNodeKind::Command => {
                for name in assigned_at(node_ast) {
                    for frame in &mut open {
                        frame.insert(name.clone());
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::build;

    /// One resolved word, lowered to a tiny comparison-friendly value for ergonomic asserts
    /// (`Lit(text)` = literal, `Top` = ⊤). The analysis derives every value end-to-end from
    /// parsed sh — no test constructs an [`Abstract`]/`ValueOf` by fiat
    /// (`inv-probe-sourced-values`, the anti-masking discipline). A bespoke enum (not
    /// `Option<String>`) keeps clippy's `unnecessary_wraps` quiet on the `lit` helper.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Word {
        Lit(String),
        Top,
    }

    /// Parse `src`, build its CFG, run the analysis, and return the resolved argv of the
    /// FIRST `Command` whose command-word literal is `cmd`. Resolves `Symbol`s back to text
    /// through the same interner `analyze` used.
    fn argv_of(src: &str, cmd: &str) -> Vec<Word> {
        let parsed = dorc_syntax::parse(src);
        let cfg = build(&parsed.value).value;
        let mut interner = Interner::default();
        let flow = analyze(&cfg, &parsed.value, &mut interner);
        let node = command_node(&cfg, &parsed.value, cmd)
            .unwrap_or_else(|| panic!("no command `{cmd}` in {src:?}"));
        flow.argv_values(node)
            .into_iter()
            .map(|v| word_of(v, &interner))
            .collect()
    }

    /// Resolve one [`ValueOf`] to the test-side [`Word`] through the analysis's interner.
    fn word_of(v: ValueOf, interner: &Interner) -> Word {
        match v {
            ValueOf::Literal(s) => Word::Lit(interner.resolve(s).to_owned()),
            ValueOf::Top => Word::Top,
        }
    }

    /// `argv_of` plus the convergence flag (for the loop test).
    fn argv_and_converged(src: &str, cmd: &str) -> (Vec<Word>, bool) {
        let parsed = dorc_syntax::parse(src);
        let cfg = build(&parsed.value).value;
        let mut interner = Interner::default();
        let flow = analyze(&cfg, &parsed.value, &mut interner);
        let node = command_node(&cfg, &parsed.value, cmd).unwrap_or_else(|| panic!("no `{cmd}`"));
        let words = flow
            .argv_values(node)
            .into_iter()
            .map(|v| word_of(v, &interner))
            .collect();
        (words, flow.converged())
    }

    /// The first `Command` CFG node whose source `Simple`'s command word is exactly `cmd`.
    fn command_node(cfg: &Cfg, ast: &Ast, cmd: &str) -> Option<CfgNodeId> {
        cfg.iter()
            .filter(|(_, n)| n.kind == CfgNodeKind::Command)
            .find(|(_, n)| command_word(ast, n.ast).as_deref() == Some(cmd))
            .map(|(id, _)| id)
    }

    /// The command-word literal of a `Simple`, if statically fixed (mirrors the helpers in
    /// `cfg.rs`/`effect.rs`). Used only to *locate* a node — never to feed the analysis.
    fn command_word(ast: &Ast, id: AstId) -> Option<String> {
        let NodeKind::Simple { words, .. } = &ast.node(id).kind else {
            return None;
        };
        let first = words.first()?;
        match &ast.node(*first).kind {
            NodeKind::Word { parts } => match parts.as_slice() {
                [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    /// Shorthand: a literal-text expected word.
    fn lit(s: &str) -> Word {
        Word::Lit(s.to_owned())
    }

    // ---- the 19H §1.4 floor case + quoting forms -----------------------------------------

    #[test]
    fn const_prop_through_quoted_variable() {
        // The 19H §1.4 floor: `pkg=nginx; apt-get install -y "$pkg"` ⇒ every word literal,
        // the `"$pkg"` resolving to `nginx`. This is the headline reason the analysis exists.
        assert_eq!(
            argv_of(r#"pkg=nginx; apt-get install -y "$pkg""#, "apt-get"),
            vec![lit("apt-get"), lit("install"), lit("-y"), lit("nginx")]
        );
    }

    #[test]
    fn bare_unquoted_variable_resolves_when_value_cannot_split() {
        // `$pkg` UNQUOTED: in shell this may word-split, so its arity is not statically one
        // argument — we conservatively ⊤ it (`Word::may_split`), even though `nginx` happens
        // not to contain IFS/glob chars (the analyzer cannot know that statically). This is a
        // deliberate precision floor, asserted so the choice is visible.
        assert_eq!(
            argv_of(r"pkg=nginx; apt-get install -y $pkg", "apt-get"),
            vec![lit("apt-get"), lit("install"), lit("-y"), Word::Top],
            "unquoted $pkg ⇒ ⊤ (may word-split); the quoted form is the literal one"
        );
    }

    #[test]
    fn reassignment_last_write_wins() {
        // Flow-sensitivity: `pkg=nginx; pkg=apache; cmd "$pkg"` ⇒ apache. A non-flow-sensitive
        // analysis would join the two and yield ⊤; the worklist must keep the later def.
        assert_eq!(
            argv_of(r#"pkg=nginx; pkg=apache; cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), lit("apache")]
        );
    }

    #[test]
    fn variable_to_variable_copy() {
        // `a=nginx; b=$a; cmd "$b"` ⇒ nginx: a copy chains the literal. (`b=$a` unquoted RHS
        // of an assignment does NOT word-split — assignment RHS is not subject to splitting —
        // but our `recipe_of_word` treats the assignment value's top level as unquoted; assert
        // the conservative outcome we actually produce.)
        // NB: `b=$a` — the RHS `$a` is a bare Param at top level ⇒ our collector ⊤s it
        // (unquoted may-split rule). So `cmd "$b"` is ⊤. This pins the (conservative) reality.
        assert_eq!(
            argv_of(r#"a=nginx; b=$a; cmd "$b""#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "bare $a on an assignment RHS is conservatively ⊤ (may-split rule applies uniformly)"
        );
        // The quoted-RHS form preserves it:
        assert_eq!(
            argv_of(r#"a=nginx; b="$a"; cmd "$b""#, "cmd"),
            vec![lit("cmd"), lit("nginx")],
            "quoted RHS `b=\"$a\"` copies the literal"
        );
    }

    // ---- branch divergence (the join) ----------------------------------------------------

    #[test]
    fn branch_divergence_joins_to_top() {
        // `if c; then pkg=a; else pkg=b; fi; cmd "$pkg"` ⇒ ⊤: two different literals join to ⊤
        // at the merge. The pointwise lattice join is doing the work.
        assert_eq!(
            argv_of(r#"if c; then pkg=a; else pkg=b; fi; cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn branch_agreement_keeps_literal() {
        // Control for the join: if BOTH branches assign the SAME literal, it survives the join
        // (`Elem(a) ⊔ Elem(a) = Elem(a)`). Proves the ⊤ above is the disagreement, not the
        // branch itself.
        assert_eq!(
            argv_of(r#"if c; then pkg=a; else pkg=a; fi; cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), lit("a")]
        );
    }

    #[test]
    fn half_assigned_branch_joins_to_top() {
        // The half-assigned variant (assigned in the then-branch only): the else-path leaves
        // `pkg` at its (unset ⇒ ⊤) entry value, so the join is `Elem(a) ⊔ ⊤ = ⊤`. This is the
        // case the entry-⊤-seed exists to get right (without it, MapL's absent≡⊥ join would
        // wrongly yield `a`).
        assert_eq!(
            argv_of(r#"if c; then pkg=a; fi; cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn assignment_before_branch_then_one_arm_reassigns() {
        // `pkg=base; if c; then pkg=new; fi; cmd "$pkg"` ⇒ ⊤: the then-branch may overwrite
        // `base` with `new`, the else keeps `base` — distinct literals join to ⊤. Confirms the
        // pre-branch binding does not spuriously survive a divergent reassignment.
        assert_eq!(
            argv_of(r#"pkg=base; if c; then pkg=new; fi; cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    // ---- subshell containment ------------------------------------------------------------

    #[test]
    fn subshell_assignment_does_not_leak() {
        // `(pkg=evil); cmd "$pkg"` ⇒ NOT evil. A subshell's var mutations don't escape
        // (`an-leaf-scope`); our sound fallback ⊤s anything assigned inside at the ScopeExit.
        // The wrong-elision-relevant direction: assert it is never the inner value.
        let argv = argv_of(r#"(pkg=evil); cmd "$pkg""#, "cmd");
        assert_eq!(argv, vec![lit("cmd"), Word::Top]);
        assert_ne!(
            argv[1],
            lit("evil"),
            "the inner subshell value must NOT leak out"
        );
    }

    #[test]
    fn outer_binding_survives_unrelated_subshell() {
        // Precision check on the scope fallback: a subshell that does NOT assign `pkg` must
        // not clobber it. `pkg=nginx; (echo hi); cmd "$pkg"` ⇒ nginx still. (Proves the
        // clobber set is per-assigned-name, not ⊤-everything-at-every-exit.)
        assert_eq!(
            argv_of(r#"pkg=nginx; (echo hi); cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), lit("nginx")]
        );
    }

    #[test]
    fn nested_subshell_inner_assignment_does_not_leak() {
        // Deeply-nested scopes: `a=outer; ( ( a=inner ) ); cmd "$a"` ⇒ ⊤ (the inner assign
        // clobbers `a` at BOTH enclosing exits). Exercises the bracket-matched clobber stack.
        let argv = argv_of(r#"a=outer; ( ( a=inner ) ); cmd "$a""#, "cmd");
        assert_eq!(argv, vec![lit("cmd"), Word::Top]);
        assert_ne!(argv[1], lit("inner"));
        assert_ne!(
            argv[1],
            lit("outer"),
            "conservative: clobbered to ⊤, not restored"
        );
    }

    // ---- command substitution / arithmetic / operator-expansion RHS ----------------------

    #[test]
    fn command_substitution_rhs_is_top() {
        // `x=$(whatever); cmd "$x"` ⇒ ⊤: a command-substitution result is runtime-dynamic.
        assert_eq!(
            argv_of(r#"x=$(whatever); cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn mixed_literal_and_command_subst_is_top() {
        // Adversarial concatenation: `x=a$(b)c; cmd "$x"` ⇒ ⊤. One ⊤ fragment poisons the whole
        // word even though `a` and `c` are literal.
        assert_eq!(
            argv_of(r#"x=a$(b)c; cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn arithmetic_rhs_is_top() {
        // `x=$((1+1)); cmd "$x"` ⇒ ⊤: arithmetic expansion is a ⊤-trigger (dynamic).
        assert_eq!(
            argv_of(r#"x=$((1+1)); cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn operator_param_expansion_rhs_is_top() {
        // `x=${z:-default}; cmd "$x"` ⇒ ⊤: a parameter-expansion-with-operators is kept opaque
        // (we do NOT model the `:-` default, even though it has a literal fallback).
        assert_eq!(
            argv_of(r#"x=${z:-default}; cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    // ---- concatenation the AST DOES expose -----------------------------------------------

    #[test]
    fn brace_param_concatenation_resolves() {
        // `x=ng; y="${x}inx"; cmd "$y"` ⇒ nginx. `${x}` (name-only) parses as a plain Param,
        // so inside the double-quote it concatenates with the literal `inx`. We implement the
        // PRECISE outcome (the literal `nginx`), and assert it — the AST exposes the parts, so
        // ⊤ would be needlessly conservative here.
        assert_eq!(
            argv_of(r#"x=ng; y="${x}inx"; cmd "$y""#, "cmd"),
            vec![lit("cmd"), lit("nginx")]
        );
    }

    #[test]
    fn literal_concatenation_with_quoted_var() {
        // `pfx=lib; cmd "${pfx}c"` directly in argv ⇒ `libc`. Concatenation of a resolved var
        // and a trailing literal, no intermediate assignment.
        assert_eq!(
            argv_of(r#"pfx=lib; cmd "${pfx}c""#, "cmd"),
            vec![lit("cmd"), lit("libc")]
        );
    }

    // ---- partial-⊤ argv (per-word independence) ------------------------------------------

    #[test]
    fn partial_top_argv_is_per_word() {
        // `apt-get install -y "$dyn"` (dyn never assigned ⇒ unset ⇒ ⊤) ⇒
        // [apt-get, install, -y, ⊤]. The per-word independence is the whole point: literal
        // neighbours stay literal; only the dynamic word is ⊤ (202 §1 — the consumer, not us,
        // decides what a partial-⊤ argv means).
        assert_eq!(
            argv_of(r#"apt-get install -y "$dyn""#, "apt-get"),
            vec![lit("apt-get"), lit("install"), lit("-y"), Word::Top]
        );
    }

    // ---- positional / special parameters -------------------------------------------------

    #[test]
    fn positional_and_at_params_are_top() {
        // `cmd "$@" "$1"` at book top-level ⇒ [cmd, ⊤, ⊤]: script args are runtime input
        // (19H), always ⊤, regardless of quoting.
        assert_eq!(
            argv_of(r#"cmd "$@" "$1""#, "cmd"),
            vec![lit("cmd"), Word::Top, Word::Top]
        );
    }

    #[test]
    fn assignment_from_positional_is_top() {
        // `x=$1; cmd "$x"` ⇒ ⊤: a value derived from a positional parameter is runtime input.
        // (Also: bare `$1` on an RHS is special-param ⇒ ⊤ regardless of the may-split rule.)
        assert_eq!(
            argv_of(r#"x=$1; cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    // ---- the unset / absent cases --------------------------------------------------------

    #[test]
    fn assignment_from_unset_variable_is_top() {
        // `x=$y` where `y` is never assigned ⇒ ⊤ (`cmd "$x"` ⇒ ⊤). Absent-is-⊤: an unset var
        // is not a known constant. (`$y` is also unquoted-may-split ⇒ ⊤ either way; the
        // quoted form below isolates the absent-is-⊤ rule.)
        assert_eq!(
            argv_of(r#"x=$y; cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
        // Quoted RHS isolates absent-is-⊤ from the may-split rule: `x="$y"` with y unset ⇒ ⊤.
        assert_eq!(
            argv_of(r#"x="$y"; cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn word_that_is_only_a_top_var() {
        // A word that is ONLY a ⊤-var (no literal frags): `cmd "$undefined"` ⇒ [cmd, ⊤].
        assert_eq!(
            argv_of(r#"cmd "$undefined""#, "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn unset_clears_to_top() {
        // `pkg=nginx; unset pkg; cmd "$pkg"` ⇒ ⊤ (19H: `unset name` clobbers). A stale
        // `nginx` here would be a confidently-wrong propagation — the shell passes
        // empty/unset where the analysis claims the old literal (the no-floor class,
        // 19H §1.3); the orchestrator review closed this gap.
        assert_eq!(
            argv_of(r#"pkg=nginx; unset pkg; cmd "$pkg""#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "`unset pkg` clobbers the tracked literal to ⊤"
        );
    }

    #[test]
    fn read_clobbers_target_to_top() {
        // F-read (round-20 neutral crosscheck, demonstrated end-to-end): `read PKG`
        // overwrites PKG with runtime stdin; a surviving static `nginx` elided a
        // runtime-determined install. The lvalue family clobbers to ⊤.
        assert_eq!(
            argv_of("PKG=nginx\nread PKG\ncmd \"$PKG\"", "cmd"),
            vec![lit("cmd"), Word::Top],
            "`read PKG` clobbers the tracked literal (runtime stdin wins)"
        );
        // `-r` is read's value-irrelevant POSIX flag; the clobber still applies.
        assert_eq!(
            argv_of("PKG=nginx\nread -r PKG\ncmd \"$PKG\"", "cmd"),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn export_and_readonly_with_assignment_clobber_to_top() {
        // F-export/F-readonly: `export PKG=nginx` is command-word `export` + an
        // operand `PKG=nginx` — not a bare assignment, not a prefix — so the old
        // transfer carried the stale prior binding (`apache`) past it; dash installs
        // nginx. We clobber the assigned name to ⊤ (degrade, never model the value).
        assert_eq!(
            argv_of("PKG=apache\nexport PKG=nginx\ncmd \"$PKG\"", "cmd"),
            vec![lit("cmd"), Word::Top],
            "`export NAME=v` clobbers NAME (the engine does not model the new value)"
        );
        assert_eq!(
            argv_of("PKG=apache\nreadonly PKG=nginx\ncmd \"$PKG\"", "cmd"),
            vec![lit("cmd"), Word::Top]
        );
        // The bare no-`=` form only exports the EXISTING binding — dash does not
        // change the value, so the tracked literal legitimately survives.
        assert_eq!(
            argv_of("PKG=apache\nexport PKG\ncmd \"$PKG\"", "cmd"),
            vec![lit("cmd"), lit("apache")],
            "`export NAME` (no `=`) changes no value; the binding survives"
        );
    }

    #[test]
    fn unset_with_flag_or_dynamic_operand_havocs_all() {
        // `unset -v "$which"` (or any flagged/dynamic operand): WHICH var died is unknowable
        // ⇒ havoc every tracked var (sound, imprecise — the conservative floor).
        assert_eq!(
            argv_of("a=1\nb=2\nunset -v a\ncmd \"$b\"", "cmd"),
            vec![lit("cmd"), Word::Top],
            "a flagged unset havocs all tracked vars (cannot prove which died)"
        );
    }

    #[test]
    fn top_region_havocs_preassigned_vars() {
        // gap-3 (crosscheck-surfaced): a STILL-⊤-rejected region's UNPARSED body may
        // reassign anything. A no-`in` `for` (iterates runtime "$@") is still ⊤ post-L1:
        // `pkg=nginx; for x; do pkg=evil; done; cmd "$pkg"` — without the Top-node havoc
        // the stale `nginx` survives the ⊤ region (the `DP-8` half-modeling / no-floor
        // class, 19H §1.3). The havoc forces ⊤. (A LITERAL-list loop is NOT ⊤ now — its
        // visible body + back-edge join gives the same ⊤ for a body reassignment, pinned
        // by `for_loop_body_reassignment_converges_via_back_edge`; this test guards the
        // residual ⊤-region path that survives L1.)
        assert_eq!(
            argv_of("pkg=nginx\nfor x; do pkg=evil; done\ncmd \"$pkg\"", "cmd"),
            vec![lit("cmd"), Word::Top],
            "a pre-assigned var must not survive a ⊤-rejected region with its old literal"
        );
        // The havoc is total: every tracked var goes ⊤ past the opaque region, not only
        // ones it provably touches (it proves nothing — it is unparsed).
        assert_eq!(
            argv_of("a=1\nb=2\neval x\ncmd \"$a\" \"$b\"", "cmd"),
            vec![lit("cmd"), Word::Top, Word::Top],
            "every tracked var is ⊤ past a ⊤-rejected `eval`"
        );
    }

    // ---- command-scoped env prefix (FOO=bar cmd) -----------------------------------------

    #[test]
    fn command_prefix_assignment_not_visible_in_own_argv() {
        // POSIX §2.9.1: argv words expand BEFORE prefix assignments take effect, so
        // `FOO=bar cmd "$FOO"` passes FOO's *prior* binding (here: none ⇒ ⊤-conservative;
        // dash passes empty). The original transient-visibility behavior here was a wrong
        // concrete (round-20 adversarial crosscheck, priority-1: `pkg=nginx; pkg=apache
        // apt-get install "$pkg"` resolved `apache`, dash passes `nginx`, and the wrong
        // entity licensed a wrong elision end-to-end).
        assert_eq!(
            argv_of(r#"FOO=bar cmd "$FOO""#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "a prefix binding must not be read by the same command's own argv"
        );
        // The concrete-vs-concrete disagreement case: the PRIOR binding wins.
        assert_eq!(
            argv_of("pkg=nginx\npkg=apache cmd \"$pkg\"", "cmd"),
            vec![lit("cmd"), lit("nginx")],
            "argv reads the incoming env: dash expands words before the prefix assignment"
        );
    }

    #[test]
    fn command_prefix_assignment_does_not_persist() {
        // The containment direction: `FOO=bar cmd; other "$FOO"` ⇒ `other`'s `$FOO` is ⊤. A
        // command-scoped env prefix does NOT persist to later commands (it is not a shell
        // variable assignment). This is the `19H` adversarial case; the binding must not leak.
        let argv = argv_of(r#"FOO=bar cmd; other "$FOO""#, "other");
        assert_eq!(argv, vec![lit("other"), Word::Top]);
        assert_ne!(
            argv[1],
            lit("bar"),
            "command-prefix env must not persist past the command"
        );
    }

    #[test]
    fn bare_assignment_does_persist() {
        // Contrast with the prefix case: a BARE assignment (no command word) DOES persist.
        // `FOO=bar; other "$FOO"` ⇒ bar. Proves the persist/no-persist split is the
        // has-command-word distinction, not assignment-shape.
        assert_eq!(
            argv_of(r#"FOO=bar; other "$FOO""#, "other"),
            vec![lit("other"), lit("bar")]
        );
    }

    // ---- loops: the FIRST real cyclic CFG the worklist sees (task-L1, `209` brk-1) -------

    #[test]
    fn for_var_single_literal_word_resolves_precisely() {
        // The for-variable binds the JOIN of the literal list words at body entry. ONE
        // word ⇒ that literal (Flat precision): `for f in nginx; do cmd "$f"; done` ⇒ the
        // in-body `cmd` sees `f = nginx`. This is the precise end of brk-1 (a) before the
        // Powerset slice; the in-loop floor still RUNS the leaf (plan), but the value plane
        // resolves it (so its identity/entity is known).
        assert_eq!(
            argv_of(r#"for f in nginx; do cmd "$f"; done"#, "cmd"),
            vec![lit("cmd"), lit("nginx")],
            "single-word for-list ⇒ the loop var resolves to that literal"
        );
    }

    #[test]
    fn for_var_multiple_words_joins_to_top() {
        // ≥2 distinct list words ⇒ the Flat join saturates ⇒ the for-var is ⊤ (the
        // Powerset of {a,b} is the deferred member-elision precision, `209` brk-1 (b)).
        assert_eq!(
            argv_of(r#"for f in a b; do cmd "$f"; done"#, "cmd"),
            vec![lit("cmd"), Word::Top],
            ">1 distinct for-list words ⇒ loop var ⊤ (Flat join saturates)"
        );
        // Two IDENTICAL words ⇒ still that literal (the join of Elem(a)⊔Elem(a)=Elem(a)).
        assert_eq!(
            argv_of(r#"for f in a a; do cmd "$f"; done"#, "cmd"),
            vec![lit("cmd"), lit("a")],
            "repeated identical word ⇒ the literal survives the join"
        );
    }

    #[test]
    fn for_loop_body_reassignment_converges_via_back_edge() {
        // THE first real cycle: a body reassignment flows back through the back-edge to
        // the loop head and JOINs there. `pkg=base; for f in a b; do pkg=$f...` — but the
        // simplest pin: a body var reassigned to the (⊤) loop var, observed AFTER the loop,
        // is ⊤, AND the solve converges (the worklist's back-edge reaches a fixed point).
        // This is the property task-A's note flagged as untestable until a real loop existed.
        let (argv, converged) = argv_and_converged(
            r#"pkg=base; for f in a b; do pkg="$f"; done; cmd "$pkg""#,
            "cmd",
        );
        assert!(converged, "the worklist converges on the real cyclic CFG");
        assert_eq!(
            argv,
            vec![lit("cmd"), Word::Top],
            "pkg joins base (0 iters) with $f (⊤, ≥1 iter) across the back-edge ⇒ ⊤"
        );
    }

    #[test]
    fn while_loop_body_var_is_top_after_loop() {
        // A `while` head binds no loop var; a body assignment joins with the pre-loop value
        // across the back-edge. `pkg=base` before, `pkg=x` in the body ⇒ join(base, x) = ⊤
        // after the loop (ran-0-times keeps base; ran-≥1 sets x). Converges.
        let (argv, converged) =
            argv_and_converged("pkg=base\nwhile c; do pkg=x; done\ncmd \"$pkg\"", "cmd");
        assert!(converged, "the while back-edge converges");
        assert_eq!(argv, vec![lit("cmd"), Word::Top]);
    }

    #[test]
    fn nested_loop_book_converges() {
        // item-4(c) convergence smoke: a NESTED loop is two back-edges feeding one another;
        // the monotone worklist over the finite-height Flat domain must still reach a fixed
        // point (`solve`'s converged flag true). The values saturate to ⊤ (multi-word lists),
        // but the load-bearing property here is *termination on nested cycles*.
        let (argv, converged) = argv_and_converged(
            "for o in a b; do for i in 1 2; do x=\"$o$i\"; done; done\ncmd \"$x\"",
            "cmd",
        );
        assert!(
            converged,
            "the solve converges on a NESTED-loop CFG (two back-edges)"
        );
        assert_eq!(argv, vec![lit("cmd"), Word::Top]);
    }

    #[test]
    fn post_loop_var_unaffected_by_pure_loop_keeps_literal() {
        // Precision check (the brk-1 value-unlock's value-plane half): a var set BEFORE a
        // loop whose body does NOT touch it survives the loop with its literal. `pkg=nginx;
        // for f in a b; do echo "$f"; done; cmd "$pkg"` ⇒ pkg still nginx (the loop's
        // back-edge join carries pkg=nginx unchanged; only `f` and echo's args move).
        assert_eq!(
            argv_of(
                r#"pkg=nginx; for f in a b; do echo "$f"; done; cmd "$pkg""#,
                "cmd"
            ),
            vec![lit("cmd"), lit("nginx")],
            "a pre-loop var untouched by the body survives the loop with its literal"
        );
    }

    // ---- determinism + totality ----------------------------------------------------------

    #[test]
    fn analysis_is_deterministic() {
        // `inv-determinism`: identical input ⇒ identical resolved argv (same Symbol order, so
        // resolved text matches). Run twice, compare.
        let src = r#"pkg=nginx; apt-get install -y "$pkg""#;
        assert_eq!(argv_of(src, "apt-get"), argv_of(src, "apt-get"));
    }

    #[test]
    fn total_on_hostile_sources() {
        // `inv-no-throw`: analyze must not panic on hostile/⊤-laden input. Mirrors the cfg
        // totality battery; we only assert it returns (and converges — all these are
        // finite-height).
        let hostile = [
            "",
            "$(((",
            "eval \"$x\"",
            "unset \"$dyn\"",
            "for i in 1 2 3; do x=$i; done",
            "x=$(y=$(z=deep)); cmd \"$x\"",
            "( ( ( ( a=1 ) ) ) )",
            "FOO=$BAR BAZ=$QUX cmd \"$FOO\" \"$BAZ\"",
            "a=1 b=2 c=$(cmd) echo hi",
            "if x; then a=1; elif y; then a=2; else a=3; fi; cmd \"$a\"",
        ];
        for src in hostile {
            let parsed = dorc_syntax::parse(src);
            let cfg = build(&parsed.value).value;
            let mut interner = Interner::default();
            let flow = analyze(&cfg, &parsed.value, &mut interner);
            assert!(flow.converged(), "finite-height ⇒ converges on {src:?}");
        }
    }

    #[test]
    fn elif_chain_joins_to_top() {
        // A three-way `if/elif/else` with distinct literals joins to ⊤ at the merge — the
        // join generalises past two branches.
        assert_eq!(
            argv_of(
                r#"if x; then a=1; elif y; then a=2; else a=3; fi; cmd "$a""#,
                "cmd"
            ),
            vec![lit("cmd"), Word::Top]
        );
    }

    #[test]
    fn single_arm_case_default_keeps_unmatched_fallthrough_top() {
        // `case` join: `pkg=base; case $h in foo) pkg=a ;; esac; cmd "$pkg"` ⇒ ⊤. The
        // no-match fall-through path keeps `base`, the matched arm sets `a` ⇒ join ⊤.
        assert_eq!(
            argv_of(
                r#"pkg=base; case $h in foo) pkg=a ;; esac; cmd "$pkg""#,
                "cmd"
            ),
            vec![lit("cmd"), Word::Top]
        );
    }
}
