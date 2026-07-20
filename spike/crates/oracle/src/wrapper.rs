//! `wrapper` — the peeling-wrapper model (`273`; `271:rul-predict-absorbs-wrapper-modeling` +
//! `rul-lend-map` + `rul-env-claim-inversion`).
//!
//! Wrapper-ness is DETECTED, never declared (`273` §1, typed): a `<provider>__predict` body whose
//! command-position `"$@"` runs its argument-slot is a peeling wrapper by tautology. This module
//! reads three things off the authored surface, NEVER by running it (`static-lift-only`):
//!
//! 1. **peel detection + the ρ-claim** ([`detect_peel`]) — the command whose guest (`"$@"`) is in
//!    executing position, and the env-idiom on its head (the ρ-ladder, `271:rul-env-claim-inversion`).
//! 2. **`cmd__lend_map()`** ([`LendMap`] / [`derive_lend_map`]) — the wrapper's dimension member:
//!    one entry per dimension, an ABSENT dimension ⊤ (the enumerate-every-dimension law).
//! 3. **dual-peel coherence** ([`check_peel_coherence`]) — where a wrapper authors BOTH members,
//!    their `"$@"` must reach the same tail position, else static incoherence ⇒ fail-fast (`273` §5).
//!
//! # Scope fence (`27J` §2.1, this lane MODELS only)
//!
//! This lane builds the model and detection; it mints NO elision or probe license (the entry /
//! dial / vouch machinery is `lane-context-entry`, next). Nothing here is consumed by
//! `analysis`/`plan` yet — a wrapped book site still walls opaquely (`silence-licenses-nothing`),
//! so the wrapper-free corpus stays byte-stable (`empty-world-byte-identical`). The one seat left
//! open for the next lane is the inner node's CONTEXT ([`InnerContext`]); its threading into
//! `core::FactKey` is a cross-cutting decision flagged `tc-*` (`27J` §4).
//!
//! `inv-referent-agnostic`: the walk reads the body's own STRUCTURE (the command head, the
//! dimension token on a mark) — never what an operand MEANS.

use std::collections::BTreeMap;

use dorc_core::diag::{Diag, DiagCode, LendMapUnknownDimension};
use dorc_core::{Carrier, Interner, Symbol};

use crate::predict::{
    Command, Predict, PredictSet, Stmt, Word, eval_test, lift_lend_maps, pattern_matches,
    resolve_word,
};
use dorc_syntax::sem::UnsetPolicy;

/// An engine-owned **dimension** — a closed, version-scoped set (`273` §3; `27C` §0: user ·
/// fs-view · netns). A `cmd__lend_map()` answers one entry per dimension; an ABSENT dimension is
/// ⊤ (walls) — the enumerate-every-dimension law (`271:rul-lend-map`; absent-key-means-full-lend
/// is REJECTED). The ρ dimension is NOT here: it rides the predict body's env-idioms
/// ([`RhoClaim`]), never a `lend_map` mark (`273` §10 "environment belongs to ρ forever").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dimension {
    /// The acting user/identity (`sudo -u`, `su`). `: user`.
    User,
    /// The filesystem view (`chroot`, mount namespace). `: fs-view` — kept DISTINCT from the
    /// `fs` substrate token (`273` §8 token-collision).
    FsView,
    /// The network namespace (`ip netns exec`). `: netns` (`271:rul-networking-unpunt`).
    Netns,
}

impl Dimension {
    /// Every dimension the dialect knows, in a stable order (`inv-determinism`). The
    /// enumerate-every-dimension consumer iterates this to find the ⊤ (absent) entries.
    pub const ALL: [Dimension; 3] = [Dimension::User, Dimension::FsView, Dimension::Netns];

    /// The engine-owned mark token for this dimension (`273` §8; the closed vocabulary).
    #[must_use]
    pub fn as_token(self) -> &'static str {
        match self {
            Dimension::User => "user",
            Dimension::FsView => "fs-view",
            Dimension::Netns => "netns",
        }
    }

    /// The dimension a `lend_map` mark token names, or `None` if the token is not a known dimension
    /// (an unknown token on a `lend_map` line is out-of-vocabulary — the consumer rejects it loudly).
    #[must_use]
    pub fn from_token(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|d| d.as_token() == s)
    }
}

/// How a peel's env-head transforms the environment the guest is born into — the ρ-claim ladder
/// (`271:rul-env-claim-inversion`; `274` §12 riders r1–r6 bind). Every rung is a runnable sh
/// idiom read off the peel command's head; silence (bare `"$@"`) is the floor, NEVER "claims
/// isolation" (derived separation is barred).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RhoClaim {
    /// bare `"$@"` — claims NOTHING ⇒ ⊤ (the guest's environment is unknown to us; never read as
    /// isolation). An identity wrapper that wants env value-flow must upgrade to `env "$@"`.
    Nothing,
    /// `env "$@"` (optionally `env VAR=v … "$@"`) — full ambient passthrough (the `env` syllable
    /// IS the claim). Per-variable overrides on top of ambient are a refinement recorded in
    /// `overrides` (empty for bare `env "$@"`).
    FullAmbient { overrides: Vec<String> },
    /// `env -i [VAR=v …] "$@"` — exactly-these: the guest sees ONLY the enumerated variables (a
    /// scrubbed base). `env -` reads as `-i` (r2). `vars` are the `VAR` names (values are runtime
    /// argv, resolved next lane).
    ExactlyThese { vars: Vec<String> },
    /// `VAR=x "$@"` — the bare PREFIX-ASSIGNMENT rung (`271:rul-env-claim-inversion` "per-variable
    /// claim, rest ⊤"; the `27K` disclosed gap, modeled here). The named vars are positively claimed;
    /// everything else about the environment is ⊤ (UNCLAIMED — distinct from `env "$@"`, whose `env`
    /// syllable claims the full ambient passthrough). `vars` are the `VAR` names.
    ///
    /// # Churn-avoidance disclosure (`ru-26`; the `27K` gap)
    ///
    /// The predict parser splits a leading `VAR=x` into a SCRIPT-scoped [`Stmt::Assign`], NOT a
    /// command-scoped prefix, so `VAR=x "$@"` parses as `[Assign, Command(bare "$@")]` — the same
    /// shape as a genuine two-statement `V=$1; "$@"`. We recognize the rung only in the UNAMBIGUOUS
    /// top-level case (a body whose statements before the bare-`"$@"` peel are ALL assignments — no
    /// argparse `shift`/`case`), where the assigns can only be env-prefix overrides. A body that also
    /// argparses (`verb=$1; shift; "$@"`) stays [`Nothing`](RhoClaim::Nothing) (the safe floor — the
    /// assigns are argparse, not env claims). The precise prefix-vs-statement distinction needs a
    /// parser fold (deferred by `27K` for founding-pin churn risk); this models the clean idiom.
    ///
    /// `vars` are the interned assign NAMEs (referent-agnostic identities; the value-flow lane that
    /// consumes ρ resolves them for display — this module holds no interner).
    PerVariable { vars: Vec<Symbol> },
}

