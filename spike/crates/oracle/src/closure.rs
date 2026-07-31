//! `closure` — what a pinned role definition needs BESIDES itself (`28K` §4
//! `rul-pin-by-definition-bytes`).
//!
//! A shipped role body is not self-contained. It may call helper functions and read file-level
//! constants its author declared beside it, and neither travels with the funcdef span. Shipping the
//! body alone therefore ships a body whose helpers resolve to nothing on the host — measured on this
//! tree before the pass existed: a verdict body calling `_wombat_check "$1"` lifted cleanly, shipped
//! alone, and would have answered rc 127 at the host. That is usually the safe direction (a
//! non-zero verdict declines and the site runs), but it is not reliably so: a body that IGNORES a
//! helper's status and answers 0 from a later test reports converged off a helper that never ran,
//! which is the priority-1 under-execute (`execution-priority-order`).
//!
//! The unit this pass emits is therefore the definition PLUS its closure, and the closure's shape
//! follows `28M` §8's overlay riders: helpers are resolved across the whole loaded source set, not
//! only the defining file, because a well-engineered oracle package is expected to split its bulk
//! logic into a helpers file and keep one thin entrypoints file (`28M` §7
//! `tune-explicit-composition-is-sanctioned` — explicitly-spelled composition is sanctioned; only
//! implicit engine-owned merging is refused).
//!
//! # Two rules worth defending
//!
//! **Conflicting definitions REFUSE rather than resolve** ([`ClosureRefusal`]). sh itself would take
//! the last-loaded definition of a helper name and rebind an author's helper out from under them
//! silently. `28M` §8's diamond rider says unit-identity keys to the DEFINING file, so
//! version-skewed vendored copies must refuse rather than dedup; byte-IDENTICAL copies are the
//! common vendoring case and dedup to one emission. The refusal withholds the pin, which withholds
//! the vouch and the ship, which runs the site — the safe direction, and loud
//! (`inv-top-reject`).
//!
//! **Constants ride per CONTRIBUTING FILE, not per reference.** The lexer collapses every
//! parameter-expansion operator form to one opaque `ParamComplex` and discards the name
//! (`28O:res-load-inert-conservatism`), so a reference-driven constant capture could not prove
//! itself complete: `${ROOT%/}` names a constant this pass cannot see. Emitting every top-level
//! constant of every file that contributes code sidesteps that hole entirely — the constants are
//! proven inert to evaluate (`rul-marked-file-is-load-inert`) and a file's constants are the
//! natural unit to travel with that file's code. The residue it accepts is named in the ledger: a
//! body reading a constant from a file that contributes NO code is not captured, because nothing
//! ties that file to this definition.

use std::collections::{BTreeMap, BTreeSet};

use dorc_core::Span;
use dorc_syntax::ast::{Ast, NodeKind, WordPart};

/// A top-level declaration one loaded source contributes that is not a role member: a helper
/// funcdef or a file-level constant. `bytes` is the authored text, verbatim — exactly what
/// `dorc strip` leaves of a non-role top-level item (the whole-file strip erases only role bodies),
/// so the emitted closure keeps the `strip-is-pure-erasure` byte floor.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Declaration {
    file: usize,
    span: Span,
    bytes: String,
}

/// Why a closure could not be pinned. One world-state with one remediation ("make the loaded
/// sources agree, or load only one of them"), so it is one reason rather than sibling classes
/// (`28L:rul-reason-enums-not-sibling-codes`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureRefusal {
    /// The name the loaded sources disagree about.
    pub name: String,
    /// The source indices that declare it, in load order — at least two, by construction.
    pub files: Vec<usize>,
}

/// Every non-role top-level declaration in the loaded source set, indexed by name.
///
/// Built ONCE per unit and shared by every seat that pins a definition (the guard preamble's vouch
/// lift and the probe's two ship seams), because re-deriving it per site would re-parse every
/// source per site and, worse, leave two copies of the resolution rule to drift — the failure
/// `oracle/CLAUDE.md live-source-is-the-only-resolution-seat` records for the role lane.
#[derive(Debug, Clone, Default)]
pub struct HelperIndex {
    /// Helper funcdef name → its declarations across the loaded sources, in load order.
    helpers: BTreeMap<String, Vec<Declaration>>,
    /// Constant name → its declarations across the loaded sources, in load order. A single
    /// `A=1 B=2` item declares two names and carries one `bytes` (the whole item).
    constants: BTreeMap<String, Vec<Declaration>>,
    /// Per source index, its top-level constant declarations in source order — the emission unit
    /// (see the module doc: constants ride per contributing file).
    constants_by_file: BTreeMap<usize, Vec<Declaration>>,
}

