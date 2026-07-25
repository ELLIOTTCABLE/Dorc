# Vein: build-system "why" / dependency-path explanation surfaces

Gathered for the Dorc decision-provenance-chain rendering question. Disproof-first.
Every source below was FULLY READ (read-depth caveats noted inline). Reddit and one
HN thread could NOT be read — see NEGATIVE RESULTS.

---

## (a) DON'T-DOS

### dont-inline-excerpt-per-link — the per-link inlined code excerpt got DEMOTED to opt-in

The single most on-point finding. Dorc plans an inlined code excerpt on every link.
Nix shipped exactly that, as the DEFAULT, and removed it.

`nix why-depends` originally printed, for every edge in the chain, the *file* in the
parent plus a windowed excerpt showing the referencing string. Issue NixOS/nix#5912
(lilyball, a Nix MEMBER, 2022-01-13) titled ``nix why-depends --derivation` output is
very noisy, a little hard to read``:

> "`nix why-depends` shows the location of the store hash in each file along the path.
> This can be interesting for runtime closures, but for `--derivation` output it's just
> very noisy as every "file" along the path is reported as `/` and the contents are a
> compact derivation, thus causing confusing output where every line starts with the end
> of the previous entry in the derivation. Also longer paths can cause the relevant
> output to be truncated"

> "Only the inclusion of colors on the matched hashes make it even semi-readable, and of
> course those go away when you have no color."

Eight days later thufschmitt (also MEMBER) merged PR #5942, titled verbatim:

> **"Make `nix why-depends` quieter by default"**
> "Unless `--precise` is passed, make `nix why-depends` only show the dependencies
> between the store paths, without introspecting them to find the actual references.
> This also makes it ~3x faster"

Two independent costs, both paid by the excerpt: legibility AND a 3x runtime penalty.
The excerpt is now behind `--precise`, documented in the current manual as:
"For each edge in the dependency graph, show the files in the parent that cause the
dependency."

Corroborating: Buck2's `explain` has the same shape of flag —
`--stack`: "Add target code pointer. **This invalidates cache, slowing things down**".
Source-locus attribution is opt-in in Buck2 too, for the same performance reason.

Dorc implication: an inlined excerpt + `file:line` on EVERY link is the exact design
two independent teams (Nix, Meta) made opt-in. Treat "excerpt everywhere by default"
as a known-failed default.

### dont-linearize-a-path-set — first-party admission that path-sets don't render linearly

Bazel query reference, `--output graph` section (canonical spec):

> "This output format is particularly useful for `allpaths`, `deps`, or `rdeps` queries,
> where the result includes a *set of paths* that **cannot be easily visualized when
> rendered in a linear form**, such as with `--output label`."

Dorc's proposed rendering *is* the linear form (numbered links 1,2,3 with 1a/1b -> 2
joins). Bazel's own docs say a set of paths does not survive that transformation. Note
the phrasing is about a *set of paths*, i.e. exactly the join-node case.

### dont-trust-the-single-path — the "one path" primitive returns an arbitrary path

Bazel query reference (present in the 3.2.0-pinned doc; **this caveat has since been
deleted from the current page**, which is itself a rot signal):

> "Please note, the ranked output of a `somepath` query is basically meaningless because
> `somepath` doesn't guarantee to return either a shortest or a longest path, and it may
> include "transitive" edges from one path node to another that are **not direct edges in
> original graph**."

Two separate failures in one sentence: (1) the chain you show is not the shortest, so the
user's "why" is answered with a needlessly long story; (2) the chain may contain edges
that DO NOT EXIST in the real graph — the linearization fabricates adjacency. For Dorc,
whose links carry epistemic tier-words and are meant to be individually actionable, a
fabricated adjacency is a correctness bug, not a cosmetic one.

### dont-order-by-traversal — graph-walk order is meaningful to the implementer, not the user

gradle/gradle#32469 (open, 2025-02-17), reporter pkubowicz:

> "dependencyInsight suggests several unhelpful paths first: [1,2,3] ... before printing
> out a mildly helpful path: 4. ... Note that path 4 is helpful, but not the shortest one"

> "I define 'unhelpful' as any path that does not include spring-boot-starter-web (which
> is the one and only dependency declared in build.gradle.kts). This is a project with 2
> subprojects in total, and 1 direct dependency, but the output of dependencyInsight is
> **very noisy and very hard to interpret**. You can imagine that as we scale this 2x1
> project to something closer to real life work, the task output signal-to-noise ratio
> gets much worse than visible here."

Gradle maintainer tresat's ruling (2025-03-20) — the WONTFIX is the finding:

> "Our initial thoughts are that the current ordering should remain, as it is meaningful.
> **It is the order the dependencies are encountered when walking the graph.** We are open
> to the idea of adding additional arguments to this report that would allow filtering or
> refining the output. We don't have the capacity to explore this right now"

A 1-direct-dependency project already buries the actionable path in position 4. Dorc's
annoyed ops engineer will not read to position 4. Ordering must be by user-relevance
(shortest, or "contains the thing the user declared"), never by traversal order.

### dont-name-the-decision-without-the-evidence — "One of the files has changed."

Bazel's `--explain` log is the closest existing analogue to a per-link tier-word, and it
is the canonical example of a provenance line that states the decision and withholds the
evidence. Verbatim output, captured by jmmv (Julio Merino, ex-Google Bazel team) in a
2025 primary-source post:

```
Build options: --explain=log
Executing action 'BazelWorkspaceStatusAction stable-status.txt': unconditional execution is requested.
Executing action 'Executing genrule //:date': One of the files has changed.
Executing action 'Executing genrule //:copy': One of the files has changed.
Executing action 'Executing genrule //:count': One of the files has changed.
```

bazelbuild/bazel#13566 (metti, CONTRIBUTOR, 2021) asked for the missing half:

> "Running `bazel --explain explain.log` will create `explain.log` with an explanation why
> a target was rebuilt. **Unfortunately it lacks detailed information about which files
> changed.**"

> "I would like to specify `--explain explain.log` for my builds unconditionally to adhoc
> debug "huh-why-did-this-now-build?"-situations."

The Bazel-side answer (lberki, CONTRIBUTOR) is a structural one worth internalising:

> "The reason why Bazel doesn't have this admittedly basic piece of functionality is that
> it's, surprisingly, not doable with the data Bazel keeps about an action. Since some
> actions have a lot of inputs, Bazel doesn't store the checksums of each input or even
> the set of inputs for each cached action (except for C++). Instead, what it does it
> stores just the xor of all the checksums ... This is so that the action cache uses less
> RAM. **So when one of these files changes, it's detected, but Bazel cannot tell which one
> changed** (hell, I don't even know how we detect when the set of inputs files changed...)"

Closed 2023-05-25 by stale-bot, `state_reason: not_planned`, 4x +1, never fixed.

Dorc implication: decide UP FRONT whether each tier-word can be backed by a retained
witness. A tier-word you cannot cash out into a locus is Bazel's "One of the files has
changed." — and once the data isn't retained, the fix is a storage-architecture change
nobody will fund. `--verbose_explanations` is `@Deprecated` +
`documentationCategory = UNDOCUMENTED` in current Bazel source (BazelRulesModule.java),
i.e. the escape hatch was itself abandoned.

### dont-elide-the-interesting-middle — a chain whose nodes are the wrong abstraction

discourse.nixos.org "Better why-depends (fill in the gaps)" (2023-08-03, 16 likes). The
user gets a perfectly clean 4-line chain and it is useless:

```
/nix/store/qb0lwhnby4gmyrww2gpdirrh473marcb-nixos-system-framie-...drv
└───/nix/store/nq9dj7dpvazjg7n437hxbfcjly9di17b-system-path.drv
    └───/nix/store/yjbvja9b80zzzl2b21pr05z4z2269n38-capitaine-cursors-4.drv
        └───/nix/store/0yfa0rs0cpn3nws7hcm7rw7lchnbghxh-inkscape-1.2.2.drv
