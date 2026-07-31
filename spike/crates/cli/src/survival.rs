//! The SURVIVAL lane: authored + derived `disturbs()` footprints, the wrapped-site peel, and the
//! diagnostics both raise.
//!
//! Third instance of the loom-seam pattern (`lib-target-is-a-loom-seam`, after `kinds` and
//! `results`): every function here is a pure function of already-read oracle SOURCE plus the
//! analysis products the kernel built, so the binary and a harness can call the SAME one and a
//! defining case for one of its codes fires the run's own check rather than a second implementation
//! of it (`289:rul-worldless-route-honest-trigger`). Nothing here opens a file, reads a clock, or
//! touches a host.
//!
//! Flag-gated by construction: with `--risk-faultless-skips` off, `touches_paired` is empty, the
//! derivation lane compiles nothing, and the whole module is invisible
//! (`empty-world-byte-identical`).

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::diag::{
    CarriedAcrossSubstrateAxis, DerivFamilyIncomplete, Diag, DiagCode, FootprintIncoherent,
    TouchesEscalated, WrappedSiteAdoptionHint,
};
use dorc_aid::{Carrier, CollapseKind, CollapseNarrative, SpeechAct};
use dorc_core::{Interner, Symbol};

use crate::results::SiteResults;
use crate::why::oracle_locus;
use crate::world::{ship_predict_body, ship_verdict_body};

/// Lift each oracle's `touches()` set for the authored survival lane, carrying the lift's
/// diagnostics out as data rather than printing them from inside the lift (`io-at-edges-only`).
pub fn lift_touches_sets(
    oracle_refs: &[&str],
    interner: &mut Interner,
) -> Carrier<Vec<dorc_oracle::touches::TouchesSet>> {
    let mut diags = Vec::new();
    let sets = oracle_refs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::touches::TouchesSet::lift(interner, src);
            diags.extend(lifted.diags);
            lifted.value
        })
        .collect();
    Carrier::new(sets, diags)
}

/// Lift the survival footprints (Stage 2 / rul24-mode-gate) — called ONLY on the
/// `--risk-faultless-skips` path (TC-1: the footprint data does not exist unflagged). For each
/// wall-candidate site (an establish-bearing class, or a kill) whose provider declares a
/// `touches()`, trace it over the site's resolved argv and record the emitted footprint —
/// after a **coherence check** (23M / the Stage-2 brief): the site's OWN establish coordinate
/// must be ⊆ its lifted footprint (at-least ⊆ at-most), else the footprint is a loud
/// contradiction and is REFUSED (⇒ the site walls). A ⊤/empty lift, a non-literal argv, or a
/// missing `touches()` all mean "no trustworthy footprint" ⇒ absence from the map ⇒ wall.
///
/// `inv-referent-agnostic`: emitted `kind:entity` fragments are interned into the SAME
/// vocabulary the book/predict analysis uses (one interner) — `package` here is the SAME
/// [`KindId`] a predict annotation minted — never a parallel string-typed universe (24A §1b).
#[expect(
    clippy::too_many_arguments,
    reason = "the cli-edge footprint lift threads the whole compiled context (touches-sets/classes/kills/kill-coords/value/cfg/ast/interner); each is a distinct pipeline output, not a bundle-able struct"
)]
pub fn build_survival_footprints(
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    value: &dorc_analysis::value::ValueFlow,
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    interner: &mut Interner,
) -> Carrier<dorc_plan::TrustedFootprints> {
    use dorc_analysis::effect::SkipClass;
    let mut footprints = dorc_plan::TrustedFootprints::new();
    let mut diags = Vec::new();
    for (node, class) in classes {
        // A wall candidate: an establish-bearing class OR a kill. Both now carry their OWN effect
        // coordinate for the coherence check (24E §7: the kill's coord rides `kill_coords`).
        let establish = match class {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        };
        if establish.is_none() && !kills.contains(node) {
            continue; // not a wall candidate (a pure builtin, a Query, an opaque)
        }
        let Some((provider, coords_with_selectors, arm_span)) =
            resolve_touches_footprint(*node, value, touches_sets, interner)
        else {
            continue; // no touches / non-literal argv / ⊤ / empty emission ⇒ no footprint ⇒ wall
        };
        let coords: Vec<dorc_plan::EntityCoord> =
            coords_with_selectors.iter().map(|(c, _)| *c).collect();
        let own = own_wall_coord(*node, classes, kill_coords);
        // Coherence CANARY (authored lane only, PRE-union — 24G §8 / 24E §7): the site's OWN effect
        // coordinate (its establish, or its killed cell) must be ⊆ the author's RAW `touches()`
        // emission (at-least ⊆ at-most). A violation is a cross-lane contradiction — the author's
        // touches() disagrees with their own establish/kill — ⇒ refuse ⇒ wall. Real teeth here, and
        // UNCHANGED. Closes resid-kill-coherence (a drifted kill footprint omitting the killed cell).
        if let Some(own_coord) = own
            && !coords.contains(&own_coord)
        {
            let span = ast.node(cfg.node(*node).ast).span;
            diags.push(Diag::new(
                DiagCode::FootprintIncoherent(FootprintIncoherent {
                    detail: "touches() footprint omits this command's own effect coordinate \
                             (at-least not-within at-most) -- footprint refused, the site walls"
                        .to_string(),
                }),
                span,
            ));
            continue;
        }
        // 24G §8: UNION the site's own effect coordinate (engine-supplied provenance) into the
        // footprint. A no-op on the hit-surface HERE (the canary just proved own ∈ coords), but it
        // records own for the why-lens and keeps the two lanes uniform. Empty emission ⇒ None from
        // `authored` ⇒ `with_own` cannot resurrect it (anti-233).
        // `tc-disturbs-span-threading`: the MATCHED ARM over the funcdef, still the honest floor.
        let defining = arm_span.or_else(|| touches_defining_span(provider, touches_sets, interner));
        if let Some(mut footprint) = dorc_plan::Footprint::authored(provider, coords)
            .map(|fp| fp.with_own(own).with_defining(defining))
        {
            // `277` §3: record each emission's `@selector` so a selector-bearing disturbs mark can
            // SPARE a sibling cell under the dialect. Whole-entity emissions (the corpus default,
            // `None`) record nothing ⇒ ⊤ ⇒ collide (empty-world-byte-identical).
            for (coord, selector) in coords_with_selectors {
                if let Some(sel) = selector {
                    footprint.set_selector(coord, sel);
                }
            }
            footprints.insert(*node, footprint);
        }
    }
    Carrier::new(footprints, diags)
}

