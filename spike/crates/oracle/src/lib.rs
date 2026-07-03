//! `dorc-oracle` — lifts the oracle contract out of plain sh into the analyzer's
//! internal *kind index*.
//!
//! An oracle file is ordinary sh whose `<provider>__predict` function IS the contract
//! (23D §1 — the check is the oracle). The analyzer reads the effect-map off the check
//! body's own control flow, never by running it:
//!
//! ```sh
//! apt_get__predict() {
//!    verb=$1; shift
//!    pkg : package = "$1"                                    # the kind annotation
//!    case $verb in
//!       install) dpkg-query -W "$pkg" : package:"$pkg".installed ;;   # establish
//!       purge)   dpkg-query -W "$pkg" : package:"$pkg".installed! ;;  # inverted
//!    esac
//! }
//! ```
//!
//! From a book's bare `apt-get install -y nginx`, the analyzer derives the effect
//! `(apt-get, install) → (package, #installed, Establish)` (the `case $verb` arm names the
//! verb, the inline `pkg : package` annotation names the kind, the trailing mark names the
//! selector + rc convention — see [`predict::derive_predict`]). The kind name is the only
//! cross-oracle anchor (apt's `package` ≡ yum's `package`); it is never decoded for meaning
//! (`inv-referent-agnostic`).
//!
//! This crate is *lightly typed on purpose* — it is pure declaration-extraction, with no
//! soundness-orientation in play. The heavy orientation-locks (`May`/`Must`, phase-typed
//! verdicts, the skip witness) live downstream in the analyses that consume this, where a
//! wrong direction is catastrophic (note 165). Here, a wrong lift is a missing/garbled
//! effect cell, caught by the consumer treating an absent effect as ⊤ (run it), never a
//! silent wrong-skip.

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; this crate-root expect
// ratchets away during the rebuild (an unfulfilled `expect` warns, so it
// self-removes as the seeded layer is replaced). It never relaxes the policy
// for new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::indexing_slicing,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use dorc_core::{Carrier, Interner, KindId, ProviderId, SelectorId, Symbol};
use std::collections::BTreeMap;

/// The command-keyed `check()` contract (19H §2 / 202 §1 face-check): a dedicated
/// parser for the constrained oracle-contract dialect plus a concrete evaluator that
/// traces a known argv through a check's argparse to its kind-annotation.
///
/// Round-20 input-side mechanism, wired in by task-W: `analysis::effect` threads a
/// book's value-flow through [`predict::evaluate`] (the oracle's own argparse) to its
/// inline kind-annotation — the real entity-resolution. [`derive_predict`](predict::derive_predict)
/// reads the same check body's `case $verb` arms + trailing marks to build the effect-map
/// [`lift`] indexes per `(provider, verb)`.
pub mod predict;

/// The at-most **footprint** lift (`provides-behavior` sub-shape 3; 24A §1b / 23M): the
/// `<provider>.touches()` role-sibling, lifted STATICALLY to the entity-coordinates a verb
/// mutates. Feeds the survival/disjointness tier (`dorc_plan`). Reuses the `predict` dialect.
pub mod touches;

/// The guard-**verdict** function lift (rul-role-split / rul24-vouch-is-verdict-authoring, 24A
/// §1c / 24D §3): the `<provider>.is_converged()` / `.is_diverged()` role-siblings. Authoring one
/// IS the vouch; this module decides STATICALLY whether a site's argv reaches a vouching path (the
/// judgment-tier license source), and the guard emitter ships the same body strip-only. Reuses the
/// `predict` dialect.
pub mod verdict;

/// How a `(provider, verb)` reads/writes a fact's OPAQUE boolean — the lifted
/// representation is [`ValueClaim`] (jc-polarity-vs-rc, FINAL — human 2026-07-02).
/// The former `Polarity{Establish, Kill, Query}` is RETIRED: no create/destroy axis
/// survives the lifted representation, ever. Re-exported at the crate root so consumers
/// name one type. `EstablishInverted` (the former `Kill`'s `!` mark) carries rc-inversion
/// only — no "kill" concept; a site reaching it classifies `MustRun` under the transitional
/// freeze (see `analysis::effect::cell_effect`).
pub use predict::ValueClaim;

