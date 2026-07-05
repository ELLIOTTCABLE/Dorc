//! `reaches` — the REACH function lift (24G §4, the FIFTH role-sibling; the cross-author
//! footprint-EXPANSION mechanism). A kind-OWNER may ship `<kind>.reaches()`: a body invoked with an
//! ENTITY whose EMITTING ARMS declare, per kind, what touching that entity DRAGS WITH IT —
//!
//! ```sh
//! package.reaches() {
//!    printf '%s\n' "$1"    : service     # STATIC arm — traced at plan time, ships nothing
//!    dpkg -L "$1"          : file        # DYNAMIC arm — escalates, runs read-only at probe
//! }
//! ```
//!
//! # Typed emission (24G §4 — the load-bearing novelty)
//!
//! An emitting arm's KIND rides a TRAILING ANNOTATION on the emitting command (`: service`); the
//! command's stdout lines are RAW ENTITIES (no `kind:` prefix, no `| sed` dressing). The kind symbol
//! is fixed by the annotation at LIFT — a host can never mint a kind at runtime (the vocabulary
//! fence, closed at analysis; contrast the `touches()` stringly readback that interns kind-strings
//! out of host stdout — that migration is a separate, later task, NOT touched here). Each annotated
//! arm is ONE capture unit (24G §4 per-arm-capture cost): the STATIC form is a traceable `printf`
//! (this module resolves it via the shared predict value-flow, so `$1` → the entity); the DYNAMIC
//! form is a host command / pipeline that ⊤s at trace and ESCALATES (ships strip-only, runs read-only
//! in the probe, its stdout captured per-arm so the controller joins arm→kind statically).
//!
//! # Kind-keyed, invoked with an ENTITY (like `resolve()`, 24F §3)
//!
//! `reaches()` is the second per-KIND family member (the kind-owner holds the nouns — 23M
//! contribution-vs-identity), NOT a per-command role-sibling: `<kind>.reaches` interns its name as
//! the SAME [`Symbol`] the coordinate's `KindId` wraps. The engine expands EVERY footprint coordinate
//! of a reach-bearing kind through it, WHOEVER emitted the coordinate (the cross-author point).
//!
//! # Soundness posture (`inv-kfail`, apply direction; `inv-referent-agnostic`)
//!
//! Expansion only ever WIDENS a footprint (24G — the safe direction: a wider footprint HITs more, so
//! it demotes-toward-run, never elides more). So OVER-emission is safe here: a control-flow branch
//! this module cannot statically prove dead is traced anyway (over-approximation widens). The emitted
//! `kind`/`entity` fragments are OPAQUE (`inv-referent-agnostic`): the engine interns the annotated
//! kind + the raw entities into the shared vocabulary, never decoding their meaning. Un-annotated
//! emission means NOTHING (24G §5 — contributes no coordinate and draws a smell, NEVER a refusal;
//! the lift's only hard errors stay syntax + genuine static conflicts, enforced by the parser).

use std::collections::BTreeMap;

use dorc_core::{Carrier, Interner, Span, Symbol};
use dorc_syntax::sem::UnsetPolicy;

use crate::predict::{
    Command, Predict, PredictSet, Stmt, TopReason, Word, lift_reaches, resolve_word,
};
use crate::touches::printf_lines;

/// The set of `<kind>.reaches()` funcdefs lifted from one oracle file, keyed by KIND (like
/// [`crate::resolve::ResolverSet`]). Reuses the predict dialect AST ([`Predict`]) — a reaches
/// funcdef has the identical body grammar (with the pipeline-may-carry-a-mark carve-out, 24G §4);
/// only its name-suffix (`.reaches`) and its purpose (emit typed entity-coordinates) differ.
#[derive(Debug, Clone, Default)]
pub struct ReachesSet(PredictSet);

impl ReachesSet {
    /// Lift every `<kind>.reaches` / `<kind>__reaches` funcdef in `src`. Fail-soft (`inv-no-throw`)
    /// and deterministic (`inv-determinism`) — the same contract as [`crate::predict::lift_predicts`],
    /// routed through the shared role-parametrized parser.
    #[must_use]
    pub fn lift(interner: &mut Interner, src: &str) -> Carrier<Self> {
        lift_reaches(interner, src).map(Self)
    }

    /// The reach funcdef for a KIND, if the file declared one. `kind` is the SAME interned symbol a
    /// coordinate's `KindId` wraps (the vocabulary fence — one interned universe).
    #[must_use]
    pub fn get(&self, kind: Symbol) -> Option<&Predict> {
        self.0.get(kind)
    }