```

> "**After ~15m of digging** I discovered that this `capitaine-cursors` dependency is
> *probably* pulled in from `lightdm.greeters.enso.enable = true;` in my system flake"

> "But, this discovery **isn't verified to be true** ... Also, the investigation path that I
> took was a little heuristic (educated guessing/grepping)."

> "It seems that `why-depends` **glosses over parts of the tree** between `system-path.drv`
> and `capitaine-cursors-4.drv` in such a way as to **elide the relationship** between
> `system-path.drv` and `config.lightdm.greeters.enso`. I want to see this relationship."

Maintainer-level answer (Atemu) explains why it is structurally unfixable at that layer:

> "The problem here is that why-depends cannot know how the dep came into system-path.drv
> as it works on drv level. Drv files are dumb data that aren't even meant to be
> introspectable ... **There is no info about *how* one drv became a direct dependency of
> another drv inside the drv files; none whatsoever.**"

The user's stated want is verbatim Dorc's design:

> "Is there a way for me to ... answer the question "which lines of which derivations
> require this dependency" almost like a stacktrace-version of `why-depends` such that I
> would see something like: `.../flake.nix:179 # the line which defines ...`"

Dorc implication: the chain is only as useful as the layer its NODES live at. If Dorc's
links are at the analyzer's internal layer rather than the layer the admin wrote, a
syntactically perfect chain still costs 15 minutes and yields an unverified guess. The
"other user" check bites here: an oracle-author's node granularity is not an admin's.

### dont-assume-the-chain-adds-information — it can be a re-serialisation of the error

n8henrie (2025-10-12), a full practitioner walkthrough. After ~6 failed invocations
(wrong argument order, flake-vs-store-path confusion, `--derivation` needed, package must
already be built), he finally gets the chain — and:

> "**Anticlimactically, careful inspect reveals this to be essentially a cleaned-up version
> of the output we got from `darwin-rebuild` in the first place.**"

The plain error message already contained the chain. The dedicated provenance tool's
marginal information was ~zero; its marginal cost was a multi-step expedition.

This is the harshest test for Dorc: before rendering a chain, ask what it adds over the
diagnostic the user already has. Note also the failure-mode inventory he hits, all of
which are UX cliffs Dorc will have analogues of:
- `nix why-depends A B` where B must already have been successfully built
- `'…' does not depend on '…'` when the user names the right thing at the wrong layer
- needing `nix derivation show --recursive | jq` just to *construct the query arguments*

