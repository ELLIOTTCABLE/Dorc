> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, NEUTRAL stance (24Ka): verbatim extract from commit 19df800 on branch ai/24Ka-langreview. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Ka — Language review: Phase 1 skill-up (source ledger)

Reviewer: Fable-class agent, round 24Ka. Task: review the accreted sh-dialect *as a language*,
against pre-registered lessons minted from outside research BEFORE corpus exposure.

## Integrity disclosure (exposure before the gate)

Before writing the pre-registered lessons I have seen ONLY:
- the dispatch brief itself (names the surface families abstractly: period-named role-functions,
  inline binds, trailing annotation-marks, exit-status partition, refusal idioms; names the two
  audiences; names kLANG as welded);
- `AGENTS.md` (injected unavoidably into my system prompt by the harness): terminology-firmings
  (oracle/book/guard/replace/elide, fail-fast), "metadata is spelled in sh" design reminder, the
  two-users reminder;
- top-level file/directory NAMES only (no contents).

No other corpus content read pre-gate. Lessons are minted from outside sources + prior expert
knowledge; every load-bearing claim gets a ledger entry below.

## Method

Four research domains (per brief):
1. language ergonomics/usability as a discipline
2. recorded regrets of language designers
3. gradually-typed languages in practice
4. shell-adjacent languages and backwards-compatibility

Mechanics: four fan-out gatherer subagents (sanctioned exception), one per domain, collecting
primary-source digests with URLs and exact quotes; I then read the top raw sources myself in
main context before minting lessons. Ledger grading follows interactive-research convention:
[A]=primary/designer's own words, [B]=strong secondary (talk transcript, official doc),
[C]=community secondary (graded skeptically).

## Source ledger

Grades: [A]=primary/designer's own words · [B]=strong secondary · [C]=community.
Entries [main-N] were read directly by me in main context; [erg-/reg-/grad-/shell-N] arrive
via the four gatherers (their ledgers embedded below when they land, spot-verified where
load-bearing).

### Main-context reads

[main-1] (A) TypeScript Design Goals — github.com/microsoft/TypeScript/wiki/TypeScript-Design-Goals
  Goals: "3. Impose no runtime overhead on emitted programs. 4. Emit clean, idiomatic,
  recognizable JavaScript code. ... 7. Preserve runtime behavior of all JavaScript code.
  ... 9. Use a consistent, fully erasable, structural type system."
  Non-goals: "3. Apply a sound or 'provably correct' type system. Instead, strike a balance
  between correctness and productivity." "7. Introduce behaviour that is likely to surprise
  users."
  Digest: the canonical superset-discipline charter. Erasability + preserve-all-JS-behavior is
  what let TS ride the substrate instead of forking it; soundness explicitly traded away for
  adoption; "no expression-level syntax" (goal 8) kept the runtime surface pure-JS.

[main-2] (B, community-canonical) BashFAQ 105 (Wooledge wiki) — mywiki.wooledge.org/BashFAQ/105
  "the shell can't tell whether an external program encountered something that it considers an
  error"; "So the shell implementors made a bunch of special rules, like 'commands that are part
  of an if test are immune', and 'commands in a pipeline, other than the last one, are immune'.
  These rules are extremely convoluted, and they still fail to catch even some remarkably simple
  cases. Even worse, the rules *change* from one Bash version to another".
  Digest: the definitive record of what happens when a shell feature keys control-flow off
  exit-status heuristics: context-dependence (checked vs unchecked positions), per-version
  drift, and the criminal-catching-robot allegory (heuristics recalibrated per-site diverge
  into incompatible dialects). Any language partitioning exit statuses inherits exactly this
  minefield: an rc's meaning depends on the syntactic CONTEXT that consumes it.

[main-3] (A) Oils FAQ 2021, "Why Create a New Unix Shell?" (Andy Chu) —
  oilshell.org/blog/2021/01/why-a-new-shell.html
  "OSH ... compatible with both POSIX and bash. The goal is to run existing shell scripts."
  "The Oil language is a brand new, incompatible shell language." "the same interpreter runs
  both styles ... there's a gradual transition between the two." "It's important to be
  compatible with existing code." Shell as "a language that grows"; polyglot reality.
  Digest: the one project that lived the exact bargain under review — extend entrenched sh
  without breaking it — chose TWO named languages with one runtime and an explicit gradual
  ramp, rather than one dialect trying to be both. Compat-first motivated by the installed
  base (Kubernetes 48k lines of sh, etc).

