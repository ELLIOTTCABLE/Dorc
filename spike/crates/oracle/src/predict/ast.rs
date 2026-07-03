//! The dialect AST — the precise, greppable surface the contract dialect admits.
//!
//! Grown ONLY as 19H §2's five examples demand (apt-get §2.1, command §2.2,
//! useradd §2.3, systemctl §2.5, the cross-oracle pair §2.4), not speculatively.
//! Every node here is something one of those bodies contains; nothing else parses.

use dorc_core::{Span, Symbol};

/// The set of `<provider>__predict` functions lifted from one oracle file. Keyed by
/// the **provider** (the name before `__predict`, with the underscore↔hyphen mapping
/// applied — see [`Predict::provider`]). `BTreeMap`-ordered (`inv-determinism`).
#[derive(Debug, Clone, Default)]
pub struct PredictSet {
    pub(super) checks: std::collections::BTreeMap<Symbol, Predict>,
}

impl PredictSet {
    /// The check for a provider, if the file declared one.
    #[must_use]
    pub fn get(&self, provider: Symbol) -> Option<&Predict> {
        self.checks.get(&provider)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.checks.len()
    }

    /// Providers with a lifted check, in deterministic order.
    pub fn providers(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.checks.keys().copied()
    }
}

/// One `<provider>__predict` function: the provider it serves plus the dialect
/// statements of its body, in source order. The evaluator ([`super::evaluate`])
/// executes [`body`](Predict::body) over a concrete argv.
#[derive(Debug, Clone)]
pub struct Predict {
    /// The provider this check argparses for — the name before `__predict`, with
    /// underscores mapped to hyphens (`apt_get__predict` ⇒ `apt-get`). Interned.
    pub provider: Symbol,
    /// The function-name span (for diagnostics pointing at the definition).
    pub name_span: Span,
    /// The whole funcdef span (from the name word through the closing `}`), so the
    /// strip (R1c) can slice the funcdef out of the oracle source and surgically edit it.
    pub span: Span,
    /// The interned symbol of the conventional verb-binding name (`verb`), stamped
    /// at lift time so the (interner-free) evaluator can recognize a `verb=…`
    /// assignment by symbol equality without decoding text. Always present (the
    /// parser interns the fixed name once); a check that never assigns it simply
    /// binds no verb.
    pub verb_sym: Symbol,
    /// The body statements, in source order.
    pub body: Vec<Stmt>,
}

/// A dialect statement. Each variant is drawn from a 19H §2 example body.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// `name=WORD` / `verb=$1` — a plain assignment (one lvalue, one rvalue word).
    Assign { name: Symbol, value: Word },
    /// `shift` / `shift N` — consume positional parameters. `None` ⇒ `shift 1`.
    Shift { count: Option<u32> },
    /// `while TEST; do … done` — the flag-strip loop (`[ "${1#-}" != "$1" ]`).
    While { test: Test, body: Vec<Stmt> },
    /// `case WORD in ARMS esac` — verb/flag dispatch over `$1` or `$verb`.
    Case { scrutinee: Word, arms: Vec<CaseArm> },
    /// `if TEST; then … [else …] fi` — admitted by the dialect surface though no
    /// §2 example uses it (19H §2 says "`if`/`then`/`fi` where needed").
    If {
        test: Test,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    /// `name : reverse.dns.Kind = "$N"` — the inline kind-annotation (the
    /// `ch-shape-anno` inline form, sanctioned spike debt). A command-shaped word
    /// sequence `[name, :, kind, =, value]` recognized as the annotation.
    Annotation(Annotation),
    /// A plain command (a read-only probe body, e.g. `dpkg-query -W "$pkg"`). Its
    /// VERBATIM SOURCE TEXT is preserved span-exactly ([`Command::span`]) for
    /// shipping into a probe artifact later. May carry a trailing effect [`Mark`]
    /// (233 ESTABLISH/OBSERVE — `Command::mark`).
    Command(Command),
    /// A bare inline-dialect mark in statement position (233 §1–§4, R1b): a POISON
    /// no-op mention (`: kind`), an ACK vouch (`: kind:entity.prop~`), or the
    /// CONVERGED-VOUCH placeholder (`: provider:verb~`). Distinguished from a trailing
    /// [`Command::mark`] by having no command in front of the `:` marker. Never
    /// evaluated (a no-op for entity-resolution); consumed by the lift + strip.
    Mark(Mark),
}

