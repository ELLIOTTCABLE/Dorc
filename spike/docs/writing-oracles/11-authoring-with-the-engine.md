# Authoring with the engine in the loop

You have written the oracle and learned every construct it can carry. This page is
about the loop between you and Dorc while you refine the file, before you hand it to
strangers. Three things close that loop: you can tell Dorc what kind of decline each
of your refusals is, you can run a checker that reads your file the way Dorc will,
and you can ask Dorc what an admin will see when they ask why. Each one puts the
engine's own knowledge in front of you at the moment you are deciding something.

## Telling Dorc why you declined

Back when you grew your oracle to cover a real tool, you learned two shapes of
refusal: the silent `*) return 2`, and the loud one that leaves a short note on
Dorc's report channel before it declines. This page sharpens the loud one. A refusal
is more useful to everyone - to Dorc, to the next author, to the admin reading a
plan - when it says not just that you declined but *what kind* of decline it is.

Here is the sharpened form, on a `sysctl` oracle. Writing certain kernel keys is an
action the kernel performs and forgets, not a setting it stores; there is nothing to
read back, so "is this already done?" has no answer, ever:

```sh
sysctl__is_converged() {
   key=$1
   case $key in
   vm.drop_caches|vm.compact_memory)
      # Writing one of these keys tells the kernel to act now; it keeps no state
      # you could read back, so convergence has no meaning here - this line must
      # always run. An admin who asks why will be shown this comment.
      printf 'decline unsound %s is a write-only trigger key\n' "$key" \
         >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   *)
      # ordinary keys carry a value you can read and compare; that is the
      # covering-a-real-tool work, and it lives in this arm
      return 2 ;;
   esac
}
```

The `printf` line is ordinary defensive shell, and it is worth reading closely
because nothing about it is Dorc-specific. It appends one line to whatever file the
variable `DREP_V1` names. When Dorc runs your oracle it sets that variable to a
channel it is listening on; when anyone else runs your stripped file, the variable is
unset and the `:-/dev/null` default sends the line to the bit bucket. So the idiom is
total and safe everywhere: inside Dorc it is a report, outside Dorc it is a harmless
no-op, and it never trips `set -u`. The `strip` off-ramp leaves it untouched, because
it is working shell, not annotation. You could set `DREP_V1` yourself, in a plain
script that has nothing to do with Dorc, and collect these self-descriptions - the
facility stands on its own.

## The four kinds of decline

The word after `decline` is the class, and there are four of them. They are a fixed,
engine-owned vocabulary; it only ever grows, and a name once published keeps its
meaning. In prose these read as four plain situations:

- Permanently unanswerable (`unsound`). The question has no answer and never will,
  on any machine, because of how the tool works - a write-only trigger like the
  `sysctl` keys above, a command whose result is nondeterministic. You are not
  saying "not yet"; you are saying "there is nothing here to check, stop looking."
- Not modeled yet (`unmodeled`). A better oracle could answer this shape - you just
  have not written that arm. A flag you have not handled, a verb you have not gotten
  to. This is the honest "I could, but I have not."
- Interactive by construction (`interactive`). The shape prompts a human by design,
  so it cannot be probed on a headless machine at all. Think of a verb that opens an
  editor or asks a yes/no question before it acts.
- The author's hazard warning (`hazard`). You are not describing what you cannot
  check; you are telling the admin something about their own script - that this usage
  is deprecated, discouraged, a known footgun. It is the one class aimed at the
  person running the book rather than at Dorc's own reasoning.

Choosing a class is the same kind of judgment as choosing to decline in the first
place: reach for the one that states what you actually know. When you are unsure
which of two classes fits, you are usually unsure whether the shape is
permanently-unanswerable or merely not-modeled-yet, and the tell is simple - could a
more careful oracle answer it? If yes, it is not-modeled-yet; if no, it is
permanently-unanswerable.

## Your comment is written for a stranger

The comment above the `printf` is unusual, and worth a deliberate habit. Everywhere
else, a comment explains your code to someone maintaining it. This one does that too,
but it has a second reader: when an admin asks Dorc why a declined line ran, Dorc
shows them your arm, and your comment rides along as part of what they see. Dorc
never reads the comment for meaning - it is not configuration, it is not parsed, it
changes no decision - it is simply displayed, as your own words, to the person trying
to understand their plan.

So write the reason for a human who has your machine in front of them and no idea why
one line will not go away. State the tool-fact plainly. That comment is the closest
thing an oracle has to talking directly to the operator, and it costs you one honest
sentence.

## Spell the class literally

Dorc reads the class the good way - by reading your source, without running it -
whenever the format string is a plain literal, as it is above. If you build the
format in a variable first and then print the variable, Dorc cannot see the class by
reading; it can only learn it by running that arm on a real machine, which happens
later and helps you less while you are authoring. The rule is small: keep the format
string a literal in the `printf`.

```sh
# Dorc can read this class while you author, without running anything:
printf 'decline unmodeled %s: the --root form is not built yet\n' "$flag" \
   >>"${DREP_V1:-/dev/null}"

# Dorc cannot read this one until the arm actually runs on a host:
fmt='decline unmodeled %s: the --root form is not built yet\n'
printf "$fmt" "$flag" >>"${DREP_V1:-/dev/null}"
```

