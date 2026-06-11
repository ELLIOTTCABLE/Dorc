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
//! REFINED (F1/`19D`): vouched (replaceable) only when the consumer's rc is *known* —
//! an `if`/`elif` guard always blocks (`f1_status_consumed_by_if_guard_blocks_replacement`,
//! unconditional render floor); a `&&`/`||` left operand
//! (`andor_left_operand_undeclared_rc_runs_kfail_perform`) blocks on an undeclared/⊤ rc
//! (the `kFAIL-perform` floor), where eliding to a fabricated rc-0 would change which
//! branch/operand runs.
//!
//! ROUND-20 (`fork-mutator-rc` adopted, notes/201 §1 + 202 §5): a MUTATOR's rc has NO
//! sanctioned source — the probe never runs mutators, and oracle-declared rc-values
//! are rejected ("no values except what the probe gives us"). So a branch-consumed
//! converged mutator RUNS, full stop; that lost elision is the ruling's deliberate
//! cost (19H §2.3). The engine's `StatusRelaxable`-relaxes-on-declared-rc seam STAYS —
//! it is what probe-sourced *Query-guard* rcs ride next (202 §2); only the mutator-rc
//! injection that previously exercised it here is gone (it was the masking 19I §2
//! strips).
//!
//! ROUND-20 C-3 (19A C-3 / 205 §2, task-E): `set -e` and `$?` are now HONORED as
//! ordinary rc-consumers, not special-cased-as-vouched. The engine marks an
//! errexit-region command's rc — and a `$?`-reader's predecessor's rc — the
//! value-relaxable `StatusRelaxable` (`analysis/tests/cfg.rs`
//! `consumed_errexit_marks_relaxable_status_c3` / `consumed_dollar_question_*`). Composed
//! with `fork-mutator-rc` (a mutator's rc is
//! ⊤): a converged mutator under `set -e` now RUNS (`errexit_consumed_top_status_runs_c3`
//! below) — the priority-2 over-execute the committed engine hid by leaving errexit
//! un-marked is closed. A *known/probe-sourced* rc still folds (the relaxation seam),
//! so a conforming converged establish with a declared rc-0 stays replaceable; only the
//! ⊤-rc case is lost. (fs-4 still holds at the EFFECT layer: `set -e` does not POISON
//! ambient-ness — the install stays `EstablishAmbient`; it runs for the *status* reason,
//! not the poison reason. The adjacent spec at the bottom pins exactly that separation.)
//!
//! NOTE — most cases below deliberately omit `set -e` to isolate the observable
//! dimension; the C-3 errexit cell is exercised explicitly where named. `set -e` is
//! target-state-pure (fs-4) so it does not poison the effect analysis, but under C-3 it
//! DOES consume each command's status — two distinct effects a real defensive book
//! carries at once.
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

use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_core::{
    EntityRef, Interner, KindId, Observable, OpaqueToken, Predicted, ProviderId, Rc, SelectorId,
    Verdict,
};
use dorc_oracle::{KindIndex, Polarity};
use dorc_plan::{Disposition, Plan, build_plan};

/// Corpus-shaped apt-get check (flag-strip → verb → single-operand `package`
/// annotation with a `[ "$2" = "" ]` multi-operand refusal). The matrix only models
/// install/purge on `package`, so no `update`/Singleton arm is needed. Lifted with
/// the test's interner so provider symbols match the book's command words.
const CORPUS_CHECK_SRC: &str = r#"
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then probe-pkg "$pkg"; fi
}
"#;

/// Run value-flow + the corpus checks + classify, returning the classified leaves.
fn classify_value(
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    idx: &KindIndex,
    i: &mut Interner,
) -> Vec<(dorc_analysis::cfg::CfgNodeId, SkipClass)> {
    let value = dorc_analysis::value::analyze(cfg, ast, i);
    let checks = vec![dorc_oracle::check::lift_checks(i, CORPUS_CHECK_SRC).value];
    dorc_analysis::effect::classify(cfg, &value, idx, &checks, i).value
}

/// The package oracle: `apt-get install ⇒ establishes package`, `apt-get purge ⇒
/// kills`. Round-20: whether the tool is "idempotent-success" (a converged install
/// exits 0) no longer matters to these tests — a mutator's rc is ⊤ regardless
/// (`fork-mutator-rc`); only the errexit consumer still leans on the
/// establishes-contract vouch, structurally (see the module doc).
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
    let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
    build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        // No rc is ever carried for these mutator facts: `fork-mutator-rc` (adopted,
        // 202 §5) — a mutator's status is ⊤ (`inv-probe-sourced-values`); only the
        // Effect channel (convergence) arrives from the probe.
        if held.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
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

// ===========================================================================
// PINS — current behaviour that is correct; keep it correct.
// ===========================================================================

// NOTE: the verdict-axis baselines (diverged⇒run, converged⇒replace for a lone
// install) live in the plan-unit e2e tests (`diverged_install_runs`,
// `converged_ambient_install_is_replaced_rest_runs`), which subsume them — this
// matrix isolates the OBSERVABLE dimension, so every cell below assumes converged.
//
// "status consumed by `set -e`" IS now a clean cell (C-3, 205 §2): `set -e` is
// target-state-pure (fs-4) so the install stays `EstablishAmbient` and DOES reach the
// status question, where its ⊤-rc `StatusRelaxable` mark blocks the license ⇒ it runs
// (`errexit_consumed_top_status_runs_c3`). The `&&`/`||` cells exercise the same
// rc-relaxable status from a different locus.

#[test]
fn pins_converged_status_via_andand_runs_mutator_rc_top() {
    // observable=STATUS, consumed=YES (&& reads the rc), converged — but the rc of a
    // MUTATOR has no sanctioned source (`fork-mutator-rc`, 202 §5): the probe never
    // runs `apt-get`, so its status is ⊤ and the `StatusRelaxable` floor refuses the
    // license ⇒ the install RUNS. (Pre-round-20 this pinned `Replace` via an injected
    // conforming rc=0 — the masking class 19I §2 strips.) The lost elision is the
    // ruling's deliberate cost; the relaxation seam re-activates for probe-sourced
    // Query-guard rcs (202 §2), never for mutators. HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx && systemctl enable nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a branch-consumed converged mutator runs: its rc is ⊤ (no fabricated rc-0)"
    );
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
fn pins_converged_status_via_oror_runs_mutator_rc_top() {
    // observable=STATUS, consumed=YES (|| reads the rc — the dangerous dual of &&),
    // converged. Same `fork-mutator-rc` disposition as the `&&` pin above: no
    // sanctioned source for the install's rc ⇒ ⊤ ⇒ the `StatusRelaxable` floor refuses ⇒
    // RUNS. This is also the safer floor for the `||` shape specifically: a fabricated
    // rc-0 here would suppress the `|| handler` — the 19D under-execute family.
    // HOST: installed.
    let plan = plan_for(
        "apt-get install -y nginx || systemctl start nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a ||-consumed converged mutator runs: its rc is ⊤ (no fabricated rc-0)"
    );
}

