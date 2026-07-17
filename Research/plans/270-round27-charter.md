# 270 — Round-27 charter: the consolidation round (rest-of-round-24 + the wrapper/payload work)

AI-authored (Fable close-out conductor), 2026-07-10; the block arc, its gating
structure, and the stdlib-timing ruling are **human-acked in-session** (typed,
2026-07-10). Plan-tier: keep scanned for currency; annotate, don't rewrite, when
superseded. Companions: `notes/24U` (the round-24 close-out this charter succeeds — read
it first for what-happened-and-why), `notes/24C` (residue ledger), `plans/24S`/`24T`
(the design keystones this round builds), `plans/262` §2 (the wire contract this round
partially imports). Authority: root docs and human-typed rulings outrank this;
`spike/CLAUDE.md` rulings bind every builder.

> **⚠ CHARTER STATUS (2026-07-16):** this charter froze at its 2026-07-10 creation
> while its opens resolved. `block-settle` is CLOSED (last sittings 2026-07-12;
> `plans/271` is the rulings ledger); every §3 design-pass adjudication is
> dispositioned — per-item brackets below. The §2 block-context wrapper and
> read-value paragraphs predate the wrapper-surface redesign (`notes/273`: the
> `24S` §2b four-job context-function dissolved into `cmd__predict()` +
> `cmd__lend_map()`) and context-entry probing (`plans/27C` — THE current
> wrapper/context-transport spec); readers take those first. Task-14 (minted after
> this charter) was DISSOLVED 2026-07-16 and `271:rul-only-oracle-bytes-ship`
> RATIFIED. The live block-rebuild dispatch gate is now `279f:ask-amendment-acks`
> (`notes/279f` §6). §2 block-rebuild's BUILD ORDER itself stands unchanged.

## §0 — Mission

Consolidate every in-flight reimplementation/respell obligation into ONE
dependency-clean arc, then build the wrapper-context and payload machinery — so that
the oracle stdlib is authored exactly once, against the final entity algebra, the final
dialect, and the *real* sudo mechanism; and so the field trial, when it revives,
measures the true product instead of a sudo-blind approximation of it.

The round's ordering (human-acked): **block-settle → block-rebuild → block-context →
block-stdlib**, then the field-trial revival (né r25), then the multi-host resumption
(né r26). r25 and r26 are tabled, not cancelled; their revival conditions are §5.

## §1 — Naming discipline (BINDING on this round's conductors and every brief)

Ruled by the human 2026-07-10; high priority — naming-debt was measurably slowing
velocity.

- Names of things are **hyphenated full-word slugs** (`corpus-respell`,
  `entity-algebra-rebuild`, `adj-capture-claim`) — never opaque letter-number IDs.
- A reference to a slug minted in ANOTHER document carries the minting document's ID as
  a prefix: **`docID:slug`** (e.g. `24C:rul-selector-pre-stdlib` — note the round-ID
  dedupes out of the slug when the prefix already carries it). No prefix inside the
  minting document itself.
- De-naming-debt the corpus **progressively**: when touching an old opaque label (P5,
  queue-3b, 24I-batch3), rename it at point of use and subscript the old label once
  ("né P5"). No central renaming ledger — greps look backwards, not forwards.
  Judgment-bound: balance against conductor-tokens.
- Propagate this section into every subagent brief and successor-conductor handoff.

## §2 — The block arc

### block-settle — the spelling-settlement design-pass (human + conductor; NO builders)

Everything that could re-churn the corpus if decided late, decided FIRST. Contents:

- **The entity-algebra design note** (the `24C:rul-selector-pre-stdlib` obligation):
  the structured entity/selector algebra — 17N recursive-struct vs bounded selectors,
  `24F` §10 alt6 as seed; canonical motivating case `systemctl enable --now`
  (`#enabled` vs `#active` as distinct cells). It MUST reserve the two `24S:A7` seams
  (a context-qualifier slot in the coordinate representation; room in the kind-owner
  surface for per-kind-per-axis topology declarations) — cheap now, retrofit-hostile
  later. Its **authored-spelling half needs the human's ack** before block-rebuild
  dispatches, because it may move the mark grammar the corpus-respell rewrites into.
