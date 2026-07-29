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
//!    pkg : sm.dorc.Package = "$1"                            # the kind annotation
//!    case $verb in
//!       install) dpkg-query -W "$pkg" : package:"$pkg"@installed ;;   # establish
//!       purge)   dpkg-query -W "$pkg" : package:"$pkg"@installed! ;;  # inverted
//!    esac
//! }
//! ```
//!
//! From a book's bare `apt-get install -y nginx`, the analyzer derives the effect
//! `(apt-get, install) → (package, @installed, Establish)` (the `case $verb` arm names the
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

use dorc_aid::Carrier;
use dorc_core::{Interner, KindId, ProviderId, SelectorId, Symbol};
use std::collections::{BTreeMap, BTreeSet};

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
/// §1c / 24D §3): the `<provider>.is_converged()` role member. Authoring it IS the vouch; this
/// module decides STATICALLY whether a site's argv reaches a vouching path (the
/// judgment-tier license source), and the guard emitter ships the same body strip-only. Reuses the
/// `predict` dialect.
pub mod verdict;

/// The tier-1 static report-sink **inventory** (`27W` §3 `rul-static-first-three-tier`): an
/// oracle-alone AST pass over verdict bodies that enumerates the `decline-class-emission`
/// idioms (`printf '<verb> <class> …' >>"${DREP_V1:-…}"`), value-threading the LITERAL format
/// string into per-arm `(arm span, class)`. Feeds tier-2 per-site classification and the
/// oracle-solo lint surface; a dynamically-built format defeats static reading and degrades to
/// tier-3 runtime emission (by design).
pub mod report;

/// The identity **canonicalizer** lift (24F §3 — the resid-aliasing closure; the FOURTH
/// role-sibling): the `<kind>.resolve()` role-sibling, keyed by KIND (not the command word). Lifted
/// and shipped strip-only to the probe lane, it runs read-only per coordinate and prints the
/// entity's canonical form; the engine canonicalizes both footprint and backing coordinates through
/// it before `disjoint`, closing the two-names-one-referent under-execute. Reuses the `predict`
/// dialect. Host-run only (no static evaluator — identity resolution needs the host).
pub mod resolve;

/// The REACH function lift (24G §4 — the cross-author footprint-EXPANSION mechanism; the FIFTH
/// role-sibling): the `<kind>.reaches()` role-sibling, keyed by KIND (like [`resolve`]). Invoked with
/// an ENTITY, its emitting arms declare — per kind, via a TRAILING ANNOTATION that types the emission
/// (24G §4) — what touching that entity DRAGS WITH IT (a package reaches its files + same-named unit).
/// The engine expands every footprint coordinate of a reach-bearing kind through it, whoever emitted
/// it (the cross-author point). Static arms trace at plan time; dynamic arms escalate to the probe.
/// Reuses the `predict` dialect (with the pipelines-may-carry-a-mark carve-out).
pub mod reaches;

/// The peeling-wrapper model (`273`; `271:rul-predict-absorbs-wrapper-modeling` + `rul-lend-map`
/// + `rul-env-claim-inversion`): peel DETECTION inside `<provider>__predict` (command-position
/// `"$@"` ⇒ peeling wrapper by tautology), the ρ-claim ladder, `<provider>__lend_map` with the
/// enumerate-every-dimension law, the wrapper/inner context split, and dual-peel coherence
/// fail-fast. MODELS only — mints no license (the entry/dial/vouch machinery is the next lane);
/// nothing here is consumed by `analysis`/`plan` yet, so the wrapper-free corpus stays byte-stable.
pub mod wrapper;

/// Context ENTRY (`27C` — THE primary lane): the `cmd__enter()` member (the one licensed seat for
/// real context entry), the two-axis consent decision (mechanical capability × the escalation dial ×
/// the `safe-across` vouch), the composition algebra across a peel chain (pointwise fold, ⊤
/// propagates, order-sensitive), the degrade ladder, and the entry-form author's vouched
/// self-effects (`27C:rul-probe-mutation-ownership-split`). Builds on [`wrapper`]'s peel model.
pub mod entry;