[main-4] (A-quoting, B-vehicle) LYSE Dialyzer chapter, quoting Lindahl & Sagonas success-typings
  paper — learnyousomeerlang.com/dialyzer ; paper: user.it.uu.se/~kostis/Papers/succ_types.pdf
  "it should impose no code rewrites of any kind. ... Rewriting, often safety critical,
  applications consisting of hundreds of thousand lines of code just to satisfy a type
  inferencer is not an option which will enjoy much success." "the inferred typings should
  never be wrong." "only complain on type errors that would guarantee a crash." "Dialyzer
  begins each analysis optimistically ... because there is the *possibility* that the function
  call ... succeeds at some point, Dialyzer will keep silent."
  Digest: the trust-contract for best-effort analysis over an installed base: optimism by
  default, silence unless provably wrong, extract implicit information rather than demand
  annotations, annotations accepted as hints not gates. This bought Dialyzer near-universal
  acceptance in a community allergic to type systems.

[main-5] (A) ShellCheck directive wiki — shellcheck.net/wiki/Directive
  Comment-directives (`# shellcheck disable=SC2059`) scope to "the command that follows"
  (including compound commands); shebang-adjacent = whole file; rc-file for project-wide.
  Wart in their own docs: "There is no support for scoping a directive to the first structure
  of the script. In these cases, use a dummy command like `true`". They recommend a human
  "why" comment beside every directive.
  Digest: the de-facto prior art for analyzer-annotations in sh: comments, not syntax, so the
  substrate never sees them; scoping rules subtle enough to need a wiki page and a dummy-command
  workaround. Also NB (from general ShellCheck behavior): it emits explicit "can't follow"
  warnings (SC1090/91) when its analysis loses track — coverage-loss is surfaced, not silent.

[main-6] (A, empirical — run by me on 2026-07-05) Dotted function names vs real shells.
  `a.b() { echo hi; }; a.b`:
    dash 0.5.x (WSL Ubuntu /bin/sh):  "Syntax error: Bad function name", rc=2 (PARSE-time death)
    busybox ash (WSL):                "syntax error: bad function name", rc=2 (PARSE-time death)
    bash 5.x (git-bash):              accepts; ALSO accepts under `bash --posix`
  POSIX XCU 2.9.5 requires fname to be a NAME (XBD 3.235: alnum+underscore, no leading digit) —
  spec text not re-fetched (+SURE from prior knowledge; empirical results above are the
  load-bearing evidence anyway).
  Digest: period-named functions are a bash/ksh/zsh-ism. On the two most-deployed strict
  /bin/sh implementations (Debian dash, Alpine busybox) they are parse-FATAL for the whole
  file — not graceful degradation. Any "it's still just sh" claim by a dialect using them is
  really "it's still bash/ksh/zsh".

[main-7] (A) Swift API Design Guidelines — swift.org/documentation/api-design-guidelines/
  "Clarity at the point of use is your most important goal. Entities ... are declared only
  once but *used* repeatedly." "Clarity is more important than brevity. ... it is a non-goal
  to enable the smallest possible code with the fewest characters."
  Digest: the canonical read-side naming doctrine; evaluate a spelling by examining USE SITES,
  not declarations. Brevity earned via the type system, never via terseness of names.

[main-8] (A) Odersky, Scala 3 "Critique of the Status Quo" (contextual abstractions) —
  docs.scala-lang.org/scala3/reference/contextual/
  "implicits are at the same time Scala's most distinguished and most controversial feature ...
  harder to learn than necessary and ... harder to prevent abuses." Criticism 3: "The syntax of
  implicit definitions is too minimal. It consists of a single modifier, implicit, that can be
  attached to a large number of language constructs. A problem with this for newcomers is that
  it CONVEYS MECHANISM INSTEAD OF INTENT." Also: "conversions and values just look too similar
  for comfort"; implicit scope "can hide anywhere in a long list of imports" -> "inscrutable
  type errors that go away with the right import incantation."
  Digest: the best recorded case of a library-author power feature poisoning the app-author
  reading path, and of ONE terse marker overloaded across constructs being a design regret the
  designer himself paid a major-version redesign to fix (Scala 3 split it into given/using/
  Conversion by INTENT).

