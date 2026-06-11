# 217 — Wave-1+2 crosscheck reconciliation · #13 harvest · resumption record

> Orchestrator note. Slug 217 was reserved (211 §4) for the wave-1 reconciliation and
> never written before the conductor crash; 21C (reserved for wave-2's) was never used.
> This note folds both waves, the #13 harvest, and the resumption-session verification
> into one reconciliation, per 21Y's queue. Evidence: 21Y + the %TEMP% handoff artifacts
> (13-report.md, inflight-13-uncommitted.diff — byte-identical to the harvested tree) +
> this session's own gate runs and source reads. Reconcile-by-source discipline (21F
> fb-7): every verdict below was re-verified against the cited artifact, not vote-counted.
> Append-only; confidence-marked. AI-run gates/crosschecks are PROCESS EVIDENCE, never
> proof (standing never-vouch rule).

## §1 Wave-1 verdicts (arch-1 render · arch-2 inlining, at their build HEADs)

| target | finding | disposition |
|---|---|---|
| arch-1 (214) | P1: adjacent multi-line span-edit orphan + comment-in-quote corruption — silent over-execute w/ corrupted operand on the plain shape; hard `dash -n` break on the odd-quote variant. Pre-flagged by 214 §9 hunt-7. | FIXED in 21E (`685a61f`): f-1 region-grouping (transitive line-overlap closure, count-invariant tripwire) + f-2 quote-state `comment_safe` hardening (drop the COMMENT never the EDIT). E2e `render21-adjacent-multiline-elides`. |
| arch-1 | No other P1s; churn table (31 text-only / 2 newly-expressible / 1 payoff) inspected by the then-orchestrator. | Stands. |
| arch-2 (216) | Self-crosscheck (216 §6 hunt-6) found the in-loop-floor bypass pre-ship. | Fixed pre-landing (explicit `in_loop_body` check in `inline_disposition` + e2e + unit pin; 216 §7). |

## §2 Wave-2 verdicts (the m-* series; drove task #13)

