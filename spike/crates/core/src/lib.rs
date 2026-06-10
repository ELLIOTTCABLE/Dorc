//! `dorc-core` — the shared vocabulary every other spike crate agrees on *first*.
//!
//! Research chord `dac-B` (error/provenance synthesis, plans/111): the analyzer
//! and the error/diagnostic layer MUST agree the graph and result types *before*
//! either is built, or they grow two incompatible graphs. This crate is that
//! agreement.
//!
//! Two invariants are load-bearing and enforced here:
//!
//! * **Determinism.** No clock, RNG, filesystem, or network — directly or
//!   transitively. The analyzer kernel is a pure function of its inputs, which is
//!   what lets the whole pipeline run inside deterministic-simulation tests
//!   without dependency-injection ceremony. Keep it that way.
//! * **No-throw stages (`dn-7`).** Every pipeline stage yields a [`Carrier<T>`] —
//!   a *result paired with accumulated diagnostics* — and never panics on
//!   malformed input. Errors are data, not control flow.
//!
//! Identifiers are newtypes, never bare integers (`make illegal states
//! unrepresentable`): you cannot pass an [`AstId`] where the type wants a fact
//! token, and the compiler enforces it.

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; these crate-root expects
// ratchet away during the rebuild (an unfulfilled `expect` warns, so they
// self-remove as the seeded layer is replaced). They never relax the policy for
// new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::indexing_slicing,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use std::collections::HashMap;

// ===========================================================================
// Identifiers
// ===========================================================================

/// Index of a node in the parsed AST arena (crate `dorc-syntax`).
///
/// Other id spaces (CFG nodes, executable leaves, facts, kinds, providers) are
/// added to this crate as the phases that need them begin — demand-driven, like
/// the parser itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AstId(pub u32);

// ===========================================================================
// Source positions
// ===========================================================================

/// A byte offset into a single source script. Byte- (not char-) indexed: the
/// lexer works over bytes, and POSIX sh is effectively byte-oriented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(pub u32);

/// A half-open `[lo, hi)` byte range in one source script.
///
/// Kept as a compact pair (research chord `ch-handle`): the hot analysis path
/// carries spans, never source text; text is resolved lazily for reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Span {
    #[must_use]
    pub fn new(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    /// The covering span of `self` and `other` (smallest range containing both).
    #[must_use]
    pub fn to(self, other: Span) -> Span {
        Span {
            lo: BytePos(self.lo.0.min(other.lo.0)),
            hi: BytePos(self.hi.0.max(other.hi.0)),
        }
    }
}

// ===========================================================================
// Diagnostics + the no-throw Carrier (dn-7)
// ===========================================================================

/// Severity of a [`Diagnostic`]. `Error` does not abort the pipeline (stages
/// never throw); it marks that the carried result is best-effort / degraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

/// A stable, greppable diagnostic code (research chord `ch-catalog`: messages
/// live in a catalog keyed by code, decoupled from the emitting logic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagCode(pub &'static str);

/// One diagnostic. Provenance-bearing: it points back at the source span that
/// triggered it. (The richer N-tier locator DAG — `ch-locator-list` — is a later
/// phase; a single optional span suffices while there are no real hosts.)
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagCode,
    pub span: Option<Span>,
    pub message: String,
}

impl Diagnostic {
    #[must_use]
    pub fn error(code: DiagCode, span: Option<Span>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            span,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn warning(code: DiagCode, span: Option<Span>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            span,
            message: message.into(),
        }
    }
}

/// `result × accumulated diagnostics` — the type every pipeline stage returns
/// (research chord `dn-7` / `ch-carrier`). A writer-monad shape: `map` transforms
/// the value, `and_then` sequences a stage while concatenating its diagnostics.
/// Stages never throw; malformed input yields a degraded `value` plus `Error`
/// diagnostics, so downstream stages still run and surface *unrelated* problems.
#[derive(Debug, Clone)]
pub struct Carrier<T> {
    pub value: T,
    pub diags: Vec<Diagnostic>,
}

impl<T> Carrier<T> {
    /// A clean result with no diagnostics.
    #[must_use]
    pub fn pure(value: T) -> Self {
        Self {
            value,
            diags: Vec::new(),
        }
    }

