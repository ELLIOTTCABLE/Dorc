You're the top-level agent for round 21 of spike-3, continuing the SAME codebase and
worktree the round-20 wave built (you're in worktree spike3.). This is a continuation, not a reseed: the spike/ tree, its corpus, its
notes, and its invariants are live inheritance, not reference material. Your core job is
twofold: understand this project deeply at the high level and corral your subagents in
the herding-cats sense — catching their errors in cross-cutting judgement and high-level
design — and reach my overall goals effectively. You have wide latitude in service of
those two. Use subagents liberally to protect your own context-window — you
under-delegate by default, so correct for it — and keep your own window for
adjudication, synthesis, and the balance-calls below. You're round 21: notes go to
`Research/notes/21[0-9A-Z]-descriptive-slug.md`, append-only, a new numbered note per
chunk of work. Rich logging of what strained and where remains a primary deliverable —
this is state-space exploration, and the product is the record, not a green checkmark;
but note the charter-tension item under "open forks" below: this round leans harder
toward making the thing real than any before it.

----
SAFETY (this block is non-negotiable and applies for the entire process; copy it
verbatim into the top of every subagent prompt):
- No git mutation outside this worktree; never, ever push. Local commits on this ai/*
  branch are encouraged — commit granularly, with `(AI …)` labels per the repo's style.
- Subagents get NO git index or working-tree-restore operations (`add`/`commit`/`mv`/
  `checkout --`/`restore`): filesystem edits only; they report, the orchestrator
  commits.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't mutate
  global state (no system packages or system config; worktree-local `mise`
  installs/config are fine).
- Everything you build follows DST discipline: deterministic, local, mutation-safe.
  Clock, network, disk, and randomness only through DI seams; correctness-critical
  kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert mocks
  under `PATH=mocks-only`) — never real mutators. Real-command strawmen in the repo are
  frozen evidence; they must never be executed. The only sanctioned executor of fixture
  material is `sh e2e/run.sh` (and `BLESS=1` — see the exclusivity rule below).
- `Research/notes/quarantine-DO-NOT-READ/` stays unread, including by you, except for
  files I explicitly hand you.
----

(The frozen-evidence rule has bitten on this very machine; so has each of the process
rules below — they're scar tissue, not ceremony.)

Orientation, in this order — deliberate (first-party docs before LLM-generated
material); don't dig into Research/notes/ prospectively:
1. README.md, DESIGN.md, IMPLEMENTATION.md, KNOBS.md, TODO.md — human-written, final
   authority (KNOBS is AI-drafted but human-adopted; its welds stand). If a
   STALENESS-AUDIT.md is present at the root, it's my voice on where those docs lag;
   rulings win over prose.
2. AGENTS.md — its reading-guide governs everything under Research/ (terminology
   firming, the two-users discipline, exclusion-checking, the prior-art gotchas).
3. `spike/CLAUDE.md` — the working agreement: every inv-* invariant, the standing
   human rulings (mutation-analysis-impossible, TOCTOU-WONTFIX, rc-opacity,
   order-sacred, "skip" banned), the gate set, BLESS exclusivity, the subagent
   supervisor rule. This file is binding and current; it is also yours to keep
   updating as the round teaches.
4. `Research/plans/20K-round20-take3-report.md` then
   `Research/plans/20U-round20-overnight-addendum.md` — the round-20 close and the
   post-close MVP wave: what's real, what's stand-in, the disaster-class ledger, the
   dispatch heuristics you inherit, and §6 of 20U, which is the skeleton of YOUR
   charter below. Then `Research/plans/20V-errexit-doors.md` — the arch-3 charter and
   the round's center of gravity; the GATE below asks for its content, so read it
   BEFORE synthesizing.
5. Per-need, not prospectively: notes 20M/20S (loop lowering + member-precision and
   its license), 20O/20T (the two crosscheck reconciliations — the negative results
   are load-bearing), 20E (Query effect-class), 206/207 (status channels; the open
   errexit tension), 209 (the value-plane breakdown map: brk-*), 20A (whack-a-mole
   prognosis and countermeasures cm-*). ANALYZER-NEEDS.md for the oracle-side contract.
6. Theory as needed: Research/sources/B-moller-schwartzbach-static-program-analysis
   (ch. 4/5 the engine, ch. 8/9 the IFDS road deliberately not yet taken — read enough
   to recognize when the hand-rolled worklist strains toward it; 209 records where).

GATE — before reading anything else, synthesize back to me, brief and
confidence-marked: the two phase-keyed soundnesses (kFAIL) and why a confidently-wrong
concrete value is the disaster class with no floor; the two users and the no-cliff
gradient; "spelled in sh" and its off-ramp; elision-as-replacement and the one-
Observable channel model (StatusRelaxable vs StatusRenderFloor, and why that split is
render-capability, not construct-identity); the C-3 × fork-mutator-rc arithmetic and
what it does to mutator-elision under `set -e`; the canary reframe and the five-arm
provenance taxonomy (20V §2–§3), naming which doors are build-work and which
dq-errexit forks are OPEN and mine; the self-reach license in one paragraph (why
eliding-all is self-consistent and ANY non-self writer refuses). Flag anything that
doesn't sit right — pushback here is wanted; 20V §3 claims completeness, and finding
a sixth provenance arm now is worth a week of building. Wait for my go. The rest
describes your autonomous round.

The charter — a deliberate arc, front-loaded with the thing that's been fighting us;
each item names its acceptance. You own the how and the ordering within reason, but
arch-1 lands before anything that touches render, arch-3(a) door-3 is independent and
cheap (land it early for the corpus's sake), and arch-6's dashboard exists by
round-close in some form:

- arch-1 — leaf-exact render. Three carve-out waves (T14 case-arms, F2 scaffolding
  lines, group-closer) all exist because "the source line" is the wrong substitution
  unit, and `Channel::StatusRenderFloor` exists ONLY because the line-granular render
  can't substitute an `if`/`elif` guard in-situ. task-R already fenced every emission
  site into `plan::render` (20P). Rebuild apply-rendering span-based: a Replace
  substitutes the leaf's exact byte-span; the carve-out family and its detection
  machinery retire; StatusRenderFloor retires or demonstrably narrows, and the lone
  if-guard becomes an ordinary guard-capable substitution (a NEW elision class — pin it
  with corpus cases both poles). Acceptance: zero semantic golden churn outside the
  newly-expressible cases, every retired mechanism deleted not bypassed, the 20R/20M
  floor tests re-grounded. The 20O find-6 latents and 20S §10 hunt-4 shapes unfreeze
  here — re-check them against the new render.
- arch-2 — brk-2: budget-bounded function inlining. The biggest modeled-subset gap a
  real book hits: helper functions are currently Opaque poison to values, pristine
  prefixes, and licenses. Design is yours within: inline bounded (size/depth budget,
  no recursion — recursion ⊤-rejects loudly), uniformly for books and oracles;
  positional-parameter binding through the existing value plane; everything
  over-budget stays Opaque-with-diagnostic (proportional degradation, never a cliff).
  This unlocks the 207 wrapper-pun directions — build the mechanism, leave the 207
  policy fork to me. Hostile crosscheck mandatory before anything builds on it (the
  L2 pattern: builder writes the hunt-list, a clean-context Fable attacks it).
- arch-3 — the errexit doors (the PRODUCT-VIABILITY keystone; charter:
  `Research/plans/20V-errexit-doors.md`, read it in full before touching this). Under
  `set -e` — which every responsible book uses — bare mutators never elide at HEAD
  (the five-weld chain, 20V §1); without this arc the spike's product elides almost
  nothing on real books. Build, in dependency order: (a) door-3 rc-deadness — the
  `cmd || true` consumed-but-invariant refinement; independent of everything, do it
  early; pin both poles (an `||` consumer with observably-DIFFERENT continuations
  stays blocked). (b) door-1 cascade verification — guarded BLOCKS (`query || { edit;
  restart; }`) must fold whole, the restart eliding as dead control-flow (the
  Ansible-handler payoff); corpus cases proving the cascade, and post-arch-2, the
  wrapper-function form. (c) door-4 guard-insertion — the transform (bare oracled
  mutator ⇒ `<vouched-probe-body> || <original>`) as a NEW license category
  (guard-insertion, NOT a relaxation of observable-reproduction; 20V §4), mintable
  only when the kind's oracle carries the door-2 converged-run declaration AND
  non-Status channels pass existing gates; rides arch-1's span-render, lands after
  arch-2 so the wrapper-style population validates the semantics analytically first.
  The declaration's sh spelling is acceptable-debt inline form (kTYANNOT precedent);
  the PRECEDENCE POLICY (20V §5: admin-explicit > oracle-default >
  engine-conservative) goes behind a policy-seam in ONE module — the human is
  adjudicating the UX-shape in parallel with your round and rulings WILL land
  mid-round; build so they hot-swap. Hostile crosscheck mandatory on (c) — attack the
  four-world trace (20V §4 door-4) and the disclosure floor. dq-errexit-1/2/3 are the
  human's: surface, never settle.
- arch-4 — brk-4 first slice: command-substitution. At minimum: honest, specific
  ⊤-diagnostics everywhere `$()` appears today (no silent phantoms — the find-3
  lesson). The real prize if the design holds: a Query-shaped `$()` — an oracle-backed
  substitution whose value arrives by probe (the probe-results lane already carries
  per-site records; a `$()` site is a site). Treat this as a design-first task: a
  short note weighing it BEFORE building, flagged to me if the seams resist.
- arch-5 — partial-member list-rewriting (the deferred half of L2's value,
  tc-l2-member-list-not-rewritten): one diverged member currently runs ALL members.
  A licensed partial elision rewrites the for-list to the diverged members only —
  this is the first render that CHANGES a loop header, so it rides on arch-1's
  span-render and needs its own hostile pass (the 20T did-not-survive list is your
  prior art for what to attack).
- arch-6 — the H2SALS coverage dashboard (the metric-deliverable; see the north-star
  note below). A side-quest (round 1A, possibly running CONCURRENTLY in another
  worktree) is producing a plain-POSIX-sh rewrite of "How To Secure A Linux Server"
  plus a command/construct census and oracle seeds, landing at .claude/worktrees/ai-r1A-H2SALS/Research/corpora/H2SaLS —
  ask me if absent and you need it; its artifacts may continue arriving mid-round. Build a re-runnable
  analyzer-report over any book+oracle set: per command-site — analyzable-without-⊤?
  (and which trigger if not) · oracled? (does any oracle resolve it) ·
  probed-converged? (per supplied results) · elided, AND THROUGH WHICH DOOR? (20V §7:
  fold / dead / guard-transform / static-declared / runs) — rolled up per-axis,
  count-weighted AND criticality-weighted (the 1A matrix supplies weights when it
  lands; line-count is the stand-in). The per-door attribution is what decomposes the
  north star into separately-ownable terms; without it the number is unactionable. If the 1A artifacts don't exist yet, build and
  exercise it against the e2e corpus and leave the adapter seam; do NOT block on the
  side-quest, and never edit its tree — it has its own orchestrator. Acceptance: the dashboard runs in
  the gate set without becoming a gate (it reports, it doesn't fail builds).
- arch-7 (stretch, only if the arc lands early) — hostsim DST at scale: seeded-random
  book/oracle generation driven through the differential argv-echo harness (gate-5),
  the cheapest local approximation of cm-1 until a Linux tier exists.

The north star, stated carefully so it can't quietly become a target: the arc-level
goal is ~80% non-trivial elision coverage of the H2SALS rewrite on a converged host —
criticality-weighted, where "non-trivial" excludes sites with no remote-mutation or
remote-read cost (the census marks them), and where a door-4 guard-transform COUNTS
(it converts a slow remote no-op into a millisecond read — under network-dominance
that is the win; report full-elisions and guard-transforms as separate columns and
never blur them). It is a DERIVED observable of the arch-6 dashboard, never a number
any task optimizes directly. With the doors built, the reachable ceiling is oracle
coverage × declaration coverage (20V §7) — NOT guard-idiom density of the book (a
property we must not game) and NOT raw engine quality. It stays contingent on things
that are not yours: the 1A rewrite existing; oracle/declaration authoring that is
partly corpus-work; and my open dq-errexit rulings, which can move the
bare-middle-default population wholesale. When the dashboard makes the decomposition
concrete, bring me the per-door numbers and the open forks — do not grind toward a
percentage my rulings could halve or double.

Open forks — surface to me, don't relitigate alone: the dq-errexit ledger (20V §8 —
dq-1 canary-only-cost, dq-2 who-owns-the-bare-middle-default, dq-3 guard-insertion's
trust status, and the declaration's ratified spelling; I am adjudicating these IN
PARALLEL with your round, so expect mid-round rulings and keep the policy-seam hot;
YOLO/207 stays set aside as an escape-hatch — do not build toward it); the
exploratory-vs-MVP charter tension (20K §7 / 20U §6 — my lean for this round:
strain-mining stays the deliverable, but strain found against a REAL target outranks
strain found against synthetic corpus cases); and the four round-20 autonomy-rulings
awaiting my review (20U §4) — treat them as standing unless I say otherwise, but
don't build anything that would make reversing them expensive.

The hardest part, and the reason you (not your subagents) hold the wheel — the
balance-points. They come in three kinds: KNOBS we'll turn by choice later (keep their
decision-points alive in the code's shape; weld only when implementation legitimately
demands it, then document the cost); decided-things that are nonetheless inherent
balances (the four-outcome lattice; fail-fast batching vs warning-fatigue;
degradation proportional to the user's omission and never an inch further); and
implementation-balances nobody "decided" (when a license mints; when superposition
collapses and in which phase; how generous a budget dial runs before degrade-to-⊤ —
arch-2 is MADE of these). The tc-* discipline stands: a component that hits one flags
it up; you collapse it, and a sentence in the current note saying which way and why is
worth more than the code. Exclusion-check anything you're about to dismiss: re-test
under the other direction, the other phase, the other user, the other reliability. A
subagent quietly resolving one of these locally is a smell — pull it up.

The meta-goal, unchanged and still live: this project is partly my proof-to-myself
that LLMs can do real, complex, difficult software engineering. The highest-level
deliverable is your demonstrated capability. Two narrow sub-deliverables: note
anywhere better seeding from me would have prevented a struggle, and surface it at
round-close; and keep refining the dispatch heuristics you inherit (20U §5) — the
load-bearing ones so far: split tasks that would make an agent DECIDE cross-cutting
things, not tasks that are merely big (a 538k-token build succeeded because its
design was fully pre-spelled in the brief; a 490k one failed because it wasn't);
hostile Fable crosschecks are the consistent highest value-per-token spend (every
priority-1 of the last wave — budget them as a fixed fraction of build spend, with
the hostile-identity briefing and mandatory engine-vs-dash construction discipline);
verify-don't-relay (run the gates yourself before every commit — diagnostics tooling
cried wolf 4+ times); reserve note-slugs yourself when dispatching parallel work.

Process notes, learned the hard way — these are now rules, not suggestions:
- /adversarial-crosscheck at real junctures, clean contexts unseeded from your own
  notes, rotate the target (harness and charter-adherence, not only core soundness);
  carve out market-fit/value-prop as explicit dead-ends. The builder writes its own
  adversarial hunt-list as part of every substantial task's report; the crosscheck
  brief starts from that list and is told to exceed it.
- Acceptance is executable: `dash -n` plus exec-under-mocks plus the full six-gate
  harness; never a text golden-diff alone. Hand-derive goldens; report every delta.
- BLESS is EXCLUSIVE: never while any build-agent is in flight; orchestrator-only, on
  a freshly-verified binary, diff inspected case-by-case (a sibling's mid-flight
  binary once baked a bug into a golden).
- Concurrent subagents need disjointness in build artifacts and goldens, not just
  source files — they share `spike/target/` and the corpus tree.
- SyncThing is live in a parent dir: `*.sync-conflict-*` ghosts of legitimately-
  deleted files may reappear (it has happened — a husk directory broke the harness).
  Cleanup is mechanical (verify against git history, remove the ghosts); never vendor
  repos into the tree un-ignored.
- Subagent mechanics: build-agents work in THIS worktree (spawned isolated worktrees
  check out a stale base and can't commit); read-only crosscheck agents isolate fine;
  a running subagent can't be course-corrected mid-flight, so scope briefs droppable;
  give subagents the goal and the invariants and leave them room.
- Gates before every commit, from `spike/`: `cargo fmt --check` · `clippy -D warnings`
  (no new expects; ratchet seeded ones away when fulfilled) · `cargo test --workspace`
  · `sh e2e/run.sh` ×2 · `mise x -- typos spike` from the root. There is NO git hook;
  you run these yourself, every time.

When the gate has passed and you've absorbed the inheritance, tell me your rough
plan-of-attack — brief, confidence-marked, including where you expect arch-1's render
rebuild to bite and your crosscheck budget — and go.
