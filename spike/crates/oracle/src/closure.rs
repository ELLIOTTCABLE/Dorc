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
//! (`28R:rul-resolution-matches-shell-loading` · `rul-vouch-reaches-own-custody-only`). A name with
//! several declarations resolves to the LAST one in load order, because that is the body a shell
//! would actually run — an engine that answered differently would ship a body no execution binds.
//! What the engine refuses is not the resolution but the LICENSE over a composition its voucher
//! never wrote: the vouch suspends whenever the resolved definition sits outside the voucher's own
//! CUSTODY — its file plus everything that file's `.` lines pull in, transitively
//! (`core::CustodyClosures`). Naming several files on one command line is INGESTION and composes no
//! custody, so there is no load-order arrangement of strangers' files that lets one of them serve
//! another's vouch. `28M` §7's helpers-file + thin-entrypoints package shape is licensed by the
//! entrypoints file SOURCING its helpers — the `.` is the utterance that takes custody, and nothing
//! else does. Suspension withholds the pin, which withholds the vouch and the ship, which runs the
//! site — the safe direction, and attributed.
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
    /// Which world this is.
    pub reason: DenialReason,
    /// Where the loaded sources declare the name, in load order — empty when only the book does.
    pub sites: Vec<(usize, Span)>,
}

/// The worlds [`ClosureDenial`] distinguishes for AID; the license outcome is identical.
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
    /// The declaration a shell binds lies outside the voucher's own custody. Co-loading files on
    /// one command line is ingestion, never composition, so the reach lands in an utterance this
    /// voucher did not make — and load order is not an adjudicator of authorship (`28K` §6).
    /// `30I` §3.4 case 3: the voucher's own file DID select a dependency declaring this name — a
    /// `.`, or an include guard naming it — but the declaration a shell binds is not the one that
    /// selection reaches. The guard's recognition failed, a later file shadowed the helper, or the
    /// load could not be aligned exactly. The author wrote an acceptance act; it did not land.
    DependencySelectedButUnaligned,
    /// `30I` §3.4 case 4: ordinary shell name resolution supplied the declaration, with no
    /// attributable dependency selection anywhere in the voucher's own file. Co-loading files on
    /// one command line is ingestion, never composition, so the reach lands in an utterance this
    /// voucher did not make — and load order is not an adjudicator of authorship (`28K` §6).
    ///
    /// This is ORDINARY SH and never an invalid oracle set
    /// (`30I:rul-ambient-dependencies-are-ordinary-shell`): authors already owe defensiveness
    /// against ambient command and PATH resolution, and learning that a call resolves to a
    /// function in another loaded file may not turn accepted sh into a refusal. It suspends the
    /// composition exactly as its sibling does; only the sentence differs.
    DependencyAmbientOrUntraceable,
    /// The voucher's own custody declares the name more than once, with DIFFERING bytes. Which one
    /// a shell binds depends on the exact interleaving of a file's own declarations with the ones
    /// its `.` lines pull in, and a flat load-order vector cannot express that interleaving — so
    /// rather than pick a winner by an order that is not sh's, the composition suspends. Order then
    /// decides nothing anywhere: agreeing bytes make it irrelevant, and disagreeing bytes stop
    /// here (`28Q` §1's WITHHOLD floor, closure-keyed).
    ContestedWithinCustody,
    /// The voucher's file `.`-sources something the engine could not load as dorc-lang oracle code —
    /// absent, outside the working directory the operand names, or failing the contract its author
    /// would sign by marking it. The environment the body would run in cannot be reconstructed, so
    /// nothing about it may be vouched for.
    UnresolvedLoad,
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
    /// Every function name the BOOK defines, at any depth — the census the book-side denial arms
    /// read (`rul-vouch-reaches-own-custody-only`). The book is never an index CONTRIBUTOR (its top level is
    /// not load-inert), but what it defines still decides whether somebody else's vouch survives at
    /// apply, so it is carried separately rather than not at all.
    book_defines: BTreeSet<String>,
    /// Whose utterance each source may rest on (`core::CustodyClosures`). Defaults to SINGLETONS,
    /// which is what makes every lane holding no include-tree keep its pre-sourcing answers and the
    /// no-oracle-sourcing world byte-identical.
    closures: dorc_core::CustodyClosures,
    /// THE NARRATIVE RELATION (`30I:rul-one-load-account-separate-projections`, projection 3):
    /// which sources each file's author SELECTED — every dependency its own program named,
    /// whether or not the selection aligned exactly. Strictly wider than `closures`, and used for
    /// nothing but telling `30I` §3.4's two non-exact cases apart in the aid plane.
    ///
    /// `None` ⇒ read `closures`, which is right rather than merely safe: a lane with no
    /// include-tree has no sourcing for anyone to have selected, so every out-of-custody reach
    /// there really is ambient.
    selected: Option<dorc_core::CustodyClosures>,
    /// Sources whose own `.` named nothing admissible: every vouch they carry suspends.
    unresolved_loads: BTreeSet<usize>,
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
        let mut index = Self {
            closures: dorc_core::CustodyClosures::singletons(srcs.len()),
            ..Self::default()
        };
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
            // `unset -f NAME…` REMOVES what this file declared above it, and the index has to
            // model that rather than merely tolerate the construct
            // (`30I:rul-oracle-loading-stays-load-safe` admits it at a marked top level; the
            // `p-helper-unset-f` pin is what says a widened allow-list without a model is a
            // wrong-elision route). A body below the removal reaches an UNBOUND name, so borrowing
            // the declaration above would ship a judgment no execution could have reached —
            // `271:rul-sin-ordering`'s mis-attribution tier. Shipping nothing declines at rc 127
            // on the host, which is the safe direction.
            //
            // It removes EVERY declaration indexed above it, not merely this file's. `.`-sourcing
            // applies definitions into ONE environment, so a removal cannot see a file boundary —
            // and this walk runs in load order over the indexable population, which is the same
            // argument `28R:rul-resolution-matches-shell-loading` rests last-wins on: what is
            // indexed so far IS what a shell would have bound here.
            NodeKind::Simple { words, .. }
                if crate::load_inert::unset_functions(ast, words).is_some() =>
            {
                for name in crate::load_inert::unset_functions(ast, words).unwrap_or_default() {
                    if let Some(declarations) = self.helpers.get_mut(&name) {
                        declarations.clear();
                    }
                }
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

    /// Attach the driver's include-tree: whose custody reaches whose, and which sources sourced
    /// something the driver could not load (`28Q` §2; the derivation is `dorc_cli::sourcing`).
    ///
    /// Threaded post-construction rather than taken by `build`, because the overwhelming majority of
    /// this workspace's index builders — the instrument, hint, survival-snapshot, and hand-built
    /// seats — hold no include-tree and must keep answering exactly as they did. Only the two real
    /// drivers know one, and only they call this.
    #[must_use]
    pub fn with_include_tree(
        mut self,
        closures: dorc_core::CustodyClosures,
        unresolved_loads: BTreeSet<usize>,
    ) -> Self {
        self.closures = closures;
        self.unresolved_loads = unresolved_loads;
        self
    }

    /// Attach the loader's SELECTION relation — which dependencies each file's author named at
    /// all (`30I:rul-one-load-account-separate-projections`, projection 3).
    ///
    /// Separate from [`with_include_tree`](Self::with_include_tree) because the two relations
    /// answer different questions and only the two real drivers hold the wider one. Nothing here
    /// widens a license: the relation is read at exactly one seat, to choose between two
    /// decision-inert sentences for a suspension that has already happened.
    #[must_use]
    pub fn with_selection(mut self, selected: dorc_core::CustodyClosures) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Did `asker`'s author name `target` as a dependency at all?
    fn selects(&self, asker: usize, target: usize) -> bool {
        self.selected
            .as_ref()
            .unwrap_or(&self.closures)
            .reaches(asker, target)
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
    /// the resolution left the voucher's custody, or a call cannot be enumerated. Each withholds the
    /// pin, hence the vouch, hence the ship — the site runs.
    pub fn closure_for(&self, file: usize, body: &str) -> Result<Closure, ClosureDenial> {
        // The unresolved-load suspension precedes the empty-index shortcut, and that ORDER is the
        // whole of it: a file that sourced something the driver could not load contributes no
        // declarations, so the index it produces is empty — and an empty index taking the shortcut
        // would ship the body BARE, which is precisely the rc-127-or-worse this pass exists to
        // prevent. Measured on this tree: the shortcut swallowed the suspension and a package whose
        // helpers never loaded shipped a check that answered 127.
        if self.unresolved_loads.contains(&file) {
            return Err(ClosureDenial {
                name: String::new(),
                reason: DenialReason::UnresolvedLoad,
                sites: Vec::new(),
            });
        }
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
    /// THE CUSTODY PREDICATE (`rul-vouch-reaches-own-custody-only`): resolution is always sh's
    /// last-wins, and the license suspends whenever the resolved declaration sits outside the
    /// voucher's own CUSTODY — the file itself plus everything its `.` lines pull in, transitively
    /// (`core::CustodyClosures`). CLI co-loading composes no custody, so a reach into a merely
    /// co-loaded file is a reach into somebody else's utterance whatever load order made of it;
    /// a reach into a SOURCED file is the author's own, because sourcing is the promise that makes
    /// it so.
    ///
    /// Three suspensions, in the order a reader should meet them:
    ///
    /// 1. The asker's own file sourced something the driver could not load, so the environment its
    ///    body would run in is not reconstructible at all.
    /// 2. The declaration a shell binds lies outside the asker's custody. It splits into two
    ///    decision-inert SENTENCES on whether the asker selected that dependency at all
    ///    (`30I` section 3.4 cases 3 and 4); the suspension is the same either way.
    /// 3. The asker's custody declares the name more than once with DIFFERING bytes. Which one a
    ///    shell binds turns on how a file's own declarations interleave with the ones its `.` lines
    ///    pull in, and the flat load-order vector this index is built from cannot express that
    ///    interleaving. Suspending is what keeps ORDER from deciding anything: agreeing bytes make
    ///    it irrelevant, disagreeing bytes stop here, and no licence anywhere rests on a
    ///    load-order the engine cannot promise is sh's.
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
        if self.unresolved_loads.contains(&asker) {
            return Err(deny(DenialReason::UnresolvedLoad));
        }
        if !self.closures.reaches(asker, chosen.file) {
            // `30I` §3.4's two non-exact cases, told apart by the SELECTION relation and by
            // nothing else. Both suspend identically; the distinction buys the author the right
            // repair — 'your guard did not align' versus 'you named no dependency at all' — and
            // naming the second author in the first's sentence is the pope-sin direction
            // (`271:rul-sin-ordering`). ANY declaration of the name inside the selection answers:
            // the act was written even where load order handed the site somebody else's body.
            let selected = declarations
                .iter()
                .any(|declaration| self.selects(asker, declaration.file));
            return Err(deny(if selected {
                DenialReason::DependencySelectedButUnaligned
            } else {
                DenialReason::DependencyAmbientOrUntraceable
            }));
        }
        if declarations
            .iter()
            .filter(|other| self.closures.reaches(asker, other.file))
            .any(|other| other.bytes != chosen.bytes)
        {
            return Err(deny(DenialReason::ContestedWithinCustody));
        }
        Ok(Some(chosen))
    }

    /// Every literal command-position word in `body`, or a denial when the body carries a definition
    /// vector the walk cannot follow through.
    ///
    /// A body carrying one cannot have its snapshot closed — the vector can bind or invoke a name this
    /// walk will never see — so the tier is WITHHELD, and permanently
    /// (`28R:rul-instantiation-hash-dedup`'s bottom rung). Reachability is another matter: see
    /// [`is_definition_vector`] for why a shipped body can hold `alias` and cannot hold the other two,
    /// and for the ⊤-reject reading that must NOT be substituted here.
    fn enumerable_calls(body: &str) -> Result<Vec<String>, ClosureDenial> {
        let names = called_names(body);
        if let Some(vector) = names.iter().find(|name| is_definition_vector(name)) {
            return Err(ClosureDenial {
                name: vector.clone(),
                reason: DenialReason::UnenumerableCall,
                sites: Vec::new(),
            });
        }
        Ok(names)
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

/// Is `name` an in-process DEFINITION VECTOR — a command that can bind a name in the shell that runs
/// it (`28R:rul-defensive-mode-definition-vectors`)?
///
/// Deliberately NOT any-⊤: an unmodeled command is an external binary and cannot define a function in
/// the executing shell, so `hork` must never qualify.
///
/// **And deliberately not any ⊤-REJECT either**, which is the sharper trap. The rule's three named
/// vectors are `eval`, a computed `.` target, and an `alias`; the parser folds the first two into ONE
/// AST reason (`UnsupportedReason::DynamicExecution`) TOGETHER WITH a dynamic command name — and a
/// command-position `"$@"` is a dynamic command name, which is the defining tautology of every
/// peeling wrapper (`wrapper-law`). Keying on that reason therefore puts every wrapper oracle in the
/// world into defensive emission, which is `hork must not flip the mode` wearing a different costume
/// (measured on this tree: `context-entry-wrapped-guard` munged for no reason). The finer
/// `SyntaxUnsupportedReason` that tells the three apart is DIAGNOSTIC-only and does not ride the node.
///
/// What that leaves is honest rather than lossy: `eval` cannot reach an emission decision at all — it
/// is ERROR-tier in a book (the run refuses before a plan exists) and banned outright in an oracle
/// (`dialect-quality-law` · `declarations-only-files`) — and a computed `.` is the same. So the
/// reachable vector is `alias`, which parses as an ordinary command word, plus `funcenv`'s own
/// `unresolvable_loads` at the caller. `eval` stays listed for the day a body can carry one.
#[must_use]
pub fn is_definition_vector(name: &str) -> bool {
    matches!(name, "eval" | "alias")
}

/// Every definition vector the given sources carry, in name order — the whole-artifact question
/// behind DEFENSIVE emission. Empty is the overwhelming case, and empty means the artifact may ship
/// bare names.
#[must_use]
pub fn definition_vectors(srcs: &[&str]) -> BTreeSet<String> {
    srcs.iter()
        .flat_map(|src| called_names(src))
        .filter(|name| is_definition_vector(name))
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

/// Every literal command-position word in a body — the helper CANDIDATES, and the seat
/// [`is_definition_vector`] reads.
///
/// Over-collects on purpose: a candidate the index does not know is an external tool and is dropped.
/// Under-collecting is the dangerous direction (a missed helper ships a body that cannot run), so the
/// walk descends through every construct that can hold a command, command substitutions included. A
/// dynamic command word contributes nothing here because the parser ⊤-rejects it upstream
/// (`syntax/CLAUDE.md syntactic-top-triggers`) — and reading that ⊤-reject as a definition vector is
/// the trap [`is_definition_vector`] documents.
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
    use std::collections::BTreeSet;

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

    /// `command <name>` asks for the external UTILITY by that name, never for whatever function
    /// happens to be live — so it reaches no helper, closes cleanly, and denies nothing
    /// (`30I` §3.4 `deliberate-external-utility`). This is the escape hatch
    /// `unannounced-cross-custody-call` suggests, and the suggestion is only honest while
    /// the walk keeps treating the operand as an argument rather than a call.
    #[test]
    fn a_command_routed_utility_reaches_no_helper() {
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let entry =
            format!("{MARKER}wombat__is_converged() {{\n   command _wombat_check \"$1\"\n}}\n");
        let closure = index(&[&helpers, &entry])
            .closure_for(
                1,
                "wombat__is_converged() {\n   command _wombat_check \"$1\"\n}",
            )
            .expect("a command-routed utility is not a cross-custody reach");
        assert!(
            closure.decls().is_empty(),
            "the operand is an argument to `command`, never a helper the closure must carry"
        );
    }

    /// `28M` §8's two-file package shape, CO-LOADED: two files named on one command line, the
    /// helper in one and the entrypoint in the other. Co-loading is ingestion, so the entrypoint
    /// reaches an utterance nobody put in its custody and the vouch suspends
    /// (`rul-vouch-reaches-own-custody-only`). The package shape is not lost — it is spelled with a
    /// `.`, which is the act that takes custody; this test pins that co-loading alone never is.
    #[test]
    fn a_co_loaded_helper_leaves_the_entrypoints_custody() {
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let denied = index(&[&helpers, &entry])
            .closure_for(1, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect_err("co-loading composes no custody");
        assert_eq!(denied.name, "_wombat_check");
        assert_eq!(
            denied.reason,
            super::DenialReason::DependencyAmbientOrUntraceable
        );
    }

    /// ...and the SAME two files compose the moment the entrypoint SOURCES the helpers. This is
    /// the payoff of the whole sourcing build, stated as the smallest possible difference from the
    /// test above: the files are unchanged, only the `.` line and the include-tree it mints are
    /// added, and the vouch that suspended now lifts (`28M` §7's package shape, licensed).
    #[test]
    fn a_sourced_helper_is_inside_the_entrypoints_custody() {
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let entry = format!(
            "{MARKER}. ./helpers.sh\nwombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n"
        );
        let closure = HelperIndex::build(&[&helpers, &entry], None)
            .with_include_tree(
                dorc_core::CustodyClosures::from_edges(2, &[(1, 0)]),
                BTreeSet::new(),
            )
            .closure_for(1, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect("the `.` takes custody of the helpers file")
            .sh();
        assert!(closure.contains("_wombat_check() {"), "{closure}");
    }

    /// Custody does NOT flow back up: the helpers file's own vouch may not rest on its entrypoints.
    /// Sourcing is a promise the SOURCER makes, and the sourced author made none.
    #[test]
    fn a_sourced_file_does_not_reach_its_sourcer() {
        let helpers = format!("{MARKER}wombat__is_converged() {{\n   _entry_only \"$1\"\n}}\n");
        let entry =
            format!("{MARKER}. ./helpers.sh\n_entry_only() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let denied = HelperIndex::build(&[&helpers, &entry], None)
            .with_include_tree(
                dorc_core::CustodyClosures::from_edges(2, &[(1, 0)]),
                BTreeSet::new(),
            )
            .closure_for(0, "wombat__is_converged() {\n   _entry_only \"$1\"\n}")
            .expect_err("custody flows down the include-tree, never up");
        assert_eq!(
            denied.reason,
            super::DenialReason::DependencyAmbientOrUntraceable
        );
    }

    /// Two declarations inside ONE custody with DIFFERING bytes suspend rather than letting load
    /// order pick. The flat source vector cannot express how a file's own declarations interleave
    /// with the ones its `.` pulls in, so nothing may rest on that order — and with this suspension
    /// in place, nothing does.
    #[test]
    fn differing_bytes_within_one_custody_suspend() {
        let helpers = format!("{MARKER}_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let entry = format!(
            "{MARKER}. ./helpers.sh\n_check() {{\n   wombat verify -- \"$1\"\n}}\n\
             wombat__is_converged() {{\n   _check \"$1\"\n}}\n"
        );
        let denied = HelperIndex::build(&[&helpers, &entry], None)
            .with_include_tree(
                dorc_core::CustodyClosures::from_edges(2, &[(1, 0)]),
                BTreeSet::new(),
            )
            .closure_for(1, "wombat__is_converged() {\n   _check \"$1\"\n}")
            .expect_err("one custody, two bodies, no order the engine can promise is sh's");
        assert_eq!(denied.reason, super::DenialReason::ContestedWithinCustody);
    }

    /// A file that sourced something the driver could not load suspends every vouch it carries,
    /// including one reaching a helper it declares ITSELF. The body would run in an environment the
    /// engine never reconstructed, and a body that ignores a missing helper's status and answers 0
    /// from a later test reports converged off a helper that never ran.
    #[test]
    fn an_unresolved_load_suspends_the_whole_file() {
        let entry = format!(
            "{MARKER}. ./gone.sh\n_check() {{\n   wombat cmp -- \"$1\"\n}}\n\
             wombat__is_converged() {{\n   _check \"$1\"\n}}\n"
        );
        let denied = HelperIndex::build(&[&entry], None)
            .with_include_tree(
                dorc_core::CustodyClosures::singletons(1),
                BTreeSet::from([0]),
            )
            .closure_for(0, "wombat__is_converged() {\n   _check \"$1\"\n}")
            .expect_err("an unreconstructible environment vouches for nothing");
        assert_eq!(denied.reason, super::DenialReason::UnresolvedLoad);
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

    /// The CAPTURE half of `rul-vouch-reaches-own-custody-only`: version-skewed copies resolve by
    /// sh's own last-wins, and when the winner sits in the VOUCHER's own file the reach never left
    /// custody — the snapshot ships. The load edge still reports the collision, because the earlier
    /// author's helper really was rebound for every caller.
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

    /// The SUSPEND half: the same plurality, resolved into a file that is NOT the voucher's. The
    /// site reports every declaration it found, so the reader can see which files are in play.
    #[test]
    fn a_plural_helper_resolving_outside_the_vouchers_custody_suspends() {
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let a = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let b = format!("{MARKER}_wombat_check() {{\n   wombat cmp --strict -- \"$1\"\n}}\n");
        let denied = index(&[&entry, &a, &b])
            .closure_for(0, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect_err("the resolved body is somebody else's");
        assert_eq!(denied.name, "_wombat_check");
        assert_eq!(
            denied.reason,
            super::DenialReason::DependencyAmbientOrUntraceable
        );
        assert_eq!(
            denied.sites.iter().map(|(f, _)| *f).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    /// A SINGULAR cross-file reach is DE-LICENSED too (`rul-vouch-reaches-own-custody-only`). The
    /// retired composite let one lone declaration through on the reasoning that there was nothing
    /// to adjudicate, which mistook the absence of a dispute for the presence of custody: the
    /// engineer still vouched over bytes a stranger wrote and the CLI happened to hand them.
    #[test]
    fn a_singular_cross_custody_reach_is_de_licensed() {
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let denied = index(&[&entry, &helpers])
            .closure_for(0, "wombat__is_converged() {\n   _wombat_check \"$1\"\n}")
            .expect_err("one declaration in another custody is still another custody");
        assert_eq!(denied.name, "_wombat_check");
        assert_eq!(
            denied.reason,
            super::DenialReason::DependencyAmbientOrUntraceable
        );
    }

    /// `30I` §3.4's TWO NON-EXACT CASES, told apart, with the disposition held constant.
    ///
    /// Both worlds are one voucher reaching a helper a shell binds from outside its custody, and
    /// both suspend — same denial, same name, same sites, site runs. The only difference is the
    /// SENTENCE the author gets, and it is the difference between "your guarded dependency did not
    /// align" and "you named no dependency at all". Naming the first author in the second's words
    /// points them at a repair they already made, which is the pope-sin direction
    /// (`271:rul-sin-ordering`) and the whole reason the split exists.
    ///
    /// The engine sees them apart ONLY through the selection relation: the two runs below differ in
    /// nothing else, not one byte of source.
    #[test]
    fn selecting_a_dependency_changes_the_sentence_and_not_the_disposition() {
        let helpers = format!("{MARKER}_wombat_check() {{\n   wombat cmp -- \"$1\"\n}}\n");
        let entry = format!("{MARKER}wombat__is_converged() {{\n   _wombat_check \"$1\"\n}}\n");
        let body = "wombat__is_converged() {\n   _wombat_check \"$1\"\n}";
        let index = || {
            HelperIndex::build(&[&helpers, &entry], None)
                .with_include_tree(dorc_core::CustodyClosures::singletons(2), BTreeSet::new())
        };

        let ambient = index()
            .closure_for(1, body)
            .expect_err("co-loading composes no custody, selection or not");
        assert_eq!(
            ambient.reason,
            super::DenialReason::DependencyAmbientOrUntraceable
        );

        let unaligned = index()
            .with_selection(dorc_core::CustodyClosures::from_edges(2, &[(1, 0)]))
            .closure_for(1, body)
            .expect_err("selecting a dependency never mints custody");
        assert_eq!(
            unaligned.reason,
            super::DenialReason::DependencySelectedButUnaligned
        );
        assert_eq!(unaligned.name, ambient.name);
        assert_eq!(unaligned.sites, ambient.sites);
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

    /// The permanent WITHHELD tier: a body reaching a definition vector can bind or invoke a name no
    /// walk sees, so its snapshot cannot be closed. Not scaffolding — the bottom rung.
    #[test]
    fn an_unenumerable_call_withholds_permanently() {
        let src = format!(
            "{MARKER}_helper() {{\n   wombat cmp\n}}\n\
             wombat__is_converged() {{\n   alias wombat=hork\n   _helper\n}}\n"
        );
        let denied = index(&[&src])
            .closure_for(
                0,
                "wombat__is_converged() {\n   alias wombat=hork\n   _helper\n}",
            )
            .expect_err("a body that can rebind a name at parse time has no closable snapshot");
        assert_eq!(denied.reason, super::DenialReason::UnenumerableCall);
        assert_eq!(denied.name, "alias");
    }

    /// Defensive emission keys on real in-process definition vectors ONLY. TWO ways to get this
    /// wrong, and the second is the one that bit: an unmodeled command is an external binary and
    /// cannot bind a function in the executing shell (`hork`), and a ⊤-REJECT is not a vector either
    /// — a peeling wrapper's command-position `"$@"` is a dynamic command name, which the parser
    /// folds into the same AST reason as `eval`, so reading that reason would put every wrapper
    /// oracle into defensive emission (`28R:rul-defensive-mode-definition-vectors`).
    #[test]
    fn definition_vectors_ignore_unmodeled_commands_and_top_rejects() {
        assert!(super::definition_vectors(&["hork tune web\nwombat sync a\n"]).is_empty());
        assert!(
            super::definition_vectors(&["sudo__predict() {\n   env -i HOME=/root \"$@\"\n}\n"])
                .is_empty(),
            "a wrapper's own delegation is not a definition vector"
        );
        assert_eq!(
            super::definition_vectors(&["hork tune\n", "alias ls='ls -l'\n"])
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["alias".to_owned()]
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
