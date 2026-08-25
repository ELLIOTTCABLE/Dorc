//! The final-presentation witness (`quarantine/30Rb:post-compliance-source-and-identity-advice`).
//!
//! One value carrying the three identities of a finished approval surface into receipt
//! projection. It is NOT an identity: it satisfies no identity API, and nothing downstream may
//! read it as one.
//!
//! It accepts no identity either. Both identities it can compute are computed HERE, from the
//! typed material, in the constructor — so there is no seat at which an identity minted
//! elsewhere could be substituted for one of these. The planned image is the exception by
//! construction rather than by choice: its sole mint is the image's own constructor, and no
//! image exists at plan time, so the field reads absent.

use dorc_aid::Diag;
use dorc_core::Interner;
use dorc_receipt::ids::{ApplyArtifactImageId, PlanningInputId, PresentedPlanId};

use crate::erasability::presented_plan_id;
use crate::planning_input::PlanningInputs;
use crate::{Plan, ProbePlan};

/// The three identities of one finished approval surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinalPresentation {
    planning_input: PlanningInputId,
    presented_plan: PresentedPlanId,
    planned_image: Option<ApplyArtifactImageId>,
}

impl FinalPresentation {
    /// Witness one settled surface.
    ///
    /// The SEAT licenses the mint, exactly as it licenses [`presented_plan_id`]'s: every input
    /// here is final — the `Plan` comes from its one constructor, after settlement quiesced and
    /// the certifier latch was spent, and the canon reads the rendered artifacts — so the human
    /// view, the executable bytes, and every site and region decision are settled before a byte
    /// is hashed. Constructing this earlier would witness a fragment.
    #[must_use]
    #[expect(
        clippy::too_many_arguments,
        reason = "a settled surface IS a wide tuple of independent final artifacts (plan/probe/source/ast/interner/diagnostics) beside the planner's inputs; a params struct would re-spell this signature one layer down while enforcing nothing"
    )]
    pub fn of_settled(
        plan: &Plan,
        probe: &ProbePlan,
        src: &str,
        ast: &dorc_syntax::ast::Ast,
        interner: &Interner,
        diags: &[Diag],
        inputs: PlanningInputs<'_>,
        planned_image: Option<ApplyArtifactImageId>,
    ) -> Self {
        Self {
            planning_input: inputs.identity(),
            presented_plan: presented_plan_id(plan, probe, src, ast, interner, diags),
            planned_image,
        }
    }

    /// The identity of the complete input tuple the planner consumed.
    #[must_use]
    pub const fn planning_input(&self) -> PlanningInputId {
        self.planning_input
    }

    /// The identity of the approval surface itself.
    #[must_use]
    pub const fn presented_plan(&self) -> PresentedPlanId {
        self.presented_plan
    }

    /// The identity of the image the apply was planned to use, where one was built.
    #[must_use]
    pub const fn planned_image(&self) -> Option<ApplyArtifactImageId> {
        self.planned_image
    }
}
