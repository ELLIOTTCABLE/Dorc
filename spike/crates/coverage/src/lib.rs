//! `dorc-coverage` — the analyzer-coverage dashboard (round-21 arch-6).
//!
//! A re-runnable report over any *book + oracle set* (+ optional probe-results)
//! that decomposes the round's north star — "~80% criticality-weighted non-trivial
//! elision coverage on a converged host" (`211` §1) — into per-site, per-door,
//! separately-ownable terms (`20V` §7: "attribute every elision to its door … so
//! the 80% question decomposes into measurable, separately-ownable terms").
//!
//! Per command-site it answers four questions (the charter's c1–c4):
//! * **c1** analyzable-without-⊤? — and if not, the Opaque/⊤ reason the engine fired;
//! * **c2** oracled? — did an oracle's `check()` + effect-map resolve a fact here;
//! * **c3** probed-converged? — when probe-results are supplied, what the host said;
//! * **c4** disposition AND THROUGH WHICH DOOR — the [`Door`] vocabulary below.
//!
//! ## This crate consumes the engine; it never re-implements it
//!
//! Everything here is derived from the SAME public pipeline the cli drives
//! (`syntax::parse → analysis::cfg::build → analysis::value::analyze →
//! analysis::effect::classify → oracle::lift / check::lift_checks →
//! plan::compile_probe → plan::build_plan`). The dashboard reads the engine's
//! outputs (the [`SkipClass`], the [`Disposition`], the consumed-channel set, the
//! probe verdict) and *attributes* them; it makes no analysis decision of its own.
//!
//! ## Evolution-survival: NEVER exhaustively match an engine enum
//!
//! The main tree is concurrently gaining new enum variants (a `SkipClass::InlineCall`,
//! new `Channel`s, new `LicenseVia`s). Every `match` over an engine enum here ends in
//! `_ =>` routing to an honest [`Door::Unattributed`] / [`BlockReason::Unattributed`]
//! bucket carrying the `{:?}` debug-repr — so the crate keeps compiling across engine
//! evolution AND reports its own blind spots instead of silently miscounting. The
//! `unattributed` bucket masking a systematic gap is itself a hunt-list item (the
//! report surfaces its population loudly).
//!
//! Determinism (`inv-determinism`): the analysis the dashboard drives is the pure
//! kernel; the dashboard sorts everything by site-id and uses `BTreeMap` throughout.
//! The only nondeterminism-edge (file reads, stdout) lives in the binary, not here.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use dorc_analysis::cfg::{Cfg, CfgNodeId};
use dorc_analysis::effect::SkipClass;
use dorc_analysis::value::ValueOf;
use dorc_core::{Channel, Interner, Verdict};
use dorc_plan::{Disposition, LeafId, LicenseVia};

pub mod weights;

// ===========================================================================
// The door vocabulary (charter c4 / `20V` §3–§4 / §7)
// ===========================================================================

/// Through which DOOR a site's disposition was reached — the charter's c4
/// vocabulary, with full-elisions and guard-transforms kept as SEPARATE doors
/// ("never blurred", `20V` §7). The provenance taxonomy of `20V` §3 (each arm the
/// negation of the others) maps onto these.
///
/// Ordering note: the `Ord` derive orders doors for the rollup's stable column
/// walk; it is presentation, never logic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Door {
    /// **door-1 `fold`** — dead control-flow under a *measured guard rc* (`20V` §4):
    /// the leaf lies in a provably-dead `&&`/`||`/`if`/`!` branch whose controlling
    /// guard's rc the probe measured. The engine's [`Disposition::Omit`]. The
    /// guard's restart/sed cascade falls out of plain control-flow analysis (the
    /// Ansible handler/notify semantic) with zero new trust.
    Fold,
    /// **door-3 `dead-invariant`** — a `StatusInvariant`-licensed replace (`20V`
    /// §4): the `cmd || true` shape, consumed-in-form but dead-in-fact. The engine
    /// mints a [`Disposition::Replace`] even at a ⊤ status because both `||`
    /// continuations rejoin identically; we detect it by the site's consumed set
    /// carrying [`Channel::StatusInvariant`]. The admin's own spelled-in-sh "this
    /// rc is not load-bearing".
    DeadInvariant,
    /// **`replace-converged`** — a plain converged-establish elision: a
    /// [`Disposition::Replace`] via [`LicenseVia::ConvergedEstablish`] or
    /// [`LicenseVia::MembersLoop`] (the in-loop all-converged variant), whose site
    /// is NOT a door-3 invariant shape. The bread-and-butter full elision.
    ReplaceConverged,
    /// **`query-substituted`** — a read-only Query guard value-substituted to its
    /// probed rc ([`Disposition::Replace`] via [`LicenseVia::QueryGuard`]). NOT a
    /// mutation-elision (the guard mutates nothing) and NOT door-4 (which *inserts*
    /// a guard before a bare mutator) — its own column so it is never blurred with
    /// either full elisions or guard-transforms (`20V` §7 spirit). The headline
    /// `dpkg -s X >/dev/null 2>&1`-as-pre-flight shape.
    QuerySubstituted,
    /// **door-4 `guard-transform`** — DOES NOT EXIST YET (`20V` §4 keystone;
    /// `211` §1 arch-3(c) lands later in the round). Kept as a column reporting 0
    /// so the instrument's shape is stable when door-4 lands (a guard-insertion
    /// license, never observable-reproduction relaxation).
    GuardTransform,
    /// **door-2 `static-declared`** — DOES NOT EXIST YET (`20V` §4): a bare mutator
    /// statically elided per the kind's oracle-declared converged-run observable.
    /// Column reporting 0 until door-2 lands.
    StaticDeclared,
    /// The site **runs** — its effect is needed, unprobed, or an unvouched
    /// observable it emits is consumed. Carries the dominant [`BlockReason`].
    Runs(BlockReason),
    /// The engine produced a disposition shape this dashboard does not recognise —
    /// a NEW `Disposition`/`LicenseVia` variant the build predates. Carries the
    /// `{:?}` debug-repr so the gap is visible, never silently mis-bucketed. Its
    /// population appearing non-zero is a "go re-read the engine" signal.
    Unattributed(String),
}

impl Door {
    /// The short, stable column label for the rollup table + TSV (the greppable slug).
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Door::Fold => "fold",
            Door::DeadInvariant => "dead-invariant",
            Door::ReplaceConverged => "replace-converged",
            Door::QuerySubstituted => "query-substituted",
            Door::GuardTransform => "guard-transform",
            Door::StaticDeclared => "static-declared",
            Door::Runs(_) => "runs",
            Door::Unattributed(_) => "unattributed",
        }
    }

    /// Is this an elision/substitution door (a site the apply did NOT run verbatim)?
    /// True for every door but [`Door::Runs`] and [`Door::Unattributed`] — the
    /// numerator of the coverage fraction.
    #[must_use]
    pub fn is_elided(&self) -> bool {
        !matches!(self, Door::Runs(_) | Door::Unattributed(_))
    }

    /// Is this a FULL elision (the run-set shrinks: the mutator/guard is replaced or
    /// the branch omitted), as opposed to a guard-TRANSFORM? `20V` §7 demands these
    /// stay separate columns; this predicate keeps the north-star rollup honest.
    /// door-4 `guard-transform` is the only transform door (and it reports 0 today).
    #[must_use]
    pub fn is_full_elision(&self) -> bool {
        matches!(
            self,
            Door::Fold | Door::DeadInvariant | Door::ReplaceConverged | Door::QuerySubstituted
        )
    }
}

