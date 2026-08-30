//! The `plan` species recorded model: one typed row per record kind, and the aggregate that
//! closes them over one another.
//!
//! The aggregate is where relational closure lives. A document whose declared counts, ordinal
//! sequences, and cross-row references do not agree parses under the grammar and refuses here,
//! because that is an agreement only a typed model can see.

use crate::format::{RefusalReason, SkeletonRecord};
use crate::grammar::RecordKind;
use crate::model::{PlanReceipt, Species};
use crate::reingested::RecordedInfluence;
use crate::rows::{
    self, LoadOrdinal, ModelRefusal, NarrativeOrdinal, RecordedAst, RecordedInvocation,
    RecordedLeaf, RecordedMember, RecordedOperands, RecordedProjectionOmission, RecordedRow,
    RecordedSite, RegionOrdinal, RelationFault, SourceOrdinal,
};
use crate::tokens::{
    ClosedToken, OpaqueState, RecordedAdmissionOutcome, RecordedDisposition,
    RecordedLicenseCustody, RecordedLicenseVerb, RecordedLoadOutcome, RecordedNarrativeKind,
    RecordedRenderKind, RecordedShipLane, RecordedSiteClass, RecordedSolvePass,
    RecordedSourceClass, RecordedSourceRole, RecordedSpeechAct, RecordedSurvivalOutcome,
    RenderSubjectAxis, bool_token,
};

/// One file the run acquired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSource {
    ordinal: SourceOrdinal,
    role: RecordedSourceRole,
    digest: String,
    bytes: u64,
    path: OpaqueState,
    excerpt: OpaqueState,
    class: RecordedSourceClass,
    content: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedSource {
    /// One source row.
    #[must_use]
    pub const fn of(
        ordinal: SourceOrdinal,
        role: RecordedSourceRole,
        digest: String,
        bytes: u64,
        path: OpaqueState,
        excerpt: OpaqueState,
        class: RecordedSourceClass,
        content: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            ordinal,
            role,
            class,
            content,
            digest,
            bytes,
            path,
            excerpt,
            account,
        }
    }

    /// Where this source sat in the acquired-source table.
    #[must_use]
    pub const fn ordinal(&self) -> SourceOrdinal {
        self.ordinal
    }

    /// What the source was to the run.
    #[must_use]
    pub const fn role(&self) -> RecordedSourceRole {
        self.role
    }

    /// The content digest, as spelled.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }

    /// How many bytes the source held.
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// What the projection holds in place of the path.
    #[must_use]
    pub const fn path(&self) -> OpaqueState {
        self.path
    }

    /// What the projection holds in place of the excerpt.
    #[must_use]
    pub const fn excerpt(&self) -> OpaqueState {
        self.excerpt
    }

    /// Which dialect the run accepted this source as.
    #[must_use]
    pub const fn class(&self) -> RecordedSourceClass {
        self.class
    }

    /// Whether this source's exact bytes are in the document, and if not, why not.
    #[must_use]
    pub const fn content(&self) -> OpaqueState {
        self.content
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedSource {
    const KIND: RecordKind = RecordKind::Source;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.ordinal.get().to_string(),
            self.role.token().to_owned(),
            self.digest.clone(),
            self.bytes.to_string(),
            self.path.token().to_owned(),
            self.excerpt.token().to_owned(),
            self.class.token().to_owned(),
            self.content.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            SourceOrdinal::of(rows::count(record, "ordinal")?),
            rows::closed(record, "role")?,
            rows::digest(record, "digest")?,
            rows::wide(record, "bytes")?,
            rows::closed(record, "path")?,
            rows::closed(record, "excerpt")?,
            rows::closed(record, "class")?,
            rows::closed(record, "content")?,
            rows::account(record),
        ))
    }
}

