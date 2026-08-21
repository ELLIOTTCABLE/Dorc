# 30Ng — the artifact-semantics adjudication sitting (human-typed 2026-08-21)

> Tier: adjudication record. The human ruled on five conductor items IN CHAT, with the
> explicit caveat that each ack binds what they READ in the conversation, not necessarily
> the document text — reconciliation notes below account for any gap. Distillations are
> faithful-to-typed-substance; where I paraphrase, I mark it. Conductor: Fable,
> second-half conduct (`notes/30N`).

## §1 — `rul-region-refusal-discloses-region-keyed`: ACKED

Attribution points at the line containing the renderer-refused span. No gap between the
chat description and the as-built (`30Ne`'s region-keyed axis + diagnostic).

**RIDER, human-typed, NOT a ruling — `attn-render-refusal-feeds-the-spine` ("needs
attention"):** render-refusal nacks a foundational assumption the human held — that the
decision spine is frozen at a fixation point between analysis and plan-emission, with
emission a static view over it. "I won't elide this because of required syntax" is a
mutative-difference-causing DECISION in exactly the analytic sense, and must feed BACK
from the render layer into the spine. Their lean (typed): a second rerun-to-fixpoint
stage that keeps the spine pure — the render can REFUSE, but never change a decision; it
can only MINT new ones, and then the entire cycle repeats. Explicitly not ruled this
instant; queued for a sitting.

Reconciliation note (conductor): the as-built is PART-way there already — render
feasibility participates in settlement (`only-a-proof-retires-a-wall`: a Replace the
render will refuse walls), and `Plan::decided` takes every render answer once — but the
refusal predicates run as settlement INPUTS rather than as a minted-decision species
cycling to fixpoint. The gap between that and the leaned shape is the sitting's subject.

## §2 — `rul-license-mints-at-settlement-from-shared-conclusion`: ACKED, with a directive

Acked and agreed. **DIRECTIVE (typed): the narrative must carry the ENTIRE DAG (or at
least tree) of causative contributors** — in the `install_pkg` example, every
invocation's evidence, not a sample.

Reconciliation gap (conductor, owed): as-built, `SpineRegionDecision.routes` is a
k-CAPPED `Account` and a route whose invocation cannot be keyed to a leaf is filtered
SILENTLY (the `30Nd` meaning-audit narrowed its doc to say so). Under this directive the
cap may stand only if truncation is LOUD and the full contributor set remains reachable
on the pull/why plane; the silent filter must go. Queued as
`work-region-routes-account-loud-and-complete` (rides the next lane that churns the
seat).

## §3 — `rul-census-inputs-are-non-optional`: ACKED

"Basically always pro-make-poor-states-unrepresentable." No gap.

## §4 — the artifact-stream reading: NACKED → `rul-piped-stdout-carries-a-full-plan`

The conductor endorsement (`--artifact-dir` moves the artifact off stdout, piped stdout
proceeds) is REVERSED. Typed substance:

- stdin/stdout channels must have PRINCIPLED, CONSTANT meanings — for `plan`, under ALL
  flag-forms and statuses.
- The two knowns: (1) the user asked for a plan; (2) a non-interactive stdout means they
  want a meaningful output stream. The only principled position: produce a FULL plan —
  fully bundled into a single transport stream (the same artifact mode needed for hosts
  where a directory cannot be created) — and put it on that stream.
- FAIL-FAST when that is incoherent with other options or derived limitations: "if we
  can't do what we're pretty sure the user asked for coherently at all, we stop."
- **FRAMING (typed, follow-up):** the piped `plan` output is a REVIEW SURFACE. A
  non-interactive stdout means the user is piping the plan out to review it some OTHER
  way — pipe-to-less/editor/GUI are the design-centric examples; pipe-to-ssh is never
  recorded as supported, exemplary, or design-centric (unpreventable, but not designed
  for). The design-centric flow when relying on pipes: plan → review surface, then
  reviewed-and-tuned plan → piped back to `apply`. (This strengthens the full-bundle
  requirement rather than relaxing it: the reviewed stream must be the true, complete
  artifact — what you approved is exactly what executes.)

Consequences (conductor derivation, veto-eligible where marked):

- The stream posture derives from stdout INTERACTIVITY again — the probe is an
  edge input behind a DI seam (injected/mockable; hermeticity is preserved by injection,
  not by absence).
- A piped `dorc plan` over a book whose loads cannot yet be fully bundled into one
  stream REFUSES pre-network rather than emitting a plan that cannot run where it lands.
  This makes the single-stream bundling (and eventually the book-inlining lowering)
  LOAD-BEARING, not a value-add.
- [PROPOSED, veto-eligible] piped stdout + `--artifact-dir` together = the incoherence
  case (two competing complete artifacts) ⇒ refuse; interactive stdout +
  `--artifact-dir` = tree to the directory, render to the terminal.

## §5 — the multipart mirroring: RE-RULED → `rul-bundle-at-dorc-lang-boundaries`

The conductor endorsement (mirror everything at authored paths; zero rewrites) is
SUPERSEDED as the default. Typed substance:

- Generated plans are durables but NOT off-ramp durables. Dorc reserves the right to
  rewrite, specifically and exclusively, IMPORTS in generated plans — rewriting a `.`
  line to import the correct bundle, or with the corrected path.
- The default bundles at exactly the transitive-dep-graph points where dependencies
  become dorc-lang. Dorc holds MORE rights (and can act more safely) to
  modify/rearrange/combine/compile dorc-lang and non-mutative probes than arbitrary
  book sh.
- Worked example (typed): `book.sh → book_special_case.sh (book) → some_oracle.sh
  (dorc-lang) → whatever.sh`; DEFAULT flattens `whatever.sh` + `some_oracle.sh` into
  `some_oracle.dorc-bundle.sh` and emits the two book-code files as a coherent plan
  (naming TBD — plausibly `book.plan.sh` + `book_special_case.plan.sh`; semantics ruled
  here, never specific CLI flags).
- BOTH extremes fully supported: a `--flatten`-shaped mode producing ONE emission (the
  same mode a redirect causes) with oracles bundled AND book code inlined, possibly
  per-host (TBD); and a `--no-flatten`-shaped mode emitting fully-remapped trees.
- Bundle-points must be an explicitly TUNABLE axis; the default may sit at NEITHER
  extreme. CLI specifics stay malleable — do not lock flag names, file names, or
  defaults tightly.

Reconciliation notes (conductor):

- The as-built mirroring survives as machinery INSIDE the new default (dependencies
  that stay separate files still place by load-cwd relativization; the placement-defect
  repair and its cells stand). What changes: book-reached dorc-lang subgraphs bundle by
  default, and the imports that reach them are REWRITTEN in the generated plan.
- This largely discharges the burndown's `rule-book-sourcing-artifact` question in the
  rewrite-imports direction, and re-prices the book-INLINING half of `--flatten` (still
  gated on inlining soundness; the minted `floor30-inline-dot-boundary` measurement is
  its evidence base; dorc-lang bundling proceeds first under the more-rights grant).
- `two-surfaces`' byte-floor: the authored BOOK remains byte-floored where it appears;
  the license just granted is scoped to GENERATED plans' import lines and dorc-lang
  material. Steering must say this narrowly.

## §6 — The single-stream review surface is a core product TENSION, not a bug (typed follow-up)

Typed substance (2026-08-21):

- The fully-collapsed single-stream bundle, though explicitly a review surface, is a
  LESS-THAN-IDEAL one — a core product tension that cannot be resolved, not a bug to fix.
  Using it is the human EXPLICITLY requesting one fully-reviewable artifact for some
  reason of their own.
- It sucks for the attention model: the non-mutative oracle ocean cannot be hidden from
  the end-user in one stream. The day-one buy-in to Dorc is the admin choosing to
  review "the small bit of code I wrote and KNOW is mutative, deciding which known-
  mutative bits run today" while agreeing that "the much larger body of supposedly-
  non-mutative code is something I may review occasionally but probably won't". The
  single-stream request is, inherently, backing slightly out of that choice — asking to
  have the vaguely-related oracle ocean placed in-stream between the user and their
  task. Dorc respects the request and still does its best toward the attention goal.
- Takeaway (typed, details TBD, do not over-encode): the single-stream output STILL
  uses the bundling and lifting mechanics — lifting/hoisting, munging as necessary to
  keep shell semantics — to improve that review surface. Probable shape: TWO explicit
  sections; all supposedly-non-mutative / low-attention code lifted to the FRONT
  (munged as needed); the INTENDED review surface — the book's mutative code — placed
  at the END (shell has no native hoisting, so the definitions must precede their
  uses) after some header/divider.

Provenance note (conductor): this restates and strengthens the single-stream
presentation obligation the prior conductor recorded in `plans/30L`'s render section
from the 2026-08-20 sitting (top-lift + munge per `28Q:pin-emission-planner-universal`;
section boundary comments). The artifact-forms lane discharged it at v0 scale only
(`30Ne`'s presentation section: the stream then carried only the guard preamble; the
closing boundary was priced and deferred as `tc-flattened-section-boundary`). Under
`rul-piped-stdout-carries-a-full-plan` the obligation is live at full scale; the rework
lane carries it.

Candidate KNOB (conductor-proposed for the human's registry, not minted): the
single-fully-reviewable-artifact request vs the attention-product's mutative/non-
mutative partition — a dial the admin sets per invocation by choosing the stream;
Dorc's job on the single-artifact pole is partition-by-LAYOUT rather than
partition-by-absence.
