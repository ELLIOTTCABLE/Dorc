# 201 — Round-20 (take-3) opening: gate outcome, new rulings, seeding adjudications

> First round-20 note (append-only series; `200` is the priming-prompt's slug in quarantine, so
> notes start at `201`). AI-authored, confidence-marked. Trust R/D/I/K + STALENESS-AUDIT + the
> human rulings quoted here over everything else, including this note's own prose.

## §0 Gate

The orientation gate (R/D/I/K/TODO → STALENESS-AUDIT → AGENTS) passed with four flags raised;
the human's responses produced two durable rulings (§1), one doc-fix (§2), and one elaboration
obligation (§3). Charter material (19H/19I/19F/19G/191/16P/16Q + spike2 CLAUDE.md) absorbed
after the go.

## §1 New human rulings recorded this round (do not relitigate)

- **rul-toctou** — probe→apply staleness (TOCTOU) is *deferred-to-actively-WONTFIX*. Direction
  quote (2026-06-10, near-verbatim): "depends in what direction the tool as a whole heads, but I
  mildly suspect trying to attack it would erode the last ounce of remaining value-prop we have
  to offer. *maybe* someday … some sort of oracle tooling for this ('here's a super-cheap
  last-second check to run before the real thing'), but very very deferred if so." Engineering
  consequence: build nothing aimed at it — no re-probe-before-apply, no freshness windows; the
  kSTATE verdict-shape question is unaffected.

- **rul-mutation-impossible** — the deeper frame under kFAIL's probe direction. Near-verbatim:
  "We *cannot* meaningfully analyze for mutation. Mutation in shell-things, the *entire
  universe* of commands people ship onto computers, is just too diverse and huge and
  unavoidable… the *best imaginable* oracle is still just some random devopstype, spending
  *hours*, trying to externally observe… every corner-case behaviour of every flag of that
  third-party-tool which may even be closed-source… and then all their effort is good for
  *maybe* a few months." And: "the PLT literature will actively hurt us here — even *using* the
  word 'mutation' invokes a PL-design environment where totalistic control of mutation is
  aspired-to; that is utterly beyond us, permanently. All Dorc can ever hope to do is 'pipe A to
  B' (book → oracle) and wave our hands vaguely in the direction of
  whatever-we-can-determine-about-the-failure."
  Consequences I draw (+SURE on the first two, ~SUSPECT the third):
  - probe-inertness is *structurally vouched* (self-vouch carve-out), never analyzed-into-being;
    no analysis-confidence threshold ever ships an un-vouched probe. (This was already the
    design; the ruling re-grounds it in impossibility rather than in trust-asymmetry logic.)
  - oracle effect-claims are *declarations with a shelf-life*, not verifiable properties — the
    calibration harness (kVERIFY-calibrate) is a confidence lever, never a proof; version-drift
    (TODO's parked binary-identity items) is the named decay mode.
  - vocabulary: in fresh design prose prefer "effect-claim"/"declared effect" over bare
    "mutation(-analysis)" when the referent is the oracle contract — the latter invites the
    totalistic frame the human is warning against. (Not a rename of kFAIL-withhold's
    "never-mutate" phrasing, which is about *our* probe behavior, not analysis of tools.)

- **rul-kelision-naming** — flag-1 (the kELISION parenthetical defining "elision" as
  *not-checking*, opposite the firmed replacement sense) accepted as a real collision; fix
  applied at root `Dorc/KNOBS.md` (uncommitted, pending human review): the parenthetical now
  carries a naming-caution distinguishing scope-elision (this knob: what gets checked at all)
  from replacement-elision (observable-preserving substitution under kFAIL). A knob-rename was
  deliberately NOT done (slug-stability; human owns naming). Residual not touched: kUNIT
  "per-function skip" and kVOLATILES "skip-cache"/"skip system" still use the banned word in
  adjacent-but-different senses — flagged here for the human's rewrite pass rather than edited.

## §2 flag-4 disposition

The probe-side/apply-side education asymmetry (audit dir-soundiness-ux AI-caveat) stands, but
the human reframed its foundation per rul-mutation-impossible: the probe-side needs structural
backstops not because education fails there, but because the analysis alternative *does not
exist*. Recorded; no code consequence beyond what inv-kfail already carries.

## §3 flag-2 elaborated — probe-sourced replacement values (now inv-probe-sourced-values)

The reading the human confirmed-by-asking-for-elaboration: an elision's stand-in may reproduce
only channel-values with *probe-provenance*. Spelled out (and now in spike/CLAUDE.md as
`inv-probe-sourced-values`):

- A `Replace` plan-step substitutes a leaf with a stand-in that must reproduce every *consumed*
  channel of the leaf's observable-tuple. The question is where those values may come from.
  Sanctioned sources: (a) a concrete observable the shipped probe actually produced (the
  check()'s own rc/stdout, e.g. the guard idiom 19H §2.2); (b) nothing else, currently. The
  oracle-declared `fact-state → observable` bridge (19H §2.3's alternative pole) is exactly the
  "declared value" the human is rejecting under fork-mutator-rc; it stays unbuilt unless he
  rules otherwise.
- Why correctness-critical: the apply-direction floor (⊤ ⇒ run) cannot catch a *confidently
  wrong* value — a fabricated rc is not ⊤, it is a wrong Must-fact, and it licenses an
  under-execute (the worst lattice outcome) with no degrade to save it. 19D is the specimen:
  fabricated rc=0 collapsed `useradd deploy || mkdir /srv/app` and silently dropped the
  fallback. Fabrication converts "missing knowledge" (safe, runs) into "false knowledge"
  (unsafe, elides) — the one transformation the whole design forbids.
- Type-shape consequence (~SUSPECT on exact spelling, +SURE on direction): the value a StandIn
  emits should carry provenance in the type — e.g. `Predicted<Rc>` only constructible from a
  `ProbeResult`, or a `ProbeSourced<T>` wrapper minted only by the probe-result ingestion path —
  so "where could this number have come from?" is answerable by construction, and a fixture
  literally cannot inject one (the 19F §6 anti-masking contract becomes a compile-time property
  for the engine, leaving tests to exercise only the sanctioned ingestion path).
- Boundary note (exclusion-check, other-phase): pre-probe the same discipline reads as "the
  *entity* and *body* shipped must come from the oracle's own argparse" (identity-declared);
  post-probe it reads as "the *values* folded must come from the probe". Same invariant, two
  faces, matching 19H §1.2's one-analysis-two-faces.

## §4 Seeding adjudications (the workspace is live)

- **adj-seed-copy** — copied the spike2 keep-list wholesale (all 7 crates + fixtures +
  mkmocks.sh) onto the human's fresh scaffold, rather than cleanrooming the substrate. Reasoning:
  19F §4 explicitly KEEPs the substrate ("re-deriving wastes effort"); the corpus must run
  day-1 (19I de-cruft instructions name specific Rust tests to strip — they must exist); token
  economics favor spending Fable/Opus capacity on the *new* input side, which is cleanroom by
  construction (it never existed in spike2). The "mild cleanroom attempt" lands as: (a) the
  value-plane, check()-lifting, and probe-projection are built fresh from 19H/19F specs; (b)
  the oracle-contract layer gets re-shaped around the command-keyed check() (superseding the
  `oracle_*` marker spelling per 19I group I); (c) poison-control is enforced by briefing — the
  spike CLAUDE.md now bans subagents from the quarantine, and build-subagents get specs, not
  spike2 internals. -GUESS this is what the human intended; if he wanted a harder cleanroom,
  the copy is one commit (ca1919c) and trivially re-doable the other way.
- **adj-edition** — kept the scaffold's edition-2024/rust-1.96; one mechanical fix (test helper
  `gen` → `gen_xfer`; `gen` is reserved in 2024). Folded into the seed commit.
- **adj-baseline** — baseline verified green before any de-cruft: 138 cargo tests pass (1
  ignored — the deferred 16G HOLE#1, expected), e2e 43/43 with the one standing xfail
  (`render-case-arm-oneliner-wrong`) reporting as xfail. Matches 19G's "137+43" within
  count-drift (~SUSPECT the +1 is a test added after that note; not investigated).
- **adj-lint-debt** — the scaffold's strict lint table (clippy all+pedantic+cargo, missing_docs,
  no-panic family) emits ~480 warnings against the seeded code under --all-targets. Resolution
  in flight (Opus subagent): `clippy.toml` in-tests allowances (the policy already exempts
  tests) + crate-root `#![expect(lint, reason)]` on seeded crates only + mechanical fixes for
  the tail. Chosen over (a) demoting the workspace policy — guts it for future code — and (b)
  hand-fixing 480 sites in code the round partially rips out. `#![expect]` self-ratchets: when a
  rebuild removes the last offending site, the unfulfilled expect itself warns and gets deleted.
- **strain-hk-worktree** — environment finding worth the human's attention: hk v1.44.3
  (libgit2-based) cannot open this repo from inside `.claude/worktrees/*` — the repo uses
  `extensions.relativeWorktrees` (+ global `worktree.useRelativePaths`), which libgit2 refuses
  ("unsupported extension"). The config-based `hook.hk-pre-commit` therefore hard-fails on ANY
  commit from this worktree (the human's too, not just agents'). Workaround in use: run the four
  gates directly (fmt/clippy/deny/typos), commit with the hook's own designed `HK=0` bypass.
  Possible real fixes (human's call): newer hk (if it has moved off libgit2 or libgit2 gained
  the extension), or absolute-path worktrees for this repo. Flagged in spike/CLAUDE.md so
  subagents don't "fix" it by `--no-verify`.

## §5 Plan-of-attack (confidence-marked; the de-cruft and rebuild)

Sequence (keystone-first per ap-1; each step lands with its tests + a note entry):

1. **De-cruft per 19I §2** (blocked on the lint agent finishing — same files). Strip the
   stdin-rc-injection mechanism + `andor-rc-vouch-wrong` + `fold-oror-guard-omits`'s injected
   rc + the masking matrix tests + the baked-verb useradd fixtures; keep
   `andor-rc-undeclared-runs` (the safe-default keeper) and the exec-under-mocks gate verbatim;
   XFAIL stays pinned but the round's definition-of-done flips it to must-pass. +SURE on scope.
2. **The value-plane keystone** (the round's ap-1 keystone): a propagation analysis over the
   existing CFG/worklist — constants, assignments, parameters/`$1..$n`/`shift`/`"$@"`, the fold
   constructs (`&&`/`||`/`if`/`case`/`!`), cross-file via the oracle's lifted function bodies.
   Substrate decision deferred until the seams complain (19H §4 holds it open; the committed
   worklist stands — re-evaluate only on strain, per 191 §4). ~SUSPECT the existing
   reaching-defs/`MapL` machinery carries a flat `Flat<Value>`-per-var domain without a new
   engine; that IS the prior-art check, performed by building the smallest version and watching
   seam-interproc/seam-finite.
3. **check()-contract lifting** (19H §2): the oracle crate re-keyed from `oracle_*` markers to
   the command-keyed full-args `check()`; the engine flow-tracks book-constants through the
   check's argparse to the inline kind-annotation (fork-annotation-spelling: inline accepted as
   spike debt, parser stays a disposable front-end — massage, don't generalize). Entity
   resolution moves from find-3 flag-strip to declared-annotation + value-flow; the find-3
   markers come out.
4. **Probe-projection**: ship the check body as a function + full verbatim argv into the probe
   artifact; the probe's concrete observables (not stdin fixtures) feed the fold. The cli
   round-trip re-keys from `kind:entity verdict` lines to per-check observable reporting.
   (Exact OOB framing minimal — enough to carry `(check-id, channel, value)`; kCOMMS stays
   un-built.)
5. **One-Observable completion**: stdout/stderr become predictable channels of the same tuple
   (still mostly ⊤ in practice); the AndOrStatus-collapse decision gets made deliberately when
   the render floor is reworked (group E supersession) — recorded either way.
6. **Forks**: fork-mutator-rc — adopting the human's lean (un-probeable mutator is ⊤ ⇒ runs;
   the one elision lost; `(exit 9)` machinery retires with the masking tests). Recorded as
   adopted-lean, reversible if a real oracle-declared-observable case earns it.
7. **Crosschecks** (ap-3 rotation): after the value-plane lands → target propagation-correctness
   (the no-floor obligation, 19H §1.3); after probe-projection → target the *harness* ("is the
   corpus still measuring the right thing with injection gone?"); near round-close → target
   charter-adherence ("did we build 19H or scaffold around it?"). Fable-class pairs, clean
   contexts, SAFETY block carried.

## §6 Seeding-feedback (running; the meta-goal deliverable)

- fb-1: The priming prompt's "copy quarantine-spike2's tests … verify green" vs item-8's "mild
  cleanroom attempt" pulled in opposite directions for the *implementation* crates; I resolved
  per adj-seed-copy but spent real cycles on it. One sentence in the prompt ("substrate: copy;
  input-side layers: build fresh from 19H — don't port the oracle/plan shapes") would have
  removed the ambiguity.
- fb-2: The hk/relativeWorktrees breakage (strain-hk-worktree) cost a diagnostic loop
  mid-commit. If worktree rounds continue, a line in the priming prompt about the known hook
  state (or fixing it repo-side) saves every future round the same loop. (You likely hit this
  yourself when committing the scaffold — 8c9f632 etc. presumably went in from the main
  checkout.)
