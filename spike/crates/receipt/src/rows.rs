//! The shared record-row machinery: the row trait, the typed field newtypes, the reader
//! helpers, and the two rows every species carries.
//!
//! A row model exists so a projection cannot transpose two atoms of the same scalar type.
//! [`crate::format::SkeletonRecord::build`] checks each atom against the grammar table, which
//! catches a wrong TYPE but never a swapped PAIR — `leaf` and `ast` are both counts. Building
//! a row from named typed fields, and reading it back BY KEY, is what closes that: a
//! transposition in [`RecordedRow::atoms`] fails the round trip rather than shipping.
//!
//! Relational closure — a declared count agreeing with the rows present, an ordinal naming a
//! row that exists — lives here rather than in the grammar layer, which stays a pure
//! one-exact-form byte check.

use crate::format::{RefusalReason, SkeletonRecord};
use crate::grammar::{self, RecordKind};
use crate::reingested::RecordedInfluence;
use crate::tokens::{
    ClosedToken, OpaqueState, RecordedMode, RecordedOmissionReason, RecordedSpineSpecies,
    bool_of_token,
};

/// Why a document parsed under the grammar but could not become a model.
///
/// A second family beside [`RefusalReason`], which stays the grammar layer answer. This one
/// names what a typed model could see and a byte check could not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelRefusal {
    /// An atom the row model could not read as its declared type.
    Atom {
        /// Which kind.
        kind: &'static str,
        /// Which key.
        key: &'static str,
    },
    /// A record of the wrong kind reached a row model.
    Kind {
        /// The kind the model wanted.
        expected: &'static str,
        /// The kind it was handed.
        found: &'static str,
    },
    /// The records parsed and did not close over one another.
    Relation(RelationFault),
}

/// How a set of records failed to close over itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationFault {
    /// A kind that must appear exactly once did not appear.
    MissingSingleton {
        /// Which kind.
        kind: &'static str,
    },
    /// A kind that must appear exactly once appeared more than once.
    DuplicateSingleton {
        /// Which kind.
        kind: &'static str,
    },
    /// An ordinal sequence was not contiguous from zero.
    OrdinalNotContiguous {
        /// Which kind.
        kind: &'static str,
        /// The ordinal the sequence wanted next.
        expected: u32,
        /// The ordinal it found.
        found: u32,
    },
    /// A declared count and the rows present disagree.
    CountDisagrees {
        /// Which kind declared it.
        kind: &'static str,
        /// What the document declared.
        declared: u64,
        /// What the document carries.
        present: u64,
    },
    /// A row named an assignment ordinal no assignment row declares.
    DanglingAssignment {
        /// The ordinal named.
        assignment: u32,
    },
    /// A render row named a region ordinal no region row declares.
    DanglingRegion {
        /// The ordinal named.
        region: u32,
    },
    /// A render row populated an identity slot its kind does not own.
    SubjectAxisDisagrees {
        /// The axis the kind owns.
        expected: &'static str,
        /// The axis the row supplied.
        supplied: &'static str,
        /// The kind whose axis was contradicted.
        kind: &'static str,
    },
    /// The document level origin state and the assignments disagree.
    OriginStateDisagrees {
        /// What the intent row declared.
        declared: &'static str,
        /// How many assignments name an originating plan.
        with_origins: u32,
    },
}

impl From<RelationFault> for ModelRefusal {
    fn from(fault: RelationFault) -> Self {
        Self::Relation(fault)
    }
}

/// One typed record row.
pub trait RecordedRow: Sized {
    /// The kind this row spells.
    const KIND: RecordKind;

    /// Every atom, in the grammar table order.
    fn atoms(&self) -> Vec<String>;

    /// Read this row from a record, by key.
    ///
    /// # Errors
    /// Refuses a record of another kind, or an atom the model cannot read.
    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal>;

    /// Serialize this row, checking every atom against the grammar table.
    ///
    /// # Errors
    /// Refuses whatever the table refuses.
    fn to_record(&self) -> Result<SkeletonRecord, RefusalReason> {
        SkeletonRecord::build(Self::KIND, self.atoms())
    }
}

/// Check a record is the kind a row model wants.
///
/// # Errors
/// Refuses a record of another kind.
pub fn expect_kind(record: &SkeletonRecord, want: RecordKind) -> Result<(), ModelRefusal> {
    if record.kind() == want {
        Ok(())
    } else {
        Err(ModelRefusal::Kind {
            expected: want.token(),
            found: record.kind().token(),
        })
    }
}

