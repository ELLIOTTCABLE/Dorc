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
//! **Resolution is sh's own late binding; the LICENSE is what custody gates**
//! (`28R:rul-resolution-matches-shell-loading` · `rul-emission-custody-composite`). A name with
//! several declarations resolves to the LAST one in load order, because that is the body a shell
//! would actually run — an engine that answered differently would ship a body no execution binds.
//! What the engine refuses is not the resolution but the LICENSE over a composition its voucher
//! never wrote: the vouch suspends when the resolved definition sits in another custody AND either
//! the book redefines the name or the name is plural-with-differing-bytes, so load order never
//! silently adjudicates whose body serves whose vouch. A singular cross-file reach — one
//! declaration, anywhere in the loaded set — stays licensed, which is what keeps `28M` §7's
//! helpers-file + thin-entrypoints package shape working. Byte-identical plurality counts as
//! singular (there is nothing to adjudicate). Suspension withholds the pin, which withholds the
//! vouch and the ship, which runs the site — the safe direction, and attributed.
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

/// One declaration the emission carries, with the identity that dedups it across bodies.
///
/// The key is the DECLARATION SITE, not the name: a constants item binds several names and a
/// name-keyed dedup could not spell it. Two bodies whose snapshots resolved to the same
/// declaration therefore emit it once (`28R:rul-instantiation-hash-dedup`'s dedup, computed
/// eagerly — the resolved identity IS the key, so equality here is structural rather than a hash
/// that would need confirming).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClosureDecl {
    /// The loaded source that spells it, and its offset there — the dedup key and the emission
    /// order (load order, then source order).
    key: (usize, u32),
    /// The authored text, verbatim.
    bytes: String,
}

impl ClosureDecl {
    /// The declaration's authored bytes — what an emission seat writes.
    #[must_use]
    pub fn bytes(&self) -> &str {
        &self.bytes
    }

    /// The `(source index, offset)` identity two snapshots dedup on.
    #[must_use]
    pub fn key(&self) -> (usize, u32) {
        self.key
    }
}

/// What a pinned definition carries besides itself.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Closure {
    /// The declarations to emit BEFORE the definition, in emission order; empty when it needs
    /// nothing. Kept as declarations rather than one blob because the apply artifact hoists ONE
    /// shared preamble above the whole book, so two guards that reach one helper must emit it once
    /// — two same-named funcdefs in the emitted preamble is the shape
    /// `plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding` forbids. The probe lane, which
    /// re-emits per site immediately before each invocation, just joins them ([`Closure::sh`]).
    decls: Vec<ClosureDecl>,
    /// The external command words the closure's own bodies reach, in name order.
    ///
    /// The guard's dual-rail attribution (`24D`'s `guardcmd` ledger) allowlists the commands a
    /// shipped check legitimately runs at apply, and it was derived from the ROLE body alone. Once
    /// a helper travels with the definition, the real check-commands live in the helper — measured
    /// on this tree by the first end-to-end closure case, which the gate flagged as an
    /// unaccounted-for apply-only line. Attribution only, never decision data.
    pub commands: Vec<String>,
}

impl Closure {
    /// The declarations joined, each on its own line — the per-site emission the probe lane uses.
    #[must_use]
    pub fn sh(&self) -> String {
        let mut out = String::new();
        for decl in &self.decls {
            push_block(&mut out, &decl.bytes);
        }
        out
    }

    /// The declarations, in emission order — for a seat that dedups across bodies.
    #[must_use]
    pub fn decls(&self) -> &[ClosureDecl] {
        &self.decls
    }
}

/// Why a shipped body's composition carries no license (`28R:rul-mixed-custody-suspends-vouch` ·
/// `rul-contested-name-never-resolved` · the permanent WITHHELD tier).
///
/// One census answers all of it — which names the book defines, and which names the loaded sources
/// declare more than once — so these are REASON VARIANTS of one world (a composition the voucher
/// did not write, or one the engine cannot enumerate), never sibling classes
/// (`28L:rul-reason-enums-not-sibling-codes`). Every variant lands the same place: no elide, no
/// guard, the site runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureDenial {
    /// The name whose resolution carries no license.
    pub name: String,
    /// Which of the four worlds this is.
    pub reason: DenialReason,
    /// Where the loaded sources declare the name, in load order — empty when only the book does.
    pub sites: Vec<(usize, Span)>,
}

