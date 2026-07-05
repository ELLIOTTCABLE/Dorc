//! `drive` — the in-process kernel driver + the two-host evolution (24B §3 / §5).
//!
//! [`run_kernel`] runs the REAL kernel pipeline in-process — `syntax::parse → analysis::{cfg,
//! value, effect} → plan::build_plan_walled` — with the survival footprints lifted EXACTLY as the
//! cli's `build_survival_footprints` does (the composition is replicated here, not re-invented, so
//! the net drives the same code the shipped tool does). It takes only the DECLARED half of a
//! scenario plus the probe-time host — never the [`crate::scenario::GroundTruth`], so the analyzer
//! provably cannot observe a command's true effect (rul24-overtype, the whole point of the split).
//!
//! [`evolve`] then walks the two host copies from the same S0: **bare** applies every mutator's
//! true effect in book order; **plan** applies only the `Run`-disposition sites' effects (an
//! elided/omitted site "didn't run"). End-state equality of the two is the soundness oracle
//! (24B §5).
//!
//! Approach-#3 discipline (`notes/123` f18/f23): the kernel is pure, the sole entropy is the
//! generator's seed, divergence is injected by the generator (declared-vs-true) — so there is NO
//! madsim, NO tokio-shim, NO libc-override, NO `async` here, and the net is trivially
//! cross-platform (it dodges the Mac/Windows determinism trap general Rust DST fights).

use std::collections::BTreeSet;

use dorc_analysis::cfg::CfgNodeId;
use dorc_analysis::effect::SkipClass;
use dorc_analysis::value::{ValueFlow, ValueOf};
use dorc_core::{FactKey, Interner, Observable, ProvArena, Symbol, Verdict};
use dorc_hostsim::Host;
use dorc_oracle::predict::PredictSet;
use dorc_oracle::touches::{TouchesResolution, TouchesSet, TouchesTop, evaluate_touches};
use dorc_plan::{
    DerivationShip, Disposition, EntityCoord, Footprint, Plan, TrustedFootprints,
    compile_derivations,
};

use crate::ORACLE_SH;
use crate::scenario::{DeclaredScenario, GroundTruth};

/// Run the real kernel over a scenario's DECLARED half and observe `s0` (24B §3). `flag_on`
/// selects the survival tier: `true` lifts footprints and passes them to
/// [`dorc_plan::build_plan_walled`] (the golden hill); `false` passes `None` (the honest Stage-1
/// baseline). Ground truth is NOT a parameter — the analyzer cannot see a command's true effect,
/// which is exactly the property that makes the honesty split meaningful (rul24-overtype).
///
/// The composition mirrors the cli edge byte-for-byte in spirit: lift the oracle's effect-map +
/// predict-set, classify (taking the `kills` set — 24A §3), compile the probe (the
/// can't-probe ⇒ can't-elide gate), lift the footprints under the flag, then build the walled
/// plan with `observe` sourced from `s0` through the probe's `checks_fact` gate.
#[must_use]
pub fn run_kernel(declared: &DeclaredScenario, s0: &Host, flag_on: bool, i: &mut Interner) -> Plan {
    let parsed = dorc_syntax::parse(&declared.book_sh);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed.value, i);

    let oracle_refs = [ORACLE_SH];
    let idx = dorc_oracle::lift(i, &oracle_refs).value;
    let checks: Vec<PredictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(i, src).value)
        .collect();

    let mut arena = ProvArena::new();
    let (classified, _why, kills, _kill_coords) = dorc_analysis::effect::classify_with_why_diags(
        &cfg,
        &value,
        &parsed.value,
        &idx,
        &checks,
        i,
        &mut arena,
    );
    let classes = classified.value;

    // The probe: a site is elidable only if its fact is actually checked (can't-probe ⇒
    // can't-elide). `ship` reborrows the interner immutably; it is dropped when `compile_probe`
    // returns, before the footprint lift reborrows `i` mutably.
    let probe = {
        let ship = |provider: Symbol, argv: &[Symbol]| {
            ship_predict_body(ORACLE_SH, &checks, i, provider, argv)
        };
        // The sweep exercises the elision/survival soundness net, not the GUARD tier: no
        // EstablishWritten site ships a guard probe (`is_vouched: |_| false` — guard scenarios are a
        // Stage-3 stretch, tc-flagged in the report). The ELIDE vouches (below) are separate — an
        // ambient site always ships its probe, and the elide-weld demands its vouch.
        dorc_plan::compile_probe(&parsed.value, &cfg, &value, &classes, ship, |_| false)
    };

    // The survival tier data (TC-1): lifted ONLY under the flag; `None` off ⇒ the total-wall
    // baseline, and the footprints never exist. Under the flag, the AUTHORED footprints (static
    // touches) are merged with the host-DERIVED ones (24E §6 — escalated `place` walls; the sweep
    // mirror of the cli's derived pipeline stage, sourcing the footprint from `Host::derive`).
    let survival = if flag_on {
        let mut fps = build_survival_footprints(&classes, &kills, &value, i);
        merge_derived_footprints(
            &mut fps,
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &kills,
            s0,
            i,
        );
        Some(fps)
    } else {
        None
    };

    // The elide-weld (24D §3): a converged ambient site elides ONLY with a reached vouch. Thread
    // them via the shared `dorc_plan::build_vouches` (the SAME composition the cli drives), or
    // every `install` victim would run and the net's elision coverage would vanish. Always-on
    // (independent of `flag_on`, which gates only the survival tier); the lift diags are dropped
    // (the net asserts on behaviour, not stderr text).
    let vouches = dorc_plan::build_vouches(&[ORACLE_SH], &classes, &value, i).value;

    // The identity-CANONICALIZATION map (24F §3/§7.1): built from the modeled host's DECLARED
    // resolver answers (`Host::resolve` — the sweep stand-in for shipping `package.resolve()` +
    // reading its per-coordinate stdout, mirroring how the derived footprint sources from
    // `Host::derive`). An HONEST resolver maps two aliased names to one canonical (the closure
    // DEMOTES the victim ⇒ safe); a LYING one keeps them apart (the victim wrongly survives ⇒ RED).
    // Consumed only in the survival walk (flag-on); flag-off ignores it (total-wall baseline).
    let resolutions = build_resolutions(s0);

    dorc_plan::build_plan_walled(
        &declared.book_sh,
        &parsed.value,
        &cfg,
        &classes,
        &kills,
        survival.as_ref(),
        Some(&resolutions),
        &vouches,
        |f| {
            if probe.checks_fact(f) {
                s0.observe(f)
            } else {
                Observable::verdict_only(Verdict::Unknown)
            }
        },
        &mut arena,
    )
}

