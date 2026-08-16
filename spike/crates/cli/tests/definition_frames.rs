//! The definition-factoring lane's own battery (`28Q` §1 `syn-definition-factored-indices`).
//!
//! Four properties live here, and the first is the precondition for the whole conversion. The other
//! three exist because the migration gate cannot see them: `syn-single-frame-byte-identical` asks
//! the corpus to be byte-stable, and today's corpus is very nearly single-frame, so it is silent
//! EXACTLY where the new machinery decides. These assert against the shells' own committed answers
//! and against an enumerated corpus census instead — neither of which a golden can go vacuous on.
//!
//! # The join census
//!
//! Derived rows (checks, cell declarations, argparse arm-models, footprint claims) are about to be
//! keyed by the `DefinitionId` that produced them, and that id carries a SPAN. But the span can
//! only come from one of the two parsers that read a source: `dorc_syntax::parse`, which feeds
//! [`dorc_cli::world::definition_table`] and therefore the function environment, or the DIALECT
//! parser, which produces the lifted rows. The two are known to disagree about which funcdefs
//! exist (`28O:fnd-two-parsers-disagree-on-funcdefs`), so the conversion joins them on the only
//! thing both spell identically — `(file, role name)` — and lets the span ride in from the
//! definition table alone.
//!
//! [`every_lifted_role_row_joins_to_a_parsed_definition`] is what makes that join safe to build
//! on. A lifted row with no parsed definition behind it would be an UNKEYABLE row: it can never
//! match a frame's answer, so under the conversion its site withholds — silently, corpus-wide, in
//! the withhold direction the byte-identity gate would catch only where the corpus happens to
//! license something. Measuring it first is cheaper than debugging it later.
//!
//! The census does NOT demand a bare zero, because the disagreeing class genuinely exists: a source
//! name that is not a legal sh NAME (`hork.tool`, `中pkg`) lifts a row under its MUNGED funcname
//! while `dorc_syntax` records the authored one, so the two never meet.
//!
//! What contains that class is `28P:dec-the-gate-applies-only-to-names-the-unit-knows` — the RULED
//! permissive arm: a name the definition table does not know has no positional opinion, so the gate
//! answers rather than manufacturing one. `oracle::reserved::lint_oracle_reserved_names` marks the
//! same population at Error severity, but that is a REPORT, not a refusal: `validate`'s stages reach
//! `report_at` and nothing else, and the run's fast-fail reads parse/cfg Errors plus
//! `wrapper_incoherent` only. So the fence here is the ruled arm, and the lint is the marker that
//! makes the arm's population nameable and this census's exception branch checkable.
//!
//! The census therefore demands the sharper thing: every unjoined row sits in a source the lint
//! marks. An unjoined row in an UNMARKED source is the lane-halting finding.

#![expect(
    clippy::print_stderr,
    reason = "`support`'s selection reporter is compiled into every test binary that uses it; this one drives no trials and never calls it"
)]

mod support;

use std::collections::BTreeSet;
use std::path::Path;

use support::case_roots;

/// Whether `name`'s extension is exactly `ext` — the `ends_with` cousin `clippy` insists on,
/// hoisted so the four call sites share one spelling.
fn has_extension(name: &str, ext: &str) -> bool {
    Path::new(name).extension().is_some_and(|got| got == ext)
}

/// One source text the corpus could hand the engine, and where it came from.
struct CorpusSource {
    /// A human-pointable label (`<case>/<file>`), for a failure message that names the file.
    label: String,
    /// The verbatim text.
    text: String,
}

/// Every `.sh` text in the committed corpus, dir-form and loom-form alike, in a deterministic
/// order (`inv-determinism`).
///
/// Deliberately WIDER than any single run's input set: the join is a per-FILE property, so
/// censusing every file independently is strictly stronger than censusing the combinations the
/// corpus happens to load.
fn corpus_sources() -> Vec<CorpusSource> {
    let mut out = Vec::new();
    for root in case_roots() {
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
                continue; // sync residue is never a case (`crates/cli/CLAUDE.md`)
            }
            if path.is_dir() {
                collect_dir_sources(&path, &name, &mut out);
            } else if has_extension(&name, "loom") {
                collect_loom_sources(&path, &name, &mut out);
            }
        }
    }
    out
}