impl HelperIndex {
    /// Index the ordered loaded sources. `srcs` is the SOURCE-wide vector (the book included), in
    /// load order, so an index into it is the [`dorc_core::SourceFileId`]
    /// (`28O:dec-load-order-is-the-id-order`).
    #[must_use]
    pub fn build(srcs: &[&str]) -> Self {
        let mut index = Self::default();
        for (file, src) in srcs.iter().enumerate() {
            let ast = dorc_syntax::parse(src).value;
            let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
                continue;
            };
            for &item in items {
                index.record(file, src, &ast, item);
            }
        }
        index
    }

    fn record(&mut self, file: usize, src: &str, ast: &Ast, item: dorc_core::AstId) {
        let node = ast.node(item);
        let bytes = || slice(src, node.span);
        match &node.kind {
            NodeKind::FuncDef { name, .. } if crate::reserved::role_family(name).is_none() => {
                self.helpers
                    .entry(name.clone())
                    .or_default()
                    .push(Declaration {
                        file,
                        span: node.span,
                        bytes: bytes(),
                    });
            }
            NodeKind::Simple {
                assigns,
                words,
                redirs,
            } if words.is_empty() && redirs.is_empty() => {
                let declaration = Declaration {
                    file,
                    span: node.span,
                    bytes: bytes(),
                };
                for &assign in assigns {
                    let NodeKind::Assign { name, .. } = &ast.node(assign).kind else {
                        continue;
                    };
                    self.constants
                        .entry(name.clone())
                        .or_default()
                        .push(declaration.clone());
                }
                if !assigns.is_empty() {
                    self.constants_by_file
                        .entry(file)
                        .or_default()
                        .push(declaration);
                }
            }
            _ => {}
        }
    }

    /// Whether the unit declares any non-role top-level material at all. The empty answer is the
    /// whole corpus today, and it is what makes this pass `empty-world-byte-identical`: with
    /// nothing to capture, every pinned definition is exactly its own stripped bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.helpers.is_empty() && self.constants.is_empty()
    }

    /// The closure PREFIX for a role definition authored in source `file` with body text `body`.
    ///
    /// `body` is the definition's own text (stripped or authored alike — the walk reads command
    /// positions and recurses through substitutions, and a dialect mark carries no command
    /// position). The returned string is emitted immediately BEFORE the definition; it is empty
    /// whenever the definition needs nothing, which keeps the single-file no-helper case
    /// byte-identical to the definition alone.
    ///
    /// # Errors
    ///
    /// [`ClosureRefusal`] when the loaded sources declare one needed name with differing bytes —
    /// the diamond rider (`28M` §8): version-skewed vendored copies refuse rather than dedup.
    pub fn closure_for(&self, file: usize, body: &str) -> Result<String, ClosureRefusal> {
        if self.is_empty() {
            return Ok(String::new());
        }
        let mut contributing: BTreeSet<usize> = BTreeSet::new();
        contributing.insert(file);
        let mut helpers: BTreeMap<(usize, u32), String> = BTreeMap::new();
        let mut pending: Vec<String> = called_names(body);
        let mut visited: BTreeSet<String> = BTreeSet::new();
        while let Some(name) = pending.pop() {
            if !visited.insert(name.clone()) {
                continue;
            }
            let Some(declarations) = self.helpers.get(&name) else {
                continue; // An external tool, not a helper — the ordinary case.
            };
            let chosen = agree(&name, declarations)?;
            contributing.insert(chosen.file);
            helpers.insert((chosen.file, chosen.span.lo.0), chosen.bytes.clone());
            pending.extend(called_names(&chosen.bytes));
        }
        // Constants of every contributing file, in load then source order, before any helper: a
        // funcdef body reads them at CALL time, so definition order is free, but a deterministic
        // one is not optional (`inv-determinism`).
        let mut out = String::new();
        for &contributor in &contributing {
            for declaration in self
                .constants_by_file
                .get(&contributor)
                .map_or(&[][..], Vec::as_slice)
            {
                for name in self.names_declared_by(declaration) {
                    agree(
                        &name,
                        self.constants.get(&name).map_or(&[][..], Vec::as_slice),
                    )?;
                }
                push_block(&mut out, &declaration.bytes);
            }
        }
        for bytes in helpers.values() {
            push_block(&mut out, bytes);
        }
        Ok(out)
    }

    /// Which constant names one emitted declaration binds — the `A=1 B=2` item binds two.
    fn names_declared_by(&self, declaration: &Declaration) -> Vec<String> {
        self.constants
            .iter()
            .filter(|(_, decls)| decls.contains(declaration))
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// The one declaration a name resolves to, or a refusal.
///
/// Byte-identical declarations across files are the common vendoring case and collapse to one
/// (content-dedup, `28K` §4); differing ones refuse. Note what is NOT here: sh's own
/// last-definition-wins. Taking the last would silently rebind one author's helper to another's
/// body, which is the committee-speech hazard `28M` §2 exists to fence — and unlike the role lane,
/// no admin spelling selects between two helpers of the same name.
fn agree<'a>(
    name: &str,
    declarations: &'a [Declaration],
) -> Result<&'a Declaration, ClosureRefusal> {
    let mut iter = declarations.iter();
    let first = iter.next().ok_or_else(|| ClosureRefusal {
        name: name.to_owned(),
        files: Vec::new(),
    })?;
    if iter.any(|other| other.bytes != first.bytes) {
        return Err(ClosureRefusal {
            name: name.to_owned(),
            files: declarations.iter().map(|d| d.file).collect(),
        });
    }
    Ok(first)
}