/// A parsed inline-dialect mark (233 §1–§4, R1b): an effect / vouch / mention
/// annotation, either trailing a command (ESTABLISH/OBSERVE) or standing alone
/// (ACK/POISON/converged-vouch). Every fragment is an OPAQUE syntactic string
/// (`inv-referent-agnostic`): the parser splits `kind:entity.prop` structurally and
/// NEVER decodes what the tokens mean. Carries a [`span`](Mark::span) covering the
/// marker plus target (for the surgical strip, R1c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    /// Which dialect mark this is.
    pub kind: MarkKind,
    /// The `kind:entity.prop` coordinate (any level may be absent).
    pub target: MarkTarget,
    /// The mark span, from the `:`/`:?` marker token through the end of the target
    /// (including any `~`/`!` suffix). The strip deletes/rewrites exactly this region.
    pub span: Span,
}

/// The dialect mark discriminant (233 §1–§4 / R1b). ESTABLISH/OBSERVE trail a
/// command; ACK/POISON/converged-vouch stand alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkKind {
    /// `cmd … : kind:entity.prop` — the command's rc establishes the property
    /// (→ `ValueClaim::Establish` in the derivation).
    Establish,
    /// `cmd … : kind:entity.prop!` — ESTABLISH with the rc sense inverted; the verb
    /// makes the fact NOT hold (→ `ValueClaim::EstablishInverted`; 233 §1's `!` pun).
    EstablishInverted,
    /// `cmd … :? kind:entity.prop` — depends-upon / read-only observe
    /// (→ `ValueClaim::Observe` in the derivation; 233 OBSERVE).
    Observe,
    /// `: kind:entity.prop~` — a considered-untouched vouch (233 ACK). A no-op under
    /// the dead m×n negative-enumeration (23D §5): parsed and carried, licenses
    /// nothing. Distinguished from [`ConvergedVouch`](MarkKind::ConvergedVouch) by
    /// carrying a `.prop` (three-level `kind:entity.prop`), where the vouch is
    /// two-level `provider:verb` (jc-vouch-vs-ack).
    Ack,
    /// bare `: kind` / `: kind:entity` / `: kind:entity.prop` — a no-op mention
    /// (233 POISON). Its cells are what the lift may poison (a mention with no `~`).
    Poison,
    /// `: provider:verb~` in statement position — the CONVERGED-VOUCH placeholder.
    /// A STRAWMAN for an open spelling (dq-kOOB); the parser derives it into a
    /// [`DerivedVouch`](super::DerivedVouch). Two-level (`provider:verb`, no `.prop`)
    /// + a `~` suffix.
    ConvergedVouch,
}

/// The `kind:entity.prop` coordinate of a [`Mark`], split syntactically and left
/// OPAQUE (`inv-referent-agnostic` — never decoded). Any level may be absent (a
/// kind-only POISON `: fs.Path`). For a [`MarkKind::ConvergedVouch`] the two-level
/// `provider:verb` shape reuses `kind`=provider and `entity`=verb.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkTarget {
    /// The kind fragment (everything before the first `:`). Opaque.
    pub kind: String,
    /// The entity fragment (between the first `:` and the last `.`), if present.
    pub entity: Option<String>,
    /// The property/selector fragment (after the last `.`), if present. Opaque.
    pub prop: Option<String>,
    /// The optional `= value` tail on an ESTABLISH (233 §1: an explicit value).
    pub value: Option<Word>,
}

/// The inline kind-annotation `name : kind = value` (19H §2.1, ch-shape-anno).
///
/// `name` and `kind` are diagnostic/coordination data; the load-bearing part is
/// [`value`](Annotation::value) — the word whose concrete resolution against the
/// argv IS the resolved entity.
#[derive(Debug, Clone)]
pub struct Annotation {
    /// The local name bound (`pkg`, `tool`, `svc`, `user`). Interned. Not used by
    /// the evaluator's resolution (the value-position is what matters) but kept for
    /// provenance and so an over-eager future binding-tracker has it.
    pub name: Symbol,
    /// The reverse-DNS kind string (`com.debian.apt.Package`) or a short kind name
    /// (`package`) — the derivation keys the effect-map on exactly this string, so the
    /// annotation-kind IS the effect-map kind. An opaque coordination handle
    /// (`inv-referent-agnostic`); never decoded for meaning.
    pub kind: String,
    /// The annotated value word (`"$1"`), or `None` for the **nullary/Singleton**
    /// form (`index : pkgindex` with no `= value`): a verb whose resource has no
    /// operand (`apt-get update`; 202 §2). A present value resolves to a concrete
    /// argv element (else ⊤); `None` resolves to the Singleton entity.
    pub value: Option<Word>,
    /// The whole annotation span (diagnostics), covering `name : kind [= value]`.
    /// The strip (R1c) replaces exactly this region with `name=value` (or `name=`
    /// for the nullary form).
    pub span: Span,
    /// The span of the bound `name` token (`pkg`), for the surgical strip: the
    /// stripped assignment reuses the author's verbatim name bytes.
    pub name_span: Span,
    /// The span of the value word (`"$1"`), if present. For the nullary form it is
    /// `None` (the strip emits `name=`).
    pub value_span: Option<Span>,
}

