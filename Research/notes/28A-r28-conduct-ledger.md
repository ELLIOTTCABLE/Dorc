# 28A — round-28 conduct ledger (the ONE durable for the build round)

AI-authored (Fable conductor, seated 2026-07-19). Executes `plans/280` (charter,
human-acked) over the seed specs `plans/281` (mark grammar) + `plans/282` (errorloom).
Authority: root docs, `spike/CLAUDE.md`, human-typed rulings outrank. Per the `27U`
precedent (one note per phase, human-ruled): builders write NO landing notes; as-built
detail lives in granular commits + this ledger. Conductor stack: **`ai/r28-impl`**
(base `40f8005` = ai/main tip = the folded `ai/r27-aid`); re-pointed at verified lane
tips as folds land.

## §0 — Session directives (human-typed 2026-07-19, this sitting)

- The conductor has LATITUDE over the `280` §1/§2 division: "making errorloom a
  quality product" is a meaningful goal; shift scope into/out of the bounded product
  by *utility-to-others and product-value*, never by implementation convenience or
  parallelization-plan fit. The prior conductor's split is advisory, not law.
  (This explicitly relaxes `282:rul-own-crate-own-tests`'s "invest nothing further"
  lean — the rul's two-layer split stands; the investment cap is lifted. Sharp-edges
  posture (`282:rul-internal-tool-sharp-edges`) STANDS for refusal UX; "quality"
  buys API shape, docs, self-tests, genericity — not friendliness tiers.)
- Push work through autonomously; stop only where human ack is genuinely
  quality-load-bearing. Quality trumps progress. Rewind-resilience: keep rulings and
  state durable here.
- Side-quest (human-typed mid-session): an Opus janitor sweep of ancient
  worktrees/branches — guarded, merge-checked, dropped-work surfaced never deleted.

## §0b — Directives typed 2026-07-19 (mid-round, second sitting)

- **DocID naming**: no semantic/lettered suffixes; serial claim-the-next
  (`283`, `284`, …) across the round's shared ID space. The already-minted
  `28A`/`28B`/`28C` stand GRANDFATHERED (their `28A:rul-*` slugs are live in
  builder contexts and commits; renaming mid-flight breaks references; human
  graded the nit not-important). Next free ID: **283**. Propagate to every
  future brief.
- **ANALYZER-NEEDS union-merge ACKED**, subagent-driven (human hands-off):
  lane `ai/r28-analyzer-needs-merge`, 3-way against
  `merge-base(ai/main, de22017)`, faithful union of both refresh lineages, NO
  spelling modernization (the docs pass owns spelling sync post-respell).
- **Quarantine restock**: the full 24K*/279* ledgers are believed to live in
  `Research/notes/quarantine-DO-NOT-READ/` (the 280-series reports were renamed
  279-something there, per human recollection). Opus builders ARE welcome in the
  quarantine (human-typed license) but must report upward ONLY ack/nacks —
  filenames, existence, provenance; never content. Lane
  `ai/r28-quarantine-restock`: verify existence, rescue any missing from the
  surviving branches (janitor findings A/B/F), never overwrite quarantine files.

## §1 — `28A:rul-errorloom-product-cut` (conductor ruling; amends `280` §1 scope)

The charter's crate lane = `282` phases 1+3 (transport + container/runner). Re-cut by
product-value; the lane stays dorc-free and the parallel-lane file-disjointness is
preserved (everything moved IN is buildable against fakes/self-tests):