/// The head that puts a peel's guest (`"$@"`) in executing position — the ONLY two modeled
/// transparent executors (`273` §1 tautology + the `env`-syllable recognition, `274` §12 r1):
/// bare `"$@"` (the guest is argv[0]) and a static command-word `env` (env execs its trailing
/// operand). Any other head with a trailing `"$@"` is NOT a peel (the guest is that command's
/// argument, not its command — the site walls, `silence-licenses-nothing`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PeelHead {
    /// bare `"$@"` — the guest is the command itself (identity-wrapper idiom).
    Bare,
    /// `env … "$@"` — env is the transparent executor; the ρ rides its argv.
    Env,
}

/// A detected peel: the ρ-claim the guest is born into, plus the peel command's index within its
/// containing block (for provenance / the tail-position coherence check). Detection is by tautology
/// (`273` §1) — a body with such a command IS a peeling wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peel {
    /// The ρ-claim read off the peel command's env-head (`271:rul-env-claim-inversion`).
    pub rho: RhoClaim,
}

/// Detect whether `check` (a `<provider>__predict` body) is a peeling wrapper, and if so read its
/// ρ-claim (`273` §1). Returns `None` for an ordinary (non-peeling) predict body — the vast
/// majority. A pre-order walk finds the FIRST reachable peel command (the flag-strip converges to
/// one `"$@"`; a body with several picks the first, deterministically). Pure/total
/// (`inv-no-throw`/`inv-determinism`).
#[must_use]
pub fn detect_peel(check: &Predict) -> Option<Peel> {
    let cmd = first_peel_command(&check.body)?;
    // The `VAR=x "$@"` prefix-assignment rung (`27K` gap): a bare-`"$@"` peel whose top-level body is
    // ALL assignments before the peel is a per-variable env claim (unambiguous — no argparse). Any
    // other body keeps the head-derived ρ (bare ⇒ Nothing).
    if matches!(peel_head(cmd), Some(PeelHead::Bare))
        && let Some(vars) = top_level_prefix_assignment_vars(&check.body)
    {
        return Some(Peel {
            rho: RhoClaim::PerVariable { vars },
        });
    }
    Some(Peel { rho: rho_of(cmd) })
}

/// The `VAR=x "$@"` prefix-assignment ρ recognizer (`27K` gap): `Some(vars)` iff `body`'s top level
/// is a run of [`Stmt::Assign`] immediately followed by a bare-`"$@"` [`Stmt::Command`], with NOTHING
/// else before the peel (no `shift`/`case`/`if` argparse — those would make the assigns argparse
/// bindings, not env overrides). Returns the assigned NAME symbols. `None` for anything ambiguous
/// (or no leading assigns) — the safe floor.
fn top_level_prefix_assignment_vars(body: &[Stmt]) -> Option<Vec<Symbol>> {
    let mut vars = Vec::new();
    for stmt in body {
        match stmt {
            Stmt::Assign { name, .. } => vars.push(*name),
            Stmt::Command(c) if matches!(peel_head(c), Some(PeelHead::Bare)) => {
                // Reached the bare peel with only assigns before it ⇒ per-variable claim (if any).
                return (!vars.is_empty()).then_some(vars);
            }
            // Any non-assign, non-bare-peel statement ⇒ ambiguous / argparse ⇒ not this rung.
            _ => return None,
        }
    }
    None
}

/// The first reachable command whose guest (`"$@"`) is in executing position, walking control flow
/// in source order (a peel in a `case`/`if`/`while` arm counts). `None` ⇒ not a peeling wrapper.
fn first_peel_command(body: &[Stmt]) -> Option<&Command> {
    for stmt in body {
        match stmt {
            Stmt::Command(c) if peel_head(c).is_some() => return Some(c),
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    if let Some(c) = first_peel_command(&arm.body) {
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
                    first_peel_command(then_body).or_else(|| first_peel_command(else_body))
                {
                    return Some(c);
                }
            }
            Stmt::While { body, .. } => {
                if let Some(c) = first_peel_command(body) {
                    return Some(c);
                }
            }
            _ => {}
        }
    }
    None
}

/// The [`PeelHead`] of a command whose LAST word is the guest `"$@"` ([`Word::PositionalArgs`]),
/// or `None` if this command does not put its guest in executing position. Bare `"$@"` (head IS
/// the guest) and `env … "$@"` (env execs the guest) are the only modeled transparent executors.
fn peel_head(cmd: &Command) -> Option<PeelHead> {
    // A pipeline never peels (its guest flows through the pipe, not exec position); `inv-top-reject`.
    if cmd.pipeline {
        return None;
    }
    match cmd.words.as_slice() {
        // bare `"$@"` — the guest is argv[0].
        [Word::PositionalArgs] => Some(PeelHead::Bare),
        // `env … "$@"` — a STATIC command-word `env` (r1: a path-qualified `/usr/bin/env` or a
        // dynamic head is UNRECOGNIZED ⇒ not this peel ⇒ walls). The guest is env's last operand.
        [Word::Literal(head), .., Word::PositionalArgs] if head == "env" => Some(PeelHead::Env),
        _ => None,
    }
}