// ===========================================================================
// F1/C-3 — the status A/B contrast (`notes/195` F1; 19A C-3 / 205 §2 honored).
// The round-16 model decided "no status gate" (rc-0 vouched by the establishes-
// contract). That is unsound wherever the rc is unknown and a decision turns on it. The
// settled model: a status consumer blocks the license unless the rc is *known*. An
// `if`/`elif` GUARD blocks unconditionally (render floor). An errexit-region command's
// status is consumed too (C-3: `set -e` reads every rc — NOT special-cased-as-vouched);
// composed with `fork-mutator-rc` (mutator rc ⊤), a converged mutator under `set -e`
// RUNS. These two cases ARE that contrast — same converged install, both run, by locus:
// one via the if-guard floor, one via the C-3 errexit ⊤-rc block.
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
fn errexit_consumed_top_status_runs_c3() {
    // B (FLIPPED for 19A C-3 / 205 §2): observable=STATUS, consumed=YES by ERREXIT
    // (`set -e`), converged. The committed engine special-cased this ("errexit-status
    // stays vouched, still elides") — the human's C-3 ruling rejects that: `set -e`
    // reads every command's rc (non-zero ⇒ abort), so it is an ordinary status consumer,
    // marked the value-relaxable `StatusRelaxable`. Composed with `fork-mutator-rc` (a
    // mutator's rc is ⊤, never a fabricated rc-0), the `StatusRelaxable` floor refuses the
    // license ⇒ the install RUNS. This closes the priority-2 over-execute the old vouch
    // hid: a NON-conforming converged establish under `set -e` (one that exits non-zero
    // when converged) would abort a real run, which eliding to `true` silently masked.
    // A *known/probe-sourced* rc would still relax (the seam survives — `set -e` Query
    // guards fold later, 202 §2); only the ⊤-rc mutator is lost. HOST: nginx installed.
    let plan = plan_for(
        "set -e\napt-get install -y nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "errexit-consumed ⊤-rc status RUNS (C-3: not special-cased-as-vouched)"
    );
}

#[test]
fn cmd_consuming_dollar_question_blocks_predecessor() {
    // C-3's second consumer (19A C-3 / 205 §2): a `$?`-reader makes its PREDECESSOR a
    // status consumer. `apt-get install -y nginx` (converged) then `[ $? -ne 0 ] && echo
    // recover`: the install's rc is read by `$?`, so it is marked `StatusRelaxable`. Its rc
    // is ⊤ (`fork-mutator-rc` — a mutator has no sanctioned rc), so the license is
    // refused ⇒ the install RUNS. The committed engine left `$?` un-marked, so it would
    // have wrongly elided the install to `true` (rc 0) and suppressed the `recover`
    // branch a real non-conforming run would take — the priority-1 exposure C-3 names.
    // HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx\n[ $? -ne 0 ] && echo recover\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a converged mutator whose rc `$?` reads must run (⊤ rc, no fabricated rc-0)"
    );
}

// (Round-20 cut, `fork-mutator-rc`: `andand_left_operand_declared_rc0_relaxes_and_replaces`
// asserted the declared-mutator-rc relaxation — rc=0 hand-injected for `apt-get install`.
// No sanctioned source can produce a mutator's rc, so the test was masking (19I §2). The
// engine's relaxation seam survives untested-here until probe-sourced Query-guard rcs land
// (202 §2) — the e2e `fold-oror-guard-omits` carries the fold-from-known-rc behavior
// meanwhile, via its stdin guard-rc, dying in stage-2.)

#[test]
fn andor_left_operand_undeclared_rc_runs_kfail_perform() {
    // `19D` THE DEFAULT-PATH FIX (the prompt's required assertion, the un-masking of the
    // fabricated-rc-0 under-execute): a converged establish consumed as a `&&`/`||` LEFT
    // operand with NO declared rc must **Run** — never `Replace`/`Omit`. With no rc the
    // value-preserving stand-in would default to `true` (rc 0), a fabricated success;
    // for a non-conforming establish (`useradd` exits 9 converged) that suppresses the
    // `|| fallback` — the priority-1 `kFAIL-perform` under-execute the round-19
    // adversarial pass proved. Here `apt-get install` is converged but its rc is
    // UNDECLARED (verdict-only): `StatusRelaxable` consumed + rc None ⇒ the license is
    // refused ⇒ Run. Round-20: with `fork-mutator-rc` adopted, undeclared is the ONLY
    // state a mutator's rc can be in — this floor is now the rule, not the default-half
    // of a declared/undeclared split.
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
    let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
    // Converged, but NO rc declared (the real CLI/hostsim default after `19D` — an
    // un-injected rc is ⊤, never a fabricated 0).
    let plan = build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        if f == nginx {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
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
// door-3 (`20V` §4 / note 213): `cmd || true` — the admin's spelled-in-sh "this rc is
// not load-bearing". The left operand is `StatusInvariant`-consumed (never blocks), so a
// converged mutator there MINTS a license even at ⊤ rc — the ONLY behavioral delta. The
// classification pins (which channel) are in `analysis/tests/cfg.rs`; these pin the gate
// collapse + disposition. Contrast `andor_left_operand_undeclared_rc_runs_kfail_perform`
// directly above: a plain `|| systemctl start` left RUNS (StatusRelaxable + ⊤), a
// `|| true` left REPLACES (StatusInvariant + ⊤).
// ===========================================================================

#[test]
fn door3_oror_true_converged_mutator_is_replaced() {
    // The payoff: `apt-get install -y nginx || true`, converged. The install's rc is ⊤
    // (`fork-mutator-rc`), but its consumer is `|| true` ⇒ `StatusInvariant` ⇒ the gate does
    // NOT block ⇒ the convergence-elision license mints. The stand-in is `True` — licensed
    // by INVARIANCE (both `||` continuations rejoin identically), not by a claim the mutator
    // exits 0 (weld-5 intact). HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx || true\n",
        &[("package", "nginx")],
    );
    assert!(
        is_replaced(&plan, "install -y nginx"),
        "a converged `cmd || true` left mints despite ⊤ rc (door-3: StatusInvariant never blocks)"
    );
    assert!(
        plan.steps.iter().any(|s| s.sh.contains("install -y nginx")
            && matches!(&s.disposition, Disposition::Replace(_, stand_in) if stand_in.sh() == "true")),
        "the door-3 stand-in is `true` (the idiom, not a predicted rc-0)"
    );
}

#[test]
fn door3_oror_true_diverged_mutator_runs_effect_still_gates() {
    // door-3 clears ONLY the Status channel; the Effect channel still gates. A DIVERGED
    // `cmd || true` must RUN — the StatusInvariant mark unblocks Status, but
    // `prove_replaceable`'s convergence check (`verdict.resolve() != Replaceable`) refuses a
    // non-converged effect. This pins that door-3 is NOT an elision-license relaxation.
    // HOST: nginx NOT installed (diverged).
    let plan = plan_for("apt-get install -y nginx || true\n", &[]);
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a diverged `cmd || true` runs: door-3 clears Status, Effect (convergence) still gates"
    );
}

