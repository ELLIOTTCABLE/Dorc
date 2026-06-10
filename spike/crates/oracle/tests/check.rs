//! Adversarial integration tests for the command-keyed `check()` contract
//! (`dorc_oracle::check`, 19H §2 / 202 §1 face-check).
//!
//! Each test parses REAL dialect text (no AST-by-fiat) and pins one invariant with
//! a reasoned argument. The five 19H §2 example bodies are transcribed verbatim; the
//! assertions check exactly what the transcribed argparse code *does*, never what
//! the real tool "really" does (`inv-superposition`: the Resolution is a fact about
//! the oracle's own code over a given argv).
//!
//! Honored invariants: `inv-no-throw` (hostile input ⇒ diagnostics, never a panic),
//! `inv-kfail` (every ambiguity ⇒ Top — the safe degrade, both directions),
//! `inv-referent-agnostic` (entity is an argv *element*, never decoded for meaning),
//! `inv-determinism` (pure, ordered).

use dorc_core::Interner;
use dorc_oracle::check::{Resolution, ResolvedEntity, TopReason, evaluate, lift_checks};

/// A resolved operand entity, for terse `assert_eq!(r.entity, operand("nginx"))`.
/// (The nullary/Singleton form is asserted directly against
/// [`ResolvedEntity::Singleton`].)
fn operand(s: &str) -> ResolvedEntity {
    ResolvedEntity::Operand(s.to_owned())
}

// =============================================================================
// Transcribed 19H §2 example bodies (verbatim) and the helpers tests share.
// =============================================================================

/// 19H §2.1 — the apt-get check, transcribed verbatim from the design doc.
const APT_GET: &str = r#"
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do
      case $1 in -t|-o) shift 2 ;; *) shift ;; esac
   done
   verb=$1; shift
   pkg : com.debian.apt.Package = "$1"
   dpkg-query -W "$pkg"
}
"#;

/// 19H §2.2 — the `command -v` idempotency guard, verbatim.
const COMMAND: &str = r#"
command__check() {
   case $1 in -v) shift ;; esac
   tool : org.freedesktop.Tool = "$1"
   command -v -- "$tool" >/dev/null
}
"#;

/// 19H §2.3 — useradd: a bare-operand entity, NO verb.
const USERADD: &str = r#"
useradd__check() {
   user : org.openldap.PosixAccount = "$1"
   getent passwd "$user"
}
"#;

/// 19H §2.5 — systemctl: the verb selects a different probe per arm.
const SYSTEMCTL: &str = r#"
systemctl__check() {
   verb=$1; shift
   svc : org.freedesktop.systemd.Unit = "$1"
   case $verb in
      enable) systemctl is-enabled -- "$svc" ;;
      start)  systemctl is-active  -- "$svc" ;;
   esac
}
"#;

/// Lift one source, asserting it produced exactly one check with no diagnostics, and
/// return `(interner, source, resolution)` for the given argv. Centralizes the
/// "parse real text, then evaluate" pattern every positive test uses.
#[expect(
    clippy::panic,
    reason = "test helper: a missing lifted check is a loud test failure, not production code"
)]
fn resolve(src: &str, provider: &str, argv: &[&str]) -> Resolution {
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(
        lifted.diags.is_empty(),
        "expected a clean lift, got diagnostics: {:?}",
        lifted.diags
    );
    let provider_sym = interner.intern(provider);
    let check = lifted
        .value
        .get(provider_sym)
        .unwrap_or_else(|| panic!("no check lifted for provider `{provider}`"));
    evaluate(check, argv)
}

/// Extract the verbatim source text of every probe-body span in a resolution, so a
/// test can assert the probe round-trips span-exactly (C-1: shipped verbatim).
#[expect(
    clippy::panic,
    clippy::expect_used,
    reason = "test helper: an out-of-bounds span or unexpected Top is a loud test failure"
)]
fn probe_texts<'a>(src: &'a str, res: &Resolution) -> Vec<&'a str> {
    match res {
        Resolution::Resolved(r) => r
            .probe_body
            .iter()
            .map(|sp| {
                src.get(sp.lo.0 as usize..sp.hi.0 as usize)
                    .expect("probe span in bounds")
            })
            .collect(),
        Resolution::Top(reason) => panic!("expected Resolved, got Top({reason:?})"),
    }
}

/// Unwrap a `Resolved`, or panic with the Top reason (keeps positive tests terse).
#[expect(
    clippy::panic,
    reason = "test helper: an unexpected Top is a loud test failure, not production code"
)]
fn resolved(res: &Resolution) -> &dorc_oracle::check::Resolved {
    match res {
        Resolution::Resolved(r) => r,
        Resolution::Top(reason) => panic!("expected Resolved, got Top({reason:?})"),
    }
}

// =============================================================================
// 19H §2.1 — apt-get: the anatomy example, all argparse paths.
// =============================================================================

