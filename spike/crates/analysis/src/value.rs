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
    /// Per in-loop body `Command` node: the PER-MEMBER resolved argvs (task-L2 item-1/2,
    /// `209` brk-1(b)). Present ONLY for a site inside a `for`-loop whose iteration var is
    /// **Members**-bound (every list word a single concrete, dups kept) and whose argv
    /// references that var — the one place the Members value is read (it never flows
    /// through the general transfer, item-1). Outer `Vec` = the members in list order
    /// (duplicates kept — dash iterates them); inner `Vec` = that member's argv (the
    /// for-var substituted to that one concrete, so each is a normal concrete argv). A
    /// site whose argv has a non-member `⊤` word, or whose loop is Members-ineligible,
    /// is ABSENT here ⇒ the consumer falls back to the (⊤) [`argv`] entry.
    member_argv: BTreeMap<CfgNodeId, Vec<Vec<ValueOf>>>,
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

    /// The PER-MEMBER resolved argvs of an in-loop Members site (task-L2 item-1/2), or
    /// `None` if this site is not a Members site (its loop is ineligible, or its argv does
    /// not reference the for-var, or any member word fails to resolve concretely). Each
    /// inner argv is a normal concrete argv (the for-var substituted to one member); the
    /// consumer evaluates the check once per member ⇒ a fact-family (item-2). The order is
    /// the list order with duplicates kept (dash iterates them; dedup would mis-model
    /// `for x in a a`). See [`ValueFlow::member_argv`].
    #[must_use]
    pub fn member_argv(&self, node: CfgNodeId) -> Option<&Vec<Vec<ValueOf>>> {
        self.member_argv.get(&node)
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

    // task-L2 item-1/2: the per-member argvs for in-loop Members sites — a SEPARATE pass
    // off the same converged solution (the Members value never rode the lattice, item-1).
    let member_argv = prep.members_pass(&solution.states, solution.converged, interner);

    ValueFlow {
        argv,
        member_argv,
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
    /// Whether the book NEVER touches `IFS` (no assignment, prefix-env, `unset`, or
    /// lvalue-builtin write of `IFS` anywhere). This is the brk-3 field-splitting
    /// precondition (`209` brk-3): a known-literal unquoted `$PKGS` splits on default IFS
    /// only when IFS is provably pristine; ANY touch ⇒ every unquoted-split word degrades to
    /// `⊤` (we cannot model splitting under an unknown IFS). Computed once as a book-wide
    /// pre-pass ([`scan_ifs_pristine`]).
    ifs_pristine: bool,
}

/// A flattened recipe for one word's value: the ordered fragments to concatenate. Any
/// fragment the analysis cannot turn into a literal makes the whole word `⊤` (`19H`: a word
/// containing a `⊤`-var or an unmodeled expansion is `⊤`).
#[derive(Debug, Clone)]
enum Recipe {
    /// Unconditionally `⊤` (held an unmodeled/dynamic part, an unquoted positional/special,
    /// or an unquoted command-substitution/arithmetic): no point tracking fragments.
    Top,
    /// Concatenate these fragments left-to-right; if any resolves to `⊤`, the word is `⊤`.
    /// If a [`Frag::SplitVar`] is present the word may field-split (`209` brk-3) — see
    /// [`resolve_recipe_fields`].
    Parts(Vec<Frag>),
}

/// One fragment of a [`Recipe`].
#[derive(Debug, Clone)]
enum Frag {
    /// Literal text (verbatim; contributes to a single field, never a split boundary).
    Lit(String),
    /// A *quoted* plain-variable reference (`"$x"`), resolved against the per-point state.
    /// Value-preserving: it does not field-split.
    Var(String),
    /// An *unquoted* plain-variable reference (`$x`), resolved against the per-point state
    /// and then **field-split** under default IFS (`209` brk-3, XCU §2.6.5). Modelable only
    /// when IFS is book-pristine and the resulting fields are glob-free; otherwise the word
    /// degrades to `⊤` (`resolve_recipe`/`site_argv`).
    SplitVar(String),
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
                    // A word-leading unquoted `~` on the RHS tilde-expands (`x=~` ⇒ `$HOME`),
                    // which we cannot reproduce ⇒ ⊤ (fix-1). A source-literal glob does NOT
                    // expand on an assignment RHS (`x=*.txt` stores `*.txt`), so it is kept
                    // concrete here — the glob hazard fires only at the unquoted USE site.
                    Some(v) if word_assign_rhs_hazard(ast, *v) => Recipe::Top,
                    Some(v) => recipe_of_word(ast, *v),
                };
                list.push((name.clone(), recipe));
            }
            assigns.insert(node.ast, list);
        }

        let scope_exit_clobbers = compute_scope_clobbers(cfg, ast);
        let ifs_pristine = scan_ifs_pristine(ast);

        Prep {
            cfg,
            ast,
            assigned_vars,
            scope_exit_clobbers,
            assigns,
            ifs_pristine,
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
        // distinct literals to ⊤ — exactly the >1-element rule. A `for`-list word is an
        // expansion site (dash globs `for f in *.conf`, tilde-expands `for f in ~`), so an
        // unquoted source-literal glob / word-leading `~` (fix-1) makes that word ⊤.
        let mut acc = Flat::Bottom;
        for &w in words {
            let resolved = if word_expansion_hazard(self.ast, w) {
                Abstract::Top
            } else {
                resolve_recipe(&recipe_of_word(self.ast, w), incoming)
            };
            acc = acc.join(&flat_of(&resolved));
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
    ///
    /// Each word resolves to one OR MORE argv slots (`209` brk-3): an unquoted known-literal
    /// var field-splits in place under default IFS (`$PKGS` ⇒ `[nginx, curl]`), and an
    /// empty-value split word contributes ZERO slots (field elision — `cmd $EMPTY x` ⇒
    /// `[cmd, x]`, dash-verified). A non-splitting word is exactly one slot, preserving the
    /// per-word independence the consumer relies on (`202 §1`). See [`resolve_recipe_fields`].
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
        self.resolve_site_words(words, incoming)
    }

    /// Resolve a `Simple`'s `words` against a given environment to the flattened argv
    /// slots (the shared core of [`site_argv`] and the per-member [`members_pass`]). Each
    /// word expands to ≥0 slots per the glob/tilde-hazard + field-split rules.
    fn resolve_site_words(&self, words: &[AstId], env: &ValueEnv) -> Vec<Abstract> {
        words
            .iter()
            .flat_map(|&w| {
                // An unquoted source-literal glob / word-leading `~` (fix-1) expands against
                // the live filesystem / `$HOME` ⇒ one ⊤ slot (we cannot enumerate it). This
                // is the direct-literal channel (`cmd *.deb`); a glob arriving through an
                // unquoted variable's VALUE is the split path's `field_is_modelable` concern.
                if word_expansion_hazard(self.ast, w) {
                    return vec![Abstract::Top];
                }
                resolve_recipe_fields(&recipe_of_word(self.ast, w), env, self.ifs_pristine)
            })
            .collect()
    }

    /// Compute the per-member argvs for every in-loop **Members** site (task-L2 item-1/2,
    /// `209` brk-1(b)). A SEPARATE pass off the converged solution — the Members value
    /// never rode the dataflow lattice (item-1), so this reads each eligible `for`-loop's
    /// list off the AST and substitutes each concrete member into the body sites' argv.
    ///
    /// Eligibility (item-1, STRICT — bias every ambiguity to ineligible ⇒ the existing ⊤):
    /// * the body contains NO nested loop (this slice is single-level only; nested-loop
    ///   member interactions are the deferred multi-leaf case, item-3 "stay floored"); and
    /// * every list WORD resolves to a single concrete literal (post-F1: no glob/tilde; a
    ///   split-literal list IS fine — it composes to more members); and
    /// * the for-var is NOT reassigned anywhere inside the body (an assignment, an
    ///   lvalue-builtin naming it, a nested binder, or an unmodeled ⊤-region) — any such
    ///   reassignment ⇒ the Members binding is invalid ⇒ omit the loop (the consumer falls
    ///   back to the Flat-⊤ `argv`, the existing degrade).
    ///
    /// A non-converged solve yields no Members sites (the all-⊤ fold, `16P` DP-9). Only a
    /// body site whose argv actually REFERENCES the for-var (its per-member argvs differ)
    /// is recorded — a body site that ignores the var has no member-family (it is the same
    /// concrete every iteration; the ordinary `argv` entry already serves it).
    fn members_pass(
        &self,
        states: &[ValueEnv],
        converged: bool,
        interner: &mut Interner,
    ) -> BTreeMap<CfgNodeId, Vec<Vec<ValueOf>>> {
        let mut out = BTreeMap::new();
        if !converged {
            return out;
        }
        for (head_id, node) in self.cfg.iter() {
            if node.kind != CfgNodeKind::LoopHead {
                continue;
            }
            // Single-level only this slice (item-3): a `for` head that is ITSELF inside an
            // enclosing loop is the deferred multi-leaf/nested case ⇒ refuse (its body
            // sites' members interact with the outer iteration). With `body_has_nested_loop`
            // below, BOTH directions of a nested pair are refused.
            if self.cfg.in_loop_body(head_id) {
                continue;
            }
            let NodeKind::ForLoop {
                var, words, body, ..
            } = &self.ast.node(node.ast).kind
            else {
                continue; // while/until head: no loop var ⇒ never Members
            };
            let Some(members) = self.eligible_members(words, var, *body, states, head_id) else {
                continue;
            };
            self.record_member_sites(*body, var, &members, states, interner, &mut out);
        }
        out
    }

    /// The eligible Members list for a `for`-loop, or `None` if ineligible (item-1). The
    /// list is resolved against the loop head's INCOMING state (the same state
    /// `transfer_loop_head` joins), so a split-literal list composes correctly; duplicates
    /// are KEPT (dash iterates them — dedup would mis-count `for x in a a`).
    fn eligible_members(
        &self,
        words: &[AstId],
        var: &str,
        body: AstId,
        states: &[ValueEnv],
        head_id: CfgNodeId,
    ) -> Option<Vec<String>> {
        if words.is_empty() {
            return None; // empty list ⇒ 0 iterations ⇒ no members (the ⊤ degrade)
        }
        // Single-level only this slice (item-3): a nested loop inside the body is the
        // deferred multi-leaf case ⇒ ineligible.
        if self.body_has_nested_loop(body) {
            return None;
        }
        // The for-var rebinding INVALIDATES Members if the body reassigns it — the
        // Members value is the head binding only (item-1). Conservative: any write to the
        // name inside the body subtree (assignment, lvalue-builtin, or a ⊤-region that
        // havocs everything) ⇒ ineligible.
        if self.body_reassigns_var(body, var) {
            return None;
        }
        let incoming = states.get(head_id.index())?;
        let mut members = Vec::with_capacity(words.len());
        for &w in words {
            // A for-list word is an expansion site (glob/tilde) — fix-1 ⇒ ineligible.
            if word_expansion_hazard(self.ast, w) {
                return None;
            }
            // Each word must resolve to single concretes (split-to-many composes into more
            // members; a ⊤ slot ⇒ ineligible). `resolve_recipe_fields` gives the field
            // slots in list order.
            let recipe = recipe_of_word(self.ast, w);
            for field in resolve_recipe_fields(&recipe, incoming, self.ifs_pristine) {
                match field {
                    Abstract::Lit(s) => members.push(s),
                    Abstract::Top => return None,
                }
            }
        }
        if members.is_empty() {
            return None;
        }
        Some(members)
    }

    /// Record the per-member argvs for each body command-site of a Members loop whose argv
    /// REFERENCES the for-var. For each member, clone the site's incoming state, override
    /// the for-var to that one concrete, and resolve the site's words — each is a normal
    /// concrete argv (item-2: N members ⇒ N argvs ⇒ N cells). A site whose argv does NOT
    /// reference the for-var is skipped (no family — the ordinary `argv` entry serves it,
    /// the same concrete every iteration). A site that references it gets a family even for
    /// a single member (one cell), so the in-loop license (item-3) routes uniformly through
    /// the Members path rather than the Flat `EstablishAmbient` (which the in-loop floor
    /// still runs).
    fn record_member_sites(
        &self,
        body: AstId,
        var: &str,
        members: &[String],
        states: &[ValueEnv],
        interner: &mut Interner,
        out: &mut BTreeMap<CfgNodeId, Vec<Vec<ValueOf>>>,
    ) {
        for (site_id, site) in self.cfg.iter() {
            if site.kind != CfgNodeKind::Command || !self.cfg.in_loop_body(site_id) {
                continue;
            }
            // Only THIS loop's body sites (span-contained), and never an expansion-internal
            // non-leaf (`$(…)` body). With no nested loop allowed (eligibility), every such
            // site belongs to this loop alone.
            if self.cfg.is_expansion_internal(site_id) || !node_within(self.ast, site.ast, body) {
                continue;
            }
            let NodeKind::Simple { words, .. } = &self.ast.node(site.ast).kind else {
                continue;
            };
            // A site that does not reference the for-var is the same concrete every
            // iteration — no family (the `argv` entry serves it).
            if !words.iter().any(|&w| word_references_var(self.ast, w, var)) {
                continue;
            }
            let Some(incoming) = states.get(site_id.index()) else {
                continue;
            };
            let per_member: Vec<Vec<ValueOf>> = members
                .iter()
                .map(|m| {
                    let mut env = incoming.clone();
                    env.insert(var.to_owned(), Flat::Elem(m.clone()));
                    intern_argv(self.resolve_site_words(words, &env), interner)
                })
                .collect();
            out.insert(site_id, per_member);
        }
    }

    /// Does `body`'s AST subtree contain a nested `for`/`while`/`until` loop? (item-1's
    /// single-level restriction.) Span-contained scan; cheap (corpus bodies are tiny).
    fn body_has_nested_loop(&self, body: AstId) -> bool {
        self.ast.iter().any(|(id, n)| {
            id != body
                && node_within(self.ast, id, body)
                && matches!(
                    n.kind,
                    NodeKind::ForLoop { .. } | NodeKind::WhileLoop { .. }
                )
        })
    }

    /// Does `body`'s AST subtree reassign `var`? (item-1's degrade trigger.) Any assignment
    /// to the name, an lvalue-builtin (`read`/`unset`/`export`/`readonly`/`local`/
    /// `getopts`) naming it, or a ⊤ (unmodeled) region inside the body ⇒ `true`
    /// (conservative: a ⊤-region havocs everything, so it could rebind the var). Pure scan.
    fn body_reassigns_var(&self, body: AstId, var: &str) -> bool {
        for (id, n) in self.ast.iter() {
            if id == body || !node_within(self.ast, id, body) {
                continue;
            }
            match &n.kind {
                NodeKind::Assign { name, .. } if name == var => return true,
                NodeKind::Simple { words, .. } if self.simple_writes_var(words, var) => {
                    return true;
                }
                // A ⊤ (unsupported) region inside the body may rebind anything.
                NodeKind::Unsupported { .. } => return true,
                _ => {}
            }
        }
        false
    }

    /// Does this `Simple`'s command word make it an lvalue-builtin writing `var`? Mirrors
    /// the [`Prep::transfer_lvalue_builtin`] family (`read`/`unset`/`export`/`readonly`/
    /// `local`/`getopts`): a bare-name or `name=…` operand matching `var`, or ANY dynamic/
    /// flagged operand to such a builtin (which havocs — conservative ⇒ treat as a write).
    fn simple_writes_var(&self, words: &[AstId], var: &str) -> bool {
        let Some((&cmd_word, operands)) = words.split_first() else {
            return false;
        };
        let Some(cmd) = literal_text(self.ast, cmd_word) else {
            return false;
        };
        match cmd.as_str() {
            "read" | "unset" | "export" | "readonly" | "local" => operands.iter().any(|&op| {
                match literal_text(self.ast, op) {
                    // `name` or `name=…` naming the for-var ⇒ a write.
                    Some(t) => t == var || t.strip_prefix(var).is_some_and(|r| r.starts_with('=')),
                    // A dynamic operand ⇒ which var it writes is unknown ⇒ conservative write.
                    None => true,
                }
            }),
            // `getopts optstring name …` writes `name` (operand 1) plus, always,
            // OPTIND/OPTARG; a dynamic `name` operand may write anything ⇒ conservative.
            "getopts" => {
                var == "OPTIND"
                    || var == "OPTARG"
                    || match operands.get(1) {
                        Some(&w) => literal_text(self.ast, w).is_none_or(|name| name == var),
                        None => false,
                    }
            }
            // `cd` rebinds `$PWD`/`$OLDPWD` (POSIX `cd` step 10). It stays blessed-pure
            // on the fact-cell axis (establishes nothing), but it IS a var-writer: the
            // L2-crosscheck's find-cd-pwd drove `for PWD in …; do cd /tmp; install
            // "$PWD"` to a wrong elision through exactly this gap (20T).
            "cd" => var == "PWD" || var == "OLDPWD",
            _ => false,
        }
    }
}