#[test]
fn door3_oror_false_converged_mutator_still_runs() {
    // The `|| false` pole at the gate level: `false` changes the list rc, so the left
    // operand keeps the blocking `StatusRelaxable` mark (not StatusInvariant). Composed with
    // ⊤ (`fork-mutator-rc`), the gate refuses ⇒ the install RUNS. door-3 must NOT widen to
    // `|| false`. HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx || false\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "`|| false` keeps the blocking StatusRelaxable ⇒ converged ⊤-rc mutator runs (not door-3)"
    );
}

#[test]
fn door3_oror_chain_true_replaces_only_outer_left() {
    // d-3 asymmetry at the plan level: `a || b || true`, both converged. `a` (nginx) keeps
    // StatusRelaxable (inner `||` reads its rc) ⇒ runs. `b` (curl) is StatusInvariant (outer
    // `|| true`) ⇒ replaces. Two same-command sites must NOT collapse (`inv-site-keyed-results`).
    // HOST: nginx + curl both installed.
    let plan = plan_for(
        "apt-get install -y nginx || apt-get install -y curl || true\n",
        &[("package", "nginx"), ("package", "curl")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "a (inner-`||` left) runs — its rc controls whether b runs (StatusRelaxable + ⊤)"
    );
    assert!(
        is_replaced(&plan, "install -y curl"),
        "b (outer `|| true` left) replaces — its rc is dead-in-fact (StatusInvariant)"
    );
}

#[test]
fn door3_oror_true_then_dollar_question_runs_residual() {
    // The documented RESIDUAL (`20V` §4 UNIT PINS / note 213): `cmd || true; echo $?`. The
    // `$?`-reader marks the predecessors (`cmd` AND `true`) `StatusRelaxable` via the
    // pred-walk, and that blocking mark is added to `cmd`'s set alongside its
    // StatusInvariant. Mark-union: a present StatusRelaxable + ⊤ blocks ⇒ `cmd` RUNS. This is
    // acceptable-conservative this slice (kFAIL-perform): although `$?` after `cmd || true` is
    // ALWAYS 0 (the list rc is invariant), the pred-walk does not yet know the read is
    // invariant, so it blocks. A later slice that propagates the list-rc-invariance to the
    // `$?`-read could unlock it. HOST: nginx installed.
    let plan = plan_for(
        "apt-get install -y nginx || true\necho $?\n",
        &[("package", "nginx")],
    );
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "`cmd || true; echo $?` runs the mutator: the $?-pred StatusRelaxable mark blocks \
         (residual — conservative, could later unlock via list-rc-invariance)"
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
// ADJACENT FINDING (effect-poison / fs-4) — surfaced by this matrix, distinct from
// BOTH the observable-liveness gate AND the C-3 status mark. fs-4: target-state-pure
// builtins (`set`, `echo`, `cd`, `:`, `[`/`test`) must classify Pure, not Opaque, so
// they do not poison downstream ambient-ness. That fix is at the EFFECT layer
// (classify). C-3 (205 §2) is a SEPARATE, later gate: even with `set -e` non-poisoning
// at the effect layer, `set -e` consumes each command's status, so a ⊤-rc mutator under
// it runs anyway. The exclusion-check below keeps the two cleanly apart — the install
// stays `EstablishAmbient` (fs-4 holds) yet its plan disposition is Run (C-3 blocks).
// ===========================================================================

#[test]
fn spec_set_e_pure_at_effect_layer_but_c3_status_blocks() {
    // HOST: nginx installed (converged). Two separate claims, exclusion-checked apart:
    //
    // fs-4 (EFFECT layer): `set -e` toggles a shell option and touches NO package fact,
    // so it must NOT poison — the install stays `EstablishAmbient` (NOT `EstablishWritten`
    // / `MustRun`). Were `set -e` still Opaque ⇒ Reach::Top, the install would be Written.
    //
    // C-3 (STATUS layer, 205 §2): `set -e` nonetheless consumes the install's rc, which
    // for a mutator is ⊤ (`fork-mutator-rc`) ⇒ the license is refused ⇒ disposition Run.
    //
    // So the install runs for the *status* reason, NOT the poison reason — the round's
    // honest headline cost. (Pre-C-3 this asserted `is_replaced`; the committed engine's
    // un-marked errexit made it pass, which is exactly the C-3 hole task-E closes.)
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let nginx = FactKey {
        kind: KindId(i.intern("package")),
        entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        selector: installed,
    };
    let src = "set -e\napt-get install -y nginx\n";
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
    // fs-4: the install is EstablishAmbient (set -e did not poison the effect analysis).
    let install_is_ambient = classes
        .iter()
        .any(|(_, class)| matches!(class, SkipClass::EstablishAmbient(f) if *f == nginx));
    assert!(
        install_is_ambient,
        "fs-4: set -e is target-state-pure ⇒ the install stays EstablishAmbient (not poisoned)"
    );
    // C-3: but its ⊤-rc status is errexit-consumed ⇒ Run.
    let plan = build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        if f == nginx {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    });
    assert!(
        !is_replaced(&plan, "install -y nginx"),
        "C-3: errexit-consumed ⊤-rc status ⇒ the install runs (despite fs-4 non-poison)"
    );
}

// ===========================================================================
// QUERY GUARDS (task-D2 — 202 §2, the read-only guard-class). A `command -v X`
// guard is now a first-class Query: its OWN probed rc is fold-usable as the Status
// channel (gated by rule-query-validity), so the canonical `guard || mutator` idiom
// folds and the guard itself is value-preservingly substituted. These cases exercise
// BOTH rc directions (Build 5 — the Exit(n) revival), the validity gate (the
// invalidation pin), and the consumption gates.
//
// FIREWALL FIDELITY: `plan_query` mirrors the cli's firewall (`facts_from_sites`) — a
// Query site's probed rc reaches the fold's Status ONLY when the site's
// `SkipClass::QueryResolvable { valid }` bit holds; an establish site's rc never does.
// So these are honest end-to-end (classify → firewall → plan), not Observable-injection
// fakes (`inv-probe-sourced-values` anti-masking: the status a guard's check predicts
// is exactly the probed rc, never hand-injected past the validity gate).
// ===========================================================================

/// apt-get check + the `command -v` check (verbless: strips `-v`, annotates the
/// operand as `tool`). Lifted with the test's interner so provider symbols match.
const CORPUS_CHECK_SRC_Q: &str = r#"
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then probe-pkg "$pkg"; fi
}
command__check() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1
}
"#;

