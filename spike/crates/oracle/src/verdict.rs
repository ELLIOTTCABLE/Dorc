//! `verdict` — the guard-verdict function lift (rul-role-split / rul24-vouch-is-verdict-authoring,
//! 24A §1c / 24D §3). The STATIC half of the vouch: authoring an `<provider>.is_converged()` /
//! `.is_diverged()` verdict function IS the vouching act, and this module decides — for a site's
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
//! # Declines: `return` and the inert builtins (find-return-vouches, 24C; the hz-refusepath fence)
//!
//! A reached `return N` is a DECLINE, never a vouch (24A §1c's sanctioned decline): `return`
//! author-forces the function's rc PAST any check (rul-rc-partition: ≥2 confused, 1 complement,
//! and even `return 0` is a vacuous unconditional "converged"), so it is never a check result —
//! it ENDS the path declined. The inert fixed-rc builtins `false` (rc 1 = complement) / `:` /
//! `true` (rc 0 VACUOUSLY) likewise run no check ⇒ never vouch. Before this
//! ([`VerdictResolution`] modeled only "reached a command"), a `*) return 2 ;;` catch-all reached
//! [`Tracer::run_command`] and wrongly VOUCHED — harmless in the guard tier (a declined path's
//! `( check )` returns non-zero ⇒ `||` runs the original) but a wrong-ELISION once a vouch
//! licenses full skip (Part B). This is a TRACER fix.
//!
//! **Scope note (ru-26 churn-avoidance):** `return` still parses as a plain command (the dialect
//! has no `Stmt::Return`), caught HERE in the tracer, not at parse. A bare test-led shorthand
//! `[ … ] || return N` remains out of dialect and ⊤-rejects at LIFT — a deliberate parser
//! scope-cut, NOT closed by this fix (extending the parser is out of the #12 scope): a verdict
//! function needing that arity-refuse spells it in-dialect as `if [ … ]; then return N; fi`.
//!
//! `inv-referent-agnostic`: the tracer never decodes the entity's text — it reuses the predict
//! argparse primitives ([`resolve_word`]/[`eval_test`]/[`pattern_matches`]) to find the reached
//! path, then asks only "did an authored command run there", never what the command *means*.

use std::collections::BTreeMap;

use dorc_core::{Carrier, Interner, Symbol};
use dorc_syntax::sem::UnsetPolicy;

use crate::predict::{
    CaseArm, Command, Predict, PredictSet, Stmt, Test, TopReason, Word, eval_test,
    lift_verdicts_converged, lift_verdicts_diverged, pattern_matches, resolve_word,
};

/// Which sense a provider's verdict function was authored in (rul-role-split: "sense DECLARED BY
/// NAME"). The guard emitter maps this to the `||`-glue: [`Converged`](VerdictSense::Converged) is
/// the direct glue `( f_is_converged … ) || <orig>`; [`Diverged`](VerdictSense::Diverged) is the
/// lossless sense-flip `( f_is_diverged …; [ $? -eq 1 ] ) || <orig>` (rul-rc-partition).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictSense {
    /// `is_converged`: 0 = converged (skip-licensable), 1 = diverged, ≥2 = confused (run).
    Converged,
    /// `is_diverged`: 0 = diverged, 1 = converged (skip-licensable), ≥2 = confused (run).
    Diverged,
}

impl VerdictSense {
    /// The mangled funcname suffix this sense strips to (`crate::predict::strip_verdict`).
    #[must_use]
    pub fn mangled_suffix(self) -> &'static str {
        match self {
            VerdictSense::Converged => "__is_converged",
            VerdictSense::Diverged => "__is_diverged",
        }
    }
}

/// The set of verdict funcdefs lifted from one oracle file — the two senses kept apart (a provider
/// declares ONE sense by name; if a file declares both for a provider, [`get`](VerdictSet::get)
/// prefers converged deterministically). Reuses the predict dialect AST ([`Predict`]); only the
/// scanned name-suffix and the collected outcome differ.
#[derive(Debug, Clone, Default)]
pub struct VerdictSet {
    converged: PredictSet,
    diverged: PredictSet,
}

