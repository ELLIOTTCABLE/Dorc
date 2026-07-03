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
use dorc_oracle::touches::{TouchesResolution, TouchesSet, evaluate_touches};
use dorc_plan::{Disposition, EntityCoord, Footprint, Plan, TrustedFootprints};

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
    let (classified, _why, kills) = dorc_analysis::effect::classify_with_why_diags(
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
        dorc_plan::compile_probe(&parsed.value, &cfg, &value, &classes, ship)
    };

    // The survival tier data (TC-1): lifted ONLY under the flag; `None` off ⇒ the total-wall
    // baseline, and the footprints never exist.
    let survival = if flag_on {
        Some(build_survival_footprints(&classes, &kills, &value, i))
    } else {
        None
    };

    dorc_plan::build_plan_walled(
        &declared.book_sh,
        &parsed.value,
        &cfg,
        &classes,
        &kills,
        survival.as_ref(),
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
        if let Some(footprint) = Footprint::new(provider, coords) {
            footprints.insert(*node, footprint);
        }
    }
    footprints
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