/// Read the ρ-claim off a peel command (`271:rul-env-claim-inversion`; `274` §12 r1–r6). A bare
/// `"$@"` claims nothing; an `env`-headed peel parses env's argv into the ladder rung.
fn rho_of(cmd: &Command) -> RhoClaim {
    match peel_head(cmd) {
        Some(PeelHead::Bare) | None => RhoClaim::Nothing,
        Some(PeelHead::Env) => rho_of_env(&cmd.words),
    }
}

/// Parse `env`'s argv (between the `env` head and the trailing `"$@"`) into a ρ rung. The
/// recognized grammar is SYNTACTIC and enumerated (`274` §12 r1): assignments (`VAR=v`) and `-i`
/// (and `env -` = `-i`, r2). Any UNRECOGNIZED flag (`-u`, `-S`/`-C`/`-P`, signal flags) ⇒
/// claims-nothing (safe + hint, r1/r6): we still peel (env execs the guest) but assert no ρ.
fn rho_of_env(words: &[Word]) -> RhoClaim {
    // words = [env, <args…>, "$@"]; scan the middle args.
    let mid = &words[1..words.len().saturating_sub(1)];
    let mut scrubbed = false;
    let mut assignments: Vec<String> = Vec::new();
    for w in mid {
        let Word::Literal(tok) = w else {
            // A dynamic env arg (`env "$flag" "$@"`) is unrecognized ⇒ claims-nothing (r1/r6).
            return RhoClaim::Nothing;
        };
        if tok == "-i" || tok == "-" {
            // `env -i` scrub-base; `env -` is `-i` in every implementation (r2 — POSIX-unspecified,
            // never read as passthrough).
            scrubbed = true;
        } else if let Some((name, _val)) = split_var_assignment(tok) {
            assignments.push(name.to_owned());
        } else {
            // An unrecognized flag (`-u FOO`, `-S`, path stuff) ⇒ claims-nothing, safe + hint (r1).
            return RhoClaim::Nothing;
        }
    }
    if scrubbed {
        RhoClaim::ExactlyThese { vars: assignments }
    } else {
        RhoClaim::FullAmbient {
            overrides: assignments,
        }
    }
}

/// A `VAR=value` env-assignment token split into `(name, value)`, or `None` if it is not a valid
/// assignment (the name must be a sh NAME). Used to read `env`'s assignment args.
fn split_var_assignment(tok: &str) -> Option<(&str, &str)> {
    let (name, value) = tok.split_once('=')?;
    dorc_syntax::sem::is_name(name).then_some((name, value))
}

/// One dimension's lend, per `271:rul-lend-map` (`273` §3–§4). Empty result for a PRESENT key =
/// full lend; contents = mapped lend; a MISSING key = ⊤ (walls).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LendEntry {
    /// Full lend (the colon-line `:   : user`): the guest borrows the caller's world wholesale on
    /// this dimension (live referent-sameness). The most-likely-wrong-by-thoughtlessness entry
    /// (`273` §4 — the cargo-cult default), so it is spelled explicitly, never a default.
    Full,
    /// Mapped lend (`printf … "$target" : user`): the guest's dimension is the caller's THROUGH the
    /// map — license-free re-indexing (`272` §3). The mapped VALUE is ρ-resolved argv per-site
    /// (an unresolvable → ⊤ value); this lane records only that it IS mapped.
    Mapped,
    /// ⊤ — a MISSING key: the dimension is not answered. Walls; a hint-tier nudge fires. This is
    /// the enumerate-every-dimension law's default (`271:rul-lend-map`).
    Top,
}

/// A wrapper's lifted `cmd__lend_map()` — the per-dimension entries it answers plus whether its
/// body reaches the peel boundary (`"$@"`). Absent dimensions are ⊤ ([`LendMap::lend`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LendMap {
    entries: BTreeMap<Dimension, LendEntry>,
    /// Whether the body reaches a terminal `"$@"` — the peel boundary (`273` §3). A `lend_map` with
    /// no peel boundary is malformed (it does not enumerate its guest); recorded for the coherence
    /// check.
    pub peels: bool,
}

impl LendMap {
    /// This dimension's lend — the enumerate-every-dimension law (`271:rul-lend-map`): a dimension
    /// the body did NOT answer is ⊤ (walls), NOT a full lend.
    #[must_use]
    pub fn lend(&self, dim: Dimension) -> LendEntry {
        self.entries.get(&dim).copied().unwrap_or(LendEntry::Top)
    }

    /// The dimensions this `lend_map` did NOT answer (each ⊤ ⇒ walls; each is a hint-tier nudge
    /// site). Iterates [`Dimension::ALL`] so a newly-minted dialect dimension automatically reads
    /// ⊤ against an old member (`273` §3 version story).
    #[must_use]
    pub fn missing_dimensions(&self) -> Vec<Dimension> {
        Dimension::ALL
            .into_iter()
            .filter(|d| !self.entries.contains_key(d))
            .collect()
    }
}

/// Derive the [`LendMap`] from a lifted `<provider>__lend_map` body (`271:rul-lend-map`; `273`
/// §3). Each dimension-marked command line contributes one entry: a bare `:` colon-line = full
/// lend, a producing command (e.g. `printf`) = mapped lend; a terminal `"$@"` sets the peel
/// boundary. An unknown mark token on a `lend_map` line is out-of-vocabulary — reported as a loud
/// diagnostic (`inv-top-reject`), never silently accepted. Pure/total.
#[must_use]
pub fn derive_lend_map(check: &Predict) -> (LendMap, Vec<Diag>) {
    let mut map = LendMap::default();
    let mut diags = Vec::new();
    walk_lend_body(&check.body, &mut map, &mut diags);
    (map, diags)
}

