# 30Ia — The static-loading/bundle-emission conduct ledger

> Conductor ledger for the `Research/plans/30I` build lane. `30I` is the semantic
> authority and the work order; this file is *state* — who is running, what was
> adjudicated, what is owed. Crash-resistant by design: compress or delete rows
> that stop mattering. Nothing important may live ONLY here.

## §1 — The seat

- Conductor: Opus-class, session opened 2026-08-18, dispatched from the primary
  checkout; conductor worktree `.claude/worktrees/r30-loading`, branch
  `ai/r30-static-loading`, cut from `ai/r30-loader-bundle-design` @ `80faf71d`
  ("(AI dsn) Turn static loading into a one-builder dispatch").
- Remit, as given: implement `30I`. The top implementation section's *phrasing*
  is conductor-tunable; the product spec is fixed, and difficulty in reaching it
  is a stop-and-report.
- Sibling lanes, untouched: `ai/r30-conduct` (kernel wave-two),
  `ai/r30-loom-surface` (loom authoring). `ai/r30-loader-bundle-design` is the
  lineage this branch folds back to.

## §2 — Baseline

`gate:quick-quiet` (retired since; the lifecycle rungs are now pre-commit /
`both gate:full-quiet` / `gate:arc`) measured GREEN by the conductor's own hand in this
exact worktree at `80faf71d`: 1354/1354, 0 skipped. Any red a builder reports is
theirs, not inherited.

## §3 — Dispatch decision (conductor call, 2026-08-18)

`dec-dispatch-one-builder-not-in-window` — the human's mild lean was in-window
work, conditioned on "if you think it can all fit without context-collapse". The
call went the other way, on scale measured rather than assumed:

- the lane's seats span `plan` (25k LOC), `cli` (23k), `oracle` (20k),
  `analysis` (17k), `aid` (15k), `core` (6k);
- three of its seven work items are NEW subsystems, not edits: the bundle
  projection, the multipart artifact set with atomic publication, and the
  generalized locator DAG. Scouted: today's artifact is stdout-only, and no
  bundle or artifact-directory machinery exists anywhere in the tree;
- iterative compile/gate churn on a lane this wide is exactly the traffic that
  costs a WINDOW rather than a budget, and a collapsed conductor cannot
  adjudicate the deviations the lane is certain to produce.

The conductor seat stays clean for the fold. Brief at `_tmp-30I-brief.md`
(untracked, worktree-local, regenerable from `30I` + §4). `30I:impl-one-builder-one-lane`
was read as forbidding phased sub-lanes at dispatch; §4a is the human's later
softening and governs — the intent (one resolver, one source-order model) still
binds, the one-window claim does not.

## §4 — Conductor scouting, banked (the four crossings handed to the builder)

Measured in-tree at `80faf71d`. These are the value the conductor seat added over
handing `30I` across bare; if a successor re-derives them, that is wasted time.

1. `fnd-30G-b8-gap-is-closed-by-replacement` — `30G` §4 item 8 STOPPED believing
   the only exits were "keep requiring the sourced tree" or "neutralise a book
   `.` line, which is a `rul-attention-honesty` ruling no component may make".
   `30I` §7.1/§7.3 rules a third that `30G` did not have: the book `.` naming a
   contracted dorc-lang root is REPLACED at its own source position by a `.` of
   the generated bundle — the ordinary replace mechanism, with `plan.sh` visibly
   naming every bundle at its original source point. Attention-honesty holds by
   construction, not by exception. `FORFEITS:forfeit-book-sourcing-walls` stops
   binding on the dorc-lang half; the non-dorc-lang half stays walled and falls
   to `30I` §7.1 mode 3.