[main-9] (A, empirical 2026-07-05) Prefix-assignment scoping across shells.
  `f() { echo "in:$V"; }; V=x f; echo "after:${V-unset}"` -> bash, dash, busybox ash ALL print
  in:x / after:unset (temporal scoping, uniform). Corrects my prior belief that modern dash
  persists. POSIX XCU 2.9.1 still marks persistence-after-function-call unspecified; ksh93 is
  the known divergent implementation (~SUSPECT, not tested — no ksh on this box).
  Also: `local` inside functions works on dash AND busybox ash (not POSIX, universally safe in
  practice — the contrast-class to dotted fnames, which are parse-fatal).
  Digest: two flavors of "not POSIX": de-facto-universal (local) vs actually-fatal (dotted
  fnames). A dialect's substrate claim must be tested idiom-by-idiom, not vibes-by-vibes.

## DRAFT candidate lessons (pre-gatherer scratch; finalized post-gatherer, PRE-corpus)

Working list; each will be firmed, cited, and renumbered in the final pre-registration.
Corpus has NOT been opened. Candidates:

c1 substrate-precision: define WHICH sh; test every blessed idiom on the weakest claimed shell
   (dash/busybox parse-fatality class vs local-class) [main-6, main-9, main-1 g7]
c2 erasability/tool-absence: every construct needs defined, correct behavior when the tool is
   gone; the script must still be the script [main-1 g3/g9]
c3 analyzer-never-lies trust contract; silence must be safe; annotations hints-not-gates
   [main-4]
c4 one marker = one intent; terse modifiers overloaded across roles convey mechanism-not-intent
   [main-8]
c5 optimize read-site over write-site; declared once, read forever; greppability/searchability
   of spellings [main-7]
c6 exit-status meaning is context-dependent in sh (checked positions, pipelines, negation,
   set -e immunity zoo); any rc-partition must specify every consuming context [main-2]
c7 evolution mechanism BEFORE first users: dated compat scope + spelled version/epoch marker
   (Go1, editions, per-file sigils); otherwise day-one accidents freeze [gatherers pending]
c8 the recognized-idiom set IS the API (Hyrum for analyzers); enumerate and version it
c9 escape-hatches become permanent: make refusals/opt-outs visible, scoped, reasoned, lintable
   [main-5, Sorbet pending]
c10 inference carries the weight; annotation-overhead tolerance is per-file/per-function
    ratchets, not per-expression marks [main-4, gatherer-3 pending]
c11 two audiences = layered strictness in ONE runtime with explicit ladder, not one surface
    stretched over both (OSH/YSH, typed:false->strict) [main-3]
c12 defaults right-for-the-demo wrong-at-scale; re-ask every default at 200-file scale
c13 naming must form a predictable grammar (verb-noun, guessable N+1th name); check collisions
    with substrate namespace [main-7, PowerShell pending]
c14 unvalidated annotations rot; prefer metadata the tool can cross-check against behavior
    (stub-drift, DefinitelyTyped pending)
c15 machine-emitted sh in user artifacts: must be idiomatic, bounded, attributable, and
    regenerable-idempotent, or it becomes autoconf (pending gatherer-4)
c16 static parseability: blessed idioms must be recognizable without execution (no eval/alias/
    dynamic-name dependence) [Oils pending]
c17 diagnostics are the teaching channel; every refusal must name the idiom to write instead
    (Elm/Rust pending)
c18 don't break the ecosystem's tools: a spelling that breaks shellcheck/shfmt taxes every
    editor forever (bats @test precedent, pending verify)
c19 TIMTOWTDI-tax: one blessed spelling per intent; synonym spellings split the corpus into
    recognized/unrecognized dialects
c20 coverage-loss must be visible: "not analyzed" must be distinguishable from "analyzed fine"
    at the artifact surface (SC1090-style "can't follow") [main-5]