fn walk_lend_body(body: &[Stmt], map: &mut LendMap, diags: &mut Vec<Diag>) {
    for stmt in body {
        match stmt {
            Stmt::Command(c) => {
                if peel_head(c).is_some() {
                    map.peels = true;
                }
                let Some(mark) = &c.mark else { continue };
                // A dimension entry: the mark's `kind` fragment is the dimension token (`: user`).
                let token = &mark.target.kind;
                let Some(dim) = Dimension::from_token(token) else {
                    diags.push(Diag::new(
                        DiagCode::LendMapUnknownDimension(LendMapUnknownDimension {
                            token: token.clone(),
                            expected: Dimension::ALL.map(Dimension::as_token).join(", "),
                        }),
                        mark.span,
                    ));
                    continue;
                };
                // A bare `:` colon-line = full lend; any producing command (printf …) = mapped
                // lend. `inv-referent-agnostic`: keyed on the command's own head, never the value.
                let entry = if is_colon_line(c) {
                    LendEntry::Full
                } else {
                    LendEntry::Mapped
                };
                // First entry per dimension wins (a duplicate is dropped — the safe direction).
                map.entries.entry(dim).or_insert(entry);
            }
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    walk_lend_body(&arm.body, map, diags);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                walk_lend_body(then_body, map, diags);
                walk_lend_body(else_body, map, diags);
            }
            Stmt::While { body, .. } => walk_lend_body(body, map, diags),
            _ => {}
        }
    }
}

/// Is this command the sh no-op `:` (a colon-line — the full-lend spelling)? Its single word is
/// the literal `:` (`273` §3: "sh's nothing-command, marked; strips to a harmless bare `:`").
fn is_colon_line(cmd: &Command) -> bool {
    matches!(cmd.words.as_slice(), [Word::Literal(w)] if w == ":")
}

/// Lift every `<provider>__lend_map` in `src` into a [`PredictSet`] (COMMAND-keyed like the
/// wrapper's predict). The consumer calls [`derive_lend_map`] per body. Same fail-soft contract
/// as `lift_predicts`.
#[must_use]
pub fn lift_lend_map_set(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_lend_maps(interner, src)
}

/// The inner node's CONTEXT — what a wrapper chain denotes for the site it wraps (`273` §1
/// wrapper/inner split; `27C` §0 "the fact is born in the site's context"). A wrapped book site
/// `W_1 W_2 … cmd args` splits into wrapper-node(s) + an inner node whose facts are keyed in THIS
/// context (the `core::coord` context slot, `Context::HostDefault` unminted at HEAD, begins to be
/// populated here).
///
/// # Where `FactKey` stands (`27J` §4 — flagged `tc-context-slot-on-coord-not-factkey`)
///
/// This descriptor is the DESIGN of the population, computed from the wrapper chain's lends. It is
/// NOT yet threaded into [`dorc_core::FactKey`]: doing so touches `FactKey` and its ~47-site map so
/// two same-cell facts in different contexts do not collide, a cross-cutting decision `27J` §4
/// routes to `lane-context-entry`. Until then every fact stays `HostDefault` and keying is
/// unaffected (`empty-world-byte-identical`). `compare` already answers `Unknown` on a context
/// mismatch (`core::coord`), so the survival/transport consumers are ready for a populated slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InnerContext {
    /// Every crossed dimension is a FULL lend (or the chain shifts no dimension — an identity
    /// wrapper): the inner node sits in the SAME world as the caller (`Context::HostDefault` maps
    /// here). Identity wrappers (nice/nohup) are peel-transparent — their inner is `HostDefault`.
    HostDefault,
    /// At least one dimension is SHIFTED (a mapped lend, or a ⊤ that walls): the inner node is in a
    /// distinct, wrapper-denoted world. Carries the per-dimension shift so the next lane can key
    /// the fact and (with an entry form) probe in-context. A ⊤ dimension is a WALL (the site can't
    /// be answered on that dimension) — recorded so the next lane degrades to guard/run.
    Shifted {
        shifts: BTreeMap<Dimension, LendEntry>,
    },
}

/// Compute the inner node's [`InnerContext`] from a wrapper's `lend_map` (`273` §1/§4). Full lends
/// (and unshifted dimensions) keep the inner in `HostDefault`; a mapped or ⊤ (missing) dimension
/// shifts it. This is the "design the population" — the computation that WOULD key an inner fact's
/// context slot; the `FactKey` threading is `lane-context-entry`'s (`tc-*`, above).
#[must_use]
pub fn inner_context(lend: &LendMap) -> InnerContext {
    let mut shifts = BTreeMap::new();
    for dim in Dimension::ALL {
        match lend.lend(dim) {
            // Full lend on this dimension ⇒ same world (no shift contributed).
            LendEntry::Full => {}
            // Mapped ⇒ a distinct world; ⊤ (missing) ⇒ a wall on this dimension. Both shift.
            e @ (LendEntry::Mapped | LendEntry::Top) => {
                shifts.insert(dim, e);
            }
        }
    }
    if shifts.is_empty() {
        InnerContext::HostDefault
    } else {
        InnerContext::Shifted { shifts }
    }
}

/// Whether a wrapper's predict and `lend_map` peels are COHERENT over `argv` (`273` §5): both must
/// reach their guest (`"$@"`) after consuming the SAME number of leading argv tokens (the guest
/// starts at the same position). Disagreement is genuine static incoherence — the
/// declarations-genuinely-contradict category ⇒ plan-time fail-fast (`rul-proven-mutation-fails-
/// fast` posture). `None` ⇒ coherent (or one member does not peel — declining adds no license).
/// `Some(Incoherence)` ⇒ they disagree.
///
/// This is the per-invocation shape (`273` §5: "given both members answer the same
/// book-invocation"). At oracle-load the caller runs it over a canonical argv; the per-site check
/// over real book argvs is `lane-context-entry`'s refinement.
#[must_use]
pub fn check_peel_coherence(
    predict: &Predict,
    lend_map: &Predict,
    argv: &[&str],
) -> Option<Incoherence> {
    let p = peel_tail_depth(&predict.body, argv)?;
    let l = peel_tail_depth(&lend_map.body, argv)?;
    (p != l).then_some(Incoherence {
        predict_depth: p,
        lend_map_depth: l,
    })
}