impl VerdictSet {
    /// Lift every `<provider>.is_converged` / `.is_diverged` funcdef in `src`. Fail-soft
    /// (`inv-no-throw`) and deterministic (`inv-determinism`) — the same contract as
    /// [`crate::predict::lift_predicts`], routed through the shared role-parametrized parser.
    #[must_use]
    pub fn lift(interner: &mut Interner, src: &str) -> Carrier<Self> {
        let conv = lift_verdicts_converged(interner, src);
        let div = lift_verdicts_diverged(interner, src);
        // Concatenate both lifts' diagnostics; the value is the paired sets.
        conv.and_then(|converged| {
            div.map(|diverged| Self {
                converged,
                diverged,
            })
        })
    }

    /// The verdict funcdef for a provider + its declared [`VerdictSense`], if the file authored
    /// one. Prefers `is_converged` when both exist (deterministic; a provider authoring both is a
    /// fixture oddity, not a designed shape — rul-role-split declares ONE sense).
    #[must_use]
    pub fn get(&self, provider: Symbol) -> Option<(&Predict, VerdictSense)> {
        if let Some(p) = self.converged.get(provider) {
            return Some((p, VerdictSense::Converged));
        }
        self.diverged
            .get(provider)
            .map(|p| (p, VerdictSense::Diverged))
    }

    /// Providers with a lifted verdict funcdef, in deterministic order (converged then diverged;
    /// duplicates possible if a provider authored both — the caller dedups via [`get`](Self::get)).
    pub fn providers(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.converged.providers().chain(self.diverged.providers())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.converged.is_empty() && self.diverged.is_empty()
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
    /// The argv reached NO authored check — an unhandled `case`, an `if`-false with no `else`, an
    /// empty body, OR a reached DECLINE idiom (`return N` / `false` / `:` / `true`;
    /// find-return-vouches, 24C). A DECLINE (24A §1c "an unhandled path"): no witness forms ⇒ the
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
}

impl VerdictTop {
    /// A short human-readable form for diagnostics/provenance.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            VerdictTop::EmptyArgv => "empty argv",
            VerdictTop::NonConcreteWord(w) => w,
            VerdictTop::BudgetExceeded => "iteration budget exceeded",
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
    let budget = argv.len().saturating_mul(4).saturating_add(BUDGET_CONSTANT);
    let mut tr = Tracer {
        positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
        vars: BTreeMap::new(),
        reached_command: false,
        budget,
        steps: 0,
    };
    match tr.run_block(&verdict.body) {
        Flow::Normal => {
            if tr.reached_command {
                VerdictResolution::Vouched
            } else {
                VerdictResolution::Declined
            }
        }
        // A reached `return` declined the path outright (find-return-vouches, 24C): it exited the
        // function with an author-forced rc that is never a check result, overriding any earlier
        // reached check (a `return` past a check makes the path's rc vacuous ⇒ still a decline).
        Flow::Declined => VerdictResolution::Declined,
        Flow::Top(reason) => VerdictResolution::Top(reason),
    }
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
    budget: usize,
    steps: usize,
}

enum Flow {
    Normal,
    /// A reached `return` (find-return-vouches, 24C): the verdict function exited with an
    /// author-forced rc that is never a check result ⇒ the path DECLINES. Propagates up like
    /// [`Flow::Top`], ending the block/loop.
    Declined,
    Top(VerdictTop),
}

impl Tracer {
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
            // An annotation desugars to a binding (as in touches); a bare mark is a no-op. Neither
            // is a "check command", so neither vouches on its own.
            Stmt::Annotation(anno) => {
                if let Some(value) = &anno.value
                    && let Ok(v) = self.resolve(value)
                {
                    self.vars.insert(anno.name, v);
                }
                Flow::Normal
            }
            Stmt::Mark(_) => Flow::Normal,
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
    /// But NOT every reached command is a check (find-return-vouches, 24C): a DECLINE idiom runs
    /// no measurement, so it never vouches. `return` ([`Decline::Return`]) author-forces the rc
    /// past any check ⇒ ENDS the path DECLINED; the inert fixed-rc builtins `false`/`:`/`true`
    /// ([`Decline::Inert`]) run but record no vouch (the path continues). Only a resolved,
    /// non-idiom command sets the vouch.
    fn run_command(&mut self, cmd: &Command) -> Flow {
        for w in &cmd.words {
            if let Err(reason) = self.resolve(w) {
                return Flow::Top(top_from_word(reason));
            }
        }
        match decline_idiom(cmd.words.first()) {
            // `return` exits the function declined — never a check (rul-rc-partition: an
            // author-forced rc, even `return 0`, is vacuous, not a measurement).
            Some(Decline::Return) => Flow::Declined,
            // `false`/`:`/`true` ran but measured nothing ⇒ no vouch; the path continues.
            Some(Decline::Inert) => Flow::Normal,
            // A real check ran on this path ⇒ the vouch signal (hz-refusepath: only here).
            None => {
                self.reached_command = true;
                Flow::Normal
            }
        }
    }

