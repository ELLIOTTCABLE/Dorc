//! `touches` — the at-most **footprint** lift (`provides-behavior` sub-shape 3;
//! ORACLE_PROVIDES.md, 24A §1b, 23M frame-rule mechanism).
//!
//! A rich oracle may grow a third role-sibling next to `predict()` / `is_converged()`:
//! `<provider>.touches()`. Invoked with a site's argv (the same contract as its
//! siblings), its body is ordinary author sh that `printf`s the **entity-coordinates
//! the verb MUTATES**, one per line (`kind:entity`; an empty entity is the kind's
//! singleton — `pkgindex:`). Emitting anything for a reached path is the **at-most
//! claim** ("whatever else this touches is residue I answer for"); emitting NOTHING is
//! *no claim* ⇒ the wall stands (silence = wall, 23O §2).
//!
//! This module is the STATIC lift only: it parses `.touches` funcdefs (reusing the
//! `predict` dialect — one grammar, [`crate::predict::lift_touches`]) and traces a
//! concrete argv through the body to collect its emitted coordinates. It NEVER executes
//! the body — probe-time derivation of payload-bound footprints (`dpkg -L`) is Stage 4.
//!
//! # Soundness posture (`inv-kfail`, apply direction)
//!
//! A footprint is an *at-most* claim a downstream elision leans on to survive this
//! command's run, so a WRONG footprint silently under-executes someone else's line
//! (rul24-divergence-is-the-game). The lift therefore biases every ambiguity to ⊤
//! ([`TouchesResolution::Top`]) — an unresolved argv word, a non-`printf` command, an
//! unmodeled printf directive, a malformed coordinate. **All-or-nothing** (TC-4): ANY ⊤
//! fragment on the reached path discards the WHOLE footprint (a partial at-most claim is
//! incoherent), and the site walls exactly as if it had no `touches()` at all.
//!
//! `inv-referent-agnostic`: the emitted `kind:entity` fragments are OPAQUE strings here;
//! the engine interns them into the shared vocabulary (KindId + entity token) at the
//! wiring boundary and NEVER decodes their text (24A §1b vocabulary fence).

use std::collections::BTreeMap;

use dorc_core::{Carrier, Interner, Symbol};
use dorc_syntax::sem::UnsetPolicy;

use crate::predict::{
    Command, Predict, PredictSet, Stmt, Word, eval_test, lift_touches, pattern_matches,
    resolve_word,
};

/// The set of `<provider>.touches()` funcdefs lifted from one oracle file, keyed by
/// provider. Reuses the `predict` dialect AST ([`Predict`]) — a touches funcdef has the
/// identical body grammar; only its emitted content differs (coordinates, not
/// annotations + probe bodies).
#[derive(Debug, Clone, Default)]
pub struct TouchesSet(PredictSet);

impl TouchesSet {
    /// Lift every `<provider>.touches` / `<provider>__touches` funcdef in `src`. Fail-soft
    /// (`inv-no-throw`) and deterministic (`inv-determinism`) — the same contract as
    /// [`crate::predict::lift_predicts`], routed through the shared role-parametrized parser.
    #[must_use]
    pub fn lift(interner: &mut Interner, src: &str) -> Carrier<Self> {
        lift_touches(interner, src).map(Self)
    }

    /// The touches funcdef for a provider, if the file declared one.
    #[must_use]
    pub fn get(&self, provider: Symbol) -> Option<&Predict> {
        self.0.get(provider)
    }

