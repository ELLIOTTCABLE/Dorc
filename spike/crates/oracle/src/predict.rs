//! `check` — the command-keyed `check()` contract (19H §2 / 202 §1 face-check).
//!
//! An oracle ships one sh function per command-family, `<provider>__predict`, that
//! argparses the command the way the real tool does, inline-annotates which value
//! is which named kind, and is itself the read-only probe body. This module is the
//! *static half* (202 §2.7's "read twice"): a dedicated mini-parser for the
//! constrained contract dialect plus a concrete evaluator that traces a known argv
//! through a check's argparse to its kind-annotation, yielding a [`Resolution`].
//!
//! # Why a separate parser (`adj-dialect-parser`, note 203 §4)
//!
//! The contract dialect is *not* arbitrary sh and is *not* the book dialect. The
//! book parser (`dorc-syntax`) ⊤-rejects loops by design; a `check()`'s argparse
//! *needs* `while`. Extending the book parser would drag its CFG-lowering, errexit
//! and consumption machinery along. Instead this module owns a small parser whose
//! grammar *is* the contract: anything it cannot parse is a loud per-function lift
//! failure (`inv-no-throw`: a diagnostic, never a panic; the file's other checks
//! still lift). The dialect parser structurally enforces "the dialect is NOT
//! arbitrary sh" (19G §2) — outside-dialect input fails to parse ⇒ unresolvable.
//!
//! # Soundness posture (`inv-kfail`, both directions)
//!
//! The evaluator never guesses. A flag the argparse does not consume, an arm `case`
//! cannot select concretely, an annotation whose value-position is not a positional
//! or known binding, a missing annotation, a budget overrun — every one is
//! [`Top`](Resolution::Top) with a reason. `Top` is always safe (the site stays
//! un-probeable and un-elidable). A *wrong* resolution is the disaster class (19H
//! §1.3: propagation-correctness has no floor), so every ambiguity biases to `Top`.
//!
//! `inv-referent-agnostic`: the evaluator resolves which **argv element** is the
//! entity (by tracing the oracle's own argparse); it never branches on what the
//! entity's text *means*. Kind strings are opaque coordination handles.

use dorc_core::diag::{Diag, DiagCode as Code, PredictOutOfDialect, PredictUnterminated};
use dorc_core::{Diagnostic, Interner, Span};

mod ast;
mod derive;
mod eval;
mod lexer;
mod parser;

pub use ast::{Mark, MarkKind, MarkTarget, Predict, PredictSet, Stmt};
pub use derive::{DerivedEffect, ValueClaim, derive_predict};
pub use eval::{
    Resolution, Resolved, ResolvedEntity, StageStdout, TopReason, evaluate, predict_stage_stdout,
};
pub use parser::lift_predicts;

// The touches-footprint lift (`crate::touches`, 24A §1b) reuses the predict dialect: the
// same funcdef AST ([`ast`]) and the same word-resolution ([`eval`]) so footprint fragments
// travel the exact value-flow predict does (the vocabulary fence). Re-exported `pub(crate)`
// for that sibling module — these are internal to the oracle crate, not public API.
pub(crate) use ast::{CaseArm, Command, Test, Word};
pub(crate) use eval::{eval_test, pattern_matches, resolve_word};
pub(crate) use parser::{
    lift_reaches, lift_resolvers, lift_state_stored_only_in, lift_touches, lift_verdicts_converged,
};

