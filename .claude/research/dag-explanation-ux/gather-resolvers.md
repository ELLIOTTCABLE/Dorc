# Vein: package-manager dependency explanation + resolver-error fatigue

Gathered by read-only subagent. Every source below was FULLY READ unless the grading-reasoning
says otherwise. Reddit could not be read (robots.txt Disallow: / on both `www.` and `old.`);
see NEGATIVE RESULTS.

---

## (a) DON'T-DOS

### dont-dump-the-raw-derivation-set
**Who/where:** jazzband/pip-tools#1821 (2023-02), user `mtesch`, filed against pip's new
backtracking resolver as surfaced through pip-tools 6.12.3.
**What failed:** the new (better) algorithm emitted the raw `ResolutionImpossible` payload —
a Python `repr` of the backtrack-cause list, ~30 near-identical `RequirementInformation(...)`
records, the same `apache_airflow-1.10.12` URL repeated 15 times. The OLD, algorithmically
weaker resolver produced a readable summary. Title verbatim: "Error message human unreadable
when using `--resolver=backtracking`". Suggested fix, verbatim: "dont use new resolver."
**Lesson for Dorc:** a linearized DAG with one line per node, un-deduplicated, is *worse than
a dumber tool's summary*. The upgrade in inference quality was experienced as a regression
purely on rendering.

### dont-let-a-single-fact-fan-out-into-n-lines
**Who/where:** astral-sh/uv#2519 (2024-03-18), filed by `konstin` — a uv core maintainer, i.e.
first-party self-criticism. Title: "No build error message very verbose".
**What failed:** PubGrub faithfully walked the derivation graph and produced 8+ lines of
`And because pytest-pep8==0.9 is unusable because no wheels are usable and building from source
is disabled and pytest-pep8==0.9.1 is unusable because no wheels are usable and building from
source is disabled, we can conclude that pytest-pep8<1.0 cannot be used.` The second example
(`superset`) is longer still and the reporter elides it with `[...many more cases]`.
**Lesson:** one *user-level* fact ("no wheels for this package, at all") got expanded into a
per-node chain because each node was individually true. Correct linearization != useful
linearization. Dorc's per-link tier-words + `file:line` + inlined excerpt multiply this cost by
~4 lines per link.

### dont-let-the-readability-transform-change-the-meaning
**Who/where:** pubgrub-rs#297 (2024-12-13), filed by `zanieb` (Zanie Blue, uv maintainer),
against the upstream algorithm. Title: "Missing term in derivation tree".
**What failed:** uv applied a merge/collapse transform to make the chain shorter. The shortened
output concluded `we can conclude that open3d<0.9.0.0 cannot be used` — which, in zanieb's
words, "doesn't make any sense here". `open3d==0.8.0.0` had silently fallen out of the terms.
Verbatim from the issue: *"Notice we create the incompatibility `open3d==0.8.0.0 no wheels with
a matching Python ABI tag` but it's never reflected in the terms — so it's missing from the
conclusion."* And: *"That transformation may be entirely unsound and PubGrub is working as
intended here."*
**Lesson:** the elision heuristics that make a DAG readable are a *second inference system*, and
it can be wrong while the underlying solver is right. For Dorc this is worse than for a package
manager: Dorc's chain is a provenance/epistemic claim, so a dropped join-node is a false
provenance claim, not just an ugly message.

### dont-recurse-unboundedly-over-the-derivation-graph-to-render-it
**Who/where:** pubgrub-rs#293 (2024-12-11), `x-hgg-x`, label `bug`, still open.
**What failed:** rendering — not solving — blows the stack. `// Stack overflow here` sits on the
line `let _out = DefaultStringReporter::report(&error).to_string();`. Reproduced from a real
cargo resolver test (`resolving_with_constrained_cousins_backtrack`, DEPTH 100, BRANCHING 50).
**Lesson:** the explanation renderer needs its own depth/size budget, independent of the
analysis. Real DAGs get deep enough to kill a naive DFS printer.

### dont-assume-back-references-are-cheap-to-cross-reference
**Who/where:** `cargo tree(1)`, official Cargo Book — first-party, and it is an *admission*.
**Verbatim:** *"If you're having difficulty cross-referencing the de-duplicated `(*)` entries,
try with the `--no-dedupe` flag to get the full output."*
**And the cost of the escape hatch,** from a practitioner (r/rust, Kagi snippet, UNGRADED):
*"I tried `--no-dedupe`, but it generates a really huge tree; I gave up after 100MB. :)"*
**Lesson:** this is exactly Dorc's `1a/1b -> 2` join-node design. Cargo shipped the de-dup
marker, then documented that users can't follow it, then offered explosion as the only fix.
Numbered back-references are a *known-hard* affordance in ASCII terminal output. If Dorc uses
them, it needs the third option Cargo lacks (see DO-DOS: dont-number-what-isnt-referenced).

### dont-expect-users-to-read-bottom-to-top
**Who/where:** Stack Overflow Q#64573177 (npm ERESOLVE), comment by `juanpaco`, 2023-03-24, on
the one answer that actually explains the tree.
**Verbatim:** *"I understood the concept of dependencies and all that, but you explained what the
error message was saying. I kept trying to read from top to bottom, so I was drawing completely
the wrong conclusions. I wish the error were more user friendly."*
The explanatory answer itself opens: *"First you should start to read the problem from the bottom
to the top."*
**Lesson:** npm's ERESOLVE block prints the *found* state first and the *requirement* last;
readers default to top-down and reach the wrong root cause. Dorc's numbered links (1, 2, 3 →
conclusion) must be readable in the direction the eye actually travels, and the conclusion must
not depend on having read the links in reverse.

### dont-ship-the-explanation-without-an-escape-hatch-and-expect-it-to-be-read
**Who/where:** SO Q#64573177 vote distribution, and pip's own UX survey.
- The top four npm answers by score are all "add the flag": 893 (`--force`/`--legacy-peer-deps`),
  400 (`--legacy-peer-deps`), 145 (`npm config set legacy-peer-deps true`), 74, 73, 64.
  The single answer that *decodes the tree* scores 388 — below two flag answers.
- Comment by `holengzai` (2022-04-12), verbatim: *"That's the best answer because it is the only
  answer which explained the error log from OP instead of stupidly applying force or
  legacy-peer-deps option."*
- Comment by `umagon` (2024-12-12), verbatim: *"I hate how this npm error is so hard to
  understand. It would have been so much better something like 'this package expects this version
  and you have this one' and after this, all the details..."*
- pip's own survey (n=415, first-party): *">70% of respondents indicated that they want some kind
  of override that allows them to install packages when there are dependency conflicts"*, and
  *"most respondents said if it exists they would use it 'not often'"*.
**Lesson:** the modal response to a correct, complete dependency explanation is to bypass it.
This is the strongest single finding in this vein. It does NOT mean don't explain — it means the
explanation's success metric is "did the annoyed engineer act correctly", not "was it complete".

