//! Per-command system-state effects + the ambient-invariant gate — the input the
//! skip decision consumes (this module decides *nothing* itself; it classifies).
//!
//! Two steps (note 163 §2):
//! 1. **effect resolution** — for each `Command` node, thread the book's
//!    flow-resolved argv (`analysis::value::ValueFlow`) through the oracle's own
//!    `check()` (`oracle::predict::evaluate`) to its inline kind-annotation, then key
//!    the resulting `(verb, entity, kind)` into the oracle effect-map for the
//!    `(selector, polarity)` cells (`Establishes`/`Kills`, or `Opaque` on any ⊤).
//!    The engine parses NO argv itself — *identity is declared, never inferred*
//!    (`inv-referent-agnostic`); the old engine-side flag-strip stand-in is gone.
//! 2. **ambient gate** — a forward reaching-definitions pass over the mutated
//!    facts: a fact is *ambient* at a command iff NO upstream in-script command
//!    mutated it (so the host's resting state is authoritative and we may probe
//!    it). A fact mutated upstream is *written* — its resting value is stale —
//!    catching `apt-get purge X; … apt-get install X` (note 162 O-1 / break-10).
//!
//! Lock-style note (note 165): this module is deliberately **all forward-may**
//! (over-approximate), so there is no `May`/`Must` wrapper here yet — there is
//! nothing of the opposite orientation to confuse it with. That lock arrives with
//! the first *must* analysis (statically-definitely-established) and the backward
//! apply-slice. Here the only conservative direction is "when unsure ⇒ `Opaque`
//! ⇒ not ambient ⇒ run", which is safe for the skip decision.

use crate::certify::{CertifierTrip, SolveConsistency, solve_certified};
use crate::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use crate::lattice::Lattice;
use crate::solve::{Direction, Graph};
use crate::value::{ValueFlow, ValueOf};
use dorc_aid::Carrier;
use dorc_aid::diag::{
    CmdsubInnerNonleaf, CmdsubOperandTop, CommandName, Diag, DiagCode as Code,
    EffectKindDisagreement, OperandPosition, RedirTargetTop, SiteId, SolvePass,
    SolverConsistencyFailure,
};
use dorc_core::{
    Context, EntityRef, FactBacking, Interner, KindId, LeafId, OpaqueToken, ProviderId, SelectorId,
    Span,
};
use dorc_oracle::predict::{self, PredictSet, ResolvedEntity, TopReason};
use dorc_oracle::verdict::{VERDICT_SUFFIX, VerdictIndex};
use dorc_oracle::{EffectCell, KindIndex, ValueClaim, empty_verb};
use std::collections::{BTreeMap, BTreeSet};

/// The dataflow fact-key the engine reaches over. **Re-exported from `core`**
/// (`dec-seam-ownership`, `notes/193` §2): the structured entity-algebra is the
/// shared vocabulary defined in `core` so `oracle`/`plan`/`hostsim` all key on one
/// type; `analysis` is its largest *consumer*, not a parallel owner. The flat
/// `(kind, entity)` pair of spike-1 became `core::FactKey { kind, entity:
/// EntityRef, selector }` — the per-entity selector is what kills the poison wall
/// (`apt-get update` ⇒ `package-index@fresh`, distinct from `install`'s
/// `package:nginx@installed`).
pub use dorc_core::FactKey;

/// What a command does to — or *observes about* — system state, as far as the
/// analyzer can determine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandEffect {
    /// No modeled system-state effect (a bare assignment, or a read-only command).
    Pure,
    /// Establishes `fact` (`apt-get install nginx`).
    Establishes(FactKey),
    /// Kills `fact` (`apt-get purge nginx`).
    Kills(FactKey),
    /// READS `fact` and mutates nothing — the read-only guard-class (`command -v
    /// nginx` ⇒ `Queries(tool:nginx@present)`; 202 §2 / task-D2). A `Query`
    /// **poisons no reaching-defs and establishes nothing** (the reaching-defs gen
    /// treats it like `Pure`): a guard reads state, it does not change it, so it must
    /// not force a downstream establish to `EstablishWritten` nor invalidate a
    /// downstream Query (st-3, 20A §4). Distinct from `Pure` only so a Query SITE is
    /// probe-resolvable (its check IS the probe) and its probed rc can feed the fold's
    /// Status channel (gated by rule-query-validity).
    Queries(FactKey),
    /// ⊤: cannot characterize — a ⊤ argv word, no provider/check, an `evaluate`
    /// `Top`, or no effect-map entry for the resolved verb. Conservatively MAY mutate
    /// anything (so it poisons downstream ambient-ness) and itself must run
    /// (`inv-top-reject`).
    Opaque,
}

// B4 sweep: EffectKindDisagreement migrated onto Diag spine (payload in dorc_aid::diag).

/// The source identity of a give-up site for the migrated `dq-cmdsub-operand-top` spine
/// (`22B` §5 worked-3): a real source [`Span`] (the drop-A fix — s-2 resolves it) and a stable
/// [`SiteId`] for grouping. The leaf id is the CFG-node index: the kernel runs BEFORE the plan
/// assigns plan-`LeafId`s, and this Note is render-plane-only (it never enters the probe-RESULTS
/// lane `inv-site-keyed-results` governs), so the CFG-node-space id is an honest grouping site
/// here (flagged `tc-cmdsub-siteid`). `command_effect` takes `Option<DiagSite>`: `Some` ⇒ emit
/// the disclosure with a real span; `None` ⇒ suppress it (the member-family path, which
/// re-discloses at the single-cell fallback — avoiding a double-report).
#[derive(Debug, Clone, Copy)]
pub struct DiagSite {
    span: Span,
    site: SiteId,
}

impl DiagSite {
    /// Build a give-up site from a node's source span + its CFG-node id (the grouping leaf).
    #[must_use]
    fn of(span: Span, node: CfgNodeId) -> Self {
        Self {
            span,
            site: SiteId::leaf(LeafId(node.0)),
        }
    }
}

/// A `DiagCode::CmdsubOperandTop` disclosure DEFERRED until after `mint_top_causes` (stage-1
/// cause-wiring, the corrected `tc-cmdsub-cause` resolution). The effects pass discovers WHICH
/// nodes went ⊤-via-`$(…)`-operand and at WHICH position, but the arch-1 ⊤-cause that links the
/// origin to its poisoned consumers is minted only AFTER the effects pass (a node's opaqueness IS
/// the effects pass's output — the ordering is inherent; see [`mint_top_causes`]). So the emit
/// site records just `(site, position)` here; [`classify`] finalizes the typed [`Diag`] with the
/// real `top_causes[node]` cause and lowers it once the arena's `&mut` is live and the causes are
/// minted. This keeps the effects pass a pure `Fn` (no `&mut arena` threaded into `solve`).
///
/// `pub` only because it appears in the (already-`pub`) [`command_effect`] signature as the
/// deferred-collector parameter; it carries no decision data and is never consumed cross-crate.
#[derive(Debug, Clone)]
pub struct CmdsubTop {
    /// The ⊤-origin site (its `site.leaf.0` is the CFG node index — the cause-lookup key).
    site: DiagSite,
    /// Which argv position is the `$(…)`/dynamic value (command word or 1-based operand).
    position: OperandPosition,
    /// The value-plane [`TopCause`] category (`219` q-2 — the cause-named ⊤): names WHY the word
    /// went ⊤ (a `$(…)` subst vs an unresolvable positional vs a dynamic var …), so the disclosure
    /// is specific. Exempt-plane (display only; distinct from the attribution `ProvId` cause).
    top_cause: dorc_core::TopCause,
    /// The command-word name for the `{command}` fill (`282` §12 item-6): the resolved literal at an
    /// operand-⊤ site, or [`CommandName::Unclear`] when the command word itself went ⊤.
    command: CommandName,
}

/// RECORD a `DiagCode::CmdsubOperandTop` disclosure for post-mint finalization (`22B` §5
/// worked-3; stage-1 cause-wiring). `site == None` ⇒ SUPPRESS (record nothing).
///
/// f-3b (`224` §10 22-q4, CORRECTED): the `None` caller is `member_family`'s per-member loop, and
/// that suppress is a LIVE dedup, REACHED in production — not belt-and-braces. `member_argv` is NOT
/// ⊤-free (`record_member_sites` resolves each member argv with no ⊤-gate; only the for-LIST words
/// and for-var reassignment are eligibility-gated, never other body-command operands), so
/// `for p in nginx curl; do apt-get install "$p" "$(date)"; done` yields a member argv carrying a ⊤
/// word. When `member_family` resolves it, the first ⊤ member hits `command_effect`'s ⊤-operand arm
/// → `Opaque` → the family's `_ => return None` → the family COLLAPSES → the site falls to the
/// single-cell fallback, which discloses the ⊤ once with the REAL span. Suppressing the member-scan
/// record here stops that disclosure from doubling the fallback's. No mis-elision rides this: a ⊤
/// operand always returns `Opaque` ⇒ the site runs (`kFAIL-perform`), so the ⊤ is never silently
/// elided — it is disclosed exactly once, at the fallback. (There is no sound assert to add: the
/// member argv legitimately CAN carry ⊤, so an "members are concrete" assertion would fire on the
/// valid input above.)
///
/// The actual [`Diag`] (carrying the minted `cause`) is built in [`finalize_cmdsub_tops`], post-
/// mint. The label is produced there from `position` (pure) so the disclosure text stays stable.
fn emit_cmdsub_operand_top(
    cmdsub_tops: &mut Vec<CmdsubTop>,
    site: Option<DiagSite>,
    position: OperandPosition,
    top_cause: dorc_core::TopCause,
    command: CommandName,
) {
    let Some(site) = site else {
        return; // member-family path: a ⊤ member IS reached here and SUPPRESSED (dedup) — disclosed once at the single-cell fallback; see fn doc f-3b
    };
    cmdsub_tops.push(CmdsubTop {
        site,
        position,
        top_cause,
        command,
    });
}

/// Finalize the deferred [`CmdsubTop`] records into typed [`Diag`]s, NOW carrying the real
/// arch-1 ⊤-cause (stage-1 cause-wiring; the corrected `tc-cmdsub-cause`). Runs in [`classify`]
/// AFTER [`mint_top_causes`], so `top_causes[node]` is available: the disclosure's site leaf IS
/// the CFG node index, so the cause for the node that went ⊤ is `top_causes[leaf]` (present
/// because the node bears an `Opaque` — that is why the disclosure fired; `fallback_cause` guards
/// the should-not-happen miss, matching [`reach_transfer`]).
///
/// Returns TYPED [`Diag`]s (cause-bearing); [`classify`] lowers them to the legacy stream. Kept
/// typed at this boundary because `to_legacy` DROPS the cause — the why-lens consumer reads the
/// cause off the typed `Diag`, so the cause must live on a typed value, not the lowered one.
///
/// THE WELD (ru-11): the `cause` is EXEMPT-plane — it rides the diagnostic for the why-lens /
/// dashboard dedup (`228` dc-1) but reaches no artifact (`to_legacy` keeps it off the bytes; the
/// `render_artifact_comment` for this code is `None`) and drives no decision. The cause is a
/// pure [`dorc_core::ProvId`] (non-`Display`, !`Ord`), so it cannot key a decision-output map.
fn finalize_cmdsub_tops(
    cmdsub_tops: &[CmdsubTop],
    top_causes: &[Option<dorc_core::ProvId>],
    fallback_cause: dorc_core::ProvId,
) -> Vec<Diag> {
    cmdsub_tops
        .iter()
        .map(|top| {
            let node = top.site.site.leaf.0 as usize;
            let cause = top_causes
                .get(node)
                .copied()
                .flatten()
                .unwrap_or(fallback_cause);
            Diag::new(
                Code::CmdsubOperandTop(CmdsubOperandTop {
                    site: top.site.site,
                    position: top.position,
                    cause: Some(cause),
                    top_cause: top.top_cause,
                    command: top.command.clone(),
                }),
                top.site.span,
            )
        })
        .collect()
}

/// Determine a `Command` node's effect cells from the book's resolved argv + the
/// oracle's own `check()` (the real entity-resolution mechanism; replaces the deleted
/// engine-side argparse stand-in). The engine parses NOTHING: it threads the
/// flow-resolved argv through the oracle's argparse (`predict::evaluate`) and reads
/// the inline kind-annotation. *Identity is declared, never inferred* — true in
/// code now (`inv-referent-agnostic`).
///
/// Returns a `Vec` of cells (a multi-cell verb is legal — `us-effectmap`); each is
/// `Establishes`/`Kills`/`Queries` (`Queries` is the read-only guard-class, 202 §2).
/// ANY ⊤ — a ⊤ argv word, no provider, no check, an `evaluate` `Top`, or no
/// effect-map entry — yields `[Opaque]` (`inv-top-reject`: the degrade is the floor;
/// both `kFAIL` directions). A bare assignment yields `[Pure]`.
///
/// `inv-superposition`: the cells are phase-/orientation-agnostic facts; this
/// classifies, it decides nothing. Diagnostics (kind-disagreement) accumulate in
/// `diags`.
///
/// `degrade` is the DIAGNOSTICS-ONLY reason channel (`inv-top-reject`'s "say so" half): the
/// [`predict::TopReason`] of the first candidate check that degraded, so the probe-side
/// `site-unresolvable` note can name a CAUSE and not just a site. Nothing branches on it —
/// the `Opaque` it accompanies is minted identically whether the slot is read or dropped, and
/// every caller that does not want it passes a throwaway.
/// The VERDICT-LANE decision at a concrete-argv site whose predict declared no marked effect: what
/// cell does a verdict-bearing provider's site establish? Three answers, in decreasing precision.
///
/// 1. The verdict body authored a coordinate for THIS argv (`26H` §3 / `evaluate_verdict_coord`)
///    ⇒ that AUTHORED cell. oracle-contract §4: "attach facts to the one line that measured them."
/// 2. It bears a verdict function but authored no usable coordinate here ⇒ the synthetic
///    **auto-cell** (`dorc_core::auto_fact` — a per-provider singleton), the `24L` §2/§3 typeless
///    floor: enough to light up the site's own elision/guard tier, and nothing more.
/// 3. No verdict function at all ⇒ `Opaque` (`inv-top-reject`, the honest floor).
///
/// Row 2 is the founding shape and stays byte-identical: `24L` §3 priced the singleton's coarseness
/// ("more same-cell staleness ⇒ more forced runs, never fewer") for the MARKLESS body it was
/// written for, and a body that authored nothing is still exactly that body. Row 1 is outside that
/// pricing — the author named the cell, so the engine stops pretending every site of one command
/// touches one thing (`26G:fnd-shared-auto-cell-collides`). What row 1 must NEVER do is split cells
/// the author did not split: two sites resolving the SAME coordinate share one cell and still
/// ⊤-merge on disagreement, because `an-written-stale` rests on same-state sites COLLIDING
/// (`26H` §3.4 — per-site synthetic cells are forbidden for exactly this reason).
///
/// Reached ONLY after the argv is confirmed fully concrete — a ⊤ command word or operand returned
/// `Opaque` earlier (never a cell over a non-per-site-resolvable argv). The auto-kind is keyed by
/// the MAPPED provider name so `apt-get`/`apt_get` share one cell (the same normalization
/// `build_vouches` and the probe funcname use).
///
/// `verdict_keyed` is set on rows 1 and 2 — the site's establish came from the verdict lane, so its
/// probe must ship the VERDICT body (there is no predict to answer this cell). That signal is
/// site-keyed and threaded out rather than re-derived downstream from the fact's KIND: a row-1 cell
/// is an ordinary authored kind, indistinguishable from a predict-minted one, so a kind test would
/// silently route it to the predict lane and the site would run (`26H` §3.5's likeliest breakage).
///
/// The definition arrives RESOLVED ([`live_verdict`]), because verdict primacy asks two questions of
/// one body — does it vouch this argv, and which cell does it key — and asking the frame twice is how
/// two readings of one environment come to disagree (`oracle/CLAUDE.md
/// the-frame-lookup-is-the-only-resolution-seat`).
///
/// The backing's minting family is threaded EXACTLY (`Some(provider)`), never left for
/// `sole_family` to recover: an authored verdict coordinate is not in the sparing dialect
/// (`build_dialect` mints from predict-derived cells only), so a recovered family could hand this
/// fact some OTHER family's dialect and spare a cell the verdict never minted a token for. Threading
/// keeps it colliding, which is `sparing-algebra`'s answer for an unminted token.
fn verdict_cell_or_auto(
    verdict: Option<&predict::Predict>,
    provider: ProviderId,
    arg_refs: &[&str],
    interner: &mut Interner,
    backings: &mut BTreeMap<FactKey, FactBacking>,
    verdict_keyed: &mut bool,
) -> Vec<CommandEffect> {
    let Some(verdict) = verdict else {
        return vec![CommandEffect::Opaque];
    };
    *verdict_keyed = true;
    let Some(coord) = dorc_oracle::verdict::evaluate_verdict_coord(verdict, arg_refs) else {
        let pname = interner.resolve(provider.0).to_owned();
        return vec![CommandEffect::Establishes(dorc_core::auto_fact(
            interner, &pname,
        ))];
    };
    let kind = KindId(interner.intern(&coord.kind));
    let entity = match &coord.entity {
        ResolvedEntity::Operand(text) => EntityRef::Operand(OpaqueToken(interner.intern(text))),
        ResolvedEntity::Singleton => EntityRef::Singleton,
    };
    let selector = SelectorId(interner.intern(&coord.selector));
    let fact = FactKey::cell(kind, entity, selector);
    let observed: BTreeSet<SelectorId> = coord
        .observed
        .iter()
        .map(|s| SelectorId(interner.intern(s)))
        .collect();
    record_backing(backings, fact, provider, &observed);
    vec![CommandEffect::Establishes(fact)]
}

/// The site's frame, plus the munged family segment (`apt_get` for a book word `apt-get`), derived
/// once per site rather than once per role.
///
/// It carried a positional AGREEMENT gate until `28Q` §1.3 replaced agreement with resolution: the
/// gate asked whether a whole-unit winner happened to be live here, which is a second reading of
/// the same environment the frame lookup reads directly. Two readings can disagree; one cannot.
struct VisibleRole<'a> {
    live: crate::funcenv::LiveDefinitions<'a>,
    node: CfgNodeId,
    family: String,
}

impl<'a> VisibleRole<'a> {
    fn at(
        live: crate::funcenv::LiveDefinitions<'a>,
        node: CfgNodeId,
        provider: ProviderId,
        interner: &Interner,
    ) -> Self {
        let family = dorc_oracle::to_funcname_segment(interner.resolve(provider.0));
        Self { live, node, family }
    }

    /// Which source index's rows answer for `suffix` at this site, among `count` candidates that
    /// `has` says declared the role (`28Q` §1.3 — the frame lookup is the only resolution seat).
    ///
    /// **Winner-shifting** (`28Q` §1, permanent): with no agreement veto standing behind it, a
    /// function-environment precision bug here SELECTS WHOSE JUDGMENT governs the site. This is a
    /// licensure seat wearing a lookup's clothes, and it is license-review-tier forever.
    fn answering(&self, suffix: &str, count: usize, has: impl Fn(usize) -> bool) -> Option<usize> {
        let name = format!("{}{suffix}", self.family);
        dorc_core::answering_file(self.live.definition_before(self.node, &name), count, |i| {
            has(i).then(|| self.live.provenance_of(i, &name))
        })
    }
}

/// The verdict funcdef live at THIS site, or `None` — resolved ONCE and handed to every act that
/// reads a verdict body here (`28Q` §4 `rul-verdict-primacy-at-the-ship-seat` gave this seat a second
/// consumer: the primacy test asks whether the body vouches this argv, and the cell mint asks which
/// cell it keys). Two lookups would be two readings of one environment, which is the failure class
/// `28M:fnd-verdict-resolution-duplicates-live-source` records; the wrapped lane already resolves its
/// inner verdict once for exactly this reason (`308:rul-carry-proof-is-same-definition`).
fn live_verdict<'i>(
    verdicts: &'i VerdictIndex,
    provider: ProviderId,
    visible: &VisibleRole<'_>,
) -> Option<&'i predict::Predict> {
    let file = visible.answering(VERDICT_SUFFIX, verdicts.source_count(), |i| {
        verdicts.contains(i, provider)
    })?;
    verdicts.get(file, provider)
}

/// `28Q` §4 `rul-verdict-primacy-at-the-ship-seat` — does the VERDICT body own this site's
/// convergence measurement, displacing a predict that also answers here?
///
/// Two conditions, and both are load-bearing. The site must be **mutation-capable with elision
/// statically available**: exactly one `Establishes` cell, which is the only shape
/// [`classify_one_site`] turns into an `Establish*` class. Anything else (a `Kills`, a `Queries`, a
/// multi-cell verb) is `MustRun` — elision is already unavailable there, so ship-predict-alone stays
/// licensed and the predict's declared cells stay exactly where they were. And the body must reach a
/// **vouching** answer for this argv: a declining verdict has nothing to measure
/// (`guard23-refusepath-rc0-never-passes`), and preferring it would key a record to a body the ship
/// seat then refuses to ship.
///
/// What this deliberately does NOT do is re-key the site's cell. The predict's argparse and cells
/// keep feeding the static concern topology unchanged (`28Q` §8 stage-0, verbatim): the site still
/// establishes what its author declared, so nothing downstream loses an invalidation it had — and the
/// elision that follows is a MONOLOGUE again, since the shipped body, its rc, and the vouch are now
/// one author's (`28P:tc-split-family-elides-on-two-authors`, retired at the license tier; the
/// cross-author residue that remains is the sparing tier's).
fn verdict_owns_the_measurement(
    verdict: Option<&predict::Predict>,
    cells: &[CommandEffect],
    arg_refs: &[&str],
) -> bool {
    use dorc_oracle::verdict::{VerdictResolution, evaluate_verdict};
    let Some(verdict) = verdict else {
        return false;
    };
    if !matches!(cells, [CommandEffect::Establishes(_)]) {
        return false;
    }
    matches!(
        evaluate_verdict(verdict, arg_refs),
        VerdictResolution::Vouched
    )
}