/// package install/purge + the read-only `command '' query present` guard on `tool`.
fn query_index(i: &mut Interner) -> KindIndex {
    let mut idx = package_index(i);
    let tool = KindId(i.intern("tool"));
    let present = SelectorId(i.intern("present"));
    let command = ProviderId(i.intern("command"));
    let eps = dorc_oracle::empty_verb(i);
    idx.add_effect(command, eps, tool, present, Polarity::Query);
    idx
}

/// Run the whole pipeline with a Query guard, mirroring the cli's wrong-concrete
/// FIREWALL: the `tool:<guard_tool>#present` Query cell is observed with `guard_rc`,
/// but that rc reaches the fold's Status ONLY when the classified site is a VALID
/// `QueryResolvable` (else withheld — status ⊤). `package:<e>#installed` cells are
/// answered verdict-only by `pkg_holds` (a mutator's rc is always ⊤). The Effect verdict
/// of the guard cell is derived from its rc (0 ⇒ Converged/holds, else Diverged).
fn plan_query(src: &str, guard_tool: &str, guard_rc: i32, pkg_holds: &[&str]) -> Plan {
    let mut i = Interner::default();
    let idx = query_index(&mut i);
    let installed = SelectorId(i.intern("installed"));
    let present = SelectorId(i.intern("present"));
    let tool_kind = KindId(i.intern("tool"));
    let package = KindId(i.intern("package"));
    let guard_fact = FactKey {
        kind: tool_kind,
        entity: EntityRef::Operand(OpaqueToken(i.intern(guard_tool))),
        selector: present,
    };
    let pkg_facts: Vec<FactKey> = pkg_holds
        .iter()
        .map(|e| FactKey {
            kind: package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: installed,
        })
        .collect();

    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
    let checks = vec![dorc_oracle::check::lift_checks(&mut i, CORPUS_CHECK_SRC_Q).value];
    let classes = dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut i).value;

    // Mirror the cli firewall: is the guard site a VALID Query? (Only then does its rc
    // reach Status.) Read the bit off the guard cell's classification.
    let guard_valid = classes.iter().any(|(_, c)| {
        matches!(c, SkipClass::QueryResolvable { fact, valid: true } if *fact == guard_fact)
    });
    let guard_effect = if guard_rc == 0 {
        Verdict::Converged
    } else {
        Verdict::Diverged
    };

    build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        if f == guard_fact {
            Observable {
                effect: guard_effect,
                // THE FIREWALL: feed the guard's own rc only when valid; else withhold.
                status: if guard_valid {
                    Predicted::Value(Rc(guard_rc))
                } else {
                    Predicted::Top
                },
                // stdout/stderr unpredicted this round (19F §3 shape; nothing produces them).
                stdout: Predicted::Top,
                stderr: Predicted::Top,
            }
        } else if pkg_facts.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    })
}

/// Is the leaf containing `needle` **omitted** (fold-dead branch)? Distinct from
/// `is_replaced` (a value-preserving substitution of a live leaf).
fn is_omitted(plan: &Plan, needle: &str) -> bool {
    plan.steps
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Omit { .. }))
}

#[test]
fn query_guard_holds_omits_install_and_substitutes_guard() {
    // THE composition (202 §2 / task-D2, the headline idempotency idiom): `command -v
    // nginx || apt-get install` with the guard's probed rc=0 (nginx on PATH) and no
    // upstream mutation (valid). The fold reads the guard's KNOWN rc 0 ⇒ the `||`
    // install is provably DEAD ⇒ Omit. The guard itself mutates nothing, its rc is
    // known + `||`-consumed (StatusRelaxable) ⇒ value-preservingly substituted
    // (Replace ⇒ `true`). HOST: nginx present; package irrelevant (the branch is dead).
    let plan = plan_query(
        "command -v nginx >/dev/null 2>&1 || apt-get install -y nginx\n",
        "nginx",
        0,
        &[],
    );
    assert!(
        is_omitted(&plan, "apt-get install"),
        "guard rc 0 ⇒ the || install is fold-dead (Omit): {:?}",
        plan.steps
            .iter()
            .map(|s| (&s.sh, &s.disposition))
            .collect::<Vec<_>>()
    );
    assert!(
        is_replaced(&plan, "command -v nginx"),
        "the Query guard itself is value-preservingly substituted (Replace ⇒ true)"
    );
}

#[test]
fn query_guard_absent_keeps_install_live_exit_revival() {
    // Build 5, the OTHER rc direction (the Exit(n) revival, us-sure-drift / 20B): the
    // guard's probed rc=1 (nginx NOT on PATH) ⇒ the `||` install is LIVE (it runs). The
    // guard's branch decision is still fully resolved (rc 1 known), so the guard itself
    // is substitutable — its stand-in is `StandIn::from_rc(1)` = `false` (the formerly
    // zero-coverage non-zero-rc path). The install must NOT be omitted nor replaced.
    let plan = plan_query(
        "command -v nginx >/dev/null 2>&1 || apt-get install -y nginx\n",
        "nginx",
        1,
        &[],
    );
    assert!(
        !is_omitted(&plan, "apt-get install"),
        "guard rc 1 ⇒ the || install is LIVE (not dead)"
    );
    assert!(
        !is_replaced(&plan, "apt-get install"),
        "the install is diverged (absent) ⇒ runs, not replaced"
    );
    // The guard substitutes to `false` (rc 1) — the Exit(n)/from_rc non-zero path.
    let guard = plan
        .steps
        .iter()
        .find(|s| s.sh.contains("command -v nginx"))
        .expect("the guard is a leaf");
    assert!(
        matches!(&guard.disposition, Disposition::Replace(_, stand_in) if stand_in.sh() == "false"),
        "the guard substitutes to its exact rc-1 stand-in `false`: {:?}",
        guard.disposition
    );
}