### dont-let-internal-state-drive-the-wording
**Who/where:** pypa/pip#8377, `pfmoore` (pip maintainer), 2020-06-17.
**Verbatim:** *"This is one of the (few 🙂) advantages of writing messages in terms of the internal
state of pip - we can at least be sure that we're accurately describing the situation, even if
it's hard to do so in a way that is informative to the user."*
And 2020-06-04: *"Unfortunately, the exception that we get in the code doesn't include enough
information to answer the questions 'what else did you try?' or 'what was the requirement I typed
on the command line that caused you to attempt ward 0.44.1b0?'."*
**Lesson:** accuracy-to-solver-state and informativeness-to-human are in direct tension, and the
solver's data structure will not naturally carry the fields the human wants. Dorc should design
the *link record* (what a link must carry) from the question the engineer asks, not from what the
analyzer happens to have on hand.

### dont-emit-nodes-the-reader-cannot-situate
**Who/where:** astral-sh/uv#12511 (2025-03-27), `notatallshaw` — a pip *and* uv collaborator,
i.e. a domain expert, defeated by a 4-line PubGrub chain.
**Verbatim:** *"Where did 'python-nvd3<=0.15.0' and 'python-nvd3==0.16.0' come from? And what are
they trying to tell me?"* His "Expected output" deletes the version enumeration entirely,
keeping only the two lines that carry the actual cause.
**Lesson:** every rendered link must be traceable to something the reader supplied or can see.
An intermediate derived term that exists only because the algorithm needed it is noise even to
experts.

### dont-add-fix-suggestions-you-cannot-scope
**Who/where:** pypa/pip#8495, `di` (Dustin Ingram, PyPA), 2020-07-30.
**Verbatim:** *"if I am not a maintainer/owner of that package, and I don't know much about
dependency specifications, the two proposed fixes would make little to no sense to me. Even if I
did know something about what that means, it would be entirely out of my control."*
And `pradyunsg`, 2020-08-26: *"I'm wondering if what we should do here is drop the suggestions for
how to fix this from the printed message... non-expert users would not be presented with
non-actionable suggestions without additional context."*
Also pypa/pip#8377 records the team's reaction to iteration 1: *"providing 3. (displays possible
ways to solve the error) is too verbose."*
**Lesson:** suggestions must be gated on whether the named thing is in the reader's control.
Dorc's tier-words (`measured`/`vouched`/`consented`) already encode who owns a link — that
ownership should gate what advice is printed, and links owned by nobody the reader can reach
should probably not generate advice at all.

### dont-repeat-the-same-name-on-both-sides-of-a-join
**Who/where:** pypa/pip#8495, `uranusjr` (pip maintainer), title: "New resolver error message is
confusing if a package has inconsistent dependencies". Shipped output read
`ERROR: Cannot install testpkg 1 and testpkg because...`.
**Verbatim (uranusjr, filing):** *"`testpkg` shows up twice in the first `ERROR:` message, which
looks confusing. It is 'correct' that `testpkg` is conflicting with itself, but we probably need
to produce a message that makes more sense to humans."*
**Lesson:** self-joins in the DAG render as tautologies. `pfmoore` hits the same wall separately:
*"I'm a bit concerned that it's stating the obvious ('Cannot install click 5 and click 7 because
the user requested click 5 and click 7')."* Dorc's join-nodes will produce these whenever two
branches trace back to the same file:line.

### dont-narrate-progress-you-cannot-bound
**Who/where:** pip's own official docs, `topics/dependency-resolution`.
**Verbatim message:** `INFO: pip is looking at multiple versions of this package to determine
which version is compatible with other requirements. This could take a while.` — printed
repeatedly, interleaved with dozens of `Downloading cup-3.1x.0` lines.
Doc's own admission: *"If pip starts backtracking during dependency resolution, it does not know
how many choices it will reconsider, and how much computation would be needed."*
**Lesson:** an honest "this may take a while" with no bound, repeated, reads as a hang. This is
relevant to Dorc as the network-phase analogue.

---

## (b) DO-DOS

### number-only-what-gets-referenced-later
**Source:** dart-lang/pub `doc/solver.md`, Error Reporting section. Canonical PubGrub spec.
**Verbatim:** *"We use line numbers to do this, but we only number incompatibilities that we
**know** will need to be referred to later on. In the simple linear case, we don't include line
numbers at all."*
Mechanism, verbatim: *"Before running the error reporting algorithm proper, walk the derivation
graph and record how many outgoing edges each derived incompatibility has–that is, how many
different incompatibilities it causes."* Then, terminating rule: *"Finally, if `incompatibility`
causes two or more incompatibilities, give the line that was just written a line number."*
**Directly answers Dorc's design question.** Numbering is not a property of a link; it's a
property of *out-degree > 1* in the DAG. A pure chain gets zero numbers. Dorc's proposed
"numbered links 1,2,3 with join-nodes 1a/1b" numbers everything up-front — PubGrub's spec says
number nothing until you've proven you need a back-reference.

### fold-away-every-other-derived-node
**Source:** same, dart-lang/pub `doc/solver.md`.
**Verbatim:** *"The only nuance is that, in practice, this tends to end up a little verbose. You
can skip every other derived incompatibility without losing clarity."*
Worked example — instead of:
> *"... And, because `root` depends on `foo ^1.0.0`, `root` requires `baz ^3.0.0`. So, because
> `root` depends on `baz ^1.0.0`, `root` isn't valid and version solving has failed."*

emit:
> *"... So, because `root` depends on both `foo ^1.0.0` and `baz ^3.0.0`, `root` isn't valid and
> version solving has failed."*

**Lesson:** a derived node whose only job is to be consumed by the next derived node is not worth
a line. Dorc's tier-word-per-link design fights this: it gives every link a reason to exist.

### collapse-node-classes-before-rendering
**Source:** pubgrub-rs guide, "Solution and error reporting" (first-party crate docs).
`derivation_tree.collapse_no_versions()` is called *before* `DefaultStringReporter::report`.
**Verbatim, before:**
```
Because there is no version of foo in 1.0.1 <= v < 2.0.0
and foo 1.0.0 depends on bar 2.0.0 <= v < 3.0.0,
foo 1.0.0 <= v < 2.0.0 depends on bar 2.0.0 <= v < 3.0.0.
...
```
**after:**
```
Because foo 1.0.0 <= v < 2.0.0 depends on bar 2.0.0,
...
```
**Lesson:** ship named, opt-in graph-simplification passes as part of the reporting API, not as
ad-hoc string munging. (But see dont-let-the-readability-transform-change-the-meaning — uv's
own collapse pass was unsound. These passes need tests at the *tree* level.)

