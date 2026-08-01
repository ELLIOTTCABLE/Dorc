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

    /// A coin flip true with probability `num / den` (deterministic given the seed). Routes through
    /// the HIGH-bit [`below`](Lcg::below), NOT a low-bit `next_u64() % den`: an odd-multiplier LCG's
    /// low bits are periodic, so consecutive `% den` coins deterministically correlate — the
    /// find-lcg-thinning bug (24C) that made [`Host::seeded`]'s ½-subset a patterned slice of the
    /// 2^N state-space. Mirrors `differential.rs`'s `Rng::chance` (the 21D-triage fix).
    pub fn chance(&mut self, num: u32, den: u32) -> bool {
        den != 0 && self.below(u64::from(den)) < u64::from(num)
    }

    /// A draw in `0..bound` (deterministic; `0` for `bound == 0`), taken from the HIGH bits via
    /// Lemire's multiply-high. This is load-bearing, NOT decorative: an odd-multiplier LCG's LOW
    /// bits are periodic (the low bit flips every step, the low `k` bits have period `2^k`), so a
    /// naive `next_u64() % small` makes consecutive small-modulus draws deterministically
    /// correlate. The round-24 sweep draws EVERY axis through here so its
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

/// The `262` §2 / §5 byte-tier fault vocabulary for the records lane: seeded, deterministic
/// mutations of a framed record stream that a best-effort pipe can inflict. `model-the-outcome`
/// (mutate the bytes, never a real kernel/pipe); the DST feeds the RESULT through the PRODUCTION
/// deframer and asserts the safe direction (loss/refusal, never a fabricated shrunken record).
/// Plan-free by construction (the terminal token is passed IN) so `hostsim`'s kernel stays clean.
pub mod fault {
    use super::Lcg;