#[test]
fn query_guard_invalid_after_mutator_runs_for_real() {
    // THE invalidation pin (rule-query-validity, 205 §2 / 20A §4 st-3): the SAME guard
    // BELOW an oracled mutator (`apt-get install -y curl` establishes
    // package:curl#installed) ⇒ a write reaches the guard from entry ⇒ the guard is an
    // INVALID Query ⇒ the firewall withholds its rc (status ⊤) ⇒ the fold cannot resolve
    // the `||` ⇒ the nginx install stays LIVE, and the guard itself runs for real
    // (StatusRelaxable-consumed + ⊤ rc ⇒ no license). curl install runs (diverged). Nothing
    // folds — the guard re-runs against the possibly-changed state (kFAIL-perform).
    let plan = plan_query(
        "apt-get install -y curl\ncommand -v nginx >/dev/null 2>&1 || apt-get install -y nginx\n",
        "nginx",
        0,
        &["curl"],
    );
    assert!(
        !is_omitted(&plan, "apt-get install -y nginx"),
        "invalid guard ⇒ rc withheld ⇒ the nginx install is NOT folded dead (runs)"
    );
    assert!(
        !is_replaced(&plan, "command -v nginx"),
        "an invalid Query guard's rc is stale ⇒ it is NOT substituted (runs for real)"
    );
}

#[test]
fn query_guard_consumed_stdout_blocks_substitution() {
    // The consumption gate honored for Query guards too (Build 5: "a guard whose stdout
    // is consumed still blocks"): `out=$(command -v nginx)` captures the guard's stdout,
    // which is value-bearing and vouched by nothing (16F §3) ⇒ the guard must run, NOT
    // substitute — even though it is a valid Query with a known rc. (The `$()`-internal
    // command is also expansion-internal, so it is not even a plan leaf — either way it
    // is never Replaced.) HOST: nginx present.
    let plan = plan_query("out=$(command -v nginx)\necho \"$out\"\n", "nginx", 0, &[]);
    assert!(
        !is_replaced(&plan, "command -v nginx"),
        "a stdout-consumed Query guard blocks substitution (runs)"
    );
}

/// Drive the full pipeline keeping the AST, so a test can assert on `render_apply`
/// (which needs both the `Plan` and the `&Ast`). Mirrors `plan_for` but returns the
/// parsed tree alongside the plan.
fn plan_and_ast(src: &str, holds: &[(&str, &str)]) -> (Plan, dorc_syntax::ast::Ast) {
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
    let plan = build_plan(src, &parsed.value, &cfg, &classes, move |f| {
        if held.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    });
    (plan, parsed.value)
}

#[test]
fn render_one_liner_case_arm_body_substitutes_in_situ_keeping_arm_structure() {
    // T14 (notes/199 cluster-C): a converged install that is the body of a one-liner
    // `case` arm (`pat) cmd ;;` all on one line) must be substituted IN-SITU — only the
    // command span replaced by its stand-in — NOT whole-line commented. Commenting the
    // whole line would swallow the `nginx)`/`;;` scaffolding, leaving `case nginx in`
    // followed by a bare stand-in where a `pat)` is required (a `dash -n` syntax error,
    // the defect the standing xfail pinned). HOST: nginx installed (converged ⇒ replace).
    let src = "case nginx in\n  nginx) apt-get install -y nginx ;;\n  *) : ;;\nesac\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "nginx")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains("  nginx) true ;;"),
        "the arm body is substituted in situ, `nginx)` + `;;` intact:\n{rendered}"
    );
    assert!(
        !rendered.contains("# nginx) apt-get install"),
        "the structural `nginx)`/`;;` is NOT commented out (the T14 mangling):\n{rendered}"
    );
    // The `*) : ;;` arm and the `case`/`esac` keywords pass through verbatim.
    assert!(
        rendered.contains("case nginx in")
            && rendered.contains("  *) : ;;")
            && rendered.contains("esac"),
        "the surrounding case structure is preserved:\n{rendered}"
    );
}

#[test]
fn render_multi_line_case_arm_body_substitutes_span_in_situ() {
    // Re-homed for arch-1 (was `…_keeps_whole_line_comment_form`): the leaf-exact render
    // substitutes a leaf's exact byte-span regardless of whether it shares the `pat)`/`;;`
    // line. An OWN-LINE arm body now substitutes in-situ too — the line-granular whole-line
    // comment form is retired (the "source line" was the wrong unit; the leaf span is). The
    // `case`/arm scaffolding is untouched.
    let src = "case nginx in\n  nginx)\n    apt-get install -y nginx\n    ;;\n  *) : ;;\nesac\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "nginx")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains(
            "    true   # dorc: elided [apt-get install -y nginx] (already converged / dead branch)"
        ),
        "an own-line arm body is span-substituted in-situ (leaf-exact render):\n{rendered}"
    );
    // The pattern line `  nginx)` and the `;;`/`case`/`esac` scaffolding survive verbatim.
    assert!(
        rendered.contains("\n  nginx)\n") && rendered.contains("\n    ;;\n"),
        "the `nginx)` pattern + `;;` scaffolding are untouched:\n{rendered}"
    );
}

// ===========================================================================
// task-F2 (20O find-2): in-situ substitution for a Replace leaf sharing its line
// with LOOP / `if` scaffolding (`done`/`for`/`fi` …). The T14 case-arm fix
// generalised: whole-line commenting `done; install` swallows the `done`, breaking
// `dash -n` (the apply aborts MID-RUN on the host — violating fail-before-network).
// The render keeps the keyword and substitutes only the leaf span.
// ===========================================================================

