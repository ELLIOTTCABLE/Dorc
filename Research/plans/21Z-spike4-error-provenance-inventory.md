# 21Z — LIVING inventory: error/provenance primitives at round-21 close

> The round-close obligation from 21G §5: which round-11 error/provenance primitives
> (plans/111, notes/110) are already-welded vs embryonic vs absent in the spike, as the
> seeding document for the round-22 lean (21K d-1: errors + provenance, including the
> derivation-dump/`why` direction and light OTel). LIVING: update statuses in place as
> rounds land (this file is the one sanctioned exception to append-only, scoped to the
> status/evidence columns; provenance of each update stays in the git history).
> Evidence basis: a dedicated read-only survey agent (round-21 close session,
> 2026-06-11) with file:line verification at HEAD `e512cc3`, re-verified in spots by the
> orchestrator. AI-surveyed; process evidence, never proof (never-vouch). Line numbers
> drift — anchor by symbol name.
>
> Top-line corrections to 21G §5's pre-classification (it predates the 21H build):
> the DiagCode catalog physically EXISTS now (built-and-green embryonic, 5 codes), and
> the catalog has FIVE codes not four (`dq-depth-2-positional-unthreaded` was added by
> the #13 wave-2 fix after 21H was written).

## The inventory

### welded (load-bearing, codebase-wide; r22 builds ON these)

- **Carrier&lt;T&gt; / never-throw (inv-no-throw).** `core/src/lib.rs` `Carrier{value, diags}`
  (writer-monad shape: pure/map/and_then/push/has_errors); every pipeline stage returns
  it (parse → cfg::build → value::analyze → classify → lift → plan), threaded in
  `cli/main.rs` via `report(stage, &diags)`. Coverage honesty: untrusted-input
  front-ends are clean (parser anti-stall + totality test; cfg MAX_DEPTH → ⊤); the
  one latent sharp edge is `Interner::resolve`'s un-bounds-checked index
  (`#![expect]`-ratcheted, self-minted symbols only) — harden if provenance values ever
  flow through foreign interners. r22 need: formalize "every give-up path spells
  `Carrier::push(Diagnostic::error(..))`" as the layer-1 gate's target.
- **⊤-poison cascade (the dataflow half).** `effect.rs` `Reach::{Facts,Top}` with
  absorbing join; `plan` `has_top_successor` blocks licenses at three gates. BUT the
  DIAGNOSTIC half of "report only root-cause" is ABSENT (see below): each ⊤ site
  Notes independently; nothing links a poisoned site to its ⊤ origin — `Reach::Top`
  is causally opaque (does not carry WHICH command made it ⊤). That link IS the
  derivation DAG's job.
- **Span origin-handles.** `core` `BytePos`/`Span`; parser sets every AST node's span;
  `Diagnostic.span: Option<Span>`. LEAKY AT BOTH ENDS (verified, extends 21H hunt-6):
  drop-A — `cli report()` never renders the span (computed-then-ignored at the only
  user surface); drop-B — the NEW catalog Notes are span-POORER than the old scattered
  codes (3 of 5 pass `None`: `command_effect` is called with `None` on the production
  path, classify holds the CFG not the AST), while every older code (`cfg-*`,
  `oracle-*`, `render-heredoc-refused`) carries `Some(span)`. The newest
  error-handling code regressed span coverage relative to its older siblings — the
  inversion to fix first (gated on the same classify-signature widening as 21H's
  deferred s-2).

### embryonic (exists, narrow; r22 widens)

- **DiagCode catalog + completeness test (the layer-1 embryo).** `core/src/diag.rs`:
  per-code const + template() arm + structured-param constructor + `CATALOG` registry
  (5 entries) + `fill()` + `severity()`; tests: every-registered-code-has-template
  (the rq-2 Pottier embryo), all-Note-severity, constructors-fill-templates.
  **Coverage honesty — the load-bearing inversion (survey note-A):** the catalog is a
  Note-only island; all 17 SCATTERED codes bypass it (`syntax-unsupported`,
  `syntax-malformed`, `check-out-of-dialect`, `check-unterminated`, 8× `oracle-*`,
  `cfg-top-node`, `cfg-errexit-unknown`, `cfg-inline-refused` [6 distinct free-text
  emit sites], `effect-kind-disagreement`, `render-heredoc-refused` [inline literal,
  not even a named const]) — and the `error[`-severity codes that actually trip
  gate-3 are ALL outside the catalog. Layer-1's ambition ("every give-up path carries
  a slugged catalog code") targets exactly the population the catalog does not yet
  cover. r22: retrofit the 17 (mechanical, no behavior change), then the real
  path-enumeration gate ([A-pottier-reachability-2016]); per-code DECLARED severity
  migrates in at the same time (217 §6's tc-fix3-severity disposition). ALSO decide:
  `hostsim/differential.rs` `Finding{class, diagnosis: String}` is a SECOND, fully
  separate diagnostic vocabulary (DST-judge products, free-text, no DiagCode) —
  in-catalog or formally out (survey note-D).
- **Artifact provenance comments.** ONE emitter post-arch-1: `render::apply::
  provenance_comment(originals)` (+ probe-side `site_comment`/`unresolvable_comment`).
  Discloses elided originals `;`-joined, newline-flattened. **Genuine provenance hole
  (survey note-C, verified):** when `comment_safe` refuses (open quote / trailing-`\` /
  `<<`), the comment is dropped and the code's own doc-comments claim "the OOB verdict
  lane still carries the disclosure" — IT DOES NOT (the lane carries verdicts+rc keyed
  by site, not original-source-text; the disclosure goes to stderr at best). The
  dropped-provenance text currently has NO carrier. r22: give it one (a structured
  record on the lane, or an own-line leading comment — 21E res-3 deferred the latter
  for golden churn); and to feed a DAG, the emitter needs `(Span, FactKey, LicenseVia)`
  handles, not pre-rendered strings.
- **OOB verdict lane.** Real wire, naming caveat (survey flag-4): the literal
  `$DORC_VERDICT` env var does NOT exist — the implemented lane is the probe artifact's
  stdout record protocol (`site <key> effect=<verdict> rc=<n> [stdout= stderr=]`),
  emitted by `render::probe::record_scaffold`, parsed by `cli parse_results` into
  site-keyed `SiteRecord`s (`site N.M` member keying); rc gated by the wrong-concrete
  firewall (only valid-Query rcs feed the fold). `stdout=`/`stderr=` slots are
  plumbed-but-dead (nothing emits values — the arch-4 q-3 future). Diagnostics travel
  on STDERR, a different channel — do not conflate (the provenance-comment hole above
  did exactly that). r22: the lane's record grammar is where a provenance field rides
  if receipts become typed; `inv-site-keyed-results` is the natural anchor.

### absent (r22 builds from scratch; nearest seams named)

- **Derivation DAG.** Nearest seam: `plan` `Derivation{fact, via, ambient, grade,
  verdict}` — a flat ONE-TIER record per license (plan/CLAUDE.md names it "the
  degenerate one-tier case"), constructed only in `prove_replaceable`. Not a linked
  DAG; no located-nodes/typed-edges; `Reach::Top` carries no origin. r22 shape: grow
  Derivation into the `loc-*` multi-locator list (110/111's N-tier, per-host-forking
  direction), make ⊤ carry its originating site (which also unlocks diagnostic
  root-cause dedup), and thread it into the provenance-comment emitter + the lane.
  The 21K d-1 derivation-DUMP mode (one durable log per run; `why` becomes a query
  over it; golden-TRACE fixtures for critical-tier DST) consumes exactly this.
  <!-- /* corrected 2026-06-11 (round-22, ru-16 need-4 + ru-19): the golden-TRACE
  half of d-1 was adjudicated AGAINST (no trace-pinning; verdicts everywhere); the
  dump+`why` half stands and is arch-4. See 22A concl-9 + 224 ru-19 dist-1/dist-2. */ -->
- **Catalog-completeness as a real gate.** The existing test asserts
  registered⇒templated (the trivial direction); NOTHING asserts give-up-path⇒registered
  (the Pottier direction). No build-script/CI mechanism enforces catalog membership;
  the pre-commit gate set has no catalog check. r22: the path-enumeration gate, after
  the 17-code retrofit.
- **Provenance-typed user surfaces.** `Diagnostic.message` and `provenance_comment`'s
  input are bare `String`; no provenance-carrying newtype exists anywhere
  (`OutClaim(Symbol)` is the closest "text the engine won't decode" wrapper, no chain).
  r22 shape (21G §2's make-bad-states-unrepresentable approximation): a `UserText`
  constructible only from provenance-carrying values; retype `Diagnostic::message` and
  the comment emitter first; the no-raw-text lint falls out.

## Round-21 additions to the r22 worklist (from this close's crosschecks; cite-on-pickup)

- The span-bridge family (217 §5 obs-3; two convergent hostile passes at close): the
  dashboard's render-refusal demotion rides an unchecked span-coincidence bridge —
  tier-1 count-tripwire built at close (B3); tier-2 keyed readout (~5-line
  `Plan::render_refused_leaves -> BTreeSet<LeafId>` — the diagnostics loop already
  holds `step.leaf` and throws it away) and tier-3 single-source-of-truth refusal
  (derive `collect_edits`' drops and the diagnostics from ONE decision point — the
  only fix for the unmirrored-second-refusal-class hole, which is cli-dark and
  tripwire-blind) are r22's, recommended together whenever `plan` is next touched.
- Consumer-provenance tags (`StatusConsumer::{ErrexitImplicit, BranchOperand, DollarQ,
  IfGuard, LoopCond}`, 218 §5 / 218a ps-2): required by the doors program's precedence
  ladder, and independently useful as provenance (who reads a status is exactly a
  derivation edge).
- Diagnostic root-cause dedup (the ⊤-cascade's missing diagnostic half, above) — the
  fail-fast rule's "only root-cause must be reported" has no machinery; volume is
  already flagged (21H hunt-5's per-⊤-operand Note multiplication).