/// Evolve two host copies from `s0` and return `(S_bare, S_apply)` (24B §3). **bare** applies
/// every mutator's true effect in book order (the whole book ran); **plan** applies only the
/// `Run`-disposition sites' effects (an elided/omitted site applies nothing — it "didn't run").
/// The zip is 1:1 by book order — the plan's steps are span-sorted (== book order) and the
/// generator emits one command per site — so a length mismatch is a generator/kernel drift and
/// panics WITH nothing to hide (a structural bug in the net itself, not an untrusted-input path).
///
/// `pub(crate)`: the only consumer of [`GroundTruth`], keeping ground truth off every public
/// surface.
pub(crate) fn evolve(
    s0: &Host,
    plan: &Plan,
    ground: &GroundTruth,
) -> (BTreeSet<FactKey>, BTreeSet<FactKey>) {
    assert_eq!(
        plan.steps.len(),
        ground.site_effects.len(),
        "sweep net drift: {} plan steps vs {} generated commands — the 1:1 book-order zip broke",
        plan.steps.len(),
        ground.site_effects.len(),
    );
    let mut bare = s0.clone();
    let mut applied = s0.clone();
    for (step, effect) in plan.steps.iter().zip(&ground.site_effects) {
        let Some(effect) = effect else { continue };
        bare.apply_delta(effect.delta());
        if matches!(step.disposition, Disposition::Run) {
            applied.apply_delta(effect.delta());
        }
    }
    (bare.snapshot(), applied.snapshot())
}

// ===========================================================================
// The cli composition, replicated (mirrors cli/src/main.rs — do not diverge)
// ===========================================================================

/// Resolve the stripped `<provider>__predict` funcdef a probe site ships (mirror of the cli's
/// `ship_predict_body`): the first check whose provider matches and whose own argparse resolves
/// this argv, stripped. `None` ⇒ un-shippable ⇒ the site is un-elidable (`kFAIL-perform`). One
/// oracle source here, so the cli's `zip(oracle_srcs, checks)` collapses to a single pairing.
fn ship_predict_body(
    oracle_src: &str,
    checks: &[PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
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
                return Some(strip_predict(oracle_src, check, interner));
            }
        }
    }
    None
}