#[test]
fn render_post_loop_install_sharing_done_line_substitutes_in_situ() {
    // 20O find-2 case (a): a converged install on the loop's `done` line. The loop body
    // (`echo "$x"`, value-flow-pure so the post-loop install stays elidable) is on the
    // `do` line; `done` shares the install's line. Whole-line commenting would yield
    // `# done; …` ⇒ `for…do…` with no terminator ⇒ `dash -n` "expecting done". The fix
    // keeps `done` and substitutes the install (`done; true`). HOST: nginx converged.
    let src = "for x in a b; do echo \"$x\"\ndone; apt-get install -y nginx\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "nginx")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains("\ndone; true   # dorc: elided"),
        "the `done` keyword is kept; only the install is substituted (`done; true`):\n{rendered}"
    );
    assert!(
        !rendered.contains("# done;"),
        "the `done` is NOT commented out (the find-2 mangling that breaks `dash -n`):\n{rendered}"
    );
}

#[test]
fn render_pre_loop_install_sharing_for_line_substitutes_in_situ() {
    // 20O find-2 case (b): a converged install BEFORE a loop, on the `for` opener line.
    // The body is on the next line, so the install's line has no Run leaf to protect it;
    // whole-line commenting would eat `for x in a`. The fix keeps the opener and
    // substitutes the install (`true; for x in a`).
    let src = "apt-get install -y nginx; for x in a\ndo echo \"$x\"; done\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "nginx")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains("true; for x in a   # dorc: elided"),
        "the `for …` opener is kept; the install is substituted (`true; for x in a`):\n{rendered}"
    );
    // The `do …; done` body line is untouched (the loop still has its body + `done`).
    assert!(
        rendered.contains("\ndo echo \"$x\"; done\n"),
        "the loop body + `done` survive verbatim:\n{rendered}"
    );
}

#[test]
fn render_post_if_install_sharing_fi_line_substitutes_in_situ() {
    // 20O find-2 case (c): a converged install on the `if`'s closing `fi` line. The guard
    // (`true`) and then-body (`echo y`) are on the `if`/`then` line; `fi` shares the
    // install's line. Whole-line commenting would eat `fi` ⇒ `if…then…` unterminated. The
    // fix keeps `fi` and substitutes the install (`fi; true`).
    let src = "if true; then echo y\nfi; apt-get install -y nginx\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "nginx")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains("\nfi; true   # dorc: elided"),
        "the `fi` keyword is kept; the install is substituted (`fi; true`):\n{rendered}"
    );
    assert!(
        !rendered.contains("# fi;"),
        "the `fi` is NOT commented out:\n{rendered}"
    );
}

#[test]
fn render_own_line_then_body_substitutes_span_in_situ() {
    // Re-homed for arch-1 (was `…_keeps_whole_line_comment_form`, the `guarded` e2e shape):
    // a converged install that is the then-body and sits ALONE on its line is now
    // span-substituted in-situ to its stand-in `true` (the leaf-exact render — the
    // whole-line comment form is gone). The `if true; then`/`fi` scaffolding is untouched
    // (the guard `true` is Pure ⇒ Run ⇒ no edit).
    let src = "if true; then\n   apt-get install -y nginx\nfi\necho done\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "nginx")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains(
            "   true   # dorc: elided [apt-get install -y nginx] (already converged / dead branch)"
        ),
        "an own-line then-body install is span-substituted in-situ to `true`:\n{rendered}"
    );
    // The `then`/`fi` keywords survive verbatim on their own lines.
    assert!(
        rendered.contains("if true; then\n") && rendered.contains("\nfi\n"),
        "the `if`/`then`/`fi` scaffolding is intact:\n{rendered}"
    );
}

#[test]
fn render_multiline_leaf_on_scaffolding_line_substitutes_cleanly() {
    // Re-derived for arch-1 (d-6; was `…_refuses_license_and_runs_verbatim`): a MULTI-LINE
    // leaf is NEWLY EXPRESSIBLE under the leaf-exact render — a span edit may cover multiple
    // source lines, so a converged install whose argv operand carries a LITERAL NEWLINE
    // (span crosses two lines) while sharing the loop's `done` line is substituted CLEANLY:
    // its whole span collapses to `true`, keeping `done` (the line-render's old multi-line
    // refusal is retired). The provenance comment's embedded original newline is flattened
    // (else the `#` comment would split into a stray unterminated-quote line).
    let src = "for x in a b; do echo \"$x\"\ndone; apt-get install -y \"multi\nline\"\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "multi\nline")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains("\ndone; true   # dorc: elided ["),
        "the multi-line install is span-substituted to `true`, `done` kept:\n{rendered}"
    );
    // The provenance comment is a SINGLE line (the operand's interior newline flattened).
    assert!(
        !rendered.contains("\nline\"]"),
        "the comment's embedded original newline is flattened (no stray `line\"]` line):\n{rendered}"
    );
    // No `# done` (the loop terminator is never commented/eaten).
    assert!(
        !rendered.contains("# done"),
        "the `done` is NOT commented (the splice must not break the loop):\n{rendered}"
    );
}

// ===========================================================================
// arch-2 (brk-2): the all-or-nothing inlined-CALL license + site N.M probe records.
// `plan_for` drives the whole pipeline; the CALL leaf (whose text is the call word)
// is what elides, not the spliced body. `i-3` (the license, both poles), `i-4` (the
// per-body-site probe sub-records).
// ===========================================================================

/// Is the CALL leaf whose verbatim text is exactly `call` REPLACED? (The call's own span is
/// the render unit — its `Step.sh` is the call text, e.g. `w nginx`.)
fn call_replaced(plan: &Plan, call: &str) -> bool {
    plan.steps
        .iter()
        .any(|s| s.sh.trim() == call && matches!(s.disposition, Disposition::Replace(_, _)))
}

/// Is the CALL leaf whose verbatim text is exactly `call` a RUN step?
fn call_runs(plan: &Plan, call: &str) -> bool {
    plan.steps
        .iter()
        .any(|s| s.sh.trim() == call && matches!(s.disposition, Disposition::Run))
}

#[test]
fn inline_call_converged_body_elides_the_call() {
    // `i-3` pole A (all body establishes converged ⇒ the CALL elides): `w() { apt-get install
    // -y "$1"; }; w nginx`, nginx converged ⇒ the call `w nginx` is Replaced (its whole span →
    // a stand-in). The body is gone; the call is the render unit.
    let plan = plan_for(
        "w() { apt-get install -y \"$1\"; }\nw nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        call_replaced(&plan, "w nginx"),
        "the converged inline call elides (all-or-nothing license, body establish converged)"
    );
}

#[test]
fn inline_call_diverged_body_runs_the_call() {
    // `i-3` pole B (a non-converged body establish ⇒ the CALL runs whole): same wrapper, nginx
    // NOT in `holds` ⇒ Diverged ⇒ the call runs (the real body executes).
    let plan = plan_for("w() { apt-get install -y \"$1\"; }\nw nginx\n", &[]);
    assert!(
        call_runs(&plan, "w nginx"),
        "a diverged body establish runs the whole call (all-or-nothing)"
    );
}