/// The predict check that answers at THIS site, or `None`.
///
/// ONE question, where there were three conditions. The whole-unit `live_source` scan and the
/// positional gate that narrowed its answer are gone together — they were two readings of one
/// environment (`28P:fnd-build-vouches-relifted-the-verdict-sets` is what a disagreement between
/// two such readings cost the last time). And the third condition, which checked that the effect
/// map's row came from the same file the identity resolved through, is now STRUCTURALLY
/// unnecessary: both the argparse and the cells are addressed by the SAME file index, so the
/// chimera — identity through one author's arms, cells another author declared
/// (`271:rul-sin-ordering`, the worst class) — cannot be spelled rather than being caught.
fn live_predict_source(
    checks: &[PredictSet],
    provider: ProviderId,
    visible: &VisibleRole<'_>,
) -> Option<usize> {
    visible.answering(predict::PREDICT_SUFFIX, checks.len(), |i| {
        checks.get(i).is_some_and(|cs| cs.get(provider.0).is_some())
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "the typeless-floor seam (`24L` §7) threads the verdict index alongside the \
              existing effect-map/checks/argv/interner/diag inputs; each is a distinct kernel \
              input, not a bundle-able struct. `degrade` and `verdict_keyed` are the reason and \
              lane channels, on the same out-param footing as `diags`/`cmdsub_tops`/`backings`; \
              `node`+`live` are the `28K` §2 positional-visibility pair, which is a fact about \
              WHERE the site is and cannot be recovered from the argv"
)]
pub fn command_effect(
    idx: &KindIndex,
    checks: &[PredictSet],
    verdicts: &VerdictIndex,
    argv: &[ValueOf],
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
    cmdsub_tops: &mut Vec<CmdsubTop>,
    site: Option<DiagSite>,
    backings: &mut BTreeMap<FactKey, FactBacking>,
    degrade: &mut Option<TopReason>,
    verdict_keyed: &mut bool,
    node: CfgNodeId,
    live_defs: crate::funcenv::LiveDefinitions<'_>,
) -> Vec<CommandEffect> {
    // A bare assignment-only command (`pkg=nginx`) has an empty argv ⇒ no
    // system-state effect (value::analyze yields `[]` for words.is_empty()).
    let Some(&word0) = argv.first() else {
        return vec![CommandEffect::Pure];
    };
    // The command word must be a concrete literal; a ⊤ word (`"$dyn" install …`) is
    // an un-modeled command ⇒ Opaque (`inv-top-reject`). The ⊤-degradation is no longer
    // silent (q-2 / find-3 no-silent-phantoms): disclose it through the migrated
    // `DiagCode::CmdsubOperandTop` spine (`22B` §5 worked-3). RECORDED here, finalized with
    // its arch-1 ⊤-cause post-mint (stage-1; see [`finalize_cmdsub_tops`]).
    let provider_sym = match word0 {
        ValueOf::Literal(s) => s,
        ValueOf::Top(cause) => {
            emit_cmdsub_operand_top(
                cmdsub_tops,
                site,
                OperandPosition::CommandWord,
                cause,
                CommandName::Unclear,
            );
            return vec![CommandEffect::Opaque];
        }
    };
    let provider_str = interner.resolve(provider_sym).to_owned();
    // Target-state-pure shell builtins (shell-env/stdout/control only, never an
    // oracle-modeled fact) are Pure, not Opaque, so they do NOT poison downstream
    // ambient-ness (fs-4 / spec_set_e; note 16G §4 B). Anything not listed stays
    // Opaque (the safe over-refusing direction).
    if is_target_state_pure_builtin(&provider_str) {
        return vec![CommandEffect::Pure];
    }
    // The provider symbol: the book's command word through the SHARED hyphen↔underscore
    // convention (`predict::map_provider_name`) — so it equals the `PredictSet` key and
    // `KindIndex`'s `ProviderId` (204 §6 seam #2). The book word is already hyphenated
    // (`apt-get`), so the map is a no-op here, but routing through the one helper keeps
    // the vocabularies welded.
    let provider = ProviderId(interner.intern(&predict::map_provider_name(&provider_str)));

    // The trailing args (command word excluded — C-1) must ALL be concrete literals
    // to run the check (202 §1 fully-concrete-argv scope). A ⊤ hole ⇒ unresolved site
    // ⇒ Opaque (runs). Collect the resolved text, holding it so `&str` slices borrow
    // it for `evaluate`.
    let mut arg_texts: Vec<String> = Vec::with_capacity(argv.len().saturating_sub(1));
    for (i, word) in argv[1..].iter().enumerate() {
        match word {
            ValueOf::Literal(s) => arg_texts.push(interner.resolve(*s).to_owned()),
            // ⊤ arg ⇒ unresolved ⇒ Opaque; disclose WHICH operand went ⊤ (q-2, the
            // 1-based operand index excluding the command word — the migrated
            // `DiagCode::CmdsubOperandTop` spine, `22B` §5 worked-3). RECORDED; finalized
            // with its arch-1 ⊤-cause post-mint (stage-1; see [`finalize_cmdsub_tops`]).
            ValueOf::Top(cause) => {
                let position = OperandPosition::Operand(
                    u32::try_from(i.saturating_add(1)).unwrap_or(u32::MAX),
                );
                emit_cmdsub_operand_top(
                    cmdsub_tops,
                    site,
                    position,
                    *cause,
                    CommandName::Literal(provider_str.clone()),
                );
                return vec![CommandEffect::Opaque];
            }
        }
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    let visible = VisibleRole::at(live_defs, node, provider, interner);
    let verdict_here = live_verdict(verdicts, provider, &visible);
    let live = live_predict_source(checks, provider, &visible);
    let resolved = live
        .and_then(|i| checks.get(i))
        .and_then(|cs| cs.get(provider.0))
        .and_then(|c| match predict::evaluate(c, &arg_refs) {
            predict::Resolution::Resolved(r) => Some(r),
            // The reason used to die here, which is why an unresolvable site could name itself but
            // never its cause.
            predict::Resolution::Top(reason) => {
                degrade.get_or_insert(reason);
                None
            }
        });
    // The verb key: the check's derived verb, or the ε-verb when the check binds none
    // (`useradd`, `command -v` — 202 §2 / task-W §4). `evaluate`'s verb is compared
    // against the effect-map's verb through the SAME `Interner` (204 seam #2).
    // The cells are read from the SAME file the argparse resolved through, which is what makes the
    // chimera unrepresentable rather than merely checked (`28Q` §1.3).
    let keyed = resolved.zip(live).and_then(|(r, file)| {
        let verb_key = match &r.verb {
            Some(v) => interner.intern(v),
            None => empty_verb(interner),
        };
        (!idx.effect_of(file, provider, verb_key).is_empty()).then_some((r, verb_key, file))
    });
    // TWO ways to arrive with no declared cell — (a) nothing resolved this argv, (b) something
    // RESOLVED but declared no cells for its verb — both handed to the VERDICT LANE (never a
    // verb-by-position fallback: the deleted engine-side argparse sin). (b) is why the lane is a
    // fact about the SITE, not re-derived by try-order: it leaves a shippable predict behind.
    let Some((resolved, verb_key, live_file)) = keyed else {
        return verdict_cell_or_auto(
            verdict_here,
            provider,
            &arg_refs,
            interner,
            backings,
            verdict_keyed,
        );
    };
    let effects = declared_cell_effects(
        idx, provider, verb_key, live_file, &resolved, interner, diags, backings,
    );
    // `28Q` §4 `rul-verdict-primacy-at-the-ship-seat` — this site has a resolvable predict AND a
    // vouching verdict. The VERDICT body owns the measurement: setting the lane out-param is what
    // makes the ship seat send that body, so the rc licensing the elision is the vouching author's
    // own. Prediction never licenses elision. `effects` is NOT re-keyed — the topology stays the
    // predict author's, which is why no downstream invalidation moves.
    if verdict_owns_the_measurement(verdict_here, &effects, &arg_refs) {
        *verdict_keyed = true;
    }
    effects
}

/// The cells the PREDICT author declared for this site's (provider, verb), as `CommandEffect`s, with
/// each establish's survival-backing threaded.
///
/// The cell's kind comes from the annotation (the declared identity, 204 §6); the effect-map supplies
/// selector + polarity per (provider, verb). Kind-agreement (204 open seam): if a cell's effect-map
/// kind disagrees with the annotation kind, diagnose and let the ANNOTATION win (the effect-map row is
/// re-keyed under it).
///
/// `277` §5 backing-SETS: each ESTABLISH fact's provenance is its minting FAMILY (this site's
/// `provider`, exact — not the `sole_family` reverse-lookup; `27D`
/// disposition-backing-family-recovery) plus the observe-backing-widening SELECTORS
/// (`idx.widening_of` — the `:?` observes that co-occurred with the verdict in this verb's predict
/// body; empty for the whole corpus). Only real oracle establishes are threaded; an auto-cell /
/// file-write / Members fact is absent here ⇒ `plan` falls back to the singleton `Backing::of_fact`
/// (the safe floor).
#[expect(
    clippy::too_many_arguments,
    reason = "extracted from command_effect to stay under the line cap; the args are the caller's \
              already-resolved keying quadruple (index, provider, verb, answering file) plus the \
              three out-params it shares with every other cell path"
)]
fn declared_cell_effects(
    idx: &KindIndex,
    provider: ProviderId,
    verb_key: dorc_core::Symbol,
    live_file: usize,
    resolved: &predict::Resolved,
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
    backings: &mut BTreeMap<FactKey, FactBacking>,
) -> Vec<CommandEffect> {
    let cells = idx.effect_of(live_file, provider, verb_key);
    let annotation_kind = KindId(interner.intern(&resolved.kind));
    let entity = match &resolved.entity {
        ResolvedEntity::Operand(text) => EntityRef::Operand(OpaqueToken(interner.intern(text))),
        ResolvedEntity::Singleton => EntityRef::Singleton,
    };
    // `EffectCell` is `Copy` and `cells` borrows `idx` (disjoint from `&mut interner`),
    // so iterate by copy — `cell_effect` takes `&mut interner` for the kind-agreement
    // diagnostic without conflicting with the `idx` borrow.
    let effects: Vec<CommandEffect> = cells
        .iter()
        .copied()
        .map(|cell| {
            cell_effect(
                cell,
                annotation_kind,
                &resolved.kind,
                entity,
                interner,
                diags,
            )
        })
        .collect();
    let observed = idx.widening_of(live_file, provider, verb_key);
    for e in &effects {
        if let CommandEffect::Establishes(fact) = e {
            record_backing(backings, *fact, provider, observed);
        }
    }
    effects
}

/// Merge a fact's threaded survival-backing (`277` §5). A COLLISION — the SAME fact established
/// at two sites by DIFFERENT providers — folds the family toward the safe floor `None` (no
/// sparing, exactly as `sole_family` answers an ambiguous `(kind, selector)`); the observed
/// widening selectors UNION (kill-surface only grows — `inv-kfail`, apply). Same-provider re-mint
/// is idempotent. Deterministic (`BTreeMap`/`BTreeSet`).
fn record_backing(
    backings: &mut BTreeMap<FactKey, FactBacking>,
    fact: FactKey,
    provider: ProviderId,
    observed: &BTreeSet<SelectorId>,
) {
    let entry = backings.entry(fact).or_insert_with(|| FactBacking {
        family: Some(provider),
        observed: BTreeSet::new(),
    });
    if entry.family != Some(provider) {
        entry.family = None; // cross-provider collision ⇒ safe floor (no sparing)
    }
    entry.observed.extend(observed.iter().copied());
}

/// Resolve an in-loop Members site to its establish-cell FAMILY (task-L2 item-2), or
/// `None` if it is not a Members site OR any member fails to resolve to a single
/// establish (ALL-OR-NOTHING — the family is never partial). For each per-member argv
/// ([`crate::value::ValueFlow::member_argv`]) run the oracle's own `check()` exactly as a
/// straight-line command; require `[CommandEffect::Establishes(fact)]` for EVERY member.
/// Any member that is Opaque (a ⊤ word, no check, the check refuses), a Kill, a Query, a
/// Pure, or a multi-cell verb ⇒ the whole site is `None` (it falls back to the single-cell
/// classification, which for an in-loop site is the render-floored Flat path ⇒ `MustRun`).
///
/// The kind-disagreement diagnostics each member's check may raise accumulate in `diags`
/// (shared with the straight-line path). Deterministic; never panics (`inv-no-throw`).
#[expect(
    clippy::too_many_arguments,
    reason = "the `28K` §2 positional oracle is one more distinct kernel input: a member resolves \
              through the same site's environment as the single-cell path, and recovering that \
              from the argv is exactly what the rule forbids"
)]
fn member_family(
    id: CfgNodeId,
    cfg: &Cfg,
    value: &ValueFlow,
    idx: &KindIndex,
    checks: &[PredictSet],
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
    live_defs: crate::funcenv::LiveDefinitions<'_>,
) -> Option<Vec<FactKey>> {
    if cfg.node(id).kind != CfgNodeKind::Command {
        return None;
    }
    let members = value.member_argv(id)?;
    let mut family = Vec::with_capacity(members.len());
    // A loop member NEVER forms a verdict-lane cell: the in-loop floor runs every member anyway
    // (`disposition_for`), so an EMPTY index keeps them Opaque ⇒ MustRun ⇒ Run (safe direction).
    let no_verdict_lane_in_members = VerdictIndex::default();
    for argv in members {
        // Each member is a concrete-or-⊤ argv; resolve it through the oracle check. All-or-nothing:
        // ANY non-single-establish member kills the whole family. `site: None` is a LIVE dedup
        // (f-3b CORRECTED, `224` §10 22-q4): a member argv CAN carry a ⊤ word (`record_member_sites`
        // does not ⊤-gate body operands), and a ⊤ member resolves Opaque → the family collapses
        // (`_ => None` below) → the site falls back to the single-cell `argv` path, which discloses
        // that ⊤ once with the REAL span. Passing `None` here stops the member-scan emit from
        // doubling that single fallback disclosure. (⊤ ⇒ Opaque ⇒ MustRun ⇒ the site runs, so the
        // ⊤ is disclosed, never mis-elided.) `site: None` records NO cmdsub-top disclosure, so a
        // discarded local collector is honest here (nothing is ever pushed to it).
        let mut suppressed = Vec::new();
        // A Members site is render-floored (every member RUNS), so its facts fall through to
        // `plan`'s singleton `Backing::of_fact` fallback — no threaded widening is needed here.
        // A throwaway map keeps the resolution local (`277` §5: member-widening is deferred, safe).
        let mut member_backings = BTreeMap::new();
        match command_effect(
            idx,
            checks,
            &no_verdict_lane_in_members,
            argv,
            interner,
            diags,
            &mut suppressed,
            None,
            &mut member_backings,
            // A member's degrade never reaches a surface: the whole family collapses to the
            // single-cell path below, which re-runs `command_effect` and records the reason there.
            &mut None,
            &mut false,
            id,
            live_defs,
        )
        .as_slice()
        {
            [CommandEffect::Establishes(fact)] => family.push(*fact),
            _ => return None,
        }
    }
    // An empty family cannot arise (a Members site has ≥1 member), but guard defensively.
    if family.is_empty() {
        return None;
    }
    Some(family)
}

/// Build one [`CommandEffect`] from a declared [`EffectCell`] under the resolved
/// (annotation-kind, entity). Enforces the kind-agreement rule (204 §6): the
/// annotation is the declared identity, so on a mismatch the cell is re-keyed under
/// the annotation kind and a warning is recorded.
fn cell_effect(
    cell: EffectCell,
    annotation_kind: KindId,
    annotation_kind_str: &str,
    entity: EntityRef,
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
) -> CommandEffect {
    if cell.kind != annotation_kind {
        let em_kind = interner.resolve(cell.kind.0).to_owned();
        // Spanless: no source location at this classification depth (arch-3-residual-2).
        diags.push(Diag::new_spanless_site(Code::EffectKindDisagreement(
            EffectKindDisagreement {
                annotated: annotation_kind_str.to_owned(),
                effect_map: em_kind,
            },
        )));
    }
    // The annotation wins (declared identity). Ambient context (`HostDefault`) — an in-book
    // establish is born in the caller's world; wrapped-site re-keying is `27C`'s probe-lane act,
    // never a classification-time fact property.
    let fact = FactKey::cell(annotation_kind, entity, cell.selector);
    match cell.claim {
        ValueClaim::Establish => CommandEffect::Establishes(fact),
        // TRANSITIONAL freeze (jc-polarity-vs-rc FINAL, ru-26 churn-disclosure): "no
        // polarity doctrine here — dissolves into the uniform no-vouch-no-elide license
        // when the guard/vouch tier lands." An rc-INVERTED claim (the former `kill` mark,
        // now rc-inversion plumbing on an OPAQUE value — NO create/destroy concept)
        // classifies MustRun: `Kills` gens the fact into `Reach` AND falls to the site
        // classifier's `_ => MustRun` arm, reproducing HEAD's Kill behaviour EXACTLY so
        // this re-spelling never begins eliding a formerly-kill site as a side effect.
        // DELIBERATELY spike-scoped churn-avoidance — this is NOT a polarity doctrine and
        // MUST NOT leak into greenfield work; it vanishes when the guard/vouch tier lands.
        ValueClaim::EstablishInverted => CommandEffect::Kills(fact),
        ValueClaim::Observe => CommandEffect::Queries(fact),
    }
}

/// Shell builtins with no *target-system* (location-3) effect: they change shell
/// options/cwd/variables or write to stdout/stderr, but never a package/file/
/// service fact an oracle models. Treated as `Pure` so they don't poison
/// reaching-defs ambient-ness (note 16G). Deliberately small and conservative —
/// anything not listed stays `Opaque` (the safe over-refusing direction); the
/// dynamic-lvalue forms (`unset "$x"`, `printf -v`) are already ⊤-rejected upstream
/// by the parser, so only their static uses reach here.
///
/// The classification assumes the word resolves to the BUILTIN — a book-defined
/// same-named function shadows a regular builtin in dash, which `cfg` discloses
/// per-funcdef (find-I, `cfg-builtin-shadowed`); `pub` so that disclosure — AND the cli's
/// firehose-suppression (cheap-7): a structurally-unprobeable pure-builtin/assignment site
/// gets NO "declare a probe" advice, since none could ever exist — share this ONE list (never
/// a parallel notion of "inert command").
#[must_use]
pub fn is_target_state_pure_builtin(word: &str) -> bool {
    matches!(
        word,
        "set"
            | "cd"
            | "export"
            | "unset"
            | "shift"
            | "read"
            | "readonly"
            | "local"
            | ":"
            | "true"
            | "false"
            | "echo"
            | "printf"
            | "test"
            | "["
    )
}

/// The per-path `file` cell a write-redirect (`>`/`>>`) to `path` establishes (y-1,
/// redirect-effects, `21F` imp-1). Follows the existing FactKey/kind vocabulary: a
/// blessed `file` kind (core: the Tier-A well-known kind names include `file`), the
/// resolved path as the entity operand (referent-agnostic — the path is an interned
/// token, never decoded beyond the syntactic `/dev/null` exemption at resolution), and
/// a single `written` selector (append vs truncate are BOTH write-shaped ⇒ the same
/// cell this round; no read-back / content discrimination). The cell GENS into
/// reaching-defs (so it poisons ambient-ness + invalidates a downstream Query, st-3),
/// but a `file` cell has no oracle/probe ⇒ it never licenses an elision (the charter's
/// "gen and poison, nothing licenses" — a `Redir` node is never a plan leaf anyway).
fn file_write_cell(path: dorc_core::Symbol, interner: &mut Interner) -> FactKey {
    FactKey::cell(
        KindId(interner.intern("file")),
        EntityRef::Operand(OpaqueToken(path)),
        SelectorId(interner.intern("written")),
    )
}