#[test]
fn apt_get_book_order_install_y_nginx_binds_the_flag_strain_1() {
    // STRAIN-1 (19H §2.1 internal inconsistency — recorded as a primary deliverable).
    // The book line is `apt-get install -y "$pkg"` ⇒ argv `[install, -y, nginx]` (the
    // flag is AFTER the verb). The transcribed `while [ "${1#-}" != "$1" ]` strips
    // only LEADING (pre-verb) flags: `$1=install` is not a flag, so the while exits
    // immediately; `verb=install`, `shift` ⇒ `$1=-y`; the annotation binds `-y`.
    //
    // So "what the code does" for the BOOK-ORDER argv is entity=`-y`, NOT nginx —
    // even though 19H §2.1's prose says "the while consumes `-y`, stops at install"
    // (which only holds for a flag-FIRST argv; see the next test). The prompt's rule
    // "assert exactly what its code does, not what apt-get really does" governs: we
    // pin the faithful result. This is a genuine 19H §2.1 defect, not an evaluator
    // bug — flagged for the wiring task.
    let res = resolve(APT_GET, "apt-get", &["install", "-y", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.kind, "com.debian.apt.Package");
    assert_eq!(
        r.entity,
        operand("-y"),
        "book-order argv: the post-verb `-y` lands in the entity position (strain-1)"
    );
    assert_eq!(r.verb.as_deref(), Some("install"));
    assert_eq!(probe_texts(APT_GET, &res), vec![r#"dpkg-query -W "$pkg""#]);
}

#[test]
fn apt_get_flag_first_y_install_nginx_binds_nginx() {
    // The argv ordering 19H §2.1's WALKTHROUGH actually assumes: `[-y, install,
    // nginx]` (flag FIRST). Here the `while` does consume `-y` (`${1#-}`=`y` != `-y`
    // ⇒ loop runs, `*) shift`), stops at `install`; `verb=install`, `shift`,
    // `$1=nginx`. entity=nginx — matching 19H's stated expectation. This is the
    // canonical "flag-strip then verb then operand" path the dialect is built for.
    let res = resolve(APT_GET, "apt-get", &["-y", "install", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.kind, "com.debian.apt.Package");
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb.as_deref(), Some("install"));
    assert_eq!(probe_texts(APT_GET, &res), vec![r#"dpkg-query -W "$pkg""#]);
}

#[test]
fn apt_get_pre_verb_flag_with_argument_shift_2() {
    // `[-t, exp, install, nginx]`: the `while` sees `-t` (strips to `t` != `-t` ⇒
    // loop runs), the inner `case` matches `-t` ⇒ `shift 2` consuming BOTH `-t` and
    // its argument `exp`. Next iteration: `$1=install`, `${1#-}` == `install` ⇒ loop
    // exits. `verb=install`, `shift`, `$1=nginx`. Same resolution as the simple case,
    // proving the `shift 2` path consumes the flag-argument exactly as written.
    let res = resolve(APT_GET, "apt-get", &["-t", "exp", "install", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb.as_deref(), Some("install"));
    assert_eq!(r.kind, "com.debian.apt.Package");
}

#[test]
fn apt_get_multiple_leading_flags() {
    // `[-y, -q, purge, tree]`: two single-char flags, each hitting the `*) shift`
    // arm (neither is `-t`/`-o`), then `purge` ends the loop. verb=purge, entity=tree.
    // Asserts the loop iterates correctly more than once on the default arm.
    let res = resolve(APT_GET, "apt-get", &["-y", "-q", "purge", "tree"]);
    let r = resolved(&res);
    assert_eq!(r.entity, operand("tree"));
    assert_eq!(r.verb.as_deref(), Some("purge"));
}

#[test]
fn apt_get_no_leading_flags() {
    // `[install, nginx]`: `${1#-}` == `install` == `$1` immediately ⇒ the while body
    // never runs. verb=install, shift, $1=nginx. The flag-strip loop's zero-iteration
    // path.
    let res = resolve(APT_GET, "apt-get", &["install", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb.as_deref(), Some("install"));
}

#[test]
fn apt_get_book_order_install_long_yes_binds_the_flag_strain_1() {
    // The prompt's `["install","--yes","nginx"]` variant. Book order ⇒ `--yes` is
    // post-verb: `verb=install`, `shift` ⇒ `$1=--yes` ⇒ entity=`--yes`. Same strain-1
    // shape as the `-y` case — pinned because the prompt named this argv explicitly.
    // (Assert what the code does, not what apt-get does.)
    let res = resolve(APT_GET, "apt-get", &["install", "--yes", "nginx"]);
    let r = resolved(&res);
    assert_eq!(
        r.entity,
        operand("--yes"),
        "book-order post-verb long flag binds (strain-1)"
    );
    assert_eq!(r.verb.as_deref(), Some("install"));
}

#[test]
fn apt_get_long_flag_double_dash_is_stripped_as_written() {
    // `[--yes, install, nginx]`: IMPORTANT — assert what the CODE does, not what
    // apt-get does. `${1#-}` on `--yes` strips ONE leading `-` ⇒ `-yes`, which !=
    // `--yes`, so the `while` treats `--yes` as a flag and the `*) shift` arm drops
    // it. (apt-get's real long-option `=value` grammar is NOT modeled by this check;
    // the transcribed body strips it as a plain leading-dash token.) Then `install`
    // ends the loop. entity=nginx, verb=install.
    let res = resolve(APT_GET, "apt-get", &["--yes", "install", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb.as_deref(), Some("install"));
}

// =============================================================================
// 19H §2.2 — command -v: the read-only guard, no verb bound.
// =============================================================================

#[test]
fn command_v_nginx_resolves_tool_no_verb() {
    // `[-v, nginx]`: the `case` matches `-v` ⇒ shift; `$1=nginx`. The annotation
    // binds nginx as org.freedesktop.Tool. There is NO `verb=` assignment in this
    // check, so the Resolution carries no verb — the no-verb shape is first-class.
    let res = resolve(COMMAND, "command", &["-v", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.kind, "org.freedesktop.Tool");
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb, None, "command__check binds no verb");
    assert_eq!(
        probe_texts(COMMAND, &res),
        vec![r#"command -v -- "$tool" >/dev/null"#],
        "the probe body round-trips verbatim, INCLUDING the >/dev/null redirection"
    );
}

#[test]
fn command_v_absent_flag_still_resolves_first_operand() {
    // `[nginx]` with no `-v`: the `case $1 in -v)` arm does not match, and there is
    // no `*` arm — but a `case` with no matching arm and no catch-all simply falls
    // through in sh (no effect), so evaluation continues. `$1` is still `nginx`.
    // entity=nginx. This pins that a non-matching verbless `case` is NOT a Top (it is
    // a legal fall-through), distinct from the apt-get `case` which always has `*`.
    let res = resolve(COMMAND, "command", &["nginx"]);
    let r = resolved(&res);
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb, None);
}

// =============================================================================
// 19H §2.3 — useradd: bare-operand entity, the no-verb shape the old engine missed.
// =============================================================================

#[test]
fn useradd_deploy_resolves_user_no_verb() {
    // `[deploy]`: there is no flag-strip and no `verb=` — `$1` is the bare first
    // operand and the annotation binds it directly. entity=deploy, NO verb. This is
    // exactly the shape the deleted engine-side stand-in mis-read (verb=word-1 ⇒
    // verb=deploy); the dialect resolves it correctly because the oracle's own code
    // says `$1` is the User and never binds a verb.
    let res = resolve(USERADD, "useradd", &["deploy"]);
    let r = resolved(&res);
    assert_eq!(r.kind, "org.openldap.PosixAccount");
    assert_eq!(r.entity, operand("deploy"));
    assert_eq!(
        r.verb, None,
        "useradd binds no verb — absence is first-class"
    );
    assert_eq!(probe_texts(USERADD, &res), vec![r#"getent passwd "$user""#]);
}

// =============================================================================
// Nullary / Singleton verb — `apt-get update` (the value-less annotation form).
// =============================================================================

#[test]
fn nullary_verb_value_less_annotation_resolves_singleton() {
    // `apt-get update`: a verb whose resource has NO operand (the package index as a
    // whole). The check binds `verb=update`, the `case` selects the `update` arm, and
    // the VALUE-LESS annotation `index : pkgindex` resolves the Singleton entity (no
    // operand to bind). This is the explicit nullary spelling task-W needs to key the
    // cell on `EntityRef::Singleton` (preserving `package-index#fresh` semantics).
    let src = r"
apt_get__check() {
   verb=$1
   case $verb in
      update) index : pkgindex; test -n fresh ;;
   esac
}
";
    let res = resolve(src, "apt-get", &["update"]);
    let r = resolved(&res);
    assert_eq!(r.kind, "pkgindex");
    assert_eq!(
        r.entity,
        ResolvedEntity::Singleton,
        "a value-less annotation resolves the Singleton (no-operand) entity"
    );
    assert_eq!(r.verb.as_deref(), Some("update"));
}

#[test]
fn value_less_annotation_with_equals_is_an_error() {
    // The nullary form is `name : kind` with NO `=`. A dangling `name : kind =` (an
    // `=` then no value) is malformed ⇒ a lift diagnostic, not a silent Singleton.
    // (Keeps the value-less spelling EXPLICIT and unambiguous.)
    let src = "q__check() { x : pkgindex = ; true; }";
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(
        !lifted.diags.is_empty(),
        "`name : kind =` with no value must diagnose"
    );
}

// =============================================================================
// 19H §2.5 — systemctl: the verb selects DIFFERENT probe bodies per arm.
// =============================================================================

#[test]
fn systemctl_enable_selects_is_enabled_probe() {
    // `[enable, nginx]`: verb=enable, shift, $1=nginx (annotation binds svc). The
    // `case $verb` selects the `enable)` arm ⇒ the probe is `is-enabled`. Asserts the
    // arm-selection AND verbatim round-trip of the selected body only.
    let res = resolve(SYSTEMCTL, "systemctl", &["enable", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.kind, "org.freedesktop.systemd.Unit");
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb.as_deref(), Some("enable"));
    assert_eq!(
        probe_texts(SYSTEMCTL, &res),
        vec![r#"systemctl is-enabled -- "$svc""#]
    );
}

#[test]
fn systemctl_start_selects_is_active_probe() {
    // `[start, nginx]`: same entity, DIFFERENT verb ⇒ DIFFERENT probe (`is-active`).
    // The selector-from-verb mechanism (19H §2.5): enable and start touch different
    // cells; the check carries a distinct probe per arm and the evaluator ships the
    // one the selected path reaches.
    let res = resolve(SYSTEMCTL, "systemctl", &["start", "nginx"]);
    let r = resolved(&res);
    assert_eq!(r.entity, operand("nginx"));
    assert_eq!(r.verb.as_deref(), Some("start"));
    assert_eq!(
        probe_texts(SYSTEMCTL, &res),
        vec![r#"systemctl is-active  -- "$svc""#],
        "the is-active body round-trips verbatim (note the doubled space preserved)"
    );
}

#[test]
fn systemctl_unknown_verb_is_top_no_probe() {
    // `[restart, nginx]`: verb=restart; the `case $verb` has arms only for `enable`
    // and `start` and NO `*` catch-all. Per sh, the case falls through harmlessly —
    // but the probe commands live INSIDE the arms, so the selected path runs NO
    // probe. The annotation *was* reached (svc=nginx) but a probe-less resolution is
    // not actionable ⇒ Top(NoProbeReached). Biasing to Top is the safe degrade
    // (kFAIL): an unknown verb must not silently resolve to one arm's probe, nor to a
    // shippable-but-empty probe.
    let res = resolve(SYSTEMCTL, "systemctl", &["restart", "nginx"]);
    assert_eq!(res, Resolution::Top(TopReason::NoProbeReached));
}

// =============================================================================
// 19H §2.4 — cross-oracle identity: two providers, one kind, in one file.
// =============================================================================

#[test]
fn cross_oracle_two_providers_share_one_kind() {
    // Two command-keyed checks in one file resolving to the SAME named kind via
    // different argparses (apt-get's `$1`, dnf's `$2`). Pins: (a) both lift from one
    // file, keyed by their (hyphen-mapped) providers; (b) the kind string is the
    // shared coordination handle (19H §2.4) — the evaluator never decodes it, it just
    // carries the verbatim reverse-DNS string through. The differing argparse is why
    // identity is command-keyed while the kind is cross-oracle.
    let src = r#"
apt_get__check() {
   verb=$1; shift
   pkg : com.debian.apt.Package = "$1"
   dpkg-query -W "$pkg"
}
dnf__check() {
   verb=$1
   pkg : com.debian.apt.Package = "$2"
   rpm -q "$pkg"
}
"#;
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(lifted.diags.is_empty(), "{:?}", lifted.diags);
    assert_eq!(lifted.value.len(), 2);

    let apt = lifted
        .value
        .get(interner.intern("apt-get"))
        .expect("apt-get check lifted");
    let apt_res = evaluate(apt, &["install", "nginx"]);
    let r_apt = resolved(&apt_res);
    assert_eq!(r_apt.entity, operand("nginx"));
    assert_eq!(r_apt.kind, "com.debian.apt.Package");

    // dnf binds the entity to `$2` (not `$1`): `[install, nginx]` ⇒ entity=nginx via
    // a *different* argparse, same kind. verb=install (dnf binds it but does not
    // shift — the annotation reads $2 regardless).
    let dnf = lifted
        .value
        .get(interner.intern("dnf"))
        .expect("dnf check lifted");
    let dnf_res = evaluate(dnf, &["install", "nginx"]);
    let r_dnf = resolved(&dnf_res);
    assert_eq!(
        r_dnf.entity,
        operand("nginx"),
        "dnf resolves the entity via $2"
    );
    assert_eq!(r_dnf.kind, "com.debian.apt.Package", "same shared kind");
}

// =============================================================================
// Top cases — the safe degrade (`inv-kfail`), exhaustively.
// =============================================================================

#[test]
fn empty_argv_is_top() {
    // No command for the argparse to consume ⇒ EmptyArgv. (The engine receives full
    // verbatim args sans the command word; an empty arg list is a real possibility,
    // e.g. a bare `apt-get` with no operands, and must not resolve.)
    let res = resolve(APT_GET, "apt-get", &[]);
    assert_eq!(res, Resolution::Top(TopReason::EmptyArgv));
}

#[test]
fn annotation_positional_past_end_is_top() {
    // A check whose annotation reads `$3` evaluated over a 2-element argv: the value
    // position resolves to a positional past the end ⇒ Top. The disaster to avoid is
    // resolving to a stale/empty entity; biasing to Top is correct.
    let src = r#"
foo__check() {
   x : com.example.Thing = "$3"
   true
}
"#;
    let res = resolve(src, "foo", &["a", "b"]);
    assert_eq!(res, Resolution::Top(TopReason::UnresolvedAnnotationValue));
}

#[test]
fn missing_annotation_is_top() {
    // A well-formed dialect body with NO inline annotation cannot resolve an entity
    // ⇒ Top(MissingAnnotation). (A check that argparses but never declares which value
    // is the entity is useless to entity-resolution; it must degrade, not guess.)
    let src = r#"
bar__check() {
   verb=$1; shift
   some-probe "$1"
}
"#;
    let res = resolve(src, "bar", &["install", "nginx"]);
    assert_eq!(res, Resolution::Top(TopReason::MissingAnnotation));
}

#[test]
fn unbound_variable_in_annotation_is_top() {
    // The annotation value references a variable that was never assigned ⇒ the word
    // does not resolve concretely ⇒ Top. (`$missing` is not a positional and has no
    // binding.) Pins that an undefined var never silently becomes empty-string.
    let src = r#"
baz__check() {
   x : com.example.K = "$undefined"
   true
}
"#;
    let res = resolve(src, "baz", &["a"]);
    assert_eq!(res, Resolution::Top(TopReason::UnresolvedAnnotationValue));
}

#[test]
fn shift_past_end_is_top() {
    // A `shift 5` over a 2-element argv is a runtime error in sh; the evaluator
    // degrades to Top rather than inventing positionals. (Reaching the annotation
    // after an over-shift would otherwise mis-resolve.)
    let src = r#"
q__check() {
   shift 5
   x : com.example.K = "$1"
   true
}
"#;
    let res = resolve(src, "q", &["a", "b"]);
    assert!(matches!(res, Resolution::Top(_)), "got {res:?}");
}

#[test]
fn unconsumed_flag_reaches_annotation_position() {
    // A check that does NOT strip flags: the annotation reads `$1` directly. Over
    // `[-y, nginx]`, `$1` is `-y` — the flag itself lands in the entity position. The
    // evaluator resolves entity="-y" (it faithfully runs the code, which never
    // stripped the flag). This is the CORRECT behavior for THIS (deliberately weak)
    // check: the resolution reflects exactly what the oracle's argparse does. A
    // *real* oracle would strip the flag (as APT_GET does); a check that doesn't gets
    // the literal flag as its entity — and the test pins that we do not second-guess
    // the oracle (`inv-referent-agnostic`: the engine parses NOTHING on its own).
    let src = r#"
naive__check() {
   x : com.example.K = "$1"
   probe "$x"
}
"#;
    let res = resolve(src, "naive", &["-y", "nginx"]);
    let r = resolved(&res);
    assert_eq!(
        r.entity,
        operand("-y"),
        "the check never stripped the flag, so $1 is the flag — we run its code as written"
    );
}

#[test]
fn budget_bounds_a_nonterminating_loop() {
    // A while-loop whose test never becomes false and whose body never shifts: in a
    // real shell this is an infinite loop. The evaluator's iteration budget bounds it
    // ⇒ Top(BudgetExceeded), never a hang. Constructed with `[ x = x ]` (always
    // true) and an empty-ish body that does not consume args. This directly exercises
    // the budget mechanism (the loops the dialect *usually* admits terminate by
    // construction, so we must construct a pathological one to test the guard).
    let src = r#"
loopy__check() {
   while [ "$1" = "$1" ]; do
      probe-step
   done
   x : com.example.K = "$1"
   true
}
"#;
    let res = resolve(src, "loopy", &["a", "b", "c"]);
    assert_eq!(res, Resolution::Top(TopReason::BudgetExceeded));
}

#[test]
fn test_context_past_end_positional_is_empty_string() {
    // sh semantics inside `[ … ]`: an unset/past-end positional expands to the empty
    // string, NOT a degrade. This is load-bearing for the corpus apt check, which uses
    // (a) a post-verb flag-strip `while [ "${1#-}" != "$1" ]` that must TERMINATE when
    // the argv is exhausted, and (b) a single-operand guard `[ "$2" = "" ]` that gates
    // the probe (so `install nginx curl` — a SECOND operand — reaches no probe ⇒ Top ⇒
    // runs, never a wrong single-entity elision). Here: a check whose `if [ "$2" = "" ]`
    // gates the probe resolves `[nginx]` (one operand, `$2` empty ⇒ probe runs) but
    // degrades `[nginx, curl]` (a second operand ⇒ no probe ⇒ NoProbeReached).
    let src = r#"
pkgone__check() {
   pkg : package = "$1"
   if [ "$2" = "" ]; then probe "$pkg"; fi
}
"#;
    let one = resolve(src, "pkgone", &["nginx"]);
    assert_eq!(
        resolved(&one).entity,
        operand("nginx"),
        "one operand: `$2` is empty (sh) ⇒ the guard passes ⇒ probe reached ⇒ Resolved"
    );
    let two = resolve(src, "pkgone", &["nginx", "curl"]);
    assert_eq!(
        two,
        Resolution::Top(TopReason::NoProbeReached),
        "a SECOND operand ⇒ `[ \"$2\" = \"\" ]` false ⇒ no probe ⇒ Top (the multi-operand refusal)"
    );
}

#[test]
fn naive_oracle_without_operand_guard_drops_trailing_operands_known_hazard() {
    // KNOWN HAZARD — pinned, NOT fixed (20I §3 find-3 / 208 strain-W3). A NAIVE oracle
    // that binds `pkg : package = "$1"` WITHOUT the single-operand guard `[ "$2" = "" ]`
    // resolves `apt-get install nginx curl` to entity=nginx and ships a probe for nginx
    // ALONE — silently DROPPING curl. If the host has nginx but not curl, the probe says
    // `holds`, the apply elides the whole `install nginx curl`, and curl is NEVER
    // installed: a priority-1 UNDER-EXECUTE.
    //
    // This is NOT an engine bug to fix here: the engine parses nothing
    // (`inv-referent-agnostic`); the multi-operand refusal is the ORACLE's job, spelled
    // `if [ "$2" = "" ]; then probe "$pkg"; fi` (which degrades a 2nd-operand argv to
    // Top ⇒ run — see `test_context_past_end_positional_is_empty_string`). The defense
    // lives in the oracle-quality bar (oracle/CLAUDE.md), and the example authors copy
    // (19H §2.1's annotation). This test pins the naive drop as a DATUM so it can't be
    // mistaken for correct, and so a future engine-side "fix" (which would re-introduce
    // the deleted engine-side argparse) is visibly the wrong layer.
    let src = r#"
naive__check() {
   pkg : com.debian.apt.Package = "$1"
   dpkg-query -W "$pkg"
}
"#;
    let res = resolve(src, "naive", &["install", "nginx", "curl"]);
    let r = resolved(&res);
    // The hazard, made concrete: entity = the FIRST operand (`install`, here — the check
    // has no verb-strip either), and the trailing operands are simply gone. The probe
    // resolves and would license eliding a multi-target install. SOUND oracles add the
    // guard; this one didn't, and the drop is silent.
    assert_eq!(
        r.entity,
        operand("install"),
        "KNOWN HAZARD: the unguarded check binds $1 and drops every trailing operand"
    );
    assert_eq!(
        probe_texts(src, &res),
        vec![r#"dpkg-query -W "$pkg""#],
        "…and it SHIPS a probe (resolvable ⇒ elidable) — the under-execute surface"
    );
}

// =============================================================================
// Quoting — `"$1"` vs `$1` vs `'$1'` in the annotation value-position.
// =============================================================================

#[test]
fn double_quoted_positional_is_a_positional() {
    // `"$1"` resolves to the first argv element (the canonical idiom). Already
    // exercised by every §2 example; pinned here in isolation for the quoting matrix.
    let src = r#"q__check() { x : K = "$1"; true; }"#;
    let res = resolve(src, "q", &["nginx"]);
    assert_eq!(resolved(&res).entity, operand("nginx"));
}

#[test]
fn bare_unquoted_positional_is_a_positional() {
    // `$1` (unquoted) resolves identically to `"$1"` for a single-token value. (Field
    // splitting/globbing on an unquoted expansion is not modeled — and irrelevant for
    // a single concrete argv element. The dialect treats `$1` and `"$1"` the same.)
    let src = "q__check() { x : K = $1; true; }";
    let res = resolve(src, "q", &["nginx"]);
    assert_eq!(resolved(&res).entity, operand("nginx"));
}

#[test]
fn single_quoted_dollar_one_is_a_literal_not_a_positional() {
    // `'$1'` is the LITERAL string `$1` in sh (single quotes suppress expansion), NOT
    // the first argument. The evaluator resolves entity to the literal "$1". This is
    // the documented choice (Word::SingleQuotedLiteral): a single-quoted value is a
    // literal, so the entity is the two-character string `$1`, regardless of argv.
    let src = "q__check() { x : K = '$1'; true; }";
    let res = resolve(src, "q", &["nginx"]);
    assert_eq!(
        resolved(&res).entity,
        operand("$1"),
        "single-quoted '$1' is the literal dollar-one string, not the positional"
    );
}

// =============================================================================
// Fail-soft + out-of-dialect (per-function lift diagnostic, never a panic).
// =============================================================================

#[test]
fn half_garbage_file_lifts_the_good_check() {
    // One good check + one out-of-dialect check in the same file. Fail-soft
    // (`inv-no-throw`): the bad one yields a diagnostic and contributes nothing; the
    // good one still lifts. Pins that a single malformed function does not poison the
    // whole file.
    let src = r#"
good__check() {
   x : com.example.K = "$1"
   probe "$x"
}
bad__check() {
   x=`hostname`
   y : com.example.K = "$1"
}
"#;
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(
        !lifted.diags.is_empty(),
        "the bad check must produce a diagnostic"
    );
    assert_eq!(lifted.value.len(), 1, "only the good check lifts");
    assert!(lifted.value.get(interner.intern("good")).is_some());
    assert!(
        lifted.value.get(interner.intern("bad")).is_none(),
        "the backtick-using check is out of dialect and contributes nothing"
    );
}

#[test]
fn backtick_command_substitution_is_out_of_dialect() {
    // Backticks (command substitution) are not in the dialect ⇒ lift diagnostic, no
    // check, no panic.
    let src = "c__check() { x=`whoami`; y : K = \"$1\"; }";
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(!lifted.diags.is_empty());
    assert!(lifted.value.is_empty());
}

#[test]
fn dollar_paren_command_substitution_is_out_of_dialect() {
    // `$(...)` command substitution — the modern spelling — is likewise rejected.
    let src = r#"c__check() { x=$(whoami); y : K = "$1"; }"#;
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(!lifted.diags.is_empty(), "$(...) must be out of dialect");
    assert!(lifted.value.is_empty());
}

#[test]
fn eval_construct_is_out_of_dialect() {
    // `eval` is a plain command to the parser, so it would lift as a probe body. But
    // the point of the dialect is that the *control constructs* are constrained — a
    // `for` loop is the real out-of-dialect risk. `eval` as a bare command is not
    // specially rejected (it is just a command word); document that here so the
    // wiring task knows `eval` in a probe body is NOT caught by the parser — it would
    // be caught (if at all) by the separate reflexive-inertness check
    // (dq-reflexive-probe-inertness, 19H §1.3), which is out of scope for this
    // module. We assert the parser's actual behavior, not an aspiration.
    let src = r#"c__check() { x : K = "$1"; eval "$x"; }"#;
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    // It lifts (eval is a command word); the inertness gate is a different component.
    assert_eq!(
        lifted.value.len(),
        1,
        "eval-as-command is not the parser's gate"
    );
}

#[test]
fn bare_for_loop_is_out_of_dialect() {
    // A `for` loop is NOT in the dialect (only `while`/`case`/`if`). The book parser
    // ⊤-rejects loops by design; this dialect admits only the flag-strip `while`. A
    // `for` ⇒ lift diagnostic, no check.
    let src = r#"
c__check() {
   for a in 1 2 3; do probe "$a"; done
   x : K = "$1"
}
"#;
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(!lifted.diags.is_empty(), "for-loops are out of dialect");
    assert!(lifted.value.is_empty());
}

#[test]
fn non_check_functions_are_ignored_not_errors() {
    // A file with `oracle_kind=`, a helper function, and a real check. The non-check
    // top-level items are ignored (this module only owns `__check`); only the check
    // lifts, with no spurious diagnostics. Pins coexistence with the existing
    // `oracle_kind`/`oracle_effect`/helper content (the existing `lift` owns those).
    let src = r#"
oracle_kind=package
helper() { echo hi; }
apt_get__check() {
   verb=$1; shift
   pkg : com.debian.apt.Package = "$1"
   dpkg-query -W "$pkg"
}
"#;
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, src);
    assert!(
        lifted.diags.is_empty(),
        "non-check items must be ignored silently: {:?}",
        lifted.diags
    );
    assert_eq!(lifted.value.len(), 1);
}

// =============================================================================
// Determinism + no-throw property: hostile garbage never panics, is bounded.
// =============================================================================

#[test]
fn hostile_garbage_never_panics() {
    // The no-throw property (`inv-no-throw`): feed the parser a battery of hostile
    // inputs — NUL bytes, unterminated quotes, deep nesting, lone metacharacters,
    // truncated headers — and assert only that it RETURNS (a Carrier, possibly with
    // diagnostics), never panics and never hangs. We do not assert specific outputs;
    // the property is totality + termination.
    let hostile: &[&str] = &[
        "",
        "\0\0\0",
        "x__check() {",
        "x__check() { \"unterminated",
        "x__check() { 'unterminated",
        "x__check() { case",
        "x__check() { while",
        "x__check() { ${",
        "x__check() { $(((((",
        "x__check() { [ [ [ [ [",
        "}}}}}}}}",
        ";;;;;;;;",
        "((((((((",
        "x__check() { x : : : = = = }",
        &"{".repeat(5000),
        &"x__check() { while [ \"$1\" != \"$1\" ]; do ".repeat(200),
        "\u{feff}x__check() { x : K = \"$1\"; }", // BOM prefix
        "𝓊𝓃𝒾𝒸ℴ𝒹ℯ__check() { x : K = \"$1\"; }",   // multibyte name
    ];
    for src in hostile {
        let mut interner = Interner::default();
        // The mere fact that this returns (no panic, no hang) is the assertion.
        let lifted = lift_checks(&mut interner, src);
        // And any lifted check must itself evaluate without panicking on any argv.
        for provider in lifted.value.providers().collect::<Vec<_>>() {
            if let Some(check) = lifted.value.get(provider) {
                let _ = evaluate(check, &[]);
                let _ = evaluate(check, &["a"]);
                let _ = evaluate(check, &["-x", "y", "z"]);
            }
        }
    }
}

#[test]
fn lift_is_deterministic() {
    // `inv-determinism`: lifting the same source twice yields the same providers in
    // the same order (BTreeMap-backed). Pins that nothing hashes into observable
    // output.
    let mut i1 = Interner::default();
    let mut i2 = Interner::default();
    let a = lift_checks(&mut i1, SYSTEMCTL);
    let b = lift_checks(&mut i2, SYSTEMCTL);
    let pa: Vec<_> = a
        .value
        .providers()
        .map(|s| i1.resolve(s).to_owned())
        .collect();
    let pb: Vec<_> = b
        .value
        .providers()
        .map(|s| i2.resolve(s).to_owned())
        .collect();
    assert_eq!(pa, pb);
}

// =============================================================================
// Provider-name underscore↔hyphen mapping (the tc-* flagged rule).
// =============================================================================

#[test]
fn provider_name_underscore_maps_to_hyphen() {
    // `apt_get__check` ⇒ provider `apt-get` (underscore→hyphen). This is the chosen
    // rule (flagged tc-*); pin it so a future change is visible. A single-segment
    // name (`command__check` ⇒ `command`) has no underscore to map.
    let mut interner = Interner::default();
    let lifted = lift_checks(&mut interner, APT_GET);
    assert!(
        lifted.value.get(interner.intern("apt-get")).is_some(),
        "apt_get__check must key on provider `apt-get`"
    );
    assert!(
        lifted.value.get(interner.intern("apt_get")).is_none(),
        "the underscore form must NOT be the key"
    );
}

// =============================================================================
// Round-20 crosscheck finding 2 — globby/longest-match prefix-strips diverge from
// dash (fnmatch vs literal strip) and must be Unmodeled ⇒ Top in EVERY position.
// =============================================================================

/// A check whose annotation value uses a GLOB prefix-strip (`${1#*=}` — dash strips
/// up to the first `=` by fnmatch; a literal strip of `*=` matches nothing).
const GLOB_ANNO: &str = r#"
flagged__check() {
   pkg : package = "${1#*=}"
   dpkg-query -W "$pkg"
}
"#;

/// A check using the `##` longest-match form (`${1##*/}` — dash basename). The
/// naive `split_once('#')` parse would mangle it into a literal `#*/` strip.
const HASHHASH_ANNO: &str = r#"
based__check() {
   pkg : package = "${1##*/}"
   dpkg-query -W "$pkg"
}
"#;

/// A check whose `[ ]` TEST uses a globby strip — the position where a Literal
/// fallback would have silently compared the raw `${1#*=}` text (wrong concrete).
const GLOB_TEST: &str = r#"
globtest__check() {
   while [ "${1#-*}" != "$1" ]; do shift; done
   pkg : package = "$1"
   dpkg-query -W "$pkg"
}
"#;

#[test]
fn glob_prefix_strip_in_annotation_is_top() {
    // dash: `--pkg=nginx` ⇒ `${1#*=}` ⇒ `nginx`. A literal strip would keep
    // `--pkg=nginx` — a wrong concrete entity. The dialect must refuse: Top.
    let res = resolve(GLOB_ANNO, "flagged", &["--pkg=nginx"]);
    assert!(
        matches!(res, Resolution::Top(_)),
        "a globby `${{N#pat}}` must be Unmodeled ⇒ Top, got {res:?}"
    );
}

#[test]
fn hashhash_longest_match_strip_is_top() {
    // dash: `/usr/bin/dpkg` ⇒ `${1##*/}` ⇒ `dpkg`. The mangled-literal parse kept
    // the path unchanged — a wrong concrete. Must be Top.
    let res = resolve(HASHHASH_ANNO, "based", &["/usr/bin/dpkg"]);
    assert!(
        matches!(res, Resolution::Top(_)),
        "`${{N##pat}}` longest-match is out of dialect ⇒ Top, got {res:?}"
    );
}

#[test]
fn glob_prefix_strip_in_test_position_is_top() {
    // The sharper pole: in `[ ]`-test position a Literal fallback would COMPARE the
    // raw `${1#-*}` text against `$1` (a concrete comparison dash disagrees with)
    // and the loop would mis-decide. Unmodeled fails the test resolution ⇒ the
    // whole check degrades to Top — never a wrong branch decision.
    let res = resolve(GLOB_TEST, "globtest", &["-y", "nginx"]);
    assert!(
        matches!(res, Resolution::Top(_)),
        "a globby strip in test-position must degrade the check to Top, got {res:?}"
    );
}
