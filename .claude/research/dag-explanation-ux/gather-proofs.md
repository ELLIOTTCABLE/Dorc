# Vein: type-error provenance, unsat cores, proof/derivation explanation UX

Gathered 2026-07-25 by a read-only web source-gatherer. 17 sources fully read.
Every claim below is anchored to a verbatim excerpt from a source I actually read end-to-end
(or, where read-depth was partial, that fact is stated in the grading line).

The dominant finding: **the field measured its own artifacts and found them too big to read.**
Every mature system in this vein ships a *truncation* mechanism, not the derivation.

---

## (a) DON'T-DOS

### dont-render-the-whole-derivation
The only system that actually ships a terminal proof-tree (Soufflé `explain`) measured its own
trees and found them unreadable, then capped its default render depth at 4 levels.

- Proof-tree heights in real analyses (Doop/DaCapo, Java points-to): **"can be more than 300"**;
  branching factor **1.466**; a 20-level fragment already **"contain[s] over 15,000 nodes"**;
  a full tree means the user **"may have to interpret millions of nodes to find an explanation."**
  [A-datalog-proof-tree-heights-measured-2019]
- First-party docs: `setdepth` default is **4**, because **"a full derivation tree is too unwieldy
  to be understood."** Beyond that, Soufflé needed a *separate ncurses mode* (`-t explore`)
  purely "to allow scrolling around a large proof tree."
  [B-souffle-explain-command-docs-2019]

Implication for Dorc: a linearized DAG of numbered links is a proof tree with the tree flattened.
The depth cap is not a nicety, it is the shipped design.

### dont-report-every-contributing-location
Reporting the full set of contributing locations is the approach the field tried first and
then abandoned — including by the people who invented it.