/// The dominant reason a [`Door::Runs`] site could not be elided — the charter's
/// "dominant blocking reason" for c4. Picked by a fixed precedence (see
/// [`block_reason`]); a site usually has one root cause and the report names it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockReason {
    /// A ⊤-trigger / Opaque: the command is unmodeled (a ⊤ argv word, no oracle, an
    /// unmodeled construct, a multi-operand the check refused). c1's "not analyzable".
    TopTrigger,
    /// Modeled + analyzable, but no oracle resolved a fact (the verb has no
    /// effect-map row / no check). c2's "not oracled". A `MustRun` that is not a ⊤.
    NoOracle,
    /// Oracled + probed, but the host reported the fact diverged (not yet
    /// established) ⇒ the mutation must run (`kFAIL-perform`).
    Diverged,
    /// Oracled, but no probe-result was supplied for the site (the dashboard ran
    /// without probe-results, or the record was absent) ⇒ `Unknown` ⇒ runs.
    Unprobed,
    /// A consumed Status channel the probe could not relax: a `StatusRelaxable`
    /// reader (`&&`/`||`/`if`/errexit) with a ⊤ rc — the bare-mutator-under-`set -e`
    /// shape door-2/door-4 exist to rescue. (The "consumed-⊤-status" bucket.)
    ConsumedStatusTop,
    /// A consumed `StatusIterated` (a `while`/`until` condition): blocks
    /// unconditionally — its per-iteration rc-sequence no single rc reproduces.
    LoopCondition,
    /// The site is inside a loop body (the in-loop render floor) and is not the
    /// all-converged Members shape ⇒ floored to run this round.
    InLoopFloor,
    /// A consumed `Stdout`/`Stderr`: vouched by nothing ⇒ blocks unconditionally.
    ConsumedOutput,
    /// The fact was mutated upstream in-script (`EstablishWritten`) ⇒ the resting
    /// probe is stale ⇒ runs (the `purge X; … install X` shape).
    WrittenUpstream,
    /// The disposition LICENSED an elision (`Replace`) but the leaf-exact render
    /// REFUSES to edit its span — a heredoc-bearing leaf (`<<EOF`), whose span covers
    /// the operator, not the body lines — so the artifact RUNS it verbatim (`20V` §4 d-6 /
    /// `21B` hunt-1). Without this the dashboard would over-count it `replace-converged`
    /// (the disposition's lie); we consult `Plan::render_refusal_diagnostics` and demote it.
    RenderRefusal,
    /// The disposition was `Run` but the dashboard could not localise a more specific
    /// reason from the public surfaces — carries the `{:?}` of the class for the seam
    /// wishlist (what attribution cannot see).
    Unattributed(String),
}

impl BlockReason {
    /// The short, stable slug for the table/TSV.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            BlockReason::TopTrigger => "top-trigger",
            BlockReason::NoOracle => "no-oracle",
            BlockReason::Diverged => "diverged",
            BlockReason::Unprobed => "unprobed",
            BlockReason::ConsumedStatusTop => "consumed-top-status",
            BlockReason::LoopCondition => "loop-condition",
            BlockReason::InLoopFloor => "in-loop-floor",
            BlockReason::ConsumedOutput => "consumed-output",
            BlockReason::WrittenUpstream => "written-upstream",
            BlockReason::RenderRefusal => "render-refusal",
            BlockReason::Unattributed(_) => "unattributed",
        }
    }
}

// ===========================================================================
// The dq-2 rung split (charter / `20V` §6)
// ===========================================================================

/// The dq-2 effort-rung population split (`20V` §6 / charter): for a site whose
/// elision is reachable *in principle*, by what EFFORT-RUNG is it (or could it be)
/// reached — a readable guard/wrapper idiom anyone can read, or an oracle
/// declaration the engineer must author?
///
/// This is the "how do we degrade gracefully" answer-shape: it tells the engineer
/// which sites are already paying off from cheap idioms vs which are waiting on the
/// door-2/door-4 declaration work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rung {
    /// **r-2/r-4-shaped** — elision reachable via a guard/wrapper idiom anyone can
    /// read: a fold (door-1) under a fold-usable Query, a Query-substitution, or the
    /// admin's own `|| true` (door-3). Zero new trust; the idiom is the proof.
    GuardReadable,
    /// **r-3-shaped** — a bare mutator (under `set -e` or otherwise consumed) whose
    /// only elision path is an oracle's converged-run DECLARATION (door-2/door-4).
    /// The site `runs` today; it is the population the declaration work would move.
    NeedsDeclaration,
    /// Not part of the rung split — the site is not an elision candidate at all
    /// (a `Run` blocked for a non-declaration reason: diverged, unprobed, ⊤-trigger,
    /// no-oracle, a loop floor, consumed output). Counted out so the split's
    /// denominator is honest.
    NotApplicable,
}

// ===========================================================================
// One site's row (c1–c4) + the whole report
// ===========================================================================

/// **c1** analyzability — a TRI-state, because the public `SkipClass` surface cannot
/// distinguish a ⊤-Opaque `MustRun` from a pure/kill/unreachable `MustRun` (the
/// underlying `CommandEffect` is not exposed). Asserting `bool` would lie about
/// `set -e`/`echo` (genuinely analyzable, just not elidable). The seam wishlist wants
/// a public per-site `CommandEffect`/⊤-reason readout to collapse `Indeterminate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Analyzable {
    /// The engine resolved a fact (establish/query/members) ⇒ definitely analyzable.
    Yes,
    /// `MustRun` — could be Opaque(⊤), pure, a kill, or unreachable; the public
    /// surface cannot tell which, so analyzability is unknown here (NOT asserted ⊤).
    Indeterminate,
}

/// One command-site's dashboard row — the charter's per-site c1–c4, plus the
/// criticality weight and the dq-2 rung. Keyed by the stable [`LeafId`]
/// (`inv-site-keyed-results`): the SAME id the probe record and apply plan use.
#[derive(Debug, Clone)]
pub struct SiteRow {
    /// The stable site id (probe-record / apply-leaf id space).
    pub site: LeafId,
    /// The verbatim command text (first line, whitespace-collapsed) for the table.
    pub command: String,
    /// 1-based source line of the site's first byte (provenance for the census
    /// spot-verify against `commands.tsv`).
    pub line: u32,
    /// **c1**: analyzable without collapsing to ⊤ — a tri-state ([`Analyzable`]):
    /// `Yes` when a fact resolved, `Indeterminate` for a `MustRun` (the public
    /// surface cannot split Opaque-⊤ from pure/kill).
    pub analyzable: Analyzable,
    /// **c1** detail: the ⊤-trigger / Opaque / indeterminacy reason; `None` when
    /// definitely analyzable.
    pub top_reason: Option<String>,
    /// **c2**: did an oracle resolve a fact here (`check()` + effect-map)?
    pub oracled: bool,
    /// **c3**: the probe's Effect verdict for the site's fact, when probe-results
    /// were supplied (`None` when no probe-results, or the site has no fact).
    pub probed: Option<Verdict>,
    /// **c4**: the door the disposition was reached through.
    pub door: Door,
    /// Criticality weight (line-count stand-in for the future 1A matrix weights —
    /// see [`weights`]). The criticality-weighted rollup sums these.
    pub weight: u32,
    /// The dq-2 effort-rung population this site falls in.
    pub rung: Rung,
}

/// The whole-book coverage report: every site's row + the rolled-up tallies. Sites
/// are in stable site-id order.
#[derive(Debug, Clone)]
pub struct Report {
    /// Per-site rows, in site-id order.
    pub rows: Vec<SiteRow>,
    /// Count-weighted per-door tally (one per site).
    pub by_door_count: BTreeMap<String, u32>,
    /// Criticality-weighted per-door tally (summing [`SiteRow::weight`]).
    pub by_door_weight: BTreeMap<String, u32>,
    /// The dq-2 rung-population split, count-weighted.
    pub rung_count: BTreeMap<&'static str, u32>,
    /// The dq-2 rung-population split, criticality-weighted.
    pub rung_weight: BTreeMap<&'static str, u32>,
    /// Count of render-refusal diagnostics the span-bridge could NOT attribute to a leaf
    /// (217 §5 obs-3). MUST be 0: a non-zero value means a refusal silently dropped, so a
    /// render-refused leaf was re-counted as an elision (the heredoc OVER-count fix-1 kills).
    /// The binary prints it as a loud warning; `> 0` is a "go re-read the span bridge" signal,
    /// the honest-blind-spot analog of [`Door::Unattributed`] keyed on the bridge not an enum.
    pub bridge_suspect: u32,
}

impl Report {
    /// Total site count.
    #[must_use]
    pub fn total_sites(&self) -> u32 {
        u32::try_from(self.rows.len()).unwrap_or(u32::MAX)
    }

