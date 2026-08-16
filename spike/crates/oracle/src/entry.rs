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

use dorc_aid::diag::{
    Diag, DiagCode, HeavyContextNoTolerance, ToleratesOverIdentityDependence,
    ToleratesUnknownDimension,
};
use dorc_core::{Capability, Context, ContextKey, EscalationDial, Interner, Symbol};

use crate::predict::{MarkKind, Predict, PredictSet, Stmt, Word};
use crate::wrapper::{Dimension, LendEntry, LendMap, RhoClaim};

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
        _ => "...".to_owned(),
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

/// The wrapper's entry-form member suffix. `pub` for [`crate::predict::PREDICT_SUFFIX`]'s reason:
/// the frame lookup asks the function environment for the exact NAME a lifted member was authored
/// under, and re-spelling a suffix at the consumer is how one drifts.
pub const ENTER_SUFFIX: &str = "__enter";

/// Lift every `<provider>__enter` in `src` into a [`PredictSet`] (COMMAND-keyed like the wrapper's
/// `predict`/`lend_map`). The consumer calls [`detect_entry_form`] per body. Same fail-soft contract as
/// `lift_predicts`.
pub fn lift_entry_set(interner: &mut Interner, src: &str) -> dorc_aid::Carrier<PredictSet> {
    crate::predict::lift_enters(interner, src)
}

/// A fold-entry coherence failure (`27C:rul-fold-entry-coherence-failfast`, HUMAN-ACKED 2026-07-17):
/// the entry form's argparse (leading argv it consumes before the guest) DISAGREES with the paired
/// `lend_map`'s. The declarations-genuinely-contradict category (dual-peel pattern, third instance)
/// ⇒ plan-time fail-fast, pre-network. STATIC sh-structure ONLY.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryIncoherence {
    /// Leading argv tokens the entry form consumes (top-level `shift`s) before its guest `"$@"`.
    pub entry_shifts: usize,
    /// Leading argv tokens the `lend_map`'s fold consumes before its guest.
    pub lend_shifts: usize,
}

/// Fold-entry coherence (`27C:rul-fold-entry-coherence-failfast`, HUMAN-ACKED): does the entry form
/// agree with the lend-fold on the ARGV FLOW, by STATIC sh-structure alone? The scope is EXACTLY
/// sh-structure (`inv-referent-agnostic`): whether an entry invocation actually EFFECTS the declared
/// shifts is TOOL-SEMANTICS the engine never holds — that is the traversal vouch (a wrong one is an
/// attributed authored error, `hole-bad-oracle-blast`), NEVER statically detected or fail-fasted.
///
/// The ONE static trigger this implements soundly: an ARGPARSING entry form (one that `shift`s
/// leading args) must consume the SAME leading args the lend-fold did — else it drops/transforms
/// args the fold relied on. A TRIVIAL RE-PASS entry (`sudo -n "$@"`, zero shifts) delegates ALL
/// parsing to the real tool and is coherent by construction (never false-failed). Bodies with
/// intervening control-flow (a flag-strip `while`) are conservatively SKIPPED (`None`) — the finer
/// loop-shaped coherence is not covered here (a narrow-scope disclosure, `ru-26`). The coarse
/// no-entry-member case is NOT this rule — it walls per §4/§5 ([`EntryDegrade::NoEntryForm`]).
#[must_use]
pub fn check_entry_coherence(enter: &Predict, lend_map: &Predict) -> Option<EntryIncoherence> {
    let entry_shifts = leading_shifts_before_guest(&enter.body)?;
    let lend_shifts = leading_shifts_before_guest(&lend_map.body)?;
    // A trivial re-pass entry (0 shifts) delegates to the real tool ⇒ coherent regardless of the
    // fold. An argparsing entry must match the fold's consumption.
    (entry_shifts > 0 && entry_shifts != lend_shifts).then_some(EntryIncoherence {
        entry_shifts,
        lend_shifts,
    })
}

/// Count the top-level `shift`s a body consumes before the first command whose LAST word is the
/// guest `"$@"` (any terminal guest — command-position bare, `env … "$@"`, or an entry `head … "$@"`).
/// `None` if intervening control-flow (`while`/`case`/`if`) is hit before the guest (conservatively
/// un-comparable) or no guest is found. The simple, sound tail-consumption for the fixture-tier
/// forms; a flag-strip `while` yields `None` (skip).
fn leading_shifts_before_guest(body: &[Stmt]) -> Option<usize> {
    let mut consumed: usize = 0;
    for stmt in body {
        match stmt {
            Stmt::Shift { count } => {
                consumed = consumed.saturating_add(count.unwrap_or(1) as usize);
            }
            Stmt::Command(c)
                if !c.pipeline && matches!(c.words.last(), Some(Word::PositionalArgs)) =>
            {
                return Some(consumed);
            }
            // Assigns, annotations, and non-guest COMMANDS (a `printf` mapped-lend line, a colon-line
            // mark) consume NO positionals — skip them, counting only `shift`s.
            Stmt::Assign { .. } | Stmt::Annotation(_) | Stmt::Command(_) => {}
            // Control-flow before the guest may consume argv unpredictably ⇒ un-comparable.
            Stmt::While { .. } | Stmt::Case { .. } | Stmt::If { .. } | Stmt::AndOr(_) => {
                return None;
            }
        }
    }
    None
}

// ===========================================================================
// The tolerance vouch (`27C` §2 — the oracle surface)
// ===========================================================================

