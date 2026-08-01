//! `verdict` — the guard-verdict function lift (rul-role-split / rul24-vouch-is-verdict-authoring,
//! 24A §1c / 24D §3). The STATIC half of the vouch: authoring an `<provider>.is_converged()`
//! verdict function IS the vouching act, and this module decides — for a site's
//! constant-propagated argv — whether the verdict function reaches a **vouching path** (the
//! license) or a **declining path** (24A §1c: "an unhandled path" / a path that ran no authored
//! check). The APPLY half is the shipped guard: [`crate::predict::strip_verdict`] emits the same
//! body strip-only, and the `|| <original>` glue re-checks live at position (rul-ternary-verdict).
//!
//! # Why "reached a check command" is the vouch (the hz-refusepath fence, 23A §6)
//!
//! The corpus-standard bodies EXIT 0 ON THEIR REFUSE PATHS (`case` with no matching arm returns
//! 0; `if [ "$2" = "" ]; then …; fi` returns 0 when the condition is false). So a guard minted at
//! a site whose argv reaches such a path would `check || mutator` with the check vacuously rc-0 ⇒
//! the mutator is suppressed on a path the author NEVER vouched — silent wrong-elision. The fence:
//! the vouch is available ONLY when the argparse traces the site's argv to a path that actually
//! RAN AN AUTHORED CHECK COMMAND ([`VerdictResolution::Vouched`]); an unhandled verb, an
//! `if`-false with no `else`, or an empty arm reaches no command ⇒ [`VerdictResolution::Declined`]
//! ⇒ no witness ⇒ run (kFAIL-perform). This is the reached-path component of rul-guard-license's
//! witness, made the load-bearing check exactly where hz-refusepath bites.
//!
//! # `return N`: an explicit verdict, read by the declared sense (fix-return-decline-inert, 24Kc F2
//! / 24M; the hz-refusepath fence)
//!
//! A reached `return N` with a LITERAL code is the author's explicit verdict, read against the
//! sole `is_converged` sense per rul-rc-partition — the same universal partition `USER_STORY`
//! teaches (0 = the named sense holds; 1 = its complement; ≥2 = can't-say). The converged code
//! (`0`) VOUCHES; the complement (`1`) or a ≥2 confused code DECLINES ⇒ the site runs. This closes
//! fix-return-decline-inert: an author who writes their verdict in the explicit-return style
//! taught by the partition (`case $1 in synced) return 0 ;; *) return 1 ;; esac`) is HONORED
//! (rul24M-rungs-default: an authored verdict-function reads as full-license, honor the author's
//! plain intent), not silently inert (loud-friend law). The apply glue needs nothing new: the
//! shipped guard re-runs the whole stripped body live and the `return N` bytes flow through the
//! same sense-flip glue as a live check's tool-rc (rul-rc-partition) — every sense×code cell
//! resolves correctly at apply.
//!
//! This REFINES find-return-vouches (24C), which blanket-declined every reached `return N` to fix
//! a `*) return 2 ;;` catch-all wrongly vouching. That case is UNCHANGED: `return 2` is ≥2 =
//! confused ⇒ still a decline. What changes is only the explicit `return 0`/`return 1` verdict —
//! previously silently inert, now lifted. The IMPLICIT vacuous-rc-0 paths the fence guards
//! (an unmatched `case`, an `if`-false with no `else`, an empty body) reach NO command and NO
//! `return`, stay [`VerdictResolution::Declined`], and never vouch — the distinction is exactly
//! authored-speech-act (explicit `return`) vs sh-fall-through (silence), rul24M-rungs-default's own
//! line. The inert fixed-rc builtins `false`/`:`/`true` ([`Decline::Inert`]) run no check and carry
//! no authored code ⇒ never vouch.
//!
//! A NON-literal `return` (`return $?`, a bare `return`, a non-integer arg) cannot be read to a
//! code ⇒ [`VerdictResolution::Declined`] (conservative: no vouch ⇒ run, kFAIL-perform).
//!
//! **Scope note (ru-26 churn-avoidance):** `return` still parses as a plain command (the dialect
//! has no `Stmt::Return`), caught HERE in the tracer, not at parse. An and-or list parses into
//! [`crate::predict::AndOr`], and exactly two GATE forms are modeled
//! ([`crate::predict::recognize_gate`]): `[ … ] ||/&& return N` and `cmd || return N`, both with a
//! LITERAL `N ≥ 2`. Everything else ⊤s ([`VerdictTop::AndOrList`]). A gate's test-led left is
//! argparse and never vouches; a command-led left may, because `|| return N≥2` leaves the body's rc
//! an honest verdict where `|| true` and `|| return 0` forge one.
//!
//! `inv-referent-agnostic`: the tracer never decodes the entity's text — it reuses the predict
//! argparse primitives ([`resolve_word`]/[`eval_test`]/[`pattern_matches`]) to find the reached
//! path, then asks only "did an authored command run there", never what the command *means*.

use std::collections::BTreeMap;

use dorc_aid::Carrier;
use dorc_aid::narrative::{DeclineClass, DeclineGate};
use dorc_core::{Interner, ProviderId, Rc, Span, Symbol};
use dorc_syntax::sem::UnsetPolicy;

use crate::report::recognized_class;

use crate::predict::{
    AndOr, AndOrItem, CaseArm, Command, MarkKind, Predict, PredictSet, ResolvedEntity, Stmt, Test,
    TopReason, Word, brace_tokens, eval_test, gate_fires, lift_verdicts_converged,
    map_provider_name, pattern_matches, recognize_gate, resolve_word, state_mutating_builtin,
};

/// The verdict funcdefs of a whole oracle set, keyed the way
/// `dorc_analysis::effect::command_effect` keys a book command word ([`map_provider_name`] then
/// intern). The typeless-floor seam (`24L` §7): the analyzer kernel is **verdict-unaware by
/// design** (`inv-determinism` — no oracle lift inside the kernel), so a driver lifts the verdict
/// role at the edge and threads the result INTO `classify` as DATA.
///
/// This is the `24L` §7 seam widened from membership to EVALUABLE data (`26H` §3.1): the kernel
/// asks not only "did this provider earn the synthetic auto-cell" ([`contains`](Self::contains))
/// but "does its verdict body author a coordinate for THIS argv" ([`get`](Self::get) +
/// [`evaluate_verdict_coord`]). One seam, so a membership answer and a body answer can never drift
/// apart. Owning (funcdefs are cloned in) so the three drivers need no lifetime surgery — the
/// bodies are small and the analysis side of this tool is never the constraint (`perf-doctrine`).
#[derive(Debug, Clone, Default)]
pub struct VerdictIndex {
    by_provider: BTreeMap<ProviderId, Predict>,
    /// Per provider, the SOURCE INDEX its verdict body came from — the twin of
    /// [`KindIndex::source_of`](crate::KindIndex::source_of), and for the same reason: a
    /// site-keyed act must be able to ask whether the file that spoke is the one live at its line
    /// (`28K` §2 rul-visibility-is-full-positional). Empty on a hand-built index ⇒ no opinion.
    sources: BTreeMap<ProviderId, usize>,
}

impl VerdictIndex {
    /// Lift the verdict role from each source and key it by mapped provider. Diags are DROPPED
    /// (`crate::validate` lifts the same role per-file and surfaces them once, framed into their
    /// own source, for gate-3).
    #[must_use]
    pub fn of(interner: &mut Interner, srcs: &[&str]) -> Self {
        let sets: Vec<VerdictSet> = srcs
            .iter()
            .map(|src| VerdictSet::lift(interner, src).value)
            .collect();
        Self::from_sets(interner, &sets)
    }

    /// Key already-lifted [`VerdictSet`]s, for a driver that holds them for other reasons (the cli
    /// pre-lifts them for the probe ship-closure) — one lift, not two.
    ///
    /// A provider authored by TWO files keeps the LAST — sh's last-definition-wins
    /// (`28K` §1 rul-sh-loads-dorc-reads), the same rule `command_effect` applies to competing
    /// predict checks. That answer is taken from the ONE seat, [`crate::live_source`], rather than
    /// re-derived from iteration order: the two spellings agree today, and an
    /// iteration-order-derived one would split the verdict's winner from the predict's SILENTLY —
    /// the site would then measure one author's cell and key the record to another's
    /// (`28M:fnd-verdict-resolution-duplicates-live-source`).
    ///
    /// The chosen source INDEX rides along, because a site-keyed consumer must be able to ask
    /// whether the file that spoke here is the one live at its line (`28K` §2
    /// rul-visibility-is-full-positional).
    #[must_use]
    pub fn from_sets(interner: &mut Interner, sets: &[VerdictSet]) -> Self {
        let mut by_provider = BTreeMap::new();
        let mut sources = BTreeMap::new();
        // Keyed by the MAPPED name — the same key `command_effect` looks a command word up under,
        // so two files spelling one provider differently still contest for one slot.
        let mapped_of = |interner: &mut Interner, set: &VerdictSet| -> Vec<(ProviderId, Symbol)> {
            let providers: Vec<Symbol> = set.providers().collect();
            providers
                .into_iter()
                .map(|p| {
                    let mapped = map_provider_name(interner.resolve(p));
                    (ProviderId(interner.intern(&mapped)), p)
                })
                .collect()
        };
        let keyed: Vec<Vec<(ProviderId, Symbol)>> =
            sets.iter().map(|set| mapped_of(interner, set)).collect();
        for (index, per_file) in keyed.iter().enumerate() {
            for &(key, raw) in per_file {
                let live = crate::live_source(keyed.len(), |i| {
                    keyed
                        .get(i)
                        .is_some_and(|f| f.iter().any(|(k, _)| *k == key))
                });
                if live != Some(index) {
                    continue;
                }
                if let Some(verdict) = sets.get(index).and_then(|set| set.get(raw)) {
                    by_provider.insert(key, verdict.clone());
                    sources.insert(key, index);
                }
            }
        }
        Self {
            by_provider,
            sources,
        }
    }

