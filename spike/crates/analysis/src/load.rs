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

use std::collections::BTreeMap;

use crate::funcenv::DefId;

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
    /// The include guard: `command -v <function>` selecting between two branches.
    Guard {
        /// The function the guard asks about.
        function: String,
        /// Whether the condition is `!`-negated.
        negated: bool,
        /// Taken when the condition succeeds.
        then_: Vec<LoadControl>,
        /// Taken when it fails.
        else_: Vec<LoadControl>,
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

    use super::{LoadControl, LoadProgram, LoadStep, LoadTarget, TargetPart};
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
                function: "_q".to_owned(),
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
