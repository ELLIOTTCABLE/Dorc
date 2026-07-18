> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> RAW continuation-turn report: DeepSeek V4-Pro (foreign lineage), NEUTRAL lane. Un-adjudicated.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

I now have sufficient understanding of the corpus. Let me compose the final report.

---

# FINAL REPORT — Dorc Language Review

## PART 1 — PRE-REGISTERED LESSONS

(Reproduced verbatim as fixed before corpus exposure; see the pre-registration statement at the end.)

**L1.** Every default becomes load-bearing within a surprisingly small user-base (hundreds, not millions). Change a default after adoption begins and you *will* break someone.

**L2.** Syntax choices — even trivial-seeming ones — can never be fully reclaimed. The grammar once shipped is a permanent lien on future syntax additions.

**L3.** The null/absent/silent-default problem: every language construct that can silently default to a meaning the author didn't intend will eventually cause a production incident.

**L4.** A language with two distinct user populations cannot satisfy both with one surface. The only known mechanism that works is *progressive disclosure of complexity*.

**L5.** "Syntactic salt" — making undesirable things possible but awkward — works better than "syntactic sugar" for guiding behavior.

**L6.** Gradual typing lives or dies on the migration experience. "Can I add one type annotation without changing anything else, and does the tooling improve *immediately*?"

**L7.** The boundary between typed and untyped code is where all the pain concentrates. The design must explicitly address what the analyzer does at that boundary.

**L8.** Annotation syntax must be syntactically foreign to the host language's executable semantics. Annotations that look like runtime operations confuse users about when they execute.

**L9.** The "I'll fix the syntax later" label is only credible before there are users. "Strawman syntax" becomes permanent syntax within weeks of anyone depending on it.

**L10.** Error handling is the connective tissue of every program. The language must distinguish intent — "I handled this" vs "I forgot" vs "I'm deliberately ignoring" — at the call site, visibly.

**L11.** Conflating "boolean false" with "error" at the exit-code level is a standing design mistake. Any shell-adjacent language must solve the `false`-vs-`error` partition at the substrate level.

**L12.** Hidden dependencies are the most expensive class of notation defects. Every dependency that crosses a lexical boundary should be greppable.

**L13.** Consistency across related constructs is a direct multiplier on learnability. Unprincipled exceptions accumulate during accretion development and are nearly impossible to remove later.

**L14.** The notation's "abstraction gradient" — how steeply a user must climb from "I can write a one-liner" to "I can build a reusable library" — determines whether casual users ever become power users.

**L15.** Viscosity: if adding a type annotation forces cascading annotations throughout callers, adoption scales with codebase size rather than need, and inversely.

**L16.** Shell's fundamental design flaw is the entanglement of parsing and expansion. Any shell-adjacent language that hopes to be statically analyzable must commit to clear phase separation and be explicit about which constructs break the analyzer.

**L17.** Compatibility with existing sh scripts is a one-way door. Breaking compatibility buys a cleaner language; keeping it buys an installed base. Choose explicitly.

**L18.** Error messages, tutorial path, reference documentation, and the names of things are as much the language as its grammar.

**L19.** Backwards compatibility is a feature requiring active design: what happens when old code meets a new analyzer? Old script under new runtime?

**L20.** A language's name is its most permanent syntax. Pick the name and its scope boundaries before users form habits.

**Pre-registration timestamp**: These 20 lessons were formulated on 2026-07-05 based on research across the four domains specified in the brief, *before opening any team corpus files beyond the AGENTS.md already in context*. No lessons were added or substantively altered after corpus exposure.

---

## PART 2 — FINDINGS

### FINDING 1: The strip-discipline creates two languages users must hold in their head simultaneously

**Statement**: The dialect has a design-surface (period-named functions: `foobar.is_converged()`) and a mechanical-surface (double-underscore: `apt_get__predict()`), and authors must reason about both, because both appear in the same file — the source uses dots, the off-ramp artifact uses underscores — but the strip transform is not the identity transform, so the off-ramp script is *not* the authored script.

**Severity**: High. **Confidence**: High.

**Instantiates**: L9 (strawman permanence — the double-underscore is already load-bearing across 187 oracle fixtures), L13 (consistency — two naming conventions for the same thing), L8 (annotations look like runtime operations).

**Citations**:
- `USER_STORY.md:249` — `foobar.is_converged()` (design-surface, single dot)
- `spike/e2e/cases/headline-pi-webhost/package.oracle.sh:6` — `apt_get__predict()` (mechanical-surface, double underscore)
- `spike/CLAUDE.md:179-180` — "shipped strip-only (annotations removed, `name.predict()` → `name_predict()`, nothing else changed)"
- `KNOBS.md:62` — "the off-ramp cost paid by the strip pass (strip-fidelity: bare marks delete whole-statement; `name.predict()` → `name_predict()`"
- All 187 `*.oracle.sh` files use `__predict`, not `.predict`

