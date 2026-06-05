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
use dorc_core::{AstId, Interner, KindId, OpaqueToken, ProviderId};
use dorc_oracle::{KindIndex, Polarity};
use dorc_syntax::ast::{Ast, NodeKind, WordPart};
use std::collections::BTreeSet;

/// A system-state fact as a dataflow key: a named kind + an opaque entity. It
/// carries NO source span — two commands establishing `package:nginx` from
/// different lines denote the *same* fact for reaching-defs (provenance is the
/// node's, tracked separately).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FactKey {
    pub kind: KindId,
    pub entity: OpaqueToken,
}

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
/// (`Opaque`) on ANY uncertainty: dynamic command word/verb, no oracle entry, or
/// not-exactly-one literal non-flag operand.
///
/// The crude flag-strip ("operand = a literal not starting with `-`") and the
/// single-entity restriction are a known coarse spot (note 162 O-3/O-4): sound
/// (errs to `Opaque`) but value-eroding, and it mis-handles pre-verb flags
/// (`apt-get -t x install …`) and attached values (`-oKey=Val`). Precise
/// per-provider flag grammars are deferred.
pub fn command_effect(ast: &Ast, idx: &KindIndex, interner: &mut Interner, simple: AstId) -> CommandEffect {
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
    let provider = ProviderId(interner.intern(provider_s));
    let Some(verb_s) = words.get(1).and_then(|&w| word_literal(ast, w)) else {
        return CommandEffect::Opaque; // no static verb ⇒ no effect to look up
    };
    let verb = interner.intern(verb_s);
    let Some((kind, polarity)) = idx.effect_of(provider, verb) else {
        return CommandEffect::Opaque; // unknown (provider, verb)
    };
    // Entity = the single literal, non-flag operand after the verb.
    let mut operands = words[2..].iter().filter_map(|&w| {
        let lit = word_literal(ast, w)?;
        if lit.starts_with('-') {
            None
        } else {
            Some(lit)
        }
    });
    let (Some(entity_s), None) = (operands.next(), operands.next()) else {
        return CommandEffect::Opaque; // zero / multiple / non-literal operands
    };
    let fact = FactKey { kind, entity: OpaqueToken(interner.intern(entity_s)) };
    match polarity {
        Polarity::Establish => CommandEffect::Establishes(fact),
        Polarity::Kill => CommandEffect::Kills(fact),
    }
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
    /// Not an elidable establish (opaque, pure, kill, or unrecognized) ⇒ run.
    MustRun,
    /// Establishes `fact`, ambient here (no upstream mutation) ⇒ probe the host.
    EstablishAmbient(FactKey),
    /// Establishes `fact`, but the fact was mutated upstream in-script ⇒ the
    /// resting probe is not authoritative ⇒ run (or reason in-script; conservatively
    /// run). The `purge X; … install X` case.
    EstablishWritten(FactKey),
}

/// Classify every `Command` node for the skip decision: look up each command's
/// effect, then a forward reaching-defs pass tells us, per establishing command,
/// whether its fact is ambient. Deterministic; never panics.
#[must_use]
pub fn classify(cfg: &Cfg, ast: &Ast, idx: &KindIndex, interner: &mut Interner) -> Vec<(CfgNodeId, SkipClass)> {
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
    let reach = solve(cfg, Direction::Forward, |i, incoming: &Reach| match &effects[i] {
        CommandEffect::Establishes(f) | CommandEffect::Kills(f) => incoming.with(*f),
        CommandEffect::Opaque => incoming.join(&Reach::Top),
        CommandEffect::Pure => incoming.clone(),
    });
    debug_assert!(reach.converged, "reaching-defs must converge (finite fact set)");

    let mut out = Vec::new();
    for (i, effect) in effects.iter().enumerate() {
        let id = CfgNodeId(i as u32);
        if cfg.node(id).kind != CfgNodeKind::Command {
            continue;
        }
        let class = match effect {
            CommandEffect::Establishes(f) if reach.states[i].mutated(f) => SkipClass::EstablishWritten(*f),
            CommandEffect::Establishes(f) => SkipClass::EstablishAmbient(*f),
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

    /// Build (interner, index) with the package oracle's effects, plus the
    /// pre-interned kind/provider needed to assert facts.
    fn package_setup() -> (Interner, KindIndex, KindId) {
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");
        let purge = interner.intern("purge");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, Polarity::Establish);
        idx.add_effect(apt, purge, package, Polarity::Kill);
        (interner, idx, package)
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
        let (mut i, idx, package) = package_setup();
        let nginx = OpaqueToken(i.intern("nginx"));
        let classes = classify_src("apt-get install nginx", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::EstablishAmbient(FactKey { kind: package, entity: nginx })]
        );
    }

    #[test]
    fn upstream_purge_makes_install_written() {
        // Why (note 162 O-1 / break-10, THE wrong-skip): an upstream same-run kill
        // means the resting probe is stale — the install must NOT be treated as
        // ambient/skippable.
        let (mut i, idx, package) = package_setup();
        let nginx = OpaqueToken(i.intern("nginx"));
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        // purge ⇒ MustRun (a kill, not an elidable establish); install ⇒ Written.
        assert!(classes.contains(&SkipClass::EstablishWritten(FactKey { kind: package, entity: nginx })));
        assert!(!classes
            .iter()
            .any(|c| matches!(c, SkipClass::EstablishAmbient(_))));
    }

    #[test]
    fn opaque_upstream_poisons_ambientness() {
        // Why (note 162 O-3, the precision COST being surfaced): an unrecognized
        // command (no oracle) is Opaque ⇒ ⊤ ⇒ it conservatively poisons every
        // downstream fact's ambient-ness. Sound but value-eroding — this test
        // documents the cost so we feel it.
        let (mut i, idx, _package) = package_setup();
        let classes = classify_src("ufw allow 80/tcp\napt-get install nginx", &mut i, &idx);
        assert!(classes.iter().any(|c| matches!(c, SkipClass::EstablishWritten(_))));
        assert!(!classes
            .iter()
            .any(|c| matches!(c, SkipClass::EstablishAmbient(_))));
    }

    #[test]
    fn command_effect_is_top_conservative() {
        let (mut i, idx, package) = package_setup();
        let nginx = OpaqueToken(i.intern("nginx"));
        let eff = |src: &str, i: &mut Interner| {
            let parsed = dorc_syntax::parse(src);
            // the script's single top-level Simple is item 0 of the Script
            let NodeKind::Script { items } = &parsed.value.node(parsed.value.root()).kind else {
                panic!()
            };
            command_effect(&parsed.value, &idx, i, items[0])
        };
        assert_eq!(eff("apt-get install -y nginx", &mut i), CommandEffect::Establishes(FactKey { kind: package, entity: nginx }), "flag -y stripped");
        assert_eq!(eff("apt-get install nginx curl", &mut i), CommandEffect::Opaque, "multi-entity ⇒ ⊤");
        assert_eq!(eff("$cmd install nginx", &mut i), CommandEffect::Opaque, "dynamic command name ⇒ ⊤");
        assert_eq!(eff("apt-get update", &mut i), CommandEffect::Opaque, "unknown verb ⇒ ⊤");
    }
}
