# synthesis — teaching DAG-structured causality to annoyed users in constrained text

Round opened 2026-07-25. 99 manifest rows / 97 distinct works (Barik and Johnson each
double-graded — once by me, once by a fan-out agent; both times A, which is corroboration,
not error). Grades: 30 A · 47 B · 19 C · 3 D. 9 rows `top-level-agent` (read end-to-end in
main context), 90 `subagent` (provisional per the skill's trust rule — the five load-bearing
subagent claims I have NOT re-verified against primaries are flagged in §5).

AI-gathered and AI-synthesised. Process evidence, never proof.

**The context being evaluated** (stated so findings can bear on it; not defended here). Dorc
plans to render decision-provenance chains as a linearised DAG in plain ASCII CLI text:
numbered links with join-nodes (`1a`/`1b -> 2`), a typed epistemic tier-word per link
(measured / vouched / ran / claimed / derived / consented), inlined massaged code excerpts
with gutters, density registers from a one-word tail up to a full teaching page, and a
pull-surface/push-surface split where pull renders maximally and push stays ruthlessly
selected. The built artifact is `spike/e2e/cases/survivebite27-naked-trust-chain`; the design
sits in `plans/24S` §4a, `notes/27V` §4, and `plans/286`.

---

## §0 — The four sentences

1. **The attention premise is inverted.** Users *do* read; reading costs them source-code
   rates; and *more* reading predicts *worse* outcomes. Every additional link is priced at
   the same rate as a line of the user's own code, and buys nothing automatically.
2. **Placement beats rendering by two orders of magnitude.** The same analysis at the same
   precision went from ~0% to >70% action rate purely by moving *where* it appeared. No
   rendering decision in this round has an effect size anywhere near that.
3. **The full chain is not an explanation; it is causal attribution, and the surplus links
   actively degrade inference from the load-bearing ones.** This is the dilution effect, and
   its mechanism is that the reader assumes you curated what you showed.
4. **Everything Dorc proposes has been shipped somewhere, and the *specific* shipped forms
   are systematically smaller than Dorc's.** Number only join-points, not every link. Two
   confidence ranks as glyphs, not six as words. Head-plus-tail with the middle elided and
   counted. Excerpt opt-in, not default. That convergence — Clang 2010, PubGrub 2018,
   TypeScript 2019, Nix 2022, independently — is the strongest single signal in the round.

---

## §1 — DON'T-DOS

Ordered by evidential strength, strongest first. Grade in brackets is the source's; the
confidence marker is mine for the *inference to Dorc*.

### dont-validate-by-preference — the methodological kill-shot
+SURE. Three independent controlled studies find enriched explanations are *preferred* and
*not more effective*. 8 semesters / 36,050 submissions: 78% said the enhancements helped;
same-error-repeat went 13.71% → 13.99% and time-between-submissions rose ~150s → ~250s
[A-pettit-enhanced-messages-inconclusive-2017]. n=106 randomised: verbose always-correct
explanations beat the terse control in 1 of 6 tasks while being rated significantly higher
[A-santos-llm-messages-not-silver-bullet-2024]. n=83 randomised, explanation + worked example
+ corrected diff: "no empirical evidence to support the use of enhanced error messages"
[A-enhancing-syntax-errors-ineffectual-2014].

**Bearing:** the field trial (`plans/250`, `plans/252`) must not ask whether `dorc why` was
illuminating. `252:377` currently asks exactly that. Ask instead: did they act, how fast, and
did they act *correctly*.

### dont-treat-attention-as-the-only-cost — a plausible chain steers people wrong
+SURE. 53 of 55 eye-tracked participants converged on the *same incorrect* fix because the
message's wording framed the solution space; incorrect solutions clustered into 0–3 types per
task [A-barik-developers-read-error-messages-2017]. The mechanism is observed directly
elsewhere: "once the error message mentions a missing part, students felt prompted to provide
the missing part, though this might not be the correct fix"
[A-marceau-mind-your-language-vocabulary-2011]. And the direction of effect is the wrong way
round: revisits to the message significantly predict *incorrect* resolution (G²=60.9,
p<.0001) [A-barik-developers-read-error-messages-2017].

