//! The kind-keyed oracle families — resolvers (`<kind>__resolve`) and reach-functions
//! (`<kind>__disturbance_reaches_only`) — lifted per oracle file, with the confusability
//! enforcement that keeps a mis-keyed one LOUD rather than a silent dud.
//!
//! This sits on the loom seam (`lib-target-is-a-loom-seam`) because both checks are pure functions
//! of already-read oracle SOURCE: no path resolution, no clock, no host. That is what lets a
//! defining case for one of their four codes fire the run's OWN check rather than a second
//! implementation of it (`289:rul-worldless-route-honest-trigger`) — the binary and the harness
//! call the same [`build_kind_resolvers`] / [`build_kind_reaches`].

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::diag::{
    Diag, DiagCode, ReachesConflict, ReachesProviderCollision, ResolverConflict,
    ResolverProviderCollision,
};
use dorc_core::{Interner, Symbol};

/// A kind-keyed lift's product with every diagnostic it raised held as DATA — nothing writes fd 2
/// from inside the lift (`io-at-edges-only`; it is also what keeps a unit test driving one of these
/// from painting a red caret frame across a green run).
///
/// Two groups, because they frame against different sources: `lift` is the per-file lift's own
/// diagnostics, which the report seat frames spanlessly as it always has, while `confusability` is
/// keyed by the oracle file each diagnostic's caret resolves against (`law-lineno-identity`).
/// `run` reports them in that order, at the point the lift used to print them.
#[derive(Debug)]
pub struct KindLift<T> {
    /// The kept lift itself.
    pub value: T,
    /// The per-file lift's own diagnostics, framed spanlessly.
    pub lift: Vec<Diag>,
    /// The confusability findings, keyed by the oracle-file index their caret resolves against.
    pub confusability: BTreeMap<usize, Vec<Diag>>,
}

/// The per-KIND resolvers (24F §3, corr-kind-keying §10): `<kind>.resolve()` funcdefs lifted per
/// oracle file, combined with CONFUSABILITY enforcement. Resolvers are a SECOND family keyed by KIND
/// (the kind-owner holds the nouns — 23M contribution-vs-identity), NOT per-command role-siblings;
/// the engine looks one up by a coordinate's kind symbol, never its provider.
#[derive(Debug)]
pub struct KindResolvers {
    /// Per-file resolver sets (indexed by `by_kind`).
    sets: Vec<dorc_oracle::resolve::ResolverSet>,
    /// The kept, non-conflicting map from a RAW coordinate kind to `(file-index, munged-base
    /// symbol)`. Kind-keyed funcdefs are NAMED by the kind's forward-munge (`sm_dorc_Package__resolve`),
    /// so the inner [`ResolverSet`] is keyed by the munged base; this map re-keys to the RAW kind a
    /// coordinate carries (`flag-forward-munge-keying`), so a lookup by `coord.kind()` finds the
    /// funcdef named by that kind's munge. A kind ABSENT here is resolver-LESS (the token-equality
    /// floor) — never declared, or REFUSED for a cross-file duplicate.
    by_kind: BTreeMap<Symbol, (usize, Symbol)>,
}

impl KindResolvers {
    /// The resolver-bearing RAW kinds (the engine marks each; a coordinate of such a kind that fails
    /// to resolve degrades to may-alias, §3a).
    pub fn resolver_kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.by_kind.keys().copied()
    }

    /// The `(file-index, resolver funcdef)` for a RAW coordinate kind, if it has a kept resolver.
    #[must_use]
    pub fn get(&self, kind: Symbol) -> Option<(usize, &dorc_oracle::predict::Predict)> {
        let (idx, base) = *self.by_kind.get(&kind)?;
        self.sets.get(idx)?.get(base).map(|p| (idx, p))
    }
}