/// Strip an authored check funcdef to runnable sh — the STRIP-ONLY pass (R1c / 23D §1).
/// It rewrites a period-form name (`apt-get.predict`) to the mangled `<provider>__predict`
/// the engine keys on, and removes the inline dialect annotations: the identity
/// `name : kind = value` → `name=value`; a trailing effect mark `cmd … : kind:entity.prop`
/// → just `cmd …`; and a BARE-mark statement (`: kind`, `: kind:entity.prop~`, …) → DELETED
/// WHOLE — the mark line and one adjacent separator gone, never left as a `:` null command
/// (23H §9.4 strip-fidelity: a bare mark is an annotation-LINE, equivalent to a comment; a
/// stripped-in trailing `:` would clobber the preceding command's tool-rc to 0, which in
/// guard position is an always-skip guard — the disaster shape). The author's LAST
/// substantive command thus stays the last exit-status-affecting statement. Nothing else
/// changes — other bytes (whitespace, `while`/`case`, redirs, comments) are preserved
/// verbatim. A compound body containing ONLY bare marks would strip to an empty block
/// (invalid sh); that is ⊤-rejected at LIFT (`parser`), so it never reaches the strip.
///
/// `src` is the whole oracle source; [`Predict::span`] locates the funcdef within it. The
/// result is `dash -n`-clean (the period name is the only dash-rejected form; annotations
/// are dash-valid-but-runtime-wrong, so removing them fixes the shipped probe's runtime,
/// not its syntax) and **byte-stable** — a deterministic function of its input, so a
/// golden built from it never churns without an authored change.
///
/// Surgical (`23A §1`: "the annotation-strip is the only byte delta from the authored
/// oracle"): it edits source byte-ranges rather than re-rendering the AST, so formatting
/// survives. `inv-no-throw`: a non-char-boundary span is skipped rather than panicking
/// (the ASCII sh corpus never hits this).
#[must_use]
pub fn strip_predict(src: &str, check: &Predict, interner: &Interner) -> String {
    strip_role(src, check, interner, "__predict")
}

/// Strip an authored **verdict** funcdef (`<provider>.is_converged`/`.is_diverged`) to runnable
/// sh for shipping in GUARD position (24D §2/§3 — the guard's check IS the oracle's own verdict
/// body, strip-only). Identical to [`strip_predict`] but mangles the funcname to the verdict
/// suffix the guard emitter invokes (`apt-get.is_converged` → `apt_get__is_converged`), so the
/// shipped preamble def and the guard invocation agree byte-for-byte. `mangled_suffix` is
/// `"__is_converged"` or `"__is_diverged"` (the caller passes it from the lifted
/// [`crate::verdict::VerdictSense`]); everything else — annotation removal, bare-mark deletion,
/// verbatim body bytes — is the strip's standing contract (strip-fidelity, 23H §9.4).
#[must_use]
pub fn strip_verdict(
    src: &str,
    verdict: &Predict,
    interner: &Interner,
    mangled_suffix: &str,
) -> String {
    strip_role(src, verdict, interner, mangled_suffix)
}

/// Strip an authored **touches** funcdef (`<provider>.touches`) to runnable sh for shipping in
/// the DERIVATION-PROBE lane (24E §2/§9 — a payload-bound footprint the tool emits itself). A
/// touches body that reaches a host tool (`dpkg -L`) cannot be traced statically, so Stage 4
/// ships it (strip-only) to run read-only on the host; its stdout coord-lines are the derived
/// footprint. Identical to [`strip_predict`] but mangles the funcname to `__touches`
/// (`apt-get.touches` → `apt_get__touches`), so the shipped def and the derivation invocation
/// agree byte-for-byte. Everything else — annotation removal, bare-mark deletion, verbatim body
/// bytes — is the strip's standing contract (strip-fidelity, 23H §9.4). Same self-vouch tier as
/// `strip_predict`/`strip_verdict` (fork-4A: no new trust edge; authorship IS the vouch).
#[must_use]
pub fn strip_touches(src: &str, touches: &Predict, interner: &Interner) -> String {
    strip_role(src, touches, interner, "__disturbs")
}

/// Strip an authored **resolver** funcdef (`<kind>.resolve`) to runnable sh for the
/// CANONICALIZATION probe lane (24F §3 — the identity role-sibling / the resid-aliasing closure). A
/// resolver reaches a host tool (`dpkg-query -W`, `realpath`) to canonicalize an entity, so it
/// cannot resolve statically; Stage 5 ships it strip-only to run read-only per coordinate, its
/// stdout the canonical form. Identical to [`strip_predict`] but mangles the funcname to `__resolve`
/// (`package.resolve` → `package__resolve`), so the shipped def and the resolver invocation agree
/// byte-for-byte. NB `<kind>` is the KIND name here (the resolver is kind-keyed), and
/// [`crate::to_funcname_segment`] maps it identically. Everything else — annotation removal,
/// bare-mark deletion, verbatim body bytes — is the strip's standing contract (strip-fidelity, 23H
/// §9.4). Same self-vouch tier as `strip_predict`/`strip_touches` (fork-4A: no new trust edge —
/// authoring IS the vouch; the rc-127 mocks net is the live guarantee).
#[must_use]
pub fn strip_resolve(src: &str, resolver: &Predict, interner: &Interner) -> String {
    strip_role(src, resolver, interner, "__resolve")
}

