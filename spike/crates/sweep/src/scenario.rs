//! `gen` — the scenario generator (24B §1-C / §3). Per `u64` seed it mints a whole
//! interference scenario: a **book** (real sh, run through the real parser in [`crate::drive`]),
//! a fixed real **oracle** (`predict()` establishes + `touches()` footprints — [`crate::ORACLE_SH`]),
//! a seeded initial host state ([`hostsim::Host`]), AND — the load-bearing capability — a
//! per-site **ground-truth effect model** ([`TrueEffect`]) controlled INDEPENDENTLY of the
//! declared oracle. That independence is the whole point (24B §3): it lets the same net generate
//! HONEST scenarios (the true effect stays inside the declared footprint) and LYING scenarios (a
//! wall whose true effect clobbers a cell its `touches()` never declared — the resid-aliasing /
//! wrong-footprint disaster, 24C resid-aliasing).
//!
//! # Structured, not free-form (the realistic-interference call — 24B §5 coverage-humility)
//!
//! A random book almost never contains a REAL interference (a converged elision downstream of a
//! running wall that touches its backing) — so the generator does NOT emit free-form books. It
//! emits a fixed 2–3 command TEMPLATE — `[wall, (extra-wall)?, victim]` — and randomises the
//! AXES that matter: the wall's kind (establish / kill / silent), whether its footprint HITS or
//! MISSES the victim's entity, whether the victim is converged @S0, honesty, and multi-wall.
//! This GUARANTEES the interesting chronology by construction; the unsolved half is coverage of
//! book SHAPES beyond the template ("science more than engineering", `128` fc-5). The net HUNTS
//! bugs at breadth; it never proves elision soundness.
//!
//! # The declared/ground-truth type split (rul24-overtype; the engine never sees ground truth)
//!
//! [`TrueEffect`] is a hostsim [`CellDelta`] — a MODEL-altitude value, a DISTINCT type from the
//! plan's `Footprint`/`touches()` (the declared at-most claim). A [`Scenario`] holds both, but
//! the driver that runs the kernel ([`crate::drive::run_kernel`]) is handed only the
//! [`DeclaredScenario`] + the host; the [`GroundTruth`] is `pub(crate)` and reaches ONLY the
//! host-evolution ([`crate::drive::evolve`]). Feeding ground truth into the analyzer is therefore
//! not expressible — a test that tried would be measuring nothing, so we make it not typecheck.

use dorc_core::{EntityRef, FactKey, Interner, KindId, OpaqueToken, SelectorId};
use dorc_hostsim::{CellDelta, Host, Lcg};

/// A scenario seed — the replay handle surfaced on every failure (`128` L0 / `notes/123` f20).
/// Newtype so a bare `u64` book-count / index can never be passed where a seed is meant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seed(pub u64);

