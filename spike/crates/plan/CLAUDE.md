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
- **ship-seam-reads-the-lane-not-the-kind** (`26H` §3.5; `28Q` §4
  rul-verdict-primacy-at-the-ship-seat, BUILT at stage-0, 2026-08-16) — `compile_probe` takes TWO ship
  closures, and the verdict-body one is gated on the caller's per-SITE verdict-lane set:
  never on the fact's kind, never on try-order. Every vouched, mutation-capable site is
  verdict-lane: the verdict body IS the probe check, its own reached answer is the
  convergence measurement, and it PRECEDES the predict lane — shipping a resolvable
  predict there would measure a different cell than the record keys, and prediction never
  licenses elision. A predict ships alone only where elision is statically unavailable;
  its cells feed the static concern topology whatever ships. The vouch gate stays on the
  verdict branch alone (a declined argv has nothing to measure and must ship no
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
- **pinned-definitions-are-the-artifact's-binding** (`28K` §4
  `rul-runtime-resolution-never-load-bearing`) — `Plan::pinned_definitions` decides, for the whole
  artifact, which body each guard invokes and under what name; the render consults it and nothing
  re-derives a binding at runtime. A misalignment there could swap WHOSE judgment executes, which is
  pope-sin tier (`271:rul-sin-ordering`), so it is structural rather than three mechanisms agreeing.
  Since the emission stage: the SNAPSHOT (helper declarations) hoists ONCE, deduped by
  declaration site, ahead of the role bodies; a vouch carries `closure` and `body` separately
  and a body owns only its own bytes — which makes the munge a header-only edit by
  construction, and made ALREADY-IN-PLACE answerable (pre-split, the blob comparison never
  matched and a copy of the BOOK's own body hoisted above the book — corpus-reached, pinned
  twice). Whole-artifact DEFENSIVE emission (every emitted name munges) triggers on real
  definition vectors only (`oracle/CLAUDE.md a-top-reject-is-not-a-definition-vector`) —
  and per `spike/CLAUDE.md rul-happy-path-is-a-closed-set`, every idiomatic tier above the
  defensive floor is licensed only by PROVEN enumeration, never assumption.
  Three rules in order: CONTENT-DEDUP (byte-identical bodies are one definition); ALREADY-IN-PLACE
  (a body the book's own text defines at top level with the same bytes is not copied — the
  EMITTED preamble never carries two same-named funcdefs, which is what dissolves the
  `reserved.rs` tension; a regional BOOK definition may share a preamble body's name, because
  book bytes are untouchable and sh scoping binds the region to its own body —
  `frame30-subshell-body-answers-inside-only` is the pinned case (`308:cr-artifact-two-funcdefs-letter`);
  the positional regime guarantees a book-sited definition PRECEDES its guards, so nothing
  is re-derived); BARE-IF-SINGLETON (over emitted material ∪ the book's top-level funcdef
  names); HASH-MUNGE (two distinct bodies under one name each emit once as
  `<name>_h<digest>`, digest over the definition BYTES). The retired dedup-by-funcname emitted the
  first body and let both sites invoke it. A munged name cannot parse as a `__role`, so a
  re-ingested artifact reads the guard as an opaque call ⇒ conservative run (`23A:P-reingest`).
  `Plan::render_sh` is the flat DST render and emits no preamble at all — never wire a second
  binding authority there.
- **the-pinned-unit-includes-the-closure** — what a guard's preamble ships is the stripped definition
  PLUS `dorc_oracle::closure`'s prefix (helpers + file-level constants). Two riders: a CONTESTED
  closure withholds the VOUCH (no guard, no elide, the site runs), and the vouch's `check_cmds` —
  the dual-rail `guardcmd` allowlist — must cover the closure's own commands, because once a helper
  travels with the definition the real check-command lives in the helper.
- **check-tax-awareness** — a guarded site pays its check on every apply,
  forever (`KNOBS:kPROBING`): an expensive check must earn its vouch or
  just-run. Planner economics, never a license question.
- **certifier-trip-cleanup-runs-in-every-driver** (`302:rul-certifier-trip-guard-only`)
  — `certifier_trip::demote_on_trip` runs immediately after `build_plan_walled` in
  EVERY plan-producing driver; a NEW driver MUST call it. On a tripped run,
  Replace and Omit demote to run (`DemoteTag::CertifierTripped`, a reason-enum arm,
  never a sibling code); guards STAND only on the syntactic occupancy-1 census over
  `DefinitionTable` — the census consults NO solve, and that independence IS its
  admissibility (a trip disqualifies solver and certifier together; nothing either
  touched may testify). Runs run; the mid-pipeline §3 floors are untouched and
  still fire in place; no recovery, no carves, no re-planning.

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
- **an-at-most-claim-has-two-atomicities** (`28P:dec-whole-body-atomic-refusal`) — a derived
  footprint must prove BOTH that its record stream arrived whole AND that the body which wrote
  it finished. They are independent and only the first reads like a gate: `deriv-end n=<K>` is
  counted by the SCAFFOLD from lines received, so a body that emits three coordinates and dies
  on an unbound helper closes at `n=3` and agrees with itself — transport intact, survey false,
  at-most claim wrongly NARROW, and narrow SPARES MORE (measured wrong-elision). So the
  scaffold captures the emitting body's status BEFORE the record pipe (a pipeline's status is
  its RHS's, which is why the body's was previously unreachable) and carries it as
  `body-rc=<R>`; non-zero refuses the WHOLE family ⇒ the site walls total. Never file a
  body-death under the transport code `deriv-family-incomplete` — the stream was perfect, and
  saying otherwise mis-attributes (`271:rul-sin-ordering`). Two things this is NOT: a verdict
  rc (`rul-rc-partition` binds verdict functions; this is a binary did-the-body-finish, spelled
  `body-rc=` so it can never be read as the site record's `rc=`), and a completion signal —
  a body that truncates and exits 0 stays invisible, and that residue is human-owned design
  (`ANALYZER-NEEDS:an-atmost-completion-signal`). Do not build toward it.
- **rederivation-is-demote-only** (r30; `notes/300` §2b) — before a plan ships, every
  survival re-derives through the naive reference model (`dorc-sparing-reference`) via
  `plan::rederive`, seated INSIDE `wall_walk_survival`'s Survived arm (a post-pass
  demote would let a now-running site cast no wall downstream). The minted
  `SurvivalWitness` goes in BY VALUE and comes back `Confirmed(witness) | Demoted(..)`
  — the re-check cannot fabricate a witness, agreement licenses nothing new, and the
  adapter never touches the production compare path (zero shared helpers; lexically
  fenced both ways). The differential's one disclosed coverage limit: the backing-side
  dialect-membership conjunct is adapter-computed, not model-re-derived
  (`sparing_differential.rs` header).

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