    /// Total criticality weight across all sites (the denominator of the
    /// criticality-weighted coverage fraction).
    #[must_use]
    pub fn total_weight(&self) -> u32 {
        self.rows.iter().map(|r| r.weight).sum()
    }

    /// Count of FULL-elision sites (run-set shrinks) — the `20V` §7 numerator kept
    /// separate from guard-transforms.
    #[must_use]
    pub fn full_elided_count(&self) -> u32 {
        u32::try_from(
            self.rows
                .iter()
                .filter(|r| r.door.is_full_elision())
                .count(),
        )
        .unwrap_or(u32::MAX)
    }

    /// Criticality weight of FULL-elision sites — the north-star number's numerator
    /// (criticality-weighted full elision), NEVER blurred with guard-transforms.
    #[must_use]
    pub fn full_elided_weight(&self) -> u32 {
        self.rows
            .iter()
            .filter(|r| r.door.is_full_elision())
            .map(|r| r.weight)
            .sum()
    }

    /// Criticality weight of guard-TRANSFORM sites (door-4) — its own term (0 today).
    #[must_use]
    pub fn transform_weight(&self) -> u32 {
        self.rows
            .iter()
            .filter(|r| matches!(r.door, Door::GuardTransform))
            .map(|r| r.weight)
            .sum()
    }
}

// ===========================================================================
// Building the report — the engine round-trip + attribution
// ===========================================================================

/// The inputs the dashboard analyses: a book + its oracle sources + optional
/// probe-results (the site-keyed records the rendered probe would emit). Strings,
/// not paths, so the library stays I/O-free (`inv-determinism`); the binary reads
/// the files.
#[derive(Debug, Clone)]
pub struct Inputs<'a> {
    /// The book's sh source.
    pub book: &'a str,
    /// Each oracle file's sh source (mirrors the cli's `-o <oracle>` set).
    pub oracles: &'a [&'a str],
    /// The probe-results text (the `site <id> effect=… rc=…` records), if supplied.
    /// `None` ⇒ c3 is unknown everywhere and convergence-gated doors cannot mint
    /// (every elidable site shows `runs` with `unprobed`), which is the honest
    /// "no host answers" shape.
    pub probe_results: Option<&'a str>,
    /// Criticality weights per 1-based source line. Empty ⇒ the default
    /// line-count stand-in (every site weight 1). The future-1A adapter
    /// ([`weights::Weights`]) fills this.
    pub weights: &'a weights::Weights,
}

/// Build the coverage report by driving the SAME public pipeline the cli drives,
/// then attributing each site's engine outputs to a [`Door`] / [`BlockReason`] /
/// [`Rung`]. Pure: a deterministic function of `inputs` (the kernel it calls is
/// pure; the probe-results are parsed deterministically).
///
/// The site-id space is the engine's own (`build_plan`'s span-sorted [`LeafId`]s),
/// so a row's `site` matches the probe record and the apply leaf one-to-one.
#[must_use]
pub fn build_report(inputs: &Inputs<'_>) -> Report {
    let mut interner = Interner::default();

    // Shared interner across oracles + book ⇒ provider symbols match (cli parity).
    let lifted = dorc_oracle::lift(&mut interner, inputs.oracles);
    let idx = lifted.value;
    let checks: Vec<dorc_oracle::check::CheckSet> = inputs
        .oracles
        .iter()
        .map(|src| dorc_oracle::check::lift_checks(&mut interner, src).value)
        .collect();

    let parsed = dorc_syntax::parse(inputs.book);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut interner);
    let classes = dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut interner).value;

    // c3 source: per-site Effect verdict (the dashboard reads the plan's own
    // dispositions, not a fact re-key, so it only needs the verdict off the wire).
    let probe_verdicts: BTreeMap<LeafId, Verdict> = inputs
        .probe_results
        .map(parse_probe_verdicts)
        .unwrap_or_default();

    let probe = dorc_plan::compile_probe(&parsed.value, &cfg, &classes, |kind, selector| {
        idx.resolve_probe(kind, selector).map(|p| p.body.clone())
    });
    let observe = observe_from_sites(&probe, &probe_verdicts);
    let plan = dorc_plan::build_plan(inputs.book, &parsed.value, &cfg, &classes, observe);

    // classify yields CfgNodeId→SkipClass; the plan keys by AstId. Bridge via AstId.
    let disposition_of: BTreeMap<dorc_core::AstId, &Disposition> =
        plan.steps.iter().map(|s| (s.ast, &s.disposition)).collect();
    let leaf_of: BTreeMap<dorc_core::AstId, LeafId> =
        plan.steps.iter().map(|s| (s.ast, s.leaf)).collect();

    // fix-1 (the dashboard heredoc lie, `21B` hunt-1): a `Replace` disposition the leaf-exact
    // render REFUSES (a heredoc-bearing leaf) runs verbatim despite the disposition. Consult the
    // SAME refusal the render applies — `Plan::render_refusal_diagnostics` — and collect the
    // refused leaves so `attribute_door` demotes them to `runs(render-refusal)` instead of
    // mis-bucketing them `replace-converged` (the disposition's lie, 100% coverage that isn't).
    // `bridge_suspect` is the loud blind-spot count of refusals that did NOT bridge (217 §5 obs-3).
    let (render_refused, bridge_suspect) = render_refused_leaves(&plan, &parsed.value, &leaf_of);

    let mut rows: Vec<SiteRow> = classes
        .iter()
        .filter_map(|(node, class)| {
            let ast_id = cfg.node(*node).ast;
            let site = *leaf_of.get(&ast_id)?;
            let disposition = disposition_of.get(&ast_id).copied()?;
            Some(attribute_site(
                site,
                *node,
                class,
                disposition,
                &cfg,
                inputs.book,
                &parsed.value,
                &value,
                &probe_verdicts,
                &render_refused,
                inputs.weights,
                &interner,
            ))
        })
        .collect();
    rows.sort_by_key(|r| r.site.0);

    rollup(rows, bridge_suspect)
}

/// fix-1: the set of site [`LeafId`]s the leaf-exact render REFUSES to elide (a heredoc-bearing
/// `Replace`/neutralised-`Omit` leaf, `20V` §4 d-6). Consults the SAME public refusal the cli
/// reports — [`Plan::render_refusal_diagnostics`] — so the dashboard's "elided" set matches the
/// artifact the render emits, never the disposition's bare claim (the heredoc lie, `21B` hunt-1).
///
/// Each refusal diagnostic carries the refused leaf's source span; `leaf_of` keys leaves by
/// `AstId`, so we bridge span→leaf via the steps' own `AstId`→span. Returns the matched-leaf set
/// AND the `bridge_suspect` count of refusals that did NOT bridge (217 §5 obs-3 / the span-bridge
/// crosscheck's tier-1): see [`bridge_refusals_to_leaves`] for why that count is load-bearing.
fn render_refused_leaves(
    plan: &dorc_plan::Plan,
    ast: &dorc_syntax::ast::Ast,
    leaf_of: &BTreeMap<dorc_core::AstId, LeafId>,
) -> (std::collections::BTreeSet<LeafId>, u32) {
    // span (lo,hi) → leaf, from the steps (the render-refusal diagnostics carry the same span).
    let leaf_by_span: BTreeMap<(u32, u32), LeafId> = plan
        .steps
        .iter()
        .filter_map(|s| {
            let span = ast.node(s.ast).span;
            leaf_of
                .get(&s.ast)
                .map(|&leaf| ((span.lo.0, span.hi.0), leaf))
        })
        .collect();
    bridge_refusals_to_leaves(&plan.render_refusal_diagnostics(ast), &leaf_by_span)
}

