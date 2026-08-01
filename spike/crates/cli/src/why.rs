//! The `dorc why` REPORT, composed as a stamped part stream.
//!
//! It lives on the lib side of `lib-target-is-a-loom-seam` so a loom case can drive the real
//! report in-process and carry an editable transcript of it (`28L:rul-full-driver-this-arc`).
//! Nothing here reads the world: every input is an already-resolved VALUE the binary computed
//! (`io-at-edges-only`), and the one output is a [`dorc_aid::tagged::RenderParts`] the caller
//! prints or attributes an edit against.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::chain::{ChainLink, ChainModel, Excerpt};
use dorc_aid::diag::Diag;
use dorc_aid::said::{Said, WHY_SOURCE_CAP, WHY_VALUE_CAP};
use dorc_aid::tagged::RenderParts;
use dorc_aid::weave::Face;
use dorc_aid::{CollapseKind, CollapseNarrative, Knowability, RenderCtx, SpeechAct};
use dorc_core::{Interner, ProvArena};
use weft::{
    Banner, Branch, CodeBlock, CodeCell, CodeLine, Join, LabeledRow, Literalness, Node, NodeKind,
    Paragraph, Payload, PointerLine, Quoting, Run, Section, SpeakerRow, Truncation,
};

use crate::{CONSENT_FLAG, Receipt, paragraph, receipt_banner, registry_paragraph, why_parts};

/// Why one site's guard became trustworthy only after the fixpoint ran (`26H` §4.6 —
/// ATTRIBUTION IS A HARD REQUIREMENT). Decision-inert: the aid plane reads it, nothing else.
#[derive(Debug)]
pub struct CascadeAttribution {
    /// Source lines of the erased mutators that had to be proven dead first, in book order.
    pub erased_lines: Vec<usize>,
    /// The line whose measured rc proved the last of them dead.
    pub controller_line: usize,
    /// The round that proof landed in.
    pub round: u32,
}

/// cheap-7: is this command source text a STRUCTURALLY-UNPROBEABLE site — one for which no
/// read-only probe could ever be authored, so the firehose disclosure would be actively-wrong
/// advice? Two shapes: a bare assignment (`NAME=value`, no command), and a pure/no-target-state
/// builtin (the engine's OWN [`dorc_analysis::effect::is_target_state_pure_builtin`] list — never a
/// parallel notion). Everything else (a real un-oracled command like `make install`) is a genuine
/// "runs unprobed" the admin should see aggregated.
#[must_use]
pub fn is_structurally_unprobeable(cmd_text: &str) -> bool {
    let first = cmd_text.split_whitespace().next().unwrap_or("");
    // A bare assignment: `NAME=…` where NAME is a valid sh name (no command word to probe).
    if let Some((name, _)) = first.split_once('=')
        && !name.is_empty()
        && name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return true;
    }
    dorc_analysis::effect::is_target_state_pure_builtin(first)
}

/// Collapse interior whitespace runs (incl. newlines) to single spaces for a ONE-LINE disclosure
/// of a possibly multi-line command — the aggregate Note stays one line per rul-attention-honesty's
/// compactness (the artifact still carries the verbatim command).
#[must_use]
pub fn flatten_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Resolve a threaded oracle `(Span, SourceFileId)` to a `path:line` locus (C7 `file:line`;
/// `law-lineno-identity` — the file id disambiguates WHICH oracle's line-number space, since a
/// bare span is file-ambiguous once >1 oracle is loaded). `None` when the vouch/claim was
/// unthreaded, or the id is out of range (the render omits the locus — never fabricates one).
#[must_use]
pub fn oracle_locus(
    defining: Option<(dorc_core::Span, dorc_core::SourceFileId)>,
    source_paths: &[String],
    source_srcs: &[String],
) -> Option<String> {
    let (span, file) = defining?;
    let i = file.0 as usize;
    let (path, src) = (source_paths.get(i)?, source_srcs.get(i)?);
    let (line, _col) = dorc_aid::diag::line_col(src, span.lo.0 as usize);
    Some(format!("{path}:{line}"))
}

/// The most lines of an author's arm the surface will inline before cutting a middle out of it.
const EXCERPT_LINES: usize = 8;
/// The most preceding comment lines attached to an arm.
const EXCERPT_COMMENT_LINES: usize = 4;

/// Slice an oracle's own source around a threaded span, for display beneath the row that quotes it.
///
/// The massaging is licensed and bounded (`27W:rul-report-surface-massaging`): the CONTRIBUTING
/// lines are the span's own, the author's ADJACENT comment block is attached because a comment
/// above an arm is the author explaining that arm, and a long middle is CUT with the cut shown.
/// Authorship-implying and repair-directing, never byte-obligated — and never runnable, which is
/// why the render marks any cut rather than quietly closing over it.
///
/// `None` when the span was unthreaded or its file is out of range: an absent excerpt is an
/// omission, never a fabrication.
fn oracle_excerpt(
    defining: Option<(dorc_core::Span, dorc_core::SourceFileId)>,
    source_paths: &[String],
    source_srcs: &[String],
) -> Option<Excerpt> {
    let (span, file) = defining?;
    let index = file.0 as usize;
    let (path, src) = (source_paths.get(index)?, source_srcs.get(index)?);
    let source: Vec<&str> = src.lines().collect();
    // A span ending at end-of-file resolves PAST the last line, so both ends clamp.
    let first = dorc_aid::diag::line_col(src, span.lo.0 as usize)
        .0
        .min(source.len());
    let last = dorc_aid::diag::line_col(src, span.hi.0 as usize)
        .0
        .min(source.len())
        .max(first);

    let mut start = first;
    while start > 1
        && first.saturating_sub(start) < EXCERPT_COMMENT_LINES
        && source
            .get(start.saturating_sub(2))
            .is_some_and(|line| line.trim_start().starts_with('#'))
    {
        start = start.saturating_sub(1);
    }

    let numbered = |line: usize| {
        source
            .get(line.saturating_sub(1))
            .map(|text| (line, (*text).to_owned()))
    };
    let shown = last.saturating_sub(start).saturating_add(1);
    if shown <= EXCERPT_LINES {
        return Some(Excerpt {
            path: path.clone(),
            head: (start..=last).filter_map(numbered).collect(),
            tail: Vec::new(),
            elided: 0,
        });
    }
    // Keep the head and the last line, and say how much was dropped between them.
    let head_end = start.saturating_add(EXCERPT_LINES).saturating_sub(2);
    Some(Excerpt {
        path: path.clone(),
        head: (start..=head_end).filter_map(numbered).collect(),
        tail: std::iter::once(last).filter_map(numbered).collect(),
        elided: last.saturating_sub(head_end).saturating_sub(1),
    })
}

/// upcoming-firstwall-hint (`USER_STORY` stage 3): the role a plan step plays in the poison-wall
/// walk, reduced for the first-wall hint. The wall the hint TARGETS is specifically an UNMODELED
/// (opaque) running command — the class an oracle could describe; a modeled-but-diverged wall is
/// honest and never the hint's subject ("the hint's whole point is 'an oracle would help HERE'").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallRole {
    /// A running UNMODELED (opaque) command — the poison wall. Its run ⊤-poisons every downstream
    /// fact in `classify` (⇒ `EstablishWritten` ⇒ guard-or-run), so no downstream elision survives
    /// it. Detected at the cli edge as a probe-UNRESOLVABLE real command — the same
    /// `probe.unresolvable` ∩ not-[`is_structurally_unprobeable`] set the firehose already discloses.
    Opaque,
    /// A running MODELED mutator (a diverged establish) or a kill — an HONEST wall. It BOUNDS the
    /// un-wall count (a downstream guard past it is walled by IT, not by the opaque wall), but is
    /// never the hint's subject: it is already described by an oracle.
    Honest,
    /// A converged-but-walled GUARDED site — it would upgrade guard→elide if the wall above it
    /// lifted ("an elided command casts no wall"). The un-wall count tallies exactly these, in the
    /// first opaque wall's own window.
    Guard,
    /// Transparent to the walk — an elision, an omit, or an inert running builtin: neither a wall
    /// nor an improvable guard.
    Transparent,
}

impl WallRole {
    /// The occurrence a wall row's payload is keyed by, so an UNDESCRIBED wall and a described
    /// running mutator never wear each other's words: one may touch anything because nobody said
    /// otherwise, the other has an author who said exactly what it touches.
    const fn occurrence(self) -> usize {
        match self {
            WallRole::Opaque => 0,
            WallRole::Honest | WallRole::Guard | WallRole::Transparent => 1,
        }
    }
}

/// One plan step reduced to (leaf, source line, command word, wall role) for [`first_wall_hint`].
/// `line` is the SOURCE line (rul24-lineno-identity); `word` is the command's first word — the
/// `'hork'` the hint names.
#[derive(Debug)]
pub struct WallStep {
    /// The plan step this row reduces.
    pub leaf: dorc_plan::LeafId,
    /// Its SOURCE line.
    pub line: usize,
    /// Its command word.
    pub word: String,
    /// The role it plays in the wall walk.
    pub role: WallRole,
}

/// The first-wall hint payload (upcoming-firstwall-hint / `USER_STORY` stage 3): the FIRST opaque
/// wall in book order and the counterfactual un-wall count.
#[derive(Debug)]
pub struct FirstWallHint {
    /// The wall site's leaf — the `dorc why` detail attaches to exactly this site.
    pub leaf: dorc_plan::LeafId,
    /// The wall's SOURCE line (rul24-lineno-identity: queryable as `dorc why book.sh:line`).
    pub line: usize,
    /// The wall command's first word (`'hork'`).
    pub word: String,
    /// `M` — the un-wall count: the number of converged-but-walled GUARD sites strictly between
    /// this wall and the next wall (opaque or honest). These are the sites that would upgrade
    /// guard→elide if this wall's command were modeled-and-converged ("an elided command casts no
    /// wall").
    ///
    /// CONSERVATIVE APPROXIMATION (flagged — the honest counterfactual is NOT a plan-level re-fold):
    /// an opaque wall's poison is applied in `classify` (⊤-reach ⇒ downstream `EstablishWritten`),
    /// NOT in the plan wall-walk, so "re-fold the plan with this wall treated as non-walling" cannot
    /// un-poison — the honest count needs a re-CLASSIFY with the command's effect forced Pure. This
    /// tally instead counts the walled guards in the wall's own window. It is EXACT in the common
    /// case and OVER-counts only when a downstream guard is `EstablishWritten` from a same-cell
    /// in-script write rather than from this wall (the `install X; hork; install X` shape), where
    /// lifting this wall alone would not recover it. Erring high on an advisory nag is acceptable.
    pub unwall: usize,
    /// Other opaque walls after the first — the trailing "N more unmodeled walls" pointer count.
    pub more_walls: usize,
}

impl FirstWallHint {
    /// The hint body (no `hint: ` prefix — the caller adds it, matching the `why:`/`dorc:` lanes).
    /// `USER_STORY` stage-3 register; plain English (24H ack-4 — no ⊤, no jargon; "unmodeled" is
    /// established vocabulary).
    #[must_use]
    pub fn body(&self) -> String {
        let unwall_clause = if self.unwall == 0 {
            String::new()
        } else {
            let sites = if self.unwall == 1 { "site" } else { "sites" };
            format!(", and un-wall {} downstream {sites}", self.unwall)
        };
        let more_clause = if self.more_walls == 0 {
            String::new()
        } else {
            let walls = if self.more_walls == 1 {
                "wall"
            } else {
                "walls"
            };
            format!("; {} more unmodeled {walls} -- dorc why", self.more_walls)
        };
        format!(
            "'{}' (line {}) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged{unwall_clause}{more_clause}",
            self.word, self.line
        )
    }

    /// The `dorc why` detail row for the wall's own site (the reasoning behind the plan-mode nag).
    /// Registry-homed like every other why-surface string (`28G` §0), and stated in admin-English:
    /// the engine's `elide` never reaches a render.
    ///
    /// TWO sentence forms, keyed by occurrence, because there are two: with a recovery clause and
    /// without. The one-row shape fed the recovery slot an EMPTY value when nothing was recoverable
    /// — a seat defect under `28H` ruling 1, since a value that renders as nothing is a value the
    /// span map cannot carry and a human cannot edit around. Occurrence 0 has no value at all; the
    /// registry's occurrence discriminator is exactly for one thing said two ways
    /// (`why-operand-position` is the precedent).
    fn why_said(&self) -> Said {
        if self.unwall == 0 {
            return Said::words_at("why-reason-first-wall", Some(0), &[]);
        }
        Said::sentence(
            "why-reason-first-wall",
            Some(1),
            vec![Said::words(
                "why-reason-first-wall-unwall",
                &[&self.unwall.to_string()],
            )],
        )
    }
}

/// upcoming-firstwall-hint: the PURE first-wall computation. Find the first [`WallRole::Opaque`]
/// step in book order; tally the [`WallRole::Guard`] steps between it and the next wall (opaque or
/// honest) as the un-wall count; count the remaining opaque walls for the trailing pointer.
/// `None` ⇒ no opaque wall ⇒ no hint. Pure + total (`inv-determinism`); unit-tested over
/// hand-built scenarios.
#[must_use]
pub fn first_wall_hint(steps: &[WallStep]) -> Option<FirstWallHint> {
    let w1 = steps.iter().position(|s| s.role == WallRole::Opaque)?;
    let wall = steps.get(w1)?;
    let after = steps.get(w1.saturating_add(1)..).unwrap_or(&[]);
    let mut unwall: usize = 0;
    for s in after {
        match s.role {
            WallRole::Guard => unwall = unwall.saturating_add(1),
            WallRole::Opaque | WallRole::Honest => break,
            WallRole::Transparent => {}
        }
    }
    let more_walls = after.iter().filter(|s| s.role == WallRole::Opaque).count();
    Some(FirstWallHint {
        leaf: wall.leaf,
        line: wall.line,
        word: wall.word.clone(),
        unwall,
        more_walls,
    })
}