/// One declared effect cell of a `(provider, verb)`: which `kind`, which `selector`
/// facet, and the [`ValueClaim`]. A `(provider, verb)` may declare **several** cells
/// (`us-effectmap`, note 205 §3: a multi-cell verb is real — `purge` kills
/// `#installed` and may dirty a `#config` cell). The wiring (`analysis::effect`)
/// treats each cell as written, in declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectCell {
    pub kind: KindId,
    pub selector: SelectorId,
    pub claim: ValueClaim,
}

/// A duplicate-effect conflict (`us-effectmap`, note 205 §3): a *second* effect for the
/// same `(provider, verb)` on the **same** `selector` cell. First-writer-wins (the
/// duplicate is dropped). A different *selector* for the same verb is NOT a conflict —
/// that is the legitimate multi-cell case ([`EffectCell`]).
#[derive(Debug, Clone, Copy)]
pub struct EffectConflict {
    pub provider: ProviderId,
    pub verb: Symbol,
    pub selector: SelectorId,
}

/// The analyzer-internal kind index — the dn-1 artifact. A 3-place relation
/// (kind, provider, verb→effect), *not* a 1-place naming convention (which would
/// clobber when two providers coexist; note 162 F-3). Built by [`lift`] from the check
/// bodies' derived effects, queried by the analyses.
#[derive(Debug, Clone, Default)]
pub struct KindIndex {
    /// (provider, verb) → the derived effect cells. Accumulating + clobber-free:
    /// many providers and many verbs coexist (`apt-get install` vs `apt-get purge`
    /// vs `dpkg -i`). The value is a **Vec** of [`EffectCell`]s (`us-effectmap`, note
    /// 205 §3): a verb may gate several cells. The `selector`
    /// (`#installed`/`#fresh`/`#enabled`/`#active`) is the per-entity facet
    /// (`an-per-entity-selector`, `notes/193` §4): `enable` and `start` target
    /// *different* selectors on the same `service` cell, so neither discharges the
    /// other. A **verbless** provider (`useradd`, `command -v`) keys on the ε-verb
    /// ([`empty_verb`]) — the check binds no verb, so the wiring looks up `(provider, ε)`
    /// (202 §2 / task-W §4).
    effects: BTreeMap<(ProviderId, Symbol), Vec<EffectCell>>,
}

impl KindIndex {
    /// Record that `provider verb …` has `claim` on `kind`'s `selector` cell.
    /// Returns `Some(EffectConflict)` if a cell on the **same** `(provider, verb,
    /// selector)` was already declared — first-writer-wins, the duplicate is
    /// dropped (`us-effectmap`, note 205 §3). A *different* selector for the same verb
    /// is appended (the legitimate multi-cell case).
    pub fn add_effect(
        &mut self,
        provider: ProviderId,
        verb: Symbol,
        kind: KindId,
        selector: SelectorId,
        claim: ValueClaim,
    ) -> Option<EffectConflict> {
        let cells = self.effects.entry((provider, verb)).or_default();
        if cells.iter().any(|c| c.selector == selector) {
            return Some(EffectConflict {
                provider,
                verb,
                selector,
            });
        }
        cells.push(EffectCell {
            kind,
            selector,
            claim,
        });
        None
    }

