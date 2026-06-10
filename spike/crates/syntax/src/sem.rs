//! `syntax::sem` — the single home of the engine's beliefs about POSIX sh **word
//! semantics** (`cm-3`, note `20A` §2; `20D`).
//!
//! Before this module those beliefs were re-implemented independently in the
//! value-plane (`analysis::value`), the contract-dialect parser+evaluator
//! (`oracle::check`), and the probe render (`plan`). Each re-implementation was an
//! *independent dash-divergence surface* — `20A` §1 fam-B — and the round's two
//! priority-1 wrong-elision bugs were exactly such divergences (prefix-env argv
//! visibility; the `${N#pat}` literal-vs-glob split). Collapsing them to one place
//! reduces that surface from `O(components)` to `O(1)` and lets a single
//! differential gate (`cm-2`) validate all consumers at once.
//!
//! # Scope and posture
//!
//! Each function documents the POSIX.1-2024 Shell & Utilities (XCU) §2.x clause it
//! implements and a one-line note on dash's observable behaviour, because dash —
//! not the spec — is the trust oracle (`an-differential-vs-shell`). The module is a
//! pure, dependency-clean kernel (`inv-determinism`/`inv-no-throw`): no clock, RNG,
//! I/O, allocation-of-unbounded-size, or panic. It models a deliberately-narrow
//! slice; everything outside it is reported as *not modelable* so the caller
//! degrades to ⊤ (`inv-top-reject`) rather than guessing — a wrong concrete is the
//! disaster class (`19H §1.3`).
//!
//! # What stays scattered (coverage map — see `20D` for the full table)
//!
//! This module owns *word-level* semantics: parameter classification, the quoting
//! classes, the one modelable parameter-expansion (`${N#literal}`), unset-parameter
//! policy, literal extraction, and shell-quoting. It does **not** own statement- or
//! command-level semantics — errexit edge pruning (`analysis::cfg`), command-prefix
//! argv-vs-environment ordering (`analysis::value::site_argv`), `case`/`while`
//! argparse control-flow (`oracle::check::eval`), or leaf/word *assembly* in the two
//! renders. Those remain where the control-flow they are entangled with lives.

use crate::ast::WordPart;

// ===========================================================================
// §1 Parameter classification (XCU §2.5 Parameters and Variables)
// ===========================================================================

/// What a parameter name denotes (XCU §2.5). The three classes the analyzer treats
/// differently: a **name** is a plain shell variable the dataflow may track; a
/// **positional** (`$1`, `${12}`) is runtime script input; a **special** parameter
/// (`$@ $* $# $? $- $$ $! $0`) is dynamic state. Only [`Name`](ParamClass::Name) is
/// ever statically trackable — positionals and specials are runtime input ⇒ ⊤
/// (`19H`: script args are runtime input, always ⊤ regardless of quoting).
///
/// dash note: `$0` is the script/function name — never a positional operand — so it
/// is classed [`Special`](ParamClass::Special), not `Positional(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamClass {
    /// A plain variable name `[A-Za-z_][A-Za-z0-9_]*` (XCU §3.235 "Name").
    Name,
    /// A positional parameter `N ≥ 1` (XCU §2.5.1). Carries the 1-based index.
    Positional(u32),
    /// A special parameter (`@ * # ? - $ ! 0`) — dynamic, never statically fixed.
    Special,
}

/// Classify a parameter *name* (the text between `$`/`${` and any operator or
/// close-brace) into its [`ParamClass`] (XCU §2.5).
///
/// This is the **one** definition of "is this `$name` a plain var" the engine has;
/// `analysis::value` and `oracle::check::parser` both keyed off private copies that
/// agreed on the predicate but disagreed on what the *other* cases meant (the
/// value-plane folded every non-name — positional, special, multi-digit `${12}` —
/// into one ⊤ bucket via a `false` return, conflating three distinct POSIX classes;
/// the dialect kept `Positional(n)` first-class). Both behaviours are preserved: a
/// caller wanting only the boolean uses [`is_name`]; one needing the positional
/// index reads [`ParamClass::Positional`].
///
/// dash note: a multi-digit positional is reachable only as `${12}` — bare `$12` is
/// `$1` followed by the literal `2` (XCU §2.6.2: only `${...}` brackets a
/// multi-digit positional). An empty name is [`Special`](ParamClass::Special)-ward
/// (never a valid plain var); we return `Positional`/`Name` only on a definite match.
#[must_use]
pub fn classify_param(name: &str) -> ParamClass {
    // All-digit ⇒ positional, except `$0` (the program name, a special). Covers both
    // the single-digit `$1` and the braced multi-digit `${12}` (the only multi-digit
    // spelling). A non-parsing or zero index falls through to Special.
    if !name.is_empty() && name.bytes().all(|b| b.is_ascii_digit()) {
        return match name.parse::<u32>() {
            Ok(n) if n != 0 => ParamClass::Positional(n),
            // `$0` (program/function name) and an overflowing index are both Special.
            _ => ParamClass::Special,
        };
    }
    if is_name(name) {
        return ParamClass::Name;
    }
    // Everything else is Special — the single-char special parameters (XCU §2.5.2:
    // `$@ $* $# $? $- $$ $!`) and any other non-name, non-positional text (empty,
    // `${!ref}`-leftovers, …). Callers treat all of these as ⊤ (never statically
    // fixed), so they need not be distinguished from one another.
    ParamClass::Special
}