/// Strip an authored **reaches** funcdef (`<kind>.reaches`) to runnable sh for shipping a DYNAMIC
/// arm into the REACH probe lane (24G §4 — the cross-author footprint-expansion mechanism). A
/// dynamic reaches arm reaches a host tool (`dpkg -L`) whose stdout lines are the reached entities;
/// it cannot resolve statically, so its body ships strip-only to run read-only per coordinate.
/// Identical to [`strip_predict`] but mangles the funcname to `__reaches` (`package.reaches` →
/// `package__reaches`), so the shipped def and the reach invocation agree byte-for-byte. NB `<kind>`
/// is the KIND name here (reaches is kind-keyed, like the resolver), and [`crate::to_funcname_segment`]
/// maps it identically. The typed-emission trailing marks (`… : service`) are annotation-LINEs the
/// strip DELETES whole (strip-fidelity, 23H §9.4) — the reached kind was already interned at LIFT
/// (24G §4 vocabulary fence), so the shipped body needs only the raw emitting command. Same
/// self-vouch tier as `strip_predict`/`strip_resolve` (24G inv-kfail: reach bodies are probe-lane
/// read-only — `kFAIL-withhold`; authoring IS the vouch; the rc-127 mocks net is the live guarantee).
#[must_use]
pub fn strip_reaches(src: &str, reaches: &Predict, interner: &Interner) -> String {
    strip_role(src, reaches, interner, "__disturbance_reaches_only")
}

/// The shared STRIP-ONLY pass (R1c / 23D §1), parametrized by the target mangled suffix so the
/// probe lane (`__predict`), the guard lane (`__is_converged`/`__is_diverged`), and the
/// derivation lane (`__touches`, 24E §2) route through ONE audited implementation. See
/// [`strip_predict`] for the full contract.
fn strip_role(src: &str, check: &Predict, interner: &Interner, mangled_suffix: &str) -> String {
    let base = check.span.lo.0 as usize;
    let funcdef = src
        .get(base..check.span.hi.0 as usize)
        .unwrap_or_default()
        .to_owned();

    // (lo, hi, replacement) — all funcdef-relative; applied back-to-front so earlier
    // offsets stay valid. The regions (funcname, each annotation, each mark) are disjoint.
    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    let rel = |p: Span| {
        (
            (p.lo.0 as usize).saturating_sub(base),
            (p.hi.0 as usize).saturating_sub(base),
        )
    };

    // 1. Funcname: `apt-get.predict` / `apt_get__predict` → `apt_get<suffix>` (idempotent on
    //    the already-mangled form for the same suffix).
    let (nlo, nhi) = rel(check.name_span);
    edits.push((
        nlo,
        nhi,
        format!(
            "{}{mangled_suffix}",
            crate::to_funcname_segment(interner.resolve(check.provider))
        ),
    ));

    // 2/3/4. Annotations + marks, recursively through the body's control-flow.
    collect_strip_edits(&check.body, src, base, &mut edits);

    edits.sort_by_key(|e| core::cmp::Reverse(e.0));
    let mut out = funcdef;
    for (lo, hi, repl) in edits {
        if lo <= hi && hi <= out.len() && out.is_char_boundary(lo) && out.is_char_boundary(hi) {
            out.replace_range(lo..hi, &repl);
        }
    }
    out
}

