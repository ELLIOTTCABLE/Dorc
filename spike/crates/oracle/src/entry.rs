//! `entry` — context ENTRY: the primary lane of `27C` (measurement in the site's denoted context).
//!
//! Where [`crate::wrapper`] MODELS the peel (which dimensions a wrapper shifts, the ρ-claim), this
//! module makes entry REAL: the `cmd__enter()` member (the ONE licensed seat for real context
//! entry), the two-axis consent decision, the composition algebra across a peel chain, the degrade
//! ladder, and the entry-form author's vouched self-effects.
//!
//! # The primary lane in one screen (`27C` §0.1 / §3)
//!
//! A wrapped site (`sudo pipx install poddle`) is answered by ENTERING the context and running the
//! inner oracle's body there. Entry requires all of: the admin's dial permits ([`EscalationDial`])
//! × the connection can mechanically effect the shift ([`Capability`]) × the executed body carries
//! the tolerance vouch ([`ToleranceVouch`]) × a modeled wrapper with an entry form ([`EntryForm`]).
//! Any missing ⇒ can't-say ⇒ guard/run ([`EntryDecision`]/[`EntryDegrade`]). NO fallback-lane
//! cross-dimension carry is built here (that is a later lane, `27C` §0.2).
//!
//! # Reuse-never-acquire (the security spine, `27C` §0)
//!
//! The probe lane NEVER acquires authority — it re-uses what the connection already holds, only for
//! oracle bodies whose authors accepted context-shifted execution, only where the dial permits. The
//! entry form is non-interactive BY CONSTRUCTION (the author writes `sudo -n`, never bare `sudo`);
//! that non-interactivity is the AUTHOR's vouch (`authoring-is-vouching`), not an engine check
//! (`inv-referent-agnostic`: the engine ships the authored bytes, never decodes "sudo").
//!
//! # Entry self-effects (`27C:rul-probe-mutation-ownership-split`, WELDED 2026-07-17)
//!
//! Authoring `sudo__enter()` claims the entry's own residue (the auth-log line, the sudo-timestamp
//! refresh) — the entry-form AUTHOR's vouched residue, attributed to the entry command's line, the
//! same `authoring-is-vouching` chain as every other authored claim. NOT an engine-decided
//! acceptable-effect class. [`EntryForm::self_effect_span`] is where that attribution points.

use std::collections::{BTreeMap, BTreeSet};

use dorc_core::{Capability, Context, ContextKey, EscalationDial, Interner, Symbol};

use crate::predict::{Predict, PredictSet, Stmt, Word};
use crate::wrapper::{Dimension, LendEntry, LendMap};

// ===========================================================================
// The entry form (`27C` §3 — the `cmd__enter()` member)
// ===========================================================================

/// A lifted `<provider>__enter()` — the wrapper-family member that makes context entry REAL
/// (`27C` §3). Its body is the entry command wrapping the guest (`sudo -n "$@"`): the guest `"$@"`
/// sits in ARGUMENT position of a head command (the exact COMPLEMENT of a transparent peel, where
/// `"$@"` is command-position — [`crate::wrapper::detect_peel`]). Detection is structural
/// (`inv-referent-agnostic`): a terminal command with a non-empty head and a trailing `"$@"`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryForm {
    /// The wrapper provider this entry form serves (`sudo`, `chroot`, …), interned.
    pub provider: Symbol,
    /// The entry command's head words BEFORE the guest `"$@"` (`[sudo, -n]`, `[chroot, /mnt/target]`),
    /// as DISPLAY text for the authority-disclosure line (`27C:render-authority-disclosure`). Display
    /// only (exempt-plane, `inv-referent-agnostic`): the entry form SHIPS strip-only (the whole
    /// funcdef span), so nothing branches on these; they only NAME the entry command to the human.
    pub head: Vec<String>,
    /// The entry command's whole span — where the AUTHOR's vouched self-effects attribute
    /// (`27C:rul-probe-mutation-ownership-split`; `authoring-is-vouching`). The entry command's
    /// auth-log line / timestamp refresh is claimed by whoever authored THIS line.
    pub self_effect_span: dorc_core::Span,
}

/// Detect whether `enter` (a `<provider>__enter` body) is a well-formed entry form (`27C` §3).
/// Returns `None` for a body that is not an entry-shape (no terminal head-plus-`"$@"` command) —
/// an unmodeled/malformed entry form ⇒ its contexts are NEVER entered (the site walls, unchanged
/// law). Pure/total (`inv-no-throw`/`inv-determinism`). Reads the FIRST reachable entry command in
/// source order (a peel chain composes these; one entry form has one entry command by construction).
#[must_use]
pub fn detect_entry_form(enter: &Predict) -> Option<EntryForm> {
    let cmd = first_entry_command(&enter.body)?;
    // The head is every word before the trailing `"$@"` — display text for the disclosure line.
    let head = cmd.words[..cmd.words.len().saturating_sub(1)]
        .iter()
        .map(head_word_display)
        .collect();
    Some(EntryForm {
        provider: enter.provider,
        head,
        self_effect_span: cmd.span,
    })
}

/// Display text for a head word of an entry command (the authority-disclosure line, exempt-plane).
/// A dynamic word renders as an ellipsis placeholder — never decoded for meaning.
fn head_word_display(w: &Word) -> String {
    match w {
        Word::Literal(s) | Word::SingleQuotedLiteral(s) => s.clone(),
        Word::Positional(n) => format!("${n}"),
        _ => "…".to_owned(),
    }
}

/// The first reachable command whose LAST word is the guest `"$@"` ([`Word::PositionalArgs`]) and
/// which has a NON-EMPTY head (≥1 word before the guest) — the entry command. A bare `"$@"` (empty
/// head) is a transparent peel, NOT an entry form (`crate::wrapper`), so it returns `None` here.
fn first_entry_command(body: &[Stmt]) -> Option<&crate::predict::Command> {
    for stmt in body {
        match stmt {
            Stmt::Command(c)
                if !c.pipeline
                    && matches!(c.words.last(), Some(Word::PositionalArgs))
                    && c.words.len() >= 2 =>
            {
                return Some(c);
            }
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    if let Some(c) = first_entry_command(&arm.body) {
                        return Some(c);
                    }
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                if let Some(c) =
                    first_entry_command(then_body).or_else(|| first_entry_command(else_body))
                {
                    return Some(c);
                }
            }
            Stmt::While { body, .. } => {
                if let Some(c) = first_entry_command(body) {
                    return Some(c);
                }
            }
            _ => {}
        }
    }
    None
}

