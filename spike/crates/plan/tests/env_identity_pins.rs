//! Reds for `30S` — environment identity (`30S:finding-prefix-stripped-at-dispatch` ·
//! `30S:finding-rho-fold-value-blind` · `30S:finding-export-never-fences`). Target
//! behaviour the engine does not implement yet; every test rides
//! `internal_tooling::xfail::xfail_until` per `spike/CLAUDE.md xfail-pins-ride-one-seat`.
//!
//! Harness idiom lifted from `plan/tests/observable_matrix.rs`'s `plan_for`: one fixture
//! provider (`wombat`, verb `ensure`, establishes kind `instance`) driven through the
//! ordinary parse → cfg → classify → vouch → `build_plan` pipeline, with `holds` standing
//! in for the (simulated) probe's converged facts. Pins 1/3/4 inspect per-site
//! `Disposition::Replace` on the resulting [`dorc_plan::Plan`]; pin 2 sits closer to the
//! wrapper machinery itself (`dorc_oracle::wrapper::detect_peel` +
//! `dorc_oracle::entry::compose_chain`), per the design doc's own steer.

use dorc_analysis::cfg::{Cfg, CfgNodeId};
use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_analysis::value::ValueFlow;
use dorc_core::{
    Context, EntityRef, Interner, KindId, Observable, OpaqueToken, ProviderId, SelectorId, Verdict,
};
use dorc_oracle::{KindIndex, ValueClaim};
use dorc_plan::{Disposition, Plan, build_plan};
use dorc_syntax::ast::Ast;
use std::collections::BTreeSet;

const WOMBAT_PREDICT_SRC: &str = r#"
wombat__predict() {
   verb=$1; shift
   ent : instance = "$1"
   if [ "$2" = "" ]; then probe-ent "$ent"; fi
}
"#;

const WOMBAT_VERDICT_SRC: &str = r"
wombat__is_converged() { return 0; }
";

/// The fixture kind index: `wombat ensure <entity>` establishes `instance:<entity>@converged`.
fn wombat_index(i: &mut Interner) -> (KindIndex, SelectorId) {
    let instance = KindId(i.intern("instance"));
    let converged = SelectorId(i.intern("converged"));
    let wombat = ProviderId(i.intern("wombat"));
    let ensure = i.intern("ensure");
    let mut idx = KindIndex::default();
    idx.add_effect(
        0,
        wombat,
        ensure,
        instance,
        converged,
        ValueClaim::Establish,
    );
    (idx, converged)
}

/// Run value-flow + the fixture predict + classify, returning the classified leaves.
fn classify_value(
    cfg: &Cfg,
    ast: &Ast,
    idx: &KindIndex,
    i: &mut Interner,
) -> (Vec<(CfgNodeId, SkipClass)>, BTreeSet<CfgNodeId>, ValueFlow) {
    let value = dorc_analysis::value::analyze(cfg, ast, i);
    let checks = vec![dorc_oracle::predict::lift_predicts(i, WOMBAT_PREDICT_SRC).value];
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
    (classification.value, classification.invalidators, value)
}

/// Vouch every establish-bearing site (`wombat__is_converged` always vouches) — the elision
/// MECHANICS are what these pins exercise, not the vouch gate itself.
fn vouch_all(
    classes: &[(CfgNodeId, SkipClass)],
    value: &ValueFlow,
    interner: &mut Interner,
) -> dorc_plan::Vouches {
    dorc_plan::build_vouches(
        &[WOMBAT_VERDICT_SRC],
        &[],
        &dorc_oracle::closure::HelperIndex::default(),
        classes,
        value,
        interner,
        dorc_analysis::funcenv::LiveDefinitions::unsolved(),
    )
    .0
    .value
}

/// Run the whole pipeline with `holds` as the injected (simulated) probe state: a fact in
/// `holds` (`(kind, entity)`, always selector `converged`, context `HostDefault`) reads
/// Converged; anything else reads Diverged.
fn plan_for(src: &str, holds: &[(&str, &str)]) -> Plan {
    let mut i = Interner::default();
    let (idx, converged) = wombat_index(&mut i);
    let held: Vec<FactKey> = holds
        .iter()
        .map(|(k, e)| FactKey {
            kind: KindId(i.intern(k)),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: converged,
            context: Context::HostDefault,
        })
        .collect();
    let parsed = dorc_syntax::parse(src);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let (classes, invalidators, value) = classify_value(&cfg, &parsed.value, &idx, &mut i);
    let observe = move |f: FactKey| {
        if held.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    };
    build_plan(
        src,
        &parsed.value,
        &cfg,
        &classes,
        &invalidators,
        &vouch_all(&classes, &value, &mut i),
        observe,
        &mut dorc_core::ProvArena::new(),
    )
}

/// Is ANY step containing `needle` replaced (elided to a value-preserving stand-in)?
fn is_replaced(plan: &Plan, needle: &str) -> bool {
    plan.steps()
        .iter()
        .any(|s| s.sh.contains(needle) && matches!(s.disposition, Disposition::Replace(_, _)))
}

/// Per-occurrence replacement, in source order — for books whose sites render byte-identical
/// text, so a single `is_replaced` cannot tell the first site's disposition from the second's.
fn replaced_by_occurrence(plan: &Plan, needle: &str) -> Vec<bool> {
    plan.steps()
        .iter()
        .filter(|s| s.sh.contains(needle))
        .map(|s| matches!(s.disposition, Disposition::Replace(_, _)))
        .collect()
}

