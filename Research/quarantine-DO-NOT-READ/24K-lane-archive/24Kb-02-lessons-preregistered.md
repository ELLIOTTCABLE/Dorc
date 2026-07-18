> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, ADVERSARIAL stance (24Kb): verbatim extract from commit fd5fa82 on branch worktree-agent-abd7ff8be88067e1b. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Kb-02 — pre-registered lessons (THE GATE)

Minted BEFORE any corpus file was opened (see 24Kb-01 for the contamination disclosure: root AGENTS.md +
memory-index were harness-injected pre-brief; nothing else has been read). Lessons marked `[shaped-by-injection?]`
are ones whose *selection* was plausibly influenced by knowing, from the injected AGENTS.md, that the project
(a) spells metadata in sh idioms, (b) has two user roles, (c) hangs meaning on an analyzer reading scripts. The
lesson *content* in every case is sourced from outside research, not from the corpus.

Firming record (verifiable via git history): v1 committed at 2099cb3 with citation-slots, before any gatherer
returned; this revision fills the slots from gatherer-verified sources and adds lesson-23 (whose provenance is
noted inline: minted after gatherer D1's return, still strictly pre-corpus). Lesson statements are otherwise
unchanged in substance; lesson-03 gained an honesty moderation from a D1 anti-lesson.

Format per lesson: statement → source anchors (see 24Kb-01 ledger) → DETECT: the falsifiable check a reviewer
applies to a candidate language. A finding will cite a lesson number; a lesson none of whose DETECT checks could
in principle fail is malformed and should be discounted by the adjudicator.

Any lesson added after corpus exposure will be appended under "POST-HOC" at the bottom, plainly marked.

---

## A. Ergonomics / evaluation discipline

lesson-01 — Naming and argument-order consistency is a measurable usability variable, not taste; a surface that
mixes conventions (per-word choices of prefix/order/voice) taxes every reader forever, and the tax is unfixable
after adoption ([php-naming-consistency], maintainers' own record: "argument order and return inconsistency is a
much worse problem than name inconsistency. Unfortunately, it's also harder to fix without massive BC breaks";
[lerdorf-strlen-buckets], the designer's own origin story — names "picked specifically to make them fit into a
specific length bucket" for a strlen() hash while PHP "was a tool just for my own personal use" — a private
implementation convenience became permanent user-facing law; honest gap — no controlled study isolates the cost,
so anchor on [green-petre-cogdims]'s consistency dimension: "When some of the language has been learnt, how much
of the rest can be inferred?").
DETECT: collect every blessed name/spelling in the dialect; write down the implied naming rule(s); count
exceptions. If you cannot write the rule down, that is the finding.

lesson-02 — Designer intuition about what target users find natural is reliably wrong when unchecked; the only
cheap correctives are (a) walkthroughs against the target population's *existing* idioms and (b) corpus
frequency checks. ([stefik-siebert-syntax] verbatim: "languages using a more traditional C-style syntax (both
Perl and Java) did not afford accuracy rates significantly higher than a language with randomly generated
keywords"; [myers-pane-ko-natprog]: the audience's spontaneous expressions diverge from designer defaults —
aggregate iteration, usual-case-first; [stefik-hanenberg-language-wars]: the field ships surfaces with no
human-factors evidence as a matter of course.)
DETECT: for each novel spelling, ask: does the target population already write this, or something visually near
it, with a DIFFERENT meaning? Is there any evidence anyone checked?

lesson-03 — Diagnostics are part of the language surface and must be designed with the syntax, not after it
([elm-compiler-errors]: near-free once treated as a goal, plus the error-message-catalog corpus method;
[becker-error-survey]: 50+ years of novices unable to use default diagnostics). For an inference-driven tool the
diagnostic question is sharper: when inference fails or infers something the author didn't intend, what exactly
does the user see, and which of the two user classes is blamed? MODERATION (D1 anti-lesson, honesty): the
replicated claim is that diagnostics need designing and corpus-testing, NOT that friendlier wording reliably
helps (enhanced-message studies are mixed); and [how-to-evaluate-blame] finds precise blame often no better than
imprecise for a rational programmer — so the bar is an EXISTING, actionable diagnostic story per construct, not
a perfect one.
DETECT: for each construct, find the specified failure-mode message/blame target. Absence of any designed
diagnostic story for a construct that can misfire = finding.

lesson-04 — If a surface serves two user classes, the transition between them must be a slope, not a cliff
([papert-resnick-low-floor]: low floor / high ceiling (/ wide walls); [blackwell-attention-investment]: users
decide whether to attempt an abstraction economically — cost now, uncertain payoff, risk of waste — so a cliff
suppresses the first step entirely). The classic failure: the simple mode is genuinely simple, the powerful mode
is genuinely powerful, and the first step from one to the other requires learning most of the powerful mode at
once.
DETECT: script the smallest task that outgrows the simple mode; measure what the user must newly understand to
take that one step. [shaped-by-injection? — I know the project has an admin/engineer split.]

lesson-05 — Semantic puns are error-prone: when an existing spelling is overloaded with additional tool-read
meaning, the text hides a dependency the reader cannot see ([green-petre-cogdims]: "Hidden dependencies: Is
every dependency overtly indicated in both directions?", error-proneness; Oils design FAQ, first-party: "when
there's a shell **semantics** change, there must be a **syntax** change. In general, you should be able to read
code on its own, without context" [oils-error-handling]). A dialect whose constructs are ordinary-looking
host-language code has TWO readers (the host executes it; the tool infers from it); every spelling where the two
readings can diverge is a standing false-inference generator — in BOTH directions (tool reads meaning the author
didn't intend; author intends meaning the tool doesn't read).
DETECT: for each idiom the tool treats as meaningful: (a) what happens when a user writes that idiom for its
plain host meaning, unaware of the tool meaning? (b) is there a plain-host way to write the same runtime
behavior WITHOUT triggering the tool meaning? If (a) = silent semantic capture and (b) = no, that is the rake.
[shaped-by-injection? — I know metadata is 'spelled in sh'.]

## B. Compatibility / evolution

lesson-06 — The compat event-horizon arrives at the first dozen users, not at 1.0; whatever is in the surface at
first-external-user is the surface forever, and designers know it at the time and ship anyway
([feldman-make-tabs-email], verbatim: "I already had a dozen friends who were using it... I didn't want to
disrupt my user base. So instead I wrought havoc on tens of millions"; corroborated independently in
[feldman-taoup]: "I didn't want to screw up my embedded base. The rest, sadly, is history"). Refinement (D4
anti-lessons): later breaks CAN succeed, but only with a large visible payoff plus shipped migration tooling
([debian-dash-switch]: `checkbashisms`, `dash -n`); breaks that are merely cleaner fail
([sysexits-failed-taxonomy]: "Do not use them"). The deliberate anti-regret machinery is cheap to declare
pre-1.0 and pre-registers its own escape hatches ([go1compat]; [cox-go-boring]: "Boring is good. Boring is
stable."; [rust-editions-rfc2052]: per-crate opt-in editions; [typescript-semver]: pin to major.minor).
DETECT: the project is pre-first-user. Enumerate what is currently marked experimental vs settled; check whether
ANY explicit pre-1.0 breakage right is reserved in writing. Absence of a written breakage policy at the moment
of first-user onboarding = the Feldman moment, live.

lesson-07 — Hyrum's Law applies to every observable channel, not just the documented API: output text, exit
codes, generated artifacts, error messages, and *analysis verdicts* are all de-facto interface once observed
([hyrums-law] verbatim: "it does not matter what you promise in the contract: all observable behaviors of your
system will be depended on by somebody"). The known mitigation is an explicit stable/unstable channel split,
declared from day one ([git-porcelain-stability]: porcelain v1 "guaranteed not to change in a
backwards-incompatible way"; v2 headers with "Parsers should ignore headers they don't recognize").
DETECT: inventory every observable output of the tool; for each, find the stability declaration. Machine-parseable
output with no declared stability = freeze-by-scraper waiting to happen.

lesson-08 — A language must plan its growth mechanism, and the Steele test for libraries is exact: "A true
library does not change the rules of meaning for the language; it just adds new words. The key point is that the
new words defined by a library should look just like the primitives of the language" [steele-growing-a-language].
The anti-pattern (APL): user vocabulary is visibly second-class, so only the designer can grow the language —
every new capability needs a tool release.
DETECT: can a third-party library author mint a new contract-carrying word without a tool change? Do
library-defined words look/behave like blessed primitives? If the blessed set is closed and hardcoded, growth =
designer-bottleneck.

lesson-09 — Defaults are the real semantics and become unremovable; the recurring regret-shape is a
convenient-when-small default that is wrong-at-scale ([eich-infernal-semicolon]: "I wish I had made newlines
*more* significant in JS back in those ten days"; sh's errexit-off; and the passive version — unspecified
observable behavior hardens into spec whether you choose it or not: [guido-dict-ordered]/[python37-whatsnew-dict],
the 3.6 implementation detail that 3.7 had to bless). Corollary: what the dialect does when the author writes
NOTHING is its most permanent design decision.
DETECT: enumerate the dialect's defaults (unannotated/unmarked/absent-case behavior); for each ask "if this is
wrong for a 10x-larger user, can it ever be flipped?"

lesson-10 — A tool that assigns meaning to programs has a second compat surface: the meaning-assignment itself.
Inference improvements change behavior for existing programs; mature tools treat this explicitly and still can't
make it painless ([shellcheck-directives-versioning], README verbatim: "It's a good idea to manually install a
specific ShellCheck version regardless. This avoids any surprise build breaks when a new version with new
warnings is published"; [typescript-nonsemver], maintainer verbatim: "If we followed semver rules exactly,
literally every single release would be a major version bump. Any time we produced the wrong type or emitted the
wrong code or failed to issue a correct error, that's a breaking change").
DETECT: find the policy for "the analyzer got smarter and now says something different about your unchanged
script". No policy + no verdict-pinning mechanism = every upgrade is a potential silent behavior change.

## C. Gradual typing bargain

lesson-11 — Gradual typing won where annotations were erasable and behavior-preserving
([typescript-design-goals], verbatim goals: "Impose no runtime overhead", "Preserve runtime behavior of all
JavaScript code", "consistent, fully erasable, structural type system"; non-goal: "emit different code based on
the results of the type system") and was repudiated where annotations carried unsound runtime meaning
([dart-null-safety], first-party verbatim: "since we replaced the original unsound optional type system with a
sound static type system in Dart 2.0"). The design axis has three poles (contracts/blame → runtime-readable-but-
inert → fully erased; [how-to-evaluate-blame], [pep-563], [typescript-design-goals]) and a design must know
where it stands. Where the 'annotations' are executable host code, erasability is forfeit by construction — so
the burden inverts: the design must prove the tool-reading and the runtime effect coincide, and must show that
de-adoption (running the same scripts without the tool, or under a different sh) is genuinely clean.
[shaped-by-injection?]
DETECT: take each annotation-idiom; run the thought-experiment "tool deleted tomorrow": does the script still do
the right thing under plain sh? Does it do the SAME thing? Where not, de-adoption is a lie and lock-in is real.

lesson-12 — Untyped gravity: unannotated regions dominate a gradual ecosystem unless there is a visible,
per-unit strictness marker with a ratchet culture (Sorbet's five-level `# typed:` sigil ladder with default
`false` [sorbet-static-sigils]; the ratchet works: [sorbet-stripe] ~85% of non-test files at `strict`, 95%+ at
`true` after ~4 years [WF-flagged figures]; [dropbox-mypy] Any-poisoning verbatim: "you'd get values with the
Any type, which are not checked at all. This resulted in a major loss of typing precision", ending in "We now
require type annotations in new Python files"; [airbnb-ts-migrate]: seed `any` deliberately, ratchet later).
Invisible strictness = no ratchet = permanent bottom.
DETECT: is there a per-script/per-library strictness level? Is it visible in the file? Can a team ratchet it and
CI it? What fraction of the seed corpus sits at which level?

lesson-13 — The typed/untyped boundary is where the pain concentrates: cost, blame-confusion, and soundness
holes all live at crossings ([is-sound-gradual-typing-dead] verbatim: "Almost all partially typed configurations
exhibit slowdowns of up to 105x" — scope-noted to deep/guarded enforcement; [migratory-typing-ten-years]
Principle 3 verbatim: the unit of migration "must be small enough to encourage the incremental migration... (b)
It must also be large enough to keep values from crossing the language boundary too often"; Sorbet's FOUR
distinct opt-out granularities — file/method/argument/call-site [sorbet-static-sigils] — exist precisely because
boundary placement needs to be that fine-grained).
DETECT: where are the boundaries in a mixed program (covered/uncovered, annotated/raw)? Is there a spelling for
boundary-crossing at each useful granularity, or only all-or-nothing? Was the unit of migration CHOSEN, or
inherited by accident?

lesson-14 — False positives kill optional tools: an opt-in checker that cries wolf gets deleted, so optional
analyzers must under-claim by default ([dialyzer-adoption], keynote verbatim: "The optimistic, 'never-wrong for
defect detection', approach to type inference that success typings advocate has been key in Dialyzer's
successful adoption... other approaches to typing Erlang and Elixir code have not managed to gain similar levels
of adoption"; OTP docs: "sound warnings (no false positives)") AND provide a cheap, LOCAL, standard silencing
idiom for the residue (ShellCheck `disable=SCnnnn` scoped to one command [shellcheck-directive-wiki]). A global
off-switch is not an escape hatch; it is an eject handle.
DETECT: what is the author's move when the tool infers wrongly at ONE site? Count the characters and the blast
radius. If the smallest override is file-global or requires restructuring working code, adoption folklore
predicts users eject.

lesson-15 — Annotation semantics must be settled before the ecosystem writes them down: Python's PEP 563/649
saga (what does an annotation even evaluate to?) burned years and deadlocked libraries because annotation
*semantics* changed under an installed base ([pep-563], canonical: "The features proposed in this PEP never
became the default behaviour, and have been replaced with deferred evaluation of annotations" — Superseded-By
649/749, after two Steering Council delays because runtime consumers depended on annotations being live
objects). Decide up front whether annotations are inert text, live runtime objects, or enforcement (D3
anti-lesson). One-way doors: anything the first library bakes into its published code is the permanent meaning.
DETECT: list every construct a LIBRARY author writes that the tool interprets; ask which of their meanings are
still marked provisional. Provisional semantics + published library = the saga, replayed.

## D. Shell-substrate specifics

lesson-16 — The exit byte is a colonized channel with reserved ranges and per-tool paradigms: 126/127/128+n are
shell-reserved ([bash-manual-exit-status]: "the shell may use values above 125 specially"; [posix-exit-status]
§2.8.2 verbatim: not-found "shall be 127", not-executable 126, signal "greater than 128"); builtins use 2 for
usage errors; grep's 0/1/>1 partition ([posix-grep-exit-status]) vs the plain failure paradigm — "Each process
or builtin decides the meaning of its exit status independently" [oils-error-handling]. Prior art on imposing a
richer taxonomy failed to achieve adoption ([sysexits-failed-taxonomy], OpenBSD man verbatim: "A few programs
exit with the following non-portable error codes. Do not use them."). Any dialect law over exit codes must dodge
the reserved ranges, must state what happens when a FOREIGN command emits the dialect's special codes, and must
handle the error-vs-false conflation explicitly (Oils grew a dedicated `boolstatus` builtin for exactly this).
DETECT: read the dialect's exit-code law; check collisions with 1/2/126/127/128+n; write the strawman where a
wrapped third-party tool returns the special code by coincidence; check the boolean-vs-failure paradigm split.

lesson-17 — Status semantics are context-warped by the host: sh rewrites what an exit status MEANS depending on
syntactic position (errexit immunity in if/&&/|| and all-but-last pipeline stages; the Disabled-errexit Quirk
inside functions called in conditional position; `local x=$(cmd)` masking [wooledge-errexit-faq105]
[oils-error-handling] — both first-rank sources agree). Any contract of the form "this function's status means
X" is therefore incomplete without a per-context table (command position / condition position / pipeline stage /
command-substitution / negation).
DETECT: take each status-bearing construct in the dialect; ask what the contract says in each of the five
contexts. Silence about context = authors will discover the warp in production, one context at a time.

lesson-18 — "Works with sh" requires naming the sh: bashisms under a POSIX promise are latent breakage, and the
ecosystem's one mass migration turned exactly that gap into a breakage wave ([debian-dash-switch]: Debian policy
"shell scripts specifying /bin/sh as interpreter must only use POSIX features"; migration ran on `dash -n` +
`checkbashisms`; "Programs should be written to the standard, and if they use extensions they should declare
them"). Even POSIX's own core has spellings with "fatally incompatible" cross-implementation meanings
([posix-echo-disaster]: portable echo is impossible with -n or escapes; "New applications are encouraged to use
printf"). Oils' entire compat strategy was corpus-defined ("run real bash scripts") [oils-errexit-catalog].
DETECT: find the dialect's named substrate (POSIX? dash? bash? which options?); then lint the project's OWN
fixtures/examples against it. Divergence between declared substrate and in-repo practice = the promise is
already broken in-house.

lesson-19 — In shell tooling the entrenched author-to-analyzer channel is the comment directive
([shellcheck-directive-wiki]: `disable`, and `source=` which exists precisely because static analysis hits
opacity on dynamic `source`); a design that refuses comment-annotations must supply an in-dialect escape hatch
of equal locality and cheapness, or users will invent unsanctioned conventions (and Hyrum will make those
conventions permanent). TENSION, both sides sourced: comment-directives are ALSO a documented interop hazard —
[crockford-json-comments]: "I saw people were using them to hold parsing directives, a practice which would have
destroyed interoperability" — so refusing them is a defensible position; the lesson is that the refusal must be
priced (an equally-local substitute), not that comments must be adopted.
[shaped-by-injection? — I know the project's no-annotations stance.]
DETECT: side-by-side: the dialect's local override vs `# shellcheck disable=SCnnnn`; compare locality, cost,
greppability, and whether the override itself has runtime meaning (per lesson-11's erasability inversion).

lesson-20 — Human output and machine output must be separate channels with separate stability promises from day
one ([git-porcelain-stability]; [clig-human-vs-machine] verbatim: "Changing output for humans is usually OK...
if the output is considered an interface, then you can't iterate on it"; "Keep changes additive where you can").
The failure mode: humans' output gets scraped, then frozen; or machines are fed prose that was never a contract.
Machine-generated text embedded inside human-facing artifacts is the compound case: it inherits BOTH audiences'
constraints ([autoconf-generated-shell-precedent]: the generated configure is deliberately self-contained —
"independent of Autoconf when they are run" — and the maintainers accepted a permanent maintainer-side cost:
"The primary goal of Autoconf is making the *user's* life easier; making the *maintainer's* life easier is only
a secondary goal").
DETECT: inventory emitted artifacts and rendered output; for each, which audience, and is any single artifact
serving both? For generated sh specifically: provenance header? regeneration story? stability policy? readable?

lesson-21 — Names injected into a shared flat namespace need a reserved, documented prefix from day one: sh has
ONE namespace for functions and (mostly) one for variables; the C/Python worlds solved this with reserved
prefixes, and YSH's own convention marks tool-special variables with a leading underscore
([oils-error-handling]: `_error`, `_pipeline_status`, "The leading `_` is a PHP-like convention for special
variables / 'registers' in YSH"). Unreserved special names collide with user code and with FUTURE tool versions
(adding a new special name after v1 can capture an existing user function silently — the reverse collision).
DETECT: list every name the dialect blesses (functions, variables, env) and every name its generated code
introduces; check for a documented reservation rule; strawman the collision in both directions (user shadows
tool; tool update captures user).

lesson-22 — Cleverness that must be decoded reads as noise and is a documented regret-shape
([wall-apocalypse-2], the designer's own diagnosis of sigil variance: "that initial funny character was trying
to do too much in both introducing the 'root' of the reference, as well as the context to apply to the final
subscript" — ruled invariant for Raku; his adjacent confession: "I was unduly influenced by Ada syntax here, and
it was a mistake"; [stefik-siebert-syntax]: 'intuitive' punctuation-heavy syntax performed no better than random
for novices). Boring words beat encodings; if a spelling requires a legend, the legend will be wrong somewhere
in the wild. The shape to hunt: ONE surface element carrying TWO stacked meanings ("trying to do too much").
DETECT: show each dialect spelling cold to the standard of "shell-literate reader, no manual": can they guess
(a) that it is special at all, (b) roughly what it means, (c) crucially, what it does NOT promise? Puns that
LOOK like familiar sh but mean more fail (c) by construction — cross-reference lesson-05.

lesson-23 — (provenance: minted after gatherer D1 returned, still strictly pre-corpus — see firming record.)
Safety-instinct surfaces measurably lose to exploration-friendly ones: requiring correctness ceremony up-front
was BEATEN in controlled studies by permissive-then-tighten designs ([stylos-clarke-constructors]: required
constructor parameters lost to create-set-call "for all" user personas, because required params "forced
programmers to instantiate each of the parameter objects before they felt they could explore";
[ellis-stylos-myers-factory]: the abstraction-for-correctness pattern cost time at p = 0.005; echoed at scale by
best-effort-first adoption stories [sorbet-stripe] [airbnb-ts-migrate]). For a correctness tool this is the
sharpest tension: every up-front demand the surface makes must buy its keep, or it drives users off the tool
entirely (cross-ref lesson-14's eject handle).
DETECT: for each ceremony the dialect requires before a user's first success (per role: admin, library author),
ask what the permissive-then-tighten alternative would be and whether the design considered it. Count the
mandatory concepts on the critical path to "hello world" per role.

---

## POST-HOC additions (after corpus exposure)

(none yet — anything added here after corpus reading begins is marked with date+reason)
