//! `carry` — the pure-predicate carry (`27C` §4(a) / `plans/27C` §9; steering
//! `pure-predicate-carry`): the ONLY UNFLAGGED cross-substrate-boundary carry. A verdict fact
//! measured in the AMBIENT context may answer a wrapped site across a substrate boundary (fs-view,
//! netns) — unflagged — iff BOTH hold:
//!
//! * **(A) authored axis-invariance** ([`InvarianceIndex`]): every marked backing kind of the fact
//!   carries its kind-owner's `undivided-by-transit-across <axis>` line inside `kind__state_stored_only_in()`
//!   (`277` §4e). Authored + attributable — a wrong line is the kind-owner's pointable error
//!   (vouch-species), never the engine's.
//! * **(B) engine-proved read-set closure** ([`read_set_closed`]): a conservative sh-taint over the
//!   verdict body proving everything influencing the verdict rc — data AND control-flow — traces to
//!   the site's argv (through the author's argparse) or a MARKED read, with NO unmarked external
//!   input. DEFAULT-DISQUALIFY: any construct off the audited [`PURE_BUILTINS`] / safe-list
//!   disqualifies. The pass reads MARKS + sh structure, NEVER tool semantics
//!   (`inv-referent-agnostic`). Fails safe (a missed-safe body loses an elision, never carries a
//!   hidden read).
//!
//! # Scope edges (`27C` §4(a), ruled)
//!
//! * **Substrate axes ONLY** (fs-view, netns). The user/identity dimension is EXCLUDED — a
//!   user-shift changes ACCESS to the body's own reads (EACCES flips a structurally-closed body's
//!   answer), so (B) cannot certify it; user rides the enter-context lane or §4(b).
//! * **netns caveat** — `net.*` state is per-netns, so [`InvarianceIndex::lift`] FORBIDS
//!   `undivided-by-transit-across netns` on a kind whose store is the `net-kernel` substrate (a loud diagnostic; the
//!   claim is not honored). fs-view has no analogue.
//! * **VERDICT-fact carry only** (human-confirmed 2026-07-17). World-cell VALUE carry rides r26 with
//!   the capture fold; nothing here transports a value.
//!
//! The carry is UNFLAGGED (`rul-flag-is-razor-residue`: the flag is for HUMAN at-most claims; (B)
//! CLOSES the open-world "and nothing else" by structure, so no at-most claim remains). It is a
//! DISTINCT, explicitly-licensed path: the cli keys a carried fact `Context::HostDefault` (measure
//! ambient), so `core::coord::compare` is UNTOUCHED — cross-context compare stays `Unknown` for
//! every OTHER consumer (`ternary-compare-consumer-map` / `pin-no-outcome-as-generator`).

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::diag::{CarryNetnsOnNetKernelForbidden, Diag, DiagCode};
use dorc_core::{Interner, SourceFileId, Span, Symbol};

use crate::predict::{Command, MarkKind, Predict, Stmt, Word};
use crate::wrapper::Dimension;

/// The substrate emission token (`272` §2) naming per-netns network kernel state — the ONE store
/// whose axis-invariance across netns an owner may not claim (the netns caveat, `27C` §4(a)). A
/// `net-kernel` store IS `net.*` sysctl state, which is per-netns namespaced.
const NET_KERNEL_SUBSTRATE: &str = "net-kernel";

/// The pure sh builtins a closed verdict body may invoke UNMARKED (`27C` §4(a)-(B) safe-list): they
/// read NO external system state, so their rc traces purely to their (clean) argv operands. Every
/// OTHER command word is an external read/effect and is admissible ONLY when MARKED (a declared read
/// — the mark names the cell, `inv-referent-agnostic`). The audit IS the artifact: this list is the
/// whole unmarked-command safe-list.
///
/// * `:` / `true` / `false` — constant-rc no-ops.
/// * `return` — sets the verdict rc from a (clean) literal (the `return 2` decline sink).
/// * `[` / `test` — a string test over (clean) argv operands (branch/verdict conditions).
/// * `printf` — emits its (clean) argv to stdout; touches no system state.
const PURE_BUILTINS: &[&str] = &[":", "true", "false", "return", "[", "test", "printf"];

// ===========================================================================
// (A) authored axis-invariance — the `undivided-by-transit-across <axis>` line index
// ===========================================================================