/// Resolve a wall-candidate site's `touches()` footprint: split its resolved argv into
/// `(provider, operands)` (all must be literal — a ⊤ word ⇒ no footprint), find the provider's
/// touches funcdef (through the shared hyphen↔underscore convention, like the probe), trace it,
/// and intern the emitted coordinates. `None` ⇒ any of: non-literal argv, no matching
/// `touches()`, a ⊤ trace, or an EMPTY emission (no claim = wall).
/// One footprint coordinate plus its disturbs-emission selector (`277` §3): the entity-granular
/// [`dorc_plan::EntityCoord`] that drives canonicalization/render, and the `@selector` cell the
/// dialect consults (`None` ⇒ whole-entity ⊤).
type FootprintCoord = (dorc_plan::EntityCoord, Option<dorc_core::SelectorId>);

/// One resolved `disturbs` footprint: whose claim it is, the cells it names, and the arm that
/// emitted them (`tc-disturbs-span-threading`; `None` when the trace located no emitting line).
type ResolvedFootprint = (
    Symbol,
    Vec<FootprintCoord>,
    Option<(dorc_core::Span, dorc_core::OracleFileId)>,
);

/// Resolve one wall-candidate site's authored `disturbs()` footprint (see the type doc above).
pub fn resolve_touches_footprint(
    node: dorc_analysis::cfg::CfgNodeId,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
) -> Option<ResolvedFootprint> {
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::touches::{TouchesResolution, evaluate_touches_located};

    let argv = value.argv_values(node);
    let (first, rest) = argv.split_first()?;
    let ValueOf::Literal(provider) = first else {
        return None; // ⊤ command word
    };
    let mut arg_texts = Vec::with_capacity(rest.len());
    for w in rest {
        let ValueOf::Literal(s) = w else {
            return None; // a ⊤ operand ⇒ the argparse cannot resolve ⇒ no footprint
        };
        arg_texts.push(interner.resolve(*s).to_owned());
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    let want = map_provider_name(interner.resolve(*provider));
    let (coords, arm) = touches_sets.iter().enumerate().find_map(|(index, set)| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .and_then(
                |touches| match evaluate_touches_located(touches, &arg_refs) {
                    (TouchesResolution::Emitted(coords), arm) if !coords.is_empty() => Some((
                        coords,
                        arm.map(|span| {
                            (
                                span,
                                dorc_core::OracleFileId(u32::try_from(index).unwrap_or(u32::MAX)),
                            )
                        }),
                    )),
                    // Emitted(empty) = no claim = wall; Top = ⊤ = wall. Both ⇒ no footprint.
                    (TouchesResolution::Emitted(_) | TouchesResolution::Top(_), _) => None,
                },
            )
    })?;

    // Intern each opaque `kind:entity@selector` fragment into the shared vocabulary (the fence).
    // The selector rides alongside the entity-granular coord (`277` §3): absent ⇒ whole-entity ⊤.
    let entity_coords = coords
        .iter()
        .map(|c| {
            let kind = dorc_core::KindId(interner.intern(&c.kind));
            let entity = match &c.entity {
                Some(text) => {
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(text)))
                }
                None => dorc_core::EntityRef::Singleton,
            };
            let selector = c
                .selector
                .as_deref()
                .map(|s| dorc_core::SelectorId(interner.intern(s)));
            (dorc_plan::EntityCoord::new(kind, entity), selector)
        })
        .collect();
    Some((*provider, entity_coords, arm))
}

/// The `disturbs` funcdef's defining `(Span, OracleFileId)` for a provider (`tc-disturbs-span-
/// threading`; `27V:mech-minting-line-threading`) — a NAME-keyed lookup (no argv trace): the touches
/// funcdef's `name_span` is the leverage point a survival's `claimed` link points at ("the line to
/// widen"). The funcdef `name_span` is the honest coarsest-true span; per-arm precision is deferred.
/// `None` when the provider has no touches funcdef in the loaded set.
fn touches_defining_span(
    provider: Symbol,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &Interner,
) -> Option<(dorc_core::Span, dorc_core::OracleFileId)> {
    use dorc_oracle::predict::map_provider_name;
    let want = map_provider_name(interner.resolve(provider));
    touches_sets.iter().enumerate().find_map(|(idx, set)| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .map(|t| {
                (
                    t.name_span,
                    dorc_core::OracleFileId(u32::try_from(idx).unwrap_or(u32::MAX)),
                )
            })
    })
}

