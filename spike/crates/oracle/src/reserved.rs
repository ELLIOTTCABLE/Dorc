//! `reserved` — the munge-reservation lint (24Kc `fix-munge-reservation`; 24M
//! `ca-munge-charclass`). Three checks over the reserved `<munged>__<role>` sh-function
//! namespace the oracle emitter owns:
//!
//! 1. **charclass** ([`lint_oracle_reserved_names`]) — a role funcdef whose emitted funcname is
//!    not a legal POSIX NAME (a leading-digit provider like `7z`, a dot in a reverse-DNS kind,
//!    a non-ASCII/IDN label) is REFUSED loudly. The real munge failure-mode is character
//!    validity, not length (24M §4b: the name-length errand cleared ~300-char names as safe
//!    everywhere; the munger breaks on *chars*).
//! 2. **collision** ([`lint_oracle_reserved_names`]) — two DISTINCT source names (provider or
//!    kind) that [munge](crate::to_funcname_segment) to the SAME sh NAME (`apt-get`/`apt_get` →
//!    `apt_get__predict`; the non-injective hyphen-munge 24Kc flagged "recorded nowhere") are
//!    REFUSED, never silently merged — the shipped artifact would carry two same-named funcdefs,
//!    last-writer-wins. Aligns with the reingest-collision floor (refuse-and-run).
//! 3. **squat** ([`lint_book_reserved_names`]) — a BOOK funcdef coincidentally named
//!    `<x>__<role>` (the reserved emitted shape) is SURFACED loudly. rul24M-bare-dorcism-names
//!    prices this coincidental-capture-of-an-innocent-function as accepted-not-prevented; this
//!    lint is the standing "loud-friend" mitigation (a captured function misbehaves loudly). Its
//!    live corpus instance is the `guard23-reingest-collision-verbatim` reingest floor, whose
//!    book defines `apt_get__predict`.
//!
//! # Scope at HEAD (the respell has not happened)
//!
//! Dotted kind names (`sm.dorc.Package`, rul24M-reverse-dns-kinds) are NOT yet live — the
//! corpus respell that introduces them, and the dot→`_` transliteration the munger will grow to
//! handle them, are a later pass. What is checkable NOW: [`validate_sh_name`] is the munge-safety
//! primitive the `ca-munge-charclass` ruling demands live "at the munger", and it already REFUSES
//! every invalid shape (dots, leading digits, non-ASCII) the moment such a name reaches the emit
//! boundary — via a leading-digit provider (`7z.predict`) or a reverse-DNS resolver funcdef
//! (`sm.dorc.Package.resolve`) authored today. What is DEFERRED to the respell: whether the
//! munger should TRANSLITERATE those (dot→`_`, per-label leading-digit repair) and ACCEPT them,
//! rather than refuse — a policy decision the respell owns. This lint's job is only to guarantee
//! no broken NAME ships silently (`kFAIL`-safe: refuse over emit-broken).

use dorc_aid::diag::{
    Diag, DiagCode, MungeNameCollision, MungeNameInvalid, ReservedNamespaceSquat,
};
use dorc_core::{Interner, Span};
use std::collections::BTreeMap;

use crate::predict::PredictSet;

/// The reserved role-suffixes of the emitted `<munge>__<role>` function namespace — the six
/// role members (`277` §4d; rul24-ditch-is-diverged removed `is_diverged`): the probe body, the
/// at-most footprint (`disturbs`, né touches), the converged-verdict, the identity canonicalizer,
/// the reach-expander (`disturbance_reaches_only`, né reaches), and the substrate/invariance member
/// (`state_stored_only_in`). KEEP IN SYNC with the parser's per-role suffix ([`crate::predict`]'s
/// `FnRole::mangled_suffix`); the [`suffixes_match_lifted_roles`](tests) test ties the two.
pub const RESERVED_ROLE_SUFFIXES: &[&str] = &[
    "__predict",
    "__disturbs",
    "__is_converged",
    "__resolve",
    "__disturbance_reaches_only",
    "__state_stored_only_in",
];

/// Why a munged name is not a legal POSIX NAME (`XBD §3.216`: a NAME is `[A-Za-z_][A-Za-z0-9_]*`
/// — character-class + no-leading-digit, no length bound). The `ca-strict-set` posture (24M §4b):
/// dash/busybox hold the strict letters/digits/underscore set, so we stay strict for cross-shell
/// safety even though bash/zsh accept extras.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameProblem {
    /// The name is empty (no funcname at all).
    Empty,
    /// The first character is an ASCII digit (`7z` → `7z__predict`; a leading-digit DNS label
    /// like `3com.example.Foo`). Invalid as the first char of a NAME.
    LeadingDigit(char),
    /// A non-ASCII character (an IDN/UTF-8 label). Not in the NAME character-class; ASCII-fold or
    /// refuse (24M §4b (c)).
    NonAscii(char),
    /// An ASCII character outside `[A-Za-z0-9_]` (a residual `.` from an un-transliterated
    /// reverse-DNS kind, a stray hyphen the munge did not map).
    InvalidChar(char),
}