/// The (A)-side index (`27C` §4(a)): per marked backing kind, the substrate axes its owner declared
/// the kind's state INVARIANT across, lifted from every `kind__state_stored_only_in()` body's
/// `undivided-by-transit-across <axis>` colon-lines (`277` §4e). Keyed by the kind's MUNGED funcname segment
/// ([`crate::to_funcname_segment`]) so a dotted mark kind (`sm.dorc.KernelParam`) and the funcdef
/// name (`sm_dorc_KernelParam__state_stored_only_in`) agree.
///
/// Empty when no `state_stored_only_in()` declares an invariance line ⇒ carry never licenses
/// (`empty-world-byte-identical`, `silence-licenses-nothing`).
#[derive(Debug, Clone, Default)]
pub struct InvarianceIndex {
    /// Per kind, each invariant [`Dimension`] mapped to its `undivided-by-transit-across <axis>` line's defining
    /// `(Span, SourceFileId)` (`tc-disturbs-span-threading`'s sibling — `27V:mech-minting-line-
    /// threading`), so a carry's attribution renders the kind-owner's line as `file:line` (render 3/3;
    /// `27C` §9). The span is the mark's own command line; the file id disambiguates which oracle.
    per_kind: BTreeMap<String, BTreeMap<Dimension, (Span, SourceFileId)>>,
}

impl InvarianceIndex {
    /// Lift the (A) index from `state_stored_only_in()` bodies (`27C` §4(a); `277` §4e). Each body
    /// is a [`Predict`]-shaped funcdef; its `undivided-by-transit-across <axis>` colon-lines mark whole-member
    /// invariance. Enforces the **netns caveat**: a kind whose body emits the `net-kernel` substrate
    /// (`printf … : net-kernel`) and ALSO claims `undivided-by-transit-across netns` is `declarations-genuinely-
    /// contradict` — the netns claim is DROPPED and a loud diagnostic emitted (`net.*` is per-netns;
    /// the model must not let an owner claim it netns-invariant). Fail-soft (`inv-no-throw`),
    /// deterministic (`inv-determinism`); reads marks + structure only (`inv-referent-agnostic`).
    #[must_use]
    pub fn lift(interner: &mut Interner, srcs: &[&str]) -> (Self, Vec<Diag>) {
        let mut per_kind: BTreeMap<String, BTreeMap<Dimension, (Span, SourceFileId)>> =
            BTreeMap::new();
        let mut diags = Vec::new();
        for (idx, src) in srcs.iter().enumerate() {
            let file = SourceFileId(u32::try_from(idx).unwrap_or(u32::MAX));
            let set = crate::predict::lift_state_stored_only_in(interner, src);
            for provider in set.value.providers() {
                let Some(body) = set.value.get(provider) else {
                    continue;
                };
                let kind_munged = interner.resolve(provider).to_owned();
                let scan = scan_state_body(body);
                let entry = per_kind.entry(kind_munged.clone()).or_default();
                for (dim, span) in scan.invariant {
                    // netns caveat: net-kernel state is per-netns ⇒ an `undivided-by-transit-across netns` claim on it
                    // is a contradiction; drop it and diagnose (never honor a false invariance line).
                    if dim == Dimension::Netns && scan.stores_net_kernel {
                        diags.push(Diag::new(
                            DiagCode::CarryNetnsOnNetKernelForbidden(
                                CarryNetnsOnNetKernelForbidden {
                                    kind_munged: kind_munged.clone(),
                                },
                            ),
                            // The refused mark's own line, which the sibling arm below already
                            // carries; the funcdef header can be many lines from the claim.
                            span,
                        ));
                        continue;
                    }
                    entry.entry(dim).or_insert((span, file));
                }
            }
        }
        (Self { per_kind }, diags)
    }

    /// Does `kind` (a DOTTED mark kind, e.g. `sm.dorc.KernelParam`) carry an `undivided-by-transit-across <dim>` line
    /// (`27C` §4(a)(A))? Munges the mark kind to the funcname segment for the lookup. Absent ⇒ false
    /// (`silence-licenses-nothing`).
    #[must_use]
    pub fn invariant_across(&self, kind: &str, dim: Dimension) -> bool {
        self.per_kind
            .get(&crate::to_funcname_segment(kind))
            .is_some_and(|dims| dims.contains_key(&dim))
    }

    /// The `undivided-by-transit-across <dim>` line's defining `(Span, SourceFileId)` for `kind` (render 3/3, `27C` §9):
    /// the kind-owner's attributable line the carry attribution renders as `file:line`. `None` when
    /// the kind carries no such line (`silence-licenses-nothing`).
    #[must_use]
    pub fn invariant_span(&self, kind: &str, dim: Dimension) -> Option<(Span, SourceFileId)> {
        self.per_kind
            .get(&crate::to_funcname_segment(kind))
            .and_then(|dims| dims.get(&dim).copied())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.per_kind.values().all(BTreeMap::is_empty)
    }
}

/// What one `state_stored_only_in()` body declared (`272` §2 / `277` §4e): the substrate-invariance
/// axes and whether it emits the per-netns `net-kernel` store (for the netns caveat).
struct StateBodyScan {
    invariant: BTreeMap<Dimension, Span>,
    stores_net_kernel: bool,
}

