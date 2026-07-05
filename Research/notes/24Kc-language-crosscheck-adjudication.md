# 24Kc — cross-model language-design crosscheck: adjudication + retrospective ledger

AI-authored (Fable conductor/adjudicator), 2026-07-05, round 24 (the language-design arc the
round pivoted into). **Dual purpose, per the `254` precedent:** (1) the conductor's adjudicated
verdict; (2) a retrospective-attribution record — every cluster and notable single carries its
disposition, so a later running issue that matches a deferred/discarded finding is the signal to
revisit. Raw lane material — all six reports, the as-run packets, and the conductor's working
ledger — is archived verbatim in `quarantine-DO-NOT-READ/24K-lane-archive/` (rescued to durables
at the human's direction after the conducting session compacted mid-flight); per the purity
protocol it stays provenance-labeled raw material, and this note remains the only adjudicated
synthesis.

## Setup

Six parallel reviewers, three model lineages, two stance-engineered prompts each (the 24K
prompt-pair, quarantined; neutralised/disowned + adversarial/opposition-owned), all six carrying
the same two-phase protocol: **online skill-up first** (language ergonomics · designer regrets ·
gradual-typing practice · shell-adjacent backwards-compat), then a **pre-registration gate**
(lessons written before corpus exposure), then the corpus review with USER_STORY as primary
target. Review pinned at `259b27d` (one lane verified target docs unchanged through `e3f67a5`).

Lanes: Fable ×2 (Anthropic, isolated worktrees; full notes committed as `24Ka-*` on branch
`ai/24Ka-langreview`, tip `19df800`, and `24Kb-*` on `worktree-agent-abd7ff8be88067e1b`, tip
`fd5fa82` — both gates PROVABLE from their own commit ordering; verbatim extracts also in the
lane archive) · Codex/GPT-5.5 ×2 (foreign) · DeepSeek V4-Pro ×2 (foreign;
neutral lane required a continuation dispatch after ending its turn at the gate).
Protocol integrity: all six honored lessons-before-corpus and kill-your-own-findings; both
in-repo lanes disclosed the one contamination honestly (the harness auto-injects AGENTS.md
before the brief — a clean-context leak recorded as machinery-feedback for future crosschecks).

Credal stance (`[[crosscheck-adjudication-skepticism]]`): deflationary; cross-model agreement is
*mild* evidence (lineages share training-data blind spots); Codex over-flags severity by
calibration; DeepSeek is near-frontier. Ranked below by convergence × verifiability × cost-to-fix.

## The cleared core (lead result — most of the language SURVIVED)

Unanimous or near-unanimous credit, across stances and lineages, each verified against the
corpus by at least one lane's own kill-attempt discipline:

- **The rc-partition (0/1/≥2, decline-as-control-flow)** — all six lanes independently judged it
  well-chosen; one ran its own pre-registered DETECT against the errexit/status-zoo lesson and
  passed it ("better than I expected to find anywhere"). The consumed-status taxonomy
  (Relaxable/Invariant/Iterated) drew specific praise.
- **The guard shape** (`( check ) || original-bytes`): subshell isolation, errexit-exemption,
  bytes-verbatim, never-synthesized-sh — "reads as the human idiom"; explicitly cleared of the
  "second language" suspicion.
- **Books stay clean**: census 153/154 books pure POSIX; the admin-side erasability promise
  HOLDS today (the problem is the oracle side — below).
- **rul-attention-honesty** ("language-design gold"), silence-licenses-nothing, the bought-
  unsoundness section's honesty, the means-nothing-not-refusal error posture, ORACLE_PROVIDES
  as exactly the enumerated feature-ledger the compat literature demands, and 24G's
  name-as-contract naming doctrine.
- Withdrawn-by-reviewers after checking: "nobody noticed the dash break" (known + priced),
  "strawman makes it unreviewable", "verdict rcs leak into ambient set-e", period-name collision.

Net: every load-bearing criticism lands on the **authored surface and its compatibility story**,
not on the verdict/license semantics or the emitted machinery. The semantic skeleton passed a
three-lineage adversarial panel.

## The clusters (convergence ledger + dispositions)

**cluster-authored-surface (ALL SIX LANES; the dominant finding).** The authored-oracle dialect
is a compiled language documented as sh. Components, each independently verified by ≥2 lanes:
(a) dotted names are dash/ash parse-fatal and — live-executed by the Fable-adversarial lane —
trailing marks become *argv*, so a raw-run authored oracle can **silently answer wrong**, not
merely crash; (b) USER_STORY's blessed share-a-file path (stage 3 appends the oracle to the
book's own file) makes the *book* non-runnable, directly contradicting the kTYANNOT containment
("books stay verbatim-runnable") — one settled artifact is false as written; (c) the strip
transform is the de-facto compiler pass: no user-facing tool, no versioning, and its spec is
inconsistent across settled layers (`name_predict` single-underscore in spike/CLAUDE.md vs
`name__is_converged` double in goldens vs `foobar_is_converged` in USER_STORY vs `foobar_check`
in renders), with a non-injective hyphen-munge (`apt-get`/`apt_get` collide) recorded nowhere;
(d) census: **0 of 187** oracle fixtures are dialect-free, and 185 use the *stripped* form as if
it were the authored form — the corpus teaches the wrong surface; (e) one lane's lever, aimed at
the open weld: the kOOB redline's own config-vs-metadata wording may make **erased eol-comment
annotations redline-clean** (metadata, not configuration), i.e. the inline pole's off-ramp cost
may have been an unforced trade. Conductor counter, held with it: the human's kOOB clarification
*explicitly* lists no-comment-parsing in the verboten set — the lever is a question for the
weld-owner (was that ban's intent configuration-channels, or does it cover erased annotations?),
not a refutation. **Disposition: THE stop-the-world item — see verdict.**

**cluster-rungs (all six lanes).** `is_converged`'s name is an answer; its authorship is a
license; a wary author has no rung-0/1 position. Every lane found it independently.
**Disposition: already registered as `kCONTRACT-RUNGS` (same day, before the panel returned) —
the panel unanimously validates the tension and adds urgency: resolve or consciously re-affirm
`-single` BEFORE the P5 stdlib is authored.** Rides the same decision moment as the verdict.

**cluster-marks (all six lanes).** One trailing-`:` syntax, ~five role-dependent meanings.
Known cost (24G documents it) — but the panel added live evidence it is already biting: ACK and
POISON exist in law with ZERO corpus occurrences (unexercised grammar), and `command -v` is
marked *establish* in one fixture family and *observe* in another (author-taste polarity drift).
**Disposition: language-round agenda, elevated from "documented cost" to "evidenced drift";
distinct mark-heads per intent is the recorded candidate fix.**

**cluster-kinds (five lanes).** Kind vocabulary has no ownership/namespacing/evolution rule in
the wild corpus (bare stdlib kinds beside `fb.*` beside plumbing-kinds; the 17N reverse-DNS lean
appears nowhere in fixtures); cross-lineage same-name-different-meaning remains the Seam's
silent cell. **Disposition: already the dq-kOOB residual in TODO-ADDTL; the cheap pre-stdlib act
is writing the one-paragraph ownership/namespace rule the P5 brief can obey.**

**cluster-compat (all six lanes).** No dialect-version marker, no verdict-churn policy
(inv-top-reject's trigger-set is designed to shrink ⇒ same-book plan drift across releases is
built-in and unnamed), no per-channel stability declarations (the `# dorc:` trailer grammar is
byte-frozen by rec-1 law yet was never designed as a stable format), respell-labels without
codemods (the touches() migration is sequenced but pattern-less). **Disposition: the v0
compatibility stamp (already on the table this round) is the immediate containment; the fuller
epoch/changelog/channel-stability policy is language-round work. All specifics recorded here for
the retrospective hook.**

## Verdict — the one stop-the-world item

**Rule the authored-surface weld — dq-kOOB/kTYANNOT — formally, before P5 authors the stdlib,
as one decision-package.** The panel's material gives the weld-owner three coherent poles:

1. **Inline stays (status quo made honest):** ship `dorc strip` as a first-class, stable,
   versioned tool; fix the strip-spec inconsistency and document the munge + reserved namespace;
   re-word the identity claims ("a compiled superset of sh with a one-command off-ramp", not
   "still just sh"); either un-bless share-a-file for dash-targeted books or scope the book
   off-ramp promise to oracle-free books.
2. **The eol-comment pole, re-opened via the metadata reading:** decide whether the kOOB
   no-comment-parsing ban was aimed at configuration channels or covers erased annotations; if
   the former, the inline pole's entire off-ramp cost was optional — this is the panel's
   sharpest lever and only the human can rule it.
3. **Accept compiled-language identity fully** (the TypeScript position): keep inline, drop the
   sh-identity marketing for the authored surface, and budget the ecosystem-tooling story
   (shellcheck/shfmt/editor) that position demands.

Whichever pole: repair the settled-layer contradictions regardless (share-a-file vs containment;
the strip-spec drift; USER_STORY's `foobar_check`-vs-`foobar_is_converged` naming). The timing
constraint is the real teeth: the respell window closes at the first external-ish corpus, and P5
is that corpus.

## Small fixes for the r24 conductor (cheap, non-blocking, verified-or-verifiable)

- **fix-return-decline-inert** (Fable-adversarial F2; verify in `cli/main.rs` verdict-lift):
  explicit-return style (`if probe; then return 0; else return 1; fi`) now parses as
  all-declines ⇒ silently inert oracle, no diagnostic — while USER_STORY teaches rc-is-read.
  Either lift bare `return 0/1` reached-paths as verdicts or emit an inert-oracle diagnostic;
  one sentence of doc either way.
- **fix-nounset-fixture-idioms** (Fable-neutral F9, empirically verified): `[ "$2" = "" ]` is
  set-u-fatal on the well-formed path, universal in fixtures (133 files), and *regressed* from
  the older strawmen's safe `$#` form; `${2-}` costs nothing. P5 quality-bar line + fixture
  sweep candidate.
- **fix-munge-reservation** (three lanes): document the emitted `*__role` namespace + the
  hyphen-munge non-injectivity; a collision lint is near-free now.
- **fix-onramp-arity-honesty** (two lanes): the stage-3 nine-liner omits the arity gate its own
  license semantics require; either the walkthrough notes it or the "real minimal" is shown once.
  (Human-voice doc queue; pairs with the existing R2-MULTIOP quality-bar item.)

## Recorded-not-actioned (retrospective hooks)

Argparse-boilerplate viscosity (×405 census; capped by provides-decoding failing safe; helper-
idiom candidate for the language round) · name-semantic-overload (roles/tiers/polarity all ride
bare names; grammar-carries-tier candidate if kCONTRACT-RUNGS goes ladder) · emitted-collapse
legibility (`: | true || : | :` wants a render legend) · theatre-flag framing (known-priced by
the ruling's own words; panel adds nothing the human didn't already write) · two-declines
synonymy (weakly-sourced; hold) · UNK-channel in-band tension (principled carve-out; teach it).
**If a later running issue matches one of these, that is the signal to revisit this note.**

## Panel-quality notes (for future crosscheck design)

The pre-registration gate worked (both Fable lanes' gates are git-provable; findings trace to
lessons); the clean-context leak (AGENTS.md auto-injection) is the one integrity hole to fix in
future machinery — run reviewers outside the repo cwd or suppress instruction-loading. Foreign
lanes were cheap and confirmatory rather than generative: every foreign finding was found
independently by an in-lineage lane, but the three-lineage unanimity on the top cluster is
exactly the correlated-blind-spot check the exercise was designed to buy. Dispatch-tooling
friction (nine items: model-pin frontmatter, heredoc/argv limits, PATH, permission-classifier,
turn-truncation, etc.) lives with the human's harness notes, not in this corpus.