impl NameProblem {
    /// The fact-plane prose naming the offending character and why it breaks the NAME.
    #[must_use]
    pub fn describe(self) -> String {
        match self {
            NameProblem::Empty => "empty name".to_owned(),
            NameProblem::LeadingDigit(c) => {
                format!("leading digit `{c}` (a NAME's first character must be a letter or `_`)")
            }
            NameProblem::NonAscii(c) => {
                format!("non-ASCII character `{c}` (a NAME admits only ASCII letters/digits/`_`)")
            }
            NameProblem::InvalidChar(c) => {
                format!("character `{c}` outside the NAME set `[A-Za-z0-9_]`")
            }
        }
    }
}

/// Validate that `name` is a legal POSIX NAME (`[A-Za-z_][A-Za-z0-9_]*`) — the munge-safety
/// primitive the `ca-munge-charclass` ruling (24M §4b) demands live at the munger. Pure; no
/// allocation. This is what the reverse-DNS→NAME munger MUST gate its output through: a name that
/// fails here cannot be emitted as an sh function name without producing a broken (or silently
/// mis-parsed) artifact.
///
/// # Errors
///
/// Returns the first [`NameProblem`] encountered (empty, leading digit, non-ASCII, or an ASCII
/// char outside the NAME set), scanning left-to-right.
pub fn validate_sh_name(name: &str) -> Result<(), NameProblem> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err(NameProblem::Empty);
    };
    if !first.is_ascii() {
        return Err(NameProblem::NonAscii(first));
    }
    if first.is_ascii_digit() {
        return Err(NameProblem::LeadingDigit(first));
    }
    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(NameProblem::InvalidChar(first));
    }
    for c in chars {
        if !c.is_ascii() {
            return Err(NameProblem::NonAscii(c));
        }
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(NameProblem::InvalidChar(c));
        }
    }
    Ok(())
}

/// One lifted role funcdef reduced to what the lint needs: the emitted funcname, the source name
/// it munged from, and the name span to point a diagnostic at.
struct EmittedName {
    /// The emitted sh function name (`<munged-source>__<role>`).
    funcname: String,
    /// The as-authored source name (provider command word, or kind), for the collision message.
    source: String,
    /// The funcdef name span (`inv-no-throw`: always real — the parser stamps it).
    span: Span,
}

/// Lift every role funcdef in one oracle source and reduce each to its [`EmittedName`]. Re-lifts
/// through the shared role-parametrized parser (the same lifts the cli's stages use); the lift's
/// OWN dialect diagnostics are the cli's `check` stage's job and are dropped here — a funcdef that
/// fails to lift emits no probe, so it needs no reservation check (`kFAIL`-safe by omission).
fn emitted_names(interner: &mut Interner, src: &str) -> Vec<EmittedName> {
    // (suffix, lifted set). The `provider` symbol a set is keyed by is the source name that
    // munges into the funcname — a command word for predict/touches/verdicts, a KIND for
    // resolve/reaches (both route through the same `to_funcname_segment`, so the lint is uniform).
    let roles: [(&str, PredictSet); 6] = [
        (
            "__predict",
            crate::predict::lift_predicts(interner, src).value,
        ),
        (
            "__disturbs",
            crate::predict::lift_touches(interner, src).value,
        ),
        (
            "__is_converged",
            crate::predict::lift_verdicts_converged(interner, src).value,
        ),
        (
            "__resolve",
            crate::predict::lift_resolvers(interner, src).value,
        ),
        (
            "__disturbance_reaches_only",
            crate::predict::lift_reaches(interner, src).value,
        ),
        (
            "__state_stored_only_in",
            crate::predict::lift_state_stored_only_in(interner, src).value,
        ),
    ];
    let mut out = Vec::new();
    for (suffix, set) in &roles {
        for sym in set.providers() {
            let Some(p) = set.get(sym) else { continue };
            let source = interner.resolve(sym).to_owned();
            let funcname = format!("{}{suffix}", crate::to_funcname_segment(&source));
            out.push(EmittedName {
                funcname,
                source,
                span: p.name_span,
            });
        }
    }
    out
}