/// A dual-peel coherence failure (`273` §5): the two members' `"$@"` reach different tail
/// positions for one argv — the guest would start at a different token depending on which member
/// dispatched. `declarations-genuinely-contradict` ⇒ loud, pre-network fail-fast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Incoherence {
    /// How many leading argv tokens the predict argparse consumed before `"$@"`.
    pub predict_depth: usize,
    /// How many the `lend_map` argparse consumed — different from `predict_depth`.
    pub lend_map_depth: usize,
}

/// The number of leading argv tokens a body consumes before reaching its peel command (its guest's
/// start index), or `None` if the body does not peel over this argv. Runs the argparse (the same
/// `while`/`case`/`shift`/`if` control flow the predict evaluator traces, reusing its primitives —
/// the 24A §1b vocabulary fence), tracking how many positionals `shift` consumed. Pure/total: the
/// budget bounds loops (`inv-no-throw`).
fn peel_tail_depth(body: &[Stmt], argv: &[&str]) -> Option<usize> {
    let mut tr = PeelTracer {
        positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
        vars: BTreeMap::new(),
        consumed: 0,
        budget: argv.len().saturating_mul(4).saturating_add(32),
        steps: 0,
    };
    match tr.run(body) {
        PeelFlow::Reached => Some(tr.consumed),
        PeelFlow::FellThrough | PeelFlow::Top => None,
    }
}

/// The argparse tracer for [`peel_tail_depth`] — records positionals CONSUMED (by `shift`) before
/// the peel command. A separate loop from the predict evaluator (the tc-touches-eval-dup precedent:
/// the collectors differ), reusing the shared word primitives.
struct PeelTracer {
    positionals: Vec<String>,
    vars: BTreeMap<Symbol, String>,
    consumed: usize,
    budget: usize,
    steps: usize,
}

enum PeelFlow {
    /// Reached the peel command (`"$@"` in executing position).
    Reached,
    /// Fell through the block without reaching a peel.
    FellThrough,
    /// Degraded (budget / non-concrete test) — treated as "does not peel here" (the safe floor).
    Top,
}

impl PeelTracer {
    fn tick(&mut self) -> Result<(), ()> {
        self.steps = self.steps.saturating_add(1);
        if self.steps > self.budget {
            Err(())
        } else {
            Ok(())
        }
    }

    fn run(&mut self, body: &[Stmt]) -> PeelFlow {
        for stmt in body {
            if self.tick().is_err() {
                return PeelFlow::Top;
            }
            match stmt {
                Stmt::Command(c) if peel_head(c).is_some() => return PeelFlow::Reached,
                Stmt::Command(_) | Stmt::Annotation(_) => {}
                Stmt::Assign { name, value } => {
                    if let Ok(v) = resolve_word(
                        value,
                        &self.positionals,
                        &self.vars,
                        UnsetPolicy::Unresolved,
                    ) {
                        self.vars.insert(*name, v);
                    }
                }
                Stmt::Shift { count } => {
                    let n = count.unwrap_or(1) as usize;
                    if n > self.positionals.len() {
                        return PeelFlow::Top;
                    }
                    self.positionals.drain(0..n);
                    self.consumed = self.consumed.saturating_add(n);
                }
                Stmt::While { test, body } => {
                    let mut guard = self.budget;
                    loop {
                        guard = guard.saturating_sub(1);
                        if guard == 0 {
                            return PeelFlow::Top;
                        }
                        match eval_test(test, &self.positionals, &self.vars) {
                            Ok(true) => match self.run(body) {
                                PeelFlow::FellThrough => {}
                                other => return other,
                            },
                            Ok(false) => break,
                            Err(_) => return PeelFlow::Top,
                        }
                    }
                }
                Stmt::If {
                    test,
                    then_body,
                    else_body,
                } => match eval_test(test, &self.positionals, &self.vars) {
                    Ok(true) => match self.run(then_body) {
                        PeelFlow::FellThrough => {}
                        other => return other,
                    },
                    Ok(false) => match self.run(else_body) {
                        PeelFlow::FellThrough => {}
                        other => return other,
                    },
                    Err(_) => return PeelFlow::Top,
                },
                Stmt::Case { scrutinee, arms } => {
                    let Ok(value) = resolve_word(
                        scrutinee,
                        &self.positionals,
                        &self.vars,
                        UnsetPolicy::Unresolved,
                    ) else {
                        return PeelFlow::Top;
                    };
                    for arm in arms {
                        if arm.patterns.iter().any(|p| pattern_matches(p, &value)) {
                            match self.run(&arm.body) {
                                PeelFlow::FellThrough => {}
                                other => return other,
                            }
                            break; // sh: first matching arm only
                        }
                    }
                }
            }
        }
        PeelFlow::FellThrough
    }
}

// ===========================================================================
// Book-side peel execution (lane-integration `27N`): the concrete-argv primitives the
// book pipeline needs to split a wrapped site into (context, inner-site). MODELS-only above;
// these are the seams `analysis`/`plan` consume through the cli edge.
// ===========================================================================

/// How many leading argv tokens a wrapper's `predict` consumes before its guest `"$@"` for a
/// concrete site argv (`27C` §3) — i.e. where the INNER command begins (`argv[peel_consumed..]`).
/// `None` if the body does not peel this argv (not a wrapper here, or the argparse degraded ⊤ —
/// the safe wall: a site whose wrapper cannot be peeled walls as an opaque command, unchanged law).
/// The public face of [`peel_tail_depth`] for the book pipeline; pure/total.
#[must_use]
pub fn peel_consumed(predict: &Predict, argv: &[&str]) -> Option<usize> {
    peel_tail_depth(&predict.body, argv)
}

