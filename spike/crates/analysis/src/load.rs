//! `analysis::load` — a loadable file's own top level, as the closed program the loader evaluates
//! (`30I:rul-static-loading-is-the-whole-model`).
//!
//! Before this, a `.` bound "the definitions of that path" as a flat list. That is only a total
//! model of a file whose top level IS a flat list — and the healthy shared-library shape is not:
//! it opens with an include guard that decides whether a dependency loads at all
//! (`30I:rul-include-guards-are-load-semantics`, TYPED). So a loadable file carries a PROGRAM, and
//! the function-environment domain interprets it in place at each load site.
//!
//! # A closed vocabulary, not an interpreter
//!
//! The steps below are the whole of `30I:rul-oracle-loading-stays-load-safe`'s positive surface:
//! declarations, known-value assignments, known-value load operands, one include-guard shape, and
//! `unset -f`. Nothing here evaluates arbitrary shell, and the admission gate
//! (`dorc_oracle::load_inert`) is what guarantees no other construct reaches this type. A file
//! whose top level falls outside that gate never becomes a program at all: it is not admitted, its
//! sourcer suspends, and the site walls.
//!
//! # The operand is a TEMPLATE, because the root lives in the caller
//!
//! `30I` §2.1's ordinary idiom assigns a root in the book and reads it inside the package, so a
//! package's own `.` operand cannot be resolved when the package is READ — only when it is LOADED,
//! against the variables live at that point. [`LoadTarget`] keeps the operand unexpanded for
//! exactly that reason, and the engine recognizes no root variable name: any ordinary variable
//! does the same work.

use std::collections::{BTreeMap, BTreeSet};

use crate::funcenv::DefId;

/// ONE account of every statically possible resolved load occurrence, from which consumers derive
/// three non-interchangeable projections (`30I:rul-one-load-account-separate-projections`).
///
/// The loader resolves each occurrence exactly once and keeps enough of it — sourcer, target,
/// locus, positional context, nesting — that no consumer has to re-parse a source or re-resolve a
/// target. A second resolver is what `30I:rul-one-loader-many-projections` forbids, and a
/// target-only pair set is what it cannot be replaced by: distinct textual load points naming one
/// entrypoint are distinct occurrences, and both bundle keying (`rul-bundles-key-to-load-occurrences`)
/// and locator composition need them kept apart.
///
/// The three projections, and why none of them substitutes for another:
///
/// 1. **possible-load** ([`occurrences`](LoadAccount::occurrences)) — every occurrence the walk
///    resolved, an undecided guard's fallback branch INCLUDED. The conservative union a bundle
///    consumes, so an artifact never omits a file the runtime `.` may load.
/// 2. **speaker** ([`speaker_edges`](LoadAccount::speaker_edges)) — only the occurrences whose
///    exact custody proof succeeded. Vouch composition and every other authority consumer see this
///    one and no other.
/// 3. **narrative** ([`selection_edges`](LoadAccount::selection_edges)) — which file's author
///    SELECTED which dependency at all, whether or not the selection aligned. Decision-inert, and
///    what `30I` §3.4 reads to tell a source act that failed to align from an ambient resolution
///    nobody selected.
///
/// Absence from the speaker projection never means absence from the other two.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoadAccount {
    occurrences: Vec<LoadOccurrence>,
    wanted: BTreeSet<String>,
    unresolved: BTreeSet<String>,
}

/// Which file spelled a load act.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadSourcer {
    /// A CLI-named pre-source root (`30I:rul-pre-source-is-dot-prelude`). Ingestion: it composes no
    /// custody, so it mints no speaker edge and selects nothing on anyone's behalf.
    Invocation,
    /// The main book's own `.`. Visibility only (`30I:rul-books-load-but-do-not-speak`).
    Book,
    /// A loaded file's own program spelled the `.`, by canonical key.
    File(String),
}

