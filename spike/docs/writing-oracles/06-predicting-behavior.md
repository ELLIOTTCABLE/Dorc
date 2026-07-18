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
   *)  return 2 ;;
   esac
}
```

Each line of a predict body is a claim, and the vocabulary is ordinary shell read
the obvious way. Running the real tool (as above) claims every channel faithfully
- the output is genuine, the exit status is genuine, and you have vouched the
invocation is read-only. A `printf` claims "this is what stdout would be". An
explicit `return` claims the exit status. Redirecting a channel to `/dev/null`
declines exactly that channel - "I make no claim about stderr here", as above -
which is honest and often right, since tools chatter unpredictable diagnostics.
And `return 2` before doing anything declines the whole shape, exactly as in
`is_converged`: mutating verbs, unmodeled flags, and anything you cannot stand in
for read-only all take this exit.

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

<!-- quoted: spike/CLAUDE.md rul-only-oracle-bytes-ship, rul-argv-flows-bytes-do-not,
     inv-one-observable, role-menu predict vocabulary; 23O rul-role-split;
     USER_STORY.md stage 4 predict-lane note; 273 predict-absorbs-wrapper-modeling -->