    /// Which fault a seed selected (for the DST's `sometimes-assert` reachability).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RecordFault {
        /// A record lost its terminating write ⇒ no terminal token (a prefix-truncated coordinate
        /// necessarily lands here — it can never parse as a shorter valid record).
        Torn,
        /// Two atomic writes merged ⇒ bytes after the first token (the deframer refuses the unit).
        Glued,
        /// A `>PIPE_BUF` line: content inflated but still terminated (WIDENS, the safe direction).
        Oversize,
        /// The seed left the stream clean (the negative control).
        Clean,
    }

    /// Apply one seeded fault to a framed record `stream`. `token` is the terminal token to
    /// tear/inflate around. Deterministic in `seed`.
    #[must_use]
    pub fn mutate(seed: u64, stream: &str, token: &str) -> (String, RecordFault) {
        let mut rng = Lcg::new(seed);
        let mut lines: Vec<String> = stream.lines().map(str::to_owned).collect();
        let framed: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.trim_end().ends_with(token))
            .map(|(i, _)| i)
            .collect();
        if framed.is_empty() {
            return (stream.to_owned(), RecordFault::Clean);
        }
        let class = match rng.below(4) {
            0 => RecordFault::Torn,
            1 => RecordFault::Glued,
            2 => RecordFault::Oversize,
            _ => RecordFault::Clean,
        };
        // Pick one framed line index (safe: `framed` is non-empty; `below` stays in-range).
        let pick = |rng: &mut Lcg, from: &[usize]| -> Option<usize> {
            from.get(usize::try_from(rng.below(from.len() as u64)).unwrap_or(0))
                .copied()
        };
        match class {
            RecordFault::Torn => {
                if let Some(i) = pick(&mut rng, &framed)
                    && let Some(l) = lines.get_mut(i)
                {
                    *l = strip_token(l, token);
                }
            }
            RecordFault::Glued => {
                // Join a framed line with its SUCCESSOR (drop the newline between) ⇒ the first
                // token gets trailing bytes. Fall back to Clean if the pick has no successor.
                let cands: Vec<usize> = framed
                    .iter()
                    .copied()
                    .filter(|&i| i.checked_add(1).is_some_and(|n| n < lines.len()))
                    .collect();
                let Some(i) = pick(&mut rng, &cands) else {
                    return (stream.to_owned(), RecordFault::Clean);
                };
                let j = i.saturating_add(1);
                if let (Some(a), Some(b)) = (lines.get(i).cloned(), lines.get(j).cloned()) {
                    if let Some(l) = lines.get_mut(i) {
                        *l = format!("{a}{b}");
                    }
                    lines.remove(j);
                }
            }
            RecordFault::Oversize => {
                if let Some(i) = pick(&mut rng, &framed)
                    && let Some(l) = lines.get_mut(i)
                {
                    let pad = "x".repeat(9000); // comfortably > PIPE_BUF
                    *l = format!("{} {pad} {token}", strip_token(l, token));
                }
            }
            RecordFault::Clean => {}
        }
        (lines.join("\n") + "\n", class)
    }

    /// Remove the trailing ` {token}` (and any trailing whitespace) from a framed line.
    fn strip_token(line: &str, token: &str) -> String {
        line.trim_end()
            .strip_suffix(token)
            .unwrap_or(line)
            .trim_end()
            .to_owned()
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
    /// The DECLARED reach answer (24G §4 — the cross-author footprint-EXPANSION mechanism): per
    /// `(kind, entity)` coordinate, the set of `(kind, entity)` coordinates its `<kind>.reaches()`
    /// DRAGS WITH IT (a package reaches its files / same-named unit). The SAME shape discipline as
    /// [`manifests`]/[`resolutions`]: DECLARED scenario data, deterministic, no ssh, NOT a `dpkg -L`
    /// simulation — a declared-data oracle standing in for shipping the `reaches()` dynamic-arm body and
    /// reading its per-arm stdout. Rides the sweep's declared-vs-TRUE split for REACH (24G soundness
    /// net): the generator invents a TRUE reach (the wall's `CellDelta` really touches a reached cell)
    /// AND a declared reach answer; an HONEST answer INCLUDES the truly-reached coord (the expansion
    /// HITs ⇒ the victim DEMOTES ⇒ safe); a LYING answer OMITS it (the expansion misses ⇒ the victim
    /// wrongly survives ⇒ the end-state differential goes RED). Omission is THE sharp edge.
    reaches: BTreeMap<(KindId, EntityRef), BTreeSet<(KindId, EntityRef)>>,
    /// The facts whose probe RACED to rc-141 this run (`sigpipe-flap-class`, `279f` §5): a
    /// `pipefail`-off composed pipe whose early-exit consumer (`… | grep -q`) closed the pipe
    /// before an upstream stage finished writing. A raced fact [`observe`](Host::observe)s
    /// [`Verdict::Unknown`] — the ≥2 flat-sink (cant-tell) landing — INSTEAD of its true verdict,
    /// modelling the SIGPIPE outcome at the seam (`model-the-outcome`, never `tc`/real SIGPIPE).
    /// Seeded + precomputed by [`with_sigpipe_race`](Host::with_sigpipe_race) so it is bit-for-bit
    /// reproducible and goldens cannot flap. Empty on every ordinary host (no behaviour change).
    sigpipe_raced: BTreeSet<FactKey>,
    /// The connection's mechanical [`Capability`] (`27C` §1(1) axis 1 — CAN the connection effect a
    /// shift, with zero new credentials). A host FACT the probe never self-acquires: injected here
    /// in DST (`with_capability`), the cli edge in reality. The consent decision
    /// (`dorc_oracle::entry::decide_entry`) reads it to gate context entry. Defaults to
    /// [`Capability::Root`] (the spike posture); a degraded/NOPASSWD host is a distinct cell to fuzz.
    capability: dorc_core::Capability,
}

