> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, ADVERSARIAL stance (24Kb): verbatim extract from commit fd5fa82 on branch worktree-agent-abd7ff8be88067e1b. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Kb-01 — phase-1 skill-up: method + source ledger

External language-design review of the Dorc authored dialect (as of `259b27d`), commissioned as a fresh-eyes audit.
Reviewer: Fable-class agent, isolated worktree, 2026-07-05.

## Pre-registration discipline (read this first, adjudicator)

The brief's gate: mint numbered, falsifiable language-design lessons from *outside* research BEFORE opening any
corpus file, so lessons cannot be retro-fitted to nits. Compliance record:

- CONTEXT DISCLOSURE (unavoidable contamination, declared): the harness auto-injected at session start, before the
  brief was readable: the root `AGENTS.md` (terminology-firmings: oracle/book, elide/guard/replace, fail-fast
  rephrasing, "metadata is all spelled in sh", the two-users admin/engineer framing, prior-art-gotchas section), the
  user's global CLAUDE.md, and a memory index (mentions of e2e tiers, goldens, round history). I cannot unread those.
  No other corpus file has been opened pre-gate: not USER_STORY.md, not ORACLE_PROVIDES.md, not KNOBS.md, not
  spike/CLAUDE.md, not README/DESIGN/IMPLEMENTATION/TODO, no fixtures, no Research/plans or notes.
- Where a lesson below was plausibly *shaped* by the injected AGENTS.md content (e.g. I know the project says
  "spelled in sh" and has two user roles), I mark it `[shaped-by-injection?]` so the adjudicator can discount.
- Lessons live in `24Kb-02-lessons-preregistered.md`; committed before first corpus read. Any lesson added after
  corpus exposure will be explicitly marked POST-HOC there.

## Method

- Four fan-out source-gathering subagents (Opus, no-subagents clamp, web-only, verbatim-quote discipline), one per
  brief domain: (D1) language ergonomics/usability as a discipline; (D2) recorded regrets of language designers;
  (D3) gradual typing in practice; (D4) shell-adjacent languages and backwards compatibility.
- In parallel, main-context verification reads by me of the most load-bearing primary sources (the ones lessons will
  hang off), so key claims sit unfiltered in reviewer context, not only in subagent summaries.
- Everything load-bearing gets a source-slug in the ledger below; grades: A = designer's own words / first-party
  design doc / peer-reviewed study; B = official docs or community-canonical wiki-of-record; C = quality secondhand.

## Source ledger

Slugs are full-word per house convention. `(main-context)` = fetched and read by the reviewer directly;
`(gatherer)` = verified by a fan-out librarian subagent, quote checked against the fetched page by that agent.

### Verified main-context reads (cross-domain core)

- [typescript-design-goals] Microsoft, "TypeScript Design Goals", TypeScript wiki (raw md, microsoft/TypeScript-wiki),
  fetched 2026-07-05. Grade A (first-party design doc). Establishes: goals include "Impose no runtime overhead on
  emitted programs", "Emit clean, idiomatic, recognizable JavaScript code", "Preserve runtime behavior of all
  JavaScript code", "Use a consistent, fully erasable, structural type system"; non-goals include "Apply a sound or
  'provably correct' type system. Instead, strike a balance between correctness and productivity" and "Add or rely on
  run-time type information in programs, or emit different code based on the results of the type system", and
  "Introduce behaviour that is likely to surprise users." (main-context)