/// The closed intake outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedAdmission {
    outcome: RecordedAdmissionOutcome,
    records: u64,
    bytes: u64,
    stream: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedAdmission {
    /// One admission row.
    #[must_use]
    pub const fn of(
        outcome: RecordedAdmissionOutcome,
        records: u64,
        bytes: u64,
        stream: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            outcome,
            records,
            bytes,
            stream,
            account,
        }
    }

    /// What intake answered.
    #[must_use]
    pub const fn outcome(&self) -> RecordedAdmissionOutcome {
        self.outcome
    }

    /// How many records were admitted.
    #[must_use]
    pub const fn records(&self) -> u64 {
        self.records
    }

    /// How many bytes they accounted for.
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// What the projection holds in place of the admitted record stream.
    #[must_use]
    pub const fn stream(&self) -> OpaqueState {
        self.stream
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedAdmission {
    const KIND: RecordKind = RecordKind::Admission;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.outcome.token().to_owned(),
            self.records.to_string(),
            self.bytes.to_string(),
            self.stream.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            rows::closed(record, "outcome")?,
            rows::wide(record, "records")?,
            rows::wide(record, "bytes")?,
            rows::closed(record, "stream")?,
            rows::account(record),
        ))
    }
}

/// The identities of one complete approval surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedPresentedPlan {
    planning_input: String,
    presented_plan: String,
    planned_image: Option<String>,
    account: RecordedInfluence,
}

impl RecordedPresentedPlan {
    /// One presented-plan row.
    #[must_use]
    pub const fn of(
        planning_input: String,
        presented_plan: String,
        planned_image: Option<String>,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            planning_input,
            presented_plan,
            planned_image,
            account,
        }
    }

    /// The identity of the complete planner input tuple, as spelled.
    #[must_use]
    pub fn planning_input(&self) -> &str {
        &self.planning_input
    }

    /// The identity of the approval surface, as spelled.
    #[must_use]
    pub fn presented_plan(&self) -> &str {
        &self.presented_plan
    }

    /// The identity of the planned apply image, where the run had one.
    #[must_use]
    pub fn planned_image(&self) -> Option<&str> {
        self.planned_image.as_deref()
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedPresentedPlan {
    const KIND: RecordKind = RecordKind::PresentedPlan;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.planning_input.clone(),
            self.presented_plan.clone(),
            rows::spell_opt_digest(self.planned_image.as_deref()),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            rows::digest(record, "planning-input")?,
            rows::digest(record, "presented-plan")?,
            rows::opt_digest(record, "planned-image")?,
            rows::account(record),
        ))
    }
}

/// One site plan outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSiteDecision {
    site: RecordedSite,
    ast: RecordedAst,
    disposition: RecordedDisposition,
    shell: OpaqueState,
    locator: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedSiteDecision {
    /// One site-decision row.
    #[must_use]
    pub const fn of(
        site: RecordedSite,
        ast: RecordedAst,
        disposition: RecordedDisposition,
        shell: OpaqueState,
        locator: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            site,
            ast,
            disposition,
            shell,
            locator,
            account,
        }
    }

    /// Which site this decided.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which syntax node the site came from.
    #[must_use]
    pub const fn ast(&self) -> RecordedAst {
        self.ast
    }

    /// What the plan does with the site.
    #[must_use]
    pub const fn disposition(&self) -> RecordedDisposition {
        self.disposition
    }

    /// What the projection holds in place of the source text.
    #[must_use]
    pub const fn shell(&self) -> OpaqueState {
        self.shell
    }

    /// Whether this site's provenance DAG is in the document, and if not, why not.
    #[must_use]
    pub const fn locator(&self) -> OpaqueState {
        self.locator
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedSiteDecision {
    const KIND: RecordKind = RecordKind::SiteDecision;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.site.leaf_atom(),
            self.site.member_atom(),
            self.ast.get().to_string(),
            self.disposition.token().to_owned(),
            self.shell.token().to_owned(),
            self.locator.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            RecordedSite::of_record(record)?,
            RecordedAst::of(rows::count(record, "ast")?),
            rows::closed(record, "disposition")?,
            rows::closed(record, "shell")?,
            rows::closed(record, "locator")?,
            rows::account(record),
        ))
    }
}