/// How the modeled shell reaches a load occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadRoute {
    /// The engine can say this `.` runs.
    Taken,
    /// A speculative branch of a guard nobody decided: the runtime `.` may or may not run. A bundle
    /// must carry the target anyway; no authority may rest on it
    /// (`rul-speaker-minting-is-oracle-sourcing-only`).
    Speculative,
    /// A recognized package sentinel's fallback whose REUSE arm the environment selected: this `.`
    /// provably does not run here, and the same exact target is live from another act. It mints its
    /// author's speaker edge all the same (`30I:rul-guarded-source-mints-exact-speaker-edge`).
    Reused,
}

/// One statically possible resolved load occurrence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOccurrence {
    /// Who spelled the act.
    pub sourcer: LoadSourcer,
    /// The canonical key the operand resolved to.
    pub target: String,
    /// The `. <operand>` item's own byte range inside the SOURCER's file — what a locator names
    /// when it says which line brought a dependency in. `None` at a root act, which is spelled on
    /// the command line or read off the book's CFG rather than out of a load program.
    pub locus: Option<dorc_core::Span>,
    /// The book program point the whole load act descends from — the positional context a bundle
    /// placed at its original source point needs.
    pub at: crate::cfg::CfgNodeId,
    /// The enclosing occurrence, for a nested `.`; `None` at a root act. Index into
    /// [`LoadAccount::occurrences`].
    pub within: Option<usize>,
    /// How the modeled shell reaches it.
    pub route: LoadRoute,
}

impl LoadAccount {
    /// THE POSSIBLE-LOAD PROJECTION: every occurrence, in walk order, speculative branches included.
    #[must_use]
    pub fn occurrences(&self) -> &[LoadOccurrence] {
        &self.occurrences
    }

    /// Canonical paths a load NAMED that the table does not hold — the acquisition loop's whole
    /// input, and what makes the engine that decides a package's dependencies the same engine that
    /// reads them.
    #[must_use]
    pub fn wanted(&self) -> &BTreeSet<String> {
        &self.wanted
    }

    /// Loaded files whose own load named nothing the table holds. They SUSPEND: a file whose
    /// environment the engine could not reconstruct may ship no composition.
    #[must_use]
    pub fn unresolved(&self) -> &BTreeSet<String> {
        &self.unresolved
    }

    /// THE SPEAKER PROJECTION: `(sourcer, target)` for the occurrences whose exact custody proof
    /// succeeded — a loaded file's own act, on a route the engine can say happened.
    ///
    /// A book `.` and an invocation-named root contribute none, by the type: neither is a
    /// [`LoadSourcer::File`].
    #[must_use]
    pub fn speaker_edges(&self) -> BTreeSet<(String, String)> {
        self.edges(|route| route != LoadRoute::Speculative)
    }

    /// THE NARRATIVE PROJECTION: `(sourcer, target)` for every occurrence a loaded file's own
    /// program spelled, whatever route it sits on.
    ///
    /// Decision-inert, and deliberately WIDER than [`speaker_edges`](Self::speaker_edges): an
    /// author who guarded a dependency the engine could not decide still SELECTED it, and `30I`
    /// §3.4 owes them a different sentence from one who selected nothing at all.
    #[must_use]
    pub fn selection_edges(&self) -> BTreeSet<(String, String)> {
        self.edges(|_| true)
    }

    fn edges(&self, admit: impl Fn(LoadRoute) -> bool) -> BTreeSet<(String, String)> {
        self.occurrences
            .iter()
            .filter(|occurrence| admit(occurrence.route))
            .filter_map(|occurrence| match &occurrence.sourcer {
                LoadSourcer::File(key) => Some((key.clone(), occurrence.target.clone())),
                LoadSourcer::Invocation | LoadSourcer::Book => None,
            })
            .collect()
    }

    /// Record one occurrence, answering its index so a nested one can name it.
    ///
    /// The four mutators are `pub(crate)` deliberately: outside `analysis` the account is a
    /// READ-ONLY answer, so no consumer can forge an occurrence into the loader's own report.
    pub(crate) fn record(&mut self, occurrence: LoadOccurrence) -> usize {
        self.occurrences.push(occurrence);
        self.occurrences.len().saturating_sub(1)
    }

    /// Note a canonical path a load named that the table does not hold.
    pub(crate) fn want(&mut self, key: String) {
        self.wanted.insert(key);
    }