/// Scan a `state_stored_only_in()` body for its `undivided-by-transit-across <axis>` lines + `net-kernel` substrate
/// emissions (`277` §4e). Distinguished by the typed VERB (`MarkKind::Undivided` vs
/// `MarkKind::StoredIn` with the `net-kernel` substrate token). Whole-member scope (`277` §4e);
/// control-flow arms are all scanned (over-approximation is safe for the caveat — it can only ADD a
/// forbid). Substrate axes only reach the index (`from_token` maps `fs-view`/`netns`; a stray
/// `undivided-by-transit-across user` line is dropped here — user is not a carry axis).
fn scan_state_body(body: &Predict) -> StateBodyScan {
    let mut invariant = BTreeMap::new();
    let mut stores_net_kernel = false;
    scan_state_block(&body.body, &mut invariant, &mut stores_net_kernel);
    StateBodyScan {
        invariant,
        stores_net_kernel,
    }
}

fn scan_state_block(
    body: &[Stmt],
    invariant: &mut BTreeMap<Dimension, Span>,
    stores_net_kernel: &mut bool,
) {
    for stmt in body {
        match stmt {
            Stmt::Command(cmd) => {
                let Some(mark) = &cmd.mark else { continue };
                if mark.kind == MarkKind::Undivided {
                    // The axis token lives in the uniform `kind` payload home
                    // (`28A:rul-uniform-kind-payload-home`): `undivided-by-transit-across fs-view`
                    // ⇒ kind="fs-view" (`281` §5).
                    if let Some(dim) = Dimension::from_token(&mark.target.kind)
                        && dim != Dimension::User
                    {
                        // The mark's own command-line span is the `file:line` the carry attribution
                        // points at (render 3/3). First occurrence wins (deterministic).
                        invariant.entry(dim).or_insert(cmd.span);
                    }
                } else if mark.kind == MarkKind::StoredIn
                    && mark.target.kind == NET_KERNEL_SUBSTRATE
                {
                    *stores_net_kernel = true;
                }
            }
            Stmt::Case { arms, .. } => {
                for a in arms {
                    scan_state_block(&a.body, invariant, stores_net_kernel);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                scan_state_block(then_body, invariant, stores_net_kernel);
                scan_state_block(else_body, invariant, stores_net_kernel);
            }
            Stmt::While { body, .. } => scan_state_block(body, invariant, stores_net_kernel),
            _ => {}
        }
    }
}

// ===========================================================================
// (B) engine-proved read-set closure — the conservative sh-taint pass
// ===========================================================================

/// The (B) closure outcome (`27C` §4(a)-(B)): CLOSED with the set of DOTTED read kinds the marked
/// reads named (the (A) lookup keys), or OPEN with the first disqualifying construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureOutcome {
    /// The body is read-set-closed: everything influencing the verdict rc traces to argv or a marked
    /// read. `read_kinds` are the DOTTED kinds of those marked reads (the (A) check's inputs).
    Closed { read_kinds: BTreeSet<String> },
    /// The body has an unmarked external input — carry disqualified (`Open`; fail safe ⇒ wall).
    Open(ClosureReject),
}

/// Why a verdict body is NOT read-set-closed (`27C` §4(a)-(B)) — the first disqualifying construct,
/// for the why-lens/disclosure. Every reason is an unmarked external input the pass cannot trace to
/// argv or a declared read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureReject {
    pub reason: RejectReason,
    /// The disqualifying construct's span, when the walk has one (a command; a var reference has
    /// none — words carry no span in the dialect AST).
    pub span: Option<Span>,
}

/// The disqualifying-construct class (`27C` §4(a)-(B) default-disqualify).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    /// An unmarked command that is not a [`PURE_BUILTINS`] member (`cat`, `sysctl`, `id`, …) — an
    /// external read/effect with no declared cell.
    UnmarkedExternalCommand,
    /// A [`Word::Unmodeled`] anywhere a value flows — a command substitution (`$(cat …)`), a
    /// `${x:-y}` env-default, arithmetic, process-substitution artifact: opaque external input.
    UnmodeledWord,
    /// A pipeline (`cmd | cmd`) — an external composition whose rc the pass cannot trace.
    Pipeline,
    /// An and-or list (`cmd && cmd`, `cmd || cmd`) — like a pipeline, a composition whose rc the
    /// pass cannot trace, and whose items' marks are refused, so it declares nothing either.
    AndOrList,
    /// A [`Word::Var`] reference to a variable NOT bound to a clean value earlier in the body — an
    /// ambient ENV read (`$HOME`, `$USER`).
    AmbientVar,
    /// The body has NO marked read — a degenerate argv-only tautology establishes no host fact worth
    /// carrying (conservative: carry requires ≥1 declared read).
    NoMarkedRead,
}

