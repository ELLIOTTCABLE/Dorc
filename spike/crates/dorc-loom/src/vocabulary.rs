//! The closed frontmatter-key vocabulary a case may declare, and the gate that reads each key.
//!
//! One definition with two readers: the corpus runner refuses a key outside this set, and
//! `dorc-loom keys` prints it so an author can FIND the set without first provoking a refusal
//! (`crates/cli/CLAUDE.md` loom-form-is-the-same-battery). It lives here rather than in the runner
//! because a `harness = false` runner is not importable and the tool that mints cases has to be able
//! to say what a case may say.
//!
//! Frontmatter survives ONLY for what is ABOUT THE CASE AS AN AUTHORING HOME
//! (`30X:loom-frontmatter-is-registry-metadata-only`; `30Xa:rul-survivors-are-the-criterion-not-the-count`):
//! everything that was a run knob or an assertion is spelled in the session instead (flags on the
//! `$ dorc` line, the plan exit as a `$ echo $?` block, the artifact set as `--artifact-dir`, the
//! diagnostics as the transcript), leaving the thirteen below. Two of those thirteen are case-level
//! declarations about the RUN no session line can honestly carry (`30Xa:rul-survivors-are-the-criterion-not-the-count`):
//! `apply-exit` (the exec rail's own expected exit) and `xfail` (a registry-keyed defect pin).

/// One legal frontmatter key and the gate that gives it effect.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FrontmatterKey {
    /// The key as a case spells it.
    pub name: &'static str,
    /// What reads it, in one clause — which IS the reason the vocabulary is closed: a key nobody
    /// reads is an assertion its author only believes they armed.
    pub read_by: &'static str,
}

/// What a case declares it DEFINES — the one thing a reader cannot infer from the key list, and the
/// split a blind reviewer found explained nowhere.
pub const DEFINING_KEYS_NOTE: &str = "a case declares exactly ONE of `code:` or `arrangement:`. \
     `code:` defines a diagnostic code, whose message/help registers live in the catalog \
     (crates/aid/src/catalog_lock.rs); `arrangement:` defines a chrome slug -- a help page, a \
     summary line, a structure word -- whose words live in the arrangement registry \
     (arrangement_lock.rs). The two corpora partition the collection, and a case declaring neither \
     defines nothing.";

/// Every frontmatter key some gate reads, across the corpus runner and this tool.
///
/// `todo` is the one deliberate exception and is why it is safe unread: it asserts nothing about
/// the case, being an author's note about the case's own future.
pub const FRONTMATTER_KEYS: [FrontmatterKey; 13] = [
    FrontmatterKey {
        name: "code",
        read_by: "the diagnostic code this case defines; keys its catalog row. On a whole-product \
                  case it is ALSO the assertion that one of the case's own drives emitted it",
    },
    FrontmatterKey {
        name: "arrangement",
        read_by: "the chrome slug this case defines; keys its arrangement row",
    },
    FrontmatterKey {
        name: "when-fires",
        read_by: "catalog metadata: when the code fires. Replacing it needs --accept-metadata",
    },
    FrontmatterKey {
        name: "when-used",
        read_by: "arrangement metadata: when the chrome is used. Same acknowledgement",
    },
    FrontmatterKey {
        name: "why",
        read_by: "metadata for either registry: why the entry reads as it does. Same acknowledgement",
    },
    FrontmatterKey {
        name: "owns",
        read_by: "extra prose-components this case is the authoring home for; scanned corpus-wide, \
                  one home per component",
    },
    FrontmatterKey {
        name: "todo",
        read_by: "nothing, deliberately: a note about the case's own future",
    },
    FrontmatterKey {
        name: "probe-results",
        read_by: "`authored`: the hand-authored probe records the mocks could not produce, so gate-1's \
                  mocked-probe reproduction compare is opted out",
    },
    FrontmatterKey {
        name: "tolerate",
        read_by: "a named nondeterminism class, whose normalizer is applied to the RUN LOG at bless \
                  and at check. Never to rendered output",
    },
    FrontmatterKey {
        name: "apply-exit",
        read_by: "the exec rail's expected exit for the rendered apply under the case's mocks; \
                  `exec_check` asserts the apply's rc against it. A runner-only execution no \
                  session line spells, default 0",
    },
    FrontmatterKey {
        name: "xfail",
        read_by: "names a pin in `dorc_testbed::xfail::PINS`: the round-trip battery's \
                  structural gates are TOLERATED and reported per gate while the transcript compare \
                  stays enforced, and every structural gate passing is a loud XPASS naming the pin \
                  to promote",
    },
    FrontmatterKey {
        name: "tests-critical-law",
        read_by: "dorc-verify: this case PROPOSES itself as evidence for the named minispec law. \
                  A proposal only; a catalogue promote accepts it, and the binder checks the \
                  agreement both ways",
    },
    FrontmatterKey {
        name: "envelope",
        read_by: "which report seat really prints this code -- `stderr`: the plan route's whole \
                  stderr envelope; `invocation`: the invocation-error seat, prefix and usage \
                  synopsis included",
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
}