    /// Note a sourcer whose own load named nothing loadable.
    pub(crate) fn suspend(&mut self, sourcer: String) {
        self.unresolved.insert(sourcer);
    }

    /// Seed the wanted set from the sites whose target the table never held.
    pub(crate) fn want_all(&mut self, keys: impl IntoIterator<Item = String>) {
        self.wanted.extend(keys);
    }
}

/// One loadable file's top level, in source order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoadProgram {
    steps: Vec<LoadStep>,
}

impl LoadProgram {
    /// Build from steps in source order.
    #[must_use]
    pub fn of(steps: Vec<LoadStep>) -> Self {
        Self { steps }
    }

    /// The steps, in source order.
    #[must_use]
    pub fn steps(&self) -> &[LoadStep] {
        &self.steps
    }

    /// Every definition this file declares, in file order — the flat view a consumer that only
    /// needs "what does this file declare" reads.
    ///
    /// Guard branches contribute NONE, and cannot: [`LoadControl`] has no declaring variant.
    #[must_use]
    pub fn declarations(&self) -> Vec<DefId> {
        self.steps
            .iter()
            .filter_map(|step| match step {
                LoadStep::Define(def) => Some(*def),
                LoadStep::Assign { .. } | LoadStep::Control(_) => None,
            })
            .collect()
    }

    /// Does this file's top level assign `name`? A NAME question, never a value one — the sentinel
    /// recognition asks who could have populated a value, not what it is (`30I` §3.4).
    ///
    /// Top level only, which is exact: [`LoadControl`] has no assigning variant, so a guard branch
    /// cannot assign and there is nowhere else for an assignment to hide.
    #[must_use]
    pub fn assigns(&self, name: &str) -> bool {
        self.steps.iter().any(|step| match step {
            LoadStep::Assign { name: assigned, .. } => assigned == name,
            LoadStep::Define(_) | LoadStep::Control(_) => false,
        })
    }

    /// The LAST wholly-literal value this file's top level assigns to `name`, or `None` when there
    /// is none or the winning assignment reads a variable.
    ///
    /// The VALUE half of the sentinel comparison, which [`assigns`](Self::assigns) deliberately
    /// does not answer: `30I:rul-load-semantics-stay-full-fidelity` keeps the live constant and the
    /// compared literal in the FULL load model because a package assigning `v1` under a guard
    /// testing `v2` is sourced again by a real shell. The lossy speech projection
    /// (`rul-guarded-source-speech-is-lossy`) still asks the NAME question and must never gain this
    /// one.
    ///
    /// Wholly literal, because a value that reads the loading context is a value this seat cannot
    /// read without becoming a second load interpreter — and a value the loader cannot read decides
    /// nothing.
    #[must_use]
    pub fn last_literal_assignment(&self, name: &str) -> Option<String> {
        self.steps.iter().rev().find_map(|step| match step {
            LoadStep::Assign {
                name: assigned,
                value,
            } if assigned == name => Some(value.expand(&BTreeMap::new(), &|_| None)),
            LoadStep::Assign { .. } | LoadStep::Define(_) | LoadStep::Control(_) => None,
        })?
    }

    /// Does this file's top level `unset -f` any of `names`, in a guard branch or out of one?
    #[must_use]
    pub fn removes_any(&self, names: &BTreeSet<&str>) -> bool {
        fn walk(controls: &[LoadControl], names: &BTreeSet<&str>) -> bool {
            controls.iter().any(|control| match control {
                LoadControl::UnsetFunctions(removed) => {
                    removed.iter().any(|name| names.contains(name.as_str()))
                }
                LoadControl::Guard { then_, else_, .. } => walk(then_, names) || walk(else_, names),
                LoadControl::Load { .. } => false,
            })
        }
        self.steps.iter().any(|step| match step {
            LoadStep::Control(control) => walk(std::slice::from_ref(control), names),
            LoadStep::Define(_) | LoadStep::Assign { .. } => false,
        })
    }

