//! `dorc-sweep` — the round-24 **chronology net** (24B §1-C / §3 / §5; the flavour-C DST harness).
//!
//! An in-memory, seeded deterministic-simulation sweep that catches wrong ELISIONS the existing
//! run-set differential is structurally blind to. Per `u64` seed it generates an interference
//! scenario, runs the REAL kernel pipeline in-process, evolves two [`dorc_hostsim::Host`] copies
//! from one S0 (**bare** = the whole book ran; **plan** = only `Run`-disposition sites ran), and
//! asserts **end-state equality** across generated interference topologies. The generator holds a
//! ground-truth effect model INDEPENDENT of the declared oracle, which lets it mint LYING
//! scenarios (a wall that truly touches a cell its `touches()` omits) and assert
//! **attribution-under-lies** (the survival witness names the actual liar) — the property that
//! makes the dangerous survival tier accountable.
//!
//! # Altitude honesty (24B §5 C-altitude) — this is ELISION-SOUNDNESS DST, NOT coordination DST
//!
//! The net rides `Host` as the fact-VERDICT stand-in and asks one question: *does the elided plan
//! reach the bare book's end-state?* That is elision-soundness at the fact-verdict seam — a level
//! BELOW the round-12 controller↔host transport STREAM. It is explicitly **NOT** the multi-host
//! coordination DST: there is no orchestrator, no result-stream, no network faults here.
//!
//! ## Explicitly out of scope — round-25 / `22H` (named so it is not lost)
//!
//! Network-fault injection (drop/timeout/partition/truncate/reorder), multi-host fan-in, the
//! three-valued-verdict-under-truncation test (`128` fc-2), and retry-safety/no-double-apply under
//! `Unknown` (`128` fc-4) all live at the transport-stream seam with the coordination orchestrator
//! that does not exist in the spike yet. They COMPOSE with this net (shared `Host`), never collide.
//! Do not grow this crate toward them.
//!
//! # Coverage humility (24B §5 / `128` fc-5) — the net HUNTS bugs, never PROVES soundness
//!
//! The per-class sometimes-asserts are the *reachability* half of "is my DST working"; state-space
//! COVERAGE is the unsolved half ("science more than engineering", unsolved industry-wide). A green
//! sweep raises confidence at breadth — it is Jepsen's "presence, not absence".

use std::collections::BTreeSet;

use dorc_core::{EntityRef, Interner, OpaqueToken};
use dorc_plan::{Disposition, EntityCoord, Plan};

pub mod drive;
pub mod scenario;

pub use scenario::{Honesty, Scenario, Seed, TopologyClass};

/// The ONE fixed oracle the whole sweep drives (a deliberate scoping, 24B §3 strain note): the
/// interference lives in the book/host/ground-truth, not in oracle variety. A real package oracle,
/// lifted through the real `predict`/`touches`/`is_converged` lifts every seed. `predict()` derives
/// the effect-map; `touches()` the at-most footprints; `is_converged()` is the VOUCH the elide-weld
/// (24D §3) demands — without it a converged ambient `install` would no longer elide, and the whole
/// net's elision coverage would vanish. `install`/`config`/`purge` touch `package:<operand>` via a
/// STATIC (authored) `touches()`; `refresh` establishes but has NO `touches()` arm ⇒ a
/// footprint-less (silent, total-walling) mutator. `config` establishes a DIFFERENT selector
/// (`#configured`) so a same-entity victim's `#installed` stays ambient — the entity-granular HIT.
/// **`place` (24E §6) establishes `package:<op>#installed` like `install`, but its `touches()` arm
/// reaches a host tool (`apt-manifest`) ⇒ the static tracer ⊤s (`NonPrintfCommand`) ⇒ it ESCALATES
/// to host-derivation: its footprint is DERIVED via [`dorc_hostsim::Host::derive`], not authored —
/// the derived-footprint wall the lying-derived net drives.** `is_converged()` vouches the establish
/// verbs (install/config/refresh/place); a `purge` is a KILL (never elides), declined by `*) return 2`.
pub const ORACLE_SH: &str = r#"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
         config)  dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".configured ;;
         refresh) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".refreshed ;;
         purge)   dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed! ;;
         place)   dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
      esac
   fi
}

apt-get.touches() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install|config|purge) printf 'package:%s\n' "$1" ;;
      place) apt-manifest "$1" ;;
   esac
}

apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install|config|refresh|place) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
"#;