| id | finding | disposition |
|---|---|---|
| m-2 (P1) | imp-1 stale-guard hole (21F a-2) confirmed LIVE at then-HEAD: book-level `printf >> f` before a grep-guard minted a stale fold. | CLOSED by y-1 (`0c48e07`, note 21H). Orchestrator additionally verified `: > f` and bare `> f` keep the guarded install live; #13 fix-4 pins both shapes as regressions. |
| m-6 (P2) | Dashboard heredoc over-count: a render-refused `Replace` counted `replace-converged` though the artifact runs it (21B hunt-1's own flagged residual). | CLOSED by #13 fix-1: `BlockReason::RenderRefusal`; `build_report` consults `Plan::render_refusal_diagnostics`; demotion sits BEFORE every elision-door arm; unit-pinned (0% not 100% on the heredoc book). |
| m-8 (P2) | Depth-2 transitive inlining broken-but-SAFE, two halves: the positional does NOT thread two levels (inner `$1` ⇒ ⊤ ⇒ body Opaque ⇒ runs — never a wrong elision), and the inner leaf double-counted in the outer site list (`site 0.0` + `site 0.1`). REFUTES 216 §1.2's bounded-iteration claim and §6 hunt-1's verify-item. | CLOSED by #13 fix-2 (`flattened_inner` dedupe; pin: outer list 2→1) + fix-3 (loud refusal `dq-depth-2-positional-unthreaded` for positional-referencing nested calls; literal-arg nested calls still inline; threading deliberately NOT built). 216 carries an IB annotation (§7 below). |
| m-7 | `render21-adjacent-multiline-elides` exec-gate flake. | Root-caused: STALE shared-`target/` binary, not a harness race. Closed; no code change. (BLESS-exclusivity discipline unchanged.) |
| — | arch-2 single-level inlining: attacked, no break found. | SOUND-under-attack — process evidence only. |

## §3 The 216 correction (formal; the append-only record)

216 §1.2 claimed depth-2 positional threading works ("the pass is BOUNDED-iterated … so
an inner-of-inner positional settles once the outer binding lands"); §6 hunt-1 carried
"verify a depth-2 … resolves the deepest install to nginx" as if expected to pass. The
wave-2 crosscheck disproved both: the overlay never bound a second level. Behavior at
HEAD (`60e04f3`): a nested call whose OWN argument words reference a positional
(`$1`–`$9`, `$#`, quoted or `${N}`-braced) REFUSES inlining with the catalogued Note;
literal-argument nested calls still inline; depth-2 positional threading is OUT of the
modeled subset. Soundness was never at risk (the un-bound positional already degraded
the body to ⊤ ⇒ runs); the defect was a false working-claim plus a silent imprecision,
now a loud documented one. IB annotations placed at 216 §1.2 and §6 hunt-1.

## §4 #13 harvest record

- Tree ≡ snapshot: live `git diff` byte-identical to `inflight-13-uncommitted.diff`
  before commit. Committed as `60e04f3` (5 files, +329/−2).
- Gates re-verified by this orchestrator, not relayed: `fmt --check` OK ·
  `clippy --workspace --all-targets -D warnings` clean · `cargo test --workspace`
  448 passed / 0 failed / 1 pre-existing ignored · `sh e2e/run.sh` ×2 = 93/93, six
  gates · `typos` clean.
- Notes quiesce `a6c7c53` (21K, 21Y, 21Xa). This note + amendments follow.

## §5 Resumer's review observations (line-review of the #13 diff; hunt-fodder)

- **obs-1 — RESOLVED, no gap.** Concern: `b "${1}"` might slip fix-3's refusal if `${1}`
  lexed as `ParamComplex`. Verified against `lexer.rs::lex_braced_param`: an
  all-alphanumeric brace body lowers to `Param{name}` — `${1}` IS matched by
  `word_parts_reference_positional`; the refusal covers the brace spelling. +SURE.
  Residual (acceptable): operator-forms carrying positionals (`${1:-x}`) are
  `ParamComplex` ⇒ unmatched ⇒ the OLD silent-but-safe path (⊤ ⇒ runs, no Note) —
  consistent with `ParamComplex`'s general engine-wide opacity; not worth a special case.
- **obs-2 — load-bearing ordering assumption, unpinned.** fix-2's dedupe is correct
  because an inner CALL's node-id precedes its spliced body's node-ids and the site scan
  ascends (`flattened_inner` accumulates before the direct hits arrive). Holds by
  construction at depth ≤ 2 (depth-3 is budget-refused before the question arises) but
  no assert pins it. A future change to lowering order or the depth budget must re-check;
  a cheap debug_assert (inner-call id < each flattened id) is a candidate hardening.
  ~SUSPECT worth doing opportunistically, not worth a dedicated slice.
- **obs-3 — span-coincidence bridge in fix-1.** `render_refused_leaves` recovers LeafIds
  by matching refusal-diagnostic byte-spans against step AST spans. Sound today under
  `inv-leaf-seam` span-disjointness; unmatched diagnostics drop defensively. Failure
  mode if spans ever stop identifying leaves: the demotion silently misses and the
  heredoc over-count RETURNS — exactly the lie fix-1 kills. Queue for the next dashboard
  crosscheck to attack, and fold into the seam-1 wishlist (a public keyed per-site
  readout would replace the bridge outright).

## §6 tc-* resolutions (orchestrator-grade; human may overrule)

- **tc-fix3-severity → RESOLVED: keep the catalogued Note this round.** The catalog
  carries a tested Note-only severity invariant (21H); honoring catalog discipline
  (21G §3 rq-1/rq-3) outranks severity-matching the pre-catalog `cfg-inline-refused`
  warnings. The inconsistency is real and recorded: one refusal-class is a Note among
  warning siblings. The right eventual shape is per-code severity DECLARED IN the
  catalog, which belongs to the round-22 catalog retrofit (21G §2 layer-1), where
  `cfg-inline-refused` itself migrates in.
- **detached-copy asymmetry → recorded, untouched stands.** The funcdef-definition's
  detached body splices a positional inner call without the refusal (empty inline
  stack) — a non-leaf unreachable island, never consulted for plan/probe; the reachable
  copy refuses. Editing `lower_funcdef` for zero behavioral delta was judged risky by
  the builder; concur. ~SUSPECT a one-line doc-comment at `lower_funcdef` would
  pre-empt future confusion; fold into any future touch of that function.
- **Carried unchanged:** tc-door1-door3-composition's d×d cell (crosscheck-pending),
  tc-exec-nonzero-exit (`EXIT_RC=` marker — task #12 owns it), tc-provenance-string-
  coverage, tc-heredoc-diagnostic-boundary, tc-multiline-original-flatten.

## §7 IB amendments applied with this note (the user-ratified annotation discipline:
genuine incorrectness likely to mislead a grepping agent; `<!-- /*` form, never prose
rewrites)

- 216 §1.2 + §6 hunt-1 — the depth-2 refutation (§3 above).
- 21B §6 hunt-1 — "Deferred" render-refusal demotion is DONE (#13 fix-1); prevents
  re-implementation.
- 221 header NB — "21D does not exist / was never written" was true at drafting, since
  overtaken (arch-7 landed as 21D, `8d87e15`).
- 220/221/222 H1 titles — corrected from their provisional drafting slugs (21H/21I/21J)
  to the filed 22x slugs, each with a provenance annotation preserving the old slug for
  grep; 220's collision with the REAL 21H build note was the trip-hazard.

## §8 Queue after this note (unchanged from 21Y, next first)

Task #7 arch-5 (entry-gate findings stand: global-pristine confirmed in code, 21K d-6;
re-scope under expanded obligations + dedicated hostile pass) → task #12 harness pass →
task #5 door-4/door-2/precedence-seam LAST behind default-OFF flag → round-close report
+ plans/21Z living spike-4 inventory + seeding feedback.
