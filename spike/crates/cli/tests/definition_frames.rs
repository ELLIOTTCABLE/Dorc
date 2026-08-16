//! The definition-factoring lane's own battery (`28Q` §1 `syn-definition-factored-indices`).
//!
//! Two properties live here, and the first is the precondition for the whole conversion.
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