/// Lift every `<provider>__enter` in `src` into a [`PredictSet`] (COMMAND-keyed like the wrapper's
/// `predict`/`lend_map`). The consumer calls [`detect_entry_form`] per body. Same fail-soft contract as
/// `lift_predicts`.
#[must_use]
pub fn lift_entry_set(interner: &mut Interner, src: &str) -> dorc_core::Carrier<PredictSet> {
    crate::predict::lift_enters(interner, src)
}

// ===========================================================================
// The tolerance vouch (`27C` §2 — the oracle surface)
// ===========================================================================

/// The engine-owned mark-token that introduces a tolerance vouch (`27C` §2, STRAWMAN spelling):
/// `: tolerates:<dim>`. The mark parses as a bare-`:` colon-line carrying an `Establish`-sigil mark
/// whose `kind` fragment is exactly this token; the `entity` fragment (and any brace-alternation) is
/// the dimension set.
const TOLERATES_TOKEN: &str = "tolerates";

/// A lifted `tolerates:` vouch over a verdict body (`27C:vouch-tolerates`): per-function, per-
/// dimension, reachability-scoped. Asserts, for exactly the reached path: "this body's effects are
/// read-only BY DESIGN, not by privilege-starvation — executing it context-shifted along the named
/// dimensions will not mutate." It does NOT claim anything about the answer (answers differ per
/// context — that is the point of measuring in place).
///
/// Reachability-scoped (`27C` §2): a mark at the TOP of the body vouches for every verb; a mark
/// inside a `case` arm vouches only when that arm is reached. [`tolerated_on_path`] resolves the
/// vouched dimension set for a concrete argv (the same reachability the verdict/peel tracers walk).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ToleranceVouch {
    /// Unconditional (top-of-body) tolerated dimensions — vouched for EVERY reached path.
    unconditional: BTreeSet<Dimension>,
    /// Per-`case`-arm tolerated dimensions: `(arm-scrutinee-word-index unused here)` — modeled as a
    /// flat list of `(patterns, dims)` so [`tolerated_on_path`] can match the verb. Kept simple: the
    /// spike traces one top-level `case "$verb"` (the babby/stdlib shape); nested arms union upward.
    per_arm: Vec<ArmTolerance>,
}

/// One `case`-arm's tolerated dimensions plus the literal patterns that reach it (`27C` §2
/// reachability-scoping). `Wildcard`/dynamic patterns are recorded as [`ArmTolerance::catch_all`]
/// (matches any verb) — the safe direction for a vouch is NARROWER, so a catch-all arm's tolerance
/// applies broadly only because the AUTHOR placed it under `*`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ArmTolerance {
    /// The arm's literal verb patterns (`install`, `enable`); empty ⇒ [`catch_all`](Self::catch_all).
    verbs: Vec<String>,
    /// Whether this arm is the `*` catch-all (matches any verb).
    catch_all: bool,
    /// The dimensions vouched inside this arm.
    dims: BTreeSet<Dimension>,
}

impl ToleranceVouch {
    /// The tolerated dimension set on the path a concrete `verb` reaches (`27C` §2). Unconditional
    /// marks always apply; an arm's marks apply iff `verb` matches its patterns (or it is the
    /// catch-all). When `verb` is `None` (no verb dispatch), only the unconditional set applies —
    /// the safe direction (an arm's tolerance is not granted to an unknown path).
    #[must_use]
    pub fn tolerated_on_path(&self, verb: Option<&str>) -> BTreeSet<Dimension> {
        let mut out = self.unconditional.clone();
        for arm in &self.per_arm {
            let reached = match verb {
                Some(v) => arm.catch_all || arm.verbs.iter().any(|p| p == v),
                None => false,
            };
            if reached {
                out.extend(arm.dims.iter().copied());
            }
        }
        out
    }

    /// Does this body vouch tolerance for `dim` on ANY path? (For corroboration lints and the
    /// "heavy context-handling with no mark" hint — a coarse presence check, never a license.)
    #[must_use]
    pub fn mentions(&self, dim: Dimension) -> bool {
        self.unconditional.contains(&dim) || self.per_arm.iter().any(|a| a.dims.contains(&dim))
    }

    /// True iff the body carries no tolerance mark at all (drives the one-line adoption hint).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.unconditional.is_empty() && self.per_arm.iter().all(|a| a.dims.is_empty())
    }
}

/// A loud diagnostic code for an unknown dimension token on a `tolerates:` mark (`inv-top-reject`).
const TOLERATES_UNKNOWN_DIMENSION: dorc_core::DiagCode =
    dorc_core::DiagCode("tolerates-unknown-dimension");

/// Lift the [`ToleranceVouch`] from a verdict body (`27C` §2 — the `is_converged` member). Walks the
/// body for `: tolerates:<dim>` colon-lines: a mark at top level is unconditional; a mark inside a
/// `case` arm is scoped to that arm's verb patterns. Brace-alternation `tolerates:{user,fs-view}`
/// expands to a per-dimension set. An unknown dimension token is a LOUD diagnostic (`inv-top-reject`)
/// that mints no tolerance. Pure/total.
#[must_use]
pub fn lift_tolerance(verdict: &Predict) -> (ToleranceVouch, Vec<dorc_core::Diagnostic>) {
    let mut vouch = ToleranceVouch::default();
    let mut diags = Vec::new();
    collect_tolerance(&verdict.body, None, &mut vouch, &mut diags);
    (vouch, diags)
}

