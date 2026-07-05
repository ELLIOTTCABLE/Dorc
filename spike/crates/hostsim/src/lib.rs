//! `dorc-hostsim` — a seeded, deterministic state-machine model of a target host,
//! the deterministic-simulation (DST) test substrate (notes 162 §4 / 163 §4).
//!
//! Two jobs:
//!
//! * **Answer fact-probes** against a modeled system-state — the concrete stand-in
//!   for the kernel's injected `verdict_of` seam ([`Host::verdict`]). `Converged`
//!   iff the fact holds on this host.
//! * **Detect a probe attempting a modeled mutation** — the `kFAIL-withhold` check
//!   (note 162 DP-4): a read-only probe that tries to `Establish`/`Kill` during the
//!   probe phase is flagged AND refused ([`Host::run`]). This is the spike
//!   stand-in for the real seccomp/sandbox enforcement, which the contract frame
//!   provably cannot supply.
//!
//! Nondeterminism lives ONLY here, behind a seeded PRNG injected as a `u64` seed —
//! the one place `inv-determinism` permits it, and only because it is seeded and
//! reproducible. No async, no real I/O: the host is a pure state machine over
//! [`FactKey`]s. The kernel crates depend on none of this.

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; this crate-root expect
// ratchets away during the rebuild (an unfulfilled `expect` warns, so it
// self-removes as the seeded layer is replaced). It never relaxes the policy
// for new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::arithmetic_side_effects,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use std::collections::{BTreeMap, BTreeSet};

use dorc_analysis::effect::FactKey;
use dorc_core::{EntityRef, KindId, Observable, Phase, Verdict};

pub mod differential;

/// A tiny deterministic linear-congruential PRNG — the host's seeded
/// nondeterminism. Hand-rolled (no `rand` dependency): the DST host must be
/// reproducible bit-for-bit from its seed, and the kernel stays dep-free. The
/// multiplier/increment are the common 64-bit LCG constants (Knuth/PCG lineage).
///
/// PUBLIC as the spike's single home of seeded entropy (`inv-determinism`): the round-24
/// `sweep` chronology net drives its scenario generator from this same `Lcg` rather than
/// forking a second PRNG (24B §3 "reuses hostsim's `Lcg` for entropy — keep nondeterminism
/// single-homed"). Not cryptographic; bit-reproducible from the seed is the whole contract.
#[derive(Debug, Clone)]
pub struct Lcg(u64);

impl Lcg {
    /// Seed the PRNG. Same seed ⇒ same stream, forever (`inv-determinism`).
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Lcg(seed)
    }

    /// The next 64-bit draw (advances the state).
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// A coin flip true with probability `num / den` (deterministic given the seed).
    pub fn chance(&mut self, num: u32, den: u32) -> bool {
        den != 0 && (self.next_u64() % u64::from(den)) < u64::from(num)
    }

    /// A draw in `0..bound` (deterministic; `0` for `bound == 0`), taken from the HIGH bits via
    /// Lemire's multiply-high. This is load-bearing, NOT decorative: an odd-multiplier LCG's LOW
    /// bits are periodic (the low bit flips every step, the low `k` bits have period `2^k`), so a
    /// naive `next_u64() % small` — and thus [`chance`](Lcg::chance) — makes consecutive small-modulus
    /// draws deterministically correlate. The round-24 sweep draws EVERY axis through here so its
    /// independent coins are actually independent (a low-bit `% 2` made `lying` perfectly
    /// anti-correlate `victim_converged`, silently erasing a whole topology cell). The high 64 bits
    /// of the `u64 × bound` product are well-distributed for an LCG; bias is negligible for the
    /// small option-sets a generator draws.
    pub fn below(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            return 0;
        }
        u64::try_from((u128::from(self.next_u64()) * u128::from(bound)) >> 64).unwrap_or(0)
    }
}

/// One operation a shipped probe/apply step performs against the host, abstracted
/// to its system-state effect (the DST models effects, not real sh execution). A
/// well-behaved *probe* is all [`Query`](HostOp::Query); an `Establish`/`Kill`
/// during the probe phase is the `kFAIL-withhold` breach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostOp {
    /// Read-only: does this fact hold? (Inert in both phases.)
    Query(FactKey),
    /// Mutate: make the fact hold. Legitimate only in the apply phase.
    Establish(FactKey),
    /// Mutate: make the fact not hold. Legitimate only in the apply phase.
    Kill(FactKey),
}

/// A recorded `kFAIL-withhold` breach: a mutating op attempted during the probe
/// phase. The DST stand-in for what a sandbox would catch on a real host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Violation {
    pub phase: Phase,
    pub op: HostOp,
}

