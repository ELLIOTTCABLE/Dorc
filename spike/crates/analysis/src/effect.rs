//! Per-command system-state effects + the ambient-invariant gate — the input the
//! skip decision consumes (this module decides *nothing* itself; it classifies).
//!
//! Two steps (note 163 §2):
//! 1. **effect resolution** — for each `Command` node, thread the book's
//!    flow-resolved argv (`analysis::value::ValueFlow`) through the oracle's own
//!    `check()` (`oracle::check::evaluate`) to its inline kind-annotation, then key
//!    the resulting `(verb, entity, kind)` into the oracle effect-map for the
//!    `(selector, polarity)` cells (`Establishes`/`Kills`, or `Opaque` on any ⊤).
//!    The engine parses NO argv itself — *identity is declared, never inferred*
//!    (`inv-referent-agnostic`); the old engine-side flag-strip stand-in is gone.
//! 2. **ambient gate** — a forward reaching-definitions pass over the mutated
//!    facts: a fact is *ambient* at a command iff NO upstream in-script command
//!    mutated it (so the host's resting state is authoritative and we may probe
//!    it). A fact mutated upstream is *written* — its resting value is stale —
//!    catching `apt-get purge X; … apt-get install X` (note 162 O-1 / break-10).
//!
//! Lock-style note (note 165): this module is deliberately **all forward-may**
//! (over-approximate), so there is no `May`/`Must` wrapper here yet — there is
//! nothing of the opposite orientation to confuse it with. That lock arrives with
//! the first *must* analysis (statically-definitely-established) and the backward
//! apply-slice. Here the only conservative direction is "when unsure ⇒ `Opaque`
//! ⇒ not ambient ⇒ run", which is safe for the skip decision.

use crate::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use crate::lattice::Lattice;
use crate::solve::{Direction, Graph, solve};
use crate::value::{ValueFlow, ValueOf};
use dorc_core::{
    Carrier, DiagCode, Diagnostic, EntityRef, Interner, KindId, OpaqueToken, ProviderId, Span, diag,
};
use dorc_oracle::check::{self, CheckSet, ResolvedEntity};
use dorc_oracle::{EffectCell, KindIndex, Polarity, empty_verb};
use std::collections::BTreeSet;

/// The dataflow fact-key the engine reaches over. **Re-exported from `core`**
/// (`dec-seam-ownership`, `notes/193` §2): the structured entity-algebra is the
/// shared vocabulary defined in `core` so `oracle`/`plan`/`hostsim` all key on one
/// type; `analysis` is its largest *consumer*, not a parallel owner. The flat
/// `(kind, entity)` pair of spike-1 became `core::FactKey { kind, entity:
/// EntityRef, selector }` — the per-entity selector is what kills the poison wall
/// (`apt-get update` ⇒ `package-index#fresh`, distinct from `install`'s
/// `package:nginx#installed`).
pub use dorc_core::FactKey;

/// What a command does to — or *observes about* — system state, as far as the
/// analyzer can determine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandEffect {
    /// No modeled system-state effect (a bare assignment, or a read-only command).
    Pure,
    /// Establishes `fact` (`apt-get install nginx`).
    Establishes(FactKey),
    /// Kills `fact` (`apt-get purge nginx`).
    Kills(FactKey),
    /// READS `fact` and mutates nothing — the read-only guard-class (`command -v
    /// nginx` ⇒ `Queries(tool:nginx#present)`; 202 §2 / task-D2). A `Query`
    /// **poisons no reaching-defs and establishes nothing** (the reaching-defs gen
    /// treats it like `Pure`): a guard reads state, it does not change it, so it must
    /// not force a downstream establish to `EstablishWritten` nor invalidate a
    /// downstream Query (st-3, 20A §4). Distinct from `Pure` only so a Query SITE is
    /// probe-resolvable (its check IS the probe) and its probed rc can feed the fold's
    /// Status channel (gated by rule-query-validity).
    Queries(FactKey),
    /// ⊤: cannot characterize — a ⊤ argv word, no provider/check, an `evaluate`
    /// `Top`, or no effect-map entry for the resolved verb. Conservatively MAY mutate
    /// anything (so it poisons downstream ambient-ness) and itself must run
    /// (`inv-top-reject`).
    Opaque,
}

/// Diagnostic code for a kind-disagreement between a check's annotation and the
/// effect-map (`ch-catalog`; 204 §6 open seam).
const KIND_DISAGREEMENT: DiagCode = DiagCode("effect-kind-disagreement");