- **The adjudication agenda** (§3, the design-pass items).
- Output: spellings settled; the `notes/24P` specimens amended conductor-side if the
  mark grammar moved (spec-by-example stays the review vehicle — amend five specimens,
  never re-churn hundreds of files).

### block-rebuild — the reimplementation block (ONE conductor; one worktree lineage)

Human-acked shape: a single owning conductor runs the whole block — staged builder
sub-dispatches are fine (fresh contexts), but under one owner, one branch lineage,
granular commits, so no fixture is rewritten twice and nobody re-onboards. Rationale
(recorded because the previous queue said otherwise): the old respell → floor → rebuild
three-dispatch sequencing predated the entity-algebra de-deferral; its only real
rationale was serialize-the-churn, which single-ownership satisfies while cutting the
fixture sweep from potentially-three passes to one.

Contents, in build order:

1. **corpus-respell** — the corpus-wide dialect churn. Brief is fully assembled:
   `24P` §2 (checklist) + §3 (grammar flags, as deflated by §9b) + §5 (d1–d9, ALL
   acked; d8 typed 2026-07-10) + §7 (is_diverged ditch + one-liner flag rider) + §8
   (rider-comment-budget — STANDING for all future briefs — and rider-shebang) + §9/§9b
   (dorc-sh adjudication + the implementor's bless flow: delete-XFAIL → BLESS on a
   verified binary → conductor inspects against §4's predicted delta classes; the ~11
   bless-artifact cases from §9b get inspect-and-revert-or-accept) + the `24C` deferral
   entries (resp-munge-policy, resp-collision-ship-refusal,
   resid-guard23-stale-comments) + `24C:rul-ditch-is-diverged` + the marker stamp
   (`24C:rul-marker-in-churn`). Rider imported from r26 (`262` §7 handoff): make
   gate-1's record-compare **order-insensitive** during this same golden churn — the
   cheapest moment that will ever exist.
2. **bless-checkpoint-one** — conductor inspects the respell's golden diff against the
   predicted delta classes (updated in block-settle if the algebra moved mark
   spelling). Kept as its own checkpoint deliberately: folding later engine churn into
   the same bless conflates causes and degrades inspection to vibes.
3. **typeless-floor** — the auto-cell + four fences build, from the
   `24C` §24L-gating-errand touch-point map (the verdict-unaware-kernel seam: thread
   verdict-bearing providers into classify AS DATA; the probe-emission fourth
   touch-point). Folded riders: quoted-`"$@"` modeling in the oracle tracer + the
   founding-one-liner e2e pin (`24C:fd-headline-oneliner-gap`);
   `24C:resid-return-arity`; the pipe-guard fixture's improvised auto-cell retires
   here (`24O` item-24).
4. **entity-algebra-rebuild** (né queue-3b) — the re-key: transfer functions ×
   footprint intersection × resolver coordinates (`CanonicalCoord`) × probe keying,
   per the block-settle design note, with the `24S:A7` seams reserved in the
   representation.
5. **value-recipe-reshape** (né 219 tc-fork-ii / `24T:P-A5`) — the fragment-preserving,
   cause-tagged ValueOf/Recipe reshape; first required consumer is the payload work,
   and it also buys the `notes/219` q-2 cause-named ⊤-diagnostics. Slotted HERE, in the
   same fact-domain churn window as the rebuild (this resolves the sequencing `24T`
   left deliberately unspecified: one merge-pain instead of two).