impl Host {
    /// A host whose initial state is exactly `holding` (no PRNG — the state is
    /// given, not generated). Context-qualified injection (`27C` §3 / `plans/27C` §9): a fact
    /// keyed [`dorc_core::Context::Wrapped`] models "in THIS wrapper-denoted world, the cell holds"
    /// — it is a DISTINCT [`FactKey`] from the ambient cell, so [`verdict`](Host::verdict) answers
    /// the two independently (a wrapped measurement never aliases the ambient one). No new
    /// mechanism: the context rides the fact, so `Host::new([cell.in_context(ctx)])` just works.
    #[must_use]
    pub fn new(holding: impl IntoIterator<Item = FactKey>) -> Self {
        Host {
            facts: holding.into_iter().collect(),
            violations: Vec::new(),
            manifests: BTreeMap::new(),
            resolutions: BTreeMap::new(),
            resolver_kinds: BTreeSet::new(),
            reaches: BTreeMap::new(),
            sigpipe_raced: BTreeSet::new(),
            capability: dorc_core::Capability::Root,
        }
    }

    /// Inject the connection's mechanical [`Capability`] (`27C` §1(1) — the capability-cell
    /// injection: root / NOPASSWD-non-root / degraded). Consuming-builder shape so a scenario spells
    /// its capability inline. The probe NEVER self-acquires (`27C` §1): this is a declared host fact,
    /// exactly the `verdict`/`manifest`/`resolve` declared-data discipline.
    #[must_use]
    pub fn with_capability(mut self, capability: dorc_core::Capability) -> Self {
        self.capability = capability;
        self
    }

    /// The connection's declared mechanical [`Capability`] (`27C` §1(1)) — read by the consent
    /// decision at the cli edge. `Root` unless [`with_capability`](Host::with_capability) set otherwise.
    #[must_use]
    pub fn capability(&self) -> dorc_core::Capability {
        self.capability
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

    /// Attach a DECLARED reach answer (24G §4): the `(kind, entity)` coordinate REACHES `reached`
    /// (the coords its `<kind>.reaches()` drags with it) when the reach-function runs host-side.
    /// Consuming-builder shape so a scenario spells its reach map inline. An HONEST answer INCLUDES
    /// the wall's truly-reached coord (the expansion HITs ⇒ demote ⇒ safe); a LYING one OMITS it
    /// (the expansion misses ⇒ the victim wrongly survives — the sweep net's RED).
    #[must_use]
    pub fn with_reach(
        mut self,
        kind: KindId,
        entity: EntityRef,
        reached: impl IntoIterator<Item = (KindId, EntityRef)>,
    ) -> Self {
        self.reaches
            .insert((kind, entity), reached.into_iter().collect());
        self
    }

    /// The DECLARED reach answer for a `(kind, entity)` coordinate (24G §4) — the reach analogue of
    /// [`derive`](Host::derive)/[`resolve`](Host::resolve). Deterministic, scenario-driven, no ssh,
    /// no `dpkg -L` simulation (a declared-data oracle). An unmodeled coordinate yields the EMPTY set
    /// (no declaration ⇒ no expansion ⇒ the footprint is unchanged — the reach-less floor). Same
    /// declared-vs-true discipline as [`derive`](Host::derive): an honest answer ⊇ the wall's true
    /// reach; a lying one omits a truly-reached coord (⇒ the victim wrongly survives).
    #[must_use]
    pub fn reach(&self, kind: KindId, entity: EntityRef) -> BTreeSet<(KindId, EntityRef)> {
        self.reaches
            .get(&(kind, entity))
            .cloned()
            .unwrap_or_default()
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
            reaches: BTreeMap::new(),
            sigpipe_raced: BTreeSet::new(),
            capability: dorc_core::Capability::Root,
        }
    }

