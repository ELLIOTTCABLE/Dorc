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

## §2g — errorloom d2 landing + flag rulings + lane fold (conductor, 2026-07-19)

d2 LANDED @ `37b26bd` (6 commits; 38 crate tests incl. the full toy-consumer loop;
all six taste-addendum items done; budget 28/30). Lane FOLDED @ merge `8d84b01`,
conductor own-hand verified: fresh build · 996 unit · e2e 97/97 · gates ok
(`DRY=1 conduct-bless`). The crate is COMPLETE pending the human's LICENSE +
publish flip. Flag rulings:

- **`editable_text` as a 4th Consumer method: ACCEPTED** — which blocks of a
  multi-replay case are prose-editable is consumer knowledge by construction
  (`282:rul-multi-replay-per-case`: machine-format replays are whole-block
  structural, never prose-diffed); errorloom cannot infer it. The `28A` §1 trait
  shape is amended to four methods.
- **Absolute-path hygiene gate scoped to temp-dir-leak: ACCEPTED** — refusing any
  `/…` string would false-positive on legitimate transcript content (`/etc/…` in
  rendered errors); the deterministic sandbox-leak check matches `282` §7's
  intent. Broader path-policy, if wanted, is a Dorc-side case-lint later.
- **Render-level fixpoint accepted as the GENERIC floor; RIDER minted for the
  unify lane**: Dorc's CI gate must ALSO implement `282` §6's literal
  promote→catalog-byte-identical check consumer-side (where the serializer
  lives) — a render-invisible catalog metadata hand-edit is not caught at the
  generic layer.
- Toy-consumer in-process render split, and the skip-sans-git `SubprocessGit`
  test (hermetic via `FakeGit`): accepted as reported.
- **Emitter contract addition banked**: per-key ALL-OR-NOTHING instance
  stamping (any stamped span for a key ⇒ every span of that key stamped, else
  structural fallback) joins the two ruled emitter requirements for the unify
  lane's phase-2 builder.

## §2h — Syntax-lane CP-B+CP-C checkpoint rulings + the CP-D go (conductor, 2026-07-19)

The coupled unit LANDED @ `0373dfa` on `ai/r28-syntax-respell-2` (5 commits atop
CP-A; the four codes as full covered() cases — the e2e-trigger contingency never
fired, the "fires" gate greps cfg(test) too; the new parser + strip in a
`#[cfg(test)]`-gated module, zero production surface — ACCEPTED, un-gating is
CP-D step one; 20 license-behavior unit tests; budget 24/24; e2e 97/97 old
grammar). Checkpoint rulings on the five flags:

- **`28A:rul-uniform-kind-payload-home`** — token payloads live uniformly in
  `target.kind` (the old `.entity` home for safe-across/undivided was an
  artifact of splitting `tolerates:user` on its colon; the new grammar has no
  such split). CP-D migrates `entry.rs`/`carry.rs` to read `.kind` when the new
  parser drives; the `28B` §2a payload-enum reshape is SANCTIONED-OPTIONAL —
  take it only if it doesn't balloon the cutover, flag if it would.
- **`28A:rul-per-intro-head-decode`** — ACCEPTED as the only reading consistent
  with `281` §11 (the `:?` continuation requires per-intro sugar decode). A
  bare-coordinate continuation legitimately reads `asserts` and trips rc-arity
  against a head verdict — correct behavior, right diagnostic. Docs-pass rider:
  one clarifying sentence into `281` §4 (kept-current plan; the ambiguity is
  real).
- **`28A:rul-brace-on-reads-legal`** — `281` §6's plain text wins over the
  277-era observe refusal: brace on `reads` expands to N observe facts
  (backing-widening is always-safe and honesty-positive); the verdict refusal
  stands. CP-D updates `derive.rs` accordingly; `MarkBraceVerdictSingleCell`'s
  slug is already verdict-scoped — no wire change.
- **`28A:rul-value-tail-inventory-precondition`** (the hazard) — the old
  grammar's verdict-position `= value` tail (`… @active = false`-shaped) has NO
  `281` spelling, and neither `28B` nor `281` dispositioned it. HARD CP-D
  PRECONDITION: inventory corpus+test usage of `= value` in verdict position
  (bind excluded). Zero instances ⇒ dead construct, drops silently. ANY
  instance ⇒ STOP before the cutover and flag the list up — the conductor rules
  a spelling then. The cutover may not flip with this unresolved.
- Remediation-class `DeclareIdentity` on all four: accepted as placeholder;
  re-cut at prose-authoring time.