    /// The declared effect cells of a book's `provider verb …`, if any check
    /// declared it (the empty slice when not). Each cell is one `(kind, selector,
    /// claim)` the verb gates; the wiring treats each as written. A verbless
    /// command keys on `(provider, ε)` ([`empty_verb`]). An empty result means "no
    /// oracle knows this" → the consumer treats the command as ⊤ (run).
    #[must_use]
    pub fn effect_of(&self, provider: ProviderId, verb: Symbol) -> &[EffectCell] {
        self.effects
            .get(&(provider, verb))
            .map_or(&[], Vec::as_slice)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}

/// The ε-verb symbol: the effect-map key for a **verbless** provider (`useradd
/// deploy`, `command -v nginx` — the oracle's check binds no verb). Interned as the
/// empty string, which no real argv verb token can be, so it never collides with a
/// declared verb. The wiring maps a check's `verb: None` to this same symbol (202 §2 /
/// task-W §4 — one shared spelling, both sides).
#[must_use]
pub fn empty_verb(interner: &mut Interner) -> Symbol {
    interner.intern("")
}

/// Map an interned kind/selector name to its **function-name segment** form: `-` → `_`
/// (`package-index` ⇒ `package_index`). The inverse direction of
/// [`predict::map_provider_name`] (`_` → `-`), and the shared home of the
/// hyphen↔underscore convention on the *emit*/match side (the shipped `<provider>__predict`
/// wrapper name routes through here, so both sides agree).
///
/// **Lossy** in the same way `map_provider_name` is: a literal `_` in the name is
/// indistinguishable from a hyphen after the round-trip.
#[must_use]
pub fn to_funcname_segment(name: &str) -> String {
    name.replace('-', "_")
}

/// Lift a set of oracle sh sources into the kind index, interning kind/provider/verb
/// names through the shared `interner` (so they match the names the book analysis
/// interns). The effect-map is DERIVED from each oracle's `<provider>__predict` bodies
/// (23D §1 — the check is the oracle): [`predict::lift_predicts`] parses the dialect,
/// [`predict::derive_predict`] reads the `case $verb` arms + inline annotation + trailing
/// marks off each check into `(provider, verb) → (kind, selector, claim)` cells.
///
/// Never panics (`inv-no-throw`): `derive_predict` is total (a shape it cannot characterize
/// simply contributes no cell, the safe direction), and a check that fails to lift is a
/// per-function diagnostic surfaced by the caller's own [`predict::lift_predicts`] pass (the
/// cli reports check diagnostics separately), not a crash. Deterministic
/// (`inv-determinism`): sources are walked in argument order, the index is
/// `BTreeMap`-backed, and nothing here touches clock/RNG/IO.
#[must_use]
pub fn lift(interner: &mut Interner, oracle_sources: &[&str]) -> Carrier<KindIndex> {
    let mut out = Carrier::pure(KindIndex::default());
    for src in oracle_sources {
        // The check-lift's own diagnostics are reported by the caller's separate
        // `lift_predicts` pass (cli/coverage); here we consume only the parsed checks to
        // derive the effect-map, so a malformed check contributes no cells (safe).
        let checks = predict::lift_predicts(interner, src).value;
        for provider in checks.providers() {
            let Some(c) = checks.get(provider) else {
                continue;
            };
            let effects = predict::derive_predict(c);
            for e in effects {
                let verb = match e.verb {
                    Some(v) => interner.intern(&v),
                    None => empty_verb(interner),
                };
                let kind = KindId(interner.intern(&e.kind));
                let selector = SelectorId(interner.intern(&e.selector));
                // A duplicate cell cannot arise from a well-formed check (each verb-arm
                // names a distinct selector); drop a derived duplicate silently.
                out.value
                    .add_effect(ProviderId(provider), verb, kind, selector, e.claim);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Resolve a `(provider, verb)` effect through a fresh interner. Each lift gets
    /// its own interner; we re-intern the same names here to query, relying on the
    /// interner's determinism (equal text ⇒ equal symbol).
    fn effect(
        idx: &KindIndex,
        i: &mut Interner,
        provider: &str,
        verb: &str,
    ) -> Option<(KindId, SelectorId, ValueClaim)> {
        match idx.effect_of(ProviderId(i.intern(provider)), i.intern(verb)) {
            [] => None,
            [cell, ..] => Some((cell.kind, cell.selector, cell.claim)),
        }
    }

    #[test]
    fn index_lookups_round_trip() {
        // Pins the hand-built index API the consumer relies on (independent of lift).
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");
        let installed = SelectorId(interner.intern("installed"));

        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, ValueClaim::Establish);

        assert_eq!(
            idx.effect_of(apt, install),
            &[EffectCell {
                kind: package,
                selector: installed,
                claim: ValueClaim::Establish
            }]
        );
        // An unknown (provider, verb) is the empty slice ⇒ consumer must run it (⊤).
        let purge = interner.intern("purge");
        assert!(idx.effect_of(apt, purge).is_empty());
    }

    #[test]
    fn duplicate_cell_first_writer_wins() {
        // us-effectmap (note 205 §3): a second effect on the same (provider, verb,
        // selector) cell reports a conflict and is dropped; the first survives.
        let mut i = Interner::default();
        let apt = ProviderId(i.intern("apt-get"));
        let install = i.intern("install");
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));

        let mut idx = KindIndex::default();
        assert!(
            idx.add_effect(apt, install, package, installed, ValueClaim::Establish)
                .is_none()
        );
        // Second cell, same selector ⇒ conflict, dropped.
        assert!(
            idx.add_effect(
                apt,
                install,
                package,
                installed,
                ValueClaim::EstablishInverted
            )
            .is_some()
        );
        assert_eq!(
            effect(&idx, &mut i, "apt-get", "install"),
            Some((package, installed, ValueClaim::Establish)),
            "first-writer-wins"
        );
        // A different selector on the same verb is the legitimate multi-cell case.
        let configured = SelectorId(i.intern("configured"));
        assert!(
            idx.add_effect(apt, install, package, configured, ValueClaim::Establish)
                .is_none()
        );
        assert_eq!(idx.effect_of(apt, install).len(), 2);
    }

    #[test]
    fn lifts_the_package_fixture_cleanly() {
        // The acceptance fixture: a real, fully-formed oracle must lift (check-based) to a
        // complete effect-map. jc-dpkg-i: the fixture declares BOTH the `apt_get__predict`
        // (verb-dispatched) and the minimal verbless `dpkg__predict` (strips `-i`, so `dpkg`
        // is verbless ⇒ the ε-verb), preserving the pinned intent "dpkg -i establishes
        // package#installed" under check-is-the-oracle.
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/package.oracle.sh"
        ));
        let mut i = Interner::default();
        let out = lift(&mut i, &[fixture]);