/// The interference-topology class a scenario realises (24B §1-C). The sweep asserts each is
/// actually GENERATED across the seed range (the reachability half of "is my DST working" —
/// Antithesis sometimes-assertions, `128` fc-5): a net that never produces a HIT case, or never
/// a survival case, is silently greenwashing and must fail loud. A single headline per scenario
/// (priority order in [`generate`]); the axes are richer than six cells, but these are the ones
/// whose ABSENCE would mean the net stopped exercising a distinct kernel path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyClass {
    /// A footprinted running wall MISSES a converged victim's entity ⇒ the victim SURVIVES
    /// (elides past the running wall) under `--trust-footprints`. The golden-hill happy path.
    MissConverged,
    /// Footprint misses, but the victim is DIVERGED @S0 ⇒ it runs regardless (no elision).
    MissDiverged,
    /// The wall's footprint HITS the victim's entity (same `(kind, entity)`, different selector)
    /// ⇒ `disjoint` returns `None` ⇒ the victim is DEMOTED to run even under the flag.
    HitConverged,
    /// The wall is a `purge` (a kill verb) — a real mutator that walls via the `kills` set
    /// (24A §3), its coherence check SKIPPED (24C resid-kill-coherence). Exercises kill-wall
    /// survival/attribution.
    KillWall,
    /// The wall declares NO footprint (a verb with no `touches()` arm) ⇒ a TOTAL wall (silence =
    /// wall) ⇒ the victim demotes even flag-on. Exercises the `total_wall` demotion path.
    SilentWall,
    /// ≥2 running walls precede the victim ⇒ a multi-crossing survival witness (or a demotion by
    /// any one of them). Exercises witness aggregation.
    MultiWall,
    /// The primary wall is a `place` (24E §6) — its footprint is DERIVED at probe time via
    /// [`dorc_hostsim::Host::derive`] (an escalating `touches()`), not authored statically. A MISS
    /// against a converged victim ⇒ the victim SURVIVES past a payload-bound wall (the derived-
    /// footprint golden hill). A LYING derived footprint here (⊂ the wall's true `CellDelta`)
    /// under-declares ⇒ the victim wrongly survives ⇒ the end-state differential goes RED — the
    /// automated soundness proof `find-net-covers-what` (24C) says only a lying scenario can catch.
    DerivedWall,
    /// The wall's footprint name and the victim's backing name are TWO NAMES for ONE referent
    /// (`nginx` / `nginx-full`; 24F §3 the resid-aliasing closure). The wall TRULY touches the shared
    /// referent (its `CellDelta` kills the victim's cell — the intrinsic aliasing hazard), so
    /// token-equality would wrongly call them disjoint ⇒ the victim wrongly survives. A `package`
    /// RESOLVER bridges the names: HONEST (the `!lying` coin) canonicalizes both to one ⇒ the closure
    /// HITs ⇒ the victim DEMOTES ⇒ safe (end-states match); LYING keeps them apart ⇒ the victim
    /// wrongly survives ⇒ the end-state differential goes RED, attributed. The lying-RESOLVER
    /// soundness net (24F §7.1), the identity analogue of the lying-footprint / lying-derived nets.
    AliasWall,
    /// The wall's footprint (`package:base`) is EXPANDED by `package.reaches()` to drag `package:dep`
    /// — the reach-function knowledge the wall's own `touches()` cannot spell (24G §4, the cross-author
    /// point). The wall TRULY reaches the shared referent (its `CellDelta` kills the victim's
    /// `package:dep#installed`), so WITHOUT expansion the `package:base` footprint is disjoint from the
    /// victim's `package:dep` backing ⇒ the victim wrongly survives. The DECLARED reach answer bridges
    /// them: HONEST (`!lying`) INCLUDES `package:dep` ⇒ the expanded footprint HITs ⇒ the victim
    /// DEMOTES ⇒ safe (end-states match); LYING OMITS it ⇒ the victim wrongly survives ⇒ the end-state
    /// differential goes RED, attributed to the reach-function. The lying-REACHES soundness net (24G),
    /// where OMISSION is the sharp edge — the fourth lying net beside static / derived / alias.
    ReachWall,
}

/// Whether a scenario's ground truth matches its declared oracle. `Lying` records WHICH wall
/// under-declared (its leaf), so the attribution assertion checks the survival witness against
/// the generator's OWN record of the liar, never a re-derivation (rul24-overtype). Interner-free
/// (the leaf is a plain id), so it rides a self-contained [`crate::TrialResult`] for the
/// determinism guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Honesty {
    /// Every wall's true effect stays inside its declared footprint — end-states MUST match
    /// (a mismatch is engine unsoundness, 24B §5).
    Honest,
    /// The wall at `liar_leaf` truly touches a cell its `touches()` omits — end-states MAY
    /// diverge (the priced residue); if they do, the survival witness MUST blame `liar_leaf`.
    Lying {
        /// The leaf id of the under-declaring wall (leaf 0 in this generator — the primary wall).
        liar_leaf: u32,
    },
}

/// The concrete GROUND-TRUTH effect of one book command — the cells it REALLY flips, wrapping a
/// hostsim [`CellDelta`] (24B §3). A DISTINCT type from the declared footprint (the plan's
/// `Footprint`), held only in [`GroundTruth`] and applied only to the evolving hosts. The kernel
/// driver never receives one, so ground truth cannot leak into the analysis (rul24-overtype).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrueEffect(CellDelta);

impl TrueEffect {
    /// The underlying cell-delta — `pub(crate)`, reached ONLY by [`crate::drive::evolve`] (the
    /// host evolution). No public accessor: the engine half of the crate cannot read it.
    pub(crate) fn delta(&self) -> &CellDelta {
        &self.0
    }
}

/// The ground-truth half of a scenario — the per-site true-effects. `pub(crate)` field: only
/// [`crate::drive::evolve`] consumes it, and it is structurally impossible to hand to
/// [`crate::drive::run_kernel`] (rul24-overtype — the analyzer sees only the declared half). The
/// honesty marker lives on the [`Scenario`] (a public copy the assertion split reads).
#[derive(Debug, Clone)]
pub struct GroundTruth {
    /// One entry per book command, in book order (== the plan's span-sorted leaf order): the
    /// site's true effect, or `None` for a non-mutator. Zipped 1:1 with the plan's steps.
    pub(crate) site_effects: Vec<Option<TrueEffect>>,
}

