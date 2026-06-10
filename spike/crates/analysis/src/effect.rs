//! Per-command system-state effects + the ambient-invariant gate — the input the
//! skip decision consumes (this module decides *nothing* itself; it classifies).
//!
//! Two steps (note 163 §2):
//! 1. **effect resolution** — for each `Command` node, thread the book's
//!    flow-resolved argv (`analysis::value::ValueFlow`) through the oracle's own
//!    `check()` (`oracle::check::evaluate`) to its inline kind-annotation, then key
//!    the resulting `(verb, entity, kind)` into the oracle effect-map for the
//!    `(selector, polarity)` cells (`Establishes`/`Kills`, or `Opaque` on any ⊤).
//!    The engine parses NO argv itself — *identity is declared, never inferred*
//!    (`inv-referent-agnostic`); the old engine-side flag-strip stand-in is gone.
//! 2. **ambient gate** — a forward reaching-definitions pass over the mutated
//!    facts: a fact is *ambient* at a command iff NO upstream in-script command
//!    mutated it (so the host's resting state is authoritative and we may probe
//!    it). A fact mutated upstream is *written* — its resting value is stale —
//!    catching `apt-get purge X; … apt-get install X` (note 162 O-1 / break-10).
//!
//! Lock-style note (note 165): this module is deliberately **all forward-may**
//! (over-approximate), so there is no `May`/`Must` wrapper here yet — there is
//! nothing of the opposite orientation to confuse it with. That lock arrives with
//! the first *must* analysis (statically-definitely-established) and the backward
//! apply-slice. Here the only conservative direction is "when unsure ⇒ `Opaque`
//! ⇒ not ambient ⇒ run", which is safe for the skip decision.

use crate::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use crate::lattice::Lattice;
use crate::solve::{Direction, Graph, solve};
use crate::value::{ValueFlow, ValueOf};
use dorc_core::{
    Carrier, DiagCode, Diagnostic, EntityRef, Interner, KindId, OpaqueToken, ProviderId,
};
use dorc_oracle::check::{self, CheckSet, ResolvedEntity};
use dorc_oracle::{EffectCell, KindIndex, Polarity, empty_verb};
use std::collections::BTreeSet;

/// The dataflow fact-key the engine reaches over. **Re-exported from `core`**
/// (`dec-seam-ownership`, `notes/193` §2): the structured entity-algebra is the
/// shared vocabulary defined in `core` so `oracle`/`plan`/`hostsim` all key on one
/// type; `analysis` is its largest *consumer*, not a parallel owner. The flat
/// `(kind, entity)` pair of spike-1 became `core::FactKey { kind, entity:
/// EntityRef, selector }` — the per-entity selector is what kills the poison wall
/// (`apt-get update` ⇒ `package-index#fresh`, distinct from `install`'s
/// `package:nginx#installed`).
pub use dorc_core::FactKey;

/// What a command does to — or *observes about* — system state, as far as the
/// analyzer can determine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandEffect {
    /// No modeled system-state effect (a bare assignment, or a read-only command).
    Pure,
    /// Establishes `fact` (`apt-get install nginx`).
    Establishes(FactKey),
    /// Kills `fact` (`apt-get purge nginx`).
    Kills(FactKey),
    /// READS `fact` and mutates nothing — the read-only guard-class (`command -v
    /// nginx` ⇒ `Queries(tool:nginx#present)`; 202 §2 / task-D2). A `Query`
    /// **poisons no reaching-defs and establishes nothing** (the reaching-defs gen
    /// treats it like `Pure`): a guard reads state, it does not change it, so it must
    /// not force a downstream establish to `EstablishWritten` nor invalidate a
    /// downstream Query (st-3, 20A §4). Distinct from `Pure` only so a Query SITE is
    /// probe-resolvable (its check IS the probe) and its probed rc can feed the fold's
    /// Status channel (gated by rule-query-validity).
    Queries(FactKey),
    /// ⊤: cannot characterize — a ⊤ argv word, no provider/check, an `evaluate`
    /// `Top`, or no effect-map entry for the resolved verb. Conservatively MAY mutate
    /// anything (so it poisons downstream ambient-ness) and itself must run
    /// (`inv-top-reject`).
    Opaque,
}

/// Diagnostic code for a kind-disagreement between a check's annotation and the
/// effect-map (`ch-catalog`; 204 §6 open seam).
const KIND_DISAGREEMENT: DiagCode = DiagCode("effect-kind-disagreement");