### dont-ship-a-full-trace-and-call-it-explanation — make -d, and the reduced tier people use

StackOverflow "Debugging GNU make", accepted answer:

> "`make -d` should give you *more* than enough information to debug your makefile.
> **Be warned: it will take some time and effort to analyze the output but loading the
> output into your favorite editor and doing searches will assist a lot.**"

> "It looks like `make --debug=b` is the best option for what you need"

The recommended workflow for the maximal trace is: dump to a file, open an editor, search.
Nobody reads it in the terminal. The answer's actual recommendation is the *basic* tier
(`--debug=b`, 6 lines) over the *all* tier (`-d`). GNU make ships graduated verbosity
(`a`/`b`/`v`/`i`/`j`/`m`) precisely because one level is wrong for everyone.

Corroborating from a different ecosystem — Ninja's `-d explain` is a real, widely-used
flag (Chromium's first-party `debugging_slow_builds.md` says "Use `ninja -n -d explain`
to figure out why ninja thinks a target is dirty") and it is **not documented in the
Ninja manual at all**. Expert-only escape hatches drift out of the docs.

### dont-print-the-whole-tree — scoping beats formatting, by 10x

Katie Barnett (proandroiddev, 2024), on the Gradle dependency tree:

> "If you take a look at the resulting file, you will see it is massive (**for a simple
> test Hello World app it was 7435 lines long**). You can narrow this down by specifying
> the configuration you are interested in ... **Now my file is only 692 lines long. Much
> easier to use!**"

And the causal question the tree cannot answer:

> "But from this **I can't tell if** `androidx.navigation:navigation-compose:2.8.0-beta02`
> **is forcing** `androidx.compose.foundation:foundation-layout` to use version
> `1.7.0-beta02`. In a large project this could also be very **tedious** go through every
> mention of the problematic library and compare them."

A 10x reduction came from a SCOPE argument, not from better rendering. Dorc should treat
"which slice of the DAG" as the primary control, ahead of how the slice is drawn.

### dont-expect-terminal-to-win-at-scale — Buck2 didn't even try

`buck2 explain` (canonical docs, current):

> "Generates **web browser view** that shows actions that ran in the last build mapped to
> the target graph"

The entire surface is a browser view (`app/buck2_explain/js/` in-tree). Meta, operating
the largest monorepo of the three, shipped no ASCII explain rendering. Its terminal
`buck2 log` subcommands are deliberately *machine* surfaces: `what-ran`, `what-failed`,
`critical-path`, all with `--format json|csv|tabulated`, and `buck2 log cmd` carries the
tell: **"This command output is not machine readable. Robots, please use `buck2 log show`."**

Gradle did the same reversal and advertises it *inside its own terminal output*. Every
`dependencies` / `dependencyInsight` invocation in the canonical docs ends with the
literal line:

> "A web-based, searchable dependency report is available by adding the `--scan` option."

and the docs' third section is titled "**Get a holistic view using Build Scan**". Ninja
ships `-t browse` ("browse the dependency graph in a web browser") and `-t graph` (pipe
to graphviz). Bazel's guide pipes to `dot`. Four of five systems examined route
graph-shaped explanation out of the terminal.

CAVEAT — the reversal is not clean, see DO-DOS.

---

## (b) DO-DOS

### do-shortest-path-by-default-all-behind-a-flag

The strongest positive result, with a clean A/B in one document. n8henrie's default
(single shortest path) chain is 7 lines and immediately actionable:

```
/nix/store/...-darwin-system-25.11.9a9ab01.drv
└───/nix/store/...-system-applications.drv
    └───/nix/store/...-actionlint-1.7.7.drv
        └───/nix/store/...-ronn-0.10.1.drv
            └───/nix/store/...-ronn-gems.drv
                └───/nix/store/...-ruby3.3-nokogiri-1.16.0.drv
                    └───/nix/store/...-nokogiri-1.16.0.gem.drv
```

> "Thankfully, with some help from the cleaner output, we can see fairly easily that the
> system depends on system-applications, which depends on actionlint."

The same query with `--all` sprawls to ~30 lines with 4 top-level branches and repeated
subtrees, and his own verdict is:

> "there is also an `--all` flag that shows other paths to the same dependency; **its output
> looks more similar to the original output from `darwin-rebuild`**"

i.e. all-paths regresses to the unreadable thing he was escaping. Canonical Nix manual
confirms the default: "showing a **shortest** sequence in the references graph". Gradle
independently offers the same escape valve — `dependencyInsight --single-path`: "Render
only a single path to the dependency."

Dorc: default to ONE shortest chain. Put the join-node/all-paths view behind a flag, and
expect most users never to type it.

### do-merge-topologically-equivalent-nodes

Bazel's `--output graph` factoring, canonical:

> "By default, the graph is rendered in a *factored* form. That is,
> **topologically-equivalent nodes are merged together into a single node with multiple
> labels.** This makes the graph more compact and readable, **because typical result graphs
> contain highly repetitive patterns.** For example, a `java_library` rule may depend on
> hundreds of Java source files all generated by the same `genrule`; in the factored
> graph, all these files are represented by a single node."