/// The span-bridge from render-refusal diagnostics to leaf ids, returning the matched set AND a
/// `bridge_suspect` count of refusals that could NOT be attributed (a `None`-span diagnostic, or
/// one whose span matches no step). Split out as a pure function so the mismatch path is unit-
/// testable without constructing a whole [`dorc_plan::Plan`].
///
/// **Why count the misses (217 §5 obs-3 / the honest-blind-spot discipline):** a refusal that
/// silently fails to bridge means a leaf the render runs verbatim is NOT demoted — so the
/// dashboard re-counts it as an elision (`replace-converged`/`fold`), the exact flattering
/// OVER-count fix-1 exists to kill (`21B` hunt-1). The bridge is sound TODAY under
/// `inv-leaf-seam` span-disjointness (every refusal originates from a step with a unique span),
/// but a future lowering/render change that breaks span-identity must surface LOUDLY, never
/// regrow the heredoc lie. The count is reported in [`Report::bridge_suspect`] and the binary
/// prints it as a loud warning — the crate's `Unattributed`-style "report my own blind spot"
/// idiom, here keyed on the span bridge rather than an engine enum.
///
/// Matches are counted per-diagnostic (NOT as the matched set's len) so two refusals resolving
/// to one leaf — were span-disjointness ever violated — could not mask a miss by set-dedup.
fn bridge_refusals_to_leaves(
    refusals: &[dorc_core::Diagnostic],
    leaf_by_span: &BTreeMap<(u32, u32), LeafId>,
) -> (std::collections::BTreeSet<LeafId>, u32) {
    let mut refused = std::collections::BTreeSet::new();
    let mut unbridged: u32 = 0;
    for d in refusals {
        match d
            .span
            .and_then(|s| leaf_by_span.get(&(s.lo.0, s.hi.0)).copied())
        {
            Some(leaf) => {
                refused.insert(leaf);
            }
            None => unbridged = unbridged.saturating_add(1),
        }
    }
    (refused, unbridged)
}

/// Drive `build_plan`'s `observe` closure from the supplied per-site verdicts. The
/// cli re-keys site → fact through its private firewall; we cannot reach that, but
/// `build_plan` calls `observe(fact)`, so we map each resolvable site's fact to its
/// reported verdict here (status stays ⊤ — the dashboard never needs the fold's
/// status-relaxation precision, only the Effect verdict that gates full elision and
/// the disposition the plan lands on). A fact with no site-record ⇒ `Unknown` ⇒ runs
/// (`kFAIL-perform`), the honest "no answer" shape.
///
/// NB this is a *coarsening* of the cli's firewall: a Query guard's rc-relaxation
/// (which licenses `query-substituted` and fold) needs the *status* channel, which
/// we leave ⊤ here — so against a real probe-results file the dashboard will under-
/// count Query substitutions and folds relative to the cli. We DISCLOSE this in the
/// seam wishlist (the cli's `facts_from_sites`/firewall is private; a public
/// "site → Observable" seam would let the dashboard mirror the cli exactly). For the
/// Effect-gated doors (replace-converged, dead-invariant) the Effect verdict suffices
/// and the attribution is exact.
fn observe_from_sites<'a>(
    probe: &'a dorc_plan::ProbePlan,
    verdicts: &'a BTreeMap<LeafId, Verdict>,
) -> impl Fn(dorc_core::FactKey) -> dorc_core::Observable + 'a {
    use dorc_core::{Observable, Predicted, Rc};
    let mut by_fact: BTreeMap<dorc_core::FactKey, Observable> = BTreeMap::new();
    for check in &probe.checks {
        let verdict = verdicts
            .get(&check.site)
            .copied()
            .unwrap_or(Verdict::Unknown);
        // seam-2: the wire rc is not on this surface, so reconstruct a valid-Query rc
        // from its verdict (holds⇒0/absent⇒1, the oracle convention) — exact for the
        // Effect-gated doors, a coarsening for fold/query-substitute. Establish ⇒ ⊤.
        let status = match check.site_kind {
            dorc_plan::ProbeSiteKind::Query { valid: true } => match verdict {
                Verdict::Converged => Predicted::Value(Rc(0)),
                Verdict::Diverged => Predicted::Value(Rc(1)),
                Verdict::Unknown => Predicted::Top,
            },
            _ => Predicted::Top,
        };
        by_fact
            .entry(check.fact)
            .and_modify(|prior| {
                // Same-fact conflict ⇒ conservative ⊤ (mirrors the cli's merge floor).
                if prior.effect != verdict {
                    prior.effect = Verdict::Unknown;
                }
                if prior.status != status {
                    prior.status = Predicted::Top;
                }
            })
            .or_insert(Observable {
                effect: verdict,
                status,
                stdout: Predicted::Top,
                stderr: Predicted::Top,
            });
    }
    move |fact| {
        by_fact
            .get(&fact)
            .copied()
            .unwrap_or_else(|| Observable::verdict_only(Verdict::Unknown))
    }
}

/// Parse the probe-results wire into per-site Effect verdicts (the c3 source). Mirror
/// of the cli's grammar, narrowed to what the dashboard needs: `site <id>[.<m>]
/// effect=<holds|absent|cant-tell> …`. A member sub-record (`site N.M`) folds into its
/// parent site N for the dashboard's per-site c3 (the dashboard reports at site
/// granularity; member precision lives in the cli). `holds ⇒ Converged`, `absent ⇒
/// Diverged`, else `Unknown`. Unrecognised lines dropped (⇒ that site stays `Unknown`).
fn parse_probe_verdicts(input: &str) -> BTreeMap<LeafId, Verdict> {
    let mut out: BTreeMap<LeafId, Verdict> = BTreeMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        if it.next() != Some("site") {
            continue;
        }
        let Some(site) = it.next().and_then(parse_site_id) else {
            continue;
        };
        let mut verdict = Verdict::Unknown;
        for tok in it {
            if let Some(w) = tok.strip_prefix("effect=") {
                verdict = match w {
                    "holds" => Verdict::Converged,
                    "absent" => Verdict::Diverged,
                    _ => Verdict::Unknown,
                };
            }
        }
        // A member record folds into its parent site: a converged parent stays
        // converged only if ALL records agree converged (any non-converged ⇒ the
        // weakest, since the parent site elides all-or-nothing). Conservative meet.
        out.entry(site)
            .and_modify(|prior| {
                if *prior != verdict {
                    // disagreement among a site's records ⇒ Unknown (kFAIL-perform).
                    *prior = if *prior == Verdict::Converged && verdict == Verdict::Converged {
                        Verdict::Converged
                    } else {
                        Verdict::Unknown
                    };
                }
            })
            .or_insert(verdict);
    }
    out
}

/// Parse a site id token (`N` or `N.M` — the member sub-key folds to parent N).
fn parse_site_id(tok: &str) -> Option<LeafId> {
    let head = tok.split_once('.').map_or(tok, |(n, _)| n);
    Some(LeafId(head.parse::<u32>().ok()?))
}

/// Attribute one classified site to its [`Door`] / [`BlockReason`] / [`Rung`] +
/// fill the c1–c4 fields. This is the dashboard's only "judgement", and it reads
/// ONLY engine outputs: the [`SkipClass`] (c1/c2), the [`Disposition`] + consumed
/// channels (c4 door), the probe verdict (c3).
#[expect(
    clippy::too_many_arguments,
    reason = "the attribution reads many disjoint engine surfaces by reference; bundling them \
              into a struct would only move the plumbing, not reduce it (coverage is a leaf consumer)"
)]
fn attribute_site(
    site: LeafId,
    node: CfgNodeId,
    class: &SkipClass,
    disposition: &Disposition,
    cfg: &Cfg,
    src: &str,
    ast: &dorc_syntax::ast::Ast,
    value: &dorc_analysis::value::ValueFlow,
    probe_verdicts: &BTreeMap<LeafId, Verdict>,
    render_refused: &std::collections::BTreeSet<LeafId>,
    weights: &weights::Weights,
    interner: &Interner,
) -> SiteRow {
    let span = ast.node(cfg.node(node).ast).span;
    let command =
        command_oneline(value, node, interner).unwrap_or_else(|| span_text_oneline(src, span));
    let line = line_of_byte(src, span.lo.0);
    let end_line = line_of_byte(src, span.hi.0.saturating_sub(1).max(span.lo.0));

    // c1 analyzable + c2 oracled, from the SkipClass.
    let (analyzable, top_reason, oracled, fact) = classify_facts(class);
    // c3 probed: the site's Effect verdict (when probe-results supplied + a fact exists).
    let probed = fact.and(probe_verdicts.get(&site).copied());

    let consumed = cfg.consumed_observables(node);
    let door = attribute_door(
        disposition,
        consumed,
        class,
        probed,
        probe_verdicts.is_empty(),
        render_refused.contains(&site),
    );
    let rung = attribute_rung(&door, class);
    // Criticality weight = the span's line-count (stand-in for the 1A matrix).
    let weight = weights.weight_for_span(line, end_line);

    SiteRow {
        site,
        command,
        line,
        analyzable,
        top_reason,
        oracled,
        probed,
        door,
        weight,
        rung,
    }
}

