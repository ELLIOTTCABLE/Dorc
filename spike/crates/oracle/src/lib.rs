//! `dorc-oracle` — lifts the **fact-centric** oracle contract (design note 162 v2)
//! out of plain sh into the analyzer's internal *kind index*.
//!
//! An oracle file is ordinary sh that declares three things, which we lift
//! statically (never by running it):
//!
//! ```sh
//! oracle_kind=package                            # the named kind this file serves
//! oracle_probe_package() { … dpkg-query … }      # a READ-ONLY check: does package:$1 hold?
//! oracle_effect apt-get install establish installed  # establishes package:<pkg>#installed
//! oracle_effect apt-get purge   kill      installed  # removes it (same selector cell)
//! oracle_effect apt-get update  establish fresh      # nullary: package-index#fresh (Singleton)
//! oracle_effect command ''      query     present    # READ-ONLY: observes tool:<x>#present
//! ```
//!
//! From a book's bare `apt-get install -y nginx`, the analyzer looks up the
//! effect `(apt-get, install) → (package, #installed, Establish)`, and to decide whether it
//! is already done it ships the `package` kind's probe. The kind name is the only
//! cross-oracle anchor (apt's `package` ≡ yum's `package`); it is never decoded
//! for meaning.
//!
//! This crate is *lightly typed on purpose* — it is pure declaration-extraction,
//! with no soundness-orientation in play. The heavy orientation-locks
//! (`May`/`Must`, phase-typed verdicts, the skip witness) live downstream in the
//! analyses that consume this, where a wrong direction is catastrophic
//! (note 165). Here, a wrong lift is a missing/garbled `ProviderDecl`, caught by
//! the consumer treating an absent effect as ⊤ (run it), never a silent wrong-skip.

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; this crate-root expect
// ratchets away during the rebuild (an unfulfilled `expect` warns, so it
// self-removes as the seeded layer is replaced). It never relaxes the policy
// for new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::indexing_slicing,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use dorc_core::{
    AstId, Carrier, DiagCode, Diagnostic, Interner, KindId, ProviderId, SelectorId, Span, Symbol,
};
use dorc_syntax::ast::{Ast, NodeKind, WordPart};
use std::collections::{BTreeMap, BTreeSet};

/// The command-keyed `check()` contract (19H §2 / 202 §1 face-check): a dedicated
/// parser for the constrained oracle-contract dialect plus a concrete evaluator that
/// traces a known argv through a check's argparse to its kind-annotation.
///
/// Round-20 input-side mechanism, wired in by task-W: `analysis::effect` threads a
/// book's value-flow through [`check::evaluate`] (the oracle's own argparse) to its
/// inline kind-annotation — the real entity-resolution, replacing the former
/// engine-side argparse stand-in. Coexists with [`lift`]/[`KindIndex`]/[`Polarity`]
/// (the effect-map still supplies selector/polarity per `(provider, verb)`).
pub mod check;

/// What a `(provider, verb)` invocation does to — or *observes about* — a fact of
/// its kind (202 §2). Establish/Kill are MUTATORS; `Query` is the read-only
/// guard-class (`command -v`, `dpkg -s`, `getent`), first-classed in round-20 task-D2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Makes the fact hold (`apt-get install` ⇒ `package:X` present).
    Establish,
    /// Makes the fact not hold (`apt-get purge` ⇒ `package:X` absent).
    Kill,
    /// Reads the fact and mutates NOTHING (`command -v X` ⇒ observes
    /// `tool:X#present`). The read-only guard-class (202 §2 / task-D2): a `Query`
    /// poisons no reaching-defs and establishes nothing, but its check IS the probe
    /// — and its probed rc becomes the guard site's Status channel (gated by
    /// rule-query-validity, 205 §2). The disaster-class asymmetry: a Query site's
    /// record-rc is the guard's OWN rc (fold-usable), unlike a mutator site's
    /// record-rc (the probe-command's rc, never the mutator's — the wrong-concrete
    /// firewall, 20C §2).
    Query,
}

/// A read-only fact-probe for one kind (or one `(kind, selector)` cell): shippable sh
/// that observes whether `kind:entity` holds for an entity passed as `$1`. By contract
/// it is three-outcome (exit `0` = holds, `1` = absent, `2` = can't-tell) and must not
/// mutate — the latter is the consumer's/runner's obligation to enforce, not this
/// crate's (note 162 DP-4).
///
/// task-P / find-1 (`strain-D1-perselector`, F-BLESSED): a probe may be declared
/// **per kind** (`oracle_probe_<kind>`, the kind-default) or **per `(kind, selector)`**
/// (`oracle_probe_<kind>_<selector>`). The latter is what an honest `service` oracle
/// needs — `is-enabled` discharges `#enabled`, `is-active` discharges `#active`, and a
/// single body cannot soundly observe both ("an honest service probe is two commands").
/// The resolution rule lives in [`KindIndex::resolve_probe`]; this struct only carries
/// the lifted body and the cell coordinate the lift bound it to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactProbe {
    pub kind: KindId,
    /// The probe function's sh body, lifted verbatim for shipping to the host.
    pub body: String,
}

/// One declared effect cell of a `(provider, verb)`: which `kind`, which `selector`
/// facet, and the `polarity`. A `(provider, verb)` may declare **several** cells
/// (`us-effectmap`, note 205 §3: a multi-cell verb is real — `purge` kills
/// `#installed` and may dirty a `#config` cell). The wiring (`analysis::effect`)
/// treats each cell as written, in declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectCell {
    pub kind: KindId,
    pub selector: SelectorId,
    pub polarity: Polarity,
}

/// A duplicate-effect conflict (`us-effectmap`, note 205 §3): a *second*
/// `oracle_effect` for the same `(provider, verb)` on the **same** `selector` cell.
/// First-writer-wins (the duplicate is dropped); the lifter turns this into a loud
/// [`Diagnostic`]. A different *selector* for the same verb is NOT a conflict — that
/// is the legitimate multi-cell case ([`EffectCell`]).
#[derive(Debug, Clone, Copy)]
pub struct EffectConflict {
    pub provider: ProviderId,
    pub verb: Symbol,
    pub selector: SelectorId,
}