    /// Does this provider bear a verdict funcdef? The `24L` §2 auto-cell mint's own gate.
    #[must_use]
    pub fn contains(&self, provider: ProviderId) -> bool {
        self.by_provider.contains_key(&provider)
    }

    /// This provider's verdict funcdef, for tracing over a site argv.
    #[must_use]
    pub fn get(&self, provider: ProviderId) -> Option<&Predict> {
        self.by_provider.get(&provider)
    }

    /// Which source index this provider's verdict body came from, or `None` on a provenance-less
    /// (hand-built) index — no opinion, never "any file will do".
    #[must_use]
    pub fn source_of(&self, provider: ProviderId) -> Option<usize> {
        self.sources.get(&provider).copied()
    }

    /// The keyed providers, in deterministic order (`inv-determinism`).
    pub fn providers(&self) -> impl Iterator<Item = ProviderId> + '_ {
        self.by_provider.keys().copied()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_provider.is_empty()
    }
}

/// The mangled funcname suffix a verdict body strips to (`crate::predict::strip_verdict`) and the
/// name [`predict_fn_name`](../../dorc_plan)-side emitters build. `is_converged` is the sole verdict
/// role (`24C:rul24-ditch-is-diverged`); the inverted sense is spelled with explicit-return `case $?`
/// manual inversion inside an `is_converged`.
pub const VERDICT_SUFFIX: &str = "__is_converged";

/// The set of `<provider>__is_converged` verdict funcdefs lifted from one oracle file. Reuses the
/// predict dialect AST ([`Predict`]); only the scanned name-suffix and the collected outcome differ.
#[derive(Debug, Clone, Default)]
pub struct VerdictSet {
    converged: PredictSet,
}

impl VerdictSet {
    /// Lift every `<provider>__is_converged` funcdef in `src`. Fail-soft (`inv-no-throw`) and
    /// deterministic (`inv-determinism`) — the same contract as [`crate::predict::lift_predicts`],
    /// routed through the shared role-parametrized parser.
    pub fn lift(interner: &mut Interner, src: &str) -> Carrier<Self> {
        lift_verdicts_converged(interner, src).map(|converged| Self { converged })
    }

    /// The verdict funcdef for a provider, if the file authored one.
    #[must_use]
    pub fn get(&self, provider: Symbol) -> Option<&Predict> {
        self.converged.get(provider)
    }

    /// Providers with a lifted verdict funcdef, in deterministic order.
    pub fn providers(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.converged.providers()
    }

    /// Withhold a contested family's verdict members ([`PredictSet::withdrawing`]).
    #[must_use]
    pub fn withdrawing(
        self,
        contested: &dorc_core::ContestedFamilies,
        interner: &Interner,
    ) -> Self {
        Self {
            converged: self.converged.withdrawing(contested, interner),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.converged.is_empty()
    }
}

/// The result of tracing a verdict body over a concrete argv (`inv-superposition`: a
/// phase-agnostic fact; the phased caller collapses it). It answers ONLY "does the author's
/// verdict function vouch this argv's path" — never the convergence itself (that is the guard's
/// live re-check at apply).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerdictResolution {
    /// The argv reached a path that ran ≥1 authored check command — the VOUCH (a licensing path).
    Vouched,
    /// The argv reached no license: an unhandled `case`, an `if`-false with no `else`, an empty
    /// body, an inert builtin (`false`/`:`/`true`), OR an explicit `return` whose code is NOT the
    /// converged sense — the complement / a ≥2 confused code / an unreadable code
    /// (fix-return-decline-inert). A DECLINE (24A §1c "an unhandled path"): no witness forms ⇒ the
    /// site runs.
    Declined,
    /// Non-concrete argv / out-of-dialect-at-runtime — ⊤ (no witness; kFAIL-perform ⇒ run).
    Top(VerdictTop),
}

/// Why a verdict trace degraded to ⊤. A closed enum so a new degrade-reason breaks every
/// exhaustive match (the compiler-as-checklist), mirroring [`crate::touches::TouchesTop`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictTop {
    /// The argv was empty (no command for the argparse to consume).
    EmptyArgv,
    /// A reached word resolved to no concrete value (unbound var, unmodeled expansion, `$0`, a
    /// positional past the end in strict position) — the constprop half of the witness failed.
    NonConcreteWord(&'static str),
    /// The iteration budget was exhausted (a loop did not terminate within bound).
    BudgetExceeded,
    /// The reached path contains an and-or list (`a && b`, `a || b`, `a & b`), which no supported
    /// form covers yet ⇒ no witness ⇒ run.
    ///
    /// This variant fences a SOUNDNESS hole, not a precision one. The tracer had no list guard at
    /// all, so an or-list's LEFT operand read as a reached authored check: `dpkg-query -W "$1" ||
    /// true` VOUCHED, and the guard that vouch licenses re-runs the same body live, where the
    /// `|| true` forces rc 0 on every host — an always-skip guard suppressing the mutator whatever
    /// the world says (`23H` §9.4's disaster shape; the errexit-masked rc `R2-ORTRUE` forbids as a
    /// verdict). `|| return 0` vouched identically.
    AndOrList,
    /// The reached path contains a state-mutating builtin
    /// ([`state_mutating_builtin`](crate::predict::state_mutating_builtin)) ⇒ no witness ⇒ run.
    ///
    /// The verdict twin of the predict lane's [`TopReason::StateMutatingBuiltin`]. A reached check
    /// vouches for the coordinate the tracer resolved, so a head that rebinds the tracer's
    /// positionals or vars between the bind and the check makes the vouch name a different
    /// referent than the guard will re-measure live (`26I`, `26J`).
    StateMutatingBuiltin(&'static str),
}

impl VerdictTop {
    /// A short human-readable form for diagnostics/provenance.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            VerdictTop::EmptyArgv => "empty argv",
            VerdictTop::NonConcreteWord(w) => w,
            VerdictTop::BudgetExceeded => "iteration budget exceeded",
            VerdictTop::AndOrList => "reached an and-or list (out of dialect => runs)",
            // One table for both lanes: a head denied here reads exactly as it does in a predict.
            VerdictTop::StateMutatingBuiltin(head) => {
                TopReason::StateMutatingBuiltin(head).as_str()
            }
        }
    }
}

/// Trace `verdict` over `argv` — the full, concrete, verbatim argument list of the book's command
/// (NOT including the command word itself; the same contract as [`crate::predict::evaluate`] and
/// [`crate::touches::evaluate_touches`]). Returns a [`VerdictResolution`].
///
/// Pure + total (`inv-determinism`/`inv-no-throw`): no clock/RNG/IO, ordered collections only,
/// every path returns a resolution (the budget bounds loops).
#[must_use]
pub fn evaluate_verdict(verdict: &Predict, argv: &[&str]) -> VerdictResolution {
    if argv.is_empty() {
        return VerdictResolution::Top(VerdictTop::EmptyArgv);
    }
    let mut tr = Tracer::over(argv);
    match tr.run_block(&verdict.body) {
        Flow::Normal => {
            if tr.reached_command {
                VerdictResolution::Vouched
            } else {
                VerdictResolution::Declined
            }
        }
        // An explicit `return N` is the author's verdict (fix-return-decline-inert): `return 0`
        // vouches (the converged sense), the complement / ≥2 confused declines. It overrides any
        // earlier reached check (`return` forces the function's rc).
        Flow::Returned(code) => classify_return(code),
        // A `return` we cannot read to a code (bare/`$?`/non-integer) ⇒ no vouch ⇒ run.
        Flow::Declined => VerdictResolution::Declined,
        Flow::Top(reason) => VerdictResolution::Top(reason),
    }
}

/// Read an explicit `return N` code, per rul-rc-partition (0 = converged holds, 1 = its
/// complement, ≥2 = confused). Only `return 0` vouches; everything else declines. The inverted
/// sense is now spelled with explicit-return `case $?` manual inversion, which reaches a
/// `return 0` on the converged path — so this single-code partition covers it.
fn classify_return(code: Rc) -> VerdictResolution {
    if code.0 == 0 {
        VerdictResolution::Vouched
    } else {
        VerdictResolution::Declined
    }
}

/// A genuine verdict-trace DECLINE, fully classified (`27V` Lane A / `27W` §3): the gate, the
/// precise declining-arm span, and — when the reached path ran a recognized report-sink emission —
/// the tier-2 per-site authored class + emitting-arm span. All decision-inert (the rc-partition
/// stays a flat sink; the license plane reads none of it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclineInfo {
    /// Which gate the decline reached (an explicit non-converged `return`, a reached arm that ran
    /// no check, or an inert fixed-rc builtin).
    pub gate: DeclineGate,
    /// The PRECISE declining-statement span (the `return N` or the inert builtin), or `None` for
    /// the `Unreached` gate — nothing was reached to point at (`tc-unreached-arm-span-fallback`;
    /// the caller falls back to the funcdef `name_span`, the honest coarsest-true span).
    pub arm_span: Option<Span>,
    /// The tier-2 per-site emission on the reached path (`27W` §3): the recognized decline class +
    /// the emitting arm's span, or `None` (emitted nothing / dynamic format ⇒ tier-3 fallback).
    pub emission: Option<(DeclineClass, Span)>,
}