/// The `.sh` files sitting directly in a dir-form case (its book and its oracles).
fn collect_dir_sources(dir: &Path, case: &str, out: &mut Vec<CorpusSource>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !has_extension(&name, "sh") || !path.is_file() {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            out.push(CorpusSource {
                label: format!("{case}/{name}"),
                text,
            });
        }
    }
    // A multi-file loom lives at `<case>/<case>.loom`.
    let inner = dir.join(format!("{case}.loom"));
    if inner.is_file() {
        collect_loom_sources(&inner, case, out);
    }
}

/// The `.sh` SECTIONS of a loom-form case — the same bytes `run_loom` would materialize.
fn collect_loom_sources(path: &Path, case: &str, out: &mut Vec<CorpusSource>) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    let Ok(parsed) = errorloom::Case::parse(&text) else {
        return; // a malformed loom is the looms runner's failure to report, not this one's
    };
    for section in parsed.sections() {
        if has_extension(section.name(), "sh") {
            out.push(CorpusSource {
                label: format!("{case}:{}", section.name()),
                text: section.content().to_owned(),
            });
        }
    }
}

/// The role funcdef names `dorc_syntax::parse` sees in `text` — exactly the filter
/// [`dorc_cli::world::definition_table`] applies when it records a `Definition`.
fn parsed_role_definitions(text: &str) -> BTreeSet<String> {
    use dorc_syntax::ast::NodeKind;

    let ast = dorc_syntax::parse(text).value;
    ast.iter()
        .filter_map(|(_, node)| match &node.kind {
            NodeKind::FuncDef { name, .. } => Some(name.clone()),
            _ => None,
        })
        .filter(|name| dorc_oracle::reserved::role_family(name).is_some())
        .collect()
}

/// The role funcdef names the DIALECT lift produced a keyable row for, reconstructed exactly as
/// every resolution seat reconstructs them (`map_provider_name` + the role suffix — the spelling
/// at `world::ship_predict_body`, `plan::build_vouches_from_sets`, and `main::ship_predict_stage`).
///
/// The kind-owner trio is deliberately absent: `vocabulary-acts-stay-ambient` keeps those rows off
/// the frame, so they are never keyed by a `DefinitionId` and cannot fail this join.
fn lifted_role_rows(text: &str) -> BTreeSet<String> {
    use dorc_oracle::predict::{PREDICT_SUFFIX, lift_predicts, map_provider_name};
    use dorc_oracle::touches::{DISTURBS_SUFFIX, TouchesSet};
    use dorc_oracle::verdict::{VERDICT_SUFFIX, VerdictSet};

    let mut interner = dorc_core::Interner::default();
    let mut out = BTreeSet::new();

    let predicts = lift_predicts(&mut interner, text).value;
    let named: Vec<_> = predicts.providers().collect();
    for provider in named {
        let base = map_provider_name(interner.resolve(provider));
        out.insert(format!("{base}{PREDICT_SUFFIX}"));
    }

    let verdicts = VerdictSet::lift(&mut interner, text).value;
    let named: Vec<_> = verdicts.providers().collect();
    for provider in named {
        let base = map_provider_name(interner.resolve(provider));
        out.insert(format!("{base}{VERDICT_SUFFIX}"));
    }

    let touches = TouchesSet::lift(&mut interner, text).value;
    let named: Vec<_> = touches.providers().collect();
    for provider in named {
        let base = map_provider_name(interner.resolve(provider));
        out.insert(format!("{base}{DISTURBS_SUFFIX}"));
    }

    out
}