    #[must_use]
    pub fn new(value: T, diags: Vec<Diagnostic>) -> Self {
        Self { value, diags }
    }

    /// Transform the carried value, preserving diagnostics.
    #[must_use]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Carrier<U> {
        Carrier {
            value: f(self.value),
            diags: self.diags,
        }
    }

    /// Sequence a stage, concatenating its diagnostics after `self`'s.
    #[must_use]
    pub fn and_then<U>(mut self, f: impl FnOnce(T) -> Carrier<U>) -> Carrier<U> {
        let mut next = f(self.value);
        self.diags.append(&mut next.diags);
        Carrier {
            value: next.value,
            diags: self.diags,
        }
    }

    pub fn push(&mut self, diag: Diagnostic) {
        self.diags.push(diag);
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.severity == Severity::Error)
    }

    #[must_use]
    pub fn into_parts(self) -> (T, Vec<Diagnostic>) {
        (self.value, self.diags)
    }
}

// ===========================================================================
// String interning + the referent-agnostic opaque token (dn-4, W4)
// ===========================================================================

/// An interned string handle. Cheap to copy and compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(u32);

/// Interns strings to [`Symbol`]s. Deterministic: equal input → equal symbol,
/// and symbol assignment is order-of-interning (never hashed/random).
#[derive(Debug, Default)]
pub struct Interner {
    strings: Vec<Box<str>>,
    lookup: HashMap<Box<str>, Symbol>,
}

impl Interner {
    pub fn intern(&mut self, text: &str) -> Symbol {
        if let Some(&sym) = self.lookup.get(text) {
            return sym;
        }
        let sym = Symbol(u32::try_from(self.strings.len()).unwrap_or(u32::MAX));
        let boxed: Box<str> = text.into();
        self.strings.push(boxed.clone());
        self.lookup.insert(boxed, sym);
        sym
    }

    /// Resolve a symbol minted by *this* interner back to its text.
    #[must_use]
    pub fn resolve(&self, sym: Symbol) -> &str {
        &self.strings[sym.0 as usize]
    }
}

/// An opaque state-entity token (research wall `W4`, chord `referent-agnostic`):
/// the analyzer keeps relational contracts over symbols it is forbidden to
/// *understand*. You may compare two `OpaqueToken`s for equality (intra-script
/// co-reference) and resolve one for display/provenance — but you must NEVER
/// branch on its decoded text to infer meaning (that what-is-`nginx` job belongs
/// to the oracle, not the engine). Cross-oracle identity binds to a named kind,
/// never to a shared token (chord `cross-oracle-named-kind`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpaqueToken(pub Symbol);

/// A named, oracle-declared *kind* (`package`, `service`, …) — the anchor for
/// cross-oracle identity (wall `W4`, the dn-1 hinge). Like [`OpaqueToken`], the
/// name is NEVER decoded for meaning; two oracles declaring the same kind name
/// are coherent providers of one kind (chord `cross-oracle-named-kind`). The
/// Tier-A blessed forms use well-known kind names (`file`, `tool`, `freshness`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindId(pub Symbol);

/// An oracle *provider* (`apt-get`, `dpkg`, …) — the `(provider, verb)` key of
/// the fact-centric effect map (note 162). An interned name, never decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProviderId(pub Symbol);

// ===========================================================================
// Analysis vocabulary: phase, verdict, grade, fact-domain, fact
// ===========================================================================

/// Which pass we are in. The two soundnesses are *phase-keyed*, with opposite
/// fail-directions (welded knob `kFAIL`, chords `two-soundnesses`/`phase-flip`):
/// the probe pass fails toward "don't touch it", the apply pass toward "don't
/// skip it". A shortcut is only legal if it fails the conservative way for its
/// phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Read-only probe projection — never mutates (`kFAIL-withhold`).
    Probe,
    /// Mutating apply — never skips a needed mutation (`kFAIL-perform`).
    Apply,
}