/// The DECLARED half of a scenario — everything the analyzer is allowed to see. Just the book
/// sh; the oracle is the fixed [`crate::ORACLE_SH`] (a deliberate scoping — the interference
/// lives in the book/host/truth, not in oracle variety, 24B strain note). No `TrueEffect` in
/// sight, by construction.
#[derive(Debug, Clone)]
pub struct DeclaredScenario {
    /// The book sh, run through the REAL parser in [`crate::drive::run_kernel`].
    pub book_sh: String,
}

/// A whole generated scenario: the declared half (for the kernel), the seeded initial host
/// state, the ground truth (for evolution only), plus metadata the assertions read.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// The seed that minted this scenario (surfaced on failure).
    pub seed: Seed,
    /// The interference class (coverage tracking).
    pub topology: TopologyClass,
    /// The honesty marker (drives the assertion split, and names the liar).
    pub honesty: Honesty,
    /// The declared half — the ONLY thing [`crate::drive::run_kernel`] receives.
    pub declared: DeclaredScenario,
    /// The seeded initial host state S0 — the probe-time world the engine legitimately observes.
    pub s0: Host,
    /// The victim's own fact label (`package:nginx:installed`) — the cell whose survival/clobber
    /// the assertions key on. Interner-free (resolved at generation).
    pub victim_fact_label: String,
    /// The ground truth — `pub(crate)`, reached only by [`crate::drive::evolve`].
    pub(crate) ground: GroundTruth,
}

/// The pool of package entities the generator draws from. Six is enough that "a distinct entity"
/// always exists after excluding one or two.
const ENTITY_POOL: [&str; 6] = ["nginx", "curl", "vim", "htop", "oldpkg", "redis"];

/// The kind every generated fact lives in (the oracle is a package oracle).
const KIND: &str = "package";

/// The one modeled selector a fact-cell carries in the sweep (`#installed`). Config/refresh walls
/// deliberately establish a DIFFERENT selector on the same entity (see [`WallKind`]).
const INSTALLED: &str = "installed";

/// How the primary wall mutates — the axis that selects establish / kill / silent behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallKind {
    /// `apt-get install X` — establishes `package:X#installed`, footprint `package:X`.
    Install,
    /// `apt-get config X` — establishes `package:X#configured` (a DIFFERENT selector, so a
    /// same-entity victim's `#installed` stays ambient), footprint `package:X`. The HIT wall.
    Config,
    /// `apt-get purge X` — a KILL (`EstablishInverted` ⇒ `MustRun` ⇒ walls via the `kills` set),
    /// footprint `package:X`, coherence-unchecked (24C resid-kill-coherence).
    Purge,
    /// `apt-get refresh X` — establishes `package:X#refreshed` but has NO `touches()` arm ⇒ no
    /// footprint ⇒ a TOTAL wall (silence = wall).
    Silent,
    /// `apt-get place X` (24E §6) — establishes `package:X#installed` like `Install`, but its
    /// `touches()` arm ESCALATES (reaches `apt-manifest`), so its footprint is DERIVED via
    /// [`Host::derive`], not authored. A MISS establish wall with a probe-time footprint.
    Derived,
}

/// Build one fact-cell in the shared interner (the vocabulary fence — the SAME `KindId`/entity
/// token the book/predict analysis mints for these strings, `inv-referent-agnostic`). All
/// scenario ground-truth cells are minted through here, so they compare equal to the cells the
/// kernel resolves from the book's `install nginx`.
fn cell(i: &mut Interner, entity: &str, selector: &str) -> FactKey {
    FactKey {
        kind: KindId(i.intern(KIND)),
        entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
        selector: SelectorId(i.intern(selector)),
    }
}

/// The `kind:entity:selector` label of a cell (interner-free, for the self-contained result).
fn label(i: &Interner, fact: FactKey) -> String {
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

/// Pick a pool entity distinct from every entity in `exclude` (bounded, always succeeds — the
/// pool is larger than any exclusion set the generator builds).
fn distinct_entity(rng: &mut Lcg, exclude: &[&str]) -> &'static str {
    loop {
        let idx = usize::try_from(rng.below(ENTITY_POOL.len() as u64)).unwrap_or(0);
        let cand = ENTITY_POOL.get(idx).copied().unwrap_or("nginx");
        if !exclude.contains(&cand) {
            return cand;
        }
    }
}