/// A lifted `safe-across` vouch over a verdict body (`27C:vouch-tolerates`): per-function, per-
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

/// Lift the [`ToleranceVouch`] from a verdict body (`27C` §2 — the `is_converged` member). Walks the
/// body for `: safe-across <dim>` colon-lines: a mark at top level is unconditional; a mark inside a
/// `case` arm is scoped to that arm's verb patterns. Brace-alternation `safe-across {user,fs-view}`
/// expands to a per-dimension set. An unknown dimension token is a LOUD diagnostic (`inv-top-reject`)
/// that mints no tolerance. Pure/total.
#[must_use]
pub fn lift_tolerance(verdict: &Predict) -> (ToleranceVouch, Vec<Diag>) {
    let mut vouch = ToleranceVouch::default();
    let mut diags = Vec::new();
    collect_tolerance(&verdict.body, None, &mut vouch, &mut diags);
    (vouch, diags)
}

/// Recursively collect `safe-across` marks. `arm` is `Some(patterns, catch_all)` when inside a `case`
/// arm (the marks scope to it), `None` at top level (unconditional).
fn collect_tolerance(
    body: &[Stmt],
    arm: Option<(&[String], bool)>,
    vouch: &mut ToleranceVouch,
    diags: &mut Vec<Diag>,
) {
    for stmt in body {
        match stmt {
            Stmt::Command(c) => {
                let Some(mark) = &c.mark else { continue };
                if mark.kind != MarkKind::SafeAcross {
                    continue;
                }
                // The dimension(s) live in the mark's `kind` fragment — the uniform token-payload
                // home (`28A:rul-uniform-kind-payload-home`): `safe-across user` ⇒ kind="user";
                // `safe-across {user,fs-view}` ⇒ kind="{user,fs-view}" (`281` §5/§6).
                let raw = mark.target.kind.as_str();
                let mut dims = BTreeSet::new();
                for tok in expand_dimension_set(raw) {
                    match Dimension::from_token(&tok) {
                        Some(d) => {
                            dims.insert(d);
                        }
                        None => diags.push(Diag::new(
                            DiagCode::ToleratesUnknownDimension(ToleratesUnknownDimension {
                                token: tok.clone(),
                                expected: Dimension::ALL.map(Dimension::as_token).join(", "),
                            }),
                            mark.span,
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

/// Expand a `safe-across` dimension fragment into tokens: a bare `user` ⇒ `[user]`; a brace-set
/// `{user,fs-view}` ⇒ `[user, fs-view]` (`27C` §2 brace-alternation). Trims whitespace; empty
/// members are dropped. Referent-agnostic string surgery (the tokens are validated against the
/// closed dimension vocabulary by the caller).
pub(crate) fn expand_dimension_set(raw: &str) -> Vec<String> {
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
    /// A canonical tag for this shift in the folded NORMAL-FORM key (`27C` §3 ruling 4). Referent-
    /// agnostic identity string — compared for equality, never decoded.
    #[must_use]
    fn tag(&self) -> String {
        match self {
            Shift::Full => "F".to_owned(),
            Shift::Top => "T".to_owned(),
            Shift::Mapped(v) => format!("M:{v}"),
        }
    }
}

/// The engine-internal compose OP a dimension fixes ONCE (`27C:rul-dimension-owned-compose-ops`,
/// HUMAN-ACKED) — NEVER on the authored surface. Wrapper authors emit single-step strings (`273`
/// §3) and never reason about nesting; the engine applies the dimension's op to opaque values
/// (`inv-referent-agnostic` holds).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DimOp {
    /// ABSOLUTE overwrite: a map denotes its value regardless of the caller (`sudo -u alice` = alice
    /// whoever called; a netns name is absolute). The inner (last-entered) map wins. user · netns.
    AbsoluteOverwrite,
    /// CALLER-RELATIVE: a map composes with the caller's value by path (chroot-in-chroot:
    /// `chroot /t` inside `chroot /mnt` = `/mnt/t`; relative-to-⊤ is ⊤). fs-view.
    CallerRelative,
}

/// The compose op each dimension OWNS (`27C:rul-dimension-owned-compose-ops`). Fixed once, engine-
/// internal: user/netns are absolute (a uid/nsname denotes an absolute target); fs-view is
/// caller-relative (paths nest under the caller's root).
#[must_use]
fn dimension_op(dim: Dimension) -> DimOp {
    match dim {
        Dimension::User | Dimension::Netns => DimOp::AbsoluteOverwrite,
        Dimension::FsView => DimOp::CallerRelative,
    }
}

/// Fold one more (inner) link's shift into the accumulated shift for a dimension (`27C` §3): apply
/// the dimension's owned op, with ⊤ STICKY (`27C:rul-top-absorbs-absolute-maps`, HUMAN-ACKED — NO
/// overwrite-rescue through an inner absolute map: once ⊤, stays ⊤; a link's own ⊤ absorbs). `Full`
/// is the identity (pass-through). Folds outermost→innermost.
#[must_use]
fn compose_shift(acc: Shift, link: Shift, op: DimOp) -> Shift {
    match (acc, link) {
        // ⊤ STICKY both ways: a ⊤ accumulator (an unknown outer/middle link) OR the link's own ⊤
        // ⇒ ⊤ chain-wide. An inner absolute map does NOT rescue a ⊤ middle (`rul-top-absorbs-
        // absolute-maps`: whether the inner even executes, and how its argv resolved, depend on the
        // unknown middle — skip-the-middle holds in raw sh-analysis, NEVER in machine-state logic).
        (Shift::Top, _) | (_, Shift::Top) => Shift::Top,
        // A Full link borrows the caller's value on this dimension — pass-through (the identity).
        (acc, Shift::Full) => acc,
        // A mapped link over a Full accumulator: the map takes effect (absolute, or relative-to-root).
        (Shift::Full, Shift::Mapped(v)) => Shift::Mapped(v),
        // Both mapped: the dimension's owned op composes them.
        (Shift::Mapped(base), Shift::Mapped(v)) => match op {
            DimOp::AbsoluteOverwrite => Shift::Mapped(v), // inner wins; caller-independent
            DimOp::CallerRelative => Shift::Mapped(join_path(&base, &v)), // paths nest
        },
    }
}

/// Compose two fs-view paths (caller-relative, `27C:rul-dimension-owned-compose-ops`): `base` then
/// `rel` ⇒ `base/rel` (`chroot /t` inside `chroot /mnt` = `/mnt/t`). A leading `/` on `rel` still
/// nests — chroot is always relative to the CURRENT root (`/etc` inside `chroot /mnt` is `/mnt/etc`).
/// Referent-agnostic string surgery over opaque path values.
#[must_use]
fn join_path(base: &str, rel: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        rel.trim_start_matches('/')
    )
}

/// The ρ accumulator for the normal-form key (`27C` §3 ruling 3/4 — ρ threads through every link).
/// A bare-`"$@"` link ([`RhoClaim::Nothing`]) is the IDENTITY (a `nice`-style pass-through — it
/// perturbs no world, so nice-permutations share ONE key, ruling 4). An `env`-syllable claim
/// contributes: `FullAmbient`/`PerVariable` ADD overrides on top; `ExactlyThese` SCRUBS the base
/// (so `env A=1 sudo` ≠ `sudo env A=1` — sudo scrubs A away).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RhoAccum {
    /// `Some(vars)` once a scrub (`env -i`/`ExactlyThese`) reset the base to exactly those vars;
    /// `None` = the ambient-passthrough base (identity).
    scrubbed: Option<BTreeSet<String>>,
    /// Overrides added SINCE the last scrub (`env VAR=v`/`VAR=v "$@"`).
    overrides: BTreeSet<String>,
}

impl RhoAccum {
    /// Fold one more (inner) link's ρ-claim (`27C` §3 ruling 3). `Nothing` is identity.
    fn fold(mut self, rho: &RhoClaim) -> Self {
        match rho {
            RhoClaim::Nothing => self, // identity (nice); claims no env transform for the KEY
            RhoClaim::FullAmbient { overrides } => {
                self.overrides.extend(overrides.iter().cloned());
                self
            }
            RhoClaim::ExactlyThese { vars } => {
                // A scrub discards the accumulated ambient/overrides and resets to exactly `vars`.
                self.scrubbed = Some(vars.iter().cloned().collect());
                self.overrides.clear();
                self
            }
            RhoClaim::PerVariable { vars } => {
                // Per-variable claim (`VAR=x "$@"`): add the vars (symbol-id tags, referent-agnostic).
                self.overrides
                    .extend(vars.iter().map(|s| format!("@{}", s.as_u32())));
                self
            }
        }
    }

    /// The canonical ρ tag for the key — EMPTY when identity (no scrub, no overrides), so a pure
    /// `nice`/`env "$@"` chain contributes nothing (ruling 4).
    fn tag(&self) -> String {
        let scrub = self.scrubbed.as_ref().map_or(String::new(), |vars| {
            format!(
                "scrub:{}",
                vars.iter().cloned().collect::<Vec<_>>().join(",")
            )
        });
        let ovr = if self.overrides.is_empty() {
            String::new()
        } else {
            format!(
                "ovr:{}",
                self.overrides.iter().cloned().collect::<Vec<_>>().join(",")
            )
        };
        [scrub, ovr]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(";")
    }
}

/// One link (one wrapper) in a peel chain, resolved for a site (`27C` §3): its per-dimension shift
/// plus its ρ-claim. The composition folds a chain outermost-first (entry order = book order). The
/// provider is NOT a key component — the key is the folded NORMAL FORM (ruling 4), so an identity
/// wrapper (`nice`) folds away regardless of its provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainLink {
    /// This wrapper's shift per dimension (an absent dimension ⇒ [`Shift::Top`] via [`Self::shift`]).
    pub shifts: BTreeMap<Dimension, Shift>,
    /// This wrapper's ρ-claim (read off its predict env-head; `271:rul-env-claim-inversion`). Threads
    /// through the fold into the normal-form key (ruling 3/4).
    pub rho: RhoClaim,
}

impl ChainLink {
    /// This link's shift for `dim` — an ABSENT dimension is [`Shift::Top`] (the enumerate-every-
    /// dimension law; `271:rul-lend-map`).
    #[must_use]
    pub fn shift(&self, dim: Dimension) -> Shift {
        self.shifts.get(&dim).cloned().unwrap_or(Shift::Top)
    }

    /// Build a link from a wrapper's [`LendMap`] + ρ-claim and a per-dimension mapped-value resolver.
    /// `Full`/`Top` come straight from the lend map; a `Mapped` dimension calls `resolve(dim)` for
    /// its value (`None` ⇒ [`Shift::Top`], the safe wall for an unresolved mapped target — e.g. an
    /// unresolvable `sudo -u "$VAR"` under an unknown ρ). The mapped value is expected ρ-RESOLVED by
    /// the caller under the ρ composed so far (`27C` §3 ruling 3 cross-link ρ-threading).
    #[must_use]
    pub fn from_lend_map(
        lend: &LendMap,
        rho: RhoClaim,
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
        Self { shifts, rho }
    }
}

/// The composed inner context of a peel chain (`27C` §3): the folded per-dimension NORMAL FORM plus
/// its canonical key. Built by [`compose_chain`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposedContext {
    /// The composed shift per dimension (the per-dimension owned-op fold).
    per_dimension: BTreeMap<Dimension, Shift>,
    /// The composed ρ tag (empty when identity).
    rho_tag: String,
}

impl ComposedContext {
    /// The composed shift for `dim`. An absent dimension is [`Shift::Top`].
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

    /// The folded NORMAL-FORM canonical string (`27C` §3 ruling 4): the per-dimension composed
    /// values + the composed ρ tag — NEVER the chain's syntax. Order-sensitive EXACTLY where the
    /// folds genuinely differ (nice-permutations share it; a scrub-reorder does not). Used to key
    /// the fact plane AND to batch entered segments (both consume this ONE key).
    #[must_use]
    pub fn canonical(&self) -> String {
        let dims = Dimension::ALL
            .into_iter()
            .map(|d| format!("{}={}", d.as_token(), self.shift(d).tag()))
            .collect::<Vec<_>>()
            .join(";");
        if self.rho_tag.is_empty() {
            dims
        } else {
            format!("{dims}|rho:{}", self.rho_tag)
        }
    }

    /// The [`Context`] this composition denotes (`27C` §3 / the `27L` `FactKey` seam). An identity
    /// chain (every dimension `Full`, ρ identity) is [`Context::HostDefault`] — the inner sits in the
    /// caller's world, so its facts key exactly as an unwrapped site's (rung-0). Any shift, wall, or
    /// ρ-transform mints [`Context::Wrapped`] keyed by the interned NORMAL-FORM canonical.
    #[must_use]
    pub fn to_context(&self, interner: &mut Interner) -> Context {
        let identity = self.rho_tag.is_empty()
            && Dimension::ALL
                .into_iter()
                .all(|d| matches!(self.shift(d), Shift::Full));
        if identity {
            Context::HostDefault
        } else {
            Context::Wrapped(ContextKey(interner.intern(&self.canonical())))
        }
    }
}

/// Compose a peel chain into its inner [`ComposedContext`] (`27C` §3, HUMAN-ACKED rulings
/// 2026-07-17): the pointwise fold, outermost→innermost, per dimension via the dimension's OWNED op
/// (`rul-dimension-owned-compose-ops` — user/netns absolute, fs-view path-relative), with ⊤ STICKY
/// (`rul-top-absorbs-absolute-maps` — no rescue). ρ threads through into the normal-form key
/// (ruling 3/4). `chain` is outermost-first (`sudo chroot CMD` ⇒ `[sudo, chroot]`). The key is the
/// folded NORMAL FORM, so nice-permutations collapse and only genuine fold differences key apart.
#[must_use]
pub fn compose_chain(chain: &[ChainLink]) -> ComposedContext {
    let mut per_dimension = BTreeMap::new();
    for dim in Dimension::ALL {
        let op = dimension_op(dim);
        let composed = chain.iter().fold(Shift::Full, |acc, link| {
            compose_shift(acc, link.shift(dim), op)
        });
        per_dimension.insert(dim, composed);
    }
    let rho_tag = chain
        .iter()
        .fold(RhoAccum::default(), |acc, link| acc.fold(&link.rho))
        .tag();
    ComposedContext {
        per_dimension,
        rho_tag,
    }
}

// ===========================================================================
// Book-side chain peeling (lane-integration `27N`): turn a wrapped BOOK site's argv into
// (inner command, composed context, chain) — the seam the classify/probe/plan pipeline consumes.
// ===========================================================================

/// A loaded wrapper's models, keyed by book command word in a [`WrapperIndex`] (`27N`). Built at the
/// cli edge from the oracle sources (the wrapper's peeling `__predict`, its `__lend_map`, its
/// `__enter`). The book pipeline consults this to recognize a wrapped site and peel it.
#[derive(Debug, Clone)]
pub struct WrapperModel {
    /// The peeling `__predict` (its argparse counts the flags consumed before the guest — the inner
    /// command's start; [`crate::wrapper::peel_consumed`]).
    pub predict: Predict,
    /// The ρ-claim read off the predict's peel command ([`crate::wrapper::detect_peel`]).
    pub rho: RhoClaim,
    /// The derived per-dimension lend (`derive_lend_map`; all-⊤ when no `__lend_map` ⇒ the site
    /// walls, `271:rul-lend-map`).
    pub lend: LendMap,
    /// The `__lend_map` predict, for per-site mapped-value resolution
    /// ([`crate::wrapper::resolve_lend_values`]). `None` ⇒ no mapped values (every mapped dimension
    /// is ⊤).
    pub lend_map: Option<Predict>,
    /// The `__enter` form, if authored (the ONE licensed seat for real context entry, `27C` §3).
    /// `None` ⇒ the wrapper's contexts are never entered ([`EntryDegrade::NoEntryForm`]).
    pub enter: Option<EntryForm>,
    /// The wrapper's provider symbol (display / dedup keying).
    pub provider: Symbol,
}

/// The loaded wrappers, keyed by the book command word (`sudo`, `chroot`, `nice`). The cli builds
/// it; [`peel_book_chain`] consults it.
pub type WrapperIndex = BTreeMap<String, WrapperModel>;

/// One peeled link's identity for the entry-composed probe + the authority disclosure (`27N`).
#[derive(Debug, Clone)]
pub struct ChainLinkId {
    /// The wrapper's provider symbol.
    pub provider: Symbol,
    /// The wrapper's entry form, if authored (drives the entry-composed shipping + degrade).
    pub entry: Option<EntryForm>,
}

/// A wrapped book site peeled into its inner command + composed context (`27C` §3 / `27N`). The
/// `analysis` classify uses `inner_argv` to resolve the fact and re-keys it into `composed`'s
/// [`Context`]; the probe/plan lanes use `links` (outermost-first entry forms) to ship the
/// entry-composed check and to trace the consent decision.
#[derive(Debug, Clone)]
pub struct PeeledChain {
    /// The inner (non-wrapper) command's full argv (its command word FIRST) — what `command_effect`
    /// resolves against the inner oracle.
    pub inner_argv: Vec<String>,
    /// The composed inner context (the per-dimension fold + ρ; [`compose_chain`]).
    pub composed: ComposedContext,
    /// The peel chain, outermost-first (`sudo chroot …` ⇒ `[sudo, chroot]`).
    pub links: Vec<ChainLinkId>,
}

/// Peel a wrapped BOOK site's fully-resolved argv into its inner command + composed context (`27C`
/// §3 / `27N`). `book_argv` is the site's whole argv (command word first), every element a resolved
/// literal. Returns `None` when the site is NOT wrapped (`book_argv[0]` is no loaded wrapper — the
/// ordinary path) OR when a wrapper cannot peel this argv (the safe wall: the site stays opaque,
/// unchanged law — `silence-licenses-nothing`). Iteratively peels wrapper after wrapper (a chain
/// `sudo chroot CMD`), composing each link's shift, until the head is a non-wrapper. Budget-bounded.
#[must_use]
pub fn peel_book_chain(book_argv: &[&str], wrappers: &WrapperIndex) -> Option<PeeledChain> {
    if !wrappers.contains_key(*book_argv.first()?) {
        return None; // not a wrapped site — the ordinary classify path
    }
    let mut links = Vec::new();
    let mut chain_links = Vec::new();
    let mut cur: Vec<String> = book_argv.iter().map(|s| (*s).to_owned()).collect();
    let mut budget = 32usize;
    loop {
        budget = budget.checked_sub(1)?; // runaway chain ⇒ wall (the safe direction)
        let Some(model) = wrappers.get(cur.first()?) else {
            break; // the inner (non-wrapper) command
        };
        let after: Vec<&str> = cur[1..].iter().map(String::as_str).collect();
        let consumed = crate::wrapper::peel_consumed(&model.predict, &after)?;
        let lend_values = model
            .lend_map
            .as_ref()
            .map(|lm| crate::wrapper::resolve_lend_values(lm, &after))
            .unwrap_or_default();
        chain_links.push(ChainLink::from_lend_map(
            &model.lend,
            model.rho.clone(),
            |dim| lend_values.get(&dim).cloned().flatten(),
        ));
        links.push(ChainLinkId {
            provider: model.provider,
            entry: model.enter.clone(),
        });
        cur = cur.get(consumed.saturating_add(1)..)?.to_vec();
        if cur.is_empty() {
            return None; // a wrapper with no guest ⇒ wall
        }
    }
    Some(PeeledChain {
        inner_argv: cur,
        composed: compose_chain(&chain_links),
        links,
    })
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
         (one line: `: safe-across {}`)",
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

/// Corroboration lint, forward direction (`27C` §6): a `safe-across user` mark over a body that
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
) -> Option<Diag> {
    (vouch.mentions(Dimension::User) && reads_identity(verdict, interner)).then(|| {
        Diag::new(
            DiagCode::ToleratesOverIdentityDependence(ToleratesOverIdentityDependence),
            span,
        )
    })
}

/// Corroboration lint, reverse direction (`27C` §6): a body doing heavy context-handling (visible
/// identity reads) with NO tolerance mark ⇒ the one-line hint (it would become context-shiftable
/// with a `safe-across` mark). A Note (recognize-never-license). `None` when the body is already
/// vouched or reads no identity.
#[must_use]
pub fn hint_heavy_context_no_vouch(
    vouch: &ToleranceVouch,
    verdict: &Predict,
    interner: &Interner,
    span: dorc_core::Span,
) -> Option<Diag> {
    (vouch.is_empty() && reads_identity(verdict, interner)).then(|| {
        Diag::new(
            DiagCode::HeavyContextNoTolerance(HeavyContextNoTolerance),
            span,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predict::{lift_predicts, lift_verdicts_converged};

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
    fn one_lend_map(src: &str) -> (Interner, Predict) {
        let mut i = Interner::default();
        let set = crate::wrapper::lift_lend_map_set(&mut i, src);
        assert!(set.diags.is_empty(), "clean lend_map lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        let lm = set.value.get(p).expect("the lend_map").clone();
        (i, lm)
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

    // ── fold-entry coherence (27C:rul-fold-entry-coherence-failfast, narrow static) ──
    #[test]
    fn trivial_repass_entry_is_coherent_by_delegation() {
        // `sudo -n "$@"` (0 shifts) delegates ALL parsing to the real tool ⇒ coherent regardless of
        // the sudo lend_map (whose flag-strip `while` yields `None` anyway — conservatively skipped).
        let (_i, e) = one_enter("sudo__enter() { sudo -n \"$@\" ;}");
        let (_il, lm) = one_lend_map(
            "sudo__lend_map() { while [ \"${1#-}\" != \"$1\" ]; do shift; done; printf '%s\\n' root : lends user; \"$@\"; }",
        );
        assert_eq!(
            check_entry_coherence(&e, &lm),
            None,
            "a trivial re-pass entry never fail-fasts (delegates to the real tool)"
        );
    }
    #[test]
    fn argparsing_entry_matching_lend_fold_is_coherent() {
        // chroot: the entry shifts the dir (1) and the lend_map shifts the dir (1) ⇒ agree ⇒ coherent.
        let (_i, e) = one_enter("chroot__enter() { dir=$1; shift; chroot \"$dir\" \"$@\" ;}");
        let (_il, lm) = one_lend_map(
            "chroot__lend_map() { printf '%s\\n' \"$1\" : lends fs-view; shift; \"$@\" ;}",
        );
        assert_eq!(
            check_entry_coherence(&e, &lm),
            None,
            "matching shift-counts cohere"
        );
    }
    #[test]
    fn argparsing_entry_dropping_an_arg_fails_fast() {
        // A malformed entry that shifts MORE than the lend-fold consumed drops an arg the fold relied
        // on ⇒ argv-flow divergence ⇒ fail-fast (declarations-genuinely-contradict).
        let (_i, e) =
            one_enter("chroot__enter() { a=$1; shift; b=$1; shift; chroot \"$a\" \"$@\" ;}");
        let (_il, lm) = one_lend_map(
            "chroot__lend_map() { printf '%s\\n' \"$1\" : lends fs-view; shift; \"$@\" ;}",
        );
        assert_eq!(
            check_entry_coherence(&e, &lm),
            Some(EntryIncoherence {
                entry_shifts: 2,
                lend_shifts: 1
            }),
            "entry drops an arg the fold consumed ⇒ static incoherence"
        );
    }

    // ── the tolerance vouch ──────────────────────────────────────────────────────
    // NB the parseable spelling is the colon-line `: safe-across <dim>` (no-op `:` command + a
    // `: target` mark, like the corpus `: undivided-by-transit-across user`), NOT an attached-colon
    // `tolerates:<dim>` token (the retired v0.1 `27C` §2 strawman; a single `:` hosts no mark).
    #[test]
    fn top_level_tolerates_is_unconditional() {
        // A `safe-across user` mark at the top of the body vouches user for EVERY verb (the babby
        // template).
        let (_i, v) = one_verdict(
            "pipx__is_converged() { : safe-across user\n verb=$1; shift; case \"$verb\" in \
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
        // A `safe-across` mark inside the `install` arm vouches ONLY the install path (reachability-
        // scoped): a `remove` site is NOT licensed to shift (the safe direction).
        let (_i, v) = one_verdict(
            "pipx__is_converged() { verb=$1; shift; case \"$verb\" in \
             install) : safe-across user\n pipx list ;; remove) pipx list ;; *) return 2 ;; esac }",
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
        let (_i, v) = one_verdict("x__is_converged() { : safe-across {user,fs-view}\n return 0 }");
        let (vouch, diags) = lift_tolerance(&v);
        assert!(diags.is_empty(), "clean: {diags:?}");
        let dims = vouch.tolerated_on_path(None);
        assert!(dims.contains(&Dimension::User) && dims.contains(&Dimension::FsView));
    }
    #[test]
    fn unknown_tolerates_dimension_is_loud() {
        let (_i, v) = one_verdict("x__is_converged() { : safe-across universe\n return 0 }");
        let (vouch, diags) = lift_tolerance(&v);
        assert_eq!(diags.len(), 1, "one loud diag: {diags:?}");
        assert_eq!(diags[0].code.slug(), "tolerates-unknown-dimension");
        assert!(vouch.is_empty(), "an unknown token vouches nothing");
    }

    // ── the composition algebra (HUMAN-ACKED rulings 2026-07-17) ──────────────────
    /// A test link — unspecified dimensions default to `Full` (a wrapper only touches the dimensions
    /// its `lend_map` maps; the rest are borrowed), ρ defaults to `Nothing` (identity).
    fn link(shifts: &[(Dimension, Shift)]) -> ChainLink {
        link_rho(shifts, RhoClaim::Nothing)
    }
    fn link_rho(shifts: &[(Dimension, Shift)], rho: RhoClaim) -> ChainLink {
        let mut m: BTreeMap<Dimension, Shift> = Dimension::ALL
            .into_iter()
            .map(|d| (d, Shift::Full))
            .collect();
        for (d, s) in shifts {
            m.insert(*d, s.clone());
        }
        ChainLink { shifts: m, rho }
    }

    #[test]
    fn top_propagates_no_rescue_through_full_or_absolute() {
        // ruling 1 (`rul-top-absorbs-absolute-maps`): a ⊤ MIDDLE link poisons the dimension chain-
        // wide — no rescue, whether the inner link is a FULL lend OR an ABSOLUTE map.
        // user via an inner ABSOLUTE map (bob) under a ⊤ middle:
        let via_absolute = compose_chain(&[
            link(&[(Dimension::User, Shift::Mapped("root".into()))]),
            link(&[(Dimension::User, Shift::Top)]),
            link(&[(Dimension::User, Shift::Mapped("bob".into()))]),
        ]);
        assert_eq!(
            via_absolute.shift(Dimension::User),
            Shift::Top,
            "an inner absolute map NEVER rescues a ⊤ middle"
        );
        // fs-view via an inner FULL lend under a ⊤ middle:
        let via_full = compose_chain(&[
            link(&[(Dimension::FsView, Shift::Mapped("/a".into()))]),
            link(&[(Dimension::FsView, Shift::Top)]),
            link(&[(Dimension::FsView, Shift::Full)]),
        ]);
        assert_eq!(
            via_full.shift(Dimension::FsView),
            Shift::Top,
            "an inner full lend NEVER rescues a ⊤ middle"
        );
    }

    #[test]
    fn user_is_absolute_overwrite_inner_wins() {
        // ruling 2 (`rul-dimension-owned-compose-ops`): user = absolute overwrite. `sudo -u root …
        // sudo -u bob` ⇒ bob (inner wins, caller-independent), NOT a ⊤ conflict.
        let c = compose_chain(&[
            link(&[(Dimension::User, Shift::Mapped("root".into()))]),
            link(&[(Dimension::User, Shift::Mapped("bob".into()))]),
        ]);
        assert_eq!(c.shift(Dimension::User), Shift::Mapped("bob".into()));
    }

    #[test]
    fn fsview_is_caller_relative_paths_nest() {
        // ruling 2/6: fs-view = caller-relative. `chroot /mnt` then `chroot /t` ⇒ `/mnt/t`; reversed
        // ⇒ `/t/mnt` (path nesting is order-sensitive — the genuine fold difference of ruling 4).
        let mnt = link(&[(Dimension::FsView, Shift::Mapped("/mnt".into()))]);
        let t = link(&[(Dimension::FsView, Shift::Mapped("/t".into()))]);
        assert_eq!(
            compose_chain(&[mnt.clone(), t.clone()]).shift(Dimension::FsView),
            Shift::Mapped("/mnt/t".into())
        );
        assert_eq!(
            compose_chain(&[t, mnt]).shift(Dimension::FsView),
            Shift::Mapped("/t/mnt".into())
        );
    }

    #[test]
    fn identity_chain_is_host_default() {
        let mut i = Interner::default();
        let c = compose_chain(&[link(&[]), link(&[])]); // all Full, ρ Nothing
        assert!(c.crossed().is_empty(), "an all-full chain shifts nothing");
        assert_eq!(
            c.to_context(&mut i),
            Context::HostDefault,
            "identity chain ⇒ HostDefault (rung-0)"
        );
    }

    #[test]
    fn nice_permutation_shares_key_but_scrub_reorder_differs() {
        // ruling 4 (canonical = folded NORMAL FORM, order-sensitive ONLY where folds differ):
        let mut i = Interner::default();
        let sudo = link(&[(Dimension::User, Shift::Mapped("postgres".into()))]);
        let nice = link(&[]); // identity wrapper
        // `sudo -u postgres nice` and `nice sudo -u postgres` fold to the SAME normal form (nice
        // perturbs nothing) ⇒ ONE key.
        let a = compose_chain(&[sudo.clone(), nice.clone()]).to_context(&mut i);
        let b = compose_chain(&[nice, sudo]).to_context(&mut i);
        assert_eq!(a, b, "nice-permutation shares ONE key (ruling 4)");
        // `env A=1 sudo` vs `sudo env A=1` DIFFER — sudo SCRUBS A away, so the composed ρ differs.
        let env_a = link_rho(
            &[],
            RhoClaim::FullAmbient {
                overrides: vec!["A".into()],
            },
        );
        let sudo_scrub = link_rho(
            &[(Dimension::User, Shift::Mapped("root".into()))],
            RhoClaim::ExactlyThese {
                vars: vec!["TERM".into(), "HOME".into()],
            },
        );
        let env_then_sudo = compose_chain(&[env_a.clone(), sudo_scrub.clone()]).to_context(&mut i);
        let sudo_then_env = compose_chain(&[sudo_scrub, env_a]).to_context(&mut i);
        assert_ne!(
            env_then_sudo, sudo_then_env,
            "sudo scrubs ⇒ `env A=1 sudo` ≠ `sudo env A=1` (ruling 4)"
        );
    }

    #[test]
    fn different_dimensions_are_order_independent() {
        // ruling 4 corollary: `sudo chroot` and `chroot sudo` shift DIFFERENT dimensions (user vs
        // fs-view), so they fold to the SAME normal form ⇒ ONE key (order matters only within a
        // dimension's fold, never across independent dimensions).
        let mut i = Interner::default();
        let sudo = link(&[(Dimension::User, Shift::Mapped("root".into()))]);
        let chroot = link(&[(Dimension::FsView, Shift::Mapped("/mnt".into()))]);
        let a = compose_chain(&[sudo.clone(), chroot.clone()]).to_context(&mut i);
        let b = compose_chain(&[chroot, sudo]).to_context(&mut i);
        assert_eq!(a, b, "independent dimensions ⇒ order-independent key");
    }

    #[test]
    fn same_chain_same_context_key() {
        // Two `sudo`-wrapped sites (different inner command) share ONE context — the key is the
        // folded WRAPPER-chain normal form, not the inner. A re-measurement self-heals to one slot.
        let sudo = link(&[(Dimension::User, Shift::Mapped("root".into()))]);
        let mut i = Interner::default();
        let a = compose_chain(std::slice::from_ref(&sudo)).to_context(&mut i);
        let b = compose_chain(&[sudo]).to_context(&mut i);
        assert_eq!(a, b, "same wrapper chain ⇒ same context key");
    }

    #[test]
    fn unresolvable_mapped_user_walls() {
        // ruling 6: an unresolvable `sudo -u "$VAR"` ⇒ ⊤-value ⇒ walls. `from_lend_map` maps a
        // `Mapped` dimension whose resolver returns `None` to `Shift::Top`.
        use crate::wrapper::{derive_lend_map, lift_lend_map_set};
        let mut i = Interner::default();
        let set = lift_lend_map_set(
            &mut i,
            "sudo__lend_map() { printf '%s\\n' root : lends user; : lends fs-view; : lends netns; \"$@\"; }",
        );
        let p = set.value.providers().next().unwrap();
        let lm = set.value.get(p).unwrap().clone();
        let (map, _d) = derive_lend_map(&lm);
        // The resolver could not resolve the mapped user target (`sudo -u "$VAR"`, $VAR unknown).
        let link = ChainLink::from_lend_map(&map, RhoClaim::Nothing, |_dim| None);
        assert_eq!(
            link.shift(Dimension::User),
            Shift::Top,
            "unresolved mapped user ⇒ ⊤ (walls)"
        );
        assert_eq!(
            link.shift(Dimension::FsView),
            Shift::Full,
            "full colon-line ⇒ Full"
        );
        let c = compose_chain(&[link]);
        assert!(c.walls().contains(&Dimension::User));
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
        // Forward (`27C` §6): `safe-across user` over a body that reads `$USER` ⇒ "are you sure?".
        let (i, v) = one_verdict(
            "x__is_converged() { : safe-across user\n me=$USER; case \"$me\" in \
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
            hint.contains(": safe-across user"),
            "suggests the colon-line form"
        );
    }

    // ── book-side chain peeling (lane-integration `27N`) ─────────────────────────

    /// Build a one-wrapper [`WrapperIndex`] for `sudo` from its three members, sharing one interner.
    fn babby_sudo_index() -> (Interner, WrapperIndex) {
        let mut i = Interner::default();
        let predict_set = lift_predicts(
            &mut i,
            "sudo__predict() { while [ \"${1#-}\" != \"$1\" ]; do \
             case \"$1\" in -u) shift 2 ;; *) shift ;; esac; done; env -i HOME=/root \"$@\" ; }",
        );
        let lm_set = crate::wrapper::lift_lend_map_set(
            &mut i,
            "sudo__lend_map() { target=root; while [ \"${1#-}\" != \"$1\" ]; do \
             case \"$1\" in -u) target=\"$2\"; shift 2 ;; *) shift ;; esac; done; \
             printf '%s\\n' \"$target\" : lends user\n: lends fs-view\n: lends netns\n\"$@\" ; }",
        );
        let enter_set = lift_entry_set(&mut i, "sudo__enter() { sudo -n \"$@\" ;}");
        let p = predict_set.value.providers().next().unwrap();
        let predict = predict_set.value.get(p).unwrap().clone();
        let lend_map = lm_set.value.get(p).unwrap().clone();
        let enter = enter_set
            .value
            .get(p)
            .map(|e| detect_entry_form(e).unwrap());
        let (lend, _) = crate::wrapper::derive_lend_map(&lend_map);
        let rho = crate::wrapper::detect_peel(&predict).unwrap().rho;
        let mut idx = WrapperIndex::new();
        idx.insert(
            "sudo".to_owned(),
            WrapperModel {
                predict,
                rho,
                lend,
                lend_map: Some(lend_map),
                enter,
                provider: p,
            },
        );
        (i, idx)
    }

    #[test]
    fn babby_sudo_peels_to_the_root_user_context() {
        // `sudo pipx install poddle` peels to the inner `pipx install poddle`, in a context that
        // shifts USER to root (mapped), fs-view + netns FULL (the enumerate-every-dimension lend).
        // So the chain CROSSES exactly `user` (the gate the entry decision keys on) and walls
        // nothing — the babby-sudo story's context (`27C` §8).
        let (mut i, idx) = babby_sudo_index();
        let chain =
            peel_book_chain(&["sudo", "pipx", "install", "poddle"], &idx).expect("a wrapped site");
        assert_eq!(chain.inner_argv, vec!["pipx", "install", "poddle"]);
        assert_eq!(chain.composed.crossed(), vec![Dimension::User]);
        assert!(chain.composed.walls().is_empty(), "no walled dimension");
        assert_eq!(
            chain.composed.shift(Dimension::User),
            Shift::Mapped("root".to_owned())
        );
        // The context is Wrapped (a user shift) — distinct from an unwrapped site's HostDefault.
        assert!(matches!(
            chain.composed.to_context(&mut i),
            Context::Wrapped(_)
        ));
        assert_eq!(chain.links.len(), 1, "one wrapper in the chain");
        assert!(
            chain.links[0].entry.is_some(),
            "sudo authored an entry form"
        );
    }

    #[test]
    fn unwrapped_site_does_not_peel() {
        // `pipx install httpie` — its head is no loaded wrapper ⇒ `None` (the ordinary path). The
        // babby story's second site probes bare in the ambient context.
        let (_i, idx) = babby_sudo_index();
        assert!(peel_book_chain(&["pipx", "install", "httpie"], &idx).is_none());
    }
}
