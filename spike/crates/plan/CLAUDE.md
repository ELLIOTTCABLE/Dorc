# spike/crates/plan — CLAUDE.md

Role: elision/replacement, the probe→apply compiler, and render — the crate where
irreversible verdicts are MINTED. Owns `PhasedVerdict`/`Bias`, the license
witnesses, `Disposition`/`Step`/`Plan`, `compile_probe`/`ProbePlan`, `build_plan`,
and the renders. Read `spike/CLAUDE.md` first; root `DESIGN.md` (plan/apply UX)
and `IMPLEMENTATION.md` ("To execute, or not to execute?") outrank `Research/`.
Registry discipline: one rule per bullet, slugged; append to the matching section.

## Law — the license mints (the sole minting sites for the two irreversible verbs)

- **sole-mint-witnesses** — `ReplaceLicense` (elide) and `GuardLicense` (guard)
  are private-field, sole-mint witnesses; BOTH demand a
  `core::claim::ByVouch<VerdictVouch>` **by value**. If a mint signature blocks
  your build, you likely lack the real vouch: obtain it (author the
  `is_converged()`) or let the command run — NEVER fabricate or convert a claim
  to satisfy the type. That conversion is the vouchless-elide hole the weld
  closes.
- **aggregate-mints-carry-the-same-demand** — the demand follows the MUTATION,
  not the node shape: an establish erased inside a member-loop or inline-call
  aggregate consumes its own reached vouch exactly as a standalone one does.
  `AllEstablishesVouched` is the private, non-empty (head+tail) proof; its mint
  takes the exact ORDERED `(site, fact)` list and rejects missing, extra,
  duplicate, reordered, wrong-site, and wrong-fact vouches — the whole aggregate,
  atomically. Query-only bodies prove `ReadSubstitutionProof` separately and must
  NEVER manufacture a vouch to share an API. Keep both types and both provers
  private: the doctest in the crate doc pins that, and exposing either "for a
  test" is the regression it guards.
- **vouch-built-once** — vouches are built once (`build_vouches`) and threaded
  via the `Vouches` map; a vouch informs a license and never enters the
  fact-plane.
- **elision-predicate** — elide leaf L iff `probe(L.fact) = Converged ∧ ambient ∧
  Must ∧ no-consumed-unvouched-observable ∧ ¬⊤-contained`. ⊤-containment is a
  SEPARATE guard in `build_plan` — it guards a different failure (unmodeled
  execution context, not a stale fact); keep it separate.
- **cant-probe-cant-elide** — a kind with an effect but no shippable check is
  absent from the probe, so the apply runs it (`kFAIL-perform`). A missing or
  unknown verdict always folds to run.
- **no-rc0-vouch-exists** — "converged ⇒ rc 0" was bought false three times; a
  mutator's rc is ⊤. Never resurrect an establishes-contract rc-0 vouch.
- **consumed-channel-blocking** — consumed `Stdout`/`Stderr` arrive un-collapsed
  and can only BLOCK (`inv-must-may`); consumed Status follows the trichotomy
  (`StatusRelaxable` substitutes an exact known rc, ⊤ blocks; `StatusInvariant`
  never blocks, still recorded, mark-union — any other blocking mark wins;
  `StatusIterated` blocks unconditionally).
- **ship-seam-reads-the-lane-not-the-kind** (`26H` §3.5) — `compile_probe` takes TWO ship
  closures, and the verdict-body one is gated on the caller's per-SITE verdict-lane set:
  never on the fact's kind, never on try-order. It must keep PRECEDING the predict lane —
  a verdict-lane site can also carry a resolvable predict, and shipping that would measure
  a different cell than the record keys. The vouch gate stays on the verdict branch alone
  (the verdict IS the probe, so a declined argv has nothing to measure and must ship no
  record — `guard23-refusepath-rc0-never-passes`).