/// A concrete GROUND-TRUTH cell-delta: the set of cells a running command really flips
/// (the `sweep` chronology net's `TrueEffect` payload — 24B §3 "apply-a-cell-delta").
///
/// The model altitude the chronology net needs on `Host`: the `sweep` generator invents,
/// per book command, what that command *actually does* to host state — independently of what
/// the oracle *declares* it does (the declared-vs-true split, 24B §3 / §5). This type carries
/// only the true half; enacting it is [`Host::apply_delta`]. Deltas are always applied during
/// the sweep's host EVOLUTION (apply-semantics), never during a probe, so — unlike
/// [`HostOp::Establish`]/[`HostOp::Kill`] under [`Host::run`] — there is no phase and no
/// `kFAIL-withhold` monitoring here: a delta *is* a command running.
///
/// `establishes` and `kills` are disjoint by construction (a command flips a cell one way);
/// [`Host::apply_delta`] applies establishes then kills, so a (mis-built) overlap resolves
/// to killed, deterministically. Ordered sets (`inv-determinism`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CellDelta {
    establishes: BTreeSet<FactKey>,
    kills: BTreeSet<FactKey>,
}

impl CellDelta {
    /// An empty delta — a command that touches no modeled cell (a no-op mutator).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a cell this command makes hold. Consuming-builder shape so a generator can spell a
    /// site's true effect inline (`CellDelta::new().establish(a).kill(b)`).
    #[must_use]
    pub fn establish(mut self, fact: FactKey) -> Self {
        self.establishes.insert(fact);
        self
    }

    /// Add a cell this command makes NOT hold (a kill/purge's true effect, or a LYING
    /// footprint's undeclared clobber — the resid-aliasing disaster the net hunts).
    #[must_use]
    pub fn kill(mut self, fact: FactKey) -> Self {
        self.kills.insert(fact);
        self
    }

    /// Whether this delta flips nothing (a site with no ground-truth effect — e.g. an
    /// already-converged re-run in a converged≠no-op-free world).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.establishes.is_empty() && self.kills.is_empty()
    }

    /// The cells this delta makes hold (attribution/coverage inspection).
    pub fn establishes(&self) -> impl Iterator<Item = FactKey> + '_ {
        self.establishes.iter().copied()
    }

    /// The cells this delta makes not-hold (attribution/coverage inspection).
    pub fn kills(&self) -> impl Iterator<Item = FactKey> + '_ {
        self.kills.iter().copied()
    }
}

/// A seeded, deterministic model of a target host: the set of facts that currently
/// hold, the `kFAIL-withhold` monitor, plus the DECLARED derivation manifests (24E §6).
#[derive(Debug, Clone)]
pub struct Host {
    facts: BTreeSet<FactKey>,
    violations: Vec<Violation>,
    /// The DECLARED derived-footprint manifests (24E §6 / fork-s4-declaredtrue): per escalated
    /// wall-site cell, the modeled entity-set its host-run `touches()` would emit — the SAME shape
    /// discipline as [`verdict`](Host::verdict): DECLARED scenario data, deterministic, no ssh, and
    /// emphatically NOT a `dpkg` simulation (the `hostsim/CLAUDE.md` fidelity-vs-coverage line — a
    /// declared-data oracle, never a re-implemented tool). Entity-granular (selector ignored by the
    /// plan's `disjoint`). This rides the sweep's declared-vs-true split: a manifest NARROWER than a
    /// site's TRUE [`CellDelta`] is the LYING derived footprint (⊂ true) that under-declares what the
    /// wall touches ⇒ a downstream fact wrongly survives ⇒ the end-state differential goes RED.
    manifests: BTreeMap<FactKey, BTreeSet<FactKey>>,
    /// The DECLARED identity-resolver answers (24F §3 — the resid-aliasing closure): per
    /// `(kind, entity)` coordinate, the CANONICAL entity its `<kind>.resolve()` prints host-side.
    /// The SAME shape discipline as [`manifests`]/[`verdict`](Host::verdict): DECLARED scenario data,
    /// deterministic, no ssh, NOT a `dpkg-query -W` simulation — a declared-data oracle. This rides
    /// the sweep's declared-vs-TRUE split for IDENTITY (24F §7.1): the generator invents a TRUE
    /// identity (two names → one referent, carried in the `CellDelta`) AND a declared resolver
    /// answer; an HONEST resolver maps both names to one canonical (the aliasing closure DEMOTES the
    /// victim ⇒ safe); a LYING resolver keeps them apart (returns each name's own canonical) ⇒ the
    /// victim wrongly survives ⇒ the end-state differential goes RED.
    resolutions: BTreeMap<(KindId, EntityRef), EntityRef>,
    /// Kinds whose owner shipped a resolver (24F §3). A coordinate in such a kind with NO
    /// [`resolutions`] entry degrades to may-alias (§3a); the sweep's alias scenarios always declare
    /// both names, so this set is populated implicitly by [`with_resolution`](Host::with_resolution).
    resolver_kinds: BTreeSet<KindId>,
}

impl Host {
    /// A host whose initial state is exactly `holding` (no PRNG — the state is
    /// given, not generated).
    #[must_use]
    pub fn new(holding: impl IntoIterator<Item = FactKey>) -> Self {
        Host {
            facts: holding.into_iter().collect(),
            violations: Vec::new(),
            manifests: BTreeMap::new(),
            resolutions: BTreeMap::new(),
            resolver_kinds: BTreeSet::new(),
        }
    }