**Bearing:** tier-words are authority signals. A chain that says `4. claimed && certsync.oracle.sh`
tells the user where to go. When the analysis is wrong — and best-effort means it will be —
the confident render makes the wrong action *more* likely, not less. Being wrong gets more
expensive as the render gets better.

### dont-render-the-whole-derivation — the surplus is not neutral
+SURE on the direction; ~SUSPECT on magnitude for expert readers. People select one or two
causes from chains that genuinely contain ~30 relevant ones; the discarded causes are *not*
noise, and people discard them anyway [A-miller-explanation-social-sciences-survey-2019].
Irrelevant-but-true material *dilutes* inference from the relevant material — and the
follow-up experiment shows the effect vanishes when subjects are told the material was chosen
at random, i.e. the harm is caused by the reader's assumption that you curated
[A-miller-explanation-social-sciences-survey-2019]. Measured sizes: Soufflé proof-tree heights
"can be more than 300", a 20-level fragment holds "over 15,000 nodes"
[A-datalog-proof-tree-heights-measured-2019]. Infer holds a 61-step interprocedural trace and
renders *three lines* [A-distefano-scaling-static-analyses-facebook-2019]. SHErrLoc's authors
(Peyton-Jones among them) explicitly considered and rejected reporting all contributing
locations as "often verbose and hard to understand"
[A-sherrloc-diagnosing-type-errors-2015]; the inventors of type-error slicing closed their own
paper the same way [A-type-error-slicing-haack-wells-2004]. Alloy's non-minimised core
highlights 2–3× as many constraints as the minimal one and "has not been particularly useful"
as a result [A-finding-minimal-unsat-cores-2008].

**The counterweight, which is real:** Kulesza et al., the one cited study on an actual
explanatory-*debugging* interface, found completeness beat soundness, and sound-but-incomplete
was the *least* preferred, producing the most clarification requests
[A-miller-explanation-social-sciences-survey-2019]. Dorc's user is debugging, not being
taught. This is why `law-pull-runs-wide-open` is not simply refuted — see §3.

### dont-let-the-readability-transform-be-unsound — the sharpest single artifact
+SURE. uv's error-message owner, on a collapse pass applied to a PubGrub derivation tree:
the transform produced `we conclude that open3d<0.9.0.0 cannot be used`, "which doesn't make
any sense here", because a term silently fell out — "we create the incompatibility … but it's
never reflected in the terms — so it's missing from the conclusion", and "that transformation
may be entirely unsound" [B-missing-term-in-derivation-tree-2024].

**Bearing:** Dorc's chain is *provenance*, not a message. An elision heuristic is a second
inference system with its own soundness burden, and a dropped join-node is a false provenance
claim. Any fold/collapse pass needs tests at the *tree* level, not the byte level — which
cuts against `27V:rul-output-form-unwelded`'s "byte-golden re-blesses freely" posture for the
*structural* assertions specifically.

### dont-give-every-link-a-number — the one canonical spec says the opposite
+SURE. PubGrub — the only artifact in the corpus that solves *precisely* Dorc's problem
(linearising a derivation DAG into plain text for a human) — numbers a line only when a
pre-pass finds that node has out-degree ≥ 2. "we only number incompatibilities that we know
will need to be referred to later on. In the simple linear case, we don't include line numbers
at all" [A-pubgrub-solver-specification-error-reporting-2018]. Its seven-node linear example
carries **zero** numbers; its branching example carries **one**. And its join is not
`1a`/`1b -> 2`: branch A in full → its *conclusion only* numbered → blank line → branch B in
full → a prose back-reference to `(1)`.

Cargo shipped Dorc's back-reference (`(*)`) and then documented in its own manual that users
can't follow it, offering only full expansion as the fix
[B-cargo-tree-dedupe-cross-reference-difficulty-2024].

