//! `render_corpus` — the in-memory RENDER-SHAPE tier (`27D`/`24I` batch-3 e2e de-graduation). The
//! DEGRADE e2e cases (small precise render/fold logic-tests that did not need real-shell-ness) move
//! here as string-asserting in-process twins: parse → cfg → classify → build_plan → `render_apply`,
//! asserting the rendered apply artifact's SHAPE plus the structural `Disposition`. Each twin
//! carries the same end-state its retired e2e case pinned.
//!
//! # THE dash -n net (`24I` design-flag — carried loudly)
//!
//! The in-memory twins ARE the ap-2 trap at scale: `observable_matrix.rs` render-asserts via
//! `.contains()` with NO shell check, and that text-diff blindness shipped non-runnable sh green
//! TWICE historically (an empty `then`-clause; a stray heredoc body). So this tier's harness runs
//! one-shot `dash -n` (else `sh -n`) on EVERY rendered artifact — the cheap 90% of ap-2, no mock
//! machinery, a pure syntax parse. It is wired into [`render_for`] itself so a twin CANNOT forget
//! it, and [`net_catches_a_deliberately_broken_render`] proves it screams. Non-negotiable.

// Tests are sanctioned to `expect()` (workspace `[lints]`: "tests can opt out per-item with a
// justified `#[expect]`") — a failed harness precondition SHOULD panic loudly here; and the module
// doc names sibling test items in prose without intra-doc-link backticks (cosmetic in a test file).
#![expect(
    clippy::expect_used,
    clippy::doc_markdown,
    reason = "test harness: a failed precondition panics loudly; doc prose names sibling tests"
)]

use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_core::{
    EntityRef, Interner, KindId, Observable, OpaqueToken, Predicted, ProviderId, Rc, SelectorId,
    Verdict,
};
use dorc_oracle::{KindIndex, ValueClaim};
use dorc_plan::{Disposition, Plan, SurvivalReport, build_plan};
use std::io::Write;
use std::process::{Command, Stdio};

/// Corpus-shaped apt-get predict (flag-strip → verb → single-operand `package` bind with a
/// multi-operand refusal). The matrix only models install/purge on `package`. Lifted with the
/// test's interner so provider symbols match the book's command words. (Mirrors
/// `observable_matrix.rs`; test repetition is fine — the human's style guide.)
const CORPUS_PREDICT_SRC: &str = r#"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then probe-pkg "$pkg"; fi
}
"#;

/// The package oracle: `apt-get install ⇒ establishes package@installed`, `purge ⇒ inverts`.
fn package_index(i: &mut Interner) -> KindIndex {
    let package = KindId(i.intern("package"));
    let installed = SelectorId(i.intern("installed"));
    let apt = ProviderId(i.intern("apt_get"));
    let install = i.intern("install");
    let purge = i.intern("purge");
    let mut idx = KindIndex::default();
    idx.add_effect(0, apt, install, package, installed, ValueClaim::Establish);
    idx.add_effect(
        0,
        apt,
        purge,
        package,
        installed,
        ValueClaim::EstablishInverted,
    );
    idx
}

/// Run value-flow + the given predict sources + classify, returning the classified leaves.
fn classify_with(
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    predict_srcs: &[&str],
    i: &mut Interner,
) -> (
    Vec<(dorc_analysis::cfg::CfgNodeId, SkipClass)>,
    std::collections::BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    dorc_analysis::value::ValueFlow,
) {
    let value = dorc_analysis::value::analyze(cfg, ast, i);
    let checks: Vec<_> = predict_srcs
        .iter()
        .map(|s| dorc_oracle::predict::lift_predicts(i, s).value)
        .collect();
    let mut arena = dorc_core::ProvArena::new();
    let classification = dorc_analysis::effect::classify(
        cfg,
        &value,
        ast,
        idx,
        &checks,
        &dorc_oracle::verdict::VerdictIndex::default(),
        i,
        &mut arena,
    );
    let classes = classification.value;
    let invalidators = classification.invalidators;
    (classes, invalidators, value)
}

const CORPUS_VERDICT_SRC: &str = r"
apt_get__is_converged() { return 0; }
";

#[derive(Clone, Copy, PartialEq, Eq)]
enum VouchMode {
    None,
    Reached,
}

fn vouch_all(
    classes: &[(dorc_analysis::cfg::CfgNodeId, SkipClass)],
    value: &dorc_analysis::value::ValueFlow,
    mode: VouchMode,
    i: &mut Interner,
) -> dorc_plan::Vouches {
    match mode {
        VouchMode::None => dorc_plan::Vouches::new(),
        VouchMode::Reached => {
            dorc_plan::build_vouches(
                &[CORPUS_VERDICT_SRC, SERVICE_VERDICT_SRC, YUM_VERDICT_SRC],
                &[],
                &dorc_oracle::closure::HelperIndex::default(),
                classes,
                value,
                i,
                dorc_analysis::funcenv::LiveDefinitions::unsolved(),
            )
            .0
            .value
        }
    }
}

/// Parse → classify → build_plan → `render_apply` a book, with `holds` the injected host state (a
/// listed `(kind, entity)` cell is Converged, anything else Diverged — the invisible global
/// convergence state). Returns the RENDERED apply artifact plus the `Plan` (for structural
/// disposition asserts). THE dash -n net fires here, on every rendered artifact, so no twin can skip
/// it.
fn render_for(src: &str, holds: &[(&str, &str)]) -> (String, Plan) {
    render_for_mode(src, holds, VouchMode::Reached)
}

/// [`render_for`] with an explicit [`VouchMode`] (the package-oracle world). `VouchMode::None` models
/// an oracle that authored no verdict body — a converged install then RUNS (the `guard23-no-vouch-runs`
/// no-vouch floor).
fn render_for_mode(src: &str, holds: &[(&str, &str)], mode: VouchMode) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let held: Vec<FactKey> = holds
        .iter()
        .map(|(k, e)| FactKey {
            kind: KindId(i.intern(k)),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: installed,
            context: dorc_core::Context::HostDefault,
        })
        .collect();
    render_core(src, &[CORPUS_PREDICT_SRC], &idx, held, mode, &mut i)
}

/// The shared pipeline (`render_for`'s package-oracle specialization, the service/seam/singleton
/// harnesses below, all funnel here): parse → classify (with `predict_srcs` + `idx`) → build_plan →
/// `render_apply`, observing `held` (each listed `FactKey` cell Converged, else Diverged). Vouches
/// every ambient establish (elision MECHANICS; the vouch GATE is pinned elsewhere). THE dash -n net
/// fires on the rendered artifact so no twin can skip it.
fn render_core(
    src: &str,
    predict_srcs: &[&str],
    idx: &KindIndex,
    held: Vec<FactKey>,
    mode: VouchMode,
    i: &mut Interner,
) -> (String, Plan) {
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let (classes, invalidators, value) = classify_with(&cfg, &parsed.value, idx, predict_srcs, i);
    let observe = move |f: FactKey| {
        if held.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    };
    let plan = build_plan(
        src,
        &parsed.value,
        &cfg,
        &classes,
        &invalidators,
        &vouch_all(&classes, &value, mode, i),
        observe,
        &mut dorc_core::ProvArena::new(),
    );
    let rendered = plan.render_apply(src, &parsed.value);
    assert_runnable(&rendered);
    (rendered, plan)
}

/// THE ap-2 net: syntax-check a rendered artifact with `dash -n` (else `sh -n`). No mock machinery —
/// a pure parse. Panics loudly if the artifact does not parse (the twin's failure), or if no POSIX
/// shell is available (the net is non-negotiable — a silent skip would reopen the trap).
fn assert_runnable(rendered: &str) {
    let (shell, out) = run_syntax_check(rendered).expect(
        "a POSIX shell (dash or sh) is required for the ap-2 dash -n net — none found on PATH",
    );
    assert!(
        out.status.success(),
        "ap-2: the rendered artifact is not `{shell} -n` clean (non-runnable sh — the trap that \
         shipped green twice):\n{rendered}\n--- {shell} -n stderr ---\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Try `dash -n`, then `sh -n`, feeding `rendered` on stdin. Returns the shell name + its output, or
/// `None` when neither shell can be spawned.
fn run_syntax_check(rendered: &str) -> Option<(&'static str, std::process::Output)> {
    // Resolved by `internal-tooling`, not searched for on PATH: native Windows has no
    // POSIX shell there, so this net silently could never run — the failure it produced
    // was invisible behind the e2e runner refusing earlier in the same suite.
    let posix = internal_tooling::Posix::find().ok()?;
    let mut child = Command::new(&posix.shell)
        .arg("-n")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(rendered.as_bytes());
    }
    child.wait_with_output().ok().map(|out| (posix.name, out))
}

/// The systemd service oracle (`enable ⇒ service@enabled`, `start ⇒ service@active`), simplified-kind
/// like `CORPUS_PREDICT_SRC`. Effects come from `service_index`; this only argv-parses verb + operand.
const SERVICE_PREDICT_SRC: &str = r#"
systemctl__predict() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable) systemctl is-enabled -- "$svc" ;;
      start)  systemctl is-active  -- "$svc" ;;
   esac
}
"#;

const SERVICE_VERDICT_SRC: &str = r"
systemctl__is_converged() { return 0; }
";

/// `package_index` + the service oracle: `enable ⇒ service@enabled`, `start ⇒ service@active` — two
/// DISTINCT selectors of one Service entity (enabling ≠ activating; that distinctness is the point of
/// the exec-distinct-selectors / exec-enabled-not-active twins).
fn service_index(i: &mut Interner) -> KindIndex {
    let mut idx = package_index(i);
    let service = KindId(i.intern("service"));
    let enabled = SelectorId(i.intern("enabled"));
    let active = SelectorId(i.intern("active"));
    let systemctl = ProviderId(i.intern("systemctl"));
    let enable = i.intern("enable");
    let start = i.intern("start");
    // File 1: `SERVICE_PREDICT_SRC` is the SECOND source `render_service_for` hands the classifier,
    // and since `28Q` §1.1 a cell answers only at the file whose definition the frame names. A row
    // keyed to file 0 here would be a row the systemctl site can never read.
    idx.add_effect(
        1,
        systemctl,
        enable,
        service,
        enabled,
        ValueClaim::Establish,
    );
    idx.add_effect(1, systemctl, start, service, active, ValueClaim::Establish);
    idx
}