/// Three-valued convergence verdict (chord `ch-verdict`: ok/fail/unknown, kept
/// distinct from the diagnostic stream). `Unknown` is first-class and folds
/// conservatively — an unreachable host or an un-probeable fact is `Unknown`,
/// never silently `Converged` (that would be a `kFAIL-perform` violation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Desired state already established → the mutation may be skipped.
    Converged,
    /// Desired state not established → the mutation must run.
    Diverged,
    /// Cannot tell → must act conservatively for the phase.
    Unknown,
}

/// A concrete observed **exit status** (`19A §5`, `an-probe-shape`/`DP-3`): the
/// *value* a leaf's command yields, held opaquely. The apply abstract-interpreter
/// folds `&&`/`||`/`if`/`!` over this value (`9 || cmd` ⇒ `cmd` runs, by the shell's
/// own semantics) and the substitution reproduces it exactly. **rc is opaque to
/// Dorc** (`inv-referent-agnostic`-adjacent): we hold `9`, never interpret what `9`
/// *means* — the author already encoded the meaning by choosing `!`/`&&`/`||`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rc(pub i32);

/// A predicted **output claim** for the `Stdout`/`Stderr` channels (`inv-one-observable`,
/// `19F` §3 tuple completion): the captured text a substitution would have to reproduce.
/// An interned [`Symbol`] (the cheapest deterministic `Copy` representation — keeps
/// [`Observable`] `Copy`, and the interner is order-of-interning so it never leaks
/// nondeterminism, `inv-determinism`). The engine NEVER decodes it (`inv-referent-agnostic`):
/// a substitution compares/reproduces the claim, the analyzer does not branch on its text.
///
/// NOTHING produces a non-⊤ `OutClaim` this round (the existing consumed-stdout/stderr gate
/// stays the unconditional block it is — a consumed channel with a ⊤ prediction blocks,
/// today's rule). The newtype exists so a future stdout-producing probe is a value-plumbing
/// change, not a representation change (the `19F` failure was exactly representation drift).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutClaim(pub Symbol);

/// A predicted value for one observable channel (`inv-one-observable`): a concrete
/// value, or a loud out-of-band ⊤ "can't-predict". A `Top` on a *consumed* channel
/// forces the consuming leaf to run (`inv-kfail`/`kFAIL-perform`): the check could not
/// predict the value a downstream context reads, so no stand-in can reproduce it. (The
/// fold's former `AbstractRc` was this type by another name — `Known`/`Top`.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Predicted<T> {
    /// The check predicts this exact value (an oracle-declared converged-rc, …).
    Value(T),
    /// ⊤: the check cannot predict this channel ⇒ no fold / no substitution through it.
    Top,
}

/// The channels of a command's observable output — the single shared vocabulary of the
/// ONE Observable (`inv-one-observable`). A **closed** enum: adding a channel must break
/// every exhaustive `match` (the compiler-as-checklist), so it carries NO
/// `#[non_exhaustive]`. Replaces the former `analysis::cfg::Observable` consumption enum,
/// unifying it with `Verdict`/`Observed` — the round-19 three-way split (`19F`).
///
/// Two views key off this one vocabulary: an [`Observable`] *predicts* a value per
/// value-bearing channel; an enclosing context *consumes* a `Powerset<Channel>` (the
/// liveness set). The `Effect` channel is vouched by convergence (the forward gate), so
/// it never enters the *consumed* set — it gates the elision license instead.
///
/// The two status channels differ by **render-expressibility, not construct identity**
/// (`206` §3, executed in task-O): four sources now feed the value-relaxable channel (a
/// `&&`/`||` operand, an errexit-region command's rc, a `$?`-reader's predecessor, and any
/// future reader a KNOWN rc reproduces exactly), but only ONE feeds the render-floor
/// channel (an `if`/`elif` guard). With four-vs-one the axis is provably *can the
/// line-granular render substitute the consumer in-situ*, not *which construct consumed
/// the status* — so the names say which render capability retires each, not which syntax
/// produced it. The `AndOrStatus` name (round-19, `19G`'s deferred `ch-wrong` bake-and-see)
/// is retired as misleading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    /// The command's effect on managed state (mutation). Vouched by convergence ⇒ never
    /// in the consumed set; its predicted value is the [`Observable::effect`] verdict.
    Effect,
    /// Status consumed by an `if`/`elif` **guard** — blocks unconditionally TODAY because
    /// the line-granular render cannot substitute a guard in-situ (it shares its line with
    /// the `if`/`then`/`fi` scaffolding; eliding it breaks `dash -n`). Retired ONLY by a
    /// guard-capable leaf-exact render (`C-5`), NOT by channel semantics — a ⊤ vs known rc
    /// makes no difference here, the floor is the render, not the value (`19C` strain-D).
    StatusRenderFloor,
    /// Status consumed by a value-relaxable reader — `&&`/`||` operands, errexit-region
    /// commands, `$?`-readers' predecessors, and any future reader whose decision a KNOWN
    /// rc reproduces exactly (`206` §3: four sources feed this channel). A ⊤ rc blocks; a
    /// probe-sourced/known rc substitutes (the value-preserving stand-in reproduces the
    /// exact status, so the branch decides identically). Eliding a ⊤-rc operand to a
    /// fabricated rc-0 `true` would suppress a `|| fallback` — the `kFAIL-perform`
    /// under-execute (`19D`).
    StatusRelaxable,
    /// fd 1 captured to a real (non-`/dev/null`) sink ⇒ value-bearing, vouched by
    /// nothing ⇒ a consumed `Stdout` always blocks (16F §3).
    Stdout,
    /// fd 2 captured to a real sink — as `Stdout`.
    Stderr,
}

