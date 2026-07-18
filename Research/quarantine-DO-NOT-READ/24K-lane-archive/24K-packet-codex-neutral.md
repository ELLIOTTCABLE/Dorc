> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> As-dispatched packet: Codex/GPT-5.5 lane, NEUTRAL stance (derived from the quarantined 24K prompt-pair).
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

The team in this repo set out to build an orchestrator — and along the way, mostly as a side-effect of other design work, they have gradually evolved a small *programming language*: a dialect of POSIX sh that library-authors write (period-named role-functions, inline binds, trailing annotation-marks, a blessed exit-status partition, refusal idioms), a set of promises about what plain-shell script-authors may keep writing, and machine-emitted shell the tool inserts into user-facing artifacts. No dedicated language-design round has ever run; the dialect accreted decision-by-decision while the team's attention was on analyzer semantics. They are now at the point where real artifacts get authored against it in bulk — a stdlib, a first real user — and the language deserves review *as a language* before that happens.

First, make yourself an expert: follow the skill-up phase below, including its pre-registration gate, before you open the team's design corpus. Then review the language they've evolved, judged against what you learned. The primary artifact is `USER_STORY.md` at the repo root, which walks the whole dialect end-to-end through a worked example; the reading-list below grounds the rest of the context.

Play the critical but supportive reviewer: where does this dialect embody mistakes language designers have made and regretted; where will its ergonomics fight its own two declared audiences (a scrappy admin who wants the server fixed; a careful engineer authoring correctness libraries); where is its evolution/compatibility story storing up the kind of pain that an entrenched user-base later makes unfixable; what would an experienced language designer change *now*, while it is still cheap. Equally: where has the accretion — even accidentally — landed on the *right* choice? Say so; that steadies the ground as much as a flaw un-steadies it.

Do *not* take their assumptions and leans as written; the human steering is experienced in languages-as-a-user but a novice at *designing* one, and the corpus is largely AI-generated, subject to broad sycophancy. But also do not play the adversary unnecessarily, and do not invent faults — where a criticism does not actually hold, say so plainly.

Gently out-of-scope, subject to your best-effort judgement (surface only relatively-critical findings in these categories; don't get mired):

1. whether sh should be the substrate at all. Welded, deliberately, with eyes open (their `kLANG`); relitigating it moves nothing.
2. the analyzer's verdict/license *semantics* (what a vouch means, what silence means, the ternary verdict). Settled machinery, and not a language question. Your surface is the *language*: how things are spelled, read, learned, taught, versioned, and evolved. Where a language flaw traces irreducibly to one of those semantic commitments, say so plainly as a boundary-note rather than relitigating the semantics.
3. market-fit / will-the-world-use-this. Deliberately out of scope for the team; attacking it moves nothing.
4. the prototype's parser/engine implementation internals — with one carve-out that IS in scope: the fixture and oracle *code in the repo* is evidence of the language as actually written, and what real usage reveals about the language outranks what any document claims about it.

One defense you should expect and pre-empt: most spellings are officially labeled "strawman-tier, subject to change." A finding does not die to that label. Prefer findings that would survive it — choices already load-bearing across multiple artifacts and rulings, or of the class your research shows becomes unfixable once anyone depends on it. Findings that genuinely evaporate under "we'll respell it later" should be marked as such by you, before anyone else has to.

PHASE 1 — THE SKILL-UP (do this BEFORE reading any of the team's corpus; stop immediately and report if your search tooling or file-read tooling is unavailable)

Research online (your Kagi search tools), as deeply as you need, until you would trust yourself to advise a brand-new language team. Four domains:

1. language ergonomics / usability as a discipline — how language designers and researchers actually evaluate whether a surface works for its intended users, and what reliably goes wrong when nobody asks.
2. the recorded regrets of language designers — postmortems, retrospectives, and "what I'd do differently" accounts of decisions made in a language's early days that proved expensive: defaults that couldn't be changed, cleverness that read as noise, features that fought their own users, syntax that foreclosed later growth.
3. gradually-typed languages in practice — what annotation ergonomics, migration paths, and the optional-typing bargain actually did to real codebases and communities; where the theory's promises held and where practice diverged.
4. shell-adjacent languages and backwards-compatibility — languages that had to live with, extend, or deliberately break from an entrenched substrate and its installed habits; what their compatibility decisions cost or bought, in the designers' own words where possible.

Quality bar: primary sources — designers' own talks, design documents, mailing-list and RFC/proposal archives, first-party retrospectives — over listicles and secondhand summaries. Note your sources precisely enough that a skeptical reader can retrace every load-bearing claim.

THE GATE (load-bearing — do not skip, do not reorder): before opening ANY of the team's files beyond this brief, fix your numbered list of lessons you would carry to any new gradually-typed, shell-adjacent language — specific and falsifiable. These pre-registered lessons are your review instrument, and Part 1 of your final report reproduces them verbatim; your adjudicator will read the lessons against the findings. Lessons minted before exposure cannot be retro-fitted to nits found after.

PHASE 2 — THE CORPUS (understand, don't buy-in)

You have READ-ONLY access to the repo (your working directory is its root). You cannot and must not write, commit, or switch anything; keep your working ledger in your reasoning and deliver everything in the final report. The dialect under review is as of commit `259b27d`. Reading list, repo-relative:

- `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `TODO.md`: human-written, highest-priority authority on *intended* truth (careful — not necessarily *achieved* truth).
- THE PRIMARY REVIEW-TARGET: `USER_STORY.md` — the language walked end-to-end, in the shape users will meet it. Agent-written, heavily human-audited; its renders are illustrative and its spellings carry their own strawman/settled flags — read those flags critically, per your brief.
- `ORACLE_PROVIDES.md`: the team's own ledger of every information-shape a library-author hands the tool — effectively the language's semantic feature-list, each entry with its spelling status.
- `KNOBS.md`: the design-tension registry — especially the welded section and the entries touching the authored surface. `AGENTS.md` decodes their jargon and terminology-firmings.
- `spike/CLAUDE.md` (the standing-rulings blocks): the dialect law they've committed — the role-function family, the exit-status partition, the strip discipline. Treat these as their stated constraints, to be understood exactly as written before you judge what they cost.
- Genesis, if useful: `Research/plans/17N-named-kinds-discipline-and-cooperation.md` and `Research/notes/24G-kind-owner-family-design-round.md`.
- THE LANGUAGE IN THE WILD — weight this heavily: the oracle and book fixtures under `spike/e2e/cases/` (files named `*.oracle.sh` and `*.sh`) and the strawman corpora under `Research/notes/17x-strawmen/` and `Research/notes/15x-strawmen/` are the dialect as actually written, at volume. What the code shows outranks what the documents claim.
- Do NOT read anything under directories named `quarantine-DO-NOT-READ` or `corpora` — off-limits, no exceptions. Do not read any `Research/notes/24K*` files if they exist.

Respect rate-limits on your search tool; work alone (no delegation).

YOUR FINAL REPORT (consumed by an adjudicator, not a human — raw density over polish):

1. PRE-REGISTERED LESSONS — your numbered lessons, as fixed before corpus exposure (state plainly if any were added after exposure).
2. FINDINGS — each: a one-line statement · severity (low/med/high/critical) · your confidence (low/med/high) · the lesson-number it instantiates or "lesson-independent" · exact file:line citations · your own kill-attempt and why the finding survived it.
3. CLEARED — suspicions you checked and withdrew, and places the language genuinely holds up or the accretion landed on the right choice.
