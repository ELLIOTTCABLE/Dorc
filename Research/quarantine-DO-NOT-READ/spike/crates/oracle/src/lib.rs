//! `dorc-oracle` — lifts the **fact-centric** oracle contract (design note 162 v2)
//! out of plain sh into the analyzer's internal *kind index*.
//!
//! An oracle file is ordinary sh that declares three things, which we lift
//! statically (never by running it):
//!
//! ```sh
//! oracle_kind=package                          # the named kind this file serves
//! oracle_probe_package() { … dpkg-query … }    # a READ-ONLY check: does package:$1 hold?
//! oracle_effect apt-get install establish      # `apt-get install <pkgs>` establishes package
//! oracle_effect apt-get purge   kill           # `apt-get purge   <pkgs>` removes it
//! ```
//!
//! From a book's bare `apt-get install -y nginx`, the analyzer looks up the
//! effect `(apt-get, install) → (package, Establish)`, and to decide whether it
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

use dorc_core::{AstId, Carrier, DiagCode, Diagnostic, Interner, KindId, ProviderId, Span, Symbol};
use dorc_syntax::ast::{Ast, NodeKind, WordPart};
use std::collections::BTreeMap;

/// What a `(provider, verb)` invocation does to a fact of its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Makes the fact hold (`apt-get install` ⇒ `package:X` present).
    Establish,
    /// Makes the fact not hold (`apt-get purge` ⇒ `package:X` absent).
    Kill,
}

/// A read-only fact-probe for one kind: shippable sh that observes whether
/// `kind:entity` holds for an entity passed as `$1`. By contract it is
/// three-outcome (exit `0` = holds, `1` = absent, `2` = can't-tell) and must not
/// mutate — the latter is the consumer's/runner's obligation to enforce, not this
/// crate's (note 162 DP-4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactProbe {
    pub kind: KindId,
    /// The probe function's sh body, lifted verbatim for shipping to the host.
    pub body: String,
}

/// The analyzer-internal kind index — the dn-1 artifact. A 3-place relation
/// (kind, provider, verb→effect), *not* a 1-place naming convention (which would
/// clobber when two providers coexist; note 162 F-3). Built by [`lift`], queried
/// by the analyses.
#[derive(Debug, Clone, Default)]
pub struct KindIndex {
    /// kind → how to observe it (one probe per kind).
    probes: BTreeMap<KindId, FactProbe>,
    /// (provider, verb) → (kind, polarity). Accumulating + clobber-free: many
    /// providers and many verbs coexist (`apt-get install` vs `apt-get purge` vs
    /// `dpkg -i`), unlike a single `oracle_verb`.
    effects: BTreeMap<(ProviderId, Symbol), (KindId, Polarity)>,
}

impl KindIndex {
    /// Record a kind's probe. Last-writer-wins on a duplicate kind (the lifter
    /// should diagnose duplicates; this stays total).
    pub fn add_probe(&mut self, probe: FactProbe) {
        self.probes.insert(probe.kind, probe);
    }

    /// Record that `provider verb …` has `polarity` on `kind`.
    pub fn add_effect(
        &mut self,
        provider: ProviderId,
        verb: Symbol,
        kind: KindId,
        polarity: Polarity,
    ) {
        self.effects.insert((provider, verb), (kind, polarity));
    }

    /// The read-only probe for `kind`, if any oracle declared one.
    #[must_use]
    pub fn probe_for(&self, kind: KindId) -> Option<&FactProbe> {
        self.probes.get(&kind)
    }

    /// What a book's `provider verb …` does, if any oracle declared it. `None`
    /// means "no oracle knows this" → the consumer treats the command as ⊤ (run).
    #[must_use]
    pub fn effect_of(&self, provider: ProviderId, verb: Symbol) -> Option<(KindId, Polarity)> {
        self.effects.get(&(provider, verb)).copied()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.probes.is_empty() && self.effects.is_empty()
    }
}

/// Diagnostic codes the lifter emits (greppable; `ch-catalog`).
const NON_LITERAL_KIND: DiagCode = DiagCode("oracle-non-literal-kind");
const MISSING_KIND: DiagCode = DiagCode("oracle-missing-kind");
const MISSING_PROBE: DiagCode = DiagCode("oracle-missing-probe");
const BAD_EFFECT: DiagCode = DiagCode("oracle-bad-effect");
const TOP_LEVEL_MUTATOR: DiagCode = DiagCode("oracle-top-level-mutator");
const NON_DECL_CONSTRUCT: DiagCode = DiagCode("oracle-non-declaration");

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
    out.diags.extend(parsed.diags);
    let ast = &parsed.value;

    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return; // parse() always roots a Script; defensive only.
    };

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

