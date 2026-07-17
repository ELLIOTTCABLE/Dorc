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
    EntityRef, Interner, KindId, Observable, OpaqueToken, ProviderId, SelectorId, Verdict,
};
use dorc_oracle::{KindIndex, ValueClaim};
use dorc_plan::{Disposition, Plan, build_plan};
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

/// The package oracle: `apt-get install ⇒ establishes package#installed`, `purge ⇒ inverts`.
fn package_index(i: &mut Interner) -> KindIndex {
    let package = KindId(i.intern("package"));
    let installed = SelectorId(i.intern("installed"));
    let apt = ProviderId(i.intern("apt_get"));
    let install = i.intern("install");
    let purge = i.intern("purge");
    let mut idx = KindIndex::default();
    idx.add_effect(apt, install, package, installed, ValueClaim::Establish);
    idx.add_effect(
        apt,
        purge,
        package,
        installed,
        ValueClaim::EstablishInverted,
    );
    idx
}

/// Run value-flow + the corpus checks + classify, returning the classified leaves.
fn classify_value(
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    i: &mut Interner,
) -> Vec<(dorc_analysis::cfg::CfgNodeId, SkipClass)> {
    let value = dorc_analysis::value::analyze(cfg, ast, i);
    let checks = vec![dorc_oracle::predict::lift_predicts(i, CORPUS_PREDICT_SRC).value];
    let mut arena = dorc_core::ProvArena::new();
    dorc_analysis::effect::classify(
        cfg,
        &value,
        ast,
        idx,
        &checks,
        &std::collections::BTreeSet::new(),
        i,
        &mut arena,
    )
    .value
}

/// Vouch EVERY establish-bearing site so these render twins exercise the elision MECHANICS (the
/// vouch GATE is pinned elsewhere — plan units + e2e). Ambient-only (an EstablishWritten would fire
/// the guard tier, out of this tier's render scope).
fn vouch_all(classes: &[(dorc_analysis::cfg::CfgNodeId, SkipClass)]) -> dorc_plan::Vouches {
    let mut vouches = dorc_plan::Vouches::new();
    for (node, class) in classes {
        if matches!(class, SkipClass::EstablishAmbient(_)) {
            let vouch = dorc_plan::VerdictVouch::new(
                "apt_get__is_converged".to_string(),
                "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
                "apt_get__is_converged".to_string(),
                "package".to_string(),
                vec!["dpkg-query".to_string()],
            );
            vouches.insert(
                *node,
                dorc_core::ByVouch::vouched(vouch, dorc_core::Rung::Both),
            );
        }
    }
    vouches
}

/// Parse → classify → build_plan → `render_apply` a book, with `holds` the injected host state (a
/// listed `(kind, entity)` cell is Converged, anything else Diverged — the invisible global
/// convergence state). Returns the RENDERED apply artifact plus the `Plan` (for structural
/// disposition asserts). THE dash -n net fires here, on every rendered artifact, so no twin can skip
/// it.
fn render_for(src: &str, holds: &[(&str, &str)]) -> (String, Plan) {
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let held: Vec<FactKey> = holds
        .iter()
        .map(|(k, e)| FactKey {
            kind: KindId(i.intern(k)),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: installed,
        })
        .collect();
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
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
        &vouch_all(&classes),
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
    for shell in ["dash", "sh"] {
        let Ok(mut child) = Command::new(shell)
            .arg("-n")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        else {
            continue;
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(rendered.as_bytes());
        }
        if let Ok(out) = child.wait_with_output() {
            return Some((shell, out));
        }
    }
    None
}

/// Is the leaf whose verbatim text contains `needle` **replaced** (elided to a stand-in)?
fn is_replaced(plan: &Plan, needle: &str) -> bool {
    plan.steps
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Replace(_, _)))
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
        plan.steps
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
        plan.steps
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
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
        plan.steps
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
// identical books). A `purge` (EstablishInverted) KILLS `package:nginx#installed`; the following
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
        plan.steps
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