/// The pure-predicate carry (`27C` §4(a); steering `pure-predicate-carry`): the ONLY unflagged
/// cross-substrate-boundary carry. The (A) `undivided-by-transit-across <axis>` index lifted from
/// `state_stored_only_in()` (with the netns caveat), the (B) read-set-closure sh-taint pass over a
/// verdict body (default-disqualify, an audited pure-construct safe-list), and the combined carry
/// decision (substrate axes only; user excluded). Reads marks + structure only
/// (`inv-referent-agnostic`); fails safe (a missed-safe body loses an elision, never carries a read).
pub mod carry;

/// The eval'er surface (`274` · `271:rul-evaler-merge-no-structure-member`): reentry DETECTION
/// inside `<provider>__predict` (a body that delegates to the `dorc:sh`/`dorc-sh` reentry primitive
/// IS an eval'er — no separate structure member), the payload shape (which-arg-is-code / stdin /
/// file), and the reentry ρ-claim. MODELS only — mints no license; nothing here is consumed by
/// `analysis`/`plan` yet, so the eval'er-free corpus stays byte-stable.
pub mod evaler;

/// Payload decomposition (`24T` §4/§5 · `274` · `27J` §2.3): the accept/refuse basic-forms table
/// (`classify_payload_form` — single-quoted/const literal accepted, interpolated refused to ⊤), the
/// nested parse of an accepted literal as book-code (site-local degrade on unparseable /
/// nested-annotation), and the whole-line fold (`fold_line` — elide iff every leaf elides, else
/// guard-conjunction, else run). MODELS only; the per-leaf disposition is supplied (plan-wiring is
/// the follow-on) and no payload bytes are re-serialized (the synthesized-render door stays open).
pub mod payload;

/// The munge-reservation lint (24Kc `fix-munge-reservation`; 24M `ca-munge-charclass`): the
/// reserved `<munged>__<role>` sh-function namespace's charclass refusal, non-injective-munge
/// collision refusal, and book-squat disclosure. A standing mitigation for the coincidental
/// capture rul24M-bare-dorcism-names prices as accepted-not-prevented.
pub mod reserved;

/// The `# dorc-lang/v0.2` marker gate (`marker-gates-syntax-only`): dialect constructs (binds,
/// trailing marks) in an UNMARKED file are a loud error; bare `__role` floor bodies work markerless.
pub mod marker;

/// The marked-file load-inertness gate (`28K` §2a `rul-marked-file-is-load-inert`): a marked
/// file's top level holds definitions and bare assignments only, so both the abstract load
/// (`28K` §2) and the executed subshell re-source (`28K` §3) are provably no-ops.
pub mod load_inert;

/// The book-free oracle-side validation surface (`27S:seam-oracle-validate-factoring`): the cli's
/// inline oracle lints, factored into one entry the cli and the lint rung-oracle-solo lane share.
pub mod validate;

/// The whole-file off-ramp cleaner (`dorc strip` / `dorc-sh`): parser-backed erasure of every
/// dialect construct, yielding runnable stock sh (`strip-is-pure-erasure`, `274` §13).
pub mod strip;