        assert!(
            out.diags.is_empty(),
            "no diagnostics at all on the clean fixture: {:?}",
            out.diags
        );

        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));

        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, installed, ValueClaim::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "purge"),
            Some((package, installed, ValueClaim::EstablishInverted))
        );
        // `dpkg` is verbless (its check strips the `-i` flag), so its effect keys on the
        // ε-verb, not on a `-i` verb token (check-is-the-oracle: `-i` is a flag here).
        let eps = empty_verb(&mut i);
        assert_eq!(
            out.value
                .effect_of(ProviderId(i.intern("dpkg")), eps)
                .first()
                .map(|c| (c.kind, c.selector, c.claim)),
            Some((package, installed, ValueClaim::Establish)),
            "dpkg -i establishes package#installed via the ε-verb"
        );
    }

    #[test]
    fn multiple_sources_accumulate_deterministically() {
        // dn-1's whole point: many oracle files contribute to one index, in argument
        // order, with no cross-file interference. Two providers, same kind (the Seam).
        let a = "apt-get.predict() { verb=$1; shift; pkg : package = \"$1\"; \
                 case $verb in install) dpkg-query -W \"$pkg\" : package:\"$pkg\".installed ;; esac; }";
        let b = "yum.predict() { verb=$1; shift; pkg : package = \"$1\"; \
                 case $verb in install) rpm -q \"$pkg\" : package:\"$pkg\".installed ;; esac; }";
        let mut i = Interner::default();
        let out = lift(&mut i, &[a, b]);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, installed, ValueClaim::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "yum", "install"),
            Some((package, installed, ValueClaim::Establish))
        );
    }

    #[test]
    fn verbless_predict_keys_the_epsilon_verb() {
        // A verbless check (`command -v`) derives its effect on the ε-verb — the key the
        // wiring uses for a check that binds no verb (202 §2 / task-W §4).
        let src = "command.predict() { case $1 in -v) shift ;; esac; tool : tool = \"$1\"; \
                   command -v -- \"$tool\" >/dev/null 2>&1 :? tool:\"$tool\".present; }";
        let mut i = Interner::default();
        let out = lift(&mut i, &[src]);
        assert!(!out.value.is_empty(), "the verbless guard lifts a cell");
        let tool = KindId(i.intern("tool"));
        let present = SelectorId(i.intern("present"));
        let eps = empty_verb(&mut i);
        let cells = out.value.effect_of(ProviderId(i.intern("command")), eps);
        assert_eq!(
            cells,
            &[EffectCell {
                kind: tool,
                selector: present,
                claim: ValueClaim::Observe
            }],
            "the verbless guard keys on the ε-verb with an Observe claim: {cells:?}"
        );
    }

    #[test]
    fn empty_source_is_empty_index_no_panic() {
        // Totality (inv-no-throw): the degenerate input contributes nothing and is
        // silent — not an oracle, not an error.
        let mut i = Interner::default();
        let out = lift(&mut i, &[""]);
        assert!(out.value.is_empty());
        assert!(
            out.diags.is_empty(),
            "an empty source is not an error: {:?}",
            out.diags
        );
    }
}