/// The span of the reached VOUCHING arm (the first authored check) for a `Vouched` trace over
/// `argv` (C7 vouch span; `27V:mech-minting-line-threading`) — the guard attribution's `file:line`.
/// `None` for a decline / ⊤ (no vouch) or a vouch with no located check (an explicit `return 0`
/// vouch runs no check ⇒ no reached-check span; the caller falls back to the funcdef `name_span`).
/// Re-traces (analysis-side, cheap), like [`classify_decline`].
#[must_use]
pub fn vouch_site(verdict: &Predict, argv: &[&str]) -> Option<Span> {
    if argv.is_empty() {
        return None;
    }
    let mut tr = Tracer::over(argv);
    match tr.run_block(&verdict.body) {
        // A `return 0` vouch reaches no check ⇒ `vouch_span` is `None` (name_span fallback).
        Flow::Normal if tr.reached_command => tr.vouch_span,
        Flow::Returned(code) if code.0 == 0 => tr.vouch_span,
        _ => None, // a decline / ⊤ — no vouch
    }
}

/// The **authored coordinate** a verdict body keys for `argv` (`26H` §3 — the W-B fix for
/// `26G:fnd-shared-auto-cell-collides`). `Some` iff the trace VOUCHES *and* the reached path
/// authored exactly one fully-resolved verdict coordinate; `None` sends the site to the `24L` §2
/// auto-cell, which is the founding markless floor and the safe answer for every other shape.
///
/// # Why this exists (oracle-contract §4)
///
/// "Verdict and observe marks mint selector tokens into the kind's vocabulary, and attach facts to
/// the one line that measured them." Before this, a verdict body's authored coordinate was never
/// read for keying, so every site of one command shared a per-provider singleton and a sibling that
/// merely failed to report de-licensed the rest. The `24L` §3 singleton coarseness was priced for
/// the MARKLESS body it was written for; a body that authored a coordinate is outside that pricing.
///
/// # The selection rule, and why each half of it is narrow
///
/// The coordinate comes ONLY from a VERDICT mark (`asserts`/`refutes`) on the reached path —
/// never an observe, never a bind alone. Polarity does NOT change the cell (the sense lives in the
/// vouch and the guard's glue, not the key). The kind and entity come from the reached inline BIND
/// exactly as [`crate::predict::evaluate`] resolves them, and the mark's own entity fragment is
/// never parsed: `identity-declared-never-inferred` gives the oracle's argparse sole authority
/// over identity, so the engine resolves the author's declared bind and nothing else. The selector
/// comes from the mark. A body that reaches TWO verdict marks keys NOTHING: one exit status can
/// witness exactly one cell (`281` §7 rc-arity), and guessing which is the disaster class.
///
/// Pure + total (`inv-determinism`/`inv-no-throw`), and re-traces rather than widening
/// [`evaluate_verdict`]'s hot path (the [`classify_decline`] precedent).
#[must_use]
pub fn evaluate_verdict_coord(verdict: &Predict, argv: &[&str]) -> Option<VerdictCoord> {
    if argv.is_empty() {
        return None;
    }
    let mut tr = Tracer::over(argv);
    let vouched = match tr.run_block(&verdict.body) {
        Flow::Normal => tr.reached_command,
        Flow::Returned(code) => code.0 == 0,
        Flow::Declined | Flow::Top(_) => false,
    };
    if !vouched {
        return None;
    }
    let [(mark_kind, mark_selector)] = tr.verdict_marks.as_slice() else {
        return None; // markless, or two marks the single rc cannot both witness
    };
    // `28A:rul-singleton-bind-drops`: no bind still keys an entity-LESS mark; a mark NAMING an
    // entity with no bind to resolve it keys nothing (never a garbage key).
    let (kind, entity) = tr.annotation.clone().or_else(|| {
        (mark_kind.entity_is_empty && !mark_kind.kind.is_empty())
            .then(|| (mark_kind.kind.clone(), ResolvedEntity::Singleton))
    })?;
    let selector = mark_selector.clone()?;
    // Brace-alternation on a VERDICT is single-cell-illegal (`277` §4c), as on the predict side.
    if brace_tokens(&selector).is_some() {
        return None;
    }
    Some(VerdictCoord {
        kind,
        entity,
        selector,
        observed: tr.observed,
    })
}

/// The coordinate a verdict body authored for one site's argv ([`evaluate_verdict_coord`]) — the
/// cell the site's establish keys, plus the observe cells the same reached path read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdictCoord {
    /// The reverse-DNS kind, opaque (`inv-referent-agnostic` — never decoded).
    pub kind: String,
    /// The entity the reached bind resolved to, or the Singleton for an entity-less coordinate.
    pub entity: ResolvedEntity,
    /// The verdict mark's selector.
    pub selector: String,
    /// The `:?` observe selectors the reached path also carried (`277` §5
    /// observe-backing-widening): each widens the established fact's backing with a sibling cell,
    /// so the kill-surface only GROWS (`inv-kfail`, apply). Deduped, source order.
    pub observed: Vec<String>,
}

/// A verdict mark's own coordinate fragments, as the tracer collected them.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkCoord {
    kind: String,
    /// Whether the mark named NO entity (or an explicitly empty one) — the Singleton re-point's gate.
    entity_is_empty: bool,
}

/// The [`DeclineGate`] a verdict trace decline took (C5). Thin wrapper over [`classify_decline`].
#[must_use]
pub fn decline_gate(verdict: &Predict, argv: &[&str]) -> Option<DeclineGate> {
    classify_decline(verdict, argv).map(|i| i.gate)
}

/// The gate + PRECISE declining-arm span (C7). Thin wrapper over [`classify_decline`].
#[must_use]
pub fn decline_site(verdict: &Predict, argv: &[&str]) -> Option<(DeclineGate, Option<Span>)> {
    classify_decline(verdict, argv).map(|i| (i.gate, i.arm_span))
}

/// Fully classify a verdict trace's DECLINE over `argv` (C5 gate + C7 arm span + `27W` tier-2
/// class): `Some(DeclineInfo)` for a [`VerdictResolution::Declined`], `None` for a `Vouched` or ⊤
/// trace. Re-traces (analysis-side, cheap) so [`evaluate_verdict`]'s hot decision path stays
/// byte-untouched (`inv-determinism`).
#[must_use]
pub fn classify_decline(verdict: &Predict, argv: &[&str]) -> Option<DeclineInfo> {
    if argv.is_empty() {
        return None; // empty argv ⇒ ⊤ (not a decline)
    }
    let mut tr = Tracer::over(argv);
    let gate = match tr.run_block(&verdict.body) {
        Flow::Normal if tr.reached_command => return None, // Vouched
        Flow::Normal if tr.reached_inert => DeclineGate::InertBuiltin,
        Flow::Normal => DeclineGate::Unreached,
        Flow::Returned(code) if code.0 == 0 => return None, // Vouched
        Flow::Returned(_) | Flow::Declined => DeclineGate::Return,
        Flow::Top(_) => return None, // ⊤, not a decline
    };
    // `Unreached` reached no statement ⇒ no honest span (its `decline_span` is already `None`).
    Some(DeclineInfo {
        gate,
        arm_span: tr.decline_span,
        emission: tr.emission,
    })
}

/// Budget = `4 * argv.len() + BUDGET_CONSTANT` — mirrors the predict/touches evaluators.
const BUDGET_CONSTANT: usize = 32;

/// The verdict interpreter: the SAME argparse control-flow as the predict/touches evaluators
/// (`while`/`case`/`shift`/assign/`if` — reusing [`resolve_word`]/[`eval_test`]/[`pattern_matches`]
/// so the vouch travels the exact value-flow predict does, the 24A §1b fence), but its Command
/// handler records that a check RAN (the vouch signal) rather than resolving an annotation or
/// collecting a coordinate. Deliberately a SEPARATE run-loop (the touches precedent,
/// tc-touches-eval-dup): the three collectors differ fundamentally, and a duplicated argparse loop
/// keeps the load-bearing predict path untouched.
struct Tracer {
    positionals: Vec<String>,
    vars: BTreeMap<Symbol, String>,
    /// Set true the moment a reached [`Stmt::Command`] runs — the vouch signal (a path the author
    /// wrote a check for). An argparse-only path (`while`/`shift`/assign, an unmatched `case`)
    /// never sets it ⇒ [`VerdictResolution::Declined`].
    reached_command: bool,
    /// Set true when a reached path ran an inert fixed-rc builtin (`false`/`:`/`true`) but no real
    /// check — the `DeclineGate::InertBuiltin` signal for the aid plane ([`decline_gate`]). Display
    /// tier only; never read by [`evaluate_verdict`]'s decision (an inert path declines either way).
    reached_inert: bool,
    /// The span of the LAST reached declining statement (a `return N` or an inert builtin) — the
    /// C7 precise arm span for [`decline_site`]. Display tier only; never read by the decision.
    decline_span: Option<Span>,
    /// The tier-2 per-site emission (`27W` §3): the recognized decline CLASS + the emitting arm's
    /// span, captured when a reached path ran a `report_sink` emission with a readable literal
    /// format. `None` when the reached path emitted nothing, or the format was dynamic (⇒ tier-3
    /// runtime fallback). Decision-inert (`two-plane-aid-law`); never read by the decision.
    emission: Option<(DeclineClass, Span)>,
    /// The span of the FIRST reached authored check (the vouching arm) — the C7 precise vouch span
    /// for [`vouch_site`], surfaced as `file:line` at the guard attribution render. Display tier
    /// only; never read by the decision (the vouch signal is `reached_command`).
    vouch_span: Option<Span>,
    /// The first inline BIND reached, resolved to (kind, entity) — the same first-annotation-wins
    /// identity [`crate::predict::evaluate`] resolves, traced through the same value-flow so a
    /// verdict body's coordinate can never disagree with a predict body's over one argv.
    /// Keying tier ([`evaluate_verdict_coord`]); never read by the vouch decision.
    annotation: Option<(String, ResolvedEntity)>,
    /// Every VERDICT mark (`asserts`/`refutes`) reached on the selected path, in source order. A
    /// Vec rather than a first-wins Option deliberately: TWO reached verdict marks must key
    /// NOTHING (one rc witnesses one cell — `281` §7), and that is only visible from the count.
    verdict_marks: Vec<(MarkCoord, Option<String>)>,
    /// The `:?` observe selectors reached on the selected path (`277` §5 backing-widening).
    observed: Vec<String>,
    budget: usize,
    steps: usize,
}

