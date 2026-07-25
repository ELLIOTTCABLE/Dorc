# Vein: compiler note-chains, backtrace fatigue, and the two poles (rustc vs Elm)

Gathered for the `dag-explanation-ux` round. Read-only web gather; 23 sources fully read.
Bias per brief: **disproof first**. Negative evidence outweighs positive.

---

## (a) DON'T-DOS — with evidence

### dont-render-the-whole-chain-ship-head-and-tail

The single most direct precedent. Clang ships a **defaulted-ON truncation** of exactly the
structure Dorc plans to render. Douglas Gregor's own commit message, 2010-04-20
(`ffed1cb33`, llvm-svn 101882), verbatim:

> Introduce a limit on the depth of the template instantiation backtrace
> we will print with each error that occurs during template
> instantiation. When the backtrace is longer than that, we will print
> N/2 of the innermost backtrace entries and N/2 of the outermost
> backtrace entries, then skip the middle entries with a note such as:
>
>   note: suppressed 2 template instantiation contexts; use
>   -ftemplate-backtrace-limit=N to change the number of template
>   instantiation entries shown
>
> **This should eliminate some excessively long backtraces that aren't
> providing any value.**

Two weeks later, the macro version (`cd121fb01`, 2010-05-04, llvm-svn 103014):

> The macro instantiation backtrace is limited to 10 "instantiated from:"
> diagnostics; when it's longer than that, **we'll show the first half, then say
> how many were suppressed, then show the second half.** The limit can be changed
> with -fmacro-instantiation-limit=N, and turned off with N=0.
>
> **This eliminates a lot of note spew** with libraries making use of the
> Boost.Preprocess library.

Design rules extractable, verbatim from the source: (1) keep **head + tail**, elide the
**middle**; (2) **say how many** you suppressed; (3) name the **flag** that recovers them;
(4) the justification is that middle links "aren't providing any value" — not that they're
wrong, that they're *worthless*.

Third instance, `f6f003af6` (2011-12-16): `-fconstexpr-backtrace-limit`, "default 10".

First-party Clang User's Manual confirms the shipped defaults:
- `-ferror-limit=123` — "The default is 20"
- `-ftemplate-backtrace-limit=123` — "Only emit up to 123 template instantiation notes
  within the template instantiation backtrace for a single warning or error. The default
  is 10, and the limit can be disabled with `-ftemplate-backtrace-limit=0`."
- `-fcaret-diagnostics-max-lines` — "Controls how many lines of code clang prints for
  diagnostics. **By default, clang prints a maximum of 16 lines of code.**" (a cap on the
  *inlined code excerpt*, which is Dorc's per-link payload)

Corroboration from a second, independent implementation — GCC caps concepts-constraint
chain depth at **1** by default, and says so inline in its own output. Quoted verbatim
inside [A-concepts-error-messages-humans-2022]:

> `cc1plus: note: set '-fconcepts-diagnostics-depth=' to at least 2 for more detail`

...and then again at depth 2: "set ... to at least 3 for more detail". The chain is
default-collapsed to one link and *ratcheted open one notch at a time*.

### dont-assume-users-read-the-middle

TypeScript's own first-party documentation states the failure outright. From the TS 3.7
Playground doc "Flattened Error Reporting", verbatim:

> TypeScript's error messages can sometimes be a tad verbose... With 3.7, we've taken a
> few cases which could be particularly egregious.
>
> Before, it was 2 lines of code per nested property, **which quickly meant people learned
> to read error messages by reading the first and then last line of an error message.**
> Now they're inline.

This is the documented behavioural evidence the round wanted: users **skip to first+last**.
Note the convergence — Gregor's clang fix (2010) and TypeScript's flattening (2019)
independently arrived at the same conclusion: only the head and the tail get read.

The "before" shape TS was fixing (verbatim, their doc):

```
Type '{ b: { c: { d: { e: number; }; }; }; }' is not assignable to type '{ b: { c: { d: { e: string; }; }; }; }'.
  Types of property 'b' are incompatible.
    Type '{ c: { d: { e: number; }; }; }' is not assignable to type '{ c: { d: { e: string; }; }; }'.
      Types of property 'c' are incompatible.
        Type '{ d: { e: number; }; }' is not assignable to type '{ d: { e: string; }; }'.
          Types of property 'd' are incompatible.
            Type '{ e: number; }' is not assignable to type '{ e: string; }'.
              Types of property 'e' are incompatible.
                Type 'number' is not assignable to type 'string'
```

That is a **linearized chain of numbered links with a shared visual register** — the exact
silhouette of Dorc's plan. It is the canonical hated artifact of the entire JS ecosystem.
TypeScript's fix was **not** to number them or tag them; it was to **delete the chain** and
inline the endpoint.

TS's own maintainers flagged the scaling failure back in 2015 (#4451, Ryan Cavanaugh,
closed): "**The problem with the way we currently report errors will compound as types get
larger.**"

### dont-let-links-look-alike

Practitioner statement of the same failure, r/typescript (2021), quoted via Kagi index
(NOT fully read — see negative results): "it's one large blob where everything looks
exactly the same as the line preceding it."

This is the specific risk for Dorc: a chain of `1 / 1a / 1b / 2 / 3` links each with a
tier-word, a `file:line`, and a gutter excerpt has **maximum inter-line self-similarity**.
Uniformity is what converts a chain into a blob.

### dont-expect-richer-explanation-to-improve-outcomes

Three controlled studies, escalating in rigour and recency, all negative.

**[A-enhancing-syntax-errors-ineffectual-2014]** (Denny, Luxton-Reilly, Carpenter; ITiCSE;
n=83, randomised control/intervention). The intervention was precisely the "teaching page"
register: verbose explanation + worked example + corrected version + diff commentary.
Verbatim:

> We evaluate the effectiveness of the enhanced error messages with a controlled empirical
> study and **find no significant effect.**

> Although we anticipated that the enhanced error messages would help students to identify
> and correct errors, analysis of the data shows **no significant (or practical) effect**.
> [...] **In essence, we found no empirical evidence to support the use of enhanced error
> messages.**

Their own post-hoc diagnosis is the load-bearing bit for Dorc:

> The enhanced error messages we provided were **more verbose** than the raw error messages
> and although they may have provided an opportunity to learn about the likely cause of the
> error, **students may have been resistant to reading additional detail beyond the simple
> compiler output, especially when they encountered the same error multiple times.**

> A third possible explanation was that the enhanced feedback did not provide examples and
> explanations that **students could relate to their own code**. [...] it relies on students
> understanding the idea and transferring the knowledge to their own situation.

That second point vindicates Dorc's inlined-excerpt instinct (generic teaching text fails;
*their own code* is what lands) — but the first point is a direct hit on the density-register
ladder: repeat exposure actively *destroys* willingness to read the long form.

**[A-not-silver-bullet-llm-pems-2024]** (Santos & Becker; UKICER; n=106, within-subjects,
three conditions). Verbatim:

> we found that GPT-4 generated error messages outperformed conventional compiler error
> messages **in only 1 of the 6 tasks**, measured by students' time-to-fix each problem.
> Handwritten explanations still outperform LLM and conventional error messages, both on
> objective and subjective measures.

> **Even though students are not any faster, they still prefer GPT-4's explanations** to
> conventional compiler error messages.

This is the **preference/performance dissociation** and it is the most dangerous finding in
the whole vein for Dorc. Users will *tell you* the rich chain is better while being no
faster with it. Any validation of density registers by asking users which they like is
methodologically dead on arrival.

Same paper, the field's own summary of 50 years:

> **Little progress has been made to improve compiler and runtime error messages**, despite
> decades of guidelines proposed to improve them [...] error message enhancement has seen
> **weak to insignificant results** in improving student outcomes.

### dont-treat-reading-as-free

