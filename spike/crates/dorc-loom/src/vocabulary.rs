//! The closed frontmatter-key vocabulary a case may declare, and the gate that reads each key.
//!
//! One definition with three readers: the looms runner refuses a key outside this set, the e2e
//! runner refuses one outside its run-lane subset, and `dorc-loom keys` prints it so an author can
//! FIND the set without first provoking a refusal (`crates/cli/CLAUDE.md`
//! loom-form-is-the-same-battery). It lives here rather than in a runner because a `harness = false`
//! runner is not importable and the tool that mints cases has to be able to say what a case may say.

/// One legal frontmatter key and the gate that gives it effect.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FrontmatterKey {
    /// The key as a case spells it.
    pub name: &'static str,
    /// What reads it, in one clause — which IS the reason the vocabulary is closed: a key nobody
    /// reads is an assertion its author only believes they armed.
    pub read_by: &'static str,
    /// Whether a WHOLE-PRODUCT (`run:`-bearing) case may carry it. The e2e runner's own refusal
    /// derives its set from this flag rather than keeping a second list, because that list had to
    /// stay a subset of this one by hand and nothing checked it.
    pub run_lane: bool,
}

/// What a case declares it DEFINES — the one thing a reader cannot infer from the key list, and the
/// split a blind reviewer found explained nowhere.
pub const DEFINING_KEYS_NOTE: &str = "a case declares exactly ONE of `code:` or `arrangement:`. \
     `code:` defines a diagnostic code, whose message/help registers live in the catalog \
     (crates/aid/src/catalog_lock.rs); `arrangement:` defines a chrome slug -- a help page, a \
     summary line, a structure word -- whose words live in the arrangement registry \
     (arrangement_lock.rs). The two corpora partition the collection, and a case declaring neither \
     defines nothing.";

/// Every frontmatter key some gate reads, across BOTH runners and this tool.
///
/// `todo` is the one deliberate exception and is why it is safe unread: it asserts nothing about
/// the case, being an author's note about the case's own future.
pub const FRONTMATTER_KEYS: [FrontmatterKey; 22] = [
    FrontmatterKey {
        name: "code",
        read_by: "the diagnostic code this case defines; keys its catalog row",
        run_lane: false,
    },
    FrontmatterKey {
        name: "arrangement",
        read_by: "the chrome slug this case defines; keys its arrangement row",
        run_lane: false,
    },
    FrontmatterKey {
        name: "when-fires",
        read_by: "catalog metadata: when the code fires. Replacing it needs --accept-metadata",
        run_lane: false,
    },
    FrontmatterKey {
        name: "when-used",
        read_by: "arrangement metadata: when the chrome is used. Same acknowledgement",
        run_lane: false,
    },
    FrontmatterKey {
        name: "why",
        read_by: "metadata for either registry: why the entry reads as it does. Same acknowledgement",
        run_lane: false,
    },
    FrontmatterKey {
        name: "owns",
        read_by: "extra prose-components this case is the authoring home for; scanned corpus-wide, \
                  one home per component",
        run_lane: true,
    },
    FrontmatterKey {
        name: "todo",
        read_by: "nothing, deliberately: a note about the case's own future",
        run_lane: false,
    },
    FrontmatterKey {
        name: "run",
        read_by: "`round-trip` or `lint`: makes this a WHOLE-PRODUCT case the e2e runner executes",
        run_lane: true,
    },
    FrontmatterKey {
        name: "fixpoint",
        read_by: "`executed`: the transcript is proven by that e2e run, not by the looms render \
                  fixpoint. Refused without `run:`",
        run_lane: true,
    },
    FrontmatterKey {
        name: "flags",
        read_by: "e2e: DORC_FLAGS for the run-lane invocation",
        run_lane: true,
    },
    FrontmatterKey {
        name: "exit",
        read_by: "e2e: the plan (or lint) invocation's expected exit code",
        run_lane: true,
    },
    FrontmatterKey {
        name: "apply-exit",
        read_by: "e2e: the apply invocation's expected exit code",
        run_lane: true,
    },
    FrontmatterKey {
        name: "tolerate",
        read_by: "e2e: a named nondeterminism class, whose normalizer is applied to the RUN LOG at \
                  bless and at check. Never to rendered output",
        run_lane: true,
    },
    FrontmatterKey {
        name: "probe-results",
        read_by: "e2e: the probe results the run-lane invocation is fed",
        run_lane: true,
    },
    FrontmatterKey {
        name: "dual-rail",
        read_by: "e2e: also drive the machine-format rail",
        run_lane: true,
    },
    FrontmatterKey {
        name: "why-addr",
        read_by: "e2e: the `dorc why` address the case queries",
        run_lane: true,
    },
    FrontmatterKey {
        name: "expect-diagnostic",
        read_by: "e2e: code slugs that MUST fire. Validated against the catalog, so a dead slug is \
                  refused and a declaration is an assertion",
        run_lane: true,
    },
    FrontmatterKey {
        name: "expect-why",
        read_by: "e2e: free-text needles the why render must carry",
        run_lane: true,
    },
    FrontmatterKey {
        name: "expect-hint",
        read_by: "e2e: free-text needles a hint must carry",
        run_lane: true,
    },
    FrontmatterKey {
        name: "expect-why-chain",
        read_by: "e2e: free-text needles the numbered why-chain must carry",
        run_lane: true,
    },
    FrontmatterKey {
        name: "edit-loop",
        read_by: "the generated editing recipe; every primary-collection case is held to the \
                  current mint",
        run_lane: false,
    },
    FrontmatterKey {
        name: "envelope",
        read_by: "which report seat really prints this code -- `stderr`: the plan route's whole \
                  stderr envelope; `invocation`: the invocation-error seat, prefix and usage \
                  synopsis included",
        run_lane: false,
    },
];

