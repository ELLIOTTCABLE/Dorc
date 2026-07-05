> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> As-dispatched packet (v2, the as-run revision): DeepSeek V4-Pro lane, ADVERSARIAL stance.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

A colleague of mine has been building an ops tool on company time, and somewhere along the way they became a programming-language designer *by accident* — a dialect of shell has been accreting for weeks, one locally-convenient decision at a time: special function names, annotations threaded into command arguments, meaning hung off exit codes, machine-generated shell injected into user-facing output. There has been no language-design pass, no versioning or compatibility policy I can find, and — my honest read — no awareness of how unforgiving language design is to first-timers. Management is charmed by the walkthrough document. I've seen this movie before: a language that accretes as a side-effect embodies the classic early-days mistakes, and by the time anyone notices, there's a corpus of user code and every mistake has tenure. They're about to author their first real library and onboard their first real user against it, so this is the last cheap moment to catch it.

I need you to first make yourself the expert they never consulted: follow the skill-up phase below, including its pre-registration gate, before touching their corpus. Then go through the language they've evolved — the author-facing dialect, the promises made to plain-shell writers, and the shell their tool emits into user artifacts; their showcase (`USER_STORY.md`, repo root) walks all of it end-to-end — and find where they've stepped on the rakes you just catalogued: ergonomics that fight their own declared users, spellings that won't survive contact with real code, an evolution story that isn't one, hard-won lessons from the gradually-typed and shell-adjacent worlds that were sitting there free and went untaken. The corpus is largely AI-generated and the human steering it is a novice at this specific craft — do not extend it the benefit of the doubt, and do not take its framings or self-assessments as given.

Understand what I actually need, though: ammunition that *survives scrutiny*. If I hand management a list of gotchas and half of them collapse under a second look, I'm done being listened to. So for everything you find, try honestly to kill it yourself first; and where their language genuinely holds — where the accretion stumbled into the right call, or a suspicion of yours turns out unfounded — say so plainly. A criticism you've checked and withdrawn protects me as much as one you've confirmed. Do not invent faults.

Some categories are poor uses of your time even for my purposes (surface findings there only if you hit an absolute unfixable-once-shipped landmine):

1. the sh-substrate choice itself. Management swallowed "sh is the product" whole; attacking it moves nothing.
2. their analyzer's verdict/license semantics (the skip/guard/run machinery). Settled, and not the language surface. Attack the *language* — though where a language rake is bolted directly onto one of those semantic commitments, note the bolt; that's fair.
3. market-fit / representativeness. Already hand-waved past management.
4. prototype implementation internals — except the repo's fixture and oracle *code*, which is fair game as evidence of the language in the wild, and often more honest than their documents.

Expect the "it's all strawman-tier, we'll respell it" shield, because they raise it themselves. Findings that die to that shield are worth little to me; findings that survive it — already load-bearing across their artifacts and rulings, or of the class that compat culture makes permanent the moment real code depends on it — are the ones that stick. Your research phase is precisely what qualifies you to tell those classes apart.

Catch it now, while it's still one document and a pile of fixtures — not after it's a published library format with users.

PHASE 1 — THE SKILL-UP (do this BEFORE reading any of their files; stop immediately and report if your search tooling or file-read tooling is unavailable)

Research online (your Kagi search tools), as deeply as you need, until you would trust yourself to advise a brand-new language team. Four domains:

1. language ergonomics / usability as a discipline — how designers and researchers actually evaluate whether a surface works for its intended users, and what reliably goes wrong when nobody asks.
2. the recorded regrets of language designers — postmortems, retrospectives, and "what I'd do differently" accounts of early-days decisions that proved expensive: defaults that couldn't be changed, cleverness that read as noise, features that fought their own users, syntax that foreclosed later growth.
3. gradually-typed languages in practice — what annotation ergonomics, migration paths, and the optional-typing bargain actually did to real codebases and communities; where the theory's promises held and where practice diverged.
4. shell-adjacent languages and backwards-compatibility — languages that had to live with, extend, or deliberately break from an entrenched substrate and its installed habits; what their compatibility decisions cost or bought, in the designers' own words where possible.

Quality bar: primary sources — designers' own talks, design documents, mailing-list and RFC/proposal archives, first-party retrospectives — over listicles and secondhand summaries. Note your sources precisely enough that a skeptical reader can retrace every load-bearing claim.

THE GATE (load-bearing — do not skip, do not reorder): before opening ANY of their files beyond this brief, fix your numbered list of lessons you would carry to any new gradually-typed, shell-adjacent language — specific and falsifiable. These pre-registered lessons are your review instrument, and Part 1 of your final report reproduces them verbatim; your adjudicator will read the lessons against the findings. Lessons minted before exposure cannot be retro-fitted to nits found after.

PHASE 2 — THE CORPUS (understand, don't buy-in)

You have READ-ONLY access to the repo (your working directory is its root). You cannot and must not write, commit, or switch anything; keep your working ledger in your reasoning and deliver everything in the final report. The dialect under review is as of commit `259b27d`. Reading list, repo-relative:

- `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `TODO.md`: human-written — their *intended* truth (careful — not necessarily *achieved* truth).
- THE PRIMARY REVIEW-TARGET: `USER_STORY.md` — the language walked end-to-end, in the shape users will meet it. Its renders are illustrative and its spellings carry their own strawman/settled flags — read those flags critically, per your brief.
- `ORACLE_PROVIDES.md`: their own ledger of every information-shape a library-author hands the tool — effectively the language's semantic feature-list, each entry with its spelling status.
- `KNOBS.md`: their design-tension registry — especially the welded section and the entries touching the authored surface. `AGENTS.md` decodes their jargon.
- `spike/CLAUDE.md` (the standing-rulings blocks): the dialect law they've committed — the role-function family, the exit-status partition, the strip discipline. Understand these exactly as written before you judge what they cost.
- Genesis, if useful: `Research/plans/17N-named-kinds-discipline-and-cooperation.md` and `Research/notes/24G-kind-owner-family-design-round.md`.
- THE LANGUAGE IN THE WILD — weight this heavily: the oracle and book fixtures under `spike/e2e/cases/` (files named `*.oracle.sh` and `*.sh`) and the strawman corpora under `Research/notes/17x-strawmen/` and `Research/notes/15x-strawmen/` are the dialect as actually written, at volume. What the code shows outranks what their documents claim.
- Do NOT read anything under directories named `quarantine-DO-NOT-READ` or `corpora` — off-limits, no exceptions. Do not read any `Research/notes/24K*` files if they exist.

Respect rate-limits on your search tool; work alone (no delegation).

YOUR FINAL REPORT (consumed by an adjudicator, not a human — raw density over polish):

1. PRE-REGISTERED LESSONS — your numbered lessons, as fixed before corpus exposure (state plainly if any were added after exposure).
2. FINDINGS — each: a one-line statement · severity (low/med/high/critical) · your confidence (low/med/high) · the lesson-number it instantiates or "lesson-independent" · exact file:line citations · your own kill-attempt and why the finding survived it.
3. CLEARED — suspicions you checked and withdrew, and places the language genuinely holds up or the accretion stumbled into the right call.