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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    /// The ordinary gate: filesystem and parsing only. No Lean, no Kani, no mutants.
    Cheap,
    /// The opt-in verify lane, with the Lean build's verdict in hand.
    WithLean {
        /// Whether `lake build` over `minispec` succeeded.
        lean_built: bool,
    },
}

/// Every badge's evidence for one law, in [`Badge::ALL`] order.
#[must_use]
pub fn compute(row: &LawRow, unit: Option<&Unit>, repo_root: &Path, tier: Tier) -> Vec<Evidence> {
    Badge::ALL
        .iter()
        .map(|badge| one(*badge, row, unit, repo_root, tier))
        .collect()
}

fn one(badge: Badge, row: &LawRow, unit: Option<&Unit>, repo_root: &Path, tier: Tier) -> Evidence {
    let Some(unit) = unit else {
        return Evidence::Absent(format!("no unit file for {}", row.slug));
    };
    if unit.statement == Statement::Unwritten {
        return Evidence::Absent("unit is an unwritten stub".to_owned());
    }
    match badge {
        // Cheap and REAL: a proof is a file that exists, names the theorem, and carries no
        // hole. Its Lean-checkedness rides `elaborated`'s build — this badge answers the
        // narrower question "is there a claimed, hole-free proof at all", which is exactly
        // what a reader wants distinguished from "the whole package builds".
        Badge::Proved => proved(row, repo_root),
        Badge::Elaborated => match tier {
            Tier::Cheap => Evidence::NotAtThisTier,
            Tier::WithLean { lean_built } => {
                if unit.statement != Statement::Stated {
                    Evidence::Absent(format!("no `def {} : Prop`", row.slug))
                } else if lean_built {
                    Evidence::Earned
                } else {
                    Evidence::Absent("lake build failed".to_owned())
                }
            }
        },
        Badge::Interrogated => match tier {
            Tier::Cheap => Evidence::NotAtThisTier,
            Tier::WithLean { lean_built } => interrogated(unit, lean_built),
        },
        // Named seams. Each renders a real "absent" with the reason, so the report nags
        // structurally and forgetting is impossible (`301` §5's gentle-must).
        Badge::Pinned => Evidence::Absent(match row.harness {
            Some(name) => format!("seam-kani-pairing-unbuilt: harness {name} not resolved"),
            None => "seam-kani-pairing-unbuilt: no paired harness".to_owned(),
        }),
        Badge::Demonstrated => demonstrated(row),
        Badge::KillTested => Evidence::Absent("seam-statement-mutation-unbuilt".to_owned()),
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

/// `interrogated` needs BOTH halves and neither substitutes for the other: a green battery
/// with no positive witness is green vacuously (a false precondition proves any implication),
/// and a witness that never runs proves nothing at all.
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