/// Collect the strip edits for a statement list, recursing into `case`/`if`/`while`
/// bodies (annotations and marks nest there). `base` is the funcdef start offset (edits
/// are funcdef-relative); `src` is sliced for verbatim value text.
fn collect_strip_edits(
    body: &[Stmt],
    src: &str,
    base: usize,
    edits: &mut Vec<(usize, usize, String)>,
) {
    let rel = |p: Span| {
        (
            (p.lo.0 as usize).saturating_sub(base),
            (p.hi.0 as usize).saturating_sub(base),
        )
    };
    for stmt in body {
        match stmt {
            // identity `name : kind [= value]` → `name=value` (verbatim name + value bytes).
            Stmt::Annotation(a) => {
                let (lo, hi) = rel(a.span);
                let name = src
                    .get(a.name_span.lo.0 as usize..a.name_span.hi.0 as usize)
                    .unwrap_or_default();
                let value = a.value_span.map_or("", |vs| {
                    src.get(vs.lo.0 as usize..vs.hi.0 as usize)
                        .unwrap_or_default()
                });
                edits.push((lo, hi, format!("{name}={value}")));
            }
            // trailing effect mark: delete ` : target` (command-end .. mark-end).
            Stmt::Command(c) => {
                if let Some(m) = &c.mark {
                    let lo = (c.span.hi.0 as usize).saturating_sub(base);
                    let hi = (m.span.hi.0 as usize).saturating_sub(base);
                    edits.push((lo, hi, String::new()));
                }
            }
            Stmt::Case { arms, .. } => {
                for a in arms {
                    collect_strip_edits(&a.body, src, base, edits);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_strip_edits(then_body, src, base, edits);
                collect_strip_edits(else_body, src, base, edits);
            }
            Stmt::While { body, .. } => collect_strip_edits(body, src, base, edits),
            Stmt::Assign { .. } | Stmt::Shift { .. } => {}
        }
    }
}

/// Canonicalize a command word (or a funcdef provider fragment) to its **funcname segment** —
/// the FORWARD munge (`rul24-totalistic-munge`): `-`/`.` → `_`, leading-digit repaired. THE single
/// home of the command-word↔key convention (204 §6 seam #2): the parser keys a [`PredictSet`] and
/// the [`KindIndex`](crate::KindIndex) by this form, AND `analysis::effect` derives a book command
/// word's provider through the same function, so the book word, the effect-map key, and the funcname
/// all agree. An oracle named `<seg>__role` therefore serves every book word munging to `<seg>`
/// (a literal `_` book word `my_tool` finds `my_tool__is_converged`; a dotted `my.tool` finds
/// `my_tool__role`); two distinct co-loaded source names munging to one segment are the landed
/// `munge-name-collision` refusal.
///
/// Was the LOSSY backward un-munge (`_` → `-`) — deleted per `24C:rul24-totalistic-munge` so
/// command-keyed lookup matches the kind-keyed forward-munge (`rekey_to_raw_kinds`). Delegates to
/// [`crate::to_funcname_segment`] (the one munge); kept as the named seam so a future escape lands here.
#[must_use]
pub fn map_provider_name(raw: &str) -> String {
    crate::to_funcname_segment(raw)
}

// B4 sweep: check codes migrated onto Diag spine.

/// The conventional local variable name an oracle assigns the verb to (`verb=$1`,
/// 19H §2.1/§2.5). Recognizing it is a *structural convention in the oracle's own
/// code* (like the `__predict` suffix or the annotation shape), NOT decoding entity
/// text — so it does not breach `inv-referent-agnostic`. Whether `verb` should be a
/// reserved dialect name is a `tc-*`-shaped cross-cutting question (flagged in the
/// build report); the conservative local choice is: if the oracle does not use this
/// name, the [`Resolution`] simply carries no verb (always safe).
const VERB_BINDING: &str = "verb";

/// A per-function lift failure: the named function is in the file but its body is
/// out of dialect. Fail-soft (`inv-no-throw`): the function contributes no [`Predict`]
/// and the rest of the file still lifts.
///
/// `is_unterminated`: selects `PredictUnterminated` vs `PredictOutOfDialect`. Both carry
/// the message as `detail`. The `span` is ALWAYS real: every caller that previously had
/// no token (an EOF give-up — an unterminated body, a `fail_here`/`true_with` at
/// end-of-input) now synthesizes a zero-width end-of-input span via
/// [`Parser::eof_span`](super::parser) (human ruling 22-q1: pointing the UI at
/// end-of-file is right for a truncated/chopped check body), so these two codes lower
/// through [`Diag::new`] like every other and do NOT join the spanless-mint allow-list.
///
/// Routed through the typed [`Diag`] spine, NOT `Diagnostic::error` (x3a-5/t-4 fix,
/// `224` §10): severity comes from [`dorc_core::diag::registry`] keyed on the code, never
/// hardcoded here. Both check codes are registry-declared `Error`, so the lowered output is
/// byte-identical to the prior `Diagnostic::error(…)` form — but a future registry edit now
/// actually takes effect instead of being a silent no-op.
///
/// The two `Code::Predict…(…)` payloads are spelled as LITERALS at each `Diag::new` site (not
/// built once into a variable and threaded), so the `diag_tidy` constructed-scan actually SEES
/// these emits — a `Diag::new(var, …)` form would be invisible to the needle-shape scanner (t-4
/// non-literal bypass) and read as dead catalog. Verbose-on-purpose; the literals are the gate's
/// eyes.
pub(crate) fn lift_failure(
    is_unterminated: bool,
    span: Span,
    message: impl Into<String>,
    interner: &mut Interner,
) -> Diagnostic {
    let msg = message.into();
    // `Diag::to_legacy` rebuilds the legacy `message` from the primary label plus the render body
    // (empty for these `detail`-only payloads), so `.label(msg)` reproduces the prior bare text
    // exactly. Severity/code/span all flow from the typed value (severity via `registry`).
    let diag = if is_unterminated {
        Diag::new(
            Code::PredictUnterminated(PredictUnterminated {
                detail: msg.clone(),
            }),
            span,
        )
    } else {
        Diag::new(
            Code::PredictOutOfDialect(PredictOutOfDialect {
                detail: msg.clone(),
            }),
            span,
        )
    }
    .label(msg);
    diag.to_legacy(interner)
}

#[cfg(test)]
mod strip_tests {
    //! R1c: the STRIP-ONLY pass. Every fixture-shaped body strips to runnable sh whose
    //! only deltas from the author are the funcname rewrite and annotation removal, and
    //! the strip is byte-stable (a golden built from it never churns).
    use super::{lift_predicts, strip_predict};
    use dorc_core::Interner;

    fn strip_one(src: &str) -> String {
        let mut i = Interner::default();
        let out = lift_predicts(&mut i, src);
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let provider = out.value.providers().next().expect("one provider");
        let check = out.value.get(provider).expect("the check");
        strip_predict(src, check, &i)
    }

    /// The flagship strip (23A §1): `apt-get.predict` → `apt_get__predict` and
    /// `pkg : package = "$1"` → `pkg="$1"` — and NOTHING else. Byte-exact against the
    /// hand-authored flagship golden's apply preamble.
    #[test]
    fn flagship_body_strips_to_the_golden_preamble() {
        let authored = "\
apt_get__predict() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   pkg : package = \"$1\"
   if [ \"$2\" = \"\" ]; then dpkg-query -W \"$pkg\" >/dev/null 2>&1; fi
}";
        let expected = "\
apt_get__predict() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   pkg=\"$1\"
   if [ \"$2\" = \"\" ]; then dpkg-query -W \"$pkg\" >/dev/null 2>&1; fi
}";
        assert_eq!(strip_one(authored), expected);
    }

    /// A trailing effect mark is deleted cleanly (no dangling `:` / argv residue): the
    /// probe command that ships is the bare `dpkg-query …`.
    #[test]
    fn trailing_mark_is_removed_leaving_the_bare_command() {
        let authored = "apt_get__predict() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\" : package:\"$pkg\"#installed; }";
        let stripped = strip_one(authored);
        assert!(
            stripped.contains("dpkg-query -W \"$pkg\";")
                || stripped.contains("dpkg-query -W \"$pkg\" "),
            "the probe command survives bare: {stripped}"
        );
        assert!(
            !stripped.contains(": package:"),
            "no annotation residue: {stripped}"
        );
        assert!(
            stripped.starts_with("apt_get__predict()"),
            "funcname mangled: {stripped}"
        );
    }

    /// Idempotence / no-regression (R1c): a body strips byte-stably (re-stripping is a fixpoint).
    #[test]
    fn mark_free_body_strips_unchanged_and_stably() {
        let authored = "apt_get__predict() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\" : package:\"$pkg\"#installed; }";
        let expected = "apt_get__predict() { pkg=\"$1\"; dpkg-query -W \"$pkg\"; }";
        assert_eq!(strip_one(authored), expected);
        assert_eq!(strip_one(authored), strip_one(authored));
    }

    /// A VERDICT funcdef strips with the verdict funcname suffix (24D §2/§3): the guard preamble
    /// def and the guard invocation must agree, so `apt-get.is_converged` mangles to
    /// `apt_get__is_converged`, body bytes otherwise verbatim (strip-fidelity). Pins the guard
    /// lane's strip alongside the probe lane's.
    #[test]
    fn verdict_body_strips_with_the_verdict_funcname() {
        use super::{lift_verdicts_converged, strip_verdict};
        let authored = "\
apt_get__is_converged() {
   verb=$1; shift
   case $verb in
   install) dpkg-query -W \"$1\" >/dev/null 2>&1 ;;
   esac
}";
        let mut i = Interner::default();
        let out = lift_verdicts_converged(&mut i, authored);
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let provider = out.value.providers().next().expect("one provider");
        let v = out.value.get(provider).expect("the verdict funcdef");
        let stripped = strip_verdict(authored, v, &i, "__is_converged");
        assert!(
            stripped.starts_with("apt_get__is_converged()"),
            "funcname mangled to the verdict suffix: {stripped}"
        );
        assert!(
            stripped.contains("dpkg-query -W \"$1\" >/dev/null 2>&1"),
            "the check body survives verbatim: {stripped}"
        );
        assert!(
            !stripped.contains(".is_converged("),
            "no period name remains: {stripped}"
        );
    }

    /// A TOUCHES funcdef strips with the `__touches` funcname suffix (24E §2/§9): a payload-bound
    /// body reaching `dpkg -L` ships to the derivation lane, so `apt-get.touches` mangles to
    /// `apt_get__touches`, body bytes (incl. the host-tool call the static tracer would ⊤ on)
    /// otherwise verbatim (strip-fidelity). Pins the derivation lane's strip alongside probe/guard.
    #[test]
    fn touches_body_strips_with_the_touches_funcname() {
        use super::strip_touches;
        use crate::touches::TouchesSet;
        // A payload-bound body reaching a coordinate-emitting host tool (`apt-manifest`, a SIMPLE
        // command — the dialect parser rejects the pipe/loop a raw `dpkg -L | sed` would need,
        // surfaced 24E-build). The static tracer ⊤s on it (NonPrintfCommand) ⇒ escalate.
        let authored = "\
apt_get__disturbs() {
   verb=$1; shift
   case $verb in
   install) apt-manifest \"$1\" ;;
   esac
}";
        let mut i = Interner::default();
        let set = TouchesSet::lift(&mut i, authored);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one provider");
        let t = set.value.get(provider).expect("the touches funcdef");
        let stripped = strip_touches(authored, t, &i);
        assert!(
            stripped.starts_with("apt_get__disturbs()"),
            "funcname mangled to the disturbs suffix: {stripped}"
        );
        assert!(
            stripped.contains("apt-manifest \"$1\""),
            "the host-tool call survives verbatim (the static tracer would ⊤ on it): {stripped}"
        );
    }

    /// A RESOLVER funcdef strips with the `__resolve` funcname suffix (24F §3): the shipped
    /// canonicalization-probe def and its per-coordinate invocation must agree, so `package.resolve`
    /// mangles to `package__resolve`, the host-tool body (which the static tracers would ⊤ on)
    /// otherwise verbatim (strip-fidelity). Pins the canonicalization lane's strip alongside
    /// probe/guard/derivation. NB the name segment IS the kind (`package`), kind-keyed not
    /// command-keyed.
    #[test]
    fn resolver_body_strips_with_the_resolve_funcname() {
        use super::strip_resolve;
        use crate::resolve::ResolverSet;
        let authored = "\
package__resolve() {
   dpkg-query -W -f '${Package}\\n' -- \"$1\" 2>/dev/null || printf '%s\\n' \"$1\"
}";
        let mut i = Interner::default();
        let set = ResolverSet::lift(&mut i, authored);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let kind = set.value.kinds().next().expect("one resolver kind");
        let r = set.value.get(kind).expect("the resolver funcdef");
        let stripped = strip_resolve(authored, r, &i);
        assert!(
            stripped.starts_with("package__resolve()"),
            "funcname mangled to the resolve suffix: {stripped}"
        );
        assert!(
            stripped.contains("dpkg-query -W -f '${Package}\\n' -- \"$1\""),
            "the host-tool body survives verbatim (the static tracers would ⊤ on it): {stripped}"
        );
        assert!(
            !stripped.contains(".resolve("),
            "no period name remains: {stripped}"
        );
    }

    /// A REACHES funcdef strips with the `__reaches` funcname suffix (24G §4): the typed-emission
    /// trailing marks (`: service` / `: file`) are annotation-LINEs the strip DELETES WHOLE
    /// (strip-fidelity, 23H §9.4 — the reached kind was already interned at LIFT, the vocabulary
    /// fence), leaving the raw emitting commands verbatim as plain runnable sh. Pins that the
    /// typed-emission grammar strips clean (`package.reaches` → `package__reaches`, no mark residue).
    #[test]
    fn reaches_body_strips_with_the_reaches_funcname() {
        use super::strip_reaches;
        use crate::reaches::ReachesSet;
        let authored = "\
package__disturbance_reaches_only() {
   printf '%s\\n' \"$1\"    : service
   dpkg -L \"$1\"           : file
}";
        let mut i = Interner::default();
        let set = ReachesSet::lift(&mut i, authored);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let kind = set.value.kinds().next().expect("one reaches kind");
        let r = set.value.get(kind).expect("the reaches funcdef");
        let stripped = strip_reaches(authored, r, &i);
        assert!(
            stripped.starts_with("package__disturbance_reaches_only()"),
            "funcname mangled to the disturbance_reaches_only suffix: {stripped}"
        );
        assert!(
            stripped.contains("printf '%s\\n' \"$1\"") && stripped.contains("dpkg -L \"$1\""),
            "the raw emitting commands survive verbatim: {stripped}"
        );
        assert!(
            !stripped.contains(": service") && !stripped.contains(": file"),
            "the typed-emission marks are deleted whole (no annotation residue): {stripped}"
        );
    }

    /// Byte-stability (R1c): strip is a deterministic function of its input.
    #[test]
    fn strip_is_byte_stable() {
        let authored = "systemctl__predict() { verb=$1; shift; svc : service = \"$1\"; case $verb in enable) systemctl is-enabled -- \"$svc\" : service:\"$svc\"#enabled ;; esac; }";
        assert_eq!(strip_one(authored), strip_one(authored));
        let stripped = strip_one(authored);
        assert!(stripped.starts_with("systemctl__predict()"));
        assert!(
            !stripped.contains(".predict("),
            "no period name: {stripped}"
        );
        assert!(
            !stripped.contains(": service:"),
            "no establish annotation: {stripped}"
        );
        assert!(
            stripped.contains("svc=\"$1\""),
            "identity stripped to assignment: {stripped}"
        );
    }
}

