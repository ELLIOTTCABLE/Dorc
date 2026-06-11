# 22B — the structured-diagnostic API (first draft, round-22)

> Design-only note (2026-06-11). The human ruled this could be "The Product of the
> spike, and one of the first bits of actual code to write for the real codebase" —
> so it is drafted battlefield-bound: the spike proves the shape, the real codebase
> extracts it. Cribs deliberately from **rustc's `Diag` API** and **Elm's narrative
> discipline**, both human-endorsed by name. Evidence basis read in full for this
> draft: `plans/22A` (concl-7/8, §1 arch-3), `notes/226` (§1 tidy gate, §2 the
> Fluent regret — the central guardrail, §6 severity, §7 Elm, §8 golden economics,
> §12 exhaustive-enum spine), `notes/228` §2/§5 (site identity dc-3), `plans/21Z`
> (current state), `notes/224` §7 ru-6 (remediation-class), and the live source
> (`core/src/diag.rs`, `core/src/lib.rs`, `cli/src/main.rs` `report()`, the 17
> scattered `Diagnostic::error` sites). rustc's `Applicability` enum + diagnostic
> structure re-verified live against rustc-dev-guide + nightly-rustc docs this turn.
> AI-authored design; process evidence, never proof (never-vouch). Confidence marks
> +SURE / ~SUSPECT / -GUESS / --WONDER. Slugs: `crib-N`, `refuse-N`, `type-sketch-N`,
> `render-N`, `worked-N`, `fork-N`.

## §0 The thesis up front

The job is the *authoring economics* of a diagnostic, not the diagnostic itself. We
want the GOOD shape — multi-span labels, a suggestion carrying an applicability
confidence, structured help/note children, a coherent narrative render, a typed
payload that proves the diagnostic cites the right objects — to be the **cheapest
thing to write**, so a taste-free contributor who reaches for the obvious
constructor lands in the right form by default, and the wrong form (free-text prose,
a missing span, an un-keyed receipt) is *more* work or does not compile.

The single load-bearing constraint, from which everything else is derived, is the
Fluent-migration regret (`226` §2, `friction-fluent-1..4`): rustc's *deny-level
authoring mandate* — every user string through a Fluent DSL + a complex derive +
a mandatory multi-file edit — imposed real contributor friction and was walked back
to `allow` in Oct 2024 (PR #132182, one file changed, after the discipline had
metastasized into hundreds of `#[allow]`s not worth removing). The friction test
this design must pass, stated and answered honestly in §7:

> **friction test:** adding a new diagnostic code is ≈ one edit in one file. No DSL.
> No derive machinery beyond stock `#[derive]`. No multi-file mandate. No proc-macro
> (forbidden workspace-wide anyway — `inv-no-unsafe`).

Everything rustc's *structural* gate did (the tidy grep, the exhaustive registry)
survived and thrives; everything its *authoring DSL* mandated died. We take the
former and refuse the latter, by name.

A second framing, equally load-bearing and equally human-ratified (ru-12 inversion,
`22A` arch-2): a diagnostic's **artifact-visible parts are fact-plane only**.
Receipts/provenance detail (ProvId, derivation trace) render OUT of shipped `.sh`
artifacts; they surface only in the CLI / `why` / a future dashboard. The Diag value
may *hold* provenance hooks, but the rendering that reaches a committed artifact is
the fact-plane projection. This is a render-time partition, designed into the value
in §3 and the render model in §4.

## §1 What a Diag IS here (and what current code lacks)

Today (`core/src/diag.rs` + `core/src/lib.rs:88-146`, verified at HEAD):

```rust
pub struct DiagCode(pub &'static str);          // a string newtype
pub enum Severity { Error, Warning, Note }       // 3 levels, no floor concept
pub struct Diagnostic {                          // ONE optional span, bare String
    pub severity: Severity,
    pub code: DiagCode,
    pub span: Option<Span>,
    pub message: String,
}
impl Diagnostic { fn error/warning/note(code, span, message) -> Self }
```

and 17 give-up sites call `Diagnostic::error(DiagCode("..."), span, "free text")`
directly (`plans/21Z`: `syntax-unsupported`, 8× `oracle-*`, `cfg-*`,
`render-heredoc-refused` — an inline literal, not even a named const), bypassing the
5-code catalog entirely. The catalog's own Notes are *span-poorer* than the scattered
codes they were meant to supersede (3 of 5 pass `None`). The span is dropped at the
one user surface (`cli report()` renders `{stage}: {sev}[{code}]: {message}` and
never touches `d.span` — `21Z` drop-A). So the current shape has: no multi-span, no
labels, no suggestions, no help/note children, no narrative render, no typed payload,
and the one provenance handle it *does* carry is thrown away unrendered.

The gap between this and "the GOOD shape" is exactly the rustc `Diag` API surface.
That is the crib.

## §2 The cribbing survey

### From rustc's `Diag` API

`crib-1 (the structured shape: primary message + windows of span-labels + ordered
sub-diagnostics).` +SURE. rustc's diagnostic structure (re-verified, rustc-dev-guide
"Diagnostic structure"): a `level[code]`, a primary `message` that "should be general
and able to stand on its own" (makes sense in isolation), a diagnostic *window* of
**primary and secondary spans, each optionally carrying a label**, and ordered
*sub-diagnostics* (`note`/`help`) for when "the order of the explanation might not
correspond with the order of the code." We take this whole anatomy: a Diag is a
message + an ordered set of *labeled spans* (one primary, ≥0 secondary) + ordered
*children* (notes and helps). This is the structural answer to `21Z` drop-B (the
span-poverty) and to `228`'s ⊤-cascade (a secondary label is how the cause-site and
the poisoned-site live in ONE diagnostic instead of N).

