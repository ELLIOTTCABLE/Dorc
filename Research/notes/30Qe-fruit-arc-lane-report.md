# 30Qe -- lane-fruit-arc builder report

> Tier: Sonnet builder, one-shot, `26K` sect0a fruit arc (4 items). Branch `ai/r30-lane-fruit`,
> based on `ai/main` at `aabcc2d9`. +SURE unless marked. STOPPED EARLY on explicit human
> authorization mid-turn (context pressure) before the completion gate finished; see
> `human-injected-stop` below -- the conductor should read that paragraph first.

## human-injected-stop

Mid-turn, the human sent two messages directly into this session (not through the conductor):
one authorizing me to stop before finishing the dictated `mise run both gate:full-quiet` gate
given context-exhaustion risk, and a second instructing me to tell the conductor explicitly that
(a) this stop was human-injected, not my own judgment call, (b) the conductor is authorized to
re-assign any incomplete work, and (c) the conductor should NOT re-awaken this builder session.
Recording verbatim intent here per that instruction. Working tree is clean (everything below is
committed); nothing was left dirty or half-applied.

## What landed (11 commits, `ai/r30-lane-fruit`, oldest first)

```
6b0a4a8e (AI new) Detect paste-hazard lines (tty-cap length, leading tilde) in renders
24f8ead8 (AI new) Flag a for-loop whose word-list is one brace-range literal
85808626 (AI new) Publish the catalog entry for the brace-range loop finding
73f34fce (AI new) Warn when a loaded oracle never matches a site
df6dfe23 (AI new) Publish the catalog entry for the zero-sites oracle warning
5ff84f7b (AI new) Whole-product round-trip case for the zero-sites warning
17e98b6d (AI re)  Tighten comments toward the lane's inline-narration budget
e749b29d (AI fix) Route the loop-brace-range case through a valid oracle shape
f32ae2a3 (AI fix) Re-publish the catalog entry after the case content fix
f2eaa0cc (AI fix) Backtick a doc-comment identifier for clippy
9d210ae3 (AI fix) Backtick a doc identifier and rename a shadowing binding
```

Comment budget (`git diff ai/main`): inline `//` narration net-new = **24** (budget 25, under).
`///` doc-comments net-new = **76** (sized to pin count -- 3 new `DiagCode` payload structs +
their catalog wiring + a new `Vouches` accessor + a new lint-source module + a new
`plan::render` module, each doc-commented per project convention).

## Item 1 -- `fruit-emit-hygiene-paste-rules` (DONE, no deviation)

`plan/src/render.rs`: `CANONICAL_TTY_LINE_CAP_BYTES` (4095), `PasteHygieneHazard` enum
(`LineTooLong`/`LeadingTilde`), `paste_hygiene_hazards(rendered: &str) -> Vec<...>` -- a pure,
detection-only scan (never rewrites bytes, satisfying `two-surfaces` by construction: nothing
calls it to mutate). Pinned in `plan/tests/render_corpus.rs` with 3 tests: a line at the cap
(CFG shape: one top-level unmodeled `Simple` leaf, no edit, ships byte-identical), a line
beginning `~` (same shape), and a negative control (an ordinary render carries zero hazards).

**Open item, not resolved (report-only):** the item's own text says "A violating render is a
refusal/diagnostic, never a silent rewrite" -- I built the DETECTOR only, not a live
`DiagCode`-catalog integration wiring it into `render_apply`'s call sites. Minting a full catalog
entry (the `defining-case-catalog` machinery: enum variant + payload + slug + `CodeSpec` +
`params_of_raw` + a `.loom` defining case + `dorc-loom publish`) is real, multi-file work I *did*
end up doing twice more in this lane (items 2 and 3), and each time it ran well past the item's
own "if it needs more than ~100 lines, stop and report" framing once you count the catalog
plumbing. Item 1's own wording never explicitly said "mint a DiagCode" (unlike items 2 and 3,
which do), so I read the detector-plus-tests as the item's actual ask and left the
refusal-wiring as a scoped-out follow-on. Flagging for conductor judgment: wire
`paste_hygiene_hazards` into a real refusal at the `render_apply`/cli report boundary, with its
own catalog entry, as a small follow-up lane item if wanted.

## Item 2 -- `fruit-oracle-matched-zero-sites` (DONE)

`plan/src/lib.rs`: `Vouches::vouched_fn_names(&self) -> BTreeSet<&str>` -- reads the crate-private
`by_establish` map (only reached vouches live there), so it costs nothing new to compute; License
values flowing into narrative freely (`two-plane-aid-law`), never back.