pub use strip::strip_file;

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
/// `@installed` and may dirty a `@config` cell). The wiring (`analysis::effect`)
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
    /// (`@installed`/`@fresh`/`@enabled`/`@active`) is the per-entity facet
    /// (`an-per-entity-selector`, `notes/193` §4): `enable` and `start` target
    /// *different* selectors on the same `service` cell, so neither discharges the
    /// other. A **verbless** provider (`useradd`, `command -v`) keys on the ε-verb
    /// ([`empty_verb`]) — the check binds no verb, so the wiring looks up `(provider, ε)`
    /// (202 §2 / task-W §4).
    effects: BTreeMap<(ProviderId, Symbol), Vec<EffectCell>>,
    /// observe-backing-widening (`277` §5 `seam-backing-sets` / `271` observe-backing-widening):
    /// per `(provider, verb)`, the OBSERVE (`:?`) selectors that co-occur with a VERDICT
    /// (`:`/`:!`) mark in that verb's predict-body arm. Such an observe is INSIDE a verdict body,
    /// so it widens the enclosing establish fact's backing (a SIBLING cell — same kind+entity,
    /// this selector) rather than standing as its own Query row (`derive.rs`: the model forces
    /// all a `(provider, verb)`'s cells to one `(kind, entity)`, so the widening is selector-only).
    /// Recorded here INSTEAD of an [`EffectCell`], so the establish site still classifies
    /// `EstablishAmbient`/`Written` (a mixed `[Establishes, Queries]` slice would fall to
    /// `MustRun`). An observe with NO co-occurring verdict stays a `Queries` cell (its own row,
    /// "widens nothing"). Safe direction: widening only GROWS a fact's kill-surface.
    widenings: BTreeMap<(ProviderId, Symbol), BTreeSet<SelectorId>>,
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

    /// Record that `(provider, verb)`'s predict body carries a co-occurring OBSERVE mark for
    /// `selector` — i.e. an observe INSIDE a verdict body (`277` §5 observe-backing-widening).
    /// It widens the verb's establish fact backing (a sibling cell), never becoming a Query cell.
    pub fn add_widening(&mut self, provider: ProviderId, verb: Symbol, selector: SelectorId) {
        self.widenings
            .entry((provider, verb))
            .or_default()
            .insert(selector);
    }

    /// The observe-backing-widening selectors for `(provider, verb)` (`277` §5): the observed
    /// sibling cells whose reads back this verb's establish fact. Empty (the corpus default —
    /// every corpus observe is standalone, no co-occurring verdict) ⇒ no widening. The survival
    /// backing unions these (same kind+entity as the fact, these selectors) — kill-surface only
    /// grows (`inv-kfail`, apply).
    #[must_use]
    pub fn widening_of(&self, provider: ProviderId, verb: Symbol) -> &BTreeSet<SelectorId> {
        static EMPTY: BTreeSet<SelectorId> = BTreeSet::new();
        self.widenings.get(&(provider, verb)).unwrap_or(&EMPTY)
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

/// The forward-munge: map a command word or reverse-DNS kind to its sh-NAME function-name
/// segment (resp-munge-policy / `flag-forward-munge-keying`). `-` → `_` and `.` → `_`
/// (`apt-get` ⇒ `apt_get`; `sm.dorc.Package` ⇒ `sm_dorc_Package`), and a leading-digit
/// RESULT gains a `_` prefix (`7z` ⇒ `_7z`; `24P` §2). The shared home of the munge on both
/// the *emit* side (the shipped `<munge>__<role>` funcname) and the *lookup* side: a kind-keyed
/// role is authored `<munge(kind)>__resolve` and keyed by that base name AS-IS, so the engine
/// munges a coordinate's kind through THIS function to find its owner (the two agree by
/// construction). ASCII path only; non-ASCII (IDN) input is left for `validate_sh_name` to refuse
/// (rul24-idn-punycode punycoding is a spec-note, not implemented — `24P` §0).
///
/// **Lossy** (non-injective): `-`, `.`, and a literal `_` all collapse to `_`. The
/// `munge-name-collision` reservation lint catches any non-injectivity that co-loads.
#[must_use]
pub fn to_funcname_segment(name: &str) -> String {
    let munged = name.replace(['-', '.'], "_");
    match munged.chars().next() {
        Some(c) if c.is_ascii_digit() => format!("_{munged}"),
        _ => munged,
    }
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
            let (effects, diags) = predict::derive_predict(c);
            // A derive-side diagnostic (a brace-alternation on a verdict/observe mark, `277` §4c)
            // is surfaced here — the effect-map lift IS reported (cli `report_at`); this is NOT a
            // check-lift/parse diag (those come from the separate `lift_predicts` pass).
            out.diags.extend(diags);
            // Intern each derived cell, keyed by verb, so we can decide observe-backing-widening
            // per verb (an arm = a verb; `derive.rs` binds one verb per literal-pattern arm).
            let mut by_verb: BTreeMap<Symbol, Vec<(KindId, SelectorId, ValueClaim)>> =
                BTreeMap::new();
            for e in effects {
                let verb = match e.verb {
                    Some(v) => interner.intern(&v),
                    None => empty_verb(interner),
                };
                let kind = KindId(interner.intern(&e.kind));
                let selector = SelectorId(interner.intern(&e.selector));
                by_verb
                    .entry(verb)
                    .or_default()
                    .push((kind, selector, e.claim));
            }
            let provider = ProviderId(provider);
            for (verb, cells) in by_verb {
                // observe-backing-widening (`277` §5): an OBSERVE (`:?`) that co-occurs with a
                // VERDICT (`:`/`:!`) in one verb's arm is INSIDE a verdict body ⇒ it WIDENS the
                // establish fact's backing (recorded via `add_widening`), NOT a Query cell. An
                // observe with NO co-occurring verdict is standalone ⇒ stays a `Queries` cell
                // (its own row, "widens nothing" — every corpus `:?` is this case, verbless).
                let has_verdict = cells.iter().any(|(_, _, c)| {
                    matches!(c, ValueClaim::Establish | ValueClaim::EstablishInverted)
                });
                for (kind, selector, claim) in cells {
                    if has_verdict && claim == ValueClaim::Observe {
                        out.value.add_widening(provider, verb, selector);
                    } else {
                        // A duplicate cell cannot arise from a well-formed check (each verb-arm
                        // names a distinct selector); drop a derived duplicate silently.
                        out.value.add_effect(provider, verb, kind, selector, claim);
                    }
                }
            }
        }
    }
    out
}