`crib-2 (suggestions carry an Applicability confidence).` +SURE. rustc's
`Applicability` (re-verified live, `rustc_lint_defs`): a 4-variant confidence on
every suggestion — `MachineApplicable` (auto-apply, preserves meaning),
`MaybeIncorrect` (valid code but uncertain — consult the user), `HasPlaceholders`
(contains `(...)`-style holes, cannot auto-apply), `Unspecified`. The discipline
that matters: *a tool decides whether to auto-apply from the applicability, not from
the prose.* We take this verbatim as the model for how a Dorc remediation becomes an
*actionable, confidence-marked* suggestion (§3 `type-sketch-3`). It composes
perfectly with the remediation-class axis (`crib-5`): the class says *what kind* of
fix, the applicability says *how confident* we are it's correct.

`crib-3 (help-vs-message separation).` +SURE (re-verified, rustc-dev-guide diagnostic
levels): "The error or warning portion should *not* suggest how to fix the problem,
only the 'help' sub-diagnostic should." The primary message states the *fact* (what
went ⊤, what could not be probed); the *help* child states the *remediation*. This
maps cleanly onto the fact-plane/exempt-plane partition (`22A` concl-2): the primary
message is fact-plane (artifact-eligible); the help is admin-facing remediation (CLI
only). One crib enforces two of our contract lines at once.

`crib-4 (severity declared as data, the registry not the call site).` +SURE
(`226` §6 sev-2, Clang's tablegen class + TS `category`). rustc lint levels are a
property of the lint, not the emit site; Clang's severity is the *class you
instantiate*. We take "severity lives in the registry keyed by code, never at the
construction site." This kills the ESLint fragmentation failure mode (`226` §6:
per-call-site severity drifts to all-warnings) before it can start.

`crib-5 (the un-overridable floor tier + the `expect`-style positive assertion).`
+SURE (`226` §6, `finding-8`, sev-1/sev-3/sev-4; `22A` concl-8). rustc's
`forbid`/`force-warn` are the un-overridable tiers; `future-incompatible` is a
severity *floor* (warn-or-deny, never off); `expect` is the *inverse-completeness*
assertion ("this site MUST emit X; CI fails when it stops"). We take all three as a
small floor-tier concept on the registry (§3 `type-sketch-4`) plus a DST-composed
must-emit assertion (§4 render/test tail). The remediation-class axis itself is the
*sixth* crib, but it is Dorc-native (ru-6), so it sits in §3 not here.

### From Elm's narrative discipline

`crib-6 (the four-part narrative render: title / region / problem / hint).` +SURE on
the shape, ~SUSPECT on the exact field names being Elm's canonical terms. Elm's
famously-good errors (`226` §7) read as: a **title** (the named problem, e.g.
`TYPE MISMATCH`), a **region** (the source excerpt pointed-at), a **problem**
statement in plain prose, and one or more **hints** (the nudge toward the fix). We
take this as the *CLI render template* (§4): our `code` + `severity` produce the
title; our labeled primary span produces the region; our `message` is the problem;
our help children are the hints. Crucially Elm's render is *narrative* — it reads as
a paragraph a human follows top-to-bottom — not rustc's denser caret-art. For an ops
tool whose users are admins and oracle-authors (not compiler hackers), the
narrative register is the better default; we crib Elm's *tone and ordering*, rustc's
*data model*.

`crib-7 (quality is craft + dogfooding, NOT a gate).` +SURE (`226` §7, Czaplicki
verbatim: world-class diagnostics, zero catalog/golden machinery, sustained by
single-author craft + ecosystem battle-testing). This is a crib of *restraint*: it
tells us the gate (§4) buys regression-safety and multi-author consistency, never
message *quality*, and must never creep toward prose enforcement. It is the positive
case for the §7 friction test — the discipline must stay cheap or it earns its own
downgrade.

### What we explicitly REFUSE

`refuse-1 (rustc's Fluent translation layer + the diagnostic derive DSL).` +SURE,
**the named refusal.** No `.ftl` files, no `#[derive(Diagnostic)]`-style attribute
DSL, no `#[primary_span]`/`#[suggestion(...)]` attribute machinery, no multi-file
edit-the-fluent-file-and-the-errors.rs-and-the-emit-site dance. This is the exact
mechanism that earned the Oct-2024 downgrade (`226` §2). We are also *forced* into
this refusal by `inv-no-unsafe` (proc-macros forbidden workspace-wide) — but we
would refuse it on friction grounds regardless. The replacement is a plain builder +
typed-payload constructors (§3): the *types* do the work the derive-DSL did, with the
stock Rust compiler as the only enforcement engine.