fn push_block(out: &mut String, bytes: &str) {
    out.push_str(bytes);
    out.push('\n');
}

fn slice(src: &str, span: Span) -> String {
    src.get(span.lo.0 as usize..span.hi.0 as usize)
        .unwrap_or_default()
        .to_owned()
}

/// Every literal command-position word in a body — the helper CANDIDATES.
///
/// Over-collects on purpose: a candidate the index does not know is an external tool and is
/// dropped. Under-collecting is the dangerous direction (a missed helper ships a body that cannot
/// run), so the walk descends through every construct that can hold a command, command
/// substitutions included, and a dynamic command word contributes nothing here because the parser
/// ⊤-rejects it upstream (`syntax/CLAUDE.md syntactic-top-triggers`).
fn called_names(body: &str) -> Vec<String> {
    let ast = dorc_syntax::parse(body).value;
    let mut out = Vec::new();
    walk(&ast, ast.root(), &mut out);
    out
}

fn walk(ast: &Ast, id: dorc_core::AstId, out: &mut Vec<String>) {
    match &ast.node(id).kind {
        NodeKind::Script { items } | NodeKind::List { items } => {
            for &item in items {
                walk(ast, item, out);
            }
        }
        NodeKind::Simple { words, .. } => {
            if let Some(&first) = words.first()
                && let Some(text) = literal_word(ast, first)
            {
                out.push(text);
            }
            for &word in words {
                walk_word(ast, word, out);
            }
        }
        NodeKind::Pipeline { stages, .. } => {
            for &stage in stages {
                walk(ast, stage, out);
            }
        }
        NodeKind::AndOr { left, right, .. } => {
            walk(ast, *left, out);
            walk(ast, *right, out);
        }
        NodeKind::Subshell { body, .. }
        | NodeKind::Group { body, .. }
        | NodeKind::FuncDef { body, .. } => walk(ast, *body, out),
        NodeKind::If {
            cond,
            then_body,
            elifs,
            else_body,
        } => {
            walk(ast, *cond, out);
            walk(ast, *then_body, out);
            for elif in elifs {
                walk(ast, elif.cond, out);
                walk(ast, elif.body, out);
            }
            if let Some(body) = else_body {
                walk(ast, *body, out);
            }
        }
        NodeKind::Case { word, arms } => {
            walk_word(ast, *word, out);
            for arm in arms {
                walk(ast, arm.body, out);
            }
        }
        NodeKind::ForLoop { words, body, .. } => {
            for &word in words {
                walk_word(ast, word, out);
            }
            walk(ast, *body, out);
        }
        NodeKind::WhileLoop { cond, body, .. } => {
            walk(ast, *cond, out);
            walk(ast, *body, out);
        }
        NodeKind::Unsupported { salvaged, .. } => {
            for &child in salvaged {
                walk(ast, child, out);
            }
        }
        NodeKind::Word { .. } | NodeKind::Assign { .. } | NodeKind::Redir { .. } => {}
    }
}