/// The ONE Observable (`inv-one-observable`): a command's predicted output over
/// [`Channel`]s. Replaces the round-19 three-way split — the `analysis::cfg::Observable`
/// consumption enum, the standalone `core::Verdict`, and the bolted `Observed{verdict,
/// rc}` (`19F`). The oracle `.check()` PREDICTS it; an enclosing context CONSUMES some
/// channels; a substitution REPRODUCES the consumed channels' predicted values, and is
/// licensed only when the `Effect` channel predicts no-mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observable {
    /// `Effect` channel: the host-reported convergence [`Verdict`], refined to
    /// no-mutation by the ambient gate downstream. "Convergence is the *derived* state
    /// of the Effect channel" (`19F` §3) — `Verdict` is its value, no longer a separate
    /// probe-reported concept.
    pub effect: Verdict,
    /// The predicted exit **status** when converged — the oracle's declared converged-rc.
    /// `Predicted::Top` ⇒ undeclared ⇒ no fold through this leaf (the `19D` `kFAIL-perform`
    /// floor: never fabricate a conforming rc-0). The consuming side decides which status
    /// channel reads it: a [`Channel::StatusRelaxable`] reader folds/substitutes a known
    /// value; a [`Channel::StatusRenderFloor`] guard blocks regardless (the render floor).
    pub status: Predicted<Rc>,
    /// `Stdout` channel: the predicted fd-1 [`OutClaim`] a substitution must reproduce.
    /// ALWAYS `Predicted::Top` this round (`19F` §3 shape completion — nothing produces a
    /// value yet): a *consumed* `Stdout` with a ⊤ prediction blocks the license
    /// unconditionally, which is exactly today's rule (`consumption_ok`, 16F §3), now
    /// expressed through the tuple rather than a side-channel.
    pub stdout: Predicted<OutClaim>,
    /// `Stderr` channel: the predicted fd-2 [`OutClaim`] — as [`stdout`](Self::stdout).
    pub stderr: Predicted<OutClaim>,
}

impl Observable {
    /// An observable carrying only the convergence verdict, with an **unpredicted**
    /// status and unpredicted stdout/stderr (all `Predicted::Top` ⇒ ⊤ for the fold). The
    /// conservative shape: convergence still drives elision, but no branch folds through
    /// this leaf's status, and a consumed stdout/stderr blocks (16F §3).
    #[must_use]
    pub fn verdict_only(effect: Verdict) -> Self {
        Self {
            effect,
            status: Predicted::Top,
            stdout: Predicted::Top,
            stderr: Predicted::Top,
        }
    }
}

/// Belief grade (Engler MUST/MAY, chord `must-may`) — the sound/unsound line.
/// Only a `Must` fact may license a skip; `May` (mined/distributional) is a hint
/// that bootstraps the oracle library and never authorizes elision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Must,
    May,
}