CP-D GO issued to the same builder (context-hot on its own parser beats fresh
onboarding for the delicate wiring; gates back-stop degradation). CP-D scope =
`28B` §5 step 4 + the banked riders: un-gate + wire the parser · consumer
`.kind` migration · derive.rs brace update · the mechanical corpus respell
(incl. `28A:rul-singleton-bind-drops` drops + the `@` flip + verb-words +
double-colon collapse) · crate `CLAUDE.md` authored-surface blocks · goldens
regenerated as WORKING-STATE (authoritative bless stays conductor's at fold) ·
comment budget: 0 net-new non-doc (doc-comments exempt; flag genuine need).

## §2i — The value-tail ruling (conductor, 2026-07-19; precondition fired as designed)

Inventory: ZERO corpus instances; ONE test literal (the old parser's `= value`
capture test, `parser.rs:1742`, semantically incoherent by its own content); one
production consumer branch (`carry.rs:360-362` value-cleanliness in the
read-set-closure walk) that nothing ever populates.

**`28A:rul-verdict-value-tail-drops`** — the verdict-position `= value` construct
is corpus-dead and undocumented (absent from the oracle-contract reference §4;
never dispositioned by `277`/`281`): it DROPS with the old parser. Foreclosure
check: the value-bearing-verdict future (the `.diff` verb, is_noop-style
state-precise siblings, MH2/an-ide-value-layer) is PARKED under
extend-by-name-never-re-read (TODO-ADDTL item 1) — that future mints its own
spelling; a dead vestigial capture forecloses nothing. Excision posture:
PREFERRED = remove `MarkTarget.value` + the dead carry branch entirely
(always-None fields are representable-illegal-state smell; the closure walk is
default-disqualify and must not carry zombie surface), with the closure tests
pinning unchanged behavior on remaining inputs; FALLBACK if the field threads
wider than expected = keep it with a deprecated-dead doc-comment citing this
ruling. Builder judgment between the two, reported. The capture test dies with
the old parser. CP-D precondition RESOLVED — but the cutover is HELD on the
human grammar-nit ack round (human-directed mid-session 2026-07-19: "surface
grammar nits… I would like this to be the last respell"). Nit list in chat +
pushed; the builder stays stopped until acks land and the conductor sends the
combined ruling+go.

## §2j — The grammar-nit ack round (HUMAN-TYPED, 2026-07-19; the "last respell" gate)

All eight nits acked; the cutover hold LIFTS. Itemized with grade:

- `nit-at-selector-permanence` — ACKED, spike-grade permanence semantics
  clarified: the grammar being built this round is **v0.2** ("once it's all
  committed and solid"); a non-`@` selector would be a **v0.3** event. That IS
  what permanent-as-anything-can-be means here.
- `nit-singleton-bind-drops` — **SOFT ack** ("not entirely convinced it's
  valueless"): status = dropped-for-0.2. Re-entry, if ever, is a fresh design.
- `nit-verdict-value-tail-drops` — same soft grade: dropped; "if we need it,
  we'll find out the hard way, and re-add it" (extend-by-name covers re-entry).
- `nit-marked-colon-owns-statement-start` — ACKED; human classes it with the
  trailing-mark argv hazard: same danger-class, cost of entry.
- `nit-continuation-attachment` (incl. the standalone-rc fail-fast) — ACKED.
- `nit-per-intro-head-decode` — ACKED.
- `nit-brace-on-reads-legal` — ACKED, and generalized into a NEW HUMAN LAW,
  **`28A:rul-consistency-standing-authorization`** (typed): consistency is the
  goal of this pass — authors learn a construct once and naturally reuse it;
  standing authorization to take any ACKED construct and make it work in a new
  context someone may type it, PROVIDED doing so is sound and encourages
  correctness. (Conductor-mediated: builders still flag new-context extensions
  up; the conductor authorizes under this law.)
- `nit-entityless-transitional-form` — ACKED, lives on for 0.2.

**The brace MODEL (human-typed, supersedes the conductor's "two shapes"
framing, which was wrong):** braces are NOT asymmetric — they may surround an
entire payload word (N payloads) or a subset of one (shell-identical
"N payloads with this portion differing"). Braces trend DUMB: shell-like
lexing and semantics, never context-aware. v0.2 positional constraint (human:
"only before/after certain tokens… let's not stray into `fs{view,amble}`"):
brace groups legal ONLY as a whole payload word or immediately following `@`;
mid-token infix refused for now (extensible later under the standing
authorization). Implementation framing: expand-then-validate — expansion is
uniform and dumb; the existing laws (rc-arity) refuse what expansion makes
illegal; the specific braced-verdict diagnostic is retained as a friendlier
pre-check, not as context-aware lexing.

**Marker plan (conductor default, veto-window open):** the cutover stamps
`# dorc-lang/v0.2` as a SEPARATE FINAL COMMIT in the lane (veto = drop one
commit) — rationale: post-cutover files carrying v0.1 markers over new-grammar
syntax would be incoherent with the human's newly-named version semantics, and
the stamp rides the same sweep for free. Engine recognized-set moves to
{v0.2}; a `# dorc-lang/vX.Y`-shaped but unrecognized marker should diagnose
loudly rather than silently degrade to plain-sh (builder checks for an
existing marker-version code before minting; unwritten-ceiling bump to 6
pre-authorized if a mint is needed). `28A:rul-marker-version-unchanged` is
SUPERSEDED by this block.

## §2k — CP-D deltas landed; the single-mark ratification; the cutover handoff (conductor, 2026-07-19)

Landed green on `ai/r28-syntax-respell-2` @ `632070b`: the value-tail excision
(`5bbda5b`, PREFERRED posture — `MarkTarget.value` removed whole, re-add seams
doc-commented) and the brace expand-then-validate model in the reference
(`632070b`, four position tests). The builder then declined to rush the atomic
cutover at deep budget and handed off a turnkey plan — accepted; the fresh
cutover executor runs it (map→fresh-executor, third application this round).

**`28A:rul-single-mark-production-subset`** (ratifies the builder's
architecture flag) — production keeps single `Command.mark` + consumer-side
brace expansion; the full mark-block model (multi-mark lines, continuations,
cross-line rc-arity) stays tested-in-reference-only, adopted in a future
`Command.marks: Vec` round. THE LAST-RESPELL TEST DECIDES IT: `Vec` adoption
later changes zero spellings (single-mark corpus stays byte-valid; multi-mark
is additive ACCEPTANCE, not a respell), while adopting now would swell the
riskiest commit of the round across ~10 consumers for capability the corpus
does not use. Expressiveness note, honest: same-line `reads` disclosure and
same-line meta-verb chaining are deferred — authors spell them as separate
statements/standalone lines (the corpus-live idiom; body-wide observe marks
remain the superior spelling for real reads anyway). DOCS-PASS RIDER: document
v0.2-production as one-mark-per-physical-line, the block model
specified-but-not-yet-accepted (a named seam, `281` remains THE spec); plus
the §2h rider (one clarifying sentence into `281` §4).

## §2l — The cutover LANDED + FOLDED + conductor-verified (2026-07-20; task 7 closed)

Cutover `79257c0` (224 files, one atomic commit; WIPs squashed clean after the
API-death recovery) + detachable marker commit `ab7cb4c` (v0.2 stamp;
recognized-set {v0.2}; wrong-version marked files fall to the LOUD
`MissingDialectMarker` — the distinct `marker-version-unrecognized` message is
DEFERRED to the unify lane's empty-loop case-authoring, rider below). Folded @
`2ca2236`. CONDUCTOR VERIFICATION, own-hand: fresh build · 1018 unit · e2e 97
BLESSED on my binary with ZERO golden drift except one benign cosmetic flip
(`strawman24-pipe-guard-floor/expected.ran` — pipeline-stage log-order under
real pipes; the case's `RAN_ORDER=lax` check tolerates both; my flip restored;
NIT: bless could skip rewriting lax-order `.ran` files) · gates ok · and the
no-verdict-change claim MECHANICALLY CROSSCHECKED: every changed disposition/
summary line across all expected.out diffs is byte-identical modulo `@`/`#`
(normalized set-diff empty). Builder residue accepted: decode_intro duplicated
~25 lines (reference module stays frozen spec-in-code); trailing-bind
⊤-rejects in production; +10 comment budget with per-comment justification;
reaches/wrapper module doc-comments + CLAUDE.md prose v0.1 mentions → docs
pass. NEW unify riders: mint `marker-version-unrecognized` through the empty
loop (ceiling→6 pre-authorized); the lax-order bless nit. SyncThing dropped a
conflict copy of this ledger into the conductor worktree mid-bless
(`28A-…sync-conflict-20260719-182207-PHNHRER.md`, untracked) — left in place,
human-owned cleanup, third incursion class this round.

## §2m — Unify dispatch 1 (tagged render) LANDED; rulings (conductor, 2026-07-20)

LANDED @ `3bbd031` (4 commits; additive twins `fill_template_tagged`/
`render_body_tagged` — zero product-path edits, byte-stability by construction
+ a three-layer gate; core-owned `core/src/tagged.rs` types; adapter crate
`dorc-loom` validating gap-free covers through `errorloom::TaggedRender::new`;
instance ids stamped ALWAYS; 1024 unit / 97 e2e; budget 6/15). My brief's
render-seat pointer (tier_word/ChainRender) was STALE — the real seat is
catalog.rs::fill_template + diag.rs::render_body; the builder re-derived
correctly; the CLI aid-chain is a distinct prose surface, untouched. Rulings:

- notes/help/suggest emit-site text classifies WHOLE as Arrangement — STANDING
  (it is builder-latent, no production emitter exists; Suggestion is re-parked
  per 27U d4b). The classification re-opens when the first production emitter
  lands. Not a fifth class.
- The multi-paragraph seam stays unbuilt (zero corpus cases; folds gap-free
  today); pairs with the prose-quality-sprint deferral + `28A` §2c's v1 model.
- **PHASE-4 SCOPE RIDER (from flag 3)**: prose-bless baseline-verify needs the
  FULL-transcript tagged render — `render_cli`'s title/caret/gutter composed as
  Arrangement runs around `render_body_tagged`'s spans. This is phase-4 work
  `282` §9's ladder under-stated; the map half must size it.
- De-passthrough inputs banked for phase 6: 18 `detail`-heuristic ForeignText
  codes (list in the dispatch-1 report, this ledger's git blame); named
  our-words de-passthrough candidates: `escalation-policy`,
  `aid-unloaded-sibling-oracle`; the genuinely-foreign relays are the
  `syntax-*`/`predict-*` parser-description carriers.

## §2n — Generation-flip map (`notes/283`) rulings (conductor, 2026-07-20)

Map LANDED on `ai/r28-flip-map` @ `9e03647`. Rulings on its ten flags; the
executor runs the ladder with the map's step-7 checkpoint (stop before the
flip) honored:

- **`flag-in-process-vs-real-firing`: IN-PROCESS ACCEPTED, one real-fired
  pilot required** — the conflation the flag exposes: two distinct operations
  wear "firing." (a) The fixpoint GATE reconstructs a render to verify the
  prose round-trip — it MUST be in-process (the span map exists only in-process;
  a subprocess can't emit it). (b) A committed case's transcript must actually
  be what the binary produces. Resolution by surface-owner: the defining-case
  corpus tests RENDER COMPOSITION, which `render_cli(_tagged)` IS in production
  — exercised in-process = faithful; CLI subprocess framing (argv, exit codes,
  stream muxing) is the E2E corpus's job, already real-binary via `sh
  e2e/run.sh`. errorloom's `282` §7 subprocess runner is its own generic
  capability, proven by loom-mock-tool self-tests; Dorc's defining-case
  consumer need not re-prove it. RIDER: the `marker-version-unrecognized` pilot
  fires **world-as-pipeline** (the real in-process kernel `syntax→…→plan` over
  an in-memory marked-bad-version file), giving ONE real-fired proof inside the
  hermetic tier that in-process reconstruction matches real production. Do NOT
  add a subprocess-runner case to phase 4. The five roster codes stay
  world-as-payload (expensive/artificial worlds; phase-5 may upgrade).
- **`flag-mirror-fights-promote-v1`: ACCEPT** the `CatalogLookup`+`OwnedEntry`
  render-seat parameterization; reject the dorc-loom-reimplements-`render_cli`
  alternative (duplicate seat). `&dyn CatalogLookup` in the render path is
  fine — diagnostics are human-facing, never the network hot-path (perf-
  doctrine); dynamic dispatch costs nothing that matters.
- **`flag-roster-retire-needs-mini-backport`: ACCEPT** the phase-4 mini-backport
  of the five roster codes → case-owned; the roster retires FOR REAL in phase 4.
  The coupling is clean: exactly the codes losing roster protection gain
  fixpoint protection.
- **`flag-ratchet-redefine-not-regrow`: ACCEPT** the transient `fragment-covered`
  third state; `ratchet_only_shrinks` stays 35≤35, never weakened, never
  baseline-reset.
- **`flag-metadata-frontmatter-vs-const`: ACCEPT** frontmatter-derived
  `when-fires`/`why` for case-owned codes (`282` §8 letter; makes the Dorc-side
  byte-identical gate meaningful); case-less codes keep const carry-forward
  (accepted shrinking gap).
- **`flag-message-option-vs-sentinel`: ACCEPT** `message: Option<&'static str>`
  (illegal-states-unrepresentable); flag back only if it threads awkwardly.
- **`flag-editable-text-selection`: ACCEPT** the `--format=` sniff.
- Confirmed no-action: count is 21/63 (use it); BLESS-law intact (promote-v2
  orchestrator-only, gates are read-only); multi-paragraph stays unbuilt.
- **BOTH fixpoint gates required** (`28A` §2g rider stands): errorloom
  render-level (catches prose hand-edits) AND the Dorc-side
  promote→catalog-byte-identical (catches metadata hand-edits).

Phase 5 (backport of the remaining 15) + phase 6 (de-passthrough) are SEPARATE
later dispatches, not this executor's. The executor stops at the map's
step-7 checkpoint for conductor review before the flip.

## §2o — Generation-flip CHECKPOINT review + the flip go (conductor, 2026-07-20)

Steps 1–6 landed on `ai/r28-flip-exec` @ `cb99482` (1037 unit / 97 e2e / gates;
title-split span relocation proven byte-identical to `render_cli` + gap-free
through `to_errorloom` incl. the straddle case; five FakeGit Consumer scenarios;
world-as-pipeline marker pilot fires the REAL in-process gate with a real
spanned caret frame — the §2n rider discharged). Flag dispositions:

- **`flag-render-seat-threading-shape`: ACCEPTED as an IMPROVEMENT** — the
  `*_with` lookup-siblings + const-delegates shape is within §2n (one seat,
  parameterized, byte-identical) and edits fewer call sites than 283's
  thread-everywhere. Better than proposed; keep.
- **`flag-corpus-and-oracle-dep-sited-in-dorc-loom`: ACCEPTED, forced-correct**
  — world-as-pipeline needs `oracle::check_dialect_marker`; `core` can't dep
  `oracle`; 283 §1a flagged the contingency. Corpus + gates + the
  dorc-loom→dorc-oracle dep (core+syntax only, DST-clean) live in dorc-loom.
- **`flag-byte-identical-gate-is-partial`: ACCEPTED; whole-file gate DEFERRED to
  a conductor canonicalization act** — the literal whole-file
  promote→byte-identity gate is unreachable until `catalog.rs` is canonicalized
  (47/56 `example` fields are pre-promote hand-strawmen; literals hand-wrapped
  multi-line), and canonicalizing is `DORC_CATALOG_PROMOTE` = BLESS-law
  orchestrator-only, correctly NOT run by the builder. The achievable half
  (params-regen + carry + idempotence) is wired and passing. `28A:rul-catalog-
  canonicalization-is-conductor`: I run the promote on a fresh binary, INSPECT
  the diff (schematic examples replacing strawmen + single-line literals — watch
  for any semantically-richer authored example being flattened; preserve if
  found), commit the canonicalized catalog, THEN the whole-file byte gate
  tightens. Paired with the prose-authoring pass. NOT a flip precondition.
- **`flag-reholing-mangles-glued-params`: NAMED SEAM, human-surfaced at close;
  NOT a flip blocker** — backtick-GLUED params (`` `{which}` ``, every
  `whylog-*` code) straddle errorloom's dumb word-boundary, so prose-bless
  mangles their re-holing (bakes value + dup hole); space-delimited + foreign
  `detail` re-hole cleanly. Phase 4 unaffected (roster cases are
  world-as-payload, rendered-never-blessed). But it means whylog-family prose
  can't yet round-trip the transport. `28A:rul-glued-param-rehole-seam`: three
  candidate answers for the human (a) teach errorloom's tokenizer backtick =
  word-boundary — principled (shell-consistent), but reaches into the published
  crate's core word-model; (b) respell the glued-param prose space-delimited;
  (c) accept + document. Pairs with the prose-quality sprint (deferred,
  human-owned). Surface at round close, do not resolve.
- Count nit (4 stored `[unwritten:]`, not 5): noted.

**THE FLIP: GO (Option B — roster retires under render-level protection).** The
roster's five codes are already case-owned (step 6c); the render-level
`fixpoint_check` protects their prose (a catalog hand-edit moves the render off
the committed case bytes), so the `CONDUCTOR_AUTHORED` roster retires SAFELY on
that gate alone — the whole-file byte gate is additional metadata protection,
not a retirement precondition. Flip scope = retire the roster + re-key
`message_registers_are_sm_or_unwritten` to "sm | None | case-owned" + confirm
the wired metadata gate stays green. Small, per 283 §5.8. Phase 5 (backport 15)
+ phase 6 (de-passthrough) remain SEPARATE later dispatches.

## §2p — Flip LANDED + FOLDED + cold-verified (conductor, 2026-07-20)

Flip `a556b13` (roster retired; `message_registers_are_sm_or_unwritten` re-keyed
to filesystem `is_case_owned(slug)` — a REAL check: delete a case file and the
ex-roster code's unprefixed prose fails the gate). Folded @ `e072eef`. Phase 4
COMPLETE: catalog prose is case-file-derived + fixpoint-protected; the 5
`[unwritten:]` codes (`marker-version-unrecognized` + the 4 mark codes) await
the conductor prose pass.

CONDUCTOR VERIFICATION was cold-cache by necessity (see the infra finding): full
`cargo clean` (3.1GB) → rebuild clean → `clippy --all-targets -D warnings` ZERO
errors → 1037 unit → 97 e2e → gates ok → drift only the known lax-order
`.ran` (restored). HEAD genuinely clean.

- **`28A:finding-incremental-clippy-serves-stale-clean`** (infra, teeth) — on
  this Win/mise/worktree setup cargo/clippy incremental caching served
  STALE-CLEAN lint across per-step runs; 6 genuine latent clippy errors in
  steps 2–5 (broken doc-link; raw arithmetic vs saturating; unused-self;
  single_match; expect_used in non-`#[test]` integration helpers) surfaced only
  when the flip forced a recompile, fixed in `2e03ced`. CONSEQUENCE: the flip's
  intermediate commits are NOT individually clippy-clean on a cold checkout —
  only the folded HEAD is (I verified cold). BINDING RIDER for all remaining r28
  builder briefs in a worktree: run clippy FIRST on a cold cache, or
  `cargo clean -p <touched crates>` before the clippy gate — per-commit
  `conduct-bless`/`_gates.sh` clippy signal is otherwise untrustworthy here. The
  human's standing "codebase is slower than it could be" note + this = a real
  DX paper-cut for the round-close report. (Does NOT retroactively distrust
  earlier lanes: each was cold-verified at its own fold by me.)

## §2q — Prose + canonicalization DEFERRED to the human sprint (conductor, 2026-07-20)

The 5 `[unwritten:]` codes (`marker-version-unrecognized` + the four `281` mark
codes) STAY `[unwritten:]` at round close — a deliberate deferral, not an
omission, on the round's OWN lean `282:lean-machinery-now-prose-lazy` (machinery
now, prose lazy; `[unwritten:]` is a legal greppable designed state, ceiling 6;
the prose-quality sprint is a human-owned surface-stability-moment activity,
`282` §10). Grounds beyond the lean: four of the five are not case-owned yet
(phase-5 backport owes their case files; clean unprefixed authoring needs them
first per the re-keyed `message_registers_are_sm_or_unwritten` gate), and
`marker-version-unrecognized`'s metadata carries a `{found}`-vs-empty-`params`
mismatch that needs payload-shape verification before a template can safely use
it. Manufacturing interim prose the human rewrites in the sprint = wet-cement
churn. The empty-loop pilot DID demonstrate mint→`[unwritten:]`-render; the
author-from-render step is left to dogfic in the sprint.

Likewise **the catalog canonicalization** (`28A:rul-catalog-canonicalization-
is-conductor`, §2o): DEFERRED to the same sprint — it rewrites 47 `example`
strawmen → schematic + single-lines the literals, is a delicate 56-entry
`DORC_CATALOG_PROMOTE` diff-inspection, and the human is already in `catalog.rs`
for the prose sprint. The whole-file byte gate stays partial-but-passing
(params-regen + carry + idempotence) until then; the `FLAGGED` doc-comment
marks it.

## §2r — Round-28 completion plan (conductor, 2026-07-20)

Three deliverables LANDED: errorloom (complete, folded), syntax v0.2 respell
(complete, folded, cold-verified), integration-via-generation-flip (phase-4
DIRECTION landed — catalog case-derived, promote-v2 built, roster retired, loop
proven with pilots). Remaining charter-scope (`280` §3) mechanical completion,
sequenced SERIAL after the docs fold to avoid catalog.rs concurrency:
- **docs re-synthesis** — RUNNING (`ai/r28-docs-resync`; the live-harm item:
  teaching surfaces still show v0.1 over v0.2 code).
- **phase 5 (backport)** — the 21 covered codes → case files; retire the 63
  fragment goldens + `DORC_DEFINING_BLESS`; ratchet re-pointed. Migration only,
  prose-orthogonal (preserves `[unwritten:]`).
- **phase 6 (de-passthrough)** — mint the user-sourced-text taint type; audit
  the 18 ForeignText-ambiguous sites; our-words emit-sites → real templates.
  A correctness/hygiene property, not "more prose."
DEFERRED-by-design (explicitly lazy/human-owned; banked, specified): the
prose-quality sprint (5 unwritten + the `sm `-prose rewrite) · the catalog
canonicalization (§2q) · phase-7 ratchet burn-down + the records-* tail (`282`
§9 phase 7) · the glued-param re-hole seam ruling (§2o) · the errorloom LICENSE
+ publish flip. Round closes after phase 6 + the ledger/status/map sync.

## §2s — Human directives 2026-07-20 (scope finalization + next-turn queue)

Human-typed this sitting:
- **phase-6-de-passthrough KILLED** — an opaque sibling's lane owns the related
  (secrets/taint) work. Round-28 does NOT build it. Instead a brief
  mechanical-needs hand-off written to `Research/quarantine-DO-NOT-READ/284`
  (this round's taint-tracking owed/wanted work; NOT security analysis).
- **phase-5-backport GREENLIT** for next turn (after the human's ack). The 21
  covered codes → case files; retire fragment goldens + `DORC_DEFINING_BLESS`;
  ratchet re-point. Migration only.
- **Mark-code + `sm `-prose authoring → a FRESH FABLE under `sm ` rules**, NOT
  deferred-to-human and NOT this conductor (context full). The `282`
  transcript-authoring surface exists for it; `sm `-tier interim text is the
  posture. This RE-ASSIGNS §2q's "human sprint" for the prose half → Fable;
  the catalog canonicalization stays conductor/human.
- **root-doc spelling drift: DONE by the human** (the docs-pass inventory —
  README/DESIGN/USER_STORY `@`/v0.2 edits — is discharged; no AI action owed).
- **NEXT-TURN side-quest (after ack), low-priority**: a two-lane DeepSeek-only
  `/adversarial-crosscheck` of the PUBLISHABLE errorloom — dual remit (narrow
  Rust-user-taste code review + broad SWE published-library quality review),
  scoped EXCLUSIVELY to errorloom, NO mention of dorc, NO mutation. Low-reasoning
  agents; do not churn on issues. (DeepSeek = the `deepseek-reviewer` lane.)

## §2t — `.rs` comment-respell landing + two reported findings (2026-07-20)

Comment respell LANDED @ `791a8e7`, FOLDED (cold clippy 0 warnings, own-hand). The
misleading `mark_grammar.rs` header is corrected (now: cutover LANDED, module is
the `#[cfg(test)]` reference-spec for the future `Vec` block-model,
`281`-remains-THE-spec). `marker.rs:186`'s `v0.1` literal correctly LEFT (it's the
`marker-version-unrecognized` fixture; deliberately-unrecognized, comment fenced).
Two reported-not-fixed findings banked:
- **`28A:finding-old-prose-coupled-to-message-strings`** — core/cli/plan
  doc-comments still say `tolerates:`/`invariant:` because they are 1:1-coupled to
  catalog `message` STRINGS + render `format!` output that also still say v0.1.
  Respelling only the doc-comment would split doc-vs-emitted-text. This IS the
  `sm `-prose lazy lane → the FRESH FABLE prose pass (§2s) owns it, message + doc
  together.
- **`28A:finding-touches-rename-half-done`** (pre-existing, NOT this round's
  respell) — the `touches`→`disturbs` / `reaches`→`disturbance_reaches_only`
  sh-FUNCNAME-suffix migration is half-landed: production strip emits the NEW
  suffixes (`predict.rs` L119/152) but `predict.rs` docs L108-113/442-514 and
  hand-built sh fixtures in `cli/main.rs:5911` + `plan/lib.rs:4720/4738` still
  spell `apt_get__touches()`. Build is GREEN (fixtures pass — the funcname isn't
  asserted where they live), so LOW severity; distinct from the mark-grammar
  respell (Rust-side `touches`/`reaches` module/type/method names are
  deliberately retained). Owed a conductor/next-round investigation, NOT a
  rushed round-close fix (non-comment; would risk the verified build). Verify in
  other cells before touching.

## §2u — errorloom review (285) + phase-5 landed (conductor, 2026-07-20)

- **285-series COMPLETE + banked** (`bf40f11`): two DeepSeek lanes (`285b` taste,
  `285c` swe) → adjudication `285d`. Verified-kill: swe-F1 "Region unconstructable"
  (FALSE — enum-level non_exhaustive; dorc-loom constructs it; DeepSeek conflated
  enum/variant level). Ruled-reject: swe-F4 "make error enums exhaustive" (contra
  the standing taste ruling + the taste lane's own praise — KEEP non_exhaustive).
  Credited repairs → the last-polish pass (error-richness cluster · silent-parse
  fixes · cheap hardening · apply_field_edits-by-value); judgment-tier items
  (Display-vs-Debug dump, child-timeout) at builder latitude. Human-only forks:
  the non_exhaustive-error-enum publish-taste call; the LICENSE/publish flip.
- **PHASE 5 LANDED + FOLDED + cold-verified** (`4189e92`; builder tip `9817860`):
  16 covered codes backported to `dorc-loom/cases/` world-as-payload files (the
  count was 16 not 15 — the marker pilot had already entered covered(); benign
  doc-drift, `283 f-fragment-count` pre-flagged it); 66 fragment goldens +
  `DORC_DEFINING_BLESS` retired; completeness re-keyed to filesystem
  `is_case_owned` — partition case-owned(22) ∪ ratchet(35) == CATALOG(57),
  `ratchet_only_shrinks` untouched at 35≤35. Prose preserved verbatim (the four
  `mark-*` stay `[unwritten:]`; `sm ` stays `sm `). Cold verify own-hand: clean +
  clippy zero + conduct-bless green + e2e 97/97; only the known lax-order `.ran`
  drift, restored.
- **`28A:rul-keep-covered-with-drift-guard`** (rules the builder's escalated
  `tc`-call) — KEEP `covered()` + its two in-core tagged-render twins (the
  in-core byte-check of all 22 payloads is real breadth a core test can't get by
  depping dorc-loom; `283` §4c sanctions world-as-payload survival). The
  duplication (constructors in both `covered()` and `canonical_payload`) is
  accepted; the silent-drift gap the builder flagged (completeness doesn't force
  `covered() ⊆ case-owned`) closes with a ONE-TEST coherence guard
  (`covered()`-slugs ⊆ case-owned) — banked as a cheap round-close/next-round
  rider, NOT urgent (the lists are aligned now).

## §2z — ROUND CLOSE (conductor, 2026-07-20; stack `ai/r28-impl` @ `3897bb5`, awaiting human fold)

The `280` charter is built. Every lane conductor-cold-verified (cold mandatory per
`28A:finding-incremental-clippy-serves-stale`); at the close: cold clippy zero, 48
test suites, e2e 97/97, clean tree.

Three deliverables:
1. **errorloom** — the standalone publishable crate: d1 transport + d2
   runner/orchestration/CLI (`28A:rul-errorloom-product-cut` moved bless-orchestration,
   span-schema, generic gates, CLI, toy-consumer IN) + the `285` outside review
   (two DeepSeek lanes → `285d` adjudication → last-polish pass). `publish=false`;
   LICENSE + Cargo metadata + the non_exhaustive-error-enum fork are the human's.
2. **syntax v0.2 respell** — the `281` mark-grammar cutover, atomic, verdict-surface
   crosschecked byte-identical-modulo-`@`; `# dorc-lang/v0.2` stamped; single-mark
   production subset (block model reference-only, `28A:rul-single-mark-production-subset`).
3. **integration (generation flip)** — `282` phase 4 (catalog case-derived, promote-v2,
   both fixpoint gates, roster retired behind `is_case_owned`) + phase-5 backport (all
   covered codes case-owned; fragment goldens retired). de-passthrough (phase 6) KILLED
   → opaque sibling lane (`284` hand-off).

Plus: docs/steering/registry re-synthesis to v0.2; the janitor sweep (`28C`); the
ANALYZER-NEEDS union-merge + quarantine restock. Opaque-review NOT run (`28A` §4b
human exemption).

**The human deferred-queue (all banked, none blocking; do NOT let a successor rebuild these):**
- Fable `sm `-prose pass — the 5 `[unwritten:]` codes + the `sm `-corpus, under `sm ` rules
  (§2s; the `28A:finding-old-prose-coupled-to-message-strings` doc-comments ride it).
- Catalog canonicalization (`28A:rul-catalog-canonicalization-is-conductor`, §2o/§2q) — the
  `DORC_CATALOG_PROMOTE` diff-inspection that tightens the whole-file byte gate.
- Glued-param re-hole seam (`28A:rul-glued-param-rehole-seam`, §2o) — three candidate answers.
- `covered()⊆case-owned` drift guard (`28A:rul-keep-covered-with-drift-guard`, §2u) — one cheap test.
- errorloom LICENSE/publish + Cargo metadata + the publish-taste fork (`285d`).
- `touches`→`disturbs` sh-funcname migration half-done (`28A:finding-touches-rename-half-done`, §2t).
- The standing `TODO-ADDTL` tail riders (unchanged).

## §2z-post — Flagship errorloom case (post-close addition, human-directed 2026-07-20)

Folded, cold-verified (clippy zero, 48 suites, e2e 97/97 — zero golden churn).
The flagship: `crates/dorc-loom/cases/cmdsub-operand-top.txt` UPGRADED from
world-as-payload (spanless) to world-as-pipeline (caret) — a real pi-webhost book
whose `apt-get install -y "$(cat …)"` drives operand-3 to ⊤, fired through the
real in-process kernel, rendered with a precise caret frame + `= help:` + a
`--format=jsonl` machine-view second replay (multi-replay). Builder chose
`cmdsub-operand-top` over `site-unresolvable` (the latter's emit is cli-binary-
buried, unreachable as a library call). Prose is human-authorized for THIS file
only (real sentences, human will rewrite); every other `[unwritten:]`/`sm ` entry
untouched. TWO things rode in beyond the case file (both cold-green, each its own
revertable commit) — the human weighs at fold:
- **`28A:finding-flagship-caret-fix-is-production`** — commit `bbb9fc0` narrows the
  `cmdsub-operand-top` caret from whole-command to the exact operand word, at the
  ordinary-command emit site (analysis diag). A genuine improvement (tighter/honest
  attribution), zero golden churn (stderr-only, unpinned), revertable in isolation.
  But it IS a production caret-span change, not merely a case file.
- **`28A:finding-dorc-loom-deps-widen`** — `dorc-loom` now deps `dorc-syntax` +
  `dorc-analysis` (was core/errorloom/oracle). Inherent to world-as-pipeline pilots
  (firing a real diagnostic needs the kernel); both kernel-pure/DST-clean so
  determinism holds; but a real coupling increase on the test-consumer adapter
  (NOT the published errorloom, which stays dep-free). A `tc`-shaped watch-item if
  more pipeline pilots each pull more kernel crates.

## §2z-post-2 — Flagship polish, decision round (human-adjudicated 2026-07-20)

Six render/prose calls closed after the builder's first pass (`\__/` caret, gutter,
wrap-all-16 landed on `ai/r28-flagship-polish`; gutter now reworked per below). Full
spec rewritten into `282` §12; builder re-woken with these. NOT yet folded to `ai/r28-impl`.

- **`28A:rul-gutter-width-invocation-global`** — gutter width `W = max(3, maxDigits)`,
  `maxDigits` = digits of the LARGEST line-number anywhere in the invocation (all
  blocks), so columns never shift block-to-block; `|` at col ≥ 4 always. Right-align by
  default (ones-places match); when every rendered line-number shares ONE width ≤ 2,
  apply the slack aesthetic — 1-digit CENTERED (` 6 |`), all-2-digit LEFT (`60 |`); ≥3
  digits fill (`600|`/`6000|`); mixed widths right-align (`  6|` beneath `600|`).
  OVERRIDES the builder's butting `  6|` (its "human OK'd butting live" claim ≠ the
  written spec — I did not witness it). Mixed-1&2→right-align is my extrapolation (no
  human example); veto-able.
- **`28A:rul-connective-minimal-remediation-map`** — `= help:`→`= repair:` keyed on
  registry `RemediationClass`, MINIMAL: `ResolveDynamism`→"repair", else "help".
  CORE-side (production `dorc plan` also says `repair`; re-bless the ResolveDynamism
  help→repair golden churn). No fuller map exists — human tunes iteratively as errors
  surface (handling the whole message-category dynamically).
- **`28A:rul-blank-line-is-errorloom`** — the inter-block blank line is an ERRORLOOM
  `render_case` presentation choice (gated "another block follows", not trailing), NOT
  dorc-production output. A future dorc beauty-newline stacking to a double blank is
  accepted.
- **`28A:rul-following-ness-deferred-punt`** — renderer-owned terminal `:`/`.` is
  DEFERRED (my judgment; human delegated, flagged "may be the gnarly-punt class"). It
  risks the prose-bless byte-equality invariant (load-bearing for the flip gates) for a
  refinement the flagship already gets right with a baked `:`. Revisit when a genuinely
  `.`-terminal diagnostic (message, no source-span, no help) exists.
- **`28A:rul-command-name-typed-three-state`** — `{command}` is TYPED, not a bare
  `String`: static-literal / dynamic-but-const-prop-resolved / no-single-clear-name,
  threaded from the analysis site (value-flow known there, not synthesized late).
  Render: literal→name; resolved-dynamic→"This dynamic command-word, which resolves to
  `apt-get`, …"; unresolvable→fallback (no fill). Literal path end-to-end now; dynamic
  variants get TYPE+RENDER shaped now, analysis-population may be a marked follow-up if
  it needs real value-flow work. Human: name needed in MANY messages, so API naming
  must follow the eventual purpose. Escalate if scope leaves the kernel.
- case-rewrap: all 16 corpus cases stay rewrapped (human: nicer to edit in-editor).

## §3 — Lane map and state (final)

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

## §4b — Standing exemption (HUMAN-TYPED, 2026-07-20; survives rewind — do not re-derive from AGENTS.md)

Root `AGENTS.md` gained a "Memetic hazards and information hygiene" section
requiring Fable-class conductors to deploy `.claude/skills/opaque-review` at
end-of-work. The human TYPED, same sitting: **it does not yet apply to this
round's conductor** — the reviewer infrastructure is not yet in place. So: the
round-28 conductor does NOT invoke opaque-review at round close; the directive
presumably binds future conductors once the human says the infrastructure
exists. A rewound/compacted successor conductor reading AGENTS.md (or seeing
the skill listed) must honor THIS typed exemption over the document's text for
round 28. (The root-AGENTS edit itself is human-authored, single-auth; it flows
into branches at the human's own fold — never copy it into the worktree.)

## §5 — Dispatch log

- 2026-07-19: `lane-errorloom-crate` d1 (Opus, worktree, bg) — transport engine.
- 2026-07-19: `lane-syntax-respell` phase A (Opus, worktree, bg) — proposal note
  `28B`, no engine edits.
- 2026-07-19: janitor (Opus, primary checkout, bg) — survey + guarded cleanup +
  dropped-work report; report to conductor scratchpad, findings banked here after.