/// Is `s` a POSIX *name* — `[A-Za-z_][A-Za-z0-9_]*` (XCU §3.235)? Used to recognize
/// plain variables, lvalues, function names, and the annotation `name`.
///
/// dash note: dash rejects a leading digit (`1x=…` is not an assignment) and any
/// non-`[A-Za-z0-9_]` byte; this predicate matches that exactly. It is the lexical
/// rule behind `analysis::value::is_plain_var`, `oracle::check::parser::is_ident`,
/// and `syntax::parser::is_func_name`/`is_assignment_name` — all four were
/// byte-for-byte identical (no divergence) and now route here.
#[must_use]
pub fn is_name(s: &str) -> bool {
    let mut bytes = s.bytes();
    match bytes.next() {
        Some(b) if b == b'_' || b.is_ascii_alphabetic() => {}
        _ => return false,
    }
    bytes.all(|b| b == b'_' || b.is_ascii_alphanumeric())
}

// ===========================================================================
// §2 Quoting classes (XCU §2.2 Quoting, §2.6 Word Expansions, §2.6.5 Field Splitting)
// ===========================================================================

/// How one [`WordPart`], in a given quoting context, contributes to a word's
/// *statically-known* value (XCU §2.2 / §2.6).
///
/// This is the value-plane's "what may an unquoted / double-quoted / single-quoted
/// fragment contain" rule, named once. The load-bearing case is
/// [`SplitRisk`](FragClass::SplitRisk): an **unquoted** expansion may word-split or
/// glob (XCU §2.6.5 Field Splitting, §2.6.6 Pathname Expansion), so its arity is not
/// statically one argument ⇒ the whole word collapses to ⊤. This is the named form
/// of `value.rs`'s "unquoted expansion ⇒ ⊤" rule and the derived-hazard behind
/// [`Word::may_split`](crate::ast::Word::may_split).
///
/// dash note: inside double-quotes (XCU §2.2.3) an expansion is value-preserving —
/// it does not re-split — so `"$x"` is one field but `$x` may be many. Single-quotes
/// (XCU §2.2.2) suppress every expansion; their content is fully literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragClass<'a> {
    /// Literal text contributed verbatim (a `Literal`/`SingleQuoted` part, or a
    /// double-quoted literal). Value-preserving.
    Literal(&'a str),
    /// A plain-variable expansion that is value-preserving in this context (a
    /// *quoted* `$name`). Resolvable against dataflow state without split risk.
    Var(&'a str),
    /// An expansion whose static value is unknown but which is value-preserving in
    /// arity (a quoted positional/special/command-subst/arithmetic/operator-form):
    /// the word stays one argument, but this fragment is ⊤. The whole word is ⊤,
    /// without a word-splitting hazard.
    OpaqueValue,
    /// An **unquoted** expansion that may word-split/glob (XCU §2.6.5/§2.6.6): the
    /// word's arity is not statically one ⇒ collapse the word to ⊤.
    SplitRisk,
}

/// Classify a [`WordPart`] under a quoting context into its [`FragClass`] (XCU §2.2 /
/// §2.6). `quoted` is `true` when the part appears inside a double-quote (so an
/// expansion does not word-split).
///
/// A [`DoubleQuoted`](WordPart::DoubleQuoted) part is *structural* (it nests parts at
/// `quoted = true`); it has no single class, so callers recurse into its inner parts
/// themselves — this returns [`None`] for it. Every other part maps to exactly one
/// class.
///
/// The split between [`Var`](FragClass::Var) (a quoted plain `$name`, trackable) and
/// [`OpaqueValue`](FragClass::OpaqueValue) (a quoted positional/special/subst, ⊤ but
/// arity-safe) and [`SplitRisk`](FragClass::SplitRisk) (any unquoted expansion) is
/// the precise rule `value.rs::collect_frags` hand-rolled.
#[must_use]
pub fn classify_frag(part: &WordPart, quoted: bool) -> Option<FragClass<'_>> {
    match part {
        WordPart::Literal(s) | WordPart::SingleQuoted(s) => Some(FragClass::Literal(s)),
        WordPart::DoubleQuoted(_) => None,
        WordPart::Param { name } => {
            // An unquoted expansion may word-split / glob ⇒ arity not statically one.
            if !quoted {
                return Some(FragClass::SplitRisk);
            }
            match classify_param(name) {
                ParamClass::Name => Some(FragClass::Var(name)),
                // Positional/special are runtime input ⇒ ⊤, but quoted ⇒ one field.
                ParamClass::Positional(_) | ParamClass::Special => Some(FragClass::OpaqueValue),
            }
        }
        // Command-substitution, arithmetic, and operator-expansions are runtime or
        // unmodeled. Unquoted they also split; quoted they are arity-safe but ⊤.
        WordPart::CommandSubst(_) | WordPart::Arithmetic | WordPart::ParamComplex => {
            if quoted {
                Some(FragClass::OpaqueValue)
            } else {
                Some(FragClass::SplitRisk)
            }
        }
    }
}

// ===========================================================================
// §3 The one modelable parameter expansion: `${N#literal}` (XCU §2.6.2)
// ===========================================================================

/// A parsed `${N#prefix}` — a positional with a leading **literal** prefix removed
/// (XCU §2.6.2 "Remove Smallest Prefix Pattern", the `#` form).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixStrip<'a> {
    /// The positional index `N`.
    pub n: u32,
    /// The literal prefix to strip (already verified glob-free and not `##`).
    pub prefix: &'a str,
}