/// Build the selector [`Dialect`](dorc_core::Dialect) (`277` §3) from a lifted [`KindIndex`]: for
/// each `(provider, verb)` effect cell, MINT `(family = provider, cell.kind, cell.selector)`. The
/// index holds exactly the verdict/observe-mark cells (`derive_predict` reads `:`/`:!`/`:?` marks)
/// — disturbs/claim emissions never enter it, so this is the exact minting set (`sparing-algebra`:
/// only verdict/observe marks mint). Family = the command provider (`271:rul-family`, name-derived).
/// The survival comparison consults `dialect(backing's family, kind)` to spare a sibling cell.
#[must_use]
pub fn build_dialect(index: &KindIndex) -> dorc_core::Dialect {
    let mut dialect = dorc_core::Dialect::empty();
    for ((provider, _verb), cells) in &index.effects {
        for cell in cells {
            dialect.mint(*provider, cell.kind, cell.selector);
        }
    }
    dialect
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
        let package = KindId(interner.intern("sm.dorc.Package"));
        let apt = ProviderId(interner.intern("apt_get"));
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
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let package = KindId(i.intern("sm.dorc.Package"));
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
            effect(&idx, &mut i, "apt_get", "install"),
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
        // package@installed" under check-is-the-oracle.
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

        let package = KindId(i.intern("sm.dorc.Package"));
        let installed = SelectorId(i.intern("installed"));

        assert_eq!(
            effect(&out.value, &mut i, "apt_get", "install"),
            Some((package, installed, ValueClaim::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt_get", "purge"),
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
            "dpkg -i establishes package@installed via the ε-verb"
        );
    }

    #[test]
    fn multiple_sources_accumulate_deterministically() {
        // dn-1's whole point: many oracle files contribute to one index, in argument
        // order, with no cross-file interference. Two providers, same kind (the Seam).
        let a = "apt_get__predict() { verb=$1; shift; pkg : sm.dorc.Package = \"$1\"; \
                 case $verb in install) dpkg-query -W \"$pkg\" : sm.dorc.Package:\"$pkg\"@installed ;; esac; }";
        let b = "yum__predict() { verb=$1; shift; pkg : sm.dorc.Package = \"$1\"; \
                 case $verb in install) rpm -q \"$pkg\" : sm.dorc.Package:\"$pkg\"@installed ;; esac; }";
        let mut i = Interner::default();
        let out = lift(&mut i, &[a, b]);
        let package = KindId(i.intern("sm.dorc.Package"));
        let installed = SelectorId(i.intern("installed"));
        assert_eq!(
            effect(&out.value, &mut i, "apt_get", "install"),
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
        let src = "command__predict() { case $1 in -v) shift ;; esac; tool : sm.dorc.Tool = \"$1\"; \
                   command -v -- \"$tool\" >/dev/null 2>&1 :? sm.dorc.Tool:\"$tool\"@present; }";
        let mut i = Interner::default();
        let out = lift(&mut i, &[src]);
        assert!(!out.value.is_empty(), "the verbless guard lifts a cell");
        let tool = KindId(i.intern("sm.dorc.Tool"));
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
    fn co_occurring_observe_widens_the_verdict_fact_not_a_query_cell() {
        // `277` §5 observe-backing-widening: a `:?` observe INSIDE a verdict body (same verb arm
        // as a `:` establish) widens the establish fact's backing — recorded as a widening
        // SELECTOR, NOT emitted as a `Queries` cell (a mixed establish+query slice at the book
        // site would fall to `MustRun`). Here `install` establishes `@installed` AND observes
        // `@indexed`: the effect map keeps ONLY the establish cell; `@indexed` is a widening.
        let src = "apt_get__predict() { verb=$1; shift; pkg : sm.dorc.Package = \"$1\"; \
                   case $verb in install) dpkg-query -W \"$pkg\" : sm.dorc.Package:\"$pkg\"@installed; \
                   dpkg-query -s \"$pkg\" :? sm.dorc.Package:\"$pkg\"@indexed ;; esac; }";
        let mut i = Interner::default();
        let out = lift(&mut i, &[src]);
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let package = KindId(i.intern("sm.dorc.Package"));
        let installed = SelectorId(i.intern("installed"));
        let indexed = SelectorId(i.intern("indexed"));
        // The effect map keeps only the establish cell (NOT a Query cell for @indexed).
        assert_eq!(
            out.value.effect_of(apt, install),
            &[EffectCell {
                kind: package,
                selector: installed,
                claim: ValueClaim::Establish
            }],
            "the observe co-occurring with the verdict is NOT a Query cell"
        );
        // @indexed is recorded as a backing-widening for (apt_get, install).
        assert!(
            out.value.widening_of(apt, install).contains(&indexed),
            "the co-occurring observe widens the verdict fact's backing"
        );
    }

    #[test]
    fn standalone_observe_stays_a_query_cell_widens_nothing() {
        // The corpus reality: a verbless `:?` observe with NO co-occurring verdict is standalone
        // ⇒ it stays a `Queries` cell (its own row) and widens NOTHING (`277` §5). This is why the
        // whole corpus is byte-identical: every corpus observe is exactly this shape.
        let src = "dpkg__predict() { case $1 in -s) shift ;; esac; pkg : sm.dorc.Package = \"$1\"; \
                   dpkg -s -- \"$pkg\" >/dev/null 2>&1 :? sm.dorc.Package:\"$pkg\"@installed; }";
        let mut i = Interner::default();
        let out = lift(&mut i, &[src]);
        let dpkg = ProviderId(i.intern("dpkg"));
        let eps = empty_verb(&mut i);
        let package = KindId(i.intern("sm.dorc.Package"));
        let installed = SelectorId(i.intern("installed"));
        assert_eq!(
            out.value.effect_of(dpkg, eps),
            &[EffectCell {
                kind: package,
                selector: installed,
                claim: ValueClaim::Observe
            }],
            "a standalone observe stays a Query cell"
        );
        assert!(
            out.value.widening_of(dpkg, eps).is_empty(),
            "a standalone observe widens nothing"
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
