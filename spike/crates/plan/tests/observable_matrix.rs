//! Observable / replace state-space matrix — the round-16 (16C–16J) findings as
//! executable, END-TO-END cases (parse → cfg → classify → plan → disposition).
//! The observable-liveness gate has LANDED (16H/16J), so this is a **passing
//! regression suite**: `pins_*` assert behaviour that must stay correct, `spec_*`
//! assert the gate's must-run cases. Only `spec_converged_subst_in_redir_target_poisons`
//! stays `#[ignore]`d — the one deferred gap (HOLE#1, CFG-lowering completeness).
//!
//! THE MODEL (16F, for orientation — verify it against these cases, don't take it
//! on faith): replacing a converged leaf = substituting a `true`-stub that
//! *defaults* every OBSERVABLE — effect→none, status→rc 0, stdout/stderr→empty. A
//! replacement is sound iff, for each observable a downstream consumer reads, the
//! default is acceptable (the observable is dead, or its default is vouched).
//! Vouching today: effect ← convergence (the forward gate, already built);
//! status ← the `establishes` contract (an idempotent establish exits 0 when
//! converged, so rc-0 is free); stdout/stderr ← NOTHING.
//!
//! THE A/B CONTRAST this matrix makes concrete: a CONSUMED *stdout*/*stderr* is NOT
//! fine — its empty default is unvouched — so every `spec_*stdout*` is "not replaced"
//! (the backward observable-liveness gate landed, 16H/16J). A CONSUMED *status* is
//! REFINED (F1/`19D`): vouched (replaceable) for an errexit/post-condition consumer,
//! but NOT for a branch consumer whose rc is UNDECLARED — an `if`/`elif` guard
//! (`f1_status_consumed_by_if_guard_blocks_replacement`, unconditional render floor) or
//! a `&&`/`||` left operand with no declared rc
//! (`andor_left_operand_undeclared_rc_runs_kfail_perform`, the `kFAIL-perform` floor),
//! where eliding to a fabricated rc-0 would change which branch/operand runs. A
//! *declared* rc relaxes the `&&`/`||` case (the value-preserving stand-in reproduces
//! the exact status). So status is establish-discharged ONLY when its rc is genuinely
//! vouched (errexit on a converged establish) or declared.
//!
//! NOTE — these cases deliberately omit `set -e` to isolate the observable
//! dimension. `set -e` is itself un-oracled ⇒ it independently *poisons* downstream
//! ambient-ness (an effect-precision bug, fs-4; see the adjacent spec at the
//! bottom), so a real defensive book carries both confounds at once.
//!
//! INVISIBLE GLOBAL STATE: a book's text never says whether the target is already
//! in the desired state. `plan_for(src, holds)` injects it — `holds` is the set of
//! facts the (simulated) host already has (≈ a `hostsim::Host` seed; a probe would
//! observe exactly these). It is stated per test. Empty `holds` ⇒ everything
//! Diverged (unconverged); a listed fact ⇒ Converged.
//!
//! LAYERING: the consumption fact is the ENGINE's, computed during CFG lowering and
//! asserted directly in `analysis/tests/cfg.rs` (the `consumed_*` tests); this file
//! asserts the END-TO-END collapse of that fact into a run/replace disposition
//! (`inv-superposition`, note 16J). `is_replaced` localizes the disposition check.

use dorc_analysis::effect::FactKey;
use dorc_core::{
    EntityRef, Interner, KindId, Observed, OpaqueToken, ProviderId, Rc, SelectorId, Verdict,
};
use dorc_oracle::{KindIndex, Polarity};
use dorc_plan::{build_plan, Disposition, Plan, StandIn};

/// The package oracle: `apt-get install ⇒ establishes package`, `apt-get purge ⇒
/// kills`. It is **idempotent-success**: a converged `apt-get install` exits 0 —
/// which is what vouches the STATUS default. (Contrast a hypothetical `mkdir`,
/// which exits non-zero when its dir already exists; it is therefore NOT a
/// conforming establish, and the converged-non-zero status hazard it represents is
/// un-modellable here — see the note at the bottom of this file.)
fn package_index(i: &mut Interner) -> KindIndex {
    let package = KindId(i.intern("package"));
    let installed = SelectorId(i.intern("installed"));
    let apt = ProviderId(i.intern("apt-get"));
    let install = i.intern("install");
    let purge = i.intern("purge");
    let mut idx = KindIndex::default();
    idx.add_effect(apt, install, package, installed, Polarity::Establish);
    idx.add_effect(apt, purge, package, installed, Polarity::Kill);
    idx
}