    /// Every load operand this file spells, guard branches included, in source order — what an
    /// acquisition or a custody edge asks for, before any branch has been decided.
    #[must_use]
    pub fn load_targets(&self) -> Vec<(&LoadTarget, dorc_core::Span)> {
        fn walk<'a>(controls: &'a [LoadControl], out: &mut Vec<(&'a LoadTarget, dorc_core::Span)>) {
            for control in controls {
                match control {
                    LoadControl::Load { target, span } => out.push((target, *span)),
                    LoadControl::Guard { then_, else_, .. } => {
                        walk(then_, out);
                        walk(else_, out);
                    }
                    LoadControl::UnsetFunctions(_) => {}
                }
            }
        }
        let mut out = Vec::new();
        for step in &self.steps {
            if let LoadStep::Control(control) = step {
                walk(std::slice::from_ref(control), &mut out);
            }
        }
        out
    }
}

/// One step of a loadable file's top level.
///
/// Declaring is a TOP-LEVEL act and appears only here — [`LoadControl`] carries what a guard's
/// branches may hold, and it cannot declare. See that type for the measured reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadStep {
    /// A function definition: binds a name, runs nothing.
    Define(DefId),
    /// A bare assignment whose value expands without running anything. Kept because a package may
    /// site its own dependency through a constant it sets itself.
    Assign {
        /// The variable name.
        name: String,
        /// Its value, which may itself read variables from the loading context.
        value: LoadTarget,
    },
    /// Load control: a `.`, a removal, or a guard over either.
    Control(LoadControl),
}

/// What a guard's branches may hold — and, by inclusion, what a top level may hold besides
/// declaring.
///
/// **This type is why a guard branch cannot declare.** A role funcdef inside a conditional branch
/// is a measured wrong-elision route: the dialect lift recognizes a role header only as a
/// TOP-LEVEL ITEM, so a nested definition is registered by the definition table while producing
/// ZERO lifted rows — described nowhere, detected nowhere, and licensing off a body the lift never
/// read (`oracle/CLAUDE.md only-load-inert-sources-contribute`; pinned by `sh_parity.rs`'s
/// `a_host_conditional_oracle_definition_licenses_nothing` and its expected-fail twin).
///
/// The admission gate refuses that shape too. Having BOTH is deliberate: the gate is what an
/// author is told, and this is what the loader can even be handed, so a future builder cannot
/// re-open the route by widening the gate alone.
///
/// A branch that deliberately does nothing (`then :`) is the EMPTY vector — there is no no-op
/// step, because a no-op that had to be represented could be mistaken for one that acts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadControl {
    /// `unset -f NAME…` — the removal half of the load surface.
    UnsetFunctions(Vec<String>),
    /// A `.` of the named target, at its own byte range in the file that spells it — the span a
    /// locator names when it says which line brought a dependency in.
    Load {
        /// The operand, unexpanded.
        target: LoadTarget,
        /// The whole `. <operand>` item.
        span: dorc_core::Span,
    },
    /// The include guard, selecting between two branches.
    Guard {
        /// What the guard asks.
        condition: LoadCondition,
        /// Whether the condition is `!`-negated.
        negated: bool,
        /// Taken when the condition succeeds.
        then_: Vec<LoadControl>,
        /// Taken when it fails.
        else_: Vec<LoadControl>,
    },
}

/// What an include guard asks (`30I` §2.2). The admission gate's own vocabulary is
/// `dorc_oracle::load_inert::GuardCondition`; this is the loader's copy of it, so the loading
/// domain does not have to reach into the gate to interpret a program it was handed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCondition {
    /// `command -v <function>` — what a shell would resolve under that name.
    CommandV {
        /// The function the guard asks about.
        function: String,
    },
    /// `[ "${name-}" = 'literal' ]` — the package sentinel.
    Value {
        /// The variable the test reads.
        name: String,
        /// The literal it is compared against.
        literal: String,
        /// `=` rather than `!=`.
        equals: bool,
    },
}

/// A load operand, kept unexpanded until the file is loaded.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoadTarget {
    parts: Vec<TargetPart>,
}