c21 strawman labels do not protect: the respell window closes at first external author unless
    a mechanical migration (formatter/codemod) exists (Make-tab pending quote)
c22 punctuation budget: sh is sigil-saturated; new marks should avoid chars with live sh
    semantics nearby; words over punctuation (fish pending)
c23 formatter/quoting robustness: constructs that change meaning under shfmt/re-quoting are
    built on sand

## Gatherer digest: domain 1 — ergonomics/usability (received, embedded verbatim-condensed)

[erg-1] (A) Green & Petre 1996, Cognitive Dimensions founding paper (JVLC 7(2)).
  "broad-brush evaluation technique"; "The dimensions are not guidelines... they are discussion
  tools"; "For different types of user activity it is possible to set up a preferred profile
  across the dimensions. Exploratory design will require one type of profile, tightly-specified
  safety-critical design will require a different profile."; "an afternoon of careful thought".
  ~14 dimensions incl. abstraction gradient, closeness of mapping, consistency, diffuseness,
  error-proneness, hard mental operations, hidden dependencies, premature commitment,
  progressive evaluation, role-expressiveness, secondary notation, viscosity, visibility.
[erg-2] (A) Stefik & Siebert 2013, TOCE 13(4): "languages using a more traditional C-style
  syntax (both Perl and Java) did not afford accuracy rates significantly higher than a language
  with randomly generated keywords [Randomo], but ... languages which deviate (Quorum, Python,
  Ruby) did." Placebo-control methodology for syntax.
[erg-3] (A) Ko/Myers/Aung 2004, Six Learning Barriers: design/selection/coordination/use/
  understanding/information; "invisible rules"; barriers = places the system invites a
  plausible-but-wrong assumption.
[erg-4] (A) Stylos & Myers 2008 (FSE): method placement; users explore from one "main" object
  along exposed references; off-path capability 2-11x slower; NAMED CONFLICT between
  learnability (put it where they look) and information-hiding (experts don't want the
  coupling). n=10 caveat.
[erg-5] (A) Becker et al. 2019 ITiCSE-WGR: 50+ yrs of error-message research; messages
  "inadequate and not understandable"(1967)..."useless"(1976); designed for experts.
[erg-6] (A) Hermans 2020 (ICER), Hedy: gradual SYNTAX (withhold brackets/colons/indentation,
  add level-by-level toward Python); mismatched brackets = single most common novice error
  (Altadmri & Brown, 37M compilations); "attention to detail that does not come naturally".
[erg-7] (A) Pane/Ratanamahatana/Myers 2001: non-programmers express solutions with events and
  aggregate ops, rarely loops; "then is interpreted as afterwards instead of in these
  conditions"; COBOL/HyperTalk warning: English-like surface does not fix mismatch.
[erg-8] (A) Myers/Pane/Ko 2004 CACM: natural-programming design loop: identify audience ->
  study their natural expression -> design -> evaluate; indicts unexamined C-construct copying.
[erg-9] (A) Coblenz et al. PLIERS (TOCHI 2020): user-centered design adapted to expert-audience
  languages: Wizard-of-Oz, back-porting, rapid prototyping + formal semantics constraints;
  "high iteration costs... significant learning time... high variance".
[erg-10] (A) Clarke (MSFT) 2005: CDs adapted to APIs; THREE PERSONAS (opportunistic, pragmatic,
  systematic) each with target dimension profile; "without such goals teams are able to talk
  about API usability knowledgeably, but... unable to determine what they need to do."
[erg-11] (A) Du/Schantong/Siegmund CSEET 2025: FALSE FRIENDS: "syntactically similar in the new
  programming language, but not semantically" -> systematic, PERSISTENT interference surviving
  instruction.
[erg-12] (A) Barik et al. ICSE 2017 (eye-tracking, n=56): devs DO read error messages; reading
  them is as hard as reading code; 13-25% of task time; difficulty predicts task failure.
[erg-13] (A) Pettit/Homer/Gee SIGCSE 2017: "enhancing compiler error messages shows no
  measurable benefit" — field contested/inconclusive.