/// Is node `inner` within node `outer`'s subtree, by span containment? The AST's spans
/// nest by construction (a child's `[lo,hi)` lies within its parent's), so a byte-range
/// containment test is a sound subtree-membership check. `inner == outer` counts as
/// within. Used by the Members pass to scope a loop's body sites (task-L2 item-1/2).
fn node_within(ast: &Ast, inner: AstId, outer: AstId) -> bool {
    let i = ast.node(inner).span;
    let o = ast.node(outer).span;
    o.lo.0 <= i.lo.0 && i.hi.0 <= o.hi.0
}

/// Does a `Word` reference the named variable `var` (`$var`/`${var}`/`"$var"`)? A
/// plain-`Param`-named reference, quoted or not, recursing into double-quotes (task-L2
/// item-1/2: which body sites form a per-member family). Positional/special params
/// (`$1`/`$@`) never name a for-var, so they don't match.
fn word_references_var(ast: &Ast, word: AstId, var: &str) -> bool {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return false;
    };
    parts_reference_var(parts, var)
}

/// Recurse word-parts for a plain `Param { name == var }` reference (double-quotes nest).
fn parts_reference_var(parts: &[WordPart], var: &str) -> bool {
    parts.iter().any(|p| match p {
        WordPart::Param { name } => name == var,
        WordPart::DoubleQuoted(inner) => parts_reference_var(inner, var),
        _ => false,
    })
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

/// Book-wide pre-pass: is `IFS` provably never touched? (`209` brk-3 splitting
/// precondition.) Field-splitting of an unquoted known literal is modelable only under the
/// DEFAULT IFS; ANY book-side write to `IFS` makes the separator set unknown, so EVERY
/// unquoted-split word must then degrade to `⊤`. We over-refuse deliberately — a single
/// mention as an lvalue anywhere (even in dead/⊤-rejected code) flips the whole book — which
/// is the safe direction (`inv-kfail`: a wrong split is a wrong argv ⇒ a wrong entity ⇒ a
/// wrong elision). Pure over the AST.
///
/// What counts as a touch (each dash-confirmed to change IFS):
/// * an [`Assign`](NodeKind::Assign) named `IFS` — covers `IFS=…` standalone, the
///   command-prefix `IFS=… cmd` (a prefix assignment IS an `Assign` node), and the
///   assignment carried by `export IFS=…` parsed as an assign;
/// * an lvalue-builtin (`unset`/`export`/`readonly`/`local`/`read`) whose operand is `IFS`
///   or `IFS=…` — `read IFS` reads runtime stdin into IFS, the others set/unset it. (The
///   `read`-with-`IFS=`-prefix case is the `Assign` arm above; `getopts` writes only
///   `OPTIND`/`OPTARG`/its name, never IFS, so it is irrelevant — prompt-confirmed.)
fn scan_ifs_pristine(ast: &Ast) -> bool {
    const IFS: &str = "IFS";
    const LVALUE_BUILTINS: [&str; 5] = ["unset", "export", "readonly", "local", "read"];
    for (_, node) in ast.iter() {
        match &node.kind {
            NodeKind::Assign { name, .. } if name == IFS => return false,
            NodeKind::Simple { words, .. } => {
                let Some((&cmd_word, operands)) = words.split_first() else {
                    continue;
                };
                let Some(cmd) = literal_text(ast, cmd_word) else {
                    continue;
                };
                if !LVALUE_BUILTINS.contains(&cmd.as_str()) {
                    continue;
                }
                for &op in operands {
                    if literal_text(ast, op).is_some_and(|t| t == IFS || t.starts_with("IFS=")) {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    true
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

/// Resolve a [`Recipe`] to a SINGLE [`Abstract`] against a state — the NON-argv contexts
/// (assignment RHS, `for`-list words): concatenate fragments; any `⊤` fragment, any
/// `⊤`-recipe, makes the whole word `⊤`. A concatenation of literals is the joined literal
/// (`19H`: `x=ng; y="${x}inx"` ⇒ `nginx` when the AST exposes the parts).
///
/// Field-splitting is deliberately NOT applied here: an assignment RHS does not field-split
/// (`x=$y` assigns the whole value, dash-verified), and `for`-list member splitting is the
/// deferred brk-1 precision slice — so an unquoted [`Frag::SplitVar`] degrades to `⊤` in
/// these contexts, exactly as it did before brk-3 (the existing conservative behavior, e.g.
/// `b=$a` ⇒ ⊤). Only [`site_argv`] (via [`resolve_recipe_fields`]) splits.
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
            // An unquoted var outside argv position is ⊤ here (no split applied — see doc).
            Frag::SplitVar(_) => return Abstract::Top,
        }
    }
    Abstract::Lit(buf)
}

/// Resolve a [`Recipe`] in **argv position** to the field list it expands to (`209` brk-3,
/// XCU §2.6.5): the ONE context where an unquoted [`Frag::SplitVar`] field-splits. Returns
/// the resolved argv slots (one `Abstract` per field) — usually one, but N for a split word
/// (`$PKGS` ⇒ `[nginx, curl]`) and ZERO for an empty-value word (`$EMPTY` ⇒ elided from
/// argv, dash-verified).
///
/// `ifs_pristine` is the book-wide precondition (no `IFS` touch anywhere — see
/// [`Prep::ifs_pristine`]): when `false`, every split-bearing word degrades to a single `⊤`
/// (splitting under an unknown IFS is unmodelable). A word with NO split fragment resolves
/// exactly as [`resolve_recipe`] (one slot, preserving the empty-string-is-one-slot
/// behavior); a `⊤`/absent split-var value, or a resulting glob-bearing field, degrades the
/// whole word to a single `⊤`.
/// A resolved-to-text word fragment owning its string, so the borrowed [`sem::Field`]s
/// built from it (which borrow `&str`) outlive the call into `sem`. `Split` marks an
/// unquoted-var value subject to field-splitting; `Literal` marks non-splitting text.
enum OwnedField {
    Literal(String),
    Split(String),
}

fn resolve_recipe_fields(recipe: &Recipe, env: &ValueEnv, ifs_pristine: bool) -> Vec<Abstract> {
    let parts = match recipe {
        Recipe::Top => return vec![Abstract::Top],
        Recipe::Parts(p) => p,
    };
    // No unquoted split fragment ⇒ this word's arity is statically one; resolve it exactly
    // as the single-value path (an empty literal stays one empty slot, never elided).
    if !parts.iter().any(|f| matches!(f, Frag::SplitVar(_))) {
        return vec![resolve_recipe(recipe, env)];
    }
    // A split-bearing word under a non-pristine IFS is unmodelable ⇒ one ⊤ slot.
    if !ifs_pristine {
        return vec![Abstract::Top];
    }
    // Resolve each fragment to OWNED text tagged splittable-or-not. Any ⊤/absent value ⇒
    // the whole word is one ⊤ slot (we cannot split an unknown value). The owned buffer
    // outlives the borrowed `sem::Field`s built from it just below.
    let mut owned = Vec::with_capacity(parts.len());
    for frag in parts {
        let resolved = match frag {
            Frag::Lit(s) => OwnedField::Literal(s.clone()),
            Frag::Var(v) | Frag::SplitVar(v) => match lookup(env, v) {
                Abstract::Lit(s) if matches!(frag, Frag::SplitVar(_)) => OwnedField::Split(s),
                Abstract::Lit(s) => OwnedField::Literal(s),
                Abstract::Top => return vec![Abstract::Top],
            },
        };
        owned.push(resolved);
    }
    let fields: Vec<sem::Field<'_>> = owned
        .iter()
        .map(|f| match f {
            OwnedField::Literal(s) => sem::Field::Literal(s),
            OwnedField::Split(s) => sem::Field::Split(s),
        })
        .collect();
    // `split_fields_join` returns `None` when any resulting field carries a glob char
    // (pathname expansion against the remote fs ⇒ unmodelable ⇒ ⊤).
    match sem::split_fields_join(&fields) {
        Some(fs) => fs.into_iter().map(Abstract::Lit).collect(),
        None => vec![Abstract::Top],
    }
}

/// Flatten an AST word into a [`Recipe`] via the shared quoting-class rules
/// ([`sem::classify_frag`]): a quoted plain variable is a trackable [`Frag::Var`]; a literal
/// is a [`Frag::Lit`]; an *unquoted* plain variable is a [`Frag::SplitVar`] (it may
/// field-split, `209` brk-3); and any other ⊤-class fragment (a quoted positional/special/
/// subst — `FragClass::OpaqueValue` — or an unquoted positional/special/subst/arithmetic)
/// collapses the whole word to [`Recipe::Top`]. The arity/value-preservation split that was
/// hand-rolled here lives once in `sem`; the field-split refinement of the unquoted-var case
/// is applied here (it needs the resolved *value*, which `sem`'s quoting-classifier cannot
/// hold).
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

/// Collect fragments from word-parts; returns `false` (⇒ whole word `⊤`) on the first part
/// that is not value-preserving *and* not field-split-modelable. `quoted` tracks whether we
/// are inside a double-quote. The per-part decision is [`sem::classify_frag`]; the one
/// refinement this adds is the `209` brk-3 split case: an UNQUOTED plain `$name` is kept as a
/// [`Frag::SplitVar`] (resolve-then-split) instead of collapsing the word — but an unquoted
/// positional/special/command-subst/arithmetic still collapses (its value is not a known
/// literal, so there is nothing to split).
fn collect_frags(parts: &[WordPart], quoted: bool, out: &mut Vec<Frag>) -> bool {
    for part in parts {
        // Non-DoubleQuoted parts classify directly; a DoubleQuoted recurses at quoted=true.
        let WordPart::DoubleQuoted(inner) = part else {
            match sem::classify_frag(part, quoted) {
                Some(FragClass::Literal(s)) => out.push(Frag::Lit(s.to_owned())),
                Some(FragClass::Var(name)) => out.push(Frag::Var(name.to_owned())),
                // An unquoted expansion (`SplitRisk`): a plain `$name` is split-modelable
                // (resolve its literal, then field-split); anything else (an unquoted
                // positional/special, or a command-subst/arithmetic/operator-form) has no
                // known literal value ⇒ collapse the word.
                Some(FragClass::SplitRisk) => match split_var_name(part) {
                    Some(name) => out.push(Frag::SplitVar(name.to_owned())),
                    None => return false,
                },
                // A quoted positional/special/subst (`OpaqueValue`) is arity-safe but ⊤.
                // `None` is only `DoubleQuoted` (handled above); defensive ⊤ otherwise.
                Some(FragClass::OpaqueValue) | None => return false,
            }
            continue;
        };
        if !collect_frags(inner, true, out) {
            return false;
        }
    }
    true
}

/// The unquoted word-expansion hazards a word triggers at a *command/`for`-list expansion*
/// site (`20O` find-1, fix-1): an unquoted source-literal glob (`*.deb`, XCU §2.6.6) OR a
/// word-leading unquoted `~` (XCU §2.6.1). Either makes the word's expansion runtime-
/// dependent (filesystem / `$HOME`) and unreproducible ⇒ the word degrades to a single ⊤.
/// Both predicates live in `sem` (sharing `GLOB_CHARS` with the split-result guard); this
/// is their value-plane application at the two *expansion* sites only.
fn word_expansion_hazard(ast: &Ast, word: AstId) -> bool {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return false;
    };
    sem::word_has_unquoted_glob(parts) || sem::word_has_leading_tilde(parts)
}

/// The subset of [`word_expansion_hazard`] that fires at an *assignment-RHS* site (`20O`
/// fix-1, the three-row table): a word-leading unquoted `~` only. dash expands a tilde-prefix
/// on an assignment RHS (`x=~` ⇒ `$HOME`) — unreproducible ⇒ ⊤ — but does **not** glob it
/// (`x=*.txt` stores the literal `*.txt` concretely), so the source-literal glob check is
/// deliberately excluded here. The store/unquoted-use/quoted-use distinction then falls out:
/// the literal is stored concrete; an unquoted *use* (`cmd $x`) globs the value via the split
/// path's [`sem::field_is_modelable`]; a quoted use (`cmd "$x"`) stays concrete.
fn word_assign_rhs_hazard(ast: &Ast, word: AstId) -> bool {
    let NodeKind::Word { parts } = &ast.node(word).kind else {
        return false;
    };
    sem::word_has_leading_tilde(parts)
}

/// If `part` is an unquoted plain-variable expansion (`$name` / `${name}` where `name` is a
/// POSIX *name*), return that name — the one unquoted expansion whose split is modelable
/// (`209` brk-3). A positional/special parameter (`$1`/`$@`) returns [`None`]: its value is
/// runtime input, so there is no known literal to split.
fn split_var_name(part: &WordPart) -> Option<&str> {
    match part {
        WordPart::Param { name } if matches!(sem::classify_param(name), sem::ParamClass::Name) => {
            Some(name)
        }
        _ => None,
    }
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

    /// The per-member argvs (task-L2 item-1/2) of the FIRST in-loop `Command` whose
    /// command-word literal is `cmd`, or `None` if it is not a Members site. Each inner
    /// `Vec<Word>` is one member's resolved argv. Resolves through the analysis's interner.
    fn member_argv_of(src: &str, cmd: &str) -> Option<Vec<Vec<Word>>> {
        let parsed = dorc_syntax::parse(src);
        let cfg = build(&parsed.value).value;
        let mut interner = Interner::default();
        let flow = analyze(&cfg, &parsed.value, &mut interner);
        let node = command_node(&cfg, &parsed.value, cmd)
            .unwrap_or_else(|| panic!("no command `{cmd}` in {src:?}"));
        flow.member_argv(node).map(|members| {
            members
                .iter()
                .map(|argv| argv.iter().map(|&v| word_of(v, &interner)).collect())
                .collect()
        })
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
    fn bare_unquoted_single_value_var_splits_to_one_field() {
        // brk-3 (`209`): an unquoted known-literal var field-splits in place. A single-field
        // value (`pkg=nginx`) splits to exactly ONE field ⇒ the slot resolves to `nginx`
        // (matching dash: `$pkg` with pkg=nginx expands to one word). Before brk-3 this was a
        // conservative ⊤ (the may-split floor); the split now lifts it precisely. The quoted
        // form below resolves identically — they agree when the value has no IFS char.
        assert_eq!(
            argv_of(r"pkg=nginx; apt-get install -y $pkg", "apt-get"),
            vec![lit("apt-get"), lit("install"), lit("-y"), lit("nginx")],
            "unquoted $pkg (single-field value) splits to one field ⇒ resolves to nginx"
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

    // ---- brk-3 deliberate word-splitting (`209` brk-3, XCU §2.6.5) -----------------------

    #[test]
    fn unquoted_multi_value_var_splits_to_multiple_argv_slots() {
        // THE brk-3 headline: `PKGS="nginx curl"; apt-get install -y $PKGS` (unquoted,
        // intentional) ⇒ the `$PKGS` word expands IN PLACE to TWO argv slots — matching
        // dash's `[apt-get, install, -y, nginx, curl]` exactly, instead of the pre-brk-3 ⊤.
        assert_eq!(
            argv_of(r#"PKGS="nginx curl"; apt-get install -y $PKGS"#, "apt-get"),
            vec![
                lit("apt-get"),
                lit("install"),
                lit("-y"),
                lit("nginx"),
                lit("curl")
            ],
            "unquoted $PKGS splits into separate argv slots in place"
        );
    }

    #[test]
    fn quoted_multi_value_var_stays_one_slot() {
        // The quoting contrast (existing behavior, preserved): `"$PKGS"` is ONE argv slot
        // holding the whole value — double-quotes suppress field-splitting (dash: one word).
        assert_eq!(
            argv_of(
                r#"PKGS="nginx curl"; apt-get install -y "$PKGS""#,
                "apt-get"
            ),
            vec![lit("apt-get"), lit("install"), lit("-y"), lit("nginx curl")],
            "quoted \"$PKGS\" stays a single (multi-word-valued) slot"
        );
    }

    #[test]
    fn empty_value_unquoted_var_elides_from_argv() {
        // Field elision: `EMPTY=""; cmd $EMPTY x` ⇒ the `$EMPTY` word contributes ZERO argv
        // slots ⇒ `[cmd, x]` (dash-verified). The empty word disappears entirely — it is NOT
        // a `⊤` slot and NOT an empty-string slot.
        assert_eq!(
            argv_of(r#"EMPTY=""; cmd $EMPTY x"#, "cmd"),
            vec![lit("cmd"), lit("x")],
            "an empty unquoted var elides from argv (zero fields)"
        );
        // Contrast: QUOTED empty is a real (empty-string) slot, NOT elided.
        assert_eq!(
            argv_of(r#"EMPTY=""; cmd "$EMPTY" x"#, "cmd"),
            vec![lit("cmd"), lit(""), lit("x")],
            "quoted empty stays one empty-string slot"
        );
    }

    #[test]
    fn unquoted_var_collapsing_whitespace_splits_precisely() {
        // Leading/trailing/repeated separators collapse: `V="  a   b  "; cmd $V` ⇒ [cmd,a,b].
        assert_eq!(
            argv_of(r#"V="  a   b  "; cmd $V"#, "cmd"),
            vec![lit("cmd"), lit("a"), lit("b")],
            "surrounding/repeated IFS whitespace collapses; no empty fields"
        );
    }

    #[test]
    fn mixed_word_literal_prefix_and_unquoted_var_splits_per_posix() {
        // Mixed word (the precise §2.6.5 field-boundary join, dash-verified): a literal
        // prefix joins the FIRST split field; the internal separator breaks. `PKGS="nginx
        // curl"; cmd pre$PKGS` ⇒ [cmd, prenginx, curl].
        assert_eq!(
            argv_of(r#"PKGS="nginx curl"; cmd pre$PKGS"#, "cmd"),
            vec![lit("cmd"), lit("prenginx"), lit("curl")],
            "literal prefix joins the first split field"
        );
        // Trailing literal joins the LAST split field: `cmd $PKGS.deb` ⇒ [cmd, nginx, curl.deb].
        assert_eq!(
            argv_of(r#"PKGS="nginx curl"; cmd $PKGS.deb"#, "cmd"),
            vec![lit("cmd"), lit("nginx"), lit("curl.deb")],
            "trailing literal joins the last split field"
        );
        // A single-field value in a mixed word ⇒ pure concatenation, one slot (no split).
        assert_eq!(
            argv_of(r#"P="nginx"; cmd pre$P.deb"#, "cmd"),
            vec![lit("cmd"), lit("prenginx.deb")],
            "single-field value ⇒ mixed word is one concatenated slot"
        );
    }

    #[test]
    fn quoted_var_then_unquoted_var_splits_at_the_unquoted_boundary_only() {
        // The cross-quoting boundary (dash-verified): `A="x y"; B="p q"; cmd "$A"$B` ⇒
        // [cmd, "x yp", "q"]. The QUOTED `"$A"` is one literal field (its internal space
        // does NOT split); the UNQUOTED `$B`'s first split field joins it (`x y`+`p`), then
        // the internal separator breaks. Proves a quoted-resolved fragment is non-splitting
        // text even when its value contains IFS, while an adjacent unquoted var splits.
        assert_eq!(
            argv_of(r#"A="x y"; B="p q"; cmd "$A"$B"#, "cmd"),
            vec![lit("cmd"), lit("x yp"), lit("q")],
            "quoted-var value is non-splitting text; only the unquoted var splits"
        );
    }

    #[test]
    fn ifs_touched_book_degrades_every_unquoted_split_to_top() {
        // The IFS-pristine precondition: ANY book-side IFS write makes the separator set
        // unknown ⇒ every unquoted-split word degrades to ⊤ (we cannot model the split).
        // `IFS=,; PKGS="nginx curl"; apt-get install -y $PKGS` ⇒ the `$PKGS` slot is ⊤.
        assert_eq!(
            argv_of(
                r#"IFS=,; PKGS="nginx curl"; apt-get install -y $PKGS"#,
                "apt-get"
            ),
            vec![lit("apt-get"), lit("install"), lit("-y"), Word::Top],
            "an IFS assignment anywhere ⇒ unquoted split is unmodelable ⇒ ⊤"
        );
        // `unset IFS` is also a touch (IFS becomes the default, but we over-refuse — a
        // mention as an lvalue flips the book; the safe direction).
        assert_eq!(
            argv_of("unset IFS\nPKGS=nginx\napt-get install -y $PKGS", "apt-get"),
            vec![lit("apt-get"), lit("install"), lit("-y"), Word::Top],
            "`unset IFS` flips the book to non-pristine (conservative)"
        );
        // An IFS PREFIX-env on an unrelated command is a touch too (`IFS=: read x`).
        assert_eq!(
            argv_of("IFS=: read x\nPKGS=nginx\napt-get install $PKGS", "apt-get"),
            vec![lit("apt-get"), lit("install"), Word::Top],
            "an `IFS=…` command-prefix anywhere flips the book"
        );
    }

    #[test]
    fn touching_a_non_ifs_variable_keeps_splitting_modelable() {
        // Control for the IFS scan: touching some OTHER variable (here a plain `FOO=bar`
        // prefix, and an `unset OTHER`) does NOT flip the book — splitting still models.
        assert_eq!(
            argv_of("OTHER=x\nunset OTHER\nPKGS=\"a b\"\ncmd $PKGS", "cmd"),
            vec![lit("cmd"), lit("a"), lit("b")],
            "a non-IFS lvalue touch leaves IFS pristine ⇒ split still models"
        );
    }

    #[test]
    fn unquoted_var_with_glob_field_is_top() {
        // The wrong-concrete frontier: a split-result field bearing a glob char triggers
        // pathname expansion against the remote fs ⇒ unmodelable ⇒ the whole word is ⊤.
        // `PKGS="*.deb nginx"; cmd $PKGS` ⇒ the word is ⊤ (we cannot enumerate the glob).
        assert_eq!(
            argv_of(r#"PKGS="*.deb nginx"; cmd $PKGS"#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "a glob char in a split field ⇒ ⊤ (pathname expansion is runtime-dependent)"
        );
    }

    #[test]
    fn unquoted_var_with_tilde_field_is_literal_not_top() {
        // The other side of the frontier: a leading `~` in a SPLIT field is a LITERAL tilde
        // (dash does not tilde-expand split-result fields), so it resolves — never ⊤.
        // `PKGS="~ ~root"; cmd $PKGS` ⇒ [cmd, ~, ~root] (literal tildes).
        assert_eq!(
            argv_of(r#"PKGS="~ ~root"; cmd $PKGS"#, "cmd"),
            vec![lit("cmd"), lit("~"), lit("~root")],
            "split-result tilde fields are literal ⇒ resolve, not ⊤"
        );
    }

    #[test]
    fn unquoted_top_valued_var_stays_one_top_slot() {
        // A split-eligible var whose VALUE is ⊤ (here unset ⇒ absent-as-⊤) cannot be split —
        // it stays a single ⊤ slot (not zero, not many). `cmd $undef x` ⇒ [cmd, ⊤, x].
        assert_eq!(
            argv_of(r"cmd $undef x", "cmd"),
            vec![lit("cmd"), Word::Top, lit("x")],
            "an unresolved split-var value ⇒ one ⊤ slot (cannot split the unknown)"
        );
    }

    #[test]
    fn unquoted_positional_in_argv_is_top_not_split() {
        // A positional/special param is NOT a known literal, so even unquoted it does not
        // become a split-var — it stays a single ⊤ slot. `cmd $1 $@` ⇒ [cmd, ⊤, ⊤].
        assert_eq!(
            argv_of(r"cmd $1 $@", "cmd"),
            vec![lit("cmd"), Word::Top, Word::Top],
            "unquoted positional/special ⇒ ⊤ slot, never a modeled split"
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

    // ---- task-L2 item-1/2: Members-valued for-var + per-member argvs (`209` brk-1(b)) ---

    #[test]
    fn members_multi_word_loop_yields_per_member_argvs() {
        // THE member-precision unlock: `for pkg in nginx curl; do apt-get install -y "$pkg";
        // done`. The Flat `argv` stays ⊤ (everything-else-unchanged, item-1), but the SEPARATE
        // member channel resolves a per-member argv per list word, each a normal concrete.
        assert_eq!(
            argv_of(
                r#"for pkg in nginx curl; do apt-get install -y "$pkg"; done"#,
                "apt-get"
            ),
            vec![lit("apt-get"), lit("install"), lit("-y"), Word::Top],
            "the Flat argv stays ⊤ for a >1-member loop (Members rides a separate channel)"
        );
        assert_eq!(
            member_argv_of(
                r#"for pkg in nginx curl; do apt-get install -y "$pkg"; done"#,
                "apt-get"
            ),
            Some(vec![
                vec![lit("apt-get"), lit("install"), lit("-y"), lit("nginx")],
                vec![lit("apt-get"), lit("install"), lit("-y"), lit("curl")],
            ]),
            "each member substitutes to a concrete per-member argv (item-2: N members ⇒ N argvs)"
        );
    }

    #[test]
    fn members_keeps_duplicates_no_dedup() {
        // Dups are KEPT (dash iterates `for x in a a` twice) — dedup would mis-model the
        // iteration count. Two identical members ⇒ two identical per-member argvs.
        assert_eq!(
            member_argv_of(
                r#"for p in nginx nginx; do apt-get install "$p"; done"#,
                "apt-get"
            ),
            Some(vec![
                vec![lit("apt-get"), lit("install"), lit("nginx")],
                vec![lit("apt-get"), lit("install"), lit("nginx")],
            ]),
            "duplicate list words are kept as duplicate members (no dedup)"
        );
    }

    #[test]
    fn members_split_literal_list_composes() {
        // A split-literal list composes into more members (item-1: "a split literal list is
        // fine"). `LIST="nginx curl"; for p in $LIST` ⇒ the unquoted `$LIST` field-splits to
        // two members, exactly as if written `for p in nginx curl`.
        assert_eq!(
            member_argv_of(
                r#"LIST="nginx curl"; for p in $LIST; do apt-get install "$p"; done"#,
                "apt-get"
            ),
            Some(vec![
                vec![lit("apt-get"), lit("install"), lit("nginx")],
                vec![lit("apt-get"), lit("install"), lit("curl")],
            ]),
            "an unquoted split-literal for-list composes into per-member argvs"
        );
    }

    #[test]
    fn members_body_reassign_var_degrades_to_none() {
        // item-1 degrade: a body that REASSIGNS the for-var invalidates the Members binding
        // (the var no longer carries the head's member at the site) ⇒ NO member-family (the
        // consumer falls back to the Flat ⊤). `for p in a b; do p=evil; apt-get install
        // "$p"; done`.
        assert_eq!(
            member_argv_of(
                r#"for p in nginx curl; do p=evil; apt-get install "$p"; done"#,
                "apt-get"
            ),
            None,
            "a body reassignment of the for-var degrades Members to None (the ⊤ fallback)"
        );
    }

    #[test]
    fn members_for_pwd_with_cd_in_body_degrades_to_none() {
        // find-cd-pwd (L2-crosscheck, 20T): `cd` rebinds `$PWD`, so a `for PWD in …` body
        // containing `cd` does NOT carry the head's member at the install site — dash
        // installs `/tmp`, not `aaa`/`bbb`. Pre-fix this elided wrongly (kFAIL-perform
        // violation); the eligibility degrade is the fix surface.
        assert_eq!(
            member_argv_of(
                r#"for PWD in aaa bbb; do cd /tmp; apt-get install "$PWD"; done"#,
                "apt-get"
            ),
            None,
            "cd's implicit $PWD write makes a for-PWD loop member-ineligible"
        );
        // The same loop WITHOUT a cd keeps Members: dash iterates PWD=aaa, PWD=bbb
        // faithfully (PWD is an ordinary assignable var until something rebinds it).
        assert_eq!(
            member_argv_of(
                r#"for PWD in aaa bbb; do apt-get install "$PWD"; done"#,
                "apt-get"
            ),
            Some(vec![
                vec![lit("apt-get"), lit("install"), lit("aaa")],
                vec![lit("apt-get"), lit("install"), lit("bbb")],
            ]),
            "without cd, a for-PWD loop is an ordinary Members family"
        );
    }

    #[test]
    fn members_getopts_implicit_writes_degrade_to_none() {
        // Same class as find-cd-pwd: getopts ALWAYS writes OPTIND (and OPTARG on
        // flag-with-argument), regardless of its named operand.
        assert_eq!(
            member_argv_of(
                r#"for OPTIND in 1 2; do getopts ab f; apt-get install "$OPTIND"; done"#,
                "apt-get"
            ),
            None,
            "getopts' implicit OPTIND write makes a for-OPTIND loop member-ineligible"
        );
        // A dynamic name operand may write anything — conservative degrade (mirrors the
        // read-family's dynamic-operand arm).
        assert_eq!(
            member_argv_of(
                r#"for p in a b; do getopts ab "$dest"; apt-get install "$p"; done"#,
                "apt-get"
            ),
            None,
            "a dynamic getopts name operand conservatively counts as writing the for-var"
        );
    }

    #[test]
    fn members_body_read_clobbers_var_degrades_to_none() {
        // The lvalue-builtin degrade: `read pkg` inside the body overwrites the for-var with
        // runtime stdin ⇒ ineligible (item-1, same family as the value-plane's `read`
        // clobber). Bias-to-⊤.
        assert_eq!(
            member_argv_of(
                "for pkg in nginx curl; do read pkg; apt-get install \"$pkg\"; done",
                "apt-get"
            ),
            None,
            "`read <for-var>` in the body degrades Members to None"
        );
    }

    #[test]
    fn members_nested_loop_degrades_to_none() {
        // item-3 single-level restriction: a nested loop inside the body ⇒ the OUTER loop is
        // ineligible (multi-leaf member interactions are deferred). The inner install site is
        // not a Members site of the outer loop.
        assert_eq!(
            member_argv_of(
                "for p in a b; do for q in c d; do apt-get install \"$q\"; done; done",
                "apt-get"
            ),
            None,
            "a nested loop in the body ⇒ Members ineligible (single-level only this slice)"
        );
    }

    #[test]
    fn members_glob_list_word_degrades_to_none() {
        // fix-1 carry-through: an unquoted glob list word expands against the fs ⇒ ineligible
        // (the for-var would bind ⊤). No member-family.
        assert_eq!(
            member_argv_of(r#"for f in *.conf; do cmd "$f"; done"#, "cmd"),
            None,
            "a glob for-list word ⇒ Members ineligible (fix-1)"
        );
    }

    #[test]
    fn members_body_site_not_reading_var_has_no_family() {
        // A body site that does NOT read the for-var is the same concrete every iteration —
        // no member-family (the ordinary `argv` entry serves it). `for f in a b; do echo hi;
        // done` ⇒ `echo hi` is not a Members site.
        assert_eq!(
            member_argv_of(r"for f in nginx curl; do echo hi; done", "echo"),
            None,
            "a body site not referencing the for-var has no member-family"
        );
    }

    #[test]
    fn members_single_word_loop_has_no_family() {
        // A single-member loop's body site is already precisely resolved by the Flat `argv`
        // (the for-var binds that one literal). It is NOT a member-family (one member ⇒ the
        // ordinary single-concrete argv suffices; `member_argv` is for the >1 case). Pins
        // that we don't redundantly record a 1-element family.
        assert_eq!(
            argv_of(
                r#"for f in nginx; do apt-get install -y "$f"; done"#,
                "apt-get"
            ),
            vec![lit("apt-get"), lit("install"), lit("-y"), lit("nginx")],
            "single-word for ⇒ the Flat argv already resolves the body site precisely"
        );
        assert_eq!(
            member_argv_of(
                r#"for f in nginx; do apt-get install -y "$f"; done"#,
                "apt-get"
            ),
            Some(vec![vec![
                lit("apt-get"),
                lit("install"),
                lit("-y"),
                lit("nginx")
            ]]),
            "a single-member loop still yields a (1-element) family — all members substitute"
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

    // ---- fix-1: unquoted glob / tilde literals ⇒ ⊤ (`20O` find-1, XCU §2.6.6 / §2.6.1) ----

    #[test]
    fn straightline_argv_glob_literal_is_top() {
        // The priority-1 channel (pre-existing for straight-line argv): an unquoted literal
        // glob word expands against the live fs ⇒ ⊤ (dash: `*.deb` ⇒ the matching paths). The
        // literal NEIGHBOURS stay concrete (per-word independence) — only the glob word is ⊤.
        assert_eq!(
            argv_of(r"apt-get install -y *.deb", "apt-get"),
            vec![lit("apt-get"), lit("install"), lit("-y"), Word::Top],
            "unquoted literal glob `*.deb` ⇒ ⊤ (pathname expansion is runtime-dependent)"
        );
    }

    #[test]
    fn quoted_glob_literal_stays_concrete() {
        // The non-over-degrade pin (the engine is RIGHT here — do not break it): a QUOTED glob
        // is a dash-literal (`cmd "*.conf"` ⇒ `[*.conf]`), so it must resolve concrete.
        assert_eq!(
            argv_of(r#"install "*.conf""#, "install"),
            vec![lit("install"), lit("*.conf")],
            "double-quoted `\"*.conf\"` is a literal ⇒ concrete (no glob)"
        );
        assert_eq!(
            argv_of(r"install '*.conf'", "install"),
            vec![lit("install"), lit("*.conf")],
            "single-quoted `'*.conf'` is a literal ⇒ concrete (no glob)"
        );
    }

    #[test]
    fn word_leading_tilde_unquoted_is_top_quoted_is_concrete() {
        // Word-leading unquoted `~` tilde-expands to `$HOME` (dash-verified) ⇒ ⊤ (no $HOME
        // model). The quoted forms are dash-literals ⇒ concrete; a mid-word `~` is literal.
        assert_eq!(
            argv_of(r"cmd ~", "cmd"),
            vec![lit("cmd"), Word::Top],
            "word-leading unquoted `~` ⇒ ⊤ (tilde expansion, unmodelable)"
        );
        assert_eq!(
            argv_of(r"cmd '~'", "cmd"),
            vec![lit("cmd"), lit("~")],
            "single-quoted `'~'` is a dash-literal ⇒ concrete"
        );
        assert_eq!(
            argv_of(r#"cmd "~""#, "cmd"),
            vec![lit("cmd"), lit("~")],
            "double-quoted `\"~\"` is a dash-literal ⇒ concrete"
        );
        assert_eq!(
            argv_of(r"cmd x~", "cmd"),
            vec![lit("cmd"), lit("x~")],
            "a mid-word `~` (not word-leading) is a literal ⇒ concrete"
        );
    }

    #[test]
    fn assignment_rhs_glob_three_row_table() {
        // The dash-verified three-row table (the prompt's headline ask). The hazard is the
        // unquoted USE, NOT the store:
        //   store        — `x=*.txt` stores the literal `*.txt` CONCRETELY (no RHS glob);
        //   unquoted use  — `cmd $x` field-splits then globs the value ⇒ ⊤ (split path);
        //   quoted use    — `cmd "$x"` does not glob ⇒ stays concrete `*.txt`.
        assert_eq!(
            argv_of(r#"x=*.txt; cmd "$x""#, "cmd"),
            vec![lit("cmd"), lit("*.txt")],
            "store + quoted-use: assignment-RHS glob is stored concrete and survives a quoted use"
        );
        assert_eq!(
            argv_of(r"x=*.txt; cmd $x", "cmd"),
            vec![lit("cmd"), Word::Top],
            "unquoted-use: the stored glob value globs at the unquoted use ⇒ ⊤ (split path)"
        );
    }

    #[test]
    fn assignment_rhs_leading_tilde_is_top() {
        // Tilde DIVERGES from glob on an assignment RHS: dash expands `x=~` to `$HOME` (it is
        // an assignment-word tilde context, XCU §2.6.1), which we cannot reproduce ⇒ ⊤ even
        // for a quoted later use. The quoted RHS forms stay concrete (dash-literal `~`).
        assert_eq!(
            argv_of(r#"x=~; cmd "$x""#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "an unquoted word-leading `~` on the RHS expands at assignment ⇒ ⊤"
        );
        assert_eq!(
            argv_of(r#"x="~"; cmd "$x""#, "cmd"),
            vec![lit("cmd"), lit("~")],
            "a quoted RHS `~` is a dash-literal ⇒ stored concrete"
        );
    }

    #[test]
    fn for_list_glob_word_makes_for_var_top() {
        // The demonstrated end-to-end channel (`20O` find-1): a `for`-list word that is an
        // unquoted literal glob expands against the fs ⇒ the for-var binds ⊤ ⇒ the in-body
        // use is ⊤. (Before fix-1 the for-var wrongly bound the literal `*.conf`.)
        assert_eq!(
            argv_of(r#"for f in *.conf; do cmd "$f"; done"#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "a glob `for`-list word ⇒ for-var ⊤ ⇒ post-bind use ⊤"
        );
        // A word-leading `~` list word is the same hazard.
        assert_eq!(
            argv_of(r#"for f in ~; do cmd "$f"; done"#, "cmd"),
            vec![lit("cmd"), Word::Top],
            "a word-leading `~` `for`-list word ⇒ for-var ⊤"
        );
        // Control: a glob-free literal list word still resolves precisely (no over-degrade).
        assert_eq!(
            argv_of(r#"for f in nginx; do cmd "$f"; done"#, "cmd"),
            vec![lit("cmd"), lit("nginx")],
            "a glob-free single literal list word still resolves (no over-degrade)"
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
