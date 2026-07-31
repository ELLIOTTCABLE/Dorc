//! Which case is the authoring HOME of which prose-component (`28L:rul-ownership-declaration-adopted`).
//!
//! A case's FILENAME always contributes its implicit entry when the stem names a registered
//! component; the closed `owns:` frontmatter key is what a case carrying SEVERAL components needs,
//! and nothing else. One component has one owner corpus-wide, so a second declarant is refused by
//! name — an edit that reached a foreign component through a multi-component render would rewrite
//! an entry whose own case still says the old thing.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use errorloom::{Case, FrontmatterValue};

/// The frontmatter key a case declares extra prose-components in.
pub const OWNS_KEY: &str = "owns";

/// The frontmatter key carrying the case's own editing loop.
pub const EDIT_LOOP_KEY: &str = "edit-loop";

/// The frontmatter key opting a plan-route replay into a wider stderr surface.
pub const ENVELOPE_KEY: &str = "envelope";

/// The one [`ENVELOPE_KEY`] value: render the plan route's whole stderr envelope.
pub const ENVELOPE_STDERR: &str = "stderr";

/// The loop, spelled out inside the case that needs it — GENERATED, never authored.
///
/// A `.loom` is the whole teaching surface for someone who may not open the crates that read it
/// (`28L:rul-rust-and-loom-are-the-only-edit-surfaces`), and the two things it could not tell them
/// were what to run afterwards and that a value can be typed at all. One const mints it, one gate
/// holds every case to it, so it cannot drift into 75 slightly different sentences.
#[must_use]
pub fn edit_loop_hint(slug: &str) -> String {
    format!(
        "edit a sentence in the transcript below, then run \
         `mise run loom:compile -- {slug} && mise run loom:promote -- {slug}`. \
         `mise run loom:vars -- --all {slug}` lists this case's values; type {{{{name}}}} to \
         insert or move one."
    )
}

/// One prose-component, as an ownership declaration names it: a registry/catalog slug, optionally
/// narrowed to one occurrence with `slug@N`. A slug with no occurrence claims every occurrence.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ComponentRef {
    /// The component's slug.
    pub slug: String,
    /// The occurrence claimed, or `None` for the whole slug.
    pub occurrence: Option<usize>,
}

impl ComponentRef {
    /// Parse one declaration entry, or `None` when the `@` tail is not a number.
    fn parse(entry: &str) -> Option<Self> {
        match entry.split_once('@') {
            None => Some(Self {
                slug: entry.to_owned(),
                occurrence: None,
            }),
            Some((slug, occurrence)) => Some(Self {
                slug: slug.to_owned(),
                occurrence: Some(occurrence.parse().ok()?),
            }),
        }
    }
}

/// Every prose-component the corpus declares an authoring home for, keyed by component.
#[derive(Clone, Default, Debug)]
pub struct CaseOwnership {
    owners: BTreeMap<ComponentRef, PathBuf>,
}

impl CaseOwnership {
    /// Walk `dir`'s cases and resolve the declaration-union.
    ///
    /// `registered` answers whether a slug names a real component; a filename matching nothing
    /// registered contributes no implicit entry, so an `.rs` test's fixture case beside the corpus
    /// claims nothing.
    ///
    /// An explicit `owns:` OUTRANKS a filename's implicit claim rather than colliding with it, so
    /// the losing case is not refused here — it is refused at the moment it matters, when an edit
    /// through it hits [`refuse_foreign_components`] and is told which file to make the change in.
    ///
    /// # Errors
    /// Returns a refusal naming BOTH files when two cases DECLARE one component, and a refusal
    /// naming the file when a case is unreadable, unparseable, or spells an entry it cannot mean.
    pub fn scan(dir: &Path, registered: &dyn Fn(&str) -> bool) -> Result<Self, String> {
        let mut owners: BTreeMap<ComponentRef, PathBuf> = BTreeMap::new();
        let entries =
            std::fs::read_dir(dir).map_err(|error| format!("read corpus dir: {error}"))?;
        let mut paths: Vec<PathBuf> = Vec::new();
        for entry in entries {
            let path = entry
                .map_err(|error| format!("read corpus entry: {error}"))?
                .path();
            if path
                .extension()
                .is_some_and(|extension| extension == "loom")
            {
                paths.push(path);
            }
        }
        paths.sort();
        // DECLARED claims first, so an explicit `owns:` wins the slug its FILENAME would otherwise
        // claim implicitly. A case is named for the world it demonstrates, and the case that
        // RENDERS a component editably is often a different one; without this the component's home
        // is a case that can only refuse the edit, and the prose has no home at all.
        let mut declared: Vec<(PathBuf, Vec<ComponentRef>, Vec<ComponentRef>)> = Vec::new();
        for path in paths {
            let (explicit, implicit) = claims_of(&path, registered)?;
            declared.push((path, explicit, implicit));
        }
        for (path, explicit, _) in &declared {
            for claim in explicit {
                insert(&mut owners, claim.clone(), path)?;
            }
        }
        for (path, _, implicit) in &declared {
            for claim in implicit {
                if owners.keys().any(|owned| owned.slug == claim.slug) {
                    continue;
                }
                insert(&mut owners, claim.clone(), path)?;
            }
        }
        Ok(Self { owners })
    }