/// One authored region shared outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRegionDecision {
    region: RegionOrdinal,
    ast: RecordedAst,
    disposition: RecordedDisposition,
    routes: u64,
    shell: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedRegionDecision {
    /// One region-decision row.
    #[must_use]
    pub const fn of(
        region: RegionOrdinal,
        ast: RecordedAst,
        disposition: RecordedDisposition,
        routes: u64,
        shell: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            region,
            ast,
            disposition,
            routes,
            shell,
            account,
        }
    }

    /// Which region this decided.
    #[must_use]
    pub const fn region(&self) -> RegionOrdinal {
        self.region
    }

    /// Which syntax node the region body came from.
    #[must_use]
    pub const fn ast(&self) -> RecordedAst {
        self.ast
    }

    /// What the plan does with every invocation of the region.
    #[must_use]
    pub const fn disposition(&self) -> RecordedDisposition {
        self.disposition
    }

    /// How many routes reach the region.
    #[must_use]
    pub const fn routes(&self) -> u64 {
        self.routes
    }

    /// What the projection holds in place of the source text.
    #[must_use]
    pub const fn shell(&self) -> OpaqueState {
        self.shell
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedRegionDecision {
    const KIND: RecordKind = RecordKind::RegionDecision;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.region.get().to_string(),
            self.ast.get().to_string(),
            self.disposition.token().to_owned(),
            self.routes.to_string(),
            self.shell.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            RegionOrdinal::of(rows::count(record, "region")?),
            RecordedAst::of(rows::count(record, "ast")?),
            rows::closed(record, "disposition")?,
            rows::wide(record, "routes")?,
            rows::closed(record, "shell")?,
            rows::account(record),
        ))
    }
}

/// One definition-plane outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedLoadDecision {
    ordinal: LoadOrdinal,
    outcome: RecordedLoadOutcome,
    name: OpaqueState,
    custody: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedLoadDecision {
    /// One load-decision row.
    #[must_use]
    pub const fn of(
        ordinal: LoadOrdinal,
        outcome: RecordedLoadOutcome,
        name: OpaqueState,
        custody: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            ordinal,
            outcome,
            name,
            custody,
            account,
        }
    }

    /// Where this decision sat in decision order.
    #[must_use]
    pub const fn ordinal(&self) -> LoadOrdinal {
        self.ordinal
    }

    /// What the definition plane decided.
    #[must_use]
    pub const fn outcome(&self) -> RecordedLoadOutcome {
        self.outcome
    }

    /// What the projection holds in place of the name.
    #[must_use]
    pub const fn name(&self) -> OpaqueState {
        self.name
    }

    /// What the projection holds in place of the custody.
    #[must_use]
    pub const fn custody(&self) -> OpaqueState {
        self.custody
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedLoadDecision {
    const KIND: RecordKind = RecordKind::LoadDecision;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.ordinal.get().to_string(),
            self.outcome.token().to_owned(),
            self.name.token().to_owned(),
            self.custody.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            LoadOrdinal::of(rows::count(record, "ordinal")?),
            rows::closed(record, "outcome")?,
            rows::closed(record, "name")?,
            rows::closed(record, "custody")?,
            rows::account(record),
        ))
    }
}

/// One site analysis classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSiteClassification {
    site: RecordedSite,
    ast: RecordedAst,
    class: RecordedSiteClass,
    verdict_lane: bool,
    invalidator: bool,
    cells: RecordedOperands,
    account: RecordedInfluence,
}