/// Determine a `Command` node's effect cells from the book's resolved argv + the
/// oracle's own `check()` (the real entity-resolution mechanism; replaces the deleted
/// engine-side argparse stand-in). The engine parses NOTHING: it threads the
/// flow-resolved argv through the oracle's argparse (`check::evaluate`) and reads
/// the inline kind-annotation. *Identity is declared, never inferred* — true in
/// code now (`inv-referent-agnostic`).
///
/// Returns a `Vec` of cells (a multi-cell verb is legal — `us-effectmap`); each is
/// `Establishes`/`Kills`/`Queries` (`Queries` is the read-only guard-class, 202 §2).
/// ANY ⊤ — a ⊤ argv word, no provider, no check, an `evaluate` `Top`, or no
/// effect-map entry — yields `[Opaque]` (`inv-top-reject`: the degrade is the floor;
/// both `kFAIL` directions). A bare assignment yields `[Pure]`.
///
/// `inv-superposition`: the cells are phase-/orientation-agnostic facts; this
/// classifies, it decides nothing. Diagnostics (kind-disagreement) accumulate in
/// `diags`.
pub fn command_effect(
    idx: &KindIndex,
    checks: &[CheckSet],
    argv: &[ValueOf],
    interner: &mut Interner,
    diags: &mut Vec<Diagnostic>,
    site: Option<Span>,
) -> Vec<CommandEffect> {
    // A bare assignment-only command (`pkg=nginx`) has an empty argv ⇒ no
    // system-state effect (value::analyze yields `[]` for words.is_empty()).
    let Some(&word0) = argv.first() else {
        return vec![CommandEffect::Pure];
    };
    // The command word must be a concrete literal; a ⊤ word (`"$dyn" install …`) is
    // an un-modeled command ⇒ Opaque (`inv-top-reject`). The ⊤-degradation is no longer
    // silent (q-2 / find-3 no-silent-phantoms): disclose it as a Note (`dq-cmdsub-operand-top`).
    let ValueOf::Literal(provider_sym) = word0 else {
        diags.push(diag::cmdsub_operand_top(site, "the command word"));
        return vec![CommandEffect::Opaque];
    };
    let provider_str = interner.resolve(provider_sym).to_owned();
    // Target-state-pure shell builtins (shell-env/stdout/control only, never an
    // oracle-modeled fact) are Pure, not Opaque, so they do NOT poison downstream
    // ambient-ness (fs-4 / spec_set_e; note 16G §4 B). Anything not listed stays
    // Opaque (the safe over-refusing direction).
    if is_target_state_pure_builtin(&provider_str) {
        return vec![CommandEffect::Pure];
    }
    // The provider symbol: the book's command word through the SHARED hyphen↔underscore
    // convention (`check::map_provider_name`) — so it equals the `CheckSet` key and
    // `KindIndex`'s `ProviderId` (204 §6 seam #2). The book word is already hyphenated
    // (`apt-get`), so the map is a no-op here, but routing through the one helper keeps
    // the vocabularies welded.
    let provider = ProviderId(interner.intern(&check::map_provider_name(&provider_str)));

    // The trailing args (command word excluded — C-1) must ALL be concrete literals
    // to run the check (202 §1 fully-concrete-argv scope). A ⊤ hole ⇒ unresolved site
    // ⇒ Opaque (runs). Collect the resolved text, holding it so `&str` slices borrow
    // it for `evaluate`.
    let mut arg_texts: Vec<String> = Vec::with_capacity(argv.len().saturating_sub(1));
    for (i, word) in argv[1..].iter().enumerate() {
        match word {
            ValueOf::Literal(s) => arg_texts.push(interner.resolve(*s).to_owned()),
            // ⊤ arg ⇒ unresolved ⇒ Opaque; disclose WHICH operand went ⊤ (q-2, the
            // 1-based operand index excluding the command word — `dq-cmdsub-operand-top`).
            ValueOf::Top => {
                diags.push(diag::cmdsub_operand_top(
                    site,
                    &format!("operand {}", i + 1),
                ));
                return vec![CommandEffect::Opaque];
            }
        }
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    // Run the oracle's own argparse. Multiple oracle files may each declare a check
    // for this provider (`apt-get` install/purge in one file, `apt-get update` in
    // another — different kinds, different authors); try each and take the FIRST that
    // resolves concretely. A check that does not handle this verb falls through to its
    // own `Top` (no annotation reached / positional past end), so the partition is
    // clean for the corpus. (tc-*: if two checks both resolve, first-in-file-order
    // wins — flagged; no corpus case is ambiguous.) The `CheckSet` key is the same
    // provider symbol (interning is idempotent; `ProviderId` wraps it).
    let resolved = checks
        .iter()
        .filter_map(|cs| cs.get(provider.0))
        .find_map(|c| match check::evaluate(c, &arg_refs) {
            check::Resolution::Resolved(r) => Some(r),
            check::Resolution::Top(_) => None,
        });
    let Some(resolved) = resolved else {
        // No check resolved this site (no check for the provider, or every candidate
        // degraded to ⊤). ⊤ ⇒ Opaque (`inv-top-reject`). We do NOT fall back to a
        // verb-by-position lookup — that was the deleted engine-side argparse sin.
        return vec![CommandEffect::Opaque];
    };

    // The verb key: the check's derived verb, or the ε-verb when the check binds none
    // (`useradd`, `command -v` — 202 §2 / task-W §4). `evaluate`'s verb is compared
    // against the effect-map's verb through the SAME `Interner` (204 seam #2).
    let verb_key = match &resolved.verb {
        Some(v) => interner.intern(v),
        None => empty_verb(interner),
    };
    let cells = idx.effect_of(provider, verb_key);
    if cells.is_empty() {
        // The check resolved an identity, but no oracle declared an effect for this
        // (provider, verb). Not this analysis's concern ⇒ ⊤ (runs). A read-only guard
        // whose oracle declares `oracle_effect … query …` lands as `Queries` below;
        // only an un-declared guard falls through to Opaque here (task-D2, 202 §2).
        return vec![CommandEffect::Opaque];
    }

    // The cell's kind comes from the annotation (the declared identity, 204 §6); the
    // effect-map supplies selector + polarity per (provider, verb). Kind-agreement
    // (204 open seam): if a cell's effect-map kind disagrees with the annotation kind,
    // diagnose and let the ANNOTATION win (the effect-map row is re-keyed under it).
    let annotation_kind = KindId(interner.intern(&resolved.kind));
    let entity = match &resolved.entity {
        ResolvedEntity::Operand(text) => EntityRef::Operand(OpaqueToken(interner.intern(text))),
        ResolvedEntity::Singleton => EntityRef::Singleton,
    };
    // `EffectCell` is `Copy` and `cells` borrows `idx` (disjoint from `&mut interner`),
    // so iterate by copy — `cell_effect` takes `&mut interner` for the kind-agreement
    // diagnostic without conflicting with the `idx` borrow.
    cells
        .iter()
        .copied()
        .map(|cell| {
            cell_effect(
                cell,
                annotation_kind,
                &resolved.kind,
                entity,
                interner,
                diags,
            )
        })
        .collect()
}

/// Resolve an in-loop Members site to its establish-cell FAMILY (task-L2 item-2), or
/// `None` if it is not a Members site OR any member fails to resolve to a single
/// establish (ALL-OR-NOTHING — the family is never partial). For each per-member argv
/// ([`crate::value::ValueFlow::member_argv`]) run the oracle's own `check()` exactly as a
/// straight-line command; require `[CommandEffect::Establishes(fact)]` for EVERY member.
/// Any member that is Opaque (a ⊤ word, no check, the check refuses), a Kill, a Query, a
/// Pure, or a multi-cell verb ⇒ the whole site is `None` (it falls back to the single-cell
/// classification, which for an in-loop site is the render-floored Flat path ⇒ `MustRun`).
///
/// The kind-disagreement diagnostics each member's check may raise accumulate in `diags`
/// (shared with the straight-line path). Deterministic; never panics (`inv-no-throw`).
fn member_family(
    id: CfgNodeId,
    cfg: &Cfg,
    value: &ValueFlow,
    idx: &KindIndex,
    checks: &[CheckSet],
    interner: &mut Interner,
    diags: &mut Vec<Diagnostic>,
) -> Option<Vec<FactKey>> {
    if cfg.node(id).kind != CfgNodeKind::Command {
        return None;
    }
    let members = value.member_argv(id)?;
    let mut family = Vec::with_capacity(members.len());
    for argv in members {
        // Each member is a normal concrete argv; resolve it through the oracle check.
        // All-or-nothing: ANY non-single-establish member kills the whole family.
        // `site: None` for the ⊤-disclosure (q-2): a ⊤ member collapses the family ⇒ the
        // site falls back to the single-cell `argv` classification below, which re-runs
        // `command_effect` with the REAL span and discloses there — emitting here too would
        // double-report the same site.
        match command_effect(idx, checks, argv, interner, diags, None).as_slice() {
            [CommandEffect::Establishes(fact)] => family.push(*fact),
            _ => return None,
        }
    }
    // An empty family cannot arise (a Members site has ≥1 member), but guard defensively.
    if family.is_empty() {
        return None;
    }
    Some(family)
}

/// Build one [`CommandEffect`] from a declared [`EffectCell`] under the resolved
/// (annotation-kind, entity). Enforces the kind-agreement rule (204 §6): the
/// annotation is the declared identity, so on a mismatch the cell is re-keyed under
/// the annotation kind and a warning is recorded.
fn cell_effect(
    cell: EffectCell,
    annotation_kind: KindId,
    annotation_kind_str: &str,
    entity: EntityRef,
    interner: &mut Interner,
    diags: &mut Vec<Diagnostic>,
) -> CommandEffect {
    if cell.kind != annotation_kind {
        let em_kind = interner.resolve(cell.kind.0).to_owned();
        diags.push(Diagnostic::warning(
            KIND_DISAGREEMENT,
            None,
            format!(
                "check annotation kind `{annotation_kind_str}` disagrees with the effect-map \
                 kind `{em_kind}` for this verb — the annotation (declared identity) wins"
            ),
        ));
    }
    let fact = FactKey {
        kind: annotation_kind, // the annotation wins (declared identity)
        entity,
        selector: cell.selector,
    };
    match cell.polarity {
        Polarity::Establish => CommandEffect::Establishes(fact),
        Polarity::Kill => CommandEffect::Kills(fact),
        Polarity::Query => CommandEffect::Queries(fact),
    }
}

/// Shell builtins with no *target-system* (location-3) effect: they change shell
/// options/cwd/variables or write to stdout/stderr, but never a package/file/
/// service fact an oracle models. Treated as `Pure` so they don't poison
/// reaching-defs ambient-ness (note 16G). Deliberately small and conservative —
/// anything not listed stays `Opaque` (the safe over-refusing direction); the
/// dynamic-lvalue forms (`unset "$x"`, `printf -v`) are already ⊤-rejected upstream
/// by the parser, so only their static uses reach here.
fn is_target_state_pure_builtin(word: &str) -> bool {
    matches!(
        word,
        "set"
            | "cd"
            | "export"
            | "unset"
            | "shift"
            | "read"
            | "readonly"
            | "local"
            | ":"
            | "true"
            | "false"
            | "echo"
            | "printf"
            | "test"
            | "["
    )
}

/// The per-path `file` cell a write-redirect (`>`/`>>`) to `path` establishes (y-1,
/// redirect-effects, `21F` imp-1). Follows the existing FactKey/kind vocabulary: a
/// blessed `file` kind (core: the Tier-A well-known kind names include `file`), the
/// resolved path as the entity operand (referent-agnostic — the path is an interned
/// token, never decoded beyond the syntactic `/dev/null` exemption at resolution), and
/// a single `written` selector (append vs truncate are BOTH write-shaped ⇒ the same
/// cell this round; no read-back / content discrimination). The cell GENS into
/// reaching-defs (so it poisons ambient-ness + invalidates a downstream Query, st-3),
/// but a `file` cell has no oracle/probe ⇒ it never licenses an elision (the charter's
/// "gen and poison, nothing licenses" — a `Redir` node is never a plan leaf anyway).
fn file_write_cell(path: dorc_core::Symbol, interner: &mut Interner) -> FactKey {
    FactKey {
        kind: KindId(interner.intern("file")),
        entity: EntityRef::Operand(OpaqueToken(path)),
        selector: dorc_core::SelectorId(interner.intern("written")),
    }
}

/// Render a resolved argv to display text for a diagnostic (q-2): each literal
/// resolved to its text, a `⊤` word shown as `⟨⊤⟩`. Display/provenance only — never
/// branched on (`inv-referent-agnostic`). Deterministic.
fn render_argv(argv: &[ValueOf], interner: &Interner) -> String {
    argv.iter()
        .map(|w| match w {
            ValueOf::Literal(s) => interner.resolve(*s).to_owned(),
            ValueOf::Top => "⟨⊤⟩".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Facts mutated by some command on a path to here — or `Top` once an `Opaque`
/// command has run (then ANY fact may have changed). This is the reaching-defs
/// lattice; a fact is ambient at a point iff it is NOT in the in-state here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reach {
    Facts(BTreeSet<FactKey>),
    Top,
}

impl Lattice for Reach {
    fn bottom() -> Self {
        Reach::Facts(BTreeSet::new())
    }
    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (Reach::Top, _) | (_, Reach::Top) => Reach::Top,
            (Reach::Facts(a), Reach::Facts(b)) => Reach::Facts(a.union(b).copied().collect()),
        }
    }
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // `Top` is the join-absorbing ⊤, hence meet's identity (`⊤ ⊓ x = x`).
            (Reach::Top, x) | (x, Reach::Top) => x.clone(),
            (Reach::Facts(a), Reach::Facts(b)) => {
                Reach::Facts(a.intersection(b).copied().collect())
            }
        }
    }
}

impl Reach {
    fn with(&self, fact: FactKey) -> Reach {
        match self {
            Reach::Top => Reach::Top,
            Reach::Facts(s) => {
                let mut s = s.clone();
                s.insert(fact);
                Reach::Facts(s)
            }
        }
    }
    fn mutated(&self, fact: &FactKey) -> bool {
        match self {
            Reach::Top => true,
            Reach::Facts(s) => s.contains(fact),
        }
    }

    /// Is this a **pristine** reaching-state — NO write-or-unknown reached here? The
    /// rule-query-validity bit (205 §2 / 20A §4 st-3): a Query's probed rc is
    /// fold-usable iff no invalidating command (an oracled MUTATOR — any
    /// Establish/Kill — or an Opaque) reaches the guard from entry. Because Queries
    /// and pure builtins gen nothing into `Reach`, "no write-or-unknown reached" is
    /// exactly the empty (⊥) fact-set; a non-empty set (some mutator genned a cell) or
    /// `Top` (an opaque ran) is non-pristine ⇒ the guard's resting rc is stale.
    fn is_pristine(&self) -> bool {
        matches!(self, Reach::Facts(s) if s.is_empty())
    }
}