MOVED INTO errorloom (beyond the charter's soft split):

- **The bless orchestration** (`282` §6): mode inference over the touched-set,
  prose-bless vs structure-bless exclusivity (never-both), the baseline-verify law,
  the fixpoint-gate skeleton, and the two-method git trait (`head_version_of`,
  `dirty_paths`; subprocess-git impl + fake impl). Rationale: without this, errorloom
  is "a diff library plus a cram runner" — the orchestration IS the product's novel
  claim ("the transcript is the authoring surface"). Consumers implement a small
  trait (baseline tagged render · apply field-edits · re-render); errorloom drives
  the loop.
- **The span-map schema**: errorloom OWNS the region vocabulary
  (TemplateLiteral/ParamValue/ForeignText/Arrangement) generically, keyed by opaque
  consumer keys. Dorc's tagged render (serial lane, `282` phase 2) emits core-owned
  span types; a thin adapter maps onto errorloom's — dorc-core takes NO dependency
  on errorloom (kernel-dep-cleanliness; the adapter is the normal consumer shape).
- **Generic case-quality gates**: txtar-marker-collision refusal, CRLF refusal,
  absolute-path refusal, and a configurable required-token coherence gate (driven by
  a frontmatter key the consumer names). All generic transcript hygiene.
- **A thin CLI** (`errorloom run` / structure-bless = the cram mode) — fully generic.
  The prose-promote flow stays library-only (it needs consumer callbacks).
- **The toy consumer**: an in-crate end-to-end self-test (tiny fake catalog + fake
  templated renderer) exercising the FULL promote loop dorc-free. This is the
  API-fit proof that de-risks phase-4 consumption.

STAYS OUT (Dorc-side, serial lane): tagged-render emission (the walker twin);
catalog serialization + field-edit application; frontmatter schema SEMANTICS
(`code`/`when-fires`/`why` are Dorc's keys; errorloom treats frontmatter as an
opaque flat map); Dorc case-policy lints (harness-env-must-not-appear,
param-value-word-distinctiveness); the inert-mocks PATH policy (errorloom provides
the controlled-env mechanism; policy is injected per-invocation).

Publication posture: `publish = false` until the human flips it; LICENSE choice is
the human's (rider `28A:rider-errorloom-license-choice`); README + doc-comments are
in-scope now.

## §2 — Conductor pre-rulings for the syntax lane (flag-if-it-fights, never silently)

- **`28A:rul-continuation-attachment`** — a mark-only physical line accrues to the
  preceding statement's mark-block iff the preceding line ended with a mark-block
  (chains of mark-only lines continue the same block); otherwise it stands alone
  (position-/path-scoped, the old bare-colon-line posture). A standalone block
  containing an rc-consumer (`asserts`/`refutes`) or `reads` is a loud diagnostic ⇒
  that block drops to ⊤ (there is no statement to measure/back). Meta verbs with
  member-/kind-collected semantics (`stored-in`'s invariance sibling,
  `undivided-by-transit-across`) are collected member-wide wherever they ride, so
  attachment is semantics-neutral for them — which is why `281` §11's kind-owner
  example is unambiguous in effect despite reading as a continuation. rc-arity is
  enforced over the WHOLE block including continuations.
- **`28A:rul-respell-atomic-cutover`** — no shipped dual-parse. Additive-first
  granular commits (new verb tables, strip machinery, new codes, unit tests that
  don't break e2e), then ONE cutover commit flipping the parser + the mechanical
  corpus respell + regenerated goldens together, gates green at that commit.
  Builder-run golden regeneration inside the lane worktree is licensed as
  WORKING-STATE only; the authoritative bless is conductor-executed at lane close
  on a fresh verified binary with case-by-case diff inspection (`280` §4 stands).
- **`28A:rul-ratchet-accepts-new-codes`** — the shrink-only DEFINING_CASE_RATCHET
  governs coverage-regression of existing codes; a newly-minted code legitimately
  enters the ratchet with a per-entry injection-surface note (d4b practice). If the
  gate test literally forbids additions, FLAG UP — never hack the gate.
- **`28A:rul-marker-version-unchanged`** — the respell lands within
  `# dorc-lang/v0.1` (pre-release marker-gated agility, `281` §R4 / `278` §4). No
  v0.2 mint.
- New parse-diagnostic codes mint with EMPTY prose under the AS-BUILT placeholder
  mechanism (stored `[unwritten: <slug>]`) — the absent-field render arrives only
  with the `282` phase-4 flip, serial lane.

## §2b — Phase-B rulings (conductor, 2026-07-19, on `notes/28B`'s flag list)

- **`28A:rul-new-codes-ship-covered-cases`** (SUPERSEDES `rul-ratchet-accepts-new-codes`;
  resolves `28B:flag-ratchet-forbids-additions`) — the as-built `ratchet_only_shrinks`
  gate is correct and stands; new DiagCodes ship as `covered()` defining cases with
  `[unwritten: <slug>]` prose (the d4b `aid-unloaded-sibling-oracle` precedent). The
  `unwritten <= 1` ceiling bump to 5 is hereby consciously authorized (the test's own
  "conscious conductor act"). The four codes' REAL cases + prose arrive through the
  `282` empty loop after the generation flip — a named rider on the unify lane's
  case-authoring fan-out, not this lane's work.
- **`28A:rul-marked-colon-is-the-grammars`** (resolves the dialect edge inside
  `28B:flag-standalone-single-colon-restructure`) — in a MARKED file, a
  statement-leading `:` followed by any content is the mark intro, always (loud
  malformed-⊤ per `281` §9; inv-top-reject bias). A LONE `:` with no following
  content on the line stays the POSIX null command (`while :; do` survives — an
  intro requires WS + at least one mark). Genuine null-command-with-args idioms
  (`: "${VAR:=default}"`) are thereby outside the marked dialect: respell corpus
  instances to a `true`-spelled equivalent; phase B greps statement-leading-`:`
  non-mark uses and reports the count, flagging back if any are load-bearing or
  teach-facing.
- **`28A:rul-bind-equals-tail-disambiguates`** (resolves
  `28B:flag-inline-bind-vs-head-decode`) — inline-bind dispatch runs BEFORE mark
  head-decode, keyed on the `= value` tail per `281` §8 (not merely on bare-kind
  shape). No `=` tail ⇒ the `:` is a mark intro. Value-less inline binds are OUT of
  the grammar; respell the corpus-live `index : pkgindex` instance to a proper
  two-dot kind + valued form (flag if it resists).
- **`28A:rul-one-checkpoint-after-machinery`** — ONE mid-lane conductor checkpoint,
  sited AFTER CP-C (AST + consumers + diagnostics + lexer/parser machinery all
  unit-green, corpus untouched, parser still unwired from e2e): phase B executes
  CP-A→CP-B→CP-C then STOPS for conductor review of the license-bearing parse
  behavior; CP-D (the one-commit cutover + respell + working-state bless) runs only
  on the conductor's explicit go.
- Accepted as proposed, no amendment: both brace shapes (`@{a,b}` attached ·
  `verb {a,b}` standalone payload); absorbing selector-charset/malformed-target
  failures into `PredictOutOfDialect` (no `mark-selector-invalid` mint); reusing
  `MarkBraceVerdictSingleCell`; `mark-hashcolon-malformed` at Warning; the four-code
  mint list; zero wire-slug renames (prose mentions only); the
  `28B:flag-emptied-case-arm` handling PLUS a rider: a strip-floor differential
  fixture (stripped whole-kind-disturbs oracle → empty case arm → parses+runs under
  the e2e dash gate; inline-arm marks take the region-delete path);
  `reads`/`:?` identical wiring to Observe.

## §2c — errorloom d1 flag rulings (conductor, 2026-07-19, on the d1 landing report)

d1 LANDED at `c397cf8` on `ai/r28-errorloom-crate` (two commits off `0825f6e`; six
tests incl. the 500-seed round-trip property; zero deps — hand-rolled word-LCS,
accepted with its stated dep-clean rationale; comment budget 0/30; all gates + e2e
97/97 green; the human's mid-flight Rust-taste redirect absorbed, one real fix:
refusal-context borrows sources, dump materializes only on the error path).

- **`28A:rul-paragraph-model-v1-refuses-restructure`** (on d1
  `flag-paragraph-restructuring-refuses`) — ACCEPTED: intra-paragraph word edits
  only; break add/remove refuses as ArrangementEdited (`282:rul-words-and-
  paragraphs-only`'s "starts this small"). NAMED SEAM, recorded honestly:
  post-generation-flip there is NO path to add/remove a paragraph in existing
  prose except a v2 model growth or the `282` §5 annotated-editing retreat — if
  the human's prose-rewrite pass hits this, it surfaces fast and the growth is
  priced then, not now.
- **`28A:rul-tagged-render-emits-instance-ids`** (on d1
  `flag-instance-inference-is-structural` + `flag-hole-between-two-instances`) —
  ACCEPTED the builder's recommendation, and threading it forward: d2 grows the
  span schema with an OPTIONAL instance/occurrence discriminator on
  TemplateLiteral (+ParamValue), structural inference retained as the
  absent-case fallback; the unify lane's phase-2 emitter MUST emit instance ids
  (retiring the two heuristics for Dorc's own use). Schema change now is cheap
  (pre-publication); after publication it would be a breaking change.
- **`28A:rul-span-cover-stays-total`** (on d1 `flag-full-coverage-span-map-
  contract`) — the gap-free total-cover contract STANDS (fail-fast beats
  permissive); the unify lane's emitter renders inter-region whitespace as
  Arrangement runs (which `282` §4 already classifies as Arrangement: "blank
  structure"). If the phase-2 builder finds this genuinely awkward, that is a
  flag-up, and relaxation ("non-overlapping, word-starts-covered") is a
  documented API change to adjudicate then.

## §2d — Syntax-lane CP-A checkpoint + the Singleton-bind ruling (conductor, 2026-07-19)

- CP-A LANDED @ `8cd0221` on `ai/r28-syntax-respell` (2 commits: `MarkKind` grown to
  the typed verb set with role-aware recovery from OLD spellings; carry/entry
  consumers verb-routed, dead tokens removed; 958 unit + 97/97 e2e + four gates
  green; 1/25 comment budget). Builder retired at budget, principledly early.
- **Coupled-CP finding ACCEPTED**: CP-B (the four DiagCodes) and CP-C (the new
  parser) are one indivisible green unit — `diag_tidy`'s production-emit gate needs
  the new parser's literal emit sites. The remaining unit executes under a FRESH
  builder (27U map→fresh-executor pattern) on branch `ai/r28-syntax-respell-2`;
  the `28A:rul-one-checkpoint-after-machinery` shape is unchanged (stop before
  CP-D; conductor go required). Contingency granted: if the covered() harness's
  e2e-trigger half mechanically requires the wired parser, land what lands green
  pre-cutover and carry the remainder into CP-D — never weaken a gate; report
  actual mechanics.
- **Colon-carve grep result banked**: ZERO corpus resistance to
  `rul-marked-colon-is-the-grammars` (no `while :`, no null-command-with-args in
  any marked file; the 33 double-colon lines are all mark intros). Lone-`:`
  survival = unit-pin only.
- **`28A:rul-singleton-bind-drops`** (resolves the builder's bind-resists flag) —
  option (b): the value-less Singleton bind (`idx : sm.dorc.PkgIndex`) is DROPPED,
  not respelled; the entity-less whole-kind coordinate (e.g.
  `sm.dorc.PkgIndex@fresh`) IS the sanctioned nullary spelling, and binds require
  the `= value` tail with no exceptions. Rationale: a bind types a VALUE as an
  entity (`281` §8); a singleton kind has no value to type — the coordinate names
  it directly, and the bound name was already unused by the marks. Corpus respell
  (pkgindex.oracle.sh + 5 e2e cases + test literals) rides CP-D.

## §2e — Makework lane LANDED + FOLDED (2026-07-19; branch `ai/r28-makework` @ `bb176ba`, merged `3517689`)

- ANALYZER-NEEDS union: 277-slug exact union of both lineages; two two-sided rows
  (header ¶s kept both, chronological; `an-verdict-function` resolved to
  de22017's strictly-subsuming text). Conductor own-hand structural verification
  passed (both refresh ¶s · section P · aid rows · slug sentinels). Human reviews
  at round fold.
- Quarantine restock, with two PREMISE CORRECTIONS now authoritative: the
  quarantine lives at `Research/quarantine-DO-NOT-READ/` (moved by human commit
  `f8d8add`; the old `notes/`-nested path is empty), and the 280-report series
  lives there renamed `27X{a..e}` (not 279-renamed). 15 acks · 16 rescues · 0
  uncertain; two `-recovered` suffixed files preserve divergent authoring passes;
  nothing overwritten. Human follow-ups: bucket the flat-rescued 279/280 material
  into a subdir if preferred; the `24Ka`/`24Kb`/xcheck branches are now fully
  redundant with quarantine copies and safe to retire at leisure.

## §2f — Tasteful-Rust research adjudication (conductor, 2026-07-19)

The human-resumed research round LANDED (`.claude/research/tasteful-rust/`: 22
graded sources, validate-clean; anchor = Microsoft Pragmatic Rust Guidelines
v2026.6 + official API Guidelines). Conductor adjudication: most of the rule-set
is already enforced (workspace lints = panic-free/no-unsafe; DST law =
M-MOCKABLE-SYSCALLS stated first-party; comment-budget law =
M-NO-META-DESIGN-DOCUMENTATION; d1/d2 briefs = borrow-first,
concrete-over-abstract, typed errors). THREADED to errorloom d2 mid-flight (six
items): common-derives audit · `#[non_exhaustive]` judgment pass on growable
public enums (semver, pre-publication) · `#[must_use]` audit · doc conventions
(crate example, `## Errors`, `?`-examples, missing_docs) · MSRV declaration ·
the taste-stance synthesis (both LLM-slop and idiomatic-maximalism are distance
from simple-and-correct, opposite directions; visible clone ≠ defeat). DECLINED
to thread: iterator-combinator taste (clippy covers), crate-splitting
(simplicity beats compile-time for one small crate), pedantic-clippy expansion
+ cargo-audit/miri (lint-table/CI churn = human's call at publication). The
syntax lane gets none of this (dorc-internal conventions rule).

## §3 — Lane map and state (update on every change)

| lane | branch | shape | state |
|---|---|---|---|
| errorloom-crate d1: transport engine | `ai/r28-errorloom-crate` | single dispatch | LANDED 2026-07-19 @ `c397cf8` (flags ruled §2c) |
| errorloom-crate d2: container/runner/orchestration/CLI | same branch | FRESH executor (27U pattern), off `c397cf8` | DISPATCHED 2026-07-19 |
| syntax-respell phase A: proposal (`notes/28B`) | `ai/r28-syntax-respell` | map-then-execute, map half | DISPATCHED 2026-07-19 |
| syntax-respell phase B: execute | same branch | after conductor ruling on 28B | pending |
| janitor sweep (side-quest) | no branch; repo-global surgery | single dispatch, guarded | LANDED 2026-07-19 → `notes/28C` (15 worktrees removed, 34 merged branches `-d`'d, zero force-ops; dropped-work findings A–F for human adjudication) |
| errorloom-unify (`280` §3) | `ai/r28-errorloom-unify` | serial, off both folds | pending |
| docs/steering/registry re-synthesis | rides unify tail | must ALSO sync `KNOBS:kOOB`/`kTYANNOT` (`281` §12: both carriers ship; the "exactly ONE comment-parse" text is stale post-`#:`-ack) + `spike/CLAUDE.md` authored-surface + marker-gates-syntax-only + strip bullets + `docs/reference/oracle-contract.md` §4 + author-oracle skill | pending |

## §4 — Ack-ledger (only human-TYPED items count)

- 2026-07-19 session brief: the three deliverables; conductor latitude over the
  division; autonomy posture; "quality trumps progress". (TYPED.)
- 2026-07-19 mid-session: the janitor side-quest. (TYPED.)
- Outstanding asks to the human (none block the lanes): (1) `28C` findings A–F —
  chiefly the STRANDED ANALYZER-NEEDS block-settle re-grade (`de22017`,
  human-reviewed 2026-07-17, absent from ai/main; the round-28 docs pass is
  chartered to union-merge it with ai/main's 2026-07-18 aid refresh and present
  for review — veto if you'd rather hand-merge); the unmerged 24Ka/24Kb
  language-review ledgers + 279a–e/280a–e crosscheck reports (presumed
  intentional, `279f` being the distilled product — adjudicate at leisure); the
  three dirty worktrees holding uncommitted unique-looking notes (janitor
  refused removal, correctly). (2) errorloom LICENSE choice. The `TODO-ADDTL`
  tail riders remain banked (charter law — none block the lanes).

## §5 — Dispatch log

- 2026-07-19: `lane-errorloom-crate` d1 (Opus, worktree, bg) — transport engine.
- 2026-07-19: `lane-syntax-respell` phase A (Opus, worktree, bg) — proposal note
  `28B`, no engine edits.
- 2026-07-19: janitor (Opus, primary checkout, bg) — survey + guarded cleanup +
  dropped-work report; report to conductor scratchpad, findings banked here after.