/// Render harness for the package+service two-oracle world. `holds` cells are `(kind, entity,
/// selector)` triples — the service selectors `@enabled` / `@active` are distinct, so the selector
/// is explicit (unlike `render_for`, which hardwires `@installed`).
fn render_service_for(src: &str, holds: &[(&str, &str, &str)]) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = service_index(&mut i);
    let held: Vec<FactKey> = holds
        .iter()
        .map(|(k, e, s)| FactKey {
            kind: KindId(i.intern(k)),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern(s)),
            context: dorc_core::Context::HostDefault,
        })
        .collect();
    render_core(
        src,
        &[CORPUS_PREDICT_SRC, SERVICE_PREDICT_SRC],
        &idx,
        held,
        VouchMode::Reached,
        &mut i,
    )
}

/// A second provider (`yum`) for the SAME `package` kind (the cross-oracle Seam): its own check
/// (`rpm -q`), same install verb → `package@installed`. Simplified-kind like `CORPUS_PREDICT_SRC`.
const YUM_PREDICT_SRC: &str = r#"
yum__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then probe-pkg "$pkg"; fi
}
"#;

const YUM_VERDICT_SRC: &str = r"
yum__is_converged() { return 0; }
";

/// `package_index` + the `yum` provider on the SAME `package` kind (two providers, one kind).
fn seam_index(i: &mut Interner) -> KindIndex {
    let mut idx = package_index(i);
    let package = KindId(i.intern("package"));
    let installed = SelectorId(i.intern("installed"));
    let yum = ProviderId(i.intern("yum"));
    let install = i.intern("install");
    // File 1, for the reason `service_index` states: `YUM_PREDICT_SRC` is the second source.
    idx.add_effect(1, yum, install, package, installed, ValueClaim::Establish);
    idx
}

/// Render harness for the two-providers-one-kind seam (apt + yum ⇒ `package@installed`). `holds` are
/// `(entity)` package cells (selector fixed `@installed`, both providers share it).
fn render_seam_for(src: &str, holds: &[&str]) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = seam_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let package = KindId(i.intern("package"));
    let held: Vec<FactKey> = holds
        .iter()
        .map(|e| FactKey {
            kind: package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: installed,
            context: dorc_core::Context::HostDefault,
        })
        .collect();
    render_core(
        src,
        &[CORPUS_PREDICT_SRC, YUM_PREDICT_SRC],
        &idx,
        held,
        VouchMode::Reached,
        &mut i,
    )
}

/// A nullary-verb Singleton oracle: `apt-get update ⇒ pkgindex@fresh` on the kind's implicit single
/// cell (no operand). The `idx : pkgindex` bind is the value-less Singleton form.
const PKGINDEX_PREDICT_SRC: &str = r"
apt_get__predict() {
   verb=$1; shift
   case $verb in
      update) test -n fresh : sm.dorc.PkgIndex@fresh ;;
   esac
}
";

/// Render harness for the Singleton `apt-get update` world (`pkgindex@fresh`, the kind's one cell).
fn render_singleton_for(src: &str, holds_fresh: bool) -> (String, Plan) {
    let mut i = Interner::default();
    let pkgindex = KindId(i.intern("sm.dorc.PkgIndex"));
    let fresh = SelectorId(i.intern("fresh"));
    let apt = ProviderId(i.intern("apt_get"));
    let update = i.intern("update");
    let mut idx = KindIndex::default();
    idx.add_effect(0, apt, update, pkgindex, fresh, ValueClaim::Establish);
    let held: Vec<FactKey> = if holds_fresh {
        vec![FactKey {
            kind: pkgindex,
            entity: EntityRef::Singleton,
            selector: fresh,
            context: dorc_core::Context::HostDefault,
        }]
    } else {
        Vec::new()
    };
    render_core(
        src,
        &[PKGINDEX_PREDICT_SRC],
        &idx,
        held,
        VouchMode::Reached,
        &mut i,
    )
}

/// The read-only `dpkg -s <pkg>` package-status QUERY oracle (the DESIGN door-1 `dpkg -s || install`
/// idiom): `-s` stripped, operand annotated as `pkgstate` (a DISTINCT kind from `package`). Its probed
/// rc feeds the fold's Status. Effects come from `query_index`.
const DPKG_QUERY_PREDICT_SRC: &str = r#"
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : pkgstate = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1
}
"#;

/// `package_index` + the `dpkg -s` read-only Query on `pkgstate@installed` (Observe). The door-1 guard
/// is a Query on `pkgstate`, a DIFFERENT kind from the `package` an install establishes — no cross-kind
/// identity; the fold turns purely on the guard's own rc.
fn query_index(i: &mut Interner) -> KindIndex {
    let mut idx = package_index(i);
    let pkgstate = KindId(i.intern("pkgstate"));
    let installed = SelectorId(i.intern("installed"));
    let dpkg = ProviderId(i.intern("dpkg"));
    let eps = dorc_oracle::empty_verb(i);
    // File 1, for the reason `service_index` states: `DPKG_QUERY_PREDICT_SRC` is the second source.
    idx.add_effect(1, dpkg, eps, pkgstate, installed, ValueClaim::Observe);
    idx
}

/// Render harness for the door-1 `dpkg -s` Query-guard idiom, mirroring the cli's wrong-concrete
/// FIREWALL (`observable_matrix.rs::plan_query`): the guard's `pkgstate:<guard_entity>@installed` cell
/// is observed with `guard_rc`, but that rc reaches the fold's Status ONLY when the site classified a
/// VALID `QueryResolvable` (else withheld ⇒ status ⊤, e.g. an in-loop/invalidated guard). `package`
/// cells (inner installs) are answered verdict-only by `pkg_holds`. The guard's Effect verdict derives
/// from its rc (0 ⇒ Converged). THE dash -n net fires.
fn render_query_for(
    src: &str,
    guard_entity: &str,
    guard_rc: i32,
    pkg_holds: &[&str],
) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = query_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let pkgstate = KindId(i.intern("pkgstate"));
    let package = KindId(i.intern("package"));
    let guard_fact = FactKey {
        kind: pkgstate,
        entity: EntityRef::Operand(OpaqueToken(i.intern(guard_entity))),
        selector: installed,
        context: dorc_core::Context::HostDefault,
    };
    let pkg_facts: Vec<FactKey> = pkg_holds
        .iter()
        .map(|e| FactKey {
            kind: package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: installed,
            context: dorc_core::Context::HostDefault,
        })
        .collect();

    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let (classes, invalidators, value) = classify_with(
        &cfg,
        &parsed.value,
        &idx,
        &[CORPUS_PREDICT_SRC, DPKG_QUERY_PREDICT_SRC],
        &mut i,
    );

    // Mirror the cli firewall: the guard's rc reaches Status only when the site is a VALID Query.
    let guard_valid = classes.iter().any(|(_, c)| {
        matches!(c, SkipClass::QueryResolvable { fact, valid: true } if *fact == guard_fact)
    });
    let guard_effect = if guard_rc == 0 {
        Verdict::Converged
    } else {
        Verdict::Diverged
    };
    let observe = move |f: FactKey| {
        if f == guard_fact {
            Observable {
                effect: guard_effect,
                status: if guard_valid {
                    Predicted::Value(Rc(guard_rc))
                } else {
                    Predicted::Top
                },
                stdout: Predicted::Top,
                stderr: Predicted::Top,
            }
        } else if pkg_facts.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    };
    let plan = build_plan(
        src,
        &parsed.value,
        &cfg,
        &classes,
        &invalidators,
        &vouch_all(&classes, &value, VouchMode::Reached, &mut i),
        observe,
        &mut dorc_core::ProvArena::new(),
    );
    let rendered = plan.render_apply(src, &parsed.value);
    assert_runnable(&rendered);
    (rendered, plan)
}

/// Render harness for the GUARD tier (`guard23-why-attribution`): a vouched install PAST a poison
/// wall. Vouches `EstablishProbeWritten` too (`incl_written`), so a converged past-wall install mints a
/// `Guard` (the oracle's verdict body re-decides LIVE at apply). `holds` are `(entity)` package cells.
fn render_guard_for(src: &str, holds: &[&str]) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let package = KindId(i.intern("package"));
    let held: Vec<FactKey> = holds
        .iter()
        .map(|e| FactKey {
            kind: package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: installed,
            context: dorc_core::Context::HostDefault,
        })
        .collect();
    render_core(
        src,
        &[CORPUS_PREDICT_SRC],
        &idx,
        held,
        VouchMode::Reached,
        &mut i,
    )
}

