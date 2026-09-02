# Predicting behavior

Everything so far has been the judging member of your oracle: `is_converged`
answers one question and licenses one thing. This page introduces its sibling, the
modeling member: `foobar__predict`, the one place you describe what invocations of
your tool actually *do*. The two members split the work cleanly - `predict` states
facts and makes no judgments; `is_converged` makes a judgment and states no facts.
You will usually write `is_converged` first and add `predict` later, when the
plan's hints tell you what it would unlock.

## Why modeling is a separate job

Two situations create the need, and both come from a hard rule you already know
pieces of: during probing, only oracle-authored bytes ever execute. Dorc never
ships fragments of the admin's book to a host to "just try them read-only" - book
bytes were written to mutate, and nobody vouched for them.

First: compound lines. Books are full of pipelines and multi-command constructs:

```sh
foobar list | grep -q wombat
```

To learn anything about that line at probe time, Dorc needs to execute something
shaped like it - but it may not use the book's bytes, and `is_converged` answers
a different question entirely ("is some invocation's work already done?", not
"what would this command emit?"). What Dorc actually ships is the same structure
with each participant replaced by its oracle's `predict`, invoked with that
participant's arguments. Your `predict` is the stand-in for your tool inside other
people's constructs.

Second: lifting hand-written guards. Admins already write defensive checks:

```sh
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx
```

That left-hand check is exactly the kind of read Dorc would love to run at probe
time - but the book's bytes still do not ship. Instead, the guard's shape is
recognized, and what ships is `dpkg__predict '-s' 'nginx'`: the *arguments* flow
across, the *bytes* come from the oracle's author. The admin's defensive habit is
rewarded, and every executed byte still has a vouching author. If no `predict`
exists for `dpkg`, that guard simply cannot lift, and the admin's line keeps
running as written.

## The shape of a predict

`predict` receives an invocation's arguments, exactly like `is_converged`, and its
job is to faithfully stand in for the command on every channel a consumer might
read: its effect on the machine, its exit status, its stdout, its stderr. It is an
oracle body, so the entire probe contract applies - above all, it never mutates,
no matter what the invocation it is modeling would have done.

For the read-only shapes of your tool, the best model is usually the tool itself:

```sh
dpkg__predict() {
   case "${1-}" in
   -s) dpkg "$@" 2>/dev/null ;;
   *)  printf 'predicts none unmodeled %s\n' "${1-}" >>"${DREP_V1:-/dev/null}"
       false ;;
   esac
}
```

A predict speaks channel by channel, and the rules are simple once you know
which way each channel defaults. The exit status is always claimed: whatever
the function finally exits with is the prediction, every value included, so a
`return 2` here predicts that the tool exits 2 - it is not a decline. Running
the real read-only tool (as above) therefore predicts its status faithfully.
Stdout and stderr go the other way: declined unless you say otherwise.
Redirecting stderr to `/dev/null`, as above, changes nothing about the claim -
stderr was already unclaimed - it only keeps noise out of the probe.

To claim an output channel, write one line after the complete output:

```sh
printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}"
```

It appends a short record to a channel Dorc provides through an environment
variable (harmless off Dorc, where the default sends it nowhere), and its
position matters: arriving after the bytes, it witnesses that the body got that
far. To decline a whole shape - mutating verbs, unmodeled flags, anything you
cannot stand in for read-only - write the same kind of line with `none`, as
the `*` arm above does. That record, not an exit status, is how a predict
refuses; write it for every unexplored shape before modeling any.

The engine's use of these claims is all-or-nothing per consumer: your predict
stands in for your tool inside a construct only if it covers every channel that
construct actually reads from it. If something downstream consumes your stdout and
your model only claimed an exit status, no substitution happens and the site keeps
its safe, unproven behavior. You do not need to model every channel - each honest
claim adds coverage, silence stays inert, and partial models are worth shipping.

## What predict does not do

A predict never licenses skipping anything. However precisely you model a mutating
verb - even "this invocation would do exactly nothing" - the license to not run a
line comes only from a reached, converged-answering `is_converged`. The split is
deliberate: facts and predictions can be checked, calibrated, and reported; the
decision that not-running is *acceptable* is a human judgment, and it lives only
in the member whose name signs it. If you find yourself wanting a prediction to
make a line disappear, what you actually want is a verdict arm for that verb.

One more thing `predict` quietly enables: if your tool is a wrapper - a command
whose job is to run another command (`sudo`, `env`, `nice`, `chroot`) - the shape
of your predict is how Dorc discovers that. A predict whose body ends by running
its own argument-slot marks the tool as a wrapper by construction, which opens a
whole set of machinery covered on page eight. You do not declare wrapper-ness
anywhere; you model the behavior, and the analyzer sees it.

And one thing it enables for the marks you wrote on page four: the first
genuinely modeled arm of a predict promotes the family's selector names into
the shared vocabulary that the footprint machinery (page seven) reasons with.
Until then those names still address your own facts; after it they can also
keep other people's lines elided. The contract reference has the exact rule.

<!-- quoted: spike/CLAUDE.md rul-only-oracle-bytes-ship, rul-argv-flows-bytes-do-not,
     inv-one-observable, role-menu predict vocabulary; 23O rul-role-split;
     USER_STORY.md stage 4 predict-lane note; 273 predict-absorbs-wrapper-modeling;
     notes/30D (channel defaults; predicts records; status keeps every value);
     plans/30J (predict-qualified family vocabulary) -->