/// Reduce each plan step to its [`WallStep`] (role + source line + command word) — the input
/// [`first_wall_hint`] consumes. The role classification is the load-bearing bit:
/// * a `Guard` disposition ⇒ [`WallRole::Guard`];
/// * a `Run` that is a KILL (`kills`) or a modeled establish ([`is_establish_bearing`]) ⇒
///   [`WallRole::Honest`] (a running mutator — it bounds the count);
/// * a `Run` that is probe-UNRESOLVABLE and a real command (not [`is_structurally_unprobeable`]) ⇒
///   [`WallRole::Opaque`] (the unmodeled poison wall — the same set the firehose discloses);
/// * everything else (elide / omit / inert builtin run) ⇒ [`WallRole::Transparent`].
///
/// The kill check PRECEDES the opaque check so a modeled kill is never mistaken for an unmodeled
/// wall. `by_ast` maps a step's `AstId` back to its `(CfgNodeId, SkipClass)` (steps ⊆ classified
/// leaves by construction; an unexpected miss degrades to `Transparent`, the safe non-claim).
#[must_use]
pub fn collect_wall_steps(
    plan: &dorc_plan::Plan,
    probe: &dorc_plan::ProbePlan,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    cfg: &dorc_analysis::cfg::Cfg,
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
) -> Vec<WallStep> {
    let by_ast: BTreeMap<
        dorc_core::AstId,
        (
            dorc_analysis::cfg::CfgNodeId,
            &dorc_analysis::effect::SkipClass,
        ),
    > = classes
        .iter()
        .map(|(node, class)| (cfg.node(*node).ast, (*node, class)))
        .collect();
    plan.steps
        .iter()
        .map(|step| {
            let span = ast.node(step.ast).span;
            let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
            let line = dorc_aid::diag::line_col(book_src, lo).0;
            let text = book_src.get(lo..hi).unwrap_or("");
            let word = text.split_whitespace().next().unwrap_or("").to_owned();
            let role = match &step.disposition {
                dorc_plan::Disposition::Guard(_) => WallRole::Guard,
                dorc_plan::Disposition::Replace(..) | dorc_plan::Disposition::Omit { .. } => {
                    WallRole::Transparent
                }
                dorc_plan::Disposition::Run => {
                    let cls = by_ast.get(&step.ast);
                    if cls.is_some_and(|(node, _)| kills.contains(node)) {
                        WallRole::Honest
                    } else if probe.unresolvable.contains(&step.leaf)
                        && !is_structurally_unprobeable(text)
                    {
                        WallRole::Opaque
                    } else if cls.is_some_and(|(_, class)| is_establish_bearing(class)) {
                        WallRole::Honest
                    } else {
                        WallRole::Transparent
                    }
                }
            };
            WallStep {
                leaf: step.leaf,
                line,
                word,
                role,
            }
        })
        .collect()
}

/// Mirror of the plan crate's private `class_is_establish_bearing` (a running establish is a
/// mutator wall). Re-derived here rather than exported: a small, stable predicate, and the cli edge
/// already reaches into `SkipClass` variants for other readouts. Kept in step by the shared slug.
fn is_establish_bearing(class: &dorc_analysis::effect::SkipClass) -> bool {
    use dorc_analysis::effect::SkipClass as Sc;
    match class {
        Sc::EstablishAmbient(_) | Sc::EstablishWritten(_) | Sc::EstablishMembers { .. } => true,
        Sc::InlineCall { sites } => sites.iter().any(|s| {
            matches!(
                s.class,
                Sc::EstablishAmbient(_) | Sc::EstablishWritten(_) | Sc::EstablishMembers { .. }
            )
        }),
        Sc::QueryResolvable { .. } | Sc::MustRun => false,
    }
}

/// The ONE seat that renders a [`SpeechAct`] to a word (`law-trust-tier-is-syntax`;
/// `27V:mech-trust-tier-typed`): the chain walker below is the ONLY code that turns a typed tier into
/// prose, so a `claims` link can never wear a `reported`'s clothes (mis-attribution is the worst aid
/// failure — `271:rul-sin-ordering`).
///
/// The words are arrangement-registry rows keyed by the tier's ordinal, never literals (`28G` §0):
/// the tier SET is the law, the words ride `27V:rul-output-form-unwelded`. `28E` §8 fixes the
/// grammar they must obey — the tier word is the sentence's VERB, past tense for run events
/// (`reported`, `ran`) and present for standing text (`vouches`, `claims`, `derives`).
fn verb_said(tier: SpeechAct) -> Said {
    let occurrence = match tier {
        SpeechAct::Measured => 0,
        SpeechAct::Vouched => 1,
        SpeechAct::Ran => 2,
        SpeechAct::Claimed => 3,
        SpeechAct::Derived => 4,
        SpeechAct::Consented => 5,
        SpeechAct::Declined => 6,
    };
    Said::words_at("why-tier-word", Some(occurrence), &[])
}

/// What happened to a line, in the ADMIN's terms rather than the engine's — the typed twin of
/// [`outcome_word`], so counting and comparing never go through rendered prose.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OutcomeKind {
    Skipped,
    Guarded,
    Ran,
    Dropped,
}

impl OutcomeKind {
    /// The disposition, re-read in admin terms. The engine's own vocabulary stops here.
    fn of(disposition: &dorc_plan::Disposition) -> Self {
        match disposition {
            dorc_plan::Disposition::Replace(..) => OutcomeKind::Skipped,
            dorc_plan::Disposition::Guard(_) => OutcomeKind::Guarded,
            dorc_plan::Disposition::Run => OutcomeKind::Ran,
            dorc_plan::Disposition::Omit { .. } => OutcomeKind::Dropped,
        }
    }

    /// The other thing that could have happened to the line — what the contrastive OUTCOME sentence
    /// answers against (`28E` §7 adopt-contrastive-first: the foil is the line's other disposition,
    /// and it is free).
    const fn foil(self) -> Self {
        match self {
            OutcomeKind::Skipped | OutcomeKind::Ran => OutcomeKind::Guarded,
            OutcomeKind::Guarded | OutcomeKind::Dropped => OutcomeKind::Skipped,
        }
    }

    /// The admin-English word (`28E` §8, human-demonstrated). Registry-homed by ordinal like
    /// [`verb_word`]. The `skip`-ban is LLM-facing law over design and code layers
    /// (`271:rul-skip-ban-is-llm-facing`); this is the deliberate user-surface carve, and engine
    /// vocabulary (elide / replace / omit) never appears in a render.
    fn word(self, ctx: &RenderCtx<'_>) -> String {
        let occurrence = match self {
            OutcomeKind::Skipped => 0,
            OutcomeKind::Guarded => 1,
            OutcomeKind::Ran => 2,
            OutcomeKind::Dropped => 3,
        };
        Said::words_at("why-outcome-word", Some(occurrence), &[]).text(ctx)
    }
}

/// The admin-English disposition word for a plan step.
fn outcome_word(ctx: &RenderCtx<'_>, disposition: &dorc_plan::Disposition) -> String {
    OutcomeKind::of(disposition).word(ctx)
}

/// The word for a disposition's FOIL — a skip's is the guard it would otherwise have worn, a
/// guard's is the skip it could not earn, a run's is a guard.
fn foil_word(ctx: &RenderCtx<'_>, disposition: &dorc_plan::Disposition) -> String {
    OutcomeKind::of(disposition).foil().word(ctx)
}

/// The gutter glyph a chain row wears in the DEFAULT render (`28E` §7
/// adapt-two-rank-default-render, sharpened by §8 `rul-danger-axis-is-completion-class`). ASCII
/// forever (`28E` §0 `rul-ascii-output-forever`, human-typed). The rank itself is the ordered
/// [`Knowability`] projection over the seven [`SpeechAct`] kinds, minted at the ONE derivation seat
/// (`SpeechAct::knowability`, `28F:rul-speechact-rename`) — this function only picks the glyph a
/// `Knowability` already decided, never re-derives one.
const fn rank_glyph(rank: Knowability) -> &'static str {
    match rank {
        Knowability::Witnessed => "*",
        Knowability::CoversUnmeasured => "!",
    }
}

/// The label a NEXT STEPS row wears. Registry-homed by ordinal like [`verb_word`] — the label SET
/// is the structure, the words ride `27V:rul-output-form-unwelded`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum StepLabel {
    Suspect,
    Fix,
    Verify,
    Review,
    /// The improvements-not-repairs step: what to DESCRIBE so a guarded line can eventually skip
    /// (`28G` strawmen `b-wide-guarded` and `d-guard-fell-through` — a healthy guard has no repair,
    /// so its arc is a different verb).
    Describe,
}

impl StepLabel {
    fn said(self) -> Said {
        let occurrence = match self {
            StepLabel::Suspect => 0,
            StepLabel::Fix => 1,
            StepLabel::Verify => 2,
            StepLabel::Review => 3,
            StepLabel::Describe => 4,
        };
        Said::words_at("why-next-step-label", Some(occurrence), &[])
    }
}

/// One row of the remediation arc.
struct StepRow {
    label: StepLabel,
    body: Said,
    /// Whether this row is an ALTERNATIVE to the one before it — mutually-exclusive repairs the
    /// reader picks between, rather than a further step. A run of them renders as one join.
    alternative: bool,
}

/// The NEXT STEPS panel of the triptych (`28E` §8: the human's markup grew this from a two-line
/// epilogue into a labeled STRUCTURAL remediation arc — `lean-prose-down-one-step`, mechanical
/// explanation over flowing paragraphs). The panel is OMITTED entirely when it has no rows, which
/// is the triptych-collapse `28G` strawman `e-skipped-quiet` demonstrates.
struct NextSteps {
    /// The line that frames the rows. A suspected-wrong skip opens on the reader's doubt; a
    /// deliberate decline opens by saying there is nothing to repair (`28G` strawman
    /// `c-declined-unsound`), and reusing one opener for both would ask a reader to fix a
    /// correctly-behaving line.
    opener: Said,
    rows: Vec<StepRow>,
}

/// What a survived elision spent to survive: the `N|command` references of every wall it was kept
/// past, and the provider whose at-most claim licensed that.
///
/// Only a SURVIVAL carries one. A guard spends nothing — it re-asks the question live — and a
/// declined line was never skipped at all, so neither belongs in the aggregate's TRUST SPENT panel
/// ("skips resting on a human claim, not proof"). Before this was optional, every chain reached
/// that panel and the two that had spent nothing rendered a claimant-less possessive over an empty
/// wall list, telling the reader their guarded and their declined line had been skipped on
/// somebody's word.
struct TrustSpent {
    crossed: String,
    claimant: String,
}

/// A `dorc why <addr>` triptych (`28G` Phase W1): the contrastive OUTCOME, the quoted-speakers
/// ANALYSIS, and the structural NEXT STEPS. Content + structure are the law; wording and arrangement
/// ride `27V:rul-output-form-unwelded` — transcripts re-bless freely on churn here.
struct ChainRender {
    /// What this line's skip SPENT, when it spent anything at all — see [`TrustSpent`].
    trust: Option<TrustSpent>,
    outcome: Said,
    /// The because-clause the OUTCOME sentence ends on, rendered as its own line beneath it
    /// (`28L:rul-composed-saids-render-as-own-lines`). It is a registry line in its own right —
    /// several of them are — so it renders as one, rather than being flattened into the outcome
    /// row's value where nothing downstream could name the entry an edit has to rewrite.
    because: Option<Said>,
    /// The ANALYSIS opener, then the speaker rows, then the numberless join restatement
    /// (`28E` §7 adapt-join-only-numbering: a linear chain carries no numbering at all). The rows
    /// and the restatement are the [`ChainModel`]: what the walk derived, and what this render
    /// selected out of it.
    analysis_opener: Said,
    chain: ChainModel,
    /// Every book line this answer names, in source order — the participating-lines block
    /// (`28E` §8 presence-complete, density-selected).
    ///
    /// PRESENCE is the invariant: a participant the ANALYSIS mentions and this list omits would be
    /// a false provenance claim, so the block is complete over the closure it declares and the
    /// panels below select only how MUCH each one gets. The closure is the answer's own references
    /// — the asked line plus every wall and crossing the chain names. It is NOT the value closure:
    /// no reaching-definitions query is exposed, so a `PORT=443` feeding the asked line's argv
    /// (`28G` strawman `b-wide-guarded` line 29) does not appear, which is exactly why the block
    /// has to say which closure it is complete over rather than saying "participating lines" flat.
    participants: Vec<usize>,
    /// The guard dorc shipped in place of a skip, as sh — the answer to "so what DID it do"
    /// (`28G` strawman `b-wide-guarded`). Not our bytes: the oracle author wrote the check and the
    /// admin wrote the command it fronts.
    shipped: Option<String>,
    next_steps: NextSteps,
}

/// The engine's own name in the speaker column — the only row dorc speaks in its own voice, and it
/// speaks only about its own derivations (`28E` §8 quoted-speakers).
const ENGINE_SPEAKER: &str = "dorc";