/// Run the whole pipeline (parse → cfg → classify → plan) with `holds` as the
/// injected host state (the invisible global convergence state; see the module
/// doc). A fact in `holds` ⇒ Converged, anything else ⇒ Diverged.
fn plan_for(src: &str, holds: &[(&str, &str)]) -> Plan {
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    // Every cell in this matrix is `<kind>:<entity>#installed` (the install/purge
    // selector — the only one this oracle models), so the host-held facts carry it.
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
    let classes = dorc_analysis::effect::classify(&cfg, &parsed.value, &idx, &mut i);
    build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        // The package oracle is idempotent-success: a converged install exits 0, so a
        // held fact is conforming `rc=0` (the value-preserving substitution's `true`).
        if held.contains(&f) {
            Observed {
                verdict: Verdict::Converged,
                rc: Some(Rc(0)),
            }
        } else {
            Observed::verdict_only(Verdict::Diverged)
        }
    })
}

/// Is the leaf whose verbatim text contains `needle` **replaced** (elided to a value-
/// preserving stand-in)? `false` means it runs — a `Run` step, an `Omit`ted (dead)
/// step rendered verbatim, or not a plan step at all (e.g. expansion-internal).
/// (`Omit` is the fold's dead-branch disposition; `is_replaced` is specifically about
/// the convergence-elision `Replace`, so it does NOT count `Omit`.)
fn is_replaced(plan: &Plan, needle: &str) -> bool {
    plan.steps
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Replace(_, _)))
}

/// The value-preserving [`StandIn`] of the (first) `Replace` leaf containing `needle`,
/// or `None` if it is not a `Replace`. Lets a test assert the substitution reproduces
/// the *exact* observed rc (the `19A §5` value-preserving fix), not just that it is
/// replaced.
fn replace_standin(plan: &Plan, needle: &str) -> Option<StandIn> {
    plan.steps.iter().find_map(|s| match &s.disposition {
        Disposition::Replace(_, stand_in) if s.sh.contains(needle) => Some(*stand_in),
        _ => None,
    })
}

// ===========================================================================
// PINS — current behaviour that is correct; keep it correct.
// ===========================================================================

// NOTE: the verdict-axis baselines (diverged⇒run, converged⇒replace for a lone
// install) live in the plan-unit e2e tests (`diverged_install_runs`,
// `converged_ambient_install_is_replaced_rest_runs`), which subsume them — this
// matrix isolates the OBSERVABLE dimension, so every cell below assumes converged.
//
// "status consumed by `set -e`" cannot be pinned as *replaced* here — `set -e`
// itself poisons (the adjacent spec at the bottom), so the install is
// EstablishWritten and never reaches the status question. The status dimension is
// exercised cleanly below via `&&` and `||`, which sit *after* the install.

#[test]
fn pins_converged_status_via_andand_replaced() {
    // observable=STATUS, consumed=YES (&& reads the rc), converged. `true && …`
    // runs the rhs as a converged install (rc 0) would. HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx && systemctl enable nginx\n",
        &[("package", "nginx")],
    );
    assert!(is_replaced(&plan, "install -y nginx"));
}

#[test]
fn pins_converged_stdout_captured_in_subst_runs() {
    // observable=STDOUT, consumed=YES (captured by $()), converged. Handled today
    // *by accident*: the $()-internal install is excluded as expansion-internal
    // (16B), so it is never a replace candidate ⇒ it runs. HOST: nginx installed.
    // IMPLEMENTOR: once the general observable-liveness gate exists and the
    // temporary subst-internal exclusion is lifted (16C: $()-internals ARE leaves),
    // this must STILL come out "not replaced" — then via stdout-liveness, not the
    // exclusion. (So this pin should survive that refactor.)
    let plan = plan_for(
        "x=$(apt-get install -y nginx)\necho \"$x\"\n",
        &[("package", "nginx")],
    );
    assert!(!is_replaced(&plan, "install -y nginx"));
}