## What classing buys you

Silence is always legal - an unclassed `return 2` is exactly as safe as it ever was,
and the `*` arm usually stays quiet. Classing is enhancement, and each class turns a
different piece of Dorc's behavior in your favor:

- A permanently-unanswerable decline stops the nags. Dorc notices unmodeled commands
  and suggests, gently, that an oracle would help. For a shape you have declared has
  no answer, that suggestion is noise, and the class silences it: you looked, there
  is nothing to model, and Dorc stops asking.
- A not-modeled-yet decline turns a vague nudge into a specific invitation. Instead
  of "this command is unmodeled," the tooling can say your oracle covers these verbs
  and honestly declines that flag as not-yet-built - real coverage arithmetic, useful
  to you and to anyone deciding whether to extend your file.
- An interactive-by-construction decline explains a headless failure instead of
  looking like a gap. Dorc knows not to nag you toward probing something that cannot
  be probed without a human.
- A hazard warning reaches the admin. It is the one class that can surface, capped
  and attributed, on the plan an operator reads - your editorial judgment about their
  script, in your words.

Across all four, the shared payoff is the same one the comment convention hints at:
your words reach the people downstream as your own code, attributed to you, not
laundered into some generic sentence Dorc made up.

## dorc lint: the checker in your hot loop

While you author, you want the fast feedback that does not touch a single machine.
That is `dorc lint`. It is a grab-bag doctor for oracle files, in the spirit of the
`doctor` subcommand other tools ship: point it at your files and it reports what it
can find by reading alone. It contacts no hosts, ships no probes, and runs nothing of
your tool - it is safe to wire into a pre-commit hook on a laptop with the network
off. Crucially, it works on an oracle file with no book in sight; you do not need a
script that uses your tool to lint the oracle that describes it.

What it reports, in plain terms:

- Parsing and structure problems - the same understanding failures Dorc would hit
  during a real run, surfaced early.
- When you also hand it a book, an inventory of that book's walls: which commands
  are unmodeled, and where the first one falls (the first wall is the one that costs
  you the most downstream, so it is called out). The other checks need no book; this
  one uses one when it is there.
- Mechanical hazards in your verdict bodies - most importantly a body that ends in a
  pipeline, where the status you answer with might be the wrong command's.
- Findings from `shellcheck` and `checkbashisms` when those tools are installed,
  mapped back onto your original file's line numbers even though Dorc lints the
  stripped form.
- Your own decline inventory: for each verdict body, the shapes you deliberately
  decline and the class you gave each. This is where you check, before publishing,
  that your refusals say what you meant them to say.

For a publisher, the same lint run has a machine-readable mode meant for continuous
integration. The exact flags are still settling and shown here only to convey the
shape, but a CI invocation looks about like this:

```sh
dorc lint --format=jsonl --fail-on=warn --require-tools --expect-files 3 -- *.oracle.sh
```

It asks for machine output, fails the build on anything warning-or-worse, insists the
external tools actually be present rather than quietly skipped, and asserts how many
files it expected to lint so a file silently dropping out of the set is caught. The
interactive form - just `dorc lint` on your files - is tuned for reading, not for
gating, and stays out of your way.

## dorc why: seeing what the admin sees

The last part of the loop is asking Dorc the same question your admin will ask. Point
`dorc why` at a declined line and Dorc inlines the arm that declined it - your
`case` branch, your comment riding along, tagged with your file and the line number
of the emission. That is the whole feedback loop for a decline: you write the arm and
its comment, and `dorc why` shows you exactly the words a confused operator will read
back.

The same machinery names you everywhere else your judgment is load-bearing, not only
on declines. When a guard wraps a line, the plan can name the check that licensed it.
When a survival rests on a footprint claim, or a fact carries across contexts, the
trail names the claim and the file and line it came from. Asking why during
authoring is how you audit your own attribution - if the wrong file:line shows up, or
a claim you did not mean to make is holding up a skip, you find it here, before a
stranger does. The next page is about what it means to own that: publishing is
accepting that your name sits in every one of those chains.

One caution on reading these outputs. What Dorc prints - the exact words, the
ordering, how a chain is laid out - is deliberately unstable and improving release by
release. Read it for its content, never as a fixed format to build automation
against. The stable thing is the meaning: that your decline was classed the way you
classed it, that the line named is the line you wrote.

<!-- quoted: 27W:rul-emission-grammar-v1, rul-class-starter-set, rul-versioned-entry,
     rul-flagless-selection, rul-report-noise-tolerant, rul-report-surface-massaging;
     AID-NEEDS:aid-authored-decline-classes, aid-lint-oracle-solo-mode,
     aid-lint-unmodeled-inventory, aid-lint-verdict-body-mechanicals,
     aid-why-decline-narration; spike/CLAUDE.md decline-class-emission,
     report-lane-versioned-entry; 27R sect-0/2/4/5 lint surface;
     27V:rul-output-form-unwelded -->
