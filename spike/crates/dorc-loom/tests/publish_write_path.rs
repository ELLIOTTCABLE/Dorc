//! `publish`' WRITE path, driven as a process against a world of its own (`-C`).
//!
//! These three could not be written before the root was injectable: reaching the write path meant
//! writing the real corpus and the real locks, so a green `cargo test` could publish a developer's
//! in-progress loom edit. The world here is a temp directory holding one case, both locks and a
//! git repository of its own — the full corpus keeps retirement semantics honest, while one case is
//! selected for rewriting.
//!
//! The case is CHOSEN, never named, and its bytes are re-rendered from the current engine rather
//! than copied: a fixture that named a slug would make this file a second owner of prose the loom
//! flow exists to let somebody rewrite (`aid/CLAUDE.md` prose-pins-live-where-the-prose-does).

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "process-level harness over a temp world; the no-panic lints guard untrusted input"
)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use dorc_aid::prose::ProseTier;
use dorc_loom::{DorcConsumer, Roots};
use errorloom::{Case, CaseRenderer, RenderComponent, RunEnv, RunError};

/// The words a test rewrites a chosen sentence to. Short and one line, so the edit is a prose
/// rewrite rather than an added-line refusal, and greppable enough to find in a generated lock.
const REWRITTEN: &str = "rewritten by the root-injection lane";

/// One committed case re-rendered to current truth, plus the message sentence a test will rewrite.
struct Chosen {
    slug: String,
    canonical: String,
    sentence: String,
}

/// A world `publish` can be pointed at: the tree shape it resolves, a git repository, and the one
/// case it may rewrite.
struct World {
    root: PathBuf,
    chosen: Chosen,
}

impl Drop for World {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

impl World {
    fn new(label: &str, chosen: Chosen) -> Self {
        let root = std::env::temp_dir().join(format!(
            "dorc-loom-publish-{label}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        for dir in ["crates/aid/tests", "crates/aid/src", "target"] {
            std::fs::create_dir_all(root.join(dir)).expect("the world's shape");
        }
        let committed = Roots::built_in().expect("the committed world");
        for lock in [committed.catalog_lock(), committed.arrangement_lock()] {
            let name = lock.file_name().expect("the lock has a filename");
            std::fs::copy(&lock, root.join("crates/aid/src").join(name)).expect("copy the lock");
        }
        for entry in std::fs::read_dir(committed.corpus()).expect("read committed corpus") {
            let path = entry.expect("corpus entry").path();
            if path.extension().is_none_or(|extension| extension != "loom")
                || path
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().contains(".sync-conflict-"))
            {
                continue;
            }
            std::fs::copy(
                &path,
                root.join("crates/aid/tests")
                    .join(path.file_name().expect("case filename")),
            )
            .expect("copy committed case");
        }
        let world = Self { root, chosen };
        std::fs::write(world.case_path(), &world.chosen.canonical).expect("seed the case");
        world.git(&["init", "-q", "-b", "main"]);
        world.git(&["add", "-A"]);
        world.git(&["commit", "-q", "-m", "seed"]);
        world
    }

    fn case_path(&self) -> PathBuf {
        self.root
            .join("crates/aid/tests")
            .join(format!("{}.loom", self.chosen.slug))
    }

    fn catalog_lock(&self) -> PathBuf {
        self.root.join("crates/aid/src/catalog_lock.rs")
    }

    fn staged_publication(&self) -> PathBuf {
        self.root.join("target/dorc-loom/staged.publication")
    }

    /// Rewrite the chosen sentence in the worktree copy of the case — the prose edit every test
    /// here publishes.
    fn rewrite_the_chosen_sentence(&self) -> String {
        let edited = self
            .chosen
            .canonical
            .replace(&self.chosen.sentence, REWRITTEN);
        assert_ne!(edited, self.chosen.canonical, "the rewrite changed nothing");
        std::fs::write(self.case_path(), &edited).expect("write the edit");
        edited
    }