/// One independently-mutation-gating facet of a kind's ≥enum state-model
/// (`17N inc-S` / `an-per-entity-selector`). An interned name; never decoded
/// (`inv-referent-agnostic`) — compared for co-reference, resolved for display.
///
/// The selector is what splits a flat per-(kind,entity) bit into independent
/// cells: `service#enabled` and `service#active` are *separately* mutation-gating
/// (`systemctl enable --now` writes both; an `is-active` probe must not discharge
/// an unmet `#enabled`), which a flat key could not hold (`notes/193` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectorId(pub Symbol);

/// The cell a fact is about: an operand-named cell, or the kind's implicit
/// singleton (`notes/193` §3; `an-host-identity-fact`-adjacent).
///
/// `apt-get update` is a nullary mutator on the one package index — no operand —
/// so the key must carry [`Singleton`](EntityRef::Singleton), not require an
/// [`OpaqueToken`]. The old flat key required a token, so a no-operand mutator
/// fell through to `Opaque ⇒ Reach::Top ⇒ the poison wall`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityRef {
    /// A cell named by a literal operand (`package:nginx`). Two operand tokens
    /// denote the same cell iff they compare equal (`an-entity-coref`).
    Operand(OpaqueToken),
    /// The kind's implicit single cell (`package-index`, the one apt index).
    Singleton,
}

/// A system-state fact-key, re-keyed for spike-2 (`notes/193` §3 / charter §3 /
/// `16Q §1`). The flat `(kind, entity)` pair gains a [`selector`](FactKey::selector)
/// — the cell coordinate the whole engine reaches over.
///
/// `dec-seam-ownership` (closed → `core`): the structured entity-algebra is the
/// shared vocabulary every crate agrees on first (`dac-B`), so it is *defined here*
/// and `analysis::effect::FactKey` re-exports this type rather than holding a
/// parallel key. Carries NO source span (provenance is the node's). Two keys are
/// equal iff `kind` + `entity` + `selector` all match.
///
/// `Copy`/`Ord`/`Hash` are preserved: `Reach`'s `BTreeSet<FactKey>` needs `Ord`,
/// and [`EntityRef`]/[`SelectorId`] are themselves `Copy`+`Ord`, so the bound holds.
/// `inv-determinism`: any map/set keyed on `FactKey` stays `BTree*`, never
/// hashed-into-output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactKey {
    pub kind: KindId,
    pub entity: EntityRef,
    pub selector: SelectorId,
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interner_dedups_and_roundtrips() {
        let mut i = Interner::default();
        let nginx_a = i.intern("nginx");
        let nginx_b = i.intern("nginx");
        let apt = i.intern("apt");
        assert_eq!(nginx_a, nginx_b, "equal text must intern to equal symbol");
        assert_ne!(nginx_a, apt);
        assert_eq!(i.resolve(nginx_a), "nginx");
        assert_eq!(i.resolve(apt), "apt");
    }

    #[test]
    fn interner_symbol_assignment_is_deterministic() {
        let mut a = Interner::default();
        let mut b = Interner::default();
        for s in ["one", "two", "three", "two"] {
            let _ = a.intern(s);
            let _ = b.intern(s);
        }
        assert_eq!(a.intern("one"), b.intern("one"));
        assert_eq!(a.intern("three"), b.intern("three"));
    }

    #[test]
    fn carrier_threads_diagnostics_through_stages() {
        let result = Carrier::pure(2).map(|n| n + 1).and_then(|n| {
            Carrier::new(
                n * 10,
                vec![Diagnostic::warning(DiagCode("test-warn"), None, "heads up")],
            )
        });
        assert_eq!(result.value, 30);
        assert_eq!(result.diags.len(), 1);
        assert!(!result.has_errors());
    }

    #[test]
    fn carrier_reports_errors_without_panicking() {
        let mut c = Carrier::pure(());
        c.push(Diagnostic::error(
            DiagCode("boom"),
            None,
            "bad input, kept going",
        ));
        assert!(c.has_errors());
    }
}
