//! The terminal certifier-trip cleanup (`302` §3 `rul-certifier-trip-guard-only`).
//!
//! A trip means the solver and the solve-certifier DISAGREE, and the pair shares substrate (`Eq`,
//! canonical forms, the transfer model), so the disagreement cannot distinguish a solver-class
//! defect from a substrate-class one: it disqualifies BOTH, and neither may testify afterward.
//! This pass is what stops them testifying — one flat walk over the finished plan, immediately
//! before emission, demoting every elision-family outcome in the scope to run.
//!
//! # What it is NOT
//!
//! It is not a floor. The `302` §3 consumer floors (value · funcenv · reach · self-reach) are
//! unchanged and still fire in place, mid-pipeline, because a terminal pass cannot un-ship a wrong
//! pinned body — by the time a plan exists the probe has already been compiled and run. This is
//! the thin cross-window policy layered ABOVE them: it exists because a window that certified
//! clean may still have consumed an answer from a window that did not, and no per-window floor can
//! see that.
//!
//! It is also not recovery, and none is coming (`302` §9). Nothing here re-plans, carves a region,
//! or buys value back — re-planning would re-consult the machinery the trip disqualified.
//!
//! # Why guards may stand
//!
//! `( check ) || <original bytes>` re-verifies live, on the host, at apply time, and the original
//! command survives verbatim as the `||`-right whatever the check says. Everything a guard rests
//! on is therefore re-measured — except ONE analysis-chosen conjunct: WHICH body the name
//! resolves to. The body-occupancy census answers exactly that conjunct without consulting any
//! solve (`dorc_analysis::funcenv::DefinitionTable::occupancy` is a syntactic count), so a guard
//! whose family is census-unique keeps a live net that a trip cannot have corrupted, and one whose
//! family is plural demotes with everything else.

use dorc_aid::CollapseNarrative;
use dorc_aid::narrative::{CollapseKind, DemoteTag, SpeechAct};

use crate::{Disposition, Plan};

/// What the cleanup did (`302` §5): the count for the plan-prominent banner, and the per-site
/// narrative records that stay pull-tier.
#[derive(Debug, Clone, Default)]
pub struct TripCleanup {
    demoted: usize,
    narrative: Vec<CollapseNarrative>,
}

impl TripCleanup {
    /// How many sites the cleanup demoted. Zero is a legitimate answer for a tripped run — a plan
    /// that elided nothing loses nothing — and the banner fires on the TRIP, never on this count.
    #[must_use]
    pub fn demoted(&self) -> usize {
        self.demoted
    }

    /// One `Derived`-tier `Demotion` record per demoted site (`collapse-mints-narrative`),
    /// decision-inert like every other narrative.
    #[must_use]
    pub fn narrative(&self) -> &[CollapseNarrative] {
        &self.narrative
    }
}

/// Demote every elision-family outcome in `plan` to run, because a certifier tripped this run.
///
/// The three elision-family outcomes are two dispositions: `Replace` covers both elide-by-proof
/// and SURVIVE (an elision kept past a running wall — the split is in the license's witness, not
/// the verb), and `Omit` is the fold-proved-dead branch. `Run` steps are left alone: runs run.
///
/// `census_unique` answers the body-occupancy question for a guard's verdict funcname. A guard it
/// answers `true` for STANDS; every other guard demotes. Callers with no census in hand answer
/// `false` and take the `FORFEITS:forfeit-certifier-trip-demotes-guards` posture — verbatim plus
/// banner — which is always safe and merely poorer.
///
/// Deliberately stupid, and that is the design: the whole policy is one boolean and this walk
/// (`302:rul-certifier-value-is-stupidity` applied to the trip's consequences).
pub fn demote_on_trip(plan: &mut Plan, census_unique: impl Fn(&str) -> bool) -> TripCleanup {
    let mut out = TripCleanup::default();
    for step in &mut plan.steps {
        let stands = match &step.disposition {
            Disposition::Run => true,
            Disposition::Guard(license) => census_unique(license.insert().fn_name()),
            Disposition::Replace(..) | Disposition::Omit { .. } => false,
        };
        if stands {
            continue;
        }
        step.disposition = Disposition::Run;
        out.demoted = out.demoted.saturating_add(1);
        out.narrative.push(CollapseNarrative::new(
            SpeechAct::Derived,
            CollapseKind::Demotion {
                site: dorc_aid::diag::SiteId::leaf(step.leaf),
                reason: DemoteTag::CertifierTripped,
            },
        ));
    }
    out
}