/// The four worlds [`ClosureDenial`] distinguishes for AID; the license outcome is identical.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenialReason {
    /// The book defines a function under a name the shipped body calls, and the loaded sources
    /// declare it too: at apply the book's rebinding wins below its own definition, so the
    /// engineer's vouch would cover a body the admin replaced.
    BookRedefinesHelper,
    /// The book defines a function under a name the shipped body calls as an external UTILITY. The
    /// engine never chooses between the documented tool and the admin's function
    /// (`28R:rul-contested-name-never-resolved`): honoring runs unvouched book code inside a check,
    /// bypassing under-executes in wrapper books, and both are engine referent-choices between two
    /// humans' meanings.
    BookShadowsCommand,
    /// Two loaded sources declare the name with DIFFERING bytes and the resolved one lies outside
    /// the voucher's custody: sh's last-wins would decide whose body serves this vouch, and load
    /// order is not an adjudicator of authorship (`28K` §6).
    PluralAcrossCustody,
    /// The body reaches a call the engine cannot enumerate (a non-literal command word, or `eval`),
    /// so its snapshot cannot be closed. The permanent bottom rung — never scaffolding
    /// (`28R:rul-instantiation-hash-dedup` tier 3).
    UnenumerableCall,
}

/// Why two loaded sources cannot both speak for one non-role name — the LOAD-EDGE report, which is
/// a different question from whether any body's license survives ([`ClosureDenial`]). One
/// world-state with one remediation ("make the loaded sources agree, or load only one of them").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureRefusal {
    /// The name the loaded sources disagree about.
    pub name: String,
    /// Where each disagreeing source declares it, in load order — at least two, by construction.
    /// The caller resolves `(source index, span)` to the `file:line` a diagnostic points at.
    pub sites: Vec<(usize, Span)>,
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
    /// Every function name the BOOK defines, at any depth — the census both custody arms read
    /// (`rul-emission-custody-composite`). The book is never an index CONTRIBUTOR (its top level is
    /// not load-inert), but what it defines still decides whether somebody else's vouch survives at
    /// apply, so it is carried separately rather than not at all.
    book_defines: BTreeSet<String>,
}