/// One fragment of a load operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetPart {
    /// Literal text.
    Literal(String),
    /// A variable read, resolved against the loading context.
    Param(String),
}

impl LoadTarget {
    /// Build from fragments in source order.
    #[must_use]
    pub fn of(parts: Vec<TargetPart>) -> Self {
        Self { parts }
    }

    /// A wholly literal operand.
    #[must_use]
    pub fn literal(text: impl Into<String>) -> Self {
        Self {
            parts: vec![TargetPart::Literal(text.into())],
        }
    }

    /// The operand as text, with every variable resolved through `lookup` — `locals` first, since
    /// a package that sets its own constant reads its own value.
    ///
    /// `None` the moment one fragment cannot be resolved: an operand the engine cannot read is an
    /// unresolvable load, never a guessed file (`30I` §3.2).
    #[must_use]
    pub fn expand(
        &self,
        locals: &BTreeMap<String, String>,
        lookup: &impl Fn(&str) -> Option<String>,
    ) -> Option<String> {
        let mut out = String::new();
        for part in &self.parts {
            match part {
                TargetPart::Literal(text) => out.push_str(text),
                TargetPart::Param(name) => match locals.get(name) {
                    Some(text) => out.push_str(text),
                    None => out.push_str(&lookup(name)?),
                },
            }
        }
        Some(out)
    }

    /// The operand's fragments, for a consumer that must render or re-spell it.
    #[must_use]
    pub fn parts(&self) -> &[TargetPart] {
        &self.parts
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{LoadCondition, LoadControl, LoadProgram, LoadStep, LoadTarget, TargetPart};
    use crate::funcenv::DefId;

    fn nowhere() -> dorc_core::Span {
        dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(0))
    }

    fn rooted() -> LoadTarget {
        LoadTarget::of(vec![
            TargetPart::Param("OPS_LIB".to_owned()),
            TargetPart::Literal("/common.dorc.sh".to_owned()),
        ])
    }

    /// The whole point of keeping the operand unexpanded: the root is assigned by the CALLER, so
    /// the same package text names different files under different loading contexts
    /// (`30I:force-root-value-flow`). No variable name is recognized — `OPS_LIB` here does exactly
    /// what the spike's `SM_ORACLE_ROOT` mnemonic does.
    #[test]
    fn an_operand_expands_against_the_loading_context() {
        let empty = BTreeMap::new();
        assert_eq!(
            rooted().expand(&empty, &|name| (name == "OPS_LIB")
                .then(|| "./oracles".to_owned())),
            Some("./oracles/common.dorc.sh".to_owned())
        );
        assert_eq!(
            rooted().expand(&empty, &|_| None),
            None,
            "a root the context cannot name is an unresolvable load, never a guessed file"
        );
    }

    /// A package's own constant wins over the caller's — it is the value a shell would have live
    /// once the assignment has run.
    #[test]
    fn a_file_local_constant_shadows_the_loading_context() {
        let locals = BTreeMap::from([("OPS_LIB".to_owned(), "./vendored".to_owned())]);
        assert_eq!(
            rooted().expand(&locals, &|_| Some("./oracles".to_owned())),
            Some("./vendored/common.dorc.sh".to_owned())
        );
    }

    /// Declarations are TOP-LEVEL only and loads are found through guards — the two halves every
    /// consumer splits on, and the asymmetry the TYPES create: a guard's branches are
    /// [`LoadControl`], which has no declaring variant, so `declarations()` cannot miss one and a
    /// branch cannot hide one.
    #[test]
    fn declarations_are_flat_and_loads_are_not() {
        let program = LoadProgram::of(vec![
            LoadStep::Control(LoadControl::Guard {
                condition: LoadCondition::CommandV {
                    function: "_q".to_owned(),
                },
                negated: false,
                then_: Vec::new(),
                else_: vec![LoadControl::Load {
                    target: LoadTarget::literal("./common.sh"),
                    span: nowhere(),
                }],
            }),
            LoadStep::Define(DefId(7)),
        ]);
        assert_eq!(program.declarations(), vec![DefId(7)]);
        assert_eq!(program.load_targets().len(), 1, "found inside the guard");
    }
}
