//! Per-command system-state effects + the ambient-invariant gate — the input the
//! skip decision consumes (this module decides *nothing* itself; it classifies).
//!
//! Two steps (note 163 §2):
//! 1. **effect lookup** — for each `Command` node, ask the oracle index what its
//!    `(provider, verb, entity)` does (`Establishes`/`Kills` a fact, or `Opaque`).
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
use crate::solve::{solve, Direction, Graph};
use dorc_core::{AstId, EntityRef, Interner, OpaqueToken, ProviderId};
use dorc_oracle::{KindIndex, Polarity};
use dorc_syntax::ast::{Ast, NodeKind, WordPart};
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

/// What a command does to system state, as far as the analyzer can determine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandEffect {
    /// No modeled system-state effect (a bare assignment, or a read-only command).
    Pure,
    /// Establishes `fact` (`apt-get install nginx`).
    Establishes(FactKey),
    /// Kills `fact` (`apt-get purge nginx`).
    Kills(FactKey),
    /// ⊤: cannot characterize — no oracle entry, dynamic command word/verb, or an
    /// ambiguous entity. Conservatively MAY mutate anything (so it poisons
    /// downstream ambient-ness) and itself must run.
    Opaque,
}

/// The single literal text of a Word node, if it is exactly one literal fragment.
/// The *only* statically-trusted case (`haz-unquoted`): a `may_split` word has
/// unknown arity/targets and is never treated as a fixed token.
fn word_literal(ast: &Ast, w: AstId) -> Option<&str> {
    match &ast.node(w).kind {
        NodeKind::Word { parts } => match parts.as_slice() {
            [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
}

/// Determine a `Command` node's effect from the oracle index. ⊤-conservative
/// (`Opaque`) on ANY uncertainty: dynamic command word/verb, no oracle entry, a
/// non-literal operand, or more than one non-flag operand.
///
/// The provider, verb, and operand are each a ⊤-source: a non-literal one forces
/// `Opaque` (`inv-top-reject`). Entity resolution — nullary `Singleton` vs one
/// `Operand` vs ⊤ — is [`resolve_entity`], concentrated there so a ⊤ operand can
/// never silently become a known cell (strain-8; the K1 inline `filter_map` let it).
///
/// The crude flag-strip ("operand = a literal not starting with `-`") is a known
/// coarse spot (note 162 O-3/O-4): sound (errs to `Opaque`/`Singleton`) but
/// value-eroding, and it mis-handles pre-verb flags (`apt-get -t x install …`) and
/// attached values (`-oKey=Val`). Precise per-provider flag grammars are deferred.
pub fn command_effect(
    ast: &Ast,
    idx: &KindIndex,
    interner: &mut Interner,
    simple: AstId,
) -> CommandEffect {
    let NodeKind::Simple { words, .. } = &ast.node(simple).kind else {
        return CommandEffect::Opaque;
    };
    // A bare assignment (no command word) has no modeled system-state effect.
    let Some(&first) = words.first() else {
        return CommandEffect::Pure;
    };
    let Some(provider_s) = word_literal(ast, first) else {
        return CommandEffect::Opaque; // dynamic command name
    };
    // Target-state-pure shell builtins (they touch shell-env / stdout / control
    // only, never an oracle-modeled system-state fact) are Pure, not Opaque — so
    // they do NOT poison downstream ambient-ness (fs-4 / spec_set_e; note 16G §4 B).
    // The poison is broader than `set -e` (every un-oracled token poisons); blessing
    // the common pure builtins recovers the common case, and anything not on the
    // list stays Opaque — the safe, over-refusing direction. (`echo`/`printf` stdout
    // is the observable-liveness gate's concern, not a system-state effect.)
    if is_target_state_pure_builtin(provider_s) {
        return CommandEffect::Pure;
    }
    let provider = ProviderId(interner.intern(provider_s));
    let Some(verb_s) = words.get(1).and_then(|&w| word_literal(ast, w)) else {
        return CommandEffect::Opaque; // no static verb ⇒ no effect to look up
    };
    let verb = interner.intern(verb_s);
    let Some((kind, selector, polarity)) = idx.effect_of(provider, verb) else {
        return CommandEffect::Opaque; // unknown (provider, verb)
    };
    // Resolve the operand cell ⊤-contagiously (strain-8): an operand the analyzer
    // cannot resolve is an *unknown* cell, so it can never collapse into the kind's
    // `Singleton` — it forces `Opaque ⇒ run`. This mirrors the verb/provider
    // ⊤-guards above; `resolve_entity` concentrates `inv-top-reject` at the entity
    // boundary (where the old inline `filter_map` let a ⊤ leak).
    let Some(entity) = resolve_entity(ast, &words[2..], interner) else {
        return CommandEffect::Opaque; // a non-literal operand, or more than one ⇒ ⊤
    };
    let fact = FactKey {
        kind,
        entity,
        selector,
    };
    match polarity {
        Polarity::Establish => CommandEffect::Establishes(fact),
        Polarity::Kill => CommandEffect::Kills(fact),
    }
}

/// Resolve the post-verb words to the entity a [`FactKey`] needs, or `None` if the
/// operand shape is ⊤ (unknown) ⇒ the caller emits `Opaque`. The entity boundary's
/// `inv-top-reject` enforcement, concentrated in one reviewable + unit-testable place
/// (strain-8): the classification is **total** — every word is a flag, a literal
/// operand, or a ⊤ (non-literal) — and ⊤ is **contagious**, so an operand the analyzer
/// cannot resolve forces `None` and can never silently collapse into the kind's
/// `Singleton` cell (the priority-1 wrong-elision the inline `filter_map` allowed).
///
/// * no non-flag operand at all (`apt-get update`, or only flags) ⇒ `Singleton`;
/// * exactly one literal non-flag operand ⇒ `Operand`;
/// * a non-literal operand (`install $PKG`), or more than one ⇒ `None` (⊤).
fn resolve_entity(ast: &Ast, post_verb: &[AstId], interner: &mut Interner) -> Option<EntityRef> {
    let mut operand: Option<&str> = None;
    for &w in post_verb {
        match word_literal(ast, w) {
            Some(lit) if lit.starts_with('-') => {} // a flag — not part of the entity
            Some(lit) => match operand {
                None => operand = Some(lit), // the first literal operand
                Some(_) => return None,      // a second ⇒ multi-entity ⊤
            },
            None => return None, // a non-literal operand (`$dyn`) ⇒ unknown cell ⇒ ⊤
        }
    }
    Some(match operand {
        Some(lit) => EntityRef::Operand(OpaqueToken(interner.intern(lit))),
        None => EntityRef::Singleton, // genuinely nullary: no operand survived, no ⊤ leaked
    })
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

/// Classify every `Command` node for the skip decision: look up each command's
/// effect, then a forward reaching-defs pass tells us, per establishing command,
/// whether its fact is ambient. An establish is only offered as
/// `EstablishAmbient` when its reaching-context is *trustworthy* — reachable from
/// entry AND under a converged solve; otherwise it folds to the safe `MustRun`
/// (find-A/find-B). Deterministic; never panics.
#[must_use]
pub fn classify(
    cfg: &Cfg,
    ast: &Ast,
    idx: &KindIndex,
    interner: &mut Interner,
) -> Vec<(CfgNodeId, SkipClass)> {
    let n = cfg.node_count();
    // Precompute each node's effect once (interning happens here, with &mut).
    let effects: Vec<CommandEffect> = (0..n)
        .map(|i| {
            let id = CfgNodeId(i as u32);
            match cfg.node(id).kind {
                CfgNodeKind::Command => command_effect(ast, idx, interner, cfg.node(id).ast),
                // An unmodeled construct may mutate anything ⇒ ⊤.
                CfgNodeKind::Top => CommandEffect::Opaque,
                _ => CommandEffect::Pure,
            }
        })
        .collect();

    // Forward reaching-defs: out = in ⊔ gen(node).
    let reach = solve(
        cfg,
        Direction::Forward,
        |i, incoming: &Reach| match &effects[i] {
            CommandEffect::Establishes(f) | CommandEffect::Kills(f) => incoming.with(*f),
            CommandEffect::Opaque => incoming.join(&Reach::Top),
            CommandEffect::Pure => incoming.clone(),
        },
    );
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
    for (i, effect) in effects.iter().enumerate() {
        let id = CfgNodeId(i as u32);
        // Only genuinely-runnable command leaves are plan/apply units. A command
        // inside a `$( … )` substitution body is effect-bearing (it stayed in the
        // reaching-defs above, so its mutations still poison/establish) but is NOT
        // a leaf (find-cli-1, the dn-3 leaf-seam).
        if cfg.node(id).kind != CfgNodeKind::Command || cfg.is_expansion_internal(id) {
            continue;
        }
        let class = match effect {
            // Only a reachable establish under a converged solve is reasoned about;
            // every other case — opaque/pure/kill, an unreachable establish, or a
            // non-converged solve — folds to MustRun, always the run-it side.
            CommandEffect::Establishes(f) if trust_reach && reachable[i] => {
                if reach.states[i].mutated(f) {
                    SkipClass::EstablishWritten(*f)
                } else {
                    SkipClass::EstablishAmbient(*f)
                }
            }
            _ => SkipClass::MustRun,
        };
        out.push((id, class));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg;
    use dorc_core::{KindId, SelectorId};

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

    fn classify_src(src: &str, interner: &mut Interner, idx: &KindIndex) -> Vec<SkipClass> {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        classify(&built.value, &parsed.value, idx, interner)
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
        assert!(!classes
            .iter()
            .any(|c| matches!(c, SkipClass::EstablishAmbient(_))));
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
        assert!(classes
            .iter()
            .any(|c| matches!(c, SkipClass::EstablishWritten(_))));
        assert!(!classes
            .iter()
            .any(|c| matches!(c, SkipClass::EstablishAmbient(_))));
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
        assert!(!classes
            .iter()
            .any(|c| matches!(c, SkipClass::EstablishAmbient(_))));
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
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(&mut i, &s, "nginx"))),
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
        let (mut i, idx, s) = package_setup();
        let eff = |src: &str, i: &mut Interner| {
            let parsed = dorc_syntax::parse(src);
            // the script's single top-level Simple is item 0 of the Script
            let NodeKind::Script { items } = &parsed.value.node(parsed.value.root()).kind else {
                panic!()
            };
            command_effect(&parsed.value, &idx, i, items[0])
        };
        // One operand ⇒ Operand cell; the flag `-y` is stripped.
        let nginx_cell = pkg_installed(&mut i, &s, "nginx");
        assert_eq!(
            eff("apt-get install -y nginx", &mut i),
            CommandEffect::Establishes(nginx_cell),
            "flag -y stripped ⇒ Operand(nginx)#installed"
        );
        // Zero non-flag operands on a MODELED verb ⇒ Singleton (the poison-wall fix);
        // a flag-only tail is still nullary.
        let pkg_index_fresh = CommandEffect::Establishes(FactKey {
            kind: s.package_index,
            entity: EntityRef::Singleton,
            selector: s.fresh,
        });
        assert_eq!(
            eff("apt-get update", &mut i),
            pkg_index_fresh,
            "nullary modeled verb ⇒ Singleton(package-index#fresh)"
        );
        assert_eq!(
            eff("apt-get update -y", &mut i),
            pkg_index_fresh,
            "flag-only tail stays nullary ⇒ Singleton"
        );
        // strain-8 (adversarial-crosscheck): a present-but-NON-LITERAL operand is an
        // UNKNOWN cell, NOT the singleton — else `install $PKG` would be wrongly
        // elidable (a priority-1 wrong-elision regression). It must stay ⊤ ⇒ run.
        assert_eq!(
            eff("apt-get install $PKG", &mut i),
            CommandEffect::Opaque,
            "non-literal operand ⇒ ⊤, not Singleton"
        );
        // Still ⊤ on the other ambiguous shapes.
        assert_eq!(
            eff("apt-get install nginx curl", &mut i),
            CommandEffect::Opaque,
            "multi-entity ⇒ ⊤"
        );
        assert_eq!(
            eff("$cmd install nginx", &mut i),
            CommandEffect::Opaque,
            "dynamic command name ⇒ ⊤"
        );
        assert_eq!(
            eff("apt-get autoclean", &mut i),
            CommandEffect::Opaque,
            "unknown verb ⇒ ⊤"
        );
    }
}