    /// Whether any case is the authoring home of `slug`, at any occurrence.
    #[must_use]
    pub fn owns(&self, slug: &str) -> bool {
        self.owners.keys().any(|owned| owned.slug == slug)
    }

    /// The case that owns `slug`'s given occurrence, if one does.
    #[must_use]
    pub fn owner(&self, slug: &str, occurrence: Option<usize>) -> Option<&Path> {
        self.owners
            .iter()
            .find(|(owned, _)| {
                owned.slug == slug && (owned.occurrence.is_none() || owned.occurrence == occurrence)
            })
            .map(|(_, path)| path.as_path())
    }

    /// The case that owns `slug`, when it is NOT `editing` — the per-edit half of the one-home law.
    ///
    /// Scan-time uniqueness stops two cases CLAIMING one component; this stops a case that merely
    /// RENDERS one from rewriting it. Eleven invocation-error cases print the usage synopsis, so
    /// without this an edit through any of them lands on an entry whose own case still says the old
    /// thing. A component nobody owns is NOT foreign — that is the mint path, and it stays open.
    #[must_use]
    pub fn foreign_owner(
        &self,
        slug: &str,
        occurrence: Option<usize>,
        editing: &Path,
    ) -> Option<&Path> {
        // Cases are flat in one collection, so the filename is the identity — and it is the only
        // comparison that survives a caller who named the case by slug rather than by path.
        self.owner(slug, occurrence)
            .filter(|owner| owner.file_name() != editing.file_name())
    }
}

/// Refuse a compiled edit that reaches a prose-component another case is the authoring home of.
///
/// The per-EDIT half of the one-home law; [`CaseOwnership::scan`] holds the corpus-wide half.
/// A section's occurrence is meaningful only for chrome, where the registry key is
/// `(slug, occurrence)`; a catalog register's `instance` counts render positions of one code, so
/// it never narrows ownership.
///
/// # Errors
/// Returns [`crate::DorcSectionEditRefusal::ForeignComponent`] naming the component and its home.
pub fn refuse_foreign_components(
    ownership: &CaseOwnership,
    editing: &Path,
    preview: &crate::CompilePreview,
) -> Result<(), crate::DorcSectionEditRefusal> {
    for section in preview.sections() {
        let key = section.section();
        let occurrence = matches!(
            key.field,
            crate::ARRANGEMENT_FIELD | crate::ARRANGEMENT_LINE_FIELD
        )
        .then_some(key.instance);
        let Some(owner) = ownership.foreign_owner(&key.owner, occurrence, editing) else {
            continue;
        };
        return Err(crate::DorcSectionEditRefusal::ForeignComponent {
            component: key.owner.clone(),
            // The FILENAME, which every case-taking verb resolves: a full path here would be
            // sixty columns of worktree the reader already knows they are standing in.
            owner: owner.file_name().map_or_else(
                || owner.display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            ),
        });
    }
    Ok(())
}

