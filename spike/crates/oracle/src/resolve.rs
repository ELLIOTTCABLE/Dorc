//! `resolve` — the identity CANONICALIZER lift (24F §3, the resid-aliasing closure; the FOURTH
//! role-sibling). A kind-OWNER may ship `<kind>.resolve()`: a body invoked host-side per coordinate
//! with the entity text, printing the entity's CANONICAL form on stdout (a `dpkg-query -W`
//! provides-resolution, a `realpath -m` symlink-resolution). The engine canonicalizes BOTH footprint
//! and backing coordinates through it BEFORE `disjoint`, closing the two-names-one-referent
//! under-execute (`nginx`/`nginx-full`; `/etc/nginx` vs a symlinked path).
//!
//! # Keyed by KIND, not the command word (the one structural difference from its siblings)
//!
//! `predict()`/`touches()`/`is_converged()` are keyed by the COMMAND word (`apt-get`); a resolver is
//! keyed by the KIND (23M contribution-vs-identity: the kind-OWNER holds entity-identity — authority
//! over the *nouns*). So `<kind>.resolve` interns its name as the SAME [`Symbol`] the coordinate's
//! `KindId` wraps (the vocabulary fence — one interned universe): `package__resolve()` ⇒ the `package`
//! kind's resolver. The engine looks up a resolver by a coordinate's kind symbol, never its provider.
//!
//! # Host-run ONLY — no static evaluator (24F §3, fork-4A rails)
//!
//! Identity resolution NEEDS the host (dpkg/realpath), so — unlike `touches()`, which has a static
//! fixed-footprint path — a resolver ALWAYS ships to the probe lane strip-only
//! ([`crate::predict::strip_resolve`]) and runs read-only per coordinate. It rides the Stage-4 rails
//! exactly: the SAME structural self-vouch (authoring IS the vouch — no closure-check), the SAME
//! rc-127 mocks net as the live guarantee, the SAME one-flag-wide professed caveat. NO new trust
//! edge (a resolver sits at the same tier as a host-run `predict()`/`touches()`).
//!
//! `inv-referent-agnostic`: this module lifts + ships the owner's resolver; the ENGINE never decodes
//! an entity's text — it interns the resolver's OUTPUT as an opaque canonical token and compares
//! canonical forms as symbols. The OWNER decodes; the engine plumbs (24F reconciliation).

use dorc_core::{Carrier, Interner, Symbol};

use crate::predict::{Predict, PredictSet, lift_resolvers};

/// The set of `<kind>.resolve()` funcdefs lifted from one oracle file, keyed by KIND. Reuses the
/// predict dialect AST ([`Predict`]) — a resolver funcdef has the identical body grammar; only its
/// name-suffix (`.resolve`) and its purpose (print a canonical form) differ.
#[derive(Debug, Clone, Default)]
pub struct ResolverSet(PredictSet);

impl ResolverSet {
    /// Lift every `<kind>.resolve` / `<kind>__resolve` funcdef in `src`. Fail-soft (`inv-no-throw`)
    /// and deterministic (`inv-determinism`) — the same contract as [`crate::predict::lift_predicts`],
    /// routed through the shared role-parametrized parser.
    #[must_use]
    pub fn lift(interner: &mut Interner, src: &str) -> Carrier<Self> {
        lift_resolvers(interner, src).map(Self)
    }

    /// The resolver funcdef for a KIND, if the file declared one. `kind` is the SAME interned symbol
    /// a coordinate's `KindId` wraps (the vocabulary fence — one interned universe).
    #[must_use]
    pub fn get(&self, kind: Symbol) -> Option<&Predict> {
        self.0.get(kind)
    }

    /// Kinds with a lifted resolver, in deterministic order (the engine marks each resolver-BEARING;
    /// a coordinate in such a kind that fails to resolve degrades to may-alias, 24F §3a).
    pub fn kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.0.providers()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `<kind>.resolve` funcdef lifts, keyed by the KIND symbol (not a command word). The
    /// vocabulary fence: the lifted key is the SAME symbol `KindId(intern("package"))` wraps.
    #[test]
    fn resolver_lifts_keyed_by_kind() {
        let mut i = Interner::default();
        let src = "\
package__resolve() {
   dpkg-query -W -f '${Package}\\n' -- \"$1\" 2>/dev/null || printf '%s\\n' \"$1\"
}";
        let set = ResolverSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let kind = i.intern("package");
        assert!(
            set.value.get(kind).is_some(),
            "the package resolver is keyed by the kind symbol"
        );
        assert_eq!(set.value.kinds().count(), 1, "exactly one resolver kind");
    }

    /// A resolver's body reaching a host tool (`dpkg-query`) lifts fine — the resolver is host-run,
    /// never statically traced, so a body the predict/touches static tracers would ⊤ on is a normal
    /// resolver (it ships strip-only and runs on the host).
    #[test]
    fn realpath_style_resolver_lifts() {
        let mut i = Interner::default();
        let src = "fs__resolve() { realpath -m -- \"$1\"; }";
        let set = ResolverSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        assert!(
            set.value.get(i.intern("fs")).is_some(),
            "the fs resolver lifts"
        );
    }

    /// A file with predict/touches/verdict siblings but NO resolver yields an empty set — a
    /// resolver-less kind (the token-equality floor, per-kind gradual enhancement).
    #[test]
    fn no_resolver_is_empty_set() {
        let mut i = Interner::default();
        let src = "apt_get__disturbs() { printf 'package:%s\\n' \"$1\"; }";
        let set = ResolverSet::lift(&mut i, src);
        assert!(set.diags.is_empty());
        assert!(
            set.value.is_empty(),
            "no .resolve funcdef ⇒ empty resolver set"
        );
    }
}