/// Build the survived-elision triptych (`28G` Phase W1 over the `27V` §4 flagship). Pure over the
/// plan's [`dorc_plan::SurvivalWitness`] + display context. `None` when the step survived no wall (an
/// ordinary elision has no chain to walk).
///
/// The row set is `28G` strawman `a-fire-morning`'s exactly: the probe's REPORT, the site oracle's
/// standing VOUCH, each crossed wall's at-most CLAIM, and dorc's own disjointness DERIVATION. Two
/// links the as-built chain carried as rows are stated in the contrastive OUTCOME instead — the
/// wall's run and the admin's consent — because neither has a speaker to quote, and OUTCOME puts the
/// consent AHEAD of the chain rather than last in it.
///
/// The `suspect:` row's claim of UNIQUENESS is a model fact — a count of covers-unmeasured rows —
/// never one fragment knowing what another rendered (`28E` lean-start-without-mutual-awareness).
#[expect(
    clippy::too_many_arguments,
    reason = "the chain builder threads the display context it quotes (reference/address/disposition/license/wall-map/interner/oracle paths+sources); each is a distinct pipeline output, not a bundle-able struct"
)]
#[expect(
    clippy::too_many_lines,
    reason = "one linear row-by-row chain construction followed by its NEXT STEPS rows; splitting it would scatter the ONE place the strawman's row set is expressed"
)]
fn survival_chain(
    ctx: &RenderCtx<'_>,
    reference: &str,
    address: &str,
    disposition: &dorc_plan::Disposition,
    license: &dorc_plan::ReplaceLicense,
    walls: &BTreeMap<dorc_plan::LeafId, String>,
    interner: &Interner,
    source_paths: &[String],
    source_srcs: &[String],
) -> Option<ChainRender> {
    let witness = license.derivation().survival.as_ref()?;
    let backing = render_coord(witness.backing(), interner);
    let outcome = outcome_word(ctx, disposition);
    let reported = license.derivation().probe.and_then(|p| p.reported);
    let report = ChainLink {
        tier: SpeechAct::Measured,
        speaker: reported_speaker(reference, reported, source_paths, source_srcs),
        payload: Said::Value(dorc_plan::fact_label(interner, license.fact())),
        quoted: true,
        event: reported.map(reported_event),
        explanation: None,
        excerpt: None,
    };
    let vouches: Vec<ChainLink> = if license.derivation().establish_vouches.is_empty() {
        vec![ChainLink {
            tier: SpeechAct::Vouched,
            speaker: oracle_locus(license.derivation().vouch_span, source_paths, source_srcs),
            payload: Said::words("why-vouch-payload-site", &[&backing]),
            quoted: true,
            event: None,
            explanation: None,
            excerpt: None,
        }]
    } else {
        let mut by_speaker: Vec<(Option<String>, Vec<String>)> = Vec::new();
        for receipt in &license.derivation().establish_vouches {
            let speaker = oracle_locus(receipt.defining_span, source_paths, source_srcs);
            let label = dorc_plan::fact_label(interner, receipt.fact);
            match by_speaker.iter_mut().find(|(who, _)| *who == speaker) {
                Some((_, labels)) => labels.push(label),
                None => by_speaker.push((speaker, vec![label])),
            }
        }
        by_speaker
            .into_iter()
            .map(|(speaker, labels)| ChainLink {
                tier: SpeechAct::Vouched,
                speaker,
                payload: Said::words("why-vouch-payload-establish", &[&brace_selectors(&labels)]),
                quoted: true,
                event: None,
                explanation: None,
                excerpt: None,
            })
            .collect()
    };
    let mut wall_refs: Vec<String> = Vec::new();
    let mut claimants: Vec<String> = Vec::new();
    let mut leverage: Option<String> = None;
    let mut claims: Vec<ChainLink> = Vec::new();
    for c in witness.crossings() {
        let provider = interner.resolve(c.provider()).to_owned();
        claimants.push(provider.clone());
        wall_refs.push(
            walls
                .get(&c.wall_leaf())
                .cloned()
                .unwrap_or_else(|| provider.clone()),
        );
        let coords: Vec<String> = c
            .footprint()
            .iter()
            .map(|fc| render_coord(*fc, interner))
            .collect();
        let locus = oracle_locus(c.footprint_span(), source_paths, source_srcs);
        leverage = leverage.or_else(|| locus.clone());
        claims.push(ChainLink {
            tier: SpeechAct::Claimed,
            speaker: locus,
            payload: Said::words("why-claims-payload", &[&provider, &coords.join(" ")]),
            quoted: true,
            event: None,
            explanation: Some(Said::words("why-claims-covers-unmeasured", &[])),
            excerpt: oracle_excerpt(c.footprint_span(), source_paths, source_srcs),
        });
    }
    let derivation = ChainLink {
        tier: SpeechAct::Derived,
        speaker: Some(ENGINE_SPEAKER.to_owned()),
        payload: Said::words("why-derives-payload-disjoint", &[&backing]),
        quoted: false,
        event: None,
        explanation: None,
        excerpt: None,
    };
    let links = survival_row_order(report, vouches, claims, derivation);

    let joined_walls = wall_refs.join(", ");
    let unmeasured = links
        .iter()
        .filter(|l| l.tier.knowability() == Knowability::CoversUnmeasured)
        .count();
    let mut rows = vec![StepRow {
        label: StepLabel::Suspect,
        body: if unmeasured == 1 {
            Said::words(
                "why-next-step-suspect-sole-claim",
                &[&joined_walls, &backing],
            )
        } else {
            Said::words(
                "why-next-step-suspect-several-claims",
                &[&unmeasured.to_string(), &backing],
            )
        },
        alternative: false,
    }];
    if let Some(lev) = &leverage {
        rows.push(StepRow {
            label: StepLabel::Fix,
            body: Said::words("why-next-step-fix-widen", &[lev]),
            alternative: false,
        });
    }
    rows.push(StepRow {
        label: StepLabel::Fix,
        body: Said::words("why-next-step-fix-replan", &[CONSENT_FLAG]),
        alternative: leverage.is_some(),
    });
    rows.push(StepRow {
        label: StepLabel::Verify,
        body: Said::words("why-next-step-verify", &[]),
        alternative: false,
    });
    rows.push(StepRow {
        label: StepLabel::Review,
        body: Said::words("why-next-step-review", &[address]),
        alternative: false,
    });
    Some(ChainRender {
        trust: Some(TrustSpent {
            crossed: joined_walls.clone(),
            claimant: claimants.join(", "),
        }),
        outcome: contrastive(reference, &outcome, &foil_word(ctx, disposition)),
        because: Some(Said::words(
            "why-outcome-because-survived",
            &[&joined_walls, CONSENT_FLAG],
        )),
        analysis_opener: Said::words("why-analysis-opener", &[reference, &outcome]),
        chain: ChainModel::all_selected(
            links,
            Some(Said::words("why-analysis-join", &[&joined_walls, &backing])),
        ),
        participants: Vec::new(),
        shipped: None,
        next_steps: NextSteps {
            opener: Said::words("why-next-steps-opener", &[reference]),
            rows,
        },
    })
}

/// The DEFAULT order of a survival chain's ANALYSIS rows: what was measured, who vouched, whose
/// at-most claim was spent, and what dorc derived from all of it.
///
/// This is a RENDER DEFAULT and nothing more (`28E:lean-ordering-is-a-seam`). The order carries no
/// semantics — it is not causal, not temporal, and not a ranking — so it lives in ONE named seat
/// rather than implicitly in the sequence of `push` calls, which is what a later
/// distrust-ordered-or-otherwise-arranged render would otherwise have to reverse-engineer out of
/// straight-line code. No ordering machinery: the seat is the affordance.
fn survival_row_order(
    report: ChainLink,
    vouches: Vec<ChainLink>,
    claims: Vec<ChainLink>,
    derivation: ChainLink,
) -> Vec<ChainLink> {
    let mut links = vec![report];
    links.extend(vouches);
    links.extend(claims);
    links.push(derivation);
    links
}

/// Build the HEALTHY-GUARD triptych (`28G` strawmen `b-wide-guarded` and `d-guard-fell-through`):
/// what dorc knew, why it was not enough to skip, and what it shipped instead.
///
/// The wall rows are the point (`289:fnd-guarded-chain-omits-the-wall`). A guarded line's whole
/// story is that a good report went stale — and until now the chain named the report and the vouch
/// but never the thing that came between them, leaving the reader with two links that ought to have
/// been sufficient and no account of why they were not.
#[expect(
    clippy::too_many_arguments,
    reason = "the guard chain quotes the same display context the survival chain does, plus the wall walk it names its walls from; each is a distinct pipeline output"
)]
fn guard_chain(
    ctx: &RenderCtx<'_>,
    reference: &str,
    address: &str,
    original: &str,
    license: &dorc_plan::GuardLicense,
    walls_above: &[&WallStep],
    interner: &Interner,
    source_paths: &[String],
    source_srcs: &[String],
) -> ChainRender {
    let backing = dorc_plan::fact_label(interner, license.fact());
    let reported = license.reported();
    let mut links = vec![
        ChainLink {
            tier: SpeechAct::Measured,
            speaker: reported_speaker(reference, reported, source_paths, source_srcs),
            payload: Said::Value(backing.clone()),
            quoted: true,
            event: reported.map(reported_event),
            explanation: None,
            excerpt: None,
        },
        ChainLink {
            tier: SpeechAct::Vouched,
            speaker: oracle_locus(license.insert().defining_span(), source_paths, source_srcs),
            payload: Said::words("why-vouch-payload-site", &[&backing]),
            quoted: true,
            event: None,
            explanation: None,
            excerpt: None,
        },
    ];
    links.extend(walls_above.iter().map(|wall| ChainLink {
        tier: SpeechAct::Ran,
        speaker: Some(format!("{}|{}", wall.line, wall.word)),
        payload: Said::words_at("why-wall-payload", Some(wall.role.occurrence()), &[]),
        quoted: false,
        event: None,
        explanation: None,
        excerpt: None,
    }));
    let wall_refs: Vec<String> = walls_above
        .iter()
        .map(|wall| format!("{}|{}", wall.line, wall.word))
        .collect();
    let joined_walls = wall_refs.join(", ");
    // ANALYSIS names every wall; `describe:` only the UNDESCRIBED ones, else the nag lands wrong.
    let describable: Vec<String> = walls_above
        .iter()
        .filter(|wall| wall.role == WallRole::Opaque)
        .map(|wall| format!("{}|{}", wall.line, wall.word))
        .collect();
    let rows = if describable.is_empty() {
        Vec::new()
    } else {
        vec![
            StepRow {
                label: StepLabel::Describe,
                body: Said::words_at(
                    "why-next-step-describe-walls",
                    Some(usize::from(describable.len() > 1)),
                    &[&describable.join(", ")],
                ),
                alternative: false,
            },
            StepRow {
                label: StepLabel::Review,
                body: Said::words("why-next-step-review", &[address]),
                alternative: false,
            },
        ]
    };
    ChainRender {
        trust: None,
        outcome: contrastive(
            reference,
            &OutcomeKind::Guarded.word(ctx),
            &OutcomeKind::Guarded.foil().word(ctx),
        ),
        because: Some(Said::words("why-outcome-because-guarded", &[&joined_walls])),
        analysis_opener: Said::words("why-analysis-opener-guarded", &[]),
        chain: ChainModel::all_selected(
            links,
            Some(Said::words("why-analysis-join-guarded", &[reference])),
        ),
        participants: Vec::new(),
        shipped: Some(license.insert().display_line(original)),
        next_steps: NextSteps {
            opener: Said::words("why-next-steps-opener-guarded", &[]),
            rows,
        },
    }
}

/// The narrative classes this run MINTED and no render CONSUMED, as greppable
/// `[unnarrated: <class>]` lines (`28E:prop-unnarrated-is-visible`).
///
/// The aid plane fails toward narration (`two-plane-aid-law`), and a class that mints without ever
/// being rendered fails toward SILENCE instead — the standing
/// `289:seam-narrative-render-unconsumed`. Deepest pull tier only: this is a maintainer's disclosure
/// about the surface's own coverage, and putting it on the default surface would spend the
/// firefighter's attention on dorc's gaps rather than on their host.
///
/// `narratable` carries the version coupling. On a replay it is false when the durable's record
/// stream and this binary's narrative plane disagree, because the census would then be a confident
/// claim about a run whose class set this binary never held.
fn unnarrated_lines(narrative: &[CollapseNarrative], narratable: bool) -> Vec<String> {
    if !narratable {
        return Vec::new();
    }
    let mut classes: Vec<&'static str> = Vec::new();
    for record in narrative {
        let rendered = matches!(
            record.kind(),
            CollapseKind::VerdictDecline {
                authored_reason: Some(_),
                ..
            }
        );
        let class = record.class_name();
        if !rendered && !classes.contains(&class) {
            classes.push(class);
        }
    }
    classes.sort_unstable();
    classes
        .into_iter()
        .map(|class| format!("[unnarrated: {class}]"))
        .collect()
}