/// The analyzer-internal kind index — the dn-1 artifact. A 3-place relation
/// (kind, provider, verb→effect), *not* a 1-place naming convention (which would
/// clobber when two providers coexist; note 162 F-3). Built by [`lift`], queried
/// by the analyses.
#[derive(Debug, Clone, Default)]
pub struct KindIndex {
    /// kind → the kind-default probe (`oracle_probe_<kind>`). Used by
    /// [`resolve_probe`](KindIndex::resolve_probe) for a site ONLY when the kind has
    /// exactly one declared selector in the effect-map (the sound floor — a
    /// kind-default cannot soundly stand in for one of several distinct selectors;
    /// `strain-D1-perselector`/F-BLESSED).
    probes: BTreeMap<KindId, FactProbe>,
    /// `(kind, selector)` → a per-selector probe (`oracle_probe_<kind>_<selector>`),
    /// task-P/find-1. When present it ALWAYS wins for that exact cell; it is the only
    /// sound probe for a multi-selector kind (`service#enabled` via `is-enabled`,
    /// `#active` via `is-active`).
    selector_probes: BTreeMap<(KindId, SelectorId), FactProbe>,
    /// (provider, verb) → the declared effect cells. Accumulating + clobber-free:
    /// many providers and many verbs coexist (`apt-get install` vs `apt-get purge`
    /// vs `dpkg -i`), unlike a single `oracle_verb`. The value is a **Vec** of
    /// [`EffectCell`]s (`us-effectmap`, note 205 §3): a verb may gate several cells.
    /// The `selector` (`#installed`/`#fresh`/`#enabled`/`#active`) is the per-entity
    /// facet the spike-2 re-key added (`an-per-entity-selector`, `notes/193` §4):
    /// `enable` and `start` target *different* selectors on the same `service` cell,
    /// so neither discharges the other. A **verbless** provider (`useradd`,
    /// `command -v`) keys on the ε-verb ([`empty_verb`]) — the check binds no verb,
    /// so the wiring looks up `(provider, ε)` (202 §2 / task-W §4).
    effects: BTreeMap<(ProviderId, Symbol), Vec<EffectCell>>,
}

impl KindIndex {
    /// Record a kind's **kind-default** probe (`oracle_probe_<kind>`). Last-writer-wins
    /// on a duplicate kind (the lifter should diagnose duplicates; this stays total).
    pub fn add_probe(&mut self, probe: FactProbe) {
        self.probes.insert(probe.kind, probe);
    }

    /// Record a **per-`(kind, selector)`** probe (`oracle_probe_<kind>_<selector>`,
    /// task-P/find-1). Last-writer-wins on a duplicate cell.
    pub fn add_selector_probe(&mut self, selector: SelectorId, probe: FactProbe) {
        self.selector_probes.insert((probe.kind, selector), probe);
    }

    /// Record that `provider verb …` has `polarity` on `kind`'s `selector` cell.
    /// Returns `Some(EffectConflict)` if a cell on the **same** `(provider, verb,
    /// selector)` was already declared — first-writer-wins, the duplicate is
    /// dropped, and the caller diagnoses (`us-effectmap`, note 205 §3). A *different*
    /// selector for the same verb is appended (the legitimate multi-cell case).
    pub fn add_effect(
        &mut self,
        provider: ProviderId,
        verb: Symbol,
        kind: KindId,
        selector: SelectorId,
        polarity: Polarity,
    ) -> Option<EffectConflict> {
        let cells = self.effects.entry((provider, verb)).or_default();
        if cells.iter().any(|c| c.selector == selector) {
            return Some(EffectConflict {
                provider,
                verb,
                selector,
            });
        }
        cells.push(EffectCell {
            kind,
            selector,
            polarity,
        });
        None
    }

    /// The **kind-default** read-only probe for `kind`, if any oracle declared one
    /// (`oracle_probe_<kind>`). Prefer [`resolve_probe`](KindIndex::resolve_probe) at a
    /// real site — it applies the per-selector resolution rule. This raw accessor is
    /// kept for the unit/DST tests that pin the kind-default body directly.
    #[must_use]
    pub fn probe_for(&self, kind: KindId) -> Option<&FactProbe> {
        self.probes.get(&kind)
    }

    /// The distinct selectors any oracle declared an effect cell for, under `kind`.
    /// Drives the [`resolve_probe`](KindIndex::resolve_probe) single-selector floor:
    /// a kind-default may stand in only when this set has exactly one member (a
    /// kind with one selector has nothing for the default to mis-observe).
    #[must_use]
    pub fn selectors_for_kind(&self, kind: KindId) -> BTreeSet<SelectorId> {
        self.effects
            .values()
            .flatten()
            .filter(|c| c.kind == kind)
            .map(|c| c.selector)
            .collect()
    }

    /// Resolve the read-only probe a SITE on `(kind, selector)` ships (task-P/find-1,
    /// the F-BLESSED sound floor — `strain-D1-perselector`):
    ///
    /// 1. the **per-`(kind, selector)`** probe if declared (`oracle_probe_<kind>_<selector>`)
    ///    — it always wins, and is the only sound probe for a multi-selector kind;
    /// 2. else the **kind-default** (`oracle_probe_<kind>`) ONLY IF the effect-map
    ///    declares exactly ONE selector for that kind. With one selector the default
    ///    has nothing to mis-discharge; with several, a single body cannot soundly
    ///    observe a specific cell (an `is-active` verdict must not satisfy an unmet
    ///    `#enabled`), so the site is **un-probeable** (returns `None` ⇒
    ///    `compile_probe` records it `skip-unresolvable` ⇒ the apply RUNS it —
    ///    `kFAIL-perform`).
    ///
    /// `None` is always the safe direction (`can't-probe ⇒ can't-elide`). The selector
    /// floor is intentionally a STRUCTURAL gate, not a semantic one: Dorc cannot read
    /// whether `is-active` "means" `#enabled` (`inv-referent-agnostic`); it can only
    /// observe that a multi-selector kind needs more than one probe body declared.
    #[must_use]
    pub fn resolve_probe(&self, kind: KindId, selector: SelectorId) -> Option<&FactProbe> {
        if let Some(probe) = self.selector_probes.get(&(kind, selector)) {
            return Some(probe);
        }
        if self.selectors_for_kind(kind).len() == 1 {
            return self.probes.get(&kind);
        }
        None
    }

    /// The declared effect cells of a book's `provider verb …`, if any oracle
    /// declared it (the empty slice when not). Each cell is one `(kind, selector,
    /// polarity)` the verb gates; the wiring treats each as written. A verbless
    /// command keys on `(provider, ε)` ([`empty_verb`]). An empty result means "no
    /// oracle knows this" → the consumer treats the command as ⊤ (run).
    #[must_use]
    pub fn effect_of(&self, provider: ProviderId, verb: Symbol) -> &[EffectCell] {
        self.effects
            .get(&(provider, verb))
            .map_or(&[], Vec::as_slice)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.probes.is_empty() && self.selector_probes.is_empty() && self.effects.is_empty()
    }
}