    /// Kinds with a lifted reach-function, in deterministic order (the engine expands every footprint
    /// coordinate of such a kind through it).
    pub fn kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.0.providers()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// One emitting arm of a `reaches()` body, resolved over a concrete entity (24G §4). The KIND is the
/// arm's trailing-annotation kind (opaque — interned at the wiring boundary into the shared
/// vocabulary); the OUTCOME is the arm's per-maturity shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReachArm {
    /// The trailing-annotation kind (`service`/`file`) — the kind of every entity this arm emits.
    /// Opaque (`inv-referent-agnostic`); fixed at LIFT (the vocabulary fence, 24G §4).
    pub kind: String,
    /// The arm's position in the body (0-based), the DYNAMIC readback demux key: a dynamic arm's
    /// per-coordinate stdout rides `reach <coord> arm=<index> entity=<line>`, and this index re-keys
    /// each readback line back to THIS arm's `kind`.
    pub index: usize,
    /// The arm's per-maturity outcome.
    pub outcome: ArmOutcome,
}

/// A reaches arm's per-maturity resolution (24G §4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmOutcome {
    /// A STATIC arm (a traceable `printf`): the raw entity lines it emits for THIS entity, resolved
    /// at plan time (no host). Each line is an entity in the arm's [`kind`](ReachArm::kind).
    Static(Vec<String>),
    /// A DYNAMIC arm (a host command / pipeline): it ⊤'d at trace ⇒ escalates. `cmd_span` is the
    /// arm's command source span (the trailing mark already EXCLUDED by the parser), so the cli ships
    /// `src[cmd_span]` as a strip-clean per-arm wrapper, runs it read-only per coordinate, and reads
    /// its stdout back into entities (`reach <coord> arm=<index> entity=<line>`).
    Dynamic {
        /// The arm command's byte-span (mark-free) — the cli slices + wraps it for the reach probe.
        cmd_span: Span,
    },
}

/// The full expansion of a `reaches()` body over one entity (24G §4): its emitting arms plus the spans
/// of any un-annotated emitting commands (24G §5 — each contributes NOTHING and draws a smell, never
/// a refusal).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReachExpansion {
    /// The annotated emitting arms, in body order.
    pub arms: Vec<ReachArm>,
    /// The spans of un-annotated emitting commands (24G §5 smell-diagnostic anchors). Present ⇒ the
    /// author emitted without typing the emission; the cli surfaces a smell and draws nothing from it.
    pub smells: Vec<Span>,
}

/// Trace a `reaches()` body over a concrete `entity` (24G §4). Walks the body arm-by-arm, classifying
/// each emitting command STATIC (a traceable `printf` — its lines resolved for `entity`) or DYNAMIC
/// (a host command / pipeline — escalates), and collecting un-annotated commands as smells.
///
/// Pure + total (`inv-determinism`/`inv-no-throw`): no clock/RNG/IO, ordered collections only, every
/// path returns. OVER-approximates deliberately (control-flow arms are all traced) — for footprint
/// EXPANSION over-emission WIDENS the footprint, the safe direction (`inv-kfail`, apply: widening
/// demotes toward run, never elides more).
#[must_use]
pub fn evaluate_reaches(reaches: &Predict, entity: &str) -> ReachExpansion {
    let mut w = Walk {
        positionals: vec![entity.to_owned()],
        vars: BTreeMap::new(),
        arms: Vec::new(),
        smells: Vec::new(),
        next_index: 0,
    };
    w.run_block(&reaches.body);
    ReachExpansion {
        arms: w.arms,
        smells: w.smells,
    }
}

/// The reaches walker: resolves words through the SAME value-flow the predict/touches tracers use
/// ([`resolve_word`]), so a static arm's `$1` binds to the entity exactly as a predict annotation's
/// `$1` binds to an operand (the 24A §1b vocabulary fence). Deliberately a small dedicated walk (not
/// the touches `Emitter`) — reaches collects PER-ARM typed emissions, a fundamentally different shape
/// from touches's single all-or-nothing coordinate list.
struct Walk {
    positionals: Vec<String>,
    vars: BTreeMap<Symbol, String>,
    arms: Vec<ReachArm>,
    smells: Vec<Span>,
    next_index: usize,
}

impl Walk {
    fn run_block(&mut self, body: &[Stmt]) {
        for stmt in body {
            self.run_stmt(stmt);
        }
    }

