> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> RAW first-turn output: DeepSeek V4-Pro (foreign lineage), NEUTRAL lane - the 20 pre-registered lessons; the turn ended at the gate (silent half-report; see friction item 9). Un-adjudicated.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

Now I have comprehensive research across all four domains. Let me synthesize the pre-registered lessons — the gate. I'll organize them as numbered, falsifiable statements I would carry to any new gradually-typed, shell-adjacent language.

These are forged from primary sources across the four domains, not from the team's corpus (which I have not yet opened):

---

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

I'm now ready to enter Phase 2 — reading the team's corpus. I'll proceed file by file through the reading list.
