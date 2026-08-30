//! The read-back seal.
//!
//! Everything recovered from a published document wears [`Reingested`]. The wrapper has a
//! private field and a private mint, implements no dereference or borrow, and offers no
//! generic map, unwrap, or accessor of any kind: there is deliberately no way to ask a
//! `Reingested<T>` for its `T`. Decomposition is per-species and answers either another
//! [`Reingested`] or a report-only scalar, so a value that came back from a document cannot be
//! handed to anything that takes a live one.
//!
//! The seal is structural rather than a rule about which types join [`RecordedType`]: even if
//! a live type were admitted to that set tomorrow, no accessor exists that would hand it out.

use crate::apply::RecordedApplyIntent;
use crate::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId, PresentedPlanId};
use crate::model::{ApplyIntent, ApplyOutcome, PlanReceipt, Projection, Species};
use crate::outcome::RecordedApplyOutcome;
use crate::plan::{RecordedPlanReceipt, RecordedSiteDecision, RecordedSource};
use crate::reader::Receipt;
use crate::rows::{ModelRefusal, RecordedProjectionOmission};
use crate::tokens::{
    ClosedToken, RecordedApplyPolicy, RecordedDisposition, RecordedInvocationMode,
    RecordedOmissionReason, RecordedOriginState, RecordedSourceRole, RecordedSpineSpecies,
    RecordedTerminalState,
};

mod sealed {
    pub trait RecordedType {}
    pub trait ReDerived {}
}

/// The closed set of things a document can yield back.
pub trait RecordedType: sealed::RecordedType {}

/// The closed set of things the CURRENT arm of a comparison may hold.
///
/// Sealed for the same reason [`RecordedType`] is, against the opposite mistake. Comparing a
/// recorded conclusion with one derived today is a report act, so the value standing beside the
/// recorded one has to be a report value too. Left open, the arm accepts a live decision — and a
/// live decision carries the licence for the irreversible verb inside itself, so a report would
/// then be holding one and could hand it on.
pub trait ReDerived: sealed::ReDerived {}

/// A plan outcome derived under CURRENT inputs, for comparison against a recorded one.
///
/// The four verbs are the same words a document spells, and that is exactly why this is its own
/// type rather than a reuse of either neighbour. It is not a
/// [`RecordedDisposition`](crate::tokens::RecordedDisposition) — that came off a document, this
/// did not — and it is not a live decision, which carries a licence. There is no conversion from
/// either: the four constructors are the only way to make one, so a seat producing one has to
/// name the verb it derived, and no bulk `From` can turn a document's answer into a
/// freshly-derived one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReDerivedDisposition(RecordedDisposition);

impl sealed::ReDerived for ReDerivedDisposition {}
impl ReDerived for ReDerivedDisposition {}

impl ReDerivedDisposition {
    /// The authored bytes execute.
    #[must_use]
    pub const fn run() -> Self {
        Self(RecordedDisposition::Run)
    }

    /// The authored bytes are replaced by a value-preserving stand-in.
    #[must_use]
    pub const fn replace() -> Self {
        Self(RecordedDisposition::Replace)
    }

    /// The site lies in a branch proved dead.
    #[must_use]
    pub const fn omit() -> Self {
        Self(RecordedDisposition::Omit)
    }

    /// A check is inserted ahead of the authored bytes, which survive verbatim.
    #[must_use]
    pub const fn guard() -> Self {
        Self(RecordedDisposition::Guard)
    }

    /// The word a report renders.
    #[must_use]
    pub fn token(self) -> &'static str {
        self.0.token()
    }

    /// Whether a recorded answer and this one name the same verb.
    ///
    /// A comparison, never a coercion: it answers a question about two values and yields neither.
    #[must_use]
    pub fn agrees_with(self, recorded: RecordedDisposition) -> bool {
        self.0 == recorded
    }
}

impl<D: Species, P: Projection> sealed::RecordedType for Receipt<D, P> {}
impl<D: Species, P: Projection> RecordedType for Receipt<D, P> {}

