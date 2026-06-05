//! `dorc-oracle` — lifts the **fact-centric** oracle contract (design note 162 v2)
//! out of plain sh into the analyzer's internal *kind index*.
//!
//! An oracle file is ordinary sh that declares three things, which we lift
//! statically (never by running it):
//!
//! ```sh
//! oracle_kind=package                          # the named kind this file serves
//! oracle_probe_package() { … dpkg-query … }    # a READ-ONLY check: does package:$1 hold?
//! oracle_effect apt-get install establish      # `apt-get install <pkgs>` establishes package
//! oracle_effect apt-get purge   kill           # `apt-get purge   <pkgs>` removes it
//! ```
//!
//! From a book's bare `apt-get install -y nginx`, the analyzer looks up the
//! effect `(apt-get, install) → (package, Establish)`, and to decide whether it
//! is already done it ships the `package` kind's probe. The kind name is the only
//! cross-oracle anchor (apt's `package` ≡ yum's `package`); it is never decoded
//! for meaning.
//!
//! This crate is *lightly typed on purpose* — it is pure declaration-extraction,
//! with no soundness-orientation in play. The heavy orientation-locks
//! (`May`/`Must`, phase-typed verdicts, the skip witness) live downstream in the
//! analyses that consume this, where a wrong direction is catastrophic
//! (note 165). Here, a wrong lift is a missing/garbled `ProviderDecl`, caught by
//! the consumer treating an absent effect as ⊤ (run it), never a silent wrong-skip.

#![forbid(unsafe_code)]

use dorc_core::{Carrier, DiagCode, Diagnostic, Interner, KindId, ProviderId, Symbol};
use std::collections::BTreeMap;

/// What a `(provider, verb)` invocation does to a fact of its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Makes the fact hold (`apt-get install` ⇒ `package:X` present).
    Establish,
    /// Makes the fact not hold (`apt-get purge` ⇒ `package:X` absent).
    Kill,
}

/// A read-only fact-probe for one kind: shippable sh that observes whether
/// `kind:entity` holds for an entity passed as `$1`. By contract it is
/// three-outcome (exit `0` = holds, `1` = absent, `2` = can't-tell) and must not
/// mutate — the latter is the consumer's/runner's obligation to enforce, not this
/// crate's (note 162 DP-4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactProbe {
    pub kind: KindId,
    /// The probe function's sh body, lifted verbatim for shipping to the host.
    pub body: String,
}

/// The analyzer-internal kind index — the dn-1 artifact. A 3-place relation
/// (kind, provider, verb→effect), *not* a 1-place naming convention (which would
/// clobber when two providers coexist; note 162 F-3). Built by [`lift`], queried
/// by the analyses.
#[derive(Debug, Clone, Default)]
pub struct KindIndex {
    /// kind → how to observe it (one probe per kind).
    probes: BTreeMap<KindId, FactProbe>,
    /// (provider, verb) → (kind, polarity). Accumulating + clobber-free: many
    /// providers and many verbs coexist (`apt-get install` vs `apt-get purge` vs
    /// `dpkg -i`), unlike a single `oracle_verb`.
    effects: BTreeMap<(ProviderId, Symbol), (KindId, Polarity)>,
}

impl KindIndex {
    /// Record a kind's probe. Last-writer-wins on a duplicate kind (the lifter
    /// should diagnose duplicates; this stays total).
    pub fn add_probe(&mut self, probe: FactProbe) {
        self.probes.insert(probe.kind, probe);
    }

    /// Record that `provider verb …` has `polarity` on `kind`.
    pub fn add_effect(&mut self, provider: ProviderId, verb: Symbol, kind: KindId, polarity: Polarity) {
        self.effects.insert((provider, verb), (kind, polarity));
    }

    /// The read-only probe for `kind`, if any oracle declared one.
    #[must_use]
    pub fn probe_for(&self, kind: KindId) -> Option<&FactProbe> {
        self.probes.get(&kind)
    }

    /// What a book's `provider verb …` does, if any oracle declared it. `None`
    /// means "no oracle knows this" → the consumer treats the command as ⊤ (run).
    #[must_use]
    pub fn effect_of(&self, provider: ProviderId, verb: Symbol) -> Option<(KindId, Polarity)> {
        self.effects.get(&(provider, verb)).copied()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.probes.is_empty() && self.effects.is_empty()
    }
}

/// Lift a set of oracle sh sources into the kind index, interning kind/provider/
/// verb names through the shared `interner` (so they match the names the book
/// analysis interns). Never panics (`inv-no-throw`): a non-literal anchor, a
/// missing probe, or a top-level mutator in an oracle file yields a `Diagnostic`,
/// not a crash, and that file simply contributes no (or partial) declarations.
///
/// NOTE: stub. The lifting walk over each parsed oracle AST is delegated; this
/// returns an empty index so the workspace stays green until it lands.
#[must_use]
pub fn lift(_interner: &mut Interner, _oracle_sources: &[&str]) -> Carrier<KindIndex> {
    Carrier::new(
        KindIndex::default(),
        vec![Diagnostic::warning(
            DiagCode("oracle-lift-unimplemented"),
            None,
            "oracle lift not yet implemented (stub)",
        )],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_lookups_round_trip() {
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");

        let mut idx = KindIndex::default();
        idx.add_probe(FactProbe { kind: package, body: "dpkg-query -W \"$1\"".into() });
        idx.add_effect(apt, install, package, Polarity::Establish);

        assert_eq!(idx.effect_of(apt, install), Some((package, Polarity::Establish)));
        assert!(idx.probe_for(package).is_some());
        // An unknown (provider, verb) is None ⇒ consumer must run it (⊤), never skip.
        let purge = interner.intern("purge");
        assert_eq!(idx.effect_of(apt, purge), None);
    }

    #[test]
    fn lift_stub_is_total() {
        let mut interner = Interner::default();
        let out = lift(&mut interner, &["oracle_kind=package\n"]);
        assert!(out.value.is_empty());
        assert!(!out.diags.is_empty());
    }
}
