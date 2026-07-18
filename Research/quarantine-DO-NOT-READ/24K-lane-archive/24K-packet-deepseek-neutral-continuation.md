> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> As-dispatched continuation packet: DeepSeek NEUTRAL lane ended its first turn at the pre-registration gate; this fed its own lessons back verbatim to obtain the findings phase.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

CONTINUATION OF YOUR OWN IN-PROGRESS REVIEW. You already completed Phase 1 (the skill-up + pre-registration gate) in a prior turn; your twenty pre-registered lessons are reproduced VERBATIM at the bottom of this packet — they are yours, do not re-derive or alter them. Proceed DIRECTLY to Phase 2 and Phase 3 of the brief below: read the corpus per the reading list, then deliver the COMPLETE final report in one message — Part 1 (reproduce the lessons below verbatim), Part 2 (FINDINGS), Part 3 (CLEARED). Do NOT stop after reading; the report is the deliverable. The original brief follows.

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
3. CLEARED — suspicions you checked and withdrew, and places the language genuinely holds up or the accretion stumbled into the right call.

===== YOUR PHASE-1 PRE-REGISTERED LESSONS (verbatim, already written by you) =====

## PART 1 — PRE-REGISTERED LESSONS

### Defaults & Lock-in

**L1.** Every default becomes load-bearing within a surprisingly small user-base (hundreds, not millions). Change a default after adoption begins and you *will* break someone. The only credible escape hatches are: (a) a new opt-in mode that old code never sees (Rust editions, Go GODEBUG), or (b) a whole new language identity under a different name (Oil→YSH, Perl 6→Raku).

**L2.** Syntax choices — even trivial-seeming ones like whether annotations use colons or parentheses — can never be fully reclaimed. The grammar once shipped is a permanent lien on future syntax additions. Any language that hopes to grow over time must leave deliberate, explicit syntax headroom.

