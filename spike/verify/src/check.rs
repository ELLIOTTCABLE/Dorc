//! The cheap gate (`301` §5 gate tiers) — everything checkable with no external toolchain.
//!
//! It runs on both platform legs and must stay green on a box with neither Lean nor Kani
//! installed: those are opt-in lanes, loud only when explicitly requested. What it checks is
//! the SHAPE of the corpus and the coherence of the catalogue against it — the class of rot
//! that a rename, a move or a forgotten promote produces, which is most of the rot there is.

use std::path::Path;

use crate::badge::{Badge, Evidence};
use crate::catalogue::LawRow;
use crate::catalogue_lock::LAWS;
use crate::evidence::{self, Tier};
use crate::unit::{self, BYTE_BUDGET, Statement, Unit};
use crate::{binding, seat};

/// Everything one cheap-gate pass found. Empty `failures` is the pass condition; `advisories`
/// never fail anything.
#[derive(Debug, Default)]
pub struct Findings {
    /// Refusals, each a complete sentence naming what to do.
    pub failures: Vec<String>,
    /// Advisory notes — the byte-budget tripwire and its kin. Printed, never fatal.
    pub advisories: Vec<String>,
}

/// Run the cheap gate.
///
/// # Errors
/// When the corpus cannot be read at all — a missing `minispec/Minispec/` is a broken
/// checkout, not a finding.
pub fn run(repo_root: &Path) -> Result<Findings, String> {
    let mut findings = Findings::default();
    let units = unit::load_all(repo_root)?;
    let proposals = binding::proposals(repo_root)?;

    for unit in &units {
        check_unit(unit, &mut findings);
    }
    for law in &LAWS {
        check_law(law, &units, repo_root, &mut findings);
    }
    // Both directions: a unit nobody catalogued is as much a hole as a catalogued unit that
    // vanished — it looks like coverage in the directory listing and is coverage nowhere else.
    for unit in &units {
        if !LAWS.iter().any(|law| law.slug == unit.slug) {
            findings.failures.push(format!(
                "{} is not in the catalogue (promote it, or delete the unit)",
                unit.path.display()
            ));
        }
    }
    for d in binding::disagreements(&LAWS, &proposals, repo_root) {
        findings.failures.push(binding::describe(&d));
    }

    let generated = repo_root.join("minispec").join("Generated");
    if generated.is_dir() {
        let (holes, _axioms) = crate::pipeline::census(&generated)?;
        if holes > 0 {
            findings.failures.push(format!(
                "minispec/Generated/ carries {holes} proof hole(s): a hole typechecks, so every \
                 law downstream of one is vacuous. Re-run `mise run verify:translate` (STRICT) \
                 and read what it refuses"
            ));
        }
    }
    Ok(findings)
}

fn check_unit(unit: &Unit, findings: &mut Findings) {
    if !unit::slug_is_well_formed(&unit.slug) {
        findings.failures.push(format!(
            "{}: slug `{}` is not DromedaryCase with at least three full words",
            unit.path.display(),
            unit.slug
        ));
    }
    if unit.has_hole {
        findings.failures.push(format!(
            "{}: a unit STATES a law and may never carry a proof hole (proofs live in \
             Minispec/Proofs/)",
            unit.path.display()
        ));
    }
    if unit.bytes > BYTE_BUDGET {
        findings.advisories.push(format!(
            "{}: {} bytes, over the {BYTE_BUDGET}-byte advisory budget — consider decomposing. \
             Readability and sanity trump the limit; this line is the whole enforcement",
            unit.path.display(),
            unit.bytes
        ));
    }
    if unit.statement == Statement::Missing {
        findings.failures.push(format!(
            "{}: declares no `def {} : Prop` and carries no `{}` marker — a unit file must \
             either state a law or say it does not yet",
            unit.path.display(),
            unit.slug,
            unit::UNWRITTEN_MARKER
        ));
    }
}

fn check_law(law: &LawRow, units: &[Unit], repo_root: &Path, findings: &mut Findings) {
    let unit = units.iter().find(|u| u.slug == law.slug);
    if unit.is_none() {
        findings.failures.push(format!(
            "{}: catalogued, but minispec/Minispec/{}.lean does not exist",
            law.slug, law.slug
        ));
    }
    if let Err(why) = seat::resolve(law.seat, repo_root) {
        findings
            .failures
            .push(format!("{}: unresolved seat — {why}", law.slug));
    }
    let computed = evidence::compute(law, unit, repo_root, Tier::Cheap);
    for (index, badge) in Badge::ALL.iter().enumerate() {
        let Some(found) = computed.get(index) else {
            continue;
        };
        let expected = law.expectation(*badge);
        if !found.agrees_with(expected) {
            findings.failures.push(format!(
                "{}: `{badge}` promoted as {} but evidence says {} — the gate refuses a mismatch \
                 in EITHER direction, so this is either rot to fix or a promote to run",
                law.slug,
                expected.render(),
                found.render()
            ));
        }
    }
    // A row claiming an engine badge the cheap tier cannot see is not a failure here, but the
    // report must never let it read as confirmed.
    debug_assert!(
        computed
            .iter()
            .zip(Badge::ALL)
            .all(|(e, b)| !(b.needs_external_engine() && matches!(e, Evidence::Earned))),
        "the cheap tier must never mint an external-engine badge"
    );
}