/// Render a resolved argv to display text for a diagnostic (q-2): each literal
/// resolved to its text, a `⊤` word shown as `⟨⊤⟩`. Display/provenance only — never
/// branched on (`inv-referent-agnostic`). Deterministic.
fn render_argv(argv: &[ValueOf], interner: &Interner) -> String {
    argv.iter()
        .map(|w| match w {
            ValueOf::Literal(s) => interner.resolve(*s).to_owned(),
            ValueOf::Top(_) => "⟨⊤⟩".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Facts mutated by some command on a path to here — or `Top` once an `Opaque`
/// command has run (then ANY fact may have changed). This is the reaching-defs
/// lattice; a fact is ambient at a point iff it is NOT in the in-state here.
///
/// `Top` now carries a **cause receipt** (arch-1 `Top(cause)`, `notes/220` §6 / `21Z`):
/// `Reach::Top` was "causally opaque" — it recorded THAT a give-up happened, not WHICH
/// command caused it. The [`ProvId`] makes the ⊤-poison cascade attributable (the
/// why-lens consumer, arch-2): every poisoned downstream site can name the origin that
/// poisoned it.
///
/// THE WELD (ru-11 / `22A` §1 arch-1): the cause is on the **exempt** plane — it may
/// influence no decision. Two structural facts enforce that here:
/// * the cause is **excluded from `Eq`** (the hand-written impl below compares only the
///   variant + fact-set), so `Top(a) ≡ Top(b)` exactly as the contract demands. This is
///   not merely a keying nicety — `solve`'s fixpoint test is `joined != state[w]`
///   (`solve.rs`), so a cause-sensitive `Eq` would make a ⊤ re-derived with a fresh cause
///   look "changed" forever and the worklist would NOT terminate. Excluding the cause is
///   correctness-critical, and the gate (`plan::erasability`) re-proves it adversarially.
/// * `classify` returns only [`SkipClass`]es; the cause never rides out of this module
///   into a license input. It is read only by the (controller-side, lazy) why-render.
#[derive(Debug, Clone)]
pub enum Reach {
    Facts(BTreeSet<FactKey>),
    /// ⊤ with its cause receipt (the give-up origin). The cause is EXEMPT (excluded from
    /// `Eq`/the fixpoint), per the WELD.
    Top(dorc_core::ProvId),
}

/// `Eq` **excludes the `Top` cause** (the WELD + termination, see [`Reach`]): two `Top`s
/// are equal regardless of cause, so the lattice fixpoint converges and a receipt can never
/// perturb the reaching-defs solution. `Facts` compares its set as usual.
impl PartialEq for Reach {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reach::Top(_), Reach::Top(_)) => true,
            (Reach::Facts(a), Reach::Facts(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for Reach {}

impl Lattice for Reach {
    fn bottom() -> Self {
        Reach::Facts(BTreeSet::new())
    }
    fn join(&self, other: &Self) -> Self {
        // ⊤ absorbs, carrying a cause. First-cause-wins (`notes/220` §6: "keep first-cause or
        // a k-capped Join node" — first-cause is termination-trivial here, and since `Eq`
        // ignores the cause the choice is decision-invariant by construction). When `self` is
        // ⊤ its cause carries; else if `other` is ⊤ that cause carries; else union the facts.
        match self {
            Reach::Top(cause) => Reach::Top(*cause),
            Reach::Facts(a) => match other {
                Reach::Top(cause) => Reach::Top(*cause),
                Reach::Facts(b) => Reach::Facts(a.union(b).copied().collect()),
            },
        }
    }
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // `Top` is the join-absorbing ⊤, hence meet's identity (`⊤ ⊓ x = x`).
            (Reach::Top(_), x) | (x, Reach::Top(_)) => x.clone(),
            (Reach::Facts(a), Reach::Facts(b)) => {
                Reach::Facts(a.intersection(b).copied().collect())
            }
        }
    }
}

impl Reach {
    fn with(&self, fact: FactKey) -> Reach {
        match self {
            // ⊤ absorbs an establish (its cause is preserved).
            Reach::Top(cause) => Reach::Top(*cause),
            Reach::Facts(s) => {
                let mut s = s.clone();
                s.insert(fact);
                Reach::Facts(s)
            }
        }
    }
    fn mutated(&self, fact: &FactKey) -> bool {
        match self {
            Reach::Top(_) => true,
            Reach::Facts(s) => s.contains(fact),
        }
    }

    /// The ⊤ cause receipt, if this state is `Top` (the why-lens's read; never a decision
    /// input — the WELD). `None` for a `Facts` state.
    #[must_use]
    pub fn top_cause(&self) -> Option<dorc_core::ProvId> {
        match self {
            Reach::Top(cause) => Some(*cause),
            Reach::Facts(_) => None,
        }
    }

    /// Is this a **pristine** reaching-state — NO write-or-unknown reached here? The
    /// rule-query-validity bit (205 §2 / 20A §4 st-3): a Query's probed rc is
    /// fold-usable iff no invalidating command (an oracled MUTATOR — any
    /// Establish/Kill — or an Opaque) reaches the guard from entry. Because Queries
    /// and pure builtins gen nothing into `Reach`, "no write-or-unknown reached" is
    /// exactly the empty (⊥) fact-set; a non-empty set (some mutator genned a cell) or
    /// `Top` (an opaque ran) is non-pristine ⇒ the guard's resting rc is stale.
    fn is_pristine(&self) -> bool {
        matches!(self, Reach::Facts(s) if s.is_empty())
    }
}

/// How a `Command` relates to the skip decision. This is the *input* the probe/
/// plan stage consumes — it does not skip anything itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipClass {
    /// Not an elidable establish — opaque, pure, kill, unrecognized, OR an
    /// establish whose reaching-context cannot be trusted: unreachable from entry
    /// (e.g. a function body with no modeled call-edge), or produced under an
    /// un-certified solve (`SolveConsistency::is_consistent`) ⇒ run.
    MustRun,
    /// Establishes `fact`, ambient here (no upstream mutation) ⇒ probe the host.
    EstablishAmbient(FactKey),
    /// Establishes `fact`, but the fact was mutated upstream in-script ⇒ the
    /// resting probe is not authoritative ⇒ run (or reason in-script; conservatively
    /// run). The `purge X; … install X` case.
    EstablishWritten(FactKey),
    /// A read-only **Query** guard reading `fact` (`command -v nginx` ⇒
    /// `tool:nginx@present`; 202 §2 / task-D2). Probe-resolvable like an
    /// `EstablishAmbient` (its check IS the probe), but its probed rc feeds the
    /// fold's **Status** channel rather than gating a mutation-elision — and ONLY
    /// when [`valid`](SkipClass::QueryResolvable::valid) holds.
    ///
    /// `valid` is the rule-query-validity bit (205 §2 / 20A §4 st-3): the guard's
    /// probe-time rc is fold-usable IFF NO invalidating command reaches the guard
    /// from entry — invalidating = an oracled MUTATOR (any Establish/Kill) or an
    /// Opaque; NOT invalidating = other Queries or blessed-pure builtins. When
    /// `valid == false` the guard's resting rc is stale (a mutator may have changed
    /// the cell), so the phased caller withholds the rc (status ⇒ ⊤) and the guard
    /// runs for real at apply — never a stale fold (`inv-superposition`: the bit is a
    /// phase-agnostic fact; the collapse stays in the caller).
    QueryResolvable { fact: FactKey, valid: bool },
    /// An in-loop **Members** establish site (task-L2 item-2, `209` brk-1(b)): the
    /// for-var is Members-bound and this body site's argv references it, so the site
    /// evaluates the check ONCE PER MEMBER ([`crate::value::ValueFlow::member_argv`])
    /// ⇒ a fact-FAMILY — one cell per member, in list order (duplicates kept). Each
    /// member is a normal concrete establish.
    ///
    /// ALL-OR-NOTHING at resolution (item-2): if ANY member's per-member argv fails to
    /// resolve to a single-establish cell (a ⊤ word, the check refuses, a multi-cell
    /// verb, …) the WHOLE site is `MustRun` (the family is never partial).
    ///
    /// `self_reached` is the item-3(b) **self-reach** bit (the subtle core of the license),
    /// a phase-agnostic engine fact (`inv-superposition`): the ONLY in-script writers
    /// reaching this site (including via the loop back-edge) are THIS leaf's own per-member
    /// establishes — no pre-loop writer, no in-loop sibling, no Opaque (⊤) reached it. The
    /// license (item-3, in `plan`) may elide the body ONLY when `self_reached` AND every
    /// member is Converged AND the consumption gates pass. RATIONALE (the fixed-point
    /// argument, preserved at the license site): the elision's own effect removes the
    /// body's writes, so under the elision the resting probe stays authoritative
    /// (elide-all is self-consistent); ANY non-self writer breaks that argument ⇒ refuse.
    EstablishMembers {
        members: Vec<FactKey>,
        self_reached: bool,
    },
    /// An inlined function-CALL site (arch-2, brk-2, `i-3`/`i-4`): the call's command word
    /// resolved to a same-file-earlier funcdef and its body was spliced into the CFG. This is
    /// the render/substitution LEAF (the call's own span); the spliced body commands are
    /// `spliced_internal` (not their own leaves). It carries the per-body-site classifications
    /// (`sites`, one [`InlineSite`] per effect-bearing/probeable body leaf, in source order) so
    /// the all-or-nothing CALL license (`plan`) can aggregate them and the probe can ship a
    /// `site N.M` sub-record per site.
    ///
    /// ALL-OR-NOTHING (the Members precedent, 20S): the call licenses a `Replace` ONLY when
    /// EVERY effect-bearing body site licenses elision — every body Establish/Kill is an
    /// `EstablishAmbient` whose fact is Converged (a body Kill, an Opaque, a ⊤, or a non-self/
    /// written establish blocks the WHOLE call), Queries pass their own gates, and the CALL
    /// site's own consumed channels are reproduced. One non-licensing body leaf ⇒ the call
    /// RUNS (the real body executes). No partial-body render ever (`i-3`).
    InlineCall { sites: Vec<InlineSite> },
}

/// arch-2 (`i-3`/`i-4`): one spliced funcdef-body LEAF site under an [`SkipClass::InlineCall`].
/// Carries the body command's CFG node (provenance + the plan's `has_top_successor` check) and
/// its own [`SkipClass`] classification (resolved with the call's positional bindings — `i-2`).
/// The plan aggregates these into the all-or-nothing CALL license; the probe ships one
/// `site N.M` sub-record per site (M = the index into the call's site list).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineSite {
    /// The spliced body command's CFG node (provenance; `has_top_successor` gate; never a Step
    /// leaf of its own — the CALL is the render unit, `i-3`).
    pub node: CfgNodeId,
    /// The body site's own classification (with the call's positionals bound, `i-2`): an
    /// `EstablishAmbient`/`QueryResolvable` is per-site probeable (`site N.M`); a Kill/Opaque/
    /// MustRun/EstablishWritten blocks the whole call.
    pub class: SkipClass,
}

/// Nodes reachable from the program `entry` by forward edges. The reaching-defs
/// in-state of an *unreachable* node is a vacuous ⊥ (its only predecessors, if
/// any, are themselves unreached), which is indistinguishable from a genuinely
/// clean "nothing upstream mutated this fact" — so [`classify`] must not read an
/// unreachable establish as ambient (find-A). Today the only unreachable
/// `Command`s are function bodies: a call site has no modeled call-edge into the
/// body (cfg `find-7`), so the body is a detached island. A simple forward
/// graph reachability; deterministic and total (indices are in-bounds by
/// construction, so it never panics — `inv-no-throw`).
fn reachable_from_entry(cfg: &Cfg) -> Vec<bool> {
    let mut seen = vec![false; cfg.node_count()];
    let mut stack = vec![cfg.entry()];
    seen[cfg.entry().index()] = true;
    while let Some(v) = stack.pop() {
        for w in cfg.succ_ids(v) {
            if !seen[w.index()] {
                seen[w.index()] = true;
                stack.push(w);
            }
        }
    }
    seen
}

/// The reaching-defs transfer: `out = in ⊔ gen(node)`, with an optional `suppress` node
/// whose gen is SKIPPED (task-L2 item-3(b) self-reach). Each cell gens its fact; an Opaque
/// joins ⊤ **with this node's pre-minted cause** (`top_causes[node]`, the arch-1
/// `Top(cause)`); Pure/Queries gen nothing. Suppressing a Members site's gen (its self-
/// establishes) lets the back-edge carry only OTHER writers' cells to its in-state, so a
/// pristine result there proves only-self-reaches. Pure; monotone (a smaller gen set is
/// still monotone) and finite-height ⇒ `solve` converges.
///
/// `top_causes[node]` is the node's pre-minted give-up cause, present iff the node bears an
/// `Opaque` (`classify` mints one per such node); `fallback_cause` is a single arena-real
/// `TopCause` covering the should-not-happen case. Both are computed ONCE up front (in
/// [`classify`], where the arena's `&mut` lives), so the transfer stays a pure `Fn` callable
/// inside `solve`'s closure and the arena is never mutated mid-fixpoint. The cause is EXEMPT
/// (excluded from `Reach`'s `Eq`, [`Reach`]), so it cannot perturb the fixpoint or any decision.
fn reach_transfer(
    effects: &[Vec<CommandEffect>],
    top_causes: &[Option<dorc_core::ProvId>],
    fallback_cause: dorc_core::ProvId,
    incoming: &Reach,
    node: usize,
    suppress: Option<usize>,
) -> Reach {
    if suppress == Some(node) {
        return incoming.clone();
    }
    let mut state = incoming.clone();
    for cell in &effects[node] {
        state = match cell {
            CommandEffect::Establishes(f) | CommandEffect::Kills(f) => state.with(*f),
            // An Opaque ALWAYS poisons to ⊤ (the correctness floor — never lose the poison);
            // it carries THIS node's give-up cause (`top_causes[node]`, the arch-1
            // `Top(cause)`). `fallback_cause` is a defensive arena-real cause for the
            // invariant-should-hold case where a per-node cause is unexpectedly absent
            // (`debug_assert`ed in `classify`); it keeps ⊤ correct without an arena `&mut` here.
            CommandEffect::Opaque => {
                let cause = top_causes
                    .get(node)
                    .copied()
                    .flatten()
                    .unwrap_or(fallback_cause);
                state.join(&Reach::Top(cause))
            }
            CommandEffect::Pure | CommandEffect::Queries(_) => state,
        };
    }
    state
}

/// The effect cells one CFG node gens into the reaching-defs (the per-node closure body of
/// [`classify`], extracted so `classify` stays under the line cap). A resolved Members site
/// gens its per-member establishes; an inlined CALL gens `Pure` (the spliced body carries the
/// effects); a `Command` resolves through the oracle check; a `Top` node is `Opaque`; a
/// WRITE-shaped `Redir` gens a per-path `file` cell (y-1) or `Opaque`+disclosure on a ⊤
/// target; everything else is `Pure`. Diagnostics (kind-disagreement, the q-2/y-1 ⊤
/// disclosures) accumulate in `diags`. Deterministic; never panics (`inv-no-throw`).
#[expect(
    clippy::too_many_arguments,
    reason = "extracted verbatim from classify's per-node closure to stay under the line cap; \
              the args are the closure's captured inputs (cfg/value/idx/checks/interner/diags); \
              s-2 adds `ast` so the q-2 ⊤-disclosures carry a REAL span (the migrated \
              dq-cmdsub-operand-top spine), not None; stage-1 adds `cmdsub_tops` so the cmdsub-⊤ \
              disclosures are RECORDED for post-mint cause-finalization (tc-cmdsub-cause)"
)]
fn node_effects(
    id: CfgNodeId,
    member_family: Option<&Vec<FactKey>>,
    cfg: &Cfg,
    value: &ValueFlow,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    checks: &[PredictSet],
    verdicts: &VerdictIndex,
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
    cmdsub_tops: &mut Vec<CmdsubTop>,
    backings: &mut BTreeMap<FactKey, FactBacking>,
    degrades: &mut BTreeMap<CfgNodeId, TopReason>,
    verdict_lane: &mut BTreeSet<CfgNodeId>,
    live_defs: crate::funcenv::LiveDefinitions<'_>,
) -> Vec<CommandEffect> {
    if let Some(family) = member_family {
        return family
            .iter()
            .map(|f| CommandEffect::Establishes(*f))
            .collect();
    }
    // arch-2: an inlined CALL gens Pure, NOT the Opaque its unmodeled word would resolve to —
    // the body (spliced after it) carries the effects. Opaque here would poison the call's OWN
    // spliced body (the establish reads Written) — the very poison the splice removes.
    if cfg.call_body_sites(id).is_some() {
        return vec![CommandEffect::Pure];
    }
    // s-2 (the EARLY classify-widening): resolve THIS node's real source span + a stable
    // site identity, so the migrated `dq-cmdsub-operand-top` spine carries a real span (not
    // None — drop-A) and a `SiteId`. The leaf id is the CFG-node index (the kernel runs
    // BEFORE the plan assigns LeafIds; this Note is render-plane-only and never keys the
    // probe-RESULTS lane, so the CFG-node-space id is an honest grouping site — flagged
    // `tc-cmdsub-siteid`). The legacy `redir_target_top` (NOT migrated) gains the real span too.
    let site = DiagSite::of(ast.node(cfg.node(id).ast).span, id);
    match cfg.node(id).kind {
        CfgNodeKind::Command => {
            let argv = value.argv_values(id);
            let before = cmdsub_tops.len();
            let mut degrade = None;
            let mut keyed_by_verdict = false;
            let effect = command_effect(
                idx,
                checks,
                verdicts,
                &argv,
                interner,
                diags,
                cmdsub_tops,
                Some(site),
                backings,
                &mut degrade,
                &mut keyed_by_verdict,
                id,
                live_defs,
            );
            if let Some(reason) = degrade {
                degrades.insert(id, reason);
            }
            if keyed_by_verdict {
                verdict_lane.insert(id);
            }
            narrow_cmdsub_spans_to_operand(&mut cmdsub_tops[before..], cfg, ast, id);
            effect
        }
        // An unmodeled construct may mutate anything ⇒ ⊤.
        CfgNodeKind::Top => vec![CommandEffect::Opaque],
        // y-1 (redirect-effects): a WRITE-shaped redirect (`>`/`>>`) to a real sink is a
        // file-write EFFECT — previously invisibly `Pure`, which MASKED a downstream guard
        // reading the just-written file (`21F` imp-1: a `printf >> f` before a `grep`-guard of
        // `f` minted a stale-guard elision). A resolved target gens a per-path `file` cell (a
        // WRITER ⇒ st-3's coarse invalidation makes a downstream Query non-pristine ⇒ `valid:
        // false`); a ⊤ target joins ⊤ (Opaque-poison) + a disclosure. A non-write redirect
        // (read, fd-dup, here-doc, `/dev/null`) is absent from `redir_target` ⇒ stays Pure.
        CfgNodeKind::Redir => match value.redir_target(id) {
            Some(ValueOf::Literal(path)) => {
                vec![CommandEffect::Establishes(file_write_cell(path, interner))]
            }
            Some(ValueOf::Top(_)) => {
                // Migrated onto the Diag spine (B4 sweep): the site carries a real span (s-2
                // widening) and a CFG-node-space SiteId (pre-plan; same precedent as
                // CmdsubOperandTop — flagged `tc-cmdsub-siteid`).
                diags.push(Diag::new(
                    Code::RedirTargetTop(RedirTargetTop { site: site.site }),
                    site.span,
                ));
                vec![CommandEffect::Opaque]
            }
            None => vec![CommandEffect::Pure],
        },
        _ => vec![CommandEffect::Pure],
    }
}

/// Narrow each just-recorded `cmdsub-operand-top` disclosure's caret from the whole-command span
/// to the exact ⊤ operand word's span (`aid-caret-span-precision`): a resolved argv is one entry
/// per source word (a `ValueOf` never word-splits — `value::ValueOf`), so argv position `k` maps
/// 1:1 onto the `Simple` node's word `k` (word 0 = command word, word `n` = operand `n`). Only the
/// ORDINARY command path narrows; the peeled/wrapper path (`peeled_node_effects`) keeps the
/// whole-command span, since a peeled inner-argv index no longer maps onto a book word. A missing
/// word index falls back to the recorded whole-command span (`inv-no-throw`; never panics).
fn narrow_cmdsub_spans_to_operand(
    recorded: &mut [CmdsubTop],
    cfg: &Cfg,
    ast: &dorc_syntax::ast::Ast,
    id: CfgNodeId,
) {
    let dorc_syntax::ast::NodeKind::Simple { words, .. } = &ast.node(cfg.node(id).ast).kind else {
        return;
    };
    for top in recorded {
        let k = match top.position {
            OperandPosition::CommandWord => 0,
            OperandPosition::Operand(n) => n as usize,
        };
        if let Some(&word_id) = words.get(k) {
            top.site.span = ast.node(word_id).span;
        }
    }
}

/// Does the item-3(b) **self-reach** condition hold at the Members site `site`? Re-solve
/// the reaching-defs with `site`'s own gen suppressed and check the site's in-state is
/// pristine (the empty fact-set, NOT ⊤). With the self-establish removed, the in-state is
/// exactly the cells written by OTHER reaching paths (pre-loop, in-loop sibling, or an
/// Opaque ⇒ ⊤); pristine ⟺ ONLY this leaf's own establishes reach it. An un-certified
/// suppressed solve ⇒ `false` (conservative refuse — the safe direction). This is a small
/// extra solve per Members site (≤ a handful per book; perf is network-dominated anyway).
fn self_reach_holds(
    cfg: &Cfg,
    effects: &[Vec<CommandEffect>],
    top_causes: &[Option<dorc_core::ProvId>],
    fallback_cause: dorc_core::ProvId,
    site: usize,
) -> (bool, SolveConsistency<Reach>) {
    let (sol, consistency) = solve_certified(cfg, Direction::Forward, |i, incoming: &Reach| {
        reach_transfer(effects, top_causes, fallback_cause, incoming, i, Some(site))
    });
    let holds = self_reach_answer(&consistency, sol.states.get(site));
    (holds, consistency)
}

/// THE SELF-REACH FLOOR (`302` §3.3): an un-certified re-solve answers `false` — the existing
/// conservative refuse, which costs an `EstablishMembers` license and never grants one.
///
/// A named seat so `302` §6.8 can exercise it with a REAL `Inconsistent`: the load-bearing half is
/// that a PRISTINE state does not rescue an uncertified solve, and only a test that holds both can
/// say so.
fn self_reach_answer(consistency: &SolveConsistency<Reach>, state: Option<&Reach>) -> bool {
    consistency.is_consistent() && state.is_some_and(Reach::is_pristine)
}

/// What the self-reach pass really saw, kept as scalars for the aid plane (R4, cross-lineage
/// review): the failing-CHECK total across every uncertified solve, how many SOLVES failed, the
/// first failing solve's real advisory, and the failing indices.
///
/// The account it replaces reported a SOLVE count under a failing-CHECK name and hard-coded the
/// advisory. That is the aid plane asserting things it did not observe, which is the one failure
/// mode this plane exists to prevent (`271:rul-sin-ordering`: a mis-attributed account is worse
/// than a missing one). Everything here is measured or absent.
#[derive(Debug, Default)]
struct SelfReachAccount {
    solves: usize,
    failing_checks: usize,
    advisory: Option<crate::certify::SolverAdvisory>,
    checks: Vec<dorc_aid::narrative::FailedCheck>,
}

impl SelfReachAccount {
    fn record(&mut self, report: &crate::certify::FailedChecks<Reach>) {
        self.solves = self.solves.saturating_add(1);
        self.failing_checks = self.failing_checks.saturating_add(report.total());
        if self.advisory.is_none() {
            self.advisory = Some(report.advisory());
        }
        self.checks.extend(failing_check_indices(report));
    }
}

/// The decision-inert record a consistency failure mints at its degrade
/// (`collapse-mints-narrative`), carrying SCALARS ONLY — the failing check INDICES, capped, plus
/// the counts and the solver's advisory report. The lattice values that failed stay behind in the
/// in-memory `SolveConsistency`: `Reach::Top` carries a `ProvId`, which this plane forbids
/// (`303:fnd-witness-operands-cannot-enter-narrative`).
fn failing_check_indices(
    report: &crate::certify::FailedChecks<Reach>,
) -> Vec<dorc_aid::narrative::FailedCheck> {
    let mut checks: Vec<dorc_aid::narrative::FailedCheck> = Vec::new();
    for &node in report.failing().boundary() {
        checks.push(dorc_aid::narrative::FailedCheck::Boundary {
            node: u32::try_from(node).unwrap_or(u32::MAX),
        });
    }
    for &(from, to) in report.failing().edges() {
        checks.push(dorc_aid::narrative::FailedCheck::Edge {
            from: u32::try_from(from).unwrap_or(u32::MAX),
            to: u32::try_from(to).unwrap_or(u32::MAX),
        });
    }
    checks
}

fn consistency_narrative(
    pass: SolvePass,
    report: &crate::certify::FailedChecks<Reach>,
) -> dorc_aid::CollapseNarrative {
    let checks = failing_check_indices(report);
    let advisory = report.advisory();
    dorc_aid::CollapseNarrative::new(
        dorc_aid::narrative::SpeechAct::Derived,
        dorc_aid::CollapseKind::SolverConsistencyFailure {
            pass,
            operands: dorc_aid::narrative::Operands::capped(checks),
            shown: u32::try_from(report.shown()).unwrap_or(u32::MAX),
            total: u32::try_from(report.total()).unwrap_or(u32::MAX),
            solves: 1,
            advisory: dorc_aid::narrative::SolverRounds {
                converged: advisory.converged,
                rounds: u32::try_from(advisory.rounds).unwrap_or(u32::MAX),
            },
        },
    )
}

/// The per-site classification, over already-computed state (`302` §3.4's floor lives here).
///
/// `trust_reach` false — which is what an un-certified reaching-defs answer produces — sends
/// EVERY shape to `SkipClass::MustRun`: the stage-0/⊤ posture, safe under both phases. Extracted
/// from the closure it used to be so `302` §6.8 can drive that floor with a real `Inconsistent`
/// rather than argue it; the closure remains, delegating.
fn classify_one_site(
    i: usize,
    effects: &[Vec<CommandEffect>],
    member_families: &[Option<Vec<FactKey>>],
    reach: &[Reach],
    trust_reach: bool,
    reachable: &[bool],
    self_reached: &BTreeMap<usize, bool>,
) -> SkipClass {
    let (Some(cells), Some(state), Some(&site_reachable)) =
        (effects.get(i), reach.get(i), reachable.get(i))
    else {
        return SkipClass::MustRun;
    };
    // task-L2: a resolved in-loop Members site (reachable + certified) ⇒ EstablishMembers.
    if let Some(Some(family)) = member_families.get(i)
        && trust_reach
        && site_reachable
    {
        return SkipClass::EstablishMembers {
            members: family.clone(),
            // Answered by `self_reach_pass` above; absent ⇒ the conservative `false`.
            self_reached: self_reached.get(&i).copied().unwrap_or(false),
        };
    }
    match cells.as_slice() {
        [CommandEffect::Establishes(f)] if trust_reach && site_reachable => {
            if state.mutated(f) {
                SkipClass::EstablishWritten(*f)
            } else {
                SkipClass::EstablishAmbient(*f)
            }
        }
        [CommandEffect::Queries(f)] if trust_reach && site_reachable => {
            SkipClass::QueryResolvable {
                fact: *f,
                valid: state.is_pristine(),
            }
        }
        _ => SkipClass::MustRun,
    }
}

/// Answer self-reach for every eligible Members site AHEAD of the per-site classifier
/// (`303:fnd-self-reach-has-no-diagnostic-channel`).
///
/// The classifier is a pure `Fn` closure with no diagnostic channel, so a refusal taken inside it
/// could be acted on but never narrated. Hoisting the whole population here gives the failure
/// somewhere to be reported from, and costs nothing: the per-site re-solves happen either way.
fn self_reach_pass(
    cfg: &Cfg,
    effects: &[Vec<CommandEffect>],
    top_causes: &[Option<dorc_core::ProvId>],
    fallback_cause: dorc_core::ProvId,
    eligible: &[usize],
    trip: &mut CertifierTrip,
) -> (BTreeMap<usize, bool>, SelfReachAccount) {
    let mut answers = BTreeMap::new();
    let mut account = SelfReachAccount::default();
    for &site in eligible {
        let (holds, consistency) = self_reach_holds(cfg, effects, top_causes, fallback_cause, site);
        answers.insert(site, holds);
        trip.record(&consistency);
        if let SolveConsistency::Inconsistent(report) = &consistency {
            account.record(report);
        }
    }
    (answers, account)
}

/// Mint the arch-1 `Top(cause)` receipts: a per-node give-up origin for every Opaque-bearing
/// node, keyed on that node's source [`Span`] (the stable site, `vp-9`), plus one site-less
/// `fallback_cause` for the defensive [`reach_transfer`] path. Done in ONE place (the only one
/// with the arena's `&mut`) so the transfer stays a pure `Fn` for `solve`. Hash-consing makes
/// two give-ups at the same site share one id and a re-mint across the self-reach re-solve
/// free. The causes are EXEMPT — they ride [`Reach::Top`] (excluded from its `Eq`) and never
/// leave `classify` as a decision input (`plan::erasability` proves the inertness).
fn mint_top_causes(
    cfg: &Cfg,
    ast: &dorc_syntax::ast::Ast,
    effects: &[Vec<CommandEffect>],
    arena: &mut dorc_core::ProvArena,
) -> (Vec<Option<dorc_core::ProvId>>, dorc_core::ProvId) {
    let top_causes: Vec<Option<dorc_core::ProvId>> = (0..effects.len())
        .map(|i| {
            if effects[i].contains(&CommandEffect::Opaque) {
                let site = ast.node(cfg.node(CfgNodeId(i as u32)).ast).span;
                Some(arena.leaf(dorc_core::OriginKind::TopCause, Some(site)))
            } else {
                None
            }
        })
        .collect();
    let fallback_cause = arena.leaf(dorc_core::OriginKind::TopCause, None);
    debug_assert!(
        (0..effects.len())
            .all(|i| !effects[i].contains(&CommandEffect::Opaque) || top_causes[i].is_some()),
        "every Opaque-bearing node must have a pre-minted Top(cause)"
    );
    (top_causes, fallback_cause)
}

/// Mint the `Derived`-tier fact-merge narrative the static value-plane `Reach::Top` collapse
/// narrates (C3; `27V` Lane A, `AID-NEEDS:law-collapse-mints-narrative`): one
/// [`dorc_aid::CollapseKind::FactMergeDisagreement`] per Opaque-bearing node — the cell whose
/// establishers meet to ⊤. Mirrors [`mint_top_causes`] (same Opaque-bearing key, same node-index
/// order), so the product `Vec` is mint-pass-ordered — deterministic, no clock (`inv-determinism`;
/// the `two-plane-aid-law` mint-order pin). Decision-inert: the narrative rides OUT of
/// [`classify_with_why_diags`] for the why-lens and feeds no decision (`two-plane-aid-law`;
/// `empty-world-byte-identical` holds — an oracle-free book has no Opaque nodes and mints none).
///
/// Operands are EMPTY here (`Operands::default`): the c8 reaching-defs walk recovers the
/// disagreeing establisher values+sites into them; until then the cell keys the disagreement and
/// each operand's `shown` is `None`. Tier is `Derived` (an engine derivation), CONTRASTING the
/// `Measured` probe merge (C4, `facts_from_sites`).
fn mint_merge_narrative(effects: &[Vec<CommandEffect>]) -> Vec<dorc_aid::CollapseNarrative> {
    (0..effects.len())
        .filter(|&i| effects[i].contains(&CommandEffect::Opaque))
        .map(|i| {
            dorc_aid::CollapseNarrative::new(
                dorc_aid::SpeechAct::Derived,
                dorc_aid::CollapseKind::FactMergeDisagreement {
                    cell: SiteId::leaf(LeafId(i as u32)),
                    operands: dorc_aid::narrative::Operands::default(),
                },
            )
        })
        .collect()
}

