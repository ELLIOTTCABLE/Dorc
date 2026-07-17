//! Derive the effect-map from the inline oracle dialect (R2, 23E §3).
//!
//! The reconciled design (`23D §1`): the check IS the oracle. Which `(provider, verb)`
//! touches which `(kind, selector)` is READ OFF the check body's own control
//! flow: the `case $verb` arms name the verbs, the inline identity annotation
//! (`pkg : package = "$1"`) names the kind, and the trailing effect mark on the
//! reached probe command (`… : package:"$pkg"#installed`) names the selector and the
//! rc convention. Nothing new is authored — the author writes idiomatic sh and Dorc
//! narrows it (`AGENTS.md`: annotation-by-narrowing, never a config surface).
//!
//! # The value-claim, not a polarity (jc-polarity-vs-rc, FINAL — human 2026-07-02)
//!
//! A property value is an OPAQUE boolean. The engine knows no "creation" or
//! "destruction"; there is NO Establish/Kill polarity. A claim only says how the
//! probe command's rc maps onto that opaque boolean: DIRECTLY (`:`), INVERTED (`:!`
//! — the `!` mark is plain rc-inversion plumbing), or as a read-only OBSERVE (`:?`,
//! which mutates nothing). Verb asymmetries (why `install` is elidable-when-converged
//! but `purge` is not) are the oracle-author's domain, expressed by WHICH arms they
//! vouch — not by an engine-side polarity. See [`ValueClaim`].
//!
//! `inv-referent-agnostic`: the walk reads the check's own *structure* (its `case`
//! arms, its annotation shape, its mark punctuation) — never the meaning of a kind /
//! entity / selector string. Those stay opaque coordination handles.

use super::ast::{MarkKind, MarkTarget, Pattern, Predict, Stmt, Word};

/// How a reached claim maps the probe command's rc onto the property's OPAQUE boolean
/// (jc-polarity-vs-rc). This REPLACES the old `Polarity{Establish, Kill, Query}`: there
/// is no create/destroy axis here, only "how does the author read the rc, and does the
/// command mutate?".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueClaim {
    /// `cmd … : k:e.p` — a write-claim whose rc reflects the property DIRECTLY
    /// (rc 0 ⇒ the property holds). The former `Establish`.
    Establish,
    /// `cmd … : k:e.p!` — a write-claim whose rc reflects the property INVERTED
    /// (the `!` mark: rc 0 ⇒ the property does NOT hold). The former `Kill`'s mark,
    /// but carrying no "kill" concept — only rc-inversion.
    ///
    /// TRANSITIONAL freeze (jc-polarity-vs-rc): a site whose reached arm carries an
    /// inverted claim classifies `MustRun` (see `analysis::effect`), so HEAD's
    /// pre-vouch-law elision machinery never begins eliding a formerly-kill site as a
    /// side effect of this re-spelling. Dissolves into the uniform no-vouch-no-elide
    /// license when the guard/vouch tier lands.
    EstablishInverted,
    /// `cmd … :? k:e.p` — a read-only OBSERVE (the guard-class). Mutates nothing, so it
    /// poisons no reaching-defs; its check IS the probe. The former `Query`.
    Observe,
}

/// One effect the derivation read off a check: which `(verb, kind, selector)` cell,
/// under which [`ValueClaim`]. `verb == None` is the ε-verb (a verbless check —
/// `command -v`, `useradd`). All strings are the check's verbatim opaque fragments;
/// the caller interns them (matching the book-side interning).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedEffect {
    /// The verb naming this cell (a `case $verb` arm literal), or `None` for a
    /// verbless check (the ε-verb).
    pub verb: Option<String>,
    /// The kind from the inline identity annotation reached on this path.
    pub kind: String,
    /// The selector from the trailing mark's `.prop`.
    pub selector: String,
    /// The rc convention / read-vs-write of the claim.
    pub claim: ValueClaim,
}

/// Derive a check's effect-map rows from its body (R2, 23E §3).
///
/// A structural walk in source order, accumulating the current annotation-kind and, on
/// a `case $verb`, the current verb. Each command carrying a trailing ESTABLISH/OBSERVE
/// mark emits one [`DerivedEffect`]; bare marks (ACK/POISON) emit nothing. Deterministic
/// and total (`inv-no-throw`: no panics — a shape the walk cannot characterize simply
/// emits nothing, the safe direction). NB the converged-vouch is no longer a mark
/// (rul24-vouch-is-verdict-authoring, 24A §1c): it is an authored `is_converged()`/
/// `is_diverged()` verdict function, unread by this derivation.
#[must_use]
pub fn derive_predict(check: &Predict) -> Vec<DerivedEffect> {
    let mut effects = Vec::new();
    let ctx = Ctx {
        verb: None,
        kind: None,
        verb_sym: check.verb_sym,
    };
    walk(&check.body, ctx, &mut effects);
    effects
}

