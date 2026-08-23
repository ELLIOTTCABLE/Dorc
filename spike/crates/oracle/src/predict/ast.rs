//! The dialect AST — the precise, greppable surface the contract dialect admits.
//!
//! Grown ONLY as 19H §2's five examples demand (apt-get §2.1, command §2.2,
//! useradd §2.3, systemctl §2.5, the cross-oracle pair §2.4), not speculatively.
//! Every node here is something one of those bodies contains; nothing else parses.

use dorc_core::{ContestedFamilies, Interner, Span, Symbol};

/// The set of `<provider>__predict` functions lifted from one oracle file. Keyed by
/// the **provider** (the name before `__predict`, with the underscore↔hyphen mapping
/// applied — see [`Predict::provider`]). `BTreeMap`-ordered (`inv-determinism`).
///
/// One row per DEFINITION, not per provider (`28Q` §1.1): a file may declare a role twice — the
/// `unset -f`-then-redefine shape is BLESSED, not contested — and each definition binds at every
/// site between it and its successor, so keeping only one leaves the earlier frame with no row to
/// find. The map's VALUE is therefore the definitions in source order; [`get`](Self::get) answers
/// the last of them, which is the file's exit binding and what every whole-file consumer asks for,
/// and [`all`](Self::all) is what a per-FRAME consumer enumerates.
#[derive(Debug, Clone, Default)]
pub struct PredictSet {
    pub(super) checks: std::collections::BTreeMap<Symbol, Vec<Predict>>,
    pub(super) detected: Vec<DetectedFn>,
}

/// A role-funcdef header the parse RECOGNIZED, recorded BEFORE its body is attempted — so a
/// header present here but absent from `checks` is a funcdef the file declared and the lift lost.
/// That difference is the whole input to the marks-lost backstop (`crate::validate`); nothing
/// else reads it, and it steers no lift decision.
#[derive(Debug, Clone)]
pub struct DetectedFn {
    /// The funcdef name exactly as the file spells it (`wombat__is_converged`).
    pub name: String,
    /// The provider/kind symbol the header keys — the same key `checks` would use.
    pub provider: Symbol,
    /// The name token's span, so a diagnostic's caret lands on the declaration.
    pub name_span: Span,
}

impl PredictSet {
    /// The check a shell would have bound at the END of this file — the LAST declaration, which is
    /// what a whole-file consumer (the marks backstop, the dialect scan, a hand-built index) means
    /// by "the file's check".
    #[must_use]
    pub fn get(&self, provider: Symbol) -> Option<&Predict> {
        self.checks.get(&provider).and_then(|rows| rows.last())
    }

    /// EVERY definition of a provider this file declares, in source order — the candidate list a
    /// per-frame resolution seat enumerates (`28Q` §1.3; the rule itself is
    /// [`dorc_core::answering_row`]'s, never this accessor's).
    #[must_use]
    pub fn all(&self, provider: Symbol) -> &[Predict] {
        self.checks.get(&provider).map_or(&[][..], Vec::as_slice)
    }

    /// The role-funcdefs the file DECLARED whose bodies never reached [`checks`](PredictSet::get)
    /// — a declared-but-lost funcdef, whatever lost it. Source order (`inv-determinism`).
    pub fn unlifted(&self) -> impl Iterator<Item = &DetectedFn> + '_ {
        self.detected
            .iter()
            .filter(|d| !self.checks.contains_key(&d.provider))
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