- **erasure-demands-a-proof-and-a-rendered-death** (`26H` §4 — W-C; `erase.rs` is THE transform
  where this can be wrong, and its tests are the review's centre of gravity) — `prove_dead_branches`
  is the sole mint of a `DeadBranchProof`, and it demands FOUR things, not one: the fold proved the
  site unreachable from a KNOWN controlling status; the site actually gens into reach (the caller's
  invalidator set — never `Disposition`, because omitted-for-any-other-reason is not dead); the site
  is not floored into running anyway (in-loop, ⊤-successor); and the controller will really be
  SUBSTITUTED AWAY. That fourth is the wrong-yes fence — a fold-`Omit`ted leaf whose controller is
  not neutralised renders VERBATIM and runs behind the live guard (`is_neutralised`,
  `omitsafe21-heredoc-guard-keeps-body`), so erasing it would license downstream elisions off a
  mutator the artifact still executes. It is computed from analysis data and the fold input alone,
  through the shared `query_substitutes` seat, so no OUTCOME becomes a premise
  (`pin-no-outcome-as-generator`). Its refusal check is `leaf_has_heredoc` and ONLY that, matching
  `is_neutralised`'s `Replace` arm: `leaf_has_blocking_output_redirect` is the GUARD tier's refusal
  and including it would refuse `cmd >/dev/null 2>&1 || mutator`, the exact ladder idiom the
  fixpoint exists to fix.
- **erasure-is-records-grounded-only** — a statically-known controlling rc (empty list, bare
  assignment, funcdef — all rc 0 in the fold) is SOUND but is not a measurement, and the ledger's
  name promises records. Those branches keep today's behaviour; widening to static deadness is its
  own future design with its own name, never a quiet relaxation of this predicate.

## Law — guards

- **guard-shape** — `( <verdict-fn> <site argv> ) || <original bytes>`; the
  original bytes always survive verbatim; failure lands on run. Declared-dual
  glue is the engine's mechanical sense-flip
  (`( f args; [ $? -eq 1 ] ) || <bytes>`) — lossless inversion.
- **guards-mint-no-values** — a GuardInsert carries no StandIn, no Predicted, no
  Observable: on pass, the check's live rc is the line's rc; on fall-through the
  original runs genuinely. Licensed by the vouch, never by value-provenance.
- **never-synthesized-never-mutating** — never engine-synthesized sh; never
  declared/claimed output in guard position; a body that provably mutates lifts
  nowhere (`271:rul-no-mutating-guards`).
- **check-tax-awareness** — a guarded site pays its check on every apply,
  forever (`KNOBS:kPROBING`): an expensive check must earn its vouch or
  just-run. Planner economics, never a license question.

## Law — the survival tier (the design's ONE naked-trust cell)

- **survive-license** — an elision kept past a RUNNING wall requires: the vouch
  + footprint×backing provably-disjoint (through the core chokepoints only) +
  the admin's `--risk-faultless-skips` (per-invocation, never a default).
  Keying and hint-lane values NEVER feed it; the flag permits acting on
  separation claims, never manufactures them.
- **universal-meet-here** — sparing over backing-SETS quantifies universally:
  any unknown member ⇒ collide, at every iteration, order-independent.
- **attribution-rendered** — every survival renders its full attribution chain
  (whose claim licensed it, line-level first link); the why-lens names them.
- **empty-world-byte-identical** — no oracles loaded ⇒ output byte-identical;
  rung-0 pin in every brief touching this crate.

## Law — render

- **ap-2-runnable** — `render_apply` must emit runnable, `sh -n`-clean POSIX;
  acceptance executes or `-n`-checks the artifact, never text-diffs alone (the
  historical trap: a non-runnable empty `then`-clause shipped green, twice).
- **attention-honesty-here** — the render is the whole book, original order; a
  may-execute line is never hidden (at most dimmed). Span-exact substitution
  (the leaf's byte-span is the edit unit); a leaf the span render cannot safely
  edit (heredoc-carrying) REFUSES its license at render time, loudly.
- **display-only-resolution** — resolving interned tokens to text here is for
  display/provenance only; never branch on the resolved text.

## Direction

- **re-key** — the elision predicate goes per-selector/per-cell at the
  entity-algebra-rebuild; ALL coordinate/selector comparison through core's
  chokepoints, never inline.
- **conditional-tails** (`27C` §5, block-context) — a guarded wall sets a flag
  iff its fallback body actually executed; tail lines conditioned on it keep
  their probe-time elision along the didn't-act branch. Render stays under the
  attention law (any fold is a future human product ruling).
- **wire-records** — probe results move to the `262` §2 records lane at
  block-rebuild (partial deriv-family ⇒ wall-total; additive keys).

## Determinism + precision tests

- **verdict-injected** — `build_plan`/`compile_probe` are pure; the host verdict
  is injected (`verdict_of`); output order is span-sorted, ordered collections
  only.
- **R2-CHANGEDELTA** — "do B because A changed": the author's `changed=1` flag
  is a consumed observable the discipline must PRESERVE, never synthesize.
  Never elide a delta-gated effect via a *state*-probe; never synthesize the
  cross-kind `file:`→`service:` edge. Encode the precision test; don't add
  effect-map dimensions.