/// The derivation-probe seam (24E §2/§3 — fork-4A: the SAME self-vouch tier as `predict`, no new
/// trust edge): for a wall-candidate site's (provider-word, argv), find the provider's `touches()`
/// funcdef and trace it statically. `Some(DerivationShip)` iff the trace ESCALATED — it ⊤'d
/// specifically on a `NonPrintfCommand` (the body reached a host query the static tracer cannot
/// resolve, e.g. `dpkg -L`), the sanctioned escalation trigger (fork-4B). The body then ships
/// strip-only (`strip_touches`; the funcdef mangles to `<provider>__disturbs`), the SAME strip
/// discipline as the probe/guard lanes. `None` for: a statically-resolvable body (`Emitted` — the
/// authored-footprint lane owns it), any OTHER ⊤ (degrade-to-wall, fork-4B — the site runs), an
/// empty emission, or a provider with no touches funcdef. `inv-referent-agnostic`: the operands are
/// resolved for the trace/invocation, never decoded.
pub fn ship_touches_body(
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<dorc_plan::DerivationShip> {
    use dorc_oracle::predict::{map_provider_name, strip_touches};
    use dorc_oracle::touches::{TouchesResolution, TouchesTop, evaluate_touches};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    touches_paired.iter().find_map(|(src, set)| {
        let p = set
            .providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
        let touches = set.get(p)?;
        match evaluate_touches(touches, &arg_refs) {
            // The EXPECTED escalation (24E §4): the body reached a host query ⇒ ship it.
            TouchesResolution::Top(TouchesTop::NonPrintfCommand) => {
                Some(dorc_plan::DerivationShip {
                    // Display the BOOK command word (`apt-get`), not the munged funcdef segment
                    // (`apt_get`, the forward-munge key) — the why-lens reads better with the word
                    // the admin wrote (`24C:rul24-totalistic-munge` keeps the segment internal).
                    call: format!("{}.touches()", interner.resolve(provider)),
                    sh: strip_touches(src, touches, interner),
                })
            }
            // Static-resolvable, an OTHER ⊤ (degrade-to-wall), or empty ⇒ NOT a derivation.
            TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
        }
    })
}

/// Read back the host-DERIVED footprints (24E §2 corr-§2) and merge them into the survival set.
/// For each escalated [`dorc_plan::ProbeDerivation`], intern its readback `deriv` coordinate lines
/// into the SHARED vocabulary (the 24A §1b fence — `package` here is the SAME [`dorc_core::KindId`]
/// a predict annotation minted), build a `Derived` [`dorc_plan::Footprint`], and UNION the site's own
/// effect coordinate into it (24G §8 — the derived lane no longer REQUIRES own-membership; the
/// boilerplate `printf 'kind:%s' "$1"` that used to supply it was a decoy the coherence check tested
/// instead of the derivation). Insert keyed by the site's node. An escalated site with NO readback
/// records ⇒ empty ⇒ wall (silence = wall, kFAIL-safe).
///
/// ALL-OR-NOTHING (24E §4 / the static path's TC-4): a MALFORMED derived coordinate refuses the
/// WHOLE footprint (the site walls) — never silently dropped, because a footprint is an *at-most*
/// claim and dropping a coordinate NARROWS it (⇒ a downstream fact wrongly survives ⇒ under-execute).
///
/// SPIKE-ONLY (ru-26): the `touches-escalated` advisory below makes the static→dynamic boundary
/// visible in the render/differential; it must NOT leak into greenfield as a permanent
/// per-escalation requirement.
// The product is the `&mut` merge, so the diagnostics are the one thing a caller can drop without
// noticing — which is the failure this whole lane exists to make impossible.
#[must_use]
pub fn merge_derived_footprints(
    footprints: &mut dorc_plan::TrustedFootprints,
    derivations: &dorc_plan::DerivationPlan,
    results: &SiteResults,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    node_spans: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::Span>,
    interner: &mut Interner,
) -> Vec<Diag> {
    let mut diags = Vec::new();
    for d in &derivations.derivations {
        // The escalated book command's own span (`aid-caret-span-precision`): every diag this loop
        // emits points at the site that escalated (the same `CfgNodeId`→AST span the canary uses,
        // precomputed at the cli edge). Absent ⇒ wall this site silently (kFAIL-safe; never happens
        // in production, where the map covers every derivation node).
        let Some(&span) = node_spans.get(&d.node) else {
            continue;
        };
        diags.push(Diag::new(
            DiagCode::TouchesEscalated(TouchesEscalated {
                site: d.site.0,
                call: d.call.clone(),
            }),
            span,
        ));
        let Some(coord_strs) = results.derivations.get(&d.site) else {
            continue; // no readback records ⇒ empty derived footprint ⇒ wall (kFAIL-safe)
        };
        // `262` §2 / `26A` stop-1 — THE at-most family completeness gate. A deriv footprint is
        // an AT-MOST claim, so a mid-family cut SHRINKS it (⇒ more survivals — the
        // under-execution cardinal sin). The family MUST close with `deriv-end n=<K>` whose K
        // equals the received coord count; a missing end-record or a count mismatch ⇒ the
        // family is INCOMPLETE ⇒ refuse the footprint ⇒ the site walls TOTAL (never keep a
        // partial at-most family). This is the SAME wall-total path as the malformed-coord
        // refusal below.
        match results.derivation_ends.get(&d.site) {
            // Legacy (unframed) fixtures carry no `deriv-end`; they are trusted-complete, so the
            // gate is framed-only (the framed round-trip + DST enforce the real contract).
            _ if !results.framed => {}
            Some(&k) if k as usize == coord_strs.len() => {}
            reason => {
                diags.push(Diag::new(
                    DiagCode::DerivFamilyIncomplete(DerivFamilyIncomplete {
                        site: d.site.0,
                        reason: match reason {
                            Some(&k) => format!("declared n={k}, received {}", coord_strs.len()),
                            None => "no deriv-end close-record".to_string(),
                        },
                    }),
                    span,
                ));
                continue;
            }
        }
        let mut coords = Vec::with_capacity(coord_strs.len());
        let mut malformed = false;
        for line in coord_strs {
            if let Some(c) = intern_coordinate(line, interner) {
                coords.push(c);
            } else {
                malformed = true;
                break;
            }
        }
        if malformed {
            diags.push(Diag::new(
                DiagCode::FootprintIncoherent(FootprintIncoherent {
                    detail: "derived touches() emitted a malformed coordinate (not kind:entity) \
                             -- footprint refused, the site walls (an at-most claim cannot be \
                             partial)"
                        .to_string(),
                }),
                span,
            ));
            continue;
        }
        // 24G §8: the DERIVED lane DROPS the own-membership requirement — the boilerplate
        // `printf 'kind:%s' "$1"` that satisfied it was a DECOY the check tested INSTEAD of the
        // derivation. UNION the site's own effect coordinate (its establish, or its killed cell from
        // `kill_coords`) into the footprint instead — engine-supplied provenance. An empty emission
        // still walls: `derived` returns None on empty coords ⇒ `with_own` cannot resurrect it (the
        // anti-233 boundary — the engine never manufactures a claim from silence).
        let own = own_wall_coord(d.node, classes, kill_coords);
        if let Some(fp) = dorc_plan::Footprint::derived(d.provider, coords, d.call.clone())
            .map(|fp| fp.with_own(own))
        {
            footprints.insert(d.node, fp);
        }
    }
    diags
}

/// Intern one readback `kind:entity` coordinate line into the shared vocabulary (24A §1b fence —
/// split on the FIRST `:`; an empty entity is the kind's singleton). `None` on a malformed line
/// (no `:` / empty kind) — the caller refuses the WHOLE footprint (all-or-nothing).
fn intern_coordinate(line: &str, interner: &mut Interner) -> Option<dorc_plan::EntityCoord> {
    let (kind, entity) = line.split_once(':')?;
    if kind.is_empty() {
        return None;
    }
    let kind = dorc_core::KindId(interner.intern(kind));
    let entity = if entity.is_empty() {
        dorc_core::EntityRef::Singleton
    } else {
        dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(entity)))
    };
    Some(dorc_plan::EntityCoord::new(kind, entity))
}