And the inverse is stated as a known-bad: "`--nograph:factored` ... makes visualization
using GraphViz **impractical**, but the simpler format may ease processing by other tools."

This is the principled version of Dorc's `1a/1b -> 2` join. Merge by topological
equivalence and carry multiple labels on one node, rather than emitting 1a/1b/1c/1d…

Gradle ships the terminal-text form of the same idea, as documented annotations:
- `(*)`: "Indicates repeated occurrences of a transitive dependency subtree. Gradle
  expands transitive dependency subtrees only once per project; repeat occurrences only
  display the root of the subtree, followed by this annotation."
- `(c)`: "This element is a dependency constraint, not a dependency."
- `(n)`: "A dependency or dependency configuration that cannot be resolved."

A one-character suffix that means "I already showed you this subtree" is cheap and works.
Dorc's tier-words could occupy the same slot.

### do-emit-grep-parseable-loci

Bazel `--output location`, canonical:

> "this option prints out, for each target in the result, the target's kind and label, but
> it is prefixed by a string describing the location of that target, **as a filename and
> line number. The format resembles the output of `grep`. Thus, tools that can parse the
> latter (such as Emacs or vi) can also use the query output to step through a series of
> matches**, allowing the Bazel query tool to be used as a dependency-graph-aware "grep for
> BUILD files"."

Dorc already plans `file:line` per link. The instruction here is about FORMAT: emit it in
grep/`file:line:` shape so `vim -q`, `M-x compile`, and `fzf` consume it for free. This
also gives the annoyed engineer a way to *skip reading* and just jump — which is the
realistic best case for a user whose attention is at a premium.

Same doc, the honesty note worth copying: for generated files, "(The query tool does not
have sufficient information to find the actual location ... and in any case, it might not
exist if a build has not yet been performed.)" — i.e. state where a locus is unavailable
rather than fabricating one. Maps directly onto Dorc's tier-words.

### do-truncate-labels-with-a-tunable

`--graph:node_limit n`: "specifies the maximum length of the label string for a graph
node in the output. Longer labels will be truncated; -1 disables truncation. Due to the
factored form in which graphs are usually printed, the node labels may be very long."
Default 1024. A hard cap with an explicit disable is shipped, boring, and works.

### do-name-the-selection-reason-from-a-closed-vocabulary

Gradle's `dependencyInsight` "Selection reasons" is the closest shipped analogue to Dorc's
epistemic tier-words, and it works because it is a small CLOSED table, documented, with
each entry saying what it means: `(Absent)`, `Was requested : <text>`,
`Was requested : didn't match versions`, `Was requested : reject version`,
`By conflict resolution : between versions <v>`, `By constraint`, `By ancestor`,
`Selected by rule`, `Rejection : <version> by rule because <text>`, `Forced`.

Real output carries several at once and remains readable:

```
   Selection reasons:
      - By constraint: foundation-layout is in atomic group androidx.compose.foundation
      - By constraint: prevents a critical bug in Text
      - By conflict resolution: between versions 1.7.0-beta02, 1.6.7, 1.4.0 and 1.6.0
```