/// A plain command in a probe body, with its verbatim source span preserved.
#[derive(Debug, Clone)]
pub struct Command {
    /// The command words (`[dpkg-query, -W, "$pkg"]`), each a [`Word`]. Kept so the
    /// evaluator can confirm the command is well-formed dialect; the *shipped* form
    /// is the verbatim [`span`](Command::span), not a re-render of these.
    pub words: Vec<Word>,
    /// VERBATIM source span of the whole command, EXCLUDING any trailing [`mark`](Command::mark)
    /// (the span ends at the last real word/redirect). Includes any `>/dev/null`
    /// redirection that is part of it. This is what ships into the probe artifact —
    /// span-exact, never re-serialized (202 §3 / C-1).
    pub span: Span,
    /// The trailing effect [`Mark`] (233 ESTABLISH/OBSERVE), if the command carried
    /// one. `None` for a bare probe command. Not evaluated (the command still runs as
    /// the probe body); consumed by the lift (effect-map derivation) and the strip
    /// (removal — the byte-region `[span.hi .. mark.span.hi]` is deleted).
    pub mark: Option<Mark>,
}

/// A test inside `while`/`if`. The dialect admits exactly the shape the flag-strip
/// idiom needs: `[ WORD OP WORD ]` with a string comparison operator.
#[derive(Debug, Clone)]
pub struct Test {
    pub lhs: Word,
    pub op: TestOp,
    pub rhs: Word,
    pub span: Span,
}

/// String-comparison operators admitted in a `[ … ]` test. `!=`/`=` are what the
/// `${1#-}` prefix-strip idiom (19H §2.1) uses; nothing else is needed yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOp {
    /// `=` — string equality.
    Eq,
    /// `!=` — string inequality.
    Ne,
}

/// One `case` arm: a set of patterns and the statements run when one matches.
#[derive(Debug, Clone)]
pub struct CaseArm {
    /// The arm's patterns (`-t|-o` ⇒ two patterns). A match is "any pattern
    /// matches the scrutinee".
    pub patterns: Vec<Pattern>,
    /// The arm body, run on the first matching arm (sh `case` semantics).
    pub body: Vec<Stmt>,
}

/// A `case` arm pattern. The dialect admits only literal patterns and the `*`
/// catch-all — no `?`/`[…]`/`@(…)` globbing (those would make arm-selection a
/// pattern-match problem; out of dialect ⇒ the parser rejects them).
#[derive(Debug, Clone)]
pub enum Pattern {
    /// A literal pattern (`-t`, `enable`, `install`). Matches iff the scrutinee
    /// equals it exactly.
    Literal(String),
    /// `*` — the catch-all. Matches anything.
    Wildcard,
}

/// A word — the dialect's value expression. Resolved to a concrete string (or Top)
/// at evaluation time against the argv and the binding environment. Each variant is
/// drawn from a §2 example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Word {
    /// A bare literal (`install`, `-W`, `--`). From an unquoted or single-quoted
    /// token with no expansion.
    Literal(String),
    /// `$N` / `"$N"` — a positional parameter (1-based). `$0` is the function name,
    /// which the dialect has no use for; `$0` ⇒ resolves to Top at eval (we never
    /// model a function name).
    Positional(u32),
    /// `${N#PREFIX}` — positional `N` with a leading literal `PREFIX` stripped
    /// (shortest match; sh `${var#pat}`). Only the literal-prefix form the
    /// flag-strip idiom uses is admitted (`${1#-}`).
    PositionalStripPrefix { n: u32, prefix: String },
    /// `$name` / `"$name"` — a variable reference (`$verb`, `$pkg`, `$svc`). Resolved
    /// against the binding environment; unbound ⇒ Top.
    Var(Symbol),
    /// `'$1'` — a single-quoted token whose `$` is literal (NOT a positional). A
    /// distinct variant so the evaluator can treat it as the literal string `$1`,
    /// per sh single-quote semantics. Kept separate from [`Literal`](Word::Literal)
    /// only for clarity at the parse boundary; evaluates identically to a literal.
    SingleQuotedLiteral(String),
    /// A parameter-expansion form the dialect does not model (`${x:-y}`, a globby
    /// or `##` prefix-strip, …). MUST fail to resolve in EVERY position — value,
    /// annotation, and `[ ]` test alike. (Round-20 crosscheck: routing these to
    /// [`Literal`](Word::Literal) made a test compare the literal `${1#*=}` text —
    /// a wrong concrete vs dash's glob semantics. Unmodeled ⇒ Top, never a value.)
    Unmodeled(String),
}