/// Try to parse the inside of a `${...}` (the text between the braces) as the **one**
/// modelable prefix-strip form: `${N#literal}` where `N` is a positional and the
/// prefix is a glob-free literal (XCU §2.6.2).
///
/// Returns [`None`] — *not modelable* — for every other `${...}` form, which is the
/// is-modelable predicate the dialect parser uses to **reject** (round-20 ruling:
/// globby/`##` forms are out-of-model). The rejected forms and why each diverges
/// from a literal strip:
///
/// * `${N##pat}` — longest-match (XCU §2.6.2): `split_once('#')` would mangle it into
///   a literal prefix starting with `#`; dash strips a *pattern*. → `None`.
/// * `${N#*=}` / `${N#?x}` / `${N#[ab]}` — the prefix is an fnmatch glob (XCU §2.13);
///   dash matches by pattern, a literal strip matches nothing. → `None`.
/// * `${name#...}` — a *variable* (not positional) strip; not modeled. → `None`.
///
/// dash note: this is the literal-prefix case of `${var#pattern}`, where shortest ==
/// the literal because a literal pattern has exactly one match length. Misreading a
/// glob form as literal was round-20 crosscheck finding 2 — a wrong concrete (the
/// disaster class); hence the conservative `None`-on-anything-non-literal.
#[must_use]
pub fn parse_prefix_strip(inner: &str) -> Option<PrefixStrip<'_>> {
    let (digits, prefix) = inner.split_once('#')?;
    let n = digits.parse::<u32>().ok()?;
    // `##` (longest match) presents here as a prefix that itself starts with `#`.
    if prefix.starts_with('#') {
        return None;
    }
    // A globby prefix is an fnmatch pattern, not a literal — out of model.
    if prefix.contains(['*', '?', '[']) {
        return None;
    }
    Some(PrefixStrip { n, prefix })
}

/// Apply `${var#prefix}` for a **literal** prefix (XCU §2.6.2, `#` shortest-match):
/// remove `prefix` from the start of `s` once, or return `s` unchanged if it does not
/// start with `prefix`.
///
/// dash note: for a literal (glob-free) prefix the shortest match equals the literal,
/// so a single `strip_prefix` is exact. (The longest-match `##` and glob forms are
/// rejected upstream by [`parse_prefix_strip`], so this only ever sees a literal.)
#[must_use]
pub fn strip_prefix_literal<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

// ===========================================================================
// §4 Unset-parameter policy (XCU §2.5.3 / §2.6.2)
// ===========================================================================

/// How an *unset* parameter expands, which differs by syntactic context (XCU
/// §2.5.3). The engine has two contexts with opposite needs, so the policy is a
/// named choice rather than a buried branch:
///
/// * [`ExpandEmpty`](UnsetPolicy::ExpandEmpty) — a plain `$N`/`${N}` of an unset
///   parameter expands to the **empty string** (XCU §2.6.2: an unset parameter
///   without a modifier expands to null). This is the `[ … ]` *test* context: the
///   flag-strip `while [ "${1#-}" != "$1" ]` must terminate cleanly when argv is
///   exhausted, and `[ "$2" = "" ]` reads true at the end.
/// * [`Unresolved`](UnsetPolicy::Unresolved) — an unset parameter does **not** yield
///   a concrete value; the consumer must degrade to ⊤. This is the *strict* context:
///   a value-flow argv word, or a check's annotation value-position, where an entity
///   must resolve concretely or the site stays un-elidable (`inv-kfail`).
///
/// dash note: dash expands an unset `$1` to empty in *every* word context; the
/// engine's [`Unresolved`](UnsetPolicy::Unresolved) is **deliberately stricter than
/// dash** — it refuses to mint a concrete entity from emptiness (a soundness floor,
/// not a dash model). A `$0`/unbound *variable* is non-concrete under *both*
/// policies (only past-the-end positionals take the empty-vs-unresolved fork).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsetPolicy {
    /// Test/word context: unset positional ⇒ empty string (dash-faithful).
    ExpandEmpty,
    /// Strict context: unset ⇒ no concrete value (degrade to ⊤; soundness floor).
    Unresolved,
}

// ===========================================================================
// §5 Literal-text extraction (the `ValueOf::Literal` contract, XCU §2.6)
// ===========================================================================

/// Extract a word's **compile-time-constant** text from its parts: `Some(text)` iff
/// every part is literal — *no* parameter expansion, command-substitution,
/// arithmetic, or operator-form anywhere, quoted or not (XCU §2.6: none of the
/// expansions fire). Returns [`None`] the moment any non-literal part appears.
///
/// This is the "no variables at all" case — strictly narrower than a *resolved*
/// literal (a quoted `$x` that dataflow happens to know): it is used where a
/// command's **shape** must be recognized independent of any dataflow state
/// (`unset name`, the command word of a builtin, the dn-1 metadata anchor). It is the
/// single home of `analysis::value::literal_text`'s rule and is exactly
/// [`Word::as_literal`](crate::ast::Word::as_literal) generalized past the
/// single-part case (concatenated literals `'a'b"c"` ⇒ `abc`).
///
/// The distinct, *wider* "fully-expanded given state" guarantee — the
/// [`ValueOf::Literal`] contract that a resolved value is the single argument dash
/// would pass — is the value-plane's recipe resolution built on [`classify_frag`];
/// it is documented at its consumer (`analysis::value`), not duplicated here, because
/// it needs the live dataflow environment this dependency-free kernel must not hold.
#[must_use]
pub fn const_literal_text(parts: &[WordPart]) -> Option<String> {
    fn push_parts(parts: &[WordPart], buf: &mut String) -> bool {
        for part in parts {
            match part {
                WordPart::Literal(s) | WordPart::SingleQuoted(s) => buf.push_str(s),
                WordPart::DoubleQuoted(inner) => {
                    if !push_parts(inner, buf) {
                        return false;
                    }
                }
                WordPart::Param { .. }
                | WordPart::CommandSubst(_)
                | WordPart::Arithmetic
                | WordPart::ParamComplex => return false,
            }
        }
        true
    }
    let mut buf = String::new();
    push_parts(parts, &mut buf).then_some(buf)
}

// ===========================================================================
// §6 Shell-quoting — the F-QUOTE single-quote-always rule (XCU §2.2.2)
// ===========================================================================