impl sealed::RecordedType for RecordedPlanReceipt {}
impl RecordedType for RecordedPlanReceipt {}
impl sealed::RecordedType for RecordedApplyIntent {}
impl RecordedType for RecordedApplyIntent {}
impl sealed::RecordedType for RecordedApplyOutcome {}
impl RecordedType for RecordedApplyOutcome {}
impl sealed::RecordedType for RecordedSource {}
impl RecordedType for RecordedSource {}
impl sealed::RecordedType for RecordedSiteDecision {}
impl RecordedType for RecordedSiteDecision {}
impl sealed::RecordedType for RecordedProjectionOmission {}
impl RecordedType for RecordedProjectionOmission {}

/// A value recovered from a published document.
#[derive(Debug)]
pub struct Reingested<T: RecordedType>(T);

impl<T: RecordedType> Reingested<T> {
    /// Seal a value the reader produced. Crate-private: the only way a `Reingested` comes into
    /// being is by reading a document.
    pub(crate) const fn seal(value: T) -> Self {
        Self(value)
    }
}

/// Two sealed values compare without either being unwrapped.
///
/// Comparison rather than extraction is what lets graph correlation tell one document read twice
/// from two documents sharing an identity, with no accessor handing the inner value out.
impl<T: RecordedType + PartialEq> PartialEq for Reingested<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: RecordedType + Eq> Eq for Reingested<T> {}

/// Cloning a sealed value answers another sealed value. Nothing is extracted, so a report may
/// hold a second handle on one recorded document without the seal being weakened.
impl<T: RecordedType + Clone> Clone for Reingested<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D: Species, P: Projection> Reingested<Receipt<D, P>> {
    /// How many records the document carries.
    #[must_use]
    pub fn record_count(&self) -> usize {
        self.0.skeleton().records.len()
    }

    /// The document's own identity, as spelled.
    #[must_use]
    pub fn receipt_id_hex(&self) -> String {
        self.0.skeleton().receipt_id.clone()
    }

    /// The signing provider's identity, as spelled.
    #[must_use]
    pub fn signing_key_id_hex(&self) -> String {
        self.0.skeleton().signing_key_id.clone()
    }
}

impl<P: Projection> Reingested<Receipt<PlanReceipt, P>> {
    /// The typed model of this document.
    ///
    /// # Errors
    /// Refuses a record stream that parsed but does not close over itself.
    pub fn model(&self) -> Result<Reingested<RecordedPlanReceipt>, ModelRefusal> {
        RecordedPlanReceipt::of_records(&self.0.skeleton().records).map(Reingested::seal)
    }

    /// This document's own identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<PlanReceiptId> {
        PlanReceiptId::of_hex(&self.0.skeleton().receipt_id)
    }
}

impl<P: Projection> Reingested<Receipt<ApplyIntent, P>> {
    /// The typed model of this document.
    ///
    /// # Errors
    /// Refuses a record stream that parsed but does not close over itself.
    pub fn model(&self) -> Result<Reingested<RecordedApplyIntent>, ModelRefusal> {
        RecordedApplyIntent::of_records(&self.0.skeleton().records).map(Reingested::seal)
    }

    /// This document's own identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<ApplyIntentId> {
        ApplyIntentId::of_hex(&self.0.skeleton().receipt_id)
    }
}

impl<P: Projection> Reingested<Receipt<ApplyOutcome, P>> {
    /// The typed model of this document.
    ///
    /// # Errors
    /// Refuses a record stream that parsed but does not close over itself.
    pub fn model(&self) -> Result<Reingested<RecordedApplyOutcome>, ModelRefusal> {
        RecordedApplyOutcome::of_records(&self.0.skeleton().records).map(Reingested::seal)
    }

    /// This document's own identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<ApplyOutcomeId> {
        ApplyOutcomeId::of_hex(&self.0.skeleton().receipt_id)
    }
}

impl Reingested<RecordedPlanReceipt> {
    /// Which invocation shape produced the document.
    #[must_use]
    pub const fn mode(&self) -> RecordedInvocationMode {
        self.0.invocation().mode()
    }

    /// Where the invocation stood relative to host contact.
    #[must_use]
    pub const fn invocation_account(&self) -> RecordedInfluence {
        self.0.invocation().account()
    }