[erg-14] (B) Stefik & Hanenberg 2014 Onward!: evidence-based-design manifesto.
[erg-15] (C) Wikipedia CDs gloss (definitions only).

Gatherer-1 candidate lessons L1-L10 and disagreements D1-D6 recorded; key carried claims:
familiarity-is-not-evidence (Randomo); false-friends persistence; two-audience = two explicit
conflicting dimension profiles (falsifier: name the dimension where they pull apart, else
you've collapsed the audiences); discoverability is structural; punctuation density =
first-order novice barrier; error-message payoff contested; naturalness = structure-match,
not English-likeness.

## Gatherer digest: domain 4 — shell-adjacent & backwards-compat (received, embedded condensed)

[shell-1] (A) Oils FAQ 2023 (Chu): "single shell interpreter that runs in two modes"; "We extend
  the bash shopt mechanism, which is similar to upgrade mechanisms in Python (from __future__)
  and Perl (use strict)."; "the options are the ONLY thing that distinguish OSH and YSH";
  adoption = "coordination problem"; cost conceded = implementer complexity.
[shell-2] (A) Oils 2021 FAQ: "limiting yourself to POSIX is not just inconvenient, it's also an
  ill-defined and virtually untestable concept"; "POSIX is a descriptive specification and not
  a normative one"; local essential-but-non-POSIX; "spec tests are an executable specification";
  "CoffeeScript was a success because it influenced subsequent versions of JavaScript."
[shell-3] (A) Oils 2019 parsing post: "Static Parsing. Every part of your program is recognized
  without reference to runtime data."; "almost all of bash can be statically parsed"; bash/zsh/
  mksh dynamically parse ${}; alias requires dynamic parsing; static parseability is ENGINEERED
  by defining-out constructs (companion: "Parsing Bash is Undecidable").
[shell-4] (A) Snover Monad Manifesto retrospective 2011: manifesto=vision-not-shipped-record;
  "It is only fair to point out where the Manifesto has not been implemented or implemented
  well." (Manifesto PDF itself unextractable; object-vs-text wording = paraphrase.)
[shell-5] (A/B) PowerShell approved-verbs doc: fixed verb list, import warnings on violation;
  discoverability via uniform Verb-Noun; cost = break from terse Unix names.
[shell-6] (A) fish design doc: "The law of orthogonality"; "Configurability is the root of all
  evil"; POSIX only "whenever possible without breaking the above goals".
[shell-7] (A/C) fish issue #4620 (&& || added in 3.0 after years of refusal): "one of the
  costliest incompatibilities. For one thing: ssh-copy-id... Gerrit; its 'cherry-pick' button
  puts a command like `git fetch && git checkout <sha1>` into your clipboard."; `if true; and
  false` = "schrödingbug". COPY-PASTE SURVIVAL as hard constraint, distinct from scripting
  compat.
[shell-8] (A) Chet Ramey, AOSA vol.1 bash chapter: "backwards compatibility means never having
  to say you're sorry. The world, however, isn't quite that simple."; "I would have introduced
  something like formal bash compatibility levels earlier." (BASH_COMPAT exists now.)
[shell-9] (B) BashFAQ/105 (redundant w/ [main-2], adds): `local var=$(fail)` — "the exit status
  of local masks it"; three named maintainers, three verdicts on the same page (don't use /
  use carefully / rely on explicit handling): no community consensus on errexit.
[shell-10] (A) ShellCheck directives (redundant w/ [main-5], adds): `# shellcheck shell=sh`
  dialect selector; enable=require-variable-braces optional-strictness; "idiomatic shell" now
  tracks ShellCheck (SC2086 quoting, SC2006 $()-over-backticks).
[shell-11] (A) Duff, rc paper 1990: "less idiosyncratic syntax"; lists as THE difference;
  "User demand has dictated that rc insert carets in certain places, to make the syntax look
  more like the Bourne shell." Substrate gravity bends even from-scratch redesigns.
[shell-12] (A) Haahr & Rakitzis, es paper 1993: "shells have proven to be resistant to
  innovation in programming languages"; surface syntax "unchanged from the Bourne shell";
  novelty budget spent on semantics. Neither rc nor es displaced sh.
[shell-13] (A) Nushell "Thinking in Nu": `>` is greater-than, not redirection (use `save`);
  "Think of Nushell as a Compiled Language"; deliberate break + dedicated onboarding chapter.
[shell-14] (A) Elvish unique-semantics: "Elvish has no concept of exit status. Instead, it has
  exceptions"; two-channel pipeline (bytes + values); cost = manual to-json/from-json at every
  external-command border.
[shell-15] (A) TS design goals (= [main-1]).
[shell-16] (A) CoffeeScript 2 announcement: "The golden rule of CoffeeScript is: 'It's just
  JavaScript.'"; ES6 absorbed its ideas -> "no need for the CoffeeScript compiler to duplicate
  this functionality" -> retreat to syntax shim.
[shell-17] (A) Autoconf manual §Portable Shell: (Allbery) "The GNU assumption that /bin/sh is
  the one and only shell leads to a permanent deadlock. Vendors don't want to break users'
  existing shell scripts"; target = Unix v7 ~1977 subset ("you should not use aliases... or
  even unset"). Machine-emitted sh against a frozen floor = debugging tarpit.
[shell-18] (A) Miller, Recursive Make Considered Harmful 1997: idiomatic substrate use can be
  the WRONG use ("counter to much accumulated folk wisdom"); make gotchas (fresh shell per
  recipe line, tabs) = hidden execution-model mismatch class.

Gatherer-4 lessons L1-L10 carried; sharpest: copy-paste survival as a distinct compat
constraint [shell-7]; erasability decides absorbed-vs-strangled [shell-15/16]; POSIX floor
untestable, test real shells [shell-2]; exit-status int is lossy and non-compositional
[shell-9/14]; versioned compat levels regretted-as-missing by bash's own maintainer [shell-8];
emitted-sh legibility [shell-17].

## Gatherer digest: domain 2 — designer regrets (received, embedded condensed)

[reg-1] (A) Hoare, QCon 2009: "I call it my billion-dollar mistake... the null reference";
  "I couldn't resist the temptation to put in a null reference, simply because it was so easy
  to implement." Knew the sound alternative (discriminated unions), rejected as "cumbersome".
[reg-2] (B->A) Eich (InfoWorld/dotJS): `==` coercion added because one early in-house tester
  wanted "2"==2 to work: "And I did it. And that's a big regret... It led to the addition of a
  second kind of equality operator when we standardized." "once something is released into the
  wild, bugs or imperfections quickly become essential features and are nearly impossible to
  change." typeof: "a mess that will be hard to reform sensibly." (this/hoisting/with regrets
  NOT verified to Eich's own words — gatherer flagged.)
[reg-3] (A/B) Guido, CHM oral history on the py3 break: "Well, why was it necessary? I don't
  know that it was necessary." Regret = transition strategy (big-bang, nothing-of-value in
  3.0 to justify port), not the features. 2002 "Python Regrets" deck exists but slides not
  extractable verbatim (flagged).