/// Batch-4 scoped harness (the last no-mint floors): observe `converged` cells Converged, `canttell`
/// cells Unknown (cant-tell), everything else Diverged; and vouch every establish whose fact-KIND is
/// in `vouch_kinds` (ambient AND written), leaving other kinds UNVOUCHED — modeling an oracle's vouch
/// being scoped to its OWN kind (`vouch-scope-is-the-body-never-the-tool`: apt's verdict never guards
/// systemctl's sites). Uses `service_index` (package + service) so both tools resolve. Cells are
/// `(kind, entity, selector)` triples. THE dash -n net fires.
fn render_scoped(
    src: &str,
    converged: &[(&str, &str, &str)],
    canttell: &[(&str, &str, &str)],
    vouch_kinds: &[&str],
) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = service_index(&mut i);
    let cell = |i: &mut Interner, (k, e, s): &(&str, &str, &str)| FactKey {
        kind: KindId(i.intern(k)),
        entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
        selector: SelectorId(i.intern(s)),
        context: dorc_core::Context::HostDefault,
    };
    let conv: Vec<FactKey> = converged.iter().map(|c| cell(&mut i, c)).collect();
    let cant: Vec<FactKey> = canttell.iter().map(|c| cell(&mut i, c)).collect();
    let vkinds: Vec<KindId> = vouch_kinds.iter().map(|k| KindId(i.intern(k))).collect();

    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let (classes, invalidators, value) = classify_with(
        &cfg,
        &parsed.value,
        &idx,
        &[CORPUS_PREDICT_SRC, SERVICE_PREDICT_SRC],
        &mut i,
    );
    let mut verdict_srcs = Vec::new();
    if vkinds.contains(&KindId(i.intern("package"))) {
        verdict_srcs.push(CORPUS_VERDICT_SRC);
    }
    if vkinds.contains(&KindId(i.intern("service"))) {
        verdict_srcs.push(SERVICE_VERDICT_SRC);
    }
    let vouches = dorc_plan::build_vouches(
        &verdict_srcs,
        &[],
        &dorc_oracle::closure::HelperIndex::default(),
        &classes,
        &value,
        &mut i,
        dorc_analysis::funcenv::LiveDefinitions::unsolved(),
    )
    .0
    .value;
    let observe = move |f: FactKey| {
        if conv.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else if cant.contains(&f) {
            Observable::verdict_only(Verdict::Unknown)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    };
    let plan = build_plan(
        src,
        &parsed.value,
        &cfg,
        &classes,
        &invalidators,
        &vouches,
        observe,
        &mut dorc_core::ProvArena::new(),
    );
    let rendered = plan.render_apply(src, &parsed.value);
    assert_runnable(&rendered);
    (rendered, plan)
}

/// Is the leaf whose verbatim text contains `needle` **replaced** (elided to a stand-in)?
fn is_replaced(plan: &Plan, needle: &str) -> bool {
    plan.steps()
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Replace(_, _)))
}

/// Is the leaf containing `needle` **omitted** (a fold-dead branch — distinct from a `Replace`)?
fn is_omitted(plan: &Plan, needle: &str) -> bool {
    plan.steps()
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Omit { .. }))
}

/// Is the leaf containing `needle` **guarded** (the oracle's verdict re-decides live at apply)?
fn is_guarded(plan: &Plan, needle: &str) -> bool {
    plan.steps()
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Guard(_)))
}

// ===========================================================================
// THE NET — proof it fires (`24I` design-flag: a deliberately-broken render must be caught).
// ===========================================================================

#[test]
fn net_catches_a_deliberately_broken_render() {
    // The historical trap shape: a non-runnable artifact (an `if` with no `then` body) that a
    // text-only `.contains()` assertion would pass green. `dash -n`/`sh -n` must reject it — proving
    // the net actually parses, so the real twins' clean-parse assertions are meaningful.
    let broken = "#!/bin/sh\nif true; then\nfi\n"; // empty then-clause ⇒ a syntax error
    let checked = run_syntax_check(broken).expect("a POSIX shell is required for the ap-2 net");
    assert!(
        !checked.1.status.success(),
        "the ap-2 net must REJECT a non-runnable artifact (empty then-clause); it did not — the net \
         is blind, exactly the ap-2 trap"
    );
}

#[test]
fn net_passes_a_runnable_render() {
    // The negative control: a genuinely runnable artifact passes, so the net discriminates (it is
    // not vacuously failing everything).
    let ok = "#!/bin/sh\nif true; then echo y\nfi\n";
    let checked = run_syntax_check(ok).expect("a POSIX shell is required for the ap-2 net");
    assert!(
        checked.1.status.success(),
        "a runnable artifact must pass the net"
    );
}

// ===========================================================================
// MUST-COVER — `24C:st-1` (the `true || true` door-3 idiom): a `cmd || true` converged elision
// renders the left operand to `true`, yielding `true || true`, which MUST be dash -n clean. This
// exercises the net on the door-3 shape the tier is most likely to (wrongly) render non-runnable.
// ===========================================================================

#[test]
fn must_cover_door3_oror_true_renders_runnable() {
    // né e2e door-3 idiom: `apt-get install -y nginx || true`, converged ⇒ the install is a
    // door-3 `StatusInvariant` replace (stand-in `true`) ⇒ the line renders `true || true`. The net
    // (in `render_for`) proves it parses; the shape + disposition are pinned here.
    let (rendered, plan) = render_for(
        "apt-get install -y nginx || true\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the converged `cmd || true` left mints a Replace (door-3 StatusInvariant): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("true || true"),
        "the door-3 elision renders `true || true` (the must-cover shape):\n{rendered}"
    );
}

// ===========================================================================
// RENDER-SHAPE twins (`24I` batch-3). Each carries the exact render shape + disposition its retired
// e2e case pinned: the surrounding structure survives verbatim, the converged install elides to
// `true` with the `# dorc: elided [...]` receipt, and the artifact is dash -n clean (the net).
// ===========================================================================

#[test]
fn twin_render_case_arm_oneliner() {
    // né render-case-arm-oneliner: an install inside a one-line `case` arm elides in situ, the
    // `case`/arm structure kept verbatim.
    let (rendered, plan) = render_for(
        "case nginx in\n  nginx) apt-get install -y nginx ;;\n  *) : ;;\nesac\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the arm install elides"
    );
    assert!(
        rendered.contains("case nginx in"),
        "the case opener survives:\n{rendered}"
    );
    assert!(
        rendered.contains("nginx) true ;;"),
        "the install elides to `true` in the arm, the arm structure kept:\n{rendered}"
    );
    assert!(
        rendered.contains("*) : ;;"),
        "the other arm survives verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("# dorc: elided [apt-get install -y nginx]"),
        "the elision carries its receipt comment:\n{rendered}"
    );
}

#[test]
fn twin_render_multileaf_line_all_elide() {
    // né render-multileaf-line-all-elide: two installs on ONE line (`a; b`), both converged ⇒ both
    // elide, the whole line rendering `true; true` with one receipt for the line.
    let (rendered, plan) = render_for(
        "apt-get install -y nginx; apt-get install -y curl\n",
        &[("package", "nginx"), ("package", "curl")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the first install elides"
    );
    assert!(
        is_replaced(&plan, "install -y curl"),
        "the second install elides"
    );
    assert!(
        rendered.contains("true; true"),
        "both leaves on the shared line render `true; true`:\n{rendered}"
    );
}

#[test]
fn twin_fi_shared_line() {
    // né fi-shared-line: the post-`if` install shares its physical line with the closing `fi`; the
    // in-situ render keeps the `fi` and substitutes only the install (`fi; true`).
    let (rendered, plan) = render_for(
        "if true; then echo y\nfi; apt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the post-if install elides"
    );
    assert!(
        rendered.contains("fi; true"),
        "the `fi` is kept, only the install substituted (`fi; true`):\n{rendered}"
    );
    assert!(
        rendered.contains("if true; then echo y"),
        "the if-body survives verbatim:\n{rendered}"
    );
}

#[test]
fn twin_pre_loop_shared_for_line() {
    // né pre-loop-shared-for-line: the pre-loop install shares its line with the `for` opener; the
    // render keeps `for x in a` and substitutes only the install (`true; for x in a`).
    let (rendered, plan) = render_for(
        "apt-get install -y nginx; for x in a\ndo echo \"$x\"; done\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the pre-loop install elides"
    );
    assert!(
        rendered.contains("true; for x in a"),
        "the `for` opener is kept, the install substituted (`true; for x in a`):\n{rendered}"
    );
    assert!(
        rendered.contains("do echo \"$x\"; done"),
        "the loop body survives verbatim:\n{rendered}"
    );
}

#[test]
fn twin_post_loop_shared_done_line() {
    // né post-loop-shared-done-line: the post-loop install shares its line with the loop's `done`;
    // the render keeps `done` and substitutes only the install (`done; true`).
    let (rendered, plan) = render_for(
        "for x in a b; do echo \"$x\"\ndone; apt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the post-loop install elides"
    );
    assert!(
        rendered.contains("done; true"),
        "the `done` is kept, the install substituted (`done; true`):\n{rendered}"
    );
    assert!(
        rendered.contains("for x in a b; do echo \"$x\""),
        "the loop survives verbatim:\n{rendered}"
    );
}

// ===========================================================================
// LOOP-MEMBER twins (`24I` batch-3, task-L2): the all-or-nothing in-loop render floor. A converged
// loop whose EVERY member holds elides the body once (`do true; done`); any diverged member, or an
// unanalyzable/empty host, keeps the whole loop verbatim (no in-loop license — `inv-kfail`).
// ===========================================================================

#[test]
fn twin_loop_members_all_converged_elides() {
    // né loop-members-all-converged-elides: `for pkg in nginx curl; do install "$pkg"; done`, BOTH
    // members converged ⇒ the body elides to `true`, the loop scaffold kept.
    let (rendered, plan) = render_for(
        "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\n",
        &[("package", "nginx"), ("package", "curl")],
    );
    assert!(
        is_replaced(&plan, "install -y \"$pkg\""),
        "both members converged ⇒ the in-loop install elides: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    // Batch-4 GuardLicense-absence tightening (subsumes né guard23-inloop-unchanged, the donor of
    // this very shape): an in-loop member elides via the Members-loop license — the guard tier never
    // touches it (no `Guard` disposition), so the guard leaves in-loop sites structurally unchanged.
    assert!(
        !is_guarded(&plan, "install -y \"$pkg\""),
        "the in-loop member is NEVER guarded (Members-loop elision, not a guard)"
    );
    assert!(
        rendered.contains("for pkg in nginx curl; do true; done"),
        "the loop body renders `do true; done`, the scaffold kept:\n{rendered}"
    );
}

#[test]
fn twin_loop_members_partial_runs() {
    // né loop-members-partial-runs: ONE member (curl) diverged ⇒ the all-or-nothing floor keeps the
    // WHOLE loop verbatim (a single per-iteration `true` would skip curl's install — under-execute).
    let (rendered, plan) = render_for(
        "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y \"$pkg\""),
        "a partially-converged loop keeps the install (all-or-nothing floor): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("for pkg in nginx curl; do apt-get install -y \"$pkg\"; done"),
        "the whole loop renders verbatim:\n{rendered}"
    );
}