2. `fnd-command-v-walls-and-loads-on-one-line` — the same construct answers two
   questions and neither answer may contaminate the other. `command` is an
   explicitly punted oracle, so `FORFEITS:forfeit-command-v-poison-wall` is
   half-irrelevant to the LOADER-ANALYSIS machinery — a `command -v` at marked-file
   top level feeding an include guard is load-time control flow the loader
   evaluates (`30I:force-guarded-fallback`). But it remains TRUE that a
   BOOK-position `command -v` walls, preventing elision on all content under that
   command's branch [human-typed correction, 2026-08-18]; `load30-subshell-errexit-fallback`'s
   book spells exactly that. The wall is correct and `silence-licenses-nothing`
   forbids un-walling it; needing to un-wall `command` to green a specimen is a
   licensure widening routed to the human (that forfeit's CAPTURE row), never lane
   work. Specimens are safe either way: `expected.ran` asserts run SETS, and a
   walled line still runs. `load30-two-point-frames` is the loader-side
   discriminator — one guard must answer FALSE ambiently and TRUE inside the
   subshell that pre-defines `sm_pick`.
3. `fnd-e2e-analysis-cwd-is-unresolved` — the specimens spell
   `SM_ORACLE_ROOT=crates/cli/tests/load30-…`, relative to `spike/`; the harness's
   `dorc` invocation (`e2e.rs` `fn dorc`) sets no `current_dir` and so inherits
   Cargo's PACKAGE-root cwd (`spike/crates/cli`); the execution rail deliberately
   runs artifacts from a throwaway sandbox. Under `30I:rul-dot-resolves-as-sh` the
   analysis cwd and the execution sandbox cwd are separate modeled questions.
   Absolute roots are not an escape — transcripts may not carry machine-specific
   absolute paths (`282` §7). The builder owes a stated pin and its reason.
4. `fnd-sourcing-seat-is-pre-factored` — `spike/crates/cli/src/sourcing.rs`
   names `resolve_against` in its own doc-comment as the single function to change
   for sh parity, and argues the sourcing-file-relative case at length. `30I` §3.2
   rejects that argument; the rule and its test are removed rather than kept for
   compatibility with an unreleased mistake, and the module doc is rewritten to
   current truth (`plans-current-tense-only`, no supersession annotation). The
   doc's own measured objection — that `28M` §7's helpers-plus-thin-entrypoints
   package becomes unreachable under cwd parity unless a cwd is pinned — IS
   crossing 3, now owned rather than routed around.

## §4a — The handoff authorization [human-typed, 2026-08-18]

`dec-one-builder-order-softened-to-a-seam` — the human read the brief, judged one
context-window unlikely to carry the lane, and AUTHORIZED (not dictated) a
mid-lane handoff: builder 1 targets a completion-of-work and a durable product;
builder 2 boots from that product and closes.

What is relaxed and what is not. `30I:impl-one-builder-one-lane`'s INTENT still
binds absolutely — no second resolver, no second source-order model, no
decorative source map. What is relaxed is only the claim that one window must
carry it. The seam is placed to serve the intent rather than to fight it: a
successor consuming a typed, documented load answer is LESS likely to re-resolve
than one exhausted builder improvising an emitter at the end of its window.

The seam is FIXED, not builder-judged: **builder 1 lands work-order items 2–4 and
stops; builder 2 carries items 5–7.**

`rul-never-trigger-on-self-assessed-context` [human-typed, 2026-08-18] — the seam
was first issued with a floor/preference/ceiling and a "declare past roughly
60–70% context" trigger. That trigger was WITHDRAWN within the hour: a model has
no reliable read on its own context usage, so it was not an instruction a builder
could follow, and the latitude it gated was therefore unevaluable too. The
conductor error was writing a self-assessment gate at all; the repair is a fixed
target chosen conservatively, because where the window actually sits is exactly
what nobody can measure. Binds every future brief: never gate a builder's control
flow on its own resource introspection.

The conservative pole was taken deliberately. Two consequences ride it:

- Item 4's force-now proof is DEFERRED, and is explicitly not a builder-1 stop
  condition. `30I` work-order §4 wants one locator chain through a real
  diagnostic render; the interesting chain runs through the bundle, which is item
  5. Builder 1 proves the DAG on a chain that exists today (book source span →
  planned replacement, two stages) and records the residual as builder 2's first
  task. This is the priced cost of cutting at the floor rather than after item 5.
- Item 7 (promotion, e2e lowering, golden-drift enumeration) belongs to whoever
  CLOSES the lane. It is atomic; half-done lies about what is proven.

`Research/notes/30Ib-static-loading-lane-report.md` is consequently a HARD
deliverable rather than insurance: it is the whole mechanism by which the seam
does not become the second resolver `30I:impl-one-builder-one-lane` forbids.

Handoff artifact: `Research/notes/30Ib-static-loading-lane-report.md`, maintained
LIVE from the first slice and committed with it — as-built seats and their
consumption API, what is pinned where, deviations left OPEN, open questions, exact
next steps. Not a re-plan of `30I`, not a chronology.

Conductor's read, for the record: ~70% that one builder could not have closed
this cleanly. The
volume sits in items 6–7 (a new artifact subsystem in a 23k-LOC crate whose only
artifact surface is stdout today, then golden churn across a corpus whose quick
tier alone is 1354 tests, twice, on two platforms). The discipline costs builder 1
almost nothing if it turns out unnecessary. Its one real cost is a successor
re-paying onboarding — which is exactly what `30Ib` is for.

## §5 — Live state

- **Builder lane `lane-30I-static-loading`: BUILDER 1 RETURNED 2026-08-18** at
  `4c5cd055`, tree clean. Items 2–4 landed bar one clause (§8 row D); items 5–7
  untouched. Builder gates: `both gate:full-quiet` W 2287/2287 · WSL 2283/2283,
  zero golden drift, ~40 new native tests. Handoff artifact
  `Research/notes/30Ib-static-loading-lane-report.md` is written and substantive.
  Conductor has NOT re-run the gates (that is the fold's own-hand act, still owed).
- Conductor hands are OFF this worktree while the lane runs (shared `target/`);
  ledger commits go in by explicit pathspec only, which has coexisted cleanly
  with the builder's own commits so far.
- **HARD GATE [human-typed, 2026-08-18]: HOLD after builder 1 returns, for the
  human's ack. Builder 2 is NOT dispatchable on conductor judgement alone.** This
  binds any successor conductor resuming from a collapse: builder 1's report,
  its deviations, and `30Ib` go to the human first, and the seam is crossed only
  on their typed word. Do not read the seam's existence in §4a as standing
  authorization to cross it.

## §6 — Owed at the fold

- Adjudicate every disclosed deviation as an OPEN item, re-derived from the
  global picture — never as a resolved footnote (conductor-skill law).
- Conductor's own-hand `mise run both gate:full-quiet` over the folded tip
  (never-vouch: a builder's green is reported, not borrowed).
- Steering prose: the builder PROPOSES `CLAUDE.md` text; the conductor judges
  whether it earns a permanent seat in every future context, and writes it once.
- `LIVING_STATUS` CURRENT STATE block; `FORFEITS` rows that this lane discharges
  or rewrites (at minimum `forfeit-book-sourcing-walls`); `30I` §14 pin status.
- The human queue items `30I` does NOT close stay open — `LIVING_STATUS`'s
  2026-08-17 queue minus items 2 and 3.

## §8 — Builder-1 return: conductor adjudication (ALL OPEN; awaiting the human)

Builder 1's six disclosed deviations, re-derived rather than accepted. `30Ib` §4
carries the builder's own account; this section carries the conductor's verdict
and, where one is owed, the conductor's own error.

- **A · `dev-analysis-cwd-pinned-to-the-case-dir` — pin SOUND, consequential
  spec-edit is the HUMAN's.** Pinning analysis cwd to the case directory is the
  right answer to `fnd-e2e-analysis-cwd-is-unresolved`: cargo's package-root cwd
  resolves nothing, `spike/`-relative ties every case to repo layout and cannot
  work for a materialized loom case, and the case dir is the shape an admin
  actually gets by running `dorc` where their files are — the test shape matching
  the product shape. Verified by conductor hand: the whole lane touched the
  committed specimens in exactly three places, one line each,
  `SM_ORACLE_ROOT=crates/cli/tests/<case>` → `.`; `expected.ran`, `XFAIL`,
  `head-expected.ran`, mocks and every `floor30-*` byte untouched. CONDUCTOR
  ERROR: the brief said "pin the analysis cwd deterministically" and never said
  that a pin forcing an edit to the human's authored specimen is a STOP. The pin
  forced the re-spell; the re-spell is ratifiable but was never the builder's to
  ratify.
- **B · `dev-slash-less-dot-is-now-unresolvable` — correct, and the residue is the
  lane's most dangerous artifact.** `pin28-variable-resolved-source-loads` now
  pins the opposite of its own header prose with GREEN goldens, because the site
  walled either way. That is invisible drift: a future agent reads the prose,
  believes the property is pinned, and it is not. The builder was right that
  re-spelling and re-blessing is golden work and out of remit, and right to pin
  the property natively both ways meanwhile. RULING SOUGHT: this must be
  discharged BEFORE the lane closes, listed in item 7 rather than left to be
  noticed.
- **C · `dev-guard-false-direction-fenced-to-role-names` — flagged not settled,
  which is correct; and it surfaces a genuine collision in `30I` itself.** The
  guard decides TRUE from a live definition unconditionally, but FALSE only for a
  role-shaped name (`28M:dec-decidable-set-v0`'s warrant: nobody ships a binary
  called `apt_get__is_converged`). `30I:rul-include-guards-are-load-semantics` is
  TYPED and its own §2.2 example guards an ORDINARY helper name
  (`example_common_query`). Deciding FALSE there models the fallback as loaded on
  a host where a like-named binary answered `command -v` and the other branch
  ran — `271:rul-sin-ordering`'s mis-attributed class, the worst cell. So the
  typed rule and the safety direction genuinely collide, and the safe carve is
  narrower than the rule as written. Withholding meanwhile (⊤ binds; nothing
  licenses off ⊤), so nothing is unsafe today. HUMAN's, per `inv-superposition`.
- **D · `dev-cross-custody-refusal-not-built` — the reason that matters STANDS;
  the reason offered first does NOT.** Reason (b) is sound and sufficient: `30I`
  §3.4's `dependency-explicitly-selected` case is a licensure widening against
  human-ruled `rul-vouch-reaches-own-custody-only` (verified: the slug is real
  law, cited across `FORFEITS`, `oracle/CLAUDE.md`, and two code seats), and
  building a classifier with no consumer is the multi-phasic scaffolding this
  project's own law warns against. Reason (a) — "golden movement was removed from
  my remit" — is an OVER-READ; golden movement was never forbidden, only item 7's
  promotion/lowering/enumeration sweep was. CONDUCTOR ERROR: the ceiling was
  spelled "do not start item 7 (promotion, e2e lowering, golden-drift
  enumeration)", which a builder can read as "no golden may move". Also noted:
  under a typed-ruling collision the brief's own stop condition fired, and the
  builder continued rather than stopping — defensible only because the seam meant
  it would report within the same lane, and not a precedent.
  **[TYPED RESOLUTION 2026-08-18]:** `30I` §3.4 now admits the canonical guarded
  source as one speaker edge only under exact target-closure and reached-vouch-path
  `Must` resolution. It does not admit generic ambient dependency injection;
  unannounced cross-custody calls still refuse pre-network. The broader guard/load-
  order space remains NYI as an explicit FORFEITS item, not welded law.
- **E · `dev-ambient-include-guards-are-not-evaluated` — the one to VERIFY rather
  than trust.** Admitting guards created a shape where an ambient package's guard
  is not evaluated and its guarded dependency binds UNCONDITIONALLY. Binding is
  the licensing direction, not the withholding one; the builder's safety argument
  is that the only winner-shifting case is already withheld by the cross-unit
  shadow refusal. That is a real argument of exactly the "safe because another
  mechanism catches it" shape that deserves an adversarial pass before the lane
  closes. Not urgent (it blocks nothing today), but it should not fold unchecked.
- **F · `dev-one-commit-lumped-two-changes`** — granularity nit, no action.

Conductor's overall read: the lane did what it was told, disclosed honestly
including two things it could have buried (B's invisible drift and E's
self-created hole), and the handoff artifact is genuinely resumable. The two
errors above are the conductor's, not the builder's.

## §9 — The human's remaining board (nothing dispatchable until these are typed)

1. `ack-specimen-respell` — CLOSED, ratified (§10).
2. `rule-guard-false-direction` — PARKED, not resolved (§11); costs priced, no
   build blocked.
3. `rule-cross-custody-refusal-licensure` — CLOSED by
   `rul-guarded-source-mints-exact-speaker-edge` (§12).
4. `rule-unresolvable-ambient-load` — BELIEVED DISSOLVED, conductor-unconfirmed
   (`~SUSPECT`). It was a licensure call only while CLI-supplied packages loaded
   through a flat declaration list. Under `rul-pre-source-is-dot-prelude` a
   pre-source is an ordinary `.`, which splits the question in two and leaves
   neither as licensure: a CLI-named pre-source that will not resolve is a plain
   pre-network usage error at the edge (the admin named a file we cannot read),
   and a load INSIDE a pre-sourced file that will not resolve is the existing
   `unresolved` ⇒ suspend-the-sourcer path. Wants the human's word before it is
   struck; nothing is blocked meanwhile.
5. `ack-dispatch-builder-two` — the §5 HARD GATE. Not conductor-dispatchable.

Owed by the conductor at the fold regardless: own-hand `both gate:full-quiet`;
the steering prose (builder PROPOSED text in its report — conductor writes it, and
judges first whether each line earns a permanent seat in every future context).

## §10 — ratify-specimen-respell-and-test-shape-debts [human-typed, 2026-08-18]

The case-dir cwd pin and the three one-line root re-spells stand. Ground: hardcoding
the test-dir path into each test is a boondoggle that makes cases immovable. No ruling
between `./oracles/foo.sh` and `./foo.sh` — both are valid product-surface; reach for
TEST-ERGONOMICS over literalness, which leans toward keeping a case's files in one
directory. Ideally one test covers each shape.

Two LOW-priority follow-ups, explicitly punt-able, and not worth a builder's attention
beyond a footnote (do not pollute `30I` with them unless one becomes actively wrong):

- `owed-one-absolute-path-specimen` — at least one hand-written case should exercise an
  absolute path. A mechanical e2e that copies a case into a tmpdir and drives it by
  absolute path is one shape; performance is the constraint, and punting is sanctioned
  if it is painful, slow, or judged not worth it.
- `owed-e2e-subtree-support` — the e2e sandbox copy is `read_dir(case)`, top-level files
  only, no recursion. Arbitrarily-nested relative-directory cases need it. Low priority
  unless this arc's own new e2es force it.

## §11 — park-command-v-guard-direction

Parked 2026-08-18 pending a deeper human-led dig with the usual rubber-duck model.
Nothing is blocked; this is a value hole, not a build blocker. Resume from here.

**The state.** `funcenv::run_control`'s `Guard` arm decides three ways: a live
function in frame decides TRUE (host-independent); undefined AND role-shaped decides
FALSE (`28M:dec-decidable-set-v0` — nobody ships a binary named
`apt_get__is_converged`); anything else walks BOTH branches and joins the
environments. So "model both and spread over the lattice" is already built.

**Why the residue is honest, not a modeling gap.** The then-branch is `:`, so the
engine carries `Undefined` into a world where the name is live. Path-sensitivity does
not rescue it: in the then-world the name is live-of-unknown-body and MAY BE A BINARY
on PATH, which is a different thing from the fallback's function. Joining those is
genuinely ⊤. No join-machinery change can shrink this; only making the FALSE direction
decidable can.

**What the residue costs.** Licensing at ordinary-named guarded helpers — `30I` §2.2's
canonical cross-author shared dependency is exactly this cell (`sm_common_query` is not
role-shaped). `load30-rooted-shared-dependency` still greens, because both runtime paths
reach Common and the run SET matches; what is lost is the vouch resting on that helper
(the `vouched-composition-not-present` shape, `30Ib` §5 row 2). Everything fails toward
run/withhold; nothing decides wrong.

**A conductor claim that FAILED its bar, recorded so it is not re-derived.** The
proposal to discriminate functions from binaries via `case $(command -v x) in /*)` does
NOT hold. POSIX writes a bare name for shell functions AND for special built-ins,
regular built-ins not associated with a PATH search, and reserved words — so `/*` means
"PATH binary", never "not a function". Worse, the classification differs inside our own
floor pair: `printf` is a builtin in dash 0.5.12 and an external in posh 0.14.1
(measured; the note lives in `cli/tests/e2e.rs`'s real-binary-lane comment). The human
had already been bitten here once; the bar for blessing an idiom is brutal cross-shell
certainty, and this cleared nothing.

**The unspent candidate.** Bless the VARIABLE-SENTINEL include guard — the C
header-guard shape `30I` §6.3 already puts on the page — as the decidable form, leaving
`command -v` guards exactly as they are today. A sentinel is decidable from the value
plane in both directions for any name: no PATH, no builtin classification, no
cross-shell exposure, nothing to be uncertain about. Prefer the `if`-wrapped spelling
over the early `return`, which keeps it flattenable (§6.3's top-level-`return` problem).
Cost: new work, not a re-reading — `oracle::load_inert`'s `include_guard` admits only
`command -v <literal name>`, and `LoadControl::Guard` is keyed on a FUNCTION name, so
the admitted grammar widens and the decision moves to the literal/value plane
(`LoadStep::Assign` already exists). Additive; revokes nothing.
~SUSPECT and unverified: whether the value plane's variable window reaches a CROSS-FILE
sentinel (set in the loaded file, read by the loader) — `30Ib` §5 row 4 flags a grade
obligation on that accessor.

**If the dig happens instead.** The bounded empirical question is: across dash, posh,
bash, ash and busybox, for a name that is neither a builtin nor a reserved word in any
of them, are `command -v`'s output shape and exit status identical? The floor pair is
already instrumented (`mise run test:floor`).

**Watch item.** `30I` §2.2's worked example guards an ordinary name and reads as though
it fully works. That is now known to under-deliver. Not corrected in the plan while this
is parked; if the park becomes permanent, §2.2 owes a caveat.

## §12 — ack-guarded-source-ruling-and-its-two-gates

`rul-guarded-source-mints-exact-speaker-edge` [TYPED 2026-08-18] CLOSES
`rule-cross-custody-refusal-licensure`. Work-order step 4 is buildable.

**Where it matches the conductor's expectation.** The direction is the one proposed:
the canonical guarded shared dependency earns custody through the author's own source
act, so no general widening of `rul-vouch-reaches-own-custody-only` is spent, and case 3
keeps refusing pre-network.

**Where it is NARROWER, and better — a correction to the conductor's reasoning.** The
proposal was containment-based: mint an edge from a load act ANYWHERE in the load
program, and lean on the guard-⊤ (`§11`) to withhold where the live body is unknown.
The ruling instead demands POSITIVE proof at the mint: the target resolves exactly from
authored-before-contact input, and every transitively load-bearing helper on the REACHED
vouch path `Must`-resolves inside the voucher's file or that exact target closure. Same-
name, same-file, or same-bytes is explicitly not the proof. Declining paths prove nothing.

That removes a real failure mode the conductor's version carried. Containment-plus-⊤ is
SAFETY BY COMPOSITION ACROSS TWO MECHANISMS: it holds only while the second mechanism
keeps withholding. Bless the variable-sentinel guard (`§11`'s unspent candidate) and the
⊤ disappears — at which point the containment version would begin licensing a voucher
onto a same-named private helper from another author's package, silently. The ruled
version has no such coupling; its proof is local and positive. Noted because the
conductor flagged exactly this composition-shape as needing adversarial review when the
BUILDER used it (`§8` row E), then reproduced it one message later.

**Also narrower, and cheaper:** the proof is paid only on a reached vouching path, not
minted unconditionally from text, so no custody is claimed in worlds where nothing rests
on it.

**THE INTERACTION TO EXPECT AT BUILD TIME** (`+SURE` on the mechanism, `~SUSPECT` on the
consequence): the ruling and `§11`'s park are TWO GATES, and the canonical specimen
currently passes only the first. In `load30-rooted-shared-dependency`, Alpha's guard sees
`sm_common_query` Undefined-and-not-role-shaped, so `run_control` joins both branches;
the flat-lattice join of `Undefined` with `Defined(common)` is ⊤. A ⊤ helper cannot
`Must`-resolve to the target closure, so the vouch does not license even though custody
is now minted. This is not a defect in either ruling — it is the park's cost, already
priced in `§11` as "licensing at `30I` §2.2's canonical cell", now visible from the other
side. Blessing the sentinel form discharges both at once. A builder discovering this
mid-lane should read it as expected, not as a broken ruling.

**Reconciliation note.** The ruling arrived as uncommitted work in the PRIMARY checkout
against `ec/play` (an older tip than this lane). It was applied here three-way and landed
clean across all five files; the primary tree still holds the same edits uncommitted and
they are safe to discard once this commit is verified.

## §13 — correct-the-guarded-source-reading [human-typed, 2026-08-18]

§12's framing was WRONG in a way that would have misled builder 2, and the correction
is the whole point of the ruling. Superseding §12's "two gates" reading:

**It is RECOGNITION, not a licensure widening.** The idiom is a method, spelled in sh,
by which an author says "I want exactly this function from exactly this file." It looks
like a fork at runtime; in the cell where vouch-minting fires it is NOT one, because both
arms land on the same binding at every later call site — either a prior oracle already
loaded that exact target, or this file loads it now. The engine's job is to SEE that
there is no analysis-time branch and decline to drive to ⊤. Nothing extra is trusted.

That dissolves the conductor's earlier safety worry entirely. The worry was "this lets a
voucher rest on someone else's body." In the fired cell there IS no someone else's body —
both arms are the same body. Where there might be, the recognition does not fire.

**The reading is (b), conservatively** [human-typed]: precise idiom, precise analytic
world-state, full match. Anything ambiguous withholds vouch, licensure and speaker
status AND MINTS NARRATIVE saying why — an explained withhold, not a silent one. sh's
dynamism supplies the ways the shape can mislead: a same-named `PATH` binary, a book's
own hand-written function, an `unset -f` and redefine, any dynamic load. The door is
deliberately narrow and widens slowly; the structurally-sound wide version is later work,
and book-author override stays the open DI forfeit (an oracle author who needs to be
explicit already has the unset-then-source pattern).

**The engineering posture** [human-typed]: ideally this needs NO new machinery — a rich
enough engine walks the reality and derives it. The ruling is that where ours is not that
rich, we enrich it, and/or special-case narrowly and temporarily while it is too dumb.

**THE CONSEQUENCE THAT MATTERS FOR SEQUENCING** (`+SURE` on the mechanism): this
SUBSUMES `§11` rather than sitting beside it. Recognition fires only if the engine can
rule out the then-arm being answered by anything but that exact target. For the FIRST
guard on a name — no prior modeled load — that is precisely `§11`'s parked FALSE
direction. And the diamond cascades off it: decide Alpha's guard and Alpha loads Common
exactly, so Beta's guard sees `Defined(common)` and decides TRUE exactly, and the whole
specimen resolves; leave it undecided and Alpha joins to ⊤, Beta inherits the ⊤, and
neither resolves. So `§11` is not a side topic to be parked independently — it is this
ruling's precondition for the first-loader case, and sizing it is the first thing builder
2 does in work-order step 4. `§11`'s unspent variable-sentinel candidate is one way to
discharge it; a dictate that contracted dependency names cannot be shadowed by host
binaries is another; both are human calls.

## §14 — supersede-the-command-v-park [human-typed, 2026-08-18]

**`§11` and `§13`'s conclusions are SUPERSEDED.** Evidence: `notes/30Ic`. Ruling:
`30I` `c57dd0cf`. Read those, not the two sections above, for current truth. What
changed and what a successor must not re-derive:

- The conductor's slash-discriminator proposal is REFUTED empirically, not just
  weakened. `PATH=:` — an ordinary leading/trailing/adjacent colon — makes BOTH pinned
  floors print a BARE name for an external, contradicting POSIX Issue 8's own output
  table (`30Ic:fnd-zero-prefix-defeats-slash-test`, `+SURE`, with dash's `padvance()`
  named as the cause). Plus: alias text can carry slashes; posh exposes neither aliases
  nor reserved words; POSIX warns subenvironment queries lose shell-resident categories;
  and no searched source uses slash-parsing as an include-guard idiom, so blessing it
  would have minted a Dorc invention rather than recognizing a habit. The conductor's
  POSIX reading was correct and its empirical check was absent — the same failure the
  human had already been bitten by once.
- The carve-out basis is now the VARIABLE SENTINEL, which passes the habit bar the
  slash test failed (Shellac, 1Password agent-hooks, published guidance;
  `30Ic:fnd-sentinel-guards-are-established`) and whose floor semantics agree across
  dash and posh on all three states (`30Ic:obs-sentinel-floor-semantics-agree`).
- `§13`'s claim that the ruling SUBSUMES `§11` as its precondition is now WRONG. A
  value test carries no PATH exposure, so the first-guard decision no longer routes
  through `command -v` at all and the diamond cascade the conductor described does not
  arise. `command -v` stays a supported, aspired-to route under
  `30I:pin-command-v-load-model` — conservatively withholding, explicitly not forfeited.
- The design's sharp half, which the conductor had not reached: the exact target
  closure must INDEPENDENTLY supply both the decisive guard value AND the reached
  helper. Either alone is forgeable (a copied sentinel assignment with no load; a
  same-named helper from anywhere), and demanding both trace to one closure means the
  only way to satisfy it is that the package really loaded and is really what is live.
  Both are `Must`, so ambiguity withholds. The condition never mints authority.

**Not a concern** [human-typed]: the parent environment happening to carry a matching
sentinel. Dorc is not pursuing hermeticity against its invocation environment yet, and
whether a local pivot book should inherit system environment or be scrubbed toward
ssh-parity is genuinely open (`30Ic` §8.4). Do not treat it as a gate on this work.

**Not lawed** [human-typed]: the residual sizing question — whether the value plane can
say *provably unset* at a first guard, given `analysis/src/value.rs:266` models an unset
variable as absent-as-⊤/⊥ — is handed to the builder as spec, with no
size-this-first directive and no descope path pre-authorized. It reports only if the
work proves wildly unexpected.

## §15 — rebase-onto-new-main-and-the-third-seam

`ai/main` moved under the lane (new gate lifecycle, conductor-skill revision,
verify-lane growth). Zero file overlap with the 15 lane commits, so the fold was a
clean REBASE rather than a merge; lane tip is now 15 commits atop `ai/main`. Hashes
before that rebase are stale by construction — cite commit subjects, not hashes.

**Gate lifecycle changed and the old spelling is DEAD.** `gate:quick-quiet` no longer
exists. Rungs: pre-commit (sub-3s floor) · `mise run both gate:full-quiet` (builder
completion, and the working loop — there is no agent-facing quick rung) · `mise run
gate:arc` (CONDUCTOR-ONLY arc close, ~20GiB, run from the populated branch BEFORE
folding into `ai/main`, because hk derives applicable checks from that branch diff and
selects nothing once the diff is empty) · `mise run gate:step -- <step>` for a named
failure. Whole-workspace clippy moved off the hook into builder completion.

**Builder 2 landed steps 1–4 minus one clause** (tip subject: "List the two new slugs
where the retire-guard reads them"). Its report claimed "steps 1–4 all landed" AND named
`rul-unannounced-cross-custody-fails-before-network` as not built; the second statement
governs. Two findings worth keeping: the mutation check CAUGHT a bad specimen (the first
shape sat on the source arm, where the ordinary `.` mints anyway, so the recognition was
never under test — it had to move to the reuse arm); and a pre-existing wrong-elision
route was closed (an UNDECIDED guard's fallback `.` used to mint custody, contained only
by the ⊤ the join produced — the safety-by-composition shape §12 flagged and the
conductor then reproduced).

**`owed-no-flag-defaults-to-stdin` [conductor finding, human-confirmed as a drop]** —
`plan --results` and `apply --plan` both DEFAULT to stdin, which is precisely the
implicit acquisition `rul-stdin-is-claimed-only-by-dash` forbids. Builder 2 declared them
as claimants (the interim) rather than retiring the defaults (the rule), which is why
`dorc plan -` currently refuses unless `--results FILE` frees the stream. Retire both
together; they are one seat and splitting costs a second corpus argv respell. NB
`--results` is an INPUT (read probe records from FILE instead of stdin), not an output.

**Seam 3** [human-offered, conductor-taken]: builder 3 takes the step-4 remainder, the
stdin-default retirement, and steps 5–6 (bundle projection, then locator composition
carried into a real diagnostic). Builder 4 takes steps 7–8 (artifact forms with atomic
publication, then the close: promotion, the `pin28` re-spell, e2e lowering, the owed
inlining floor cell). Rationale: 6 composes onto 5 and is small, so they belong together;
7 is a new on-disk subsystem and 8 is the corpus-wide golden sweep, so they get their own
window. Builder 2 spent ~793k tokens on steps 1–4, which is the sizing evidence.

**Punted, human-typed:** `--oracle-dir` dies soon-ish; punt if not trivial.
`dorc lint --oracle <path>` is UNRELATED to the retired loading flag — it names a file to
lint AS an oracle (dialect rules), and lint loads nothing; it stays. Help-prose drift is a
non-issue: the mechanical path drives to `Slop()` and it is all slop today.