    fn run_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign { name, value } => {
                // A non-concrete rvalue leaves the var unbound (a later use degrades to ⊤ ⇒ the arm
                // escalates) — never a bogus value. Same posture as the predict/touches evaluators.
                if let Ok(v) = self.resolve(value) {
                    self.vars.insert(*name, v);
                }
            }
            // `shift` is unusual in a reaches body (invoked with a single entity), but model it
            // faithfully (drop leading positionals) so a body that argparses does not misresolve.
            Stmt::Shift { count } => {
                let n = (count.unwrap_or(1) as usize).min(self.positionals.len());
                self.positionals.drain(0..n);
            }
            Stmt::Command(cmd) => self.run_command(cmd),
            // Recurse control-flow, tracing ALL arms (over-approximation — over-emission WIDENS the
            // footprint, the safe direction for expansion; 24G inv-kfail). A `while` body is walked
            // once (a reaches loop over `$1` is not idiomatic; a single pass suffices for emission).
            Stmt::Case { arms, .. } => {
                for a in arms {
                    self.run_block(&a.body);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.run_block(then_body);
                self.run_block(else_body);
            }
            Stmt::While { body, .. } => self.run_block(body),
            Stmt::Annotation(a) => {
                if let Some(value) = &a.value
                    && let Ok(v) = self.resolve(value)
                {
                    self.vars.insert(a.name, v);
                }
            }
            Stmt::Mark(_) => {}
        }
    }

    fn run_command(&mut self, cmd: &Command) {
        let Some(mark) = &cmd.mark else {
            // 24G §5: an un-annotated emitting arm contributes NOTHING and draws a smell — NEVER a
            // refusal. Record the span so the wiring can surface the smell-diagnostic.
            self.smells.push(cmd.span);
            return;
        };
        let kind = mark.target.kind.clone();
        let index = self.next_index;
        self.next_index = self.next_index.saturating_add(1);
        // Classify STATIC vs DYNAMIC. A pipeline never statically resolves (24E §14 / 24G §4 —
        // trace-conservatively) ⇒ DYNAMIC. Otherwise trace it AS printf: a resolvable printf ⇒
        // STATIC (its lines are the entities); a non-printf / unresolvable / unmodeled-format
        // command ⇒ DYNAMIC (escalate — a host command or a printf we cannot model runs fine on the
        // host, so shipping it is safe: over-emission widens).
        let outcome = if cmd.pipeline {
            ArmOutcome::Dynamic { cmd_span: cmd.span }
        } else {
            match self.trace_printf(cmd) {
                Some(entities) => ArmOutcome::Static(entities),
                None => ArmOutcome::Dynamic { cmd_span: cmd.span },
            }
        };
        self.arms.push(ReachArm {
            kind,
            index,
            outcome,
        });
    }

    /// Trace a command AS `printf` → its raw entity lines (24G §4 typed emission — lines are bare
    /// entities, NOT `kind:entity` coordinates; contrast [`crate::touches`]). `None` iff the command
    /// is not a statically-resolvable `printf` (non-printf verb, an unresolved word, or an unmodeled
    /// format) ⇒ the caller escalates it to a dynamic arm.
    fn trace_printf(&self, cmd: &Command) -> Option<Vec<String>> {
        let (verb, rest) = cmd.words.split_first()?;
        if self.resolve(verb).ok()? != "printf" {
            return None;
        }
        let (format_word, arg_words) = rest.split_first()?;
        let format = self.resolve(format_word).ok()?;
        let mut args = Vec::with_capacity(arg_words.len());
        for w in arg_words {
            args.push(self.resolve(w).ok()?);
        }
        // `printf_lines` reuses the touches printf model (`\n`/`\t`/`\\`/`%s`/`%%`); a ⊤ (unmodeled
        // directive, arg-count mismatch) ⇒ `None` ⇒ the arm escalates (safe: over-emission widens).
        printf_lines(&format, &args).ok()
    }

    fn resolve(&self, word: &Word) -> Result<String, TopReason> {
        resolve_word(word, &self.positionals, &self.vars, UnsetPolicy::Unresolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::Interner;

    /// Lift the sole reaches funcdef from `src` and trace it over `entity`.
    fn expand(src: &str, entity: &str) -> ReachExpansion {
        let mut i = Interner::default();
        let set = ReachesSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let kind = set.value.kinds().next().expect("one reaches funcdef");
        let reaches = set.value.get(kind).expect("the reaches funcdef");
        evaluate_reaches(reaches, entity)
    }

    /// A `<kind>.reaches` funcdef lifts, keyed by the KIND symbol (not a command word). The
    /// vocabulary fence: the lifted key is the SAME symbol `KindId(intern("package"))` wraps.
    #[test]
    fn reaches_lifts_keyed_by_kind() {
        let mut i = Interner::default();
        let src = "package.reaches() { printf '%s\\n' \"$1\" : service; }";
        let set = ReachesSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let kind = i.intern("package");
        assert!(
            set.value.get(kind).is_some(),
            "the package reaches is keyed by the kind symbol"
        );
        assert_eq!(set.value.kinds().count(), 1, "exactly one reaches kind");
    }

    /// The flagship shape (24G §4): a STATIC printf arm (traced → the entity, in kind `service`) and
    /// a DYNAMIC host-command arm (`dpkg -L` — escalates, kind `file`). One capture unit per arm.
    #[test]
    fn static_and_dynamic_arms_classify_and_type() {
        let src = "\
package.reaches() {
   printf '%s\\n' \"$1\"    : service
   dpkg -L \"$1\"           : file
}";
        let e = expand(src, "nginx");
        assert!(
            e.smells.is_empty(),
            "both arms are annotated: {:?}",
            e.smells
        );
        assert_eq!(e.arms.len(), 2);
        // Arm 0: static `service` emission of the entity itself.
        assert_eq!(e.arms[0].kind, "service");
        assert_eq!(e.arms[0].index, 0);
        assert_eq!(
            e.arms[0].outcome,
            ArmOutcome::Static(vec!["nginx".to_owned()])
        );
        // Arm 1: dynamic `file` escalation (its span ships to the reach probe).
        assert_eq!(e.arms[1].kind, "file");
        assert_eq!(e.arms[1].index, 1);
        assert!(
            matches!(e.arms[1].outcome, ArmOutcome::Dynamic { .. }),
            "dpkg -L escalates: {:?}",
            e.arms[1].outcome
        );
    }

    /// An un-annotated emitting arm contributes NOTHING and draws a smell (24G §5 — never a refusal).
    #[test]
    fn unannotated_arm_is_a_smell_not_a_refusal() {
        // The lift itself is CLEAN (no diagnostic — silence is never punished); the smell is a
        // per-arm advisory the expansion surfaces.
        let src = "package.reaches() { dpkg -L \"$1\"; }";
        let e = expand(src, "nginx");
        assert!(e.arms.is_empty(), "no typed arm ⇒ contributes nothing");
        assert_eq!(e.smells.len(), 1, "the un-annotated emitting arm smells");
    }

    /// A pipeline arm CARRIES its trailing mark (the 24G §4 carve-out) and escalates (dynamic —
    /// trace-conservatively). The mark types the emission; the pipeline ships byte-exact.
    #[test]
    fn pipeline_arm_carries_mark_and_escalates() {
        let src = "package.reaches() { dpkg -L \"$1\" | grep '\\.service$' : service; }";
        let e = expand(src, "nginx");
        assert!(
            e.smells.is_empty(),
            "the pipeline carried its mark: {:?}",
            e.smells
        );
        assert_eq!(e.arms.len(), 1);
        assert_eq!(e.arms[0].kind, "service");
        assert!(
            matches!(e.arms[0].outcome, ArmOutcome::Dynamic { .. }),
            "a pipeline escalates: {:?}",
            e.arms[0].outcome
        );
    }

    /// A static printf may emit SEVERAL entity lines (one arm, many entities — all in the arm's kind).
    #[test]
    fn static_arm_may_emit_multiple_entities() {
        let src = "svc.reaches() { printf '%s\\n%s\\n' \"$1\" \"$1\" : unit; }";
        let e = expand(src, "nginx");
        assert_eq!(e.arms.len(), 1);
        assert_eq!(
            e.arms[0].outcome,
            ArmOutcome::Static(vec!["nginx".to_owned(), "nginx".to_owned()])
        );
    }

    /// A resolver-less file with NO reaches funcdef yields an empty set (a reach-less kind — the
    /// footprint stays un-expanded, the per-kind gradual floor).
    #[test]
    fn no_reaches_is_empty_set() {
        let mut i = Interner::default();
        let set = ReachesSet::lift(
            &mut i,
            "apt-get.touches() { printf 'package:%s\\n' \"$1\"; }",
        );
        assert!(set.diags.is_empty());
        assert!(
            set.value.is_empty(),
            "no .reaches funcdef ⇒ empty reaches set"
        );
    }
}