**[A-do-developers-read-error-messages-2017]** (Barik et al.; ICSE; n=56, eye-tracking).
Verbatim findings:

> 1) participants read error messages and **the difficulty of reading these messages is
> comparable to the difficulty of reading source code**, 2) difficulty reading error
> messages significantly predicts participants' task performance, and 3) participants
> allocate a substantial portion of their total task to reading error messages (13%–25%).

The framing matters: this *refutes* the comfortable excuse ("they didn't help because
nobody read them" — which is what Denny hypothesised). People **do** read. Reading is just
**as expensive as reading source code**. Every additional chain link is priced at
source-code-reading rates. A 6-link chain with inlined excerpts is, cognitively, ~6 code
reviews.

### dont-let-the-chains-framing-anchor-the-wrong-fix

Same paper, the motivating example, which is an *outcome* not an anecdote:

> If you were Barry, would you have done anything differently? If not, you're not all that
> different from the participants in our own study, where **53 of the 55 participants
> adopted a similar comprehension and resolution strategy. Unfortunately, this fix turns
> out to be incorrect.**

> Did Barry simply not pay enough attention to the error message? On close inspection, the
> error message does in fact mention supertype methods, though not explicitly by name. Or
> **could it also be the case that the error message leads developers to prioritize certain
> solutions spaces for their code over others?**

96% of participants were steered to the wrong fix *by the framing of a correct message*.
Dorc's chain carries a strong implicit claim — "here is the causal path" — and each
tier-word (`measured`/`vouched`/`claimed`) is an authority signal. A chain that is
*correct but truncated at the wrong join* will anchor an annoyed engineer harder than a
vague message would.

Barik also documents redundancy and occlusion as concrete harms:

> Unfortunately, the popup is less helpful than he expected **because it repeats the message
> he has already seen** in the problems pane. Moreover, **the error popup text is obscuring
> the method signature** which is where he believes the problem is actually located.

### dont-ship-friendly-verbose-as-the-only-register (the Elm counter-thesis)

The best-sourced critic, [B-elm-paternalistic-error-messages-2021] (jamalambda). He is a
*fan* of Elm, which makes the critique stronger. Verbatim:

> The more I found myself loathing my dance partner's smug attitude. Like "thanks Elm,
> didn't occur to me that `Int` and `Float` were examples of valid types." **Instead of
> seeing the compiler as a patient, wise instructor, I increasingly began to think of it as
> a condescending, paternalistic nag that just could not wait to lord over me with its
> knowledge.**

> Well, as it turns out, in the time I'd been away from Elm, **I had grown significantly as
> a software engineer.** [...] There wasn't anything the compiler was telling me that I
> didn't already know, and I was able to spot my mistakes immediately **without requiring a
> paragraph of didactic prose to explain what was going on.**

> The crux of the problem is that its communication style doesn't take experience into
> consideration. **It treats *everyone* like they are a novice.** For novices, this is
> great, but experts don't generally appreciate having basic concepts spelled out for them
> at every turn.

> It's not just the programmer's ego that benefits here — **the trade off with these wordy
> error messages is that they take up a lot of screen real estate**, and make for really
> huge popups when you hover over the red squigglies in your editor.

> That's why I'm proposing a design goal for language designers: **design for multiple
> levels of experience.** Maybe this is a flag that lets *me* tell the *compiler* how much
> help I need.

**Critically for Dorc: he shows exactly what he'd cut and what he'd keep.** His shortened
versions retain the header, the `Line N, Column M`, and **the gutter'd code excerpt with the
`>` / `^^^` markers**; what he deletes is the prose ("Why? Well, imagine one
`HeadingStyles`..." and "Note: Here is an example of a valid `type alias` for reference:").
The code excerpt survives the density cut. The teaching prose is what dies. That is a
direct, evidenced answer to "what does a density register actually strip?"

Second critic voice, r/haskell (2016), via Kagi index (NOT fully read): "I find Elm's error
messages verbose and annoying."

Elm's own compiler concedes the failure mode for the hardest cases, quoted verbatim inside
[B-writing-good-compiler-errors-2019]:

> Staring at this type is usually not so helpful, so I recommend reading the hints at
> <https://elm-lang.org/0.19.0/infinite-type> to get unstuck!

i.e. when the structure genuinely is a big tangled graph, Elm gives up on rendering it and
**punts to a URL**.

### dont-assume-the-teaching-register-will-ever-ship

Rust RFC 1644 (2016, accepted) specified a second, maximal density register: `--explain
errors`, which would render the *user's own code* inside a templated pedagogical narrative.
That is almost exactly Dorc's "full teaching page" register. Its tracking issue,
rust-lang/rust#34827 "Tracking issue for --explain expansion", opened 2016-07-14, was
**closed 2020-01-01 without the feature ever shipping**. Verified live: `rustc 1.96.0`
today still answers `rustc --explain` with `error: Argument to option 'explain' missing`.
There is no `--explain errors`. Ten years, never shipped.

The only substantive comment on why, GuillaumeGomez, 2016-07-20, verbatim:

> Some error explanations are (I think) very easy to update to this new RFC. Some others
> (the longest or most "general" ones) are **almost impossible as is**. Some rework on these
> last ones is needed first (split them in smaller errors for example).

The maximal register did not fail on taste. It failed because **the general/long cases
resisted templating** — precisely the cases that needed it. That is a costing lesson: the
teaching register is cheap for the easy facts and impossible for the hard ones, which
inverts the value curve.

Related: RFC 1644 itself admits it had no way to know if it was right —

> **How do we measure the readability of error messages?** This RFC details an educated
> guess as to what would improve the current state but **shows no ways to measure success.**

...and flags the density hazard directly:

> Can additional error notes be shown without the **"rainbow problem"** where too many
> colors and too much boldness cause errors to become less readable?

The follow-on effort to re-standardise the long-form `--explain` docs, **rust-lang/rfcs
PR #3370** ("(Re)standardise error code documentation", Ezrashaw, opened 2023-01-12), is
**still open and unmerged 3+ years later**; last substantive comment (GuillaumeGomez,
2023-02-14): "RFC looks good to me [...] I'm not sure what happens for the RFC approval and
merge now though." Its own motivation section is a warning about format drift:

> Some changes must be made, otherwise PR authors coming up with their own formatting for
> error docs will continue to increase. ***This is already happening*** and only makes error
> docs harder to understand.

(Note for the brief: there is **no 2026 draft RFC 3370**. The number is right, the year is
2023, and it never landed.)

### dont-underestimate-how-hard-ascii-dag-rendering-is

[B-magit-readable-log-graphs-2017] — Jonas Bernoulli (magit maintainer), issue #2989,
**still open after 9 years**, i.e. the redesign was proposed and never executed. Verbatim:

> **The main reason I find Git's log graphs unreadable as soon as things get a bit more
> complicated is that it tries to hard to preserve horizontal space.** In addition to making
> the complex graphs very hard to read, this also comes at the cost of wasting vertical
> space.

His proposed fix inverts the usual instinct — **spend horizontal space to buy legibility**:

> 1. All commits of a given branch are drawn in the same column, **even when that wastes
>    horizontal space.**
> 2. Arrange the branches based on their type (mainline, bug-fix, feature etc.).

And the concrete ASCII limits, verbatim, with his own rendering:

> 4. If a commit is both a merge commit, as well as a branch point, then it becomes hard to
>    do express that on a single line when using ascii. With unicode it's easier but still a
>    bit confusing. [...] **Using ascii it looks confusing:**
>
>    ```
>    *         *
>    | *       | *
>    *-        *<
>    | *       | *
>    *         *
>    ```
>
> 5. **Graph lines may "cross". That's also not easily expressed using ascii**, sub-optimal
>    in unicode, and not much of an issue using vector graphics.

Directly transferable: Dorc's join-nodes (`1a`/`1b -> 2`) are exactly the "merge commit that
is also a branch point" case, and edge-crossing is exactly what a DAG-with-joins produces.
Also note the pipeline he sketches: "1. Parse the `git log --parents` output. **This should
be easy.** 2. Turn that into a pretty graph. **This is hard.** 3. Draw that graph." The
analysis is not the hard part; the *layout* is.

Reinforcing this from the other side: RFC 1644's own constraint list says

> **Be careful using "ascii art" and avoid unicode.** Instead look for ways to show the
> information concisely that will work across the broadest number of terminals.

...and rustc, having said that, ships **no graph rendering at all** — only a vertical gutter
"wall" and labels.

### dont-emit-duplicate-or-cascading-links

- rustc-dev-guide, verbatim: "**Try not to emit multiple error messages for the same
  error.** This may require detecting duplicates."
- Elm shipped a fix specifically for this (Compilers as Assistants, verbatim): "**there are
  no more cascading errors in Elm 0.16** thanks to Hacker News! [...] with many compilers a
  single mistake can lead to 3 or 4 different error messages, leaving the programmer to
  figure out which one is the real problem."
- Santos & Becker's methodology treats it as settled practice, verbatim: "Whenever a single
  programming error would induce multiple spurious, cascading error messages, **we would
  only show the first error message, as advised by Becker et al.**"
- rustc has a whole triage label for it: `D-verbose` = "Diagnostics: Too much output caused
  by a single piece of incorrect code." **204 issues filed, 88 still open** (measured
  2026-07-25 via `gh issue list -R rust-lang/rust --label D-verbose`). Sibling labels
  include `D-terse` ("doesn't give enough information") — so the world's most celebrated
  diagnostics team maintains **bug categories in both directions simultaneously**, and has
  ~4x more open too-much than they can close. Verbosity is not a solved problem there; it
  is a permanent standing cost.

### dont-print-what-the-reader-can-already-see (a genuine, unresolved tension)

[B-writing-good-compiler-errors-2019] (Caleb Meredith, who redesigned Flow's error
messages) argues the *opposite* of Elm/rustc/RFC-1644 on inlined code, verbatim:

> **80%** of the time, the developer will immediately know what the fix is. [...] The
> developer may even know how to fix it *just* by seeing where the red squiggly in their
> IDE is. **No need to read the error message.**
> **20%** of the time, the developer won't immediately know why [...]

> In the **80%** case a developer is either quickly iterating through their code and **a
> long error message would be a *disservice* to their productivity.**

> **Don't print out information a developer could easily find in their code.** Instead print
> a reference to that information which is linked in an IDE. **TypeScript likes to print out
> huge types in error messages, this makes it hard to read the message.**

> **Short, simple, and clear error messages are *much* better than long and detailed error
> messages.**

He is explicit that this stance comes from his target surface:

> I design error messages **IDE first, not CLI first.** [...] Usually, most compiler
> developers are designing their error messages in the command line (CLI). **Which is why
> you get colored multiline messages that print out code.**

**This is a real, documented disagreement between two credible primaries, and it resolves on
surface, not on truth.** Dorc is CLI-native, so the Elm/rustc/RFC-1644 pole applies — but
Meredith's 80/20 split is still the correct prior on *how often the chain is even wanted*,
and his "print a reference, not the content" rule is the right instinct for any link whose
excerpt the reader is already looking at.

---

## (b) DO-DOS — with evidence for "works"

### do-anchor-every-link-to-the-users-own-code

The strongest positive finding in the vein, and it is the one thing that beat everything
else in a head-to-head. Santos & Becker 2024: the **handwritten** condition beat both stock
GCC and GPT-4 "on objective and subjective measures". Its structure, verbatim:

> Every message was written in a consistent structure: first was a line beginning with the
> word `Error:` which states what the detected error is, **followed by a relevant excerpt
> from the source code.** Then one or more sections beginning with the word `Help:` or
> `Note:` would either suggest a possible solution, or highlight relevant information to fix
> the problem. **The structure of the messages was greatly inspired by the diagnostics
> emitted by the Rust compiler, with source code excerpts mimicking the structure of Rust's
> "diagnostic windows".**

So: short + tiered keyword prefix + **inlined source excerpt in a rust-style window** is
the empirically winning shape. That is close to Dorc's per-link plan. The winning version
was, however, **one error, not a chain**.

Elm's first-party reasoning for the same choice, verbatim
[B-elm-compiler-errors-for-humans-2015]:

> With many compilers you get a location like `program.x:43:22` that you have to decipher.
> [...] You also often get a pretty-printed version of the problematic code, but **it looks
> nothing like the code you wrote.** [...] The error shows the code **exactly as you wrote
> it.** [...] Users can ask "does this look like that?" **without really needing much
> conscious analysis** of lines and columns and code.

Direct implication for Dorc's "lightly-massaged code excerpt": *the massaging is a hazard*.
Elm's whole claimed win here is byte-fidelity to what the user typed. Any normalisation
(reflowing, expanding `$vars`, canonicalising quoting) reintroduces the "mental
transformation" cost Elm was eliminating.

### do-put-the-fact-on-the-code-not-in-a-trailing-note

RFC 1644, verbatim constraint:

> **Where possible, use labels on the source itself rather than sentence "notes" at the
> end.**
> Keep `filename:line` easy to spot for people who use editors that let them click on
> errors.
> Each error should have a "header" section that is **visually distinct** from the code
> section.
> Code should visually stand out from text and other error messages.
> Error messages should be just as readable **when not using colors**.

rustc-dev-guide restates the preference in policy form, verbatim:

> If the order of the explanation can be "order free", **leveraging secondary labels in the
> main diagnostic is preferred, as it is typically less verbose** [than sub-diagnostics].

Translated to Dorc: a fact that can ride as a label on the excerpt should **not** be
promoted to its own numbered link. Links cost a whole visual register; labels are nearly
free. The chain should be the *residue* after everything label-able has been demoted.

RFC 1644's primary/secondary split is also a ready-made model for tier-words:

> The **primary label** [...] explains the **what** [...] uses the `^^^` underline.
> **Secondary labels** [...] explain the **why** [...] use blue text and `---` underline.
> Taken together, primary and secondary labels create a 'flow' to the message. Flow in the
> message lets the user **glance at the colored labels and quickly form an educated guess**.

Note: exactly **two** ranks, differentiated by *glyph* (`^^^` vs `---`) so it survives
colourlessness. Dorc's six tier-words
(`measured`/`vouched`/`ran`/`claimed`/`derived`/`consented`) are 3x that. That is a
scanning-cost claim worth pressure-testing: two ranks are glanceable, six probably require
reading.

### do-elide-the-shared-part-and-show-only-the-difference

Three independent implementations converged, which is about as strong as convergent
evidence gets in this space:

- **Clang**, default-on: "The default for template type printing is to **elide as many
  template arguments as possible, removing those which are the same in both template types,
  leaving only the differences.**" (`-fno-elide-type` turns it off; rendering is
  `vector<map<[...], map<float, [...]>>>`.)