/// Lift the survival footprints (mirror of the cli's `build_survival_footprints`) — with the
/// load-bearing **coherence check** kept (24C resid-argparse-drift is CONTAINED by it: a drifted
/// establish footprint fails at-least ⊆ at-most ⇒ refused ⇒ the site walls, fail-safe). The cli's
/// advisory-diag reporting is dropped (the net asserts on behaviour, not stderr text). Kill-walls
/// skip coherence (no single establish cell — 24C resid-kill-coherence).
fn build_survival_footprints(
    classes: &[(CfgNodeId, SkipClass)],
    kills: &BTreeSet<CfgNodeId>,
    value: &ValueFlow,
    interner: &mut Interner,
) -> TrustedFootprints {
    let touches_sets: Vec<TouchesSet> = [ORACLE_SH]
        .iter()
        .map(|src| TouchesSet::lift(interner, src).value)
        .collect();

    let mut footprints = TrustedFootprints::new();
    for (node, class) in classes {
        let establish = match class {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        };
        if establish.is_none() && !kills.contains(node) {
            continue; // not a wall candidate
        }
        let Some((provider, coords)) =
            resolve_touches_footprint(*node, value, &touches_sets, interner)
        else {
            continue; // no touches / non-literal argv / ⊤ / empty emission ⇒ no footprint ⇒ wall
        };
        // Coherence (establish sites only): the site's own establish coordinate must be inside its
        // footprint (at-least ⊆ at-most); a violation refuses the footprint ⇒ the site walls. (The
        // cli additionally renders a `footprint-incoherent` span-diag here; the net just walls.)
        if let Some(fact) = establish {
            let own = EntityCoord::new(fact.kind, fact.entity);
            if !coords.contains(&own) {
                continue;
            }
        }
        if let Some(footprint) = Footprint::authored(provider, coords) {
            footprints.insert(*node, footprint);
        }
    }
    footprints
}

/// Merge the host-DERIVED footprints (24E §2/§6 — the sweep mirror of the cli's
/// `merge_derived_footprints`): under the flag, an escalated wall-candidate's DECLARED footprint
/// comes from the host's derivation-answer ([`Host::derive`]) — the sweep stand-in for shipping the
/// `touches()` body + reading its stdout (NO sh execution here; the declared entity-set IS the
/// answer). Rides the declared-vs-true split: a manifest ⊂ the wall's TRUE `CellDelta` is the LYING
/// derived footprint that makes the victim wrongly survive (fork-s4-declaredtrue). Coherence-checked
/// (own establish coord ⊆ footprint) exactly as the authored lane. The escalation DETECTION lifts +
/// traces the real `touches()` (parity with the cli); only the footprint SOURCE is the host-answer.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the cli's derived-merge — threads the compiled context (ast/cfg/value/classes/kills) + the probe-time host + interner; each is a distinct pipeline output, not a bundle-able struct"
)]
fn merge_derived_footprints(
    footprints: &mut TrustedFootprints,
    ast: &dorc_syntax::ast::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &ValueFlow,
    classes: &[(CfgNodeId, SkipClass)],
    kills: &BTreeSet<CfgNodeId>,
    s0: &Host,
    interner: &mut Interner,
) {
    let touches_sets: Vec<TouchesSet> = [ORACLE_SH]
        .iter()
        .map(|src| TouchesSet::lift(interner, src).value)
        .collect();
    let derivations = {
        let derive = |provider: Symbol, argv: &[Symbol]| {
            ship_touches_escalation(&touches_sets, interner, provider, argv)
        };
        compile_derivations(ast, cfg, value, classes, kills, derive)
    };
    for d in &derivations.derivations {
        // The wall's own establish cell keys `Host::derive` (the declared manifest) AND is the
        // coherence comparand. A kill's coordinate would ride the killcoord side-map (24E §7).
        let Some(fact) = establish_fact_of(classes, d.node) else {
            continue;
        };
        let coords: Vec<EntityCoord> = s0
            .derive(fact)
            .into_iter()
            .map(|f| EntityCoord::new(f.kind, f.entity))
            .collect();
        let own = EntityCoord::new(fact.kind, fact.entity);
        if !coords.contains(&own) {
            continue; // coherence-refuse (own establish ⊄ footprint) ⇒ the site walls (fail-safe)
        }
        if let Some(fp) = Footprint::derived(d.provider, coords, "apt-get.touches()".to_owned()) {
            footprints.insert(d.node, fp);
        }
    }
}