/// The primary wall's book line + its HONEST true effect (before any lying clobber).
fn wall_command(i: &mut Interner, entity: &str, kind: WallKind) -> (String, CellDelta) {
    match kind {
        WallKind::Install => (
            format!("apt-get install {entity}"),
            CellDelta::new().establish(cell(i, entity, INSTALLED)),
        ),
        WallKind::Config => (
            format!("apt-get config {entity}"),
            CellDelta::new().establish(cell(i, entity, "configured")),
        ),
        WallKind::Purge => (
            format!("apt-get purge {entity}"),
            CellDelta::new().kill(cell(i, entity, INSTALLED)),
        ),
        WallKind::Silent => (
            format!("apt-get refresh {entity}"),
            CellDelta::new().establish(cell(i, entity, "refreshed")),
        ),
        WallKind::Derived => (
            format!("apt-get place {entity}"),
            CellDelta::new().establish(cell(i, entity, INSTALLED)),
        ),
    }
}

/// An ordinary `apt-get install X` command + its true effect (the victim, and any extra wall).
fn install_command(i: &mut Interner, entity: &str) -> (String, CellDelta) {
    (
        format!("apt-get install {entity}"),
        CellDelta::new().establish(cell(i, entity, INSTALLED)),
    )
}

/// The topology headline (priority: silence and multi-wall dominate the wall's own verb). `wall_kind`
/// already encodes the HIT (`Config`) and DERIVED (`Derived`) axes, so only `multi`/`alias`/
/// `victim_converged` are needed besides it. The single per-scenario class the coverage
/// sometimes-asserts key on (the axes are richer than nine cells; these are the distinct kernel paths).
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "each bool is an independent generator AXIS (multi/alias/reach/victim_converged) the headline priority-orders; a bundling enum would obscure the one-axis-per-coin structure the module doc + determinism rely on"
)]
fn topology_of(
    wall_kind: WallKind,
    multi: bool,
    alias: bool,
    reach: bool,
    victim_converged: bool,
) -> TopologyClass {
    if wall_kind == WallKind::Silent {
        TopologyClass::SilentWall
    } else if multi {
        TopologyClass::MultiWall
    } else if wall_kind == WallKind::Purge {
        TopologyClass::KillWall
    } else if wall_kind == WallKind::Config {
        TopologyClass::HitConverged
    } else if wall_kind == WallKind::Derived {
        TopologyClass::DerivedWall
    } else if alias {
        TopologyClass::AliasWall
    } else if reach {
        TopologyClass::ReachWall
    } else if victim_converged {
        TopologyClass::MissConverged
    } else {
        TopologyClass::MissDiverged
    }
}

/// Build the seeded S0 host from the cells that `holding` at probe time, plus a DERIVED wall's
/// declared manifest (24E §6), an ALIAS wall's declared `package` RESOLVER (24F §3/§7.1), OR a REACH
/// wall's declared `package` REACH-ANSWER (24G). The manifest/resolver/reach honesty rides the `lying`
/// coin — the DECLARED half of the declared-vs-true split; the TRUE clobber lives in the `CellDelta`.
/// (`derived`/`alias`/`reach` are mutually exclusive by the draw gating; at most one branch fires.)
#[expect(
    clippy::too_many_arguments,
    clippy::fn_params_excessive_bools,
    reason = "the scenario-host builder threads each independent axis (derived/alias/reach/lying) + the two entity names; each selects a distinct declared-answer branch, not a bundle-able struct"
)]
fn scenario_host(
    i: &mut Interner,
    holding: Vec<FactKey>,
    derived: bool,
    alias: bool,
    reach: bool,
    lying: bool,
    wall_entity: &str,
    victim: &str,
) -> Host {
    if derived {
        // An HONEST manifest ⊇ the wall's establish ({package:wall_entity}); the LIE, if any, is the
        // undeclared clobber in the TRUE `CellDelta` (24E §6 — the footprint SOURCE is `Host::derive`).
        let wall_cell = cell(i, wall_entity, INSTALLED);
        Host::new(holding).with_manifest(wall_cell, [wall_cell])
    } else if alias {
        // The `package` resolver (24F §3/§7.1). The wall name and the victim's ALIAS name are two
        // names for one referent. HONEST (`!lying`): both canonicalize to the wall's (base) entity ⇒
        // the closure HITs ⇒ the victim DEMOTES ⇒ safe. LYING: the alias resolves to ITSELF ⇒ kept
        // apart ⇒ the victim wrongly survives ⇒ RED (the wall's true kill of the victim bites).
        let package = KindId(i.intern(KIND));
        let wall_ent = EntityRef::Operand(OpaqueToken(i.intern(wall_entity)));
        let victim_ent = EntityRef::Operand(OpaqueToken(i.intern(victim)));
        let victim_canonical = if lying { victim_ent } else { wall_ent };
        Host::new(holding)
            .with_resolution(package, wall_ent, wall_ent)
            .with_resolution(package, victim_ent, victim_canonical)
    } else if reach {
        // The DECLARED reach answer (24G): touching `package:wall_entity` REACHES the victim's dep.
        // HONEST (`!lying`) INCLUDES `package:dep` (⇒ the expanded footprint HITs the victim's backing
        // ⇒ the victim DEMOTES ⇒ safe); LYING OMITS it (an EMPTY reach ⇒ the expansion misses ⇒ the
        // victim wrongly survives ⇒ the wall's true kill of the dep bites ⇒ RED). Omission is the edge.
        let package = KindId(i.intern(KIND));
        let wall_ent = EntityRef::Operand(OpaqueToken(i.intern(wall_entity)));
        let victim_ent = EntityRef::Operand(OpaqueToken(i.intern(victim)));
        let reached: Vec<(KindId, EntityRef)> = if lying {
            Vec::new()
        } else {
            vec![(package, victim_ent)]
        };
        Host::new(holding).with_reach(package, wall_ent, reached)
    } else {
        Host::new(holding)
    }
}