6. **wire-records-v1-import** (human-acked 2026-07-10: pulled forward from r26 because
   reading host data back into the analysis framing is a must for the read-value
   lane). Scope = the `262` §2 contract's bare minimum, single-host, serial:
   framing header/sentinel + per-record terminal token + coordinate-fields
   last-to-token (fixing the incumbent whitespace-truncation bug) + deriv-family
   end-records with partial-family ⇒ wall-total + attempt keying + duplicate
   merge-by-meet + alien/late discard + the additive-keys policy. Acceptance: the
   `262` pins that need no interleaver (terminal determinism at width-1;
   pin-partial-deriv-demotes-to-wall; torn/glued byte-tier rejection through the
   PRODUCTION deframer). Explicitly EXCLUDED (stays r26): waves/width flag, subshell
   isolation, scheduler ports, fleet/transport crates, ssh drivers. Its golden churn
   (framing lines in probe artifacts) rides bless-checkpoint-two.
7. **e2e-degraduation-remainder** (né `24I` batches 3–4) — the ~50 string-asserting
   in-memory twins (`render_corpus.rs` + the one-shot-`dash -n`-per-artifact net + the
   `24C:st-1` `true || true` must-cover) authored ONCE here, at block end, in final
   spelling; the guard23 no-mint floors convert to GuardLicense-absence structural
   asserts.
8. **bless-checkpoint-two** — one inspected re-bless for the engine-side churn
   (floor + algebra + reshape + wire framing), each contributor's predicted delta
   narrow and named in its brief.

Standing fences ride every brief: the `spike/CLAUDE.md` safety block · step-zero
worktree reset + tip-gate · step-one root-docs read · `mise trust` ·
rider-comment-budget (`24P` §8) · foreground final e2e · BLESS exclusivity · granular
`(AI …)` commits · the sonnet no-subagent clamp · §1 naming discipline.

### block-context — the context-and-payload spike

The `24S`/`24T` machinery, staged per their own §8s; implementation-planning for this
block ratifies the payload pins (§3 adj-payload-pins) first.

**[SUPERSEDED IN SURFACE — 2026-07-16, `notes/273` + `plans/27C`: the wrapper stages
below predate the wrapper-surface redesign (the `24S` §2b four-job context-function
is DEAD — `cmd__predict()` + `cmd__lend_map()`; `271:rul-predict-absorbs-wrapper-modeling`,
`271:rul-lend-map`) and context-entry probing (`plans/27C`). Staging intent stands;
surfaces per the new specs (`273` §11 re-reads the W1–W2 staging against them).]**

- **wrapper-peel stage** (né `24S` W1): the context-function role surface (peel + axes
  + ρ-transform + self-vouch/self-footprint) · context regions generalizing the
  `notes/219` q-1.c subst-scope machinery · wrapper/inner node split · identity
  wrappers (nice/nohup) · all floors + diagnostics. Zero new trust surface; proves the
  region machinery; rung-0 regression = wrapper-free corpus goldens byte-stable.
- **wrapper-sudo stage** (né `24S` W2): the user axis + ρ-scrub · stdlib
  user-invariance declarations · probe-outside licensing · whole-ρ replication. The
  value headline (the `24S` §2 walkthrough book elides). **The referendum watch-item**:
  if build contact ever forces a wrapper-aware arm into a TOOL oracle, stop and
  re-audit the kBURDEN story (`24S` §2c) — this is a stop-the-block finding, not a
  workaround site. **[SUPERSEDED — 2026-07-16: whole-ρ replication died into the
  authored predict body (`notes/273` §0/§6; RATIFIED as composed predicts,
  `271:rul-only-oracle-bytes-ship`); probe-outside-via-invariance is re-cut as
  `plans/27C`'s lanes — measurement in the site's denoted context first, the
  invariance-line × `--risk-faultless-skips` fallback second (`27C` §0/§4,
  `271:rul-invariance-speech-act`). The referendum watch-item stands (`273` §11).]**