fn raw<'a>(record: &'a SkeletonRecord, key: &'static str) -> Result<&'a str, ModelRefusal> {
    record.atom(key).ok_or(ModelRefusal::Atom {
        kind: record.kind().token(),
        key,
    })
}

fn refuse(record: &SkeletonRecord, key: &'static str) -> ModelRefusal {
    ModelRefusal::Atom {
        kind: record.kind().token(),
        key,
    }
}

/// Read a canonical count.
///
/// # Errors
/// Refuses an absent key or a non-canonical integer.
pub fn count(record: &SkeletonRecord, key: &'static str) -> Result<u32, ModelRefusal> {
    grammar::canonical_u32(raw(record, key)?).ok_or_else(|| refuse(record, key))
}

/// Read a canonical count that may be absent.
///
/// # Errors
/// Refuses an absent key or a non-canonical integer.
pub fn opt_count(record: &SkeletonRecord, key: &'static str) -> Result<Option<u32>, ModelRefusal> {
    let atom = raw(record, key)?;
    if atom == grammar::ABSENT {
        return Ok(None);
    }
    grammar::canonical_u32(atom)
        .map(Some)
        .ok_or_else(|| refuse(record, key))
}

/// Read a canonical wide integer.
///
/// # Errors
/// Refuses an absent key or a non-canonical integer.
pub fn wide(record: &SkeletonRecord, key: &'static str) -> Result<u64, ModelRefusal> {
    grammar::canonical_u64(raw(record, key)?).ok_or_else(|| refuse(record, key))
}

/// Read a canonical wide integer that may be absent.
///
/// # Errors
/// Refuses an absent key or a non-canonical integer.
pub fn opt_wide(record: &SkeletonRecord, key: &'static str) -> Result<Option<u64>, ModelRefusal> {
    let atom = raw(record, key)?;
    if atom == grammar::ABSENT {
        return Ok(None);
    }
    grammar::canonical_u64(atom)
        .map(Some)
        .ok_or_else(|| refuse(record, key))
}

/// Read an exact digest spelling.
///
/// # Errors
/// Refuses an absent key or a spelling that is not exactly sixty-four lowercase hex.
pub fn digest(record: &SkeletonRecord, key: &'static str) -> Result<String, ModelRefusal> {
    let atom = raw(record, key)?;
    if grammar::is_digest(atom) {
        Ok(atom.to_owned())
    } else {
        Err(refuse(record, key))
    }
}

/// Read a digest spelling that may be absent.
///
/// # Errors
/// Refuses an absent key or a malformed spelling.
pub fn opt_digest(
    record: &SkeletonRecord,
    key: &'static str,
) -> Result<Option<String>, ModelRefusal> {
    let atom = raw(record, key)?;
    if atom == grammar::ABSENT {
        return Ok(None);
    }
    if grammar::is_digest(atom) {
        Ok(Some(atom.to_owned()))
    } else {
        Err(refuse(record, key))
    }
}

/// Read one token of a closed vocabulary.
///
/// # Errors
/// Refuses an absent key or a token outside the vocabulary.
pub fn closed<T: ClosedToken>(
    record: &SkeletonRecord,
    key: &'static str,
) -> Result<T, ModelRefusal> {
    T::of_token(raw(record, key)?).ok_or_else(|| refuse(record, key))
}

/// Read a boolean.
///
/// # Errors
/// Refuses an absent key or a spelling outside the boolean vocabulary.
pub fn flag(record: &SkeletonRecord, key: &'static str) -> Result<bool, ModelRefusal> {
    bool_of_token(raw(record, key)?).ok_or_else(|| refuse(record, key))
}

/// Read an influence grade. Total: anything unreadable lands at the conservative point.
#[must_use]
pub fn account(record: &SkeletonRecord) -> RecordedInfluence {
    RecordedInfluence::of_token(record.atom("account"))
}

/// Spell an optional count.
#[must_use]
pub fn spell_opt_count(value: Option<u32>) -> String {
    value.map_or_else(|| grammar::ABSENT.to_owned(), |n| n.to_string())
}

/// Spell an optional wide integer.
#[must_use]
pub fn spell_opt_wide(value: Option<u64>) -> String {
    value.map_or_else(|| grammar::ABSENT.to_owned(), |n| n.to_string())
}

/// Spell an optional digest.
#[must_use]
pub fn spell_opt_digest(value: Option<&str>) -> String {
    value.map_or_else(|| grammar::ABSENT.to_owned(), ToOwned::to_owned)
}