`cli/src/main.rs`: `oracle_matched_zero_sites_diagnostics(oracle_paths, verdict_sets, vouches,
interner)` -- index-zips `oracle_paths` (oracle-only prefix) against `verdict_sets` (source-wide,
oracle-first per `the-book-is-a-definition-source`), munges each file's declared
`__is_converged` provider names, and checks non-membership in `vouches.vouched_fn_names()`.
Wired via `report_at(advisory, "oracle", None, ...)` right after `vouches` is finalized
(post-`build_wrapped_vouches`), reusing the FROZEN, once-built vouch set
(`the-fixpoint-owns-the-rounds`: vouches never rebuild per round) -- never a second lift.
Verified against BOTH a positive (oracle command never invoked -> warns) and negative (oracle
command invoked and reached -> silent) real-binary manual test before committing.

`aid/src/diag.rs`: new `DiagCode::OracleMatchedZeroSites(OracleMatchedZeroSites { oracle: String
})`, spanless (added to `SPANLESS_SITE_PAYLOADS` -- the claim is about the whole FILE), `Severity::
Warning`, `Floor::None`, `RemediationClass::ProvideModel`, `message: None`.

**Catalog-defining case is fixture-backed, not render-fixpointed** (found, not a deviation from
spec -- matches an EXISTING project pattern): `dorc-loom`'s in-process `DorcConsumer` replay
driver has no seat to reconstruct a whole-run `Vouches` aggregation for a single-file `code:`
case (confirmed empirically: the render genuinely returns `Err` for this shape, not merely a
missing slug). `aid/tests/oracle-matched-zero-sites.loom` is therefore a fixture-tier case
(`aid::fixture::canonical_payload`, matching the EXISTING precedent for the records-lane's
refusal codes and `AidUnloadedSiblingOracle`/`RoleFamilyContested` -- all main.rs-only
diagnostics with the identical structural gap, per `fixture.rs`'s own doc comments). The REAL
firing route is proven by a genuine whole-product round-trip case,
`cli/tests/oracle-matched-zero-sites-round-trip.loom` (`run: round-trip`, `fixpoint: executed`,
`expect-diagnostic: [oracle-matched-zero-sites]` -- asserts the warning ACTUALLY fires on stderr
during a real subprocess `dorc` invocation; scoped-bless-minted, verified green standalone and
as part of the full e2e/loom corpora before commit).

**Naming judgment (argued, not resolved):** `RemediationClass::ProvideModel` ("extend/fix an
oracle so the tool is no longer unmodeled") is an approximate fit -- the honest remediation when
the book DOES invoke the family is "widen the oracle's own argparse/verdict coverage", which
IS provide-model-shaped, so I judged this a genuine fit (unlike item 3's naming problem below).

**Types, product-wide:** `Vouches::vouched_fn_names` makes representable "which providers this
run's vouch set actually contains" without a second traversal/lift; it does not change what
states `Vouches` can hold (still admits the same set of `(site, fact) -> ByVouch<VerdictVouch>`
entries) -- purely an additive read projection, so no bad-state changes either direction.

## Item 3 -- `fruit-loop-does-not-loop-lint` (DONE, stayed inside the boundary law)

Confirmed pure lint-crate rule over EXISTING parse output before building: `lint/src/
source_portability.rs`'s `LoopBraceRange` source calls `dorc_syntax::parse` (the same parser
every stage uses) and walks the AST once via `Ast::iter()` -- zero new walking primitive, zero
`dorc_analysis`/`dorc_plan` touch. Detects `for X in {A..B[..C]}` (a for-loop whose ENTIRE
word-list is one literal brace-range word) via `is_brace_range` (hand-rolled, no new dependency).
Registered in `lint::source::registry()`. New `DiagCode::ForLoopBraceRangeRunsOnce { range:
String }`, spanned (the offending word's span), `Severity::Warning`, `Floor::None`.

Test: `lint/tests/loop-brace-range-runs-once/` -- the FIRST real instance of the `X/cmd` shape
(`spike/CLAUDE.md flat-test-tree-and-loom-placement`), scoped-bless-minted, clean golden (no
noise), verified standalone and in the full e2e corpus.

**Naming judgment (argued, unresolved -- flagging for conductor):** none of the four
`RemediationClass` variants name "fix a shell-portability mistake independent of any Dorc
model"; I used `ResolveDynamism` as the closest available and said so in both the code comment
and the catalog `why:` text. This is a genuine categorization gap, not a naming mistake on my
part -- worth considering a fifth `RemediationClass` variant if more pure-portability lint rules
land later (this lane's boundary law forbids me widening that enum myself; flagging up).

**Finding (out of my lane, report-only): a latent `dorc_oracle::predict` lexer bug.** Authoring
this case's catalog-defining `.loom` surfaced that `dorc-loom`'s in-process lint replay driver
(`lint::production::lint_materialized_source`) deliberately treats its ONE materialized source as
BOTH lint target AND oracle candidate ("this is the author-facing `dorc lint oracle.sh` lane" --
its own doc comment). Feeding a brace-range `for` loop through the oracle PREDICT-lift parser
(not the main `dorc_syntax` parser my lint source uses) produces a spurious `error
[predict-out-of-dialect]: empty command` pointing at the `{` in `{1..10}` -- confirmed absent
under the real `dorc` binary (manually verified: `dorc lint <file>` on the identical content
produces ONLY my finding, cleanly) and absent when the SAME content is fed through
`AnalysisDiagnostics`'s plain `dorc_syntax::parse`+`cfg::build`. I ~SUSPECT the oracle predict-lift
parser (a narrower, separate lexer/parser from the main `dorc_syntax` one, used only for
`__predict`/`__is_converged` body lifting) misreads a for-loop's brace-range WORD as the opener of
a `{ ... }` compound-group construct. This is `dorc_oracle` territory, explicitly outside this
lane's boundary ("anything touching analysis/effect/plan kernel crates" -- oracle predict-lift is
adjacent kernel machinery, not lint-crate); I worked around it by wrapping my catalog case's
demonstration content in a valid oracle-shaped function body (the false finding still fires
inertly beside mine and doesn't block anything -- `dorc-loom`'s OWN test suite passed with this
shape), but the underlying oracle-lexer defect is real and un-investigated further. Recommend
routing to the kernel batch if anyone wants it chased; I did not open a Research/ note for it
beyond this paragraph given the lane's scope.

## Item 4 -- `fruit-doc-no-secrets-payload` (NOT WRITTEN TO A FILE -- deviation, deliberate)

Investigated `spike/docs/` (the `writing-oracles/`/`running-books/`/`reference/` human-facing
tree) per the item's own "authoring doc" hint. `spike/docs/CLAUDE.md` says this tree is
"generated-and-maintained by agents" in general, but my BRIEF's standing rider is more specific
and more binding for this lane: "Builders author ZERO user-facing prose... never edit any
CLAUDE.md, AGENTS.md, or root human doc; PROPOSE steering sentences... for the conductor to
apply." I read that as overriding the doc-tree's own general permission for THIS lane, and did
not write to any file. Also found: NOTHING in `spike/docs/` currently teaches `dorc compile` /
cloud boot-payload delivery / IMDS-readability at all (grepped `payload|user-data|cloud-init|
dorc compile|IMDS` across the whole tree, only false-friend "payload" hits in an unrelated mark-
grammar sense) -- so inserting a bare secrets-warning paragraph anywhere risks violating
`spike/docs/CLAUDE.md`'s own "a concept must not be used before the point where the path teaches
it" style law, since the underlying concept (`dorc compile`'s offline/boot delivery face,
`26K` sect "concl-offline-compile-is-a-face-not-a-product") isn't taught anywhere yet.

**Proposed paragraph** (self-contained, teaches the concept inline so it doesn't require a prior
lesson; ASCII-only per `spike/docs/CLAUDE.md` style law), for the conductor to site:

> Never write credential material into an oracle body: check bodies, predict bodies, and
> everything else you author compile into the same artifact Dorc hands to a target, and on the
> major clouds that artifact can travel through boot-time delivery channels (cloud instance
> metadata and user-data services) that any process on the running instance can read, not only
> the one Dorc itself invoked. Treat every byte you write as though it will end up on that
> channel: code and read-only checks are safe there by design, but a password, API key, or
> private key baked into a check body leaks to anyone with a shell on the target the moment it
> ships.

**Recommended siting:** `spike/docs/reference/oracle-contract.md`, section 1 ("The standing rules
of the world") as a new bullet -- that section is already a flat list of non-negotiable rules in
the same register (dense, no jargon-before-teaching), and this rule genuinely IS one ("never
suspended, never traded against performance"). Secondary candidate:
`writing-oracles/03-the-probe-contract.md` (teaching-tier version) if the conductor wants the
gradual-enhancement-arc version instead of the reference-tier one. Cites `26K:concl-
offline-compile-is-a-face-not-a-product` ("no credential material in payloads (IMDS-world-
readable)") as the source finding.

## Register/steering proposals (for the conductor)

- FORFEITS/ANALYZER-NEEDS/AID-NEEDS: none proposed -- none of the four items touch a forfeit-shaped
  conservatism or an analyzer-need.
- `spike/crates/plan/CLAUDE.md` "Law -- render": propose a new bullet citing
  `paste_hygiene_hazards`/`CANONICAL_TTY_LINE_CAP_BYTES` under `30Qe:fruit-emit-hygiene-paste-
  rules`, once/if item 1's open refusal-wiring item lands.
- `spike/crates/lint/CLAUDE.md`: none exists; if one is ever minted, `source_portability.rs`'s
  `is_brace_range` pure-AST-walk pattern is a clean template for future syntax-only lint sources.
- Item 4's paragraph above, sited per the recommendation.

## `tc-*` judgment calls

None identified as cross-cutting/winner-shifting/license-widening in the `tc-*` sense -- all four
items are non-kernel, aid/lint-plane-only, and none touch a licensing chokepoint.

## Gate status at stop

`mise run both gate:full-quiet` was run to fix real failures iteratively (preflight RAM
transient, then two real clippy `doc_markdown`/`similar_names` findings, both fixed and
committed -- `f2eaa0cc`, `9d210ae3`). The FINAL foreground re-run was in progress (had cleared
`preflight`, `hk check --pr`, `hk check --staged`, and had started `hk check --unstaged` with no
failures observed at any point) when the human's mid-turn stop landed; the WSL leg of `both` and
the full `test`/`test:e2e`/`test:looms` re-confirmation after the last two commits were NOT
re-run to completion. Everything committed passed its own targeted verification before commit
(`cargo check`/`clippy` per touched crate, `cargo test -p dorc-aid`, `mise run test:looms-quiet`,
`mise run test:e2e-quiet`, all green at the time each commit landed) -- the risk surface left open
is narrow: whether the LAST two commits (backtick/rename-only clippy fixes, no logic change)
interact badly with something `gate:full-quiet`'s fuller battery catches that targeted checks
didn't. I judge this low-risk (`~SUSPECT`) given the changes are doc-comment/identifier-only, but
it is unconfirmed. `mise run xfail:census` was not run.

## Confidence

+SURE: items 1-3's implementations are correct and match their briefs (each independently
verified against both real-binary manual tests and the automated corpus before commit).
~SUSPECT: the final gate leg (WSL + full re-test after the last two trivial fixes) is clean --
plausible but unconfirmed.
-GUESS: item 4's recommended siting is the RIGHT one among the two candidates named; a doc-tree
specialist might prefer the teaching-tier page instead.

## §close

Branch `ai/r30-lane-fruit-2` (from `306a7a00`). +SURE unless marked.
Item 1: `DiagCode::EmittedLineUnsafeForPaste` (`line` + typed `PasteHygieneHazardReason`
{`LineTooLong`,`LeadingTilde`}), called in `cli/src/main.rs` right after `with_plan`, scanning
`artifact.primary().bytes` (the finalized emitted artifact, not the raw `render_apply` string).
Catalog case is FIXTURE-tier (`aid::fixture::canonical_payload`) -- same gap as
`OracleMatchedZeroSites`: the emit site lives in `main.rs`'s `run()`, unreachable from the
in-process consumer (empirically confirmed: a genuine `book.sh` case rendered nothing). Round-trip
case proves the real route via a plain unmodeled `~doesnotexist` line -- simpler than the brief's
oracle+mocks sketch, same call site, no exec needed (DEVIATION, judged equivalent). Reason
components hand-seeded `words: None` in `arrangement_lock.rs`, matching the
`solver-consistency-failure-*` precedent.
Item 2: `RemediationClass` is a plain hand-authored enum, not a `@generated` lock -- widening it
costs one compiler-forced arm plus one more unwritten arrangement row, judged NOT
lock/registry-governed. Minted `RemediationClass::RepairAuthorship` ("the author's own sh is wrong,
independent of Dorc"); re-pointed `ForLoopBraceRangeRunsOnce`. Render-invisible for that code
(`HelpRegister::Absent`), confirmed via its unchanged e2e golden and defining-case fixpoint.
Commits: `461b220a` wire detector, `47c553ea` publish catalog, `a5e9cba3` round-trip case,
`5843df8c` RepairAuthorship + repoint.
Gate: `gate:full-quiet` GREEN both legs (Windows then WSL, foreground). `xfail:census` unchanged
(17 live pins). `check-quiet` clean. Item 4 (doc paragraph) from the prior report untouched.