/// Prove a verdict body READ-SET-CLOSED (`27C` §4(a)-(B); steering `pure-predicate-carry`). Walks
/// the body DEFAULT-DISQUALIFY: a value is CLEAN iff it is argv-derived or bound to a clean value; a
/// command is admissible iff it is a [`PURE_BUILTINS`] member with clean operands OR carries a mark
/// (a declared read whose operands are clean). ANY dirty construct (unmarked external command,
/// [`Word::Unmodeled`], pipeline, ambient var, dirty branch condition) ⇒ [`ClosureOutcome::Open`].
///
/// The pass reads MARKS + sh structure, never tool semantics (`inv-referent-agnostic`). It fails
/// safe: an OPEN verdict loses its carry, never carries a hidden read (`27C` §4(a): missed-safe
/// loses an elision). Pure/total (`inv-no-throw`/`inv-determinism`).
#[must_use]
pub fn read_set_closed(body: &Predict) -> ClosureOutcome {
    let mut walk = ClosureWalk {
        clean: BTreeSet::new(),
        read_kinds: BTreeSet::new(),
    };
    // The verb-binding convention (`verb=$1`) and positional params are clean argv sources; the walk
    // seeds nothing and learns clean vars from clean assignments as it goes.
    if let Err(reject) = walk.block(&body.body) {
        return ClosureOutcome::Open(reject);
    }
    if walk.read_kinds.is_empty() {
        return ClosureOutcome::Open(ClosureReject {
            reason: RejectReason::NoMarkedRead,
            span: Some(body.name_span),
        });
    }
    ClosureOutcome::Closed {
        read_kinds: walk.read_kinds,
    }
}

/// The closure taint walk (`27C` §4(a)-(B)). `clean` grows monotonically with clean assignments; a
/// DIRTY assignment disqualifies the whole body outright (so no dirty-bound var ever exists — a
/// later `Var` reference is clean iff its name is in `clean`, dirty otherwise).
struct ClosureWalk {
    clean: BTreeSet<Symbol>,
    read_kinds: BTreeSet<String>,
}

impl ClosureWalk {
    fn block(&mut self, body: &[Stmt]) -> Result<(), ClosureReject> {
        for stmt in body {
            self.stmt(stmt)?;
        }
        Ok(())
    }

    fn stmt(&mut self, stmt: &Stmt) -> Result<(), ClosureReject> {
        match stmt {
            Stmt::Assign { name, value } => {
                self.clean_word(value, None)?;
                self.clean.insert(*name);
                Ok(())
            }
            Stmt::Annotation(a) => {
                if let Some(v) = &a.value {
                    self.clean_word(v, Some(a.span))?;
                }
                self.clean.insert(a.name);
                Ok(())
            }
            Stmt::Shift { .. } => Ok(()),
            Stmt::While { test, body } => {
                self.clean_word(&test.lhs, Some(test.span))?;
                self.clean_word(&test.rhs, Some(test.span))?;
                self.block(body)
            }
            Stmt::If {
                test,
                then_body,
                else_body,
            } => {
                self.clean_word(&test.lhs, Some(test.span))?;
                self.clean_word(&test.rhs, Some(test.span))?;
                self.block(then_body)?;
                self.block(else_body)
            }
            Stmt::Case { scrutinee, arms } => {
                self.clean_word(scrutinee, None)?;
                for arm in arms {
                    self.block(&arm.body)?;
                }
                Ok(())
            }
            Stmt::Command(cmd) => self.command(cmd),
            Stmt::AndOr(list) => Err(reject(RejectReason::AndOrList, Some(list.span))),
        }
    }

    /// A command is admissible iff it is a pure builtin with clean operands, OR a MARKED read whose
    /// operands (and value tail) are clean. An unmarked external command, or a pipeline, disqualifies.
    fn command(&mut self, cmd: &Command) -> Result<(), ClosureReject> {
        if cmd.pipeline {
            return Err(reject(RejectReason::Pipeline, Some(cmd.span)));
        }
        for w in &cmd.words {
            self.clean_word(w, Some(cmd.span))?;
        }
        if let Some(mark) = &cmd.mark {
            // (The old verdict `= value` cleanliness read DROPPED here — seam in `ast::MarkTarget`.)
            // A verdict/observe mark declares the read — record its (dotted) kind for the (A) check.
            if matches!(
                mark.kind,
                MarkKind::Asserts | MarkKind::Refutes | MarkKind::Reads
            ) {
                self.read_kinds.insert(mark.target.kind.clone());
            }
            return Ok(());
        }
        // Unmarked: admissible ONLY as a pure builtin (a real tool read hides an undeclared cell).
        match cmd.words.first() {
            Some(Word::Literal(w) | Word::SingleQuotedLiteral(w))
                if PURE_BUILTINS.contains(&w.as_str()) =>
            {
                Ok(())
            }
            _ => Err(reject(
                RejectReason::UnmarkedExternalCommand,
                Some(cmd.span),
            )),
        }
    }