/// Whether `oracle::reserved` MARKS this source at Error severity — the marker for the one class
/// where the two parsers name a funcdef differently (`28O:fnd-two-parsers-disagree-on-funcdefs`).
///
/// Deliberately not called a refusal: the mark reaches stderr and the run proceeds
/// (`307:fnd-reserved-name-error-does-not-refuse`). It is a nameable population, not a gate.
fn reserved_names_mark_an_error(text: &str) -> bool {
    let mut interner = dorc_core::Interner::default();
    dorc_oracle::reserved::lint_oracle_reserved_names(&mut interner, &[text])
        .iter()
        .any(|diag| diag.severity() == dorc_aid::Severity::Error)
}

/// THE STEP-ZERO CENSUS (`28Q` §1): every derived row the conversion will key by a
/// `DefinitionId` either has a parsed definition to take that id's span from, or lives in a source
/// the reserved-name lint marks at Error severity.
///
/// An unjoined row in an UNMARKED source is a LANE-HALTING finding, not a site to work around: it
/// can never match a frame's answer, so its site would withhold silently under the conversion, and
/// the `(file, role name)` join would need re-cutting rather than patching. The message names every
/// offender so the disagreement is diagnosable from one run.
#[test]
fn every_lifted_role_row_joins_to_a_parsed_definition() {
    let sources = corpus_sources();
    assert!(
        !sources.is_empty(),
        "discovery floor: the corpus walk found no sources, so this census proves nothing"
    );

    let mut unmarked: Vec<String> = Vec::new();
    let mut marked = 0usize;
    let mut rows = 0usize;
    for source in &sources {
        let parsed = parsed_role_definitions(&source.text);
        let lifted = lifted_role_rows(&source.text);
        rows += lifted.len();
        let unjoined: Vec<&String> = lifted.iter().filter(|row| !parsed.contains(*row)).collect();
        if unjoined.is_empty() {
            continue;
        }
        if reserved_names_mark_an_error(&source.text) {
            marked += unjoined.len();
            continue;
        }
        for row in unjoined {
            unmarked.push(format!("{}: {row}", source.label));
        }
    }

    assert!(
        rows > 0,
        "vacuity floor: {} sources yielded zero lifted role rows, so the join was never exercised",
        sources.len()
    );
    assert!(
        marked > 0,
        "vacuity floor: no corpus source carries a reserved-name Error, so this census's exception \
         branch proves nothing — the disagreeing class needs a live specimen"
    );
    assert!(
        unmarked.is_empty(),
        "{} lifted role row(s) in UNMARKED sources have no parsed definition to key against, over \
         {rows} row(s) in {} source(s) ({marked} row(s) sitting in reserved-name-marked \
         sources):\n  {}",
        unmarked.len(),
        sources.len(),
        unmarked.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// The load-set model, shared by the two batteries below.

/// One case's modeled world: the sources a run of it would load, in load order, plus its book.
///
/// The oracle set is the `*.oracle.sh` glob the e2e runner itself turns into `-o` arguments
/// (`shared_args`), so this is the runner's own rule rather than a second reading of it; the book is
/// a definition source too (`cli/CLAUDE.md the-book-is-a-definition-source`) and sits one past the
/// oracle vector exactly as `source_table` places it.
struct CaseWorld {
    /// The case's directory or loom name.
    label: String,
    /// The `-o` path strings, glob-sorted.
    paths: Vec<String>,
    /// Those files' texts, in the same order.
    srcs: Vec<String>,
    /// The book text.
    book: String,
}

/// Every committed case that has a book, with the sources a run of it would load.
fn corpus_worlds() -> Vec<CaseWorld> {
    let mut out = Vec::new();
    for root in case_roots() {
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
                continue;
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
            } else if has_extension(&name, "loom")
                && let Some(world) = loom_world(&path, &name)
            {
                out.push(world);
            }
        }
    }
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
    })
}

/// A loom-form case's world, over the sections `run_loom` would materialize.
fn loom_world(path: &Path, case: &str) -> Option<CaseWorld> {
    let text = std::fs::read_to_string(path).ok()?;
    let parsed = errorloom::Case::parse(&text).ok()?;
    let mut book = None;
    let mut oracles: Vec<(String, String)> = Vec::new();
    for section in parsed.sections() {
        let name = section.name();
        if name == "book.sh" {
            book = Some(section.content().to_owned());
        } else if name.ends_with(".oracle.sh") {
            oracles.push((name.to_owned(), section.content().to_owned()));
        }
    }
    oracles.sort();
    Some(CaseWorld {
        label: case.to_owned(),
        paths: oracles.iter().map(|(name, _)| name.clone()).collect(),
        srcs: oracles.into_iter().map(|(_, text)| text).collect(),
        book: book?,
    })
}