- **Elm 0.16**, on Richard Feldman's report: "**The Elm compiler now does type diffs** where
  it compares any two types and highlights the differences. Notice that it **hides
  information that is not directly relevant**."
- **TypeScript 3.7**, the flattening described above.

For Dorc: on a chain where consecutive links share most of their state, render the **delta**
per link, not the full state per link.

### do-elide-unannotated-context-lines-aggressively

RFC 1644, verbatim: "Because we only show lines that are of interest for the given
error/warning, **we elide lines if they are not annotated as part of the message** (we
currently use the heuristic to **elide after one un-annotated line**)."

Concrete, shippable heuristic for the gutter excerpt: an unannotated line is allowed as
glue, two is a gap.

### do-make-the-structured-tree-a-pull-surface

Clang already has a tree renderer for its deepest structure, and it is **opt-in**:
`-fdiagnostics-show-template-tree` — "For diffing large templated types, this option will
cause Clang to display the templates as an **indented text tree, one argument per line**,
with differences marked inline." Verbatim output:

```
t.cc:4:5: note: candidate function not viable: no known conversion for 1st argument;
  vector<
    map<
      [...],
      map<
        [float != double],
        [...]>>>
```

Note the two features that make it survivable: `[...]` elision of unchanged subtrees, and
`[float != double]` collapsing a whole diff into one leaf. And note it is **off by default**
— which is exactly Dorc's push/pull split, already validated by a shipped compiler.