/// The establish fact a wall-candidate node establishes, if it is an establish class. A kill's
/// coordinate rides the `kill_coords` side-map instead (24E §7).
fn establish_fact_of(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    node: dorc_analysis::cfg::CfgNodeId,
) -> Option<dorc_core::FactKey> {
    use dorc_analysis::effect::SkipClass;
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

/// The wall-candidate node's OWN effect coordinate — the coherence comparand (own ⊆ footprint,
/// 24E §7): its establish coordinate (an establish class) OR its killed coordinate (a kill node,
/// from `kill_coords`). `None` for a node with neither (nothing to check coherence against). This
/// unifies the establish-wall check (Stage 2) with the kill-wall check (24E §7) for BOTH the
/// authored and derived footprint lanes.
#[must_use]
pub fn own_wall_coord(
    node: dorc_analysis::cfg::CfgNodeId,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
) -> Option<dorc_plan::EntityCoord> {
    establish_fact_of(classes, node)
        .or_else(|| kill_coords.get(&node).copied())
        .map(|f| dorc_plan::EntityCoord::new(f.kind, f.entity))
}

/// The lane-integration `27N` product of the wrapped-BOOK-site analysis: the peel-map (for
/// `classify` to birth each wrapped fact in its context), the wrapped-probe decisions (for
/// `compile_probe`), and the adoption/disclosure hints.
#[derive(Debug)]
pub struct WrappedAnalysis {
    /// Wrapped sites keyed by [`CfgNodeId`] → (inner argv, composed context) for `classify`.
    pub peeled: BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_analysis::effect::PeeledSite>,
    /// Wrapped-probe dispositions (Enter / Degrade) keyed by node, for `compile_probe`.
    pub wrapped: dorc_plan::WrappedProbes,
    /// One-line adoption hints (a degraded-on-vouch site) + degrade disclosures (`27C` §2/§6).
    pub hints: Vec<Diag>,
    /// Pure-predicate-carry attribution chains keyed by the carried site's [`AstId`] (`27C` §4(a)):
    /// the why-lens tether emitted for every carried elision (`emit_carry_attribution`). Keyed by
    /// `AstId` so the plan's per-site step re-keys to the site number for the `why: site N …` line.
    pub carried: BTreeMap<dorc_core::AstId, String>,
    /// C5 aid plane (`27V` Lane A): the decision-inert [`CollapseKind::EntryDenial`] narrative minted
    /// when a wrapped site's entry consent degrades to guard/run (`two-plane-aid-law`; steers
    /// nothing). Threaded to the why-lens seam by the cli edge (d4 renders).
    pub collapse_narrative: Vec<CollapseNarrative>,
}

/// Build the wrapped-BOOK-site analysis (`27C` §3 / lane-integration `27N`): recognize each site
/// whose head is a loaded wrapper, peel it into (inner command, composed context), decide entry
/// (dial × capability × vouch × entry-form), and produce the peel-map + wrapped-probe decisions +
/// hints. Empty when no wrapper oracle is loaded ⇒ the pipeline is byte-identical
/// (`empty-world-byte-identical`). The entry-composed probe ships ONLY oracle bytes
/// (`271:rul-only-oracle-bytes-ship`); the admin's argv flows through the inner oracle's argparse.
#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "the wrapped-site analysis threads the whole compiled context (oracle sources + predict/verdict sets + cfg/value) plus the two admin axes (dial/capability); the per-site peel→resolve-inner→decide loop is one cohesive unit (`27N`), its sub-steps already extracted to build_wrapper_index + resolve_inner_check"
)]
pub fn build_wrapped_analysis(
    oracle_srcs: &[String],
    oracle_refs: &[&str],
    oracle_paths: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    ast: &dorc_syntax::ast::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    dial: dorc_core::EscalationDial,
    capability: dorc_core::Capability,
    interner: &mut Interner,
) -> WrappedAnalysis {
    use dorc_aid::narrative::EntryDegradeTag;
    use dorc_analysis::cfg::{CfgNodeId, CfgNodeKind};
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::entry::{EntryDecision, EntryDegrade, decide_entry, peel_book_chain};
    use dorc_oracle::predict::map_provider_name;

    let WrapperIndexBundle {
        wrappers,
        enter_defs,
        tolerance,
    } = build_wrapper_index(oracle_refs, verdict_sets, interner);

    let mut out = WrappedAnalysis {
        peeled: BTreeMap::new(),
        wrapped: dorc_plan::WrappedProbes::new(),
        hints: Vec::new(),
        carried: BTreeMap::new(),
        collapse_narrative: Vec::new(),
    };
    if wrappers.is_empty() {
        return out; // no wrapper oracle ⇒ nothing peels (rung-0 byte-identical)
    }

    // (A) the authored axis-invariance index (`27C` §4(a) pure-predicate carry) — lifted once from
    // every `state_stored_only_in()` body. Empty when no invariance line is declared ⇒ carry never
    // licenses (`silence-licenses-nothing`). Its netns-caveat contradictions are `validate`'s to
    // report: they are a property of the oracle file, and this walk never runs for a wrapper-free
    // unit.
    let (invariance, _) = dorc_oracle::carry::InvarianceIndex::lift(interner, oracle_refs);

    let command_nodes: Vec<CfgNodeId> = cfg
        .iter()
        .filter(|(id, n)| {
            n.kind == CfgNodeKind::Command
                && !cfg.is_expansion_internal(*id)
                && !cfg.is_spliced_internal(*id)
        })
        .map(|(id, _)| id)
        .collect();
    for node in command_nodes {
        // Resolve the site's whole argv to literals; a ⊤ word ⇒ not peelable (walls opaquely).
        let argv = value.argv_values(node);
        let mut argv_strs: Vec<String> = Vec::with_capacity(argv.len());
        for w in &argv {
            match w {
                ValueOf::Literal(s) => argv_strs.push(interner.resolve(*s).to_owned()),
                ValueOf::Top(_) => {
                    argv_strs.clear();
                    break;
                }
            }
        }
        let argv_refs: Vec<&str> = argv_strs.iter().map(String::as_str).collect();
        let Some(chain) = peel_book_chain(&argv_refs, &wrappers) else {
            continue; // not a wrapped site (or a wrapper that cannot peel ⇒ walls)
        };
        let Some((inner_word, inner_rest)) = chain.inner_argv.split_first() else {
            continue;
        };
        let context = chain.composed.to_context(interner);
        let inner_provider = interner.intern(&map_provider_name(inner_word));
        let inner_operands: Vec<Symbol> = inner_rest.iter().map(|a| interner.intern(a)).collect();
        let mut peeled_argv = vec![ValueOf::Literal(inner_provider)];
        peeled_argv.extend(inner_operands.iter().map(|s| ValueOf::Literal(*s)));
        // The inner check body (predict first, else the auto-cell verdict body) — mirrors the ambient
        // shape `compile_probe` would ship, now composed inside the entry chain.
        let Some((inner_fn, inner_sh)) = resolve_inner_check(
            oracle_srcs,
            checks,
            verdict_sets,
            inner_word,
            inner_provider,
            &inner_operands,
            interner,
        ) else {
            // No inner check ⇒ run; the fact is still born in-context for classify.
            out.peeled.insert(
                node,
                dorc_analysis::effect::PeeledSite {
                    inner_argv: peeled_argv,
                    context,
                },
            );
            out.wrapped.insert(node, dorc_plan::WrappedProbe::Degrade);
            continue;
        };
        let composed_enter_defs: Vec<(String, String)> = chain
            .links
            .iter()
            .filter_map(|l| l.entry.as_ref().and(enter_defs.get(&l.provider)).cloned())
            .collect();
        let composed = dorc_plan::EntryComposed {
            enter_defs: composed_enter_defs,
            inner_fn,
            inner_sh,
            inner_argv: inner_operands,
        };
        // An identity chain (HostDefault) needs NO entry — it ships the plain inner check in the
        // ambient world. A shifted chain runs the two-axis consent decision (`27C` §1).
        let decision = if context == dorc_core::Context::HostDefault {
            EntryDecision::Enter
        } else {
            let has_entry_form = chain.links.iter().all(|l| l.entry.is_some());
            let tolerated = tolerance
                .get(&inner_provider)
                .map(|t| t.tolerated_on_path(inner_rest.first().map(String::as_str)))
                .unwrap_or_default();
            decide_entry(
                has_entry_form,
                capability,
                dial,
                &chain.composed.crossed(),
                &chain.composed.walls(),
                &tolerated,
            )
        };
        // The fact's context: Wrapped for Enter/Degrade (born in-context); HostDefault for a
        // pure-predicate CARRY (measure ambient, carry across the substrate boundary, `27C` §4(a)).
        let (fact_context, probe) = match decision {
            EntryDecision::Enter => (
                context,
                dorc_plan::WrappedProbe::Enter {
                    provider: inner_provider,
                    composed,
                },
            ),
            EntryDecision::Degrade(reason) => {
                // Try pure-predicate carry (`27C` §4(a)) before defaulting to run. Gated on the
                // shipped inner check BEING the verdict body (auto-cell) — the closed body must be
                // the measured body; the predict-inner carry path is deferred (disclosed, `27O`).
                let carried = if composed.inner_fn.ends_with("__is_converged") {
                    try_carry(&chain, inner_provider, verdict_sets, &invariance)
                } else {
                    None
                };
                if let Some(read_kinds) = carried {
                    // Attribution chain (`27C` §9: every cross-context elision renders it from day
                    // one): the crossed substrate axes; each backing kind's owner `invariant:<axis>`
                    // line (vouch-species); the engine read-set-closure proof. One note per site,
                    // deterministic. Rides the diagnostic + why lanes only (two-surfaces: never the
                    // `.sh` artifact).
                    let span = ast.node(cfg.node(node).ast).span;
                    // render 3/3 (`27C` §9): each carried kind's owner `invariant:<axis>` line as
                    // `file:line` (first crossed axis with a threaded span wins; absent ⇒ no locus).
                    let loci: BTreeMap<String, String> = read_kinds
                        .iter()
                        .filter_map(|k| {
                            chain
                                .composed
                                .crossed()
                                .iter()
                                .find_map(|d| invariance.invariant_span(k, *d))
                                .and_then(|sp| oracle_locus(Some(sp), oracle_paths, oracle_srcs))
                                .map(|loc| (k.clone(), loc))
                        })
                        .collect();
                    let (axes, kinds) =
                        carry_attribution_values(&chain.composed.crossed(), &read_kinds, &loci);
                    out.hints.push(Diag::new(
                        DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                            axes: axes.clone(),
                            kinds: kinds.clone(),
                        }),
                        span,
                    ));
                    out.carried.insert(
                        cfg.node(node).ast,
                        carry_attribution_text(&chain.composed.crossed(), &read_kinds, &loci),
                    );
                    (
                        dorc_core::Context::HostDefault,
                        dorc_plan::WrappedProbe::Carry {
                            provider: inner_provider,
                            composed: dorc_plan::EntryComposed {
                                enter_defs: Vec::new(), // ambient: no entry form
                                ..composed
                            },
                        },
                    )
                } else {
                    // C5 aid: narrate the STATIC entry-degrade rung (`27C` §3; Consented-tier).
                    let rung = match reason {
                        EntryDegrade::NoCapability(_) => Some(EntryDegradeTag::NoCapability),
                        EntryDegrade::DialForbids => Some(EntryDegradeTag::DialForbids),
                        EntryDegrade::Unvouched(_) => Some(EntryDegradeTag::Unvouched),
                        EntryDegrade::TopDimension(_) => Some(EntryDegradeTag::TopDimension),
                        EntryDegrade::NoEntryForm => Some(EntryDegradeTag::NoEntryForm),
                        EntryDegrade::RuntimeEntryFailure => None,
                    };
                    if let Some(rung) = rung {
                        out.collapse_narrative.push(CollapseNarrative::new(
                            SpeechAct::Consented,
                            CollapseKind::EntryDenial { rung },
                        ));
                    }
                    if let EntryDegrade::Unvouched(dim) = reason {
                        out.hints.push(Diag::new(
                            DiagCode::WrappedSiteAdoptionHint(WrappedSiteAdoptionHint {
                                provider: inner_word.to_owned(),
                                dimension: dim.as_token().to_owned(),
                            }),
                            ast.node(cfg.node(node).ast).span,
                        ));
                    }
                    (context, dorc_plan::WrappedProbe::Degrade)
                }
            }
        };
        out.peeled.insert(
            node,
            dorc_analysis::effect::PeeledSite {
                inner_argv: peeled_argv,
                context: fact_context,
            },
        );
        out.wrapped.insert(node, probe);
    }
    out
}