/// Resolve each MAPPED dimension's value for a concrete site argv (`27C` §3: "the mapped VALUE is
/// ρ-resolved argv per-site; an unresolvable → ⊤ value"). Runs the `lend_map` argparse (the same
/// `shift`/`case`/`while`/`if` control flow the peel tracer walks, reusing the shared word
/// primitives) and, at each reached mapped-dimension mark line, resolves the producing command's
/// value. One entry per mapped dimension the selected path reached: `Some(value)` when it resolved
/// concretely, `None` (⊤ value ⇒ walls) otherwise. Full-lend and absent dimensions carry no value
/// (the [`LendMap`] classifies them) and are absent here.
///
/// Value model (`ru-26` scope-cut): the strawman `printf FMT VAL… : dim` idiom maps `dim` to the
/// space-joined resolved VAL operands (argv after the format string). Any other producing shape is
/// a value ⊤ (safe wall) — a richer producing surface is a later refinement. Pure/total.
#[must_use]
pub fn resolve_lend_values(
    lend_map: &Predict,
    argv: &[&str],
) -> BTreeMap<Dimension, Option<String>> {
    let mut r = LendResolver {
        positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
        vars: BTreeMap::new(),
        values: BTreeMap::new(),
        budget: argv.len().saturating_mul(4).saturating_add(32),
        steps: 0,
    };
    r.run(&lend_map.body);
    r.values
}

/// The argparse walker for [`resolve_lend_values`] — records each reached mapped-dimension line's
/// resolved value. Mirrors [`PeelTracer`]'s control-flow walk (the collectors differ: this one runs
/// PAST the argparse to the mark lines, resolving values). Budget-bounded (`inv-no-throw`).
struct LendResolver {
    positionals: Vec<String>,
    vars: BTreeMap<Symbol, String>,
    values: BTreeMap<Dimension, Option<String>>,
    budget: usize,
    steps: usize,
}

impl LendResolver {
    fn run(&mut self, body: &[Stmt]) {
        for stmt in body {
            self.steps = self.steps.saturating_add(1);
            if self.steps > self.budget {
                return;
            }
            match stmt {
                Stmt::Assign { name, value } => {
                    if let Ok(v) = resolve_word(
                        value,
                        &self.positionals,
                        &self.vars,
                        UnsetPolicy::Unresolved,
                    ) {
                        self.vars.insert(*name, v);
                    }
                }
                Stmt::Shift { count } => {
                    let n = count.unwrap_or(1) as usize;
                    if n <= self.positionals.len() {
                        self.positionals.drain(0..n);
                    }
                }
                Stmt::Command(c) => self.record_mapped(c),
                Stmt::While { test, body } => {
                    let mut guard = self.budget;
                    while matches!(eval_test(test, &self.positionals, &self.vars), Ok(true)) {
                        guard = guard.saturating_sub(1);
                        if guard == 0 {
                            return;
                        }
                        self.run(body);
                    }
                }
                Stmt::If {
                    test,
                    then_body,
                    else_body,
                } => match eval_test(test, &self.positionals, &self.vars) {
                    Ok(true) => self.run(then_body),
                    Ok(false) => self.run(else_body),
                    Err(_) => {}
                },
                Stmt::Case { scrutinee, arms } => {
                    if let Ok(value) = resolve_word(
                        scrutinee,
                        &self.positionals,
                        &self.vars,
                        UnsetPolicy::Unresolved,
                    ) {
                        for arm in arms {
                            if arm.patterns.iter().any(|p| pattern_matches(p, &value)) {
                                self.run(&arm.body);
                                break;
                            }
                        }
                    }
                }
                Stmt::Annotation(_) => {}
            }
        }
    }

    /// Record a mapped-dimension line's resolved value (`printf FMT VAL… : dim`). A colon-line
    /// (full lend) carries no value; an unknown-token or valueless line is skipped.
    fn record_mapped(&mut self, cmd: &Command) {
        let Some(mark) = &cmd.mark else { return };
        let Some(dim) = Dimension::from_token(&mark.target.kind) else {
            return;
        };
        if is_colon_line(cmd) {
            return; // full lend — no value
        }
        self.values.insert(dim, self.mapped_value_of(cmd));
    }