    /// Providers with a lifted touches funcdef, in deterministic order.
    pub fn providers(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.0.providers()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// One entity-coordinate a `touches()` body emitted, split syntactically and left OPAQUE
/// (`inv-referent-agnostic` — never decoded here; the wiring interns it). `kind:entity`,
/// where `entity == None` is the kind's singleton (`pkgindex:` ⇒ the one package index).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmittedCoord {
    /// The kind fragment (everything before the FIRST `:`). Opaque.
    pub kind: String,
    /// The entity fragment (everything after the first `:`), or `None` for a singleton
    /// (empty entity). Opaque. May itself contain `:` / `.` (`kernel.Sysctl:net.ipv4.ip_forward`).
    pub entity: Option<String>,
}

/// The result of tracing a `touches()` body over a concrete argv (`inv-superposition`: a
/// phase-agnostic fact; the phased caller collapses it). Either the reached emissions or a
/// single safe ⊤ degrade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TouchesResolution {
    /// The reached path emitted these coordinates (possibly EMPTY — an unmatched verb ⇒
    /// no claim ⇒ the caller treats an empty footprint as "wall", silence = wall).
    Emitted(Vec<EmittedCoord>),
    /// Non-concrete / out-of-dialect-at-runtime / malformed emission — the whole footprint
    /// is discarded (all-or-nothing, TC-4). Always the safe outcome (the site walls).
    Top(TouchesTop),
}

/// Why a `touches()` trace degraded to ⊤. A closed enum so a new degrade-reason breaks
/// every exhaustive match (the compiler-as-checklist).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchesTop {
    /// The argv was empty (no command for the argparse to consume).
    EmptyArgv,
    /// A reached word resolved to no concrete value (unbound var, unmodeled expansion,
    /// `$0`, a positional past the end in strict position).
    NonConcreteWord(&'static str),
    /// A reached command was NOT `printf` — touches bodies emit coordinates only via
    /// `printf`, so any other reached command is unmodeled ⇒ the footprint is untrusted.
    NonPrintfCommand,
    /// The printf format carried an escape/directive outside the modeled set
    /// (`\n` `\t` `\\` `%s` `%%`).
    UnmodeledFormat,
    /// The `%s` placeholder count and the printf argument count disagreed (missing arg, or
    /// a leftover arg — no format-cycling is modeled; bias ⊤).
    ArgCountMismatch,
    /// An emitted line was not a well-formed `kind:entity` coordinate (no `:`, or an empty
    /// kind, or an empty non-final line).
    MalformedCoordinate,
    /// The iteration budget was exhausted (a loop did not terminate within bound).
    BudgetExceeded,
}

impl TouchesTop {
    /// A short human-readable form for diagnostics/provenance.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            TouchesTop::EmptyArgv => "empty argv",
            TouchesTop::NonConcreteWord(w) => w,
            TouchesTop::NonPrintfCommand => "touches body reached a non-printf command",
            TouchesTop::UnmodeledFormat => "printf format carried an unmodeled escape/directive",
            TouchesTop::ArgCountMismatch => "printf %s count disagreed with argument count",
            TouchesTop::MalformedCoordinate => "emitted line is not a kind:entity coordinate",
            TouchesTop::BudgetExceeded => "iteration budget exceeded",
        }
    }
}

/// Trace `touches` over `argv` — the full, concrete, verbatim argument list of the book's
/// command, **not** including the command word itself (the same contract as
/// [`crate::predict::evaluate`]). Returns a [`TouchesResolution`].
///
/// Pure + total (`inv-determinism`/`inv-no-throw`): no clock/RNG/IO, ordered collections
/// only, every path returns a resolution (the budget bounds loops).
#[must_use]
pub fn evaluate_touches(touches: &Predict, argv: &[&str]) -> TouchesResolution {
    if argv.is_empty() {
        return TouchesResolution::Top(TouchesTop::EmptyArgv);
    }
    let budget = argv.len().saturating_mul(4).saturating_add(BUDGET_CONSTANT);
    let mut ev = Emitter {
        positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
        vars: BTreeMap::new(),
        coords: Vec::new(),
        budget,
        steps: 0,
    };
    match ev.run_block(&touches.body) {
        Flow::Normal => TouchesResolution::Emitted(ev.coords),
        Flow::Top(reason) => TouchesResolution::Top(reason),
    }
}

/// Budget = `4 * argv.len() + BUDGET_CONSTANT` — mirrors the predict evaluator.
const BUDGET_CONSTANT: usize = 32;