#[cfg(test)]
mod lift_failure_tests {
    use super::{Code, lift_failure, lift_predicts};
    use dorc_core::diag::{PredictOutOfDialect, PredictUnterminated, registry};
    use dorc_core::{BytePos, Interner, Span};

    /// The emit-vs-registry AGREEMENT pin (x3a-5/t-4 fix, `224` §10): `lift_failure` must source
    /// its severity from [`registry`], never hardcode it. This catches the exact regression the
    /// crosscheck found — a reversion to `Diagnostic::error(…)` would hardcode `Error`, so if a
    /// future registry edit moved either check code OFF `Error`, the emit would DISAGREE and this
    /// test trips. (At HEAD both are registry-`Error`, so the agreement holds AND the lowered output
    /// is byte-identical to the old hardcoded form — but the agreement is now structural, not luck.)
    ///
    /// Adversarial discipline (`inv-probe-sourced-values` spirit): the assertion does NOT hardcode
    /// `Severity::Error` on the emit side — it compares the EMITTED severity against the registry's,
    /// so the test stays correct (and keeps protecting) if the human re-grades the code at harvest.
    ///
    /// Span is ALWAYS real now (human ruling 22-q1): `lift_failure` takes a [`Span`], not
    /// `Option<Span>`, so this exercises the single real-span path per code (the EOF-synthesis path
    /// is pinned separately in [`eof_give_up_carries_a_real_end_span`]).
    #[test]
    fn lift_failure_severity_agrees_with_registry() {
        let mut interner = Interner::default();
        let span = Span::new(BytePos(3), BytePos(7));
        let want_unterm = registry(&Code::PredictUnterminated(PredictUnterminated {
            detail: String::new(),
        }))
        .severity;
        let d = lift_failure(true, span, "unterminated", &mut interner);
        assert_eq!(d.code.0, "predict-unterminated");
        assert_eq!(
            d.severity, want_unterm,
            "unterminated severity must equal the registry's, not a hardcoded value"
        );
        assert_eq!(d.span, Some(span), "span flows through unchanged");
        assert_eq!(
            d.message, "unterminated",
            "message is the bare text (no body added)"
        );

        let want_dialect = registry(&Code::PredictOutOfDialect(PredictOutOfDialect {
            detail: String::new(),
        }))
        .severity;
        let d = lift_failure(false, span, "out of dialect", &mut interner);
        assert_eq!(d.code.0, "predict-out-of-dialect");
        assert_eq!(
            d.severity, want_dialect,
            "out-of-dialect severity must equal the registry's, not a hardcoded value"
        );
        assert_eq!(d.span, Some(span));
        assert_eq!(d.message, "out of dialect");
    }