`refuse-2 (translation/i18n entirely, this round).` +SURE. Dorc has one audience
language; the entire translation indirection (the thing that made rustc's authoring
"edit multiple files") buys us nothing and costs the friction that killed it. Message
text is English literals at the construction site, decoupled from the *code* (which
is the stable API surface — `226` finding-6, TS's code-stable/message-free split).
If i18n is ever wanted it is a downstream projection over the structured value, never
an authoring mandate. --WONDER whether even reserving a translation seam is worth it;
I lean no (YAGNI; the structured value is already the seam).

`refuse-3 (Menhir-style derive-the-complete-set + a heavyweight generate/compare
CI step).` +SURE (`226` §12). Menhir must *derive* its error states from the grammar
(an automaton-state enumeration costing tens of seconds + gigabytes, a 64-bit
machine, and a `--merge-errors` tool for collaborative editing). Dorc's give-up
sites are **nameable source points**, not implicit automaton states — a genuine
structural advantage. So we refuse the generate-from-scratch machinery and take the
cheap half: an exhaustive Rust `enum` (the compiler enforces handle-every-variant for
free, `226` §12) + a tidy-style grep for the reachability/docs half the type system
can't see (§4).

`refuse-4 (Elm's NO-catalog pole).` +SURE — we refuse Elm's *absence* of a registry.
Elm is one-language/one-author; Dorc is multi-author infra with a correctness
mandate, where the thing that rots silently across authors is exactly
registration/completeness (`226` §7 design-takeaway). We take Elm's render and
quality-is-craft lesson, but NOT its "skip the catalog" — that is the rustc tidy
gate's job, and it survived precisely because it is cheap and structural.

`refuse-5 (golden-test the rendered prose).` +SURE (`226` §8, golden-2/golden-4).
We do NOT snapshot the narrative render text — that churns on every wording tweak and
re-imposes the churn Elm deliberately refused. We snapshot the *structured value*
(code + payload + span-keys + applicability), which is stable across prose edits.
The prose render is exercised by a handful of human-reviewed examples, not a
wall of goldens. (This also dodges the `--bless` footgun, `226` §8 golden-1: the
review burden is the real cost, so we minimize what needs reviewing.)

## §3 The proposed Rust types

Sketch-level but compilable-looking. All in `core` (the `dac-B` shared vocabulary;
`core/CLAUDE.md`). Honors `inv-no-throw` (constructors return data, never panic),
`inv-determinism` (ordered collections only; no `HashMap` iterated into output),
`inv-no-unsafe` (stock `#[derive]` only). I mark where each contract line lands.

### type-sketch-1 — the catalog enum (exhaustive spine) + typed payloads

The contract: *catalog codes are an exhaustive Rust enum; each variant's PAYLOAD is
typed to demand exactly the objects the diagnostic cites* (the capability instinct,
realized structurally). The code is no longer a string — it is the enum, and the
payload travels WITH it:

```rust
/// The exhaustive catalog of every diagnostic the analyzer can emit. One variant
/// per give-up/disclosure class; the compiler enforces handle-every-code (226 §12).
/// Adding a code is ONE new variant here + ONE registry row (§type-sketch-4) + ONE
/// render arm — the friction test (§7). NO derive DSL: stock derives only.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]                      // forces a wildcard-free match to be deliberate
pub enum DiagCode {
    /// A `$(…)`/runtime-dynamic operand forced a command to ⊤ (runs, never elided).
    CmdsubOperandTop(CmdsubOperandTop),
    /// A probe could not ship a read-only check for this site ⇒ the apply runs it.
    SiteUnresolvable(SiteUnresolvable),
    /// A write-redirect to an unresolved target joined ⊤.
    RedirTargetTop(RedirTargetTop),
    // … the other ~15 retrofitted give-up classes, each with its own payload struct.
}

/// The PAYLOAD is typed to demand exactly the objects this diagnostic cites. A
/// probe-blaming error cannot be constructed without the probe-record handle; a
/// license-adjacent error cannot be constructed without site+license evidence. The
/// capability instinct made structural: you cannot author the diagnostic wrong
/// because you cannot NAME the wrong objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteUnresolvable {
    pub site: SiteId,                  // §type-sketch-5 — first-class site identity
    pub probe: ProbeSiteRef,           // the probe-record handle it blames (plan crate)
    pub source_excerpt: OutClaim,      // referent-agnostic text (inv-referent-agnostic)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdsubOperandTop {
    pub site: SiteId,
    pub position: OperandPosition,     // newtype: `the command word` | `operand N`
    pub cause: Option<ProvId>,         // §type-sketch-6 — the ⊤-origin hook (228 dc-1)
}
```

Why payload-per-variant and not one fat struct: it is the *structural* form of
"a diagnostic cites the right objects." `SiteUnresolvable` literally cannot exist
without a `ProbeSiteRef`; the type system is the capability check. This is what
replaces rustc's `#[derive(Diagnostic)]` attribute machinery — the demand is encoded
in the *field types*, enforced by the stock compiler, no DSL. ~SUSPECT the one cost
to flag to the human: ~15-20 small payload structs is more *types* than today's flat
strings, but each is a 3-5 line stock-derive struct, and the win is that every emit
site is type-checked against its evidence. This is priority-1 (maintainability /
make-illegal-states-unrepresentable) bought at priority-2's expense (slightly more
code) — the trade the project's priority order endorses (`core/CLAUDE.md` newtypes
ethos).

### type-sketch-2 — the Diag value (message + labeled spans + children)