    /// A word is CLEAN iff it is argv-derived (positional forms, `"$@"`), a literal, or a `Var` bound
    /// clean earlier. A [`Word::Unmodeled`] (cmdsub/`${x:-y}`/arith) or an ambient (unbound) `Var`
    /// is DIRTY (`span` names the enclosing construct for the reject).
    fn clean_word(&self, w: &Word, span: Option<Span>) -> Result<(), ClosureReject> {
        match w {
            Word::Literal(_)
            | Word::SingleQuotedLiteral(_)
            | Word::Positional(_)
            | Word::PositionalArgs
            | Word::PositionalStripPrefix { .. }
            | Word::PositionalDefault { .. } => Ok(()),
            Word::Var(sym) if self.clean.contains(sym) => Ok(()),
            Word::Var(_) => Err(reject(RejectReason::AmbientVar, span)),
            Word::Unmodeled(_) => Err(reject(RejectReason::UnmodeledWord, span)),
        }
    }
}

fn reject(reason: RejectReason, span: Option<Span>) -> ClosureReject {
    ClosureReject { reason, span }
}

// ===========================================================================
// The combined carry decision (A) ∧ (B) ∧ substrate-scope
// ===========================================================================

/// The pure-predicate carry decision for a wrapped site (`27C` §4(a)). CARRY iff EVERY crossed
/// dimension is a substrate axis AND (B) the verdict body is read-set-closed AND (A) every read kind
/// is invariant across every crossed dimension; else `NoCarry` with the first blocking reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarryDecision {
    /// Carry: the ambient measurement answers the wrapped site across the substrate boundary,
    /// unflagged. The cli keys the fact `Context::HostDefault` (measure ambient). `read_kinds` are
    /// the marked backing kinds whose owners' `undivided-by-transit-across <axis>` lines licensed the crossing — the
    /// (A) half of the attribution chain the carry note renders (`carried-across-substrate-axis`).
    Carry { read_kinds: BTreeSet<String> },
    /// No carry — the reason drives the wall (and the why-lens), never a permissive default.
    NoCarry(CarryReject),
}

/// Why a wrapped site cannot pure-predicate-carry (`27C` §4(a)) — the first blocking reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarryReject {
    /// A crossed dimension that is NOT a substrate axis — the user/identity dimension (EACCES
    /// access-flip; excluded, `27C` §4(a)). Carry serves fs-view/netns only.
    NonSubstrateCrossing(Dimension),
    /// (B) failed — the verdict body is not read-set-closed.
    BodyNotClosed(ClosureReject),
    /// (A) failed — a marked backing kind carries no `undivided-by-transit-across <dim>` line for a crossed dimension.
    KindNotInvariant { kind: String, dim: Dimension },
}