    /// An EOF give-up now carries a REAL zero-width end-of-input span, never a span-less mint
    /// (human ruling 22-q1: point the UI at end-of-file for a truncated/chopped body). Pins the
    /// observable change — a truncated check body's diagnostic gains a `Some(span)` where it
    /// previously had `None` — through the public [`lift_predicts`] entry, so it exercises the real
    /// `eof_span()`-via-`fail_here`/`true_with` wiring rather than a hand-built span.
    #[test]
    fn eof_give_up_carries_a_real_end_span() {
        let mut interner = Interner::default();
        // An unterminated function body: the lexer runs out of tokens inside `parse_block`, so
        // `true_with` fires at EOF (the pre-22-q1 span-less case).
        let src = "x__predict() { x : K = \"$1\"";
        let lifted = lift_predicts(&mut interner, src);
        let diag = lifted
            .diags
            .first()
            .expect("an unterminated body yields a lift diagnostic");
        let span = diag
            .span
            .expect("the EOF give-up carries a real span now, not None (22-q1)");
        assert_eq!(
            span.lo, span.hi,
            "the synthesized EOF span is zero-width (a caret at end-of-input)"
        );
        // It lands at end-of-input — the last real token's `hi`, somewhere PAST the file start
        // (so it is genuinely the EOF position, not the byte-0 `ZERO_SPAN` fallback) and within
        // the source bytes. We avoid pinning the exact offset (lexer token-end detail), only the
        // load-bearing property: a real, non-zero end-of-input caret.
        let src_len = u32::try_from(src.len()).expect("fixture fits u32");
        assert!(
            span.hi.0 > 0 && span.hi.0 <= src_len,
            "the EOF caret lands where input ran out (0 < {} <= {src_len}), not at byte 0",
            span.hi.0
        );
    }