    /// Git, told nothing about the developer running this: no user, system or global config
    /// reaches the world, so a signing or `autocrlf` setting cannot change what it commits.
    fn git(&self, args: &[&str]) {
        let nowhere = self.root.join(".no-git-config");
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .env("GIT_CONFIG_GLOBAL", &nowhere)
            .env("GIT_CONFIG_SYSTEM", &nowhere)
            .env("GIT_AUTHOR_NAME", "dorc-loom tests")
            .env("GIT_AUTHOR_EMAIL", "tests@example.invalid")
            .env("GIT_AUTHOR_DATE", "2000-01-01T00:00:00+00:00")
            .env("GIT_COMMITTER_NAME", "dorc-loom tests")
            .env("GIT_COMMITTER_EMAIL", "tests@example.invalid")
            .env("GIT_COMMITTER_DATE", "2000-01-01T00:00:00+00:00")
            .output()
            .unwrap_or_else(|error| panic!("git {args:?}: {error}"));
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn publish(&self, extra: &[&str]) -> Output {
        let root = self.root.to_str().expect("the world path is UTF-8");
        let mut args = vec!["-C", root, "publish"];
        args.extend(extra);
        args.push(&self.chosen.slug);
        Command::new(env!("CARGO_BIN_EXE_dorc-loom"))
            .args(&args)
            .output()
            .unwrap_or_else(|error| panic!("dorc-loom starts: {error}"))
    }
}

/// The committed collection, read at RUN time.
fn corpus_dir() -> PathBuf {
    Roots::built_in().expect("the committed world").corpus()
}

/// A case whose OWN message register renders as one unwrapped sentence somewhere in its
/// transcript, carrying holes or not as the caller needs.
///
/// Everything in the filter is load-bearing. The case must own the register it edits (a foreign
/// component refuses); its register must not be human-written (an agent-vs-person demotion branch
/// would make the exit code depend on who ran the suite); the sentence must appear EXACTLY once in
/// the file, so a string rewrite is a replay-output edit and not a frontmatter one.
fn a_case_whose_message_sentence_can_be_rewritten(wants_holes: bool) -> Chosen {
    let consumer = DorcConsumer::new();
    let mut paths: Vec<PathBuf> = std::fs::read_dir(corpus_dir())
        .expect("the corpus dir is readable")
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.extension().is_some_and(|kind| kind == "loom"))
        // `sync-residue-is-never-a-case`: a conflict copy keeps the extension.
        .filter(|path| !path.to_string_lossy().contains(".sync-conflict-"))
        .collect();
    paths.sort();
    for path in &paths {
        let Some(chosen) = candidate(&consumer, path, wants_holes) else {
            continue;
        };
        return chosen;
    }
    panic!(
        "no case in the collection carries a rewritable message sentence with holes={wants_holes} \
         ({} tried)",
        paths.len()
    )
}

fn candidate(consumer: &DorcConsumer, path: &Path, wants_holes: bool) -> Option<Chosen> {
    let text = std::fs::read_to_string(path).ok()?;
    let case = Case::parse(&text).ok()?;
    let slug = case.frontmatter().scalar("code")?.to_owned();
    if path.file_stem()? != slug.as_str() {
        return None;
    }
    let entry = consumer.mirror().iter().find(|entry| entry.slug == slug)?;
    if !matches!(
        entry.message,
        Some(ProseTier::Slop(_) | ProseTier::Migrated(_))
    ) {
        return None;
    }
    let render = dorc_loom::replay_case(&case, consumer, &RunEnv::new(), |_, _| {
        Err(RunError::ShellNotConfigured)
    })
    .ok()?
    .into_iter()
    .rev()
    .find_map(|result| result.editable_render().cloned())?;
    let baseline = consumer.baseline_from_render(&case, render).ok()?;
    let (sentence, holes) = baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            RenderComponent::EditableSection(section)
                if section.id().owner == slug && section.id().field == "message" =>
            {
                Some(measure(section))
            }
            _ => None,
        })?;
    let canonical = consumer.render_case(&case).ok()?;
    ((holes > 0) == wants_holes && canonical.matches(&sentence).count() == 1).then_some(Chosen {
        slug,
        canonical,
        sentence,
    })
}

/// One section's rendered bytes and how many holes it spends.
fn measure(
    section: &errorloom::EditableSection<dorc_loom::SectionKey, dorc_loom::SectionVariableId>,
) -> (String, usize) {
    let mut text = String::new();
    let mut holes = 0usize;
    for fragment in section.fragments() {
        match fragment {
            errorloom::EditableFragment::Text(bytes) => text.push_str(bytes),
            errorloom::EditableFragment::Variable { rendered, .. } => {
                holes = holes.saturating_add(1);
                text.push_str(rendered);
            }
        }
    }
    (text, holes)
}

/// The committed collection and both committed locks, so a test can prove a `-C` run left them
/// exactly as it found them.
fn committed_bytes() -> Vec<(PathBuf, Vec<u8>)> {
    let world = Roots::built_in().expect("the committed world");
    let mut watched = vec![world.catalog_lock(), world.arrangement_lock()];
    watched.extend(
        std::fs::read_dir(world.corpus())
            .expect("the corpus dir is readable")
            .filter_map(|entry| Some(entry.ok()?.path()))
            .filter(|path| path.extension().is_some_and(|kind| kind == "loom")),
    );
    watched.sort();
    watched
        .into_iter()
        .map(|path| {
            let bytes = std::fs::read(&path).expect("a committed file is readable");
            (path, bytes)
        })
        .collect()
}