/// The ε-verb symbol: the effect-map key for a **verbless** provider (`useradd
/// deploy`, `command -v nginx` — the oracle's check binds no verb). Interned as the
/// empty string, which no real argv verb token can be, so it never collides with a
/// declared verb. The `oracle_effect` grammar spells it `''` (an empty single-quoted
/// word; see [`lift_command`]), and the wiring maps a check's `verb: None` to this
/// same symbol (202 §2 / task-W §4 — one shared spelling, both sides).
#[must_use]
pub fn empty_verb(interner: &mut Interner) -> Symbol {
    interner.intern("")
}

/// Diagnostic codes the lifter emits (greppable; `ch-catalog`).
const NON_LITERAL_KIND: DiagCode = DiagCode("oracle-non-literal-kind");
const MISSING_KIND: DiagCode = DiagCode("oracle-missing-kind");
const MISSING_PROBE: DiagCode = DiagCode("oracle-missing-probe");
const BAD_EFFECT: DiagCode = DiagCode("oracle-bad-effect");
const TOP_LEVEL_MUTATOR: DiagCode = DiagCode("oracle-top-level-mutator");
const NON_DECL_CONSTRUCT: DiagCode = DiagCode("oracle-non-declaration");
const DUPLICATE_EFFECT: DiagCode = DiagCode("oracle-duplicate-effect");
/// A `oracle_probe_<kind>_<selector>` whose selector funcname-segment cannot
/// round-trip through the hyphen↔underscore mangling (a selector name carrying a
/// literal `_` is inexpressible; `tc-perselector-mangle`).
const PROBE_SELECTOR_ROUNDTRIP: DiagCode = DiagCode("oracle-probe-selector-roundtrip");

/// Map an interned kind/selector name to its **function-name segment** form: `-` → `_`
/// (`package-index` ⇒ `package_index`). The inverse direction of
/// [`check::map_provider_name`] (`_` → `-`), and the shared home of the
/// hyphen↔underscore convention on the *emit*/match side (task-P: the per-selector
/// probe funcname `oracle_probe_<kind>_<selector>` and the shipped wrapper name both
/// route through here, so the two sides agree).
///
/// **Lossy** in the same way `map_provider_name` is: a literal `_` in the name is
/// indistinguishable from a hyphen after the round-trip. The caller round-trip-checks
/// and flags ([`PROBE_SELECTOR_ROUNDTRIP`]).
#[must_use]
pub fn to_funcname_segment(name: &str) -> String {
    name.replace('-', "_")
}

/// Lift a set of oracle sh sources into the kind index, interning kind/provider/
/// verb names through the shared `interner` (so they match the names the book
/// analysis interns). Never panics (`inv-no-throw`): a non-literal anchor, a
/// missing probe, or a top-level mutator in an oracle file yields a `Diagnostic`,
/// not a crash, and that file simply contributes no (or partial) declarations.
///
/// Deterministic (`inv-determinism`): sources are walked in argument order, the
/// index is `BTreeMap`-backed, and nothing here touches clock/RNG/IO. Each source
/// is parsed by [`dorc_syntax::parse`] and only its *top-level* items are examined
/// — an oracle file is, by contract (note 162 O-6), nothing but the `oracle_kind`
/// assignment, the `oracle_probe_<kind>` function, `oracle_effect` markers, and
/// plain helper assignments/functions. Anything else at the top level (a stray
/// mutator, a control-flow construct, a ⊤-rejected node) is `inv-top-reject`'d
/// loudly as a diagnostic.
#[must_use]
pub fn lift(interner: &mut Interner, oracle_sources: &[&str]) -> Carrier<KindIndex> {
    let mut out = Carrier::pure(KindIndex::default());
    for src in oracle_sources {
        lift_one(interner, src, &mut out);
    }
    out
}

/// Lift a single oracle source into `out` (index + diagnostics), in two passes:
/// first scan the top-level items (recording the declared kind, the
/// `oracle_probe_*` bodies, and the effects); then resolve the one probe whose
/// suffix matches the declared kind. The split is needed because `oracle_kind=`
/// may follow the probe `funcdef` in source order, so we cannot resolve the probe
/// until the whole file is scanned.
fn lift_one(interner: &mut Interner, src: &str, out: &mut Carrier<KindIndex>) {
    let parsed = dorc_syntax::parse(src);
    let ast = &parsed.value;

    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return; // parse() always roots a Script; defensive only.
    };

    // A `<provider>__check` funcdef body is the COMMAND-KEYED dialect (203 §4) — NOT
    // book sh — so the book parser legitimately ⊤-rejects its `while`/`case`. Those
    // funcdefs are `check::lift_checks`' domain, not this lifter's (we ignore them
    // below), so suppress parse diagnostics whose span falls inside one: surfacing
    // them would falsely flag a well-formed oracle file (the `__check` bodies are
    // owned by a different front-end). A parse error OUTSIDE a `__check` body still
    // surfaces (it is real book-level malformedness).
    let check_spans: Vec<Span> = items
        .iter()
        .filter_map(|&item| match &ast.node(item).kind {
            NodeKind::FuncDef { name, .. } if name.ends_with("__check") => {
                Some(ast.node(item).span)
            }
            _ => None,
        })
        .collect();
    let inside_check = |span: Span| {
        check_spans
            .iter()
            .any(|cs| span.lo.0 >= cs.lo.0 && span.hi.0 <= cs.hi.0)
    };
    out.diags
        .extend(parsed.diags.into_iter().filter(|d| match d.span {
            Some(s) => !inside_check(s),
            None => true,
        }));

    // Scan pass: gather the kind anchor, probe bodies (by name suffix), and raw
    // effect tuples. Binding to a `KindId` is deferred to a second pass because
    // `oracle_kind=` may appear *after* the probe/effects in source order, and
    // because effects are kind-dependent — they are dropped if the kind is unusable.
    let mut declared_kind: Option<&str> = None;
    let mut probe_bodies: BTreeMap<&str, AstId> = BTreeMap::new();
    let mut effects: Vec<RawEffect<'_>> = Vec::new();

    for &item in items {
        let node = ast.node(item);
        match &node.kind {
            NodeKind::Simple { assigns, words, .. } if words.is_empty() => {
                scan_kind_assigns(ast, assigns, &mut declared_kind, out);
            }
            NodeKind::Simple { words, .. } => {
                if let Some(eff) = lift_command(ast, words, node.span, out) {
                    effects.push(eff);
                }
            }
            NodeKind::FuncDef { name, body, .. } => {
                if let Some(suffix) = name.strip_prefix("oracle_probe_") {
                    // Last-writer-wins on a duplicate suffix; one kind per file is
                    // the contract, so duplicates are pathological and rare.
                    probe_bodies.insert(suffix, *body);
                }
                // A helper function with any other name is allowed and ignored.
            }
            NodeKind::Unsupported { .. } => out.push(Diagnostic::error(
                NON_DECL_CONSTRUCT,
                Some(node.span),
                "oracle file has an unsupported top-level construct (⊤-rejected)",
            )),
            _ => out.push(Diagnostic::error(
                NON_DECL_CONSTRUCT,
                Some(node.span),
                "oracle file has a non-declaration top-level construct \
                 (only assignments, oracle_* markers, and function defs are allowed)",
            )),
        }
    }

    bind(
        interner,
        src,
        ast,
        declared_kind,
        &probe_bodies,
        &effects,
        out,
    );
}

