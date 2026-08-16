//! `core::contested` — the cross-unit shadow refusal's LICENSE-PLANE fact (`28K` §1
//! `rul-silent-shadowing-refuses`).
//!
//! When one unit's definition overrides a role family a DIFFERENT unit defined, with no
//! intervening `unset -f` of that name, the family's licenses switch off: it goes UNDESCRIBED for
//! the rest of the run. The withdrawal is applied to the lifted sets ONCE, at the driver edge, so
//! no downstream consumer has to remember to ask (the `erasure-is-applied-once-never-consulted`
//! shape: a withdrawn family is indistinguishable from one nobody ever wrote an oracle for, which
//! is exactly `silence-licenses-nothing`'s floor).
//!
//! # Direction of travel is one-way, by construction
//!
//! There is no `un_contest`, no `remove`, no `retain`, and no `&mut` accessor: a set is built once
//! from the environment's own answer and can only be READ afterwards. That asymmetry is the whole
//! soundness argument for shipping the refusal ahead of the decidable-condition fold
//! (`28O:res-polyfill-binding-tops-pending-fold` option (ii)) — an under-firing refusal grants
//! nothing, so the only way this can hurt is if a family could be quietly un-withheld.
//!
//! The DIAGNOSTIC derives FROM this fact and never the reverse (`two-plane-aid-law`): licensure
//! reads the set, narration reads the set, and nothing reads a diagnostic to decide.

use std::collections::BTreeSet;

/// The role families whose licenses are withheld for this run.
///
/// Keyed by the MUNGED family base name — the `<base>` of `<base>__<role>`, which is exactly what
/// `to_funcname_segment` produces from a provider or kind (`apt-get` and `apt_get` are one family;
/// `sm.dorc.Package` and `sm_dorc_Package` are one family). Comparing munged names rather than
/// authored ones is what makes the withdrawal cover every member of the family regardless of how
/// each file spelled it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContestedFamilies {
    families: BTreeSet<String>,
}

impl ContestedFamilies {
    /// The empty set: no family contested, every license available. The shape every driver that
    /// does not model loading (the DST harnesses, the coverage dashboard) passes.
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    /// Build the set from the munged family names the environment proved contested. Write-once:
    /// the returned value has no mutator.
    #[must_use]
    pub fn new(families: impl IntoIterator<Item = String>) -> Self {
        Self {
            families: families.into_iter().collect(),
        }
    }

    /// Are `family`'s licenses withheld? `family` is the munged base name.
    #[must_use]
    pub fn withholds(&self, family: &str) -> bool {
        self.families.contains(family)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.families.is_empty()
    }

    /// The contested family names, in deterministic order (`inv-determinism`) — for disclosure.
    pub fn families(&self) -> impl Iterator<Item = &str> + '_ {
        self.families.iter().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::ContestedFamilies;

    /// The empty set withholds nothing — the `empty-world-byte-identical` precondition: a run with
    /// no contest must be indistinguishable from one before this mechanism existed.
    #[test]
    fn an_empty_set_withholds_nothing() {
        let none = ContestedFamilies::none();
        assert!(none.is_empty());
        assert!(!none.withholds("apt_get"));
    }

    /// Withholding is exact-match on the MUNGED base: a family withheld is withheld for every role
    /// member (the caller munges once and asks once), and a neighbouring family is untouched.
    ///
    /// **This mechanism is load-bearing for the definition-factoring lane's byte-identity gate, and
    /// is not dead code** (`307:fnd-corpus-carries-twelve-plural-families`). Twelve committed cases
    /// declare one role name in two of their own files; seven never load the second file, and the
    /// remaining FIVE (`contest28-*`, `guard23-reingest-collision-verbatim`) really do load both and
    /// are held byte-stable by exactly this withdrawal. Retiring it would not merely lose a
    /// diagnostic — it would let five plural families reach the resolution seats, where the
    /// single-definition coincidence that `syn-single-frame-byte-identical` rests on no longer
    /// holds.
    #[test]
    fn withholding_is_keyed_by_the_munged_family_base() {
        let set = ContestedFamilies::new(["apt_get".to_owned()]);
        assert!(set.withholds("apt_get"));
        assert!(!set.withholds("apt_get_extra"));
        assert!(!set.withholds("dpkg"));
        assert_eq!(set.families().collect::<Vec<_>>(), ["apt_get"]);
    }
}