/// Try pure-predicate carry (`27C` §4(a); steering `pure-predicate-carry`) for a wrapped site whose
/// entry DEGRADED: does the inner verdict body's read-set close (B) across a SUBSTRATE boundary
/// whose backing kinds are authored-invariant (A)? Runs [`dorc_oracle::carry::read_set_closed`] over
/// the inner verdict body and [`dorc_oracle::carry::decide_carry`] over the chain's crossed
/// dimensions. `Some(read_kinds)` (the (A) attribution inputs) on carry; `None` when there is no
/// inner verdict body, or (A)/(B)/substrate-scope fails — the site then runs (fail safe: a missed
/// carry loses an elision, never carries a hidden read).
fn try_carry(
    chain: &dorc_oracle::entry::PeeledChain,
    inner_provider: Symbol,
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    invariance: &dorc_oracle::carry::InvarianceIndex,
) -> Option<BTreeSet<String>> {
    let verdict = verdict_sets
        .iter()
        .find_map(|set| set.get(inner_provider))?;
    let closure = dorc_oracle::carry::read_set_closed(verdict);
    match dorc_oracle::carry::decide_carry(&chain.composed.crossed(), &closure, invariance) {
        dorc_oracle::carry::CarryDecision::Carry { read_kinds } => Some(read_kinds),
        dorc_oracle::carry::CarryDecision::NoCarry(_) => None,
    }
}