/// The touches interpreter: the SAME argparse control-flow as the predict evaluator
/// (`while`/`case`/`shift`/assign — reusing [`resolve_word`]/[`eval_test`]/[`pattern_matches`]
/// so footprint fragments travel the exact value-flow predict does, 24A §1b fence), but its
/// Command handler COLLECTS `printf` coordinates instead of recording a probe span.
///
/// Deliberately a SEPARATE run-loop from the predict [`Evaluator`](crate::predict) rather
/// than a shared generic interpreter: the two collect fundamentally different things, and a
/// duplicated ~90-line mechanical loop keeps the load-bearing predict path untouched at the
/// cost of the duplication (flagged `tc-touches-eval-dup` — a future unification is possible
/// once both collectors are proven).
struct Emitter {
    positionals: Vec<String>,
    vars: BTreeMap<Symbol, String>,
    coords: Vec<EmittedCoord>,
    budget: usize,
    steps: usize,
}

enum Flow {
    Normal,
    Top(TouchesTop),
}

impl Emitter {
    fn tick(&mut self) -> Result<(), TouchesTop> {
        self.steps = self.steps.saturating_add(1);
        if self.steps > self.budget {
            Err(TouchesTop::BudgetExceeded)
        } else {
            Ok(())
        }
    }

    fn run_block(&mut self, body: &[Stmt]) -> Flow {
        for stmt in body {
            match self.run_stmt(stmt) {
                Flow::Normal => {}
                top @ Flow::Top(_) => return top,
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
                // A non-concrete rvalue leaves the var unbound (a later use degrades to ⊤) —
                // never bound to a bogus value. Same posture as the predict evaluator.
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
            Stmt::Command(cmd) => self.run_command(cmd),
            // An annotation in a touches body desugars to `name=value` (the strip's own
            // rule); treat it as a binding (unresolvable value ⇒ leave unbound). A bare
            // mark is a no-op. Neither emits a coordinate.
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
            return Flow::Top(TouchesTop::NonConcreteWord("shift past end of argv"));
        }
        self.positionals.drain(0..n);
        Flow::Normal
    }

    fn run_while(&mut self, test: &crate::predict::Test, body: &[Stmt]) -> Flow {
        loop {
            if let Err(reason) = self.tick() {
                return Flow::Top(reason);
            }
            match eval_test(test, &self.positionals, &self.vars) {
                Ok(true) => match self.run_block(body) {
                    Flow::Normal => {}
                    top @ Flow::Top(_) => return top,
                },
                Ok(false) => return Flow::Normal,
                Err(reason) => return Flow::Top(top_from_word(reason)),
            }
        }
    }

    fn run_case(&mut self, scrutinee: &Word, arms: &[crate::predict::CaseArm]) -> Flow {
        let value = match self.resolve(scrutinee) {
            Ok(v) => v,
            Err(reason) => return Flow::Top(top_from_word(reason)),
        };
        for arm in arms {
            if arm.patterns.iter().any(|p| pattern_matches(p, &value)) {
                return self.run_block(&arm.body); // sh: first matching arm only
            }
        }
        // No arm matched, no `*` catch-all: sh falls through with no effect (⇒ no emission
        // ⇒ no claim ⇒ the caller walls). Faithful to sh, not a degrade.
        Flow::Normal
    }

    /// A reached command: it MUST be `printf` (the sole emission verb); its format + args
    /// yield the emitted coordinate lines. Any other command is unmodeled ⇒ ⊤ (the whole
    /// footprint is untrusted — we cannot know what an unmodeled command in the footprint
    /// body would touch).
    fn run_command(&mut self, cmd: &Command) -> Flow {
        let Some((verb, rest)) = cmd.words.split_first() else {
            return Flow::Top(TouchesTop::NonPrintfCommand); // empty command (defensive)
        };
        match self.resolve(verb) {
            Ok(v) if v == "printf" => {}
            Ok(_) => return Flow::Top(TouchesTop::NonPrintfCommand),
            Err(reason) => return Flow::Top(top_from_word(reason)),
        }
        let Some((format_word, arg_words)) = rest.split_first() else {
            return Flow::Top(TouchesTop::UnmodeledFormat); // `printf` with no format
        };
        let format = match self.resolve(format_word) {
            Ok(f) => f,
            Err(reason) => return Flow::Top(top_from_word(reason)),
        };
        let mut args = Vec::with_capacity(arg_words.len());
        for w in arg_words {
            match self.resolve(w) {
                Ok(a) => args.push(a),
                Err(reason) => return Flow::Top(top_from_word(reason)),
            }
        }
        match printf_lines(&format, &args) {
            Ok(lines) => {
                for line in lines {
                    match parse_coordinate(&line) {
                        Some(coord) => self.coords.push(coord),
                        None => return Flow::Top(TouchesTop::MalformedCoordinate),
                    }
                }
                Flow::Normal
            }
            Err(top) => Flow::Top(top),
        }
    }

    /// Resolve a word in strict context (`Unresolved` on a past-end positional) — a footprint
    /// fragment must resolve concretely, exactly as a predict annotation value must.
    fn resolve(&self, word: &Word) -> Result<String, crate::predict::TopReason> {
        resolve_word(word, &self.positionals, &self.vars, UnsetPolicy::Unresolved)
    }
}

/// Map a predict word-resolution [`TopReason`](crate::predict::TopReason) into a
/// [`TouchesTop`] — a resolve failure inside a touches trace is the same non-concreteness,
/// carried under the touches degrade-enum.
fn top_from_word(reason: crate::predict::TopReason) -> TouchesTop {
    TouchesTop::NonConcreteWord(reason.as_str())
}

/// Emulate `printf FORMAT ARGS…` enough for coordinate emission — the STRAWMAN emission
/// grammar (24A §1b; a NEW parse surface, every judgment noted). Modeled: `\n` `\t` `\\`
/// escapes, `%s` (consumes the next arg), `%%`. Anything else (`%d`, `\x41`, a `%s`
/// without an arg, a leftover arg — no format-cycling) ⇒ ⊤. Returns the emitted output
/// split into LINES (the trailing newline's empty tail dropped); each line is one
/// coordinate.
fn printf_lines(format: &str, args: &[String]) -> Result<Vec<String>, TouchesTop> {
    let mut out = String::new();
    let mut arg_idx = 0usize;
    let mut chars = format.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                _ => return Err(TouchesTop::UnmodeledFormat),
            },
            '%' => match chars.next() {
                Some('s') => {
                    let Some(a) = args.get(arg_idx) else {
                        return Err(TouchesTop::ArgCountMismatch);
                    };
                    out.push_str(a);
                    arg_idx = arg_idx.saturating_add(1);
                }
                Some('%') => out.push('%'),
                _ => return Err(TouchesTop::UnmodeledFormat),
            },
            other => out.push(other),
        }
    }
    // POSIX printf reuses the format for leftover args; the strawman never cycles, so a
    // leftover arg is bias-⊤ (an authoring shape we do not model).
    if arg_idx != args.len() {
        return Err(TouchesTop::ArgCountMismatch);
    }
    // Split into lines; drop exactly the trailing empty tail after a final `\n`. An empty
    // NON-final line is malformed (caught downstream as a coordinate with no `:`).
    let mut lines: Vec<String> = out.split('\n').map(str::to_owned).collect();
    if lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    Ok(lines)
}