### dont-count-on-the-controlled-vocabulary-landing
+SURE, and this is the most direct threat to the tier-word scheme. DrRacket's deliberately
chosen terms, after 2+ months of daily exposure: only 4 of 15 identified correctly by >50% of
students (n=163, three universities). Using a term in lectures bought +13.8%. A supplied
vocabulary guide showed **no reliable effect**
[A-marceau-mind-your-language-vocabulary-2011]. An OCaml compiler maintainer, refusing to add
provenance hints to a type error: "I find that those hints only add noise"
[C-ocaml-discuss-improve-type-errors-2023]. Users will also *disbelieve* a declared tag when
their own code-reading disagrees: "programmers do side with the implementation details in
spite of type annotations and may assume the annotations in the program to be incorrect"
[B-typeslicer-wizard-of-oz-user-study-2024]. And epistemic markers are read as *stance*, not
metadata — Malle's marked/unmarked reasons sit on a distancing-to-embracing scale, so the
reader infers whether the system endorses the claim from the marker you picked
[A-miller-explanation-social-sciences-survey-2019].

Contrast the shipped analogues: Alloy uses **two** levels rendered as colour intensity inline
[B-alloy-unsat-core-quickguide-2021]. rustc's model is two ranks by glyph — primary `^^^` =
what, secondary `---` = why — chosen so users "glance at the colored labels and quickly form
an educated guess" [A-rust-rfc-default-expanded-errors-2016]. Six words is 3× the deployed
precedent.

### dont-let-one-fact-fan-out-into-n-lines
+SURE. uv's own maintainer, against his own project: one user-level fact ("no wheels, any
version") rendered as 8–40 near-identical derivation lines because each was individually true
[B-uv-no-build-message-very-verbose-2024]. pip-tools: the *better* resolver's un-deduplicated
dump repeated the same package 15× and was experienced as a regression against the weaker
resolver's 4-line summary; reporter's suggested fix was "dont use new resolver"
[B-backtracking-resolver-human-unreadable-error-2023]. TypeScript's first-party diagnosis of
its own nested chains: "which quickly meant people learned to read error messages by reading
the first and then last line" — and its fix was to *delete the chain*, not to number or tag it
[B-typescript-flattened-error-reporting-2019].

Dorc's line shape (tier-word + both loci + gutter'd excerpt ≈ 4 lines/link) has maximum
inter-line self-similarity, which is the exact property TypeScript identified as causing
first-and-last-line reading.

### dont-default-the-per-link-excerpt
~SUSPECT (two independent reversals, no measurement). Nix shipped exactly this and removed it:
issue titled "`nix why-depends --derivation` output is very noisy, a little hard to read", with
"Only the inclusion of colors on the matched hashes make it even semi-readable, and of course
those go away when you have no color" [B-nix-why-depends-noisy-issue-2022]; merged eight days
later as "Make `nix why-depends` quieter by default", moving excerpts behind `--precise` and
gaining ~3× speed [B-nix-quieter-by-default-pull-2022]. Buck2's source-locus flag `--stack` is
opt-in for the same reason [A-buck2-explain-command-reference-2025]. Clang caps caret output at
16 lines by default [B-clang-users-manual-diagnostic-defaults-2026].

Note this is in *tension* with the excerpt do-dos in §2 — resolved there.

### dont-order-by-traversal
+SURE. Gradle maintainer, defending the current ordering on a WONTFIX: "it is the order the
dependencies are encountered when walking the graph" — on a project with one direct dependency
where the actionable path lands at position 4
[B-gradle-dependencyinsight-noisy-issue-2025]. Miller supplies what the order *should* be:
fact/foil difference first, then intentionality/functionality, then abnormality, then
necessity/sufficiency and robustness; recency is a weak tiebreaker that controllability
overrides [A-miller-explanation-social-sciences-survey-2019].

### dont-assume-the-chain-adds-information
~SUSPECT. A practitioner's full transcript, after six failed `nix why-depends` invocations:
"Anticlimactically, careful inspect reveals this to be essentially a cleaned-up version of the
output we got from `darwin-rebuild` in the first place"
[B-nix-why-does-system-depend-2025]. Cheap test before building: what does the chain add over
the diagnostic the user already holds?

### dont-name-a-decision-you-cannot-cash-out
+SURE. Bazel's `--explain` log prints `One of the files has changed.` for every action.
The request for *which* file died as `not_planned` because "Bazel doesn't store the checksums
of each input or even the set of inputs for each cached action … So when one of these files
changes, it's detected, but Bazel cannot tell which one changed"
[B-bazel-explain-what-files-changed-2021]. `--verbose_explanations` is now `@Deprecated` and
`UNDOCUMENTED`.