/// c1/c2 from the [`SkipClass`]: (analyzable, top-reason, oracled, fact?). NON-
/// exhaustive over `SkipClass` — a new variant routes to the honest "modeled but
/// unrecognised" path (analyzable + not-oracled), never a silent miscount.
///
/// The `_ =>` arm is unreachable TODAY (`SkipClass` is currently closed) but is a
/// HARD charter requirement ("NEVER write an exhaustive match over engine enums; always
/// `_ =>` into an honest `unattributed` bucket so the crate survives engine evolution"
/// — the main tree is concurrently gaining a `SkipClass::InlineCall`). `#[expect]` over
/// `#[allow]` so when that variant lands the now-reachable arm makes the expectation
/// unfulfilled ⇒ a loud "re-read the engine, attribute the new class" signal.
#[expect(
    unreachable_patterns,
    reason = "charter-mandated evolution-survival catch-all; warns loudly when a new SkipClass lands"
)]
fn classify_facts(
    class: &SkipClass,
) -> (Analyzable, Option<String>, bool, Option<dorc_core::FactKey>) {
    match class {
        // MustRun conflates opaque-⊤ with pure/kill/unreachable (the CommandEffect is
        // not public — seam-1) ⇒ c1 `Indeterminate`, never a false ⊤-assertion.
        SkipClass::MustRun => (
            Analyzable::Indeterminate,
            Some("must-run: opaque-⊤ OR pure/kill/unreachable (CommandEffect not public)".into()),
            false,
            None,
        ),
        // ambient-vs-written is a c4 BlockReason distinction, not a c1/c2 one.
        SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => {
            (Analyzable::Yes, None, true, Some(*f))
        }
        SkipClass::QueryResolvable { fact, .. } => (Analyzable::Yes, None, true, Some(*fact)),
        SkipClass::EstablishMembers { members, .. } => {
            (Analyzable::Yes, None, true, members.first().copied())
        }
        // arch-2: the CALL aggregates its spliced body sites (i-3/i-4); oracled iff any
        // body site is, keyed (Members-style) on the first body fact for the c3 lookup.
        SkipClass::InlineCall { sites } => {
            let mut fact = None;
            let mut oracled = false;
            for s in sites {
                let (_, _, o, f) = classify_facts(&s.class);
                oracled |= o;
                if fact.is_none() {
                    fact = f;
                }
            }
            (Analyzable::Yes, None, oracled, fact)
        }
        // a new SkipClass: it classified ⇒ analyzable, but un-oracled-from-here.
        other => (
            Analyzable::Yes,
            Some(format!("new-skipclass:{other:?}")),
            false,
            None,
        ),
    }
}

/// The c4 DOOR for a site, from its [`Disposition`] + consumed channels + probe
/// verdict. NON-exhaustive over `Disposition`/`LicenseVia`: an unrecognised shape
/// becomes [`Door::Unattributed`] carrying the `{:?}` repr.
///
/// The door-3 discriminator (`20V` §4): a [`Disposition::Replace`] whose site's
/// consumed set carries [`Channel::StatusInvariant`] is `dead-invariant`, regardless
/// of its [`LicenseVia`] (the engine mints it via `ConvergedEstablish`, but the
/// licensing *reason* is invariance, not a claim-of-rc-0). Otherwise the `LicenseVia`
/// names the door.
///
/// Both `_ =>` arms are charter-mandated evolution-survival (unreachable today, since
/// `Disposition`/`LicenseVia` are closed; loud when a new variant lands — `#[expect]`).
#[expect(
    unreachable_patterns,
    reason = "charter-mandated evolution-survival fallbacks; warn loudly when a new Disposition/LicenseVia lands"
)]
fn attribute_door(
    disposition: &Disposition,
    consumed: &dorc_analysis::lattice::Powerset<Channel>,
    class: &SkipClass,
    probed: Option<Verdict>,
    no_probe_results: bool,
    render_refused: bool,
) -> Door {
    // fix-1 (the heredoc lie): the leaf-exact render REFUSES this leaf (a heredoc-bearing
    // `Replace`/neutralised-`Omit`), so the artifact RUNS it verbatim despite the disposition
    // licensing an elision. This OVERRIDES every elision door — a refused leaf is never elided in
    // fact — so it is the run-set residue, attributed `runs(render-refusal)`, never
    // `replace-converged`/`fold` (the disposition's bare claim, `21B` hunt-1).
    if render_refused {
        return Door::Runs(BlockReason::RenderRefusal);
    }
    match disposition {
        Disposition::Omit { .. } => Door::Fold,
        Disposition::Replace(license, _) => {
            // door-3 takes precedence: a StatusInvariant-consumed replace is dead-in-fact.
            if consumed.contains(&Channel::StatusInvariant) {
                return Door::DeadInvariant;
            }
            match license.derivation().via {
                // arch-2's InlineCall: the all-or-nothing CALL aggregate is a
                // converged-establish elision through a call — counted as
                // replace-converged (a per-door column for inline elision is a future
                // report decision, not a new door).
                LicenseVia::ConvergedEstablish
                | LicenseVia::MembersLoop
                | LicenseVia::InlineCall => Door::ReplaceConverged,
                LicenseVia::QueryGuard => Door::QuerySubstituted,
                // A NEW LicenseVia variant: name it loudly, don't fold into a door.
                other => Door::Unattributed(format!("Replace/LicenseVia::{other:?}")),
            }
        }
        Disposition::Run => Door::Runs(block_reason(class, consumed, probed, no_probe_results)),
        // A NEW Disposition variant the build predates ⇒ visible blind spot.
        other => Door::Unattributed(format!("{other:?}")),
    }
}

/// The dominant [`BlockReason`] for a `Run` site, by a fixed precedence so the
/// report names one root cause. Precedence (most-fundamental first): ⊤-trigger →
/// no-oracle/written → loop floor/condition → consumed output → consumed-⊤-status →
/// diverged → unprobed. NON-exhaustive over `SkipClass` (charter evolution-survival).
///
/// `no_probe_results` distinguishes the two `Unknown`-verdict causes: with no
/// probe-results at all the cause is `Unprobed` (the dashboard ran host-less); with
/// probe-results present but this site's record absent/`cant-tell` it is ALSO
/// `Unprobed` — but kept as a separate guard so a future "probed-but-cant-tell"
/// distinction has its hook.
#[expect(
    unreachable_patterns,
    reason = "charter-mandated evolution-survival catch-all; warns loudly when a new SkipClass lands"
)]
fn block_reason(
    class: &SkipClass,
    consumed: &dorc_analysis::lattice::Powerset<Channel>,
    probed: Option<Verdict>,
    no_probe_results: bool,
) -> BlockReason {
    let _ = no_probe_results; // reserved for the probed-but-cant-tell split (above)
    // MustRun ⇒ no-oracle: the public surface can't split opaque-⊤ from pure/kill
    // here (seam-1), so the coarse bucket is the honest floor.
    match class {
        SkipClass::MustRun => BlockReason::NoOracle,
        SkipClass::EstablishWritten(_) => BlockReason::WrittenUpstream,
        SkipClass::EstablishAmbient(_)
        | SkipClass::QueryResolvable { .. }
        | SkipClass::EstablishMembers { .. }
        // arch-2: a ran InlineCall blocks like any oracled site — the call's consumed
        // channels (the set-e ⊤-status shape) or its first body fact's verdict name why.
        | SkipClass::InlineCall { .. } => {
            // an oracled site that ran: channel set + verdict name why, by precedence.
            if consumed.contains(&Channel::Stdout) || consumed.contains(&Channel::Stderr) {
                BlockReason::ConsumedOutput
            } else if consumed.contains(&Channel::StatusIterated) {
                BlockReason::LoopCondition
            } else if in_loop_floor(class, consumed) {
                BlockReason::InLoopFloor
            } else if consumed.contains(&Channel::StatusRelaxable) {
                // A relaxable reader that still blocked ⇒ the rc was ⊤ (a known rc
                // would have relaxed it) — the bare-mutator-under-set-e shape.
                BlockReason::ConsumedStatusTop
            } else {
                match probed {
                    Some(Verdict::Diverged) => BlockReason::Diverged,
                    Some(Verdict::Unknown) | None => BlockReason::Unprobed,
                    Some(Verdict::Converged) => {
                        // Converged + oracled + no consumed blocker, yet it ran — a shape
                        // the public surface cannot localise (e.g. a ⊤-successor like
                        // `cmd &`, or a non-conforming establish status the cli firewalled).
                        BlockReason::Unattributed(format!("converged-but-ran ({class:?})"))
                    }
                }
            }
        }
        _ => BlockReason::Unattributed(format!("{class:?}")),
    }
}