impl RecordedSiteClassification {
    /// One site-classification row.
    #[must_use]
    pub const fn of(
        site: RecordedSite,
        ast: RecordedAst,
        class: RecordedSiteClass,
        verdict_lane: bool,
        invalidator: bool,
        cells: RecordedOperands,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            site,
            ast,
            class,
            verdict_lane,
            invalidator,
            cells,
            account,
        }
    }

    /// Which site this classified.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which syntax node the site came from.
    #[must_use]
    pub const fn ast(&self) -> RecordedAst {
        self.ast
    }

    /// The classification.
    #[must_use]
    pub const fn class(&self) -> RecordedSiteClass {
        self.class
    }

    /// Whether the site is on the verdict lane.
    #[must_use]
    pub const fn verdict_lane(&self) -> bool {
        self.verdict_lane
    }

    /// Whether the site invalidates anything.
    #[must_use]
    pub const fn invalidator(&self) -> bool {
        self.invalidator
    }

    /// The capped account of cells the classification keys on.
    #[must_use]
    pub const fn cells(&self) -> RecordedOperands {
        self.cells
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedSiteClassification {
    const KIND: RecordKind = RecordKind::SiteClassification;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.site.leaf_atom(),
            self.site.member_atom(),
            self.ast.get().to_string(),
            self.class.token().to_owned(),
            bool_token(self.verdict_lane).to_owned(),
            bool_token(self.invalidator).to_owned(),
            self.cells.shown().to_string(),
            self.cells.dropped().to_string(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            RecordedSite::of_record(record)?,
            RecordedAst::of(rows::count(record, "ast")?),
            rows::closed(record, "class")?,
            rows::flag(record, "verdict-lane")?,
            rows::flag(record, "invalidator")?,
            RecordedOperands::of_record(record)?,
            rows::account(record),
        ))
    }
}

/// One dataflow certification answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSolveCertification {
    pass: RecordedSolvePass,
    consistent: bool,
    tripped: bool,
    account: RecordedInfluence,
}

impl RecordedSolveCertification {
    /// One solve-certification row.
    #[must_use]
    pub const fn of(
        pass: RecordedSolvePass,
        consistent: bool,
        tripped: bool,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            pass,
            consistent,
            tripped,
            account,
        }
    }

    /// Which dataflow answer this certified.
    #[must_use]
    pub const fn pass(&self) -> RecordedSolvePass {
        self.pass
    }

    /// Whether the certifier agreed with the solver.
    #[must_use]
    pub const fn consistent(&self) -> bool {
        self.consistent
    }

    /// Whether the latch tripped.
    #[must_use]
    pub const fn tripped(&self) -> bool {
        self.tripped
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedSolveCertification {
    const KIND: RecordKind = RecordKind::SolveCertification;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.pass.token().to_owned(),
            bool_token(self.consistent).to_owned(),
            bool_token(self.tripped).to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            rows::closed(record, "pass")?,
            rows::flag(record, "consistent")?,
            rows::flag(record, "tripped")?,
            rows::account(record),
        ))
    }
}

/// Which body one probe site shipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedProbeShip {
    site: RecordedSite,
    lane: RecordedShipLane,
    source: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedProbeShip {
    /// One probe-ship row.
    #[must_use]
    pub const fn of(
        site: RecordedSite,
        lane: RecordedShipLane,
        source: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            site,
            lane,
            source,
            account,
        }
    }

    /// Which site shipped.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which body it shipped.
    #[must_use]
    pub const fn lane(&self) -> RecordedShipLane {
        self.lane
    }

    /// What the projection holds in place of the defining source.
    #[must_use]
    pub const fn source(&self) -> OpaqueState {
        self.source
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedProbeShip {
    const KIND: RecordKind = RecordKind::ProbeShip;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.site.leaf_atom(),
            self.site.member_atom(),
            self.lane.token().to_owned(),
            self.source.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            RecordedSite::of_record(record)?,
            rows::closed(record, "lane")?,
            rows::closed(record, "source")?,
            rows::account(record),
        ))
    }
}