    /// Every acquired source, each still sealed.
    #[must_use]
    pub fn sources(&self) -> Vec<Reingested<RecordedSource>> {
        self.0
            .sources()
            .iter()
            .cloned()
            .map(Reingested::seal)
            .collect()
    }

    /// Every population the projection declined to carry, each still sealed.
    #[must_use]
    pub fn omissions(&self) -> Vec<Reingested<RecordedProjectionOmission>> {
        self.0
            .omissions()
            .iter()
            .cloned()
            .map(Reingested::seal)
            .collect()
    }

    /// How many site decisions the document carries.
    #[must_use]
    pub fn site_count(&self) -> usize {
        self.0.sites().len()
    }

    /// Every site decision, each still sealed.
    #[must_use]
    pub fn sites(&self) -> Vec<Reingested<RecordedSiteDecision>> {
        self.0
            .sites()
            .iter()
            .cloned()
            .map(Reingested::seal)
            .collect()
    }

    /// How many region decisions the document carries.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.0.regions().len()
    }

    /// The approval-surface identity this plan presented, where it recorded one.
    #[must_use]
    pub fn presented_plan(&self) -> Option<PresentedPlanId> {
        PresentedPlanId::of_hex(self.0.presented()?.presented_plan())
    }
}

/// One detail a validated region carried, sealed under the sink question its slot poses.
///
/// The ONE public exit for a reingested document's plaintext. It carries a
/// [`RecordedValue`](crate::report::RecordedValue), which has no accessor and a redacted `Debug`,
/// so the bytes leave only through a destination encoder — and the class rides along because the
/// encoder needs to know which question it is answering.
#[derive(Debug)]
pub struct RecordedDetail {
    record: u64,
    tag: crate::projection::OpaqueFieldTag,
    value: crate::report::RecordedValue,
}

impl RecordedDetail {
    /// Which record this enriches.
    #[must_use]
    pub const fn record(&self) -> u64 {
        self.record
    }

    /// Which slot it fills.
    #[must_use]
    pub const fn tag(&self) -> crate::projection::OpaqueFieldTag {
        self.tag
    }

    /// The sealed value.
    #[must_use]
    pub const fn value(&self) -> &crate::report::RecordedValue {
        &self.value
    }
}

impl<D: Species> Reingested<Receipt<D, crate::model::Rich>> {
    /// Every detail the validated region carried, in canonical order, each sealed under its own
    /// class.
    ///
    /// THE exit. It replaced a public `detail()` answering `&[u8]`, which was the easier of two
    /// routes out and the one a listing adapter had already taken — so the class-aware exit was
    /// sound only for a caller that chose it. There is no second route now.
    #[must_use]
    pub fn recorded_details(&self) -> Vec<RecordedDetail> {
        self.0
            .region()
            .slots()
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|(record, tag)| self.recorded_detail(record, tag))
            .collect()
    }

    /// One detail, where the region carried that slot.
    #[must_use]
    pub fn recorded_detail(
        &self,
        record: u64,
        tag: crate::projection::OpaqueFieldTag,
    ) -> Option<RecordedDetail> {
        self.0.detail(record, tag).map(|bytes| RecordedDetail {
            record,
            tag,
            value: crate::report::RecordedValue::sealed(
                crate::report::ValueClass::of_tag(tag),
                bytes.to_vec(),
            ),
        })
    }

    /// The bytes filling one slot of one record, as the validated region carried them.
    ///
    /// Crate-private: the report decomposition seals these under a class immediately, and a
    /// public reader answering raw bytes is exactly the exit that made the sealed model optional.
    pub(crate) fn detail(
        &self,
        record: u64,
        tag: crate::projection::OpaqueFieldTag,
    ) -> Option<&[u8]> {
        self.0.detail(record, tag)
    }

    /// The KIND of every record, in the document's own order.
    ///
    /// Kinds and nothing else: no atoms, no payload. It exists so a consumer can find the record
    /// ORDINAL a detail entry is keyed by, by walking the same stream those entries were keyed
    /// against. The alternative — deriving an ordinal by counting which record species the
    /// projection emits first — makes every consumer a second copy of the projection's ordering,
    /// and a detail entry keyed by position enriches whichever row shares its integer when those
    /// two copies disagree.
    #[must_use]
    pub fn record_kinds(&self) -> Vec<crate::grammar::RecordKind> {
        self.0
            .skeleton()
            .records
            .iter()
            .map(crate::format::SkeletonRecord::kind)
            .collect()
    }
}