P2429's own proposal for concepts is the same shape and also explicitly interactive:

> we could **visualise constraint failures as trees**, where disjunctions and conjunctions
> appear as forks in the tree, and **you can expand nodes to delve deeper** into the reasons
> that they failed. [...] All of this would help **manage cognitive load** and provide
> dynamic interaction.

### do-give-the-user-the-density-knob-and-expect-experts-to-turn-it-down

Evidence that experts actively ask for *less*, from cargo#4165 (vitiral, 2017-06-13),
verbatim — this is the cleanest statement of the expert-decay curve I found:

> I've been programming in rust for the better part of a year now. The compiler has been my
> friend and mentor through this time, and a big part of that has been the fantastic human
> readable error messages. Thank you so much for them!
>
> However, **I have gotten much more familiar with the language and my common mistakes, so
> (often) no longer need error messages to be so long (sometimes they are longer than 10-15
> lines!). Normally I just need the compiler to give the me the file, line/col number and
> name of the error and I can spot it myself very quickly.**
>
> **For an experienced programmer, a shorter format would make finding errors much more
> ergonomic.**

He was already doing it by hand, verbatim:

```
# cargo test |& rg "(error\[\w+\])|(^\s+--> \w+/\w+)" -N --color never
```

i.e. **he was grepping out exactly two registers: the headline and the `file:line`.** That
is an empirically-observed "one-word tail + location" density register, discovered by a user
under pressure. Strong support for Dorc's minimal register being *headline + `file:line`*
and nothing else.

This shipped as `--message-format=short` / `--error-format=short`. Its shortcoming is also
documented — internals.rust-lang.org thread title (surfaced via Kagi, not fully read):
"`--message-format` short doesn't give enough information for mismatched types". So the
minimal register is *known to under-serve* a specific class; the ladder is real.

The literature agrees the knob should exist. P2429, citing the Becker et al. guidelines
survey, verbatim:

> **Allow dynamic interaction** by providing the user with **autonomy over error message
> presentation.**
> **Report errors at the right time** by giving the user **the right amount of information
> when they need it.**

And the 2021 readability study it summarises, verbatim:

> **Experts, non-experts and students assess the readability of messages differently.**
> [...] **Shorter messages tend to be more readable.** [...] Messages with more jargon and
> acronyms tend to be less readable.

### do-expect-users-to-want-the-causal-slice (the positive case for the DAG)

The single best piece of evidence *for* Dorc's core idea comes from
[A-why-developers-dont-use-static-analysis-2013] — interviews with 20 developers about why
they abandon static analysis tools. Verbatim, participant "Jason":

> Jason wishes that his tool's output would be **a "slice" that shows what the problem is
> and what else could be affected** in order to more quickly assess what is or is not
> important. **This "slice" should be taken from the entire project, using call hierarchies,
> to show which parts are affected by each defect.**

That is a developer, unprompted, asking for a provenance/impact DAG. And the same paper
shows exactly what happens without one, verbatim:

> Jason: "**...like I mentioned with FlexLint it gives you so many warnings and sifting
> through them is so, arduous that whenever I just look at it I'm like ehhh forget this.**"

> Cody, who likes using Dehydra, finds himself frustrated at times because **the results are
> dumped onto his screen with no distinct structure** causing him to spend a lot of time
> trying to figure out what needs to be done.

> He had a large list of warnings to scroll through but **without there being any context to
> the problems it just seemed like "a bunch of junk to sift through," which made him not
> want to bother using it.**

And the participatory-design session produced, verbatim, an almost exact description of
Dorc's inline-expansion register — participant "Chris":

> "I don't mind the idea of the actual source code itself having some plasticity . . . let's
> say the fourth line there was some error here. . . **having the 5th line drop down and
> having the content expand with maybe all sorts of annotations about my code.**"

Aggregate severity, verbatim: "**14 people [of 20] expressed the negative impacts of poorly
presented output**" and "**all but one of our participants have had problems with
understanding results.**" Presentation, not correctness, is the adoption blocker for
exactly Dorc's tool category.

Understand the ask precisely though: Jason wants a **slice that shows what else could be
affected** — an *impact* projection to triage importance. He did not ask for a *derivation*
of why the tool believes the fact. Dorc's chain answers "why should I trust this?"; the
observed demand is for "what does this touch?". Those are different graphs over the same
edges, and only one of them was asked for.

### do-put-the-scaffolding-cost-on-the-tier-word, carefully

P2429 on jargon-vs-scaffolding, verbatim — directly applicable to `measured`/`vouched`/
`claimed`/`derived`/`consented`:

> this is where there's **a tension between using jargon terms and simple language**: the
> simple language can make the diagnostic more readable, but **the jargon terms may aid in
> building mental scaffolding if it's clear how they relate to the code.** As such, when
> using jargon terms in errors, we must be **especially careful to make it clear what
> they're referring to in the code.**

And on the chain-as-argument framing, which is precisely Dorc's model, verbatim:

> **Logical argumentation** maps onto concepts error messages through **constraint failure
> stacks**. I.e. the messages provide logical argumentation by saying "Constraint A failed
> because it consists of the disjunction of Constraints B and C; Constraint B failed because
> […]". **Providing this context allows users to build up a mental model of what caused the
> failure.**

Note that P2429 endorses the *idea* and then, when it looks at GCC actually doing it,
condemns the execution — verbatim:

> **This gives us a lot of context. Too much, I would argue.** I don't think most users need
> to know which line of code in a standard library header the instantiation of
> `std::integral` failed at. [...] I think GCC is commendable in giving the user all they
> need to diagnose the error, but **the design of it raises cognitive load and hurts
> readability.**

> Similar to the first example, **we got all the information we could possibly need to
> diagnose the issue, but it's really hard to comprehend.**

The distinguishing rule P2429 lands on is **"Just My Code"**, verbatim:

> A problem that GCC's output has is that **its constraint failures often go deep into some
> standard library headers, whose code is hard to understand, and is difficult to connect to
> the user code. Ideally we would minimise this**, potentially by **only diagnosing the user
> code** and specifically handling each standard library concept to avoid needing to rely on
> STL internals.

For Dorc, the direct translation: links whose `file:line` lands **inside an oracle** are
GCC's stdlib-header frames. They are the ones that should collapse to a single vouched
summary line by default, with the oracle-internal chain available only on pull. Dorc's
oracle/book split gives it a *cleaner* "Just My Code" boundary than C++ has — this is
probably the highest-leverage cheap win in the whole gather.

### do-write-the-headline-so-it-stands-alone

rustc-dev-guide, verbatim:

> **Primary spans should have enough text to describe the problem in such a way that if it
> were the only thing being displayed** (for example, in an IDE) **it would still make
> sense.** Because it is "spatially aware" (it points at the code), it can generally be
> **more succinct than the error message**.

> Error messages should be succinct. **Users will see these error messages many times**, and
> more verbose descriptions can be viewed with the `--explain` flag. That said, don't make
> it so terse that it's hard to understand.

> When emitting a message with span, **try to reduce the span to the smallest amount possible
> that still signifies the issue**.