/// One survival-tier outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSurvival {
    site: RecordedSite,
    outcome: RecordedSurvivalOutcome,
    wall: Option<RecordedLeaf>,
    aggregate: Option<u32>,
    poison: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedSurvival {
    /// One survival row.
    #[must_use]
    pub const fn of(
        site: RecordedSite,
        outcome: RecordedSurvivalOutcome,
        wall: Option<RecordedLeaf>,
        aggregate: Option<u32>,
        poison: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            site,
            outcome,
            wall,
            aggregate,
            poison,
            account,
        }
    }

    /// Which site the walk decided.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// What it decided.
    #[must_use]
    pub const fn outcome(&self) -> RecordedSurvivalOutcome {
        self.outcome
    }

    /// The leaf of the wall that stood, where one did.
    #[must_use]
    pub const fn wall(&self) -> Option<RecordedLeaf> {
        self.wall
    }

    /// How many establishes an aggregate carried, where the outcome names one.
    #[must_use]
    pub const fn aggregate(&self) -> Option<u32> {
        self.aggregate
    }

    /// What the projection holds in place of the poisoning kind.
    #[must_use]
    pub const fn poison(&self) -> OpaqueState {
        self.poison
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedSurvival {
    const KIND: RecordKind = RecordKind::Survival;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.site.leaf_atom(),
            self.site.member_atom(),
            self.outcome.token().to_owned(),
            rows::spell_opt_count(self.wall.map(RecordedLeaf::get)),
            rows::spell_opt_count(self.aggregate),
            self.poison.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            RecordedSite::of_record(record)?,
            rows::closed(record, "outcome")?,
            rows::opt_count(record, "wall")?.map(RecordedLeaf::of),
            rows::opt_count(record, "aggregate")?,
            rows::closed(record, "poison")?,
            rows::account(record),
        ))
    }
}

/// Which identity a render row is keyed by.
///
/// One value rather than two independent slots: the axis is a function of the kind, so a
/// region-keyed row carrying a member, or a leaf-keyed row carrying none, is unrepresentable
/// rather than merely refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderSubject {
    /// The row is keyed by an execution.
    Leaf(RecordedSite),
    /// The row is keyed by an authored region.
    Region(RegionOrdinal),
    /// The row owns neither axis.
    None,
}

impl RenderSubject {
    /// Which axis this subject occupies.
    #[must_use]
    pub const fn axis(self) -> RenderSubjectAxis {
        match self {
            Self::Leaf(_) => RenderSubjectAxis::Leaf,
            Self::Region(_) => RenderSubjectAxis::Region,
            Self::None => RenderSubjectAxis::None,
        }
    }
}