```rust
/// One diagnostic, ready to render or to ride the OOB lane. The structured shape
/// cribbed from rustc (crib-1): a primary message, a window of labeled spans (one
/// primary, ≥0 secondary — this is how a ⊤-cause-site and a poisoned-site live in
/// ONE diagnostic, 228), and ordered children (notes/helps). Severity is NOT a field
/// — it is looked up from the registry by `code` (crib-4), so it cannot drift per-site.
#[derive(Debug, Clone)]
pub struct Diag {
    pub code: DiagCode,                // carries its typed payload (type-sketch-1)
    pub primary: SpanLabel,            // the one span the region renders around
    pub secondary: Vec<SpanLabel>,     // additional labeled spans (cause-site, etc.)
    pub children: Vec<SubDiag>,        // ordered notes/helps (crib-1/crib-3)
    pub suggestion: Option<Suggestion>,// the actionable fix, if any (crib-2)
}

/// A span with an optional label — the rustc primary/secondary-label model (crib-1).
/// Fixes 21Z drop-B: a span is no longer Option-on-the-whole-Diag; the PRIMARY span
/// is mandatory (you cannot author a span-less diagnostic), secondaries are the
/// optional extras. Span-poverty becomes a compile error, not a habit.
#[derive(Debug, Clone)]
pub struct SpanLabel {
    pub span: Span,                    // core::Span (mandatory on the primary)
    pub label: Option<String>,        // "this went ⊤", "first poisoned here" — prose
}

/// A note or help child (crib-1/crib-3). `Help` is remediation-facing (CLI only,
/// fact-plane-exempt); `Note` is additional fact context. The split lets the render
/// model (§4) drop helps from artifact-eligible output while keeping notes.
#[derive(Debug, Clone)]
pub enum SubDiag {
    Note(String),
    Help(String),
}
```

The mandatory-primary-span is the structural fix for `21Z` drop-A/drop-B together:
there is no `Diag` without a primary `Span`, so the CLI render *cannot* drop what was
never optional, and an author *cannot* forget the span because the constructor
demands it (§type-sketch-7).

### type-sketch-3 — suggestion + applicability + remediation-class

```rust
/// An actionable fix. Cribbed from rustc (crib-2): a replacement + a confidence the
/// tooling reads to decide auto-apply. Dorc adds the human-ratified remediation-class
/// (ru-6, 224 §7): WHICH user action clears this. Applicability says how confident;
/// class says what kind. Together they drive the render's grouping AND a future
/// `--fix`-style auto-apply story.
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub message: String,               // "declare nginx's `installed` selector in the oracle"
    pub applicability: Applicability,  // crib-2, verbatim from rustc
    pub remediation: RemediationClass, // ru-6 — the render axis
    // NOTE: no machine-applicable SPAN-EDIT to a shipped .sh artifact this round.
    // A suggestion is admin-facing guidance (CLI), not an artifact rewrite — the
    // artifact stays fact-plane (ru-12). A future `dorc fix` could consume the
    // applicability, but that is out of this round's scope.
}

/// rustc's confidence model, verbatim (crib-2, re-verified live).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    MachineApplicable,                 // auto-apply, preserves meaning
    MaybeIncorrect,                    // valid but uncertain — consult the user
    HasPlaceholders,                   // contains `(…)` holes — cannot auto-apply
    Unspecified,
}

/// The human-ratified render axis (ru-6, 224 §7; 22A arch-2): classify every
/// remediable origin by what USER ACTION clears it, and rank the render by that.
/// The dashboard's four-cause decomposition, generalized per-site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationClass {
    AuthorOracle,        // an oracle must be written/extended (the dev-team author)
    AddDeclaration,      // a missing kind/selector/Query declaration (oracle or book)
    FixBookLine,         // the book line itself is wrong/ambiguous (the admin)
    Structural,          // unmodeled construct — Dorc itself must grow (no user fix)
}
```

`Structural` is the honest "no user action clears this; it's a Dorc limitation"
bucket — load-bearing for not lying to an admin that they can fix a ⊤ that is really
ours. (Exclusion-check, the "other user": `AuthorOracle` speaks to the dev-team
author, `FixBookLine` to the ops admin — the two users `AGENTS.md` insists we keep
separate get separate remediation verbs, and the render can address each in their own
language.)

### type-sketch-4 — the registry: severity-as-data with a floor tier

```rust
/// Per-code declared severity (crib-4) with an un-overridable floor (crib-5). This
/// is the ONLY place severity is decided — never at a construction site. A single
/// `match` keyed on the code's discriminant; adding a code adds one arm (friction
/// test). 22A gate2-ask-1: the HUMAN ratifies which codes are floor-pinned.
#[must_use]
pub fn registry(code: &DiagCode) -> CodeSpec {
    match code {
        DiagCode::SiteUnresolvable(_) =>
            CodeSpec { severity: Severity::Note, floor: Floor::None },
        DiagCode::CmdsubOperandTop(_) =>
            CodeSpec { severity: Severity::Note, floor: Floor::None },
        // an Error-class give-up that must NEVER be silenced below a warning:
        // DiagCode::SomeCorrectnessGiveUp(_) =>
        //     CodeSpec { severity: Severity::Error, floor: Floor::WarnOrDeny },
        // … exhaustive: the compiler forces a row per variant (226 §12).
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeSpec { pub severity: Severity, pub floor: Floor }

/// The un-overridable floor (crib-5; rustc future-incompatible = a floor, not a level).
/// `WarnOrDeny` = an admin/oracle MAY raise to Error but may NEVER drop below Warning.
/// `Pinned` = the rustc `forbid`/`force-warn` analog: exactly this severity, no override.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Floor { None, WarnOrDeny, Pinned }
```

Note `Severity` itself I'd extend with `Help` (rustc has it; our `SubDiag::Help`
needs no top-level severity, so this is optional — flagged as `fork-5`). The
floor concept is the anti-fragmentation guarantee: even when admin override lands
(not this round), the floor-pinned codes cannot be silenced — exactly the few-chosen
non-negotiables rustc's `forbid` protects (`226` sev-1).

### type-sketch-5 — first-class site identity (the slot, not the fleet machinery)