/// Parse, lower, and solve one world exactly as `cli::run` does, so what this file asserts is the
/// run's own environment and not a second model of it.
fn solve_world(
    world: &CaseWorld,
) -> (
    dorc_syntax::Ast,
    dorc_analysis::cfg::Cfg,
    dorc_analysis::funcenv::DefinitionTable,
    dorc_analysis::funcenv::FuncEnv,
) {
    let mut interner = dorc_core::Interner::default();
    let parsed = dorc_syntax::parse(&world.book).value;
    let cfg = dorc_analysis::cfg::build(&parsed).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed, &mut interner);
    let mut refs: Vec<&str> = world.srcs.iter().map(String::as_str).collect();
    refs.push(world.book.as_str());
    let defs = dorc_cli::world::definition_table(
        &world.paths,
        &refs,
        dorc_analysis::funcenv::source_file_of_index(world.srcs.len()),
        &parsed,
    );
    let env = {
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        dorc_analysis::funcenv::analyze(&parsed, &cfg, &defs, &plane)
    };
    (parsed, cfg, defs, env)
}

/// The families whose licenses a run of this world would WITHHOLD — the contested set the cli edge
/// mints, computed here by the same two calls in the same order (`28K` §1).
fn withheld_families(world: &CaseWorld) -> dorc_core::ContestedFamilies {
    let (parsed, cfg, defs, env) = solve_world(world);
    let shadows = dorc_analysis::funcenv::contests(&parsed, &cfg, &defs, &env);
    let unprovable = dorc_analysis::funcenv::unprovable(&defs, &env, cfg.exit());
    dorc_core::ContestedFamilies::new(
        shadows
            .iter()
            .map(|c| c.name.as_str())
            .chain(unprovable.iter().map(String::as_str))
            .filter_map(|name| {
                dorc_oracle::reserved::role_family(name).map(|(base, _)| base.to_owned())
            }),
    )
}

/// Every role name at least two of this world's loaded sources DECLARE — textual plurality, before
/// any withholding is considered.
fn plural_role_names(world: &CaseWorld) -> BTreeSet<String> {
    let mut once: BTreeSet<String> = BTreeSet::new();
    let mut again: BTreeSet<String> = BTreeSet::new();
    for text in world.srcs.iter().chain(std::iter::once(&world.book)) {
        for name in parsed_role_definitions(text) {
            if !once.insert(name.clone()) {
                again.insert(name);
            }
        }
    }
    again
}

/// The cases that deliberately carry a plural-definition idiom the environment BLESSES.
///
/// Both are `28K` §1's sanctioned shapes rather than shadow collisions, which is exactly why the
/// contested withdrawal leaves them alone and the frame lookup has to answer them: a define-if-absent
/// polyfill whose guard the decidable-condition fold proves dead (`28M` §9), and an `unset -f` above
/// a redefinition, the blessing that makes an override an override instead of a contest. They are
/// where today's corpus exercises the plural arm with licenses intact.
///
/// This measurement REFINES `307:fnd-corpus-carries-twelve-plural-families`, which counted the
/// twelve textual cases and the five the withdrawal holds byte-stable but did not separate the
/// blessed remainder.
///
/// A case appearing UNCOVERED here must be REVIEWED, never listed reflexively: an unblessed plural
/// family reaching the resolution seats with licenses intact is precisely the world the
/// byte-identity gate cannot see into.
const PLURAL_IDIOM_CASES: &[&str] = &[
    "contest28-polyfill-guard-defers-to-the-oracle.loom",
    "contest28-unset-f-blesses-elision.loom",
];