/// Recursively collect `tolerates:` marks. `arm` is `Some(patterns, catch_all)` when inside a `case`
/// arm (the marks scope to it), `None` at top level (unconditional).
fn collect_tolerance(
    body: &[Stmt],
    arm: Option<(&[String], bool)>,
    vouch: &mut ToleranceVouch,
    diags: &mut Vec<dorc_core::Diagnostic>,
) {
    for stmt in body {
        match stmt {
            Stmt::Command(c) => {
                let Some(mark) = &c.mark else { continue };
                if mark.target.kind != TOLERATES_TOKEN {
                    continue;
                }
                // The dimension(s) live in the mark's `entity` fragment (`tolerates:user` ⇒
                // entity="user"; `tolerates:{user,fs-view}` ⇒ entity="{user,fs-view}").
                let raw = mark.target.entity.as_deref().unwrap_or_default();
                let mut dims = BTreeSet::new();
                for tok in expand_dimension_set(raw) {
                    match Dimension::from_token(&tok) {
                        Some(d) => {
                            dims.insert(d);
                        }
                        None => diags.push(dorc_core::Diagnostic::warning(
                            TOLERATES_UNKNOWN_DIMENSION,
                            Some(mark.span),
                            format!(
                                "`{tok}` is not a known context dimension on a `tolerates:` vouch \
                                 (expected one of {}); the mark vouches nothing and the site stays \
                                 walled on that dimension (`27C` §2).",
                                Dimension::ALL.map(Dimension::as_token).join(", ")
                            ),
                        )),
                    }
                }
                if dims.is_empty() {
                    continue;
                }
                match arm {
                    None => vouch.unconditional.extend(dims),
                    Some((verbs, catch_all)) => vouch.per_arm.push(ArmTolerance {
                        verbs: verbs.to_vec(),
                        catch_all,
                        dims,
                    }),
                }
            }
            Stmt::Case { arms, .. } => {
                for a in arms {
                    let (verbs, catch_all) = arm_pattern_literals(a);
                    collect_tolerance(&a.body, Some((&verbs, catch_all)), vouch, diags);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                // Marks inside `if` keep the enclosing arm scope (an `if` inside a `case` arm stays
                // that arm's; a top-level `if` is unconditional). Conservative: both branches merge.
                collect_tolerance(then_body, arm, vouch, diags);
                collect_tolerance(else_body, arm, vouch, diags);
            }
            Stmt::While { body, .. } => collect_tolerance(body, arm, vouch, diags),
            _ => {}
        }
    }
}

/// The literal verb patterns of a `case` arm plus whether it is the `*` catch-all
/// (`27C` §2 reachability). A dynamic/glob pattern is treated as catch-all (the safe direction:
/// an arm we cannot pin matches broadly, so its tolerance applies where the author put it).
fn arm_pattern_literals(arm: &crate::predict::CaseArm) -> (Vec<String>, bool) {
    let mut verbs = Vec::new();
    let mut catch_all = false;
    for p in &arm.patterns {
        match p {
            crate::predict::Pattern::Literal(s) => verbs.push(s.clone()),
            crate::predict::Pattern::Wildcard => catch_all = true,
        }
    }
    (verbs, catch_all)
}

/// Expand a `tolerates:` dimension fragment into tokens: a bare `user` ⇒ `[user]`; a brace-set
/// `{user,fs-view}` ⇒ `[user, fs-view]` (`27C` §2 brace-alternation). Trims whitespace; empty
/// members are dropped. Referent-agnostic string surgery (the tokens are validated against the
/// closed dimension vocabulary by the caller).
fn expand_dimension_set(raw: &str) -> Vec<String> {
    let raw = raw.trim();
    if let Some(inner) = raw.strip_prefix('{').and_then(|r| r.strip_suffix('}')) {
        inner
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect()
    } else if raw.is_empty() {
        Vec::new()
    } else {
        vec![raw.to_owned()]
    }
}

// ===========================================================================
// The composition algebra (`27C` §3, as amended 2026-07-17)
// ===========================================================================

/// One dimension's SHIFT resolved for a site — the composition input (`27C` §3). A [`LendEntry`]
/// enriched with the mapped VALUE where the wrapper's `lend_map` resolved it against the site argv:
/// two `Mapped` shifts to DIFFERENT targets (`sudo -u bob` vs `sudo -u alice`) are DIFFERENT worlds
/// and must key distinctly, so an unresolved mapped value degrades to [`Top`](Shift::Top) — the
/// SAFE direction (a wall, never a false same-world collision).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shift {
    /// Full lend — the guest borrows the caller's world on this dimension wholesale. Identity of the
    /// composition (composes away).
    Full,
    /// Mapped lend to a resolved target (`user=root`, `fs-view=/mnt/target`). The value keys the
    /// world so distinct targets never collide.
    Mapped(String),
    /// ⊤ — a missing dimension (the enumerate-every-dimension wall) OR a mapped lend whose value the
    /// `lend_map` could not resolve at this site. Walls (the site can't be answered on this dimension).
    Top,
}

impl Shift {
    /// The composition MEET (`27C` §3 pointwise fold): `Full` is the identity, `Top` is absorbing,
    /// and two `Mapped` values compose only if they AGREE (a disagreement is a genuine two-world
    /// conflict ⇒ `Top`, the safe wall). Order-independent as a LATTICE value; the composed CONTEXT
    /// KEY carries the order separately (below), so `sudo chroot` ≠ `chroot sudo`.
    #[must_use]
    fn meet(self, other: Shift) -> Shift {
        match (self, other) {
            (Shift::Top, _) | (_, Shift::Top) => Shift::Top,
            (Shift::Full, x) | (x, Shift::Full) => x,
            (Shift::Mapped(a), Shift::Mapped(b)) if a == b => Shift::Mapped(a),
            (Shift::Mapped(_), Shift::Mapped(_)) => Shift::Top,
        }
    }
}

/// One link (one wrapper) in a peel chain, resolved for a site (`27C` §3): the wrapper provider and
/// its per-dimension shift. The composition folds a chain of these outermost-first (entry order =
/// book order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainLink {
    /// The wrapper provider (`sudo`, `chroot`) — interned; used only as an order-sensitive KEY
    /// component (referent-agnostic).
    pub provider: Symbol,
    /// This wrapper's shift per dimension (an absent dimension ⇒ [`Shift::Top`] via [`Self::shift`]).
    pub shifts: BTreeMap<Dimension, Shift>,
}

impl ChainLink {
    /// This link's shift for `dim` — an ABSENT dimension is [`Shift::Top`] (the enumerate-every-
    /// dimension law; `271:rul-lend-map`).
    #[must_use]
    pub fn shift(&self, dim: Dimension) -> Shift {
        self.shifts.get(&dim).cloned().unwrap_or(Shift::Top)
    }

    /// Build a link from a wrapper's [`LendMap`] and a per-dimension mapped-value resolver. `Full`
    /// and `Top` come straight from the lend map; a `Mapped` dimension calls `resolve(dim)` for its
    /// value (`None` ⇒ [`Shift::Top`], the safe wall for an unresolved mapped target).
    #[must_use]
    pub fn from_lend_map(
        provider: Symbol,
        lend: &LendMap,
        mut resolve: impl FnMut(Dimension) -> Option<String>,
    ) -> Self {
        let mut shifts = BTreeMap::new();
        for dim in Dimension::ALL {
            let shift = match lend.lend(dim) {
                LendEntry::Full => Shift::Full,
                LendEntry::Top => Shift::Top,
                LendEntry::Mapped => resolve(dim).map_or(Shift::Top, Shift::Mapped),
            };
            shifts.insert(dim, shift);
        }
        Self { provider, shifts }
    }
}

/// The composed inner context of a peel chain (`27C` §3): the per-dimension MEET plus the order-
/// sensitive canonical identity. Built by [`compose_chain`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposedContext {
    /// The composed shift per dimension (the pointwise meet across the chain).
    per_dimension: BTreeMap<Dimension, Shift>,
    /// The order-sensitive canonical string (the chain's providers + shifts in BOOK order). Two
    /// chains that shift the same dimensions in a DIFFERENT order produce DIFFERENT canonicals ⇒
    /// different [`ContextKey`]s (the nested-permutation pin).
    canonical: String,
}