/// Lift the per-kind resolvers + ENFORCE confusability (24F §3 / corr-kind-keying §10 — a LOUD
/// diagnostic, never a silent dud). Two checks: (1) at-most-one-resolver-per-kind — two files
/// declaring `<kind>.resolve()` for the SAME kind ⇒ REFUSE BOTH (the kind stays resolver-less) + an
/// error; (2) a resolver keyed to a name matching a known PROVIDER (a lifted predict/touches
/// command) ⇒ a WARNING (the exact mis-keying the brief itself made — `apt-get.resolve()` would mint
/// identity for a "kind" no coordinate uses). `inv-referent-agnostic`: names compared as interned
/// symbols/strings, never decoded.
pub fn build_kind_resolvers(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    coord_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
) -> KindLift<KindResolvers> {
    use dorc_oracle::resolve::ResolverSet;
    use dorc_oracle::to_funcname_segment;

    let mut lift = Vec::new();
    let sets: Vec<ResolverSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = ResolverSet::lift(interner, src);
            lift.extend(lifted.diags);
            lifted.value
        })
        .collect();

    // Every (kind, file-index) declaration, grouped by kind (the same kind in ≥2 files is a conflict).
    let mut per_kind: BTreeMap<Symbol, Vec<usize>> = BTreeMap::new();
    for (idx, set) in sets.iter().enumerate() {
        for kind in set.kinds() {
            per_kind.entry(kind).or_default().push(idx);
        }
    }

    // The known PROVIDER names, FORWARD-MUNGED into NAME space (`flag-forward-munge-keying`: a
    // kind-keyed resolver interns its base by the kind's forward-munge, so the collision compares in
    // the same NAME space the funcdefs live in) — a resolver whose kind munges to a provider's is the
    // mis-keying we warn on.
    let mut providers: BTreeSet<String> = BTreeSet::new();
    for cs in checks {
        for p in cs.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }
    for (_, ts) in touches_paired {
        for p in ts.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }

    let mut diags_by_file: BTreeMap<usize, Vec<Diag>> = BTreeMap::new();
    let mut base_to_idx: BTreeMap<Symbol, usize> = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        // The diagnostic points at the FIRST declaring file's `<kind>__resolve` funcdef name
        // (`aid-caret-span-precision`); the file index carries its `law-lineno-identity` space.
        let anchor = files
            .first()
            .and_then(|&idx| Some((idx, sets.get(idx)?.get(kind)?.name_span)));
        if files.len() > 1 {
            if let Some((idx, span)) = anchor {
                diags_by_file.entry(idx).or_default().push(Diag::new(
                    DiagCode::ResolverConflict(ResolverConflict {
                        kind: name.clone(),
                        count: files.len(),
                    }),
                    span,
                ));
            }
            continue; // refuse both ⇒ resolver-less
        }
        if providers.contains(&name)
            && let Some((idx, span)) = anchor
        {
            // Kept (it may legitimately match a kind of the same name); the warning surfaces the risk.
            diags_by_file.entry(idx).or_default().push(Diag::new(
                DiagCode::ResolverProviderCollision(ResolverProviderCollision {
                    name: name.clone(),
                }),
                span,
            ));
        }
        if let Some(&idx) = files.first() {
            base_to_idx.insert(kind, idx);
        }
    }
    let by_kind = rekey_to_raw_kinds(&base_to_idx, coord_kinds, interner);
    KindLift {
        value: KindResolvers { sets, by_kind },
        lift,
        confusability: diags_by_file,
    }
}

/// Re-key a kind-keyed `munged-base → file-index` map to the RAW coordinate kinds
/// (`flag-forward-munge-keying`). A raw coord kind K maps to `(idx, munged-base)` iff its
/// forward-munge is a kept base. Shared by [`build_kind_resolvers`] and [`build_kind_reaches`].
fn rekey_to_raw_kinds(
    base_to_idx: &BTreeMap<Symbol, usize>,
    coord_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
) -> BTreeMap<Symbol, (usize, Symbol)> {
    let mut by_kind = BTreeMap::new();
    for &raw in coord_kinds {
        let munged_text = dorc_oracle::to_funcname_segment(interner.resolve(raw));
        let base = interner.intern(&munged_text);
        if let Some(&idx) = base_to_idx.get(&base) {
            by_kind.insert(raw, (idx, base));
        }
    }
    by_kind
}

/// The per-KIND reach-functions (24G §4): `<kind>.reaches()` funcdefs lifted per oracle file, with
/// CONFUSABILITY enforcement — kind-keyed exactly like the resolvers ([`KindResolvers`], corr-kind-keying
/// §10). The engine expands a footprint coord through the reach-function keyed by the coord's kind.
#[derive(Debug)]
pub struct KindReaches {
    /// Per-file reach sets (indexed by `by_kind`).
    sets: Vec<dorc_oracle::reaches::ReachesSet>,
    /// The kept, non-conflicting map from a RAW coordinate kind to `(file-index, munged-base symbol)`
    /// — re-keyed from the funcdef's munged base to the raw kind coords carry
    /// (`flag-forward-munge-keying`; see [`KindResolvers::by_kind`]). A kind ABSENT here is reach-LESS
    /// (its footprints never expand) — never declared, or REFUSED for a cross-file duplicate.
    by_kind: BTreeMap<Symbol, (usize, Symbol)>,
}

