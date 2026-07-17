# 27D — block-rebuild landings ledger (accretes per stage; the `24C` role for round 27)

AI-authored (Fable conductor, r27-impl session, begun 2026-07-17). One section per
landed `270:block-rebuild` stage: what landed, the conductor's checkpoint dispositions,
and the residue/handoff riders later stages own. Authority: root docs and human-typed
rulings outrank this; `plans/271`/`notes/277` are the specs the stages build against.
Append-only per stage; never rewrite a landed section.

## stage-1: corpus-respell — LANDED 2026-07-17 (bless-checkpoint-one STAMPED)

Commits `1112526..4f0d89d` on `ai/r27-corpus-respell`, fast-forwarded into
`ai/spike3-r27` (the r27 live lineage — supersedes `ai/spike3-r23` as the forward
branch). Builder: Opus lead + clamped Sonnet sweeps. Scope delivered: the dorc-lang
v0.1 corpus churn (153 oracle fixtures; `#`-selector marks; `:`/`:!`/`:?` sigil
family; `disturbs`/`disturbance_reaches_only` renames; `sm.dorc.*` re-keys;
`is_diverged` + `VerdictSense` hard-deleted per `24C:rul24-ditch-is-diverged`;
marker stamps + `#!/usr/bin/env dorc-sh` shebangs; comment-rip 185.7k → 20.1k bytes
against the ≤38.4k budget; forward-munge keying for KIND-keyed roles;
`Word::PositionalDefault` for the `${2-}` idiom; gate-1 compare verified already
order-insensitive per `262` §7).

**Conductor verification (own hand):** ancestry 8-ahead/0-behind; fresh
build + 636 unit tests + e2e 126/126 zero-xfail re-run green on the folded lineage;
leftover greps EMPTY across all six pattern classes (dotted role-defs,
touches/reaches defs, is_diverged, stringly emission, dot-selectors in marks,
unmunged kinds). Bless diff inspected by `24P` §4 delta-class census: label re-keys
(bulk), comment-echo shrinkage (rec-1 flow from the rip), ±newline bless-artifacts,
lax-order re-captures (builder verified identical multisets). Spot-read:
`285e30c` (the forward-munge `rekey_to_raw_kinds` bridge — the sharpest fix; the
respell had silently un-fired resolvers/reaches for dotted kinds and the first
bless masked it), the flagship golden, the nounset golden.

**Checkpoint dispositions (conductor-adjudicated):**

- disposition-books-comment-rip — ACCEPTED. The brief's "books byte-untouched"
  collided with the budget arithmetic (the human's own baseline counted books;
  books alone were 90k > the 38.4k target). Book comments ripped; golden echoes
  shrank accordingly. Surfaced to the human as flag-worthy; goldens churn freely
  per standing ruling.
- disposition-nounset-pin-strengthened — ACCEPTED. `guard23-nounset-book-survives`
  changed behavior: the `${2-}` sweep made the shipped guard body set-u-safe, so
  the converged-past-wall curl site now GUARDS-and-short-circuits (live
  `dpkg-query` re-check) instead of crash-falling-through. The 23C-fd2 pin is
  strengthened, not lost: guard-under-`set -u` now works rather than merely
  failing safe. (Builder's report said "elides" — imprecise; it is a guard
  short-circuit; the wall stands.)
- disposition-state-stored-only-in-inert — ACCEPTED with a stage-2 caution rider:
  the member parses (marks read as opaque Establish-shaped marks) but nothing
  consumes it; stage 2's auto-cell/classify work must not accidentally hand those
  marks live Establish semantics.
- disposition-sharefile-book-no-marker — ACCEPTED.
  `guard23-reingest-collision-verbatim`'s book is a stripped off-ramp artifact:
  marker-free is correct; the `reserved-namespace-squat` warning pin retained.

**Handoff riders (owner assigned):**

- rider-marker-enforcement-deferred → stage 2 (part B): markers stamped but the
  engine still treats them as comments; enforcement (dialect-in-unmarked = loud
  error) deferred because ~100 inline-source unit tests carry unmarked dialect.
- rider-command-keyed-backward-munge → stage 2 (part C): command-keyed roles kept
  the backward `_`→`-` un-munge, violating `24C:rul24-totalistic-munge`
  (a literal-underscore book word like `my_tool` silently finds no oracle).
  Bounded debt, corpus-absent today, must die before the stdlib.
- rider-dorc-sh-unbuilt → stage 6 (e2e-degraduation): the `dorc-sh` thin bin +
  whole-file `dorc strip` do NOT exist (only per-funcdef strip); the stamped
  shebangs are inert in e2e. Human steer recorded 2026-07-17: keep the strip's
  shebang-runner rewrite an extremely simple plain-text substitution
  (`dorc-sh`→`sh`-ish, per the `24Q` adjudication).