/// Determine a `Command` node's effect cells from the book's resolved argv + the
/// oracle's own `check()` (the real entity-resolution mechanism; replaces the deleted
/// engine-side argparse stand-in). The engine parses NOTHING: it threads the
/// flow-resolved argv through the oracle's argparse (`check::evaluate`) and reads
/// the inline kind-annotation. *Identity is declared, never inferred* — true in
/// code now (`inv-referent-agnostic`).
///
/// Returns a `Vec` of cells (a multi-cell verb is legal — `us-effectmap`); each is
/// `Establishes`/`Kills`/`Queries` (`Queries` is the read-only guard-class, 202 §2).
/// ANY ⊤ — a ⊤ argv word, no provider, no check, an `evaluate` `Top`, or no
/// effect-map entry — yields `[Opaque]` (`inv-top-reject`: the degrade is the floor;
/// both `kFAIL` directions). A bare assignment yields `[Pure]`.
///
/// `inv-superposition`: the cells are phase-/orientation-agnostic facts; this
/// classifies, it decides nothing. Diagnostics (kind-disagreement) accumulate in
/// `diags`.
pub fn command_effect(
    idx: &KindIndex,
    checks: &[CheckSet],
    argv: &[ValueOf],
    interner: &mut Interner,
    diags: &mut Vec<Diagnostic>,
) -> Vec<CommandEffect> {
    // A bare assignment-only command (`pkg=nginx`) has an empty argv ⇒ no
    // system-state effect (value::analyze yields `[]` for words.is_empty()).
    let Some(&word0) = argv.first() else {
        return vec![CommandEffect::Pure];
    };
    // The command word must be a concrete literal; a ⊤ word (`"$dyn" install …`) is
    // an un-modeled command ⇒ Opaque (`inv-top-reject`).
    let ValueOf::Literal(provider_sym) = word0 else {
        return vec![CommandEffect::Opaque];
    };
    let provider_str = interner.resolve(provider_sym).to_owned();
    // Target-state-pure shell builtins (shell-env/stdout/control only, never an
    // oracle-modeled fact) are Pure, not Opaque, so they do NOT poison downstream
    // ambient-ness (fs-4 / spec_set_e; note 16G §4 B). Anything not listed stays
    // Opaque (the safe over-refusing direction).
    if is_target_state_pure_builtin(&provider_str) {
        return vec![CommandEffect::Pure];
    }
    // The provider symbol: the book's command word through the SHARED hyphen↔underscore
    // convention (`check::map_provider_name`) — so it equals the `CheckSet` key and
    // `KindIndex`'s `ProviderId` (204 §6 seam #2). The book word is already hyphenated
    // (`apt-get`), so the map is a no-op here, but routing through the one helper keeps
    // the vocabularies welded.
    let provider = ProviderId(interner.intern(&check::map_provider_name(&provider_str)));

    // The trailing args (command word excluded — C-1) must ALL be concrete literals
    // to run the check (202 §1 fully-concrete-argv scope). A ⊤ hole ⇒ unresolved site
    // ⇒ Opaque (runs). Collect the resolved text, holding it so `&str` slices borrow
    // it for `evaluate`.
    let mut arg_texts: Vec<String> = Vec::with_capacity(argv.len().saturating_sub(1));
    for word in &argv[1..] {
        match word {
            ValueOf::Literal(s) => arg_texts.push(interner.resolve(*s).to_owned()),
            ValueOf::Top => return vec![CommandEffect::Opaque], // ⊤ arg ⇒ unresolved
        }
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    // Run the oracle's own argparse. Multiple oracle files may each declare a check
    // for this provider (`apt-get` install/purge in one file, `apt-get update` in
    // another — different kinds, different authors); try each and take the FIRST that
    // resolves concretely. A check that does not handle this verb falls through to its
    // own `Top` (no annotation reached / positional past end), so the partition is
    // clean for the corpus. (tc-*: if two checks both resolve, first-in-file-order
    // wins — flagged; no corpus case is ambiguous.) The `CheckSet` key is the same
    // provider symbol (interning is idempotent; `ProviderId` wraps it).
    let resolved = checks
        .iter()
        .filter_map(|cs| cs.get(provider.0))
        .find_map(|c| match check::evaluate(c, &arg_refs) {
            check::Resolution::Resolved(r) => Some(r),
            check::Resolution::Top(_) => None,
        });
    let Some(resolved) = resolved else {
        // No check resolved this site (no check for the provider, or every candidate
        // degraded to ⊤). ⊤ ⇒ Opaque (`inv-top-reject`). We do NOT fall back to a
        // verb-by-position lookup — that was the deleted engine-side argparse sin.
        return vec![CommandEffect::Opaque];
    };

    // The verb key: the check's derived verb, or the ε-verb when the check binds none
    // (`useradd`, `command -v` — 202 §2 / task-W §4). `evaluate`'s verb is compared
    // against the effect-map's verb through the SAME `Interner` (204 seam #2).
    let verb_key = match &resolved.verb {
        Some(v) => interner.intern(v),
        None => empty_verb(interner),
    };
    let cells = idx.effect_of(provider, verb_key);
    if cells.is_empty() {
        // The check resolved an identity, but no oracle declared an effect for this
        // (provider, verb). Not this analysis's concern ⇒ ⊤ (runs). A read-only guard
        // whose oracle declares `oracle_effect … query …` lands as `Queries` below;
        // only an un-declared guard falls through to Opaque here (task-D2, 202 §2).
        return vec![CommandEffect::Opaque];
    }

    // The cell's kind comes from the annotation (the declared identity, 204 §6); the
    // effect-map supplies selector + polarity per (provider, verb). Kind-agreement
    // (204 open seam): if a cell's effect-map kind disagrees with the annotation kind,
    // diagnose and let the ANNOTATION win (the effect-map row is re-keyed under it).
    let annotation_kind = KindId(interner.intern(&resolved.kind));
    let entity = match &resolved.entity {
        ResolvedEntity::Operand(text) => EntityRef::Operand(OpaqueToken(interner.intern(text))),
        ResolvedEntity::Singleton => EntityRef::Singleton,
    };
    // `EffectCell` is `Copy` and `cells` borrows `idx` (disjoint from `&mut interner`),
    // so iterate by copy — `cell_effect` takes `&mut interner` for the kind-agreement
    // diagnostic without conflicting with the `idx` borrow.
    cells
        .iter()
        .copied()
        .map(|cell| {
            cell_effect(
                cell,
                annotation_kind,
                &resolved.kind,
                entity,
                interner,
                diags,
            )
        })
        .collect()
}

/// Build one [`CommandEffect`] from a declared [`EffectCell`] under the resolved
/// (annotation-kind, entity). Enforces the kind-agreement rule (204 §6): the
/// annotation is the declared identity, so on a mismatch the cell is re-keyed under
/// the annotation kind and a warning is recorded.
fn cell_effect(
    cell: EffectCell,
    annotation_kind: KindId,
    annotation_kind_str: &str,
    entity: EntityRef,
    interner: &mut Interner,
    diags: &mut Vec<Diagnostic>,
) -> CommandEffect {
    if cell.kind != annotation_kind {
        let em_kind = interner.resolve(cell.kind.0).to_owned();
        diags.push(Diagnostic::warning(
            KIND_DISAGREEMENT,
            None,
            format!(
                "check annotation kind `{annotation_kind_str}` disagrees with the effect-map \
                 kind `{em_kind}` for this verb — the annotation (declared identity) wins"
            ),
        ));
    }
    let fact = FactKey {
        kind: annotation_kind, // the annotation wins (declared identity)
        entity,
        selector: cell.selector,
    };
    match cell.polarity {
        Polarity::Establish => CommandEffect::Establishes(fact),
        Polarity::Kill => CommandEffect::Kills(fact),
        Polarity::Query => CommandEffect::Queries(fact),
    }
}

/// Shell builtins with no *target-system* (location-3) effect: they change shell
/// options/cwd/variables or write to stdout/stderr, but never a package/file/
/// service fact an oracle models. Treated as `Pure` so they don't poison
/// reaching-defs ambient-ness (note 16G). Deliberately small and conservative —
/// anything not listed stays `Opaque` (the safe over-refusing direction); the
/// dynamic-lvalue forms (`unset "$x"`, `printf -v`) are already ⊤-rejected upstream
/// by the parser, so only their static uses reach here.
fn is_target_state_pure_builtin(word: &str) -> bool {
    matches!(
        word,
        "set"
            | "cd"
            | "export"
            | "unset"
            | "shift"
            | "read"
            | "readonly"
            | "local"
            | ":"
            | "true"
            | "false"
            | "echo"
            | "printf"
            | "test"
            | "["
    )
}

/// Facts mutated by some command on a path to here — or `Top` once an `Opaque`
/// command has run (then ANY fact may have changed). This is the reaching-defs
/// lattice; a fact is ambient at a point iff it is NOT in the in-state here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reach {
    Facts(BTreeSet<FactKey>),
    Top,
}