/// A lifted but not-yet-kind-bound `oracle_effect`: the provider/verb names and
/// the polarity. Held with the source's lifetime; interned only when a usable
/// kind exists to bind them to.
struct RawEffect<'a> {
    provider: &'a str,
    verb: &'a str,
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

    // `oracle_effect <provider> <verb> <polarity>` — exactly three literal args.
    let literal_args: Option<Vec<&str>> =
        words[1..].iter().map(|&w| word_literal(ast, w)).collect();
    let Some([provider, verb, polarity]) = literal_args.as_deref() else {
        // Either a non-literal arg (collect → None) or the wrong arity (slice
        // pattern fails). Both are the same kind of malformed marker.
        out.push(Diagnostic::error(
            BAD_EFFECT,
            Some(span),
            "oracle_effect takes exactly three literal arguments: \
             <provider> <verb> <establish|kill>",
        ));
        return None;
    };
    let polarity = match *polarity {
        "establish" => Polarity::Establish,
        "kill" => Polarity::Kill,
        other => {
            out.push(Diagnostic::error(
                BAD_EFFECT,
                Some(span),
                format!("oracle_effect polarity must be `establish` or `kill`, not `{other}`"),
            ));
            return None;
        }
    };
    Some(RawEffect {
        provider,
        verb,
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

    match probe_bodies.get(kind_name) {
        Some(&body_id) => {
            let span = ast.node(body_id).span;
            let body = src
                .get(span.lo.0 as usize..span.hi.0 as usize)
                .unwrap_or_default()
                .to_string();
            out.value.add_probe(FactProbe { kind, body });
        }
        None => out.push(Diagnostic::error(
            MISSING_PROBE,
            None,
            format!(
                "oracle_kind=`{kind_name}` has no matching `oracle_probe_{kind_name}` function"
            ),
        )),
    }

    for eff in effects {
        out.value.add_effect(
            ProviderId(interner.intern(eff.provider)),
            interner.intern(eff.verb),
            kind,
            eff.polarity,
        );
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
        [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s),
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
    ) -> Option<(KindId, Polarity)> {
        idx.effect_of(ProviderId(i.intern(provider)), i.intern(verb))
    }

    #[test]
    fn index_lookups_round_trip() {
        // Pins the hand-built index API the consumer relies on (independent of lift).
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");

        let mut idx = KindIndex::default();
        idx.add_probe(FactProbe {
            kind: package,
            body: "dpkg-query -W \"$1\"".into(),
        });
        idx.add_effect(apt, install, package, Polarity::Establish);

        assert_eq!(
            idx.effect_of(apt, install),
            Some((package, Polarity::Establish))
        );
        assert!(idx.probe_for(package).is_some());
        // An unknown (provider, verb) is None ⇒ consumer must run it (⊤), never skip.
        let purge = interner.intern("purge");
        assert_eq!(idx.effect_of(apt, purge), None);
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
            Some((package, Polarity::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "purge"),
            Some((package, Polarity::Kill))
        );
        // A sub-verb flag (`-i`) is just another verb token — no flag grammar here.
        assert_eq!(
            effect(&out.value, &mut i, "dpkg", "-i"),
            Some((package, Polarity::Establish))
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
            &["oracle_kind=package\noracle_effect apt-get install establish\n"],
        );
        assert!(
            out.diags.iter().any(|d| d.code == MISSING_PROBE),
            "{:?}",
            out.diags
        );
        // The effect still binds to the (probeless) kind; the consumer finds no probe
        // and treats it as ⊤. Verify the kind was interned and the effect recorded.
        let package = KindId(i.intern("package"));
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, Polarity::Establish))
        );
        assert!(out.value.probe_for(package).is_none());
    }

    #[test]
    fn malformed_effect_is_skipped_not_fatal() {
        // Wrong arity and an unknown polarity each drop just that one effect, with a
        // diagnostic, while the rest of the file lifts (fail-soft, dn-7).
        let mut i = Interner::default();
        let src = "oracle_kind=package\n\
                   oracle_probe_package() { :; }\n\
                   oracle_effect apt-get install\n\
                   oracle_effect apt-get remove maybe\n\
                   oracle_effect apt-get purge kill\n";
        let out = lift(&mut i, &[src]);
        assert_eq!(
            out.diags.iter().filter(|d| d.code == BAD_EFFECT).count(),
            2,
            "{:?}",
            out.diags
        );
        // The good probe + the good effect survive the two bad markers.
        let package = KindId(i.intern("package"));
        assert!(out.value.probe_for(package).is_some());
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "purge"),
            Some((package, Polarity::Kill))
        );
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            None,
            "dropped (no polarity)"
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
                 oracle_effect apt-get install establish\n";
        let b = "oracle_kind=package\noracle_effect yum install establish\n";
        let mut i = Interner::default();
        let out = lift(&mut i, &[a, b]);
        let package = KindId(i.intern("package"));
        assert_eq!(
            effect(&out.value, &mut i, "apt-get", "install"),
            Some((package, Polarity::Establish))
        );
        assert_eq!(
            effect(&out.value, &mut i, "yum", "install"),
            Some((package, Polarity::Establish))
        );
    }
}