/// Descend into a word's command substitutions — `$(_wombat_dest "$1")` calls a helper.
fn walk_word(ast: &Ast, id: dorc_core::AstId, out: &mut Vec<String>) {
    let NodeKind::Word { parts } = &ast.node(id).kind else {
        return;
    };
    walk_parts(ast, parts, out);
}

fn walk_parts(ast: &Ast, parts: &[WordPart], out: &mut Vec<String>) {
    for part in parts {
        match part {
            WordPart::CommandSubst(inner) => walk(ast, *inner, out),
            WordPart::DoubleQuoted(inner) => walk_parts(ast, inner, out),
            _ => {}
        }
    }
}

/// A word that is exactly one literal run — the only shape that can NAME a helper. A quoted or
/// expanded command word is either the same literal text (harmless to miss: the parser ⊤-rejects a
/// dynamic command name) or not a name at all.
fn literal_word(ast: &Ast, id: dorc_core::AstId) -> Option<String> {
    let NodeKind::Word { parts } = &ast.node(id).kind else {
        return None;
    };
    match parts.as_slice() {
        [WordPart::Literal(text) | WordPart::SingleQuoted(text)] => Some(text.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{ClosureRefusal, HelperIndex};

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn index(srcs: &[&str]) -> HelperIndex {
        HelperIndex::build(srcs)
    }

    /// The corpus as it stands: no helpers, no constants. The pass must be invisible there, or
    /// every golden in the tree moves for nothing (`empty-world-byte-identical`).
    #[test]
    fn a_unit_with_no_helpers_pins_the_definition_alone() {
        let src = format!("{MARKER}wombat__is_converged() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let index = index(&[&src]);
        assert!(index.is_empty());
        assert_eq!(
            index.closure_for(0, "wombat__is_converged() { wombat cmp -- \"$1\"; }"),
            Ok(String::new())
        );
    }

    /// The measured motivation (module doc): a helper call in a verdict body lifts today and ships
    /// alone. The closure must carry the helper AND the file's constants.
    #[test]
    fn a_same_file_helper_and_its_file_constants_travel_with_the_definition() {
        let src = format!(
            "{MARKER}WOMBAT_ROOT=/etc/wombat\n\
             _wombat_check() {{\n   wombat cmp -- \"$1\" \"$WOMBAT_ROOT/$1\"\n}}\n\
             wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n"
        );
        let index = index(&[&src]);
        let closure = index
            .closure_for(0, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect("one source cannot disagree with itself");
        assert!(
            closure.contains("WOMBAT_ROOT=/etc/wombat"),
            "the file's constant rides with its code:\n{closure}"
        );
        assert!(
            closure.contains("_wombat_check() {"),
            "the called helper rides with the definition:\n{closure}"
        );
    }

    /// `28M` §8's two-file package shape: the helpers file is a SEPARATE loaded source, and the
    /// entrypoint's closure must reach across it. This is the property that makes an oracle package
    /// splittable at all, so it is pinned rather than assumed.
    #[test]
    fn a_helper_in_another_loaded_source_is_captured() {
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let index = index(&[&helpers, &entry]);
        let closure = index
            .closure_for(1, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect("the two sources agree");
        assert!(
            closure.contains("_wombat_check() {"),
            "a cross-file helper rides with the entrypoint (`28M` §8 overlay riders):\n{closure}"
        );
    }

    /// Transitivity: a helper that calls a helper. A one-level walk would ship a body that still
    /// cannot run, which is the whole failure this pass exists to end.
    #[test]
    fn the_helper_walk_is_transitive() {
        let src = format!(
            "{MARKER}_inner() {{\n   wombat probe\n}}\n\
             _outer() {{\n   _inner\n}}\n\
             wombat__is_converged() {{\n   _outer \"$1\"\n}}\n"
        );
        let closure = index(&[&src])
            .closure_for(0, "wombat__is_converged() {\n   _outer \"$1\"\n}")
            .expect("one source");
        assert!(closure.contains("_outer() {"), "{closure}");
        assert!(closure.contains("_inner() {"), "{closure}");
    }

    /// A helper called from inside a command substitution is still called.
    #[test]
    fn a_helper_reached_through_a_command_substitution_is_captured() {
        let src = format!(
            "{MARKER}_dest() {{\n   printf '%s\\n' \"$1\"\n}}\n\
             wombat__is_converged() {{\n   wombat cmp -- \"$(_dest \"$1\")\"\n}}\n"
        );
        let closure = index(&[&src])
            .closure_for(
                0,
                "wombat__is_converged() {\n   wombat cmp -- \"$(_dest \"$1\")\"\n}",
            )
            .expect("one source");
        assert!(closure.contains("_dest() {"), "{closure}");
    }

    /// The vendoring case `28K` §4 names: byte-identical copies across files are ONE definition,
    /// and must not refuse.
    #[test]
    fn byte_identical_copies_across_sources_dedup_rather_than_refuse() {
        let body = "_wombat_check() {\n   wombat cmp -- \"$1\"\n}\n";
        let a = format!("{MARKER}{body}");
        let b = format!("{MARKER}{body}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let closure = index(&[&a, &b])
            .closure_for(1, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect("identical copies agree");
        assert_eq!(
            closure.matches("_wombat_check() {").count(),
            1,
            "one emission, not two:\n{closure}"
        );
    }

    /// The diamond rider (`28M` §8): version-skewed vendored copies REFUSE rather than dedup. sh
    /// would take the last silently; here the pin is withheld and the site runs.
    #[test]
    fn version_skewed_copies_refuse_the_pin() {
        let a = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let b = format!(
            "{MARKER}_wombat_check() {{\n   wombat cmp --strict -- \"$1\"\n}}\n\
             wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n"
        );
        assert_eq!(
            index(&[&a, &b]).closure_for(1, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}"),
            Err(ClosureRefusal {
                name: "_wombat_check".to_owned(),
                files: vec![0, 1],
            })
        );
    }

    /// A role funcdef is never closure material: the role lane resolves those through
    /// `live_source`, and capturing one here would be a second, unblessed resolution seat
    /// (`oracle/CLAUDE.md live-source-is-the-only-resolution-seat`).
    #[test]
    fn a_role_member_is_not_captured_as_a_helper() {
        let src = format!(
            "{MARKER}other__predict() {{\n   other status\n}}\n\
             wombat__is_converged() {{\n   other__predict \"$1\"\n}}\n"
        );
        let closure = index(&[&src])
            .closure_for(0, "wombat__is_converged() {\n   other__predict \"$1\"\n}")
            .expect("one source");
        assert!(
            !closure.contains("other__predict() {"),
            "role members resolve through the role lane, never here:\n{closure}"
        );
    }

    /// Determinism over the emission order (`inv-determinism`): two helpers, one answer, whatever
    /// the walk's discovery order.
    #[test]
    fn the_emission_order_is_stable() {
        let src = format!(
            "{MARKER}_b() {{\n   wombat b\n}}\n_a() {{\n   wombat a\n}}\n\
             wombat__is_converged() {{\n   _b; _a\n}}\n"
        );
        let index = index(&[&src]);
        let body = "wombat__is_converged() {\n   _b; _a\n}";
        let once = index.closure_for(0, body).expect("one source");
        assert_eq!(once, index.closure_for(0, body).expect("one source"));
        assert!(
            once.find("_b() {") < once.find("_a() {"),
            "source order, not discovery order:\n{once}"
        );
    }
}
