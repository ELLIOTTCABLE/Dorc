//! Computing a law's badges from evidence (`301` §5) — never from a declaration.
//!
//! Two tiers, and the split is structural rather than a policy anyone remembers to honour:
//! [`Badge::needs_external_engine`] decides which badges this module may answer with a real
//! verdict and which it must answer [`Evidence::NotAtThisTier`]. Cheap checks ride the
//! ordinary gate on both platform legs with zero external toolchains; Lean, Kani and
//! `cargo-mutants` evidence is recomputed in the opt-in lane at the fold/bless tier. Nothing
//! anywhere reads a cached verdict.

use std::path::Path;

use crate::badge::{Badge, Evidence};
use crate::catalogue::LawRow;
use crate::unit::{self, Statement, Unit};

/// Which engines the caller is prepared to run.
///
/// The engine slots are `Option` because the lanes are independently opt-in: a run with Lean
/// and no Kani must answer [`Evidence::NotAtThisTier`] for `pinned` rather than `absent`, or a
/// cheap-plus-Lean run would report every law's Kani pin as missing.
#[derive(Clone, Copy, Debug)]
pub enum Tier<'a> {
    /// The ordinary gate: filesystem and parsing only. No Lean, no Kani, no mutants.
    Cheap,
    /// The opt-in verify lane, with whichever engine verdicts the caller has in hand.
    WithEngines {
        /// Whether `lake build` over `minispec` succeeded; `None` if Lean was not run.
        lean_built: Option<bool>,
        /// What the Kani lane found; `None` if it was not run.
        kani: Option<&'a crate::kani::Report>,
    },
}

/// Every badge's evidence for one law, in [`Badge::ALL`] order.
#[must_use]
pub fn compute(
    row: &LawRow,
    unit: Option<&Unit>,
    repo_root: &Path,
    tier: Tier<'_>,
) -> Vec<Evidence> {
    Badge::ALL
        .iter()
        .map(|badge| one(*badge, row, unit, repo_root, tier))
        .collect()
}

fn one(
    badge: Badge,
    row: &LawRow,
    unit: Option<&Unit>,
    repo_root: &Path,
    tier: Tier<'_>,
) -> Evidence {
    let Some(unit) = unit else {
        return Evidence::Absent(format!("no unit file for {}", row.slug));
    };
    if unit.statement == Statement::Unwritten {
        return Evidence::Absent("unit is an unwritten stub".to_owned());
    }
    match badge {
        Badge::Proved => match lean_verdict(tier) {
            None => Evidence::NotAtThisTier,
            Some(true) => proved(row, repo_root),
            Some(false) => Evidence::Absent("lake build failed".to_owned()),
        },
        Badge::Elaborated => match lean_verdict(tier) {
            None => Evidence::NotAtThisTier,
            Some(lean_built) => {
                if unit.statement != Statement::Stated {
                    Evidence::Absent(format!("no `def {} : Prop`", row.slug))
                } else if lean_built {
                    Evidence::Earned
                } else {
                    Evidence::Absent("lake build failed".to_owned())
                }
            }
        },
        Badge::Interrogated => match lean_verdict(tier) {
            None => Evidence::NotAtThisTier,
            Some(lean_built) => interrogated(unit, lean_built),
        },
        Badge::Pinned => match kani_verdict(tier) {
            None => Evidence::NotAtThisTier,
            Some(report) => pinned(row, report),
        },
        // A named seam. It renders a real "absent" with the reason, so the report nags
        // structurally and forgetting is impossible (`301` §5's gentle-must).
        Badge::Demonstrated => demonstrated(row),
        Badge::KillTested => Evidence::Absent("seam-statement-mutation-unbuilt".to_owned()),
    }
}

fn lean_verdict(tier: Tier<'_>) -> Option<bool> {
    match tier {
        Tier::Cheap => None,
        Tier::WithEngines { lean_built, .. } => lean_built,
    }
}

fn kani_verdict(tier: Tier<'_>) -> Option<&crate::kani::Report> {
    match tier {
        Tier::Cheap => None,
        Tier::WithEngines { kani, .. } => kani,
    }
}

/// `pinned`: the paired harness resolves against the toolchain's OWN harness list, and it
/// verified at its declared bounds.
///
/// Resolution before verdict, and the two refusals say different things on purpose. A harness
/// that does not resolve is a citation pointing at nothing — a rename or a deletion, the class
/// of rot the binder exists to catch — while one that resolved and failed is a finding about
/// the code. Collapsing them would make a deleted harness read exactly like a broken law.
fn pinned(row: &LawRow, report: &crate::kani::Report) -> Evidence {
    let Some(name) = row.harness else {
        return Evidence::Absent("no paired harness".to_owned());
    };
    if !report.resolves(name) {
        return Evidence::Absent(format!(
            "harness `{name}` is not in the toolchain's harness list"
        ));
    }
    if report.is_green(name) {
        Evidence::Earned
    } else {
        Evidence::Absent(format!("harness `{name}` did not verify"))
    }
}

fn proved(row: &LawRow, repo_root: &Path) -> Evidence {
    let Some(rel) = row.proof else {
        return Evidence::Absent("no proof claimed".to_owned());
    };
    let path = repo_root.join(rel);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Evidence::Absent(format!("claimed proof missing: {rel}"));
    };
    if !text.contains(&format!("theorem {}_holds", row.slug)) {
        return Evidence::Absent(format!("{rel} does not declare {}_holds", row.slug));
    }
    if unit::contains_hole(&text) {
        return Evidence::Absent(format!("{rel} carries a proof hole"));
    }
    Evidence::Earned
}

/// `interrogated` needs all THREE halves and none substitutes for another: a green battery with
/// no positive witness is green vacuously (a false precondition proves any implication), a
/// witness that never runs proves nothing at all, and a battery with no coupling proves facts
/// that merely SIT BESIDE the law — every one of them survives a statement edited out from
/// under them (`30B:fnd-battery-never-instantiates-its-own-law`).
fn interrogated(unit: &Unit, lean_built: bool) -> Evidence {
    if unit.battery_entries == 0 {
        return Evidence::Absent("no instance battery".to_owned());
    }
    if !unit.has_nonvacuity_probe {
        return Evidence::Absent(format!(
            "no anti-vacuity probe (`theorem {}_nonvacuous`)",
            unit.slug
        ));
    }
    if !unit.has_coupling {
        return Evidence::Absent(format!(
            "no coupling to the law (`theorem {}{}…`), so the battery is beside the statement \
             rather than an instance of it",
            unit.slug,
            unit::COUPLING_INFIX
        ));
    }
    if lean_built {
        Evidence::Earned
    } else {
        Evidence::Absent("lake build failed".to_owned())
    }
}

/// `demonstrated` is gated on the assertion-subset check, which in turn waits on a product
/// surface that does not exist yet (`301` §7.2a — see `crate::binding`). Until then the badge
/// says so rather than resting on a binding's mere presence: a bound loom that nobody
/// re-verifies is exactly the frozen-referent rot the whole design cites as its cautionary
/// case.
fn demonstrated(row: &LawRow) -> Evidence {
    if row.bindings.is_empty() {
        return Evidence::Absent("no accepted binding".to_owned());
    }
    Evidence::Absent("seam-decision-record-read-mode: assertion subsets unverifiable".to_owned())
}