/// Collapse coordinate labels sharing one `kind:entity` into the brace-alternation display
/// `kind:entity@{a,b}` (`28G` strawman `a-fire-morning` line 72; `281` §7's own spelling for a
/// multi-cell coordinate, reused rather than a second one invented).
///
/// DISPLAY only, and deliberately so: the model still holds N separate [`dorc_core::FactKey`]s,
/// each with its own selector, and every comparison the engine does still runs per-cell through
/// the `selector_covers` chokepoint (`selector-chokepoint`). Folding the model would make two cells
/// look like one to the algebra, which is exactly the collision the chokepoint exists to prevent.
/// Order is preserved and duplicates are kept out; a label with no `@` passes through untouched.
fn brace_selectors(labels: &[String]) -> String {
    let mut grouped: Vec<(String, Vec<String>)> = Vec::new();
    for label in labels {
        let (head, selector) = match label.rsplit_once('@') {
            Some((head, selector)) => (head.to_owned(), Some(selector.to_owned())),
            None => (label.clone(), None),
        };
        if !grouped.iter().any(|(seen, _)| *seen == head) {
            grouped.push((head.clone(), Vec::new()));
        }
        if let Some(entry) = grouped.iter_mut().find(|(seen, _)| *seen == head)
            && let Some(selector) = selector
            && !entry.1.contains(&selector)
        {
            entry.1.push(selector);
        }
    }
    grouped
        .into_iter()
        .map(|(head, selectors)| match selectors.len() {
            0 => head,
            1 => format!("{head}@{}", selectors.join("")),
            _ => format!("{head}@{{{}}}", selectors.join(",")),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The asked line plus every line the answer names, deduped and in source order.
///
/// Source order, not chain order: the block is a reading aid over the book, and the reader's eye
/// expects the file's own sequence. Chain ORDER stays the ANALYSIS panel's, where it means
/// something (`28E` lean-ordering-is-a-seam).
fn participants(asked: usize, named: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut lines: BTreeSet<usize> = named.collect();
    lines.insert(asked);
    lines.into_iter().collect()
}

/// The participating-lines block that opens an addressed answer (`28E` §8 presence-complete,
/// density-selected; `28G` strawman `a-fire-morning` lines 57–59).
///
/// The qualification row beneath it is not decoration. "Participating lines" read alone, at 03:40,
/// becomes "nothing else was involved" — a claim about the WORLD rather than about a closure, and
/// exactly the negative `28E:rul-never-a-dinna-do-it-layer` forbids Dorc from ever synthesizing.
/// The block therefore states which closure it is complete over.
fn participating_block(
    ctx: &RenderCtx<'_>,
    lines: &[usize],
    filename: &str,
    book_src: &str,
) -> Vec<Node<Face>> {
    let source: Vec<&str> = book_src.lines().collect();
    let rows: Vec<CodeLine<Face>> = lines
        .iter()
        .filter_map(|number| {
            let text = source.get(number.saturating_sub(1))?;
            Some(CodeLine {
                gutter: Some(dorc_aid::weave::value(
                    number.to_string(),
                    "why-participating-lines-gutter",
                    WHY_VALUE_CAP,
                )),
                cells: vec![CodeCell::new(vec![dorc_aid::weave::foreign(
                    &dorc_aid::ForeignBytes::from_io_edge(text.trim_end()),
                    filename.to_owned(),
                    WHY_SOURCE_CAP,
                )])],
            })
        })
        .collect();
    if rows.is_empty() {
        return Vec::new();
    }
    vec![
        Node::new(NodeKind::Code(CodeBlock {
            table: Some(Face::Table(format!("participating:{filename}"))),
            mode: Literalness::Literal,
            locus: Some(
                Said::words("why-participating-lines-locus", &[filename])
                    .runs(ctx, "why-participating-lines"),
            ),
            lines: rows,
        })),
        registry_paragraph(ctx, "why-participating-lines-closure"),
    ]
}

/// The sites whose author DELIBERATELY declined, keyed by leaf — the pull surface's index into the
/// `VerdictDecline` narratives (`collapse-mints-narrative`).
///
/// Only a decline carrying an `authored_reason` counts: an unclassed decline is ordinary
/// control-flow (`rul-vouch-is-verdict-authoring` — no vouch ⇒ run) and says nothing a reader could
/// act on, while a CLASSED one is the author stating why. Reading a narrative for display is the
/// one direction the two planes allow (`two-plane-aid-law`); nothing here reaches a license.
fn authored_declines(
    narrative: &[CollapseNarrative],
) -> BTreeMap<dorc_plan::LeafId, dorc_aid::narrative::AuthoredReason> {
    let mut out = BTreeMap::new();
    for record in narrative {
        if let CollapseKind::VerdictDecline {
            site,
            authored_reason: Some(reason),
            ..
        } = record.kind()
        {
            out.entry(site.leaf).or_insert(*reason);
        }
    }
    out
}

/// Build the AUTHORED-REFUSAL triptych (`28G` strawman `c-declined-unsound`): the answer for a
/// line that runs because the person who knows the tool ruled the question unanswerable.
///
/// This is the pull-surface half of `27W`'s decline design, which until now existed only as a
/// stderr push line (`289:fnd-decline-class-is-push-only`): asking `dorc why` about a declined site
/// showed the generic ran-blind answer, which reads as a GAP in dorc's knowledge when it is the
/// opposite — a place the knowledge exists and says no.
///
/// The class drives everything class-specific through occurrence-keyed rows, so an `unmodeled`
/// decline (the author's "not yet") never wears an `unsound` decline's words (the author's "not
/// ever, by anyone").
fn decline_chain(
    ctx: &RenderCtx<'_>,
    reference: &str,
    address: &str,
    word: &str,
    reason: &dorc_aid::narrative::AuthoredReason,
    source_paths: &[String],
    source_srcs: &[String],
) -> ChainRender {
    let class = reason.class;
    let occurrence = Some(class.occurrence());
    let arm = Some((reason.arm.0, reason.arm_file));
    let class_said = |slug: &'static str| Said::words_at(slug, occurrence, &[]);
    let links = vec![
        ChainLink {
            tier: SpeechAct::Declined,
            speaker: oracle_locus(arm, source_paths, source_srcs),
            payload: Said::words("why-declines-payload", &[class.token()]),
            quoted: true,
            event: None,
            explanation: Some(class_said("why-declines-explanation")),
            excerpt: oracle_excerpt(arm, source_paths, source_srcs),
        },
        ChainLink {
            tier: SpeechAct::Derived,
            speaker: Some(ENGINE_SPEAKER.to_owned()),
            payload: Said::words("why-declines-derives-cannot-say-runs", &[]),
            quoted: false,
            event: None,
            explanation: None,
            excerpt: None,
        },
    ];
    ChainRender {
        trust: None,
        outcome: contrastive(
            reference,
            &OutcomeKind::Ran.word(ctx),
            &OutcomeKind::Ran.foil().word(ctx),
        ),
        because: Some(Said::words("why-outcome-because-declined", &[word])),
        analysis_opener: Said::words("why-analysis-opener-plain", &[reference]),
        chain: ChainModel::all_selected(links, Some(class_said("why-declines-join"))),
        participants: Vec::new(),
        shipped: None,
        next_steps: NextSteps {
            opener: class_said("why-declines-next-steps-opener"),
            rows: vec![StepRow {
                label: StepLabel::Review,
                body: Said::words("why-next-step-review", &[address]),
                alternative: false,
            }],
        },
    }
}

/// The speaker of a `reported` row: the oracle line whose body produced the report
/// (`service.oracle.sh:12`, the strawmen's shape), from the reporting record's own threaded
/// predict-defining span.
///
/// Falls back to the shipped funcname when the span is honestly absent — an entry-composed or
/// connected-pipe body has no single defining funcdef, and no record reported at all when the
/// license was minted without a probe-attribution map. Naming a file we did not derive would be a
/// mis-attributed speaker, the worst class of aid failure (`271:rul-sin-ordering`).
fn reported_speaker(
    reference: &str,
    reported: Option<dorc_plan::ReportedObservation>,
    source_paths: &[String],
    source_srcs: &[String],
) -> Option<String> {
    reported
        .and_then(|r| oracle_locus(r.predict_span, source_paths, source_srcs))
        .or_else(|| Some(predict_speaker(reference)))
}

/// The `reported` row's payload trailer: WHEN the controller took the report in, and what the
/// probe command exited with (`28G` strawman `a-fire-morning`'s `(ran 01:59:52, rc 0)` slot).
///
/// The instant is CONTROLLER-minted (`28F:rul-probe-instants-host-says-no-times`, human-typed: the
/// host says no times, ever), and the moment it names is the one this edge actually holds — when
/// the record was received, not when the check ran on the host. The word says so; a `ran` here
/// would date a host event we were never told about. `None` for the instant ⇒ the rc alone, never
/// a fabricated moment.
fn reported_event(reported: dorc_plan::ReportedObservation) -> Said {
    let rc = reported.tool_rc.0.to_string();
    match reported.stamp.received_at {
        Some(at) => Said::words(
            "why-chain-event-received",
            &[&dorc_aid::instant::time_text(at), &rc],
        ),
        None => Said::words("why-chain-event-rc-only", &[&rc]),
    }
}

/// The speaker of a `reported` row whose record carried no defining span: the funcname the probe
/// actually shipped and invoked (`<provider>__predict`) — exact, and claiming no file.
fn predict_speaker(reference: &str) -> String {
    let word = reference.split_once('|').map_or(reference, |(_, w)| w);
    format!(
        "{}__predict",
        dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(word))
    )
}

/// The indent the whole `dorc why <addr>` triptych sits at.
const TRIPTYCH_INSET: usize = 3;

/// The table every NEXT STEPS row joins. Naming one relates the alternatives buried inside the
/// repair join to the steps around them, which no structural rule can do — and the table then
/// hangs or stacks as a unit (`28F:rul-table-degrades-whole`).
const STEPS_TABLE: &str = "why-next-steps";

/// The blank line between two laid-out documents of one report — structure the report computed,
/// so it stamps as [`dorc_aid::tagged::RenderPart::Arrangement`] and closes the section before it.
const REPORT_GAP: &str = "why-report-gap";

/// Separate two documents with one blank line, IN the part stream.
///
/// It used to be a bare `println!()` between two prints, which meant the bytes a reader saw and
/// the bytes the stream carried were not the same bytes — the exact split
/// `28L:rul-editability-is-stamped-never-re-derived` forbids.
fn push_gap(parts: &mut RenderParts) {
    parts.push(dorc_aid::tagged::RenderPart::Arrangement {
        text: "\n".to_owned(),
        slug: REPORT_GAP,
    });
}

/// A one-sentence report: a registry row laid out as its own document, so the row wears its own
/// editable face instead of being flattened to text by a `println!`.
fn sentence_document(ctx: &RenderCtx<'_>, slug: &'static str, values: &[&str]) -> RenderParts {
    why_parts(vec![paragraph(ctx, &Said::words(slug, values), slug)], 0)
}

/// The OUTCOME panel's one contrastive sentence, from all four chain walkers.
///
/// The because-clause is NOT one of its values: it is its own registry line, rendered beneath this
/// one (`28L:rul-composed-saids-render-as-own-lines`). Nesting it here flattened a whole sentence
/// into an immutable value — the entry it came from could not be named, so it could not be
/// edited — and it also re-capped OUR OWN words at [`WHY_VALUE_CAP`], cutting a 281-byte reason at
/// 240 (`28H:ask-because-clause-truncates-at-two-forty`).
fn contrastive(reference: &str, outcome: &str, foil: &str) -> Said {
    Said::words("why-outcome-contrastive", &[reference, outcome, foil])
}

/// The OUTCOME panel's body: the contrastive sentence, with its because-clause hanging beneath it
/// as a line of its own (`28L:rul-composed-saids-render-as-own-lines`).
///
/// The banner is what gives the clause its indent WITHOUT giving it its own section-splitting
/// chrome: two registry lines, two sections, one per line — the transport law is
/// `a-chrome-line-is-one-section`, not one-section-per-panel.
fn outcome_node(ctx: &RenderCtx<'_>, chain: &ChainRender) -> Node<Face> {
    let Some(because) = &chain.because else {
        return paragraph(ctx, &chain.outcome, "why-outcome");
    };
    Node::new(NodeKind::Banner(Banner {
        headline: chain.outcome.runs(ctx, "why-outcome"),
        body: vec![paragraph(ctx, because, "why-outcome-because")],
    }))
}

/// A titled panel. The header WORDS come from the registry; weft mints the rule around them and
/// nothing else (`28F:rul-weft-geometry-vs-words`).
fn panel(ctx: &RenderCtx<'_>, header: &'static str, body: Vec<Node<Face>>) -> Node<Face> {
    Node::new(NodeKind::Section(Section {
        header: Said::words(header, &[]).runs(ctx, header),
        counts: None,
        body,
    }))
}

/// The ANALYSIS panel's quoted-speakers rows (`28E` §8): who spoke, the tier word as the sentence's
/// verb, and their own words quoted. The rows are adjacent siblings, so weft resolves them as one
/// table and every payload starts in one column — a `claims` row's covers-unmeasured paragraph and
/// its `as-written:` excerpt hang below the quote without breaking the run.
fn chain_rows(ctx: &RenderCtx<'_>, links: &[&ChainLink]) -> Vec<Node<Face>> {
    links
        .iter()
        .map(|link| {
            let mut attachments: Vec<Node<Face>> = link
                .explanation
                .iter()
                .map(|said| paragraph(ctx, said, "why-chain-explanation"))
                .collect();
            attachments.extend(
                link.excerpt
                    .iter()
                    .flat_map(|excerpt| excerpt_nodes(ctx, excerpt)),
            );
            Node::new(NodeKind::Speaker(SpeakerRow {
                table: None,
                gutter: Some(dorc_aid::weave::mark(
                    rank_glyph(link.tier.knowability()),
                    "why-rank-mark",
                )),
                speaker: link
                    .speaker
                    .iter()
                    .map(|who| dorc_aid::weave::value(who, "why-chain-row-speaker", WHY_VALUE_CAP))
                    .collect(),
                verb: Some(verb_said(link.tier).runs(ctx, "why-chain-row")),
                payload: Payload {
                    quoting: if link.quoted {
                        Quoting::Quoted
                    } else {
                        Quoting::Bare
                    },
                    runs: link.payload.runs(ctx, "why-chain-row"),
                    trailer: link
                        .event
                        .iter()
                        .flat_map(|event| event.runs(ctx, "why-chain-event"))
                        .collect(),
                },
                attachments,
            }))
        })
        .collect()
}

/// A speaker's source, inlined beneath their row (`28G` §0's foreign-text class; the strawman's
/// `as-written:` gutter). LITERAL mode: these bytes are byte-honest and never rewrapped, because a
/// break the source does not contain would be a lie about what the author wrote. A cut middle is
/// SHOWN, and the two halves name one table so their gutters stay in one column across it.
fn excerpt_nodes(ctx: &RenderCtx<'_>, excerpt: &Excerpt) -> Vec<Node<Face>> {
    let table = Some(Face::Table(format!("as-written:{}", excerpt.path)));
    let block = |lines: &[(usize, String)], locus: bool| {
        Node::new(NodeKind::Code(CodeBlock {
            table: table.clone(),
            mode: Literalness::Literal,
            locus: locus.then(|| {
                Said::words("why-as-written-locus", &[&excerpt.path]).runs(ctx, "why-as-written")
            }),
            lines: lines
                .iter()
                .map(|(number, text)| CodeLine {
                    gutter: Some(dorc_aid::weave::value(
                        number.to_string(),
                        "why-as-written-gutter",
                        WHY_VALUE_CAP,
                    )),
                    cells: vec![CodeCell::new(vec![dorc_aid::weave::foreign(
                        &dorc_aid::ForeignBytes::from_io_edge(text),
                        excerpt.path.clone(),
                        WHY_SOURCE_CAP,
                    )])],
                })
                .collect(),
        }))
    };
    let mut out = vec![block(&excerpt.head, true)];
    if excerpt.elided > 0 {
        out.push(Node::new(NodeKind::Truncation(Truncation {
            note: Said::words("why-as-written-elided", &[&excerpt.elided.to_string()])
                .runs(ctx, "why-as-written"),
        })));
        out.push(block(&excerpt.tail, false));
    }
    out
}

/// The guard as dorc shipped it, inlined beneath the ANALYSIS restatement.
///
/// Rides the same foreign-text class the `as-written:` excerpt does (`28G` §0), and for the same
/// reason: the check is the oracle author's invocation and the fallback is the admin's own line, so
/// nothing in the block is ours to rephrase. LITERAL mode — it is displayed sh, and a break the
/// shipped bytes do not contain would be a lie about what runs.
fn shipped_block(sh: &str) -> Node<Face> {
    Node::new(NodeKind::Code(CodeBlock {
        table: None,
        mode: Literalness::Literal,
        locus: None,
        lines: vec![CodeLine {
            gutter: None,
            cells: vec![CodeCell::new(vec![dorc_aid::weave::foreign(
                &dorc_aid::ForeignBytes::from_io_edge(sh),
                "the shipped guard",
                WHY_SOURCE_CAP,
            )])],
        }],
    }))
}

/// One row of the remediation arc.
fn step_row(ctx: &RenderCtx<'_>, row: &StepRow) -> Node<Face> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table: Some(Face::Table(STEPS_TABLE.to_owned())),
        label: row.label.said().runs(ctx, "why-next-step"),
        body: row.body.runs(ctx, "why-next-step"),
        attachments: Vec::new(),
    }))
}

/// The remediation arc. A row followed by ALTERNATIVES becomes one join under the consumer's own
/// connective, so the reader sees a choice rather than a to-do list; the shared table keeps the
/// branch rows squared up with the steps they sit between.
fn step_nodes(ctx: &RenderCtx<'_>, steps: &NextSteps) -> Vec<Node<Face>> {
    let mut out: Vec<Node<Face>> = Vec::new();
    let mut index = 0usize;
    while index < steps.rows.len() {
        let mut last = index;
        while steps
            .rows
            .get(last.saturating_add(1))
            .is_some_and(|next| next.alternative)
        {
            last = last.saturating_add(1);
        }
        if last == index {
            out.extend(steps.rows.get(index).map(|row| step_row(ctx, row)));
        } else {
            let branches = steps
                .rows
                .get(index..=last)
                .unwrap_or_default()
                .iter()
                .enumerate()
                .map(|(position, row)| Branch {
                    connective: (position > 0).then(|| {
                        Said::words("why-alternative-connective", &[]).runs(ctx, "why-next-step")
                    }),
                    nodes: vec![step_row(ctx, row)],
                })
                .collect();
            out.push(Node::new(NodeKind::Join(Join {
                branches,
                restatement: None,
            })));
        }
        index = last.saturating_add(1);
    }
    out
}

/// A [`ChainRender`] as the three panels of the `dorc why <addr>` triptych (`28G` Phase W1).
///
/// The contrastive OUTCOME, the quoted-speakers ANALYSIS closed by its numberless join
/// restatement, and the structural NEXT STEPS — which is OMITTED whole when the question has no
/// next step, the question-relative floor `28G` strawman `e-skipped-quiet` demonstrates.
///
/// `exhaustive` is the reader's `--all`: the default render shows the model's SELECTED links, and
/// `--all` shows every link the walk derived, which is what the printed pointer promises
/// (`ask-all-flag-promises-exhaustive`). Today every walker selects everything, so the two agree —
/// the flag is wired to the model rather than to a second render path so that stays true.
fn chain_nodes(ctx: &RenderCtx<'_>, chain: &ChainRender, exhaustive: bool) -> Vec<Node<Face>> {
    let mut out = vec![panel(
        ctx,
        "why-outcome-heading",
        vec![outcome_node(ctx, chain)],
    )];
    let links = chain.chain.rendered(exhaustive);
    if links.is_empty() {
        return out;
    }
    let mut analysis = vec![paragraph(
        ctx,
        &chain.analysis_opener,
        "why-analysis-opener",
    )];
    analysis.extend(chain_rows(ctx, &links));
    if links
        .iter()
        .any(|link| link.tier.knowability() == Knowability::CoversUnmeasured)
    {
        analysis.push(registry_paragraph(ctx, "why-mark-legend"));
    }
    analysis.extend(
        chain
            .chain
            .conclusion
            .iter()
            .map(|join| paragraph(ctx, join, "why-analysis-join")),
    );
    analysis.extend(chain.shipped.iter().map(|sh| shipped_block(sh)));
    out.push(panel(ctx, "why-analysis-heading", analysis));

    if !chain.next_steps.rows.is_empty() {
        let mut arc = vec![paragraph(
            ctx,
            &chain.next_steps.opener,
            "why-next-steps-opener",
        )];
        arc.extend(step_nodes(ctx, &chain.next_steps));
        out.push(panel(ctx, "why-next-steps-heading", arc));
    }
    out
}
/// Which aggregate section a site belongs to in the zero-argument `dorc why` (`28E` §8, the
/// human-demonstrated three-way split; the PROBLEMS section name is RETIRED — genuine breakage
/// surfaces as a SURPRISE, and everything else dorc could do better about is an IMPROVEMENT).
#[derive(Clone, Copy, PartialEq, Eq)]
enum AggregateClass {
    /// Nothing to say about this site in the aggregate.
    Quiet,
    /// The world disagreed with the plan. Leads the aggregate when no trust was spent.
    Surprise,
    /// dorc could do better here, if the reader described more of their world.
    Improvement,
}