impl Lattice for Reach {
    fn bottom() -> Self {
        Reach::Facts(BTreeSet::new())
    }
    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (Reach::Top, _) | (_, Reach::Top) => Reach::Top,
            (Reach::Facts(a), Reach::Facts(b)) => Reach::Facts(a.union(b).copied().collect()),
        }
    }
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // `Top` is the join-absorbing ⊤, hence meet's identity (`⊤ ⊓ x = x`).
            (Reach::Top, x) | (x, Reach::Top) => x.clone(),
            (Reach::Facts(a), Reach::Facts(b)) => {
                Reach::Facts(a.intersection(b).copied().collect())
            }
        }
    }
}

impl Reach {
    fn with(&self, fact: FactKey) -> Reach {
        match self {
            Reach::Top => Reach::Top,
            Reach::Facts(s) => {
                let mut s = s.clone();
                s.insert(fact);
                Reach::Facts(s)
            }
        }
    }
    fn mutated(&self, fact: &FactKey) -> bool {
        match self {
            Reach::Top => true,
            Reach::Facts(s) => s.contains(fact),
        }
    }

    /// Is this a **pristine** reaching-state — NO write-or-unknown reached here? The
    /// rule-query-validity bit (205 §2 / 20A §4 st-3): a Query's probed rc is
    /// fold-usable iff no invalidating command (an oracled MUTATOR — any
    /// Establish/Kill — or an Opaque) reaches the guard from entry. Because Queries
    /// and pure builtins gen nothing into `Reach`, "no write-or-unknown reached" is
    /// exactly the empty (⊥) fact-set; a non-empty set (some mutator genned a cell) or
    /// `Top` (an opaque ran) is non-pristine ⇒ the guard's resting rc is stale.
    fn is_pristine(&self) -> bool {
        matches!(self, Reach::Facts(s) if s.is_empty())
    }
}

/// How a `Command` relates to the skip decision. This is the *input* the probe/
/// plan stage consumes — it does not skip anything itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipClass {
    /// Not an elidable establish — opaque, pure, kill, unrecognized, OR an
    /// establish whose reaching-context cannot be trusted: unreachable from entry
    /// (e.g. a function body with no modeled call-edge), or produced under a
    /// non-converged solve ⇒ run.
    MustRun,
    /// Establishes `fact`, ambient here (no upstream mutation) ⇒ probe the host.
    EstablishAmbient(FactKey),
    /// Establishes `fact`, but the fact was mutated upstream in-script ⇒ the
    /// resting probe is not authoritative ⇒ run (or reason in-script; conservatively
    /// run). The `purge X; … install X` case.
    EstablishWritten(FactKey),
    /// A read-only **Query** guard reading `fact` (`command -v nginx` ⇒
    /// `tool:nginx#present`; 202 §2 / task-D2). Probe-resolvable like an
    /// `EstablishAmbient` (its check IS the probe), but its probed rc feeds the
    /// fold's **Status** channel rather than gating a mutation-elision — and ONLY
    /// when [`valid`](SkipClass::QueryResolvable::valid) holds.
    ///
    /// `valid` is the rule-query-validity bit (205 §2 / 20A §4 st-3): the guard's
    /// probe-time rc is fold-usable IFF NO invalidating command reaches the guard
    /// from entry — invalidating = an oracled MUTATOR (any Establish/Kill) or an
    /// Opaque; NOT invalidating = other Queries or blessed-pure builtins. When
    /// `valid == false` the guard's resting rc is stale (a mutator may have changed
    /// the cell), so the phased caller withholds the rc (status ⇒ ⊤) and the guard
    /// runs for real at apply — never a stale fold (`inv-superposition`: the bit is a
    /// phase-agnostic fact; the collapse stays in the caller).
    QueryResolvable { fact: FactKey, valid: bool },
}