impl Reingested<RecordedSource> {
    /// Where this source sat in the acquired-source table.
    #[must_use]
    pub fn ordinal(&self) -> u32 {
        self.0.ordinal().get()
    }

    /// What the source was to the run.
    #[must_use]
    pub const fn role(&self) -> RecordedSourceRole {
        self.0.role()
    }

    /// The content digest, as spelled.
    #[must_use]
    pub fn digest(&self) -> String {
        self.0.digest().to_owned()
    }

    /// How many bytes the source held.
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.0.bytes()
    }

    /// Which dialect the run accepted this source as.
    #[must_use]
    pub const fn class(&self) -> crate::tokens::RecordedSourceClass {
        self.0.class()
    }

    /// Whether the source's exact bytes are in the document, and if not, why not.
    #[must_use]
    pub const fn content(&self) -> crate::tokens::OpaqueState {
        self.0.content()
    }

    /// Whether the source's path is in the document.
    #[must_use]
    pub const fn path(&self) -> crate::tokens::OpaqueState {
        self.0.path()
    }
}

impl Reingested<RecordedSiteDecision> {
    /// Which site this decided, as the document identifies it.
    #[must_use]
    pub const fn site(&self) -> crate::rows::RecordedSite {
        self.0.site()
    }

    /// Which syntax node the site came from.
    #[must_use]
    pub const fn ast(&self) -> crate::rows::RecordedAst {
        self.0.ast()
    }

    /// What the document says the plan did with the site.
    #[must_use]
    pub const fn disposition(&self) -> RecordedDisposition {
        self.0.disposition()
    }

    /// Whether the site's own shell text is in the document.
    #[must_use]
    pub const fn shell(&self) -> crate::tokens::OpaqueState {
        self.0.shell()
    }

    /// Whether the site's provenance DAG is in the document.
    #[must_use]
    pub const fn locator(&self) -> crate::tokens::OpaqueState {
        self.0.locator()
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.0.account()
    }
}

impl Reingested<RecordedProjectionOmission> {
    /// Which in-memory decision species went uncarried.
    #[must_use]
    pub const fn species(&self) -> RecordedSpineSpecies {
        self.0.species()
    }

    /// How many of it there were.
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.0.count()
    }

    /// Why the projection did not carry it.
    #[must_use]
    pub const fn reason(&self) -> RecordedOmissionReason {
        self.0.reason()
    }
}

impl Reingested<RecordedApplyIntent> {
    /// Which publication route authorized the apply.
    #[must_use]
    pub const fn policy(&self) -> RecordedApplyPolicy {
        self.0.intent().policy()
    }

    /// Whether any assignment names an originating plan.
    #[must_use]
    pub const fn origin_state(&self) -> RecordedOriginState {
        self.0.intent().origin_state()
    }

    /// How many assignments the intent carries.
    #[must_use]
    pub fn assignment_count(&self) -> usize {
        self.0.assignments().len()
    }

    /// Every originating plan document identity this intent names, with duplicates retained.
    #[must_use]
    pub fn origin_receipts(&self) -> Vec<PlanReceiptId> {
        let mut out = Vec::new();
        for target in self.0.assignments() {
            if let crate::apply::OriginatingPlans::Known(origins) = target.origins() {
                for origin in origins.get() {
                    if let Some(id) = PlanReceiptId::of_hex(origin.receipt()) {
                        out.push(id);
                    }
                }
            }
        }
        out
    }
}

impl Reingested<RecordedApplyOutcome> {
    /// The graceful terminal state the apply reached.
    #[must_use]
    pub const fn terminal(&self) -> RecordedTerminalState {
        self.0.outcome().terminal()
    }

    /// How many site rows the outcome carries.
    #[must_use]
    pub fn site_count(&self) -> usize {
        self.0.sites().len()
    }