/// How a `Command` relates to the skip decision. This is the *input* the probe/
/// plan stage consumes — it does not skip anything itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipClass {
    /// Not an elidable establish — opaque, pure, kill, unrecognized, OR an
    /// establish whose reaching-context cannot be trusted: unreachable from entry
    /// (e.g. a function body with no modeled call-edge), or produced under a
    /// non-converged solve ⇒ run.
    MustRun,
    /// Establishes `fact`, ambient here (no upstream mutation) ⇒ probe the host.
    EstablishAmbient(FactKey),
    /// Establishes `fact`, but the fact was mutated upstream in-script ⇒ the
    /// resting probe is not authoritative ⇒ run (or reason in-script; conservatively
    /// run). The `purge X; … install X` case.
    EstablishWritten(FactKey),
    /// A read-only **Query** guard reading `fact` (`command -v nginx` ⇒
    /// `tool:nginx#present`; 202 §2 / task-D2). Probe-resolvable like an
    /// `EstablishAmbient` (its check IS the probe), but its probed rc feeds the
    /// fold's **Status** channel rather than gating a mutation-elision — and ONLY
    /// when [`valid`](SkipClass::QueryResolvable::valid) holds.
    ///
    /// `valid` is the rule-query-validity bit (205 §2 / 20A §4 st-3): the guard's
    /// probe-time rc is fold-usable IFF NO invalidating command reaches the guard
    /// from entry — invalidating = an oracled MUTATOR (any Establish/Kill) or an
    /// Opaque; NOT invalidating = other Queries or blessed-pure builtins. When
    /// `valid == false` the guard's resting rc is stale (a mutator may have changed
    /// the cell), so the phased caller withholds the rc (status ⇒ ⊤) and the guard
    /// runs for real at apply — never a stale fold (`inv-superposition`: the bit is a
    /// phase-agnostic fact; the collapse stays in the caller).
    QueryResolvable { fact: FactKey, valid: bool },
    /// An in-loop **Members** establish site (task-L2 item-2, `209` brk-1(b)): the
    /// for-var is Members-bound and this body site's argv references it, so the site
    /// evaluates the check ONCE PER MEMBER ([`crate::value::ValueFlow::member_argv`])
    /// ⇒ a fact-FAMILY — one cell per member, in list order (duplicates kept). Each
    /// member is a normal concrete establish.
    ///
    /// ALL-OR-NOTHING at resolution (item-2): if ANY member's per-member argv fails to
    /// resolve to a single-establish cell (a ⊤ word, the check refuses, a multi-cell
    /// verb, …) the WHOLE site is `MustRun` (the family is never partial).
    ///
    /// `self_reached` is the item-3(b) **self-reach** bit (the subtle core of the license),
    /// a phase-agnostic engine fact (`inv-superposition`): the ONLY in-script writers
    /// reaching this site (including via the loop back-edge) are THIS leaf's own per-member
    /// establishes — no pre-loop writer, no in-loop sibling, no Opaque (⊤) reached it. The
    /// license (item-3, in `plan`) may elide the body ONLY when `self_reached` AND every
    /// member is Converged AND the consumption gates pass. RATIONALE (the fixed-point
    /// argument, preserved at the license site): the elision's own effect removes the
    /// body's writes, so under the elision the resting probe stays authoritative
    /// (elide-all is self-consistent); ANY non-self writer breaks that argument ⇒ refuse.
    EstablishMembers {
        members: Vec<FactKey>,
        self_reached: bool,
    },
    /// An inlined function-CALL site (arch-2, brk-2, `i-3`/`i-4`): the call's command word
    /// resolved to a same-file-earlier funcdef and its body was spliced into the CFG. This is
    /// the render/substitution LEAF (the call's own span); the spliced body commands are
    /// `spliced_internal` (not their own leaves). It carries the per-body-site classifications
    /// (`sites`, one [`InlineSite`] per effect-bearing/probeable body leaf, in source order) so
    /// the all-or-nothing CALL license (`plan`) can aggregate them and the probe can ship a
    /// `site N.M` sub-record per site.
    ///
    /// ALL-OR-NOTHING (the Members precedent, 20S): the call licenses a `Replace` ONLY when
    /// EVERY effect-bearing body site licenses elision — every body Establish/Kill is an
    /// `EstablishAmbient` whose fact is Converged (a body Kill, an Opaque, a ⊤, or a non-self/
    /// written establish blocks the WHOLE call), Queries pass their own gates, and the CALL
    /// site's own consumed channels are reproduced. One non-licensing body leaf ⇒ the call
    /// RUNS (the real body executes). No partial-body render ever (`i-3`).
    InlineCall { sites: Vec<InlineSite> },
}

/// arch-2 (`i-3`/`i-4`): one spliced funcdef-body LEAF site under an [`SkipClass::InlineCall`].
/// Carries the body command's CFG node (provenance + the plan's `has_top_successor` check) and
/// its own [`SkipClass`] classification (resolved with the call's positional bindings — `i-2`).
/// The plan aggregates these into the all-or-nothing CALL license; the probe ships one
/// `site N.M` sub-record per site (M = the index into the call's site list).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineSite {
    /// The spliced body command's CFG node (provenance; `has_top_successor` gate; never a Step
    /// leaf of its own — the CALL is the render unit, `i-3`).
    pub node: CfgNodeId,
    /// The body site's own classification (with the call's positionals bound, `i-2`): an
    /// `EstablishAmbient`/`QueryResolvable` is per-site probeable (`site N.M`); a Kill/Opaque/
    /// MustRun/EstablishWritten blocks the whole call.
    pub class: SkipClass,
}

/// Nodes reachable from the program `entry` by forward edges. The reaching-defs
/// in-state of an *unreachable* node is a vacuous ⊥ (its only predecessors, if
/// any, are themselves unreached), which is indistinguishable from a genuinely
/// clean "nothing upstream mutated this fact" — so [`classify`] must not read an
/// unreachable establish as ambient (find-A). Today the only unreachable
/// `Command`s are function bodies: a call site has no modeled call-edge into the
/// body (cfg `find-7`), so the body is a detached island. A simple forward
/// graph reachability; deterministic and total (indices are in-bounds by
/// construction, so it never panics — `inv-no-throw`).
fn reachable_from_entry(cfg: &Cfg) -> Vec<bool> {
    let mut seen = vec![false; cfg.node_count()];
    let mut stack = vec![cfg.entry()];
    seen[cfg.entry().index()] = true;
    while let Some(v) = stack.pop() {
        for w in cfg.succ_ids(v) {
            if !seen[w.index()] {
                seen[w.index()] = true;
                stack.push(w);
            }
        }
    }
    seen
}

/// The reaching-defs transfer: `out = in ⊔ gen(node)`, with an optional `suppress` node
/// whose gen is SKIPPED (task-L2 item-3(b) self-reach). Each cell gens its fact; an Opaque
/// joins ⊤; Pure/Queries gen nothing. Suppressing a Members site's gen (its self-
/// establishes) lets the back-edge carry only OTHER writers' cells to its in-state, so a
/// pristine result there proves only-self-reaches. Pure; monotone (a smaller gen set is
/// still monotone) and finite-height ⇒ `solve` converges.
fn reach_transfer(
    effects: &[Vec<CommandEffect>],
    incoming: &Reach,
    node: usize,
    suppress: Option<usize>,
) -> Reach {
    if suppress == Some(node) {
        return incoming.clone();
    }
    let mut state = incoming.clone();
    for cell in &effects[node] {
        state = match cell {
            CommandEffect::Establishes(f) | CommandEffect::Kills(f) => state.with(*f),
            CommandEffect::Opaque => state.join(&Reach::Top),
            CommandEffect::Pure | CommandEffect::Queries(_) => state,
        };
    }
    state
}

/// The effect cells one CFG node gens into the reaching-defs (the per-node closure body of
/// [`classify`], extracted so `classify` stays under the line cap). A resolved Members site
/// gens its per-member establishes; an inlined CALL gens `Pure` (the spliced body carries the
/// effects); a `Command` resolves through the oracle check; a `Top` node is `Opaque`; a
/// WRITE-shaped `Redir` gens a per-path `file` cell (y-1) or `Opaque`+disclosure on a ⊤
/// target; everything else is `Pure`. Diagnostics (kind-disagreement, the q-2/y-1 ⊤
/// disclosures) accumulate in `diags`. Deterministic; never panics (`inv-no-throw`).
#[expect(
    clippy::too_many_arguments,
    reason = "extracted verbatim from classify's per-node closure to stay under the line cap; \
              the args are the closure's captured inputs (cfg/value/idx/checks/interner/diags)"
)]
fn node_effects(
    id: CfgNodeId,
    member_family: Option<&Vec<FactKey>>,
    cfg: &Cfg,
    value: &ValueFlow,
    idx: &KindIndex,
    checks: &[CheckSet],
    interner: &mut Interner,
    diags: &mut Vec<Diagnostic>,
) -> Vec<CommandEffect> {
    if let Some(family) = member_family {
        return family
            .iter()
            .map(|f| CommandEffect::Establishes(*f))
            .collect();
    }
    // arch-2: an inlined CALL gens Pure, NOT the Opaque its unmodeled word would resolve to —
    // the body (spliced after it) carries the effects. Opaque here would poison the call's OWN
    // spliced body (the establish reads Written) — the very poison the splice removes.
    if cfg.call_body_sites(id).is_some() {
        return vec![CommandEffect::Pure];
    }
    match cfg.node(id).kind {
        CfgNodeKind::Command => {
            let argv = value.argv_values(id);
            // `site: None` — `classify` holds the CFG, not the AST, so a source span is not
            // cheaply reachable here; the q-2 disclosure names the ⊤ POSITION (`the command
            // word` / `operand N`), which is what the cli `report()` surfaces (no spans).
            command_effect(idx, checks, &argv, interner, diags, None)
        }
        // An unmodeled construct may mutate anything ⇒ ⊤.
        CfgNodeKind::Top => vec![CommandEffect::Opaque],
        // y-1 (redirect-effects): a WRITE-shaped redirect (`>`/`>>`) to a real sink is a
        // file-write EFFECT — previously invisibly `Pure`, which MASKED a downstream guard
        // reading the just-written file (`21F` imp-1: a `printf >> f` before a `grep`-guard of
        // `f` minted a stale-guard elision). A resolved target gens a per-path `file` cell (a
        // WRITER ⇒ st-3's coarse invalidation makes a downstream Query non-pristine ⇒ `valid:
        // false`); a ⊤ target joins ⊤ (Opaque-poison) + a disclosure. A non-write redirect
        // (read, fd-dup, here-doc, `/dev/null`) is absent from `redir_target` ⇒ stays Pure.
        CfgNodeKind::Redir => match value.redir_target(id) {
            Some(ValueOf::Literal(path)) => {
                vec![CommandEffect::Establishes(file_write_cell(path, interner))]
            }
            Some(ValueOf::Top) => {
                diags.push(diag::redir_target_top(None));
                vec![CommandEffect::Opaque]
            }
            None => vec![CommandEffect::Pure],
        },
        _ => vec![CommandEffect::Pure],
    }
}