#[test]
fn twin_loop_analyzed_body_runs() {
    // né loop-analyzed-body-runs: the in-loop render floor STILL holds for a fixed-entity install
    // inside a loop with an UNANALYZED host (empty holds) — no in-loop license, the loop runs
    // verbatim.
    let (rendered, plan) = render_for("for x in a b; do apt-get install -y nginx; done\n", &[]);
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "the in-loop install runs (no license)"
    );
    assert!(
        rendered.contains("for x in a b; do apt-get install -y nginx; done"),
        "the loop renders verbatim:\n{rendered}"
    );
}

#[test]
fn twin_loop_post_elision_revives() {
    // né loop-post-elision-revives: a pure-body loop (echo) does not poison the POST-loop converged
    // install — it elides (whole-line comment, its own physical line).
    let (rendered, plan) = render_for(
        "for f in a b; do echo \"$f\"; done\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the post-loop install elides"
    );
    assert!(
        rendered.contains("for f in a b; do echo \"$f\"; done"),
        "the loop survives verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("# apt-get install -y nginx"),
        "the post-loop install elides on its own line (whole-line comment):\n{rendered}"
    );
}

// ===========================================================================
// SAME-CELL-KILL twin (`24I` batch-3 collapse: kill-then-install ≡ exec-same-cell-kill, byte-
// identical books). A `purge` (EstablishInverted) KILLS `package:nginx@installed`; the following
// `install` of the same cell must RUN even when the host reports it converged — the kill walls the
// elision (frame problem). This carries the STRONGER converged-host pin (the retired cases used an
// empty host, where the install runs trivially-diverged); both render both lines verbatim.
// ===========================================================================

#[test]
fn twin_same_cell_kill_forces_install() {
    let (rendered, plan) = render_for(
        "apt-get purge nginx\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "purge nginx"),
        "the purge is a mutator ⇒ runs"
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "the same-cell kill (purge) walls the converged install's elision ⇒ it runs: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("apt-get purge nginx"),
        "the purge renders verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("apt-get install -y nginx"),
        "the install renders verbatim:\n{rendered}"
    );
}

// ===========================================================================
// EXEC / RENDER package-oracle twins (`24I` batch-3 remainder, stage-6b). Each carries the same
// render end-state its retired e2e case pinned, on the existing `render_for` (package-oracle) harness:
// a converged install elides (own line ⇒ whole-line comment; shared line ⇒ `true`), a pure/opaque
// neighbour is or is-not a poison wall, an install used as a status-consumer runs. dash -n net rides.
// ===========================================================================

#[test]
fn twin_exec_devnull_exempt() {
    // né exec-devnull-exempt: `>/dev/null` is the discard sink the gate exempts (no consumer) ⇒ the
    // converged install still elides (whole-line comment, its own physical line).
    let (rendered, plan) = render_for(
        "apt-get install -y nginx >/dev/null\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the devnull-redirected install elides"
    );
    assert!(
        rendered.contains("# apt-get install -y nginx >/dev/null"),
        "the whole line is commented out (the redirect kept in the receipt):\n{rendered}"
    );
    assert!(
        rendered.contains("# dorc: elided"),
        "the elision carries its receipt:\n{rendered}"
    );
}

#[test]
fn twin_exec_pure_builtin_cd() {
    // né exec-pure-builtin: `cd` is a target-state-pure builtin (fs-4) — it does NOT poison the
    // downstream converged install, which elides on its own line.
    let (rendered, plan) = render_for(
        "cd /tmp\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the post-`cd` install elides (cd is pure)"
    );
    assert!(
        rendered.contains("cd /tmp"),
        "the `cd` survives verbatim (not poisoned away):\n{rendered}"
    );
    assert!(
        rendered.contains("# apt-get install -y nginx"),
        "the install elides on its own line (whole-line comment):\n{rendered}"
    );
}

#[test]
fn twin_exec_literal_unset_pure() {
    // né exec-literal-unset-pure: `unset` of a literal name is target-state-pure ⇒ no poison ⇒ the
    // converged install elides.
    let (rendered, plan) = render_for(
        "unset TMPDIR\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the post-`unset` install elides"
    );
    assert!(
        rendered.contains("unset TMPDIR"),
        "the `unset` survives verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("# apt-get install -y nginx"),
        "the install elides on its own line:\n{rendered}"
    );
}

#[test]
fn twin_exec_multileaf_line_mixed() {
    // né exec-multileaf-line-mixed: two leaves on ONE line — a converged install and an un-oracled
    // `systemctl reload`. The install elides to `true`; the un-oracled neighbour RUNS verbatim, so
    // the line renders `true; systemctl reload nginx` with a receipt naming the elided command.
    let (rendered, plan) = render_for(
        "apt-get install -y nginx; systemctl reload nginx\n",
        &[("package", "nginx")],
    );
    assert!(is_replaced(&plan, "install -y nginx"), "the install elides");
    assert!(
        !is_replaced(&plan, "systemctl reload nginx"),
        "the un-oracled neighbour runs (no model ⇒ no elision)"
    );
    assert!(
        rendered.contains("true; systemctl reload nginx"),
        "the elided install becomes `true`, the neighbour kept:\n{rendered}"
    );
    assert!(
        rendered.contains("# dorc: elided [apt-get install -y nginx]"),
        "the receipt names the elided leaf:\n{rendered}"
    );
}

#[test]
fn twin_render_if_guard_toprc_runs() {
    // né render21-if-guard-toprc-runs: the install used AS the `if` condition is an establish
    // (mutator) whose rc is ⊤ (fork-mutator-rc) ⇒ StatusRelaxable blocks ⇒ it RUNS, the whole `if`
    // rendering verbatim. Converged host — the ONLY reason it runs is the unusable ⊤ rc.
    let (rendered, plan) = render_for(
        "if apt-get install -y nginx\nthen\n   echo started\nfi\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "the install-as-if-guard runs (⊤ rc, StatusRelaxable): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("if apt-get install -y nginx"),
        "the whole `if` renders verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guarded_if_true_then_body_elides() {
    // né guarded: a converged install inside an `if true; then … fi` body elides to `true` in situ,
    // the `if`/`fi` scaffold and the trailing `echo` kept verbatim.
    let (rendered, plan) = render_for(
        "if true; then\n   apt-get install -y nginx\nfi\necho done\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the then-body install elides"
    );
    assert!(
        rendered.contains("if true; then"),
        "the `if` opener survives:\n{rendered}"
    );
    assert!(
        rendered.contains("true   # dorc: elided [apt-get install -y nginx]"),
        "the install elides to `true` in the body:\n{rendered}"
    );
    assert!(
        rendered.contains("echo done"),
        "the trailing echo survives verbatim:\n{rendered}"
    );
}

#[test]
fn twin_door3_or_handler_blocks() {
    // né door3-or-handler-blocks: `cmd || { handler; }` is NOT the door-3 `|| true` idiom — the rhs
    // is a handler group, so the left keeps its blocking StatusRelaxable mark. Converged install +
    // ⊤ rc ⇒ RUNS; door-3 must not widen to non-`true` handlers.
    let (rendered, plan) = render_for(
        "set -e\napt-get install -y nginx || { printf 'recovering\\n'; }\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a `|| handler-group` left is not door-3 ⇒ the converged mutator runs: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("apt-get install -y nginx || { printf 'recovering\\n'; }"),
        "the whole `|| {{ handler; }}` line renders verbatim:\n{rendered}"
    );
}

#[test]
fn twin_exec_opaque_neighbour_poisons() {
    // né exec-opaque-neighbour (~SUSPECT → MIGRATE, twin STRENGTHENED to a converged-host pin): an
    // un-oracled `ufw` is Opaque ⇒ a poison wall (opaque-poison-is-the-product). The DOWNSTREAM
    // install runs EVEN THOUGH the host reports it converged — the wall, not divergence, is why. The
    // converged host makes the poison the sole cause (the retired e2e never probed the install).
    let (rendered, plan) = render_for(
        "ufw allow 80/tcp\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "the un-oracled `ufw` walls the converged install ⇒ it runs (poison, not divergence): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("ufw allow 80/tcp"),
        "the opaque neighbour runs verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("apt-get install -y nginx"),
        "the walled install runs verbatim (not commented/elided):\n{rendered}"
    );
}

// ===========================================================================
// INLINE-SPLICE twins (`24I` batch-3, arch-2). A same-file eligible call SPLICES the callee body at
// the call site (seam-interproc); the CALL leaf is the render unit (`inv-leaf-seam`). The
// all-or-nothing CALL license: the call elides iff its spliced body establish holds; any diverged
// member, an in-loop back-edge self-establish, or a running-wall above it keeps the call verbatim.
// ===========================================================================

#[test]
fn twin_exec_detached_fn_splice() {
    // né exec-detached-fn: a same-file `prov() { apt-get install -y nginx; }` splices at the bare
    // `prov` call. Diverged pole (the retired case's pin): body absent ⇒ the CALL runs verbatim.
    // Converged pole (added, so the splice is NON-vacuous): body converged ⇒ the CALL elides — proof
    // the body establish actually spliced through to the call's license.
    let (diverged, dplan) = render_for("prov() { apt-get install -y nginx; }\nprov\n", &[]);
    assert!(
        !is_replaced(&dplan, "prov"),
        "diverged body ⇒ the call runs"
    );
    assert!(
        diverged.contains("prov() { apt-get install -y nginx; }") && diverged.contains("\nprov"),
        "the funcdef and the running call render verbatim:\n{diverged}"
    );
    let (converged, cplan) = render_for(
        "prov() { apt-get install -y nginx; }\nprov\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&cplan, "prov"),
        "converged spliced body ⇒ the CALL elides (all-or-nothing license): {:?}",
        cplan
            .steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        converged.contains("# prov"),
        "the elided call renders as a whole-line comment:\n{converged}"
    );
}

#[test]
fn twin_inline21_wrapper_converged_elides() {
    // né inline21-wrapper-converged-elides: a `$1`-parameterized wrapper `apt_install() { apt-get
    // install -y "$1" >/dev/null 2>&1; }` called twice; BOTH entities (nginx, curl) converged ⇒ each
    // call's spliced body establish holds ⇒ each CALL elides (per-call, independent).
    let (rendered, plan) = render_for(
        "apt_install() { apt-get install -y \"$1\" >/dev/null 2>&1; }\napt_install nginx\napt_install curl\n",
        &[("package", "nginx"), ("package", "curl")],
    );
    assert!(
        is_replaced(&plan, "apt_install nginx"),
        "the nginx call elides"
    );
    assert!(
        is_replaced(&plan, "apt_install curl"),
        "the curl call elides"
    );
    assert!(
        rendered.contains("apt_install() { apt-get install -y \"$1\" >/dev/null 2>&1; }"),
        "the wrapper funcdef survives verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("# apt_install nginx") && rendered.contains("# apt_install curl"),
        "both calls elide as whole-line comments:\n{rendered}"
    );
}

#[test]
fn twin_inline21_wrapper_diverged_runs() {
    // né inline21-wrapper-diverged-runs: the diverged pole — nginx converged (call 0 elides), curl
    // DIVERGED (call 1 runs whole). Calls are INDEPENDENT (the all-or-nothing license is per call).
    let (rendered, plan) = render_for(
        "apt_install() { apt-get install -y \"$1\" >/dev/null 2>&1; }\napt_install nginx\napt_install curl\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "apt_install nginx"),
        "the converged nginx call elides"
    );
    assert!(
        !is_replaced(&plan, "apt_install curl"),
        "the diverged curl call runs (independent per-call license): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("# apt_install nginx"),
        "the nginx call elides:\n{rendered}"
    );
    assert!(
        rendered.contains("\napt_install curl"),
        "the curl call runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_inline21_errexit_call_composes() {
    // né inline21-errexit-call-composes: `set -e` + both calls converged. The bare call (`apt_install
    // nginx`) has its ⊤ body-status errexit-consumed ⇒ RUNS; the `|| true` call would elide on status
    // (door-3) but is WALLED by the running nginx call above it (silence=wall) ⇒ it RUNS too. Both run.
    let (rendered, plan) = render_for(
        "set -e\napt_install() { apt-get install -y \"$1\" >/dev/null 2>&1; }\napt_install nginx\napt_install curl || true\n",
        &[("package", "nginx"), ("package", "curl")],
    );
    assert!(
        !is_replaced(&plan, "apt_install nginx"),
        "the bare call runs (errexit-consumed ⊤ body status): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        !is_replaced(&plan, "apt_install curl"),
        "the `|| true` call is walled by the running nginx call above (silence=wall) ⇒ runs"
    );
    assert!(
        rendered.contains("apt_install nginx") && rendered.contains("apt_install curl || true"),
        "both calls render verbatim:\n{rendered}"
    );
}

#[test]
fn twin_inline21_in_loop_call_floored() {
    // né inline21-in-loop-call-floored: an inlined call INSIDE a loop is floored (task-L1 in-loop
    // floor + the 20M §5 self-establish-via-back-edge ⇒ the body reads EstablishProbeWritten). Converged
    // host — the call STILL runs verbatim (the floor, not divergence, is why; a stronger pin than the
    // retired empty-host case).
    let (rendered, plan) = render_for(
        "w() { apt-get install -y \"$1\" >/dev/null 2>&1; }\nfor pkg in nginx; do w \"$pkg\"; done\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "w \"$pkg\""),
        "the in-loop call is floored ⇒ runs even converged: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("for pkg in nginx; do w \"$pkg\"; done"),
        "the whole loop renders verbatim:\n{rendered}"
    );
}

