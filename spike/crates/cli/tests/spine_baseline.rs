//! The decision-state SMOKE-DIFF baseline (`309` §4; `30E` §6) — **migration scaffolding,
//! build-to-kill**.
//!
//! Byte gates are known-vacuous at byte-lossy seams on this refactor class (`28Q` §8's stage-0
//! retroactive audit: outcomes hold while records silently change). Two such seams are known here —
//! the decision digest collapses in-loop member sites into their leaf, and a guard's binding can
//! move between byte-identical definition bodies carrying different custody. Both change
//! ATTRIBUTION that no golden sees, and big-bang leaves no bisection point.
//!
//! So: freeze this walk's output at the base commit, land the Spine, then have a Spine projection
//! reproduce the schema and diff the two by eye at the fold sitting.
//!
//! **It is a smoke-testing machine, NOT an acceptance gate** (`309` §4, TYPED). Non-empty output is
//! material for judgment, never an auto-fail — gaming it leads to backflips.
//!
//! Three fences, each load-bearing:
//!
//! 1. it is never the whylog — no durable-tripwire contact, and it writes only where told;
//! 2. it is never the census `new`-arm debug dump (`309` §3) — different mechanism, different
//!    lifetime, and its schema INFORMS the owed `SiteId` decision-dump product feature without ever
//!    becoming it;
//! 3. it runs only when asked (`#[ignore]`, driven by `mise run spine:baseline`), so it costs the
//!    ordinary suite nothing but still COMPILES with it — a signature change during the burn-down
//!    reddens here loudly rather than rotting.
//!
//! Honest residual (`309` §4): this covers only decision-state the current code makes explicit
//! enough to walk. A fully-implicit decision is invisible to the baseline exactly as it is to the
//! census, which is why the hidden-decision audit (`30E` §3) is a separate instrument.
//!
//! COVERAGE, measured 2026-08-17 — **read this before trusting a clean diff.** Of 232 committed
//! cases the intake ADMITS 111; 86 commit no records at all, and 35 still refuse. The corpus
//! commits `probe-results.txt` RAW, so every case is re-framed through `support::frame_records` —
//! the e2e runner's own seat — after a real `dorc probe` run supplies the header and site list.
//! Getting there mattered: before the re-homing this file admitted 9 cases and froze 8 `Replace`,
//! 3 `Guard` and 2 bindings, which is the instrument not existing, since both seams it watches
//! (the member-collapse, and a guard binding moving between byte-identical bodies with different
//! custody) live only in MEASURED worlds. It now freezes 80 / 28 / 21 / 26.
//!
//! The residual 35 skew toward `-runs` / `-walls` / `-tops` cases, whose sites run regardless, so
//! what they would contribute is largely the same either way — observed from the case names, not
//! exhaustively verified. A refused stream analyses as the unmeasured world (every fact ⊤ ⇒ every
//! site runs) and the row says `records refused`, never a bare absence.
//!
//! Case discovery is DUPLICATED from `definition_frames.rs` rather than shared through
//! `support.rs`: a parallel custody lane is editing that battery, and a build-to-kill file is the
//! wrong place to spend a merge conflict.

#![expect(
    clippy::print_stderr,
    reason = "`support`'s selection reporter is compiled into every test binary that uses it, and this instrument reports its own destination to a human running it by hand"
)]

mod support;

/// The load snapshot a hand-built world is: sources in load order, the book last, and one flat
/// modeled working directory their relative paths are spelled against.
fn snapshot_of(
    paths: &[String],
    srcs: &[String],
    book_path: &str,
    book_src: &str,
) -> dorc_cli::snapshot::StaticLoadSnapshot {
    dorc_cli::snapshot::StaticLoadSnapshot::over(
        dorc_core::loadpath::Cwd::default(),
        paths.to_vec(),
        srcs.to_vec(),
        &dorc_cli::snapshot::LoadPositions::roots_only(),
        book_path,
        book_src,
    )
}

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use dorc_cli::results::{RunClock, RunSources, admit_fixture_records};
use dorc_core::Interner;
use dorc_plan::Disposition;
use dorc_plan::records::Admission;