/// Decide pure-predicate carry (`27C` §4(a); steering `pure-predicate-carry`). `crossed` are the
/// substrate/identity dimensions the wrapper chain shifts; `closure` is (B) over the inner verdict
/// body; `inv` is the (A) index. Order: substrate-scope, then (B), then (A) per crossed dimension ×
/// read kind. A never-derive-separation-clean decision (declared line (A) + engine-proof (B), never
/// derivation-as-license).
#[must_use]
pub fn decide_carry(
    crossed: &[Dimension],
    closure: &ClosureOutcome,
    inv: &InvarianceIndex,
) -> CarryDecision {
    // Substrate scope: any crossed identity dimension excludes carry (a user-shift changes ACCESS).
    for &d in crossed {
        if d == Dimension::User {
            return CarryDecision::NoCarry(CarryReject::NonSubstrateCrossing(d));
        }
    }
    // (B): the body must be read-set-closed.
    let read_kinds = match closure {
        ClosureOutcome::Closed { read_kinds } => read_kinds,
        ClosureOutcome::Open(reject) => {
            return CarryDecision::NoCarry(CarryReject::BodyNotClosed(reject.clone()));
        }
    };
    // (A): every read kind invariant across every crossed dimension (universal meet — any gap walls).
    for &d in crossed {
        for kind in read_kinds {
            if !inv.invariant_across(kind, d) {
                return CarryDecision::NoCarry(CarryReject::KindNotInvariant {
                    kind: kind.clone(),
                    dim: d,
                });
            }
        }
    }
    CarryDecision::Carry {
        read_kinds: read_kinds.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verdict::VerdictSet;

    /// Lift the sole verdict funcdef in `src` to its [`Predict`] body (the (B)-pass input).
    fn verdict_body(src: &str) -> Predict {
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean verdict lift: {:?}", set.diags);
        let provider = set.value.providers().next().expect("one verdict funcdef");
        set.value.get(provider).expect("the verdict body").clone()
    }

    /// Build the (A) index from `srcs`, returning the index + its diagnostics.
    fn invariance(srcs: &[&str]) -> (InvarianceIndex, Vec<Diag>) {
        let mut i = Interner::default();
        InvarianceIndex::lift(&mut i, srcs)
    }

    // ── (B) read-set closure: the safe-list ─────────────────────────────────────────────────────

    /// The closable shape (`27C` §9 battery row 1): the marked-command realization of the hermetic
    /// argument-driven predicate (`[ "$(sysctl -n "$1")" = "$2" ]`) — the tool does the compare, its
    /// rc is the verdict, the read is DECLARED by the mark, operands are argv. CLOSED, read-kind
    /// recorded. (The cmdsub-comparison spelling itself needs cmdsub-VALUE modeling; see the
    /// straddler test — this is the spike-dialect closable idiom.)
    #[test]
    fn closure_marked_command_argv_only_is_closed() {
        let body = verdict_body(
            "sysctlcheck__is_converged() { hork --check \"$1\" \"$2\" : sm.dorc.KernelParam:\"$1\" ; }",
        );
        assert_eq!(
            read_set_closed(&body),
            ClosureOutcome::Closed {
                read_kinds: ["sm.dorc.KernelParam".to_owned()].into_iter().collect()
            }
        );
    }

    /// A case-dispatched verdict (verb argparse + `return 2` decline) with a marked read per arm is
    /// CLOSED — `case`/`return`/argv scrutinee are all safe-list, the read is declared.
    #[test]
    fn closure_case_dispatch_with_marked_read_is_closed() {
        let body = verdict_body(
            "kp__is_converged() { verb=\"$1\" ; case \"$verb\" in check) hork -c \"$2\" \"$3\" : sm.dorc.KernelParam:\"$2\" ;; *) return 2 ;; esac }",
        );
        assert!(matches!(
            read_set_closed(&body),
            ClosureOutcome::Closed { .. }
        ));
    }

    /// The straddler (`27C` §9 battery row 2): an unmarked `$(cat …)` command-substitution captured
    /// into the compare value — an ambient fs-view-DEPENDENT read the mark does not declare. The
    /// cmdsub lifts to [`Word::Unmodeled`] ⇒ DISQUALIFY (the exact 27Xb `policyctl` counterexample).
    #[test]
    fn closure_straddler_cmdsub_walls() {
        let body = verdict_body(
            "policyctl__is_converged() { want=\"$(cat /etc/policy)\" ; hork --check \"$1\" \"$want\" : sm.dorc.KernelParam:\"$1\" ; }",
        );
        let ClosureOutcome::Open(reject) = read_set_closed(&body) else {
            panic!("straddler must be OPEN: {:?}", read_set_closed(&body));
        };
        assert_eq!(reject.reason, RejectReason::UnmodeledWord);
    }

    /// An unmarked external command (`27C` §9 battery row 4 opaque-call) — a real tool read with no
    /// declared cell — DISQUALIFIES.
    #[test]
    fn closure_unmarked_external_command_walls() {
        let body =
            verdict_body("c__is_converged() { cat /etc/x ; hork -c \"$1\" : sm.dorc.K:\"$1\" ; }");
        let ClosureOutcome::Open(reject) = read_set_closed(&body) else {
            panic!("unmarked read must be OPEN");
        };
        assert_eq!(reject.reason, RejectReason::UnmarkedExternalCommand);
    }

    /// An ambient env read (`27C` §9 battery row 4 env-read): a `Var` not bound to a clean value
    /// earlier — `$HOME`/`$USER`-class — DISQUALIFIES.
    #[test]
    fn closure_ambient_env_var_walls() {
        let body = verdict_body(
            "c__is_converged() { want=\"$HOME\" ; hork -c \"$want\" : sm.dorc.K:\"$1\" ; }",
        );
        let ClosureOutcome::Open(reject) = read_set_closed(&body) else {
            panic!("ambient var must be OPEN");
        };
        assert_eq!(reject.reason, RejectReason::AmbientVar);
    }

    /// A pipeline stage DISQUALIFIES (an external composition; also covers the clock/process-subst
    /// classes whose producers are pipelines or cmdsubs).
    #[test]
    fn closure_pipeline_walls() {
        let body = verdict_body(
            "c__is_converged() { hork -n \"$1\" | grep -q \"$2\" : sm.dorc.K:\"$1\" ; }",
        );
        let ClosureOutcome::Open(reject) = read_set_closed(&body) else {
            panic!("pipeline must be OPEN");
        };
        assert_eq!(reject.reason, RejectReason::Pipeline);
    }

    /// A branch condition reading a varying store (`27C` §9 battery row 5): an `if` whose test reads
    /// an ambient var gates the verdict on external input ⇒ DISQUALIFY (control-flow is tainted too).
    #[test]
    fn closure_dirty_branch_condition_walls() {
        let body = verdict_body(
            "c__is_converged() { if [ \"$XDG\" = live ] ; then hork -c \"$1\" : sm.dorc.K:\"$1\" ; fi }",
        );
        let ClosureOutcome::Open(reject) = read_set_closed(&body) else {
            panic!("dirty branch condition must be OPEN");
        };
        assert_eq!(reject.reason, RejectReason::AmbientVar);
    }

    /// A body with NO marked read is degenerate (an argv-only tautology establishes no host fact) ⇒
    /// conservatively NOT carriable (`RejectReason::NoMarkedRead`).
    #[test]
    fn closure_no_marked_read_walls() {
        let body = verdict_body("c__is_converged() { true ; }");
        let ClosureOutcome::Open(reject) = read_set_closed(&body) else {
            panic!("no-marked-read must be OPEN");
        };
        assert_eq!(reject.reason, RejectReason::NoMarkedRead);
    }

    // ── (A) invariance index + the netns caveat ─────────────────────────────────────────────────

    /// The `undivided-by-transit-across fs-view` line lifts to per-kind invariance (`277` §4e), keyed so a dotted mark
    /// kind resolves it.
    #[test]
    fn invariance_index_lifts_fsview_line() {
        let (inv, diags) = invariance(&[
            "sm_dorc_KernelParam__state_stored_only_in() { printf 'sys\\n' : stored-in kernel ; : undivided-by-transit-across fs-view ; }",
        ]);
        assert!(diags.is_empty(), "clean lift: {diags:?}");
        assert!(inv.invariant_across("sm.dorc.KernelParam", Dimension::FsView));
        assert!(!inv.invariant_across("sm.dorc.KernelParam", Dimension::Netns));
        assert!(!inv.invariant_across("sm.dorc.Other", Dimension::FsView));
    }

    /// The netns caveat (`27C` §4(a)): a `net-kernel` store claiming `undivided-by-transit-across netns` is a
    /// contradiction — the claim is DROPPED and a loud diagnostic emitted (`net.*` is per-netns).
    #[test]
    fn invariance_netns_on_net_kernel_is_dropped_and_diagnosed() {
        let (inv, diags) = invariance(&[
            "sm_dorc_Fw__state_stored_only_in() { printf 'nft\\n' : stored-in net-kernel ; : undivided-by-transit-across netns ; }",
        ]);
        assert!(
            !inv.invariant_across("sm.dorc.Fw", Dimension::Netns),
            "netns invariance on net-kernel state must NOT be honored"
        );
        assert_eq!(diags.len(), 1, "the caveat is diagnosed loudly");
        assert_eq!(diags[0].code.slug(), "carry-netns-on-net-kernel-forbidden");
        // The caret sits on the refused mark, not on the funcdef header many lines above it.
        let source = "sm_dorc_Fw__state_stored_only_in() { printf 'nft\\n' : stored-in net-kernel ; : undivided-by-transit-across netns ; }";
        let at = diags[0].primary.span().expect("a spanned refusal").lo.0 as usize;
        assert!(
            source[at..].starts_with(": undivided-by-transit-across netns"),
            "the caret points at {:?}",
            &source[at..]
        );
    }

    /// A NON-net `kernel` store CAN be netns-invariant (`vm.swappiness`-class) — the caveat is
    /// specific to the per-netns `net-kernel` substrate.
    #[test]
    fn invariance_netns_on_plain_kernel_is_honored() {
        let (inv, diags) = invariance(&[
            "sm_dorc_Vm__state_stored_only_in() { printf 'sys\\n' : stored-in kernel ; : undivided-by-transit-across netns ; }",
        ]);
        assert!(diags.is_empty());
        assert!(inv.invariant_across("sm.dorc.Vm", Dimension::Netns));
    }

    /// The empty-oracle world: no `state_stored_only_in()` ⇒ empty index ⇒ carry never licenses
    /// (`empty-world-byte-identical`, `silence-licenses-nothing`).
    #[test]
    fn invariance_empty_world_licenses_nothing() {
        let (inv, diags) =
            invariance(&["hork__is_converged() { hork -c \"$1\" : sm.dorc.K:\"$1\" ; }"]);
        assert!(diags.is_empty());
        assert!(inv.is_empty());
        assert!(!inv.invariant_across("sm.dorc.K", Dimension::FsView));
    }

    // ── the combined carry decision ─────────────────────────────────────────────────────────────

    /// (A) ∧ (B) hold across fs-view ⇒ CARRY (`27C` §9 battery row 1 end-to-end).
    #[test]
    fn decide_carries_closed_invariant_across_fsview() {
        let body = verdict_body(
            "kp__is_converged() { hork -c \"$1\" \"$2\" : sm.dorc.KernelParam:\"$1\" ; }",
        );
        let (inv, _) = invariance(&[
            "sm_dorc_KernelParam__state_stored_only_in() { printf 's\\n' : stored-in kernel ; : undivided-by-transit-across fs-view ; }",
        ]);
        assert!(matches!(
            decide_carry(&[Dimension::FsView], &read_set_closed(&body), &inv),
            CarryDecision::Carry { .. }
        ));
    }

    /// (`27C` §9 battery row 3): a marked read of a NON-invariant kind walls — (B) holds but (A)
    /// fails (no `undivided-by-transit-across fs-view` line for the read's kind).
    #[test]
    fn decide_walls_marked_read_of_non_invariant_kind() {
        let body =
            verdict_body("kp__is_converged() { hork -c \"$1\" : sm.dorc.KernelParam:\"$1\" ; }");
        let (inv, _) = invariance(&[
            "sm_dorc_KernelParam__state_stored_only_in() { printf 's\\n' : stored-in kernel ; }", // no invariance line
        ]);
        assert_eq!(
            decide_carry(&[Dimension::FsView], &read_set_closed(&body), &inv),
            CarryDecision::NoCarry(CarryReject::KindNotInvariant {
                kind: "sm.dorc.KernelParam".to_owned(),
                dim: Dimension::FsView
            })
        );
    }

    /// The user dimension is EXCLUDED (`27C` §4(a)): a crossed identity dimension walls carry even
    /// with a closed invariant body (a user-shift changes ACCESS — EACCES flips a closed body).
    #[test]
    fn decide_walls_user_dimension_crossing() {
        let body =
            verdict_body("kp__is_converged() { hork -c \"$1\" : sm.dorc.KernelParam:\"$1\" ; }");
        let (inv, _) = invariance(&[
            "sm_dorc_KernelParam__state_stored_only_in() { printf 's\\n' : stored-in kernel ; : undivided-by-transit-across fs-view ; }",
        ]);
        assert_eq!(
            decide_carry(&[Dimension::User], &read_set_closed(&body), &inv),
            CarryDecision::NoCarry(CarryReject::NonSubstrateCrossing(Dimension::User))
        );
    }

    /// (`27C` §9 battery row 6): a `net.*` fact does NOT carry across netns — the caveat drops the
    /// netns invariance, so (A) fails.
    #[test]
    fn decide_walls_net_kernel_across_netns() {
        let body = verdict_body("fw__is_converged() { hork -c \"$1\" : sm.dorc.Fw:\"$1\" ; }");
        let (inv, _) = invariance(&[
            "sm_dorc_Fw__state_stored_only_in() { printf 'nft\\n' : stored-in net-kernel ; : undivided-by-transit-across netns ; }",
        ]);
        assert!(matches!(
            decide_carry(&[Dimension::Netns], &read_set_closed(&body), &inv),
            CarryDecision::NoCarry(CarryReject::KindNotInvariant {
                dim: Dimension::Netns,
                ..
            })
        ));
    }

    /// A non-net kernel fact DOES carry across netns (positive netns; the caveat is net-kernel only).
    #[test]
    fn decide_carries_plain_kernel_across_netns() {
        let body = verdict_body("vm__is_converged() { hork -c \"$1\" : sm.dorc.Vm:\"$1\" ; }");
        let (inv, _) = invariance(&[
            "sm_dorc_Vm__state_stored_only_in() { printf 's\\n' : stored-in kernel ; : undivided-by-transit-across netns ; }",
        ]);
        assert!(matches!(
            decide_carry(&[Dimension::Netns], &read_set_closed(&body), &inv),
            CarryDecision::Carry { .. }
        ));
    }

    /// A straddling body walls carry via (B) even when (A) would hold for its declared read.
    #[test]
    fn decide_walls_straddler_via_closure() {
        let body = verdict_body(
            "kp__is_converged() { want=\"$(cat /etc/policy)\" ; hork -c \"$1\" \"$want\" : sm.dorc.KernelParam:\"$1\" ; }",
        );
        let (inv, _) = invariance(&[
            "sm_dorc_KernelParam__state_stored_only_in() { printf 's\\n' : stored-in kernel ; : undivided-by-transit-across fs-view ; }",
        ]);
        assert!(matches!(
            decide_carry(&[Dimension::FsView], &read_set_closed(&body), &inv),
            CarryDecision::NoCarry(CarryReject::BodyNotClosed(_))
        ));
    }
}
