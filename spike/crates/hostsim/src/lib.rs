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

use std::collections::BTreeSet;

use dorc_analysis::effect::FactKey;
use dorc_core::{Phase, Verdict};

/// A tiny deterministic linear-congruential PRNG — the host's seeded
/// nondeterminism. Hand-rolled (no `rand` dependency): the DST host must be
/// reproducible bit-for-bit from its seed, and the kernel stays dep-free. The
/// multiplier/increment are the common 64-bit LCG constants (Knuth/PCG lineage).
#[derive(Debug, Clone)]
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// A coin flip true with probability `num / den` (deterministic given the seed).
    fn chance(&mut self, num: u32, den: u32) -> bool {
        den != 0 && (self.next_u64() % u64::from(den)) < u64::from(num)
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

/// A seeded, deterministic model of a target host: the set of facts that currently
/// hold, plus the `kFAIL-withhold` monitor.
#[derive(Debug, Clone)]
pub struct Host {
    facts: BTreeSet<FactKey>,
    violations: Vec<Violation>,
}

impl Host {
    /// A host whose initial state is exactly `holding` (no PRNG — the state is
    /// given, not generated).
    #[must_use]
    pub fn new(holding: impl IntoIterator<Item = FactKey>) -> Self {
        Host {
            facts: holding.into_iter().collect(),
            violations: Vec::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{Interner, KindId, OpaqueToken};

    fn fk(i: &mut Interner, kind: &str, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern(kind)),
            entity: OpaqueToken(i.intern(entity)),
        }
    }

    #[test]
    fn verdict_reflects_modeled_state() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let curl = fk(&mut i, "package", "curl");
        let host = Host::new([nginx]);
        assert_eq!(host.verdict(nginx), Verdict::Converged, "held fact ⇒ converged");
        assert_eq!(host.verdict(curl), Verdict::Diverged, "absent fact ⇒ diverged");
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
        assert_eq!(host.verdict(nginx), Verdict::Diverged, "probe mutation must NOT take effect");
        assert_eq!(host.violations().len(), 1, "the withhold breach is recorded");
        assert_eq!(host.violations()[0].op, HostOp::Establish(nginx));
    }

    #[test]
    fn apply_phase_mutation_takes_effect_no_violation() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let mut host = Host::new([]);
        host.run(Phase::Apply, HostOp::Establish(nginx));
        assert_eq!(host.verdict(nginx), Verdict::Converged, "apply establish takes effect");
        assert!(host.violations().is_empty(), "apply mutation is legitimate");
        host.run(Phase::Apply, HostOp::Kill(nginx));
        assert_eq!(host.verdict(nginx), Verdict::Diverged, "apply kill takes effect");
    }

    #[test]
    fn query_is_inert_in_both_phases() {
        let mut i = Interner::default();
        let nginx = fk(&mut i, "package", "nginx");
        let mut host = Host::new([nginx]);
        assert_eq!(host.run(Phase::Probe, HostOp::Query(nginx)), Some(Verdict::Converged));
        assert_eq!(host.run(Phase::Apply, HostOp::Query(nginx)), Some(Verdict::Converged));
        assert!(host.violations().is_empty(), "a query never violates");
    }

    #[test]
    fn dst_plan_skips_match_the_modeled_host_over_seeds() {
        // Integration + DST: drive the REAL pipeline (parse → cfg → classify →
        // plan) with the modeled host as the probe. Invariant per seed: an ambient
        // install is Skipped iff the host holds its fact (skip ⟺ converged); the
        // un-oracled `systemctl reload` always runs. Looping seeds fuzzes the four
        // host states, reproducibly, with no network.
        use dorc_core::ProviderId;
        use dorc_oracle::{KindIndex, Polarity};

        let src = "apt-get install -y nginx\napt-get install -y curl\nsystemctl reload nginx\n";
        for seed in 0..64u64 {
            let mut i = Interner::default();
            let package = KindId(i.intern("package"));
            let apt = ProviderId(i.intern("apt-get"));
            let install = i.intern("install");
            let mut idx = KindIndex::default();
            idx.add_effect(apt, install, package, Polarity::Establish);

            let nginx = FactKey { kind: package, entity: OpaqueToken(i.intern("nginx")) };
            let curl = FactKey { kind: package, entity: OpaqueToken(i.intern("curl")) };
            let host = Host::seeded(seed, &[nginx, curl]);

            let parsed = dorc_syntax::parse(src);
            let cfg = dorc_analysis::cfg::build(&parsed.value).value;
            let classes = dorc_analysis::effect::classify(&cfg, &parsed.value, &idx, &mut i);
            let plan = dorc_plan::build_plan(src, &parsed.value, &cfg, &classes, |f| host.verdict(f));

            let is_skipped = |needle: &str| {
                plan.steps
                    .iter()
                    .find(|s| s.sh.contains(needle))
                    .is_some_and(|s| matches!(s.disposition, dorc_plan::Disposition::Replace(_)))
            };
            assert_eq!(
                is_skipped("install -y nginx"),
                host.verdict(nginx) == Verdict::Converged,
                "seed {seed}: nginx skip ⟺ host holds nginx"
            );
            assert_eq!(
                is_skipped("install -y curl"),
                host.verdict(curl) == Verdict::Converged,
                "seed {seed}: curl skip ⟺ host holds curl"
            );
            let reload_runs = plan
                .steps
                .iter()
                .find(|s| s.sh.contains("systemctl reload"))
                .is_some_and(|s| matches!(s.disposition, dorc_plan::Disposition::Run));
            assert!(reload_runs, "seed {seed}: un-oracled reload always runs");
        }
    }
}