[reg-4] (A) Russ Cox, go.dev/blog/compat 2023: "We released Go 1 and its compatibility promise
  to remove the excitement... Boring is good." Compat is MACHINED: automated api/*.txt diff
  gate + testing next-Go against all Google code. Breakage taxonomy: semantic changes (sort
  order, time precision, ParseInt underscores) break more than API removals. GODEBUG/go.mod
  lets new defaults ship while old programs keep old behavior.
[reg-5] (A) Go error-syntax post-mortem 2025 (check/handle, try, ?): "we likely diminished our
  chances for a better outcome by presenting an almost fully baked proposal with little space
  for community feedback and a 'threatening' implementation timeline." Community veto; team
  concluded "We think not" (stop trying).
[reg-6] (A via reproduction; primary robots-blocked) Graydon Hoare 2023: "the Rust We Got is
  many, many miles away from The Rust I Wanted"; "The Rust I Wanted probably had no future";
  he would have traded performance/expressivity for simplicity and concedes the community was
  right to overrule him.
[reg-7] (A) Steele, Growing a Language, OOPSLA 1998: "A language design can no longer be a
  thing. It must be a pattern... for growth"; "a main goal in designing a language should be
  to plan for growth." Ship small core + user-extension mechanism.
[reg-8] (A/B) Stroustrup: "Within C++, there is a much smaller and cleaner language struggling
  to get out" — HIS gloss: semantics cleaner than the C-inherited SYNTAX that carries them.