/// Build the identity-canonicalization map (24F §3/§7.1 — the sweep mirror of the cli's resolver
/// readback): copy the modeled host's DECLARED resolver-bearing kinds + per-coordinate canonicals
/// into a [`dorc_plan::Resolutions`]. This is the sweep stand-in for shipping `<kind>.resolve()` and
/// reading its per-coordinate stdout — NO sh execution here; the host's declared answer IS the
/// resolution (exactly as `Host::derive` stands in for the derived-footprint readback). Rides the
/// declared-vs-TRUE split for IDENTITY: an HONEST answer merges two aliased names to one canonical;
/// a LYING answer keeps them apart (the victim wrongly survives ⇒ the end-state differential RED).
fn build_resolutions(s0: &Host) -> dorc_plan::Resolutions {
    let mut resolutions = dorc_plan::Resolutions::none();
    for kind in s0.resolver_kinds() {
        resolutions.add_resolver_kind(kind);
    }
    for ((kind, entity), canonical) in s0.resolutions() {
        resolutions.record(EntityCoord::new(kind, entity), canonical);
    }
    resolutions
}

/// The escalation seam for the sweep's `compile_derivations` (mirror of the cli's
/// `ship_touches_body`): `Some` iff a wall-candidate's `touches()` trace ⊤s on `NonPrintfCommand`
/// (it reached the host tool `apt-manifest` — the `place` verb's arm). The sweep sources the
/// footprint from [`Host::derive`], NOT the shipped body, so `sh` is unused (no derivation-probe
/// render); the escalation SIGNAL is all `compile_derivations` needs.
fn ship_touches_escalation(
    touches_sets: &[TouchesSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<DerivationShip> {
    use dorc_oracle::predict::map_provider_name;
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    touches_sets.iter().find_map(|set| {
        let p = set
            .providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
        let touches = set.get(p)?;
        match evaluate_touches(touches, &arg_refs) {
            TouchesResolution::Top(TouchesTop::NonPrintfCommand) => Some(DerivationShip {
                sh: String::new(),
                call: "apt-get.touches()".to_owned(),
            }),
            TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
        }
    })
}

/// The establish fact a wall-candidate node establishes (the `Host::derive` key + coherence
/// comparand for a derived footprint). `None` for a non-establish class.
fn establish_fact_of(classes: &[(CfgNodeId, SkipClass)], node: CfgNodeId) -> Option<FactKey> {
    classes.iter().find_map(|(n, c)| {
        if *n != node {
            return None;
        }
        match c {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        }
    })
}

/// Resolve one wall-candidate site's `touches()` footprint (mirror of the cli's
/// `resolve_touches_footprint`): split the resolved argv into `(provider, operands)` (all
/// literal), find the provider's touches funcdef, trace it, intern the emitted coordinates.
/// `None` ⇒ non-literal argv, no matching `touches()`, a ⊤ trace, or an EMPTY emission (wall).
fn resolve_touches_footprint(
    node: CfgNodeId,
    value: &ValueFlow,
    touches_sets: &[TouchesSet],
    interner: &mut Interner,
) -> Option<(Symbol, Vec<EntityCoord>)> {
    use dorc_core::{EntityRef, KindId, OpaqueToken};
    use dorc_oracle::predict::map_provider_name;

    let argv = value.argv_values(node);
    let (first, rest) = argv.split_first()?;
    let ValueOf::Literal(provider) = first else {
        return None;
    };
    let mut arg_texts = Vec::with_capacity(rest.len());
    for w in rest {
        let ValueOf::Literal(s) = w else {
            return None;
        };
        arg_texts.push(interner.resolve(*s).to_owned());
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    let want = map_provider_name(interner.resolve(*provider));
    let coords = touches_sets.iter().find_map(|set| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .and_then(|touches| match evaluate_touches(touches, &arg_refs) {
                TouchesResolution::Emitted(coords) if !coords.is_empty() => Some(coords),
                TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
            })
    })?;

    let entity_coords = coords
        .iter()
        .map(|c| {
            let kind = KindId(interner.intern(&c.kind));
            let entity = match &c.entity {
                Some(text) => EntityRef::Operand(OpaqueToken(interner.intern(text))),
                None => EntityRef::Singleton,
            };
            EntityCoord::new(kind, entity)
        })
        .collect();
    Some((*provider, entity_coords))
}