- [wooledge-errexit-faq105] Greg's Wiki, "BashFAQ/105 — Why doesn't set -e (or set -o errexit, or trap ERR) do what I
  expected?", mywiki.wooledge.org/BashFAQ/105. Grade B (community-canonical wiki of record for bash). Establishes:
  errexit's exception rules are "extremely convoluted, and they still fail to catch even some remarkably simple
  cases. Even worse, the rules *change* from one Bash version to another"; exit status conflates error-with-false;
  functions used in conditional position have errexit disabled *inside the function body* ("As soon as a function is
  used as a conditional ... set -e stops being applied within the function. This may not only cause code to
  unexpectedly start executing in the function but also change its return status!"); `local var=$(cmd)` masks the
  command's status behind `local`'s; only the rightmost pipeline element's status counts sans pipefail; the
  criminal-catching-robot allegory (heuristic detectors on an uncontrolled substrate get divergently patched
  per-deployment, forever). (main-context)
- [hyrums-law] Hyrum Wright, hyrumslaw.com. Grade A (author's own site). Verbatim: "With a sufficient number of users
  of an API, it does not matter what you promise in the contract: all observable behaviors of your system will be
  depended on by somebody." Also: "the implementation has become the interface", "bug-for-bug compatibility."
  (main-context)
- [feldman-make-tabs-email] Michael Stillwell, "Tabs and Makefile", beebo.org 2015-04-20 — publishes a 2015 email
  exchange with Stuart Feldman (Make's author). Grade A (Feldman's own words). Verbatim Feldman: "Within a few weeks
  of writing Make, I already had a dozen friends who were using it. So even though I knew that 'tab in column 1' was
  a bad idea, I didn't want to disrupt my user base. So instead I wrought havoc on tens of millions." Establishes:
  the compat event-horizon arrives at the first *dozen* users, and the author knew at the time. (main-context)
- [sorbet-static-sigils] Sorbet docs, "Enabling Static Checks", sorbet.org/docs/static. Grade B (official docs).
  Establishes the per-file `# typed:` sigil ladder (ignore/false/true/strict/strong), with default `# typed: false`
  for unmarked files, and the four granularities of opt-out: file sigil, method `sig`, argument `T.untyped`,
  call-site `T.unsafe`. "At `# typed: strict`, Sorbet no longer implicitly marks things as being dynamically typed."
  (main-context)
- [oils-error-handling] Andy Chu / Oils project, "YSH Fixes Shell's Error Handling (errexit)",
  oils.pub/release/latest/doc/error-handling.html. Grade A (designer's own design doc). Establishes: "Each process or
  builtin decides the meaning of its exit status independently" — the *Failure Paradigm* (0 ok, nonzero error) vs the
  *Boolean Paradigm* (0 true, 1 false, 2 error); the *Error or False Pitfall* (`if grep` conflates rc=2 error with
  rc=1 false — their fix is a `boolstatus` builtin that aborts on rc>1); the *Disabled errexit Quirk* ("when you use
  a shell function in a conditional context, errors are unexpectedly ignored... We can't fix this decades-old bug in
  shell"); YSH partitions its own codes (1 runtime, 2 syntax, 3 expression errors, "an arbitrary non-zero status
  that's not used by other shells"); and the design FAQ ruling: "I've learned the hard way that when there's a shell
  **semantics** change, there must be a **syntax** change. In general, you should be able to read code on its own,
  without context." (main-context)
- [git-porcelain-stability] git docs, git-status, "Porcelain Format Version 1" section. Grade B (official docs).
  Verbatim: "guaranteed not to change in a backwards-incompatible way between Git versions or based on user
  configuration. This makes it ideal for parsing by scripts." Porcelain v2 additionally defines "an extensible set of
  easy to parse optional headers" with "Parsers should ignore headers they don't recognize." (main-context)
- [posix-grep-exit-status] POSIX.1-2017, grep utility, EXIT STATUS. Grade A (spec text). Verbatim: "0: One or more
  lines were selected. 1: No lines were selected. >1: An error occurred." Note also `-q` warps it: "the exit status
  shall be zero if an input line is selected, even if an error was detected." (main-context)

- [bash-manual-exit-status] GNU Bash Reference Manual §3.7.5 "Exit Status". Grade B (official manual). Verbatim:
  "Exit statuses fall between 0 and 255, though, as explained below, the shell may use values above 125 specially";
  127 = command not found, 126 = found but not executable, fatal signal N = 128+N; "All builtins return an exit
  status of 2 to indicate incorrect usage, generally invalid options or missing arguments." (main-context)
- [shellcheck-directive-wiki] ShellCheck wiki, "Directive" page (koalaman/shellcheck). Grade B (official docs of the
  de-facto shell analyzer). Establishes: the entrenched escape-hatch idiom for shell analysis tooling is the COMMENT
  DIRECTIVE — `# shellcheck disable=SCnnnn` scoped to the next complete command, post-shebang placement for
  file-wide, `.shellcheckrc` for project-wide; `# shellcheck source=path` exists precisely because static analysis
  cannot resolve dynamic `source "$(...)"` — i.e., when inference hits opacity, the ecosystem norm is annotate-in-
  comment. Note the scoping wart: "There is no support for scoping a directive to the first structure of the script.
  In these cases, use a dummy command like `true`" — even escape hatches must bend around sh syntax. (main-context)
- [steele-growing-a-language] Guy L. Steele Jr., "Growing a Language", OOPSLA 1998 invited talk (paper: HOSC 12,
  1999; PDF langev.com/pdf/steele99growing.pdf). Grade A (author's text; growth sentence verified against primary PDF
  text; library passage cross-checked via transcription at swyx.io/notes-on-growing-a-language-by-guy-steele-5501).
  Verbatim: "I need to design a language that can grow. I need to plan ways in which it might grow—but I need, too,
  to leave some choices so that other persons can make those choices at a later time." Also the library test: "A true
  library does not change the rules of meaning for the language; it just adds new words. The key point is that the
  new words defined by a library should look just like the primitives of the language" — with the APL-vs-Lisp
  contrast (APL user vocabulary doesn't look primitive, so users couldn't help grow it). (main-context)

### D1 — ergonomics/usability (gatherer ledger, merged 2026-07-05)

- [green-petre-cogdims] Green & Petre, "Usability Analysis of Visual Programming Environments: a 'cognitive
  dimensions' framework", JVLC 1996. Grade A. Dimensions as diagnostic questions, verbatim: "Consistency: When some
  of the language has been learnt, how much of the rest can be inferred?"; "Error-proneness: Does the design of the
  notation induce 'careless mistakes'?"; "Hidden dependencies: Is every dependency overtly indicated in both
  directions?"; "Role-expressiveness: Can the reader see how each component of a program relates to the whole?";
  viscosity = "resistance to local change"; "Premature commitment: Do programmers have to make decisions before they
  have the information they need?" Explicitly a cheap checklist: "an afternoon of careful thought about a system is
  probably all that is needed."
- [stefik-siebert-syntax] Stefik & Siebert, ACM TOCE 13(4) 2013. Grade A. Verbatim abstract: "languages using a more
  traditional C-style syntax (both Perl and Java) did not afford accuracy rates significantly higher than a language
  with randomly generated keywords, but that languages which deviate (Quorum, Python, and Ruby) did." (Per-language
  numeric rates NOT extracted; do not quote figures.)
- [stefik-hanenberg-language-wars] Stefik & Hanenberg, Onward! 2014. Grade A. "for novices, a randomly generated
  language is about as easy to use as Perl or Java"; PL design decisions rest on near-zero human-factors evidence.
- [pane-myers-natural] Pane, Ratanamahatana & Myers, IJHCS 54 (2001). Grade A. Non-programmers' spontaneous
  solutions systematically diverge from designer defaults; English-likeness is NOT the fix (HyperTalk "violates the
  human-computer interaction principle of consistency").
- [myers-pane-ko-natprog] Myers, Pane & Ko, CACM 47(9) 2004 + CMU-CS-98-101. Grade A. Method: study how the audience
  naturally expresses the task BEFORE designing. Documented mismatches: aggregate/implicit iteration ("participants
  usually expressed iterations implicitly, by operating on sets of objects"); "the usual case was often expressed
  first with exceptions afterwards."
- [ellis-stylos-myers-factory] Ellis, Stylos & Myers, ICSE 2007. Grade A. "factories are detrimental to API
  usability... users require significantly more time (p = 0.005) to construct an object with a factory than with a
  constructor." A designer-beloved pattern, measured harm.
- [stylos-clarke-constructors] Stylos & Clarke, ICSE 2007 (+ PPIG 2006). Grade A. Required constructor parameters
  (the static-safety instinct) measurably LOST to create-set-call: "APIs that used the create-set-call pattern (not
  requiring any constructor parameters) were preferred and less problematic for all" — required params "forced
  programmers to instantiate each of the parameter objects before they felt they could explore." Microsoft persona
  studies + CDs flagged as the effective cheap method.
- [elm-compiler-errors] Czaplicki, "Compiler Errors for Humans", elm-lang.org 2015. Grade A. Diagnostics as
  first-class surface, nearly free ("required no significant changes to the type inference algorithm"); the
  error-message-catalog: "a collection of Elm programs that trigger error messages... a huge help in finding
  problems and patterns and evaluating improvements."
- [papert-resnick-low-floor] Resnick, "Designing for Wide Walls" 2016/2020. Grade A. Papert's low-floor/high-ceiling
  + Resnick's wide-walls, first-party phrasing.
- [blackwell-attention-investment] Blackwell, IEEE HCC 2002 + Blackwell & Green PPIG 1999. Grade A. Attempting an
  abstraction is an attention-economics decision (cost now, uncertain payoff, risk of waste) — explains why cliffs
  suppress the first step entirely. Clean hidden-dependency example: phone quick-dial codes ("not possible to tell
  which quick dial codes I have programmed without actually dialling them").
- [becker-error-survey] Becker, Denny, Pettit et al., ITiCSE-WGR 2019. Grade A. 50+ years of evidence that novices
  cannot use standard diagnostics ("barriers to progress").
- [php-naming-consistency] PHP RFC "Consistent Function Names" 2015 + php.internals. Grade B. Maintainers' own
  record: "argument order and return inconsistency is a much worse problem than name inconsistency. Unfortunately,
  it's also harder to fix without massive BC breaks." HONEST GAP: no controlled study isolates the COST of PHP's
  inconsistency; anchor consistency claims on [green-petre-cogdims] + API-usability line instead.

D1 anti-lessons: (a) enhanced/friendlier compiler-error studies have a MIXED replication record (Pettit et al.
SIGCSE 2017 "Results Inconclusive"; survey synthesis "weak positive effects") — robust claim is
diagnostics-as-designed-surface + corpus-testing, NOT "friendlier wording reliably helps"; (b) CDs never validated
as a measurement instrument (originators say so) — checklist, not metric; (c) "make it read like English" is a
repeatedly-failed inference from the naturalness findings.

### D2 — designer regrets (gatherer ledger, merged 2026-07-05)

- [eich-infernal-semicolon] Eich, "The infernal semicolon", brendaneich.com 2012. Grade A. Verbatim: "I wish I had
  made newlines *more* significant in JS back in those ten days in May, 1995"; "ASI is (formally speaking) a
  syntactic error correction procedure. If you start to code as if it were a universal significant-newline rule, you
  will get into trouble."
- [eich-tendays-idg] Eich via The New Stack transcript of 2018 IDG video. Grade C (spoken first-party, secondhand
  transcript). The `==` regret: "And I did it. And that's a big regret, because that breaks an important
  mathematical property"; plus the 2012 Computer aside: "once something is released into the wild, bugs or
  imperfections quickly become essential features and are nearly impossible to change."
- [guido-python-regrets] van Rossum, "Python Regrets" OSCON 2002 deck. Grade A. His own removal list: lambda
  "crippled... confusing"; reduce "nobody uses it, few understand it"; int/int; string exceptions.
- [guido-dict-ordered] van Rossum, python-dev 2017-12-15. Grade A. Full ruling: "Make it so. 'Dict keeps insertion
  order' is the ruling. Thanks!" Paired with [python37-whatsnew-dict] (Grade B): "the insertion-order preservation
  nature of dict objects has been declared to be an official part of the Python language spec" — the clean two-hop
  record of accident-becomes-spec (3.6 implementation detail explicitly not-to-be-relied-on → 3.7 guarantee).
- [lerdorf-strlen-buckets] Lerdorf, php-internals 2013-12-16 (mirrored at LWN 577494). Grade A. The naming-chaos
  origin, verbatim: "Back when PHP had less than 100 functions and the function hashing mechanism was strlen(). In
  order to get a nice hash distribution of function names across the various function name lengths names were
  picked specifically to make them fit into a specific length bucket. This was circa late 1994 when PHP was a tool
  just for my own personal use and I wasn't too worried about not being able to remember the few function names."
  An implementation convenience, invisible to users, became a permanent naming law.
- [matz-evrone-regrets] Matsumoto, Evrone interview 2019. Grade A. His stated triad: global variables ("more like an
  appendix"), threads ("we should have had a better concurrency abstraction"), needless mutability (in-place
  timezone change on Time).
- [feldman-taoup] Feldman quoted in Raymond, The Art of Unix Programming ch.15. Grade A/B. Independent second
  capture corroborating [feldman-make-tabs-email]: "then a few weeks later I had a user population of about a dozen,
  most of them friends, and I didn't want to screw up my embedded base. The rest, sadly, is history." ESR's frame:
  "one of the worst design botches in the history of Unix."
- [crockford-json-comments] Crockford, Google+ ~2012 (primary host dead; wording uncontested and identical across
  canonical reproductions — nlohmann/json README, JSON Wikipedia). Grade B. "I removed comments from JSON because I
  saw people were using them to hold parsing directives, a practice which would have destroyed interoperability."
  NOTE: this is the counter-pressure to the ShellCheck comment-directive precedent — comments-as-directives is
  simultaneously the entrenched escape-hatch idiom AND a documented interop hazard.
- [steele-growing-a-language] (firmed by gatherer, same slug as above): the nub verbatim: "A language design can no
  longer be a thing. It must be a pattern—a pattern for growth—a pattern for growing the pattern for defining the
  patterns that programmers can use for their real work and their main goal."
- [go1compat] go.dev/doc/go1compat. Grade B. "It is intended that programs written to the Go 1 specification will
  continue to compile and run correctly, unchanged, over the lifetime of that specification" — AND it pre-registers
  its escape hatches (security, unspecified behavior, spec errors, bugs...).
- [cox-go-boring] Cox, "Backward Compatibility, Go 1.21, and Go 2", Go blog 2023. Grade A. "We released Go 1 and its
  compatibility promise to remove the excitement, so that new releases of Go would be boring. Boring is good.
  Boring is stable." GODEBUG as the keep-old-behavior mechanism; Go 2 (breaking) shelved.
- [rust-editions-rfc2052] RFC 2052 (Turon & Matsakis 2017). Grade B. Opt-in per-crate incompatible evolution:
  "the feature is only available by explicitly opting in to the new edition. Existing code continues to compile,
  and crates can freely mix dependencies using different editions."
- [typescript-semver] Hegazy on microsoft/TypeScript#14116. Grade A/B. "TypeScript never claimed to follow semantic
  versioning... My recommendation is fix your version of typescript to `major.minor` instead of just major."
- [stroustrup-quotes-dne] stroustrup.com/quotes.html (his own authentication page). Grade A. "Within C++, there is a
  much smaller and cleaner language struggling to get out" — pinned to D&E p.207; C-compat as deliberate expensive
  tradeoff.
- [wall-apocalypse-2] Wall, "Apocalypse Two", perl.com 2001-05-03. Grade A. Sigil variance diagnosis: "that initial
  funny character was trying to do too much in both introducing the 'root' of the reference, as well as the context
  to apply to the final subscript"; ruling for invariant sigils; bonus regret (RFC 071): "I was unduly influenced by
  Ada syntax here, and it was a mistake... We'll try to make different mistakes this time."

D2 anti-lessons (folklore control): (a) "Matz regrets flip-flop" is folklore — the removal issue was Magnus Holm's,
flip-flop was deprecated in 2.6 then UN-deprecated in 2.7; Matz's stated regrets are globals/threads/mutability;
(b) Eich regretting `typeof null` / `with` specifically: no locatable first-party quote — community consensus only;
(c) the "Eich grabbed the mic, apologized for semicolons" story is unverified and may be cross-contaminated with
Feldman's (verified) ACM apology; (d) Stroustrup's smaller-language quote is routinely weaponized beyond his own
documented gloss; (e) Cox "compatibility is far more valuable than any possible break" — secondary attribution,
not verified verbatim.

### D3 — gradual typing in practice (gatherer ledger, merged 2026-07-05)

Gatherer flags: [WF] = via summarizer-model extraction, re-verify before external quoting; [snip] = search-snippet
verbatim, not independently re-fetched; [mirror] = read via faithful mirror.

- [typescript-nonsemver] TS maintainers (Hegazy, Cavanaugh), issue #14116, 2017. Grade A. Verbatim Cavanaugh: "If we
  followed semver rules exactly, literally every single release would be a major version bump. Any time we produced
  the wrong type or emitted the wrong code or failed to issue a correct error, that's a breaking change, and we fix
  dozens of bugs like that in every release." Checker upgrades are a breaking-change treadmill BY POLICY.
- [is-sound-gradual-typing-dead] Takikawa et al., POPL 2016. Grade A. "We find that Typed Racket's cost of soundness
  is not tolerable... then sound gradual typing is dead"; "Almost all partially typed configurations exhibit
  slowdowns of up to 105x"; suffixtree: fully-typed 0.7x but any workhorse module alone typed = 35x+. SCOPE: measures
  deep/guarded contract enforcement specifically; transient/shallow semantics later recovered much cost (see
  anti-lessons).
- [migratory-typing-ten-years] Tobin-Hochstadt et al., SNAPL 2017. Grade A. Principle 3 verbatim: "The 'unit of
  migration' must satisfy two opposing desires: (a) It must be small enough to encourage the incremental migration
  of untyped code... (b) It must also be large enough to keep values from crossing the language boundary too often
  because every crossing may trigger a run-time check."
- [sorbet-stripe] Zimmerman, stripe.dev 2022. Grade A. 15M lines / 150k files; adoptable BECAUSE best-effort
  untyped-by-default ("all methods behave as though their arguments were annotated with T.untyped" [WF]); ratchet
  outcome "85% of all non-test files opt into # typed: strict... over 95% # typed: true" after ~4 years [WF].
- [dropbox-mypy] Lehtosalo, dropbox.tech 2019. Grade A. "In essence, it provides *verified documentation*";
  Any-poisoning verbatim: "If you imported anything from a module outside the build, you'd get values with the Any
  type, which are not checked at all. This resulted in a major loss of typing precision"; PyAnnotate (runtime trace
  → annotations) "didn't see much adoption... generated types often required a lot of manual polish"; end-state
  ratchet: "We now require type annotations in new Python files."
- [monkeytype] Meyer, Instagram Eng 2017 [mirror]. Grade A. Runtime-trace-driven DRAFT annotations for human review,
  not an oracle.
- [dart-null-safety] Nystrom, dart.dev 2020. Grade A. Verbatim: "Null safety is the largest change we've made to
  Dart since we replaced the original unsound optional type system with a sound static type system in Dart 2.0";
  "Code should be safe by default... we give you a boat that doesn't sink." First-party repudiation of unsound
  optional typing.
- [flow-launch] Facebook Eng 2014. Grade A. Flow's founding assumption ("most JavaScript code is implicitly
  statically typed") vs TS's; predicted TS "reduced coverage" — but see anti-lessons: the coverage edge never showed
  up in bug-detection data and Flow lost the ecosystem (no first-party concession exists; inferred from ecosystem
  data — flagged).
- [pep-563] Langa, PEP 563 (Status: Superseded). Grade B. Annotation-SEMANTICS churn: stringified annotations
  scheduled default for 3.10, delayed twice (runtime consumers — pydantic/FastAPI/dataclasses — depended on live
  objects), finally "replaced with deferred evaluation of annotations, as proposed by PEP 649 and PEP 749."
  Multi-year ecosystem standoff over what an annotation IS.
- [success-typings] Lindahl & Sagonas, PPDP 2006. Grade A. "success typings... never disallow a use of a function
  that will not result in a type clash during runtime" [snip].
- [dialyzer-adoption] Erlang/OTP docs + Sagonas ICFP-workshop 2021 keynote abstract. Grade B/A. OTP docs verbatim:
  "Dialyzer bases its analysis on the concept of success typings, which allows for sound warnings (no false
  positives)." Keynote verbatim: "The optimistic, 'never-wrong for defect detection', approach to type inference
  that success typings advocate has been key in Dialyzer's successful adoption... other approaches to typing Erlang
  and Elixir code have not managed to gain similar levels of adoption." ("never cry wolf" is a gloss, not verbatim.)
- [gao-bird-barr] Gao, Bird & Barr, ICSE 2017. Grade A. "both Flow 0.30 and TypeScript 2.0 successfully detect 15%!"
  of public bugs; stated as a conservative floor.
- [airbnb-ts-migrate] Rudenko, Airbnb Eng 2020. Grade A. All-in migration seeded with `any`/@ts-ignore to ratchet
  later: "We'll need to add some any types and @ts-ignore comments so the project compiles without errors, but over
  time we can replace them with more descriptive types."
- [how-to-evaluate-blame] Lazarek, Greenman, Felleisen, Dimoulas, ICFP 2021. Grade A. "Practical implementations of
  gradual typing almost completely ignore the idea of blame assignment"; industrial systems erase and "rely on the
  built-in safety checks of the underlying language"; empirically "strategies with imprecise blame assignment are as
  helpful to a rationally acting programmer as strategies with provably correct blame."

D3 anti-lessons: (a) "sound gradual typing is dead" is scoped to deep/guarded enforcement circa 2016 —
transient/shallow semantics + later work recover much cost; treat 105x as enforcement-strategy artifact; (b)
erasure-vs-enforcement is a values split, and the blame apparatus may not earn its overhead (same research group,
both sides); (c) Dart and TS drew OPPOSITE institutional conclusions from the same unsoundness evidence — split
tracks whether you control the whole runtime (Dart) or erase into someone else's (TS); (d) Dialyzer: warning TRUST
beat warning completeness for adoption; (e) annotation-semantics churn (PEP 563/649) is an underrated multi-year
tax — decide up front whether annotations are inert text, live runtime objects, or enforcement; (f) re-verify
[WF]/[snip]-flagged figures before quoting externally.

### D4 — shell-adjacent + backwards compatibility (gatherer ledger, merged 2026-07-05)

- [oils-errexit-catalog] Chu, error-handling.md (living). Grade A. (Same doc as [oils-error-handling] above;
  gatherer adds:) "POSIX shell has fundamental problems with error handling. With set -e aka errexit, you're damned
  if you do and damned if you don't"; "This issue means that shell scripts fundamentally **lose errors**. The
  language is unreliable"; "GNU bash fixes some of the problems, but adds its own."
- [bashpitfalls-hazard-density] Wooledge, BashPitfalls. Grade B. Dozens of numbered flawed common idioms — substrate
  hazard density evidence ("This page is a compilation of common mistakes made by bash users. Each example is
  flawed in some way.")
- [posix-exit-status] POSIX.1-2017 §2.8.2 (gatherer-verified verbatim): "If a command is not found, the exit status
  shall be 127"; found-but-not-executable = 126; killed-by-signal "reported as greater than 128". Grade A. (Pairs
  with [bash-manual-exit-status] and [posix-grep-exit-status] above.)
- [sysexits-failed-taxonomy] Allman 1980 / OpenBSD man sysexits + BSD header. Grade A/B. The aspirational 64-78
  exit-code taxonomy that never displaced installed habits. OpenBSD man verdict verbatim: "A few programs exit with
  the following non-portable error codes. Do not use them." Header self-description: "attempts to categorize";
  BUGS: "The choice of an appropriate exit value is often ambiguous."
- [posix-echo-disaster] POSIX.1-2017 echo. Grade A. Verbatim: "It is not possible to use echo portably across all
  POSIX systems unless both -n (as the first argument) and escape sequences are omitted"; RATIONALE: "The two
  different historical versions of echo vary in fatally incompatible ways"; "New applications are encouraged to use
  printf." The canonical case of one spelling, divergent substrate meanings.
- [debian-dash-switch] Ubuntu wiki DashAsBinSh (~2006, rev 2017). Grade B. Official rationale ("The major reason to
  switch the default shell was efficiency"); Debian policy backing ("shell scripts specifying /bin/sh as interpreter
  must only use POSIX features"); migration tooling named: `dash -n`, `checkbashisms`; framing: "Programs should be
  written to the standard, and if they use extensions they should declare them."
- [monad-manifesto-objects] Snover, Monad Manifesto 2002. Grade A (verified via faithful reproduction + PDF
  extraction — flagged). "Monad replaces pipelines passing text with pipelines passing .Net objects... without the
  need to perform error-prone text parsing and object lookup." Note the compat concession: built on "Bourne Shell
  syntax and control structures facilitating the skill transfer of Unix Admins."
- [fish-design-orthogonality] fish design doc. Grade A. POSIX ranked THIRD among goals; "The law of orthogonality";
  "Configurability is the root of all evil... a place where the program is too stupid to figure out for itself what
  the user really wants." (Gatherer honesty: no literal "no surprises" law exists in the doc.)
- [nushell-posix-nongoal] Nushell contributor book. Grade A. "POSIX-compliance. Nu intentionally optimizes for a
  pleasant experience over matching how commandline programs work in a POSIX-compliant way... maintaining strict
  compatibility is a non-goal."
- [shellcheck-directives-versioning] ShellCheck wiki + README. Grade B. (Directive mechanics as in
  [shellcheck-directive-wiki] above.) README on interpretation-versioning, verbatim: "It's a good idea to manually
  install a specific ShellCheck version regardless. This avoids any surprise build breaks when a new version with
  new warnings is published." A linter's reading of your code is itself a versioned surface; pin it.
- [autoconf-generated-shell-precedent] GNU Autoconf manual 2.72, Introduction + Genesis. Grade A. "The configuration
  scripts produced by Autoconf are independent of Autoconf when they are run, so their users do not need to have
  Autoconf" — generated artifact deliberately self-contained; "The primary goal of Autoconf is making the *user's*
  life easier; making the *maintainer's* life easier is only a secondary goal."
- [clig-human-vs-machine] clig.dev 2020. Grade B. "Return zero exit code on success, non-zero on failure"; "Have
  machine-readable output where it does not impact usability"; "Changing output for humans is usually OK... if the
  output is considered an interface, then you can't iterate on it"; "Keep changes additive where you can... maybe
  you can add a new flag"; "Don't have a catch-all subcommand."
- [posix-2024-accretion] POSIX.1-2024 (Issue 8). Grade B. `set -o pipefail` standardized in 2024, decades after
  universal ksh/bash practice. (`local` status hedged — gatherer could not positively confirm Issue 8 inclusion;
  multiple pre-2024 sources say unspecified; divergent semantics across shells blocked it for years. ~SUSPECT, do
  not load-bear.) Net: adoption precedes standardization by decades; divergent semantics can block indefinitely.

D4 anti-lessons: (a) successful deliberate breaks exist (fish ranks POSIX third; Nushell non-goal; PowerShell
object pipelines then shipped onto Unix) — installed habits are NOT inviolable when payoff is large and visible;
(b) the dash switch is a successful FORCED break: real acknowledged breakage, bought boot-speed + portability, and
critically shipped migration lint (`checkbashisms`); (c) strict-mode gospel is disputed at the substrate's own
canonical wiki; (d) the human/machine split dissolves compatibility-forever into two regimes — the right question
is WHICH output is an API; (e) sysexits: publishing a cleaner convention does not create adoption — breaks succeed
on payoff + migration tooling, fail on mere cleanliness; (f) Oils' dual-mode wager (OSH compat + YSH break, opt-in
per-script) is the open "have both" question.

## Digest / cross-domain observations

Seven themes recur across all four domains (these drove the lesson set in 24Kb-02):

1. Observed behavior acquires tenure instantly and independently of intent: Feldman's dozen friends, Eich's
   "bugs or imperfections quickly become essential features", Hyrum's law, dict-ordering accident→spec. The
   compat clock starts at first user, not first promise.
2. For OPT-IN analysis tools, warning trust beats warning completeness (Dialyzer's stated adoption edge;
   ShellCheck's pin-your-version advice; TS's deliberate unsoundness) — and the escape hatch must be local
   (ShellCheck directive) yet is itself hazardous as a channel (Crockford removed JSON comments because they
   became parsing directives).
3. Semantics-change demands syntax-change (Chu, hard-won); its inverse — semantic puns on existing spellings —
   is the CD hidden-dependency failure. A dialect read by two readers (host + tool) doubles this hazard.
4. Growth must be user-extensible vocabulary that looks primitive (Steele; Lisp-vs-APL), or every capability
   bottlenecks on the designer.
5. The stable/unstable channel split (porcelain; clig; go1compat's pre-registered escape hatches; Rust editions;
   TS pin-minor advice) is cheap to declare BEFORE users arrive and near-impossible to retrofit after.
6. Up-front correctness ceremony measurably loses to permissive-then-ratchet (create-set-call; Sorbet/Stripe
   best-effort start; Airbnb any-seeding; Dropbox's end-state-only requirement) — the ratchet needs a visible
   per-unit marker.
7. The substrate itself is hostile to imposed meaning: exit statuses are context-warped (errexit quirks) and
   colonized (126/127/128+n; grep 0/1/2; sysexits' failure); one spelling can have "fatally incompatible"
   cross-implementation meanings (echo). Dialect law that ignores the substrate's existing law loses.