#[test]
fn inline_call_independent_per_call() {
    // `i-3`: calls are INDEPENDENT — nginx converged ⇒ `w nginx` elides; curl diverged ⇒
    // `w curl` runs. One diverged call does not affect the other's elision.
    let plan = plan_for(
        "w() { apt-get install -y \"$1\"; }\nw nginx\nw curl\n",
        &[("package", "nginx")],
    );
    assert!(call_replaced(&plan, "w nginx"), "the converged call elides");
    assert!(
        call_runs(&plan, "w curl"),
        "the diverged call runs (independent)"
    );
}

#[test]
fn inline_call_with_body_kill_blocks_the_whole_call() {
    // `i-3` all-or-nothing: a body containing a KILL (`apt-get purge`) blocks the call even
    // when the host reports the install converged — a Kill is a mutation with no
    // already-done convergence story, so the whole call runs. `w() { apt-get install -y "$1";
    // apt-get purge -y old; }; w nginx`, nginx converged.
    let plan = plan_for(
        "w() { apt-get install -y \"$1\"; apt-get purge -y old; }\nw nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        call_runs(&plan, "w nginx"),
        "a body Kill blocks the all-or-nothing license ⇒ the call runs"
    );
}

#[test]
fn inline_call_with_unoracled_body_command_blocks() {
    // `i-3` all-or-nothing: a body with an UNORACLED command (Opaque ⇒ MustRun body site)
    // blocks the call — the Opaque could mutate anything. `w() { apt-get install -y "$1";
    // frobnicate; }; w nginx`, nginx converged ⇒ the call still runs.
    let plan = plan_for(
        "w() { apt-get install -y \"$1\"; frobnicate; }\nw nginx\n",
        &[("package", "nginx")],
    );
    assert!(
        call_runs(&plan, "w nginx"),
        "an Opaque body command blocks the all-or-nothing license ⇒ the call runs"
    );
}

#[test]
fn inline_call_emits_site_n_m_probe_records() {
    // `i-4`: an inlined call ships ONE probe check per spliced body establish, keyed
    // `site N.M` (N = the call's LeafId, M = the body-site index) with the entity bound at the
    // call. Two calls (`w nginx`/`w curl`) ⇒ records `site 0.0` (nginx) and `site 1.0` (curl).
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let src = "w() { apt-get install -y \"$1\"; }\nw nginx\nw curl\n";
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
    let probe = dorc_plan::compile_probe(&parsed.value, &cfg, &classes, |_kind, _sel| {
        Some("{ dpkg-query -W \"$1\"; }".to_string())
    });
    // Each check carries (site, member); collect the (site.0, member) pairs.
    let mut keys: Vec<(u32, Option<u32>)> =
        probe.checks.iter().map(|c| (c.site.0, c.member)).collect();
    keys.sort();
    assert_eq!(
        keys,
        vec![(0, Some(0)), (1, Some(0))],
        "two calls ⇒ two body-site sub-records site 0.0 and site 1.0"
    );
    // The bound entities differ (nginx for call 0, curl for call 1) — the back-map
    // non-injectivity (`i-6`) keeps the records distinct by call-site even though the body
    // AstId is shared.
    let entities: Vec<EntityRef> = {
        let mut v: Vec<_> = probe
            .checks
            .iter()
            .map(|c| (c.site.0, c.fact.entity))
            .collect();
        v.sort_by_key(|(s, _)| *s);
        v.into_iter().map(|(_, e)| e).collect()
    };
    assert_eq!(entities.len(), 2);
    assert_ne!(
        entities[0], entities[1],
        "the two calls' body establishes resolve DISTINCT entities (nginx vs curl)"
    );
}

#[test]
fn inline_call_unprobeable_body_establish_is_unresolvable() {
    // `i-4` all-or-nothing probe-ability: if a body establish has no probe body, the WHOLE
    // call is unresolvable (`can't-probe ⇒ can't-elide`). With the probe_body returning None,
    // the call's site appears in `unresolvable`, not `checks`.
    let mut i = Interner::default();
    let idx = package_index(&mut i);
    let src = "w() { apt-get install -y \"$1\"; }\nw nginx\n";
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let classes = classify_value(&cfg, &parsed.value, &idx, &mut i);
    let probe = dorc_plan::compile_probe(&parsed.value, &cfg, &classes, |_kind, _sel| None);
    assert!(probe.checks.is_empty(), "no probe body ⇒ no checks");
    assert!(
        !probe.unresolvable.is_empty(),
        "the call is unresolvable when its body establish can't be probed"
    );
}

#[test]
fn inline_call_inside_loop_is_floored_even_when_converged() {
    // arch-2 + task-L1 composition (note 216 hunt-6, the riskiest edge): an inlined CALL inside
    // a `for` loop is MustRun this round (the in-loop render floor), EVEN when its body
    // establish is converged. `for pkg in nginx; do w "$pkg"; done` with `w` inlining + nginx
    // converged ⇒ the call `w "$pkg"` RUNS (the loop renders verbatim). `inline_disposition`
    // re-checks `in_loop_body` EXPLICITLY (not relying on the back-edge self-poison that also
    // tends to make the in-loop body establish EstablishWritten). The single-member loop makes
    // the for-var a CONCRETE `nginx` (so the positional binds and the body would otherwise be
    // an EstablishAmbient site), isolating the floor as the operative block.
    let plan = plan_for(
        "w() { apt-get install -y \"$1\" >/dev/null 2>&1; }\nfor pkg in nginx; do w \"$pkg\"; done\n",
        &[("package", "nginx")],
    );
    assert!(
        call_runs(&plan, "w \"$pkg\""),
        "an in-loop inlined call is floored (runs) even when converged"
    );
    assert!(
        !call_replaced(&plan, "w \"$pkg\""),
        "the in-loop floor prevents the inlined call from eliding"
    );
}