/// One site's WHY-record ([`emit_why_report`]): its SOURCE line (`rul24-lineno-identity`), the
/// one-line command, its admin-English outcome, its ANALYSIS rows, and which aggregate section it
/// belongs to.
struct WhySite {
    line: usize,
    /// The command's first word — the `certsync` of an `8|certsync` inline reference.
    word: String,
    command: String,
    outcome: String,
    foil: String,
    reasons: Vec<Said>,
    class: AggregateClass,
    /// The improvement's one-line reason, when this site is an [`AggregateClass::Improvement`].
    improvement: Option<Said>,
}

impl WhySite {
    /// The `N|command` inline reference (`28E` §8, human-demonstrated row shape): short enough to
    /// sit inside a sentence, and unambiguous because the line number is the SOURCE file's.
    fn reference(&self) -> String {
        format!("{}|{}", self.line, self.word)
    }

    /// The file-qualified address this site answers to — the exact bytes `dorc why` accepts back
    /// (`28E` §7 held-placement-reread: a pointer line must be copy-paste-true).
    fn address(&self, filename: &str) -> String {
        format!("{filename}:{}", self.line)
    }
}

/// ack-2 `dorc why`: the source-line-keyed WHY report — the focused query surface (the `plan`
/// preview points here). **rul24-lineno-identity** (a product invariant): the ONE line-number
/// space is the SOURCE file's, so a `file:N` this report PRINTS is exactly the `book.sh:N` a query
/// ACCEPTS — the mapping is 1:1 through [`dorc_aid::diag::line_col`]. Three addressing forms:
/// * `None` (unargumented) — the CURRENT run's PROBLEMS: every site that runs on a ⊤, runs
///   unprobed, or carries a guard / render-refusal (never a clean elide/omit) — "can't be typing
///   lines manually when you're already annoyed" (NO cross-run state; kSTATE stays parked).
/// * a `book.sh:N` / bare `N` line-address — the site(s) on that source line.
/// * free content — the site(s) whose command text contains it.
///
/// An ADDRESSED site renders the `28G` triptych (OUTCOME / ANALYSIS / NEXT STEPS); the unargumented
/// form renders the aggregate (TRUST SPENT first and uncapped, then SURPRISES, then IMPROVEMENTS).
///
/// The seventeen inputs travel as ONE borrowed context rather than seventeen parameters. They are
/// all pure data by the time they get here — the binary spent every query at its own edge
/// (`io-at-edges-only`) — which is exactly what lets a loom driver assemble the same struct from a
/// materialized case and drive the REAL report (`lib-target-is-a-loom-seam`: values cross, queries
/// do not).
#[derive(Clone, Copy, Debug)]
pub struct WhyReport<'a> {
    /// The `dorc why <address>` positional, or `None` for the aggregate.
    pub address: Option<&'a str>,
    /// The plan whose dispositions this report explains.
    pub plan: &'a dorc_plan::Plan,
    /// The compiled probe, for the unresolvable-site set.
    pub probe: &'a dorc_plan::ProbePlan,
    /// The first unmodeled wall, when the book has one.
    pub first_wall: Option<&'a FirstWallHint>,
    /// Every plan step's wall role, in book order.
    pub wall_steps: &'a [WallStep],
    /// The ⊤-cause disclosures the why-lens renders.
    pub why_diags: &'a [Diag],
    /// The render refusals, which turn an elision into a SURPRISE.
    pub refusals: &'a [Diag],
    /// The provenance arena the ⊤-causes resolve in.
    pub arena: &'a ProvArena,
    /// The parsed book.
    pub ast: &'a dorc_syntax::ast::Ast,
    /// The book's bytes — every quoted line comes from here.
    pub book_src: &'a str,
    /// The book's display path; the file half of every address this report prints.
    pub filename: &'a str,
    /// The shared interner coordinates resolve through, for display only.
    pub interner: &'a Interner,
    /// Every loaded DEFINITION SOURCE's path, in load order — oracles then the book. Named
    /// `source_` rather than `oracle_` since `28K` §2a: a book is a first-class definition source
    /// (an in-book role function is an ordinary oracle, recognized by name alone), so a locus this
    /// report resolves can land in the book, and `oracle_paths` understated the vector's contents.
    /// The one filler that is still oracle-only discloses itself at its own seat (`world.rs`).
    pub source_paths: &'a [String],
    /// Those sources' bytes, positionally matched, for excerpts and loci.
    pub source_srcs: &'a [String],
    /// The run's collapse narratives — declines and the `--all` census.
    pub narrative: &'a [CollapseNarrative],
    /// Per-leaf cascade attribution from the validity fixpoint.
    pub cascades: &'a BTreeMap<dorc_plan::LeafId, CascadeAttribution>,
    /// The invocation record the aggregate opens with.
    pub receipt: &'a Receipt,
}
/// The whole `dorc why` report as ONE stamped part stream — the seat both the binary and a loom
/// driver render through, so a committed transcript is the bytes the binary prints.
#[expect(
    clippy::too_many_lines,
    reason = "one linear per-disposition reason-derivation + the three addressing branches; splitting it would scatter the ONE report shape"
)]
pub fn why_report_parts(ctx: &RenderCtx<'_>, report: &WhyReport<'_>) -> RenderParts {
    use dorc_plan::Disposition;
    let WhyReport {
        address,
        plan,
        probe,
        first_wall,
        wall_steps,
        why_diags,
        refusals,
        arena,
        ast,
        book_src,
        filename,
        interner,
        source_paths,
        source_srcs,
        narrative,
        cascades,
        receipt,
    } = *report;
    let declines = authored_declines(narrative);
    let unnarrated = if receipt.deepest_tier {
        unnarrated_lines(narrative, receipt.narratable)
    } else {
        Vec::new()
    };
    let mut sites: Vec<WhySite> = Vec::new();
    // A chain names the walls it crossed by `N|command`, never by internal site id (`28E` §8).
    let walls: BTreeMap<dorc_plan::LeafId, String> = plan
        .steps
        .iter()
        .map(|step| {
            let span = ast.node(step.ast).span;
            let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
            let line = dorc_aid::diag::line_col(book_src, lo).0;
            let text = book_src.get(lo..hi).unwrap_or("");
            let word = text.split_whitespace().next().unwrap_or("").to_owned();
            (step.leaf, format!("{line}|{word}"))
        })
        .collect();
    let lines_by_leaf: BTreeMap<dorc_plan::LeafId, usize> = plan
        .steps
        .iter()
        .map(|step| {
            let lo = ast.node(step.ast).span.lo.0 as usize;
            (step.leaf, dorc_aid::diag::line_col(book_src, lo).0)
        })
        .collect();
    let mut chains: Vec<(usize, ChainRender)> = Vec::new();
    for step in &plan.steps {
        let span = ast.node(step.ast).span;
        let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
        let line = dorc_aid::diag::line_col(book_src, lo).0;
        let raw = book_src.get(lo..hi).unwrap_or("<source unavailable>");
        let command = flatten_ws(raw);
        let word = raw.split_whitespace().next().unwrap_or("").to_owned();
        let reference = format!("{line}|{word}");
        if let Disposition::Replace(license, _) = &step.disposition
            && let Some(mut chain) = survival_chain(
                ctx,
                &reference,
                &format!("{filename}:{line}"),
                &step.disposition,
                license,
                &walls,
                interner,
                source_paths,
                source_srcs,
            )
        {
            let crossed = license
                .derivation()
                .survival
                .iter()
                .flat_map(dorc_plan::SurvivalWitness::crossings)
                .filter_map(|c| lines_by_leaf.get(&c.wall_leaf()).copied());
            chain.participants = participants(line, crossed);
            chains.push((line, chain));
        }
        if let Disposition::Guard(license) = &step.disposition {
            let above: Vec<&WallStep> = wall_steps
                .iter()
                .take_while(|wall| wall.leaf != step.leaf)
                .filter(|wall| matches!(wall.role, WallRole::Opaque | WallRole::Honest))
                .collect();
            let mut chain = guard_chain(
                ctx,
                &reference,
                &format!("{filename}:{line}"),
                &command,
                license,
                &above,
                interner,
                source_paths,
                source_srcs,
            );
            chain.participants = participants(line, above.iter().map(|wall| wall.line));
            chains.push((line, chain));
        }
        let authored_decline = declines.get(&step.leaf);
        if let Some(reason) = authored_decline {
            let mut chain = decline_chain(
                ctx,
                &reference,
                &format!("{filename}:{line}"),
                &word,
                reason,
                source_paths,
                source_srcs,
            );
            chain.participants = participants(line, std::iter::empty());
            chains.push((line, chain));
        }
        let refused = refusals.iter().any(|d| {
            d.primary
                .span()
                .is_some_and(|s| s.lo == span.lo && s.hi == span.hi)
        });
        let (reasons, class, improvement): (Vec<Said>, AggregateClass, Option<Said>) = match &step
            .disposition
        {
            Disposition::Run => {
                if let Some(reason) = authored_decline {
                    (
                        // No reason row: a declined site always has its own `decline_chain`, so
                        // `plain_chain`'s because-clause is unreachable here.
                        Vec::new(),
                        if reason.class.an_oracle_could_still_answer() {
                            AggregateClass::Improvement
                        } else {
                            AggregateClass::Quiet
                        },
                        reason
                            .class
                            .an_oracle_could_still_answer()
                            .then(|| Said::words("why-improvement-declined-unmodeled", &[&word])),
                    )
                } else if let Some(reason) = top_run_reason(ctx, span, why_diags, arena, book_src) {
                    (vec![reason], AggregateClass::Quiet, None)
                } else if probe.unresolvable.contains(&step.leaf)
                    && !is_structurally_unprobeable(&command)
                {
                    let mut reasons = vec![Said::words("why-reason-run-unprobed", &[])];
                    // upcoming-firstwall-hint: the FIRST unmodeled wall carries the forward
                    // reasoning here — the pull detail behind the plan-mode `hint:` nag.
                    if let Some(fw) = first_wall.filter(|fw| fw.leaf == step.leaf) {
                        reasons.push(fw.why_said());
                    }
                    (
                        reasons,
                        AggregateClass::Improvement,
                        Some(Said::words("why-improvement-ran-blind", &[&word])),
                    )
                } else {
                    (
                        vec![Said::words("why-reason-run-not-elidable", &[])],
                        AggregateClass::Quiet,
                        None,
                    )
                }
            }
            Disposition::Replace(license, _) => {
                let mut reasons = vec![Said::words(
                    "why-reason-skipped-converged",
                    &[&dorc_plan::fact_label(interner, license.fact())],
                )];
                if let Some(cascade) = cascades.get(&step.leaf) {
                    reasons.push(Said::words(
                        "why-reason-elide-cascaded",
                        &[
                            &cascade
                                .erased_lines
                                .iter()
                                .map(usize::to_string)
                                .collect::<Vec<_>>()
                                .join(", "),
                            &cascade.controller_line.to_string(),
                            &cascade.round.to_string(),
                        ],
                    ));
                }
                if refused {
                    reasons.push(Said::words("why-reason-render-refused", &[]));
                    (reasons, AggregateClass::Surprise, None)
                } else {
                    (reasons, AggregateClass::Quiet, None)
                }
            }
            Disposition::Guard(license) => {
                let kind = interner.resolve(license.fact().kind.0).to_owned();
                if refused {
                    (
                        vec![Said::words("why-reason-guard-refused", &[&kind])],
                        AggregateClass::Surprise,
                        None,
                    )
                } else {
                    // The leverage is the WALL, never the guarded line: an elided command
                    // casts no wall, so describing the wall is what frees this line.
                    let wall = first_wall.map(|fw| format!("{}|{}", fw.line, fw.word));
                    (
                        // No reason row: a guarded site always has its own `guard_chain`, so
                        // `plain_chain`'s because-clause is unreachable here.
                        Vec::new(),
                        if wall.is_some() {
                            AggregateClass::Improvement
                        } else {
                            AggregateClass::Quiet
                        },
                        wall.map(|w| Said::words("why-improvement-guarded-past-wall", &[&w])),
                    )
                }
            }
            Disposition::Omit { .. } => (
                vec![Said::words("why-reason-omitted", &[])],
                AggregateClass::Quiet,
                None,
            ),
        };
        sites.push(WhySite {
            line,
            word,
            command,

            outcome: outcome_word(ctx, &step.disposition),
            foil: foil_word(ctx, &step.disposition),
            reasons,
            class,
            improvement,
        });
    }

    match address {
        Some(addr) => why_triptych_parts(
            ctx,
            addr,
            &sites,
            &chains,
            filename,
            book_src,
            &unnarrated,
            receipt.deepest_tier,
        ),
        None => why_aggregate_parts(ctx, &sites, &chains, filename, first_wall, receipt),
    }
}

/// The ADDRESSED pull answer (`28G` Phase W1): the triptych for every site the address matched.
///
/// Two addressing forms, both file-qualified on the way OUT (`rul24-lineno-identity`: the ONE
/// line-number space is the source file's, so a `file:N` this prints is exactly the address a query
/// accepts back): a `book.sh:N` / bare `N` line-address, or free content substring-matched against
/// the command text.
///
/// A survived elision already carries a fully-populated triptych; every other disposition gets the
/// same three panels built from its own ANALYSIS rows, so the surface has exactly ONE shape.
#[expect(
    clippy::too_many_arguments,
    reason = "the addressed answer needs the matched sites, their chains, the book it quotes, and the two reader-tier flags; every one is a distinct pipeline output"
)]
fn why_triptych_parts(
    ctx: &RenderCtx<'_>,
    address: &str,
    sites: &[WhySite],
    chains: &[(usize, ChainRender)],
    filename: &str,
    book_src: &str,
    unnarrated: &[String],
    exhaustive: bool,
) -> RenderParts {
    let matched: Vec<&WhySite> = match parse_line_address(address) {
        Some(n) if address_names_book(address, filename) => {
            sites.iter().filter(|s| s.line == n).collect()
        }
        Some(_) => Vec::new(),
        None => sites
            .iter()
            .filter(|s| s.command.contains(address))
            .collect(),
    };
    if matched.is_empty() {
        return sentence_document(ctx, "why-address-unmatched", &[address]);
    }
    let mut nodes: Vec<Node<Face>> = Vec::new();
    for site in matched {
        let built;
        let chain = if let Some((_, chain)) = chains.iter().find(|(l, _)| *l == site.line) {
            chain
        } else {
            built = plain_chain(site);
            &built
        };
        let participants = if chain.participants.is_empty() {
            vec![site.line]
        } else {
            chain.participants.clone()
        };
        nodes.extend(participating_block(ctx, &participants, filename, book_src));
        nodes.extend(chain_nodes(ctx, chain, exhaustive));
    }
    nodes.extend(
        unnarrated
            .iter()
            .map(|line| paragraph(ctx, &Said::Value(line.clone()), "why-unnarrated-class")),
    );
    let mut parts = why_parts(nodes, TRIPTYCH_INSET);
    push_gap(&mut parts);
    // No trailing gap after the footer. It used to print one, and a trailing blank line is layout
    // the loom container cannot round-trip — so the transcript and the part stream said different
    // bytes, which is exactly what `28L:rul-editability-is-stamped-never-re-derived` forbids.
    parts.append(why_parts(
        vec![registry_paragraph(ctx, "why-receipt-footer")],
        0,
    ));
    parts
}