[reg-9] (A, personal email) Feldman on make's tab (1976, recounted 2015): "even though I knew
  that 'tab in column 1' was a bad idea, I didn't want to disrupt my user base. So instead I
  wrought havoc on tens of millions." "Within a few weeks of writing Make, I already had a
  dozen friends who were using it." Ritchie parallel: "more than three sites running C."
[reg-10] (A) Hejlsberg, artima 2003, checked exceptions: "It is a breaking change for me to add
  D to the throws clause... After you publish an interface, it is for all practical purposes
  immutable."; at scale "people completely circumvent the feature... 'throws Exception'
  everywhere". Versionability + users-defeat-the-feature as the two kill criteria.
[reg-11] (B) Java serialization: Reinhold "[Serialization] was a horrible mistake in 1997"
  (NB: Reinhold, not Goetz); Goetz: "the gift that keeps on giving."
[reg-12] (A/B) History of Haskell 2007: "The trouble with runaway success... is that you get
  too many users, and the language becomes bogged down in standards, user groups, and legacy
  issues." "avoid (success at all costs)" as evolution-freedom strategy.
[reg-13] (A) Larry Wall, Present Continuous ~2006: "The fact that we put sigils in front of the
  variables meant that the namespaces were protected from new keywords... so we could evolve
  the language fairly rapidly." "we changed all the sigils to be more consistent" (Raku
  invariant sigils = fixing Perl 5 "false minima"). "Dialects will diverge, so you should
  plan for it."
[reg-14] (A) Lerdorf: "there was never any intent to write a programming language... I just
  kept adding the next logical step on the way."; function-name inconsistency traced to
  strlen()-as-hash dispatch. Accretion-without-design as self-described genesis of the warts.
[reg-15] (B) Swift 3 era (Kremenek): last day of source-breaking changes; then per-module
  language-version flag (like -std=c99) reconciling "diametrically opposing goals".
[reg-16] (A/B) Rust RFC 2052 editions: "Since editions are opt-in, existing crates won't use
  the changes unless they explicitly migrate"; new keywords quarantined behind edition
  boundary; all editions interoperate in one toolchain.

Gatherer-2 lessons: freeze-point is a dozen users not a million; implementer-convenience
defaults produce the expensive regrets; feature-for-the-first-user punishes every later
reader; compat must be machined not promised; opt-in epochs beat big-bang breaks; semantic
changes break more than API removals; community veto is real and sometimes right; design the
growth pattern; proposal PROCESS matters; small-userbase = evolution freedom.

## Gatherer digest: domain 3 — gradual typing in practice (received, embedded condensed)

[grad-1] (A) TS design goals (= main-1); goal 11 no-substantial-breaks-from-1.0.
[grad-2] (A) Takikawa et al. POPL 2016: "given the state of current implementation
  technologies, sound gradual typing is dead" — 2^n config lattice; boundary contract checks
  produce order-of-magnitude valleys; the SOUNDNESS mechanism is the perf killer.
[grad-3] (A) PEP 484: "no type checking happens at runtime"; checker = "a very powerful
  linter"; Any "consistent with all types"; "no new syntax needs to be added".
[grad-4] (A) Lehtosalo/Dropbox 2019 (4M lines): Any-erosion — "If you imported anything from a
  module outside the build, you'd get values with the Any type... a major loss of typing
  precision."; typed ISLANDS later merged and "the types weren't compatible between the two
  islands"; ratchet "We now require type annotations in new Python files"; binding constraint
  was CHECKER LATENCY (daemon, mypyc), not runtime overhead.
[grad-5] (A) Dart 1 optional/unsound/erased -> FAILED (types untrustable); Dart 2 = BREAKING
  rebuild to sound static+runtime checks; payoff: field access 26 native instructions -> 3.
[grad-6] (A) PEP 563->649->749 saga: the annotation-EVALUATION mechanism churned for six
  years, broke Pydantic/FastAPI, never became default — the annotation feature litigated
  itself.