/// A wrapped BOOK site peeled into its inner command + composed context (`27N`; `27C` §3 "the fact
/// is born in the site's context"). Precomputed at the cli edge (`dorc_oracle::entry::peel_book_chain`)
/// — a pure DATA input threaded into [`classify_with_why_diags`]: the kernel stays wrapper-unaware,
/// resolving `inner_argv` against the inner oracle and re-keying the fact into `context`. The entry
/// DECISION is the phased cli/plan collapse, NOT here (`inv-superposition`). tc-flag (`27N`): the
/// FactKey-widening is done via this precomputed map, NOT wrapper-recursion in `command_effect` — the
/// "peel into `command_effect` per `thread-the-flat-coordinate`" question is flagged UP, not settled.
#[derive(Debug, Clone)]
pub struct PeeledSite {
    /// The inner (non-wrapper) command's full argv (command word first), resolved literals.
    pub inner_argv: Vec<ValueOf>,
    /// The composed inner context the wrapper chain denotes — the fact is re-keyed into it.
    pub context: Context,
}

/// Resolve a wrapped site's INNER command effect and re-key its facts into the composed context
/// (`27N`). Runs `command_effect` on `site.inner_argv` (the inner oracle resolves it — the wrapper
/// is peeled away) into a LOCAL backing map, then re-keys every fact-bearing effect and backing into
/// `site.context`: two same-cell facts in different contexts stay DISTINCT (`inv-site-keyed-results`,
/// now context-qualified) and never transport (`compare` answers `Unknown` across the gap).
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors node_effects: per-node effect resolution threads the whole compiled context (id/site/cfg/ast/idx/checks/verdict-providers/interner/diags/cmdsub-tops/backings/degrades); each a distinct input, not a bundle"
)]
fn peeled_node_effects(
    id: CfgNodeId,
    site: &PeeledSite,
    cfg: &Cfg,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    checks: &[PredictSet],
    verdicts: &VerdictIndex,
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
    cmdsub_tops: &mut Vec<CmdsubTop>,
    backings: &mut BTreeMap<FactKey, FactBacking>,
    degrades: &mut BTreeMap<CfgNodeId, TopReason>,
    verdict_lane: &mut BTreeSet<CfgNodeId>,
    live_defs: crate::funcenv::LiveDefinitions<'_>,
) -> Vec<CommandEffect> {
    let diag_site = DiagSite::of(ast.node(cfg.node(id).ast).span, id);
    let mut local: BTreeMap<FactKey, FactBacking> = BTreeMap::new();
    let mut degrade = None;
    let mut keyed_by_verdict = false;
    let raw = command_effect(
        idx,
        checks,
        verdicts,
        &site.inner_argv,
        interner,
        diags,
        cmdsub_tops,
        Some(diag_site),
        &mut local,
        &mut degrade,
        &mut keyed_by_verdict,
        id,
        live_defs,
    );
    if let Some(reason) = degrade {
        degrades.insert(id, reason);
    }
    if keyed_by_verdict {
        verdict_lane.insert(id);
    }
    for (fact, backing) in local {
        backings.insert(fact.in_context(site.context), backing);
    }
    raw.into_iter()
        .map(|e| match e {
            CommandEffect::Establishes(f) => CommandEffect::Establishes(f.in_context(site.context)),
            CommandEffect::Kills(f) => CommandEffect::Kills(f.in_context(site.context)),
            CommandEffect::Queries(f) => CommandEffect::Queries(f.in_context(site.context)),
            other => other,
        })
        .collect()
}

/// Precompute, per CFG node, its in-loop Members family (task-L2 item-2) and its effect cells,
/// collecting the deferred cmdsub-⊤ disclosures (stage-1) along the way. Extracted from
/// [`classify`]'s body to keep it under the line cap; reads only `&` inputs plus `&mut interner`
/// (interning) and `&mut diags` (the kind-disagreement / `$()`-inner-nonleaf disclosures).
///
/// Member-families are computed FIRST so `effects` can gen the member cells into the reaching-
/// defs (a resolved Members site gens its member cells, NOT Opaque — else its own back-edge would
/// poison its in-state to ⊤ and break item-3's self-reach). The cmdsub-⊤ records carry no cause
/// yet (the arch-1 cause is minted post-effects-pass); [`classify`] finalizes them after
/// [`mint_top_causes`]. Deterministic; never panics (`inv-no-throw`).
#[expect(
    clippy::type_complexity,
    clippy::too_many_arguments,
    reason = "the parallel-by-node products (member families, effect cells, deferred cmdsub-⊤ \
              records, backings) are returned as one tuple so classify stays under the line cap; \
              naming a struct for a single-call-site internal helper buys nothing. The \
              verdict-provider set (`24L` §7 seam) is one more distinct kernel input, and the \
              degrade-reason map rides in as an out-param beside `diags`"
)]
fn resolve_node_effects(
    cfg: &Cfg,
    value: &ValueFlow,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    checks: &[PredictSet],
    verdicts: &VerdictIndex,
    peeled: &BTreeMap<CfgNodeId, PeeledSite>,
    interner: &mut Interner,
    diags: &mut Vec<Diag>,
    degrades: &mut BTreeMap<CfgNodeId, TopReason>,
    verdict_lane: &mut BTreeSet<CfgNodeId>,
    live_defs: crate::funcenv::LiveDefinitions<'_>,
) -> (
    Vec<Option<Vec<FactKey>>>,
    Vec<Vec<CommandEffect>>,
    Vec<CmdsubTop>,
    BTreeMap<FactKey, FactBacking>,
) {
    let n = cfg.node_count();
    let member_families: Vec<Option<Vec<FactKey>>> = (0..n)
        .map(|i| {
            member_family(
                CfgNodeId(i as u32),
                cfg,
                value,
                idx,
                checks,
                interner,
                diags,
                live_defs,
            )
        })
        .collect();
    let mut cmdsub_tops: Vec<CmdsubTop> = Vec::new();
    // `277` §5 backing-SETS: the fact → survival-backing-provenance map, threaded from the
    // establishing `command_effect` (minting family + observe-widening selectors).
    let mut backings: BTreeMap<FactKey, FactBacking> = BTreeMap::new();
    let effects: Vec<Vec<CommandEffect>> = (0..n)
        .map(|i| {
            let id = CfgNodeId(i as u32);
            // A wrapped BOOK site (`27N`): resolve the INNER command + re-key in-context. The
            // wrapper word itself would wall opaquely (unchanged law) — the peel replaces that.
            if let Some(site) = peeled.get(&id) {
                return peeled_node_effects(
                    id,
                    site,
                    cfg,
                    ast,
                    idx,
                    checks,
                    verdicts,
                    interner,
                    diags,
                    &mut cmdsub_tops,
                    &mut backings,
                    degrades,
                    verdict_lane,
                    live_defs,
                );
            }
            node_effects(
                id,
                member_families[i].as_ref(),
                cfg,
                value,
                ast,
                idx,
                checks,
                verdicts,
                interner,
                diags,
                &mut cmdsub_tops,
                &mut backings,
                degrades,
                verdict_lane,
                live_defs,
            )
        })
        .collect();
    (member_families, effects, cmdsub_tops, backings)
}

/// Classify every `Command` node for the skip decision: resolve each command's
/// effect cells (through the book's value-flow [`ValueFlow`] + the oracle's own
/// `check()`), then a forward reaching-defs pass tells us, per establishing command,
/// whether its fact is ambient. An establish is only offered as `EstablishAmbient`
/// when its reaching-context is *trustworthy* — reachable from entry AND under a
/// CERTIFIED solve (`SolveConsistency::is_consistent`, never the advisory `converged`
/// flag); otherwise it folds to the safe `MustRun` (find-A/find-B).
///
/// `value` is the book-side value-flow (`analysis::value::analyze`, the caller
/// threads it); `checks` are the per-oracle-file `PredictSet`s (the engine parses no
/// argv itself — `inv-referent-agnostic`). `ast` is threaded only to mint each give-up
/// site's `Top(cause)` receipt at its source [`Span`] (arch-1, `notes/220` §6); `arena`
/// is the per-run receipts plane the causes land in. Returns a [`Carrier`] so
/// kind-disagreement warnings (204 §6) surface. Deterministic; never panics (`inv-no-throw`).
///
/// THE WELD (ru-11): the minted causes are EXEMPT — they ride [`Reach::Top`] (excluded from
/// its `Eq`, so they cannot perturb the fixpoint) and never leave this function as a
/// [`SkipClass`] field, so no license input can depend on one. The arena grows but the
/// classification is byte-identical with the causes stripped (the `plan::erasability` gate
/// proves exactly this).
///
/// This is the thin wrapper over [`classify_with_why_diags`] for the 13 callers that do not
/// consume the typed why-lens diags (the cli's stage-3 disclosure is the one that does).
#[expect(
    clippy::too_many_arguments,
    reason = "the typeless-floor seam (`24L` §7) threads the verdict-provider set through the \
              classify entry points as DATA (the kernel stays verdict-unaware); one more distinct \
              input, not a bundle"
)]
/// This is the thin wrapper's OWN caveat: it drives an UNSOLVED function environment
/// ([`crate::funcenv::LiveDefinitions::unsolved`]), so the `28K` §2 positional gate is inert on
/// this path. That is right for its callers — kernel unit tests over hand-built indices, where no
/// source text exists for an environment to be solved over — and wrong for a driver, which is why
/// both real drivers call [`classify_with_why_diags`] with a solved one.
pub fn classify(
    cfg: &Cfg,
    value: &ValueFlow,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    checks: &[PredictSet],
    verdicts: &VerdictIndex,
    interner: &mut Interner,
    arena: &mut dorc_core::ProvArena,
) -> Carrier<Vec<(CfgNodeId, SkipClass)>> {
    classify_with_why_diags(
        cfg,
        value,
        ast,
        idx,
        checks,
        verdicts,
        &BTreeMap::new(),
        &crate::erase::ErasedSites::none(),
        interner,
        arena,
        &mut BTreeMap::new(),
        &mut BTreeSet::new(),
        &mut CertifierTrip::default(),
        crate::funcenv::LiveDefinitions::unsolved(),
    )
    .0
}

/// [`classify_with_why_diags`]'s survival-backing product accessor (`277` §5): the fact →
/// [`FactBacking`] map (minting family + observe-widening selectors) the cli threads into
/// [`dorc_plan::build_plan_walled`]. Named here only for the doc-link; the map is the 5th tuple
/// element (see the fn's return type).
#[doc(hidden)]
pub type BackingMap = BTreeMap<FactKey, FactBacking>;

/// [`classify`] PLUS the TYPED cause-bearing cmdsub-⊤ disclosures for the why-lens (`22D`
/// stage-3). The legacy [`Carrier`]'s `diags` already carries these LOWERED (cause-dropped, for
/// `report`/gate-3); this ALSO returns them TYPED so the cli's why-lens render can read the
/// `cause` off them (`to_legacy` drops it — [`dorc_aid::diag::why`] needs the typed value).
///
/// Returns `(Carrier<dispositions+legacy-diags>, typed-why-lens-diags, kill-node-set,
/// kill-coords, backing-map, collapse-narrative)`. `degrades` is an OUT-PARAM rather than a
/// seventh product, on the same footing as `diags`: the diagnostics-only per-node
/// [`predict::TopReason`] map (`command_effect`'s `degrade` channel) that the probe-side
/// `site-unresolvable` note renders as a CAUSE. Nothing branches on it — a site's `Opaque` is
/// minted identically whether a caller keeps the map or throws it away.
/// The collapse-narrative element is the C3 aid plane
/// (`27V` Lane A): one `Derived`-tier [`dorc_aid::CollapseKind::FactMergeDisagreement`] per
/// Opaque-bearing node ([`mint_merge_narrative`]), decision-inert (`two-plane-aid-law`) and threaded
/// to the why-lens seam. The typed diags are a subset-by-construction of the lowered ones
/// (the same `CmdsubOperandTop`s, before lowering) — no second pass, no divergence. EXEMPT
/// (ru-11): the typed diags' `cause` informs the render only, never a decision. The **backing-map**
/// (`277` §5) is fact → [`FactBacking`] (minting family + observe-widening selectors), threaded to
/// [`dorc_plan::build_plan_walled`] so the survival tier builds each fact's backing SET.
///
/// The **kill-node set** (R3 / 24A §3 — the kill gap) is the set of leaf [`CfgNodeId`]s whose
/// [`CommandEffect`] is a `Kills` (`apt-get purge`; `EstablishInverted` ⇒ `Kills` ⇒ classifies
/// `MustRun`, indistinguishable from a pure builtin / opaque in the [`SkipClass`] alone). A
/// RUNNING kill mutates the world, so it must WALL downstream different-cell converged
/// establishes (the same under-execute shape [`build_plan`](dorc_plan) closed at plan time for
/// modeled mutators, fd10). The phased caller threads this set to
/// [`dorc_plan::build_plan_walled`] so the wall predicate can see kills; the pure kernel stays
/// phase-agnostic (`inv-superposition`). Deterministic (`BTreeSet`, `inv-determinism`).
#[expect(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "the six parallel products (site classifications + typed why-lens diags + the R3 \
              kill-node set + the killed-coordinate side-map, 24E §7 + the `277` §5 backing-map + \
              the C3 collapse-narrative aid plane) are the fn's whole output; a named struct for a \
              two-call-site return (the cli + the plan test seam) buys nothing. The verdict-provider \
              set (`24L` §7 seam) is one more input, and its threaded call pushes the body just over \
              the line cap — the classify core is irreducibly long"
)]
pub fn classify_with_why_diags(
    cfg: &Cfg,
    value: &ValueFlow,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    checks: &[PredictSet],
    verdicts: &VerdictIndex,
    peeled: &BTreeMap<CfgNodeId, PeeledSite>,
    erased: &crate::erase::ErasedSites,
    interner: &mut Interner,
    arena: &mut dorc_core::ProvArena,
    degrades: &mut BTreeMap<CfgNodeId, TopReason>,
    verdict_lane: &mut BTreeSet<CfgNodeId>,
    trip: &mut CertifierTrip,
    live_defs: crate::funcenv::LiveDefinitions<'_>,
) -> (
    Carrier<Vec<(CfgNodeId, SkipClass)>>,
    Vec<Diag>,
    BTreeSet<CfgNodeId>,
    BTreeMap<CfgNodeId, FactKey>,
    BTreeMap<FactKey, FactBacking>,
    Vec<dorc_aid::CollapseNarrative>,
    BTreeSet<CfgNodeId>,
) {
    let mut diags: Vec<Diag> = Vec::new();
    // Precompute every node's member-family + effect cells, recording the deferred cmdsub-⊤
    // disclosures (stage-1) and the `277` §5 survival-backing provenance. Extracted so this fn
    // stays under the line cap. `27N`: a wrapped BOOK site (`peeled`) resolves its INNER command
    // and re-keys the fact into the composed context.
    let (member_families, mut effects, cmdsub_tops, backings) = resolve_node_effects(
        cfg,
        value,
        ast,
        idx,
        checks,
        verdicts,
        peeled,
        interner,
        &mut diags,
        degrades,
        verdict_lane,
        live_defs,
    );

    // THE erasure seam (`26H` §4), applied ONCE. `Pure` deliberately: an `Erased` variant would
    // recreate the every-consumer-must-remember surface the overlay model exists to abolish.
    for site in erased.iter() {
        if let Some(cells) = effects.get_mut(site.index()) {
            *cells = vec![CommandEffect::Pure];
        }
    }

    // From the RESIDUAL model; `SkipClass` cannot answer this (a Kill, an Opaque, and a pure
    // builtin all say `MustRun`).
    let invalidators: BTreeSet<CfgNodeId> = effects
        .iter()
        .enumerate()
        .filter(|(_, cells)| cells.iter().any(gens_into_reach))
        .map(|(i, _)| CfgNodeId(i as u32))
        .collect();

    // arch-1 `Top(cause)`: mint a give-up origin per Opaque-bearing node (+ a fallback),
    // keyed on source spans, so the ⊤-poison cascade is attributable. The cause is EXEMPT
    // (rides `Reach::Top`, excluded from `Eq`); it perturbs no decision.
    let (top_causes, fallback_cause) = mint_top_causes(cfg, ast, &effects, arena);

    // C3 (`27V` Lane A): narrate the give-up as a decision-inert record (see `mint_merge_narrative`).
    let mut collapse_narrative = mint_merge_narrative(&effects);

    // stage-1 cause-wiring (the corrected `tc-cmdsub-cause`): NOW that `top_causes` is minted,
    // finalize the deferred cmdsub-⊤ disclosures with each node's real ⊤-cause. The TYPED diags
    // are returned for the why-lens (stage-3); a LOWERED copy rides `diags` for `report`/gate-3.
    // This is the post-mint pass the ordering DEMANDS (a node's opaqueness is the effects pass's
    // output, so its cause cannot exist earlier). The cause is EXEMPT (ru-11) — it rides the
    // typed diagnostic for the why-lens, never an artifact or a decision.
    let why_diags = finalize_cmdsub_tops(&cmdsub_tops, &top_causes, fallback_cause);
    // A COPY rides `diags` for `report`/gate-3; the originals are returned for the why-lens
    // (stage-3) — the typed `cause` on the returned diags is what `dorc_aid::diag::why` reads.
    diags.extend(why_diags.iter().cloned());

    // Forward reaching-defs: out = in ⊔ gen(node). Each of a node's cells is genned
    // (a multi-cell verb writes every cell); an Opaque cell joins ⊤ (carrying its
    // pre-minted cause). A `Queries` cell gens NOTHING — a read poisons no ambient-ness and
    // invalidates no downstream Query (it is a write-free observation; task-D2 / st-3, 20A
    // §4). This is the gen-side of rule-query-validity: because a Query gens nothing,
    // `reach.states` (the IN-state at each node) carries exactly the writes-or-opaque reached.
    let (reach, reach_consistency) =
        solve_certified(cfg, Direction::Forward, |i, incoming: &Reach| {
            reach_transfer(&effects, &top_causes, fallback_cause, incoming, i, None)
        });
    // NO ASSERTION HERE, deliberately (R5, cross-lineage review). An assert would fire BEFORE the
    // floor and the report, so a debug build — which is what DST and every integration test run —
    // would panic exactly where the machinery is supposed to demote and explain, leaving the real
    // path exercised only in release. The diagnostic minted below IS the loud signal, and it is
    // the same one in both profiles.
    //
    // Two reasons the reaching in-state cannot be trusted to mean "nothing
    // upstream mutated this fact", both folding the safe way (→ MustRun):
    //   * an un-certified answer: the solver's states are not a post-fixpoint of its own system,
    //     so a real upstream kill may not have propagated.
    //   * unreachability (find-A): an establish unreachable from entry has a vacuous
    //     ⊥ in-state; its true call context is unmodeled (cfg find-7).
    // THE REACH FLOOR (`302` §3.4): an un-certified reaching-defs answer sends every site to
    // `SkipClass::MustRun` — the stage-0/⊤ posture, safe under both phases.
    let trust_reach = reach_consistency.is_consistent();
    trip.record(&reach_consistency);
    let reachable = reachable_from_entry(cfg);

    // The self-reach population, answered ahead of the classifier so a refusal can be narrated.
    let members_sites: Vec<usize> = (0..effects.len())
        .filter(|&i| member_families[i].is_some() && trust_reach && reachable[i])
        .collect();
    let (self_reached, self_reach) = self_reach_pass(
        cfg,
        &effects,
        &top_causes,
        fallback_cause,
        &members_sites,
        trip,
    );

    if let SolveConsistency::Inconsistent(report) = &reach_consistency {
        diags.push(Diag::new_spanless_site(Code::SolverConsistencyFailure(
            SolverConsistencyFailure {
                pass: SolvePass::ReachingDefs,
                failing: report.total().to_string(),
            },
        )));
        collapse_narrative.push(consistency_narrative(SolvePass::ReachingDefs, report));
    }
    // `failing` is the failing-CHECK total here exactly as it is for every other pass; the plural
    // SOLVE count rides the narrative's own field. Reporting one under the other's name is the
    // fabricated account R4 removed.
    if let Some(advisory) = self_reach.advisory {
        diags.push(Diag::new_spanless_site(Code::SolverConsistencyFailure(
            SolverConsistencyFailure {
                pass: SolvePass::SelfReach,
                failing: self_reach.failing_checks.to_string(),
            },
        )));
        collapse_narrative.push(dorc_aid::CollapseNarrative::new(
            dorc_aid::narrative::SpeechAct::Derived,
            dorc_aid::CollapseKind::SolverConsistencyFailure {
                pass: SolvePass::SelfReach,
                operands: dorc_aid::narrative::Operands::capped(self_reach.checks.clone()),
                shown: u32::try_from(self_reach.checks.len()).unwrap_or(u32::MAX),
                total: u32::try_from(self_reach.failing_checks).unwrap_or(u32::MAX),
                solves: u32::try_from(self_reach.solves).unwrap_or(u32::MAX),
                advisory: dorc_aid::narrative::SolverRounds {
                    converged: advisory.converged,
                    rounds: u32::try_from(advisory.rounds).unwrap_or(u32::MAX),
                },
            },
        ));
    }

    // The per-site single-fact / member classification (the shared core, used by both the
    // ordinary leaf path below and the arch-2 inlined-call body-site aggregation). Reads only
    // already-computed state (`effects`, `member_families`, `reach`, `trust_reach`,
    // `reachable`), so it is a pure closure.
    let classify_site = |i: usize| -> SkipClass {
        classify_one_site(
            i,
            &effects,
            &member_families,
            &reach.states,
            trust_reach,
            &reachable,
            &self_reached,
        )
    };

    let mut out = Vec::new();
    // R3 (24A §3 — the kill gap): leaf nodes whose effect is a `Kills`. A `Kills` classifies
    // `MustRun` (indistinguishable from pure/opaque in the `SkipClass`), so the plan-time wall
    // predicate can't see it — this set carries it out to `build_plan_walled`. `Kills` is a
    // real mutator: a RUNNING kill must wall downstream, exactly like a modeled establish.
    let mut kills: BTreeSet<CfgNodeId> = BTreeSet::new();
    // 24E §7 (resid-kill-coherence): the killed COORDINATE per single-kill DIRECT-LEAF node — the
    // comparand the cli applies to kill-walls (own-killed-coord ⊆ footprint, the establish-wall
    // coherence check extended to kills). A multi-kill node (none in the corpus) is left OUT (no
    // single comparand) ⇒ it keeps the pre-24E behaviour (no kill-wall coherence), still safe: the
    // check only ever REFUSES a drifted footprint, never licenses one. InlineCall kills likewise
    // skip it (the kill lives in a spliced body site — no corpus case, cheap to defer).
    let mut kill_coords: BTreeMap<CfgNodeId, FactKey> = BTreeMap::new();
    let bears_kill = |cs: &[CommandEffect]| cs.iter().any(|e| matches!(e, CommandEffect::Kills(_)));
    for (i, cells) in effects.iter().enumerate() {
        let id = CfgNodeId(i as u32);
        // Only genuinely-runnable command leaves are plan/apply units. A command
        // inside a `$( … )` substitution body is effect-bearing (it stayed in the
        // reaching-defs above, so its mutations still poison/establish) but is NOT
        // a leaf (find-cli-1, the dn-3 leaf-seam). arch-2: a SPLICED funcdef-body command is
        // likewise effect-bearing-but-not-a-leaf — its `site N.M` record rides the CALL (below).
        if cfg.node(id).kind == CfgNodeKind::Command && cfg.is_expansion_internal(id) {
            // q-2 (`dq-cmdsub-inner-nonleaf`, the `exec-subst-body-nonleaf` disclosure): an
            // EFFECT-BEARING `$()`-internal command runs un-elidably (it has no leaf of its
            // own, so it executes whenever its enclosing line runs). Today this is silent
            // (`219` q-1.f). A Pure inner command discloses nothing (nothing un-elidable
            // happens), so gate on a non-Pure effect.
            if cells.iter().any(|e| *e != CommandEffect::Pure) {
                // Migrated onto the Diag spine (B4 sweep). Real span from s-2 widening;
                // CFG-node-space SiteId (pre-plan; flagged `tc-cmdsub-siteid`).
                let span = ast.node(cfg.node(id).ast).span;
                let inner = render_argv(&value.argv_values(id), interner);
                diags.push(Diag::new(
                    Code::CmdsubInnerNonleaf(CmdsubInnerNonleaf {
                        site: SiteId::leaf(LeafId(id.0)),
                        inner,
                    }),
                    span,
                ));
            }
            continue;
        }
        if cfg.node(id).kind != CfgNodeKind::Command || cfg.is_spliced_internal(id) {
            continue;
        }
        // arch-2 (`i-3`/`i-4`): an inlined CALL node aggregates its spliced body sites'
        // classifications into one `InlineCall` (the all-or-nothing license + per-site probe
        // sub-records live in `plan`). The body sites are classified with the call's
        // positionals bound (the value plane resolved their argv, `i-2`).
        if let Some(body_sites) = cfg.call_body_sites(id) {
            // A running CALL whose spliced body KILLS mutates when the call runs ⇒ it walls
            // (the InlineCall analogue of the direct-leaf kill; no corpus case yet, cheap to
            // cover). The CALL node is the render/wall unit (`i-3`).
            if body_sites
                .iter()
                .any(|&s| effects.get(s.index()).is_some_and(|cs| bears_kill(cs)))
            {
                kills.insert(id);
            }
            let sites = body_sites
                .iter()
                .map(|&site| InlineSite {
                    node: site,
                    class: classify_site(site.index()),
                })
                .collect();
            out.push((id, SkipClass::InlineCall { sites }));
            continue;
        }
        if bears_kill(cells) {
            kills.insert(id);
            // The single-kill node's killed coordinate — the kill-wall coherence comparand (24E §7).
            if let Some(f) = single_killed_coord(cells) {
                kill_coords.insert(id, f);
            }
        }
        out.push((id, classify_site(i)));
    }
    (
        Carrier { value: out, diags },
        why_diags,
        kills,
        kill_coords,
        backings,
        collapse_narrative,
        invalidators,
    )
}