/// Where the baseline is written. Unset ⇒ the walk still runs and its floor still asserts, so the
/// instrument cannot rot into a no-op that reports success.
const DUMP_VAR: &str = "DORC_SPINE_DUMP";

/// One committed case, with the sources and records a run of it would load.
struct CaseWorld {
    label: String,
    paths: Vec<String>,
    srcs: Vec<String>,
    book: String,
    /// The case's records, RE-FRAMED through `support::frame_records` exactly as the e2e runner
    /// frames them. Absent ⇒ the unmeasured world (every fact ⊤ ⇒ every site runs), which decides
    /// far less and is worth far less to diff.
    results: Option<String>,
}

/// Run the built `dorc probe` over a dir-form case and re-frame its committed records.
///
/// The corpus commits records RAW; only a real probe run supplies the header and site list the
/// intake checks against. `CARGO_BIN_EXE_dorc` is the binary cargo built for this target, so the
/// baseline measures the same engine the suite does.
fn framed_for(dir: &Path) -> Option<String> {
    if !dir.join("probe-results.txt").is_file() {
        return None;
    }
    let probe = std::process::Command::new(env!("CARGO_BIN_EXE_dorc"))
        .arg("probe")
        .arg(format!("--book={}", dir.join("book.sh").display()))
        .args(oracle_args(dir))
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    let framed = support::frame_records(&String::from_utf8_lossy(&probe.stdout), dir);
    (!framed.is_empty()).then_some(framed)
}

/// The `-o <path>` arguments a dir-form case's run carries, glob-sorted like the runner's.
fn oracle_args(dir: &Path) -> Vec<String> {
    let mut oracles: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".oracle.sh"))
        })
        .collect();
    oracles.sort();
    oracles
        .iter()
        .flat_map(|path| ["--pre-source".to_owned(), path.display().to_string()])
        .collect()
}

#[test]
#[ignore = "migration scaffolding: `mise run spine:baseline` drives it"]
fn spine_decision_state_baseline() {
    let worlds = corpus_worlds();
    assert!(
        !worlds.is_empty(),
        "discovery floor: the corpus walk found no worlds, so this baseline would freeze nothing"
    );

    let mut out = String::new();
    let _ = writeln!(out, "dorc-spine-baseline/1 cases={}", worlds.len());
    for world in &worlds {
        render_case(&mut out, world);
    }

    match std::env::var(DUMP_VAR) {
        Ok(path) if !path.is_empty() => {
            std::fs::write(&path, &out).expect("the baseline destination is writable");
            eprintln!("spine baseline: {} cases -> {path}", worlds.len());
        }
        _ => eprintln!(
            "spine baseline: {} cases walked; set {DUMP_VAR}=<path> to freeze them",
            worlds.len()
        ),
    }
}