> Care should be taken when adding warnings to **avoid warning fatigue**, and avoid
> false-positives where there really isn't a problem with the code.

And their rule for *when a fact earns the long form at all* — this is the closest thing in
the corpus to a density-register admission policy, verbatim:

> **As a general rule, give an error a code (with an associated explanation) if the
> explanation would give more information than the error itself. A lot of the time it's
> better to put all the information in the emitted error itself.** However, sometimes that
> would make the error verbose or there are **too many possible triggers** to include useful
> information for all cases in the error, in which case it's a good idea to add an
> explanation.

Note the default: **inline it**. The separate register is the exception, justified by
combinatorial explosion of triggers, not by richness.

### do-keep-a-machine-readable-escape-hatch

Every one of the surveyed tools ships one, and P2429 recommends it as *the* way out of the
density dilemma, verbatim:

> Currently, most tools print their output to the console or error window as text. However,
> they could instead output **structured output such as JSON, XML, or formats designed for
> structured errors such as SARIF. This would allow tools to filter, manipulate, and
> visualise the errors in any way they wanted.**

Elm shipped `--report=json` in the same release as the human format
([B-elm-compiler-errors-for-humans-2015]); rustc has `--error-format=json`; RFC 1644 lists
IDE-integration breakage as its main drawback and points at JSON as the fix. For a DAG this
matters more than for a flat error: the graph is the thing a downstream tool most wants and
ASCII is the worst possible transport for it.

---

## (c) GRADED SOURCE TABLE

Every row below was fully read by me except where read-depth is stated in the reasoning.