/// `p-x-prefix-assignment-splits-fact-identity` (`30S:rul-prefix-joins-site-identity`):
/// two sites differing ONLY in a leading assignment-prefix VALUE must not share fact
/// state — a fact probed converged for the `AWS_PROFILE=staging`-prefixed site must never
/// license an elision at the `AWS_PROFILE=prod`-prefixed site.
#[test]
fn prefix_assignment_splits_fact_identity() {
    let plan = plan_for(
        "AWS_PROFILE=staging wombat ensure web1\nAWS_PROFILE=prod wombat ensure web1\n",
        &[("instance", "web1")],
    );
    let reps = replaced_by_occurrence(&plan, "wombat ensure web1");
    assert_eq!(
        reps.len(),
        2,
        "two prefixed sites, byte-identical rendered command"
    );
    assert!(
        reps[0] && reps[1],
        "interim: the prefix is dropped at dispatch (`30S:finding-prefix-stripped-at-dispatch`), \
         so both prefixed sites share one FactKey and both elide off the single held fact"
    );
    internal_tooling::xfail::xfail_until("p-x-prefix-assignment-splits-fact-identity", || {
        assert!(
            !reps[1],
            "target: distinct prefix VALUES mint distinct fact identity, so the prod-prefixed \
             site must never elide off a fact measured under staging"
        );
    });
}

/// `p-x-env-wrapper-context-carries-values` (`30S:finding-rho-fold-value-blind`; `27K` §8's
/// disclosed debt): the `env`-utility spelling must compose a `Context` that differs when a
/// claimed variable's VALUE differs — `env A=a …` and `env A=b …` must never share a
/// wrapped-context cell.
#[test]
fn env_wrapper_context_carries_values() {
    let mut i = Interner::default();
    let peel_a = dorc_oracle::wrapper::detect_peel(&predict_from(
        &mut i,
        "x__predict() { env A=a \"$@\"; }",
    ))
    .expect("`env A=a \"$@\"` peels");
    let peel_b = dorc_oracle::wrapper::detect_peel(&predict_from(
        &mut i,
        "x__predict() { env A=b \"$@\"; }",
    ))
    .expect("`env A=b \"$@\"` peels");
    let ctx_a = dorc_oracle::entry::compose_chain(&[dorc_oracle::entry::ChainLink {
        shifts: std::collections::BTreeMap::default(),
        rho: peel_a.rho,
    }])
    .to_context(&mut i);
    let ctx_b = dorc_oracle::entry::compose_chain(&[dorc_oracle::entry::ChainLink {
        shifts: std::collections::BTreeMap::default(),
        rho: peel_b.rho,
    }])
    .to_context(&mut i);
    assert_eq!(
        ctx_a, ctx_b,
        "interim: `RhoAccum::fold`/`RhoClaim` carry the claimed variable's NAME only, so \
         `env A=a` and `env A=b` compose one wrapped-context key"
    );
    internal_tooling::xfail::xfail_until("p-x-env-wrapper-context-carries-values", || {
        assert_ne!(
            ctx_a, ctx_b,
            "target: the claimed variable's VALUE must join the composed context key, so \
             `env A=a` and `env A=b` never share a wrapped-context cell"
        );
    });
}

/// Lift a single-provider predict body for the peel-detection pin above.
#[expect(
    clippy::expect_used,
    reason = "test fixture helper: a malformed fixture is the failure mode, panic is correct"
)]
fn predict_from(i: &mut Interner, src: &str) -> dorc_oracle::predict::Predict {
    let set = dorc_oracle::predict::lift_predicts(i, src);
    assert!(set.diags.is_empty(), "clean predict lift: {:?}", set.diags);
    let p = set.value.providers().next().expect("one provider");
    set.value.get(p).expect("the predict").clone()
}

/// `p-x-ambient-export-fences-fact-transport` (`30S:rul-export-is-an-index-fence`): a fact
/// established above an ambient `export` must never Must-transport below it.
#[test]
fn ambient_export_fences_fact_transport() {
    let plan = plan_for(
        "wombat ensure web1\nexport VERBOSE=true\nwombat ensure web1\n",
        &[("instance", "web1")],
    );
    let reps = replaced_by_occurrence(&plan, "wombat ensure web1");
    assert_eq!(reps.len(), 2, "one site above, one below the export");
    assert!(
        reps[0] && reps[1],
        "interim: `export` never fences transport (`30S:finding-export-never-fences`), so the \
         below-export site shares the above site's fact and both elide"
    );
    internal_tooling::xfail::xfail_until("p-x-ambient-export-fences-fact-transport", || {
        assert!(
            !reps[1],
            "target: an ambient export is an index fence — the below-export site must not \
             Must-transport the above-export fact"
        );
    });
}

/// `p-x-unwitnessed-env-delta-withholds-probe` (`30S:model-pin-or-sever-composition`): a
/// verdict body that neither pins nor severs a book env-delta variable is not probeable below
/// that delta — the site must withhold toward run/guard rather than measuring under an
/// unwitnessed environment, even against a hand-fed convergent fact.
#[test]
fn unwitnessed_env_delta_withholds_probe() {
    let plan = plan_for(
        "export VERBOSE=true\nwombat ensure web1\n",
        &[("instance", "web1")],
    );
    assert!(
        is_replaced(&plan, "wombat ensure web1"),
        "interim: no pin-or-sever machinery exists yet (`30S:model-pin-or-sever-composition`), \
         so a hand-fed converged fact still elides the site"
    );
    internal_tooling::xfail::xfail_until("p-x-unwitnessed-env-delta-withholds-probe", || {
        assert!(
            !is_replaced(&plan, "wombat ensure web1"),
            "target: the verdict body neither pins nor severs VERBOSE, so the site must \
             withhold — never elide — below the unwitnessed delta"
        );
    });
}