/// A lifted but not-yet-kind-bound `oracle_effect`: the provider/verb names, the
/// `selector` cell name, and the polarity. Held with the source's lifetime;
/// interned only when a usable kind exists to bind them to.
struct RawEffect<'a> {
    provider: &'a str,
    verb: &'a str,
    selector: &'a str,
    polarity: Polarity,
}

/// Scan an assignment-only command's `assigns` for the `oracle_kind` anchor,
/// recording its literal value (or diagnosing a non-literal one). A non-`oracle_kind`
/// assignment is a benign helper var and is ignored.
fn scan_kind_assigns<'a>(
    ast: &'a Ast,
    assigns: &[AstId],
    declared_kind: &mut Option<&'a str>,
    out: &mut Carrier<KindIndex>,
) {
    for &a in assigns {
        let NodeKind::Assign {
            name,
            value,
            name_span,
        } = &ast.node(a).kind
        else {
            continue;
        };
        if name != "oracle_kind" {
            continue;
        }
        match value.map(|v| &ast.node(v).kind) {
            Some(NodeKind::Word { parts }) if parts_literal(parts).is_some() => {
                *declared_kind = parts_literal(parts);
            }
            _ => out.push(Diagnostic::error(
                NON_LITERAL_KIND,
                Some(*name_span),
                "oracle_kind must be a single literal (e.g. `oracle_kind=package`); \
                 a variable/substitution value cannot be lifted",
            )),
        }
    }
}

/// Classify a top-level command word-list. An `oracle_effect <provider> <verb>
/// <polarity>` marker returns a [`RawEffect`]; anything else is a real,
/// statically-named command — at an oracle's top level a mutator — and is
/// `inv-top-reject`'d as a diagnostic (returning `None`).
fn lift_command<'a>(
    ast: &'a Ast,
    words: &[AstId],
    span: Span,
    out: &mut Carrier<KindIndex>,
) -> Option<RawEffect<'a>> {
    if words.first().and_then(|&w| word_literal(ast, w)) != Some("oracle_effect") {
        // The parser ⊤-rejects dynamic command names, so a `Simple` with words is a
        // real, statically-named command. At an oracle's top level that is a mutator.
        out.push(Diagnostic::error(
            TOP_LEVEL_MUTATOR,
            Some(span),
            "oracle file has a top-level mutator (an oracle file must be only \
             assignments, oracle_* markers, and function defs)",
        ));
        return None;
    }

    // `oracle_effect <provider> <verb> <polarity> <selector>` — exactly four
    // literal args. The 4th (selector) is the spike-2 re-key's per-entity facet
    // (`ch-shape-anno` / `an-per-entity-selector`): which cell of the kind this
    // verb gates (`#installed`, `#fresh`, `#enabled`, `#active`). A **verbless**
    // provider (`useradd`, `command -v`) spells `<verb>` as `''` (an empty
    // single-quoted word) — it interns to the ε-verb ([`empty_verb`]), the key the
    // wiring uses when a check binds no verb (202 §2 / task-W §4).
    let literal_args: Option<Vec<&str>> =
        words[1..].iter().map(|&w| word_literal(ast, w)).collect();
    let Some([provider, verb, polarity, selector]) = literal_args.as_deref() else {
        // Either a non-literal arg (collect → None) or the wrong arity (slice
        // pattern fails). Both are the same kind of malformed marker.
        out.push(Diagnostic::error(
            BAD_EFFECT,
            Some(span),
            "oracle_effect takes exactly four literal arguments: \
             <provider> <verb> <establish|kill> <selector>",
        ));
        return None;
    };
    let polarity = match *polarity {
        "establish" => Polarity::Establish,
        "kill" => Polarity::Kill,
        "query" => Polarity::Query,
        other => {
            out.push(Diagnostic::error(
                BAD_EFFECT,
                Some(span),
                format!(
                    "oracle_effect polarity must be `establish`, `kill`, or `query`, not `{other}`"
                ),
            ));
            return None;
        }
    };
    Some(RawEffect {
        provider,
        verb,
        selector,
        polarity,
    })
}