Note the two design moves Dorc should copy: (1) the docs enumerate the vocabulary in a
2-column Reason/Meaning table — the tier-word is not self-explanatory and Gradle doesn't
pretend it is; (2) several reasons stack on one node ("If multiple selection reasons
exist, the insight report lists all of them"), rather than forcing one tier-word per link.
Gradle also lets the *author* inject prose into the reason via `because` text, so the
reason string is partly user-authored — a good match for Dorc's oracle-author/admin split.

### do-give-graduated-verbosity-with-a-named-escalation-path

The one genuinely useful thing in the (LLM-smelling) Bazel Knowledge Hub chapter is the
staged protocol, and it matches what the primary sources independently show people doing:

> "If the first `--explain` log is too vague, keep the same target and **add one clue at a
> time instead of changing everything at once**: `--verbose_explanations` when Bazel says
> the command changed; `--announce_rc` when you suspect `.bazelrc` or `--config=...` drift;
> stamping checks when workspace-status-related actions are the noisy input"

> "`--explain` is intentionally a first-response tool. It is great at answering "why did
> Bazel rerun this build step?" **but not at exhaustively diffing two invocations.**"

GNU make's `--debug=a|b|v|i|j|m` is the same idea, 30 years earlier.

### do-ship-a-machine-format-because-experts-will-diff-not-read

The expert workflow, in three independent sources, is DIFF TWO DUMPS — not read one
explanation. jmmv's actual recommended procedure for the hard case bypasses `--explain`
entirely:

```
$ bazel clean
$ bazel build --noremote_accept_cached --execution_log_json_file=before //:copy2
$ bazel clean
$ bazel build --noremote_accept_cached --execution_log_json_file=after  //:copy2
```
> "I like doing `diff -u before after | cdiff`"

Katie Barnett, independently, on Gradle: "Also sending the output the results of this to a
file so I could then **do a diff** with the result before the changes and after the changes."
Buck2 ships this as a first-class subcommand: `buck2 log diff action-divergence`,
"Identifies the first divergent action between two builds."

Dorc: the prose chain is for the first 80%; ship a stable machine format (and ideally a
`dorc ... diff` verb) for the rest. Do not try to make the prose chain serve the expert.

### do-not-over-read-the-web-ui-reversal

Counter-evidence, and it is the reason to keep the terminal rendering good. Katie Barnett,
who knows about `--scan`:

> "If you want to get fancy, you can also add `--scan` ... to produce a searchable web based
> report. **This involves verifying your email with gradle and often I find it quicker just
> to view the text version.**"

The web UI lost on friction (auth, upload, network) for a real practitioner doing a real
task. Develocity's own marketing post for the improved scan UI is entirely
screenshot-driven and never argues the terminal was the wrong medium — it argues for
*comparison* features ("compare this build to the last one to see which dependencies have
changed"), i.e. the diff workflow again, not the explanation workflow.

Dorc is an ops tool run over SSH on someone else's box. The friction that killed `--scan`
for Barnett is strictly worse in Dorc's context. The lesson is not "build a web UI"; it is
"the terminal rendering must be good enough that nobody wants one, and the scaling story
is scoping + machine output, not a browser."

---

## (c) GRADED SOURCE TABLE

```json
{"slug":"A-bazel-query-language-reference-2026","url":"https://bazel.build/query/language","grading-certainty":"+1:SURE","grading-reasoning":"Canonical first-party spec for the query language and every --output format, read in full across two paginated chunks; deciding factor over B is that this IS the normative definition of somepath/allpaths/--output graph, not documentation about them. Docked nothing for rot here because I separately verified the one stale passage against the pinned 3.2.0 copy.","relevance-certainty":"+1:SURE","relevance-description":"Supplies the load-bearing first-party statements that a set of paths cannot be rendered linearly, that graphs are factored by merging topologically-equivalent nodes, that --output location is deliberately grep-shaped, and the --graph:node_limit truncation default.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://bazel.build/query/language', start_index: 14000 and 30000)"}
{"slug":"A-bazel-query-how-to-guide-2026","url":"https://bazel.build/query/guide","grading-certainty":"+1:SURE","grading-reasoning":"First-party canonical how-to, fully read, and it is the document that actually names the user's question ('Why does this dependency exist'); primary rather than secondary, hence A not C.","relevance-certainty":"+1:SURE","relevance-description":"Shows Bazel's recommended escalation: start with somepath (single path) and only reach for allpaths + dot when that fails — the shortest-path-first default, stated as guidance rather than inferred.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://bazel.build/query/guide')"}
{"slug":"A-gradle-viewing-debugging-dependencies-2026","url":"https://docs.gradle.org/current/userguide/viewing_debugging_dependencies.html","grading-certainty":"+1:SURE","grading-reasoning":"Canonical first-party user manual chapter, fully read including the complete Selection-reasons vocabulary table and the (*)/(c)/(n) annotation definitions; the closed reason-vocabulary is normative spec, which is what separates A from B here.","relevance-certainty":"+1:SURE","relevance-description":"The nearest shipped analogue to Dorc's epistemic tier-words (a small documented closed vocabulary, stackable per node, with author-injected 'because' prose), plus --single-path, plus the in-terminal advertisement for the --scan web UI.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://docs.gradle.org/current/userguide/viewing_debugging_dependencies.html')"}
{"slug":"A-buck2-explain-command-reference-2025","url":"https://buck2.build/docs/users/commands/explain/","grading-certainty":"+1:SURE","grading-reasoning":"Short canonical first-party command reference, read in full (the page is complete at this length, not truncated); it is the authoritative statement of what Meta's explain surface IS, so A rather than B despite its brevity.","relevance-certainty":"+1:SURE","relevance-description":"Establishes that Buck2's explain surface is a web browser view rather than terminal text, and that source-code pointers (--stack) are opt-in because they invalidate cache — an independent corroboration of Nix's --precise demotion.","graded-by":"subagent","published":"2025-04-16","via":"mcp__fetch__fetch(url: 'https://buck2.build/docs/users/commands/explain/')"}
{"slug":"B-buck2-log-subcommand-reference-2023","url":"https://buck2.build/docs/users/commands/log/","grading-certainty":"-0:SUSPECT","grading-reasoning":"First-party canonical reference, but my read was truncated at 20k chars partway through 'buck2 log diff', so read-depth is incomplete — that alone drops it from A to B; content quoted is from the fully-read portion.","relevance-certainty":"+1:SURE","relevance-description":"Shows Buck2's terminal log surfaces are machine-first (what-ran/what-failed/critical-path, all with --format json|csv, plus the explicit 'Robots, please use buck2 log show' note) and that action-divergence diffing is a first-class subcommand. Also establishes there is no 'buck2 log why-ran'.","graded-by":"subagent","published":"2023-11-30","via":"mcp__fetch__fetch(url: 'https://buck2.build/docs/users/commands/log/')"}
{"slug":"B-nix-why-depends-manual-page-2026","url":"https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-why-depends.html","grading-certainty":"-0:SUSPECT","grading-reasoning":"First-party canonical manual, fully read, but held below A for internal rot: the Options section correctly documents --precise as opt-in while every worked Example still shows the pre-2022 excerpt-by-default output, so the page contradicts itself for a reader.","relevance-certainty":"+1:SURE","relevance-description":"Normative text for the --precise/--all flags and the 'shortest sequence in the references graph' default; the stale examples double as a preserved specimen of the rejected excerpt-everywhere rendering.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-why-depends.html')"}
{"slug":"B-nix-quieter-by-default-pull-2022","url":"https://github.com/NixOS/nix/pull/5942","grading-certainty":"+1:SURE","grading-reasoning":"Core-maintainer (thufschmitt) writing on the merged change itself — primary provenance for the design reversal, not a report of it; body read in full via the search payload, which for this PR is the complete text.","relevance-certainty":"+1:SURE","relevance-description":"THE canonical don't-do: the per-edge inlined excerpt was removed from the default output for legibility AND a ~3x speedup, exactly the feature Dorc plans to make default.","graded-by":"subagent","published":"2022-01-19","via":"mcp__github__search_pull_requests(query: 'repo:NixOS/nix why-depends precise')"}
{"slug":"B-nix-why-depends-noisy-issue-2022","url":"https://github.com/NixOS/nix/issues/5912","grading-certainty":"+1:SURE","grading-reasoning":"Bug report by a Nix MEMBER with the failing output pasted verbatim; primary, fully read (confirmed zero comments via get_comments, so nothing is missing). Below A only because it is a bug report rather than spec or peer-reviewed work.","relevance-certainty":"+1:SURE","relevance-description":"The complaint that motivated the reversal, with the unreadable excerpt-per-link output preserved in full, including the truncation and the colour-dependence failure.","graded-by":"subagent","published":"2022-01-13","via":"mcp__github__issue_read(method: 'get' and 'get_comments', owner: 'NixOS', repo: 'nix', issue_number: 5912)"}
{"slug":"B-bazel-explain-what-files-changed-2021","url":"https://github.com/bazelbuild/bazel/issues/13566","grading-certainty":"+1:SURE","grading-reasoning":"Read body plus all 4 comments; the substantive comment is from a Bazel CONTRIBUTOR giving the internal data-model reason, which is core-author primary writing. Not A because it is an issue thread, not spec.","relevance-certainty":"+1:SURE","relevance-description":"Documents that Bazel's provenance line names the decision but cannot name the evidence, that the cause is a deliberate RAM tradeoff in the action cache, and that the request died to a stale-bot as not_planned after two years.","graded-by":"subagent","published":"2021-06-09","via":"mcp__github__issue_read(method: 'get' and 'get_comments', owner: 'bazelbuild', repo: 'bazel', issue_number: 13566)"}
{"slug":"B-gradle-dependencyinsight-noisy-issue-2025","url":"https://github.com/gradle/gradle/issues/32469","grading-certainty":"+1:SURE","grading-reasoning":"Body and both comments read in full; the deciding factor for B is that the second comment is the Gradle maintainer's actual design ruling (primary), not commentary about it. Still an issue thread rather than spec, so not A.","relevance-certainty":"+1:SURE","relevance-description":"A reproducible case where a 1-direct-dependency project buries the useful path in position 4, plus the maintainer's on-record defence of graph-traversal ordering — the strongest evidence that traversal order is the wrong presentation order.","graded-by":"subagent","published":"2025-02-17","via":"mcp__github__issue_read(method: 'get' and 'get_comments', owner: 'gradle', repo: 'gradle', issue_number: 32469)"}
{"slug":"B-bazel-action-nondeterminism-jmmv-2025","url":"https://blogsystem5.substack.com/p/bazel-action-determinism","grading-certainty":"+1:SURE","grading-reasoning":"Deep primary-source post by a known practitioner (Julio Merino, ex-Google Bazel team) with reproduced terminal transcripts rather than recollection; fully read. Not A only because it is a personal blog rather than peer-reviewed or normative.","relevance-certainty":"+1:SURE","relevance-description":"Captures real --explain log output verbatim ('One of the files has changed.') and then demonstrates that the expert workflow for the hard case abandons the explanation entirely in favour of diffing two JSON execution logs.","graded-by":"subagent","published":"2025-07","via":"mcp__fetch__fetch(url: 'https://blogsystem5.substack.com/p/bazel-action-determinism')"}
{"slug":"B-nix-why-does-system-depend-2025","url":"https://n8henrie.com/2025/10/nix-why-does-my-system-depend-on-pkg/","grading-certainty":"-0:SUSPECT","grading-reasoning":"Personal blog, so not first-party — but graded B rather than C because it is a complete primary transcript of one practitioner's real session with every failed invocation and its exact error preserved, which is stronger evidence than a tidied-up secondary write-up; fully read.","relevance-certainty":"+1:SURE","relevance-description":"Provides the default-vs---all A/B in one document (7 legible lines vs ~30 sprawling), the inventory of query-construction cliffs, and the finding that the finished chain was 'essentially a cleaned-up version' of the error message the user already had.","graded-by":"subagent","published":"2025-10-12","via":"mcp__fetch__fetch(url: 'https://n8henrie.com/2025/10/nix-why-does-my-system-depend-on-pkg/')"}
{"slug":"B-chromium-debugging-slow-builds-2026","url":"https://chromium.googlesource.com/chromium/src/+/master/build/docs/debugging_slow_builds.md","grading-certainty":"-0:SUSPECT","grading-reasoning":"First-party Chromium engineering doc, fully read, but it is a terse internal tips page that merely names the flag with no rationale — thin content is what holds it at B rather than A.","relevance-certainty":"-0:SUSPECT","relevance-description":"Corroborates that 'ninja -n -d explain' is the real in-practice why-is-this-dirty tool at Chromium scale, which matters because that flag is absent from the Ninja manual. Supporting rather than load-bearing.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://chromium.googlesource.com/chromium/src/+/master/build/docs/debugging_slow_builds.md')"}
{"slug":"B-ninja-build-manual-tools-2026","url":"https://ninja-build.org/manual.html","grading-certainty":"-0:SUSPECT","grading-reasoning":"Canonical first-party manual but my read was truncated at 30k chars (through the -t tool table and into deps handling); more importantly its relevance is indirect since -d explain is undocumented here, so B not A on read-depth plus fit.","relevance-certainty":"-1:GUESS","relevance-description":"Establishes that Ninja routes graph inspection to a browser (-t browse) or graphviz (-t graph) rather than ASCII, and — by omission — that the widely-used -d explain flag is undocumented. Weakest source in the set; the omission claim is the valuable half.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://ninja-build.org/manual.html')"}
{"slug":"C-better-why-depends-fill-gaps-2023","url":"https://discourse.nixos.org/t/better-why-depends-fill-in-the-gaps/31246","grading-certainty":"-0:SUSPECT","grading-reasoning":"Forum thread, so social rather than primary-authoritative — but a well-referenced one (16 likes, substantive replies from recognised community members) with the user's real output pasted, which is why C rather than D. Fully read including all 11 posts.","relevance-certainty":"+1:SURE","relevance-description":"The '15 minutes of digging on a clean 4-line chain' case, the structural explanation of why drv-level nodes cannot carry the reason edge, and a user request phrased as literally Dorc's design ('a stacktrace-version of why-depends' with file:line).","graded-by":"subagent","published":"2023-08-03","via":"mcp__fetch__fetch(url: 'https://discourse.nixos.org/t/better-why-depends-fill-in-the-gaps/31246')"}
{"slug":"C-bazel-query-somepath-caveat-2020","url":"https://docs.bazel.build/versions/3.2.0/query.html","grading-certainty":"+1:SURE","grading-reasoning":"First-party spec text, but deliberately graded C on rot: this is a 2020-pinned version and the passage I rely on has been DELETED from the current page, so the claim cannot be attributed to today's Bazel without that caveat. Read the relevant sections in full at two offsets.","relevance-certainty":"+1:SURE","relevance-description":"Sole source for the admission that somepath returns neither shortest nor longest path and 'may include transitive edges ... that are not direct edges in original graph' — the fabricated-adjacency risk in any DAG linearisation.","graded-by":"subagent","published":"2020-05","via":"mcp__fetch__fetch(url: 'https://docs.bazel.build/versions/3.2.0/query.html', start_index: 37500 and 42000)"}
{"slug":"C-debugging-gnu-make-answer-2009","url":"https://stackoverflow.com/questions/1745939/debugging-gnu-make","grading-certainty":"-0:SUSPECT","grading-reasoning":"High-visibility accepted StackOverflow answer with a reproduced transcript, but it is a social post and my fetch returned only the accepted answer rather than the full thread — read-depth plus venue put it at C, not B.","relevance-certainty":"+1:SURE","relevance-description":"The canonical 'the full trace is unreadable, dump it to an editor and search' admission, and the recommendation of the reduced --debug=b tier over -d; establishes graduated verbosity as the shipped answer.","graded-by":"subagent","published":"2009-11","via":"mcp__fetch__fetch(url: 'https://stackoverflow.com/questions/1745939/debugging-gnu-make')"}
{"slug":"C-debugging-dependencies-in-gradle-2024","url":"https://proandroiddev.com/debugging-dependencies-in-gradle-54c8be444849","grading-certainty":"-0:SUSPECT","grading-reasoning":"Medium tutorial — secondary in form — but carries the author's own measured numbers and real command transcripts, which lifts it above D; not B because it is a general how-to rather than deep primary investigation. Fully read.","relevance-certainty":"+1:SURE","relevance-description":"Quantifies scoping as a 7435-to-692-line reduction, states the tree cannot answer the causal 'is X forcing Y' question, records the dump-to-file-and-diff workflow, and supplies the counter-evidence that --scan's auth friction sends a real user back to the terminal.","graded-by":"subagent","published":"2024-06-09","via":"mcp__fetch__fetch(url: 'https://proandroiddev.com/debugging-dependencies-in-gradle-54c8be444849')"}
{"slug":"D-develocity-dependency-insights-scans-2019","url":"https://develocity.ai/blog/improved-gradle-dependency-resolution-insights-in-build-scans/","grading-certainty":"+1:SURE","grading-reasoning":"Vendor marketing post for a paid product, screenshot-driven with no rationale content and a trial-request CTA; commercial non-primary is a straight D under the rubric despite being first-party to Gradle. Fully read.","relevance-certainty":"-1:GUESS","relevance-description":"Weak negative evidence: the vendor's own case for the scan UI is about build-to-build COMPARISON, never about the terminal being the wrong medium for explanation. Useful only for what it declines to argue.","graded-by":"subagent","published":"2019-09-06","via":"mcp__fetch__fetch(url: 'https://develocity.ai/blog/improved-gradle-dependency-resolution-insights-in-build-scans/')"}
{"slug":"D-bazel-diagnosing-cache-misses-chapter-2026","url":"https://bazel.virtuslab.com/book/2~4~4/","grading-certainty":"-0:SUSPECT","grading-reasoning":"Reads as LLM-generated: template-uniform section scaffolding, a duplicated keyword block before the prose, and footnotes that all point back to the same four canonical Bazel pages rather than to independent evidence. D on LLM-smell; not F because the citations do check out against those first-party pages. Fully read.","relevance-certainty":"-1:GUESS","relevance-description":"Only genuinely useful for its staged escalation protocol ('add one clue at a time') and the framing of --explain as a first-response tool. Every factual claim in it is available first-party; treat as corroboration only, never as the citation.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch(url: 'https://bazel.virtuslab.com/book/2~4~4/')"}
```

Also read but not separately graded (short, single-purpose, folded into the Nix entries
above): NixOS/nix PR #8786 (`--precise` output detached from the tree because `std::cout`
was buffered while the rest used `logger->cout`) and PR #6072 (non-`--precise` output
indented four spaces too deep, so children rendered as siblings). Both are pure
tree-rendering regressions in the excerpt/no-excerpt split — evidence that maintaining
two rendering modes over one chain is itself a recurring bug source.