/// Parse one emitted line into an [`EmittedCoord`] — split on the FIRST `:` (kind ⟂ entity;
/// the entity may itself hold `:`/`.`). `None` (⊤) if there is no `:` or the kind is empty.
/// An empty entity is the kind's singleton.
fn parse_coordinate(line: &str) -> Option<EmittedCoord> {
    let (kind, entity) = line.split_once(':')?;
    if kind.is_empty() {
        return None;
    }
    Some(EmittedCoord {
        kind: kind.to_owned(),
        entity: (!entity.is_empty()).then(|| entity.to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::Interner;

    /// Lift the sole touches funcdef from `src` and trace it over `argv`.
    fn trace(src: &str, argv: &[&str]) -> TouchesResolution {
        let mut i = Interner::default();
        let set = TouchesSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one touches funcdef");
        let touches = set.value.get(provider).expect("the touches funcdef");
        evaluate_touches(touches, argv)
    }

    // Mirrors the real apt argparse: flag-strip BEFORE and AFTER the verb (so `install -y
    // nginx` resolves the operand `nginx`, not the flag `-y`).
    const APT: &str = "\
apt-get.touches() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   case $verb in
   update) printf 'pkgindex:\\n' ;;
   install) printf 'package:%s\\n' \"$1\" ;;
   esac
}";

    #[test]
    fn singleton_coordinate_from_matched_verb() {
        // `apt-get update` ⇒ the flag-strip drops `-y`? no flags here; verb=update ⇒ the arm
        // emits the singleton `pkgindex:` (empty entity).
        let r = trace(APT, &["update"]);
        assert_eq!(
            r,
            TouchesResolution::Emitted(vec![EmittedCoord {
                kind: "pkgindex".to_owned(),
                entity: None,
            }])
        );
    }

    #[test]
    fn operand_coordinate_via_percent_s() {
        // `apt-get install -y nginx`: flag-strip drops `-y`, verb=install, `$1`=nginx ⇒
        // `package:nginx` (the %s resolves the operand through the SAME value-flow predict uses).
        let r = trace(APT, &["install", "-y", "nginx"]);
        assert_eq!(
            r,
            TouchesResolution::Emitted(vec![EmittedCoord {
                kind: "package".to_owned(),
                entity: Some("nginx".to_owned()),
            }])
        );
    }

    #[test]
    fn unmatched_verb_emits_nothing_no_claim() {
        // A verb no arm matches emits nothing ⇒ Emitted(empty) ⇒ the caller walls (silence=wall).
        let r = trace(APT, &["remove", "nginx"]);
        assert_eq!(r, TouchesResolution::Emitted(vec![]));
    }

    #[test]
    fn unresolved_arg_tops_the_whole_footprint() {
        // A `%s` whose operand does not resolve (empty argv past the verb) ⇒ ⊤ (all-or-nothing).
        // `install` with no operand: `$1` after the shift is past-end ⇒ NonConcreteWord.
        let r = trace(APT, &["install"]);
        assert!(matches!(r, TouchesResolution::Top(_)), "got {r:?}");
    }

    #[test]
    fn non_printf_command_tops() {
        let src = "\
hork.touches() {
   verb=$1
   case $verb in
   tune) rm -rf / ;;
   esac
}";
        // A reached non-printf command ⇒ ⊤ (we cannot trust a footprint whose body does
        // something unmodeled).
        assert_eq!(
            trace(src, &["tune"]),
            TouchesResolution::Top(TouchesTop::NonPrintfCommand)
        );
    }

    #[test]
    fn unmodeled_directive_tops() {
        let src = "x.touches() { printf 'k:%d\\n' 5 ; }";
        assert_eq!(
            trace(src, &["anything"]),
            TouchesResolution::Top(TouchesTop::UnmodeledFormat)
        );
    }

    #[test]
    fn multi_coordinate_one_printf() {
        // One printf may emit several lines — each a coordinate.
        let src = "x.touches() { printf 'a:one\\nb:two\\n' ; }";
        assert_eq!(
            trace(src, &["v"]),
            TouchesResolution::Emitted(vec![
                EmittedCoord {
                    kind: "a".to_owned(),
                    entity: Some("one".to_owned()),
                },
                EmittedCoord {
                    kind: "b".to_owned(),
                    entity: Some("two".to_owned()),
                },
            ])
        );
    }

    #[test]
    fn entity_may_hold_dots_and_colons() {
        // Split on the FIRST `:` only — a kind like `kernel.Sysctl` and a dotted entity survive.
        let src = "x.touches() { printf 'kernel.Sysctl:net.ipv4.ip_forward\\n' ; }";
        assert_eq!(
            trace(src, &["v"]),
            TouchesResolution::Emitted(vec![EmittedCoord {
                kind: "kernel.Sysctl".to_owned(),
                entity: Some("net.ipv4.ip_forward".to_owned()),
            }])
        );
    }

    #[test]
    fn malformed_coordinate_without_colon_tops() {
        let src = "x.touches() { printf 'nocolon\\n' ; }";
        assert_eq!(
            trace(src, &["v"]),
            TouchesResolution::Top(TouchesTop::MalformedCoordinate)
        );
    }
}