**Bearing:** decide *now*, per tier-word, whether it can be cashed out into a retained
witness. Once the data isn't kept, the fix is a storage-architecture change nobody funds.

### dont-let-the-renderer-outgrow-the-solver
-GUESS — **uncited lead, not evidence.** A gatherer reports pubgrub-rs issue #293 as an open
bug with `// Stack overflow here` sitting on `DefaultStringReporter::report(&error)`,
reproduced from cargo's own resolver test suite. That issue is **not in the manifest** — it was
described in the agent's prose but never registered, so nobody in this round has read it and
it must not be cited. Recorded here only as a cheap thing to check: the reporter can die on
inputs the solver survives. The registered neighbour, on a different failure of the same
reporter, is [B-missing-term-in-derivation-tree-2024].

### dont-expect-ordered-or-terminating-consumption
+SURE. Fault-localisation study: "37% of the visits jumped more than one position and, on
average, each jump skipped 10 positions"; moving the faulty statement from rank 83 to rank 16
produced *no* speedup; participants "spent (or wasted) on average 61% of their time continuing
to inspect statements … after they had already encountered the fault"
[A-are-automated-debugging-techniques-helping-2011]. Practitioners abandon fast and silently:
31% of tasks never engaged the slice toggle, 11% turned it off within 15 seconds
[B-typeslicer-wizard-of-oz-user-study-2024]. Attention budget ≈ five items
[A-practitioners-expectations-fault-localization-2016].

**And the demographic sting:** a statistically significant *negative* correlation between years
of experience and rating fault-localisation "Essential" (ρ=−0.14, p=0.007)
[A-practitioners-expectations-fault-localization-2016]. Dorc's admin is measured as the *least*
receptive population for this tool class.