/// Render the pure-predicate-carry attribution chain (`27C` §9: every cross-context elision renders
/// its four-link chain from day one). Names the crossed substrate axes, each marked backing kind
/// whose owner's `invariant:<axis>` line licensed the crossing (vouch-species — the kind-owner's
/// attributable claim), and the engine read-set-closure proof. Deterministic (sorted axes/kinds).
fn carry_attribution_text(
    crossed: &[dorc_oracle::wrapper::Dimension],
    read_kinds: &BTreeSet<String>,
    loci: &BTreeMap<String, String>,
) -> String {
    let (axes, kinds) = carry_attribution_values(crossed, read_kinds, loci);
    format!(
        "pure-predicate carry across {axes} (unflagged, 27C section 4(a)): {kinds} -- each vouched invariant \
         across {axes} by its kind-owner's `invariant:` line (vouch-species); the verdict body is \
         engine-proved read-set-closed"
    )
}

/// The two VALUES that attribution is made of, split so the diagnostic register can hold the
/// sentence and interpolate them (`282:rul-passthrough-type-gated`). The why-lens line above still
/// composes its own text, because that surface has no registry home yet.
fn carry_attribution_values(
    crossed: &[dorc_oracle::wrapper::Dimension],
    read_kinds: &BTreeSet<String>,
    loci: &BTreeMap<String, String>,
) -> (String, String) {
    let axes = crossed
        .iter()
        .map(|d| d.as_token())
        .collect::<Vec<_>>()
        .join("+");
    // render 3/3: each kind names its owner's `invariant:` line as `file:line` when threaded.
    let kinds = read_kinds
        .iter()
        .map(|k| match loci.get(k) {
            Some(loc) => format!("{k} (invariant: line at {loc})"),
            None => k.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ");
    (axes, kinds)
}

/// The lifted wrapper models, per-provider stripped `__enter` defs, and `tolerates:` vouches — the
/// wrapper-side inputs [`build_wrapped_analysis`] peels book sites against (`27N`).
struct WrapperIndexBundle {
    wrappers: dorc_oracle::entry::WrapperIndex,
    enter_defs: BTreeMap<Symbol, (String, String)>,
    tolerance: BTreeMap<Symbol, dorc_oracle::entry::ToleranceVouch>,
}

/// Build the [`WrapperIndexBundle`] from the loaded oracle sources (`27N`): every peeling `__predict`
/// (with its ρ, `__lend_map`, `__enter`) keyed by book word, the stripped `__enter` funcdefs, and
/// the per-provider `tolerates:` vouches (off the already-lifted verdict bodies, `27C` §2).
fn build_wrapper_index(
    oracle_refs: &[&str],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &mut Interner,
) -> WrapperIndexBundle {
    use dorc_oracle::entry::{
        WrapperIndex, WrapperModel, detect_entry_form, lift_entry_set, lift_tolerance,
    };
    use dorc_oracle::predict::{lift_predicts, map_provider_name};
    use dorc_oracle::wrapper::{derive_lend_map, detect_peel, lift_lend_map_set};

    let mut wrappers: WrapperIndex = WrapperIndex::new();
    let mut enter_defs: BTreeMap<Symbol, (String, String)> = BTreeMap::new();
    let mut tolerance: BTreeMap<Symbol, dorc_oracle::entry::ToleranceVouch> = BTreeMap::new();
    for src in oracle_refs {
        let ps = lift_predicts(interner, src).value;
        let ls = lift_lend_map_set(interner, src).value;
        let es = lift_entry_set(interner, src).value;
        for p in ps.providers() {
            let Some(predict) = ps.get(p) else { continue };
            let Some(peel) = detect_peel(predict) else {
                continue; // not a peeling wrapper
            };
            let word = interner.resolve(p).to_owned();
            let lend_map = ls.get(p).cloned();
            let lend = lend_map
                .as_ref()
                .map_or_else(Default::default, |lm| derive_lend_map(lm).0);
            let enter = es.get(p).and_then(detect_entry_form);
            if let Some(form) = es.get(p) {
                let stripped = dorc_oracle::predict::strip_enter(src, form, interner);
                let fname = format!(
                    "{}__enter",
                    dorc_oracle::to_funcname_segment(&map_provider_name(&word))
                );
                enter_defs.entry(p).or_insert((fname, stripped));
            }
            wrappers.entry(word).or_insert(WrapperModel {
                predict: predict.clone(),
                rho: peel.rho,
                lend,
                lend_map,
                enter,
                provider: p,
            });
        }
    }
    for vs in verdict_sets {
        for p in vs.providers() {
            if let Some(v) = vs.get(p) {
                let (vouch, _) = lift_tolerance(v);
                tolerance.entry(p).or_insert(vouch);
            }
        }
    }
    WrapperIndexBundle {
        wrappers,
        enter_defs,
        tolerance,
    }
}

/// Resolve the inner oracle's check for a wrapped site's entry-composed probe (`27N`): the `__predict`
/// body if the inner is a modeled command, else the auto-cell `__is_converged` verdict body (the
/// markless shape). `None` ⇒ no inner check ⇒ the site can't be probed ⇒ runs. Returns
/// `(mangled funcname, stripped funcdef)` — the funcname matches the strip's mangled name byte-for-byte.
fn resolve_inner_check(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    inner_word: &str,
    inner_provider: Symbol,
    inner_operands: &[Symbol],
    interner: &Interner,
) -> Option<(String, String)> {
    use dorc_oracle::predict::map_provider_name;
    let seg = dorc_oracle::to_funcname_segment(&map_provider_name(inner_word));
    if let Some(shipped) = ship_predict_body(
        oracle_srcs,
        checks,
        interner,
        inner_provider,
        inner_operands,
    ) {
        return Some((format!("{seg}__predict"), shipped.sh));
    }
    // Entry-composition is out of both the tier-3 drain scope and the span-threading scope this
    // round: the composed body has no single defining funcdef to name, so its site stays span-less.
    let shipped = ship_verdict_body(oracle_srcs, verdict_sets, interner, inner_provider)?;
    Some((format!("{seg}__is_converged"), shipped.sh))
}

/// Every diagnostic the WRAPPED-SITE and SURVIVAL lanes raise for one world — the harness half of
/// this module's seam, and the sibling of [`crate::world::WhyWorld::analyze`] on the diagnostic
/// plane.
///
/// It runs the binary's own call sequence, in the binary's own order (`cli/src/main.rs`'s `run`):
/// lift the oracles, parse/CFG/value-flow, classify with the wrapped peel threaded in, then — under
/// `consented` only — lift the authored `disturbs()` sets, build the footprints, compile the
/// derivation probes, and merge the host-derived footprints back. That order is what makes a
/// defining case for one of its four codes honest rather than decorative
/// (`289:rul-worldless-route-honest-trigger`).
///
/// `consented` is `--risk-faultless-skips`. With it off the survival half is not merely quiet but
/// ABSENT: no touches set is lifted, no derivation compiles, and the returned diagnostics are the
/// wrapped lane's alone (`empty-world-byte-identical` — the flag-off world is the one an ordinary
/// run sees).
///
/// `results` is the run's ADMITTED probe records. A world with none still reaches the derivation
/// lane (an escalated site announces itself before any readback exists), but the deriv-family
/// completeness gate can only fire against a FRAMED stream that really carried `deriv` records —
/// which is what the fixture intake exists to supply (`crate::results::admit_fixture_records`).
#[must_use]
pub fn survival_diagnostics(
    book_src: &str,
    oracle_paths: &[String],
    oracle_srcs: &[String],
    consented: bool,
    dial: dorc_core::EscalationDial,
    capability: dorc_core::Capability,
    results: &SiteResults,
) -> Vec<Diag> {
    let mut interner = Interner::default();
    let mut arena = dorc_core::ProvArena::new();
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();

    let idx = dorc_oracle::lift(&mut interner, &oracle_refs).value;
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
        .collect();
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
        .collect();
    let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);

    let parsed = dorc_syntax::parse(book_src);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);

    let wrapped = build_wrapped_analysis(
        oracle_srcs,
        &oracle_refs,
        oracle_paths,
        &checks,
        &verdict_sets,
        &parsed.value,
        &cfg.value,
        &value,
        dial,
        capability,
        &mut interner,
    );
    let mut out = wrapped.hints;

    let mut degrades = BTreeMap::new();
    let mut verdict_lane = BTreeSet::new();
    let (classified, _why, kills, kill_coords, _backings, _narrative, _inval) =
        dorc_analysis::effect::classify_with_why_diags(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &verdicts,
            &wrapped.peeled,
            &dorc_analysis::erase::ErasedSites::none(),
            &mut interner,
            &mut arena,
            &mut degrades,
            &mut verdict_lane,
        );
    let classes = classified.value;
    if !consented {
        return out;
    }

    let touches_paired: Vec<(&str, dorc_oracle::touches::TouchesSet)> = oracle_refs
        .iter()
        .map(|src| {
            (
                *src,
                dorc_oracle::touches::TouchesSet::lift(&mut interner, src).value,
            )
        })
        .collect();
    let derivations = {
        let derive = |p, a: &[Symbol]| ship_touches_body(&touches_paired, &interner, p, a);
        dorc_plan::compile_derivations(&parsed.value, &cfg.value, &value, &classes, &kills, derive)
    };

    let touches = lift_touches_sets(&oracle_refs, &mut interner);
    out.extend(touches.diags);
    let lifted = build_survival_footprints(
        &touches.value,
        &classes,
        &kills,
        &kill_coords,
        &value,
        &cfg.value,
        &parsed.value,
        &mut interner,
    );
    out.extend(lifted.diags);
    let mut footprints = lifted.value;
    let derived_node_spans: BTreeMap<_, _> = derivations
        .derivations
        .iter()
        .map(|d| (d.node, parsed.value.node(cfg.value.node(d.node).ast).span))
        .collect();
    out.extend(merge_derived_footprints(
        &mut footprints,
        &derivations,
        results,
        &classes,
        &kill_coords,
        &derived_node_spans,
        &mut interner,
    ));
    out
}