#[test]
fn pins_converged_devnull_discard_replaced() {
    // observable=STDOUT+STDERR, consumed=NO (both to /dev/null — the discard sink the
    // gate must exempt). Replacement stays sound, so the leaf MUST stay replaced once
    // the gate lands — a precision guard (the gate is a scalpel, not a hammer). HOST:
    // nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx > /dev/null 2>&1\n",
        &[("package", "nginx")],
    );
    assert!(is_replaced(&plan, "install -y nginx"));
}

#[test]
fn pins_converged_status_via_oror_replaced() {
    // observable=STATUS, consumed=YES (|| reads the rc — the dangerous dual of &&),
    // converged. rc-0 is vouched by the establish contract ⇒ `true || …` does not
    // fire the handler, matching a converged install (also rc 0). HOST: installed.
    let plan = plan_for(
        "apt-get install -y nginx || systemctl start nginx\n",
        &[("package", "nginx")],
    );
    assert!(is_replaced(&plan, "install -y nginx"));
}

// ===========================================================================
// F1 — the BRANCH-vs-ERREXIT status A/B contrast (`notes/195` F1, round-19 stopgap).
// The round-16 model decided "no status gate" (rc-0 vouched by the establishes-
// contract). That is sound for a POST-condition / errexit consumer but UNSOUND for a
// guard / PRE-condition consumer (a *different branch* runs on the rc), where eliding
// the converged command to `:` (rc 0) destroys the branch decision — a `kFAIL-perform`
// under-execute. The fix: **branch**-consumed status (the test of an `if`/`elif`)
// blocks the license; **errexit**-consumed status (`set -e`) stays vouched. These two
// cases ARE that contrast — same converged install, opposite disposition by locus.
// ===========================================================================

#[test]
fn f1_status_consumed_by_if_guard_blocks_replacement() {
    // A: observable=STATUS, consumed=YES by an `if` GUARD (a different branch runs on
    // the rc), converged. `apt-get install` used AS the guard is a pre-condition
    // consumer: eliding it to `:` would force the branch (and orphan `then`). The
    // status is branch-consumed ⇒ it MUST block ⇒ the guard runs (the safe floor; the
    // value-recovering fix is Half-B subsumption). HOST: nginx installed (converged).
    // (`notes/195` F1: this reproduces with ONLY the package oracle, install-as-guard
    // — no new oracle needed; it is the same wrong-classification as `if ! command -v`.)
    let plan = plan_for(
        "if apt-get install -y nginx; then echo done; fi\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "status consumed by an if-guard must block the license (the guard runs, not elides)"
    );
}

#[test]
fn f1_status_consumed_by_errexit_stays_vouched() {
    // B: observable=STATUS, consumed=YES by ERREXIT (`set -e`), converged. This is the
    // contrast to A: errexit-status IS the establishes-contract's domain (a converged
    // idempotent establish exits 0, so `set -e` does not abort), so it stays vouched ⇒
    // the install is still replaced. Gating *all* consumed status (the over-gate the
    // 16G "load-bearing" tension warned of — under `set -e` every status is consumed)
    // would never elide anything; the resolution is to gate BRANCH-status only. HOST:
    // nginx installed. (`set -e` is target-state-pure — fs-4 — so it does not poison.)
    let plan = plan_for(
        "set -e\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "errexit-consumed status stays vouched by the establishes-contract ⇒ still replaced"
    );
}

#[test]
fn andand_left_operand_declared_rc0_relaxes_and_replaces() {
    // `19D` (the former `tc-mint` gap, now RESOLVED): a `&&`/`||` left operand IS marked
    // (`AndOrStatus`), so the caller's rc-conditional collapse decides — no
    // post-condition-vs-guard guess. Here `install && start` is converged with a
    // DECLARED rc 0 (`plan_for` injects it): the `AndOrStatus` block RELAXES (the
    // value-preserving stand-in `true` reproduces the exact rc-0, so `start` decides
    // identically — it would run after a real rc-0 install), and the install stays
    // `Replace`d. CONTRAST `andor_left_operand_undeclared_rc_runs_kfail_perform` below
    // (no declared rc ⇒ block ⇒ run) — the *declared rc*, not a structural guess, is
    // what splits the post-condition (keep) from the under-execute (run).
    let plan = plan_for(
        "apt-get install -y nginx && systemctl enable nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "declared rc-0 relaxes the AndOrStatus block ⇒ the conforming post-condition \
         `install && start` stays replaced (value-preserving rc-0 stand-in)"
    );
}