/// Is this oracled `Run` site floored purely by being in a loop body (and not the
/// all-converged Members shape that lifts the floor)? A best-effort read: a Members
/// site that ran is the partial/non-self shape; a single-fact in-loop establish/query
/// is floored. We can only see the consumed set + class here, so this is a heuristic
/// the report marks `in-loop-floor` (vs the more specific consumed-status). It fires
/// only when no consumed-status blocker already explained the run.
fn in_loop_floor(class: &SkipClass, consumed: &dorc_analysis::lattice::Powerset<Channel>) -> bool {
    // A Members site that ran (license refused) is the in-loop all-or-nothing floor
    // expressing a partial/non-self refusal.
    matches!(class, SkipClass::EstablishMembers { .. })
        && !consumed.contains(&Channel::StatusRelaxable)
}

/// The dq-2 [`Rung`] population for a site, from its door + class (`20V` §6).
fn attribute_rung(door: &Door, class: &SkipClass) -> Rung {
    match door {
        // Already paying off from a readable idiom — r-2/r-4-shaped: fold (door-1),
        // query-substitute, the admin's own `|| true` (door-3), or a plain converged-
        // establish elision (the value-substitution the engine does with no declaration).
        Door::Fold | Door::QuerySubstituted | Door::DeadInvariant | Door::ReplaceConverged => {
            Rung::GuardReadable
        }
        // door-4/door-2 (when they land) are the declaration rung.
        Door::GuardTransform | Door::StaticDeclared => Rung::NeedsDeclaration,
        Door::Runs(reason) => match reason {
            // A bare oracled mutator whose status is consumed-⊤ (the `set -e` shape)
            // is EXACTLY the population an oracle converged-run declaration (door-2/4)
            // would move — r-3-shaped.
            BlockReason::ConsumedStatusTop => {
                if matches!(
                    class,
                    SkipClass::EstablishAmbient(_) | SkipClass::EstablishMembers { .. }
                ) {
                    Rung::NeedsDeclaration
                } else {
                    Rung::NotApplicable
                }
            }
            // Everything else that runs is not a declaration-movable elision candidate
            // (diverged/unprobed must run regardless; ⊤-trigger/no-oracle/output/loop
            // are not declaration-rescuable).
            _ => Rung::NotApplicable,
        },
        Door::Unattributed(_) => Rung::NotApplicable,
    }
}

// ===========================================================================
// Rollup
// ===========================================================================

/// Roll the per-site rows into the count- and criticality-weighted tallies. EVERY
/// door label is seeded to 0 first (including the not-yet-existing door-4/door-2), so
/// the table column set is stable regardless of which doors a particular book hits.
fn rollup(rows: Vec<SiteRow>, bridge_suspect: u32) -> Report {
    let mut by_door_count: BTreeMap<String, u32> = BTreeMap::new();
    let mut by_door_weight: BTreeMap<String, u32> = BTreeMap::new();
    for label in DOOR_COLUMNS {
        by_door_count.insert((*label).to_string(), 0);
        by_door_weight.insert((*label).to_string(), 0);
    }
    let mut rung_count: BTreeMap<&'static str, u32> = BTreeMap::new();
    let mut rung_weight: BTreeMap<&'static str, u32> = BTreeMap::new();
    for label in RUNG_COLUMNS {
        rung_count.insert(label, 0);
        rung_weight.insert(label, 0);
    }

    for row in &rows {
        let dl = row.door.label().to_string();
        bump(&mut by_door_count, dl.clone(), 1);
        bump(&mut by_door_weight, dl, row.weight);
        let rk = rung_label(row.rung);
        bump(&mut rung_count, rk, 1);
        bump(&mut rung_weight, rk, row.weight);
    }

    Report {
        rows,
        by_door_count,
        by_door_weight,
        rung_count,
        rung_weight,
        bridge_suspect,
    }
}

/// Add `n` to a tally key, saturating (`arithmetic_side_effects`: tallies never wrap —
/// a book with `u32::MAX` sites is not a real input, and saturating is the safe floor).
fn bump<K: Ord>(map: &mut BTreeMap<K, u32>, key: K, n: u32) {
    let slot = map.entry(key).or_insert(0);
    *slot = slot.saturating_add(n);
}

/// The stable door-column order for the rollup (the `20V` §7 doors, full-elisions
/// and guard-transforms kept separate; door-4/door-2 seeded at 0).
pub const DOOR_COLUMNS: &[&str] = &[
    "fold",
    "dead-invariant",
    "replace-converged",
    "query-substituted",
    "guard-transform",
    "static-declared",
    "runs",
    "unattributed",
];

/// The stable rung-column order (the dq-2 split).
pub const RUNG_COLUMNS: &[&str] = &["guard-readable", "needs-declaration", "not-applicable"];

/// The stable slug for a [`Rung`] (the table/TSV column + the rollup key).
#[must_use]
pub fn rung_label(rung: Rung) -> &'static str {
    match rung {
        Rung::GuardReadable => "guard-readable",
        Rung::NeedsDeclaration => "needs-declaration",
        Rung::NotApplicable => "not-applicable",
    }
}

// ===========================================================================
// Small display helpers (provenance only — `inv-referent-agnostic`)
// ===========================================================================

/// The site's resolved argv flattened to one space-joined line (`TOP` for an
/// unresolved word) — the value-flow view, the same the cli's `--debug-argv` prints.
/// For an in-loop **Members** site the single-fact argv shows the loop-var as `TOP`,
/// which is misleading; so we prefer the per-member view (`member_argv`), rendering
/// the command with the member operands joined `{a|b|c}` (the loop iterates them).
/// `None` when the node has no argv (then the caller falls back to the source span).
fn command_oneline(
    value: &dorc_analysis::value::ValueFlow,
    node: CfgNodeId,
    interner: &Interner,
) -> Option<String> {
    if let Some(members) = value.member_argv(node)
        && let Some(label) = members_oneline(members, interner)
    {
        return Some(label);
    }
    let words = value.argv_values(node);
    if words.is_empty() {
        return None;
    }
    Some(argv_oneline(&words, interner))
}