/// Lint the oracle-side reserved namespace over the WHOLE analysis unit (`oracle_srcs`): the
/// **charclass** refusal (an emitted funcname that is not a legal sh NAME) and the **collision**
/// refusal (two distinct source names munging to one NAME). Both are Error-severity — a broken or
/// silently-merged emitted name is a correctness give-up (`kFAIL`: refuse over ship-broken /
/// ship-hijacked). Deterministic (`inv-determinism`): sources walked in argument order, collisions
/// keyed through a `BTreeMap`.
///
/// The spans are per-file byte offsets; the cli reports this stage with no threaded source (the
/// `oracle`-stage precedent), so a multi-file collision frames as a byte-offset rather than a
/// `file:line:col` — a spike-scoped simplification (ru-26): threading per-file source through a
/// cross-file collision needs a file handle on the span the spike does not carry, deferred.
#[must_use]
pub fn lint_oracle_reserved_names(interner: &mut Interner, oracle_srcs: &[&str]) -> Vec<Diag> {
    let mut diags = Vec::new();
    // funcname → the distinct (source-name, span) pairs that emit it. `>1` distinct source ⇒ a
    // collision. `BTreeMap` for determinism.
    let mut by_funcname: BTreeMap<String, Vec<(String, Span)>> = BTreeMap::new();

    for src in oracle_srcs {
        for e in emitted_names(interner, src) {
            // charclass: the emitted funcname must be a legal sh NAME, or it cannot ship.
            if let Err(problem) = validate_sh_name(&e.funcname) {
                diags.push(Diag::new(
                    DiagCode::MungeNameInvalid(MungeNameInvalid {
                        source: e.source.clone(),
                        funcname: e.funcname.clone(),
                        problem: problem.describe(),
                    }),
                    e.span,
                ));
            }
            let entry = by_funcname.entry(e.funcname).or_default();
            if !entry.iter().any(|(s, _)| *s == e.source) {
                entry.push((e.source, e.span));
            }
        }
    }

    // collision: any emitted funcname reachable from >1 DISTINCT source name. The same source in
    // two files is contribution, not collision (one distinct name); the period-and-mangled forms
    // of one provider intern to one name, so they never appear as two.
    for (funcname, sources) in &by_funcname {
        if sources.len() < 2 {
            continue;
        }
        let names: Vec<&str> = sources.iter().map(|(s, _)| s.as_str()).collect();
        for (source, span) in sources {
            diags.push(Diag::new(
                DiagCode::MungeNameCollision(MungeNameCollision {
                    source: source.clone(),
                    funcname: funcname.clone(),
                    count: names.len(),
                    names: names.join(", "),
                }),
                *span,
            ));
        }
    }

    diags
}