/// THE PLURALITY CENSUS (`305a` §1): every REACHABLE plural family in the committed corpus sits in
/// [`PLURAL_IDIOM_CASES`].
///
/// "Reachable" is load-set-modeled and withholding-aware: a family two files declare but only one
/// run loads is not plural in any run, and one the contested withdrawal withholds licenses nothing
/// whichever definition would have answered. What is left is the population where the frame lookup
/// genuinely decides, and the point of enumerating it is that such a case must be a DELIBERATE
/// mint — a new one arriving silently is how the byte-identity gate goes vacuous without saying so.
#[test]
fn every_reachable_plural_family_is_an_enumerated_plural_idiom() {
    let worlds = corpus_worlds();
    assert!(
        !worlds.is_empty(),
        "discovery floor: the corpus walk found no worlds, so this census proves nothing"
    );

    let mut textual = 0usize;
    let mut covered: BTreeSet<&str> = BTreeSet::new();
    let mut unlisted: Vec<String> = Vec::new();
    for world in &worlds {
        let plural = plural_role_names(world);
        if plural.is_empty() {
            continue;
        }
        textual += 1;
        let withheld = withheld_families(world);
        let reachable: Vec<&String> = plural
            .iter()
            .filter(|name| {
                dorc_oracle::reserved::role_family(name)
                    .is_none_or(|(base, _)| !withheld.withholds(base))
            })
            .collect();
        if reachable.is_empty() {
            continue;
        }
        if let Some(listed) = PLURAL_IDIOM_CASES
            .iter()
            .find(|listed| **listed == world.label)
        {
            covered.insert(listed);
            continue;
        }
        for name in reachable {
            unlisted.push(format!("{}: {name}", world.label));
        }
    }

    assert!(
        textual > 0,
        "vacuity floor: no world carries a textually plural role family, so the census's \
         withholding filter was never exercised"
    );
    assert!(
        unlisted.is_empty(),
        "{} plural role family/families reach the resolution seats with licenses intact and are \
         not enumerated plural idioms:\n  {}",
        unlisted.len(),
        unlisted.join("\n  ")
    );
    // Two-way, like every allow-list in this tree: a listed case that stopped carrying a reachable
    // plural family has stopped covering the arm it was listed for, and the list must say so.
    let stale: Vec<&&str> = PLURAL_IDIOM_CASES
        .iter()
        .filter(|listed| !covered.contains(**listed))
        .collect();
    assert!(
        stale.is_empty(),
        "enumerated plural-idiom case(s) no longer carry a reachable plural family: {stale:?}"
    );
}

// ---------------------------------------------------------------------------
// The frame differential: the engine's answer against the shells' committed one.

/// The role every floor30 cell measures. One name across the battery, because the cells differ in
/// FRAME SHAPE and not in vocabulary.
const FLOOR_ROLE: &str = "hork__is_converged";

/// The emitted token a cell writes when NOTHING is live — the `||` tail speaking.
const NOTHING_LIVE: &str = "gone";

/// One differential cell: the loom's sourced sections as inputs under the exact path strings its
/// book spells, its book, and the answers the real shells gave.
struct FloorCell {
    /// The case name, for a failure that names the file.
    name: String,
    /// `(the path the book sources, that file's text)`, in load order.
    inputs: Vec<(String, String)>,
    /// The book text.
    book: String,
    /// The committed `expected.emitted` lines — dash∩posh's own answers, measured once and never
    /// churned (`spike/CLAUDE.md emitted-is-measure-once-ground-truth`).
    emitted: Vec<String>,
}