    /// MUST-EMIT pin (x3n PINNED-BY-NOTHING, B8): drive the production `lift_predicts` path for an
    /// UNTERMINATED body and pin the registered code `predict-unterminated`. The existing
    /// [`eof_give_up_carries_a_real_end_span`] drives the same path but pins only the SPAN, and
    /// [`lift_failure_severity_agrees_with_registry`] pins the code via a DIRECT `lift_failure`
    /// call (a construction, the x3a-B/t-1 vacuity). This closes the gap: the code identity is
    /// asserted on a real source-driven give-up.
    #[test]
    fn unterminated_predict_body_emits_predict_unterminated_from_lift() {
        let mut i = Interner::default();
        let lifted = lift_predicts(&mut i, "x__predict() { x : K = \"$1\"");
        assert!(
            lifted
                .diags
                .iter()
                .any(|d| d.code.0 == "predict-unterminated"),
            "an unterminated check body must disclose predict-unterminated: {:?}",
            lifted.diags
        );
    }

    /// MUST-EMIT pin (x3n PINNED-BY-NOTHING, B8): drive `lift_predicts` for an OUT-OF-DIALECT body
    /// and pin `predict-out-of-dialect`. The check dialect is a strict subset of sh with no `for`
    /// loop, so a `for` in the body is rejected via `fail_here` (the `is_unterminated == false`
    /// path). No prior test drove this give-up from source — only the direct-construction
    /// `lift_failure(false, …)` did. Pins the registered code on a real source-driven path.
    #[test]
    fn out_of_dialect_predict_body_emits_predict_out_of_dialect_from_lift() {
        let mut i = Interner::default();
        let lifted = lift_predicts(&mut i, "x__predict() { for y in a b; do shift; done; }");
        assert!(
            lifted
                .diags
                .iter()
                .any(|d| d.code.0 == "predict-out-of-dialect"),
            "a `for` loop (outside the check dialect) must disclose predict-out-of-dialect: {:?}",
            lifted.diags
        );
    }
}