    /// The intent this outcome answers.
    #[must_use]
    pub fn intent(&self) -> Option<ApplyIntentId> {
        ApplyIntentId::of_hex(self.0.outcome().intent())
    }
}

/// How a recorded conclusion and one derived under current inputs relate.
///
/// Four states, and they never substitute for one another. Disagreement is a finding that
/// keeps both values, never a resolution that picks one.
#[derive(Debug)]
pub enum RecordedCurrent<R: RecordedType, C: ReDerived> {
    /// Only the document has it.
    RecordedOnly(Reingested<R>),
    /// Only the current derivation has it.
    CurrentOnly(C),
    /// Both have it and they agree.
    BothAgreeing {
        /// What the document recorded.
        recorded: Reingested<R>,
        /// What the current derivation produced.
        current: C,
    },
    /// Both have it and they disagree.
    BothDisagreeing {
        /// What the document recorded.
        recorded: Reingested<R>,
        /// What the current derivation produced.
        current: C,
    },
}

impl<R: RecordedType, C: ReDerived> RecordedCurrent<R, C> {
    /// The word a report renders for this comparison.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::RecordedOnly(_) => "recorded-only",
            Self::CurrentOnly(_) => "current-only",
            Self::BothAgreeing { .. } => "both-agreeing",
            Self::BothDisagreeing { .. } => "both-disagreeing",
        }
    }

    /// Whether this comparison is a finding a report must surface.
    #[must_use]
    pub const fn is_finding(&self) -> bool {
        matches!(self, Self::BothDisagreeing { .. })
    }
}

impl RecordedCurrent<RecordedSiteDecision, ReDerivedDisposition> {
    /// Classify one site's recorded conclusion against the one derived today.
    ///
    /// The arm is decided HERE, from the two values, rather than chosen by a caller. A caller
    /// that picks `BothAgreeing` picks it whether or not the two agree, which makes the one arm
    /// a reader would most trust the one arm nothing checks — and there is no repair downstream,
    /// because both values survive into a report that has already been told they match.
    ///
    /// Absent on both sides is no comparison at all, not a vacuous agreement.
    #[must_use]
    pub fn of_site(
        recorded: Option<Reingested<RecordedSiteDecision>>,
        current: Option<ReDerivedDisposition>,
    ) -> Option<Self> {
        match (recorded, current) {
            (Some(recorded), Some(current)) => {
                Some(if current.agrees_with(recorded.disposition()) {
                    Self::BothAgreeing { recorded, current }
                } else {
                    Self::BothDisagreeing { recorded, current }
                })
            }
            (Some(recorded), None) => Some(Self::RecordedOnly(recorded)),
            (None, Some(current)) => Some(Self::CurrentOnly(current)),
            (None, None) => None,
        }
    }
}

/// An influence grade as a document carries it.
///
/// Flattened on the way in and never rehydrated: there is no accessor yielding a live account,
/// no conversion, and no join. Absent, unknown, malformed, or unverifiable material reads
/// [`RecordedInfluence::MostInfluenced`], which is the conservative direction — losing this
/// metadata can only make a reader more careful.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedInfluence {
    /// The document recorded that the object was computed before host contact.
    AuthoredBeforeContact,
    /// The document recorded that host-reported material reached the object.
    HostInfluenced,
    /// The document recorded nothing usable, or recorded that the account was untracked.
    MostInfluenced,
}

impl RecordedInfluence {
    /// Read one grade from a document's closed vocabulary.
    #[must_use]
    pub fn of_token(token: Option<&str>) -> Self {
        match token {
            Some("authored-before-contact") => Self::AuthoredBeforeContact,
            Some("host-influenced") => Self::HostInfluenced,
            _ => Self::MostInfluenced,
        }
    }

    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::AuthoredBeforeContact => "authored-before-contact",
            Self::HostInfluenced => "host-influenced",
            Self::MostInfluenced => "untracked",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Plain;