/// Every `floor30-*` cell, with its sections read back as a world.
///
/// The inputs are named `./<section>` because that is how each book sources them: the engine binds
/// a `.` target by the path STRING the load edge registered, so the differential must hand it the
/// strings the book spells or nothing resolves and every cell would pass vacuously — which is what
/// the per-cell resolution floor below refuses.
fn floor_cells() -> Vec<FloorCell> {
    let mut out = Vec::new();
    for root in case_roots() {
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
            if !name.starts_with("floor30-") || !has_extension(&name, "loom") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Ok(parsed) = errorloom::Case::parse(&text) else {
                continue;
            };
            let mut book = None;
            let mut emitted = Vec::new();
            let mut inputs = Vec::new();
            for section in parsed.sections() {
                match section.name() {
                    "book.sh" => book = Some(section.content().to_owned()),
                    "expected.emitted" => {
                        emitted = section
                            .content()
                            .lines()
                            .map(str::trim)
                            .filter(|line| !line.is_empty())
                            .map(str::to_owned)
                            .collect();
                    }
                    other if has_extension(other, "sh") => {
                        inputs.push((format!("./{other}"), section.content().to_owned()));
                    }
                    _ => {}
                }
            }
            inputs.sort();
            if let Some(book) = book {
                out.push(FloorCell {
                    name,
                    inputs,
                    book,
                    emitted,
                });
            }
        }
    }
    out
}

/// The input whose body prints `token` — the file the shells were running when they emitted it.
///
/// Mechanical rather than hand-mapped: each floor30 body's whole content is one `printf` of a
/// token unique to it, so the emitted line IDENTIFIES its author, and a rename or a re-authored
/// body cannot silently drift the mapping the way a committed table would.
fn author_of(cell: &FloorCell, token: &str) -> Option<usize> {
    let needle = format!("printf '{token}\\n'");
    let mut found = None;
    for (index, (_, text)) in cell.inputs.iter().enumerate() {
        if text.contains(&needle) {
            assert!(
                found.is_none(),
                "{}: two inputs print {token:?}, so the emitted line names no single author",
                cell.name
            );
            found = Some(index);
        }
    }
    found
}

/// The book's calls to [`FLOOR_ROLE`], in the order a shell reaches them.
fn role_call_sites(
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    interner: &dorc_core::Interner,
) -> Vec<dorc_analysis::cfg::CfgNodeId> {
    use dorc_analysis::value::ValueOf;
    cfg.iter()
        .filter(|(_, node)| node.kind == dorc_analysis::cfg::CfgNodeKind::Command)
        .filter(|(id, _)| {
            matches!(
                value.argv_values(*id).first(),
                Some(ValueOf::Literal(word)) if interner.resolve(*word) == FLOOR_ROLE
            )
        })
        .map(|(id, _)| id)
        .collect()
}

