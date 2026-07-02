//! Derive the effect-map from the inline oracle dialect (R2, 23E §3).
//!
//! The reconciled design (`23D §1`): the check IS the oracle. What the retired
//! `oracle_effect`/`oracle_kind` markers used to declare — which `(provider, verb)`
//! touches which `(kind, selector)` — is now READ OFF the check body's own control
//! flow: the `case $verb` arms name the verbs, the inline identity annotation
//! (`pkg : package = "$1"`) names the kind, and the trailing effect mark on the
//! reached probe command (`… : package:"$pkg".installed`) names the selector and the
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

use super::ast::{Check, MarkKind, MarkTarget, Pattern, Stmt, Word};

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

/// A converged-vouch the derivation read off a `: provider:verb~` bare mark
/// (`MarkKind::ConvergedVouch`) — the retired `oracle_vouch_converged=` datum. Carries
/// the two opaque fragments (provider, verb); the vouch's real sh spelling stays OPEN
/// (dq-kOOB), so this is a strawman carrier, not a committed shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedVouch {
    /// The provider fragment (`kind` slot of the two-level `provider:verb` target).
    pub provider: String,
    /// The verb fragment (`entity` slot), if the mark carried one.
    pub verb: Option<String>,
}

/// Derive a check's effect-map rows + converged-vouches from its body (R2, 23E §3).
///
/// A structural walk in source order, accumulating the current annotation-kind and, on
/// a `case $verb`, the current verb. Each command carrying a trailing ESTABLISH/OBSERVE
/// mark emits one [`DerivedEffect`]; each bare CONVERGED-VOUCH mark emits a
/// [`DerivedVouch`]. Deterministic and total (`inv-no-throw`: no panics — a shape the
/// walk cannot characterize simply emits nothing, the safe direction).
#[must_use]
pub fn derive_check(check: &Check) -> (Vec<DerivedEffect>, Vec<DerivedVouch>) {
    let mut effects = Vec::new();
    let mut vouches = Vec::new();
    let ctx = Ctx {
        verb: None,
        kind: None,
        verb_sym: check.verb_sym,
    };
    walk(&check.body, ctx, &mut effects, &mut vouches);
    (effects, vouches)
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

fn walk(
    body: &[Stmt],
    mut ctx: Ctx,
    effects: &mut Vec<DerivedEffect>,
    vouches: &mut Vec<DerivedVouch>,
) {
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
            // A bare mark: a converged-vouch is carried; POISON/ACK are no-ops this
            // round (the dead m×n negative-enumeration, 23D §5).
            Stmt::Mark(m) => {
                if m.kind == MarkKind::ConvergedVouch {
                    vouches.push(DerivedVouch {
                        provider: m.target.kind.clone(),
                        verb: m.target.entity.clone(),
                    });
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
                                walk(&arm.body, arm_ctx, effects, vouches);
                            }
                        }
                    } else {
                        walk(&arm.body, ctx.clone(), effects, vouches);
                    }
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                walk(then_body, ctx.clone(), effects, vouches);
                walk(else_body, ctx.clone(), effects, vouches);
            }
            Stmt::While { body, .. } => walk(body, ctx.clone(), effects, vouches),
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
        // ACK / POISON / CONVERGED-VOUCH never trail a probe command as an effect.
        MarkKind::Ack | MarkKind::Poison | MarkKind::ConvergedVouch => return,
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
    //! The R2 differential discipline (23E §3, the mandated safety net): for every
    //! oracle shape, the inline-dialect derivation must reproduce EXACTLY the effect-map
    //! the retired markers used to lift. Each test builds the derived cell-set from a
    //! converted-dialect check body AND the cell-set the OLD `lift` produces from the
    //! equivalent marker source, then asserts they are identical (mapping the polarity-
    //! free [`ValueClaim`] onto the old `Polarity` only for the comparison). This is
    //! process-evidence, not proof (never-vouch): it pins that the re-spelling is
    //! behaviour-preserving on these shapes, catching a wrong-derivation before any
    //! marker is deleted.
    use super::*;
    use crate::check::lift_checks;
    use crate::{Polarity, empty_verb, lift};
    use dorc_core::{Interner, ProviderId};
    use std::collections::BTreeSet;

    /// The comparison key: `(verb, kind, selector, polarity-label)` — the verb spelled
    /// `""` for the ε-verb, the polarity as a stable label (`Polarity` is not `Ord`, so
    /// a label keys the set) so both sides normalize identically.
    type Cell = (String, String, String, &'static str);

    fn claim_label(c: ValueClaim) -> &'static str {
        // Differential-comparison ONLY: the eventual lifted representation carries NO
        // polarity (jc-polarity-vs-rc). The `!`-inverted claim maps to the old `kill`
        // label so the derived set is comparable to the marker effect-map.
        match c {
            ValueClaim::Establish => "establish",
            ValueClaim::EstablishInverted => "kill",
            ValueClaim::Observe => "query",
        }
    }

    fn polarity_label(p: Polarity) -> &'static str {
        match p {
            Polarity::Establish => "establish",
            Polarity::Kill => "kill",
            Polarity::Query => "query",
        }
    }

    /// The cell-set the inline-dialect derivation produces for `provider`'s check.
    fn derived_set(dialect_src: &str, provider: &str) -> BTreeSet<Cell> {
        let mut i = Interner::default();
        let cs = lift_checks(&mut i, dialect_src);
        assert!(cs.diags.is_empty(), "dialect lifts clean: {:?}", cs.diags);
        let sym = i.intern(provider);
        let check = cs.value.get(sym).expect("a check for the provider");
        let (effects, _vouches) = derive_check(check);
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

    /// The cell-set the OLD marker lift produces for `provider`, over `verbs` (the ε-verb
    /// passed as `""`). Resolved back to strings so it is comparable to [`derived_set`].
    fn oldlift_set(marker_src: &str, provider: &str, verbs: &[&str]) -> BTreeSet<Cell> {
        let mut i = Interner::default();
        let idx = lift(&mut i, &[marker_src]);
        assert!(!idx.has_errors(), "marker src lifts: {:?}", idx.diags);
        let pid = ProviderId(i.intern(provider));
        let mut out = BTreeSet::new();
        for &v in verbs {
            let vsym = if v.is_empty() {
                empty_verb(&mut i)
            } else {
                i.intern(v)
            };
            for cell in idx.value.effect_of(pid, vsym) {
                out.insert((
                    v.to_owned(),
                    i.resolve(cell.kind.0).to_owned(),
                    i.resolve(cell.selector.0).to_owned(),
                    polarity_label(cell.polarity),
                ));
            }
        }
        out
    }

    #[test]
    fn package_apt_get_matches_markers() {
        let dialect = "\
apt-get.check() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   verb=$1; shift
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   pkg : package = \"$1\"
   if [ \"$2\" = \"\" ]; then
      case $verb in
         install|reinstall) dpkg-query -W \"$pkg\" >/dev/null 2>&1 : package:\"$pkg\".installed ;;
         purge|remove) dpkg-query -W \"$pkg\" >/dev/null 2>&1 : package:\"$pkg\".installed! ;;
      esac
   fi
}";
        let markers = "\
oracle_kind=package
oracle_probe_package() { dpkg-query -W \"$1\" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get reinstall establish installed
oracle_effect apt-get purge kill installed
oracle_effect apt-get remove kill installed
";
        let verbs = ["install", "reinstall", "purge", "remove"];
        assert_eq!(
            derived_set(dialect, "apt-get"),
            oldlift_set(markers, "apt-get", &verbs),
            "the derived package cells must equal the marker effect-map"
        );
    }

    #[test]
    fn service_systemctl_matches_markers() {
        // The multi-selector service shape: enable→#enabled, start→#active (both
        // establish), disable→#enabled INVERTED (the former Kill, now the `!` mark).
        let dialect = "\
systemctl.check() {
   verb=$1; shift
   svc : service = \"$1\"
   case $verb in
      enable)  systemctl is-enabled -- \"$svc\" : service:\"$svc\".enabled ;;
      start)   systemctl is-active -- \"$svc\" : service:\"$svc\".active ;;
      disable) systemctl is-enabled -- \"$svc\" : service:\"$svc\".enabled! ;;
   esac
}";
        let markers = "\
oracle_kind=service
oracle_probe_service_enabled() { systemctl is-enabled -- \"$1\"; }
oracle_probe_service_active() { systemctl is-active -- \"$1\"; }
oracle_effect systemctl enable establish enabled
oracle_effect systemctl start establish active
oracle_effect systemctl disable kill enabled
";
        let verbs = ["enable", "start", "disable"];
        assert_eq!(
            derived_set(dialect, "systemctl"),
            oldlift_set(markers, "systemctl", &verbs),
        );
    }

    #[test]
    fn tool_command_v_verbless_observe_matches_markers() {
        // The verbless read-only guard: `command -v` is an OBSERVE of tool:#present. The
        // ε-verb on both sides; Observe maps to the old Query polarity.
        let dialect = "\
command.check() {
   case $1 in -v) shift ;; esac
   tool : tool = \"$1\"
   command -v -- \"$tool\" >/dev/null 2>&1 :? tool:\"$tool\".present
}";
        let markers = "\
oracle_kind=tool
oracle_probe_tool() { command -v -- \"$1\" >/dev/null 2>&1; }
oracle_effect command '' query present
";
        assert_eq!(
            derived_set(dialect, "command"),
            oldlift_set(markers, "command", &[""]),
        );
    }

    #[test]
    fn converged_vouch_mark_is_derived() {
        // The retired `oracle_vouch_converged='apt-get install'` becomes a bare
        // `: apt-get:install~` on the install arm's path (23E §5, flagship).
        let dialect = "\
apt-get.check() {
   verb=$1; shift
   pkg : package = \"$1\"
   case $verb in
      install) dpkg-query -W \"$pkg\" : package:\"$pkg\".installed; : apt-get:install~ ;;
   esac
}";
        let mut i = Interner::default();
        let cs = lift_checks(&mut i, dialect);
        assert!(cs.diags.is_empty(), "{:?}", cs.diags);
        let check = cs.value.get(i.intern("apt-get")).expect("check");
        let (_effects, vouches) = derive_check(check);
        assert_eq!(
            vouches,
            vec![DerivedVouch {
                provider: "apt-get".to_owned(),
                verb: Some("install".to_owned()),
            }],
            "the converged-vouch mark lifts to a (provider, verb) vouch"
        );
    }

    #[test]
    fn wildcard_arm_keys_no_verb() {
        // A `*` catch-all names no verb, so a mark under it emits no effect row (it is
        // entity-resolution / fall-through only). Pins that the walk never invents a
        // literal-`*` verb.
        let dialect = "\
apt-get.check() {
   verb=$1; shift
   pkg : package = \"$1\"
   case $verb in
      install) dpkg-query -W \"$pkg\" : package:\"$pkg\".installed ;;
      *) dpkg-query -W \"$pkg\" : package:\"$pkg\".installed ;;
   esac
}";
        let set = derived_set(dialect, "apt-get");
        assert_eq!(
            set.len(),
            1,
            "only the literal `install` arm keys a cell: {set:?}"
        );
        assert!(set.iter().all(|(v, ..)| v == "install"));
    }
}
