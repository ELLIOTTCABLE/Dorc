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
    CarriedAcrossSubstrateAxis, DanglingReference, DerivFamilyIncomplete, Diag, DiagCode,
    FootprintIncoherent, FootprintIncoherentReason, TouchesEscalated, WrappedSiteAdoptionHint,
};
use dorc_aid::{Carrier, CollapseKind, CollapseNarrative, SpeechAct};
use dorc_core::{Interner, Symbol};

use crate::kinds::KindReaches;
use crate::results::{ResolvOutcome, SiteResults};
use crate::why::{oracle_locus, render_coord};
use crate::world::ship_predict_body;

/// Lift each oracle's `touches()` set for the authored survival lane, carrying the lift's
/// diagnostics out as data rather than printing them from inside the lift (`io-at-edges-only`).
///
/// The lift WITHDRAWS `contested` on the way out, like every other lifted set
/// (`cli/CLAUDE.md withdrawal-is-applied-once-never-consulted`): a two-author at-most claim is
/// exactly the naked-trust input the survival tier must not read, and withdrawal removes claims,
/// which is the over-execute direction. Taking the fact by parameter rather than consulting it
/// per-seat is the point — no downstream footprint consumer has to remember to ask.
pub fn lift_touches_sets(
    oracle_refs: &[&str],
    interner: &mut Interner,
    contested: &dorc_core::ContestedFamilies,
) -> Carrier<Vec<dorc_oracle::touches::TouchesSet>> {
    let mut diags = Vec::new();
    let sets = oracle_refs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::touches::TouchesSet::lift(interner, src);
            diags.extend(lifted.diags);
            lifted.value.withdrawing(contested, interner)
        })
        .collect();
    Carrier::new(sets, diags)
}

/// The `(source text, withdrawn `disturbs` set)` pairs the derivation lane ships bodies from —
/// [`lift_touches_sets`]' twin for the seat that needs the source text alongside the set, withdrawn
/// on the same terms and for the same reason.
#[must_use]
pub fn pair_touches_sets<'a>(
    oracle_refs: &[&'a str],
    interner: &mut Interner,
    contested: &dorc_core::ContestedFamilies,
) -> Vec<(&'a str, dorc_oracle::touches::TouchesSet)> {
    oracle_refs
        .iter()
        .map(|src| {
            let set = dorc_oracle::touches::TouchesSet::lift(interner, src).value;
            (*src, set.withdrawing(contested, interner))
        })
        .collect()
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
    reason = "the cli-edge footprint lift threads the whole compiled context (touches-sets/classes/kills/kill-coords/value/cfg/ast/interner) plus the `28K` §2 positional pair; each is a distinct pipeline output, not a bundle-able struct"
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
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
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
            resolve_touches_footprint(*node, value, touches_sets, interner, live)
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
                    reason: FootprintIncoherentReason::OmitsOwnCoordinate,
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
        let defining = arm_span
            .or_else(|| touches_defining_span(provider, touches_sets, interner, *node, live));
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
    Option<(dorc_core::Span, dorc_core::SourceFileId)>,
);