/// Lint a BOOK for funcdefs squatting the reserved `<x>__<role>` namespace — the coincidental
/// capture rul24M-bare-dorcism-names prices as accepted-not-prevented. Warning-severity and LOUD
/// (rul24-warnings-tune-high: warnings tune high this era; detection now, curation later). A
/// captured book function is treated as an ordinary opaque command (run-verbatim) by the rest of
/// the engine — this lint is only the disclosure that its NAME collides with the emitted oracle
/// namespace, so an author who did NOT mean it as an oracle role learns of it loudly rather than
/// silently.
#[must_use]
pub fn lint_book_reserved_names(ast: &dorc_syntax::Ast) -> Vec<Diag> {
    use dorc_syntax::ast::NodeKind;
    let mut diags = Vec::new();
    for (_, node) in ast.iter() {
        let NodeKind::FuncDef {
            name, name_span, ..
        } = &node.kind
        else {
            continue;
        };
        let Some(role) = RESERVED_ROLE_SUFFIXES.iter().find(|suffix| {
            name.strip_suffix(**suffix)
                .is_some_and(|base| !base.is_empty())
        }) else {
            continue;
        };
        diags.push(Diag::new(
            DiagCode::ReservedNamespaceSquat(ReservedNamespaceSquat {
                name: name.clone(),
                role: (*role).to_owned(),
            }),
            *name_span,
        ));
    }
    diags
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `validate_sh_name` accepts the ordinary munged shapes and refuses each `ca-munge-charclass`
    /// failure-dimension (24M §4b): leading digit, dot (an un-transliterated reverse-DNS label),
    /// non-ASCII, and the empty name.
    #[test]
    fn sh_name_charclass() {
        assert!(validate_sh_name("apt_get__predict").is_ok());
        assert!(validate_sh_name("_underscore_first").is_ok());
        assert!(validate_sh_name("package__resolve").is_ok());
        assert_eq!(
            validate_sh_name("7z__predict"),
            Err(NameProblem::LeadingDigit('7')),
            "a leading-digit provider is invalid (7z, 2to3)"
        );
        assert_eq!(
            validate_sh_name("sm.dorc.Package__resolve"),
            Err(NameProblem::InvalidChar('.')),
            "an un-transliterated reverse-DNS dot is invalid (the respell must transliterate)"
        );
        assert_eq!(
            validate_sh_name("\u{4e2d}pkg__predict"),
            Err(NameProblem::NonAscii('\u{4e2d}')),
            "a non-ASCII/IDN label is invalid"
        );
        assert_eq!(validate_sh_name(""), Err(NameProblem::Empty));
    }

    /// A non-ASCII provider funcdef lifts fine (its body is in-dialect) but its emitted funcname
    /// is refused by the charclass check — the surviving refusal path (the munge transliterates
    /// `.`/`-` and repairs leading digits, but leaves non-ASCII for `validate_sh_name` to refuse;
    /// rul24-idn-punycode punycoding is a spec-note, not implemented — `24P` §0).
    #[test]
    fn charclass_refuses_non_ascii_provider() {
        let mut i = Interner::default();
        let src = "\u{4e2d}pkg__predict() { pkg : archive = \"$1\"; foo l -- \"$pkg\"; }";
        let diags = lint_oracle_reserved_names(&mut i, &[src]);
        assert!(
            diags.iter().any(|d| d.code.slug() == "munge-name-invalid"),
            "a non-ASCII provider funcname is refused as an invalid NAME: {diags:?}"
        );
    }

    /// A leading-digit provider is now REPAIRED (resp-munge-policy: `7z` → `_7z`), not refused —
    /// the transliterate-and-accept direction the specimens exhibit. So it emits a CLEAN funcname
    /// with no `munge-name-invalid` diagnostic.
    #[test]
    fn leading_digit_provider_is_repaired_not_refused() {
        let mut i = Interner::default();
        let src = "7z__predict() { pkg : archive = \"$1\"; foo l -- \"$pkg\"; }";
        let diags = lint_oracle_reserved_names(&mut i, &[src]);
        assert!(
            !diags.iter().any(|d| d.code.slug() == "munge-name-invalid"),
            "a leading-digit provider repairs to `_7z__predict`, no refusal: {diags:?}"
        );
    }

    /// Two distinct source names that munge to one funcname collide — both refused, never merged.
    /// `apt.get__predict` (source `apt.get`, dot→`_`) and `apt_get__predict` (source `apt-get`,
    /// `-`→`_`) both emit `apt_get__predict` — the non-injective munge the collision lint catches.
    #[test]
    fn collision_refuses_munge_non_injectivity() {
        let mut i = Interner::default();
        let dotted =
            "apt.get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }";
        let under =
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }";
        let diags = lint_oracle_reserved_names(&mut i, &[dotted, under]);
        let hits: Vec<_> = diags
            .iter()
            .filter(|d| d.code.slug() == "munge-name-collision")
            .collect();
        assert_eq!(
            hits.len(),
            2,
            "BOTH colliding source names are refused (refuse-and-run, not first-wins): {diags:?}"
        );
        assert!(
            hits.iter()
                .all(|d| dorc_aid::diag::render_body(d, &i).contains("apt_get__predict")),
            "the collision names the shared emitted funcname: {diags:?}"
        );
    }

    /// A single provider defined in two files is CONTRIBUTION, not collision — one distinct source
    /// name, so no collision diagnostic (the multi-file contribution model working).
    #[test]
    fn same_provider_two_files_is_not_a_collision() {
        let mut i = Interner::default();
        let a = "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }";
        let b = "apt_get__disturbs() { printf '%s\\n' \"$1\" : disturbs package; }";
        let diags = lint_oracle_reserved_names(&mut i, &[a, b]);
        assert!(
            !diags
                .iter()
                .any(|d| d.code.slug() == "munge-name-collision"),
            "one provider, two files, two roles ⇒ no collision: {diags:?}"
        );
    }

    /// The clean corpus-standard package oracle trips NEITHER lint (a regression guard: a false
    /// positive here would fail every e2e case through gate-3).
    #[test]
    fn clean_oracle_is_quiet() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/package.oracle.sh"
        ));
        let mut i = Interner::default();
        let diags = lint_oracle_reserved_names(&mut i, &[fixture]);
        assert!(
            diags.is_empty(),
            "the clean package oracle is quiet: {diags:?}"
        );
    }

    /// A book funcdef named `<x>__<role>` squats the reserved namespace and is surfaced (Warning).
    /// The live corpus instance is the reingest floor's `apt_get__predict` book function.
    #[test]
    fn book_squat_is_surfaced() {
        let book = "\
apt_get__predict() { dpkg-query -W \"$1\"; }
apt_get__predict install -y nginx || apt-get install -y nginx
";
        let ast = dorc_syntax::parse(book).value;
        let diags = lint_book_reserved_names(&ast);
        assert_eq!(diags.len(), 1, "one squatting funcdef: {diags:?}");
        assert_eq!(diags[0].code.slug(), "reserved-namespace-squat");
        assert_eq!(diags[0].severity(), dorc_aid::Severity::Warning);
        assert!(
            dorc_aid::diag::render_body(&diags[0], &Interner::default())
                .contains("apt_get__predict")
        );
    }

    /// An ordinary book helper (no reserved suffix) is not flagged — the squat lint fires ONLY on
    /// the reserved shape, never on arbitrary book functions.
    #[test]
    fn ordinary_book_function_is_not_squatting() {
        let book = "install_pkg() { apt-get install -y \"$1\"; }\ninstall_pkg nginx\n";
        let ast = dorc_syntax::parse(book).value;
        assert!(lint_book_reserved_names(&ast).is_empty());
    }

    /// The deleted `is_diverged` role name is neither RESERVED nor RECOGNIZED — permanent-surface
    /// hygiene (`24C:rul24-ditch-is-diverged` hard-deleted it; `27Xf` Tier-2 residue; the
    /// lane-payload-v1 rider per `27D`). Role names are a permanent, unversionable compat surface
    /// (`271`), so a resurrection of `__is_diverged` anywhere — the reserved suffix list, the
    /// squat-lint's recognized shapes, or the role lift — is a silent re-introduction of the retired
    /// sense-flip. This pins all three surfaces at once:
    ///
    /// 1. `__is_diverged` is absent from [`RESERVED_ROLE_SUFFIXES`] (the closed six).
    /// 2. A BOOK funcdef `foo__is_diverged` is NOT surfaced as a namespace-squat (it squats no
    ///    reserved role — it is an ordinary opaque book function).
    /// 3. An oracle funcdef `foo__is_diverged` lifts to NO role-suffixed emitted name (the role does
    ///    not exist; `emitted_names` runs the six real role lifts and none claims it).
    #[test]
    fn is_diverged_is_neither_reserved_nor_recognized() {
        assert!(
            !RESERVED_ROLE_SUFFIXES.contains(&"__is_diverged"),
            "`__is_diverged` is a RETIRED role name (hard-deleted, `24C:rul24-ditch-is-diverged`) — \
             it must never re-enter the reserved suffix set"
        );

        // (2) a book `foo__is_diverged` is an ordinary opaque function, not a squat.
        let book = "foo__is_diverged() { : ; }\nfoo__is_diverged install\n";
        let ast = dorc_syntax::parse(book).value;
        assert!(
            lint_book_reserved_names(&ast).is_empty(),
            "`foo__is_diverged` squats no reserved role — the retired suffix recognizes nothing"
        );

        // (3) an oracle `foo__is_diverged` lifts to no role-suffixed emitted name.
        let mut i = Interner::default();
        let names = emitted_names(&mut i, "foo__is_diverged() { dpkg-query -W \"$1\"; }");
        let emitted: Vec<&str> = names.iter().map(|e| e.funcname.as_str()).collect();
        assert!(
            emitted.iter().all(|f| !f.ends_with("__is_diverged")),
            "no role lift recognizes the `__is_diverged` suffix: {emitted:?}"
        );
    }

    /// The reserved-suffix list agrees with what the parser's role lifts actually recognize: for
    /// each suffix, an oracle authored with the matching mangled name lifts to exactly one emitted
    /// funcname carrying that suffix. Ties [`RESERVED_ROLE_SUFFIXES`] to the parser's `FnRole`
    /// mangled-suffix pairing mechanically (a drift between the two would surface here).
    #[test]
    fn suffixes_match_lifted_roles() {
        for suffix in RESERVED_ROLE_SUFFIXES {
            let mut i = Interner::default();
            // A minimal in-dialect body under the mangled name for this role (`foo__<role>`).
            let src = format!("foo{suffix}() {{ dpkg-query -W \"$1\"; }}");
            let names = emitted_names(&mut i, &src);
            assert!(
                names.iter().any(|e| e.funcname == format!("foo{suffix}")),
                "role suffix `{suffix}` must lift `foo{suffix}` to an emitted name of that shape \
                 (RESERVED_ROLE_SUFFIXES vs FnRole drift)"
            );
        }
    }
}