enum Flow {
    Normal,
    /// A reached `return N` with a LITERAL code (fix-return-decline-inert): the author's explicit
    /// verdict, read against the declared sense in [`evaluate_verdict`] via [`classify_return`].
    /// Exits the function, overriding any earlier reached check. Propagates up like [`Flow::Top`].
    Returned(Rc),
    /// A reached `return` we cannot read to a code (bare / `$?` / non-integer arg): the path
    /// DECLINES ⇒ no vouch ⇒ run (conservative, kFAIL-perform). Propagates up, ending the block.
    Declined,
    Top(VerdictTop),
}

impl Tracer {
    /// A fresh tracer over a concrete `argv` — the ONE constructor every entry point shares, so a
    /// new collection field cannot reach one trace and miss another. Callers have already ruled
    /// out an empty argv. Budget mirrors the predict/touches evaluators.
    fn over(argv: &[&str]) -> Self {
        Tracer {
            positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
            vars: BTreeMap::new(),
            reached_command: false,
            reached_inert: false,
            decline_span: None,
            emission: None,
            vouch_span: None,
            annotation: None,
            verdict_marks: Vec::new(),
            observed: Vec::new(),
            budget: argv.len().saturating_mul(4).saturating_add(BUDGET_CONSTANT),
            steps: 0,
        }
    }

    fn tick(&mut self) -> Result<(), VerdictTop> {
        self.steps = self.steps.saturating_add(1);
        if self.steps > self.budget {
            Err(VerdictTop::BudgetExceeded)
        } else {
            Ok(())
        }
    }