/// One render-time decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRenderDecision {
    subject: RenderSubject,
    kind: RecordedRenderKind,
    detail: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedRenderDecision {
    /// One render-decision row.
    ///
    /// # Errors
    /// Refuses a subject whose axis the kind does not own.
    pub fn of(
        subject: RenderSubject,
        kind: RecordedRenderKind,
        detail: OpaqueState,
        account: RecordedInfluence,
    ) -> Result<Self, RelationFault> {
        if subject.axis() != kind.subject_axis() {
            return Err(RelationFault::SubjectAxisDisagrees {
                expected: kind.subject_axis().token(),
                supplied: subject.axis().token(),
                kind: Self::KIND.token(),
            });
        }
        Ok(Self {
            subject,
            kind,
            detail,
            account,
        })
    }

    /// Which identity the row is keyed by.
    #[must_use]
    pub const fn subject(&self) -> RenderSubject {
        self.subject
    }

    /// Which decision the row records.
    #[must_use]
    pub const fn kind(&self) -> RecordedRenderKind {
        self.kind
    }

    /// What the projection holds in place of the detail.
    #[must_use]
    pub const fn detail(&self) -> OpaqueState {
        self.detail
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedRenderDecision {
    const KIND: RecordKind = RecordKind::RenderDecision;

    fn atoms(&self) -> Vec<String> {
        let (subject, member) = match self.subject {
            RenderSubject::Leaf(site) => (site.leaf_atom(), site.member_atom()),
            RenderSubject::Region(ordinal) => {
                (ordinal.get().to_string(), rows::spell_opt_count(None))
            }
            RenderSubject::None => (rows::spell_opt_count(None), rows::spell_opt_count(None)),
        };
        vec![
            subject,
            member,
            self.kind.token().to_owned(),
            self.detail.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        let kind: RecordedRenderKind = rows::closed(record, "kind")?;
        let raw_subject = rows::opt_count(record, "subject")?;
        let raw_member = rows::opt_count(record, "member")?;
        // What the two slots look like, rather than a guess at what the writer meant: a reader
        // can only report the shape it was handed, and naming it exactly is what lets a negative
        // test pin which departure it caught.
        let supplied = match (raw_subject.is_some(), raw_member.is_some()) {
            (true, true) => "leaf",
            (true, false) => "subject-without-member",
            (false, true) => "member-without-subject",
            (false, false) => "none",
        };
        let axis_fault = || {
            ModelRefusal::Relation(RelationFault::SubjectAxisDisagrees {
                expected: kind.subject_axis().token(),
                supplied,
                kind: Self::KIND.token(),
            })
        };
        let subject = match kind.subject_axis() {
            RenderSubjectAxis::Leaf => {
                let leaf = raw_subject.ok_or_else(axis_fault)?;
                RenderSubject::Leaf(RecordedSite::of(
                    RecordedLeaf::of(leaf),
                    raw_member.map(RecordedMember::of),
                ))
            }
            RenderSubjectAxis::Region => {
                if raw_member.is_some() {
                    return Err(axis_fault());
                }
                RenderSubject::Region(RegionOrdinal::of(raw_subject.ok_or_else(axis_fault)?))
            }
            RenderSubjectAxis::None => {
                if raw_subject.is_some() || raw_member.is_some() {
                    return Err(axis_fault());
                }
                RenderSubject::None
            }
        };
        Self::of(
            subject,
            kind,
            rows::closed(record, "detail")?,
            rows::account(record),
        )
        .map_err(ModelRefusal::Relation)
    }
}

/// One decision-inert narrative.
///
/// The row carries the collapse class and its operand account and nothing that identifies a
/// site: the narrative operands are not durable, so a reader learns that N collapses of a class
/// occurred and never which line each was about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedNarrative {
    ordinal: NarrativeOrdinal,
    speech: RecordedSpeechAct,
    kind: RecordedNarrativeKind,
    operands: RecordedOperands,
    account: RecordedInfluence,
}

impl RecordedNarrative {
    /// One narrative row.
    #[must_use]
    pub const fn of(
        ordinal: NarrativeOrdinal,
        speech: RecordedSpeechAct,
        kind: RecordedNarrativeKind,
        operands: RecordedOperands,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            ordinal,
            speech,
            kind,
            operands,
            account,
        }
    }

    /// Where this narrative sat in mint order.
    #[must_use]
    pub const fn ordinal(&self) -> NarrativeOrdinal {
        self.ordinal
    }

    /// The speech act.
    #[must_use]
    pub const fn speech(&self) -> RecordedSpeechAct {
        self.speech
    }

    /// The collapse class.
    #[must_use]
    pub const fn kind(&self) -> RecordedNarrativeKind {
        self.kind
    }

    /// The capped operand account.
    #[must_use]
    pub const fn operands(&self) -> RecordedOperands {
        self.operands
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedNarrative {
    const KIND: RecordKind = RecordKind::Narrative;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.ordinal.get().to_string(),
            self.speech.token().to_owned(),
            self.kind.token().to_owned(),
            self.operands.shown().to_string(),
            self.operands.dropped().to_string(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            NarrativeOrdinal::of(rows::count(record, "ordinal")?),
            rows::closed(record, "speech")?,
            rows::closed(record, "kind")?,
            RecordedOperands::of_record(record)?,
            rows::account(record),
        ))
    }
}

/// What licensed one irreversible verb.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedLicensor {
    site: RecordedSite,
    license: RecordedLicenseVerb,
    custody: RecordedLicenseCustody,
    locus: OpaqueState,
    account: RecordedInfluence,
}