/// One survived elision's attribution, lifted from a flag-on plan (interner-free). The
/// attribution-under-lies assertion checks that a diverging lying scenario has a survival whose
/// `crossed_walls` names the generator's recorded liar leaf (TC-3 — the witness IS the attribution).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Survival {
    /// The elided site's leaf id.
    pub elided_leaf: u32,
    /// The elided site's fact label (`package:nginx:installed`).
    pub elided_label: String,
    /// The running walls this elision crossed, in execution order (≥1 by construction).
    pub crossed_walls: Vec<CrossedWall>,
}

/// One wall a survival crossed (interner-free): its leaf + the provider whose footprint licensed
/// the crossing (the why-lens names the liar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossedWall {
    /// The crossed wall's leaf id.
    pub wall_leaf: u32,
    /// The licensor provider (`apt-get`).
    pub provider: String,
}

/// A fully self-contained, interner-free trial result — everything resolved to strings/enums, so
/// two runs of the same seed are directly comparable (the determinism guard is just
/// `assert_eq!(trial_for_seed(s), trial_for_seed(s))`, `notes/123` f20). Holds no `Symbol`/interner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialResult {
    /// The replay handle.
    pub seed: Seed,
    /// The interference class this scenario realised (coverage tracking).
    pub topology: TopologyClass,
    /// Honest vs lying (drives the assertion split; names the liar leaf).
    pub honesty: Honesty,
    /// The victim's fact label — the cell whose survival/clobber the assertions key on.
    pub victim_label: String,
    /// End-state of the bare book (every command ran) — fact labels.
    pub s_bare: BTreeSet<String>,
    /// End-state of the flag-ON plan (survival tier) applied — fact labels.
    pub s_apply_on: BTreeSet<String>,
    /// End-state of the flag-OFF plan (baseline) applied — fact labels.
    pub s_apply_off: BTreeSet<String>,
    /// The survived-elision attributions from the flag-ON plan (attribution-under-lies input).
    pub survivals: Vec<Survival>,
    /// A canonical fingerprint of the flag-ON plan's dispositions + attribution (determinism).
    pub plan_on_fp: String,
    /// A canonical fingerprint of the flag-OFF plan (determinism).
    pub plan_off_fp: String,
}

impl TrialResult {
    /// Did the flag-ON survival tier UNDER-EXECUTE relative to the bare book? (The soundness
    /// question: a mismatch under an HONEST scenario is engine unsoundness; under a LYING one it
    /// is the priced residue.)
    #[must_use]
    pub fn on_diverged(&self) -> bool {
        self.s_bare != self.s_apply_on
    }

    /// Did the flag-OFF baseline under-execute? (Should be NEVER — the baseline demotes every
    /// post-wall elision, so it can never diverge from bare; a `true` here is a deeper bug.)
    #[must_use]
    pub fn off_diverged(&self) -> bool {
        self.s_bare != self.s_apply_off
    }
}

/// Run one whole trial for `seed` (24B §3): fresh interner → generate → drive the kernel (flag-on
/// AND flag-off) → evolve the two hosts → package an interner-free [`TrialResult`]. The single
/// entry the sweep tests and the determinism guard both use; owns the interner lifecycle so the
/// result is self-contained and comparable.
///
/// rul24-overtype in action: [`scenario::generate`] mints the scenario (ground truth included), but the
/// kernel is driven via [`drive::run_kernel`], whose signature omits [`scenario::GroundTruth`] — so the
/// analyzer here provably never observes a command's true effect. Only [`drive::evolve`] touches
/// the ground truth, and only to evolve the two hosts.
#[must_use]
pub fn trial_for_seed(seed: Seed) -> TrialResult {
    let mut i = Interner::default();
    let scenario = scenario::generate(seed, &mut i);

    let plan_on = drive::run_kernel(&scenario.declared, &scenario.s0, true, &mut i);
    let plan_off = drive::run_kernel(&scenario.declared, &scenario.s0, false, &mut i);

    let (s_bare, s_apply_on) = drive::evolve(&scenario.s0, &plan_on, &scenario.ground);
    let (s_bare_off, s_apply_off) = drive::evolve(&scenario.s0, &plan_off, &scenario.ground);
    debug_assert_eq!(
        s_bare, s_bare_off,
        "bare evolution must be plan-independent (the bare book runs every command)"
    );

    TrialResult {
        seed,
        topology: scenario.topology,
        honesty: scenario.honesty,
        victim_label: scenario.victim_fact_label.clone(),
        s_bare: label_set(&s_bare, &i),
        s_apply_on: label_set(&s_apply_on, &i),
        s_apply_off: label_set(&s_apply_off, &i),
        survivals: survivals_of(&plan_on, &i),
        plan_on_fp: plan_fingerprint(&plan_on, &i),
        plan_off_fp: plan_fingerprint(&plan_off, &i),
    }
}