/// Render an argv as a space-joined line (`TOP` for ⊤ words).
fn argv_oneline(words: &[ValueOf], interner: &Interner) -> String {
    words
        .iter()
        .map(|w| match w {
            ValueOf::Literal(sym) => interner.resolve(*sym).to_string(),
            ValueOf::Top => "TOP".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render an in-loop Members site as `cmd flags... {m0|m1|…}` — the command prefix
/// shared by every member, with the varying operand shown as the member alternation.
/// `None` if there are no members. Picks the FIRST member's argv as the prefix
/// template, replacing the position(s) that differ across members with the
/// `{…}`-joined member operands (a readable approximation; provenance only).
fn members_oneline(members: &[Vec<ValueOf>], interner: &Interner) -> Option<String> {
    let first = members.first()?;
    // The operand that varies is the for-var's position; show every member's whole
    // argv joined by `|` if they differ, else the single shared argv. Cheap + honest:
    // join each member's one-line argv with `|`, then collapse a shared prefix.
    let lines: Vec<String> = members.iter().map(|m| argv_oneline(m, interner)).collect();
    let head = lines.first()?;
    if lines.iter().all(|l| l == head) {
        return Some(head.clone());
    }
    // Differing members: show `<prefix> {alt0|alt1|…}` using the last word of each as
    // the alternation (the common Members shape is a trailing operand).
    let prefix = argv_oneline(
        first.get(..first.len().saturating_sub(1)).unwrap_or(&[]),
        interner,
    );
    let alts: Vec<String> = members
        .iter()
        .filter_map(|m| m.last())
        .map(|w| match w {
            ValueOf::Literal(sym) => interner.resolve(*sym).to_string(),
            ValueOf::Top => "TOP".to_string(),
        })
        .collect();
    Some(format!("{prefix} {{{}}}", alts.join("|")))
}

/// The node's source span text, whitespace-collapsed to one line — the fallback
/// command label when value-flow yields no argv (a bare assignment / structural
/// leaf). Span resolution for display is allowed under `inv-referent-agnostic`.
fn span_text_oneline(src: &str, span: dorc_core::Span) -> String {
    let text = src
        .get(span.lo.0 as usize..span.hi.0 as usize)
        .unwrap_or_default();
    let one = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if one.is_empty() {
        "<empty>".to_string()
    } else {
        one
    }
}

/// 1-based source line of a byte offset into `src` (the provenance line for the
/// census spot-verify against `commands.tsv`). Counts newlines before the byte.
fn line_of_byte(src: &str, byte: u32) -> u32 {
    let upto = src.get(..byte as usize).unwrap_or(src);
    u32::try_from(upto.bytes().filter(|&b| b == b'\n').count())
        .unwrap_or(u32::MAX)
        .saturating_add(1)
}

// ===========================================================================
// Per-door attribution tests (charter (a): "unit tests asserting per-door
// attributions match known dispositions"). Each pins ONE door against a
// known-disposition book + oracle + probe, mirroring the e2e cases.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The package oracle (apt/dpkg-query establish) — install/purge mutators. Mirrors
    /// the e2e `package.oracle.sh` idiom.
    const PACKAGE_ORACLE: &str = r#"
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge kill installed
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
}
"#;

    /// The pkgstate QUERY oracle (`dpkg -s X` reads installed-ness) — a read-only
    /// `Queries` cell, the fold-usable guard. Mirrors the e2e `pkgstate.oracle.sh`
    /// (an EXTERNAL query, mock-reproducible, unlike the builtin `command -v`).
    const PKGSTATE_ORACLE: &str = r#"
oracle_kind=pkgstate
oracle_probe_pkgstate() { dpkg -s "$1" >/dev/null 2>&1; }
oracle_effect dpkg '' query installed
dpkg__check() {
   case $1 in -s) shift ;; esac
   pkg : pkgstate = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1
}
"#;

    fn report(book: &str, probe: Option<&str>) -> Report {
        let weights = weights::Weights::line_count_standin();
        let inputs = Inputs {
            book,
            oracles: &[PACKAGE_ORACLE, PKGSTATE_ORACLE],
            probe_results: probe,
            weights: &weights,
        };
        build_report(&inputs)
    }

    /// The door at a given site id (panics if absent — the test wants that site).
    fn door_at(r: &Report, site: u32) -> Door {
        match r.rows.iter().find(|row| row.site.0 == site) {
            Some(row) => row.door.clone(),
            None => panic!("no site {site} in report: {:?}", site_ids(r)),
        }
    }

    fn site_ids(r: &Report) -> Vec<u32> {
        r.rows.iter().map(|row| row.site.0).collect()
    }

    /// No site is ever attributed `Unattributed` on a fully-modeled book — the
    /// unattributed bucket masking a systematic gap is the headline adversarial risk.
    fn assert_no_unattributed(r: &Report) {
        for row in &r.rows {
            assert!(
                !matches!(row.door, Door::Unattributed(_)),
                "site {} unexpectedly unattributed: {:?} ({})",
                row.site.0,
                row.door,
                row.command,
            );
            if let Door::Runs(BlockReason::Unattributed(why)) = &row.door {
                panic!(
                    "site {} runs-unattributed: {why} ({})",
                    row.site.0, row.command
                );
            }
        }
    }

    #[test]
    fn heredoc_bearing_converged_mutator_reports_render_refusal_not_elided() {
        // fix-1 (the dashboard heredoc lie, `21B` hunt-1): a converged heredoc-bearing mutator
        // gets `Disposition::Replace` (the install establishes `package:nginx`, holds), but the
        // leaf-exact render REFUSES to edit its span (the span covers `<<EOF`, not the body) and
        // runs it VERBATIM. The dashboard must consult the SAME render refusal and bucket it
        // `runs(render-refusal)`, NEVER `replace-converged` — and report 0% coverage, not the
        // 100% the disposition alone would claim.
        let book = "apt-get install -y nginx <<EOF\nconfig line\nEOF\n";
        let r = report(book, Some("site 0 effect=holds\n"));
        assert_eq!(
            door_at(&r, 0),
            Door::Runs(BlockReason::RenderRefusal),
            "a render-refused heredoc mutator runs verbatim — never replace-converged: {:?}",
            r.rows
                .iter()
                .map(|x| (x.site.0, x.door.clone()))
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            r.full_elided_count(),
            0,
            "no site is a full elision (the render runs it): {r:?}"
        );
        assert_eq!(
            r.full_elided_weight(),
            0,
            "0% criticality-weighted coverage (the heredoc lie corrected)"
        );
        assert_no_unattributed(&r);
        // The happy-path bridge attributes its one refusal cleanly ⇒ no blind spot (217 §5 obs-3).
        assert_eq!(
            r.bridge_suspect, 0,
            "the heredoc refusal bridges to its leaf — no unattributed refusal: {r:?}"
        );
    }

    /// A render-refusal diagnostic shaped like the real one (`render-heredoc-refused`).
    fn refusal_diag(span: Option<dorc_core::Span>) -> dorc_core::Diagnostic {
        dorc_core::Diagnostic::error(
            dorc_core::DiagCode("render-heredoc-refused"),
            span,
            "leaf-exact render refuses a heredoc-bearing leaf",
        )
    }

    fn span_at(lo: u32, hi: u32) -> dorc_core::Span {
        dorc_core::Span {
            lo: dorc_core::BytePos(lo),
            hi: dorc_core::BytePos(hi),
        }
    }

    #[test]
    fn bridge_counts_a_none_span_refusal_as_suspect() {
        // 217 §5 obs-3 tier-1: a refusal with NO span cannot reach a leaf, so the demotion is
        // silently lost ⇒ a render-refused leaf would re-count as an elision (the heredoc lie).
        // It MUST be counted loud, not dropped. (None-span refusals don't arise today — every
        // refusal carries `Some(step.span)` — but a future refusal path that forgets the span
        // must trip this, not silently regrow the over-count.)
        let leaf_by_span: BTreeMap<(u32, u32), LeafId> =
            [((0, 10), LeafId(0))].into_iter().collect();
        let (matched, suspect) = bridge_refusals_to_leaves(&[refusal_diag(None)], &leaf_by_span);
        assert!(matched.is_empty(), "a None-span refusal bridges to nothing");
        assert_eq!(
            suspect, 1,
            "the unbridged refusal is counted, never dropped"
        );
    }

    #[test]
    fn bridge_counts_an_unmatched_span_refusal_as_suspect() {
        // The other silent-drop arm: a refusal whose span matches NO step (span-identity broke —
        // the obs-3 failure mode). Must surface, never silently miss the demotion.
        let leaf_by_span: BTreeMap<(u32, u32), LeafId> =
            [((0, 10), LeafId(0))].into_iter().collect();
        let (matched, suspect) =
            bridge_refusals_to_leaves(&[refusal_diag(Some(span_at(99, 105)))], &leaf_by_span);
        assert!(matched.is_empty(), "a non-matching span bridges to nothing");
        assert_eq!(suspect, 1, "the unmatched refusal is counted loudly");
    }

    #[test]
    fn bridge_matches_a_known_span_with_no_suspect() {
        // The positive control: a refusal whose span IS a step's span bridges to that leaf and is
        // NOT counted suspect — so the tripwire fires ONLY on a real miss, never on the happy path.
        let leaf_by_span: BTreeMap<(u32, u32), LeafId> =
            [((0, 10), LeafId(7))].into_iter().collect();
        let (matched, suspect) =
            bridge_refusals_to_leaves(&[refusal_diag(Some(span_at(0, 10)))], &leaf_by_span);
        assert_eq!(
            matched.into_iter().collect::<Vec<_>>(),
            vec![LeafId(7)],
            "the matching refusal bridges to its leaf"
        );
        assert_eq!(suspect, 0, "a bridged refusal is not suspect");
    }

    #[test]
    fn replace_converged_lone_install() {
        // A lone converged install => replace-converged (the bread-and-butter full
        // elision). Effect=holds gates it; no set -e, no consumer => it mints.
        let r = report("apt-get install -y nginx\n", Some("site 0 effect=holds\n"));
        assert_eq!(door_at(&r, 0), Door::ReplaceConverged);
        assert_no_unattributed(&r);
        assert!(
            r.full_elided_weight() >= 1,
            "a full elision is counted: {r:?}"
        );
    }

    #[test]
    fn dead_invariant_door3_or_true() {
        // `apt-get install -y nginx || true` under set -e, CONVERGED => door-3
        // dead-invariant (the install's rc is StatusInvariant-consumed, so the
        // convergence-elision mints despite the top status). The e2e door3-or-true-elides.
        let r = report(
            "set -e\napt-get install -y nginx || true\n",
            Some("site 1 effect=holds\n"),
        );
        assert_eq!(
            door_at(&r, 1),
            Door::DeadInvariant,
            "the `|| true` install must attribute door-3 dead-invariant: {:?}",
            site_ids(&r),
        );
        assert_no_unattributed(&r);
    }

    #[test]
    fn dead_invariant_diverged_still_runs() {
        // door-3 clears only Status; Effect still gates. `|| true` install, DIVERGED =>
        // runs (the e2e door3-or-true-diverged-runs). Proof door-3 is not an
        // elision-license relaxation.
        let r = report(
            "set -e\napt-get install -y nginx || true\n",
            Some("site 1 effect=absent\n"),
        );
        assert_eq!(door_at(&r, 1), Door::Runs(BlockReason::Diverged));
        assert_no_unattributed(&r);
    }

    #[test]
    fn query_substituted_lone_guard() {
        // A bare valid Query guard, holds => query-substituted (value-substituted to its
        // probed rc). NOT a mutation-elision; its own column.
        let r = report(
            "dpkg -s ca-certificates >/dev/null 2>&1\n",
            Some("site 0 effect=holds rc=0\n"),
        );
        assert_eq!(door_at(&r, 0), Door::QuerySubstituted);
        assert_no_unattributed(&r);
    }

    #[test]
    fn fold_door1_oror_guard() {
        // `dpkg -s nginx || apt-get install nginx`, guard holds (rc 0) => the install is
        // provably-dead control-flow under the MEASURED guard rc => the install is FOLDED
        // (door-1). The guard itself is query-substituted. The canonical idempotency idiom.
        let r = report(
            "dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx\n",
            Some("site 0 effect=holds rc=0\n"),
        );
        // site 0 is the guard (span-first), site 1 the install.
        assert_eq!(door_at(&r, 0), Door::QuerySubstituted, "guard substituted");
        assert_eq!(
            door_at(&r, 1),
            Door::Fold,
            "the dead install must attribute door-1 fold: {:?}",
            site_ids(&r),
        );
        assert_no_unattributed(&r);
    }

    #[test]
    fn runs_diverged_install() {
        // A lone install, DIVERGED => runs/diverged (Effect gates).
        let r = report("apt-get install -y nginx\n", Some("site 0 effect=absent\n"));
        assert_eq!(door_at(&r, 0), Door::Runs(BlockReason::Diverged));
    }

    #[test]
    fn runs_unprobed_when_no_results() {
        // No probe-results at all => the elidable install runs/unprobed (the honest
        // host-less shape; anti-masking: no fabricated convergence).
        let r = report("apt-get install -y nginx\n", None);
        assert_eq!(door_at(&r, 0), Door::Runs(BlockReason::Unprobed));
    }

    #[test]
    fn runs_consumed_top_status_under_set_e() {
        // The errexit headline cost (20V s1): a bare converged install under `set -e`
        // => its rc is StatusRelaxable-consumed and top (fork-mutator-rc) => runs with
        // consumed-top-status, the r-3 (needs-declaration) population door-2/door-4 move.
        let r = report(
            "set -e\napt-get install -y nginx\n",
            Some("site 1 effect=holds\n"),
        );
        assert_eq!(
            door_at(&r, 1),
            Door::Runs(BlockReason::ConsumedStatusTop),
            "a converged install under set -e runs (consumed-top-status): {:?}",
            site_ids(&r),
        );
        // It is the declaration-movable rung (the dq-2 split's whole point).
        let row = r.rows.iter().find(|x| x.site.0 == 1).unwrap();
        assert_eq!(row.rung, Rung::NeedsDeclaration);
    }

    #[test]
    fn members_loop_all_converged_replace_converged() {
        // `for pkg in nginx curl; do apt-get install -y "$pkg"; done`, both members
        // converged => the in-loop all-or-nothing license elides the body
        // (MembersLoop => replace-converged). The e2e loop-members-all-converged-elides.
        let r = report(
            "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\n",
            Some("site 0.0 effect=holds\nsite 0.1 effect=holds\n"),
        );
        assert_eq!(
            door_at(&r, 0),
            Door::ReplaceConverged,
            "an all-converged Members loop elides (replace-converged): {:?}",
            site_ids(&r),
        );
        assert_no_unattributed(&r);
    }

    #[test]
    fn written_upstream_purge_then_install() {
        // `purge X; install X` (same cell) => the install is EstablishWritten => runs with
        // written-upstream (its resting probe is stale). Pins the c4 reason precision.
        let r = report(
            "apt-get purge nginx\napt-get install -y nginx\n",
            Some("site 1 effect=holds\n"),
        );
        assert_eq!(door_at(&r, 1), Door::Runs(BlockReason::WrittenUpstream));
    }

    #[test]
    fn unmodeled_command_is_indeterminate_not_top_assertion() {
        // An un-oracled command (`ufw allow`) => MustRun => c1 Indeterminate (NOT a false
        // top-assertion), runs/no-oracle, rung not-applicable. The MustRun-conflation seam.
        let r = report("ufw allow 80/tcp\n", None);
        let row = r.rows.iter().find(|x| x.site.0 == 0).expect("site 0");
        assert_eq!(row.analyzable, Analyzable::Indeterminate);
        assert_eq!(row.door, Door::Runs(BlockReason::NoOracle));
        assert_eq!(row.rung, Rung::NotApplicable);
    }

    #[test]
    fn rollup_has_every_door_column_seeded_zero() {
        // The instrument's stable shape: every door column present even when the book
        // hits none of them (door-4/door-2 always 0 today).
        let r = report("ufw allow 80/tcp\n", None);
        for label in DOOR_COLUMNS {
            assert!(
                r.by_door_count.contains_key(*label),
                "door column `{label}` seeded"
            );
        }
        assert_eq!(r.by_door_count.get("guard-transform").copied(), Some(0));
        assert_eq!(r.by_door_count.get("static-declared").copied(), Some(0));
    }

    #[test]
    fn determinism_same_input_same_report() {
        // inv-determinism: byte-identical rollups from identical input, twice.
        let book = "set -e\napt-get install -y nginx || true\napt-get install -y curl\n";
        let a = report(book, Some("site 1 effect=holds\nsite 2 effect=holds\n"));
        let b = report(book, Some("site 1 effect=holds\nsite 2 effect=holds\n"));
        assert_eq!(a.by_door_count, b.by_door_count);
        assert_eq!(a.by_door_weight, b.by_door_weight);
        assert_eq!(a.rung_count, b.rung_count);
    }
}