impl RecordedLicensor {
    /// One licensor row.
    #[must_use]
    pub const fn of(
        site: RecordedSite,
        license: RecordedLicenseVerb,
        custody: RecordedLicenseCustody,
        locus: OpaqueState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            site,
            license,
            custody,
            locus,
            account,
        }
    }

    /// Which site the verb applied to.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which verb was licensed.
    #[must_use]
    pub const fn license(&self) -> RecordedLicenseVerb {
        self.license
    }

    /// Whose utterance it rests on.
    #[must_use]
    pub const fn custody(&self) -> RecordedLicenseCustody {
        self.custody
    }

    /// What the projection holds in place of the authoring locus.
    #[must_use]
    pub const fn locus(&self) -> OpaqueState {
        self.locus
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedLicensor {
    const KIND: RecordKind = RecordKind::Licensor;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.site.leaf_atom(),
            self.site.member_atom(),
            self.license.token().to_owned(),
            self.custody.token().to_owned(),
            self.locus.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            RecordedSite::of_record(record)?,
            rows::closed(record, "license")?,
            rows::closed(record, "custody")?,
            rows::closed(record, "locus")?,
            rows::account(record),
        ))
    }
}

/// One plan document, as a typed model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedPlanReceipt {
    invocation: RecordedInvocation,
    sources: Vec<RecordedSource>,
    admission: Option<RecordedAdmission>,
    presented: Option<RecordedPresentedPlan>,
    sites: Vec<RecordedSiteDecision>,
    regions: Vec<RecordedRegionDecision>,
    loads: Vec<RecordedLoadDecision>,
    classifications: Vec<RecordedSiteClassification>,
    certifications: Vec<RecordedSolveCertification>,
    ships: Vec<RecordedProbeShip>,
    survivals: Vec<RecordedSurvival>,
    renders: Vec<RecordedRenderDecision>,
    narratives: Vec<RecordedNarrative>,
    licensors: Vec<RecordedLicensor>,
    omissions: Vec<RecordedProjectionOmission>,
}

impl RecordedPlanReceipt {
    /// Build the model from one document record stream, closing the records over one another.
    ///
    /// # Errors
    /// Refuses an unreadable row, a missing or repeated singleton, a non-contiguous ordinal
    /// sequence, or a render row naming a region the document does not declare.
    pub fn of_records(records: &[SkeletonRecord]) -> Result<Self, ModelRefusal> {
        let model = Self {
            invocation: rows::singleton_of(records)?,
            sources: rows::rows_of(records)?,
            admission: rows::optional_of(records)?,
            presented: rows::optional_of(records)?,
            sites: rows::rows_of(records)?,
            regions: rows::rows_of(records)?,
            loads: rows::rows_of(records)?,
            classifications: rows::rows_of(records)?,
            certifications: rows::rows_of(records)?,
            ships: rows::rows_of(records)?,
            survivals: rows::rows_of(records)?,
            renders: rows::rows_of(records)?,
            narratives: rows::rows_of(records)?,
            licensors: rows::rows_of(records)?,
            omissions: rows::rows_of(records)?,
        };
        model.close()?;
        Ok(model)
    }

    fn close(&self) -> Result<(), RelationFault> {
        rows::contiguous(
            RecordKind::Source,
            self.sources.iter().map(|row| row.ordinal().get()),
        )?;
        rows::contiguous(
            RecordKind::LoadDecision,
            self.loads.iter().map(|row| row.ordinal().get()),
        )?;
        rows::contiguous(
            RecordKind::Narrative,
            self.narratives.iter().map(|row| row.ordinal().get()),
        )?;
        rows::contiguous(
            RecordKind::RegionDecision,
            self.regions.iter().map(|row| row.region().get()),
        )?;
        let declared_regions = u32::try_from(self.regions.len()).unwrap_or(u32::MAX);
        for render in &self.renders {
            if let RenderSubject::Region(ordinal) = render.subject()
                && ordinal.get() >= declared_regions
            {
                return Err(RelationFault::DanglingRegion {
                    region: ordinal.get(),
                });
            }
        }
        Ok(())
    }