/// Generate the scenario for `seed` (24B §1-C / §3). Deterministic in the seed (the sole entropy
/// is one [`Lcg`] — `inv-determinism`, no forked PRNG). The draws pick the interference AXES; the
/// template guarantees a real chronology (see the module docs).
#[must_use]
#[expect(
    clippy::too_many_lines,
    reason = "a linear per-seed scenario builder — draw the axes, then compute victim/wall/holding/host in one straight read; the five interference axes (hit/multi/derived/alias/reach) make it inherently long, and splitting it would scatter the one-place draw ORDER the determinism guard depends on"
)]
pub fn generate(seed: Seed, i: &mut Interner) -> Scenario {
    let mut rng = Lcg::new(seed.0);

    // Every axis is drawn through `below` (the HIGH-bit draw), NOT `chance`: the LCG's low bits
    // are periodic, so low-bit coins would correlate (see [`Lcg::below`]). One draw per axis keeps
    // them independent.
    let base = distinct_entity(&mut rng, &[]);
    // A HIT (wall footprint touches the victim's entity) is deliberately rarer than a miss — a
    // realistic book seldom re-touches the same entity — but frequent enough for coverage.
    let hit = rng.below(3) == 0;
    let multi = rng.below(3) == 0;
    // The `lying` coin does double duty: for a footprint/derived scenario it is the FOOTPRINT lie;
    // for an alias scenario it is the RESOLVER lie (honest merges the names, lying keeps them apart).
    let lying = rng.below(2) == 0;
    // The DERIVED-footprint axis (24E §6): drawn unconditionally (one draw per axis, module doc),
    // but realized only for a clean single-wall MISS (not a hit/multi) — a `place` wall whose
    // footprint is host-DERIVED. This is the axis the lying-derived soundness net exercises.
    let derived = rng.below(3) == 0 && !hit && !multi;
    // The ALIASING axis (24F §7.1): a clean single-wall scenario where the wall footprints one name
    // and the victim backs ANOTHER name of the SAME referent, bridged by a `package` resolver whose
    // honesty is the `lying` coin — the lying-RESOLVER soundness net.
    let alias = rng.below(3) == 0 && !hit && !multi && !derived;
    // The REACH axis (24G): a clean single-wall scenario where the wall footprints `package:base` and
    // the victim backs `package:dep` — a coord the wall's `touches()` cannot spell — bridged by
    // `package.reaches()` whose DECLARED answer honesty is the `lying` coin (OMIT ⇒ under-execute).
    // The lying-REACHES soundness net, mutually exclusive with the other special axes.
    let reach = rng.below(3) == 0 && !hit && !multi && !derived && !alias;

    // The victim entity. For an alias scenario the victim wears the ALIAS name (`<base>-full`, a
    // provides/virtual downstream name); for a reach scenario it is a distinct DEPENDENCY the wall
    // reaches (a coord the wall's touches() never names); otherwise it is the base pool entity.
    let victim: String = if alias {
        format!("{base}-full")
    } else if reach {
        distinct_entity(&mut rng, &[base]).to_owned()
    } else {
        base.to_owned()
    };

    // The wall kind: HIT ⇒ `config` (a different selector on the SAME entity, so the victim's
    // `#installed` stays ambient); `derived` ⇒ `place` (host-derived footprint); `alias` ⇒ plain
    // `install` (the aliasing lives in the names + resolver, not the verb); else establish/kill/silent.
    let wall_kind = if hit {
        WallKind::Config
    } else if derived {
        WallKind::Derived
    } else if alias || reach {
        // Alias/reach live in the names + resolver/reach-answer, not the verb — a plain install wall.
        WallKind::Install
    } else {
        match rng.below(3) {
            0 => WallKind::Install,
            1 => WallKind::Purge,
            _ => WallKind::Silent,
        }
    };
    let wall_entity: String = if hit {
        victim.clone()
    } else if alias || reach {
        // The wall footprints the BASE name; the victim backs the ALIAS (alias) or a reached DEP
        // (reach) — bridged by the resolver / reach-answer respectively.
        base.to_owned()
    } else {
        distinct_entity(&mut rng, &[victim.as_str()]).to_owned()
    };
    // A HIT / DERIVED / ALIAS / REACH wall only interferes-with a would-elide (converged) victim; a
    // diverged victim runs regardless, so pin converged for a clean class. Otherwise a coin flip.
    let victim_converged = if hit || derived || alias || reach {
        true
    } else {
        rng.below(2) == 0
    };

    // Book order: [primary wall (leaf 0), (extra wall)?, victim (last)]. The victim is downstream
    // of every wall, so it crosses them; the primary wall is leaf 0, so a lie names leaf 0.
    let mut commands: Vec<(String, Option<TrueEffect>)> = Vec::new();

    let (wall_line, honest_wall_delta) = wall_command(i, &wall_entity, wall_kind);
    // The LIE (rul24-divergence-is-the-game): the wall truly ALSO kills the victim's `#installed` —
    // the footprint's undeclared clobber (`lying`), the INTRINSIC aliasing hazard (`alias`: the two
    // names ARE one referent), OR the INTRINSIC reach hazard (`reach`: touching `package:base` really
    // drags `package:dep`, so the true effect ALWAYS kills the dep — independent of whether the
    // DECLARED reach ANSWER admits it). The analyzer can't see the true effect.
    let wall_delta = if lying || alias || reach {
        honest_wall_delta.kill(cell(i, &victim, INSTALLED))
    } else {
        honest_wall_delta
    };
    commands.push((wall_line, Some(TrueEffect(wall_delta))));

    if multi {
        let extra = distinct_entity(&mut rng, &[victim.as_str(), wall_entity.as_str()]);
        let (line, delta) = install_command(i, extra);
        commands.push((line, Some(TrueEffect(delta))));
    }

    let (victim_line, victim_delta) = install_command(i, &victim);
    commands.push((victim_line, Some(TrueEffect(victim_delta))));

    let topology = topology_of(wall_kind, multi, alias, reach, victim_converged);
    // S0: the cells that hold at probe time. The victim holds iff converged. Every wall's own
    // establish cell is ABSENT so it RUNS; a purge's target is seeded present (realism).
    let mut holding = Vec::new();
    if victim_converged {
        holding.push(cell(i, &victim, INSTALLED));
    }
    if wall_kind == WallKind::Purge {
        holding.push(cell(i, &wall_entity, INSTALLED));
    }
    let s0 = scenario_host(
        i,
        holding,
        derived,
        alias,
        reach,
        lying,
        &wall_entity,
        &victim,
    );

    let honesty = if lying {
        Honesty::Lying { liar_leaf: 0 }
    } else {
        Honesty::Honest
    };

    let book_sh = format!(
        "# sweep scenario — seed {} — topology {:?} — honesty {:?}\n{}\n",
        seed.0,
        topology,
        honesty,
        commands
            .iter()
            .map(|(line, _)| line.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
    );

    let victim_cell = cell(i, &victim, INSTALLED);
    let victim_fact_label = label(i, victim_cell);

    Scenario {
        seed,
        topology,
        honesty,
        declared: DeclaredScenario { book_sh },
        s0,
        victim_fact_label,
        ground: GroundTruth {
            site_effects: commands.into_iter().map(|(_, e)| e).collect(),
        },
    }
}