    /// Drop every provider whose family `contested` withholds (`28K` §1
    /// `rul-silent-shadowing-refuses`) — the family goes UNDESCRIBED, indistinguishable from one
    /// nobody wrote an oracle for.
    ///
    /// `detected` is filtered alongside `checks`, deliberately: leaving the header behind would
    /// make the marks-lost backstop (`crate::validate`) report a WITHDRAWN funcdef as a lift
    /// failure, which points the author at the wrong repair (`271:rul-sin-ordering`).
    #[must_use]
    pub fn withdrawing(mut self, contested: &ContestedFamilies, interner: &Interner) -> Self {
        if contested.is_empty() {
            return self;
        }
        let withheld =
            |p: Symbol| contested.withholds(&crate::to_funcname_segment(interner.resolve(p)));
        self.checks.retain(|p, _| !withheld(*p));
        self.detected.retain(|d| !withheld(d.provider));
        self
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
    /// (`277` §4a/§4d — a verdict `:`/`:!` or observe `:?` mark). The substrate/
    /// invariance colon-lines of `state_stored_only_in` (`277` §4e) are ALSO plain
    /// commands: the sh no-op `:` carrying a trailing `: <token>` mark.
    Command(Command),
    /// An AND-OR list (`a && b`, `a || b`, `a & b`) — see [`AndOr`].
    AndOr(AndOr),
}

/// An and-or list: a first item plus a run of operator-joined items. FLAT, not left-nested — sh's
/// and-or operators share one precedence and associate left, so a tree would encode structure the
/// grammar does not have.
///
/// ACCEPTED at parse and degraded at TRACE, the same posture a pipeline gets, for a concrete
/// reason: the corpus's one and-or list is in a `sm_dorc_Package__resolve()` body, and a resolver
/// is host-run strip-only — a parse rejection would delete a working resolver over a construct
/// nothing traces.
#[derive(Debug, Clone)]
pub struct AndOr {
    /// The list's first item.
    pub first: AndOrItem,
    /// The operator-joined remainder, in source order. Never empty (a list with no operator is
    /// just a statement).
    pub rest: Vec<AndOrLink>,
    /// VERBATIM span of the whole list, first item through last — what ships, byte-exact.
    pub span: Span,
    /// Trailing effect marks REFUSED off this list's items, kept ONLY so [`crate::strip`] can
    /// still erase their bytes; no semantic consumer can reach them from here.
    ///
    /// A verdict mark claims "THIS command's rc establishes the property", but a list's rc is the
    /// LIST's: in `probe : k:e@sel || return 2` the marked cell's complement sense (rc 1) can never
    /// occur, and in `probe : k:e@sel || true` the rc is forged 0 always (the `R2-ORTRUE`
    /// masked-verdict shape). Refusing forces the unmasked spelling that law already demands, and
    /// `281` §7's one-verdict-per-line rc-arity says the same from the grammar side.
    pub refused_marks: Vec<RefusedMark>,
}

impl AndOr {
    /// The list's items, in source order.
    pub fn items(&self) -> impl Iterator<Item = &AndOrItem> + '_ {
        std::iter::once(&self.first).chain(self.rest.iter().map(|l| &l.item))
    }

    /// The list's COMMAND items, in source order — the walkers' usual need (a
    /// [`Test`](AndOrItem::Test) item carries no words, marks, or redirects to visit).
    pub fn commands(&self) -> impl Iterator<Item = &Command> + '_ {
        self.items().filter_map(|i| match i {
            AndOrItem::Command(c) => Some(c),
            AndOrItem::Test(_) => None,
        })
    }
}

/// One operator-joined item of an [`AndOr`].
#[derive(Debug, Clone)]
pub struct AndOrLink {
    /// The operator joining this item to what precedes it.
    pub op: AndOrOp,
    /// The operator token's own span (diagnostics point here).
    pub op_span: Span,
    /// The item to the operator's right.
    pub item: AndOrItem,
}

/// An [`AndOr`] list operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndOrOp {
    /// `&&` — the right item runs only if the left SUCCEEDED.
    AndThen,
    /// `||` — the right item runs only if the left FAILED.
    OrElse,
    /// `&` — background the LEFT item and continue. A separator, not a conjunction; it rides this
    /// enum so a backgrounded item lands in the one node that permanently degrades instead of
    /// needing a second unmodeled-statement shape. No supported form ever admits it.
    Async,
}

/// One item of an [`AndOr`] list.
#[derive(Debug, Clone)]
pub enum AndOrItem {
    /// A simple command — the ordinary item.
    Command(Command),
    /// A `[ … ]` bracket test in list position. Produced ONLY inside a list (a bare test statement
    /// stays out of dialect), which is what lets a tracer decide such a list statically.
    Test(Test),
}

/// A trailing mark refused off an [`AndOr`] item, carrying its host command's span so the strip
/// deletes exactly the `[host.hi .. mark.span.hi]` region it deletes for any marked command.
#[derive(Debug, Clone)]
pub struct RefusedMark {
    /// The host command's verbatim span; erasure begins at its end.
    pub host: Span,
    /// The refused mark.
    pub mark: Mark,
}

/// A parsed inline-dialect mark (`277` §4a/§4d): an effect / observe / emission
/// annotation trailing a command. Every fragment is an OPAQUE syntactic string
/// (`inv-referent-agnostic`): the parser splits `kind:entity@selector` structurally
/// and NEVER decodes what the tokens mean. Carries a [`span`](Mark::span) covering
/// the marker plus target (for the surgical strip, R1c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    /// The mark VERB (`281` §5) — the typed payload discriminant.
    pub kind: MarkKind,
    /// The `kind:entity@selector` coordinate (entity/selector may be absent).
    pub target: MarkTarget,
    /// The mark span, from the `:`/`:!`/`:?` marker token through the end of the
    /// target. The strip deletes exactly this region.
    pub span: Span,
}