impl HelperIndex {
    /// Index the ordered loaded sources. `srcs` is the SOURCE-wide vector (the book included), in
    /// load order, so an index into it is the [`dorc_core::SourceFileId`]
    /// (`28O:dec-load-order-is-the-id-order`).
    /// Only a source whose WHOLE top level is provably inert to load contributes — every item a
    /// funcdef or a bare statically-valued assignment (`rul-marked-file-is-load-inert`'s own
    /// predicate). That inertness is exactly the license to hoist a declaration above somebody's
    /// book, and it is also what keeps the BOOK out of the index without threading its id here: a
    /// runbook has commands at top level, so its helpers stay where its author put them, in the
    /// artifact's own text, and are never copied above it.
    ///
    /// `book` names which source index is the BOOK, when the caller has one. It is threaded
    /// explicitly rather than inferred from load-inertness, because "not inert" and "the admin's
    /// file" are different facts and attributing a suspension to the book when a malformed ORACLE
    /// caused it would be mis-attribution (`271:rul-sin-ordering`, pope-sin tier). The oracle-only
    /// lanes pass `None` and see no book census, which is correct: they answer questions about the
    /// loaded package set.
    #[must_use]
    pub fn build(srcs: &[&str], book: Option<usize>) -> Self {
        let mut index = Self::default();
        for (file, src) in srcs.iter().enumerate() {
            let ast = dorc_syntax::parse(src).value;
            if book == Some(file) {
                for (_, node) in ast.iter() {
                    if let NodeKind::FuncDef { name, .. } = &node.kind {
                        index.book_defines.insert(name.clone());
                    }
                }
                continue;
            }
            let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
                continue;
            };
            if !items
                .iter()
                .all(|&item| crate::load_inert::item_is_load_inert(&ast, item))
            {
                continue;
            }
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

    /// Every name the loaded sources declare with DIFFERING bytes, in name order.
    ///
    /// Reported at the load edge rather than per pinned definition, and reported whether or not a
    /// closure reaches the name, because the collision is real either way: loading both sources
    /// means sh binds the later body for EVERY caller, so the earlier author's helper is already
    /// rebound out from under them. A per-definition report would also be a correlated cascade —
    /// one collision, N families, N-1 of them pointed at the wrong repair
    /// (`28O:dec-one-diagnostic-per-file-not-per-item`).
    #[must_use]
    pub fn conflicts(&self) -> Vec<ClosureRefusal> {
        self.helpers
            .iter()
            .chain(&self.constants)
            .filter_map(|(name, declarations)| agree(name, declarations).err())
            .collect()
    }

    /// The SNAPSHOT a role definition authored in source `file` ships with, given body text `body`.
    ///
    /// `body` is the definition's own text (stripped or authored alike — the walk reads command
    /// positions and recurses through substitutions, and a dialect mark carries no command
    /// position). The snapshot is emitted BEFORE the definition; it is empty whenever the definition
    /// needs nothing, which keeps the single-file no-helper case byte-identical to the definition
    /// alone. Every reached name resolves to the declaration a shell would bind — the LAST in load
    /// order (`28R:rul-resolution-matches-shell-loading`) — and the whole reached set is one
    /// transplanted unit, so a body and its helpers are the environment one program point holds.
    ///
    /// # Errors
    ///
    /// [`ClosureDenial`] when the composition carries no license: the book redefines a reached name,
    /// the resolution crossed custody on a plural-with-differing-bytes name, or a call cannot be
    /// enumerated. Each withholds the pin, hence the vouch, hence the ship — the site runs.
    pub fn closure_for(&self, file: usize, body: &str) -> Result<Closure, ClosureDenial> {
        if self.is_empty() && self.book_defines.is_empty() {
            return Ok(Closure::default());
        }
        let mut contributing: BTreeSet<usize> = BTreeSet::new();
        contributing.insert(file);
        let mut decls: BTreeMap<(usize, u32), String> = BTreeMap::new();
        let mut reached: BTreeSet<String> = BTreeSet::new();
        let mut pending: Vec<String> = Self::enumerable_calls(body)?;
        let mut visited: BTreeSet<String> = BTreeSet::new();
        while let Some(name) = pending.pop() {
            if !visited.insert(name.clone()) {
                continue;
            }
            let declarations = self.helpers.get(&name).map_or(&[][..], Vec::as_slice);
            let Some(chosen) = self.resolve(&name, declarations, file)? else {
                continue; // An external tool, not a helper — the ordinary case.
            };
            contributing.insert(chosen.file);
            decls.insert((chosen.file, chosen.span.lo.0), chosen.bytes.clone());
            let inner = Self::enumerable_calls(&chosen.bytes)?;
            reached.extend(inner.iter().cloned());
            pending.extend(inner);
        }
        let mut constants: BTreeMap<(usize, u32), String> = BTreeMap::new();
        for &contributor in &contributing {
            for declaration in self
                .constants_by_file
                .get(&contributor)
                .map_or(&[][..], Vec::as_slice)
            {
                for name in self.names_declared_by(declaration) {
                    let declarations = self.constants.get(&name).map_or(&[][..], Vec::as_slice);
                    self.resolve(&name, declarations, file)?;
                }
                constants.insert(
                    (declaration.file, declaration.span.lo.0),
                    declaration.bytes.clone(),
                );
            }
        }
        // Constants precede helpers: a helper body may read one, and neither order is a choice.
        let decls = constants
            .into_iter()
            .chain(decls)
            .map(|(key, bytes)| ClosureDecl { key, bytes })
            .collect();
        Ok(Closure {
            decls,
            commands: reached
                .into_iter()
                .filter(|name| !self.helpers.contains_key(name))
                .collect(),
        })
    }

    /// The declaration a shell would bind for `name`, or `None` when nothing loaded declares it (an
    /// external utility — the ordinary case).
    ///
    /// THE COMPOSITE PREDICATE (`rul-emission-custody-composite`): resolution is always sh's
    /// last-wins, and the license suspends iff the resolved custody differs from the voucher's
    /// (`asker`) AND either the book redefines the name or the declarations disagree on bytes. A
    /// singular cross-file reach is licensed — one declaration is not an adjudication — which is
    /// what keeps the sanctioned helpers-file package shape working. Byte-identical plurality
    /// counts as singular.
    fn resolve<'a>(
        &self,
        name: &str,
        declarations: &'a [Declaration],
        asker: usize,
    ) -> Result<Option<&'a Declaration>, ClosureDenial> {
        let sites =
            || -> Vec<(usize, Span)> { declarations.iter().map(|d| (d.file, d.span)).collect() };
        let deny = |reason| ClosureDenial {
            name: name.to_owned(),
            reason,
            sites: sites(),
        };
        if self.book_defines.contains(name) {
            return Err(deny(if declarations.is_empty() {
                DenialReason::BookShadowsCommand
            } else {
                DenialReason::BookRedefinesHelper
            }));
        }
        let Some(chosen) = declarations.last() else {
            return Ok(None);
        };
        let differs = declarations.iter().any(|other| other.bytes != chosen.bytes);
        if differs && chosen.file != asker {
            return Err(deny(DenialReason::PluralAcrossCustody));
        }
        Ok(Some(chosen))
    }

    /// Every literal command-position word in `body`, or a denial when the body carries a definition
    /// vector the walk cannot follow through.
    ///
    /// A dynamic command NAME needs no arm here: the parser ⊤-rejects one upstream
    /// (`syntax/CLAUDE.md syntactic-top-triggers`), so a lifted oracle body cannot hold one. What
    /// remains is a literal vector — `eval` or `alias` — which can bind or invoke a name this walk
    /// will never see, so the snapshot cannot be closed and the tier is WITHHELD (permanent, per
    /// `28R:rul-instantiation-hash-dedup`).
    fn enumerable_calls(body: &str) -> Result<Vec<String>, ClosureDenial> {
        let calls = scan_calls(body);
        if let Some(vector) = calls.definition_vector() {
            return Err(ClosureDenial {
                name: vector,
                reason: DenialReason::UnenumerableCall,
                sites: Vec::new(),
            });
        }
        Ok(calls.names)
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

/// What one walk over a body found: the literal command words, and whether any construct can bind or
/// invoke a name the walk cannot see.
struct Calls {
    names: Vec<String>,
    /// A `NodeKind::Unsupported { reason: DynamicExecution }` — `eval`, a `.`/`source` of a computed
    /// target, or a dynamic command name. The parser has already classified all three, so this reads
    /// its classification rather than re-deriving one from byte shapes.
    dynamic_execution: bool,
}

impl Calls {
    /// The in-process DEFINITION VECTOR this body carries, if any
    /// (`28R:rul-defensive-mode-definition-vectors`).
    ///
    /// Deliberately NOT any-⊤: an unmodeled command is an external binary and cannot define a
    /// function in the executing shell, so `hork` must never qualify — only a construct that binds a
    /// NAME here does. Two shapes qualify, and the parser supplies the harder one: `DynamicExecution`
    /// (which is exactly `eval` · a computed `.` · a dynamic command name) and a literal `alias`,
    /// which parses as an ordinary command word.
    fn definition_vector(&self) -> Option<String> {
        if self.dynamic_execution {
            return Some(DYNAMIC_EXECUTION.to_owned());
        }
        self.names.iter().find(|name| *name == "alias").cloned()
    }
}

/// The name a `DynamicExecution` ⊤-reject travels under. Not a command word: the parser folds
/// `eval`, a computed `.`, and a dynamic command name into ONE reason, and re-deriving which of the
/// three it was from byte shapes is the re-detection layer `28L:rul-editability-is-stamped-never-re-derived`
/// retired. The reason IS the answer.
const DYNAMIC_EXECUTION: &str = "dynamic-execution";

/// Every definition vector the given sources carry, in name order — the whole-artifact question
/// behind DEFENSIVE emission. Empty is the overwhelming case, and empty means the artifact may ship
/// bare names.
#[must_use]
pub fn definition_vectors(srcs: &[&str]) -> BTreeSet<String> {
    srcs.iter()
        .filter_map(|src| scan_calls(src).definition_vector())
        .collect()
}

/// Whether the loaded sources agree about one name — the LOAD-EDGE report's question only.
///
/// Byte-identical declarations across files are the common vendoring case and agree
/// (content-dedup, `28K` §4); differing ones are reported, because loading both already rebound the
/// name for every caller and the earlier author should hear about it. This is not the RESOLUTION
/// rule ([`HelperIndex::resolve`] holds that, and it follows sh): a disagreement here is a warning
/// plus, where it would decide whose body serves whose vouch, a suspended license.
fn agree<'a>(
    name: &str,
    declarations: &'a [Declaration],
) -> Result<&'a Declaration, ClosureRefusal> {
    let refusal = || ClosureRefusal {
        name: name.to_owned(),
        sites: declarations.iter().map(|d| (d.file, d.span)).collect(),
    };
    let mut iter = declarations.iter();
    let first = iter.next().ok_or_else(refusal)?;
    if iter.any(|other| other.bytes != first.bytes) {
        return Err(refusal());
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

/// One walk, both answers: every literal command-position word, and whether the body carries a
/// definition vector.
///
/// The names OVER-collect on purpose: a candidate the index does not know is an external tool and is
/// dropped. Under-collecting is the dangerous direction (a missed helper ships a body that cannot
/// run), so the walk descends through every construct that can hold a command, command substitutions
/// included. What a literal-word walk structurally cannot see — `eval`, a computed `.`, a dynamic
/// command name — the PARSER has already classified as one `DynamicExecution` ⊤-reject, so the flag
/// reads that classification instead of re-deriving one.
fn scan_calls(body: &str) -> Calls {
    let ast = dorc_syntax::parse(body).value;
    let mut out = Vec::new();
    walk(&ast, ast.root(), &mut out);
    let dynamic_execution = ast.iter().any(|(_, node)| {
        matches!(
            node.kind,
            NodeKind::Unsupported {
                reason: dorc_syntax::ast::UnsupportedReason::DynamicExecution,
                ..
            }
        )
    });
    Calls {
        names: out,
        dynamic_execution,
    }
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
    use super::HelperIndex;

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn index(srcs: &[&str]) -> HelperIndex {
        HelperIndex::build(srcs, None)
    }

    /// The corpus as it stands: no helpers, no constants. The pass must be invisible there, or
    /// every golden in the tree moves for nothing (`empty-world-byte-identical`).
    #[test]
    fn a_unit_with_no_helpers_pins_the_definition_alone() {
        let src = format!("{MARKER}wombat__is_converged() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let index = index(&[&src]);
        assert!(index.is_empty());
        assert_eq!(
            index
                .closure_for(0, "wombat__is_converged() { wombat cmp -- \"$1\"; }")
                .map(|c| c.sh()),
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
            .expect("one source cannot disagree with itself")
            .sh();
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
            .expect("the two sources agree")
            .sh();
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
            .expect("one source")
            .sh();
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
            .expect("one source")
            .sh();
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
            .expect("identical copies agree")
            .sh();
        assert_eq!(
            closure.matches("_wombat_check() {").count(),
            1,
            "one emission, not two:\n{closure}"
        );
    }

    /// The CAPTURE half of `rul-emission-custody-composite`: version-skewed copies resolve by sh's
    /// own last-wins, and when the winner sits in the VOUCHER's own file there is nothing for load
    /// order to adjudicate — the snapshot ships. The load edge still reports the collision, because
    /// the earlier author's helper really was rebound for every caller.
    #[test]
    fn a_plural_helper_resolving_into_the_vouchers_file_still_ships() {
        let a = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let b = format!(
            "{MARKER}_wombat_check() {{\n   wombat cmp --strict -- \"$1\"\n}}\n\
             wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n"
        );
        let index = index(&[&a, &b]);
        let closure = index
            .closure_for(1, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect("last-wins lands in the voucher's own custody")
            .sh();
        assert!(
            closure.contains("wombat cmp --strict"),
            "the LAST declaration is the one a shell binds:\n{closure}"
        );
        assert!(
            !closure.contains("wombat cmp -- \"$1\""),
            "the shadowed earlier body must not travel:\n{closure}"
        );
        assert_eq!(
            index
                .conflicts()
                .iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>(),
            vec!["_wombat_check"],
            "the load edge reports the collision once, by name"
        );
    }

    /// The SUSPEND half: the same plurality, resolved into a file that is NOT the voucher's. Here
    /// load order would decide whose body serves this engineer's vouch, which is the one thing the
    /// composite predicate refuses — no pin, no vouch, the site runs.
    #[test]
    fn a_plural_helper_resolving_outside_the_vouchers_custody_suspends() {
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let a = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let b = format!("{MARKER}_wombat_check() {{\n   wombat cmp --strict -- \"$1\"\n}}\n");
        let denied = index(&[&entry, &a, &b])
            .closure_for(0, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect_err("cross-custody plurality is load order adjudicating authorship");
        assert_eq!(denied.name, "_wombat_check");
        assert_eq!(denied.reason, super::DenialReason::PluralAcrossCustody);
        assert_eq!(
            denied.sites.iter().map(|(f, _)| *f).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    /// A SINGULAR cross-file reach stays licensed even though custody differs — the sanctioned
    /// helpers-file + thin-entrypoints package shape (`28M` §7). One declaration is not an
    /// adjudication, so the composite's second conjunct never fires.
    #[test]
    fn a_singular_cross_custody_reach_stays_licensed() {
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let closure = index(&[&entry, &helpers])
            .closure_for(0, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect("a lone declaration decides nothing")
            .sh();
        assert!(closure.contains("_wombat_check() {"), "{closure}");
    }

    /// The book redefining a helper the engineer's body reaches SUSPENDS: at apply the hoisted
    /// preamble sits above the book, so the book's rebinding wins at every guard below it and the
    /// vouch would cover a composition its author never wrote.
    #[test]
    fn a_book_redefinition_of_a_reached_helper_suspends() {
        let oracle = format!(
            "{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n\
             wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n"
        );
        let book = "_wombat_check() {\n   printf 'always converged\\n'\n}\nwombat sync a\n";
        let denied = HelperIndex::build(&[&oracle, book], Some(1))
            .closure_for(0, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect_err("the admin replaced the body the vouch rests on");
        assert_eq!(denied.name, "_wombat_check");
        assert_eq!(denied.reason, super::DenialReason::BookRedefinesHelper);
    }

    /// The contested TOOL name (`28R:rul-contested-name-never-resolved`): the book defines a function
    /// under the name the shipped body calls as an external utility. Honoring it runs unvouched book
    /// code inside a read-only check; bypassing it under-executes in wrapper books. Both are engine
    /// referent-choices between two humans, so the engine declines instead.
    #[test]
    fn a_book_function_shadowing_a_called_tool_declines() {
        let oracle = format!("{MARKER}wombat__is_converged() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let book = "wombat() {\n   hork tune\n}\nwombat sync a\n";
        let denied = HelperIndex::build(&[&oracle, book], Some(1))
            .closure_for(0, "wombat__is_converged() {\n   wombat cmp -- \"$1\"\n}")
            .expect_err("the referent of `wombat` is now two humans' question");
        assert_eq!(denied.name, "wombat");
        assert_eq!(denied.reason, super::DenialReason::BookShadowsCommand);
    }

    /// The oracle-only lanes pass no book index and must see no census: they answer questions about
    /// the loaded package set, and a book-shaped denial there would be attributed to a file the lane
    /// was never asked about.
    #[test]
    fn without_a_book_index_the_census_is_empty() {
        let oracle = format!("{MARKER}wombat__is_converged() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let book = "wombat() {\n   hork tune\n}\nwombat sync a\n";
        assert!(
            HelperIndex::build(&[&oracle, book], None)
                .closure_for(0, "wombat__is_converged() {\n   wombat cmp -- \"$1\"\n}")
                .is_ok()
        );
    }

    /// The permanent WITHHELD tier: a body reaching `eval` can bind or invoke a name no walk sees,
    /// so its snapshot cannot be closed. Not scaffolding — the bottom rung.
    #[test]
    fn an_unenumerable_call_withholds_permanently() {
        let src = format!(
            "{MARKER}_helper() {{\n   wombat cmp\n}}\n\
             wombat__is_converged() {{\n   eval \"$1\"\n}}\n"
        );
        let denied = index(&[&src])
            .closure_for(0, "wombat__is_converged() {\n   eval \"$1\"\n}")
            .expect_err("an eval'd body has no enumerable call graph");
        assert_eq!(denied.reason, super::DenialReason::UnenumerableCall);
        assert_eq!(denied.name, super::DYNAMIC_EXECUTION);
    }

    /// Defensive emission keys on real in-process definition vectors ONLY: an unmodeled command is
    /// an external binary and cannot bind a function in the executing shell, so `hork` must never
    /// flip the mode (`28R:rul-defensive-mode-definition-vectors`).
    #[test]
    fn definition_vectors_ignore_unmodeled_commands() {
        assert!(super::definition_vectors(&["hork tune web\nwombat sync a\n"]).is_empty());
        assert_eq!(
            super::definition_vectors(&["hork tune\n", "alias ls='ls -l'\n", "eval \"$x\"\n"])
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["alias".to_owned(), super::DYNAMIC_EXECUTION.to_owned()],
            "both vectors are found, and the ⊤-reject travels under the parser's own reason"
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
            .expect("one source")
            .sh();
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
        let once = index.closure_for(0, body).expect("one source").sh();
        assert_eq!(once, index.closure_for(0, body).expect("one source").sh());
        assert!(
            once.find("_b() {") < once.find("_a() {"),
            "source order, not discovery order:\n{once}"
        );
    }
}