- rider-selector-charset-unenforced → stage 3: `277` §4b's loud-diagnostic on
  charset-violating selectors is not built (fragments stay opaque).
- rider-brace-alternation-unexpanded → stage 3: `#{a,b}` parses as one opaque
  token; per-token expansion wires when the claim algebra consumes it.
- rider-resolver-coverage-watch → stage 3: the `rekey_to_raw_kinds` bridge keys
  off kinds collected from establish/query backings + wall-candidate footprints;
  any lookup path outside that set silently misses resolvers — the
  entity-algebra-rebuild must subsume the bridge into the re-keyed coordinate
  machinery, not inherit it blind.
- rider-internal-rust-naming-debt → stage 3, in passing: `touches`/`reaches`
  survive as internal module/type names (`lift_touches`, `TouchesSet`),
  documented at each site; rename as the re-key churns those modules.
- finding-one-liner-candidates (for the stdlib quality-bar's both-shapes
  teaching): flagged-never-converted passthrough-shaped verdicts at
  strawman24-alias-symlink (writeconf), five service.oracle.sh copies
  (systemctl is-enabled delegation), strawman24-reach-static-service (enablesvc).

## stage-2: typeless-floor — LANDED 2026-07-17 (conductor checkpoint PASSED)

Commits `296643a..ea77549` on `ai/r27-typeless-floor`, fast-forwarded into
`ai/spike3-r27`. Builder: Opus solo. Full landing record: **`notes/27E`** (the
builder's note — auto-cell mechanism, kernel seam shape, tc-flags, deferrals).
645 unit / 128 e2e green; ZERO existing-golden churn (two new exemplar cases
only). Conductor spot-checks: `auto_or_opaque` mint conditions (concrete argv
only, no-check-resolved only, mapped-provider keyed) and the
`survival::disjoint` auto-kind may-touch fence — both per-spec, slug-cited.

**Checkpoint dispositions (conductor-adjudicated):**

- disposition-marker-gate-home — tc-marker-gate-home CONFIRMED as built: the
  dedicated cli-edge pass is the right home (kernel/lift purity; the gate fires
  at the user boundary; the ~100 unmarked-dialect unit tests exercise lift
  MECHANICS, which is test-scope, not product-scope). The lift-level+stamp
  alternative is retired.
- disposition-set-e-floor-limitation — tc-set-e-blocks-auto-elide ACCEPTED as a
  principled product limitation (inv-probe-sourced-values: the markless floor
  cannot fabricate the book command's rc, so under `set -e` only the guard tier
  is reachable; marked verdicts declare converged-rc and elide). RECORD FOR THE
  STDLIB TEACHING DOCS: "the bare floor under `set -eu` buys guards, not
  elisions — the first mark buys the elide back."
- disposition-positional-literal-model — the founding one-liner licenses because
  the lexer routes `$@` to `Word::Literal("$@")` — a WRONG MODEL whose outcomes
  are caught by rc-authority (the shipped body is authored bytes; the probe's
  real rc is the verdict; wrong-concrete static reads decline⇒run). Accepted for
  now, NOT blessed as design: rider-positional-modeling-hardening assigned to
  stage value-recipe-reshape (route bare `$@`/`$*`/`"$*"` to ⊤ per
  inv-top-reject; model quoted `"$@"` as concrete positionals per
  `24C:fd-headline-oneliner-gap`; the e2e pin must stay green across the change).
- disposition-effect-check-punt — ACCEPTED (the typed sizing rider licensed
  exactly this: not nearly-free ⇒ punt, zero guilt). The seam now exists;
  re-homing decision deferred to conductor discretion post-block.
- disposition-invited-rooms-pass-forward — ACCEPTED; the compile-failure pin
  obligation travels to whichever stage first mints a hint-lane/license-lane
  type split (likely block-context's descend-don't-license types).

**Handoff (owner assigned):**

- rider-raw-ship-repair-with-otelcol → stage-2b (dedicated dispatch, next): the
  `24J` raw-ship repair (`271:rul-only-oracle-bytes-ship` + three riders) plus
  the COUPLED A6 otelcol-boilerplate retirement; build surface per `27E`
  (`connected_check_pipes` + `compile_probe` connected path + `render_sh`).
  Also carries the `279f` §5 connected-probe riders: SIGPIPE-flap as a named
  nondeterminism class (why-lane note on rc-141 sink landings; hostsim race
  injection; any future `--exit-code` computes from divergence-of-world, never
  raw sink-landings).
- rider-positional-modeling-hardening → stage value-recipe-reshape (above).

## stage-2b: raw-ship repair (composed predicts) — LANDED 2026-07-17 (checkpoint PASSED)

Commits `51b3706..d61b531` on `ai/r27-raw-ship-repair`, fast-forwarded into
`ai/spike3-r27`. Builder: Opus solo. Landing note: **`notes/27F`** (coverage-rule
shape for stage-4; tc-flags). The `24J` standing-law debt is REPAIRED:
connected probes ship composed predicts only (`271:rul-only-oracle-bytes-ship`),
argv-through-argparse, machine-pinned by
`composed_probe_renders_predicts_never_raw_book_bytes` + two refusal pins + five
`273` §2 classification pins. SIGPIPE riders landed (hostsim seeded race
injection + DST; why-lane note on rc-141 sink landings; exit-code contract
recorded at the sink site). 128/128 e2e; conductor inspected the changed-shape
probe artifact (composed form verbatim, delegation-produced bytes).

**Checkpoint dispositions:**

- disposition-a6-scope-expansion — ACCEPTED: converting `if-form` (beyond the
  brief's named case) was FORCED by the coverage rule itself (its `>/dev/null`
  non-last stage declines the consumed channel ⇒ refuse ⇒ elide→run regression);
  the conversion restores the elision honestly via delegation. The rule firing
  against the brief's own scope is validation, not drift. `unvouched-mid`
  consistency-converted, still walled by its `cat`.
- disposition-last-stage-rc-honesty — the governing stage's stdout-gate
  exemption is sound (only its rc is consumed); a printf-constant-rc arm
  carrying a verdict mark would be AUTHORED wrongness (oracles-may-lie,
  attributed to the marked line), not engine fabrication. BANKED as a stdlib
  quality-bar lint candidate: "verdict/observe mark on a constant-rc line."
- disposition-multi-command-arm-grading — tc-last-reached-command-coverage
  ACCEPTED as safe-direction interim: classification decides substitution
  grading only (real authored bytes always flow at probe runtime);
  subsumed when stage-4's per-channel Observable production lands.
- tc-stage-ship-triplication — inherited debt (mirrors ship_predict_body);
  cleanup candidate for e2e-degraduation or block-context planning, not urgent.

## stage-3: entity-algebra-rebuild — LANDED 2026-07-17 (checkpoint PASSED)

Commits `c519d52..a94d96f` on `ai/r27-entity-algebra`, fast-forwarded into
`ai/spike3-r27`. Builder: Opus solo. Landing note: **`notes/27G`**. The `277`
re-key landed: both chokepoints in `core::coord` (`selector_covers` +
ternary `compare` + `selector_identifies`), the context slot
(`Context::HostDefault`, unminted), the amended sparing algebra with the
`279f` regression pins, per-(family × kind) dialect sets with safe-floor
provenance recovery (`sole_family`; ambiguous ⇒ None ⇒ collide), survival
routed through `compare`, emission `KIND#SELECTOR` lifted end-to-end
(a latent `split_mark_target` gap fixed), charset loud-reject + brace
expansion. 679 unit (+43 pins) / 128 e2e; ZERO golden churn (the re-key is
dormant until an oracle authors a selector-bearing disturbs mark —
empty-world-byte-identical holding by construction).

**Checkpoint dispositions:**

- disposition-context-slot-home — tc-context-slot-on-coord-not-factkey
  ACCEPTED for v1 (nothing populates the slot; fact-plane keying untouched).
  NAMED SEAM ⇒ block-context planning: when contexts become real, fact keying
  (FactKey and its 47-site map) must be revisited — two same-cell facts in
  different contexts must not collide in the fact plane.
- disposition-relation-same-misnomer — tc-same-is-overlap-not-identity: the
  routing is CORRECT today (`Same`→Poisoned for sparing; transport-direction
  gated on `selector_identifies`; no transport consumer exists), but the
  variant name is a trap. RIDER to stage-4: rename `Relation::Same` to an
  overlap-honest name; the consumer-map doc-comment states transport-same is
  `selector_identifies`-gated, never the overlap variant.
- disposition-brace-verdict-silent — tc-brace-verdict-silent-skip REJECTED
  as-landed: a verdict/observe brace minting nothing SILENTLY is the
  silent-inert authored-construct class (24Kc F2). RIDER to stage-4: loud
  diagnostic + test.
- disposition-backing-family-recovery — tc-backing-family-via-dialect-
  reverse-lookup ACCEPTED as interim (ambiguity falls to the safe floor,
  pinned); the value-recipe-reshape's per-channel backing derivation should
  thread the minting line/family properly rather than inherit the reverse
  lookup.
- disposition-resolutions-in-plan — ACCEPTED (core = pure comparison, plan =
  license machinery; the CanonicalCoord MINT's continuity is what the ruling
  protects, and it survived). Crate-CLAUDE.md wording alignment rides the
  next steering-doc refresh.
- rider-internal-rust-naming-debt — still open (builder deferred as
  high-churn cosmetic); reassigned to stage-6 as an optional tail item.

## stage-4: value-recipe-reshape — LANDED 2026-07-17 (checkpoint PASSED)

Commits `fd8225e..9fe885c` on `ai/r27-value-recipe`, fast-forwarded into
`ai/spike3-r27`. Builder: Opus solo. Landing note: **`notes/27H`** (the
foreclosure walk + residue). Landed: cause-tagged fragment-preserving
ValueOf/Recipe (`TopCause`; every ⊤ names its why); the `ValueGrade` lattice
(⊤ < AuthorComposed < WorldSpoken < Register < ProgramText — ProgramText the
non-prediction top, delivering seam-literal-provenance as representation);
weakest-fragment provenance derivation (`argv_word_grades`); `OutClaim` →
`OutBytes`; `Relation::Same` → `Overlaps` (+ consumer-map doc); brace-on-
verdict loud reject; q-2 cause-named cmdsub-⊤ disclosure; value-plane
positional hardening (`$@`/`$*`/`"$@"`/`"$*"` → Top(UnresolvablePositional) in
book flow). ZERO golden churn; 128/128 e2e unblessed. Foreclosure walk: all
six `219` capture-chain steps representation-open.

**Checkpoint dispositions:**

- disposition-positional-split — ACCEPTED. The oracle-predict-parser side
  stays the wrong-Literal model (rc-authority-caught): doing it right needs a
  position-aware `Word::PositionalArgs` across both evaluators, and the
  parser cannot even represent quoted-vs-bare today. ASSIGNED to
  block-context's wrapper-peel lane — command-position `"$@"` modeling IS
  peel detection (`273`), the natural rebuild site. `27H`
  finding-positional-oracle-side-couples-founding-pin is the analysis of
  record; the founding-one-liner e2e pin guards the transition.
- disposition-effect-plane-deferral — ACCEPTED AS CAUTION, not as a drop:
  observe-backing-widening PRODUCTION + `Backing`→SET + survival
  universal-meet migration + minting-family threading is minted as
  **stage-4b**, a dedicated dispatch INSIDE block-rebuild (the naked-trust
  survival cell deserves a fresh-context builder, and the churn belongs in
  bless-checkpoint-two's window, not block-context). Non-dormant: the corpus
  carries `:?` marks, so widening may demote survivals — each demotion is
  correctness-improving and must be individually named for checkpoint
  inspection.
- Carried tc-flags (context-slot/factkey · resolutions-in-plan ·
  backing-family-reverse-lookup) roll into stage-4b + the block-context
  planning seam list.

## stage-4b: effect-plane backing-SETS — LANDED 2026-07-17 (checkpoint PASSED)

Commits `e4c6da9..cecaa13` on `ai/r27-backing-sets`, fast-forwarded into
`ai/spike3-r27`. Builder: Opus solo. Landing note: **`notes/27I`**. Landed:
`FactBacking` (family + observed-selector set) threaded classify→plan;
`Backing`→SET with `widened`/`of_fact`; observe-backing-widening PRODUCING
(per-(provider,verb) linkage: co-occurring `:?` selectors widen the arm's
verdict facts; standalone observes stay Queries); survival universal-meet on
real sets (both `27G` pins re-pinned non-synthetically; fence-no-disjoint
member-wise); minting-family threading authoritative (reverse-lookup demoted
to the inert map-miss floor). 691 unit / 128 e2e; ZERO golden churn — dormancy
robust, not coincidental (verbless corpus observes; ⊤ corpus disturbs).

**Checkpoint dispositions:**

- disposition-widen-not-also-queries — ACCEPTED (an oracle-internal read;
  the mark asserts one thing, consumers derive — `275` §2; keeping the
  Queries cell would regress the establish to MustRun).
- disposition-arm-wide-widening — ACCEPTED (kill-surface only grows).
- disposition-minting-line-deferral — ACCEPTED as corpus-inert TODAY, but
  NAMED PRECONDITION: attribution-at-sparing-verdict requires the minting
  LINE (source span) once sparing can actually fire — MUST land before
  block-stdlib mints selector-bearing disturbs claims. Owner: block-context
  planning's seam list (or an early stdlib-block errand).
- limitation-backing-sets-same-kind-only — NAMED vs the `275` §2 design: the
  derive model forces every (provider,verb) cell to one (kind,entity), so
  backing sets today hold sibling SELECTORS only — the cross-kind backing
  member (the decorated-output clock∪state case) is unrepresentable until the
  derive model widens. Not corpus-observable; block-context's
  read-value-slice / per-channel backing work owns the widening.

## Human rulings received mid-block (2026-07-17, typed in-session)

- **rul-payload-pins-near-weld** (HUMAN-TYPED 2026-07-17): the human reviewed
  `270:adj-payload-pins` thoroughly; nothing new worth noting; grading verbatim:
  "very close to weld, but not quite." Consequence: block-context proceeds on
  the `24T` pins as reviewed; the formal packet is still assembled at
  block-context implementation-planning for the record, but the
  conservative-proceed posture is authorized — the payload-v1 lane does not
  stall on the human's presence.
- **correction-strip-not-gsub-tier** (HUMAN-TYPED 2026-07-17, superseding the
  same-day chat steer recorded under stage-1's rider-dorc-sh-unbuilt): "the
  strip *cannot* be gsub-tier simplicity" — the earlier
  simple-plain-text-substitution recollection is retracted as a mistake.
  Authority: main's updated DESIGN.md (human-authored): `dorc-strip` is a
  "focused, simple, `ubi`-friendly standalone-binary cleaner to remove any/all
  non-POSIX Dorc annotations" — parser-backed erasure shipped as its own
  focused binary, never a text-substitution pass. The stage-6 builder was
  corrected mid-run (E1); the strip-is-pure-erasure constraints were already
  binding and are unchanged.
- **observation-book-acceptance-carve-in-human-voice** (conductor-banked from
  the same main root-doc update): DESIGN.md's "POSIX sh" section now carries
  the human's own firming of the kWHICHSH open half — dorc-lang files =
  POSIX-superset weld (stripping/reuse/cross-shell sharing); book-code = "as
  unconstrained as we can afford," aspiring to partial zsh/bashism tolerance.
  The TODO-ADDTL book-acceptance-carve item now has human-voice direction;
  bears on block-context/stdlib framing and any future book-tolerance work,
  not on block-rebuild. (IMPLEMENTATION.md also fixed the three→four outcomes
  enumeration — an old doc-queue item discharged.)
- **rul-synthesized-payload-render-stays-unwelded** (né "R2"; HUMAN-TYPED
  2026-07-17; slug conductor-minted per `270` §1 — the mechanism: rendering
  `sh -c "$SOMECONSTANT"` via value-propagation into an engine-synthesized
  payload string): the refuse-forever-vs-refuse-for-v1 fork is ruled
  **deliberately just-barely-unwelded** — refused at v1, never welded shut.
  Rationale verbatim: undecidable right now — "an *ocean* of downsides to
  doing it, but a completely unknown-size-of-upside"; "we won't know if we
  need it until we discover that we need it." Build consequences: payload-v1
  refuses synthesized-payload-render; NO machinery, lint, or representation
  choice may foreclose a future un-refusal (the door-open obligation is
  load-bearing for the payload-v1 brief); the re-entry trigger is discovered
  need, not a scheduled revisit.

## stage-5: wire-records-v1-import — LANDED 2026-07-17 (checkpoint PASSED)

Commits `52542cc..97854fe` on `ai/r27-wire-records`, folded into
`ai/spike3-r27` (fold note: the fast-forward briefly orphaned the lineage's
payload-rulings ledger commit, recovered by cherry-pick at `1b841ca`;
conductor protocol now checks behind-count before every fold). Builder: Opus
solo (survived two transient 529 interruptions, resumed with context intact).
Landed: the full `dorc-records/1` framing contract per `262` §2 — production
deframer in `plan::records`, terminal token `@@dorc@@`, last-to-token
free-content fields (whitespace-truncation bug fixed), deriv-family
end-records with the SAFETY INVERSION honored (partial at-most family ⇒
wall-total), attempt keying, merge-by-meet, additive-keys, alien/late
discipline. All four acceptance pins + the `279f` stdout-rider round-trip pin
+ byte-tier fault DST (seeded torn/glued/oversize through the PRODUCTION
deframer). 708 unit / 128 e2e; golden delta = framing lines only (apply
artifacts verified byte-identical) + one lax re-capture. r26's S1 scope
shrinks to everything-not-in-§2 (width flag, waves, subshell isolation,
ports).

**Checkpoint dispositions:**

- disposition-legacy-deframe-tolerance — tc-legacy-deframe-tolerance is a REAL
  production hole as landed (a headerless stream on the real read path folds
  leniently instead of refusing — a truncated-before-header artifact would
  bypass every integrity key). RIDER to stage 6 (errand E4): gate the lenient
  headerless regime behind an explicit harness/test-only escape; the
  production read path becomes strict-only. Fixtures stay unframed (the
  churn-avoidance disclosure stands).
- disposition-book-digest-fnv — ACCEPTED for the spike (the `book=` key
  detects accidental stream/book skew, not adversaries — hostile-host rides
  the parked `kSTATE` coupling; rec-5 forbids cross-run re-ingest so a later
  algorithm swap is cheap). NAMED for r26/real-executor: adopt sha256 at the
  cli edge (never a kernel dep) when the shape ships beyond the spike.
- disposition-fixed-spike-nonce — ACCEPTED (single-process spike; the
  re-mint/zombie-discard MECHANISM is DST-tested with varying nonces; real
  nonces arrive with the real executor).
- `TrustedFootprints::contains` pub promotion (read-only, for the soundness
  pin) — accepted.

## stage-6: e2e-degraduation, first slice — LANDED 2026-07-17 (checkpoint PASSED; stage-6b continuation minted)

Commits `13a8e51..29d0202` on `ai/r27-e2e-degraduation`, folded into
`ai/spike3-r27` (second behind-count-1 fold; the strip-correction ledger
commit recovered by cherry-pick at `6adf472` — conductor now separates the
count-check from the fold command). Builder: Opus (honest partial delivery —
front-loaded the errands + the load-bearing net, deferred the bulk migration
unstarted rather than half-landing). Landed: **E1** dorc-sh + whole-file
`dorc strip` (parser-backed erasure per correction-strip-not-gsub-tier,
acknowledged in-run; bare-mark whole-statement deletion pinned; smoke e2e) ·
**E2** trap-at-tip walls-loudly pin (in-memory home; discharges the `271`
task-12 conductor errand) · **E4** strict-deframe production path
(`LegacyPolicy` at the cli edge; `DORC_ALLOW_LEGACY_RESULTS` harness escape) ·
the `render_corpus.rs` twin tier with the MANDATORY dash-n net wired into the
harness itself + proof-it-fires test + the `24C:st-1` must-cover · an 11-case
migration slice (13 twins; kill-then-install ≡ exec-same-cell-kill collapsed,
twin STRENGTHENED to a converged-host pin). e2e 128 → 117, deletions-only
churn, no bless needed; 27 suites green.

**Checkpoint dispositions:**

- disposition-strip-keeps-marker — ACCEPTED as landed, human-veto invited: the
  `# dorc-lang/v0.1` marker survives stripping. Consistent with main-DESIGN's
  own wording ("remove any/all NON-POSIX Dorc annotations" — the marker is a
  valid POSIX comment) and harmless on re-ingest (marker-gates-syntax-only; a
  stripped file has no dialect constructs). The opposite call (strip it — the
  file is no longer dialect text) is defensible; one-word human ruling welcome,
  low stakes, reversible.
- disposition-dorc-sh-spawn-not-exec — ACCEPTED (Windows-portable spawn-and-
  wait; disclosed; true exec is a real-executor-era nicety).
- disposition-partial-delivery — ACCEPTED; the remainder (batch-3's ~40
  remaining cases incl. all ~SUSPECT rows, batch-4 guard23 conversions,
  optional batch-5 + E3) is pattern-established mechanical work ⇒ minted as
  **stage-6b**, dispatched immediately (Opus lead + clamped Sonnets per the
  established twin pattern).

**Environment incident (surfaced to human 2026-07-17):** a SyncThing conflict file
(`effect.sync-conflict-…-PHNHRER.rs`) appeared INSIDE the stage-1 builder's agent
worktree mid-edit — device PHNHRER is live-syncing `.claude/worktrees/`, which the
standing exclusion intent says it must not. Inert this time (untracked, not
compiled); a real mid-edit corruption risk for future parallel agent work.
Cleanup + `.stignore` repair are human-owned.

## Ruling addendum (2026-07-17, human-typed)

- **rul-strip-erases-marker** (HUMAN-TYPED: "Strip it… very low priority"):
  supersedes disposition-strip-keeps-marker — `dorc strip` erases the
  `# dorc-lang/v0.1` marker line; a stripped file is no longer dialect text.
  Routed as errand E5 to the running stage-6b builder (mid-run addendum).

## BLOCK CLOSE — bless-checkpoint-two STAMPED 2026-07-17; block-rebuild COMPLETE

Eight dispatches (corpus-respell · typeless-floor · raw-ship-repair ·
entity-algebra-rebuild · value-recipe-reshape · backing-SETS · e2e-degraduation
× 2), every one landed, checkpointed, and folded; lineage `ai/spike3-r27` tip at
block close = the commit carrying this entry. Conductor's own block-close
verification: fresh build, 27 test suites green, 76/76 e2e, four-gates-clean
tips throughout. Cumulative golden census `4f0d89d..close` fully attributed:
stage-5 framing rewrite pairs (predict-invocation bytes unchanged — verified
byte-level) · stage-2b composed-predict reshapes (inspected at its checkpoint) ·
new exemplar cases (stages 2/6) · migration deletions (117→76; stages 6/6b) ·
accepted lax re-captures. No unexplained delta class; no bless was performed at
this checkpoint (nothing needed re-blessing — all churn was already isolated,
attributed, and inspected per-stage).

**Rides forward to block-context planning** (the consolidated seam list):
fact-plane context keying (né tc-context-slot-on-coord-not-factkey) ·
oracle-side positional model (`"$@"` PositionalArgs across both evaluators; the
founding-pin guard) · minting-LINE threading before stdlib mints
selector-bearing disturbs · cross-kind backing members (derive-model widening)
· the invited-rooms compile-failure pin (first hint/license type split) ·
effect-check re-homing (punted, seam built) · tc-stage-ship-triplication
cleanup · batch-5 + E3 tails (optional, unowned) · rul-payload-pins-near-weld +
rul-synthesized-payload-render-stays-unwelded (human-typed, governing
payload-v1) · the book-acceptance carve now in human voice (main DESIGN.md).

## lane-wrapper-peel (block-context lane 1) — LANDED 2026-07-17 (checkpoint PASSED)

Commits (rebased across the merge of main) topped by `e26b448` on
`ai/r27-wrapper-peel`, ff-folded into `ai/spike3-r27`. Landing note:
**`notes/27K`**. Landed MODELS-only, zero new trust: the positional model
(`Word::PositionalArgs`; quoted-`"$@"` faithful, bare forms Unmodeled; both
founding pins green) · `oracle::wrapper` (`detect_peel` command-position
tautology; the `RhoClaim` ladder with the `274` §12 riders; the closed
`Dimension` set; `cmd__lend_map` lift under the enumerate-every-dimension law;
`InnerContext` population design) · dual-peel coherence plan-time fail-fast.
Rung-0: 76 pre-existing e2e cases BYTE-STABLE; 80/80 total. The REFERENDUM did
not fire. Disclosed gap: the bare prefix-assignment ρ rung (`VAR=x "$@"`) is
unmodeled (parser splits it script-scoped) — rides lane-context-entry.

**Post-27Xf conductor repairs applied same-day:** `277` §2 consumer-map
transport-row annotation + §5 non-emptiness side-invariants
(inv-backing-set-non-empty · inv-top-never-empty-set, ratifying stage-4b's
in-build shape); `27C` §3 gains the pointwise lend/ρ composition algebra
(⊤ propagates; nested-permutation pins). The human struck `is_diverged` from
steering + ANALYZER-NEEDS and handled the hazard quarantine (27Xn ignored by
their ruling); main merged into the lineage at `c19a4f5` by their hand.

**Still gating lane-context-entry (human-owed, asked):** the 27Xf Tier-1
structural carried-by ruling (recommendation on record: keying-only at v1) ·
the `27C` §1 authority predicate-vs-rule sentence. Lane-payload-v1 is NOT
gated and dispatches next; its brief carries the negative is_diverged
acceptance pin (neither reserved nor recognized) as a rider.

## Ruling addendum (2026-07-17, human-typed — the authority axes)

- **27C:rul-two-axis-escalation-consent** (HUMAN-TYPED, applied to `27C` §1 in
  place): mechanical-capability and oracle-escalation-consent are ORTHOGONAL
  axes. Axis 1 = can the probe-bootstrap mechanically acquire its setup
  privileges (capability test, never identity; NOPASSWD non-root counts;
  acquisition mechanisms are open user-established infrastructure; the probe
  never self-acquires mid-run). Axis 2 = admin consent to apply escalation to
  fallible oracle code ("escalation" = any permission/access-changing
  machinery, sudo-upgrade and su-sidegrade inclusive) — DEFAULTS TO YES via
  the double-ended ack; `--no-probe-escalation` is the opt-out. The old
  "a non-root connection performs none of them" sentence was a drafting
  artifact, corrected. Discharges `27Xf:cr-27C-1-authority-predicate-
  contradicts-rule`; lane-context-entry's gate-2 CLEARED.
- **27Xf Tier-1 status**: `27Xf:cr-structural-carried-by-transports-a-
  measurement` is UNDER RE-ADJUDICATION in the human's separate design session
  ("there's unsoundness in the report" — treat `27Xf` §3 Tier-1 as
  under-review, act on nothing from it until the human's adjudication lands).
  Consequence: the `27C` §4 fallback lane is CARVED OUT of lane-context-entry's
  scope entirely (primary entry lane + dial + vouch + guards only; no fallback
  lane, no carried-by transport, no invariance-line consumption built this
  lane) — lane-context-entry is therefore UNGATED and dispatches after
  lane-payload-v1 folds.

## lane-payload-v1 (block-context lane 3, dispatched ahead) — LANDED 2026-07-17 (checkpoint PASSED)

Commits (rebased) topped by the `27L` landing-note commit on `ai/r27-payload-v1`,
ff-folded. Landing note: **`notes/27L`**. Models-only, zero new trust, rung-0
byte-stable (80 untouched, +2 honest-wall cases; 82/82). Landed: the `dorc:sh`
three-spelling recognition + row-3 strip pin · invited-rooms type split
(`core::room`, sealed; `into_license_input()` exists only on Invited; the `279f`
§5 compile-failure pin is a real compile_fail doctest) · eval'er reentry
detection · payload decomposition with the accept frontier at
const_literal_text (interpolation ⇒ REFUSE-⊤ with cause; site-local degrade) ·
whole-line fold {elide, guard-conjunction, run} · the per-run shim model ·
negative is_diverged pin. The synthesized-payload-render door honored
STRUCTURALLY (no payload bytes re-serialized; un-refusal = one additive
LineFold variant) per `27D:rul-synthesized-payload-render-stays-unwelded`.

**Dispositions:** tc-room-tag-on-fact-vs-factkey → joins the fact-keying seam
family (context slot + room tag), OWNED by lane-context-entry's FactKey
decision · tc-book-bare-sh-room-source → wiring-correctness note for whoever
makes payload facts real (the room sources from detect_evaler's head, never
the book head); rides the reentry-execution follow-on · `24T` §6 L1/L2/L4/L5/L7
differential obligations TRANSFERRED (need reentry execution; probe-shipping
follow-on) · the two build-surfaced facts (predict-dialect while-argparse gap;
dorc-sh Windows-unmockable) recorded as fixture constraints.

## The Tier-1 adjudication is IN (2026-07-17, human's session, arrived by merge)

`27C` §0's fallback lane is REWRITTEN at the source: sub-lane (a) =
**pure-predicate carry** — unflagged, substrate axes only, requiring BOTH the
kind-owner's `invariant:<axis>` line AND an engine-proved READ-SET-CLOSED
verdict body (everything influencing the verdict traces to site argv or a
marked read; the engine closes the "and nothing else", so no human at-most
claim remains ⇒ no flag). Sub-lane (b) = ingredient/identity axes stay behind
`--risk-faultless-skips`. Supersedes the old engine-warranted carried-by row
AND this ledger's interim keying-only carve-out. Lane replan: the fallback
lane becomes its own follow-on lane (**lane-fallback-carry**, after
read-value-slice — the read-set-closure proof is real machinery sharing bones
with the punted effect-check); lane-context-entry stays primary-entry-only and
dispatches now.

## Ruling addendum (2026-07-17, human-typed — the probe-mutation ownership weld)

- **27C:rul-probe-mutation-ownership-split** (WELDED; applied to `27C` §3 +
  spike/CLAUDE.md; discharges `27Xf:cr-entry-self-effects-carve-is-AI-asserted`;
  the running lane-context-entry builder notified — excision-isolation lifted):
  the probe-never-mutates law allocates by OWNERSHIP. Authored/oracle code
  (stdlib included) = the uncontrolled ocean ⇒ the loud frontloaded contract
  stands; an entry-form's self-effects (sudo auth-log line, timestamp refresh)
  are the AUTHOR's vouched residue, claimed by the authoring speech-act,
  attributed to their line — if the community someday rejects probe log-lines,
  the at-fault party is a human author, orthogonal to Dorc. Engine-generated
  constructs = OWNED ⇒ no hard line; judgment/UX tier, careful-and-necessary
  against the flag-derived user-story (whom-we're-writing-for is a function of
  the active flags). The conductor's "no state you care about changes" framing
  is STRUCK (referent-agnostic violation + a phantom user-instruction);
  kFAIL's KNOBS prose is untouched — this ruling allocates responsibility
  within the weld, it does not move the weld.

## Mid-run redirect (2026-07-17 — lane-context-entry, from the human's in-flight 27C revision)

The human's root working-dir carries an uncommitted `27C` §3 revision that
SUPERSEDES the conductor's interim composition-algebra block. Redirect issued
to the running builder with the ruled cells inline:
`27C:rul-top-absorbs-absolute-maps` (⊤ propagates uniformly; NO
overwrite-rescue through inner absolute maps — machine-state logic never
skips the middle) · `27C:rul-dimension-owned-compose-ops` (compose ops +
value frames engine-internal, per-dimension: user=absolute overwrite,
fs-view=caller-relative path composition; authored surface stays single-step)
· cross-link ρ-threading (the one dimension that threads through every link's
argv-resolution) · canonical context key = the folded per-dimension NORMAL
FORM, never chain syntax (nice-position chains share a key; env-vs-sudo order
does not) · fold-entry coherence stays conductor-proposed/strawman · the DST
pin list. Full text arrives with the human's commit; this entry is the
pointer, not the content. Also noted from the same root read: the fallback
lane's settle commit (`c55ffbe`) RETIRES the old engine-warranted carried-by
row explicitly and names the conservative-closure pass "the spike's
obligation to discharge and prove in practice."