/// One case's decision-state, in a fixed field order over sorted rows (`inv-determinism`).
fn render_case(out: &mut String, world: &CaseWorld) {
    let mut interner = Interner::default();
    let sources = RunSources {
        book_name: &world.label,
        book: &world.book,
        oracle_paths: &world.paths,
        oracle_sources: &world.srcs,
    };

    // The FIXTURE intake — the sanctioned route for a harness to drive the real admission over a
    // case's own committed bytes. Its signature cannot name a managed host (`28L`
    // rul-records-seam-approved, rider (b)).
    let mut clock = RunClock::Absent;
    let measured = world.results.as_ref().and_then(|text| {
        match admit_fixture_records(&sources, text.as_bytes(), &mut clock, &mut interner) {
            Admission::Admitted(admitted) => Some(admitted.scoped),
            Admission::NoObservation | Admission::Refused(_) => None,
        }
    });

    let empty = dorc_cli::results::SiteResults::default();
    let results = measured.as_ref().map_or(&empty, |scoped| scoped.results());
    // Consented, so the survival tier EXISTS to be dumped; unflagged it is absent rather than quiet
    // (`empty-world-byte-identical`), which would hide every wall-crossing from the diff.
    let built = dorc_cli::world::WhyWorld::analyze_measured(
        &snapshot_of(&world.paths, &world.srcs, &world.label, &world.book),
        results,
        true,
    );
    // The world's OWN interner: a `Symbol` resolves only against the one that minted it, and the
    // local interner above belongs to the intake, not to the analysis.
    let (plan, _ast, symbols) = built.plan_ast_and_interner();

    let _ = writeln!(out, "case {}", world.label);
    let _ = writeln!(out, "  digest {}", built.presented_plan_hex());
    // What ADMISSION answered, never whether a file was present: a case whose committed records the
    // intake refuses analyses as the unmeasured world, and reporting its mere file as `measured`
    // would overstate what this baseline froze.
    let _ = writeln!(
        out,
        "  records {}",
        match (&world.results, &measured) {
            (None, _) => "absent",
            (Some(_), Some(_)) => "admitted",
            (Some(_), None) => "refused",
        }
    );

    let pinned = plan.pinned_definitions();
    // Read the DECISION PLANE, not the projection: byte-identity against the frozen baseline then
    // proves the reification itself, rather than proving one projection agrees with itself
    // (`309` §4). The Spine is `SiteId`-keyed, so the member index appears here the moment one
    // exists — the known keying change needs no whitelist.
    for record in built.spine().dispositions() {
        let member = record
            .site()
            .member
            .map_or_else(|| "-".to_owned(), |m| m.to_string());
        let _ = write!(
            out,
            "  site {}.{member} ast={} ",
            record.site().leaf.0,
            record.ast().0
        );
        match record.decision() {
            Disposition::Run => out.push_str("run"),
            Disposition::Replace(license, stand_in) => {
                let _ = write!(
                    out,
                    "replace custody={:?} via={:?} standin={stand_in:?} fact={}",
                    license.custody(),
                    license.derivation().via,
                    dorc_plan::fact_label(symbols, license.fact()),
                );
            }
            Disposition::Omit { controller } => {
                let _ = write!(out, "omit controller={}", controller.0);
            }
            Disposition::Guard(license) => {
                let _ = write!(
                    out,
                    "guard fact={}",
                    dorc_plan::fact_label(symbols, license.fact())
                );
            }
        }
        // The BINDING, beside the decision it belongs to: which body this guard invokes, under what
        // name. `pinned-definitions-are-the-artifact's-binding` is a render-time decision no golden
        // distinguishes when two definition bodies are byte-identical, which is the seam — and it is
        // now a Spine record of its own (`30E` §3), so the diff reads it there.
        if let Some(name) = pinned.invoked(record.ast()) {
            let _ = write!(out, " invoked={name}");
        }
        out.push('\n');
    }

    // The hoisted preamble by DIGEST rather than by bytes: the diff wants to know that the binding
    // set moved, not to re-litigate the emission's formatting.
    let _ = writeln!(
        out,
        "  hoisted bytes={} digest={}",
        pinned.typeset(&dorc_plan::Placement::Hoist).len(),
        short_digest(&pinned.typeset(&dorc_plan::Placement::Hoist))
    );

    let survival = &plan.survival_report;
    let _ = writeln!(out, "  may-alias {}", survival.may_alias_fires());
    let mut poisonings: Vec<String> = survival
        .reach_poisonings()
        .map(|(leaf, kind)| format!("  poisoned {} kind={}", leaf.0, symbols.resolve(kind.0)))
        .collect();
    poisonings.sort();
    for row in poisonings {
        let _ = writeln!(out, "{row}");
    }
    let mut demotions: Vec<String> = survival
        .rederivation_demotions()
        .map(|(leaf, wall)| format!("  rederive-demoted {} wall={wall}", leaf.0))
        .collect();
    demotions.sort();
    for row in demotions {
        let _ = writeln!(out, "{row}");
    }
}

