//! The provenance tier BOTH prose registries key on — who wrote a written register, as a type.

/// Who authored one written prose register.
///
/// The mint table is the whole point, and it is one-way: [`Migrated`](Self::Migrated) was minted
/// once by the migration and is never re-minted; [`Slop`](Self::Slop) is what the `dorc-loom`
/// compile/promote loop mints by default, whoever is driving; and
/// [`WrittenByHumanOnly`](Self::WrittenByHumanOnly) is minted ONLY under `dorc-loom promote
/// --human`, which refuses in an agent-marked environment. Generic over the text so the catalog's
/// single strings and the arrangement registry's word sequences share one enum, compiled-in or
/// owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProseTier<T> {
    /// Pre-pipeline builder text, frozen verbatim from the base tip and never re-minted.
    Migrated(T),
    /// Loom-authored without `--human` — AI-tier by definition, and the default mint.
    Slop(T),
    /// A human at the keyboard, through `dorc-loom promote --human`.
    WrittenByHumanOnly(T),
}

impl<T> ProseTier<T> {
    /// The wrapped text, tier-erased.
    #[must_use]
    pub fn text(&self) -> &T {
        match self {
            ProseTier::Migrated(text)
            | ProseTier::Slop(text)
            | ProseTier::WrittenByHumanOnly(text) => text,
        }
    }

    /// The same tier over transformed text — the owned-mirror conversions' one seat.
    pub fn map<U>(self, transform: impl FnOnce(T) -> U) -> ProseTier<U> {
        match self {
            ProseTier::Migrated(text) => ProseTier::Migrated(transform(text)),
            ProseTier::Slop(text) => ProseTier::Slop(transform(text)),
            ProseTier::WrittenByHumanOnly(text) => ProseTier::WrittenByHumanOnly(transform(text)),
        }
    }

    /// Whether the loom loop minted this — true for everything but [`Migrated`](Self::Migrated),
    /// which is exactly the set the case-ownership gates bind.
    #[must_use]
    pub fn is_loom_minted(&self) -> bool {
        !matches!(self, ProseTier::Migrated(_))
    }
}

impl ProseTier<&str> {
    /// The owned twin — the promote-time mirror's carry-forward conversion.
    #[must_use]
    pub fn to_owned_tier(self) -> ProseTier<String> {
        self.map(str::to_owned)
    }
}

impl<T: AsRef<str>> AsRef<str> for ProseTier<T> {
    fn as_ref(&self) -> &str {
        self.text().as_ref()
    }
}

/// Which tier a FRESH mint lands in — `dorc-loom`'s `--human` decision, carried without any text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mint {
    /// The unflagged default, whoever is driving.
    Slop,
    /// `dorc-loom promote --human`, outside an agent-marked environment.
    Human,
}

impl Mint {
    /// Wrap freshly-authored text in this mint's tier.
    pub fn tier<T>(self, text: T) -> ProseTier<T> {
        match self {
            Mint::Slop => ProseTier::Slop(text),
            Mint::Human => ProseTier::WrittenByHumanOnly(text),
        }
    }

    /// Whether minting over `previous` would re-mark a human's words as slop.
    #[must_use]
    pub fn demotes<T>(self, previous: Option<&ProseTier<T>>) -> bool {
        self == Mint::Slop && matches!(previous, Some(ProseTier::WrittenByHumanOnly(_)))
    }
}