impl ComposedContext {
    /// The composed shift for `dim` (the meet). An absent dimension is [`Shift::Top`].
    #[must_use]
    pub fn shift(&self, dim: Dimension) -> Shift {
        self.per_dimension.get(&dim).cloned().unwrap_or(Shift::Top)
    }

    /// The dimensions this chain WALLS (composed to [`Shift::Top`]) — each is a can't-say ⇒ the site
    /// degrades to guard/run on that boundary (`27C` §3 degrade ladder).
    #[must_use]
    pub fn walls(&self) -> Vec<Dimension> {
        Dimension::ALL
            .into_iter()
            .filter(|&d| matches!(self.shift(d), Shift::Top))
            .collect()
    }

    /// The dimensions this chain actually SHIFTS (composed to a mapped value — not `Full`, not
    /// walled). These are the boundaries entry must cross; the consent decision gates on them.
    #[must_use]
    pub fn crossed(&self) -> Vec<Dimension> {
        Dimension::ALL
            .into_iter()
            .filter(|&d| matches!(self.shift(d), Shift::Mapped(_)))
            .collect()
    }

    /// The [`Context`] this composition denotes (`27C` §3 / the `27L` `FactKey` seam). An identity
    /// chain (every dimension `Full`, nothing shifted) is [`Context::HostDefault`] — the inner sits
    /// in the caller's world, so its facts key exactly as an unwrapped site's (rung-0). Any shift or
    /// wall mints [`Context::Wrapped`] keyed by the interned canonical (order-sensitive).
    #[must_use]
    pub fn to_context(&self, interner: &mut Interner) -> Context {
        let all_full = Dimension::ALL
            .into_iter()
            .all(|d| matches!(self.shift(d), Shift::Full));
        if all_full {
            Context::HostDefault
        } else {
            Context::Wrapped(ContextKey(interner.intern(&self.canonical)))
        }
    }
}

/// Compose a peel chain into its inner [`ComposedContext`] (`27C` §3, as amended 2026-07-17): the
/// POINTWISE fold, outermost-first, per dimension — identity = full lend; ⊤ PROPAGATES (one silent
/// link walls the dimension chain-wide, never inherits a neighbor's lend); the inner context's key
/// is the composed per-dimension result, ORDER-SENSITIVE (entry order = book order of the chain).
/// `chain` is outermost-first (`sudo chroot CMD` ⇒ `[sudo, chroot]`).
#[must_use]
pub fn compose_chain(chain: &[ChainLink]) -> ComposedContext {
    let mut per_dimension = BTreeMap::new();
    for dim in Dimension::ALL {
        // Fold the meet from the identity (Full) across the chain, in order.
        let composed = chain
            .iter()
            .fold(Shift::Full, |acc, link| acc.meet(link.shift(dim)));
        per_dimension.insert(dim, composed);
    }
    // The canonical is BOOK-ORDER over the chain: `provider-id{dim=shift;…}` per link, `|`-joined.
    // Order-sensitive by construction (the provider sequence and each link's shift are positional).
    let canonical = chain
        .iter()
        .map(|link| {
            let shifts = Dimension::ALL
                .into_iter()
                .map(|d| format!("{}={}", d.as_token(), shift_tag(&link.shift(d))))
                .collect::<Vec<_>>()
                .join(";");
            format!("p{}{{{}}}", link.provider.as_u32(), shifts)
        })
        .collect::<Vec<_>>()
        .join("|");
    ComposedContext {
        per_dimension,
        canonical,
    }
}

/// A canonical tag for one shift in the context key (referent-agnostic identity string).
fn shift_tag(s: &Shift) -> String {
    match s {
        Shift::Full => "F".to_owned(),
        Shift::Top => "T".to_owned(),
        Shift::Mapped(v) => format!("M:{v}"),
    }
}

// ===========================================================================
// The two-axis consent decision (`27C` §1) + the degrade ladder (`27C` §3/§5)
// ===========================================================================

/// The per-dimension mapping of a mechanical [`Capability`] (`27C` §1(1)): can the connection effect
/// a shift on `dim` with zero new credentials? Lives here (not `core`) because it needs the wrapper
/// [`Dimension`]. Root does all; a NOPASSWD non-root does the user dimension only (`sudo -n`-class);
/// the substrate dimensions (fs-view/chroot, netns) are root-only; degraded does none.
#[must_use]
pub fn capability_permits(cap: Capability, dim: Dimension) -> bool {
    match cap {
        Capability::Root => true,
        Capability::NonRootNopasswd => matches!(dim, Dimension::User),
        Capability::Degraded => false,
    }
}

/// The outcome of the two-axis consent decision for entering a wrapped site's context (`27C` §1/§3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryDecision {
    /// Enter: every crossed dimension is mechanically-capable AND dial-and-vouch-licensed. The site
    /// is answered by measurement in-context (the primary lane).
    Enter,
    /// Can't-say ⇒ guard/run (`27C` §3 degrade ladder): the named reason is why entry is refused.
    /// EVERY rung lands the same place (guard/run) — the reason drives the disclosure/hint only.
    Degrade(EntryDegrade),
}

/// Why a wrapped site could not be entered — every rung ⇒ can't-say ⇒ guard/run (`27C` §3/§5). The
/// STATIC rungs (decided at plan time); the RUNTIME rungs (entry refused, impossible, rc 127,
/// in-context decline) surface through the probe record's rc-partition as `Unknown`/`Diverged` ⇒
/// run, and are named by [`EntryDegrade::runtime`] variants for the disclosure only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryDegrade {
    /// A crossed dimension the connection CANNOT mechanically effect (degraded mode, or a root-only
    /// dimension under a non-root capability) — `27C:hole-static-identity`. Names the dimension.
    NoCapability(Dimension),
    /// The dial is `--no-probe-escalation`: no oracle code shifts, in either lane (`27C` §1).
    DialForbids,
    /// The dial is the default (`--probe-escalation`) but the executed function carries NO tolerance
    /// vouch for a crossed dimension — the one-line adoption hint fires (`27C:hole-unvouched-oracles`).
    /// Names the unvouched dimension.
    Unvouched(Dimension),
    /// A crossed dimension the wrapper chain WALLS (⊤ — a missing `lend_map` key, or an unresolved
    /// mapped target). The enumerate-every-dimension wall (`271:rul-lend-map`). Names the dimension.
    TopDimension(Dimension),
    /// The wrapper has NO entry form (`<provider>__enter` absent): its contexts are never entered
    /// (`27C` §3). Distinct from an unmodeled wrapper (which never peels at all).
    NoEntryForm,
    /// A RUNTIME entry failure surfaced through the probe record (`27C` §3): entry refused
    /// (`sudo -n` failure), impossible (chroot target missing), rc 127 (missing deps in the view),
    /// or an in-context decline (rc ≥ 2). All land `Unknown`/run through the rc-partition; named
    /// here for the disclosure only.
    RuntimeEntryFailure,
}