```
{"slug":"A-concepts-error-messages-humans-2022","url":"https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2022/p2429r0.pdf","grading-certainty":"+1:SURE","grading-reasoning":"A not B: it is a numbered ISO/IEC JTC1/SC22/WG21 committee document (P2429R0) presented to SG15, i.e. a canonical first-party standards-body artifact, not a vendor blog; it also functions as a literature survey with named citations to the peer-reviewed PEM corpus, so it carries secondary-source authority on top of primary. Full text extracted and read end-to-end (26pp) via Exa PDF extraction after mcp-fetch returned raw PDF bytes.","relevance-certainty":"+1:SURE","relevance-description":"The single closest analogue to Dorc's problem in the literature: it surveys constraint-failure CHAINS (conjunction/disjunction stacks) rendered as linear note sequences, condemns GCC's full-depth rendering as 'too much context', names 'Just My Code' as the cut, and proposes expandable trees + structured output. Supplies both don't-dos (GCC verbatim walls) and do-dos (logical-argumentation framing, jargon/scaffolding tension).","graded-by":"subagent","published":"2022-05-16","via":"mcp__claude_ai_Exa__web_fetch_exa urls=[open-std.org/.../p2429r0.pdf] maxCharacters=60000, then Read of the persisted 51.7KB JSON"}
{"slug":"A-enhancing-syntax-errors-ineffectual-2014","url":"https://www.cs.auckland.ac.nz/courses/compsci747s2c/lectures/errors.pdf","grading-certainty":"+1:SURE","grading-reasoning":"A not B: ACM ITiCSE'14 peer-reviewed full paper (DOI 10.1145/2591708.2591748) with a randomised controlled design (n=83, control vs intervention), reported t-tests and null results — not an experience report. 177+ citations. Read in full including all five results tables and the discussion's self-diagnosis. Hosted copy is the author-institution mirror, so provenance is clean despite not being the ACM DL URL.","relevance-certainty":"+1:SURE","relevance-description":"The load-bearing disproof: the intervention is structurally Dorc's maximal density register (verbose explanation + worked example + corrected diff) and it produced no significant effect on any of three measures. Its post-hoc explanations name repeat-exposure resistance and generic-example non-transfer, both of which bear directly on the register ladder.","graded-by":"subagent","published":"2014-06","via":"mcp__claude_ai_Exa__web_fetch_exa urls=[cs.auckland.ac.nz/.../errors.pdf,jssmith1.github.io/.../ICSE_2017_EYE.pdf] maxCharacters=30000, then Read of persisted JSON"}
{"slug":"A-do-developers-read-error-messages-2017","url":"https://jssmith1.github.io/assets/pdf/ICSE_2017_EYE.pdf","grading-certainty":"+1:SURE","grading-reasoning":"A not B: ICSE 2017 peer-reviewed (DOI 10.1109/ICSE.2017.59), 141+ citations, instrumented eye-tracking study (n=56) with IRB number, OpenCV/Tesseract AOI pipeline and KL-divergence analysis disclosed — this is measurement, not opinion. Author-hosted preprint, identical to the IEEE version. Read fully through the analysis section.","relevance-certainty":"+1:SURE","relevance-description":"Supplies the price of a chain link: reading an error message is as cognitively hard as reading source code, and consumes 13-25% of task time. Also supplies the anchoring hazard — 53/55 participants were steered by a correct message to an incorrect fix — which is the sharpest risk to Dorc's authoritative tier-tagged chain.","graded-by":"subagent","published":"2017-05","via":"same Exa batch as A-enhancing-syntax-errors-ineffectual-2014"}
{"slug":"A-not-silver-bullet-llm-pems-2024","url":"https://arxiv.org/pdf/2409.18661","grading-certainty":"+1:SURE","grading-reasoning":"A not B: ACM UKICER'24 peer-reviewed (DOI 10.1145/3689535.3689554), within-subjects randomised n=106 with counterbalancing and a stated null result. Recency (2024) and the third-condition design (stock/handwritten/LLM) make it the strongest available replication of the enhancement-ineffectiveness finding. Read fully through methodology and tasks; the numeric results table was reached only in summary form via the abstract's 1-of-6 claim, hence not +1 on any per-task figure.","relevance-certainty":"+1:SURE","relevance-description":"Two things Dorc needs: (1) the preference/performance dissociation — users prefer the richer explanation while being no faster, which invalidates preference-based validation of density registers; (2) the winning condition's exact shape (short + Error:/Help:/Note: prefixes + rust-style inlined source excerpt), which is close to Dorc's per-link design and beat both alternatives.","graded-by":"subagent","published":"2024-09-05","via":"mcp__claude_ai_Exa__web_fetch_exa urls=[arxiv.org/pdf/2409.18661] then urls=[arxiv.org/abs/2409.18661]"}
{"slug":"A-why-developers-dont-use-static-analysis-2013","url":"https://petertsehsun.github.io/soen7481/papers/icse13b.pdf","grading-certainty":"-0:SUSPECT","grading-reasoning":"A not B on venue and method: ICSE 2013 peer-reviewed, 1224+ citations, 20 semi-structured interviews with coded transcripts and inter-rater reconciliation described. Downgraded from +1 certainty on read-depth only: the Exa extraction was capped at 28k chars and terminated partway through the Customizability subsection, so I read abstract/related/method/RQ1-results in full but not the Implications and Future Work sections. Course-page mirror rather than IEEE DL, but text matches the published abstract.","relevance-certainty":"+1:SURE","relevance-description":"The only source in the vein about Dorc's actual tool category (static analysers, not compilers) and the only one where a practitioner unprompted asks for a causal slice using call hierarchies. Also supplies the abandonment mechanism verbatim ('a bunch of junk to sift through', 'dumped onto his screen with no distinct structure') and a participatory-design sketch of inline expanding annotations.","graded-by":"subagent","published":"2013-05","via":"mcp__claude_ai_Exa__web_fetch_exa urls=[petertsehsun.github.io/soen7481/papers/icse13b.pdf] maxCharacters=28000"}
{"slug":"A-clang-template-backtrace-limit-commit-2010","url":"https://github.com/llvm/llvm-project/commit/ffed1cb33","grading-certainty":"+1:SURE","grading-reasoning":"A not B: this is the primary artifact itself — the authored commit introducing the feature, retrieved from the canonical repository via the GitHub API, with author (Douglas Gregor) and llvm-svn revision (101882) verified. There is no more authoritative statement of the rationale than the commit that implemented it; every doc page downstream is a paraphrase. Message read in full.","relevance-certainty":"+1:SURE","relevance-description":"THE direct precedent. A defaulted-ON truncation of a note-chain identical in shape to Dorc's, with the head/tail-keep, middle-elide, count-the-suppressed, name-the-flag policy stated verbatim and justified on the grounds that middle links 'aren't providing any value'.","graded-by":"subagent","published":"2010-04-20","via":"gh api repos/llvm/llvm-project/commits/ffed1cb33 --jq '.commit.author.date + \"\\n\" + .commit.message'"}
{"slug":"A-clang-macro-backtrace-limit-commit-2010","url":"https://github.com/llvm/llvm-project/commit/cd121fb01","grading-certainty":"+1:SURE","grading-reasoning":"A not B for the same reason as its sibling: primary authored artifact retrieved from the canonical repo (llvm-svn 103014, Douglas Gregor), not a secondary description. Graded separately from the template commit because it carries independent rationale — the Boost.Preprocessor 'note spew' trigger — rather than restating it.","relevance-certainty":"+1:SURE","relevance-description":"Confirms the head/tail policy was applied twice, independently, to a second chain type, and names the real-world trigger (a macro library generating long uniform chains). Supplies the term 'note spew' and the explicit first-half/count/second-half rendering order.","graded-by":"subagent","published":"2010-05-04","via":"gh api repos/llvm/llvm-project/commits/cd121fb01 --jq '.commit.author.date + \"\\n\" + .commit.message'"}
{"slug":"A-rust-rfc-default-expanded-errors-2016","url":"https://rust-lang.github.io/rfcs/1644-default-and-expanded-rustc-errors.html","grading-certainty":"-0:SUSPECT","grading-reasoning":"A rather than B because it is the accepted, canonical first-party design specification for the diagnostic format that the entire industry subsequently copied (Elm-influenced, then copied back by Santos&Becker's winning condition and by miette/ariadne) — a spec of direct relevance, fully read. Held at -0 not +1 because it is a design RFC rather than peer-reviewed research and it admits by its own text that it has no measurement backing, which is exactly the weakness the A tier otherwise implies it lacks.","relevance-certainty":"+1:SURE","relevance-description":"Supplies the constraint list Dorc's renderer should be judged against (labels-over-notes, ascii-art caution, colourless readability, clickable file:line, elide-after-one-unannotated-line, the rainbow problem), the two-rank primary/secondary label model as a tier-word precedent, and the specification of the maximal --explain errors register that then never shipped.","graded-by":"subagent","published":"2016-06-07","via":"mcp__fetch__fetch url=https://rust-lang.github.io/rfcs/1644-default-and-expanded-rustc-errors.html max_length=22000"}
{"slug":"B-rustc-dev-guide-diagnostics-policy-2026","url":"https://rustc-dev-guide.rust-lang.org/diagnostics.html","grading-certainty":"+1:SURE","grading-reasoning":"B not A: official first-party maintainer documentation and binding contributor policy for the most-praised diagnostics implementation in the industry — but it is a living guide with no peer review and no empirical backing for its rules, so it cannot sit with the measured studies. Read fully through the Lints section (the remainder is lint-machinery internals, not presentation policy).","relevance-certainty":"+1:SURE","relevance-description":"The operative policy text on exactly Dorc's questions: when a fact earns a separate explain-register vs being inlined, note-vs-help role split, the succinctness rule, dedupe-your-diagnostics, minimise-the-span, warning fatigue, and the preference for secondary labels over sub-diagnostics as the less verbose option.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch url=https://rustc-dev-guide.rust-lang.org/diagnostics.html max_length=25000"}
{"slug":"B-clang-users-manual-diagnostic-defaults-2026","url":"https://clang.llvm.org/docs/UsersManual.html","grading-certainty":"+1:SURE","grading-reasoning":"B not C: canonical first-party vendor documentation for the shipping compiler, generated from tree and current (24.0.0git), with no rot — the flag semantics and defaults it states are the authoritative ones. Not A because it is reference documentation stating what the defaults are, with the reasoning living elsewhere (in the commits, graded separately). Read the two relevant sections in full via two offset fetches; the surrounding warning-group catalogue was skipped as irrelevant.","relevance-certainty":"+1:SURE","relevance-description":"Supplies the shipped numbers Dorc can calibrate against — error-limit 20, template-backtrace-limit 10, caret-diagnostics-max-lines 16 (a cap on inlined excerpt length specifically) — plus two default-on elision behaviours (-fno-elide-type's diff-only type printing) and the opt-in structured tree renderer -fdiagnostics-show-template-tree with its [...] subtree elision.","graded-by":"subagent","published":"2026","via":"mcp__fetch__fetch url=https://clang.llvm.org/docs/UsersManual.html start_index=18000 and start_index=11500"}
{"slug":"B-typescript-flattened-error-reporting-2019","url":"https://www.typescriptlang.org/play/3-7/syntax-and-messaging/flattened-error-reporting.ts.html","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: it is first-party Microsoft/TypeScript documentation shipped with the 3.7 release notes, authored by the team, and it contains a behavioural claim about their own users that no secondary source could establish. Held at -0 rather than +1 because it is a Playground sample page rather than the handbook proper, so it is first-party-but-informal; the substance is corroborated by the 3.7 release notes and issue #4451. Read in full (short page).","relevance-certainty":"+1:SURE","relevance-description":"The single most damning quote in the gather for chain rendering: TypeScript states that its nested chain 'quickly meant people learned to read error messages by reading the first and then last line'. Independently corroborates Gregor's head/tail policy from a completely different ecosystem, and shows the before-shape verbatim — a numbered uniform chain that is the exact silhouette of Dorc's plan.","graded-by":"subagent","published":"2019-11","via":"mcp__fetch__fetch url=https://www.typescriptlang.org/play/3-7/syntax-and-messaging/flattened-error-reporting.ts.html max_length=10000"}
{"slug":"B-elm-compiler-errors-for-humans-2015","url":"https://elm-lang.org/news/compiler-errors-for-humans","grading-certainty":"+1:SURE","grading-reasoning":"B not C: core-author writing (Evan Czaplicki, language designer) on the official project site, and the originating document for the entire modern diagnostics movement — cited by RFC 1644, P2429, Flix's Onward!2022 paper, and the CHI 2021 readability study. Not A because it is a release blog post with no evaluation. mcp-fetch failed HTML simplification; read in full via Exa extraction.","relevance-certainty":"+1:SURE","relevance-description":"The canonical statement of the pole Dorc is aiming at, and the source of the byte-fidelity rule that most threatens Dorc's 'lightly-massaged' excerpt: Elm's claimed win is showing 'the code exactly as you wrote it' so users can pattern-match without conscious analysis. Also the origin of the --report=json escape hatch shipped alongside the human format.","graded-by":"subagent","published":"2015-06-30","via":"mcp__claude_ai_Exa__web_fetch_exa urls=[elm-lang.org/news/compiler-errors-for-humans,elm-lang.org/news/compilers-as-assistants] maxCharacters=18000"}
{"slug":"B-elm-compilers-as-assistants-2015","url":"https://elm-lang.org/news/compilers-as-assistants","grading-certainty":"+1:SURE","grading-reasoning":"B on the same basis as its companion — core-author, official site, foundational and heavily cited. Graded separately because it carries distinct load: the type-diff mechanism and the cascading-errors elimination, neither of which appear in the 2015 post. Read fully.","relevance-certainty":"+1:SURE","relevance-description":"Supplies the type-diff convergence datapoint (hide what is not directly relevant, show only the difference — matching Clang's -fno-elide-type default and TS 3.7) and the first-party account of eliminating cascading errors, i.e. the one-root-cause discipline Dorc calls fail-fast batching.","graded-by":"subagent","published":"2015-11-19","via":"same Exa batch as B-elm-compiler-errors-for-humans-2015"}
{"slug":"B-elm-paternalistic-error-messages-2021","url":"https://jamalambda.com/posts/2021-06-13-elm-errors.html","grading-certainty":"-1:GUESS","grading-reasoning":"B rather than C on read-depth and self-evidencing content: it is a low-reference personal blog with no institutional standing (which argues C), but it is a deep primary first-person account that shows its own worked evidence — three literal Elm error renderings plus the author's own shortened rewrites of each — so the argument can be evaluated without trusting the author. That artifact quality is what lifts it over C. Certainty is -1 because the B/C call here is genuinely a judgement about whether demonstrated reasoning outweighs absent provenance, and a reviewer could defensibly put it at C.","relevance-certainty":"+1:SURE","relevance-description":"The best-sourced statement of the counter-thesis and the only source that shows WHAT a density register strips: his shortened versions keep the header, the line:column, and the gutter'd code excerpt with ^^^ markers, and delete only the didactic prose. Also states the expert-decay mechanism ('I had grown significantly as a software engineer') and the screen-real-estate cost, and independently proposes Dorc's exact density-knob design.","graded-by":"subagent","published":"2021-06-13","via":"mcp__fetch__fetch url=https://jamalambda.com/posts/2021-06-13-elm-errors.html max_length=25000"}
{"slug":"B-writing-good-compiler-errors-2019","url":"https://calebmer.com/2019/07/01/writing-good-compiler-error-messages.html","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: the author redesigned Flow's error messages at Facebook and the post is the personal style guide behind that shipped work, i.e. core-author writing about a system he built, and it is the source P2429 cites for the Flow guidelines. Not A: personal blog, no evaluation, and the 80/20 split is asserted from experience rather than measured. Read in full.","relevance-certainty":"+1:SURE","relevance-description":"The strongest documented dissent from the inline-the-code approach ('Don't print out information a developer could easily find in their code'), explicitly grounded in IDE-first vs CLI-first design — which makes it a surface-dependent disagreement Dorc must resolve deliberately rather than a contradiction. His 80/20 prior on how often the chain is wanted at all is the cleanest available estimate for push/pull budgeting.","graded-by":"subagent","published":"2019-07-01","via":"mcp__fetch__fetch url=https://calebmer.com/2019/07/01/writing-good-compiler-error-messages.html max_length=28000"}
{"slug":"B-magit-readable-log-graphs-2017","url":"https://github.com/magit/magit/issues/2989","grading-certainty":"-1:GUESS","grading-reasoning":"B rather than C because the author is magit's sole maintainer writing a design analysis of a rendering problem he has shipped against for a decade, and the post contains literal ASCII renderings demonstrating each claimed limit — primary practitioner design writing, not a complaint. It sits below the vendor docs because it is an issue tracker post with no review. -1 certainty: the B/C boundary for maintainer-authored issue-tracker design docs is genuinely fuzzy and I would not argue hard against C.","relevance-certainty":"+1:SURE","relevance-description":"The only substantial writing found on ASCII DAG readability limits, and it names precisely Dorc's two hard cases: a node that is both a join and a fork cannot be expressed on one ASCII line, and crossing edges are not expressible at all. Its inverted principle — spend horizontal space to buy legibility — contradicts the compaction instinct. Still open after 9 years, so it also evidences that this redesign gets proposed and abandoned.","graded-by":"subagent","published":"2017-02-06","via":"mcp__fetch__fetch url=https://github.com/magit/magit/issues/2989 max_length=15000; state confirmed via gh api repos/magit/magit/issues/2989"}
{"slug":"B-rustc-explain-expansion-abandoned-2016","url":"https://github.com/rust-lang/rust/issues/34827","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: first-party tracking issue in the canonical repo, retrieved via API with state, state_reason and close date verified, and corroborated by executing rustc 1.96.0 locally to confirm --explain errors does not exist today. That live verification is what lifts it above a bare issue link. Not A because it is three comments of process record, not a design document. Body and all comments read.","relevance-certainty":"+1:SURE","relevance-description":"Documents that the maximal teaching register — user's own code templated into a pedagogical narrative, i.e. Dorc's full teaching page — was specified, accepted, prototyped and then closed in 2020 without shipping. GuillaumeGomez's comment gives the reason: the longest and most general explanations were 'almost impossible' to template, which inverts the expected value curve of a teaching register.","graded-by":"subagent","published":"2016-07-14","via":"gh api repos/rust-lang/rust/issues/34827 and .../comments; corroborated by `rustc --version` and `rustc --explain`"}
{"slug":"B-rust-rfc-restandardise-error-docs-2023","url":"https://github.com/rust-lang/rfcs/pull/3370","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: first-party RFC text in the rust-lang process, read in full from the author's rendered branch, with PR state and comment history verified via API. Not A because it is an unmerged draft with no normative force — indeed its being unmerged for three years is the finding. Certainty -0 because 'official but never accepted' is an awkward tier fit and C is arguable.","relevance-certainty":"-0:SUSPECT","relevance-description":"Directly answers the brief's question about a re-standardisation of the explain format: it exists, it is PR 3370, but it is 2023 not 2026 and it has sat unmerged. Its motivation section evidences that a teaching-register format DRIFTS without enforced spec ('This is already happening'). Relevance held at -0 because it concerns doc prose style, not chain rendering, so it informs register governance rather than layout.","graded-by":"subagent","published":"2023-01-10","via":"gh api repos/rust-lang/rfcs/pulls/3370 --jq .body; then mcp__fetch__fetch of the rendered text/0000-standardize-error-docs.md on the author branch"}
{"slug":"B-rustc-d-verbose-label-taxonomy-2026","url":"https://github.com/rust-lang/rust/labels/D-verbose","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not C: this is first-party repository metadata measured directly against the live GitHub API rather than any narrative about it — the label description string and the open/closed counts are facts, not claims. Not A because a bug-label count is a proxy metric, not a study; a reviewer could argue triage practice inflates or deflates it. Counts measured 2026-07-25 and reproducible.","relevance-certainty":"+1:SURE","relevance-description":"Quantifies that too-much-output is a permanent standing defect class, not a solved problem, at the most diagnostics-invested compiler project in existence: 204 D-verbose issues filed, 88 still open, alongside a symmetric D-terse label. Establishes that Dorc should expect to run both failure modes concurrently and forever.","graded-by":"subagent","published":"2026-07-25","via":"gh issue list -R rust-lang/rust --label D-verbose --limit 900 --state all/open --json number --jq length; gh api repos/rust-lang/rust/labels --paginate"}
{"slug":"C-rust-errors-too-verbose-since-172-2023","url":"https://github.com/rust-lang/rust/issues/115382","grading-certainty":"+1:SURE","grading-reasoning":"C not B: it is a single user bug report — one data point of user sentiment with no authority — but it is on the canonical tracker, carries verbatim before/after renderings, and drew maintainer response within a day, so it is a decent primary anecdote rather than slop. Not D because it is first-party-tracker primary material. Body and both comments read in full.","relevance-certainty":"-0:SUSPECT","relevance-description":"A concrete, dated instance of a correct causal chain (async future-not-Send trait-bound provenance) being reported as harmful because it buried the actual E0308 error. Relevance held at -0 because it was fixed within days, so it evidences the failure mode and the responsiveness rather than a durable design lesson.","graded-by":"subagent","published":"2023-08-30","via":"mcp__fetch__fetch url=https://github.com/rust-lang/rust/issues/115382; comments via mcp__github__issue_read method=get_comments"}
{"slug":"C-cargo-message-format-short-request-2017","url":"https://github.com/rust-lang/cargo/issues/4165","grading-certainty":"+1:SURE","grading-reasoning":"C not B: a single feature request from one user, no institutional weight — but it is primary, on the canonical tracker, dated, and contains the requester's actual working grep pipeline, which makes the claim self-evidencing rather than merely asserted. Not D for that reason. Read body and both maintainer comments in full.","relevance-certainty":"+1:SURE","relevance-description":"The cleanest available statement of the expert density-decay curve, from a user who loved the verbose format for a year and then needed less of it. His hand-rolled ripgrep filter empirically identifies the minimal register as exactly headline plus file:line — direct calibration for Dorc's lowest density rung.","graded-by":"subagent","published":"2017-06-13","via":"gh api repos/rust-lang/cargo/issues/4165 and .../comments"}
{"slug":"C-typescript-intersection-error-nearmiss-2015","url":"https://github.com/microsoft/TypeScript/issues/4451","grading-certainty":"-0:SUSPECT","grading-reasoning":"C rather than B despite being authored by a TypeScript maintainer, because it is a three-comment issue stub with no design resolution — it states a problem and was closed, so it carries anecdotal rather than documentary weight. Held at -0: an argument for B on maintainer-authorship is reasonable. Read in full.","relevance-certainty":"-0:SUSPECT","relevance-description":"Supplies the early first-party prediction that chain-style error reporting degrades with scale ('will compound as types get larger') and the failure mode of chasing the wrong branch of a disjunction to its end rather than reporting the nearest miss — which is exactly the join-node selection problem in Dorc's 1a/1b -> 2 shape. Relevance -0 because it is a one-line prediction, corroborative rather than load-bearing.","graded-by":"subagent","published":"2015-08-25","via":"gh api repos/microsoft/TypeScript/issues/4451"}
{"slug":"B-clang-constexpr-backtrace-limit-commit-2011","url":"https://github.com/llvm/llvm-project/commit/f6f003af6","grading-certainty":"-0:SUSPECT","grading-reasoning":"B not A: primary authored commit from the canonical repo (llvm-svn 146749), same provenance class as the two Gregor commits — but its message states the default without restating a rationale, so it is corroborating evidence of a pattern rather than an independent argument. That thinness is why it sits a tier below its siblings. Message read in full.","relevance-certainty":"-0:SUSPECT","relevance-description":"Third independent application of the same default-10 chain cap, this time to constexpr evaluation note stacks, showing the policy was treated as settled house style by 2011 rather than a one-off. Corroborative only.","graded-by":"subagent","published":"2011-12-16","via":"gh api repos/llvm/llvm-project/commits/f6f003af6"}
```