The contract: *site identity is first-class (the `site N.M` keying; the hierarchical
fine/coarse grouping keys from `228` §2) — design the slot, don't build the fleet
machinery.* Today the spike keys probe records by `RecordKey{site: LeafId, member:
Option<u32>}` (`cli/main.rs:400`). I promote that into `core` as the diagnostic's
identity, and design the fine/coarse grouping slot per `228` dc-3 WITHOUT building the
fleet rollup:

```rust
/// A diagnostic's first-class site identity. The `site N.M` keying (member = the
/// in-loop family index, inv-site-keyed-results) is the FINE key. The COARSE key
/// (228 dc-3 / finding-grouping-key-design) is computed on demand for fleet rollup
/// but NOT stored-as-a-fleet-index this round — the slot exists, the machinery does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SiteId {
    pub leaf: LeafId,                  // promoted from cli::RecordKey (plan crate id)
    pub member: Option<u32>,           // site N.M — the loop-family member
}

/// The hierarchical grouping keys (228 dc-3: CodeChecker context-free-v2 + Sentry
/// match-either-hash). The FINE key distinguishes per-host detail (the engineer
/// debugging one host); the COARSE key collapses for fleet aggregation (the admin
/// seeing "one rot, 12 hosts"). Both emitted; neither dropped (the AGENTS exclusion-
/// check — both users served). This is a TRAIT/method slot, not a built subsystem:
/// the fleet rollup that CONSUMES coarse keys is explicitly out of scope this round.
pub trait GroupingKey {
    /// Fine: `(analysis-rule-id, site, whitespace-normalized command text)`.
    fn fine_key(&self) -> FineKey;
    /// Coarse: `(analysis-rule-id, enclosing-structural-scope)` — drops the call-site,
    /// survives code movement, collapses M manifestations of one cause.
    fn coarse_key(&self) -> CoarseKey;
}
```

The two keys are the structural answer to dc-3's four-by-two check: fine serves the
"other user" (engineer, one host), coarse serves the admin (fleet). Crucially I am
*not* building the rollup — the trait names the slot so the value-shape is right when
fleet UI is built, but `22A` arch-2 says "pre-build the seam without committing to
fleet UI." ~SUSPECT `enclosing-structural-scope` needs an analyzer-side notion (the
enclosing function/oracle-decl) that the spike has only partially (`AstId` →
enclosing-decl is computable but not currently surfaced); flagged `fork-2`.

### type-sketch-6 — the ProvId hook (artifact-exempt by construction)

The contract: *receipts/provenance render OUT of shipped artifacts; a diagnostic may
hold ProvId hooks but its artifact-visible parts are fact-plane only.*

```rust
/// An opaque handle into the derivation/provenance arena (22A arch-1: ProvId arena +
/// Top(cause)). A Diag MAY carry these (the ⊤-cause for dedup, 228 dc-1), but they
/// are EXEMPT-plane: the render model (§4) projects them ONLY to CLI/why/dashboard,
/// NEVER to a shipped .sh artifact (ru-12 inversion). The type does not enforce this
/// alone — the RENDER enforces it (§4 render-3) — but keeping ProvId a distinct,
/// opaque, non-Display handle makes "leaked a receipt into an artifact" a visible
/// review smell rather than a silent string concatenation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvId(u32);                // arena index; NOT Display, NOT Into<String>
```

The discipline (`22A` concl-2, the include-by-default partition): a ProvId is *named
exempt* — it is the receipt plane, and the erasability gate asserts the shipped
artifact is byte-identical with and without it. Making `ProvId` deliberately
information-poor (no `Display`, no `Into<String>`) is the §3-analog of rustc's
`ErrorGuaranteed` being information-poor (`226` finding-5): the type can't be
*accidentally* rendered into user text. ~SUSPECT this is necessary-but-not-sufficient
— the actual guarantee is the §4 render partition + the adversarial erasability gate
(`22A` concl-1), which is arch-1's job, not this note's. I flag the seam, I don't
build the gate.

### type-sketch-7 — the builder / constructor API (the friction surface)

This is where the friction test is won or lost. Two layers, both no-DSL:

```rust
// Layer A — the typed constructor (the common case, ≈ today's catalog constructors
// but payload-typed). Mints a Diag with a MANDATORY primary span. This is the
// cheapest authoring path: name the code, hand it its evidence, point at the span.
impl Diag {
    pub fn new(code: DiagCode, primary: Span) -> Self {
        Self { code, primary: SpanLabel { span: primary, label: None },
               secondary: Vec::new(), children: Vec::new(), suggestion: None }
    }
}

// Layer B — fluent (small-f, NOT Fluent-the-i18n-system) chaining for the extras.
// Each method is one obvious call; nothing is mandatory beyond `new`. This is the
// rustc Diag-builder ERGONOMICS (crib-1) without the derive/translation MACHINERY
// (refuse-1). A taste-free author writes `Diag::new(code, span)` and stops; a careful
// author chains a label, a cause, a suggestion — and the GOOD shape was the easy path.
impl Diag {
    #[must_use] pub fn label(mut self, s: impl Into<String>) -> Self { … }      // labels the primary
    #[must_use] pub fn secondary(mut self, span: Span, label: impl Into<String>) -> Self { … }
    #[must_use] pub fn note(mut self, s: impl Into<String>) -> Self { … }
    #[must_use] pub fn help(mut self, s: impl Into<String>) -> Self { … }
    #[must_use] pub fn suggest(mut self, s: Suggestion) -> Self { … }
}
```

The message: there is no `Diagnostic::error`/`warning`/`note` *severity* constructor
anymore (severity is registry data, crib-4). There is exactly one mint —
`Diag::new(code, span)` — and the code's typed payload is constructed inline at the
call site (`DiagCode::SiteUnresolvable(SiteUnresolvable { site, probe, excerpt })`),
which is *where the author already has those objects in scope*. That co-location is
the friction win: the evidence the payload demands is exactly what the give-up site
holds when it gives up.