/// Does the item-3(b) **self-reach** condition hold at the Members site `site`? Re-solve
/// the reaching-defs with `site`'s own gen suppressed and check the site's in-state is
/// pristine (the empty fact-set, NOT ⊤). With the self-establish removed, the in-state is
/// exactly the cells written by OTHER reaching paths (pre-loop, in-loop sibling, or an
/// Opaque ⇒ ⊤); pristine ⟺ ONLY this leaf's own establishes reach it. A non-converged
/// suppressed solve ⇒ `false` (conservative refuse — the safe direction). This is a small
/// extra solve per Members site (≤ a handful per book; perf is network-dominated anyway).
fn self_reach_holds(cfg: &Cfg, effects: &[Vec<CommandEffect>], site: usize) -> bool {
    let sol = solve(cfg, Direction::Forward, |i, incoming: &Reach| {
        reach_transfer(effects, incoming, i, Some(site))
    });
    sol.converged && sol.states.get(site).is_some_and(Reach::is_pristine)
}

/// Classify every `Command` node for the skip decision: resolve each command's
/// effect cells (through the book's value-flow [`ValueFlow`] + the oracle's own
/// `check()`), then a forward reaching-defs pass tells us, per establishing command,
/// whether its fact is ambient. An establish is only offered as `EstablishAmbient`
/// when its reaching-context is *trustworthy* — reachable from entry AND under a
/// converged solve; otherwise it folds to the safe `MustRun` (find-A/find-B).
///
/// `value` is the book-side value-flow (`analysis::value::analyze`, the caller
/// threads it); `checks` are the per-oracle-file `CheckSet`s (the engine parses no
/// argv itself — `inv-referent-agnostic`). Returns a [`Carrier`] so kind-disagreement
/// warnings (204 §6) surface. Deterministic; never panics (`inv-no-throw`).
#[must_use]
pub fn classify(
    cfg: &Cfg,
    value: &ValueFlow,
    idx: &KindIndex,
    checks: &[CheckSet],
    interner: &mut Interner,
) -> Carrier<Vec<(CfgNodeId, SkipClass)>> {
    let n = cfg.node_count();
    let mut diags: Vec<Diagnostic> = Vec::new();
    // task-L2 item-2: per in-loop Members site, resolve its per-member argv family to a
    // family of establish cells (`None` if not a Members site or any member fails to
    // resolve — all-or-nothing). Computed FIRST so `effects` can gen the member cells into
    // the reaching-defs (a resolved Members site gens its member cells, NOT Opaque — else
    // its own back-edge would poison its in-state to ⊤ and break item-3's self-reach).
    let member_families: Vec<Option<Vec<FactKey>>> = (0..n)
        .map(|i| {
            let id = CfgNodeId(i as u32);
            member_family(id, cfg, value, idx, checks, interner, &mut diags)
        })
        .collect();

    // Precompute each node's effect cells once (interning happens here, with &mut).
    // A multi-cell verb yields several cells; the reaching-defs gen applies each. A
    // resolved Members site's cells are its per-member establishes (item-2), so the
    // reaching-defs writes exactly those member cells (its self-establishes), keeping its
    // own in-state pristine-of-others for item-3's self-reach carve-out.
    let effects: Vec<Vec<CommandEffect>> = (0..n)
        .map(|i| {
            node_effects(
                CfgNodeId(i as u32),
                member_families[i].as_ref(),
                cfg,
                value,
                idx,
                checks,
                interner,
                &mut diags,
            )
        })
        .collect();

    // Forward reaching-defs: out = in ⊔ gen(node). Each of a node's cells is genned
    // (a multi-cell verb writes every cell); an Opaque cell joins ⊤. A `Queries` cell
    // gens NOTHING — a read poisons no ambient-ness and invalidates no downstream
    // Query (it is a write-free observation; task-D2 / st-3, 20A §4). This is the
    // gen-side of rule-query-validity: because a Query gens nothing, `reach.states`
    // (the IN-state at each node) carries exactly the writes-or-opaque that reached it.
    let reach = solve(cfg, Direction::Forward, |i, incoming: &Reach| {
        reach_transfer(&effects, incoming, i, None)
    });
    debug_assert!(
        reach.converged,
        "reaching-defs must converge (finite fact set)"
    );

    // Two reasons the reaching in-state cannot be trusted to mean "nothing
    // upstream mutated this fact", both folding the safe way (→ MustRun):
    //   * non-convergence (find-B): a capped solve returns a partial
    //     under-approximation — a real upstream kill may not have propagated. The
    //     `Reach` lattice is monotone + finite-height so this never trips here (the
    //     `debug_assert` catches a regression loudly), but trusting a non-converged
    //     state in *release* would be a silent wrong-skip, so guard it explicitly.
    //   * unreachability (find-A): an establish unreachable from entry has a vacuous
    //     ⊥ in-state; its true call context is unmodeled (cfg find-7).
    let trust_reach = reach.converged;
    let reachable = reachable_from_entry(cfg);

    // The per-site single-fact / member classification (the shared core, used by both the
    // ordinary leaf path below and the arch-2 inlined-call body-site aggregation). Reads only
    // already-computed state (`effects`, `member_families`, `reach`, `trust_reach`,
    // `reachable`), so it is a pure closure.
    let classify_site = |i: usize| -> SkipClass {
        // task-L2: a resolved in-loop Members site (reachable + converged) ⇒ EstablishMembers.
        if let Some(family) = &member_families[i]
            && trust_reach
            && reachable[i]
        {
            let self_reached = self_reach_holds(cfg, &effects, i);
            return SkipClass::EstablishMembers {
                members: family.clone(),
                self_reached,
            };
        }
        match effects[i].as_slice() {
            [CommandEffect::Establishes(f)] if trust_reach && reachable[i] => {
                if reach.states[i].mutated(f) {
                    SkipClass::EstablishWritten(*f)
                } else {
                    SkipClass::EstablishAmbient(*f)
                }
            }
            [CommandEffect::Queries(f)] if trust_reach && reachable[i] => {
                SkipClass::QueryResolvable {
                    fact: *f,
                    valid: reach.states[i].is_pristine(),
                }
            }
            _ => SkipClass::MustRun,
        }
    };

    let mut out = Vec::new();
    for (i, cells) in effects.iter().enumerate() {
        let id = CfgNodeId(i as u32);
        // Only genuinely-runnable command leaves are plan/apply units. A command
        // inside a `$( … )` substitution body is effect-bearing (it stayed in the
        // reaching-defs above, so its mutations still poison/establish) but is NOT
        // a leaf (find-cli-1, the dn-3 leaf-seam). arch-2: a SPLICED funcdef-body command is
        // likewise effect-bearing-but-not-a-leaf — its `site N.M` record rides the CALL (below).
        if cfg.node(id).kind == CfgNodeKind::Command && cfg.is_expansion_internal(id) {
            // q-2 (`dq-cmdsub-inner-nonleaf`, the `exec-subst-body-nonleaf` disclosure): an
            // EFFECT-BEARING `$()`-internal command runs un-elidably (it has no leaf of its
            // own, so it executes whenever its enclosing line runs). Today this is silent
            // (`219` q-1.f). A Pure inner command discloses nothing (nothing un-elidable
            // happens), so gate on a non-Pure effect.
            if cells.iter().any(|e| *e != CommandEffect::Pure) {
                diags.push(diag::cmdsub_inner_nonleaf(
                    None,
                    &render_argv(&value.argv_values(id), interner),
                ));
            }
            continue;
        }
        if cfg.node(id).kind != CfgNodeKind::Command || cfg.is_spliced_internal(id) {
            continue;
        }
        // arch-2 (`i-3`/`i-4`): an inlined CALL node aggregates its spliced body sites'
        // classifications into one `InlineCall` (the all-or-nothing license + per-site probe
        // sub-records live in `plan`). The body sites are classified with the call's
        // positionals bound (the value plane resolved their argv, `i-2`).
        if let Some(body_sites) = cfg.call_body_sites(id) {
            let sites = body_sites
                .iter()
                .map(|&site| InlineSite {
                    node: site,
                    class: classify_site(site.index()),
                })
                .collect();
            out.push((id, SkipClass::InlineCall { sites }));
            continue;
        }
        out.push((id, classify_site(i)));
    }
    Carrier { value: out, diags }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg;
    use crate::value::analyze;
    use dorc_core::{KindId, SelectorId};
    use dorc_oracle::check::lift_checks;

    /// The shared corpus-shaped check dialect the classify tests lift: an `apt-get`
    /// check (flag-strip → verb → per-verb arm: `update` ⇒ a Singleton `package-index`
    /// annotation; everything else ⇒ a post-verb flag-strip, the single-operand
    /// `package` annotation, and a `[ "$2" = "" ]` guard that refuses a SECOND operand
    /// — `install nginx curl` reaches no probe ⇒ Top ⇒ runs), plus a `systemctl` check
    /// (verb → per-arm probe). Annotation kinds MATCH the effect-map's (`package`,
    /// `package-index`, `service`) so the kind-agreement rule never fires. The probe
    /// bodies are inert placeholders (this round resolves identity only).
    ///
    /// Lifted with the CALLER's interner (`i`), so the [`CheckSet`]'s provider symbol
    /// equals the one `classify` interns from the book's command word (Symbols only
    /// compare across one interner — 204 seam #2).
    const CORPUS_CHECK_SRC: &str = r#"
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      update) idx : package-index; probe-fresh ;;
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then probe-pkg "$pkg"; fi ;;
   esac
}
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable) probe-enabled "$svc" ;;
      start)  probe-active "$svc" ;;
      disable) probe-enabled "$svc" ;;
   esac
}
command__check() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1
}
"#;

    /// The interned ids a package-fixture test asserts against. Kept together so a
    /// test reads `s.installed` / `s.nginx` instead of re-interning inline.
    struct Syms {
        package: KindId,
        package_index: KindId,
        installed: SelectorId,
        fresh: SelectorId,
    }

    /// Build (interner, index, syms) modeling the package oracle's effects — now
    /// *including* `apt-get update → (package-index, #fresh)`, the modeled nullary
    /// that the poison-wall fix relies on (`notes/193` §1). `install`/`purge` gate
    /// the `#installed` selector of `package`.
    fn package_setup() -> (Interner, KindIndex, Syms) {
        let mut interner = Interner::default();
        let package = KindId(interner.intern("package"));
        let package_index = KindId(interner.intern("package-index"));
        let installed = SelectorId(interner.intern("installed"));
        let fresh = SelectorId(interner.intern("fresh"));
        let apt = ProviderId(interner.intern("apt-get"));
        let install = interner.intern("install");
        let purge = interner.intern("purge");
        let update = interner.intern("update");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, Polarity::Establish);
        idx.add_effect(apt, purge, package, installed, Polarity::Kill);
        idx.add_effect(apt, update, package_index, fresh, Polarity::Establish);
        (
            interner,
            idx,
            Syms {
                package,
                package_index,
                installed,
                fresh,
            },
        )
    }

    /// `package:<entity>#installed` — the cell `apt-get install <entity>` gates.
    fn pkg_installed(i: &mut Interner, s: &Syms, entity: &str) -> FactKey {
        FactKey {
            kind: s.package,
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: s.installed,
        }
    }

    /// Run the full pipeline on `src` (value-flow + the corpus checks + classify) and
    /// return just the [`SkipClass`]es, in classify order. Everything shares one
    /// interner so the [`CheckSet`]'s provider symbols match the book's command words.
    fn classify_src(src: &str, interner: &mut Interner, idx: &KindIndex) -> Vec<SkipClass> {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, interner);
        let checks = vec![lift_checks(interner, CORPUS_CHECK_SRC).value];
        classify(&built.value, &value, idx, &checks, interner)
            .value
            .into_iter()
            .map(|(_, c)| c)
            .collect()
    }

    /// Like [`classify_src`] but return the classify-stage diagnostics (the q-2 emit-site
    /// pins): the codes a `$()`/⊤ book discloses.
    fn classify_src_diags(src: &str, interner: &mut Interner, idx: &KindIndex) -> Vec<Diagnostic> {
        let parsed = dorc_syntax::parse(src);
        let built = cfg::build(&parsed.value);
        let value = analyze(&built.value, &parsed.value, interner);
        let checks = vec![lift_checks(interner, CORPUS_CHECK_SRC).value];
        classify(&built.value, &value, idx, &checks, interner).diags
    }

    fn has_code(diags: &[Diagnostic], code: &str) -> bool {
        diags.iter().any(|d| d.code.0 == code)
    }

    #[test]
    fn lone_install_is_ambient() {
        // Why: the simplest establish with nothing upstream — must be probe-able
        // (EstablishAmbient), the precondition for ever skipping it.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get install nginx", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))]
        );
    }

    #[test]
    fn upstream_purge_makes_install_written() {
        // Why (note 162 O-1 / break-10, THE wrong-skip): an upstream same-run kill
        // means the resting probe is stale — the install must NOT be treated as
        // ambient/skippable. purge + install gate the SAME (package:nginx#installed)
        // cell, so the kill reaches the establish.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        // purge ⇒ MustRun (a kill, not an elidable establish); install ⇒ Written.
        assert!(classes.contains(&SkipClass::EstablishWritten(pkg_installed(
            &mut i, &s, "nginx"
        ))));
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn opaque_upstream_poisons_ambientness() {
        // Why (note 162 O-3, the precision COST being surfaced): a genuinely
        // unrecognized command (`ufw allow` — NO oracle entry) is still Opaque ⇒ ⊤ ⇒
        // it conservatively poisons every downstream fact's ambient-ness. The re-key
        // does NOT rescue an un-oracled neighbor; it rescues a *modeled* nullary
        // (`apt-get update`, the `poison_wall_dies_*` test below). This documents the
        // residual, correct cost so we still feel it.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("ufw allow 80/tcp\napt-get install nginx", &mut i, &idx);
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_)))
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn poison_wall_dies_modeled_update_does_not_poison_install() {
        // THE keystone win (`notes/193` §1 / acceptance §7.2): a modeled `apt-get
        // update` establishes a *distinct cell* (`package-index#fresh`), so it no
        // longer poisons the `apt-get install nginx` below it. Before the re-key,
        // `update` was doubly-unkeyable (no operand, and — pre-modeling — no verb) ⇒
        // Opaque ⇒ Reach::Top ⇒ install forced EstablishWritten. Now it's ambient.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get update\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "modeled `update` (distinct cell) must leave install EstablishAmbient: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no Written: update's cell (package-index#fresh) ≠ install's (package:nginx#installed)"
        );
    }

    #[test]
    fn genuine_same_cell_kill_still_forces_written() {
        // exclusion-check (`notes/193` §7.3): the re-key must NOT over-loosen the
        // ambient gate. A real same-cell kill (`purge nginx; install nginx`, both on
        // package:nginx#installed) must STILL force Written — resting probe is stale.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src("apt-get purge nginx\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishWritten(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "same-cell purge must keep install EstablishWritten (no over-loosening): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_)))
        );
    }

    #[test]
    fn distinct_selectors_do_not_discharge_each_other() {
        // The selector regression (`notes/193` §7.4): `systemctl enable nginx` and
        // `systemctl start nginx` gate DIFFERENT selectors of the SAME service:nginx
        // cell (#enabled vs #active). Neither discharges the other — both stay
        // EstablishAmbient (an `is-active` probe must not satisfy an unmet `#enabled`).
        // A flat key (one bit per kind+entity) could not hold this — the second would
        // see the first reach its cell and (mis-)classify Written.
        let mut i = Interner::default();
        let service = KindId(i.intern("service"));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        let systemctl = ProviderId(i.intern("systemctl"));
        let enable = i.intern("enable");
        let start = i.intern("start");
        let mut idx = KindIndex::default();
        idx.add_effect(systemctl, enable, service, enabled, Polarity::Establish);
        idx.add_effect(systemctl, start, service, active, Polarity::Establish);

        let classes = classify_src(
            "systemctl enable nginx\nsystemctl start nginx",
            &mut i,
            &idx,
        );
        let enabled_cell = FactKey {
            kind: service,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: enabled,
        };
        let active_cell = FactKey {
            kind: service,
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: active,
        };
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(enabled_cell)),
            "enable nginx ⇒ service:nginx#enabled, ambient: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(active_cell)),
            "start nginx ⇒ service:nginx#active, ambient (NOT discharged by #enabled): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "distinct selectors ⇒ neither reaches the other's cell ⇒ no Written"
        );
    }

    #[test]
    fn pure_builtin_upstream_does_not_poison_ambientness() {
        // fs-4 (note 16G), the contrast to `opaque_upstream_poisons_ambientness`:
        // the blessed target-state-pure builtins (`:`/`echo`/`cd`/…) touch
        // shell-env/stdout, never an oracle-modeled fact, so they must NOT poison a
        // later establish's ambient-ness. Guards the WHOLE `is_target_state_pure_builtin`
        // allowlist + the Ambient-vs-Written line (the `set`-only end-to-end case does
        // not isolate this); a mis-edit dropping `:`/`echo` would silently re-poison —
        // a wrong-skip surface.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(":\necho hi\napt-get install nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "pure builtins (`:`/`echo`) upstream must keep the install EstablishAmbient: {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no spurious Written from a pure-builtin upstream"
        );
    }

    #[test]
    fn called_function_body_inlines_to_a_single_call_leaf() {
        // arch-2 (brk-2): a call to a same-file-earlier funcdef is INLINED — the body is
        // spliced at the call, and the CALL is the one render/apply leaf, aggregating the
        // body's effect-bearing sites. `p() { apt-get install nginx; }\np` ⇒ exactly ONE
        // leaf: an `InlineCall` whose single body site is the install's `EstablishAmbient`
        // (the body becomes reachable through the splice — the find-7 un-detaching). The
        // detached DEFINITION body is no longer an independent leaf (`i-3`), so there is no
        // second `MustRun`. (Supersedes the round-20 `detached_function_body_establish_is_
        // not_ambient`: the detached-poison shape is re-homed to the refused-call cases below.)
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { apt-get install nginx; }\np", &mut i, &idx);
        assert_eq!(
            classes.len(),
            1,
            "the call is the only leaf (body is non-leaf)"
        );
        let SkipClass::InlineCall { sites } = &classes[0] else {
            panic!("the call must classify InlineCall, got {:?}", classes[0]);
        };
        assert_eq!(sites.len(), 1, "one effect-bearing body site (the install)");
        assert!(
            matches!(sites[0].class, SkipClass::EstablishAmbient(_)),
            "the body install is EstablishAmbient (reachable via the splice), not Written/\
             MustRun — the call node gens Pure, so it does not poison its own spliced body"
        );
    }

    #[test]
    fn uncalled_function_definition_contributes_no_runnable_leaf() {
        // arch-2: a funcdef DEFINED but never CALLED stays a detached, non-leaf island — its
        // body commands are not independent plan/apply leaves (`i-3`: a definition's body runs
        // only via a call, which would splice it). So `p() { apt-get install nginx; }\necho hi`
        // has exactly ONE leaf — the top-level `echo hi` — and the install does NOT surface as
        // a `MustRun`/`skip-unresolvable` leaf of its own. (This re-homes the find-A
        // reachability intent: an unreachable funcdef body advertises no elidable establish.)
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { apt-get install nginx; }\necho hi", &mut i, &idx);
        assert_eq!(classes.len(), 1, "only the top-level `echo hi` is a leaf");
        assert_eq!(
            classes[0],
            SkipClass::MustRun,
            "echo hi is unmodeled ⇒ MustRun"
        );
    }

    #[test]
    fn recursive_call_refuses_inline_and_poisons_the_body() {
        // arch-2 (`i-1`): a recursive call ⊤-rejects the inline (the cycle guard) — the inner
        // `p` stays an ordinary unmodeled command (Opaque). The OUTER call still inlines, but
        // its body now contains that Opaque, which poisons the body install to `MustRun` ⇒ the
        // whole call cannot elide (one non-licensing body leaf runs the call). This pins that
        // the detached-poison semantics survive a refused (recursive) call — the brief's
        // re-homed poison pin.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("p() { p; apt-get install nginx; }\np", &mut i, &idx);
        assert_eq!(classes.len(), 1, "the outer call is the only leaf");
        let SkipClass::InlineCall { sites } = &classes[0] else {
            panic!("the outer call inlines, got {:?}", classes[0]);
        };
        assert!(
            sites.iter().any(|s| s.class == SkipClass::MustRun),
            "the recursion-refused inner `p` (Opaque) poisons the body ⇒ a MustRun body site \
             ⇒ the call will run (the poison-pin is preserved across a refused call)"
        );
    }

    #[test]
    fn command_effect_resolves_operand_singleton_and_top() {
        // Resolve a single-command book through value-flow + the corpus apt check,
        // returning the node's effect cells. (One command ⇒ one Command node.)
        fn eff(src: &str, i: &mut Interner, idx: &KindIndex) -> Vec<CommandEffect> {
            let parsed = dorc_syntax::parse(src);
            let built = cfg::build(&parsed.value);
            let value = analyze(&built.value, &parsed.value, i);
            let checks = vec![lift_checks(i, CORPUS_CHECK_SRC).value];
            // A dynamic command word (`$cmd …`) is ⊤-rejected by the parser ⇒ a `Top`
            // CFG node, not a `Command` — classify treats that as Opaque. Mirror it.
            let Some(node) = built
                .value
                .iter()
                .find(|(_, n)| n.kind == CfgNodeKind::Command)
                .map(|(id, _)| id)
            else {
                return vec![CommandEffect::Opaque];
            };
            let mut diags = Vec::new();
            command_effect(idx, &checks, &value.argv_values(node), i, &mut diags, None)
        }
        let (mut i, idx, s) = package_setup();
        // One operand ⇒ Operand cell; the flag `-y` is post-verb-stripped by the check.
        let nginx_cell = pkg_installed(&mut i, &s, "nginx");
        assert_eq!(
            eff("apt-get install -y nginx", &mut i, &idx),
            vec![CommandEffect::Establishes(nginx_cell)],
            "the check strips the post-verb -y ⇒ Operand(nginx)#installed"
        );
        // Nullary modeled verb (`update`) ⇒ the check's value-less `package-index`
        // annotation ⇒ Singleton (the poison-wall fix). A flag-only tail stays nullary
        // (the `update` arm ignores the trailing `-y`).
        let pkg_index_fresh = CommandEffect::Establishes(FactKey {
            kind: s.package_index,
            entity: EntityRef::Singleton,
            selector: s.fresh,
        });
        assert_eq!(
            eff("apt-get update", &mut i, &idx),
            vec![pkg_index_fresh.clone()],
            "nullary modeled verb ⇒ Singleton(package-index#fresh)"
        );
        assert_eq!(
            eff("apt-get update -y", &mut i, &idx),
            vec![pkg_index_fresh],
            "flag-only tail stays nullary ⇒ Singleton"
        );
        // A non-literal operand (`$PKG` ⇒ ⊤ in value-flow) is an UNKNOWN cell, NOT the
        // singleton — else `install $PKG` would be wrongly elidable (priority-1
        // wrong-elision). ⊤ arg ⇒ unresolved site ⇒ Opaque ⇒ run.
        assert_eq!(
            eff("apt-get install $PKG", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "non-literal operand ⇒ ⊤, not Singleton"
        );
        // Multi-operand: the single-`$1` check binds nginx, but its `[ "$2" = "" ]`
        // guard sees the SECOND operand `curl` ⇒ no probe reached ⇒ Top ⇒ Opaque ⇒ run.
        // This is the check's OWN multi-operand refusal (the oracle's code, not the
        // engine): a wrong single-entity elision that would silently drop `curl` is
        // avoided — the safety the deleted engine-side stand-in used to provide.
        assert_eq!(
            eff("apt-get install nginx curl", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "second operand ⇒ the check's guard refuses ⇒ ⊤"
        );
        // Dynamic command name ⇒ ⊤ word0 ⇒ Opaque.
        assert_eq!(
            eff("$cmd install nginx", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "dynamic command name ⇒ ⊤"
        );
        // Unknown verb: `autoclean` ⇒ the check's `*` arm reads `$1` (past end ⇒ Top),
        // and the effect-map has no (apt-get, autoclean) row anyway ⇒ Opaque.
        assert_eq!(
            eff("apt-get autoclean", &mut i, &idx),
            vec![CommandEffect::Opaque],
            "unknown verb ⇒ ⊤"
        );
    }

    #[test]
    fn multi_operand_is_not_wrongly_elided() {
        // The kFAIL-perform guard the new check preserves (the deleted stand-in's
        // multi-operand refusal): `apt-get install nginx curl` must NOT resolve to a single-entity
        // cell (which could elide, silently dropping curl). It stays MustRun.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src("apt-get install nginx curl", &mut i, &idx);
        assert_eq!(
            classes,
            vec![SkipClass::MustRun],
            "multi-operand install ⇒ MustRun (no single-entity wrong-elision)"
        );
    }

    #[test]
    fn opaque_var_operand_is_top_when_unresolved_but_resolves_when_flowed() {
        // The value-plane's reach: a command-prefix/assigned operand. `PKG=nginx;
        // apt-get install -y "$PKG"` — value-flow resolves `"$PKG"` to nginx, so the
        // site is fully concrete and the check resolves entity=nginx (EstablishAmbient).
        // This is the value-flow win the old engine-side stand-in (which saw `"$PKG"`
        // as a non-literal operand ⇒ Opaque) could not reach. Contrast: an UNASSIGNED
        // `"$X"` stays ⊤ ⇒ Opaque. (GOLDEN: `exec-opaque-var` flips elsewhere — flagged.)
        let (mut i, idx, s) = package_setup();
        // The bare `PKG=nginx` assignment is also a leaf (MustRun); the install is the
        // one we assert resolved.
        let flowed = classify_src("PKG=nginx\napt-get install -y \"$PKG\"", &mut i, &idx);
        assert!(
            flowed.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "value-flow resolves the assigned operand ⇒ the install is identity-resolved: {flowed:?}"
        );
        let unresolved = classify_src("apt-get install -y \"$X\"", &mut i, &idx);
        assert_eq!(
            unresolved,
            vec![SkipClass::MustRun],
            "an unassigned ⊤ operand ⇒ unresolved site ⇒ MustRun"
        );
    }

    // --- task-D2: the Query effect-class + rule-query-validity (202 §2 / 205 §2) ---

    /// `tool:<entity>#present` — the cell `command -v <entity>` queries.
    fn tool_present(i: &mut Interner, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("present")),
        }
    }

    /// A package index (install/purge/update) PLUS a read-only `command '' query
    /// present` guard on `tool` (the canonical `command -v` Query). Threads a
    /// caller-provided interner so the Query tests share one across index-build +
    /// classify + assertions.
    fn package_and_query_index(i: &mut Interner) -> KindIndex {
        let package = KindId(i.intern("package"));
        let package_index = KindId(i.intern("package-index"));
        let installed = SelectorId(i.intern("installed"));
        let fresh = SelectorId(i.intern("fresh"));
        let apt = ProviderId(i.intern("apt-get"));
        let install = i.intern("install");
        let purge = i.intern("purge");
        let update = i.intern("update");
        let tool = KindId(i.intern("tool"));
        let present = SelectorId(i.intern("present"));
        let command = ProviderId(i.intern("command"));
        let eps = empty_verb(i);
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, Polarity::Establish);
        idx.add_effect(apt, purge, package, installed, Polarity::Kill);
        idx.add_effect(apt, update, package_index, fresh, Polarity::Establish);
        idx.add_effect(command, eps, tool, present, Polarity::Query);
        idx
    }

    #[test]
    fn lone_query_guard_is_resolvable_and_valid() {
        // The simplest Query: `command -v nginx` with nothing upstream ⇒
        // QueryResolvable + valid (pristine prefix — no write-or-unknown reached it).
        // This is the headline guard, classified as a first-class read-only Query.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx", &mut i, &idx);
        let fact = tool_present(&mut i, "nginx");
        assert_eq!(
            classes,
            vec![SkipClass::QueryResolvable { fact, valid: true }],
            "a lone Query guard is resolvable + valid: {classes:?}"
        );
    }

    #[test]
    fn query_does_not_poison_downstream_establish() {
        // A Query READS, it does not write — so an upstream `command -v nginx` must NOT
        // poison a downstream `apt-get install nginx`'s ambient-ness (contrast an Opaque
        // neighbour, which does). The install stays EstablishAmbient. This is the
        // gen-side of task-D2 (a Query gens nothing into Reach).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx\napt-get install -y nginx", &mut i, &idx);
        let install = pkg_installed_with(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(install)),
            "an upstream Query must NOT poison the install (it reads, doesn't write): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "no Written: a Query gens nothing into Reach"
        );
    }

    #[test]
    fn query_after_query_stays_valid_st3() {
        // st-3 (20A §4): an upstream QUERY does not invalidate a downstream Query (reads
        // don't write — the guard-stack idiom keeps all its folds). Two `command -v`
        // guards: BOTH stay valid. A pure builtin between them likewise doesn't
        // invalidate (it gens nothing).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("command -v nginx\n:\ncommand -v curl", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        let curl = tool_present(&mut i, "curl");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "first Query valid: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: curl,
                valid: true
            }),
            "second Query STILL valid — an upstream Query (+ pure `:`) does not invalidate (st-3): {classes:?}"
        );
    }

    #[test]
    fn query_after_mutator_is_invalid() {
        // rule-query-validity (205 §2): an upstream MUTATOR (a write) invalidates a
        // downstream Query — its resting rc is now stale. `apt-get install curl`
        // (establishes package:curl#installed) ⇒ the `command -v nginx` guard below it
        // is QueryResolvable but INVALID (valid: false). The cell mutated is irrelevant
        // (ANY write invalidates — the pristine-prefix rule, not same-cell).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("apt-get install -y curl\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a Query below a mutator is INVALID (stale resting rc — pristine-prefix fails): {classes:?}"
        );
    }

    #[test]
    fn query_after_opaque_is_invalid() {
        // rule-query-validity, the Opaque arm: an upstream un-oracled (Opaque) command
        // ⇒ Reach::Top ⇒ the downstream Query is INVALID (an unknown command may have
        // changed anything). `ufw allow 80/tcp` is un-oracled here ⇒ Opaque.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("ufw allow 80/tcp\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a Query below an Opaque command is INVALID (⊤ reached it): {classes:?}"
        );
    }

    // --- y-1 (redirect-effects, `21F` imp-1): a write-redirect is a file-write WRITER ----

    /// `file:<path>#written` — the cell a write-redirect (`>`/`>>`) to `path` gens.
    fn file_written(i: &mut Interner, path: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("file")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(path))),
            selector: SelectorId(i.intern("written")),
        }
    }

    /// The y-1 file-write cell is built by `file_write_cell` from the resolved path; the
    /// test-side `file_written` must reproduce its exact shape (kind `file`, entity = the
    /// path operand, selector `written`), or every other y-1 pin is asserting the wrong cell.
    #[test]
    fn file_write_cell_has_the_declared_shape() {
        let mut i = Interner::default();
        let path = i.intern("/etc/app.conf");
        assert_eq!(
            file_write_cell(path, &mut i),
            file_written(&mut i, "/etc/app.conf"),
            "the gen'd file-write cell shape must match the documented (file, path, written)"
        );
    }

    #[test]
    fn write_redirect_invalidates_downstream_query() {
        // THE `21F` imp-1 regression pin (the reason y-1 exists). A write-redirect to a real
        // sink is a WRITER: `: > /etc/marker` gens `file:/etc/marker#written`, so the
        // downstream `command -v nginx` guard fails rule-query-validity (its resting rc is now
        // stale — a file the book just wrote sits between entry and the guard). Pre-y-1 the
        // redirect was invisibly Pure ⇒ the guard read `valid: true` ⇒ a stale-guard fold
        // MANUFACTURED a wrong-elision (the imp-1 hole). Same shape as
        // `query_after_mutator_is_invalid`, but the invalidator is a redirect, not an oracled
        // mutator.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > /etc/marker\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a write-redirect upstream must invalidate the downstream Query (imp-1 pin): {classes:?}"
        );
    }

    #[test]
    fn append_redirect_also_invalidates_query() {
        // Append vs truncate are BOTH write-shaped (the charter unit pin): `printf x >> f`
        // (append) invalidates exactly as `>` (truncate) does. `printf` is a blessed-pure
        // builtin, so WITHOUT y-1 the `>> f` would be the only write — and it was invisible
        // (the precise imp-1 strawman: `set -e; printf 'x' >> f; grep ... f || mutator`).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(
            "printf 'x' >> /etc/app.conf\ncommand -v nginx",
            &mut i,
            &idx,
        );
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "an APPEND (`>>`) redirect is write-shaped too ⇒ invalidates the Query: {classes:?}"
        );
    }

    #[test]
    fn var_resolved_redirect_target_invalidates_query() {
        // The value-plane integration the charter emphasizes (y1-a: "resolve the target word
        // through the EXISTING value plane"): a redirect target is an ordinary expansion, so
        // `logfile=app.log; : > "$logfile"` resolves `$logfile` ⇒ `app.log` ⇒ gens
        // `file:app.log#written` ⇒ invalidates the downstream Query. Constant propagation
        // composes with the redirect-target resolution (shared `resolve_recipe` machinery).
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(
            "logfile=app.log\n: > \"$logfile\"\ncommand -v nginx",
            &mut i,
            &idx,
        );
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a var-resolved redirect target (via the value plane) invalidates the Query: {classes:?}"
        );
    }

    #[test]
    fn devnull_redirect_does_not_invalidate_query() {
        // The exemption set (the charter unit pin): `>/dev/null` is the discard sink — NOT a
        // file-write effect — so it gens no cell and a downstream Query stays valid. This is
        // the `exec-devnull-exempt` mechanism at the validity layer: a redirect to the bit
        // bucket must not poison.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > /dev/null\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "`>/dev/null` is exempt (the discard sink) ⇒ the Query stays valid: {classes:?}"
        );
    }

    #[test]
    fn fd_dup_redirect_does_not_invalidate_query() {
        // The exemption set, the fd-dup arm: `2>&1` is a file-descriptor dup, NOT a
        // file-write — so it gens no cell and a downstream Query stays valid. (`2>&1` stays
        // exempt per the existing devnull/dup vocabulary — charter y1-a.)
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src("echo hi 2>&1\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: true
            }),
            "`2>&1` is an fd-dup, not a file-write ⇒ the Query stays valid: {classes:?}"
        );
    }

    #[test]
    fn write_redirect_poisons_downstream_establish_ambientness() {
        // A write-redirect is a WRITER, so — like any Opaque/mutator — it makes a downstream
        // establish non-ambient when... actually NO: a `file` cell is a DIFFERENT cell from
        // `package:nginx#installed`, so by the poison-wall keystone it must NOT poison the
        // install (distinct cells don't cross-poison). The install stays EstablishAmbient.
        // This pins that the file-cell is a real per-path cell (not a ⊤ that havocs): only
        // the SAME cell (or an Opaque ⊤) invalidates ambient-ness.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(": > /etc/marker\napt-get install -y nginx", &mut i, &idx);
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a file-write cell is a distinct cell ⇒ it must NOT poison a package install (keystone): {classes:?}"
        );
    }

    #[test]
    fn top_target_redirect_poisons_downstream_query() {
        // A ⊤ (dynamic) redirect target joins ⊤ (the Opaque-poison shape, charter y1-a): the
        // path is unresolved so no per-path cell can be keyed, and a downstream Query is
        // INVALID (an unknown file — possibly anything — was written). `> "$dyn"` where `$dyn`
        // is never assigned ⇒ the target is ⊤. (The disclosure `dq-redir-target-top` fires;
        // the validity-invalidation is the behavior pinned here.)
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let classes = classify_src(": > \"$dyn\"\ncommand -v nginx", &mut i, &idx);
        let nginx = tool_present(&mut i, "nginx");
        assert!(
            classes.contains(&SkipClass::QueryResolvable {
                fact: nginx,
                valid: false
            }),
            "a ⊤-target redirect joins ⊤ ⇒ invalidates the downstream Query: {classes:?}"
        );
    }

    #[test]
    fn top_target_redirect_discloses_not_silent() {
        // The ⊤-target redirect disclosure (`dq-redir-target-top`, the redirect-effects analog
        // of `dq-cmdsub-operand-top`): a write to a dynamic target is surfaced, never silent.
        let mut i = Interner::default();
        let idx = package_and_query_index(&mut i);
        let diags = classify_src_diags(": > \"$dyn\"\ncommand -v nginx", &mut i, &idx);
        assert!(
            has_code(&diags, "dq-redir-target-top"),
            "a ⊤-target write-redirect must disclose dq-redir-target-top: {diags:?}"
        );
    }

    // --- task-L1 (`209` brk-1): reaching-defs over the loop back-edge -------------

    #[test]
    fn post_loop_install_ambient_when_loop_body_is_pure() {
        // THE brk-1 value-unlock at the EFFECT layer: a PURE loop body (`echo` only —
        // gens nothing) does NOT poison a converged install BELOW the loop. The
        // reaching-defs back-edge carries no write out of the loop, so the post-loop
        // install stays EstablishAmbient (elidable). Pre-L1 the loop was a ⊤ node whose
        // havoc + ⊤-containment killed this — the poison the L1 lowering removes.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for f in a b; do echo \"$f\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a pure loop body must NOT poison the post-loop install: {classes:?}"
        );
    }

    #[test]
    fn opaque_in_loop_body_poisons_post_loop_install() {
        // The honest residual cost (exclusion-check, the other cell): an OPAQUE command
        // inside the loop body propagates Reach::Top across the back-edge and OUT to the
        // post-loop install ⇒ it is forced EstablishWritten (runs). A loop is not magic —
        // an un-oracled body command poisons exactly as it would in straight-line code.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "for f in a b; do ufw allow \"$f\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        assert!(
            classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishWritten(_))),
            "an Opaque loop-body command poisons the post-loop install (back-edge ⊤): {classes:?}"
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishAmbient(_))),
            "no ambient install survives the in-loop Opaque"
        );
    }

    #[test]
    fn classify_converges_on_nested_loop_back_edges() {
        // The reaching-defs fixpoint must converge on a NESTED loop (two back-edges).
        // `classify` carries a `debug_assert!(reach.converged)`; a non-converging
        // reaching-defs would trip it (or, in release, fold every establish to MustRun
        // via `trust_reach`). Drive a nested loop with a body establish and assert we get
        // a classification back at all (the post-loop install) — convergence implied.
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            "for o in a b; do for p in c d; do apt-get install -y \"$p\"; done; done\nsystemctl reload nginx",
            &mut i,
            &idx,
        );
        assert!(
            !classes.is_empty(),
            "classify returns (reaching-defs converged on the nested back-edges): {classes:?}"
        );
    }

    /// `package:<entity>#installed` via a shared interner (sibling of `pkg_installed`
    /// for the Query tests that build their own index inline).
    fn pkg_installed_with(i: &mut Interner, entity: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern("installed")),
        }
    }

    // --- task-L2 item-2 (`209` brk-1(b)): the in-loop Members fact-family ----------

    #[test]
    fn in_loop_members_site_classifies_as_establish_members_family() {
        // THE item-2 unlock: `for pkg in nginx curl; do apt-get install -y "$pkg"; done` ⇒
        // the body install is `EstablishMembers` carrying the per-member family
        // [package:nginx#installed, package:curl#installed], in list order. Each member
        // resolved through the oracle check exactly as a straight-line install would.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            r#"for pkg in nginx curl; do apt-get install -y "$pkg"; done"#,
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        let curl = pkg_installed(&mut i, &s, "curl");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, curl],
                self_reached: true,
            }),
            "the in-loop Members install resolves a per-member fact-family in list order, self-reached: {classes:?}"
        );
    }

    #[test]
    fn members_family_keeps_duplicate_cells() {
        // Dups are kept (dash iterates them): `for p in nginx nginx` ⇒ a two-element family
        // of the SAME cell. (item-1's no-dedup carried into the cell family.)
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            r#"for p in nginx nginx; do apt-get install -y "$p"; done"#,
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, nginx],
                self_reached: true,
            }),
            "duplicate members ⇒ duplicate cells in the family: {classes:?}"
        );
    }

    #[test]
    fn members_family_all_or_nothing_one_member_unresolvable_tops() {
        // ALL-OR-NOTHING (item-2): if ANY member fails to resolve to a single establish,
        // the WHOLE site is NOT a family. `for p in nginx "a b"; do apt-get install -y $p;
        // done` — the list is two eligible single-concrete members (`nginx`, `a b`), but the
        // body's UNQUOTED `$p` field-splits each member's value: `nginx` ⇒ one operand
        // (resolves to package:nginx#installed), while `a b` ⇒ TWO operands (`apt-get
        // install -y a b`) ⇒ the check's `[ "$2" = "" ]` guard refuses ⇒ that member is
        // Opaque. One member unresolvable ⇒ NO family (not a partial [nginx-only] one) ⇒
        // the in-loop site falls to the single-cell Flat path ⇒ MustRun (the floor).
        let (mut i, idx, _s) = package_setup();
        let classes = classify_src(
            r#"for p in nginx "a b"; do apt-get install -y $p; done"#,
            &mut i,
            &idx,
        );
        assert!(
            !classes
                .iter()
                .any(|c| matches!(c, SkipClass::EstablishMembers { .. })),
            "one unresolvable member ⇒ NO family (all-or-nothing), falls to MustRun: {classes:?}"
        );
        assert!(
            classes.contains(&SkipClass::MustRun),
            "the all-or-nothing failure floors the in-loop site to MustRun: {classes:?}"
        );
    }

    #[test]
    fn members_family_gens_member_cells_not_opaque_post_loop_stays_clean() {
        // The reaching-defs consequence (load-bearing for item-3's self-reach): a resolved
        // Members site gens its MEMBER cells into Reach, NOT Opaque. So a post-loop install
        // of a DISTINCT package is NOT poisoned to Written by the loop. `for pkg in nginx
        // curl; do apt-get install -y "$pkg"; done; apt-get install -y redis` ⇒ the redis
        // install stays EstablishAmbient (the loop genned nginx/curl cells, not ⊤).
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\napt-get install -y redis",
            &mut i,
            &idx,
        );
        assert!(
            classes.contains(&SkipClass::EstablishAmbient(pkg_installed(
                &mut i, &s, "redis"
            ))),
            "a resolved Members loop gens member cells (not ⊤) ⇒ a distinct post-loop install stays ambient: {classes:?}"
        );
    }

    #[test]
    fn members_family_poisons_post_loop_same_cell() {
        // Exclusion-check (the other cell): a post-loop install of a cell the LOOP
        // establishes IS reached by the loop's member-establish ⇒ EstablishWritten (stale
        // resting probe). `for pkg in nginx curl; …; apt-get install -y nginx` ⇒ the
        // post-loop nginx install sees the loop's nginx member-cell upstream ⇒ Written.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\napt-get install -y nginx",
            &mut i,
            &idx,
        );
        // The post-loop nginx is Written (a member-cell reached it); curl was never
        // post-installed. No EstablishAmbient for nginx.
        assert!(
            classes.contains(&SkipClass::EstablishWritten(pkg_installed(
                &mut i, &s, "nginx"
            ))),
            "a post-loop install of a loop-member cell is Written (the member-establish reaches it): {classes:?}"
        );
    }

    #[test]
    fn members_self_reach_broken_by_pre_loop_writer() {
        // item-3(b) self-reach FALSE (the `loop-member-external-writer-runs` core): a
        // PRE-LOOP `apt-get purge curl` kills `package:curl#installed` — a member cell. That
        // write reaches the in-loop install via the in-state, so the site's in-state is NOT
        // a subset of its own family ⇒ `self_reached: false`. The family still resolves
        // (item-2); only the self-reach bit flips ⇒ the license (item-3) will refuse.
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "apt-get purge curl\nfor pkg in nginx curl; do apt-get install -y \"$pkg\"; done",
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        let curl = pkg_installed(&mut i, &s, "curl");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, curl],
                self_reached: false,
            }),
            "a pre-loop purge of a member cell breaks self-reach (in-state ⊄ family): {classes:?}"
        );
    }

    #[test]
    fn members_self_reach_broken_by_opaque_in_body() {
        // item-3(b) self-reach FALSE via an in-loop Opaque sibling: `for pkg in nginx curl;
        // do ufw allow "$pkg"; apt-get install -y "$pkg"; done` — the un-oracled `ufw allow`
        // is Opaque ⇒ Reach::Top reaches the install ⇒ `self_reached: false`. (The install's
        // family still resolves; the sibling Opaque is the non-self writer.)
        let (mut i, idx, s) = package_setup();
        let classes = classify_src(
            "for pkg in nginx curl; do ufw allow \"$pkg\"; apt-get install -y \"$pkg\"; done",
            &mut i,
            &idx,
        );
        let nginx = pkg_installed(&mut i, &s, "nginx");
        let curl = pkg_installed(&mut i, &s, "curl");
        assert!(
            classes.contains(&SkipClass::EstablishMembers {
                members: vec![nginx, curl],
                self_reached: false,
            }),
            "an in-loop Opaque sibling (⊤) breaks self-reach: {classes:?}"
        );
    }

    // ---- q-2: the `$()` ⊤-diagnostics floor (find-3 no-silent-phantoms) ----

    #[test]
    fn cmdsub_operand_top_disclosed_not_silent() {
        // Why (219 q-1.f silent-2, the find-3 violation q-2 closes): a `$()`-captured operand
        // forces the command Opaque, and that degradation used to be SILENT. The disclosure must
        // now fire (`dq-cmdsub-operand-top`). `PKG=$(cat /etc/pkg)` ⇒ `$PKG` is ⊤ ⇒ the install's
        // operand is ⊤ ⇒ Opaque + the Note.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags(
            "PKG=$(cat /etc/pkg)\napt-get install -y \"$PKG\"",
            &mut i,
            &idx,
        );
        assert!(
            has_code(&diags, "dq-cmdsub-operand-top"),
            "a ⊤ operand must disclose dq-cmdsub-operand-top, never silently Opaque: {diags:?}"
        );
    }

    #[test]
    fn cmdsub_inner_nonleaf_disclosed_for_effectbearing_inner() {
        // Why (219 q-1.f, the exec-subst-body-nonleaf disclosure): an EFFECT-BEARING command
        // inside `$()` runs un-elidably (no leaf of its own) and is invisible today. The
        // disclosure surfaces it (`dq-cmdsub-inner-nonleaf`). `apt-get install -y nginx` inside
        // `$()` is oracled (Establishes) ⇒ effect-bearing ⇒ disclosed; the enclosing `echo` is
        // Pure so it never independently elides the inner install.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags(
            "echo \"installed: $(apt-get install -y nginx)\"",
            &mut i,
            &idx,
        );
        assert!(
            has_code(&diags, "dq-cmdsub-inner-nonleaf"),
            "an effect-bearing $()-inner command must be disclosed: {diags:?}"
        );
    }

    #[test]
    fn pure_inner_cmdsub_discloses_nothing() {
        // Why (the gate on the disclosure): a PURE `$()`-inner command does nothing un-elidable,
        // so it must NOT emit `dq-cmdsub-inner-nonleaf` (warning-fatigue floor — disclose only
        // what actually runs un-elidably). `echo "$(echo hi)"`: the inner `echo` is Pure.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags("echo \"got: $(echo hi)\"", &mut i, &idx);
        assert!(
            !has_code(&diags, "dq-cmdsub-inner-nonleaf"),
            "a pure $()-inner command discloses nothing un-elidable: {diags:?}"
        );
    }

    #[test]
    fn straightline_concrete_book_has_no_cmdsub_diagnostics() {
        // Why (the negative pin): a fully-concrete straight-line book has no ⊤ and no `$()`, so
        // NEITHER cmdsub code fires — the disclosure is specific to the degradation, not noise on
        // every command.
        let (mut i, idx, _s) = package_setup();
        let diags = classify_src_diags("apt-get install -y nginx", &mut i, &idx);
        assert!(
            !has_code(&diags, "dq-cmdsub-operand-top")
                && !has_code(&diags, "dq-cmdsub-inner-nonleaf"),
            "a concrete book emits no cmdsub ⊤-diagnostics: {diags:?}"
        );
    }
}