/// The triptych for a site with no survival chain: the same three panels, with each of the site's
/// ANALYSIS rows spoken by dorc in its own voice — which is honest, because these rows ARE engine
/// derivations rather than quotations of any speaker (`28E` §8 quoted-speakers).
///
/// The site's LEADING reason becomes the contrastive because-clause and the rest become ANALYSIS
/// rows, so nothing is said twice: a one-reason site collapses to OUTCOME alone
/// (`28E` §7 adopt-question-relative-informativeness — demote what the asker's own question already
/// fixed). NEXT STEPS is likewise omitted, the triptych-collapse `28G` strawman `e-skipped-quiet`
/// demonstrates. The richer per-disposition panels — a guarded line naming its wall, a declined line
/// showing the author's arm — are the narration lane's.
///
fn plain_chain(site: &WhySite) -> ChainRender {
    let (because, rest) = site
        .reasons
        .split_first()
        .map_or((None, &[][..]), |(head, tail)| (Some(head.clone()), tail));
    ChainRender {
        trust: None,
        outcome: contrastive(&site.reference(), &site.outcome, &site.foil),
        because,
        analysis_opener: Said::words("why-analysis-opener-plain", &[&site.reference()]),
        chain: ChainModel::all_selected(
            rest.iter()
                .map(|reason| ChainLink {
                    tier: SpeechAct::Derived,
                    speaker: Some(ENGINE_SPEAKER.to_owned()),
                    payload: reason.clone(),
                    quoted: false,
                    event: None,
                    explanation: None,
                    excerpt: None,
                })
                .collect(),
            None,
        ),
        participants: Vec::new(),
        shipped: None,
        next_steps: NextSteps {
            opener: Said::Value(String::new()),
            rows: Vec::new(),
        },
    }
}

/// The zero-argument `dorc why` aggregate (`28E` §8, human-demonstrated).
///
/// Section order is LAW, not taste: TRUST SPENT leads and is never capped
/// (`28E` §0 `rul-trust-spent-first-argless-why`, human-typed — danger in the user's face first),
/// SURPRISES follows and renders only when the world disagreed with the plan, and IMPROVEMENTS
/// closes, calm and quantified. The retired PROBLEMS section name appears nowhere.
///
/// The invocation record leads it ([`receipt_banner`]), because the reader arriving at 03:40 has to
/// know WHICH run they are reading before any item on it means anything.
fn why_aggregate_parts(
    ctx: &RenderCtx<'_>,
    sites: &[WhySite],
    chains: &[(usize, ChainRender)],
    filename: &str,
    first_wall: Option<&FirstWallHint>,
    receipt: &Receipt,
) -> RenderParts {
    let surprises: Vec<&WhySite> = sites
        .iter()
        .filter(|s| s.class == AggregateClass::Surprise)
        .collect();
    let improvements: Vec<&WhySite> = sites
        .iter()
        .filter(|s| s.class == AggregateClass::Improvement)
        .collect();
    let spent: Vec<(usize, &TrustSpent)> = chains
        .iter()
        .filter_map(|(line, chain)| Some((*line, chain.trust.as_ref()?)))
        .collect();
    if spent.is_empty() && surprises.is_empty() && improvements.is_empty() {
        let mut parts = why_parts(vec![receipt_banner(ctx, receipt)], 0);
        push_gap(&mut parts);
        parts.append(sentence_document(ctx, "why-nothing-to-report", &[filename]));
        return parts;
    }

    let mut nodes: Vec<Node<Face>> = vec![receipt_banner(ctx, receipt)];
    if !spent.is_empty() {
        let items = spent
            .iter()
            .filter_map(|(line, trust)| {
                let site = sites.iter().find(|s| s.line == *line)?;
                let reason = Said::words(
                    "why-trust-spent-item-reason",
                    &[&trust.crossed, &trust.claimant],
                );
                Some(aggregate_item(ctx, site, filename, &[&reason]))
            })
            .collect();
        nodes.push(panel(ctx, "why-trust-spent-heading", items));
    }
    if !surprises.is_empty() {
        let items = surprises
            .iter()
            .map(|site| {
                let reasons: Vec<&Said> = site.reasons.iter().collect();
                aggregate_item(ctx, site, filename, &reasons)
            })
            .collect();
        nodes.push(panel(ctx, "why-surprises-heading", items));
    }
    if !improvements.is_empty() {
        let mut items: Vec<Node<Face>> = improvements
            .iter()
            .map(|site| {
                let reasons: Vec<&Said> = site.improvement.iter().collect();
                aggregate_item(ctx, site, filename, &reasons)
            })
            .collect();
        if let Some(fw) = first_wall.filter(|fw| fw.unwall > 0) {
            items.push(paragraph(
                ctx,
                &Said::words(
                    "why-improvement-quantified",
                    &[&fw.word, &fw.unwall.to_string()],
                ),
                "why-improvement-quantified",
            ));
        }
        nodes.push(panel(ctx, "why-improvements-heading", items));
    }
    nodes.push(registry_paragraph(ctx, "why-receipt-footer"));
    why_parts(nodes, 0)
}

/// One aggregate item: the `file:N | command` headline, its reason beneath, and the
/// `dorc why <addr>` pointer that turns it into the next question (`28E` §8 row shape).
///
/// The command is the ADMIN's own bytes, so it rides the foreign-text class rather than any
/// registry row — un-editable, and encoded on the way in (`28G` §0).
fn aggregate_item(
    ctx: &RenderCtx<'_>,
    site: &WhySite,
    filename: &str,
    reasons: &[&Said],
) -> Node<Face> {
    let address = site.address(filename);
    let mut runs: Vec<Run<Face>> = Vec::new();
    for reason in reasons {
        if !runs.is_empty() {
            runs.push(dorc_aid::weave::mark(" ", "why-item-reason-gap"));
        }
        runs.extend(reason.runs(ctx, "why-item-reason"));
    }
    Node::new(NodeKind::Banner(Banner {
        headline: vec![
            dorc_aid::weave::value(&address, "why-item-address", WHY_VALUE_CAP),
            dorc_aid::weave::mark(" | ", "why-item-gutter"),
            dorc_aid::weave::foreign(
                &dorc_aid::ForeignBytes::from_io_edge(&site.command),
                filename,
                WHY_SOURCE_CAP,
            ),
        ],
        body: vec![
            Node::new(NodeKind::Prose(Paragraph { runs })),
            Node::new(NodeKind::Pointer(PointerLine {
                placement: weft::Placement::Standalone,
                target: Said::words("why-item-pointer", &[&address]).runs(ctx, "why-item"),
            })),
        ],
    }))
}

/// The ⊤-run cause for a Run site, if a `why_diags` disclosure covers it: the FIRST diag whose
/// primary span starts inside this command's span (the cmdsub-⊤ origin sits at/within the
/// command), rendered through the why-lens [`dorc_aid::diag::why`] (the same cause-chain the
/// `plan` render surfaces). `None` ⇒ no ⊤-cause (the caller falls to unprobed / not-elidable).
fn top_run_reason(
    ctx: &RenderCtx<'_>,
    span: dorc_core::Span,
    why_diags: &[Diag],
    arena: &ProvArena,
    book_src: &str,
) -> Option<Said> {
    why_diags.iter().find_map(|d| {
        let psp = d.primary.span()?;
        (psp.lo.0 >= span.lo.0 && psp.lo.0 < span.hi.0)
            .then(|| dorc_aid::diag::why(ctx, d, arena, book_src).map(|e| Said::Parts(e.parts)))
            .flatten()
    })
}

/// Parse a `dorc why` address as a SOURCE line-number (rul24-lineno-identity): `book.sh:12` ⇒ 12,
/// bare `12` ⇒ 12 (the tail after the last `:` when numeric); a non-numeric tail ⇒ `None` ⇒ the
/// caller treats the address as free CONTENT to substring-match.
fn parse_line_address(addr: &str) -> Option<usize> {
    addr.rsplit(':')
        .next()
        .unwrap_or(addr)
        .parse::<usize>()
        .ok()
}

/// Does a file-QUALIFIED address name the book this run analyzed? A bare `12` names no file and
/// always matches; `web.sh:12` matches only `web.sh`, compared on the trailing path component so a
/// pasted `./web.sh:12` or an absolute path still resolves.
///
/// Load-bearing because the render now PRINTS file-qualified pointers: without the check, a
/// qualified address naming some other book silently answers for the analyzed one at rc 0 — the
/// same silent-wrong-surface class as `289:rider-why-last-address-order`.
fn address_names_book(addr: &str, book_name: &str) -> bool {
    let Some((file, _)) = addr.rsplit_once(':') else {
        return true;
    };
    if file.is_empty() {
        return true;
    }
    let tail = |path: &str| {
        path.rsplit(['/', '\\'])
            .next()
            .unwrap_or(path)
            .to_ascii_lowercase()
    };
    tail(file) == tail(book_name)
}

/// Render a [`dorc_plan::EntityCoord`] as `kind:entity` for the attribution surface (empty
/// entity ⇒ `kind:`, the singleton form). DISPLAY only — resolving an interned symbol for
/// provenance is explicitly permitted; the engine never DECODES it for meaning
/// (`inv-referent-agnostic`).
#[must_use]
pub fn render_coord(coord: dorc_plan::EntityCoord, interner: &Interner) -> String {
    let kind = interner.resolve(coord.kind().0);
    let entity = match coord.entity() {
        dorc_core::EntityRef::Operand(token) => interner.resolve(token.0),
        dorc_core::EntityRef::Singleton => "",
    };
    format!("{kind}:{entity}")
}

#[cfg(test)]
mod because_clause_tests {
    use super::{RenderCtx, Said, WHY_VALUE_CAP, contrastive};

    /// The full cmdsub reason, as `dorc_aid::diag::why` composes it: our opener row, our locus
    /// value, the book's own bytes, our remediation row. 281 bytes at the shape that motivated
    /// the fix.
    fn composed_reason() -> Said {
        Said::Parts(vec![
            Said::words("why-reason-cmdsub-opener", &["operand 3"]),
            Said::Value("6:20".to_owned()),
            Said::Mark("why-cause-quote", " `".to_owned()),
            Said::foreign(
                &dorc_aid::ForeignBytes::from_io_edge("apt-get install -y \"$(resolve-dynamism)\""),
                "book.sh",
            ),
            Said::Mark("why-cause-quote", "`".to_owned()),
            Said::words("why-reason-cmdsub-closer", &[]),
            Said::words("why-remediation-resolve-dynamism", &[]),
        ])
    }

    /// A because-clause is its own registry LINE, never a value of the outcome row
    /// (`28L:rul-composed-saids-render-as-own-lines`). Two things follow and both are pinned here:
    /// it reaches the render WHOLE — as a value it was re-capped at [`WHY_VALUE_CAP`] and cut a
    /// 281-byte reason mid-sentence (`28H:ask-because-clause-truncates-at-two-forty`) — and its
    /// runs name its OWN entries, which is what a transcript edit has to be able to address.
    #[test]
    fn a_because_clause_is_its_own_line_and_reaches_the_render_whole() {
        let ctx = RenderCtx::production();
        let reason = composed_reason();
        let whole = reason.text(&ctx);
        assert!(
            whole.len() > WHY_VALUE_CAP,
            "the fixture must exceed the raw-value budget or it proves nothing: {} bytes",
            whole.len()
        );
        let runs = reason.runs(&ctx, "why-outcome-because");
        let bytes: String = runs.iter().map(|run| run.text.clone()).collect();
        assert_eq!(bytes, whole, "nothing re-caps our own words");
        assert!(
            runs.iter().any(|run| matches!(
                &run.provenance,
                weft::Provenance::Arrangement {
                    key: Some(dorc_aid::weave::Face::Row { slug, .. })
                } if *slug == "why-reason-cmdsub-opener"
            )),
            "the clause's own entry is nameable in the span map"
        );
    }

    /// The budget still binds where it was FOR: a raw value is somebody else's bytes, and one
    /// pathological book word may not own the render.
    #[test]
    fn a_raw_value_interpolated_into_the_outcome_row_is_still_capped() {
        let sentence = contrastive("14|apt-get", &"z".repeat(WHY_VALUE_CAP * 2), "skipped");
        let rendered: String = sentence
            .runs(&RenderCtx::production(), "why-outcome")
            .iter()
            .map(|run| run.text.clone())
            .collect();
        assert!(
            rendered.contains("..."),
            "a raw value is capped: {rendered}"
        );
    }
}

#[cfg(test)]
mod brace_selector_tests {
    use super::brace_selectors;

    /// The whole point of the aggregation is that ONE entity reads as one thing. Two cells of one
    /// entity spelled as two full coordinates make a reader compare two long strings character by
    /// character to notice they differ only in the selector; `281` §7's brace-alternation puts the
    /// difference where the eye already is.
    #[test]
    fn cells_of_one_entity_collapse_to_one_braced_coordinate() {
        assert_eq!(
            brace_selectors(&[
                "sm.dorc.Package:nginx@enabled".to_owned(),
                "sm.dorc.Package:nginx@active".to_owned(),
            ]),
            "sm.dorc.Package:nginx@{enabled,active}"
        );
    }

    /// Grouping must never merge across entities: `nginx` and `redis` are different things, and a
    /// render that ran them together would be claiming a skip rested on cells it did not.
    #[test]
    fn different_entities_stay_separate_coordinates() {
        assert_eq!(
            brace_selectors(&[
                "sm.dorc.Package:nginx@installed".to_owned(),
                "sm.dorc.Package:redis@installed".to_owned(),
            ]),
            "sm.dorc.Package:nginx@installed sm.dorc.Package:redis@installed"
        );
    }

    /// A single cell keeps its plain spelling — braces around one token would suggest a set where
    /// there is one member — and a selector-less label passes through untouched, since the
    /// whole-entity form means something different from any braced set.
    #[test]
    fn a_lone_cell_and_a_selectorless_label_are_left_alone() {
        assert_eq!(
            brace_selectors(&["sm.dorc.Package:nginx@installed".to_owned()]),
            "sm.dorc.Package:nginx@installed"
        );
        assert_eq!(
            brace_selectors(&["sm.dorc.Package:nginx".to_owned()]),
            "sm.dorc.Package:nginx"
        );
    }