    /// Resolve a word in strict context (`Unresolved` on a past-end positional) — the vouch's
    /// constprop half must resolve concretely, exactly as a predict annotation value must.
    fn resolve(&self, word: &Word) -> Result<String, TopReason> {
        resolve_word(word, &self.positionals, &self.vars, UnsetPolicy::Unresolved)
    }
}

/// A reached command that is a DECLINE idiom, not an authored check (find-return-vouches, 24C /
/// rul-rc-partition / the hz-refusepath fence). None of these MEASURES state, so none vouches.
enum Decline {
    /// `return …` — exits the function; the path DECLINES (author-forced rc, never a check
    /// result — ≥2 confused, 1 complement, and even `return 0` is a vacuous "converged").
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
            Stmt::Assign { .. } | Stmt::Shift { .. } | Stmt::Annotation(_) | Stmt::Mark(_) => {}
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
        let (verdict, _sense) = set.value.get(provider).expect("the verdict funcdef");
        evaluate_verdict(verdict, argv)
    }

    // Mirrors the real apt argparse: flag-strip before and after the verb, bind the verb, and
    // check the operand — the guard23 flagship's `is_converged` shape.
    const APT: &str = "\
apt-get.is_converged() {
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
apt-get.is_converged() {
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
    fn diverged_sense_lifts_and_carries_its_sense() {
        let src = "\
systemctl.is_diverged() {
   verb=$1; shift
   case $verb in
   enable) ! systemctl is-enabled -- \"$1\" >/dev/null 2>&1 ;;
   esac
}";
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one verdict funcdef");
        let (_p, sense) = set.value.get(provider).expect("the verdict funcdef");
        assert_eq!(sense, VerdictSense::Diverged);
        assert_eq!(sense.mangled_suffix(), "__is_diverged");
    }

    #[test]
    fn catchall_return_declines_never_vouches() {
        // find-return-vouches (24C): a `*) return 2 ;;` catch-all REACHED by an unhandled verb is
        // a DECLINE (rul-rc-partition: return ≥2 = confused ⇒ run), NEVER a vouch. Before this fix
        // it wrongly VOUCHED (a reached command was the vouch) — which, once a vouch licenses full
        // skip (Part B), would ELIDE a mutation on a path the author declined. The `install` arm
        // (a real check) still vouches; only the return-arm declines.
        let src = "\
apt-get.is_converged() {
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
    fn arity_gate_return_declines_multi_operand() {
        // The in-dialect arity-refuse (the refusepath floor's form): `if [ "$2" != "" ]; then
        // return 2; fi`. A multi-operand invocation hits the `return 2` ⇒ Declined; a single
        // operand skips it and reaches the real check ⇒ Vouched. This is the shape a verdict
        // function uses instead of the out-of-dialect `[ … ] || return N` shorthand.
        let src = "\
apt-get.is_converged() {
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
                "apt-get.is_converged() {{ verb=$1; shift; case $verb in restart) {inert} ;; esac }}"
            );
            assert_eq!(
                trace(&src, &["restart", "nginx"]),
                VerdictResolution::Declined,
                "`{inert}` is an inert non-check ⇒ Declined, never a vouch"
            );
        }
    }
}