    /// The mapped value of a producing command — the strawman `printf FMT VAL… : dim` idiom's
    /// resolved operands after the format string, space-joined; `None` (⊤) for any other shape.
    fn mapped_value_of(&self, cmd: &Command) -> Option<String> {
        let [Word::Literal(head), _fmt, rest @ ..] = cmd.words.as_slice() else {
            return None;
        };
        if head != "printf" || rest.is_empty() {
            return None;
        }
        let mut parts = Vec::with_capacity(rest.len());
        for w in rest {
            parts.push(
                resolve_word(w, &self.positionals, &self.vars, UnsetPolicy::Unresolved).ok()?,
            );
        }
        Some(parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predict::{lift_predicts, lift_verdicts_converged};

    /// Lift the sole predict/`lend_map`/verdict funcdef from `src`, by role.
    fn one_predict(src: &str) -> (Interner, Predict) {
        let mut i = Interner::default();
        let set = lift_predicts(&mut i, src);
        assert!(set.diags.is_empty(), "clean predict lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        let check = set.value.get(p).expect("the predict").clone();
        (i, check)
    }
    fn one_lend_map(src: &str) -> (Interner, Predict) {
        let mut i = Interner::default();
        let set = lift_lend_map_set(&mut i, src);
        assert!(set.diags.is_empty(), "clean lend_map lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        let check = set.value.get(p).expect("the lend_map").clone();
        (i, check)
    }
    fn one_verdict(src: &str) -> (Interner, Predict) {
        let mut i = Interner::default();
        let set = lift_verdicts_converged(&mut i, src);
        assert!(set.diags.is_empty(), "clean verdict lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one provider");
        let check = set.value.get(p).expect("the verdict").clone();
        (i, check)
    }

    // ── peel detection + the ρ-ladder ───────────────────────────────────────────

    #[test]
    fn identity_wrapper_bare_at_claims_nothing() {
        // `nice__predict() { "$@"; }` — a bare-`"$@"` peel (`273` §8 identity wrappers). It peels
        // (transparent) but claims NOTHING on env (`271:rul-env-claim-inversion`: bare `"$@"` is ⊤,
        // never "isolation").
        let (_i, check) = one_predict("nice__predict() { \"$@\"; }");
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::Nothing
            })
        );
    }

    #[test]
    fn env_at_claims_full_ambient() {
        // `env "$@"` — the one-syllable ρ-claim identity wrappers owe for env value-flow (`273` §8;
        // the `env` syllable IS the claim).
        let (_i, check) = one_predict("someenv__predict() { env \"$@\"; }");
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::FullAmbient { overrides: vec![] }
            })
        );
    }

    #[test]
    fn env_dash_i_exactly_these() {
        // `env -i VAR=v … "$@"` — exactly-these (a scrubbed base). The sudo flagship's ρ shape.
        let (_i, check) = one_predict("sudo__predict() { env -i TERM=x HOME=/root \"$@\"; }");
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::ExactlyThese {
                    vars: vec!["TERM".to_owned(), "HOME".to_owned()]
                }
            })
        );
    }

    #[test]
    fn env_dash_is_dash_i() {
        // `env - "$@"` reads as `env -i "$@"` in every implementation (`274` §12 r2 — never as
        // passthrough).
        let (_i, check) = one_predict("x__predict() { env - \"$@\"; }");
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::ExactlyThese { vars: vec![] }
            })
        );
    }

    #[test]
    fn env_unrecognized_flag_claims_nothing() {
        // `env -u FOO "$@"` — `-u` is UNRECOGNIZED (`274` §12 r1) ⇒ claims-nothing (safe + hint):
        // it still peels (env execs the guest) but asserts no ρ.
        let (_i, check) = one_predict("x__predict() { env -u FOO \"$@\"; }");
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::Nothing
            })
        );
    }

    #[test]
    fn prefix_assignment_is_per_variable_rho() {
        // `VAR=x "$@"` — the bare prefix-assignment rung (`27K` gap). Parses as `[Assign, bare "$@"]`;
        // recognized as a per-variable claim (rest ⊤), distinct from `env "$@"` (full ambient).
        let (_i, check) = one_predict("w__predict() { LC_ALL=C \"$@\"; }");
        let Some(Peel {
            rho: RhoClaim::PerVariable { vars },
        }) = detect_peel(&check)
        else {
            panic!("VAR=x \"$@\" is a per-variable ρ claim");
        };
        assert_eq!(vars.len(), 1, "one prefix var (LC_ALL)");
    }

    #[test]
    fn argparse_before_bare_peel_stays_nothing() {
        // `verb=$1; shift; "$@"` — the assigns are ARGPARSE, not env overrides; the `shift` makes it
        // unambiguous ⇒ the bare peel claims NOTHING (the safe floor, not a false per-variable claim).
        let (_i, check) = one_predict("w__predict() { verb=$1; shift; \"$@\"; }");
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::Nothing
            }),
            "argparse before a bare peel ⇒ Nothing, never a false env claim"
        );
    }

    #[test]
    fn ordinary_predict_is_not_a_peel() {
        // A normal tool predict (the vast majority) does NOT peel: no command-position `"$@"`.
        let (_i, check) = one_predict(
            "apt_get__predict() { verb=$1; shift; pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }",
        );
        assert_eq!(detect_peel(&check), None);
    }

    #[test]
    fn trailing_at_in_argument_position_is_not_a_peel() {
        // `grep "$@"` — `"$@"` is grep's ARGUMENT, not its command; grep does not exec it, so this
        // is NOT a peel (the site walls). Only bare `"$@"` and `env … "$@"` are transparent execs.
        let (_i, check) = one_predict("x__predict() { grep \"$@\"; }");
        assert_eq!(detect_peel(&check), None);
    }

    #[test]
    fn peel_after_flag_strip_is_detected() {
        // The realistic sudo shape: a flag-strip loop, then the env-peel. The peel in the reached
        // path is found.
        let (_i, check) = one_predict(
            "sudo__predict() { while [ \"${1#-}\" != \"$1\" ]; do shift; done; env -i HOME=/root \"$@\"; }",
        );
        assert_eq!(
            detect_peel(&check),
            Some(Peel {
                rho: RhoClaim::ExactlyThese {
                    vars: vec!["HOME".to_owned()]
                }
            })
        );
    }

    // ── cmd__lend_map + the enumerate-every-dimension law ───────────────────────

    #[test]
    fn lend_map_full_and_missing() {
        // `sudo__lend_map` answers `user` (mapped) and `fs-view` (full colon-line); it does NOT
        // answer `netns` ⇒ that dimension is ⊤ (walls) — the enumerate-every-dimension law
        // (`271:rul-lend-map`: absent-key-means-full-lend is REJECTED).
        let (_i, check) = one_lend_map(
            "sudo__lend_map() { printf '%s\\n' root : lends user; : lends fs-view; \"$@\"; }",
        );
        let (map, diags) = derive_lend_map(&check);
        assert!(diags.is_empty(), "clean derive: {diags:?}");
        assert_eq!(map.lend(Dimension::User), LendEntry::Mapped);
        assert_eq!(map.lend(Dimension::FsView), LendEntry::Full);
        assert_eq!(
            map.lend(Dimension::Netns),
            LendEntry::Top,
            "an absent dimension is ⊤ (walls), NOT full lend"
        );
        assert_eq!(map.missing_dimensions(), vec![Dimension::Netns]);
        assert!(map.peels, "the terminal `\"$@\"` sets the peel boundary");
    }

    #[test]
    fn identity_lend_map_all_full() {
        // `nice__lend_map` colon-lines every dimension = full lend everywhere (identity wrapper:
        // same world) ⇒ the inner sits in HostDefault.
        let (_i, check) = one_lend_map(
            "nice__lend_map() { : lends user; : lends fs-view; : lends netns; \"$@\"; }",
        );
        let (map, diags) = derive_lend_map(&check);
        assert!(diags.is_empty());
        assert!(
            map.missing_dimensions().is_empty(),
            "all three dimensions answered"
        );
        assert_eq!(inner_context(&map), InnerContext::HostDefault);
    }

    #[test]
    fn lend_map_unknown_dimension_is_loud() {
        // A mark token that is not a known dimension (`: universe`) is out-of-vocabulary — a LOUD
        // diagnostic (`inv-top-reject`), and the line mints no lend.
        let (_i, check) = one_lend_map("x__lend_map() { : lends universe; \"$@\"; }");
        let (map, diags) = derive_lend_map(&check);
        assert_eq!(diags.len(), 1, "one loud diagnostic: {diags:?}");
        assert_eq!(diags[0].code.slug(), "lend-map-unknown-dimension");
        assert!(
            map.missing_dimensions().len() == 3,
            "the unknown token minted no dimension"
        );
    }

    #[test]
    fn inner_context_shifts_on_mapped_or_missing() {
        // A mapped `user` (distinct world) OR a ⊤ `netns` (wall) shifts the inner out of
        // HostDefault (`273` §1 wrapper/inner split).
        let (_i, check) = one_lend_map(
            "sudo__lend_map() { printf '%s\\n' root : lends user; : lends fs-view; \"$@\"; }",
        );
        let (map, _d) = derive_lend_map(&check);
        let InnerContext::Shifted { shifts } = inner_context(&map) else {
            panic!("a mapped/⊤ dimension must shift the inner context");
        };
        assert_eq!(shifts.get(&Dimension::User), Some(&LendEntry::Mapped));
        assert_eq!(shifts.get(&Dimension::Netns), Some(&LendEntry::Top));
        assert!(
            !shifts.contains_key(&Dimension::FsView),
            "full lend does not shift"
        );
    }

    // ── dual-peel coherence ─────────────────────────────────────────────────────

    #[test]
    fn coherent_peels_agree_on_tail_position() {
        // predict and lend_map both consume the verb before `"$@"` ⇒ coherent (same tail depth).
        let (_ip, predict) = one_predict("w__predict() { verb=$1; shift; env \"$@\"; }");
        let (_il, lend) = one_lend_map("w__lend_map() { verb=$1; shift; : lends user; \"$@\"; }");
        assert_eq!(
            check_peel_coherence(&predict, &lend, &["install", "nginx"]),
            None,
            "both reach `\"$@\"` after consuming 1 token ⇒ coherent"
        );
    }

    #[test]
    fn incoherent_peels_fail_fast() {
        // predict shifts the verb (guest at depth 1) but lend_map does NOT (guest at depth 0):
        // the guest would start at a different token ⇒ static incoherence (`273` §5). The
        // declarations-genuinely-contradict category.
        let (_ip, predict) = one_predict("w__predict() { verb=$1; shift; env \"$@\"; }");
        let (_il, lend) = one_lend_map("w__lend_map() { : lends user; \"$@\"; }");
        assert_eq!(
            check_peel_coherence(&predict, &lend, &["install", "nginx"]),
            Some(Incoherence {
                predict_depth: 1,
                lend_map_depth: 0
            })
        );
    }

    #[test]
    fn a_non_peeling_member_is_coherent_by_declining() {
        // A member that does not peel over this argv adds no license and cannot contradict
        // (`273` §5: declining is coherent). `None`.
        let (_ip, predict) = one_predict("w__predict() { verb=$1; shift; env \"$@\"; }");
        let (_il, lend) = one_lend_map("w__lend_map() { : lends user; }"); // no `"$@"` boundary
        assert_eq!(check_peel_coherence(&predict, &lend, &["x"]), None);
    }

    #[test]
    fn founding_verdict_body_is_not_a_peel() {
        // The founding one-liner `mycmd__is_converged() { mycmd --dry-run "$@" ;}` is NOT a peel:
        // `"$@"` is `mycmd`'s ARGUMENT (mycmd is the head), not command-position. Guards the
        // positional-model transition against a false peel-detection.
        let (_i, verdict) = one_verdict("mycmd__is_converged() { mycmd --dry-run \"$@\" ;}");
        assert_eq!(detect_peel(&verdict), None);
    }

    // ── book-side peel execution (lane-integration `27N`) ────────────────────────

    #[test]
    fn peel_consumed_counts_flag_strip_before_the_guest() {
        // `sudo__predict` flag-strips `-*` then execs the guest — `sudo -u bob pipx …` consumes
        // the two `-u bob` tokens; the inner command begins at `pipx`. A no-flag `sudo pipx …`
        // consumes nothing (the peel starts at argv[0]).
        let (_i, p) = one_predict(
            "sudo__predict() { while [ \"${1#-}\" != \"$1\" ]; do shift; done; env -i \"$@\" ; }",
        );
        assert_eq!(peel_consumed(&p, &["pipx", "install", "poddle"]), Some(0));
        // A leading flag IS stripped by the `${1#-}` loop (one shift), so `-x` consumes one token.
        assert_eq!(peel_consumed(&p, &["-x", "pipx", "install"]), Some(1));
    }

    #[test]
    fn peel_consumed_none_when_not_a_peel() {
        // A non-peeling verdict body (the guest is an argument, not command-position) does not peel.
        let (_i, v) = one_predict("dpkg__predict() { dpkg -s \"$@\" : d.Pkg = \"$1\" ; }");
        assert_eq!(peel_consumed(&v, &["nginx"]), None);
    }

    #[test]
    fn resolve_lend_values_reads_the_mapped_user_target() {
        // The babby-sudo lend_map: default `target=root`, remapped by `-u`. `resolve_lend_values`
        // resolves `user` to the printf-mapped value; `fs-view` is a colon-line (full lend, no
        // value ⇒ absent here). A `-u alice` site maps `user` to `alice` (distinct worlds key apart).
        let (_i, lm) = one_lend_map(
            "sudo__lend_map() { target=root; while [ \"${1#-}\" != \"$1\" ]; do \
             case \"$1\" in -u) target=\"$2\"; shift 2 ;; *) shift ;; esac; done; \
             printf '%s\\n' \"$target\" : lends user\n: lends fs-view\n\"$@\" ; }",
        );
        let vals = resolve_lend_values(&lm, &["pipx", "install", "poddle"]);
        assert_eq!(vals.get(&Dimension::User), Some(&Some("root".to_owned())));
        assert_eq!(vals.get(&Dimension::FsView), None); // full-lend colon-line: no value
        let alice = resolve_lend_values(&lm, &["-u", "alice", "pipx", "install"]);
        assert_eq!(alice.get(&Dimension::User), Some(&Some("alice".to_owned())));
    }
}