## §4 The render model

One structured `Diag` value; three lanes; authored once.

### render-1 — the CLI narrative (Elm-style)

The default human surface. Cribs Elm's four-part narrative (crib-6) over rustc's
data (crib-1). Replaces the current flat `report()` (`{stage}: {sev}[{code}]: {msg}`,
span dropped):

```
warning[site-unresolvable]: this command will run on every apply
  --> book.sh:42:3
   |
42 |   wombat reload --config "$dyn"
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no read-only probe could be shipped for this
   |
   = note: the apply runs it unconditionally (kFAIL-perform: when unsure, act)
   = help: declare a probe for `wombat`'s `reloaded` selector in its oracle [author-oracle]
```

Mapping (the authored-once payoff): `severity`+`code` → the **title** line;
`primary.span` resolved to source → the **region**; `primary.label` → the caret
label; `message` (from the code's registry template, payload-filled) → the
**problem**; `children` (notes then helps) → the **hints**; a `Help` derived from
`suggestion` carries its `[remediation-class]` tag inline. The narrative ordering
(problem first, hints last) and plain register are Elm's; the span-caret window is
rustc's. ~SUSPECT the spike's first cut can render a *simpler* form (no caret art —
just `book.sh:42:3: <label>`) and still satisfy every structural requirement; the
caret window is a render-quality refinement, not a data-model one (and per crib-7,
quality is craft, ungated).

### render-2 — the OOB lane (fact-plane, no re-authoring)

The same `Diag` feeds the out-of-band record lane (`21Z`: the probe artifact's
stdout record protocol, `site <key> effect=… rc=…`, parsed into site-keyed
`SiteRecord`s). The Diag's `SiteId` is *already* the lane's key (`type-sketch-5`
promoted `RecordKey` into it) — so a diagnostic and its OOB record share identity for
free. A diagnostic projects to the lane as `{ site, code-discriminant, severity,
fine_key }` — the *fact-plane* fields only (`render-3`). No prose, no help, no
ProvId. The lane grammar grows a `code=` field on the existing site-keyed anchor
(`21Z` arch-4: "the lane's record grammar is where a provenance field rides"). The
author wrote one `Diag`; the lane projection is a pure function of it, not a second
authoring.

### render-3 — the artifact partition (the ru-12 weld, enforced at render)

This is the load-bearing partition. A shipped `.sh` artifact may carry, at most, a
*fact-plane* projection of a diagnostic (a provenance comment naming the elided
original — `21Z`'s `provenance_comment`, which already exists). It may NEVER carry:
the narrative prose, the help/remediation, or any ProvId-derived receipt text. The
enforcement is a single render function whose *type* admits only fact-plane inputs —
the exempt-plane fields (`suggestion`, `children`-helps, `ProvId`) are simply not in
scope for the artifact renderer. The adversarial erasability gate (`22A` concl-1)
asserts byte-identity of the artifact with receipts stripped; that gate is arch-1's,
but the render partition is what makes it *true* by construction. ~SUSPECT the
cleanest realization is two functions over `Diag` — `render_cli(&Diag) -> String`
(everything) and `render_artifact_comment(&Diag) -> Option<String>` (fact-plane,
returns `None` when `comment_safe` refuses — and that refusal's dropped text gets a
carrier on the OOB lane per `21Z`'s genuine-provenance-hole, NOT a silent drop).

### render-4 — a future dashboard (same value, richer projection)

The dashboard (`22A` arch-2) is a third projection: it renders the *full* Diag
including secondary labels, the remediation-class grouping (ru-6: rank/group by
`RemediationClass`), the ⊤-cascade dedup (collapse by `coarse_key`, show the
cause-site once — `228` dc-4: dedup in RENDERING, never destroy receipts at capture),
and the ProvId-resolved derivation trace (exempt-plane, dashboard-only). It is
authored zero times beyond the `Diag` value — it is the richest *consumer*, not a new
*producer*. This is the whole point of the structured value: CLI / OOB / dashboard
are three renders of one authored thing.

### the completeness gate + the must-emit assertion (the tail)

Two mechanisms, both cheap, both from `226`:

- `gate-grep (the tidy-style bidirectional check, 226 §1, refuse-3's cheap half).`
  A plain `cargo test` that (a) extracts the registry's variant set, (b) greps the
  analyzer source for `DiagCode::` construction sites, (c) cross-checks both
  directions (every variant constructed somewhere; every construction site a
  registry variant), (d) `git diff`-guards against silent variant deletion, (e)
  carries a hardcoded, reviewer-visible, self-cleaning allow-list for grandfathered
  gaps. The exhaustive `enum` already gives "every variant handled" for free (the
  `registry` match and every render match won't compile with a variant missing);
  the grep covers the "every variant *reached*" half the type system can't see.
- `must-emit (the rustc `expect` analog, 226 §6 sev-3 + §10 fault-2, composed with
  DST).` A DST scenario that forces each probe/oracle failure seam (via the existing
  DI seams — `226` fault-2: Dorc's harness gives fault-injection for free) and
  asserts the *registered code fires*. This is the positive completeness assertion:
  not just "this code exists" but "this give-up PATH still emits it," CI-failing when
  a refactor silently stops giving up. It reuses the DST machinery; no `fail` crate.

Neither gate touches prose (crib-7 / refuse-5): they police registration and
reachability, never message quality.

## §5 Worked examples (before/after — the friction delta)

Three of today's 17 scattered codes, rewritten in the new API.

### worked-1 — `dq-site-unresolvable` (a catalog Note that already exists, but loses its span today)

**Before** (`core/src/diag.rs`, today — and note it's called with `None` on the
production path per `21Z` drop-B, so the span is absent):

```rust
pub fn site_unresolvable(span: Option<Span>, leaf: &str, source: &str) -> Diagnostic {
    Diagnostic::note(SITE_UNRESOLVABLE, span,
        fill(template(SITE_UNRESOLVABLE), &[("leaf", leaf), ("source", source)]))
}
// emit site: site_unresolvable(None, leaf_str, src_str)  ← span dropped, leaf/source are bare &str
```

**After**:

```rust
// emit site — the payload DEMANDS the probe handle and the site; the span is mandatory:
Diag::new(
    DiagCode::SiteUnresolvable(SiteUnresolvable {
        site: SiteId { leaf, member: None },
        probe: probe_ref,                          // can't construct without it
        source_excerpt: OutClaim(src_sym),         // referent-agnostic, not bare &str
    }),
    site_span,                                     // MANDATORY — drop-A/drop-B impossible
)
.label("no read-only probe could be shipped for this")
.note("the apply runs it unconditionally (kFAIL-perform)")
.suggest(Suggestion {
    message: "declare a probe for this kind's selector in its oracle".into(),
    applicability: Applicability::MaybeIncorrect,
    remediation: RemediationClass::AuthorOracle,
});
```

Friction delta: +1 payload struct (5 lines, stock-derive), but the span can no
longer be silently `None`, `leaf`/`source` are now typed (no bare-`&str` to fumble),
and the diagnostic now carries a remediation-classed suggestion the dashboard can
group — all at the site that already held `probe_ref` and `site_span` in scope.
Severity moved to the registry (one row), so it can't drift.

### worked-2 — `render-heredoc-refused` (today an inline literal — the worst case)

**Before** (`21Z`: "inline literal, not even a named const" — the maximally
taste-free form, free text with a stringly code):

```rust
out.push(Diagnostic::error(
    DiagCode("render-heredoc-refused"),            // a string, unregistered, ungreppable-as-a-type
    Some(span),
    format!("cannot edit leaf {leaf}: its span covers `<<EOF`, not the body"),
));
```

**After**:

```rust
out.push(
    Diag::new(
        DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: SiteId { leaf, member: None } }),
        span,
    )
    .label("this leaf carries a heredoc; its span covers the `<<` opener, not the body")
    .help("split the heredoc body to its own leaf, or mark the kind un-elidable")
    // registry row: RenderHeredocRefused => { severity: Error, floor: WarnOrDeny }
);
```

Friction delta: the *most-improved* case. An anonymous inline literal becomes a
first-class enum variant the compiler tracks, the grep gate sees, and the dashboard
can group — and it gained a remediation help. The message prose is *shorter* at the
site (the boilerplate "cannot edit leaf {leaf}" moves to the registry template), and
the `Error` severity + `WarnOrDeny` floor are declared once, not re-typed per emit.

### worked-3 — a ⊤-cascade pair (`cmdsub-operand-top` with a cause, the 228 payoff)

This shows the multi-span / cause-pointer crib doing real work — the thing today's
flat one-span `Diagnostic` *cannot* express.

**Before**: N independent Notes, one per poisoned downstream consumer, none linked to
the ⊤ origin (`21Z`: "`Reach::Top` is causally opaque"; `228` rq-E: one ⊤ sprays N
notes).

**After** — the ⊤-origin mints ONE diagnostic carrying its cause; downstream
consumers inherit silently (`228` dc-1: emit-at-origin, pure-propagation consumers
never emit):

```rust
// at the ⊤-ORIGIN (the $(…) operand), with a cause handle minted in the ProvId arena:
Diag::new(
    DiagCode::CmdsubOperandTop(CmdsubOperandTop {
        site: SiteId { leaf: origin_leaf, member: None },
        position: OperandPosition::Operand(2),
        cause: Some(cause_id),                     // the ⊤-cause hook (exempt-plane)
    }),
    operand_span,
)
.label("this `$(…)` is unresolvable (⊤)")
.secondary(consumer_span, "and so this command cannot be elided")  // the poisoned site, in ONE diag
.note("downstream commands that depend on this run unconditionally");
// the poisoned consumers do NOT each emit — they inherit the poison silently (228 dc-1)
```

Friction delta: the secondary-span model means one authored diagnostic replaces N
scattered Notes, AND the render can show cause-then-effect in one window. The `cause:
ProvId` is exempt-plane (dashboard-only); the secondary span is fact-plane (it points
at real source). This is the crib-1 multi-span model paying for the `228` dedup
contract directly.

## §6 How it all wires together (the one-paragraph mental model)

A give-up site holds, in scope, the objects it is giving up *on* (a probe ref, a
site, a ⊤-cause). It writes `Diag::new(DiagCode::Variant(payload), span)` — the
payload's field types are the capability check (you can't cite what you don't hold),
the span is mandatory (no span-poverty), and it chains `.label/.note/.help/.suggest`
for the extras the GOOD shape wants. Severity and floor come from the registry by
code, never the site. The one `Diag` value then renders three ways — CLI narrative
(Elm tone, rustc data), OOB lane projection (fact-plane fields only), artifact
comment (fact-plane, receipt-stripped) — plus a future dashboard (full, with
remediation-class grouping and coarse-key dedup). Two cheap gates keep it honest: a
tidy-style grep (registration/reachability) and a DST must-emit assertion (the path
still fires). No DSL, no derive machinery, no translation layer, no multi-file
mandate.

## §7 The friction test, stated and answered honestly

> **friction test:** adding a new diagnostic code is ≈ one edit in one file. No DSL.
> No derive machinery. No multi-file mandate. No proc-macro.

Honest answer: **it is THREE edits in (mostly) ONE file, all in `core`:**
1. one new `enum DiagCode` variant + its payload struct (`core/src/diag.rs`);
2. one `registry()` match arm (same file);
3. one render-template arm (same file, or the render module).

Plus, at the *emit* site (wherever the give-up happens), the one `Diag::new(...)`
call — but that is not "adding the code," that is "using it," and it is exactly one
call where the old code was also exactly one call. The grep gate and (if wired) the
must-emit DST assertion are satisfied *by* writing the emit site, not by extra files.

Is "three arms in one file" honestly ≈ "one edit"? ~SUSPECT yes, and here is the
adversarial check (the thing that would make me wrong): rustc's *downgraded* mandate
was "edit the .ftl file AND errors.rs AND the emit site AND fight the derive DSL's
quirks" — four *distinct files/systems* with *cross-file consistency* the compiler
didn't check. Ours is three arms in *one* file, and the compiler *does* check their
consistency: a missing registry arm or render arm **won't compile** (exhaustive match
on a `#[non_exhaustive]`-free internal enum). So the friction is *bounded and
compiler-guided* — you cannot half-add a code and ship it; the type system walks you
to the missing arms. That is categorically lighter than the Fluent mandate, and it is
the distinction `226` finding-4 draws between the surviving structural gate and the
dying authoring DSL. The residual friction I will NOT hand-wave: ~15-20 payload
structs is real upfront retrofit work (the 17-code migration), and a contributor
adding a code *does* touch three arms — if that ever earns complaints, the escape
valve is a single `Diag::misc(code_str, span, msg)` free-text constructor for
genuinely one-off give-ups (the rustc-`allow` analog), gated so it can't become the
default. I lean against shipping that valve pre-emptively (it's the camel's nose),
but name it as the pressure-relief if the friction test starts failing in practice.

## §8 Open forks for the human (confidence-marked)

- `fork-1 (the big one — payload-struct proliferation vs flat-enum).` ~SUSPECT the
  typed-payload-per-variant (`type-sketch-1`) is the right call (it IS the capability
  instinct made structural, the contract's word), but it is ~15-20 new small structs
  for the 17-code retrofit. The lighter alternative is a flat `enum DiagCode { ... }`
  with NO payload, and the evidence passed *alongside* in `Diag` fields — but that
  loses the "the variant demands its evidence" type-check, which is the whole
  capability point. **Decision needed:** typed payloads (more types, stronger
  guarantee) vs flat enum + side-carried evidence (fewer types, weaker)? My lean:
  typed payloads — it is exactly priority-1 (make-illegal-states-unrepresentable)
  bought at priority-2 (a little more code), the trade your priority order endorses.
  But it's the highest-code-volume decision in the design, so it's yours.

- `fork-2 (enclosing-structural-scope — does the spike have it?).` ~SUSPECT the
  coarse grouping key (`type-sketch-5`, `228` dc-3) wants an "enclosing function /
  oracle-decl identity" the spike computes only partially (`AstId` → enclosing-decl
  is derivable but not currently surfaced as a first-class scope id). **Decision
  needed:** build the enclosing-scope id now (so coarse keys are real), or stub
  `coarse_key()` to fall back to the fine key this round and defer real scope-keying?
  My lean: stub-and-defer — `22A` arch-2 says design the slot, don't build the fleet
  machinery, and the coarse key only *matters* once fleet rollup exists (also not
  this round). The trait slot is the deliverable; a degenerate coarse=fine is honest
  for now.

- `fork-3 (the OOB lane `code=` field — discriminant or slug?).` -GUESS the lane
  projection (`render-2`) needs a stable wire token for the code. A numeric
  discriminant is compact but churns if variants are reordered; a string slug
  (`"site-unresolvable"`) is stable and greppable (the `226` finding-6 code-stable
  principle) but verbose on the wire. **Decision needed:** wire the lane's `code=`
  as a stable string slug (my lean — matches TS/rustc's stable-identifier discipline
  and the OOB lane is not perf-critical, `AGENTS.md` network-dominated) or a numeric
  id? Minor, but it's a wire-format commitment, so flag it.

- `fork-4 (gate2-ask-1 lands here — which codes are floor-pinned?).` This is the
  `22A` gate2-ask-1 the synthesis already routed to you: with the registry as the
  home (`type-sketch-4`), the concrete question is *which of the 17+ codes get
  `Floor::WarnOrDeny` or `Floor::Pinned`*. -GUESS the kFAIL-correctness give-ups
  (a redirect-to-⊤, a heredoc-refusal that would otherwise ship a broken artifact)
  want at least `WarnOrDeny`; the pure disclosures (the `$()`-ran Notes) want
  `Floor::None`. **Decision needed:** ratify the floor column when the retrofit PR
  proposes one — I'd have the builder propose, you dispose, per the contract line.

- `fork-5 (does `Severity` gain a top-level `Help` variant?).` --WONDER. rustc has
  `Help` as a level; our help is a `SubDiag::Help` child, so a top-level `Severity::
  Help` may be redundant. Trivial, but it's a `core` enum touch — flagging only so
  the retrofit doesn't quietly add it without a reason. My lean: no (the child
  covers it).