### classify-before-you-narrate
**Source:** pubgrub-rs#64, `mpizenberg` (pubgrub-rs maintainer), 2020-11-16, milestone
"v0.5 error reporting", still open five years later.
**Verbatim:** *"I've classified those errors in different categories and I think it would be
really nice if the error reporter was able to give one such class of high level problem before
diving into the detailed tree. Here are some examples of potential categories: Direct/Indirect
dependency on a non-existing package; Indirect dependency on two incompatible versions of the
same package; Indirect dependency on another version of itself."*
**Lesson:** a one-line *shape* classification ahead of the chain lets the annoyed engineer decide
whether to read the chain at all. Corroborated by the practitioner ask in umagon's SO comment
("something like 'this package expects this version and you have this one' and after this, all
the details"). Dorc has a natural taxonomy available: the tier-word of the *weakest* link in the
chain.

### collapse-to-the-two-conflicting-leaves-and-then-offer-actions
**Source:** pip's shipped format, official docs `topics/dependency-resolution` — this is what
survived the funded 2020 UX programme, having started as a full-chain proposal.
```
ERROR: Cannot install package_coffee==0.44.1 and package_tea==4.3.0 because these package
versions have conflicting dependencies.

The conflict is caused by:
    package_coffee 0.44.1 depends on package_water<3.0.0,>=2.4.2
    package_tea 4.3.0 depends on package_water==2.3.1

To fix this you could try to:
1. loosen the range of package versions you've specified
2. remove package versions to allow pip attempt to solve the dependency conflict
```
**Note what pip did NOT ship:** `nlhkabu`'s intermediate proposal in pypa/pip#8495 had an explicit
indented `Dependency tree:` block. It was dropped. The recorded reason (nlhkabu, 2020-10-22),
verbatim: *"It is not currently possible to display the dependency tree, but pip does know the
parent of the conflicting package (or if no parent exists)"* — and a separate issue (#9036) was
opened for the tree. As of the current docs, the tree is still absent.
**Lesson:** the structure that shipped is `<one-line conclusion> / <exactly the two leaf facts> /
<numbered ACTIONS>`. The numbered list is actions, not derivations. That is a deliberate
inversion of the PubGrub presentation and it is what a funded UX team converged on.

### make-the-explanation-a-separate-on-demand-command
**Source:** `npm explain` (alias `npm why`) official docs; `cargo tree --invert`; `aptitude why`.
Three ecosystems independently split "here is the failure" from "here is the full chain", and put
the chain behind a second, explicitly-invoked command. `npm explain glob` verbatim output:
```
glob@7.1.6
node_modules/glob
  glob@"^7.1.4" from the root project
glob@7.1.1 dev
node_modules/tacks/node_modules/glob
  glob@"^7.0.5" from rimraf@2.6.2
  node_modules/tacks/node_modules/rimraf
    rimraf@"^2.6.2" from tacks@1.3.0
    node_modules/tacks
      dev tacks@"^1.3.0" from the root project
```
`aptitude why nautilus-data` verbatim: `i   unity-settings-daemon Depends nautilus-data (>= 2.91.3-1)`
— one line, with a status column. `aptitude` also ships `--show-summary`, documented verbatim as:
*"Changes the behavior of 'aptitude why' to summarize each dependency chain that it outputs,
rather than displaying it in long form."*
**Lesson:** the strongest convergent evidence in this vein. Dorc should consider whether the
provenance chain belongs in the failure output at all, or behind `dorc why <thing>` — with the
failure output carrying a *pointer* to the command that would print the chain.

### offer-a-linearization-mode-not-only-a-tree-mode
**Source:** `cargo tree(1)`, `--prefix` option. Values: `indent` (default), `depth` ("Show as a
list, with the numeric depth printed before each entry"), `none` ("Show as a flat list").
Practitioner use, corroborating (r/rust, Kagi snippet, UNGRADED): pipeline
`cargo tree -e features --prefix none | sort | uniq > deps` then diff — i.e. the flat mode exists
because people need to *diff and grep* the graph, not read it.
**Lesson:** Dorc's ASCII linearization has a second consumer — grep/diff/CI. Design the numbered
form so a line is self-contained enough to survive `grep`.

### state-the-reading-direction-or-remove-the-need-for-one
**Source:** SO#64573177's top *explanatory* answer (388 votes) exists solely to say
*"First you should start to read the problem from the bottom to the top."* npm never says this
in the error itself.
**Lesson:** cheapest possible fix, never applied by the tool.

### drop-ceremonial-connective-tissue
**Source:** astral-sh/uv#14115 (2025-06-17), filed by `nedbat` (Ned Batchelder), labelled
`error messages`, open.
**Verbatim:** *"To my ear, this message seems a bit stuffy: 'Because flash-attn was not found in
the package registry and you require flash-attn, we can conclude that your requirements are
unsatisfiable.' I think the phrase 'we can conclude that' can be dropped."*
**Lesson:** PubGrub's prose scaffolding (`Because... And because... So, because... we can conclude
that...`) is per-line overhead that compounds across a chain. Dorc's tier-words are the same
category of per-line token; budget them.

---

## (c) GRADED SOURCE TABLE

```
{"slug":"A-pubgrub-next-generation-version-solving-2018","url":"https://nex3.medium.com/pubgrub-2fb6470504f","grading-certainty":"+1:SURE","grading-reasoning":"A not B: this is the algorithm's originating description by its author (Natalie Weizenbaum, pub maintainer), not a practitioner writeup about it; it is the canonical citation every downstream implementation (uv, pubgrub-rs, poetry) points back to, and it contains the primary worked error-report example verbatim. Read end-to-end including the failure-reporting section.","relevance-certainty":"+1:SURE","relevance-description":"Directly describes linearizing a derivation DAG into plain English for humans, names the DFS traversal, the folding heuristic, and the line-numbering fallback for non-linear graphs. This is Dorc's proposed rendering, ten years earlier.","graded-by":"subagent","published":"2018-04-02","via":"mcp__fetch__fetch url=https://nex3.medium.com/pubgrub-2fb6470504f max_length=30000"}
{"slug":"A-pubgrub-solver-specification-error-reporting-2018","url":"https://raw.githubusercontent.com/dart-lang/pub/master/doc/solver.md","grading-certainty":"+1:SURE","grading-reasoning":"A not B: this is the normative specification the blog post defers to, first-party in dart-lang/pub, and it gives the error-reporting algorithm as executable pseudocode with the exact line-numbering rule plus two fully worked renderings (Linear and Branching). Primary, not secondary; no rot (still master). Read in two passes covering the whole document.","relevance-certainty":"+1:SURE","relevance-description":"The single most on-point artifact for Dorc: it specifies when to number a line (out-degree >= 2 only), when to fold, when to blank-line-separate branches, and shows the literal ASCII output for both a chain and a join. Also warns that naively-linear explanation fails on branching graphs.","graded-by":"subagent","published":"2018","via":"mcp__fetch__fetch url=.../dart-lang/pub/master/doc/solver.md max_length=40000 then start_index=40000"}
{"slug":"B-backtracking-resolver-human-unreadable-error-2023","url":"https://github.com/jazzband/pip-tools/issues/1821","grading-certainty":"+1:SURE","grading-reasoning":"B not A: a first-party bug report with complete verbatim before/after output, but a single user's report rather than a spec or study. B not C: the evidence is the raw artifact itself (the actual unreadable message and the actual readable predecessor), which is primary and non-reconstructible.","relevance-certainty":"+1:SURE","relevance-description":"Cleanest documented case of an un-deduplicated derivation dump being strictly worse than a dumber tool's summary; the same node repeats ~15 times. Direct disproof of 'more complete provenance is better'.","graded-by":"subagent","published":"2023-02","via":"mcp__fetch__fetch url=https://github.com/jazzband/pip-tools/issues/1821 max_length=25000"}
{"slug":"B-pip-resolution-impossible-message-design-2020","url":"https://github.com/pypa/pip/issues/8377","grading-certainty":"+1:SURE","grading-reasoning":"B not A: first-party design thread from the funded pip UX programme including a reported usability test, but the test's protocol and n are not published here so it is not gradable as research. B not C: authored by pip maintainers (pfmoore) and the UX contractor (ei8fdb) with five verbatim message iterations. Body plus all 11 comments read.","relevance-certainty":"+1:SURE","relevance-description":"Shows a design converging away from chain-rendering toward two-leaf-plus-actions, and records the maintainer's explicit statement of the accuracy-vs-informativeness tension and the 'too verbose' rejection of fix-suggestions.","graded-by":"subagent","published":"2020-06","via":"mcp__github__issue_read method=get + get_comments owner=pypa repo=pip issue_number=8377"}
{"slug":"B-conflicting-message-when-package-conflicts-itself-2020","url":"https://github.com/pypa/pip/issues/8495","grading-certainty":"+1:SURE","grading-reasoning":"B not C: filed and argued by four pip maintainers (uranusjr, pfmoore, di, pradyunsg) plus the UX contractor, with the abandoned 'Dependency tree:' proposal preserved verbatim. B not A: it is deliberation, not a finding. Body plus all 22 comments read.","relevance-certainty":"+1:SURE","relevance-description":"Contains the explicit decision NOT to print the dependency tree in the error, the self-join tautology problem, and di's argument that advice must be gated on reader control.","graded-by":"subagent","published":"2020-06-25","via":"mcp__github__issue_read method=get + get_comments owner=pypa repo=pip issue_number=8495"}
{"slug":"B-pip-dependency-resolution-shipped-error-format-2025","url":"https://pip.pypa.io/en/stable/topics/dependency-resolution/","grading-certainty":"+1:SURE","grading-reasoning":"B not A: official first-party documentation of what actually shipped, which is stronger evidence of the endpoint than any issue thread, but it is documentation rather than a study. No rot: current stable docs, includes ResolutionTooDeep added well after 2020.","relevance-certainty":"+1:SURE","relevance-description":"The surviving format after a funded UX programme: one-line conclusion, exactly two leaf facts, numbered ACTIONS (not derivations). Also carries the verbatim unbounded-progress message and pip's admission it cannot bound the work.","graded-by":"subagent","published":"2025","via":"mcp__fetch__fetch url=https://pip.pypa.io/en/stable/topics/dependency-resolution/ max_length=20000"}
{"slug":"B-pip-users-mental-models-study-2020","url":"https://pip.pypa.io/en/latest/ux-research-design/research-results/mental-models/","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not A: a real coded study (48 interviews, scored against a maintainer-derived 18/13/10-aspect rubric) but published as project documentation without peer review, and the scoring rubric itself is not reproduced. B not C: first-party, funded, methodology stated, raw data linked. SUSPECT because I could not check the linked spreadsheet.","relevance-certainty":"+1:SURE","relevance-description":"Quantifies audience model-depth: >90% of participants lacked a deep understanding of the tool, median 3-of-13 aspects on the install process. Its own recommendation asks 'Is it actually necessary for users to know everything that pip is doing?' — the exact question Dorc's DAG rendering presupposes an answer to.","graded-by":"subagent","published":"2020","via":"mcp__fetch__fetch url=https://pip.pypa.io/en/latest/ux-research-design/research-results/mental-models/ max_length=20000"}
{"slug":"B-override-conflicting-dependencies-survey-2020","url":"https://pip.pypa.io/en/latest/ux-research-design/research-results/override-conflicting-dependencies/","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: first-party survey with a stated n (415), the prompt reproduced verbatim, and stated question set. B not A: self-selected respondents from a recruited panel, no significance testing, results given as '>70%' bands. SUSPECT on whether the band figures are robust.","relevance-certainty":"+1:SURE","relevance-description":"Direct measurement of demand for the escape hatch over the explanation: >70% want an override, and most say they would use it 'not often' — i.e. the bypass is an emergency valve people insist on having even though the explanation is what they mostly need.","graded-by":"subagent","published":"2020","via":"mcp__fetch__fetch url=https://pip.pypa.io/en/latest/ux-research-design/research-results/override-conflicting-dependencies/ max_length=20000"}
{"slug":"B-improving-pip-documentation-research-2020","url":"https://pip.pypa.io/en/latest/ux-research-design/research-results/improving-pips-documentation/","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: first-party research with stated n per instrument (5 interviews, 141 + 159 survey responses) and multi-method (interviews, surveys, keyword research). B not A: no peer review, small interview n, and it self-reports two methods that failed outright (diary study abandoned, feedback widget yielded nothing).","relevance-certainty":"-0:SUSPECT","relevance-description":"Establishes the escape route users actually take when a CLI message does not suffice: 81.9% Google it, 56.9% Stack Overflow, 25.6% read the docs. Also records the finding that 'improving pip's error messages would reduce the need for better documentation'. Relevant as behaviour-around-the-message rather than about DAG rendering itself.","graded-by":"subagent","published":"2020","via":"mcp__fetch__fetch url=https://pip.pypa.io/en/latest/ux-research-design/research-results/improving-pips-documentation/ max_length=20000"}
{"slug":"B-pip-ux-resolution-impossible-prototype-2020","url":"https://pip.pypa.io/en/latest/ux-research-design/resolution-impossible-example/","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: first-party UX design artifact hosted in the official docs, showing the team's intended message before implementation constraints bit. Marked down from A and flagged SUSPECT because the page is visibly incomplete (three '-vv / -vvv: what would they see?' sections left unanswered) — partial rot / abandoned artifact.","relevance-certainty":"+1:SURE","relevance-description":"Shows the aspirational design: two bullet facts, then a numbered list of five ACTIONS, then a pointer to a separate tree command. Confirms the numbered-list slot was reserved for remedies, never for derivation steps.","graded-by":"subagent","published":"2020","via":"mcp__fetch__fetch url=https://pip.pypa.io/en/latest/ux-research-design/resolution-impossible-example/ max_length=25000"}
{"slug":"B-uv-resolver-internals-official-documentation-2024","url":"https://docs.astral.sh/uv/reference/internals/resolver/","grading-certainty":"+1:SURE","grading-reasoning":"B not C: official first-party internals documentation by the implementing team, describing design intent in their own words. B not A: it is a design narrative, not a spec or a study, and it does not give error-message examples.","relevance-certainty":"-0:SUSPECT","relevance-description":"Carries the team's explicit statement of the requirement ('the resolver should produce an understandable error trace that states which packages are involved in a way that allows a user to remove the conflict') — note the success criterion is removability of the conflict, not comprehension of the proof. Most of the page is resolution mechanics irrelevant to rendering.","graded-by":"subagent","published":"2024-08-08","via":"mcp__fetch__fetch url=https://docs.astral.sh/uv/reference/internals/resolver/ max_length=30000"}
{"slug":"B-uv-no-build-message-very-verbose-2024","url":"https://github.com/astral-sh/uv/issues/2519","grading-certainty":"+1:SURE","grading-reasoning":"B not C: filed by a uv core maintainer (konstin) against his own project, with two complete verbatim renderings; self-criticism from the implementer is stronger provenance than a user complaint. B not A: single case, no measurement.","relevance-certainty":"+1:SURE","relevance-description":"The canonical fan-out failure: one user-level fact rendered as 8-40 near-identical derivation lines because each node is individually true. Exactly the risk in Dorc's per-link tier-word + file:line + excerpt format.","graded-by":"subagent","published":"2024-03-18","via":"mcp__github__issue_read method=get owner=astral-sh repo=uv issue_number=2519"}
{"slug":"B-uv-extra-requirements-are-confusing-2025","url":"https://github.com/astral-sh/uv/issues/12511","grading-certainty":"+1:SURE","grading-reasoning":"B not C: reporter (notatallshaw) is a pip and uv collaborator, i.e. a domain expert, which makes the failure far more probative than a novice's confusion. Includes verbatim actual and desired output. B not A: n=1.","relevance-certainty":"+1:SURE","relevance-description":"An expert cannot situate two intermediate nodes in a four-line chain and asks where they came from; his fix deletes them. Evidence that intermediate derived terms are noise even to the most capable readers.","graded-by":"subagent","published":"2025-03-27","via":"mcp__github__issue_read method=get owner=astral-sh repo=uv issue_number=12511"}
{"slug":"B-missing-term-in-derivation-tree-2024","url":"https://github.com/pubgrub-rs/pubgrub/issues/297","grading-certainty":"+1:SURE","grading-reasoning":"B not A: an upstream bug report, not a paper, but authored by uv's error-message owner (zanieb) with three verbatim renderings of the same failure at three transform levels plus the debug-printer source. Unusually high read-depth for an issue; effectively a primary case study of a readability transform going unsound.","relevance-certainty":"+1:SURE","relevance-description":"The highest-value negative result in this vein: the collapse pass that makes the chain readable produced a semantically wrong conclusion. Cautions that Dorc's elision heuristics are a second inference system with its own correctness burden.","graded-by":"subagent","published":"2024-12-13","via":"mcp__github__search_issues owner=pubgrub-rs repo=pubgrub query=\"error message readability derivation tree hard to understand report\""}
{"slug":"B-stack-overflow-displaying-derivation-tree-2024","url":"https://github.com/pubgrub-rs/pubgrub/issues/293","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: reproducible minimal test case with the crashing line marked, labelled bug by maintainers, still open. SUSPECT on grade because it is a narrow implementation defect rather than a design finding, and one could argue C; kept at B because the repro is imported from cargo's own resolver test suite, so the depth is realistic rather than synthetic.","relevance-certainty":"-0:SUSPECT","relevance-description":"Establishes that real derivation graphs get deep enough that RENDERING them (not solving) exhausts the stack. Argues for an explicit budget on the explanation renderer. Relevance is bounded — Dorc's DAGs may never get that deep.","graded-by":"subagent","published":"2024-12-11","via":"mcp__github__search_issues owner=pubgrub-rs repo=pubgrub query=\"error message readability derivation tree hard to understand report\""}
{"slug":"B-error-reporting-summaries-before-detail-2020","url":"https://github.com/pubgrub-rs/pubgrub/issues/64","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: opened by the pubgrub-rs maintainer (mpizenberg) from a real corpus sweep (all elm packages), and milestoned. SUSPECT because I read only the body, not its single comment, and because the proposal was never implemented — five years open, which is itself evidence but weakens it as a 'do-do'.","relevance-certainty":"+1:SURE","relevance-description":"Names the classify-before-narrate pattern and derives the taxonomy empirically from a whole-ecosystem sweep. Directly transferable: Dorc could classify by weakest tier-word in the chain.","graded-by":"subagent","published":"2020-11-16","via":"mcp__github__search_issues owner=pubgrub-rs repo=pubgrub query=\"error message readability derivation tree hard to understand report\""}
{"slug":"B-pubgrub-guide-collapse-no-versions-2024","url":"https://pubgrub-rs-guide.pages.dev/pubgrub_crate/solution","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: first-party guide for the reference Rust implementation, with verbatim before/after output for a named simplification pass. SUSPECT because it is a short page on a pages.dev host and I could not verify it tracks the current crate version.","relevance-certainty":"+1:SURE","relevance-description":"Shows graph-simplification shipped as a named API call invoked before rendering, with the concrete win. The pattern Dorc should copy; pair with issue 297 for the correctness caveat.","graded-by":"subagent","published":"2024-05-27","via":"mcp__fetch__fetch url=https://pubgrub-rs-guide.pages.dev/pubgrub_crate/solution max_length=15000"}
{"slug":"B-cargo-tree-dedupe-cross-reference-difficulty-2024","url":"https://doc.rust-lang.org/cargo/commands/cargo-tree.html","grading-certainty":"+1:SURE","grading-reasoning":"B not C: canonical first-party Cargo Book manual page, current. B not A: reference documentation rather than a spec of an algorithm or a study. The deciding factor is that the load-bearing sentence is a first-party admission of a UX failure inside official docs, which is rare and strong.","relevance-certainty":"+1:SURE","relevance-description":"The (*) de-dup marker is the exact analogue of Dorc's join-node back-reference, and Cargo documents that users cannot follow it, offering only full expansion as the fix. Also supplies --prefix depth/none as linearization precedent and --invert/-d as the separate why-command pattern.","graded-by":"subagent","published":"2024","via":"mcp__fetch__fetch url=https://doc.rust-lang.org/cargo/commands/cargo-tree.html max_length=18000"}
{"slug":"B-npm-explain-chain-of-dependencies-2024","url":"https://docs.npmjs.com/cli/v11/commands/npm-explain","grading-certainty":"+1:SURE","grading-reasoning":"B not C: official npm CLI v11 reference, first-party and current. Not A because it is command reference with no design rationale.","relevance-certainty":"-0:SUSPECT","relevance-description":"Primary evidence that npm split explanation into a separate on-demand verb (explain / why) rather than inlining chains into failures, plus the verbatim indented chain format. Relevance is structural rather than about failure rendering.","graded-by":"subagent","published":"2024","via":"mcp__fetch__fetch url=https://docs.npmjs.com/cli/v11/commands/npm-explain max_length=10000"}
{"slug":"B-npm-v7-beta-legacy-peer-deps-rationale-2020","url":"https://blog.npmjs.org/post/626173315965468672/npm-v7-series-beta-release-and-semver-major.html","grading-certainty":"+1:SURE","grading-reasoning":"B not C: the npm team's own release announcement, the primary source for why --legacy-peer-deps exists and how they expected it to be used. Not A: announcement prose, no data.","relevance-certainty":"-0:SUSPECT","relevance-description":"Establishes intent behind the escape hatch verbatim: 'It may be that the disruption is too great to take all at once, and we have to have this flag enabled by default for a while.' The designers anticipated the bypass. Indirect relevance to DAG rendering, direct relevance to explanation-vs-escape-hatch.","graded-by":"subagent","published":"2020-08-11","via":"mcp__fetch__fetch url=https://blog.npmjs.org/post/626173315965468672/... max_length=20000"}
{"slug":"C-npm-eresolve-unable-to-resolve-tree-2020","url":"https://stackoverflow.com/questions/64573177/unable-to-resolve-dependency-tree-error-when-installing-npm-packages","grading-certainty":"+1:SURE","grading-reasoning":"C not B: high-signal social with quantified attention (vote counts across competing answers act as a crude preference measurement over ~5 years), but every claim in it is a practitioner's, none first-party to npm, and answer quality is uneven. NOT graded higher despite volume. Read-depth caveat: fetch truncated at 20000 chars, so I read the question plus the top ~10 answers and their comments, not the full page — the graded claims are all from within the portion read.","relevance-certainty":"+1:SURE","relevance-description":"Best available quantification of skip-the-explanation behaviour: flag answers outrank the explaining answer, and two comments state verbatim that the reading direction defeated them and that the message should lead with the one-line conflict. Supplies the verbatim ERESOLVE block.","graded-by":"subagent","published":"2020-10-30","via":"mcp__fetch__fetch url=https://stackoverflow.com/questions/64573177/... max_length=20000"}
{"slug":"C-aptitude-why-and-why-not-explained-2016","url":"https://askubuntu.com/questions/774513/what-does-the-aptitude-why-and-why-not-mean","grading-certainty":"-0:SUSPECT","grading-reasoning":"C not B: a competent secondary explanation with real terminal output, but not first-party Debian documentation and a decade old; the reader should treat the command semantics as possibly drifted. C not D because the verbatim outputs are genuine and reproducible.","relevance-certainty":"-0:SUSPECT","relevance-description":"Shows the minimal end of the design space — 'aptitude why' answers with ONE line carrying a status column, and answers 'Unable to find a reason to install X' rather than a proof. Useful as the extreme contrast to a numbered DAG, but ancient and thin.","graded-by":"subagent","published":"2016-05-19","via":"mcp__fetch__fetch url=https://askubuntu.com/questions/774513/... max_length=15000"}
{"slug":"C-pip-ux-research-results-index-2020","url":"https://pip.pypa.io/en/latest/ux-research-design/research-results/","grading-certainty":"+1:SURE","grading-reasoning":"C not B: first-party but it is an index page; its value is the inventory of studies and their response counts, not findings. Graded on function, not on host.","relevance-certainty":"-1:GUESS","relevance-description":"Establishes the scale and funding of the 2020 pip UX programme (48 interviews, 10 surveys, 472-person panel, 459 resolver-feedback responses) which is what licenses treating the pip artifacts above as evidence rather than opinion. No direct rendering content.","graded-by":"subagent","published":"2020","via":"mcp__fetch__fetch url=https://pip.pypa.io/en/latest/ux-research-design/research-results/ max_length=8000"}
{"slug":"C-uv-simplify-conclusion-messages-2025","url":"https://github.com/astral-sh/uv/issues/14115","grading-certainty":"+1:SURE","grading-reasoning":"C not B: a single-paragraph wording nit, albeit from a well-known practitioner (nedbat) and labelled 'error messages' by maintainers. Too thin to be B; not D because it is first-party to the tracker and still open.","relevance-certainty":"-0:SUSPECT","relevance-description":"Evidence that PubGrub's prose connectives are perceived as overhead per line. Minor but confirms that per-link ceremonial tokens (Dorc's tier-words) have a cost that compounds.","graded-by":"subagent","published":"2025-06-17","via":"mcp__github__issue_read method=get owner=astral-sh repo=uv issue_number=14115"}
{"slug":"C-uv-no-solution-python-version-marker-2024","url":"https://github.com/astral-sh/uv/issues/8601","grading-certainty":"+1:SURE","grading-reasoning":"C not B: a user bug report closed as duplicate/not-planned; its value is one verbatim rendering, nothing more. Not D because the rendering is genuine first-party output.","relevance-certainty":"-1:GUESS","relevance-description":"Supplies a verbatim in-the-wild PubGrub message and the reporter's flat statement \"I don't understand uv's logic\" after reading a complete, correct four-step chain. Weak as evidence (n=1, marker semantics confound it) but on-theme.","graded-by":"subagent","published":"2024-10-27","via":"mcp__github__issue_read method=get owner=astral-sh repo=uv issue_number=8601"}
{"slug":"C-uv-confusing-resolution-error-collector-2023","url":"https://github.com/astral-sh/uv/issues/309","grading-certainty":"+1:SURE","grading-reasoning":"C not B: a four-line stub whose entire content is four upstream links. Its only evidential value is that uv opened a dedicated collector for confusing resolution messages within days of the project going public, and closed it once the upstream issues were filed.","relevance-certainty":"-1:GUESS","relevance-description":"Weak on its own; useful only as the provenance trail to pubgrub-rs 149-152 and as evidence that PubGrub message quality was a day-one problem for a well-resourced team.","graded-by":"subagent","published":"2023-11-03","via":"mcp__github__issue_read method=get owner=astral-sh repo=uv issue_number=309"}
{"slug":"C-pubgrub-root-package-version-in-messages-2023","url":"https://github.com/pubgrub-rs/pubgrub/issues/150","grading-certainty":"+1:SURE","grading-reasoning":"C not B: narrow API-ergonomics issue, and its evidential content is a single leaked-internal-detail example. Kept above D because it is first-party from uv's maintainer against the reference implementation.","relevance-certainty":"-0:SUSPECT","relevance-description":"Small but transferable: a synthetic root node's dummy version leaked into user-facing prose ('root 0.0.0 depends on foo 1.0.0'), and escaping it required reimplementing the whole Reporter. Warns that Dorc's synthetic/entry nodes need a rendering opt-out designed in, not bolted on.","graded-by":"subagent","published":"2023-11-10","via":"mcp__github__issue_read method=get owner=pubgrub-rs repo=pubgrub issue_number=150"}
{"slug":"C-cargo-update-recommends-broken-invert-command-2024","url":"https://github.com/rust-lang/cargo/issues/14993","grading-certainty":"+1:SURE","grading-reasoning":"C not B: a well-written bug report from a contributor, but it is about CLI argument plumbing rather than explanation design; the explanation-relevant content is incidental. Not D because first-party tracker with reproducible steps.","relevance-certainty":"-0:SUSPECT","relevance-description":"Relevant only for one pattern: Cargo prints a pointer to the why-command inside its normal output ('note: to see how you depend on a package, run cargo tree --invert --package @') and the pointed-to command fails in common configurations. If Dorc points at a 'dorc why' command, the pointer must be executable as printed.","graded-by":"subagent","published":"2024-12-30","via":"mcp__github__issue_read method=get owner=rust-lang repo=cargo issue_number=14993"}
{"slug":"C-releasing-pip-twenty-three-resolver-2020","url":"https://pyfound.blogspot.com/2020/11/pip-20-3-new-resolver.html","grading-certainty":"+1:SURE","grading-reasoning":"C not B: first-party PSF announcement but it is promotional/procedural; on error messages it says only 'Substantial improvements in new resolver for performance, output and error messages' with no detail. Graded on informational content, not on the authority of the host.","relevance-certainty":"-1:GUESS","relevance-description":"Only useful for framing: the team labelled the rollout 'DISRUPTION' in capitals and kept a --use-deprecated=legacy-resolver escape for a full release cycle. Corroborates that even a well-funded rewrite shipped with a bypass. No rendering content.","graded-by":"subagent","published":"2020-11-30","via":"mcp__fetch__fetch url=https://pyfound.blogspot.com/2020/11/pip-20-3-new-resolver.html max_length=18000"}
{"slug":"D-aptitude-resolver-hints-configuration-2012","url":"https://www.debian.org/doc/manuals/aptitude/ch02s03s05.en.html","grading-certainty":"+1:SURE","grading-reasoning":"D not C: first-party Debian manual, but I fetched the wrong section — it documents resolver hint syntax (approve/reject/discard/safety-cost), not 'aptitude why' output. Included only so the miss is on the record; it contributed nothing to any claim above.","relevance-certainty":"-2:WONDER","relevance-description":"Essentially irrelevant to explanation rendering. Retained only to mark the aptitude vein as under-covered.","graded-by":"subagent","published":"2012","via":"mcp__fetch__fetch url=https://www.debian.org/doc/manuals/aptitude/ch02s03s05.en.html max_length=20000"}
```

---

## (d) VERBATIM EXCERPTS — the PubGrub rendering, literally

### The blog-post example (A-pubgrub-next-generation-version-solving-2018)
Note: no line numbers at all — the graph is a straight line.
```
Because dropdown >=2.0.0 depends on icons >=2.0.0 and root depends
  on icons <2.0.0, dropdown >=2.0.0 is forbidden.

And because menu >=1.1.0 depends on dropdown >=2.0.0, menu >=1.1.0
  is forbidden.

And because menu <1.1.0 depends on dropdown >=1.0.0 <2.0.0 which
  depends on intl <4.0.0, every version of menu requires intl
  <4.0.0.

So, because root depends on both menu >=1.0.0 and intl >=5.0.0,
  version solving failed.
```
Author's own framing of the machinery, verbatim: *"To produce user-friendly output, PubGrub does a
depth-first traversal of this graph and converts it into plain English. Since this conversion is
all about making things readable by humans, it involves a lot of fuzzy heuristics — things like
combining incompatibility descriptions in clever ways, folding together steps that would seem too
obvious, and adding line numbers when a purely linear argument won't suffice."*

### The LINEAR case, spec output (A-pubgrub-solver-specification-error-reporting-2018)
Two lines for a seven-node derivation graph. Note: zero line numbers.
```
Because every version of foo depends on bar ^2.0.0 which depends on baz ^3.0.0, every version of foo requires baz ^3.0.0.
So, because root depends on both baz ^1.0.0 and foo ^1.0.0, version solving failed.
```
The spec's own list of the human-friendliness special-cases, verbatim:
> * When we're talking about every version of a package, we explicitly write "every version of `foo`" rather than "`foo any`".
> * In the first line, instead of writing "every version of `foo` depends on `bar ^2.0.0` and every version of `bar` depends on `baz ^3.0.0`", we write "every version of `foo` depends on `bar ^2.0.0` which depends on `baz ^3.0.0`".
> * In the second line, instead of writing "`root` depends on `baz ^1.0.0` and `root` depends on `foo ^1.0.0`", we write "`root` depends on both `baz ^1.0.0` and `foo ^1.0.0`".
> * We omit the version number for the entrypoint package `root`.
> * Instead of writing "And" for the final line, we write "So," to help indicate that it's a conclusion.
> * Instead of writing "`root` is forbidden", we write "version solving failed".

And on collapsing, verbatim: *"The second line collapses together the explanations of two
incompatibilities (`{foo any, root 1.0.0}` and `{root 1.0.0}`)... We never explicitly explain that
every version of `foo` is incompatible with `root`, but the output is still clear."*

### The BRANCHING case, spec output — THE join-node rendering Dorc is proposing
Six message lines, ONE line number (`1`), one blank-line separator. Reproduced with the spec's
own Line column.
```
Message                                                                                              | Line
-----------------------------------------------------------------------------------------------------|-----
Because foo <1.1.0 depends on a ^1.0.0 which depends on b ^2.0.0, foo <1.1.0 requires b ^2.0.0.      |
So, because foo <1.1.0 depends on b ^1.0.0, foo <1.1.0 is forbidden.                                 |  1
                                                                                                     |
Because foo >=1.1.0 depends on x ^1.0.0 which depends on y ^2.0.0, foo >=1.1.0 requires y ^2.0.0.    |
And because foo >=1.1.0 depends on y ^1.0.0, foo >=1.1.0 is forbidden.                               |
And because foo <1.1.0 is forbidden (1), foo is forbidden.                                           |
So, because root depends on foo ^1.0.0, version solving failed.                                      |
```
Spec's explanation, verbatim: *"Because the derivation graph is non-linear — the incompatibility
`{not foo any}` is caused by two derived incompatibilities — we can't just explain everything in a
single sequence like we did in the last example. We first explain why `foo <1.1.0` is forbidden,
giving the conclusion an explicit line number so that we can refer back to it later on. Then we
explain why `foo >=1.1.0` is forbidden before finally concluding that version solving has failed."*

**Read this closely — it is the direct answer to Dorc's design question.** The join is NOT rendered
as `1a/1b -> 2`. It is rendered as: branch A written out in full, its *conclusion only* numbered
`(1)`, a blank line, branch B written out in full, then the join expressed as prose referring back
to `(1)`. Only ONE node in a seven-node graph earns a number.

### The numbering rule, verbatim (A-pubgrub-solver-specification-error-reporting-2018)
> Before running the error reporting algorithm proper, walk the derivation graph and record how
> many outgoing edges each derived incompatibility has–that is, how many different
> incompatibilities it causes.
> [...]
> In these cases, a naïvely linear explanation won't be clear. We need to refer to previous
> derivations that may not be physically nearby. We use line numbers to do this, but we only number
> incompatibilities that we *know* will need to be referred to later on. In the simple linear case,
> we don't include line numbers at all.
> [...]
> * Finally, if `incompatibility` causes two or more incompatibilities, give the line that was just
>   written a line number. Set this as `incompatibility`'s line number.

And the case analysis for two derived causes, verbatim (spec step 1.iii.a):
> If at least one cause's incompatibility is caused by two external incompatibilities:
> * Call this cause `simple` and the other cause `complex`. The `simple` cause can be described in a
>   single line, which is short enough that we don't need to use a line number to refer back to
>   `complex`.

Also, verbatim, the spec explicitly declines to fix the wording:
> Note that the text in the "Write" lines above is meant as a suggestion rather than a prescription.
> It's up to each implementation to determine the best way to convert each incompatibility to a
> human-readable string representation in a way that makes sense for that package manager's
> particular domain.

### The fan-out failure, verbatim (B-uv-no-build-message-very-verbose-2024)
```
  × No solution found when resolving dependencies:
  ╰─▶ Because only the following versions of pytest-pep8 are available:
          pytest-pep8==0.5
          ... [13 versions]
      and pytest-pep8==0.5 is unusable because no wheels are usable and building from source is disabled, we can conclude that pytest-pep8<0.6 cannot be used.
      And because pytest-pep8==0.6 is unusable because no wheels are usable and building from source is disabled, we can conclude that pytest-pep8<0.7 cannot be used.
      And because pytest-pep8==0.7 is unusable because no wheels are usable and building from source is disabled and pytest-pep8==0.8 is unusable because no wheels
      are usable and building from source is disabled, we can conclude that pytest-pep8<0.9 cannot be used.
      [... 5 more identical-shape lines ...]
      And because pytest-pep8==1.0.6 is unusable because no wheels are usable and building from source is disabled and you require pytest-pep8, we can conclude that
      the requirements are unsatisfiable.
```

### The unsound-collapse failure, verbatim (B-missing-term-in-derivation-tree-2024)
zanieb, on uv's merged output:
> But I want to focus on `open3d<0.9.0.0`, which doesn't make any sense here.

and:
> Notice we create the incompatibility `open3d==0.8.0.0 no wheels with a matching Python ABI tag`
> but it's never reflected in the terms — so it's missing from the conclusion.

and:
> So.. we do construct this term, it just doesn't make it into the tree? Is this expected? It seems
> incorrect? [...] That transformation may be entirely unsound and PubGrub is working as intended
> here.

### The raw-dump failure, verbatim (B-backtracking-resolver-human-unreadable-error-2023)
New resolver (excerpt; the real output is ~30 of these on one line):
```
pip._vendor.resolvelib.resolvers.ResolutionImpossible: [RequirementInformation(requirement=SpecifierRequirement('attrs~=19.3'), parent=LinkCandidate('https://files.pythonhosted.org/packages/36/07/368cf47f06564d7ffff603ade4c60039ecf3f5b368b75201f4ccb5512d78/apache_airflow-1.10.12-py2.py3-none-any.whl (from https://pypi.org/simple/apache-airflow/) (requires-python:>=2.7,!=3.0.*,...)')), RequirementInformation(requirement=SpecifierRequirement('attrs>=17.3.0'), parent=LinkCandidate('...aiohttp-3.8.1-...whl...')), ...]
```
Old (worse) resolver, same failure:
```
There are incompatible versions in the resolved dependencies:
  attrs>=20 (from cattrs==1.10.0->apache-airflow==1.10.12->-r requirements.in (line 1))
  attrs~=19.3 (from apache-airflow==1.10.12->-r requirements.in (line 1))
  attrs>=17.4.0 (from jsonschema==3.2.0->apache-airflow==1.10.12->-r requirements.in (line 1))
  attrs>=17.3.0 (from aiohttp==3.8.1->utilimarc-utils==2.24.0->-r requirements.in (line 13))
```
Note what the *readable* one does: four lines, each a leaf constraint, each carrying its full
provenance chain inline as an arrow-path terminating in a `file (line N)` citation. That inline
`->`-chain-per-leaf shape is a real alternative to a numbered DAG, and it is the one the reporter
preferred.

### npm's ERESOLVE, verbatim (C-npm-eresolve-unable-to-resolve-tree-2020)
```
npm ERR! ERESOLVE unable to resolve dependency tree
npm ERR!
npm ERR! While resolving: myapp@0.1.0
npm ERR! Found: @angular/common@11.0.3
npm ERR! node_modules/@angular/common
npm ERR!   @angular/common@"11.0.3" from the root project
npm ERR!
npm ERR! Could not resolve dependency:
npm ERR! peer @angular/common@"^9.1.0 || ^10.0.0" from @agm/core@3.0.0-beta.0
npm ERR! node_modules/@agm/core
npm ERR!   @agm/core@"3.0.0-beta.0" from the root project
```
And the offer of the escape hatch, verbatim from the same error:
```
npm ERR! Fix the upstream dependency conflict, or retry
npm ERR! this command with --force, or --legacy-peer-deps
npm ERR! to accept an incorrect (and potentially broken) dependency resolution.
```

### pip's shipped form, verbatim (B-pip-dependency-resolution-shipped-error-format-2025)
```
ERROR: Cannot install package_coffee==0.44.1 and package_tea==4.3.0 because these package versions have conflicting dependencies.

The conflict is caused by:
    package_coffee 0.44.1 depends on package_water<3.0.0,>=2.4.2
    package_tea 4.3.0 depends on package_water==2.3.1
```

### The abandoned pip tree-block, verbatim (B-conflicting-message-when-package-conflicts-itself-2020)
nlhkabu's proposal, 2020-10-13 — never shipped:
```
The conflict is caused by:
    package_1 requires six<1.9
    package_2 requires six<2 and >=1.9.0

Dependency tree:
- package_1 requires six<1.9
- package_4 requires package_3
  - package_3 requires package_2
    - package_2 requires six<2 and >=1.9.0

To fix this you could try to:
- loosen the range of package versions you've specified
- remove package versions to allow pip to attempt to solve the dependency conflict
- ask the package maintainers to loosen their dependencies
```
Her own hedge in the same comment, verbatim: *"Not sure this all makes sense? May also be a bit
too verbose?"*

---

## (e) NEGATIVE RESULTS

- **Reddit: NOT READ.** `www.reddit.com` and `old.reddit.com` both return
  `User-agent: * / Disallow: /`. I did not proxy around it. Kagi `site:reddit.com` returned
  snippets only, and I have not graded any of them. Two are worth chasing manually if the human
  can load them:
  - r/rust, "Common crates in Cargo workspace recompiled due to different features"
    (`/r/rust/comments/nvd6y7/`) — snippet: *"I tried `--no-dedupe`, but it generates a really huge
    tree; I gave up after 100MB. :)"* and the pipeline `cargo tree -e features --prefix none | sort
    | uniq`. This is the best practitioner corroboration of the de-dup cross-reference problem.
  - r/node, "How do you proceed updating your dependencies…" (`/r/node/comments/11zobne/`) —
    snippet: *"Your tree is broken, and the common wisdom is to use `--legacy-peer-deps` which
    might work, but is incorrect, lazy, and may result in bugs."*
- **No pip retrospective on the resolver rollout found.** The pip 20.3 announcement is
  promotional; the UX research pages predate the rollout; `pyfound.blogspot.com/2020/11/pip-team-final-report.html`
  is a 404. I could not find any published post-mortem measuring whether the new error messages
  worked. This is a real gap: the funded programme published its *design* research but apparently
  no *outcome* evaluation.
- **Poetry: not covered.** Poetry uses a PubGrub port (mixology) and would be a fourth data point
  on the same algorithm. Not reached.
- **`aptitude why` first-party docs: not reached.** I fetched the wrong manual section (resolver
  hints). The `--show-summary` flag exists (summarize each chain rather than long-form) and is
  on-point, but I only have it via a search snippet of the aptitude reference manual, so it is not
  graded. `aptitude why-not` output was not seen beyond one askubuntu example.
- **No eye-tracking / controlled-attention study found within this vein.** Nothing in the
  package-manager literature measures whether users read resolver explanations. The pip mental-models
  and override surveys are the closest, and both are self-report. If the synthesis wants
  measured attention data it will have to come from the compiler-diagnostics literature, which is
  outside this vein.
- **No abandoned-design retrospective found for PubGrub's line numbering itself.** The dart-lang
  spec's numbering rule has, as far as I can tell, never been publicly revisited or criticised —
  every downstream critique (uv, pubgrub-rs) targets verbosity and node selection, not the
  numbering scheme. Treat "numbering only out-degree>=2 nodes works" as UNVALIDATED-BUT-UNCHALLENGED
  rather than as proven.