**L3.** The null/absent/silent-default problem (Tony Hoare's "billion-dollar mistake" generalized): every language construct that can silently default to a meaning the author didn't intend — silent error-ignoring, implicit coercion, default values that look correct — will eventually cause a production incident. The cost accumulates over the language's lifetime, and it is nearly always unfixable after adoption.

### Dual-audience design

**L4.** A language with two distinct user populations (casual/scripting vs professional/library-author; admin vs engineer) cannot satisfy both with one surface. The friction-free path for Population A *will* be the footgun for Population B, and vice versa. The only known mechanism that works is *progressive disclosure of complexity*: a simple surface that can be gradually tightened/annotated, not two separate modes that require a phase transition to cross. Oil chose "same interpreter, two modes, gradual transition via shopt." TypeScript chose "superset — everything JS is valid TS, but you can add types." Both are trying to solve this; neither fully succeeds without pain.

**L5.** "Syntactic salt" — making undesirable things possible but awkward — works better than "syntactic sugar" — making desirable things easier — for guiding behavior, because salt resists normalization-of-deviance and doesn't create the "everyone uses the sugar even when it's wrong" problem.

### Migration & gradual typing

**L6.** Gradual typing lives or dies on the migration experience, not the type-theoretic properties. The question is not "is the type system sound?" but "can I add one type annotation to my existing code without changing anything else, and does the tooling improve *immediately*?" If annotations require refactoring before they yield value, adoption stalls. (Evidence: TypeScript's IDE-first approach, Python typing's mixed reception.)

**L7.** The boundary between typed and untyped code is where all the pain concentrates. If the language design doesn't explicitly address what the analyzer/compiler does at that boundary — what it trusts, what it checks, what it warns about — users will fill the gap with incorrect mental models. "Shut-up" escape hatches (`any`, `# type: ignore`, `as`) are operationally necessary but must degrade gracefully: suppressing one check should not cascade into silently trusting more than intended.

**L8.** Annotation syntax must be syntactically foreign to the host language's executable semantics. If an annotation looks like it could be a runtime operation, users will expect it to *be* a runtime operation. Python's `x: int` works because colon-in-assignment was previously unused and syntactically illegal; TypeScript's `x: string` works because colon-after-variable-name is not meaningful JS. Annotations that look like commands will confuse users about when they execute.

**L9.** The "I'll fix the syntax later" label is only credible before there are users. Once *any* artifact — a tutorial, a stdlib, a third-party library — is written against the current spelling, that spelling is load-bearing. "Strawman syntax" becomes permanent syntax within weeks of anyone depending on it.

### Error handling

**L10.** Error handling is the connective tissue of every program; its design shapes every line of user code. Two well-established traps: (a) silent swallowing (bash's errexit quirks, ignored return values) creates debugging nightmares; (b) excessive ceremony (Go's `if err != nil` at every call site) creates fatigue and encourages copy-paste. The language must distinguish intent — "I handled this" vs "I forgot about this" vs "I'm deliberately ignoring this" — at the call site, visibly.

**L11.** In the shell domain specifically: conflating "boolean false" with "error" at the exit-code level is a standing design mistake that propagates into every conditional. Any shell-adjacent language must solve the `false`-vs-`error` partition at the substrate level, because users cannot reliably solve it in user code. (Evidence: Oil's `boolstatus` builtin.)

### Notation & cognitive dimensions

**L12.** Hidden dependencies are the most expensive class of notation defects. When action-at-a-distance exists — errexit behavior changing inside a conditional without visible indication, a declaration in file A affecting parsing in file B — it must be visible at the *reading* site, not just the *declaring* site. Every dependency that crosses a lexical boundary should be greppable.

**L13.** Consistency across related constructs is a direct multiplier on learnability. Each special case a user must remember is a standing cognitive tax. The cognitive-dimensions framework measures this as "consistency" — notations with high consistency are easier to learn and harder to misuse. Specific threat: unprincipled exceptions accumulate during "accretion" development and are nearly impossible to remove later without breaking users who learned the exceptions as features.

**L14.** The notation's "abstraction gradient" — how steeply a user must climb from "I can write a one-liner" to "I can build a reusable library" — determines whether casual users ever become power users. A cliff between "just scripting" and "library-authoring" kills the pipeline from one audience to the other.

**L15.** Viscosity in a shell-adjacent language is especially dangerous: if adding a type annotation to a function forces cascading annotations throughout its callers, the annotation effort scales with codebase size rather than with need, and adoption will be inversely proportional to codebase size. Type inference boundaries are therefore not a performance concern but a *migration* concern.

### Shell-adjacent specifics

**L16.** Shell's fundamental design flaw is the entanglement of parsing and expansion: what the parser sees is not what the runtime evaluates. Any shell-adjacent language that hopes to be statically analyzable must commit to a clear phase separation. Without it, analysis is always approximate and the tool's confidence must degrade in ways the user cannot see. Oil acknowledged this by creating OSH (approximate analysis of bash) and YSH (clean phase separation for analyzable code). A language that claims to analyze sh must be explicit about which shell constructs break the analyzer and what the analyzer does when it encounters them.

**L17.** Compatibility with existing sh scripts is a one-way door. Breaking compatibility buys you a cleaner language; keeping it buys you an installed base. Different projects have chosen differently (Oil chose both via OSH/YSH; Nushell chose to break; PowerShell chose to break but on a different platform). There is no universally correct answer, but the answer has irreversible downstream consequences, and the community will litigate it forever. Choose explicitly and document the rationale permanently.

### Language as product

**L18.** The error messages, tutorial path, reference documentation, and the names of things are as much the language as its grammar. A user who cannot understand an error message cannot fix the bug — and blames the language, not themselves. (Evidence: Rust's investment in error messages, Elm's legendary compiler messages, Go's clarity as deliberate design goal.)

**L19.** Backwards compatibility is a feature, not an absence of change. It requires active design: what happens when old code meets a new analyzer? What happens when new code imports an old library? What happens when an old script runs under a new runtime? "We'll figure it out later" is the policy that produced the Python 2→3 decade.

**L20.** A language's name is its most permanent syntax. Renaming is expensive (Perl 6→Raku), confusing to users (Oil→Oils/OSH/YSH), and rare. Pick the name and its scope boundaries before users form habits around the old one.

---

**Pre-registration timestamp**: These 20 lessons were formulated on 2026-07-05 based on research across the four domains specified in the brief, *before opening any team corpus files beyond the AGENTS.md already in context*. No lessons were added or substantively altered after corpus exposure.