/// One executable leaf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedLeaf(u32);

/// One member index within an in-loop fact family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedMember(u32);

/// One node in the parsed syntax arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedAst(u32);

/// One authored region, as this document numbers them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionOrdinal(u32);

/// One acquired source, in load order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceOrdinal(u32);

/// One definition-plane decision, in decision order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoadOrdinal(u32);

/// One narrative, in mint order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NarrativeOrdinal(u32);

/// One target assignment within an apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssignmentOrdinal(u32);

/// One originating plan within an assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OriginOrdinal(u32);

/// One executed site within an apply outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SiteOutcomeOrdinal(u32);

impl RecordedLeaf {
    /// One leaf, by its position in source order.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The position.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl RecordedMember {
    /// One member index.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The index.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl RecordedAst {
    /// One syntax node.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The arena index.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl RegionOrdinal {
    /// One region, as this document numbers them.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl SourceOrdinal {
    /// One source, in load order.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl LoadOrdinal {
    /// One definition-plane decision, in decision order.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl NarrativeOrdinal {
    /// One narrative, in mint order.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl AssignmentOrdinal {
    /// One target assignment.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl OriginOrdinal {
    /// One originating plan within an assignment.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl SiteOutcomeOrdinal {
    /// One executed site.
    #[must_use]
    pub const fn of(value: u32) -> Self {
        Self(value)
    }
    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// The site identity two same-command sites must not collapse across.
///
/// One value rather than two fields, so the pair cannot be split or reordered by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedSite {
    leaf: RecordedLeaf,
    member: Option<RecordedMember>,
}

impl RecordedSite {
    /// One site, with an optional in-loop member index.
    #[must_use]
    pub const fn of(leaf: RecordedLeaf, member: Option<RecordedMember>) -> Self {
        Self { leaf, member }
    }

    /// The leaf.
    #[must_use]
    pub const fn leaf(self) -> RecordedLeaf {
        self.leaf
    }

    /// The member index, where the site has one.
    #[must_use]
    pub const fn member(self) -> Option<RecordedMember> {
        self.member
    }

    /// Read a site from the `leaf` and `member` keys.
    ///
    /// # Errors
    /// Refuses either atom the model cannot read.
    pub fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        Ok(Self::of(
            RecordedLeaf::of(count(record, "leaf")?),
            opt_count(record, "member")?.map(RecordedMember::of),
        ))
    }

    /// Spell the `leaf` atom.
    #[must_use]
    pub fn leaf_atom(self) -> String {
        self.leaf.get().to_string()
    }

    /// Spell the `member` atom.
    #[must_use]
    pub fn member_atom(self) -> String {
        spell_opt_count(self.member.map(RecordedMember::get))
    }
}

/// A capped operand account: how many a record shows, and how many it dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordedOperands {
    shown: u32,
    dropped: u32,
}

impl RecordedOperands {
    /// One account.
    #[must_use]
    pub const fn of(shown: u32, dropped: u32) -> Self {
        Self { shown, dropped }
    }

    /// How many operands the record shows.
    #[must_use]
    pub const fn shown(self) -> u32 {
        self.shown
    }

    /// How many the cap dropped.
    #[must_use]
    pub const fn dropped(self) -> u32 {
        self.dropped
    }

    /// Read an account from the `operands` and `dropped` keys.
    ///
    /// # Errors
    /// Refuses either atom the model cannot read.
    pub fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        Ok(Self::of(
            count(record, "operands")?,
            count(record, "dropped")?,
        ))
    }
}

/// The producing invocation and its controller-minted attempt identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedInvocation {
    mode: RecordedMode,
    started: Option<u64>,
    argv: OpaqueState,
    target: OpaqueState,
    attempt: u32,
    account: RecordedInfluence,
}