/// Decide whether to ENTER a wrapped site's context (`27C` §1/§3 — the two-axis consent, both cells
/// implemented). Returns the FIRST blocking rung (deterministic order: capability, then dial, then
/// vouch), or [`EntryDecision::Enter`]. `crossed` are the dimensions the chain shifts; `walls` are
/// the ⊤ dimensions; `tolerated` is the vouched set on the site's reached path.
///
/// The consent table (`27C` §1, all four operational cells + the vouch axis):
/// * capability cannot effect a crossed dimension ⇒ [`EntryDegrade::NoCapability`] (host-fact wall);
/// * `--no-probe-escalation` ⇒ [`EntryDegrade::DialForbids`] (no oracle code shifts, ever);
/// * default `--probe-escalation` ⇒ enter iff EVERY crossed dimension is tolerance-vouched, else
///   [`EntryDegrade::Unvouched`] (the double-ended ack: author's mark × admin's default);
/// * `--escalate-any-probe` ⇒ enter regardless of the vouch (admin owns the blast-radius);
/// * any ⊤ (walled) crossed-or-otherwise dimension ⇒ [`EntryDegrade::TopDimension`].
#[must_use]
pub fn decide_entry(
    has_entry_form: bool,
    capability: Capability,
    dial: EscalationDial,
    crossed: &[Dimension],
    walls: &[Dimension],
    tolerated: &BTreeSet<Dimension>,
) -> EntryDecision {
    // A wrapper with no entry form never enters (its authored seat is absent).
    if !has_entry_form {
        return EntryDecision::Degrade(EntryDegrade::NoEntryForm);
    }
    // A ⊤ (walled) dimension is a can't-say regardless of dial/capability (silence-licenses-nothing).
    if let Some(&d) = walls.first() {
        return EntryDecision::Degrade(EntryDegrade::TopDimension(d));
    }
    // Axis 1 — mechanical capability: a crossed dimension the connection cannot effect walls first
    // (the host fact bounds everything; the dial cannot license a shift the connection can't make).
    for &d in crossed {
        if !capability_permits(capability, d) {
            return EntryDecision::Degrade(EntryDegrade::NoCapability(d));
        }
    }
    // Axis 2 — consent (the dial):
    match dial {
        EscalationDial::NoEscalation => EntryDecision::Degrade(EntryDegrade::DialForbids),
        EscalationDial::AnyProbe => EntryDecision::Enter, // admin overrides absent author consent
        EscalationDial::VouchedOnly => {
            // Default: enter iff EVERY crossed dimension is tolerance-vouched (both-sides consent).
            for &d in crossed {
                if !tolerated.contains(&d) {
                    return EntryDecision::Degrade(EntryDegrade::Unvouched(d));
                }
            }
            EntryDecision::Enter
        }
    }
}

// ===========================================================================
// §6 mined-idiom lints (recognize, NEVER license) + the authority disclosure
// ===========================================================================

/// The one-line ADOPTION HINT for a site that degraded on a missing tolerance vouch
/// (`27C` §2 / `EntryDegrade::Unvouched`): "line N would elide if <provider>'s oracle vouched
/// context-tolerance". Recognize-never-license: this is a HINT, never a gate — the site runs/guards
/// regardless. The suggested spelling is the parseable colon-line form.
#[must_use]
pub fn adoption_hint(provider_display: &str, dim: Dimension) -> String {
    format!(
        "would elide if {provider_display}'s oracle vouched context-tolerance \
         (one line: `:   : tolerates:{}`)",
        dim.as_token()
    )
}

/// The authority-DISCLOSURE line for the plan header (`27C:render-authority-disclosure`): one line
/// naming which contexts the probe will enter and under what. `entered` is `(entry-head-display,
/// site-count)` per entered context. Consent legibility — the human sees, once, what escalation the
/// probe re-uses. Empty ⇒ `None` (no wrapped entry ⇒ no disclosure line).
#[must_use]
pub fn authority_disclosure(capability: Capability, entered: &[(String, usize)]) -> Option<String> {
    if entered.is_empty() {
        return None;
    }
    let cap = match capability {
        Capability::Root => "root",
        Capability::NonRootNopasswd => "non-root (NOPASSWD)",
        Capability::Degraded => "degraded",
    };
    let contexts = entered
        .iter()
        .map(|(head, n)| format!("{head} ({n} site{})", if *n == 1 { "" } else { "s" }))
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "probe re-uses connection authority ({cap}): {contexts}; forbid with --no-probe-escalation"
    ))
}

/// Does a verdict body VISIBLY depend on the caller's identity (`27C` §6 idiom-honest-read /
/// idiom-demand-guard)? Recognizes the parseable identity idioms: a `$USER`/`$HOME`/`$LOGNAME` var
/// read, or an `id`/`whoami` command. Consumed for CORROBORATION lints only (recognize-never-
/// license): the recognizer over-triggers on incidental text and under-triggers on spelling variants
/// (only lint-tier tolerates that — `27C:law-perfect-overlap`). Needs the interner to resolve var
/// names (referent-agnostic elsewhere; a lint may read text for a HINT).
#[must_use]
pub fn reads_identity(verdict: &Predict, interner: &Interner) -> bool {
    fn walk(body: &[Stmt], interner: &Interner) -> bool {
        body.iter().any(|s| match s {
            Stmt::Command(c) => {
                matches!(c.words.first(), Some(Word::Literal(w)) if w == "id" || w == "whoami")
                    || c.words.iter().any(|w| is_identity_var(w, interner))
            }
            Stmt::Assign { value, .. } => is_identity_var(value, interner),
            Stmt::Case { arms, .. } => arms.iter().any(|a| walk(&a.body, interner)),
            Stmt::If {
                then_body,
                else_body,
                ..
            } => walk(then_body, interner) || walk(else_body, interner),
            Stmt::While { body, .. } => walk(body, interner),
            _ => false,
        })
    }
    walk(&verdict.body, interner)
}

/// Is `w` a read of an identity variable (`$USER`/`$HOME`/`$LOGNAME`)? Lint-tier text read.
fn is_identity_var(w: &Word, interner: &Interner) -> bool {
    matches!(w, Word::Var(sym) if matches!(interner.resolve(*sym), "USER" | "HOME" | "LOGNAME"))
}