/// POSIX single-quote `s` so it becomes exactly **one** literal argument when parsed
/// by any `sh` (XCU §2.2.2 Single-Quotes) — the F-QUOTE rule (`notes/198`,
/// `inv-kfail` both directions).
///
/// An operand is interned post-parse (quotes already stripped, embedded metachars
/// preserved), so passing it raw into a rendered probe (`package__check my pkg`)
/// would word-split into two args (⇒ probes the wrong entity, a `kFAIL-perform`
/// wrong-elision) or, for `x; touch /tmp/PWNED`, re-parse as a second command (⇒ a
/// `kFAIL-withhold` probe-mutation). Wrapping in `'…'` makes the value inert and
/// exactly one positional argument, everywhere.
///
/// dash note: single-quotes suppress *all* expansion, so the only byte needing
/// escape is `'` itself — emitted as the idiomatic `'\''` (close-quote, escaped
/// literal quote, re-open). This is a pass-through byte transform, never a decode
/// (`inv-referent-agnostic`): it branches only on which bytes must be escaped, never
/// on what the operand means.
#[must_use]
pub fn single_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len().saturating_add(2));
    out.push('\'');
    for ch in s.chars() {
        if ch == '\'' {
            out.push_str("'\\''"); // close-quote, escaped literal quote, re-open
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

// ===========================================================================
// §7 Field splitting of a known literal under default IFS (XCU §2.6.5)
// ===========================================================================

/// The default-IFS field separators (XCU §2.6.5): `<space>`, `<tab>`, `<newline>`.
/// When `IFS` is unset the shell behaves as if `IFS=" \t\n"`; this models *only* the
/// default — any book-side `IFS` touch makes splitting unmodelable upstream (see
/// `analysis::value`'s IFS-pristine pre-pass), so this module never sees a custom IFS.
const DEFAULT_IFS: [char; 3] = [' ', '\t', '\n'];

/// A character that triggers pathname expansion (XCU §2.6.6 / §2.13 fnmatch): `*`, `?`,
/// `[`. A *split-result field* containing one of these is matched against the live
/// filesystem (dash-verified: `V="*.txt"; cmd $V` expands to the matching paths), which
/// is unmodelable statically ⇒ the word degrades to ⊤. (Tilde is deliberately absent:
/// for a SPLIT-result field dash does *not* tilde-expand — `V="~"; cmd $V` passes a
/// literal `~` — so a leading `~` in a split field is the safe literal, never a hazard.)
const GLOB_CHARS: [char; 3] = ['*', '?', '['];

/// Split one **literal value** into fields under default IFS (XCU §2.6.5), as the shell
/// does for an *unquoted* expansion. Maximal runs of IFS-whitespace are field
/// separators; leading, trailing, and repeated separators collapse and elide (they
/// never produce empty fields); an empty value yields **zero** fields (field elision —
/// `cmd $EMPTY x` runs `cmd x`, dash-verified).
///
/// This is the primitive behind [`split_fields_join`] (which handles a word *mixing*
/// literals and split values). It models only the default IFS — the safety of using it
/// at all (a known literal value + a book-pristine IFS + glob-free fields) is the
/// caller's precondition (see [`field_is_modelable`] and `analysis::value`).
///
/// dash note: default IFS-whitespace separators are special — adjacent separators and
/// leading/trailing separators do NOT create empty fields (XCU §2.6.5: a sequence of
/// IFS white-space delimits with no null fields). This differs from a custom non-blank
/// IFS, which we never model.
#[must_use]
pub fn split_default_ifs(value: &str) -> Vec<&str> {
    value.split(DEFAULT_IFS).filter(|f| !f.is_empty()).collect()
}

/// Is a split-result `field` statically modelable (XCU §2.6.6)? `false` when it carries
/// a pathname-expansion metacharacter (`*`, `?`, `[`): dash expands such a field against
/// the remote filesystem, which is runtime-dependent ⇒ the whole word is ⊤ (the
/// wrong-concrete frontier — see [`GLOB_CHARS`]). A `~`-leading field is modelable (it
/// is a literal `~` post-split; tilde expansion fired, if at all, on the *original* word
/// before splitting, never on a split-result field).
#[must_use]
pub fn field_is_modelable(field: &str) -> bool {
    !field.contains(GLOB_CHARS)
}

/// One resolved fragment of an *unquoted* word, ready for field-splitting (XCU §2.6.5).
/// A word that mixes literal text with a resolved-variable value (`pre$PKGS`,
/// `$PKGS.deb`) is a sequence of these; [`split_fields_join`] applies the POSIX
/// field-boundary rule across them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field<'a> {
    /// Verbatim literal text (a literal fragment, or a *quoted* expansion that was
    /// already resolved to a literal). Contributes to the current field; an IFS
    /// character *inside* it is NOT a separator (it was quoted, or it would have been a
    /// word boundary at parse time) — only [`Split`](Field::Split) introduces breaks.
    Literal(&'a str),
    /// A resolved-variable value appearing *unquoted* — subject to field-splitting. IFS
    /// runs inside it ARE field separators (this is the only source of breaks).
    Split(&'a str),
}

/// Apply POSIX field-splitting (XCU §2.6.5) under default IFS across a word's resolved
/// fragments, reproducing dash's argv exactly for a known-literal unquoted (or
/// mixed-literal) word. Returns the resulting field strings, or [`None`] if any field
/// carries a glob metacharacter (unmodelable pathname expansion ⇒ caller degrades the
/// word to ⊤).
///
/// The field-boundary rule (dash-verified against the differential gate): only a
/// separator *contributed by a [`Split`](Field::Split) fragment* breaks a field;
/// literal text and the boundaries *between* fragments join. So:
/// * `[Literal("pre"), Split("nginx curl")]` ⇒ `["prenginx", "curl"]` — the literal
///   joins the first split field; the internal separator breaks.
/// * `[Split("nginx curl"), Literal(".deb")]` ⇒ `["nginx", "curl.deb"]` — the trailing
///   literal joins the last split field.
/// * `[Literal("pre"), Split(""), Literal(".post")]` ⇒ `["pre.post"]` — an empty value
///   contributes no separator, so the literals join (one field).
/// * `[Literal("pre"), Split("   "), Literal(".post")]` ⇒ `["pre", ".post"]` — an
///   all-separator value DOES break (the separator run is internal to the word).
///
/// dash note: the algorithm is an open-field accumulator — literal text and split-field
/// *text* extend the open field; an IFS run inside a [`Split`](Field::Split) flushes the
/// open field (only when one is open, so leading/trailing/repeated separators neither
/// create nor leave empty fields). This matches dash's "split only on expansion-
/// introduced separators" precisely (a literal IFS byte in the word never splits).
#[must_use]
pub fn split_fields_join(frags: &[Field<'_>]) -> Option<Vec<String>> {
    let mut acc = FieldAccumulator::default();
    for frag in frags {
        match frag {
            Field::Literal(s) => acc.extend(s),
            Field::Split(value) => {
                // Each token sits between IFS runs; a non-first token means a separator
                // run preceded it ⇒ break the open field. `str::split` yields an empty
                // token for a separator at an edge or in a run, which `break_field`
                // (open-only) and `extend` (no-op on empty) handle without empty fields.
                for (i, token) in value.split(DEFAULT_IFS).enumerate() {
                    if i > 0 {
                        acc.break_field();
                    }
                    acc.extend(token);
                }
            }
        }
    }
    let fields = acc.finish();
    fields
        .iter()
        .all(|f| field_is_modelable(f))
        .then_some(fields)
}

/// The open-field accumulator behind [`split_fields_join`] (XCU §2.6.5): builds fields
/// left-to-right, where literal/split *text* extends the currently-open field and an
/// IFS run inside a split value [`break`](FieldAccumulator::break_field)s it. Only an
/// *open* field flushes, so leading/trailing/repeated separators neither create nor
/// leave empty fields (the default-IFS-whitespace no-null-fields rule).
#[derive(Default)]
struct FieldAccumulator {
    done: Vec<String>,
    cur: String,
    open: bool,
}

impl FieldAccumulator {
    /// Append text to the open field (opening one if needed). Empty text is a no-op, so
    /// it never spuriously opens a field.
    fn extend(&mut self, text: &str) {
        if !text.is_empty() {
            self.cur.push_str(text);
            self.open = true;
        }
    }

    /// A field separator: flush the open field, if any. A closed field is a no-op, so
    /// adjacent separators collapse.
    fn break_field(&mut self) {
        if self.open {
            self.done.push(std::mem::take(&mut self.cur));
            self.open = false;
        }
    }

    /// Finish: flush any trailing open field and return the fields in order.
    fn finish(mut self) -> Vec<String> {
        self.break_field();
        self.done
    }
}

// ===========================================================================
// §8 Word-level unquoted expansion hazards: pathname (glob) + tilde (XCU §2.6.6 / §2.6.1)
// ===========================================================================

/// Does this word contain an **unquoted literal** pathname-expansion metacharacter
/// (`*`, `?`, `[`) — i.e. one typed directly into an unquoted fragment of the word
/// (XCU §2.6.6)? Such a word is matched against the live filesystem at expansion, which
/// is runtime-dependent ⇒ the consumer must degrade it to ⊤ (the wrong-concrete frontier,
/// `19H §1.3`). Quoted fragments (`"*.conf"`, `'*'`, a double-quoted literal) are exempt:
/// dash treats a quoted `*` as a literal byte (dash-verified `cmd "*.conf"` ⇒ `[*.conf]`).
///
/// This is the **word-source** companion to [`field_is_modelable`] (the **resolved-value**
/// glob check) — they share [`GLOB_CHARS`], the one definition (the prompt's "split-field
/// guard and word-level guard share it"). `field_is_modelable` catches a glob char arriving
/// through an *unquoted variable's value* (`x="*.deb"; cmd $x`, routed via the split path);
/// this catches one typed *literally and unquoted* in the word (`cmd *.deb`). The two are
/// complementary and non-overlapping: a source `$x` carries no literal glob char, and a
/// resolved field never re-derives the word's quoting.
///
/// dash note: pathname expansion fires on the fields of a command word and a `for`-list
/// word, but **not** on an assignment-statement RHS — `x=*.txt` stores the literal `*.txt`
/// (dash-verified). So callers apply this at *expansion* sites (argv, `for`-list) only; an
/// assignment RHS keeps a source-literal glob concrete (the three-row store/use/quoted-use
/// table, `analysis::value`). An unterminated `[` (no closing `]`) is a *literal* to dash,
/// so this over-degrades that one shape to ⊤ — the safe direction (`inv-kfail`).
#[must_use]
pub fn word_has_unquoted_glob(parts: &[WordPart]) -> bool {
    parts.iter().any(|part| match part {
        // A *bare* (unquoted) literal fragment globs on its metacharacters.
        WordPart::Literal(s) => s.contains(GLOB_CHARS),
        // Single-quoted, double-quoted, and every expansion are NOT a source-literal
        // glob: single/double quotes suppress globbing of their bytes, and an expansion's
        // *value*-glob is the split path's concern ([`field_is_modelable`]), not this one.
        WordPart::SingleQuoted(_)
        | WordPart::DoubleQuoted(_)
        | WordPart::Param { .. }
        | WordPart::CommandSubst(_)
        | WordPart::Arithmetic
        | WordPart::ParamComplex => false,
    })
}

/// Does this word begin with an **unquoted word-leading `~`** (XCU §2.6.1 Tilde
/// Expansion)? True iff the word's first [`WordPart`] is an unquoted [`Literal`] whose
/// first byte is `~`. dash expands such a tilde-prefix to a home directory (dash-verified
/// `cmd ~` ⇒ `[/home/...]`), which we cannot reproduce (no `$HOME` model) ⇒ the consumer
/// degrades the word to ⊤.
///
/// dash note: tilde expansion fires word-leading-unquoted only — `cmd x~` and `cmd "$x"~`
/// pass a literal `~` (the tilde is not word-initial), and `cmd '~'`/`cmd "~"` are literal
/// (quoted). Unlike pathname expansion, tilde DOES fire on an assignment RHS (`x=~` ⇒
/// `$HOME`, dash-verified), so callers apply this at *all* value-resolution sites including
/// assignment RHS. This is a deliberately-conservative syntactic test: it over-degrades
/// `~$x` and `~nouser` (which dash leaves literal because the login-name part is unknown /
/// invalid) to ⊤ — the safe direction (`inv-kfail`); we model no tilde-prefix *positively*.
/// The `:`-delimited assignment tilde (`x=a:~`, XCU §2.6.1) is NOT covered (not word-leading);
/// it is a recorded residual (`20Q`), an under-degrade that no observed idiom exercises.
#[must_use]
pub fn word_has_leading_tilde(parts: &[WordPart]) -> bool {
    matches!(parts.first(), Some(WordPart::Literal(s)) if s.starts_with('~'))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- §1 parameter classification -----------------------------------------

    #[test]
    fn names_positionals_specials_classified() {
        assert_eq!(classify_param("pkg"), ParamClass::Name);
        assert_eq!(classify_param("_x"), ParamClass::Name);
        assert_eq!(classify_param("PKG_2"), ParamClass::Name);
        assert_eq!(classify_param("1"), ParamClass::Positional(1));
        assert_eq!(classify_param("9"), ParamClass::Positional(9));
        // `${12}` — a multi-digit positional, reachable only via braces.
        assert_eq!(classify_param("12"), ParamClass::Positional(12));
        // `$0` is the program name, a special — NOT a positional operand.
        assert_eq!(classify_param("0"), ParamClass::Special);
        for sp in ["@", "*", "#", "?", "-", "$", "!"] {
            assert_eq!(classify_param(sp), ParamClass::Special, "{sp} is special");
        }
        // Empty / junk ⇒ Special-ward (never a fixed value).
        assert_eq!(classify_param(""), ParamClass::Special);
    }

    #[test]
    fn is_name_matches_posix_name_rule() {
        assert!(is_name("x"));
        assert!(is_name("_"));
        assert!(is_name("a1_b2"));
        assert!(!is_name("1x"), "leading digit is not a name");
        assert!(!is_name(""), "empty is not a name");
        assert!(!is_name("a-b"), "hyphen is not a name char");
        assert!(!is_name("a b"), "space is not a name char");
    }

    // ---- §2 quoting classes ---------------------------------------------------

    #[test]
    fn unquoted_param_is_split_risk_quoted_var_is_trackable() {
        let p = WordPart::Param {
            name: "x".to_owned(),
        };
        assert_eq!(classify_frag(&p, false), Some(FragClass::SplitRisk));
        assert_eq!(classify_frag(&p, true), Some(FragClass::Var("x")));
    }

    #[test]
    fn quoted_positional_is_opaque_value_not_var() {
        let p = WordPart::Param {
            name: "1".to_owned(),
        };
        // Quoted: arity-safe but ⊤ (a positional is runtime input).
        assert_eq!(classify_frag(&p, true), Some(FragClass::OpaqueValue));
        // Unquoted: also a split risk.
        assert_eq!(classify_frag(&p, false), Some(FragClass::SplitRisk));
    }

    #[test]
    fn literal_parts_are_literal_in_any_context() {
        let lit = WordPart::Literal("abc".to_owned());
        let sq = WordPart::SingleQuoted("a b".to_owned());
        assert_eq!(classify_frag(&lit, false), Some(FragClass::Literal("abc")));
        assert_eq!(classify_frag(&sq, true), Some(FragClass::Literal("a b")));
    }

    #[test]
    fn command_subst_arith_complex_split_unquoted_opaque_quoted() {
        for part in [
            WordPart::CommandSubst(dorc_core::AstId(0)),
            WordPart::Arithmetic,
            WordPart::ParamComplex,
        ] {
            assert_eq!(classify_frag(&part, false), Some(FragClass::SplitRisk));
            assert_eq!(classify_frag(&part, true), Some(FragClass::OpaqueValue));
        }
    }

    #[test]
    fn double_quoted_part_has_no_single_class() {
        let dq = WordPart::DoubleQuoted(vec![WordPart::Literal("x".to_owned())]);
        assert_eq!(classify_frag(&dq, false), None);
    }

    // ---- §3 prefix-strip ------------------------------------------------------

    #[test]
    fn literal_prefix_strip_parses() {
        let got = parse_prefix_strip("1#-").unwrap();
        assert_eq!(got, PrefixStrip { n: 1, prefix: "-" });
        assert_eq!(strip_prefix_literal("-y", "-"), "y");
        assert_eq!(
            strip_prefix_literal("nginx", "-"),
            "nginx",
            "no prefix ⇒ as-is"
        );
    }

    #[test]
    fn globby_and_longest_match_and_var_strips_are_unmodelable() {
        // `${1#*=}` — fnmatch glob ⇒ not modelable.
        assert_eq!(parse_prefix_strip("1#*=").map(|p| p.prefix), None);
        // `${1##*/}` — longest match ⇒ prefix begins with `#` ⇒ not modelable.
        assert_eq!(parse_prefix_strip("1##*/"), None);
        // `${1#?x}` / `${1#[ab]}` — other glob metachars.
        assert_eq!(parse_prefix_strip("1#?x"), None);
        assert_eq!(parse_prefix_strip("1#[ab]"), None);
        // `${name#-}` — variable (non-positional) strip ⇒ not modelable here.
        assert_eq!(parse_prefix_strip("name#-"), None);
        // No `#` at all ⇒ not a strip.
        assert_eq!(parse_prefix_strip("1"), None);
    }

    // ---- §5 const literal text ------------------------------------------------

    #[test]
    fn const_literal_text_concatenates_literals_only() {
        let parts = vec![
            WordPart::Literal("a".to_owned()),
            WordPart::SingleQuoted("b".to_owned()),
            WordPart::DoubleQuoted(vec![WordPart::Literal("c".to_owned())]),
        ];
        assert_eq!(const_literal_text(&parts).as_deref(), Some("abc"));
    }

    #[test]
    fn const_literal_text_is_none_with_any_expansion() {
        let parts = vec![
            WordPart::Literal("a".to_owned()),
            WordPart::Param {
                name: "x".to_owned(),
            },
        ];
        assert_eq!(const_literal_text(&parts), None, "a var ⇒ not const");
        // Even a *quoted* var disqualifies const-text (it is state-dependent).
        let quoted = vec![WordPart::DoubleQuoted(vec![WordPart::Param {
            name: "x".to_owned(),
        }])];
        assert_eq!(const_literal_text(&quoted), None);
    }

    // ---- §7 field splitting (XCU §2.6.5) --------------------------------------

    #[test]
    fn split_default_ifs_basic_and_whitespace_collapse() {
        // The §2.6.5 corpus, each entry dash-verified against `cmd $V`:
        assert_eq!(split_default_ifs("a b"), vec!["a", "b"]);
        assert_eq!(split_default_ifs("a"), vec!["a"], "one field");
        // Leading / trailing / repeated separators collapse, never empty fields.
        assert_eq!(split_default_ifs(" a  b "), vec!["a", "b"]);
        // Empty value ⇒ ZERO fields (field elision — `cmd $EMPTY x` ⇒ `cmd x`).
        assert!(split_default_ifs("").is_empty(), "empty ⇒ zero fields");
        // An all-separator value ⇒ ZERO fields too (still elision when alone).
        assert!(
            split_default_ifs("   ").is_empty(),
            "all-sep alone ⇒ zero fields"
        );
        // Tab and newline are default-IFS whitespace alongside space.
        assert_eq!(split_default_ifs("a\tb\nc"), vec!["a", "b", "c"]);
        assert_eq!(split_default_ifs("\t a \n b \t"), vec!["a", "b"]);
    }

    #[test]
    fn field_is_modelable_rejects_glob_keeps_tilde() {
        assert!(field_is_modelable("nginx"));
        assert!(field_is_modelable("-y"));
        // A leading `~` in a split field is a LITERAL tilde (dash does not tilde-expand
        // split-result fields), so it is modelable — never a ⊤ trigger.
        assert!(
            field_is_modelable("~"),
            "split-field tilde is literal ⇒ modelable"
        );
        assert!(field_is_modelable("~root/x"));
        // Glob metacharacters trigger pathname expansion against the live fs ⇒ ⊤.
        assert!(!field_is_modelable("*.deb"));
        assert!(!field_is_modelable("pkg?"));
        assert!(!field_is_modelable("[abc]"));
    }

    #[test]
    fn split_fields_join_single_unquoted_var() {
        // The exactly-one-unquoted-var word `$PKGS` (PKGS="nginx curl") ⇒ two fields.
        assert_eq!(
            split_fields_join(&[Field::Split("nginx curl")]),
            Some(vec!["nginx".to_owned(), "curl".to_owned()])
        );
        // One-element value ⇒ one field (the single-operand idiom that elides).
        assert_eq!(
            split_fields_join(&[Field::Split("nginx")]),
            Some(vec!["nginx".to_owned()])
        );
        // Empty value ⇒ zero fields (the word disappears from argv — elision).
        assert_eq!(split_fields_join(&[Field::Split("")]), Some(Vec::new()));
        // All-separator value alone ⇒ zero fields.
        assert_eq!(
            split_fields_join(&[Field::Split("  \t ")]),
            Some(Vec::new())
        );
    }

    #[test]
    fn split_fields_join_mixed_literal_and_split() {
        // dash-verified field-boundary rule (T9–T13 in the strain note 20N):
        // literal joins the FIRST split field; the internal separator breaks.
        assert_eq!(
            split_fields_join(&[Field::Literal("pre"), Field::Split("nginx curl")]),
            Some(vec!["prenginx".to_owned(), "curl".to_owned()])
        );
        // Trailing literal joins the LAST split field.
        assert_eq!(
            split_fields_join(&[Field::Split("nginx curl"), Field::Literal(".deb")]),
            Some(vec!["nginx".to_owned(), "curl.deb".to_owned()])
        );
        // Both sides.
        assert_eq!(
            split_fields_join(&[
                Field::Literal("pre"),
                Field::Split("nginx curl"),
                Field::Literal(".post")
            ]),
            Some(vec!["prenginx".to_owned(), "curl.post".to_owned()])
        );
        // Single-field value in a mixed word ⇒ pure concatenation (one field).
        assert_eq!(
            split_fields_join(&[
                Field::Literal("pre"),
                Field::Split("nginx"),
                Field::Literal(".deb")
            ]),
            Some(vec!["prenginx.deb".to_owned()])
        );
        // Empty value between literals ⇒ they JOIN (no separator contributed).
        assert_eq!(
            split_fields_join(&[
                Field::Literal("pre"),
                Field::Split(""),
                Field::Literal(".post")
            ]),
            Some(vec!["pre.post".to_owned()])
        );
        // All-separator value between literals ⇒ they BREAK (internal separator run).
        assert_eq!(
            split_fields_join(&[
                Field::Literal("pre"),
                Field::Split("   "),
                Field::Literal(".post")
            ]),
            Some(vec!["pre".to_owned(), ".post".to_owned()])
        );
        // Two adjacent split vars: A's last field concatenates B's first field.
        assert_eq!(
            split_fields_join(&[Field::Split("p q"), Field::Split("r s")]),
            Some(vec!["p".to_owned(), "qr".to_owned(), "s".to_owned()])
        );
    }

    #[test]
    fn split_fields_join_pure_literal_is_one_field() {
        // No split fragment at all ⇒ the literal text is exactly one field (a fully
        // literal word never splits — IFS in literal text is not a separator).
        assert_eq!(
            split_fields_join(&[Field::Literal("a b c")]),
            Some(vec!["a b c".to_owned()]),
            "literal IFS does NOT split — only an expansion's separators do"
        );
    }

    #[test]
    fn split_fields_join_glob_field_refuses() {
        // A split field bearing a glob char ⇒ None (caller degrades the word to ⊤):
        // pathname expansion against the remote fs is unmodelable.
        assert_eq!(split_fields_join(&[Field::Split("a *.deb b")]), None);
        // The glob can be formed by a literal+split JOIN at the boundary, too.
        assert_eq!(
            split_fields_join(&[Field::Literal("*"), Field::Split("a.deb b")]),
            None,
            "the joined first field `*a.deb` is a glob ⇒ refuse"
        );
        // A glob char in a SAFE position must still refuse the whole word (any field).
        assert_eq!(split_fields_join(&[Field::Split("ok [x]")]), None);
    }

    // ---- §6 single-quote ------------------------------------------------------

    #[test]
    fn single_quote_wraps_and_escapes() {
        assert_eq!(single_quote("nginx"), "'nginx'");
        assert_eq!(single_quote("my pkg"), "'my pkg'");
        assert_eq!(single_quote("x; touch /tmp/PWNED"), "'x; touch /tmp/PWNED'");
        // An embedded single-quote: close, escaped literal quote, re-open.
        assert_eq!(single_quote("a'b"), "'a'\\''b'");
        // Two embedded quotes: open + `'\''` + `'\''` + close.
        assert_eq!(single_quote("''"), "''\\'''\\'''");
    }

    // ---- §8 word-level unquoted glob / tilde hazards (XCU §2.6.6 / §2.6.1) ----

    #[test]
    fn word_has_unquoted_glob_fires_on_bare_literal_only() {
        // An unquoted literal glob char ⇒ pathname expansion ⇒ hazard (dash globs `*.deb`).
        assert!(word_has_unquoted_glob(&[WordPart::Literal(
            "*.deb".to_owned()
        )]));
        assert!(word_has_unquoted_glob(&[WordPart::Literal(
            "pkg?".to_owned()
        )]));
        assert!(word_has_unquoted_glob(&[WordPart::Literal(
            "[abc]".to_owned()
        )]));
        // A glob char in a *mixed* word still globs (the bare `*` part is unquoted).
        assert!(word_has_unquoted_glob(&[
            WordPart::DoubleQuoted(vec![WordPart::Literal("pre".to_owned())]),
            WordPart::Literal("*.deb".to_owned()),
        ]));
        // Quoted glob chars are LITERAL to dash ⇒ NOT a hazard (`"*.conf"`, `'*'`).
        assert!(!word_has_unquoted_glob(&[WordPart::SingleQuoted(
            "*.conf".to_owned()
        )]));
        assert!(!word_has_unquoted_glob(&[WordPart::DoubleQuoted(vec![
            WordPart::Literal("*.conf".to_owned()),
        ])]));
        // A variable/subst is not a *source-literal* glob (its value-glob is the split
        // path's `field_is_modelable` concern, not this one).
        assert!(!word_has_unquoted_glob(&[WordPart::Param {
            name: "x".to_owned()
        }]));
        // A glob-free literal is fine.
        assert!(!word_has_unquoted_glob(&[WordPart::Literal(
            "nginx".to_owned()
        )]));
    }

    #[test]
    fn word_has_leading_tilde_is_word_leading_unquoted_only() {
        // Word-leading unquoted `~` ⇒ tilde expansion (dash: `~` ⇒ $HOME).
        assert!(word_has_leading_tilde(&[WordPart::Literal("~".to_owned())]));
        assert!(word_has_leading_tilde(&[WordPart::Literal(
            "~/foo".to_owned()
        )]));
        assert!(word_has_leading_tilde(&[WordPart::Literal(
            "~root".to_owned()
        )]));
        // Conservative over-degrade: `~$x` (first part is `Literal("~")`) ⇒ hazard, though
        // dash leaves it literal `~foo`. The safe direction (`inv-kfail`).
        assert!(word_has_leading_tilde(&[
            WordPart::Literal("~".to_owned()),
            WordPart::Param {
                name: "x".to_owned()
            },
        ]));
        // NOT word-leading: a mid-word tilde (`x~`) is a literal in dash.
        assert!(!word_has_leading_tilde(&[WordPart::Literal(
            "x~".to_owned()
        )]));
        // Quoted tilde ⇒ literal (`'~'`, `"~"`).
        assert!(!word_has_leading_tilde(&[WordPart::SingleQuoted(
            "~".to_owned()
        )]));
        assert!(!word_has_leading_tilde(&[WordPart::DoubleQuoted(vec![
            WordPart::Literal("~".to_owned()),
        ])]));
        // A leading variable then tilde (`$x~`) is not word-leading-tilde.
        assert!(!word_has_leading_tilde(&[
            WordPart::Param {
                name: "x".to_owned()
            },
            WordPart::Literal("~".to_owned()),
        ]));
    }
}