**Kill-attempt**: Could the design settle on one surface and eliminate the other? The dot form is syntactically illegal in POSIX sh (dots are not valid in function names per POSIX), so the double-underscore *must* exist for the off-ramp. The team could declare the off-ramp artifact "not Dorc's concern" — but DESIGN.md explicitly makes the trivial off-ramp a core value. The two-surface problem is structural: the designed language and the shipped language are different texts. Survives.

**Why it matters**: Every oracle author must learn: "I write `foo.predict()`, but it ships as `foo_predict()`, and annotations vanish." The mental model requires holding both texts. When debugging a plan, the user sees `foo_predict` in the render but wrote `foo.predict` — a permanent, low-grade cognitive translation tax.

---

### FINDING 2: The inline annotation syntax is off-ramp-hostile, contradicting a core DESIGN promise

**Statement**: The central annotation construct `name : Kind = value` is not inert under stock shells — `dash` aborts with `local: :: bad variable name` — meaning the "absolutely trivial" off-ramp (`ssh host 'dash -s' <script`) is broken without running Dorc's stripper first.

**Severity**: Critical. **Confidence**: High.

**Instantiates**: L8 (annotation syntax must be syntactically foreign to host semantics — here it crashes the host), L17 (the compatibility decision's downstream costs), L14 (abstraction-gradient cliff — the off-ramp now requires tooling).

**Citations**:
- `DESIGN.md:63-64` — "the first 'I used Dorc successfully' is five minutes in: `apt-get install dorc; printf 'apt-get install docker' >docker-host.dorc.sh; dorc apply docker-host.dorc.sh some-host.my-domain`"
- `DESIGN.md:76-77` — "if Dorc fails you in any way, the off-ramp is absolutely trivial, both in the immediate term (`ssh some-host.tld 'dash -s' <myscript.dorc.sh`)"
- `Research/plans/17N-named-kinds-discipline-and-cooperation.md:36-42` — "Under stock `dash`, `local w : Foo = "$1"` aborts (`local: :: bad variable name`); under `bash` it silently leaves `w` empty and returns 0 (corruption, not a crash); the dotted name fails `dash -n`"
- `KNOBS.md:61-62` — "`kTYANNOT-inline` annotates directly on a command argument — ergonomic, intuitive, significant-meaning-in-place — but is *not inert* under stock shells (aborts or silently corrupts; verified, `Research/plans/17O` F-OFFRAMP), so it demands a correctness-critical strip pass"
- `KNOBS.md:63` — "the eol pole was never actually clean (the `kOOB` breach)"
- `spike/e2e/cases/headline-pi-webhost/package.oracle.sh:10` — `pkg : package = "$1"` — live annotation in fixture code

**Kill-attempt**: The team could argue "oracles always require stripping; books stay verbatim-runnable." This is the current containment: annotations live only in oracle bodies. But (a) the `kOOB` redline forbids comment-parsing for the eol-comment alternative, so the team has boxed themselves into inline as the only path forward while acknowledging it breaks the off-ramp; and (b) DESIGN's "absolutely trivial off-ramp" promise is a *product* promise, not an oracle-author promise — it's the admin who needs the off-ramp, and the admin's book is clean only because all the dirt is hidden in libraries. The containment works *if* the admin never authors annotations, but the USER_STORY's stage 3 explicitly shows the admin writing `foobar.is_converged()` with inline annotations in their own file (`USER_STORY.md:236-246`). The containment leaks at the very moment of the tool's value proposition.

**Why it matters**: This is a DESIGN-level contradiction. The team simultaneously claims (a) trivial off-ramp via plain-sh execution and (b) inline annotations as the type-surface. Both poles of `kTYANNOT` violate a core principle — inline breaks the off-ramp; eol-comment breaks `kOOB`'s no-comment-parsing redline. The team is *aware* of this trade (17N and KNOBS document it thoroughly), but the resolution ("de-facto inline, formal weld human-reserved") is a stay of execution, not a resolution.

---

### FINDING 3: The colon-mark grammar overloads one syntactic form with five distinct semantic roles

**Statement**: The trailing colon annotation (`:`) serves as entity-binding, state-establishment, state-negation, observation/dependency, and emission-typing — five distinct semantics on indistinguishable syntax, distinguished only by position and character-level micro-syntax (`:` vs `:?` vs `!` suffix).

**Severity**: Medium. **Confidence**: High.

**Instantiates**: L13 (consistency — same syntax, different semantics), L18 (the notation *is* the language — readers must decode which meaning applies from context).

**Citations**:
- `USER_STORY.md:249` — `dest : fb.Certs = "$1"` (entity-binding — `: Kind = value`)
- `USER_STORY.md:250` — `: fb.Certs:"$dest".synced` (state-establishment — `: kind:entity.prop`)
- `ORACLE_PROVIDES.md:96-97` — "trailing establish-marks (`probe-cmd : fb.Certs:"$dest".synced` — this exit status establishes that cell), the `!` exit-code-inversion plumbing, the `:?` observe/depends-upon mark"
- `spike/e2e/cases/headline-pi-webhost/package.oracle.sh:13` — `: package:"$pkg".installed` (establish) and `: package:"$pkg".installed!` (negated establish)
- `spike/e2e/cases/y1-redirect-write-invalidates-query/confline.oracle.sh:11` — `:? confline:"$file".present` (observe/depends)
- `Research/notes/24G-kind-owner-family-design-round.md:77-80` — `printf '%s\n' "$1" : service` (emission-typing — kind as trailing annotation on stdout-emitting lines)
- `ORACLE_PROVIDES.md:99` — "the surviving bare per-cell marks (ACK, POISON)"

**Kill-attempt**: Is the overloading actually confusing in practice? The team might argue that position disambiguates: binding occurs on assignment lines, marks on probe-command lines, emission-typing on stdout-emitting lines. But the USER_STORY and ORACLE_PROVIDES both acknowledge the complexity — the `:?` observe mark is semantically distinct from establish, the `!` inversion changes the logic, and stage 7's emission-typing repurposes the colon yet again. The *design documents* struggle to keep them distinct (ORACLE_PROVIDES exists partly because the marks went "conversationally invisible"). If the design-team needs a dedicated ledger to track which colon means what, users will too.

**Why it matters**: This is a classic accretion problem. Each colon-role was added at a different time to solve a different problem. Individually, each is defensible. Collectively, they create a micro-grammar that an author must internalize. The cost is paid every time someone reads an oracle body — "is this colon establishing, observing, or typing?" — and misreading produces silent wrongness.

---

### FINDING 4: The unregulated kind namespace is a coordination failure waiting to happen

**Statement**: Kind names are declared by individual oracle authors with no registry, no namespacing convention, and no cross-author conflict detection beyond within-book linting. Two authors who independently mint `package` and `pkg` for the same referent produce disjoint kinds, silently degrading cross-oracle reasoning to the `⊤` floor.

**Severity**: High. **Confidence**: Medium.

**Instantiates**: L12 (hidden dependencies — the dependency between two oracles' kind names is invisible at either declaration site), L4 (dual-audience — the engineer naming a kind cannot anticipate all admins who will consume it).

**Citations**:
- `USER_STORY.md:261` — "Nobody approves kind names; there is no registry. It only has to agree with itself."
- `ORACLE_PROVIDES.md:52-54` — "Nobody approves kind names; there is no registry; a kind only has to agree with itself (the round-17 symbol-grounding settlement: identity is *declared*, never inferred)"
- `Research/plans/17N-named-kinds-discipline-and-cooperation.md:423-425` — "silent same-name-different-meaning *never conflicts*. The boolean 'unsure ⇒ false ⇒ run' default protects *within-oracle* uncertainty; it does nothing against *cross-oracle* over-correlation"
- `KNOBS.md:66` — "coherence as a CONTRACT ... Dorc can't *enforce* it (never rejects plain sh) ⇒ a kind-owner-declared contract + best-effort CI lint"
- `seam-two-providers-one-kind/book.sh:1-2` — `apt-get install -y nginx` / `yum install -y httpd` — two providers, both use `package` kind — the happy path where authors converge, not the failure path where they don't

**Kill-attempt**: The team might argue that reverse-DNS kind names (17N §6 C2: `net.example.wombat`) solve this, and that the current fixtures use short names only because they're spike-internal. But (a) reverse-DNS is listed as a strawman in 17N, not a settled spelling; (b) the USER_STORY uses `fb.Certs`, not `com.example.foobar.Certs`; (c) the design explicitly rejects a central registry; and (d) the cross-oracle collaboration that is the tool's value proposition *requires* kind-name agreement. The "no registry" decision is principled (avoids bureaucracy), but it offloads the coordination cost onto every author who wants their oracle to compose with others'. Survives because the alternative (central registry) is explicitly rejected and the reverse-DNS alternative is unsettled.

**Why it matters**: This is the tool's core value-prop at risk. If two oracles for related tools use different kind names, no cross-oracle reasoning occurs — walls stay walls, guards stay guards. The "silence licenses nothing" safety principle means the failure mode is degraded value, not incorrectness, but for a tool whose value hinges on elision, degraded value *is* the failure mode.

---

### FINDING 5: Conflating the answer with the license eliminates a valuable authorial position

**Statement**: The current spelling welds `is_converged()` (providing an answer) to the license to act on it. An oracle author who wants to provide convergence information *without* authorizing automated skips has no mechanism to do so — every `is_converged()` implicitly licenses elision. The ORACLE_PROVIDES ladder (rung-0 through rung-2) acknowledges this as a live tension but the incumbent ruling is `kCONTRACT-RUNGS-single`.

**Severity**: Medium. **Confidence**: High.

**Instantiates**: L10 (error handling — conflating "I measured this" with "I accept blame for this"), L5 (syntactic salt — the dangerous act of licensing a skip should be *more* syntactically visible than the safe act of reporting state).

**Citations**:
- `spike/CLAUDE.md:203-204` — "[SPELLING RESOLVED — 2026-07-03, round 24, human-acked...] The vouch is no longer a mark: **authoring the verdict-function IS the vouching act**"
- `ORACLE_PROVIDES.md:167-222` — provides-license section describing the ladder (rung-0 through rung-2) as the "odd one out" and the "least sh-native thing in the entire design"
- `KNOBS.md:66-70` — `kCONTRACT-RUNGS`: "Single risks over-licensing by authors who only meant to inform (there is no way to provide a verdict without licensing skips)"
- `USER_STORY.md:258-259` — "the period-name is the opt-in semaphore ... authoring the verdict-function IS the vouching act"
- `ORACLE_PROVIDES.md:210-211` — "this ledger's taxonomy pressures an un-weld (the wary engineer who wants to hand over answers while standing on rung-0 or rung-1 — the engineer's escape hatch)"

**Kill-attempt**: The current ruling's strongest defense is simplicity: minimal ceremony, one act. The team acknowledges the ladder tension in ORACLE_PROVIDES and `kCONTRACT-RUNGS` but has deferred resolution pending field-trial data. The finding survives because (a) the weld is *already in the fixtures* — 187 oracle files all use `__predict()` without a separate license construct; (b) ORACLE_PROVIDES itself argues for un-welding; and (c) the ladder is acknowledged as "the least sh-native thing in the design," suggesting the team hasn't found a clean spelling for it — but "we haven't found a spelling" is different from "the concept doesn't belong."

**Why it matters**: The asymmetry matters. An author who provides `is_converged()` for a tool they partially understand is *forced* to license skips. The only escape is to not write the function at all — which means no drift-reporting, no hint-machinery benefits, no display annotations. An author who wants to say "here's what I know, but don't skip on my word" has no voice. The ladder's rung-0 (answers for display only) is a real user need that the current spelling forecloses.

---

### FINDING 6: The `kSURVIVAL` flag is self-described as theatre while being sold as safety

**Statement**: The team explicitly labels the `--trust-footprints` flag as "marketing at best ... and theatre at worst" (USER_STORY, `kSURVIVAL`), acknowledging that the opt-in provides false assurance. A language mechanism that its own designers describe as theatre undermines the trust relationship the tool's entire value proposition depends on.

**Severity**: Medium. **Confidence**: High.

**Instantiates**: L3 (silent-default — the flag's presence suggests safety where none exists), L18 (the mechanism *is* the language's user interface — if the UI is theatre, the language has a trust problem).

**Citations**:
- `USER_STORY.md:476-480` — "an honesty note that must outlive every future edit of this document: this opt-in is marketing at best (you chose the danger; it isn't a Dorc bug when an author's claim is wrong) and theatre at worst (it is desirable enough that nearly everyone will turn it on and forget it)"
- `KNOBS.md:147` — "the explicit flag — never a default, short enough not to alias away, honestly 'marketing at best, theatre at worst' and demanded anyway"
- `KNOBS.md:146-148` — `kSURVIVAL-trusted` pole description: "the design's one *naked* trust: a wrong at-most claim silently under-executes *someone else's* line with no runtime net"

**Kill-attempt**: The team's honesty is commendable and unusual. The argument for the flag's existence — "the choice should be typed by the person who owns the consequences, even when everyone types it" — is principled. But a flag whose designers call it theatre *and still ship it* creates a peculiar position: the tool tells the user "type this to enable dangerous behavior," the user types it, and later when it bites, the tool says "you chose the danger." The flag exists to attribute blame, not to provide safety — which is a coherent position, but one that should be named honestly in the product surface rather than disguised as a safety switch. Survives because the team's own documents make the case against it.

---

### FINDING 7: The argparse boilerplate in every oracle body is a viscosity tax that will silently resist improvement

**Statement**: Every oracle must re-implement its command's argparse in sh (two `while [ "${1#-}" != "$1" ]` loops, verb dispatch, arity gating), and this idiom is copied verbatim across 187+ fixture files. Any evolution of the argparse contract requires touching every oracle.

**Severity**: Low. **Confidence**: High.

**Instantiates**: L15 (viscosity — adding precision to one oracle's argparse requires local work, not systemic improvement), L14 (abstraction gradient — the boilerplate raises the cost of writing even a minimal oracle).

**Citations**:
- All 187 `*.oracle.sh` files contain substantially identical argparse prologues (two `while` loops, `verb=$1; shift`, `case $verb in`)
- `spike/e2e/cases/headline-pi-webhost/package.oracle.sh:7-16` — representative argparse body
- `ORACLE_PROVIDES.md:34-37` — "provides-decoding — 'here is how to read my command's invocations' ... The argparse: flag-stripping, operand extraction, dispatch — ordinary authored control-flow"
- `USER_STORY.md:348-353` — the verbose argparse in the stage-4 battle-ready oracle, including `[ "$2" = "" ]` arity gate and `printf 'UNK multi-operand foobar\n'`

**Kill-attempt**: The team might argue that the argparse is ordinary sh that the author would write anyway — it's not Dorc-specific boilerplate, it's defensive shell scripting. This is partially true: flag-stripping and dispatch *are* necessary for any tool that parses arguments. But (a) the specific two-while-loop pattern is *taught* by the fixtures as the canonical form, and (b) the arity gate (`[ "$2" = "" ]`) and the `UNK` breadcrumb idiom are Dorc-specific conventions. If the argparse contract ever needs to change (e.g., to support `--long-flag=value` syntax), every oracle needs updating. Low severity because the cost is distributed and the pattern is genuinely sh-native; it's a nit, not a defect.

---

### FINDING 8: The exit-status partition (0/1/≥2) is well-chosen and well-integrated

**Statement**: The blessed partition — 0 = named sense holds, 1 = complement, ≥2 = can't-say — inherits POSIX utility convention, makes refusal ordinary control-flow (`return 2` or unhandled case arm), and maps cleanly to the ternary plan-verdict (converged/diverged/can't-say → elide/guard/run).

**Severity**: N/A (positive finding). **Confidence**: High.

**Instantiates**: Lesson-independent — this is a place where accretion landed on the right choice. It directly addresses L11 (the `false`-vs-`error` partition problem) by making "can't-say" a distinct, un-ignorable status that always forces a run.

**Citations**:
- `spike/CLAUDE.md:247-255` — `rul-rc-partition`: "0 = the named sense holds; 1 = its complement; ≥2 = CONFUSED, and confusion always lands on run"
- `USER_STORY.md:268-269` — "`*) return 2` is the native *decline*. The exit-status partition is fixed and blessed: 0 = the named sense holds; 1 = its complement; anything ≥2 = 'can't say,' and can't-say always runs"
- `USER_STORY.md:267` — "declining is ordinary control-flow, not an annotation"

**Why it's good**: The partition solves the shell-specific problem that L11 identifies without introducing new syntax. It leverages existing POSIX convention. It makes refusal a first-class outcome rather than an error or a special return value. And it composes with the guard mechanism: `( foo_is_converged args ) || <original bytes>` naturally degrades can't-say to run because rc≥2 is truthy in `||`.

---

### FINDING 9: The monotonicity guarantee is principled and well-defended but the fixtures show signs of strain

**Statement**: The design promises that every oracle addition is monotonic — adding a function never removes previously-won value. This is a sound principle. However, the actual fixture set shows that the guarantee depends on the specific spelling of function names carrying rungs (FINDING 5), and the stringly-typed→typed-emission migration (USER_STORY stage 5 FIXME) represents a planned non-monotonic change to the oracle contract.

**Severity**: Low. **Confidence**: Medium.

**Instantiates**: L19 (backwards compatibility — the acknowledgment of a breaking change to `touches()` emission format), L9 (strawman permanence — the stringly-typed `kind:entity` format is already load-bearing in fixtures and documented in USER_STORY).

**Citations**:
- `ORACLE_PROVIDES.md:18-19` — "each shape, individually, eventually needs a fully-monotonic gradual-degradation story of the blessed form — *every partial-effort-increase yields vaguely-corresponding value; no partial-effort-increase removes previously-won value*"
- `USER_STORY.md:392-397` — "The *spelling* remains strawman. One change already known to be owed: `touches()` bodies below emit stringly-typed `kind:entity` lines — the kind-half is due to migrate to stage 7's annotation-typed emission"
- `KNOBS.md:44-45` — "every added member buys value, none removes prior value (rul24-threefunc-monotonic)"
- `Research/notes/24G-kind-owner-family-design-round.md:132` — "`touches()` migration to typed emission: LAST — after `reaches()` proves the shape; a conscious cleanup, not a blocker"

**Kill-attempt**: The team could argue that the migration is cosmetic (same information, different spelling) and the engine can accept both formats during a transition. But the fixtures already use the stringly-typed format, and the annotation-typed replacement changes the contract: the `| sed` dressing dies, the kind moves to a trailing annotation, and the output format changes. This is a genuine contract change. Low severity because it's planned and scoped; it survives as a finding because it illustrates the tension between "strawman syntax" labeling and the reality that fixtures already depend on the strawman.

---

### FINDING 10: The `kTYANNOT` knob's "directional, de-facto inline" status is a decision the team is deferring rather than making

**Statement**: KNOBS lists `kTYANNOT` as "directional — de-facto `kTYANNOT-inline`; the formal weld is human-reserved." The team has built the inline annotation surface, written 187 oracles against it, and documented the off-ramp cost — but refuses to weld the decision. This is L9 in microcosm: the spelling is already load-bearing.

**Severity**: Low. **Confidence**: High.

**Instantiates**: L9 (strawman permanence — the inline annotations are already in 187 fixture files), L2 (syntax headroom — the inline colon syntax consumes syntactic space that can't be reclaimed).

**Citations**:
- `KNOBS.md:62-63` — "directional — **de-facto `kTYANNOT-inline`; the formal weld is human-reserved**"
- `KNOBS.md:63` — "the inline dialect (period-named functions, inline binds, trailing marks) is stamped and implemented, with the off-ramp cost paid by the strip pass"
- All 187 `*.oracle.sh` files use inline `: Kind = value` annotations
- `Research/plans/17N-named-kinds-discipline-and-cooperation.md:42` — "17N itself treats 'breaks the off-ramp' as a **welded kill** (kill-8, HM) — so this spelling fails 17N's own bar"

**Kill-attempt**: The team might argue that "human-reserved" means the human is actively monitoring and can change course. But the 17N document itself notes that the inline spelling fails its own off-ramp bar. The fixtures are stamped. The strip pass is implemented. The cost of switching to eol-comment (or a third option) grows with every oracle written. This is a decision that has been made in practice but not in principle — the worst of both worlds: the cost of lock-in without the clarity of commitment.

---

### FINDING 11: The plan-render's attention-honesty commitment is a genuine language-design asset

**Statement**: The commitment that "attention is saved ONLY by provable elision" and that the plan render is "the whole book in original order; lines that will execute are never hidden" (`rul-attention-honesty`) is an unusual, principled decision that shapes the entire language surface. It directly addresses the trust problem that haunts automation tools.

**Severity**: N/A (positive finding). **Confidence**: High.

**Instantiates**: Lesson-independent — this is a positive instance of L18 (the render IS the language's user interface) and a deliberate choice against L3 (no silent defaults — nothing disappears without explicit proof).

**Citations**:
- `spike/CLAUDE.md:216-218` — `rul-attention-honesty (welded)`: "Attention is saved ONLY by provable elision. The plan render is the whole book in original order; lines that will execute are never hidden or folded (at most dimmed, warily). 'Scrappy, but correct: never hide risk from the user.'"
- `USER_STORY.md:8-10` — "the plan is the whole book, in original order, as plain sh; elided lines are present-but-commented-out; anything that will execute is never hidden; every surviving line carries its reason"
- `USER_STORY.md:126-128` — "without the illustrative `--verbose`, Dorc knows enough to *not even show those first lines to the user most of the time.* User-attention is conserved, safety is preserved"

**Why it's good**: Most orchestration tools obscure their decisions. Dorc's explicit commitment to show everything that executes, with attribution, is a trust-building mechanism that doubles as a teaching tool — the user sees *why* lines survived and *what* they can do to improve coverage. The language around this (elide/guard/run/descope/survive with `why`-lens attribution) is unusually coherent for an accreted design.

---

### FINDING 12: The function-name-as-contract principle is powerful but has no static defense against misnaming

**Statement**: The design explicitly states that "in this dialect the function name is nearly the entire contract surface" (24G §6). A function named `is_converged` whose body only checks partial state is a correctness bug the engine cannot detect. The name carries semantic weight the analyzer cannot verify.

**Severity**: Low. **Confidence**: Medium.

**Instantiates**: L12 (hidden dependencies — the name is a dependency between the author's intent and the engine's behavior, visible at neither), L10 (error handling — there's no mechanism for the engine to detect "this function claims to answer convergence but doesn't").

**Citations**:
- `Research/notes/24G-kind-owner-family-design-round.md:113-114` — "In this dialect the function name is nearly the entire contract surface — a period-function carries no signature; `is_converged`'s name already IS its license"
- `spike/CLAUDE.md:203-204` — "authoring the verdict-function IS the vouching act"
- `USER_STORY.md:258-259` — "the period-name is the opt-in semaphore ... the name is a *contract*"

**Kill-attempt**: The team could argue that this is no different from any naming convention in any language — if you name a function `is_sorted` but it doesn't check sorting, that's a bug the type system can't catch. True. But the difference is that in Dorc, the name carries *legal force* (it licenses elision), not just documentation value. A misnamed `is_converged` that returns 0 for the wrong reason causes under-execution. The engine has no mechanism to cross-check "does this body actually probe what its name claims?" The only defense is human review of oracle bodies. Low severity because this is an inherent limitation of the self-vouch model, not a design mistake per se — but worth flagging because the function-name-as-contract principle amplifies the blast radius of naming errors.

---

### FINDING 13: The `UNK` breadcrumb idiom is an out-of-band channel in an in-band-first language

**Statement**: The `printf 'UNK ...' >>"$DORC_REPORT"` idiom creates an ad-hoc diagnostic channel that contradicts the `kOOB` principle of in-band-first design. The report file is a side-channel, not sh observable flow. The tension between "everything is sh" and "we need OOB channels for diagnostics" is unresolved.

**Severity**: Low. **Confidence**: Medium.

**Instantiates**: L13 (consistency — the language's philosophy says in-band, but the practice uses out-of-band), L7 (typed/untyped boundary — the UNK channel is where the analyzer's limitations surface to the user, but through a mechanism invisible in the sh source).

**Citations**:
- `USER_STORY.md:357` — `printf 'UNK unmodeled foobar verb: %s\n' "$verb" >>"$DORC_REPORT"; return 2`
- `ORACLE_PROVIDES.md:228` — "Scoped refusal with attribution (live): the `UNK` out-of-band report idiom — 'I decline to answer here, and here is why' — feeding hints and plan-reasons. OOB is legal for facts, diagnostics, refusals; never for verdicts or licenses (rul-role-split)"
- `KNOBS.md:53-54` — "the sanctioned OOB *metadata* lanes (the `UNK`/refusal report; the probe-readback lanes) carry no configuration"
- `KNOBS.md:56` — the OOB redline is "user-configuration form, not metadata transport — out-of-band *metadata* is fine"

**Kill-attempt**: The team has already thought about this — the KNOBS `kOOB` entry explicitly carves out OOB metadata as acceptable, restricting the redline to OOB *configuration*. The `UNK` report is metadata/diagnostics, not configuration. So this is not a contradiction; it's a deliberate carve-out. But the carve-out's rationale ("the redline is configuration form") creates a distinction that users must learn: some out-of-band things are fine (diagnostics), others are forbidden (config). This is a subtlety that the "everything is sh" marketing doesn't prepare the user for. Survives at low severity because the tension is real even if the resolution is principled.

---

### FINDING 14: The language lacks a story for versioning, deprecation, and evolution of the oracle contract

**Statement**: No mechanism exists for an oracle author to declare "this oracle targets contract version N," for the engine to detect version skew, or for the community to evolve the contract without breaking existing oracles. The `ORACLE_PROVIDES` ledger explicitly notes that version/context-pins are a "reserved future entry."

**Severity**: High. **Confidence**: Medium.

**Instantiates**: L19 (backwards compatibility requires active design — the team acknowledges the gap but hasn't filled it), L1 (defaults become load-bearing — the current contract defaults will be what the first real users depend on).

**Citations**:
- `ORACLE_PROVIDES.md:232-234` — "Reserved future entries (each will need its own ledger row when unparked): ... *version/context-pins* (the MH2 seed: 'the binary you're eliding around is the binary I described' — the elision-site's mechanical check of a traveled claim's context, the missing tether under rul24-divergence-is-the-game)"
- `TODO.md:6` — "prior-art on linking oracles' binaries-to-be-invoked to *hash*, spelled-in-sh (`if [ "$(shasum thebin)" = "abcdef..." ]; then ...`)"
- No `version` or `deprecated` mechanism exists anywhere in the fixture corpus or design documents

**Kill-attempt**: The team could argue that the contract is still in spike-phase and versioning is premature. But the fixtures exist. The `USER_STORY` describes publication and sharing of oracles. The `ORACLE_PROVIDES` ledger is intended to become "the contract." Once an oracle is published and adopted, the contract it was written against is fossilized. Without a versioning mechanism, every contract change is a breaking change. Medium confidence because the team is aware of the gap and has reserved a slot for it — the finding is about the absence, not about a bad decision.

---

### FINDING 15: The two-audience design (admin vs engineer) is addressed in prose but not in the language surface

**Statement**: The design documents extensively discuss the two-audience problem, but the language itself provides no progressive-disclosure mechanism. An admin writing a book and an engineer writing an oracle use the same file format, same function conventions, and same annotation syntax — the only distinction is whether they *choose* to write period-named functions. There is no "admin mode" that restricts or simplifies the surface.

**Severity**: Low. **Confidence**: Medium.

**Instantiates**: L4 (dual-audience — the language conflates the admin and engineer surfaces), L14 (abstraction gradient — the cliff between "just scripting" and "library authoring" is steep but not visibly scaffolded).

**Citations**:
- `DESIGN.md:488-503` — extensive discussion of oracle vs runbook, two users, two flavours of code
- `USER_STORY.md:13-15` — "The cast: one person, wearing both hats"
- `USER_STORY.md:231-234` — "Annoyed, our admin puts on the engineer hat for exactly the length of a coffee. `foobar` already has a status query (most tools do). They append to the book's own file — oracles and runbooks can share a file"
- `KNOBS.md:189-195` — `kSILO` pseudo-knob discussing the gradient where correctness-code migrates from books to oracles

**Kill-attempt**: The team might argue that the *absence* of a mode distinction is the point — the whole value proposition is that the admin can gradually enrich their script without crossing a boundary. The USER_STORY's stage 3 shows the admin adding `foobar.is_converged()` to their own book file, and the tool just handles it. This is genuinely the right approach for the tool's philosophy. Low severity because the current design *intentionally* avoids a mode distinction; the finding is more about whether the lack of scaffolding will confuse users who don't understand the contract they're entering. The positive read is that this *is* progressive disclosure — you start with a book, then add oracle functions in the same file — which is exactly what L4 calls for.

---

## PART 3 — CLEARED

### C1. "The language has no error messages" — CHECKED AND WITHDRAWN

I initially suspected that the language's heavy reliance on the plan-render as the primary user interface meant that error reporting was under-designed. The corpus shows the opposite: `rul-attention-honesty`, the `why`-lens, the `UNK` breadcrumb channel, the hint/nag machinery, and the explicit `$DORC_REPORT` channel collectively form a rich diagnostic surface. The plan-render's per-line reason annotations are themselves error messages. This is not a gap; it's a deliberate design that treats diagnostics as a first-class product.

### C2. "The guard mechanism introduces a second language" — CHECKED AND WITHDRAWN

I initially worried that the `( check ) || command` guard shape constituted a second, machine-emitted language that users must also learn. On examination, the guard body is the *oracle author's own sh* shipped strip-only — not machine-synthesized code. `spike/CLAUDE.md:179-180` explicitly forbids engine-synthesized sh. The guard shape is the same `check || command` idiom a human writes by hand; the machine just inserts it. This is not a second language; it's the existing language composed by the engine.

### C3. "The period-naming convention will collide with existing shell function names" — CHECKED AND PARTIALLY WITHDRAWN

Periods in function names are not valid in POSIX sh, so `foo.is_converged()` cannot collide with any existing shell function. The mechanical rename to `foo_is_converged()` uses double underscores, which are also unlikely to collide. However, the *unstripped* form is not runnable by any shell — it's a Dorc-only surface. So collision is not the problem; the off-ramp breakage is (see FINDING 2).

### C4. "The `touches()` function emits stringly-typed coordinates" — ACKNOWLEDGED BY THE TEAM, NOT A FINDING

The USER_STORY stage 5 FIXME and ORACLE_PROVIDES both acknowledge that the `kind:entity` line format is stringly-typed and migrating to annotation-typed emission. The team is aware and has sequenced the fix. This is not a finding; it's a known TODO.

### C5. "The `predict()` function was renamed from `check()` mid-design" — NOT A LANGUAGE DEFECT

The mechanical rename from `check()` to `predict()` (`spike/CLAUDE.md:227-228`) is an example of the team managing naming evolution while the user-base is zero. This is exactly the right time to rename things. The rename itself is evidence of good naming hygiene, not a problem.

### C6. The `kOOB` redline vs `kTYANNOT` tension — THE TEAM HAS ACKNOWLEDGED IT HONESTLY

Both poles of `kTYANNOT` violate a design principle (inline breaks off-ramp; eol-comment breaks `kOOB`). The team's documentation of this as a genuinely horrible trade (17N's "A genuinely horrible trade") rather than papering over it is credit-worthy. The unresolved status is flagged in FINDING 2 and FINDING 10, but the *honesty* about the tension is a design positive, not a defect.

### C7. The guard-mechanism's composition with `set -e` — WELL-HANDLED

The `||` left-hand side is errexit-exempt by POSIX rules, so guards naturally compose with the book's `set -eu` without special handling. USER_STORY stage 2 explicitly notes this. The fixtures confirm it (`exec-query-guard-composition/book.sh` uses `set -e` with a `command -v ... || apt-get install` line). This is a case where sh's existing semantics align with the tool's needs.

### C8. The probe/apply phase separation — WELL-MOTIVATED AND WELL-SCOPED

The `kFAIL` weld (probe withholds, apply performs) is clear, the phase-keying is explicit, and the plan/apply consent gate gives the user visibility. The probe-phase contract (non-mutation) has a structural enforcement (self-vouch, rc-127 scan) rather than relying on inference. This is good language-level design for a best-effort tool.

### C9. The "no registry" decision for kind names — PRINCIPLED BUT WITH ACKNOWLEDGED COSTS

The rejection of a central kind registry is consistent with the tool's anti-bureaucracy philosophy. 17N §6 provides a coherent alternative (reverse-DNS handles, consumer-driven matching). The risk of name collision is real but the failure mode is degraded value (walls stay walls), not incorrectness. The design's explicit choice to accept this risk in exchange for zero-coordination-cost authoring is defensible. Flagged in FINDING 4, but the *decision* to reject a registry is not itself a mistake.

### C10. The `rul24-warnings-tune-high` ruling — CORRECT FOR THE SPIKE PHASE

The decision to emit many diagnostics during development (spike phase) despite conventional wisdom about warning fatigue is properly scoped and reasoned. The three grounds (mechanism-keepalive, LLM-feedback, warnings-are-the-product) are specific to this project's development methodology. This is a process decision, not a language design decision, and it's appropriately time-bounded to the spike era.

---

## SUMMARY

The Dorc language is an honest, principled accretion that gets several hard things right: the exit-status partition, the monotonicity guarantee, the attention-honesty commitment, the guard-as-authored-sh-not-synthesized-code principle, and the probe/apply phase separation. These are non-trivial achievements for a design that evolved as a side-effect of analyzer work.

The load-bearing concerns are FINDING 2 (off-ramp hostility contradicting a core DESIGN promise), FINDING 1 (two-surface mental model), FINDING 4 (unregulated kind namespace silently degrading cross-oracle value), and FINDING 14 (absence of versioning/evolution story). These are all in the "expensive to fix later, cheap to address now" category — the kind of decisions L1 and L19 warn about.

The issues that are genuine but currently containable are FINDING 3 (colon-mark overloading), FINDING 5 (answer/license conflation), and FINDING 6 (theatre flag). These are well-understood by the team and documented as open tensions; the risk is not that they're unknown but that the field-trial data needed to resolve them may not arrive before the user-base makes changing them expensive.