#[test]
fn andor_left_operand_undeclared_rc_runs_kfail_perform() {
    // `19D` THE DEFAULT-PATH FIX (the prompt's required assertion, the un-masking of the
    // fabricated-rc-0 under-execute): a converged establish consumed as a `&&`/`||` LEFT
    // operand with NO declared rc must **Run** — never `Replace`/`Omit`. With no rc the
    // value-preserving stand-in would default to `true` (rc 0), a fabricated success;
    // for a non-conforming establish (`useradd` exits 9 converged) that suppresses the
    // `|| fallback` — the priority-1 `kFAIL-perform` under-execute the round-19
    // adversarial pass proved. Here `apt-get install` is converged but its rc is
    // UNDECLARED (verdict-only): `AndOrStatus` consumed + rc None ⇒ the license is
    // refused ⇒ Run. The dual of `andand_left_operand_declared_rc0_relaxes_and_replaces`
    // — same construct, opposite disposition, split ONLY by whether the rc is declared.
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let nginx = FactKey {
        kind: KindId(i.intern("package")),
        entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        selector: installed,
    };
    let src = "apt-get install -y nginx || systemctl start nginx\n";
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let classes = dorc_analysis::effect::classify(&cfg, &parsed.value, &idx, &mut i);
    // Converged, but NO rc declared (the real CLI/hostsim default after `19D` — an
    // un-injected rc is ⊤, never a fabricated 0).
    let plan = build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        if f == nginx {
            Observed::verdict_only(Verdict::Converged)
        } else {
            Observed::verdict_only(Verdict::Diverged)
        }
    });
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "undeclared-rc `&&`/`||` left operand must NOT be replaced (kFAIL-perform floor)"
    );
    assert!(
        plan.steps.iter().any(|s| s.sh.contains("install -y nginx")
            && matches!(s.disposition, Disposition::Run)),
        "its disposition is Run — a converged establish whose status is branch-consumed \
         but whose rc is undeclared runs (never a fabricated-rc-0 elision)"
    );
}

// ===========================================================================
// SPECS — the gate's must-run cases: a consumed UNVOUCHED output (stdout/stderr/fd)
// ⇒ run. Formerly the #[ignore]d build-against targets; all pass now the gate has
// landed. (Only the HOLE#1 subst-in-redir-target spec below stays #[ignore]d.)
// ===========================================================================

#[test]
fn spec_converged_stdout_piped_to_grep_must_run() {
    // observable=STDOUT, consumed=YES (piped to grep whose rc then gates `echo`),
    // converged. Replacing ⇒ `true | grep -q nginx` ⇒ empty stdout ⇒ grep no-match
    // ⇒ `echo present` does NOT run, diverging from the real run. STDOUT's empty
    // default is UNVOUCHED ⇒ the leaf must run. HOST: nginx installed.
    // CURRENT: the install is EstablishAmbient + converged ⇒ REPLACED (wrong).
    let plan = plan_for(
        "apt-get install -y nginx | grep -q nginx && echo present\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "stdout piped to grep (which gates echo) is value-bearing; the install must run"
    );
}

#[test]
fn spec_converged_redirect_is_an_effect_must_run() {
    // observable=STDOUT redirected to a real file with NO later reader. The redirect
    // itself (`> /etc/marker` creates/truncates the file — haz-redir-as-mutation) is
    // dropped by the stub, so the leaf must run regardless of whether the content is
    // read; conservative floor: any non-/dev/null output redirect ⇒ run (audit
    // g-redir-effect). HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx > /etc/marker\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "non-/dev/null output redirect ⇒ run"
    );
}

#[test]
fn spec_converged_enclosing_group_redirect_must_run() {
    // 16G kill-shot: the establish is inside `{ … }` and the redirect is on the
    // GROUP, not the leaf — the gate must see the enclosing redirect. HOST: installed.
    let plan = plan_for(
        "{ apt-get install -y nginx; } > /tmp/out\ncat /tmp/out\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "enclosing-group redirect consumes output => run"
    );
}