impl KindReaches {
    /// The reach-bearing RAW kinds (the engine expands every footprint coord of such a kind).
    pub fn reach_kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.by_kind.keys().copied()
    }

    /// The `(file-index, reaches funcdef)` for a RAW coordinate kind, if it has a kept reach-function.
    #[must_use]
    pub fn get(&self, kind: Symbol) -> Option<(usize, &dorc_oracle::predict::Predict)> {
        let (idx, base) = *self.by_kind.get(&kind)?;
        self.sets.get(idx)?.get(base).map(|p| (idx, p))
    }
}

/// Lift the per-kind reach-functions + ENFORCE confusability (24G §4, kind-keyed like the resolver —
/// a LOUD diagnostic, never a silent dud). Two checks, mirroring [`build_kind_resolvers`]: (1)
/// at-most-one-reaches-per-kind — two files declaring `<kind>.reaches()` for the SAME kind ⇒ REFUSE
/// BOTH (the kind stays reach-less) + an error; (2) a reaches keyed to a name matching a known
/// PROVIDER ⇒ a WARNING (the reaches is keyed by KIND, not command). `inv-referent-agnostic`: names
/// compared as interned strings, never decoded.
pub fn build_kind_reaches(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    coord_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
) -> KindLift<KindReaches> {
    use dorc_oracle::reaches::ReachesSet;
    use dorc_oracle::to_funcname_segment;

    let mut lift = Vec::new();
    let sets: Vec<ReachesSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = ReachesSet::lift(interner, src);
            lift.extend(lifted.diags);
            lifted.value
        })
        .collect();

    let mut per_kind: BTreeMap<Symbol, Vec<usize>> = BTreeMap::new();
    for (idx, set) in sets.iter().enumerate() {
        for kind in set.kinds() {
            per_kind.entry(kind).or_default().push(idx);
        }
    }

    let mut providers: BTreeSet<String> = BTreeSet::new();
    for cs in checks {
        for p in cs.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }
    for (_, ts) in touches_paired {
        for p in ts.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }

    let mut diags_by_file: BTreeMap<usize, Vec<Diag>> = BTreeMap::new();
    let mut base_to_idx: BTreeMap<Symbol, usize> = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        // Point at the FIRST declaring file's `<kind>__reaches` funcdef name (`aid-caret-span-precision`).
        let anchor = files
            .first()
            .and_then(|&idx| Some((idx, sets.get(idx)?.get(kind)?.name_span)));
        if files.len() > 1 {
            if let Some((idx, span)) = anchor {
                diags_by_file.entry(idx).or_default().push(Diag::new(
                    DiagCode::ReachesConflict(ReachesConflict {
                        kind: name.clone(),
                        count: files.len(),
                    }),
                    span,
                ));
            }
            continue;
        }
        if providers.contains(&name)
            && let Some((idx, span)) = anchor
        {
            diags_by_file.entry(idx).or_default().push(Diag::new(
                DiagCode::ReachesProviderCollision(ReachesProviderCollision { name: name.clone() }),
                span,
            ));
        }
        if let Some(&idx) = files.first() {
            base_to_idx.insert(kind, idx);
        }
    }
    let by_kind = rekey_to_raw_kinds(&base_to_idx, coord_kinds, interner);
    KindLift {
        value: KindReaches { sets, by_kind },
        lift,
        confusability: diags_by_file,
    }
}

/// Every confusability diagnostic an oracle SET raises on its own — the two kind-keyed families'
/// conflict + provider-collision checks, in the order `run` reports them (resolvers, then reaches).
///
/// Book-free by construction: the checks read only the oracle sources and the already-lifted check
/// sets, so this is the whole of what the four codes can say about a world with no probe records
/// and no survival flag. `coord_kinds` is empty here on purpose — it re-keys the KEPT map for the
/// probe lanes and never gates a diagnostic, so a caller that wants only the diagnostics owes no
/// book. `touches_paired` likewise only widens the known-provider set, and is populated only under
/// `--risk-faultless-skips`.
#[must_use]
pub fn confusability_diagnostics(
    checks: &[dorc_oracle::predict::PredictSet],
    oracle_srcs: &[&str],
    interner: &mut Interner,
) -> Vec<Diag> {
    let owned: Vec<String> = oracle_srcs.iter().map(|src| (*src).to_owned()).collect();
    let coord_kinds = BTreeSet::new();
    let mut out = Vec::new();
    for lift in [
        build_kind_resolvers(&owned, checks, &[], &coord_kinds, interner).into_diags(),
        build_kind_reaches(&owned, checks, &[], &coord_kinds, interner).into_diags(),
    ] {
        out.extend(lift);
    }
    out
}

impl<T> KindLift<T> {
    /// This lift's diagnostics in report order: the per-file lift's own, then the confusability
    /// findings by oracle-file index.
    fn into_diags(self) -> Vec<Diag> {
        let mut out = self.lift;
        out.extend(self.confusability.into_values().flatten());
        out
    }
}