/// A short content digest, for comparing emitted blobs without pinning their bytes.
fn short_digest(text: &str) -> String {
    dorc_plan::invocation::book_digest(text)
        .get(..12)
        .unwrap_or_default()
        .to_owned()
}

/// Every committed case that has a book, with the sources a run of it would load.
fn corpus_worlds() -> Vec<CaseWorld> {
    let mut out = Vec::new();
    for root in support::case_roots() {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if name.contains(".sync-conflict-") {
                continue; // sync residue is never a case
            }
            if path.is_dir() {
                if let Some(world) = dir_world(&path, &name) {
                    out.push(world);
                }
                let inner = path.join(format!("{name}.loom"));
                if inner.is_file()
                    && let Some(world) = loom_world(&inner, &name)
                {
                    out.push(world);
                }
            } else if Path::new(&name)
                .extension()
                .is_some_and(|got| got == "loom")
                && let Some(world) = loom_world(&path, &name)
            {
                out.push(world);
            }
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// A dir-form case's world: `book.sh` plus its glob-sorted `*.oracle.sh` siblings.
fn dir_world(dir: &Path, case: &str) -> Option<CaseWorld> {
    let book = std::fs::read_to_string(dir.join("book.sh")).ok()?;
    let mut oracles: Vec<_> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".oracle.sh"))
        })
        .collect();
    oracles.sort();
    let mut paths = Vec::new();
    let mut srcs = Vec::new();
    for oracle in oracles {
        let Ok(text) = std::fs::read_to_string(&oracle) else {
            continue;
        };
        paths.push(oracle.display().to_string());
        srcs.push(text);
    }
    Some(CaseWorld {
        label: case.to_owned(),
        paths,
        srcs,
        book,
        results: framed_for(dir),
    })
}

/// A loom-form case's world, over the sections `run_loom` would materialize.
fn loom_world(path: &Path, case: &str) -> Option<CaseWorld> {
    let text = std::fs::read_to_string(path).ok()?;
    let parsed = errorloom::Case::parse(&text).ok()?;
    let mut book = None;
    let mut results = None;
    let mut oracles: BTreeMap<String, String> = BTreeMap::new();
    for section in parsed.sections() {
        let name = section.name();
        if name == "book.sh" {
            book = Some(section.content().to_owned());
        } else if name == "probe-results.txt" {
            results = Some(section.content().to_owned());
        } else if name.ends_with(".oracle.sh") {
            oracles.insert(name.to_owned(), section.content().to_owned());
        }
    }
    let label = case.strip_suffix(".loom").unwrap_or(case).to_owned();
    let book = book?;
    // MATERIALIZE to re-frame: a loom case's records are sections, and `frame_records` reads the
    // committed file beside a book. Without this the whole-product looms — which are exactly the
    // MEASURED cases worth diffing — would all analyse as the unmeasured world.
    let framed = results.and_then(|records| {
        let scratch = std::env::temp_dir().join(format!("dorc-spine-baseline-{label}"));
        std::fs::create_dir_all(&scratch).ok()?;
        std::fs::write(scratch.join("book.sh"), &book).ok()?;
        std::fs::write(scratch.join("probe-results.txt"), &records).ok()?;
        for (name, text) in &oracles {
            std::fs::write(scratch.join(name), text).ok()?;
        }
        let framed = framed_for(&scratch);
        std::fs::remove_dir_all(&scratch).ok();
        framed
    });

    Some(CaseWorld {
        label,
        paths: oracles.keys().cloned().collect(),
        srcs: oracles.into_values().collect(),
        book,
        results: framed,
    })
}