    fn run_block(&mut self, body: &[Stmt]) -> Flow {
        for stmt in body {
            match self.run_stmt(stmt) {
                Flow::Normal => {}
                // A `return` (Declined) or a degrade (Top) ends the block/loop, propagating up.
                other => return other,
            }
        }
        Flow::Normal
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> Flow {
        if let Err(reason) = self.tick() {
            return Flow::Top(reason);
        }
        match stmt {
            Stmt::Assign { name, value } => {
                // A non-concrete rvalue leaves the var unbound (a later use degrades to ⊤) — never
                // bound to a bogus value. Same posture as predict/touches.
                if let Ok(v) = self.resolve(value) {
                    self.vars.insert(*name, v);
                }
                Flow::Normal
            }
            Stmt::Shift { count } => self.run_shift(count.unwrap_or(1)),
            Stmt::While { test, body } => self.run_while(test, body),
            Stmt::Case { scrutinee, arms } => self.run_case(scrutinee, arms),
            Stmt::If {
                test,
                then_body,
                else_body,
            } => match eval_test(test, &self.positionals, &self.vars) {
                Ok(true) => self.run_block(then_body),
                Ok(false) => self.run_block(else_body),
                Err(reason) => Flow::Top(top_from_word(reason)),
            },
            // The vouch signal: an authored check command RAN on the reached path.
            Stmt::Command(cmd) => self.run_command(cmd),
            Stmt::AndOr(list) => self.run_and_or(list),
            // An annotation desugars to a binding (as in touches); a bare mark is a no-op. Neither
            // is a "check command", so neither vouches on its own.
            Stmt::Annotation(anno) => {
                // First bind wins, as on the predict side. An UNRESOLVABLE value deliberately does
                // NOT degrade the trace — the vouch decision predates this collector and must not
                // move — so the site simply keys nothing (`26H` §3.3).
                let entity = match &anno.value {
                    None => Some(ResolvedEntity::Singleton),
                    Some(value) => match self.resolve(value) {
                        Ok(v) => {
                            self.vars.insert(anno.name, v.clone());
                            Some(ResolvedEntity::Operand(v))
                        }
                        Err(_) => None,
                    },
                };
                if let Some(entity) = entity
                    && self.annotation.is_none()
                {
                    self.annotation = Some((anno.kind.clone(), entity));
                }
                Flow::Normal
            }
        }
    }

    /// Walk an and-or list. Only the closed forms ([`recognize_gate`]) survive; everything else ⊤s.
    ///
    /// A gate's TEST-led left is argparse, never a measurement, so it cannot vouch — it decides
    /// statically which side runs and nothing more. A COMMAND-led left MAY vouch: its rc still
    /// reaches the function unmasked when it succeeds, and the `|| return N≥2` routes its failure
    /// into the rc-partition's flat can't-say sink rather than forging a pass. That is exactly the
    /// property `R2-ORTRUE` demands ("a lifted guard's rc is a verdict only if the analyzer can
    /// prove it unmasked") and exactly what `|| true` / `|| return 0` lack.
    fn run_and_or(&mut self, list: &AndOr) -> Flow {
        let Some(gate) = recognize_gate(list) else {
            return Flow::Top(VerdictTop::AndOrList);
        };
        match gate.left {
            AndOrItem::Test(test) => match eval_test(test, &self.positionals, &self.vars) {
                Ok(held) if gate_fires(gate.op, held) => {
                    self.decline_span = Some(gate.return_span);
                    Flow::Returned(Rc(gate.code))
                }
                Ok(_) => Flow::Normal,
                Err(reason) => Flow::Top(top_from_word(reason)),
            },
            AndOrItem::Command(cmd) => self.run_command(cmd),
        }
    }

    fn run_shift(&mut self, count: u32) -> Flow {
        let n = count as usize;
        if n > self.positionals.len() {
            return Flow::Top(VerdictTop::NonConcreteWord("shift past end of argv"));
        }
        self.positionals.drain(0..n);
        Flow::Normal
    }

    fn run_while(&mut self, test: &Test, body: &[Stmt]) -> Flow {
        loop {
            if let Err(reason) = self.tick() {
                return Flow::Top(reason);
            }
            match eval_test(test, &self.positionals, &self.vars) {
                Ok(true) => match self.run_block(body) {
                    Flow::Normal => {}
                    // A `return` (Declined) or a degrade (Top) breaks the loop, propagating up.
                    other => return other,
                },
                Ok(false) => return Flow::Normal,
                Err(reason) => return Flow::Top(top_from_word(reason)),
            }
        }
    }

    fn run_case(&mut self, scrutinee: &Word, arms: &[CaseArm]) -> Flow {
        let value = match self.resolve(scrutinee) {
            Ok(v) => v,
            Err(reason) => return Flow::Top(top_from_word(reason)),
        };
        for arm in arms {
            if arm.patterns.iter().any(|p| pattern_matches(p, &value)) {
                return self.run_block(&arm.body); // sh: first matching arm only
            }
        }
        // No arm matched, no `*` catch-all: sh falls through with no effect ⇒ no command runs ⇒ a
        // DECLINE (the reached path did not vouch). Faithful to sh, not a degrade.
        Flow::Normal
    }

    /// A reached authored CHECK is the vouch: the author wrote a real state-measurement for this
    /// path. Its words must RESOLVE concretely (the constprop half of the witness): a check whose
    /// operand does not resolve (`dpkg-query -W "$1"` with `$1` past-end) is not a characterizable
    /// check ⇒ ⊤ (conservative; kFAIL-perform), exactly the touches emitter's posture minus the
    /// printf restriction.
    ///
    /// But NOT every reached command is a check: `return N` is the author's explicit VERDICT, not
    /// a measurement (fix-return-decline-inert) — [`run_return`](Self::run_return) exits the path
    /// with its code for [`evaluate_verdict`] to read against the sense. The inert fixed-rc builtins
    /// `false`/`:`/`true` ([`Decline::Inert`]) run but record no vouch (the path continues). Only a
    /// resolved, non-idiom command sets the vouch.
    fn run_command(&mut self, cmd: &Command) -> Flow {
        // A recognized report-sink emission is DECISION-INERT (`tc-emission-inert-in-tracer`,
        // `27W` §2): never vouches, never ⊤s — a license-plane fix in the safe direction (an
        // emission-only body would else read the printf's rc-0 as a vouch). Skipped BEFORE the
        // word-resolve loop so its ⊤-shaped sink word never ⊤s the trace; the tier-2 class + arm
        // are captured here (`27W` §3), `None` for a dynamic format (⇒ tier-3).
        if cmd.report_sink {
            self.reached_inert = true;
            self.decline_span = Some(cmd.span);
            if let Some(class) = recognized_class(cmd) {
                self.emission = Some((class, cmd.span));
            }
            return Flow::Normal;
        }
        for w in &cmd.words {
            // `"$@"` in command position is the faithful positional list — concrete-by-
            // construction (the traced positionals), so it does NOT ⊤ the check. This is the
            // founding-pin fix (`27H` finding-positional-oracle-side-couples-founding-pin): the
            // one-liner `mycmd --dry-run "$@"` vouches because `"$@"` re-expands the site's argv,
            // which the shipped body runs verbatim (the probe's real rc is authoritative). A
            // VALUE-position `"$@"` never reaches here (it ⊤s in `resolve_word`).
            if matches!(w, Word::PositionalArgs) {
                continue;
            }
            if let Err(reason) = self.resolve(w) {
                return Flow::Top(top_from_word(reason));
            }
        }
        // Ahead of the idiom read: letting a denied head set `reached_command` would vouch for a
        // coordinate the shipped guard re-measures under different positionals (`26I`).
        if let Some(head) = state_mutating_builtin(cmd) {
            return Flow::Top(VerdictTop::StateMutatingBuiltin(head));
        }
        match decline_idiom(cmd.words.first()) {
            // `return N` exits the function with the author's explicit verdict code (sense-read).
            Some(Decline::Return) => self.run_return(cmd),
            // `false`/`:`/`true` ran but measured nothing ⇒ no vouch; the path continues.
            Some(Decline::Inert) => {
                self.reached_inert = true;
                self.decline_span = Some(cmd.span);
                Flow::Normal
            }
            // A real check ran on this path ⇒ the vouch signal (hz-refusepath: only here). The
            // FIRST such check's span is the C7 vouch arm (guard render `file:line`).
            None => {
                self.reached_command = true;
                if self.vouch_span.is_none() {
                    self.vouch_span = Some(cmd.span);
                }
                self.collect_mark(cmd);
                Flow::Normal
            }
        }
    }

    /// Read a reached `return`'s code (fix-return-decline-inert). A LITERAL non-negative integer
    /// arg (resolved through constprop, so `n=0; return $n` is read too) surfaces as
    /// [`Flow::Returned`] for the sense-read in [`evaluate_verdict`]; a bare `return`, `return $?`,
    /// or a non-integer arg cannot be read to a code ⇒ [`Flow::Declined`] (conservative: run). The
    /// words already resolved in [`run_command`], so re-resolving the arg here never ⊤s.
    fn run_return(&mut self, cmd: &Command) -> Flow {
        self.decline_span = Some(cmd.span); // the `return`'s own span is the precise decline arm (C7)
        // A malformed `return 0 junk` (≥2 args) is a runtime arity error in dash (rc≠0), so it is
        // NOT the author's converged verdict — DECLINE it (run), never read `words[1]` and ignore
        // the rest (resid-return-arity, `24C`: reading `get(1)` alone silently VOUCHED the wrong
        // direction). Exactly one arg is the readable-verdict shape.
        if cmd.words.len() > 2 {
            return Flow::Declined;
        }
        match cmd.words.get(1) {
            Some(arg) => match self.resolve(arg) {
                Ok(s) => match s.parse::<i32>() {
                    Ok(code) => Flow::Returned(Rc(code)),
                    Err(_) => Flow::Declined,
                },
                Err(_) => Flow::Declined,
            },
            None => Flow::Declined,
        }
    }

    /// Record a reached CHECK's trailing mark for [`evaluate_verdict_coord`]. A verdict mark
    /// (`asserts`/`refutes`) is a candidate KEY — polarity is not part of the cell, so both land in
    /// one list. An observe (`reads`) only WIDENS the backing and can never key. Meta-plane verbs
    /// (`safe-across`, `stored-in`, …) ride other members and are ignored here. Collection tier
    /// only: nothing in this function reaches the vouch decision.
    fn collect_mark(&mut self, cmd: &Command) {
        let Some(mark) = &cmd.mark else { return };
        match mark.kind {
            MarkKind::Asserts | MarkKind::Refutes => self.verdict_marks.push((
                MarkCoord {
                    kind: mark.target.kind.clone(),
                    entity_is_empty: mark.target.entity.as_deref().unwrap_or("").is_empty(),
                },
                mark.target.prop.clone(),
            )),
            MarkKind::Reads => {
                if let Some(selector) = &mark.target.prop
                    && !self.observed.iter().any(|s| s == selector)
                {
                    self.observed.push(selector.clone());
                }
            }
            MarkKind::SafeAcross
            | MarkKind::Disturbs
            | MarkKind::Lends
            | MarkKind::StoredIn
            | MarkKind::Undivided => {}
        }
    }

    /// Resolve a word in strict context (`Unresolved` on a past-end positional) — the vouch's
    /// constprop half must resolve concretely, exactly as a predict annotation value must.
    fn resolve(&self, word: &Word) -> Result<String, TopReason> {
        resolve_word(word, &self.positionals, &self.vars, UnsetPolicy::Unresolved)
    }
}

/// A reached command that is not an authored state-CHECK (rul-rc-partition / the hz-refusepath
/// fence). Neither MEASURES state; the distinction from a check drives whether a vouch can form.
enum Decline {
    /// `return N` — exits the function with the author's explicit verdict code, read against the
    /// declared sense in [`evaluate_verdict`] (fix-return-decline-inert): the converged code
    /// vouches, the complement / ≥2 confused declines. NOT a state-measurement — the author states
    /// the verdict outright rather than deriving it from a check's tool-rc.
    Return,
    /// `false` (rc 1 = complement) / `:` / `true` (rc 0 VACUOUSLY — the hz-refusepath vacuous-pass
    /// a guard must never read as check-passed) — an inert non-check; runs but does not vouch.
    Inert,
}

/// Classify a reached command's argv[0]: is it a DECLINE idiom rather than an authored check?
/// Only a LITERAL argv[0] matches (a `$cmd`-word command is opaque ⇒ not a named idiom, and
/// resolves-or-⊤s upstream). `:`/`true` reproduce a fixed rc-0; treating them as vouches would be
/// the vacuous-pass the fence exists to stop.
fn decline_idiom(word: Option<&Word>) -> Option<Decline> {
    let name = match word {
        Some(Word::Literal(s) | Word::SingleQuotedLiteral(s)) => s.as_str(),
        _ => return None,
    };
    match name {
        "return" => Some(Decline::Return),
        "false" | ":" | "true" => Some(Decline::Inert),
        _ => None,
    }
}

/// Map a predict word-resolution [`TopReason`] into a [`VerdictTop`] — a resolve failure inside a
/// verdict trace is the same non-concreteness, carried under the verdict degrade-enum.
fn top_from_word(reason: TopReason) -> VerdictTop {
    VerdictTop::NonConcreteWord(reason.as_str())
}

/// The distinct literal command names (argv[0]) a verdict body would RUN — a guard's own
/// **check-commands** (23A §5). gate-6's widened dual-rail judge allowlists a guard's own
/// check-command as a legitimate apply-only line (the guard's live check runs at apply but is
/// absent from the bare book); the cli emits one `guardcmd <argv0>` ledger line per entry so the
/// judge screams ONLY on UNRELATED apply-only lines (cf-5). A non-literal argv[0] (a dynamic
/// command word) is skipped — it cannot be statically named for the allowlist. Deterministic
/// first-seen order, deduped (`inv-determinism`). Recurses into every control-flow body so a check
/// buried in a `case` arm (the corpus idiom) is found.
#[must_use]
pub fn check_commands(verdict: &Predict) -> Vec<String> {
    let mut out = Vec::new();
    collect_check_commands(&verdict.body, &mut out);
    out
}

fn collect_check_commands(body: &[Stmt], out: &mut Vec<String>) {
    for stmt in body {
        match stmt {
            Stmt::Command(cmd) => {
                if let Some(Word::Literal(w)) = cmd.words.first()
                    && !out.iter().any(|c| c == w)
                {
                    out.push(w.clone());
                }
            }
            Stmt::Case { arms, .. } => {
                for a in arms {
                    collect_check_commands(&a.body, out);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_check_commands(then_body, out);
                collect_check_commands(else_body, out);
            }
            Stmt::While { body, .. } => collect_check_commands(body, out),
            // A list's commands RUN when the guard runs; omitting them leaves gate-6's judge
            // screaming at the guard's own checks as unrelated apply-only lines (cf-5).
            Stmt::AndOr(list) => {
                for cmd in list.commands() {
                    if let Some(Word::Literal(w)) = cmd.words.first()
                        && !out.iter().any(|c| c == w)
                    {
                        out.push(w.clone());
                    }
                }
            }
            Stmt::Assign { .. } | Stmt::Shift { .. } | Stmt::Annotation(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::Interner;

    /// Lift the sole verdict funcdef from `src` and trace it over `argv`.
    fn trace(src: &str, argv: &[&str]) -> VerdictResolution {
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one verdict funcdef");
        let verdict = set.value.get(provider).expect("the verdict funcdef");
        evaluate_verdict(verdict, argv)
    }

    /// Lift the sole verdict funcdef and classify its decline gate over `argv`.
    fn gate(src: &str, argv: &[&str]) -> Option<DeclineGate> {
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one verdict funcdef");
        let verdict = set.value.get(provider).expect("the verdict funcdef");
        decline_gate(verdict, argv)
    }

    /// Lift the sole verdict funcdef, trace over `argv`, and return the SOURCE TEXT the decline
    /// arm span covers (or `None` for `Unreached` / a vouch). Pins C7's precise arm span against
    /// the real bytes — an anti-masking check (the span is derived, never hand-set).
    fn decline_arm_text<'a>(src: &'a str, argv: &[&str]) -> Option<&'a str> {
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one verdict funcdef");
        let verdict = set.value.get(provider).expect("the verdict funcdef");
        decline_site(verdict, argv)
            .and_then(|(_, span)| span.map(|s| &src[s.lo.0 as usize..s.hi.0 as usize]))
    }

    #[test]
    fn emission_only_body_declines_never_vouches() {
        // tc-emission-inert-in-tracer: an emission-only arm must NOT vouch on the printf's rc-0.
        let src = "\
x__is_converged() {
   case $1 in
   drop_caches) printf 'decline unsound %s\\n' \"$1\" >>\"${DREP_V1:-/dev/null}\" ;;
   esac
}";
        assert_eq!(
            trace(src, &["drop_caches"]),
            VerdictResolution::Declined,
            "an emission-only arm declines (never vouches on the printf's rc-0)"
        );
    }

    #[test]
    fn canonical_emission_then_return_two_declines_with_arm_captured() {
        // The canonical idiom (`27W` §2): emit, then `return 2` ⇒ Return gate, arm = the `return 2`.
        let src = "\
x__is_converged() {
   case $1 in
   drop_caches)
      printf 'decline unsound %s\\n' \"$1\" >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   esac
}";
        assert_eq!(trace(src, &["drop_caches"]), VerdictResolution::Declined);
        assert_eq!(
            gate(src, &["drop_caches"]),
            Some(DeclineGate::Return),
            "the `return 2` after the emission is the decline gate (not a ⊤ from the sink word)"
        );
        assert_eq!(
            decline_arm_text(src, &["drop_caches"]),
            Some("return 2"),
            "the arm span is the reached `return 2`"
        );
    }

    #[test]
    fn classify_decline_captures_the_tier2_class() {
        // Tier-2 (`27W` §3): argv threads to a reached emission ⇒ class + emitting-arm span at the site.
        let src = "\
x__is_converged() {
   case $1 in
   drop_caches)
      printf 'decline unsound %s\\n' \"$1\" >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   restart) systemctl is-active -- \"$2\" ;;
   esac
}";
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        let p = set.value.providers().next().unwrap();
        let v = set.value.get(p).unwrap();
        let info = classify_decline(v, &["drop_caches"]).expect("a decline");
        let (class, span) = info.emission.expect("the reached emission's class");
        assert_eq!(class, DeclineClass::Unsound);
        assert_eq!(
            &src[span.lo.0 as usize..span.hi.0 as usize],
            "printf 'decline unsound %s\\n' \"$1\" >>\"${DREP_V1:-/dev/null}\"",
            "the emitting-arm span is the printf, distinct from the `return 2` decline arm"
        );
        // `restart` reaches a real check ⇒ vouches ⇒ no decline.
        assert!(classify_decline(v, &["restart", "nginx"]).is_none());
    }

    #[test]
    fn vouch_site_points_at_the_reached_check_arm() {
        // C7 vouch span: the reached CHECK arm, not the funcdef.
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, APT);
        let p = set.value.providers().next().unwrap();
        let v = set.value.get(p).unwrap();
        let span = vouch_site(v, &["install", "-y", "curl"]).expect("a reached check");
        assert_eq!(
            &APT[span.lo.0 as usize..span.hi.0 as usize],
            "dpkg-query -W \"$1\" >/dev/null 2>&1",
            "the vouch span is the reached check command, not the funcdef name"
        );
        assert_eq!(vouch_site(v, &["restart", "nginx"]), None); // a decline reaches no check
    }

    #[test]
    fn decline_site_points_at_the_precise_reached_arm() {
        // C7: the decline span is the EXACT reached declining statement, not the funcdef.
        // (Return arm ⇒ the `return`; Unreached ⇒ None/name_span; inert ⇒ the builtin.)
        let src = "\
x__is_converged() {
   case $1 in
   install) dpkg-query -W \"$2\" ;;
   *) return 2 ;;
   esac
}";
        assert_eq!(
            decline_arm_text(src, &["remove"]),
            Some("return 2"),
            "an unhandled verb reaches the `*) return 2` arm ⇒ the `return 2` span"
        );
        let no_catchall = "x__is_converged() { case $1 in install) dpkg-query -W \"$2\" ;; esac }";
        assert_eq!(
            decline_arm_text(no_catchall, &["remove"]),
            None,
            "Unreached has no reached statement ⇒ span is None (name_span fallback)"
        );
        let inert = "x__is_converged() { verb=$1; shift; case $verb in restart) false ;; esac }";
        assert_eq!(
            decline_arm_text(inert, &["restart"]),
            Some("false"),
            "the inert `false` arm ⇒ the `false` span"
        );
    }

    #[test]
    fn decline_gate_names_the_reached_decline_shape() {
        // C5 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): the gate is DERIVED from the
        // reached path, never a hand-set tag; a Vouched trace has no gate.
        assert_eq!(
            gate("x__is_converged() { return 2; }\n", &["a"]),
            Some(DeclineGate::Return),
            "a non-converged explicit return declines through the Return gate"
        );
        assert_eq!(
            gate(
                "x__is_converged() { case $1 in install) dpkg-query -W \"$2\" ;; esac }\n",
                &["remove"],
            ),
            Some(DeclineGate::Unreached),
            "an unmatched case reaches no check ⇒ the Unreached gate"
        );
        assert_eq!(
            gate("x__is_converged() { false; }\n", &["a"]),
            Some(DeclineGate::InertBuiltin),
            "an inert fixed-rc builtin runs no check ⇒ the InertBuiltin gate"
        );
        assert_eq!(
            gate(
                "x__is_converged() { case $1 in install) dpkg-query -W \"$2\" ;; esac }\n",
                &["install", "nginx"],
            ),
            None,
            "a reached authored check vouches ⇒ no decline gate"
        );
    }

    // Mirrors the real apt argparse: flag-strip before and after the verb, bind the verb, and
    // check the operand — the guard23 flagship's `is_converged` shape.
    const APT: &str = "\
apt_get__is_converged() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   case $verb in
   install) dpkg-query -W \"$1\" >/dev/null 2>&1 ;;
   esac
}";

    #[test]
    fn matched_verb_reaches_a_check_and_vouches() {
        // `install -y curl`: flag-strip drops `-y`, verb=install, the install arm runs
        // `dpkg-query` (a check) ⇒ Vouched (the reached-path license).
        assert_eq!(
            trace(APT, &["install", "-y", "curl"]),
            VerdictResolution::Vouched
        );
    }

    #[test]
    fn unhandled_verb_reaches_no_check_and_declines() {
        // `restart nginx`: no `restart` arm, no `*` catch-all ⇒ no command runs ⇒ Declined — the
        // P-rundelta / hz-refusepath fence (a state-guard must never eat a run-delta verb).
        assert_eq!(
            trace(APT, &["restart", "nginx"]),
            VerdictResolution::Declined
        );
    }

    #[test]
    fn top_argv_operand_tops_the_verdict() {
        // `install` with no operand ⇒ the `$1` the check reads is past-end ⇒ ⊤ (P-topargv: an
        // un-propagatable argv reaches no vouched path). The install arm IS reached, but its
        // check word does not resolve — a ⊤, not a decline.
        assert!(matches!(
            trace(APT, &["install"]),
            VerdictResolution::Top(_)
        ));
    }

    #[test]
    fn empty_argv_tops() {
        assert_eq!(
            trace(APT, &[]),
            VerdictResolution::Top(VerdictTop::EmptyArgv)
        );
    }

    #[test]
    fn if_false_no_else_reaches_no_check_and_declines() {
        // The multi-operand refuse shape (P-multiop): `if [ "$2" = "" ]; then check; fi` with a
        // SECOND operand ⇒ the `if` is false, no `else`, no command runs ⇒ Declined (not a
        // vacuous rc-0 vouch — the hz-refusepath fence).
        let src = "\
apt_get__is_converged() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   if [ \"$2\" = \"\" ]; then dpkg-query -W \"$1\" >/dev/null 2>&1; fi
}";
        // one operand ⇒ `$2` empty ⇒ if-true ⇒ check runs ⇒ Vouched.
        assert_eq!(
            trace(src, &["install", "nginx"]),
            VerdictResolution::Vouched
        );
        // two operands ⇒ `$2`=curl ⇒ if-false ⇒ no check ⇒ Declined.
        assert_eq!(
            trace(src, &["install", "nginx", "curl"]),
            VerdictResolution::Declined
        );
    }

    #[test]
    fn catchall_return_declines_never_vouches() {
        // find-return-vouches (24C): a `*) return 2 ;;` catch-all REACHED by an unhandled verb is
        // a DECLINE (rul-rc-partition: return ≥2 = confused ⇒ run), NEVER a vouch. Before this fix
        // it wrongly VOUCHED (a reached command was the vouch) — which, once a vouch licenses full
        // skip (Part B), would ELIDE a mutation on a path the author declined. The `install` arm
        // (a real check) still vouches; only the return-arm declines.
        let src = "\
apt_get__is_converged() {
   verb=$1; shift
   case $verb in
   install) dpkg-query -W \"$1\" >/dev/null 2>&1 ;;
   *) return 2 ;;
   esac
}";
        assert_eq!(
            trace(src, &["restart", "nginx"]),
            VerdictResolution::Declined,
            "the `*) return 2 ;;` catch-all DECLINES, never vouches"
        );
        assert_eq!(
            trace(src, &["install", "nginx"]),
            VerdictResolution::Vouched,
            "the real `install` check still vouches"
        );
    }

    #[test]
    fn resid_return_arity_extra_args_declines() {
        // resid-return-arity (`24C`): `return 0 junk` is a runtime arity error in dash (rc≠0), so it
        // is NOT the author's converged verdict — DECLINE (run). Reading `words[1]` alone and
        // ignoring the rest silently VOUCHED the wrong direction (own-line-authored, corpus-absent).
        let clean = "\
apt_get__is_converged() {
   case $1 in
   synced) return 0 ;;
   *) return 2 ;;
   esac
}";
        assert_eq!(
            trace(clean, &["synced"]),
            VerdictResolution::Vouched,
            "a clean single-arg `return 0` still vouches"
        );
        let junk = "\
apt_get__is_converged() {
   case $1 in
   synced) return 0 junk ;;
   *) return 2 ;;
   esac
}";
        assert_eq!(
            trace(junk, &["synced"]),
            VerdictResolution::Declined,
            "`return 0 junk` (≥2 args ⇒ dash arity error) DECLINES, never vouches"
        );
    }

    #[test]
    fn arity_gate_return_declines_multi_operand() {
        // The in-dialect arity-refuse (the refusepath floor's form): `if [ "$2" != "" ]; then
        // return 2; fi`. A multi-operand invocation hits the `return 2` ⇒ Declined; a single
        // operand skips it and reaches the real check ⇒ Vouched. This is the shape a verdict
        // function uses instead of the out-of-dialect `[ … ] || return N` shorthand.
        let src = "\
apt_get__is_converged() {
   verb=$1; shift
   if [ \"$2\" != \"\" ]; then return 2; fi
   case $verb in
   install) dpkg-query -W \"$1\" >/dev/null 2>&1 ;;
   *) return 2 ;;
   esac
}";
        assert_eq!(
            trace(src, &["install", "nginx", "curl"]),
            VerdictResolution::Declined,
            "a second operand trips the arity gate's `return 2` ⇒ DECLINE"
        );
        assert_eq!(
            trace(src, &["install", "nginx"]),
            VerdictResolution::Vouched,
            "a single operand clears the gate and reaches the real check"
        );
    }

    #[test]
    fn inert_fixed_rc_builtins_never_vouch() {
        // `false` (rc 1 = complement) and `true` / `':'` (rc 0 VACUOUSLY — the hz-refusepath
        // vacuous-pass) run no check ⇒ never a vouch. A guard/elide reading a vacuous rc-0 as
        // "converged" is exactly the wrong-elision the fence forbids. (`:` is written quoted here
        // because a bare `:` lexes as the dialect mark-marker, not a command.)
        for inert in ["false", "true", "':'"] {
            let src = format!(
                "apt_get__is_converged() {{ verb=$1; shift; case $verb in restart) {inert} ;; esac }}"
            );
            assert_eq!(
                trace(&src, &["restart", "nginx"]),
                VerdictResolution::Declined,
                "`{inert}` is an inert non-check ⇒ Declined, never a vouch"
            );
        }
    }

    #[test]
    fn explicit_return_zero_vouches_in_converged_sense() {
        // fix-return-decline-inert (24Kc F2 / 24M): the explicit-return style USER_STORY's
        // exit-status partition teaches (0 = the named sense holds). `synced) return 0` is the
        // author's converged verdict ⇒ Vouched; `*) return 1` is the complement (diverged) ⇒
        // Declined (run). Before the fix, EVERY reached return declined ⇒ the oracle was silently
        // inert (the loud-friend violation this fix closes).
        let src = "\
foo__is_converged() {
   case $1 in
   synced) return 0 ;;
   *) return 1 ;;
   esac
}";
        assert_eq!(
            trace(src, &["synced"]),
            VerdictResolution::Vouched,
            "`return 0` under is_converged is the author's converged verdict ⇒ Vouched"
        );
        assert_eq!(
            trace(src, &["stale"]),
            VerdictResolution::Declined,
            "`return 1` is the complement (diverged) ⇒ Declined ⇒ run"
        );
    }

    #[test]
    fn unconditional_explicit_return_vouches_but_implicit_fallthrough_declines() {
        // rul24M-rungs-default: an AUTHORED verdict-function reads as full-license, so an explicit
        // `return 0` is a speech-act ⇒ Vouched even unconditionally. The hz-refusepath fence bites
        // only IMPLICIT vacuous rc-0 (an unmatched `case` reaching no command and no `return`),
        // which stays Declined. The line the fix draws is authored-speech-act vs sh-silence.
        let explicit = "\
foo__is_converged() {
   return 0
}";
        assert_eq!(
            trace(explicit, &["anything"]),
            VerdictResolution::Vouched,
            "an explicit unconditional `return 0` is the author's speech-act ⇒ Vouched"
        );

        let implicit = "\
foo__is_converged() {
   case $1 in
   handled) return 0 ;;
   esac
}";
        assert_eq!(
            trace(implicit, &["unhandled"]),
            VerdictResolution::Declined,
            "an unmatched `case` reaches no command and no `return` ⇒ implicit fall-through ⇒ Declined"
        );
        assert_eq!(
            trace(implicit, &["handled"]),
            VerdictResolution::Vouched,
            "the matched arm reaches the explicit `return 0` ⇒ Vouched"
        );
    }

    /// The verdict half of `26I:fnd-state-builtins-silently-mis-key`. A reached check vouches for
    /// the coordinate the tracer resolved, so a head that rebinds the positionals between the bind
    /// and the check makes the vouch name one referent while the guard — which re-runs this body
    /// live — measures another. Loose `!matches!` deliberately: a later reclassification of these
    /// away from ⊤ must not silently re-bless them as vouches.
    #[test]
    fn a_state_mutating_builtin_never_vouches() {
        for head in [
            "set --",
            "unset pkg",
            "eval \"pkg=x\"",
            "cd /tmp",
            "read pkg",
        ] {
            let src = format!("x__is_converged() {{\n   {head}\n   dpkg-query -W \"$1\"\n}}\n");
            assert!(
                !matches!(trace(&src, &["nginx"]), VerdictResolution::Vouched),
                "`{head}` diverges the traced positionals from the shipped body ⇒ never vouches"
            );
        }
    }

    /// The fence-not-blanket half: the same body without the mutating head still vouches, and a
    /// modeled `shift` still both vouches and consumes.
    #[test]
    fn the_unmutated_spellings_still_vouch() {
        for src in [
            "x__is_converged() {\n   dpkg-query -W \"$1\"\n}\n",
            "x__is_converged() {\n   shift\n   dpkg-query -W \"$1\"\n}\n",
        ] {
            assert_eq!(trace(src, &["nginx", "curl"]), VerdictResolution::Vouched);
        }
    }

    /// `fnd-ortrue-vouches-today` — the SOUNDNESS pin. The tracer had no list guard, so an
    /// or-list's LEFT operand read as a reached authored check and `check || true` VOUCHED. The
    /// guard that vouch licenses re-runs this body live, where `|| true` forces rc 0 on every
    /// host — the mutator suppressed whatever the world says (`23H` §9.4's always-skip shape, and
    /// exactly the errexit-masked rc `R2-ORTRUE` refuses as a verdict). `|| return 0` was the same
    /// hole spelled numerically. Neither may ever vouch again.
    #[test]
    fn an_rc_masking_or_list_never_vouches() {
        for masked in [
            "dpkg-query -W \"$1\" || true",
            "dpkg-query -W \"$1\" || return 0",
        ] {
            let src = format!("x__is_converged() {{\n   {masked}\n}}\n");
            assert!(
                !matches!(trace(&src, &["nginx"]), VerdictResolution::Vouched),
                "`{masked}` forges the body's rc ⇒ it must never vouch"
            );
        }
    }

    /// The other side of that discriminator, so the fix is a fence and not a blanket: the SAME
    /// body without the masking tail still vouches. What changed is the rc-forging list, not the
    /// reached-check rule.
    #[test]
    fn the_unmasked_spelling_still_vouches() {
        assert_eq!(
            trace(
                "x__is_converged() {\n   dpkg-query -W \"$1\"\n}\n",
                &["nginx"]
            ),
            VerdictResolution::Vouched,
        );
    }

    /// The SUPPORTED gate, and the discriminator that makes the whole design hold: `|| return 2`
    /// vouches where `|| true` and `|| return 0` never can. All three have a left operand whose rc
    /// is consumed by a `||`; only the `≥ 2` tail leaves the body's rc an honest verdict, because
    /// it routes the gate's failure into the can't-say sink instead of forging a pass.
    #[test]
    fn a_declining_gate_vouches_where_a_masking_tail_cannot() {
        let gated = "\
x__is_converged() {
   command -v dpkg-query >/dev/null 2>&1 || return 2
   dpkg-query -W \"$1\"
}
";
        assert_eq!(trace(gated, &["nginx"]), VerdictResolution::Vouched);
        for masked in ["|| true", "|| return 0", "|| return 1"] {
            let src = gated.replace("|| return 2", masked);
            assert!(
                !matches!(trace(&src, &["nginx"]), VerdictResolution::Vouched),
                "`{masked}` is not a decline ⇒ no vouch"
            );
        }
    }

    /// The test-led arity gate, now spellable inline. It must agree with the `if` spelling the
    /// dialect has always had — same argv, same verdict — and its DECLINE must point at the
    /// `return` itself, not the funcdef (the C7 precise arm span).
    #[test]
    fn an_inline_arity_gate_matches_its_if_spelling() {
        let inline = "\
apt_get__is_converged() {
   verb=$1; shift
   [ \"${2-}\" = \"\" ] || return 2
   case $verb in install) dpkg-query -W \"$1\" >/dev/null 2>&1 ;; esac
}
";
        let spelled = "\
apt_get__is_converged() {
   verb=$1; shift
   if [ \"${2-}\" != \"\" ]; then return 2; fi
   case $verb in install) dpkg-query -W \"$1\" >/dev/null 2>&1 ;; esac
}
";
        for argv in [
            vec!["install", "nginx"],
            vec!["install", "nginx", "curl"],
            vec!["restart", "nginx"],
        ] {
            assert_eq!(
                trace(inline, &argv),
                trace(spelled, &argv),
                "the two spellings agree for {argv:?}"
            );
        }
        assert_eq!(
            decline_arm_text(inline, &["install", "nginx", "curl"]),
            Some("return 2"),
            "the decline points at the gate's own `return`"
        );
    }

    /// Note A, documented as AUTHOR-OWNED JUDGMENT rather than an engine property: a body that is
    /// ONLY a gate vouches on the gate's success — but it vouches identically WITHOUT the
    /// `|| return N`, because a reached `command -v` is a reached command either way. The gate adds
    /// no license; whether an existence check is an adequate verdict is the author's call, and the
    /// contract's §5a wrong-yes attribution puts it on them. Pinned so nobody later reads the gate
    /// as the thing that granted the vouch.
    #[test]
    fn a_gate_only_body_vouches_identically_with_and_without_its_gate() {
        let with = "x__is_converged() {\n   command -v x >/dev/null 2>&1 || return 2\n}\n";
        let without = "x__is_converged() {\n   command -v x >/dev/null 2>&1\n}\n";
        assert_eq!(trace(with, &["nginx"]), trace(without, &["nginx"]));
        assert_eq!(trace(with, &["nginx"]), VerdictResolution::Vouched);
    }

    /// Chains and the `&&` command-led form stay ⊤ in a verdict body too — the fence is one
    /// recognizer, shared with the predict tracer, so neither can drift from the other.
    #[test]
    fn unsupported_list_shapes_still_top_in_a_verdict_body() {
        for body in [
            "dpkg-query -W \"$1\" && return 2",
            "dpkg-query -W \"$1\" || return 2 || return 3",
            "dpkg-query -W \"$1\" || dpkg-query -W other",
        ] {
            let src = format!("x__is_converged() {{\n   {body}\n}}\n");
            assert!(
                matches!(trace(&src, &["nginx"]), VerdictResolution::Top(_)),
                "`{body}` is not a supported gate ⇒ ⊤"
            );
        }
    }

    /// Lift the sole verdict funcdef and resolve its authored coordinate over `argv`.
    fn coord(src: &str, argv: &[&str]) -> Option<VerdictCoord> {
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one verdict funcdef");
        let verdict = set.value.get(provider).expect("the verdict funcdef");
        evaluate_verdict_coord(verdict, argv)
    }

    /// The `cp`-shaped body the r26 smoke kit exercises, and the whole point of W-B: two sites of
    /// ONE command whose authored coordinates name DIFFERENT entities must resolve to different
    /// cells. Before this they shared `dorc-auto:cp@converged`, so a sibling that merely failed to
    /// report de-licensed every other site (`26G:fnd-shared-auto-cell-collides`).
    const DROP: &str = "\
x__is_converged() {
   dst : sm.dorc.File = \"$2\"
   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content
}
";

    #[test]
    fn two_sites_of_one_command_key_their_own_authored_entities() {
        let a = coord(DROP, &["a.conf", "/etc/a.conf"]).expect("a resolved coordinate");
        let b = coord(DROP, &["b.conf", "/etc/b.conf"]).expect("a resolved coordinate");
        assert_eq!(a.kind, "sm.dorc.File");
        assert_eq!(a.selector, "content");
        assert_eq!(a.entity, ResolvedEntity::Operand("/etc/a.conf".to_owned()));
        assert_eq!(b.entity, ResolvedEntity::Operand("/etc/b.conf".to_owned()));
        assert_ne!(a, b, "different destinations are different cells");
        // One destination is ONE cell (`26H` §3.4 — `an-written-stale` rests on it).
        assert_eq!(
            coord(DROP, &["a.conf", "/etc/a.conf"]),
            coord(DROP, &["z.conf", "/etc/a.conf"]),
            "one destination is one cell, whatever the source"
        );
    }

    #[test]
    fn polarity_does_not_change_the_cell() {
        let refutes = DROP.replace("   : sm.dorc.File", "   :! sm.dorc.File");
        assert_eq!(
            coord(&refutes, &["a.conf", "/etc/a.conf"]),
            coord(DROP, &["a.conf", "/etc/a.conf"]),
        );
    }

    #[test]
    fn only_a_verdict_mark_keys_and_an_observe_only_widens() {
        let observed_too = "\
x__is_converged() {
   dst : sm.dorc.File = \"$2\"
   x stat -- \"$dst\"   :? sm.dorc.File:\"$dst\"@mode
   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content
}
";
        let c = coord(observed_too, &["a.conf", "/etc/a.conf"]).expect("a coordinate");
        assert_eq!(c.selector, "content", "the VERDICT mark keys the cell");
        assert_eq!(c.observed, vec!["mode".to_owned()], "the observe widens");

        let observe_only = "\
x__is_converged() {
   dst : sm.dorc.File = \"$2\"
   x stat -- \"$dst\"   :? sm.dorc.File:\"$dst\"@mode
}
";
        assert_eq!(
            coord(observe_only, &["a.conf", "/etc/a.conf"]),
            None,
            "an observe-only body authors no key ⇒ the auto-cell floor"
        );
    }

    #[test]
    fn every_unkeyable_shape_falls_to_the_auto_cell() {
        // `26H` §3.3 is EXHAUSTIVE: anything else takes the `24L` §2 floor, never a garbage key.
        let cases: &[(&str, &str, &[&str])] = &[
            (
                "markless (the 24L §2 founding shape)",
                "x__is_converged() { x cmp -- \"$1\" \"$2\" ;}",
                &["a.conf", "/etc/a.conf"],
            ),
            (
                "no selector on the mark (the corpus `kp` shape)",
                "x__is_converged() {\n   dst : sm.dorc.File = \"$2\"\n   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"\n}\n",
                &["a.conf", "/etc/a.conf"],
            ),
            (
                "a mark naming an entity with no bind to resolve it",
                "x__is_converged() {\n   x cmp -- \"$1\" \"$2\"   : sm.dorc.File:\"$2\"@content\n}\n",
                &["a.conf", "/etc/a.conf"],
            ),
            (
                "the bind's value does not resolve on this argv",
                "x__is_converged() {\n   dst : sm.dorc.File = \"$7\"\n   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content\n}\n",
                &["a.conf", "/etc/a.conf"],
            ),
            (
                "brace-alternation on a verdict (`277` §4c single-cell law)",
                "x__is_converged() {\n   dst : sm.dorc.File = \"$2\"\n   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@{content,mode}\n}\n",
                &["a.conf", "/etc/a.conf"],
            ),
            (
                "two verdict marks — one rc cannot witness two cells (`281` §7)",
                "x__is_converged() {\n   dst : sm.dorc.File = \"$2\"\n   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content\n   x stat -- \"$dst\"   : sm.dorc.File:\"$dst\"@mode\n}\n",
                &["a.conf", "/etc/a.conf"],
            ),
            (
                "the argv reaches a DECLINE — the author refused this shape",
                "x__is_converged() {\n   case $1 in\n   put) dst : sm.dorc.File = \"$2\"\n        x cmp -- \"$2\" \"$dst\"   : sm.dorc.File:\"$dst\"@content ;;\n   *) return 2 ;;\n   esac\n}\n",
                &["yank", "/etc/a.conf"],
            ),
            (
                "the reached path ⊤s before the mark (an unsupported and-or list)",
                "x__is_converged() {\n   dst : sm.dorc.File = \"$2\"\n   x probe -- \"$dst\" || x probe other\n   x cmp -- \"$1\" \"$dst\"   : sm.dorc.File:\"$dst\"@content\n}\n",
                &["a.conf", "/etc/a.conf"],
            ),
        ];
        for (why, src, argv) in cases {
            assert_eq!(coord(src, argv), None, "{why} must key nothing");
        }
    }

    #[test]
    fn an_entity_less_coordinate_keys_the_singleton() {
        // `28A:rul-singleton-bind-drops`: predict re-points identically; the two must agree.
        let nullary = "\
x__is_converged() {
   x freshness   : sm.dorc.PkgIndex@fresh
}
";
        let c = coord(nullary, &["update"]).expect("a coordinate");
        assert_eq!(c.kind, "sm.dorc.PkgIndex");
        assert_eq!(c.entity, ResolvedEntity::Singleton);
        assert_eq!(c.selector, "fresh");
    }

    #[test]
    fn explicit_return_two_still_declines() {
        // Regression pin for find-return-vouches (24C), preserved by the refinement: a ≥2 code is
        // CONFUSED ⇒ run. This is the corpus's sole return idiom (`*) return 2`).
        let conv = "foo__is_converged() { case $1 in x) return 2 ;; esac }";
        assert_eq!(trace(conv, &["x"]), VerdictResolution::Declined);
    }
}