/// Does this effect cell gen into the reaching-defs — i.e. is it INVALIDATING?
///
/// The gen-side of rule-query-validity, stated once so `reach_transfer` and the
/// invalidator set cannot drift: an `Establishes`/`Kills` gens its fact, an `Opaque`
/// joins ⊤, and `Pure`/`Queries` gen nothing. `is_pristine` is an EMPTY-SET test, so a
/// modeled mutator and an unmodeled opaque invalidate a downstream guard exactly alike —
/// the re-measured `26G` premise (`26H` §4 `lad-modeled-mutating-rhs`).
fn gens_into_reach(cell: &CommandEffect) -> bool {
    match cell {
        CommandEffect::Establishes(_) | CommandEffect::Kills(_) | CommandEffect::Opaque => true,
        CommandEffect::Pure | CommandEffect::Queries(_) => false,
    }
}

/// The killed coordinate of a command's effects (24E §7): `Some(f)` IFF there is EXACTLY one `Kills`
/// cell — the single-kill node, the corpus reality. A multi-kill node has no single comparand
/// (`None` ⇒ no kill-wall coherence, still safe: the coherence check only REFUSES a drifted
/// footprint, never licenses one).
fn single_killed_coord(cells: &[CommandEffect]) -> Option<FactKey> {
    let mut killed = cells.iter().filter_map(|e| match e {
        CommandEffect::Kills(f) => Some(*f),
        _ => None,
    });
    match (killed.next(), killed.next()) {
        (Some(f), None) => Some(f),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg;
    use crate::value::analyze;
    use dorc_core::{KindId, SelectorId};
    use dorc_oracle::predict::lift_predicts;

    /// A REAL `Inconsistent` over `Reach`, and a pristine state to go with it. The perturbation is
    /// genuine and `certify_solution` is the judge — only the SOLVER's answer is faulted
    /// (`302` §6.1's shape). The failing value deliberately carries a `ProvId`, so anything that
    /// tried to move it into the aid plane would be caught by the type system.
    fn a_real_reach_inconsistency(
        arena: &mut dorc_core::ProvArena,
    ) -> (SolveConsistency<Reach>, Reach) {
        use crate::certify::certify_solution;
        use crate::solve::{Direction, Graph, Solution};

        struct SelfLoop;
        impl Graph for SelfLoop {
            fn node_count(&self) -> usize {
                1
            }
            fn succ(&self, _: usize) -> &[usize] {
                &[0]
            }
            fn pred(&self, _: usize) -> &[usize] {
                &[0]
            }
        }
        let cause = arena.leaf(dorc_core::OriginKind::TopCause, None);
        let pristine = Reach::Facts(BTreeSet::new());
        let solution = Solution {
            states: vec![pristine.clone()],
            converged: true,
            rounds: 1,
        };
        let outcome = certify_solution(
            &SelfLoop,
            Direction::Forward,
            std::slice::from_ref(&pristine),
            |_, _| Reach::Top(cause),
            &solution,
        );
        assert!(!outcome.is_consistent(), "the fixture must really fail");
        (outcome, pristine)
    }

    /// `302` §6.8 — THE REACH FLOOR, driven end-to-end by a real `Inconsistent` (the F9
    /// completion). Every shape the classifier can answer — a Members site, an establish, a query
    /// — falls to `MustRun` when the reaching-defs answer did not certify, whatever the states
    /// say. The states handed in are the FAVOURABLE ones (pristine, reachable, a resolved member
    /// family): if certification were merely advisory here, each of these would classify as
    /// something licensable, so the assertions below can only pass because the floor holds.
    #[test]
    fn an_uncertified_reach_floors_every_site_to_must_run() {
        let mut arena = dorc_core::ProvArena::new();
        let (consistency, pristine) = a_real_reach_inconsistency(&mut arena);
        let trust_reach = consistency.is_consistent();
        assert!(!trust_reach);

        let mut interner = Interner::default();
        let fact = FactKey::cell(
            KindId(interner.intern("sm.dorc.Package")),
            EntityRef::Operand(OpaqueToken(interner.intern("nginx"))),
            SelectorId(interner.intern("installed")),
        );
        let effects = vec![
            vec![CommandEffect::Establishes(fact)],
            vec![CommandEffect::Queries(fact)],
            vec![CommandEffect::Establishes(fact)],
        ];
        let member_families = vec![None, None, Some(vec![fact])];
        let reach = vec![pristine.clone(), pristine.clone(), pristine];
        let reachable = vec![true, true, true];
        let mut self_reached = BTreeMap::new();
        self_reached.insert(2usize, true);

        for site in 0..3 {
            assert_eq!(
                classify_one_site(
                    site,
                    &effects,
                    &member_families,
                    &reach,
                    trust_reach,
                    &reachable,
                    &self_reached,
                ),
                SkipClass::MustRun,
                "site {site} must floor: an un-certified reach licenses nothing"
            );
        }

        // The control: the SAME favourable inputs under a trusted answer really would license,
        // so the floor above is doing the work rather than the fixture being inert.
        assert_ne!(
            classify_one_site(
                0,
                &effects,
                &member_families,
                &reach,
                true,
                &reachable,
                &self_reached,
            ),
            SkipClass::MustRun,
            "with certification the same site licenses — the assertions above are not vacuous"
        );
    }

    /// `302` §6.8 — THE SELF-REACH FLOOR, driven by a real `Inconsistent`. The load-bearing half
    /// is that a PRISTINE state does NOT rescue an uncertified solve: pristine-ness is exactly the
    /// condition that would otherwise say "yes", so this is the assertion that distinguishes a
    /// real floor from a coincidence.
    #[test]
    fn an_uncertified_self_reach_answers_false_even_when_pristine() {
        let mut arena = dorc_core::ProvArena::new();
        let (consistency, pristine) = a_real_reach_inconsistency(&mut arena);

        assert!(
            pristine.is_pristine(),
            "the state itself would have said yes"
        );
        assert!(
            !self_reach_answer(&consistency, Some(&pristine)),
            "an un-certified re-solve refuses however good its state looks"
        );
    }

    /// `302` §6.8 — the SELF-REACH ACCOUNT is measured, never manufactured (R4). Two failing
    /// solves contribute their real failing-CHECK totals and their real indices, the solve count
    /// is its own quantity rather than being smuggled in as a check count, and the advisory is a
    /// solver's own report rather than a hard-coded pair.
    #[test]
    fn the_self_reach_account_sums_real_checks_and_keeps_a_real_advisory() {
        let mut arena = dorc_core::ProvArena::new();
        let mut account = SelfReachAccount::default();
        for _ in 0..2 {
            let (consistency, _) = a_real_reach_inconsistency(&mut arena);
            let SolveConsistency::Inconsistent(report) = &consistency else {
                panic!("the fixture must really fail");
            };
            account.record(report);
        }

        assert_eq!(account.solves, 2, "two SOLVES failed");
        assert_eq!(
            account.failing_checks, 2,
            "and their failing CHECKS are summed, not conflated with the solve count"
        );
        assert_eq!(
            account.checks,
            vec![
                dorc_aid::narrative::FailedCheck::Edge { from: 0, to: 0 },
                dorc_aid::narrative::FailedCheck::Edge { from: 0, to: 0 },
            ],
            "the real indices are retained"
        );
        let advisory = account.advisory.expect("a real advisory was retained");
        assert!(advisory.converged);
        assert_eq!(advisory.rounds, 1, "measured from the solve, not invented");
    }

    /// `302` §6.8 — the DEGRADE RECORD carries SCALARS ONLY.
    ///
    /// The load-bearing assertion is the negative one: `Reach` values hold a `ProvId` on their
    /// `Top` variant, and `operands-are-pure-and-capped` forbids arena handles in this plane, so
    /// the narrative must carry failing-check INDICES and counts while the values that failed stay
    /// behind in the in-memory `SolveConsistency`. The verdict is real — a genuine perturbation
    /// judged by the genuine checker (anti-masking).
    #[test]
    fn the_degrade_record_carries_indices_and_never_a_lattice_value() {
        use crate::certify::certify_solution;
        use crate::solve::{Direction, Graph, Solution};

        struct SelfLoop;
        impl Graph for SelfLoop {
            fn node_count(&self) -> usize {
                1
            }
            fn succ(&self, _: usize) -> &[usize] {
                &[0]
            }
            fn pred(&self, _: usize) -> &[usize] {
                &[0]
            }
        }
        let mut arena = dorc_core::ProvArena::new();
        let cause = arena.leaf(dorc_core::OriginKind::TopCause, None);
        let solution = Solution {
            states: vec![Reach::Facts(BTreeSet::new())],
            converged: true,
            rounds: 1,
        };
        // The transferred value is ⊤ (cause-bearing) while the state is pristine, so the edge
        // check fails AND the failing value carries the very `ProvId` this plane may not hold.
        let outcome = certify_solution(
            &SelfLoop,
            Direction::Forward,
            &[Reach::Facts(BTreeSet::new())],
            |_, _| Reach::Top(cause),
            &solution,
        );
        let SolveConsistency::Inconsistent(report) = &outcome else {
            panic!("the fixture must really fail");
        };

        let narrative = consistency_narrative(SolvePass::ReachingDefs, report);
        let dorc_aid::CollapseKind::SolverConsistencyFailure {
            pass,
            operands,
            shown,
            total,
            solves,
            advisory,
        } = narrative.kind()
        else {
            panic!("the narrative must carry the consistency-failure class");
        };
        assert_eq!(*pass, SolvePass::ReachingDefs);
        assert_eq!(*total, 1);
        assert_eq!(*shown, 1);
        assert_eq!(*solves, 1, "a whole-unit pass is one solve");
        assert!(
            advisory.converged,
            "the advisory rides through MEASURED, never manufactured"
        );
        assert_eq!(advisory.rounds, 1);
        assert_eq!(
            operands.kept(),
            &[dorc_aid::narrative::FailedCheck::Edge { from: 0, to: 0 }],
            "the record names the failing check by INDEX"
        );
    }

    /// The shared corpus-shaped check dialect the classify tests lift: an `apt-get`
    /// check (flag-strip → verb → per-verb arm: `update` ⇒ a Singleton `package-index`
    /// annotation; everything else ⇒ a post-verb flag-strip, the single-operand
    /// `package` annotation, and a `[ "$2" = "" ]` guard that refuses a SECOND operand
    /// — `install nginx curl` reaches no probe ⇒ Top ⇒ runs), plus a `systemctl` check
    /// (verb → per-arm probe). Annotation kinds MATCH the effect-map's (`package`,
    /// `package-index`, `service`) so the kind-agreement rule never fires. The probe
    /// bodies are inert placeholders (this round resolves identity only).
    ///
    /// Lifted with the CALLER's interner (`i`), so the [`PredictSet`]'s provider symbol
    /// equals the one `classify` interns from the book's command word (Symbols only
    /// compare across one interner — 204 seam #2).
    const CORPUS_PREDICT_SRC: &str = r#"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      update) probe-fresh : sm.dorc.PkgIndex@fresh ;;
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then probe-pkg "$pkg"; fi ;;
   esac
}
systemctl__predict() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable) probe-enabled "$svc" ;;
      start)  probe-active "$svc" ;;
      disable) probe-enabled "$svc" ;;
   esac
}
command__predict() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1
}
"#;

    /// The interned ids a package-fixture test asserts against. Kept together so a
    /// test reads `s.installed` / `s.nginx` instead of re-interning inline.
    struct Syms {
        package: KindId,
        package_index: KindId,
        installed: SelectorId,
        fresh: SelectorId,
    }

    /// Build (interner, index, syms) modeling the package oracle's effects — now
    /// *including* `apt-get update → (package-index, @fresh)`, the modeled nullary
    /// that the poison-wall fix relies on (`notes/193` §1). `install`/`purge` gate
    /// the `@installed` selector of `package`.
    fn package_setup() -> (Interner, KindIndex, Syms) {
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let package_index = KindId(interner.intern("sm.dorc.PkgIndex"));
        let installed = SelectorId(interner.intern("installed"));
        let fresh = SelectorId(interner.intern("fresh"));
        let apt = ProviderId(interner.intern("apt_get"));
        let install = interner.intern("install");
        let purge = interner.intern("purge");
        let update = interner.intern("update");
        let mut idx = KindIndex::default();
        idx.add_effect(0, apt, install, package, installed, ValueClaim::Establish);
        idx.add_effect(
            0,
            apt,
            purge,
            package,
            installed,
            ValueClaim::EstablishInverted,
        );
        idx.add_effect(0, apt, update, package_index, fresh, ValueClaim::Establish);
        (
            interner,
            idx,
            Syms {
                package,
                package_index,
                installed,
                fresh,
            },
        )
    }

    /// `package:<entity>@installed` — the cell `apt-get install <entity>` gates.
    fn pkg_installed(i: &mut Interner, s: &Syms, entity: &str) -> FactKey {
        FactKey {
            kind: s.package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: s.installed,
            context: Context::HostDefault,
        }
    }

    /// Run the full pipeline on `src` (value-flow + the corpus checks + classify) and
    /// return just the [`SkipClass`]es, in classify order. Everything shares one
    /// interner so the [`PredictSet`]'s provider symbols match the book's command words.
    fn classify_src(src: &str, interner: &mut Interner, idx: &KindIndex) -> Vec<SkipClass> {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, interner);
        let checks = vec![lift_predicts(interner, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        classify(
            &built.value,
            &value,
            &parsed.value,
            idx,
            &checks,
            &VerdictIndex::default(),
            interner,
            &mut arena,
        )
        .value
        .into_iter()
        .map(|(_, c)| c)
        .collect()
    }

    /// Like [`classify_src`] but return the classify-stage diagnostics (the q-2 emit-site
    /// pins): the codes a `$()`/⊤ book discloses.
    fn classify_src_diags(src: &str, interner: &mut Interner, idx: &KindIndex) -> Vec<Diag> {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, interner);
        let checks = vec![lift_predicts(interner, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        classify(
            &built.value,
            &value,
            &parsed.value,
            idx,
            &checks,
            &VerdictIndex::default(),
            interner,
            &mut arena,
        )
        .diags
    }

    fn has_code(diags: &[Diag], code: &str) -> bool {
        diags.iter().any(|d| d.code.slug() == code)
    }

    #[test]
    fn lone_install_is_ambient() {
        // Why: the simplest establish with nothing upstream — must be probe-able
        // (EstablishAmbient), the precondition for ever skipping it.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get install nginx", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))]
        );
    }

    #[test]
    fn upstream_purge_makes_install_written() {
        // Why (note 162 O-1 / break-10, THE wrong-skip): an upstream same-run kill
        // means the resting probe is stale — the install must NOT be treated as
        // ambient/skippable. purge + install gate the SAME (package:nginx@installed)
        // cell, so the kill reaches the establish.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        // purge ⇒ MustRun (a kill, not an elidable establish); install ⇒ Written.
        assert!(classes.contains(&SkipClass::EstablishWritten(pkg_installed(
            &mut i, &s, "nginx"
        ))));
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn kill_coords_records_the_killed_coordinate_per_single_kill_node() {
        // 24E §7 (resid-kill-coherence): `classify_with_why_diags` records each single-kill node's
        // KILLED coordinate in the side-map — the comparand the cli's kill-wall coherence check uses
        // (own-killed-coord ⊆ footprint, closing the gap kill-walls left open). `apt-get purge nginx`
        // kills package:nginx@installed; its node maps to exactly that cell.
        let (mut i, idx, s) = package_setup();
        let parsed = dorc_syntax::parse("apt-get purge nginx\n");
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, &mut i);
        let checks = vec![lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        let (_classes, _why, kills, kill_coords, _backings, _narrative, _invalidators) =
            classify_with_why_diags(
                &built.value,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &VerdictIndex::default(),
                &BTreeMap::new(),
                &crate::erase::ErasedSites::none(),
                &mut i,
                &mut arena,
                &mut BTreeMap::new(),
                &mut BTreeSet::new(),
                &mut CertifierTrip::default(),
                crate::funcenv::LiveDefinitions::unsolved(),
            );
        assert_eq!(kills.len(), 1, "the purge is the sole kill node");
        let node = *kills.iter().next().expect("one kill node");
        let killed = pkg_installed(&mut i, &s, "nginx");
        assert_eq!(
            kill_coords.get(&node),
            Some(&killed),
            "the kill node maps to its killed coordinate package:nginx@installed"
        );
    }

    #[test]
    fn an_opaque_reached_cell_mints_one_fact_merge_disagreement() {
        // C3 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): the collapse MINTS its own
        // narrative (one `Derived` FactMergeDisagreement per Opaque node), never hand-injected.
        let (mut i, idx, _s) = package_setup();
        let checks = vec![lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];

        let collapse = |src: &str, i: &mut Interner| -> Vec<dorc_aid::CollapseNarrative> {
            let parsed = dorc_syntax::parse(src);
            let built = cfg::build(&parsed.value);
            let value = analyze(&built.value, &parsed.value, i);
            let mut arena = dorc_core::ProvArena::new();
            classify_with_why_diags(
                &built.value,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &VerdictIndex::default(),
                &BTreeMap::new(),
                &crate::erase::ErasedSites::none(),
                i,
                &mut arena,
                &mut BTreeMap::new(),
                &mut BTreeSet::new(),
                &mut CertifierTrip::default(),
                crate::funcenv::LiveDefinitions::unsolved(),
            )
            .5
        };

        let narrative = collapse("ufw allow 80/tcp\n", &mut i);
        assert_eq!(
            narrative.len(),
            1,
            "one Opaque node ⇒ one merge-disagreement"
        );
        assert_eq!(narrative[0].tier(), dorc_aid::SpeechAct::Derived);
        assert!(
            matches!(
                narrative[0].kind(),
                dorc_aid::CollapseKind::FactMergeDisagreement { .. }
            ),
            "the collapse minted a FactMergeDisagreement, not a hand-injected record"
        );

        let modeled = collapse("apt-get install nginx\n", &mut i);
        assert!(
            modeled.is_empty(),
            "a modeled-only book has no Opaque collapse and mints no narrative"
        );
    }

    #[test]
    fn the_merge_mint_pairs_with_the_top_cause_mint_in_release_builds() {
        // `289:rul-mint-hardening-package` item 3. `mint_merge_narrative` mirrors `mint_top_causes`
        // by CONSTRUCTION only, and the sibling's `debug_assert` vanishes under `--release`. Order
        // is load-bearing here: the two mints are consumed positionally downstream.
        let effects = |opaque: &[bool]| -> Vec<Vec<CommandEffect>> {
            opaque
                .iter()
                .map(|&is_opaque| {
                    if is_opaque {
                        vec![CommandEffect::Opaque]
                    } else {
                        vec![]
                    }
                })
                .collect()
        };
        for shape in [
            vec![],
            vec![false, false],
            vec![true],
            vec![false, true, false, true, true],
        ] {
            let built = effects(&shape);
            let minted = mint_merge_narrative(&built);
            let opaque_indices: Vec<u32> = shape
                .iter()
                .enumerate()
                .filter_map(|(i, &is_opaque)| is_opaque.then_some(i as u32))
                .collect();
            assert_eq!(
                minted.len(),
                opaque_indices.len(),
                "one narrative per Opaque-bearing node, for {shape:?}"
            );
            let cells: Vec<u32> = minted
                .iter()
                .map(|narrative| match narrative.kind() {
                    dorc_aid::CollapseKind::FactMergeDisagreement { cell, .. } => cell.leaf.0,
                    other => panic!("the merge mint minted {other:?}"),
                })
                .collect();
            assert_eq!(
                cells, opaque_indices,
                "the minted cells are the Opaque-bearing node indices in ascending order, for \
                 {shape:?}"
            );
        }
    }

    #[test]
    fn opaque_upstream_poisons_ambientness() {
        // Why (note 162 O-3, the precision COST being surfaced): a genuinely
        // unrecognized command (`ufw allow` — NO oracle entry) is still Opaque ⇒ ⊤ ⇒
        // it conservatively poisons every downstream fact's ambient-ness. The re-key
        // does NOT rescue an un-oracled neighbor; it rescues a *modeled* nullary
        // (`apt-get update`, the `poison_wall_dies_*` test below). This documents the
        // residual, correct cost so we still feel it.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("ufw allow 80/tcp\napt-get install nginx", &mut i, &idx);
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_)))
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn poison_wall_dies_modeled_update_does_not_poison_install() {
        // THE keystone win (`notes/193` §1 / acceptance §7.2): a modeled `apt-get
        // update` establishes a *distinct cell* (`package-index@fresh`), so it no
        // longer poisons the `apt-get install nginx` below it. Before the re-key,
        // `update` was doubly-unkeyable (no operand, and — pre-modeling — no verb) ⇒
        // Opaque ⇒ Reach::Top ⇒ install forced EstablishWritten. Now it's ambient.
        //
        // This pins the STATIC (classify) tier, which `23Ib-fd10` leaves UNCHANGED: the
        // ambient gate is same-cell only, so a distinct-cell `update` never poisons the
        // install's ambient-ness *here*. The honest-baseline wall lives one tier UP, in
        // `plan::build_plan` (the phased caller, `inv-superposition`): a *running* `update`
        // (diverged) now WALLS the converged install at plan time (silence=wall), demoting
        // its Replace→Run. See the plan-tier pins
        // `running_modeled_mutator_walls_downstream_converged_establish` (the wall fires) and
        // `elided_upstream_mutator_casts_no_shadow` (a converged/elided `update` does not).
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get update\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "modeled `update` (distinct cell) must leave install EstablishAmbient: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no Written: update's cell (package-index@fresh) ≠ install's (package:nginx@installed)"
        );
    }

    #[test]
    fn genuine_same_cell_kill_still_forces_written() {
        // exclusion-check (`notes/193` §7.3): the re-key must NOT over-loosen the
        // ambient gate. A real same-cell kill (`purge nginx; install nginx`, both on
        // package:nginx@installed) must STILL force Written — resting probe is stale.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishWritten(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "same-cell purge must keep install EstablishWritten (no over-loosening): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn distinct_selectors_do_not_discharge_each_other() {
        // The selector regression (`notes/193` §7.4): `systemctl enable nginx` and
        // `systemctl start nginx` gate DIFFERENT selectors of the SAME service:nginx
        // cell (@enabled vs @active). Neither discharges the other — both stay
        // EstablishAmbient (an `is-active` probe must not satisfy an unmet `@enabled`).
        // A flat key (one bit per kind+entity) could not hold this — the second would
        // see the first reach its cell and (mis-)classify Written.
        let mut i = Interner::default();
        let service = KindId(i.intern("service"));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        let systemctl = ProviderId(i.intern("systemctl"));
        let enable = i.intern("enable");
        let start = i.intern("start");
        let mut idx = KindIndex::default();
        idx.add_effect(
            0,
            systemctl,
            enable,
            service,
            enabled,
            ValueClaim::Establish,
        );
        idx.add_effect(0, systemctl, start, service, active, ValueClaim::Establish);

        let classes = classify_src(
            "systemctl enable nginx\nsystemctl start nginx",
            &mut i,
            &idx,
        );
        let enabled_cell = FactKey {
            kind: service,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: enabled,
            context: Context::HostDefault,
        };
        let active_cell = FactKey {
            kind: service,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: active,
            context: Context::HostDefault,
        };
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(enabled_cell)),
            "enable nginx ⇒ service:nginx@enabled, ambient: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(active_cell)),
            "start nginx ⇒ service:nginx@active, ambient (NOT discharged by @enabled): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "distinct selectors ⇒ neither reaches the other's cell ⇒ no Written"
        );
    }

    #[test]
    fn pure_builtin_upstream_does_not_poison_ambientness() {
        // fs-4 (note 16G), the contrast to `opaque_upstream_poisons_ambientness`:
        // the blessed target-state-pure builtins (`:`/`echo`/`cd`/…) touch
        // shell-env/stdout, never an oracle-modeled fact, so they must NOT poison a
        // later establish's ambient-ness. Guards the WHOLE `is_target_state_pure_builtin`
        // allowlist + the Ambient-vs-Written line (the `set`-only end-to-end case does
        // not isolate this); a mis-edit dropping `:`/`echo` would silently re-poison —
        // a wrong-skip surface.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(":\necho hi\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "pure builtins (`:`/`echo`) upstream must keep the install EstablishAmbient: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no spurious Written from a pure-builtin upstream"
        );
    }

    #[test]
    fn trap_at_tip_walls_and_is_never_silently_pure() {
        // `27D` E2 / `276` trap fold-in t1 (the "pin what tip does on a trap-registering book"
        // errand, in-memory home): a top-level `trap` is recognized-but-UNMODELED and must WALL
        // LOUDLY — never be silently accepted as an ordinary/pure command. A silent-ordinary trap
        // is a soundness bug (`276`: "silently-ordinary-command would be a soundness bug; wall is
        // fine"): it would let a converged downstream establish elide PAST an unmodeled cleanup
        // handler. Two teeth:
        //
        // (1) the DIRECT guard — `trap` is NOT in the target-state-pure allowlist. That allowlist
        //     is the ONE place a mis-edit could silently re-classify trap as inert; pin it so
        //     adding "trap" fails HERE, at the soundness surface.
        assert!(
            !is_target_state_pure_builtin("trap"),
            "trap must never be a target-state-pure builtin (silent-ordinary trap is a soundness bug)"
        );
        // (2) the END-TO-END wall — `trap … EXIT` upstream is Opaque ⇒ ⊤ ⇒ it poisons the
        //     downstream converged install's ambient-ness, exactly like the un-oracled `ufw allow`
        //     dual (`opaque_upstream_poisons_ambientness`). The install is EstablishWritten
        //     (walled), never EstablishAmbient (elidable): no silent acceptance, no modeling.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "trap 'rm -f /tmp/lock' EXIT\napt-get install nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "trap at tip walls the downstream install (EstablishWritten): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_))),
            "no EstablishAmbient survives past an unmodeled trap — no silent acceptance"
        );
    }

    #[test]
    fn called_function_body_inlines_to_a_single_call_leaf() {
        // arch-2 (brk-2): a call to a same-file-earlier funcdef is INLINED — the body is
        // spliced at the call, and the CALL is the one render/apply leaf, aggregating the
        // body's effect-bearing sites. `p() { apt-get install nginx; }\np` ⇒ exactly ONE
        // leaf: an `InlineCall` whose single body site is the install's `EstablishAmbient`
        // (the body becomes reachable through the splice — the find-7 un-detaching). The
        // detached DEFINITION body is no longer an independent leaf (`i-3`), so there is no
        // second `MustRun`. (Supersedes the round-20 `detached_function_body_establish_is_
        // not_ambient`: the detached-poison shape is re-homed to the refused-call cases below.)
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { apt-get install nginx; }\np", &mut i, &idx);
        assert_eq!(
            classes.len(),
            1,
            "the call is the only leaf (body is non-leaf)"
        );
        let SkipClass::InlineCall { sites } = &classes[0] else {
            panic!("the call must classify InlineCall, got {:?}", classes[0]);
        };
        assert_eq!(sites.len(), 1, "one effect-bearing body site (the install)");
        assert!(
            matches!(sites[0].class, SkipClass::EstablishAmbient(_)),
            "the body install is EstablishAmbient (reachable via the splice), not Written/\
             MustRun — the call node gens Pure, so it does not poison its own spliced body"
        );
    }

    #[test]
    fn uncalled_function_definition_contributes_no_runnable_leaf() {
        // arch-2: a funcdef DEFINED but never CALLED stays a detached, non-leaf island — its
        // body commands are not independent plan/apply leaves (`i-3`: a definition's body runs
        // only via a call, which would splice it). So `p() { apt-get install nginx; }\necho hi`
        // has exactly ONE leaf — the top-level `echo hi` — and the install does NOT surface as
        // a `MustRun`/`unresolvable-no-probe` leaf of its own. (This re-homes the find-A
        // reachability intent: an unreachable funcdef body advertises no elidable establish.)
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { apt-get install nginx; }\necho hi", &mut i, &idx);
        assert_eq!(classes.len(), 1, "only the top-level `echo hi` is a leaf");
        assert_eq!(
            classes[0],
            SkipClass::MustRun,
            "echo hi is unmodeled ⇒ MustRun"
        );
    }

    #[test]
    fn recursive_call_refuses_inline_and_poisons_the_body() {
        // arch-2 (`i-1`): a recursive call ⊤-rejects the inline (the cycle guard) — the inner
        // `p` stays an ordinary unmodeled command (Opaque). The OUTER call still inlines, but
        // its body now contains that Opaque, which poisons the body install to `MustRun` ⇒ the
        // whole call cannot elide (one non-licensing body leaf runs the call). This pins that
        // the detached-poison semantics survive a refused (recursive) call — the brief's
        // re-homed poison pin.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { p; apt-get install nginx; }\np", &mut i, &idx);
        assert_eq!(classes.len(), 1, "the outer call is the only leaf");
        let SkipClass::InlineCall { sites } = &classes[0] else {
            panic!("the outer call inlines, got {:?}", classes[0]);
        };
        assert!(
            sites.iter().any(|s| s.class == SkipClass::MustRun),
            "the recursion-refused inner `p` (Opaque) poisons the body ⇒ a MustRun body site \
             ⇒ the call will run (the poison-pin is preserved across a refused call)"
        );
    }

    #[test]
    fn command_effect_resolves_operand_singleton_and_top() {
        // Resolve a single-command book through value-flow + the corpus apt check,
        // returning the node's effect cells. (One command ⇒ one Command node.)
        fn eff(src: &str, i: &mut Interner, idx: &KindIndex) -> Vec<CommandEffect> {
            let parsed = dorc_syntax::parse(src);
            let built = cfg::build(&parsed.value);
            let value = analyze(&built.value, &parsed.value, i);
            let checks = vec![lift_predicts(i, CORPUS_PREDICT_SRC).value];
            // A dynamic command word (`$cmd …`) is ⊤-rejected by the parser ⇒ a `Top`
            // CFG node, not a `Command` — classify treats that as Opaque. Mirror it.
            let Some(node) = built
                .value
                .iter()
                .find(|(_, n)| n.kind == CfgNodeKind::Command)
                .map(|(id, _)| id)
            else {
                return vec![CommandEffect::Opaque];
            };
            let mut diags = Vec::new();
            let mut cmdsub_tops = Vec::new();
            let mut backings = BTreeMap::new();
            command_effect(
                idx,
                &checks,
                &VerdictIndex::default(),
                &value.argv_values(node),
                i,
                &mut diags,
                &mut cmdsub_tops,
                None,
                &mut backings,
                &mut None,
                &mut false,
                node,
                crate::funcenv::LiveDefinitions::unsolved(),
            )
        }
        let (mut i, idx, s) = package_setup();
        // One operand ⇒ Operand cell; the flag `-y` is post-verb-stripped by the check.
        let nginx_cell = pkg_installed(&mut i, &s, "nginx");
        assert_eq!(
            eff("apt-get install -y nginx", &mut i, &idx),
            vec![CommandEffect::Establishes(nginx_cell)],
            "the check strips the post-verb -y ⇒ Operand(nginx)@installed"
        );
        // Nullary modeled verb (`update`) ⇒ the check's value-less `package-index`
        // annotation ⇒ Singleton (the poison-wall fix). A flag-only tail stays nullary
        // (the `update` arm ignores the trailing `-y`).
        let pkg_index_fresh = CommandEffect::Establishes(FactKey {
            kind: s.package_index,
            entity: EntityRef::Singleton,
            selector: s.fresh,
            context: Context::HostDefault,
        });
        assert_eq!(
            eff("apt-get update", &mut i, &idx),
            vec![pkg_index_fresh.clone()],
            "nullary modeled verb ⇒ Singleton(package-index@fresh)"
        );
        assert_eq!(
            eff("apt-get update -y", &mut i, &idx),
            vec![pkg_index_fresh],
            "flag-only tail stays nullary ⇒ Singleton"
        );
        // A non-literal operand (`$PKG` ⇒ ⊤ in value-flow) is an UNKNOWN cell, NOT the
        // singleton — else `install $PKG` would be wrongly elidable (priority-1
        // wrong-elision). ⊤ arg ⇒ unresolved site ⇒ Opaque ⇒ run.
        assert_eq!(
            eff("apt-get install $PKG", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "non-literal operand ⇒ ⊤, not Singleton"
        );
        // Multi-operand: the single-`$1` check binds nginx, but its `[ "$2" = "" ]`
        // guard sees the SECOND operand `curl` ⇒ no probe reached ⇒ Top ⇒ Opaque ⇒ run.
        // This is the check's OWN multi-operand refusal (the oracle's code, not the
        // engine): a wrong single-entity elision that would silently drop `curl` is
        // avoided — the safety the deleted engine-side stand-in used to provide.
        assert_eq!(
            eff("apt-get install nginx curl", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "second operand ⇒ the check's guard refuses ⇒ ⊤"
        );
        // Dynamic command name ⇒ ⊤ word0 ⇒ Opaque.
        assert_eq!(
            eff("$cmd install nginx", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "dynamic command name ⇒ ⊤"
        );
        // Unknown verb: `autoclean` ⇒ the check's `*` arm reads `$1` (past end ⇒ Top),
        // and the effect-map has no (apt-get, autoclean) row anyway ⇒ Opaque.
        assert_eq!(
            eff("apt-get autoclean", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "unknown verb ⇒ ⊤"
        );
    }

    /// One command site's `command_effect` outcome under [`verdict_lane_effects`].
    #[derive(Debug, PartialEq, Eq)]
    struct LaneSite {
        cells: Vec<CommandEffect>,
        /// Did this site's establish come from the VERDICT lane (authored cell or auto-cell)?
        keyed: bool,
    }

    /// A whole book's worth of [`LaneSite`]s, sharing ONE interner.
    struct LaneRun {
        sites: Vec<LaneSite>,
        backings: BTreeMap<FactKey, FactBacking>,
        interner: Interner,
    }

    /// Run `command_effect` over EVERY command site of `book` in ONE interner.
    ///
    /// One interner is load-bearing, not tidiness: two `FactKey`s minted in separate interners
    /// carry separate symbol spaces, so comparing them across runs answers nothing (they collide
    /// whenever the intern ORDER matches, which for two structurally identical runs it always
    /// does). Every cross-site assertion below therefore shares this one.
    fn verdict_lane_effects(book: &str, oracle: Option<&str>) -> LaneRun {
        let mut interner = Interner::default();
        let parsed = dorc_syntax::parse(book);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, &mut interner);
        let verdicts = match oracle {
            Some(src) => VerdictIndex::of(&mut interner, &[src]),
            None => VerdictIndex::default(),
        };
        let nodes: Vec<CfgNodeId> = built
            .value
            .iter()
            .filter(|(_, n)| n.kind == CfgNodeKind::Command)
            .map(|(id, _)| id)
            .collect();
        let mut backings = BTreeMap::new();
        let mut sites = Vec::new();
        for node in nodes {
            let argv = value.argv_values(node);
            let mut keyed = false;
            let cells = command_effect(
                &KindIndex::default(),
                &[],
                &verdicts,
                &argv,
                &mut interner,
                &mut Vec::new(),
                &mut Vec::new(),
                None,
                &mut backings,
                &mut None,
                &mut keyed,
                node,
                crate::funcenv::LiveDefinitions::unsolved(),
            );
            sites.push(LaneSite { cells, keyed });
        }
        LaneRun {
            sites,
            backings,
            interner,
        }
    }

    /// [`verdict_lane_effects`] for a single-site book.
    fn verdict_lane_effect(book: &str, oracle: Option<&str>) -> LaneRun {
        let run = verdict_lane_effects(book, oracle);
        assert_eq!(run.sites.len(), 1, "a single-site book");
        run
    }

    #[test]
    fn typeless_floor_auto_cell_mints_only_for_verdict_bearing_providers() {
        // `24L` §2/§7: a would-be-Opaque site mints the auto-cell IFF the provider bears a verdict
        // function; empty index ⇒ the honest Opaque floor (byte-identical to no-oracle).
        let bare = verdict_lane_effect("foobar sync\n", None);
        assert_eq!(
            bare.sites[0].cells,
            vec![CommandEffect::Opaque],
            "no verdict function ⇒ the honest floor (Opaque ⇒ run)"
        );
        assert!(
            !bare.sites[0].keyed,
            "an Opaque site never claims the verdict lane"
        );

        let markless = "foobar__is_converged() { foobar status -- \"$2\" ;}\n";
        let mut run = verdict_lane_effect("foobar sync\n", Some(markless));
        let expect = dorc_core::auto_fact(&mut run.interner, "foobar");
        assert_eq!(
            run.sites[0].cells,
            vec![CommandEffect::Establishes(expect)],
            "a markless verdict-bearing provider mints the per-provider auto-cell (§2)"
        );
        assert!(
            run.sites[0].keyed,
            "the auto-cell site ships its verdict body ⇒ the lane"
        );
    }

    #[test]
    fn an_authored_verdict_coordinate_keys_its_own_cell_and_threads_its_family() {
        // `26H` §3 — the W-B fix: an authored coordinate keys THAT cell, so two sites of one
        // command stop sharing a fact. Still the verdict LANE — no predict answers this cell.
        let oracle = "\
# dorc-lang/v0.2
foobar__is_converged() {
   dst : sm.dorc.File = \"$2\"
   foobar cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content
}
";
        let mut run = verdict_lane_effect("foobar a.conf /etc/a.conf\n", Some(oracle));
        let expect = FactKey::cell(
            KindId(run.interner.intern("sm.dorc.File")),
            EntityRef::Operand(OpaqueToken(run.interner.intern("/etc/a.conf"))),
            SelectorId(run.interner.intern("content")),
        );
        assert_eq!(run.sites[0].cells, vec![CommandEffect::Establishes(expect)]);
        assert!(
            run.sites[0].keyed,
            "an authored verdict cell is still the verdict lane"
        );
        assert!(
            !dorc_core::is_auto_kind(&run.interner, expect.kind),
            "the authored kind is an ordinary kind — which is exactly why the ship \
             discriminator cannot be a kind test"
        );
        let foobar = ProviderId(run.interner.intern(&predict::map_provider_name("foobar")));
        assert_eq!(
            run.backings.get(&expect).and_then(|b| b.family),
            Some(foobar),
            "the verdict-minted fact carries its own family, never a recovered one"
        );
    }

    #[test]
    fn a_second_site_of_one_command_keys_a_different_authored_cell() {
        // Under the auto-cell all three sites were one cell, so any one `cant-tell` de-licensed
        // the rest.
        let oracle = "\
# dorc-lang/v0.2
foobar__is_converged() {
   dst : sm.dorc.File = \"$2\"
   foobar cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content
}
";
        let run = verdict_lane_effects(
            "foobar a.conf /etc/a.conf\nfoobar b.conf /etc/b.conf\nfoobar z.conf /etc/a.conf\n",
            Some(oracle),
        );
        let [a, b, a_again] = run.sites.as_slice() else {
            panic!("three command sites");
        };
        assert_ne!(
            a.cells, b.cells,
            "distinct authored destinations are distinct cells"
        );
        // …and one destination is still ONE cell (`26H` §3.4 — `an-written-stale` rests on it).
        assert_eq!(
            a.cells, a_again.cells,
            "one destination is one cell, whatever the source"
        );
    }

    /// Both lanes live at one site: an oracle authoring a `__predict` that declares the cell AND an
    /// `__is_converged` that may or may not vouch the site's argv. The verdict-primacy tests below
    /// need a REAL effect map and a REAL argparse (the shared `verdict_lane_effects` helper deliberately
    /// has neither), because primacy's whole question is what happens when both answer.
    fn both_lanes_effects(book: &str, oracle: &str) -> LaneRun {
        let mut interner = Interner::default();
        let idx = dorc_oracle::lift(&mut interner, &[oracle]).value;
        let checks = vec![lift_predicts(&mut interner, oracle).value];
        let verdicts = VerdictIndex::of(&mut interner, &[oracle]);
        let parsed = dorc_syntax::parse(book);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, &mut interner);
        let nodes: Vec<CfgNodeId> = built
            .value
            .iter()
            .filter(|(_, n)| n.kind == CfgNodeKind::Command)
            .map(|(id, _)| id)
            .collect();
        let mut backings = BTreeMap::new();
        let mut sites = Vec::new();
        for node in nodes {
            let argv = value.argv_values(node);
            let mut keyed = false;
            let cells = command_effect(
                &idx,
                &checks,
                &verdicts,
                &argv,
                &mut interner,
                &mut Vec::new(),
                &mut Vec::new(),
                None,
                &mut backings,
                &mut None,
                &mut keyed,
                node,
                crate::funcenv::LiveDefinitions::unsolved(),
            );
            sites.push(LaneSite { cells, keyed });
        }
        LaneRun {
            sites,
            backings,
            interner,
        }
    }

    /// One oracle authoring both members over four verbs, so each primacy conjunct gets a book line
    /// rather than a contrived index: `install` establishes and is vouched; `purge` refutes (a Kill);
    /// `refresh` establishes but the verdict's `*` arm declines it; `status` observes (a Query).
    const BOTH_LANES_ORACLE: &str = "\
# dorc-lang/v0.2
apt_get__predict() {
   verb=$1; shift
   pkg : sm.dorc.Package = \"$1\"
   case $verb in
      install) dpkg-query -W \"$pkg\" >/dev/null 2>&1 : sm.dorc.Package:\"$pkg\"@installed ;;
      purge) dpkg-query -W \"$pkg\" >/dev/null 2>&1 :! sm.dorc.Package:\"$pkg\"@installed ;;
      refresh) dpkg-query -W \"$pkg\" >/dev/null 2>&1 : sm.dorc.Package:\"$pkg\"@fresh ;;
      status) dpkg-query -W \"$pkg\" >/dev/null 2>&1 :? sm.dorc.Package:\"$pkg\"@installed ;;
   esac
}
apt_get__is_converged() {
   verb=$1; shift
   case $verb in
      install) aptcheck -q -- \"$1\" ;;
      purge) aptcheck -q -- \"$1\" ;;
      status) aptcheck -q -- \"$1\" ;;
      *) return 2 ;;
   esac
}
";

    /// `28Q` §4 `rul-verdict-primacy-at-the-ship-seat` — a site with a resolvable predict AND a
    /// vouching verdict is VERDICT-lane, which is what sends the verdict body to the ship seat. The
    /// as-built ordering preferred the predict here, so the rc licensing the elision was a
    /// measurement from one author under a permission from another (`28P:fnd-a-split-family-elides-
    /// on-two-authors`); prediction now licenses nothing.
    ///
    /// The second assertion is the half that keeps the corpus still: the cell is NOT re-keyed. The
    /// predict's declared coordinate remains the site's establish, so every downstream invalidation,
    /// backing, and why-coordinate is exactly what it was — only the measuring BODY moved.
    #[test]
    fn a_vouching_verdict_takes_the_measurement_from_a_resolvable_predict() {
        let mut run = both_lanes_effects("apt-get install nginx\n", BOTH_LANES_ORACLE);
        let declared = FactKey::cell(
            KindId(run.interner.intern("sm.dorc.Package")),
            EntityRef::Operand(OpaqueToken(run.interner.intern("nginx"))),
            SelectorId(run.interner.intern("installed")),
        );
        let [site] = run.sites.as_slice() else {
            panic!("one command site, got {:?}", run.sites);
        };
        assert!(
            site.keyed,
            "a vouched, mutation-capable site ships the verdict body: {site:?}"
        );
        assert_eq!(
            site.cells,
            vec![CommandEffect::Establishes(declared)],
            "primacy moves the BODY, never the cell: the predict author's declared coordinate is \
             still the site's establish, and an auto-cell here would be a silently lost measurement"
        );
    }

    /// The verdict's argparse is narrower than the predict's: `refresh` reaches the `*) return 2` arm.
    /// A declined verdict has nothing to measure (`guard23-refusepath-rc0-never-passes`), so primacy
    /// must not claim the lane — the record would key a body the ship seat then refuses to ship, and
    /// the site would lose its check for nothing.
    #[test]
    fn a_declining_verdict_leaves_the_predict_measuring() {
        let mut run = both_lanes_effects("apt-get refresh nginx\n", BOTH_LANES_ORACLE);
        let declared = FactKey::cell(
            KindId(run.interner.intern("sm.dorc.Package")),
            EntityRef::Operand(OpaqueToken(run.interner.intern("nginx"))),
            SelectorId(run.interner.intern("fresh")),
        );
        let [site] = run.sites.as_slice() else {
            panic!("one command site, got {:?}", run.sites);
        };
        assert_eq!(
            site.cells,
            vec![CommandEffect::Establishes(declared)],
            "the predict resolved this argv and declared its cell — the fallback never ran"
        );
        assert!(
            !site.keyed,
            "a declining verdict never takes the measurement: {site:?}"
        );
    }

    /// The other primacy conjunct: elision must be statically AVAILABLE. `classify_one_site` turns
    /// only a lone `Establishes` into an `Establish*` class, so a `Kills` — like the multi-cell and
    /// `Queries` shapes that share this arm — is `MustRun` whatever the probe says. There is no
    /// elision for a verdict to license, so ship-predict-alone stays licensed (`28Q` §8 stage-0) and
    /// the predict's model keeps feeding the concern topology.
    #[test]
    fn an_unelidable_shape_keeps_its_predict_whatever_the_verdict_vouches() {
        let kill = both_lanes_effects("apt-get purge nginx\n", BOTH_LANES_ORACLE);
        let [killed] = kill.sites.as_slice() else {
            panic!("one command site, got {:?}", kill.sites);
        };
        assert!(
            matches!(killed.cells.as_slice(), [CommandEffect::Kills(_)]),
            "the refuting claim is a Kill: {killed:?}"
        );
        assert!(
            !killed.keyed,
            "a Kill classifies MustRun, so the verdict has no elision to license: {killed:?}"
        );

        let query = both_lanes_effects("apt-get status nginx\n", BOTH_LANES_ORACLE);
        let [queried] = query.sites.as_slice() else {
            panic!("one command site, got {:?}", query.sites);
        };
        assert!(
            matches!(queried.cells.as_slice(), [CommandEffect::Queries(_)]),
            "the observe claim is a Query: {queried:?}"
        );
        assert!(
            !queried.keyed,
            "a Query's replacement is read-reproduction, licensed by the fact tier and never by a \
             vouch, so primacy leaves its own model measuring: {queried:?}"
        );
    }

    /// The degrade reason survives to the caller instead of dying at the `Resolution::Top(_) => None`
    /// that used to discard it — `26G:fnd-existence-gate-darkens-oracle`'s "make it loud" half.
    /// Fixtured on a FALLBACK or-list, which stays ⊤ permanently: the contract's `|| return 2` gate
    /// this once used is a supported form now, so it resolves (see the sibling below).
    #[test]
    fn a_degrading_check_reports_its_reason() {
        let src = "# dorc-lang/v0.2\nwombat__predict() {\n   thing : sm.dorc.Thing = \"$1\"\n   wombat query \"$thing\" || wombat sync \"$thing\"\n}\n";
        let mut i = Interner::default();
        let checks = vec![lift_predicts(&mut i, src).value];
        let (effects, reason) = degrade_of("wombat sync\n", &checks, &mut i);
        assert_eq!(effects, vec![CommandEffect::Opaque], "unchanged: still ⊤");
        assert_eq!(reason, Some(TopReason::OrList));
    }

    /// The other half of `26G:fnd-existence-gate-darkens-oracle`: the contract's own existence gate
    /// no longer darkens the oracle at all. The whole finding was that this exact body — the idiom
    /// `oracle-contract.md:103-104` prescribes by name — silently converted a working oracle into a
    /// non-oracle. It resolves now, and the site is probed rather than run blind.
    #[test]
    fn the_contracts_existence_gate_no_longer_darkens_the_oracle() {
        let gated = "# dorc-lang/v0.2\nwombat__predict() {\n   command -v wombat >/dev/null 2>&1 || return 2\n   thing : sm.dorc.Thing = \"$1\"\n   wombat query \"$thing\" : sm.dorc.Thing:\"$thing\"@present\n}\n";
        let bare = "# dorc-lang/v0.2\nwombat__predict() {\n   thing : sm.dorc.Thing = \"$1\"\n   wombat query \"$thing\" : sm.dorc.Thing:\"$thing\"@present\n}\n";
        let mut i = Interner::default();
        let with = vec![lift_predicts(&mut i, gated).value];
        let without = vec![lift_predicts(&mut i, bare).value];
        let (_, gated_reason) = degrade_of("wombat query thing\n", &with, &mut i);
        assert_eq!(gated_reason, None, "the gate no longer degrades the check");
        assert_eq!(
            degrade_of("wombat query thing\n", &with, &mut i),
            degrade_of("wombat query thing\n", &without, &mut i),
            "the gated body behaves exactly as the same body without its gate"
        );
    }

    /// With NO function environment, two competing definitions of one provider answer NOTHING —
    /// identically whichever order they load in (`28Q` §1's withhold floor; the ruling set banked
    /// in `305a` §1).
    ///
    /// This pin has now retired TWO resolution expedients. It first pinned first-in-file-order;
    /// then last-wins, on the reasoning that a shell's live definition is the last one loaded. Both
    /// were load order standing in for an environment nobody had solved, and under true resolution
    /// neither is available: `dorc_core::answering_file`'s `NoOpinion` arm answers from a SOLE
    /// candidate and withholds on plural ones, because picking between two authors by load order is
    /// load-order-as-trust-adjudicator — the fence `28K` §6 permanently refuses.
    ///
    /// The ORDER SYMMETRY is what makes this a stronger pin than either predecessor: an
    /// order-dependent expectation can only ever assert which expedient is in force, while this one
    /// asserts that no expedient is. Withholding also fails in the safe direction — the site falls
    /// to Opaque ⇒ `MustRun` ⇒ run, and what is lost is an aid-plane note, never a license.
    #[test]
    fn competing_definitions_without_an_environment_withhold_in_either_order() {
        let or_list = "# dorc-lang/v0.2\nwombat__predict() {\n   wombat query \"$1\" || wombat sync \"$1\"\n}\n";
        let pipeline = "# dorc-lang/v0.2\nwombat__predict() {\n   wombat list | wombat count\n}\n";
        let mut i = Interner::default();
        let a = lift_predicts(&mut i, or_list).value;
        let b = lift_predicts(&mut i, pipeline).value;
        let (or_list_first, _) = degrade_of("wombat sync\n", &[a.clone(), b.clone()], &mut i);
        let (pipeline_first, _) = degrade_of("wombat sync\n", &[b.clone(), a.clone()], &mut i);
        assert_eq!(
            or_list_first,
            vec![CommandEffect::Opaque],
            "no definition answers, so the site runs"
        );
        assert_eq!(
            pipeline_first, or_list_first,
            "and the load order cannot change that — neither body is reachable as an answer"
        );
        let (_, or_list_reason) = degrade_of("wombat sync\n", &[a.clone(), b.clone()], &mut i);
        let (_, pipeline_reason) = degrade_of("wombat sync\n", &[b, a], &mut i);
        assert_eq!(
            or_list_reason, None,
            "and neither body's give-up reason surfaces: attributing one would name a body that \
             was never selected (`271:rul-sin-ordering`)"
        );
        assert_eq!(pipeline_reason, None, "symmetrically, in the other order");
    }

    /// The single-definition case still answers, which is what keeps the withhold above a statement
    /// about PLURALITY rather than about the no-environment posture as such. Without this, the pin
    /// above would pass just as well if unsolved environments withheld unconditionally — walling
    /// every hand-built index in the workspace.
    #[test]
    fn a_sole_definition_without_an_environment_still_answers() {
        let or_list = "# dorc-lang/v0.2\nwombat__predict() {\n   wombat query \"$1\" || wombat sync \"$1\"\n}\n";
        let mut i = Interner::default();
        let sole = lift_predicts(&mut i, or_list).value;
        let (_, reason) = degrade_of("wombat sync\n", &[sole], &mut i);
        assert_eq!(reason, Some(TopReason::OrList));
    }

    /// Run `command_effect` over a one-command book; return its effects plus the degrade reason.
    fn degrade_of(
        src: &str,
        checks: &[PredictSet],
        i: &mut Interner,
    ) -> (Vec<CommandEffect>, Option<TopReason>) {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, i);
        let node = built
            .value
            .iter()
            .find(|(_, n)| n.kind == CfgNodeKind::Command)
            .map(|(id, _)| id)
            .expect("the book's one command node");
        let mut reason = None;
        let effects = command_effect(
            &KindIndex::default(),
            checks,
            &VerdictIndex::default(),
            &value.argv_values(node),
            i,
            &mut Vec::new(),
            &mut Vec::new(),
            None,
            &mut BTreeMap::new(),
            &mut reason,
            &mut false,
            node,
            crate::funcenv::LiveDefinitions::unsolved(),
        );
        (effects, reason)
    }

    #[test]
    fn multi_operand_is_not_wrongly_elided() {
        // The kFAIL-perform guard the new check preserves (the deleted stand-in's
        // multi-operand refusal): `apt-get install nginx curl` must NOT resolve to a single-entity
        // cell (which could elide, silently dropping curl). It stays MustRun.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("apt-get install nginx curl", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::MustRun],
            "multi-operand install ⇒ MustRun (no single-entity wrong-elision)"
        );
    }

    #[test]
    fn opaque_var_operand_is_top_when_unresolved_but_resolves_when_flowed() {
        // The value-plane's reach: a command-prefix/assigned operand. `PKG=nginx;
        // apt-get install -y "$PKG"` — value-flow resolves `"$PKG"` to nginx, so the
        // site is fully concrete and the check resolves entity=nginx (EstablishAmbient).
        // This is the value-flow win the old engine-side stand-in (which saw `"$PKG"`
        // as a non-literal operand ⇒ Opaque) could not reach. Contrast: an UNASSIGNED
        // `"$X"` stays ⊤ ⇒ Opaque. (GOLDEN: `exec-opaque-var` flips elsewhere — flagged.)
        let (mut i, idx, s) = package_setup();
        // The bare `PKG=nginx` assignment is also a leaf (MustRun); the install is the
        // one we assert resolved.
        let flowed = classify_src("PKG=nginx\napt-get install -y \"$PKG\"", &mut i, &idx);
        assert!(
            flowed.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "value-flow resolves the assigned operand ⇒ the install is identity-resolved: {flowed:?}"
        );
        let unresolved = classify_src("apt-get install -y \"$X\"", &mut i, &idx);
        assert_eq!(
            unresolved,
            vec![SkipClass::MustRun],
            "an unassigned ⊤ operand ⇒ unresolved site ⇒ MustRun"
        );
    }

    // --- task-D2: the Query effect-class + rule-query-validity (202 §2 / 205 §2) ---

    /// `tool:<entity>@present` — the cell `command -v <entity>` queries.
    fn tool_present(i: &mut Interner, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("present")),
            context: Context::HostDefault,
        }
    }

    /// A package index (install/purge/update) PLUS a read-only `command '' query
    /// present` guard on `tool` (the canonical `command -v` Query). Threads a
    /// caller-provided interner so the Query tests share one across index-build +
    /// classify + assertions.
    fn package_and_query_index(i: &mut Interner) -> KindIndex {
        let package = KindId(i.intern("package"));
        let package_index = KindId(i.intern("sm.dorc.PkgIndex"));
        let installed = SelectorId(i.intern("installed"));
        let fresh = SelectorId(i.intern("fresh"));
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let purge = i.intern("purge");
        let update = i.intern("update");
        let tool = KindId(i.intern("tool"));
        let present = SelectorId(i.intern("present"));
        let command = ProviderId(i.intern("command"));
        let eps = empty_verb(i);
        let mut idx = KindIndex::default();
        idx.add_effect(0, apt, install, package, installed, ValueClaim::Establish);
        idx.add_effect(
            0,
            apt,
            purge,
            package,
            installed,
            ValueClaim::EstablishInverted,
        );
        idx.add_effect(0, apt, update, package_index, fresh, ValueClaim::Establish);
        idx.add_effect(0, command, eps, tool, present, ValueClaim::Observe);
        idx
    }

    #[test]
    fn lone_query_guard_is_resolvable_and_valid() {
        // The simplest Query: `command -v nginx` with nothing upstream ⇒
        // QueryResolvable + valid (pristine prefix — no write-or-unknown reached it).
        // This is the headline guard, classified as a first-class read-only Query.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx", &mut i, &idx);
        let fact = tool_present(&mut i, "nginx");
        assert_eq!(
            classes,
            vec![SkipClass::QueryResolvable { fact, valid: true }],
            "a lone Query guard is resolvable + valid: {classes:?}"
        );
    }

    #[test]
    fn query_does_not_poison_downstream_establish() {
        // A Query READS, it does not write — so an upstream `command -v nginx` must NOT
        // poison a downstream `apt-get install nginx`'s ambient-ness (contrast an Opaque
        // neighbour, which does). The install stays EstablishAmbient. This is the
        // gen-side of task-D2 (a Query gens nothing into Reach).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx\napt-get install -y nginx", &mut i, &idx);
        let install = pkg_installed_with(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(install)),
            "an upstream Query must NOT poison the install (it reads, doesn't write): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no Written: a Query gens nothing into Reach"
        );
    }

    #[test]
    fn query_after_query_stays_valid_st3() {
        // st-3 (20A §4): an upstream QUERY does not invalidate a downstream Query (reads
        // don't write — the guard-stack idiom keeps all its folds). Two `command -v`
        // guards: BOTH stay valid. A pure builtin between them likewise doesn't
        // invalidate (it gens nothing).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx\n:\ncommand -v curl", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        let curl = tool_present(&mut i, "curl");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "first Query valid: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: curl,
                valid: true
            }),
            "second Query STILL valid — an upstream Query (+ pure `:`) does not invalidate (st-3): {classes:?}"
        );
    }

    #[test]
    fn query_after_mutator_is_invalid() {
        // rule-query-validity (205 §2): an upstream MUTATOR (a write) invalidates a
        // downstream Query — its resting rc is now stale. `apt-get install curl`
        // (establishes package:curl@installed) ⇒ the `command -v nginx` guard below it
        // is QueryResolvable but INVALID (valid: false). The cell mutated is irrelevant
        // (ANY write invalidates — the pristine-prefix rule, not same-cell).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("apt-get install -y curl\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a Query below a mutator is INVALID (stale resting rc — pristine-prefix fails): {classes:?}"
        );
    }

    #[test]
    fn query_after_opaque_is_invalid() {
        // rule-query-validity, the Opaque arm: an upstream un-oracled (Opaque) command
        // ⇒ Reach::Top ⇒ the downstream Query is INVALID (an unknown command may have
        // changed anything). `ufw allow 80/tcp` is un-oracled here ⇒ Opaque.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("ufw allow 80/tcp\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a Query below an Opaque command is INVALID (⊤ reached it): {classes:?}"
        );
    }

    #[test]
    fn opaque_pipe_predecessor_invalidates_downstream_query() {
        // THE pipe-guard gap (round-25 field-trial flagship `otelcol --version | grep -q V
        // || curl … | tar xz`): rule-query-validity fires through a PIPE, not only across
        // sequential lines. The check-pipeline's GOVERNING status is its LAST stage; the
        // admin's own tool sits in the NON-last stage, whose stdout is consumed and whose
        // opacity (un-oracled) reaches the last-stage Query as ⊤ ⇒ the Query is INVALID
        // (valid: false) ⇒ its resting rc is withheld from the fold ⇒ the `||` never folds ⇒
        // the whole line runs. `ufw allow 80/tcp` stands in for the opaque first stage
        // (un-oracled here); `command -v nginx` for the last-stage Query (the corpus test
        // index has no `grep` kind — the flagship's real last stage is `grep -q`, same
        // mechanism). Contrast `opaque_upstream` is the SEQUENTIAL sibling above; this pins
        // that the pipe predecessor is upstream too (cfg lowers a pipeline as a stage
        // sequence, so the first stage reaches the last in reaching-defs).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("ufw allow 80/tcp | command -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a last-stage Query with an OPAQUE pipe-predecessor is INVALID (⊤ reached it \
             through the pipe — the pipe-guard block): {classes:?}"
        );
    }

    #[test]
    fn query_pipe_predecessor_keeps_downstream_query_valid() {
        // The isolation control for the pipe-guard gap: the blocker is the first stage's
        // OPACITY, not the pipe structure. A read-only Query pipe-predecessor gens nothing
        // into Reach, so the last-stage Query stays pristine ⇒ valid: true (it WOULD fold).
        // Mirrors `query_after_query_stays_valid_st3` but across a `|` instead of a newline.
        // (Empirically the whole shape folds with a non-opaque first stage — a pure `true |
        // grep -q X` or a modeled `dpkg -s x | grep -q y` both replace+omit; the flagship
        // fails ONLY because `otelcol` is un-oracled.)
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v curl | command -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "a last-stage Query with a non-mutating Query pipe-predecessor stays VALID — the \
             pipe alone does not invalidate; only opacity/mutation upstream does: {classes:?}"
        );
    }

    #[test]
    fn observe_singleton_pipe_predecessor_keeps_downstream_query_valid() {
        // 24J §2 — the vouched-case mirror of `query_pipe_predecessor_keeps_downstream_query_valid`.
        // The flagship's NON-last stage is a DESCRIBED read-only tool (`otelcol --version`, an Observe
        // SINGLETON — no operand, unlike `command -v`'s operand-Observe). It must ALSO keep the
        // last-stage Query valid through the pipe (an Observe gens nothing, singleton or operand),
        // which is exactly what makes `otelcol --version | grep -q V` an all-vouched-read-only
        // CONNECTED check-pipe. Self-contained dialect (no CORPUS churn); `command -v nginx` stands in
        // for grep as the last-stage Query (the corpus test index has no `grep` kind — same mechanism
        // as the flagship note). Contrast the Opaque-predecessor pin above (which INVALIDATES).
        let mut i = Interner::default();
        let dialect = "\
otelcol__predict() {
   case $1 in
      --version) v : otelcol; otelcol --version >/dev/null 2>&1 :? otelcol@v0155 ;;
   esac
}
command__predict() {
   case $1 in -v) shift ;; esac
   tool : tool = \"$1\"
   command -v -- \"$tool\" >/dev/null 2>&1 :? tool:\"$tool\"@present
}
";
        // LIFT the effect-map from the SAME dialect (as the cli does) so the index + the predict
        // agree by construction — a hand-built cell whose verb/kind diverges from the predict's marks
        // resolves to MustRun (the failure this authoring avoids).
        let idx = dorc_oracle::lift(&mut i, &[dialect]).value;
        let checks = vec![lift_predicts(&mut i, dialect).value];

        let parsed = dorc_syntax::parse("otelcol --version | command -v nginx");
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, &mut i);
        let mut arena = dorc_core::ProvArena::new();
        let classes = classify(
            &built.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &VerdictIndex::default(),
            &mut i,
            &mut arena,
        )
        .value;
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.iter().any(|(_, c)| c
                == &SkipClass::QueryResolvable {
                    fact: nginx,
                    valid: true
                }),
            "a described read-only (Observe-singleton) pipe-predecessor keeps the downstream Query \
             VALID — the 24J connected-pipe precondition: {classes:?}"
        );
    }

    // --- y-1 (redirect-effects, `21F` imp-1): a write-redirect is a file-write WRITER ----

    /// `file:<path>@written` — the cell a write-redirect (`>`/`>>`) to `path` gens.
    fn file_written(i: &mut Interner, path: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("file")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(path))),
            selector: SelectorId(i.intern("written")),
            context: Context::HostDefault,
        }
    }

    /// The y-1 file-write cell is built by `file_write_cell` from the resolved path; the
    /// test-side `file_written` must reproduce its exact shape (kind `file`, entity = the
    /// path operand, selector `written`), or every other y-1 pin is asserting the wrong cell.
    #[test]
    fn file_write_cell_has_the_declared_shape() {
        let mut i = Interner::default();
        let path = i.intern("/etc/app.conf");
        assert_eq!(
            file_write_cell(path, &mut i),
            file_written(&mut i, "/etc/app.conf"),
            "the gen'd file-write cell shape must match the documented (file, path, written)"
        );
    }

    #[test]
    fn write_redirect_invalidates_downstream_query() {
        // THE `21F` imp-1 regression pin (the reason y-1 exists). A write-redirect to a real
        // sink is a WRITER: `: > /etc/marker` gens `file:/etc/marker@written`, so the
        // downstream `command -v nginx` guard fails rule-query-validity (its resting rc is now
        // stale — a file the book just wrote sits between entry and the guard). Pre-y-1 the
        // redirect was invisibly Pure ⇒ the guard read `valid: true` ⇒ a stale-guard fold
        // MANUFACTURED a wrong-elision (the imp-1 hole). Same shape as
        // `query_after_mutator_is_invalid`, but the invalidator is a redirect, not an oracled
        // mutator.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > /etc/marker\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a write-redirect upstream must invalidate the downstream Query (imp-1 pin): {classes:?}"
        );
    }

    #[test]
    fn append_redirect_also_invalidates_query() {
        // Append vs truncate are BOTH write-shaped (the charter unit pin): `printf x >> f`
        // (append) invalidates exactly as `>` (truncate) does. `printf` is a blessed-pure
        // builtin, so WITHOUT y-1 the `>> f` would be the only write — and it was invisible
        // (the precise imp-1 strawman: `set -e; printf 'x' >> f; grep ... f || mutator`).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(
            "printf 'x' >> /etc/app.conf\ncommand -v nginx",
            &mut i,
            &idx,
        );
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "an APPEND (`>>`) redirect is write-shaped too ⇒ invalidates the Query: {classes:?}"
        );
    }

    #[test]
    fn var_resolved_redirect_target_invalidates_query() {
        // The value-plane integration the charter emphasizes (y1-a: "resolve the target word
        // through the EXISTING value plane"): a redirect target is an ordinary expansion, so
        // `logfile=app.log; : > "$logfile"` resolves `$logfile` ⇒ `app.log` ⇒ gens
        // `file:app.log@written` ⇒ invalidates the downstream Query. Constant propagation
        // composes with the redirect-target resolution (shared `resolve_recipe` machinery).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(
            "logfile=app.log\n: > \"$logfile\"\ncommand -v nginx",
            &mut i,
            &idx,
        );
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a var-resolved redirect target (via the value plane) invalidates the Query: {classes:?}"
        );
    }

    #[test]
    fn var_resolved_redirect_target_gens_concrete_cell_not_top() {
        // Companion to var_resolved_redirect_target_invalidates_query (21H §9 correction): that
        // test's lone `valid: false` ALSO passes if `$logfile` had degraded to ⊤ — a ⊤ target
        // invalidates the Query too (top_target_redirect_poisons_downstream_query). The cheap
        // discriminator is the disclosure: the resolved-literal arm gens a CONCRETE file cell and
        // fires NO `dq-redir-target-top` (only the ⊤ arm discloses — pinned by
        // top_target_redirect_discloses_not_silent). Pinning its ABSENCE here proves the value
        // plane RESOLVED `$logfile` ⇒ `app.log`, never that it silently collapsed to ⊤.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let diags = classify_src_diags(
            "logfile=app.log\n: > \"$logfile\"\ncommand -v nginx",
            &mut i,
            &idx,
        );
        assert!(
            !has_code(&diags, "redir-target-top"),
            "a var-RESOLVED redirect target takes the concrete-cell path (no ⊤ disclosure): {diags:?}"
        );
    }

    #[test]
    fn devnull_redirect_does_not_invalidate_query() {
        // The exemption set (the charter unit pin): `>/dev/null` is the discard sink — NOT a
        // file-write effect — so it gens no cell and a downstream Query stays valid. This is
        // the `exec-devnull-exempt` mechanism at the validity layer: a redirect to the bit
        // bucket must not poison.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > /dev/null\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "`>/dev/null` is exempt (the discard sink) ⇒ the Query stays valid: {classes:?}"
        );
    }

    #[test]
    fn fd_dup_redirect_does_not_invalidate_query() {
        // The exemption set, the fd-dup arm: `2>&1` is a file-descriptor dup, NOT a
        // file-write — so it gens no cell and a downstream Query stays valid. (`2>&1` stays
        // exempt per the existing devnull/dup vocabulary — charter y1-a.)
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("echo hi 2>&1\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "`2>&1` is an fd-dup, not a file-write ⇒ the Query stays valid: {classes:?}"
        );
    }

    #[test]
    fn write_redirect_poisons_downstream_establish_ambientness() {
        // A write-redirect is a WRITER, so — like any Opaque/mutator — it makes a downstream
        // establish non-ambient when... actually NO: a `file` cell is a DIFFERENT cell from
        // `package:nginx@installed`, so by the poison-wall keystone it must NOT poison the
        // install (distinct cells don't cross-poison). The install stays EstablishAmbient.
        // This pins that the file-cell is a real per-path cell (not a ⊤ that havocs): only
        // the SAME cell (or an Opaque ⊤) invalidates ambient-ness.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(": > /etc/marker\napt-get install -y nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a file-write cell is a distinct cell ⇒ it must NOT poison a package install (keystone): {classes:?}"
        );
    }

    #[test]
    fn top_target_redirect_poisons_downstream_query() {
        // A ⊤ (dynamic) redirect target joins ⊤ (the Opaque-poison shape, charter y1-a): the
        // path is unresolved so no per-path cell can be keyed, and a downstream Query is
        // INVALID (an unknown file — possibly anything — was written). `> "$dyn"` where `$dyn`
        // is never assigned ⇒ the target is ⊤. (The disclosure `dq-redir-target-top` fires;
        // the validity-invalidation is the behavior pinned here.)
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > \"$dyn\"\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a ⊤-target redirect joins ⊤ ⇒ invalidates the downstream Query: {classes:?}"
        );
    }

    #[test]
    fn top_target_redirect_discloses_not_silent() {
        // The ⊤-target redirect disclosure (`dq-redir-target-top`, the redirect-effects analog
        // of `dq-cmdsub-operand-top`): a write to a dynamic target is surfaced, never silent.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let diags = classify_src_diags(": > \"$dyn\"\ncommand -v nginx", &mut i, &idx);
        assert!(
            has_code(&diags, "redir-target-top"),
            "a ⊤-target write-redirect must disclose redir-target-top: {diags:?}"
        );
    }

    #[test]
    fn blessed_pure_colon_with_write_redirect_invalidates_downstream_query() {
        // fix-4(a) regression pin (y-1): `: > f` is a blessed-pure colon builtin carrying a
        // write-redirect. The `:` command itself gens nothing, but the `> f` Redir node gens
        // `file:f@written` into reaching-defs — so a downstream Query reading the just-written
        // file is non-pristine ⇒ INVALID. Pins that the redirect's file-write effect is NOT
        // masked by the blessed-pure command word (the precise imp-1 hazard: the write is on the
        // redirect, not the verb). Mirrors `write_redirect_invalidates_downstream_query`, kept
        // as its own pin so the colon-specific shape has an explicit guard.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > /etc/app.conf\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "`: > f` (blessed-pure colon + write-redirect) gens the file-write cell ⇒ the \
             downstream Query is INVALID (the write is on the redirect, not the verb): {classes:?}"
        );
    }

    #[test]
    fn bare_redirect_empty_argv_invalidates_downstream_query() {
        // fix-4(b) regression pin (y-1): a BARE `> f` (an empty-argv command — no command word,
        // only a write-redirect) is still a file-write WRITER. The empty-argv command node is a
        // `MustRun` (no verb to classify), but the `> f` Redir node gens `file:f@written` into
        // reaching-defs — so a downstream Query is non-pristine ⇒ INVALID. Pins that the
        // redirect-effect is seen even with NO command word (the redirect runs in the current
        // shell, truncating the file). The novel shape the other y-1 pins (`:`/`printf`/`echo`
        // prefixes) do not cover.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("> /etc/app.conf\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a bare `> f` (empty-argv command + write-redirect) gens the file-write cell ⇒ the \
             downstream Query is INVALID (the redirect writes with no command word): {classes:?}"
        );
    }

    // --- task-L1 (`209` brk-1): reaching-defs over the loop back-edge -------------

    #[test]
    fn post_loop_install_ambient_when_loop_body_is_pure() {
        // THE brk-1 value-unlock at the EFFECT layer: a PURE loop body (`echo` only —
        // gens nothing) does NOT poison a converged install BELOW the loop. The
        // reaching-defs back-edge carries no write out of the loop, so the post-loop
        // install stays EstablishAmbient (elidable). Pre-L1 the loop was a ⊤ node whose
        // havoc + ⊤-containment killed this — the poison the L1 lowering removes.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for f in a b; do echo \"$f\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a pure loop body must NOT poison the post-loop install: {classes:?}"
        );
    }

    #[test]
    fn opaque_in_loop_body_poisons_post_loop_install() {
        // The honest residual cost (exclusion-check, the other cell): an OPAQUE command
        // inside the loop body propagates Reach::Top across the back-edge and OUT to the
        // post-loop install ⇒ it is forced EstablishWritten (runs). A loop is not magic —
        // an un-oracled body command poisons exactly as it would in straight-line code.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "for f in a b; do ufw allow \"$f\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "an Opaque loop-body command poisons the post-loop install (back-edge ⊤): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_))),
            "no ambient install survives the in-loop Opaque"
        );
    }

    #[test]
    fn classify_converges_on_nested_loop_back_edges() {
        // The reaching-defs fixpoint must converge on a NESTED loop (two back-edges).
        // `classify` carries a `debug_assert!(reach.converged)`; a non-converging
        // reaching-defs would trip it (or, in release, fold every establish to MustRun
        // via `trust_reach`). Drive a nested loop with a body establish and assert we get
        // a classification back at all (the post-loop install) — convergence implied.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "for o in a b; do for p in c d; do apt-get install -y \"$p\"; done; done\nsystemctl reload nginx",
            &mut i,
            &idx,
        );
        assert!(
            !classes.is_empty(),
            "classify returns (reaching-defs converged on the nested back-edges): {classes:?}"
        );
    }

    /// `package:<entity>@installed` via a shared interner (sibling of `pkg_installed`
    /// for the Query tests that build their own index inline).
    fn pkg_installed_with(i: &mut Interner, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("installed")),
            context: Context::HostDefault,
        }
    }

    // --- task-L2 item-2 (`209` brk-1(b)): the in-loop Members fact-family ----------

    #[test]
    fn in_loop_members_site_classifies_as_establish_members_family() {
        // THE item-2 unlock: `for pkg in nginx curl; do apt-get install -y "$pkg"; done` ⇒
        // the body install is `EstablishMembers` carrying the per-member family
        // [package:nginx@installed, package:curl@installed], in list order. Each member
        // resolved through the oracle check exactly as a straight-line install would.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            r#"for pkg in nginx curl; do apt-get install -y "$pkg"; done"#,
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        let curl = pkg_installed(&mut i, &s, "curl");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, curl],
                self_reached: true,
            }),
            "the in-loop Members install resolves a per-member fact-family in list order, self-reached: {classes:?}"
        );
    }

    #[test]
    fn members_family_keeps_duplicate_cells() {
        // Dups are kept (dash iterates them): `for p in nginx nginx` ⇒ a two-element family
        // of the SAME cell. (item-1's no-dedup carried into the cell family.)
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            r#"for p in nginx nginx; do apt-get install -y "$p"; done"#,
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, nginx],
                self_reached: true,
            }),
            "duplicate members ⇒ duplicate cells in the family: {classes:?}"
        );
    }

    #[test]
    fn members_family_all_or_nothing_one_member_unresolvable_tops() {
        // ALL-OR-NOTHING (item-2): if ANY member fails to resolve to a single establish,
        // the WHOLE site is NOT a family. `for p in nginx "a b"; do apt-get install -y $p;
        // done` — the list is two eligible single-concrete members (`nginx`, `a b`), but the
        // body's UNQUOTED `$p` field-splits each member's value: `nginx` ⇒ one operand
        // (resolves to package:nginx@installed), while `a b` ⇒ TWO operands (`apt-get
        // install -y a b`) ⇒ the check's `[ "$2" = "" ]` guard refuses ⇒ that member is
        // Opaque. One member unresolvable ⇒ NO family (not a partial [nginx-only] one) ⇒
        // the in-loop site falls to the single-cell Flat path ⇒ MustRun (the floor).
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            r#"for p in nginx "a b"; do apt-get install -y $p; done"#,
            &mut i,
            &idx,
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishMembers { .. })),
            "one unresolvable member ⇒ NO family (all-or-nothing), falls to MustRun: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::MustRun),
            "the all-or-nothing failure floors the in-loop site to MustRun: {classes:?}"
        );
    }

    #[test]
    fn members_family_gens_member_cells_not_opaque_post_loop_stays_clean() {
        // The reaching-defs consequence (load-bearing for item-3's self-reach): a resolved
        // Members site gens its MEMBER cells into Reach, NOT Opaque. So a post-loop install
        // of a DISTINCT package is NOT poisoned to Written by the loop. `for pkg in nginx
        // curl; do apt-get install -y "$pkg"; done; apt-get install -y redis` ⇒ the redis
        // install stays EstablishAmbient (the loop genned nginx/curl cells, not ⊤).
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\napt-get install -y redis",
            &mut i,
            &idx,
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "redis"
            ))),
            "a resolved Members loop gens member cells (not ⊤) ⇒ a distinct post-loop install stays ambient: {classes:?}"
        );
    }

    #[test]
    fn members_family_poisons_post_loop_same_cell() {
        // Exclusion-check (the other cell): a post-loop install of a cell the LOOP
        // establishes IS reached by the loop's member-establish ⇒ EstablishWritten (stale
        // resting probe). `for pkg in nginx curl; …; apt-get install -y nginx` ⇒ the
        // post-loop nginx install sees the loop's nginx member-cell upstream ⇒ Written.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        // The post-loop nginx is Written (a member-cell reached it); curl was never
        // post-installed. No EstablishAmbient for nginx.
        assert!(
            classes.contains(&SkipClass::EstablishWritten(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a post-loop install of a loop-member cell is Written (the member-establish reaches it): {classes:?}"
        );
    }

    #[test]
    fn members_self_reach_broken_by_pre_loop_writer() {
        // item-3(b) self-reach FALSE (the `loop-member-external-writer-runs` core): a
        // PRE-LOOP `apt-get purge curl` kills `package:curl@installed` — a member cell. That
        // write reaches the in-loop install via the in-state, so the site's in-state is NOT
        // a subset of its own family ⇒ `self_reached: false`. The family still resolves
        // (item-2); only the self-reach bit flips ⇒ the license (item-3) will refuse.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "apt-get purge curl\nfor pkg in nginx curl; do apt-get install -y \"$pkg\"; done",
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        let curl = pkg_installed(&mut i, &s, "curl");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, curl],
                self_reached: false,
            }),
            "a pre-loop purge of a member cell breaks self-reach (in-state ⊄ family): {classes:?}"
        );
    }

    #[test]
    fn members_self_reach_broken_by_opaque_in_body() {
        // item-3(b) self-reach FALSE via an in-loop Opaque sibling: `for pkg in nginx curl;
        // do ufw allow "$pkg"; apt-get install -y "$pkg"; done` — the un-oracled `ufw allow`
        // is Opaque ⇒ Reach::Top reaches the install ⇒ `self_reached: false`. (The install's
        // family still resolves; the sibling Opaque is the non-self writer.)
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for pkg in nginx curl; do ufw allow \"$pkg\"; apt-get install -y \"$pkg\"; done",
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        let curl = pkg_installed(&mut i, &s, "curl");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, curl],
                self_reached: false,
            }),
            "an in-loop Opaque sibling (⊤) breaks self-reach: {classes:?}"
        );
    }

    // ---- q-2: the `$()` ⊤-diagnostics floor (find-3 no-silent-phantoms) ----

    #[test]
    fn cmdsub_operand_top_disclosed_not_silent() {
        // Why (219 q-1.f silent-2, the find-3 violation q-2 closes): a `$()`-captured operand
        // forces the command Opaque, and that degradation used to be SILENT. The disclosure must
        // now fire (`dq-cmdsub-operand-top`). `PKG=$(cat /etc/pkg)` ⇒ `$PKG` is ⊤ ⇒ the install's
        // operand is ⊤ ⇒ Opaque + the Note.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags(
            "PKG=$(cat /etc/pkg)\napt-get install -y \"$PKG\"",
            &mut i,
            &idx,
        );
        assert!(
            has_code(&diags, "cmdsub-operand-top"),
            "a ⊤ operand must disclose cmdsub-operand-top, never silently Opaque: {diags:?}"
        );
    }

    #[test]
    fn cmdsub_operand_top_carries_the_literal_command_name() {
        // item-6 (`282` §12): a ⊤ OPERAND at a LITERAL-command-word site carries that command word
        // as `CommandName::Literal`, so the `{command}` fill speaks the command in the caller's terms
        // (the flagship: `apt-get`). This is the end-to-end literal path — value-flow-derived at the
        // emit site, never synthesized late. (A ⊤ COMMAND WORD carries `Unclear`; that emit is the
        // other arm, and const-prop `Resolved` is the marked analysis-side follow-up.)
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags("apt-get install -y \"$(date)\"", &mut i, &idx);
        let command = diags
            .iter()
            .find_map(|d| match &d.code {
                Code::CmdsubOperandTop(p) => Some(p.command.clone()),
                _ => None,
            })
            .expect("a cmdsub-operand-top disclosure fired");
        assert_eq!(command, CommandName::Literal("apt-get".to_owned()));
    }

    #[test]
    fn cmdsub_operand_top_carries_real_arena_cause_post_mint() {
        // STAGE-1 cause-wiring (the corrected `tc-cmdsub-cause`, 22D §1): the `dq-cmdsub-operand-top`
        // disclosure's `cause` was hard-`None` (the cause is minted AFTER the effects pass, so it was
        // unavailable at the kernel-early emit site). It must now carry the REAL arch-1 ⊤-cause
        // (`top_causes[node]`) — the why-lens consumer reads it off the typed `Diag`. Drive the
        // classify-internal pipeline (the same steps `classify` runs) and assert the finalized typed
        // diag carries `cause: Some(id)` AND that the id resolves to a `TopCause` node in the arena
        // (a real receipt, not a fabricated `ProvId`). `$(date)` makes the install's operand ⊤.
        let (mut i, idx, _s) = package_setup();
        let parsed = dorc_syntax::parse("apt-get install -y \"$(date)\"");
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, &mut i);
        let checks = vec![lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let mut diags: Vec<Diag> = Vec::new();
        // The same precompute classify runs (member families + effects + the deferred cmdsub-⊤
        // records), then the same post-mint finalize — so this exercises the real wiring.
        let (_families, effects, cmdsub_tops, _backings) = resolve_node_effects(
            &built.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &VerdictIndex::default(),
            &BTreeMap::new(),
            &mut i,
            &mut diags,
            &mut BTreeMap::new(),
            &mut BTreeSet::new(),
            crate::funcenv::LiveDefinitions::unsolved(),
        );
        let mut arena = dorc_core::ProvArena::new();
        let (top_causes, fallback) =
            mint_top_causes(&built.value, &parsed.value, &effects, &mut arena);
        let finalized = finalize_cmdsub_tops(&cmdsub_tops, &top_causes, fallback);
        assert_eq!(
            finalized.len(),
            1,
            "exactly one cmdsub-⊤ disclosure (the ⊤ operand): {finalized:?}"
        );
        let Code::CmdsubOperandTop(payload) = &finalized[0].code else {
            panic!(
                "the finalized diag must be a CmdsubOperandTop: {:?}",
                finalized[0].code
            );
        };
        let cause = payload
            .cause
            .expect("stage-1: the cmdsub-⊤ disclosure now carries a Some(cause), not None");
        // The cause is a REAL arena receipt: it resolves to a TopCause give-up node (not a fabricated id).
        let node = arena
            .node(cause)
            .expect("the cause resolves to a real arena node");
        assert_eq!(
            node.kind,
            dorc_core::OriginKind::TopCause,
            "the wired cause is the ⊤-give-up origin: {node:?}"
        );
    }

    #[test]
    fn member_body_top_operand_collapses_family_and_dedups_disclosure() {
        // Why (f-3b CORRECTED, `224` §10 22-q4): pins the LIVE dedup the corrected doc describes.
        // `record_member_sites` does not ⊤-gate body operands, so this loop's member argv carries a
        // ⊤ word (`$(date)`). `member_family` resolves the first ⊤ member to Opaque ⇒ the family
        // collapses (`_ => None`) ⇒ the in-loop site falls to the single-cell Flat path ⇒ MustRun
        // (it RUNS; not an EstablishMembers family, not elided — kFAIL-perform). The member-scan emit
        // passes `site: None` (suppressed); the single-cell fallback discloses the ⊤ once. So exactly
        // ONE `dq-cmdsub-operand-top` fires — the COUNT (not mere presence) is what proves the dedup:
        // a count ≠ 1 would mean the conductor's dedup model is wrong, not that the assertion is.
        let src = r#"for p in nginx curl; do apt-get install "$p" "$(date)"; done"#;

        let (mut i1, idx1, _s1) = package_setup();
        let classes = classify_src(src, &mut i1, &idx1);
        assert!(
            classes.contains(&SkipClass::MustRun),
            "the ⊤-operand member site runs (single-cell fallback), not an elided family: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishMembers { .. })),
            "the collapsed family must NOT classify as EstablishMembers (all-or-nothing): {classes:?}"
        );

        let (mut i2, idx2, _s2) = package_setup();
        let diags = classify_src_diags(src, &mut i2, &idx2);
        let count = diags
            .iter()
            .filter(|d| d.code.slug() == "cmdsub-operand-top")
            .count();
        assert_eq!(
            count, 1,
            "exactly ONE cmdsub-operand-top must fire — the member-scan emit is suppressed and \
             the single-cell fallback discloses once (the dedup). count={count}, diags={diags:?}"
        );
    }

    #[test]
    fn cmdsub_inner_nonleaf_disclosed_for_effectbearing_inner() {
        // Why (219 q-1.f, the exec-subst-body-nonleaf disclosure): an EFFECT-BEARING command
        // inside `$()` runs un-elidably (no leaf of its own) and is invisible today. The
        // disclosure surfaces it (`dq-cmdsub-inner-nonleaf`). `apt-get install -y nginx` inside
        // `$()` is oracled (Establishes) ⇒ effect-bearing ⇒ disclosed; the enclosing `echo` is
        // Pure so it never independently elides the inner install.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags(
            "echo \"installed: $(apt-get install -y nginx)\"",
            &mut i,
            &idx,
        );
        assert!(
            has_code(&diags, "cmdsub-inner-nonleaf"),
            "an effect-bearing $()-inner command must be disclosed: {diags:?}"
        );
    }

    #[test]
    fn pure_inner_cmdsub_discloses_nothing() {
        // Why (the gate on the disclosure): a PURE `$()`-inner command does nothing un-elidable,
        // so it must NOT emit `dq-cmdsub-inner-nonleaf` (warning-fatigue floor — disclose only
        // what actually runs un-elidably). `echo "$(echo hi)"`: the inner `echo` is Pure.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags("echo \"got: $(echo hi)\"", &mut i, &idx);
        assert!(
            !has_code(&diags, "cmdsub-inner-nonleaf"),
            "a pure $()-inner command discloses nothing un-elidable: {diags:?}"
        );
    }

    #[test]
    fn straightline_concrete_book_has_no_cmdsub_diagnostics() {
        // Why (the negative pin): a fully-concrete straight-line book has no ⊤ and no `$()`, so
        // NEITHER cmdsub code fires — the disclosure is specific to the degradation, not noise on
        // every command.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags("apt-get install -y nginx", &mut i, &idx);
        assert!(
            !has_code(&diags, "cmdsub-operand-top") && !has_code(&diags, "cmdsub-inner-nonleaf"),
            "a concrete book emits no cmdsub ⊤-diagnostics: {diags:?}"
        );
    }

    // ---- must-emit pin (B8 act-4 / residual-1): effect-kind-disagreement ----

    #[test]
    fn effect_kind_disagreement_emits_from_production_path() {
        // MUST-EMIT pin (x3n PINNED-BY-NOTHING): `effect-kind-disagreement` had no driving test —
        // a behavior change to it was invisible. Drive the real `cell_effect` give-up: a check whose
        // annotation kind (`package`) disagrees with the effect-map cell's kind (`widget`) for the
        // same (apt-get, install). The annotation wins (the cell re-keys under it), and the
        // disagreement is disclosed. Asserts the registered code FIRES from production, not merely
        // that the variant is constructed (the x3a-B/t-1 vacuity).
        let mut i = Interner::default();
        let predict_src = "\
apt_get__predict() {
   verb=$1; shift
   pkg : package = \"$1\"
   probe-pkg \"$pkg\"
}
";
        let widget = KindId(i.intern("widget"));
        let installed = SelectorId(i.intern("installed"));
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let mut idx = KindIndex::default();
        idx.add_effect(0, apt, install, widget, installed, ValueClaim::Establish);
        let checks = vec![lift_predicts(&mut i, predict_src).value];

        let parsed = dorc_syntax::parse("apt-get install nginx");
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, &mut i);
        let mut arena = dorc_core::ProvArena::new();
        let diags = classify(
            &built.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &VerdictIndex::default(),
            &mut i,
            &mut arena,
        )
        .diags;
        assert!(
            has_code(&diags, "effect-kind-disagreement"),
            "an annotation-vs-effect-map kind mismatch must disclose effect-kind-disagreement: {diags:?}"
        );
    }

    // ── `Reach`'s cause-excluding equality (`303:fnd-reach-equality-excludes-its-cause`) ─────
    //
    // PLACEMENT, flagged: these belong at the Kani tier by shape — they are ∀-laws over small
    // values, exactly what an exhaustive checker is for. They are HERE because `Reach` is still
    // raw-`BTreeSet`-backed, and a `BTreeSet` is out of that tier's reach (`300`
    // fnd-reach-lattice-outside-scope defers the eviction; the facade lane's `SortedSet` is
    // what made the rest of the algebra checkable). So the domain below is enumerated by hand
    // and the quantifier is a loop: exhaustive over a small domain rather than over a bounded
    // one, which is a weaker claim honestly made. When `Reach` moves onto the facade, these
    // move to `spike/verify/kani` unchanged in statement.

    /// The sample domain: both `Top` causes, and the four fact-sets over two cells. Distinct
    /// causes are the whole point — the equality must not be able to see them.
    fn reach_samples(i: &mut Interner) -> (Vec<Reach>, dorc_core::ProvId, dorc_core::ProvId) {
        let mut arena = dorc_core::ProvArena::new();
        let one = arena.leaf(dorc_core::OriginKind::TopCause, None);
        let two = arena.leaf(
            dorc_core::OriginKind::TopCause,
            Some(Span::new(dorc_core::BytePos(7), dorc_core::BytePos(9))),
        );
        assert_ne!(one, two, "the two causes must really differ");

        let kind = KindId(i.intern("com.example.Widget"));
        let cell = |selector: &str, i: &mut Interner| {
            FactKey::cell(kind, EntityRef::Singleton, SelectorId(i.intern(selector)))
        };
        let a = cell("installed", i);
        let b = cell("running", i);
        let facts = |members: &[FactKey]| Reach::Facts(members.iter().copied().collect());
        (
            vec![
                facts(&[]),
                facts(&[a]),
                facts(&[b]),
                facts(&[a, b]),
                Reach::Top(one),
                Reach::Top(two),
            ],
            one,
            two,
        )
    }

    #[test]
    fn reach_equality_is_an_equivalence_relation_that_cannot_see_a_cause() {
        // Reflexive, symmetric, transitive — and blind to the cause on purpose. `solve`'s
        // fixpoint test is `joined != state[w]`, so an equality that DID see the cause would
        // make a ⊤ re-derived at a fresh give-up point look changed forever and the worklist
        // would never terminate. The receipt still has to survive for the why-lens, which is
        // the last assertion: excluded from equality, readable through `top_cause`.
        let mut i = Interner::default();
        let (samples, one, two) = reach_samples(&mut i);

        for a in &samples {
            assert_eq!(a, a, "reflexive");
            for b in &samples {
                assert_eq!(a == b, b == a, "symmetric");
                for c in &samples {
                    if a == b && b == c {
                        assert_eq!(a, c, "transitive");
                    }
                }
            }
        }

        assert_eq!(
            Reach::Top(one),
            Reach::Top(two),
            "two ⊤s are equal however they were caused"
        );
        assert_eq!(
            Reach::Top(one).top_cause(),
            Some(one),
            "…and the receipt still reads back, unperturbed by that"
        );
        assert_eq!(Reach::Top(two).top_cause(), Some(two));
        assert_eq!(Reach::Facts(BTreeSet::new()).top_cause(), None);
    }

    #[test]
    fn reach_merges_respect_that_equality() {
        // The congruence property, and the one that actually makes the fixpoint terminate:
        // equal-modulo-cause inputs must produce equal-modulo-cause outputs. If `join` could
        // turn two equal values into two unequal ones, the solver would oscillate between them
        // forever — and every arm of `join`/`meet` that carries a cause is a place to get this
        // wrong (first-cause-wins is what makes it hold).
        let mut i = Interner::default();
        let (samples, _, _) = reach_samples(&mut i);

        for a in &samples {
            for a2 in &samples {
                if a != a2 {
                    continue;
                }
                for b in &samples {
                    for b2 in &samples {
                        if b != b2 {
                            continue;
                        }
                        assert_eq!(a.join(b), a2.join(b2), "⊔ respects the equality");
                        assert_eq!(a.meet(b), a2.meet(b2), "⊓ respects the equality");
                        assert_eq!(a.leq(b), a2.leq(b2), "⊑ respects the equality");
                    }
                }
            }
        }
    }

    #[test]
    fn reach_is_a_lattice_over_that_equality() {
        // The lattice laws the solver assumes, read through the cause-excluding equality —
        // because that equality is the only one the solver ever uses. `Reach` is not in the
        // Kani battery's reach, so this is where its laws are checked at all.
        let mut i = Interner::default();
        let (samples, _, _) = reach_samples(&mut i);
        let bottom = Reach::bottom();

        for a in &samples {
            assert_eq!(bottom.join(a), *a, "⊥ ⊔ a = a");
            assert_eq!(a.join(&bottom), *a, "a ⊔ ⊥ = a");
            assert_eq!(bottom.meet(a), bottom, "⊥ ⊓ a = ⊥");
            assert_eq!(a.join(a), *a, "⊔ idempotent");
            assert_eq!(a.meet(a), *a, "⊓ idempotent");
            assert!(a.leq(a), "⊑ reflexive");
            for b in &samples {
                assert_eq!(a.join(b), b.join(a), "⊔ commutative");
                assert_eq!(a.meet(b), b.meet(a), "⊓ commutative");
                assert_eq!(a.join(&a.meet(b)), *a, "a ⊔ (a ⊓ b) = a");
                assert_eq!(a.meet(&a.join(b)), *a, "a ⊓ (a ⊔ b) = a");
                for c in &samples {
                    assert_eq!(a.join(&b.join(c)), a.join(b).join(c), "⊔ associative");
                    assert_eq!(a.meet(&b.meet(c)), a.meet(b).meet(c), "⊓ associative");
                }
            }
        }
    }
}