    /// A repeated cell is one cell. Two erased establishes of the same coordinate say the same
    /// thing, and `@{active,active}` would read as two distinct pieces of evidence.
    #[test]
    fn a_repeated_cell_is_not_listed_twice() {
        assert_eq!(
            brace_selectors(&[
                "sm.dorc.Service:nginx@active".to_owned(),
                "sm.dorc.Service:nginx@active".to_owned(),
            ]),
            "sm.dorc.Service:nginx@active"
        );
    }
}

#[cfg(test)]
mod first_wall_tests {
    //! upcoming-firstwall-hint (`USER_STORY` stage 3): the pure `first_wall_hint` M-computation and
    //! its wording. Each `WallStep` scenario mirrors a real book shape — the role classification
    //! (opaque = probe-unresolvable real command; honest = running modeled mutator; guard =
    //! converged-but-walled) is exercised end-to-end on `guard23-ternary-flagship` (M=1) and
    //! `strawman24-opaque-wall` (M=0) by the e2e corpus; here we pin the algorithm over the roles.
    use super::{
        Excerpt, FirstWallHint, RenderCtx, WallRole, WallStep, first_wall_hint, oracle_excerpt,
    };

    fn ws(leaf: u32, line: usize, word: &str, role: WallRole) -> WallStep {
        WallStep {
            leaf: dorc_plan::LeafId(leaf),
            line,
            word: word.to_owned(),
            role,
        }
    }

    /// The flagship shape (guard23-ternary-flagship): nginx elides, `hork` is the opaque wall,
    /// curl guards past it, vim runs diverged (an honest wall). Lifting `hork` un-walls exactly
    /// curl ⇒ M=1, no further unmodeled walls.
    #[test]
    fn opaque_wall_with_downstream_guard_yields_m1() {
        let steps = [
            ws(0, 20, "apt-get", WallRole::Transparent),
            ws(1, 21, "hork", WallRole::Opaque),
            ws(2, 22, "apt-get", WallRole::Guard),
            ws(3, 23, "apt-get", WallRole::Honest),
        ];
        let fw = first_wall_hint(&steps).expect("an opaque wall fires the hint");
        assert_eq!(fw.line, 21);
        assert_eq!(fw.word, "hork");
        assert_eq!(fw.unwall, 1);
        assert_eq!(fw.more_walls, 0);
    }

    /// No opaque wall (every command modeled) ⇒ no hint — the mission's no-unmodeled-wall negative.
    #[test]
    fn no_opaque_wall_yields_none() {
        let steps = [
            ws(0, 1, "apt-get", WallRole::Transparent),
            ws(1, 2, "apt-get", WallRole::Guard),
            ws(2, 3, "apt-get", WallRole::Honest),
        ];
        assert!(first_wall_hint(&steps).is_none());
    }

    /// A modeled-but-diverged wall (Honest) with NO opaque wall ⇒ no hint. The mission's explicit
    /// "NOT fire for modeled-but-diverged walls" — those are honest walls, not oracle-gaps.
    #[test]
    fn honest_wall_only_yields_none() {
        let steps = [
            ws(0, 1, "apt-get", WallRole::Honest),
            ws(1, 2, "systemctl", WallRole::Guard),
        ];
        assert!(first_wall_hint(&steps).is_none());
    }

    #[test]
    fn empty_book_yields_none() {
        assert!(first_wall_hint(&[]).is_none());
    }