### dont-mix-modalities-carelessly-on-one-line
-GUESS (author's hypothesis, untested). Barik's proposed reason error messages read harder than
source: they are "both natural language … and code, but are not entirely either", forcing a
modality switch [A-barik-developers-read-error-messages-2017]. Dorc's link line is
tier-word (prose) + kind-identifier (code) + `file:line` + excerpt — three or four switches per
line.

### dont-assume-research-depth-implies-adoption
+SURE, and it is the base rate for this whole area. 25 years after Weiser, Dagstuhl
participants "could name no existing debugger that includes slicing"; a GDB slicing patch
existed and "has now fallen by the wayside" [B-making-slicing-mainstream-dagstuhl-2005].
rustc's `--explain errors` — the user's own code templated into a teaching narrative, i.e.
Dorc's teaching register — was RFC-accepted in 2016 and closed unshipped in 2020; the only
substantive reason given: "Some others (the longest or most 'general' ones) are almost
impossible as is" [B-rustc-explain-expansion-abandoned-2016]. That inverts the value curve:
the register is cheap where it is least needed.

---

## §2 — DO-DOS

### do-number-only-join-points
[A-pubgrub-solver-specification-error-reporting-2018]. Pre-pass the graph for out-degree;
number only nodes referenced later; fold every derived node whose sole job is feeding the next
one ("You can skip every other derived incompatibility without losing clarity"). Blank-line
between branches, prose back-reference at the join.

### do-keep-head-and-tail-elide-the-middle-state-the-count-name-the-flag
[A-clang-template-backtrace-limit-commit-2010] [A-clang-macro-backtrace-limit-commit-2010]
[B-clang-constexpr-backtrace-limit-commit-2011]. Three commits by the same author, 2010–2011,
shipping a defaulted-ON truncation of exactly Dorc's structure: "we will print N/2 of the
innermost backtrace entries and N/2 of the outermost … then skip the middle entries with a note
… This should eliminate some excessively long backtraces that aren't providing any value."
Shipped defaults: template backtrace 10, macro backtrace 6, error limit 20, caret lines 16
[B-clang-users-manual-diagnostic-defaults-2026]. Soufflé's default proof-tree depth is **4**
[B-souffle-explain-command-docs-2019].

### do-show-only-the-delta-between-adjacent-links
Three independent convergences: Clang's default-on `-fno-elide-type` ("elide as many template
arguments as possible, removing those which are the same … leaving only the differences")
[B-clang-users-manual-diagnostic-defaults-2026]; Elm 0.16 type diffs
[B-elm-compilers-as-assistants-2015]; TypeScript 3.7 flattening
[B-typescript-flattened-error-reporting-2019]. And Bazel's principled version of the join:
"topologically-equivalent nodes are merged together into a single node with multiple labels …
because typical result graphs contain highly repetitive patterns"
[A-bazel-query-language-reference-2026].

### do-frame-contrastively — it is both what people want and cheaper to compute
[A-miller-explanation-social-sciences-survey-2019]. "explaining a contrastive question is often
easier than giving a full causal attribution because one only needs to understand what is
different between the two cases, so one can provide a complete explanation without determining
or even knowing all of the causes." Answering "why P?" non-contrastively "is equivalent to
providing all causes for P — something that is not so useful." For a one-shot CLI with no foil
elicited, Miller's own workaround is a *default* foil: the normal/stereotypical case.

**For Dorc that is unusually cheap.** The foil is already sitting there: the line's other
disposition. "elided rather than guarded, because …" / "guarded rather than elided, because …".
That is one link, not six.

### do-use-the-DAG-as-a-filter-not-as-output
+SURE, best-supported positive result in the round. "the type error slice is so important that
it is actually beneficial to automatically *discard* expressions that are not part of the
slice, rather than letting the classifier learn to do so … this domain-specific insight is
crucial" [A-learning-to-blame-type-errors-2017]. Compute the whole DAG; ship the survivors.

### do-anchor-to-code — but byte-faithfully, and as the winning *shape*
The condition that beat both a terse compiler and a verbose LLM, on objective *and* subjective
measures, was: `Error:` line + relevant source excerpt + `Help:`/`Note:` sections, "greatly
inspired by the diagnostics emitted by the Rust compiler"
[A-not-silver-bullet-llm-pems-2024]. Length perception: 88.1% "just right"
[A-santos-llm-messages-not-silver-bullet-2024]. Barik names LLVM `scan-build` — "the error as
a sequence of steps that the developer can follow alongside in the context of the code" — as
the promising alternative [A-barik-developers-read-error-messages-2017]. Nienaltowski's
visual-inline style was significantly *faster* than both short and long prose (166.9s vs
247.2s / 223.0s) [B-nienaltowski-what-can-help-novices-2008].

**But: "massaged" is the word to worry about.** Elm's entire claimed win is byte-fidelity —
"The error shows the code exactly as you wrote it … Users can ask 'does this look like that?'
without really needing much conscious analysis" [B-elm-compiler-errors-for-humans-2015]. Any
normalisation reintroduces the recognition cost the excerpt was there to remove.

**Resolution of the excerpt tension:** the evidence splits on *how many*. One excerpt at the
locus wins (Santos, Nienaltowski, Barik). One excerpt *per link* over N links loses (Nix,
Buck2, Clang's caret cap). Excerpt the endpoints; text-only the middle.

### do-demote-oracle-internal-links — the "just my code" move
[A-concepts-error-messages-humans-2022] on GCC: "This gives us a lot of context. Too much, I
would argue. I don't think most users need to know which line of code in a standard library
header the instantiation failed at … Ideally we would minimise this … by only diagnosing the
user code." Dorc's oracle/book split gives a *cleaner* boundary than C++ has: links whose
`file:line` lands inside an oracle are stdlib frames. Collapse them to one summary by default;
the admin does not own them.

### do-put-facts-on-the-code-as-labels-not-as-numbered-links
[A-rust-rfc-default-expanded-errors-2016]: "Where possible, use labels on the source itself
rather than sentence 'notes' at the end." rustc-dev-guide: secondary labels are "preferred, as
it is typically less verbose" [B-rustc-dev-guide-diagnostics-policy-2026]. **Restated for
Dorc: the chain should be the residue after everything label-able has been demoted to a
label.**

### do-emit-grep-parseable-loci-and-a-machine-format
Bazel's `--output location` is deliberately grep-shaped so "tools that can parse the latter
(such as Emacs or vi) can also use the query output to step through a series of matches"
[A-bazel-query-language-reference-2026]. Buck2 puts it bluntly in its own output: "This command
output is not machine readable. Robots, please use `buck2 log show`"
[B-buck2-log-subcommand-reference-2023]. P2429 asks for SARIF/JSON so tools can "filter,
manipulate, and visualise" [A-concepts-error-messages-humans-2022]; Elm shipped `--report=json`
in the same release as the human format [B-elm-compiler-errors-for-humans-2015]. This is
predicted by information-foraging theory: users *enrich* their environment (grep, bookmarks,
tabs) to cut travel cost rather than reading linearly
[B-lawrance-information-foraging-debugging-2013], and it is observed — experts diff two dumps
rather than read one [B-bazel-action-nondeterminism-jmmv-2025]
[C-debugging-dependencies-in-gradle-2024].

### do-give-the-artifact-a-mental-model — the strongest argument *for* a chain
The diagnosed root cause of ranked-list failure is incoherence, not size: "most ranking based
automated debugging techniques remove any source of coherence by mixing statements in a
fashion that has no mental model to which the developer can relate … developers are currently
presented with a set of apparently disconnected statements"
[A-are-automated-debugging-techniques-helping-2011]. A causal walk has a mental model a ranked
list does not. This is the round's best affirmative case for Dorc's shape — provided it reads
as a walk and not as a table of facts.

### do-name-tier-words-from-a-closed-documented-table-that-stacks
Gradle's "Selection reasons" is the closest *working* shipped analogue: a small closed
vocabulary (`By conflict resolution`, `By constraint`, `By ancestor`, `Selected by rule`,
`Forced`), enumerated in a Reason/Meaning table because the term is *not* self-explanatory and
Gradle does not pretend otherwise; reasons **stack on one node** rather than one-word-per-link;
and authors inject prose via `because` [A-gradle-viewing-debugging-dependencies-2026]. That
last is a good fit for Dorc's oracle-author/admin split.

### do-state-the-reading-direction
Cheapest fix in the whole corpus, applied by nobody. The one Stack Overflow answer that decodes
npm's ERESOLVE tree opens with "First you should start to read the problem from the bottom to
the top"; npm never says this in the error
[C-npm-eresolve-unable-to-resolve-tree-2020]. A commenter: "I kept trying to read from top to
bottom, so I was drawing completely the wrong conclusions."

### do-expect-experts-to-turn-density-down-and-read-their-floor-off-their-behaviour
A Rust user requesting `--message-format=short`: "Normally I just need the compiler to give me
the file, line/col number and name of the error and I can spot it myself very quickly" — and he
was already grepping it by hand with `rg "(error\[\w+\])|(^\s+--> \w+/\w+)"`
[C-cargo-message-format-short-request-2017]. That regex *is* the lowest rung: headline +
`file:line`, nothing else.

---

## §3 — Direct bearings on the stated Dorc direction

| Element | Verdict | Why |
|---|---|---|
| Linearised DAG in ASCII | **threatened at length, confirmed short** | Bazel first-party: a *set of paths* "cannot be easily visualized when rendered in a linear form" [A-bazel-query-language-reference-2026]. PubGrub and Nix both ship legible linearisations — of 2–7 lines. Nix's 7-line default is "fairly easily" actionable; `--all` at ~30 lines "looks more similar to the original output" [B-nix-why-does-system-depend-2025]. |
| Numbered links (every link) | **threatened** | PubGrub numbers only out-degree ≥2, zero in the linear case [A-pubgrub-solver-specification-error-reporting-2018]. Dorc's six-link flagship would carry ~0–1 numbers under that rule. |
| Join-nodes `1a`/`1b -> 2` | **threatened in that spelling** | PubGrub's join is branch-then-blank-then-branch-then-prose-backref, not sibling labels. Cargo's `(*)` back-reference is documented-unfollowable [B-cargo-tree-dedupe-cross-reference-difficulty-2024]. magit's maintainer, on ASCII specifically: a node that is both merge and branch-point "is hard to express … using ascii it looks confusing", and "Graph lines may 'cross'. That's also not easily expressed using ascii" [B-magit-readable-log-graphs-2017] — those are precisely Dorc's two cases. |
| Six epistemic tier-words | **most threatened element** | Vocabulary is not acquired and a glossary does not fix it [A-marceau-mind-your-language-vocabulary-2011]; markers read as stance, not metadata [A-miller-explanation-social-sciences-survey-2019]; a maintainer calls provenance hints "noise" [C-ocaml-discuss-improve-type-errors-2023]; users disbelieve declared tags [B-typeslicer-wizard-of-oz-user-study-2024]. Shipped precedent is 2 ranks (Alloy, rustc), not 6 words. **Salvage path:** Gradle's closed-documented-stacking table [A-gradle-viewing-debugging-dependencies-2026]. |
| Inlined excerpts w/ gutters | **confirmed once, threatened per-link** | Wins as *the* anchor [A-not-silver-bullet-llm-pems-2024] [A-barik-developers-read-error-messages-2017] [B-nienaltowski-what-can-help-novices-2008]; loses replicated per-link [B-nix-quieter-by-default-pull-2022] [A-buck2-explain-command-reference-2025] [B-clang-users-manual-diagnostic-defaults-2026]. |
| "massaged" excerpts | **threatened** | Elm's win is byte-fidelity; normalisation gives back the recognition savings [B-elm-compiler-errors-for-humans-2015]. |
| Density registers (concept) | **confirmed as necessary** | Expertise reversal is measured and large: integrated formats help novices (d=+1.67/+1.89) and *harm* experts (d=−0.44/−0.88) on a fault-finding task [A-kalyuga-expertise-reversal-effect-2007]. One density cannot serve admin and oracle-author. |
| Density registers (that anyone climbs) | **unvalidated, not prior art** | No telemetry, survey, or usage data on `rustc --explain` exists anywhere. Treat "users will pull for more detail" as an untested assumption. |
| Full teaching page | **threatened** | rustc designed exactly this, accepted the RFC, and abandoned it after four years — the hard cases "are almost impossible" [B-rustc-explain-expansion-abandoned-2016]. Three controlled studies find the teaching register preferred and ineffective (§1). |
| `law-pull-runs-wide-open` | **threatened, with a real counterweight** | Dilution says surplus links degrade the relevant ones, *because the reader assumes curation* [A-miller-explanation-social-sciences-survey-2019]. But Kulesza found completeness beat soundness in a debugging interface, and sound-but-incomplete was least preferred. **Reconciliation the evidence supports:** "maximally" should mean *complete over the selected question*, not *the whole closure* — and if an exhaustive dump is offered, **label it as exhaustive rather than curated**, since Tetlock's follow-up made dilution vanish precisely by telling readers the material was not selected for relevance. |
| `rul-chain-is-pull-only` | **threatened** | The largest effect in the round is placement: identical analysis, identical FP rate, ~0% → >70% action rate by moving reports into the user's current context; the named mechanism is context-switch cost [A-distefano-scaling-static-analyses-facebook-2019]. Corroborated: "having to open another perspective to know what is going on is a guarantee that unmotivated people will not do it" [A-why-developers-dont-use-static-analysis-2013]. A pointer line is not free — Cargo prints `cargo tree --invert --package @` and the printed command doesn't work in common configs [C-cargo-update-recommends-broken-invert-command-2024]. |
| Push stays ruthlessly selected | **confirmed** | Unanimous; and Miller adds that unprompted explanation *decays* in value: "An intelligent agent that presents — unprompted — an explanation alongside every decision, runs a risk of providing explanations that become less needed and more distracting over time" [A-miller-explanation-social-sciences-survey-2019]. Aligns with `plans/111`'s precise-or-silent. |
| The chain as the product's flagship | **question the projection** | The one time a practitioner spontaneously designed this artifact, they asked for an **impact slice** — "what the problem is and *what else could be affected*", via call hierarchies [A-why-developers-dont-use-static-analysis-2013] — not a **provenance derivation** ("why do I believe this"). Same edge set, opposite direction. Only the impact projection has demand evidence. |

---

## §4 — Open questions worth a follow-up turn

1. **`ask-impact-versus-provenance-projection`** — the sharpest strategic question this round
   surfaced. Dorc's `why` traverses backward (what licensed this decision). Every piece of
   practitioner demand evidence found asks for the forward projection (what else does this
   touch). Both are cheap given the same graph. Which is the flagship?
2. **`ask-measure-our-own-chain-length-distribution`** — Soufflé measured theirs and redesigned
   around it (heights >300 → default depth 4). Nobody else in the corpus measured anything, and
   "chains are usually short" is asserted everywhere and quantified nowhere. Dorc already has a
   corpus and a whylog; a histogram of link-count and branch-factor over the e2e corpus is
   cheap and would settle most of §1 and §2 empirically rather than by analogy.
3. **`ask-why-not-provenance-summarisation-literature`** — Lee/Ludäscher/Glavic, "Approximate
   Summaries for Why and Why-not Provenance" (PVLDB 13(6) 2020) states that why-not provenance
   "can be very large, resulting in severe scalability and usability challenges" and proposes
   pattern-based summarisation with formal informativeness/conciseness/completeness metrics.
   **UNREAD** — the PDF resisted extraction. This is the source most likely to change the
   design, because it would give Dorc a principled summarisation objective instead of the folk
   heuristics above. Extended version at `arxiv.org/abs/2002.00084`.
4. **`ask-do-ladders-get-climbed`** — no telemetry exists anywhere for any tool's `--explain`
   surface. Dorc could be the first to instrument it, cheaply, in the field trial.
5. **`ask-tier-word-cash-out-audit`** — per Bazel's regret: for each of the six tier-words, can
   it be cashed out *today* into a retained witness the user can inspect? Any that cannot
   should either gain a witness now or be merged away.
6. **`ask-contrastive-foil-selection`** — if the chain becomes contrastive, what is the default
   foil? The line's other disposition is the obvious candidate and is essentially free.
7. **Reddit was not read.** `robots.txt` disallows all agents; three gatherers independently
   stopped rather than route around it. Practitioner sentiment on `npm ERESOLVE`, C++ template
   errors, and `cargo tree --no-dedupe` is therefore absent. Cheapest human-in-the-loop upgrade
   available: `r/rust/comments/nvd6y7`, `r/typescript/comments/qgalog`, `r/cpp_questions` on
   template verbosity.

---

## §5 — Honesty section

**Unverified subagent grades that are load-bearing above.** Per the skill's trust rule these
are provisional until re-read against primaries:
- The PubGrub numbering rule and its two literal renderings
  [A-pubgrub-solver-specification-error-reporting-2018] — highest-value claim in §2, taken
  entirely on a gatherer's read.
- The three Clang backtrace-limit commit messages
  [A-clang-template-backtrace-limit-commit-2010] et al. — verbatim commit text, high-trust, but
  I did not open the commits.
- The Nix reversal pair [B-nix-why-depends-noisy-issue-2022] [B-nix-quieter-by-default-pull-2022].
- Miller's dilution and causal-attribution passages
  [A-miller-explanation-social-sciences-survey-2019] — the gatherer read ar5iv HTML, not the
  published AIJ text; wording should be spot-checked before anything quotes it externally.
- Soufflé's measured tree heights [A-datalog-proof-tree-heights-measured-2019].

**Corroboration where it exists.** The two works graded twice, independently, by me and by a
gatherer, both landed on A with compatible reasoning
([A-barik-developers-read-error-messages-2017] / [A-do-developers-read-error-messages-2017];
[A-johnson-why-not-static-analysis-tools-2013] / [A-why-developers-dont-use-static-analysis-2013]).
[A-santos-llm-messages-not-silver-bullet-2024] and [A-not-silver-bullet-llm-pems-2024] are the
same work under two URLs, likewise concordant. I have left the duplicate rows in place rather
than hand-edit the manifest.

**Known transfer weaknesses.** The comprehension literature is overwhelmingly CS1 novices;
Dorc's admin is not a novice programmer — though they *are* a novice at Dorc's private
vocabulary, which is why [A-marceau-mind-your-language-vocabulary-2011] still bites.
[A-kalyuga-expertise-reversal-effect-2007] measures learning in lab instruction, not diagnosis
under time pressure. Miller's selection findings rest largely on 1980s–90s student-vignette
studies with no reported effect sizes or replication status, and concern lay judgements about
social causation, not expert readings of machine traces.

**No user study exists in either the build-system or the package-manager vein.** Every
behavioural claim there is revealed preference — what practitioners describe doing, or what
maintainers changed. The measured evidence in this round comes entirely from compiler
diagnostics, fault localisation, and instructional psychology.
