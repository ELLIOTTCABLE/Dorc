# The probe contract

Dorc makes one promise to the people running books that outranks everything else:
planning never changes the machine. When an admin asks for a plan, Dorc probes the
host - running checks, in parallel, on a real production machine, often as a
privileged user - and the admin has been told, flatly, that this is safe. Dorc
cannot keep that promise by itself, because the checks it runs are not Dorc's code.
They are oracle bodies. They are your code.

This page is the contract you accepted when you wrote your first verdict function.
It has one absolute rule, and then a set of softer qualities that determine whether
your oracle is merely safe or actually good.

## The absolute rule: never mutate

Your verdict function's body must not change the machine. Not the state it checks,
not any other state, not "only a little", not "only the first time". This rule is
different in kind from everything else in Dorc, and it is worth understanding why.

Almost everywhere else, Dorc degrades gracefully: a partial description gives
partial value, a wrong judgment hurts one attributed line, a missing oracle just
means more lines run. Mutation-during-probe has no gradient to degrade along. The
admin was promised none; there is no such thing as acceptably little. A probe that
mutates is not a lower-quality probe, it is a broken promise with your name inside
it - and unlike nearly every other mistake in this system, it cannot be caught and
absorbed by "the line just runs". The damage is done before any plan exists.

Dorc does what it can mechanically: bodies are analyzed, and provable mutation is
refused loudly at plan time, before anything ships to a host. But that analysis is
a net under you, not the contract itself. It catches what can be proven; shell
being shell, most of what your body does is commands Dorc cannot see inside - the
whole reason you are writing this file is that only you know what `foobar status`
does. Your authorship is the vouch. There is no flag, no analysis mode, and no
review process that makes an arbitrary command safe to probe; the only source of
probe-safety in the entire system is a knowledgeable human choosing invocations
they know to be inert, inside a body they signed.

## Mutation is sneakier than it sounds

Nobody writes `rm -rf` in a status check. The failures that actually happen are
quieter, and worth a checklist-pass over any body you write:

Tools scaffold. Plenty of tools, asked an innocent question, will happily create
their config directory, initialize a cache, or write a lock file if one is missing,
as a courtesy on the way to answering. A check that is read-only on your
workstation (where all of that already exists) can be a writer on a fresh host.

Privilege changes the answer. A subtle trap: a check can look read-only only
because it lacked permission to write - its opportunistic cache refresh fails
silently as your user, so you never noticed it. Probes can legitimately run in
elevated or shifted contexts (a later page covers this), and the day your check
runs as root, it writes. The standard you are aiming for is read-only by design,
not read-only by privilege-starvation: pick invocations that do not attempt writes,
rather than invocations whose writes happen to fail.

Dry-run flags lie. A tool's `--dry-run` or `--check` mode is a claim by that
tool's authors, of varying honesty; some acquire locks, refresh metadata, or touch
state files anyway. If you have not verified what it really does (trace it, read
its source, or watch its file activity on a scratch machine), do not vouch for it.

Remote reads are someone else's write. A check that queries a network service is
mutating that service's logs, rate-limit counters, and possibly its state. Prefer
reading the durable local state a tool leaves on the machine over re-asking some
remote endpoint whether the machine is correct.

## Fast, and honest about cost

Probes run while a human sits waiting for their plan; every check you write is on
that critical path. Worse, a check does not only run at plan time: when Dorc cannot
fully elide a line, your check body gets inserted in front of it as a runtime
guard, which then runs on *every* apply, forever. That standing cost is called the
check-tax, and it is the lens for a judgment you must make per invocation-shape: if
checking would cost as much as just doing the work (the classic example is
`mkdir -p`, which is its own check), decline instead. A cheap idempotent command
does not need your protection; answering `2` and letting it run is the honest
optimum, and pretending otherwise buys the user a slower book.

The practical bar: a verdict body should be a handful of local reads - a file
stat, a database lookup, one query subcommand. If you find yourself iterating,
retrying, or reaching over the network, stop and reconsider the shape.

## Stable under repetition and concurrency

Probes are batched and run in parallel with each other, and nothing guarantees
your body runs exactly once. Write it as a pure function of machine state: no
temp files without unique names, no reliance on being the only reader, no
order-dependence with other oracles' checks. This is rarely hard - checks that
follow the previous section's advice are naturally reentrant - but it rules out a
few habits like caching your own results in a fixed path.

A related quality bears its own paragraph: answer from durable state, not from the
weather. A check whose answer depends on wall-clock time, a live DNS resolution, a
warm cache, or anything random gives Dorc an answer that may already be false when
the plan renders, and false again differently at apply. Read-only is necessary but
not sufficient; the answer should be a stable property of the machine's stored
state. If the truth genuinely lives behind something volatile, that is usually a
sign the honest answer is `2`.

## Fail toward can't-say

Your body will run on machines you have never seen: the tool missing entirely, a
different version with different output, a permission arrangement you did not
anticipate. Structure the body so that every surprising path lands on an exit
status of `2` or higher, not on a fake verdict. The standard opening line for any
check that depends on a binary:

```sh
command -v foobar >/dev/null 2>&1 || return 2
```

Silencing a tool's stderr is fine once you have gated on its existence; silencing
it instead of gating is how "command not found" becomes a confident wrong answer
on someone else's machine. The system-wide rule - everything fails toward run -
only holds if your body routes its own confusion into the can't-say row rather
than swallowing it.

<!-- quoted: spike/CLAUDE.md structural-vouch-only, rul-no-mutating-guards,
     rul-proven-mutation-fails-fast, rul-unprovable-rides-the-vouch,
     hermeticity-precondition; 27C vouch-tolerates rationale; KNOBS kPROBING
     check-tax; IMPLEMENTATION.md correctness-band -->