    #[test]
    fn an_unreadable_grade_reads_at_the_most_influenced_point() {
        // Losing this metadata must only ever make a reader more careful, so every way of
        // failing to read one lands at the same conservative point.
        assert_eq!(
            RecordedInfluence::of_token(None),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("")),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("untracked")),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("not-a-grade")),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("AUTHORED-BEFORE-CONTACT")),
            RecordedInfluence::MostInfluenced,
            "case is exact"
        );
    }

    #[test]
    fn a_recorded_grade_reads_back_as_itself() {
        for grade in [
            RecordedInfluence::AuthoredBeforeContact,
            RecordedInfluence::HostInfluenced,
            RecordedInfluence::MostInfluenced,
        ] {
            assert_eq!(RecordedInfluence::of_token(Some(grade.token())), grade);
        }
    }

    #[test]
    fn only_disagreement_is_a_finding() {
        let current: RecordedCurrent<Receipt<PlanReceipt, Plain>, ReDerivedDisposition> =
            RecordedCurrent::CurrentOnly(ReDerivedDisposition::run());
        assert!(!current.is_finding());
        assert_eq!(current.token(), "current-only");
    }

    fn site(disposition: RecordedDisposition) -> Reingested<RecordedSiteDecision> {
        use crate::rows::{RecordedAst, RecordedLeaf, RecordedSite};
        Reingested::seal(RecordedSiteDecision::of(
            RecordedSite::of(RecordedLeaf::of(0), None),
            RecordedAst::of(0),
            disposition,
            crate::tokens::OpaqueState::Uncollected,
            crate::tokens::OpaqueState::Uncollected,
            RecordedInfluence::MostInfluenced,
        ))
    }

    #[test]
    fn the_four_states_are_decided_from_the_values_never_chosen() {
        // The whole point of classifying here: a caller cannot label a disagreement as agreement.
        let agreeing = RecordedCurrent::of_site(
            Some(site(RecordedDisposition::Guard)),
            Some(ReDerivedDisposition::guard()),
        )
        .expect("two values compare");
        assert_eq!(agreeing.token(), "both-agreeing");
        assert!(!agreeing.is_finding());

        let disagreeing = RecordedCurrent::of_site(
            Some(site(RecordedDisposition::Replace)),
            Some(ReDerivedDisposition::run()),
        )
        .expect("two values compare");
        assert_eq!(disagreeing.token(), "both-disagreeing");
        assert!(disagreeing.is_finding());

        assert_eq!(
            RecordedCurrent::of_site(Some(site(RecordedDisposition::Omit)), None)
                .expect("one value still compares")
                .token(),
            "recorded-only"
        );
        assert_eq!(
            RecordedCurrent::of_site(None, Some(ReDerivedDisposition::omit()))
                .expect("one value still compares")
                .token(),
            "current-only"
        );
        assert!(
            RecordedCurrent::of_site(None, None).is_none(),
            "neither side holding anything is no comparison, not a vacuous agreement"
        );
    }

    #[test]
    fn a_disagreement_keeps_both_values() {
        // Disagreement is a finding that PRESERVES both, never a selection of either — so a
        // report can say what the document recorded AND what today derives, and name the gap.
        let comparison = RecordedCurrent::of_site(
            Some(site(RecordedDisposition::Replace)),
            Some(ReDerivedDisposition::run()),
        )
        .expect("two values compare");
        let RecordedCurrent::BothDisagreeing { recorded, current } = comparison else {
            panic!("a Replace beside a run is a disagreement")
        };
        assert_eq!(recorded.disposition(), RecordedDisposition::Replace);
        assert_eq!(current.token(), "run");
    }

    #[test]
    fn a_rederived_verb_reads_as_itself_and_agrees_only_with_its_own() {
        for (current, recorded) in [
            (ReDerivedDisposition::run(), RecordedDisposition::Run),
            (
                ReDerivedDisposition::replace(),
                RecordedDisposition::Replace,
            ),
            (ReDerivedDisposition::omit(), RecordedDisposition::Omit),
            (ReDerivedDisposition::guard(), RecordedDisposition::Guard),
        ] {
            assert_eq!(current.token(), recorded.token());
            assert!(current.agrees_with(recorded));
        }
        assert!(!ReDerivedDisposition::run().agrees_with(RecordedDisposition::Guard));
    }
}