/// Nodes reachable from the program `entry` by forward edges. The reaching-defs
/// in-state of an *unreachable* node is a vacuous ⊥ (its only predecessors, if
/// any, are themselves unreached), which is indistinguishable from a genuinely
/// clean "nothing upstream mutated this fact" — so [`classify`] must not read an
/// unreachable establish as ambient (find-A). Today the only unreachable
/// `Command`s are function bodies: a call site has no modeled call-edge into the
/// body (cfg `find-7`), so the body is a detached island. A simple forward
/// graph reachability; deterministic and total (indices are in-bounds by
/// construction, so it never panics — `inv-no-throw`).
fn reachable_from_entry(cfg: &Cfg) -> Vec<bool> {
    let mut seen = vec![false; cfg.node_count()];
    let mut stack = vec![cfg.entry()];
    seen[cfg.entry().index()] = true;
    while let Some(v) = stack.pop() {
        for w in cfg.succ_ids(v) {
            if !seen[w.index()] {
                seen[w.index()] = true;
                stack.push(w);
            }
        }
    }
    seen
}

/// Classify every `Command` node for the skip decision: resolve each command's
/// effect cells (through the book's value-flow [`ValueFlow`] + the oracle's own
/// `check()`), then a forward reaching-defs pass tells us, per establishing command,
/// whether its fact is ambient. An establish is only offered as `EstablishAmbient`
/// when its reaching-context is *trustworthy* — reachable from entry AND under a
/// converged solve; otherwise it folds to the safe `MustRun` (find-A/find-B).
///
/// `value` is the book-side value-flow (`analysis::value::analyze`, the caller
/// threads it); `checks` are the per-oracle-file `CheckSet`s (the engine parses no
/// argv itself — `inv-referent-agnostic`). Returns a [`Carrier`] so kind-disagreement
/// warnings (204 §6) surface. Deterministic; never panics (`inv-no-throw`).
#[must_use]
pub fn classify(
    cfg: &Cfg,
    value: &ValueFlow,
    idx: &KindIndex,
    checks: &[CheckSet],
    interner: &mut Interner,
) -> Carrier<Vec<(CfgNodeId, SkipClass)>> {
    let n = cfg.node_count();
    let mut diags: Vec<Diagnostic> = Vec::new();
    // Precompute each node's effect cells once (interning happens here, with &mut).
    // A multi-cell verb yields several cells; the reaching-defs gen applies each.
    let effects: Vec<Vec<CommandEffect>> = (0..n)
        .map(|i| {
            let id = CfgNodeId(i as u32);
            match cfg.node(id).kind {
                CfgNodeKind::Command => {
                    let argv = value.argv_values(id);
                    command_effect(idx, checks, &argv, interner, &mut diags)
                }
                // An unmodeled construct may mutate anything ⇒ ⊤.
                CfgNodeKind::Top => vec![CommandEffect::Opaque],
                _ => vec![CommandEffect::Pure],
            }
        })
        .collect();

    // Forward reaching-defs: out = in ⊔ gen(node). Each of a node's cells is genned
    // (a multi-cell verb writes every cell); an Opaque cell joins ⊤. A `Queries` cell
    // gens NOTHING — a read poisons no ambient-ness and invalidates no downstream
    // Query (it is a write-free observation; task-D2 / st-3, 20A §4). This is the
    // gen-side of rule-query-validity: because a Query gens nothing, `reach.states`
    // (the IN-state at each node) carries exactly the writes-or-opaque that reached it.
    let reach = solve(cfg, Direction::Forward, |i, incoming: &Reach| {
        let mut state = incoming.clone();
        for cell in &effects[i] {
            state = match cell {
                CommandEffect::Establishes(f) | CommandEffect::Kills(f) => state.with(*f),
                CommandEffect::Opaque => state.join(&Reach::Top),
                CommandEffect::Pure | CommandEffect::Queries(_) => state,
            };
        }
        state
    });
    debug_assert!(
        reach.converged,
        "reaching-defs must converge (finite fact set)"
    );

    // Two reasons the reaching in-state cannot be trusted to mean "nothing
    // upstream mutated this fact", both folding the safe way (→ MustRun):
    //   * non-convergence (find-B): a capped solve returns a partial
    //     under-approximation — a real upstream kill may not have propagated. The
    //     `Reach` lattice is monotone + finite-height so this never trips here (the
    //     `debug_assert` catches a regression loudly), but trusting a non-converged
    //     state in *release* would be a silent wrong-skip, so guard it explicitly.
    //   * unreachability (find-A): an establish unreachable from entry has a vacuous
    //     ⊥ in-state; its true call context is unmodeled (cfg find-7).
    let trust_reach = reach.converged;
    let reachable = reachable_from_entry(cfg);

    let mut out = Vec::new();
    for (i, cells) in effects.iter().enumerate() {
        let id = CfgNodeId(i as u32);
        // Only genuinely-runnable command leaves are plan/apply units. A command
        // inside a `$( … )` substitution body is effect-bearing (it stayed in the
        // reaching-defs above, so its mutations still poison/establish) but is NOT
        // a leaf (find-cli-1, the dn-3 leaf-seam).
        if cfg.node(id).kind != CfgNodeKind::Command || cfg.is_expansion_internal(id) {
            continue;
        }
        // The elision candidate is a SINGLE establish cell (the corpus shape). A
        // multi-cell establish has no single-fact `SkipClass` representation yet, so
        // it folds to `MustRun` — sound (`kFAIL-perform`: never wrongly elided), the
        // run-it floor; the reaching-defs above still tracked every cell. (Flagged:
        // multi-fact elision is unbuilt past `SkipClass`'s single-fact shape.)
        let class = match cells.as_slice() {
            // Only a reachable establish under a converged solve is reasoned about;
            // every other case — opaque/pure/kill, multi-cell, an unreachable
            // establish, or a non-converged solve — folds to MustRun.
            [CommandEffect::Establishes(f)] if trust_reach && reachable[i] => {
                if reach.states[i].mutated(f) {
                    SkipClass::EstablishWritten(*f)
                } else {
                    SkipClass::EstablishAmbient(*f)
                }
            }
            // A read-only Query guard, reachable + converged: probe-resolvable, with
            // its rule-query-validity bit (205 §2 / 20A §4 st-3) read off the IN-state
            // — pristine (no write-or-unknown reached from entry) ⇒ the probed rc is
            // fold-usable. `reach.states[i]` is the in-state (solve.rs: states[v] is
            // the state *before* node v); a Query gens nothing, so it is exactly the
            // writes-or-opaque that reached the guard. An unreachable/non-converged
            // guard folds to MustRun like any other (kFAIL-perform).
            [CommandEffect::Queries(f)] if trust_reach && reachable[i] => {
                SkipClass::QueryResolvable {
                    fact: *f,
                    valid: reach.states[i].is_pristine(),
                }
            }
            _ => SkipClass::MustRun,
        };
        out.push((id, class));
    }
    Carrier { value: out, diags }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg;
    use crate::value::analyze;
    use dorc_core::{KindId, SelectorId};
    use dorc_oracle::check::lift_checks;

    /// The shared corpus-shaped check dialect the classify tests lift: an `apt-get`
    /// check (flag-strip → verb → per-verb arm: `update` ⇒ a Singleton `package-index`
    /// annotation; everything else ⇒ a post-verb flag-strip, the single-operand
    /// `package` annotation, and a `[ "$2" = "" ]` guard that refuses a SECOND operand
    /// — `install nginx curl` reaches no probe ⇒ Top ⇒ runs), plus a `systemctl` check
    /// (verb → per-arm probe). Annotation kinds MATCH the effect-map's (`package`,
    /// `package-index`, `service`) so the kind-agreement rule never fires. The probe
    /// bodies are inert placeholders (this round resolves identity only).
    ///
    /// Lifted with the CALLER's interner (`i`), so the [`CheckSet`]'s provider symbol
    /// equals the one `classify` interns from the book's command word (Symbols only
    /// compare across one interner — 204 seam #2).
    const CORPUS_CHECK_SRC: &str = r#"
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      update) idx : package-index; probe-fresh ;;
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then probe-pkg "$pkg"; fi ;;
   esac
}
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable) probe-enabled "$svc" ;;
      start)  probe-active "$svc" ;;
      disable) probe-enabled "$svc" ;;
   esac
}
command__check() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1
}
"#;

    /// The interned ids a package-fixture test asserts against. Kept together so a
    /// test reads `s.installed` / `s.nginx` instead of re-interning inline.
    struct Syms {
        package: KindId,
        package_index: KindId,
        installed: SelectorId,
        fresh: SelectorId,
    }

    /// Build (interner, index, syms) modeling the package oracle's effects — now
    /// *including* `apt-get update → (package-index, #fresh)`, the modeled nullary
    /// that the poison-wall fix relies on (`notes/193` §1). `install`/`purge` gate
    /// the `#installed` selector of `package`.
    fn package_setup() -> (Interner, KindIndex, Syms) {
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let package_index = KindId(interner.intern("package-index"));
        let installed = SelectorId(interner.intern("installed"));
        let fresh = SelectorId(interner.intern("fresh"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");
        let purge = interner.intern("purge");
        let update = interner.intern("update");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, Polarity::Establish);
        idx.add_effect(apt, purge, package, installed, Polarity::Kill);
        idx.add_effect(apt, update, package_index, fresh, Polarity::Establish);
        (
            interner,
            idx,
            Syms {
                package,
                package_index,
                installed,
                fresh,
            },
        )
    }

    /// `package:<entity>#installed` — the cell `apt-get install <entity>` gates.
    fn pkg_installed(i: &mut Interner, s: &Syms, entity: &str) -> FactKey {
        FactKey {
            kind: s.package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: s.installed,
        }
    }

    /// Run the full pipeline on `src` (value-flow + the corpus checks + classify) and
    /// return just the [`SkipClass`]es, in classify order. Everything shares one
    /// interner so the [`CheckSet`]'s provider symbols match the book's command words.
    fn classify_src(src: &str, interner: &mut Interner, idx: &KindIndex) -> Vec<SkipClass> {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, interner);
        let checks = vec![lift_checks(interner, CORPUS_CHECK_SRC).value];
        classify(&built.value, &value, idx, &checks, interner)
            .value
            .into_iter()
            .map(|(_, c)| c)
            .collect()
    }

    #[test]
    fn lone_install_is_ambient() {
        // Why: the simplest establish with nothing upstream — must be probe-able
        // (EstablishAmbient), the precondition for ever skipping it.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get install nginx", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))]
        );
    }

    #[test]
    fn upstream_purge_makes_install_written() {
        // Why (note 162 O-1 / break-10, THE wrong-skip): an upstream same-run kill
        // means the resting probe is stale — the install must NOT be treated as
        // ambient/skippable. purge + install gate the SAME (package:nginx#installed)
        // cell, so the kill reaches the establish.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        // purge ⇒ MustRun (a kill, not an elidable establish); install ⇒ Written.
        assert!(classes.contains(&SkipClass::EstablishWritten(pkg_installed(
            &mut i, &s, "nginx"
        ))));
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn opaque_upstream_poisons_ambientness() {
        // Why (note 162 O-3, the precision COST being surfaced): a genuinely
        // unrecognized command (`ufw allow` — NO oracle entry) is still Opaque ⇒ ⊤ ⇒
        // it conservatively poisons every downstream fact's ambient-ness. The re-key
        // does NOT rescue an un-oracled neighbor; it rescues a *modeled* nullary
        // (`apt-get update`, the `poison_wall_dies_*` test below). This documents the
        // residual, correct cost so we still feel it.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("ufw allow 80/tcp\napt-get install nginx", &mut i, &idx);
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_)))
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn poison_wall_dies_modeled_update_does_not_poison_install() {
        // THE keystone win (`notes/193` §1 / acceptance §7.2): a modeled `apt-get
        // update` establishes a *distinct cell* (`package-index#fresh`), so it no
        // longer poisons the `apt-get install nginx` below it. Before the re-key,
        // `update` was doubly-unkeyable (no operand, and — pre-modeling — no verb) ⇒
        // Opaque ⇒ Reach::Top ⇒ install forced EstablishWritten. Now it's ambient.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get update\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "modeled `update` (distinct cell) must leave install EstablishAmbient: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no Written: update's cell (package-index#fresh) ≠ install's (package:nginx#installed)"
        );
    }

    #[test]
    fn genuine_same_cell_kill_still_forces_written() {
        // exclusion-check (`notes/193` §7.3): the re-key must NOT over-loosen the
        // ambient gate. A real same-cell kill (`purge nginx; install nginx`, both on
        // package:nginx#installed) must STILL force Written — resting probe is stale.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishWritten(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "same-cell purge must keep install EstablishWritten (no over-loosening): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn distinct_selectors_do_not_discharge_each_other() {
        // The selector regression (`notes/193` §7.4): `systemctl enable nginx` and
        // `systemctl start nginx` gate DIFFERENT selectors of the SAME service:nginx
        // cell (#enabled vs #active). Neither discharges the other — both stay
        // EstablishAmbient (an `is-active` probe must not satisfy an unmet `#enabled`).
        // A flat key (one bit per kind+entity) could not hold this — the second would
        // see the first reach its cell and (mis-)classify Written.
        let mut i = Interner::default();
        let service = KindId(i.intern("service"));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        let systemctl = ProviderId(i.intern("systemctl"));
        let enable = i.intern("enable");
        let start = i.intern("start");
        let mut idx = KindIndex::default();
        idx.add_effect(systemctl, enable, service, enabled, Polarity::Establish);
        idx.add_effect(systemctl, start, service, active, Polarity::Establish);

        let classes = classify_src(
            "systemctl enable nginx\nsystemctl start nginx",
            &mut i,
            &idx,
        );
        let enabled_cell = FactKey {
            kind: service,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: enabled,
        };
        let active_cell = FactKey {
            kind: service,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: active,
        };
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(enabled_cell)),
            "enable nginx ⇒ service:nginx#enabled, ambient: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(active_cell)),
            "start nginx ⇒ service:nginx#active, ambient (NOT discharged by #enabled): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "distinct selectors ⇒ neither reaches the other's cell ⇒ no Written"
        );
    }

    #[test]
    fn pure_builtin_upstream_does_not_poison_ambientness() {
        // fs-4 (note 16G), the contrast to `opaque_upstream_poisons_ambientness`:
        // the blessed target-state-pure builtins (`:`/`echo`/`cd`/…) touch
        // shell-env/stdout, never an oracle-modeled fact, so they must NOT poison a
        // later establish's ambient-ness. Guards the WHOLE `is_target_state_pure_builtin`
        // allowlist + the Ambient-vs-Written line (the `set`-only end-to-end case does
        // not isolate this); a mis-edit dropping `:`/`echo` would silently re-poison —
        // a wrong-skip surface.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(":\necho hi\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "pure builtins (`:`/`echo`) upstream must keep the install EstablishAmbient: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no spurious Written from a pure-builtin upstream"
        );
    }

    #[test]
    fn detached_function_body_establish_is_not_ambient() {
        // Why (find-A; both adversarial reviews independently, brk-1 / fs-1): a
        // function body is a detached sub-CFG — its caller has no modeled call-edge
        // (cfg find-7), so the in-body install is unreachable from entry and its
        // reaching-in-state is a vacuous ⊥. Reading that as "nothing upstream" would
        // advertise the establish skippable under an unknown call context — a
        // kFAIL-perform wrong-skip. It must fold to MustRun (with the `p` call, also
        // MustRun) until interprocedural call-edges land. The contrast with
        // `lone_install_is_ambient` (identical establish, ambient at top level)
        // proves the reachability gate is doing the work.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { apt-get install nginx; }\np", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::MustRun, SkipClass::MustRun],
            "detached body install + the call both fold to MustRun, no ambient skip"
        );
    }

    #[test]
    fn command_effect_resolves_operand_singleton_and_top() {
        // Resolve a single-command book through value-flow + the corpus apt check,
        // returning the node's effect cells. (One command ⇒ one Command node.)
        fn eff(src: &str, i: &mut Interner, idx: &KindIndex) -> Vec<CommandEffect> {
            let parsed = dorc_syntax::parse(src);
            let built = cfg::build(&parsed.value);
            let value = analyze(&built.value, &parsed.value, i);
            let checks = vec![lift_checks(i, CORPUS_CHECK_SRC).value];
            // A dynamic command word (`$cmd …`) is ⊤-rejected by the parser ⇒ a `Top`
            // CFG node, not a `Command` — classify treats that as Opaque. Mirror it.
            let Some(node) = built
                .value
                .iter()
                .find(|(_, n)| n.kind == CfgNodeKind::Command)
                .map(|(id, _)| id)
            else {
                return vec![CommandEffect::Opaque];
            };
            let mut diags = Vec::new();
            command_effect(idx, &checks, &value.argv_values(node), i, &mut diags)
        }
        let (mut i, idx, s) = package_setup();
        // One operand ⇒ Operand cell; the flag `-y` is post-verb-stripped by the check.
        let nginx_cell = pkg_installed(&mut i, &s, "nginx");
        assert_eq!(
            eff("apt-get install -y nginx", &mut i, &idx),
            vec![CommandEffect::Establishes(nginx_cell)],
            "the check strips the post-verb -y ⇒ Operand(nginx)#installed"
        );
        // Nullary modeled verb (`update`) ⇒ the check's value-less `package-index`
        // annotation ⇒ Singleton (the poison-wall fix). A flag-only tail stays nullary
        // (the `update` arm ignores the trailing `-y`).
        let pkg_index_fresh = CommandEffect::Establishes(FactKey {
            kind: s.package_index,
            entity: EntityRef::Singleton,
            selector: s.fresh,
        });
        assert_eq!(
            eff("apt-get update", &mut i, &idx),
            vec![pkg_index_fresh.clone()],
            "nullary modeled verb ⇒ Singleton(package-index#fresh)"
        );
        assert_eq!(
            eff("apt-get update -y", &mut i, &idx),
            vec![pkg_index_fresh],
            "flag-only tail stays nullary ⇒ Singleton"
        );
        // A non-literal operand (`$PKG` ⇒ ⊤ in value-flow) is an UNKNOWN cell, NOT the
        // singleton — else `install $PKG` would be wrongly elidable (priority-1
        // wrong-elision). ⊤ arg ⇒ unresolved site ⇒ Opaque ⇒ run.
        assert_eq!(
            eff("apt-get install $PKG", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "non-literal operand ⇒ ⊤, not Singleton"
        );
        // Multi-operand: the single-`$1` check binds nginx, but its `[ "$2" = "" ]`
        // guard sees the SECOND operand `curl` ⇒ no probe reached ⇒ Top ⇒ Opaque ⇒ run.
        // This is the check's OWN multi-operand refusal (the oracle's code, not the
        // engine): a wrong single-entity elision that would silently drop `curl` is
        // avoided — the safety the deleted engine-side stand-in used to provide.
        assert_eq!(
            eff("apt-get install nginx curl", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "second operand ⇒ the check's guard refuses ⇒ ⊤"
        );
        // Dynamic command name ⇒ ⊤ word0 ⇒ Opaque.
        assert_eq!(
            eff("$cmd install nginx", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "dynamic command name ⇒ ⊤"
        );
        // Unknown verb: `autoclean` ⇒ the check's `*` arm reads `$1` (past end ⇒ Top),
        // and the effect-map has no (apt-get, autoclean) row anyway ⇒ Opaque.
        assert_eq!(
            eff("apt-get autoclean", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "unknown verb ⇒ ⊤"
        );
    }

    #[test]
    fn multi_operand_is_not_wrongly_elided() {
        // The kFAIL-perform guard the new check preserves (the deleted stand-in's
        // multi-operand refusal): `apt-get install nginx curl` must NOT resolve to a single-entity
        // cell (which could elide, silently dropping curl). It stays MustRun.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("apt-get install nginx curl", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::MustRun],
            "multi-operand install ⇒ MustRun (no single-entity wrong-elision)"
        );
    }

    #[test]
    fn opaque_var_operand_is_top_when_unresolved_but_resolves_when_flowed() {
        // The value-plane's reach: a command-prefix/assigned operand. `PKG=nginx;
        // apt-get install -y "$PKG"` — value-flow resolves `"$PKG"` to nginx, so the
        // site is fully concrete and the check resolves entity=nginx (EstablishAmbient).
        // This is the value-flow win the old engine-side stand-in (which saw `"$PKG"`
        // as a non-literal operand ⇒ Opaque) could not reach. Contrast: an UNASSIGNED
        // `"$X"` stays ⊤ ⇒ Opaque. (GOLDEN: `exec-opaque-var` flips elsewhere — flagged.)
        let (mut i, idx, s) = package_setup();
        // The bare `PKG=nginx` assignment is also a leaf (MustRun); the install is the
        // one we assert resolved.
        let flowed = classify_src("PKG=nginx\napt-get install -y \"$PKG\"", &mut i, &idx);
        assert!(
            flowed.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "value-flow resolves the assigned operand ⇒ the install is identity-resolved: {flowed:?}"
        );
        let unresolved = classify_src("apt-get install -y \"$X\"", &mut i, &idx);
        assert_eq!(
            unresolved,
            vec![SkipClass::MustRun],
            "an unassigned ⊤ operand ⇒ unresolved site ⇒ MustRun"
        );
    }

    // --- task-D2: the Query effect-class + rule-query-validity (202 §2 / 205 §2) ---

    /// `tool:<entity>#present` — the cell `command -v <entity>` queries.
    fn tool_present(i: &mut Interner, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("present")),
        }
    }

    /// A package index (install/purge/update) PLUS a read-only `command '' query
    /// present` guard on `tool` (the canonical `command -v` Query). Threads a
    /// caller-provided interner so the Query tests share one across index-build +
    /// classify + assertions.
    fn package_and_query_index(i: &mut Interner) -> KindIndex {
        let package = KindId(i.intern("package"));
        let package_index = KindId(i.intern("package-index"));
        let installed = SelectorId(i.intern("installed"));
        let fresh = SelectorId(i.intern("fresh"));
        let apt = ProviderId(i.intern("apt-get"));
        let install = i.intern("install");
        let purge = i.intern("purge");
        let update = i.intern("update");
        let tool = KindId(i.intern("tool"));
        let present = SelectorId(i.intern("present"));
        let command = ProviderId(i.intern("command"));
        let eps = empty_verb(i);
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, Polarity::Establish);
        idx.add_effect(apt, purge, package, installed, Polarity::Kill);
        idx.add_effect(apt, update, package_index, fresh, Polarity::Establish);
        idx.add_effect(command, eps, tool, present, Polarity::Query);
        idx
    }

    #[test]
    fn lone_query_guard_is_resolvable_and_valid() {
        // The simplest Query: `command -v nginx` with nothing upstream ⇒
        // QueryResolvable + valid (pristine prefix — no write-or-unknown reached it).
        // This is the headline guard, classified as a first-class read-only Query.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx", &mut i, &idx);
        let fact = tool_present(&mut i, "nginx");
        assert_eq!(
            classes,
            vec![SkipClass::QueryResolvable { fact, valid: true }],
            "a lone Query guard is resolvable + valid: {classes:?}"
        );
    }

    #[test]
    fn query_does_not_poison_downstream_establish() {
        // A Query READS, it does not write — so an upstream `command -v nginx` must NOT
        // poison a downstream `apt-get install nginx`'s ambient-ness (contrast an Opaque
        // neighbour, which does). The install stays EstablishAmbient. This is the
        // gen-side of task-D2 (a Query gens nothing into Reach).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx\napt-get install -y nginx", &mut i, &idx);
        let install = pkg_installed_with(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(install)),
            "an upstream Query must NOT poison the install (it reads, doesn't write): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no Written: a Query gens nothing into Reach"
        );
    }

    #[test]
    fn query_after_query_stays_valid_st3() {
        // st-3 (20A §4): an upstream QUERY does not invalidate a downstream Query (reads
        // don't write — the guard-stack idiom keeps all its folds). Two `command -v`
        // guards: BOTH stay valid. A pure builtin between them likewise doesn't
        // invalidate (it gens nothing).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx\n:\ncommand -v curl", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        let curl = tool_present(&mut i, "curl");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "first Query valid: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: curl,
                valid: true
            }),
            "second Query STILL valid — an upstream Query (+ pure `:`) does not invalidate (st-3): {classes:?}"
        );
    }

    #[test]
    fn query_after_mutator_is_invalid() {
        // rule-query-validity (205 §2): an upstream MUTATOR (a write) invalidates a
        // downstream Query — its resting rc is now stale. `apt-get install curl`
        // (establishes package:curl#installed) ⇒ the `command -v nginx` guard below it
        // is QueryResolvable but INVALID (valid: false). The cell mutated is irrelevant
        // (ANY write invalidates — the pristine-prefix rule, not same-cell).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("apt-get install -y curl\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a Query below a mutator is INVALID (stale resting rc — pristine-prefix fails): {classes:?}"
        );
    }

    #[test]
    fn query_after_opaque_is_invalid() {
        // rule-query-validity, the Opaque arm: an upstream un-oracled (Opaque) command
        // ⇒ Reach::Top ⇒ the downstream Query is INVALID (an unknown command may have
        // changed anything). `ufw allow 80/tcp` is un-oracled here ⇒ Opaque.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("ufw allow 80/tcp\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a Query below an Opaque command is INVALID (⊤ reached it): {classes:?}"
        );
    }

    // --- task-L1 (`209` brk-1): reaching-defs over the loop back-edge -------------

    #[test]
    fn post_loop_install_ambient_when_loop_body_is_pure() {
        // THE brk-1 value-unlock at the EFFECT layer: a PURE loop body (`echo` only —
        // gens nothing) does NOT poison a converged install BELOW the loop. The
        // reaching-defs back-edge carries no write out of the loop, so the post-loop
        // install stays EstablishAmbient (elidable). Pre-L1 the loop was a ⊤ node whose
        // havoc + ⊤-containment killed this — the poison the L1 lowering removes.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for f in a b; do echo \"$f\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a pure loop body must NOT poison the post-loop install: {classes:?}"
        );
    }

    #[test]
    fn opaque_in_loop_body_poisons_post_loop_install() {
        // The honest residual cost (exclusion-check, the other cell): an OPAQUE command
        // inside the loop body propagates Reach::Top across the back-edge and OUT to the
        // post-loop install ⇒ it is forced EstablishWritten (runs). A loop is not magic —
        // an un-oracled body command poisons exactly as it would in straight-line code.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "for f in a b; do ufw allow \"$f\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "an Opaque loop-body command poisons the post-loop install (back-edge ⊤): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_))),
            "no ambient install survives the in-loop Opaque"
        );
    }

    #[test]
    fn classify_converges_on_nested_loop_back_edges() {
        // The reaching-defs fixpoint must converge on a NESTED loop (two back-edges).
        // `classify` carries a `debug_assert!(reach.converged)`; a non-converging
        // reaching-defs would trip it (or, in release, fold every establish to MustRun
        // via `trust_reach`). Drive a nested loop with a body establish and assert we get
        // a classification back at all (the post-loop install) — convergence implied.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "for o in a b; do for p in c d; do apt-get install -y \"$p\"; done; done\nsystemctl reload nginx",
            &mut i,
            &idx,
        );
        assert!(
            !classes.is_empty(),
            "classify returns (reaching-defs converged on the nested back-edges): {classes:?}"
        );
    }

    /// `package:<entity>#installed` via a shared interner (sibling of `pkg_installed`
    /// for the Query tests that build their own index inline).
    fn pkg_installed_with(i: &mut Interner, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("installed")),
        }
    }
}