// ===========================================================================
// P1 fix 21E (note 214 §9 hunt-7): two ADJACENT elidable MULTI-LINE leaves.
// The pre-fix `emit_span_edits` keyed edits by their lone START line and the
// line-walk jumped over a multi-line edit's CONSUMED span — so a second edit
// whose start line fell inside the first's span was ORPHANED, leaving the
// second command half-spliced and the provenance comment landing INSIDE an
// open quote (a `dash -n`-clean-by-coincidence corruption, or a hard
// "Unterminated quoted string" with an odd embedded quote). The fix groups
// line-overlapping/abutting edits and splices them as one rendered line.
// These pins reproduce the two hostile-crosscheck books BYTE-FOR-BYTE and
// assert the fixed render is dash-clean, both leaves substituted, run-set
// empty. The `debug_assert_eq!` every-edit-applied-once counter (f-1) also
// fires under the debug test build if a future change re-orphans an edit.
// ===========================================================================

#[test]
fn render_adjacent_multiline_elides_both_no_orphan() {
    // The P1 reproducer, byte-for-byte: two `apt-get install` leaves, each with a
    // double-quoted operand carrying a LITERAL NEWLINE, separated by `;` so the SECOND
    // leaf STARTS on the FIRST leaf's closing line (`b"; apt-get install -y "c`). Both
    // converged ⇒ both `replace`d to `true`. Pre-fix the second was orphaned (`true;
    // apt-get install -y "c` survived, comment spliced after `"c`); the group splice
    // collapses both to `true; true` on one line, the comment carrying BOTH originals.
    let src = "apt-get install -y \"a\nb\"; apt-get install -y \"c\nd\"\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "a\nb"), ("package", "c\nd")]);
    let rendered = plan.render_apply(src, &ast);
    // Both leaves substituted on ONE rendered line; the orphaned `apt-get install -y "c`
    // is gone, and the comment discloses both originals (interior newlines flattened).
    assert!(
        rendered.contains(
            "true; true   # dorc: elided [apt-get install -y \"a b\"; \
             apt-get install -y \"c d\"] (already converged / dead branch)"
        ),
        "both adjacent multi-line installs collapse to `true; true` with a combined \
         disclosure (no orphaned second leaf):\n{rendered}"
    );
    // The orphan signature: the second install's raw command must NOT survive in the body.
    assert!(
        !rendered.contains("apt-get install -y \"c\n"),
        "the second install is NOT left half-spliced (the pre-fix orphan):\n{rendered}"
    );
    // dash-clean: the corrupt artifact was clean-by-coincidence here, but the FIXED one is
    // clean for the right reason (one comment line, both quotes balanced and closed).
    assert!(
        rendered.lines().filter(|l| l.contains("apt-get")).count() == 1,
        "exactly one rendered line mentions apt-get (the disclosure comment), none runs \
         it:\n{rendered}"
    );
}

#[test]
fn render_adjacent_multiline_odd_embedded_quote_no_dashn_break() {
    // The P1 VARIANT: the first operand has an ODD embedded quote (`a'b`) before its
    // literal newline. Pre-fix, the orphaned second leaf left the rendered line ending
    // `… install -y "c`, and the spliced provenance comment landed inside the open `"…"`
    // — but the embedded `'` in the disclosure flipped quote-state, so `dash -n` failed
    // HARD ("Unterminated quoted string"). The group splice + f-2 comment-safety make the
    // fixed render dash-clean; the odd-quote operand is flattened into the disclosure.
    let src = "apt-get install -y \"a'b\nx\"; apt-get install -y \"c\nd\"\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "a'b\nx"), ("package", "c\nd")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains("true; true   # dorc: elided ["),
        "both installs collapse to `true; true` (the odd-quote operand no longer orphans \
         the second leaf):\n{rendered}"
    );
    // The disclosure carries the odd-quote operand (whitespace-flattened), proving it was
    // routed into the COMMENT, not left live in the body where the `'` broke `dash -n`.
    assert!(
        rendered.contains("apt-get install -y \"a'b x\""),
        "the odd-quote first original is disclosed in the comment (flattened):\n{rendered}"
    );
    assert!(
        !rendered.contains("apt-get install -y \"c\n"),
        "no orphaned second install left in the body:\n{rendered}"
    );
}

#[test]
fn render_multiline_then_single_line_orphan_cousin_both_elide() {
    // The P3 cousin (single-line orphan): a MULTI-LINE converged install followed on its
    // CLOSING line by a SINGLE-LINE converged install (`y"; apt-get install -y curl`).
    // Same orphan mechanism — the second leaf's start line is the first's end line — but
    // both edits are single-rendered-line after splice. The group merges them: `true;
    // true`, both disclosed. (Pre-fix: the `curl` install was orphaned and ran.)
    let src = "apt-get install -y \"x\ny\"; apt-get install -y curl\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "x\ny"), ("package", "curl")]);
    let rendered = plan.render_apply(src, &ast);
    assert!(
        rendered.contains(
            "true; true   # dorc: elided [apt-get install -y \"x y\"; \
             apt-get install -y curl] (already converged / dead branch)"
        ),
        "the multi-line + single-line pair both elide to `true; true` (no orphaned \
         curl):\n{rendered}"
    );
    assert!(
        !rendered.contains("install -y curl\n") || rendered.contains("curl]"),
        "the `curl` install is elided, surviving only inside the disclosure comment:\n{rendered}"
    );
}

#[test]
fn render_verbatim_run_leaf_opening_quote_drops_comment_f2() {
    // f-2 comment-drop (defense-in-depth): a converged install (multi-line, ⇒ `true`)
    // immediately followed on its closing line by a VERBATIM `Run` leaf whose operand
    // OPENS a double-quote that closes on the next line (`b"; systemctl reload "c<LF>d"`).
    // The group is just the install (the systemctl is a Run, no edit), so the spliced
    // region is `true; systemctl reload "c` — it ENDS inside an open quote. `comment_safe`
    // (via `region_ends_in_quote`) must DROP the provenance comment here: appending `#`
    // would land it inside the string literal. The systemctl runs verbatim across both
    // lines; the disclosure is dropped (OOB lane still carries it). dash-clean.
    let src = "apt-get install -y \"a\nb\"; systemctl reload \"c\nd\"\n";
    let (plan, ast) = plan_and_ast(src, &[("package", "a\nb")]);
    let rendered = plan.render_apply(src, &ast);
    // The install is substituted; the systemctl operand survives verbatim across two lines.
    assert!(
        rendered.contains("true; systemctl reload \"c\nd\""),
        "the install ⇒ `true`, the verbatim systemctl spans both lines intact:\n{rendered}"
    );
    // NO comment on the quote-opening line (it would land inside the `"c…` literal).
    assert!(
        !rendered.contains("true; systemctl reload \"c   # dorc:"),
        "the provenance comment is DROPPED (region ends inside an open quote — f-2):\n{rendered}"
    );
}