- **payload-v1** (né `24T` R0): carrier role surface (sh/bash/dash + su's `-c`;
  which-arg-is-code, stdin shapes, argv-binding) · nested parse at analysis time ·
  whole-line fold (elide / guard-conjunction / run) · derived-text locators ·
  the composed acceptance shape `echo data | sudo sh -c 'cat >> /etc/f'` (pipeline ∘
  context ∘ payload — v1's acceptance, not a stretch goal). The basic-forms
  exploration (`24T:P-A3`, ruled) belongs to this block's implementation-planner,
  punt-empowered. R1 (span-edits inside verbatim payload bodies, the
  body-as-a-separate-book reading incl. in-body guards at UNWRAPPED carriers) is the
  reached-for follow-on per the `24T` §4c annotation; the 23J carve (elevated
  wrappers) stays parked.
- **read-value-slice** — the `$(hostname)` capture fold, first slice only
  (`notes/219` q-3's refuse-non-text floor, upgraded to single-LINE by the imported
  wire's coordinate-last-to-token shape): vouched read-only inner command ships as a
  probe, captured single-line stdout folds back as a probe-provenance literal;
  multi-line/binary refuses ⇒ site stays ⊤ ⇒ runs. Gated on adj-capture-claim (§3).
  Authored 262-§2-compatible so the multi-host resumption inherits the record shape
  rather than re-framing it. **[GATE CLOSED — 2026-07-16, `271` task 7: adj-capture-claim
  ruled 2026-07-12; durable = `notes/275` (the value-prediction species). Capture ships
  REAL bytes — byte-consumption demands real execution (`271:rul-only-oracle-bytes-ship`
  rider); world-cell values measure in-context where entered (`plans/27C` §4).]**
- Wrapper stages W3–W4 (sensitive kinds; `env` per-variable refinement) FLOAT behind
  adj-survival-flag-outcome and adj-trichotomy-spelling — in-round tail if the
  adjudications land early, else post-trial. **[Both gates CLOSED — 2026-07-16
  annotation; `271` tasks 8 and 3, see the §3 brackets below.]**

### block-stdlib — stdlib-authoring + yardstick-measurement

- **stdlib-authoring** (né P5; the ~40 bootstrap oracles). Blockers, now all named:
  corpus-respell ✓dialect · typeless-floor ✓license-floor · entity-algebra-rebuild
  ✓coordinates (`24C:rul-selector-pre-stdlib`) · **the wrapper stages ✓sudo-mechanism**
  (human-ruled at close: principled sudo elision via the real mechanism is a stdlib
  precondition — a sudo-blind stdlib mis-measures the felt product, and the
  no-wrapper-awareness referendum must survive build contact before 40 oracles are
  authored against it) · adj-trichotomy-spelling for the kind-owner context
  declarations. Brief additions banked: `24M` §4 (blessed `command -v` polarity;
  UNK-lane stability; kinds/names rules) · the both-shapes teaching (one-liner
  delegation preferred where a genuine read-only dry-run exists;
  dispatch-and-decline for degraded tools; never teach ceremony as quality) · the
  `252` §9 memo-2 quality-bar checklist · read-value coverage (hostname/uname/
  `command -v` — the r25 book's actual command surface) · wrapper-oracles (sudo, su,
  env, nice) and carrier-oracles authored HERE against the proven surfaces · ops
  delegation oracles from `24R` §2d (terraform-plan ternary; the declining
  ansible-check teaching counter-example).
- **yardstick-measurement** (né Stage-6 measure/maximize, tabled since `24O`): maximum
  elision across a strawman family at varying oracle quality, on the finally-stable
  surface; produces the may-alias sensitivity number (`24O` item-25's decision data).

## §3 — The adjudication ledger (each line: what, gate, status)

Design-pass items (block-settle agenda; human-acked 2026-07-10 as the immediate next
activity after this charter lands): **[block-settle CLOSED — 2026-07-16 annotation;
last sittings 2026-07-12; each item's disposition bracketed in place below.
`plans/271` is the ledger.]**

- **adj-entity-algebra** — the design note + the human's ack of its authored spelling.
  Gates block-rebuild dispatch. **[CLOSED — 2026-07-16, `271` tasks 1/2/12: both
  halves ruled 2026-07-10, close-out 2026-07-12; durable = `notes/277`.]**
- **adj-capture-claim** (né `notes/219` fork-capture-claim-type) — is a probe-captured
  stdout an ordinary probe-observation (the reserved `OutClaim` channel finally
  producing) or a NEW claim-type with its own provenance rules? Argued both ways in
  `219` q-5. Gates the read-value-slice; on the field-trial's critical path.
  **[CLOSED — 2026-07-16, `271` task 7 (2026-07-12): the fork dissolved into the
  value-prediction species (`271:rul-value-prediction-species`); durable =
  `notes/275`.]**
- **adj-trichotomy-spelling** (né `24S:A3`) + **adj-axis-vocabulary** (né `24S:A1`) —
  the kind-owner context-declaration spelling is a genuine kOOB-redline reading
  (`24S`'s lean is newly-minted machine-read syntax, justified via DESIGN's
  be-very-not-sh test — but "all config is spelled in sh" deserves a deliberate human
  reading); plus ratifying the versioned-closed engine-owned axis vocabulary
  (v1 = {user, fs-view} + ρ). Gates stdlib context declarations and W3+.
  **[CLOSED — 2026-07-16: the trichotomy DISSOLVED into address-derived topology
  (`271` task 3, 2026-07-11; durable = `notes/272`); the axis vocabulary ratified as
  `271:rul-axis-vocabulary-v1` (fs-view soft-deferred), amended by
  `271:rul-networking-unpunt` (netns ahead of full fs-view).]**
- **adj-survival-flag-outcome** (né `24S:A2`, UNIFIED with the human's TODO
  `--trust-footprints` outcome-rename) — one ruling: the trust flag gates the
  OUTCOME (survival past a running mutation on traveled claims) rather than the
  claim-type, and gets an outcome-centric name (what the admin actually acks:
  unattributable under-execution risk). THE central trust ruling; gates W3, not
  W1–W2. **[CLOSED — 2026-07-16, `271` task 8 (2026-07-12):
  `271:rul-flag-is-razor-residue` (claims own what lines can say; the flag owns what
  no line can say) + `271:rul-flag-named-risk-faultless-skips`
  (`--risk-faultless-skips`).]**
- **adj-small-homes** — the two unhomed TODO items: systemd's 255-as-exit-fail vs our
  ≥2 convention (cheap ruling); the probe-bodies-proved-non-mutable effect-check
  (`24R` §0a marks it owed; proposed home = a typeless-floor or wrapper-peel brief
  rider). **[CLOSED — 2026-07-16, `271` task 10 (2026-07-10): the rc partition stands
  unchanged (`271:rul-rc-partition-stands` + rul-zero-one-inversion-pair; systemd's
  ternary = convergent-evolution validation, not adopted); the effect-check homes as
  a typeless-floor brief rider (`271:rul-effect-check-home-typeless-floor`;
  build-only-if-nearly-free).]**
- **adj-stopping-point** (né `24T:P-A6`) — the announced analyzer ceiling. The old
  "one and only punt: eval" weld is fully unwelded by necessity; the candidate on
  record (gentle-ack only, NOT settled by silence): the boundary is DERIVABLE as the
  composition of three standing lines — const-resolvability (`24T:imp-P1/P2`) ×
  no-escalation (`24S:imp-1`) × no-cross-host (`24S:imp-5`). Fold into the
  design-pass for a typed ratification. **[CLOSED — 2026-07-16,
  `271:rul-stopping-point-unpinned` (typed 2026-07-12): deliberately UNPINNED — no
  ratified composed boundary; the three constituent fences stand individually;
  consumers cite the constituents, never a composed announcement.]**

Later gates (not block-settle):

- **adj-payload-pins** (né `24T:P-A1` formal ratification + `24T:P-A2` the R2 weld:
  refuse-forever vs refuse-for-v1) — at block-context implementation-planning.
  **[Its block-settle pre-read was STRUCK — 2026-07-16, `271` task 11: always
  formally owed at block-context implementation-planning, which keeps sole
  ownership.]**
- **adj-adjudicability-list** (né `24S:A4`) — the machine-read topology clauses +
  binding-smell lints + differential discharge; must land before kinds go
  community-shared; not block-gating. **[ROUTING — 2026-07-16, `271` task-8 close:
  the lint/differential targets were retargeted to the authored inputs (incl. the
  invariance lines). NOT closed; deadline unchanged at kinds-go-community.]**
- Post-trial / human-keyed: co-reference unification (né `24S:A5`, joins
  `24M:rul-kind-unify-owed` + `24C:strain-coreference-crosskind`); the razor
  registration (né `24S:A8`, KNOBS-adjacent, human-owned); kCONC minting
  (`261:dec-mint-kconc`).
- **Punted by the human 2026-07-10** (acknowledged, not scheduled): the root-doc
  respell pass (human maintains the human-authored docs aggressively once spellings
  settle; the conductor cleans the LLM-managed ones on direction) — noting that the
  day the corpus-respell lands, USER_STORY's stage-3 dotted `foobar.is_converged()`
  and stage-5 stringly `touches()` become actively wrong, so this pass wants to sit
  adjacent to bless-checkpoint-one; and the privilege prior-art collation (the
  wrapper research arc's round-2: become/doas ecosystem) — HARD-DEFERRED, opens only
  on a typed human ack; not a W1–W2 gate (`24S` fences privilege semantics out).

## §4 — Explicitly out of scope (fenced, with re-entry pointers)

The field-trial execution and the multi-host build until their §5 revival points ·
fs-view beyond the reserved seams (`24S` §3b's Hard cell — user + ρ prove the machinery
first) · R2 payload rendering (refused pending adj-payload-pins) · general
syntax-position holes, automata carriage, loop-assembly (`24T` §5b fences) ·
cross-host facts (ruling-host-someday-never) · MH2 versioning, kSTATE, secrets, the
language superset + `unsafe` hatch, oracle-author DX tooling (the standing deferred
ledger, `23O` §5, remains authoritative) · per-iteration loop verdicts (atomic-command
axiom).

## §5 — Revival conditions for the tabled rounds

- **field-trial revival** (né r25; branch `ai/spike3-r25`): fires when block-stdlib
  lands. `252`'s protocol stands unchanged. Owed-on-revival (banked in `24U` §6):
  fold `255` §5 → `252` §4/§8; the `_assert_tagged` eyeball; the HHHF zsh smoke-test;
  the lying-check differential must-cover (`24C` §rc-passthrough-taxonomy). The trial
  gets materially stronger than its original design: the book's two permanent walls
  (`su - postgres -c`, the hostname host-guard) are now in-scope machinery.
- **multi-host resumption** (né r26; branch `ai/spike3-r26`, zero build commits):
  resumes after the field trial by rebasing onto the then-current lineage; its S1
  scope shrinks by whatever wire-records-v1-import already landed; `260`/`261`/`262`
  + `26A`'s stop-2/stop-3 amendments remain its spec.
- *(Annotation 2026-07-16, banked here when LIVING_STATUS compressed:)*
  **dotfiles-acceptance-day** (a round-24 settled-list one-liner, 2026-07-07) stays
  TABLED — no block owns it; revives at human discretion (design record for the
  dotfiles positions: `plans/24R`).

## §6 — Conduct notes for this round's conductors

Fable conducts, Opus builds; the block-rebuild conductor owns the whole block
start-to-finish (that is the point); adversarial crosscheck the entity-algebra note and
the wrapper-peel design before building (exclusions-not-inclusions framing); the
`24S` §8 invariants list and `24T` §6 asserted-semantics ledger are test-pinnable
obligations, not prose; every cross-context elision renders its four-link attribution
chain from day one (`24S` §4a). Wrapper-oracle and carrier briefs carry the quality-bar
checklist (`24S:A6` + the `24T:P-A4` carrier items, soft-acked 2026-07-10).