/// A loud corroboration diagnostic code — a `tolerates:` mark sits over VISIBLE identity-dependence.
const TOLERATES_OVER_IDENTITY: dorc_core::DiagCode =
    dorc_core::DiagCode("tolerates-over-identity-dependence");

/// Corroboration lint, forward direction (`27C` §6): a `tolerates:user` mark over a body that
/// VISIBLY reads identity ⇒ "are you sure?" — the vouch claims read-only-under-shift, but the body's
/// answer plainly depends on WHO is asking (a shifted user changes the answer, which is fine, but a
/// mutation-on-shift would not be). A Warning (recognize-never-license): it never blocks, only asks.
/// `None` when there is nothing to corroborate.
#[must_use]
pub fn corroborate_tolerance_over_identity(
    vouch: &ToleranceVouch,
    verdict: &Predict,
    interner: &Interner,
    span: dorc_core::Span,
) -> Option<dorc_core::Diagnostic> {
    (vouch.mentions(Dimension::User) && reads_identity(verdict, interner)).then(|| {
        dorc_core::Diagnostic::warning(
            TOLERATES_OVER_IDENTITY,
            Some(span),
            "this `is_converged` carries `tolerates:user` but VISIBLY reads the caller's identity \
             (`id`/`$USER`/`$HOME`): are you sure the body is read-only under a user shift, not just \
             answer-varying? A shifted user must not make it MUTATE (`27C` §2 corroboration)."
                .to_owned(),
        )
    })
}

/// A one-line adoption-hint diagnostic code (Note-severity — a hint, not a failure).
const HEAVY_CONTEXT_NO_VOUCH: dorc_core::DiagCode =
    dorc_core::DiagCode("heavy-context-no-tolerance");