#[ignore = "SPEC (effect-completeness, deferred 16G HOLE#1): a $() in a redirect-target / case-pattern must lower so its Kill poisons"]
#[test]
fn spec_converged_subst_in_redir_target_poisons() {
    // 16G HOLE#1: `$(apt-get purge nginx)` in a redirect TARGET runs (purges nginx)
    // but is never lowered into the CFG, so its Kill doesn't poison => the install is
    // wrongly EstablishAmbient => replaced. Fix: lower substs in redirect targets +
    // case patterns (a CFG-lowering completeness gap; deferred). HOST: installed.
    let plan = plan_for(
        "apt-get install -y nginx < \"$(apt-get purge nginx)\"\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a Kill in a redirect-target subst must poison the install"
    );
}

#[test]
fn spec_topcontext_background_leaf_must_run() {
    // hole-5 (note 16G): `&` ⊤-rejects (loud parse + cfg-top diagnostics) yet the
    // install is still replaced — build_plan never consults diagnostics, so a ⊤ in a
    // leaf's own statement doesn't inhibit replacing it (an inv-top-reject breach at
    // the plan layer). Benign for a converged no-op, latently unsound (background
    // changes observability: async exit, $!, concurrency). Fix is ⊤-containment, NOT
    // the observable gate. HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx &\necho done\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a leaf whose own construct ⊤-rejects must not be replaced"
    );
}

// ===========================================================================
// ADJACENT FINDING (effect-poison / fs-4) — surfaced by this matrix, NOT fixed by
// the observable-liveness gate. Un-oracled commands are Opaque ⇒ poison downstream
// ambient-ness — including ubiquitous builtins (`set`, `echo`, `cd`, `:`,
// `[`/`test`), none of which touch *target-system* facts, yet each kills
// replacement for everything after it. Since `set -e` opens nearly every defensive
// book, this blocks ~all replacement in practice. The fix lives in the EFFECT
// analysis (classify) — treat target-state-pure builtins as Pure, not Opaque —
// and is distinct from the observable-liveness gate. Included because it is the
// confound that stops "status under set -e" from being a clean cell above.
// ===========================================================================

#[test]
fn spec_converged_set_e_does_not_poison_replacement() {
    // HOST: nginx installed (converged). `set -e` precedes the install; it toggles a
    // shell option and touches NO package fact. EXPECTED: the install is replaced
    // (set -e is irrelevant to package:nginx). CURRENT: NOT replaced — `set -e` is
    // Opaque ⇒ Reach::Top ⇒ the install classifies EstablishWritten. Sound, but
    // ruinously over-conservative (fs-4 on the most common builtin).
    let plan = plan_for(
        "set -e\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "set -e is target-state-pure; it must not poison the install's ambient-ness"
    );
}

// ===========================================================================
// NON-CONFORMING ESTABLISH as an `&&`/`||` LEFT operand — STATUS, converged-FAILURE.
// The round-19 adversarial pass's proven `kFAIL-perform` under-execute, now FIXED at
// two layers (`19C` the fold + value-preserving substitution; `19D` the rc-conditional
// `AndOrStatus` gate + the un-fabricated rc default).
//
// A NON-conforming establish exits NON-ZERO when its fact already holds (`useradd x`
// rc 9, `mkdir d` w/o `-p`, `ln` w/o `-f`, `docker network create`). Consumed as a `||`
// LEFT operand and converged, the OLD engine wrongly `Replace`d it by `:` (rc 0), so
// `useradd deploy || mkdir /x` (user present) became `: || mkdir` ⇒ the `||` never
// fired ⇒ `mkdir` was skipped — the under-execute. Two independent failures fed it:
//   1. the value-preserving substitution did not exist (the stand-in was a flat rc-0
//      `:`), and
//   2. the engine left `&&`/`||` status UNMARKED, and the CLI/hostsim FABRICATED a
//      conforming `rc=0` for any converged fact.
// `19C` fixed (1): the stand-in is now the EXACT observed rc (`(exit 9)`), so a
// *declared* rc-9 keeps `mkdir` live. `19D` fixes (2): the engine marks `&&`/`||`
// status (`AndOrStatus`) and the caller refuses the license when the rc is UNDECLARED
// (no more fabricated rc-0) ⇒ the leaf runs. The two halves the engine could not
// distinguish are now split by the *declared rc*, not a structural guess:
//   * DECLARED rc (the oracle/build-2 produces it): the fold + exact-rc substitution
//     decide — `install && start`[rc 0] stays replaced; `useradd || mkdir`[rc 9] keeps
//     `mkdir` live. (`andand_left_operand_declared_rc0_relaxes_and_replaces` /
//     `nonconforming_establish_andor_left_operand_substitutes_exact_rc`.)
//   * UNDECLARED rc (the default): block ⇒ Run (the `kFAIL-perform` floor —
//     `andor_left_operand_undeclared_rc_runs_kfail_perform`).
// `useradd <name>` fits the (provider, verb) model: verb=`<name>` ⇒ a Singleton
// `user#present` cell (the baked username is a fixture wart, not load-bearing).
//
// The case below keeps the DECLARED-rc-9 opt-in path (the build-2 contract's target):
// the converged `useradd` is replaced by its EXACT `(exit 9)`, so `mkdir` stays live.
// ===========================================================================