/// One case's `(declared, implicit)` claims: each `owns:` entry, and its filename's entry.
fn claims_of(
    path: &Path,
    registered: &dyn Fn(&str) -> bool,
) -> Result<(Vec<ComponentRef>, Vec<ComponentRef>), String> {
    let mut implicit = Vec::new();
    if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str())
        && registered(stem)
    {
        implicit.push(ComponentRef {
            slug: stem.to_owned(),
            occurrence: None,
        });
    }
    let mut claims = Vec::new();
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read case {}: {error}", path.display()))?;
    let case =
        Case::parse(&text).map_err(|error| format!("parse case {}: {error}", path.display()))?;
    for entry in declared(&case) {
        let claim = ComponentRef::parse(&entry).ok_or_else(|| {
            format!(
                "{}: `{OWNS_KEY}: {entry}` is not a component — write `<slug>` or `<slug>@<number>`, \
                 the two spellings `dorc-loom sections` prints",
                path.display()
            )
        })?;
        if !registered(&claim.slug) {
            return Err(format!(
                "{}: `{OWNS_KEY}: {entry}` names no known prose-component. List the ones this case \
                 renders: dorc-loom sections {}",
                path.display(),
                path.display()
            ));
        }
        if !claims.contains(&claim) {
            claims.push(claim);
        }
    }
    implicit.retain(|entry| !claims.contains(entry));
    Ok((claims, implicit))
}