    /// Two converged-but-walled guards before the next wall ⇒ M=2 (both upgrade guard→elide).
    #[test]
    fn two_guards_before_next_wall_yields_m2() {
        let steps = [
            ws(0, 1, "hork", WallRole::Opaque),
            ws(1, 2, "systemctl", WallRole::Guard),
            ws(2, 3, "ufw", WallRole::Guard),
            ws(3, 4, "apt-get", WallRole::Honest),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(fw.unwall, 2);
        assert_eq!(fw.more_walls, 0);
    }

    /// An honest wall BOUNDS the count: a guard past it is walled by IT, not by the opaque wall, so
    /// lifting the opaque wall would not recover it. Only the guard in the opaque wall's own window
    /// counts ⇒ M=1.
    #[test]
    fn honest_wall_bounds_the_count() {
        let steps = [
            ws(0, 1, "hork", WallRole::Opaque),
            ws(1, 2, "systemctl", WallRole::Guard),
            ws(2, 3, "apt-get", WallRole::Honest),
            ws(3, 4, "ufw", WallRole::Guard),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(
            fw.unwall, 1,
            "the guard past the honest wall is not this wall's to un-wall"
        );
    }

    /// The two-opaque-wall shape (`USER_STORY` foobar + hork): a second opaque wall both BOUNDS the
    /// first's window (ufw past `hork` is not foobar's to un-wall) and adds a trailing pointer.
    #[test]
    fn second_opaque_wall_bounds_and_is_counted() {
        let steps = [
            ws(0, 8, "foobar", WallRole::Opaque),
            ws(1, 9, "systemctl", WallRole::Guard),
            ws(2, 10, "hork", WallRole::Opaque),
            ws(3, 11, "ufw", WallRole::Guard),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(fw.word, "foobar");
        assert_eq!(fw.unwall, 1);
        assert_eq!(fw.more_walls, 1);
    }

    /// An opaque wall with nothing improvable downstream ⇒ M=0 (still fires — you can elide it).
    #[test]
    fn opaque_wall_with_no_downstream_yields_m0() {
        let steps = [
            ws(0, 1, "apt-get", WallRole::Transparent),
            ws(1, 2, "hork", WallRole::Opaque),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(fw.unwall, 0);
        assert_eq!(fw.more_walls, 0);
    }

    /// A transparent step (inert builtin run / omit) between the wall and a guard does NOT bound the
    /// count — only a wall (opaque or honest) stops it.
    #[test]
    fn transparent_step_does_not_bound() {
        let steps = [
            ws(0, 1, "hork", WallRole::Opaque),
            ws(1, 2, "echo", WallRole::Transparent),
            ws(2, 3, "systemctl", WallRole::Guard),
            ws(3, 4, "apt-get", WallRole::Honest),
        ];
        assert_eq!(first_wall_hint(&steps).unwrap().unwall, 1);
    }

    fn hint(unwall: usize, more_walls: usize) -> FirstWallHint {
        FirstWallHint {
            leaf: dorc_plan::LeafId(1),
            line: 8,
            word: "foobar".to_owned(),
            unwall,
            more_walls,
        }
    }

    #[test]
    fn body_wording_matches_the_user_story_register() {
        // M=1, no further walls — the USER_STORY stage-3 sharpened form.
        assert_eq!(
            hint(1, 0).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged, and un-wall 1 downstream site"
        );
        // M=2 ⇒ "sites"; a further wall ⇒ the trailing pointer.
        assert_eq!(
            hint(2, 1).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged, and un-wall 2 downstream sites; 1 more \
             unmodeled wall -- dorc why"
        );
        // M=0 ⇒ the un-wall clause is dropped (never "un-wall 0").
        assert_eq!(
            hint(0, 0).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged"
        );
        // more_walls plural.
        assert!(
            hint(0, 2)
                .body()
                .ends_with("; 2 more unmodeled walls -- dorc why")
        );
    }

    /// The pull-surface detail carries the recovery COUNT when there is one, and never a bare zero.
    /// Structure, not bytes: the words are arrangement-registry rows and ride
    /// `27V:rul-output-form-unwelded`, so pinning them verbatim here would weld exactly what that
    /// rule keeps free — and would re-break on every prose pass.
    #[test]
    fn why_detail_carries_the_unwall_count() {
        let with_count = hint(1, 0).why_said().text(&RenderCtx::production());
        assert!(
            with_count.contains('1'),
            "the recovery count must reach the reader: {with_count}"
        );
        let without = hint(0, 0).why_said().text(&RenderCtx::production());
        assert!(
            !without.contains('0'),
            "a zero count is dropped, never rendered as `0 sites`: {without}"
        );
        assert!(
            without.len() < with_count.len(),
            "the count-free form is the shorter one (the clause was dropped, not blanked)"
        );
        assert!(
            !without.contains("  "),
            "the count-free occurrence renders no gap where the dropped clause used to sit: \
             {without}"
        );
    }

    /// `rul-ascii-output-forever` (`28E` §0, human-typed: "no unicode, ever. period. anywhere").
    /// The why-surface strings are the ones this lane authored or respelled; a stray em-dash or
    /// arrow creeping back into one is exactly what this catches.
    #[test]
    fn the_why_surface_renders_pure_ascii() {
        let mut checked: usize = 0;
        for entry in dorc_aid::arrangement::ARRANGEMENTS {
            if !entry.slug.starts_with("why-") {
                continue;
            }
            for word in entry.words.words().unwrap_or(&[]) {
                assert!(
                    word.is_ascii(),
                    "arrangement `{}` carries non-ASCII output: {word:?}",
                    entry.slug
                );
                checked = checked.saturating_add(1);
            }
        }
        assert!(checked > 0, "the why-surface registry rows must be reached");
        assert!(hint(1, 1).body().is_ascii());
        assert!(
            hint(1, 1)
                .why_said()
                .text(&RenderCtx::production())
                .is_ascii()
        );
    }

    /// A synthetic oracle whose `disturbs` arm is preceded by the author's comment and followed by
    /// a body long enough to force a cut.
    const ARM_SOURCE: &str = "#!/usr/bin/env dorc-sh\n\
        # dorc-lang/v0.2\n\
        \n\
        # surveyed 2026-05: cert store only.\n\
        certsync__disturbs() {\n\
        line six\n\
        line seven\n\
        line eight\n\
        line nine\n\
        line ten\n\
        line eleven\n\
        }\n";

    fn excerpt_of(lo: u32, hi: u32) -> Excerpt {
        oracle_excerpt(
            Some((
                dorc_core::Span::new(dorc_core::BytePos(lo), dorc_core::BytePos(hi)),
                dorc_core::SourceFileId(0),
            )),
            &["certsync.oracle.sh".to_owned()],
            &[ARM_SOURCE.to_owned()],
        )
        .expect("the fixture threads a span into a loaded oracle file")
    }

    /// The author's comment ABOVE an arm is the author explaining that arm, so the massaging
    /// license (`27W:rul-report-surface-massaging`) attaches it — and stops at the blank line,
    /// rather than dragging the file's whole header down with it.
    #[test]
    fn an_excerpt_attaches_the_authors_adjacent_comment_and_stops_at_the_gap() {
        let arm_start = ARM_SOURCE
            .find("certsync__disturbs")
            .expect("the fixture defines the member");
        let arm = excerpt_of(
            u32::try_from(arm_start).expect("fixture offsets fit"),
            u32::try_from(arm_start).expect("fixture offsets fit"),
        );
        let numbers: Vec<usize> = arm.head.iter().map(|(number, _)| *number).collect();
        assert_eq!(
            numbers,
            vec![4, 5],
            "the comment on line 4 rides along; the version marker on line 2 is across a blank \
             line and is not this arm's"
        );
        assert_eq!(arm.elided, 0, "a two-line excerpt is contiguous");
    }

    /// A long arm is CUT, and the cut is reported rather than closed over: an excerpt that quietly
    /// shortened an author's contract would misrepresent what they wrote.
    #[test]
    fn a_long_excerpt_reports_the_middle_it_cut() {
        let whole = excerpt_of(0, u32::try_from(ARM_SOURCE.len()).expect("fixture fits"));
        assert!(
            whole.elided > 0,
            "a twelve-line span exceeds the inline budget and must be cut"
        );
        assert!(
            !whole.tail.is_empty(),
            "the cut keeps the arm's LAST line: where a case arm returns is what the reader needs"
        );
        let shown = whole.head.len().saturating_add(whole.tail.len());
        assert_eq!(
            shown.saturating_add(whole.elided),
            ARM_SOURCE.lines().count(),
            "every source line is either shown or counted in the cut, never silently dropped"
        );
    }
}

#[cfg(test)]
mod not_ours_bytes_tests {
    //! The why surface shows bytes we did not write.
    //!
    //! Oracle arms, their authors' comments, book lines and host-reported text all reach a
    //! terminal through this surface. They are classed not-ours and encoded on the way in
    //! (`aid::weave::foreign` over the `aid::display` seat), and these four tests are what keeps
    //! that true for somebody who never read this comment. They read as one battery: the sweep
    //! covers every seat, the classification test pins which class the bytes wear, the hostile
    //! fixtures drive real dangerous input through the real render, and the last one pins the
    //! byte-floored artifact plane OUT of all of it.
    use super::*;
    use crate::PlanTally;
    use dorc_core::{Observable, Verdict};

    /// Bytes that must never reach a terminal as themselves, plus one plain non-ASCII sequence.
    const HOSTILE_SAMPLES: &[&str] = &[
        "\u{1b}",
        "\u{1b}[31m before-and-after",
        "\u{202e}reversed\u{202c}",
        "nul\u{0}and\u{7f}del",
        "tab\there",
        "na\u{ef}ve \u{feff}zero-width",
    ];

    /// A distinctive run of printable ASCII: it survives encoding VERBATIM, so wherever it lands
    /// in a render is exactly where the seat that emitted it put it.
    const SOURCE_MARK: &str = "source-mark-9137";

    /// A cause-explanation's own shape, driven with hostile bytes: our coordinate, our quotes, and
    /// somebody else's line between them ([`dorc_aid::diag::why`]'s parts). The composite is the
    /// one fragment whose runs are classed INDIVIDUALLY, so the sweep has to see one.
    fn cause_shaped_parts() -> Said {
        Said::Parts(vec![
            Said::Value(hostile_line(5)),
            Said::Mark("why-cause-quote", " `".to_owned()),
            Said::foreign(
                &dorc_aid::ForeignBytes::from_io_edge(&hostile_line(4)),
                "sweep.book.sh",
            ),
            Said::Mark("why-cause-quote", "`".to_owned()),
        ])
    }

    /// One hostile sample folded into a line of somebody else's source.
    fn hostile_line(index: usize) -> String {
        let sample = HOSTILE_SAMPLES
            .get(index.checked_rem(HOSTILE_SAMPLES.len()).unwrap_or_default())
            .copied()
            .unwrap_or_default();
        format!("{SOURCE_MARK} {sample} # arm {index}")
    }

    /// Render a why-surface node list the way the surface itself does, and hand back the span map.
    fn swept(nodes: Vec<Node<Face>>) -> weft::Rendered<Face> {
        let frame = weft::Frame::of_width(weft::Width::new(crate::WHY_WIDTH)).inset(TRIPTYCH_INSET);
        weft::render_framed(&weft::Document::new(nodes), &frame)
    }

    /// Every seat of the why surface, driven with bytes we did not write in every slot that takes
    /// them. Built from STRUCT LITERALS on purpose: a new field on any of these types stops this
    /// compiling, so whoever adds one has to decide what hostile content belongs in it.
    #[expect(
        clippy::too_many_lines,
        reason = "one exhaustive literal per why-surface seat; splitting it hides which seats the sweep covers"
    )]
    fn every_why_surface_node() -> Vec<Node<Face>> {
        let book = format!("#!/bin/sh\n{}\n{}\n", hostile_line(0), hostile_line(1));
        let excerpt = Excerpt {
            path: format!("{SOURCE_MARK}.oracle.sh"),
            head: vec![(10, hostile_line(2)), (11, hostile_line(3))],
            tail: vec![(90, hostile_line(4))],
            elided: 78,
        };
        let chain = ChainRender {
            trust: Some(TrustSpent {
                crossed: hostile_line(5),
                claimant: hostile_line(0),
            }),
            outcome: Said::words(
                "why-outcome-contrastive",
                &[&hostile_line(1), &hostile_line(2), &hostile_line(3)],
            ),
            because: Some(Said::Value(hostile_line(4))),
            analysis_opener: cause_shaped_parts(),
            chain: ChainModel::all_selected(
                vec![ChainLink {
                    tier: SpeechAct::Claimed,
                    speaker: Some(hostile_line(0)),
                    payload: Said::Value(hostile_line(1)),
                    quoted: true,
                    event: Some(Said::words("why-chain-event-rc-only", &[&hostile_line(2)])),
                    explanation: Some(Said::foreign(
                        &dorc_aid::ForeignBytes::from_io_edge(&hostile_line(3)),
                        "sweep.oracle.sh",
                    )),
                    excerpt: Some(excerpt),
                }],
                Some(Said::Value(hostile_line(5))),
            ),
            participants: vec![2, 3],
            shipped: Some(hostile_line(4)),
            next_steps: NextSteps {
                opener: Said::Value(hostile_line(0)),
                rows: vec![
                    StepRow {
                        label: StepLabel::Fix,
                        body: Said::Value(hostile_line(1)),
                        alternative: false,
                    },
                    StepRow {
                        label: StepLabel::Review,
                        body: Said::words("why-next-step-review", &[&hostile_line(2)]),
                        alternative: true,
                    },
                ],
            },
        };
        let site = WhySite {
            line: 2,
            word: hostile_line(3),
            command: hostile_line(4),
            outcome: outcome_word(&RenderCtx::production(), &dorc_plan::Disposition::Run),
            foil: foil_word(&RenderCtx::production(), &dorc_plan::Disposition::Run),
            reasons: vec![Said::Value(hostile_line(5))],
            class: AggregateClass::Improvement,
            improvement: Some(Said::foreign(
                &dorc_aid::ForeignBytes::from_io_edge(&hostile_line(0)),
                "sweep.oracle.sh",
            )),
        };
        let receipt = Receipt {
            at: None,
            replayed: false,
            host: hostile_line(1),
            book: hostile_line(2),
            book_digest: hostile_line(3),
            at_head: None,
            oracles: vec![hostile_line(4), hostile_line(5)],
            risk_profile: Some(CONSENT_FLAG),
            tally: PlanTally::Derived(dorc_plan::DispositionCounts {
                sites: 2,
                elide: 1,
                elide_by_proof: 0,
                elide_by_trusted_claim: 1,
                omit: 0,
                guard: 0,
                run: 1,
            }),
            deepest_tier: true,
            narratable: true,
        };
        let mut nodes = vec![
            receipt_banner(&RenderCtx::production(), &receipt),
            receipt_banner(&RenderCtx::production(), &at_head(&receipt)),
        ];
        nodes.extend(participating_block(
            &RenderCtx::production(),
            &[2, 3],
            &format!("{SOURCE_MARK}.book.sh"),
            &book,
        ));
        nodes.extend(chain_nodes(&RenderCtx::production(), &chain, true));
        nodes.push(aggregate_item(
            &RenderCtx::production(),
            &site,
            &format!("{SOURCE_MARK}.book.sh"),
            &[&cause_shaped_parts()],
        ));
        nodes.extend(chain_nodes(
            &RenderCtx::production(),
            &plain_chain(&site),
            false,
        ));
        nodes
    }

    /// The same receipt wearing its git-annotation row instead of its digest row. The two are
    /// exclusive, so the sweep renders both banners: a commit is a SUBPROCESS's stdout, as not-ours
    /// as anything a host reported (`28D:must-encode-per-surface`).
    fn at_head(receipt: &Receipt) -> Receipt {
        Receipt {
            at_head: Some(crate::SourceMatch {
                commit: hostile_line(6),
            }),
            oracles: receipt.oracles.clone(),
            host: receipt.host.clone(),
            book: receipt.book.clone(),
            book_digest: receipt.book_digest.clone(),
            at: receipt.at,
            replayed: receipt.replayed,
            risk_profile: receipt.risk_profile,
            tally: receipt.tally,
            deepest_tier: receipt.deepest_tier,
            narratable: receipt.narratable,
        }
    }

    /// THE SWEEP. Over every seat the why surface has, every not-ours run is already
    /// encoder-clean, and nothing else carries a control or bidi character.
    ///
    /// This is the test that bites somebody who adds a show-the-code row and hands weft the
    /// bytes directly. It does not care where they added it or what they called it: if the run
    /// reaches the render carrying anything a terminal would act on, or carrying an escape a
    /// second encoding pass would change, the sweep names the span and fails.
    #[test]
    fn every_why_surface_run_is_already_encoded_and_only_not_ours_bytes_are_escaped() {
        let rendered = swept(every_why_surface_node());
        let text = rendered.text().to_owned();
        assert!(!text.is_empty(), "the sweep must reach a real render");
        let mut foreign_spans = 0_usize;
        for span in rendered.spans() {
            let bytes = text
                .get(span.start..span.end())
                .expect("a span lies within its own render");
            if matches!(span.provenance, weft::Provenance::Foreign { .. }) {
                foreign_spans = foreign_spans.saturating_add(1);
                assert_eq!(
                    dorc_aid::display::encode_foreign(bytes, WHY_SOURCE_CAP),
                    bytes,
                    "a not-ours run reached the render un-encoded (re-encoding changed it): \
                     {bytes:?}"
                );
                continue;
            }
            for c in bytes.chars() {
                assert!(
                    c == '\n' || dorc_aid::display::is_display_safe(c),
                    "a run that is NOT classed not-ours carries {c:?}, which a terminal acts on \
                     — either the bytes are somebody else's and belong in a foreign run, or the \
                     value needs the display seat before it is interleaved: {bytes:?}"
                );
                assert!(
                    c.is_ascii(),
                    "the surface is pure ASCII (`rul-ascii-output-forever`) and weft measures \
                     bytes as columns; {c:?} reached it: {bytes:?}"
                );
            }
        }
        assert!(
            foreign_spans > 4,
            "only {foreign_spans} not-ours runs reached the sweep — the seats it means to cover \
             are not being reached"
        );
    }

    /// CLASSIFICATION. Inlined source lands ONLY in not-ours runs.
    ///
    /// The marker is printable ASCII, so encoding leaves it verbatim and every occurrence in the
    /// render is a seat that emitted somebody else's bytes. Any occurrence outside a foreign run
    /// is a show-the-code site wearing the wrong class — which reads to a round-trip as OUR
    /// words, and therefore as rephrasable prose.
    #[test]
    fn inlined_source_bytes_appear_only_inside_not_ours_runs() {
        let rendered = swept(vec![
            {
                let excerpt = Excerpt {
                    path: "certsync.oracle.sh".to_owned(),
                    head: vec![(4, format!("# {SOURCE_MARK} the author's own comment"))],
                    tail: Vec::new(),
                    elided: 0,
                };
                Node::new(NodeKind::Section(Section {
                    header: Said::words("why-analysis-heading", &[])
                        .runs(&RenderCtx::production(), "why-analysis-heading"),
                    counts: None,
                    body: excerpt_nodes(&RenderCtx::production(), &excerpt),
                }))
            },
            shipped_block(&format!("( certsync__is_converged ) || {SOURCE_MARK}")),
        ]);
        let text = rendered.text().to_owned();
        assert!(
            text.matches(SOURCE_MARK).count() >= 2,
            "both inlined-source seats must reach the render: {text}"
        );
        let mut inside_foreign = 0_usize;
        for span in rendered.spans() {
            let bytes = text
                .get(span.start..span.end())
                .expect("a span lies within its own render");
            let hits = bytes.matches(SOURCE_MARK).count();
            if hits == 0 {
                continue;
            }
            assert!(
                matches!(span.provenance, weft::Provenance::Foreign { .. }),
                "inlined source landed in a run classed {:?} — somebody else's bytes must wear \
                 the not-ours class, never a template, value or arrangement one: {bytes:?}",
                span.provenance
            );
            inside_foreign = inside_foreign.saturating_add(hits);
        }
        assert!(
            inside_foreign >= 2,
            "the marker was found in the text but not inside any not-ours span"
        );
    }

    /// HOSTILE FIXTURES, through the real render. Each is encoded, and each survives non-empty:
    /// silently dropping an author's text would be its own kind of lie about the source.
    #[test]
    fn a_hostile_oracle_comment_is_encoded_and_never_silently_dropped() {
        let long = "L".repeat(WHY_SOURCE_CAP.saturating_mul(3));
        let cases: Vec<(&str, String)> = vec![
            ("a bare escape", "\u{1b}".to_owned()),
            ("a CSI colour sequence", "\u{1b}[31mred\u{1b}[0m".to_owned()),
            ("a bidi override", "# \u{202e}rewordppa\u{202c}".to_owned()),
            ("NUL and DEL", "a\u{0}b\u{7f}c".to_owned()),
            ("a line far past the cap", long),
            (
                "valid non-ASCII UTF-8",
                "# na\u{ef}ve \u{2014} surveyed".to_owned(),
            ),
        ];
        for (what, source) in cases {
            let excerpt = Excerpt {
                path: "hostile.oracle.sh".to_owned(),
                head: vec![(1, source.clone())],
                tail: Vec::new(),
                elided: 0,
            };
            let rendered = swept(excerpt_nodes(&RenderCtx::production(), &excerpt));
            let text = rendered.text();
            assert!(
                text.is_ascii(),
                "{what} reached the terminal un-encoded: {text:?}"
            );
            for c in text.chars() {
                assert!(
                    c == '\n' || dorc_aid::display::is_display_safe(c),
                    "{what} left {c:?} in the render: {text:?}"
                );
            }
            let foreign: String = rendered
                .spans()
                .iter()
                .filter(|span| matches!(span.provenance, weft::Provenance::Foreign { .. }))
                .filter_map(|span| text.get(span.start..span.end()))
                .collect();
            assert!(
                !foreign.trim().is_empty(),
                "{what} was dropped rather than encoded — an author's text always survives in \
                 some readable form"
            );
        }
    }

    /// THE ARTIFACT PLANE STAYS OUT OF IT. Display encoding is a render-plane act; the emitted
    /// probe and apply are byte-floored (`two-surfaces`, `law-render-overlay-never-artifact`) and
    /// carry the book's bytes exactly as written.
    ///
    /// Driven over one book whose source carries an escape and a bidi override, so the two planes
    /// are forced apart in the same run: the artifacts must contain the RAW bytes and never the
    /// escaped spelling, and the why surface must contain the escaped spelling and never the raw
    /// bytes. A change that encoded on the way to an artifact fails the first half; a change that
    /// stopped encoding on the way to a terminal fails the second.
    #[test]
    fn display_encoding_never_reaches_the_emitted_artifacts() {
        let raw = "\u{1b}[31m\u{202e}";
        let book = format!("make install # {SOURCE_MARK} {raw}\n");
        let mut interner = Interner::default();
        let parsed = dorc_syntax::parse(&book);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
        let idx = dorc_oracle::KindIndex::default();
        let mut arena = ProvArena::new();
        let classified = dorc_analysis::effect::classify(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &[],
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut interner,
            &mut arena,
        );
        let classes = classified.value;
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg.value,
            &value,
            &classes,
            &BTreeMap::new(),
            &dorc_plan::ConnectedPipes::default(),
            |_, _, _| None,
            |_, _, _| None,
            |_| false,
        );
        let plan = dorc_plan::build_plan(
            &book,
            &parsed.value,
            &cfg.value,
            &classes,
            &dorc_plan::Vouches::new(),
            |_| Observable::verdict_only(Verdict::Unknown),
            &mut arena,
        );
        let framing = dorc_plan::records::Framing::spike("fixture".to_owned());
        let artifacts = format!(
            "{}{}",
            probe.render_sh(&framing, &interner),
            plan.render_apply(&book, &parsed.value)
        );
        assert!(
            artifacts.contains(raw),
            "the byte-floored artifact must carry the book's bytes verbatim: {artifacts:?}"
        );
        assert!(
            !artifacts.contains("\\x1b"),
            "a display encoding reached an emitted artifact — the overlay never becomes the \
             artifact: {artifacts:?}"
        );

        let shown = swept(participating_block(
            &RenderCtx::production(),
            &[1],
            "book.sh",
            &book,
        ));
        let text = shown.text();
        assert!(
            text.contains("\\x1b") && text.contains("\\xe2\\x80\\xae"),
            "the same bytes reach the terminal only in their encoded form: {text:?}"
        );
        assert!(
            !text.contains(raw),
            "raw escape/override bytes reached the terminal: {text:?}"
        );
    }
}

#[cfg(test)]
mod address_tests {
    use super::{address_names_book, parse_line_address};
    /// ack-2 / rul24-lineno-identity: the `dorc why` address parser reads a SOURCE line-number from
    /// `book.sh:N` or bare `N` (the tail after the last `:` when numeric), so a `file:N` the report
    /// PRINTS round-trips to the `N` a query ACCEPTS. A non-numeric tail ⇒ `None` ⇒ content-match.
    #[test]
    fn why_address_parses_line_number_or_falls_to_content() {
        assert_eq!(parse_line_address("book.sh:12"), Some(12), "path:N ⇒ N");
        assert_eq!(parse_line_address("12"), Some(12), "bare N ⇒ N");
        assert_eq!(
            parse_line_address("/abs/path/book.sh:3"),
            Some(3),
            "abs path:N ⇒ N"
        );
        assert_eq!(
            parse_line_address("apt-get"),
            None,
            "non-numeric ⇒ content match"
        );
        assert_eq!(
            parse_line_address("make install"),
            None,
            "content with a space ⇒ content match"
        );
    }

    /// A file-QUALIFIED address must name the book this run analyzed. The render prints qualified
    /// pointers now, so the un-checked reading answers for the analyzed book whatever file the
    /// address named — a silent wrong surface at rc 0, which is the failure this pins shut.
    /// Path-shape tolerance is deliberate: a pasted `./web.sh:9` is the same address as `web.sh:9`.
    #[test]
    fn a_file_qualified_address_must_name_the_analyzed_book() {
        assert!(address_names_book("web.sh:9", "web.sh"));
        assert!(address_names_book("9", "web.sh"), "a bare N names no file");
        assert!(address_names_book("./web.sh:9", "web.sh"), "leading ./");
        assert!(
            address_names_book("/srv/books/web.sh:9", "web.sh"),
            "an absolute path still resolves to the same book"
        );
        assert!(
            address_names_book("web.sh:9", "books\\web.sh"),
            "a windows-separated book path compares on its last component"
        );
        assert!(
            !address_names_book("other.sh:9", "web.sh"),
            "a DIFFERENT book must not silently answer for this one"
        );
    }
}