/// Corroboration lint, reverse direction (`27C` §6): a body doing heavy context-handling (visible
/// identity reads) with NO tolerance mark ⇒ the one-line hint (it would become context-shiftable
/// with a `tolerates:` mark). A Note (recognize-never-license). `None` when the body is already
/// vouched or reads no identity.
#[must_use]
pub fn hint_heavy_context_no_vouch(
    vouch: &ToleranceVouch,
    verdict: &Predict,
    interner: &Interner,
    span: dorc_core::Span,
) -> Option<dorc_core::Diagnostic> {
    (vouch.is_empty() && reads_identity(verdict, interner)).then(|| {
        dorc_core::Diagnostic::note(
            HEAVY_CONTEXT_NO_VOUCH,
            Some(span),
            "this `is_converged` reads the caller's identity but carries no tolerance vouch — a \
             wrapped site (`sudo …`) will run/guard instead of eliding. One line makes it \
             context-shiftable: `:   : tolerates:user` (`27C` §2)."
                .to_owned(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predict::lift_verdicts_converged;

    fn one_enter(src: &str) -> (Interner, Predict) {
        let mut i = Interner::default();
        let set = lift_entry_set(&mut i, src);
        assert!(set.diags.is_empty(), "clean enter lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        let f = set.value.get(p).expect("the enter").clone();
        (i, f)
    }
    fn one_verdict(src: &str) -> (Interner, Predict) {
        let mut i = Interner::default();
        let set = lift_verdicts_converged(&mut i, src);
        assert!(set.diags.is_empty(), "clean verdict lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        let v = set.value.get(p).expect("the verdict").clone();
        (i, v)
    }

    // ── entry-form detection ────────────────────────────────────────────────────
    #[test]
    fn sudo_enter_is_an_entry_form() {
        // `sudo__enter() { sudo -n "$@" ;}` — `"$@"` in ARGUMENT position of `sudo -n` ⇒ an entry
        // form (the complement of a transparent peel). The self-effect span points at the sudo line.
        let (_i, f) = one_enter("sudo__enter() { sudo -n \"$@\" ;}");
        let form = detect_entry_form(&f).expect("sudo -n \"$@\" is an entry form");
        assert_eq!(
            form.head.len(),
            2,
            "head = [sudo, -n] (two words before the guest)"
        );
    }
    #[test]
    fn bare_at_is_not_an_entry_form() {
        // `nice__enter() { "$@" ;}` — a bare `"$@"` is a TRANSPARENT peel (command-position), not an
        // entry form (empty head). An identity wrapper enters no context, so it has no entry form.
        let (_i, f) = one_enter("nice__enter() { \"$@\" ;}");
        assert_eq!(detect_entry_form(&f), None);
    }
    #[test]
    fn chroot_dir_guest_is_an_entry_form() {
        // `chroot__enter() { chroot /mnt/target "$@" ;}` — the guest `"$@"` is the trailing
        // argument to `chroot <dir>` (an fs-view entry form).
        let (_i, f) = one_enter("chroot__enter() { chroot /mnt/target \"$@\" ;}");
        let form = detect_entry_form(&f).expect("chroot <dir> \"$@\" is an entry form");
        assert_eq!(
            form.head,
            vec!["chroot".to_owned(), "/mnt/target".to_owned()]
        );
    }

    // ── the tolerance vouch ──────────────────────────────────────────────────────
    // NB the parseable spelling is the colon-line `:   : tolerates:<dim>` (no-op `:` command + a
    // `: target` mark, exactly like the corpus `:   : invariant:user`), NOT the spec §2 STRAWMAN
    // shorthand `: tolerates:<dim>` (a single `:` is a no-op command with an argument, no mark).
    #[test]
    fn top_level_tolerates_is_unconditional() {
        // A `tolerates:user` mark at the top of the body vouches user for EVERY verb (the babby
        // template).
        let (_i, v) = one_verdict(
            "pipx__is_converged() { :   : tolerates:user\n verb=$1; shift; case \"$verb\" in \
             install) pipx list ;; *) return 2 ;; esac }",
        );
        let (vouch, diags) = lift_tolerance(&v);
        assert!(diags.is_empty(), "clean tolerance lift: {diags:?}");
        assert!(
            vouch
                .tolerated_on_path(Some("install"))
                .contains(&Dimension::User)
        );
        assert!(
            vouch
                .tolerated_on_path(Some("anything"))
                .contains(&Dimension::User),
            "unconditional ⇒ every verb"
        );
    }
    #[test]
    fn arm_scoped_tolerates_is_per_verb() {
        // A `tolerates:` mark inside the `install` arm vouches ONLY the install path (reachability-
        // scoped): a `remove` site is NOT licensed to shift (the safe direction).
        let (_i, v) = one_verdict(
            "pipx__is_converged() { verb=$1; shift; case \"$verb\" in \
             install) :   : tolerates:user\n pipx list ;; remove) pipx list ;; *) return 2 ;; esac }",
        );
        let (vouch, _d) = lift_tolerance(&v);
        assert!(
            vouch
                .tolerated_on_path(Some("install"))
                .contains(&Dimension::User)
        );
        assert!(
            !vouch
                .tolerated_on_path(Some("remove"))
                .contains(&Dimension::User),
            "arm-scoped tolerance does not reach a different verb"
        );
    }
    #[test]
    fn brace_alternation_tolerates_multiple_dimensions() {
        let (_i, v) =
            one_verdict("x__is_converged() { :   : tolerates:{user,fs-view}\n return 0 }");
        let (vouch, diags) = lift_tolerance(&v);
        assert!(diags.is_empty(), "clean: {diags:?}");
        let dims = vouch.tolerated_on_path(None);
        assert!(dims.contains(&Dimension::User) && dims.contains(&Dimension::FsView));
    }
    #[test]
    fn unknown_tolerates_dimension_is_loud() {
        let (_i, v) = one_verdict("x__is_converged() { :   : tolerates:universe\n return 0 }");
        let (vouch, diags) = lift_tolerance(&v);
        assert_eq!(diags.len(), 1, "one loud diag: {diags:?}");
        assert_eq!(diags[0].code, TOLERATES_UNKNOWN_DIMENSION);
        assert!(vouch.is_empty(), "an unknown token vouches nothing");
    }

    // ── the composition algebra ──────────────────────────────────────────────────
    fn link(provider: u32, shifts: &[(Dimension, Shift)]) -> ChainLink {
        ChainLink {
            provider: sym(provider),
            shifts: shifts.iter().cloned().collect(),
        }
    }
    fn sym(n: u32) -> Symbol {
        // A deterministic symbol via a throwaway interner keyed on the number.
        let mut i = Interner::default();
        // intern n placeholder strings so the nth symbol has id n-ish; simplest: intern the number.
        for k in 0..=n {
            i.intern(&format!("provider{k}"));
        }
        i.intern(&format!("provider{n}"))
    }

    #[test]
    fn top_propagates_across_the_chain() {
        // `27C` §3: a MISSING key at ANY link ⇒ ⊤ for that dimension chain-wide (⊤ propagates).
        let chain = [
            link(
                0,
                &[
                    (Dimension::User, Shift::Mapped("root".into())),
                    (Dimension::FsView, Shift::Full),
                    (Dimension::Netns, Shift::Full),
                ],
            ),
            link(
                1,
                &[
                    (Dimension::User, Shift::Full),
                    (Dimension::FsView, Shift::Top),
                    (Dimension::Netns, Shift::Full),
                ],
            ),
        ];
        let c = compose_chain(&chain);
        assert_eq!(
            c.shift(Dimension::User),
            Shift::Mapped("root".into()),
            "user shifts (mapped ∘ full)"
        );
        assert_eq!(
            c.shift(Dimension::FsView),
            Shift::Top,
            "one ⊤ link walls fs-view chain-wide"
        );
        assert_eq!(c.walls(), vec![Dimension::FsView]);
    }
    #[test]
    fn full_lend_is_the_composition_identity() {
        let chain = [
            link(
                0,
                &[
                    (Dimension::User, Shift::Full),
                    (Dimension::FsView, Shift::Full),
                    (Dimension::Netns, Shift::Full),
                ],
            ),
            link(
                1,
                &[
                    (Dimension::User, Shift::Full),
                    (Dimension::FsView, Shift::Full),
                    (Dimension::Netns, Shift::Full),
                ],
            ),
        ];
        let mut i = Interner::default();
        let c = compose_chain(&chain);
        assert!(c.crossed().is_empty(), "an all-full chain shifts nothing");
        assert_eq!(
            c.to_context(&mut i),
            Context::HostDefault,
            "identity chain ⇒ HostDefault (rung-0)"
        );
    }
    #[test]
    fn nested_permutation_pins_distinct_context_keys() {
        // `27C` §3 nested-permutation pin: `sudo chroot` vs `chroot sudo` compose to DIFFERENT
        // context keys where the dimensions differ (order-sensitive). sudo shifts user, chroot shifts
        // fs-view; reversing the chain reverses the canonical string ⇒ distinct keys.
        let sudo = link(
            0,
            &[
                (Dimension::User, Shift::Mapped("root".into())),
                (Dimension::FsView, Shift::Full),
                (Dimension::Netns, Shift::Full),
            ],
        );
        let chroot = link(
            1,
            &[
                (Dimension::User, Shift::Full),
                (Dimension::FsView, Shift::Mapped("/mnt".into())),
                (Dimension::Netns, Shift::Full),
            ],
        );
        let mut i = Interner::default();
        let sudo_chroot = compose_chain(&[sudo.clone(), chroot.clone()]).to_context(&mut i);
        let chroot_sudo = compose_chain(&[chroot, sudo]).to_context(&mut i);
        assert_ne!(
            sudo_chroot, chroot_sudo,
            "order-sensitive: sudo∘chroot ≠ chroot∘sudo"
        );
        // Both are Wrapped (both shift two dimensions).
        assert!(matches!(sudo_chroot, Context::Wrapped(_)));
    }
    #[test]
    fn same_chain_same_context_key() {
        // Two `sudo`-wrapped sites (different inner command) share ONE context — the key is a
        // function of the WRAPPER chain, not the inner. So a re-measurement self-heals to the same
        // fact slot.
        let sudo = link(
            0,
            &[
                (Dimension::User, Shift::Mapped("root".into())),
                (Dimension::FsView, Shift::Full),
                (Dimension::Netns, Shift::Full),
            ],
        );
        let mut i = Interner::default();
        let a = compose_chain(std::slice::from_ref(&sudo)).to_context(&mut i);
        let b = compose_chain(&[sudo]).to_context(&mut i);
        assert_eq!(a, b, "same wrapper chain ⇒ same context key");
    }
    #[test]
    fn disagreeing_mapped_values_wall() {
        // Two links mapping the SAME dimension to DIFFERENT targets is a genuine two-world conflict ⇒
        // ⊤ (the safe wall, never a false same-world merge).
        let chain = [
            link(
                0,
                &[
                    (Dimension::User, Shift::Mapped("root".into())),
                    (Dimension::FsView, Shift::Full),
                    (Dimension::Netns, Shift::Full),
                ],
            ),
            link(
                1,
                &[
                    (Dimension::User, Shift::Mapped("bob".into())),
                    (Dimension::FsView, Shift::Full),
                    (Dimension::Netns, Shift::Full),
                ],
            ),
        ];
        let c = compose_chain(&chain);
        assert_eq!(
            c.shift(Dimension::User),
            Shift::Top,
            "disagreeing mapped targets ⇒ ⊤ wall"
        );
    }

    // ── the two-axis consent decision ────────────────────────────────────────────
    #[test]
    fn consent_default_enters_only_vouched() {
        let user = BTreeSet::from([Dimension::User]);
        // Default dial, root capability, user crossed, user vouched ⇒ ENTER.
        assert_eq!(
            decide_entry(
                true,
                Capability::Root,
                EscalationDial::VouchedOnly,
                &[Dimension::User],
                &[],
                &user
            ),
            EntryDecision::Enter
        );
        // Same, but UNVOUCHED (empty tolerance) ⇒ degrade (the one-line hint fires).
        assert_eq!(
            decide_entry(
                true,
                Capability::Root,
                EscalationDial::VouchedOnly,
                &[Dimension::User],
                &[],
                &BTreeSet::new()
            ),
            EntryDecision::Degrade(EntryDegrade::Unvouched(Dimension::User))
        );
    }
    #[test]
    fn consent_no_escalation_never_enters() {
        let user = BTreeSet::from([Dimension::User]);
        assert_eq!(
            decide_entry(
                true,
                Capability::Root,
                EscalationDial::NoEscalation,
                &[Dimension::User],
                &[],
                &user
            ),
            EntryDecision::Degrade(EntryDegrade::DialForbids),
            "--no-probe-escalation never shifts, even vouched"
        );
    }
    #[test]
    fn consent_any_probe_overrides_vouch() {
        assert_eq!(
            decide_entry(
                true,
                Capability::Root,
                EscalationDial::AnyProbe,
                &[Dimension::User],
                &[],
                &BTreeSet::new()
            ),
            EntryDecision::Enter,
            "--escalate-any-probe enters unvouched (admin owns the blast-radius)"
        );
    }
    #[test]
    fn capability_bounds_before_dial() {
        // A non-root connection cannot effect the netns (root-only) dimension — capability walls
        // BEFORE the dial is consulted (the host fact bounds everything).
        assert_eq!(
            decide_entry(
                true,
                Capability::NonRootNopasswd,
                EscalationDial::AnyProbe,
                &[Dimension::Netns],
                &[],
                &BTreeSet::new()
            ),
            EntryDecision::Degrade(EntryDegrade::NoCapability(Dimension::Netns))
        );
        // But a NOPASSWD non-root CAN effect the user dimension (sudo -n class).
        assert_eq!(
            decide_entry(
                true,
                Capability::NonRootNopasswd,
                EscalationDial::AnyProbe,
                &[Dimension::User],
                &[],
                &BTreeSet::new()
            ),
            EntryDecision::Enter
        );
        // Degraded does none.
        assert_eq!(
            decide_entry(
                true,
                Capability::Degraded,
                EscalationDial::AnyProbe,
                &[Dimension::User],
                &[],
                &BTreeSet::new()
            ),
            EntryDecision::Degrade(EntryDegrade::NoCapability(Dimension::User))
        );
    }
    #[test]
    fn no_entry_form_walls() {
        assert_eq!(
            decide_entry(
                false,
                Capability::Root,
                EscalationDial::AnyProbe,
                &[Dimension::User],
                &[],
                &BTreeSet::new()
            ),
            EntryDecision::Degrade(EntryDegrade::NoEntryForm),
            "a wrapper with no entry form never enters"
        );
    }
    #[test]
    fn walled_dimension_degrades() {
        assert_eq!(
            decide_entry(
                true,
                Capability::Root,
                EscalationDial::AnyProbe,
                &[],
                &[Dimension::Netns],
                &BTreeSet::new()
            ),
            EntryDecision::Degrade(EntryDegrade::TopDimension(Dimension::Netns))
        );
    }
    #[test]
    fn capability_permits_matrix() {
        assert!(capability_permits(Capability::Root, Dimension::Netns));
        assert!(capability_permits(
            Capability::NonRootNopasswd,
            Dimension::User
        ));
        assert!(!capability_permits(
            Capability::NonRootNopasswd,
            Dimension::FsView
        ));
        assert!(!capability_permits(Capability::Degraded, Dimension::User));
    }

    // ── §6 mined-idiom lints + disclosure (recognize, never license) ──────────────
    #[test]
    fn corroboration_fires_both_directions() {
        // Forward (`27C` §6): `tolerates:user` over a body that reads `$USER` ⇒ "are you sure?".
        let (i, v) = one_verdict(
            "x__is_converged() { :   : tolerates:user\n me=$USER; case \"$me\" in \
             root) return 0 ;; *) return 1 ;; esac }",
        );
        let (vouch, _d) = lift_tolerance(&v);
        let span = dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1));
        assert!(
            corroborate_tolerance_over_identity(&vouch, &v, &i, span).is_some(),
            "a tolerance mark over visible identity-dependence corroborates"
        );
        // Reverse (`27C` §6): heavy context-handling (`$USER`) with NO mark ⇒ the one-line hint.
        let (i2, v2) = one_verdict(
            "y__is_converged() { me=$USER; case \"$me\" in root) return 0 ;; *) return 1 ;; esac }",
        );
        let (vouch2, _d2) = lift_tolerance(&v2);
        assert!(
            hint_heavy_context_no_vouch(&vouch2, &v2, &i2, span).is_some(),
            "identity-handling with no vouch fires the adoption hint"
        );
        // A plain body (no identity read) corroborates nothing in either direction.
        let (i3, v3) = one_verdict("z__is_converged() { dpkg -s nginx ;}");
        let (vouch3, _d3) = lift_tolerance(&v3);
        assert!(corroborate_tolerance_over_identity(&vouch3, &v3, &i3, span).is_none());
        assert!(hint_heavy_context_no_vouch(&vouch3, &v3, &i3, span).is_none());
    }

    #[test]
    fn reads_identity_recognizes_id_command() {
        let (i, v) = one_verdict("w__is_converged() { id ;}");
        assert!(
            reads_identity(&v, &i),
            "an `id` command is identity-dependence"
        );
    }

    #[test]
    fn authority_disclosure_names_contexts_or_is_silent() {
        // No wrapped entry ⇒ no disclosure line.
        assert_eq!(authority_disclosure(Capability::Root, &[]), None);
        // Entered contexts ⇒ one legible line naming them + the opt-out flag.
        let line = authority_disclosure(
            Capability::Root,
            &[
                ("sudo -n".to_owned(), 1),
                ("chroot /mnt/target".to_owned(), 2),
            ],
        )
        .expect("entered contexts disclose");
        assert!(line.contains("root"));
        assert!(line.contains("sudo -n (1 site)"));
        assert!(line.contains("chroot /mnt/target (2 sites)"));
        assert!(line.contains("--no-probe-escalation"));
    }

    #[test]
    fn adoption_hint_suggests_the_parseable_spelling() {
        let hint = adoption_hint("pipx", Dimension::User);
        assert!(
            hint.contains(":   : tolerates:user"),
            "suggests the colon-line form"
        );
    }
}