[grad-7] (A) Bracha, Pluggable Type Systems 2004: "Run-time semantics are independent of type
  system; Type annotations are optional." Mandatory typing danger: "people rely on it for:
  Optimization, Security Guarantees. If type system fails, behavior is completely undefined."
  Cost of optionality: must forbid constructs whose runtime meaning depends on types.
[grad-8] (A) Sorbet sigils: "# typed: ignore/false/true/strict/strong" per-file comment dial;
  typed:false pays off pre-annotation; strong (ban T.untyped) is rare/beta — fully-typed is
  the exception, not the destination.
[grad-9] (A) Sorbet runtime: sigs enforced at runtime by default; "Opting out of runtime
  checks can significantly degrade the trustworthiness of type signatures."
[grad-10] (A/B) Sagonas, 15-years-Dialyzer (ICFP 2021 Erlang WS): "The optimistic, 'never-wrong
  for defect detection', approach... has been key in Dialyzer's successful adoption"; "a defect
  detection tool which never 'cries wolf'".
[grad-11] (A) Hack@FB 2014: gradual with RUNTIME enforcement of param/return types ("safety
  beyond what can be checked statically while type annotations are being gradually added");
  JIT trusts annotations; <200ms incremental checker; adoption codemod-driven.
[grad-12] (A) Muehlboeck & Tate OOPSLA 2017: sound gradual "nominally alive and well" —
  co-design nominal types with the runtime and boundary checks are cheap; retrofit+structural
  was the killer, not soundness per se.
[grad-13] (B) Ben Kuhn 2019: "Static types speed up large projects, but slow down small
  projects"; sweet spot = "require annotations for function signatures, but (almost) nothing
  else"; escape hatch as FEATURE ("the ability to lie about types" for mocks/ORMs).
[grad-14] (A/B) MonkeyType (Instagram): bootstrap annotations from runtime traces; Dropbox
  PyAnnotate "didn't see much adoption" — trace-bootstrap helps but needs manual polish.
[grad-15] (A) Rack/Staicu ASE 2024 "Typed and Confused" (30k repos): erased gradual typing ->
  explicit runtime checks on <3% of function parameters; >70% of projects check no primitive
  params; 33 real exploitable type-confusion cases. Erasure + no user checks = false sense of
  safety, measured.
[grad-16] (A) TS handbook: "Using any disables all further type checking"; any is viral by
  construction (every op on any yields any); noImplicitAny = the ratchet.
[grad-17] (B) TS no-semver: "Nuances of TypeScript's type checking change in virtually every
  release"; team: "TypeScript never claimed to follow semantic versioning"; community
  counter-spec semver-ts.org "no new red squiggles". VERDICT CHURN is its own compat surface.
[grad-18] (C) TS-vs-Flow: TS won on LSP/VSCode/DefinitelyTyped/framework-agnostic reach, not
  on soundness; "All JS code was valid TS."
[grad-19] (A) Siek et al. SNAPL 2015 refined criteria: "removing type annotations always
  produces a program that is still well typed"; academic gradual typing INCLUDES runtime casts
  — the gradual guarantee is about the runtime, and deployed erased systems lack the casts it
  assumes.
[grad-20] (A) TC39 types-as-comments: "a JavaScript engine ignores them, treating the types as
  comments"; stalled Stage 1 since 2022; runtimes shipped type-STRIPPING instead.
[grad-21] (A) Tobin-Hochstadt & Felleisen 2006-08: migratory typing origin; module-by-module
  with contract-protected boundaries — the contracts [grad-2] later measured as ruinous.

Gatherer-3 lessons: mass adoption required renouncing soundness (erasure won); industry
"gradual" is Bracha-OPTIONAL typing, not Siek-Taha gradual (category error with safety
consequences [grad-15]); escape hatch is load-bearing and permanent; runtime enforcement is a
distinct axis; checker LATENCY not runtime overhead was the binding perf constraint;
signature-only annotation is the ergonomic sweet spot; coverage grows by codemods+ratchets
not heroics; stub repos drift; the annotation syntax itself churns; tooling network-effects
decide winners.