/// The dialect mark VERB (`281` §5), the typed discriminant of a mark-block entry.
/// Selected by the sigil head-sugar (`:`/`:!`/`:?`/`:=`) or a period-free verb word. The
/// verb fixes the payload TYPE read out of [`MarkTarget`] (`281` §4 keystone: verbs are
/// period-free, coordinates dotted). During the additive respell ladder the OLD parser
/// still emits these from OLD spellings role-awarely ([`super::parser`]); the payload
/// LOCATION per verb is noted below (the field the split populates).
///
/// Core cell-and-value plane (coordinate payload — `target.kind`/`.entity`/`.prop`):
/// `Asserts`/`Refutes`/`Reads`. Meta plane (token/kind payload): the rest. Payload
/// homes stay where the `kind:entity` split lands until CP-D unifies them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkKind {
    /// `:` (omit sugar) / word `asserts` — verdict, named sense: the command's rc
    /// establishes the property (→ `ValueClaim::Establish`). rc-consuming. Coordinate.
    Asserts,
    /// `:!` / word `refutes` — verdict, complement sense: rc 0 witnesses the cell false
    /// (→ `ValueClaim::EstablishInverted`). rc-consuming. Coordinate.
    Refutes,
    /// `:?` / word `reads` — observe: read-only depends-upon (→ `ValueClaim::Observe`,
    /// backing-widening). Coordinate.
    Reads,
    /// word `safe-across` — the context vouch (`27C` §2; `entry.rs`). Payload = a
    /// dimension token in `target.entity` (old `tolerates:user` split; brace-set there).
    SafeAcross,
    /// word `disturbs` — first-order footprint (`cmd__disturbs`) AND transitive reach
    /// (`kind__disturbance_reaches_only`), unified (`281` §5). Payload = a kind in
    /// `target.kind` (+ `@selector` in `target.prop`); the entity rides the printf line.
    Disturbs,
    /// word `lends` — the wrapper dimension member (`273` §3; `wrapper.rs`). Payload = a
    /// dimension token in `target.kind` (old `: user` / `: fs-view`).
    Lends,
    /// word `stored-in` — the kind's substrate (`272` §2; `carry.rs`). Payload = a
    /// substrate token in `target.kind` (old `: fs` / `: net-kernel`).
    StoredIn,
    /// word `undivided-by-transit-across` — axis invariance (`277` §4e / `27C` §4(a);
    /// `carry.rs`). Payload = an axis token in `target.entity` (old `invariant:user`).
    Undivided,
}