---

## (e) NEGATIVE RESULTS — veins searched, nothing (or not enough) found

1. **Reddit practitioner sentiment could not be fully read.** `reddit.com/robots.txt`
   disallows `User-agent: *` outright, so mcp-fetch correctly refused. Exa returned
   `SOURCE_NOT_AVAILABLE` for both target threads. Kagi's summarizer *did* return content
   for r/typescript `qgalog`, but that is a generated summary, not verbatim text, so per the
   brief's rule I have **not graded** either thread and have marked the two Reddit quotes in
   §(a) as index-surfaced-only. Threads identified but unread:
   `r/typescript/comments/qgalog` ("I think something really needs to be done about
   TypeScript [errors]"), `r/haskell/comments/54l3ug` ("I find Elm's error messages verbose
   and annoying"), `r/typescript/comments/10zn2im`. **If the human can paste these three
   threads, they are the cheapest remaining upgrade to the Elm-critic and TS-blob claims.**

2. **No telemetry, survey, or usage data on `rustc --explain` exists that I could find.**
   Searched rust-lang/rust, the dev-guide, internals.rust-lang.org, and the Annual Rust
   Survey framing. The dev-guide *asserts* the explain register's purpose ("more verbose
   descriptions can be viewed with the `--explain` flag") but nobody has published whether
   anyone climbs that rung. **This is the single biggest hole in the whole vein** — the
   entire density-register thesis rests on an unmeasured assumption that ladders get
   climbed, and neither Rust, Clang, nor GCC has published evidence either way. Treat
   "users will pull for more detail" as **unvalidated**, not as prior art.

3. **`-Zmacro-backtrace` yielded nothing citable.** It exists as a nightly rustc flag but I
   found no design discussion, rationale, or usage commentary — no commit thread, no RFC, no
   maintainer writing. Its mere existence corroborates the pattern (macro chains are hidden
   by default) but I have no rationale text for it, unlike the Clang equivalents.

4. **GCC's own documentation for `-fconcepts-diagnostics-depth` could not be fetched
   first-party.** `gcc.gnu.org/robots.txt` returned a connection error, so mcp-fetch
   refused. The default-of-1 behaviour is attested only secondhand — though strongly, via
   GCC's own inline output text quoted verbatim inside P2429 ("set
   '-fconcepts-diagnostics-depth=' to at least 2 for more detail"). Worth a human fetch of
   `gcc.gnu.org/onlinedocs/gcc/C++-Dialect-Options.html` to confirm the documented default.

5. **No `git log --graph` legibility study exists.** Searched for empirical work on ASCII
   DAG readability limits and found none — no HCI paper, no measured comparison, nothing.
   The magit issue is the *only* substantive practitioner writing I located, and it is one
   maintainer's design analysis. **Dorc's ASCII-DAG plan has essentially no evidence base
   under it in either direction.** That is a genuine gap, not a null result.

6. **No `mypy --show-error-context` design discussion found.** Confirmed the flag exists in
   the mypy CLI docs, but found no rationale, no issue thread, no commentary on whether
   users enable it. Same for `tsc --pretty` — I confirmed first-party that it defaults on
   and "offers you a chance to have less terse, single colored messages", but there is no
   design writing behind it. Neither is worth a grading row.

7. **No evidence found of Elm's approach being formally *rejected* anywhere.** I looked for
   a project that tried Elm-style verbose friendly errors and reverted. Found the opposite —
   uptake (Rust RFC 1644 explicitly drew from Elm; Flix's Onward!2022 paper cites both Elm
   posts as its inspiration; Chapel's Dyno front-end cites Czaplicki). The critique is
   real and well-sourced but it is a **register complaint, not a rejection**: every critic
   found, including jamalambda, wants the verbose form kept and a terse form added.

8. **No measured evidence that anyone reads a 40-frame instantiation backtrace.** The brief
   asked for this specifically. There is abundant folklore (codegolf's "generate the longest
   C++ error message" contest, StackOverflow threads on suppressing backtraces, the
   taskflow issue where a user just sets `-ftemplate-backtrace-limit=0`) but **no
   measurement**. The nearest thing to data is TypeScript's first-party assertion that users
   read first-and-last, which is about a different chain in a different language.

---

## Cross-cutting synthesis note for the conductor

Three implementations that never coordinated — Clang (2010), Elm (2015), TypeScript
(2019) — independently converged on the **same two moves** when their note-chains got long:

1. **Keep the head and the tail; elide the middle; state the count.**
2. **Show only the difference between adjacent links; elide what they share.**

Neither move is "render the chain better". Both are "render less of the chain". No surveyed
implementation ships an always-on linearized chain of the kind Dorc plans; the closest
thing that exists (Clang's `-fdiagnostics-show-template-tree`) is opt-in, and the closest
thing that was *designed* (rustc's `--explain errors`) was abandoned after four years.

The one place a practitioner asked for a causal graph unprompted
([A-why-developers-dont-use-static-analysis-2013]) they asked for an **impact slice**
("what else could be affected"), not a **provenance derivation** ("why do I believe this").
Those are different projections of the same edge set and only one has demand evidence
behind it. That distinction seems worth putting to the human before the renderer is
specified.