    /// Serialize the model, in the species kind order.
    ///
    /// One canonical emission order, so two documents carrying the same content cannot differ
    /// in bytes.
    ///
    /// # Errors
    /// Refuses whatever the grammar table refuses.
    pub fn to_records(&self) -> Result<Vec<SkeletonRecord>, RefusalReason> {
        let mut out = Vec::new();
        for kind in PlanReceipt::KINDS {
            match *kind {
                RecordKind::Invocation => out.push(self.invocation.to_record()?),
                RecordKind::Source => push_all(&mut out, &self.sources)?,
                RecordKind::Admission => {
                    if let Some(row) = &self.admission {
                        out.push(row.to_record()?);
                    }
                }
                RecordKind::PresentedPlan => {
                    if let Some(row) = &self.presented {
                        out.push(row.to_record()?);
                    }
                }
                RecordKind::SiteDecision => push_all(&mut out, &self.sites)?,
                RecordKind::RegionDecision => push_all(&mut out, &self.regions)?,
                RecordKind::LoadDecision => push_all(&mut out, &self.loads)?,
                RecordKind::SiteClassification => push_all(&mut out, &self.classifications)?,
                RecordKind::SolveCertification => push_all(&mut out, &self.certifications)?,
                RecordKind::ProbeShip => push_all(&mut out, &self.ships)?,
                RecordKind::Survival => push_all(&mut out, &self.survivals)?,
                RecordKind::RenderDecision => push_all(&mut out, &self.renders)?,
                RecordKind::Narrative => push_all(&mut out, &self.narratives)?,
                RecordKind::Licensor => push_all(&mut out, &self.licensors)?,
                RecordKind::ProjectionOmission => push_all(&mut out, &self.omissions)?,
                _ => {}
            }
        }
        Ok(out)
    }

    /// The producing invocation.
    #[must_use]
    pub const fn invocation(&self) -> &RecordedInvocation {
        &self.invocation
    }

    /// Every acquired source.
    #[must_use]
    pub fn sources(&self) -> &[RecordedSource] {
        &self.sources
    }

    /// The intake outcome, where the run recorded one.
    #[must_use]
    pub const fn admission(&self) -> Option<&RecordedAdmission> {
        self.admission.as_ref()
    }

    /// The approval-surface identities, where the run recorded them.
    #[must_use]
    pub const fn presented(&self) -> Option<&RecordedPresentedPlan> {
        self.presented.as_ref()
    }

    /// Every site decision.
    #[must_use]
    pub fn sites(&self) -> &[RecordedSiteDecision] {
        &self.sites
    }

    /// Every region decision.
    #[must_use]
    pub fn regions(&self) -> &[RecordedRegionDecision] {
        &self.regions
    }

    /// Every definition-plane decision.
    #[must_use]
    pub fn loads(&self) -> &[RecordedLoadDecision] {
        &self.loads
    }

    /// Every site classification.
    #[must_use]
    pub fn classifications(&self) -> &[RecordedSiteClassification] {
        &self.classifications
    }

    /// Every dataflow certification.
    #[must_use]
    pub fn certifications(&self) -> &[RecordedSolveCertification] {
        &self.certifications
    }

    /// Every probe shipment.
    #[must_use]
    pub fn ships(&self) -> &[RecordedProbeShip] {
        &self.ships
    }

    /// Every survival outcome.
    #[must_use]
    pub fn survivals(&self) -> &[RecordedSurvival] {
        &self.survivals
    }

    /// Every render decision.
    #[must_use]
    pub fn renders(&self) -> &[RecordedRenderDecision] {
        &self.renders
    }

    /// Every narrative.
    #[must_use]
    pub fn narratives(&self) -> &[RecordedNarrative] {
        &self.narratives
    }

    /// Every licensor.
    #[must_use]
    pub fn licensors(&self) -> &[RecordedLicensor] {
        &self.licensors
    }

    /// Every population the projection declined to carry.
    #[must_use]
    pub fn omissions(&self) -> &[RecordedProjectionOmission] {
        &self.omissions
    }
}

fn push_all<R: RecordedRow>(
    out: &mut Vec<SkeletonRecord>,
    rows: &[R],
) -> Result<(), RefusalReason> {
    for row in rows {
        out.push(row.to_record()?);
    }
    Ok(())
}