/// THE FRAME DIFFERENTIAL (`28Q` §8 stage-i): at every site a floor30 cell measured, the definition
/// the engine names is the one whose body the real shells ran.
///
/// This is the engine-agreement half the fixtures were minted for. The committed `expected.emitted`
/// lines are dash∩posh's own answers — measured once, never churned — so the assertion is against
/// shell behaviour rather than against the engine's previous opinion, which is what a golden would
/// have given. The cells the battery covers are the ones where the ROLE ITSELF is redefined across
/// frames; the two whose redefinition is a HELPER are covered by
/// [`a_contested_helper_closure_withholds_the_role_body`] instead, because a helper name is not a
/// role and the definition table holds no opinion about it.
///
/// `gone` asserts the negative half: after `unset -f` the environment must name NOTHING. Reading
/// that as "some definition, we cannot say which" is the failure this direction exists to catch —
/// a removed binding that still answered would license off a body no shell would call.
#[test]
fn the_engine_names_the_definition_the_shells_ran() {
    let cells = floor_cells();
    assert!(
        !cells.is_empty(),
        "discovery floor: no floor30-* cell was found, so the differential proves nothing"
    );

    let mut role_cells = 0usize;
    let mut sites_checked = 0usize;
    for cell in &cells {
        // A cell whose inputs redefine the ROLE is this battery's; one that redefines a helper
        // belongs to the closure floor's.
        let redefiners = cell
            .inputs
            .iter()
            .filter(|(_, text)| parsed_role_definitions(text).contains(FLOOR_ROLE))
            .count();
        if redefiners < 2 {
            continue;
        }
        role_cells += 1;

        let world = CaseWorld {
            label: cell.name.clone(),
            paths: cell.inputs.iter().map(|(path, _)| path.clone()).collect(),
            srcs: cell.inputs.iter().map(|(_, text)| text.clone()).collect(),
            book: cell.book.clone(),
        };
        let mut interner = dorc_core::Interner::default();
        let parsed = dorc_syntax::parse(&world.book).value;
        let cfg = dorc_analysis::cfg::build(&parsed).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed, &mut interner);
        let (_, _, defs, env) = solve_world(&world);
        let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &defs);

        let sites = role_call_sites(&cfg, &value, &interner);
        assert_eq!(
            sites.len(),
            cell.emitted.len(),
            "{}: the book calls {FLOOR_ROLE} {} time(s) but committed {} measurement(s); the \
             differential cannot pair them",
            cell.name,
            sites.len(),
            cell.emitted.len()
        );

        for (site, token) in sites.iter().zip(&cell.emitted) {
            let named = live.source_before(*site, FLOOR_ROLE);
            if token == NOTHING_LIVE {
                assert_eq!(
                    named, None,
                    "{}: the shells found {FLOOR_ROLE} absent here, so the environment must name \
                     no definition",
                    cell.name
                );
                sites_checked += 1;
                continue;
            }
            let want = author_of(cell, token).unwrap_or_else(|| {
                panic!("{}: no input prints {token:?}", cell.name);
            });
            assert_eq!(
                named,
                Some(dorc_analysis::funcenv::source_file_of_index(want)),
                "{}: the shells ran {}'s body here (it emitted {token:?}), but the environment \
                 names {named:?}",
                cell.name,
                cell.inputs[want].0
            );
            sites_checked += 1;
        }
    }

    assert!(
        role_cells >= 3,
        "coverage floor: only {role_cells} floor30 cell(s) redefine the role itself; the battery \
         was minted with three"
    );
    assert!(
        sites_checked >= 10,
        "coverage floor: only {sites_checked} site(s) were paired against a committed measurement"
    );
}

/// The closure WITHHOLD floor (`28Q` §1.1): where one HELPER name holds differing bodies across
/// frames, the loaded set refuses to pin a closure, so the role body ships nowhere.
///
/// Stage-i's ruled position for the helper cells. sh binds a body's calls at INVOCATION, so a
/// closure is a property of the consuming FRAME and not of the definition — "computed once,
/// whole-unit" is dead for closures (`28R:rul-resolution-matches-shell-loading`), and until the
/// snapshot-transplant emission lands the honest answer is to withhold rather than to ship one
/// frame's helper into another's. The floor30 helper cells are exactly that world, and this asserts
/// the refusal fires at the seat that must feel it: `closure_for` on the role's OWN body.
#[test]
fn a_contested_helper_closure_withholds_the_role_body() {
    let cells = floor_cells();
    let mut helper_cells = 0usize;
    for cell in &cells {
        let role_file = cell
            .inputs
            .iter()
            .position(|(_, text)| parsed_role_definitions(text).contains(FLOOR_ROLE));
        let Some(role_file) = role_file else { continue };
        let redefiners = cell
            .inputs
            .iter()
            .filter(|(_, text)| parsed_role_definitions(text).contains(FLOOR_ROLE))
            .count();
        if redefiners != 1 {
            continue; // a role-redefinition cell — the differential above owns it
        }
        helper_cells += 1;

        let refs: Vec<&str> = cell.inputs.iter().map(|(_, text)| text.as_str()).collect();
        let helpers = dorc_oracle::closure::HelperIndex::build(&refs);
        assert!(
            !helpers.conflicts().is_empty(),
            "{}: the loaded set declares one helper name with differing bodies, so the load edge \
             owes a helper-declaration-contested",
            cell.name
        );
        let body = &cell.inputs[role_file].1;
        assert!(
            helpers.closure_for(role_file, body).is_err(),
            "{}: the role body reaches the contested helper, so pinning its closure must refuse — \
             shipping it would carry ONE frame's helper into every frame",
            cell.name
        );
    }
    assert!(
        helper_cells >= 2,
        "coverage floor: only {helper_cells} floor30 cell(s) redefine a helper rather than the \
         role; the battery was minted with two"
    );
}