---

## (e) NEGATIVE RESULTS

- **Reddit: zero sources obtained.** `reddit.com/robots.txt` is `User-agent: * / Disallow: /`.
  I did not fetch it. Kagi `site:reddit.com` surfaced plausible threads (r/NixOS
  "I can't figure out why certain programs are still installed" — snippet ends
  "...I know that `nix why-depends` exists, but what it gives me is"; r/NixOS
  "Finding out what causes a package to get built") but I will not grade from snippets, so
  practitioner sentiment from Reddit is **absent from this report**. If it is wanted, a
  human needs to open those two threads.
- **HN item 16483889 unread.** Two fetch attempts, HTTP 429 both times; I backed off rather
  than hammer. The Kagi snippet reads "so make -d output is really hard to use. Technically
  once you learn to read the output it tends to have all the details you need to figure..."
  — consistent with the StackOverflow finding but NOT graded or relied on.
- **The academic why-not-provenance vein is UNREAD and is the real gap.** The directly
  on-point peer-reviewed paper is Lee/Ludäscher/Glavic, "Approximate Summaries for Why and
  Why-not Provenance", PVLDB 13(6) 2020 — its abstract states why-not provenance "can be
  very large, resulting in severe scalability and usability challenges" and proposes
  pattern-based summarisation, which is *exactly* the merge-topologically-equivalent-nodes
  idea with formal quality metrics (informativeness / conciseness / completeness). My fetch
  of `vldb.org/pvldb/vol13/p912-lee.pdf` returned raw PDF bytes (no text layer extraction),
  so I did not read it and have not graded it. Extended version is at
  `arxiv.org/abs/2002.00084` and `cs.uic.edu/~bglavic/dbgroup/assets/pdfpubls/LLG19.pdf`.
  **This is the one source in this vein most likely to change the design** — it would give
  Dorc a principled objective function for chain summarisation rather than the folk
  heuristics above. Recommend a human or a PDF-capable path retrieve it.
