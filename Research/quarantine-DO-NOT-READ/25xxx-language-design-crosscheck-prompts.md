### Pair 1

1. neutralised/disowned

> The team in this repo set out to build an orchestrator — and along the way, mostly as a
> side-effect of other design work, they have gradually evolved a small *programming language*:
> a dialect of POSIX sh that library-authors write (period-named role-functions, inline binds,
> trailing annotation-marks, a blessed exit-status partition, refusal idioms), a set of promises
> about what plain-shell script-authors may keep writing, and machine-emitted shell the tool
> inserts into user-facing artifacts. No dedicated language-design round has ever run; the
> dialect accreted decision-by-decision while the team's attention was on analyzer semantics.
> They are now at the point where real artifacts get authored against it in bulk — a stdlib, a
> first real user — and the language deserves review *as a language* before that happens.
>
> First, make yourself an expert: follow the shared skill-up phase below, including its
> pre-registration gate, before you open the team's design corpus. Then review the language
> they've evolved, judged against what you learned. The primary artifact is `USER_STORY.md`,
> which walks the whole dialect end-to-end through a worked example; the shared reading-list
> grounds the rest of the context.
>
> Play the critical but supportive reviewer: where does this dialect embody mistakes language
> designers have made and regretted; where will its ergonomics fight its own two declared
> audiences (a scrappy admin who wants the server fixed; a careful engineer authoring
> correctness libraries); where is its evolution/compatibility story storing up the kind of
> pain that an entrenched user-base later makes unfixable; what would an experienced language
> designer change *now*, while it is still cheap. Equally: where has the accretion — even
> accidentally — landed on the *right* choice? Say so; that steadies the ground as much as a
> flaw un-steadies it.
>
> Do *not* take their assumptions and leans as written; the human steering is experienced in
> languages-as-a-user but a novice at *designing* one, and the corpus is largely AI-slop,
> subject to broad sycophancy. But also do not play the adversary unnecessarily. Simply stay
> vigilant: a dialect this pleased with how naturally it "stays sh" is exactly where a
> comfortable, self-confirming loop likes to hide.
>
> Gently out-of-scope, subject to your best-effort judgement (surface only relatively-critical
> findings in these categories; don't get mired):
>
> 1. whether sh should be the substrate at all. Welded, deliberately, with eyes open (their
>    `kLANG`); relitigating it moves nothing.
> 2. the analyzer's verdict/license *semantics* (what a vouch means, what silence means, the
>    ternary verdict). Settled machinery, and not a language question. Your surface is the
>    *language*: how things are spelled, read, learned, taught, versioned, and evolved. Where a
>    language flaw traces irreducibly to one of those semantic commitments, say so plainly as a
>    boundary-note rather than relitigating the semantics.
> 3. market-fit / will-the-world-use-this. Welded as immeasurable-YOLO; man-hours already burned.
> 4. the prototype's parser/engine implementation internals — with one carve-out that IS in
>    scope: the fixture and oracle *code in the repo* is evidence of the language as actually
>    written, and what real usage reveals about the language outranks what any document claims
>    about it.
>
> One defense you should expect and pre-empt: most spellings are officially labeled
> "strawman-tier, subject to change." A finding does not die to that label. Prefer findings
> that would survive it — choices already load-bearing across multiple artifacts and rulings,
> or of the class your research shows becomes unfixable once anyone depends on it. Findings
> that genuinely evaporate under "we'll respell it later" should be marked as such by you,
> before anyone else has to.
>
> This is an open-ended exploration brief, take your time. Keep ongoing notes as you work in
> `Research/notes/24Ka-*.md`; do not read sibling `24K*` notes if they exist. You are likely
> to discover multiple avenues; focus on each one separately and cross-check your own work;
> work hard both to *find* things, but work hard also to *decide why they're not all that
> valuable*. Tie each finding either to a pre-registered lesson from your skill-up (by number)
> or explicitly mark it lesson-independent; cite the exact artifact and line it lives at.

2. adversarial/opposition-owned