    /// Attach a DECLARED derivation manifest (24E §6): the modeled entity-set the host-run
    /// `touches()` for `cell` would emit (the derived footprint). Consuming-builder shape so a
    /// scenario spells a site's declared footprint inline. Honest ⇒ `coords` ⊇ the site's true
    /// [`CellDelta`]; lying ⇒ `coords` ⊂ true (the too-narrow footprint the sweep net hunts).
    #[must_use]
    pub fn with_manifest(
        mut self,
        cell: FactKey,
        coords: impl IntoIterator<Item = FactKey>,
    ) -> Self {
        self.manifests.insert(cell, coords.into_iter().collect());
        self
    }

    /// Attach a DECLARED identity-resolver answer (24F §3): the `(kind, entity)` coordinate
    /// canonicalizes to `canonical` when `<kind>.resolve()` runs host-side. Consuming-builder shape
    /// so a scenario spells its alias/identity map inline. Marks `kind` resolver-bearing. An HONEST
    /// resolver maps two aliased names to ONE `canonical` (the closure demotes ⇒ safe); a LYING one
    /// maps each name to itself (kept apart ⇒ the victim wrongly survives — the sweep net's RED).
    #[must_use]
    pub fn with_resolution(
        mut self,
        kind: KindId,
        entity: EntityRef,
        canonical: EntityRef,
    ) -> Self {
        self.resolver_kinds.insert(kind);
        self.resolutions.insert((kind, entity), canonical);
        self
    }

    /// Declare a kind resolver-bearing without a specific coordinate (24F §3a): a coordinate in it
    /// with no [`with_resolution`](Host::with_resolution) entry degrades to may-alias. (The sweep's
    /// alias scenarios declare both names, so this is a completeness hook, not currently exercised.)
    #[must_use]
    pub fn with_resolver_kind(mut self, kind: KindId) -> Self {
        self.resolver_kinds.insert(kind);
        self
    }

    /// The DECLARED canonical form of a `(kind, entity)` coordinate (24F §3) — the resolver analogue
    /// of [`verdict`](Host::verdict)/[`derive`](Host::derive). Deterministic, scenario-driven, no
    /// ssh, NOT a `dpkg-query` simulation. `None` ⇒ the resolver produced nothing for it (a
    /// resolver-bearing kind's unresolved coordinate ⇒ may-alias, §3a).
    #[must_use]
    pub fn resolve(&self, kind: KindId, entity: EntityRef) -> Option<EntityRef> {
        self.resolutions.get(&(kind, entity)).copied()
    }