/// Second pass: bind the declared kind into the index (its probe + its effects).
/// Effects are kind-dependent, so with no usable kind they are dropped (the
/// consumer then sees no effect and treats the book command as ⊤ → runs it, the
/// safe direction). A declared kind with no matching `oracle_probe_<kind>` is a
/// contract violation; a probe/effect with no `oracle_kind=` likewise.
fn bind(
    interner: &mut Interner,
    src: &str,
    ast: &Ast,
    declared_kind: Option<&str>,
    probe_bodies: &BTreeMap<&str, AstId>,
    effects: &[RawEffect<'_>],
    out: &mut Carrier<KindIndex>,
) {
    let Some(kind_name) = declared_kind else {
        // Diagnose only if the file *tried* to be an oracle (declared a probe or an
        // effect); a plain/empty file contributes nothing, silently.
        if !probe_bodies.is_empty() || !effects.is_empty() {
            out.push(Diagnostic::error(
                MISSING_KIND,
                None,
                "oracle file declares oracle_probe_*/oracle_effect but no `oracle_kind=<kind>`",
            ));
        }
        return;
    };

    let kind = KindId(interner.intern(kind_name));

    // Classify each `oracle_probe_*` funcdef against THIS file's declared kind. The
    // funcname kind-segment is the kind name in funcname form (`-` → `_`); a suffix
    // equal to it is the kind-default, a suffix of `<kind-seg>_<rest>` is the
    // per-selector probe for selector `<rest>` (mapped back `_` → `-`), and anything
    // else is an unrelated helper-probe (ignored — a different kind's name, say).
    let kind_seg = to_funcname_segment(kind_name);
    let mut saw_kind_default = false;
    for (&suffix, &body_id) in probe_bodies {
        let body = || {
            let span = ast.node(body_id).span;
            src.get(span.lo.0 as usize..span.hi.0 as usize)
                .unwrap_or_default()
                .to_string()
        };
        if suffix == kind_seg {
            saw_kind_default = true;
            out.value.add_probe(FactProbe { kind, body: body() });
        } else if let Some(sel_seg) = suffix
            .strip_prefix(&kind_seg)
            .and_then(|r| r.strip_prefix('_'))
            .filter(|s| !s.is_empty())
        {
            // Per-selector probe. Map the funcname segment back to the selector name
            // through the SAME hyphen↔underscore convention providers use
            // (`map_provider_name`: `_` → `-`). A funcname segment is `[A-Za-z0-9_]`
            // (no hyphen), so this direction always round-trips; the *un*-expressible
            // case (a selector NAME carrying a literal `_`) is detected against the
            // effect-map selectors below (`tc-perselector-mangle`), where the real
            // selector names live.
            let selector = SelectorId(interner.intern(&check::map_provider_name(sel_seg)));
            out.value
                .add_selector_probe(selector, FactProbe { kind, body: body() });
        }
        // else: an `oracle_probe_*` whose suffix is neither this kind nor a
        // `<kind>_<selector>` — a benign mismatched helper; ignored (one kind per file).
    }
    // A declared kind with NO kind-default AND no per-selector probe is incomplete
    // (the index would have an effect on an un-observable kind). A kind-default-less
    // file that DOES ship per-selector probes is legal (the F-BLESSED service shape):
    // diagnose only when neither form is present.
    if !saw_kind_default && !out.value.selector_probes.keys().any(|(k, _)| *k == kind) {
        out.push(Diagnostic::error(
            MISSING_PROBE,
            None,
            format!(
                "oracle_kind=`{kind_name}` has no matching `oracle_probe_{kind_name}` \
                 (nor any `oracle_probe_{kind_seg}_<selector>`) function"
            ),
        ));
    }

    for eff in effects {
        let provider = ProviderId(interner.intern(eff.provider));
        let verb = interner.intern(eff.verb);
        let selector = SelectorId(interner.intern(eff.selector));
        // tc-perselector-mangle (task-P): a selector NAME carrying a literal `_` cannot
        // be addressed by a per-selector probe funcname — `oracle_probe_<kind>_foo_bar`
        // maps to selector `foo-bar`, never `foo_bar` (the funcname mapping is `_`↔`-`).
        // Flag it WARNING (the cell is still usable via the kind-default; only its
        // per-selector probe is unreachable). No corpus selector has a `_`, so this is a
        // latent-footgun guard, not a live path.
        if check::map_provider_name(&to_funcname_segment(eff.selector)) != eff.selector {
            out.push(Diagnostic::warning(
                PROBE_SELECTOR_ROUNDTRIP,
                None,
                format!(
                    "selector `{}` contains an underscore, which cannot round-trip a \
                     per-selector probe funcname (`oracle_probe_{kind_seg}_…` maps `_`→`-`); \
                     this cell can only be probed by the kind-default",
                    eff.selector
                ),
            ));
        }
        if let Some(_conflict) = out
            .value
            .add_effect(provider, verb, kind, selector, eff.polarity)
        {
            // us-effectmap (note 205 §3): a second oracle_effect on the SAME
            // (provider, verb, selector) cell is a footgun — loud diagnostic,
            // first-writer-wins. A different selector for the same verb is the
            // legitimate multi-cell case and is NOT diagnosed.
            out.push(Diagnostic::error(
                DUPLICATE_EFFECT,
                None,
                format!(
                    "duplicate oracle_effect for (`{}`, verb=`{}`) on selector `{}` \
                     — first declaration wins, this one is dropped",
                    eff.provider, eff.verb, eff.selector
                ),
            ));
        }
    }
}

/// The single-literal text of a word node, if it is one. Returns `None` for a
/// multi-part / expanding word (which is not a statically-fixed token).
fn word_literal(ast: &Ast, id: AstId) -> Option<&str> {
    match &ast.node(id).kind {
        NodeKind::Word { parts } => parts_literal(parts),
        _ => None,
    }
}

/// Mirror of `ast::Word::as_literal`, but as a free fn so the returned `&str`
/// borrows from `parts` (lifetime `'a`) rather than from a temporary `Word`
/// wrapper. A word is a statically-fixed token iff it is exactly one unquoted-
/// or single-quoted literal part.
fn parts_literal(parts: &[WordPart]) -> Option<&str> {
    match parts {
        [WordPart::Literal(s) | WordPart::SingleQuoted(s)] => Some(s),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Resolve a `(provider, verb)` effect through a fresh interner. Each lift gets
    /// its own interner; we re-intern the same names here to query, relying on the
    /// interner's determinism (equal text ⇒ equal symbol).
    fn effect(
        idx: &KindIndex,
        i: &mut Interner,
        provider: &str,
        verb: &str,
    ) -> Option<(KindId, SelectorId, Polarity)> {
        // The index now holds a Vec of cells per (provider, verb) (`us-effectmap`);
        // these tests model single-cell verbs, so project the sole cell to the old
        // tuple shape. A genuine multi-cell verb is exercised by `multi_cell_verb_*`.
        match idx.effect_of(ProviderId(i.intern(provider)), i.intern(verb)) {
            [] => None,
            [cell, ..] => Some((cell.kind, cell.selector, cell.polarity)),
        }
    }

    #[test]
    fn index_lookups_round_trip() {
        // Pins the hand-built index API the consumer relies on (independent of lift).
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");
        let installed = SelectorId(interner.intern("installed"));

        let mut idx = KindIndex::default();
        idx.add_probe(FactProbe {
            kind: package,
            body: "dpkg-query -W \"$1\"".into(),
        });
        idx.add_effect(apt, install, package, installed, Polarity::Establish);

        assert_eq!(
            idx.effect_of(apt, install),
            &[EffectCell {
                kind: package,
                selector: installed,
                polarity: Polarity::Establish
            }]
        );
        assert!(idx.probe_for(package).is_some());
        // An unknown (provider, verb) is the empty slice ⇒ consumer must run it (⊤).
        let purge = interner.intern("purge");
        assert!(idx.effect_of(apt, purge).is_empty());
    }

    #[test]
    fn lifts_the_package_fixture_cleanly() {
        // The acceptance fixture: a real, fully-formed oracle must lift to a complete
        // index with no errors. This is the end-to-end contract (note 162 v2).
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/package.oracle.sh"
        ));
        let mut i = Interner::default();
        let out = lift(&mut i, &[fixture]);

        assert!(
            !out.has_errors(),
            "fixture must lift error-free: {:?}",
            out.diags
        );
        assert!(
            out.diags.is_empty(),
            "no diagnostics at all on the clean fixture: {:?}",
            out.diags
        );

        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let Some(probe) = out.value.probe_for(package) else {
            panic!("a package probe was lifted");
        };
        assert!(
            probe.body.contains("dpkg-query"),
            "probe body is the verbatim function body (contains its real check): {:?}",
            probe.body
        );

        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, installed, Polarity::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "purge"),
            Some((package, installed, Polarity::Kill))
        );
        // A sub-verb flag (`-i`) is just another verb token — no flag grammar here.
        assert_eq!(
            effect(&out.value, &mut i, "dpkg", "-i"),
            Some((package, installed, Polarity::Establish))
        );
    }

    #[test]
    fn non_literal_kind_is_an_error_not_a_panic() {
        // `oracle_kind=$x` cannot be lifted (W4: we never decode/guess a token's
        // text); it must diagnose, not crash, and yield no kind.
        let mut i = Interner::default();
        let out = lift(&mut i, &["oracle_kind=$x\noracle_probe_x() { :; }\n"]);
        assert!(out.has_errors(), "non-literal oracle_kind must error");
        assert!(out.value.probe_for(KindId(i.intern("x"))).is_none());
    }

    #[test]
    fn top_level_mutator_is_an_error() {
        // An oracle file is declarations only; a bare `apt-get install nginx` at the
        // top level is a mutator and must be rejected loudly (inv-top-reject / O-6).
        let mut i = Interner::default();
        let out = lift(&mut i, &["oracle_kind=package\napt-get install nginx\n"]);
        assert!(
            out.diags.iter().any(|d| d.code == TOP_LEVEL_MUTATOR),
            "a top-level mutator must raise the mutator diagnostic: {:?}",
            out.diags
        );
    }

    #[test]
    fn empty_source_is_empty_index_no_panic() {
        // Totality (inv-no-throw): the degenerate input contributes nothing and is
        // silent — not an oracle, not an error.
        let mut i = Interner::default();
        let out = lift(&mut i, &[""]);
        assert!(out.value.is_empty());
        assert!(
            out.diags.is_empty(),
            "an empty source is not an error: {:?}",
            out.diags
        );
    }

    #[test]
    fn missing_probe_for_declared_kind_errors() {
        // A kind anchor with no matching `oracle_probe_<kind>` is incomplete — the
        // index would have an effect pointing at an un-observable kind. Diagnose it.
        let mut i = Interner::default();
        let out = lift(
            &mut i,
            &["oracle_kind=package\noracle_effect apt-get install establish installed\n"],
        );
        assert!(
            out.diags.iter().any(|d| d.code == MISSING_PROBE),
            "{:?}",
            out.diags
        );
        // The effect still binds to the (probeless) kind; the consumer finds no probe
        // and treats it as ⊤. Verify the kind was interned and the effect recorded.
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, installed, Polarity::Establish))
        );
        assert!(out.value.probe_for(package).is_none());
    }

    #[test]
    fn malformed_effect_is_skipped_not_fatal() {
        // Wrong arity and an unknown polarity each drop just that one effect, with a
        // diagnostic, while the rest of the file lifts (fail-soft, dn-7). Under the
        // 4-token re-key, `install` (2 args) and `remove maybe installed` (bad
        // polarity) are the two bad markers; the good `purge kill installed` survives.
        let mut i = Interner::default();
        let src = "oracle_kind=package\n\
                   oracle_probe_package() { :; }\n\
                   oracle_effect apt-get install\n\
                   oracle_effect apt-get remove maybe installed\n\
                   oracle_effect apt-get purge kill installed\n";
        let out = lift(&mut i, &[src]);
        assert_eq!(
            out.diags.iter().filter(|d| d.code == BAD_EFFECT).count(),
            2,
            "{:?}",
            out.diags
        );
        // The good probe + the good effect survive the two bad markers.
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        assert!(out.value.probe_for(package).is_some());
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "purge"),
            Some((package, installed, Polarity::Kill))
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            None,
            "dropped (wrong arity)"
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "remove"),
            None,
            "dropped (bad polarity)"
        );
    }

    #[test]
    fn multiple_sources_accumulate_deterministically() {
        // dn-1's whole point: many oracle files contribute to one index, in argument
        // order, with no cross-file interference. Two providers, same kind.
        let a = "oracle_kind=package\noracle_probe_package() { :; }\n\
                 oracle_effect apt-get install establish installed\n";
        let b = "oracle_kind=package\noracle_effect yum install establish installed\n";
        let mut i = Interner::default();
        let out = lift(&mut i, &[a, b]);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, installed, Polarity::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "yum", "install"),
            Some((package, installed, Polarity::Establish))
        );
    }

    #[test]
    fn duplicate_effect_same_cell_is_diagnosed_first_wins() {
        // us-effectmap (note 205 §3): a SECOND oracle_effect on the same (provider,
        // verb, selector) cell is a silent-clobber footgun. Now it is a loud
        // DUPLICATE_EFFECT diagnostic and first-writer-wins (the duplicate is dropped).
        // Here `install` is declared establish then (contradictorily) kill on the SAME
        // #installed cell; the establish wins, the kill is dropped + diagnosed.
        let mut i = Interner::default();
        let src = "oracle_kind=package\noracle_probe_package() { :; }\n\
                   oracle_effect apt-get install establish installed\n\
                   oracle_effect apt-get install kill installed\n";
        let out = lift(&mut i, &[src]);
        assert_eq!(
            out.diags
                .iter()
                .filter(|d| d.code == DUPLICATE_EFFECT)
                .count(),
            1,
            "the duplicate same-cell effect must be diagnosed: {:?}",
            out.diags
        );
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, installed, Polarity::Establish)),
            "first-writer-wins: the establish survives, the kill is dropped"
        );
        // Only ONE cell recorded (the duplicate did not append).
        assert_eq!(
            out.value
                .effect_of(ProviderId(i.intern("apt-get")), i.intern("install"))
                .len(),
            1
        );
    }

    #[test]
    fn multi_cell_verb_different_selectors_both_recorded() {
        // us-effectmap (note 205 §3): a verb gating TWO DIFFERENT selectors of its
        // kind is legitimate (NOT a duplicate) — both cells are recorded, no
        // diagnostic. (A hypothetical `apt-get purge` that kills #installed AND dirties
        // a #config cell.) The wiring treats each cell as written.
        let mut i = Interner::default();
        let src = "oracle_kind=package\noracle_probe_package() { :; }\n\
                   oracle_effect apt-get purge kill installed\n\
                   oracle_effect apt-get purge kill configured\n";
        let out = lift(&mut i, &[src]);
        assert!(
            !out.diags.iter().any(|d| d.code == DUPLICATE_EFFECT),
            "distinct selectors for one verb are NOT a duplicate: {:?}",
            out.diags
        );
        let cells = out
            .value
            .effect_of(ProviderId(i.intern("apt-get")), i.intern("purge"));
        assert_eq!(cells.len(), 2, "both selector cells recorded: {cells:?}");
        let selectors: Vec<_> = cells.iter().map(|c| c.selector).collect();
        assert!(selectors.contains(&SelectorId(i.intern("installed"))));
        assert!(selectors.contains(&SelectorId(i.intern("configured"))));
    }

    #[test]
    fn empty_verb_spelling_keys_the_epsilon_verb() {
        // task-W §4: a verbless provider (`useradd`, `command -v`) spells its
        // oracle_effect verb as `''` (empty single-quoted) ⇒ it interns to the ε-verb
        // (`empty_verb`), the key the wiring uses for a check that binds no verb. Pin
        // that the `''` spelling lands on exactly that key (not on a literal "''").
        let mut i = Interner::default();
        let src = "oracle_kind=user\noracle_probe_user() { :; }\n\
                   oracle_effect useradd '' establish present\n";
        let out = lift(&mut i, &[src]);
        assert!(
            !out.has_errors(),
            "the '' (ε-verb) spelling must lift cleanly: {:?}",
            out.diags
        );
        let user = KindId(i.intern("user"));
        let present = SelectorId(i.intern("present"));
        let eps = empty_verb(&mut i);
        let cells = out.value.effect_of(ProviderId(i.intern("useradd")), eps);
        assert_eq!(
            cells,
            &[EffectCell {
                kind: user,
                selector: present,
                polarity: Polarity::Establish
            }],
            "the verbless effect keys on the ε-verb: {cells:?}"
        );
    }

    #[test]
    fn query_polarity_lifts_as_the_read_only_class() {
        // task-D2 (202 §2): `oracle_effect command '' query present` lifts to a
        // Query-polarity cell — the read-only guard-class. Pins the third polarity
        // word and the ε-verb (verbless `command -v`) together (the canonical guard).
        let mut i = Interner::default();
        let src = "oracle_kind=tool\noracle_probe_tool() { :; }\n\
                   oracle_effect command '' query present\n";
        let out = lift(&mut i, &[src]);
        assert!(
            !out.has_errors(),
            "the `query` polarity must lift cleanly: {:?}",
            out.diags
        );
        let tool = KindId(i.intern("tool"));
        let present = SelectorId(i.intern("present"));
        let eps = empty_verb(&mut i);
        let cells = out.value.effect_of(ProviderId(i.intern("command")), eps);
        assert_eq!(
            cells,
            &[EffectCell {
                kind: tool,
                selector: present,
                polarity: Polarity::Query
            }],
            "the verbless guard keys on the ε-verb with Query polarity: {cells:?}"
        );
    }

    #[test]
    fn unknown_polarity_word_is_diagnosed() {
        // The grammar accepts exactly establish/kill/query; anything else drops the
        // marker with a BAD_EFFECT diagnostic (fail-soft, dn-7).
        let mut i = Interner::default();
        let src = "oracle_kind=tool\noracle_probe_tool() { :; }\n\
                   oracle_effect command '' observe present\n";
        let out = lift(&mut i, &[src]);
        assert!(
            out.diags.iter().any(|d| d.code == BAD_EFFECT),
            "an unknown polarity word must raise BAD_EFFECT: {:?}",
            out.diags
        );
    }

    // --- task-P / find-1: per-selector probes + the resolution rule (F-BLESSED) ---

    /// A multi-selector `service` oracle declaring BOTH per-selector probes
    /// (`is-enabled` for `#enabled`, `is-active` for `#active`) — the honest F-BLESSED
    /// shape. `resolve_probe` picks the per-selector body for each cell; the two bodies
    /// are DISTINCT (the find-1 fix — one is-active body could not soundly observe both).
    #[test]
    fn per_selector_probes_resolve_distinct_bodies() {
        let mut i = Interner::default();
        let src = "oracle_kind=service\n\
                   oracle_probe_service_enabled() { systemctl is-enabled --quiet \"$1\"; }\n\
                   oracle_probe_service_active() { systemctl is-active --quiet \"$1\"; }\n\
                   oracle_effect systemctl enable establish enabled\n\
                   oracle_effect systemctl start establish active\n";
        let out = lift(&mut i, &[src]);
        assert!(
            !out.has_errors(),
            "the per-selector shape lifts: {:?}",
            out.diags
        );
        let service = KindId(i.intern("service"));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        let en = out
            .value
            .resolve_probe(service, enabled)
            .expect("#enabled resolves to its per-selector probe");
        let ac = out
            .value
            .resolve_probe(service, active)
            .expect("#active resolves to its per-selector probe");
        assert!(
            en.body.contains("is-enabled"),
            "#enabled ⇒ is-enabled: {}",
            en.body
        );
        assert!(
            ac.body.contains("is-active"),
            "#active ⇒ is-active: {}",
            ac.body
        );
        assert_ne!(
            en.body, ac.body,
            "the two selector probes are distinct bodies"
        );
    }

    /// The F-BLESSED FLOOR: a multi-selector kind whose effect-map declares two
    /// selectors but ships ONLY the kind-default probe is UN-PROBEABLE for both cells
    /// (`resolve_probe` returns `None`) — a single body cannot soundly stand in for one
    /// of several distinct selectors. This is the find-1 under-execute fix: the site
    /// becomes skip-unresolvable ⇒ runs (`kFAIL-perform`).
    #[test]
    fn kind_default_under_multi_selector_kind_is_unprobeable() {
        let mut i = Interner::default();
        let src = "oracle_kind=service\n\
                   oracle_probe_service() { systemctl is-active --quiet \"$1\"; }\n\
                   oracle_effect systemctl enable establish enabled\n\
                   oracle_effect systemctl start establish active\n";
        let out = lift(&mut i, &[src]);
        // The kind-default lifts (no MISSING_PROBE — there IS an oracle_probe_service).
        assert!(!out.has_errors(), "lifts cleanly: {:?}", out.diags);
        let service = KindId(i.intern("service"));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        assert!(
            out.value.resolve_probe(service, enabled).is_none(),
            "multi-selector kind + only kind-default ⇒ #enabled un-probeable (F-BLESSED floor)"
        );
        assert!(
            out.value.resolve_probe(service, active).is_none(),
            "…and #active un-probeable too"
        );
    }

    /// The single-selector escape hatch: a kind whose effect-map declares exactly ONE
    /// selector MAY use the kind-default (it has nothing to mis-discharge). This is what
    /// keeps `package` (one `#installed` selector) probeable from its `oracle_probe_package`.
    #[test]
    fn kind_default_under_single_selector_kind_resolves() {
        let mut i = Interner::default();
        let src = "oracle_kind=package\n\
                   oracle_probe_package() { dpkg-query -W \"$1\" >/dev/null 2>&1; }\n\
                   oracle_effect apt-get install establish installed\n\
                   oracle_effect apt-get purge kill installed\n";
        let out = lift(&mut i, &[src]);
        assert!(!out.has_errors(), "{:?}", out.diags);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let p = out
            .value
            .resolve_probe(package, installed)
            .expect("single-selector kind ⇒ kind-default resolves");
        assert!(p.body.contains("dpkg-query"), "{}", p.body);
    }

    /// A per-selector probe ALWAYS wins, even when the kind is single-selector and a
    /// kind-default also exists (the per-selector is strictly more specific).
    #[test]
    fn per_selector_probe_wins_over_kind_default() {
        let mut i = Interner::default();
        let src = "oracle_kind=package\n\
                   oracle_probe_package() { dpkg-query -W \"$1\" >/dev/null 2>&1; }\n\
                   oracle_probe_package_installed() { dpkg -s \"$1\" >/dev/null 2>&1; }\n\
                   oracle_effect apt-get install establish installed\n";
        let out = lift(&mut i, &[src]);
        assert!(!out.has_errors(), "{:?}", out.diags);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let p = out
            .value
            .resolve_probe(package, installed)
            .expect("resolves");
        assert!(
            p.body.contains("dpkg -s"),
            "the per-selector probe wins over the kind-default: {}",
            p.body
        );
    }

    /// A file shipping ONLY per-selector probes (no `oracle_probe_<kind>` kind-default)
    /// is legal — the canonical F-BLESSED service shape — and does NOT raise `MISSING_PROBE`.
    #[test]
    fn only_per_selector_probes_is_not_missing_probe() {
        let mut i = Interner::default();
        let src = "oracle_kind=service\n\
                   oracle_probe_service_enabled() { systemctl is-enabled --quiet \"$1\"; }\n\
                   oracle_probe_service_active() { systemctl is-active --quiet \"$1\"; }\n\
                   oracle_effect systemctl enable establish enabled\n";
        let out = lift(&mut i, &[src]);
        assert!(
            !out.diags.iter().any(|d| d.code == MISSING_PROBE),
            "per-selector-only is a complete oracle (no kind-default needed): {:?}",
            out.diags
        );
        // The kind-default is genuinely absent; the per-selector ones are present.
        let service = KindId(i.intern("service"));
        assert!(
            out.value.probe_for(service).is_none(),
            "no kind-default body"
        );
        assert!(
            out.value
                .resolve_probe(service, SelectorId(i.intern("enabled")))
                .is_some()
        );
    }

    /// Name-mangling: a hyphenated selector (`needs-restart`) is declared with an
    /// underscore funcname segment (`oracle_probe_service_needs_restart`) and round-trips
    /// to the hyphenated selector name through the shared `map_provider_name` convention.
    #[test]
    fn per_selector_probe_hyphen_underscore_mangling_round_trips() {
        let mut i = Interner::default();
        let src = "oracle_kind=service\n\
                   oracle_probe_service_needs_restart() { needs-restart -- \"$1\"; }\n\
                   oracle_effect systemctl restart establish needs-restart\n";
        let out = lift(&mut i, &[src]);
        assert!(
            !out.has_errors(),
            "the mangled selector lifts: {:?}",
            out.diags
        );
        let service = KindId(i.intern("service"));
        let needs_restart = SelectorId(i.intern("needs-restart"));
        assert!(
            out.value.resolve_probe(service, needs_restart).is_some(),
            "funcname `service_needs_restart` ⇒ selector `needs-restart` (hyphen↔underscore)"
        );
    }

    /// `tc-perselector-mangle`: a selector NAME carrying a literal `_` (declared in the
    /// effect-map) is flagged WARNING — no `oracle_probe_<kind>_<selector>` funcname can
    /// address it (the funcname `_`↔`-` mapping would read it as a hyphenated selector).
    /// The cell still lifts and stays usable via the kind-default; only the diagnostic
    /// surfaces the inexpressibility (latent-footgun guard; no corpus selector has a `_`).
    #[test]
    fn underscore_selector_name_flags_roundtrip_warning() {
        let mut i = Interner::default();
        let src = "oracle_kind=service\n\
                   oracle_probe_service() { systemctl is-active --quiet \"$1\"; }\n\
                   oracle_effect systemctl restart establish needs_restart\n";
        let out = lift(&mut i, &[src]);
        assert!(
            out.diags.iter().any(|d| d.code == PROBE_SELECTOR_ROUNDTRIP),
            "an underscore selector name must raise the round-trip warning: {:?}",
            out.diags
        );
        // It is a WARNING, not an error — the cell still lifts (single-selector kind ⇒
        // the kind-default probes it).
        assert!(
            !out.has_errors(),
            "the round-trip flag is non-fatal (the cell lifts): {:?}",
            out.diags
        );
        let service = KindId(i.intern("service"));
        let needs_restart = SelectorId(i.intern("needs_restart"));
        assert!(
            out.value.resolve_probe(service, needs_restart).is_some(),
            "single-selector kind ⇒ the kind-default still probes the cell"
        );
    }
}