- SHErrLoc authors (Zhang, Myers, Vytiniotis, **Simon Peyton-Jones** — i.e. GHC's lead):
  **"Rather than reporting the location of a single failed constraint, we might think to report all
  locations that might contribute to the error... But such error reports are often verbose and hard
  to understand, because many expressions can be at least partly involved in a given failure."**
  [A-sherrloc-diagnosing-type-errors-2015]
- Same authors, journal version: **"The constraints along unsatisfiable paths form a complete
  explanation of the error, but one that is often too verbose."** And on slices specifically:
  **"both variants of this approach can still require considerable manual effort to identify the
  actual error within the program slice, especially when the slice is large."**
  [A-sherrloc-static-holistic-locator-2017]
- **Haack & Wells themselves** — the inventors of type-error slicing — closed their own paper by
  rejecting explanation-generation: **"Much of the related work on type error analysis has been on
  sophisticated ways for automatically generating type error explanations. Such explanations tend to
  be complicated and lengthy. We believe that it is most important to accurately locate type errors,
  and display type error locations in a user-friendly way. For understanding errors, programmers
  typically use additional semantic knowledge that cannot be provided automatically anyway."**
  [A-type-error-slicing-haack-wells-2004]

### dont-assume-a-correct-pointer-is-enough (and beware anchoring)
An explanation that points somewhere plausible-but-wrong does *active harm* — it recruits the
reader into the wrong fix.

- Barik et al., eye-tracking, n=56: on task T1 only **2/55** participants produced a correct fix and
  on T2 only **1/55**; **53 of 55** followed the message's framing into an incorrect solution.
  The paper's own walkthrough: "Unfortunately, this fix turns out to be incorrect."
  [A-do-developers-read-error-messages-2017]
- Same study, the counter-intuitive direction of the effect: **"As revisits to error messages
  increase, the probability of successfully resolving a compiler error decreases"**
  (χ², df=1, G²=60.9, p<.0001). More time in the explanation predicted *worse* outcomes.
  [A-do-developers-read-error-messages-2017]

### dont-count-on-ordered-consumption
Users do not walk your numbered list in order, and rank barely matters.

- Parnin & Orso, n=34+24+10: **"37% of the visits jumped more than one position and, on average,
  each jump skipped 10 positions"**; **"each participant had 10.3 zigzags, with an overall range
  between 1 and 36."**
  [A-are-automated-debugging-techniques-helping-2011]
- Rank manipulation had no effect: moving the faulty statement from **rank 83 to rank 16 produced no
  speedup at all** (group D was actually *slower*: 18:30 vs 15:12).
  **"Therefore, overall, the results provide no support for Hypothesis 3."**
  [A-are-automated-debugging-techniques-helping-2011]

### dont-expect-recognition-on-arrival ("perfect bug understanding" is false)
Landing the user on the right link does not end the task.

- **"Only 1 participant out of 10 stopped using the tool after clicking on the fault. The remaining
  participants, on average, spent another ten minutes using the tool after they first examined the
  faulty statement. That is, participants spent (or wasted) on average 61% of their time continuing
  to inspect statements with the tool after they had already encountered the fault."**
  [A-are-automated-debugging-techniques-helping-2011]

### dont-exceed-the-attention-budget (~5 items, first screen)
- **"programmers will stop inspecting statements, and transition to traditional debugging, if they do
  not get promising results within the first few statements they inspect... This is consistent with
  other research in search tasks, where it is clearly shown that most users do not inspect results
  beyond the first page."**
  [A-are-automated-debugging-techniques-helping-2011]
- Corroborated by survey: Kochhar et al. surveyed **386 practitioners across 33 countries**; the
  minimum-success-criterion options offered were Top-1/2/5/10/50, and Seidel et al. summarise the
  result as **"developers are unlikely to examine more than around five potentially erroneous
  locations before falling back to manual debugging."**
  [A-practitioners-expectations-fault-localization-2016] / [A-learning-to-blame-type-errors-2017]

### dont-let-noise-in, at-all (rapid, silent abandonment)
Abandonment is fast and unannounced; you get one shot.

- **"several participants indicated in their questionnaire that they would eventually abandon the tool
  after what they felt was too large a number of false positives. This suggests that there is a very
  rapid interest drop-off if developers cannot feel confident about the results they receive from the
  tool."** Participant verbatim: **"The ranking list was too long and didn't help me with enough
  context. Actually, I know NanoXML and work with it, but [...] it was faster to use breakpoints."**
  [A-are-automated-debugging-techniques-helping-2011]
- Measured in a slicing UI: **"In almost a third of the task runs where the slicing tool was
  available (31.03% out of 87 tasks), participants never toggled the slice information"**, and
  **"About half of those (11.49% of the total tasks) turned the tool off in the first 15 seconds
  after running the code, which may indicate cases where the participants were not interested in the
  tool results."**
  [B-typeslicer-wizard-of-oz-user-study-2024]

### dont-ship-a-non-minimized-set
The cheap over-approximate core is 2–3x too big, and that over-reporting is exactly why the feature
sat unused.

- Alloy's original core extractor **"has not been particularly useful since the highlighting has been
  too conservative, often including constraints that were not in fact used"**; the fast algorithm
  **"typically highlights 2 to 3 times as many constraints"** as the minimal one.
  [A-finding-minimal-unsat-cores-2008]
- And minimization is not free: **"SAT solvers do not generally provide minimal cores, which would
  require too much computation to produce"**, plus the level-crossing problem —
  **"a small core at the boolean level may be translated back to a large core at the specification
  level."**
  [A-finding-minimal-unsat-cores-2008]
- Alloy first-party UI exposes this as user-visible knobs because there is no good default:
  three "core size" strategies where **"The 'Fast' strategy takes the least amount of computation,
  but the resulting core is also the least accurate, and may contain a number of false positives"**,
  and four granularity levels because **"a top-level conjunct might be too coarse-grained for
  effectively locating the source of an inconsistency."**
  [B-alloy-unsat-core-quickguide-2021]

### dont-add-hints-because-you-have-the-data
Provenance you *can* emit is not provenance you *should* emit. Watch a compiler maintainer refuse it.

- octachron (OCaml compiler error-message author), on adding constructor/type provenance hints:
  **"We could try to add more hints, listing all types in scope that defines the mismatched
  constructors for instance, or all constructors of the expected types. However, going back to the
  format6 example, I find that those hints only add noise"** — and on hints in general,
  **"(with the hint semantic being that sometimes they convey non-useful information)."**
  [C-ocaml-discuss-improve-type-errors-2023]
- The failure mode is visible in the same thread. The maintainer's defence of the message is that it
  is *locally complete and non-presumptuous*: "it describes all the information that we have locally
  at the point of the conflict. The error message is not making any possibly wrong assumption or
  ignoring one possible fix." A newcomer's reply enumerates six confusions and lands on:
  **"So the information they read out of the error message is 'Error: This variant expression is
  expected to have SOME OTHER TYPE' with no idea how that type should look like or what's wrong.
  The message would be less confusing if it just pointed at the variant expression and said something
  like 'There is something wrong!'"**
  [C-ocaml-discuss-improve-type-errors-2023]

Direct read for Dorc: *locally-correct, complete, non-presumptuous* is precisely the design posture
that epistemic tier-words encourage — and it is the posture that degraded to noise here.

### dont-dump-a-linear-trace (the Prolog verdict)
The logic-programming community had exactly Dorc's artifact — a linearized derivation — and
abandoned it in favour of interactive narrowing.

- **"Unfortunately, tracing falls short in several critical respects... traces quickly get very
  complex and hard to understand."** And on the framing:
  **"It would be extremely inconvenient and in most cases almost impossible to follow the exact steps
  that Prolog performs, i.e., how it fails. We care more about high-level explanations than about
  low-level traces."**
  [B-triska-declarative-prolog-debugging-2016]
- The replacement is *generalize-away bisection* (`(*)/1`), not a printed derivation: strike goals
  out until behaviour flips, then read the one goal that flipped it.
  [B-triska-declarative-prolog-debugging-2016]

### dont-hide-the-non-relevant-parts
When TypeSlicer blurred out-of-slice code, users pushed back on the *elision itself*, not the slice.

- **"negative comments suggested that the tool would be improved by highlighting or underlining
  instead of blurring the code"**; verbatim: **"It was decent, but sometimes blurring the code just
  made it harder to debug."** and **"I don't think I would use it. I mean, it could be pretty useful
  if it would markup code instead of blurring the rest."**
  [B-typeslicer-wizard-of-oz-user-study-2024]

### dont-assume-decades-of-research-implies-adoption
The base rate for this entire family of ideas is near-zero adoption.

- Dagstuhl "Beyond Program Slicing" (2005), 25 years post-Weiser:
  **"outside research circles, program slicing appears to be virtually unknown"**; and the seminar's
  own recurring question, **"if slicing is so great, why is nobody using it?"**
  Hardest datum: **"when asked, the seminar participants could name no existing debugger that
  includes slicing, though it was pointed out that for a number of years, a slicing patch against the
  GNU Debugger existed, but that this has now fallen by the wayside and is no longer maintained."**
  [B-making-slicing-mainstream-dagstuhl-2005]
- **"A recent survey by Binkley and Harman documents the current state of empirical evidence in
  favour of slicing, but does not offer unassailable evidence for the usefulness of slicing."**
  [B-making-slicing-mainstream-dagstuhl-2005]
- Parnin & Orso, independently: **"the sets of relevant statements identified are often still fairly
  large, and slicing-based debugging techniques are rarely used in practice."** And on the evidence
  base: **"only 3 out of 111 papers on slicing based debugging techniques have considered issues with
  the use of the techniques in practice."**
  [A-are-automated-debugging-techniques-helping-2011]
- Weiser and Lyle's own first controlled study of a slicing tool **"could not find any benefit"**;
  the follow-up only got a positive result after shrinking the program to **25 LOC** and replacing
  the interactive tool with a **paper printout**.
  [A-are-automated-debugging-techniques-helping-2011]

### dont-promise-minimality-you-cannot-compute
- Haack & Wells: **"Our algorithm quickly enumerates some minimal unsolvable subsets of a given
  constraint set and is then cut off by a time limit. Our algorithm is too expensive in practice for
  exhaustively enumerating all such sets"**, and the paper contains
  **"an example that in the worst case the number of minimal type error slices grows exponentially in
  the size of the program."**
  [A-type-error-slicing-haack-wells-2004]
- Soufflé measured the naive "store the full subproof per fact" encoding at **up to 100x memory**
  on a *transitive closure with 2000 tuples*, and rejected it outright.
  [A-datalog-proof-tree-heights-measured-2019]

### the-accuracy-ceiling-is-real
Even the best holistic localizer gets the single top pick wrong ~44% of the time.

- Seidel et al., 5,000+ ill-typed OCaml programs: top-1 exact-subexpression accuracy —
  **OCaml compiler 44%, SHErrLoc 56%, Nate 72%**; Nate reaches 85% at top-2 and 91% at top-3.
  [A-learning-to-blame-type-errors-2017]
- And ties are structural, not a tuning bug: **"recent techniques like SHErrLoc and Mycroft fail to
  distinguish between the [] and + expressions... Thus, these state-of-the-art techniques are forced
  to either blame both locations, or choose one arbitrarily."**
  [A-learning-to-blame-type-errors-2017]

### the-trust-gate-is-independent-of-correctness
- **"Developers were quick to disregard the tool if they felt they could not trust the results or
  understand how such results were computed."**
  [A-are-automated-debugging-techniques-helping-2011]
- Kochhar's non-adopters gave *rationale* as the reason: **"I doubt any automated software can explain
  the reason for things such as broken backwards compatibility, unclear documentation, what really
  should happen etc."** and **"Hairy bugs hide in interaction between various components and I don't
  think automated tools help much."** Note also the measured seniority gradient: a
  **statistically significant negative correlation between experience and rating fault localization
  "Essential"** (Spearman ρ = −0.14, p = 0.007; Fisher p = 0.014).
  [A-practitioners-expectations-fault-localization-2016]

Read for Dorc: your audience (annoyed *senior* engineer) is the demographic measured as *least*
receptive to this class of tool.

---

## (b) DO-DOS

### do-use-the-derivation-as-a-filter-not-as-output
The single best-supported positive finding in the whole vein.

- **"we have found that the most important contextual signal is whether or not the expression occurs
  in a minimal type error slice... We empirically demonstrate that the type error slice is so
  important that it is actually beneficial to automatically discard expressions that are not part of
  the slice, rather than letting the classifier learn to do so. Indeed, this domain-specific insight
  is crucial for learning classifiers that significantly outperform the state-of-the-art."**
  [A-learning-to-blame-type-errors-2017]

Compute the whole DAG. Ship the survivors. The DAG earns its keep as a pruning oracle.

### do-make-it-expandable-on-demand, one-level-at-a-time
Every shipped system converged on this, independently.

- Soufflé: default depth 4, `subproof <label>` to continue, and the design rationale is explicitly
  interaction-count minimisation — **"This interaction mechanism also justifies the choice to
  minimize the height of proof trees. By doing this, we minimize the number of user interactions
  (i.e., proof tree fragments) required to discover the root cause for an anomaly."**
  [A-datalog-proof-tree-heights-measured-2019] [B-souffle-explain-command-docs-2019]
- Prolog's declarative debugging: bisect by generalizing goals away, never print the derivation.
  [B-triska-declarative-prolog-debugging-2016]

### do-minimize-hard, and say which parts are certain
Alloy's UI marks core membership with two confidence levels in the *same* rendering:
**"A constraint highlighted with a bold shade of red is guaranteed to be a member of the core.
A constraint highlighted with a lighter shade of red is potentially, but not necessarily, part of
the core."** [B-alloy-unsat-core-quickguide-2021]

This is the closest prior art to Dorc's epistemic tier-words, and it is *two* levels, rendered as
intensity, inline — not six words in a column.

### do-carry-per-item-provenance-when-asked-for
Provenance is genuinely wanted — as a drill-down on a *specific* item, not as the primary view.

- **"many participants wanted some sort of explanation for the presence of a statement in the list
  and wanted to be able to trace a statement to its slices and related test cases."**
  [A-are-automated-debugging-techniques-helping-2011]
- Formal recommendation: **"Observation 2 - Providing overviews that cluster results and explanations
  that include data values, test case information, and information about slices could make faults
  easier to identify and tools ultimately more effective."**
  [A-are-automated-debugging-techniques-helping-2011]

### do-give-the-artifact-a-mental-model
The stated root cause of ranked-list failure is coherence, not size:
**"most ranking based automated debugging techniques remove any source of coherence by mixing
statements in a fashion that has no mental model to which the developer can relate. When using these
tools, instead of working with the familiar and reliable step-by-step approach of a traditional
debugger, developers are currently presented with a set of apparently disconnected statements and no
additional support."** [A-are-automated-debugging-techniques-helping-2011]

A DAG *is* a mental model — this is the strongest argument in favour of Dorc's chain over a flat list.
The cost is that it must actually read as a causal walk, not as a table of facts.

### do-inline-the-code, not just file:line
Barik's own recommendation, naming the reference implementation:
**"error presentation approaches such as those found in LLVM scan-build may prove beneficial to
developers. Unlike conventional error messages, which decouple the error message from the code
context, scan-build presents the error as a sequence of steps that the developer can follow alongside
in the context of the code to which the error message applies."**
[A-do-developers-read-error-messages-2017]

This is a direct endorsement of numbered-steps-with-inlined-code — the closest thing to a positive
result for Dorc's shape. Note it is a *recommendation*, not a measured outcome.

Corroborating mechanism from the same paper: reading an error message is as costly as reading source
(mean fixation 419ms vs 394ms for editor vs 275ms for English prose; KL divergence between error-
message and editor distributions is 0.059, vs 2.37 against silent reading). The stated cause is
modality-switching — **"error messages consist of both natural language ('is undefined for') and code
('Queue'), but are not entirely either. Consequently, developers must context switch between two
different modalities of reading."** Inlining the code removes one of the two switches.

### do-target-the-seven-criteria-the-field-converged-on
Yang et al.'s criteria, as restated by Wells' own group. A good report is:
**correct** (reports errors only for ill-typed code), **precise** (no more than the conflicting
portions), **succinct** (short reports), **non-mechanical** (no internal mechanical details),
**source-based** (only portions of source code), **unbiased** (no location privileged over others),
**comprehensive** (all conflicting portions). [C-rahli-wells-tes-challenges-talk-2009]

Note the built-in tension Dorc inherits: *succinct* vs *comprehensive*, and *unbiased* vs the ranking
that every measured-effective system does. The same deck flags that Haack & Wells' school deliberately
chose the burdensome end — highlighting even "the white spaces between a function and its arguments" —
where Chameleon "don't 'burden' the user."

### do-expect-the-slice-to-move-attention-even-when-it-does-not-narrow-it
TypeSlicer's real finding is not "slices help" but "slices *relocate* attention":
**"most participants without access to the tool proposed changing the implementation of thrice, which
is where the bug would immediately manifest. On the other hand, most participants with access to the
tool proposed changing the implementation of functions.x"** — the actual cause. Heat maps did *not*
always concentrate; they *shifted*. [B-typeslicer-wizard-of-oz-user-study-2024]

Caveat that must travel with this result, in the authors' own words:
**"Programs may be too small to expose limitations of slices... Further research is required to
evaluate whether slices scale to bigger programs and if there are empirical bounds to their
practicality"**, and **"The overwhelming positive feedback may be a consequence of participant bias."**

### do-watch-for-users-overriding-your-declared-facts
Relevant to tier-words specifically: users side with code over annotations.
**"it highlights that at least in some situations, programmers do side with the implementation details
in spite of type annotations and may assume the annotations in the program to be incorrect."**
[B-typeslicer-wizard-of-oz-user-study-2024]

A link tagged `vouched` or `claimed` will be *disbelieved* by a reader whose code-reading says
otherwise. Tier-words invite that fight rather than settling it.

---

## (d) LITERAL RENDERED ARTIFACTS

### Soufflé `explain` — an actual terminal proof tree
```
> explain path(1, 3)
         edge(2, 3)
         -------(R1)
edge(1, 2) path(2, 3)
-------------------(R2)
    path(1, 3)
```

Depth-capped, with an explicit continuation handle (this is the shipped default, depth 4):
```
> setdepth 3
Depth is now 3
> explain path(1, 4)
         edge(2, 3) subproof path(0)
         ------------------------(R2)
edge(1, 2)          path(2, 4)
------------------------------------(R2)
             path(1, 4)

> subproof path(0)
edge(3, 4)
-------(R1)
path(3, 4)
```

Negative explanation — note it is *interactive and user-guided*, because it cannot be automated:
```
> explainnegation path(1, 6)
1: path(x,y) :-
 edge(x,y).
2: path(x,z) :-
 edge(x,y),
 path(y,z).
Pick a rule number: 2
Pick a value for y: 2
====
edge(1, 2) ✓ path(2, 6) x
------------------------(R2)
       path(1,6)
```
First-party rationale: **"The approach here is required as it is not technically feasible to
automatically generate explanations for non-existence, and a bit of user guidance is required."**
[B-souffle-explain-command-docs-2019]

Directly relevant to Dorc: the ✓/x per-antecedent marking is a two-value epistemic tier applied
*inline at the join*, and it is the only tier vocabulary a shipped proof-tree renderer uses.

### Haack & Wells — a type error slice as elided source
Highlighted-in-place form (bold = in the slice):
```
val f = fn x => fn y => let val w = y + 1 in w::y end
```
Standalone elided form — a minimal program exhibiting exactly this error and nothing else:
```
type constructor clash, endpoints: int vs. list
(.. y => (.. y + (..) .. (..)::y ..) ..)
```
[A-type-error-slicing-haack-wells-2004]

`(..)` is the elision marker for irrelevant context. This is a *fourth* rendering strategy beyond
tree / list / highlight: reconstitute a minimal artifact that reproduces the finding. Worth
considering for Dorc — "here is the smallest script that still triggers this."

### Alloy — unsat core as source highlight, not as structure
No tree, no chain: red-shaded lines in the spec, bold-red = certainly in the core, light-red =
maybe. Rendered by clicking "Core" in the message panel.
[B-alloy-unsat-core-quickguide-2021]

### The counterexample renderings (what the field is escaping from)
OCaml, as-shipped, with the maintainer defending it as locally complete:
```
Error: This variant expression is expected to have type
         ('a, unit, string, node) format4
       There is no constructor :: within type format6
```
Rejected-as-noise elaboration (octachron's own strawman, then his own verdict "those hints only add
noise"):
```
Error: This variant expression is expected to have type
         ('a, unit, string, node) format4
       There is no constructor :: within type format6
Hint: In the current scope, only the list type defines a `(::)` constructor.
Hint: The format4 types defines the constructor:
  CamlinternalFormatBasics.Format
```
[C-ocaml-discuss-improve-type-errors-2023]

---

## (e) NEGATIVE RESULTS — veins searched, nothing found

- **No retrospective anywhere asking "why did type-error slicing never reach GHC or OCaml."**
  Searched the framing four ways (`why type error slicing not adopted GHC OCaml`,
  GHC-proposals, `SHErrLoc adoption`, OCaml discuss). No such document exists that I could find.
  The nearest thing is the *implicit* rejection inside SHErrLoc itself (SPJ is a co-author and the
  paper argues *against* reporting all contributing locations) and Haack & Wells' own closing
  paragraph. The absence is itself informative: the technique was not rejected in a documented
  debate, it simply never crossed into production and nobody wrote the post-mortem.
- **No user study found that measures a proof tree / unsat core against a control.** The Soufflé
  papers report runtime and memory only; the usability claim rests on an unquantified appeal to
  "several user experiences in industrial-scale applications" in a cited thesis I did not read.
  TypeSlicer is the only controlled-ish study of a derivation artifact and it is Wizard-of-Oz,
  n=29, formative, positive, and self-flagged for participant bias.
- **No measured slice-size distribution found.** Everyone asserts slices are "often large" (SHErrLoc
  twice, Parnin & Orso once) but nobody I read publishes the distribution. Haack & Wells prove
  worst-case exponential slice *count*, not slice *size*. This is a real gap: the size claim is
  universally repeated and nowhere measured in what I read.
- **Kochhar's RQ4/RQ5/RQ9 result tables not obtained.** I read the paper through §3.2/RQ1 only;
  Exa's PDF extraction returns from the head and cannot be offset. The "top-5" figure and the
  "must provide a rationale" agreement percentage are therefore carried on Seidel et al.'s
  characterisation, not on my own read of the tables. Flagged in the grading line.
- **Reddit practitioner sentiment: blocked, not gathered.** `old.reddit.com/robots.txt` disallows
  all autonomous fetching; I stopped rather than route around it. Kagi surfaced three highly
  on-point threads (r/cpp "Why are template errors so horrendously verbose?"; r/cpp_questions
  "Interpreting errors in C++" — snippet: *"find the first error... and ignore the rest... a very
  high noise to signal ratio"*; r/programming on Barik). **None are graded below — snippet only,
  not fully read.** If the human wants these, they need to be loaded manually.
- **Helium's retrospective not obtained.** Heeren/Hage's Helium is the one system in this family
  that genuinely shipped (as a teaching compiler). I located the IFL 2020 "Heuristics-based Type
  Error Diagnosis for Haskell" paper and Hage's Chalmers slides but read neither; budget went to
  the negative-evidence sources the brief prioritised. This is the largest remaining gap on the
  DO-DO side — it is the vein's only real success story.
- **ACM Digital Library is 403 for autonomous fetch.** The TOPLAS/TODS version of the Soufflé work
  (10.1145/3379446) was unreachable; I used the arXiv version instead, which carries the same
  experiments.
- **Dagstuhl and several .pdf hosts do not convert via `mcp__fetch__fetch`** (raw bytes returned).
  Exa's `web_fetch_exa` extracts PDF text and was the workaround throughout.