- **`buck2 log why-ran` does not exist.** Searched the canonical `buck2 log` reference (13
  subcommands, none named why-ran) and ran a GitHub code search for `why-ran` across
  `facebook/buck2` — 0 results. The nearest surfaces are `buck2 log what-ran`,
  `buck2 log diff action-divergence`, and `buck2 explain`. The premise should be corrected.
- **No Meta/Buck2 design rationale found for the explain web UI.** Searched for a blog post
  or talk stating why explain is a browser view (attention, size, or otherwise); nothing
  surfaced. `app/buck2_explain/` has no README — only `BUCK`, `js/`, `output_format_js/`,
  `src/`. The browser-view choice is documented as a fact, never justified in public.
- **No Bazel retrospective on explain-log usability exists.** The only substantive artefact
  is issue #13566, which died to a stale-bot. No post-mortem, no design doc, no blog.
- **`ninja -d explain` is undocumented in the Ninja manual** (verified against the manual's
  own tool/flag sections). It survives only in downstream docs like Chromium's.
- **`--verbose_explanations` is deprecated and undocumented in current Bazel** —
  `@Deprecated` with `documentationCategory = OptionDocumentationCategory.UNDOCUMENTED` in
  `src/main/java/com/google/devtools/build/lib/bazel/rules/BazelRulesModule.java`. Found via
  code search; I read only the matched fragment, not the whole file, so treat as a pointer.
- **No user study of any kind was found** on whether engineers read build-tool provenance
  traces. Every behavioural claim above is inferred from what practitioners describe
  themselves doing (dump-to-file, grep, diff) or from what maintainers changed. There is no
  measurement here, only revealed preference.