    /// The resolver-bearing kinds (24F §3 — the caller marks each in its `Resolutions`).
    pub fn resolver_kinds(&self) -> impl Iterator<Item = KindId> + '_ {
        self.resolver_kinds.iter().copied()
    }

    /// The declared `(kind, entity) → canonical` resolutions (24F §3 — the caller records each into
    /// its `Resolutions`). Deterministic order (`inv-determinism`).
    pub fn resolutions(&self) -> impl Iterator<Item = ((KindId, EntityRef), EntityRef)> + '_ {
        self.resolutions.iter().map(|(&k, &v)| (k, v))
    }

    /// A host whose initial state is a seeded random subset of `candidates` (each
    /// included with probability ½). The DST scenario generator: looping over seeds
    /// fuzzes the analyzer/plan over many host states, reproducibly.
    #[must_use]
    pub fn seeded(seed: u64, candidates: &[FactKey]) -> Self {
        let mut rng = Lcg::new(seed);
        let facts = candidates
            .iter()
            .copied()
            .filter(|_| rng.chance(1, 2))
            .collect();
        Host {
            facts,
            violations: Vec::new(),
            manifests: BTreeMap::new(),
            resolutions: BTreeMap::new(),
            resolver_kinds: BTreeSet::new(),
        }
    }

    /// Read-only verdict for a fact — the concrete `verdict_of` the plan stage
    /// injects. `Converged` iff the fact holds, else `Diverged`. (A modeled,
    /// reachable host is never `Unknown`; `Unknown` is the kernel's own fold for an
    /// un-probeable or unreachable fact.)
    #[must_use]
    pub fn verdict(&self, fact: FactKey) -> Verdict {
        if self.facts.contains(&fact) {
            Verdict::Converged
        } else {
            Verdict::Diverged
        }
    }

    /// The DECLARED derived footprint for a wall-site `cell` (24E §6) — the modeled entity-set its
    /// host-run `touches()` emits, the derivation analogue of [`verdict`](Host::verdict).
    /// Deterministic, scenario-driven, no ssh, no `dpkg` simulation (a declared-data oracle). An
    /// unmodeled cell yields the EMPTY set: no declaration ⇒ an empty derived footprint ⇒ the site
    /// walls (silence = wall — kFAIL-safe). Same declared-vs-true discipline as [`CellDelta`]: an
    /// honest manifest ⊇ the site's true delta; a lying one ⊂ it (under-declares ⇒ wrong survival).
    #[must_use]
    pub fn derive(&self, cell: FactKey) -> BTreeSet<FactKey> {
        self.manifests.get(&cell).cloned().unwrap_or_default()
    }

    /// The full read-only [`Observable`] for a fact — the concrete `observe` the plan
    /// stage's fold + value-preserving substitution inject (`19B` build-1). The host
    /// is a plain set-membership oracle: it answers *whether* a fact holds, **not** the
    /// exact exit status a tool yields when re-run converged — that is the (build-2)
    /// oracle contract's job to declare (opt-B, `19B §1`), command-by-command. So
    /// `observe` carries **no rc** (`None` ⇒ ⊤ for the fold), in BOTH the `Converged`
    /// and `Diverged` cases.
    ///
    /// This is the `19D` `kFAIL-perform` fix: synthesizing a conforming `rc=0` here was
    /// a confident *wrong* value for a non-conforming establish (`useradd` exits 9 when
    /// converged), letting the fold short-circuit a `|| fallback` dead — a priority-1
    /// under-execute (`inv-kfail`). A test needing an exact rc injects its own
    /// `Observable { rc: Some(_), .. }` (the unit matrix's non-conforming case does);
    /// the host never fabricates one. (`an-host-as-adversary`/`tc-reliability`: a
    /// modeled host states membership, not a tool's private rc convention.)
    #[must_use]
    pub fn observe(&self, fact: FactKey) -> Observable {
        Observable::verdict_only(self.verdict(fact))
    }

    /// Run one op in `phase`. A mutating op (`Establish`/`Kill`) in [`Phase::Probe`]
    /// is a `kFAIL-withhold` violation: it is RECORDED (see [`violations`]) and
    /// REFUSED — the modeled host state does not change, because a probe must never
    /// mutate. In [`Phase::Apply`] mutating ops apply. [`HostOp::Query`] returns the
    /// verdict in both phases and never mutates.
    ///
    /// [`violations`]: Host::violations
    pub fn run(&mut self, phase: Phase, op: HostOp) -> Option<Verdict> {
        match op {
            HostOp::Query(fact) => Some(self.verdict(fact)),
            HostOp::Establish(fact) => {
                if phase == Phase::Probe {
                    self.violations.push(Violation { phase, op });
                } else {
                    self.facts.insert(fact);
                }
                None
            }
            HostOp::Kill(fact) => {
                if phase == Phase::Probe {
                    self.violations.push(Violation { phase, op });
                } else {
                    self.facts.remove(&fact);
                }
                None
            }
        }
    }

    /// The `kFAIL-withhold` breaches recorded so far (empty on a well-behaved run).
    #[must_use]
    pub fn violations(&self) -> &[Violation] {
        &self.violations
    }

    /// Enact a concrete ground-truth [`CellDelta`] — the `sweep` chronology net's host
    /// evolution (24B §3). Establishes are inserted then kills removed; NO phase, NO
    /// `kFAIL-withhold` check (a delta *is* a command running, so it is apply-semantics by
    /// definition — the same effect [`Host::run`] applies for `Establish`/`Kill` in
    /// [`Phase::Apply`], factored for a whole delta). Deterministic (ordered sets).
    pub fn apply_delta(&mut self, delta: &CellDelta) {
        for fact in &delta.establishes {
            self.facts.insert(*fact);
        }
        for fact in &delta.kills {
            self.facts.remove(fact);
        }
    }

    /// The full set of cells that currently hold — the modeled END-STATE (the chronology net's
    /// `S_bare`/`S_apply` comparand, and the determinism-guard comparand). A cheap clone of the
    /// ordered fact-set; end-state equality is set equality over this (`inv-determinism`).
    #[must_use]
    pub fn snapshot(&self) -> BTreeSet<FactKey> {
        self.facts.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{EntityRef, Interner, KindId, OpaqueToken, SelectorId};

    /// Corpus-shaped apt-get check (flag-strip → verb → single-operand `package`
    /// annotation, `[ "$2" = "" ]` multi-operand refusal). These DST tests model only
    /// `apt-get install` on `package`; `systemctl reload` has no check ⇒ Opaque ⇒ runs.
    /// Lifted with the test's interner so provider symbols match the book.
    const CORPUS_PREDICT_SRC: &str = r#"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ; fi
}
"#;

    /// Test convenience (elide-weld, 24D §3): vouch EVERY establish-bearing site so these DSTs keep
    /// exercising elision MECHANICS (convergence × wall). The vouch GATE is pinned elsewhere (plan's
    /// `no_license_for_ambient_without_vouch` + the FAITHFUL sweep/coverage verdict-lift + e2e); a
    /// synthetic vouch (no oracle lift) keeps the DST focused, and its payload is inert (the elide
    /// mint consumes the vouch as the TIER check, never reads its bytes).
    fn vouch_all(
        classes: &[(
            dorc_analysis::cfg::CfgNodeId,
            dorc_analysis::effect::SkipClass,
        )],
    ) -> dorc_plan::Vouches {
        use dorc_analysis::effect::SkipClass;
        let mut vouches = dorc_plan::Vouches::new();
        for (node, class) in classes {
            // Ambient-only: a vouched+converged EstablishWritten fires the guard tier, which these
            // elision DSTs do not exercise (elide-weld's concern is EstablishAmbient — 24D §3).
            if matches!(class, SkipClass::EstablishAmbient(_)) {
                let vouch = dorc_plan::VerdictVouch::new(
                    "apt_get__is_converged".to_string(),
                    "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
                    "apt_get__is_converged".to_string(),
                    dorc_oracle::verdict::VerdictSense::Converged,
                    "package".to_string(),
                    vec!["dpkg-query".to_string()],
                );
                vouches.insert(
                    *node,
                    dorc_core::ByVouch::vouched(vouch, dorc_core::Rung::Both),
                );
            }
        }
        vouches
    }

    /// R3 test seam: resolve+strip the corpus check for a site's (provider, argv) — the same
    /// resolution the cli's `ship_predict_body` runs. `None` ⇒ un-shippable (un-oracled provider).
    fn ship_corpus(
        checks: &[dorc_oracle::predict::PredictSet],
        interner: &Interner,
        provider: dorc_core::Symbol,
        argv: &[dorc_core::Symbol],
    ) -> Option<String> {
        use dorc_oracle::predict::{Resolution, evaluate, map_provider_name, strip_predict};
        let want = map_provider_name(interner.resolve(provider));
        let arg_texts: Vec<String> = argv
            .iter()
            .map(|s| interner.resolve(*s).to_owned())
            .collect();
        let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
        for cs in checks {
            for cp in cs.providers() {
                if map_provider_name(interner.resolve(cp)) != want {
                    continue;
                }
                let Some(check) = cs.get(cp) else { continue };
                if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                    return Some(strip_predict(CORPUS_PREDICT_SRC, check, interner));
                }
            }
        }
        None
    }

    /// Run value-flow + the corpus checks + classify (the DST tests' shared pipeline).
    fn classify_value(
        cfg: &dorc_analysis::cfg::Cfg,
        ast: &dorc_syntax::ast::Ast,
        idx: &dorc_oracle::KindIndex,
        i: &mut Interner,
    ) -> Vec<(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )> {
        let value = dorc_analysis::value::analyze(cfg, ast, i);
        let checks = vec![dorc_oracle::predict::lift_predicts(i, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        dorc_analysis::effect::classify(cfg, &value, ast, idx, &checks, i, &mut arena).value
    }

    /// `kind:entity#installed` — the re-keyed cell (`notes/193`). These host-model
    /// tests only ever exercise `package#installed`, so the selector is fixed here;
    /// the host is a plain set-membership oracle over whatever `FactKey` it is given.
    fn fk(i: &mut Interner, kind: &str, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern(kind)),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("installed")),
        }
    }

    #[test]
    fn verdict_reflects_modeled_state() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let curl = fk(&mut i, "package", "curl");
        let host = Host::new([nginx]);
        assert_eq!(
            host.verdict(nginx),
            Verdict::Converged,
            "held fact ⇒ converged"
        );
        assert_eq!(
            host.verdict(curl),
            Verdict::Diverged,
            "absent fact ⇒ diverged"
        );
    }

    #[test]
    fn seeded_state_is_deterministic() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let curl = fk(&mut i, "package", "curl");
        let a = Host::seeded(42, &[nginx, curl]);
        let b = Host::seeded(42, &[nginx, curl]);
        assert_eq!(a.verdict(nginx), b.verdict(nginx), "same seed ⇒ same state");
        assert_eq!(a.verdict(curl), b.verdict(curl));
    }

    #[test]
    fn probe_phase_mutation_is_a_withhold_violation_and_refused() {
        // DP-4 (kFAIL-withhold): a "probe" that tries to mutate during the probe
        // phase is flagged AND refused — the host state is unchanged.
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let mut host = Host::new([]);
        assert_eq!(host.verdict(nginx), Verdict::Diverged);

        let refused = host.run(Phase::Probe, HostOp::Establish(nginx));
        assert!(refused.is_none());
        assert_eq!(
            host.verdict(nginx),
            Verdict::Diverged,
            "probe mutation must NOT take effect"
        );
        assert_eq!(
            host.violations().len(),
            1,
            "the withhold breach is recorded"
        );
        assert_eq!(host.violations()[0].op, HostOp::Establish(nginx));
    }

    #[test]
    fn apply_phase_mutation_takes_effect_no_violation() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let mut host = Host::new([]);
        host.run(Phase::Apply, HostOp::Establish(nginx));
        assert_eq!(
            host.verdict(nginx),
            Verdict::Converged,
            "apply establish takes effect"
        );
        assert!(host.violations().is_empty(), "apply mutation is legitimate");
        host.run(Phase::Apply, HostOp::Kill(nginx));
        assert_eq!(
            host.verdict(nginx),
            Verdict::Diverged,
            "apply kill takes effect"
        );
    }

    #[test]
    fn apply_delta_evolves_end_state_and_snapshot_reflects_it() {
        // The sweep's host-evolution primitive: a CellDelta establishes + kills cells; the
        // snapshot is the end-state comparand. No phase, no violation (a delta IS a run).
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let oldpkg = fk(&mut i, "package", "oldpkg");
        let mut host = Host::new([nginx]);
        let delta = CellDelta::new().establish(oldpkg).kill(nginx);
        host.apply_delta(&delta);
        assert_eq!(
            host.verdict(oldpkg),
            Verdict::Converged,
            "delta established oldpkg"
        );
        assert_eq!(host.verdict(nginx), Verdict::Diverged, "delta killed nginx");
        assert!(
            host.violations().is_empty(),
            "a delta is apply-semantics — never a violation"
        );
        assert_eq!(
            host.snapshot(),
            BTreeSet::from([oldpkg]),
            "snapshot is exactly the cells that now hold"
        );
    }

    #[test]
    fn derive_returns_declared_manifest_and_models_the_lying_footprint() {
        // 24E §6: the derivation-answer is DECLARED scenario data (not a dpkg sim). An HONEST
        // manifest ⊇ the site's TRUE effect ⇒ disjointness is sound; a LYING (too-narrow) manifest
        // ⊂ true under-declares a truly-touched cell ⇒ that cell wrongly looks disjoint — the
        // priced residue the declared-vs-true sweep net turns RED (fork-s4-declaredtrue).
        let mut i = Interner::default();
        let oldpkg = fk(&mut i, "package", "oldpkg");
        let oldpkg_file = fk(&mut i, "file", "/etc/oldpkg.conf");
        let victim = fk(&mut i, "package", "nginx");

        // HONEST: the wall's true effect is {oldpkg, oldpkg_file}; its manifest DECLARES both ⊇ true.
        let honest = Host::new([]).with_manifest(oldpkg, [oldpkg, oldpkg_file]);
        let declared = honest.derive(oldpkg);
        assert!(
            declared.contains(&oldpkg) && declared.contains(&oldpkg_file),
            "the declared manifest covers the wall's true footprint"
        );
        assert!(
            !declared.contains(&victim),
            "the victim is genuinely untouched ⇒ correctly disjoint ⇒ survives"
        );

        // LYING: the wall TRULY also touches the victim (CellDelta kills nginx) but its manifest
        // OMITS it — the ⊂ true under-declaration that makes the victim wrongly survive.
        let true_effect = CellDelta::new().kill(victim);
        let lying = Host::new([]).with_manifest(oldpkg, [oldpkg]);
        assert!(
            !lying.derive(oldpkg).contains(&victim),
            "the lying manifest under-declares the victim it truly kills..."
        );
        assert!(
            true_effect.kills().any(|k| k == victim),
            "...which the TRUE effect actually kills ⇒ wrong-survival (the sweep net's RED)"
        );

        // An unmodeled cell ⇒ empty ⇒ wall (silence = wall, kFAIL-safe).
        assert!(
            Host::new([]).derive(oldpkg).is_empty(),
            "no manifest ⇒ empty derived footprint ⇒ the site walls"
        );
    }

    #[test]
    fn resolve_models_honest_merge_and_lying_split() {
        // 24F §3/§7.1: the declared identity-resolver answer. HONEST maps two aliased names
        // (nginx / nginx-full) to ONE canonical (the closure will HIT ⇒ demote ⇒ safe); LYING maps
        // each to itself (kept apart ⇒ the victim wrongly survives ⇒ the sweep's RED). A declared-
        // data oracle (no dpkg-query sim), same discipline as `verdict`/`derive`.
        let mut i = Interner::default();
        let package = KindId(i.intern("package"));
        let nginx = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let nginx_full = EntityRef::Operand(OpaqueToken(i.intern("nginx-full")));

        let honest = Host::new([])
            .with_resolution(package, nginx, nginx)
            .with_resolution(package, nginx_full, nginx); // both → nginx (merge)
        assert_eq!(honest.resolve(package, nginx), Some(nginx));
        assert_eq!(
            honest.resolve(package, nginx_full),
            Some(nginx),
            "an honest resolver merges the alias to the canonical"
        );
        assert!(honest.resolver_kinds().any(|k| k == package));

        let lying = Host::new([])
            .with_resolution(package, nginx, nginx)
            .with_resolution(package, nginx_full, nginx_full); // kept apart
        assert_eq!(
            lying.resolve(package, nginx_full),
            Some(nginx_full),
            "a lying resolver keeps the alias apart (identity — the under-execute reopens)"
        );

        // An unmodeled coordinate ⇒ None (a resolver-bearing kind's unresolved coord ⇒ may-alias).
        assert!(
            Host::new([])
                .with_resolver_kind(package)
                .resolve(package, nginx)
                .is_none(),
            "a declared resolver-bearing kind with no entry ⇒ None (may-alias, §3a)"
        );
    }

    #[test]
    fn query_is_inert_in_both_phases() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let mut host = Host::new([nginx]);
        assert_eq!(
            host.run(Phase::Probe, HostOp::Query(nginx)),
            Some(Verdict::Converged)
        );
        assert_eq!(
            host.run(Phase::Apply, HostOp::Query(nginx)),
            Some(Verdict::Converged)
        );
        assert!(host.violations().is_empty(), "a query never violates");
    }

    #[test]
    fn dst_plan_skips_match_the_modeled_host_over_seeds() {
        // Integration + DST: drive the REAL pipeline (parse → cfg → classify →
        // plan) with the modeled host as the probe. Invariant per seed: the FIRST ambient
        // install (nginx) is Skipped iff the host holds its fact (skip ⟺ converged). The
        // SECOND (curl) is Skipped iff curl converged AND nginx converged — because a
        // DIVERGED nginx install RUNS, and by silence=wall (`23Ib-fd10`) a running modeled
        // mutator walls every downstream elide-license, demoting the curl Replace→Run. The
        // un-oracled `systemctl reload` always runs. Looping seeds fuzzes the four
        // host states, reproducibly, with no network.
        use dorc_core::ProviderId;
        use dorc_oracle::{KindIndex, ValueClaim};

        let src = "apt-get install -y nginx\napt-get install -y curl\nsystemctl reload nginx\n";
        for seed in 0..64u64 {
            let mut i = Interner::default();
            let package = KindId(i.intern("package"));
            let installed = SelectorId(i.intern("installed"));
            let apt = ProviderId(i.intern("apt-get"));
            let install = i.intern("install");
            let mut idx = KindIndex::default();
            idx.add_effect(apt, install, package, installed, ValueClaim::Establish);

            let cell = |i: &mut Interner, e: &str| FactKey {
                kind: package,
                entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
                selector: installed,
            };
            let nginx = cell(&mut i, "nginx");
            let curl = cell(&mut i, "curl");
            let host = Host::seeded(seed, &[nginx, curl]);

            let parsed = dorc_syntax::parse(src);
            let cfg = dorc_analysis::cfg::build(&parsed.value).value;
            let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
            let mut arena = dorc_core::ProvArena::new();
            let plan = dorc_plan::build_plan(
                src,
                &parsed.value,
                &cfg,
                &classes,
                &vouch_all(&classes),
                |f| host.observe(f),
                &mut arena,
            );

            let is_skipped = |needle: &str| {
                plan.steps
                    .iter()
                    .find(|s| s.sh.contains(needle))
                    .is_some_and(|s| matches!(s.disposition, dorc_plan::Disposition::Replace(_, _)))
            };
            assert_eq!(
                is_skipped("install -y nginx"),
                host.verdict(nginx) == Verdict::Converged,
                "seed {seed}: nginx skip ⟺ host holds nginx"
            );
            assert_eq!(
                is_skipped("install -y curl"),
                host.verdict(curl) == Verdict::Converged
                    && host.verdict(nginx) == Verdict::Converged,
                "seed {seed}: curl skip ⟺ curl held AND nginx held (a diverged nginx install \
                 RUNS and walls the curl elision — silence=wall, 23Ib-fd10)"
            );
            let reload_runs = plan
                .steps
                .iter()
                .find(|s| s.sh.contains("systemctl reload"))
                .is_some_and(|s| matches!(s.disposition, dorc_plan::Disposition::Run));
            assert!(reload_runs, "seed {seed}: un-oracled reload always runs");
        }
    }

    #[test]
    fn dst_apply2_chain_probe_simulate_elide_over_seeds() {
        // apply-2 end-to-end — the WHOLE compiler chain with NO executor (the human's
        // split): source → analyze → compile_probe → SIMULATE the probe against the
        // seeded host → build_plan from those verdicts → the eliding apply. Per seed:
        // the FIRST install (nginx) is elided iff the host holds its fact; the SECOND (curl)
        // is elided iff curl held AND nginx held — a DIVERGED nginx install RUNS and, by
        // silence=wall (`23Ib-fd10`), walls the downstream curl elision (Replace→Run). The
        // un-oracled reload always runs. Looping seeds fuzzes the host states, reproducibly,
        // no network.
        use dorc_core::ProviderId;
        use dorc_oracle::{KindIndex, ValueClaim};
        use dorc_plan::{Disposition, build_plan, compile_probe};

        let src = "apt-get install -y nginx\napt-get install -y curl\nsystemctl reload nginx\n";
        for seed in 0..64u64 {
            let mut i = Interner::default();
            let package = KindId(i.intern("package"));
            let installed = SelectorId(i.intern("installed"));
            let apt = ProviderId(i.intern("apt-get"));
            let install = i.intern("install");
            let mut idx = KindIndex::default();
            idx.add_effect(apt, install, package, installed, ValueClaim::Establish);

            let cell = |i: &mut Interner, e: &str| FactKey {
                kind: package,
                entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
                selector: installed,
            };
            let nginx = cell(&mut i, "nginx");
            let curl = cell(&mut i, "curl");
            let host = Host::seeded(seed, &[nginx, curl]);

            let parsed = dorc_syntax::parse(src);
            let cfg = dorc_analysis::cfg::build(&parsed.value).value;
            let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
            let checks =
                vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
            let classes = dorc_analysis::effect::classify(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &mut i,
                &mut dorc_core::ProvArena::new(),
            )
            .value;

            // (1) compile the SITE-keyed probe — R3: ship the provider's stripped check body
            // invoked per-site with the site's argv (`inv-site-keyed-results`, round-20 task-D1).
            let probe = compile_probe(
                &parsed.value,
                &cfg,
                &value,
                &classes,
                |provider, argv| ship_corpus(&checks, &i, provider, argv),
                // hostsim exercises elision soundness, not guards — no vouched past-wall probes.
                |_| false,
            );
            assert!(
                probe.checks_fact(nginx) && probe.checks_fact(curl),
                "seed {seed}: both ambient installs are probed (package has a probe)"
            );
            // …and the probe renders as a read-only, self-reporting shell-script.
            let probe_sh = probe.render_sh(&i);
            assert!(
                probe_sh.contains("dpkg-query") && probe_sh.contains("read-only"),
                "seed {seed}: probe renders the verbatim read-only check"
            );

            // (2) SIMULATE — the SITE→CELL bridge (round-20 task-D1): the host answers
            // each probed SITE by mapping it to its resolved cell (the probe's
            // `checks` carry site→fact) and asking the cell-keyed fact-store. This is
            // the DST stand-in for running the rendered probe and reading back its
            // `site <id> effect=…` records. Re-key to the fact-keyed observations
            // `build_plan` consumes (only the probe-answer plumbing re-keys; the
            // fact-store stays cell-keyed). An unprobed site/fact ⇒ Unknown ⇒ run.
            let mut by_fact: BTreeMap<FactKey, Observable> = BTreeMap::new();
            for check in &probe.checks {
                by_fact.insert(check.fact, host.observe(check.fact));
            }
            let observe = |f: FactKey| {
                by_fact
                    .get(&f)
                    .copied()
                    .unwrap_or(Observable::verdict_only(Verdict::Unknown))
            };
            // (3) compile the eliding apply from the simulated probe results.
            let apply = build_plan(
                src,
                &parsed.value,
                &cfg,
                &classes,
                &vouch_all(&classes),
                observe,
                &mut dorc_core::ProvArena::new(),
            );

            let elided = |needle: &str| {
                apply
                    .steps
                    .iter()
                    .find(|s| s.sh.contains(needle))
                    .is_some_and(|s| matches!(s.disposition, Disposition::Replace(_, _)))
            };
            assert_eq!(
                elided("install -y nginx"),
                host.verdict(nginx) == Verdict::Converged,
                "seed {seed}: nginx elided ⟺ host holds nginx"
            );
            assert_eq!(
                elided("install -y curl"),
                host.verdict(curl) == Verdict::Converged
                    && host.verdict(nginx) == Verdict::Converged,
                "seed {seed}: curl elided ⟺ host holds curl AND nginx (a diverged nginx install \
                 RUNS and walls the curl elision — silence=wall, 23Ib-fd10)"
            );
            let reload_runs = apply
                .steps
                .iter()
                .find(|s| s.sh.contains("systemctl reload"))
                .is_some_and(|s| matches!(s.disposition, Disposition::Run));
            assert!(reload_runs, "seed {seed}: un-oracled reload always runs");
        }
    }

    #[test]
    fn apply2_unprobeable_fact_is_not_elided() {
        // can't-probe ⇒ can't-elide: a kind with an EFFECT but NO declared probe is
        // omitted from the probe ⇒ the apply runs its install even on a host that
        // HOLDS the fact (kFAIL-perform — no convergence knowledge ⇒ run).
        use dorc_core::ProviderId;
        use dorc_oracle::{KindIndex, ValueClaim};
        use dorc_plan::{Disposition, build_plan, compile_probe};

        let mut i = Interner::default();
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let apt = ProviderId(i.intern("apt-get"));
        let install = i.intern("install");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, ValueClaim::Establish); // one effect cell

        let nginx = FactKey {
            kind: package,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: installed,
        };
        let host = Host::new([nginx]); // the host HOLDS nginx (converged)

        let src = "apt-get install -y nginx\n";
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &[dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value],
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;

        // R3: no shippable probe (the ship closure returns None — "the oracle declares no
        // probe") ⇒ the EstablishAmbient site is unresolvable ⇒ not elided (kFAIL-perform).
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            |_provider, _argv| None,
            |_| false,
        );
        assert!(
            probe.checks.is_empty(),
            "no declared probe ⇒ no resolvable site (the install is recorded unresolvable)"
        );
        assert_eq!(
            probe.unresolvable.len(),
            1,
            "the un-probeable install site is recorded unresolvable (can't-probe ⇒ can't-elide)"
        );

        let observe = |f: FactKey| {
            if probe.checks_fact(f) {
                host.observe(f)
            } else {
                Observable::verdict_only(Verdict::Unknown)
            }
        };
        let apply = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            // The site is unprobeable ⇒ Unknown ⇒ runs regardless of any vouch; empty is honest.
            &dorc_plan::Vouches::new(),
            observe,
            &mut dorc_core::ProvArena::new(),
        );
        assert!(
            matches!(apply.steps[0].disposition, Disposition::Run),
            "un-probeable fact must run even though the host holds it"
        );
        assert!(
            apply.render_sh(&i).contains("apt-get install -y nginx"),
            "the un-elided install renders verbatim in the apply sh"
        );
    }
}