/// The legal keys, spelled, in declaration order — what a refusal lists.
#[must_use]
pub fn frontmatter_key_names() -> Vec<&'static str> {
    FRONTMATTER_KEYS.iter().map(|key| key.name).collect()
}

/// Whether `key` is in the closed vocabulary.
#[must_use]
pub fn is_frontmatter_key(key: &str) -> bool {
    FRONTMATTER_KEYS.iter().any(|known| known.name == key)
}

/// The subset a WHOLE-PRODUCT case may carry — a projection, so it cannot drift out of the set the
/// looms runner (which sees run-lane cases too) will accept.
#[must_use]
pub fn run_lane_key_names() -> Vec<&'static str> {
    FRONTMATTER_KEYS
        .iter()
        .filter(|key| key.run_lane)
        .map(|key| key.name)
        .collect()
}

/// Whether a WHOLE-PRODUCT case may declare `key`.
#[must_use]
pub fn is_run_lane_key(key: &str) -> bool {
    FRONTMATTER_KEYS
        .iter()
        .any(|known| known.name == key && known.run_lane)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A duplicate would make one entry unreachable and its `read_by` a lie about which gate runs.
    #[test]
    fn the_vocabulary_has_no_duplicate_key() {
        let mut names = frontmatter_key_names();
        names.sort_unstable();
        let unique = names.len();
        names.dedup();
        assert_eq!(names.len(), unique);
    }

    /// The two defining keys are IN the vocabulary and the note explains both, so the reader who
    /// meets the "declares neither" refusal can act on the same sentence the listing prints.
    #[test]
    fn the_defining_split_is_stated_where_the_keys_are_listed() {
        assert!(is_frontmatter_key("code") && is_frontmatter_key("arrangement"));
        assert!(DEFINING_KEYS_NOTE.contains("catalog"));
        assert!(DEFINING_KEYS_NOTE.contains("arrangement registry"));
    }

    /// The run lane is a PROJECTION, never a second list: the looms runner sees whole-product cases
    /// too, so a run-lane key outside the full vocabulary would be accepted by one runner and
    /// refused by the other over the same file.
    #[test]
    fn the_run_lane_subset_is_inside_the_full_vocabulary() {
        let run_lane = run_lane_key_names();
        assert!(run_lane.iter().all(|key| is_frontmatter_key(key)));
        assert!(is_run_lane_key("probe-results"));
        assert!(!is_run_lane_key("code"), "a defining key is not run-lane");
        assert!(
            is_run_lane_key("owns"),
            "read by neither runner, but a whole-product case is a real authoring home"
        );
    }
}