/// The path-local accumulation context. Passed BY VALUE into every recursion, so a
/// per-arm annotation or verb binding never leaks to a sibling path.
#[derive(Clone)]
struct Ctx {
    /// The verb bound by an enclosing `case $verb` arm (`None` ⇒ ε / verbless).
    verb: Option<String>,
    /// The kind from the most recent inline annotation on this path (`None` until one
    /// is reached).
    kind: Option<String>,
    /// The check's verb-binding symbol, so a `case`'s scrutinee can be recognized as
    /// verb-dispatch (`case $verb`) vs an unrelated flag-strip (`case $1`) by symbol
    /// equality — never by decoding text (`inv-referent-agnostic`).
    verb_sym: dorc_core::Symbol,
}

fn walk(body: &[Stmt], mut ctx: Ctx, effects: &mut Vec<DerivedEffect>) {
    for stmt in body {
        match stmt {
            // An inline annotation names the kind for everything reached after it on
            // this path (shared-before-the-case, or per-arm — both fall out of source
            // order + by-value recursion).
            Stmt::Annotation(a) => ctx.kind = Some(a.kind.clone()),
            // A probe command carrying a trailing effect mark emits one cell.
            Stmt::Command(c) => {
                if let Some(mark) = &c.mark {
                    push_effect(&ctx, mark.kind, &mark.target, effects);
                }
            }
            // `case $verb`: recurse per literal-pattern arm, binding the verb. A `case`
            // on anything else (`case $1` flag-strip) recurses with the same context.
            Stmt::Case { scrutinee, arms } => {
                let is_verb_dispatch = matches!(scrutinee, Word::Var(s) if *s == ctx.verb_sym);
                for arm in arms {
                    if is_verb_dispatch {
                        for pat in &arm.patterns {
                            // Only a literal pattern names a verb; a `*` catch-all keys
                            // no verb (it is entity-resolution / fall-through), so it
                            // emits no effect row.
                            if let Pattern::Literal(v) = pat {
                                let mut arm_ctx = ctx.clone();
                                arm_ctx.verb = Some(v.clone());
                                walk(&arm.body, arm_ctx, effects);
                            }
                        }
                    } else {
                        walk(&arm.body, ctx.clone(), effects);
                    }
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                walk(then_body, ctx.clone(), effects);
                walk(else_body, ctx.clone(), effects);
            }
            Stmt::While { body, .. } => walk(body, ctx.clone(), effects),
            // Assign/Shift key no cell.
            Stmt::Assign { .. } | Stmt::Shift { .. } => {}
        }
    }
}

/// Emit an effect from a trailing mark, if it is an ESTABLISH/OBSERVE with a resolvable
/// kind + selector. A mark without a `.prop` (a kind-only POISON) or on a path with no
/// annotation-kind yet emits nothing (nothing to key — the safe direction).
fn push_effect(ctx: &Ctx, kind: MarkKind, target: &MarkTarget, effects: &mut Vec<DerivedEffect>) {
    let claim = match kind {
        MarkKind::Establish => ValueClaim::Establish,
        MarkKind::EstablishInverted => ValueClaim::EstablishInverted,
        MarkKind::Observe => ValueClaim::Observe,
    };
    let (Some(kind_str), Some(selector)) = (ctx.kind.clone(), target.prop.clone()) else {
        return;
    };
    effects.push(DerivedEffect {
        verb: ctx.verb.clone(),
        kind: kind_str,
        selector,
        claim,
    });
}

#[cfg(test)]
mod tests {
    //! Derivation coverage (23E §3): each test builds the derived cell-set from a
    //! converted-dialect check body and asserts it equals a hand-authored expected set —
    //! pinning that `derive_predict` reads the `case $verb` arms + inline annotation +
    //! trailing marks correctly for each corpus oracle shape. (These were the marker
    //! differential tests; with the markers retired the comparison target is the
    //! hand-authored cell-set, not the old `lift`.) Process-evidence, not proof
    //! (never-vouch).
    use super::*;
    use crate::predict::lift_predicts;
    use dorc_core::Interner;
    use std::collections::BTreeSet;

    /// The set key: `(verb, kind, selector, claim-label)` — the verb spelled `""` for the
    /// ε-verb, the claim as a stable label (`ValueClaim` is not `Ord`).
    type Cell = (String, String, String, &'static str);

    fn claim_label(c: ValueClaim) -> &'static str {
        match c {
            ValueClaim::Establish => "establish",
            ValueClaim::EstablishInverted => "inverted",
            ValueClaim::Observe => "observe",
        }
    }

    /// The cell-set the inline-dialect derivation produces for `provider`'s check.
    fn derived_set(dialect_src: &str, provider: &str) -> BTreeSet<Cell> {
        let mut i = Interner::default();
        let cs = lift_predicts(&mut i, dialect_src);
        assert!(cs.diags.is_empty(), "dialect lifts clean: {:?}", cs.diags);
        let sym = i.intern(provider);
        let check = cs.value.get(sym).expect("a check for the provider");
        let effects = derive_predict(check);
        effects
            .into_iter()
            .map(|e| {
                (
                    e.verb.unwrap_or_default(),
                    e.kind,
                    e.selector,
                    claim_label(e.claim),
                )
            })
            .collect()
    }

    /// Build an expected [`Cell`] set from `(verb, kind, selector, claim)` tuples.
    fn expect(cells: &[(&str, &str, &str, &'static str)]) -> BTreeSet<Cell> {
        cells
            .iter()
            .map(|(v, k, s, c)| ((*v).to_owned(), (*k).to_owned(), (*s).to_owned(), *c))
            .collect()
    }

    #[test]
    fn package_apt_get_derives_installed_cells() {
        let dialect = "\
apt_get__predict() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   pkg : package = \"$1\"
   if [ \"$2\" = \"\" ]; then
      case $verb in
         install|reinstall) dpkg-query -W \"$pkg\" >/dev/null 2>&1 : package:\"$pkg\"#installed ;;
         purge|remove) dpkg-query -W \"$pkg\" >/dev/null 2>&1 :! package:\"$pkg\"#installed ;;
      esac
   fi
}";
        assert_eq!(
            derived_set(dialect, "apt_get"),
            expect(&[
                ("install", "package", "installed", "establish"),
                ("reinstall", "package", "installed", "establish"),
                ("purge", "package", "installed", "inverted"),
                ("remove", "package", "installed", "inverted"),
            ]),
            "install/reinstall establish #installed; purge/remove invert it (the `!` mark)"
        );
    }

    #[test]
    fn service_systemctl_derives_multi_selector_cells() {
        // The multi-selector service shape: enable→#enabled, start→#active (both
        // establish), disable→#enabled INVERTED (the `!` mark).
        let dialect = "\
systemctl__predict() {
   verb=$1; shift
   svc : service = \"$1\"
   case $verb in
      enable)  systemctl is-enabled -- \"$svc\" : service:\"$svc\"#enabled ;;
      start)   systemctl is-active -- \"$svc\" : service:\"$svc\"#active ;;
      disable) systemctl is-enabled -- \"$svc\" :! service:\"$svc\"#enabled ;;
   esac
}";
        assert_eq!(
            derived_set(dialect, "systemctl"),
            expect(&[
                ("enable", "service", "enabled", "establish"),
                ("start", "service", "active", "establish"),
                ("disable", "service", "enabled", "inverted"),
            ]),
        );
    }

    #[test]
    fn tool_command_v_verbless_observe_derives_present() {
        // The verbless read-only guard: `command -v` is an OBSERVE of tool:#present on the
        // ε-verb (the `:?` mark).
        let dialect = "\
command__predict() {
   case $1 in -v) shift ;; esac
   tool : tool = \"$1\"
   command -v -- \"$tool\" >/dev/null 2>&1 :? tool:\"$tool\"#present
}";
        assert_eq!(
            derived_set(dialect, "command"),
            expect(&[("", "tool", "present", "observe")]),
        );
    }

    #[test]
    fn wildcard_arm_keys_no_verb() {
        // A `*` catch-all names no verb, so a mark under it emits no effect row (it is
        // entity-resolution / fall-through only). Pins that the walk never invents a
        // literal-`*` verb.
        let dialect = "\
apt_get__predict() {
   verb=$1; shift
   pkg : package = \"$1\"
   case $verb in
      install) dpkg-query -W \"$pkg\" : package:\"$pkg\"#installed ;;
      *) dpkg-query -W \"$pkg\" : package:\"$pkg\"#installed ;;
   esac
}";
        let set = derived_set(dialect, "apt_get");
        assert_eq!(
            set.len(),
            1,
            "only the literal `install` arm keys a cell: {set:?}"
        );
        assert!(set.iter().all(|(v, ..)| v == "install"));
    }
}