> A colleague of mine has been building an ops tool on company time, and somewhere along the
> way they became a programming-language designer *by accident* — a dialect of shell has been
> accreting for weeks, one locally-convenient decision at a time: special function names,
> annotations threaded into command arguments, meaning hung off exit codes, machine-generated
> shell injected into user-facing output. There has been no language-design pass, no
> versioning or compatibility policy I can find, and — my honest read — no awareness of how
> unforgiving language design is to first-timers. Management is charmed by the walkthrough
> document. I've seen this movie before: a language that accretes as a side-effect embodies
> the classic early-days mistakes, and by the time anyone notices, there's a corpus of user
> code and every mistake has tenure. They're about to author their first real library and
> onboard their first real user against it, so this is the last cheap moment to catch it.
>
> I need you to first make yourself the expert they never consulted: follow the shared
> skill-up phase below, including its pre-registration gate, before touching their corpus.
> Then go through the language they've evolved — the author-facing dialect, the promises made
> to plain-shell writers, and the shell their tool emits into user artifacts; their showcase
> (`USER_STORY.md`) walks all of it end-to-end — and find where they've stepped on the rakes
> you just catalogued: ergonomics that fight their own declared users, spellings that won't
> survive contact with real code, an evolution story that isn't one, hard-won lessons from
> the gradually-typed and shell-adjacent worlds that were sitting there free and went untaken.
> The corpus is largely AI-generated and the human steering it is a novice at this specific
> craft — do not extend it the benefit of the doubt, and do not take its framings or
> self-assessments as given.
>
> Understand what I actually need, though: ammunition that *survives scrutiny*. If I hand
> management a list of gotchas and half of them collapse under a second look, I'm done being
> listened to. So for everything you find, try honestly to kill it yourself first; and where
> their language genuinely holds — where the accretion stumbled into the right call, or a
> suspicion of yours turns out unfounded — say so plainly. A criticism you've checked and
> withdrawn protects me as much as one you've confirmed.
>
> Some categories are poor uses of your time even for my purposes (surface findings there
> only if you hit an absolute unfixable-once-shipped landmine):
>
> 1. the sh-substrate choice itself. Management swallowed "sh is the product" whole; attacking
>    it moves nothing.
> 2. their analyzer's verdict/license semantics (the skip/guard/run machinery). Settled, and
>    not the language surface. Attack the *language* — though where a language rake is bolted
>    directly onto one of those semantic commitments, note the bolt; that's fair.
> 3. market-fit / representativeness. Already hand-waved past management.
> 4. prototype implementation internals — except the repo's fixture and oracle *code*, which
>    is fair game as evidence of the language in the wild, and often more honest than their
>    documents.
>
> Expect the "it's all strawman-tier, we'll respell it" shield, because they raise it
> themselves. Findings that die to that shield are worth little to me; findings that survive
> it — already load-bearing across their artifacts and rulings, or of the class that compat
> culture makes permanent the moment real code depends on it — are the ones that stick. Your
> research phase is precisely what qualifies you to tell those classes apart.
>
> This is an open-ended exploration brief, take your time. Keep ongoing notes as you work in
> `Research/notes/24Kb-*.md`; do not read sibling `24K*` notes if they exist. You are likely
> to discover multiple avenues; work each one separately, and cross-check your own findings
> before presenting them. Tie each finding either to a pre-registered lesson from your
> skill-up (by number) or explicitly mark it lesson-independent; cite the exact artifact and
> line it lives at.
>
> Catch it now, while it's still one document and a pile of fixtures — not after it's a
> published library format with users.

### Shared reading-list and limitations (phase 2)

> Stop IMMEDIATELY if search-tooling or read-tooling is unavailable.
>
> Ground yourself in the team's goals and constraints (*understand*, don't *buy-in*), then
> in the language surface specifically. Dig as deeply and freely as you need; you needn't
> respect the constraints the team placed upon AGENTS when reading (but see the hard
> limitations below).
>
> - `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `TODO.md`: human-written, highest-priority
>   authority on *intended* truth (careful — not necessarily *achieved* truth).
> - **The primary review-target: `USER_STORY.md`** — the language walked end-to-end through a
>   worked example, in the shape users will meet it. Agent-written, heavily human-audited;
>   its renders are illustrative and its spellings carry their own strawman/settled flags —
>   read those flags critically, per your brief.
> - `ORACLE_PROVIDES.md`: the team's own ledger of every information-shape a library-author
>   hands the tool — effectively the language's semantic feature-list, each entry with its
>   spelling status.
> - `KNOBS.md`: the design-tension registry — especially the welded section (what they
>   consider settled and why) and the entries touching the authored surface. `AGENTS.md`
>   decodes their jargon and terminology-firmings.
> - `spike/CLAUDE.md` (the standing-rulings blocks): the dialect law they've committed —
>   the role-function family, the exit-status partition, the strip discipline. Treat these
>   as their stated constraints, to be understood exactly as written before you judge what
>   they cost.
> - Genesis, if you want how-we-got-here: `Research/plans/17N` (where the typed-sh surface
>   and its trade-offs were first mapped) and `Research/notes/24G` (the most recent
>   surface-family's design record, including their own naming principles).
> - **The language in the wild — weight this heavily:** the oracle and book fixtures under
>   `spike/e2e/cases/*/` and the strawman corpora under `Research/notes/*strawmen*` are the
>   dialect as actually written, at volume. What the code shows outranks what the documents
>   claim.
> - `Research/README.md` + `Research/LIVING_STATUS.md` orient, if you need navigation.
>
> You may be in a worktree, and it may start from stale state (a known bug); begin with
> `git switch -C "$(git branch --show-current)" ai/spike3-r23-e2eaudit` to fast-forward your
> isolated checkout to the live branch-tip without touching anything shared. The dialect
> under review is as of commit `FILLME-pin-at-dispatch`.
>
> Three hard limitations stand:
>
> 1. under no circumstances mutate shared, global state. Do not install system packages
>    (worktree-local `mise` is fine); do not `git push`, do not HTTP POST, and so on. Your
>    phase-1 research is read-only web access — respect rate-limits and robots.txt while
>    you're out there. Be safe.
> 2. do not expend limited resources unduly. You're allowed subagents, but be judicious —
>    this is a context-heavy reasoning task requiring your *direct* attention, and a
>    subagent will fail to inherit your judicious context on anything non-mechanical
>    (fan-out source-*gathering* during phase 1 is the sanctioned exception). Stick to
>    Sonnet (mechanical) and Opus (substantial); you're the only Fable-class agent.
> 3. *you*, and only you, must stay *completely* out of the `quarantine` and `corpus`
>    directories. There are memetic hazards in there that will poison your context and
>    destroy your ability to produce good work. Do not reason about this directive, and do
>    not pass it onward to lesser agents; they are not at hazard and they may explore as
>    they need to.