/// Which source index's `<want>__disturbs` answers at this site — the survival lane's use of the one
/// resolution seat, shared by all three of its scans so they cannot disagree about the winner.
///
/// `has` asks only "does file `i` DECLARE the role", never "does its body answer this argv": the
/// second question is the retired decline-fallthrough cascade (`28K` §6). The candidate vector is
/// oracle-only here, and the definition table sites the book one PAST it, so a site whose
/// `__disturbs` a BOOK defines resolves to a definition this vector cannot hold and answers nowhere
/// — no footprint, the site walls (`cli/CLAUDE.md the-book-is-a-definition-source` names the widening
/// as its own dispatch; withholding is the safe half).
fn touches_answering_source(
    count: usize,
    declares: impl Fn(usize) -> Option<dorc_core::Span>,
    want: &str,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<usize> {
    let name = format!("{want}{}", dorc_oracle::touches::DISTURBS_SUFFIX);
    crate::world::shipping_source(count, node, live, &name, declares)
}

/// Where source index `i` of `sets` declares its `disturbs()` for the munged provider `want` — the
/// funcdef span that identifies the definition the row came from.
fn declares_touches(
    sets: &[dorc_oracle::touches::TouchesSet],
    interner: &Interner,
    want: &str,
    i: usize,
) -> Option<dorc_core::Span> {
    use dorc_oracle::predict::map_provider_name;
    let set = sets.get(i)?;
    let provider = set
        .providers()
        .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
    set.get(provider).map(|p| p.span)
}

/// Resolve one wall-candidate site's authored `disturbs()` footprint (see the type doc above).
///
/// Which file's `__disturbs` answers is the frame's question, asked through the one resolution seat
/// (`28Q` §1.3; [`crate::world::shipping_source`]). The scan this replaces took the FIRST file that
/// declared the provider and, worse, the first whose body RESOLVED — so a declining live body fell
/// through into a shadowed one's arms (`28K` §6 `rej-decline-fallthrough-cascade`). Both were
/// wrong-elision routes rather than precision losses: a footprint answered by the wrong body can
/// NARROW an at-most claim, and a narrower claim SPARES MORE
/// (`307c:fnd-survival-footprint-lane-scans-forward`).
///
/// **Winner-shifting** (`28Q` §1, permanent): with no agreement veto behind it, a frame-solver
/// precision bug here selects whose judgment governs the site's footprint. License-review-tier.
pub fn resolve_touches_footprint(
    node: dorc_analysis::cfg::CfgNodeId,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
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
    let index = touches_answering_source(
        touches_sets.len(),
        |i| declares_touches(touches_sets, interner, &want, i),
        &want,
        node,
        live,
    )?;
    let set = touches_sets.get(index)?;
    let touches = set
        .providers()
        .find(|p| map_provider_name(interner.resolve(*p)) == want)
        .and_then(|p| set.get(p))?;
    let (coords, arm) = match evaluate_touches_located(touches, &arg_refs) {
        (TouchesResolution::Emitted(coords), arm) if !coords.is_empty() => (
            coords,
            arm.map(|span| (span, dorc_analysis::funcenv::source_file_of_index(index))),
        ),
        // Emitted(empty) = no claim = wall; Top = ⊤ = wall. A DECLINE by the resolved definition
        // is a decline: no neighbour is consulted (`28K` §6).
        (TouchesResolution::Emitted(_) | TouchesResolution::Top(_), _) => return None,
    };

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

/// The `disturbs` funcdef's defining `(Span, SourceFileId)` for a provider (`tc-disturbs-span-
/// threading`; `27V:mech-minting-line-threading`) — a NAME-keyed lookup (no argv trace): the touches
/// funcdef's `name_span` is the leverage point a survival's `claimed` link points at ("the line to
/// widen"). The funcdef `name_span` is the honest coarsest-true span; per-arm precision is deferred.
/// `None` when no definition of the provider's touches funcdef answers at this site.
///
/// It points the author at the body that ACTUALLY spoke, which is why it resolves by frame like its
/// two siblings: a first-file-wins span would caret a definition the shell would never call, and a
/// leverage link is only worth having if it names the line whose widening changes the answer
/// (`271:rul-sin-ordering` — mis-attribution outranks silence).
fn touches_defining_span(
    provider: Symbol,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &Interner,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<(dorc_core::Span, dorc_core::SourceFileId)> {
    use dorc_oracle::predict::map_provider_name;
    let want = map_provider_name(interner.resolve(provider));
    let idx = touches_answering_source(
        touches_sets.len(),
        |i| declares_touches(touches_sets, interner, &want, i),
        &want,
        node,
        live,
    )?;
    let set = touches_sets.get(idx)?;
    set.providers()
        .find(|p| map_provider_name(interner.resolve(*p)) == want)
        .and_then(|p| set.get(p))
        .map(|t| {
            (
                t.name_span,
                dorc_analysis::funcenv::source_file_of_index(idx),
            )
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
///
/// The body that ships is the one the site's FRAME names (`28Q` §1.3), for the reason the ship lane
/// already carries (`crate::world::shipping_source`): shipping a shadowed body would measure the
/// world through an author the shell would never have called.
pub fn ship_touches_body(
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    helpers: &dorc_oracle::closure::HelperIndex,
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<dorc_plan::DerivationShip> {
    use dorc_oracle::predict::{map_provider_name, strip_touches};
    use dorc_oracle::touches::{TouchesResolution, TouchesTop, evaluate_touches};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    let declares = |i: usize| {
        let (_, set) = touches_paired.get(i)?;
        let p = set
            .providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
        set.get(p).map(|t| t.span)
    };
    let idx = touches_answering_source(touches_paired.len(), declares, &want, node, live)?;
    let (src, set) = touches_paired.get(idx)?;
    let p = set
        .providers()
        .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
    let touches = set.get(p)?;
    match evaluate_touches(touches, &arg_refs) {
        // The EXPECTED escalation (24E §4): the body reached a host query ⇒ ship it.
        TouchesResolution::Top(TouchesTop::NonPrintfCommand) => {
            let body = strip_touches(src, touches, interner);
            // The at-most survey ships its snapshot too (`FORFEITS:forfeit-survival-lanes-closure-less`,
            // captured). It matters more here than anywhere: an unbound helper kills the emitting body
            // mid-survey, and a NARROW at-most claim SPARES MORE — the measured wrong-elision
            // `an-at-most-claim-has-two-atomicities` exists for. A denial ships no derivation, so the
            // site walls total.
            let closure = helpers.closure_for(idx, &body).ok()?;
            Some(dorc_plan::DerivationShip {
                // Display the BOOK command word (`apt-get`), not the munged funcdef segment
                // (`apt_get`, the forward-munge key) — the why-lens reads better with the word
                // the admin wrote (`24C:rul24-totalistic-munge` keeps the segment internal).
                call: format!("{}.touches()", interner.resolve(provider)),
                sh: format!("{}{body}", closure.sh()),
            })
        }
        // Static-resolvable, an OTHER ⊤ (degrade-to-wall), or empty ⇒ NOT a derivation.
        TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
    }
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
            Some(close) if close.count as usize == coord_strs.len() => {}
            reason => {
                diags.push(Diag::new(
                    DiagCode::DerivFamilyIncomplete(DerivFamilyIncomplete {
                        site: d.site.0,
                        reason: match reason {
                            Some(close) => {
                                format!("declared n={}, received {}", close.count, coord_strs.len())
                            }
                            None => "no deriv-end close-record".to_string(),
                        },
                    }),
                    span,
                ));
                continue;
            }
        }
        // `28P:dec-whole-body-atomic-refusal` — the SECOND atomicity, invisible to the count above:
        // `n` counts lines RECEIVED, so a body that died mid-survey closes self-consistently while
        // its at-most claim is wrongly NARROW, and narrow spares MORE. Still open by design: a body
        // that truncates and exits 0 (`ANALYZER-NEEDS:an-atmost-completion-signal`).
        if results.framed
            && let Some(close) = results.derivation_ends.get(&d.site)
            && close.body_rc != 0
        {
            diags.push(Diag::new(
                DiagCode::FootprintIncoherent(FootprintIncoherent {
                    reason: FootprintIncoherentReason::EmittingBodyDiedMidSurvey {
                        body_rc: close.body_rc,
                    },
                }),
                span,
            ));
            continue;
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
                    reason: FootprintIncoherentReason::MalformedDerivedCoordinate,
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
    helpers: &dorc_oracle::closure::HelperIndex,
    checks: &[dorc_oracle::predict::PredictSet],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    wrapper_sets: &WrapperSets,
    ast: &dorc_syntax::ast::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    dial: dorc_core::EscalationDial,
    capability: dorc_core::Capability,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> WrappedAnalysis {
    use dorc_aid::narrative::EntryDegradeTag;
    use dorc_analysis::cfg::{CfgNodeId, CfgNodeKind};
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::entry::{EntryDecision, EntryDegrade, decide_entry, peel_book_chain};
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::verdict::VERDICT_SUFFIX;

    let candidates = wrapper_candidates(checks, interner);

    let mut out = WrappedAnalysis {
        peeled: BTreeMap::new(),
        wrapped: dorc_plan::WrappedProbes::new(),
        hints: Vec::new(),
        carried: BTreeMap::new(),
        collapse_narrative: Vec::new(),
    };
    if candidates.is_empty() {
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
        // A head no file describes as a wrapper is an ordinary site: skip before resolving anything,
        // so the per-site resolution (and its narrative) is scoped to sites a wrapper really heads.
        if !argv_refs
            .first()
            .is_some_and(|w| candidates.contains_key(*w))
        {
            continue;
        }
        // Resolved AT this site, over the driver's already-withdrawn vectors: peel model, lend map,
        // and entry-form bytes all answer from the definition the frame names here
        // (`308:rul-wrapper-lane-joins-the-conversion`).
        let (wrappers, enter_defs, pair_narrative) = site_wrapper_index(
            node,
            live,
            oracle_srcs,
            helpers,
            checks,
            wrapper_sets,
            &candidates,
            interner,
        );
        out.collapse_narrative.extend(pair_narrative);
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
        // ONE resolution of the inner verdict, consumed by every act here that reads a verdict body:
        // the SHIPPED inner check (`28Q` §4 verdict primacy), the `safe-across` consent vouch, and
        // pure-predicate carry's read-set-closure proof (`308:rul-carry-proof-is-same-definition`).
        // `try_carry` takes the body by reference and cannot reach a second definition — the proof, the
        // consent, and the measured body are one definition by construction, not by a checked
        // coincidence.
        let inner_verdict =
            crate::world::verdict_answering_at(verdict_sets, interner, inner_provider, node, live);
        let inner_operand_texts: Vec<&str> = inner_rest.iter().map(String::as_str).collect();
        // The inner check body: a VOUCHING verdict measures, else the predict model, else the
        // markless verdict body — the entry-composed mirror of the ambient ship seam.
        let Some((inner_fn, inner_sh)) = resolve_inner_check(
            oracle_srcs,
            helpers,
            checks,
            inner_verdict.as_ref(),
            inner_word,
            inner_provider,
            &inner_operands,
            &inner_operand_texts,
            interner,
            node,
            live,
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
        let inner_verdict = inner_verdict.map(|(_, v)| v);
        // An identity chain (HostDefault) needs NO entry — it ships the plain inner check in the
        // ambient world. A shifted chain runs the two-axis consent decision (`27C` §1).
        let decision = if context == dorc_core::Context::HostDefault {
            EntryDecision::Enter
        } else {
            let has_entry_form = chain.links.iter().all(|l| l.entry.is_some());
            let tolerated = entry_tolerance(
                &composed.inner_fn,
                inner_verdict.as_ref(),
                inner_rest.first().map(String::as_str),
            );
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
                let carried = if composed.inner_fn.ends_with(VERDICT_SUFFIX) {
                    inner_verdict
                        .as_ref()
                        .and_then(|v| try_carry(&chain, v, &invariance))
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
/// dimensions. `Some(read_kinds)` (the (A) attribution inputs) on carry; `None` when
/// (A)/(B)/substrate-scope fails — the site then runs (fail safe: a missed carry loses an elision,
/// never carries a hidden read).
///
/// `verdict` is the body the CALLER already resolved for this site and is about to ship
/// (`308:rul-carry-proof-is-same-definition`). It arrives by reference and the verdict VECTOR is not
/// a parameter, deliberately: this seat used to scan `verdict_sets` for the first matching provider
/// in load order, so an earlier author's read-set-closed body could license ambient measurement of a
/// later, frame-live body carrying an unmarked context-sensitive read — a positive ambient answer
/// then eliding a wrapped mutation, the one cardinal sin, on the unflagged carry path. With no vector
/// in scope the wrong definition is unreachable rather than merely unchosen.
fn try_carry(
    chain: &dorc_oracle::entry::PeeledChain,
    verdict: &dorc_oracle::predict::Predict,
    invariance: &dorc_oracle::carry::InvarianceIndex,
) -> Option<BTreeSet<String>> {
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

/// The wrapper lane's per-FILE lifted vectors for the two members `checks` does not carry:
/// `__lend_map` and `__enter` (`308:rul-wrapper-lane-joins-the-conversion`).
///
/// Minted at the DRIVER EDGE beside `checks`/`verdict_sets`, and withdrawn there in the same call —
/// so a contested wrapper family is gone from every wrapper-lane vector before any site is
/// considered, exactly as `withdrawal-is-applied-once-never-consulted` requires. The constructor
/// takes the contested fact rather than exposing an un-withdrawn value, because a lane that CAN hold
/// the raw lift is a lane that will: `build_wrapper_index` used to re-lift raw source here, and that
/// left a withheld family still peeling, still supplied with entry-form bytes, and still able to
/// reach `EntryDecision::Enter`.
#[derive(Debug)]
pub struct WrapperSets {
    lend_maps: Vec<dorc_oracle::predict::PredictSet>,
    entries: Vec<dorc_oracle::predict::PredictSet>,
}

impl WrapperSets {
    /// Lift and withdraw the two wrapper members over the SOURCE-wide vector, in load order
    /// (`the-book-is-a-definition-source`: a book's `sudo__enter` is an ordinary definition).
    #[must_use]
    pub fn lift(
        source_refs: &[&str],
        interner: &mut Interner,
        contested: &dorc_core::ContestedFamilies,
    ) -> Self {
        let mut lend_maps = Vec::with_capacity(source_refs.len());
        let mut entries = Vec::with_capacity(source_refs.len());
        for src in source_refs {
            let lend = dorc_oracle::wrapper::lift_lend_map_set(interner, src).value;
            let enter = dorc_oracle::entry::lift_entry_set(interner, src).value;
            lend_maps.push(lend.withdrawing(contested, interner));
            entries.push(enter.withdrawing(contested, interner));
        }
        Self { lend_maps, entries }
    }

    /// The per-file `__enter` sets, for the one WHOLE-UNIT reader that legitimately has no frame to
    /// ask from (`308:rul-escalation-policy-consumes-withdrawn-stays-whole-unit`).
    ///
    /// "Which entry-capable wrappers are LOADED" is a question about the load set, not about any
    /// site, so frame-converting it would invent a site that does not exist. It stays whole-unit and
    /// stays a POLICY disclosure — aid-plane, licenses nothing (`two-plane-aid-law`). What it does
    /// owe the run is honest INPUTS, which is why the accessor hands out the WITHDRAWN vector: a
    /// contested wrapper family peels nothing and enters nothing, so it must not narrate as
    /// entry-capable either.
    #[must_use]
    pub fn entries(&self) -> &[dorc_oracle::predict::PredictSet] {
        &self.entries
    }
}

/// Every book word some loaded file describes as a PEELING wrapper, mapped to its provider symbol.
///
/// A whole-unit pre-filter, and only that: the resolved definition decides whether the word is a
/// wrapper AT a site ([`site_wrapper_index`]), and a definition that peels is necessarily in some
/// file's set, so this set can never hide an answer. Empty ⇒ no wrapper is described anywhere ⇒ the
/// lane returns untouched (`empty-world-byte-identical`).
fn wrapper_candidates(
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
) -> BTreeMap<String, Symbol> {
    let mut out = BTreeMap::new();
    for set in checks {
        for p in set.providers() {
            if set
                .get(p)
                .is_some_and(|c| dorc_oracle::wrapper::detect_peel(c).is_some())
            {
                out.insert(interner.resolve(p).to_owned(), p);
            }
        }
    }
    out
}

/// The wrapper models live AT one site, and the entry-form bytes that go with them.
///
/// Each of the three wrapper members resolves its OWN frame answer, because sh binds names
/// independently: the peel model, the lend map, and the entry form are three questions asked at one
/// site, not one file's package deal. `detect_peel` then runs on the RESOLVED predict, so a
/// frame-live body that declines makes the word no wrapper here — the regional decline an engineer
/// spelled inside a subshell is honoured instead of bypassed.
///
/// `308:rul-resolved-pair-coherence-walls`: independent resolution can pair one file's predict with
/// another file's `lend_map` or entry form — a composition no author wrote and no per-file check ever
/// saw, and one that can understate the crossed dimensions and so under-consent entry. Where a
/// CROSS-FILE resolved pair disagrees, the word walls rather than peels. Same-file pairs are left to
/// `oracle::validate`'s whole-unit fail-fast, which has already refused the whole run.
#[expect(
    clippy::too_many_arguments,
    reason = "one site's three wrapper members resolve independently (`308` §1), so the frame, the \
              source set, its snapshot index, and the three lifted vectors all reach one seat"
)]
fn site_wrapper_index(
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
    srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    checks: &[dorc_oracle::predict::PredictSet],
    sets: &WrapperSets,
    candidates: &BTreeMap<String, Symbol>,
    interner: &Interner,
) -> (
    dorc_oracle::entry::WrapperIndex,
    BTreeMap<Symbol, (String, String)>,
    Vec<CollapseNarrative>,
) {
    use dorc_oracle::entry::{ENTER_SUFFIX, WrapperIndex, WrapperModel, detect_entry_form};
    use dorc_oracle::predict::PREDICT_SUFFIX;
    use dorc_oracle::wrapper::{LEND_MAP_SUFFIX, derive_lend_map, detect_peel};

    let mut wrappers: WrapperIndex = WrapperIndex::new();
    let mut enter_defs: BTreeMap<Symbol, (String, String)> = BTreeMap::new();
    let mut narrative = Vec::new();
    for (word, provider) in candidates {
        let Some((predict_file, predict)) = crate::world::member_answering_at(
            checks,
            interner,
            *provider,
            PREDICT_SUFFIX,
            node,
            live,
        ) else {
            continue; // no definition answers here (withheld, contested, or defined below)
        };
        let Some(peel) = detect_peel(&predict) else {
            continue; // the definition live HERE does not peel ⇒ not a wrapper at this site
        };
        let lend_answer = crate::world::member_answering_at(
            &sets.lend_maps,
            interner,
            *provider,
            LEND_MAP_SUFFIX,
            node,
            live,
        );
        let enter_answer = crate::world::member_answering_at(
            &sets.entries,
            interner,
            *provider,
            ENTER_SUFFIX,
            node,
            live,
        );
        if let Some(tag) = resolved_pair_incoherence(
            predict_file,
            &predict,
            lend_answer.as_ref(),
            enter_answer.as_ref(),
        ) {
            narrative.push(CollapseNarrative::new(
                SpeechAct::Declined,
                CollapseKind::WrapperPairIncoherent { class: tag },
            ));
            continue;
        }
        let lend_map = lend_answer.map(|(_, lm)| lm);
        let lend = lend_map
            .as_ref()
            .map_or_else(Default::default, |lm| derive_lend_map(lm).0);
        let enter = enter_answer
            .as_ref()
            .and_then(|(_, e)| detect_entry_form(e));
        if let Some((file, form)) = &enter_answer
            && let Some(src) = srcs.get(*file)
        {
            let stripped = dorc_oracle::predict::strip_enter(src, form, interner);
            let fname = format!(
                "{}{ENTER_SUFFIX}",
                dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(word))
            );
            // The entry FORM ships its snapshot as well — the seat
            // `FORFEITS:forfeit-survival-lanes-closure-less` does not name, found while capturing the
            // three it does. An entry form whose helper never shipped 127s, which refuses entry and
            // lands can't-say ⇒ guard/run (`27C`: every entry failure lands there), so the loss is
            // value rather than safety. A denial supplies no entry form and the wrapper enters nothing.
            if let Ok(closure) = helpers.closure_for(*file, &stripped) {
                enter_defs.insert(*provider, (fname, format!("{}{stripped}", closure.sh())));
            }
        }
        wrappers.insert(
            word.clone(),
            WrapperModel {
                predict,
                rho: peel.rho,
                lend,
                lend_map,
                enter,
                provider: *provider,
            },
        );
    }
    (wrappers, enter_defs, narrative)
}

/// Whether a site's RESOLVED wrapper members contradict ACROSS FILES
/// (`308:rul-resolved-pair-coherence-walls`), and which pair did.
///
/// Two checks, both `oracle`'s existing ones re-asked of the resolved triple rather than of one
/// file's own declarations: the dual-peel tail position (`273` §5 — the guest must start at the same
/// token whichever member dispatches, or the lend map argparses the wrong argv and can report FULL
/// where the truth is MAPPED, understating `crossed()`), and the fold-entry shift agreement
/// (`27C:rul-fold-entry-coherence-failfast`). A same-file pair is skipped: `oracle::validate` has
/// already failed the whole run fast on it, and re-reporting here would double-narrate.
fn resolved_pair_incoherence(
    predict_file: usize,
    predict: &dorc_oracle::predict::Predict,
    lend: Option<&(usize, dorc_oracle::predict::Predict)>,
    enter: Option<&(usize, dorc_oracle::predict::Predict)>,
) -> Option<dorc_aid::narrative::WrapperPairTag> {
    use dorc_aid::narrative::WrapperPairTag;
    // The same canonical argvs `oracle::validate` probes the per-file pair with: bare guest, one
    // flag, two flags plus an operand.
    const CANON: [&[&str]; 3] = [&["g"], &["-a", "g"], &["-a", "-b", "g", "x"]];
    let (lend_file, lend_map) = lend?;
    if *lend_file != predict_file
        && CANON.iter().any(|argv| {
            dorc_oracle::wrapper::check_peel_coherence(predict, lend_map, argv).is_some()
        })
    {
        return Some(WrapperPairTag::PeelDepth);
    }
    if let Some((enter_file, enter_form)) = enter
        && *enter_file != *lend_file
        && dorc_oracle::entry::check_entry_coherence(enter_form, lend_map).is_some()
    {
        return Some(WrapperPairTag::EntryShifts);
    }
    None
}

/// The entry-consent set a wrapped site's SHIPPED body brings to `decide_entry` (`27C` §2, the
/// author's half of both-sides consent).
///
/// `safe-across` is per-FUNCTION: the author asserts THAT BODY's effects are read-only by design, so
/// the mark licenses a context shift only when that body is the one that EXECUTES. Under `28Q` §4
/// verdict primacy the verdict body ships wherever it vouches, which is where the mark and the
/// executing body coincide. Where the verdict DECLINED and the predict ships instead, an
/// unconditional (top-level) mark would otherwise still be lifted — licensing a shift for a body
/// carrying no consent at all, the OVER-consented direction, which `27C`'s reuse-never-acquire
/// posture refuses. So consent is gated on the shipped body BEING the marked one, the same question
/// the carry path asks of the same pair (the closed body must be the measured body).
fn entry_tolerance(
    inner_fn: &str,
    inner_verdict: Option<&dorc_oracle::predict::Predict>,
    verb: Option<&str>,
) -> BTreeSet<dorc_oracle::wrapper::Dimension> {
    use dorc_oracle::verdict::VERDICT_SUFFIX;
    inner_verdict
        .filter(|_| inner_fn.ends_with(VERDICT_SUFFIX))
        .map(|v| {
            dorc_oracle::entry::lift_tolerance(v)
                .0
                .tolerated_on_path(verb)
        })
        .unwrap_or_default()
}

/// Resolve the inner oracle's check for a wrapped site's entry-composed probe (`27N`). `None` ⇒ no
/// inner check ⇒ the site can't be probed ⇒ runs. Returns `(mangled funcname, stripped funcdef)` —
/// the funcname matches the strip's mangled name byte-for-byte.
///
/// `28Q` §4 `rul-verdict-primacy-at-the-ship-seat`, the wrapped seat: a VOUCHING inner verdict owns
/// the measurement, so it ships ahead of any predict that also answers. Two consequences beyond the
/// ambient seat's. The entry-tolerance mark rides the body that EXECUTES — as-built the `safe-across`
/// consent was lifted from `__is_converged` while the predict could ship, a consent mark on a function
/// that does not run. And `build_wrapped_vouches` mints its guard from THIS body
/// (`composed.inner_fn`/`inner_sh`), so a predict body could reach apply-time guard position, which
/// `plan/CLAUDE.md never-synthesized-never-mutating` refuses: the vouch traced the verdict while the
/// guard ran the model.
///
/// Hence the deliberate asymmetry — when the verdict vouches but its body cannot ship (a contested
/// closure), this returns `None` rather than falling back to the predict. The vouch would still mint
/// for this site, and a guard carrying the model is worse than a site that runs (`inv-kfail`).
#[expect(
    clippy::too_many_arguments,
    reason = "the entry-composed ship is a SITE-keyed act like every other (`28K` §2), so it takes \
              the site node and the positional oracle beside the already-threaded inner argv"
)]
fn resolve_inner_check(
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    checks: &[dorc_oracle::predict::PredictSet],
    inner_verdict: Option<&(usize, dorc_oracle::predict::Predict)>,
    inner_word: &str,
    inner_provider: Symbol,
    inner_operands: &[Symbol],
    inner_operand_texts: &[&str],
    interner: &Interner,
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<(String, String)> {
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::verdict::{VerdictResolution, evaluate_verdict};
    let seg = dorc_oracle::to_funcname_segment(&map_provider_name(inner_word));
    // Entry-composition is out of both the tier-3 drain scope and the span-threading scope this
    // round: the composed body has no single defining funcdef to name, so its site stays span-less.
    let verdict_ship = |file: usize, verdict: &dorc_oracle::predict::Predict| {
        crate::world::ship_resolved_verdict(oracle_srcs, helpers, interner, file, verdict)
            .map(|shipped| (format!("{seg}__is_converged"), shipped.sh))
    };
    if let Some((file, verdict)) = inner_verdict
        && matches!(
            evaluate_verdict(verdict, inner_operand_texts),
            VerdictResolution::Vouched
        )
    {
        return verdict_ship(*file, verdict);
    }
    if let Some(shipped) = ship_predict_body(
        oracle_srcs,
        helpers,
        checks,
        interner,
        inner_provider,
        inner_operands,
        node,
        live,
    ) {
        return Some((format!("{seg}__predict"), shipped.sh));
    }
    let (file, verdict) = inner_verdict?;
    verdict_ship(*file, verdict)
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
#[expect(
    clippy::too_many_lines,
    reason = "the diagnostic route runs the survival pipeline in the binary's own order; splitting it \
              is how the two orders drift, which is the failure this seat exists to prevent"
)]
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

    // The HINT lane solves no environment, so it has no proven contest to withdraw either — the
    // same honest posture its two `unsolved()` calls take (`dis-hint-lane-has-no-contest-to-withdraw`).
    let wrapper_sets = WrapperSets::lift(
        &oracle_refs,
        &mut interner,
        &dorc_core::ContestedFamilies::none(),
    );
    let wrapped = build_wrapped_analysis(
        oracle_srcs,
        &oracle_refs,
        oracle_paths,
        &dorc_oracle::closure::HelperIndex::build(&oracle_refs, None),
        &checks,
        &verdict_sets,
        &wrapper_sets,
        &parsed.value,
        &cfg.value,
        &value,
        dial,
        capability,
        &mut interner,
        dorc_analysis::funcenv::LiveDefinitions::unsolved(),
    );
    let mut out = wrapped.hints;

    let mut degrades = BTreeMap::new();
    let mut verdict_lane = BTreeMap::new();
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
            // The HINT lane licenses nothing, so its trip latch is its own and goes nowhere.
            &mut dorc_analysis::certify::CertifierTrip::default(),
            // The HINT lane reads ambiently: narrating a shape whose license the positional
            // regime withholds is the aid plane failing safe (`two-plane-aid-law`).
            dorc_analysis::funcenv::LiveDefinitions::unsolved(),
        );
    let classes = classified.value;
    if !consented {
        return out;
    }

    // The HINT lane reads ambiently throughout (see the two `unsolved()` calls above): this seat
    // solves no function environment, so its survival scans take the same no-environment posture,
    // and with no environment there is no contest to withdraw either.
    let hint_live = dorc_analysis::funcenv::LiveDefinitions::unsolved();
    let uncontested = dorc_core::ContestedFamilies::none();
    let touches_paired = pair_touches_sets(&oracle_refs, &mut interner, &uncontested);
    let derivations = {
        let hint_helpers = dorc_oracle::closure::HelperIndex::build(&oracle_refs, None);
        let derive = |n, p, a: &[Symbol]| {
            ship_touches_body(
                &touches_paired,
                &hint_helpers,
                &interner,
                p,
                a,
                n,
                hint_live,
            )
        };
        dorc_plan::compile_derivations(&parsed.value, &cfg.value, &value, &classes, &kills, derive)
    };

    let touches = lift_touches_sets(&oracle_refs, &mut interner, &uncontested);
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
        hint_live,
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

// ---- moved verbatim from `main.rs` (the survival lane's cli-edge half) ----
/// Collect the coordinates that need canonicalization (24F §3): every establish/query BACKING coord
/// and every wall-candidate FOOTPRINT coord whose KIND is resolver-bearing. Deduplicated (resolution
/// is a pure function of `(kind, entity)`) and deterministic (`BTreeSet`). Derived-footprint coords
/// (escalated walls, resolved only post-results) are NOT covered — a resolver+derived combination is
/// a second round-trip, deferred (noted `resid-resolve-derived`).
pub fn collect_resolver_coords(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    resolver_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> BTreeSet<dorc_plan::EntityCoord> {
    use dorc_analysis::effect::SkipClass;
    let mut coords = BTreeSet::new();
    let consider = |coord: dorc_plan::EntityCoord, coords: &mut BTreeSet<_>| {
        if resolver_kinds.contains(&coord.kind().0) {
            coords.insert(coord);
        }
    };
    for (node, class) in classes {
        // Backing coords: the cell each establish/query site is about.
        if let SkipClass::EstablishAmbient(f)
        | SkipClass::EstablishWritten(f)
        | SkipClass::QueryResolvable { fact: f, .. } = class
        {
            consider(dorc_plan::EntityCoord::new(f.kind, f.entity), &mut coords);
        }
        // Footprint coords: a wall-candidate's touches() emissions.
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
        ) || kills.contains(node);
        if is_wall_candidate
            && let Some((_, fp_coords, _)) =
                resolve_touches_footprint(*node, value, touches_sets, interner, live)
        {
            for (c, _selector) in fp_coords {
                consider(c, &mut coords);
            }
        }
    }
    coords
}

/// Build the [`dorc_plan::Resolutions`] map (24F §3) from the resolver-probe readback: mark every
/// resolver-bearing kind, record each `canon`, and flag each `dangling`. A resolver-bearing coord
/// with NO readback record degrades to may-alias at canonicalization (§3a — the safe direction).
/// Interning the canonical form through the SHARED interner keeps it in the one vocabulary (the
/// fence); the engine compares canonical tokens as symbols, never decoding (`inv-referent-agnostic`).
pub fn build_resolutions(
    coords: &BTreeSet<dorc_plan::EntityCoord>,
    resolver_kinds: &BTreeSet<Symbol>,
    readback: &SiteResults,
    interner: &mut Interner,
) -> dorc_plan::Resolutions {
    let mut resolutions = dorc_plan::Resolutions::none();
    for kind in resolver_kinds {
        resolutions.add_resolver_kind(dorc_core::KindId(*kind));
    }
    for coord in coords {
        let label = render_coord(*coord, interner);
        match readback.resolutions.get(&label) {
            Some(ResolvOutcome::Canonical(canon_text)) => {
                let entity = if canon_text.is_empty() {
                    dorc_core::EntityRef::Singleton
                } else {
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(
                        interner.intern(canon_text),
                    ))
                };
                resolutions.record(*coord, entity);
            }
            // Dangling OR no record ⇒ leave unrecorded ⇒ may-alias at canonicalization (§3a). A
            // dangling is additionally flagged for the loud diagnostic (§4).
            Some(ResolvOutcome::Dangling) => resolutions.record_dangling(*coord),
            None => {}
        }
    }
    resolutions
}

/// The DANGLING-reference diagnostics (24F §4): one loud per-coordinate note for each coordinate the
/// resolver flagged dangling (a reference to a non-existent entity on an enumerable kind — the
/// resolver's natural `dpkg-query -W` non-zero). Turns the third-party-typo case from silent
/// value-loss into a pointed hint; the coordinate ALSO rides the may-alias degrade (§3a). ADVISORY —
/// the apply runs the affected site either way (fail toward run), so no correctness rides on this
/// readout; it is the render surface (rec-1). `inv-referent-agnostic`: the coord label is display.
#[must_use]
pub fn dangling_diagnostics(
    resolutions: &dorc_plan::Resolutions,
    interner: &Interner,
) -> Vec<Diag> {
    resolutions
        .dangling()
        .map(|coord| {
            Diag::new_spanless_site(DiagCode::DanglingReference(DanglingReference {
                coord: render_coord(coord, interner),
            }))
        })
        .collect()
}

/// Expand every reach-bearing footprint coordinate via its kind's `reaches()` (24G §4 — the
/// compositional half; the cross-author widening). STATIC arms apply to ALL footprint coords
/// (authored + derived), traced here at the cli (no host); DYNAMIC arms apply to AUTHORED coords only
/// this pass (their entities come from the `reach` readback — derived coords are known only
/// post-results, the `resid-kindfn-derived` deferral, 24G §3). Each expanded coord is unioned into
/// the footprint via [`dorc_plan::Footprint::add_reached`] (attributed to the reach-function KIND),
/// flowing through the EXISTING `disjoint`/canonicalization path. `inv-referent-agnostic`: the engine
/// interns the annotated kind (fixed at LIFT — the vocabulary fence) + the raw entities, never
/// decoding them.
///
/// THE ATOMICITY GATE (`28P:fnd-the-reach-lane-has-no-completeness-gate-at-all`, repaired here). The
/// retired reading was that widening "only ever HITs MORE, the safe direction", so an arm that
/// answered nothing was the honest un-expanded floor. Measured, that is false whenever the `disturbs`
/// claim is not independently total — which is exactly when a kind-owner's `reaches_only` is needed:
/// a missing expansion leaves the at-most footprint wrongly NARROW, and narrow SPARES MORE (a
/// downstream converged site survived a running wall it should have collided with). So a DYNAMIC arm
/// must now CLOSE, with its stream intact and its body finished; anything less refuses the whole
/// footprint and the site walls total. The gate is framed-only, exactly as the deriv lane's is —
/// legacy unframed fixtures carry no close records and are trusted-complete.
///
/// Diagnostics are the product alongside the `&mut` expansion, so a caller that drops them drops the
/// only trace of a refusal.
#[must_use]
pub fn expand_footprints_via_reaches(
    footprints: &mut dorc_plan::TrustedFootprints,
    reaches: &KindReaches,
    reach_kinds: &BTreeSet<Symbol>,
    readback: &SiteResults,
    node_spans: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::Span>,
    interner: &mut Interner,
) -> Vec<Diag> {
    use dorc_oracle::reaches::{ArmOutcome, evaluate_reaches};
    let mut diags = Vec::new();
    footprints.expand_reaches(|node, coord, origin| {
        let kind_sym = coord.kind().0;
        if !reach_kinds.contains(&kind_sym) {
            return dorc_plan::ReachExpansion::Expanded(Vec::new());
        }
        let Some((_, reaches_fn)) = reaches.get(kind_sym) else {
            return dorc_plan::ReachExpansion::Expanded(Vec::new());
        };
        let entity_text = entity_text_of(coord, interner);
        let coord_label = render_coord(coord, interner);
        let via = coord.kind();
        let exp = evaluate_reaches(reaches_fn, &entity_text);
        let mut out = Vec::new();
        for arm in &exp.arms {
            let arm_kind = dorc_core::KindId(interner.intern(&arm.kind));
            let entities: Vec<String> = match &arm.outcome {
                // STATIC arms apply to ALL footprint coords (24G §3) — the traced lines, no host.
                ArmOutcome::Static(lines) => lines.clone(),
                // DYNAMIC arms apply to AUTHORED coords only this pass (24G §3, resid-kindfn-derived).
                ArmOutcome::Dynamic { .. } => {
                    if !matches!(origin, dorc_plan::FootprintOrigin::Authored) {
                        continue;
                    }
                    let key = (coord_label.clone(), arm.index);
                    let received = readback.reaches.get(&key).cloned().unwrap_or_default();
                    if let Some(reason) =
                        reach_arm_refusal(readback, &key, received.len(), arm.index)
                    {
                        if let Some(&span) = node_spans.get(&node) {
                            diags.push(Diag::new(
                                DiagCode::FootprintIncoherent(FootprintIncoherent { reason }),
                                span,
                            ));
                        }
                        return dorc_plan::ReachExpansion::Refused;
                    }
                    received
                }
            };
            for e in entities {
                if e.is_empty() {
                    continue; // a blank reached entity is not a coordinate
                }
                let ec = dorc_plan::EntityCoord::new(
                    arm_kind,
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(&e))),
                );
                out.push((ec, via));
            }
        }
        dorc_plan::ReachExpansion::Expanded(out)
    });
    diags
}

/// Why one dynamic `reaches()` arm's survey may not be trusted, or `None` if it closed cleanly. The
/// two conditions are INDEPENDENT and both necessary, exactly as the deriv lane's are
/// (`28P:dec-whole-body-atomic-refusal`): the count proves the record STREAM arrived whole, the
/// `body-rc` proves the arm BODY finished. An unframed stream carries neither and is
/// trusted-complete (the legacy authored fixtures).
fn reach_arm_refusal(
    readback: &SiteResults,
    key: &(String, usize),
    received: usize,
    arm: usize,
) -> Option<FootprintIncoherentReason> {
    if !readback.framed {
        return None;
    }
    match readback.reach_ends.get(key) {
        None => Some(FootprintIncoherentReason::ReachArmNeverClosed { arm }),
        Some(close) if close.count as usize != received => {
            Some(FootprintIncoherentReason::ReachArmStreamCut {
                arm,
                declared: close.count,
                received: u32::try_from(received).unwrap_or(u32::MAX),
            })
        }
        Some(close) if close.body_rc != 0 => {
            Some(FootprintIncoherentReason::ReachArmDiedMidSurvey {
                arm,
                body_rc: close.body_rc,
            })
        }
        Some(_) => None,
    }
}

/// The entity text of a coordinate for a reach/resolver invocation (an operand's text, or the empty
/// string for a Singleton). `inv-referent-agnostic`: resolved for the invocation, never decoded.
#[must_use]
pub fn entity_text_of(coord: dorc_plan::EntityCoord, interner: &Interner) -> String {
    match coord.entity() {
        dorc_core::EntityRef::Operand(tok) => interner.resolve(tok.0).to_owned(),
        dorc_core::EntityRef::Singleton => String::new(),
    }
}

/// Collect the RAW coordinate kinds present in this analysis — every establish/query BACKING kind
/// plus every wall-candidate FOOTPRINT kind. Used to re-key the munged kind-keyed resolver/reaches
/// maps to the raw kinds coordinates carry (`flag-forward-munge-keying`; `kinds::rekey_to_raw_kinds`).
///
/// rider-resolver-coverage-watch (`277` §7b): this collected set is EXACTLY the population the
/// survival comparison ([`dorc_plan::survival::disjoint`]) ever canonicalizes — backings come from
/// converged-`Replace` licenses (establish/query classes) and footprints from wall candidates, so
/// every coordinate that reaches a resolver lookup has its kind collected here. The coverage is
/// therefore sound (no silent under-cover); it stays collection-based rather than structural because
/// the resolver-SHIPPING pipeline is a cli-edge concern the comparison-layer re-key does not subsume.
#[must_use]
pub fn collect_coord_kinds(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> BTreeSet<Symbol> {
    use dorc_analysis::effect::SkipClass;
    let mut kinds = BTreeSet::new();
    for (node, class) in classes {
        if let SkipClass::EstablishAmbient(f)
        | SkipClass::EstablishWritten(f)
        | SkipClass::QueryResolvable { fact: f, .. } = class
        {
            kinds.insert(f.kind.0);
        }
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
        ) || kills.contains(node);
        if is_wall_candidate
            && let Some((_, fp_coords, _)) =
                resolve_touches_footprint(*node, value, touches_sets, interner, live)
        {
            for (c, _selector) in fp_coords {
                kinds.insert(c.kind().0);
            }
        }
    }
    kinds
}

#[cfg(test)]
mod tests {
    use dorc_core::Interner;

    /// One `disturbs` body per file, each naming a DIFFERENT kind, so the resolved footprint's kind
    /// says which definition spoke.
    fn oracle(kind: &str) -> String {
        format!(
            "# dorc-lang/v0.2\n\
             apt_get__disturbs() {{\n\
             \x20  case ${{1-}} in install) printf '%s\n' \"$2\" : disturbs {kind} ;; esac\n\
             }}\n"
        )
    }

    /// The kind of the footprint `apt-get install nginx` resolves, with the two `disturbs`
    /// definitions loaded in the given order.
    fn resolved_kind(srcs: [&str; 2]) -> String {
        let mut interner = Interner::default();
        let book = dorc_syntax::parse("apt-get install nginx\n").value;
        let cfg = dorc_analysis::cfg::build(&book).value;
        let value = dorc_analysis::value::analyze(&cfg, &book, &mut interner);
        let paths = vec!["a.oracle.sh".to_owned(), "b.oracle.sh".to_owned()];
        let snapshot = crate::snapshot::StaticLoadSnapshot::over(
            dorc_core::loadpath::Cwd::default(),
            paths,
            srcs.iter().map(|s| (*s).to_owned()).collect(),
            [].into(),
            "book.sh",
            "apt-get install nginx\n",
        );
        let defs = crate::world::definition_table(&snapshot, &book);
        let env = {
            let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
            dorc_analysis::funcenv::analyze(&book, &cfg, &defs, &plane)
        };
        let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &defs);
        let sets: Vec<dorc_oracle::touches::TouchesSet> = srcs
            .iter()
            .map(|src| dorc_oracle::touches::TouchesSet::lift(&mut interner, src).value)
            .collect();
        let node = cfg
            .iter()
            .find(|(_, n)| n.kind == dorc_analysis::cfg::CfgNodeKind::Command)
            .map(|(id, _)| id)
            .expect("the one-command book lowers one Command node");
        let (_, coords, _) =
            super::resolve_touches_footprint(node, &value, &sets, &mut interner, live)
                .expect("the live disturbs body emits one coordinate");
        let (coord, _) = coords.first().expect("one emitted coordinate");
        interner.resolve(coord.kind().0).to_owned()
    }

    /// The footprint answers from the definition the FRAME names, not the first file that happens to
    /// declare one (`307c:fnd-survival-footprint-lane-scans-forward`).
    ///
    /// Asserted in BOTH load orders, which is what makes it a statement about resolution rather than
    /// about a particular expedient: the retired scan answers `first` under both, so either order
    /// alone could be passed by an accident of fixture layout.
    ///
    /// It is a live wrong-elision route and not a precision loss: a footprint is an AT-MOST claim, a
    /// wrong body's emission can NARROW it, and a narrower claim SPARES MORE — the under-execute
    /// direction (`inv-kfail`).
    ///
    /// This pins the SEAT. Whether a two-file world of this shape survives the cli's contested
    /// withdrawal is that edge's separate question (`cli/CLAUDE.md
    /// withdrawal-is-applied-once-never-consulted`).
    /// A CROSS-FILE resolved pair whose `"$@"` reach different tail positions walls the word, and a
    /// SAME-FILE one does not (`308:rul-resolved-pair-coherence-walls`).
    ///
    /// Independent per-member frame resolution is right — sh binds names independently — but it can
    /// pair one file's peel model with another file's lend map, a composition no author wrote. The
    /// lend map then argparses the wrong argv and can report FULL where the truth is MAPPED, which
    /// UNDERSTATES `crossed()` and so under-consents entry: the dangerous direction.
    ///
    /// The same-file half is the load-bearing other end. That pair is `oracle::validate`'s
    /// whole-unit fail-fast, which has already refused the entire run
    /// (`RunOutcome::WrapperIncoherent`), so walling here too would double-narrate a refusal that
    /// already happened — and, worse, would make this seat look like the thing enforcing a law it
    /// only backstops.
    #[test]
    fn a_cross_file_resolved_pair_that_disagrees_walls_and_a_same_file_one_does_not() {
        use dorc_aid::narrative::WrapperPairTag;

        let mut interner = Interner::default();
        // Two peel depths over the same argv: the predict consumes its leading flags before the
        // guest, the lend map hands its argument slot straight through.
        let deep = "# dorc-lang/v0.2\n\
             w__predict() {\n\
             \x20  while [ \"${1#-}\" != \"$1\" ]; do shift; done\n\
             \x20  \"$@\"\n\
             }\n";
        let shallow = "# dorc-lang/v0.2\n\
             w__lend_map() {\n\
             \x20  : lends user\n\
             \x20  : lends fs-view\n\
             \x20  : lends netns\n\
             \x20  \"$@\"\n\
             }\n";
        let provider = interner.intern("w");
        let predicts = dorc_oracle::predict::lift_predicts(&mut interner, deep).value;
        let lends = dorc_oracle::wrapper::lift_lend_map_set(&mut interner, shallow).value;
        let predict = predicts
            .get(provider)
            .expect("the fixture declares a peeling predict")
            .clone();
        let lend = lends
            .get(provider)
            .expect("the fixture declares a lend map")
            .clone();

        assert_eq!(
            super::resolved_pair_incoherence(0, &predict, Some(&(1, lend.clone())), None),
            Some(WrapperPairTag::PeelDepth),
            "resolved from two different files and disagreeing on the guest's start, the word must \
             not be treated as a wrapper here"
        );
        assert_eq!(
            super::resolved_pair_incoherence(0, &predict, Some(&(0, lend)), None),
            None,
            "the SAME file's own contradiction is `oracle::validate`'s pre-network fail-fast; this \
             seat must not re-report it"
        );
    }

    /// With no lend map resolved at all there is no PAIR to contradict — the enumerate-every-dimension
    /// wall (`271:rul-lend-map`) is what handles that case, at `decide_entry`, and it is a different
    /// answer from "these two authors disagree".
    #[test]
    fn a_resolved_predict_with_no_lend_map_is_not_a_pair_disagreement() {
        let mut interner = Interner::default();
        let src = "# dorc-lang/v0.2\n\
             w__predict() {\n\
             \x20  shift\n\
             \x20  \"$@\"\n\
             }\n";
        let provider = interner.intern("w");
        let predicts = dorc_oracle::predict::lift_predicts(&mut interner, src).value;
        let predict = predicts
            .get(provider)
            .expect("the fixture declares a peeling predict")
            .clone();
        assert_eq!(
            super::resolved_pair_incoherence(0, &predict, None, None),
            None
        );
    }

    #[test]
    fn the_footprint_answers_from_the_definition_the_frame_names() {
        let first = oracle("first.dorc.Package");
        let second = oracle("second.dorc.Package");
        assert_eq!(
            resolved_kind([first.as_str(), second.as_str()]),
            "second.dorc.Package"
        );
        assert_eq!(
            resolved_kind([second.as_str(), first.as_str()]),
            "first.dorc.Package"
        );
    }

    /// One file where `hork` authors BOTH members, so the wrapped ship seat has a real choice: the
    /// predict models `tune` and `poke`, the verdict vouches `tune` alone.
    const BOTH_MEMBERS: &str = "# dorc-lang/v0.2\n\
         hork__predict() {\n\
         \x20  verb=$1; shift\n\
         \x20  thing : sm.dorc.Hork = \"$1\"\n\
         \x20  case $verb in\n\
         \x20  tune) hork model -- \"$thing\" : sm.dorc.Hork:\"$thing\"@tuned ;;\n\
         \x20  poke) hork model -- \"$thing\" : sm.dorc.Hork:\"$thing\"@poked ;;\n\
         \x20  esac\n\
         }\n\
         hork__is_converged() {\n\
         \x20  verb=$1; shift\n\
         \x20  case $verb in\n\
         \x20  tune) hork query -- \"$1\" ;;\n\
         \x20  *) return 2 ;;\n\
         \x20  esac\n\
         }\n";

    /// Which body the wrapped seat ships for the peeled inner argv `hork <verb> wombat`, as the
    /// mangled funcname the entry composition invokes.
    fn inner_check_fn(verb: &str) -> String {
        let mut interner = Interner::default();
        let srcs = vec![BOTH_MEMBERS.to_owned()];
        let refs: Vec<&str> = srcs.iter().map(String::as_str).collect();
        let helpers = dorc_oracle::closure::HelperIndex::build(&refs, None);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut interner, BOTH_MEMBERS).value];
        let verdict_sets =
            vec![dorc_oracle::verdict::VerdictSet::lift(&mut interner, BOTH_MEMBERS).value];
        let provider = interner.intern("hork");
        // Anti-masking: without both members lifted the seat has no choice to make, and every
        // assertion below would pass vacuously on whichever one survived.
        assert!(
            checks[0].get(provider).is_some(),
            "the fixture must lift a predict"
        );
        assert!(
            verdict_sets[0].get(provider).is_some(),
            "the fixture must lift a verdict"
        );
        let operands = [interner.intern(verb), interner.intern("wombat")];
        let texts = [verb, "wombat"];
        // The hint-lane posture: no environment solved, so a SOLE declaration answers
        // (`LiveDefinitions::unsolved`). The seat under test is the ship CHOICE, not the resolution.
        let live = dorc_analysis::funcenv::LiveDefinitions::unsolved();
        let node = dorc_analysis::cfg::CfgNodeId(0);
        let inner_verdict =
            crate::world::verdict_answering_at(&verdict_sets, &interner, provider, node, live);
        let (fn_name, _) = super::resolve_inner_check(
            &srcs,
            &helpers,
            &checks,
            inner_verdict.as_ref(),
            "hork",
            provider,
            &operands,
            &texts,
            &interner,
            node,
            live,
        )
        .expect("both members answer this fixture");
        fn_name
    }

    /// `27C` §2 both-sides consent is per-FUNCTION, so a `safe-across` mark licenses a context shift
    /// only for the body that EXECUTES.
    ///
    /// The mark here is UNCONDITIONAL (top level, not inside a `case` arm), which is exactly the
    /// dangerous shape: `tolerated_on_path` returns it for every verb, declining ones included. So
    /// before `28Q` §4's re-cut a verdict that declined this argv still handed its consent to a
    /// PREDICT body that ships and runs unmarked — an over-consented entry. Both halves are asserted
    /// together because the gate is only correct if it still lifts consent where the marked body IS
    /// the shipped one; a gate that simply never lifted would pass the second assertion alone.
    #[test]
    fn consent_rides_the_body_that_ships_and_no_other() {
        use dorc_oracle::wrapper::Dimension;
        use std::collections::BTreeSet;
        let mut interner = Interner::default();
        let src = "# dorc-lang/v0.2\n\
             hork__is_converged() {\n\
             \x20  : safe-across user\n\
             \x20  case ${1-} in tune) hork query -- \"$2\" ;; *) return 2 ;; esac\n\
             }\n";
        let set = dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value;
        let provider = interner.intern("hork");
        let verdict = set.get(provider).expect("the fixture lifts a verdict");
        let marked: BTreeSet<Dimension> = [Dimension::User].into_iter().collect();

        assert_eq!(
            super::entry_tolerance("hork__is_converged", Some(verdict), Some("tune")),
            marked,
            "the marked body ships here, so its author's consent is exactly what the dial weighs"
        );
        assert!(
            super::entry_tolerance("hork__predict", Some(verdict), Some("poke")).is_empty(),
            "the verdict declined and the predict ships: an unmarked body executes, so this site \
             brings NO author consent and the shift must not be licensed"
        );
    }

    /// `28Q` §4 `rul-verdict-primacy-at-the-ship-seat` at the WRAPPED seat: a vouching inner verdict
    /// ships ahead of a predict that also answers.
    ///
    /// Both halves are load-bearing, and the second is why this is not merely a preference. The
    /// declining verb must keep the predict, because `build_wrapped_vouches` mints NO vouch there —
    /// while for the vouching verb it mints one FROM this very body, so a predict winning `tune`
    /// would put the model in apply-time guard position and lift the `safe-across` consent off a
    /// function that never runs.
    #[test]
    fn a_vouching_inner_verdict_ships_ahead_of_the_predict_at_a_wrapped_site() {
        assert_eq!(
            inner_check_fn("tune"),
            "hork__is_converged",
            "the vouching author's body measures, and is the body the wrapped vouch guards with"
        );
        assert_eq!(
            inner_check_fn("poke"),
            "hork__predict",
            "the verdict declines this verb, so there is no elision for it to license and the \
             model keeps feeding the site's concern topology"
        );
    }
}