/// Resolve a fact-set to its sorted `kind:entity:selector` labels (the end-state comparand).
fn label_set(facts: &BTreeSet<dorc_core::FactKey>, i: &Interner) -> BTreeSet<String> {
    facts.iter().map(|f| fact_label(*f, i)).collect()
}

/// The `kind:entity:selector` label of a fact.
fn fact_label(fact: dorc_core::FactKey, i: &Interner) -> String {
    let entity = match fact.entity {
        EntityRef::Operand(OpaqueToken(sym)) => i.resolve(sym).to_owned(),
        EntityRef::Singleton => String::new(),
    };
    format!(
        "{}:{}:{}",
        i.resolve(fact.kind.0),
        entity,
        i.resolve(fact.selector.0),
    )
}

/// The `kind:entity` label of an entity-coordinate (for wall-footprint provenance).
fn coord_label(coord: EntityCoord, i: &Interner) -> String {
    let entity = match coord.entity() {
        EntityRef::Operand(OpaqueToken(sym)) => i.resolve(sym).to_owned(),
        EntityRef::Singleton => String::new(),
    };
    format!("{}:{}", i.resolve(coord.kind().0), entity)
}

/// Extract every survived elision's attribution from a plan (the flag-on one). A survival is a
/// `Replace` whose derivation carries a `SurvivalWitness` — the site crossed ≥1 running wall.
fn survivals_of(plan: &Plan, i: &Interner) -> Vec<Survival> {
    let mut out = Vec::new();
    for step in &plan.steps {
        let Disposition::Replace(license, _) = &step.disposition else {
            continue;
        };
        let Some(witness) = &license.derivation().survival else {
            continue;
        };
        out.push(Survival {
            elided_leaf: step.leaf.0,
            elided_label: fact_label(license.fact(), i),
            crossed_walls: witness
                .crossings()
                .iter()
                .map(|c| CrossedWall {
                    wall_leaf: c.wall_leaf().0,
                    provider: i.resolve(c.provider()).to_owned(),
                })
                .collect(),
        });
    }
    out
}

/// A canonical, interner-free fingerprint of a plan — the determinism comparand (24B §5
/// C-determinism-guard). Captures each leaf's disposition tag, verbatim sh, and (for a survived
/// `Replace`) the crossed walls' leaves + footprints. Any nondeterminism in the pure kernel (an
/// observable `HashMap` iteration, a non-`BTree` order leak — `inv-determinism`) perturbs this.
fn plan_fingerprint(plan: &Plan, i: &Interner) -> String {
    let mut lines = Vec::with_capacity(plan.steps.len());
    for step in &plan.steps {
        let disp = match &step.disposition {
            Disposition::Run => "run".to_owned(),
            Disposition::Omit { .. } => "omit".to_owned(),
            // The guard bucket (24D §2 / the Stage-3 sweep extension): fingerprint a guard by its
            // fact + verdict funcname, so a nondeterministic guard mint perturbs the determinism
            // comparand. NB the current scenario generator mints no vouches, so no guard appears in
            // the sweep yet (tc-sweep-guard-scenarios deferred); the arm keeps the fingerprint
            // total across the new disposition.
            Disposition::Guard(license) => format!(
                "guard(fn={},fact={})",
                license.insert().fn_name(),
                fact_label(license.fact(), i)
            ),
            Disposition::Replace(license, stand_in) => {
                let survival = license.derivation().survival.as_ref().map_or_else(
                    || "clean".to_owned(),
                    |w| {
                        let crossings: Vec<String> = w
                            .crossings()
                            .iter()
                            .map(|c| {
                                let fps: Vec<String> =
                                    c.footprint().iter().map(|co| coord_label(*co, i)).collect();
                                format!("@{}[{}]", c.wall_leaf().0, fps.join(","))
                            })
                            .collect();
                        format!("survived{}", crossings.join(""))
                    },
                );
                format!("replace({stand_in:?},{survival})")
            }
        };
        lines.push(format!("{}|{}|{}", step.leaf.0, disp, step.sh));
    }
    lines.join("\n")
}