    /// Inject a seeded `sigpipe-flap-class` race (`279f` §5) onto `flappy` facts: each independently
    /// RACES to rc-141 with a deterministic coin drawn from `seed`. A raced fact
    /// [`observe`](Host::observe)s [`Verdict::Unknown`] (the ≥2 flat-sink / cant-tell landing) instead
    /// of its true verdict — the DST stand-in for a `pipefail`-off composed pipe whose early-exit
    /// `| grep -q` consumer closed the pipe before an upstream stage finished writing. Seeded so the
    /// outcome is bit-for-bit reproducible (goldens cannot flap); the coin is drawn per fact in the
    /// slice's order (`model-the-outcome`: inject the OUTCOME at the seam, never real SIGPIPE). The
    /// landing is always SAFE (Unknown ⇒ can't-elide ⇒ run), so injecting it can only DEMOTE elisions.
    #[must_use]
    pub fn with_sigpipe_race(mut self, seed: u64, flappy: &[FactKey]) -> Self {
        let mut rng = Lcg::new(seed);
        self.sigpipe_raced = flappy
            .iter()
            .copied()
            .filter(|_| rng.chance(1, 2))
            .collect();
        self
    }

    /// Whether `fact`'s probe RACED to rc-141 this run (`sigpipe-flap-class`) — the reachability
    /// probe a DST asserts on so a seed-set that never fires the race fails loudly (`sometimes-assert`).
    #[must_use]
    pub fn sigpipe_raced(&self, fact: FactKey) -> bool {
        self.sigpipe_raced.contains(&fact)
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
        // sigpipe-flap-class (`279f` §5): a raced fact lands rc-141 ⇒ the ≥2 flat-sink (cant-tell)
        // ⇒ `Verdict::Unknown` — NOT its true membership. Safe (Unknown ⇒ can't-elide ⇒ run) and
        // deterministic (the raced set is seeded), so the verdict never flaps run-to-run.
        if self.sigpipe_raced.contains(&fact) {
            return Observable::verdict_only(Verdict::Unknown);
        }
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

    #[test]
    fn context_qualified_verdict_injection_answers_worlds_independently() {
        // `27C` §3 / `plans/27C` §9 — context-qualified verdict injection: a fact keyed in a
        // wrapper-denoted world (`Context::Wrapped`) is a DISTINCT `FactKey`, so the host answers it
        // INDEPENDENTLY of the ambient cell. Model the babby-sudo story: the ROOT tree has poddle
        // installed, the ambient (caller's) tree does NOT. The wrapped measurement converges; the
        // ambient one diverges — nothing traveled between the two worlds.
        let mut i = Interner::default();
        let cell = FactKey::cell(
            KindId(i.intern("pipx.Package")),
            EntityRef::Operand(OpaqueToken(i.intern("poddle"))),
            SelectorId(i.intern("installed")),
        );
        let root_ctx = dorc_core::Context::Wrapped(dorc_core::ContextKey(i.intern("user=root")));
        let in_root = cell.in_context(root_ctx);
        // The host holds ONLY the root-context fact (poddle is in root's tree, not the caller's).
        let host = Host::new([in_root]);
        assert_eq!(
            host.verdict(in_root),
            Verdict::Converged,
            "poddle IS installed in root's world"
        );
        assert_eq!(
            host.verdict(cell),
            Verdict::Diverged,
            "poddle is NOT installed in the ambient world — no transport across the context gap"
        );
    }

    #[test]
    fn capability_cell_injection_defaults_root() {
        // `27C` §1(1) — the capability-cell injection (root / NOPASSWD / degraded). Default = root;
        // a scenario injects a degraded/NOPASSWD host to fuzz the consent cells.
        use dorc_core::Capability;
        assert_eq!(
            Host::new([]).capability(),
            Capability::Root,
            "spike default"
        );
        assert_eq!(
            Host::new([])
                .with_capability(Capability::Degraded)
                .capability(),
            Capability::Degraded
        );
        assert_eq!(
            Host::new([])
                .with_capability(Capability::NonRootNopasswd)
                .capability(),
            Capability::NonRootNopasswd
        );
    }

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
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ; fi
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
            if let SkipClass::EstablishAmbient(fact) = class {
                let vouch = dorc_plan::VerdictVouch::new(
                    "apt_get__is_converged".to_string(),
                    "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
                    "apt_get__is_converged".to_string(),
                    "package".to_string(),
                    vec!["dpkg-query".to_string()],
                    dorc_core::DefinitionCustody::of_defining_file(dorc_core::SourceFileId(0)),
                );
                vouches.insert(
                    *node,
                    *fact,
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
    ) -> Option<dorc_plan::ShippedCheck> {
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
                    return Some(dorc_plan::ShippedCheck::predict(
                        strip_predict(CORPUS_PREDICT_SRC, check, interner),
                        Some((check.name_span, dorc_core::SourceFileId(0))),
                    ));
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
        dorc_analysis::effect::classify(
            cfg,
            &value,
            ast,
            idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            i,
            &mut arena,
        )
        .value
    }

    /// `kind:entity@installed` — the re-keyed cell (`notes/193`). These host-model
    /// tests only ever exercise `package@installed`, so the selector is fixed here;
    /// the host is a plain set-membership oracle over whatever `FactKey` it is given.
    fn fk(i: &mut Interner, kind: &str, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern(kind)),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("installed")),
            context: dorc_core::Context::HostDefault,
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
    fn seeded_coins_decorrelate_so_the_full_subset_lattice_is_reachable() {
        // find-lcg-thinning regression pin (24C). `Host::seeded` draws one `chance(1,2)` coin per
        // candidate. The old `% 2` read the LCG's LOW bit, which flips every step (odd multiplier +
        // odd increment), so consecutive coins PERFECTLY alternated: for two candidates exactly one
        // was ever included, and the {both}/{neither} corners of the 2^N subset-space were
        // UNREACHABLE for every seed. Routing `chance` through the high-bit `below` decorrelates the
        // coins, so all four membership cells occur across seeds. A regression to the low-bit draw
        // makes {both} and {neither} vanish ⇒ this fails.
        let mut i = Interner::default();
        let a = fk(&mut i, "package", "nginx");
        let b = fk(&mut i, "package", "curl");

        let (mut both, mut neither, mut only_a, mut only_b) = (false, false, false, false);
        for seed in 0..128u64 {
            let host = Host::seeded(seed, &[a, b]);
            match (
                host.verdict(a) == Verdict::Converged,
                host.verdict(b) == Verdict::Converged,
            ) {
                (true, true) => both = true,
                (false, false) => neither = true,
                (true, false) => only_a = true,
                (false, true) => only_b = true,
            }
        }
        assert!(
            both && neither && only_a && only_b,
            "seeded ½-subset coins are correlated — not all four membership cells reached over 128 \
             seeds (both={both} neither={neither} only_a={only_a} only_b={only_b}); the low-bit \
             `% 2` thinning regressed (find-lcg-thinning, 24C)"
        );
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
            let apt = ProviderId(i.intern("apt_get"));
            let install = i.intern("install");
            let mut idx = KindIndex::default();
            idx.add_effect(apt, install, package, installed, ValueClaim::Establish);

            let cell = |i: &mut Interner, e: &str| {
                let ent = EntityRef::Operand(OpaqueToken(i.intern(e)));
                FactKey::cell(package, ent, installed)
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
            let apt = ProviderId(i.intern("apt_get"));
            let install = i.intern("install");
            let mut idx = KindIndex::default();
            idx.add_effect(apt, install, package, installed, ValueClaim::Establish);

            let cell = |i: &mut Interner, e: &str| {
                let ent = EntityRef::Operand(OpaqueToken(i.intern(e)));
                FactKey::cell(package, ent, installed)
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
                &dorc_oracle::verdict::VerdictIndex::default(),
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
                &BTreeMap::new(),
                // hostsim's corpus has no all-Query pipeline ⇒ no connected check-pipe (24J §2);
                // default keeps compile_probe consistent with the `build_plan` wrapper it uses below.
                &dorc_plan::ConnectedPipes::default(),
                |_, provider, argv| ship_corpus(&checks, &i, provider, argv),
                |_, _, _| None,
                // hostsim exercises elision soundness, not guards — no vouched past-wall probes.
                |_| false,
            );
            assert!(
                probe.checks_fact(nginx) && probe.checks_fact(curl),
                "seed {seed}: both ambient installs are probed (package has a probe)"
            );
            // …and the probe renders as a read-only, self-reporting shell-script.
            let probe_sh = probe.render_sh(&dorc_plan::records::Framing::spike(String::new()), &i);
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
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, ValueClaim::Establish); // one effect cell

        let nginx = FactKey {
            kind: package,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: installed,
            context: dorc_core::Context::HostDefault,
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
            &dorc_oracle::verdict::VerdictIndex::default(),
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
            &BTreeMap::new(),
            &dorc_plan::ConnectedPipes::default(),
            |_, _provider, _argv| None,
            |_, _, _| None,
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

    /// A connected check-pipe corpus (`271:rul-only-oracle-bytes-ship`): two read-only Query stages
    /// that DELEGATE (real stdout), so the pipe ships as a COMPOSED probe. The A6 otelcol shape +
    /// the stdlib grep, one source (`lift`/`lift_predicts` handle multiple funcdefs).
    const CONNECTED_ORACLE: &str = r#"
otelcol__predict() {
   case $1 in
      --version)
         collector : io.opentelemetry.Collector = "otelcol"
         otelcol --version :? io.opentelemetry.Collector:"otelcol"@version
         ;;
   esac
}
grep__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   pat : sm.dorc.GrepMatch = "$1"
   grep -q -- "$pat" :? sm.dorc.GrepMatch:"$pat"@matched
}
"#;

    /// Resolve a connected pipe STAGE's stripped predict + stdout coverage from a given oracle `src`
    /// (mirror of the cli's `ship_predict_stage`). Reuses `predict_stage_stdout` for the coverage bit.
    fn ship_stage_from(
        src: &str,
        checks: &[dorc_oracle::predict::PredictSet],
        interner: &Interner,
        provider: dorc_core::Symbol,
        argv: &[dorc_core::Symbol],
    ) -> Option<dorc_plan::StageShip> {
        use dorc_oracle::predict::{
            Resolution, StageStdout, evaluate, map_provider_name, predict_stage_stdout,
            strip_predict,
        };
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
                    return Some(dorc_plan::StageShip {
                        sh: strip_predict(src, check, interner),
                        produces_real_stdout: predict_stage_stdout(check, &arg_refs)
                            == StageStdout::RealBytes,
                    });
                }
            }
        }
        None
    }

    /// Resolve a site's stripped predict body from a given oracle `src` (mirror of `ship_corpus`,
    /// but stripping from `src` rather than the apt-get corpus). For the ordinary `compile_probe` seam.
    fn ship_body_from(
        src: &str,
        checks: &[dorc_oracle::predict::PredictSet],
        interner: &Interner,
        provider: dorc_core::Symbol,
        argv: &[dorc_core::Symbol],
    ) -> Option<dorc_plan::ShippedCheck> {
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
                    return Some(dorc_plan::ShippedCheck::predict(
                        strip_predict(src, check, interner),
                        Some((check.name_span, dorc_core::SourceFileId(0))),
                    ));
                }
            }
        }
        None
    }

    #[test]
    fn dst_composed_probe_under_sigpipe_race_lands_in_sink_without_flapping() {
        // `279f` §5 / sigpipe-flap-class: a COMPOSED connected probe (`otelcol__predict |
        // grep__predict`) whose governing fact's probe RACES to rc-141 lands in the ≥2 flat sink
        // (cant-tell ⇒ Unknown). Three properties over seeds: (1) the race is SEEDED/deterministic
        // (no flap — two builds at one seed render byte-identical); (2) a raced governing verdict
        // is the ≥2 sink AND the pipe RUNS (never a wrong elision — the safe landing); (3) the race
        // SOMETIMES fires and sometimes not (`sometimes-assert` reachability). The composed-probe
        // SHAPE itself is pinned in plan's unit tests; this pins the race's SAFE interaction with it.
        use dorc_plan::{Disposition, build_plan_walled, compile_probe, connected_check_pipes};

        let book = "otelcol --version | grep -q 0.155.0 || curl https://example.com/x.tar.gz\n";
        let mut i = Interner::default();
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let idx = dorc_oracle::lift(&mut i, &[CONNECTED_ORACLE]).value;
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CONNECTED_ORACLE).value];
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;

        let connected = connected_check_pipes(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            |_n, p, a: &[dorc_core::Symbol]| ship_stage_from(CONNECTED_ORACLE, &checks, &i, p, a),
        );
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &connected,
            |_n, p, a: &[dorc_core::Symbol]| ship_body_from(CONNECTED_ORACLE, &checks, &i, p, a),
            |_, _, _| None,
            |_| false,
        );

        // The composed probe exists: the connected recognition + compose worked (ONLY oracle bytes).
        let gov = probe
            .checks
            .iter()
            .find(|c| c.connected.is_some())
            .expect("a composed connected probe ships for the all-Query delegating pipe");
        let gov_fact = gov.fact;

        let mut raced = 0u32;
        for seed in 0..64u64 {
            let host = Host::new([gov_fact]).with_sigpipe_race(seed, &[gov_fact]);
            let observe = |f: FactKey| {
                if probe.checks_fact(f) {
                    host.observe(f)
                } else {
                    Observable::verdict_only(Verdict::Unknown)
                }
            };
            let build = || {
                build_plan_walled(
                    book,
                    &parsed.value,
                    &cfg,
                    &classes,
                    &BTreeSet::new(),
                    None,
                    None,
                    &dorc_core::Dialect::empty(),
                    // Survival off (`None`) ⇒ the `277` §5 backing map is never consulted.
                    &BTreeMap::new(),
                    &dorc_plan::Vouches::new(),
                    &connected,
                    &BTreeMap::new(), // no probe-origin witnesses in DST (C6: Witness is EXEMPT)
                    observe,
                    &mut dorc_core::ProvArena::new(),
                )
            };
            // (1) no flap: two builds at the SAME seed render byte-identical.
            assert_eq!(
                build().render_sh(&i),
                build().render_sh(&i),
                "seed {seed}: the verdict must not flap run-to-run (the race is seeded)"
            );
            if host.sigpipe_raced(gov_fact) {
                raced += 1;
                // (2a) the ≥2-sink landing: a raced fact observes Unknown (cant-tell), not its
                // true membership.
                assert_eq!(
                    host.observe(gov_fact),
                    Observable::verdict_only(Verdict::Unknown),
                    "seed {seed}: a raced governing fact lands the ≥2 flat sink (Unknown)"
                );
                // (2b) SAFE: the governing grep stage RUNS (never elides) under the sink verdict.
                let plan = build();
                let gov_step = plan
                    .steps
                    .iter()
                    .find(|s| s.sh.contains("grep -q"))
                    .expect("the governing grep stage renders");
                assert!(
                    matches!(gov_step.disposition, Disposition::Run),
                    "seed {seed}: a ≥2-sink governing verdict RUNS the pipe (never a wrong elision)"
                );
            }
        }
        // (3) reachability (sometimes-assert): the seeded race must fire for SOME seeds, not all.
        assert!(
            raced > 0 && raced < 64,
            "the seeded SIGPIPE race must sometimes fire and sometimes not over 64 seeds \
             (sometimes-assert): fired {raced}/64"
        );
    }

    /// `262` §2/§5 byte-tier fault DST (THE tear-detector proof): seeded torn/glued/oversize
    /// mutations of a framed record stream, fed through bounded production admission, must fold in the
    /// SAFE direction — a torn/truncated record is DROPPED or the read unit REFUSED, never a
    /// fabricated shorter (more-licensing) record; an oversized line WIDENS (safe). Includes a
    /// space-bearing deriv coord so last-to-token is stressed under mutation. `sometimes-assert`:
    /// each fault class fires over the seed range (a mutator that never tears is a dead DST).
    #[test]
    fn dst_byte_tier_record_faults_fold_toward_safe_never_fabricate() {
        use dorc_plan::records::{
            Admission, Framing, HostEvidenceLimits, TERMINAL_TOKEN, admit_unscoped_host_records,
            read_host_evidence,
        };
        use fault::RecordFault;
        use std::io::Cursor;

        let framing = Framing::spike("bk".to_owned());
        let nonce = &framing.nonce().0;
        // A clean framed stream: two site records + a space-bearing deriv coord + its family close.
        let clean = format!(
            "dorc-records/1 nonce={nonce} attempt=1 host=localhost book=bk sites=2 {TERMINAL_TOKEN}\n\
             {nonce} site 0 effect=holds rc=0 {TERMINAL_TOKEN}\n\
             {nonce} site 1 effect=absent rc=1 {TERMINAL_TOKEN}\n\
             {nonce} deriv 0 coord=/etc/a file/with spaces {TERMINAL_TOKEN}\n\
             {nonce} deriv-end 0 n=1 body-rc=0 {TERMINAL_TOKEN}\n\
             dorc-records-end/1 nonce={nonce} {TERMINAL_TOKEN}\n"
        );
        let admit = |raw: &str| match read_host_evidence(
            Cursor::new(raw.as_bytes()),
            HostEvidenceLimits::spike_default(),
        ) {
            Admission::Admitted(bytes) => {
                admit_unscoped_host_records(&bytes, &framing, HostEvidenceLimits::spike_default())
            }
            Admission::NoObservation => Admission::NoObservation,
            Admission::Refused(refusal) => Admission::Refused(refusal),
        };
        let clean_records: BTreeSet<String> = match admit(&clean) {
            Admission::Admitted(records) => {
                records.iter().map(|record| format!("{record:?}")).collect()
            }
            other => panic!("clean stream must admit: {other:?}"),
        };
        assert!(
            clean_records.contains("Derivation { site: 0, coord: \"/etc/a file/with spaces\" }"),
            "the clean stream round-trips the space-bearing coordinate (last-to-token)"
        );

        let (mut torn, mut glued, mut oversize, mut clean_through) = (0u32, 0u32, 0u32, 0u32);
        for seed in 0..512u64 {
            let (mutated, class) = fault::mutate(seed, &clean, TERMINAL_TOKEN);
            let d = admit(&mutated);
            match class {
                RecordFault::Torn | RecordFault::Glued | RecordFault::Clean => {
                    // The safe direction: refused OR every emitted record is a CLEAN one (loss
                    // only). A prefix-truncated coordinate loses the token ⇒ dropped, never a
                    // fabricated shorter record — the whole point of the terminal token.
                    assert!(
                        matches!(d, Admission::Refused(_))
                            || matches!(
                                &d,
                                Admission::Admitted(records)
                                    if records.iter().all(|record| clean_records.contains(&format!("{record:?}")))
                            ),
                        "seed {seed} ({class:?}): fabricated a record outside the clean set: {d:?}",
                    );
                }
                RecordFault::Oversize => {
                    // A still-terminated oversized line WIDENS content (more/longer coords = more
                    // collisions = fewer survivals — safe), or exceeds an admission ceiling and
                    // refuses the whole read unit. Neither outcome fabricates a smaller record.
                    assert!(
                        matches!(d, Admission::Admitted(_) | Admission::Refused(_)),
                        "seed {seed}: the bounded reader produced no admission outcome"
                    );
                }
            }
            match class {
                RecordFault::Torn => torn += 1,
                RecordFault::Glued => glued += 1,
                RecordFault::Oversize => oversize += 1,
                RecordFault::Clean => clean_through += 1,
            }
        }
        assert!(
            torn > 0 && glued > 0 && oversize > 0 && clean_through > 0,
            "sometimes-assert: every fault class fires over 512 seeds \
             (torn={torn} glued={glued} oversize={oversize} clean={clean_through})"
        );
    }
}