// ===========================================================================
// SERVICE / TWO-ORACLE twins (`24I` batch-3; the service oracle added to the harness — `render_
// service_for`). Two providers/kinds co-resident; the systemd `@enabled` / `@active` selectors are
// DISTINCT cells of one Service entity (enabling ≠ activating), which is what the last two pin.
// ===========================================================================

#[test]
fn twin_two_oracles() {
    // né two-oracles: a package install (converged) + a service enable (converged) both elide, the
    // trailing un-oracled `echo` runs. Proves two oracles (package + service) co-resolve in one book.
    let (rendered, plan) = render_service_for(
        "apt-get install -y nginx\nsystemctl enable nginx\necho provisioned\n",
        &[
            ("package", "nginx", "installed"),
            ("service", "nginx", "enabled"),
        ],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the converged install elides"
    );
    assert!(
        is_replaced(&plan, "systemctl enable nginx"),
        "the converged service enable elides: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("# apt-get install -y nginx")
            && rendered.contains("# systemctl enable nginx"),
        "both converged sites elide as whole-line comments:\n{rendered}"
    );
    assert!(
        rendered.contains("echo provisioned"),
        "the un-oracled echo runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_exec_distinct_selectors() {
    // né exec-distinct-selectors (~SUSPECT → MIGRATE): `enable`→@enabled, `start`→@active are DISTINCT
    // selectors of one Service. BOTH cells converged ⇒ both sites elide. (The distinctness is proved
    // by the boundary sibling below — here both hold, so both elide.)
    let (rendered, plan) = render_service_for(
        "systemctl enable nginx\nsystemctl start nginx\n",
        &[
            ("service", "nginx", "enabled"),
            ("service", "nginx", "active"),
        ],
    );
    assert!(
        is_replaced(&plan, "systemctl enable nginx"),
        "enable (@enabled converged) elides"
    );
    assert!(
        is_replaced(&plan, "systemctl start nginx"),
        "start (@active converged) elides"
    );
    assert!(
        rendered.contains("# systemctl enable nginx")
            && rendered.contains("# systemctl start nginx"),
        "both sites elide:\n{rendered}"
    );
}

#[test]
fn twin_exec_enabled_not_active_host() {
    // né exec-enabled-not-active-host (~SUSPECT → MIGRATE, the DISTINCTNESS proof): the host has
    // @enabled but NOT @active (enabled≠active boundary). `enable` elides (its cell holds); `start`
    // RUNS (its @active cell is diverged). Were the selectors NOT distinct, `start` would wrongly
    // elide off `enable`'s convergence — this is the twin that would catch that.
    let (rendered, plan) = render_service_for(
        "systemctl enable nginx\nsystemctl start nginx\n",
        &[("service", "nginx", "enabled")],
    );
    assert!(
        is_replaced(&plan, "systemctl enable nginx"),
        "enable elides (@enabled holds)"
    );
    assert!(
        !is_replaced(&plan, "systemctl start nginx"),
        "start RUNS (@active diverged — distinct from @enabled): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("# systemctl enable nginx"),
        "enable elides:\n{rendered}"
    );
    assert!(
        rendered.contains("\nsystemctl start nginx"),
        "start runs verbatim:\n{rendered}"
    );
}

// ===========================================================================
// SEAM + SINGLETON twins (`24I` batch-3). Two providers on one kind (apt+yum ⇒ package), and the
// nullary-verb Singleton establish (apt-get update ⇒ pkgindex@fresh, the kind's one operand-less cell).
// ===========================================================================

#[test]
fn twin_seam_two_providers_one_kind() {
    // né seam-two-providers-one-kind: `apt-get install nginx` and `yum install httpd` are DIFFERENT
    // providers of the SAME `package` kind. Both cells converged ⇒ both elide (cross-oracle seam).
    let (rendered, plan) = render_seam_for(
        "apt-get install -y nginx\nyum install -y httpd\n",
        &["nginx", "httpd"],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the apt install elides"
    );
    assert!(
        is_replaced(&plan, "yum install -y httpd"),
        "the yum install (same kind, other provider) elides: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("# apt-get install -y nginx")
            && rendered.contains("# yum install -y httpd"),
        "both provider sites elide:\n{rendered}"
    );
}

#[test]
fn twin_exec_singleton_update() {
    // né exec-singleton-update (~SUSPECT → MIGRATE): `apt-get update` is a nullary-verb establish on
    // the Singleton `pkgindex@fresh` cell (no operand). Converged (index fresh) ⇒ it elides; diverged
    // ⇒ it runs (the pole that proves the elision is host-gated, not unconditional).
    let (rendered, plan) = render_singleton_for("apt-get update\n", true);
    assert!(
        is_replaced(&plan, "apt-get update"),
        "the converged Singleton index-refresh elides: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("# apt-get update"),
        "it elides as a whole-line comment:\n{rendered}"
    );
    let (drendered, dplan) = render_singleton_for("apt-get update\n", false);
    assert!(
        !is_replaced(&dplan, "apt-get update"),
        "the diverged (stale) index-refresh runs"
    );
    assert!(
        drendered.contains("\napt-get update") || drendered.trim_end().ends_with("apt-get update"),
        "the stale refresh runs verbatim:\n{drendered}"
    );
}

// ===========================================================================
// DOOR-1 QUERY-GUARD twins (`24I` batch-3; the `render_query_for` dpkg-pkgstate harness). The door-1
// idiom `dpkg -s X || { block }`: the guard's KNOWN probed rc folds the `||`. Converged (rc 0) ⇒ guard
// substitutes `true`, the block is DEAD (each member Omit ⇒ `:`); diverged (rc 1) ⇒ guard substitutes
// `false`, the block runs live. In-loop / invalidated guards withhold the rc ⇒ no fold.
// ===========================================================================

#[test]
fn twin_door1_cascade_block_elides() {
    // né door1-cascade-block-elides: guard converged (rc 0) ⇒ `true || { :; :; }` — the whole `||`
    // handler block is fold-dead, each member omitted to `:`.
    let (rendered, plan) = render_query_for(
        "set -e\ndpkg -s nginx >/dev/null 2>&1 || { sed -i 's/x/y/' /etc/ssh/sshd_config; systemctl restart sshd; }\n",
        "nginx",
        0,
        &[],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the converged guard substitutes `true`"
    );
    assert!(
        is_omitted(&plan, "sed -i"),
        "the dead block's sed is omitted"
    );
    assert!(
        is_omitted(&plan, "systemctl restart sshd"),
        "the dead block's systemctl is omitted"
    );
    assert!(
        rendered.contains("true || { :; :; }"),
        "the guard folds to `true`, the dead block members render `:`:\n{rendered}"
    );
}

#[test]
fn twin_door1_cascade_diverged_runs() {
    // né door1-cascade-diverged-runs: guard diverged (rc 1) ⇒ `false || { … }` — the guard substitutes
    // `false`, the `||` handler block is REACHABLE ⇒ its members run verbatim.
    let (rendered, plan) = render_query_for(
        "set -e\ndpkg -s nginx >/dev/null 2>&1 || { sed -i 's/x/y/' /etc/ssh/sshd_config; systemctl restart sshd; }\n",
        "nginx",
        1,
        &[],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the diverged guard substitutes `false`"
    );
    assert!(
        !is_omitted(&plan, "sed -i"),
        "the live block's sed runs (not omitted)"
    );
    assert!(
        !is_omitted(&plan, "systemctl restart sshd"),
        "the live block's systemctl runs"
    );
    assert!(
        rendered.contains("false || {")
            && rendered.contains("sed -i")
            && rendered.contains("systemctl restart sshd"),
        "the guard folds to `false`, the block runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_door1_cascade_multistatement() {
    // né door1-cascade-multistatement: guard converged ⇒ the whole MULTI-line handler block (incl. a
    // nested `if`) is fold-dead; each leaf omits to `:` and the `if` renders `if :; then :; fi`.
    let (rendered, plan) = render_query_for(
        "set -e\ndpkg -s nginx >/dev/null 2>&1 || {\n   sed -i 's/x/y/' /etc/ssh/sshd_config\n   if [ -f /etc/ssh/sshd_config.bak ]; then cp /etc/ssh/sshd_config /etc/ssh/sshd_config.bak; fi\n   systemctl restart sshd\n}\n",
        "nginx",
        0,
        &[],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the converged guard substitutes `true`"
    );
    assert!(is_omitted(&plan, "sed -i"), "the block's sed is omitted");
    assert!(
        rendered.contains("true || {"),
        "the guard folds to `true`, the multi-line block opens:\n{rendered}"
    );
    assert!(
        rendered.contains("if :; then :; fi"),
        "the nested if renders `if :; then :; fi` (both guard and body dead):\n{rendered}"
    );
}

#[test]
fn twin_door1_door3_dead_block_folds() {
    // né door1-door3-dead-block-folds: door-1 × door-3. Guard converged ⇒ the whole block dead,
    // INCLUDING the inner `apt-get install -y curl || true` ⇒ `: || :`. Renders `true || { : || :; :; }`.
    let (rendered, plan) = render_query_for(
        "set -e\ndpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }\n",
        "nginx",
        0,
        &["curl"],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the converged guard substitutes `true`"
    );
    assert!(
        is_omitted(&plan, "install -y curl"),
        "the inner curl install is fold-dead (omitted)"
    );
    assert!(
        is_omitted(&plan, "systemctl restart sshd"),
        "the systemctl is fold-dead"
    );
    assert!(
        rendered.contains("true || { : || :; :; }"),
        "the whole nested block folds to `: || :; :`:\n{rendered}"
    );
}

#[test]
fn twin_door1_door3_inner_elides() {
    // né door1-door3-inner-elides: guard diverged ⇒ the block runs; the inner `apt-get install -y curl
    // || true` with curl CONVERGED ⇒ door-3 StatusInvariant elides curl to `true` ⇒ `true || true`.
    let (rendered, plan) = render_query_for(
        "set -e\ndpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }\n",
        "nginx",
        1,
        &["curl"],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the diverged guard substitutes `false`"
    );
    assert!(
        is_replaced(&plan, "install -y curl"),
        "the converged inner curl elides via door-3 `|| true`: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("false || { true || true; systemctl restart sshd; }"),
        "the guard folds `false`, curl elides to `true` inside the live block:\n{rendered}"
    );
}

#[test]
fn twin_door1_door3_inner_runs() {
    // né door1-door3-inner-runs: guard diverged ⇒ block runs; the inner curl is DIVERGED ⇒ door-3
    // clears Status but the Effect still gates ⇒ curl RUNS. `false || { apt-get install -y curl || true; … }`.
    let (rendered, plan) = render_query_for(
        "set -e\ndpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }\n",
        "nginx",
        1,
        &[],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the diverged guard substitutes `false`"
    );
    assert!(
        !is_replaced(&plan, "install -y curl"),
        "the diverged inner curl runs (door-3 clears Status, Effect gates): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("false || { apt-get install -y curl || true; systemctl restart sshd; }"),
        "the live block runs the curl install verbatim:\n{rendered}"
    );
}

#[test]
fn twin_render21_if_guard_query_elides() {
    // né render21-if-guard-query-elides: `if ! dpkg -s nginx …; then apt-get install …; fi`. Guard
    // converged (rc 0) ⇒ `! 0` ⇒ if-false ⇒ the then-body install is fold-dead. Renders `if ! true …
    // then : … fi`.
    let (rendered, plan) = render_query_for(
        "set -e\nif ! dpkg -s nginx >/dev/null 2>&1\nthen\n   apt-get install -y nginx\nfi\n",
        "nginx",
        0,
        &["nginx"],
    );
    assert!(
        is_replaced(&plan, "dpkg -s nginx"),
        "the converged guard substitutes `true`"
    );
    assert!(
        is_omitted(&plan, "install -y nginx"),
        "the then-body install is fold-dead (Omit): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("if ! true"),
        "the guard folds to `if ! true`:\n{rendered}"
    );
    assert!(
        rendered.contains("\n   :   # dorc: elided"),
        "the dead then-body renders `:`:\n{rendered}"
    );
}

#[test]
fn twin_render21_while_guard_floored() {
    // né render21-while-guard-floored: an in-loop `while dpkg -s nginx …` condition is StatusIterated
    // (a per-iteration sequence no single rc reproduces) AND excluded from probing ⇒ the rc is withheld
    // ⇒ NO fold ⇒ the whole loop runs verbatim.
    let (rendered, plan) = render_query_for(
        "while dpkg -s nginx >/dev/null 2>&1\ndo\n   echo checking\ndone\n",
        "nginx",
        0,
        &[],
    );
    assert!(
        !is_replaced(&plan, "dpkg -s nginx"),
        "the in-loop guard is floored (not folded/substituted): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("while dpkg -s nginx >/dev/null 2>&1")
            && rendered.contains("echo checking")
            && rendered.contains("done"),
        "the whole loop renders verbatim:\n{rendered}"
    );
}

// ===========================================================================
// GUARD-TIER twin (`24I` batch-3; `render_guard_for`, the written-vouch harness). A vouched install
// PAST a poison wall cannot ELIDE (its resting probe is poisoned), so it GUARDS: the oracle's own
// verdict body re-decides live at apply (`( check ) || <original>`), carrying the why-attribution.
// ===========================================================================

#[test]
fn twin_guard23_why_attribution() {
    // né guard23-why-attribution (the disclosure floor: every guard decision is ATTRIBUTED): an
    // un-oracled `hork` walls the vouched `apt-get install`. Converged host + the vouch ⇒ a GUARD (not
    // elide): the oracle's verdict body re-decides LIVE at apply, carrying the why-attribution, with
    // the vouch body shipped as the preamble. This twin pins the full POISON-WALL→GUARD pipeline +
    // attribution + preamble; the exact check-invocation ARGV (`( f install -y nginx )`) is a harness
    // simplification here (the generic `vouch_all` invocation carries no site argv — the real cli's
    // `build_vouches` threads it; that render-line shape is unit-pinned by
    // converged_guard_emitter_shape_obeys_the_two_never_clauses).
    let (rendered, plan) = render_guard_for("hork wombat\napt-get install -y nginx\n", &["nginx"]);
    assert!(
        is_guarded(&plan, "install -y nginx"),
        "the walled-but-vouched converged install GUARDS (not elide/run): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        rendered.contains("hork wombat"),
        "the opaque wall runs verbatim:\n{rendered}"
    );
    assert!(
        rendered.contains("( apt_get__is_converged")
            && rendered.contains(") || apt-get install -y nginx"),
        "the guard shape `( <check> ) || <original verbatim>` — original bytes survive:\n{rendered}"
    );
    assert!(
        rendered.contains("# dorc: guard [package converged-vouch; probe: holds]"),
        "the guard carries its why-attribution (kind, vouch-source, probe verdict):\n{rendered}"
    );
    assert!(
        rendered.contains("apt_get__is_converged() {"),
        "the vouch body ships as the guard preamble (rul-ternary-verdict: authored bytes verbatim):\n{rendered}"
    );
}

// ===========================================================================
// BATCH-4 — guard23 NO-MINT floors (`24I` order-item-4). Each pins that NO GuardLicense mints (and
// no Replace either) for a shape where a converged install has the vouch AVAILABLE but a structural
// reason forecloses the license — the site RUNS (`kFAIL-perform`). The `!is_guarded` assert is the
// TIGHTER structural floor these convert to (from the retired run-set proxies). The mint-policy
// itself (Unknown/Diverged/no-vouch ⇒ None) is unit-pinned in plan/src (guard_mints_only_on_a_
// converged_probe_verdict, no_license_for_ambient_without_vouch); these carry the full-pipeline shape.
// ===========================================================================

#[test]
fn twin_guard23_background_not_guarded() {
    // né guard23-background-not-guarded: `apt-get install -y nginx &`. The `&` makes the site a
    // ⊤-successor (P-background), which forecloses BOTH the elision (spec_topcontext_background) and
    // the guard (`!has_top_successor`) — it runs. Converged host + vouch available.
    let (rendered, plan) = render_for(
        "apt-get install -y nginx &\nwait\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_guarded(&plan, "install -y nginx"),
        "a background site never guards (⊤-successor)"
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "nor elides (⊤-successor) ⇒ runs"
    );
    assert!(
        rendered.contains("apt-get install -y nginx &"),
        "the background install runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_consumed_stdout_runs() {
    // né guard23-consumed-stdout-runs: `apt-get install -y nginx | tee …`. The install's stdout is
    // consumed by the pipe ⇒ its empty stub-default is unvouched ⇒ no license (neither elide nor
    // guard) ⇒ runs. Converged host + vouch available.
    let (rendered, plan) = render_for(
        "apt-get install -y nginx | tee /var/log/install.log\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_guarded(&plan, "install -y nginx"),
        "a consumed-stdout site never guards"
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "nor elides (consumed stdout) ⇒ runs"
    );
    assert!(
        rendered.contains("apt-get install -y nginx | tee /var/log/install.log"),
        "the pipeline runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_cmdsub_position_runs() {
    // né guard23-cmdsub-position-runs: `out=$(apt-get install -y nginx)`. The install is
    // expansion-internal (inside `$()`) ⇒ not a plan leaf ⇒ never a license candidate ⇒ runs.
    let (rendered, plan) = render_for(
        "out=$(apt-get install -y nginx)\necho \"$out\"\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_guarded(&plan, "install -y nginx"),
        "a cmdsub-position install never guards"
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "nor elides (expansion-internal) ⇒ runs"
    );
    assert!(
        rendered.contains("out=$(apt-get install -y nginx)"),
        "the command-substitution runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_top_argv_runs() {
    // né guard23-top-argv-runs: `PKG=$(cat /etc/pkg); apt-get install -y "$PKG"`. `$PKG` is ⊤ (a
    // cmdsub value) ⇒ the argv is unresolvable ⇒ no cell ⇒ no license ⇒ runs (holds is irrelevant —
    // there is no resolvable entity to answer).
    let (rendered, plan) = render_for(
        "PKG=$(cat /etc/pkg)\napt-get install -y \"$PKG\"\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_guarded(&plan, "install -y \"$PKG\""),
        "a ⊤-argv install never guards"
    );
    assert!(
        !is_replaced(&plan, "install -y \"$PKG\""),
        "nor elides (unresolvable argv) ⇒ runs"
    );
    assert!(
        rendered.contains("apt-get install -y \"$PKG\""),
        "the ⊤-argv install runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_inverted_vouch_never_backwards() {
    // né guard23-inverted-vouch-never-backwards: `apt-get purge oldpkg`. A purge is EstablishInverted
    // ⇒ classifies MustRun ⇒ never elides AND never guards (a guard is an establish-tier verb; an
    // inverted/kill site has no converged-elide/guard path) ⇒ runs.
    let (rendered, plan) = render_for("apt-get purge oldpkg\n", &[]);
    assert!(
        !is_guarded(&plan, "purge oldpkg"),
        "an inverted (purge) site never guards backwards"
    );
    assert!(!is_replaced(&plan, "purge oldpkg"), "nor elides ⇒ runs");
    assert!(
        rendered.contains("apt-get purge oldpkg"),
        "the purge runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_no_vouch_runs() {
    // né guard23-no-vouch-runs: the oracle authored NO verdict body (VouchMode::None). The ambient
    // nginx install can't elide (no vouch) and the past-`hork` curl install can't guard (no vouch) ⇒
    // BOTH run. This is the no-vouch floor (rul24-vouch-is-verdict-authoring: no vouch ⇒ run).
    let (rendered, plan) = render_for_mode(
        "apt-get install -y nginx\nhork wombat\napt-get install -y curl\n",
        &[("package", "nginx"), ("package", "curl")],
        VouchMode::None,
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "no vouch ⇒ the ambient install runs (no elide)"
    );
    assert!(
        !is_guarded(&plan, "install -y curl"),
        "no vouch ⇒ the past-wall install never guards"
    );
    assert!(
        !is_replaced(&plan, "install -y curl"),
        "and doesn't elide ⇒ runs"
    );
    assert!(
        rendered.contains("apt-get install -y nginx")
            && rendered.contains("apt-get install -y curl"),
        "both installs run verbatim (nothing vouched):\n{rendered}"
    );
}

#[test]
fn twin_guard23_redirect_line_runs() {
    // né guard23-redirect-line-runs: `hork wombat; apt-get install -y nginx >>log`. The install is
    // past the poison wall AND vouched, and a non-/dev/null output redirect is a ratified render
    // REFUSE-HOME (`guard_render_refused`: leaf_has_blocking_output_redirect), so the line renders
    // VERBATIM with no preamble ⇒ the install runs at apply.
    //
    // WHICH floor produces that, this test cannot say and never could: measured 2026-08-21 the
    // redirect gens a second `file:…@written` cell and the site mints no Guard at all, so the
    // outcome is mint-ABSENCE and the render refusal is unreached. The seat is pinned by
    // `a_redirect_refused_guard_is_disclosed_on_every_surface`.
    let (rendered, _plan) =
        render_guard_for("hork wombat\napt-get install -y nginx >>log\n", &["nginx"]);
    assert!(
        rendered.contains("\napt-get install -y nginx >>log"),
        "the redirected install renders VERBATIM (the redirect refuses the guard render):\n{rendered}"
    );
    assert!(
        !rendered.contains("( apt_get__is_converged")
            && !rendered.contains("apt_get__is_converged() {"),
        "no guard line and no guard preamble are emitted for the refused redirect:\n{rendered}"
    );
}

#[test]
fn a_redirect_refused_guard_is_disclosed_on_every_surface() {
    // `30Mf` F2: the redirect half of `guard_render_refused` reached `collect_edits` and
    // `guard_refused_asts` but NOT `refused_render_steps`, so the three disclosure surfaces below
    // saw nothing at all.
    //
    // THE PAIRING IS SYNTHETIC, and disclosed: a leaf carrying `>>log` gens a second
    // `file:…@written` cell and mints no Guard at all today, so no book reaches this seat (measured
    // just below). The LICENSE is real — minted by the redirect-free twin against the same oracle
    // and fact, re-homed onto the redirect leaf.
    let src = "hork wombat\napt-get install -y nginx >>log\n";
    let (_rendered, planned) = render_guard_for(src, &["nginx"]);
    let ast = dorc_syntax::parse(src).value;
    assert!(
        !is_guarded(&planned, ">>log"),
        "the mint absence is the disclosed premise; if this reddens, the seat became reachable \
         and the pairing below should become an ordinary book case"
    );

    let (_bare_render, bare) =
        render_guard_for("hork wombat\napt-get install -y nginx\n", &["nginx"]);
    let license = bare
        .steps()
        .iter()
        .find_map(|step| match &step.disposition {
            Disposition::Guard(license) => Some(license.clone()),
            _ => None,
        })
        .expect("the redirect-free twin mints a REAL guard license");
    // Re-DECIDED, not poked: the render-time answers are a function of the dispositions
    // (`30E` §3), so re-homing the license means re-deciding the plan it belongs to.
    let mut steps = planned.steps().to_vec();
    let redirected = steps
        .iter_mut()
        .find(|step| step.sh.contains(">>log"))
        .expect("the redirect leaf is planned");
    redirected.disposition = Disposition::Guard(license);
    let plan = Plan::decided(
        steps,
        Vec::new(),
        SurvivalReport::default(),
        false,
        dorc_plan::NO_ARTIFACT_FORM,
        src,
        &ast,
        dorc_core::influence::InfluenceAccount::authored_before_contact(),
    );

    let diags = plan.render_refusal_diagnostics(&ast, &Interner::default());
    assert_eq!(diags.len(), 1, "the refusal is disclosed once: {diags:?}");
    let narratives = plan.render_refusal_narratives();
    assert!(
        narratives.iter().any(|n| matches!(
            n.kind(),
            dorc_aid::CollapseKind::RenderRefusal {
                cause: dorc_aid::narrative::RenderRefusalTag::OutputRedirect,
                ..
            }
        )),
        "and narrates its OWN cause, never the heredoc one: {narratives:?}"
    );
    assert_eq!(
        plan.refused_render_leaves()
            .iter()
            .map(|(_, verb)| *verb)
            .collect::<Vec<_>>(),
        ["guard"],
        "the decision-plane record carries the guard verb"
    );
    assert_eq!(
        plan.guard_refused_asts().len(),
        1,
        "and the why-lens still suppresses the `guarded` claim — the fourth consumer of the one \
         predicate, and before this the only one that ever saw a redirect refusal"
    );

    // The DECISION-PLANE record carries the real cause too — it hard-coded `Heredoc`, so it stated
    // a falsehood for exactly the class `30Mf` F2 made reachable (`30Nd` meaning-audit).
    let mut spine = dorc_plan::Spine::new();
    dorc_plan::spine::record_render_decisions(
        &mut spine,
        &plan,
        dorc_core::influence::InfluenceAccount::authored_before_contact(),
    );
    assert!(
        spine.render_decisions().iter().any(|record| matches!(
            record.decision(),
            dorc_core::spine::RenderDecision::Refused {
                cause: dorc_core::spine::RefusalCause::BlockingRedirect
            }
        )),
        "the recorded cause is the redirect's own: {:?}",
        spine.render_decisions()
    );
}

/// AN IMPORT REWRITE IS A FIRST-CLASS PLAN EDIT (`30Ng:rul-bundle-at-dorc-lang-boundaries`,
/// human-typed): it reaches the artifact's bytes, the plan surface, and the decision plane, and it
/// reaches all three from ONE decision taken at `Plan::decided`.
///
/// All three are asserted together on purpose. The grant is narrow — where an import points, in a
/// plan Dorc generated, and nothing else — and what keeps it narrow in practice is that no use of it
/// is silent. An edit that changed the bytes and told nobody would be the same shape as the render
/// decisions `30E` §3 audited out of hiding, on the one line whose meaning moved.
#[test]
fn a_rewritten_import_reaches_the_bytes_the_surface_and_the_plane() {
    let src = ". ./pkg.oracle.sh\napt-get install -y nginx\n";
    let ast = dorc_syntax::parse(src).value;
    // The operand WORD, which is the only thing a re-point moves.
    let operand = ast
        .iter()
        .find_map(|(_, node)| match &node.kind {
            dorc_syntax::ast::NodeKind::Simple { words, .. } => words.get(1).copied(),
            _ => None,
        })
        .expect("the load's operand");
    let placed = dorc_plan::PlacedSources::all_ambient();
    let imports = [dorc_plan::ImportEdit::Repoint {
        ast: operand,
        path: "./pkg.dorc-bundle.sh".to_owned(),
        reason: dorc_plan::PlacementReason::KeptInPlaceLadderUnconsulted,
    }];
    let plan = Plan::decided(
        Vec::new(),
        Vec::new(),
        SurvivalReport::default(),
        false,
        dorc_plan::ArtifactEmission::of(&placed, &imports),
        src,
        &ast,
        dorc_core::influence::InfluenceAccount::authored_before_contact(),
    );

    let artifact = plan.render_apply(src, &ast);
    assert!(
        artifact.contains(". './pkg.dorc-bundle.sh'") && !artifact.contains("./pkg.oracle.sh"),
        "the artifact's own import names the bundle, and nothing else on the line moved:\n{artifact}"
    );
    let disclosed = plan.import_diagnostics(&ast);
    assert_eq!(disclosed.len(), 1, "disclosed once, at the authored line");
    assert!(
        matches!(
            &disclosed[0].code,
            dorc_aid::diag::DiagCode::PlanImportRewritten(payload)
                if payload.verb == "repointed" && payload.names == "./pkg.dorc-bundle.sh"
        ),
        "and says what it now names: {:?}",
        disclosed[0].code
    );
    let mut spine = dorc_plan::Spine::new();
    dorc_plan::spine::record_render_decisions(
        &mut spine,
        &plan,
        dorc_core::influence::InfluenceAccount::authored_before_contact(),
    );
    assert!(
        spine.render_decisions().iter().any(|record| matches!(
            record.decision(),
            dorc_core::spine::RenderDecision::ImportRewritten { verb, names }
                if *verb == "repointed" && names == "./pkg.dorc-bundle.sh"
        )),
        "the decision plane holds it too: {:?}",
        spine.render_decisions()
    );
}

#[test]
fn twin_guard23_rundelta_never_guards() {
    // né guard23-rundelta-never-guards: `systemctl restart nginx`. `restart` is a run-delta verb the
    // oracle does NOT model (no effect arm) ⇒ no fact/verdict ⇒ never a guard (nor elide) ⇒ runs. An
    // oracle can't guard a verb it declined to describe.
    let (rendered, plan) = render_service_for("systemctl restart nginx\n", &[]);
    assert!(
        !is_guarded(&plan, "systemctl restart nginx"),
        "an unmodeled run-delta verb never guards"
    );
    assert!(
        !is_replaced(&plan, "systemctl restart nginx"),
        "nor elides ⇒ runs"
    );
    assert!(
        rendered.contains("systemctl restart nginx"),
        "the run-delta command runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_explicit_rc_consumers_run() {
    // né guard23-explicit-rc-consumers-run: three converged installs whose rc is CONSUMED — an `if`
    // guard, a `||` left operand, and a `$?`-read predecessor. A mutator's rc is ⊤ (fork-mutator-rc),
    // so each StatusRelaxable consumer blocks the license ⇒ all three RUN, none guard/elide.
    let (rendered, plan) = render_for(
        "if apt-get install -y nginx; then echo ok; fi\napt-get install -y curl || echo fallback\napt-get install -y vim; rc=$?\necho \"rc was $rc\"\n",
        &[
            ("package", "nginx"),
            ("package", "curl"),
            ("package", "vim"),
        ],
    );
    for pkg in ["install -y nginx", "install -y curl", "install -y vim"] {
        assert!(
            !is_guarded(&plan, pkg) && !is_replaced(&plan, pkg),
            "the rc-consumed `{pkg}` runs (StatusRelaxable + ⊤ blocks) — no guard, no elide: {:?}",
            plan.steps()
                .iter()
                .map(|s| (&s.sh, &s.disposition))
                .collect::<Vec<_>>()
        );
    }
    assert!(
        rendered.contains("if apt-get install -y nginx; then")
            && rendered.contains("apt-get install -y curl || echo fallback")
            && rendered.contains("apt-get install -y vim"),
        "all three rc-consumer sites run verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_canttell_plan_runs() {
    // né guard23-canttell-plan-runs: the past-`hork` curl install is vouched (Written) but the host
    // reports it CANT-TELL (Unknown verdict) ⇒ GuardLicense::mint returns None off a non-Converged
    // verdict ⇒ no guard ⇒ runs. The ambient nginx (converged) still elides. (Unknown→no-guard is
    // also mint-unit-pinned by guard_mints_only_on_a_converged_probe_verdict; this is the pipeline.)
    let (rendered, plan) = render_scoped(
        "apt-get install -y nginx\nhork wombat\napt-get install -y curl\n",
        &[("package", "nginx", "installed")],
        &[("package", "curl", "installed")],
        &["package"],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the converged ambient nginx elides"
    );
    assert!(
        !is_guarded(&plan, "install -y curl"),
        "a cant-tell (Unknown) verdict never mints a guard: {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        !is_replaced(&plan, "install -y curl"),
        "and cant-tell doesn't elide ⇒ curl runs"
    );
    assert!(
        rendered.contains("apt-get install -y curl"),
        "the cant-tell install runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_vouch_gates_elision() {
    // né guard23-vouch-gates-elision: WITH a vouch the converged install ELIDES; WITHOUT one it RUNS.
    // apt is vouched (kind `package`), systemctl is NOT (kind `service` excluded) — both converged.
    // The nginx install elides (vouch gates the elision); the systemctl enable runs (no vouch).
    let (rendered, plan) = render_scoped(
        "apt-get install -y nginx\nsystemctl enable nginx\n",
        &[
            ("package", "nginx", "installed"),
            ("service", "nginx", "enabled"),
        ],
        &[],
        &["package"],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "the VOUCHED converged install elides"
    );
    assert!(
        !is_replaced(&plan, "systemctl enable nginx"),
        "the UNVOUCHED converged service enable runs (vouch gates the elision): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        !is_guarded(&plan, "systemctl enable nginx"),
        "and it is not guarded (no vouch)"
    );
    assert!(
        rendered.contains("# apt-get install -y nginx")
            && rendered.contains("\nsystemctl enable nginx"),
        "nginx elides, systemctl runs verbatim:\n{rendered}"
    );
}

#[test]
fn twin_guard23_cross_oracle_vouch_scoped() {
    // né guard23-cross-oracle-vouch-scoped (23C-fd9 / vouch-scope-is-the-body-never-the-tool): apt's
    // vouch is scoped to apt's own sites. `systemctl enable foo` PAST the `hork` wall is
    // EstablishProbeWritten but UNVOUCHED by apt (different kind) ⇒ no guard ⇒ runs. The apt nginx install
    // (before the wall, vouched) elides — proving the vouch reached its OWN kind but not systemctl's.
    let (rendered, plan) = render_scoped(
        "apt-get install -y nginx\nhork wombat\nsystemctl enable foo\n",
        &[
            ("package", "nginx", "installed"),
            ("service", "foo", "enabled"),
        ],
        &[],
        &["package"],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "apt's own vouched install elides"
    );
    assert!(
        !is_guarded(&plan, "systemctl enable foo"),
        "apt's vouch does NOT guard systemctl's past-wall site (vouch-scope): {:?}",
        plan.steps()
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        !is_replaced(&plan, "systemctl enable foo"),
        "and it doesn't elide (past the wall) ⇒ runs"
    );
    assert!(
        rendered.contains("hork wombat") && rendered.contains("\nsystemctl enable foo"),
        "the wall and the unvouched service site run verbatim:\n{rendered}"
    );
}

// ===========================================================================
// `30Qe:fruit-emit-hygiene-paste-rules` — the first splice-floor damage-watch pin (`KNOBS:kBOOT`):
// a rendered artifact's physical lines carry no canonical-tty-cap or leading-`~` paste hazard.
// ===========================================================================

/// CFG shape: a single top-level `Simple` leaf with no matching provider (unmodeled) ⇒ no edit,
/// no oracle, so the leaf ships byte-identical from book to artifact — the shape that proves
/// `paste_hygiene_hazards` inspects RENDERED bytes, not source text, since here the two coincide.
/// A single authored physical line at [`dorc_plan::render::CANONICAL_TTY_LINE_CAP_BYTES`] must be
/// DETECTED, never silently shipped.
#[test]
fn paste_hygiene_flags_a_line_at_the_canonical_tty_cap() {
    let src = format!(
        "printf '%s' {}\n",
        "a".repeat(dorc_plan::render::CANONICAL_TTY_LINE_CAP_BYTES)
    );
    let (rendered, _plan) = render_for(&src, &[]);
    let hazards = dorc_plan::render::paste_hygiene_hazards(&rendered);
    assert!(
        hazards
            .iter()
            .any(|h| matches!(h, dorc_plan::render::PasteHygieneHazard::LineTooLong { .. })),
        "a line at the canonical-tty cap must be flagged, not silently shipped: {rendered}"
    );
}

/// CFG shape: a single top-level `Simple` leaf whose command word is a bare `~`-prefixed word
/// (POSIX tilde-expansion parses it; no matching login name leaves it unmodified) — unmodeled, no
/// edit, ships byte-identical. `paste_hygiene_hazards` must DETECT the leading `~` (the
/// SOL/ssh-serial escape a live paste would hand to the ssh client, never the remote shell).
#[test]
fn paste_hygiene_flags_a_line_beginning_with_tilde() {
    let (rendered, _plan) = render_for("~doesnotexist arg\n", &[]);
    let hazards = dorc_plan::render::paste_hygiene_hazards(&rendered);
    assert!(
        hazards.iter().any(|h| matches!(
            h,
            dorc_plan::render::PasteHygieneHazard::LeadingTilde { .. }
        )),
        "a line beginning `~` must be flagged: {rendered}"
    );
}

/// Negative control (feature-off): an ordinary short, non-tilde render carries NO paste-hygiene
/// hazard — without this, the two tests above could pass on a detector that fires on everything.
#[test]
fn paste_hygiene_is_silent_on_an_ordinary_render() {
    let (rendered, _plan) = render_for("apt-get install -y nginx\n", &[]);
    assert!(
        dorc_plan::render::paste_hygiene_hazards(&rendered).is_empty(),
        "an ordinary short render must carry no paste-hygiene hazard: {rendered}"
    );
}