fn assert_untouched(before: &[(PathBuf, Vec<u8>)]) {
    for (path, bytes) in before {
        assert_eq!(
            std::fs::read(path).ok().as_ref(),
            Some(bytes),
            "a -C run reached outside its root: {}",
            path.display()
        );
    }
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A publish that gives nothing up writes both the case and the lock, exits 0 — and, because `-C`
/// confines it, leaves every committed byte of the real collection alone.
#[test]
fn a_clean_publish_writes_the_case_and_the_lock_inside_its_own_root() {
    let world = World::new(
        "clean",
        a_case_whose_message_sentence_can_be_rewritten(false),
    );
    let committed = committed_bytes();
    let lock_before = read(&world.catalog_lock());
    world.rewrite_the_chosen_sentence();

    let output = world.publish(&[]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));

    let lock = read(&world.catalog_lock());
    assert_ne!(lock, lock_before, "a published edit moves the lock");
    assert!(lock.contains(REWRITTEN), "the register carries the words");
    let case = read(&world.case_path());
    assert!(
        case.contains(REWRITTEN) && case != world.chosen.canonical,
        "the case was republished from the edited register"
    );
    assert_untouched(&committed);
}

/// A publish that gives up a hole writes NOTHING and exits nonzero: the transcript renders values,
/// so the loss is invisible in the case diff and the printed census is the only place it shows
/// (`30C:rul-any-hole-loss-confirms`).
#[test]
fn a_publish_that_gives_up_a_hole_writes_nothing_and_stages_itself() {
    let world = World::new(
        "uncertain",
        a_case_whose_message_sentence_can_be_rewritten(true),
    );
    let committed = committed_bytes();
    let lock_before = read(&world.catalog_lock());
    let edited = world.rewrite_the_chosen_sentence();

    let output = world.publish(&[]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(said.contains("gives up"), "{said}");
    assert!(said.contains("--verbatim"), "{said}");

    assert_eq!(read(&world.catalog_lock()), lock_before);
    assert_eq!(read(&world.case_path()), edited);
    assert!(
        world.staged_publication().is_file(),
        "the refusal holds the interpretation --verbatim must match"
    );
    assert_untouched(&committed);
}

/// `--verbatim` applies the interpretation the refusal above printed, and SPENDS it: leaving the
/// staging would let a second one re-confirm a loss against bytes nobody looked at this time.
#[test]
fn verbatim_applies_the_staged_interpretation_and_leaves_no_staging() {
    let world = World::new(
        "verbatim",
        a_case_whose_message_sentence_can_be_rewritten(true),
    );
    let committed = committed_bytes();
    world.rewrite_the_chosen_sentence();

    let refused = world.publish(&[]);
    assert_eq!(refused.status.code(), Some(1), "{}", stderr(&refused));

    let confirmed = world.publish(&["--verbatim"]);
    assert_eq!(confirmed.status.code(), Some(0), "{}", stderr(&confirmed));
    assert!(
        read(&world.catalog_lock()).contains(REWRITTEN),
        "the confirmed interpretation reached the register"
    );
    assert!(
        !world.staged_publication().exists(),
        "an applied interpretation is spent"
    );
    assert_untouched(&committed);
}

/// The other end of the same rule, through the process rather than the store: `--verbatim` applies
/// something you were SHOWN, so with nothing staged it refuses and names the plain re-run
/// (`28L:rul-refusals-name-the-next-command`).
#[test]
fn verbatim_with_nothing_staged_refuses_and_names_the_plain_run() {
    let world = World::new(
        "unstaged",
        a_case_whose_message_sentence_can_be_rewritten(true),
    );
    world.rewrite_the_chosen_sentence();

    let output = world.publish(&["--verbatim"]);
    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(said.contains("nothing is staged"), "{said}");
    assert!(
        said.contains(&format!("dorc-loom publish {}", world.chosen.slug)),
        "{said}"
    );
    assert!(read(&world.catalog_lock()).contains("slug:"));
}

/// A publish with no edit in front of it does nothing and says so, rather than reporting a zero in
/// a summary line a reader cannot tell from "wrong worktree, wrong file" (`30C` item 6).
#[test]
fn a_publish_with_no_edit_changes_nothing_and_says_so() {
    let world = World::new(
        "nothing",
        a_case_whose_message_sentence_can_be_rewritten(false),
    );
    let committed = committed_bytes();
    let lock_before = read(&world.catalog_lock());

    let output = world.publish(&[]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("this publish changed nothing"),
        "{}",
        stderr(&output)
    );
    assert_eq!(read(&world.catalog_lock()), lock_before);
    assert_eq!(read(&world.case_path()), world.chosen.canonical);
    assert_untouched(&committed);
}