/// The `kind:entity@selector` coordinate of a [`Mark`], split syntactically and left
/// OPAQUE (`inv-referent-agnostic` — never decoded). Entity/selector may be absent (a
/// kind-only emission mark `: disturbs sm.dorc.Package`; a substrate token `: stored-in fs`).
///
/// SEAM (`28A:rul-verdict-value-tail-drops`): the old verdict-position `= value` tail (once a
/// `value: Option<Word>` field here, read by the `carry.rs` read-set-closure walk) is DROPPED
/// with the old grammar — corpus-dead, no `281` spelling. Re-add the field here + re-wire the
/// `carry.rs` value-cleanliness read if the value plane ever returns (extend-by-name; the
/// `.diff`/is-noop value-layer future mints its own spelling, TODO-ADDTL item 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkTarget {
    /// The kind fragment (everything before the first `:`). Opaque; keeps its
    /// reverse-DNS dots (`sm.dorc.Package`).
    pub kind: String,
    /// The entity fragment (between the first `:` and the `@`), if present. An
    /// explicit empty string is the empty-entity form `kind:@sel` (`281` §6).
    pub entity: Option<String>,
    /// The selector fragment (after the `@`), if present. Opaque. (Field named `prop`
    /// for continuity; it is the `@selector` third coordinate position now.)
    pub prop: Option<String>,
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
    /// Whether this "command" is actually a **pipeline** (`cmd | cmd | …`) — 24E §14
    /// (parse-permissively / trace-conservatively). The parser ACCEPTS a pipeline (the
    /// [`kLANG`] mirror-invariant: valid sh must DEGRADE, never hard-kill) as one Command whose
    /// [`span`](Command::span) covers the WHOLE pipeline, so the strip ships it BYTE-EXACT; the
    /// ⊤-bias then lives on the TRACE layer, not the parse layer. A pipeline NEVER statically
    /// resolves ([`inv-top-reject`] honored — it can't produce a wrong value/footprint): the
    /// tracers ⊤ on it (a `touches()` pipeline ESCALATES to host-derivation, 24E §2; a `predict()`
    /// pipeline can't-resolve ⇒ the site RUNS, the safe degrade). `words` holds only the FIRST
    /// stage's words (never interpreted for a pipeline — the ⊤ fires first).
    pub pipeline: bool,
    /// Whether a redirect on this command sends fd 1 (stdout) away from where it would
    /// otherwise flow (`>/dev/null`, `>&2`, `1>file`) — the §2 per-channel STDOUT DECLINE
    /// (`271:rul-only-oracle-bytes-ship` rider 1). Load-bearing ONLY for the composed-probe
    /// coverage rule ([`super::predict_stage_stdout`]): a NON-LAST pipe stage whose predict
    /// voids stdout declines the very channel the downstream stage consumes, so the compound
    /// cannot ship (can't-say ⇒ run). A stderr-only redirect (`2>&1`, `2>/dev/null`) leaves
    /// this `false` — stdout still reaches the pipe. Computed at parse from the
    /// [`super::lexer::Tok::Redirect`] chunks; irrelevant to the strip (which ships the
    /// verbatim span, redirect included).
    pub stdout_void: bool,
    /// Whether this command APPEND-redirects (`>>`) to a recognized versioned report SINK
    /// (`${DREP_V1:-…}` — the `decline-class-emission` idiom, `27W` §2). Computed at parse from
    /// the redirect chunk + its following target word against an engine-owned sink-name list
    /// (`report-lane-versioned-entry`: a new format is a list append, never surgery). Two
    /// load-bearing consumers: the tier-1 inventory ([`super::super::report`]) value-threads a
    /// sink-emitting command's literal format string into `(verb, class)`; and the verdict tracer
    /// treats it as INERT (`tc-emission-inert-in-tracer` — a recognized emission never vouches,
    /// never ⊤s: decision-inert, fail-toward-run). `false` for every ordinary command; irrelevant
    /// to the strip (which ships the verbatim span).
    pub report_sink: bool,
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
    /// `"$@"` — the whole positional list, faithfully (each element one word). The
    /// oracle-side positional model (`273`/`27H` finding-positional-oracle-side-couples-founding-pin):
    /// ONLY the double-quoted `"$@"` is modeled here — it re-expands the caller's argv
    /// verbatim, which is exactly the peeling-wrapper contract (`273` §1: a body whose
    /// command-position `"$@"` runs its argument-slot is a peeling wrapper by tautology).
    /// bare `$@`, `$*`, and `"$*"` are NOT this variant — they word-split / IFS-join and
    /// so do not preserve the argument list, routing to [`Unmodeled`](Word::Unmodeled) ⇒ ⊤
    /// (`271:rul-env-claim-inversion`; `27H` bare-forms-route-to-top).
    ///
    /// Position-aware resolution (the founding-pin transition, `27H`): in COMMAND position
    /// it is concrete-by-construction (the traced positional list) and must NOT ⊤ the check
    /// — the callers that run commands (verdict `run_command`, the predict `Command` handler)
    /// skip it; in VALUE position (annotation RHS, `[ ]` operand, `case` scrutinee) it is
    /// genuinely ⊤ (a multi-value list is not one value) ⇒ [`resolve_word`](super::eval::resolve_word)
    /// returns `Err`.
    PositionalArgs,
    /// `${N#PREFIX}` — positional `N` with a leading literal `PREFIX` stripped
    /// (shortest match; sh `${var#pat}`). Only the literal-prefix form the
    /// flag-strip idiom uses is admitted (`${1#-}`).
    PositionalStripPrefix { n: u32, prefix: String },
    /// `${N-DEFAULT}` / `${N:-DEFAULT}` — positional `N` with a DEFAULT when unset
    /// (the `${2-}` nounset idiom, `24P` §2: `[ "${2-}" = "" ]` asks "is there a
    /// second operand" without tripping `set -u`). Resolves to `$N` if present, else
    /// `default` — independent of the caller's [`UnsetPolicy`] (the `-` explicitly
    /// requests the default). We do not distinguish unset-vs-empty (`-` vs `:-`): the
    /// corpus only ever asks the is-there-an-operand question, where they coincide.
    PositionalDefault { n: u32, default: String },
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