/// Run the pipeline for the non-conforming-establish `||` cell. `useradd deploy` is a
/// NON-conforming establish: it exits **9** when `deploy` already exists. The
/// injected observation now carries that rc (`19B` build-1) — the apply fold reads it
/// and the substitution reproduces it, instead of the old `:`/rc-0 lie.
fn plan_for_user_oror(src: &str) -> Plan {
    let mut i = Interner::default();
    let user = KindId(i.intern("user"));
    let present = SelectorId(i.intern("present"));
    let useradd = ProviderId(i.intern("useradd"));
    // `useradd deploy` ⇒ provider=useradd, verb=`deploy`, no further operand ⇒ a
    // Singleton `user#present` cell.
    let deploy = i.intern("deploy");
    let mut idx = KindIndex::default();
    idx.add_effect(useradd, deploy, user, present, Polarity::Establish);
    // The host already has the user (Converged), and the observed exit status of a
    // converged `useradd` is 9 (the non-conformance — now MODELED as an injected rc,
    // the build-2 oracle-contract's job to produce for real).
    let held = FactKey {
        kind: user,
        entity: EntityRef::Singleton,
        selector: present,
    };
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let classes = dorc_analysis::effect::classify(&cfg, &parsed.value, &idx, &mut i);
    build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        if f == held {
            Observed {
                verdict: Verdict::Converged,
                rc: Some(Rc(9)),
            }
        } else {
            Observed::verdict_only(Verdict::Diverged)
        }
    })
}

#[test]
fn nonconforming_establish_andor_left_operand_substitutes_exact_rc() {
    // PROMOTED from `xfail_nonconforming_establish_andor_left_operand_wrongly_replaced`
    // (`notes/198` §1.3 / `19C`). `useradd deploy || mkdir /srv/app`, user `deploy`
    // already present (Converged), observed rc 9. The round-19 fold + value-preserving
    // substitution fix the under-execute: the converged `useradd` IS replaced (it is a
    // converged ambient establish), but by `(exit 9)` — its EXACT observed status — NOT
    // `:`/rc-0. So `(exit 9) || mkdir` still fires the `|| mkdir` fallback ⇒ `mkdir`
    // runs. The old xfail asserted "not replaced" (the only SAFE option when the
    // substitution was a rc-0 lie); value-preserving substitution makes "replaced, but
    // with the right rc" the correct, sound disposition.
    let plan = plan_for_user_oror("useradd deploy || mkdir /srv/app\n");

    // The fix: the stand-in reproduces rc 9, so the `||` still fires.
    assert_eq!(
        replace_standin(&plan, "useradd deploy"),
        Some(StandIn::Exit(9)),
        "the converged non-conforming establish must be replaced by its EXACT rc \
         (exit 9), not a rc-0 stub — else the `|| mkdir` fallback is suppressed \
         (the kFAIL-perform under-execute the round-19 adversarial pass proved)"
    );
    // `mkdir` is live (the fold proved the `||` right operand reachable: left rc 9 ≠ 0)
    // and is not an oracled establish, so it RUNS — never replaced, never omitted.
    assert!(
        !is_replaced(&plan, "mkdir /srv/app"),
        "mkdir runs (the fallback): it is not converged-elidable"
    );
    assert!(
        plan.steps
            .iter()
            .any(|s| s.sh.contains("mkdir /srv/app") && matches!(s.disposition, Disposition::Run)),
        "mkdir's disposition is Run — the fold keeps the `|| mkdir` fallback live"
    );
}