impl RecordedInvocation {
    /// One invocation row.
    #[must_use]
    pub const fn of(
        mode: RecordedMode,
        started: Option<u64>,
        argv: OpaqueState,
        target: OpaqueState,
        attempt: u32,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            mode,
            started,
            argv,
            target,
            attempt,
            account,
        }
    }

    /// Which invocation shape produced the document.
    #[must_use]
    pub const fn mode(&self) -> RecordedMode {
        self.mode
    }

    /// When the invocation started, where a clock was available.
    #[must_use]
    pub const fn started(&self) -> Option<u64> {
        self.started
    }

    /// What the projection holds in place of the argv.
    #[must_use]
    pub const fn argv(&self) -> OpaqueState {
        self.argv
    }

    /// What the projection holds in place of the target.
    #[must_use]
    pub const fn target(&self) -> OpaqueState {
        self.target
    }

    /// Which controller-minted attempt this was.
    #[must_use]
    pub const fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedInvocation {
    const KIND: RecordKind = RecordKind::Invocation;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.mode.token().to_owned(),
            spell_opt_wide(self.started),
            self.argv.token().to_owned(),
            self.target.token().to_owned(),
            self.attempt.to_string(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            closed(record, "mode")?,
            opt_wide(record, "started")?,
            closed(record, "argv")?,
            closed(record, "target")?,
            count(record, "attempt")?,
            account(record),
        ))
    }
}

/// A population the projection did not carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedProjectionOmission {
    species: RecordedSpineSpecies,
    count: u32,
    reason: RecordedOmissionReason,
    account: RecordedInfluence,
}

impl RecordedProjectionOmission {
    /// One omission row.
    #[must_use]
    pub const fn of(
        species: RecordedSpineSpecies,
        count: u32,
        reason: RecordedOmissionReason,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            species,
            count,
            reason,
            account,
        }
    }

    /// Which in-memory decision species went uncarried.
    #[must_use]
    pub const fn species(&self) -> RecordedSpineSpecies {
        self.species
    }

    /// How many of it there were.
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.count
    }

    /// Why the projection did not carry it.
    #[must_use]
    pub const fn reason(&self) -> RecordedOmissionReason {
        self.reason
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedProjectionOmission {
    const KIND: RecordKind = RecordKind::ProjectionOmission;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.species.token().to_owned(),
            self.count.to_string(),
            self.reason.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            closed(record, "species")?,
            count(record, "count")?,
            closed(record, "reason")?,
            account(record),
        ))
    }
}

/// Collect every row of one kind out of a record stream, in document order.
///
/// # Errors
/// Refuses whatever the row model refuses.
pub fn rows_of<R: RecordedRow>(records: &[SkeletonRecord]) -> Result<Vec<R>, ModelRefusal> {
    records
        .iter()
        .filter(|record| record.kind() == R::KIND)
        .map(R::of_record)
        .collect()
}

/// Take the one row of a kind that must appear exactly once.
///
/// # Errors
/// Refuses an absent or repeated row.
pub fn singleton_of<R: RecordedRow>(records: &[SkeletonRecord]) -> Result<R, ModelRefusal> {
    let mut found = rows_of::<R>(records)?;
    match found.len() {
        1 => found
            .pop()
            .ok_or(ModelRefusal::Relation(RelationFault::MissingSingleton {
                kind: R::KIND.token(),
            })),
        0 => Err(RelationFault::MissingSingleton {
            kind: R::KIND.token(),
        }
        .into()),
        _ => Err(RelationFault::DuplicateSingleton {
            kind: R::KIND.token(),
        }
        .into()),
    }
}

/// Take the at-most-one row of a kind.
///
/// # Errors
/// Refuses a repeated row.
pub fn optional_of<R: RecordedRow>(records: &[SkeletonRecord]) -> Result<Option<R>, ModelRefusal> {
    let mut found = rows_of::<R>(records)?;
    if found.len() > 1 {
        return Err(RelationFault::DuplicateSingleton {
            kind: R::KIND.token(),
        }
        .into());
    }
    Ok(found.pop())
}

/// Check an ordinal sequence runs contiguously from zero.
///
/// # Errors
/// Refuses the first ordinal out of place.
pub fn contiguous(
    kind: RecordKind,
    ordinals: impl IntoIterator<Item = u32>,
) -> Result<(), RelationFault> {
    for (position, found) in ordinals.into_iter().enumerate() {
        let expected = u32::try_from(position).unwrap_or(u32::MAX);
        if found != expected {
            return Err(RelationFault::OrdinalNotContiguous {
                kind: kind.token(),
                expected,
                found,
            });
        }
    }
    Ok(())
}

/// Check a declared count against the rows present.
///
/// # Errors
/// Refuses a disagreement.
pub fn declared_count(
    kind: RecordKind,
    declared: u64,
    present: usize,
) -> Result<(), RelationFault> {
    let present = u64::try_from(present).unwrap_or(u64::MAX);
    if declared == present {
        Ok(())
    } else {
        Err(RelationFault::CountDisagrees {
            kind: kind.token(),
            declared,
            present,
        })
    }
}