/// The `owns:` entries a case declares, scalar or list.
fn declared(case: &Case) -> Vec<String> {
    match case.frontmatter().get(OWNS_KEY) {
        Some(FrontmatterValue::Scalar(one)) => vec![one.clone()],
        Some(FrontmatterValue::List(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// Claim `component` for `path`, refusing a second declarant by name.
///
/// A whole-slug claim covers every occurrence, so it collides with an occurrence-narrowed claim on
/// the same slug from a different file — two files would otherwise both believe they were the home.
fn insert(
    owners: &mut BTreeMap<ComponentRef, PathBuf>,
    component: ComponentRef,
    path: &Path,
) -> Result<(), String> {
    let clash = owners.iter().find(|(owned, held)| {
        owned.slug == component.slug
            && held.as_path() != path
            && (owned.occurrence.is_none()
                || component.occurrence.is_none()
                || owned.occurrence == component.occurrence)
    });
    if let Some((owned, held)) = clash {
        return Err(format!(
            "prose-component `{}` is claimed by two cases: {} and {}. One component has one \
             authoring home — delete the `{OWNS_KEY}:` entry from whichever case is not it, then: \
             mise run loom:compile {}",
            spelling(owned),
            held.display(),
            path.display(),
            path.display()
        ));
    }
    owners.insert(component, path.to_owned());
    Ok(())
}

fn spelling(component: &ComponentRef) -> String {
    match component.occurrence {
        Some(occurrence) => format!("{}@{occurrence}", component.slug),
        None => component.slug.clone(),
    }
}

/// Whether `slug` names a component the built binary knows — the registration test the corpus scan
/// takes, and the one every caller wants unless it is testing the scan itself.
#[must_use]
pub fn is_registered_component(slug: &str) -> bool {
    dorc_aid::catalog::entry(slug).is_some()
        || dorc_aid::arrangement::ARRANGEMENTS
            .iter()
            .any(|entry| entry.slug == slug)
}

/// The canonical corpus scan: the primary loom collection, resolved against the built registries.
///
/// # Errors
/// Returns [`CaseOwnership::scan`]'s refusals.
pub fn corpus_ownership(dir: &Path) -> Result<CaseOwnership, String> {
    CaseOwnership::scan(dir, &is_registered_component)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus(cases: &[(&str, &str)]) -> (tempdir::TempDir, Vec<PathBuf>) {
        let dir = tempdir::TempDir::new();
        let mut paths = Vec::new();
        for (name, body) in cases {
            let path = dir.path().join(format!("{name}.loom"));
            std::fs::write(&path, body).expect("fixture case writes");
            paths.push(path);
        }
        (dir, paths)
    }

    fn case(owns: &str) -> String {
        format!(
            "---\narrangement: alpha\nwhen-used: harness.\nwhy: harness.\n{owns}---\n\
             -- replay --\n$ dorc --help\nx\n"
        )
    }

    fn every_slug(_: &str) -> bool {
        true
    }

    /// The filename is the default singleton and needs no key: an existing single-owner case keeps
    /// working untouched, which is the whole reason `owns:` is only for multi-component homes.
    #[test]
    fn a_filename_alone_owns_its_namesake() {
        let (dir, _) = corpus(&[("alpha", &case(""))]);
        let ownership = CaseOwnership::scan(dir.path(), &every_slug).expect("scan");
        assert!(ownership.owns("alpha"));
        assert!(!ownership.owns("beta"));
    }

    /// The declaration-union: the key ADDS to the filename's implicit entry rather than replacing
    /// it, so a multi-component home does not have to re-declare itself.
    #[test]
    fn the_key_adds_components_to_the_filename() {
        let (dir, _) = corpus(&[("alpha", &case("owns:\n  - beta\n  - gamma@1\n"))]);
        let ownership = CaseOwnership::scan(dir.path(), &every_slug).expect("scan");
        for slug in ["alpha", "beta", "gamma"] {
            assert!(ownership.owns(slug), "{slug}");
        }
        assert!(ownership.owner("gamma", Some(1)).is_some());
        assert!(
            ownership.owner("gamma", Some(0)).is_none(),
            "an occurrence-narrowed claim covers only that occurrence"
        );
    }

    /// A declared home beats the filename's implicit one, and does not collide with it. A case is
    /// named for the world it demonstrates; the case that renders a component EDITABLY is often a
    /// different one, and pinning the home to the filename left such a component with an owner
    /// that could only refuse the edit.
    #[test]
    fn an_explicit_claim_outranks_a_filename() {
        let (dir, _) = corpus(&[("alpha", &case("")), ("beta", &case("owns: alpha\n"))]);
        let ownership = CaseOwnership::scan(dir.path(), &every_slug).expect("scan");
        assert_eq!(
            ownership.owner("alpha", None).and_then(Path::file_name),
            Some(std::ffi::OsStr::new("beta.loom"))
        );
    }

    /// One component, one home. The refusal has to name BOTH files, because either one could be
    /// the mistake and the reader is holding only one of them.
    #[test]
    fn a_second_declarant_refuses_naming_both_files() {
        let (dir, _) = corpus(&[
            ("alpha", &case("owns:\n  - shared\n")),
            ("beta", &case("owns:\n  - shared\n")),
        ]);
        let refusal = CaseOwnership::scan(dir.path(), &every_slug).expect_err("two homes refuse");
        assert!(
            refusal.contains("alpha.loom") && refusal.contains("beta.loom"),
            "both declarants are named: {refusal}"
        );
        assert!(
            refusal.contains("mise run loom:compile"),
            "the refusal ends in its next command: {refusal}"
        );
    }

    /// A whole-slug claim covers every occurrence, so it collides with an occurrence-narrowed one
    /// from another file rather than quietly layering over it.
    #[test]
    fn a_whole_slug_claim_collides_with_a_narrowed_one() {
        let (dir, _) = corpus(&[
            ("alpha", &case("owns:\n  - shared\n")),
            ("beta", &case("owns:\n  - shared@2\n")),
        ]);
        assert!(CaseOwnership::scan(dir.path(), &every_slug).is_err());
    }

    /// A slug nothing registers is a typo, and the refusal says where the real names are printed.
    #[test]
    fn an_unregistered_claim_refuses_with_the_listing_command() {
        let (dir, _) = corpus(&[("alpha", &case("owns: nonesuch\n"))]);
        let refusal =
            CaseOwnership::scan(dir.path(), &|slug| slug == "alpha").expect_err("typo refuses");
        assert!(
            refusal.contains("dorc-loom sections"),
            "the refusal names the census command: {refusal}"
        );
    }

    /// The committed collection resolves cleanly — the corpus-wide half of the one-home law.
    #[test]
    fn the_committed_corpus_has_one_home_per_component() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests");
        corpus_ownership(&dir).expect("the committed corpus resolves");
    }

    /// A scratch directory that removes itself, so the fixtures above never touch the corpus.
    mod tempdir {
        use std::path::{Path, PathBuf};

        pub(super) struct TempDir(PathBuf);

        impl TempDir {
            pub(super) fn new() -> Self {
                let path = std::env::temp_dir().join(format!(
                    "dorc-loom-ownership-{}-{:?}",
                    std::process::id(),
                    std::thread::current().id()
                ));
                let _ = std::fs::remove_dir_all(&path);
                std::fs::create_dir_all(&path).expect("scratch dir");
                Self(path)
            }

            pub(super) fn path(&self) -> &Path {
                &self.0
            }
        }

        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }
}
